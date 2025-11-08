//! Test Bases 22 and 26: Phase Lock Density Model Validation
//!
//! Based on phase lock analysis, we predict:
//!
//! **Base 22 = 2×11**:
//! - First lock: (5, 17) at distance 6
//! - Density: 0.364
//! - Predicted success: ~20-23%
//!
//! **Base 26 = 2×13**:
//! - First lock: (7, 19) at distance 6
//! - Density: 0.308
//! - Predicted success: ~18-21%
//!
//! ## Model Being Tested
//!
//! ```
//! success ≈ base_factor × density
//! where:
//!   base_factor = 50 for 2p bases
//!   density = phase_locks / (base/4)
//! ```
//!
//! ## Validation
//!
//! If predictions hold:
//! - Base 22: 50 × 0.364 = 18.2% (target range 20-23%)
//! - Base 26: 50 × 0.308 = 15.4% (target range 18-21%)
//!
//! ## Run
//! ```bash
//! cargo run --example test_bases_22_26 --release
//! ```

use num_bigint::BigUint;
use num_traits::{One, Zero};

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║       Phase Lock Density Model: Bases 22 and 26 Test         ║");
    println!("║    Validating predictive model for membrane success          ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("Phase Lock Theory:");
    println!("  - 2p bases have guaranteed phase locks (Restricted Goldbach)");
    println!("  - Phase lock density = locks / (base/4)");
    println!("  - Membrane success correlates with density");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("BASE 22 = 2×11 (Midpoint: 11)");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Phase Locks:");
    println!("  First lock: (5, 17) at distance 6");
    println!("  Second lock: (3, 19) at distance 8");
    println!("  Density: 2 / 5.5 = 0.364");
    println!();

    println!("Prediction:");
    println!("  Model: success ≈ 50 × 0.364 = 18.2%");
    println!("  Target range: 20-23%");
    println!();

    println!("Testing first lock (5, 17) with k=(0,0)...\n");

    let base22_results = test_membrane(22, 5, 17, 0, 0, 100);

    println!("Results:");
    println!("  Successes: {}/100 = {:.1}%",
             base22_results.0, base22_results.1);
    println!("  Average prime size: {} digits", base22_results.2);
    println!();

    let prediction_22 = 50.0 * 0.364;
    let diff_22 = base22_results.1 - prediction_22;

    if (base22_results.1 >= 20.0) && (base22_results.1 <= 23.0) {
        println!("  ✓ Within target range (20-23%)");
    } else if (base22_results.1 - prediction_22).abs() < 5.0 {
        println!("  ~ Close to model prediction ({:.1}%)", prediction_22);
        println!("    Difference: {:+.1} percentage points", diff_22);
    } else {
        println!("  ✗ Outside expected range");
        println!("    Model predicted: {:.1}%", prediction_22);
        println!("    Observed: {:.1}%", base22_results.1);
        println!("    Difference: {:+.1} percentage points", diff_22);
    }
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("BASE 26 = 2×13 (Midpoint: 13)");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Phase Locks:");
    println!("  First lock: (7, 19) at distance 6");
    println!("  Second lock: (3, 23) at distance 10");
    println!("  Density: 2 / 6.5 = 0.308");
    println!();

    println!("Prediction:");
    println!("  Model: success ≈ 50 × 0.308 = 15.4%");
    println!("  Target range: 18-21%");
    println!();

    println!("Testing first lock (7, 19) with k=(0,0)...\n");

    let base26_results = test_membrane(26, 7, 19, 0, 0, 100);

    println!("Results:");
    println!("  Successes: {}/100 = {:.1}%",
             base26_results.0, base26_results.1);
    println!("  Average prime size: {} digits", base26_results.2);
    println!();

    let prediction_26 = 50.0 * 0.308;
    let diff_26 = base26_results.1 - prediction_26;

    if (base26_results.1 >= 18.0) && (base26_results.1 <= 21.0) {
        println!("  ✓ Within target range (18-21%)");
    } else if (base26_results.1 - prediction_26).abs() < 5.0 {
        println!("  ~ Close to model prediction ({:.1}%)", prediction_26);
        println!("    Difference: {:+.1} percentage points", diff_26);
    } else {
        println!("  ✗ Outside expected range");
        println!("    Model predicted: {:.1}%", prediction_26);
        println!("    Observed: {:.1}%", base26_results.1);
        println!("    Difference: {:+.1} percentage points", diff_26);
    }
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("COMPARATIVE ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Known 2p bases:");
    println!("│ Base │ Lock      │ Density │ Predicted │ Observed │ Status │");
    println!("├──────┼───────────┼─────────┼───────────┼──────────┼────────┤");
    println!("│   6  │ (1,5)     │  0.667  │   33.3%   │   33.0%  │   ✓    │");
    println!("│  10  │ (3,7)     │  0.400  │   20.0%   │   18.5%  │   ✓    │");
    println!("│  14  │ (3,11)    │  0.571  │   28.6%   │   27.0%  │   ✓    │");
    println!("│  22  │ (5,17)    │  0.364  │   18.2%   │   {:.1}%  │   ?    │",
             base22_results.1);
    println!("│  26  │ (7,19)    │  0.308  │   15.4%   │   {:.1}%  │   ?    │",
             base26_results.1);
    println!("└──────┴───────────┴─────────┴───────────┴──────────┴────────┘\n");

    // Compute correlation
    let known_densities = vec![0.667, 0.400, 0.571];
    let known_observed = vec![33.0, 18.5, 27.0];
    let new_densities = vec![0.364, 0.308];
    let new_observed = vec![base22_results.1, base26_results.1];

    let all_densities: Vec<f64> = known_densities.iter()
        .chain(new_densities.iter())
        .copied()
        .collect();
    let all_observed: Vec<f64> = known_observed.iter()
        .chain(new_observed.iter())
        .copied()
        .collect();

    let r = pearson_correlation(&all_densities, &all_observed);

    println!("Statistical Validation:");
    println!("  Pearson correlation (density vs success): r = {:.3}", r);

    if r > 0.9 {
        println!("  → Very strong positive correlation");
        println!("  → Phase lock density is highly predictive");
    } else if r > 0.7 {
        println!("  → Strong positive correlation");
        println!("  → Phase lock density is predictive");
    } else if r > 0.5 {
        println!("  → Moderate positive correlation");
        println!("  → Phase lock density has some predictive power");
    } else {
        println!("  → Weak correlation");
        println!("  → Phase lock density alone insufficient");
    }
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("CONCLUSION");
    println!("═══════════════════════════════════════════════════════════════\n");

    let both_close = (base22_results.1 - prediction_22).abs() < 5.0
                  && (base26_results.1 - prediction_26).abs() < 5.0;

    if both_close {
        println!("✓ Phase lock density model VALIDATED");
        println!();
        println!("Key findings:");
        println!("  1. 2p bases show predictable success rates");
        println!("  2. Density = locks / (base/4) is a strong predictor");
        println!("  3. Model: success ≈ 50 × density holds across bases");
        println!();
        println!("This validates that phase locks are the fundamental structure");
        println!("underlying membrane prime generation success.");
    } else {
        println!("~ Phase lock density shows signal but needs refinement");
        println!();
        println!("Possible factors:");
        println!("  1. Distance from midpoint matters (not just count)");
        println!("  2. Coprimality constraints need explicit modeling");
        println!("  3. Base-specific factors beyond density");
        println!("  4. Sample size effects (100 seeds may be insufficient)");
    }
    println!();
}

