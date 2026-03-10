//! Base Design Explorer
//!
//! Key question: Can we PREDICT membrane-friendly bases from prime core fraction?
//!
//! Hypothesis: The prime core fraction (after stripping base factors) determines
//! membrane success. We test this by:
//! 1. Computing prime core fraction for MANY bases (not just known ones)
//! 2. Predicting which bases SHOULD be membrane-friendly
//! 3. Comparing to known membrane success rates
//! 4. Identifying NEW candidate bases for experimental verification

use primes::hzlib::num_theory::{factor, Material};
use std::collections::HashMap;

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut d = 3;
    while d * d <= n {
        if n % d == 0 { return false; }
        d += 2;
    }
    true
}

/// Compute prime core fraction for a given base
/// This is the fraction of numbers (coprime to base) whose core is prime
fn prime_core_fraction(base: u64, limit: u64) -> (f64, usize, usize) {
    let base_primes: Vec<u64> = factor(base).iter().map(|(p, _)| *p).collect();

    let mut prime_cores = 0usize;
    let mut total_cores = 0usize;

    for n in 2..=limit {
        // Skip multiples of base's prime factors
        if base_primes.iter().any(|&p| n % p == 0) {
            continue;
        }

        let m = Material::for_base(n, base);
        if m.core > 1 {
            total_cores += 1;
            if is_prime(m.core) {
                prime_cores += 1;
            }
        }
    }

    let fraction = if total_cores > 0 {
        prime_cores as f64 / total_cores as f64
    } else {
        0.0
    };

    (fraction, prime_cores, total_cores)
}

/// Describe base factorization in human-readable form
fn describe_base(base: u64) -> String {
    let factors = factor(base);
    let primes: Vec<u64> = factors.iter().map(|(p, _)| *p).collect();
    let factor_str: String = factors.iter()
        .map(|(p, e)| if *e == 1 { format!("{}", p) } else { format!("{}^{}", p, e) })
        .collect::<Vec<_>>()
        .join("×");

    format!("{} (strips {{{}}})", factor_str,
            primes.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(","))
}

/// Count distinct prime factors
fn omega(n: u64) -> usize {
    factor(n).len()
}

/// Count total prime factors with multiplicity
fn big_omega(n: u64) -> u32 {
    factor(n).iter().map(|(_, e)| *e).sum()
}

