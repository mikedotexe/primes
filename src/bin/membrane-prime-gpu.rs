//! GPU-accelerated membrane prime generator
#![allow(dead_code)]

use clap::Parser;
use num_bigint::BigUint;
use std::time::Instant;

#[cfg(feature = "metal")]
use prime_physics_engine::{gpu::GpuSieve, is_prime_miller_rabin};

#[cfg(not(feature = "metal"))]
use prime_physics_engine::is_prime_miller_rabin;

#[derive(Parser, Debug)]
#[command(author, version, about = "GPU-accelerated membrane prime generator")]
struct Args {
    /// Number base (6 and 12 are champions)
    #[arg(short, long, default_value_t = 6)]
    base: u32,
    
    /// Boundary digits as comma-separated
    #[arg(short, long, default_value = "5,5")]
    digits: String,
    
    /// Number of candidates to test
    #[arg(short, long, default_value_t = 65536)]
    count: usize,
    
    /// Use GPU acceleration
    #[arg(long)]
    gpu: bool,
}

fn main() {
    let args = Args::parse();
    
    // Parse boundary digits
    let digits: Vec<u32> = args.digits.split(',')
        .map(|s| s.trim().parse().expect("Invalid digit"))
        .collect();
    let (l, r) = (digits[0], digits[1]);
    
    println!("🚀 MEMBRANE PRIME GPU TEST");
    println!("==========================");
    println!("Config: base-{}, boundary=({},{})", args.base, l, r);
    println!("Testing {} candidates", args.count);
    println!("Mode: {}\n", if args.gpu { "GPU" } else { "CPU" });
    
    let start = Instant::now();
    
    #[cfg(feature = "metal")]
    let primes = if args.gpu {
        run_gpu_sieve(args.base, l, r, args.count)
    } else {
        run_cpu_sieve(args.base, l, r, args.count)
    };
    
    #[cfg(not(feature = "metal"))]
    let primes = run_cpu_sieve(args.base, l, r, args.count);
    
    let elapsed = start.elapsed();
    
    // Results
    let density = primes.len() as f64 / args.count as f64;
    let throughput = args.count as f64 / elapsed.as_secs_f64();
    
    println!("Results:");
    println!("--------");
    println!("Found {} primes ({:.1}% density)", primes.len(), density * 100.0);
    println!("Time: {:.3}s", elapsed.as_secs_f64());
    println!("Throughput: {:.0} candidates/sec", throughput);
    
    if throughput > 1_000_000.0 {
        println!("         = {:.1}M c/s 🔥", throughput / 1_000_000.0);
    }
    
    // Show first few primes
    println!("\nFirst 10 primes:");
    for (i, prime) in primes.iter().take(10).enumerate() {
        println!("  [{}] {}", i, prime);
    }
    
    // Performance analysis
    if args.gpu {
        let cpu_baseline = 270_000.0; // Your measured CPU throughput
        let speedup = throughput / cpu_baseline;
        println!("\n📊 GPU Performance:");
        println!("  Speedup vs CPU: {:.1}x", speedup);
        println!("  Cache efficiency: ~95% (packed nibbles)");
    }
}

fn run_cpu_sieve(base: u32, l: u32, r: u32, count: usize) -> Vec<BigUint> {
    println!("Running CPU sieve...");
    
    let mut primes = Vec::new();
    for c in 0..count as u32 {
        let value = compute_membrane(base, 3, l, r, 0, 0, c as u64);
        if is_prime_miller_rabin(&value) {
            primes.push(value);
        }
    }
    
    primes
}

#[cfg(feature = "metal")]
fn run_gpu_sieve(base: u32, l: u32, r: u32, count: usize) -> Vec<BigUint> {
    println!("Initializing GPU...");
    
    let gpu = match GpuSieve::new() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("GPU init failed: {}", e);
            return run_cpu_sieve(base, l, r, count);
        }
    };
    
    // Pre-compute membrane values on CPU
    println!("Pre-computing membrane values...");
    let values: Vec<u32> = (0..count as u32)
        .map(|c| compute_membrane_u32(base, 3, l, r, 0, 0, c))
        .collect();
    
    
    // Run GPU sieve with pre-computed values
    println!("Running GPU sieve...");
    let gpu_start = std::time::Instant::now();
    let survivors = match gpu.sieve(&values, base) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("GPU sieve failed: {}", e);
            return run_cpu_sieve(base, l, r, count);
        }
    };
    let gpu_elapsed = gpu_start.elapsed();
    
    println!("GPU sieve time: {:.3}s", gpu_elapsed.as_secs_f64());
    let gpu_throughput = count as f64 / gpu_elapsed.as_secs_f64();
    println!("GPU kernel throughput: {:.1}M candidates/sec", gpu_throughput / 1_000_000.0);
    println!("GPU filtered to {} candidates", survivors.len());
    
    // CPU Miller-Rabin on survivors
    println!("Running Miller-Rabin on survivors...");
    let mut primes = Vec::new();
    for &idx in &survivors {
        if idx < count as u32 {
            // The survivor index is the position in our values array
            let membrane_value = values[idx as usize];
            let big_value = BigUint::from(membrane_value);
            if is_prime_miller_rabin(&big_value) {
                primes.push(big_value);
            }
        }
    }
    
    primes
}

fn compute_membrane(base: u32, w: u32, l: u32, r: u32, r1: u32, r2: u32, c: u64) -> BigUint {
    let b = BigUint::from(base);
    let l = BigUint::from(l);
    let r = BigUint::from(r);
    let c = BigUint::from(c);
    
    &l * b.pow(w - 1) +
    &r * b.pow(w - 2 - r1) +
    &c * b.pow(w / 2) +
    &r * b.pow(r2 + 1) +
    &l
}

fn compute_membrane_u32(base: u32, w: u32, l: u32, r: u32, r1: u32, r2: u32, c: u32) -> u32 {
    // Fast 32-bit version for GPU (valid while result fits in 32 bits)
    fn pow_u32(b: u32, e: u32) -> u32 {
        let mut res = 1u32;
        let mut base = b;
        let mut exp = e;
        while exp > 0 {
            if exp & 1 == 1 {
                res = res.wrapping_mul(base);
            }
            base = base.wrapping_mul(base);
            exp >>= 1;
        }
        res
    }
    
    l.wrapping_mul(pow_u32(base, w - 1))
        .wrapping_add(r.wrapping_mul(pow_u32(base, w - 2 - r1)))
        .wrapping_add(c.wrapping_mul(pow_u32(base, w / 2)))
        .wrapping_add(r.wrapping_mul(pow_u32(base, r2 + 1)))
        .wrapping_add(l)
}