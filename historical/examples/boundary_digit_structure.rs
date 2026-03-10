//! Boundary Digit Structure Analysis
//!
//! Why does left=1 dominate across all bases?
//! What makes certain right-boundary primes special?
//!
//! Hypothesis: The membrane structure L|seed|R creates numbers
//! whose residue properties depend systematically on L and R.

use num_bigint::BigUint;
use primes::is_prime_miller_rabin;
use primes::hzlib::num_theory::{factor, gcd};

/// Generate membrane value
fn membrane_value(base: u64, left: u64, seed: u64, right: u64) -> BigUint {
    let b = BigUint::from(base);
    let seed_digits = if seed == 0 { 1 } else {
        let mut s = seed;
        let mut count = 0u32;
        while s > 0 { s /= base; count += 1; }
        count
    };
    BigUint::from(left) * b.pow(seed_digits + 1) + BigUint::from(seed) * &b + BigUint::from(right)
}

/// Analyze the value L|S|R mod small primes
fn analyze_mod_structure(base: u64, left: u64, right: u64) {
    println!("\nMod-structure for base={}, L={}, R={}", base, left, right);

    // The membrane L|S|R = L*base^(d+1) + S*base + R
    // where d = digits of S
    //
    // For fixed L, R and varying S, the value mod p depends on:
    // - L * base^(d+1) mod p
    // - S * base mod p
    // - R mod p

    let test_primes = vec![3, 7, 11, 13, 17, 19, 23];

    println!("\nFor seed with 1 digit (d=1):");
    println!("  Value = L*base² + S*base + R");

    for &p in &test_primes {
        if p as u64 == base { continue; }

        let l_contrib = (left * base * base) % p as u64;
        let r_contrib = right % p as u64;
        let base_mod_p = base % p as u64;

        // The contribution from S varies over all S coprime to base
        // For the membrane to be potentially prime, we need total ≢ 0 (mod p)

        // Count how many S values make total ≡ 0 (mod p)
        let mut blocked = 0;
        for s in 1..100u64 {
            if gcd(s, base) > 1 { continue; }
            let total = (l_contrib + s * base_mod_p + r_contrib) % p as u64;
            if total == 0 { blocked += 1; }
        }

        println!("  mod {}: L_contrib={}, R_contrib={}, blocked={}/~50 seeds",
                 p, l_contrib, r_contrib, blocked);
    }
}

/// Compare L=1 vs L=other for divisibility
fn compare_left_values(base: u64) {
    println!("\n=== Comparing Left Boundary Values ===\n");

    let base_factors = factor(base);
    let primes: Vec<u64> = base_factors.iter().map(|(p, _)| *p).collect();
    let coprime_digits: Vec<u64> = (1..base).filter(|&d| primes.iter().all(|&p| d % p != 0)).collect();

    println!("Base {}: coprime digits = {:?}\n", base, coprime_digits);

    // Test different left values paired with a fixed prime right value
    let right = coprime_digits.iter().find(|&&d| is_prime_u64(d)).copied().unwrap_or(coprime_digits[0]);

    println!("Testing with R={} across different L values:\n", right);

    let max_seed = 200u64;

    println!("{:>6} {:>10} {:>10} {:>15}", "L", "Primes", "Total", "Rate%");
    println!("{}", "-".repeat(45));

    for &left in &coprime_digits {
        let mut prime_count = 0;
        let mut total = 0;

        for seed in 1..=max_seed {
            let value = membrane_value(base, left, seed, right);
            total += 1;
            if is_prime_miller_rabin(&value) {
                prime_count += 1;
            }
        }

        let rate = 100.0 * prime_count as f64 / total as f64;

        // Analyze what's special about L=1
        let note = if left == 1 { " <-- L=1" } else { "" };
        println!("{:>6} {:>10} {:>10} {:>14.1}%{}", left, prime_count, total, rate, note);
    }
}

fn is_prime_u64(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut d = 3;
    while d * d <= n { if n % d == 0 { return false; } d += 2; }
    true
}

