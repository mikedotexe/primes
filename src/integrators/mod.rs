//! # Numerical Integrators -- Visualization Metaphor
//!
//! **Layer**: Simulation / visualization (not core math)
//!
//! Provides integration schemes (symplectic, adaptive, RK4) for the
//! [`crate::gravity`] N-body particle simulation.

use crate::gravity::PrimeParticle;
use crate::PhysicsResult;

pub mod adaptive;
pub mod rk4;
pub mod symplectic;

pub use adaptive::AdaptiveIntegrator;
pub use rk4::RK4Integrator;
pub use symplectic::SymplecticIntegrator;

/// Common interface for all integrators
pub trait Integrator {
    /// Advance the system by one timestep
    fn step(
        &mut self,
        particles: &mut Vec<PrimeParticle>,
        forces: &[[f64; 2]],
        dt: f64,
        time: f64,
    ) -> PhysicsResult<()>;

    /// Get the actual timestep used (for adaptive integrators)
    fn get_last_dt(&self) -> f64;

    /// Reset internal state
    fn reset(&mut self);
}

/// Integration statistics for analysis
#[derive(Debug, Clone)]
pub struct IntegrationStats {
    pub total_steps: usize,
    pub rejected_steps: usize,
    pub min_dt: f64,
    pub max_dt: f64,
    pub avg_dt: f64,
    pub energy_error: f64,
    pub momentum_error: f64,
}

impl Default for IntegrationStats {
    fn default() -> Self {
        Self {
            total_steps: 0,
            rejected_steps: 0,
            min_dt: f64::MAX,
            max_dt: 0.0,
            avg_dt: 0.0,
            energy_error: 0.0,
            momentum_error: 0.0,
        }
    }
}
