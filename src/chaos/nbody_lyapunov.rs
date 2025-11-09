//! N-Body Lyapunov Exponent Calculator for Prime Dynamics
//! ======================================================
//!
//! This module implements REAL chaos measurement by tracking actual
//! gravitational trajectory divergence in prime particle systems.

use crate::gravity::{ForceCalculator, PrimeParticle};
use crate::membrane::MembraneConfig;
use crate::{PhysicalConstants, PhysicsResult};
use std::collections::HashMap;

/// Type alias for chaos configuration: (outer, inner, k_outer, k_inner)
type ChaosConfig = (u32, u32, u32, u32);
use crate::integrators::{Integrator, SymplecticIntegrator};
use num_bigint::BigUint;

/// N-Body Lyapunov calculator that tracks real trajectory divergence
pub struct NBodyLyapunov {
    /// Original system of particles
    pub particles: Vec<PrimeParticle>,

    /// Shadow system (starts ε away)
    pub shadow_particles: Vec<PrimeParticle>,

    /// Force calculators for each system
    pub force_calc: ForceCalculator,
    pub shadow_force_calc: ForceCalculator,

    /// Integrators for each system
    pub integrator: Box<dyn Integrator>,
    pub shadow_integrator: Box<dyn Integrator>,

    /// Physical constants
    pub constants: PhysicalConstants,

    /// Initial separation
    pub epsilon: f64,

    /// Time step
    pub dt: f64,

    /// Current simulation time
    pub time: f64,

    /// Separation history
    pub separation_history: Vec<(f64, f64)>, // (time, separation)

    /// Renormalization events
    pub renorm_events: Vec<(f64, f64)>, // (time, growth_factor)

    /// Energy tracking
    pub energy_history: Vec<(f64, f64, f64)>, // (time, original_energy, shadow_energy)
}

impl NBodyLyapunov {
    /// Create a new N-body Lyapunov calculator
    pub fn new(particles: Vec<PrimeParticle>, epsilon: f64, dt: f64) -> Self {
        // Create shadow system with small perturbation
        let mut shadow_particles = particles.clone();
        if !shadow_particles.is_empty() {
            shadow_particles[0].position[0] += epsilon;
        }

        Self {
            particles,
            shadow_particles,
            force_calc: ForceCalculator::new(),
            shadow_force_calc: ForceCalculator::new(),
            integrator: Box::new(SymplecticIntegrator::new()),
            shadow_integrator: Box::new(SymplecticIntegrator::new()),
            constants: PhysicalConstants::default(),
            epsilon,
            dt,
            time: 0.0,
            separation_history: Vec::new(),
            renorm_events: Vec::new(),
            energy_history: Vec::new(),
        }
    }

    /// Create a new N-body Lyapunov calculator with custom integrator
    pub fn with_integrator(
        particles: Vec<PrimeParticle>,
        epsilon: f64,
        dt: f64,
        integrator: Box<dyn Integrator>,
    ) -> Self {
        // Create shadow system with small perturbation
        let mut shadow_particles = particles.clone();
        if !shadow_particles.is_empty() {
            shadow_particles[0].position[0] += epsilon;
        }

        // Clone the integrator for shadow system
        let shadow_integrator = Box::new(SymplecticIntegrator::new()); // Fallback to symplectic

        Self {
            particles,
            shadow_particles,
            force_calc: ForceCalculator::new(),
            shadow_force_calc: ForceCalculator::new(),
            integrator,
            shadow_integrator,
            constants: PhysicalConstants::default(),
            epsilon,
            dt,
            time: 0.0,
            separation_history: Vec::new(),
            renorm_events: Vec::new(),
            energy_history: Vec::new(),
        }
    }

