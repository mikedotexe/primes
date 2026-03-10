// Binary structure analysis for prime connectors
// Analyzes binary zero-padding patterns that may correlate with directional asymmetry

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime_miller_rabin;

/// Binary structure metrics using efficient std functions
#[derive(Debug, Clone)]
struct BinaryMetrics {
    // Core value
    value: BigUint,

    // Efficient binary analysis (from std)
    trailing_zeros: u64,      // count_zeros() equivalent - divisibility by 2^n
    leading_zeros: u64,       // Most significant bit position
    count_ones: u64,          // Popcount - Hamming weight

    // Derived metrics
    bit_length: u64,          // Total bits needed
    ones_density: f64,        // count_ones / bit_length

    // Power-of-2 alignment
    is_power_of_2: bool,
    nearest_power_of_2: u64,
    power_of_2_distance: i64,

    // Bit pattern analysis
    alternating_score: f64,   // How alternating are the bits?
    run_length_avg: f64,      // Average length of consecutive 0s or 1s
}

impl BinaryMetrics {
    fn analyze(value: &BigUint) -> Self {
        let bytes = value.to_bytes_be();
        let bit_length = value.bits() as u64;

        // Trailing zeros (rightmost): how many powers of 2 divide this?
        let trailing_zeros = Self::count_trailing_zeros(value);

        // Leading zeros (leftmost): how far from next power of 2?
        let leading_zeros = if bit_length > 0 {
            // In a 64-bit representation, how many leading zeros?
            // We approximate using the bit_length
            64u64.saturating_sub(bit_length)
        } else {
            64
        };

        // Count ones (popcount)
        let count_ones = Self::count_ones(&bytes);

        // Ones density
        let ones_density = if bit_length > 0 {
            count_ones as f64 / bit_length as f64
        } else {
            0.0
        };

        // Power of 2 analysis
        let is_power_of_2 = value.count_ones() == 1 && !value.is_zero();
        let nearest_power_of_2 = if bit_length > 0 { bit_length } else { 0 };
        let actual_power_of_2 = BigUint::from(2u32).pow(nearest_power_of_2 as u32);
        let power_of_2_distance = if value > &actual_power_of_2 {
            (value - &actual_power_of_2).to_string().len() as i64
        } else {
            -(((&actual_power_of_2 - value).to_string().len()) as i64)
        };

        // Pattern analysis
        let alternating_score = Self::compute_alternating_score(&bytes);
        let run_length_avg = Self::compute_run_length_avg(&bytes);

        Self {
            value: value.clone(),
            trailing_zeros,
            leading_zeros,
            count_ones,
            bit_length,
            ones_density,
            is_power_of_2,
            nearest_power_of_2,
            power_of_2_distance,
            alternating_score,
            run_length_avg,
        }
    }

    fn count_trailing_zeros(value: &BigUint) -> u64 {
        if value.is_zero() {
            return 0;
        }

        let bytes = value.to_bytes_le(); // Little-endian for trailing
        let mut count = 0u64;

        for byte in bytes.iter() {
            if *byte == 0 {
                count += 8;
            } else {
                count += byte.trailing_zeros() as u64;
                break;
            }
        }

        count
    }

    fn count_ones(bytes: &[u8]) -> u64 {
        bytes.iter().map(|b| b.count_ones() as u64).sum()
    }

    fn compute_alternating_score(bytes: &[u8]) -> f64 {
        if bytes.is_empty() {
            return 0.0;
        }

        let mut transitions = 0u64;
        let mut total_bits = 0u64;

        for byte in bytes.iter() {
            for i in 0..7 {
                let bit1 = (byte >> i) & 1;
                let bit2 = (byte >> (i + 1)) & 1;
                if bit1 != bit2 {
                    transitions += 1;
                }
                total_bits += 1;
            }
        }

        if total_bits > 0 {
            transitions as f64 / total_bits as f64
        } else {
            0.0
        }
    }

    fn compute_run_length_avg(bytes: &[u8]) -> f64 {
        if bytes.is_empty() {
            return 0.0;
        }

        let mut runs = Vec::new();
        let mut current_run = 1u64;
        let mut last_bit = bytes[0] & 1;

        for byte in bytes.iter() {
            for i in 0..8 {
                let bit = (byte >> i) & 1;
                if bit == last_bit {
                    current_run += 1;
                } else {
                    runs.push(current_run);
                    current_run = 1;
                    last_bit = bit;
                }
            }
        }
        runs.push(current_run);

        if !runs.is_empty() {
            runs.iter().sum::<u64>() as f64 / runs.len() as f64
        } else {
            0.0
        }
    }
}

