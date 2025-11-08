//! Resonance Profiles: Multi-Configuration View of Membrane Primes
//! 
//! This module implements the discovery that each base has multiple "resonant configurations"
//! that can generate primes - like an atom having multiple stable electron orbitals.
//! 
//! # Core Concepts
//! 
//! - **Resonance Profile**: The complete set of configurations that generate primes for a base
//! - **Seed Map**: Which seeds work with which configurations (like quantum selection rules)
//! - **Exclusive Configurations**: Special cases where only ONE seed can generate primes
//! - **Configuration Space**: The full parameter landscape of (outer, inner, k_outer, k_inner)
//! 
//! # Physics Metaphor
//! 
//! Just as atoms have multiple stable electron orbitals (s, p, d, f), each base has multiple
//! stable membrane configurations. Some configurations are "exclusive" - only allowing one
//! specific seed to resonate (like a forbidden transition that only works for specific states).

use std::collections::{HashMap, HashSet, BTreeMap};
use std::fmt;

use crate::membrane::symmetric::construct_symmetric_membrane;
use crate::is_prime;
use num_bigint::BigUint;
use num_traits::Num;

/// Simple membrane configuration for resonance analysis
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MembraneConfig {
    pub outer_digit: u8,
    pub inner_digit: u8,
    pub k_outer: usize,
    pub k_inner: usize,
}

/// Represents a specific membrane configuration and its success metrics
#[derive(Debug, Clone, PartialEq)]
pub struct ConfigurationProfile {
    /// The membrane configuration (outer, inner, k_outer, k_inner)
    pub config: MembraneConfig,
    
    /// Which seeds (0-9) successfully generate primes with this config
    pub successful_seeds: HashSet<u8>,
    
    /// Total number of primes found with this configuration
    pub prime_count: usize,
    
    /// Success rate (primes / attempts)
    pub success_rate: f64,
    
    /// Is this an "exclusive" configuration (only one seed works)?
    pub is_exclusive: bool,
    
    /// Average prime length for this configuration
    pub avg_prime_length: f64,
}

impl ConfigurationProfile {
    /// Create a new configuration profile
    pub fn new(config: MembraneConfig) -> Self {
        Self {
            config,
            successful_seeds: HashSet::new(),
            prime_count: 0,
            success_rate: 0.0,
            is_exclusive: false,
            avg_prime_length: 0.0,
        }
    }
    
    /// Update the profile with a successful prime discovery
    pub fn record_success(&mut self, seed: u8, prime_length: usize) {
        self.successful_seeds.insert(seed);
        self.prime_count += 1;
        
        // Update average length incrementally
        let n = self.prime_count as f64;
        self.avg_prime_length = 
            (self.avg_prime_length * (n - 1.0) + prime_length as f64) / n;
    }
    
    /// Finalize the profile after all tests
    pub fn finalize(&mut self, total_attempts: usize) {
        self.success_rate = self.prime_count as f64 / total_attempts as f64;
        self.is_exclusive = self.successful_seeds.len() == 1 && self.prime_count > 0;
    }
    
    /// Get the exclusive seed if this is an exclusive configuration
    pub fn exclusive_seed(&self) -> Option<u8> {
        if self.is_exclusive {
            self.successful_seeds.iter().next().copied()
        } else {
            None
        }
    }
}

/// Complete resonance profile for a specific base
#[derive(Debug, Clone)]
pub struct BaseResonanceProfile {
    /// The base this profile is for
    pub base: u32,
    
    /// All configurations that produced at least one prime
    pub configurations: Vec<ConfigurationProfile>,
    
    /// Seed-centric view: which configurations work for each seed
    pub seed_map: SeedResonanceMap,
    
    /// Statistics about the profile
    pub stats: ProfileStatistics,
}

impl BaseResonanceProfile {
    /// Create a new base resonance profile
    pub fn new(base: u32) -> Self {
        Self {
            base,
            configurations: Vec::new(),
            seed_map: SeedResonanceMap::new(),
            stats: ProfileStatistics::default(),
        }
    }
    
