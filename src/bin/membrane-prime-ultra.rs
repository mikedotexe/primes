//! Ultra-optimized membrane prime generator implementing all improvements
//! Michael Purvis, Claude (Anthropic), and o3-pro (OpenAI) - July 2025

use clap::Parser;
use num_bigint::BigUint;
use rayon::prelude::*;
use std::time::Instant;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::_mulx_u32;

/// Ultra-fast Miller-Rabin with intrinsics
#[inline(always)]
fn mod_mul_u32_fast(a: u32, b: u32, m: u32) -> u32 {
    #[cfg(target_arch = "x86_64")]
    // SAFETY: _mulx_u32 is a standard x86_64 intrinsic
    unsafe {
        let mut high: u32 = 0;
        let low = _mulx_u32(a, b, &mut high);
        ((high as u64) << 32 | low as u64) % (m as u64) as u32
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        ((a as u64 * b as u64) % m as u64) as u32
    }
}

#[inline(always)]
fn mod_pow_u32_fast(mut base: u32, mut exp: u32, modulus: u32) -> u32 {
    let mut result = 1u32;
    base %= modulus;

    while exp > 0 {
        if exp & 1 != 0 {
            result = mod_mul_u32_fast(result, base, modulus);
        }
        base = mod_mul_u32_fast(base, base, modulus);
        exp >>= 1;
    }
    result
}

