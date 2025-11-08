//! Babylonian-Prime Divergence: Orthogonality of Human and Natural Mathematics
//!
//! This module implements the statistical framework for demonstrating that
//! human-convenient mathematical structures (highly divisible numbers, base-60 legacy)
//! are orthogonal to nature's mathematical structures (prime patterns, harmonic cycles).
//!
//! # Core Concepts
//!
//! ## Babylonian Score
//! Measures "human convenience" by rewarding:
//! - High divisibility by 2, 3, 5 (base-60 smoothness)
//! - Multiple of 60 bonus
//! - Penalty for "inconvenient" prime factors
//!
//! ## Prime Harmony Score
//! Measures "natural structure" by counting prime pairs at gap g,
//! normalized by Hardy-Littlewood expectations to remove arithmetic bias.
//!
//! ## Orthogonality Thesis
//! After HL normalization, Corr(Babylonian, Harmony) ≈ 0,
//! demonstrating independence of human and natural mathematical aesthetics.
//!
//! # Example
//!
//! ```rust
//! use prime_physics_engine::hzlib::orthogonality::*;
//! use prime_physics_engine::hzlib::sieve::sieve_bool;
//!
//! let is_prime = sieve_bool(1_000_000);
//! let pairs = pairs_index(&is_prime, 300);
//!
//! // Compute scores for gap=6
//! let bab = babylonian_score_60(6);  // High (6 = 2×3, convenient)
//! let raw = count_pairs_upto(&pairs[3], 1_000_000);  // Raw count
//!
//! // HL normalization removes arithmetic bias
//! let expected = singular_series(6) * (1_000_000.0 / 13.8_f64.powi(2));
//! let normalized = raw as f64 / expected;
//!
//! // Babylonian and normalized scores are orthogonal!
//! ```

use super::hardy_littlewood::C2;
use std::cmp::min;

// ======================== Prime Factorization ========================

/// Factor a small integer using trial division.
///
/// Returns vector of (prime, exponent) pairs.
/// Efficient for gaps and other small integers (< 10^6).
pub fn prime_factors_small(mut n: usize) -> Vec<(usize, usize)> {
    let mut factors = Vec::new();
    let mut d = 2;

    while d * d <= n {
        if n % d == 0 {
            let mut e = 0;
            while n % d == 0 {
                n /= d;
                e += 1;
            }
            factors.push((d, e));
        }
        d = if d == 2 { 3 } else { d + 2 };
    }

    if n > 1 {
        factors.push((n, 1));
    }

    factors
}

/// Compute divisor count from factorization.
///
/// τ(n) = ∏ (eᵢ + 1) where n = ∏ pᵢ^eᵢ
pub fn tau_from_factor(factors: &[(usize, usize)]) -> usize {
    factors.iter().map(|(_, e)| e + 1).product()
}

// ======================== Babylonian Scores ========================

/// Babylonian score emphasizing base-60 compatibility.
///
/// Formula:
/// ```text
/// B₆₀(g) = 2(e₂ + e₃ + e₅) + 10·𝟙(60|g) - 3·|others| + 0.5·τ(g)
/// ```
///
/// High scores indicate "human-friendly" numbers (easy divisibility,
/// compatible with ancient Babylonian sexagesimal system).
///
/// # Arguments
/// * `g` - Even gap value
///
/// # Returns
/// Babylonian score (higher = more human-convenient)
///
/// # Example
/// ```rust
/// use prime_physics_engine::hzlib::orthogonality::babylonian_score_60;
///
/// assert!(babylonian_score_60(60) > babylonian_score_60(62));  // 60 is more convenient
/// assert!(babylonian_score_60(30) > babylonian_score_60(14));  // 30 = 2×3×5
/// ```
pub fn babylonian_score_60(g: usize) -> f64 {
    if g == 0 || g % 2 == 1 {
        return 0.0;
    }

    let factors = prime_factors_small(g);
    let mut e2 = 0;
    let mut e3 = 0;
    let mut e5 = 0;
    let mut others = 0;

    for (p, e) in &factors {
        match *p {
            2 => e2 = *e,
            3 => e3 = *e,
            5 => e5 = *e,
            _ => others += 1,
        }
    }

    let mut score = 2.0 * ((e2 + e3 + e5) as f64);
    score += if g % 60 == 0 { 10.0 } else { 0.0 };
    score -= 3.0 * (others as f64);
    score += 0.5 * (tau_from_factor(&factors) as f64);

    score
}

