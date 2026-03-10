//! Nested Membrane Test: Double-Membrane Validation
//!
//! Tests whether using multiple phase locks in a nested structure
//! improves membrane prime generation success rates.
//!
//! ## Critical Test for Double-Membrane Hypothesis
//!
//! Base 14 = 2×7 has two phase locks:
//! - Inner: (3, 11) at distance 4 from midpoint 7
//! - Outer: (1, 13) at distance 6 from midpoint 7
//!
//! Single membrane: 3 + zeros + 11 + zeros + SEED + ...
//! Nested membrane: 1 + zeros + 3 + zeros + 11 + zeros + 13 + zeros + SEED + ...
//!
//! Hypothesis: Nested structure achieves higher success rate due to
//! multiple symmetric constraints.
//!
//! ## Run
//! ```bash
//! cargo run --example test_nested_membrane --release
//! ```

use num_bigint::BigUint;
use num_traits::{One, Zero};

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║          Nested Membrane Validation Test                      ║");
    println!("║   Testing Double-Membrane Hypothesis on Base 14               ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("Base 14 = 2×7 Phase Locks:");
    println!("  Inner: (3, 11) at distance 4 from midpoint 7");
    println!("  Outer: (1, 13) at distance 6 from midpoint 7");
    println!();

    // Test configurations
    let test_seeds = 100;
    let padding = (0, 0); // k=(0,0) for minimal padding

    println!("═══════════════════════════════════════════════════════════════");
    println!("TEST 1: Single Membrane (Baseline)");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Configuration: outer=3, inner=11, k=(0,0)");
    println!("Structure: 3 + 11 + SEED + 11 + 3\n");

    let single_success = test_single_membrane(14, 3, 11, padding, test_seeds);

    println!("Results:");
    println!("  Seeds tested: {}", test_seeds);
    println!("  Primes found: {}", single_success);
    println!(
        "  Success rate: {:.1}%",
        (single_success as f64 / test_seeds as f64) * 100.0
    );
    println!("  (Expected: ~27% based on previous data)");

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("TEST 2: Nested Membrane (Double Structure)");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Configuration:");
    println!("  Outer: (1, 13) at distance 6");
    println!("  Inner: (3, 11) at distance 4");
    println!("Structure: 1 + 0 + 3 + 11 + SEED + 11 + 3 + 0 + 13");
    println!("           └outer┘ └inner┘ └inner┘ └outer┘\n");

    let nested_success = test_nested_membrane(14, 1, 13, 3, 11, test_seeds);

    println!("Results:");
    println!("  Seeds tested: {}", test_seeds);
    println!("  Primes found: {}", nested_success);
    println!(
        "  Success rate: {:.1}%",
        (nested_success as f64 / test_seeds as f64) * 100.0
    );

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("COMPARISON");
    println!("═══════════════════════════════════════════════════════════════\n");

    let improvement = nested_success as f64 / single_success.max(1) as f64;
    let diff = nested_success as i32 - single_success as i32;

    println!("│ Configuration │ Success Rate │ Primes │ Difference │");
    println!("├───────────────┼──────────────┼────────┼────────────┤");
    println!(
        "│ Single (3,11) │    {:.1}%     │   {:2}   │     --     │",
        (single_success as f64 / test_seeds as f64) * 100.0,
        single_success
    );
    println!(
        "│ Nested layers │    {:.1}%     │   {:2}   │   {:+3}     │",
        (nested_success as f64 / test_seeds as f64) * 100.0,
        nested_success,
        diff
    );
    println!("└───────────────┴──────────────┴────────┴────────────┘\n");

    println!("Improvement factor: {:.2}x", improvement);
    println!();

    // Interpretation
    if diff > 5 {
        println!("✓ DOUBLE-MEMBRANE VALIDATED");
        println!();
        println!("Nested structure shows >5% improvement over single membrane.");
        println!("This confirms that multiple phase lock layers provide");
        println!("additional primality constraints, validating the double-membrane");
        println!("hypothesis.");
        println!();
        println!("Implications:");
        println!("  1. Structure scales: more layers → better filtering");
        println!("  2. Validates hierarchical framework at structural level");
        println!("  3. Path to arbitrarily large primes via nesting");
        println!("  4. Membranes are like atomic orbitals (s, p, d shells)");
    } else if diff >= 0 {
        println!("~ MARGINAL SIGNAL");
        println!();
        println!(
            "Nested structure shows modest improvement ({:+} primes).",
            diff
        );
        println!("Signal is positive but not decisive. Possible interpretations:");
        println!("  1. Double-membrane works but effect is subtle");
        println!("  2. Need larger sample size (test 1000+ seeds)");
        println!("  3. Benefit emerges primarily at longer seed lengths");
    } else {
        println!("✗ DOUBLE-MEMBRANE NOT VALIDATED");
        println!();
        println!(
            "Nested structure underperforms single membrane by {} primes.",
            -diff
        );
        println!("This suggests:");
        println!("  1. Additional constraints are too restrictive");
        println!("  2. Double-membrane is optional, not beneficial at current scale");
        println!("  3. Single membrane is optimal for small seeds");
    }

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("NEXT STEPS");
    println!("═══════════════════════════════════════════════════════════════\n");

    if diff > 5 {
        println!("✓ Test larger seed lengths (5-10 digits)");
        println!("✓ Test other 2p bases with multiple locks (22, 26, 34)");
        println!("✓ Formalize nested structure in Agda");
        println!("✓ Derive nested singular series formula");
    } else if diff >= 0 {
        println!("→ Increase sample size to 1000 seeds");
        println!("→ Test seed length scaling (does benefit emerge at scale?)");
        println!("→ Try triple-nested if base has 3+ locks");
    } else {
        println!("→ Focus on single membrane optimization");
        println!("→ Test bases 22, 26 to validate phase lock density model");
        println!("→ Double-membrane may be interesting but not necessary");
    }

    println!();
}

