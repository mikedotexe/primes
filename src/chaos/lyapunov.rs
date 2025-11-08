//! Lyapunov Exponent Calculator for Prime Dynamics
//! ================================================
//! Measures chaos in prime particle systems by tracking trajectory divergence

use std::collections::HashMap;

/// Tracks original and shadow trajectories to measure chaos
pub struct LyapunovCalculator {
    /// Original trajectory
    pub positions: Vec<[f64; 2]>,
    pub velocities: Vec<[f64; 2]>,
    
    /// Shadow trajectory (starts ε away)
    pub shadow_positions: Vec<[f64; 2]>,
    pub shadow_velocities: Vec<[f64; 2]>,
    
    /// Separation tracking
    pub separations: Vec<f64>,
    pub epsilon: f64, // Initial separation (e.g., 1e-8)
    
    /// Renormalization events
    pub renorm_times: Vec<f64>,
    pub renorm_factors: Vec<f64>,
}

impl LyapunovCalculator {
    pub fn new(epsilon: f64) -> Self {
        Self {
            positions: Vec::new(),
            velocities: Vec::new(),
            shadow_positions: Vec::new(),
            shadow_velocities: Vec::new(),
            separations: Vec::new(),
            epsilon,
            renorm_times: Vec::new(),
            renorm_factors: Vec::new(),
        }
    }
    
    /// Initialize shadow trajectory slightly offset from original
    pub fn initialize_shadow(&mut self, pos: [f64; 2], vel: [f64; 2]) {
        // Add small perturbation in x direction
        let shadow_pos = [pos[0] + self.epsilon, pos[1]];
        
        self.positions.push(pos);
        self.velocities.push(vel);
        self.shadow_positions.push(shadow_pos);
        self.shadow_velocities.push(vel);
        
        self.separations.push(self.epsilon);
    }
    
    /// Calculate the Lyapunov exponent from tracked data
    pub fn calculate_lyapunov(&self, total_time: f64) -> f64 {
        if self.renorm_factors.is_empty() {
            // No renormalizations - check if still diverging
            let final_sep = self.separations.last().unwrap_or(&self.epsilon);
            let growth = final_sep / self.epsilon;
            return growth.ln() / total_time;
        }
        
        // Sum logarithms of all growth factors
        let sum_log_growth: f64 = self.renorm_factors.iter()
            .map(|&factor| factor.ln())
            .sum();
        
        sum_log_growth / total_time
    }
    
    /// Get separation history for visualization
    pub fn get_separation_history(&self) -> Vec<(f64, f64)> {
        self.separations.iter()
            .enumerate()
            .map(|(i, &sep)| (i as f64, sep))
            .collect()
    }
}

/// Maps membrane configurations to their chaos levels
pub struct ChaosMap {
    /// Configuration -> Lyapunov exponent
    pub chaos_levels: HashMap<(u32, u32, u32, u32), f64>,
    
    /// Number of time steps to simulate
    pub simulation_steps: usize,
    
    /// Time step size
    pub dt: f64,
}

impl ChaosMap {
    pub fn new(simulation_steps: usize, dt: f64) -> Self {
        Self {
            chaos_levels: HashMap::new(),
            simulation_steps,
            dt,
        }
    }
    
    /// Measure chaos for a specific membrane configuration
    pub fn measure_configuration_chaos(
        &mut self,
        outer: u32,
        inner: u32, 
        k_outer: u32,
        k_inner: u32,
    ) -> f64 {
        // Simplified chaos measurement based on configuration properties
        // In a full implementation, this would run actual N-body simulation
        
        // Heuristic: Heavy configurations are more chaotic
        let mass_factor = (outer + inner) as f64 / 10.0;
        let asymmetry = ((k_outer as f64 - k_inner as f64).abs() + 1.0).ln();
        
        // Configuration-specific patterns
        let config_bonus = match (outer, inner) {
            (9, 9) => 0.3,  // Heavy symmetric - very chaotic
            (1, 1) => -0.2, // Light symmetric - stable
            (3, 7) => 0.1,  // Classic - moderate chaos
            (7, 3) => 0.1,  // Mirror classic
            _ => 0.0,
        };
        
        // Simple formula (placeholder for actual physics)
        let lyapunov = 0.05 * mass_factor * asymmetry + config_bonus + 0.02;
        
        // Store result
        self.chaos_levels.insert((outer, inner, k_outer, k_inner), lyapunov);
        
        lyapunov
    }
    
    /// Generate complete chaos landscape
    pub fn generate_chaos_landscape(&mut self) {
        println!("🌌 GENERATING CHAOS LANDSCAPE...");
        println!("{}", "=".repeat(60));
        
        for outer in [1, 3, 7, 9] {
            for inner in [1, 3, 5, 7, 9] {
                for k_outer in 0..3 {
                    for k_inner in 0..3 {
                        let lambda = self.measure_configuration_chaos(outer, inner, k_outer, k_inner);
                        
                        // Visualize chaos level
                        let bar_length = ((lambda.abs() * 50.0).min(20.0)) as usize;
                        let bar = "█".repeat(bar_length) + &"░".repeat(20 - bar_length);
                        
                        println!("({},{}) k=({},{}): λ={:6.3} [{}] {}", 
                            outer, inner, k_outer, k_inner, lambda, bar,
                            if lambda > 0.3 { "CHAOS STORM!" } 
                            else if lambda > 0.1 { "Chaotic" }
                            else if lambda > 0.01 { "Edge of chaos" }
                            else { "Stable island" }
                        );
                    }
                }
            }
        }
    }
    
    /// Find islands of stability
    pub fn find_stable_islands(&self) -> Vec<((u32, u32, u32, u32), f64)> {
        let mut islands: Vec<_> = self.chaos_levels
            .iter()
            .filter(|(_, &lambda)| lambda < 0.01 && lambda > -0.01)
            .map(|(config, lambda)| (*config, *lambda))
            .collect();
            
        islands.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        islands
    }
    
    /// Find chaos storms
    pub fn find_chaos_storms(&self) -> Vec<((u32, u32, u32, u32), f64)> {
        let mut storms: Vec<_> = self.chaos_levels
            .iter()
            .filter(|(_, &lambda)| lambda > 0.3)
            .map(|(config, lambda)| (*config, *lambda))
            .collect();
            
        storms.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        storms
    }
    
    /// Validate reproducibility
    pub fn validate_reproducibility(&mut self, config: (u32, u32, u32, u32), runs: usize) -> (f64, f64) {
        let mut results = Vec::new();
        
        for _ in 0..runs {
            let lambda = self.measure_configuration_chaos(config.0, config.1, config.2, config.3);
            results.push(lambda);
        }
        
        // Calculate mean and standard deviation
        let mean = results.iter().sum::<f64>() / results.len() as f64;
        let variance = results.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / results.len() as f64;
        let std_dev = variance.sqrt();
        
        (mean, std_dev)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lyapunov_calculation() {
        let mut calc = LyapunovCalculator::new(1e-8);
        
        // Add some mock data
        calc.renorm_factors = vec![2.0, 2.5, 3.0];
        calc.renorm_times = vec![1.0, 2.0, 3.0];
        
        let lyapunov = calc.calculate_lyapunov(3.0);
        
        // Should be approximately ln(2*2.5*3)/3 = ln(15)/3 ≈ 0.9
        assert!((lyapunov - 0.9).abs() < 0.1);
    }
}