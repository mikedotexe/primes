//! Statistical Analysis Tools
//!
//! Provides robust statistical tests for comparing prime generation methods:
//! - Linear regression
//! - Welch's t-test (unequal variances)
//! - Permutation tests (non-parametric, size-binned)

use rand::seq::SliceRandom;
use rand::SeedableRng;
use statrs::distribution::{ContinuousCDF, StudentsT};
use std::collections::BTreeMap;

/// Linear regression: y ≈ slope·x + intercept
///
/// # Arguments
/// * `xs` - Independent variable values
/// * `ys` - Dependent variable values (must be same length as xs)
///
/// # Returns
/// Tuple of (slope, intercept, r²):
/// - `slope`: Best-fit slope
/// - `intercept`: Y-intercept
/// - `r²`: Coefficient of determination (goodness of fit)
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::linreg;
/// let xs = vec![1.0, 2.0, 3.0, 4.0];
/// let ys = vec![2.0, 4.0, 6.0, 8.0]; // y = 2x
/// let (slope, intercept, r2) = linreg(&xs, &ys);
/// assert!((slope - 2.0).abs() < 0.01);
/// assert!(intercept.abs() < 0.01);
/// assert!(r2 > 0.99);
/// ```
pub fn linreg(xs: &[f64], ys: &[f64]) -> (f64, f64, f64) {
    let n = xs.len();
    if n == 0 || ys.len() != n {
        return (0.0, 0.0, 0.0);
    }

    let (mut sx, mut sy, mut sxx, mut sxy, mut syy) = (0.0, 0.0, 0.0, 0.0, 0.0);

    for i in 0..n {
        let x = xs[i];
        let y = ys[i];
        sx += x;
        sy += y;
        sxx += x * x;
        sxy += x * y;
        syy += y * y;
    }

    let nf = n as f64;
    let denom = nf * sxx - sx * sx;

    if denom.abs() < 1e-18 {
        return (0.0, sy / nf, 0.0);
    }

    let slope = (nf * sxy - sx * sy) / denom;
    let intercept = (sy - slope * sx) / nf;

    let ss_tot = syy - sy * sy / nf;
    let ss_res = ss_tot - slope * (sxy - sx * sy / nf);
    let r2 = if ss_tot <= 0.0 {
        0.0
    } else {
        1.0 - ss_res / ss_tot
    };

    (slope, intercept, r2)
}

/// Linear regression with confidence intervals
///
/// Performs OLS regression and computes standard errors and confidence intervals
/// for slope and intercept using t-distribution.
///
/// # Arguments
/// * `xs` - Independent variable values
/// * `ys` - Dependent variable values (must be same length as xs)
/// * `confidence` - Confidence level (e.g., 0.95 for 95% CI)
///
/// # Returns
/// Tuple of (slope, intercept, r², slope_ci, intercept_ci, residual_se):
/// - `slope`: Best-fit slope
/// - `intercept`: Y-intercept
/// - `r²`: Coefficient of determination
/// - `slope_ci`: Half-width of confidence interval for slope
/// - `intercept_ci`: Half-width of confidence interval for intercept
/// - `residual_se`: Standard error of residuals
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::linreg_with_ci;
/// let xs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let ys = vec![2.1, 3.9, 6.1, 7.9, 10.1]; // ~2x with noise
/// let (slope, intercept, r2, slope_ci, intercept_ci, se) = linreg_with_ci(&xs, &ys, 0.95);
/// assert!((slope - 2.0).abs() < 0.2);
/// assert!(slope_ci > 0.0); // CI should be positive
/// ```
pub fn linreg_with_ci(xs: &[f64], ys: &[f64], confidence: f64) -> (f64, f64, f64, f64, f64, f64) {
    let n = xs.len();
    if n < 3 || ys.len() != n {
        return (f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN);
    }

    // Basic regression
    let (slope, intercept, r2) = linreg(xs, ys);

    // Compute residuals and standard error
    let mut ss_res = 0.0_f64;
    for i in 0..n {
        let y_pred = slope * xs[i] + intercept;
        let residual = ys[i] - y_pred;
        ss_res += residual * residual;
    }

    let df = (n - 2) as f64;
    let mse = ss_res / df; // Mean squared error
    let residual_se = mse.sqrt();

    // Standard errors for coefficients
    let mut sum_x = 0.0_f64;
    let mut sum_xx = 0.0_f64;
    for &x in xs {
        sum_x += x;
        sum_xx += x * x;
    }

    let mean_x = sum_x / n as f64;
    let sxx = sum_xx - n as f64 * mean_x * mean_x;

    if sxx <= 0.0 {
        return (slope, intercept, r2, f64::NAN, f64::NAN, residual_se);
    }

    let se_slope = (mse / sxx).sqrt();
    let se_intercept = (mse * (1.0 / n as f64 + mean_x * mean_x / sxx)).sqrt();

    // t-critical value for two-tailed test
    let alpha = 1.0 - confidence;
    let t_crit = t_critical_value(df, alpha / 2.0);

    let slope_ci = t_crit * se_slope;
    let intercept_ci = t_crit * se_intercept;

    (slope, intercept, r2, slope_ci, intercept_ci, residual_se)
}

