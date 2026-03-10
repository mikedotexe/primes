//! Prime Linter: Batch Modular Analysis Tool
//!
//! Analyzes a list of numbers (candidates or primes) and generates
//! a spectral fingerprint report comparing against baselines.
//!
//! Usage:
//!   cargo run --release --example prime_lint -- --input candidates.txt --output report.json

use clap::Parser;
use num_bigint::BigUint;
use primes::fingerprint::{
    signature::PrimeConstructorSignature,
    export::export_ndjson,
};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "prime-lint")]
#[command(about = "Analyze spectral fingerprints of number lists", long_about = None)]
struct Args {
    /// Input file with numbers (one per line)
    #[arg(short, long)]
    input: String,

    /// Output file for analysis (JSON)
    #[arg(short, long)]
    output: String,

    /// Label for this dataset
    #[arg(short, long, default_value = "unlabeled")]
    label: String,

    /// Compare against baseline (optional JSON file)
    #[arg(short, long)]
    baseline: Option<String>,
}

fn main() {
    let args = Args::parse();

    println!("🔍 Prime Linter");
    println!("===============\n");
    println!("Input: {}", args.input);
    println!("Output: {}", args.output);
    println!("Label: {}\n", args.label);

    // Load numbers
    print!("Loading numbers from {}... ", args.input);
    let numbers = load_numbers(&args.input).expect("Failed to load numbers");
    println!("loaded {} numbers", numbers.len());

    if numbers.is_empty() {
        eprintln!("❌ No numbers found in input file");
        return;
    }

    // Compute signature
    print!("Computing spectral fingerprint... ");
    let signature = PrimeConstructorSignature::from_numbers(args.label.clone(), &numbers);
    println!("done");

    // Print summary
    println!("\n" + "=".repeat(80));
    println!("SPECTRAL FINGERPRINT SUMMARY");
    println!("=".repeat(80));
    println!("\nDataset: {}", signature.label);
    println!("Sample size: {}", signature.sample_size);
    println!("\nStructural Features:");
    println!("  Zero fraction: {:.4}", signature.features.zero_fraction);
    println!("  Digit entropy: {:.4}", signature.features.digit_entropy);
    println!("  Palindrome rate: {:.4}", signature.features.palindrome_rate);
    println!("  Mean digit count: {:.2}", signature.features.mean_digit_count);
    println!("  Std digit count: {:.2}", signature.features.var_digit_count.sqrt());
    println!("  Zero-three only: {:.4}", signature.features.zero_three_only_rate);
    println!("  Zero-six only: {:.4}", signature.features.zero_six_only_rate);

    println!("\nDigit Distribution:");
    for (digit, prob) in signature.features.digit_distribution.iter().enumerate() {
        let bar_len = (prob * 50.0) as usize;
        let bar = "█".repeat(bar_len);
        println!("  {}: {:.4} {}", digit, prob, bar);
    }

    println!("\nModular Profile (Mod 3):");
    for (residue, prob) in signature.modular_profile.mod3.iter().enumerate() {
        let bar_len = (prob * 50.0) as usize;
        let bar = "█".repeat(bar_len);
        println!("  r={}: {:.4} {}", residue, prob, bar);
    }

    println!("\nModular Profile (Mod 7):");
    for (residue, prob) in signature.modular_profile.mod7.iter().enumerate() {
        let bar_len = (prob * 50.0) as usize;
        let bar = "█".repeat(bar_len);
        println!("  r={}: {:.4} {}", residue, prob, bar);
    }

    println!("\nGap Statistics:");
    for modulus in [3u32, 7, 11, 13, 17, 19] {
        if let Some(stats) = signature.gap_stats.get(&modulus) {
            println!("  Mod {}: mean={:.2}, var={:.2}, small_excess={:.3}, large_excess={:.3}",
                     modulus, stats.mean_gap, stats.var_gap, stats.small_gap_excess, stats.large_gap_excess);
        }
    }

    // Compare to baseline if provided
    if let Some(baseline_path) = args.baseline {
        print!("\nLoading baseline from {}... ", baseline_path);
        if let Ok(baseline) = load_baseline(&baseline_path) {
            println!("done");
            let weirdness = signature.weirdness_score(&baseline);
            println!("\n" + "=".repeat(80));
            println!("BASELINE COMPARISON");
            println!("=".repeat(80));
            println!("\nWeirdness score: {:.4}", weirdness);

            if weirdness < 1.0 {
                println!("✅ Very similar to baseline (normal)");
            } else if weirdness < 5.0 {
                println!("⚠️  Moderately different from baseline");
            } else {
                println!("🚨 Highly anomalous compared to baseline!");
            }
        } else {
            eprintln!("Failed to load baseline");
        }
    }

    // Export
    print!("\nExporting to {}... ", args.output);
    export_ndjson(&[signature], &args.output).expect("Failed to export");
    println!("done");

    println!("\n✨ Lint Complete!");
    println!("\nNext steps:");
    println!("  - Use this signature as a baseline: --baseline {}", args.output);
    println!("  - Compare multiple datasets: merge NDJSON files");
    println!("  - Visualize: python plot_fingerprints.py");
}

/// Load numbers from text file (one per line)
fn load_numbers<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<BigUint>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut numbers = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue; // Skip empty lines and comments
        }

        if let Ok(num) = trimmed.parse::<BigUint>() {
            numbers.push(num);
        } else {
            eprintln!("Warning: Could not parse '{}'", trimmed);
        }
    }

    Ok(numbers)
}

/// Load baseline signature from NDJSON file
fn load_baseline<P: AsRef<Path>>(path: P) -> std::io::Result<PrimeConstructorSignature> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read first line (assuming single signature)
    if let Some(Ok(line)) = reader.lines().next() {
        let sig: PrimeConstructorSignature = serde_json::from_str(&line)?;
        Ok(sig)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "No data in baseline file",
        ))
    }
}