/// Test single membrane configuration
fn test_single_membrane(
    base: u32,
    outer: u32,
    inner: u32,
    padding: (u32, u32),
    test_seeds: usize,
) -> usize {
    let mut successes = 0;

    for seed in 1..=test_seeds {
        let candidate = construct_single_membrane(base, outer, inner, padding, seed as u32);
        if is_probably_prime(&candidate, 20) {
            successes += 1;
            if successes <= 5 {
                println!("  Prime found: {} (seed {})", candidate, seed);
            }
        }
    }

    if successes > 5 {
        println!("  ... ({} more primes)", successes - 5);
    }
    println!();

    successes
}

/// Test nested membrane configuration
fn test_nested_membrane(
    base: u32,
    outer_left: u32,
    outer_right: u32,
    inner_left: u32,
    inner_right: u32,
    test_seeds: usize,
) -> usize {
    let mut successes = 0;

    for seed in 1..=test_seeds {
        let candidate = construct_nested_membrane(
            base,
            outer_left,
            outer_right,
            inner_left,
            inner_right,
            seed as u32,
        );

        if is_probably_prime(&candidate, 20) {
            successes += 1;
            if successes <= 5 {
                println!("  Prime found: {} (seed {})", candidate, seed);
            }
        }
    }

    if successes > 5 {
        println!("  ... ({} more primes)", successes - 5);
    }
    println!();

    successes
}

/// Construct single membrane: outer + inner + seed + inner + outer
fn construct_single_membrane(
    base: u32,
    outer: u32,
    inner: u32,
    _padding: (u32, u32),
    seed: u32,
) -> BigUint {
    // Convert to base representation
    let outer_str = digit_to_base_char(outer, base);
    let inner_str = digit_to_base_char(inner, base);
    let seed_str = seed.to_string();

    // Structure: outer + inner + seed + inner + outer
    let membrane_str = format!(
        "{}{}{}{}{}",
        outer_str, inner_str, seed_str, inner_str, outer_str
    );

    // Convert from base to decimal
    base_string_to_biguint(&membrane_str, base)
}

