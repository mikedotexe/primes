// MVP Rust Adapter - Quick integration with existing membrane infrastructure
// ========================================================================
//
// This adapter provides a simple CLI interface for the Python MVP to test
// membrane scaling without rebuilding the entire analysis framework.
// 
// Integrates with Mike's existing prime-physics-engine codebase.

use std::env;

// Import from Mike's existing codebase (adapt paths as needed)
// use prime_physics_engine::membrane::*;
// use prime_physics_engine::validation::*;

// Simplified structures for MVP - replace with actual imports
#[derive(Debug, Clone)]
struct MembraneConfig {
    base: u32,
    outer: u32,
    inner: u32,
    k_outer: usize,
    k_inner: usize,
    middle_length: usize,
}

#[derive(Debug)]
struct QuickResult {
    total_tested: usize,
    primes_found: usize,
    density: f64,
    examples: Vec<u64>,
}

// MVP primality test (use Mike's production version)
fn is_prime_mvp(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    
    let limit = (n as f64).sqrt() as u64 + 1;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 { return false; }
    }
    true
}

// MVP membrane generator (integrate with Mike's actual implementation)
fn generate_membranes_mvp(config: &MembraneConfig) -> Vec<u64> {
    let mut results = Vec::new();
    let base = config.base as u64;
    
    // Generate all possible middle values
    let middle_max = base.pow(config.middle_length as u32);
    
    for middle in 0..middle_max {
        if let Some(membrane_value) = build_membrane(config, middle) {
            results.push(membrane_value);
        }
    }
    
    results
}

fn build_membrane(config: &MembraneConfig, middle: u64) -> Option<u64> {
    let base = config.base as u64;
    let mut value = 0u64;
    let mut position = 0usize;
    
    // Build membrane: outer [k_outer×0] inner [k_inner×0] MIDDLE [k_inner×0] inner [k_outer×0] outer
    
    // Right outer
    value += config.outer as u64 * base.pow(position as u32);
    position += 1;
    
    // Right k_outer zeros
    position += config.k_outer;
    
    // Right inner
    value += config.inner as u64 * base.pow(position as u32);
    position += 1;
    
    // Right k_inner zeros
    position += config.k_inner;
    
    // Middle
    value += middle * base.pow(position as u32);
    position += config.middle_length;
    
    // Left k_inner zeros
    position += config.k_inner;
    
    // Left inner
    value += config.inner as u64 * base.pow(position as u32);
    position += 1;
    
    // Left k_outer zeros  
    position += config.k_outer;
    
    // Left outer
    value += config.outer as u64 * base.pow(position as u32);
    
    Some(value)
}

fn test_membrane_config(config: &MembraneConfig) -> QuickResult {
    let candidates = generate_membranes_mvp(config);
    let total_tested = candidates.len();
    
    let mut primes_found = 0;
    let mut examples = Vec::new();
    
    for &candidate in &candidates {
        if is_prime_mvp(candidate) {
            primes_found += 1;
            if examples.len() < 5 {
                examples.push(candidate);
            }
        }
    }
    
    let density = if total_tested > 0 {
        primes_found as f64 / total_tested as f64
    } else {
        0.0
    };
    
    QuickResult {
        total_tested,
        primes_found,
        density,
        examples,
    }
}

fn print_usage() {
    println!("Membrane Scaling MVP Adapter");
    println!("Usage: membrane_mvp_adapter --base B --outer O --inner I --middle-length M --k-outer KO --k-inner KI");
    println!("       membrane_mvp_adapter --sweep --base B --outer O --inner I");
}

