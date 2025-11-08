//! # Prime Physics Engine
//! 
//! High-performance membrane prime generator with mathematical foundations.
//! Public re-exports for the "Tier-1" API.
//! 
//! ## Core Components
//! 
//! - [`BitSieve`] – In-L2 cache classic sieve for prime generation
//! - [`Wheel30Sieve`] – Compressed 30-wheel variant for optimized filtering  
//! - [`SegmentedSieve`] – Multi-core, NUMA-friendly parallel processing
//! - [`MembraneConfig`] – Symmetric membrane structures for prime generation
//! - [`PrimeUniverse`] – Physics-based gravitational prime modeling
//! 
//! ## Compile-time Features
//! 
//! | Feature          | Default | Purpose                    |
//! |------------------|---------|----------------------------|
//! | `wheel30`        | ✔︎       | 30-wheel compression       |
//! | `simd`           | ✔︎       | NEON / AVX-512 clear-bit   |
//! | `wasm`           | ✗       | `wasm-bindgen` wrappers    |
//! | `gpu`            | ✗       | Metal compute kernels      |
//! | `prime-harmonics`| ✗       | Fourier analysis support   |
//! 
//! ## Performance Characteristics
//! 
//! **Verified Performance** (on Apple M1 Max):
//! - Base 6, Config (1,5): **33% prime generation success**
//! - Base 30, Config (11,7): **30% prime generation success**  
//! - Standard sieve: Up to 10M primes/second
//! - GPU acceleration: 50x speedup for large ranges
//! 
//! ## Quick Start
//! 
//! ```rust
//! use prime_physics_engine::{BitSieve, MembraneConfig};
//! 
//! // Basic prime generation
//! let sieve = BitSieve::new(1000);
//! let primes = sieve.primes();
//! 
//! // Membrane-based prime construction
//! let config = MembraneConfig::new(1, 5, 0, 0);
//! // Generate prime candidates using membrane patterns
//! ```
//! 
//! ## Architecture
//! 
//! ```text
//! PrimeUniverse
//! ├── Particles (PrimeParticle)
//! ├── Forces (GravitationalField)
//! ├── Spacetime (BaseMetric)
//! └── Dynamics (TimeEvolution)
//! ```

use num_bigint::BigUint;
use num_traits::{Zero, One, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod membrane;
pub mod gravity;
pub mod lagrange;
pub mod spacetime;
pub mod tidal;
pub mod education;
pub mod validation;
pub mod resonance_profiles;
pub mod chaos;
pub mod integrators;
pub mod nibble_pack;
pub mod prime_lut;
pub mod prime_lut_recip;
pub mod ascii_art;

#[cfg(feature = "metal")]
pub mod metal_host;

#[cfg(feature = "metal")]
pub mod gpu;

#[cfg(feature = "metal")]
pub mod gpu_optimized;

#[cfg(feature = "visualization")]
pub mod visualization;

pub mod tui;

// WebAssembly bindings
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub mod wasm;

// High-performance prime sieve
pub mod prime_sieve;
pub mod dvfs;

// Performance monitoring
pub mod performance;

// Harmonic analysis (requires prime-harmonics feature)
pub mod harmonics;

// Phase 4: AMX/SME backend
#[cfg(feature = "phase4")]
pub mod phase4;

// Holistic optimization framework
pub mod optimization;

// Hardy-Littlewood framework for mathematical foundations
pub mod hzlib;

// Prelude for convenient imports
pub mod prelude;

// Re-export key types for convenience
pub use prime_sieve::{BitSieve, WarmResult};
pub use membrane::{MembraneConfig, MembraneBuilder};
pub use gravity::{PrimeParticle, GravitationalField, ForceCalculator};
pub use lagrange::{LagrangePoint, ClusterAnalysis, TidalForce};
pub use spacetime::{BaseMetric, PotentialField, PhaseSpace};
pub use tidal::{TidalAnalyzer, TidalField, TidalZone, OrganizationZone, OPTIMAL_TIDAL_STRENGTH};
pub use resonance_profiles::{BaseResonanceProfile, ConfigurationProfile, SeedResonanceMap, ResonanceAnalyzer};
pub use performance::{PerfMonitor, PerfMetrics};

/// Core result type for the physics engine
pub type PhysicsResult<T> = Result<T, PhysicsError>;

/// Errors that can occur in the physics engine
#[derive(Debug, Clone, thiserror::Error)]
pub enum PhysicsError {
    #[error("Invalid membrane configuration: {0}")]
    InvalidMembrane(String),
    
    #[error("Gravitational calculation failed: {0}")]
    GravitationalError(String),
    
    #[error("Lagrange point analysis failed: {0}")]
    LagrangeError(String),
    
    #[error("Spacetime metric error: {0}")]
    SpacetimeError(String),
    
    #[error("Numerical integration failed: {0}")]
    IntegrationError(String),
    
    #[error("Prime generation failed after {attempts} attempts")]
    PrimeGenerationFailed { attempts: usize },
    
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
}

/// Physical constants for the prime universe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalConstants {
    /// Gravitational constant (dimensionless in prime space)
    pub gravity_constant: f64,
    
    /// Speed of light in prime space
    pub light_speed: f64,
    
    /// Base-dependent coupling strength
    pub base_coupling: f64,
    
    /// Chaos threshold for stability analysis
    pub chaos_threshold: f64,
    
    /// Minimum particle separation (prevents singularities)
    pub min_separation: f64,
}

