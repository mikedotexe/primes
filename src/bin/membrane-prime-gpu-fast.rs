//! GPU-accelerated membrane prime generator - rayon edition (2025-07-17)

use clap::Parser;
use num_bigint::BigUint;
use rayon::prelude::*;
use std::time::Instant;


/// CLI --------------------------------------------------------------------
#[derive(Parser, Debug)]
#[command(author, version, about = "GPU-accelerated membrane prime generator")]
struct Args {
    /// Numeric base (6 and 12 give dense membranes)
    #[arg(short, long, default_value_t = 6)]
    base: u32,
    /// Boundary digits "l,r"
    #[arg(short, long, default_value = "5,5")]
    digits: String,
    /// Candidate count
    #[arg(short, long, default_value_t = 1_000_000)]
    count: usize,
    /// Launch GPU sieve
    #[arg(long)]
    gpu: bool,
}

fn main() {
    let args = Args::parse();
    let (l, r) = {
        let d: Vec<u32> = args
            .digits
            .split(',')
            .map(|s| s.trim().parse().expect("digit"))
            .collect();
        (d[0], d[1])
    };

    println!("\n🚀 MEMBRANE PRIME  —  base-{}, boundary=({},{})", args.base, l, r);
    println!("{} candidates, mode: {}", args.count, if args.gpu { "GPU" } else { "CPU" });
    println!("============================================================\n");

    let t0 = Instant::now();
    let primes = if args.gpu {
        run_gpu_pipeline(args.base, l, r, args.count)
    } else {
        run_cpu_pipeline(args.base, l, r, args.count)
    };
    let wall = t0.elapsed();

    // ---------------------------------------------------------------------
    let density    = primes.len() as f64 / args.count as f64 * 100.0;
    let throughput = args.count as f64 / wall.as_secs_f64();
    println!("\nFinished in {:.3}s  ⇒  {:.1} M c/s", wall.as_secs_f64(), throughput / 1e6);
    println!("Found {} primes ({:.1} % density)\n", primes.len(), density);

    for (i, p) in primes.iter().take(10).enumerate() {
        println!("  [{}] {}", i, p);
    }
}

/// ------------------------------------------------------------------------
/// SIMD-safe, deterministic Miller-Rabin bases for 32-bit range
///
/// Reference: Gerbicz & Jaeschke (2, 7, 61 strong MR covers <2³²)
#[inline]
fn is_u32_prime_det(n: u32) -> bool {
    const BASES: [u32; 3] = [2, 7, 61];
    if n < 2 || (n & 1 == 0 && n != 2) {
        return false;
    }
    let d = (n - 1).trailing_zeros();
    let s = (n - 1) >> d;
    'outer: for &a in &BASES {
        let mut x = mod_pow_u32(a % n, s, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..d {
            x = mod_mul_u32(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

#[inline(always)]
fn mod_mul_u32(a: u32, b: u32, m: u32) -> u32 {
    ((a as u64 * b as u64) % m as u64) as u32
}
#[inline(always)]
fn mod_pow_u32(mut base: u32, mut exp: u32, modu: u32) -> u32 {
    let mut acc = 1u32;
    while exp != 0 {
        if exp & 1 != 0 {
            acc = mod_mul_u32(acc, base, modu);
        }
        base = mod_mul_u32(base, base, modu);
        exp >>= 1;
    }
    acc
}

/// ------------------------------------------------------------------------
fn run_cpu_pipeline(base: u32, l: u32, r: u32, count: usize) -> Vec<BigUint> {
    println!("CPU pipeline …");
    let mem: Vec<u32> = (0..count as u32)
        .into_par_iter()
        .map(|c| compute_membrane_u32(base, 3, l, r, 0, 0, c))
        .collect();

    mem.into_par_iter()
        .filter(|&v| is_u32_prime_det(v))
        .map(BigUint::from)
        .collect()
}

/// ------------------------------------------------------------------------
#[cfg(feature = "metal")]
fn run_gpu_pipeline(base: u32, l: u32, r: u32, count: usize) -> Vec<BigUint> {
    use prime_physics_engine::gpu::GpuSieve;

    // ------------ pre-compute membrane values (parallel CPU) -------------
    let pre_t0 = Instant::now();
    let values: Vec<u32> = (0..count as u32)
        .into_par_iter()
        .map(|c| compute_membrane_u32(base, 3, l, r, 0, 0, c))
        .collect();
    let pre_ms = pre_t0.elapsed().as_secs_f64() * 1e3;
    println!("membrane[]  {:>6.1} ms   ({:.1} M/s)",
             pre_ms, count as f64 / pre_ms / 1e3);

    // --------------------------- GPU sieve -------------------------------
    let gpu = GpuSieve::new().expect("GPU init");
    let gpu_t0 = Instant::now();
    let survivors = gpu.sieve(&values, base).expect("GPU sieve");
    let gpu_ms = gpu_t0.elapsed().as_secs_f64() * 1e3;
    let gpu_secs = gpu_t0.elapsed().as_secs_f64();
    println!("GPU sieve   {:>6.1} ms   ({:.1} M c/s)",
             gpu_ms, count as f64 / gpu_secs / 1e6);
    println!("survivors   {}\n", survivors.len());

    // --------- Miller-Rabin on survivors (parallel, 32-bit) --------------
    let survivor_count = survivors.len();
    let mr_t0 = Instant::now();
    let primes = survivors
        .into_par_iter()
        .filter_map(|idx| {
            let v = values[idx as usize];
            is_u32_prime_det(v).then(|| BigUint::from(v))
        })
        .collect();
    let mr_ms = mr_t0.elapsed().as_secs_f64() * 1e3;
    println!("MR-filter   {:>6.1} ms   ({:.1} M/s)",
             mr_ms, survivor_count as f64 / mr_ms / 1e3);
    
    primes
}

#[cfg(not(feature = "metal"))]
fn run_gpu_pipeline(base: u32, l: u32, r: u32, count: usize) -> Vec<BigUint> {
    println!("GPU not available, falling back to CPU");
    run_cpu_pipeline(base, l, r, count)
}

/// ------------------------------------------------------------------------
fn compute_membrane_u32(
    base: u32,
    _w:   u32,
    l:    u32,
    r:    u32,
    r1:   u32,
    r2:   u32,
    c:    u32,
) -> u32 {
    let b2 = base.wrapping_mul(base); // base²
    l.wrapping_mul(b2)                                   // l·b²
        .wrapping_add(r.wrapping_mul(base.pow(1 - r1)))   // r·b^{1-r1}
        .wrapping_add(c.wrapping_mul(base))               // c·b^{w/2}; w=3 ⇒ b¹
        .wrapping_add(r.wrapping_mul(base.pow(r2 + 1)))   // r·b^{r2+1}
        .wrapping_add(l)
}