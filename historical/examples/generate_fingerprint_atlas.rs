//! Generate Prime Fingerprint Atlas
//!
//! Creates labeled dataset of prime construction signatures for ML classification.
//!
//! This example:
//! 1. Defines 5+ prime construction methods
//! 2. Generates 1000+ primes per method
//! 3. Computes spectral fingerprints
//! 4. Exports to NDJSON and CSV for ML pipeline
//!
//! Usage:
//!   cargo run --release --example generate_fingerprint_atlas -- --output-dir fingerprints/

use clap::Parser;
use num_bigint::BigUint;
use primes::fingerprint::{
    constructors::*,
    signature::PrimeConstructorSignature,
    export::{export_ndjson, export_csv},
};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(name = "fingerprint-atlas")]
#[command(about = "Generate prime construction fingerprint atlas", long_about = None)]
struct Args {
    /// Output directory for fingerprint data
    #[arg(short, long, default_value = "fingerprints")]
    output_dir: String,

    /// Number of prime samples per constructor
    #[arg(short, long, default_value = "1000")]
    samples: usize,

    /// Maximum candidates to test per constructor
    #[arg(short, long, default_value = "100000")]
    max_candidates: usize,
}

fn main() {
    let args = Args::parse();

    println!("🎨 Prime Fingerprint Atlas Generator");
    println!("=====================================\n");
    println!("Output directory: {}", args.output_dir);
    println!("Samples per constructor: {}", args.samples);
    println!("Max candidates: {}\n", args.max_candidates);

    // Create output directory
    std::fs::create_dir_all(&args.output_dir).expect("Failed to create output directory");

    // Define constructors
    let constructors = create_constructors();
    println!("Defined {} constructors:\n", constructors.len());

    let mut all_signatures = Vec::new();

    for (i, constructor) in constructors.iter().enumerate() {
        println!(
            "[{}/{}] Generating fingerprint for: {}",
            i + 1,
            constructors.len(),
            constructor.name()
        );

        let start = Instant::now();

        // Generate primes
        print!("  Finding {} primes... ", args.samples);
        let primes = constructor.generate_primes(args.samples, args.max_candidates);
        println!("found {} in {:.2}s", primes.len(), start.elapsed().as_secs_f64());

        if primes.is_empty() {
            println!("  ⚠️  No primes found, skipping");
            continue;
        }

        // Show a sample prime
        if !primes.is_empty() {
            println!("  Sample: {} ({}d)", primes[0], primes[0].to_string().len());
        }

        // Compute signature
        print!("  Computing spectral fingerprint... ");
        let sig = PrimeConstructorSignature::from_numbers(constructor.name(), &primes);
        println!("done");

        // Print quick stats
        println!("  Stats:");
        println!("    Sample size: {}", sig.sample_size);
        println!(
            "    Zero fraction: {:.3}",
            sig.features.zero_fraction
        );
        println!(
            "    Digit entropy: {:.3}",
            sig.features.digit_entropy
        );
        println!(
            "    Palindrome rate: {:.3}",
            sig.features.palindrome_rate
        );
        println!(
            "    Mean digits: {:.1}",
            sig.features.mean_digit_count
        );

        all_signatures.push(sig);
        println!();
    }

    // Export to NDJSON
    let ndjson_path = format!("{}/fingerprints.ndjson", args.output_dir);
    print!("Exporting to {}... ", ndjson_path);
    export_ndjson(&all_signatures, &ndjson_path).expect("Failed to export NDJSON");
    println!("done");

    // Export to CSV
    let csv_path = format!("{}/fingerprints.csv", args.output_dir);
    print!("Exporting to {}... ", csv_path);
    export_csv(&all_signatures, &csv_path).expect("Failed to export CSV");
    println!("done");

    // Summary
    println!("\n✨ Atlas Generation Complete!");
    println!("=========================");
    println!("Total constructors: {}", all_signatures.len());
    println!("Total primes generated: {}", all_signatures.iter().map(|s| s.sample_size).sum::<usize>());
    println!("Output files:");
    println!("  - {}", ndjson_path);
    println!("  - {}", csv_path);
    println!("\nNext steps:");
    println!("  1. Train classifier: python analyze_fingerprints.py");
    println!("  2. Visualize: python plot_fingerprints.py");
    println!("  3. Compare: python compare_constructors.py");
}

/// Create the set of prime constructors to fingerprint
fn create_constructors() -> Vec<Box<dyn PrimeConstructor>> {
    let mut constructors: Vec<Box<dyn PrimeConstructor>> = Vec::new();

    // 1. Membrane: Base 6, Champion Config (1,5) k=(0,0)
    constructors.push(Box::new(MembraneConstructor::new(6, 1, 5, 0, 0)));

    // 2. Membrane: Base 6, Alternative Config k=(1,1)
    constructors.push(Box::new(MembraneConstructor::new(6, 1, 5, 1, 1)));

    // 3. Membrane: Base 10, Classic (3,7) k=(0,0)
    constructors.push(Box::new(MembraneConstructor::new(10, 3, 7, 0, 0)));

    // 4. Membrane: Base 10, Classic (3,7) k=(2,1)
    constructors.push(Box::new(MembraneConstructor::new(10, 3, 7, 2, 1)));

    // 5. Membrane: Base 30, High Performer (11,7) k=(0,0)
    constructors.push(Box::new(MembraneConstructor::new(30, 11, 7, 0, 0)));

    // 6. Membrane: Base 14, Universal Pattern (1,5) k=(0,0)
    constructors.push(Box::new(MembraneConstructor::new(14, 1, 5, 0, 0)));

    // 7. Belphegor: Original (outer=1, padding=13)
    constructors.push(Box::new(BelphegorConstructor::new(1, 13)));

    // 8. Belphegor: Variant (outer=1, padding=7)
    constructors.push(Box::new(BelphegorConstructor::new(1, 7)));

    // 9. Connector: The famous pair (10301, 3007003007003) length=5
    constructors.push(Box::new(ConnectorConstructor::new(
        BigUint::from(10301u32),
        BigUint::from(3007003007003u64),
        5,
    )));

    // 10. Connector: Same pair, length=7
    constructors.push(Box::new(ConnectorConstructor::new(
        BigUint::from(10301u32),
        BigUint::from(3007003007003u64),
        7,
    )));

    // 11. Zero-heavy connector: {0,3,6} pattern, length=5
    constructors.push(Box::new(ZeroHeavyConnectorConstructor::new(
        BigUint::from(10301u32),
        BigUint::from(3007003007003u64),
        5,
    )));

    // 12. Zero-heavy connector: {0,3,6} pattern, length=7
    constructors.push(Box::new(ZeroHeavyConnectorConstructor::new(
        BigUint::from(10301u32),
        BigUint::from(3007003007003u64),
        7,
    )));

    // 13. Random Baseline: 10 digits
    constructors.push(Box::new(RandomConstructor::new(10)));

    // 14. Random Baseline: 20 digits
    constructors.push(Box::new(RandomConstructor::new(20)));

    // 15. Random Baseline: 30 digits
    constructors.push(Box::new(RandomConstructor::new(30)));

    constructors
}