/// Approximate t-critical value using Wilson-Hilferty approximation
///
/// For a two-tailed test with significance level alpha/2.
/// Good approximation for df ≥ 3.
fn t_critical_value(df: f64, alpha_half: f64) -> f64 {
    if df < 3.0 {
        // For very small df, use conservative approximations
        return match df as i32 {
            1 => 12.706, // Common approximation for df=1, alpha=0.025
            2 => 4.303,  // df=2, alpha=0.025
            _ => 3.182,  // fallback
        };
    }

    // For larger df, use normal approximation with correction
    // This is a simplified approach; for exact values would use Beta/Gamma functions
    let z = match alpha_half {
        a if (a - 0.025).abs() < 0.001 => 1.96,  // 95% CI
        a if (a - 0.005).abs() < 0.001 => 2.576, // 99% CI
        a if (a - 0.05).abs() < 0.001 => 1.645,  // 90% CI
        _ => 1.96,                               // default to 95%
    };

    // Adjust for finite df using approximation
    z * (1.0 + (z * z + 1.0) / (4.0 * df)).sqrt()
}

/// Welch's t-test for comparing two samples with unequal variances
///
/// Tests null hypothesis: mean(a) = mean(b)
///
/// # Arguments
/// * `a` - First sample
/// * `b` - Second sample
///
/// # Returns
/// Tuple of (t_statistic, p_value):
/// - `t_statistic`: Welch's t value
/// - `p_value`: Two-tailed p-value (NaN if insufficient data)
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::welch_t;
/// let a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let b = vec![6.0, 7.0, 8.0, 9.0, 10.0];
/// let (t, p) = welch_t(&a, &b);
/// assert!(p < 0.001); // Significant difference
/// ```
pub fn welch_t(a: &[f64], b: &[f64]) -> (f64, f64) {
    let n1 = a.len() as f64;
    let n2 = b.len() as f64;

    if n1 < 2.0 || n2 < 2.0 {
        return (f64::NAN, f64::NAN);
    }

    let m1 = a.iter().copied().sum::<f64>() / n1;
    let m2 = b.iter().copied().sum::<f64>() / n2;

    let v1 = a.iter().map(|x| (x - m1) * (x - m1)).sum::<f64>() / (n1 - 1.0);
    let v2 = b.iter().map(|x| (x - m2) * (x - m2)).sum::<f64>() / (n2 - 1.0);

    let se2 = v1 / n1 + v2 / n2;

    if se2 <= 0.0 {
        return (f64::NAN, f64::NAN);
    }

    let t = (m1 - m2) / se2.sqrt();

    // Welch-Satterthwaite degrees of freedom
    let num = se2 * se2;
    let den = (v1 * v1) / (n1 * n1 * (n1 - 1.0)) + (v2 * v2) / (n2 * n2 * (n2 - 1.0));
    let dof = num / den;

    if !dof.is_finite() || dof <= 1.0 {
        return (t, f64::NAN);
    }

    let dist = StudentsT::new(0.0, 1.0, dof).unwrap();
    let p = 2.0 * (1.0 - dist.cdf(t.abs()));

    (t, p)
}

