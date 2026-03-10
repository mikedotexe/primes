//! Hardy-Littlewood Framework
//!
//! Implements singular series computation and Goldbach pair counting
//! based on the Hardy-Littlewood conjecture for prime distributions.
//!
//! # Mathematical Conventions
//!
//! - All logarithms are **natural logs** (base e), consistent with HL literature
//! - Pair counting can be ordered (p,q) and (q,p) separate, or unordered {p,q}
//! - The twin-prime constant C₂ ≈ 0.6601618158 appears in the front factor κ
//! - Coverage uses Poisson approximation: Pr[r(n)≥1] = 1 - e^(-λ)

/// Pair counting convention for Goldbach analysis
///
/// Determines whether (p,q) and (q,p) are counted separately or as one pair.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PairCount {
    /// Count (p,q) and (q,p) as distinct ordered pairs
    Ordered,
    /// Count unordered pairs {p,q} (asymptotically half of ordered, ignoring p=q)
    Unordered,
}

/// Twin-prime constant C₂ = ∏_{p>2} (1 - 1/(p-1)²)
///
/// This appears in Hardy-Littlewood conjectures for twin primes and Goldbach.
/// Value computed to high precision: C₂ ≈ 0.6601618158468696
pub const C2: f64 = 0.6601618158468696;

/// Front constant κ for HL Goldbach prediction
///
/// - κ = 2·C₂ ≈ 1.3203236317 for ordered pairs (p,q)
/// - κ = C₂ ≈ 0.6601618158 for unordered pairs {p,q}
///
/// # Example
/// ```
/// use primes::hzlib::{kappa, PairCount, C2};
/// assert!((kappa(PairCount::Ordered) - 2.0 * C2).abs() < 1e-10);
/// assert!((kappa(PairCount::Unordered) - C2).abs() < 1e-10);
/// ```
#[inline]
pub fn kappa(pairing: PairCount) -> f64 {
    match pairing {
        PairCount::Ordered => 2.0 * C2,
        PairCount::Unordered => C2,
    }
}

/// Compute multiplicative part of Hardy-Littlewood singular series for Goldbach
///
/// For even n, computes S₂(n) = ∏_{p|n, p>2} (p-1)/(p-2)
///
/// This is the **multiplicative correction only**. The full HL expectation is:
/// ```text
/// E[r(n)] ≈ κ · S₂(n) · n / (ln n)²
/// ```
/// where κ depends on whether you count ordered or unordered pairs (see [`kappa`]).
///
/// # Arguments
/// * `n` - Even number to analyze
/// * `spf` - Smallest prime factor array (from `sieve_spf`)
///
/// # Returns
/// S₂(n) - the multiplicative singular series factor (without κ)
///
/// # Example
/// ```
/// use primes::hzlib::{sieve_spf, singular_series_goldbach_multiplicative};
/// let spf = sieve_spf(100);
/// let s2 = singular_series_goldbach_multiplicative(30, &spf); // 30 = 2×3×5
/// // S₂(30) = (3-1)/(3-2) × (5-1)/(5-2) = 2 × 4/3 = 8/3
/// assert!((s2 - 8.0/3.0).abs() < 1e-12);
/// ```
pub fn singular_series_goldbach_multiplicative(n: usize, spf: &[usize]) -> f64 {
    if n % 2 == 1 || n < 4 {
        return 0.0;
    }

    let mut m = n;
    let mut s = 1.0_f64;
    let mut last = 0usize;

    while m > 1 {
        let p = spf[m];
        if p != last {
            if p > 2 {
                s *= (p as f64 - 1.0) / (p as f64 - 2.0);
            }
            last = p;
        }
        m /= p;
    }
    s
}