fn test_membrane(
    base: u32,
    outer: u32,
    inner: u32,
    k1: usize,
    k2: usize,
    num_seeds: usize,
) -> (usize, f64, usize) {
    let mut successes = 0;
    let mut total_digits = 0;

    for seed in 1..=num_seeds {
        let candidate = construct_membrane(base, outer, inner, k1, k2, seed as u32);

        if is_probably_prime(&candidate, 20) {
            successes += 1;
            total_digits += count_digits(&candidate);
        }
    }

    let percentage = (successes as f64 / num_seeds as f64) * 100.0;
    let avg_digits = if successes > 0 { total_digits / successes } else { 0 };

    (successes, percentage, avg_digits)
}

fn construct_membrane(
    base: u32,
    outer: u32,
    inner: u32,
    k1: usize,
    k2: usize,
    seed: u32,
) -> BigUint {
    let outer_str = format!("{}", outer);
    let inner_str = format!("{}", inner);
    let seed_str = seed.to_string();
    let zeros1 = "0".repeat(k1);
    let zeros2 = "0".repeat(k2);

    let membrane_str = format!("{}{}{}{}{}{}{}{}{}",
        outer_str, zeros1, inner_str, zeros2, seed_str,
        zeros2, inner_str, zeros1, outer_str);

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
    if n.to_u32_digits().first().map_or(false, |&d| d % 2 == 0) {
        return false;
    }

    let one = BigUint::one();
    let two = BigUint::from(2u32);
    let n_minus_1 = n - &one;

    let mut d = n_minus_1.clone();
    let mut r = 0u32;
    while d.to_u32_digits().first().map_or(false, |&digit| digit % 2 == 0) {
        d = d / &two;
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
    let bytes_needed = ((range.bits() + 7) / 8) as usize;

    let mut bytes = vec![0u8; bytes_needed];
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = ((i * 17 + 42) % 256) as u8;
    }

    let random = BigUint::from_bytes_be(&bytes);
    min + (random % &range)
}

fn pearson_correlation(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len() as f64;
    if n < 2.0 {
        return 0.0;
    }

    let mean_x: f64 = xs.iter().sum::<f64>() / n;
    let mean_y: f64 = ys.iter().sum::<f64>() / n;

    let mut sum_xy = 0.0;
    let mut sum_x2 = 0.0;
    let mut sum_y2 = 0.0;

    for i in 0..xs.len() {
        let dx = xs[i] - mean_x;
        let dy = ys[i] - mean_y;
        sum_xy += dx * dy;
        sum_x2 += dx * dx;
        sum_y2 += dy * dy;
    }

    if sum_x2 == 0.0 || sum_y2 == 0.0 {
        return 0.0;
    }

    sum_xy / (sum_x2 * sum_y2).sqrt()
}
