// GUE Spacing Distribution Analysis
//
// Testing if coordinate constellation prime spacings follow the
// Gaussian Unitary Ensemble (GUE) level spacing distribution from
// Random Matrix Theory:
//
//   P(s) = (π/2) s e^(-πs²/4)
//
// CONNECTION TO MIDPOINT REPULSION:
// The Agda MidpointOrbitals formalization suggests eigenvalue-like
// repulsion around the midpoint. If primes behave like eigenvalues
// of random matrices, their spacings should follow GUE statistics.
//
// HYPOTHESIS:
// For coordinate constellations, especially with φ(base)=6 hexagonal
// structure, prime spacings may show GUE-like repulsion rather than
// Poisson (random) distribution.
//
// TEST METHODOLOGY:
// 1. Generate coordinate constellation primes
// 2. Compute normalized spacings s_i = (gap_i / mean_gap)
// 3. Compare histogram to GUE: P(s) = (π/2) s e^(-πs²/4)
// 4. Also compare to Poisson: P(s) = e^(-s)
// 5. Use KS test and χ² test for statistical validation

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime;
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

fn quintuplet_membrane(middle: u32, x: u32, y: u32, base: u32) -> BigUint {
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    result = result * &base_big + BigUint::from(y);
    result = result * &base_big + BigUint::from(x);
    result = result * &base_big + BigUint::from(middle);
    result = result * &base_big + BigUint::from(x);
    result = result * &base_big + BigUint::from(y);

    result
}

fn collect_constellation_primes(base: u32, middle: u32, limit: u64) -> Vec<BigUint> {
    let mut primes = Vec::new();

    for y in 1..base {
        for x in 1..base {
            let candidate = quintuplet_membrane(middle, x, y, base);

            if candidate > BigUint::from(limit) {
                continue;
            }

            if is_prime(&candidate) {
                primes.push(candidate);
            }
        }
    }

    primes.sort();
    primes
}