/// Compute Hardy-Littlewood singular series for Goldbach (legacy)
///
/// **Deprecated**: Use [`singular_series_goldbach_multiplicative`] instead for clarity.
/// This function returns S₂(n) without the κ constant.
///
/// For even n, the singular series S₂(n) is:
/// ```text
/// S₂(n) = ∏_{p|n, p>2} (p-1)/(p-2)
/// ```
///
/// This appears in the HL prediction for the number of Goldbach pairs:
/// ```text
/// E[#{(p,q) : p+q=n, p,q prime}] ≈ κ · S₂(n) · n / (ln n)²
/// ```
/// where κ = 2·C₂ for ordered pairs or C₂ for unordered pairs.
///
/// # Arguments
/// * `n` - Even number to analyze
/// * `spf` - Smallest prime factor array (from `sieve_spf`)
///
/// # Returns
/// Singular series S₂(n)
///
/// # Example
/// ```
/// use primes::hzlib::{sieve_spf, singular_series_goldbach};
/// let spf = sieve_spf(100);
/// let s = singular_series_goldbach(30, &spf); // 30 = 2×3×5
/// // S₂(30) = (3-1)/(3-2) × (5-1)/(5-2) = 2/1 × 4/3 = 8/3 ≈ 2.667
/// assert!((s - 8.0/3.0).abs() < 0.01);
/// ```
pub fn singular_series_goldbach(n: usize, spf: &[usize]) -> f64 {
    singular_series_goldbach_multiplicative(n, spf)
}

/// Count Goldbach pairs (p, q) where p + q = n and both are ≥ base
///
/// Used for "restricted Goldbach" analysis near 2·base.
///
/// # Arguments
/// * `n` - Target sum (must be even)
/// * `base` - Minimum value for primes
/// * `is_prime` - Boolean array from sieve
///
/// # Returns
/// Number of pairs (p, q) with p ≤ q, p+q=n, p,q ≥ base, both prime
///
/// # Example
/// ```
/// use primes::hzlib::{sieve_bool, count_pairs_for_n};
/// let is_prime = sieve_bool(100);
/// let pairs = count_pairs_for_n(30, 2, &is_prime);
/// // 30 = 11+19 = 13+17
/// assert_eq!(pairs, 2);
/// ```
pub fn count_pairs_for_n(n: usize, base: usize, is_prime: &[bool]) -> usize {
    if n < 2 * base {
        return 0;
    }

    let start = std::cmp::max(base, 2);
    let end = n / 2;
    let mut cnt = 0usize;

    for p in start..=end {
        if p < is_prime.len() && is_prime[p] {
            let q = n - p;
            if q >= base && q < is_prime.len() && is_prime[q] {
                cnt += 1;
            }
        }
    }

    cnt
}

/// Predict Goldbach pair count using HL formula
///
/// # Arguments
/// * `n` - Even number
/// * `spf` - Smallest prime factor array
/// * `k_hat` - Empirical scaling constant (fit from data)
///
/// # Returns
/// Predicted expected number of pairs
pub fn predict_goldbach_pairs(n: usize, spf: &[usize], k_hat: f64) -> f64 {
    let ln = (n as f64).ln();
    if ln <= 0.0 {
        return 0.0;
    }

    let x = singular_series_goldbach(n, spf) * (n as f64) / (ln * ln);
    k_hat * x
}

/// Hardy-Littlewood expected number of Goldbach pairs (unrestricted)
///
/// Computes λ(n) = κ · S₂(n) · n / (ln n)² where:
/// - S₂(n) is the multiplicative singular series
/// - κ depends on ordering convention (see [`PairCount`])
/// - ln is the **natural logarithm** (base e)
///
/// This gives the expected number of ways to write n as a sum of two primes.
///
/// # Arguments
/// * `n` - Even number to analyze
/// * `spf` - Smallest prime factor array
/// * `pairing` - Ordered or unordered pair counting
///
/// # Returns
/// Expected number of Goldbach pairs λ(n)
///
/// # Example
/// ```
/// use primes::hzlib::{sieve_spf, hl_goldbach_lambda, PairCount};
/// let spf = sieve_spf(10000);
/// let lambda = hl_goldbach_lambda(1000, &spf, PairCount::Unordered);
/// // For n=1000, expect λ ≈ 20-30 pairs
/// assert!(lambda > 10.0);
/// ```
pub fn hl_goldbach_lambda(n: usize, spf: &[usize], pairing: PairCount) -> f64 {
    if n % 2 == 1 || n < 4 {
        return 0.0;
    }

    let s2 = singular_series_goldbach_multiplicative(n, spf);
    let ln = (n as f64).ln(); // Natural log

    if ln <= 0.0 {
        return 0.0;
    }

    kappa(pairing) * s2 * (n as f64) / (ln * ln)
}