    /// Discover all resonant configurations for this base
    pub fn discover_resonances(
        &mut self,
        outer_range: &[u8],
        inner_range: &[u8],
        k_range: &[u8],
        max_middle_length: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut config_map: HashMap<MembraneConfig, ConfigurationProfile> = HashMap::new();
        
        // Test all configuration combinations
        for &outer in outer_range {
            for &inner in inner_range {
                for &k_outer in k_range {
                    for &k_inner in k_range {
                        let config = MembraneConfig {
                            outer_digit: outer,
                            inner_digit: inner,
                            k_outer: k_outer as usize,
                            k_inner: k_inner as usize,
                        };
                        
                        let mut profile = ConfigurationProfile::new(config.clone());
                        let mut attempts = 0;
                        
                        // Test each seed (0-9) with this configuration
                        for seed in 0..=9 {
                            for length in 1..=max_middle_length {
                                let middle = seed.to_string().repeat(length);
                                
                                // Construct the membrane
                                match construct_symmetric_membrane(
                                    config.outer_digit as u32,
                                    config.inner_digit as u32,
                                    &middle,
                                    config.k_outer as u32,
                                    config.k_inner as u32,
                                ) {
                                    Ok(membrane_str) => {
                                        attempts += 1;
                                        
                                        // Convert to appropriate base and check primality
                                        if let Ok(num) = BigUint::from_str_radix(&membrane_str, self.base) {
                                            if is_prime(&num) {
                                                let prime_length = membrane_str.len();
                                                profile.record_success(seed, prime_length);
                                            }
                                        }
                                    }
                                    Err(_) => continue,
                                }
                            }
                        }
                        
                        // Only keep configurations that found at least one prime
                        if profile.prime_count > 0 {
                            profile.finalize(attempts);
                            config_map.insert(config, profile);
                        }
                    }
                }
            }
        }
        
        // Convert to sorted vector and build seed map
        self.configurations = config_map.into_values()
            .collect();
        
        // Sort by success rate (highest first)
        self.configurations.sort_by(|a, b| 
            b.success_rate.partial_cmp(&a.success_rate).unwrap()
        );
        
        // Build seed resonance map
        self.build_seed_map();
        
        // Calculate statistics
        self.calculate_statistics();
        
        Ok(())
    }
    
    /// Build the seed-centric view of configurations
    fn build_seed_map(&mut self) {
        self.seed_map.clear();
        
        for config in &self.configurations {
            for &seed in &config.successful_seeds {
                self.seed_map.add_configuration(seed, &config.config, config.is_exclusive);
            }
        }
    }
    
    /// Calculate profile statistics
    fn calculate_statistics(&mut self) {
        self.stats = ProfileStatistics {
            total_configurations: self.configurations.len(),
            exclusive_configurations: self.configurations.iter()
                .filter(|c| c.is_exclusive)
                .count(),
            max_success_rate: self.configurations.first()
                .map(|c| c.success_rate)
                .unwrap_or(0.0),
            avg_success_rate: if self.configurations.is_empty() { 
                0.0 
            } else {
                self.configurations.iter()
                    .map(|c| c.success_rate)
                    .sum::<f64>() / self.configurations.len() as f64
            },
            seeds_with_configs: self.seed_map.active_seeds().len(),
            dead_seeds: (0..=9).filter(|&s| !self.seed_map.has_seed(s)).collect(),
        };
    }
    
    /// Get all exclusive configurations (only one seed works)
    pub fn exclusive_configurations(&self) -> Vec<&ConfigurationProfile> {
        self.configurations.iter()
            .filter(|c| c.is_exclusive)
            .collect()
    }
    
    /// Get the best configuration for a specific seed
    pub fn best_config_for_seed(&self, seed: u8) -> Option<&ConfigurationProfile> {
        self.configurations.iter()
            .filter(|c| c.successful_seeds.contains(&seed))
            .max_by(|a, b| a.success_rate.partial_cmp(&b.success_rate).unwrap())
    }
    