fn main() {
    println!("=== BASE DESIGN EXPLORER ===\n");
    println!("Hypothesis: Prime core fraction predicts membrane success\n");

    let limit = 2000u64;  // Analyze cores up to this limit

    // Known membrane success rates (for comparison)
    let known: HashMap<u64, f64> = [
        (6, 0.33),
        (10, 0.185),
        (12, 0.30),
        (30, 0.30),
    ].into_iter().collect();

    // Test a wide range of bases
    let bases: Vec<u64> = (2..=60).filter(|&b| b > 1).collect();

    println!("Analyzing {} bases (cores up to {})...\n", bases.len(), limit);

    // Collect results
    let mut results: Vec<(u64, f64, String, Option<f64>)> = Vec::new();

    for base in bases {
        let (frac, _, _) = prime_core_fraction(base, limit);
        let desc = describe_base(base);
        let membrane = known.get(&base).copied();
        results.push((base, frac, desc, membrane));
    }

    // Sort by prime core fraction (descending)
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("TOP 30 BASES BY PRIME CORE FRACTION:\n");
    println!("{:>4} {:>8} {:>10} {:>25} {:>12}",
             "Base", "PrimeFrac", "Membrane%", "Factorization", "Prediction");
    println!("{}", "-".repeat(75));

    for (base, frac, desc, membrane) in results.iter().take(30) {
        let membrane_str = membrane.map(|m| format!("{:.1}%", m * 100.0))
            .unwrap_or_else(|| "?".to_string());

        // Prediction based on prime core fraction
        let prediction = if *frac > 0.60 {
            "EXCELLENT"
        } else if *frac > 0.50 {
            "GOOD"
        } else if *frac > 0.40 {
            "MODERATE"
        } else {
            "WEAK"
        };

        println!("{:>4} {:>8.1}% {:>10} {:>25} {:>12}",
                 base, frac * 100.0, membrane_str, desc, prediction);
    }

    // Analysis: What patterns make a base have high prime core fraction?
    println!("\n\n=== PATTERN ANALYSIS ===\n");

    // Group by number of distinct prime factors
    let mut by_omega: HashMap<usize, Vec<(u64, f64)>> = HashMap::new();
    for (base, frac, _, _) in &results {
        by_omega.entry(omega(*base)).or_default().push((*base, *frac));
    }

    println!("Prime core fraction by ω(base) (distinct prime factors):\n");
    for omega_val in 1..=4 {
        if let Some(bases) = by_omega.get(&omega_val) {
            let mean_frac = bases.iter().map(|(_, f)| f).sum::<f64>() / bases.len() as f64;
            let best = bases.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            println!("ω={}: mean={:.1}%, best={:?}", omega_val, mean_frac * 100.0, best);
        }
    }

    // Find the "magic primes" to strip
    println!("\n\nWHICH PRIMES TO STRIP?\n");

    // Test different prime sets
    let prime_sets = vec![
        vec![2],           // Just evens
        vec![2, 3],        // Base 6
        vec![2, 5],        // Base 10
        vec![2, 3, 5],     // Base 30
        vec![2, 7],        // Base 14
        vec![2, 3, 7],     // Base 42
        vec![3, 5],        // Base 15
        vec![2, 3, 5, 7],  // Base 210
    ];

    println!("Testing different prime factor sets:\n");
    println!("{:>15} {:>8} {:>8}", "Primes", "Base", "PrimeFrac");
    println!("{}", "-".repeat(35));

    for primes in prime_sets {
        let base: u64 = primes.iter().product();
        let (frac, _, _) = prime_core_fraction(base, limit);
        let primes_str = primes.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(",");
        println!("{:>15} {:>8} {:>8.1}%", format!("{{{}}}", primes_str), base, frac * 100.0);
    }

    // NEW DISCOVERY CANDIDATES
    println!("\n\n=== NEW CANDIDATE BASES ===\n");
    println!("These bases have high prime core fraction but NO known membrane data:\n");

    let candidates: Vec<_> = results.iter()
        .filter(|(_, frac, _, membrane)| *frac > 0.45 && membrane.is_none())
        .take(15)
        .collect();

    println!("{:>4} {:>8} {:>30} {:>15}", "Base", "PrimeFrac", "Factorization", "Why Interesting");
    println!("{}", "-".repeat(65));

    for (base, frac, desc, _) in candidates {
        let why = if omega(*base) >= 3 {
            "Multi-prime strip"
        } else if big_omega(*base) > omega(*base) as u32 {
            "Prime power"
        } else if *frac > 0.55 {
            "Very high frac"
        } else {
            "Good frac"
        };

        println!("{:>4} {:>8.1}% {:>30} {:>15}", base, frac * 100.0, desc, why);
    }

    // Theoretical analysis: Why does stripping {2,3} beat {2,5}?
    println!("\n\n=== THEORETICAL INSIGHT ===\n");

    println!("Why does base 6 (strip {{2,3}}) outperform base 10 (strip {{2,5}})?");
    println!();

    // Count residues coprime to each
    let count_coprimes = |base: u64, limit: u64| -> usize {
        let factors = factor(base);
        let primes: Vec<u64> = factors.iter().map(|(p, _)| *p).collect();
        (1..=limit).filter(|&n| primes.iter().all(|&p| n % p != 0)).count()
    };

    let cop_6 = count_coprimes(6, 100);
    let cop_10 = count_coprimes(10, 100);
    let cop_30 = count_coprimes(30, 100);

    println!("Residues coprime to base (in 1..100):");
    println!("  Base 6:  {} residues (1/3 of all)", cop_6);
    println!("  Base 10: {} residues (2/5 of all)", cop_10);
    println!("  Base 30: {} residues (4/15 of all)", cop_30);
    println!();

    println!("Key insight: Base 6 strips the TWO SMALLEST primes (2,3).");
    println!("Base 10 misses 3, leaving more composite cores.");
    println!();
    println!("The DENSITY of primes among cores is what matters!");
    println!("Stripping more small primes → fewer but MORE PRIME-RICH cores.");

    // Prediction for untested bases
    println!("\n\n=== PREDICTIONS FOR EXPERIMENTAL VERIFICATION ===\n");

    println!("Based on prime core fraction, we PREDICT these bases should");
    println!("achieve membrane success rates comparable to known champions:\n");

    let predictions = vec![
        (42, "2×3×7", "Adds 7 to base-6 primes"),
        (66, "2×3×11", "Adds 11 to base-6 primes"),
        (210, "2×3×5×7", "Primorial - strips 4 smallest primes"),
    ];

    for (base, factors, reason) in predictions {
        let (frac, _, _) = prime_core_fraction(base, limit);
        println!("Base {} = {}", base, factors);
        println!("  Prime core fraction: {:.1}%", frac * 100.0);
        println!("  Rationale: {}", reason);
        println!("  PREDICTION: Should achieve ~{:.0}% membrane success", frac * 50.0 + 5.0);
        println!();
    }
}
