//! Prelude module for convenient imports
//!
//! This module re-exports the most commonly used types and functions
//! from the `primes` crate.

// Core types
pub use crate::{is_prime, MembraneConfig, PhysicsError, PhysicsResult};

// Prime sieve utilities
pub use crate::prime_sieve::{
    segmented_sieve, sieve_count_and_time, warm_slc, BitSieve, WarmResult,
};

// Performance monitoring
pub use crate::performance::{CycleTimer, PerfMetrics, PerfMonitor};

// Membrane construction
pub use crate::membrane::{ConstructionType, MembraneBuilder, OptimizationTarget};

// Gravity simulation
pub use crate::gravity::{ForceCalculator, PrimeParticle};

// Universe types
pub use crate::PrimeUniverse;

// Educational tools
pub use crate::education::BaseMetricEducation;

// Phase 4 (when enabled)
#[cfg(feature = "phase4")]
pub use crate::phase4::{predict_sme_padded_safe, OnChipRL, PmuDoubleBuffer, PmuSnapshot};
