//! P₈ High-Power Verification
//!
//! The initial P₈ test showed a DROP in efficiency. Let's verify with
//! more samples and different RNG seeds to confirm this isn't noise.

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

fn test_base(base: u64, name: &str, samples_per_length: usize, rng_seed: u64) -> (u64, f64, usize, usize) {
    let right = first_coprime(base);
    let mut rng = rng_seed;

    let seed_lengths: Vec<usize> = (4..=16).collect();

    let mut total_primes = 0;
    let mut total_tests = 0;
    let mut total_digits = 0.0;

    for &seed_len in &seed_lengths {
        for _ in 0..samples_per_length {
            let seed = random_seed_with_length(base, seed_len, &mut rng);
            let mem = membrane_value(base, 1, &seed, right);
            total_digits += mem.to_string().len() as f64;
            total_tests += 1;
            if is_prime_miller_rabin(&mem) {
                total_primes += 1;
            }
        }
    }

    let rate = total_primes as f64 / total_tests as f64;
    let mean_digits = total_digits / total_tests as f64;
    let pnt_expected = 1.0 / (mean_digits * 2.303);
    let efficiency = rate / pnt_expected;

    println!("  {:>12} {:>30} {:>8} / {:>8} = {:>5.1}% → {:.3}×",
             base, name, total_primes, total_tests, rate * 100.0, efficiency);

    (base, efficiency, total_primes, total_tests)
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           P₈ HIGH-POWER VERIFICATION                             ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // First, verify the coprime digit calculation
    let p8 = 9699690u64;
    let fc = first_coprime(p8);
    println!("P₈ = {} = 2×3×5×7×11×13×17×19", p8);
    println!("First coprime digit: {}\n", fc);

    // Run multiple trials with different RNG seeds
    let trials = 5;
    let samples_per_length = 300;  // 300 × 13 lengths = 3900 tests per trial

    println!("Running {} trials with {} samples/length ({} tests/trial)...\n",
             trials, samples_per_length, samples_per_length * 13);

    // Reference: P₇ with same parameters for fair comparison
    println!("P₇ = 510510 reference:");
    let mut p7_effs = vec![];
    for trial in 0..trials {
        let rng_seed = 42424242 + trial * 12345;
        let (_, eff, _, _) = test_base(510510, "P₇", samples_per_length, rng_seed);
        p7_effs.push(eff);
    }
    let p7_mean: f64 = p7_effs.iter().sum::<f64>() / trials as f64;
    let p7_std: f64 = (p7_effs.iter().map(|e| (e - p7_mean).powi(2)).sum::<f64>() / (trials - 1) as f64).sqrt();
    println!("\n  P₇ mean efficiency: {:.3} ± {:.3} (std)\n", p7_mean, p7_std);

    // P₈ with same parameters
    println!("P₈ = 9699690 test:");
    let mut p8_effs = vec![];
    for trial in 0..trials {
        let rng_seed = 42424242 + trial * 12345;
        let (_, eff, _, _) = test_base(9699690, "P₈", samples_per_length, rng_seed);
        p8_effs.push(eff);
    }
    let p8_mean: f64 = p8_effs.iter().sum::<f64>() / trials as f64;
    let p8_std: f64 = (p8_effs.iter().map(|e| (e - p8_mean).powi(2)).sum::<f64>() / (trials - 1) as f64).sqrt();
    println!("\n  P₈ mean efficiency: {:.3} ± {:.3} (std)\n", p8_mean, p8_std);

    // Statistical comparison
    println!("{}", "═".repeat(70));
    println!("STATISTICAL COMPARISON");
    println!("{}", "═".repeat(70));

    let diff = p8_mean - p7_mean;
    let pooled_std = ((p7_std.powi(2) + p8_std.powi(2)) / 2.0).sqrt();
    let effect_size = diff / pooled_std;

    println!("\nP₇ efficiency: {:.3} ± {:.3}", p7_mean, p7_std);
    println!("P₈ efficiency: {:.3} ± {:.3}", p8_mean, p8_std);
    println!("\nDifference: {:+.3} ({:+.1}%)", diff, diff / p7_mean * 100.0);
    println!("Effect size (Cohen's d): {:.2}", effect_size);

    if diff < 0.0 {
        println!("\n⚠️  CONFIRMED: P₈ efficiency is LOWER than P₇");
        println!("   The primorial efficiency scaling has a PEAK somewhere around P₆-P₇.");
    } else {
        println!("\n✓ P₈ shows improvement over P₇");
    }

    // Try to understand why
    println!("\n{}", "═".repeat(70));
    println!("DIAGNOSTIC: Seed length vs efficiency");
    println!("{}", "═".repeat(70));

    println!("\nChecking if certain seed lengths work better...\n");

    let mut rng = 99999999u64;
    let samples = 200;

    for &(base, name) in &[(510510u64, "P₇"), (9699690u64, "P₈")] {
        println!("{}:", name);
        let right = first_coprime(base);

        for seed_len in [4, 8, 12, 16] {
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

            println!("  seed_len={:2}: {:3}/{} = {:5.1}% → {:.2}× ({:.0} avg digits)",
                     seed_len, primes, samples, rate * 100.0, eff, mean_digits);
        }
        println!();
    }
}
