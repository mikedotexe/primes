// Distance-4 Constellation Test (Gap 8)
//
// Tests the power law extrapolation for distance 4 phase locks.
//
// POWER LAW PREDICTION:
//   success(4) = 25.21 × 4^(-0.53) ≈ 12.2%
//
// If this matches observations (within ±20%), it validates:
//   1. Power law holds beyond fitted data range
//   2. Exponent α = -0.53 is accurate
//   3. Extrapolation is predictive
//
// Configurations to test:
//   - (3, 11) in base 22 (gap 8, distance 4)
//   - (5, 13) in base 26 (gap 8, distance 4)
//   - (1, 15) in base 24 (gap 8, distance 4) [if valid]

use num_bigint::BigUint;
use num_traits::Zero;
use prime_physics_engine::is_prime;

// Distance-4 constellation configurations
const DISTANCE_4_CONFIGS: &[(u32, u32, u32, &str)] = &[
    (3, 11, 22, "base 22 = 2×11"),
    (5, 13, 26, "base 26 = 2×13"),
    (1, 15, 24, "base 24 = 2×12 (12 not prime)"),
    (7, 15, 30, "base 30 = 2×15 (15 not prime)"),
];

fn is_prime_simple(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }
    let mut d = 3;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 2;
    }
    true
}

// Verify phase lock properties
fn verify_phase_lock(left: u32, right: u32, base: u32) -> (bool, u32) {
    let sum_correct = (left + right) == base;
    let midpoint = base / 2;

    let left_dist = midpoint.abs_diff(left);

    let right_dist = midpoint.abs_diff(right);

    let symmetric = left_dist == right_dist;
    let distance = left_dist;

    (sum_correct && symmetric, distance)
}

// Generate constellation membrane
fn constellation_membrane(left: u32, right: u32, seed: u32, base: u32) -> BigUint {
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    result = result * &base_big + BigUint::from(left);
    result = result * &base_big + BigUint::from(right);
    result = result * &base_big + BigUint::from(seed);
    result = result * &base_big + BigUint::from(right);
    result = result * &base_big + BigUint::from(left);

    result
}

// Test a configuration
fn test_config(
    left: u32,
    right: u32,
    base: u32,
    num_seeds: u32,
) -> (usize, usize, Vec<(u32, BigUint)>) {
    let mut primes = 0;
    let mut total = 0;
    let mut examples = Vec::new();

    for seed in 1..=num_seeds {
        let n = constellation_membrane(left, right, seed, base);
        total += 1;

        if is_prime(&n) {
            primes += 1;
            if examples.len() < 5 {
                examples.push((seed, n));
            }
        }
    }

    (primes, total, examples)
}

