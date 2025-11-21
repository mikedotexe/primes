//! Solution Space Discriminant Explorer
//!
//! Per-seed discriminant analysis to validate the Quadratic Membrane Hypothesis.
//!
//! ## Purpose
//!
//! Tests whether discriminant Δ = S² - 4A² correlates with primality by tracking
//! per-seed data rather than aggregated statistics.
//!
//! ## Target Configurations
//!
//! Focuses on configurations where discriminant effects should be most visible:
//! - **Base 10, M=2, (3,7)**: The anomalous k=1 advantage case
//! - **Base 6, M=2, (1,5)**: High-performing universal pattern
//! - **Base 14, M=2**: Test case for 2p base effects
//!
//! ## Output Format
//!
//! Per-seed CSV with columns:
//! - Configuration params (base, M, outer, inner, k)
//! - Seed value and properties
//! - Discriminant and quadratic residue metrics
//! - Primality result
//!
//! ## Usage
//!
//! ```bash
//! # Analyze Base 10 M=2 anomaly (all k values)
//! cargo run --release --example solution_space_discriminant_explorer -- --base 10 --M 2
//!
//! # Quick test with limited seeds
//! cargo run --example solution_space_discriminant_explorer -- --base 10 --M 2 --limit 100
//! ```

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime;
use primes::hzlib::{
    PairCount, sieve_bool, sieve_spf,
    hl_goldbach_lambda, goldbach_coverage_from_lambda,
};
use std::env;
use std::fs::File;
use std::io::{Write as IoWrite, BufWriter};
use std::time::Instant;

// ============================================================================
// Configuration
// ============================================================================

#[derive(Debug, Clone)]
struct Args {
    base: u32,
    m: usize,
    outer: Option<u32>,
    inner: Option<u32>,
    k_min: u32,
    k_max: u32,
    seed_limit: Option<u64>,  // Optional: limit number of seeds for testing
    output_file: String,
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();

    let mut config = Args {
        base: 10,
        m: 2,
        outer: Some(3),
        inner: Some(7),
        k_min: 0,
        k_max: 3,
        seed_limit: None,
        output_file: "discriminant_per_seed.csv".to_string(),
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--base" | "-b" => {
                config.base = args[i + 1].parse().expect("Invalid base");
                i += 2;
            }
            "--M" | "-m" => {
                config.m = args[i + 1].parse().expect("Invalid M");
                i += 2;
            }
            "--outer" => {
                config.outer = Some(args[i + 1].parse().expect("Invalid outer"));
                i += 2;
            }
            "--inner" => {
                config.inner = Some(args[i + 1].parse().expect("Invalid inner"));
                i += 2;
            }
            "--k-min" => {
                config.k_min = args[i + 1].parse().expect("Invalid k_min");
                i += 2;
            }
            "--k-max" => {
                config.k_max = args[i + 1].parse().expect("Invalid k_max");
                i += 2;
            }
            "--limit" => {
                config.seed_limit = Some(args[i + 1].parse().expect("Invalid limit"));
                i += 2;
            }
            "--output" | "-o" => {
                config.output_file = args[i + 1].clone();
                i += 2;
            }
            _ => i += 1,
        }
    }

    config
}

// ============================================================================
// Per-Seed Data Structure
// ============================================================================

#[derive(Debug, Clone)]
struct SeedResult {
    // Configuration
    base: u32,
    m: usize,
    outer: u32,
    inner: u32,
    k: u32,

    // Seed properties
    seed: u64,
    seed_digit_count: usize,  // Actual digits in seed (base-b representation)

    // Membrane properties
    membrane_value: String,   // Decimal representation (for verification)
    membrane_digits: usize,   // Total digits in base-b

    // Discriminant analysis
    discriminant: i64,
    is_perfect_square: bool,
    sqrt_discriminant: i64,
    disc_mod_base: i64,
    disc_mod_3: i64,
    disc_mod_5: i64,
    disc_mod_7: i64,

