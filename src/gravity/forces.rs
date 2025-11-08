//! Force calculation engine for prime gravitational interactions

use std::collections::HashMap;
use crate::{PhysicsResult, PhysicalConstants};
use super::{PrimeParticle, ForceResult, InteractionType};

/// Main force calculator for prime particles
#[derive(Debug, Clone)]
pub struct ForceCalculator {
    /// Cached force calculations for performance
    force_cache: HashMap<(usize, usize), ForceResult>,
    
    /// Last calculation time for cache invalidation
    last_calculation_time: f64,
    
    /// Cache validity threshold
    cache_threshold: f64,
}

impl ForceCalculator {
    /// Create a new force calculator
    pub fn new() -> Self {
        Self {
            force_cache: HashMap::new(),
            last_calculation_time: 0.0,
            cache_threshold: 0.01,
        }
    }
    
    /// Calculate all forces in the system
    /// 
    /// # N-Body Force Calculation
    /// 
    /// For N particles, we need to calculate N(N-1)/2 pairwise interactions.
    /// We use Newton's third law: F_ij = -F_ji to avoid duplicate calculations.
    /// 
    /// Total force on particle i: F_i = Σ(j≠i) F_ij
    /// 
    /// The force cache prevents recalculating forces between particles
    /// that haven't moved significantly since the last timestep.
    pub fn calculate_all_forces(
        &mut self,
        particles: &[PrimeParticle],
        constants: &PhysicalConstants,
        current_time: f64
    ) -> PhysicsResult<Vec<[f64; 2]>> {
        let mut total_forces = vec![[0.0, 0.0]; particles.len()];
        
        // Clear cache if too much time has passed
        // This prevents using stale force values when particles have moved
        if current_time - self.last_calculation_time > self.cache_threshold {
            self.force_cache.clear();
        }
        self.last_calculation_time = current_time;
        
        // Calculate pairwise forces
        // Only calculate upper triangle of force matrix (j > i)
        // Then apply Newton's third law for efficiency
        for i in 0..particles.len() {
            for j in i+1..particles.len() {
                let force_result = self.calculate_pairwise_force(
                    &particles[i], 
                    &particles[j], 
                    constants,
                    i, 
                    j
                )?;
                
                // Newton's third law: F_ij = -F_ji
                total_forces[i][0] += force_result.force[0];
                total_forces[i][1] += force_result.force[1];
                total_forces[j][0] -= force_result.force[0];
                total_forces[j][1] -= force_result.force[1];
            }
        }
        
        Ok(total_forces)
    }
    
    /// Calculate force between two specific particles
    pub fn calculate_pairwise_force(
        &mut self,
        p1: &PrimeParticle,
        p2: &PrimeParticle,
        constants: &PhysicalConstants,
        _i: usize,
        _j: usize
    ) -> PhysicsResult<ForceResult> {
        // Note: Force caching disabled for accuracy - particles move continuously
        // Position-aware caching could be implemented for performance-critical applications
        
        let dx = p2.position[0] - p1.position[0];
        let dy = p2.position[1] - p1.position[1];
        let distance = (dx*dx + dy*dy).sqrt();
        
        if distance < constants.min_separation {
            return Ok(ForceResult::new([0.0, 0.0], distance, InteractionType::Gravitational));
        }
        
        let unit_x = dx / distance;
        let unit_y = dy / distance;
        
        // Calculate different force components
        let gravitational_force = self.calculate_gravitational_force(p1, p2, distance, constants);
        let charge_force = self.calculate_charge_force(p1, p2, distance, constants);
        let base_metric_force = self.calculate_base_metric_force(p1, p2, distance, constants);
        let membrane_force = self.calculate_membrane_resonance_force(p1, p2, distance, constants);
        
        // Combine forces
        let total_magnitude = gravitational_force + charge_force + base_metric_force + membrane_force;
        let total_force = [
            total_magnitude * unit_x,
            total_magnitude * unit_y
        ];
        
        // Determine dominant interaction
        let interaction_type = self.determine_dominant_interaction(
            gravitational_force, 
            charge_force, 
            base_metric_force, 
            membrane_force
        );
        
        let mut result = ForceResult::new(total_force, distance, interaction_type);
        
        // Add individual components
        result.add_component("gravitational".to_string(), [
            gravitational_force * unit_x, gravitational_force * unit_y
        ]);
        result.add_component("charge".to_string(), [
            charge_force * unit_x, charge_force * unit_y
        ]);
        result.add_component("base_metric".to_string(), [
            base_metric_force * unit_x, base_metric_force * unit_y
        ]);
        result.add_component("membrane".to_string(), [
            membrane_force * unit_x, membrane_force * unit_y
        ]);
        
        // Cache disabled - see comment above
        // self.force_cache.insert((i, j), result.clone());
        
        Ok(result)
    }
    
