// Verify Individual Anomaly Configuration
//
// Tool for high-precision verification of specific M=2 anomalies
// Supports high-power replication (n=25,000 samples) for Experiment E
//
// Usage:
//   cargo run --release --example verify_anomaly -- --base 8 --outer 5 --inner 1 --M 2 --samples 25000

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime;
use std::env;
use std::time::Instant;

// ============================================================================
// Core Functions
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

    add_digit(outer);
    for _ in 0..k { add_digit(0); }
    add_digit(inner);
    for _ in 0..k { add_digit(0); }

    let mut seed_val = seed;
    for _ in 0..m {
        add_digit((seed_val % base as u64) as u32);
        seed_val /= base as u64;
    }

    for _ in 0..k { add_digit(0); }
    add_digit(inner);
    for _ in 0..k { add_digit(0); }
    add_digit(outer);

    result
}

// ============================================================================
// Statistical Tests
// ============================================================================

struct TestResult {
    k: u32,
    samples: u64,
    primes: u64,
    density: f64,
}

fn two_proportion_z_test(p1: f64, n1: u64, p0: f64, n0: u64) -> (f64, f64) {
    // Pooled proportion
    let k1 = (p1 * n1 as f64) as u64;
    let k0 = (p0 * n0 as f64) as u64;
    let pooled_p = (k1 + k0) as f64 / (n1 + n0) as f64;

    // Standard error
    let se = (pooled_p * (1.0 - pooled_p) * (1.0 / n1 as f64 + 1.0 / n0 as f64)).sqrt();

    // Z-statistic
    let z = (p1 - p0) / se;

    // P-value (one-tailed, using normal approximation)
    let p_value = if z > 0.0 {
        // Standard normal CDF approximation
        0.5 * (1.0 - erf_approx(z / 2.0_f64.sqrt()))
    } else {
        1.0
    };

    (z, p_value)
}

// Error function approximation for p-value calculation
fn erf_approx(x: f64) -> f64 {
    let a1 =  0.254829592;
    let a2 = -0.284496736;
    let a3 =  1.421413741;
    let a4 = -1.453152027;
    let a5 =  1.061405429;
    let p  =  0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}

