//! Chaos detection and analysis for prime particle systems

use super::PrimeParticle;
use crate::PhysicsResult;

/// Chaos detector for N-body prime systems
#[derive(Debug, Clone)]
pub struct ChaosDetector {
    /// Historical chaos indicators
    pub chaos_history: Vec<f64>,

    /// Lyapunov-like exponent
    pub lyapunov_exponent: f64,

    /// Maximum observed chaos level
    pub max_chaos: f64,

    /// Time window for chaos calculation
    pub time_window: f64,
}

impl Default for ChaosDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl ChaosDetector {
    /// Create a new chaos detector
    pub fn new() -> Self {
        Self {
            chaos_history: Vec::new(),
            lyapunov_exponent: 0.0,
            max_chaos: 0.0,
            time_window: 10.0,
        }
    }

    /// Calculate current chaos indicator for the system
    pub fn calculate_chaos_indicator(
        &mut self,
        particles: &[PrimeParticle],
        current_time: f64,
    ) -> PhysicsResult<f64> {
        if particles.len() < 2 {
            return Ok(0.0);
        }

        let mut total_divergence = 0.0;
        let mut pair_count = 0;

        // Calculate pairwise velocity divergence
        for i in 0..particles.len() {
            for j in i + 1..particles.len() {
                let divergence =
                    self.calculate_velocity_divergence(&particles[i], &particles[j])?;
                total_divergence += divergence;
                pair_count += 1;
            }
        }

        let chaos_indicator = if pair_count > 0 {
            total_divergence / pair_count as f64
        } else {
            0.0
        };

        // Update history
        self.chaos_history.push(chaos_indicator);
        if self.chaos_history.len() > 1000 {
            self.chaos_history.drain(0..500);
        }

        // Update maximum
        if chaos_indicator > self.max_chaos {
            self.max_chaos = chaos_indicator;
        }

        // Update Lyapunov estimate
        self.update_lyapunov_estimate(current_time);

        Ok(chaos_indicator)
    }

    /// Calculate velocity divergence between two particles
    fn calculate_velocity_divergence(
        &self,
        p1: &PrimeParticle,
        p2: &PrimeParticle,
    ) -> PhysicsResult<f64> {
        if p1.trajectory_history.len() < 2 || p2.trajectory_history.len() < 2 {
            return Ok(0.0);
        }

        // Get recent and past velocity states
        let recent_len = p1.trajectory_history.len();
        let past_len = recent_len.saturating_sub(10);

        if past_len >= recent_len {
            return Ok(0.0);
        }

        let p1_recent = &p1.trajectory_history[recent_len - 1];
        let p1_past = &p1.trajectory_history[past_len];
        let p2_recent = &p2.trajectory_history[recent_len - 1];
        let p2_past = &p2.trajectory_history[past_len];

        // Calculate velocity change for each particle
        let p1_dv = [
            p1_recent.velocity[0] - p1_past.velocity[0],
            p1_recent.velocity[1] - p1_past.velocity[1],
        ];

        let p2_dv = [
            p2_recent.velocity[0] - p2_past.velocity[0],
            p2_recent.velocity[1] - p2_past.velocity[1],
        ];

        // Calculate relative divergence
        let relative_dv = [p1_dv[0] - p2_dv[0], p1_dv[1] - p2_dv[1]];

        let divergence = (relative_dv[0].powi(2) + relative_dv[1].powi(2)).sqrt();

        // Normalize by time difference
        let dt = p1_recent.time - p1_past.time;
        if dt > 0.0 {
            Ok(divergence / dt)
        } else {
            Ok(0.0)
        }
    }

    /// Update Lyapunov exponent estimate
    fn update_lyapunov_estimate(&mut self, _current_time: f64) {
        if self.chaos_history.len() < 10 {
            return;
        }

        // Simple Lyapunov estimate from chaos indicator growth
        let recent_window = 10.min(self.chaos_history.len());
        let recent_chaos: Vec<f64> = self
            .chaos_history
            .iter()
            .rev()
            .take(recent_window)
            .cloned()
            .collect();

        if recent_chaos.len() < 2 {
            return;
        }

        // Calculate growth rate
        let initial = recent_chaos[recent_window - 1];
        let final_val = recent_chaos[0];

        if initial > 0.0 && final_val > initial {
            let growth_rate = (final_val / initial).ln() / (recent_window as f64 * 0.01);

            // Exponentially weighted moving average
            let alpha = 0.1;
            self.lyapunov_exponent = alpha * growth_rate + (1.0 - alpha) * self.lyapunov_exponent;
        }
    }

    /// Check if the system is chaotic
    pub fn is_chaotic(&self, threshold: f64) -> bool {
        self.current_chaos_level() > threshold
    }

    /// Get current chaos level
    pub fn current_chaos_level(&self) -> f64 {
        self.chaos_history.last().copied().unwrap_or(0.0)
    }

    /// Detect if system is in a strange attractor
    pub fn detect_strange_attractor(&self) -> bool {
        // Simple heuristic: high chaos but bounded trajectories
        self.lyapunov_exponent > 0.1 && self.max_chaos < 100.0
    }

    /// Get chaos summary
    pub fn summary(&self) -> String {
        format!(
            "Chaos: {:.3} | Lyapunov: {:.3} | Max: {:.3} | Attractor: {}",
            self.current_chaos_level(),
            self.lyapunov_exponent,
            self.max_chaos,
            self.detect_strange_attractor()
        )
    }
}
