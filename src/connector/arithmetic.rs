//! Arithmetic operations for connector concatenation
//!
//! All operations use u128 and return Option to handle overflow safely.

use super::MAX_DECIMAL_DIGITS;

/// Precomputed powers of 10 from 10^0 to 10^38
///
/// This table enables O(1) lookup for pow10 operations
static POW10_TABLE: [u128; (MAX_DECIMAL_DIGITS + 1) as usize] = {
    let mut arr = [0u128; (MAX_DECIMAL_DIGITS + 1) as usize];
    arr[0] = 1;
    let mut i = 1;
    while i <= MAX_DECIMAL_DIGITS as usize {
        arr[i] = arr[i - 1] * 10;
        i += 1;
    }
    arr
};

/// Compute 10^exp using precomputed table
///
/// # Arguments
/// * `exp` - Exponent (must be <= 38)
///
/// # Returns
/// `Some(10^exp)` if exp <= 38, `None` if exp > 38
///
/// # Example
/// ```
/// use primes::connector::arithmetic::pow10;
///
/// assert_eq!(pow10(0), Some(1));
/// assert_eq!(pow10(3), Some(1000));
/// assert_eq!(pow10(13), Some(10_000_000_000_000));
/// assert_eq!(pow10(39), None); // exceeds u128
/// ```
pub fn pow10(exp: u32) -> Option<u128> {
    if exp > MAX_DECIMAL_DIGITS {
        return None;
    }
    Some(POW10_TABLE[exp as usize])
}

/// Concatenate in forward direction: left || connector || right
///
/// Computes: `left · 10^(conn_len + right_len) + connector · 10^right_len + right`
///
/// # Arguments
/// * `left` - Left prime value
/// * `right` - Right prime value
/// * `connector` - Connector value (interpreted as decimal with `conn_len` digits)
/// * `left_len` - Number of decimal digits in left
/// * `right_len` - Number of decimal digits in right
/// * `conn_len` - Number of decimal digits in connector (including leading zeros)
///
/// # Returns
/// `Some(result)` if concatenation fits in u128, `None` on overflow
///
/// # Safety
/// Uses checked arithmetic throughout to prevent overflow
///
/// # Example
/// ```
/// use primes::connector::arithmetic::concat_forward;
///
/// // 10301 || 00006 || 3007003007003
/// let result = concat_forward(10301, 3007003007003, 6, 5, 13, 5).unwrap();
/// assert_eq!(result, 10301000063007003007003u128);
/// ```
pub fn concat_forward(
    left: u128,
    right: u128,
    connector: u128,
    left_len: u32,
    right_len: u32,
    conn_len: u32,
) -> Option<u128> {
    // Check total digits don't exceed limit
    let total_digits = left_len.checked_add(conn_len)?.checked_add(right_len)?;

    if total_digits > MAX_DECIMAL_DIGITS {
        return None;
    }

    // Compute powers
    let pow_right = pow10(right_len)?;
    let pow_conn_right = pow10(conn_len.checked_add(right_len)?)?;

    // Compute parts with overflow checking
    let left_part = left.checked_mul(pow_conn_right)?;
    let conn_part = connector.checked_mul(pow_right)?;

    // Sum parts
    left_part.checked_add(conn_part)?.checked_add(right)
}

