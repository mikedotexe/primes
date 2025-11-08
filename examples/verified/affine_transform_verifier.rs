//! Affine Transform Verifier - Demonstrates the mathematical correctness of the affine transform
//! 
//! This example provides information-rich output showing:
//! 1. How M(c) mod p transforms to (s + g*c) mod p
//! 2. Performance comparisons between traditional and affine methods
//! 3. Verification that both methods produce identical results

use prime_physics_engine::membrane::MembraneConfig;
use std::time::Instant;
use num_bigint::BigUint;
use num_traits::Zero;
/// Compute membrane value for seed c
fn compute_membrane(base: u32, config: &MembraneConfig, seed: u32) -> BigUint {
    let outer = config.outer;
    let inner = config.inner;
    let k_outer = config.k_outer;
    let k_inner = config.k_inner;
    
    // Width calculation
    let width = 2 * (1 + k_outer + 1 + k_inner) + 1;
    // Build the membrane polynomial
    let mut value = BigUint::zero();
    let base_big = BigUint::from(base);
    // Left outer
    value += BigUint::from(outer) * base_big.pow(width - 1);
    // Left inner  
    value += BigUint::from(inner) * base_big.pow(width - 2 - k_outer);
    // Center (seed)
    value += BigUint::from(seed) * base_big.pow(width / 2);
    // Right inner
    value += BigUint::from(inner) * base_big.pow(k_inner + 1);
    // Right outer
    value += BigUint::from(outer);
    value
}
/// Compute affine signature for a prime
fn compute_affine_signature(base: u32, config: &MembraneConfig, prime: u32) -> (u32, u32) {
    let width = 2 * (1 + config.k_outer + 1 + config.k_inner) + 1;
    // s = M(0) mod p
    let m0 = compute_membrane(base, config, 0);
    let s = (m0 % prime).try_into().unwrap_or(0);
    // g = b^(w/2) mod p
    let base_big = BigUint::from(base);
    let g = (base_big.pow(width / 2) % prime).try_into().unwrap_or(0);
    (s, g)
}
fn main() {
    println!("🔬 Affine Transform Verification");
    println!("================================\n");
    // Test configuration
    let base = 10u32;
    let config = MembraneConfig::new(base, 3, 7, 1, 1);
    println!("Configuration:");
    println!("  Base: {base}");
    println!("  Membrane: ({},{}) k=({},{})", config.outer, config.inner, config.k_outer, config.k_inner);
    println!("  Pattern: {} {} {} {} [seed] {} {} {} {}", 
        config.outer, "0".repeat(config.k_outer as usize), config.inner, "0".repeat(config.k_inner as usize),
        "0".repeat(config.k_inner as usize), config.inner, "0".repeat(config.k_outer as usize), config.outer);
    // Test against multiple primes
    let test_primes = vec![7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
    let test_seeds = vec![0, 1, 2, 3, 5, 10, 50, 100];
    println!("\n📊 Step 1: Computing Affine Signatures");
    println!("=====================================");
    let mut signatures = Vec::new();
    for &prime in &test_primes {
        let (s, g) = compute_affine_signature(base, &config, prime);
        signatures.push((prime, s, g));
        
        println!("\nPrime {prime}: s = {s}, g = {g}");
        // Show the mathematical derivation
        let m0 = compute_membrane(base, &config, 0);
        let m1 = compute_membrane(base, &config, 1);
        println!("  M(0) = {m0} ≡ {s} (mod {prime})");
        println!("  M(1) = {} ≡ {} (mod {})", m1, (&m1 % prime), prime);
        println!("  M(1) - M(0) = {} ≡ {} (mod {})", &m1 - &m0, g, prime);
    }
    println!("\n📊 Step 2: Verifying Transform Correctness");
    println!("========================================");
    let mut all_match = true;
    for &seed in &test_seeds {
        println!("\nSeed = {seed}:");
        let membrane_value = compute_membrane(base, &config, seed);
        println!("  M({seed}) = {membrane_value}");
        for &(prime, s, g) in &signatures {
            // Traditional method
            let traditional = (&membrane_value % prime).try_into().unwrap_or(0);
            
            // Affine method
            let affine = (s + seed * g) % prime;
            let matches = traditional == affine;
            all_match &= matches;
            println!("  mod {}: traditional = {}, affine = ({} + {} × {}) mod {} = {} {}",
                prime, traditional, s, seed, g, prime, affine,
                if matches { "✓" } else { "✗ MISMATCH!" });
        }
    }
    if all_match {
        println!("\n✅ All {} tests passed! The affine transform is mathematically correct.",
            test_seeds.len() * test_primes.len());
    } else {
        println!("\n❌ Some tests failed! The affine transform has errors.");
    }
    println!("\n📊 Step 3: Performance Comparison");
    println!("================================");
    let num_tests = 1_000_000;
    println!("\nTesting {} membrane values against {} primes...", num_tests, test_primes.len());
    // Traditional method timing
    let start = Instant::now();
    let mut traditional_sum = 0u64;
    for seed in 0..num_tests {
        let m = compute_membrane(base, &config, seed);
        for &prime in &test_primes {
            traditional_sum += (&m % prime).try_into().unwrap_or(0) as u64;
        }
    }
    let traditional_time = start.elapsed();
    
    // Affine method timing
    let start = Instant::now();
    let mut affine_sum = 0u64;
    for seed in 0..num_tests {
        for &(prime, s, g) in &signatures {
            affine_sum += ((s + seed * g) % prime) as u64;
        }
    }
    let affine_time = start.elapsed();
    
    // Verify sums match (additional correctness check)
    if traditional_sum != affine_sum {
        println!("\n⚠️  Warning: Sums don't match! Traditional: {traditional_sum}, Affine: {affine_sum}");
    }
    println!("\nTraditional method:");
    println!("  Time: {:.3}s", traditional_time.as_secs_f64());
    println!("  Tests/sec: {:.0}", num_tests as f64 * test_primes.len() as f64 / traditional_time.as_secs_f64());
    println!("\nAffine method:");
    println!("  Time: {:.3}s", affine_time.as_secs_f64());
    println!("  Tests/sec: {:.0}", num_tests as f64 * test_primes.len() as f64 / affine_time.as_secs_f64());
    let speedup = traditional_time.as_secs_f64() / affine_time.as_secs_f64();
    println!("\n🚀 Speedup: {speedup:.1}x");
    println!("\n📊 Step 4: Computational Complexity Analysis");
    println!("==========================================");
    println!("\nTraditional method per test:");
    println!("  1. Compute M(seed) - O(log base)");
    println!("  2. BigInt division M(seed) mod p - O(log² M)");
    println!("  Total: ~20-50 cycles for 32-bit division");
    println!("\nAffine method per test:");
    println!("  1. Multiply: seed × g - 1 cycle");
    println!("  2. Add: s + result - 1 cycle");
    println!("  3. Modulo: result mod p - 1-3 cycles");
    println!("  Total: ~3-5 cycles");
    println!("\nTheoretical speedup: 20-50 / 3-5 = 4-17x");
    println!("Measured speedup: {speedup:.1}x");
    println!("\n📊 Step 5: Why This Works");
    println!("========================");
    println!("\nThe membrane polynomial has a special structure:");
    println!("  M(c) = [constant terms] + c × b^(w/2) + [constant terms]");
    println!("\nThis means:");
    println!("  M(c+1) - M(c) = b^(w/2) (always!)");
    println!("\nIn modular arithmetic:");
    println!("  M(c) ≡ M(0) + c × b^(w/2) (mod p)");
    println!("       ≡ s + c × g (mod p)");
    println!("\nThe sequence forms an arithmetic progression in every modular system!");
}
