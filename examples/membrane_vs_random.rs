//! Membrane vs Random Coprime Numbers
//!
//! Question: Does the L|seed|R structure provide advantage over
//! random numbers that are merely coprime to the base?
//!
//! Test: Generate random coprime numbers of same size and compare prime rates

use num_bigint::BigUint;
use primes::is_prime_miller_rabin;

fn random_seed_with_length(base: u64, length: usize, rng: &mut u64) -> BigUint {
    if length == 0 {
        return BigUint::ZERO;
    }
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
    while t > BigUint::ZERO {
        t /= &b;
        sd += 1;
    }
    if sd == 0 {
        sd = 1;
    }
    BigUint::from(left) * b.pow(sd + 1) + seed * &b + BigUint::from(right)
}

fn is_coprime_to_base(n: &BigUint, base: u64) -> bool {
    // Check coprimality by testing divisibility by small primes dividing base
    let base_primes: Vec<u64> = {
        let mut b = base;
        let mut ps = vec![];
        for p in [2, 3, 5, 7, 11, 13, 17, 19, 23] {
            if b.is_multiple_of(p) {
                ps.push(p);
                while b.is_multiple_of(p) {
                    b /= p;
                }
            }
        }
        ps
    };

    for p in base_primes {
        if n % p == BigUint::ZERO {
            return false;
        }
    }
    true
}

fn random_coprime_number(base: u64, target_digits: usize, rng: &mut u64) -> BigUint {
    // Generate random number of approximately target_digits decimal digits
    // that is coprime to base
    let mut next = || {
        *rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        *rng
    };

    loop {
        // Build random BigUint digit by digit (decimal)
        let first = (next() % 9) + 1; // 1-9 for first digit
        let mut n = BigUint::from(first);

        for _ in 1..target_digits {
            n = n * 10u64 + (next() % 10);
        }

        if is_coprime_to_base(&n, base) {
            return n;
        }
        // If not coprime, try again
    }
}