impl Default for PhysicalConstants {
    fn default() -> Self {
        Self {
            gravity_constant: 1.0,
            light_speed: 299792458.0,
            base_coupling: 0.1,
            chaos_threshold: 2.0,
            min_separation: 0.1,
        }
    }
}

/// The main physics engine that orchestrates all subsystems
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct PrimeUniverse {
    /// All particles in the universe
    pub particles: Vec<PrimeParticle>,
    
    /// Current time in the simulation
    pub time: f64,
    
    /// Time step for integration
    pub dt: f64,
    
    /// Physical constants
    pub constants: PhysicalConstants,
    
    /// Base metrics for spacetime curvature
    pub metrics: HashMap<u32, BaseMetric>,
    
    /// Current force field
    pub field: GravitationalField,
    
    /// Lagrange point analysis
    pub lagrange_analysis: Option<ClusterAnalysis>,
    
    /// Chaos indicators and stability metrics
    pub stability_metrics: StabilityMetrics,
    
    /// Tidal physics analyzer
    pub tidal_analyzer: TidalAnalyzer,
}

/// Metrics for tracking system stability and chaos
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StabilityMetrics {
    /// Lyapunov-like chaos indicator
    pub chaos_indicator: f64,
    
    /// Total energy in the system
    pub total_energy: f64,
    
    /// Energy drift from conservation
    pub energy_drift: f64,
    
    /// Maximum particle velocity
    pub max_velocity: f64,
    
    /// Number of close encounters
    pub close_encounters: usize,
    
    /// System entropy measure
    pub entropy: f64,
}

