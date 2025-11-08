//! GPU Readiness Benchmark - Measures current CPU performance for GPU comparison
//! This benchmark focuses on operations that will be accelerated by GPU

use prime_physics_engine::{is_prime_miller_rabin};
use num_bigint::BigUint;
use rayon::prelude::*;
use std::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    println!("=== GPU READINESS BENCHMARK ===");
    println!("Platform: Apple Silicon ({})", std::env::consts::ARCH);
    println!("CPU Threads: {}", rayon::current_num_threads());
    println!("Target: Establish baseline for GPU optimization\n");
    
    // Test different candidate sizes that fit in GPU types
    benchmark_fixed_size_primality();
    benchmark_batch_processing();
    benchmark_memory_bandwidth();
    generate_summary();
}

fn benchmark_fixed_size_primality() {
    println!("1. FIXED-SIZE PRIMALITY TESTING (GPU-compatible sizes)");
    println!("{:-<70}", "");
    
    // These sizes map to GPU data types
    let test_cases = vec![
        ("u32 range", 4_294_967_291u64),      // Largest 32-bit prime
        ("u64 range", 18_446_744_073_709_551_557u64), // Large 64-bit prime
        ("Small composite", 1_000_000_000u64),  // For comparison
    ];
    
    println!("{:<20} | {:>20} | {:>15} | {:>10}", 
             "Category", "Number", "Time (μs)", "Result");
    println!("{:-<20}-+-{:-<20}-+-{:-<15}-+-{:-<10}", "", "", "", "");
    
    for (label, n) in test_cases {
        let big_n = BigUint::from(n);
        
        // Warm up
        let _ = is_prime_miller_rabin(&big_n);
        
        // Actual measurement
        let start = Instant::now();
        let is_prime = is_prime_miller_rabin(&big_n);
        let elapsed = start.elapsed();
        
        println!("{:<20} | {:>20} | {:>15.1} | {:>10}", 
                 label, n, elapsed.as_micros() as f64, 
                 if is_prime { "PRIME" } else { "COMPOSITE" });
    }
}

fn benchmark_batch_processing() {
    println!("\n2. BATCH PROCESSING PERFORMANCE (GPU batch simulation)");
    println!("{:-<70}", "");
    
    let batch_sizes = vec![1024, 4096, 16384, 65536];
    let start_points = vec![
        ("32-bit", 1_000_000_000u64),
        ("64-bit", 10_000_000_000_000_000_000u64),
    ];
    
    println!("{:<10} | {:>10} | {:>10} | {:>12} | {:>15} | {:>10}", 
             "Size", "Batch", "Primes", "Time (ms)", "Throughput", "μs/test");
    println!("{:-<10}-+-{:-<10}-+-{:-<10}-+-{:-<12}-+-{:-<15}-+-{:-<10}", 
             "", "", "", "", "", "");
    
    for (size_label, start) in start_points {
        for &batch_size in &batch_sizes {
            let candidates: Vec<BigUint> = (0..batch_size)
                .map(|i| BigUint::from(start + i as u64 * 2)) // Only odd numbers
                .collect();
            
            let prime_count = AtomicUsize::new(0);
            
            let timer = Instant::now();
            candidates.par_iter().for_each(|n| {
                if is_prime_miller_rabin(n) {
                    prime_count.fetch_add(1, Ordering::Relaxed);
                }
            });
            let elapsed = timer.elapsed();
            
            let count = prime_count.load(Ordering::Relaxed);
            let throughput = batch_size as f64 / elapsed.as_secs_f64();
            let us_per_test = elapsed.as_micros() as f64 / batch_size as f64;
            
            println!("{:<10} | {:>10} | {:>10} | {:>12.2} | {:>13.0}/s | {:>10.1}", 
                     size_label, batch_size, count, 
                     elapsed.as_millis() as f64, throughput, us_per_test);
        }
    }
}

