# Hardy-Littlewood Framework Implementation

**Date**: October 2025
**Status**: Production-ready
**Test Coverage**: 23/23 tests passing

> **Research Workflow**: The library functions documented here are used by [`tools/prime_unified_cli`](tools/README.md) for reproducible dataset generation. For current research, use the unified CLI rather than the experimental examples.

## Summary

This document describes the implementation of rigorous Hardy-Littlewood analysis tools for prime distribution studies. The framework implements established mathematical conventions with careful attention to detail, making results suitable for publication.

## Mathematical Rigor

### 1. Hardy-Littlewood Framework with Truncated Expectations

Complete implementation of the Hardy-Littlewood singular series for Goldbach pair analysis:

```
┌─────────────────────────────────────────────────────────────┐
│         HARDY-LITTLEWOOD FRAMEWORK COMPONENTS               │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Singular Series ──► S₂(n) ──► Expectation λ(n) ──► Pr[·] │
│  (multiplicative)   (analytic)  (poisson approx)           │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**Multiplicative singular series:**

$$S_2(n) = \prod_{\substack{p \mid n \\ p > 2}} \frac{p-1}{p-2}$$

This product accounts for local obstructions to Goldbach representations modulo each prime $p$ dividing $n$.

**Full expectation** (unrestricted Goldbach):

$$\lambda(n) = \kappa \cdot S_2(n) \cdot \frac{n}{(\ln n)^2}$$

where:
- $\kappa = 2C_2 \approx 1.320$ for ordered pairs $(p,q)$
- $\kappa = C_2 \approx 0.660$ for unordered pairs $\\{p,q\\}$
- $C_2 = \prod_{p>2} \left(1 - \frac{1}{(p-1)^2}\right) \approx 0.6601618158$

**Truncated expectation** (restricted to primes $\geq B$):

$$\lambda(n, B) = \kappa \cdot S_2(n) \cdot \sum_{x=B}^{n-B} \frac{1}{\ln x \cdot \ln(n-x)}$$

The summation form correctly accounts for the constraint that both primes must exceed threshold $B$, preventing systematic overprediction in bounded analyses.

**Coverage probability** (Poisson/Chen-Stein approximation):

$$\Pr[r(n) \geq 1] \approx 1 - e^{-\lambda}$$

where $r(n)$ is the number of Goldbach representations of $n$.

### 2. Natural Logarithms Throughout (base e)

All logarithmic calculations use natural logarithms (base e), consistent with Hardy-Littlewood literature:

- Implemented via Rust's `.ln()` method
- Documented in module-level comments
- Verified in unit tests against hand calculations
- Eliminates ambiguity present in log₁₀ vs ln notation

### 3. Effect Size Reporting

Implemented both parametric and non-parametric effect size measures:

**Hedges' g** (parametric, bias-corrected Cohen's d):
- Corrects small-sample bias in Cohen's d
- Standard interpretation thresholds implemented
- Appropriate for normally-distributed data

**Cliff's δ** (non-parametric, rank-based):
- Measures stochastic dominance
- Robust to outliers and non-normality
- Complements parametric measures

Both measures provide scale-free effect magnitudes that contextualize statistical significance.

### 4. Benjamini-Hochberg FDR Correction

Implemented Benjamini-Hochberg procedure for controlling False Discovery Rate in multiple comparisons:

- More powerful than Bonferroni correction for large test families
- Maintains proper monotonicity of adjusted p-values
- Essential when testing multiple bases or configurations

### 5. Regression Confidence Intervals

Implemented ordinary least squares regression with confidence intervals:

- Slope and intercept standard errors via t-distribution
- 95% confidence intervals by default (configurable)
- Residual standard error reporting
- Appropriate for δ* drift analysis across digit lengths

### 6. Verification of $S_2(30) = 8/3$

Unit test verifies the singular series calculation to machine precision:

```rust
let s30 = singular_series_goldbach_multiplicative(30, &spf);
assert!((s30 - 8.0/3.0).abs() < 1e-12);
```

**Analytical derivation** for $n = 30 = 2 \times 3 \times 5$:

$$S_2(30) = \prod_{\substack{p \mid 30 \\ p > 2}} \frac{p-1}{p-2} = \frac{3-1}{3-2} \times \frac{5-1}{5-2}$$

$$= \frac{2}{1} \times \frac{4}{3} = \frac{8}{3} \approx 2.666\overline{6}$$

**Visual breakdown:**

```
  n = 30 = 2·3·5
           │ └─┬─┘
           │   └─── Odd prime divisors: 3, 5
           │
           └─────── Factor p=2 excluded (formula uses p>2)

  S₂(30) = (3-1)/(3-2) × (5-1)/(5-2)
           ─────┬──────   ─────┬──────
                2              4/3

  Product: 2 × 4/3 = 8/3 ✓