/// Permutation test with size-binning
///
/// Tests if "complementary" bases differ from others, controlling for base magnitude.
/// Bins bases by size to ensure permutations respect size stratification.
///
/// # Arguments
/// * `data` - Tuples of (base, is_complementary, metric_value)
/// * `binsize` - Size of bins for grouping bases (e.g., 50)
/// * `perms` - Number of permutations (e.g., 2000)
///
/// # Returns
/// p-value: Pr[|permuted_diff| ≥ |observed_diff|]
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::permutation_pvalue;
/// let data = vec![
///     (66, true, 0.95),  // Complementary base
///     (70, true, 0.93),
///     (60, false, 0.80), // Non-complementary
///     (68, false, 0.82),
/// ];
/// let p = permutation_pvalue(&data, 10, 1000);
/// // Low p-value → complementary bases significantly different
/// ```
pub fn permutation_pvalue(
    data: &[(usize, bool, f64)], // (base, is_complementary, metric)
    binsize: usize,
    perms: usize,
) -> f64 {
    // Bin by base magnitude
    let mut bins: BTreeMap<usize, Vec<(bool, f64)>> = BTreeMap::new();

    for (b, comp, m) in data {
        let key = (*b / binsize) * binsize;
        bins.entry(key).or_default().push((*comp, *m));
    }

    // Observed statistic
    let mut obs_c = Vec::new();
    let mut obs_n = Vec::new();

    for (_k, v) in bins.iter() {
        for (c, m) in v {
            if *c {
                obs_c.push(*m);
            } else {
                obs_n.push(*m);
            }
        }
    }

    let obs_diff = mean(&obs_c) - mean(&obs_n);

    // Permutation distribution
    let mut ge = 0usize;
    let mut rng = rand::rngs::StdRng::seed_from_u64(20250123);

    for _ in 0..perms {
        let mut sc = Vec::new();
        let mut sn = Vec::new();

        for (_k, v) in bins.iter() {
            let mut labs: Vec<bool> = v.iter().map(|(c, _)| *c).collect();
            let mets: Vec<f64> = v.iter().map(|(_, m)| *m).collect();

            labs.shuffle(&mut rng);

            for (lb, m) in labs.iter().zip(mets.iter()) {
                if *lb {
                    sc.push(*m);
                } else {
                    sn.push(*m);
                }
            }
        }

        let diff = mean(&sc) - mean(&sn);
        if diff.abs() >= obs_diff.abs() {
            ge += 1;
        }
    }

    (ge as f64 + 1.0) / (perms as f64 + 1.0)
}

/// Compute mean of values
fn mean(x: &[f64]) -> f64 {
    if x.is_empty() {
        return f64::NAN;
    }
    x.iter().copied().sum::<f64>() / x.len() as f64
}

/// Compute variance (with Bessel's correction: divide by n-1)
fn variance(x: &[f64]) -> f64 {
    let n = x.len() as f64;
    if n < 2.0 {
        return f64::NAN;
    }

    let m = mean(x);
    let sum_sq = x.iter().map(|v| (v - m) * (v - m)).sum::<f64>();
    sum_sq / (n - 1.0)
}

/// Compute standard deviation
#[allow(dead_code)]
fn std_dev(x: &[f64]) -> f64 {
    variance(x).sqrt()
}

