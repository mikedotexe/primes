# Hardy-Littlewood Framework - Quick Reference

> **Note**: For current research workflows, see [`tools/prime_unified_cli`](tools/README.md), which supersedes the experimental examples below and provides unified CCRT/midpoint-density analysis with locked CSV schemas.

## Key Claims (Publication-Ready)

1. **Implemented Hardy-Littlewood framework with truncated expectations**
   - Full expectation: λ(n) = κ·S₂(n)·n/(ln n)²
   - Truncated: λ(n,B) = κ·S₂(n)·Σ_{x=B}^{n-B} 1/(ln x · ln(n-x))
   - Prevents overprediction in restricted Goldbach analysis

2. **Used natural logarithms throughout (base e)**
   - All calculations via `.ln()` method
   - Documented in module headers
   - Verified in unit tests

3. **Reported parametric (Hedges' g) and non-parametric (Cliff's δ) effect sizes**
   - Hedges' g: bias-corrected Cohen's d
   - Cliff's δ: rank-based stochastic dominance
   - Both scale-free, complement p-values

4. **Applied Benjamini-Hochberg FDR correction for multiple comparisons**
   - Controls False Discovery Rate
   - More powerful than Bonferroni
   - Essential for multi-base testing

5. **Provided 95% confidence intervals on regression slopes**
   - OLS standard errors via t-distribution
   - Reports slope ± CI
   - Critical for δ* drift analysis

6. **Verified S₂(30) = 8/3 to machine precision**
   - S₂(30) = (3-1)/(3-2) × (5-1)/(5-2) = 8/3
   - Tested to 1e-12 tolerance
   - Reference verification for implementation

7. **Distinguished ordered vs unordered pair counting with type-safe enums**
   - `PairCount::Ordered`: κ = 2·C₂ ≈ 1.320
   - `PairCount::Unordered`: κ = C₂ ≈ 0.660
   - Eliminates common ambiguity

## Constants

```
C₂ = ∏_{p>2} (1 - 1/(p-1)²) ≈ 0.6601618158468696
```

## Core Functions

```rust
// Multiplicative singular series (without κ)
singular_series_goldbach_multiplicative(n, spf) → S₂(n)

// Full unrestricted expectation
hl_goldbach_lambda(n, spf, PairCount::Unordered) → λ

// Restricted expectation (both primes ≥ lo)
hl_goldbach_lambda_truncated(n, lo, spf, PairCount::Unordered) → λ(n,lo)

// Poisson coverage
goldbach_coverage_from_lambda(lambda) → 1 - e^(-λ)
```

## Statistical Functions

```rust
// Effect sizes
hedges_g(a, b)        // parametric
cliffs_delta(a, b)    // non-parametric

// Correlation
spearman_rho(xs, ys)  // monotonic, robust

// Regression with CI
linreg_with_ci(xs, ys, 0.95) → (slope, intercept, r², slope_ci, int_ci, se)

// Multiple comparisons
benjamini_hochberg(pvalues, 0.05) → adjusted_pvalues
```

## Test Coverage

- 23 unit tests, all passing
- Constants to 15 decimal places
- S₂(30) = 8/3 to 1e-12
- Coverage mapping to 1e-10

## Documentation

- `HARDY_LITTLEWOOD_IMPLEMENTATION.md` - Full technical details
- `CLAUDE.md` - User guide with examples
- `src/hzlib/hardy_littlewood.rs` - API documentation
- `src/hzlib/stats.rs` - Statistical functions
- `src/hzlib/density.rs` - Radical explanation

## Example Usage

The examples are in the experimental folder and should be run directly:

```bash
# Goldbach analysis with truncated HL
cd examples/experimental
cargo run --release --bin goldbach_hl_analysis -- --min-base 60 --max-base 80 --window 1000

# Or compile and run directly:
rustc goldbach_hl_analysis.rs --edition 2021 -L ../../target/release/deps \
  --extern prime_physics_engine=../../target/release/libprime_physics_engine.rlib
./goldbach_hl_analysis --min-base 60 --max-base 80 --window 1000

# Phase 2 density with CI on slopes
rustc hz_phase2_density.rs --edition 2021 -L ../../target/release/deps \
  --extern prime_physics_engine=../../target/release/libprime_physics_engine.rlib
./hz_phase2_density --bases 6,30,10 --limit 200000000 --bins 200
```

Note: These are standalone Rust files in `examples/experimental/` that import the hzlib module. Build the main library first with `cargo build --release`, then compile these examples directly.
