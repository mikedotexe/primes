//! Phase 2: Midpoint Density Analysis with δ* Drift
//!
//! Analyzes how peak density position (δ*) changes across digit-length bands.
//! Tests the hypothesis that δ* grows linearly with k (digit length).
//!
//! Key metrics:
//! - δ* (peak delta): Position of maximum prime density
//! - Slope of δ* vs k: Rate of drift per digit
//! - R² correlation: How well linear model fits
//! - Confidence intervals: 95% CI on slope
//! - Correlation with membrane success: Spearman ρ
//!
//! Statistical enhancements:
//! - Confidence intervals on regression slopes
//! - Relative δ* (fraction of half-band width)
//! - Correlation analysis: δ* slope vs membrane success rate
//!
//! Usage:
//! ```bash
//! cargo run --example hz_phase2_density -- --bases 6,30,10 --limit 200000000 --bins 200
//! ```

use primes::hzlib::*;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::time::Instant;

#[derive(Clone)]
struct Args {
    limit: usize,
    bins: usize,
    max_base: usize,
    bases: Option<Vec<usize>>,
}

fn parse_args() -> Args {
    let mut limit = 20_000_000usize;
    let mut bins = 200usize;
    let mut max_base = 2000usize;
    let mut bases: Option<Vec<usize>> = None;

    let mut it = env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--limit" => if let Some(v) = it.next() { limit = v.parse().unwrap(); }
            "--bins" => if let Some(v) = it.next() { bins = v.parse().unwrap(); }
            "--max-base" => if let Some(v) = it.next() { max_base = v.parse().unwrap(); }
            "--bases" => if let Some(v) = it.next() {
                bases = Some(v.split(',')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse().unwrap())
                    .collect());
            }
            _ => {}
        }
    }

    Args {
        limit,
        bins: bins.clamp(10, 2000),
        max_base,
        bases,
    }
}

