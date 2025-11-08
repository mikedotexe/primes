// Seed Length Crossover Validation
//
// Tests the golden ratio emergence law: crossover_length = φ × density × √base
//
// For multiple bases (6, 10, 14, 22), we test single-membrane vs double-membrane
// configurations across seed lengths 1-7 to find where double-membrane becomes optimal.
//
// Predictions:
// - Base 6:  crossover ≈ 2.6 (density 0.667, √6 ≈ 2.45, φ ≈ 1.618)
// - Base 10: crossover ≈ 2.0 (density 0.400, √10 ≈ 3.16, φ ≈ 1.618)
// - Base 14: crossover ≈ 3.5 (density 0.571, √14 ≈ 3.74, φ ≈ 1.618) [OBSERVED: 4]
// - Base 22: crossover ≈ 2.8 (density 0.364, √22 ≈ 4.69, φ ≈ 1.618)

use num_bigint::BigUint;
use num_traits::Zero;
use prime_physics_engine::is_prime;
use std::collections::HashMap;

const PHI: f64 = 1.618033988749;

// Phase lock counts for each base (from phase lock density model)
fn get_phase_locks(base: u32) -> u32 {
    match base {
        6 => 1,   // (1,5)
        10 => 1,  // (3,7)
        14 => 2,  // (3,11), (1,13)
        22 => 2,  // (3,19), (9,13)
        _ => 0,
    }
}

// Calculate phase lock density
fn phase_lock_density(base: u32) -> f64 {
    let locks = get_phase_locks(base) as f64;
    let denom = (base / 4) as f64;
    locks / denom
}

// Predict crossover using φ × density × √base
fn predict_crossover(base: u32) -> f64 {
    let density = phase_lock_density(base);
    let sqrt_base = (base as f64).sqrt();
    PHI * density * sqrt_base
}

// Get optimal single-membrane config for each base
fn get_single_config(base: u32) -> (u32, u32) {
    match base {
        6 => (1, 5),
        10 => (3, 7),
        14 => (3, 11),
        22 => (3, 19),
        _ => (1, base - 1),
    }
}

// Get optimal double-membrane config (nested) for each base
fn get_double_config(base: u32) -> ((u32, u32), (u32, u32)) {
    match base {
        6 => ((1, 5), (1, 5)),
        10 => ((3, 7), (3, 7)),
        14 => ((3, 11), (3, 11)),
        22 => ((3, 19), (9, 13)),
        _ => {
            let single = get_single_config(base);
            (single, single)
        }
    }
}

// Generate single membrane number
fn single_membrane(outer: u32, inner: u32, seed: u32, base: u32) -> BigUint {
    // outer-inner-seed-inner-outer
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    // outer
    result = result * &base_big + BigUint::from(outer);
    // inner
    result = result * &base_big + BigUint::from(inner);
    // seed
    result = result * &base_big + BigUint::from(seed);
    // inner
    result = result * &base_big + BigUint::from(inner);
    // outer
    result = result * &base_big + BigUint::from(outer);

    result
}

// Generate double membrane number (nested)
fn double_membrane(
    outer: (u32, u32),
    inner: (u32, u32),
    seed: u32,
    base: u32,
) -> BigUint {
    // outer.0-outer.1-inner.0-inner.1-seed-inner.1-inner.0-outer.1-outer.0
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    result = result * &base_big + BigUint::from(outer.0);
    result = result * &base_big + BigUint::from(outer.1);
    result = result * &base_big + BigUint::from(inner.0);
    result = result * &base_big + BigUint::from(inner.1);
    result = result * &base_big + BigUint::from(seed);
    result = result * &base_big + BigUint::from(inner.1);
    result = result * &base_big + BigUint::from(inner.0);
    result = result * &base_big + BigUint::from(outer.1);
    result = result * &base_big + BigUint::from(outer.0);

    result
}

// Generate seed of given length in base
fn generate_seed(length: usize, index: u32, base: u32) -> u32 {
    if length == 1 {
        // Single digit: 1 to base-1
        (index % (base - 1)) + 1
    } else {
        // Multi-digit: ensure first digit is non-zero
        let max_val = base.pow(length as u32) - 1;
        let min_val = base.pow((length - 1) as u32);
        min_val + (index % (max_val - min_val + 1))
    }
}

// Count digits in a number (in given base)
fn count_digits(mut n: u32, base: u32) -> usize {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    while n > 0 {
        count += 1;
        n /= base;
    }
    count
}