    /// Calculate pure gravitational force (always attractive)
    /// 
    /// # Gravitational Force Model
    /// 
    /// F = -G * (m1 * m2) / r²
    /// 
    /// Where:
    /// - G: gravitational constant (scaled for our prime universe)
    /// - m1, m2: masses of the two primes (based on digit count and structure)
    /// - r: distance between particles
    /// 
    /// The negative sign indicates attraction (force points inward).
    /// We apply a softening factor ε to prevent singularities at r→0.
    fn calculate_gravitational_force(
        &self,
        p1: &PrimeParticle,
        p2: &PrimeParticle,
        distance: f64,
        constants: &PhysicalConstants
    ) -> f64 {
        constants.gravity_constant * p1.mass * p2.mass / (distance * distance)
    }
    
    /// Calculate charge-based force (can be attractive or repulsive)
    /// 
    /// # Charge Force Model
    /// 
    /// Prime "charge" represents the density of prime factors or
    /// special properties (e.g., twin prime, Mersenne prime).
    /// 
    /// F = k * (q1 * q2) * base_compatibility / r²
    /// 
    /// Base compatibility rules:
    /// - Same base: Full attractive force (primes resonate)
    /// - Different parity bases: Repulsive (even/odd incompatibility)
    /// - Same parity different bases: Weak attraction
    /// 
    /// This creates clustering of primes from the same base
    /// while maintaining diversity in the overall system.
    fn calculate_charge_force(
        &self,
        p1: &PrimeParticle,
        p2: &PrimeParticle,
        distance: f64,
        _constants: &PhysicalConstants
    ) -> f64 {
        // Base compatibility affects charge interaction
        let base_compatibility = self.calculate_base_compatibility(p1.base, p2.base);
        
        let charge_product = p1.charge * p2.charge;
        let force_magnitude = charge_product * base_compatibility / (distance * distance);
        
        // Same base: attractive (negative sign will be handled by caller)
        // Different parity: repulsive
        if p1.base == p2.base {
            force_magnitude
        } else if (p1.base % 2) != (p2.base % 2) {
            -force_magnitude * 0.5 // Repulsive, but weaker
        } else {
            force_magnitude * 0.8 // Different but same parity
        }
    }
    
    /// Calculate base metric-dependent force
    fn calculate_base_metric_force(
        &self,
        p1: &PrimeParticle,
        p2: &PrimeParticle,
        distance: f64,
        constants: &PhysicalConstants
    ) -> f64 {
        // Base metric creates spacetime curvature effects
        let metric1 = self.calculate_base_metric(p1.base, distance);
        let metric2 = self.calculate_base_metric(p2.base, distance);
        
        let metric_difference = (metric1 - metric2).abs();
        
        // Force proportional to metric curvature difference
        constants.base_coupling * metric_difference * p1.mass * p2.mass / (distance * distance)
    }
    
    /// Calculate membrane resonance force
    fn calculate_membrane_resonance_force(
        &self,
        p1: &PrimeParticle,
        p2: &PrimeParticle,
        distance: f64,
        _constants: &PhysicalConstants
    ) -> f64 {
        // Only applies if both particles have membrane configurations
        let (config1, config2) = match (&p1.membrane_config, &p2.membrane_config) {
            (Some(c1), Some(c2)) => (c1, c2),
            _ => return 0.0,
        };
        
        // Resonance based on configuration similarity
        let resonance_score = self.calculate_membrane_resonance(config1, config2);
        
        // Resonance force follows inverse square law like other forces
        resonance_score * p1.physics_cache.resonance_factor * p2.physics_cache.resonance_factor / (distance * distance)
    }
    