fn euler_phi_fast(mut n: u64) -> u64 {
    let mut result = n;
    let mut p = 2u64;
    while p * p <= n {
        if n.is_multiple_of(p) {
            while n.is_multiple_of(p) {
                n /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    result
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           MEMBRANE vs RANDOM COPRIME NUMBERS                     ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let base = 30030u64;
    let phi = euler_phi_fast(base);
    let theoretical = base as f64 / phi as f64;

    println!("Base: {} (P₆)", base);
    println!(
        "Theoretical efficiency (random coprime): B/φ(B) = {:.3}×\n",
        theoretical
    );

    let samples = 5000;
    let digit_targets = [30, 40, 50, 60, 70];

    println!("{}", "═".repeat(75));
    println!("TEST 1: Membrane L|seed|R efficiency");
    println!("{}", "═".repeat(75));

    let mut membrane_results: Vec<(usize, f64)> = vec![];

    println!(
        "\n{:>10} {:>10} {:>10} {:>10} {:>12}",
        "~digits", "primes", "tests", "rate%", "efficiency"
    );
    println!("{}", "-".repeat(60));

    for &target in &digit_targets {
        // Calculate seed length to get approximately target digits
        // membrane = L * B^(s+1) + seed * B + R
        // digits ≈ (s+2) * log10(B) ≈ (s+2) * 4.48
        let seed_len = ((target as f64 / 4.48) - 2.0).max(2.0) as usize;

        let mut rng = 12345678u64 + target as u64 * 999;
        let mut primes = 0;
        let mut total_digits = 0.0;

        for _ in 0..samples {
            let seed = random_seed_with_length(base, seed_len, &mut rng);
            let mem = membrane_value(base, 1, &seed, 1);
            total_digits += mem.to_string().len() as f64;
            if is_prime_miller_rabin(&mem) {
                primes += 1;
            }
        }

        let rate = primes as f64 / samples as f64;
        let mean_digits = total_digits / samples as f64;
        let pnt_expected = 1.0 / (mean_digits * 2.303);
        let efficiency = rate / pnt_expected;

        membrane_results.push((mean_digits as usize, efficiency));

        println!(
            "{:>10} {:>10} {:>10} {:>9.2}% {:>12.3}",
            mean_digits as usize,
            primes,
            samples,
            rate * 100.0,
            efficiency
        );
    }

    println!("{}", "═".repeat(75));
    println!("TEST 2: Random coprime numbers (same digit counts)");
    println!("{}", "═".repeat(75));

    let mut random_results: Vec<(usize, f64)> = vec![];

    println!(
        "\n{:>10} {:>10} {:>10} {:>10} {:>12}",
        "~digits", "primes", "tests", "rate%", "efficiency"
    );
    println!("{}", "-".repeat(60));

    for &(target_digits, _) in &membrane_results {
        let mut rng = 98765432u64 + target_digits as u64 * 777;
        let mut primes = 0;
        let mut total_digits = 0.0;

        for _ in 0..samples {
            let n = random_coprime_number(base, target_digits, &mut rng);
            total_digits += n.to_string().len() as f64;
            if is_prime_miller_rabin(&n) {
                primes += 1;
            }
        }

        let rate = primes as f64 / samples as f64;
        let mean_digits = total_digits / samples as f64;
        let pnt_expected = 1.0 / (mean_digits * 2.303);
        let efficiency = rate / pnt_expected;

        random_results.push((mean_digits as usize, efficiency));

        println!(
            "{:>10} {:>10} {:>10} {:>9.2}% {:>12.3}",
            mean_digits as usize,
            primes,
            samples,
            rate * 100.0,
            efficiency
        );
    }

    // Comparison
    println!("\n{}", "═".repeat(75));
    println!("COMPARISON: Membrane vs Random Coprime");
    println!("{}", "═".repeat(75));

    println!(
        "\n{:>10} {:>15} {:>15} {:>15}",
        "digits", "Membrane", "Random", "Advantage"
    );
    println!("{}", "-".repeat(60));

    let mut total_advantage = 0.0;
    let mut count = 0;

    for ((d1, mem_eff), (_, rand_eff)) in membrane_results.iter().zip(random_results.iter()) {
        let advantage = (mem_eff - rand_eff) / rand_eff * 100.0;
        total_advantage += advantage;
        count += 1;

        println!(
            "{:>10} {:>15.3}× {:>15.3}× {:>+14.1}%",
            d1, mem_eff, rand_eff, advantage
        );
    }

    let mean_advantage = total_advantage / count as f64;

    println!("\nMean membrane advantage: {:+.1}%", mean_advantage);

    if mean_advantage > 5.0 {
        println!("\n✓ MEMBRANE STRUCTURE provides significant extra efficiency!");
        println!("  The L|seed|R form is better than random coprime numbers.");
    } else if mean_advantage > 0.0 {
        println!("\n≈ Membrane has slight advantage, but mostly matches theory.");
    } else {
        println!("\n✗ No membrane advantage - efficiency matches random coprime.");
    }

    println!("\n{}", "═".repeat(75));
    println!("THEORETICAL INTERPRETATION");
    println!("{}", "═".repeat(75));

    let mem_mean: f64 =
        membrane_results.iter().map(|(_, e)| *e).sum::<f64>() / membrane_results.len() as f64;
    let rand_mean: f64 =
        random_results.iter().map(|(_, e)| *e).sum::<f64>() / random_results.len() as f64;

    println!("\nTheoretical bound B/φ(B):     {:.3}×", theoretical);
    println!(
        "Random coprime efficiency:    {:.3}× ({:+.3} vs theory)",
        rand_mean,
        rand_mean - theoretical
    );
    println!(
        "Membrane efficiency:          {:.3}× ({:+.3} vs theory)",
        mem_mean,
        mem_mean - theoretical
    );

    if (rand_mean - theoretical).abs() < 0.3 {
        println!("\n→ Random coprime matches theory (as expected)");
    }
    if mem_mean > rand_mean + 0.2 {
        println!(
            "→ Membrane EXCEEDS random coprime by {:.3}×",
            mem_mean - rand_mean
        );
        println!("  This extra efficiency is inherent to the membrane structure!");
    }
}
