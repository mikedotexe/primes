//! Quantum membrane construction
//! 
//! Orbital-like k-patterns inspired by atomic electron configurations

use crate::PhysicsResult;
use super::{MembraneConfig, OrbitalType, construct_symmetric_membrane};

/// Construct a quantum orbital membrane
pub fn construct_quantum_membrane(
    config: &MembraneConfig,
    middle: &str,
    orbital_type: &OrbitalType,
    quantum_numbers: &[u32],
) -> PhysicsResult<String> {
    let (outer, inner, k_outer, k_inner) = get_orbital_parameters(orbital_type, quantum_numbers);
    
    // Use base configuration or orbital-specific values
    let final_outer = if outer == 0 { config.outer } else { outer };
    let final_inner = if inner == 0 { config.inner } else { inner };
    
    construct_symmetric_membrane(final_outer, final_inner, middle, k_outer, k_inner)
}

/// Get k-parameters for different orbital types
fn get_orbital_parameters(orbital_type: &OrbitalType, _quantum_numbers: &[u32]) -> (u32, u32, u32, u32) {
    match orbital_type {
        OrbitalType::S => (0, 0, 0, 1), // s-orbital: spherical, minimal padding
        OrbitalType::P => (0, 0, 1, 1), // p-orbital: dumbbell shape
        OrbitalType::D => (0, 0, 2, 2), // d-orbital: four-lobed
        OrbitalType::F => (0, 0, 3, 3), // f-orbital: complex shape
        OrbitalType::G => (0, 0, 4, 4), // g-orbital: very high energy
        OrbitalType::Hybrid(k_values) => {
            let k1 = k_values.first().unwrap_or(&2);
            let k2 = k_values.get(1).unwrap_or(&2);
            (0, 0, *k1, *k2)
        },
    }
}

/// Quantum orbital analysis
#[derive(Debug, Clone)]
pub struct QuantumAnalysis {
    pub orbital_type: OrbitalType,
    pub quantum_numbers: Vec<u32>,
    pub energy_level: u32,
    pub electron_capacity: u32,
    pub orbital_shape: OrbitalShape,
    pub probability_density: f64,
}

/// Orbital shape characteristics
#[derive(Debug, Clone)]
pub enum OrbitalShape {
    Spherical,      // s-orbital
    Dumbbell,       // p-orbital  
    Cloverleaf,     // d-orbital
    Complex,        // f-orbital and higher
    Hybrid,         // custom combinations
}

impl QuantumAnalysis {
    /// Create quantum analysis for an orbital type
    pub fn new(orbital_type: OrbitalType, quantum_numbers: Vec<u32>) -> Self {
        let (energy_level, electron_capacity, orbital_shape) = match orbital_type {
            OrbitalType::S => (1, 2, OrbitalShape::Spherical),
            OrbitalType::P => (2, 6, OrbitalShape::Dumbbell),
            OrbitalType::D => (3, 10, OrbitalShape::Cloverleaf),
            OrbitalType::F => (4, 14, OrbitalShape::Complex),
            OrbitalType::G => (5, 18, OrbitalShape::Complex),
            OrbitalType::Hybrid(_) => (0, 0, OrbitalShape::Hybrid),
        };
        
        let probability_density = calculate_probability_density(&orbital_type, &quantum_numbers);
        
        Self {
            orbital_type,
            quantum_numbers,
            energy_level,
            electron_capacity,
            orbital_shape,
            probability_density,
        }
    }
    
    /// Check if this orbital can host primes (has electron density)
    pub fn can_host_primes(&self) -> bool {
        self.probability_density > 0.1
    }
    
    /// Get orbital description
    pub fn description(&self) -> String {
        format!(
            "{:?} orbital [E: {}, capacity: {}, shape: {:?}, density: {:.3}]",
            self.orbital_type,
            self.energy_level,
            self.electron_capacity,
            self.orbital_shape,
            self.probability_density
        )
    }
}

