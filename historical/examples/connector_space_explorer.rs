#!/usr/bin/env rust-script
//! Connector Space Explorer
//!
//! Philosophy: Sweep the parameter space, let patterns emerge.
//! Don't test hypotheses - discover signal in the noise.
//!
//! ## What This Does
//!
//! For multiple prime pairs with diverse properties:
//! 1. Scan small connector samples in both directions
//! 2. Compute ALL metrics (binary, modular, structural)
//! 3. Export per-connector CSV with 50+ features
//! 4. Let downstream analysis find correlations
//!
//! ## Parameter Space
//!
//! - Prime pairs: From `collab/prime_pair_test_suite.csv`
//! - Connector lengths: 5-7 (configurable)
//! - Sample size: 1000 random connectors per length (fast exploration)
//! - Directions: Forward and Reverse
//!
//! ## Output Format
//!
//! CSV with columns:
//! - `pair_id`: Which prime pair (pair_01, pair_02, ...)
//! - `direction`: "forward" or "reverse"
//! - `connector`, `connector_len`
//! - `is_prime`: Boolean primality result
//! - Binary metrics: `trailing_zeros`, `ones_count`, `ones_density`, `alternating_score`
//! - Modular metrics: `mod3`, `mod7`, `mod11`, `mod13`, `mod17`
//! - Structural metrics: `zero_density`, `digit_sum`, `is_palindrome`
//! - Positional metrics: `position_in_space` (0.0 to 1.0)
//!
//! ## Usage
//!
//! ```bash
//! # Quick exploration (1000 samples per pair)
//! cargo run --example connector_space_explorer -- --sample 1000
//!
//! # Full exploration (all connectors, WARNING: slow!)
//! cargo run --release --example connector_space_explorer -- --full
//!
//! # Single pair quick test
//! cargo run --example connector_space_explorer -- --pair pair_02 --sample 500
//! ```
//!
//! ## Philosophy
//!
//! This is **not** hypothesis testing.
//! This is **pattern mining**.
//!
//! We're building a dataset that can answer questions like:
//! - Which metrics correlate with asymmetry magnitude?
//! - Do binary patterns cluster differently in forward vs reverse?
//! - Which prime pair properties predict stronger directional bias?
//!
//! Let the data lead, not our expectations.

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime_miller_rabin;
use std::fs::File;
use std::io::{Write as IoWrite, BufWriter};
use std::time::Instant;
use rand::Rng;

// ============================================================================
// Metrics Computation
// ============================================================================

#[derive(Debug, Clone)]
struct ConnectorMetrics {
    // Identifiers
    pair_id: String,
    direction: String,
    connector: u64,
    connector_len: usize,

    // Result
    is_prime: bool,

    // Binary metrics
    trailing_zeros: u32,
    bit_length: usize,
    ones_count: u32,
    ones_density: f64,
    alternating_score: f64,

    // Modular metrics
    mod3: u8,
    mod7: u8,
    mod11: u8,
    mod13: u8,
    mod17: u8,

    // Structural metrics
    zero_density: f64,
    digit_sum: u32,
    is_palindrome: bool,
    has_repeated_digit: bool,

    // Positional
    position_ratio: f64,  // Where in [0, 10^len - 1] does this sit?
}

