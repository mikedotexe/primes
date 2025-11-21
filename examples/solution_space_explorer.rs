// Solution Space Explorer: Systematic Parameter Space Mapping
//
// NO HYPOTHESES. NO ASSUMPTIONS. JUST DATA.
//
// Systematically explores the complete membrane prime construction parameter space:
// - Bases: 6, 8, 10, 12, 14, 15, 16, 18, 20, 22, 24, 30
// - Middle lengths (M): 1, 2, 3
// - Padding (k): 0, 1, 2, 3
// - ALL coprime boundary pairs
//
// Exhaustive enumeration (not sampling) for M≤3 to ensure complete accuracy.
// Outputs complete CSV dataset for pattern analysis.

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime;
use std::fs::File;
use std::io::{Write as IoWrite, BufWriter};
use std::time::Instant;

// ============================================================================
// Configuration and Data Structures
// ============================================================================

#[derive(Debug, Clone)]
struct ConfigResult {
    // Configuration parameters
    base: u32,
    m: usize,
    outer: u32,
    inner: u32,
    k: u32,

    // Counts
    total_candidates: u64,
    prime_count: u64,
    density: f64,

    // Base properties
    midpoint: f64,
    phi_base: u32,       // Euler's totient
    tau_base: u32,       // Number of divisors
    rad_base: u32,       // Radical (product of distinct primes)

    // Boundary properties
    outer_gcd: u32,
    inner_gcd: u32,
    outer_is_prime: bool,
    inner_is_prime: bool,

    // Structural
    min_length: usize,   // Minimum digit length
    max_length: usize,   // Maximum digit length
}

// ============================================================================
// Number Theory Utilities
// ============================================================================

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn euler_totient(n: u32) -> u32 {
    let mut result = n;
    let mut n = n;
    let mut p = 2;

    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            result -= result / p;
        }
        p += 1;
    }

    if n > 1 {
        result -= result / n;
    }

    result
}

fn count_divisors(n: u32) -> u32 {
    let mut count = 0;
    let mut i = 1;

    while i * i <= n {
        if n % i == 0 {
            count += 1;
            if i != n / i {
                count += 1;
            }
        }
        i += 1;
    }

    count
}

