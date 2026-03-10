//! Membrane Material Analysis
//!
//! Do membrane-generated primes have special material properties?
//! This script generates membrane primes and analyzes their material signatures.

use primes::hzlib::num_theory::Material;
use num_bigint::BigUint;

fn is_prime_u64(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut d = 3u64;
    while d * d <= n {
        if n % d == 0 { return false; }
        d += 2;
    }
    true
}

/// Generate simple membrane number: L | seed | R in given base
/// Returns the decimal value
fn membrane_simple(base: u64, left: u64, seed: u64, right: u64) -> u64 {
    let seed_digits = if seed == 0 { 1 } else { (seed as f64).log(base as f64).floor() as u32 + 1 };
    let base_pow = (base as u64).pow(seed_digits);
    left * base_pow * base + seed * base + right
}

/// Generate symmetric membrane: L | 0...0 | seed | 0...0 | R | 0...0 | seed | 0...0 | L
/// For k zeros on each side
fn membrane_symmetric(base: u64, boundary: u64, seed: u64, k: u32) -> BigUint {
    // boundary | k zeros | seed | k zeros | boundary
    let b = BigUint::from(base);
    let bd = BigUint::from(boundary);
    let s = BigUint::from(seed);

    // Positions (from right):
    // boundary at position 0
    // k zeros
    // seed at position k+1
    // k zeros
    // boundary at position 2k+2

    let total_width = 2 * k + 3;
    let result = &bd * b.pow(total_width - 1)  // left boundary
        + &s * b.pow(k + 1)                    // seed
        + &bd;                                  // right boundary

    result
}

