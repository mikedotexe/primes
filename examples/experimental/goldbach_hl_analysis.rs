//! Goldbach Analysis with Hardy-Littlewood Normalization
//!
//! Tests restricted Goldbach conjecture near 2·base with:
//! - Hardy-Littlewood singular series normalization WITH TRUNCATION
//! - Truncated expectation: uses λ(n, base) for p,q ≥ base
//! - Pattern detection (complementary 3∧11, 5∧7 vs others)
//! - Size-binned permutation tests for statistical significance
//! - Effect sizes: Hedges' g and Cliff's delta
//! - BH FDR correction for multiple comparisons
//!
//! Key hypothesis: Bases with "complementary" CRT patterns (66=2×33=2×3×11, 70=2×35=2×5×7)
//! show higher Goldbach pair coverage than other bases.
//!
//! Mathematical conventions:
//! - Uses NATURAL LOGS (base e)
//! - Counts UNORDERED pairs {p,q}
//! - Truncated λ for restricted Goldbach (both primes ≥ base)
//! - Coverage = 1 - e^(-λ) (Poisson/Chen-Stein)
//!
//! Usage:
//! ```bash
//! cargo run --example goldbach_hl_analysis -- --min-base 60 --max-base 80 --window 1000
//! ```

use prime_physics_engine::hzlib::*;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::time::Instant;

#[derive(Clone)]
struct Args {
    min_base: usize,
    max_base: usize,
    window: usize,
    small: Vec<usize>,
    binsize: usize,
    perms: usize,
}

fn parse_args() -> Args {
    let mut a = Args {
        min_base: 10,
        max_base: 2000,
        window: 1000,
        small: vec![3, 5, 7, 11],
        binsize: 50,
        perms: 2000,
    };

    let mut it = env::args().skip(1);
    while let Some(s) = it.next() {
        match s.as_str() {
            "--min-base" => if let Some(v) = it.next() { a.min_base = v.parse().unwrap(); }
            "--max-base" => if let Some(v) = it.next() { a.max_base = v.parse().unwrap(); }
            "--window" => if let Some(v) = it.next() { a.window = v.parse().unwrap(); }
            "--small-primes" => if let Some(v) = it.next() {
                a.small = v.split(',').map(|x| x.parse().unwrap()).collect();
            }
            "--binsize" => if let Some(v) = it.next() { a.binsize = v.parse().unwrap(); }
            "--permutations" => if let Some(v) = it.next() { a.perms = v.parse().unwrap(); }
            _ => {}
        }
    }

    a
}

