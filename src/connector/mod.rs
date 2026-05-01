//! # Connector Concatenation Utilities
//!
//! **Layer**: Math core (verified, tested)
//!
//! Efficient u128-based arithmetic for concatenating two fixed primes with
//! variable-length decimal connectors. The module supports fixed-pair
//! directional asymmetry studies and exact enumeration over bounded connector
//! ranges.
//!
//! Arithmetic-first connector vocabulary:
//!
//! - [`analysis::ConnectorHit`] for one fixed width/position/digit/direction
//!   case,
//! - [`analysis::ResidueAdmissible`] for the exact small-modulus filter layer,
//! - [`analysis::ResonancePosition`] for width/position buckets with multiple
//!   working digits in a matched scan,
//! - [`analysis::DirectionalAsymmetry`] for the remaining forward/reverse gap
//!   after those exact filters.
//!
//! The repository still permits "Lagrange point" as a historical alias for a
//! productive insertion position, but this module treats that language as
//! secondary to the arithmetic formulation.
//!
//! # Mathematical Setting
//!
//! For a fixed pair of primes `L` and `R`, the module studies the two families
//!
//! ```text
//! L || C || R
//! R || C || L
//! ```
//!
//! where `C` ranges over decimal connectors of fixed width. The canonical pair
//! `L = 10301`, `R = 3007003007003` is included because it is the repository's
//! maintained empirical test case for directional asymmetry.
//!
//! # Architecture
//!
//! The module is organized into three submodules:
//!
//! - [`types`]: Core types (`ConcatenationSystem`, `Direction`)
//! - [`arithmetic`]: Low-level concatenation functions with overflow checking
//! - [`utils`]: Helper utilities (decimal length, connector ranges, mod-3 filtering)
//!
//! # Complexity and Representation
//!
//! This implementation uses u128 arithmetic throughout, providing significant
//! performance benefits over BigUint-based implementations:
//!
//! - **2-5× faster** concatenation operations
//! - No heap allocations for number construction
//! - Deterministic performance characteristics
//!
//! # Safety
//!
//! All arithmetic operations use checked math and return `Option` types to
//! prevent overflow. The maximum safe total digit count is 38 (since 10^38 < 2^128).
//!
//! For the canonical pair with connector lengths up to 11:
//! - Total digits: 5 (left) + 11 (connector) + 13 (right) = 29
//! - Safety margin: 38 - 29 = 9 digits of headroom
//!
//! # Example Usage
//!
//! ```
//! use primes::connector::ConcatenationSystem;
//!
//! // Create system for canonical Lagrange pair
//! let sys = ConcatenationSystem::new(10301, 3007003007003);
//!
//! // Test a connector: "00006" (5 decimal digits)
//! let connector = 6;
//! let conn_len = 5;
//!
//! // Forward concatenation: 10301 || 00006 || 3007003007003
//! let n_fwd = sys.forward(connector, conn_len).unwrap();
//! assert_eq!(n_fwd, 10301000063007003007003u128);
//!
//! // Reverse concatenation: 3007003007003 || 00006 || 10301
//! let n_rev = sys.reverse(connector, conn_len).unwrap();
//! assert_eq!(n_rev, 30070030070030000610301u128);
//!
//! // Check if result would fit in u128
//! assert!(sys.fits_in_u128(20)); // 5 + 20 + 13 = 38 (max)
//! assert!(!sys.fits_in_u128(21)); // Would overflow
//! ```
//!
//! # Scanning Connectors
//!
//! ```
//! use primes::connector::{ConcatenationSystem, utils};
//!
//! let sys = ConcatenationSystem::new(10301, 3007003007003);
//!
//! // Scan all 5-digit connectors
//! let admissible: Vec<u64> = utils::connector_range(5)
//!     .filter(|&connector| {
//!         !utils::should_skip_mod3(
//!             connector,
//!             utils::CANONICAL_LEFT_MOD3,
//!             utils::CANONICAL_RIGHT_MOD3,
//!         )
//!     })
//!     .take(3)
//!     .collect();
//!
//! assert_eq!(admissible, vec![0, 1, 3]);
//!
//! let samples: Vec<(u128, u128)> = admissible
//!     .iter()
//!     .map(|&connector| {
//!         (
//!             sys.forward(connector as u128, 5).unwrap(),
//!             sys.reverse(connector as u128, 5).unwrap(),
//!         )
//!     })
//!     .collect();
//!
//! assert_eq!(samples.len(), 3);
//! assert!(samples[0].0 < samples[0].1);
//! ```
//!
//! # Current Empirical Use Cases
//!
//! The current repository uses this module for:
//!
//! - fixed-pair forward/reverse prime-count comparisons
//! - modular prefilters such as the mod-3 admissibility test
//! - exhaustive or stratified connector scans at bounded width
//!
//! See [`collab/CONNECTOR_SIGNAL.md`](../../collab/CONNECTOR_SIGNAL.md) for the
//! current claim boundaries and comparison protocol.
//!
//! # Related Tools
//!
//! Exploration tools using these utilities (in `examples/`):
//!
//! - `directional_stats.rs` - Exhaustive connector scans with per-prime elimination tracking
//! - `connector_space_explorer.rs` - Random sampling with 50+ feature metrics
//! - `connector_length_explorer_stratified.rs` - Stratified sampling for large spaces
//!
//! These tools are the repository's current connector-analysis front ends.

pub mod analysis;
pub mod arithmetic;
pub mod types;
pub mod utils;