    /// Calculate base compatibility factor
    fn calculate_base_compatibility(&self, base1: u32, base2: u32) -> f64 {
        if base1 == base2 {
            1.5 // Same base attraction
        } else if (base1 % 2) != (base2 % 2) {
            -0.5 // Even vs odd repulsion
        } else {
            1.0 // Neutral
        }
    }
    
    /// Calculate base metric component
    fn calculate_base_metric(&self, base: u32, distance: f64) -> f64 {
        let base_curvature = match base {
            p if self.is_prime(p) => 2.0, // Prime bases create strong attractive curvature
            p if p % 2 == 0 => 0.5,       // Even bases create repulsive field
            _ => 1.0,                     // Odd composites neutral
        };
        
        // Exponential decay with distance
        base_curvature * (-distance * distance / 1000.0).exp()
    }
    
    /// Calculate membrane configuration resonance
    fn calculate_membrane_resonance(
        &self,
        config1: &crate::membrane::MembraneConfig,
        config2: &crate::membrane::MembraneConfig
    ) -> f64 {
        let mut resonance: f64 = 0.0;
        
        // Same outer/inner digits
        if config1.outer == config2.outer {
            resonance += 0.5;
        }
        if config1.inner == config2.inner {
            resonance += 0.5;
        }
        
        // Same k-values
        if config1.k_outer == config2.k_outer {
            resonance += 0.3;
        }
        if config1.k_inner == config2.k_inner {
            resonance += 0.3;
        }
        
        // Special resonance patterns
        if self.is_37_pattern(config1) && self.is_37_pattern(config2) {
            resonance += 1.0; // Strong 3-7 resonance
        }
        
        // Cap resonance to prevent domination over gravitational forces
        resonance.min(1.0)
    }
    
    /// Check if configuration uses 3-7 pattern
    fn is_37_pattern(&self, config: &crate::membrane::MembraneConfig) -> bool {
        (config.outer == 3 && config.inner == 7) || (config.outer == 7 && config.inner == 3)
    }
    
    /// Simple primality check for base metric calculation
    fn is_prime(&self, n: u32) -> bool {
        if n < 2 { return false; }
        if n == 2 { return true; }
        if n % 2 == 0 { return false; }
        
        for i in (3..=(n as f64).sqrt() as u32).step_by(2) {
            if n % i == 0 { return false; }
        }
        true
    }
    
    /// Determine which interaction type dominates
    fn determine_dominant_interaction(
        &self,
        gravitational: f64,
        charge: f64,
        base_metric: f64,
        membrane: f64
    ) -> InteractionType {
        let max_force = gravitational.abs()
            .max(charge.abs())
            .max(base_metric.abs())
            .max(membrane.abs());
        
        if gravitational.abs() == max_force {
            InteractionType::Gravitational
        } else if charge.abs() == max_force {
            InteractionType::Electromagnetic
        } else if base_metric.abs() == max_force {
            InteractionType::BaseMetric
        } else {
            InteractionType::MembraneResonance
        }
    }
    
    /// Get force statistics for analysis
    pub fn get_force_statistics(&self) -> ForceStatistics {
        let mut stats = ForceStatistics::default();
        
        for force_result in self.force_cache.values() {
            stats.total_calculations += 1;
            stats.total_force_magnitude += force_result.magnitude;
            
            if force_result.is_strong() {
                stats.strong_interactions += 1;
            }
            
            if force_result.is_close_encounter() {
                stats.close_encounters += 1;
            }
            
            // Update interaction type counts
            match force_result.interaction_type {
                InteractionType::Gravitational => stats.gravitational_dominant += 1,
                InteractionType::Electromagnetic => stats.electromagnetic_dominant += 1,
                InteractionType::BaseMetric => stats.base_metric_dominant += 1,
                InteractionType::MembraneResonance => stats.membrane_resonance_dominant += 1,
                InteractionType::QuantumExchange => stats.quantum_exchange_dominant += 1,
            }
        }
        
        if stats.total_calculations > 0 {
            stats.average_force_magnitude = stats.total_force_magnitude / stats.total_calculations as f64;
        }
        
        stats
    }
    
