//! Discriminant Scanner - Test the Quadratic Membrane Hypothesis
//!
//! For membrane configurations N(X) = A·X² + S·X + A where X = b^k,
//! analyzes the discriminant Δ = S² - 4A² and its relationship to primality.
//!
//! ## Core Hypotheses
//!
//! 1. **Algebraic Lock**: If Δ is a perfect square, the polynomial factors
//!    algebraically → membranes are composite for all sufficiently large k
//!
//! 2. **Local Sieve**: For non-square Δ, Legendre symbols (Δ/q) determine
//!    sieve pressure - favorable signatures (many -1 symbols) → higher density
//!
//! 3. **Preferentialism Engine**: Configs with "good" discriminants across
//!    their seed space show sustained high density as M increases
//!
//! ## Usage
//!
//! ```bash
//! # Scan specific configuration
//! cargo run --release --example discriminant_scanner -- \
//!     --base 6 --outer 1 --inner 5 --k 0 --M-min 1 --M-max 3
//!
//! # Batch scan from CSV
//! cargo run --release --example discriminant_scanner -- \
//!     --batch solution_space_complete.csv --top 20
//! ```

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

// ============================================================================
// Discriminant Mathematics
// ============================================================================

/// Compute discriminant Δ = S² - 4A² for quadratic A·X² + S·X + A
fn compute_discriminant(shell: u64, seed: u64) -> i128 {
    let s = seed as i128;
    let a = shell as i128;
    s * s - 4 * a * a
}

/// Test if discriminant is a perfect square and return sqrt if so
fn is_square_i128(disc: i128) -> (bool, i128) {
    if disc < 0 {
        return (false, -1);
    }

    let disc_abs = disc.abs() as u128;
    let r = (disc_abs as f64).sqrt() as i128;

    // Check r-1, r, r+1 to handle floating point errors
    for candidate in [r - 1, r, r + 1] {
        if candidate < 0 {
            continue;
        }
        if (candidate * candidate) == disc {
            return (true, candidate);
        }
    }

    (false, -1)
}

/// Compute Legendre symbol (a/p) using Euler's criterion
/// Returns: +1 (quadratic residue), -1 (non-residue), 0 (divisible)
fn legendre_symbol(a: i128, p: u32) -> i8 {
    let p_i128 = p as i128;
    let a_mod = ((a % p_i128) + p_i128) % p_i128;

    if a_mod == 0 {
        return 0;
    }

    // Euler's criterion: (a/p) ≡ a^((p-1)/2) (mod p)
    let exp = (p - 1) / 2;
    let result = mod_exp(a_mod as u128, exp as u64, p as u64);

    if result == 1 {
        1
    } else if result == (p - 1) as u64 {
        -1
    } else {
        0 // Shouldn't happen for prime p
    }
}

/// Modular exponentiation: base^exp mod m
fn mod_exp(mut base: u128, mut exp: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }

    let m_u128 = m as u128;
    let mut result = 1u128;
    base %= m_u128;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % m_u128;
        }
        exp >>= 1;
        base = (base * base) % m_u128;
    }

    result as u64
}

// ============================================================================
// Discriminant Profile Analysis
// ============================================================================

#[derive(Debug, Clone)]
struct DiscriminantProfile {
    discriminant: i128,
    is_perfect_square: bool,
    sqrt_disc: i128,

    // Legendre symbols for small primes
    leg_3: i8,
    leg_5: i8,
    leg_7: i8,
    leg_11: i8,
    leg_13: i8,

    // Derived metrics
    admissible_count: usize,  // Count of -1 symbols (no roots mod q)
    obstructed_count: usize,  // Count of +1 symbols (two roots mod q)
    divisible_count: usize,   // Count of 0 symbols (divisible by q)
}

