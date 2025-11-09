//! Adaptive Step Size Integration
//! ==============================
//!
//! Automatically adjusts timestep based on local error estimates

use super::Integrator;
use crate::gravity::PrimeParticle;
use crate::PhysicsResult;

/// Adaptive timestep integrator
pub struct AdaptiveIntegrator {
    min_dt: f64,
    max_dt: f64,
    current_dt: f64,
    _tolerance: f64,
}

impl AdaptiveIntegrator {
    pub fn new(min_dt: f64, max_dt: f64, tolerance: f64) -> Self {
        Self {
            min_dt,
            max_dt,
            current_dt: (min_dt + max_dt) / 2.0,
            _tolerance: tolerance,
        }
    }
}

impl Integrator for AdaptiveIntegrator {
    fn step(
        &mut self,
        particles: &mut Vec<PrimeParticle>,
        forces: &[[f64; 2]],
        _dt: f64,
        time: f64,
    ) -> PhysicsResult<()> {
        // Simple placeholder - would implement error estimation
        for (particle, force) in particles.iter_mut().zip(forces.iter()) {
            let ax = force[0] / particle.mass;
            let ay = force[1] / particle.mass;

            particle.velocity[0] += ax * self.current_dt;
            particle.velocity[1] += ay * self.current_dt;

            particle.position[0] += particle.velocity[0] * self.current_dt;
            particle.position[1] += particle.velocity[1] * self.current_dt;

            particle.record_trajectory(time, [ax, ay]);
        }

        Ok(())
    }

    fn get_last_dt(&self) -> f64 {
        self.current_dt
    }

    fn reset(&mut self) {
        self.current_dt = (self.min_dt + self.max_dt) / 2.0;
    }
}
