//! Benchmark comparing single-core vs segmented multi-core sieve
//! Shows the performance improvements from the compact message optimizations

use prime_physics_engine::prime_sieve::{BitSieve, segmented_sieve};
use std::time::Instant;

fn benchmark_sieve(name: &str, limit: usize, mut f: impl FnMut()) {
    println!("\n{} (limit = {})", name, limit);
    println!("{}", "-".repeat(50));
    
    // Warm-up
    f();
    
    // Timed runs
    let start = Instant::now();
    f();
    let elapsed = start.elapsed();
    
    println!("Time: {:.2} ms", elapsed.as_secs_f64() * 1000.0);
    println!("Speed: {:.2} million primes/sec", 
             limit as f64 / elapsed.as_secs_f64() / 1_000_000.0);
}

fn main() {
    println!("🚀 Prime Sieve Performance Benchmark");
    println!("====================================");
    
    // Test different sizes
    for &limit in &[100_000, 1_000_000, 10_000_000] {
        // Single-core baseline
        benchmark_sieve(&format!("Single-core BitSieve"), limit, || {
            let sieve = BitSieve::new(limit);
            let _primes = sieve.primes();
        });
        
        // Multi-core segmented (L1 cache-aware)
        benchmark_sieve(&format!("Multi-core Segmented (64KB chunks)"), limit, || {
            let _primes = segmented_sieve(limit, 65536);
        });
        
        // Multi-core segmented (larger chunks)
        benchmark_sieve(&format!("Multi-core Segmented (256KB chunks)"), limit, || {
            let _primes = segmented_sieve(limit, 262144);
        });
    }
    
    println!("\n📊 Performance Analysis");
    println!("======================");
    
    // Direct comparison at 10M
    let limit = 10_000_000;
    
    let start = Instant::now();
    let sieve = BitSieve::new(limit);
    let single_primes = sieve.primes();
    let single_time = start.elapsed();
    
    let start = Instant::now();
    let multi_primes = segmented_sieve(limit, 65536);
    let multi_time = start.elapsed();
    
    println!("Primes found: {} (both methods)", single_primes.len());
    println!("Single-core time: {:.2} ms", single_time.as_secs_f64() * 1000.0);
    println!("Multi-core time: {:.2} ms", multi_time.as_secs_f64() * 1000.0);
    println!("Speedup: {:.2}x", single_time.as_secs_f64() / multi_time.as_secs_f64());
    
    // Verify correctness
    assert_eq!(single_primes.len(), multi_primes.len(), 
               "Both methods should find the same number of primes");
    
    println!("\n✅ Benchmark complete!");
}