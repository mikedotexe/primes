// Multi-Base Coordinate Constellation Test
//
// This example tests the coordinate constellation framework across
// multiple bases to verify the φ(base) constraint is universal.
//
// RESEARCH QUESTIONS:
// 1. Does outer coordinate constraint = φ(base) for all bases?
// 2. Does linear scaling hold across different bases?
// 3. Do success rates correlate with base properties?
// 4. Are there base-specific patterns?
//
// BASES TO TEST:
// - Base  6: φ(6)  = 2  (smallest, 2×3)
// - Base 10: φ(10) = 4  (familiar, 2×5)
// - Base 18: φ(18) = 6  (2×3²)
// - Base 22: φ(22) = 10 (2×11)
// - Base 30: φ(30) = 8  (2×3×5)
//
// COMPARISON WITH BASE 14:
// - Base 14: φ(14) = 6  (our reference, 2×7)

use num_bigint::BigUint;
use num_traits::Zero;
use prime_physics_engine::is_prime;
use std::collections::{HashMap, HashSet};

/// Compute Euler's totient function φ(n)
fn totient(n: u32) -> u32 {
    let mut result = n;
    let mut n_mut = n;

    // Trial division to find prime factors
    let mut p = 2;
    while p * p <= n_mut {
        if n_mut % p == 0 {
            // Remove factor p
            while n_mut % p == 0 {
                n_mut /= p;
            }
            // Apply φ formula: φ(n) = n × ∏(1 - 1/p)
            result -= result / p;
        }
        p += 1;
    }

    // If n_mut > 1, then it's a prime factor
    if n_mut > 1 {
        result -= result / n_mut;
    }

    result
}

/// Build quintuplet: y-x-MIDDLE-x-y
fn quintuplet_membrane(middle: u32, x: u32, y: u32, base: u32) -> BigUint {
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    result = result * &base_big + BigUint::from(y);
    result = result * &base_big + BigUint::from(x);
    result = result * &base_big + BigUint::from(middle);
    result = result * &base_big + BigUint::from(x);
    result = result * &base_big + BigUint::from(y);

    result
}

/// Build septuplet: z-y-x-MIDDLE-x-y-z
fn septuplet_membrane(middle: u32, x: u32, y: u32, z: u32, base: u32) -> BigUint {
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    result = result * &base_big + BigUint::from(z);
    result = result * &base_big + BigUint::from(y);
    result = result * &base_big + BigUint::from(x);
    result = result * &base_big + BigUint::from(middle);
    result = result * &base_big + BigUint::from(x);
    result = result * &base_big + BigUint::from(y);
    result = result * &base_big + BigUint::from(z);

    result
}

/// Test quintuplets for a base, return (count, outer_coords_used)
fn test_base_quintuplets(base: u32, middle_values: &[u32], limit: u64) -> (usize, HashSet<u32>) {
    let mut total_primes = 0;
    let mut outer_coords = HashSet::new();

    for &middle in middle_values {
        for y in 1..base {
            for x in 1..base {
                let candidate = quintuplet_membrane(middle, x, y, base);

                if candidate > BigUint::from(limit) {
                    continue;
                }

                if is_prime(&candidate) {
                    total_primes += 1;
                    outer_coords.insert(y);
                }
            }
        }
    }

    (total_primes, outer_coords)
}

/// Test septuplets for a base, return (count, outer_coords_used)
fn test_base_septuplets(base: u32, middle_values: &[u32], limit: u64) -> (usize, HashSet<u32>) {
    let mut total_primes = 0;
    let mut outer_coords = HashSet::new();

    for &middle in middle_values {
        for z in 1..base {
            for y in 1..base {
                for x in 1..base {
                    let candidate = septuplet_membrane(middle, x, y, z, base);

                    if candidate > BigUint::from(limit) {
                        continue;
                    }

                    if is_prime(&candidate) {
                        total_primes += 1;
                        outer_coords.insert(z);
                    }
                }
            }
        }
    }

    (total_primes, outer_coords)
}

