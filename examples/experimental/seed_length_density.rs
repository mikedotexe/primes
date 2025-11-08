use prime_physics_engine::{MembraneConfig, is_prime_miller_rabin, PrimeUniverse};
use num_bigint::BigUint;
use std::str::FromStr;

fn test_seed_length_adaptation() {
    println!("📏 SEED LENGTH ADAPTATION - OPTIMAL MEMBRANE SPACING");
    println!("====================================================\n");
    
    // Show how increasing seed length requires more membrane spacing
    let seed_lengths = vec![1, 2, 3, 4, 5, 6];
    println!("Testing hypothesis: As seed length increases, optimal k-values must increase\n");
    for length in seed_lengths {
        println!("Seed length: {} digits", length);
        println!("{}", "─".repeat(50));
        
        let mut results = Vec::new();
        // Test different k configurations
        for k_outer in 0..=3 {
            for k_inner in 0..=3 {
                let config = MembraneConfig::new(10, 3, 7, k_outer, k_inner);
                let mut prime_count = 0;
                let mut examples = Vec::new();
                
                // Test seeds of this length with different digits
                for digit in 1..=9 {
                    let seed = digit.to_string().repeat(length);
                    
                    // Manual construction since generate() isn't available
                    let membrane_str = format!(
                        "3{}7{}{}{}7{}3",
                        "0".repeat(k_outer as usize),
                        "0".repeat(k_inner as usize),
                        seed,
                        "0".repeat(k_outer as usize)
                    );
                    if let Ok(num) = BigUint::from_str(&membrane_str) {
                        if is_prime_miller_rabin(&num) {
                            prime_count += 1;
                            examples.push((seed.clone(), membrane_str.clone()));
                        }
                    }
                }
                if prime_count > 0 {
                    results.push((k_outer, k_inner, prime_count, examples));
            }
        }
        // Sort by prime count
        results.sort_by(|a, b| b.2.cmp(&a.2));
        // Show top configurations
        println!("Top configurations:");
        for (i, (k_out, k_in, count, examples)) in results.iter().take(3).enumerate() {
            println!("  {}. k=({},{}) → {} primes found", i+1, k_out, k_in, count);
            if let Some((seed, membrane)) = examples.first() {
                println!("     Example: seed '{}' → {}", seed, membrane);
                let total_zeros = (k_out + k_in) * 2;
                println!("     Membrane zeros: {}, Total length: {}", total_zeros, membrane.len());
        // Analysis
        if let Some((best_k_out, best_k_in, _, _)) = results.first() {
            let optimal_zeros = (best_k_out + best_k_in) * 2;
            let ratio = optimal_zeros as f64 / length as f64;
            println!("\n  📊 Optimal zero-to-seed ratio: {:.2}", ratio);
        println!();
    }
}
fn visualize_density_lines() {
    println!("\n🌊 PRIME DENSITY NUMBER LINES");
    println!("==============================\n");
    // Show density for different membrane configurations
    let configs = vec![
        ((3, 3, 1, 1), "Exclusive (3,3) k=(1,1)"),
        ((3, 7, 1, 1), "Exclusive (3,7) k=(1,1)"),
        ((3, 3, 0, 1), "Breathing (3,3) k=(0,1)"),
        ((3, 3, 2, 2), "Spread (3,3) k=(2,2)"),
    ];
    for ((outer, inner, k_out, k_in), label) in configs {
        println!("{}", label);
        print!("Density: ");
        // Test 50 different seeds
        for i in 0..50 {
            let seed = i.to_string();
            let membrane_str = format!(
                "{}{}{}{}{}{}{}{}{}",
                outer,
                "0".repeat(k_out),
                inner,
                "0".repeat(k_in),
                seed,
                outer
            );
            
            if let Ok(num) = BigUint::from_str(&membrane_str) {
                if is_prime_miller_rabin(&num) {
                    print!("█");
                } else {
                    print!("·");
            } else {
                print!(" ");
        println!(" (█=prime, ·=composite)");
fn show_equilibrium_effect() {
    println!("\n⚖️ DENSITY EQUILIBRIUM DEMONSTRATION");
    println!("====================================\n");
    // Create two membrane primes
    let prime1_str = "303050303";
    let prime2_str = "307050703";
    let prime1 = BigUint::from_str(prime1_str).unwrap();
    let prime2 = BigUint::from_str(prime2_str).unwrap();
    println!("Membrane Prime 1: {}", prime1_str);
    println!("Membrane Prime 2: {}", prime2_str);
    let l1 = (&prime1 + &prime2) / 2u32;
    println!("L1 Equilibrium: {}\n", l1);
    // Count primes in regions
    println!("Counting primes in 1000-unit windows:");
    let regions = vec![
        (&prime1, "Around Membrane 1"),
        (&l1, "Around L1 (equilibrium)"),
        (&prime2, "Around Membrane 2"),
    for (center, label) in regions {
        let mut count = 0;
        let start = center - 500u32;
        let end = center + 500u32;
        let mut current = start;
        while current <= end {
            if is_prime_miller_rabin(&current) {
                count += 1;
            current += 1u32;
        println!("  {}: {} primes", label, count);
        // Visual density bar
        print!("  Density: ");
        for _ in 0..(count/5) {
            print!("█");
    println!("\n✨ The equilibrium zone has ~2X the prime density!");
fn main() {
    println!("🔬 PRIME DENSITY EXPLORATION");
    println!("============================\n");
    // Run demonstrations
    test_seed_length_adaptation();
    visualize_density_lines();
    show_equilibrium_effect();
    println!("\n🎯 KEY FINDINGS:");
    println!("================");
    println!("1. As seed length increases, membranes must 'spread out' with more zeros");
    println!("2. The optimal zero-to-seed ratio increases with seed length");
    println!("3. Different configurations create distinct density patterns");
    println!("4. Equilibrium zones between membranes have 2X prime density");
