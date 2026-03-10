//! Membrane vs Random Coprime Numbers (Fast Version)
//!
//! Use smaller base (30) where coprimality is more common (~27%)
//! to get faster rejection sampling.

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

fn is_coprime_to_30(n: &BigUint) -> bool {
    // 30 = 2 × 3 × 5
    if n % 2u64 == BigUint::ZERO {
        return false;
    }
    if n % 3u64 == BigUint::ZERO {
        return false;
    }
    if n % 5u64 == BigUint::ZERO {
        return false;
    }
    true
}

fn random_coprime_30(target_digits: usize, rng: &mut u64) -> BigUint {
    let mut next = || {
        *rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        *rng
    };

    // φ(30)/30 = 8/30 ≈ 26.7%, so rejection is fast
    loop {
        let first = (next() % 9) + 1;
        let mut n = BigUint::from(first);
        for _ in 1..target_digits {
            n = n * 10u64 + (next() % 10);
        }
        if is_coprime_to_30(&n) {
            return n;
        }
    }
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           MEMBRANE vs RANDOM COPRIME (Base 30)                   ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let base = 30u64;
    let theoretical = 30.0 / 8.0; // B/φ(B) = 30/8 = 3.75

    println!("Base: {} (P₃ = 2×3×5)", base);
    println!(
        "Theoretical efficiency (random coprime): B/φ(B) = {:.3}×\n",
        theoretical
    );

    let samples = 3000;
    let seed_lengths: Vec<usize> = vec![10, 15, 20, 25, 30];

    println!("{}", "═".repeat(75));
    println!("TEST 1: Membrane L|seed|R efficiency");
    println!("{}", "═".repeat(75));

    let mut membrane_results: Vec<(usize, f64)> = vec![];

    println!(
        "\n{:>10} {:>10} {:>10} {:>10} {:>12}",
        "seed_len", "primes", "tests", "rate%", "efficiency"
    );
    println!("{}", "-".repeat(60));

    for &seed_len in &seed_lengths {
        let mut rng = 12345678u64 + seed_len as u64 * 999;
        let mut primes = 0;
        let mut total_digits = 0.0;

        for _ in 0..samples {
            let seed = random_seed_with_length(base, seed_len, &mut rng);
            let mem = membrane_value(base, 1, &seed, 7); // L=1, R=7 (coprime to 30)
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
            seed_len,
            primes,
            samples,
            rate * 100.0,
            efficiency
        );
    }

    println!("\n{}", "═".repeat(75));
    println!("TEST 2: Random coprime numbers (same digit counts)");
    println!("{}", "═".repeat(75));

    let mut random_results: Vec<(usize, f64)> = vec![];

    println!(
        "\n{:>10} {:>10} {:>10} {:>10} {:>12}",
        "digits", "primes", "tests", "rate%", "efficiency"
    );
    println!("{}", "-".repeat(60));

    for &(target_digits, _) in &membrane_results {
        let mut rng = 98765432u64 + target_digits as u64 * 777;
        let mut primes = 0;
        let mut total_digits = 0.0;

        for _ in 0..samples {
            let n = random_coprime_30(target_digits, &mut rng);
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
    println!("COMPARISON");
    println!("{}", "═".repeat(75));

    println!(
        "\n{:>10} {:>15} {:>15} {:>15}",
        "digits", "Membrane", "Random", "Advantage"
    );
    println!("{}", "-".repeat(60));

    for ((d1, mem_eff), (_, rand_eff)) in membrane_results.iter().zip(random_results.iter()) {
        let advantage = (mem_eff - rand_eff) / rand_eff * 100.0;
        println!(
            "{:>10} {:>15.3}× {:>15.3}× {:>+14.1}%",
            d1, mem_eff, rand_eff, advantage
        );
    }

    let mem_mean: f64 =
        membrane_results.iter().map(|(_, e)| *e).sum::<f64>() / membrane_results.len() as f64;
    let rand_mean: f64 =
        random_results.iter().map(|(_, e)| *e).sum::<f64>() / random_results.len() as f64;

    println!("\n{}", "═".repeat(75));
    println!("SUMMARY");
    println!("{}", "═".repeat(75));

    println!("\nTheoretical bound B/φ(B):     {:.3}×", theoretical);
    println!(
        "Random coprime mean:          {:.3}× ({:+.3} vs theory)",
        rand_mean,
        rand_mean - theoretical
    );
    println!(
        "Membrane mean:                {:.3}× ({:+.3} vs theory)",
        mem_mean,
        mem_mean - theoretical
    );
    println!(
        "\nMembrane advantage over random: {:+.1}%",
        (mem_mean - rand_mean) / rand_mean * 100.0
    );

    if mem_mean > rand_mean + 0.1 {
        println!(
            "\n✓ MEMBRANE STRUCTURE provides {:.3}× extra efficiency!",
            mem_mean - rand_mean
        );
    } else if (mem_mean - rand_mean).abs() < 0.1 {
        println!("\n≈ Membrane efficiency matches random coprime (within noise)");
    }
}
