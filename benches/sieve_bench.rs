//! Criterion benchmarks for prime sieve performance regression testing

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use prime_physics_engine::prime_sieve::{BitSieve, segmented_sieve, warm_slc, sieve_count_and_time};

/// Benchmark basic sieve for various sizes
fn bench_basic_sieve(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic_sieve");
    
    for limit in [10_000, 100_000, 1_000_000, 10_000_000] {
        group.throughput(Throughput::Elements(limit as u64));
        group.bench_with_input(BenchmarkId::from_parameter(limit), &limit, |b, &limit| {
            b.iter(|| {
                let sieve = BitSieve::new(limit);
                black_box(sieve.primes().len())
            });
        });
    }
    
    group.finish();
}

/// Benchmark segmented multi-core sieve
fn bench_segmented_sieve(c: &mut Criterion) {
    let mut group = c.benchmark_group("segmented_sieve");
    
    for limit in [100_000, 1_000_000, 10_000_000] {
        group.throughput(Throughput::Elements(limit as u64));
        group.bench_with_input(BenchmarkId::from_parameter(limit), &limit, |b, &limit| {
            b.iter(|| {
                black_box(segmented_sieve(limit, 65536).len())
            });
        });
    }
    
    group.finish();
}

/// Benchmark cache warming effectiveness
fn bench_cache_warming(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_warming");
    
    for (primes, pressure) in [(10_000, 0.1), (100_000, 0.25), (1_000_000, 0.5)] {
        let param = format!("{}_{}", primes, (pressure * 100.0) as u32);
        group.bench_with_input(
            BenchmarkId::from_parameter(&param), 
            &(primes, pressure), 
            |b, &(primes, pressure)| {
                b.iter(|| {
                    let result = warm_slc(primes, pressure);
                    black_box(result.primes_generated)
                });
            }
        );
    }
    
    group.finish();
}

/// Benchmark cycle-accurate timing
fn bench_cycle_timing(c: &mut Criterion) {
    let mut group = c.benchmark_group("cycle_timing");
    
    for limit in [10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(limit), &limit, |b, &limit| {
            b.iter(|| {
                let (count, cycles) = sieve_count_and_time(limit);
                black_box((count, cycles))
            });
        });
    }
    
    group.finish();
}

/// Benchmark prime density for different sieve sizes
fn bench_prime_density(c: &mut Criterion) {
    c.bench_function("prime_density_10M", |b| {
        b.iter(|| {
            let sieve = BitSieve::new(10_000_000);
            let count = sieve.primes().len();
            // Calculate prime density
            black_box(count as f64 / 10_000_000.0)
        });
    });
}

/// Benchmark memory efficiency
fn bench_memory_efficiency(c: &mut Criterion) {
    c.bench_function("memory_per_prime", |b| {
        b.iter(|| {
            let limit = 1_000_000;
            let sieve = BitSieve::new(limit);
            let count = sieve.primes().len();
            // Calculate bytes per prime (bit-packed)
            let bytes_used = limit / 16; // Only odds, 1 bit each
            black_box(bytes_used as f64 / count as f64)
        });
    });
}

criterion_group!(
    benches, 
    bench_basic_sieve,
    bench_segmented_sieve,
    bench_cache_warming,
    bench_cycle_timing,
    bench_prime_density,
    bench_memory_efficiency
);
criterion_main!(benches);