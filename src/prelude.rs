//! Prelude module for convenient imports
//! 
//! This module re-exports the most commonly used types and functions
//! from the prime physics engine.

// Core types
pub use crate::{
    PhysicsResult, 
    PhysicsError,
    MembraneConfig,
    is_prime,
};

// Prime sieve utilities
pub use crate::prime_sieve::{
    BitSieve,
    segmented_sieve,
    warm_slc,
    sieve_count_and_time,
    WarmResult,
};

// Performance monitoring
pub use crate::performance::{
    PerfMonitor,
    PerfMetrics,
    CycleTimer,
};

// Membrane construction
pub use crate::membrane::{
    MembraneBuilder,
    ConstructionType,
    OptimizationTarget,
};

// Gravity simulation
pub use crate::gravity::{
    PrimeParticle,
    ForceCalculator,
};

// Universe types
pub use crate::PrimeUniverse;

// Educational tools
pub use crate::education::BaseMetricEducation;

// Phase 4 (when enabled)
#[cfg(feature = "phase4")]
pub use crate::phase4::{
    predict_sme_padded_safe,
    OnChipRL,
    PmuDoubleBuffer,
    PmuSnapshot,
};