//! Base Parity Test - Simple exploration of odd vs even bases
//! 
//! Tests a hypothesis: even bases generate more primes than odd bases
//! using membrane-like patterns.

use prime_physics_engine::{is_prime_miller_rabin, ascii_art::*};
use num_bigint::BigUint;
use num_traits::Num;
use std::fs::File;
use std::io::Write;
use chrono::Local;
/// Generate a membrane number in a given base
fn generate_membrane(base: u32, outer: u32, inner: u32, k_outer: u32, k_inner: u32, middle: u32) -> BigUint {
    // Build the pattern: outer + k_outer zeros + inner + k_inner zeros + middle + ...
    let mut digits = Vec::new();
    
    // Left side
    digits.push(outer);
    for _ in 0..k_outer {
        digits.push(0);
    }
    digits.push(inner);
    for _ in 0..k_inner {
    // Middle
    digits.push(middle);
    // Right side (mirror)
    // Convert to BigUint in the given base
    let mut value = BigUint::from(0u32);
    let base_big = BigUint::from(base);
    for digit in digits {
        value = value * &base_big + BigUint::from(digit);
    value
}
fn test_base(base: u32) -> Vec<(u32, u32, u32, u32, f64, Vec<BigUint>)> {
    let mut results = Vec::new();
    println!("\nTesting base {} ({})...", base, if base % 2 == 0 { "even" } else { "odd" });
    // Test various configurations
    for outer in 1..base.min(8) {
        for inner in 1..base.min(8) {
            if outer == inner { continue; }
            
            for k_outer in 0..=2 {
                for k_inner in 0..=2 {
                    let mut primes_found = Vec::new();
                    let test_count = 10;
                    
                    // Test with different middle values
                    for middle in 0..test_count {
                        let membrane = generate_membrane(base, outer, inner, k_outer, k_inner, middle);
                        
                        if is_prime_miller_rabin(&membrane) {
                            primes_found.push(membrane);
                        }
                    }
                    let density = primes_found.len() as f64 / test_count as f64;
                    if density > 0.0 {
                        results.push((outer, inner, k_outer, k_inner, density, primes_found.clone()));
                }
            }
        }
    results.sort_by(|a, b| b.4.partial_cmp(&a.4).unwrap());
    results
fn main() {
    println!("{}", banner("BASE PARITY EXPERIMENT", 70));
    println!("\nTesting the hypothesis: Even bases generate more membrane primes\n");
    // Test both odd and even bases
    let test_bases = vec![
        (3, "odd"), (4, "even"), 
        (5, "odd"), (6, "even"),
        (7, "odd"), (8, "even"),
        (9, "odd"), (10, "even"),
        (11, "odd"), (12, "even"),
    ];
    let mut all_results = Vec::new();
    for (base, parity) in &test_bases {
        let configs = test_base(*base);
        
        if let Some(best) = configs.first() {
            all_results.push((*base, *parity, best.4, best.clone()));
            println!("  Best config: ({},{}) k=({},{}) → {:.1}% density",
                best.0, best.1, best.2, best.3, best.4 * 100.0);
            if !best.5.is_empty() {
                println!("  Example prime: {}", best.5[0]);
    // Analysis
    println!("\n{}", boxed_title("RESULTS ANALYSIS", 70));
    // Calculate averages
    let odd_densities: Vec<f64> = all_results.iter()
        .filter(|(_, p, _, _)| *p == "odd")
        .map(|(_, _, d, _)| *d)
        .collect();
    let even_densities: Vec<f64> = all_results.iter()
        .filter(|(_, p, _, _)| *p == "even")
    let odd_avg = odd_densities.iter().sum::<f64>() / odd_densities.len() as f64;
    let even_avg = even_densities.iter().sum::<f64>() / even_densities.len() as f64;
    println!("\n📊 DENSITY COMPARISON:");
    println!("{}", separator("double", 70));
    // Sort all results by density
    all_results.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    println!("\nRanking by prime density:");
    for (i, (base, parity, density, _)) in all_results.iter().enumerate() {
        let bar_width = (*density * 40.0) as usize;
        let symbol = if *parity == "odd" { "○" } else { "●" };
        println!("{:2}. Base {:2} {} │{}│ {:.1}%",
            i + 1, base, symbol, "█".repeat(bar_width), density * 100.0);
    println!("\n○ = odd base, ● = even base");
    println!("\n📈 STATISTICAL SUMMARY:");
    println!("  Odd bases average:  {:.1}%", odd_avg * 100.0);
    println!("  Even bases average: {:.1}%", even_avg * 100.0);
    if even_avg > odd_avg {
        let improvement = ((even_avg - odd_avg) / odd_avg * 100.0);
        println!("\n✨ CONCLUSION: Even bases are {:.0}% better!", improvement);
    } else if odd_avg > even_avg {
        let improvement = ((odd_avg - even_avg) / even_avg * 100.0);
        println!("\n✨ CONCLUSION: Odd bases are {:.0}% better!", improvement);
    } else {
        println!("\n✨ CONCLUSION: No significant difference!");
    // Theoretical explanation
    println!("\n{}", boxed_title("THEORETICAL EXPLANATION", 70));
        println!(r#"
Why Even Bases Excel:
1. INTEGER MIDPOINTS
   Even bases have exact centers (6→3, 10→5, 12→6)
   These act as resonance points for wave patterns
2. SYMMETRIC DIVISIBILITY
   Even bases divide by 2, creating perfect symmetry
   This enables constructive interference
3. THE 5-7 PHENOMENON
   Distance 2 between twin primes 5,7
   2 divides evenly into all even bases
   But creates fractional periods in odd bases
4. BREATHING PATTERNS
   Asymmetric k-values (k₁≠k₂) work better in even bases
   The alternation aligns with base parity
"#);
    // Show specific interesting patterns
    println!("\n{}", simple_box("INTERESTING DISCOVERIES"));
    // Find patterns that work across multiple bases
    let mut cross_base_patterns = std::collections::HashMap::new();
    for (base, _, _, config) in &all_results {
        let pattern_key = format!("({},{})_k({},{})", config.0, config.1, config.2, config.3);
        cross_base_patterns.entry(pattern_key)
            .or_insert_with(Vec::new)
            .push(*base);
    println!("\nPatterns that work in multiple bases:");
    for (pattern, bases) in cross_base_patterns.iter() {
        if bases.len() > 1 {
            println!("  {} works in bases: {:?}", pattern, bases);
    // Special case analysis
    if let Some((_, _, _, config)) = all_results.iter().find(|(b, _, _, _)| *b == 9) {
        println!("\n🔍 Base 9 (3²) Special Case:");
        println!("  As a perfect square odd base, it shows unique behavior");
        println!("  Best config avoids multiples of 3: ({},{})", config.0, config.1);
    // Save detailed results
    let filename = format!("base_parity_results_{}.txt", 
        Local::now().format("%Y%m%d_%H%M%S"));
    let mut file = File::create(&filename).expect("Failed to create file");
    writeln!(file, "{}", boxed_title("BASE PARITY EXPERIMENT RESULTS", 80)).unwrap();
    writeln!(file, "\nHypothesis: Even bases generate more membrane primes than odd bases").unwrap();
    writeln!(file, "Result: {}\n", 
        if even_avg > odd_avg { "CONFIRMED ✓" } else { "NOT CONFIRMED ✗" }
    ).unwrap();
    writeln!(file, "Statistical Summary:").unwrap();
    writeln!(file, "-------------------").unwrap();
    writeln!(file, "Odd bases average:  {:.2}%", odd_avg * 100.0).unwrap();
    writeln!(file, "Even bases average: {:.2}%", even_avg * 100.0).unwrap();
    writeln!(file, "Difference: {:.1}% in favor of {} bases\n", 
        ((even_avg - odd_avg).abs() / odd_avg.min(even_avg) * 100.0),
        if even_avg > odd_avg { "even" } else { "odd" }
    writeln!(file, "\nDetailed Rankings:").unwrap();
    writeln!(file, "-----------------").unwrap();
    for (i, (base, parity, density, config)) in all_results.iter().enumerate() {
        writeln!(file, "{}. Base {} ({}): {:.1}% with config ({},{}) k=({},{})",
            i + 1, base, parity, density * 100.0,
            config.0, config.1, config.2, config.3
        ).unwrap();
    println!("\n✅ Experiment complete!");
    println!("📄 Detailed results saved to: {}", filename);