/// Aggregate binary statistics for a set of connectors
#[derive(Debug)]
struct BinaryStats {
    count: usize,

    // Trailing zeros distribution
    trailing_zeros_mean: f64,
    trailing_zeros_median: f64,
    trailing_zeros_mode: u64,

    // Ones density
    ones_density_mean: f64,
    ones_density_variance: f64,

    // Power-of-2 alignment
    power_of_2_aligned_count: usize,  // trailing_zeros > 0
    highly_aligned_count: usize,       // trailing_zeros >= 4

    // Pattern metrics
    alternating_score_mean: f64,
    run_length_avg_mean: f64,

    // Percentiles for distribution analysis
    trailing_zeros_p25: u64,
    trailing_zeros_p75: u64,
    trailing_zeros_p95: u64,
}

impl BinaryStats {
    fn from_values(values: &[BigUint]) -> Self {
        let metrics: Vec<BinaryMetrics> = values
            .iter()
            .map(|v| BinaryMetrics::analyze(v))
            .collect();

        let count = metrics.len();

        // Trailing zeros statistics
        let mut trailing_zeros: Vec<u64> = metrics.iter().map(|m| m.trailing_zeros).collect();
        trailing_zeros.sort_unstable();

        let trailing_zeros_mean = trailing_zeros.iter().sum::<u64>() as f64 / count as f64;
        let trailing_zeros_median = if count > 0 {
            trailing_zeros[count / 2]
        } else {
            0
        };

        // Mode (most common value)
        let trailing_zeros_mode = Self::compute_mode(&trailing_zeros);

        // Percentiles
        let trailing_zeros_p25 = if count > 0 { trailing_zeros[count / 4] } else { 0 };
        let trailing_zeros_p75 = if count > 0 { trailing_zeros[3 * count / 4] } else { 0 };
        let trailing_zeros_p95 = if count > 0 { trailing_zeros[95 * count / 100] } else { 0 };

        // Ones density
        let ones_densities: Vec<f64> = metrics.iter().map(|m| m.ones_density).collect();
        let ones_density_mean = ones_densities.iter().sum::<f64>() / count as f64;
        let ones_density_variance = ones_densities.iter()
            .map(|d| (d - ones_density_mean).powi(2))
            .sum::<f64>() / count as f64;

        // Alignment counts
        let power_of_2_aligned_count = metrics.iter().filter(|m| m.trailing_zeros > 0).count();
        let highly_aligned_count = metrics.iter().filter(|m| m.trailing_zeros >= 4).count();

        // Pattern metrics
        let alternating_score_mean = metrics.iter().map(|m| m.alternating_score).sum::<f64>() / count as f64;
        let run_length_avg_mean = metrics.iter().map(|m| m.run_length_avg).sum::<f64>() / count as f64;

        Self {
            count,
            trailing_zeros_mean,
            trailing_zeros_median: trailing_zeros_median as f64,
            trailing_zeros_mode,
            ones_density_mean,
            ones_density_variance,
            power_of_2_aligned_count,
            highly_aligned_count,
            alternating_score_mean,
            run_length_avg_mean,
            trailing_zeros_p25,
            trailing_zeros_p75,
            trailing_zeros_p95,
        }
    }

    fn compute_mode(values: &[u64]) -> u64 {
        if values.is_empty() {
            return 0;
        }

        let mut counts = std::collections::HashMap::new();
        for &v in values {
            *counts.entry(v).or_insert(0) += 1;
        }

        counts.into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(value, _)| value)
            .unwrap_or(0)
    }
}