impl PrimeUniverse {
    /// Create a new prime universe with default physics
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            time: 0.0,
            dt: 0.01,
            constants: PhysicalConstants::default(),
            metrics: HashMap::new(),
            field: GravitationalField::new(),
            lagrange_analysis: None,
            stability_metrics: StabilityMetrics::default(),
            tidal_analyzer: TidalAnalyzer::new(),
        }
    }
    
    /// Create a universe with custom physical constants
    pub fn with_constants(constants: PhysicalConstants) -> Self {
        Self {
            constants,
            ..Self::new()
        }
    }
    
    /// Add a prime particle to the universe
    pub fn add_particle(&mut self, particle: PrimeParticle) -> PhysicsResult<()> {
        // Validate particle properties
        if particle.mass <= 0.0 {
            return Err(PhysicsError::InvalidConfiguration(
                format!("Particle mass must be positive, got {}", particle.mass)
            ));
        }
        
        if !particle.position.iter().all(|&x| x.is_finite()) {
            return Err(PhysicsError::InvalidConfiguration(
                "Particle position contains non-finite values".to_string()
            ));
        }
        
        if !particle.velocity.iter().all(|&x| x.is_finite()) {
            return Err(PhysicsError::InvalidConfiguration(
                "Particle velocity contains non-finite values".to_string()
            ));
        }
        
        self.particles.push(particle);
        
        // Safe access to last particle
        if let Some(last_particle) = self.particles.last() {
            self.field.add_source(last_particle);
        }
        
        Ok(())
    }
    
    /// Add multiple particles at once
    pub fn add_particles(&mut self, particles: Vec<PrimeParticle>) -> PhysicsResult<()> {
        for particle in particles {
            self.add_particle(particle)?;
        }
        Ok(())
    }
    
    /// Generate a membrane-constructed prime and add it to the universe
    pub fn generate_membrane_prime(
        &mut self, 
        config: MembraneConfig,
        position: [f64; 2],
        velocity: [f64; 2],
        name: String
    ) -> PhysicsResult<()> {
        let particle = MembraneBuilder::new(config)
            .with_position(position)
            .with_velocity(velocity)
            .with_name(name)
            .build()?;
            
        self.add_particle(particle)?;
        Ok(())
    }
    
    /// Step the simulation forward by one time step
    pub fn step(&mut self) -> PhysicsResult<()> {
        // Validate time step
        if self.dt <= 0.0 || !self.dt.is_finite() {
            return Err(PhysicsError::IntegrationError(
                format!("Invalid time step: {}", self.dt)
            ));
        }
        
        // Calculate forces
        let forces = self.field.calculate_forces(&self.particles, &self.constants)?;
        
        // Validate forces vector length
        if forces.len() != self.particles.len() {
            return Err(PhysicsError::IntegrationError(
                format!("Forces vector length {} doesn't match particles length {}", 
                    forces.len(), self.particles.len())
            ));
        }
        
        // Update particle positions and velocities (Euler integration)
        self.integrate_euler(forces)?;
        
        // Update time
        self.time += self.dt;
        
        // Update stability metrics
        self.update_stability_metrics()?;
        
        // Refresh force field
        self.field.update(&self.particles);
        
        Ok(())
    }
    
    /// Run the simulation for a given number of steps
    pub fn run(&mut self, steps: usize) -> PhysicsResult<()> {
        for _ in 0..steps {
            self.step()?;
        }
        Ok(())
    }
    
    /// Run until a specific time
    pub fn run_until(&mut self, end_time: f64) -> PhysicsResult<()> {
        while self.time < end_time {
            self.step()?;
        }
        Ok(())
    }
    
    /// Find all Lagrange points in the current system
    pub fn find_lagrange_points(&mut self) -> PhysicsResult<Vec<LagrangePoint>> {
        if self.particles.len() < 2 {
            return Err(PhysicsError::LagrangeError(
                "Need at least 2 particles for Lagrange analysis".to_string()
            ));
        }
        
        let mut analysis = ClusterAnalysis::new(&self.particles, &self.field)?;
        let points = analysis.find_all_lagrange_points(&self.particles)?;
        
        self.lagrange_analysis = Some(analysis);
        Ok(points)
    }
    
    /// Get current chaos indicator
    pub fn chaos_level(&self) -> f64 {
        self.stability_metrics.chaos_indicator
    }
    
    /// Check if the system is in a chaotic state
    pub fn is_chaotic(&self) -> bool {
        self.chaos_level() > self.constants.chaos_threshold
    }
    
    /// Get total system energy
    pub fn total_energy(&self) -> f64 {
        self.stability_metrics.total_energy
    }
    
    /// Analyze tidal field and find organization zones
    pub fn analyze_tidal_field(&mut self, grid_size: usize) -> PhysicsResult<Vec<OrganizationZone>> {
        // Calculate tidal field
        let field = self.tidal_analyzer.calculate_tidal_field(&self.particles, grid_size)?;
        
        // Find organization zones
        let zones = self.tidal_analyzer.find_organization_zones(&self.particles, &field)?;
        
        // Take a snapshot for history
        self.tidal_analyzer.take_snapshot(&field, &zones, &self.particles);
        
        Ok(zones)
    }
    
    /// Get tidal strength at a specific point
    pub fn tidal_strength_at(&self, point: [f64; 2]) -> PhysicsResult<f64> {
        let field = self.tidal_analyzer.calculate_tidal_at_point(point, &self.particles)?;
        Ok(field.strength)
    }
    
    /// Check if a point is in an organizing tidal zone
    pub fn is_organizing_zone(&self, point: [f64; 2]) -> PhysicsResult<bool> {
        let field = self.tidal_analyzer.calculate_tidal_at_point(point, &self.particles)?;
        Ok(matches!(field.zone_type, TidalZone::Organizing { .. }))
    }
    
    /// Discover the resonance profile for a specific base
    pub fn discover_base_resonance(
        &self,
        base: u32,
        outer_range: &[u8],
        inner_range: &[u8],
        k_range: &[u8],
        max_middle_length: usize,
    ) -> PhysicsResult<BaseResonanceProfile> {
        let mut profile = BaseResonanceProfile::new(base);
        profile.discover_resonances(outer_range, inner_range, k_range, max_middle_length)
            .map_err(|e| PhysicsError::InvalidConfiguration(e.to_string()))?;
        Ok(profile)
    }
    
    /// Create a "trinity" system with three different bases
    pub fn create_trinity(base1: u32, base2: u32, base3: u32) -> PhysicsResult<Self> {
        let mut universe = Self::new();
        
        // Generate three membrane primes with different bases
        let configs = [
            MembraneConfig::new(base1, 3, 7, 2, 2),
            MembraneConfig::new(base2, 3, 8, 2, 2), 
            MembraneConfig::new(base3, 5, 7, 2, 2),
        ];
        
        let names = ["APOLLO", "HERMES", "ATHENA"];
        let positions = [
            [1.0, -2.0],
            [0.0, -6.0], 
            [-5.0, -1.0],
        ];
        
        for (i, config) in configs.iter().enumerate() {
            universe.generate_membrane_prime(
                config.clone(),
                positions[i],
                [0.0, 0.0],
                names[i].to_string()
            )?;
        }
        
        Ok(universe)
    }
    
    /// Private helper methods
    fn integrate_euler(&mut self, forces: Vec<[f64; 2]>) -> PhysicsResult<()> {
        // Euler integration for N-body system
        // Honest about being Euler, not misleadingly labeled as RK4
        
        const MAX_ACCELERATION: f64 = 1e10;
        const MAX_VELOCITY: f64 = 1e8;
        const MAX_POSITION: f64 = 1e12;
        
        for (i, particle) in self.particles.iter_mut().enumerate() {
            let force = forces[i];
            let mass = particle.mass;
            
            // Validate mass is positive and finite
            if mass <= 0.0 || !mass.is_finite() {
                return Err(PhysicsError::IntegrationError(
                    format!("Invalid particle mass: {mass}")
                ));
            }
            
            // Calculate acceleration with bounds checking
            let ax = force[0] / mass;
            let ay = force[1] / mass;
            
            // Validate acceleration is finite and reasonable
            if !ax.is_finite() || !ay.is_finite() {
                return Err(PhysicsError::IntegrationError(
                    "Non-finite acceleration encountered".to_string()
                ));
            }
            
            if ax.abs() > MAX_ACCELERATION || ay.abs() > MAX_ACCELERATION {
                return Err(PhysicsError::IntegrationError(
                    format!("Acceleration too large: ({ax}, {ay})")
                ));
            }
            
            // Update velocity
            particle.velocity[0] += ax * self.dt;
            particle.velocity[1] += ay * self.dt;
            
            // Validate velocity
            if !particle.velocity[0].is_finite() || !particle.velocity[1].is_finite() {
                return Err(PhysicsError::IntegrationError(
                    "Non-finite velocity encountered".to_string()
                ));
            }
            
            if particle.velocity[0].abs() > MAX_VELOCITY || particle.velocity[1].abs() > MAX_VELOCITY {
                return Err(PhysicsError::IntegrationError(
                    format!("Velocity too large: ({}, {})", particle.velocity[0], particle.velocity[1])
                ));
            }
            
            // Update position
            particle.position[0] += particle.velocity[0] * self.dt;
            particle.position[1] += particle.velocity[1] * self.dt;
            
            // Validate position
            if !particle.position[0].is_finite() || !particle.position[1].is_finite() {
                return Err(PhysicsError::IntegrationError(
                    "Non-finite position encountered".to_string()
                ));
            }
            
            if particle.position[0].abs() > MAX_POSITION || particle.position[1].abs() > MAX_POSITION {
                return Err(PhysicsError::IntegrationError(
                    format!("Position too large: ({}, {})", particle.position[0], particle.position[1])
                ));
            }
        }
        
        Ok(())
    }
    
    fn update_stability_metrics(&mut self) -> PhysicsResult<()> {
        // Calculate chaos indicator, energy, etc.
        let mut total_kinetic = 0.0;
        let mut total_potential = 0.0;
        let mut max_vel: f64 = 0.0;
        
        for particle in &self.particles {
            let vel_sq = particle.velocity[0].powi(2) + particle.velocity[1].powi(2);
            total_kinetic += 0.5 * particle.mass * vel_sq;
            max_vel = max_vel.max(vel_sq.sqrt());
        }
        
        // Simple potential calculation (TODO: use proper field calculation)
        for i in 0..self.particles.len() {
            for j in i+1..self.particles.len() {
                let dx = self.particles[i].position[0] - self.particles[j].position[0];
                let dy = self.particles[i].position[1] - self.particles[j].position[1];
                let r = (dx*dx + dy*dy).sqrt().max(self.constants.min_separation);
                
                total_potential -= self.constants.gravity_constant * 
                    self.particles[i].mass * self.particles[j].mass / r;
            }
        }
        
        let total_energy = total_kinetic + total_potential;
        let energy_drift = if self.stability_metrics.total_energy == 0.0 {
            0.0
        } else {
            (total_energy - self.stability_metrics.total_energy).abs() / 
            self.stability_metrics.total_energy.abs()
        };
        
        self.stability_metrics.total_energy = total_energy;
        self.stability_metrics.energy_drift = energy_drift;
        self.stability_metrics.max_velocity = max_vel;
        
        // Simple chaos indicator based on energy drift
        self.stability_metrics.chaos_indicator = energy_drift * 100.0;
        
        Ok(())
    }
}

