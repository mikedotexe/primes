//! Test the hardened core structs and safety validations

use prime_physics_engine::{PrimeUniverse, PhysicsError};
use prime_physics_engine::gravity::PrimeParticle;
use prime_physics_engine::membrane::MembraneConfig;
use num_bigint::BigUint;
use std::time::SystemTime;
fn main() {
    println!("Testing hardened core structures...\n");
    
    // Test 1: Invalid particle validation
    println!("🔒 Test 1: Invalid particle validation");
    let mut universe = PrimeUniverse::new();
    let bad_particle = PrimeParticle {
        value: BigUint::from(17u32),
        base: 10,
        position: [0.0, 0.0],
        velocity: [0.0, 0.0],
        mass: 0.0, // Invalid mass!
        charge: 1.0,
        name: "Bad Particle".to_string(),
        membrane_config: None,
        creation_time: SystemTime::now(),
        trajectory_history: Vec::new(),
        physics_cache: prime_physics_engine::gravity::PhysicsCache::default(),
    };
    match universe.add_particle(bad_particle) {
        Err(PhysicsError::InvalidConfiguration(msg)) => {
            println!("   ✅ Correctly rejected invalid mass: {}", msg);
        }
        _ => println!("   ❌ Failed to reject invalid mass"),
    }
    // Test 2: Non-finite values
    println!("\n🔒 Test 2: Non-finite values");
    let nan_particle = PrimeParticle {
        position: [f64::NAN, 0.0], // Invalid position!
        mass: 1.0,
        name: "NaN Particle".to_string(),
    match universe.add_particle(nan_particle) {
            println!("   ✅ Correctly rejected non-finite position: {}", msg);
        _ => println!("   ❌ Failed to reject non-finite position"),
    // Test 3: Valid particle acceptance
    println!("\n🔒 Test 3: Valid particle acceptance");
    let good_particle = PrimeParticle {
        position: [1.0, 2.0],
        velocity: [0.1, 0.2],
        mass: 1.5,
        charge: 0.8,
        name: "Good Particle".to_string(),
    match universe.add_particle(good_particle) {
        Ok(()) => {
            println!("   ✅ Correctly accepted valid particle");
            println!("   Universe now has {} particles", universe.particles.len());
        Err(e) => println!("   ❌ Rejected valid particle: {}", e),
    // Test 4: Invalid time step
    println!("\n🔒 Test 4: Invalid time step");
    universe.dt = 0.0; // Invalid time step!
    match universe.step() {
        Err(PhysicsError::IntegrationError(msg)) => {
            println!("   ✅ Correctly rejected invalid time step: {}", msg);
        _ => println!("   ❌ Failed to reject invalid time step"),
    // Test 5: Miller-Rabin parameter validation
    println!("\n🔒 Test 5: Miller-Rabin parameter validation");
    let test_num = BigUint::from(97u32);
    // Test with 0 rounds (should warn and use default)
    let result_zero = prime_physics_engine::miller_rabin_test(&test_num, 0);
    println!("   Miller-Rabin with 0 rounds: {} (should warn)", result_zero);
    // Test with excessive rounds (should warn and cap)
    let result_excessive = prime_physics_engine::miller_rabin_test(&test_num, 200);
    println!("   Miller-Rabin with 200 rounds: {} (should warn)", result_excessive);
    // Test 6: Non-coprime configuration warning
    println!("\n🔒 Test 6: Non-coprime configuration warning");
    let _bad_config = MembraneConfig::new(6, 3, 3, 0, 0); // Should warn
    println!("\n✅ All hardening tests completed!");
}
