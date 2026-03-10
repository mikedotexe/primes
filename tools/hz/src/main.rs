use clap::{Parser, Subcommand, ValueEnum};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use primes::hzlib::harmonic_lagrange::fit_lineout_poly;
use primes::hzlib::harmonic_overtones::overtone_spectrum;
use primes::hzlib::symmetry_breaking::ridge_trough;
use primes::hzlib::verification::verify_to_csv;
use primes::hzlib::{self, join_sample_and_model, lineout, load_explain_json, Axis, JoinedGrid};

#[derive(Parser, Debug)]
#[command(author, version, about = "hz: post-processing CLI for prime-density grids", long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Join sample+model and write verification table (Δ, enrichment, CI, top moduli)
    Verify {
        #[arg(long)]
        sample: PathBuf,
        #[arg(long)]
        model: PathBuf,
        #[arg(long)]
        explain: Option<PathBuf>,
        #[arg(long, default_value = "hz_out/verification_results.csv")]
        out: PathBuf,
    },

    /// Discrete Fourier spectrum of a lineout (obs|pred|enrichment)
    Overtones {
        #[arg(long)]
        sample: PathBuf,
        #[arg(long)]
        model: PathBuf,
        #[arg(long, value_enum, default_value_t = AxisArg::Mid)]
        axis: AxisArg,
        #[arg(long, default_value_t = 0)]
        fixed: usize, // if axis=mid => fixed inner_zero; if axis=iz => fixed mid_len
        #[arg(long, value_enum, default_value_t = Quantity::Enrichment)]
        quantity: Quantity,
        #[arg(long, default_value_t = 8)]
        topk: usize,
        #[arg(long)]
        out: Option<PathBuf>, // if None: print to stdout
    },

    /// Lagrange/Newton poly fit of a lineout; writes x,y_obs,y_fit
    Lagrange {
        #[arg(long)]
        sample: PathBuf,
        #[arg(long)]
        model: PathBuf,
        #[arg(long, value_enum, default_value_t = AxisArg::Mid)]
        axis: AxisArg,
        #[arg(long, default_value_t = 0)]
        fixed: usize,
        #[arg(long, value_enum, default_value_t = Quantity::Enrichment)]
        quantity: Quantity,
        #[arg(long, default_value_t = 5)]
        degree: usize,
        #[arg(long, default_value = "hz_out/lagrange_lineout.csv")]
        out: PathBuf,
    },

    /// Trough ridge detection along the chosen axis; writes key,argmin,value
    Ridge {
        #[arg(long)]
        sample: PathBuf,
        #[arg(long)]
        model: PathBuf,
        #[arg(long, value_enum, default_value_t = AxisArg::Mid)]
        axis: AxisArg,
        #[arg(long, value_enum, default_value_t = Quantity::Pred)]
        quantity: Quantity,
        #[arg(long, default_value = "hz_out/ridge.csv")]
        out: PathBuf,
    },

    /// Emit a lineout table: x,obs,pred,enrichment
    Lineout {
        #[arg(long)]
        sample: PathBuf,
        #[arg(long)]
        model: PathBuf,
        #[arg(long, value_enum, default_value_t = AxisArg::Mid)]
        axis: AxisArg,
        #[arg(long, default_value_t = 0)]
        fixed: usize,
        #[arg(long, default_value = "hz_out/lineout.csv")]
        out: PathBuf,
    },

    /// Compare two sample CSVs (A vs B) and write per-cell deltas + top changes
    Compare {
        #[arg(long, help = "sample CSV for run A")]
        sample_a: PathBuf,
        #[arg(long, help = "sample CSV for run B")]
        sample_b: PathBuf,
        #[arg(long, default_value = "hz_out/compare.csv")]
        out: PathBuf,
        #[arg(
            long,
            default_value_t = 20,
            help = "print top N improvements/regressions"
        )]
        top: usize,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum AxisArg {
    Mid,
    Iz,
}

impl From<AxisArg> for Axis {
    fn from(a: AxisArg) -> Self {
        match a {
            AxisArg::Mid => Axis::Mid,
            AxisArg::Iz => Axis::InnerZero,
        }
    }
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Quantity {
    Obs,
    Pred,
    Enrichment,
}

fn ensure_parent(path: &Path) -> io::Result<()> {
    if let Some(dir) = path.parent() {
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }
    }
    Ok(())
}

fn load_grid(sample: &Path, model: &Path) -> io::Result<JoinedGrid> {
    let s = hzlib::load_sample_csv(sample)?;
    let m = hzlib::load_model_csv(model)?;
    Ok(join_sample_and_model(&s, &m))
}

// Helper functions for Compare subcommand
fn parse_f64(map: &HashMap<String, String>, k: &str) -> Option<f64> {
    map.get(k).and_then(|v| v.parse::<f64>().ok())
}

fn parse_usize(map: &HashMap<String, String>, k: &str) -> Option<usize> {
    map.get(k).and_then(|v| v.parse::<usize>().ok())
}