fn run_single_test(args: Vec<String>) {
    let mut config = MembraneConfig {
        base: 6,
        outer: 1, 
        inner: 5,
        k_outer: 0,
        k_inner: 0,
        middle_length: 1,
    };
    
    // Parse arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--base" => {
                config.base = args[i+1].parse().unwrap_or(6);
                i += 2;
            },
            "--outer" => {
                config.outer = args[i+1].parse().unwrap_or(1);
                i += 2;
            },
            "--inner" => {
                config.inner = args[i+1].parse().unwrap_or(5);
                i += 2;
            },
            "--middle-length" => {
                config.middle_length = args[i+1].parse().unwrap_or(1);
                i += 2;
            },
            "--k-outer" => {
                config.k_outer = args[i+1].parse().unwrap_or(0);
                i += 2;
            },
            "--k-inner" => {
                config.k_inner = args[i+1].parse().unwrap_or(0);
                i += 2;
            },
            _ => i += 1,
        }
    }
    
    println!("Testing config: {:?}", config);
    
    let result = test_membrane_config(&config);
    
    println!("Results:");
    println!("  Total tested: {}", result.total_tested);
    println!("  Primes found: {}", result.primes_found);
    println!("  Prime density: {:.6} ({:.2}%)", result.density, result.density * 100.0);
    
    if !result.examples.is_empty() {
        println!("  Examples: {:?}", result.examples);
    }
    
    // Output in format that Python can parse easily
    println!("MVP_RESULT: {} {} {:.6}",
             config.middle_length,
             config.k_outer + config.k_inner,
             result.density);
}

fn run_parameter_sweep(args: Vec<String>) {
    let mut base = 6u32;
    let mut outer = 1u32;
    let mut inner = 5u32;
    
    // Parse base configuration
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--base" => {
                base = args[i+1].parse().unwrap_or(6);
                i += 2;
            },
            "--outer" => {
                outer = args[i+1].parse().unwrap_or(1);
                i += 2;
            },
            "--inner" => {
                inner = args[i+1].parse().unwrap_or(5);
                i += 2;
            },
            _ => i += 1,
        }
    }
    
    println!("MVP Parameter Sweep: Base-{} ({}, {})", base, outer, inner);
    println!("M,k_outer,k_inner,k_total,density,primes_found");
    
    // Quick sweep for MVP
    for middle_length in 1..=4 {
        for k_outer in 0..=2 {
            for k_inner in 0..=2 {
                let config = MembraneConfig {
                    base,
                    outer,
                    inner,
                    k_outer,
                    k_inner,
                    middle_length,
                };
                
                let result = test_membrane_config(&config);

                println!("{},{},{},{},{:.6},{}",
                         middle_length,
                         k_outer,
                         k_inner,
                         k_outer + k_inner,
                         result.density,
                         result.primes_found);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    match args[1].as_str() {
        "--sweep" => run_parameter_sweep(args),
        "--help" | "-h" => print_usage(),
        _ => run_single_test(args),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_membrane_construction() {
        let config = MembraneConfig {
            base: 6,
            outer: 1,
            inner: 5,
            k_outer: 0,
            k_inner: 0,
            middle_length: 1,
        };
        
        // Test middle = 3: should give 15351 in base 6
        let result = build_membrane(&config, 3).unwrap();
        
        // Convert to base 10: 1*6^4 + 5*6^3 + 3*6^2 + 5*6 + 1
        let expected = 1*1296 + 5*216 + 3*36 + 5*6 + 1;
        assert_eq!(result, expected);
    }
    
    #[test] 
    fn test_basic_primality() {
        assert!(is_prime_mvp(2));
        assert!(is_prime_mvp(3));
        assert!(is_prime_mvp(5));
        assert!(!is_prime_mvp(4));
        assert!(!is_prime_mvp(9));
    }
    
    #[test]
    fn test_membrane_generation() {
        let config = MembraneConfig {
            base: 6,
            outer: 1,
            inner: 5,
            k_outer: 0,
            k_inner: 0,
            middle_length: 1,
        };
        
        let membranes = generate_membranes_mvp(&config);
        assert_eq!(membranes.len(), 6); // base^middle_length
        
        // All should be different values
        let mut sorted = membranes.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), membranes.len());
    }
}
