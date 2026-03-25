//! Chinese Remainder Theorem Pattern Detection
//!
//! Analyzes which small primes divide the midpoint residue `base / 2` in order
//! to classify local CRT patterns such as `(3,11)` and `(5,7)`.
//!
//! # Quick Example
//! ```
//! use primes::hzlib::crt_patterns::{classify_base, zero_pattern};
//!
//! let small = [3, 5, 7, 11];
//! let (zeros, count, _tag, comp) = zero_pattern(70, &small);
//!
//! assert_eq!(zeros, vec![5, 7]);
//! assert_eq!(count, 2);
//! assert!(comp);
//! assert_eq!(classify_base(66, &small), "complementary_3_and_11");
//! ```

/// Identify which small primes divide `base / 2`.
///
/// This function checks which small primes divide the midpoint residue and
/// records the resulting local pattern.
///
/// # Arguments
/// * `base` - The base to analyze (must be even)
/// * `small` - List of small primes to check (typically [3, 5, 7, 11])
///
/// # Returns
/// Tuple of (zeros, count, tag, is_complementary):
/// - `zeros`: List of small primes dividing base/2
/// - `count`: Number of such primes
/// - `tag`: String identifier like "3_and_11" or "only_5"
/// - `is_complementary`: True if pattern is (3,11) or (5,7)
///
/// # Example
/// ```
/// use primes::hzlib::crt_patterns::zero_pattern;
/// let (zeros, count, tag, comp) = zero_pattern(66, &[3, 5, 7, 11]);
/// // 66/2 = 33 = 3×11
/// assert_eq!(zeros, vec![3, 11]);
/// assert_eq!(count, 2);
/// assert_eq!(tag, "3_and_11");
/// assert!(comp); // Complementary pattern
/// ```
pub fn zero_pattern(base: usize, small: &[usize]) -> (Vec<usize>, usize, String, bool) {
    let p = base / 2;
    let mut z = Vec::new();

    for &m in small {
        if p.is_multiple_of(m) {
            z.push(m);
        }
    }

    z.sort_unstable();

    let len = z.len();
    let comp = z == vec![3, 11] || z == vec![5, 7];

    let tag = match len {
        0 => "none".to_string(),
        1 => format!("only_{}", z[0]),
        2 => format!("{}_and_{}", z[0], z[1]),
        _ => z
            .iter()
            .map(|w| w.to_string())
            .collect::<Vec<_>>()
            .join("_"),
    };

    (z, len, tag, comp)
}

/// Check if the base has the form `2p` with `p` prime.
///
/// Such bases have a distinguished midpoint factor because `base / 2` is
/// itself prime, creating a simple local CRT pattern.
///
/// # Arguments
/// * `b` - Base to check
///
/// # Returns
/// True if b = 2p for prime p
///
/// # Example
/// ```
/// use primes::hzlib::crt_patterns::is_double_prime_base;
/// assert!(is_double_prime_base(6));  // 6 = 2×3
/// assert!(is_double_prime_base(10)); // 10 = 2×5
/// assert!(!is_double_prime_base(12)); // 12 = 2×6 (6 not prime)
/// ```
pub fn is_double_prime_base(b: usize) -> bool {
    if !b.is_multiple_of(2) {
        return false;
    }

    let p = b / 2;
    if p < 2 {
        return false;
    }

    is_prime_simple(p)
}

/// Simple primality test for small numbers
fn is_prime_simple(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }

    let r = (n as f64).sqrt() as usize;
    let mut d = 3usize;

    while d <= r {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 2;
    }

    true
}

/// Classify base by CRT pattern type
///
/// Returns a human-readable classification for analysis purposes
///
/// # Example
/// ```
/// use primes::hzlib::crt_patterns::classify_base;
///
/// let small = [3, 5, 7, 11];
/// assert_eq!(classify_base(66, &small), "complementary_3_and_11");
/// assert_eq!(classify_base(10, &small), "double_prime_only_5");
/// assert_eq!(classify_base(27, &small), "odd_base");
/// ```
pub fn classify_base(base: usize, small_primes: &[usize]) -> String {
    if !base.is_multiple_of(2) {
        return "odd_base".to_string();
    }

    let (zeros, count, tag, comp) = zero_pattern(base, small_primes);

    if comp {
        format!("complementary_{}", tag)
    } else if is_double_prime_base(base) {
        format!("double_prime_{}", tag)
    } else if count == 0 {
        "no_small_factors".to_string()
    } else if count == 1 {
        format!("single_factor_{}", zeros[0])
    } else {
        format!("multi_factor_{}", count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_pattern() {
        let small = vec![3, 5, 7, 11];

        // 66/2 = 33 = 3×11 (complementary)
        let (z, cnt, tag, comp) = zero_pattern(66, &small);
        assert_eq!(z, vec![3, 11]);
        assert_eq!(cnt, 2);
        assert_eq!(tag, "3_and_11");
        assert!(comp);

        // 70/2 = 35 = 5×7 (complementary)
        let (z, cnt, tag, comp) = zero_pattern(70, &small);
        assert_eq!(z, vec![5, 7]);
        assert_eq!(cnt, 2);
        assert_eq!(tag, "5_and_7");
        assert!(comp);

        // 30/2 = 15 = 3×5 (not complementary)
        let (z, cnt, tag, comp) = zero_pattern(30, &small);
        assert_eq!(z, vec![3, 5]);
        assert_eq!(cnt, 2);
        assert_eq!(tag, "3_and_5");
        assert!(!comp);

        // 10/2 = 5 (single factor)
        let (z, cnt, tag, comp) = zero_pattern(10, &small);
        assert_eq!(z, vec![5]);
        assert_eq!(cnt, 1);
        assert_eq!(tag, "only_5");
        assert!(!comp);
    }

    #[test]
    fn test_double_prime() {
        assert!(is_double_prime_base(6)); // 2×3
        assert!(is_double_prime_base(10)); // 2×5
        assert!(is_double_prime_base(14)); // 2×7
        assert!(is_double_prime_base(22)); // 2×11

        assert!(!is_double_prime_base(12)); // 2×6 (6 not prime)
        assert!(!is_double_prime_base(30)); // 2×15 (15 not prime)
        assert!(!is_double_prime_base(7)); // odd
    }

    #[test]
    fn test_classify() {
        let small = vec![3, 5, 7, 11];

        assert_eq!(classify_base(66, &small), "complementary_3_and_11");
        assert_eq!(classify_base(70, &small), "complementary_5_and_7");
        assert!(classify_base(6, &small).starts_with("double_prime"));
        assert!(classify_base(30, &small).contains("multi_factor"));
    }

    #[test]
    fn test_is_prime_simple() {
        assert!(is_prime_simple(2));
        assert!(is_prime_simple(3));
        assert!(is_prime_simple(5));
        assert!(is_prime_simple(7));
        assert!(is_prime_simple(11));

        assert!(!is_prime_simple(1));
        assert!(!is_prime_simple(4));
        assert!(!is_prime_simple(6));
        assert!(!is_prime_simple(9));
    }
}