    /// Find configurations that work for multiple seeds (non-exclusive)
    pub fn versatile_configurations(&self) -> Vec<&ConfigurationProfile> {
        self.configurations.iter()
            .filter(|c| c.successful_seeds.len() > 1)
            .collect()
    }
}

/// Maps seeds to their resonant configurations
#[derive(Debug, Clone)]
pub struct SeedResonanceMap {
    /// For each seed, list of configurations that work
    map: BTreeMap<u8, Vec<SeedConfigEntry>>,
}

#[derive(Debug, Clone)]
pub struct SeedConfigEntry {
    pub config: MembraneConfig,
    pub is_exclusive: bool,
}

impl Default for SeedResonanceMap {
    fn default() -> Self {
        Self::new()
    }
}

impl SeedResonanceMap {
    /// Create a new empty seed resonance map
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
    
    /// Clear all entries
    pub fn clear(&mut self) {
        self.map.clear();
    }
    
    /// Add a configuration for a seed
    pub fn add_configuration(&mut self, seed: u8, config: &MembraneConfig, is_exclusive: bool) {
        self.map.entry(seed)
            .or_default()
            .push(SeedConfigEntry {
                config: config.clone(),
                is_exclusive,
            });
    }
    
    /// Check if a seed has any configurations
    pub fn has_seed(&self, seed: u8) -> bool {
        self.map.contains_key(&seed)
    }
    
    /// Get all configurations for a seed
    pub fn configurations_for_seed(&self, seed: u8) -> Option<&Vec<SeedConfigEntry>> {
        self.map.get(&seed)
    }
    
    /// Get all active seeds (those with at least one configuration)
    pub fn active_seeds(&self) -> Vec<u8> {
        self.map.keys().copied().collect()
    }
    
    /// Get the "orbital diagram" - how many configurations each seed has
    pub fn orbital_diagram(&self) -> BTreeMap<u8, usize> {
        self.map.iter()
            .map(|(&seed, configs)| (seed, configs.len()))
            .collect()
    }
}

/// Statistics about a base resonance profile
#[derive(Debug, Clone, Default)]
pub struct ProfileStatistics {
    /// Total number of successful configurations
    pub total_configurations: usize,
    
    /// Number of exclusive configurations (only one seed works)
    pub exclusive_configurations: usize,
    
    /// Highest success rate among all configurations
    pub max_success_rate: f64,
    
    /// Average success rate across all configurations
    pub avg_success_rate: f64,
    
    /// Number of seeds that have at least one configuration
    pub seeds_with_configs: usize,
    
    /// Seeds that don't work with any configuration
    pub dead_seeds: Vec<u8>,
}

impl fmt::Display for BaseResonanceProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Resonance Profile for Base {} ===", self.base)?;
        writeln!(f, "Total Configurations: {}", self.stats.total_configurations)?;
        writeln!(f, "Exclusive Configurations: {}", self.stats.exclusive_configurations)?;
        writeln!(f, "Max Success Rate: {:.2}%", self.stats.max_success_rate * 100.0)?;
        writeln!(f, "Active Seeds: {}/10", self.stats.seeds_with_configs)?;
        
        if !self.stats.dead_seeds.is_empty() {
            writeln!(f, "Dead Seeds: {:?}", self.stats.dead_seeds)?;
        }
        
        writeln!(f, "\n--- Top 5 Configurations ---")?;
        for (i, config) in self.configurations.iter().take(5).enumerate() {
            writeln!(f, "{}. ({},{}) k=({},{}) - {:.2}% success", 
                i + 1,
                config.config.outer_digit,
                config.config.inner_digit,
                config.config.k_outer,
                config.config.k_inner,
                config.success_rate * 100.0
            )?;
            
            if config.is_exclusive {
                writeln!(f, "   EXCLUSIVE: Only seed {} works!", 
                    config.exclusive_seed().unwrap())?;
            } else {
                writeln!(f, "   Seeds: {:?}", config.successful_seeds)?;
            }
        }
        
        writeln!(f, "\n--- Seed Orbital Diagram ---")?;
        for (seed, count) in self.seed_map.orbital_diagram() {
            let bar = "█".repeat(count.min(20));
            writeln!(f, "Seed {seed}: {count} [{bar}]")?;
        }
        
        Ok(())
    }
}