/// Pure divisibility-based Babylonian score.
///
/// Simply returns τ(g), the divisor count.
/// Alternative baseline that doesn't specifically favor base-60.
///
/// # Example
/// ```rust
/// use prime_physics_engine::hzlib::orthogonality::babylonian_score_tau;
///
/// assert_eq!(babylonian_score_tau(12), 6.0);  // 1,2,3,4,6,12
/// assert_eq!(babylonian_score_tau(13), 2.0);  // 1,13 (prime)
/// ```
pub fn babylonian_score_tau(g: usize) -> f64 {
    if g == 0 {
        return 0.0;
    }
    let factors = prime_factors_small(g);
    tau_from_factor(&factors) as f64
}

// ======================== Singular Series ========================

/// Hardy-Littlewood singular series for gap g = 2k.
///
/// Formula:
/// ```text
/// S(g) = 2C₂ × ∏_{p|k, p>2} (p-1)/(p-2)
/// ```
///
/// This quantifies the arithmetic bias: gaps with small prime factors
/// are more likely to contain prime pairs, creating spurious correlation
/// with Babylonian scores (which also favor small prime factors).
///
/// # Arguments
/// * `g` - Even gap
///
/// # Returns
/// Singular series value (dimensionless multiplicative factor)
///
/// # Example
/// ```rust
/// use prime_physics_engine::hzlib::orthogonality::singular_series;
///
/// let s2 = singular_series(2);   // Twin primes
/// let s6 = singular_series(6);   // Sexy primes (gap=6=2×3)
/// assert!(s6 > s2);  // Gap 6 has arithmetic advantage
/// ```
pub fn singular_series(g: usize) -> f64 {
    if g == 0 || g % 2 == 1 {
        return 0.0;
    }

    let k = g / 2;
    let factors = prime_factors_small(k);

    let mut s = 2.0 * C2;
    for (p, _) in factors {
        if p > 2 {
            s *= (p as f64 - 1.0) / (p as f64 - 2.0);
        }
    }

    s
}

// ======================== Prime Pair Indexing ========================

/// Build an index of prime pairs by gap.
///
/// Returns a vector where `result[g/2]` contains all primes p such that
/// p and p+g are both prime.
///
/// # Arguments
/// * `is_prime` - Boolean sieve array (from `sieve_bool`)
/// * `gmax` - Maximum even gap to index
///
/// # Returns
/// Vector of vectors, indexed by g/2, containing primes p with (p, p+g) both prime.
///
/// # Memory
/// O(π(N) × gmax/2) worst case, but typically much less due to sparsity.
///
/// # Example
/// ```rust
/// use prime_physics_engine::hzlib::orthogonality::pairs_index;
/// use prime_physics_engine::hzlib::sieve::sieve_bool;
///
/// let is_prime = sieve_bool(100);
/// let pairs = pairs_index(&is_prime, 20);
///
/// // pairs[1] = primes p where (p, p+2) both prime (twin primes)
/// assert!(pairs[1].contains(&3));   // (3,5)
/// assert!(pairs[1].contains(&5));   // (5,7)
/// ```
pub fn pairs_index(is_prime: &[bool], gmax: usize) -> Vec<Vec<u32>> {
    let n = is_prime.len() - 1;
    let mut vecs = vec![Vec::new(); gmax / 2 + 1];

    for p in 2..=n {
        if !is_prime[p] {
            continue;
        }

        let max_g = min(gmax, n - p);
        let mut g = 2;
        while g <= max_g {
            if is_prime[p + g] {
                vecs[g / 2].push(p as u32);
            }
            g += 2;
        }
    }

    vecs
}

