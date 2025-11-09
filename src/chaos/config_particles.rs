//! Configuration-Based Particle Generation for Chaos Measurement
//! ===========================================================
//!
//! This module generates test particles specifically tailored to each
//! membrane configuration, enabling accurate chaos measurements that
//! reflect the true dynamics of each configuration.

use crate::gravity::PrimeParticle;
use crate::membrane::{MembraneBuilder, MembraneConfig};
use crate::{PhysicsError, PhysicsResult};
use num_bigint::BigUint;

/// Generate a pair of test particles based on membrane configuration
pub fn generate_test_particles(
    config: &MembraneConfig,
    separation: f64,
) -> PhysicsResult<Vec<PrimeParticle>> {
    // Strategy 1: Try to generate actual membrane primes
    if let Ok(particles) = generate_membrane_particles(config, separation) {
        return Ok(particles);
    }

    // Strategy 2: Fall back to configuration-characteristic primes
    generate_characteristic_particles(config, separation)
}

/// Attempt to generate actual membrane-constructed primes
fn generate_membrane_particles(
    config: &MembraneConfig,
    separation: f64,
) -> PhysicsResult<Vec<PrimeParticle>> {
    let mut particles = Vec::new();

    // Try different seeds to get two distinct primes
    let seeds = get_optimal_seeds(config);

    for (i, &seed) in seeds.iter().take(2).enumerate() {
        let position = if i == 0 {
            [-separation / 2.0, 0.0]
        } else {
            [separation / 2.0, 0.0]
        };

        let velocity = if i == 0 { [0.0, 0.5] } else { [0.0, -0.5] };

        let builder = MembraneBuilder::new(config.clone())
            .with_position(position)
            .with_velocity(velocity)
            .with_name(format!("Config-P{}", i + 1))
            .with_seed(seed)
            .with_max_attempts(100); // Limited attempts for speed

        if let Ok(particle) = builder.build() {
            particles.push(particle);
        }
    }

    if particles.len() == 2 {
        Ok(particles)
    } else {
        Err(PhysicsError::PrimeGenerationFailed {
            attempts: seeds.len() * 100,
        })
    }
}

/// Generate characteristic primes that match configuration properties
fn generate_characteristic_particles(
    config: &MembraneConfig,
    separation: f64,
) -> PhysicsResult<Vec<PrimeParticle>> {
    let characteristic_primes = get_characteristic_primes(config);

    if characteristic_primes.len() < 2 {
        return Err(PhysicsError::InvalidConfiguration(
            "Cannot find characteristic primes for configuration".to_string(),
        ));
    }

    let particles = characteristic_primes
        .into_iter()
        .take(2)
        .enumerate()
        .map(|(i, prime_value)| {
            let position = if i == 0 {
                [-separation / 2.0, 0.0]
            } else {
                [separation / 2.0, 0.0]
            };

            let velocity = if i == 0 { [0.0, 0.5] } else { [0.0, -0.5] };

            PrimeParticle::new(
                prime_value,
                config.base,
                position,
                velocity,
                format!("Char-P{}", i + 1),
            )
        })
        .collect();

    Ok(particles)
}

/// Get optimal seeds based on configuration research
fn get_optimal_seeds(config: &MembraneConfig) -> Vec<u8> {
    match (config.base, config.outer, config.inner) {
        // Known high-performance seeds from research
        (10, 3, 7) => vec![3, 5, 2, 8, 0], // Classic (3,7) optimal seeds
        (10, 3, 3) => vec![5, 1, 9, 4, 6], // Twin boundaries prefer 5
        (10, 1, 1) => vec![2, 3, 5, 7, 8], // Minimal structure
        (10, 7, 7) => vec![3, 1, 9, 5, 2], // Twin 7s
        (11, _, _) => vec![2, 3, 5, 7, 8], // Base 11 generic
        (12, 5, 7) => vec![1, 3, 4, 6, 8], // Bridge configuration
        _ => vec![1, 2, 3, 4, 5, 6, 7, 8, 9], // Generic fallback
    }
}