    /// Step both systems forward using symplectic integration
    pub fn step(&mut self) -> PhysicsResult<()> {
        // Calculate forces for both systems
        let forces =
            self.force_calc
                .calculate_all_forces(&self.particles, &self.constants, self.time)?;

        let shadow_forces = self.shadow_force_calc.calculate_all_forces(
            &self.shadow_particles,
            &self.constants,
            self.time,
        )?;

        // Use symplectic integration for both systems
        self.integrator
            .step(&mut self.particles, &forces, self.dt, self.time)?;
        self.shadow_integrator.step(
            &mut self.shadow_particles,
            &shadow_forces,
            self.dt,
            self.time,
        )?;

        // Update time
        self.time += self.dt;

        // Track separation
        let separation = self.calculate_system_separation();
        self.separation_history.push((self.time, separation));

        // Track energy
        let orig_energy = self.calculate_total_energy(&self.particles);
        let shadow_energy = self.calculate_total_energy(&self.shadow_particles);
        self.energy_history
            .push((self.time, orig_energy, shadow_energy));

        // Renormalize if separation gets too large
        const RENORM_THRESHOLD: f64 = 1e-3;
        if separation > RENORM_THRESHOLD {
            self.renormalize(separation)?;
        }

        Ok(())
    }

    /// Run simulation for specified number of steps
    pub fn run(&mut self, steps: usize) -> PhysicsResult<()> {
        for _ in 0..steps {
            self.step()?;
        }
        Ok(())
    }

    /// Calculate the Lyapunov exponent from the simulation
    pub fn calculate_lyapunov(&self) -> f64 {
        if self.renorm_events.is_empty() {
            // No renormalizations - use final separation
            if let Some((_, final_sep)) = self.separation_history.last() {
                let growth = final_sep / self.epsilon;
                return growth.ln() / self.time;
            }
            return 0.0;
        }

        // Sum logarithms of all growth factors
        let sum_log_growth: f64 = self
            .renorm_events
            .iter()
            .map(|(_, factor)| factor.ln())
            .sum();

        sum_log_growth / self.time
    }

    /// Calculate separation between original and shadow systems
    fn calculate_system_separation(&self) -> f64 {
        let mut sum_sq = 0.0;

        for (p1, p2) in self.particles.iter().zip(&self.shadow_particles) {
            let dx = p1.position[0] - p2.position[0];
            let dy = p1.position[1] - p2.position[1];
            sum_sq += dx * dx + dy * dy;
        }

        sum_sq.sqrt()
    }

    /// Renormalize shadow system when separation gets too large
    fn renormalize(&mut self, separation: f64) -> PhysicsResult<()> {
        let growth_factor = separation / self.epsilon;
        self.renorm_events.push((self.time, growth_factor));

        // Calculate renormalization direction
        for (orig, shadow) in self.particles.iter().zip(&mut self.shadow_particles) {
            let dx = shadow.position[0] - orig.position[0];
            let dy = shadow.position[1] - orig.position[1];
            let dist = (dx * dx + dy * dy).sqrt();

            if dist > 0.0 {
                // Pull shadow back to epsilon distance
                let scale = self.epsilon / separation;
                shadow.position[0] = orig.position[0] + dx * scale;
                shadow.position[1] = orig.position[1] + dy * scale;

                // Scale velocity difference similarly
                let dvx = shadow.velocity[0] - orig.velocity[0];
                let dvy = shadow.velocity[1] - orig.velocity[1];
                shadow.velocity[0] = orig.velocity[0] + dvx * scale;
                shadow.velocity[1] = orig.velocity[1] + dvy * scale;
            }
        }

        Ok(())
    }

    /// Calculate total energy of a system
    pub fn calculate_total_energy(&self, particles: &[PrimeParticle]) -> f64 {
        let mut kinetic = 0.0;
        let mut potential = 0.0;

        // Kinetic energy
        for p in particles {
            kinetic += p.kinetic_energy();
        }

        // Potential energy (pairwise)
        for i in 0..particles.len() {
            for j in i + 1..particles.len() {
                let dist = particles[i].distance_to(&particles[j]);
                if dist > self.constants.min_separation {
                    potential -=
                        self.constants.gravity_constant * particles[i].mass * particles[j].mass
                            / dist;
                }
            }
        }

        kinetic + potential
    }

