//! PCF-Size Tradeoff Analysis
//!
//! Discovery: PCF increases with primorial size, but membrane success plateaus
//! because larger bases produce larger (lower prime density) numbers.
//!
//! Goal: Find the optimal balance between PCF advantage and size penalty.

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

fn prime_core_fraction(base: u64, limit: u64) -> f64 {
    let base_primes: Vec<u64> = factor(base).iter().map(|(p, _)| *p).collect();
    let mut prime_cores = 0usize;
    let mut total = 0usize;

    for n in 2..=limit {
        if base_primes.iter().any(|&p| n % p == 0) { continue; }
        let mut core = n;
        for &p in &base_primes { while core % p == 0 { core /= p; } }
        if core > 1 { total += 1; if is_prime_u64(core) { prime_cores += 1; } }
    }
    prime_cores as f64 / total as f64
}

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

fn test_membrane(base: u64, left: u64, right: u64, max_seed: u64) -> (usize, f64, f64) {
    let mut primes = 0;
    let mut total_size = 0.0;

    for seed in 1..=max_seed {
        let value = membrane_value(base, left, seed, right);
        let size = value.to_string().len() as f64;
        total_size += size;
        if is_prime_miller_rabin(&value) { primes += 1; }
    }

    let mean_size = total_size / max_seed as f64;
    (primes, primes as f64 / max_seed as f64, mean_size)
}

fn main() {
    println!("=== PCF-SIZE TRADEOFF ANALYSIS ===\n");

    let bases = vec![
        (6, 1, 5, "2×3"),
        (30, 1, 13, "2×3×5"),
        (210, 1, 31, "2×3×5×7"),
        (2310, 1, 59, "2×3×5×7×11"),
    ];

    let max_seed = 500u64;

    println!("FULL ANALYSIS:\n");
    println!("{:>6} {:>12} {:>8} {:>8} {:>10} {:>12} {:>12}",
             "Base", "Factors", "PCF%", "Rate%", "MeanSize", "PNT_Expected", "Efficiency");
    println!("{}", "-".repeat(85));

    for (base, left, right, factors) in &bases {
        let pcf = prime_core_fraction(*base, 5000);
        let (_, rate, mean_size) = test_membrane(*base, *left, *right, max_seed);

        // PNT expected density: 1 / ln(10^mean_size) ≈ 1 / (mean_size * ln(10))
        let pnt_expected = 1.0 / (mean_size * 2.303);

        // Efficiency = actual_rate / pnt_expected
        // This normalizes for size, showing structural advantage
        let efficiency = rate / pnt_expected;

        println!("{:>6} {:>12} {:>8.1} {:>8.1} {:>10.1} {:>12.3} {:>12.2}",
                 base, factors, pcf * 100.0, rate * 100.0, mean_size,
                 pnt_expected * 100.0, efficiency);
    }

    // Size-normalized comparison
    println!("\n\n=== SIZE-NORMALIZED EFFICIENCY ===\n");

    println!("Efficiency = Observed_Rate / PNT_Expected_Rate");
    println!("This measures how much BETTER than random each base performs.\n");

    let mut efficiencies: Vec<(u64, f64)> = Vec::new();

    for (base, left, right, _) in &bases {
        let (_, rate, mean_size) = test_membrane(*base, *left, *right, max_seed);
        let pnt_expected = 1.0 / (mean_size * 2.303);
        let efficiency = rate / pnt_expected;
        efficiencies.push((*base, efficiency));
    }

    efficiencies.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("{:>6} {:>12}", "Base", "Efficiency");
    println!("{}", "-".repeat(20));
    for (base, eff) in &efficiencies {
        let stars = if *eff == efficiencies[0].1 { " <-- WINNER" } else { "" };
        println!("{:>6} {:>12.2}{}", base, eff, stars);
    }

    // Detailed seed-by-seed analysis for optimal base
    println!("\n\n=== OPTIMAL PRIMORIAL DETERMINATION ===\n");

    // Theory: efficiency should peak at some primorial
    // because PCF increases but size also increases

    println!("Extended primorial test:\n");

    let extended_bases = vec![
        (6, "6 = 2×3"),
        (30, "30 = 2×3×5"),
        (42, "42 = 2×3×7"),  // Non-primorial for comparison
        (66, "66 = 2×3×11"), // Non-primorial for comparison
        (210, "210 = 2×3×5×7"),
        (2310, "2310 = 2×3×5×7×11"),
    ];

    let base_primes_map: std::collections::HashMap<u64, u64> = [
        (6, 5), (30, 13), (42, 19), (66, 61), (210, 31), (2310, 59)
    ].into_iter().collect();

    println!("{:>6} {:>18} {:>8} {:>8} {:>10}", "Base", "Factorization", "Rate%", "MeanSize", "Efficiency");
    println!("{}", "-".repeat(60));

    let mut best_eff = (0u64, 0.0);

    for (base, label) in &extended_bases {
        let right = *base_primes_map.get(base).unwrap_or(&1);
        let (_, rate, mean_size) = test_membrane(*base, 1, right, max_seed);
        let pnt_expected = 1.0 / (mean_size * 2.303);
        let efficiency = rate / pnt_expected;

        if efficiency > best_eff.1 { best_eff = (*base, efficiency); }

        println!("{:>6} {:>18} {:>8.1} {:>10.1} {:>10.2}",
                 base, label, rate * 100.0, mean_size, efficiency);
    }

    println!("\n=== CONCLUSION ===\n");
    println!("Best size-normalized efficiency: Base {}", best_eff.0);
    println!();
    println!("The tradeoff between PCF advantage and size penalty");
    println!("suggests an OPTIMAL PRIMORIAL exists.");
    println!();

    if best_eff.0 == 30 || best_eff.0 == 210 {
        println!("Base {} appears optimal for membrane construction!", best_eff.0);
        println!("Larger primorials have diminishing returns due to size penalty.");
    } else if best_eff.0 == 6 {
        println!("Base 6 wins despite lower PCF because its membranes are smallest.");
    } else {
        println!("Optimal base: {} - balances PCF and size effects.", best_eff.0);
    }
}