    // Quadratic residues (Legendre symbols)
    qr_3: i8,
    qr_5: i8,
    qr_7: i8,
    qr_11: i8,
    qr_positive_count: usize,  // How many QR are +1

    // Goldbach decomposition
    goldbach_pairs: usize,
    goldbach_lambda: f64,
    goldbach_coverage: f64,

    // Result
    is_prime: bool,
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

fn digit_length(n: &BigUint, base: u32) -> usize {
    if n.is_zero() { return 1; }

    let mut len = 0;
    let mut temp = n.clone();
    let base_big = BigUint::from(base);

    while temp > BigUint::zero() {
        temp /= &base_big;
        len += 1;
    }

    len
}

fn seed_digit_count(seed: u64, base: u32) -> usize {
    if seed == 0 { return 1; }

    let mut count = 0;
    let mut s = seed;
    while s > 0 {
        s /= base as u64;
        count += 1;
    }
    count
}

// ============================================================================
// Discriminant Calculations
// ============================================================================

fn compute_discriminant(outer: u32, seed: u64) -> i64 {
    let s = seed as i64;
    let a = outer as i64;
    // Δ = S² - 4A²
    (s * s) - (4 * a * a)
}

fn is_perfect_square(n: i64) -> (bool, i64) {
    if n < 0 {
        return (false, -1);
    }

    let n_abs = n.abs() as u64;
    let sqrt = (n_abs as f64).sqrt() as u64;

    // Check sqrt and sqrt±1 to handle floating point errors
    for candidate in [sqrt.saturating_sub(1), sqrt, sqrt + 1] {
        if candidate * candidate == n_abs {
            return (true, candidate as i64);
        }
    }

    (false, -1)
}

fn legendre_symbol(a: i64, p: u32) -> i8 {
    let p_i64 = p as i64;
    let a_mod = ((a % p_i64) + p_i64) % p_i64;

    if a_mod == 0 {
        return 0;
    }

    // Euler's criterion: (a/p) ≡ a^((p-1)/2) (mod p)
    let exp = (p - 1) / 2;
    let result = mod_exp(a_mod as u64, exp as u64, p as u64);

    if result == 1 {
        1
    } else if result == p as u64 - 1 {
        -1
    } else {
        0
    }
}

fn mod_exp(base: u64, exp: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }

    let mut result = 1u64;
    let mut base = base % m;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % m;
        }
        exp >>= 1;
        base = (base * base) % m;
    }

    result
}

// ============================================================================
// Goldbach Analysis
// ============================================================================

fn count_goldbach_pairs(n: u64) -> usize {
    if n < 4 || n % 2 != 0 {
        return 0;
    }

    let n_usize = n as usize;
    let is_prime_vec = sieve_bool(n_usize + 1);

    let mut count = 0;

    for p in 2..=n/2 {
        let q = n - p;
        if (p as usize) < is_prime_vec.len() && (q as usize) < is_prime_vec.len() {
            if is_prime_vec[p as usize] && is_prime_vec[q as usize] {
                count += 1;
            }
        }
    }

    count
}

fn compute_goldbach_stats(n: u64, spf: &[usize]) -> (f64, f64) {
    if n < 4 || n % 2 != 0 {
        return (0.0, 0.0);
    }

    let n_usize = n as usize;
    let lambda = hl_goldbach_lambda(n_usize, spf, PairCount::Unordered);
    let coverage = goldbach_coverage_from_lambda(lambda);

    (lambda, coverage)
}

// ============================================================================
// Per-Seed Analysis
// ============================================================================

