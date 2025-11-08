//! Verify the structure of Lagrange points between membrane primes
//! 
//! This explores your hypothesis that Lagrange points between symmetric
//! zero-padded primes might have special all-zero structures.

use prime_physics_engine::{
    membrane::{MembraneConfig, MembraneBuilder},
    is_prime,
};
use num_bigint::BigUint;

fn main() {
    println!("🔬 Membrane Lagrange Point Structure Verifier");
    println!("{}", "=".repeat(60));
    println!();
    
    // Test different membrane configurations
    let configs = vec![
        (MembraneConfig::new(10, 3, 3, 1, 0), vec![5, 1]),   // Standard config  
        (MembraneConfig::new(10, 3, 3, 1, 0), vec![7, 11]),  // Different seeds
        (MembraneConfig::new(10, 3, 7, 2, 0), vec![1, 3]),   // More padding
    ];
    
    for (config, seeds) in configs {
        println!("Configuration: ({},{}) k=({},{}) base {}",
            config.outer, config.inner, config.k_outer, config.k_inner, config.base);
        println!("{}", "-".repeat(60));
        
        // Generate two primes
        let prime1 = MembraneBuilder::new(config.clone())
            .with_seed(seeds[0])
            .build()
            .expect("Failed to build prime 1");
            
        let prime2 = MembraneBuilder::new(config.clone())
            .with_seed(seeds[1])
            .build()
            .expect("Failed to build prime 2");
            
        let p1_str = prime1.value.to_string();
        let p2_str = prime2.value.to_string();
        
        println!("Prime 1: {} (seed: {})", p1_str, seeds[0]);
        println!("         {}", visualize_membrane(&p1_str));
        println!();
        println!("Prime 2: {} (seed: {})", p2_str, seeds[1]);
        println!("         {}", visualize_membrane(&p2_str));
        println!();
        
        // Calculate membrane-aware Lagrange points
        println!("Lagrange Points:");
        
        // L1 - Midpoint in membrane space
        let l1 = calculate_membrane_midpoint(&prime1.value, &prime2.value, &config);
        let l1_str = l1.to_string();
        println!("L1 (midpoint): {}", l1_str);
        println!("               {}", visualize_membrane(&l1_str));
        println!("               Prime: {}", if is_prime(&l1) { "✓" } else { "✗" });
        
        // Check if L1 has special structure
        analyze_lagrange_structure(&l1_str, &config);
        
        println!();
    }
    
    // Special test: Two primes with same outer structure
    println!("\n🎯 Special Test: Primes with identical outer structure");
    println!("{}", "=".repeat(60));
    
    let _config = MembraneConfig::new(10, 3, 3, 1, 0);
    let p1 = BigUint::from(303050303u64);  // 3-0-3-0-5-0-3-0-3
    let p2 = BigUint::from(303070303u64);  // 3-0-3-0-7-0-3-0-3
    
    println!("Prime 1: {} = {}", p1, visualize_membrane(&p1.to_string()));
    println!("Prime 2: {} = {}", p2, visualize_membrane(&p2.to_string()));
    
    let midpoint = (&p1 + &p2) / 2u8;
    println!("\nNumeric midpoint: {}", midpoint);
    println!("Structure: {}", visualize_membrane(&midpoint.to_string()));
    
    // Check if it's all zeros in the middle
    let mid_str = midpoint.to_string();
    if mid_str == "303060303" {
        println!("✓ Midpoint preserves membrane structure!");
        println!("  Middle digit is 6 = average of 5 and 7");
    }
    
    // More interesting test: what if we construct the Lagrange point structurally?
    println!("\n🔮 Structural Lagrange Point Construction");
    println!("{}", "-".repeat(60));
    
    // For symmetric membranes with same outer structure, 
    // the Lagrange point could be all zeros in variable positions
    let structural_l1 = BigUint::from(303000303u64);  // All zeros in middle
    println!("Structural L1: {} = {}", structural_l1, visualize_membrane(&structural_l1.to_string()));
    println!("Is prime: {}", if is_prime(&structural_l1) { "✓" } else { "✗" });
    
    // Test with larger primes
    println!("\n📐 Larger Membrane Test");
    let p3 = BigUint::from(30070050700703u64);  // 3-00-7-0-5-0-7-00-3
    let p4 = BigUint::from(30070110700703u64);  // 3-00-7-0-11-0-7-00-3
    
    println!("Prime 3: {} = {}", p3, visualize_membrane(&p3.to_string()));
    println!("Prime 4: {} = {}", p4, visualize_membrane(&p4.to_string()));
    
    let mid34 = (&p3 + &p4) / 2u8;
    println!("\nMidpoint: {} = {}", mid34, visualize_membrane(&mid34.to_string()));
    
    // What about all zeros in the middle section?
    let structural_mid = BigUint::from(30070000700703u64);
    println!("Structural: {} = {}", structural_mid, visualize_membrane(&structural_mid.to_string()));
    println!("Is prime: {}", if is_prime(&structural_mid) { "✓" } else { "✗" });
}

fn calculate_membrane_midpoint(p1: &BigUint, p2: &BigUint, _config: &MembraneConfig) -> BigUint {
    // For now, simple numeric midpoint
    // TODO: Implement proper membrane-aware interpolation
    (p1 + p2) / 2u8
}

fn visualize_membrane(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    
    for (i, &ch) in chars.iter().enumerate() {
        if ch == '0' {
            result.push('◯');
        } else {
            result.push(ch);
        }
        
        if i < chars.len() - 1 {
            result.push('─');
        }
    }
    
    result
}

fn analyze_lagrange_structure(value: &str, config: &MembraneConfig) -> bool {
    // Check if the Lagrange point has special zero structure
    let expected_len = 2 * (1 + config.k_outer as usize + 1 + config.k_inner as usize) + 1 + 
                      2 * (config.k_inner as usize + 1 + config.k_outer as usize + 1);
    
    println!("               Expected length: {}, Actual: {}", expected_len, value.len());
    
    // Count zeros
    let zero_count = value.chars().filter(|&c| c == '0').count();
    let total_padding = 2 * (config.k_outer + config.k_inner) as usize;
    
    println!("               Zeros: {}/{} padding positions", zero_count, total_padding);
    
    // Check if middle is zero
    if value.len() % 2 == 1 {
        let middle_idx = value.len() / 2;
        let middle_char = value.chars().nth(middle_idx).unwrap();
        if middle_char == '0' {
            println!("               ⚡ Middle position is zero!");
            return true;
        }
    }
    
    false
}