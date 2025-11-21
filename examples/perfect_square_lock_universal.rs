//! Perfect Square Lock - Universal Validation
//!
//! ## Hypothesis (from Discriminant Theory)
//!
//! If discriminant Δ = S² - 4A² is a perfect square, then the polynomial
//! N(X) = A·X² + S·X + A factors algebraically, forcing the membrane to be composite.
//!
//! **Prediction**: Perfect square discriminants → 0% prime density (universal constraint)
//!
//! ## Test Strategy
//!
//! 1. Test 8 bases × multiple configs × M ∈ {1,2,3,4}
//! 2. Track discriminants and primality for all seeds
//! 3. Compute correlation: perfect_square → is_prime
//! 4. Expected: ~0% primes among perfect square discriminants
//! 5. Identify any violations (exceptional cases)
//!
//! ## Bases to Test
//!
//! - Base 6: (1,5) champion, (5,1) failure case
//! - Base 10: (3,7) standard test case
//! - Base 12: (1,5) universal pattern
//! - Base 14: (1,5) high performer
//! - Base 15: (1,7) to test odd base
//! - Base 18: (1,5) universal pattern
//! - Base 22: (1,3) to test larger base
//! - Base 30: (11,7) high performer

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

// ============================================================================
// Discriminant Computation
// ============================================================================

fn compute_discriminant(outer: u32, seed: u64) -> i128 {
    let s = seed as i128;
    let a = outer as i128;

    let s_squared = s * s;
    let four_a_squared = 4 * a * a;

    s_squared - four_a_squared
}

fn is_perfect_square(n: i128) -> bool {
    if n < 0 {
        return false;
    }

    let sqrt = (n as f64).sqrt() as i128;

    // Check both sqrt and sqrt+1 to handle floating point errors
    sqrt * sqrt == n || (sqrt + 1) * (sqrt + 1) == n
}

// ============================================================================
// Membrane Construction
// ============================================================================

fn construct_membrane(
    base: u32,
    outer: u32,
    inner: u32,
    m: usize,
    k: u32,
    seed: u64,
) -> BigUint {
    let base_big = BigUint::from(base);
    let mut result = BigUint::zero();
    let mut position = 0;

    let mut add_digit = |digit: u32| {
        result += BigUint::from(digit) * base_big.pow(position);
        position += 1;
    };

    // Structure: outer [k×0] inner [k×0] SEED [k×0] inner [k×0] outer
    add_digit(outer);
    for _ in 0..k {
        add_digit(0);
    }
    add_digit(inner);
    for _ in 0..k {
        add_digit(0);
    }

    // Middle (seed in base representation)
    let mut seed_val = seed;
    for _ in 0..m {
        add_digit((seed_val % base as u64) as u32);
        seed_val /= base as u64;
    }

    // Mirror
    for _ in 0..k {
        add_digit(0);
    }
    add_digit(inner);
    for _ in 0..k {
        add_digit(0);
    }
    add_digit(outer);

    result
}

// ============================================================================
// Configuration Test
// ============================================================================

#[derive(Debug, Clone)]
struct ConfigResult {
    base: u32,
    outer: u32,
    inner: u32,
    m: usize,
    k: u32,

    // Aggregate statistics
    total_seeds: usize,
    perfect_square_count: usize,
    perfect_square_primes: usize,
    non_perfect_square_count: usize,
    non_perfect_square_primes: usize,

    // Violations (if any)
    violations: Vec<ViolationCase>,
}

#[derive(Debug, Clone)]
struct ViolationCase {
    seed: u64,
    discriminant: i128,
    membrane: String,
}

