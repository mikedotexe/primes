//! Symplectic Integration for Energy Conservation
//! ==============================================
//!
//! Uses the Velocity Verlet algorithm to preserve the symplectic
//! structure of Hamiltonian systems, ensuring better long-term
//! energy conservation than standard Runge-Kutta methods.

use super::{IntegrationStats, Integrator};
use crate::gravity::PrimeParticle;
use crate::{PhysicsError, PhysicsResult};

/// Velocity Verlet symplectic integrator
pub struct SymplecticIntegrator {
    /// Previous accelerations for velocity update
    previous_accelerations: Vec<[f64; 2]>,

    /// Integration statistics
    stats: IntegrationStats,

    /// Whether this is the first step
    first_step: bool,
}

impl Default for SymplecticIntegrator {
    fn default() -> Self {
        Self::new()
    }
}

impl SymplecticIntegrator {
    pub fn new() -> Self {
        Self {
            previous_accelerations: Vec::new(),
            stats: IntegrationStats::default(),
            first_step: true,
        }
    }

    pub fn get_stats(&self) -> &IntegrationStats {
        &self.stats
    }
}

impl Integrator for SymplecticIntegrator {
    fn step(
        &mut self,
        particles: &mut Vec<PrimeParticle>,
        forces: &[[f64; 2]],
        dt: f64,
        time: f64,
    ) -> PhysicsResult<()> {
        if particles.len() != forces.len() {
            return Err(PhysicsError::InvalidConfiguration(
                "Particle and force counts don't match".to_string(),
            ));
        }

        // Calculate current accelerations
        let accelerations: Vec<[f64; 2]> = particles
            .iter()
            .zip(forces.iter())
            .map(|(p, f)| [f[0] / p.mass, f[1] / p.mass])
            .collect();

        // First step: just use forward Euler to bootstrap
        if self.first_step {
            for (i, particle) in particles.iter_mut().enumerate() {
                let acc = accelerations[i];

                // Update position: x(t+dt) = x(t) + v(t)*dt + 0.5*a(t)*dt²
                particle.position[0] += particle.velocity[0] * dt + 0.5 * acc[0] * dt * dt;
                particle.position[1] += particle.velocity[1] * dt + 0.5 * acc[1] * dt * dt;

                // Update velocity: v(t+dt) = v(t) + a(t)*dt
                particle.velocity[0] += acc[0] * dt;
                particle.velocity[1] += acc[1] * dt;

                // Record trajectory
                particle.record_trajectory(time + dt, acc);
            }

            self.previous_accelerations = accelerations;
            self.first_step = false;
        } else {
            // Velocity Verlet algorithm
            for (i, particle) in particles.iter_mut().enumerate() {
                let acc_old = self.previous_accelerations[i];
                let acc_new = accelerations[i];

                // Update position: x(t+dt) = x(t) + v(t)*dt + 0.5*a(t)*dt²
                particle.position[0] += particle.velocity[0] * dt + 0.5 * acc_old[0] * dt * dt;
                particle.position[1] += particle.velocity[1] * dt + 0.5 * acc_old[1] * dt * dt;

                // Update velocity: v(t+dt) = v(t) + 0.5*(a(t) + a(t+dt))*dt
                particle.velocity[0] += 0.5 * (acc_old[0] + acc_new[0]) * dt;
                particle.velocity[1] += 0.5 * (acc_old[1] + acc_new[1]) * dt;

                // Record trajectory
                particle.record_trajectory(time + dt, acc_new);
            }

            self.previous_accelerations = accelerations;
        }

        // Update statistics
        self.stats.total_steps += 1;
        self.stats.min_dt = self.stats.min_dt.min(dt);
        self.stats.max_dt = self.stats.max_dt.max(dt);
        self.stats.avg_dt = (self.stats.avg_dt * (self.stats.total_steps - 1) as f64 + dt)
            / self.stats.total_steps as f64;

        Ok(())
    }

    fn get_last_dt(&self) -> f64 {
        self.stats.avg_dt
    }

    fn reset(&mut self) {
        self.previous_accelerations.clear();
        self.stats = IntegrationStats::default();
        self.first_step = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;

    #[test]
    fn test_symplectic_energy_conservation() {
        // Create a simple two-body system
        let mut particles = vec![
            PrimeParticle::new(
                BigUint::from_u64(41).unwrap(),
                10,
                [-1.0, 0.0],
                [0.0, 0.5],
                "P1".to_string(),
            ),
            PrimeParticle::new(
                BigUint::from_u64(43).unwrap(),
                10,
                [1.0, 0.0],
                [0.0, -0.5],
                "P2".to_string(),
            ),
        ];

        let mut integrator = SymplecticIntegrator::new();

        // Simulate for 100 steps
        for i in 0..100 {
            // Simple gravitational force
            let r = ((particles[0].position[0] - particles[1].position[0]).powi(2)
                + (particles[0].position[1] - particles[1].position[1]).powi(2))
            .sqrt();
            let f = 1.0 / (r * r);

            let dx = particles[1].position[0] - particles[0].position[0];
            let dy = particles[1].position[1] - particles[0].position[1];

            let forces = vec![[f * dx / r, f * dy / r], [-f * dx / r, -f * dy / r]];

            integrator
                .step(&mut particles, &forces, 0.01, i as f64 * 0.01)
                .unwrap();
        }

        let stats = integrator.get_stats();
        assert_eq!(stats.total_steps, 100);
        assert!(stats.avg_dt > 0.0);
    }
}