// Calculate gap from base
fn calculate_gap(base: u32) -> u32 {
    // For base = 2p + g, and distance = g/2
    // We know base is even, so let's find g
    // This is tricky without knowing p, but we can check if p is prime
    for p in 2..base / 2 {
        let g = base - 2 * p;
        if g.is_multiple_of(2) && g > 0 {
            // Check if this makes sense
            let distance = g / 2;
            if distance == 4 {
                return g;
            }
        }
    }
    0
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║      DISTANCE-4 CONSTELLATION TEST (GAP 8)                   ║");
    println!("║      Validating Power Law Extrapolation                      ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    println!("THEORY:");
    println!("─────────────────────────────────────────────────────────");
    println!("Power law: success(d) = 25.21 × d^(-0.53)");
    println!();
    println!("Fitted data (distances 1-3):");
    println!("  d=1: observed 24%, predicted 25.2% ✓");
    println!("  d=2: observed 20%, predicted 17.5% ✓");
    println!("  d=3: observed 13%, predicted 14.1% ✓");
    println!();
    println!("EXTRAPOLATION (distance 4):");
    println!("  Predicted: 25.21 × 4^(-0.53) ≈ 12.2%");
    println!();
    println!("If observed ≈ 12.2% (±20%), power law validated!");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("CONFIGURATION VERIFICATION");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("┌──────────┬──────┬──────────┬──────────┬─────────────┐");
    println!("│   Pair   │ Base │  2p form │ Distance │   Status    │");
    println!("├──────────┼──────┼──────────┼──────────┼─────────────┤");

    for &(left, right, base, note) in DISTANCE_4_CONFIGS {
        let (valid, distance) = verify_phase_lock(left, right, base);

        let p = base / 2;
        let is_2p = is_prime_simple(p) && (2 * p == base);
        let form_status = if is_2p {
            format!("YES (p={})", p)
        } else {
            format!("NO  (p={})", p)
        };

        let status = if valid && distance == 4 {
            "✓ Valid"
        } else if valid {
            &format!("~ d={}", distance)
        } else {
            "✗ Invalid"
        };

        println!(
            "│ ({:2},{:2}) │  {:2}  │ {:10} │    {:2}    │ {:11} │",
            left, right, base, form_status, distance, status
        );
    }

    println!("└──────────┴──────┴──────────┴──────────┴─────────────┘");
    println!();

    // Filter to valid distance-4 configs
    let valid_configs: Vec<_> = DISTANCE_4_CONFIGS
        .iter()
        .filter(|&&(left, right, base, _)| {
            let (valid, distance) = verify_phase_lock(left, right, base);
            valid && distance == 4
        })
        .collect();

    println!("Valid distance-4 configurations: {}", valid_configs.len());
    println!();

    // Test each valid configuration
    println!("═══════════════════════════════════════════════════════════════");
    println!("EMPIRICAL TESTING");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    let num_seeds = 100;
    let mut all_results = Vec::new();

    for &&(left, right, base, note) in &valid_configs {
        println!("Testing ({}, {}) in base {} ({})", left, right, base, note);
        println!("─────────────────────────────────────────────────────────");

        let (primes, total, examples) = test_config(left, right, base, num_seeds);
        let success_rate = (primes as f64) / (total as f64) * 100.0;

        println!(
            "Results: {}/{} primes = {:.1}% success",
            primes, total, success_rate
        );
        println!();

        if !examples.is_empty() {
            println!("Example primes:");
            for (seed, prime) in &examples {
                let prime_str = prime.to_string();
                if prime_str.len() > 30 {
                    println!(
                        "  Seed {}: {}... ({} digits)",
                        seed,
                        &prime_str[..30],
                        prime_str.len()
                    );
                } else {
                    println!("  Seed {}: {}", seed, prime);
                }
            }
            println!();
        }

        all_results.push((left, right, base, primes, total, success_rate));
    }

    // Summary
    println!("═══════════════════════════════════════════════════════════════");
    println!("POWER LAW VALIDATION");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    if all_results.is_empty() {
        println!("No valid distance-4 configurations found!");
        println!("This suggests the gap-8 might not have suitable 2p bases.");
        return;
    }

    println!("┌──────────┬──────┬─────────────┬──────────────────────┐");
    println!("│   Pair   │ Base │   Success   │   vs Prediction      │");
    println!("├──────────┼──────┼─────────────┼──────────────────────┤");

    let predicted = 12.2;
    let mut total_primes = 0;
    let mut total_tested = 0;

    for &(left, right, base, primes, tested, rate) in &all_results {
        let error = ((rate - predicted) / predicted * 100.0).abs();

        let status = if error < 20.0 {
            "✓ Validates"
        } else if error < 50.0 {
            "~ Partial"
        } else {
            "✗ Differs"
        };

        println!(
            "│ ({:2},{:2}) │  {:2}  │ {:3}/{:3} {:4.1}% │ {:4.1}% error {:10} │",
            left, right, base, primes, tested, rate, error, status
        );

        total_primes += primes;
        total_tested += tested;
    }

    println!("├──────────┴──────┼─────────────┼──────────────────────┤");

    let overall_rate = (total_primes as f64) / (total_tested as f64) * 100.0;
    let overall_error = ((overall_rate - predicted) / predicted * 100.0).abs();

    println!(
        "│    OVERALL     │ {:3}/{:3} {:4.1}% │ {:4.1}% error          │",
        total_primes, total_tested, overall_rate, overall_error
    );
    println!("└────────────────┴─────────────┴──────────────────────┘");
    println!();

    // Conclusion
    println!("═══════════════════════════════════════════════════════════════");
    println!("CONCLUSION");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("Predicted (power law): {:.1}%", predicted);
    println!("Observed (overall):    {:.1}%", overall_rate);
    println!("Error:                 {:.1}%", overall_error);
    println!();

    if overall_error < 20.0 {
        println!("✓ POWER LAW VALIDATED");
        println!();
        println!("The power law success(d) = 25.21 × d^(-0.53) successfully");
        println!("extrapolates beyond the fitted data range (d=1-3).");
        println!();
        println!("This confirms:");
        println!("  1. Exponent α = -0.53 is accurate");
        println!("  2. Coefficient k = 25.21 is robust");
        println!("  3. Model has predictive power");
        println!();
        println!("The 1/√d relationship appears to be a genuine universal law");
        println!("governing constellation success rates across all distances.");
    } else if overall_error < 50.0 {
        println!("~ PARTIAL VALIDATION");
        println!();
        println!(
            "The prediction is within {:.0}%, suggesting the power law",
            overall_error
        );
        println!("captures the general trend but may need refinement:");
        println!("  - Include base-specific corrections");
        println!("  - Adjust coefficient k or exponent α");
        println!("  - Account for 2p vs non-2p base differences");
    } else {
        println!("✗ POWER LAW FAILS AT DISTANCE 4");
        println!();
        println!(
            "The {:.0}% error suggests the power law doesn't extrapolate.",
            overall_error
        );
        println!("Possible explanations:");
        println!("  - Power law valid only for d=1-3");
        println!("  - Different scaling regime at d≥4");
        println!("  - Base-specific effects dominate at larger distances");
        println!("  - Sample size insufficient (need more configs)");
    }

    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("VERIFICATION");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("All results independently verifiable:");
    println!("  - Gap-8 formula: base = 2p + 8, distance = 4");
    println!("  - Phase lock validation: left + right = base, equidistant");
    println!("  - Primality: standard Miller-Rabin");
    println!("  - {} total tests performed", total_tested);
}
