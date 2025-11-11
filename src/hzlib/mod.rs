//! Hardy-Littlewood Framework for Prime Analysis
//!
//! This module provides mathematical tools for analyzing prime distribution
//! through the lens of the Hardy-Littlewood conjecture and related number-theoretic
//! frameworks. It complements our membrane prime research by providing rigorous
//! statistical foundations.
//!
//! # Modules
//!
//! - `sieve`: Optimized prime sieves (segmented, SPF computation)
//! - `density`: Band-based density analysis around midpoints
//! - `hardy_littlewood`: Singular series and HL normalization
//! - `crt_patterns`: Chinese Remainder Theorem pattern detection
//! - `stats`: Statistical tests (Welch t, permutation, regression)
//! - `orthogonality`: Babylonian-Prime divergence analysis (human vs nature math)
//! - `symmetry_breaking`: Seed-level failure pattern analysis (where patterns break)
//! - `harmonic_overtones`: Harmonic resonance in multiples of successful bases

pub mod crt_patterns;
pub mod density;
pub mod hardy_littlewood;
pub mod harmonic_overtones;
pub mod orthogonality;
pub mod sieve;
pub mod stats;
pub mod symmetry_breaking;

// Re-export commonly used types
pub use crt_patterns::{is_double_prime_base, zero_pattern};
pub use density::{Band, BaseAccum};
pub use hardy_littlewood::{count_pairs_for_n, singular_series_goldbach};
pub use harmonic_overtones::{ConfigurationHarmonic, HarmonicAccumulator, HarmonicSeries};
pub use orthogonality::{babylonian_score_60, pairs_index, pearson, singular_series};
pub use sieve::{segmented_sieve, sieve_bool, sieve_primes, sieve_spf};
pub use stats::{linreg, linreg_with_ci, permutation_pvalue, welch_t};
pub use symmetry_breaking::{BreakingAccumulator, SymmetryBreaker};