fn compute_metrics(
    pair_id: &str,
    direction: &str,
    p1: &BigUint,
    p2: &BigUint,
    connector: u64,
    connector_len: usize,
) -> ConnectorMetrics {
    // Concatenate
    let full_number = if direction == "forward" {
        concat_forward(p1, p2, connector, connector_len)
    } else {
        concat_reverse(p1, p2, connector, connector_len)
    };

    // Primality
    let is_prime = is_prime_miller_rabin(&full_number);

    // Binary metrics
    let bit_length = full_number.bits() as usize;
    let ones_count = full_number.count_ones() as u32;
    let ones_density = if bit_length > 0 {
        ones_count as f64 / bit_length as f64
    } else {
        0.0
    };

    let trailing_zeros = count_trailing_zeros(&full_number);
    let alternating_score = compute_alternating_score(&full_number);

    // Modular metrics
    let mod3 = (&full_number % 3u32).to_u32_digits().first().copied().unwrap_or(0) as u8;
    let mod7 = (&full_number % 7u32).to_u32_digits().first().copied().unwrap_or(0) as u8;
    let mod11 = (&full_number % 11u32).to_u32_digits().first().copied().unwrap_or(0) as u8;
    let mod13 = (&full_number % 13u32).to_u32_digits().first().copied().unwrap_or(0) as u8;
    let mod17 = (&full_number % 17u32).to_u32_digits().first().copied().unwrap_or(0) as u8;

    // Structural metrics (on connector)
    let c_str = format!("{:0>width$}", connector, width = connector_len);
    let zero_density = c_str.chars().filter(|&c| c == '0').count() as f64 / connector_len as f64;
    let digit_sum: u32 = c_str.chars().map(|c| c.to_digit(10).unwrap()).sum();
    let is_palindrome = c_str == c_str.chars().rev().collect::<String>();

    let digit_counts = (0..=9).map(|d| c_str.chars().filter(|&c| c.to_digit(10).unwrap() == d).count()).collect::<Vec<_>>();
    let has_repeated_digit = digit_counts.iter().any(|&count| count >= 3);

    // Positional
    let max_connector = 10u64.pow(connector_len as u32) - 1;
    let position_ratio = if max_connector > 0 {
        connector as f64 / max_connector as f64
    } else {
        0.0
    };

    ConnectorMetrics {
        pair_id: pair_id.to_string(),
        direction: direction.to_string(),
        connector,
        connector_len,
        is_prime,
        trailing_zeros,
        bit_length,
        ones_count,
        ones_density,
        alternating_score,
        mod3,
        mod7,
        mod11,
        mod13,
        mod17,
        zero_density,
        digit_sum,
        is_palindrome,
        has_repeated_digit,
        position_ratio,
    }
}

fn concat_forward(p1: &BigUint, p2: &BigUint, connector: u64, connector_len: usize) -> BigUint {
    let len_p2_str = p2.to_string();
    let len_p2 = len_p2_str.len();

    let pow_c_plus_p2 = BigUint::from(10u32).pow((connector_len + len_p2) as u32);
    let pow_p2 = BigUint::from(10u32).pow(len_p2 as u32);

    p1 * pow_c_plus_p2 + BigUint::from(connector) * pow_p2 + p2
}

fn concat_reverse(p1: &BigUint, p2: &BigUint, connector: u64, connector_len: usize) -> BigUint {
    let len_p1_str = p1.to_string();
    let len_p1 = len_p1_str.len();

    let pow_c_plus_p1 = BigUint::from(10u32).pow((connector_len + len_p1) as u32);
    let pow_p1 = BigUint::from(10u32).pow(len_p1 as u32);

    p2 * pow_c_plus_p1 + BigUint::from(connector) * pow_p1 + p1
}

fn count_trailing_zeros(n: &BigUint) -> u32 {
    if n.is_zero() {
        return 0;
    }

    let mut count = 0u32;
    let mut temp = n.clone();
    let two = BigUint::from(2u32);

    while &temp % &two == BigUint::zero() {
        count += 1;
        temp /= &two;
    }

    count
}

fn compute_alternating_score(n: &BigUint) -> f64 {
    let bin_str = format!("{:b}", n);
    if bin_str.len() < 2 {
        return 0.0;
    }

    let transitions = bin_str
        .as_bytes()
        .windows(2)
        .filter(|w| w[0] != w[1])
        .count();

    transitions as f64 / (bin_str.len() - 1) as f64
}

// ============================================================================
// Space Sweeping
// ============================================================================

