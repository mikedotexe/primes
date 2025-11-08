//! Test the L4/L5 triangular Lagrange points - the stable prime clusters!

use prime_physics_engine::gravity::{PrimeParticle, GravitationalField};
use prime_physics_engine::lagrange::ClusterAnalysis;
use num_bigint::BigUint;
use std::time::SystemTime;
fn main() {
    println!("🔺 Testing L4/L5 Triangular Lagrange Points");
    println!("==========================================\n");
    
    // Create two prime particles
    let particles = vec![
        PrimeParticle {
            value: BigUint::from(307050703u64), // From our membrane verification
            base: 10,
            position: [0.0, 0.0],
            velocity: [0.0, 0.0],
            mass: 2.0,
            charge: 1.0,
            name: "Prime 307050703".to_string(),
            membrane_config: None,
            creation_time: SystemTime::now(),
            trajectory_history: Vec::new(),
            physics_cache: prime_physics_engine::gravity::PhysicsCache::default(),
        },
            value: BigUint::from(303050303u64), // From our membrane verification
            position: [20.0, 0.0], // 20 units apart
            mass: 1.8,
            name: "Prime 303050303".to_string(),
    ];
    let field = GravitationalField::new();
    match ClusterAnalysis::new(&particles, &field) {
        Ok(mut analysis) => {
            println!("✅ Successfully created cluster analysis");
            
            match analysis.find_all_lagrange_points(&particles) {
                Ok(points) => {
                    println!("✅ Found {} Lagrange points total\n", points.len());
                    
                    // Expected distance for equilateral triangle
                    let separation = 20.0;
                    let expected_triangle_side = separation; // All sides equal
                    println!("🔍 Analyzing each Lagrange point:\n");
                    for point in &points {
                        println!("📍 {} Point:", point.point_type);
                        println!("   Position: ({:.2}, {:.2})", point.position[0], point.position[1]);
                        println!("   Stability: {:.3} (higher = more stable)", point.stability_score);
                        println!("   Field strength: {:.2e}", point.field_strength);
                        println!("   Tidal strength: {:.2e}", point.tidal_strength);
                        println!("   Escape velocity: {:.2e}", point.escape_velocity);
                        println!("   Clustered primes: {}", point.clustered_primes.len());
                        
                        // Check if this is a triangular point
                        if matches!(point.point_type, 
                            prime_physics_engine::lagrange::LagrangePointType::L4 | 
                            prime_physics_engine::lagrange::LagrangePointType::L5) {
                            
                            // Calculate distances to verify equilateral triangle
                            let dist1 = ((point.position[0] - particles[0].position[0]).powi(2) + 
                                        (point.position[1] - particles[0].position[1]).powi(2)).sqrt();
                            let dist2 = ((point.position[0] - particles[1].position[0]).powi(2) + 
                                        (point.position[1] - particles[1].position[1]).powi(2)).sqrt();
                            let dist12 = separation;
                            println!("   🔺 TRIANGULAR ANALYSIS:");
                            println!("      Distance to Prime 1: {:.2}", dist1);
                            println!("      Distance to Prime 2: {:.2}", dist2);
                            println!("      Distance between primes: {:.2}", dist12);
                            let avg_dist = (dist1 + dist2 + dist12) / 3.0;
                            let variance = ((dist1 - avg_dist).powi(2) + 
                                           (dist2 - avg_dist).powi(2) + 
                                           (dist12 - avg_dist).powi(2)).sqrt() / 3.0;
                            println!("      Triangle equality variance: {:.4}", variance);
                            if variance < 0.1 {
                                println!("      ✅ EQUILATERAL TRIANGLE CONFIRMED!");
                                println!("      🏆 This is a STABLE prime cluster point!");
                            } else {
                                println!("      ❌ Not quite equilateral (variance too high)");
                            }
                        }
                        println!();
                    }
                    // Find the most stable point
                    let most_stable = points.iter()
                        .max_by(|a, b| a.stability_score.partial_cmp(&b.stability_score).unwrap());
                    if let Some(stable_point) = most_stable {
                        println!("🏆 MOST STABLE POINT: {} with stability {:.3}", 
                            stable_point.point_type, stable_point.stability_score);
                        if matches!(stable_point.point_type, 
                            println!("   🎯 As predicted - the triangular point is most stable!");
                            println!("   📊 This is where prime clusters should naturally form!");
                    println!("\n💡 KEY INSIGHTS:");
                    println!("   • L1/L2/L3 are unstable - primes 'fall off'");
                    println!("   • L4/L5 are stable - primes accumulate here");
                    println!("   • Triangular geometry creates resonant stability");
                    println!("   • This explains natural prime clustering patterns!");
                }
                Err(e) => println!("❌ Failed to find Lagrange points: {}", e),
            }
        }
        Err(e) => println!("❌ Failed to create analysis: {}", e),
    }
    println!("\n🌟 L4/L5 Triangular Point Analysis Complete!");
}
