// Solution Space Explorer: Extended M Range (M∈{5..10})
//
// EXPERIMENT A from CRITICAL_ANALYSIS_M2_ANOMALIES.md
//
// Question: Does k*=0 universality persist for M>3?
// Hypothesis: k*=0 for 100% of M∈{5..10} configurations
//
// Design:
//   - Bases: 6, 10, 14, 30 (representative sample)
//   - M values: 5, 6, 7, 8, 9, 10
//   - Boundary pairs: First 5 coprime pairs per base (sample)
//   - k values: 0, 1 (sufficient to test k*=0 vs k*=1)
//   - Seeds: First 100 valid seeds per M (sample, not exhaustive)
//
// Expected: 100% k*=0 across all M∈{5..10}
// Runtime: ~30-60 minutes

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime;
use std::fs::File;
use std::io::{Write as IoWrite, BufWriter};
use std::time::Instant;

// ============================================================================
// Core Functions (from solution_space_explorer.rs)
// ============================================================================

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

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
    for _ in 0..k { add_digit(0); }
    add_digit(inner);
    for _ in 0..k { add_digit(0); }

    // Middle (seed in base representation)
    let mut seed_val = seed;
    for _ in 0..m {
        add_digit((seed_val % base as u64) as u32);
        seed_val /= base as u64;
    }

    // Mirror
    for _ in 0..k { add_digit(0); }
    add_digit(inner);
    for _ in 0..k { add_digit(0); }
    add_digit(outer);

    result
}

fn generate_coprime_pairs(base: u32, limit: usize) -> Vec<(u32, u32)> {
    let mut pairs = Vec::new();

    for outer in 1..base {
        if gcd(outer, base) != 1 { continue; }

        for inner in 1..base {
            if gcd(inner, base) != 1 { continue; }
            if outer == inner { continue; }

            pairs.push((outer, inner));
            if pairs.len() >= limit { return pairs; }
        }
    }

    pairs
}

// ============================================================================
// Extended M Testing
// ============================================================================

#[derive(Debug)]
struct ExtendedResult {
    base: u32,
    m: usize,
    outer: u32,
    inner: u32,
    k: u32,
    samples: u64,
    prime_count: u64,
    density: f64,
}

fn test_extended_m(
    base: u32,
    outer: u32,
    inner: u32,
    m: usize,
    k: u32,
    num_samples: u64,
) -> ExtendedResult {
    let mut prime_count = 0u64;

    // Seed range
    let seed_min = if m > 1 { base.pow((m - 1) as u32) as u64 } else { 1 };
    let seed_max = base.pow(m as u32) as u64;
    let total_seeds = seed_max - seed_min;

    // Sample first num_samples seeds (or all if fewer)
    let samples_to_test = num_samples.min(total_seeds);

    for i in 0..samples_to_test {
        let seed = seed_min + i;
        let membrane = construct_membrane(base, outer, inner, m, k, seed);

        if is_prime(&membrane) {
            prime_count += 1;
        }
    }

    let density = if samples_to_test > 0 {
        prime_count as f64 / samples_to_test as f64
    } else {
        0.0
    };

    ExtendedResult {
        base,
        m,
        outer,
        inner,
        k,
        samples: samples_to_test,
        prime_count,
        density,
    }
}

// ============================================================================
// Main Experiment
// ============================================================================

fn main() -> std::io::Result<()> {
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║        EXTENDED M RANGE EXPERIMENT (M∈{{5..10}})        ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    println!("Testing M-dependence hypothesis for larger middle lengths");
    println!("Hypothesis: k*=0 universality persists for all M≥3\n");

    // Configuration
    let bases = vec![6, 10, 14, 30];
    let m_values = vec![5, 6, 7, 8, 9, 10];
    let k_values = vec![0, 1];  // Only need to test k=0 vs k=1
    let pairs_per_base = 5;      // Sample 5 coprime pairs per base
    let samples_per_config = 100; // 100 seeds per configuration

    // Create output file
    let file = File::create("extended_m_results.csv")?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "base,M,outer,inner,k,samples,prime_count,density")?;

    let total_start = Instant::now();
    let mut total_configs = 0;
    let mut k_zero_wins = 0;
    let mut k_one_wins = 0;
    let mut ties = 0;

    for &base in &bases {
        println!("════════════════════════════════════════════════════════");
        println!("Testing Base {}", base);
        println!("════════════════════════════════════════════════════════\n");

        let pairs = generate_coprime_pairs(base, pairs_per_base);
        println!("  Using {} coprime boundary pairs\n", pairs.len());

        for &m in &m_values {
            println!("  M={}: Testing {} pairs × {} k-values", m, pairs.len(), k_values.len());

            let m_start = Instant::now();

            for &(outer, inner) in &pairs {
                let mut densities: Vec<(u32, f64)> = Vec::new();

                for &k in &k_values {
                    let result = test_extended_m(base, outer, inner, m, k, samples_per_config);

                    writeln!(
                        writer,
                        "{},{},{},{},{},{},{},{:.6}",
                        result.base,
                        result.m,
                        result.outer,
                        result.inner,
                        result.k,
                        result.samples,
                        result.prime_count,
                        result.density
                    )?;

                    densities.push((k, result.density));
                    total_configs += 1;
                }

                // Determine k*
                densities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                let (k_star, _density_star) = densities[0];

                if k_star == 0 {
                    k_zero_wins += 1;
                } else if k_star == 1 {
                    k_one_wins += 1;
                } else if densities[0].1 == densities[1].1 {
                    ties += 1;
                }
            }

            let m_duration = m_start.elapsed();
            println!("    Completed in {:.2}s", m_duration.as_secs_f64());
        }

        writer.flush()?;
        println!("\n  ✓ Base {} complete\n", base);
    }

    let total_duration = total_start.elapsed();

    println!("╔════════════════════════════════════════════════════════╗");
    println!("║                 EXPERIMENT COMPLETE                    ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    println!("Statistics:");
    println!("  Total configurations tested: {}", total_configs);
    println!("  Total runtime:               {:.2}s", total_duration.as_secs_f64());
    println!("\nk* Distribution:");
    println!("  k*=0: {} ({:.1}%)", k_zero_wins, 100.0 * k_zero_wins as f64 / total_configs as f64);
    println!("  k*=1: {} ({:.1}%)", k_one_wins, 100.0 * k_one_wins as f64 / total_configs as f64);
    println!("  Ties: {} ({:.1}%)", ties, 100.0 * ties as f64 / total_configs as f64);

    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║                  HYPOTHESIS TEST                       ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    let k_zero_percentage = 100.0 * k_zero_wins as f64 / total_configs as f64;

    if k_zero_percentage == 100.0 {
        println!("✓ HYPOTHESIS CONFIRMED:");
        println!("  k*=0 for 100% of M∈{{5..10}} configurations");
        println!("  Asymptotic regime extends to M≥5");
        println!("  M=3 threshold is ROBUST");
    } else if k_zero_percentage >= 95.0 {
        println!("⚠ HYPOTHESIS MOSTLY SUPPORTED:");
        println!("  k*=0 for {:.1}% of configurations", k_zero_percentage);
        println!("  Near-universal, rare exceptions exist");
    } else {
        println!("✗ HYPOTHESIS REFUTED:");
        println!("  k*=0 for only {:.1}% of configurations", k_zero_percentage);
        println!("  M>3 does NOT exhibit universal k*=0");
    }

    println!("\nOutput: extended_m_results.csv");
    println!("Next: Analyze individual exceptions (if any)\n");

    Ok(())
}