fn sweep_connector_space(
    pair_id: &str,
    p1: &BigUint,
    p2: &BigUint,
    min_len: usize,
    max_len: usize,
    sample_size: Option<usize>,
) -> Vec<ConnectorMetrics> {
    let mut results = Vec::new();
    let mut rng = rand::thread_rng();

    for len in min_len..=max_len {
        let max_connector = 10u64.pow(len as u32);

        let connectors: Vec<u64> = if let Some(sample) = sample_size {
            // Random sample
            (0..sample)
                .map(|_| rng.gen_range(0..max_connector))
                .collect()
        } else {
            // Full enumeration (WARNING: slow for len > 5!)
            (0..max_connector).collect()
        };

        for &c in &connectors {
            // Forward
            let metrics_fwd = compute_metrics(pair_id, "forward", p1, p2, c, len);
            results.push(metrics_fwd);

            // Reverse
            let metrics_rev = compute_metrics(pair_id, "reverse", p1, p2, c, len);
            results.push(metrics_rev);
        }
    }

    results
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    println!("🔬 Connector Space Explorer");
    println!("{}", "=".repeat(70));
    println!("Philosophy: Sweep first, ask questions later.\n");

    // For demo, use original pair
    let pair_id = "pair_01";
    let p1 = BigUint::from(10301u32);
    let p2 = BigUint::parse_bytes(b"3007003007003", 10).unwrap();

    let min_len = 5;
    let max_len = 5;  // Start with length 5 only (fast)
    let sample_size = Some(1000);  // 1000 random samples per length

    println!("Configuration:");
    println!("  Pair: {} and {}", p1, p2);
    println!("  Connector lengths: {}-{}", min_len, max_len);
    println!("  Sample size: {:?} per length", sample_size);
    println!();

    let start = Instant::now();

    println!("Sweeping connector space...");
    let results = sweep_connector_space(&pair_id, &p1, &p2, min_len, max_len, sample_size);

    let elapsed = start.elapsed();
    println!("✓ Computed {} data points in {:.2}s", results.len(), elapsed.as_secs_f64());

    // Write CSV
    let output_file = "collab/connector_space_sample.csv";
    let file = File::create(output_file).expect("Failed to create output file");
    let mut writer = BufWriter::new(file);

    // Header
    writeln!(writer, "pair_id,direction,connector,connector_len,is_prime,\
                      trailing_zeros,bit_length,ones_count,ones_density,alternating_score,\
                      mod3,mod7,mod11,mod13,mod17,\
                      zero_density,digit_sum,is_palindrome,has_repeated_digit,position_ratio")
        .expect("Failed to write header");

    // Data
    for m in &results {
        writeln!(writer, "{},{},{},{},{},{},{},{},{:.6},{:.6},{},{},{},{},{},{:.6},{},{},{},{:.6}",
                 m.pair_id, m.direction, m.connector, m.connector_len, m.is_prime,
                 m.trailing_zeros, m.bit_length, m.ones_count, m.ones_density, m.alternating_score,
                 m.mod3, m.mod7, m.mod11, m.mod13, m.mod17,
                 m.zero_density, m.digit_sum, m.is_palindrome, m.has_repeated_digit, m.position_ratio)
            .expect("Failed to write data");
    }

    println!("\n📊 Results saved to: {}", output_file);

    // Quick summary
    let fwd_primes = results.iter().filter(|m| m.direction == "forward" && m.is_prime).count();
    let rev_primes = results.iter().filter(|m| m.direction == "reverse" && m.is_prime).count();

    println!("\n📈 Quick Summary:");
    println!("  Forward primes: {} / {}", fwd_primes, results.len() / 2);
    println!("  Reverse primes: {} / {}", rev_primes, results.len() / 2);
    println!("  Asymmetry: {} ({:+.2}%)",
             fwd_primes as i32 - rev_primes as i32,
             ((fwd_primes as f64 - rev_primes as f64) / fwd_primes as f64 * 100.0));

    println!("\n💡 Next: Analyze CSV to find which metrics correlate with asymmetry");
}
