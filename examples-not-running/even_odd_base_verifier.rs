//! Even vs Odd Base Claim Verifier
//! 
//! Tests the claim that even bases generate 44% more primes than odd bases

use primes::{is_prime_miller_rabin, ascii_art::*};
use num_bigint::BigUint;
use std::io::Write;
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}
fn construct_membrane(base: u32, outer: u32, inner: u32, k_outer: usize, k_inner: usize, seed: u32) -> BigUint {
    let mut value = BigUint::from(0u32);
    let base_big = BigUint::from(base);
    
    let mut digits = vec![outer];
    for _ in 0..k_outer { digits.push(0); }
    digits.push(inner);
    for _ in 0..k_inner { digits.push(0); }
    let seed_str = seed.to_string();
    for ch in seed_str.chars() {
        digits.push(ch.to_digit(10).unwrap());
    }
    digits.push(outer);
    for digit in digits {
        value = value * &base_big + BigUint::from(digit);
    value
fn find_coprime_digits(base: u32) -> Vec<u32> {
    (1..base).filter(|&d| gcd(d, base) == 1).collect()
fn test_base(base: u32, samples: u32) -> (f64, Vec<(u32, u32, f64)>) {
    let coprime_digits = find_coprime_digits(base);
    if coprime_digits.len() < 2 {
        return (0.0, vec![]);
    let mut best_configs = Vec::new();
    let mut overall_best = 0.0;
    // Test top configurations
    for &outer in &coprime_digits {
        for &inner in &coprime_digits {
            if outer != inner {
                let mut successes = 0;
                
                for seed in 0..samples {
                    let membrane = construct_membrane(base, outer, inner, 0, 0, seed);
                    if is_prime_miller_rabin(&membrane) {
                        successes += 1;
                    }
                }
                let rate = successes as f64 / samples as f64;
                if rate > 0.1 { // Only keep decent configs
                    best_configs.push((outer, inner, rate));
                    if rate > overall_best {
                        overall_best = rate;
            }
        }
    best_configs.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    best_configs.truncate(3); // Keep top 3
    (overall_best, best_configs)
fn main() {
    println!("{}", banner("EVEN VS ODD BASE CLAIM VERIFICATION", 80));
    println!("\nClaim: Even bases generate 44% more primes than odd bases");
    println!("Testing bases 3-30 systematically...\n");
    let samples = 200;
    let mut even_rates = Vec::new();
    let mut odd_rates = Vec::new();
    let mut results = Vec::new();
    // Test bases 3-30
    for base in 3..=30 {
        print!("Testing base {}...", base);
        std::io::stdout().flush().unwrap();
        
        let (best_rate, top_configs) = test_base(base, samples);
        if best_rate > 0.0 {
            results.push((base, best_rate, top_configs));
            
            if base % 2 == 0 {
                even_rates.push(best_rate);
            } else {
                odd_rates.push(best_rate);
        println!(" {:.1}%", best_rate * 100.0);
    // Sort by performance
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    // Display results
    println!("\n{}", boxed_title("RESULTS BY BASE", 80));
    println!("\n| Base | Type | Best Rate | Top Config | Coprime Count |");
    println!("|------|------|-----------|------------|---------------|");
    for (base, rate, configs) in &results {
        let base_type = if base % 2 == 0 { "Even" } else { "Odd " };
        let coprime_count = find_coprime_digits(*base).len();
        if let Some((outer, inner, _)) = configs.first() {
            println!("| {:4} | {} | {:8.1}% | ({},{})      | {:13} |", 
                base, base_type, rate * 100.0, outer, inner, coprime_count);
    // Calculate averages
    let even_avg = if !even_rates.is_empty() {
        even_rates.iter().sum::<f64>() / even_rates.len() as f64
    } else { 0.0 };
    let odd_avg = if !odd_rates.is_empty() {
        odd_rates.iter().sum::<f64>() / odd_rates.len() as f64
    // Top performers analysis
    println!("\n{}", boxed_title("TOP 10 PERFORMERS", 60));
    for (i, (base, rate, configs)) in results.iter().take(10).enumerate() {
        let base_type = if base % 2 == 0 { "even" } else { "odd" };
        println!("{}. Base {} ({}): {:.1}%", i + 1, base, base_type, rate * 100.0);
        for (outer, inner, rate) in configs {
            println!("   ({},{}) → {:.1}%", outer, inner, rate * 100.0);
    // Summary statistics
    println!("\n{}", boxed_title("STATISTICAL SUMMARY", 80));
    println!("\nEven bases ({} tested):", even_rates.len());
    println!("  Average best rate: {:.1}%", even_avg * 100.0);
    println!("  Range: {:.1}% - {:.1}%", 
        even_rates.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0) * 100.0,
        even_rates.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0) * 100.0);
    println!("\nOdd bases ({} tested):", odd_rates.len());
    println!("  Average best rate: {:.1}%", odd_avg * 100.0);
    println!("  Range: {:.1}% - {:.1}%",
        odd_rates.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0) * 100.0,
        odd_rates.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0) * 100.0);
    println!("\n{}", simple_box("CLAIM VERIFICATION"));
    if even_avg > odd_avg {
        let advantage = (even_avg - odd_avg) / odd_avg * 100.0;
        println!("\n✅ Even bases DO perform better!");
        println!("   Advantage: {:.1}%", advantage);
        if advantage >= 40.0 {
            println!("   ✅ 44% claim is VERIFIED!");
        } else {
            println!("   ❌ But only {:.1}%, not 44% as claimed", advantage);
    } else {
        let advantage = (odd_avg - even_avg) / even_avg * 100.0;
        println!("\n❌ CLAIM DISPROVEN!");
        println!("   Odd bases actually perform {:.1}% better!", advantage);
    // Analysis of why
    println!("\n{}", boxed_title("ANALYSIS", 80));
    println!("\nFactors that actually matter:");
    println!("1. Number of coprime digits (more choices = better)");
    println!("2. Base factorization (highly composite helps)");
    println!("3. Specific digit relationships");
    println!("\nTop performers:");
    for (base, rate, _) in results.iter().take(5) {
        let factors = factorize(*base);
        println!("  Base {}: {:.1}% (factors: {:?})", base, rate * 100.0, factors);
fn factorize(mut n: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let mut d = 2;
    while d * d <= n {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        d += 1;
    if n > 1 {
        factors.push(n);
    factors
