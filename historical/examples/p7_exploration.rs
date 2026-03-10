//! P₇ = 510510 Exploration
//!
//! Does the efficiency formula continue to hold for the 7th primorial?
//!
//! Prediction: efficiency ≈ 0.159 × ln(510510) + 3.66 ≈ 5.75

use num_bigint::BigUint;
use primes::is_prime_miller_rabin;

fn random_seed_with_length(base: u64, length: usize, rng: &mut u64) -> BigUint {
    if length == 0 { return BigUint::ZERO; }
    let mut next = || {
        *rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        *rng
    };
    let b = BigUint::from(base);
    let first = (next() % (base - 1)) + 1;
    let mut seed = BigUint::from(first);
    for _ in 1..length { seed = seed * &b + BigUint::from(next() % base); }
    seed
}

fn membrane_value(base: u64, left: u64, seed: &BigUint, right: u64) -> BigUint {
    let b = BigUint::from(base);
    let mut sd = 0u32;
    let mut t = seed.clone();
    while t > BigUint::ZERO { t /= &b; sd += 1; }
    if sd == 0 { sd = 1; }
    BigUint::from(left) * b.pow(sd + 1) + seed * &b + BigUint::from(right)
}

fn first_coprime(base: u64) -> u64 {
    let factors: Vec<u64> = {
        let mut n = base;
        let mut fs = vec![];
        for p in [2, 3, 5, 7, 11, 13, 17, 19, 23] {
            if n % p == 0 { fs.push(p); while n % p == 0 { n /= p; } }
        }
        fs
    };
    (1..base).find(|&d| factors.iter().all(|&p| d % p != 0)).unwrap_or(1)
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           P₇ = 510510 EFFICIENCY EXPLORATION                     ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // The primorials we're comparing
    let primorials: Vec<(u64, &str)> = vec![
        (30, "P₃ = 2×3×5"),
        (210, "P₄ = 2×3×5×7"),
        (2310, "P₅ = 2×3×5×7×11"),
        (30030, "P₆ = 2×3×5×7×11×13"),
        (510510, "P₇ = 2×3×5×7×11×13×17"),
    ];

    // Predicted efficiency formula
    let predict = |base: u64| -> f64 {
        0.159 * (base as f64).ln() + 3.66
    };

    println!("Efficiency formula: eff ≈ 0.159 × ln(base) + 3.66\n");
    println!("{:>10} {:>30} {:>12} {:>12}", "Base", "Name", "Predicted", "ln(base)");
    println!("{}", "-".repeat(70));
    for (base, name) in &primorials {
        println!("{:>10} {:>30} {:>12.3} {:>12.3}", base, name, predict(*base), (*base as f64).ln());
    }

    println!("\n\nRunning empirical tests...\n");

    let samples = 500;  // Per seed length
    let seed_lengths: Vec<usize> = (4..=20).collect();

    println!("{:>10} {:>12} {:>12} {:>12} {:>12} {:>10}",
             "Base", "Primes", "Tests", "Rate%", "Efficiency", "Predicted");
    println!("{}", "-".repeat(75));

    let mut results: Vec<(u64, f64, f64)> = vec![];

    for (base, name) in &primorials {
        let right = first_coprime(*base);
        let mut rng = 42424242u64 + base;

        let mut total_primes = 0;
        let mut total_tests = 0;
        let mut total_digits = 0.0;

        for &seed_len in &seed_lengths {
            for _ in 0..samples {
                let seed = random_seed_with_length(*base, seed_len, &mut rng);
                let mem = membrane_value(*base, 1, &seed, right);
                total_digits += mem.to_string().len() as f64;
                total_tests += 1;
                if is_prime_miller_rabin(&mem) {
                    total_primes += 1;
                }
            }
        }

        let rate = total_primes as f64 / total_tests as f64;
        let mean_digits = total_digits / total_tests as f64;
        let pnt_expected = 1.0 / (mean_digits * 2.303);  // 1/ln(10^d) = 1/(d*ln(10))
        let efficiency = rate / pnt_expected;
        let predicted = predict(*base);

        results.push((*base, efficiency, predicted));

        println!("{:>10} {:>12} {:>12} {:>11.1}% {:>12.3} {:>10.3}",
                 base, total_primes, total_tests, rate * 100.0, efficiency, predicted);

        // Progress indicator for P₇ since it takes longer
        if *base == 510510 {
            println!("         {} complete!", name);
        }
    }

    // Analysis
    println!("\n\n{}", "═".repeat(70));
    println!("ANALYSIS: Does P₇ follow the trend?");
    println!("{}", "═".repeat(70));

    // Linear regression on ln(base) vs efficiency
    let xs: Vec<f64> = results.iter().map(|(b, _, _)| (*b as f64).ln()).collect();
    let ys: Vec<f64> = results.iter().map(|(_, e, _)| *e).collect();

    let n = xs.len() as f64;
    let sum_x: f64 = xs.iter().sum();
    let sum_y: f64 = ys.iter().sum();
    let sum_xy: f64 = xs.iter().zip(ys.iter()).map(|(x, y)| x * y).sum();
    let sum_x2: f64 = xs.iter().map(|x| x * x).sum();

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
    let intercept = (sum_y - slope * sum_x) / n;

    // R²
    let y_mean = sum_y / n;
    let ss_tot: f64 = ys.iter().map(|y| (y - y_mean).powi(2)).sum();
    let ss_res: f64 = xs.iter().zip(ys.iter())
        .map(|(x, y)| (y - (slope * x + intercept)).powi(2))
        .sum();
    let r2 = 1.0 - ss_res / ss_tot;

    println!("\nFitted formula (all 5 primorials):");
    println!("  efficiency ≈ {:.4} × ln(base) + {:.3}", slope, intercept);
    println!("  R² = {:.4}", r2);

    println!("\nComparison with original formula (0.159 × ln(base) + 3.66):");
    println!("  Slope:     {:.4} vs 0.159  (ratio: {:.3})", slope, slope / 0.159);
    println!("  Intercept: {:.3} vs 3.66   (ratio: {:.3})", intercept, intercept / 3.66);

    // P₇ specific
    let p7_result = results.iter().find(|(b, _, _)| *b == 510510);
    if let Some((_, obs, pred)) = p7_result {
        let deviation = (obs - pred) / pred * 100.0;
        println!("\nP₇ = 510510 results:");
        println!("  Observed efficiency:  {:.3}", obs);
        println!("  Predicted efficiency: {:.3}", pred);
        println!("  Deviation: {:+.1}%", deviation);

        if deviation.abs() < 10.0 {
            println!("\n✓ P₇ FOLLOWS THE TREND! The formula extends to at least 510510.");
        } else if deviation > 0.0 {
            println!("\n⬆ P₇ EXCEEDS prediction - efficiency gains may accelerate!");
        } else {
            println!("\n⬇ P₇ BELOW prediction - efficiency gains may be saturating.");
        }
    }

    println!("\n{}", "═".repeat(70));
    println!("Coefficient interpretation:");
    println!("  Slope {:.4} ≈ 1/(2π) = {:.4}?  Ratio: {:.4}",
             slope, 1.0 / (2.0 * std::f64::consts::PI),
             slope / (1.0 / (2.0 * std::f64::consts::PI)));
    println!("{}", "═".repeat(70));
}
