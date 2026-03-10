//! Number-theoretic utilities for denominator and material coordinate analysis.
//!
//! This module provides fundamental number theory functions for analyzing
//! rational approximations and their "material" properties (cycle behavior of
//! base-10 under multiplication mod n).
//!
//! # Material Coordinates
//!
//! For a denominator q with core c = q / (2^v2 * 5^v5), we define:
//!
//! - **Purity**: `ord_c(10) / phi(c)` - fraction of totient used by decimal cycle
//! - **Utilization**: `ord_c(10) / lambda(c)` - fraction of Carmichael lambda used
//! - **Slippage**: `lambda(c) / phi(c)` - gap between totient and Carmichael lambda
//!
//! These coordinates are orthogonal to geometric approximation quality, enabling
//! multi-axis optimization for rational approximations.
//!
//! # Example
//!
//! ```rust
//! use prime_physics_engine::hzlib::num_theory::*;
//!
//! let q = 142857;  // Famous cyclic number (1/7)
//! let f = factor(q);
//! let phi = phi_from_factor(&f);
//! let lam = carmichael_lambda_from_factor(&f);
//! let ord = multiplicative_order(10, q);
//!
//! let purity = ord as f64 / phi as f64;
//! println!("q={} has purity={:.4}", q, purity);
//! ```

/// Greatest common divisor using Euclidean algorithm.
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::num_theory::gcd;
/// assert_eq!(gcd(48, 18), 6);
/// assert_eq!(gcd(17, 13), 1);  // Coprime
/// assert_eq!(gcd(0, 5), 5);
/// ```
#[inline]
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// Least common multiple.
///
/// Returns 0 if either input is 0 (to avoid overflow issues).
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::num_theory::lcm;
/// assert_eq!(lcm(4, 6), 12);
/// assert_eq!(lcm(3, 5), 15);
/// ```
#[inline]
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

/// Factor an integer into (prime, exponent) pairs using trial division.
///
/// Efficient for numbers up to ~10^12. Returns empty vector for n < 2.
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::num_theory::factor;
/// assert_eq!(factor(60), vec![(2, 2), (3, 1), (5, 1)]);
/// assert_eq!(factor(13), vec![(13, 1)]);  // Prime
/// assert_eq!(factor(1), vec![]);
/// ```
pub fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    if n < 2 {
        return out;
    }

    // Extract powers of 2
    let mut e = 0u32;
    while n.is_multiple_of(2) {
        n /= 2;
        e += 1;
    }
    if e > 0 {
        out.push((2, e));
    }

    // Try odd divisors
    let mut d = 3u64;
    while d * d <= n {
        let mut e = 0u32;
        while n.is_multiple_of(d) {
            n /= d;
            e += 1;
        }
        if e > 0 {
            out.push((d, e));
        }
        d += 2;
    }

    // Remaining prime factor > sqrt(original n)
    if n > 1 {
        out.push((n, 1));
    }

    out
}

/// Euler's totient function from factorization.
///
/// phi(n) = n * prod_{p|n} (1 - 1/p) = prod_{p^e || n} p^(e-1) * (p-1)
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::num_theory::{factor, phi_from_factor};
/// let f = factor(12);
/// assert_eq!(phi_from_factor(&f), 4);  // phi(12) = 4
/// ```
pub fn phi_from_factor(f: &[(u64, u32)]) -> u64 {
    let mut out: u128 = 1;
    for &(p, e) in f {
        // phi(p^e) = p^(e-1) * (p-1)
        let pe_1: u128 = (p as u128).pow(e - 1);
        out *= (p as u128 - 1) * pe_1;
    }
    out as u64
}

/// Carmichael's lambda function for a prime power.
///
/// lambda(2) = 1, lambda(4) = 2, lambda(2^k) = 2^(k-2) for k >= 3
/// lambda(p^k) = phi(p^k) = p^(k-1) * (p-1) for odd prime p
fn lambda_prime_power(p: u64, e: u32) -> u64 {
    if p == 2 {
        if e == 1 {
            1
        } else if e == 2 {
            2
        } else {
            2u64.pow(e - 2)
        }
    } else {
        // For odd prime powers, lambda = phi
        (p - 1) * p.pow(e - 1)
    }
}

