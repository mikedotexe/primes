//! Utility functions for connector analysis.
//!
//! These helpers are intentionally small, but they sit on the hot path for
//! connector-space scans. They let doctests show the same workflow that the
//! larger examples use:
//! 1. choose a connector length,
//! 2. iterate all connectors of that fixed decimal width,
//! 3. discard obvious composites with a cheap residue-class filter.
//!
//! # Quick Example
//! ```
//! use primes::connector::utils::{
//!     CANONICAL_LEFT_MOD3, CANONICAL_RIGHT_MOD3, connector_range, decimal_len, should_skip_mod3,
//! };
//!
//! let survivors: Vec<u64> = connector_range(2)
//!     .filter(|&c| !should_skip_mod3(c, CANONICAL_LEFT_MOD3, CANONICAL_RIGHT_MOD3))
//!     .take(5)
//!     .collect();
//!
//! assert_eq!(decimal_len(3007003007003), 13);
//! assert_eq!(survivors, vec![0, 1, 3, 4, 6]);
//! ```

use std::ops::Range;

/// Compute the number of decimal digits in a u128 value
///
/// # Arguments
/// * `n` - The number to measure
///
/// # Returns
/// The number of decimal digits (0 returns 1)
///
/// # Example
/// ```
/// use primes::connector::utils::decimal_len;
///
/// assert_eq!(decimal_len(0), 1);
/// assert_eq!(decimal_len(7), 1);
/// assert_eq!(decimal_len(42), 2);
/// assert_eq!(decimal_len(10301), 5);
/// assert_eq!(decimal_len(3007003007003), 13);
/// ```
pub fn decimal_len(mut n: u128) -> u32 {
    if n == 0 {
        return 1;
    }

    let mut len = 0;
    while n > 0 {
        len += 1;
        n /= 10;
    }
    len
}

/// Generate a range of all possible connectors for a given decimal length
///
/// For length `len`, returns the range `[0, 10^len)`.
///
/// # Arguments
/// * `len` - Number of decimal digits in the connector
///
/// # Returns
/// A `Range<u64>` from 0 to 10^len (exclusive)
///
/// # Panics
/// Panics if `len > 18` (would overflow u64)
///
/// # Example
/// ```
/// use primes::connector::utils::connector_range;
///
/// let range = connector_range(3);
/// assert_eq!(range.start, 0);
/// assert_eq!(range.end, 1000); // 10^3
///
/// // Iterate over all 3-digit connectors (including leading zeros)
/// let mut count = 0;
/// for _c in connector_range(3) {
///     count += 1;
/// }
/// assert_eq!(count, 1000);
///
/// // Leading-zero connectors are represented by small integers.
/// let first_four: Vec<u64> = connector_range(2).take(4).collect();
/// assert_eq!(first_four, vec![0, 1, 2, 3]); // 00, 01, 02, 03
/// ```
pub fn connector_range(len: u32) -> Range<u64> {
    assert!(
        len <= 18,
        "connector_range: length {} exceeds u64 capacity (max 18)",
        len
    );

    let max = 10u64.pow(len);
    0..max
}

/// Check if a connector should be skipped due to mod-3 divisibility
///
/// For a concatenated number N = L || C || R, where:
/// - L ≡ left_mod3 (mod 3)
/// - R ≡ right_mod3 (mod 3)
/// - C ≡ connector % 3 (mod 3)
///
/// The full number N will be divisible by 3 if:
/// N ≡ (left_mod3 + connector_mod3 + right_mod3) ≡ 0 (mod 3)
///
/// This function returns true if the connector would make N divisible by 3.
///
/// # Arguments
/// * `connector` - The connector value
/// * `left_mod3` - L mod 3 (must be 0, 1, or 2)
/// * `right_mod3` - R mod 3 (must be 0, 1, or 2)
///
/// # Returns
/// `true` if the concatenation would be divisible by 3 (and thus composite)
///
/// # Example
/// ```
/// use primes::connector::utils::should_skip_mod3;
///
/// // For the canonical pair: 10301 ≡ 2 (mod 3), 3007003007003 ≡ 2 (mod 3)
/// // If connector ≡ 2 (mod 3), then N ≡ 2+2+2 ≡ 0 (mod 3) → composite
/// assert!(should_skip_mod3(2, 2, 2));   // Skip: 2+2+2 = 6 ≡ 0 (mod 3)
/// assert!(should_skip_mod3(5, 2, 2));   // Skip: 2+2+2 = 6 ≡ 0 (mod 3)
/// assert!(!should_skip_mod3(0, 2, 2));  // Keep: 2+0+2 = 4 ≡ 1 (mod 3)
/// assert!(!should_skip_mod3(1, 2, 2));  // Keep: 2+1+2 = 5 ≡ 2 (mod 3)
///
/// let kept: Vec<u64> = (0..6)
///     .filter(|&c| !should_skip_mod3(c, 2, 2))
///     .collect();
/// assert_eq!(kept, vec![0, 1, 3, 4]);
/// ```
pub fn should_skip_mod3(connector: u64, left_mod3: u8, right_mod3: u8) -> bool {
    debug_assert!(left_mod3 < 3, "left_mod3 must be 0, 1, or 2");
    debug_assert!(right_mod3 < 3, "right_mod3 must be 0, 1, or 2");

    let connector_mod3 = (connector % 3) as u8;
    let sum_mod3 = (left_mod3 + connector_mod3 + right_mod3) % 3;

    sum_mod3 == 0
}

