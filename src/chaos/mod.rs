//! Chaos theory analysis for prime dynamics

pub mod config_particles;
pub mod lyapunov;
pub mod nbody_lyapunov;
pub mod variance_tracker;

pub use config_particles::{
    estimate_chaos_level, generate_chaos_test_particles, generate_test_particles,
};
pub use lyapunov::{ChaosMap, LyapunovCalculator};
pub use nbody_lyapunov::{ChaosPrimeMetrics, ConfigurationChaosAnalyzer, NBodyLyapunov};
pub use variance_tracker::{VarianceReport, VarianceTracker};

/// Chaos metrics for analyzing system behavior
#[derive(Debug, Clone)]
pub struct ChaosMetrics {
    pub lyapunov_exponent: f64,
    pub energy_drift: f64,
    pub max_velocity: f64,
    pub trajectory_divergence: f64,
}

impl ChaosMetrics {
    pub fn is_chaotic(&self) -> bool {
        self.lyapunov_exponent > 0.01
    }

    pub fn chaos_level(&self) -> &'static str {
        match self.lyapunov_exponent {
            l if l > 0.5 => "EXTREME CHAOS",
            l if l > 0.3 => "CHAOS STORM",
            l if l > 0.1 => "Chaotic",
            l if l > 0.01 => "Edge of chaos",
            l if l > -0.01 => "Marginally stable",
            _ => "Stable",
        }
    }
}