    /// Calculate energy drift percentage
    pub fn calculate_energy_drift(&self) -> f64 {
        if self.energy_history.len() < 2 {
            return 0.0;
        }

        let initial_energy = self.energy_history[0].1; // Original system initial energy
        let final_energy = self.energy_history.last().unwrap().1; // Original system final energy

        if initial_energy.abs() < 1e-12 {
            return 0.0; // Avoid division by zero
        }

        (final_energy - initial_energy) / initial_energy.abs()
    }

    /// Calculate forces for all particles (helper for spectrum analysis)
    pub fn calculate_all_forces(
        &mut self,
        particles: &[PrimeParticle],
    ) -> PhysicsResult<Vec<[f64; 2]>> {
        self.force_calc
            .calculate_all_forces(particles, &self.constants, self.time)
    }

    /// Get chaos classification
    pub fn chaos_classification(&self) -> &'static str {
        let lyapunov = self.calculate_lyapunov();
        match lyapunov {
            l if l > 0.5 => "EXTREME CHAOS",
            l if l > 0.3 => "CHAOS STORM",
            l if l > 0.1 => "Chaotic",
            l if l > 0.01 => "Edge of chaos",
            l if l > -0.01 => "Marginally stable",
            _ => "Stable",
        }
    }
}

/// Configuration chaos analyzer that correlates chaos with prime generation
pub struct ConfigurationChaosAnalyzer {
    /// Maps configurations to their chaos levels AND prime generation success
    pub chaos_prime_correlation: HashMap<(u32, u32, u32, u32), ChaosPrimeMetrics>,

    /// Number of steps to simulate
    pub simulation_steps: usize,

    /// Time step
    pub dt: f64,

    /// Number of prime generation attempts per config
    pub prime_attempts: usize,
}

/// Metrics combining chaos and prime generation success
#[derive(Debug, Clone)]
pub struct ChaosPrimeMetrics {
    pub lyapunov_exponent: f64,
    pub prime_success_rate: f64,
    pub average_energy_drift: f64,
    pub max_velocity: f64,
    pub classification: String,
    pub successful_primes: Vec<BigUint>,
    pub config: MembraneConfig,
}

impl ConfigurationChaosAnalyzer {
    pub fn new(simulation_steps: usize, dt: f64, prime_attempts: usize) -> Self {
        Self {
            chaos_prime_correlation: HashMap::new(),
            simulation_steps,
            dt,
            prime_attempts,
        }
    }

    /// Analyze a specific configuration for chaos AND prime generation
    pub fn analyze_configuration(
        &mut self,
        outer: u32,
        inner: u32,
        k_outer: u32,
        k_inner: u32,
    ) -> PhysicsResult<ChaosPrimeMetrics> {
        let config = MembraneConfig::new(10, outer, inner, k_outer, k_inner);

        // Generate test particles with this configuration
        let particles = self.create_test_particles(&config)?;

        // Measure chaos
        let mut lyapunov_calc = NBodyLyapunov::new(particles, 1e-8, self.dt);
        lyapunov_calc.run(self.simulation_steps)?;

        let lyapunov = lyapunov_calc.calculate_lyapunov();
        let classification = lyapunov_calc.chaos_classification().to_string();

        // Calculate energy drift
        let energy_drift = if let (Some((_, e0, _)), Some((_, ef, _))) = (
            lyapunov_calc.energy_history.first(),
            lyapunov_calc.energy_history.last(),
        ) {
            ((ef - e0).abs() / e0.abs()) * 100.0
        } else {
            0.0
        };

        // Find max velocity
        let max_velocity = lyapunov_calc
            .particles
            .iter()
            .map(|p| p.speed())
            .fold(0.0, f64::max);

        // Test prime generation success
        let (success_rate, successful_primes) = self.test_prime_generation(&config)?;

        let metrics = ChaosPrimeMetrics {
            lyapunov_exponent: lyapunov,
            prime_success_rate: success_rate,
            average_energy_drift: energy_drift,
            max_velocity,
            classification,
            successful_primes,
            config: config.clone(),
        };

        self.chaos_prime_correlation
            .insert((outer, inner, k_outer, k_inner), metrics.clone());

        Ok(metrics)
    }

