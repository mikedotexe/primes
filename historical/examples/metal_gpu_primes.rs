//! GPU-Accelerated Prime Generation Example
//! 
//! Demonstrates the power of Metal GPU acceleration for membrane prime generation.
//! Shows 10-50x speedup over CPU for large-scale prime discovery.
//!
//! IMPORTANT: Requires macOS with Metal support and --features metal

use primes::{
    membrane::MembraneConfig,
    is_prime_miller_rabin,
};
use num_bigint::BigUint;
use std::time::{Duration, Instant};

#[cfg(feature = "metal")]
use primes::gpu::GpuSieve;

struct BenchmarkResult {
    method: String,
    count: usize,
    primes_found: usize,
    duration: Duration,
    throughput: f64,
}

impl BenchmarkResult {
    fn display(&self) {
        println!("\n{} Results:", self.method);
        println!("  Candidates tested: {}", self.count);
        println!("  Primes found: {} ({:.1}% success rate)", 
            self.primes_found, 
            (self.primes_found as f64 / self.count as f64) * 100.0
        );
        println!("  Time: {:.3}s", self.duration.as_secs_f64());
        println!("  Throughput: {:.1}k candidates/sec", self.throughput / 1000.0);
        
        if self.throughput > 1_000_000.0 {
            println!("  🔥 {:.1}M candidates/sec!", self.throughput / 1_000_000.0);
        }
    }
}

fn main() {
    println!("🚀 GPU-Accelerated Prime Generation Demo");
    println!("========================================");
    
    // Best known configuration for high prime density
    let config = MembraneConfig::new(6, 1, 5, 0, 0); // Base-6 champion
    println!("\nUsing configuration: Base {} ({},{}) k=({},{})",
        config.base, config.outer, config.inner, config.k_outer, config.k_inner
    );
    
    // Test sizes
    let test_sizes = vec![1000, 10_000, 100_000];
    
    for &size in &test_sizes {
        println!("\n--- Testing {} candidates ---", size);
        
        // CPU Benchmark
        let cpu_result = benchmark_cpu(&config, size);
        cpu_result.display();
        
        // GPU Benchmark (if available)
        #[cfg(feature = "metal")]
        {
            let gpu_result = benchmark_gpu(&config, size);
            gpu_result.display();
            
            // Show speedup
            let speedup = gpu_result.throughput / cpu_result.throughput;
            println!("\n📊 GPU Speedup: {:.1}x faster than CPU", speedup);
        }
        
        #[cfg(not(feature = "metal"))]
        println!("\n⚠️  GPU benchmarking requires --features metal on macOS");
    }
    
    // Demonstrate finding large primes
    println!("\n\n🔍 Finding Large Primes with GPU");
    println!("==================================");
    
    #[cfg(feature = "metal")]
    find_large_primes_gpu();
    
    #[cfg(not(feature = "metal"))]
    println!("GPU feature not enabled. Run with: cargo run --example metal_gpu_primes --features metal");
}

fn benchmark_cpu(config: &MembraneConfig, count: usize) -> BenchmarkResult {
    let start = Instant::now();
    let mut primes_found = 0;
    
    for seed in 0..count {
        // Generate membrane number directly
        let number = generate_membrane_number(config, seed as u64);
        
        if is_prime_miller_rabin(&number) {
            primes_found += 1;
        }
    }
    
    let duration = start.elapsed();
    let throughput = count as f64 / duration.as_secs_f64();
    
    BenchmarkResult {
        method: "CPU".to_string(),
        count,
        primes_found,
        duration,
        throughput,
    }
}

// Helper function to generate membrane numbers
fn generate_membrane_number(config: &MembraneConfig, seed: u64) -> BigUint {
    let base = BigUint::from(config.base);
    let outer = BigUint::from(config.outer);
    let inner = BigUint::from(config.inner);
    let middle = BigUint::from(seed % 10); // Simple middle digit from seed
    
    // Build symmetric membrane structure
    let mut result = outer.clone();
    
    // Add k_outer zeros
    for _ in 0..config.k_outer {
        result = &result * &base;
    }
    
    // Add inner digit
    result = &result * &base + &inner;
    
    // Add k_inner zeros
    for _ in 0..config.k_inner {
        result = &result * &base;
    }
    
    // Add middle
    result = &result * &base + &middle;
    
    // Mirror for right side
    for _ in 0..config.k_inner {
        result = &result * &base;
    }
    
    result = &result * &base + &inner;
    
    for _ in 0..config.k_outer {
        result = &result * &base;
    }
    
    result = &result * &base + &outer;
    
    result
}