/// Check if a value is coprime to base
fn is_coprime(a: u32, b: u32) -> bool {
    fn gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    gcd(a, b) == 1
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║       MULTI-BASE COORDINATE CONSTELLATION TEST               ║");
    println!("║       Testing φ(base) Constraint Universality               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    // Bases to test
    let bases = vec![6u32, 10, 14, 18, 22, 30];
    let limit = 1_000_000_000_000u64;

    println!("HYPOTHESIS:");
    println!("  For any base b, the outer coordinate constraint size equals φ(b)");
    println!();

    println!("BASES TO TEST:");
    for &base in &bases {
        let phi = totient(base);
        println!("  Base {:2}: φ({:2}) = {:2}", base, base, phi);
    }
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("TESTING PROTOCOL");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("For each base:");
    println!("  1. Test k=5 quintuplets (y-x-M-x-y)");
    println!("  2. Test k=7 septuplets (z-y-x-M-x-y-z)");
    println!("  3. Record which outer coordinates (y or z) appear in primes");
    println!("  4. Verify all appearing coords are coprime to base");
    println!("  5. Check if |appearing coords| = φ(base)");
    println!();
    println!("Middle values tested: 3 coprime values per base");
    println!("Search limit: {}", limit);
    println!();

    // Results storage
    let mut results = Vec::new();

    for &base in &bases {
        println!("═══════════════════════════════════════════════════════════════");
        println!("TESTING BASE {}", base);
        println!("═══════════════════════════════════════════════════════════════");
        println!();

        let phi = totient(base);
        println!("Base {}: φ({}) = {}", base, base, phi);
        println!();

        // Choose 3 coprime middle values
        let middle_values: Vec<u32> = (1..base).filter(|&v| is_coprime(v, base)).take(3).collect();

        println!("Testing with middle values: {:?}", middle_values);
        println!();

        // Test quintuplets (k=5)
        println!("Testing k=5 quintuplets...");
        let (quint_count, quint_coords) = test_base_quintuplets(base, &middle_values, limit);

        let quint_configs = (base - 1).pow(2) * middle_values.len() as u32;
        let quint_rate = quint_count as f64 / quint_configs as f64 * 100.0;

        println!("  Primes found: {}", quint_count);
        println!("  Success rate: {:.2}%", quint_rate);
        println!("  Outer coords appearing: {} values", quint_coords.len());
        print!("  Values: {{");
        let mut sorted: Vec<_> = quint_coords.iter().collect();
        sorted.sort();
        for (i, v) in sorted.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", v);
        }
        println!("}}");

        // Check coprimality
        let all_coprime = quint_coords.iter().all(|&v| is_coprime(v, base));
        println!(
            "  All coprime to base? {}",
            if all_coprime { "✓ YES" } else { "✗ NO" }
        );
        println!(
            "  |coords| = φ(base)? {}",
            if quint_coords.len() == phi as usize {
                "✓ YES"
            } else {
                "✗ NO"
            }
        );
        println!();

        // Test septuplets (k=7)
        println!("Testing k=7 septuplets...");
        let (sept_count, sept_coords) = test_base_septuplets(base, &middle_values, limit);

        let sept_configs = (base - 1).pow(3) * middle_values.len() as u32;
        let sept_rate = sept_count as f64 / sept_configs as f64 * 100.0;

        println!("  Primes found: {}", sept_count);
        println!("  Success rate: {:.2}%", sept_rate);
        println!("  Outer coords appearing: {} values", sept_coords.len());
        print!("  Values: {{");
        let mut sorted: Vec<_> = sept_coords.iter().collect();
        sorted.sort();
        for (i, v) in sorted.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", v);
        }
        println!("}}");

        // Check coprimality
        let all_coprime = sept_coords.iter().all(|&v| is_coprime(v, base));
        println!(
            "  All coprime to base? {}",
            if all_coprime { "✓ YES" } else { "✗ NO" }
        );
        println!(
            "  |coords| = φ(base)? {}",
            if sept_coords.len() == phi as usize {
                "✓ YES"
            } else {
                "✗ NO"
            }
        );
        println!();

        // Store results
        results.push((
            base,
            phi,
            quint_count,
            quint_rate,
            quint_coords.len(),
            sept_count,
            sept_rate,
            sept_coords.len(),
        ));
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("AGGREGATE RESULTS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("┌──────┬──────┬─────────┬──────────┬────────────┬─────────┬──────────┬────────────┐");
    println!("│ Base │ φ(b) │ k=5 cnt │ k=5 rate │ k=5 coords │ k=7 cnt │ k=7 rate │ k=7 coords │");
    println!("├──────┼──────┼─────────┼──────────┼────────────┼─────────┼──────────┼────────────┤");

    for (base, phi, q_cnt, q_rate, q_coords, s_cnt, s_rate, s_coords) in &results {
        println!("│  {:2}  │  {:2}  │  {:5}  │  {:6.2}% │     {:2}     │  {:5}  │  {:6.2}% │     {:2}     │",
                 base, phi, q_cnt, q_rate, q_coords, s_cnt, s_rate, s_coords);
    }

    println!("└──────┴──────┴─────────┴──────────┴────────────┴─────────┴──────────┴────────────┘");
    println!();

    // Verify φ(base) hypothesis
    println!("═══════════════════════════════════════════════════════════════");
    println!("φ(BASE) HYPOTHESIS VERIFICATION");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    let mut k5_matches = 0;
    let mut k7_matches = 0;

    for (base, phi, _, _, q_coords, _, _, s_coords) in &results {
        let k5_match = *q_coords == *phi as usize;
        let k7_match = *s_coords == *phi as usize;

        if k5_match {
            k5_matches += 1;
        }
        if k7_match {
            k7_matches += 1;
        }

        println!("Base {:2}:", base);
        println!(
            "  k=5: |coords| = {}, φ(base) = {} → {}",
            q_coords,
            phi,
            if k5_match {
                "✓ MATCH"
            } else {
                "✗ MISMATCH"
            }
        );
        println!(
            "  k=7: |coords| = {}, φ(base) = {} → {}",
            s_coords,
            phi,
            if k7_match {
                "✓ MATCH"
            } else {
                "✗ MISMATCH"
            }
        );
        println!();
    }

    println!("SUMMARY:");
    println!(
        "  k=5 (quintuplets): {}/{} bases match φ(base)",
        k5_matches,
        results.len()
    );
    println!(
        "  k=7 (septuplets):  {}/{} bases match φ(base)",
        k7_matches,
        results.len()
    );
    println!();

    if k5_matches == results.len() && k7_matches == results.len() {
        println!("🎉 HYPOTHESIS CONFIRMED: |outer coords| = φ(base) UNIVERSALLY!");
    } else {
        println!("⚠ HYPOTHESIS PARTIALLY CONFIRMED - some bases deviate");
    }
    println!();

    // Cross-base patterns
    println!("═══════════════════════════════════════════════════════════════");
    println!("CROSS-BASE PATTERN ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("Success rate vs φ(base):");
    for (base, phi, _, q_rate, _, _, s_rate, _) in &results {
        println!(
            "  φ({:2}) = {:2}: k=5 {:.2}%, k=7 {:.2}%",
            base, phi, q_rate, s_rate
        );
    }
    println!();

    println!("Success rate vs base size:");
    for (base, _, _, q_rate, _, _, s_rate, _) in &results {
        println!("  Base {:2}: k=5 {:.2}%, k=7 {:.2}%", base, q_rate, s_rate);
    }
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("CONCLUSIONS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("This test validates whether the φ(base) constraint discovered");
    println!("for base 14 is a universal property of coordinate constellations");
    println!("or a special case.");
    println!();
    println!("If confirmed across all bases, this elevates the finding from");
    println!("'interesting pattern' to 'fundamental mathematical law'.");
    println!();
}