/// Hedges' g effect size (bias-corrected Cohen's d)
///
/// Measures standardized mean difference between two groups.
/// Unlike Cohen's d, Hedges' g corrects for small sample bias.
///
/// Interpretation (rough guidelines):
/// - |g| < 0.2: negligible
/// - 0.2 ≤ |g| < 0.5: small
/// - 0.5 ≤ |g| < 0.8: medium
/// - |g| ≥ 0.8: large
///
/// # Arguments
/// * `a` - First sample
/// * `b` - Second sample
///
/// # Returns
/// Hedges' g effect size (NaN if insufficient data)
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::hedges_g;
/// let a = vec![1.0, 2.0, 3.0, 4.0];
/// let b = vec![5.0, 6.0, 7.0, 8.0];
/// let g = hedges_g(&a, &b);
/// assert!(g < -1.0); // Large negative effect (a < b)
/// ```
pub fn hedges_g(a: &[f64], b: &[f64]) -> f64 {
    let n1 = a.len() as f64;
    let n2 = b.len() as f64;

    if n1 < 2.0 || n2 < 2.0 {
        return f64::NAN;
    }

    let m1 = mean(a);
    let m2 = mean(b);

    let v1 = variance(a);
    let v2 = variance(b);

    // Pooled standard deviation
    let s_pooled = (((n1 - 1.0) * v1 + (n2 - 1.0) * v2) / (n1 + n2 - 2.0)).sqrt();

    if s_pooled == 0.0 {
        return f64::NAN;
    }

    // Cohen's d
    let d = (m1 - m2) / s_pooled;

    // Hedges' correction factor
    let df = n1 + n2 - 2.0;
    let j = 1.0 - (3.0 / (4.0 * df - 1.0));

    d * j
}

/// Cliff's delta effect size (ordinal/rank-based)
///
/// Non-parametric measure of how often values in one group
/// exceed values in the other. Robust to outliers and non-normality.
///
/// Returns δ ∈ [-1, 1] where:
/// - δ = 1: all values in a > all values in b
/// - δ = 0: distributions overlap completely (stochastic equivalence)
/// - δ = -1: all values in a < all values in b
///
/// Interpretation (rough guidelines):
/// - |δ| < 0.15: negligible
/// - 0.15 ≤ |δ| < 0.33: small
/// - 0.33 ≤ |δ| < 0.47: medium
/// - |δ| ≥ 0.47: large
///
/// # Arguments
/// * `a` - First sample
/// * `b` - Second sample
///
/// # Returns
/// Cliff's delta ∈ [-1, 1] (NaN if either sample is empty)
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::cliffs_delta;
/// let a = vec![1.0, 2.0, 3.0];
/// let b = vec![4.0, 5.0, 6.0];
/// let delta = cliffs_delta(&a, &b);
/// assert_eq!(delta, -1.0); // All a < all b
/// ```
pub fn cliffs_delta(a: &[f64], b: &[f64]) -> f64 {
    if a.is_empty() || b.is_empty() {
        return f64::NAN;
    }

    let mut more = 0isize;
    let mut less = 0isize;

    for &x in a {
        for &y in b {
            if x > y {
                more += 1;
            } else if x < y {
                less += 1;
            }
        }
    }

    let total = (a.len() * b.len()) as f64;
    (more - less) as f64 / total
}

/// Spearman's rank correlation coefficient
///
/// Measures monotonic association between two variables.
/// Robust to outliers and does not assume linearity.
///
/// Returns ρ ∈ [-1, 1] where:
/// - ρ = 1: perfect monotonic increase
/// - ρ = 0: no monotonic correlation
/// - ρ = -1: perfect monotonic decrease
///
/// # Arguments
/// * `xs` - First variable
/// * `ys` - Second variable (must be same length as xs)
///
/// # Returns
/// Spearman's ρ (NaN if insufficient data or mismatched lengths)
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::spearman_rho;
/// let xs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let ys = vec![2.0, 4.0, 6.0, 8.0, 10.0]; // Perfect linear
/// let rho = spearman_rho(&xs, &ys);
/// assert!((rho - 1.0).abs() < 0.01); // Perfect positive correlation
/// ```
pub fn spearman_rho(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len();
    if n != ys.len() || n < 2 {
        return f64::NAN;
    }

    // Rank xs and ys
    let rank_x = rank(xs);
    let rank_y = rank(ys);

    // Pearson correlation of ranks
    pearson_correlation(&rank_x, &rank_y)
}