fn radical(n: u32) -> u32 {
    let mut rad = 1;
    let mut n = n;
    let mut p = 2;

    while p * p <= n {
        if n % p == 0 {
            rad *= p;
            while n % p == 0 {
                n /= p;
            }
        }
        p += 1;
    }

    if n > 1 {
        rad *= n;
    }

    rad
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

// ============================================================================
// Configuration Testing
// ============================================================================

fn test_configuration(
    base: u32,
    outer: u32,
    inner: u32,
    m: usize,
    k: u32,
) -> ConfigResult {
    let mut prime_count = 0u64;
    let mut total_candidates = 0u64;

    // Calculate seed range
    let seed_min = if m > 1 { base.pow((m - 1) as u32) as u64 } else { 1 };
    let seed_max = base.pow(m as u32) as u64;

    let mut min_length = usize::MAX;
    let mut max_length = 0;

    // Exhaustive enumeration
    for seed in seed_min..seed_max {
        let membrane = construct_membrane(base, outer, inner, m, k, seed);

        let len = digit_length(&membrane, base);
        min_length = min_length.min(len);
        max_length = max_length.max(len);

        if is_prime(&membrane) {
            prime_count += 1;
        }
        total_candidates += 1;
    }

    let density = if total_candidates > 0 {
        prime_count as f64 / total_candidates as f64
    } else {
        0.0
    };

    // Calculate base properties
    let midpoint = base as f64 / 2.0;
    let phi_base = euler_totient(base);
    let tau_base = count_divisors(base);
    let rad_base = radical(base);

    // Boundary properties
    let outer_gcd = gcd(outer, base);
    let inner_gcd = gcd(inner, base);
    let outer_is_prime = is_prime(&BigUint::from(outer));
    let inner_is_prime = is_prime(&BigUint::from(inner));

    ConfigResult {
        base,
        m,
        outer,
        inner,
        k,
        total_candidates,
        prime_count,
        density,
        midpoint,
        phi_base,
        tau_base,
        rad_base,
        outer_gcd,
        inner_gcd,
        outer_is_prime,
        inner_is_prime,
        min_length,
        max_length,
    }
}

// ============================================================================
// Coprime Pair Generation
// ============================================================================

fn generate_coprime_pairs(base: u32) -> Vec<(u32, u32)> {
    let mut pairs = Vec::new();

    for outer in 1..base {
        if gcd(outer, base) != 1 { continue; }

        for inner in 1..base {
            if gcd(inner, base) != 1 { continue; }
            if outer == inner { continue; }  // Require distinct boundaries

            pairs.push((outer, inner));
        }
    }

    pairs
}

// ============================================================================
// CSV Export
// ============================================================================

fn write_csv_header<W: IoWrite>(writer: &mut W) -> std::io::Result<()> {
    writeln!(
        writer,
        "base,M,outer,inner,k,total_candidates,prime_count,density,\
         midpoint,phi_base,tau_base,rad_base,\
         outer_gcd,inner_gcd,outer_is_prime,inner_is_prime,\
         min_length,max_length"
    )
}

fn write_csv_row<W: IoWrite>(writer: &mut W, result: &ConfigResult) -> std::io::Result<()> {
    writeln!(
        writer,
        "{},{},{},{},{},{},{},{:.6},\
         {:.1},{},{},{},\
         {},{},{},{},\
         {},{}",
        result.base,
        result.m,
        result.outer,
        result.inner,
        result.k,
        result.total_candidates,
        result.prime_count,
        result.density,
        result.midpoint,
        result.phi_base,
        result.tau_base,
        result.rad_base,
        result.outer_gcd,
        result.inner_gcd,
        result.outer_is_prime,
        result.inner_is_prime,
        result.min_length,
        result.max_length,
    )
}

// ============================================================================
// Main Exploration
// ============================================================================

fn main() -> std::io::Result<()> {
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║      SOLUTION SPACE EXPLORER: SYSTEMATIC MAPPING      ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    println!("Pure exploration. No hypotheses. Just data.\n");

    // Configuration
    let bases = vec![6, 8, 10, 12, 14, 15, 16, 18, 20, 22, 24, 30];
    let m_values = vec![1, 2, 3];
    let k_values = vec![0, 1, 2, 3];

    // Create output file
    let file = File::create("solution_space_complete.csv")?;
    let mut writer = BufWriter::new(file);
    write_csv_header(&mut writer)?;

    let total_start = Instant::now();
    let mut total_configs = 0;
    let mut total_tests = 0u64;

    // Explore each base
    for &base in &bases {
        println!("════════════════════════════════════════════════════════");
        println!("Exploring Base {}", base);
        println!("════════════════════════════════════════════════════════\n");

        let pairs = generate_coprime_pairs(base);
        println!("  Found {} coprime boundary pairs", pairs.len());

        // Test each configuration
        for &m in &m_values {
            println!("\n  M={}: Testing {} pairs × {} k values = {} configs",
                m, pairs.len(), k_values.len(), pairs.len() * k_values.len());

            let m_start = Instant::now();
            let mut configs_tested = 0;

            for &(outer, inner) in &pairs {
                for &k in &k_values {
                    let result = test_configuration(base, outer, inner, m, k);
                    write_csv_row(&mut writer, &result)?;

                    total_configs += 1;
                    configs_tested += 1;
                    total_tests += result.total_candidates;

                    // Progress indicator every 50 configs
                    if configs_tested % 50 == 0 {
                        print!(".");
                        std::io::stdout().flush().ok();
                    }
                }
            }

            let m_duration = m_start.elapsed();
            println!("\n    Completed {} configs in {:.2}s", configs_tested, m_duration.as_secs_f64());
        }

        writer.flush()?;
        println!("\n  ✓ Base {} complete\n", base);
    }

    let total_duration = total_start.elapsed();

    println!("╔════════════════════════════════════════════════════════╗");
    println!("║                  EXPLORATION COMPLETE                  ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    println!("Statistics:");
    println!("  Total configurations tested: {}", total_configs);
    println!("  Total primality tests:       {}", total_tests);
    println!("  Total runtime:               {:.2}s", total_duration.as_secs_f64());
    println!("  Average per config:          {:.3}s", total_duration.as_secs_f64() / total_configs as f64);

    println!("\nOutput:");
    println!("  File: solution_space_complete.csv");
    println!("  Rows: {} (+ header)", total_configs);
    println!("\nNext steps:");
    println!("  1. Run pattern_analyzer to identify correlations");
    println!("  2. Use interactive_explorer to query specific patterns");
    println!("  3. Let the data reveal what matters\n");

    Ok(())
}
