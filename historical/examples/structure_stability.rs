//! Structure Stability Test
//!
//! Run multiple trials to verify membrane vs random coprime comparison

use num_bigint::BigUint;
use primes::is_prime_miller_rabin;

fn lcg_next(rng: &mut u64) -> u64 {
    *rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
    *rng
}

fn random_base30_seed(len: usize, rng: &mut u64) -> BigUint {
    let b = BigUint::from(30u64);
    let first = (lcg_next(rng) % 29) + 1;
    let mut seed = BigUint::from(first);
    for _ in 1..len { seed = seed * &b + BigUint::from(lcg_next(rng) % 30); }
    seed
}

fn membrane(base: u64, left: u64, seed: &BigUint, right: u64) -> BigUint {
    let b = BigUint::from(base);
    let mut sd = 0u32;
    let mut t = seed.clone();
    while t > BigUint::ZERO { t /= &b; sd += 1; }
    if sd == 0 { sd = 1; }
    BigUint::from(left) * b.pow(sd + 1) + seed * &b + BigUint::from(right)
}

fn random_decimal(digits: usize, rng: &mut u64) -> BigUint {
    let first = (lcg_next(rng) % 9) + 1;
    let mut n = BigUint::from(first);
    for _ in 1..digits { n = n * 10u64 + (lcg_next(rng) % 10); }
    n
}

fn is_coprime_30(n: &BigUint) -> bool {
    n % 2u64 != BigUint::ZERO && n % 3u64 != BigUint::ZERO && n % 5u64 != BigUint::ZERO
}

fn run_trial(trial: usize, samples: usize) -> (f64, f64, f64) {
    let seed_len = 10;
    let target = 17;

    // Membrane
    let mut rng = 42424242u64 + trial as u64 * 12345;
    let mut mem_primes = 0;
    let mut mem_digits = 0.0;
    for _ in 0..samples {
        let seed = random_base30_seed(seed_len, &mut rng);
        let m = membrane(30, 1, &seed, 7);
        mem_digits += m.to_string().len() as f64;
        if is_prime_miller_rabin(&m) { mem_primes += 1; }
    }
    let mem_eff = (mem_primes as f64 / samples as f64) / (1.0 / (mem_digits / samples as f64 * 2.303));

    // Random coprime
    let mut rng = 98765432u64 + trial as u64 * 54321;
    let mut rand_primes = 0;
    let mut rand_digits = 0.0;
    let mut attempts = 0;
    while attempts < samples {
        let n = random_decimal(target, &mut rng);
        if !is_coprime_30(&n) { continue; }
        rand_digits += n.to_string().len() as f64;
        if is_prime_miller_rabin(&n) { rand_primes += 1; }
        attempts += 1;
    }
    let rand_eff = (rand_primes as f64 / samples as f64) / (1.0 / (rand_digits / samples as f64 * 2.303));

    // Structure boost
    let boost = mem_eff / rand_eff;

    (mem_eff, rand_eff, boost)
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           STRUCTURE STABILITY TEST                               ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let trials = 10;
    let samples = 2000;

    println!("Running {} trials × {} samples each...\n", trials, samples);

    println!("{:>8} {:>15} {:>15} {:>15}",
             "Trial", "Membrane", "Random Cop", "Structure×");
    println!("{}", "-".repeat(55));

    let mut boosts = vec![];

    for trial in 0..trials {
        let (mem_eff, rand_eff, boost) = run_trial(trial, samples);
        boosts.push(boost);
        println!("{:>8} {:>15.3}× {:>15.3}× {:>15.3}×",
                 trial + 1, mem_eff, rand_eff, boost);
    }

    let mean_boost: f64 = boosts.iter().sum::<f64>() / trials as f64;
    let std_boost: f64 = (boosts.iter().map(|b| (b - mean_boost).powi(2)).sum::<f64>()
                         / (trials - 1) as f64).sqrt();

    println!("{}", "-".repeat(55));
    println!("{:>8} {:>15} {:>15} {:>15.3}× ± {:.3}",
             "MEAN", "", "", mean_boost, std_boost);

    println!("\n{}", "═".repeat(65));
    println!("CONCLUSION");
    println!("{}", "═".repeat(65));

    let t_stat = (mean_boost - 1.0) / (std_boost / (trials as f64).sqrt());
    let significant = t_stat.abs() > 2.26;  // t critical for df=9, α=0.05

    println!("\nMean structure boost: {:.3}× ± {:.3}", mean_boost, std_boost);
    println!("t-statistic vs 1.0: {:.2}", t_stat);
    println!("Significant (α=0.05)? {}", if significant { "YES" } else { "NO" });

    if !significant {
        println!("\n✓ MEMBRANE STRUCTURE ADDS NO SIGNIFICANT EFFICIENCY");
        println!("  The entire advantage comes from COPRIMALITY to the base.");
        println!("  L|seed|R is just a convenient way to ensure gcd(n, base) = 1.");
    } else if mean_boost > 1.0 {
        println!("\n✓ Membrane structure adds {:.1}% efficiency (statistically significant)",
                 (mean_boost - 1.0) * 100.0);
    } else {
        println!("\n✗ Membrane is slightly WORSE than random coprime");
    }
}
