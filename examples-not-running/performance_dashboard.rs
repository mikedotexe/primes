//! Performance monitoring dashboard for the Prime Physics Engine
//! 
//! This example demonstrates comprehensive performance monitoring across
//! all major subsystems: membrane generation, prime testing, sieve operations,
//! and phase 4 neural network inference.

use primes::{
    MembraneConfig, PerfMonitor,
    prime_sieve::{BitSieve, segmented_sieve, warm_slc},
};
use std::time::Duration;

fn main() {
    println!("🚀 Prime Physics Engine - Performance Dashboard");
    println!("=" .repeat(70));
    
    let monitor = PerfMonitor::new();
    
    // 1. Membrane Generation Performance
    benchmark_membrane_generation(&monitor);
    
    // 2. Prime Sieve Performance
    benchmark_prime_sieve(&monitor);
    
    // 3. Cache Warming Performance
    benchmark_cache_operations(&monitor);
    
    // 4. Neural Network Performance (if phase4 enabled)
    #[cfg(feature = "phase4")]
    benchmark_neural_network(&monitor);
    
    // Generate comprehensive report
    monitor.report();
    
    // Additional analysis
    println!("\n📊 Performance Analysis");
    println!("=" .repeat(70));
    analyze_results(&monitor);
}

fn benchmark_membrane_generation(monitor: &PerfMonitor) {
    println!("\n🧬 Benchmarking Membrane Generation...");
    
    let configs = vec![
        (6, 1, 5, 0, 0),    // Base 6 optimal
        (10, 3, 7, 0, 0),   // Base 10 standard
        (12, 5, 7, 0, 0),   // Base 12 duodecimal
        (30, 11, 7, 0, 0),  // Base 30 high performer
    ];
    
    for (base, outer, inner, k_outer, k_inner) in configs {
        let config = MembraneConfig {
            base,
            outer,
            inner,
            k_outer,
            k_inner,
        };
        
        let name = format!("membrane_b{}_({},{})_k({},{})", base, outer, inner, k_outer, k_inner);
        
        // Test small seeds
        monitor.time(&format!("{}_small", name), || {
            for seed in 1..=100 {
                let candidate = config.generate(seed);
                std::hint::black_box(candidate);
            }
        });
        
        // Test large seeds
        monitor.time(&format!("{}_large", name), || {
            for seed in 1000..1100 {
                let candidate = config.generate(seed);
                std::hint::black_box(candidate);
            }
        });
        
        // Test primality checking
        monitor.time(&format!("{}_primality", name), || {
            let mut prime_count = 0;
            for seed in 1..=50 {
                let candidate = config.generate(seed);
                if candidate.is_prime() {
                    prime_count += 1;
                }
            }
            std::hint::black_box(prime_count);
        });
    }
}

fn benchmark_prime_sieve(monitor: &PerfMonitor) {
    println!("\n🔢 Benchmarking Prime Sieve...");
    
    let limits = vec![
        ("10K", 10_000),
        ("100K", 100_000),
        ("1M", 1_000_000),
        ("10M", 10_000_000),
    ];
    
    for (name, limit) in limits {
        // Single-core BitSieve
        monitor.time(&format!("sieve_single_{}", name), || {
            let sieve = BitSieve::new(limit);
            let primes = sieve.primes();
            std::hint::black_box(primes.len());
        });
        
        // Multi-core segmented sieve
        monitor.time(&format!("sieve_multi_{}", name), || {
            let primes = segmented_sieve(limit, 65536);
            std::hint::black_box(primes.len());
        });
        
        // Visit pattern (no allocation)
        monitor.time(&format!("sieve_visit_{}", name), || {
            let sieve = BitSieve::new(limit);
            let mut count = 0;
            sieve.visit_primes(|_p| count += 1);
            std::hint::black_box(count);
        });
    }
}

