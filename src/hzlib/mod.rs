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

pub mod crt_patterns;
pub mod density;
pub mod hardy_littlewood;
pub mod sieve;
pub mod stats;

// Re-export commonly used types
pub use crt_patterns::{is_double_prime_base, zero_pattern};
pub use density::{Band, BaseAccum};
pub use hardy_littlewood::{count_pairs_for_n, singular_series_goldbach};
pub use sieve::{segmented_sieve, sieve_bool, sieve_primes, sieve_spf};
pub use stats::{linreg, permutation_pvalue, welch_t};
