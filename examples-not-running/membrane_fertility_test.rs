//! Membrane Fertility Test - Finding the most productive configurations
//! Let's push the boundaries and find the ultimate prime-generating membranes!

use num_bigint::BigUint;
use primes::is_prime_miller_rabin;
use rayon::prelude::*;
use std::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::io::{self, Write};
fn main() {
    println!("🌸 MEMBRANE FERTILITY TEST 🌸");
    println!("=============================\n");
    
    println!("Testing extreme configurations to find the most fertile membranes...\n");
    // Test 1: Ultra-minimal membranes in small bases
    println!("🔬 Experiment 1: Ultra-Minimal Membranes");
    test_ultra_minimal();
    // Test 2: Self-inverse digit combinations 
    println!("\n🔬 Experiment 2: Self-Inverse Digit Pairs");
    test_self_inverse_pairs();
    // Test 3: Twin membrane hunting
    println!("\n🔬 Experiment 3: Twin Prime Membranes");
    test_twin_membranes();
    // Test 4: Golden ratio inspired configurations
    println!("\n🔬 Experiment 4: Golden Ratio Configurations");
    test_golden_ratio();
}
fn test_ultra_minimal() {
    let configs = vec![
        (3, 1, 1, "Base-3: 1[C]1"),
        (4, 1, 1, "Base-4: 1[C]1"),
        (4, 1, 3, "Base-4: 1[C]3"),
        (5, 1, 1, "Base-5: 1[C]1"),
        (5, 2, 3, "Base-5: 2[C]3"),
        (6, 1, 5, "Base-6: 1[C]5"),
        (6, 5, 5, "Base-6: 5[C]5"),
    ];
    for (base, l, r, label) in configs {
        let density = test_configuration(base, l, r, 3, 0, 0, 0..5000);
        println!("  {} : {:.1}% density", label, density * 100.0);
        
        if density > 0.35 {
            println!("    🎯 EXCEPTIONAL! Over 35% density!");
        }
    }
fn test_self_inverse_pairs() {
    println!("  Testing all self-inverse digits across multiple bases...");
    let base_configs = vec![
        (8, vec![3, 5, 7]),  // 3²≡1, 5²≡1, 7²≡1 mod 8
        (12, vec![1, 5, 7, 11]), 
        (15, vec![1, 4, 11, 14]),
        (20, vec![1, 9, 11, 19]),
        (24, vec![1, 5, 7, 11, 13, 17, 19, 23]),
    for (base, self_inv_digits) in base_configs {
        println!("\n  Base-{} self-inverse pairs:", base);
        // Test all pairs
        for i in 0..self_inv_digits.len() {
            for j in i..self_inv_digits.len() {
                let l = self_inv_digits[i];
                let r = self_inv_digits[j];
                let density = test_configuration(base, l, r, 3, 0, 0, 0..2000);
                
                if density > 0.30 {
                    println!("    {}[C]{} : {:.1}% 🌟", l, r, density * 100.0);
                }
            }
fn test_twin_membranes() {
    println!("  Searching for configurations that generate twin primes...");
        (10, 3, 7, 0, 0),
        (12, 5, 7, 0, 0),
        (6, 1, 5, 0, 0),
    for (base, l, r, r1, r2) in configs {
        let mut twin_count = 0;
        let mut total_primes = 0;
        let c_range: Vec<u64> = (0..5000).collect();
        let results: Vec<_> = c_range.par_iter()
            .filter_map(|&c| {
                let params = MembraneParams { base, l, r, w: 3, r1, r2 };
                let value = compute_membrane_value(&params, c);
                if is_prime_miller_rabin(&value) {
                    let twin = &value + 2u32;
                    if is_prime_miller_rabin(&twin) {
                        Some((c, value, true))
                    } else {
                        Some((c, value, false))
                    }
                } else {
                    None
            })
            .collect();
        for (_, _, is_twin) in &results {
            total_primes += 1;
            if *is_twin {
                twin_count += 1;
        let twin_ratio = if total_primes > 0 {
            twin_count as f64 / total_primes as f64
        } else {
            0.0
        };
        println!("    Base-{} {}[C]{}: {} twins out of {} primes ({:.1}% twin rate)",
            base, l, r, twin_count, total_primes, twin_ratio * 100.0);
        if twin_count > 50 {
            println!("      💎 Rich twin vein discovered!");
fn test_golden_ratio() {
    println!("  Testing Fibonacci-inspired membrane configurations...");
    // Fibonacci numbers mod various bases
    let fib_configs = vec![
        (10, vec![(1, 1), (1, 2), (2, 3), (3, 5), (5, 8)]),
        (13, vec![(1, 1), (1, 2), (2, 3), (3, 5), (5, 8), (8, 0)]), // 13≡0 mod 13
        (21, vec![(1, 1), (1, 2), (2, 3), (3, 5), (5, 8), (8, 13), (13, 0)]),
    for (base, pairs) in fib_configs {
        println!("\n  Base-{} Fibonacci pairs:", base);
        for (l, r) in pairs {
            let density = test_configuration(base, l, r, 3, 0, 0, 0..3000);
            println!("    {}[C]{} : {:.1}% density", l, r, density * 100.0);
    // Test golden ratio approximations
    println!("\n  Golden ratio approximations:");
    let phi_configs = vec![
        (10, 16, 10), // 1.6
        (13, 21, 13), // φ ≈ 21/13
        (89, 144, 89), // Better φ approximation
    for (base, num, den) in phi_configs {
        let l = num % base;
        let r = den % base;
        if l != 0 && r != 0 {
            let density = test_configuration(base, l, r, 3, 0, 0, 0..2000);
            println!("    Base-{} {}/{}→{}[C]{} : {:.1}%", 
                base, num, den, l, r, density * 100.0);
// Helper functions
struct MembraneParams {
    base: u32,
    l: u32,
    r: u32,
    w: u32,
    r1: u32,
    r2: u32,
fn test_configuration(base: u32, l: u32, r: u32, w: u32, r1: u32, r2: u32, 
                     c_range: std::ops::Range<u64>) -> f64 {
    let params = MembraneParams { base, l, r, w, r1, r2 };
    let total = c_range.end - c_range.start;
    if total > 1000 {
        println!("    Testing {} configurations...", total);
        print!("    Progress: [");
        io::stdout().flush().unwrap();
    let progress = Arc::new(AtomicUsize::new(0));
    let progress_clone = Arc::clone(&progress);
    let prime_count = c_range.clone()
        .into_par_iter()
        .filter(|&c| {
            let value = compute_membrane_value(&params, c);
            let is_prime = is_prime_miller_rabin(&value);
            
            if total > 1000 {
                let current = progress_clone.fetch_add(1, Ordering::Relaxed) + 1;
                if current % (total / 50).max(1) == 0 {
                    print!("█");
                    io::stdout().flush().unwrap();
            is_prime
        })
        .count();
        println!("] Done!");
    prime_count as f64 / total as f64
fn compute_membrane_value(params: &MembraneParams, c: u64) -> BigUint {
    let base = BigUint::from(params.base);
    let l = BigUint::from(params.l);
    let r = BigUint::from(params.r);
    let c = BigUint::from(c);
    &l * base.pow(params.w - 1) +
    &r * base.pow(params.w - 2 - params.r1) +
    &c * base.pow(params.w / 2) +
    &r * base.pow(params.r2 + 1) +
    &l
}
