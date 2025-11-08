//! Investigation: Why does MembraneBuilder find 0 primes?
//! 
//! Let's systematically test the builder vs manual construction

use prime_physics_engine::{MembraneConfig, MembraneBuilder, is_prime};
use num_bigint::BigUint;

fn main() {
    println!("🔍 MembraneBuilder Investigation");
    println!("{}", "=".repeat(80));
    println!();
    
    // Test 1: Compare manual vs builder for known working configs
    test_breathing_pattern();
    
    // Test 2: Investigate the builder internals
    test_builder_internals();
    
    // Test 3: Test various configurations
    test_multiple_configs();
    
    // Test 4: Debug the 5000 attempt limit
    test_attempt_limits();
}

fn test_breathing_pattern() {
    println!("TEST 1: Breathing Pattern (3,3) k=(0,1)");
    println!("{}", "-".repeat(80));
    
    let config = MembraneConfig::new(10, 3, 3, 0, 1);
    
    println!("A. Manual construction (what statistical_sampling_demo does):");
    let mut manual_primes = 0;
    for seed in 0..10 {
        // This is what the demo does: "33{}{}{}33"
        let membrane_str = format!("33{}{}{}33", "0", seed, "0");
        let num = membrane_str.parse::<BigUint>().unwrap();
        let is_p = is_prime(&num);
        println!("  Seed {}: {} → {}", 
            seed, membrane_str,
            if is_p { "✓ PRIME" } else { "✗" }
        );
        if is_p { manual_primes += 1; }
    }
    println!("  Manual found: {} primes\n", manual_primes);
    
    println!("B. Using MembraneBuilder:");
    let mut builder_primes = 0;
    let mut builder_attempts = 0;
    for seed in 0..10 {
        match MembraneBuilder::new(config.clone()).with_seed(seed).build() {
            Ok(particle) => {
                builder_attempts += 1;
                let is_p = is_prime(&particle.value);
                println!("  Seed {}: {} → {}", 
                    seed, particle.value,
                    if is_p { "✓ PRIME" } else { "✗" }
                );
                if is_p { builder_primes += 1; }
            }
            Err(e) => {
                println!("  Seed {}: ERROR - {:?}", seed, e);
            }
        }
    }
    println!("  Builder found: {} primes from {} successful builds", 
        builder_primes, builder_attempts);
    
    // Let's also check what the config thinks it should build
    println!("\nC. What does config.construct_number() produce?");
    for seed in 0..10 {
        match config.construct_number(seed) {
            Ok(num) => {
                let is_p = is_prime(&num);
                println!("  Seed {}: {} → {}", 
                    seed, num,
                    if is_p { "✓ PRIME" } else { "✗" }
                );
            }
            Err(e) => {
                println!("  Seed {}: ERROR - {:?}", seed, e);
            }
        }
    }
}

fn test_builder_internals() {
    println!("\n\nTEST 2: Builder Internals");
    println!("{}", "-".repeat(80));
    
    // Let's see what the builder is actually doing
    let config = MembraneConfig::new(10, 3, 3, 1, 0);
    
    println!("Testing config (3,3) k=(1,0) - known to work");
    println!("\nTrying different seed ranges:");
    
    // Try single digit seeds
    println!("\nA. Single digit seeds (0-9):");
    let mut found = 0;
    for seed in 0..10 {
        if let Ok(particle) = MembraneBuilder::new(config.clone()).with_seed(seed).build() {
            if is_prime(&particle.value) {
                found += 1;
                println!("  Seed {}: {} ✓", seed, particle.value);
            }
        }
    }
    println!("  Found {} primes", found);
    
    // Try larger seeds
    println!("\nB. Double digit seeds (10-99):");
    found = 0;
    let mut attempts = 0;
    for seed in 10..100 {
        attempts += 1;
        if let Ok(particle) = MembraneBuilder::new(config.clone()).with_seed(seed).build() {
            if is_prime(&particle.value) {
                found += 1;
                if found <= 5 {  // Show first 5
                    println!("  Seed {}: {} ✓", seed, particle.value);
                }
            }
        }
    }
    println!("  Found {} primes from {} attempts", found, attempts);
    
    // What about the supposedly broken config from the demo?
    println!("\nC. The config that found 0 primes in demo:");
    let config = MembraneConfig::new(10, 3, 3, 1, 0);  // Same as demo
    println!("  Config: ({},{}) k=({},{})", 
        config.outer, config.inner, config.k_outer, config.k_inner);
    
    // Let's try LOTS of seeds
    found = 0;
    attempts = 0;
    for seed in 0..255u8 {
        attempts += 1;
        if let Ok(particle) = MembraneBuilder::new(config.clone()).with_seed(seed).build() {
            if is_prime(&particle.value) {
                found += 1;
                if found <= 10 {  // Show first 10
                    println!("    Seed {}: {} ✓", seed, particle.value);
                }
            }
        }
    }
    println!("  Found {} primes from {} seeds tested", found, attempts);
}