fn analyze_seed(
    base: u32,
    outer: u32,
    inner: u32,
    m: usize,
    k: u32,
    seed: u64,
    spf: &[usize],
) -> SeedResult {
    // 1. Construct membrane
    let membrane = construct_membrane(base, outer, inner, m, k, seed);
    let membrane_value = membrane.to_string();
    let membrane_digits = digit_length(&membrane, base);
    let seed_digits = seed_digit_count(seed, base);

    // 2. Compute discriminant
    let discriminant = compute_discriminant(outer, seed);
    let (is_perfect_square, sqrt_disc) = is_perfect_square(discriminant);
    let disc_mod_base = ((discriminant % base as i64) + base as i64) % base as i64;
    let disc_mod_3 = ((discriminant % 3) + 3) % 3;
    let disc_mod_5 = ((discriminant % 5) + 5) % 5;
    let disc_mod_7 = ((discriminant % 7) + 7) % 7;

    // 3. Quadratic residues
    let qr_3 = legendre_symbol(discriminant, 3);
    let qr_5 = legendre_symbol(discriminant, 5);
    let qr_7 = legendre_symbol(discriminant, 7);
    let qr_11 = legendre_symbol(discriminant, 11);

    let qr_positive_count = [qr_3, qr_5, qr_7, qr_11]
        .iter()
        .filter(|&&x| x == 1)
        .count();

    // 4. Goldbach analysis
    let goldbach_pairs = count_goldbach_pairs(seed);
    let (goldbach_lambda, goldbach_coverage) = if seed < 100000 {
        compute_goldbach_stats(seed, spf)
    } else {
        (0.0, 0.0)  // Skip expensive HL calculation for large seeds
    };

    // 5. Primality test
    let is_prime_result = is_prime(&membrane);

    SeedResult {
        base,
        m,
        outer,
        inner,
        k,
        seed,
        seed_digit_count: seed_digits,
        membrane_value,
        membrane_digits,
        discriminant,
        is_perfect_square,
        sqrt_discriminant: sqrt_disc,
        disc_mod_base,
        disc_mod_3,
        disc_mod_5,
        disc_mod_7,
        qr_3,
        qr_5,
        qr_7,
        qr_11,
        qr_positive_count,
        goldbach_pairs,
        goldbach_lambda,
        goldbach_coverage,
        is_prime: is_prime_result,
    }
}

// ============================================================================
// CSV Output
// ============================================================================

fn write_csv_header<W: IoWrite>(writer: &mut W) -> std::io::Result<()> {
    writeln!(
        writer,
        "base,M,outer,inner,k,seed,seed_digits,membrane_digits,\
         discriminant,is_perfect_square,sqrt_disc,disc_mod_base,disc_mod_3,disc_mod_5,disc_mod_7,\
         qr_3,qr_5,qr_7,qr_11,qr_positive_count,\
         goldbach_pairs,goldbach_lambda,goldbach_coverage,\
         is_prime,membrane_value"
    )
}

fn write_csv_row<W: IoWrite>(writer: &mut W, result: &SeedResult) -> std::io::Result<()> {
    writeln!(
        writer,
        "{},{},{},{},{},{},{},{},\
         {},{},{},{},{},{},{},\
         {},{},{},{},{},\
         {},{:.6},{:.6},\
         {},{}",
        result.base, result.m, result.outer, result.inner, result.k,
        result.seed, result.seed_digit_count, result.membrane_digits,
        result.discriminant, result.is_perfect_square, result.sqrt_discriminant,
        result.disc_mod_base, result.disc_mod_3, result.disc_mod_5, result.disc_mod_7,
        result.qr_3, result.qr_5, result.qr_7, result.qr_11, result.qr_positive_count,
        result.goldbach_pairs, result.goldbach_lambda, result.goldbach_coverage,
        result.is_prime, result.membrane_value
    )
}

// ============================================================================
// Main Analysis Loop
// ============================================================================