// ============================================================================
// Main Verification
// ============================================================================

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse command-line arguments
    let base = get_arg(&args, "--base").unwrap_or(8);
    let outer = get_arg(&args, "--outer").unwrap_or(5);
    let inner = get_arg(&args, "--inner").unwrap_or(1);
    let m = get_arg(&args, "--M").unwrap_or(2);
    let samples = get_arg(&args, "--samples").unwrap_or(1000) as u64;

    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║           ANOMALY VERIFICATION TOOL                   ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    println!("Configuration:");
    println!("  Base:   {}", base);
    println!("  Outer:  {}", outer);
    println!("  Inner:  {}", inner);
    println!("  M:      {}", m);
    println!("  Samples: {}\n", samples);

    // Seed range
    let seed_min = if m > 1 { (base as u64).pow((m - 1) as u32) } else { 1 };
    let seed_max = (base as u64).pow(m as u32);
    let total_seeds = seed_max - seed_min;

    println!("Seed range: {} to {} ({} total seeds)", seed_min, seed_max - 1, total_seeds);

    let samples_to_test = samples.min(total_seeds);
    println!("Testing: {} seeds ({:.1}% coverage)\n", samples_to_test,
             100.0 * samples_to_test as f64 / total_seeds as f64);

    println!("════════════════════════════════════════════════════════");
    println!("Running primality tests...");
    println!("════════════════════════════════════════════════════════\n");

    let k_values = vec![0, 1, 2, 3];
    let mut results = Vec::new();

    for &k in &k_values {
        let start = Instant::now();
        let mut prime_count = 0u64;

        for i in 0..samples_to_test {
            let seed = seed_min + i;
            let membrane = construct_membrane(base, outer, inner, m as usize, k, seed);

            if is_prime(&membrane) {
                prime_count += 1;
            }

            // Progress indicator
            if i > 0 && i % 1000 == 0 {
                print!("\r  k={}: {}/{} tested ({:.1}%)", k, i, samples_to_test,
                       100.0 * i as f64 / samples_to_test as f64);
                use std::io::{self, Write};
                io::stdout().flush().ok();
            }
        }

        let duration = start.elapsed();
        let density = prime_count as f64 / samples_to_test as f64;

        println!("\r  k={}: {}/{} primes ({:.2}% density) in {:.2}s",
                 k, prime_count, samples_to_test, density * 100.0, duration.as_secs_f64());

        results.push(TestResult {
            k,
            samples: samples_to_test,
            primes: prime_count,
            density,
        });
    }

    println!("\n════════════════════════════════════════════════════════");
    println!("RESULTS");
    println!("════════════════════════════════════════════════════════\n");

    // Find optimal k
    let mut best_idx = 0;
    let mut best_density = results[0].density;

    for (i, result) in results.iter().enumerate() {
        let marker = if result.density > best_density {
            best_idx = i;
            best_density = result.density;
            "★"
        } else if result.density == best_density {
            "★"
        } else {
            " "
        };

        println!("  k={}: {}/{} = {:.6} ({:.2}%) {}",
                 result.k,
                 result.primes,
                 result.samples,
                 result.density,
                 result.density * 100.0,
                 marker);
    }

    let k_star = results[best_idx].k;
    println!("\n  Optimal k* = {}", k_star);

    // Statistical analysis if k*=1
    if k_star == 1 && results.len() >= 2 {
        println!("\n════════════════════════════════════════════════════════");
        println!("STATISTICAL SIGNIFICANCE (k=1 vs k=0)");
        println!("════════════════════════════════════════════════════════\n");

        let k1_result = &results[1]; // k=1
        let k0_result = &results[0]; // k=0

        let delta = k1_result.density - k0_result.density;
        let (z, p_value) = two_proportion_z_test(
            k1_result.density, k1_result.samples,
            k0_result.density, k0_result.samples
        );

        println!("  Advantage: Δ = {:.2} percentage points", delta * 100.0);
        println!("  Z-statistic: z = {:.3}", z);
        println!("  P-value (one-tailed): p = {:.4}", p_value);

        println!("\n  Significance levels:");
        if p_value < 0.001 {
            println!("    ✓ p < 0.001  HIGHLY SIGNIFICANT ***");
        } else if p_value < 0.01 {
            println!("    ✓ p < 0.01   SIGNIFICANT **");
        } else if p_value < 0.05 {
            println!("    ✓ p < 0.05   SIGNIFICANT *");
        } else if p_value < 0.10 {
            println!("    ⚠ p < 0.10   MARGINALLY SIGNIFICANT");
        } else {
            println!("    ✗ p ≥ 0.10   NOT SIGNIFICANT");
        }

        // Bonferroni correction for 468 M=2 configurations
        let bonferroni_alpha = 0.05 / 468.0;
        println!("\n  Bonferroni correction (468 tests):");
        println!("    Required: p < {:.6}", bonferroni_alpha);
        if p_value < bonferroni_alpha {
            println!("    ✓ PASSES Bonferroni correction");
        } else {
            println!("    ✗ FAILS Bonferroni correction (likely false positive)");
        }
    } else if k_star == 0 {
        println!("\n  Result: k*=0 (consistent with universal minimal padding)");
    }

    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║                  VERIFICATION COMPLETE                 ║");
    println!("╚════════════════════════════════════════════════════════╝\n");
}

fn get_arg(args: &[String], flag: &str) -> Option<u32> {
    for i in 0..args.len() {
        if args[i] == flag && i + 1 < args.len() {
            return args[i + 1].parse().ok();
        }
    }
    None
}