fn test_multiple_configs() {
    println!("\n\nTEST 3: Multiple Configurations");
    println!("{}", "-".repeat(80));
    
    let configs = vec![
        (MembraneConfig::new(10, 3, 3, 0, 1), "Breathing (3,3) k=(0,1)"),
        (MembraneConfig::new(10, 3, 3, 1, 0), "Asymmetric (3,3) k=(1,0)"),
        (MembraneConfig::new(10, 3, 3, 1, 1), "Symmetric (3,3) k=(1,1)"),
        (MembraneConfig::new(10, 3, 7, 0, 0), "Classic (3,7) k=(0,0)"),
        (MembraneConfig::new(10, 3, 7, 1, 1), "Exclusive (3,7) k=(1,1)"),
        (MembraneConfig::new(6, 1, 5, 0, 0), "Base 6 champion (1,5) k=(0,0)"),
    ];
    
    for (config, name) in configs {
        println!("\nTesting: {}", name);
        
        let mut found = 0;
        let mut errors = 0;
        
        // Test seeds 0-19
        for seed in 0..20 {
            match MembraneBuilder::new(config.clone()).with_seed(seed).build() {
                Ok(particle) => {
                    if is_prime(&particle.value) {
                        found += 1;
                        if found <= 3 {  // Show first 3
                            println!("  Seed {}: {} ✓", seed, particle.value);
                        }
                    }
                }
                Err(_) => {
                    errors += 1;
                }
            }
        }
        
        println!("  Results: {} primes, {} errors from 20 seeds", found, errors);
        println!("  Success rate: {:.0}%", found as f64 * 5.0);
    }
}

fn test_attempt_limits() {
    println!("\n\nTEST 4: Understanding the 5000 Attempt Limit");
    println!("{}", "-".repeat(80));
    
    // The builder tries to find a prime by testing the constructed number
    // If it's not prime, it increments something and tries again
    // Let's see what's happening
    
    let config = MembraneConfig::new(10, 3, 3, 1, 0);
    
    println!("Let's manually check what the builder might be doing...");
    println!("\nFor config (3,3) k=(1,0), seed 5:");
    
    // What would the base construction be?
    let base_num = config.construct_number(5).unwrap();
    println!("Base construction: {}", base_num);
    println!("Is prime? {}", is_prime(&base_num));
    
    // The builder might be trying variations?
    println!("\nChecking nearby numbers:");
    for offset in 0..10u32 {
        let test_num = &base_num + offset;
        println!("  {} + {} = {} → {}", 
            base_num, offset, test_num,
            if is_prime(&test_num) { "PRIME!" } else { "composite" }
        );
    }
    
    // Let's trace exactly what a builder does
    println!("\n\nDirect Builder Test:");
    let builder = MembraneBuilder::new(config.clone()).with_seed(5);
    println!("Builder config: {:?}", config);
    
    match builder.build() {
        Ok(particle) => {
            println!("Success! Generated: {}", particle.value);
            println!("Is prime: {}", is_prime(&particle.value));
            println!("Particle details: {:?}", particle.membrane_config);
        }
        Err(e) => {
            println!("Failed: {:?}", e);
            
            // Let's try building without the prime requirement
            println!("\nTrying direct construction without prime check:");
            if let Ok(num) = config.construct_number(5) {
                println!("Direct construction: {}", num);
                println!("Is prime: {}", is_prime(&num));
            }
        }
    }
}