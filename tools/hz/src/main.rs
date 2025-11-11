use clap::{Parser, Subcommand, ValueEnum};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use primes::hzlib::{
    self, Axis, JoinedGrid, lineout, join_sample_and_model, load_explain_json,
};
use primes::hzlib::harmonic_overtones::overtone_spectrum;
use primes::hzlib::harmonic_lagrange::fit_lineout_poly;
use primes::hzlib::symmetry_breaking::ridge_trough;
use primes::hzlib::verification::verify_to_csv;

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
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum AxisArg { Mid, Iz }

impl From<AxisArg> for Axis {
    fn from(a: AxisArg) -> Self {
        match a { AxisArg::Mid => Axis::Mid, AxisArg::Iz => Axis::InnerZero }
    }
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Quantity { Obs, Pred, Enrichment }

fn ensure_parent(path: &Path) -> io::Result<()> {
    if let Some(dir) = path.parent() {
        if !dir.exists() { fs::create_dir_all(dir)?; }
    }
    Ok(())
}

fn load_grid(sample: &Path, model: &Path) -> io::Result<JoinedGrid> {
    let s = hzlib::load_sample_csv(sample)?;
    let m = hzlib::load_model_csv(model)?;
    Ok(join_sample_and_model(&s, &m))
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Cmd::Verify { sample, model, explain, out } => {
            let grid = load_grid(&sample, &model)?;
            let exp_map = match explain {
                Some(p) => Some(load_explain_json(p)?),
                None => None,
            };
            ensure_parent(&out)?;
            verify_to_csv(&grid, exp_map.as_ref(), out)?;
        }

        Cmd::Overtones { sample, model, axis, fixed, quantity, topk, out } => {
            let grid = load_grid(&sample, &model)?;
            let (fixed_mid, fixed_iz) = match axis {
                AxisArg::Mid => (0usize, fixed),
                AxisArg::Iz  => (fixed, 0usize),
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
                    for (k, amp) in spec { writeln!(w, "{},{}", k, amp)?; }
                }
            }
        }

        Cmd::Lagrange { sample, model, axis, fixed, quantity, degree, out } => {
            let grid = load_grid(&sample, &model)?;
            let (fixed_mid, fixed_iz) = match axis {
                AxisArg::Mid => (0usize, fixed),
                AxisArg::Iz  => (fixed, 0usize),
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
            for (x, y, f) in series { writeln!(w, "{},{:.12},{:.12}", x, y, f)?; }
        }

        Cmd::Ridge { sample, model, axis, quantity, out } => {
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
            for r in ridges { writeln!(w, "{},{},{}", r.key, r.argmin, r.value)?; }
        }

        Cmd::Lineout { sample, model, axis, fixed, out } => {
            let grid = load_grid(&sample, &model)?;
            let (fixed_mid, fixed_iz) = match axis {
                AxisArg::Mid => (0usize, fixed),
                AxisArg::Iz  => (fixed, 0usize),
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
    }
    Ok(())
}
