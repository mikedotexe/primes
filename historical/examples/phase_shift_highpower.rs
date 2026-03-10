//! High-Power Phase Shift Analysis
//!
//! The previous analysis showed statistical instability.
//! This uses 2000 samples per length to get stable rankings.

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
        for p in [2, 3, 5, 7, 11, 13, 17, 19] {
            if n % p == 0 { fs.push(p); while n % p == 0 { n /= p; } }
        }
        fs
    };
    (1..base).find(|&d| factors.iter().all(|&p| d % p != 0)).unwrap_or(1)
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║         HIGH-POWER PHASE SHIFT ANALYSIS (2000 samples)          ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let primorials: Vec<(u64, &str)> = vec![
        (30, "P₃"),
        (210, "P₄"),
        (2310, "P₅"),
        (30030, "P₆"),
    ];

    let samples = 2000;  // High power
    let test_lengths: Vec<usize> = (6..=48).collect(); // 7 complete periods

    println!("Configuration: {} samples × {} lengths = {} tests per base\n",
             samples, test_lengths.len(), samples * test_lengths.len());

    let mut all_rankings: Vec<(u64, Vec<(usize, f64, f64)>)> = vec![];

    for (base, name) in &primorials {
        print!("Testing {} (base {})... ", name, base);
        std::io::Write::flush(&mut std::io::stdout()).ok();

        let right = first_coprime(*base);
        let mut rng = 271828u64 + base; // Fixed seed for reproducibility

        let mut by_mod6: HashMap<usize, Vec<f64>> = HashMap::new();

        for &seed_len in &test_lengths {
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

            by_mod6.entry(seed_len % 6).or_default().push(eff);
        }

        // Calculate statistics with standard error
        let mut stats: Vec<(usize, f64, f64)> = vec![];
        for r in 0..6 {
            let effs = by_mod6.get(&r).unwrap();
            let n = effs.len() as f64;
            let mean = effs.iter().sum::<f64>() / n;
            let var = effs.iter().map(|e| (e - mean).powi(2)).sum::<f64>() / n;
            let se = (var / n).sqrt(); // Standard error of mean
            stats.push((r, mean, se));
        }

        stats.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        all_rankings.push((*base, stats.clone()));

        println!("done");

        // Print ranking with confidence
        println!("\n  Ranking (with standard error):");
        for (i, (r, eff, se)) in stats.iter().enumerate() {
            let marker = match i {
                0 => "★ BEST",
                5 => "✗ WORST",
                _ => "",
            };
            println!("    {:>2}. mod6≡{}: {:.3} ± {:.3}  {}", i+1, r, eff, se, marker);
        }

        let best = &stats[0];
        let worst = &stats[5];
        let gap = best.1 - worst.1;
        let combined_se = (best.2.powi(2) + worst.2.powi(2)).sqrt();
        let z_score = gap / combined_se;

        println!("\n  Gap analysis:");
        println!("    Best - Worst = {:.3} ± {:.3}", gap, combined_se);
        println!("    z-score = {:.1} (significant if > 2)", z_score);
        println!("    Relative gain: {:.1}%", 100.0 * gap / worst.1);
        println!();
    }

    // Summary table
    println!("\n{}", "═".repeat(70));
    println!("SUMMARY: STABLE RANKINGS");
    println!("{}", "═".repeat(70));

    println!("\n{:>8} {:>10} {:>10} {:>12} {:>12}",
             "Base", "BEST", "WORST", "Gain%", "z-score");
    println!("{}", "-".repeat(55));

    for (base, stats) in &all_rankings {
        let best = stats[0].0;
        let worst = stats[5].0;
        let gain = 100.0 * (stats[0].1 / stats[5].1 - 1.0);
        let gap = stats[0].1 - stats[5].1;
        let combined_se = (stats[0].2.powi(2) + stats[5].2.powi(2)).sqrt();
        let z = gap / combined_se;

        println!("{:>8} {:>10} {:>10} {:>11.1}% {:>12.1}",
                 base,
                 format!("mod6≡{}", best),
                 format!("mod6≡{}", worst),
                 gain, z);
    }

    // Look for patterns
    println!("\n\n{}", "═".repeat(70));
    println!("PATTERN SEARCH");
    println!("{}", "═".repeat(70));

    println!("\nObserved optimal phases:");
    for (base, stats) in &all_rankings {
        println!("  Base {:>5}: mod6≡{}", base, stats[0].0);
    }

    // Check: is there a relationship with base mod 6?
    println!("\nBase mod 6 relationship:");
    for (base, stats) in &all_rankings {
        println!("  Base {:>5} ≡ {} (mod 6), optimal ≡ {} (mod 6)",
                 base, base % 6, stats[0].0);
    }

    // Check: is there a relationship with number of odd prime factors?
    println!("\nNumber of odd prime factors:");
    let factor_counts = [(30, 1), (210, 2), (2310, 3), (30030, 4)];
    for ((base, stats), (_, count)) in all_rankings.iter().zip(factor_counts.iter()) {
        println!("  Base {:>5}: {} odd primes (beyond 2,5), optimal ≡ {}",
                 base, count, stats[0].0);
    }
}
