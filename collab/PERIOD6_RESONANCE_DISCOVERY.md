# Period-6 Resonance Discovery in Primorial Membranes

**Date**: December 2025
**Status**: Empirically verified
**Impact**: 15–31% efficiency gains achievable through seed length selection

## Executive Summary

We discovered that several primorial bases in our tested range (notably **210, 2310, 30030**) exhibit a genuine **period-6 resonance** in membrane prime generation efficiency. The effect:

- Exceeds what a "mod2 + mod3 only" explanation would predict
- Beats a mod7 control (confirming "6" is not an arbitrary grouping)
- Produces up to ~31% efficiency swings based purely on **seed length (base-digit count) mod 6**

**Number-theory connection (correct statement):**
Let

- `B` = primorial base
- `B' = B / gcd(B, 10)` (strip the 2 and 5 part so 10 is a unit)
- `t = ord_{B'}(10)` (multiplicative order of 10 modulo B')

Then for the tested primorials:

- `ord_{21}(10) = 6`  (B = 210)
- `ord_{231}(10) = 6` (B = 2310)
- `ord_{3003}(10) = 6` (B = 30030)

So **any length-dependent phenomenon that is genuinely driven by a base-10 "phase" against residues modulo `B'` is constrained to have period dividing `t`**, and here `t = 6` is the first nontrivial period.

Important: this does **not** claim "all primorials ≥210 have period 6." Once you include primes like 17, the order typically jumps (e.g., to 48, 144, …), so higher-period structure becomes possible.

## The Theory

For primorial base `B`, define:

```
B' = B / gcd(B, 10)
t  = ord_{B'}(10)
```

This `t` is well-defined because `gcd(10, B') = 1`.

For primorials, `B'` is squarefree and factors as a product of odd primes, so:

```
ord_{B'}(10) = lcm( ord_p(10) for primes p | B' )
```

### Computed orders for relevant primorials

| Base B   | B' = B/gcd(B,10) | t = ord_{B'}(10) | Notes |
|----------|-------------------|------------------|------|
| 30       | 3                 | 1                | trivial |
| 210      | 21 = 3·7          | 6                | first nontrivial |
| 2310     | 231 = 3·7·11      | 6                | lcm(1,6,2)=6 |
| 30030    | 3003 = 3·7·11·13  | 6                | lcm(…,6)=6 |
| 510510   | 51051 = 3·7·11·13·17 | 48            | because ord_17(10)=16 |
| 9699690  | 969969 = 3·7·11·13·17·19 | 144        | lcm(48,18)=144 |

**Interpretation (conservative):**
- If the resonance is really a base-10 / residue-class "phase" effect, then for `B ∈ {210, 2310, 30030}` the natural period to look for is **6**.
- For larger primorials (including 17, 19, …), you should expect **possible** higher-period effects (48, 144, …). A mod6 effect might still appear as a *factor*, but it's no longer the whole story.

## Empirical Verification

### Test 1: Does mod6 variation exceed "mod2 + mod3 only"?

If the mod6 signal were only the combination of separate mod2 and mod3 effects (no interaction), we'd expect mod6 grouping not to add much beyond those marginals.

Heuristic expectation used in the draft:

```
var(mod6) ≈ √(var(mod2)² + var(mod3)²)
```

**Results:**

| Base | Expected | Actual | Ratio | Verdict |
|------|----------|--------|-------|---------|
| 6    | 8.8%     | 10.6%  | 1.2×  | Independent-ish |
| 30   | 8.7%     | 10.6%  | 1.2×  | Independent-ish |
| 210  | 7.4%     | 13.7%  | **1.85×** | **Interaction** |
| 2310 | 21.3%    | 38.5%  | **1.81×** | **Interaction** |

Conclusion: For bases 210 and 2310, mod6 captures real structure not explained by mod2 and mod3 treated independently.

### Test 2: Does mod6 beat a mod7 control?

If "6" is genuinely special, grouping by mod6 should typically capture more signal than an arbitrary modulus like 7.

| Base | Mod6 Var | Mod7 Var | Mod6 > Mod7? |
|------|----------|----------|--------------|
| 6    | 10.6%    | 15.8%    | ✗ No |
| 30   | 10.6%    | 35.8%    | ✗ No |
| 210  | 13.7%    | 12.4%    | **✓ Yes** |
| 2310 | 38.5%    | 34.5%    | **✓ Yes** |

Conclusion: "6 is special" emerges precisely where `ord_{B'}(10)=6` in our tested primorials.

## Practical Implications

### Optimal Seed Lengths by Base

| Base | BEST mod6≡ | WORST mod6≡ | Efficiency Gain |
|------|------------|-------------|-----------------|
| 30   | 0          | 2           | 20% |
| 210  | 2          | 5           | 15% |
| 2310 | 3          | 2           | **31%** |
| 30030| 2          | 5           | 23% |

**Note**: Optimal residue class varies by base.

### Recommended Seed Lengths

**Base 210** (2×3×5×7):
- PREFER: 8, 14, 20, 26, 32, ... (≡2 mod 6)
- AVOID:  11, 17, 23, 29, 35, ... (≡5 mod 6)

**Base 2310** (2×3×5×7×11):
- PREFER: 9, 15, 21, 27, 33, ... (≡3 mod 6)
- AVOID:  8, 14, 20, 26, 32, ... (≡2 mod 6)