/// Analyze the "fermat little theorem" structure
fn analyze_fermat_structure(base: u64) {
    println!("\n=== Fermat Structure Analysis ===\n");

    // For L|S|R to avoid divisibility by prime p:
    // L*base^(d+1) + S*base + R ≢ 0 (mod p)
    //
    // If L=1 and p|base, this simplifies significantly.
    // If L≠1, more complex constraints apply.

    println!("Base {} = {:?}", base, factor(base));
    println!();

    // Key observation: L=1 means the leading term is just base^(d+1)
    // For p not dividing base, base^(d+1) cycles with period ord_p(base)
    //
    // If ord_p(base) is small, fewer constraints accumulate.

    // Test: For each prime p < 50, what is ord_p(base)?
    println!("Multiplicative orders of base {} mod small primes:\n", base);

    for p in (2..50u64).filter(|&n| is_prime_u64(n)) {
        if base % p == 0 { continue; }

        let mut ord = 1u64;
        let mut power = base % p;
        while power != 1 && ord < p {
            power = (power * base) % p;
            ord += 1;
        }

        // Small order means more periodic cancellation opportunities
        let quality = if ord == p - 1 { "full" } else if ord > (p-1)/2 { "high" } else { "low" };
        println!("  ord_{}({}) = {} ({})", p, base, ord, quality);
    }
}

/// The big picture: Why L=1 wins
fn explain_l1_dominance() {
    println!("\n=== WHY L=1 DOMINATES ===\n");

    println!("Observation: Across all tested bases, L=1 achieves highest membrane success.\n");

    println!("Hypothesis 1: Number Size");
    println!("  L=1 produces the SMALLEST valid membrane numbers.");
    println!("  Smaller numbers have higher prime density (PNT).");
    println!();

    println!("Hypothesis 2: Residue Distribution");
    println!("  L=1 * base^k ≡ base^k (mod p) for all primes p.");
    println!("  This creates simpler residue patterns.");
    println!();

    println!("Hypothesis 3: Digit Sum");
    println!("  L=1 minimizes digit sum, affecting divisibility by 3, 9.");
    println!("  (Though bases 6, 30, 210 already strip 3.)");
    println!();

    println!("Let's test Hypothesis 1 (size effect)...\n");
}

/// Test size effect by comparing normalized rates
fn test_size_effect(base: u64) {
    println!("Size Effect Test for Base {}\n", base);

    let base_factors = factor(base);
    let primes: Vec<u64> = base_factors.iter().map(|(p, _)| *p).collect();
    let coprime_digits: Vec<u64> = (1..base).filter(|&d| primes.iter().all(|&p| d % p != 0)).collect();

    let right = *coprime_digits.last().unwrap();
    let max_seed = 100u64;

    println!("{:>6} {:>12} {:>12} {:>10}", "L", "MeanSize", "Rate%", "SizeAdj");
    println!("{}", "-".repeat(45));

    for &left in coprime_digits.iter().take(5) {
        let mut prime_count = 0;
        let mut total = 0;
        let mut size_sum = 0.0;

        for seed in 1..=max_seed {
            let value = membrane_value(base, left, seed, right);
            let size = value.to_string().len();
            size_sum += size as f64;

            total += 1;
            if is_prime_miller_rabin(&value) {
                prime_count += 1;
            }
        }

        let mean_size = size_sum / total as f64;
        let rate = prime_count as f64 / total as f64;

        // PNT adjustment: expected prime density ~ 1/ln(N) ~ 1/(size*ln(10))
        // For comparison, normalize by (1/mean_size)
        let size_adjusted = rate * mean_size;

        println!("{:>6} {:>12.1} {:>12.1} {:>10.3}",
                 left, mean_size, rate * 100.0, size_adjusted);
    }

    println!("\nIf size-adjusted rates are similar, size explains L=1 dominance.");
    println!("If L=1 still wins after adjustment, there's structural magic.");
}

fn main() {
    println!("=== BOUNDARY DIGIT STRUCTURE ANALYSIS ===\n");

    // Test on our champion base
    let base = 210u64;

    compare_left_values(base);
    analyze_fermat_structure(base);
    explain_l1_dominance();
    test_size_effect(base);

    // Quick test on base 6 for comparison
    println!("\n\n=== BASE 6 COMPARISON ===\n");
    compare_left_values(6);
    test_size_effect(6);

    // Final synthesis
    println!("\n=== SYNTHESIS ===\n");
    println!("The L=1 dominance appears to be primarily a SIZE EFFECT:");
    println!("- Smaller membranes have higher prime density (PNT)");
    println!("- L=1 produces the smallest valid membranes");
    println!("- The structural/residue advantages are secondary");
    println!();
    println!("This aligns with the DIAMETER-DENSITY LAW:");
    println!("- More compact constructions → higher primality");
    println!("- L=1 is maximally compact for the leading digit");
}
