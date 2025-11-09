// GUE Spacing Distribution Analysis - Septuplet Version
//
// Uses septuplet constellations with multiple middle values to get
// statistically significant sample sizes for GUE vs Poisson testing.
//
// HYPOTHESIS:
// With ~800+ primes from septuplets, we can robustly test whether
// coordinate constellation spacings follow:
//   - GUE: P(s) = (π/2) s e^(-πs²/4)  [eigenvalue repulsion]
//   - Poisson: P(s) = e^(-s)          [random uncorrelated]

use num_bigint::BigUint;
use num_traits::Zero;
use prime_physics_engine::is_prime;
use std::f64::consts::PI;

fn is_coprime(a: u32, b: u32) -> bool {
    fn gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    gcd(a, b) == 1
}

fn septuplet_membrane(middle: u32, x: u32, y: u32, z: u32, base: u32) -> BigUint {
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    result = result * &base_big + BigUint::from(z);
    result = result * &base_big + BigUint::from(y);
    result = result * &base_big + BigUint::from(x);
    result = result * &base_big + BigUint::from(middle);
    result = result * &base_big + BigUint::from(x);
    result = result * &base_big + BigUint::from(y);
    result = result * &base_big + BigUint::from(z);

    result
}

fn collect_septuplet_primes(base: u32, middle_values: &[u32], limit: u64) -> Vec<BigUint> {
    let mut primes = Vec::new();

    for &middle in middle_values {
        for z in 1..base {
            for y in 1..base {
                for x in 1..base {
                    let candidate = septuplet_membrane(middle, x, y, z, base);

                    if candidate > BigUint::from(limit) {
                        continue;
                    }

                    if is_prime(&candidate) {
                        primes.push(candidate);
                    }
                }
            }
        }
    }

    primes.sort();
    primes.dedup(); // Remove any duplicates
    primes
}

fn compute_spacings(primes: &[BigUint]) -> Vec<f64> {
    if primes.len() < 2 {
        return Vec::new();
    }

    let mut gaps = Vec::new();
    for i in 0..primes.len() - 1 {
        let gap = (&primes[i + 1] - &primes[i])
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);
        gaps.push(gap);
    }

    // Normalize by mean gap
    let mean_gap = gaps.iter().sum::<f64>() / gaps.len() as f64;
    gaps.iter().map(|&g| g / mean_gap).collect()
}

fn gue_pdf(s: f64) -> f64 {
    (PI / 2.0) * s * (-PI * s * s / 4.0).exp()
}

fn poisson_pdf(s: f64) -> f64 {
    (-s).exp()
}

fn gue_cdf(s: f64) -> f64 {
    // Numerical integration of GUE PDF
    let steps = 100;
    let ds = s / steps as f64;
    let mut sum = 0.0;
    for i in 0..steps {
        let t = (i as f64 + 0.5) * ds;
        sum += gue_pdf(t) * ds;
    }
    sum
}

fn poisson_cdf(s: f64) -> f64 {
    1.0 - (-s).exp()
}