/// Precomputed mod-3 values for canonical Lagrange pair
pub const CANONICAL_LEFT_MOD3: u8 = 2; // 10301 % 3 = 2
pub const CANONICAL_RIGHT_MOD3: u8 = 2; // 3007003007003 % 3 = 2

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_len() {
        assert_eq!(decimal_len(0), 1);
        assert_eq!(decimal_len(1), 1);
        assert_eq!(decimal_len(9), 1);
        assert_eq!(decimal_len(10), 2);
        assert_eq!(decimal_len(99), 2);
        assert_eq!(decimal_len(100), 3);
        assert_eq!(decimal_len(10301), 5);
        assert_eq!(decimal_len(3007003007003), 13);
    }

    #[test]
    fn test_decimal_len_large() {
        // 10^38 - 1 (maximum for u128)
        let max_38_digits = 10u128.pow(38) - 1;
        assert_eq!(decimal_len(max_38_digits), 38);

        // 10^38 exactly
        let exactly_39_digits = 10u128.pow(38);
        assert_eq!(decimal_len(exactly_39_digits), 39);
    }

    #[test]
    fn test_connector_range_small() {
        let range = connector_range(0);
        assert_eq!(range.start, 0);
        assert_eq!(range.end, 1); // Just 0

        let range = connector_range(1);
        assert_eq!(range.start, 0);
        assert_eq!(range.end, 10); // 0..9

        let range = connector_range(3);
        assert_eq!(range.start, 0);
        assert_eq!(range.end, 1000); // 0..999
    }

    #[test]
    fn test_connector_range_count() {
        assert_eq!(connector_range(5).count(), 100_000);
        assert_eq!(connector_range(7).count(), 10_000_000);
    }

    #[test]
    #[should_panic(expected = "exceeds u64 capacity")]
    fn test_connector_range_overflow() {
        // This should panic because 10^19 > u64::MAX
        connector_range(19);
    }

    #[test]
    fn test_should_skip_mod3_canonical_pair() {
        // For canonical pair: both primes ≡ 2 (mod 3)
        // Skip when connector ≡ 2 (mod 3): 2+2+2 = 6 ≡ 0 (mod 3)
        assert!(should_skip_mod3(2, 2, 2));
        assert!(should_skip_mod3(5, 2, 2));
        assert!(should_skip_mod3(8, 2, 2));

        // Keep when connector ≡ 0 or 1 (mod 3)
        assert!(!should_skip_mod3(0, 2, 2)); // 2+0+2 = 4 ≡ 1
        assert!(!should_skip_mod3(1, 2, 2)); // 2+1+2 = 5 ≡ 2
        assert!(!should_skip_mod3(3, 2, 2)); // 2+0+2 = 4 ≡ 1
        assert!(!should_skip_mod3(4, 2, 2)); // 2+1+2 = 5 ≡ 2
    }

    #[test]
    fn test_should_skip_mod3_different_pairs() {
        // Pair where L ≡ 1, R ≡ 1 (mod 3)
        // Skip when C ≡ 1 (mod 3): 1+1+1 = 3 ≡ 0
        assert!(should_skip_mod3(1, 1, 1));
        assert!(should_skip_mod3(4, 1, 1));
        assert!(!should_skip_mod3(0, 1, 1)); // 1+0+1 = 2
        assert!(!should_skip_mod3(2, 1, 1)); // 1+2+1 = 4 ≡ 1
    }

    #[test]
    fn test_canonical_constants() {
        assert_eq!(10301 % 3, CANONICAL_LEFT_MOD3 as u128);
        assert_eq!(3007003007003 % 3, CANONICAL_RIGHT_MOD3 as u128);
    }
}