/// Get characteristic primes that embody configuration properties
fn get_characteristic_primes(config: &MembraneConfig) -> Vec<BigUint> {
    use num_traits::FromPrimitive;

    let primes: Vec<u64> = match (
        config.base,
        config.outer,
        config.inner,
        config.k_outer,
        config.k_inner,
    ) {
        // High chaos configurations - use widely separated primes
        (10, 9, 9, k_outer, k_inner) if k_outer > 2 && k_inner > 2 => {
            vec![97, 10007, 100003, 1000003] // Large gap creates chaos
        }

        // Classic (3,7) - use primes with 3 and 7 patterns
        (10, 3, 7, 2, 2) => {
            vec![37, 73, 137, 173, 337, 373, 737, 773]
        }

        // Twin boundaries - use twin primes
        (10, x, y, _, _) if x == y => {
            vec![3, 5, 11, 13, 17, 19, 29, 31, 41, 43, 59, 61, 71, 73]
        }

        // Minimal structure - small primes
        (10, 1, 1, 1, 1) => {
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        }

        // Base 11 - primes that look good in base 11
        (11, _, _, _, _) => {
            vec![11, 23, 67, 89, 199, 331, 463] // Nice in base 11
        }

        // Base 12 - avoid even patterns
        (12, _, _, _, _) => {
            vec![5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
        }

        // Heavy symmetric - large structured primes
        (_, outer, inner, k_outer, k_inner)
            if k_outer >= 3 && k_inner >= 3 && outer > 5 && inner > 5 =>
        {
            vec![8999, 9001, 9007, 9013] // Large, closely spaced
        }

        // Breathing patterns - primes with asymmetric digit patterns
        (10, _, _, _, _)
            if matches!(
                &config.construction_type,
                crate::membrane::ConstructionType::Breathing { .. }
            ) =>
        {
            vec![139, 193, 397, 739, 937] // Asymmetric digit patterns
        }

        // Default: classic prime selection
        _ => {
            vec![41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]
        }
    };

    primes
        .into_iter()
        .map(|p| BigUint::from_u64(p).unwrap())
        .collect()
}

/// Generate particles specifically for chaos measurement
pub fn generate_chaos_test_particles(
    config: &MembraneConfig,
) -> PhysicsResult<(Vec<PrimeParticle>, Vec<PrimeParticle>)> {
    // Generate main particles
    let main_particles = generate_test_particles(config, 2.0)?;

    // Generate shadow particles with tiny offset
    let shadow_particles = main_particles
        .iter()
        .map(|p| {
            let mut shadow = p.clone();
            // Add small perturbation to position
            shadow.position[0] += 1e-8;
            shadow.position[1] += 1e-8;
            shadow.name = format!("{}-shadow", p.name);
            shadow
        })
        .collect();

    Ok((main_particles, shadow_particles))
}

/// Analyze expected chaos level based on configuration
pub fn estimate_chaos_level(config: &MembraneConfig) -> f64 {
    // Base chaos from structure
    let structure_chaos = match &config.construction_type {
        crate::membrane::ConstructionType::Symmetric => 0.1,
        crate::membrane::ConstructionType::Breathing { .. } => 0.3,
        crate::membrane::ConstructionType::Adaptive { .. } => 0.2,
        crate::membrane::ConstructionType::Quantum { .. } => 0.5,
    };

    // Chaos from boundary values
    let boundary_chaos = match (config.outer, config.inner) {
        (9, 9) => 0.8,           // Maximum boundaries create high chaos
        (3, 7) => 0.2,           // Classic stable configuration
        (x, y) if x == y => 0.3, // Twin boundaries
        (1, 1) => 0.1,           // Minimal structure
        _ => 0.4,
    };

    // Chaos from k-values
    let k_chaos = (config.k_outer + config.k_inner) as f64 * 0.05;

    // Total estimated chaos
    structure_chaos + boundary_chaos + k_chaos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_generation() {
        let config = MembraneConfig::new(10, 3, 7, 2, 2);
        let particles = generate_test_particles(&config, 2.0).unwrap();

        assert_eq!(particles.len(), 2);
        assert_eq!(particles[0].base, 10);
        assert_eq!(particles[1].base, 10);

        // Check separation
        let dx = particles[1].position[0] - particles[0].position[0];
        assert!((dx - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_chaos_estimation() {
        let config1 = MembraneConfig::new(10, 3, 7, 2, 2);
        let chaos1 = estimate_chaos_level(&config1);

        let config2 = MembraneConfig::new(10, 9, 9, 3, 3);
        let chaos2 = estimate_chaos_level(&config2);

        // Heavy symmetric should have higher chaos
        assert!(chaos2 > chaos1);
    }
}
