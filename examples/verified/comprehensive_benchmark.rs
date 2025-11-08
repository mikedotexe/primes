//! Comprehensive benchmark collecting hard performance data
//! This provides extensive metrics for GPU optimization comparison

use prime_physics_engine::{
    MembraneConfig, MembraneBuilder, is_prime, is_prime_miller_rabin,
    prime_sieve::BitSieve,
};
use num_bigint::BigUint;
use rayon::prelude::*;
use std::time::{Instant, Duration};
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    println!("=== COMPREHENSIVE PRIME ENGINE BENCHMARK ===");
    println!("Hardware: {}", std::env::consts::ARCH);
    println!("Threads: {}\n", rayon::current_num_threads());
    
    // 1. SIEVE PERFORMANCE (Deterministic baseline)
    println!("1. SIEVE PERFORMANCE (BitSieve - Deterministic)");
    println!("{:-<60}", "");
    benchmark_sieve();
    
    // 2. MILLER-RABIN PERFORMANCE (Single-threaded)
    println!("\n2. MILLER-RABIN PERFORMANCE (Single-threaded)");
    println!("{:-<60}", "");
    benchmark_miller_rabin_single();
    
    // 3. MILLER-RABIN PERFORMANCE (Multi-threaded with Rayon)
    println!("\n3. MILLER-RABIN PERFORMANCE (Multi-threaded with Rayon)");
    println!("{:-<60}", "");
    benchmark_miller_rabin_parallel();
    
    // 4. MEMBRANE GENERATION PERFORMANCE
    println!("\n4. MEMBRANE GENERATION PERFORMANCE");
    println!("{:-<60}", "");
    benchmark_membrane_generation();
    
    // 5. MEMORY ACCESS PATTERNS
    println!("\n5. MEMORY ACCESS PATTERNS");
    println!("{:-<60}", "");
    benchmark_memory_patterns();
    
    // 6. CACHE EFFECTS
    println!("\n6. CACHE EFFECTS ON PRIMALITY TESTING");
    println!("{:-<60}", "");
    benchmark_cache_effects();
}

fn benchmark_sieve() {
    let limits = vec![1_000, 10_000, 100_000, 1_000_000, 10_000_000];
    
    println!("{:<12} | {:>10} | {:>12} | {:>12} | {:>10}", 
             "Limit", "Primes", "Time (ms)", "Throughput", "ns/prime");
    
    for limit in limits {
        let start = Instant::now();
        let sieve = BitSieve::new(limit);
        let primes = sieve.primes();
        let elapsed = start.elapsed();
        
        let count = primes.len();
        let ms = elapsed.as_micros() as f64 / 1000.0;
        let throughput = (limit as f64 / elapsed.as_secs_f64() / 1_000_000.0) as i64;
        let ns_per = elapsed.as_nanos() as f64 / count as f64;
        
        println!("{:<12} | {:>10} | {:>12.2} | {:>10}M/s | {:>10.1}", 
                 format_num(limit), count, ms, throughput, ns_per);
    }
}

fn benchmark_miller_rabin_single() {
    let test_sizes = vec![
        ("32-bit", vec![1_000_000_007u64, 2_147_483_647, 4_294_967_291]),
        ("64-bit", vec![18_446_744_073_709_551_557, 9_223_372_036_854_775_783]),
    ];
    
    println!("{:<10} | {:>25} | {:>12} | {:>12}", 
             "Size", "Number", "Time (μs)", "Rounds");
    
    for (size_label, numbers) in test_sizes {
        for &n in &numbers {
            let big_n = BigUint::from(n);
            let rounds = 20;
            
            let start = Instant::now();
            let _is_prime = is_prime_miller_rabin(&big_n);
            let elapsed = start.elapsed();
            
            println!("{:<10} | {:>25} | {:>12.1} | {:>12}", 
                     size_label, n, elapsed.as_micros() as f64, rounds);
        }
    }
}

fn benchmark_miller_rabin_parallel() {
    let batch_sizes = vec![1_000, 10_000, 100_000];
    let test_ranges = vec![
        ("Near 10^6", 1_000_000u64),
        ("Near 10^9", 1_000_000_000u64),
    ];
    
    println!("{:<12} | {:>10} | {:>10} | {:>12} | {:>15}", 
             "Range", "Batch", "Primes", "Time (ms)", "Candidates/sec");
    
    for (label, start) in test_ranges {
        for &batch_size in &batch_sizes {
            let candidates: Vec<BigUint> = (start..start + batch_size as u64)
                .map(|n| BigUint::from(n))
                .collect();
            
            let timer = Instant::now();
            let prime_count = AtomicUsize::new(0);
            
            candidates.par_iter().for_each(|n| {
                if is_prime_miller_rabin(n) {
                    prime_count.fetch_add(1, Ordering::Relaxed);
                }
            });
            
            let elapsed = timer.elapsed();
            let count = prime_count.load(Ordering::Relaxed);
            let throughput = batch_size as f64 / elapsed.as_secs_f64();
            
            println!("{:<12} | {:>10} | {:>10} | {:>12.2} | {:>15.0}", 
                     label, batch_size, count, 
                     elapsed.as_micros() as f64 / 1000.0, throughput);
        }
    }
}

