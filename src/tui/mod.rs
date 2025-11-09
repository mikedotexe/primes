//! TUI module - Shared state and logic for terminal user interfaces
//!
//! This module provides the core data structures and logic that can be
//! used by both native terminal UIs and WASM web interfaces.

use crate::{
    is_prime,
    membrane::{MembraneBuilder, MembraneConfig},
};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

/// The state of the Lagrange TUI application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LagrangeUIState {
    pub particle1: Option<PrimeInfo>,
    pub particle2: Option<PrimeInfo>,
    pub config: MembraneConfig,
    pub lagrange_points: Vec<LagrangePointInfo>,
    pub cluster_analysis: Option<ClusterInfo>,
    pub selected_prime: usize,
    pub show_help: bool,
    pub is_generating: bool,
    pub status_message: String,
    pub total_generations: usize,
    pub primes_found: usize,
    pub last_generation_time: Option<u64>, // milliseconds
    pub current_prime_distance: Option<String>,
}

/// Simplified prime information for UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimeInfo {
    pub value: String,
    pub structure: String,
    pub visual: String,
    pub mass: f64,
    pub base: u32,
}

/// Simplified Lagrange point information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LagrangePointInfo {
    pub point_type: String,
    pub value: String,
    pub position: [f64; 2],
    pub field_strength: f64,
    pub stability: f64,
    pub is_prime: bool,
    pub tested: bool,
}

/// Simplified cluster analysis info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterInfo {
    pub total_captured: usize,
    pub analysis_successful: bool,
}

impl Default for LagrangeUIState {
    fn default() -> Self {
        Self {
            particle1: None,
            particle2: None,
            config: MembraneConfig::new(6, 1, 5, 0, 0),
            lagrange_points: Vec::new(),
            cluster_analysis: None,
            selected_prime: 0,
            show_help: false,
            is_generating: false,
            status_message: "Press 'g' to generate prime pair".to_string(),
            total_generations: 0,
            primes_found: 0,
            last_generation_time: None,
            current_prime_distance: None,
        }
    }
}

impl LagrangeUIState {
    /// Generate a new prime pair
    pub fn generate_prime_pair(&mut self) {
        self.particle1 = None;
        self.particle2 = None;
        self.lagrange_points.clear();

        let start = std::time::Instant::now();

        // Try different configurations
        let configs = vec![
            MembraneConfig::new(10, 3, 3, 1, 1),
            MembraneConfig::new(10, 3, 7, 1, 1),
            MembraneConfig::new(10, 3, 3, 1, 0),
        ];

        for config in configs {
            self.config = config.clone();
            let mut found_primes = Vec::new();

            // Try different seeds
            for seed in 1u8..=10 {
                if let Ok(particle) = MembraneBuilder::new(config.clone()).with_seed(seed).build() {
                    if is_prime(&particle.value) {
                        found_primes.push((particle, seed));
                        if found_primes.len() >= 2 {
                            break;
                        }
                    }
                }
            }

            if found_primes.len() >= 2 {
                let (p1, seed1) = &found_primes[0];
                let (p2, seed2) = &found_primes[1];

                self.particle1 = Some(PrimeInfo {
                    value: p1.value.to_string(),
                    structure: format_structure(&p1.value, seed1, &config),
                    visual: format_visual(&p1.value, &config),
                    mass: p1.mass,
                    base: p1.base,
                });

                self.particle2 = Some(PrimeInfo {
                    value: p2.value.to_string(),
                    structure: format_structure(&p2.value, seed2, &config),
                    visual: format_visual(&p2.value, &config),
                    mass: p2.mass,
                    base: p2.base,
                });

                // Calculate distance
                let distance = if p2.value > p1.value {
                    &p2.value - &p1.value
                } else {
                    &p1.value - &p2.value
                };
                self.current_prime_distance = Some(distance.to_string());

                self.calculate_lagrange_points();
                break;
            }
        }

        self.last_generation_time = Some(start.elapsed().as_millis() as u64);
        self.total_generations += 1;
        if self.particle1.is_some() && self.particle2.is_some() {
            self.primes_found += 2;
            self.status_message = format!(
                "Generated! Distance: {} | Press 't' to test L-points",
                self.current_prime_distance.as_ref().unwrap()
            );
        } else {
            self.status_message = "Failed to generate primes - try different config".to_string();
        }
    }