fn benchmark_cache_operations(monitor: &PerfMonitor) {
    println!("\n💾 Benchmarking Cache Operations...");
    
    let pressures = vec![
        ("10%", 0.1),
        ("25%", 0.25),
        ("50%", 0.5),
    ];
    
    for (name, pressure) in pressures {
        monitor.time(&format!("cache_warm_{}", name), || {
            warm_slc(100_000, pressure);
        });
    }
    
    // Test cache-aware prime generation
    monitor.time("cache_aware_generation", || {
        warm_slc(50_000, 0.2);
        let config = MembraneConfig {
            base: 10,
            outer: 3,
            inner: 7,
            k_outer: 0,
            k_inner: 0,
        };
        
        for seed in 1..=100 {
            let candidate = config.generate(seed);
            if candidate.is_prime() {
                std::hint::black_box(candidate);
            }
        }
    });
}

#[cfg(feature = "phase4")]
fn benchmark_neural_network(monitor: &PerfMonitor) {
    use primes::phase4::{predict_sme_padded_safe, OnChipRL};
    
    println!("\n🧠 Benchmarking Neural Network (Phase 4)...");
    
    // Test single inference
    let input = [1, -2, 3, -4, 5, -6, 7, -8, 0, 0, 0, 0, 0, 0, 0, 0];
    
    monitor.time("neural_single_inference", || {
        let result = predict_sme_padded_safe(input);
        std::hint::black_box(result);
    });
    
    // Test batch inference
    monitor.time("neural_batch_1000", || {
        for i in 0..1000 {
            let mut input = [0i8; 16];
            input[0] = (i % 128) as i8;
            input[1] = ((i / 128) % 128) as i8;
            let result = predict_sme_padded_safe(input);
            std::hint::black_box(result);
        }
    });
    
    // Test RL controller
    monitor.time("rl_controller_update", || {
        let mut rl = OnChipRL::new();
        for i in 0..1000 {
            let pmu_sample = (i % 16) as u8;
            let latency = 5 + (i % 10) as u32;
            rl.tick(pmu_sample, latency);
        }
        std::hint::black_box(rl.best_action());
    });
}

fn analyze_results(monitor: &PerfMonitor) {
    let metrics = monitor.get_metrics();
    
    // Find fastest/slowest operations
    if let Some(fastest) = metrics.iter().min_by_key(|m| m.avg_time()) {
        println!("⚡ Fastest operation: {} ({:.2} μs avg)", 
                 fastest.name, 
                 fastest.avg_time().as_secs_f64() * 1_000_000.0);
    }
    
    if let Some(slowest) = metrics.iter().max_by_key(|m| m.avg_time()) {
        println!("🐌 Slowest operation: {} ({:.2} ms avg)", 
                 slowest.name, 
                 slowest.avg_time().as_secs_f64() * 1000.0);
    }
    
    // Calculate total time spent
    let total_time: Duration = metrics.iter()
        .map(|m| m.total_time)
        .sum();
    println!("\n⏱️  Total benchmark time: {:.2} seconds", total_time.as_secs_f64());
    
    // Operations per second summary
    println!("\n📈 Throughput Summary:");
    for m in metrics.iter().filter(|m| m.name.contains("membrane") && m.name.contains("small")) {
        let ops_per_sec = m.ops_per_sec();
        println!("  {}: {:.0} ops/sec", m.name, ops_per_sec);
    }
    
    // Memory efficiency estimate
    println!("\n💾 Memory Efficiency:");
    if let Some(sieve_1m) = metrics.iter().find(|m| m.name == "sieve_single_1M") {
        let primes_per_ms = 78498.0 / (sieve_1m.avg_time().as_secs_f64() * 1000.0);
        println!("  BitSieve: {:.0} primes/ms (1M limit)", primes_per_ms);
        println!("  Memory: ~62.5 KB for 1M candidates (16x compression)");
    }
    
    // Phase 4 specific analysis
    #[cfg(feature = "phase4")]
    {
        println!("\n🎯 Neural Network Performance:");
        if let Some(nn_single) = metrics.iter().find(|m| m.name == "neural_single_inference") {
            let latency_ns = nn_single.avg_time().as_nanos();
            println!("  Single inference: {} ns", latency_ns);
            println!("  Target with SME: <3 ns ({}x speedup needed)", latency_ns / 3);
        }
    }
}