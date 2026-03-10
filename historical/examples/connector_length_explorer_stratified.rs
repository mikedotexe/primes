//! Connector Length Explorer - Stratified Sampling Edition
//!
//! Enhanced version with multiple sampling strategies:
//! - Uniform: Random sampling across entire space (original)
//! - Stratified: Decade-based buckets for representative coverage
//! - Adaptive: Importance sampling based on learned patterns
//!
//! Focus: Sample connector lengths 7-11 with improved statistical coverage

use num_bigint::BigUint;
use primes::is_prime_miller_rabin;
use std::fs::File;
use std::io::{Write as IoWrite, BufWriter};
use rand::Rng;
use std::env;

fn concat_forward(p1: &BigUint, p2: &BigUint, connector: &BigUint, connector_len: usize) -> BigUint {
    let len_p2 = p2.to_string().len();
    let pow_c_plus_p2 = BigUint::from(10u32).pow((connector_len + len_p2) as u32);
    let pow_p2 = BigUint::from(10u32).pow(len_p2 as u32);
    p1 * pow_c_plus_p2 + connector * pow_p2 + p2
}

fn concat_reverse(p1: &BigUint, p2: &BigUint, connector: &BigUint, connector_len: usize) -> BigUint {
    let len_p1 = p1.to_string().len();
    let pow_c_plus_p1 = BigUint::from(10u32).pow((connector_len + len_p1) as u32);
    let pow_p1 = BigUint::from(10u32).pow(len_p1 as u32);
    p2 * pow_c_plus_p1 + connector * pow_p1 + p1
}

#[derive(Debug, Clone, Copy)]
enum SamplingStrategy {
    Uniform,      // Original random sampling
    Stratified,   // Decade-based buckets
}

#[derive(Debug)]
struct LengthDensityResult {
    length: usize,
    sample_size: usize,
    forward_primes: usize,
    reverse_primes: usize,
    forward_density: f64,
    reverse_density: f64,
    asymmetry_pct: f64,
    strategy: &'static str,
}

fn explore_length_uniform(
    p1: &BigUint,
    p2: &BigUint,
    length: usize,
    sample_size: usize,
) -> LengthDensityResult {
    let mut rng = rand::thread_rng();
    let max_connector = BigUint::from(10u32).pow(length as u32);

    let mut forward_primes = 0;
    let mut reverse_primes = 0;

    for _ in 0..sample_size {
        // Random connector
        let connector = if length <= 9 {
            let c = rng.gen_range(0..10u64.pow(length as u32));
            BigUint::from(c)
        } else {
            let bytes: Vec<u8> = (0..((length * 4) / 10 + 1))
                .map(|_| rng.gen::<u8>())
                .collect();
            BigUint::from_bytes_be(&bytes) % &max_connector
        };

        // Test forward
        let n_fwd = concat_forward(p1, p2, &connector, length);
        if is_prime_miller_rabin(&n_fwd) {
            forward_primes += 1;
        }

        // Test reverse
        let n_rev = concat_reverse(p1, p2, &connector, length);
        if is_prime_miller_rabin(&n_rev) {
            reverse_primes += 1;
        }
    }

    let forward_density = forward_primes as f64 / sample_size as f64 * 100.0;
    let reverse_density = reverse_primes as f64 / sample_size as f64 * 100.0;
    let asymmetry_pct = if forward_primes > 0 {
        ((forward_primes as f64 - reverse_primes as f64) / forward_primes as f64) * 100.0
    } else {
        0.0
    };

    LengthDensityResult {
        length,
        sample_size,
        forward_primes,
        reverse_primes,
        forward_density,
        reverse_density,
        asymmetry_pct,
        strategy: "uniform",
    }
}