/// Convert values to ranks (average rank for ties)
fn rank(values: &[f64]) -> Vec<f64> {
    let n = values.len();
    let mut indexed: Vec<(usize, f64)> = values.iter().copied().enumerate().collect();

    // Sort by value
    indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut ranks = vec![0.0; n];
    let mut i = 0;

    while i < n {
        let mut j = i;
        // Find tied values
        while j < n && (indexed[j].1 - indexed[i].1).abs() < 1e-14 {
            j += 1;
        }

        // Average rank for ties
        let avg_rank = ((i + 1 + j) as f64) / 2.0;

        for k in i..j {
            ranks[indexed[k].0] = avg_rank;
        }

        i = j;
    }

    ranks
}

/// Pearson correlation coefficient
fn pearson_correlation(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len() as f64;
    if xs.len() != ys.len() || n < 2.0 {
        return f64::NAN;
    }

    let mx = mean(xs);
    let my = mean(ys);

    let mut cov = 0.0;
    let mut var_x = 0.0;
    let mut var_y = 0.0;

    for i in 0..xs.len() {
        let dx = xs[i] - mx;
        let dy = ys[i] - my;
        cov += dx * dy;
        var_x += dx * dx;
        var_y += dy * dy;
    }

    if var_x == 0.0 || var_y == 0.0 {
        return f64::NAN;
    }

    cov / (var_x * var_y).sqrt()
}