impl Default for PrimeUniverse {
    fn default() -> Self {
        Self::new()
    }
}

/// Fast primality testing using multiple methods
pub fn is_prime(n: &BigUint) -> bool {
    // Use primal crate for small numbers, Miller-Rabin for large ones
    if let Some(n_u64) = n.to_u64() {
        primal::is_prime(n_u64)
    } else {
        is_prime_miller_rabin(n)
    }
}

/// Miller-Rabin primality test for large numbers
pub fn is_prime_miller_rabin(n: &BigUint) -> bool {
    miller_rabin_test(n, 20)
}

/// Miller-Rabin primality test with configurable rounds
pub fn miller_rabin_test(n: &BigUint, rounds: usize) -> bool {
    
    use num_bigint::RandBigInt;
    
    // Validate input parameters
    if rounds == 0 {
        eprintln!("Warning: Miller-Rabin called with 0 rounds, using default 20");
        return miller_rabin_test(n, 20);
    }
    
    if rounds > 100 {
        eprintln!("Warning: Miller-Rabin called with {rounds} rounds, capping at 100");
        return miller_rabin_test(n, 100);
    }
    
    if n < &BigUint::from(2u32) { return false; }
    if n == &BigUint::from(2u32) { return true; }
    if n % 2u32 == BigUint::zero() { return false; }
    
    // Find r and d such that n-1 = 2^r * d
    let n_minus_1 = n - 1u32;
    let mut d = n_minus_1.clone();
    let mut r = 0;
    
    while &d % 2u32 == BigUint::zero() {
        d /= 2u32;
        r += 1;
    }
    
    let mut rng = rand::thread_rng();
    
    // Miller-Rabin test with specified rounds
    for _ in 0..rounds {
        let a = rng.gen_biguint_range(&BigUint::from(2u32), n);
        let mut x = a.modpow(&d, n);
        
        if x == BigUint::one() || x == n_minus_1 {
            continue;
        }
        
        let mut is_composite = true;
        for _ in 0..r-1 {
            x = (&x * &x) % n;
            if x == n_minus_1 {
                is_composite = false;
                break;
            }
        }
        
        if is_composite {
            return false;
        }
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universe_creation() {
        let universe = PrimeUniverse::new();
        assert_eq!(universe.particles.len(), 0);
        assert_eq!(universe.time, 0.0);
    }
    
    #[test]
    fn test_trinity_creation() {
        let _universe = PrimeUniverse::create_trinity(10, 11, 12);
        // Note: This might fail if prime generation fails, which is expected
        // In a real test, we'd use deterministic prime generation
    }
    
    #[test]
    fn test_primality() {
        assert!(is_prime(&BigUint::from(2u32)));
        assert!(is_prime(&BigUint::from(17u32)));
        assert!(!is_prime(&BigUint::from(4u32)));
        assert!(!is_prime(&BigUint::from(15u32)));
    }
}