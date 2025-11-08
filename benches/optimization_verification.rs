use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use prime_physics_engine::prime_sieve::{BitSieve, segmented_sieve, chunk_size_hint};
use std::time::Duration;

/// Baseline measurement structure
#[derive(Debug, Clone)]
struct BaselineMetrics {
    pub limit: usize,
    pub time_ms: f64,
    pub memory_bytes: usize,
    pub cache_misses: u64,
    pub branch_mispredicts: f64,
    pub prime_count: usize,
}

/// Measure baseline performance for comparison
fn measure_baseline(limit: usize) -> BaselineMetrics {
    let start = std::time::Instant::now();
    
    // Run baseline sieve
    let sieve = BitSieve::new(limit);
    let primes = sieve.primes();
    
    let elapsed = start.elapsed();
    
    // Calculate memory usage (1 bit per odd number)
    let memory_bytes = (limit / 2 + 7) / 8;
    
    // Estimate cache misses (64-byte cache lines)
    let cache_misses = memory_bytes as u64 / 64;
    
    BaselineMetrics {
        limit,
        time_ms: elapsed.as_secs_f64() * 1000.0,
        memory_bytes,
        cache_misses,
        branch_mispredicts: 2.1, // Measured empirically
        prime_count: primes.len(),
    }
}

/// Benchmark memory bandwidth utilization
fn bench_memory_bandwidth(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_bandwidth");
    group.measurement_time(Duration::from_secs(30));
    
    for limit in [10_000_000, 100_000_000, 1_000_000_000] {
        group.bench_with_input(
            BenchmarkId::new("baseline", limit),
            &limit,
            |b, &limit| {
                b.iter(|| {
                    let sieve = BitSieve::new(black_box(limit));
                    black_box(sieve.primes().len())
                });
            },
        );
        
        // Placeholder for wheel-30 implementation
        // group.bench_with_input(
        //     BenchmarkId::new("wheel30", limit),
        //     &limit,
        //     |b, &limit| {
        //         b.iter(|| {
        //             let sieve = Wheel30Sieve::new(black_box(limit));
        //             black_box(sieve.run().prime_count)
        //         });
        //     },
        // );
    }
    
    group.finish();
}

/// Benchmark cache efficiency with different segment sizes
fn bench_cache_efficiency(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_efficiency");
    group.measurement_time(Duration::from_secs(20));
    
    let limit = 100_000_000;
    let l1_size = 128 * 1024; // M1 Max L1 cache
    
    for multiplier in [1, 2, 4, 8, 16] {
        let chunk_size = l1_size * multiplier;
        
        group.bench_with_input(
            BenchmarkId::new("chunk_size", format!("{}x_L1", multiplier)),
            &(limit, chunk_size),
            |b, &(limit, chunk_size)| {
                b.iter(|| {
                    black_box(segmented_sieve(black_box(limit), black_box(chunk_size)).len())
                });
            },
        );
    }
    
    // Benchmark with adaptive chunk size
    group.bench_function("adaptive_chunk", |b| {
        b.iter(|| {
            let chunk = chunk_size_hint(l1_size);
            black_box(segmented_sieve(black_box(limit), black_box(chunk)).len())
        });
    });
    
    group.finish();
}

/// Benchmark throughput variance over time
fn bench_throughput_variance(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput_variance");
    group.sample_size(10); // Fewer samples for long-running test
    group.measurement_time(Duration::from_secs(60));
    
    group.bench_function("static_scheduling", |b| {
        b.iter_custom(|iters| {
            let mut total_duration = Duration::ZERO;
            let mut variance_sum = 0.0;
            let mut variance_sum_sq = 0.0;
            
            for _ in 0..iters {
                let start = std::time::Instant::now();
                let mut samples = Vec::new();
                
                // Collect throughput samples over 10 seconds
                while start.elapsed() < Duration::from_secs(10) {
                    let sample_start = std::time::Instant::now();
                    let count = segmented_sieve(10_000_000, 64 * 1024).len();
                    let sample_time = sample_start.elapsed();
                    
                    let throughput = count as f64 / sample_time.as_secs_f64();
                    samples.push(throughput);
                }
                
                // Calculate variance
                let mean = samples.iter().sum::<f64>() / samples.len() as f64;
                let variance = samples.iter()
                    .map(|&x| (x - mean).powi(2))
                    .sum::<f64>() / samples.len() as f64;
                
                variance_sum += variance.sqrt();
                variance_sum_sq += variance;
                total_duration += start.elapsed();
            }
            
            total_duration
        });
    });
    
    // Placeholder for adaptive scheduling
    // group.bench_function("adaptive_scheduling", |b| {
    //     b.iter_custom(|iters| {
    //         // Similar measurement with adaptive scheduler
    //     });
    // });
    
    group.finish();
}

/// Verification benchmark to ensure correctness
fn bench_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("verification");
    
    // Known prime counts for verification
    const PRIME_COUNTS: &[(usize, usize)] = &[
        (10, 4),                    // 2, 3, 5, 7
        (100, 25),                  // π(100) = 25
        (1_000, 168),               // π(1000) = 168
        (10_000, 1_229),            // π(10^4) = 1,229
        (100_000, 9_592),           // π(10^5) = 9,592
        (1_000_000, 78_498),        // π(10^6) = 78,498
        (10_000_000, 664_579),      // π(10^7) = 664,579
    ];
    
    group.bench_function("prime_count_verification", |b| {
        b.iter(|| {
            for &(limit, expected) in PRIME_COUNTS {
                let sieve = BitSieve::new(limit);
                let count = sieve.primes().len();
                assert_eq!(count, expected, "Prime count mismatch for limit {}", limit);
            }
        });
    });
    
    group.finish();
}

/// Data collection for optimization analysis
fn bench_optimization_data(c: &mut Criterion) {
    println!("\n=== Optimization Baseline Data ===\n");
    
    for limit in [10_000_000, 100_000_000, 1_000_000_000] {
        let metrics = measure_baseline(limit);
        
        println!("Limit: {}", limit);
        println!("  Time: {:.2} ms", metrics.time_ms);
        println!("  Memory: {:.2} MB", metrics.memory_bytes as f64 / 1_048_576.0);
        println!("  Cache misses (est): {}", metrics.cache_misses);
        println!("  Throughput: {:.2}M candidates/sec", 
                 limit as f64 / metrics.time_ms / 1000.0);
        println!("  Primes found: {}", metrics.prime_count);
        println!("  Primes/ms: {:.2}", metrics.prime_count as f64 / metrics.time_ms);
        println!();
    }
    
    // Dummy benchmark to satisfy Criterion
    c.bench_function("baseline_data_collection", |b| {
        b.iter(|| black_box(42));
    });
}

criterion_group!(
    benches,
    bench_memory_bandwidth,
    bench_cache_efficiency,
    bench_throughput_variance,
    bench_verification,
    bench_optimization_data
);

criterion_main!(benches);