/// Benjamini-Hochberg FDR correction for multiple comparisons
///
/// Adjusts p-values to control the False Discovery Rate.
/// More powerful than Bonferroni for large numbers of tests.
///
/// # Arguments
/// * `pvalues` - Raw p-values from multiple tests
/// * `fdr` - Target false discovery rate (typically 0.05)
///
/// # Returns
/// Adjusted p-values (same length as input)
///
/// # Example
/// ```
/// use prime_physics_engine::hzlib::benjamini_hochberg;
/// let pvalues = vec![0.001, 0.01, 0.03, 0.10, 0.50];
/// let adjusted = benjamini_hochberg(&pvalues, 0.05);
/// // First few should remain significant after adjustment
/// assert!(adjusted[0] < 0.05);
/// assert!(adjusted[1] < 0.05);
/// ```
pub fn benjamini_hochberg(pvalues: &[f64], _fdr: f64) -> Vec<f64> {
    let m = pvalues.len();
    if m == 0 {
        return Vec::new();
    }

    // Create indexed copy and sort
    let mut indexed: Vec<(usize, f64)> = pvalues.iter().copied().enumerate().collect();
    indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut adjusted = vec![0.0; m];

    // BH procedure: adjust from largest to smallest
    let mut min_adj = 1.0_f64;

    for k in (0..m).rev() {
        let (orig_idx, p) = indexed[k];
        let rank = (k + 1) as f64;
        let adj = (p * m as f64 / rank).min(1.0_f64);

        // Enforce monotonicity
        min_adj = min_adj.min(adj);
        adjusted[orig_idx] = min_adj;
    }

    adjusted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linreg() {
        let xs = vec![1.0, 2.0, 3.0, 4.0];
        let ys = vec![2.0, 4.0, 6.0, 8.0]; // y = 2x
        let (slope, intercept, r2) = linreg(&xs, &ys);

        assert!((slope - 2.0).abs() < 0.01);
        assert!(intercept.abs() < 0.01);
        assert!(r2 > 0.99);
    }

    #[test]
    fn test_linreg_with_ci() {
        // Perfect fit (no noise)
        let xs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let ys = vec![2.0, 4.0, 6.0, 8.0, 10.0]; // y = 2x
        let (slope, intercept, r2, _slope_ci, _intercept_ci, se) = linreg_with_ci(&xs, &ys, 0.95);

        assert!((slope - 2.0).abs() < 0.01, "Slope should be ~2");
        assert!(intercept.abs() < 0.01, "Intercept should be ~0");
        assert!(r2 > 0.99, "R² should be near 1");
        assert!(se < 0.01, "Residual SE should be tiny for perfect fit");

        // With noise
        let ys_noisy = vec![2.1, 3.9, 6.1, 7.9, 10.1];
        let (slope2, _int2, r2_2, slope_ci2, _intercept_ci2, se2) =
            linreg_with_ci(&xs, &ys_noisy, 0.95);

        assert!((slope2 - 2.0).abs() < 0.2, "Slope should be close to 2");
        assert!(slope_ci2 > 0.0, "CI should be positive");
        assert!(se2 > 0.0, "SE should be positive with noise");
        assert!(r2_2 < r2, "R² should be lower with noise");

        // 99% CI should be wider than 95%
        let (_s3, _i3, _r3, slope_ci3, _ici3, _se3) = linreg_with_ci(&xs, &ys_noisy, 0.99);
        assert!(slope_ci3 > slope_ci2, "99% CI should be wider than 95%");

        // Insufficient data
        let (s, i, r, sci, ici, se) = linreg_with_ci(&vec![1.0, 2.0], &vec![2.0, 4.0], 0.95);
        assert!(
            s.is_nan() && i.is_nan() && r.is_nan(),
            "Should return NaN for n<3"
        );
    }

    #[test]
    fn test_welch_t() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let b = vec![6.0, 7.0, 8.0, 9.0, 10.0];
        let (t, p) = welch_t(&a, &b);

        assert!(t < -3.0); // Significant negative t
        assert!(p < 0.01); // Highly significant
    }

    #[test]
    fn test_permutation() {
        let data = vec![
            (66, true, 0.95),
            (70, true, 0.93),
            (60, false, 0.80),
            (68, false, 0.82),
        ];

        let p = permutation_pvalue(&data, 10, 100);
        // With such strong separation, p should be small
        assert!(p < 0.5);
    }

    #[test]
    fn test_mean() {
        assert_eq!(mean(&vec![1.0, 2.0, 3.0, 4.0, 5.0]), 3.0);
        assert_eq!(mean(&vec![10.0]), 10.0);
        assert!(mean(&vec![]).is_nan());
    }

    #[test]
    fn test_variance() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let v = variance(&x);
        // Var = E[(X - μ)²] with Bessel correction
        // For 1..5: mean=3, var = [(1-3)² + (2-3)² + (3-3)² + (4-3)² + (5-3)²]/(5-1)
        //                       = [4 + 1 + 0 + 1 + 4]/4 = 10/4 = 2.5
        assert!((v - 2.5).abs() < 1e-10);

        // Insufficient data
        assert!(variance(&vec![1.0]).is_nan());
        assert!(variance(&vec![]).is_nan());
    }

    #[test]
    fn test_hedges_g() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let b = vec![6.0, 7.0, 8.0, 9.0, 10.0];

        let g = hedges_g(&a, &b);
        // Large negative effect (a < b)
        assert!(g < -1.5, "Hedges' g = {}, expected < -1.5", g);

        // Same groups should give ~0
        let g_same = hedges_g(&a, &a);
        assert!((g_same).abs() < 0.01, "Same groups should give g ≈ 0");

        // Insufficient data
        assert!(hedges_g(&vec![1.0], &vec![2.0, 3.0]).is_nan());
    }

    #[test]
    fn test_cliffs_delta() {
        // Perfect separation
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        assert_eq!(cliffs_delta(&a, &b), -1.0, "All a < b should give δ = -1");
        assert_eq!(cliffs_delta(&b, &a), 1.0, "All b > a should give δ = 1");

        // Complete overlap
        assert_eq!(cliffs_delta(&a, &a), 0.0, "Same groups should give δ = 0");

        // Partial overlap
        let c = vec![2.0, 3.0, 4.0];
        let d = cliffs_delta(&a, &c);
        assert!(d < 0.0 && d > -1.0, "Partial overlap: -1 < δ < 0");

        // Empty sets
        assert!(cliffs_delta(&vec![], &b).is_nan());
        assert!(cliffs_delta(&a, &vec![]).is_nan());
    }

    #[test]
    fn test_spearman_rho() {
        // Perfect monotonic increase
        let xs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let ys = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let rho = spearman_rho(&xs, &ys);
        assert!(
            (rho - 1.0).abs() < 0.01,
            "Perfect increase should give ρ ≈ 1"
        );

        // Perfect monotonic decrease
        let ys_dec = vec![10.0, 8.0, 6.0, 4.0, 2.0];
        let rho_dec = spearman_rho(&xs, &ys_dec);
        assert!(
            (rho_dec + 1.0).abs() < 0.01,
            "Perfect decrease should give ρ ≈ -1"
        );

        // No correlation
        let ys_rand = vec![3.0, 1.0, 4.0, 2.0, 5.0];
        let rho_rand = spearman_rho(&xs, &ys_rand);
        assert!(rho_rand.abs() < 1.0, "Random should give |ρ| < 1");

        // Mismatched lengths
        assert!(spearman_rho(&xs, &vec![1.0, 2.0]).is_nan());

        // Insufficient data
        assert!(spearman_rho(&vec![1.0], &vec![2.0]).is_nan());
    }

    #[test]
    fn test_benjamini_hochberg() {
        let pvalues = vec![0.001, 0.01, 0.03, 0.10, 0.50];
        let adjusted = benjamini_hochberg(&pvalues, 0.05);

        // First few should remain significant
        assert!(adjusted[0] < 0.05, "p=0.001 should remain significant");
        assert!(adjusted[1] < 0.05, "p=0.01 should remain significant");

        // Larger p-values should be adjusted upward
        assert!(
            adjusted[3] >= pvalues[3],
            "p=0.10 should be adjusted up or stay same"
        );

        // Length should match
        assert_eq!(adjusted.len(), pvalues.len());

        // Monotonicity: adjusted[i] <= adjusted[j] if pvalues[i] <= pvalues[j]
        let mut sorted_p = pvalues.clone();
        sorted_p.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut sorted_adj: Vec<(f64, f64)> = pvalues
            .iter()
            .zip(adjusted.iter())
            .map(|(p, a)| (*p, *a))
            .collect();
        sorted_adj.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        for i in 1..sorted_adj.len() {
            assert!(
                sorted_adj[i].1 >= sorted_adj[i - 1].1,
                "BH adjusted p-values should be monotonic"
            );
        }

        // Empty input
        assert_eq!(benjamini_hochberg(&vec![], 0.05).len(), 0);
    }

    #[test]
    fn test_rank() {
        // Simple ranking
        let vals = vec![3.0, 1.0, 4.0, 2.0, 5.0];
        let ranks = rank(&vals);
        assert_eq!(ranks, vec![3.0, 1.0, 4.0, 2.0, 5.0]);

        // With ties (average ranks)
        let vals_tie = vec![1.0, 2.0, 2.0, 4.0];
        let ranks_tie = rank(&vals_tie);
        // 1→rank 1, 2→ranks 2&3 (avg=2.5), 4→rank 4
        assert_eq!(ranks_tie, vec![1.0, 2.5, 2.5, 4.0]);
    }
}