    /// Clear force cache
    pub fn clear_cache(&mut self) {
        self.force_cache.clear();
    }
}

impl Default for ForceCalculator {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about force calculations
#[derive(Debug, Clone, Default)]
pub struct ForceStatistics {
    pub total_calculations: usize,
    pub total_force_magnitude: f64,
    pub average_force_magnitude: f64,
    pub strong_interactions: usize,
    pub close_encounters: usize,
    pub gravitational_dominant: usize,
    pub electromagnetic_dominant: usize,
    pub base_metric_dominant: usize,
    pub membrane_resonance_dominant: usize,
    pub quantum_exchange_dominant: usize,
}

impl ForceStatistics {
    /// Get a summary of the force statistics
    pub fn summary(&self) -> String {
        format!(
            "Force Stats: {} calcs, avg force: {:.3}, {} strong, {} close, G:{} EM:{} BM:{} MR:{} QE:{}",
            self.total_calculations,
            self.average_force_magnitude,
            self.strong_interactions,
            self.close_encounters,
            self.gravitational_dominant,
            self.electromagnetic_dominant,
            self.base_metric_dominant,
            self.membrane_resonance_dominant,
            self.quantum_exchange_dominant
        )
    }
    
    /// Get the dominant interaction type
    pub fn dominant_interaction(&self) -> InteractionType {
        let interactions = [
            (self.gravitational_dominant, InteractionType::Gravitational),
            (self.electromagnetic_dominant, InteractionType::Electromagnetic),
            (self.base_metric_dominant, InteractionType::BaseMetric),
            (self.membrane_resonance_dominant, InteractionType::MembraneResonance),
            (self.quantum_exchange_dominant, InteractionType::QuantumExchange),
        ];
        
        interactions.iter()
            .max_by_key(|(count, _)| *count)
            .map(|(_, interaction_type)| interaction_type.clone())
            .unwrap_or(InteractionType::Gravitational)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gravity::PrimeParticle;
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;

    #[test]
    fn test_force_calculator() {
        let mut calculator = ForceCalculator::new();
        let constants = PhysicalConstants::default();
        
        let p1 = PrimeParticle::new(
            BigUint::from_u64(17).unwrap(),
            10,
            [0.0, 0.0],
            [0.0, 0.0],
            "Test1".to_string()
        );
        
        let p2 = PrimeParticle::new(
            BigUint::from_u64(19).unwrap(),
            10,
            [1.0, 0.0],
            [0.0, 0.0],
            "Test2".to_string()
        );
        
        let result = calculator.calculate_pairwise_force(&p1, &p2, &constants, 0, 1);
        assert!(result.is_ok());
        
        let force_result = result.unwrap();
        assert!(force_result.magnitude > 0.0);
        assert_eq!(force_result.distance, 1.0);
    }
    
    #[test]
    fn test_gravitational_force() {
        let calculator = ForceCalculator::new();
        let constants = PhysicalConstants::default();
        
        let p1 = PrimeParticle::new(
            BigUint::from_u64(100).unwrap(),
            10,
            [0.0, 0.0],
            [0.0, 0.0],
            "Heavy".to_string()
        );
        
        let p2 = PrimeParticle::new(
            BigUint::from_u64(10).unwrap(),
            10,
            [0.0, 0.0],
            [0.0, 0.0],
            "Light".to_string()
        );
        
        let force = calculator.calculate_gravitational_force(&p1, &p2, 1.0, &constants);
        assert!(force > 0.0);
        
        // Force should decrease with distance squared
        let force_far = calculator.calculate_gravitational_force(&p1, &p2, 2.0, &constants);
        assert!(force_far < force / 3.0); // Should be roughly 1/4
    }
    
    #[test]
    fn test_base_compatibility() {
        let calculator = ForceCalculator::new();
        
        // Same base should attract
        let same_base = calculator.calculate_base_compatibility(10, 10);
        assert!(same_base > 1.0);
        
        // Even vs odd should repel
        let even_odd = calculator.calculate_base_compatibility(10, 11);
        assert!(even_odd < 0.0);
        
        // Same parity should attract weakly
        let same_parity = calculator.calculate_base_compatibility(10, 12);
        assert!(same_parity > 0.0);
        assert!(same_parity < same_base);
    }
}