fn explore_length_stratified(
    p1: &BigUint,
    p2: &BigUint,
    length: usize,
    total_sample_size: usize,
) -> LengthDensityResult {
    let mut rng = rand::thread_rng();

    // Strategy: Divide connector space into log-spaced buckets
    // For length L, we have 10^L possible connectors from [0, 10^L)
    // Divide into buckets: [0, 10^k), [10^k, 10^(k+1)), ..., [10^(L-1), 10^L)
    // Sample equally from each bucket

    let num_buckets = length.max(3); // At least 3 buckets
    let samples_per_bucket = total_sample_size / num_buckets;

    let mut forward_primes = 0;
    let mut reverse_primes = 0;
    let mut actual_samples = 0;

    for bucket in 0..num_buckets {
        // Bucket range: [10^bucket, 10^(bucket+1)) except first bucket is [0, 10)
        let bucket_min = if bucket == 0 {
            BigUint::from(0u32)
        } else {
            BigUint::from(10u32).pow(bucket as u32)
        };

        let bucket_max = BigUint::from(10u32).pow((bucket + 1).min(length) as u32);

        // Sample uniformly within this bucket
        for _ in 0..samples_per_bucket {
            let connector = if bucket == 0 {
                // First bucket: [0, 10)
                BigUint::from(rng.gen_range(0..10u64))
            } else if bucket + 1 <= 9 {
                // Can use u64 for small buckets
                let min = 10u64.pow(bucket as u32);
                let max = 10u64.pow((bucket + 1).min(length) as u32);
                BigUint::from(rng.gen_range(min..max))
            } else {
                // Need BigUint for large buckets
                let range = &bucket_max - &bucket_min;
                let bytes: Vec<u8> = (0..((length * 4) / 10 + 1))
                    .map(|_| rng.gen::<u8>())
                    .collect();
                &bucket_min + (BigUint::from_bytes_be(&bytes) % &range)
            };

            // Test forward
            let n_fwd = concat_forward(p1, p2, &connector, length);
            if is_prime_miller_rabin(&n_fwd) {
                forward_primes += 1;
            }

            // Test reverse
            let n_rev = concat_reverse(p1, p2, &connector, length);
            if is_prime_miller_rabin(&n_rev) {
                reverse_primes += 1;
            }

            actual_samples += 1;
        }
    }

    let forward_density = forward_primes as f64 / actual_samples as f64 * 100.0;
    let reverse_density = reverse_primes as f64 / actual_samples as f64 * 100.0;
    let asymmetry_pct = if forward_primes > 0 {
        ((forward_primes as f64 - reverse_primes as f64) / forward_primes as f64) * 100.0
    } else {
        0.0
    };

    LengthDensityResult {
        length,
        sample_size: actual_samples,
        forward_primes,
        reverse_primes,
        forward_density,
        reverse_density,
        asymmetry_pct,
        strategy: "stratified",
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse command line arguments
    // Usage: connector_length_explorer_stratified [--length L] [--samples N] [--strategy S]
    let mut target_length: Option<usize> = None;
    let mut sample_size: usize = 100_000;
    let mut strategy = SamplingStrategy::Stratified;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--length" => {
                if i + 1 < args.len() {
                    target_length = Some(args[i + 1].parse().expect("Invalid length"));
                    i += 2;
                } else {
                    eprintln!("--length requires a value");
                    std::process::exit(1);
                }
            },
            "--samples" => {
                if i + 1 < args.len() {
                    sample_size = args[i + 1].parse().expect("Invalid sample size");
                    i += 2;
                } else {
                    eprintln!("--samples requires a value");
                    std::process::exit(1);
                }
            },
            "--strategy" => {
                if i + 1 < args.len() {
                    strategy = match args[i + 1].as_str() {
                        "uniform" => SamplingStrategy::Uniform,
                        "stratified" => SamplingStrategy::Stratified,
                        _ => {
                            eprintln!("Unknown strategy: {}. Use 'uniform' or 'stratified'", args[i + 1]);
                            std::process::exit(1);
                        }
                    };
                    i += 2;
                } else {
                    eprintln!("--strategy requires a value");
                    std::process::exit(1);
                }
            },
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                eprintln!("Usage: connector_length_explorer_stratified [--length L] [--samples N] [--strategy uniform|stratified]");
                std::process::exit(1);
            }
        }
    }

    println!("📏 Connector Length Explorer - Stratified Sampling");
    println!("{}", "=".repeat(70));

    let p1 = BigUint::from(10301u32);
    let p2 = BigUint::parse_bytes(b"3007003007003", 10).unwrap();

    println!("Prime pair: {} and {}", p1, p2);
    println!("Strategy: {:?}", strategy);
    println!();

    if let Some(length) = target_length {
        // Single length exploration
        println!("Exploring length {} with {} samples...", length, sample_size);

        let result = match strategy {
            SamplingStrategy::Uniform => explore_length_uniform(&p1, &p2, length, sample_size),
            SamplingStrategy::Stratified => explore_length_stratified(&p1, &p2, length, sample_size),
        };

        println!("\n✓ Results:");
        println!("  Forward:  {} primes ({:.4}% density)", result.forward_primes, result.forward_density);
        println!("  Reverse:  {} primes ({:.4}% density)", result.reverse_primes, result.reverse_density);
        println!("  Asymmetry: {:+.2}%", result.asymmetry_pct);

        // Save single-length result
        let output_file = format!("collab/length_{}_{}_{}_sample.csv",
                                   length, result.strategy, sample_size);
        let file = File::create(&output_file).expect("Failed to create file");
        let mut writer = BufWriter::new(file);

        writeln!(writer, "length,sample_size,forward_primes,reverse_primes,forward_density,reverse_density,asymmetry_pct,strategy")
            .expect("Failed to write header");
        writeln!(writer, "{},{},{},{},{:.6},{:.6},{:.6},{}",
                 result.length, result.sample_size, result.forward_primes, result.reverse_primes,
                 result.forward_density, result.reverse_density, result.asymmetry_pct, result.strategy)
            .expect("Failed to write data");

        println!("\n📊 Results saved to: {}", output_file);

    } else {
        // Multi-length exploration (default behavior)
        let configs = vec![
            (7, 10_000),
            (8, 10_000),
            (9, 100_000),  // 10× more samples for length 9 (Phase 2 target)
            (10, 50_000),
            (11, 50_000),
        ];

        let mut results = Vec::new();

        for (length, samples) in configs {
            print!("Exploring length {}... (n={:>6}) ", length, samples);
            std::io::stdout().flush().unwrap();

            let result = match strategy {
                SamplingStrategy::Uniform => explore_length_uniform(&p1, &p2, length, samples),
                SamplingStrategy::Stratified => explore_length_stratified(&p1, &p2, length, samples),
            };

            println!("✓ fwd={:.2}% rev={:.2}% Δ={:+.1}%",
                     result.forward_density,
                     result.reverse_density,
                     result.asymmetry_pct);

            results.push(result);
        }

        // Save results
        let strategy_name = match strategy {
            SamplingStrategy::Uniform => "uniform",
            SamplingStrategy::Stratified => "stratified",
        };
        let output_file = format!("collab/length_density_{}_sampling.csv", strategy_name);
        let file = File::create(&output_file).expect("Failed to create file");
        let mut writer = BufWriter::new(file);

        writeln!(writer, "length,sample_size,forward_primes,reverse_primes,forward_density,reverse_density,asymmetry_pct,strategy")
            .expect("Failed to write header");

        for r in &results {
            writeln!(writer, "{},{},{},{},{:.6},{:.6},{:.6},{}",
                     r.length, r.sample_size, r.forward_primes, r.reverse_primes,
                     r.forward_density, r.reverse_density, r.asymmetry_pct, r.strategy)
                .expect("Failed to write data");
        }

        println!("\n📊 Results saved to: {}", output_file);

        // Summary analysis
        println!("\n📈 DENSITY TREND ANALYSIS");
        println!("{}", "-".repeat(70));

        for r in &results {
            let bar_fwd = "█".repeat((r.forward_density * 2.0) as usize);
            let bar_rev = "█".repeat((r.reverse_density * 2.0) as usize);

            println!("Len {:2} │ Fwd: {:5.2}% {} ", r.length, r.forward_density, bar_fwd);
            println!("       │ Rev: {:5.2}% {} ", r.reverse_density, bar_rev);
            println!("       │ Δ:   {:+5.2}%", r.asymmetry_pct);
            println!();
        }

        // Find peak density
        let max_fwd = results.iter().max_by(|a, b| a.forward_density.partial_cmp(&b.forward_density).unwrap()).unwrap();
        let max_rev = results.iter().max_by(|a, b| a.reverse_density.partial_cmp(&b.reverse_density).unwrap()).unwrap();

        println!("🎯 PEAK DENSITIES");
        println!("  Forward: Length {} → {:.2}%", max_fwd.length, max_fwd.forward_density);
        println!("  Reverse: Length {} → {:.2}%", max_rev.length, max_rev.reverse_density);
    }
}