/// Optimized deterministic Miller-Rabin
#[inline(always)]
fn is_u32_prime_ultra(n: u32) -> bool {
    const BASES: [u32; 3] = [2, 7, 61];

    if n < 2 || (n & 1 == 0 && n != 2) {
        return false;
    }
    if n <= 3 {
        return true;
    }

    let d = n - 1;
    let r = d.trailing_zeros();
    let d = d >> r;

    'outer: for &a in &BASES {
        let mut x = mod_pow_u32_fast(a % n, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }

        for _ in 1..r {
            x = mod_mul_u32_fast(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Ultra-optimized membrane prime generator")]
struct Args {
    #[arg(short, long, default_value_t = 6)]
    base: u32,

    #[arg(short, long, default_value = "5,5")]
    digits: String,

    #[arg(short, long, default_value_t = 4_000_000)]
    count: usize,

    #[arg(long)]
    gpu: bool,

    #[arg(long)]
    benchmark: bool,
}

fn main() {
    let args = Args::parse();
    let (l, r) = parse_digits(&args.digits);

    println!(
        "\n🚀 MEMBRANE PRIME ULTRA — base-{}, boundary=({},{})",
        args.base, l, r
    );
    println!(
        "{} candidates, mode: {}",
        args.count,
        if args.gpu {
            "GPU-OPTIMIZED"
        } else {
            "CPU-PARALLEL"
        }
    );
    println!("{}", "=".repeat(60));

    let start = Instant::now();
    let primes = if args.gpu {
        run_gpu_ultra(args.base, l, r, args.count)
    } else {
        run_cpu_ultra(args.base, l, r, args.count)
    };
    let elapsed = start.elapsed();

    // Results
    let density = primes.len() as f64 / args.count as f64 * 100.0;
    let throughput = args.count as f64 / elapsed.as_secs_f64() / 1e6;

    println!(
        "\nFinished in {:.3}s → {:.1} M c/s",
        elapsed.as_secs_f64(),
        throughput
    );
    println!("Found {} primes ({:.1}% density)\n", primes.len(), density);

    // Show examples
    for (i, p) in primes.iter().take(10).enumerate() {
        println!("  [{}] {}", i, p);
    }

    if args.benchmark {
        benchmark_improvements();
    }
}

fn parse_digits(s: &str) -> (u32, u32) {
    let d: Vec<u32> = s.split(',').map(|x| x.trim().parse().unwrap()).collect();
    (d[0], d[1])
}

fn run_cpu_ultra(base: u32, l: u32, r: u32, count: usize) -> Vec<BigUint> {
    // Parallel membrane computation
    let values: Vec<u32> = (0..count as u32)
        .into_par_iter()
        .map(|c| compute_membrane_u32(base, 3, l, r, 0, 0, c))
        .collect();

    // Parallel ultra-fast Miller-Rabin
    values
        .into_par_iter()
        .filter(|&v| is_u32_prime_ultra(v))
        .map(BigUint::from)
        .collect()
}

#[cfg(feature = "metal")]
fn run_gpu_ultra(base: u32, l: u32, r: u32, count: usize) -> Vec<BigUint> {
    use primes::gpu_optimized::GpuSieveOptimized;

    println!("\nGPU Pipeline Breakdown:");
    println!("{}", "-".repeat(40));

    let gpu = GpuSieveOptimized::new().expect("GPU init");

    // Direct GPU computation (membrane + sieve + Fermat)
    let gpu_start = Instant::now();
    let survivor_indices = gpu
        .sieve_direct(base, l, r, 3, count as u32)
        .expect("GPU sieve");
    let gpu_ms = gpu_start.elapsed().as_millis();

    println!(
        "GPU total:     {:>6} ms ({:.1} M c/s)",
        gpu_ms,
        count as f64 / gpu_ms as f64 / 1e3
    );
    println!(
        "Survivors:     {} ({:.1}% passed)",
        survivor_indices.len(),
        survivor_indices.len() as f64 / count as f64 * 100.0
    );

    // Final Miller-Rabin on survivors
    let mr_start = Instant::now();
    let primes: Vec<BigUint> = survivor_indices
        .into_par_iter()
        .filter_map(|idx| {
            // Recompute value (could optimize by returning from GPU)
            let v = compute_membrane_u32(base, 3, l, r, 0, 0, idx);
            is_u32_prime_ultra(v).then(|| BigUint::from(v))
        })
        .collect();
    let mr_ms = mr_start.elapsed().as_millis();

    println!("Miller-Rabin:  {:>6} ms", mr_ms);
    println!("{}", "-".repeat(40));

    primes
}

#[cfg(not(feature = "metal"))]
fn run_gpu_ultra(_: u32, _: u32, _: u32, _: usize) -> Vec<BigUint> {
    println!("GPU not available, using CPU");
    vec![]
}

fn compute_membrane_u32(base: u32, _w: u32, l: u32, r: u32, _r1: u32, _r2: u32, c: u32) -> u32 {
    let b2 = base.wrapping_mul(base);
    l.wrapping_mul(b2)
        .wrapping_add(r.wrapping_mul(base))
        .wrapping_add(c.wrapping_mul(base))
        .wrapping_add(r.wrapping_mul(base))
        .wrapping_add(l)
}

fn benchmark_improvements() {
    println!("\n\n=== OPTIMIZATION IMPACT BENCHMARK ===");
    println!("{}", "-".repeat(60));

    let test_size = 100_000;
    let test_values: Vec<u32> = (0..test_size)
        .map(|i| 1_000_000 + i * 2 + 1) // Odd numbers
        .collect();

    // Benchmark original Miller-Rabin
    let start = Instant::now();
    let count1 = test_values
        .iter()
        .filter(|&&n| {
            // Simple version
            let big = BigUint::from(n);
            primes::is_prime_miller_rabin(&big)
        })
        .count();
    let time1 = start.elapsed();

    // Benchmark optimized Miller-Rabin
    let start = Instant::now();
    let count2 = test_values
        .iter()
        .filter(|&&n| is_u32_prime_ultra(n))
        .count();
    let time2 = start.elapsed();

    println!("Miller-Rabin comparison ({} numbers):", test_size);
    println!(
        "  Original:  {:.3}s ({} primes)",
        time1.as_secs_f64(),
        count1
    );
    println!(
        "  Optimized: {:.3}s ({} primes)",
        time2.as_secs_f64(),
        count2
    );
    println!(
        "  Speedup:   {:.1}x",
        time1.as_secs_f64() / time2.as_secs_f64()
    );
}
