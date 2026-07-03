//! # Validation and Baseline Comparison
//!
//! **Layer**: Math core (testing infrastructure)
//!
//! Provides random baseline generation and statistical significance testing
//! for comparing membrane prime density against chance. Complements the
//! statistical tools in [`crate::hzlib::stats`].

pub mod affine_hinge;
pub mod affine_period_lock;
pub mod affine_period_lock_species;
pub mod affine_phase_residual;
pub mod affine_singular_series;
pub mod base30_reversal;
pub mod base30_wheel;
pub mod base57_affine_codec;
pub mod bounded_k;
pub mod connector_signal;
pub mod construction_density;
pub mod exhaustive_tracker;
pub mod failure_analysis;
pub mod fast_affine;
pub mod hinge_atoms;
pub mod hinge_robustness;
pub mod large_affine_witness;
pub mod matched_control;
pub mod matched_control_residue_masks;
pub mod metal_affine;
pub mod proof_build_observatory;
pub mod random_baseline;
pub mod reporting;
pub mod seed_to_witness;
pub mod signal_catalog;
pub mod statistical_tests;
pub mod timestamp_seed_policy;
pub mod unit_cycle_neighbors;
pub mod unit_cycle_phase;

use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

/// Results from comparing our method to random baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Our method's success rate
    pub method_success_rate: f64,

    /// Random baseline success rate
    pub random_success_rate: f64,

    /// Improvement factor (method/random)
    pub improvement_factor: f64,

    /// Statistical significance (p-value)
    pub p_value: f64,

    /// Confidence interval for success rate
    pub confidence_interval: (f64, f64),

    /// Sample size used
    pub sample_size: usize,

    /// Chi-square test statistic
    pub chi_square: f64,

    /// Detailed breakdown by configuration
    pub configuration_analysis: Vec<ConfigResult>,
}

/// Results for a specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigResult {
    /// Configuration description
    pub config_description: String,

    /// Number of primes found
    pub primes_found: usize,

    /// Total candidates tested
    pub candidates_tested: usize,

    /// Success rate
    pub success_rate: f64,

    /// Expected random rate
    pub expected_random_rate: f64,

    /// Specific primes discovered (first 10)
    pub example_primes: Vec<BigUint>,
}

/// Validation context maintaining state across tests
#[derive(Debug, Clone)]
pub struct ValidationContext {
    /// Random number generator
    pub rng: rand::rngs::StdRng,

    /// Whether to print verbose output
    pub verbose: bool,

    /// Statistical confidence level (e.g., 0.95)
    pub confidence_level: f64,

    /// Number of bootstrap iterations
    pub bootstrap_iterations: usize,
}

impl Default for ValidationContext {
    fn default() -> Self {
        use rand::SeedableRng;

        Self {
            rng: rand::rngs::StdRng::seed_from_u64(42),
            verbose: true,
            confidence_level: 0.95,
            bootstrap_iterations: 10000,
        }
    }
}
