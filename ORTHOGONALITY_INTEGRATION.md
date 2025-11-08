# Orthogonality Testing: Integration Plan

**Date**: 2025-11-08
**Context**: Extension of spectral analysis to Hardy-Littlewood normalization
**Purpose**: Connect structural scores to HL framework through orthogonality testing

---

## Overview

We've received TLA+ and Agda specifications for testing orthogonality after Hardy-Littlewood normalization. This document explains how these concepts integrate with our existing work and outlines the implementation path.

---

## The Core Concept

**Observation**: Structural heuristics (like Babylonian scores or our regularity scores) correlate with raw success metrics but become orthogonal after proper normalization.

**Mathematical framework**:
```
1. Structural Score S(config)   - captures systematic bias
2. Raw Success R(config)         - actual empirical results
3. HL-Normalized Success H(config) - R divided by theoretical expectation

Predictions:
- Corr(S, R) > 0   (positive correlation)
- Corr(S, H) ≈ 0   (orthogonality)
```

**Interpretation**: If orthogonality holds, the structural score captures exactly the bias that HL theory predicts. After normalizing out theoretical expectations, only random noise remains.

---

## Application to Our Work

### What We Have

**Structural Scores (from spectral analysis)**:
- Regularity score for (base, divisor) pairs
- Based on frequency distribution uniformity
- Range [0,1], where 1 = perfect regularity

**Raw Success**:
- Empirical prime generation success rates
- Base 6: 33%, Base 10: 18.5%, etc.
- Measured through actual primality testing

**HL Framework** (existing in src/hzlib/hardy_littlewood.rs):
- Singular series computation
- Goldbach pair expectations
- Coverage probability estimation

### What We Need to Add

**HL-Normalized Membrane Success**:

For a membrane configuration (base, outer, inner, k):
```rust
// Current: raw success rate
raw_success = (primes_generated / total_tested) * 100.0

// Proposed: HL-normalized success
let S = singular_series_membrane(base, outer, inner, k);
let theoretical_expected = S * baseline_rate;
hl_normalized = raw_success / theoretical_expected
```

**Challenge**: We need to define `singular_series_membrane` - what's the multiplicative correction for membrane structures?

---

## Theoretical Framework

### For Prime Pairs (Established)

Gap g between primes p and p+g:
```
S(g) = 2·C₂ · ∏_{p|k, p>2} (p-1)/(p-2)  where k = g/2
```

This accounts for divisibility constraints based on gap factorization.

### For Membranes (Proposed)

Configuration (base, outer, inner):
```
S(base, outer, inner) = ???

Factors to consider:
- rad(base) constraints (already formalized)
- gcd(outer, base) effects (residue collapse)
- coprimality requirements
- Digit position divisibility patterns
```

**Research question**: Can we derive a membrane singular series analogous to the gap singular series?

---

## Implementation Path

### Phase 1: Agda Formalization (Complete)

Created `Advanced/Orthogonality.agda`:
- Prime pair orthogonality framework
- Babylonian score computation
- Singular series for gaps
- Statistical testing (covariance, variance, correlation bounds)
- Placeholder for membrane adaptation

**Status**: Theoretical framework complete, pending computational implementation.

### Phase 2: Rust Proof-of-Concept (Planned)

Create `examples/membrane_orthogonality.rs`:

```rust
// 1. Compute regularity scores for test configurations
let configs = test_configurations();
let regularity_scores: Vec<f64> = configs.iter()
    .map(|c| spectral_regularity(c.base, 3))
    .collect();

// 2. Get raw success rates (from empirical data or testing)
let raw_success: Vec<f64> = configs.iter()
    .map(|c| test_membrane_success(c))
    .collect();

// 3. Compute HL-normalized success
let hl_normalized: Vec<f64> = configs.iter().zip(&raw_success)
    .map(|(c, &raw)| {
        let S = singular_series_membrane_estimate(c);
        raw / S
    })
    .collect();

// 4. Test correlations
let corr_raw = correlation(&regularity_scores, &raw_success);
let corr_norm = correlation(&regularity_scores, &hl_normalized);

println!("Correlation (Regularity, Raw): {:.3}", corr_raw);
println!("Correlation (Regularity, Normalized): {:.3}", corr_norm);
println!("Orthogonal: {}", corr_norm.abs() < 0.10);
```

### Phase 3: Membrane Singular Series (Research)

**Approach 1: Empirical Calibration**
```rust
// Fit S(config) to make E[raw] = S(config) · baseline
// Use regression on known successful configurations
```

**Approach 2: Theoretical Derivation**
```agda
-- Formalize divisibility constraints for membranes
-- Derive multiplicative correction from residue theory
-- Prove correctness relative to empirical data
```

**Approach 3: Hybrid**
- Derive theoretical form from residue collapse structure
- Calibrate constants empirically
- Validate with cross-validation

---

## Connection to Existing Work

### Spectral Analysis

Our spectral regularity scores are structural heuristics analogous to Babylonian scores:

| Babylonian Scores | Our Regularity Scores |
|-------------------|----------------------|
| Based on gap factorization (2,3,5,60) | Based on residue frequency uniformity |
| Weight prime factors specially | Weight gcd(base, divisor) structure |
| Empirically correlate with pair counts | Empirically correlate with membrane success |
| Become orthogonal after HL normalization | Hypothesis: same orthogonality pattern |

