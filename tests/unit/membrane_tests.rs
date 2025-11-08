//! Unit tests for membrane construction and validation

use prime_physics_engine::membrane::{MembraneConfig, MembraneBuilder};
use prime_physics_engine::is_prime;

#[test]
fn test_basic_membrane_construction() {
    let config = MembraneConfig::new(10, 3, 7, 0, 0);
    let result = MembraneBuilder::new(config)
        .with_seed(5)
        .build();
    
    assert!(result.is_ok(), "Basic membrane construction should succeed");
    let particle = result.unwrap();
    assert_eq!(particle.value.to_string(), "37573");
}

#[test]
fn test_coprimality_validation() {
    // Non-coprime digits should be rejected
    let config = MembraneConfig::new(10, 2, 4, 0, 0); // 2 and 4 share factor with 10
    assert!(!config.is_valid(), "Non-coprime configuration should be invalid");
    
    // Coprime digits should be accepted
    let config = MembraneConfig::new(10, 3, 7, 0, 0);
    assert!(config.is_valid(), "Coprime configuration should be valid");
}

#[test]
fn test_base_6_optimal_config() {
    let config = MembraneConfig::new(6, 1, 5, 0, 0);
    let mut prime_count = 0;
    
    for seed in 1..=10 {
        if let Ok(particle) = MembraneBuilder::new(config.clone()).with_seed(seed).build() {
            if is_prime(&particle.value) {
                prime_count += 1;
            }
        }
    }
    
    // Base 6 (1,5) should achieve ~30% prime density
    assert!(prime_count >= 2, "Base 6 optimal config should generate at least 2 primes out of 10");
}

#[test]
fn test_zero_padding_effects() {
    let base = 10;
    let outer = 3;
    let inner = 7;
    let seed = 5;
    
    // Test different padding values
    let configs = vec![
        (0, 0, "37573"),     // No padding
        (1, 0, "3070573"),   // Outer padding only
        (0, 1, "3750573"),   // Inner padding only
        (1, 1, "30750573"),  // Both paddings
    ];
    
    for (k_outer, k_inner, expected) in configs {
        let config = MembraneConfig::new(base, outer, inner, k_outer, k_inner);
        let result = MembraneBuilder::new(config)
            .with_seed(seed)
            .build();
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value.to_string(), expected,
            "Padding ({},{}) should produce {}", k_outer, k_inner, expected);
    }
}

#[test]
fn test_invalid_configurations() {
    // Test various invalid configurations
    let invalid_configs = vec![
        (0, 3, 7, 0, 0),    // Invalid base
        (10, 0, 7, 0, 0),   // Invalid outer digit
        (10, 3, 10, 0, 0),  // Invalid inner digit
        (10, 3, 7, 100, 0), // Excessive padding
    ];
    
    for (base, outer, inner, k_outer, k_inner) in invalid_configs {
        let config = MembraneConfig::new(base, outer, inner, k_outer, k_inner);
        assert!(!config.is_valid() || 
                MembraneBuilder::new(config).with_seed(5).build().is_err(),
                "Configuration ({}, {}, {}, {}, {}) should be invalid",
                base, outer, inner, k_outer, k_inner);
    }
}

#[test]
fn test_large_seed_handling() {
    let config = MembraneConfig::new(10, 3, 7, 0, 0);
    let large_seed = 999999;
    
    let result = MembraneBuilder::new(config)
        .with_seed(large_seed)
        .build();
    
    assert!(result.is_ok(), "Large seeds should be handled correctly");
    let value_str = result.unwrap().value.to_string();
    assert!(value_str.contains("999999"), "Large seed should appear in result");
}