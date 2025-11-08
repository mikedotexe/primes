//! Numerical Integration Methods for Prime Dynamics
//! ================================================
//! 
//! Provides various integration schemes optimized for different scenarios:
//! - Symplectic integrators for energy conservation
//! - Adaptive methods for handling close encounters
//! - High-order methods for precision studies

use crate::PhysicsResult;
use crate::gravity::PrimeParticle;

pub mod symplectic;
pub mod adaptive;
pub mod rk4;

pub use symplectic::SymplecticIntegrator;
pub use adaptive::AdaptiveIntegrator;
pub use rk4::RK4Integrator;

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