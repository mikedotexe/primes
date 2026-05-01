//! Core types for connector concatenation systems

use serde::{Deserialize, Serialize};

/// Direction of concatenation between two primes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Direction {
    /// Forward: Left || Connector || Right
    Forward,
    /// Reverse: Right || Connector || Left
    Reverse,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Forward => write!(f, "forward"),
            Self::Reverse => write!(f, "reverse"),
        }
    }
}

/// A system for concatenating two fixed primes with variable connectors
///
/// # Example
/// ```
/// use primes::connector::ConcatenationSystem;
///
/// // Canonical connector pair
/// let sys = ConcatenationSystem::new(10301, 3007003007003);
///
/// // Forward: 10301 || 00006 || 3007003007003
/// let n = sys.forward(6, 5).unwrap();
/// assert_eq!(n, 10301000063007003007003u128);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConcatenationSystem {
    /// Left prime value
    pub left: u128,
    /// Right prime value
    pub right: u128,
    /// Number of decimal digits in left prime
    pub left_len: u32,
    /// Number of decimal digits in right prime
    pub right_len: u32,
}

impl ConcatenationSystem {
    /// Create a new concatenation system from two prime values
    ///
    /// Automatically computes the decimal length of each prime.
    ///
    /// # Example
    /// ```
    /// use primes::connector::ConcatenationSystem;
    ///
    /// let sys = ConcatenationSystem::new(10301, 3007003007003);
    /// assert_eq!(sys.left_len, 5);
    /// assert_eq!(sys.right_len, 13);
    /// ```
    pub fn new(left: u128, right: u128) -> Self {
        Self {
            left,
            right,
            left_len: crate::connector::utils::decimal_len(left),
            right_len: crate::connector::utils::decimal_len(right),
        }
    }

    /// Concatenate in forward direction: Left || Connector || Right
    ///
    /// Computes: `left · 10^(conn_len + right_len) + connector · 10^right_len + right`
    ///
    /// # Arguments
    /// * `connector` - The decimal value to insert between primes
    /// * `conn_len` - The number of decimal digits in the connector (with leading zeros)
    ///
    /// # Returns
    /// `Some(result)` if the concatenation fits in u128, `None` on overflow
    ///
    /// # Example
    /// ```
    /// use primes::connector::ConcatenationSystem;
    ///
    /// let sys = ConcatenationSystem::new(10301, 3007003007003);
    ///
    /// // Connector "00006" (5 digits)
    /// let result = sys.forward(6, 5).unwrap();
    /// assert_eq!(result, 10301000063007003007003u128);
    /// ```
    pub fn forward(&self, connector: u128, conn_len: u32) -> Option<u128> {
        crate::connector::arithmetic::concat_forward(
            self.left,
            self.right,
            connector,
            self.left_len,
            self.right_len,
            conn_len,
        )
    }

    /// Concatenate in reverse direction: Right || Connector || Left
    ///
    /// Computes: `right · 10^(conn_len + left_len) + connector · 10^left_len + left`
    ///
    /// # Arguments
    /// * `connector` - The decimal value to insert between primes
    /// * `conn_len` - The number of decimal digits in the connector (with leading zeros)
    ///
    /// # Returns
    /// `Some(result)` if the concatenation fits in u128, `None` on overflow
    ///
    /// # Example
    /// ```
    /// use primes::connector::ConcatenationSystem;
    ///
    /// let sys = ConcatenationSystem::new(10301, 3007003007003);
    ///
    /// // Connector "00006" (5 digits)
    /// let result = sys.reverse(6, 5).unwrap();
    /// assert_eq!(result, 30070030070030000610301u128);
    /// ```
    pub fn reverse(&self, connector: u128, conn_len: u32) -> Option<u128> {
        crate::connector::arithmetic::concat_reverse(
            self.left,
            self.right,
            connector,
            self.left_len,
            self.right_len,
            conn_len,
        )
    }

    /// Get the total number of decimal digits in the concatenated result
    ///
    /// Returns `left_len + conn_len + right_len`
    pub fn total_digits(&self, conn_len: u32) -> u32 {
        self.left_len
            .saturating_add(conn_len)
            .saturating_add(self.right_len)
    }

    /// Check if a concatenation with given connector length would fit in u128
    ///
    /// u128 can hold up to 38 decimal digits (10^38 < 2^128 < 10^39)
    pub fn fits_in_u128(&self, conn_len: u32) -> bool {
        self.total_digits(conn_len) <= crate::connector::MAX_DECIMAL_DIGITS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_system_creation() {
        let sys = ConcatenationSystem::new(10301, 3007003007003);
        assert_eq!(sys.left, 10301);
        assert_eq!(sys.right, 3007003007003);
        assert_eq!(sys.left_len, 5);
        assert_eq!(sys.right_len, 13);
    }

    #[test]
    fn test_total_digits() {
        let sys = ConcatenationSystem::new(10301, 3007003007003);
        assert_eq!(sys.total_digits(5), 23); // 5 + 5 + 13
        assert_eq!(sys.total_digits(11), 29); // 5 + 11 + 13
    }

    #[test]
    fn test_fits_in_u128() {
        let sys = ConcatenationSystem::new(10301, 3007003007003);
        assert!(sys.fits_in_u128(5));
        assert!(sys.fits_in_u128(11));
        assert!(sys.fits_in_u128(20)); // 5 + 20 + 13 = 38 (max)
        assert!(!sys.fits_in_u128(21)); // 5 + 21 + 13 = 39 (overflow)
    }
}