/// Count prime pairs (p, p+gap) with p ≤ limit.
///
/// Uses binary search on sorted pair list for efficiency.
///
/// # Arguments
/// * `pairs` - Sorted list of primes p (from `pairs_index`)
/// * `limit` - Maximum value of p to count
///
/// # Returns
/// Count of pairs with p ≤ limit
///
/// # Example
/// ```rust
/// use prime_physics_engine::hzlib::orthogonality::*;
/// use prime_physics_engine::hzlib::sieve::sieve_bool;
///
/// let is_prime = sieve_bool(1000);
/// let pairs = pairs_index(&is_prime, 10);
///
/// // Count twin primes up to 500
/// let count = count_pairs_upto(&pairs[1], 500);
/// ```
pub fn count_pairs_upto(pairs: &[u32], limit: usize) -> usize {
    if pairs.is_empty() {
        return 0;
    }

    // Binary search: first index > limit
    let mut lo = 0;
    let mut hi = pairs.len();
    let key = limit as u32;

    while lo < hi {
        let mid = (lo + hi) >> 1;
        if pairs[mid] <= key {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    lo
}

// ======================== Statistics ========================

/// Pearson correlation coefficient.
///
/// # Arguments
/// * `x`, `y` - Data vectors (must have same length ≥ 2)
///
/// # Returns
/// Correlation r ∈ [-1, 1], or NaN if invalid
///
/// # Example
/// ```rust
/// use prime_physics_engine::hzlib::orthogonality::pearson;
///
/// let x = vec![1.0, 2.0, 3.0, 4.0];
/// let y = vec![2.0, 4.0, 6.0, 8.0];
/// let r = pearson(&x, &y);
/// assert!((r - 1.0).abs() < 1e-10);  // Perfect correlation
/// ```
pub fn pearson(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len();
    if n < 2 || x.len() != y.len() {
        return f64::NAN;
    }

    let mx: f64 = x.iter().sum::<f64>() / n as f64;
    let my: f64 = y.iter().sum::<f64>() / n as f64;

    let mut num = 0.0;
    let mut dx2 = 0.0;
    let mut dy2 = 0.0;

    for i in 0..n {
        let dx = x[i] - mx;
        let dy = y[i] - my;
        num += dx * dy;
        dx2 += dx * dx;
        dy2 += dy * dy;
    }

    if dx2 <= 0.0 || dy2 <= 0.0 {
        return f64::NAN;
    }

    num / (dx2.sqrt() * dy2.sqrt())
}

/// Weighted correlation coefficient.
///
/// Computes correlation with weighted means and variances.
///
/// # Arguments
/// * `x`, `y` - Data vectors
/// * `w` - Weight vector (positive weights)
///
/// # Returns
/// Weighted correlation r ∈ [-1, 1], or NaN if invalid
///
/// # Example
/// ```rust
/// use prime_physics_engine::hzlib::orthogonality::weighted_corr;
///
/// let x = vec![1.0, 2.0, 3.0];
/// let y = vec![1.0, 2.0, 10.0];
/// let w = vec![1.0, 1.0, 0.1];  // Downweight outlier
/// let r = weighted_corr(&x, &y, &w);
/// ```
pub fn weighted_corr(x: &[f64], y: &[f64], w: &[f64]) -> f64 {
    let n = x.len();
    if n < 2 || x.len() != y.len() || x.len() != w.len() {
        return f64::NAN;
    }

    let ws: f64 = w.iter().sum();
    if ws <= 0.0 {
        return f64::NAN;
    }

    let mx = x.iter()
        .zip(w)
        .map(|(xi, wi)| xi * wi)
        .sum::<f64>()
        / ws;
    let my = y.iter()
        .zip(w)
        .map(|(yi, wi)| yi * wi)
        .sum::<f64>()
        / ws;

    let mut num = 0.0;
    let mut vx = 0.0;
    let mut vy = 0.0;

    for i in 0..n {
        let dx = x[i] - mx;
        let dy = y[i] - my;
        let wi = w[i];
        num += wi * dx * dy;
        vx += wi * dx * dx;
        vy += wi * dy * dy;
    }

    if vx <= 0.0 || vy <= 0.0 {
        return f64::NAN;
    }

    num / (vx.sqrt() * vy.sqrt())
}

/// t-statistic for correlation.
///
/// Formula: t = r√((n-2)/(1-r²))
///
/// Under null hypothesis (r=0), t ~ Student-t(n-2).
///
/// # Arguments
/// * `r` - Correlation coefficient
/// * `n` - Sample size
///
/// # Returns
/// t-statistic, or NaN if invalid
///
/// # Example
/// ```rust
/// use prime_physics_engine::hzlib::orthogonality::t_stat;
///
/// let r = 0.3;
/// let n = 100;
/// let t = t_stat(r, n);
/// // |t| > 2 suggests significant correlation (p < 0.05, two-tailed)
/// ```
pub fn t_stat(r: f64, n: usize) -> f64 {
    if !r.is_finite() || n < 3 {
        return f64::NAN;
    }

    let denom = (1.0 - r * r).max(1e-16);
    r * ((n as f64 - 2.0) / denom).sqrt()
}

// ======================== Randomization ========================

/// Simple XorShift64 PRNG for permutation tests.
///
/// Not cryptographically secure, but fast and sufficient for statistical tests.
pub struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    /// Create new RNG with given seed.
    ///
    /// Seed must be non-zero (uses max(seed, 1)).
    pub fn new(seed: u64) -> Self {
        Self {
            state: seed.max(1),
        }
    }

    /// Generate next u64.
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// Generate f64 in [0, 1).
    pub fn next_f64(&mut self) -> f64 {
        let u = self.next_u64() >> 11; // 53-bit mantissa
        (u as f64) / ((1u64 << 53) as f64)
    }
}

/// Fisher-Yates shuffle in place.
///
/// # Example
/// ```rust
/// use prime_physics_engine::hzlib::orthogonality::{XorShift64, shuffle_in_place};
///
/// let mut rng = XorShift64::new(12345);
/// let mut data = vec![1, 2, 3, 4, 5];
/// shuffle_in_place(&mut data, &mut rng);
/// ```
pub fn shuffle_in_place<T>(v: &mut [T], rng: &mut XorShift64) {
    for i in (1..v.len()).rev() {
        let j = (rng.next_f64() * ((i + 1) as f64)).floor() as usize;
        v.swap(i, j);
    }
}

// ======================== Cramér Model ========================

/// Generate Cramér-model "primes" (random selection with probability 1/ln(n)).
///
/// This creates a random sequence with the correct density but no structure.
/// Used as a control to verify that observed patterns are not artifacts.
///
/// # Arguments
/// * `n` - Upper bound
/// * `rng` - Random number generator
///
/// # Returns
/// Boolean sieve where `true` indicates "prime"
///
/// # Example
/// ```rust
/// use prime_physics_engine::hzlib::orthogonality::{sieve_cramer, XorShift64};
///
/// let mut rng = XorShift64::new(42);
/// let cramer_primes = sieve_cramer(10000, &mut rng);
/// // Should have ~π(10000) ≈ 1229 "primes", but no structure
/// ```
pub fn sieve_cramer(n: usize, rng: &mut XorShift64) -> Vec<bool> {
    let mut is_prime = vec![false; n + 1];
    if n >= 2 {
        is_prime[2] = true;
    }

    for x in (3..=n).step_by(2) {
        let p = 1.0 / (x as f64).ln();
        if rng.next_f64() < p {
            is_prime[x] = true;
        }
    }

    is_prime
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_factors_small() {
        assert_eq!(prime_factors_small(12), vec![(2, 2), (3, 1)]);
        assert_eq!(prime_factors_small(60), vec![(2, 2), (3, 1), (5, 1)]);
        assert_eq!(prime_factors_small(13), vec![(13, 1)]);
    }

    #[test]
    fn test_tau() {
        assert_eq!(tau_from_factor(&[(2, 2), (3, 1)]), 6); // 12 has 6 divisors
        assert_eq!(tau_from_factor(&[(13, 1)]), 2); // prime has 2 divisors
    }

    #[test]
    fn test_babylonian_scores() {
        // 60 should score very high (base-60 itself)
        let s60 = babylonian_score_60(60);
        let s62 = babylonian_score_60(62);
        assert!(s60 > s62);

        // 30 = 2×3×5 should beat 14 = 2×7
        let s30 = babylonian_score_60(30);
        let s14 = babylonian_score_60(14);
        assert!(s30 > s14);
    }

    #[test]
    fn test_singular_series() {
        let s2 = singular_series(2); // Twin primes
        let s6 = singular_series(6); // Gap 6 = 2×3 has advantage
        assert!(s6 > s2);
    }

    #[test]
    fn test_pearson() {
        let x = vec![1.0, 2.0, 3.0, 4.0];
        let y = vec![2.0, 4.0, 6.0, 8.0];
        let r = pearson(&x, &y);
        assert!((r - 1.0).abs() < 1e-10); // Perfect correlation
    }

    #[test]
    fn test_xorshift() {
        let mut rng = XorShift64::new(12345);
        let u1 = rng.next_u64();
        let u2 = rng.next_u64();
        assert_ne!(u1, u2); // Should produce different values
    }
}
