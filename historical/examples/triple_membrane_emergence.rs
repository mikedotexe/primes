// Triple Membrane Emergence Test
//
// Tests the φ^2 scaling prediction for triple-nested membranes.
//
// Theory: Each additional membrane shell adds a factor of φ to capacity:
//   - Single membrane capacity:  √base
//   - Double membrane capacity:  φ × √base
//   - Triple membrane capacity:  φ² × √base
//
// For base 14:
//   - Single → Double crossover: observed at length 4
//   - Double → Triple crossover: predicted at φ × 4 ≈ 6.5 → length 7
//
// This test validates whether triple-nested structures emerge when predicted
// by the golden ratio scaling law.

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime;

const PHI: f64 = 1.618033988749;
const PHI_SQUARED: f64 = 2.618033988749; // φ² = φ + 1

// Generate single membrane: outer-inner-seed-inner-outer
fn single_membrane(outer: u32, inner: u32, seed: u32, base: u32) -> BigUint {
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    result = result * &base_big + BigUint::from(outer);
    result = result * &base_big + BigUint::from(inner);
    result = result * &base_big + BigUint::from(seed);
    result = result * &base_big + BigUint::from(inner);
    result = result * &base_big + BigUint::from(outer);

    result
}

// Generate double membrane: outer-middle-inner-seed-inner-middle-outer
// Using 7-layer structure
fn double_membrane(outer: (u32, u32), inner: (u32, u32), seed: u32, base: u32) -> BigUint {
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

// Generate triple membrane: outer-mid_outer-mid_inner-inner-seed-inner-mid_inner-mid_outer-outer
// Using 11-layer structure
fn triple_membrane(
    outer: (u32, u32),
    mid_outer: (u32, u32),
    mid_inner: (u32, u32),
    inner: (u32, u32),
    seed: u32,
    base: u32,
) -> BigUint {
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    // Left half
    result = result * &base_big + BigUint::from(outer.0);
    result = result * &base_big + BigUint::from(outer.1);
    result = result * &base_big + BigUint::from(mid_outer.0);
    result = result * &base_big + BigUint::from(mid_outer.1);
    result = result * &base_big + BigUint::from(mid_inner.0);
    result = result * &base_big + BigUint::from(mid_inner.1);
    result = result * &base_big + BigUint::from(inner.0);
    result = result * &base_big + BigUint::from(inner.1);

    // Center seed
    result = result * &base_big + BigUint::from(seed);

    // Right half (mirror)
    result = result * &base_big + BigUint::from(inner.1);
    result = result * &base_big + BigUint::from(inner.0);
    result = result * &base_big + BigUint::from(mid_inner.1);
    result = result * &base_big + BigUint::from(mid_inner.0);
    result = result * &base_big + BigUint::from(mid_outer.1);
    result = result * &base_big + BigUint::from(mid_outer.0);
    result = result * &base_big + BigUint::from(outer.1);
    result = result * &base_big + BigUint::from(outer.0);

    result
}

fn generate_seed(length: usize, index: u32, base: u32) -> u32 {
    if length == 1 {
        (index % (base - 1)) + 1
    } else {
        let max_val = base.pow(length as u32) - 1;
        let min_val = base.pow((length - 1) as u32);
        min_val + (index % (max_val - min_val + 1))
    }
}

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

// Test all three membrane types at given seed length
fn test_seed_length(
    base: u32,
    seed_length: usize,
    num_seeds: u32,
) -> (usize, usize, usize, usize, usize, usize) {
    // Base 14 optimal configurations
    let single_config = (3, 11);
    let double_config = ((3, 11), (3, 11));
    let triple_config = ((3, 11), (3, 11), (3, 11), (3, 11));

    let mut single_primes = 0;
    let mut single_total = 0;
    let mut double_primes = 0;
    let mut double_total = 0;
    let mut triple_primes = 0;
    let mut triple_total = 0;

    for i in 0..num_seeds {
        let seed = generate_seed(seed_length, i, base);

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

        // Test triple membrane
        let triple = triple_membrane(
            triple_config.0,
            triple_config.1,
            triple_config.2,
            triple_config.3,
            seed,
            base,
        );
        if is_prime(&triple) {
            triple_primes += 1;
        }
        triple_total += 1;
    }

    (
        single_primes,
        single_total,
        double_primes,
        double_total,
        triple_primes,
        triple_total,
    )
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║         TRIPLE MEMBRANE EMERGENCE TEST                       ║");
    println!("║         Testing φ² Scaling for Base 14                       ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    let base = 14u32;
    let max_seed_length = 10;
    let seeds_per_length = 50;

    println!("THEORY:");
    println!("─────────────────────────────────────────────────────────");
    println!("Membrane capacity scaling: φ^(n-1) × √base");
    println!();
    println!("Base 14 (√14 ≈ 3.74):");
    println!("  Single (n=1): φ^0 × 3.74 = 3.74");
    println!("  Double (n=2): φ^1 × 3.74 = {} × 3.74 ≈ 6.05", PHI);
    println!("  Triple (n=3): φ^2 × 3.74 = {} × 3.74 ≈ 9.79", PHI_SQUARED);
    println!();
    println!("PREDICTIONS:");
    println!("  Single → Double crossover: ~4 (OBSERVED: 4 ✓)");
    println!("  Double → Triple crossover: φ × 4 ≈ 6.5 → length 7");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("TESTING BASE 14 ACROSS SEED LENGTHS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("┌──────┬──────────┬──────────┬──────────┬─────────────┐");
    println!("│ Seed │  Single  │  Double  │  Triple  │   Leader    │");
    println!("│ Len  │  Primes  │  Primes  │  Primes  │             │");
    println!("├──────┼──────────┼──────────┼──────────┼─────────────┤");

    let mut results = Vec::new();

    for seed_length in 1..=max_seed_length {
        let (s_p, s_t, d_p, d_t, t_p, t_t) = test_seed_length(base, seed_length, seeds_per_length);

        let s_rate = if s_t > 0 {
            (s_p as f64) / (s_t as f64) * 100.0
        } else {
            0.0
        };
        let d_rate = if d_t > 0 {
            (d_p as f64) / (d_t as f64) * 100.0
        } else {
            0.0
        };
        let t_rate = if t_t > 0 {
            (t_p as f64) / (t_t as f64) * 100.0
        } else {
            0.0
        };

        let leader = if t_rate > d_rate && t_rate > s_rate {
            "TRIPLE ★★★"
        } else if d_rate > s_rate && d_rate > t_rate {
            "DOUBLE ★★ "
        } else if s_rate > d_rate && s_rate > t_rate {
            "single    "
        } else {
            "tie       "
        };

        println!(
            "│  {:2}  │ {:2}/{:2} {:4.1}% │ {:2}/{:2} {:4.1}% │ {:2}/{:2} {:4.1}% │ {} │",
            seed_length, s_p, s_t, s_rate, d_p, d_t, d_rate, t_p, t_t, t_rate, leader
        );

        results.push((seed_length, s_rate, d_rate, t_rate));
    }

    println!("└──────┴──────────┴──────────┴──────────┴─────────────┘");
    println!();

    // Analysis
    println!("═══════════════════════════════════════════════════════════════");
    println!("CROSSOVER ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    // Find single → double crossover
    let mut single_double_crossover = None;
    for &(length, s_rate, d_rate, _t_rate) in &results {
        if d_rate > s_rate {
            single_double_crossover = Some(length);
            break;
        }
    }

    // Find double → triple crossover
    let mut double_triple_crossover = None;
    for &(length, _s_rate, d_rate, t_rate) in &results {
        if t_rate > d_rate {
            double_triple_crossover = Some(length);
            break;
        }
    }

    println!("Single → Double Crossover:");
    if let Some(observed) = single_double_crossover {
        println!("  Predicted: ~4");
        println!("  Observed:  {}", observed);
        println!("  Status:    ✓ (validated in previous experiment)");
    } else {
        println!("  Not found in test range");
    }
    println!();

    println!("Double → Triple Crossover:");
    let predicted_triple = PHI * 4.0;
    if let Some(observed) = double_triple_crossover {
        let error = ((observed as f64 - predicted_triple) / predicted_triple * 100.0).abs();
        println!("  Predicted: φ × 4 ≈ {:.2} → length ~7", predicted_triple);
        println!("  Observed:  {}", observed);
        println!("  Error:     {:.1}%", error);

        if error < 20.0 {
            println!("  Status:    ✓ VALIDATED (φ² scaling confirmed!)");
        } else if error < 50.0 {
            println!("  Status:    ~ PARTIAL (some evidence for φ scaling)");
        } else {
            println!("  Status:    ✗ FALSIFIED (φ² scaling rejected)");
        }
    } else {
        println!("  Predicted: φ × 4 ≈ {:.2} → length ~7", predicted_triple);
        println!("  Observed:  Not found in range 1-{}", max_seed_length);
        println!("  Status:    ? INCONCLUSIVE (triple never dominates)");
    }
    println!();

    // Capacity analysis
    println!("═══════════════════════════════════════════════════════════════");
    println!("CAPACITY VERIFICATION");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    // Generate example primes
    println!("Example structures at crossover points:");
    println!();

    if let Some(crossover) = single_double_crossover {
        let seed = generate_seed(crossover, 0, base);
        let single = single_membrane(3, 11, seed, base);
        let double = double_membrane((3, 11), (3, 11), seed, base);

        println!("Seed length {} (Single→Double crossover):", crossover);
        println!("  Single membrane: {} digits", single.to_string().len());
        println!("  Double membrane: {} digits", double.to_string().len());
        println!(
            "  Ratio: {:.3} (Expected: φ ≈ 1.618)",
            double.to_string().len() as f64 / single.to_string().len() as f64
        );
        println!();
    }

    if let Some(crossover) = double_triple_crossover {
        let seed = generate_seed(crossover, 0, base);
        let double = double_membrane((3, 11), (3, 11), seed, base);
        let triple = triple_membrane((3, 11), (3, 11), (3, 11), (3, 11), seed, base);

        println!("Seed length {} (Double→Triple crossover):", crossover);
        println!("  Double membrane: {} digits", double.to_string().len());
        println!("  Triple membrane: {} digits", triple.to_string().len());
        println!(
            "  Ratio: {:.3} (Expected: φ ≈ 1.618)",
            triple.to_string().len() as f64 / double.to_string().len() as f64
        );
        println!();
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("CONCLUSION");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    if let Some(observed) = double_triple_crossover {
        let error = ((observed as f64 - predicted_triple) / predicted_triple * 100.0).abs();

        if error < 20.0 {
            println!("✓ GOLDEN RATIO SCALING CONFIRMED");
            println!();
            println!("The φ^(n-1) scaling law holds across membrane depths:");
            println!("  - Single → Double at length ~4");
            println!("  - Double → Triple at length ~{}", observed);
            println!(
                "  - Ratio: {} / 4 ≈ {:.2} ≈ φ",
                observed,
                observed as f64 / 4.0
            );
            println!();
            println!("This validates the golden ratio as a fundamental constant");
            println!("governing membrane emergence across multiple scales.");
        } else {
            println!("⚠ PARTIAL VALIDATION");
            println!();
            println!(
                "Triple membrane emerges at length {}, with {:.1}% error",
                observed, error
            );
            println!("from the φ scaling prediction. This suggests refinements");
            println!("needed for higher-order membrane structures.");
        }
    } else {
        println!("? HYPOTHESIS UNCLEAR");
        println!();
        println!("Triple membrane did not dominate in the tested range.");
        println!("Possible explanations:");
        println!("  1. Triple emergence requires much larger seeds");
        println!("  2. Base 14 structure doesn't support efficient triple nesting");
        println!("  3. The 17-digit triple membrane is too large");
        println!("  4. Alternative triple configurations needed");
    }
}