fn benchmark_membrane_generation() {
    let configs = vec![
        ("Base 6 (1,5)", MembraneConfig::new(6, 1, 5, 0, 0)),
        ("Base 10 (3,7)", MembraneConfig::new(10, 3, 7, 0, 0)),
        ("Base 12 (5,7)", MembraneConfig::new(12, 5, 7, 0, 0)),
    ];
    
    println!("{:<15} | {:>10} | {:>10} | {:>12} | {:>15}", 
             "Config", "Seeds", "Primes", "Time (ms)", "Success Rate");
    
    for (label, config) in configs {
        let seeds = 1000;
        let mut prime_count = 0;
        
        let start = Instant::now();
        for seed in 1..=seeds {
            if let Ok(particle) = MembraneBuilder::new(config.clone())
                .with_seed((seed % 256) as u8)
                .build() {
                if is_prime(&particle.value) {
                    prime_count += 1;
                }
            }
        }
        let elapsed = start.elapsed();
        
        let success_rate = prime_count as f64 / seeds as f64 * 100.0;
        
        println!("{:<15} | {:>10} | {:>10} | {:>12.2} | {:>14.1}%", 
                 label, seeds, prime_count, 
                 elapsed.as_micros() as f64 / 1000.0, success_rate);
    }
}

fn benchmark_memory_patterns() {
    const BUFFER_SIZE: usize = 1024 * 1024; // 1MB
    let mut buffer = vec![0u64; BUFFER_SIZE];
    
    let patterns = vec![
        ("Sequential", Box::new(|i: usize| i) as Box<dyn Fn(usize) -> usize>),
        ("Stride-8", Box::new(|i| i * 8 % BUFFER_SIZE)),
        ("Random-like", Box::new(|i| (i * 13 + 7) % BUFFER_SIZE)),
        ("Prime-based", Box::new(|i| {
            let p = [2, 3, 5, 7, 11, 13, 17, 19][i % 8];
            (i * p) % BUFFER_SIZE
        })),
    ];
    
    println!("{:<15} | {:>15} | {:>12} | {:>15}", 
             "Pattern", "Accesses", "Time (ms)", "ns/access");
    
    for (name, pattern) in patterns {
        let accesses = 1_000_000;
        
        let start = Instant::now();
        for i in 0..accesses {
            let idx = pattern(i) % BUFFER_SIZE;
            unsafe {
                std::ptr::write_volatile(&mut buffer[idx], i as u64);
            }
        }
        let elapsed = start.elapsed();
        
        let ns_per = elapsed.as_nanos() as f64 / accesses as f64;
        
        println!("{:<15} | {:>15} | {:>12.2} | {:>15.1}", 
                 name, format_num(accesses), 
                 elapsed.as_micros() as f64 / 1000.0, ns_per);
    }
}

fn benchmark_cache_effects() {
    let sizes = vec![
        ("L1 (32KB)", 32 * 1024 / 8),      // 32KB / 8 bytes per u64
        ("L2 (256KB)", 256 * 1024 / 8),     // 256KB / 8 bytes per u64
        ("L3 (12MB)", 12 * 1024 * 1024 / 8), // 12MB / 8 bytes per u64
        ("RAM (48MB)", 48 * 1024 * 1024 / 8), // 48MB / 8 bytes per u64
    ];
    
    println!("{:<12} | {:>15} | {:>12} | {:>15} | {:>10}", 
             "Cache Level", "Buffer Size", "Time (ms)", "Accesses/sec", "Speedup");
    
    let base_time = Duration::new(0, 0);
    
    for (i, (label, size)) in sizes.iter().enumerate() {
        let mut buffer: Vec<u64> = vec![0; *size];
        let accesses = 10_000_000;
        
        // Warm up
        for j in 0..*size {
            buffer[j % size] = j as u64;
        }
        
        let start = Instant::now();
        for j in 0..accesses {
            let idx = (j * 13) % size;
            unsafe {
                let val = std::ptr::read_volatile(&buffer[idx]);
                std::ptr::write_volatile(&mut buffer[idx], val + 1);
            }
        }
        let elapsed = start.elapsed();
        
        let rate = accesses as f64 / elapsed.as_secs_f64();
        let speedup = if i == 0 { 
            1.0 
        } else { 
            base_time.as_secs_f64() / elapsed.as_secs_f64() 
        };
        
        println!("{:<12} | {:>15} | {:>12.2} | {:>15.0} | {:>10.2}x", 
                 label, format_num(*size * 8), 
                 elapsed.as_micros() as f64 / 1000.0, 
                 rate, speedup);
    }
}

fn format_num(n: usize) -> String {
    if n >= 1_000_000 {
        format!("{}M", n / 1_000_000)
    } else if n >= 1_000 {
        format!("{}K", n / 1_000)
    } else {
        n.to_string()
    }
}