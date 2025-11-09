//! GPU performance benchmark example

#[cfg(feature = "metal")]
use primes::gpu::GpuSieve;
use primes::is_prime_miller_rabin;
use num_bigint::BigUint;
use std::time::Instant;
fn main() {
    #[cfg(not(feature = "metal"))]
    {
        println!("This example requires the 'metal' feature.");
        println!("Run with: cargo run --features metal --example gpu_benchmark");
        return;
    }
    
    #[cfg(feature = "metal")]
    run_benchmark();
}
fn run_benchmark() {
    println!("=== GPU Performance Benchmark ===\n");
    let test_sizes = vec![10_000, 100_000, 1_000_000, 4_000_000];
    let base = 6;
    let (l, r) = (5, 5);
    // Initialize GPU once
    let gpu = match GpuSieve::new() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("GPU initialization failed: {}", e);
            return;
        }
    };
    println!("Candidates | GPU Time | Throughput | Survivors");
    println!("-----------|----------|------------|----------");
    for &count in &test_sizes {
        // Pre-compute membrane values
        let values: Vec<u32> = (0..count)
            .map(|c| compute_membrane_u32(base, l, r, c as u32))
            .collect();
        
        // Time GPU sieve
        let start = Instant::now();
        let survivors = match gpu.sieve(&values, base) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("GPU sieve failed: {}", e);
                continue;
            }
        };
        let gpu_time = start.elapsed();
        let throughput = count as f64 / gpu_time.as_secs_f64();
        println!("{:10} | {:8.3}s | {:8.1}M/s | {:9}", 
            count,
            gpu_time.as_secs_f64(),
            throughput / 1_000_000.0,
            survivors.len()
        );
    println!("\nNote: Throughput measures GPU kernel performance only");
fn compute_membrane_u32(base: u32, l: u32, r: u32, c: u32) -> u32 {
    // Simplified membrane for u32 range
    let w = 3;
    let b = base as u64;
    let result = (l as u64) * b.pow(w - 1) +
                 (r as u64) * b.pow(w - 2) +
                 (c as u64) * b.pow(w / 2) +
                 (r as u64) * b +
                 (l as u64);
    result as u32