fn run_analysis(args: &Args) -> std::io::Result<()> {
    println!("🔬 Solution Space Discriminant Explorer");
    println!("═══════════════════════════════════════");
    println!();
    println!("Configuration:");
    println!("  Base: {}", args.base);
    println!("  M (seed length): {}", args.m);
    println!("  Boundaries: ({}, {})", args.outer.unwrap_or(0), args.inner.unwrap_or(0));
    println!("  Padding range: k ∈ [{}, {}]", args.k_min, args.k_max);
    if let Some(limit) = args.seed_limit {
        println!("  Seed limit: {} (testing mode)", limit);
    }
    println!("  Output: {}", args.output_file);
    println!();

    // Pre-compute sieve for Goldbach analysis
    let spf_limit = 100000;
    println!("📊 Pre-computing sieve (limit: {})...", spf_limit);
    let spf = sieve_spf(spf_limit);
    println!("✅ Sieve ready\n");

    // Calculate seed range
    let seed_min = if args.m > 1 {
        args.base.pow((args.m - 1) as u32) as u64
    } else {
        1
    };
    let seed_max = args.base.pow(args.m as u32) as u64;
    let seed_max = if let Some(limit) = args.seed_limit {
        seed_min + limit
    } else {
        seed_max
    };

    let total_seeds = seed_max - seed_min;
    let total_configs = (args.k_max - args.k_min + 1) as u64;
    let total_tests = total_seeds * total_configs;

    println!("Analysis scope:");
    println!("  Seeds: {} to {} ({} total)", seed_min, seed_max - 1, total_seeds);
    println!("  Padding values: {} to {} ({} total)", args.k_min, args.k_max, total_configs);
    println!("  Total primality tests: {}", total_tests);
    println!();

    // Determine boundary pairs
    let boundary_pairs = if let (Some(o), Some(i)) = (args.outer, args.inner) {
        vec![(o, i)]
    } else {
        // Generate all coprime pairs
        generate_coprime_pairs(args.base)
    };

    println!("📝 Writing results to {}...", args.output_file);
    let file = File::create(&args.output_file)?;
    let mut writer = BufWriter::new(file);
    write_csv_header(&mut writer)?;

    let start_time = Instant::now();
    let mut tests_completed = 0u64;
    let mut primes_found = 0u64;
    let report_interval = total_tests / 20;  // Report progress 20 times

    for (outer, inner) in boundary_pairs {
        for k in args.k_min..=args.k_max {
            for seed in seed_min..seed_max {
                let result = analyze_seed(args.base, outer, inner, args.m, k, seed, &spf);

                if result.is_prime {
                    primes_found += 1;
                }

                write_csv_row(&mut writer, &result)?;

                tests_completed += 1;

                if report_interval > 0 && tests_completed % report_interval == 0 {
                    let progress = 100.0 * tests_completed as f64 / total_tests as f64;
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let rate = tests_completed as f64 / elapsed;
                    let density = primes_found as f64 / tests_completed as f64;

                    println!(
                        "  Progress: {:.1}% ({}/{}) | Rate: {:.0} tests/s | Density: {:.4}",
                        progress, tests_completed, total_tests, rate, density
                    );
                }
            }
        }
    }

    writer.flush()?;
    let elapsed = start_time.elapsed().as_secs_f64();

    println!();
    println!("✅ Analysis complete!");
    println!();
    println!("Summary:");
    println!("  Total tests: {}", tests_completed);
    println!("  Primes found: {}", primes_found);
    println!("  Overall density: {:.6}", primes_found as f64 / tests_completed as f64);
    println!("  Time: {:.2}s ({:.0} tests/s)", elapsed, tests_completed as f64 / elapsed);
    println!();
    println!("🎯 Next step: Run quadratic_membrane_analyzer on this data!");

    Ok(())
}

fn generate_coprime_pairs(base: u32) -> Vec<(u32, u32)> {
    let mut pairs = Vec::new();

    fn gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    for outer in 1..base {
        if gcd(outer, base) != 1 { continue; }

        for inner in 1..base {
            if gcd(inner, base) != 1 { continue; }
            if outer == inner { continue; }

            pairs.push((outer, inner));
        }
    }

    pairs
}

// ============================================================================
// Main
// ============================================================================

fn main() -> std::io::Result<()> {
    let args = parse_args();
    run_analysis(&args)
}