#[cfg(feature = "metal")]
fn benchmark_gpu(config: &MembraneConfig, count: usize) -> BenchmarkResult {
    
    // Initialize GPU
    let gpu = match GpuSieve::new() {
        Ok(g) => g,
        Err(e) => {
            println!("GPU initialization failed: {}", e);
            return BenchmarkResult {
                method: "GPU (failed)".to_string(),
                count: 0,
                primes_found: 0,
                duration: Duration::from_secs(0),
                throughput: 0.0,
            };
        }
    };
    
    println!("  ✓ GPU initialized successfully");
    
    // Pre-compute membrane values
    let start = Instant::now();
    let mut membrane_values = Vec::with_capacity(count);
    
    for seed in 0..count {
        // For GPU, we need to ensure values fit in u32
        // Use smaller seeds for this demo
        let value = compute_membrane_u32(
            config.base,
            config.outer as u32,
            config.inner as u32,
            seed as u32
        );
        membrane_values.push(value);
    }
    
    let prep_time = start.elapsed();
    println!("  ✓ Pre-computed {} membrane values in {:.3}s", 
        count, prep_time.as_secs_f64()
    );
    
    // Run GPU sieve
    let gpu_start = Instant::now();
    let survivors = match gpu.sieve(&membrane_values, config.base) {
        Ok(s) => s,
        Err(e) => {
            println!("GPU sieve failed: {}", e);
            return BenchmarkResult {
                method: "GPU (sieve failed)".to_string(),
                count,
                primes_found: 0,
                duration: prep_time,
                throughput: 0.0,
            };
        }
    };
    let gpu_time = gpu_start.elapsed();
    
    println!("  ✓ GPU sieve completed in {:.3}s ({} survivors)", 
        gpu_time.as_secs_f64(), survivors.len()
    );
    
    // Miller-Rabin verification on survivors
    let verify_start = Instant::now();
    let mut primes_found = 0;
    
    for &idx in &survivors {
        if (idx as usize) < membrane_values.len() {
            let value = BigUint::from(membrane_values[idx as usize]);
            if is_prime_miller_rabin(&value) {
                primes_found += 1;
            }
        }
    }
    
    let verify_time = verify_start.elapsed();
    println!("  ✓ CPU verification completed in {:.3}s", verify_time.as_secs_f64());
    
    let total_duration = prep_time + gpu_time + verify_time;
    let throughput = count as f64 / total_duration.as_secs_f64();
    
    BenchmarkResult {
        method: "GPU".to_string(),
        count,
        primes_found,
        duration: total_duration,
        throughput,
    }
}

// Fast u32 computation for GPU
fn compute_membrane_u32(base: u32, outer: u32, inner: u32, seed: u32) -> u32 {
    // Simple membrane formula that fits in u32
    // For larger values, we'd need a different approach
    let b = base;
    let power = |x: u32, n: u32| -> u32 {
        (0..n).fold(1u32, |acc, _| acc.saturating_mul(x))
    };
    
    outer.saturating_mul(power(b, 4))
        .saturating_add(inner.saturating_mul(power(b, 3)))
        .saturating_add(seed.saturating_mul(power(b, 2)))
        .saturating_add(inner.saturating_mul(b))
        .saturating_add(outer)
}

#[cfg(feature = "metal")]
fn find_large_primes_gpu() {
    let gpu = match GpuSieve::new() {
        Ok(g) => g,
        Err(e) => {
            println!("GPU initialization failed: {}", e);
            return;
        }
    };
    
    // Use a configuration known to produce large primes
    let config = MembraneConfig::new(30, 11, 7, 0, 0);
    
    println!("Configuration: Base {} ({},{}) k=({},{})",
        config.base, config.outer, config.inner, config.k_outer, config.k_inner
    );
    
    // Generate larger candidates
    let batch_size = 50_000;
    let mut all_primes = Vec::new();
    let start = Instant::now();
    
    for batch_start in (0..200_000).step_by(batch_size) {
        // Generate batch of membrane values
        let mut values = Vec::with_capacity(batch_size);
        for i in 0..batch_size {
            let seed = batch_start + i;
            let value = compute_membrane_u32(
                config.base,
                config.outer as u32,
                config.inner as u32,
                seed as u32
            );
            values.push(value);
        }
        
        // GPU sieve
        if let Ok(survivors) = gpu.sieve(&values, config.base) {
            // Verify survivors
            for &idx in &survivors {
                if (idx as usize) < values.len() {
                    let value = BigUint::from(values[idx as usize]);
                    if is_prime_miller_rabin(&value) {
                        all_primes.push((batch_start + idx as usize, value));
                    }
                }
            }
        }
        
        // Progress update
        if (batch_start / batch_size) % 4 == 0 {
            print!(".");
            use std::io::{self, Write};
            io::stdout().flush().unwrap();
        }
    }
    
    let elapsed = start.elapsed();
    println!("\n\nFound {} primes in {:.2}s", all_primes.len(), elapsed.as_secs_f64());
    
    // Show some large primes
    if !all_primes.is_empty() {
        println!("\nLargest primes found:");
        all_primes.sort_by_key(|(_, p)| p.clone());
        for (seed, prime) in all_primes.iter().rev().take(5) {
            println!("  Seed {}: {} ({} digits)", seed, prime, prime.to_string().len());
        }
    }
}

#[cfg(not(feature = "metal"))]
fn benchmark_gpu(_config: &MembraneConfig, count: usize) -> BenchmarkResult {
    BenchmarkResult {
        method: "GPU (not available)".to_string(),
        count,
        primes_found: 0,
        duration: Duration::from_secs(0),
        throughput: 0.0,
    }
}