// Test configuration at given seed length
fn test_seed_length(
    base: u32,
    seed_length: usize,
    num_seeds: u32,
    single_config: (u32, u32),
    double_config: ((u32, u32), (u32, u32)),
) -> (usize, usize, usize, usize) {
    let mut single_primes = 0;
    let mut single_total = 0;
    let mut double_primes = 0;
    let mut double_total = 0;

    for i in 0..num_seeds {
        let seed = generate_seed(seed_length, i, base);

        // Verify seed has correct length
        if count_digits(seed, base) != seed_length {
            continue;
        }

        // Test single membrane
        let single = single_membrane(single_config.0, single_config.1, seed, base);
        if is_prime(&single) {
            single_primes += 1;
        }
        single_total += 1;

        // Test double membrane
        let double = double_membrane(double_config.0, double_config.1, seed, base);
        if is_prime(&double) {
            double_primes += 1;
        }
        double_total += 1;
    }

    (single_primes, single_total, double_primes, double_total)
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║       SEED LENGTH CROSSOVER VALIDATION                       ║");
    println!("║       Testing φ × density × √base Formula                    ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    let bases = vec![6, 10, 14, 22];
    let max_seed_length = 7;
    let seeds_per_length = 50;

    // Print predictions
    println!("PREDICTIONS (φ × density × √base):");
    println!("─────────────────────────────────────────────────────────");
    for &base in &bases {
        let density = phase_lock_density(base);
        let predicted = predict_crossover(base);
        println!(
            "Base {:2}: density={:.3}, √base={:.2}, predicted crossover={:.2}",
            base,
            density,
            (base as f64).sqrt(),
            predicted
        );
    }
    println!();

    // Store results for each base
    let mut all_results: HashMap<u32, Vec<(usize, f64, f64)>> = HashMap::new();

    for &base in &bases {
        println!("═══════════════════════════════════════════════════════════════");
        println!("BASE {} ANALYSIS", base);
        println!("═══════════════════════════════════════════════════════════════");

        let single_config = get_single_config(base);
        let double_config = get_double_config(base);

        println!(
            "Single membrane: ({},{})",
            single_config.0, single_config.1
        );
        println!(
            "Double membrane: (({},{}), ({},{}))",
            double_config.0.0,
            double_config.0.1,
            double_config.1.0,
            double_config.1.1
        );
        println!();

        println!("┌──────┬─────────────────┬─────────────────┬────────────┐");
        println!("│ Seed │ Single Membrane │ Double Membrane │   Winner   │");
        println!("│ Len  │  Primes  Rate   │  Primes  Rate   │            │");
        println!("├──────┼─────────────────┼─────────────────┼────────────┤");

        let mut results = Vec::new();

        for seed_length in 1..=max_seed_length {
            let (single_p, single_t, double_p, double_t) = test_seed_length(
                base,
                seed_length,
                seeds_per_length,
                single_config,
                double_config,
            );

            let single_rate = if single_t > 0 {
                (single_p as f64) / (single_t as f64) * 100.0
            } else {
                0.0
            };

            let double_rate = if double_t > 0 {
                (double_p as f64) / (double_t as f64) * 100.0
            } else {
                0.0
            };

            let winner = if double_rate > single_rate {
                "DOUBLE ★"
            } else if single_rate > double_rate {
                "single"
            } else {
                "tie"
            };

            println!(
                "│  {:2}  │  {:2}/{:2}  {:5.1}% │  {:2}/{:2}  {:5.1}% │ {:10} │",
                seed_length,
                single_p,
                single_t,
                single_rate,
                double_p,
                double_t,
                double_rate,
                winner
            );

            results.push((seed_length, single_rate, double_rate));
        }

        println!("└──────┴─────────────────┴─────────────────┴────────────┘");
        println!();

        all_results.insert(base, results);
    }

    // Summary analysis
    println!("═══════════════════════════════════════════════════════════════");
    println!("CROSSOVER ANALYSIS SUMMARY");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    for &base in &bases {
        let predicted = predict_crossover(base);
        let results = &all_results[&base];

        // Find observed crossover (first length where double wins)
        let mut observed_crossover = None;
        for &(length, single_rate, double_rate) in results {
            if double_rate > single_rate {
                observed_crossover = Some(length);
                break;
            }
        }

        println!("Base {}:", base);
        println!(
            "  Predicted crossover: {:.2} (φ × {:.3} × {:.2})",
            predicted,
            phase_lock_density(base),
            (base as f64).sqrt()
        );

        if let Some(observed) = observed_crossover {
            let error = ((observed as f64 - predicted) / predicted * 100.0).abs();
            println!("  Observed crossover:  {}", observed);
            println!("  Error:               {:.1}%", error);

            if error < 20.0 {
                println!("  Status:              ✓ VALIDATED (within 20%)");
            } else if error < 50.0 {
                println!("  Status:              ~ PARTIAL (20-50% error)");
            } else {
                println!("  Status:              ✗ FALSIFIED (>50% error)");
            }
        } else {
            println!("  Observed crossover:  Not found in range 1-{}", max_seed_length);
            println!("  Status:              ? INCONCLUSIVE (need larger seeds)");
        }
        println!();
    }

    // Fibonacci ratio analysis
    println!("═══════════════════════════════════════════════════════════════");
    println!("FIBONACCI SIZE RATIO ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("Testing if nested_size / single_size ≈ 5/3 ≈ 1.667");
    println!();

    for &base in &bases {
        println!("Base {}:", base);

        let single_config = get_single_config(base);
        let double_config = get_double_config(base);

        // Generate examples at crossover length
        let predicted_length = predict_crossover(base).round() as usize;
        let seed = generate_seed(predicted_length, 0, base);

        let single = single_membrane(single_config.0, single_config.1, seed, base);
        let double = double_membrane(double_config.0, double_config.1, seed, base);

        let single_digits = single.to_string().len();
        let double_digits = double.to_string().len();
        let ratio = double_digits as f64 / single_digits as f64;

        println!("  Single membrane digits: {}", single_digits);
        println!("  Double membrane digits: {}", double_digits);
        println!("  Ratio: {:.3}", ratio);
        println!("  Expected (5/3): 1.667");
        println!("  Deviation: {:.1}%", ((ratio - 1.667) / 1.667 * 100.0).abs());
        println!();
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("VERIFICATION COMPLETE");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("All results are independently verifiable:");
    println!("- Primality testing uses is_prime() (Miller-Rabin)");
    println!("- Crossover = first seed length where double > single");
    println!("- Formula: φ × density × √base");
    println!("- Tolerance: ±20% for validation");
}