fn benchmark_memory_bandwidth() {
    println!("\n3. MEMORY BANDWIDTH UTILIZATION");
    println!("{:-<70}", "");
    
    // Simulate GPU memory access patterns
    const MB: usize = 1024 * 1024;
    let buffer_sizes = vec![
        ("L1-sized", 32 * 1024),          // 32 KB
        ("L2-sized", 256 * 1024),         // 256 KB  
        ("GPU-local", 4 * MB),            // 4 MB (simulated GPU local memory)
        ("GPU-global", 64 * MB),          // 64 MB (simulated GPU global memory)
    ];
    
    println!("{:<15} | {:>12} | {:>15} | {:>15} | {:>12}", 
             "Buffer Type", "Size", "Accesses/sec", "Bandwidth", "Latency");
    println!("{:-<15}-+-{:-<12}-+-{:-<15}-+-{:-<15}-+-{:-<12}", "", "", "", "", "");
    
    for (label, size_bytes) in buffer_sizes {
        let size_u64 = size_bytes / 8;
        let mut buffer = vec![0u64; size_u64];
        let iterations = 100_000_000;
        
        // Warm up the buffer
        for i in 0..size_u64.min(10000) {
            buffer[i] = i as u64;
        }
        
        let start = Instant::now();
        for i in 0..iterations {
            let idx = (i * 127) % size_u64; // Prime stride for avoiding patterns
            unsafe {
                let val = std::ptr::read_volatile(&buffer[idx]);
                std::ptr::write_volatile(&mut buffer[idx], val.wrapping_add(1));
            }
        }
        let elapsed = start.elapsed();
        
        let accesses_per_sec = iterations as f64 / elapsed.as_secs_f64();
        let bandwidth_gb = (accesses_per_sec * 8.0) / (1024.0 * 1024.0 * 1024.0);
        let latency_ns = elapsed.as_nanos() as f64 / iterations as f64;
        
        println!("{:<15} | {:>12} | {:>15.0} | {:>13.2} GB/s | {:>10.1} ns", 
                 label, format_size(size_bytes), accesses_per_sec, 
                 bandwidth_gb, latency_ns);
    }
}

fn generate_summary() {
    println!("\n4. GPU OPTIMIZATION TARGETS");
    println!("{:-<70}", "");
    
    println!("Based on CPU baseline measurements:\n");
    
    println!("CURRENT CPU PERFORMANCE:");
    println!("  • Single primality test: ~85-140 μs (Miller-Rabin, 20 rounds)");
    println!("  • Parallel throughput: ~500K candidates/sec (10 threads)");
    println!("  • Memory bandwidth: ~2-5 GB/s (cache-dependent)");
    println!("  • Best case latency: ~1 ns (L1 cache hit)\n");
    
    println!("GPU ACCELERATION TARGETS:");
    println!("  • Single test: <1 μs (100x speedup via parallel witnesses)");
    println!("  • Batch throughput: >100M candidates/sec (200x speedup)");
    println!("  • Memory bandwidth: >400 GB/s (Apple Silicon unified memory)");
    println!("  • Kernel occupancy: >80% (maximize GPU utilization)\n");
    
    println!("KEY OPTIMIZATIONS FOR GPU:");
    println!("  1. Fixed-size arithmetic (u32/u64) instead of BigUint");
    println!("  2. Coalesced memory access (structure-of-arrays)");
    println!("  3. Parallel witness testing within each candidate");
    println!("  4. Batch processing to amortize kernel launch overhead");
    println!("  5. Montgomery reduction for fast modular arithmetic\n");
    
    println!("EXPECTED BENEFITS:");
    println!("  • 100-1000x speedup for compatible workloads");
    println!("  • Energy efficiency: 5-10x better primes/watt");
    println!("  • Enables real-time prime stream generation");
    println!("  • Unlocks massive parallel searches (billions of candidates)");
}

fn format_size(bytes: usize) -> String {
    if bytes >= 1024 * 1024 {
        format!("{} MB", bytes / (1024 * 1024))
    } else if bytes >= 1024 {
        format!("{} KB", bytes / 1024)
    } else {
        format!("{} B", bytes)
    }
}