/// Carmichael's lambda function from factorization.
///
/// lambda(n) = lcm of lambda(p^e) over all prime powers p^e || n
///
/// This is the exponent of the multiplicative group (Z/nZ)*.
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::num_theory::{factor, carmichael_lambda_from_factor};
/// let f = factor(8);
/// assert_eq!(carmichael_lambda_from_factor(&f), 2);  // lambda(8) = 2
/// ```
pub fn carmichael_lambda_from_factor(f: &[(u64, u32)]) -> u64 {
    let mut out = 1u64;
    for &(p, e) in f {
        out = lcm(out, lambda_prime_power(p, e));
    }
    out
}

/// Modular exponentiation: a^e mod m.
///
/// Uses binary exponentiation (square-and-multiply).
/// Returns 0 if m == 0.
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::num_theory::pow_mod;
/// assert_eq!(pow_mod(2, 10, 1000), 24);  // 2^10 = 1024 mod 1000 = 24
/// assert_eq!(pow_mod(3, 0, 7), 1);       // a^0 = 1
/// ```
pub fn pow_mod(mut a: u64, mut e: u64, m: u64) -> u64 {
    if m == 0 {
        return 0;
    }
    let mut r: u64 = 1;
    a %= m;
    while e > 0 {
        if (e & 1) == 1 {
            r = (r as u128 * a as u128 % m as u128) as u64;
        }
        a = (a as u128 * a as u128 % m as u128) as u64;
        e >>= 1;
    }
    r
}

/// Multiplicative order of base mod n.
///
/// Returns the smallest positive k such that base^k ≡ 1 (mod n).
/// Returns 0 if gcd(base, n) != 1 or n <= 1.
///
/// # Algorithm
/// Uses the fact that ord_n(base) divides phi(n). We start with phi(n)
/// and repeatedly divide by prime factors while the result remains 1.
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::num_theory::multiplicative_order;
/// assert_eq!(multiplicative_order(10, 7), 6);   // 10^6 ≡ 1 (mod 7)
/// assert_eq!(multiplicative_order(10, 9), 1);   // 10 ≡ 1 (mod 9)
/// assert_eq!(multiplicative_order(2, 7), 3);    // 2^3 = 8 ≡ 1 (mod 7)
/// ```
pub fn multiplicative_order(base: u64, n: u64) -> u64 {
    if n <= 1 {
        return if n == 1 { 1 } else { 0 };
    }
    if gcd(base, n) != 1 {
        return 0;
    }

    let fn_ = factor(n);
    let phi_n = phi_from_factor(&fn_);
    let fphi = factor(phi_n);

    // Start with phi(n) and divide by primes while base^ord ≡ 1
    let mut ord = phi_n;
    for &(p, e) in &fphi {
        for _ in 0..e {
            let cand = ord / p;
            if pow_mod(base, cand, n) == 1 {
                ord = cand;
            } else {
                break;
            }
        }
    }
    ord
}

/// Strip factors of given primes from n.
///
/// Returns (core, exponents) where core has no factors from the given primes,
/// and exponents\[i\] is the exponent of primes\[i\] in n.
///
/// Useful for extracting the "core" of a denominator after removing base factors.
/// For base 10, use primes = [2, 5] to get the repeating part of 1/n.
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::num_theory::strip_factors;
/// let (core, exps) = strip_factors(1000, &[2, 5]);
/// assert_eq!(core, 1);       // 1000 = 2^3 * 5^3
/// assert_eq!(exps, vec![3, 3]);
///
/// let (core, exps) = strip_factors(140, &[2, 5]);
/// assert_eq!(core, 7);       // 140 = 2^2 * 5 * 7
/// assert_eq!(exps, vec![2, 1]);
/// ```
pub fn strip_factors(mut n: u64, primes: &[u64]) -> (u64, Vec<u32>) {
    let mut exps = vec![0u32; primes.len()];
    for (i, &p) in primes.iter().enumerate() {
        while n.is_multiple_of(p) {
            n /= p;
            exps[i] += 1;
        }
    }
    (n, exps)
}

