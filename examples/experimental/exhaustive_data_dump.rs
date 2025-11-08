//! Exhaustive Data Dump - Output comprehensive verification data
//! 
//! This script tests many configurations and outputs detailed data
//! to verify or disprove claims in documentation.

use prime_physics_engine::{is_prime_miller_rabin, ascii_art::*};
use num_bigint::BigUint;
use std::fs::File;
use std::io::Write;
fn main() {
    println!("{}", banner("EXHAUSTIVE DATA DUMP", 100));
    
    let mut output = File::create("exhaustive_verification_data.csv").unwrap();
    writeln!(output, "base,outer,inner,k_outer,k_inner,samples,successes,percentage,is_coprime").unwrap();
    // Test bases
    let bases = vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14, 15, 16, 18, 20, 24, 30];
    println!("\nTesting {} bases with comprehensive configurations...\n", bases.len());
    for &base in &bases {
        println!("Testing base {}...", base);
        
        // Find valid digits for this base
        let valid_digits: Vec<u32> = (1..base).collect();
        // Test all digit pairs
        for &outer in &valid_digits {
            for &inner in &valid_digits {
                if outer == inner { continue; }
                
                let outer_coprime = gcd(outer, base) == 1;
                let inner_coprime = gcd(inner, base) == 1;
                let both_coprime = outer_coprime && inner_coprime;
                // Test different k values
                let k_values = vec![(0, 0), (0, 1), (1, 0), (1, 1), (2, 2)];
                for (k_outer, k_inner) in k_values {
                    let samples = 1000;
                    let successes = test_configuration(base, outer, inner, k_outer, k_inner, samples);
                    let percentage = (successes as f64 / samples as f64) * 100.0;
                    
                    writeln!(output, "{},{},{},{},{},{},{},{:.2},{}",
                        base, outer, inner, k_outer, k_inner, samples, successes, percentage, both_coprime
                    ).unwrap();
                }
            }
        }
    }
    println!("\n{}", boxed_title("SPECIAL FOCUS: DISPUTED CLAIMS", 100));
    // Test specific disputed claims with more samples
    let disputed_claims = vec![
        ("Base 6: (1,3) k=(0,0) - claimed 41%", 6, 1, 3, 0, 0),
        ("Base 6: (3,3) k=(0,1) - claimed 30.2%", 6, 3, 3, 0, 1),
        ("Base 12: (5,7) k=(0,1) - claimed 28.9%", 12, 5, 7, 0, 1),
        ("Base 4: (3,1) k=(0,0) - claimed 28%", 4, 3, 1, 0, 0),
    ];
    println!("\nTesting disputed claims with 10,000 samples each:\n");
    for (desc, base, outer, inner, k_outer, k_inner) in disputed_claims {
        print!("{:<50}", desc);
        std::io::stdout().flush().unwrap();
        let samples = 10_000;
        let successes = test_configuration(base, outer, inner, k_outer, k_inner, samples);
        let percentage = (successes as f64 / samples as f64) * 100.0;
        println!("Actual: {:.2}% ({}/{})", percentage, successes, samples);
    // Test breathing vs symmetric systematically
    println!("\n{}", boxed_title("BREATHING VS SYMMETRIC COMPARISON", 100));
    println!("\nTesting all coprime configurations in bases 6 and 12:\n");
    println!("{:<10} {:<15} {:<15} {:<15} {:<15}", "Base", "Config", "Symmetric %", "Breathing %", "Difference");
    println!("{}", "-".repeat(70));
    for &base in &[6, 12] {
        for outer in 1..base {
            if gcd(outer, base) != 1 { continue; }
            
            for inner in 1..base {
                if inner == outer || gcd(inner, base) != 1 { continue; }
                let symmetric = test_configuration(base, outer, inner, 1, 1, 1000);
                let breathing = test_configuration(base, outer, inner, 0, 1, 1000);
                let sym_pct = (symmetric as f64 / 1000.0) * 100.0;
                let breath_pct = (breathing as f64 / 1000.0) * 100.0;
                let diff = breath_pct - sym_pct;
                println!("{:<10} ({},{})         {:<15.1} {:<15.1} {:+.1}", 
                    base, outer, inner, sym_pct, breath_pct, diff);
    // Even vs odd base comparison
    println!("\n{}", boxed_title("EVEN VS ODD BASE COMPARISON", 100));
    let even_bases = vec![4, 6, 8, 10, 12, 14, 16, 18, 20, 24, 30];
    let odd_bases = vec![3, 5, 7, 9, 11, 13, 15, 17, 19, 23, 29];
    println!("\nFinding best configuration for each base (5000 samples):\n");
    let mut even_results = Vec::new();
    let mut odd_results = Vec::new();
    println!("Even bases:");
    for &base in &even_bases {
        let (best_config, best_rate) = find_best_for_base(base, 5000);
        even_results.push(best_rate);
        println!("  Base {:2}: ({},{}) k=(0,0) → {:.1}%", 
            base, best_config.0, best_config.1, best_rate);
    println!("\nOdd bases:");
    for &base in &odd_bases {
        odd_results.push(best_rate);
    let even_avg = even_results.iter().sum::<f64>() / even_results.len() as f64;
    let odd_avg = odd_results.iter().sum::<f64>() / odd_results.len() as f64;
    let advantage = ((even_avg - odd_avg) / odd_avg) * 100.0;
    println!("\nSummary:");
    println!("  Even base average: {:.1}%", even_avg);
    println!("  Odd base average:  {:.1}%", odd_avg);
    println!("  Even base advantage: {:.1}%", advantage);
    println!("\nData saved to: exhaustive_verification_data.csv");
}
fn test_configuration(base: u32, outer: u32, inner: u32, k_outer: u32, k_inner: u32, samples: u32) -> u32 {
    let mut successes = 0;
    for seed in 0..samples {
        let membrane = construct_membrane(base, outer, inner, k_outer, k_inner, seed);
        if is_prime_miller_rabin(&membrane) {
            successes += 1;
    successes
fn find_best_for_base(base: u32, samples: u32) -> ((u32, u32), f64) {
    let mut best_config = (1, 2);
    let mut best_rate = 0.0;
    for outer in 1..base {
        if gcd(outer, base) != 1 { continue; }
        for inner in 1..base {
            if inner == outer || gcd(inner, base) != 1 { continue; }
            let successes = test_configuration(base, outer, inner, 0, 0, samples);
            let rate = (successes as f64 / samples as f64) * 100.0;
            if rate > best_rate {
                best_rate = rate;
                best_config = (outer, inner);
    (best_config, best_rate)
fn construct_membrane(base: u32, outer: u32, inner: u32, k_outer: u32, k_inner: u32, seed: u32) -> BigUint {
    let width = 2 * (1 + k_outer + 1 + k_inner) + 1;
    let base_big = BigUint::from(base);
    let mut value = BigUint::from(0u32);
    value += BigUint::from(outer) * base_big.pow(width - 1);
    value += BigUint::from(inner) * base_big.pow(width - 2 - k_outer);
    value += BigUint::from(seed) * base_big.pow(width / 2);
    value += BigUint::from(inner) * base_big.pow(k_inner + 1);
    value += BigUint::from(outer);
    value
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
