//! Optimized Prime Sieves
//!
//! Provides efficient implementations of:
//! - Basic sieve of Eratosthenes (boolean array)
//! - Prime list generation
//! - Segmented sieve for large ranges
//! - Smallest Prime Factor (SPF) computation

use std::cmp::min;

/// Basic sieve of Eratosthenes returning boolean array
///
/// # Arguments
/// * `n` - Upper bound (inclusive)
///
/// # Returns
/// Vector where `result[i] == true` if `i` is prime
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::sieve_bool;
/// let is_prime = sieve_bool(100);
/// assert!(is_prime[2]);
/// assert!(is_prime[97]);
/// assert!(!is_prime[100]);
/// ```
pub fn sieve_bool(n: usize) -> Vec<bool> {
    let mut a = vec![true; n + 1];
    a[0] = false;
    if n >= 1 {
        a[1] = false;
    }
    let r = (n as f64).sqrt() as usize;
    for i in 2..=r {
        if a[i] {
            let mut j = i * i;
            while j <= n {
                a[j] = false;
                j += i;
            }
        }
    }
    a
}

/// Generate list of primes up to n
///
/// # Arguments
/// * `n` - Upper bound (inclusive)
///
/// # Returns
/// Sorted vector of all primes ≤ n
pub fn sieve_primes(n: usize) -> Vec<usize> {
    let a = sieve_bool(n);
    (2..=n).filter(|&i| a[i]).collect()
}

/// Segmented sieve for processing large ranges efficiently
///
/// Calls `on_prime` for each prime found, in ascending order.
/// Memory usage: O(√limit) for base primes + O(segment_size) for current segment
///
/// # Arguments
/// * `limit` - Upper bound (inclusive)
/// * `on_prime` - Callback function called with each prime
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::segmented_sieve;
/// let mut count = 0;
/// segmented_sieve(1000000, |_p| count += 1);
/// assert_eq!(count, 78498); // π(10^6)
/// ```
pub fn segmented_sieve(limit: usize, mut on_prime: impl FnMut(usize)) {
    if limit < 2 {
        return;
    }
    let base = sieve_primes((limit as f64).sqrt() as usize);
    let seg: usize = 1 << 20; // 1MB segment
    let mut low = 0usize;

    while low <= limit {
        let high = min(low + seg - 1, limit);
        let len = high - low + 1;
        let mut mark = vec![true; len];

        // Handle 0 and 1
        if low == 0 {
            if len > 0 {
                mark[0] = false;
            }
            if len > 1 {
                mark[1] = false;
            }
        }

        // Sieve with base primes
        for &p in &base {
            let p2 = p * p;
            if p2 > high {
                break;
            }
            let mut m = if p2 > low { p2 } else { low.div_ceil(p) * p };
            while m <= high {
                mark[m - low] = false;
                m += p;
            }
        }

        // Emit primes in this segment
        for (i, &is_prime) in mark.iter().enumerate().take(len) {
            if is_prime {
                on_prime(low + i);
            }
        }

        if high == limit {
            break;
        }
        low = high + 1;
    }
}

/// Compute Smallest Prime Factor (SPF) for all numbers up to n
///
/// Uses modified sieve to track the smallest prime dividing each number.
/// Useful for factorization and computing singular series.
///
/// # Arguments
/// * `n` - Upper bound (inclusive)
///
/// # Returns
/// Vector where `result[i]` is the smallest prime factor of `i`
/// (with `spf[0] = 0`, `spf[1] = 1`, `spf[p] = p` for primes)
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::sieve_spf;
/// let spf = sieve_spf(100);
/// assert_eq!(spf[12], 2); // 12 = 2² × 3
/// assert_eq!(spf[17], 17); // 17 is prime
/// ```
pub fn sieve_spf(n: usize) -> Vec<usize> {
    let mut spf = (0..=n).collect::<Vec<_>>();
    spf[0] = 0;
    spf[1] = 1;
    let r = (n as f64).sqrt() as usize;

    for i in 2..=r {
        if spf[i] == i {
            // i is prime
            let mut j = i * i;
            while j <= n {
                if spf[j] == j {
                    spf[j] = i;
                }
                j += i;
            }
        }
    }
    spf
}

/// Extract distinct prime factors from factorization
///
/// Uses SPF array to efficiently factorize and return unique prime factors
#[allow(dead_code)]
pub(crate) fn distinct_factors(mut x: usize, spf: &[usize]) -> Vec<usize> {
    let mut out = Vec::new();
    while x > 1 {
        let p = spf[x];
        out.push(p);
        while x > 1 && spf[x] == p {
            x /= p;
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_bool() {
        let is_prime = sieve_bool(30);
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        for p in primes {
            assert!(is_prime[p], "{} should be prime", p);
        }
        assert!(!is_prime[0]);
        assert!(!is_prime[1]);
        assert!(!is_prime[4]);
        assert!(!is_prime[30]);
    }

    #[test]
    fn test_sieve_primes() {
        let primes = sieve_primes(30);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_segmented_sieve() {
        let mut count = 0;
        segmented_sieve(1000, |_| count += 1);
        assert_eq!(count, 168); // π(1000)
    }

    #[test]
    fn test_sieve_spf() {
        let spf = sieve_spf(30);
        assert_eq!(spf[12], 2); // 12 = 2² × 3
        assert_eq!(spf[15], 3); // 15 = 3 × 5
        assert_eq!(spf[17], 17); // 17 is prime
    }

    #[test]
    fn test_distinct_factors() {
        let spf = sieve_spf(100);
        assert_eq!(distinct_factors(12, &spf), vec![2, 3]);
        assert_eq!(distinct_factors(30, &spf), vec![2, 3, 5]);
        assert_eq!(distinct_factors(17, &spf), vec![17]);
    }
}
