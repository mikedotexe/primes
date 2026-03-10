//! Primorial Frontier Exploration
//!
//! Push the primorial hypothesis to its limits:
//! - Base 2310 = 2×3×5×7×11 (primorial of 11)
//! - Base 30030 = 2×3×5×7×11×13 (primorial of 13)
//!
//! Theory predicts higher PCF = higher membrane success

use num_bigint::BigUint;
use primes::is_prime_miller_rabin;
use primes::hzlib::num_theory::factor;

fn is_prime_u64(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut d = 3;
    while d * d <= n { if n % d == 0 { return false; } d += 2; }
    true
}

/// Compute prime core fraction
fn prime_core_fraction(base: u64, limit: u64) -> f64 {
    let base_primes: Vec<u64> = factor(base).iter().map(|(p, _)| *p).collect();

    let mut prime_cores = 0usize;
    let mut total = 0usize;

    for n in 2..=limit {
        if base_primes.iter().any(|&p| n % p == 0) { continue; }

        let mut core = n;
        for &p in &base_primes {
            while core % p == 0 { core /= p; }
        }

        if core > 1 {
            total += 1;
            if is_prime_u64(core) { prime_cores += 1; }
        }
    }

    prime_cores as f64 / total as f64
}

/// Generate membrane value for arbitrary base
fn membrane_value(base: u64, left: u64, seed: u64, right: u64) -> BigUint {
    let b = BigUint::from(base);
    let seed_digits = if seed == 0 { 1 } else {
        let mut s = seed;
        let mut count = 0u32;
        while s > 0 { s /= base; count += 1; }
        count
    };
    BigUint::from(left) * b.pow(seed_digits + 1) + BigUint::from(seed) * &b + BigUint::from(right)
}

/// Find coprime digits for a base
fn coprime_digits(base: u64, max_digit: u64) -> Vec<u64> {
    let base_primes: Vec<u64> = factor(base).iter().map(|(p, _)| *p).collect();
    (1..max_digit.min(base))
        .filter(|&d| base_primes.iter().all(|&p| d % p != 0))
        .collect()
}

/// Test membrane success rate
fn test_membrane(base: u64, left: u64, right: u64, max_seed: u64) -> (usize, f64) {
    let mut primes = 0;
    for seed in 1..=max_seed {
        let value = membrane_value(base, left, seed, right);
        if is_prime_miller_rabin(&value) {
            primes += 1;
        }
    }
    (primes, primes as f64 / max_seed as f64)
}

fn main() {
    println!("=== PRIMORIAL FRONTIER EXPLORATION ===\n");

    // The primorials
    let primorials = vec![
        (6, "2×3", vec![2, 3]),
        (30, "2×3×5", vec![2, 3, 5]),
        (210, "2×3×5×7", vec![2, 3, 5, 7]),
        (2310, "2×3×5×7×11", vec![2, 3, 5, 7, 11]),
    ];

    println!("PRIME CORE FRACTION BY PRIMORIAL:\n");
    println!("{:>6} {:>15} {:>10} {:>12}", "Base", "Factorization", "PCF%", "Prediction");
    println!("{}", "-".repeat(50));

    for (base, factors, _primes) in &primorials {
        let pcf = prime_core_fraction(*base, 5000);
        let pred = pcf * 0.6 + 0.1;  // Empirical prediction formula
        println!("{:>6} {:>15} {:>10.1} {:>12.1}%", base, factors, pcf * 100.0, pred * 100.0);
    }

    // Test base 2310
    println!("\n\n=== BASE 2310 MEMBRANE TEST ===\n");

    let base = 2310u64;
    let max_seed = 300u64;

    // Get coprime digits
    let digits = coprime_digits(base, 100);
    println!("First 20 coprime digits (< 100): {:?}\n", &digits[..20.min(digits.len())]);

    // Test with L=1 and various R values
    println!("Testing L=1 with various R values (seeds 1-{}):\n", max_seed);
    println!("{:>6} {:>8} {:>10}", "R", "Primes", "Rate%");
    println!("{}", "-".repeat(28));

    let mut best = (0u64, 0.0);

    for &right in digits.iter().take(15) {
        let (primes, rate) = test_membrane(base, 1, right, max_seed);
        if rate > best.1 { best = (right, rate); }
        println!("{:>6} {:>8} {:>10.1}", right, primes, rate * 100.0);
    }

    println!("\nBest config for base 2310: L=1, R={}", best.0);
    println!("Success rate: {:.1}%", best.1 * 100.0);

    // Compare primorials
    println!("\n\n=== PRIMORIAL COMPARISON ===\n");

    let comparison = vec![
        (6, 1, 5),
        (30, 1, 13),
        (210, 1, 31),
        (2310, 1, best.0),
    ];

    println!("{:>6} {:>8} {:>8} {:>10} {:>10}", "Base", "L", "R", "PCF%", "Rate%");
    println!("{}", "-".repeat(50));

    for (base, left, right) in comparison {
        let pcf = prime_core_fraction(base, 5000);
        let (_, rate) = test_membrane(base, left, right, 500);
        println!("{:>6} {:>8} {:>8} {:>10.1} {:>10.1}",
                 base, left, right, pcf * 100.0, rate * 100.0);
    }

    // Theory check
    println!("\n\n=== THEORY VALIDATION ===\n");

    let pcf_2310 = prime_core_fraction(2310, 5000);
    let rate_2310 = best.1;

    println!("Base 2310 results:");
    println!("  Prime core fraction: {:.1}%", pcf_2310 * 100.0);
    println!("  Membrane success:    {:.1}%", rate_2310 * 100.0);

    if rate_2310 > 0.40 {
        println!("\nHYPOTHESIS CONFIRMED!");
        println!("Base 2310 achieves >{:.0}% membrane success", 40.0);
        println!("Primorial scaling continues to work.");
    } else if rate_2310 > 0.30 {
        println!("\nPARTIAL CONFIRMATION");
        println!("Base 2310 competitive with base 6/30, but doesn't exceed 210");
    } else {
        println!("\nDIMINISHING RETURNS OBSERVED");
        println!("Larger primorials may not improve membrane success");
    }

    // Additional analysis: Sample some prime membranes
    println!("\n\n=== SAMPLE PRIME MEMBRANES (Base 2310) ===\n");

    let mut found = 0;
    for seed in 1..500 {
        let value = membrane_value(2310, 1, seed, best.0);
        if is_prime_miller_rabin(&value) {
            if found < 5 {
                println!("Seed {}: {} ({} digits)",
                         seed, &value.to_string()[..20.min(value.to_string().len())],
                         value.to_string().len());
            }
            found += 1;
        }
    }
    println!("... ({} primes found in first 500 seeds)", found);
}
