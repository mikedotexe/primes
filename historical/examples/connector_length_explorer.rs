//! Connector Length Explorer
//!
//! Focus: Sample connector lengths 7-11 to see how prime density changes
//!
//! Key observation from full scans:
//! - Length 5:  5,068 primes (0.5%)
//! - Length 6: 47,195 primes (9.4%)
//! - Length 7: 452,380 primes (89.6%) ← DOMINANT
//!
//! Questions:
//! 1. Does density continue increasing past length 7?
//! 2. Is there a peak length for maximum density?
//! 3. Do longer connectors show stronger/weaker asymmetry?
//!
//! Sample strategy: Random sampling (can't test all 10^11 length-11 connectors!)

use num_bigint::BigUint;
use primes::is_prime_miller_rabin;
use std::fs::File;
use std::io::{Write as IoWrite, BufWriter};
use rand::Rng;

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

#[derive(Debug)]
struct LengthDensityResult {
    length: usize,
    sample_size: usize,
    forward_primes: usize,
    reverse_primes: usize,
    forward_density: f64,
    reverse_density: f64,
    asymmetry_pct: f64,
}

fn explore_length(
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
            // For small lengths, use u64 random
            let c = rng.gen_range(0..10u64.pow(length as u32));
            BigUint::from(c)
        } else {
            // For large lengths, sample random BigUint
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
    }
}

fn main() {
    println!("📏 Connector Length Density Explorer");
    println!("{}", "=".repeat(70));
    println!("Focus: How does prime density change with connector length?\n");

    let p1 = BigUint::from(10301u32);
    let p2 = BigUint::parse_bytes(b"3007003007003", 10).unwrap();

    println!("Prime pair: {} and {}", p1, p2);
    println!();

    // Sample sizes (larger for smaller lengths where we can afford it)
    let configs = vec![
        (5, 10000),   // Can test all 100K, so large sample
        (6, 10000),   // Can test all 1M, large sample
        (7, 10000),   // 10M total, good sample
        (8, 5000),    // 100M total, moderate sample
        (9, 5000),    // 1B total, moderate sample
        (10, 2000),   // 10B total, smaller sample
        (11, 2000),   // 100B total, smaller sample
    ];

    let mut results = Vec::new();

    for (length, sample_size) in configs {
        print!("Exploring length {}... (n={}) ", length, sample_size);
        std::io::stdout().flush().unwrap();

        let result = explore_length(&p1, &p2, length, sample_size);
        println!("✓ fwd={:.2}% rev={:.2}% Δ={:+.1}%",
                 result.forward_density,
                 result.reverse_density,
                 result.asymmetry_pct);

        results.push(result);
    }

    // Save results
    let output_file = "collab/length_density_analysis.csv";
    let file = File::create(output_file).expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "length,sample_size,forward_primes,reverse_primes,forward_density,reverse_density,asymmetry_pct")
        .expect("Failed to write header");

    for r in &results {
        writeln!(writer, "{},{},{},{},{:.6},{:.6},{:.6}",
                 r.length, r.sample_size, r.forward_primes, r.reverse_primes,
                 r.forward_density, r.reverse_density, r.asymmetry_pct)
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

    // Asymmetry trend
    println!("\n🔄 ASYMMETRY TREND");
    let avg_asymmetry: f64 = results.iter().map(|r| r.asymmetry_pct).sum::<f64>() / results.len() as f64;
    println!("  Average asymmetry: {:+.2}%", avg_asymmetry);
    println!("  Range: {:+.2}% to {:+.2}%",
             results.iter().map(|r| r.asymmetry_pct).fold(f64::INFINITY, f64::min),
             results.iter().map(|r| r.asymmetry_pct).fold(f64::NEG_INFINITY, f64::max));

    println!("\n💡 INTERPRETATION");
    println!("{}", "-".repeat(70));
    if max_fwd.length == 7 {
        println!("✓ Length 7 is indeed the peak density (confirmed from samples)");
    } else {
        println!("⚠  Peak density appears at length {}, not 7 (unexpected!)", max_fwd.length);
    }

    if avg_asymmetry.abs() > 1.0 {
        println!("✓ Asymmetry persists across lengths (avg {:+.1}%)", avg_asymmetry);
    } else {
        println!("⚠  Asymmetry is weak/inconsistent across lengths");
    }

    let later_lengths = &results[4..];  // Lengths 9-11
    let later_avg = later_lengths.iter().map(|r| r.forward_density).sum::<f64>() / later_lengths.len() as f64;

    if later_avg > results[0].forward_density {
        println!("📈 Density INCREASES for longer connectors (interesting!)");
    } else {
        println!("📉 Density DECREASES for longer connectors (as expected from PNT)");
    }
}