**Test**: If our regularity scores also decorrelate after HL normalization, this validates that spectral analysis captures the same kind of structural bias that Babylonian scores capture.

### Hardy-Littlewood Framework

Existing HL implementation (src/hzlib/hardy_littlewood.rs):
- `singular_series_goldbach_multiplicative` - for gaps
- `hl_goldbach_lambda` - expected pair counts
- `goldbach_coverage_from_lambda` - probability estimation

**Extension needed**:
- `singular_series_membrane` - for configurations
- `hl_membrane_lambda` - expected membrane success
- Integration with existing framework

### Residue Theory

Our residue collapse formalization provides the theoretical foundation:
- Frequency distributions group by gcd structure
- Regularity quantifies this grouping
- HL normalization should remove the systematic component

**Prediction**:
```
Regularity captures systematic bias from residue structure
→ HL normalization removes systematic bias
→ Post-normalization, regularity should be uncorrelated
→ Orthogonality validates both regularity framework and HL application
```

---

## Expected Outcomes

### If Orthogonality Holds

**Interpretation**: Our regularity scores capture exactly the structural bias predicted by (extended) HL theory.

**Implications**:
1. Spectral analysis approach is theoretically grounded
2. Can use regularity for autonomous search (pre-screening)
3. HL framework extends naturally to membrane structures
4. Residue collapse theory connects to classical number theory

**Research contribution**: First connection of signal processing to constructive prime generation through HL framework.

### If Orthogonality Fails

**Interpretation A**: Regularity scores capture additional structure beyond HL predictions.

**Implications**:
- Regularity has predictive power even after HL normalization
- Suggests membrane structures have novel patterns
- Research direction: identify what regularity captures beyond HL

**Interpretation B**: Our HL normalization for membranes is incorrect.

**Implications**:
- Need better theoretical derivation of membrane singular series
- Current calibration approach insufficient
- More fundamental work needed on membrane HL theory

---

## Implementation Timeline

**Week 1 (Immediate)**:
1. Create `examples/membrane_orthogonality.rs` skeleton
2. Implement correlation statistics (reuse from spectral POC)
3. Test on prime pair data (validation against known results)

**Week 2-3**:
4. Develop membrane singular series estimation
5. Test orthogonality on small configuration set
6. Refine normalization based on results

**Month 2**:
7. Large-scale validation on all tested configurations
8. Theoretical derivation of membrane singular series
9. Formalize in Agda, validate computationally in Rust

**Month 3+**:
10. Publication: "Spectral Regularity and Hardy-Littlewood Orthogonality in Constructive Prime Generation"
11. Connect to broader literature on HL methods
12. Explore extensions (quadratic residues, higher-order structure)

---

## Technical Challenges

### Challenge 1: Membrane Singular Series

**Problem**: No established formula for membrane HL correction.

**Approaches**:
- Empirical: fit to make orthogonality hold
- Theoretical: derive from residue collapse structure
- Hybrid: theoretical form + empirical calibration

**Resolution**: Start empirical, work toward theoretical justification.

### Challenge 2: Sample Size

**Problem**: Limited tested configurations (10 bases, ~50 configs total).

**Approaches**:
- Bootstrap resampling for confidence intervals
- Focus on within-base correlations (more samples)
- Generate additional test data systematically

**Resolution**: Use all available data, note statistical limitations.

### Challenge 3: Multiple Divisors

**Problem**: Regularity score aggregates over divisors 2,3,5,7,...

**Approaches**:
- Test orthogonality separately for each divisor
- Use aggregate regularity score
- Weight by prime importance

**Resolution**: Test both aggregate and per-divisor; report all results.

---

## Research Questions

1. **Does membrane orthogonality hold?**
   - If yes → validates spectral framework
   - If no → identifies novel structure

2. **What is the correct membrane singular series?**
   - Can we derive it theoretically?
   - Does it match empirical calibration?

3. **Does orthogonality differ by base factorization?**
   - Prime bases vs composite bases
   - Squarefree vs non-squarefree

4. **Can we predict optimal configurations without testing?**
   - Regularity score → raw success (current)
   - Regularity + HL → normalized success (proposed)
   - Which is more reliable?

---

## Success Metrics

**Minimal success**:
- Implement correlation testing framework
- Validate on prime pair data
- Document methodology

**Moderate success**:
- Measure orthogonality for membrane configurations
- Identify whether pattern holds
- Propose membrane singular series form

**Strong success**:
- Derive membrane singular series theoretically
- Validate orthogonality empirically (|ρ| < 0.10)
- Publish novel connection between spectral/HL methods

**Outstanding success**:
- Complete Agda formalization with proofs
- Predictive model using HL-normalized scores
- Autonomous configuration search using orthogonality

---

## Summary

Orthogonality testing provides a rigorous framework for validating whether our structural scores (regularity from spectral analysis) capture the systematic bias that Hardy-Littlewood theory predicts.

**The test is simple**: After HL normalization, structural scores should be uncorrelated with success. If this holds, we've connected signal processing, residue theory, and classical number theory in a novel and meaningful way.

**The path forward**: Implement in Rust, test on available data, refine based on results, formalize theoretically, validate computationally.

This completes the integration of orthogonality testing concepts into our membrane prime generation framework.

---

**Next steps**: Create `examples/membrane_orthogonality.rs` implementing the testing framework described above.