fn compute_spacings(primes: &[BigUint]) -> Vec<f64> {
    if primes.len() < 2 {
        return Vec::new();
    }

    let mut gaps = Vec::new();
    for i in 0..primes.len() - 1 {
        // Convert gap to f64 for statistical analysis
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
    // P(s) = (π/2) s e^(-πs²/4)
    (PI / 2.0) * s * (-PI * s * s / 4.0).exp()
}

fn poisson_pdf(s: f64) -> f64 {
    // P(s) = e^(-s)
    (-s).exp()
}

fn create_histogram(spacings: &[f64], bins: usize) -> (Vec<f64>, Vec<f64>) {
    let max_spacing = spacings.iter().cloned().fold(0.0f64, f64::max);
    let bin_width = max_spacing / bins as f64;

    let mut histogram = vec![0.0; bins];
    let mut bin_centers = Vec::new();

    for &s in spacings {
        let bin = ((s / bin_width) as usize).min(bins - 1);
        histogram[bin] += 1.0;
    }

    // Normalize to probability density
    let total = spacings.len() as f64;
    for i in 0..bins {
        histogram[i] = histogram[i] / (total * bin_width);
        bin_centers.push((i as f64 + 0.5) * bin_width);
    }

    (bin_centers, histogram)
}

fn chi_squared_test(
    observed: &[f64],
    expected_fn: fn(f64) -> f64,
    bin_centers: &[f64],
    bin_width: f64,
) -> f64 {
    let mut chi2 = 0.0;
    let n_total = observed.iter().sum::<f64>() * bin_width; // Convert density back to counts

    for i in 0..observed.len() {
        let obs_count = observed[i] * bin_width * n_total;
        let exp_count = expected_fn(bin_centers[i]) * bin_width * n_total;

        if exp_count > 0.0 {
            chi2 += (obs_count - exp_count).powi(2) / exp_count;
        }
    }

    chi2
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

fn gue_cdf(s: f64) -> f64 {
    // Approximate CDF for GUE: integrate P(t) from 0 to s
    // Using numerical integration
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

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║         GUE SPACING DISTRIBUTION ANALYSIS                    ║");
    println!("║         Random Matrix Theory ↔ Prime Repulsion               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    println!("THEORETICAL BACKGROUND:");
    println!("  GUE (Gaussian Unitary Ensemble) from Random Matrix Theory:");
    println!("    P(s) = (π/2) s e^(-πs²/4)");
    println!("    → Level repulsion: rare small spacings (s→0 gives P→0)");
    println!();
    println!("  Poisson (random/uncorrelated):");
    println!("    P(s) = e^(-s)");
    println!("    → No repulsion: small spacings common");
    println!();
    println!("  CONNECTION: Midpoint void ↔ eigenvalue repulsion?");
    println!();

    let bases_to_test = vec![
        (6, 1, "Base 6 (φ=2, high success)"),
        (7, 1, "Base 7 (φ=6, record 21.30%)"),
        (14, 1, "Base 14 (φ=6, hexagonal)"),
        (18, 1, "Base 18 (φ=6, hexagonal)"),
    ];

    let limit = 1_000_000_000_000u64;

    println!("═══════════════════════════════════════════════════════════════");
    println!("COLLECTING CONSTELLATION PRIMES");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    for (base, middle, description) in &bases_to_test {
        println!("─────────────────────────────────────────────────────────────");
        println!("{}", description);
        println!("─────────────────────────────────────────────────────────────");
        println!();

        println!("  Collecting primes for base {}...", base);
        let primes = collect_constellation_primes(*base, *middle, limit);
        println!("  Found {} primes", primes.len());

        if primes.len() < 10 {
            println!("  ⚠ Too few primes for statistical analysis");
            println!();
            continue;
        }

        println!();

        // Compute spacings
        let spacings = compute_spacings(&primes);
        println!("  Computed {} spacings", spacings.len());
        println!();

        // Basic statistics
        let mean = spacings.iter().sum::<f64>() / spacings.len() as f64;
        let variance =
            spacings.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / spacings.len() as f64;
        let std_dev = variance.sqrt();

        println!("  SPACING STATISTICS:");
        println!("    Mean: {:.4} (normalized to 1.0)", mean);
        println!("    Std dev: {:.4}", std_dev);
        println!(
            "    Min spacing: {:.4}",
            spacings.iter().cloned().fold(f64::INFINITY, f64::min)
        );
        println!(
            "    Max spacing: {:.4}",
            spacings.iter().cloned().fold(0.0f64, f64::max)
        );
        println!();

        // Count very small spacings (repulsion test)
        let small_threshold = 0.2;
        let small_count = spacings.iter().filter(|&&s| s < small_threshold).count();
        let small_fraction = small_count as f64 / spacings.len() as f64;

        println!("  REPULSION TEST:");
        println!(
            "    Spacings < {}: {} ({:.1}%)",
            small_threshold,
            small_count,
            small_fraction * 100.0
        );
        println!("    GUE predicts: {:.1}%", gue_cdf(small_threshold) * 100.0);
        println!(
            "    Poisson predicts: {:.1}%",
            poisson_cdf(small_threshold) * 100.0
        );
        println!();

        if small_fraction < 0.1 {
            println!("    ✓ REPULSION DETECTED: Few small spacings (GUE-like)");
        } else if small_fraction < 0.18 {
            println!("    ~ MODERATE REPULSION: Some clustering");
        } else {
            println!("    ✗ NO REPULSION: Random Poisson-like");
        }
        println!();

        // Create histogram
        let bins = 20;
        let (bin_centers, histogram) = create_histogram(&spacings, bins);
        let bin_width = if bin_centers.len() > 1 {
            bin_centers[1] - bin_centers[0]
        } else {
            1.0
        };

        // Chi-squared tests
        let chi2_gue = chi_squared_test(&histogram, gue_pdf, &bin_centers, bin_width);
        let chi2_poisson = chi_squared_test(&histogram, poisson_pdf, &bin_centers, bin_width);

        println!("  GOODNESS-OF-FIT TESTS:");
        println!("    χ² statistic vs GUE: {:.2}", chi2_gue);
        println!("    χ² statistic vs Poisson: {:.2}", chi2_poisson);

        if chi2_gue < chi2_poisson {
            println!(
                "    → Better fit to GUE (ratio: {:.2})",
                chi2_poisson / chi2_gue
            );
        } else {
            println!(
                "    → Better fit to Poisson (ratio: {:.2})",
                chi2_gue / chi2_poisson
            );
        }
        println!();

        // Kolmogorov-Smirnov test
        let ks_gue = ks_statistic(&spacings, gue_cdf);
        let ks_poisson = ks_statistic(&spacings, poisson_cdf);

        println!("  KOLMOGOROV-SMIRNOV TESTS:");
        println!("    KS statistic vs GUE: {:.4}", ks_gue);
        println!("    KS statistic vs Poisson: {:.4}", ks_poisson);

        if ks_gue < ks_poisson {
            println!("    → Better fit to GUE");
        } else {
            println!("    → Better fit to Poisson");
        }
        println!();

        // Histogram visualization
        println!("  SPACING DISTRIBUTION HISTOGRAM:");
        println!();
        println!("   s     Obs   GUE  Poi");

        for i in 0..bins.min(15) {
            // Show first 15 bins
            let s = bin_centers[i];
            let obs = histogram[i];
            let gue = gue_pdf(s);
            let poi = poisson_pdf(s);

            let scale = 20.0;
            let obs_bar = "█".repeat((obs * scale) as usize);
            let gue_bar = "▓".repeat((gue * scale) as usize);
            let poi_bar = "░".repeat((poi * scale) as usize);

            println!("  {:.2}  {} {:.3}", s, obs_bar, obs);
            println!("        {} (GUE)", gue_bar);
            println!("        {} (Poi)", poi_bar);
        }
        println!();
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("INTERPRETATION");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("GUE-LIKE BEHAVIOR INDICATORS:");
    println!("  1. Few very small spacings (repulsion)");
    println!("  2. Peak at s ≈ 1 (normalized mean)");
    println!("  3. χ²(GUE) < χ²(Poisson)");
    println!("  4. KS(GUE) < KS(Poisson)");
    println!();

    println!("POISSON BEHAVIOR INDICATORS:");
    println!("  1. Many small spacings (no repulsion)");
    println!("  2. Peak at s = 0 (exponential decay)");
    println!("  3. χ²(Poisson) < χ²(GUE)");
    println!("  4. KS(Poisson) < KS(GUE)");
    println!();

    println!("CONNECTION TO MIDPOINT REPULSION:");
    println!("  If GUE-like → Prime spacings exhibit repulsion");
    println!("  If Poisson → Prime spacings are uncorrelated (random)");
    println!();
    println!("  The Agda MidpointOrbitals 'honorary zero' and Roche zone");
    println!("  suggest eigenvalue-like repulsion in coordinate space.");
    println!();
    println!("  Hexagonal structure (φ(base)=6) may enhance repulsion");
    println!("  through geometric constraints on allowed coordinates.");
    println!();
}