/// Analyze resonance patterns across multiple bases
pub struct ResonanceAnalyzer {
    profiles: HashMap<u32, BaseResonanceProfile>,
}

impl Default for ResonanceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ResonanceAnalyzer {
    /// Create a new resonance analyzer
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
        }
    }
    
    /// Add a base profile to the analyzer
    pub fn add_profile(&mut self, profile: BaseResonanceProfile) {
        self.profiles.insert(profile.base, profile);
    }
    
    /// Find configurations that work across multiple bases
    pub fn universal_configurations(&self) -> Vec<MembraneConfig> {
        if self.profiles.is_empty() {
            return Vec::new();
        }
        
        // Get all configurations from first base
        let first_base = self.profiles.values().next().unwrap();
        let mut universal = Vec::new();
        
        for config_profile in &first_base.configurations {
            let config = &config_profile.config;
            
            // Check if this configuration exists in all other bases
            let is_universal = self.profiles.values()
                .all(|profile| 
                    profile.configurations.iter()
                        .any(|cp| cp.config == *config)
                );
            
            if is_universal {
                universal.push(config.clone());
            }
        }
        
        universal
    }
    
    /// Find seeds that are universally dead across all bases
    pub fn universal_dead_seeds(&self) -> Vec<u8> {
        (0..=9).filter(|&seed| 
            self.profiles.values()
                .all(|profile| profile.stats.dead_seeds.contains(&seed))
        ).collect()
    }
    
    /// Compare exclusive configurations across bases
    pub fn exclusive_comparison(&self) -> HashMap<u32, Vec<(MembraneConfig, u8)>> {
        self.profiles.iter()
            .map(|(&base, profile)| {
                let exclusives = profile.exclusive_configurations()
                    .into_iter()
                    .filter_map(|cp| 
                        cp.exclusive_seed()
                            .map(|seed| (cp.config.clone(), seed))
                    )
                    .collect();
                (base, exclusives)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_configuration_profile() {
        let config = MembraneConfig {
            outer_digit: 3,
            inner_digit: 7,
            k_outer: 2,
            k_inner: 2,
        };
        
        let mut profile = ConfigurationProfile::new(config);
        
        // Record some successes
        profile.record_success(5, 15);
        profile.record_success(5, 17);
        profile.record_success(5, 16);
        
        profile.finalize(10);
        
        assert_eq!(profile.prime_count, 3);
        assert_eq!(profile.success_rate, 0.3);
        assert!(profile.is_exclusive);
        assert_eq!(profile.exclusive_seed(), Some(5));
        assert_eq!(profile.avg_prime_length, 16.0);
    }
    
    #[test]
    fn test_seed_resonance_map() {
        let mut map = SeedResonanceMap::new();
        
        let config1 = MembraneConfig {
            outer_digit: 3,
            inner_digit: 3,
            k_outer: 1,
            k_inner: 1,
        };
        
        let config2 = MembraneConfig {
            outer_digit: 3,
            inner_digit: 7,
            k_outer: 2,
            k_inner: 2,
        };
        
        map.add_configuration(5, &config1, true);
        map.add_configuration(5, &config2, false);
        map.add_configuration(3, &config2, false);
        
        assert!(map.has_seed(5));
        assert!(map.has_seed(3));
        assert!(!map.has_seed(7));
        
        let orbital = map.orbital_diagram();
        assert_eq!(orbital[&5], 2);
        assert_eq!(orbital[&3], 1);
    }
}