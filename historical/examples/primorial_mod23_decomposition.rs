//! Mod 2 vs Mod 3 Decomposition
//!
//! Since 6 = 2 × 3, the period-6 structure might come from:
//! - Period-2 effects (even vs odd seed lengths)
//! - Period-3 effects (seed length mod 3)
//! - Interaction between them
//!
//! This test decomposes the signal to find the source.

use num_bigint::BigUint;
use primes::is_prime_miller_rabin;
use std::collections::HashMap;

fn random_seed_with_length(base: u64, length: usize, rng: &mut u64) -> BigUint {
    if length == 0 { return BigUint::ZERO; }
    let mut next = || {
        *rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        *rng
    };
    let b = BigUint::from(base);
    let first = (next() % (base - 1)) + 1;
    let mut seed = BigUint::from(first);
    for _ in 1..length {
        seed = seed * &b + BigUint::from(next() % base);
    }
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
        for p in [2, 3, 5, 7, 11, 13] {
            if n % p == 0 { fs.push(p); while n % p == 0 { n /= p; } }
        }
        fs
    };
    (1..base).find(|&d| factors.iter().all(|&p| d % p != 0)).unwrap_or(1)
}

fn variation(data: &[f64]) -> f64 {
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let max = data.iter().cloned().fold(f64::MIN, f64::max);
    let min = data.iter().cloned().fold(f64::MAX, f64::min);
    (max - min) / mean
}

fn main() {
    println!("=== MOD 2 vs MOD 3 DECOMPOSITION ===\n");

    let bases: Vec<(u64, &str, u64)> = vec![
        (6, "P₂", 1),    // theoretical ord(10) period
        (30, "P₃", 1),
        (210, "P₄", 6),
        (2310, "P₅", 6),
    ];

    let samples = 600;
    let min_len = 4;
    let max_len = 36;  // 12 complete periods of 3, 18 of 2

    println!("Decomposing variation into mod2 and mod3 components\n");

    for (base, name, theo_period) in &bases {
        println!("{}", "=".repeat(65));
        println!("{} (Base {}, theoretical period {})", name, base, theo_period);
        println!("{}", "=".repeat(65));

        let right = first_coprime(*base);
        let mut rng = 98765u64 + base;

        // Collect efficiency data
        let mut data: Vec<(usize, f64)> = vec![];

        for seed_len in min_len..=max_len {
            let mut primes = 0;
            let mut total_dig = 0.0;

            for _ in 0..samples {
                let seed = random_seed_with_length(*base, seed_len, &mut rng);
                let mem = membrane_value(*base, 1, &seed, right);
                total_dig += mem.to_string().len() as f64;
                if is_prime_miller_rabin(&mem) { primes += 1; }
            }

            let rate = primes as f64 / samples as f64;
            let mean_dig = total_dig / samples as f64;
            let eff = rate / (1.0 / (mean_dig * 2.303));
            data.push((seed_len, eff));
        }

        // Group by mod 2
        let mut by_mod2: HashMap<usize, Vec<f64>> = HashMap::new();
        for (len, eff) in &data {
            by_mod2.entry(len % 2).or_default().push(*eff);
        }

        // Group by mod 3
        let mut by_mod3: HashMap<usize, Vec<f64>> = HashMap::new();
        for (len, eff) in &data {
            by_mod3.entry(len % 3).or_default().push(*eff);
        }

        // Group by mod 6
        let mut by_mod6: HashMap<usize, Vec<f64>> = HashMap::new();
        for (len, eff) in &data {
            by_mod6.entry(len % 6).or_default().push(*eff);
        }

        // Group by mod 7 (control - should be noise for period-6 bases)
        let mut by_mod7: HashMap<usize, Vec<f64>> = HashMap::new();
        for (len, eff) in &data {
            by_mod7.entry(len % 7).or_default().push(*eff);
        }

        // Calculate mean efficiency by residue class
        let means_mod2: Vec<f64> = (0..2).map(|r|
            by_mod2.get(&r).map(|v| v.iter().sum::<f64>() / v.len() as f64).unwrap_or(0.0)
        ).collect();

        let means_mod3: Vec<f64> = (0..3).map(|r|
            by_mod3.get(&r).map(|v| v.iter().sum::<f64>() / v.len() as f64).unwrap_or(0.0)
        ).collect();

        let means_mod6: Vec<f64> = (0..6).map(|r|
            by_mod6.get(&r).map(|v| v.iter().sum::<f64>() / v.len() as f64).unwrap_or(0.0)
        ).collect();

        let means_mod7: Vec<f64> = (0..7).map(|r|
            by_mod7.get(&r).map(|v| v.iter().sum::<f64>() / v.len() as f64).unwrap_or(0.0)
        ).collect();

        // Calculate variations
        let var2 = variation(&means_mod2);
        let var3 = variation(&means_mod3);
        let var6 = variation(&means_mod6);
        let var7 = variation(&means_mod7);

        println!("\nMod 2 (even vs odd seed length):");
        for r in 0..2 {
            let label = if r == 0 { "even" } else { "odd " };
            println!("  {}: mean eff = {:.2}", label, means_mod2[r]);
        }
        println!("  Variation: {:.1}%", var2 * 100.0);

        println!("\nMod 3:");
        for r in 0..3 {
            println!("  ≡{}: mean eff = {:.2}", r, means_mod3[r]);
        }
        println!("  Variation: {:.1}%", var3 * 100.0);

        println!("\nMod 6 (combined):");
        for r in 0..6 {
            println!("  ≡{}: mean eff = {:.2}", r, means_mod6[r]);
        }
        println!("  Variation: {:.1}%", var6 * 100.0);

        println!("\nMod 7 (control):");
        for r in 0..7 {
            println!("  ≡{}: mean eff = {:.2}", r, means_mod7[r]);
        }
        println!("  Variation: {:.1}%", var7 * 100.0);

        // Analysis
        println!("\n--- ANALYSIS ---");

        let expected_var6 = (var2.powi(2) + var3.powi(2)).sqrt();
        println!("If mod2 and mod3 independent: expected mod6 var ≈ {:.1}%",
                 expected_var6 * 100.0);
        println!("Actual mod6 variation: {:.1}%", var6 * 100.0);

        if var6 > expected_var6 * 1.3 {
            println!("⚡ Mod6 variation EXCEEDS independent combination!");
            println!("   This suggests interaction effects or true period-6 structure.");
        } else if var6 < expected_var6 * 0.7 {
            println!("↓ Mod6 variation LESS than expected.");
            println!("   Mod2 and mod3 effects may be anti-correlated.");
        } else {
            println!("≈ Mod6 variation consistent with independent mod2/mod3 effects.");
        }

        if var7 > var6 {
            println!("⚠ Mod7 (control) has HIGHER variation than mod6!");
            println!("   Period-6 may not be the true structure.");
        } else {
            println!("✓ Mod7 (control) variation ({:.1}%) < mod6 ({:.1}%)",
                     var7 * 100.0, var6 * 100.0);
        }

        println!();
    }

    println!("\n{}", "=".repeat(65));
    println!("SUMMARY");
    println!("{}", "=".repeat(65));
    println!("\nQuestions answered:");
    println!("1. Is the resonance from mod2 (even/odd) or mod3?");
    println!("2. Does period-6 exceed what mod2×mod3 predicts?");
    println!("3. Is period-6 special or could period-7 work equally well?");
}