    /// Create test particles for chaos measurement
    fn create_test_particles(&self, config: &MembraneConfig) -> PhysicsResult<Vec<PrimeParticle>> {
        // Create a simple two-body system with the given configuration
        let mut particles = Vec::new();

        // Try to generate two primes with this config
        for i in 0..2 {
            if let Ok(particle) = crate::membrane::MembraneBuilder::new(config.clone())
                .with_position([i as f64 * 2.0 - 1.0, 0.0])
                .with_velocity([0.0, 0.5 * (1.0 - 2.0 * i as f64)])
                .with_name(format!("Test-{i}"))
                .with_seed(i as u8)
                .build()
            {
                particles.push(particle);
            }
        }

        // If we couldn't generate primes, use dummy particles
        if particles.len() < 2 {
            use num_traits::FromPrimitive;
            particles.clear();
            particles.push(PrimeParticle::new(
                BigUint::from_u64(41).unwrap(),
                10,
                [-1.0, 0.0],
                [0.0, 0.5],
                "Test-0".to_string(),
            ));
            particles.push(PrimeParticle::new(
                BigUint::from_u64(43).unwrap(),
                10,
                [1.0, 0.0],
                [0.0, -0.5],
                "Test-1".to_string(),
            ));
        }

        Ok(particles)
    }

    /// Test prime generation success for a configuration
    fn test_prime_generation(&self, config: &MembraneConfig) -> PhysicsResult<(f64, Vec<BigUint>)> {
        let mut successes = 0;
        let mut successful_primes = Vec::new();

        for seed in 0..self.prime_attempts {
            if let Ok(particle) = crate::membrane::MembraneBuilder::new(config.clone())
                .with_seed(seed as u8)
                .build()
            {
                successes += 1;
                successful_primes.push(particle.value);
            }
        }

        let success_rate = successes as f64 / self.prime_attempts as f64;
        Ok((success_rate, successful_primes))
    }

    /// Find configurations with optimal chaos for prime generation
    pub fn find_optimal_chaos(&self) -> Vec<(ChaosConfig, &ChaosPrimeMetrics)> {
        let mut results: Vec<_> = self
            .chaos_prime_correlation
            .iter()
            .map(|(config, metrics)| (*config, metrics))
            .collect();

        // Sort by prime success rate
        results.sort_by(|a, b| {
            b.1.prime_success_rate
                .partial_cmp(&a.1.prime_success_rate)
                .unwrap()
        });

        results
    }

    /// Test the hypothesis: moderate chaos = optimal prime generation
    pub fn test_moderate_chaos_hypothesis(&self) -> bool {
        // Group configurations by chaos level
        let mut stable: Vec<f64> = Vec::new();
        let mut moderate: Vec<f64> = Vec::new();
        let mut extreme: Vec<f64> = Vec::new();

        for metrics in self.chaos_prime_correlation.values() {
            match metrics.lyapunov_exponent {
                l if l < 0.05 => stable.push(metrics.prime_success_rate),
                l if l < 0.2 => moderate.push(metrics.prime_success_rate),
                _ => extreme.push(metrics.prime_success_rate),
            }
        }

        // Calculate averages
        let avg_stable = stable.iter().sum::<f64>() / stable.len().max(1) as f64;
        let avg_moderate = moderate.iter().sum::<f64>() / moderate.len().max(1) as f64;
        let avg_extreme = extreme.iter().sum::<f64>() / extreme.len().max(1) as f64;

        println!("\n🧪 MODERATE CHAOS HYPOTHESIS TEST:");
        println!(
            "  Stable configs (λ < 0.05): {:.1}% prime success",
            avg_stable * 100.0
        );
        println!(
            "  Moderate chaos (0.05 ≤ λ < 0.2): {:.1}% prime success",
            avg_moderate * 100.0
        );
        println!(
            "  Extreme chaos (λ ≥ 0.2): {:.1}% prime success",
            avg_extreme * 100.0
        );

        // Hypothesis is true if moderate chaos has highest success rate
        avg_moderate > avg_stable && avg_moderate > avg_extreme
    }
}