/// Hardy-Littlewood expected pairs with truncation (restricted Goldbach)
///
/// For the restricted problem where both primes must be ≥ lo, uses:
/// ```text
/// λ(n, lo) ≈ κ · S₂(n) · Σ_{x=lo}^{n-lo} 1 / (ln(x) · ln(n-x))
/// ```
///
/// This sum approximates the probability that x and n-x are both prime,
/// using the Prime Number Theorem heuristic Pr[x prime] ≈ 1/ln(x).
///
/// **Critical for analyzing Goldbach pairs near 2·base** where you enforce
/// p, q ≥ base.
///
/// # Arguments
/// * `n` - Even number (target sum)
/// * `lo` - Minimum value for both primes (typically base)
/// * `spf` - Smallest prime factor array
/// * `pairing` - Ordered or unordered pair counting
///
/// # Returns
/// Expected number of restricted Goldbach pairs λ(n, lo)
///
/// # Example
/// ```
/// use primes::hzlib::{sieve_spf, hl_goldbach_lambda_truncated, PairCount};
/// let spf = sieve_spf(10000);
/// // Pairs for 1000 with both primes ≥ 100
/// let lambda = hl_goldbach_lambda_truncated(1000, 100, &spf, PairCount::Unordered);
/// assert!(lambda > 0.0);
/// ```
pub fn hl_goldbach_lambda_truncated(n: usize, lo: usize, spf: &[usize], pairing: PairCount) -> f64 {
    if n % 2 == 1 || n < 2 * lo || lo < 2 {
        return 0.0;
    }

    let s2 = singular_series_goldbach_multiplicative(n, spf);
    let mut sum = 0.0_f64;

    let start = lo.max(2);
    let end = n - lo;

    for x in start..=end {
        let y = n - x;
        if y < 2 {
            continue;
        }
        // PNT proxy: Pr[x prime] ~ 1/ln(x), independent up to HL correction
        sum += 1.0 / ((x as f64).ln() * (y as f64).ln());
    }

    kappa(pairing) * s2 * sum
}

/// Compute Goldbach coverage probability using Poisson approximation
///
/// Uses the Chen-Stein Poisson heuristic:
/// ```text
/// Pr[r(n) ≥ 1] ≈ 1 - e^(-λ)
/// ```
/// where λ = E[r(n)] is the expected number of pairs.
///
/// This is the standard way to convert expected counts to coverage probability.
///
/// # Arguments
/// * `lambda` - Expected number of Goldbach pairs (from HL formula)
///
/// # Returns
/// Probability that at least one pair exists
///
/// # Example
/// ```
/// use primes::hzlib::goldbach_coverage_from_lambda;
/// // λ=0 → 0% coverage
/// assert_eq!(goldbach_coverage_from_lambda(0.0), 0.0);
/// // λ=1 → ~63% coverage
/// assert!((goldbach_coverage_from_lambda(1.0) - 0.632).abs() < 0.01);
/// ```
pub fn goldbach_coverage_from_lambda(lambda: f64) -> f64 {
    if lambda <= 0.0 {
        0.0
    } else {
        1.0 - (-lambda).exp()
    }
}

