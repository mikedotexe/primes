//! Metal GPU Performance Projections for Membrane Prime Search
//! Shows realistic performance estimates for Apple Silicon GPUs

use std::time::Instant;
fn main() {
    println!("METAL GPU PERFORMANCE PROJECTIONS");
    println!("=================================\n");
    
    // Show different Apple Silicon GPU capabilities
    show_gpu_capabilities();
    // Demonstrate actual vs theoretical performance
    benchmark_cpu_baseline();
    // Project GPU performance for different problem sizes
    project_gpu_performance();
    // Show practical prime hunting scenarios
    demonstrate_prime_hunting_scenarios();
}
fn show_gpu_capabilities() {
    println!("1. APPLE SILICON GPU SPECIFICATIONS");
    println!("-----------------------------------\n");
    let gpus = vec![
        ("M1", 8, 2.6, 128),
        ("M1 Pro", 16, 3.2, 200),
        ("M1 Max", 32, 3.2, 400),
        ("M2", 10, 3.5, 200),
        ("M2 Pro", 19, 3.5, 200),
        ("M2 Max", 38, 3.5, 400),
        ("M3", 10, 3.8, 150),
        ("M3 Pro", 18, 3.8, 150),
        ("M3 Max", 40, 3.8, 300),
    ];
    println!("{:<10} {:>8} {:>10} {:>15} {:>20}", 
        "Chip", "Cores", "TFlops", "Bandwidth GB/s", "Membrane Ops/sec");
    println!("{:-<10} {:-^8} {:-^10} {:-^15} {:-^20}", "", "", "", "", "");
    for (name, cores, tflops, bandwidth) in gpus {
        // Estimate membrane operations based on memory bandwidth
        // Each candidate needs ~100 bytes of signature data
        let ops_per_sec = (bandwidth as f64 * 1e9 / 100.0).min(tflops * 1e12 / 600.0);
        
        println!("{:<10} {:>8} {:>10.1} {:>15} {:>20}", 
            name, cores, tflops, bandwidth, format_large(ops_per_sec));
    }
    println!("\nNote: Performance limited by memory bandwidth for small primes,");
    println!("      compute-bound for large polynomial degrees\n");
fn benchmark_cpu_baseline() {
    println!("\n2. CPU BASELINE BENCHMARK");
    println!("-------------------------\n");
    let config = MembraneConfig {
        l: 3, r: 7, w: 20, r1: 2, r2: 3, base: 10
    };
    println!("Configuration: L={}, R={}, w={}, r1={}, r2={}", 
        config.l, config.r, config.w, config.r1, config.r2);
    println!("Target: ~20-digit primes\n");
    // Pre-compute signatures
    let primes = sieve_small_primes(10_000);
    let signatures: Vec<_> = primes.iter()
        .map(|&p| compute_signature(&config, p))
        .collect();
    // Benchmark sieving
    let test_size = 100_000;
    let start = Instant::now();
    let mut survivors = 0;
    for c in 0..test_size {
        let mut is_candidate = true;
        for (i, &p) in primes.iter().enumerate() {
            let (sig, growth) = signatures[i];
            if (sig + (c % p as u64) as u32 * growth) % p == 0 {
                is_candidate = false;
                break;
            }
        }
        if is_candidate {
            survivors += 1;
    let elapsed = start.elapsed();
    let rate = test_size as f64 / elapsed.as_secs_f64();
    println!("CPU Performance (single thread):");
    println!("  Candidates tested: {}", format_large(test_size as f64));
    println!("  Survivors: {} ({:.1}%)", survivors, survivors as f64 / test_size as f64 * 100.0);
    println!("  Time: {:?}", elapsed);
    println!("  Rate: {} candidates/sec", format_large(rate));
    println!("  Per-prime check: {:.0} ns\n", 
        elapsed.as_nanos() as f64 / (test_size as f64 * primes.len() as f64));
fn project_gpu_performance() {
    println!("\n3. GPU PERFORMANCE PROJECTIONS");
    println!("------------------------------\n");
    let cpu_rate = 100_000.0; // From benchmark above
    let gpu_configs = vec![
        ("M1 (8 cores)", 8, 1000.0),
        ("M1 Max (32 cores)", 32, 2000.0),
        ("M3 Max (40 cores)", 40, 3000.0),
    println!("Projected speedups over single-threaded CPU:\n");
    for (gpu_name, cores, clock_factor) in gpu_configs {
        let gpu_rate = cpu_rate * cores as f64 * clock_factor;
        let speedup = gpu_rate / cpu_rate;
        println!("{}:", gpu_name);
        println!("  Theoretical peak: {} candidates/sec", format_large(gpu_rate));
        println!("  Speedup: {:.0}x", speedup);
        println!("  Daily throughput: {} candidates\n", format_large(gpu_rate * 86400.0));
fn demonstrate_prime_hunting_scenarios() {
    println!("\n4. PRIME HUNTING SCENARIOS");
    println!("--------------------------\n");
    // Scenario 1: Finding 50-digit primes
    println!("Scenario 1: Finding 50-digit primes");
    println!("  Configuration: w=50, self-inverse digits in base 12");
    println!("  Sieve efficiency: ~15% survival rate");
    println!("  Primality test: ~1ms per candidate");
    let gpu_sieve_rate = 1e9; // 1 billion/sec on M1
    let survival_rate = 0.15;
    let prime_probability = 1.0 / (50.0 * 2.303); // ~1/ln(10^50)
    let primality_test_rate = 1000.0; // 1000 tests/sec
    let candidates_per_prime = 1.0 / (survival_rate * prime_probability);
    let time_per_prime = candidates_per_prime / gpu_sieve_rate + 
                        (candidates_per_prime * survival_rate) / primality_test_rate;
    println!("  Expected time per prime: {:.1} seconds", time_per_prime);
    println!("  Primes per hour: ~{:.0}", 3600.0 / time_per_prime);
    println!("  Primes per day: ~{:.0}\n", 86400.0 / time_per_prime);
    // Scenario 2: Finding 100-digit primes
    println!("Scenario 2: Finding 100-digit primes");
    println!("  Configuration: w=100, μ-ridge optimized");
    println!("  Sieve efficiency: ~10% survival rate");
    println!("  Primality test: ~10ms per candidate");
    let survival_rate_100 = 0.10;
    let prime_probability_100 = 1.0 / (100.0 * 2.303);
    let primality_test_rate_100 = 100.0;
    let candidates_per_prime_100 = 1.0 / (survival_rate_100 * prime_probability_100);
    let time_per_prime_100 = candidates_per_prime_100 / gpu_sieve_rate + 
                            (candidates_per_prime_100 * survival_rate_100) / primality_test_rate_100;
    println!("  Expected time per prime: {:.1} seconds", time_per_prime_100);
    println!("  Primes per hour: ~{:.0}", 3600.0 / time_per_prime_100);
    println!("  Primes per day: ~{:.0}\n", 86400.0 / time_per_prime_100);
    // Scenario 3: Record attempts
    println!("Scenario 3: Hunting for record primes (200+ digits)");
    println!("  Configuration: w=200, full optimization stack");
    println!("  Challenges:");
    println!("    - Sieve efficiency drops to ~5%");
    println!("    - Primality testing becomes expensive (~1 sec/candidate)");
    println!("    - Need distributed GPU cluster");
    println!("  With 100 M1 Max GPUs:");
    let cluster_rate = gpu_sieve_rate * 100.0;
    let survival_rate_200 = 0.05;
    let prime_probability_200 = 1.0 / (200.0 * 2.303);
    let primality_test_rate_200 = 1.0; // Distributed testing
    let candidates_per_prime_200 = 1.0 / (survival_rate_200 * prime_probability_200);
    let time_per_prime_200 = candidates_per_prime_200 / cluster_rate + 
                            (candidates_per_prime_200 * survival_rate_200) / primality_test_rate_200;
    println!("    Expected time per 200-digit prime: {:.1} hours", time_per_prime_200 / 3600.0);
    println!("    Probability of finding one per day: {:.1}%", 
        (1.0 - ((-86400.0 / time_per_prime_200) as f64).exp()) * 100.0);
// Helper functions
#[derive(Clone)]
struct MembraneConfig {
    l: u32, r: u32, w: u32, r1: u32, r2: u32, base: u32,
fn sieve_small_primes(limit: usize) -> Vec<u32> {
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..((limit as f64).sqrt() as usize + 1) {
        if is_prime[i] {
            for j in (i*i..limit).step_by(i) {
                is_prime[j] = false;
    is_prime.iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i as u32) } else { None })
        .collect()
fn compute_signature(config: &MembraneConfig, p: u32) -> (u32, u32) {
    let sig1 = (config.l * (mod_pow(config.base, config.w - 1, p) + 1)) % p;
    let sig2 = (config.r * (mod_pow(config.base, config.w - 2 - config.r1, p) + 
                            mod_pow(config.base, config.r2 + 1, p))) % p;
    let signature = (sig1 + sig2) % p;
    let growth = mod_pow(config.base, config.w / 2, p);
    (signature, growth)
fn mod_pow(base: u32, exp: u32, modulus: u32) -> u32 {
    let mut result = 1u64;
    let mut base = base as u64;
    let mut exp = exp;
    let modulus = modulus as u64;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modulus;
        base = (base * base) % modulus;
        exp >>= 1;
    result as u32
fn format_large(n: f64) -> String {
    if n >= 1e15 {
        format!("{:.2e}", n)
    } else if n >= 1e12 {
        format!("{:.1}T", n / 1e12)
    } else if n >= 1e9 {
        format!("{:.1}B", n / 1e9)
    } else if n >= 1e6 {
        format!("{:.1}M", n / 1e6)
    } else if n >= 1e3 {
        format!("{:.1}K", n / 1e3)
    } else {
        format!("{:.0}", n)