impl DiscriminantProfile {
    fn analyze(discriminant: i128) -> Self {
        let (is_perfect_square, sqrt_disc) = is_square_i128(discriminant);

        let leg_3 = legendre_symbol(discriminant, 3);
        let leg_5 = legendre_symbol(discriminant, 5);
        let leg_7 = legendre_symbol(discriminant, 7);
        let leg_11 = legendre_symbol(discriminant, 11);
        let leg_13 = legendre_symbol(discriminant, 13);

        let symbols = [leg_3, leg_5, leg_7, leg_11, leg_13];
        let admissible_count = symbols.iter().filter(|&&s| s == -1).count();
        let obstructed_count = symbols.iter().filter(|&&s| s == 1).count();
        let divisible_count = symbols.iter().filter(|&&s| s == 0).count();

        DiscriminantProfile {
            discriminant,
            is_perfect_square,
            sqrt_disc,
            leg_3,
            leg_5,
            leg_7,
            leg_11,
            leg_13,
            admissible_count,
            obstructed_count,
            divisible_count,
        }
    }

    /// Quality score: higher = better expected density
    /// Admissible primes are good, obstructed primes are bad, divisible is worst
    fn quality_score(&self) -> f64 {
        if self.is_perfect_square {
            return -100.0; // Algebraic lock - worst possible
        }

        // Score: +1 for each admissible prime, -1 for obstructed, -5 for divisible
        (self.admissible_count as f64) - (self.obstructed_count as f64) - 5.0 * (self.divisible_count as f64)
    }
}

// ============================================================================
// Configuration Analysis
// ============================================================================

#[derive(Debug, Clone)]
struct ConfigDiscriminantStats {
    base: u32,
    outer: u32,
    inner: u32,
    k: u32,
    m: usize,

    // Seed-level statistics
    total_seeds: usize,
    prime_seeds: usize,
    density: f64,

    // Discriminant statistics
    perfect_square_seeds: usize,
    mean_quality_score: f64,
    median_admissible_count: f64,

    // Correlation metrics
    quality_prime_correlation: f64,  // To be computed
}

fn analyze_configuration(
    base: u32,
    outer: u32,
    inner: u32,
    k: u32,
    m: usize,
) -> ConfigDiscriminantStats {
    // Calculate seed range
    let seed_min = if m > 1 {
        (base as u64).pow((m - 1) as u32)
    } else {
        1
    };
    let seed_max = (base as u64).pow(m as u32);

    let mut total_seeds = 0usize;
    let mut prime_seeds = 0usize;
    let mut perfect_square_seeds = 0usize;
    let mut quality_scores = Vec::new();
    let mut admissible_counts = Vec::new();
    let mut prime_indicators = Vec::new();

    println!("  Analyzing seeds {} to {}...", seed_min, seed_max - 1);

    for seed in seed_min..seed_max {
        // Compute discriminant profile
        let disc = compute_discriminant(outer as u64, seed);
        let profile = DiscriminantProfile::analyze(disc);

        // Construct membrane and test primality
        let membrane = construct_membrane(base, outer, inner, m, k, seed);
        let is_prime_result = is_prime(&membrane);

        // Collect statistics
        total_seeds += 1;
        if is_prime_result {
            prime_seeds += 1;
            prime_indicators.push(1.0);
        } else {
            prime_indicators.push(0.0);
        }

        if profile.is_perfect_square {
            perfect_square_seeds += 1;
        }

        quality_scores.push(profile.quality_score());
        admissible_counts.push(profile.admissible_count as f64);
    }

    let density = prime_seeds as f64 / total_seeds as f64;
    let mean_quality_score = quality_scores.iter().sum::<f64>() / total_seeds as f64;
    let median_admissible_count = median(&admissible_counts);

    // Compute Spearman correlation between quality score and primality
    let quality_prime_correlation = spearman_correlation(&quality_scores, &prime_indicators);

    ConfigDiscriminantStats {
        base,
        outer,
        inner,
        k,
        m,
        total_seeds,
        prime_seeds,
        density,
        perfect_square_seeds,
        mean_quality_score,
        median_admissible_count,
        quality_prime_correlation,
    }
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
// Statistical Utilities
// ============================================================================

fn median(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }

    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    }
}