fn parse_u32(map: &HashMap<String, String>, k: &str) -> Option<u32> {
    map.get(k).and_then(|v| v.parse::<u32>().ok())
}

fn enr(obs: Option<f64>, pred: Option<f64>) -> Option<f64> {
    match (obs, pred) {
        (Some(o), Some(p)) if p > 0.0 => Some(o / p - 1.0),
        (Some(_), Some(_)) => Some(f64::INFINITY),
        _ => None,
    }
}

fn intervals_overlap(a: (Option<f64>, Option<f64>), b: (Option<f64>, Option<f64>)) -> Option<bool> {
    match (a, b) {
        ((Some(la), Some(ha)), (Some(lb), Some(hb))) => Some(!(ha < lb || hb < la)),
        _ => None,
    }
}

fn fmt_opt(x: Option<f64>) -> String {
    match x {
        Some(v) => format!("{:.12}", v),
        None => String::from(""),
    }
}

fn fmt_bool(x: Option<bool>) -> String {
    match x {
        Some(true) => "true".into(),
        Some(false) => "false".into(),
        None => "".into(),
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Cmd::Verify {
            sample,
            model,
            explain,
            out,
        } => {
            let grid = load_grid(&sample, &model)?;
            let exp_map = match explain {
                Some(p) => Some(load_explain_json(p)?),
                None => None,
            };
            ensure_parent(&out)?;
            verify_to_csv(&grid, exp_map.as_ref(), out)?;
        }

        Cmd::Overtones {
            sample,
            model,
            axis,
            fixed,
            quantity,
            topk,
            out,
        } => {
            let grid = load_grid(&sample, &model)?;
            let (fixed_mid, fixed_iz) = match axis {
                AxisArg::Mid => (0usize, fixed),
                AxisArg::Iz => (fixed, 0usize),
            };
            let qty = match quantity {
                Quantity::Obs => "obs",
                Quantity::Pred => "pred",
                Quantity::Enrichment => "enrichment",
            };
            let spec = overtone_spectrum(&grid, axis.into(), fixed_mid, fixed_iz, qty);
            match out {
                None => {
                    for (k, amp) in spec.into_iter().take(topk) {
                        println!("k={},amp={:.8}", k, amp);
                    }
                }
                Some(path) => {
                    ensure_parent(&path)?;
                    let mut w = File::create(path)?;
                    writeln!(w, "k,amp")?;
                    for (k, amp) in spec {
                        writeln!(w, "{},{}", k, amp)?;
                    }
                }
            }
        }

        Cmd::Lagrange {
            sample,
            model,
            axis,
            fixed,
            quantity,
            degree,
            out,
        } => {
            let grid = load_grid(&sample, &model)?;
            let (fixed_mid, fixed_iz) = match axis {
                AxisArg::Mid => (0usize, fixed),
                AxisArg::Iz => (fixed, 0usize),
            };
            let qty = match quantity {
                Quantity::Obs => "obs",
                Quantity::Pred => "pred",
                Quantity::Enrichment => "enrichment",
            };
            let series = fit_lineout_poly(&grid, axis.into(), fixed_mid, fixed_iz, degree, qty);
            ensure_parent(&out)?;
            let mut w = File::create(out)?;
            writeln!(w, "x,y_obs,y_fit")?;
            for (x, y, f) in series {
                writeln!(w, "{},{:.12},{:.12}", x, y, f)?;
            }
        }

        Cmd::Ridge {
            sample,
            model,
            axis,
            quantity,
            out,
        } => {
            let grid = load_grid(&sample, &model)?;
            let qty = match quantity {
                Quantity::Obs => "obs",
                Quantity::Pred => "pred",
                Quantity::Enrichment => "enrichment",
            };
            let ridges = ridge_trough(&grid, axis.into(), qty);
            ensure_parent(&out)?;
            let mut w = File::create(out)?;
            writeln!(w, "key,argmin,value")?;
            for r in ridges {
                writeln!(w, "{},{},{}", r.key, r.argmin, r.value)?;
            }
        }

        Cmd::Lineout {
            sample,
            model,
            axis,
            fixed,
            out,
        } => {
            let grid = load_grid(&sample, &model)?;
            let (fixed_mid, fixed_iz) = match axis {
                AxisArg::Mid => (0usize, fixed),
                AxisArg::Iz => (fixed, 0usize),
            };
            let series = lineout(&grid, axis.into(), fixed_mid, fixed_iz);
            ensure_parent(&out)?;
            let mut w = File::create(out)?;
            writeln!(w, "x,obs,pred,enrichment")?;
            for (x, obs, pred) in series {
                let enr = primes::hzlib::enrichment(obs, pred);
                writeln!(w, "{},{:.12},{:.12},{:.12}", x, obs, pred, enr)?;
            }
        }

        Cmd::Compare {
            sample_a,
            sample_b,
            out,
            top,
        } => {
            let a = hzlib::load_sample_csv(&sample_a)?;
            let b = hzlib::load_sample_csv(&sample_b)?;

            let mut map_a: HashMap<(u32, usize, usize), &HashMap<String, String>> = HashMap::new();
            let mut map_b: HashMap<(u32, usize, usize), &HashMap<String, String>> = HashMap::new();

            for r in &a {
                if let (Some(base), Some(m), Some(z)) = (
                    parse_u32(r, "base"),
                    parse_usize(r, "mid_len"),
                    parse_usize(r, "inner_zero"),
                ) {
                    map_a.insert((base, m, z), r);
                }
            }
            for r in &b {
                if let (Some(base), Some(m), Some(z)) = (
                    parse_u32(r, "base"),
                    parse_usize(r, "mid_len"),
                    parse_usize(r, "inner_zero"),
                ) {
                    map_b.insert((base, m, z), r);
                }
            }

            let mut keys: Vec<(u32, usize, usize)> = map_a.keys().cloned().collect();
            for k in map_b.keys() {
                if !keys.contains(k) {
                    keys.push(*k);
                }
            }
            keys.sort_unstable();

            ensure_parent(&out)?;
            let mut w = File::create(out)?;
            writeln!(w, "base,mid_len,inner_zero,obs_a,obs_b,delta,rel,enr_a,enr_b,enr_delta,ci_lo_a,ci_hi_a,ci_lo_b,ci_hi_b,overlap")?;

            let mut deltas: Vec<(f64, (u32, usize, usize))> = Vec::new();

            for k in keys {
                let ra = map_a.get(&k).copied();
                let rb = map_b.get(&k).copied();

                let (base, mid, iz) = k;

                let obs_a = ra.and_then(|r| parse_f64(r, "prime_density"));
                let obs_b = rb.and_then(|r| parse_f64(r, "prime_density"));

                let pred_a = ra
                    .and_then(|r| parse_f64(r, "expected_density_local_exact"))
                    .or_else(|| ra.and_then(|r| parse_f64(r, "expected_density_local")));
                let pred_b = rb
                    .and_then(|r| parse_f64(r, "expected_density_local_exact"))
                    .or_else(|| rb.and_then(|r| parse_f64(r, "expected_density_local")));

                let enr_a = enr(obs_a, pred_a);
                let enr_b = enr(obs_b, pred_b);

                let ci_la = ra.and_then(|r| parse_f64(r, "ci_lo"));
                let ci_ha = ra.and_then(|r| parse_f64(r, "ci_hi"));
                let ci_lb = rb.and_then(|r| parse_f64(r, "ci_lo"));
                let ci_hb = rb.and_then(|r| parse_f64(r, "ci_hi"));

                let delta = match (obs_a, obs_b) {
                    (Some(a), Some(b)) => Some(b - a),
                    _ => None,
                };
                let rel = match (obs_a, obs_b) {
                    (Some(a), Some(b)) if a != 0.0 => Some(b / a - 1.0),
                    _ => None,
                };
                let enr_delta = match (enr_a, enr_b) {
                    (Some(a), Some(b)) => Some(b - a),
                    _ => None,
                };

                let overlap = intervals_overlap((ci_la, ci_ha), (ci_lb, ci_hb));

                writeln!(
                    w,
                    "{},{},{},{:.12?},{:.12?},{:.12?},{:.12?},{:.12?},{:.12?},{:.12?},{:.12?},{:.12?},{:.12?},{:.12?},{}",
                    base, mid, iz,
                    fmt_opt(obs_a), fmt_opt(obs_b),
                    fmt_opt(delta), fmt_opt(rel),
                    fmt_opt(enr_a), fmt_opt(enr_b), fmt_opt(enr_delta),
                    fmt_opt(ci_la), fmt_opt(ci_ha), fmt_opt(ci_lb), fmt_opt(ci_hb),
                    fmt_bool(overlap)
                )?;

                if let Some(d) = delta {
                    deltas.push((d.abs(), (base, mid, iz)));
                }
            }

            // Print top changes (human-readable)
            deltas.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            println!("# top {} changes by |delta| (obs_b - obs_a)", top);
            for (i, (_mag, (base, mid, iz))) in deltas.into_iter().take(top).enumerate() {
                let ra = map_a.get(&(base, mid, iz)).copied();
                let rb = map_b.get(&(base, mid, iz)).copied();
                let aobs = ra
                    .and_then(|r| parse_f64(r, "prime_density"))
                    .unwrap_or(0.0);
                let bobs = rb
                    .and_then(|r| parse_f64(r, "prime_density"))
                    .unwrap_or(0.0);
                println!("{:>3}. base={}, mid_len={}, inner_zero={}, obs_a={:.6}, obs_b={:.6}, delta={:+.6}",
                    i+1, base, mid, iz, aobs, bobs, bobs - aobs);
            }
        }
    }
    Ok(())
}