/// Material properties of a denominator for base-10 analysis.
///
/// Computes the "material coordinates" that describe how 1/q behaves
/// as a decimal expansion.
#[derive(Clone, Copy, Debug)]
pub struct Material {
    /// The core after stripping 2s and 5s: q / (2^v2 * 5^v5)
    pub core: u64,
    /// Exponent of 2 in q
    pub v2: u32,
    /// Exponent of 5 in q
    pub v5: u32,
    /// Euler's totient phi(core)
    pub phi: u64,
    /// Carmichael's lambda(core)
    pub lam: u64,
    /// Multiplicative order of 10 mod core
    pub ord: u64,
    /// Purity: ord / phi (fraction of totient used)
    pub purity: f64,
    /// Utilization: ord / lambda (fraction of theoretical max used)
    pub utilization: f64,
    /// Slippage: lambda / phi (multiplicative group structure loss)
    pub slippage: f64,
}

impl Material {
    /// Compute material properties for denominator q in base 10.
    ///
    /// # Example
    /// ```
    /// use prime_physics_engine::hzlib::num_theory::Material;
    /// let m = Material::for_base10(7);
    /// assert_eq!(m.core, 7);
    /// assert_eq!(m.ord, 6);  // 1/7 has period 6
    /// assert!((m.purity - 1.0).abs() < 1e-10);  // Full-period prime
    /// ```
    pub fn for_base10(q: u64) -> Self {
        Self::for_base(q, 10)
    }

    /// Compute material properties for denominator q in arbitrary base.
    ///
    /// The "core" is q with all factors of the base's prime factors removed.
    /// For base 10 = 2*5, we strip 2s and 5s.
    /// For base 6 = 2*3, we strip 2s and 3s.
    /// For base 30 = 2*3*5, we strip 2s, 3s, and 5s.
    ///
    /// # Example
    /// ```
    /// use prime_physics_engine::hzlib::num_theory::Material;
    /// let m = Material::for_base(7, 6);
    /// assert_eq!(m.core, 7);
    /// // ord_7(6) = 6 (6 is primitive root mod 7)
    /// ```
    pub fn for_base(q: u64, base: u64) -> Self {
        // Get prime factors of the base
        let base_factors = factor(base);
        let base_primes: Vec<u64> = base_factors.iter().map(|(p, _)| *p).collect();

        // Strip base factors from q
        let (core, exps) = strip_factors(q, &base_primes);

        // Store first two exponents for backward compatibility
        let v2 = if !exps.is_empty() { exps[0] } else { 0 };
        let v5 = if exps.len() > 1 { exps[1] } else { 0 };

        if core == 1 {
            return Self {
                core,
                v2,
                v5,
                phi: 1,
                lam: 1,
                ord: 0,
                purity: 0.0,
                utilization: 0.0,
                slippage: 0.0,
            };
        }

        let fc = factor(core);
        let phi = phi_from_factor(&fc);
        let lam = carmichael_lambda_from_factor(&fc);
        let ord = multiplicative_order(base, core);

        let purity = ord as f64 / phi as f64;
        let utilization = if lam == 0 {
            0.0
        } else {
            ord as f64 / lam as f64
        };
        let slippage = lam as f64 / phi as f64;

        Self {
            core,
            v2,
            v5,
            phi,
            lam,
            ord,
            purity,
            utilization,
            slippage,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(3, 5), 15);
        assert_eq!(lcm(0, 5), 0);
    }

    #[test]
    fn test_factor() {
        assert_eq!(factor(1), vec![]);
        assert_eq!(factor(2), vec![(2, 1)]);
        assert_eq!(factor(12), vec![(2, 2), (3, 1)]);
        assert_eq!(factor(60), vec![(2, 2), (3, 1), (5, 1)]);
        assert_eq!(factor(13), vec![(13, 1)]);
        assert_eq!(factor(49), vec![(7, 2)]);
    }