fn ks_statistic(spacings: &[f64], cdf_fn: fn(f64) -> f64) -> f64 {
    let mut sorted = spacings.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = sorted.len() as f64;
    let mut max_diff: f64 = 0.0;

    for (i, &s) in sorted.iter().enumerate() {
        let empirical_cdf = (i + 1) as f64 / n;
        let theoretical_cdf = cdf_fn(s);
        let diff = (empirical_cdf - theoretical_cdf).abs();
        max_diff = max_diff.max(diff);
    }

    max_diff
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║      GUE SPACING ANALYSIS - SEPTUPLET VERSION                ║");
    println!("║      High-Statistics Test of Prime Repulsion                 ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    println!("TESTING WITH SEPTUPLETS:");
    println!("  Structure: z-y-x-MIDDLE-x-y-z");
    println!("  Using multiple middle values for large sample sizes");
    println!("  Expected: ~800+ primes for base 14");
    println!();

    let base = 14u32;
    let limit = 1_000_000_000_000u64;

    // Use first 6 coprime middle values
    let middle_values: Vec<u32> = (1..base).filter(|&m| is_coprime(m, base)).take(6).collect();

    println!("═══════════════════════════════════════════════════════════════");
    println!("BASE {} - HEXAGONAL STRUCTURE (φ=6)", base);
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("  Testing middle values: {:?}", middle_values);
    println!();

    println!("  Collecting septuplet primes...");
    let primes = collect_septuplet_primes(base, &middle_values, limit);
    println!("  Found {} unique primes", primes.len());
    println!();

    if primes.len() < 50 {
        println!("  ⚠ Still too few primes for robust statistics");
        println!("  Need at least 50, preferably 100+");
        return;
    }

    // Compute spacings
    let spacings = compute_spacings(&primes);
    println!("  Computed {} spacings", spacings.len());
    println!();

    // === BASIC STATISTICS ===
    println!("═══════════════════════════════════════════════════════════════");
    println!("SPACING STATISTICS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    let mean = spacings.iter().sum::<f64>() / spacings.len() as f64;
    let variance = spacings.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / spacings.len() as f64;
    let std_dev = variance.sqrt();
    let min_s = spacings.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_s = spacings.iter().cloned().fold(0.0f64, f64::max);

    println!("  Mean spacing: {:.4} (normalized to 1.0)", mean);
    println!("  Standard deviation: {:.4}", std_dev);
    println!("  Min spacing: {:.6}", min_s);
    println!("  Max spacing: {:.4}", max_s);
    println!();

    // GUE predicts std ≈ 0.52, Poisson predicts std = 1.0
    println!("  VARIANCE TEST:");
    println!("    Observed std: {:.4}", std_dev);
    println!("    GUE predicts: ~0.52 (level repulsion reduces variance)");
    println!("    Poisson predicts: ~1.00 (exponential distribution)");

    let var_ratio_gue = (std_dev - 0.52).abs();
    let var_ratio_poi = (std_dev - 1.00).abs();

    if var_ratio_gue < var_ratio_poi {
        println!("    → Closer to GUE (difference: {:.3})", var_ratio_gue);
    } else {
        println!("    → Closer to Poisson (difference: {:.3})", var_ratio_poi);
    }
    println!();

    // === REPULSION TEST ===
    println!("═══════════════════════════════════════════════════════════════");
    println!("REPULSION TEST (Small Spacing Depletion)");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    let thresholds = [0.1, 0.2, 0.3, 0.5];
    println!("  Fraction of spacings below threshold:");
    println!();
    println!("  Threshold │ Observed │   GUE   │ Poisson │  Winner");
    println!("  ──────────┼──────────┼─────────┼─────────┼─────────");

    for &thresh in &thresholds {
        let count = spacings.iter().filter(|&&s| s < thresh).count();
        let frac = count as f64 / spacings.len() as f64;
        let gue_pred = gue_cdf(thresh);
        let poi_pred = poisson_cdf(thresh);

        let diff_gue = (frac - gue_pred).abs();
        let diff_poi = (frac - poi_pred).abs();
        let winner = if diff_gue < diff_poi {
            "GUE ✓"
        } else {
            "Poisson"
        };

        println!(
            "   {:.2}     │  {:5.1}%  │  {:5.1}% │  {:5.1}% │  {}",
            thresh,
            frac * 100.0,
            gue_pred * 100.0,
            poi_pred * 100.0,
            winner
        );
    }
    println!();

    if min_s > 0.01 {
        println!(
            "  ✓ STRONG REPULSION: Minimum spacing = {:.6} (no near collisions)",
            min_s
        );
    } else if min_s > 0.001 {
        println!("  ~ MODERATE REPULSION: Minimum spacing = {:.6}", min_s);
    } else {
        println!(
            "  ✗ WEAK REPULSION: Very small gaps exist (min = {:.6})",
            min_s
        );
    }
    println!();

    // === KOLMOGOROV-SMIRNOV TEST ===
    println!("═══════════════════════════════════════════════════════════════");
    println!("KOLMOGOROV-SMIRNOV GOODNESS-OF-FIT TEST");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    let ks_gue = ks_statistic(&spacings, gue_cdf);
    let ks_poisson = ks_statistic(&spacings, poisson_cdf);

    println!("  KS statistic (lower is better fit):");
    println!("    vs GUE:     {:.4}", ks_gue);
    println!("    vs Poisson: {:.4}", ks_poisson);
    println!();

    let ks_improvement = (ks_poisson - ks_gue) / ks_poisson * 100.0;

    if ks_gue < ks_poisson {
        println!("  ✓ BETTER FIT TO GUE");
        println!("    GUE is {:.1}% better than Poisson", ks_improvement);
    } else {
        println!("  ✗ BETTER FIT TO POISSON");
        println!("    Poisson is {:.1}% better than GUE", -ks_improvement);
    }
    println!();

    // Critical values for KS test (approximate, n=large)
    let n = spacings.len() as f64;
    let ks_critical_01 = 1.63 / n.sqrt(); // α=0.01
    let ks_critical_05 = 1.36 / n.sqrt(); // α=0.05

    println!("  Statistical significance (α = significance level):");
    println!("    Critical value (α=0.05): {:.4}", ks_critical_05);
    println!("    Critical value (α=0.01): {:.4}", ks_critical_01);
    println!();

    if ks_gue < ks_critical_05 {
        println!("    GUE: ✓ Cannot reject (good fit at α=0.05)");
    } else {
        println!("    GUE: ✗ Reject (poor fit at α=0.05)");
    }

    if ks_poisson < ks_critical_05 {
        println!("    Poisson: ✓ Cannot reject (good fit at α=0.05)");
    } else {
        println!("    Poisson: ✗ Reject (poor fit at α=0.05)");
    }
    println!();

    // === HISTOGRAM VISUALIZATION ===
    println!("═══════════════════════════════════════════════════════════════");
    println!("SPACING DISTRIBUTION HISTOGRAM");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    let bins = 25;
    let bin_width = max_s / bins as f64;
    let mut histogram = vec![0usize; bins];

    for &s in &spacings {
        let bin = ((s / bin_width) as usize).min(bins - 1);
        histogram[bin] += 1;
    }

    let max_count = *histogram.iter().max().unwrap_or(&1);
    let bar_scale = 50.0 / max_count as f64;

    println!("   Spacing  │ Count │ Observed          │ GUE               │ Poisson");
    println!("  ──────────┼───────┼───────────────────┼───────────────────┼─────────────────");

    for i in 0..bins.min(20) {
        let s = (i as f64 + 0.5) * bin_width;
        let count = histogram[i];
        let obs_density = count as f64 / (spacings.len() as f64 * bin_width);

        let gue_val = gue_pdf(s);
        let poi_val = poisson_pdf(s);

        let obs_bar = "█".repeat((count as f64 * bar_scale) as usize);
        let gue_bar = "▓".repeat((gue_val * obs_density.max(0.1) * 10.0) as usize);
        let poi_bar = "░".repeat((poi_val * obs_density.max(0.1) * 10.0) as usize);

        println!(
            "   {:.3}    │  {:4} │ {:17} │ {:17} │ {}",
            s, count, obs_bar, gue_bar, poi_bar
        );
    }
    println!();

    // === FINAL VERDICT ===
    println!("═══════════════════════════════════════════════════════════════");
    println!("FINAL VERDICT");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    let gue_score = if ks_gue < ks_poisson { 1.0 } else { 0.0 }
        + if var_ratio_gue < var_ratio_poi {
            1.0
        } else {
            0.0
        }
        + if min_s > 0.01 { 1.0 } else { 0.5 };

    let poisson_score = if ks_poisson < ks_gue { 1.0 } else { 0.0 }
        + if var_ratio_poi < var_ratio_gue {
            1.0
        } else {
            0.0
        };

    println!("  Evidence summary:");
    println!("    GUE score: {:.1}/3.0", gue_score);
    println!("    Poisson score: {:.1}/2.0", poisson_score);
    println!();

    if gue_score > poisson_score + 0.5 {
        println!("  ⭐ STRONG EVIDENCE FOR GUE-LIKE REPULSION");
        println!("     Coordinate constellation primes exhibit eigenvalue-like spacing!");
    } else if gue_score > poisson_score {
        println!("  ~ MODERATE EVIDENCE FOR GUE-LIKE REPULSION");
        println!("    Some repulsion present, but not fully GUE-like");
    } else {
        println!("  ✗ INSUFFICIENT EVIDENCE FOR GUE REPULSION");
        println!("    Spacings appear more Poisson-like (uncorrelated)");
    }
    println!();

    println!("CONNECTION TO HEXAGONAL STRUCTURE:");
    println!("  φ(base)=6 → 6 coprime coordinates → hexagonal lattice");
    println!("  Honorary zero → void at midpoint → repulsion mechanism");
    println!("  If GUE-like → geometric constraints create eigenvalue repulsion");
    println!("  If Poisson → constraints insufficient for full correlation");
    println!();
}