    /// Calculate Lagrange points between the two primes
    pub fn calculate_lagrange_points(&mut self) {
        self.lagrange_points.clear();

        if let (Some(p1), Some(p2)) = (&self.particle1, &self.particle2) {
            let val1 = BigUint::parse_bytes(p1.value.as_bytes(), 10).unwrap();
            let val2 = BigUint::parse_bytes(p2.value.as_bytes(), 10).unwrap();

            let midpoint = (&val1 + &val2) / 2u32;

            self.lagrange_points.push(LagrangePointInfo {
                point_type: "L1".to_string(),
                value: midpoint.to_string(),
                position: [0.0, 0.0],
                field_strength: 1.0,
                stability: 0.5,
                is_prime: is_prime(&midpoint),
                tested: false,
            });
        }
    }

    /// Test Lagrange points for primality
    pub fn test_lagrange_points(&mut self) {
        for point in &mut self.lagrange_points {
            if let Some(val) = BigUint::parse_bytes(point.value.as_bytes(), 10) {
                point.is_prime = is_prime(&val);
                point.tested = true;
            }
        }
        self.status_message = "Lagrange points tested".to_string();
    }

    /// Cycle through different configurations
    pub fn cycle_configuration(&mut self) {
        let configs = [
            (MembraneConfig::new(10, 3, 3, 1, 1), "(3,3) k=(1,1) base 10"),
            (
                MembraneConfig::new(10, 3, 7, 1, 1),
                "(3,7) k=(1,1) base 10 - Exclusive!",
            ),
            (MembraneConfig::new(10, 7, 7, 1, 1), "(7,7) k=(1,1) base 10"),
            (
                MembraneConfig::new(10, 3, 3, 0, 1),
                "(3,3) k=(0,1) base 10 - Breathing",
            ),
            (
                MembraneConfig::new(6, 1, 5, 0, 0),
                "(1,5) k=(0,0) base 6 - Champion",
            ),
        ];

        let current_idx = configs
            .iter()
            .position(|(c, _)| {
                c.base == self.config.base
                    && c.outer == self.config.outer
                    && c.inner == self.config.inner
                    && c.k_outer == self.config.k_outer
                    && c.k_inner == self.config.k_inner
            })
            .unwrap_or(0);

        let next_idx = (current_idx + 1) % configs.len();
        let (config, name) = &configs[next_idx];

        self.config = config.clone();
        self.status_message = format!("Switched to config: {}", name);
        self.particle1 = None;
        self.particle2 = None;
        self.lagrange_points.clear();
    }
}

fn format_structure(_value: &BigUint, seed: &u8, config: &MembraneConfig) -> String {
    format!(
        "{}-{}-[{}]-{}-{}",
        config.outer,
        "0".repeat(config.k_outer as usize),
        seed,
        "0".repeat(config.k_outer as usize),
        config.outer
    )
}

fn format_visual(value: &BigUint, _config: &MembraneConfig) -> String {
    let val_str = value.to_string();
    let chars: Vec<char> = val_str.chars().collect();

    let visual_chars: Vec<String> = chars
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            if c == '0' && i > 0 && i < chars.len() - 1 {
                '◯'.to_string()
            } else {
                c.to_string()
            }
        })
        .collect();

    visual_chars.join("─")
}

/// Render the state to text (for screenshots or debugging)
pub fn render_to_text(state: &LagrangeUIState) -> String {
    let mut output = String::new();

    // Header
    output.push_str(&"─".repeat(148));
    output.push('\n');
    output.push_str(&format!(
        "{:^148}",
        "⚛️  Lagrange Point Explorer - Prime Atomic Interactions"
    ));
    output.push('\n');
    output.push_str(&"─".repeat(148));
    output.push('\n');

    // Status
    let config_str = format!(
        "({},{}) k=({},{}) b{}",
        state.config.outer,
        state.config.inner,
        state.config.k_outer,
        state.config.k_inner,
        state.config.base
    );
    output.push_str(&format!("[{}] {}\n", config_str, state.status_message));

    output
}
