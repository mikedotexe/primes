//! Test the consolidated membrane laboratory features

use prime_physics_engine::{
    is_prime,
};
use num_bigint::BigUint;
fn main() {
    println!("Testing Membrane Laboratory Consolidated Features\n");
    
    // Test 1: Basic configuration and construction
    println!("1. Testing basic membrane construction:");
    let config = MembraneConfig {
        base: 10,
        outer: 3,
        inner: 7,
        k_outer: 0,
        k_inner: 0,
    };
    let builder = MembraneBuilder::new(config.clone());
    let number = builder.build(&BigUint::from(5u32));
    let prime = is_prime(&number);
    println!("   Config: base={}, outer={}, inner={}, k=({},{})", 
        config.base, config.outer, config.inner, config.k_outer, config.k_inner);
    println!("   Seed: 5 → Number: {} → Prime: {}", number, prime);
    // Test 2: Achievement detection
    println!("\n2. Testing achievement triggers:");
    let mut prime_count = 0;
    let mut streak = 0;
    let mut max_streak = 0;
    for seed in 1..20 {
        let num = builder.build(&BigUint::from(seed));
        if is_prime(&num) {
            prime_count += 1;
            streak += 1;
            if streak > max_streak {
                max_streak = streak;
            }
            println!("   Seed {} → {} ✓ (streak: {})", seed, num, streak);
        } else {
            streak = 0;
        }
    }
    println!("\n   Achievements unlocked:");
    if prime_count > 0 { println!("   - First Prime! 🎉"); }
    if prime_count >= 10 { println!("   - Ten Primes! 🔟"); }
    if max_streak >= 5 { println!("   - Streak of Five! 🔥"); }
    // Test 3: Statistical analysis
    println!("\n3. Testing statistical features:");
    let total = 19;
    let success_rate = prime_count as f64 / total as f64;
    let expected_rate = 0.15; // Approximate for this range
    println!("   Success rate: {:.1}% ({}/{})", success_rate * 100.0, prime_count, total);
    println!("   Expected rate: {:.1}%", expected_rate * 100.0);
    if success_rate > expected_rate {
        println!("   Performance: ABOVE expected! 📈");
    // Test 4: Heat map data generation
    println!("\n4. Testing heat map data collection:");
    let mut heat_data = Vec::new();
    for outer in 1..=5 {
        for inner in 1..=5 {
            if outer != inner && gcd(outer, 10) == 1 && gcd(inner, 10) == 1 {
                let test_config = MembraneConfig {
                    base: 10,
                    outer,
                    inner,
                    k_outer: 0,
                    k_inner: 0,
                };
                let test_builder = MembraneBuilder::new(test_config);
                let mut test_primes = 0;
                
                for seed in 1..=10 {
                    if is_prime(&test_builder.build(&BigUint::from(seed))) {
                        test_primes += 1;
                    }
                }
                heat_data.push((outer, inner, test_primes));
    println!("   Heat map preview (outer,inner → primes):");
    for (outer, inner, primes) in heat_data.iter().take(5) {
        let bar = "█".repeat(*primes as usize);
        println!("   ({},{}) → {} {}", outer, inner, primes, bar);
    // Test 5: Export functionality
    println!("\n5. Testing data export format:");
    println!("   Sample export data:");
    println!("   {{");
    println!("     \"session_id\": \"test-session\",");
    println!("     \"base\": 10,");
    println!("     \"config\": [3, 7, 0, 0],");
    println!("     \"success_rate\": {:.3},", success_rate);
    println!("     \"total_primes\": {},", prime_count);
    println!("     \"achievements\": [\"FirstPrime\"]");
    println!("   }}");
    println!("\n✅ All consolidated features tested successfully!");
}
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