/// Concatenate in reverse direction: right || connector || left
///
/// Computes: `right · 10^(conn_len + left_len) + connector · 10^left_len + left`
///
/// # Arguments
/// * `left` - Left prime value
/// * `right` - Right prime value
/// * `connector` - Connector value (interpreted as decimal with `conn_len` digits)
/// * `left_len` - Number of decimal digits in left
/// * `right_len` - Number of decimal digits in right
/// * `conn_len` - Number of decimal digits in connector (including leading zeros)
///
/// # Returns
/// `Some(result)` if concatenation fits in u128, `None` on overflow
///
/// # Safety
/// Uses checked arithmetic throughout to prevent overflow
///
/// # Example
/// ```
/// use primes::connector::arithmetic::concat_reverse;
///
/// // 3007003007003 || 00006 || 10301
/// let result = concat_reverse(10301, 3007003007003, 6, 5, 13, 5).unwrap();
/// assert_eq!(result, 30070030070030000610301u128);
/// ```
pub fn concat_reverse(
    left: u128,
    right: u128,
    connector: u128,
    left_len: u32,
    right_len: u32,
    conn_len: u32,
) -> Option<u128> {
    // Check total digits don't exceed limit
    let total_digits = left_len.checked_add(conn_len)?.checked_add(right_len)?;

    if total_digits > MAX_DECIMAL_DIGITS {
        return None;
    }

    // Compute powers
    let pow_left = pow10(left_len)?;
    let pow_conn_left = pow10(conn_len.checked_add(left_len)?)?;

    // Compute parts with overflow checking
    let right_part = right.checked_mul(pow_conn_left)?;
    let conn_part = connector.checked_mul(pow_left)?;

    // Sum parts
    right_part.checked_add(conn_part)?.checked_add(left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow10_table() {
        assert_eq!(pow10(0), Some(1));
        assert_eq!(pow10(1), Some(10));
        assert_eq!(pow10(2), Some(100));
        assert_eq!(pow10(3), Some(1_000));
        assert_eq!(pow10(5), Some(100_000));
        assert_eq!(pow10(13), Some(10_000_000_000_000));
        assert_eq!(
            pow10(38),
            Some(100_000_000_000_000_000_000_000_000_000_000_000_000)
        );
        assert_eq!(pow10(39), None);
    }

    #[test]
    fn test_concat_forward_canonical() {
        // 10301 || 00006 || 3007003007003 = 10301000063007003007003
        let result = concat_forward(10301, 3007003007003, 6, 5, 13, 5).unwrap();
        assert_eq!(result, 10301000063007003007003u128);
    }

    #[test]
    fn test_concat_reverse_canonical() {
        // 3007003007003 || 00006 || 10301 = 30070030070030000610301
        // Calculation: 3007003007003 * 10^10 + 6 * 10^5 + 10301
        //            = 30070030070030000000000 + 600000 + 10301
        let result = concat_reverse(10301, 3007003007003, 6, 5, 13, 5).unwrap();
        assert_eq!(result, 30070030070030000610301u128);
    }

    #[test]
    fn test_concat_forward_different_lengths() {
        // 10301 || 1234567890 || 3007003007003 (connector length 10)
        let result = concat_forward(10301, 3007003007003, 1234567890, 5, 13, 10).unwrap();

        // Manual calculation:
        // 10301 * 10^23 + 1234567890 * 10^13 + 3007003007003
        let expected = 10301u128 * pow10(23).unwrap()
            + 1234567890u128 * pow10(13).unwrap()
            + 3007003007003u128;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_concat_overflow_detection() {
        // Try to create a number with 39 digits (should fail)
        let result = concat_forward(10301, 3007003007003, 0, 5, 13, 21);
        assert_eq!(result, None);
    }

    #[test]
    fn test_concat_zero_connector() {
        // 10301 || 00000 || 3007003007003
        let result = concat_forward(10301, 3007003007003, 0, 5, 13, 5).unwrap();
        // Should be: 10301 * 10^18 + 0 * 10^13 + 3007003007003
        let expected = 10301u128 * pow10(18).unwrap() + 3007003007003;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_lagrange_equilibrium_l1() {
        // Buffer=5, Position=4: connector = 6 * 10^(5-4) = 60
        // But with length 5, this is "00060"
        let connector = 6 * pow10(5 - 4).unwrap();
        let result = concat_forward(10301, 3007003007003, connector, 5, 13, 5).unwrap();
        assert_eq!(result, 10301000603007003007003u128);
    }

    #[test]
    fn test_lagrange_equilibrium_l2() {
        // Buffer=6, Position=2: connector = 6 * 10^(6-2) = 60000
        let connector = 6 * pow10(6 - 2).unwrap();
        let result = concat_forward(10301, 3007003007003, connector, 5, 13, 6).unwrap();
        // 10301 || 060000 || 3007003007003
        // = 10301 * 10^19 + 60000 * 10^13 + 3007003007003
        assert_eq!(result, 103010600003007003007003u128);
    }
}