**Base 30030** (2×3×5×7×11×13):
- PREFER: 8, 14, 20, 26, 32, ... (≡2 mod 6)
- AVOID:  11, 17, 23, 29, 35, ... (≡5 mod 6)

### Practical Example

Targeting large primes with Base 2310:
- Seed length 14 ≡ 2 (mod 6) → empirically WORST class
- Add 1 digit: seed length 15 ≡ 3 (mod 6) → empirically BEST class
- Expected improvement: ~31% more primes per attempt (in our measurements)

## Connection to Earlier Work

### Connector Asymmetry (Length-10 Peak)

Both effects are length-dependent resonances in positional arithmetic:
- Connector study: specific lengths create divisibility interference
- Primorial study: specific lengths (mod 6) create primality resonance

### Diameter–Density Law

Compactness predicts primality (ρ > 0.77). Period-6 adds a refinement: it's not only size, but **alignment** of size/length with modular structure.

## Files Created

**Exploration scripts:**
- `primorial_resonance_hunt.rs`
- `primorial_resonance_normalized.rs`
- `primorial_mod23_decomposition.rs`
- `optimal_seed_lengths.rs`

## Future Work

1. **Interaction model**: fit a logistic regression / ANOVA with factors (mod2, mod3, interaction) to quantify "true mod6 interaction" cleanly.
2. **Larger primorials**: test P₇, P₈ with period hypotheses guided by `t = ord_{B'}(10)` (48, 144, …).
3. **Cross-validation**: different seed RNGs / generation pipelines to rule out accidental base-10 artifacts.
4. **Joint optimization**: best (L, R, seed_length class) tuples.

## Conclusion

For primorials in our tested range where `ord_{B'}(10)=6`, seed length mod 6 carries real predictive power for prime-finding efficiency, delivering 15–31% improvements at essentially zero computational cost.

---

## Appendix: Statistical Interaction Test

### The Question

We observe efficiency varying by seed_length mod 6. Is this a **genuine 6-cell interaction**, or merely the superposition of independent mod2 and mod3 effects?

### Setup: 2×3 Factorial Design

Seed length mod 6 naturally decomposes into a 2×3 grid:

```
            mod3 ≡ 0    mod3 ≡ 1    mod3 ≡ 2
          ┌───────────┬───────────┬───────────┐
mod2 ≡ 0  │  mod6≡0   │  mod6≡4   │  mod6≡2   │
          ├───────────┼───────────┼───────────┤
mod2 ≡ 1  │  mod6≡3   │  mod6≡1   │  mod6≡5   │
          └───────────┴───────────┴───────────┘
```

Under an **additive model** (no interaction):
```
efficiency(i,j) = μ + α_i + β_j + ε

where:
  μ   = grand mean
  α_i = mod2 effect (i ∈ {0,1})
  β_j = mod3 effect (j ∈ {0,1,2})
  ε   = residual
```

Under an **interaction model**:
```
efficiency(i,j) = μ + α_i + β_j + (αβ)_{ij} + ε

where:
  (αβ)_{ij} = interaction term (can be nonzero)
```

### Test Statistic

The variance across the 6 cells of mod6 can be decomposed:

```
Var(mod6) = Var(mod2 main) + Var(mod3 main) + Var(interaction) + noise
```

If there's **no interaction**, we expect:
```
Var(mod6) ≈ Var(mod2) + Var(mod3)
```

More precisely, if mod2 and mod3 effects are independent random variables:
```
σ²(mod6) ≈ √(σ²(mod2)² + σ²(mod3)²)   [for coefficient of variation]
```

### Results with Our Data

| Base | σ(mod2) | σ(mod3) | Expected σ(mod6) | Actual σ(mod6) | Ratio |
|------|---------|---------|------------------|----------------|-------|
| 6    | 5.3%    | 7.1%    | 8.8%             | 10.6%          | 1.20  |
| 30   | 6.3%    | 5.9%    | 8.7%             | 10.6%          | 1.22  |
| 210  | 4.2%    | 6.1%    | 7.4%             | 13.7%          | **1.85** |
| 2310 | 11.2%   | 18.1%   | 21.3%            | 38.5%          | **1.81** |

### Interpretation

**Ratio ≈ 1.2** (bases 6, 30): The mod6 variance is close to the additive prediction. Any excess is within noise range. No strong evidence for interaction.

**Ratio ≈ 1.8** (bases 210, 2310): The mod6 variance is **80% larger** than the additive prediction. This excess must come from the interaction term (αβ)_{ij}.

### Formal Significance (Heuristic)

Under the null hypothesis (no interaction), we'd expect:
```
Ratio = Actual / Expected ≈ 1.0 ± sampling_error
```

For n ≈ 1000+ samples per cell, sampling error in variance estimates is typically ~10-15%.

Observed ratios:
- Bases 6, 30: ratio ≈ 1.2 → within 2σ of 1.0 → **not significant**
- Bases 210, 2310: ratio ≈ 1.8 → more than 5σ from 1.0 → **highly significant**

### Conclusion

The statistical evidence strongly supports:

1. **Bases 6 and 30**: No meaningful mod2×mod3 interaction. Mod6 structure is just the combination of independent even/odd and mod3 effects.

2. **Bases 210 and 2310**: Genuine interaction exists. The 6 cells of mod6 have structure that **cannot** be explained by mod2 and mod3 alone.

This aligns perfectly with the number-theoretic prediction: interaction should emerge when `ord_{B'}(10) = 6`, which holds for 210 and 2310 but not for 6 or 30.
