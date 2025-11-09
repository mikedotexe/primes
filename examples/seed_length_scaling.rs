//! Seed Length Scaling Test: When Does Double-Membrane Emerge?
//!
//! Tests whether nested membrane structure becomes beneficial as seed length increases.
//!
//! ## Hypothesis
//!
//! As seed length increases (1 → 10 digits):
//! - Single membrane success drops (prime density ~ 1/ln(n))
//! - Nested membrane might MAINTAIN success (more constraints help)
//! - Crossover point where nested > single
//!
//! If true: Validates that structure emerges when needed (scaling hierarchy)
//! If false: Double-membrane is optional at all tested scales
//!
//! ## Run
//! ```bash
//! cargo run --example seed_length_scaling --release
//! ```

use num_bigint::BigUint;
use num_traits::{One, Zero};

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║        Seed Length Scaling: Double-Membrane Emergence         ║");
    println!("║   Testing when nested structure becomes beneficial            ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("Hypothesis:");
    println!("  As prime size increases, density drops (~ 1/ln(n))");
    println!("  Single membrane success should decline");
    println!("  Nested structure might maintain performance");
    println!();

    // Test seed lengths from 1 to 7 digits (beyond 7 gets slow)
    let seed_lengths = vec![1, 2, 3, 4, 5, 6, 7];
    let tests_per_length = 50; // Reduced for speed

    println!("═══════════════════════════════════════════════════════════════");
    println!("SCALING TEST: Base 14");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Testing {} seeds per length\n", tests_per_length);

    let mut results = Vec::new();

    for &seed_len in &seed_lengths {
        println!("─────────────────────────────────────────────────────────────");
        println!("Seed Length: {} digits", seed_len);
        println!("─────────────────────────────────────────────────────────────");

        let (single_count, single_avg_size) = test_single_at_length(seed_len, tests_per_length);
        let (nested_count, nested_avg_size) = test_nested_at_length(seed_len, tests_per_length);

        let single_pct = (single_count as f64 / tests_per_length as f64) * 100.0;
        let nested_pct = (nested_count as f64 / tests_per_length as f64) * 100.0;

        println!();
        println!("Results:");
        println!(
            "  Single membrane: {:2}/{} = {:5.1}%  (avg size: {} digits)",
            single_count, tests_per_length, single_pct, single_avg_size
        );
        println!(
            "  Nested membrane: {:2}/{} = {:5.1}%  (avg size: {} digits)",
            nested_count, tests_per_length, nested_pct, nested_avg_size
        );

        let diff = nested_count as i32 - single_count as i32;
        if diff > 0 {
            println!("  → Nested BETTER by +{} primes", diff);
        } else if diff < 0 {
            println!("  → Single BETTER by +{} primes", -diff);
        } else {
            println!("  → TIE");
        }
        println!();

        results.push((
            seed_len,
            single_pct,
            nested_pct,
            single_avg_size,
            nested_avg_size,
        ));
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("SUMMARY");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("│ Seed Len │ Single % │ Nested % │ Δ      │ Interpretation      │");
    println!("├──────────┼──────────┼──────────┼────────┼─────────────────────┤");

    let mut crossover_found = false;
    let mut crossover_len = 0;

    for (len, single, nested, _, _) in &results {
        let delta = nested - single;
        let interpretation = if delta > 5.0 {
            crossover_found = true;
            crossover_len = *len;
            "Nested wins"
        } else if delta < -5.0 {
            "Single wins"
        } else {
            "Similar"
        };

        println!(
            "│   {:2}     │  {:5.1}%  │  {:5.1}%  │ {:+6.1} │ {:19} │",
            len, single, nested, delta, interpretation
        );
    }
    println!("└──────────┴──────────┴──────────┴────────┴─────────────────────┘\n");

    // Analysis
    println!("═══════════════════════════════════════════════════════════════");
    println!("ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Check if single membrane drops
    let first_single = results[0].1;
    let last_single = results[results.len() - 1].1;
    let single_drop = first_single - last_single;

    println!("Single Membrane Trend:");
    println!("  Length 1: {:.1}%", first_single);
    println!("  Length {}: {:.1}%", results.len(), last_single);
    println!("  Drop: {:.1} percentage points", single_drop);

    if single_drop > 5.0 {
        println!("  → SUCCESS DECLINES as predicted (prime density effect)");
    } else {
        println!("  → Success remains stable (unexpected)");
    }
    println!();

    // Check if nested helps
    if crossover_found {
        println!("✓ CROSSOVER DETECTED at seed length {}", crossover_len);
        println!();
        println!("Nested structure becomes beneficial at longer seeds!");
        println!();
        println!("Interpretation:");
        println!("  1. Structure emerges WHEN NEEDED (validates hierarchy)");
        println!("  2. Simple suffices for small primes");
        println!("  3. Complexity required for large primes");
        println!("  4. Analogous to atomic shells (H simple, U complex)");
        println!();
        println!("This validates the double-membrane hypothesis at scale!");
    } else {
        // Check trend
        let first_nested = results[0].2;
        let last_nested = results[results.len() - 1].2;
        let nested_drop = first_nested - last_nested;

        println!("Nested Membrane Trend:");
        println!("  Length 1: {:.1}%", first_nested);
        println!("  Length {}: {:.1}%", results.len(), last_nested);
        println!("  Drop: {:.1} percentage points", nested_drop);
        println!();

        if nested_drop < single_drop {
            println!("~ PARTIAL SIGNAL");
            println!();
            println!(
                "Nested drops less than single ({:.1} vs {:.1} points).",
                nested_drop, single_drop
            );
            println!("This suggests nested structure is MORE STABLE under scaling.");
            println!();
            println!("Hypothesis: Benefit would emerge at even longer seeds (8-10 digits).");
        } else {
            println!("✗ NO CROSSOVER DETECTED");
            println!();
            println!("Nested structure doesn't help at tested seed lengths (1-7 digits).");
            println!();
            println!("Possible interpretations:");
            println!("  1. Benefit emerges only at much larger scales (>10 digits)");
            println!("  2. Double-membrane is theoretical but not practical");
            println!("  3. Single membrane is optimal for achievable prime sizes");
        }
    }

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("CONCLUSION");
    println!("═══════════════════════════════════════════════════════════════\n");

    if crossover_found {
        println!(
            "✓ Double-membrane validated at seed length {}",
            crossover_len
        );
        println!();
        println!("Structure SCALES as hypothesized.");
        println!("Simple → Complex transition observed empirically.");
        println!("Validates hierarchical framework predictions.");
    } else {
        println!("Current data (1-7 digit seeds):");
        println!("  Single membrane optimal or equivalent");
        println!();
        println!("Next steps:");
        println!("  1. Test longer seeds (8-12 digits) if performance allows");
        println!("  2. Try other 2p bases with more phase locks (22, 26, 34)");
        println!("  3. Focus optimization on single membrane for practical use");
    }

    println!();
}

fn test_single_at_length(seed_len: u32, count: usize) -> (usize, usize) {
    let min_seed = 10_u64.pow(seed_len - 1);
    let max_seed = 10_u64.pow(seed_len) - 1;
    let mut successes = 0;
    let mut total_digits = 0;

    for i in 0..count {
        let seed = min_seed + (i as u64 * (max_seed - min_seed) / count as u64);
        let candidate = construct_single_membrane_u64(14, 3, 11, seed as u32);

        if is_probably_prime(&candidate, 20) {
            successes += 1;
            total_digits += count_digits(&candidate);
        }
    }

    let avg_digits = if successes > 0 {
        total_digits / successes
    } else {
        0
    };
    (successes, avg_digits)
}

fn test_nested_at_length(seed_len: u32, count: usize) -> (usize, usize) {
    let min_seed = 10_u64.pow(seed_len - 1);
    let max_seed = 10_u64.pow(seed_len) - 1;
    let mut successes = 0;
    let mut total_digits = 0;

    for i in 0..count {
        let seed = min_seed + (i as u64 * (max_seed - min_seed) / count as u64);
        let candidate = construct_nested_membrane_u64(14, 1, 13, 3, 11, seed as u32);

        if is_probably_prime(&candidate, 20) {
            successes += 1;
            total_digits += count_digits(&candidate);
        }
    }

    let avg_digits = if successes > 0 {
        total_digits / successes
    } else {
        0
    };
    (successes, avg_digits)
}

fn construct_single_membrane_u64(base: u32, outer: u32, inner: u32, seed: u32) -> BigUint {
    let outer_str = format!("{}", outer);
    let inner_str = format!("{}", inner);
    let seed_str = seed.to_string();

    let membrane_str = format!(
        "{}{}{}{}{}",
        outer_str, inner_str, seed_str, inner_str, outer_str
    );

    base_string_to_biguint(&membrane_str, base)
}

fn construct_nested_membrane_u64(
    base: u32,
    outer_left: u32,
    outer_right: u32,
    inner_left: u32,
    inner_right: u32,
    seed: u32,
) -> BigUint {
    let ol = format!("{}", outer_left);
    let or = format!("{}", outer_right);
    let il = format!("{}", inner_left);
    let ir = format!("{}", inner_right);
    let seed_str = seed.to_string();

    let membrane_str = format!("{}0{}{}{}{}{}0{}", ol, il, ir, seed_str, ir, il, or);

    base_string_to_biguint(&membrane_str, base)
}

fn base_string_to_biguint(s: &str, base: u32) -> BigUint {
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    for ch in s.chars() {
        let digit_val = ch.to_digit(10).unwrap();
        result = result * &base_big + BigUint::from(digit_val);
    }

    result
}

fn count_digits(n: &BigUint) -> usize {
    n.to_string().len()
}

fn is_probably_prime(n: &BigUint, rounds: u32) -> bool {
    if n < &BigUint::from(2u32) {
        return false;
    }
    if n == &BigUint::from(2u32) || n == &BigUint::from(3u32) {
        return true;
    }
    if n.to_u32_digits().first().is_some_and(|&d| d % 2 == 0) {
        return false;
    }

    let one = BigUint::one();
    let two = BigUint::from(2u32);
    let n_minus_1 = n - &one;

    let mut d = n_minus_1.clone();
    let mut r = 0u32;
    while d
        .to_u32_digits()
        .first()
        .is_some_and(|&digit| digit % 2 == 0)
    {
        d /= &two;
        r += 1;
    }

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

fn mod_pow(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    base.modpow(exp, modulus)
}

fn random_range(min: &BigUint, max: &BigUint) -> BigUint {
    if max <= min {
        return min.clone();
    }

    let range = max - min;
    let bytes_needed = range.bits().div_ceil(8) as usize;

    let mut bytes = vec![0u8; bytes_needed];
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = ((i * 17 + 42) % 256) as u8;
    }

    let random = BigUint::from_bytes_be(&bytes);
    min + (random % &range)
}
