//! Test the hardened Lagrange point analysis system

use primes::{PrimeUniverse, PhysicsError};
use primes::gravity::{PrimeParticle, GravitationalField};
use primes::lagrange::{ClusterAnalysis, TidalForce};
use num_bigint::BigUint;
use std::time::SystemTime;
fn main() {
    println!("🔒 Testing Hardened Lagrange Point Analysis");
    println!("==========================================\n");
    
    // Test 1: Empty particle list
    println!("Test 1: Empty particle validation");
    let empty_particles = vec![];
    let field = GravitationalField::new();
    match ClusterAnalysis::new(&empty_particles, &field) {
        Err(PhysicsError::LagrangeError(msg)) => {
            println!("   ✅ Correctly rejected empty particle list: {}", msg);
        }
        _ => println!("   ❌ Failed to reject empty particle list"),
    }
    // Test 2: Too many particles
    println!("\nTest 2: Particle count limit");
    let too_many_particles: Vec<PrimeParticle> = (0..20000).map(|i| {
        PrimeParticle {
            value: BigUint::from(17u32),
            base: 10,
            position: [i as f64, 0.0],
            velocity: [0.0, 0.0],
            mass: 1.0,
            charge: 1.0,
            name: format!("Particle {}", i),
            membrane_config: None,
            creation_time: SystemTime::now(),
            trajectory_history: Vec::new(),
            physics_cache: primes::gravity::PhysicsCache::default(),
    }).collect();
    match ClusterAnalysis::new(&too_many_particles, &field) {
            println!("   ✅ Correctly rejected too many particles: {}", msg);
        _ => println!("   ❌ Failed to reject too many particles"),
    // Test 3: Invalid particle mass
    println!("\nTest 3: Invalid particle mass");
    let bad_particle = PrimeParticle {
        value: BigUint::from(17u32),
        base: 10,
        position: [0.0, 0.0],
        velocity: [0.0, 0.0],
        mass: -1.0, // Invalid mass
        charge: 1.0,
        name: "Bad Particle".to_string(),
        membrane_config: None,
        creation_time: SystemTime::now(),
        trajectory_history: Vec::new(),
        physics_cache: primes::gravity::PhysicsCache::default(),
    };
    match ClusterAnalysis::new(&vec![bad_particle], &field) {
            println!("   ✅ Correctly rejected invalid mass: {}", msg);
        _ => println!("   ❌ Failed to reject invalid mass"),
    // Test 4: Non-finite positions
    println!("\nTest 4: Non-finite positions");
    let nan_particle = PrimeParticle {
        position: [f64::NAN, 0.0], // Invalid position
        mass: 1.0,
        name: "NaN Particle".to_string(),
    match ClusterAnalysis::new(&vec![nan_particle], &field) {
            println!("   ✅ Correctly rejected non-finite position: {}", msg);
        _ => println!("   ❌ Failed to reject non-finite position"),
    // Test 5: Particles too close together
    println!("\nTest 5: Particles too close together");
    let close_particles = vec![
            position: [0.0, 0.0],
            name: "Particle A".to_string(),
        },
            value: BigUint::from(19u32),
            position: [1e-15, 0.0], // Too close to first particle
            name: "Particle B".to_string(),
    ];
    match ClusterAnalysis::new(&close_particles, &field) {
            println!("   ✅ Correctly rejected close particles: {}", msg);
        _ => println!("   ❌ Failed to reject close particles"),
    // Test 6: Valid analysis
    println!("\nTest 6: Valid Lagrange point analysis");
    let valid_particles = vec![
            mass: 2.0,
            name: "Prime 17".to_string(),
            position: [10.0, 0.0],
            mass: 1.5,
            name: "Prime 19".to_string(),
    match ClusterAnalysis::new(&valid_particles, &field) {
        Ok(mut analysis) => {
            println!("   ✅ Successfully created analysis");
            
            // Try to find Lagrange points
            match analysis.find_all_lagrange_points(&valid_particles) {
                Ok(points) => {
                    println!("   ✅ Found {} Lagrange points", points.len());
                    
                    for (i, point) in points.iter().enumerate() {
                        println!("     L{}: position=({:.2}, {:.2}), stability={:.3}, field={:.2e}", 
                            i+1, point.position[0], point.position[1], 
                            point.stability_score, point.field_strength);
                    }
                    println!("   Analysis successful: {}", analysis.analysis_successful);
                    println!("   Computation time: {:.3}s", analysis.computation_time);
                    println!("   Total captured primes: {}", analysis.total_captured);
                }
                Err(e) => println!("   ❌ Failed to find Lagrange points: {}", e),
            }
        Err(e) => println!("   ❌ Failed to create analysis: {}", e),
    // Test 7: Tidal force validation
    println!("\nTest 7: Tidal force validation");
    // Invalid tidal force
    match TidalForce::new(f64::NAN) {
            println!("   ✅ Correctly rejected NaN tidal force: {}", msg);
        _ => println!("   ❌ Failed to reject NaN tidal force"),
    // Valid tidal force
    match TidalForce::new(5.0) {
        Ok(force) => {
            println!("   ✅ Successfully created tidal force: strength={:.1}", force.strength);
            println!("   Is valid: {}", force.is_valid());
        Err(e) => println!("   ❌ Failed to create valid tidal force: {}", e),
    println!("\n🎯 All Lagrange point hardening tests completed!");
}
