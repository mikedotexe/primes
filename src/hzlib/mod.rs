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
//! - `grid_analysis`: CSV/JSON loading and grid analysis for density-explorer output
//! - `verification`: Verification tables comparing sample vs model
//! - `harmonic_overtones`: Fourier analysis of lineouts
//! - `harmonic_lagrange`: Polynomial fitting for lineouts
//! - `symmetry_breaking`: Ridge/trough detection

pub mod crt_patterns;
pub mod density;
pub mod grid_analysis;
pub mod harmonic_lagrange;
pub mod harmonic_overtones;
pub mod hardy_littlewood;
pub mod orthogonality;
pub mod sieve;
pub mod stats;
pub mod symmetry_breaking;
pub mod verification;

// Re-export commonly used types
pub use crt_patterns::{is_double_prime_base, zero_pattern};
pub use density::{Band, BaseAccum};
pub use grid_analysis::{
    enrichment, join_sample_and_model, lineout, load_explain_json, load_model_csv,
    load_sample_csv, Axis, JoinedGrid,
};
pub use hardy_littlewood::{count_pairs_for_n, singular_series_goldbach};
pub use orthogonality::{babylonian_score_60, pairs_index, pearson, singular_series};
pub use sieve::{segmented_sieve, sieve_bool, sieve_primes, sieve_spf};
pub use stats::{linreg, permutation_pvalue, welch_t};