/// Construct nested membrane: outer_l + 0 + inner_l + inner_r + seed + inner_r + inner_l + 0 + outer_r
fn construct_nested_membrane(
    base: u32,
    outer_left: u32,
    outer_right: u32,
    inner_left: u32,
    inner_right: u32,
    seed: u32,
) -> BigUint {
    let ol = digit_to_base_char(outer_left, base);
    let or = digit_to_base_char(outer_right, base);
    let il = digit_to_base_char(inner_left, base);
    let ir = digit_to_base_char(inner_right, base);
    let seed_str = seed.to_string();

    // Nested structure: outer + buffer + inner + seed + inner + buffer + outer
    let membrane_str = format!(
        "{}0{}{}{}{}{}0{}",
        ol,       // Outer left
        il,       // Inner left
        ir,       // Inner right
        seed_str, // Seed
        ir,       // Inner right
        il,       // Inner left
        or
    ); // Outer right

    base_string_to_biguint(&membrane_str, base)
}

/// Convert single digit to base character representation
fn digit_to_base_char(digit: u32, _base: u32) -> String {
    if digit < 10 {
        format!("{}", digit)
    } else {
        // For bases > 10, use letters A=10, B=11, etc.
        format!("{}", (b'A' + (digit - 10) as u8) as char)
    }
}

/// Convert base-N string to BigUint
fn base_string_to_biguint(s: &str, base: u32) -> BigUint {
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    for ch in s.chars() {
        let digit_val = if ch.is_ascii_digit() {
            ch as u32 - '0' as u32
        } else if ch.is_ascii_uppercase() {
            10 + (ch as u32 - 'A' as u32)
        } else {
            panic!("Invalid character in base string: {}", ch);
        };

        result = result * &base_big + BigUint::from(digit_val);
    }

    result
}

/// Miller-Rabin primality test
fn is_probably_prime(n: &BigUint, rounds: u32) -> bool {
    if n < &BigUint::from(2u32) {
        return false;
    }
    if n == &BigUint::from(2u32) || n == &BigUint::from(3u32) {
        return true;
    }
    if n.to_u32_digits().first().map_or(false, |&d| d % 2 == 0) {
        return false;
    }

    // Write n-1 as 2^r * d
    let one = BigUint::one();
    let two = BigUint::from(2u32);
    let n_minus_1 = n - &one;

    let mut d = n_minus_1.clone();
    let mut r = 0u32;
    while d
        .to_u32_digits()
        .first()
        .map_or(false, |&digit| digit % 2 == 0)
    {
        d = d / &two;
        r += 1;
    }

    // Witness loop
    'witness: for _ in 0..rounds {
        let a = random_range(&two, &(n - &two));
        let mut x = mod_pow(&a, &d, n);

        if x == one || x == n_minus_1 {
            continue 'witness;
        }

        for _ in 0..(r - 1) {
            x = mod_pow(&x, &two, n);
            if x == n_minus_1 {
                continue 'witness;
            }
        }

        return false;
    }

    true
}

/// Modular exponentiation
fn mod_pow(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    base.modpow(exp, modulus)
}

/// Generate random BigUint in range [min, max]
fn random_range(min: &BigUint, max: &BigUint) -> BigUint {
    if max <= min {
        return min.clone();
    }

    let range = max - min;
    let bytes_needed = ((range.bits() + 7) / 8) as usize;

    // Simple deterministic "random" for reproducibility
    // In production, use proper RNG
    let mut bytes = vec![0u8; bytes_needed];
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = ((i * 17 + 42) % 256) as u8;
    }

    let random = BigUint::from_bytes_be(&bytes);
    min + (random % &range)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_membrane_construction() {
        // Base 6, (1,5), seed 4
        let result = construct_single_membrane(6, 1, 5, (0, 0), 4);

        // Should be 15451 in base 6 = 1×6⁴ + 5×6³ + 4×6² + 5×6 + 1
        // = 1296 + 1080 + 144 + 30 + 1 = 2551
        assert_eq!(result, BigUint::from(2551u32));
    }

    #[test]
    fn test_base_conversion() {
        let s = "15451";
        let result = base_string_to_biguint(s, 6);
        assert_eq!(result, BigUint::from(2551u32));
    }

    #[test]
    fn test_primality() {
        assert!(is_probably_prime(&BigUint::from(2u32), 20));
        assert!(is_probably_prime(&BigUint::from(17u32), 20));
        assert!(!is_probably_prime(&BigUint::from(4u32), 20));
        assert!(!is_probably_prime(&BigUint::from(15u32), 20));
    }
}
