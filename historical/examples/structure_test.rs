//! Quick Structure Test
//!
//! Does membrane structure L|seed|R beat random coprime numbers?
//! Use small numbers (10-20 digits) for fast primality testing.

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

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           STRUCTURE TEST: Membrane vs Random                     ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let theoretical = 30.0 / 8.0;  // B/φ(B) = 3.75
    println!("Base 30: Theoretical B/φ(B) = {:.3}×\n", theoretical);

    let samples = 2000;

    // Test at 15 decimal digits (fast primality testing)
    let target_digits = 15;

    println!("Testing at ~{} decimal digits ({} samples each)\n", target_digits, samples);

    // Test 1: Membrane L|seed|R
    println!("MEMBRANE (1|seed|7 in base 30):");
    let seed_len = 10;  // ~15 decimal digits
    let mut rng = 42424242u64;
    let mut mem_primes = 0;
    let mut mem_digits = 0.0;

    for _ in 0..samples {
        let seed = random_base30_seed(seed_len, &mut rng);
        let m = membrane(30, 1, &seed, 7);
        mem_digits += m.to_string().len() as f64;
        if is_prime_miller_rabin(&m) { mem_primes += 1; }
    }

    let mem_rate = mem_primes as f64 / samples as f64;
    let mem_mean = mem_digits / samples as f64;
    let mem_eff = mem_rate / (1.0 / (mem_mean * 2.303));
    println!("  Primes: {}/{} = {:.2}%", mem_primes, samples, mem_rate * 100.0);
    println!("  Mean digits: {:.1}", mem_mean);
    println!("  Efficiency: {:.3}× (vs theory {:.3}×)", mem_eff, theoretical);

    // Test 2: Random coprime numbers
    println!("\nRANDOM COPRIME TO 30:");
    let target = mem_mean as usize;
    let mut rng = 98765432u64;
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

    let rand_rate = rand_primes as f64 / samples as f64;
    let rand_mean = rand_digits / samples as f64;
    let rand_eff = rand_rate / (1.0 / (rand_mean * 2.303));
    println!("  Primes: {}/{} = {:.2}%", rand_primes, samples, rand_rate * 100.0);
    println!("  Mean digits: {:.1}", rand_mean);
    println!("  Efficiency: {:.3}× (vs theory {:.3}×)", rand_eff, theoretical);

    // Test 3: Truly random numbers (no coprimality filter)
    println!("\nTRULY RANDOM (no coprimality filter):");
    let mut rng = 11111111u64;
    let mut true_primes = 0;
    let mut true_digits = 0.0;

    for _ in 0..samples {
        let n = random_decimal(target, &mut rng);
        true_digits += n.to_string().len() as f64;
        if is_prime_miller_rabin(&n) { true_primes += 1; }
    }

    let true_rate = true_primes as f64 / samples as f64;
    let true_mean = true_digits / samples as f64;
    let true_eff = true_rate / (1.0 / (true_mean * 2.303));
    println!("  Primes: {}/{} = {:.2}%", true_primes, samples, true_rate * 100.0);
    println!("  Mean digits: {:.1}", true_mean);
    println!("  Efficiency: {:.3}× (PNT baseline = 1.0×)", true_eff);

    // Summary
    println!("\n{}", "═".repeat(65));
    println!("SUMMARY");
    println!("{}", "═".repeat(65));

    println!("\n{:>20} {:>15} {:>15}", "Method", "Efficiency", "vs Theory");
    println!("{}", "-".repeat(55));
    println!("{:>20} {:>15.3}× {:>+14.3}×", "Truly Random", true_eff, true_eff - 1.0);
    println!("{:>20} {:>15.3}× {:>+14.3}×", "Random Coprime", rand_eff, rand_eff - theoretical);
    println!("{:>20} {:>15.3}× {:>+14.3}×", "Membrane L|S|R", mem_eff, mem_eff - theoretical);

    let coprime_boost = rand_eff / true_eff;
    let membrane_boost = mem_eff / true_eff;
    let structure_boost = mem_eff / rand_eff;

    println!("\n{}", "═".repeat(65));
    println!("DECOMPOSITION OF MEMBRANE ADVANTAGE");
    println!("{}", "═".repeat(65));

    println!("\nTruly random → Random coprime: {:.2}× boost (coprimality filter)",
             coprime_boost);
    println!("Truly random → Membrane:       {:.2}× boost (total)", membrane_boost);
    println!("Random coprime → Membrane:     {:.2}× boost (STRUCTURE)", structure_boost);

    if structure_boost > 1.05 {
        println!("\n✓ Membrane structure adds {:.1}% extra efficiency beyond coprimality!",
                 (structure_boost - 1.0) * 100.0);
    } else if structure_boost > 0.95 {
        println!("\n≈ Membrane efficiency matches random coprime (structure adds nothing)");
    } else {
        println!("\n✗ Membrane is WORSE than random coprime (unexpected!)");
    }
}