fn main() {
    let t0 = Instant::now();
    let args = parse_args();

    // Determine bases to test
    let bases: Vec<usize> = if let Some(bs) = &args.bases {
        let mut v = bs.clone();
        v.sort_unstable();
        v.dedup();
        v
    } else {
        // Default: double-prime bases + control bases
        let mut v: Vec<usize> = (4..=args.max_base)
            .step_by(2)
            .filter(|b| is_double_prime_base(*b))
            .collect();

        for &c in &[10usize, 12, 16, 30] {
            if c <= args.max_base {
                v.push(c);
            }
        }

        v.sort_unstable();
        v.dedup();
        v
    };

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║         PHASE 2: δ* DRIFT ANALYSIS (MIDPOINT DENSITY)     ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("Configuration:");
    println!("  Limit: {}", args.limit);
    println!("  Bins per band: {}", args.bins);
    println!("  Bases to test: {}", bases.len());
    println!("  Bases: {:?}", bases);
    println!();

    // Create accumulator for each base
    let mut models: Vec<BaseAccum> = bases
        .iter()
        .map(|&b| BaseAccum::new(b, args.limit, args.bins))
        .filter(|m| !m.bands.is_empty())
        .collect();

    if models.is_empty() {
        eprintln!("ERROR: No bases had complete bands under limit={}", args.limit);
        eprintln!("Try increasing --limit or decreasing --max-base");
        return;
    }

    println!("Sieving primes up to {}...", args.limit);
    let mut prime_total = 0usize;

    segmented_sieve(args.limit, |p| {
        prime_total += 1;
        for m in models.iter_mut() {
            if let Some(idx) = m.find_band(p) {
                m.bands[idx].add_prime(p);
            }
        }
    });

    println!("  ✓ Found {} primes", prime_total);
    println!();

    // Create output directory
    fs::create_dir_all("hz_out").unwrap();

    // Write detailed bin-level data
    let mut f_bins = BufWriter::new(File::create("hz_out/density_bins.csv").unwrap());
    writeln!(f_bins, "base,k,bin,delta_center,prime_count,denom_count,density").unwrap();

    // Write per-band summary
    let mut f_band = BufWriter::new(File::create("hz_out/band_summary.csv").unwrap());
    writeln!(f_band, "base,k,primes_in_band,peak_delta,peak_density,com_delta,com_density").unwrap();

    // Write per-base regression results
    let mut f_base = BufWriter::new(File::create("hz_out/base_summary.csv").unwrap());
    writeln!(f_base, "base,is_double_prime,k_count,slope,intercept,r2,slope_ci,slope_relative,delta_star_min,delta_star_max").unwrap();

    println!("Analyzing density distributions...");

    for m in &models {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        let mut dmin = f64::INFINITY;
        let mut dmax = f64::NEG_INFINITY;

        // Process each band
        for bd in &m.bands {
            // Write bin-level data
            for bi in 0..bd.bins {
                let dc = (bi as f64 + 0.5) / bd.bins as f64;
                let pc = bd.counts[bi];
                let den = bd.denom[bi];
                let dens = if den == 0 { 0.0 } else { pc as f64 / den as f64 };

                writeln!(f_bins, "{},{},{},{:.9},{},{},{:.12}",
                    m.b, bd.k, bi, dc, pc, den, dens).unwrap();
            }

            // Compute peak and COM
            let (pd, pval, _) = bd.peak_delta();
            let (cm, cmass) = bd.com_delta();

            writeln!(f_band, "{},{},{},{:.9},{:.12},{:.9},{:.12}",
                m.b, bd.k, bd.primes_in_band(), pd, pval, cm, cmass).unwrap();

            // Track for regression
            xs.push(bd.k as f64);
            ys.push(pd);

            if pd < dmin { dmin = pd; }
            if pd > dmax { dmax = pd; }
        }

        // Linear regression with confidence intervals
        let (slope, intercept, r2, slope_ci, _intercept_ci, _residual_se) =
            linreg_with_ci(&xs, &ys, 0.95);

        // Relative slope: slope per half-band width (bins/2)
        // Use first band's bin count (all bands have same bins)
        let half_band = if !m.bands.is_empty() {
            (m.bands[0].bins as f64) / 2.0
        } else {
            100.0 // fallback
        };
        let slope_relative = slope / half_band;

        writeln!(f_base, "{},{},{},{:.12},{:.12},{:.6},{:.12},{:.12},{:.9},{:.9}",
            m.b,
            if m.is_double_prime { 1 } else { 0 },
            xs.len(),
            slope,
            intercept,
            r2,
            slope_ci,
            slope_relative,
            dmin,
            dmax
        ).unwrap();

        // Print summary for this base
        println!("  Base {}: {} bands, slope={:.6} ± {:.6}, R²={:.4}",
            m.b, xs.len(), slope, slope_ci, r2);
    }

    drop(f_bins);
    drop(f_band);
    drop(f_base);

    println!();
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                    RESULTS SUMMARY                         ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("Statistical Enhancements:");
    println!("  • Slope confidence intervals (95% CI)");
    println!("  • Relative slopes (per half-band width)");
    println!("  • Ready for correlation analysis with membrane success");
    println!();
    println!("Output files created in ./hz_out/:");
    println!("  • density_bins.csv    - Per-bin density values");
    println!("  • band_summary.csv    - Peak δ* and COM per band");
    println!("  • base_summary.csv    - δ* vs k regression with CI");
    println!();
    println!("CSV Columns in base_summary.csv:");
    println!("  • slope: Absolute slope of δ* vs k");
    println!("  • slope_ci: 95% confidence interval half-width");
    println!("  • slope_relative: Slope per half-band width (comparable across bases)");
    println!();
    println!("Total runtime: {:.2?}", t0.elapsed());
    println!();
    println!("Next steps:");
    println!("  1. Correlate slope with membrane success using Spearman ρ");
    println!("  2. Compare slopes: expect base 6 > base 30 if clustering hypothesis holds");
    println!("  3. Check slope_ci: tight CI → reliable slope estimate");
    println!("  4. Use slope_relative to compare across different bin counts");
}
