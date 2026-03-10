//! Primorial Resonance Hunt
//!
//! HYPOTHESIS: Primorial bases 210+ should show period-6 oscillations in
//! membrane success rate as seed length varies, because:
//!   ord(10) mod (base/gcd(10,base)) = 6 for all primorials ≥ 210
//!
//! Base 30 should NOT show this pattern (period = 1).
//!
//! This connects to the earlier connector asymmetry findings where
//! length-dependent resonances appeared at specific positions.

use num_bigint::BigUint;
use primes::is_prime_miller_rabin;
use std::collections::HashMap;

/// Generate a random seed with exactly `length` digits in the given base
fn random_seed_with_length(base: u64, length: usize, rng: &mut u64) -> BigUint {
    if length == 0 {
        return BigUint::ZERO;
    }

    // Simple LCG for reproducibility
    let mut next = || {
        *rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        *rng
    };

    let b = BigUint::from(base);

    // First digit: 1 to base-1 (ensure length is exactly right)
    let first = (next() % (base - 1)) + 1;
    let mut seed = BigUint::from(first);

    // Remaining digits: 0 to base-1
    for _ in 1..length {
        let digit = next() % base;
        seed = seed * &b + BigUint::from(digit);
    }

    seed
}

/// Count base-ary digits of a BigUint
fn count_digits(n: &BigUint, base: u64) -> usize {
    if *n == BigUint::ZERO {
        return 1;
    }
    let mut count = 0;
    let mut temp = n.clone();
    let b = BigUint::from(base);
    while temp > BigUint::ZERO {
        temp /= &b;
        count += 1;
    }
    count
}

/// Build membrane value: L | seed | R in given base
fn membrane_value(base: u64, left: u64, seed: &BigUint, right: u64) -> BigUint {
    let b = BigUint::from(base);
    let seed_digits = count_digits(seed, base) as u32;

    // value = left * base^(seed_digits+1) + seed * base + right
    BigUint::from(left) * b.pow(seed_digits + 1) + seed * &b + BigUint::from(right)
}

/// Find first digit coprime to base
fn first_coprime(base: u64) -> u64 {
    let factors: Vec<u64> = {
        let mut n = base;
        let mut fs = vec![];
        for p in [2, 3, 5, 7, 11, 13, 17, 19, 23] {
            if n % p == 0 {
                fs.push(p);
                while n % p == 0 { n /= p; }
            }
        }
        fs
    };

    (1..base).find(|&d| factors.iter().all(|&p| d % p != 0)).unwrap_or(1)
}