```

This serves as a reference verification for the implementation with exact rational result.

### 7. Type-Safe Pair Counting Convention

Introduced `PairCount` enum to disambiguate ordered vs unordered pair counting:

```rust
pub enum PairCount {
    Ordered,   // counts (p,q) and (q,p) separately
    Unordered, // counts {p,q} once
}
```

This makes the counting convention explicit in function signatures:
- Ordered: κ = 2·C₂ ≈ 1.320
- Unordered: κ = C₂ ≈ 0.660

Eliminates a common source of ambiguity in Goldbach analyses.

## Implementation Details

### Core Functions (`src/hzlib/hardy_littlewood.rs`)

- `singular_series_goldbach_multiplicative(n, spf)` - Returns S₂(n) only
- `hl_goldbach_lambda(n, spf, pairing)` - Full unrestricted expectation
- `hl_goldbach_lambda_truncated(n, lo, spf, pairing)` - Restricted expectation
- `goldbach_coverage_from_lambda(lambda)` - Poisson coverage mapping

### Statistical Functions (`src/hzlib/stats.rs`)

- `hedges_g(a, b)` - Bias-corrected Cohen's d
- `cliffs_delta(a, b)` - Rank-based effect size
- `spearman_rho(xs, ys)` - Monotonic correlation
- `linreg_with_ci(xs, ys, conf)` - Regression with confidence intervals
- `benjamini_hochberg(pvals, fdr)` - Multiple comparison correction

### Updated Examples

**goldbach_hl_analysis.rs**:
- Uses truncated Hardy-Littlewood expectations
- Reports Hedges' g and Cliff's δ
- Applies Benjamini-Hochberg when appropriate
- Documents natural log usage

**hz_phase2_density.rs**:
- Includes 95% CI on regression slopes
- Reports relative slopes (per half-band width)
- Ready for correlation with membrane success rates

## Testing

### Unit Test Coverage

- 11 tests for Hardy-Littlewood functions
- 12 tests for statistical functions
- All tests pass with strict tolerances

### Key Verifications

- Constants verified to 15 decimal places
- S₂(30) = 8/3 exactly (to 1e-12)
- Ordered/unordered ratio = 2.0 (to 1e-2)
- Truncation monotonicity: λ(n,B₁) ≥ λ(n,B₂) when B₁ ≤ B₂
- Coverage mapping: λ=1 → coverage ≈ 0.632 (to 1e-10)

## Documentation

### Module-Level Documentation

- `hardy_littlewood.rs`: 50+ lines of mathematical conventions
- `stats.rs`: Complete function documentation with examples
- `density.rs`: 40+ lines explaining rad(b) vs φ(b)

### User Documentation

- CLAUDE.md: 175-line Hardy-Littlewood section
- Mathematical foundations
- Implementation guide with code examples
- Verification standards
- Running instructions for both examples

## Verification Standards

All Hardy-Littlewood analyses using this framework should:

1. Use natural logarithms (base e)
2. Specify pair counting convention explicitly
3. Apply truncation for restricted problems
4. Report effect sizes alongside p-values
5. Apply FDR correction when testing multiple hypotheses
6. Include confidence intervals on regression estimates

## Known Limitations

1. Truncated expectation uses discrete sum rather than continuous integral (conservative)
2. t-critical values use approximation for df ≥ 3 (adequate for typical analyses)
3. Denominators in density analysis use b-1 approximation (see density.rs for exact method)

## Future Enhancements

1. Implement exact rad(b)-based denominators with O(1) bin queries
2. Add Theil-Sen robust regression as alternative to OLS
3. Implement bootstrap confidence intervals for non-normal data
4. Add power analysis functions for experimental design

## References

Hardy, G. H., & Littlewood, J. E. (1923). Some problems of 'Partitio numerorum'; III: On the expression of a number as a sum of primes.

Montgomery, H. L., & Vaughan, R. C. (2007). Multiplicative Number Theory I: Classical Theory.

Hedges, L. V. (1981). Distribution theory for Glass's estimator of effect size and related estimators.

Cliff, N. (1993). Dominance statistics: Ordinal analyses to answer ordinal questions.

Benjamini, Y., & Hochberg, Y. (1995). Controlling the false discovery rate: a practical and powerful approach to multiple testing.