fn main() {
    println!("🔬 Binary Structure Analysis for Prime Connectors");
    println!("===================================================\n");

    let p1 = BigUint::from(10301u32);
    let p2 = BigUint::from(3007003007003u64);

    println!("Analyzing prime pair:");
    println!("  p1 = {}", p1);
    println!("  p2 = {}", p2);
    println!();

    // Analyze both primes in binary
    println!("=== PRIME BINARY STRUCTURE ===\n");

    let p1_metrics = BinaryMetrics::analyze(&p1);
    println!("p1 = {} (decimal)", p1);
    println!("   = {:b} (binary)", p1);
    println!("   Trailing zeros: {} (divisible by 2^{})", p1_metrics.trailing_zeros, p1_metrics.trailing_zeros);
    println!("   Bit length: {}", p1_metrics.bit_length);
    println!("   Ones: {} ({:.2}% density)", p1_metrics.count_ones, p1_metrics.ones_density * 100.0);
    println!("   Alternating score: {:.4}", p1_metrics.alternating_score);
    println!();

    let p2_metrics = BinaryMetrics::analyze(&p2);
    println!("p2 = {} (decimal)", p2);
    println!("   = {:b} (binary)", p2);
    println!("   Trailing zeros: {} (divisible by 2^{})", p2_metrics.trailing_zeros, p2_metrics.trailing_zeros);
    println!("   Bit length: {}", p2_metrics.bit_length);
    println!("   Ones: {} ({:.2}% density)", p2_metrics.count_ones, p2_metrics.ones_density * 100.0);
    println!("   Alternating score: {:.4}", p2_metrics.alternating_score);
    println!();

    // Sample connector analysis
    println!("=== SAMPLE CONNECTOR ANALYSIS ===\n");

    // Generate sample connectors (lengths 5-7)
    println!("Generating sample of 1000 random connectors...");
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let mut connectors = Vec::new();
    for _ in 0..1000 {
        let len = rng.gen_range(5..=7);
        let max = 10u64.pow(len);
        let c = rng.gen_range(0..max);
        connectors.push(BigUint::from(c));
    }

    let stats = BinaryStats::from_values(&connectors);

    println!("\nBinary Statistics for {} random connectors:", stats.count);
    println!("  Trailing zeros:");
    println!("    Mean: {:.2}", stats.trailing_zeros_mean);
    println!("    Median: {}", stats.trailing_zeros_median);
    println!("    Mode: {}", stats.trailing_zeros_mode);
    println!("    25th percentile: {}", stats.trailing_zeros_p25);
    println!("    75th percentile: {}", stats.trailing_zeros_p75);
    println!("    95th percentile: {}", stats.trailing_zeros_p95);
    println!();
    println!("  Power-of-2 alignment:");
    println!("    Any alignment (tz>0): {} ({:.1}%)",
             stats.power_of_2_aligned_count,
             stats.power_of_2_aligned_count as f64 / stats.count as f64 * 100.0);
    println!("    Highly aligned (tz>=4): {} ({:.1}%)",
             stats.highly_aligned_count,
             stats.highly_aligned_count as f64 / stats.count as f64 * 100.0);
    println!();
    println!("  Bit density:");
    println!("    Ones density mean: {:.4}", stats.ones_density_mean);
    println!("    Ones density variance: {:.6}", stats.ones_density_variance);
    println!();
    println!("  Bit patterns:");
    println!("    Alternating score: {:.4}", stats.alternating_score_mean);
    println!("    Avg run length: {:.2}", stats.run_length_avg_mean);
    println!();

    // Forward concatenation analysis
    println!("=== FORWARD CONCATENATION ANALYSIS ===\n");
    println!("Forward: {} → [connector] → {}", p1, p2);
    println!();

    // Sample a few forward concatenations
    let sample_connectors = vec![
        BigUint::from(6u32),
        BigUint::from(16u32),
        BigUint::from(1024u32),
        BigUint::from(5000u32),
    ];

    for (i, c) in sample_connectors.iter().enumerate() {
        // Forward: p1 * 10^(k+13) + c * 10^13 + p2
        let k = c.to_string().len() as u32;
        let full = &p1 * BigUint::from(10u32).pow(k + 13)
                 + c * BigUint::from(10u32).pow(13)
                 + &p2;

        let metrics = BinaryMetrics::analyze(&full);
        let is_prime = is_prime_miller_rabin(&full);

        println!("Connector #{}: {}", i+1, c);
        println!("  Full number: {} digits", full.to_string().len());
        println!("  Trailing zeros: {}", metrics.trailing_zeros);
        println!("  Ones density: {:.4}", metrics.ones_density);
        println!("  Alternating score: {:.4}", metrics.alternating_score);
        println!("  Is prime: {}", if is_prime { "YES ✓" } else { "NO" });
        println!();
    }

    println!("=== KEY QUESTIONS FOR ANALYSIS ===\n");
    println!("1. Do forward primes have different trailing_zeros distribution than reverse?");
    println!("2. Is there correlation between trailing_zeros and primality?");
    println!("3. Does ones_density correlate with the ~2% asymmetry?");
    println!("4. Are highly aligned numbers (tz>=4) more/less likely to be prime?");
    println!();
    println!("To answer these, run full connector scans with binary metrics tracking!");
}