/// Compute ord_n(10) - multiplicative order of 10 mod n
fn ord_10_mod(n: u64) -> u64 {
    if n <= 1 { return 0; }
    let mut power = 10u64 % n;
    let mut order = 1u64;
    while power != 1 && order < n {
        power = (power * 10) % n;
        order += 1;
    }
    if power == 1 { order } else { 0 } // 0 means 10 and n not coprime
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Theoretical period for decimal-base interference
fn theoretical_period(base: u64) -> u64 {
    let g = gcd(10, base);
    let coprime_part = base / g;
    if coprime_part <= 1 { return 1; }
    ord_10_mod(coprime_part)
}

fn main() {
    println!("=== PRIMORIAL RESONANCE HUNT ===\n");
    println!("Testing hypothesis: bases 210+ show period-6 oscillation in success rate\n");

    // Primorial bases with their theoretical periods
    let bases: Vec<(u64, &str)> = vec![
        (6, "P₂ = 2×3"),
        (30, "P₃ = 2×3×5"),
        (210, "P₄ = 2×3×5×7"),
        (2310, "P₅ = 2×3×5×7×11"),
    ];

    // First, verify theoretical periods
    println!("THEORETICAL PERIOD ANALYSIS");
    println!("{}", "=".repeat(50));
    println!("{:>8} {:>20} {:>10} {:>10}", "Base", "Name", "Coprime", "Period");
    println!("{}", "-".repeat(50));

    for (base, name) in &bases {
        let g = gcd(10, *base);
        let coprime = base / g;
        let period = theoretical_period(*base);
        println!("{:>8} {:>20} {:>10} {:>10}", base, name, coprime, period);
    }

    println!("\n\nEMPIRICAL RESONANCE SCAN");
    println!("{}", "=".repeat(70));

    let samples_per_length = 300;
    let max_seed_length = 24; // 4 complete periods of 6

    // Store results: base -> (seed_length -> success_rate)
    let mut results: HashMap<u64, Vec<(usize, f64, usize)>> = HashMap::new();

    for (base, name) in &bases {
        println!("\n{} (base {})", name, base);
        println!("{}", "-".repeat(50));

        let right = first_coprime(*base);
        let mut base_results = Vec::new();
        let mut rng = 12345u64 + base; // Reproducible per-base

        println!("{:>6} {:>8} {:>10} {:>12}", "Length", "Primes", "Rate%", "Residue");

        for seed_len in 1..=max_seed_length {
            let mut primes_found = 0;

            for _ in 0..samples_per_length {
                let seed = random_seed_with_length(*base, seed_len, &mut rng);
                let membrane = membrane_value(*base, 1, &seed, right);

                if is_prime_miller_rabin(&membrane) {
                    primes_found += 1;
                }
            }

            let rate = primes_found as f64 / samples_per_length as f64;
            let residue = seed_len % 6;

            // Mark potential resonance points
            let marker = if rate > 0.35 { " ▲" } else if rate < 0.20 { " ▼" } else { "" };

            println!("{:>6} {:>8} {:>10.1} {:>12}{}",
                     seed_len, primes_found, rate * 100.0, residue, marker);

            base_results.push((seed_len, rate, residue));
        }

        results.insert(*base, base_results);
    }

    // Analyze for periodicity
    println!("\n\n{}", "=".repeat(70));
    println!("PERIODICITY ANALYSIS");
    println!("{}", "=".repeat(70));

    for (base, name) in &bases {
        let data = results.get(base).unwrap();
        let theoretical = theoretical_period(*base);

        println!("\n{} (theoretical period: {})", name, theoretical);

        // Group by residue mod 6
        let mut by_residue: HashMap<usize, Vec<f64>> = HashMap::new();
        for (_, rate, residue) in data {
            by_residue.entry(*residue).or_default().push(*rate);
        }

        println!("  Mean success rate by (seed_length mod 6):");
        let mut residue_means: Vec<(usize, f64, f64)> = vec![];

        for r in 0..6 {
            if let Some(rates) = by_residue.get(&r) {
                let mean = rates.iter().sum::<f64>() / rates.len() as f64;
                let variance = rates.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / rates.len() as f64;
                let std = variance.sqrt();
                residue_means.push((r, mean, std));
                println!("    mod 6 ≡ {}: {:.1}% ± {:.1}%", r, mean * 100.0, std * 100.0);
            }
        }

        // Check for significant variation (evidence of periodicity)
        let means: Vec<f64> = residue_means.iter().map(|(_, m, _)| *m).collect();
        let overall_mean = means.iter().sum::<f64>() / means.len() as f64;
        let variation = means.iter().map(|m| (m - overall_mean).powi(2)).sum::<f64>().sqrt();

        let has_periodicity = variation > 0.02; // 2% threshold
        println!("  Variation from mean: {:.2}%", variation * 100.0);
        println!("  Periodicity detected: {}", if has_periodicity { "YES ⚡" } else { "no" });
    }

    // Look for the specific period-6 pattern in bases 210+
    println!("\n\n{}", "=".repeat(70));
    println!("PERIOD-6 HYPOTHESIS TEST");
    println!("{}", "=".repeat(70));

    println!("\nComparing residue-class success rates across bases:\n");
    println!("{:>12} {:>10} {:>10} {:>10} {:>10}", "Residue", "Base 30", "Base 210", "Base 2310", "Δ(210-30)");
    println!("{}", "-".repeat(55));

    for r in 0..6 {
        let get_mean = |base: u64| -> f64 {
            let data = results.get(&base).unwrap();
            let rates: Vec<f64> = data.iter()
                .filter(|(_, _, res)| *res == r)
                .map(|(_, rate, _)| *rate)
                .collect();
            if rates.is_empty() { 0.0 } else { rates.iter().sum::<f64>() / rates.len() as f64 }
        };

        let m30 = get_mean(30);
        let m210 = get_mean(210);
        let m2310 = get_mean(2310);
        let delta = m210 - m30;

        let marker = if delta.abs() > 0.03 { " ⬅" } else { "" };

        println!("{:>12} {:>10.1} {:>10.1} {:>10.1} {:>+10.1}{}",
                 r, m30 * 100.0, m210 * 100.0, m2310 * 100.0, delta * 100.0, marker);
    }

    // Final verdict
    println!("\n\n{}", "=".repeat(70));
    println!("CONCLUSIONS");
    println!("{}", "=".repeat(70));

    // Calculate overall statistics
    let base30_var = {
        let data = results.get(&30).unwrap();
        let mut by_res: HashMap<usize, Vec<f64>> = HashMap::new();
        for (_, rate, res) in data { by_res.entry(*res).or_default().push(*rate); }
        let means: Vec<f64> = (0..6).filter_map(|r| {
            by_res.get(&r).map(|v| v.iter().sum::<f64>() / v.len() as f64)
        }).collect();
        let overall = means.iter().sum::<f64>() / means.len() as f64;
        means.iter().map(|m| (m - overall).powi(2)).sum::<f64>().sqrt()
    };

    let base210_var = {
        let data = results.get(&210).unwrap();
        let mut by_res: HashMap<usize, Vec<f64>> = HashMap::new();
        for (_, rate, res) in data { by_res.entry(*res).or_default().push(*rate); }
        let means: Vec<f64> = (0..6).filter_map(|r| {
            by_res.get(&r).map(|v| v.iter().sum::<f64>() / v.len() as f64)
        }).collect();
        let overall = means.iter().sum::<f64>() / means.len() as f64;
        means.iter().map(|m| (m - overall).powi(2)).sum::<f64>().sqrt()
    };

    println!("\n1. PERIOD-6 VARIATION:");
    println!("   Base 30:  {:.2}% variation across residue classes", base30_var * 100.0);
    println!("   Base 210: {:.2}% variation across residue classes", base210_var * 100.0);

    if base210_var > base30_var * 1.5 {
        println!("\n   ⚡ Base 210 shows {:.1}× MORE variation than Base 30!", base210_var / base30_var);
        println!("   This SUPPORTS the period-6 hypothesis!");
    } else if base30_var > base210_var * 1.5 {
        println!("\n   ⚠ Base 30 shows MORE variation - unexpected!");
        println!("   The period-6 hypothesis may need revision.");
    } else {
        println!("\n   ≈ Similar variation across bases.");
        println!("   Period-6 effect may be weak or masked by noise.");
    }

    // Look for consistent peaks/valleys
    println!("\n2. PEAK/VALLEY POSITIONS:");

    for (base, name) in &bases {
        let data = results.get(base).unwrap();
        let mut by_res: HashMap<usize, f64> = HashMap::new();
        let mut by_res_count: HashMap<usize, usize> = HashMap::new();

        for (_, rate, res) in data {
            *by_res.entry(*res).or_default() += rate;
            *by_res_count.entry(*res).or_default() += 1;
        }

        let means: Vec<(usize, f64)> = (0..6)
            .filter_map(|r| {
                by_res.get(&r).map(|sum| (r, sum / *by_res_count.get(&r).unwrap() as f64))
            })
            .collect();

        let (peak_r, peak_v) = means.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap();
        let (valley_r, valley_v) = means.iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap();

        println!("   {}: peak at mod6≡{} ({:.1}%), valley at mod6≡{} ({:.1}%)",
                 name, peak_r, peak_v * 100.0, valley_r, valley_v * 100.0);
    }

    println!("\n3. RECOMMENDATIONS:");
    println!("   - If peaks align across bases 210+, the period-6 effect is real");
    println!("   - Different peak positions suggest base-specific resonances");
    println!("   - Consider testing with larger sample sizes for confirmation");
}
