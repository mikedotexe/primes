//! Orthogonal Landscape: Denominator Analysis for Rational Approximations
//!
//! This tool analyzes denominators q for approximating constants like pi, e, sqrt(2), etc.
//! It computes two independent metrics:
//!
//! - **X (geometric)**: approximation quality of p/q to constant alpha
//!   (digits of precision per digit of denominator)
//!
//! - **Y (material)**: cycle behavior of 1/q as a decimal
//!   - purity: ord/phi (fraction of totient used)
//!   - utilization: ord/lambda (fraction of Carmichael lambda used)
//!   - slippage: lambda/phi (group structure loss)
//!
//! The tool finds "double peaks" (denominators scoring well on BOTH axes)
//! and groups results by core families (after stripping 2/5 factors).
//!
//! # Example
//!
//! ```bash
//! cargo run --bin orthogonal_landscape -- --alpha pi --q-max 200000 --double 40 --families 12
//! cargo run --bin orthogonal_landscape -- --alpha sqrt2 --q-max 200000 --y-mode utilization
//! cargo run --bin orthogonal_landscape -- --alpha rat:22/7 --q-max 10000 --csv out.csv
//! ```

use primes::hzlib::num_theory::Material;
use primes::hzlib::orthogonality::pearson;
use std::collections::HashMap;
use std::env;
use std::f64::consts::{E, PI};
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Clone, Copy, Debug)]
enum XMode {
    DigitsPerDigit,
    DigitsPerBit,
}

#[derive(Clone, Copy, Debug)]
enum YMode {
    Purity,
    Utilization,
    Slippage,
}

#[derive(Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
    q: u64,
    p: u64,
    err: f64,
    mat: Material,
    nx: f64,
    ny: f64,
    score_min: f64,
    score_prod: f64,
}

fn parse_alpha(s: &str) -> (f64, String) {
    let t = s.trim().to_lowercase();
    if t == "pi" {
        return (PI, "pi".to_string());
    }
    if t == "e" {
        return (E, "e".to_string());
    }
    if t == "sqrt2" || t == "sqrt(2)" {
        return (2.0f64.sqrt(), "sqrt2".to_string());
    }
    if t == "phi" || t == "golden" {
        return ((1.0 + 5.0f64.sqrt()) / 2.0, "phi".to_string());
    }
    if t == "tau" || t == "2pi" {
        return (2.0 * PI, "tau".to_string());
    }
    if let Some(rest) = t.strip_prefix("const:") {
        let v = rest.parse::<f64>().expect("Invalid const value");
        return (v, format!("const:{v}"));
    }
    if let Some(rest) = t.strip_prefix("rat:") {
        let parts: Vec<&str> = rest.split('/').collect();
        if parts.len() != 2 {
            panic!("rat: format must be rat:a/b");
        }
        let a = parts[0].trim().parse::<f64>().expect("Invalid numerator");
        let b = parts[1].trim().parse::<f64>().expect("Invalid denominator");
        return (
            a / b,
            format!("rat:{}/{}", parts[0].trim(), parts[1].trim()),
        );
    }
    let v = t.parse::<f64>().expect("Invalid numeric value");
    (v, format!("const:{v}"))
}

fn best_p(alpha: f64, q: u64) -> u64 {
    let t = alpha * q as f64;
    let p0 = t.floor() as i64;
    let mut best = p0;
    let mut best_err = (alpha - (p0 as f64 / q as f64)).abs();
    for dp in [-2i64, -1, 0, 1, 2] {
        let p = p0 + dp;
        if p < 0 {
            continue;
        }
        let err = (alpha - (p as f64 / q as f64)).abs();
        if err < best_err {
            best_err = err;
            best = p;
        }
    }
    best as u64
}

fn geom_x(alpha: f64, p: u64, q: u64, mode: XMode) -> f64 {
    if q <= 1 {
        return 0.0;
    }
    let err = (alpha - (p as f64 / q as f64)).abs();
    if err == 0.0 {
        return f64::INFINITY;
    }
    let correct_digits = -err.log10();
    match mode {
        XMode::DigitsPerDigit => correct_digits / (q as f64).log10(),
        XMode::DigitsPerBit => correct_digits / (64 - q.leading_zeros()) as f64,
    }
}

