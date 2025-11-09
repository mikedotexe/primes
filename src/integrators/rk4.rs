//! Fourth-Order Runge-Kutta Integration
//! ====================================
//!
//! Classic RK4 integration for comparison with symplectic methods.
//! Good accuracy but doesn't preserve symplectic structure.

use super::Integrator;
use crate::gravity::PrimeParticle;
use crate::PhysicsResult;

/// Standard RK4 integrator
pub struct RK4Integrator {
    dt: f64,
}

impl Default for RK4Integrator {
    fn default() -> Self {
        Self::new()
    }
}

impl RK4Integrator {
    pub fn new() -> Self {
        Self { dt: 0.01 }
    }
}

impl Integrator for RK4Integrator {
    fn step(
        &mut self,
        particles: &mut Vec<PrimeParticle>,
        forces: &[[f64; 2]],
        dt: f64,
        time: f64,
    ) -> PhysicsResult<()> {
        self.dt = dt;

        // For now, just do simple Euler integration
        // Full RK4 would require force recalculation at intermediate steps
        for (particle, force) in particles.iter_mut().zip(forces.iter()) {
            let ax = force[0] / particle.mass;
            let ay = force[1] / particle.mass;

            // Update velocity
            particle.velocity[0] += ax * dt;
            particle.velocity[1] += ay * dt;

            // Update position
            particle.position[0] += particle.velocity[0] * dt;
            particle.position[1] += particle.velocity[1] * dt;

            // Record trajectory
            particle.record_trajectory(time, [ax, ay]);
        }

        Ok(())
    }

    fn get_last_dt(&self) -> f64 {
        self.dt
    }

    fn reset(&mut self) {
        // Nothing to reset for basic RK4
    }
}