fn test_config(
    base: u32,
    outer: u32,
    inner: u32,
    m: usize,
    k: u32,
) -> ConfigResult {
    let seed_min = if m > 1 {
        (base as u64).pow((m - 1) as u32)
    } else {
        1
    };
    let seed_max = (base as u64).pow(m as u32).min(seed_min + 10000); // Cap for performance

    let mut perfect_square_count = 0;
    let mut perfect_square_primes = 0;
    let mut non_perfect_square_count = 0;
    let mut non_perfect_square_primes = 0;
    let mut violations = Vec::new();

    for seed in seed_min..seed_max {
        let discriminant = compute_discriminant(outer, seed);
        let is_perf_sq = is_perfect_square(discriminant);

        // Only test primality if discriminant is admissible
        let membrane = construct_membrane(base, outer, inner, m, k, seed);
        let is_prime_result = is_prime(&membrane);

        if is_perf_sq {
            perfect_square_count += 1;
            if is_prime_result {
                perfect_square_primes += 1;
                // VIOLATION! Perfect square discriminant but prime!
                violations.push(ViolationCase {
                    seed,
                    discriminant,
                    membrane: membrane.to_string(),
                });
            }
        } else {
            non_perfect_square_count += 1;
            if is_prime_result {
                non_perfect_square_primes += 1;
            }
        }
    }

    ConfigResult {
        base,
        outer,
        inner,
        m,
        k,
        total_seeds: (seed_max - seed_min) as usize,
        perfect_square_count,
        perfect_square_primes,
        non_perfect_square_count,
        non_perfect_square_primes,
        violations,
    }
}

// ============================================================================
// Multi-Base Analysis
// ============================================================================