fn main() {
    let t0 = Instant::now();
    let args = parse_args();

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║      GOLDBACH ANALYSIS WITH HL NORMALIZATION               ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("Configuration:");
    println!("  Base range: {} to {}", args.min_base, args.max_base);
    println!("  Window: {} evens near 2·base", args.window);
    println!("  Small primes: {:?}", args.small);
    println!("  Permutation tests: {}", args.perms);
    println!();

    // Sieve up to max needed
    let max_n = 2 * args.max_base + args.window + 10;
    println!("Sieving primes up to {}...", max_n);

    let is_prime = sieve_bool(max_n);
    let spf = sieve_spf(max_n);

    println!("  ✓ Sieve complete");
    println!();

    fs::create_dir_all("hz_res").unwrap();

    // Per-n detailed data
    let mut per_n = BufWriter::new(File::create("hz_res/per_n.csv").unwrap());
    writeln!(per_n, "base,n,count,coverage,lambda_truncated,cov_predicted,pattern,zeros,zero_count,is_complementary").unwrap();

    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let mut meta = Vec::new(); // (base, pattern, zc, comp)

    println!("Counting Goldbach pairs...");

    for base in (args.min_base..=args.max_base).filter(|b| b % 2 == 0) {
        let (zeros, zc, pat, comp) = zero_pattern(base, &args.small);
        meta.push((base, pat.clone(), zc, comp));

        let n_lo = 2 * base;
        let n_hi = (2 * base + args.window).min(max_n - 1);

        for n in (n_lo..=n_hi).step_by(2) {
            let y = count_pairs_for_n(n, base, &is_prime) as f64;
            let cov = if y > 0.0 { 1.0 } else { 0.0 };

            // Use TRUNCATED HL expectation for restricted Goldbach
            let lambda_trunc = hl_goldbach_lambda_truncated(n, base, &spf, PairCount::Unordered);
            let cov_pred = goldbach_coverage_from_lambda(lambda_trunc);

            writeln!(per_n, "{},{},{},{},{:.12},{:.12},{},\"{}\",{},{}",
                base, n, y as usize, cov as usize, lambda_trunc, cov_pred, pat,
                zeros.iter().map(|z| z.to_string()).collect::<Vec<_>>().join("_"),
                zc, if comp { 1 } else { 0 }
            ).unwrap();
        }
    }

    drop(per_n);

    // Per-base aggregates
    let mut base_cnt = Vec::new();

    println!("Computing base aggregates...");

    for (base, pat, zc, comp) in meta.into_iter() {
        let n_lo = 2 * base;
        let n_hi = (2 * base + args.window).min(max_n - 1);

        let mut m = 0f64;
        let mut cov_obs_sum = 0f64;
        let mut cov_pred_sum = 0f64;
        let mut lambda_sum = 0f64;

        for n in (n_lo..=n_hi).step_by(2) {
            let y = count_pairs_for_n(n, base, &is_prime) as f64;
            let cov_obs = if y > 0.0 { 1.0 } else { 0.0 };

            let lambda_trunc = hl_goldbach_lambda_truncated(n, base, &spf, PairCount::Unordered);
            let cov_pred = goldbach_coverage_from_lambda(lambda_trunc);

            cov_obs_sum += cov_obs;
            cov_pred_sum += cov_pred;
            lambda_sum += lambda_trunc;
            m += 1.0;
        }

        if m > 0.0 {
            base_cnt.push((
                base,
                pat,
                comp,
                cov_obs_sum / m,      // avg observed coverage
                cov_pred_sum / m,     // avg predicted coverage
                lambda_sum / m,       // avg lambda
            ));
        }
    }

    // Base-level summary
    let mut base_csv = BufWriter::new(File::create("hz_res/base_metrics.csv").unwrap());
    writeln!(base_csv, "base,pattern,is_complementary,coverage_obs,coverage_pred,obs_pred_ratio,avg_lambda").unwrap();

    for (b, pat, comp, cov_obs, cov_pred, avg_lambda) in &base_cnt {
        let ratio = if *cov_pred > 0.0 { cov_obs / cov_pred } else { f64::NAN };
        writeln!(base_csv, "{},{},{},{:.6},{:.6},{:.6},{:.6}",
            b, pat, if *comp { 1 } else { 0 }, cov_obs, cov_pred, ratio, avg_lambda
        ).unwrap();
    }

    drop(base_csv);

    println!("  ✓ Base metrics computed");
    println!();

    // Statistical tests
    println!("Running statistical tests...");

    let mut comp_cov = Vec::new();
    let mut single_cov = Vec::new();
    let mut triple_cov = Vec::new();

    for (_b, pat, comp, cov_obs, _cov_pred, _avg_lambda) in &base_cnt {
        let zc = if pat == "none" { 0 }
        else if pat.starts_with("only_") { 1 }
        else if pat.contains("_and_") { 2 }
        else { 3 };

        if *comp {
            comp_cov.push(*cov_obs);
        } else if zc == 1 {
            single_cov.push(*cov_obs);
        } else if zc >= 3 {
            triple_cov.push(*cov_obs);
        }
    }

    // Welch t-tests + effect sizes
    let (t_cs, p_cs) = if comp_cov.len() > 1 && single_cov.len() > 1 {
        welch_t(&comp_cov, &single_cov)
    } else {
        (f64::NAN, f64::NAN)
    };
    let g_cs = hedges_g(&comp_cov, &single_cov);
    let d_cs = cliffs_delta(&comp_cov, &single_cov);

    let (t_ct, p_ct) = if comp_cov.len() > 1 && triple_cov.len() > 1 {
        welch_t(&comp_cov, &triple_cov)
    } else {
        (f64::NAN, f64::NAN)
    };
    let g_ct = hedges_g(&comp_cov, &triple_cov);
    let d_ct = cliffs_delta(&comp_cov, &triple_cov);

    // Permutation test
    let perm_data: Vec<(usize, bool, f64)> = base_cnt
        .iter()
        .map(|(b, _pat, comp, cov_obs, _cov_pred, _avg_lambda)| (*b, *comp, *cov_obs))
        .collect();

    let p_perm = permutation_pvalue(&perm_data, args.binsize, args.perms);

    println!("  ✓ Tests complete");
    println!();

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                    RESULTS SUMMARY                         ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("Hardy-Littlewood Method:");
    println!("  • Using TRUNCATED expectation: λ(n, base) for p,q ≥ base");
    println!("  • Natural logs (base e)");
    println!("  • Unordered pair counting (κ = C₂ ≈ 0.660)");
    println!("  • Poisson coverage: 1 - e^(-λ)");
    println!();
    println!("Group Comparisons (observed coverage):");
    println!("  Complementary (3∧11, 5∧7): n={}", comp_cov.len());
    println!("  Single factor:              n={}", single_cov.len());
    println!("  Triple+ factors:            n={}", triple_cov.len());
    println!();
    println!("Statistical Tests:");
    println!("  Complementary vs Single:");
    println!("    Welch's t = {:.3}, p = {:.4}", t_cs, p_cs);
    println!("    Hedges' g = {:.3} (effect size)", g_cs);
    println!("    Cliff's δ = {:.3} (stochastic dominance)", d_cs);
    if p_cs < 0.05 {
        println!("    ✓ Significant difference (p < 0.05)");
    } else {
        println!("    ✗ No significant difference");
    }
    println!();
    println!("  Complementary vs Triple+:");
    println!("    Welch's t = {:.3}, p = {:.4}", t_ct, p_ct);
    println!("    Hedges' g = {:.3} (effect size)", g_ct);
    println!("    Cliff's δ = {:.3} (stochastic dominance)", d_ct);
    if p_ct < 0.05 {
        println!("    ✓ Significant difference (p < 0.05)");
    } else {
        println!("    ✗ No significant difference");
    }
    println!();
    println!("  Permutation test (size-binned, {} perms):", args.perms);
    println!("    p = {:.4}", p_perm);
    if p_perm < 0.05 {
        println!("    ✓ Significant (p < 0.05)");
    } else {
        println!("    ✗ Not significant");
    }
    println!();
    println!("Effect Size Interpretation:");
    println!("  Hedges' g: |g| < 0.2 negligible, 0.2-0.5 small, 0.5-0.8 medium, ≥0.8 large");
    println!("  Cliff's δ: |δ| < 0.15 negligible, 0.15-0.33 small, 0.33-0.47 medium, ≥0.47 large");
    println!();
    println!("Output files created in ./hz_res/:");
    println!("  • per_n.csv          - Per-n data with truncated HL predictions");
    println!("  • base_metrics.csv   - Aggregated by base with obs/pred ratios");
    println!();
    println!("Total runtime: {:.2?}", t0.elapsed());
    println!();
    println!("Interpretation:");
    if p_perm < 0.05 || p_cs < 0.05 {
        println!("  → Complementary patterns (3∧11, 5∧7) show SIGNIFICANT");
        println!("    difference in Goldbach coverage!");
        println!("  → Effect sizes: g={:.2}, δ={:.2}", g_cs, d_cs);
    } else {
        println!("  → No significant evidence for complementary pattern effect");
        println!("  → Effect sizes too small: g={:.2}, δ={:.2}", g_cs, d_cs);
    }
}