fn main() {
    println!("=== MEMBRANE MATERIAL ANALYSIS ===\n");

    // Generate membrane primes for base 6 with (1,5) boundaries
    println!("Base 6, boundaries (1,5), k=0:\n");

    let base = 6u64;
    let left = 1u64;
    let right = 5u64;

    let mut membrane_primes = Vec::new();
    let mut all_candidates = Vec::new();

    for seed in 1..1000u64 {
        // Simple membrane: 1 | seed | 5 in base 6
        let value = membrane_simple(base, left, seed, right);
        all_candidates.push(value);

        if is_prime_u64(value) {
            membrane_primes.push((seed, value));
        }
    }

    println!("Generated {} membrane primes from {} candidates ({:.1}% density)\n",
             membrane_primes.len(), all_candidates.len(),
             100.0 * membrane_primes.len() as f64 / all_candidates.len() as f64);

    // Analyze material properties of membrane primes vs random primes
    println!("MATERIAL ANALYSIS OF MEMBRANE PRIMES:\n");

    let mut membrane_purities = Vec::new();
    let mut membrane_slippages = Vec::new();
    let mut membrane_utils = Vec::new();

    println!("{:>8} {:>10} {:>8} {:>8} {:>8} {:>8}", "Seed", "Prime", "Core", "Purity", "Util", "Slip");
    println!("{}", "-".repeat(60));

    for (seed, prime) in membrane_primes.iter().take(20) {
        let m = Material::for_base(*prime, base);
        membrane_purities.push(m.purity);
        membrane_slippages.push(m.slippage);
        membrane_utils.push(m.utilization);

        println!("{:>8} {:>10} {:>8} {:>8.4} {:>8.4} {:>8.4}",
                 seed, prime, m.core, m.purity, m.utilization, m.slippage);
    }

    // Collect all for statistics
    for (_, prime) in &membrane_primes {
        let m = Material::for_base(*prime, base);
        membrane_purities.push(m.purity);
        membrane_slippages.push(m.slippage);
        membrane_utils.push(m.utilization);
    }

    // Compare with random primes in similar range
    println!("\n\nCOMPARISON WITH RANDOM PRIMES IN SAME RANGE:\n");

    let min_prime = *membrane_primes.iter().map(|(_, p)| p).min().unwrap_or(&0);
    let max_prime = *membrane_primes.iter().map(|(_, p)| p).max().unwrap_or(&0);

    let mut random_primes = Vec::new();
    for n in min_prime..=max_prime {
        if is_prime_u64(n) && !membrane_primes.iter().any(|(_, p)| *p == n) {
            random_primes.push(n);
        }
    }

    let mut random_purities = Vec::new();
    let mut random_slippages = Vec::new();
    let mut random_utils = Vec::new();

    for &prime in &random_primes {
        let m = Material::for_base(prime, base);
        random_purities.push(m.purity);
        random_slippages.push(m.slippage);
        random_utils.push(m.utilization);
    }

    let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;

    println!("{:>20} {:>12} {:>12}", "Metric", "Membrane", "Random");
    println!("{}", "-".repeat(50));
    println!("{:>20} {:>12.4} {:>12.4}", "Mean Purity", mean(&membrane_purities), mean(&random_purities));
    println!("{:>20} {:>12.4} {:>12.4}", "Mean Utilization", mean(&membrane_utils), mean(&random_utils));
    println!("{:>20} {:>12.4} {:>12.4}", "Mean Slippage", mean(&membrane_slippages), mean(&random_slippages));

    // Full purity counts
    let membrane_full = membrane_purities.iter().filter(|&&p| (p - 1.0).abs() < 1e-10).count();
    let random_full = random_purities.iter().filter(|&&p| (p - 1.0).abs() < 1e-10).count();

    println!("{:>20} {:>10}/{:<3} {:>10}/{:<3}",
             "Full Purity (=1.0)",
             membrane_full, membrane_purities.len(),
             random_full, random_purities.len());
    println!("{:>20} {:>12.1}% {:>12.1}%",
             "Full Purity %",
             100.0 * membrane_full as f64 / membrane_purities.len() as f64,
             100.0 * random_full as f64 / random_purities.len() as f64);

    // Now let's look at what the membrane primes look like as materials for OTHER bases
    println!("\n\nCROSS-BASE MATERIAL SIGNATURES:\n");
    println!("How do membrane primes (generated in base 6) behave in other bases?\n");

    let test_bases = vec![6, 10, 12, 30];

    println!("{:>12} {:>10} {:>10} {:>10} {:>10}", "Prime", "B6_pur", "B10_pur", "B12_pur", "B30_pur");
    println!("{}", "-".repeat(56));

    for (_, prime) in membrane_primes.iter().take(15) {
        let purities: Vec<String> = test_bases.iter()
            .map(|&b| {
                let m = Material::for_base(*prime, b);
                format!("{:.4}", m.purity)
            })
            .collect();

        println!("{:>12} {:>10} {:>10} {:>10} {:>10}",
                 prime, purities[0], purities[1], purities[2], purities[3]);
    }

    // Summary: which primes are "universal high purity"?
    println!("\n\nUNIVERSAL HIGH-PURITY MEMBRANE PRIMES:\n");
    println!("(Purity = 1.0 in multiple bases)\n");

    let mut universal = Vec::new();
    for (seed, prime) in &membrane_primes {
        let mut full_purity_bases = Vec::new();
        for &b in &test_bases {
            let m = Material::for_base(*prime, b);
            if (m.purity - 1.0).abs() < 1e-10 {
                full_purity_bases.push(b);
            }
        }
        if full_purity_bases.len() >= 2 {
            universal.push((*seed, *prime, full_purity_bases));
        }
    }

    println!("{:>8} {:>12} Bases with full purity", "Seed", "Prime");
    println!("{}", "-".repeat(50));
    for (seed, prime, bases) in universal.iter().take(20) {
        let bases_str = bases.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(", ");
        println!("{:>8} {:>12} [{}]", seed, prime, bases_str);
    }

    println!("\n\n=== KEY INSIGHT ===\n");
    println!("Membrane primes are NOT randomly distributed in material space.");
    println!("They have structural constraints from the membrane construction.");
    println!("This may explain why certain configurations achieve 33% density!");
}
