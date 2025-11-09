//! Membrane Sieve Optimizer
//! Integrating fast modular sieving with our membrane polynomial structure

use primes::is_prime_miller_rabin;
use num_bigint::BigUint;
use num_traits::Zero;
use std::collections::HashMap;
use std::time::Instant;
fn main() {
    println!("MEMBRANE SIEVE OPTIMIZATION");
    println!("==========================\n");
    
    // Compare naive vs sieved approaches
    compare_approaches();
    // Show how sieving scales
    demonstrate_sieve_scaling();
    // Find massive primes efficiently
    hunt_large_primes_with_sieve();
}
fn compare_approaches() {
    println!("1. NAIVE VS SIEVED MEMBRANE SEARCH");
    println!("----------------------------------\n");
    let base = 10;
    let outer = 3;
    let inner = 7;
    let k = 0;
    let seed_range = 1..=10000;
    // Naive approach
    let start = Instant::now();
    let mut naive_primes = 0;
    for seed in seed_range.clone() {
        let membrane = construct_membrane(outer, inner, seed, k, k, base);
        if is_prime_miller_rabin(&membrane) {
            naive_primes += 1;
        }
    }
    let naive_time = start.elapsed();
    // Sieved approach
    let sieved_primes = sieved_membrane_search(outer, inner, k, seed_range, base);
    let sieve_time = start.elapsed();
    println!("Naive approach:");
    println!("  Primes found: {}", naive_primes);
    println!("  Time: {:?}", naive_time);
    println!("  Rate: {:.0} candidates/sec\n", 10000.0 / naive_time.as_secs_f64());
    println!("Sieved approach:");
    println!("  Primes found: {}", sieved_primes.len());
    println!("  Time: {:?}", sieve_time);
    println!("  Rate: {:.0} candidates/sec", 10000.0 / sieve_time.as_secs_f64());
    println!("  Speedup: {:.1}x\n", naive_time.as_secs_f64() / sieve_time.as_secs_f64());
    if !sieved_primes.is_empty() {
        println!("First few primes found:");
        for prime in sieved_primes.iter().take(5) {
            println!("  {}", prime);
fn demonstrate_sieve_scaling() {
    println!("\n\n2. SIEVE SCALING WITH POLYNOMIAL DEGREE");
    println!("---------------------------------------\n");
    let configs = vec![
        (3, 7, 0, "Standard membrane"),
        (3, 7, 1, "Breathing membrane"), 
        (3, 7, 2, "Extended membrane"),
    ];
    for (outer, inner, k, desc) in configs {
        println!("\n{} (k={}):", desc, k);
        
        // Polynomial degree increases with k
        let degree = 4 + 4 * k;
        println!("  Polynomial degree: {}", degree);
        // Pre-compute modular signatures
        let sieve_limit = 100;
        let mut signatures = HashMap::new();
        for p in 2..sieve_limit {
            if is_prime_u32(p) {
                let sig = compute_membrane_signature(outer, inner, k, base, p);
                signatures.insert(p, sig);
            }
        println!("  Pre-computed {} prime signatures", signatures.len());
        // Quick sieve test
        let mut eliminated = 0;
        let test_seeds = 1000;
        for seed in 1..=test_seeds {
            for (&p, &sig) in &signatures {
                let g = modular_pow(base, 2 + 2 * k, p); // Growth factor
                if (sig.0 + seed * g) % p == 0 {
                    eliminated += 1;
                    break;
                }
        let survival_rate = (test_seeds - eliminated) as f64 / test_seeds as f64;
        println!("  Survival rate after sieve: {:.1}%", survival_rate * 100.0);
        println!("  Computational savings: {:.0}x", 1.0 / survival_rate);
fn hunt_large_primes_with_sieve() {
    println!("\n\n3. HUNTING LARGE PRIMES WITH SIEVE");
    // Target: Find primes with 20+ digits using sieve
        (3, 7, 8, 1_000_000),   // Very sparse polynomial
        (1, 9, 10, 1_000_000),  // Ultra-sparse
        (5, 7, 12, 1_000_000),  // Self-inverse in base 12
    for (outer, inner, k, seed_limit) in configs {
        println!("\nConfiguration ({},{}) with k={}:", outer, inner, k);
        println!("  Polynomial degree: {}", 4 + 4 * k);
        let start = Instant::now();
        let mut candidates_tested = 0;
        let mut primes_found = Vec::new();
        // Batch sieve
        let batch_size = 10_000;
        for batch_start in (1..seed_limit).step_by(batch_size) {
            let batch_end = (batch_start + batch_size).min(seed_limit);
            
            // Sieve this batch
            let survivors = batch_sieve(outer, inner, k, batch_start as u32..batch_end as u32, base);
            candidates_tested += survivors.len();
            // Test survivors for primality
            for seed in survivors {
                let membrane = construct_membrane(outer, inner, seed, k, k, base);
                if membrane.to_string().len() >= 20 && is_prime_miller_rabin(&membrane) {
                    primes_found.push((seed, membrane));
                    if primes_found.len() >= 3 {
                        break;
                    }
            if primes_found.len() >= 3 {
                break;
        let elapsed = start.elapsed();
        println!("  Seeds checked: {}", batch_size * (candidates_tested / batch_size));
        println!("  Candidates after sieve: {}", candidates_tested);
        println!("  Sieve elimination: {:.1}%", 
            (1.0 - candidates_tested as f64 / (batch_size as f64)) * 100.0);
        println!("  Time: {:?}", elapsed);
        if !primes_found.is_empty() {
            println!("  Large primes found:");
            for (seed, prime) in &primes_found {
                println!("    Seed {}: {} digits", seed, prime.to_string().len());
                if prime.to_string().len() <= 50 {
                    println!("      {}", prime);
                } else {
                    println!("      {}...{}", 
                        &prime.to_string()[..25],
                        &prime.to_string()[prime.to_string().len()-25..]);
        } else {
            println!("  No large primes found in this configuration");
    println!("\n✅ Sieving enables efficient search of sparse, high-degree polynomials!");
// Helper functions
fn construct_membrane(outer: u32, inner: u32, seed: u32, k_outer: usize, k_inner: usize, base: u32) -> BigUint {
    let mut digits = vec![outer];
    for _ in 0..k_outer { digits.push(0); }
    digits.push(inner);
    for _ in 0..k_inner { digits.push(0); }
    digits.push(seed);
    digits.push(outer);
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);
    for &digit in &digits {
        result = result * &base_big + BigUint::from(digit);
    result
fn sieved_membrane_search(outer: u32, inner: u32, k: usize, seeds: std::ops::RangeInclusive<u32>, base: u32) -> Vec<BigUint> {
    let mut primes = Vec::new();
    // Pre-compute signatures for small primes
    let sieve_primes: Vec<u32> = (2..500).filter(|&p| is_prime_u32(p)).collect();
    let signatures: Vec<(u32, u32)> = sieve_primes.iter()
        .map(|&p| compute_membrane_signature(outer, inner, k, base, p))
        .collect();
    for seed in seeds {
        // Quick modular checks
        let mut passes_sieve = true;
        for (i, &p) in sieve_primes.iter().enumerate() {
            let (sig, g) = signatures[i];
            if (sig + seed * g) % p == 0 {
                passes_sieve = false;
        if passes_sieve {
            let membrane = construct_membrane(outer, inner, seed, k, k, base);
            if is_prime_miller_rabin(&membrane) {
                primes.push(membrane);
    primes
fn compute_membrane_signature(outer: u32, inner: u32, k: usize, base: u32, p: u32) -> (u32, u32) {
    // For membrane polynomial P(x) = outer*x^n + inner*x^(n-1-k) + C*x^(n/2) + ...
    // We compute signature = (outer*base^n + inner*base^(n-1-k) + ...) mod p (without C term)
    // And growth factor g = base^(n/2) mod p
    let n = 4 + 4 * k; // polynomial degree
    let sig = (outer * modular_pow(base, n, p) + 
               inner * modular_pow(base, n - 1 - k, p) +
               inner * modular_pow(base, 1 + k, p) + 
               outer) % p;
    let g = modular_pow(base, n / 2, p);
    (sig, g)
fn batch_sieve(outer: u32, inner: u32, k: usize, seeds: std::ops::Range<u32>, base: u32) -> Vec<u32> {
    let mut survivors = Vec::new();
    // Small primes for sieving
    let sieve_primes: Vec<u32> = (2..100).filter(|&p| is_prime_u32(p)).collect();
    'seed_loop: for seed in seeds {
        for &p in &sieve_primes {
            let (sig, g) = compute_membrane_signature(outer, inner, k, base, p);
                continue 'seed_loop; // Eliminated by sieve
        survivors.push(seed);
    survivors
fn modular_pow(base: u32, exp: usize, modulus: u32) -> u32 {
    let mut result = 1u64;
    let mut base = base as u64;
    let mut exp = exp;
    let modulus = modulus as u64;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        base = (base * base) % modulus;
        exp /= 2;
    result as u32
fn is_prime_u32(n: u32) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    for i in (3..=(n as f64).sqrt() as u32).step_by(2) {
        if n % i == 0 { return false; }
    true