/// Compute Goldbach coverage probability using Poisson approximation (legacy)
///
/// **Deprecated**: Use [`goldbach_coverage_from_lambda`] for clarity.
///
/// Pr[at least one pair] ≈ 1 - exp(-λ)
/// where λ is the expected pair count
pub fn goldbach_coverage(expected_pairs: f64) -> f64 {
    goldbach_coverage_from_lambda(expected_pairs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hzlib::sieve::{sieve_bool, sieve_spf};

    #[test]
    fn test_constants() {
        // C₂ constant value
        assert!((C2 - 0.6601618158468696).abs() < 1e-15);

        // κ relationship: ordered = 2 × unordered
        let k_ord = kappa(PairCount::Ordered);
        let k_uno = kappa(PairCount::Unordered);
        assert!((k_ord - 2.0 * k_uno).abs() < 1e-12);

        // Exact values
        assert!((k_ord - 2.0 * C2).abs() < 1e-12);
        assert!((k_uno - C2).abs() < 1e-12);
    }

    #[test]
    fn test_singular_series_multiplicative() {
        let spf = sieve_spf(100);

        // S₂(30) = (3-1)/(3-2) × (5-1)/(5-2) = 2 × 4/3 = 8/3
        let s30 = singular_series_goldbach_multiplicative(30, &spf);
        assert!(
            (s30 - 8.0 / 3.0).abs() < 1e-12,
            "S₂(30) should be exactly 8/3"
        );

        // S₂(6) = (3-1)/(3-2) = 2
        let s6 = singular_series_goldbach_multiplicative(6, &spf);
        assert!((s6 - 2.0).abs() < 1e-12, "S₂(6) should be exactly 2");

        // S₂(10) = (5-1)/(5-2) = 4/3
        let s10 = singular_series_goldbach_multiplicative(10, &spf);
        assert!((s10 - 4.0 / 3.0).abs() < 1e-12);

        // Odd n should return 0
        assert_eq!(singular_series_goldbach_multiplicative(15, &spf), 0.0);

        // n < 4 should return 0
        assert_eq!(singular_series_goldbach_multiplicative(2, &spf), 0.0);
    }

    #[test]
    fn test_singular_series_legacy() {
        let spf = sieve_spf(100);

        // Legacy function should match new implementation
        let s30_new = singular_series_goldbach_multiplicative(30, &spf);
        let s30_old = singular_series_goldbach(30, &spf);
        assert!((s30_new - s30_old).abs() < 1e-15);
    }

    #[test]
    fn test_hl_lambda_ordering() {
        let spf = sieve_spf(10000);

        // For large even n, ordered should be ~2× unordered
        for n in [1000, 2000, 5000].iter().copied() {
            let lambda_ord = hl_goldbach_lambda(n, &spf, PairCount::Ordered);
            let lambda_uno = hl_goldbach_lambda(n, &spf, PairCount::Unordered);

            // Ratio should be very close to 2.0
            let ratio = lambda_ord / lambda_uno;
            assert!(
                (ratio - 2.0).abs() < 0.01,
                "Ordered/Unordered ratio at n={} is {}, expected ~2.0",
                n,
                ratio
            );
        }
    }

    #[test]
    fn test_hl_lambda_natural_log() {
        let spf = sieve_spf(10000);

        // Verify we're using natural log by checking the scale
        let n = 1000;
        let s2 = singular_series_goldbach_multiplicative(n, &spf);
        let lambda = hl_goldbach_lambda(n, &spf, PairCount::Unordered);

        // Manually compute with natural log
        let ln_n = (n as f64).ln();
        let expected = C2 * s2 * (n as f64) / (ln_n * ln_n);

        assert!(
            (lambda - expected).abs() < 1e-10,
            "Lambda mismatch: got {}, expected {}",
            lambda,
            expected
        );
    }

    #[test]
    fn test_truncated_monotonicity() {
        let spf = sieve_spf(10000);
        let n = 1000;

        // λ(n, B₁) ≥ λ(n, B₂) when B₁ ≤ B₂
        let lambda_10 = hl_goldbach_lambda_truncated(n, 10, &spf, PairCount::Unordered);
        let lambda_50 = hl_goldbach_lambda_truncated(n, 50, &spf, PairCount::Unordered);
        let lambda_100 = hl_goldbach_lambda_truncated(n, 100, &spf, PairCount::Unordered);

        assert!(
            lambda_10 >= lambda_50,
            "λ(1000, 10) = {} should be ≥ λ(1000, 50) = {}",
            lambda_10,
            lambda_50
        );
        assert!(
            lambda_50 >= lambda_100,
            "λ(1000, 50) = {} should be ≥ λ(1000, 100) = {}",
            lambda_50,
            lambda_100
        );
    }

    #[test]
    fn test_truncated_vs_unrestricted() {
        let spf = sieve_spf(10000);
        let n = 1000;

        // Truncated with lo=2 should be close to unrestricted
        let lambda_unrestricted = hl_goldbach_lambda(n, &spf, PairCount::Unordered);
        let lambda_truncated = hl_goldbach_lambda_truncated(n, 2, &spf, PairCount::Unordered);

        // They use different formulas but should be in same ballpark
        assert!(lambda_truncated > 0.0);
        assert!(lambda_unrestricted > 0.0);
        // Truncated sum is usually smaller than asymptotic formula
        assert!(
            lambda_truncated <= lambda_unrestricted * 1.5,
            "Truncated {} too large vs unrestricted {}",
            lambda_truncated,
            lambda_unrestricted
        );
    }

    #[test]
    fn test_truncated_edge_cases() {
        let spf = sieve_spf(1000);

        // n < 2*lo should return 0
        assert_eq!(
            hl_goldbach_lambda_truncated(100, 100, &spf, PairCount::Unordered),
            0.0
        );

        // Odd n should return 0
        assert_eq!(
            hl_goldbach_lambda_truncated(101, 10, &spf, PairCount::Unordered),
            0.0
        );

        // lo < 2 should return 0
        assert_eq!(
            hl_goldbach_lambda_truncated(100, 1, &spf, PairCount::Unordered),
            0.0
        );
    }

    #[test]
    fn test_coverage_from_lambda() {
        // λ=0 → 0% coverage
        assert_eq!(goldbach_coverage_from_lambda(0.0), 0.0);

        // λ=1 → ~63.2% coverage (1 - 1/e)
        let cov1 = goldbach_coverage_from_lambda(1.0);
        let expected_1 = 1.0 - 1.0_f64.exp().recip();
        assert!((cov1 - expected_1).abs() < 1e-10);

        // λ=5 → ~99.3% coverage
        let cov5 = goldbach_coverage_from_lambda(5.0);
        let expected_5 = 1.0 - (-5.0_f64).exp();
        assert!((cov5 - expected_5).abs() < 1e-10);

        // Negative λ should return 0
        assert_eq!(goldbach_coverage_from_lambda(-1.0), 0.0);
    }

    #[test]
    fn test_coverage_legacy() {
        // Legacy function should match new implementation
        assert_eq!(goldbach_coverage(0.0), goldbach_coverage_from_lambda(0.0));
        assert_eq!(goldbach_coverage(1.0), goldbach_coverage_from_lambda(1.0));
        assert_eq!(goldbach_coverage(5.0), goldbach_coverage_from_lambda(5.0));
    }

    #[test]
    fn test_count_pairs() {
        let is_prime = sieve_bool(100);

        // 30 = 7+23 = 11+19 = 13+17
        assert_eq!(count_pairs_for_n(30, 2, &is_prime), 3);

        // 20 = 3+17 = 7+13
        assert_eq!(count_pairs_for_n(20, 2, &is_prime), 2);

        // Restricted: base=10
        // 30 = 11+19 = 13+17 (both work, 7+23 excluded since 7<10)
        assert_eq!(count_pairs_for_n(30, 10, &is_prime), 2);

        // 20 = 3+17 (3<10, excluded) = 7+13 (7<10, excluded)
        // Need BOTH primes ≥ base, so no valid pairs
        assert_eq!(count_pairs_for_n(20, 10, &is_prime), 0);

        // 60 = 7+53, 13+47, 17+43, 19+41, 23+37, 29+31
        // With base=10: 7+53 excluded (7<10), so 5 pairs
        assert_eq!(count_pairs_for_n(60, 10, &is_prime), 5);

        // Restricted: base=20
        // 30 = 7+23 (excluded, 7<20) 11+19 (excluded, 11<20) 13+17 (excluded, 13<20)
        assert_eq!(count_pairs_for_n(30, 20, &is_prime), 0);
    }

    /// Validate HL predictions against actual Goldbach pair counts.
    ///
    /// This is the key end-to-end test: do the HL lambda values correlate
    /// with actual pair counts? The asymptotic formula is not exact for
    /// small n, but it should be positively correlated and in the right
    /// ballpark.
    #[test]
    fn test_hl_prediction_vs_actual_counts() {
        let limit = 2000;
        let spf = sieve_spf(limit);
        let is_prime = sieve_bool(limit);

        let mut predicted = Vec::new();
        let mut actual = Vec::new();

        // Collect data for even n from 10 to limit
        for n in (10..=limit).step_by(2) {
            let lambda = hl_goldbach_lambda(n, &spf, PairCount::Unordered);
            let pairs = count_pairs_for_n(n, 2, &is_prime);

            predicted.push(lambda);
            actual.push(pairs as f64);
        }

        // 1. Correlation should be strongly positive (HL tracks actual counts)
        let n = predicted.len();
        let mean_p: f64 = predicted.iter().sum::<f64>() / n as f64;
        let mean_a: f64 = actual.iter().sum::<f64>() / n as f64;

        let mut cov = 0.0f64;
        let mut var_p = 0.0f64;
        let mut var_a = 0.0f64;

        for i in 0..n {
            let dp = predicted[i] - mean_p;
            let da = actual[i] - mean_a;
            cov += dp * da;
            var_p += dp * dp;
            var_a += da * da;
        }

        let r = cov / (var_p.sqrt() * var_a.sqrt());
        assert!(
            r > 0.90,
            "HL prediction should be strongly correlated with actual counts, got r = {:.4}",
            r
        );

        // 2. Mean ratio (actual/predicted) should be near 1.0.
        //    The asymptotic formula overestimates for small n, so we expect
        //    the ratio to be somewhat less than 1.0, but not wildly off.
        let ratios: Vec<f64> = predicted
            .iter()
            .zip(actual.iter())
            .filter(|(p, _)| **p > 1.0) // skip cases where prediction is tiny
            .map(|(p, a)| a / p)
            .collect();

        let mean_ratio: f64 = ratios.iter().sum::<f64>() / ratios.len() as f64;
        assert!(
            mean_ratio > 0.5 && mean_ratio < 2.0,
            "Mean actual/predicted ratio should be between 0.5 and 2.0, got {:.4}",
            mean_ratio
        );
    }

    /// Validate that the truncated HL prediction tracks restricted pair counts.
    ///
    /// For a given base (minimum prime value), the truncated lambda should
    /// predict the restricted pair count better than the unrestricted lambda.
    #[test]
    fn test_truncated_prediction_tracks_restricted_counts() {
        let limit = 2000;
        let spf = sieve_spf(limit);
        let is_prime = sieve_bool(limit);
        let base = 50; // Only count pairs where both primes >= 50

        let mut trunc_pred = Vec::new();
        let mut actual = Vec::new();

        for n in ((2 * base)..=limit).step_by(2) {
            let lambda = hl_goldbach_lambda_truncated(n, base, &spf, PairCount::Unordered);
            let pairs = count_pairs_for_n(n, base, &is_prime);

            trunc_pred.push(lambda);
            actual.push(pairs as f64);
        }

        // The truncated prediction should correlate with restricted counts
        let n = trunc_pred.len();
        let mean_t: f64 = trunc_pred.iter().sum::<f64>() / n as f64;
        let mean_a: f64 = actual.iter().sum::<f64>() / n as f64;

        let mut cov = 0.0f64;
        let mut var_t = 0.0f64;
        let mut var_a = 0.0f64;

        for i in 0..n {
            let dt = trunc_pred[i] - mean_t;
            let da = actual[i] - mean_a;
            cov += dt * da;
            var_t += dt * dt;
            var_a += da * da;
        }

        let r = cov / (var_t.sqrt() * var_a.sqrt());
        assert!(
            r > 0.80,
            "Truncated HL should correlate with restricted counts, got r = {:.4}",
            r
        );
    }

    /// Verify the singular series multiplicative factor for highly composite n.
    ///
    /// For n with many odd prime factors, S2(n) should be noticeably > 1,
    /// reflecting that such n have more Goldbach representations.
    #[test]
    fn test_singular_series_highly_composite() {
        let spf = sieve_spf(10000);

        // n = 2310 = 2 * 3 * 5 * 7 * 11
        // S2(2310) = (3-1)/(3-2) * (5-1)/(5-2) * (7-1)/(7-2) * (11-1)/(11-2)
        //          = 2/1 * 4/3 * 6/5 * 10/9
        //          = 2 * 1.333 * 1.2 * 1.111 = 3.556
        let s2310 = singular_series_goldbach_multiplicative(2310, &spf);
        let expected = 2.0 * (4.0 / 3.0) * (6.0 / 5.0) * (10.0 / 9.0);
        assert!(
            (s2310 - expected).abs() < 1e-10,
            "S₂(2310) = {}, expected {}",
            s2310,
            expected
        );

        // Highly composite n should have higher pair counts per unit
        // than n with few prime factors, all else being equal
        assert!(s2310 > 3.0, "S₂(2310) should be > 3.0");

        // n = 4 = 2^2, only factor is 2 (which is excluded)
        // S2(4) = 1.0 (no odd prime factors)
        let s4 = singular_series_goldbach_multiplicative(4, &spf);
        assert!(
            (s4 - 1.0).abs() < 1e-12,
            "S₂(4) should be exactly 1.0, got {}",
            s4
        );

        // n with more odd prime factors should have higher S2
        assert!(
            s2310 > s4,
            "S₂(2310) = {} should exceed S₂(4) = {}",
            s2310,
            s4
        );
    }

    /// Verify predict_goldbach_pairs with empirical k_hat.
    ///
    /// Fit k_hat from actual data, then verify it produces reasonable predictions.
    #[test]
    fn test_predict_goldbach_pairs_with_fitted_khat() {
        let limit = 1000;
        let spf = sieve_spf(limit);
        let is_prime = sieve_bool(limit);

        // Compute actual pair counts for calibration
        let mut sum_ratio = 0.0f64;
        let mut count = 0;

        for n in (100..=limit).step_by(2) {
            let actual = count_pairs_for_n(n, 2, &is_prime) as f64;
            // Compute the HL prediction without the front constant
            let s2 = singular_series_goldbach_multiplicative(n, &spf);
            let ln = (n as f64).ln();
            let raw = s2 * (n as f64) / (ln * ln);

            if raw > 0.1 {
                sum_ratio += actual / raw;
                count += 1;
            }
        }

        let k_hat = sum_ratio / count as f64;

        // k_hat should be in the neighborhood of C2 (for unordered pairs)
        // or 2*C2 (for ordered pairs). Since count_pairs_for_n uses p<=q
        // (unordered), k_hat should be near C2.
        assert!(
            k_hat > 0.3 && k_hat < 1.5,
            "Fitted k_hat = {:.4}, expected near C₂ = {:.4}",
            k_hat,
            C2
        );

        // Now verify that predict_goldbach_pairs with this k_hat is reasonable
        let pred_500 = predict_goldbach_pairs(500, &spf, k_hat);
        let actual_500 = count_pairs_for_n(500, 2, &is_prime) as f64;

        // Allow generous tolerance (within 3x) since this is a heuristic
        assert!(
            pred_500 > actual_500 * 0.3 && pred_500 < actual_500 * 3.0,
            "Prediction {:.1} should be within 3x of actual {:.0} for n=500",
            pred_500,
            actual_500
        );
    }
}