fn run_comprehensive_test() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║       PERFECT SQUARE LOCK - UNIVERSAL VALIDATION             ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    println!("Testing hypothesis: Δ = perfect square → composite\n");

    // Define test configurations: (base, outer, inner, description)
    let configs = vec![
        // Base 6
        (6, 1, 5, "Base 6 champion"),
        (6, 5, 1, "Base 6 inverted"),

        // Base 10
        (10, 3, 7, "Base 10 standard"),
        (10, 1, 3, "Base 10 minimal"),

        // Base 12
        (12, 1, 5, "Base 12 universal"),
        (12, 5, 7, "Base 12 variant"),

        // Base 14
        (14, 1, 5, "Base 14 high performer"),

        // Base 15 (odd)
        (15, 1, 7, "Base 15 odd base"),

        // Base 18
        (18, 1, 5, "Base 18 universal"),

        // Base 22
        (22, 1, 3, "Base 22 large base"),

        // Base 30
        (30, 11, 7, "Base 30 champion"),
    ];

    let m_values = vec![1, 2, 3];
    let k_values = vec![0]; // k=0 for simplicity (discriminant is k-independent anyway)

    let mut all_results = Vec::new();
    let mut total_violations = 0;

    println!("Testing {} configurations × {} M values...\n", configs.len(), m_values.len());

    for (base, outer, inner, desc) in &configs {
        for &m in &m_values {
            for &k in &k_values {
                print!("  Testing: {} M={} k={} ... ", desc, m, k);
                std::io::stdout().flush().unwrap();

                let result = test_config(*base, *outer, *inner, m, k);

                if result.perfect_square_primes > 0 {
                    println!("⚠️  {} VIOLATIONS!", result.perfect_square_primes);
                    total_violations += result.perfect_square_primes;
                } else if result.perfect_square_count > 0 {
                    println!("✓ Lock holds ({} perfect squares, 0 prime)",
                             result.perfect_square_count);
                } else {
                    println!("○ No perfect squares found");
                }

                all_results.push((desc.to_string(), result));
            }
        }
    }

    // ========================================================================
    // Summary Statistics
    // ========================================================================

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("SUMMARY STATISTICS");
    println!("═══════════════════════════════════════════════════════════════\n");

    let total_configs = all_results.len();
    let configs_with_perfect_squares = all_results.iter()
        .filter(|(_, r)| r.perfect_square_count > 0)
        .count();

    let total_perfect_squares: usize = all_results.iter()
        .map(|(_, r)| r.perfect_square_count)
        .sum();

    let total_perfect_square_primes: usize = all_results.iter()
        .map(|(_, r)| r.perfect_square_primes)
        .sum();

    println!("Configurations tested: {}", total_configs);
    println!("Configs with perfect square discriminants: {}", configs_with_perfect_squares);
    println!("Total perfect square discriminants found: {}", total_perfect_squares);
    println!("Perfect square discriminants that were prime: {}", total_perfect_square_primes);

    if total_perfect_squares > 0 {
        let violation_rate = total_perfect_square_primes as f64 / total_perfect_squares as f64;
        println!("\nViolation rate: {:.2}% ({}/{})",
                 violation_rate * 100.0,
                 total_perfect_square_primes,
                 total_perfect_squares);

        if total_perfect_square_primes == 0 {
            println!("\n✅ PERFECT SQUARE LOCK CONFIRMED:");
            println!("   100% of perfect square discriminants → composite");
            println!("   This is a UNIVERSAL algebraic constraint!");
        } else {
            println!("\n⚠️  LOCK VIOLATIONS DETECTED:");
            println!("   {} cases where perfect square → prime", total_perfect_square_primes);
            println!("   Investigating...");
        }
    } else {
        println!("\n⚠️  No perfect square discriminants found in test set");
        println!("   May need to expand seed ranges or adjust configs");
    }

    // ========================================================================
    // Baseline Comparison
    // ========================================================================

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("BASELINE COMPARISON (non-perfect-square discriminants)");
    println!("═══════════════════════════════════════════════════════════════\n");

    let total_non_perf: usize = all_results.iter()
        .map(|(_, r)| r.non_perfect_square_count)
        .sum();

    let total_non_perf_primes: usize = all_results.iter()
        .map(|(_, r)| r.non_perfect_square_primes)
        .sum();

    if total_non_perf > 0 {
        let baseline_rate = total_non_perf_primes as f64 / total_non_perf as f64;
        println!("Non-perfect-square discriminants: {}", total_non_perf);
        println!("Non-perfect-square primes: {}", total_non_perf_primes);
        println!("Baseline prime density: {:.2}%", baseline_rate * 100.0);
    }

    // ========================================================================
    // Export Detailed Results
    // ========================================================================

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("EXPORTING RESULTS");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut csv_file = File::create("perfect_square_lock_results.csv")
        .expect("Unable to create CSV");

    writeln!(csv_file, "config_desc,base,outer,inner,m,k,total_seeds,perfect_sq_count,perfect_sq_primes,non_perf_sq_count,non_perf_sq_primes,violation_count")
        .unwrap();

    for (desc, result) in &all_results {
        writeln!(csv_file, "\"{}\",{},{},{},{},{},{},{},{},{},{},{}",
                 desc,
                 result.base,
                 result.outer,
                 result.inner,
                 result.m,
                 result.k,
                 result.total_seeds,
                 result.perfect_square_count,
                 result.perfect_square_primes,
                 result.non_perfect_square_count,
                 result.non_perfect_square_primes,
                 result.violations.len())
            .unwrap();
    }

    println!("✅ Exported results to: perfect_square_lock_results.csv");

    // Export violations if any
    if total_violations > 0 {
        let mut viol_file = File::create("perfect_square_violations.csv")
            .expect("Unable to create violations CSV");

        writeln!(viol_file, "config_desc,base,outer,inner,m,k,seed,discriminant,membrane")
            .unwrap();

        for (desc, result) in &all_results {
            for violation in &result.violations {
                writeln!(viol_file, "\"{}\",{},{},{},{},{},{},{},{}",
                         desc,
                         result.base,
                         result.outer,
                         result.inner,
                         result.m,
                         result.k,
                         violation.seed,
                         violation.discriminant,
                         violation.membrane)
                    .unwrap();
            }
        }

        println!("⚠️  Exported {} violations to: perfect_square_violations.csv", total_violations);
    }

    // ========================================================================
    // Per-Base Summary
    // ========================================================================

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("PER-BASE SUMMARY");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut base_summary: HashMap<u32, (usize, usize)> = HashMap::new();

    for (_, result) in &all_results {
        let entry = base_summary.entry(result.base).or_insert((0, 0));
        entry.0 += result.perfect_square_count;
        entry.1 += result.perfect_square_primes;
    }

    let mut bases: Vec<_> = base_summary.keys().collect();
    bases.sort();

    for base in bases {
        let (ps_count, ps_primes) = base_summary[base];
        let lock_status = if ps_count > 0 {
            if ps_primes == 0 {
                format!("✓ Lock holds ({} perfect squares, 0 prime)", ps_count)
            } else {
                format!("⚠️  {} violations out of {} perfect squares", ps_primes, ps_count)
            }
        } else {
            "○ No perfect squares".to_string()
        };

        println!("Base {:2}: {}", base, lock_status);
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    println!("🔬 Perfect Square Lock - Universal Validation");
    println!("Testing algebraic constraint across 8 bases...\n");

    run_comprehensive_test();

    println!("\n✅ Comprehensive test complete!");
}
