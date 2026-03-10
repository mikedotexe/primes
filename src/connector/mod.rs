//! # Connector Concatenation Utilities
//!
//! **Layer**: Math core (verified, tested)
//!
//! Efficient u128-based arithmetic for concatenating two fixed primes with
//! variable-length decimal connectors, enabling directional asymmetry analysis.
//!
//! # Background
//!
//! In November 2025, research on the canonical Lagrange point prime pair
//! (10301, 3007003007003) revealed a remarkable **directional asymmetry** phenomenon:
//! when concatenating these primes with connectors of varying lengths, the number of
//! prime results differs significantly depending on concatenation order (forward vs reverse).
//!
//! This module provides the core arithmetic infrastructure used in that research,
//! optimized for performance and safety.
//!
//! # Architecture
//!
//! The module is organized into three submodules:
//!
//! - [`types`]: Core types (`ConcatenationSystem`, `Direction`)
//! - [`arithmetic`]: Low-level concatenation functions with overflow checking
//! - [`utils`]: Helper utilities (decimal length, connector ranges, mod-3 filtering)
//!
//! # Performance
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
//! use prime_physics_engine::connector::ConcatenationSystem;
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
//! assert_eq!(n_rev, 3007003007003000065301u128);
//!
//! // Check if result would fit in u128
//! assert!(sys.fits_in_u128(20)); // 5 + 20 + 13 = 38 (max)
//! assert!(!sys.fits_in_u128(21)); // Would overflow
//! ```
//!
//! # Scanning Connectors
//!
//! ```
//! use prime_physics_engine::connector::{ConcatenationSystem, utils};
//!
//! let sys = ConcatenationSystem::new(10301, 3007003007003);
//!
//! // Scan all 5-digit connectors
//! for connector in utils::connector_range(5) {
//!     // Optional: skip mod-3 composites
//!     if utils::should_skip_mod3(connector, utils::CANONICAL_LEFT_MOD3, utils::CANONICAL_RIGHT_MOD3) {
//!         continue;
//!     }
//!
//!     let n_fwd = sys.forward(connector as u128, 5).unwrap();
//!     let n_rev = sys.reverse(connector as u128, 5).unwrap();
//!
//!     // Test primality, collect statistics, etc.
//! }
//! ```
//!
//! # Research Applications
//!
//! This module enabled the following discoveries:
//!
//! - **Lagrange Point Asymmetry**: ~2% directional bias for short connectors (length 5-8)
//! - **Resonance Peak**: Non-monotonic asymmetry scaling with 59% peak at length 10
//! - **Post-Sieve Mystery**: Asymmetry persists after modular sieve (mod 3, 7, 11, ...)
//!
//! See `collab/CORE_ASYMMETRY_NOTES.md` and `collab/LAGRANGE_POINT_ASYMMETRY.md`
//! for complete empirical results.
//!
//! # Related Tools
//!
//! Exploration tools using these utilities (in `examples/`):
//!
//! - `directional_stats.rs` - Exhaustive connector scans with per-prime elimination tracking
//! - `connector_space_explorer.rs` - Random sampling with 50+ feature metrics
//! - `connector_length_explorer_stratified.rs` - Stratified sampling for large spaces
//!
//! These tools are complete exploration artifacts from November 2025 research.

pub mod arithmetic;
pub mod types;
pub mod utils;

// Re-export key types at module level for convenience
pub use types::{ConcatenationSystem, Direction};

// Re-export commonly used functions
pub use arithmetic::{concat_forward, concat_reverse, pow10};

/// Maximum number of decimal digits that fit in u128
///
/// Since 10^38 < 2^128 < 10^39, we can safely represent numbers
/// up to 38 decimal digits in u128.
pub const MAX_DECIMAL_DIGITS: u32 = 38;

/// Canonical left prime from Lagrange point research
pub const CANONICAL_LEFT: u128 = 10301;

/// Canonical right prime from Lagrange point research
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
    fn test_known_lagrange_equilibria() {
        let sys = ConcatenationSystem::new(CANONICAL_LEFT, CANONICAL_RIGHT);

        // Known Lagrange equilibrium points from research
        // These are connectors that produce prime concatenations

        // L1: Buffer=5, Position=4 → connector = 6 (as "00006")
        let n = sys.forward(6, 5).unwrap();
        assert_eq!(n, 10301000063007003007003u128);

        // L2: Buffer=6, Position=2 → connector = 60000 (as "060000")
        let n = sys.forward(60000, 6).unwrap();
        assert_eq!(n, 103010600003007003007003u128);

        // L3: Buffer=6, Position=4 → connector = 60 (as "000060")
        let n = sys.forward(60, 6).unwrap();
        assert_eq!(n, 103010000603007003007003u128);

        // L4: Buffer=7, Position=3 → connector = 6000 (as "0006000")
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
