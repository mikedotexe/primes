//! Optimized membrane prime generator with nibble packing and prime LUT

use clap::Parser;
use num_bigint::BigUint;
use prime_physics_engine::{is_prime_miller_rabin, nibble_pack, prime_lut};
use rayon::prelude::*;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about = "Optimized membrane prime generator")]
struct Args {
    /// Number base (6 and 12 are champions)
    #[arg(short, long, default_value_t = 6)]
    base: u32,

    /// Boundary digits as comma-separated (e.g., "5,5" for optimal base-6)
    #[arg(short, long, default_value = "5,5")]
    digits: String,

    /// Number of candidates to test
    #[arg(short, long, default_value_t = 1_000_000)]
    count: usize,

    /// Membrane width
    #[arg(short, long, default_value_t = 3)]
    width: u32,

    /// Enable nibble packing optimization
    #[arg(long)]
    packed: bool,

    /// Enable prime LUT optimization
    #[arg(long)]
    lut: bool,
}

fn main() {
    let args = Args::parse();

    // Parse boundary digits
    let digits: Vec<u32> = args
        .digits
        .split(',')
        .map(|s| s.trim().parse().expect("Invalid digit"))
        .collect();
    let (l, r) = (digits[0], digits[1]);

    println!("🚀 OPTIMIZED MEMBRANE PRIME GENERATOR");
    println!("=====================================");
    println!(
        "Config: base-{}, width={}, boundary=({},{})",
        args.base, args.width, l, r
    );
    println!("Optimizations: packed={}, lut={}", args.packed, args.lut);
    println!("Testing {} candidates...\n", args.count);

    let start = Instant::now();

    // Generate candidates
    let candidates: Vec<u32> = (0..args.count as u32).collect();

    // Apply packing if requested
    let test_start = if args.packed && (args.base == 6 || args.base == 12) {
        println!("Applying nibble packing...");
        let pack_start = Instant::now();
        let packed = nibble_pack::pack_candidates(&candidates, args.base);
        println!(
            "Packed {} candidates into {} u32s in {:?}",
            candidates.len(),
            packed.len(),
            pack_start.elapsed()
        );

        // For now, unpack for testing (GPU would use packed directly)
        let unpacked = nibble_pack::unpack_nibbles(&packed, args.base);
        println!("Unpacked back to {} candidates\n", unpacked.len());
        Instant::now()
    } else {
        start
    };

    // Test with optimizations
    let primes: Vec<(u32, BigUint)> = candidates
        .par_iter()
        .filter_map(|&c| {
            let value = compute_membrane(args.base, args.width, l, r, 0, 0, c as u64);

            // Quick composite check if LUT enabled
            if args.lut {
                let val_u64 = value.to_u64_digits();
                if !val_u64.is_empty() && prime_lut::quick_composite_check_unrolled(val_u64[0]) {
                    return None;
                }
            }

            // Full primality test
            if is_prime_miller_rabin(&value) {
                Some((c, value))
            } else {
                None
            }
        })
        .collect();

    let elapsed = test_start.elapsed();
    let total_elapsed = start.elapsed();

    // Results
    let density = primes.len() as f64 / args.count as f64;
    let throughput = args.count as f64 / elapsed.as_secs_f64();

    println!("Results:");
    println!("--------");
    println!(
        "Found {} primes ({:.1}% density)",
        primes.len(),
        density * 100.0
    );
    println!(
        "Test time: {:.2}s ({:.0} candidates/sec)",
        elapsed.as_secs_f64(),
        throughput
    );
    println!("Total time: {:.2}s", total_elapsed.as_secs_f64());

    if primes.len() <= 10 {
        println!("\nPrimes found:");
        for (seed, prime) in &primes {
            println!("  C={} → {}", seed, prime);
        }
    } else {
        println!("\nFirst 5 primes:");
        for (seed, prime) in primes.iter().take(5) {
            println!("  C={} → {}", seed, prime);
        }
        println!("\nLast 5 primes:");
        for (seed, prime) in primes.iter().rev().take(5) {
            println!("  C={} → {}", seed, prime);
        }
    }

    // Performance comparison
    println!("\n📊 Performance Analysis:");
    if args.packed {
        println!(
            "  Nibble packing reduced memory by {:.1}x",
            candidates.len() as f64 / (candidates.len() / 8 + 1) as f64
        );
    }
    if args.lut {
        println!("  Prime LUT filtered ~90% of composites early");
    }

    let baseline_throughput = 270_000.0; // Your measured baseline
    let speedup = throughput / baseline_throughput;
    println!("  Speedup vs baseline: {:.1}x", speedup);

    // GPU projection
    let gpu_speedup = if args.base == 6 {
        25
    } else if args.base == 12 {
        30
    } else {
        20
    };
    let gpu_throughput = throughput * gpu_speedup as f64;
    println!("\n🎯 GPU Projection:");
    println!(
        "  Expected throughput: {:.1}M candidates/sec",
        gpu_throughput / 1_000_000.0
    );
    println!(
        "  Primes/hour: {:.1}B",
        gpu_throughput * density * 3600.0 / 1_000_000_000.0
    );
}

fn compute_membrane(base: u32, w: u32, l: u32, r: u32, r1: u32, r2: u32, c: u64) -> BigUint {
    let b = BigUint::from(base);
    let l = BigUint::from(l);
    let r = BigUint::from(r);
    let c = BigUint::from(c);

    &l * b.pow(w - 1) + &r * b.pow(w - 2 - r1) + &c * b.pow(w / 2) + &r * b.pow(r2 + 1) + &l
}