    #[test]
    fn test_phi() {
        // phi(1) = 1 (empty product)
        assert_eq!(phi_from_factor(&[]), 1);
        // phi(12) = 12 * (1-1/2) * (1-1/3) = 4
        assert_eq!(phi_from_factor(&factor(12)), 4);
        // phi(7) = 6 (prime)
        assert_eq!(phi_from_factor(&factor(7)), 6);
        // phi(8) = 4
        assert_eq!(phi_from_factor(&factor(8)), 4);
    }

    #[test]
    fn test_carmichael() {
        // lambda(1) = 1
        assert_eq!(carmichael_lambda_from_factor(&[]), 1);
        // lambda(8) = 2
        assert_eq!(carmichael_lambda_from_factor(&factor(8)), 2);
        // lambda(12) = lcm(lambda(4), lambda(3)) = lcm(2, 2) = 2
        assert_eq!(carmichael_lambda_from_factor(&factor(12)), 2);
        // lambda(7) = 6 (prime)
        assert_eq!(carmichael_lambda_from_factor(&factor(7)), 6);
    }

    #[test]
    fn test_pow_mod() {
        assert_eq!(pow_mod(2, 10, 1000), 24);
        assert_eq!(pow_mod(3, 0, 7), 1);
        assert_eq!(pow_mod(2, 3, 7), 1); // 2^3 = 8 ≡ 1 (mod 7)
        assert_eq!(pow_mod(10, 6, 7), 1); // 10^6 ≡ 1 (mod 7)
    }

    #[test]
    fn test_multiplicative_order() {
        assert_eq!(multiplicative_order(10, 7), 6);
        assert_eq!(multiplicative_order(10, 9), 1); // 10 ≡ 1 (mod 9)
        assert_eq!(multiplicative_order(2, 7), 3);
        assert_eq!(multiplicative_order(10, 1), 1); // Edge case
        assert_eq!(multiplicative_order(10, 2), 0); // gcd(10, 2) != 1
        assert_eq!(multiplicative_order(10, 13), 6); // Period of 1/13
    }

    #[test]
    fn test_strip_factors() {
        let (core, exps) = strip_factors(1000, &[2, 5]);
        assert_eq!(core, 1);
        assert_eq!(exps, vec![3, 3]);

        let (core, exps) = strip_factors(140, &[2, 5]);
        assert_eq!(core, 7);
        assert_eq!(exps, vec![2, 1]);

        let (core, exps) = strip_factors(7, &[2, 5]);
        assert_eq!(core, 7);
        assert_eq!(exps, vec![0, 0]);
    }

    #[test]
    fn test_material() {
        // 1/7 = 0.142857142857... (period 6)
        let m = Material::for_base10(7);
        assert_eq!(m.core, 7);
        assert_eq!(m.v2, 0);
        assert_eq!(m.v5, 0);
        assert_eq!(m.phi, 6);
        assert_eq!(m.lam, 6);
        assert_eq!(m.ord, 6);
        assert!((m.purity - 1.0).abs() < 1e-10); // Full-period prime

        // 1/10 = 0.1 (terminating)
        let m = Material::for_base10(10);
        assert_eq!(m.core, 1);
        assert_eq!(m.v2, 1);
        assert_eq!(m.v5, 1);
        assert_eq!(m.ord, 0);

        // 1/142857 = 7 * 10^-6 ish (the famous cyclic number)
        // Actually 142857 = 3^3 * 11 * 13 * 37, let me verify...
        // No wait, 142857 * 7 = 999999, so 142857 = 999999/7
        // Let me check: 142857 = 3^3 * 11 * 13 * 37? No, 142857/3 = 47619
        // 47619/3 = 15873, 15873/3 = 5291 = 11 * 481 = 11 * 13 * 37
        // So 142857 = 3^3 * 11 * 13 * 37, phi = 2*10*12*36 = 8640... complex
        // Better to test with simpler numbers
    }
}
