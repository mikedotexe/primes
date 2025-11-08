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

pub mod sieve;
pub mod density;
pub mod hardy_littlewood;
pub mod crt_patterns;
pub mod stats;
pub mod orthogonality;

// Re-export commonly used types
pub use sieve::{sieve_bool, sieve_primes, segmented_sieve, sieve_spf};
pub use density::{Band, BaseAccum};
pub use hardy_littlewood::{singular_series_goldbach, count_pairs_for_n};
pub use crt_patterns::{zero_pattern, is_double_prime_base};
pub use stats::{linreg, welch_t, permutation_pvalue};
pub use orthogonality::{babylonian_score_60, singular_series, pairs_index, pearson};