fn quantile_bounds(vals: &[f64], lo_q: f64, hi_q: f64) -> (f64, f64) {
    let mut v: Vec<f64> = vals.iter().copied().filter(|x| x.is_finite()).collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if v.is_empty() {
        return (0.0, 1.0);
    }
    let n = v.len();
    let lo_i = ((lo_q * (n as f64 - 1.0)).round() as usize).min(n - 1);
    let hi_i = ((hi_q * (n as f64 - 1.0)).round() as usize).min(n - 1);
    let mut lo = v[lo_i];
    let mut hi = v[hi_i];
    if hi <= lo {
        hi = lo + 1e-12;
    }
    if !lo.is_finite() {
        lo = 0.0;
    }
    if !hi.is_finite() {
        hi = lo + 1.0;
    }
    (lo, hi)
}

fn clamp01(x: f64) -> f64 {
    x.clamp(0.0, 1.0)
}

fn ranks(vals: &[f64]) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..vals.len()).collect();
    idx.sort_by(|&i, &j| vals[i].partial_cmp(&vals[j]).unwrap());
    let mut r = vec![0usize; vals.len()];
    for (rank, i) in idx.into_iter().enumerate() {
        r[i] = rank;
    }
    r
}

fn spearman(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len().min(ys.len());
    let mut x = Vec::new();
    let mut y = Vec::new();
    for i in 0..n {
        if xs[i].is_finite() && ys[i].is_finite() {
            x.push(xs[i]);
            y.push(ys[i]);
        }
    }
    if x.len() < 2 {
        return f64::NAN;
    }
    let rx = ranks(&x);
    let ry = ranks(&y);
    let rx_f: Vec<f64> = rx.into_iter().map(|v| v as f64).collect();
    let ry_f: Vec<f64> = ry.into_iter().map(|v| v as f64).collect();
    pearson(&rx_f, &ry_f)
}

fn pareto_frontier(mut pts: Vec<Point>) -> Vec<Point> {
    pts.retain(|p| p.x.is_finite() && p.y.is_finite());
    pts.sort_by(|a, b| {
        b.x.partial_cmp(&a.x)
            .unwrap()
            .then(b.y.partial_cmp(&a.y).unwrap())
    });
    let mut out = Vec::new();
    let mut best_y = -1.0;
    for p in pts {
        if p.y > best_y {
            best_y = p.y;
            out.push(p);
        }
    }
    out
}

fn print_help() {
    eprintln!(
        "Orthogonal Landscape: Denominator Analysis

Usage:
  cargo run --bin orthogonal_landscape -- [options]

Options:
  --alpha VALUE        Target constant to approximate
                       pi, e, sqrt2, phi (golden), tau (2pi)
                       const:<float>, rat:a/b, or raw float
  --q-min N            Minimum denominator (default: 2)
  --q-max N            Maximum denominator (default: 200000)
  --stride N           Step size for scanning (default: 1)
  --x-mode MODE        X-axis metric: digits_per_digit (default), digits_per_bit
  --y-mode MODE        Y-axis metric: purity (default), utilization, slippage
  --no-penalty         Disable 0.95^(v2+v5) penalty for 2/5 satellites
  --double K           Show top K double-peak candidates (default: 30)
  --double-frontier-only  Restrict double-peaks to Pareto frontier
  --families K         Show top K core families (default: 10)
  --csv PATH           Export results to CSV file
  --help               Show this help message

Examples:
  # Find best pi approximations with high-purity denominators
  cargo run --bin orthogonal_landscape -- --alpha pi --q-max 200000 --double 40

  # Compare sqrt2 approximations using utilization metric
  cargo run --bin orthogonal_landscape -- --alpha sqrt2 --y-mode utilization

  # Export top 100 golden ratio approximations
  cargo run --bin orthogonal_landscape -- --alpha phi --double 100 --csv phi_results.csv
"
    );
}

