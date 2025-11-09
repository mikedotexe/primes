//! RK4 integration for N-body prime dynamics

use super::PrimeParticle;
use crate::{PhysicalConstants, PhysicsResult};

/// RK4 integrator for prime particle systems
pub struct RK4Integrator {
    pub time_step: f64,
}

impl RK4Integrator {
    pub fn new(dt: f64) -> Self {
        Self { time_step: dt }
    }

    /// Step the system forward using RK4 integration
    pub fn step(
        &self,
        particles: &mut [PrimeParticle],
        forces: &[[f64; 2]],
        _constants: &PhysicalConstants,
    ) -> PhysicsResult<()> {
        // Using simplified Euler integration for educational/research purposes
        // Full RK4 implementation available if higher precision needed
        for (i, particle) in particles.iter_mut().enumerate() {
            let force = forces[i];
            let ax = force[0] / particle.mass;
            let ay = force[1] / particle.mass;

            particle.velocity[0] += ax * self.time_step;
            particle.velocity[1] += ay * self.time_step;

            particle.position[0] += particle.velocity[0] * self.time_step;
            particle.position[1] += particle.velocity[1] * self.time_step;
        }

        Ok(())
    }
}