// Re-export key types at module level for convenience
pub use analysis::{
    canonical_source_hits, scan_single_digit_hits, small_primes_up_to, ConnectorCandidate,
    ConnectorHit, DirectionScanStats, DirectionSignalStats, DirectionalAsymmetry,
    PairResidueProfile, PairScanSummary, PairSignalAudit, PositionSignalRow, ResidueAdmissible,
    ResonancePosition, SmallPrimeProfile, CANONICAL_DOCUMENTED_FORWARD_HITS, CANONICAL_SOURCE_HITS,
    CANONICAL_WIDTH5_HITS, DEFAULT_SMALL_PRIMES,
};
pub use types::{ConcatenationSystem, Direction};

// Re-export commonly used functions
pub use arithmetic::{concat_forward, concat_reverse, pow10};

/// Maximum number of decimal digits that fit in u128
///
/// Since 10^38 < 2^128 < 10^39, we can safely represent numbers
/// up to 38 decimal digits in u128.
pub const MAX_DECIMAL_DIGITS: u32 = 38;

/// Canonical left prime for the maintained directional-asymmetry test pair.
pub const CANONICAL_LEFT: u128 = 10301;

/// Canonical right prime for the maintained directional-asymmetry test pair.
pub const CANONICAL_RIGHT: u128 = 3007003007003;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_canonical_system() {
        let sys = ConcatenationSystem::new(CANONICAL_LEFT, CANONICAL_RIGHT);

        assert_eq!(sys.left, 10301);
        assert_eq!(sys.right, 3007003007003);
        assert_eq!(sys.left_len, 5);
        assert_eq!(sys.right_len, 13);
    }

    #[test]
    fn test_maintained_forward_connector_hits() {
        let sys = ConcatenationSystem::new(CANONICAL_LEFT, CANONICAL_RIGHT);

        // Maintained forward-direction source hits.
        // Position counts use the left-to-right buffer convention.

        // Width=5, position=1, digit=6 -> "06000"
        let n = sys.forward(6000, 5).unwrap();
        assert_eq!(n, 10301060003007003007003u128);

        // Width=5, position=4, digit=6 -> "00006"
        let n = sys.forward(6, 5).unwrap();
        assert_eq!(n, 10301000063007003007003u128);

        // Width=6, position=1, digit=6 -> "060000"
        let n = sys.forward(60000, 6).unwrap();
        assert_eq!(n, 103010600003007003007003u128);

        // Width=6, position=4, digit=6 -> "000060"
        let n = sys.forward(60, 6).unwrap();
        assert_eq!(n, 103010000603007003007003u128);

        // Width=7, position=3, digit=6 -> "0006000"
        let n = sys.forward(6000, 7).unwrap();
        assert_eq!(n, 1030100060003007003007003u128);
    }

    #[test]
    fn test_forward_reverse_asymmetry() {
        let sys = ConcatenationSystem::new(CANONICAL_LEFT, CANONICAL_RIGHT);

        // Same connector, different orders produce different numbers
        let connector = 12345u128;
        let len = 5;

        let fwd = sys.forward(connector, len).unwrap();
        let rev = sys.reverse(connector, len).unwrap();

        // They should be different
        assert_ne!(fwd, rev);

        // Forward: 10301 || 12345 || 3007003007003
        assert_eq!(fwd, 10301123453007003007003u128);

        // Reverse: 3007003007003 || 12345 || 10301
        // = 3007003007003 * 10^10 + 12345 * 10^5 + 10301
        assert_eq!(rev, 30070030070031234510301u128);
    }

    #[test]
    fn test_mod3_filter_canonical_pair() {
        use utils::{should_skip_mod3, CANONICAL_LEFT_MOD3, CANONICAL_RIGHT_MOD3};

        // For canonical pair (both ≡ 2 mod 3), connectors ≡ 2 (mod 3) should be skipped
        assert!(should_skip_mod3(
            2,
            CANONICAL_LEFT_MOD3,
            CANONICAL_RIGHT_MOD3
        ));
        assert!(should_skip_mod3(
            5,
            CANONICAL_LEFT_MOD3,
            CANONICAL_RIGHT_MOD3
        ));
        assert!(should_skip_mod3(
            8,
            CANONICAL_LEFT_MOD3,
            CANONICAL_RIGHT_MOD3
        ));

        // Connectors ≡ 0 or 1 (mod 3) should not be skipped
        assert!(!should_skip_mod3(
            0,
            CANONICAL_LEFT_MOD3,
            CANONICAL_RIGHT_MOD3
        ));
        assert!(!should_skip_mod3(
            1,
            CANONICAL_LEFT_MOD3,
            CANONICAL_RIGHT_MOD3
        ));
        assert!(!should_skip_mod3(
            3,
            CANONICAL_LEFT_MOD3,
            CANONICAL_RIGHT_MOD3
        ));
        assert!(!should_skip_mod3(
            4,
            CANONICAL_LEFT_MOD3,
            CANONICAL_RIGHT_MOD3
        ));
    }

    #[test]
    fn test_overflow_boundary() {
        let sys = ConcatenationSystem::new(CANONICAL_LEFT, CANONICAL_RIGHT);

        // Max safe: 5 + 20 + 13 = 38 digits
        let max_safe = sys.forward(0, 20);
        assert!(max_safe.is_some());

        // Just over: 5 + 21 + 13 = 39 digits
        let overflow = sys.forward(0, 21);
        assert!(overflow.is_none());
    }
}