fn main() {
    // Parse arguments
    let mut alpha_s = "pi".to_string();
    let mut q_min: u64 = 2;
    let mut q_max: u64 = 200_000;
    let mut stride: u64 = 1;
    let mut x_mode = XMode::DigitsPerDigit;
    let mut y_mode = YMode::Purity;
    let mut no_penalty = false;
    let mut double_k: usize = 30;
    let mut allow_dominated = true;
    let mut families_k: usize = 10;
    let mut csv: Option<String> = None;

    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--alpha" => {
                i += 1;
                alpha_s = args[i].clone();
            }
            "--q-min" => {
                i += 1;
                q_min = args[i].parse().expect("Invalid q-min");
            }
            "--q-max" => {
                i += 1;
                q_max = args[i].parse().expect("Invalid q-max");
            }
            "--stride" => {
                i += 1;
                stride = args[i].parse().expect("Invalid stride");
            }
            "--x-mode" => {
                i += 1;
                x_mode = match args[i].as_str() {
                    "digits_per_bit" => XMode::DigitsPerBit,
                    _ => XMode::DigitsPerDigit,
                };
            }
            "--y-mode" => {
                i += 1;
                y_mode = match args[i].as_str() {
                    "utilization" => YMode::Utilization,
                    "slippage" => YMode::Slippage,
                    _ => YMode::Purity,
                };
            }
            "--no-penalty" => {
                no_penalty = true;
            }
            "--double" => {
                i += 1;
                double_k = args[i].parse().expect("Invalid double count");
            }
            "--double-frontier-only" => {
                allow_dominated = false;
            }
            "--families" => {
                i += 1;
                families_k = args[i].parse().expect("Invalid families count");
            }
            "--csv" => {
                i += 1;
                csv = Some(args[i].clone());
            }
            "--help" | "-h" => {
                print_help();
                return;
            }
            other => {
                eprintln!("Unknown argument: {}", other);
                print_help();
                return;
            }
        }
        i += 1;
    }

    let (alpha, alpha_label) = parse_alpha(&alpha_s);

    // Scan denominators
    let mut points: Vec<Point> = Vec::new();
    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();

    for q in (q_min..=q_max).step_by(stride as usize) {
        let p = best_p(alpha, q);
        let x = geom_x(alpha, p, q, x_mode);
        let err = (alpha - (p as f64 / q as f64)).abs();

        let mat = Material::for_base10(q);
        let penalty = if no_penalty {
            1.0
        } else {
            0.95f64.powi((mat.v2 + mat.v5) as i32)
        };

        let y_raw = match y_mode {
            YMode::Purity => mat.purity,
            YMode::Utilization => mat.utilization,
            YMode::Slippage => mat.slippage,
        };
        let y = y_raw * penalty;

        if x.is_finite() && y.is_finite() && x != f64::INFINITY {
            xs.push(x);
            ys.push(y);
        }

        points.push(Point {
            x,
            y,
            q,
            p,
            err,
            mat,
            nx: 0.0,
            ny: 0.0,
            score_min: 0.0,
            score_prod: 0.0,
        });
    }

    let r = pearson(&xs, &ys);
    let rho = spearman(&xs, &ys);
    println!(
        "alpha={} q=[{},{}] stride={} N={}",
        alpha_label,
        q_min,
        q_max,
        stride,
        points.len()
    );
    println!(
        "cloud correlation: Pearson r={:.4}  Spearman rho={:.4}",
        r, rho
    );

    // Pareto frontier
    let frontier = pareto_frontier(points.clone());
    println!("pareto frontier size: {}", frontier.len());
    if frontier.len() == 1 {
        let c = &frontier[0];
        println!(
            "frontier collapsed: champion q={} x={:.3} y={:.4}",
            c.q, c.x, c.y
        );
    }

    // Double peaks (quantile normalize)
    let xlohi = quantile_bounds(&xs, 0.05, 0.95);
    let ylohi = quantile_bounds(&ys, 0.05, 0.95);

    let base: Vec<Point> = if allow_dominated {
        points.clone()
    } else {
        frontier.clone()
    };

    let mut dps: Vec<Point> = Vec::new();
    for mut pt in base {
        if !pt.x.is_finite() || !pt.y.is_finite() || pt.x == f64::INFINITY {
            continue;
        }
        let nx = clamp01((pt.x - xlohi.0) / (xlohi.1 - xlohi.0));
        let ny = clamp01((pt.y - ylohi.0) / (ylohi.1 - ylohi.0));
        pt.nx = nx;
        pt.ny = ny;
        pt.score_min = nx.min(ny);
        pt.score_prod = nx * ny;
        dps.push(pt);
    }

    dps.sort_by(|a, b| {
        b.score_min
            .partial_cmp(&a.score_min)
            .unwrap()
            .then(b.score_prod.partial_cmp(&a.score_prod).unwrap())
            .then(b.x.partial_cmp(&a.x).unwrap())
            .then(b.y.partial_cmp(&a.y).unwrap())
    });

    println!(
        "\nDOUBLE PEAKS (top {})  normalize=5%..95% quantiles",
        double_k
    );
    for (k, pt) in dps.iter().take(double_k).enumerate() {
        let lift = match (pt.mat.v2, pt.mat.v5) {
            (0, 0) => String::new(),
            (a, 0) => format!("2^{}*", a),
            (0, b) => format!("5^{}*", b),
            (a, b) => format!("2^{}*5^{}*", a, b),
        };
        println!(
            "#{:02} q={:<8} p={:<10} x={:>7.3} y={:>7.4} nx={:>5.3} ny={:>5.3} min={:>5.3} prod={:>5.3}  core={}  lift={}",
            k + 1, pt.q, pt.p, pt.x, pt.y, pt.nx, pt.ny, pt.score_min, pt.score_prod, pt.mat.core, lift
        );
    }

    // Families by core(q)
    if families_k > 0 {
        let mut fam: HashMap<u64, Vec<&Point>> = HashMap::new();
        for pt in dps.iter().take(double_k * 50) {
            fam.entry(pt.mat.core).or_default().push(pt);
        }
        let mut fam_rank: Vec<(f64, f64, u64, usize)> = fam
            .iter()
            .map(|(&core, pts)| {
                let best = pts[0];
                (best.score_min, best.score_prod, core, pts.len())
            })
            .collect();
        fam_rank.sort_by(|a, b| {
            b.0.partial_cmp(&a.0)
                .unwrap()
                .then(b.1.partial_cmp(&a.1).unwrap())
        });

        println!("\nFAMILIES (top {}) grouped by core(q):", families_k);
        for (i, (smin, sprod, core, cnt)) in fam_rank.into_iter().take(families_k).enumerate() {
            println!(
                "  #{:02} core={:<8} best(min={:.3}, prod={:.3}) count={}",
                i + 1,
                core,
                smin,
                sprod,
                cnt
            );
            if let Some(pts) = fam.get(&core) {
                for pt in pts.iter().take(8) {
                    let lift = match (pt.mat.v2, pt.mat.v5) {
                        (0, 0) => String::new(),
                        (a, 0) => format!("2^{}*", a),
                        (0, b) => format!("5^{}*", b),
                        (a, b) => format!("2^{}*5^{}*", a, b),
                    };
                    println!(
                        "      q={:<8} x={:>6.3} y={:>7.4} nx={:>5.3} ny={:>5.3}  lift={}",
                        pt.q, pt.x, pt.y, pt.nx, pt.ny, lift
                    );
                }
            }
        }
    }

    // Optional CSV export
    if let Some(path) = csv {
        let f = File::create(&path).expect("Failed to create CSV file");
        let mut w = BufWriter::new(f);
        writeln!(
            w,
            "q,p,alpha_label,x,y,nx,ny,score_min,score_prod,err,core,v2,v5,phi,lam,ord,purity,utilization,slippage"
        )
        .unwrap();
        for pt in dps.iter() {
            let m = &pt.mat;
            writeln!(
                w,
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                pt.q,
                pt.p,
                alpha_label,
                pt.x,
                pt.y,
                pt.nx,
                pt.ny,
                pt.score_min,
                pt.score_prod,
                pt.err,
                m.core,
                m.v2,
                m.v5,
                m.phi,
                m.lam,
                m.ord,
                m.purity,
                m.utilization,
                m.slippage
            )
            .unwrap();
        }
        println!("\nWrote {}", path);
    }
}
