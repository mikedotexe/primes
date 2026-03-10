//! Primorial Resonance - Size Normalized
//!
//! The raw data showed ALL bases peak at mod6≡1 and valley at mod6≡0.
//! But this could be confounded by the SIZE EFFECT (smaller = more primes).
//!
//! This experiment normalizes by expected prime density to reveal
//! any true periodicity independent of size.

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
        let digit = next() % base;
        seed = seed * &b + BigUint::from(digit);
    }
    seed
}

fn membrane_value(base: u64, left: u64, seed: &BigUint, right: u64) -> BigUint {
    let b = BigUint::from(base);
    let mut seed_digits = 0u32;
    let mut temp = seed.clone();
    while temp > BigUint::ZERO { temp /= &b; seed_digits += 1; }
    if seed_digits == 0 { seed_digits = 1; }
    BigUint::from(left) * b.pow(seed_digits + 1) + seed * &b + BigUint::from(right)
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

fn main() {
    println!("=== SIZE-NORMALIZED PRIMORIAL RESONANCE ===\n");

    let bases: Vec<(u64, &str)> = vec![
        (6, "P₂"),
        (30, "P₃"),
        (210, "P₄"),
        (2310, "P₅"),
    ];

    let samples = 500;
    let min_len = 4;  // Skip very short seeds where variance is high
    let max_len = 30; // 5 complete periods

    println!("Testing seed lengths {}-{} ({} samples each)\n", min_len, max_len, samples);

    for (base, name) in &bases {
        println!("\n{} (base {})", "=".repeat(60), "");
        println!("{} - Base {}", name, base);
        println!("{}", "=".repeat(60));

        let right = first_coprime(*base);
        let mut rng = 42424242u64 + base;

        // Collect: (length, observed_rate, mean_decimal_digits, efficiency)
        let mut data: Vec<(usize, f64, f64, f64)> = vec![];

        println!("\n{:>4} {:>8} {:>8} {:>10} {:>12} {:>8}",
                 "Len", "Primes", "Rate%", "Dec.Digits", "Efficiency", "Mod6");
        println!("{}", "-".repeat(58));

        for seed_len in min_len..=max_len {
            let mut primes_found = 0;
            let mut total_digits = 0.0;

            for _ in 0..samples {
                let seed = random_seed_with_length(*base, seed_len, &mut rng);
                let membrane = membrane_value(*base, 1, &seed, right);
                let dec_digits = membrane.to_string().len();
                total_digits += dec_digits as f64;

                if is_prime_miller_rabin(&membrane) {
                    primes_found += 1;
                }
            }

            let rate = primes_found as f64 / samples as f64;
            let mean_digits = total_digits / samples as f64;
            let pnt_expected = 1.0 / (mean_digits * 2.303);
            let efficiency = rate / pnt_expected;

            let residue = seed_len % 6;

            println!("{:>4} {:>8} {:>8.1} {:>10.1} {:>12.2} {:>8}",
                     seed_len, primes_found, rate * 100.0, mean_digits, efficiency, residue);

            data.push((seed_len, rate, mean_digits, efficiency));
        }

        // Analyze efficiency by mod 6
        println!("\n--- SIZE-NORMALIZED ANALYSIS (Efficiency = Obs/PNT) ---\n");

        let mut by_residue: HashMap<usize, Vec<f64>> = HashMap::new();
        for (len, _, _, eff) in &data {
            by_residue.entry(*len % 6).or_default().push(*eff);
        }

        println!("{:>8} {:>12} {:>12} {:>8}", "Mod6", "Mean Eff.", "Std Dev", "N");
        println!("{}", "-".repeat(44));

        let mut residue_stats: Vec<(usize, f64, f64)> = vec![];

        for r in 0..6 {
            if let Some(effs) = by_residue.get(&r) {
                let mean = effs.iter().sum::<f64>() / effs.len() as f64;
                let var = effs.iter().map(|e| (e - mean).powi(2)).sum::<f64>() / effs.len() as f64;
                let std = var.sqrt();
                residue_stats.push((r, mean, std));
                println!("{:>8} {:>12.2} {:>12.2} {:>8}", r, mean, std, effs.len());
            }
        }

        // Find peak and valley
        let (peak_r, peak_eff, _) = residue_stats.iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap();
        let (valley_r, valley_eff, _) = residue_stats.iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap();

        println!("\nPeak:   mod6≡{} (efficiency {:.2})", peak_r, peak_eff);
        println!("Valley: mod6≡{} (efficiency {:.2})", valley_r, valley_eff);
        println!("Ratio:  {:.2}x difference", peak_eff / valley_eff);

        // Test significance: is mod6≡0 really the worst?
        let eff_mod0: Vec<f64> = by_residue.get(&0).unwrap().clone();
        let eff_others: Vec<f64> = (1..6).flat_map(|r|
            by_residue.get(&r).unwrap_or(&vec![]).clone()
        ).collect();

        let mean_mod0 = eff_mod0.iter().sum::<f64>() / eff_mod0.len() as f64;
        let mean_others = eff_others.iter().sum::<f64>() / eff_others.len() as f64;

        println!("\nMod6≡0 vs Others:");
        println!("  Mean efficiency (mod6≡0):     {:.2}", mean_mod0);
        println!("  Mean efficiency (mod6≠0):     {:.2}", mean_others);
        println!("  Difference:                   {:+.2} ({:+.1}%)",
                 mean_mod0 - mean_others,
                 100.0 * (mean_mod0 / mean_others - 1.0));
    }

    // Cross-base comparison
    println!("\n\n{}", "=".repeat(70));
    println!("CROSS-BASE SUMMARY: IS MOD6≡0 CONSISTENTLY WORST?");
    println!("{}", "=".repeat(70));

    println!("\nAfter size normalization, if mod6≡0 (lengths 6,12,18,24,30)");
    println!("is still the valley, this suggests a true structural effect,");
    println!("not just size confounding.\n");

    println!("Key question: Are lengths divisible by 6 = 2×3 special?");
    println!("These lengths may create specific modular alignments that");
    println!("increase divisibility by small primes.\n");

    println!("Next steps:");
    println!("  1. Test if effect persists at even larger seed lengths");
    println!("  2. Analyze divisibility patterns at mod6≡0 lengths");
    println!("  3. Check if mod 2 or mod 3 alone shows signal");
}