fn spearman_correlation(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.is_empty() {
        return 0.0;
    }

    // Rank transformation
    let rank_x = rank(x);
    let rank_y = rank(y);

    // Pearson on ranks
    pearson_correlation(&rank_x, &rank_y)
}

fn rank(values: &[f64]) -> Vec<f64> {
    let n = values.len();
    let mut indexed: Vec<(usize, f64)> = values.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut ranks = vec![0.0; n];
    for (rank_idx, (orig_idx, _)) in indexed.iter().enumerate() {
        ranks[*orig_idx] = (rank_idx + 1) as f64;
    }

    ranks
}

fn pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len() as f64;
    if n < 2.0 {
        return 0.0;
    }

    let mean_x = x.iter().sum::<f64>() / n;
    let mean_y = y.iter().sum::<f64>() / n;

    let mut cov = 0.0;
    let mut var_x = 0.0;
    let mut var_y = 0.0;

    for i in 0..x.len() {
        let dx = x[i] - mean_x;
        let dy = y[i] - mean_y;
        cov += dx * dy;
        var_x += dx * dx;
        var_y += dy * dy;
    }

    if var_x == 0.0 || var_y == 0.0 {
        return 0.0;
    }

    cov / (var_x * var_y).sqrt()
}

// ============================================================================
// Main Analysis
// ============================================================================

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("🔬 Discriminant Scanner - Quadratic Membrane Hypothesis Tester");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Check if specific config requested via args
    let configs = if args.len() >= 3 && args[1] == "--config" {
        let config_name = &args[2];
        match config_name.as_str() {
            "base6-5-1" => vec![
                (6, 5, 1, 0, 1),
                (6, 5, 1, 0, 2),
                (6, 5, 1, 0, 3),
            ],
            "base12-1-5" => vec![
                (12, 1, 5, 0, 1),
                (12, 1, 5, 0, 2),
                (12, 1, 5, 0, 3),
            ],
            _ => vec![
                (6, 1, 5, 0, 1),
                (6, 1, 5, 0, 2),
                (6, 1, 5, 0, 3),
            ],
        }
    } else {
        // Default: test Base 6 (1,5) k=0 M=1-3 (the growth champion)
        vec![
            (6, 1, 5, 0, 1),
            (6, 1, 5, 0, 2),
            (6, 1, 5, 0, 3),
        ]
    };

    println!("Testing configurations:");
    for (base, outer, inner, k, m) in &configs {
        println!("  Base {}, ({},{}), k={}, M={}", base, outer, inner, k, m);
    }
    println!();

    let mut all_stats = Vec::new();

    for (base, outer, inner, k, m) in configs {
        println!("▶ Analyzing Base {} ({},{}) k={} M={}...", base, outer, inner, k, m);
        let stats = analyze_configuration(base, outer, inner, k, m);
        println!("  ✅ Density: {:.4} ({}/{})", stats.density, stats.prime_seeds, stats.total_seeds);
        println!("  ✅ Perfect squares: {}/{}", stats.perfect_square_seeds, stats.total_seeds);
        println!("  ✅ Mean quality score: {:.2}", stats.mean_quality_score);
        println!("  ✅ Quality-primality correlation: ρ={:.4}", stats.quality_prime_correlation);
        println!();

        all_stats.push(stats);
    }

    // Summary
    println!("═══════════════════════════════════════════════════════════════");
    println!("SUMMARY");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Results by M:");
    for stats in &all_stats {
        println!(
            "M={}: density={:.4}, quality_ρ={:.4}, perfect_squares={}/{}",
            stats.m,
            stats.density,
            stats.quality_prime_correlation,
            stats.perfect_square_seeds,
            stats.total_seeds
        );
    }

    println!("\n✅ Analysis complete!");
}
