//! P₈ Seed Length Sweep
//!
//! The diagnostic showed P₈ hitting 8.79× at seed_len=8.
//! Let's systematically sweep seed lengths to find the optimal zone.

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
        for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31] {
            if n % p == 0 { fs.push(p); while n % p == 0 { n /= p; } }
        }
        fs
    };
    (1..base).find(|&d| factors.iter().all(|&p| d % p != 0)).unwrap_or(1)
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           P₈ SEED LENGTH SWEEP                                   ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let bases: Vec<(u64, &str)> = vec![
        (30030, "P₆"),
        (510510, "P₇"),
        (9699690, "P₈"),
    ];

    let samples = 500;  // Per seed length

    for (base, name) in &bases {
        println!("\n{} = {} (R={})\n", name, base, first_coprime(*base));
        println!("{:>10} {:>8} {:>8} {:>8} {:>10} {:>8}",
                 "seed_len", "primes", "tests", "rate%", "efficiency", "mod6");
        println!("{}", "-".repeat(60));

        let right = first_coprime(*base);
        let mut best_eff = 0.0f64;
        let mut best_len = 0usize;

        for seed_len in 2..=20 {
            let mut rng = 42424242u64 + *base + seed_len as u64 * 1000;
            let mut primes = 0;
            let mut digits_sum = 0.0;

            for _ in 0..samples {
                let seed = random_seed_with_length(*base, seed_len, &mut rng);
                let mem = membrane_value(*base, 1, &seed, right);
                digits_sum += mem.to_string().len() as f64;
                if is_prime_miller_rabin(&mem) {
                    primes += 1;
                }
            }

            let rate = primes as f64 / samples as f64;
            let mean_digits = digits_sum / samples as f64;
            let pnt_expected = 1.0 / (mean_digits * 2.303);
            let eff = rate / pnt_expected;

            let marker = if eff > best_eff { "★" } else { "" };
            if eff > best_eff {
                best_eff = eff;
                best_len = seed_len;
            }

            println!("{:>10} {:>8} {:>8} {:>7.1}% {:>10.3} {:>8} {}",
                     seed_len, primes, samples, rate * 100.0, eff, seed_len % 6, marker);
        }

        println!("\n  Best: seed_len={} → {:.3}× efficiency (mod6={})",
                 best_len, best_eff, best_len % 6);
    }

    // Period-6 analysis for P₈
    println!("\n\n{}", "═".repeat(70));
    println!("PERIOD-6 ANALYSIS FOR P₈");
    println!("{}", "═".repeat(70));

    let base = 9699690u64;
    let right = first_coprime(base);
    let samples = 1000;  // Higher samples for period-6 analysis

    println!("\nAggregating efficiency by seed_len mod 6...\n");

    let mut mod6_effs: [Vec<f64>; 6] = Default::default();

    for seed_len in 2..=20 {
        let mut rng = 99999999u64 + seed_len as u64 * 7777;
        let mut primes = 0;
        let mut digits_sum = 0.0;

        for _ in 0..samples {
            let seed = random_seed_with_length(base, seed_len, &mut rng);
            let mem = membrane_value(base, 1, &seed, right);
            digits_sum += mem.to_string().len() as f64;
            if is_prime_miller_rabin(&mem) {
                primes += 1;
            }
        }

        let rate = primes as f64 / samples as f64;
        let mean_digits = digits_sum / samples as f64;
        let pnt_expected = 1.0 / (mean_digits * 2.303);
        let eff = rate / pnt_expected;

        mod6_effs[seed_len % 6].push(eff);
    }

    println!("{:>8} {:>12} {:>12} {:>10}", "mod6", "mean_eff", "std", "count");
    println!("{}", "-".repeat(50));

    let mut best_mod6 = 0;
    let mut best_mean = 0.0;

    for m in 0..6 {
        if mod6_effs[m].is_empty() { continue; }
        let mean: f64 = mod6_effs[m].iter().sum::<f64>() / mod6_effs[m].len() as f64;
        let std: f64 = if mod6_effs[m].len() > 1 {
            (mod6_effs[m].iter().map(|e| (e - mean).powi(2)).sum::<f64>()
             / (mod6_effs[m].len() - 1) as f64).sqrt()
        } else { 0.0 };

        let marker = if mean > best_mean { "★" } else { "" };
        if mean > best_mean {
            best_mean = mean;
            best_mod6 = m;
        }

        println!("{:>8} {:>12.3} {:>12.3} {:>10} {}",
                 m, mean, std, mod6_effs[m].len(), marker);
    }

    println!("\nBest mod6 class for P₈: {} → {:.3}× mean efficiency", best_mod6, best_mean);

    // Calculate period-6 gain potential
    let all_effs: Vec<f64> = mod6_effs.iter().flatten().copied().collect();
    let overall_mean: f64 = all_effs.iter().sum::<f64>() / all_effs.len() as f64;
    let gain_potential = (best_mean - overall_mean) / overall_mean * 100.0;

    println!("\nPeriod-6 gain potential: {:.1}% (best mod6 vs overall average)", gain_potential);
}
