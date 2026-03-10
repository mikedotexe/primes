//! Phase Shift Stability Test
//!
//! Tests whether optimal phase is consistent across different RNG seeds.
//! If the effect is real, the same phase should win repeatedly.

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

fn measure_optimal_phase(base: u64, rng_seed: u64, samples: usize) -> (usize, f64) {
    let right = first_coprime(base);
    let mut rng = rng_seed;
    let test_lengths: Vec<usize> = (6..=36).collect();

    let mut by_mod6: HashMap<usize, Vec<f64>> = HashMap::new();

    for &seed_len in &test_lengths {
        let mut primes = 0;
        let mut total_dig = 0.0;

        for _ in 0..samples {
            let seed = random_seed_with_length(base, seed_len, &mut rng);
            let mem = membrane_value(base, 1, &seed, right);
            total_dig += mem.to_string().len() as f64;
            if is_prime_miller_rabin(&mem) { primes += 1; }
        }

        let rate = primes as f64 / samples as f64;
        let mean_dig = total_dig / samples as f64;
        let eff = rate / (1.0 / (mean_dig * 2.303));

        by_mod6.entry(seed_len % 6).or_default().push(eff);
    }

    let mut stats: Vec<(usize, f64)> = (0..6).map(|r| {
        let effs = by_mod6.get(&r).unwrap();
        let mean = effs.iter().sum::<f64>() / effs.len() as f64;
        (r, mean)
    }).collect();

    stats.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let best = stats[0].0;
    let worst_eff = stats[5].1;
    let gain = 100.0 * (stats[0].1 / worst_eff - 1.0);

    (best, gain)
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║            PHASE SHIFT STABILITY TEST (10 trials)               ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let bases: Vec<(u64, &str)> = vec![
        (210, "P₄ = 2×3×5×7"),
        (2310, "P₅ = 2×3×5×7×11"),
    ];

    let trials = 10;
    let samples = 800;
    let rng_seeds: Vec<u64> = vec![
        12345, 67890, 11111, 22222, 33333,
        44444, 55555, 99999, 13579, 24680
    ];

    for (base, name) in &bases {
        println!("{}", "═".repeat(60));
        println!("{} (base {})", name, base);
        println!("{}", "═".repeat(60));

        let mut winners: HashMap<usize, usize> = HashMap::new();
        let mut gains: Vec<f64> = vec![];

        println!("\nTrial results:");
        for (i, &seed) in rng_seeds.iter().enumerate() {
            let (best_phase, gain) = measure_optimal_phase(*base, seed, samples);
            *winners.entry(best_phase).or_insert(0) += 1;
            gains.push(gain);
            println!("  Trial {:>2}: optimal = mod6≡{}, gain = {:.1}%",
                     i + 1, best_phase, gain);
        }

        println!("\nWinner frequency:");
        let mut winner_vec: Vec<_> = winners.into_iter().collect();
        winner_vec.sort_by(|a, b| b.1.cmp(&a.1));
        for (phase, count) in &winner_vec {
            let pct = 100.0 * *count as f64 / trials as f64;
            println!("  mod6≡{}: {} wins ({:.0}%)", phase, count, pct);
        }

        let mean_gain = gains.iter().sum::<f64>() / trials as f64;
        let var = gains.iter().map(|g| (g - mean_gain).powi(2)).sum::<f64>() / trials as f64;
        let std = var.sqrt();

        println!("\nGain statistics:");
        println!("  Mean gain: {:.1}% ± {:.1}%", mean_gain, std);
        println!("  Range: {:.1}% to {:.1}%",
                 gains.iter().cloned().fold(f64::MAX, f64::min),
                 gains.iter().cloned().fold(f64::MIN, f64::max));

        // Stability assessment
        let top_winner = &winner_vec[0];
        let consistency = 100.0 * top_winner.1 as f64 / trials as f64;
        let verdict = if consistency >= 70.0 {
            "STABLE - same phase wins consistently"
        } else if consistency >= 40.0 {
            "WEAK - some signal but noisy"
        } else {
            "UNSTABLE - no consistent winner"
        };
        println!("\nStability: {} ({:.0}% consistency)", verdict, consistency);
        println!();
    }

    println!("{}", "═".repeat(60));
    println!("CONCLUSION");
    println!("{}", "═".repeat(60));
    println!("\nIf a phase wins >70% of trials, the effect is robust.");
    println!("If no phase dominates, the period-6 effect may be too weak");
    println!("to reliably exploit in practice.");
}