/// Calculate probability density for prime generation in this orbital
fn calculate_probability_density(orbital_type: &OrbitalType, _quantum_numbers: &[u32]) -> f64 {
    match orbital_type {
        OrbitalType::S => 0.8,  // High density, spherical
        OrbitalType::P => 0.6,  // Good density, directional
        OrbitalType::D => 0.4,  // Moderate density, complex
        OrbitalType::F => 0.2,  // Lower density, very complex
        OrbitalType::G => 0.1,  // Very low density, exotic
        OrbitalType::Hybrid(k_values) => {
            // Density based on k-value complexity
            let avg_k = k_values.iter().sum::<u32>() as f64 / k_values.len() as f64;
            (1.0 / (1.0 + avg_k * 0.2)).max(0.1)
        },
    }
}

/// Generate quantum orbital sequence for membrane construction
pub fn generate_orbital_sequence() -> Vec<OrbitalType> {
    vec![
        OrbitalType::S,
        OrbitalType::P,
        OrbitalType::S, // 2s
        OrbitalType::P, // 2p
        OrbitalType::S, // 3s
        OrbitalType::P, // 3p
        OrbitalType::D, // 3d
        OrbitalType::S, // 4s
        OrbitalType::P, // 4p
        OrbitalType::D, // 4d
        OrbitalType::F, // 4f
        OrbitalType::S, // 5s
        OrbitalType::P, // 5p
        OrbitalType::D, // 5d
        OrbitalType::F, // 5f
        OrbitalType::G, // 5g
    ]
}

/// Quantum membrane builder with electron shell filling
pub struct QuantumMembraneBuilder {
    base: u32,
    current_shell: u32,
    current_orbital: usize,
    orbital_sequence: Vec<OrbitalType>,
}

impl QuantumMembraneBuilder {
    /// Create new quantum builder
    pub fn new(base: u32) -> Self {
        Self {
            base,
            current_shell: 1,
            current_orbital: 0,
            orbital_sequence: generate_orbital_sequence(),
        }
    }
    
    /// Get next orbital configuration
    pub fn next_orbital(&mut self) -> Option<QuantumAnalysis> {
        if self.current_orbital >= self.orbital_sequence.len() {
            return None;
        }
        
        let orbital_type = self.orbital_sequence[self.current_orbital].clone();
        let quantum_numbers = vec![self.current_shell, self.current_orbital as u32];
        
        self.current_orbital += 1;
        
        // Move to next shell based on orbital filling rules
        match orbital_type {
            OrbitalType::S if self.current_orbital % 2 == 0 => self.current_shell += 1,
            OrbitalType::P if self.current_orbital % 4 == 0 => self.current_shell += 1,
            OrbitalType::D if self.current_orbital % 6 == 0 => self.current_shell += 1,
            _ => {},
        }
        
        Some(QuantumAnalysis::new(orbital_type, quantum_numbers))
    }
    
    /// Generate membrane configuration for current orbital
    pub fn build_quantum_config(&self, orbital: &QuantumAnalysis) -> MembraneConfig {
        let (_, _, k_outer, k_inner) = get_orbital_parameters(&orbital.orbital_type, &orbital.quantum_numbers);
        
        MembraneConfig {
            base: self.base,
            outer: 3, // Use good defaults
            inner: 7,
            k_outer,
            k_inner,
            middle_length: 1,
            construction_type: crate::membrane::ConstructionType::Quantum {
                orbital_type: orbital.orbital_type.clone(),
                quantum_numbers: orbital.quantum_numbers.clone(),
            },
            expected_density: orbital.probability_density * 0.05, // Scale to reasonable range
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbital_parameters() {
        let (_, _, k_outer, k_inner) = get_orbital_parameters(&OrbitalType::D, &[]);
        assert_eq!((k_outer, k_inner), (2, 2)); // d-orbital should have k=2,2
    }
    
    #[test]
    fn test_quantum_analysis() {
        let analysis = QuantumAnalysis::new(OrbitalType::S, vec![1, 0]);
        assert_eq!(analysis.energy_level, 1);
        assert_eq!(analysis.electron_capacity, 2);
        assert!(analysis.can_host_primes());
    }
    
    #[test]
    fn test_quantum_builder() {
        let mut builder = QuantumMembraneBuilder::new(10);
        let first_orbital = builder.next_orbital();
        assert!(first_orbital.is_some());
        
        let orbital = first_orbital.unwrap();
        let config = builder.build_quantum_config(&orbital);
        assert_eq!(config.base, 10);
    }
}