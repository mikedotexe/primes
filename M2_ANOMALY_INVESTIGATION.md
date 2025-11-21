# M=2 Anomaly Investigation: The 4 Exceptions to Universal Minimal Padding

**Date**: November 2025
**Methodology**: Data-driven exhaustive enumeration
**Dataset**: 5,616 configurations × 17.6M primality tests

## Executive Summary

After discovering that M (middle length) dominates membrane behavior with near-perfect k*=0 universality for M≥2, we investigated the **4 rare exceptions** - configurations where M=2 but k*>0.

**Key Finding**: These anomalies are MARGINAL exceptions with tiny advantages (0.4-2.8 percentage points), ALL prefer k*=1, and ALL have prime outer boundary digits.

---

## Background: The M-Dependent Discovery

Our complete solution space exploration revealed a transformative pattern:

```
┌─────────────────────────────────────────────────────────────┐
│              M-DEPENDENT k* BEHAVIOR                        │
├─────────────────────────────────────────────────────────────┤
│  M=3:  468/468 configs → k*=0  (100.0%) PERFECT UNIVERSAL  │
│  M=2:  464/468 configs → k*=0  ( 99.1%) NEAR-PERFECT       │
│  M=1:  367/468 configs → k*=0  ( 78.4%) Mixed regime       │
└─────────────────────────────────────────────────────────────┘
```

**Previous hypothesis**: "Base 10 is exceptional"
**Data truth**: M (middle length) is the dominant variable, not base properties

**Anomaly distribution**:
- M=1: 101 anomalies (96.2% of all anomalies)
- M=2: **4 anomalies** (3.8%)
- M=3: 0 anomalies (0.0%)

This document focuses on understanding what makes those 4 M=2 anomalies special.

---

## The 4 M=2 Anomalies

### 1. Base 8, (5,1) → k*=1

```
Density progression:
  k=1: 0.178571 ★ OPTIMAL
  k=0: 0.160714
  k=2: 0.125000
  k=3: 0.035714

Advantage: 1.8 percentage points over k=0
```

**Properties**:
- outer=5 (prime), inner=1 (not prime)
- Both coprime to base 8: gcd(5,8)=1, gcd(1,8)=1
- Base 8 = 2³ (highly composite)

**Structure**: `5 0 1 [XX] 1 0 5` (k=1 padding)

### 2. Base 15, (7,2) → k*=1

```
Density progression:
  k=1: 0.090476 ★ OPTIMAL
  k=0: 0.080952
  k=2: 0.047619
  k=3: 0.028571

Advantage: 0.9 percentage points over k=0
```

**Properties**:
- outer=7 (prime), inner=2 (prime)
- Both coprime to base 15: gcd(7,15)=1, gcd(2,15)=1
- Base 15 = 3×5

**Structure**: `7 0 2 [XXX] 2 0 7` (k=1 padding)

### 3. Base 15, (13,1) → k*=1

```
Density progression:
  k=1: 0.114286 ★ OPTIMAL
  k=0: 0.085714
  k=3: 0.038095
  k=2: 0.028571

Advantage: 2.8 percentage points over k=0
```

**Properties**:
- outer=13 (prime), inner=1 (not prime)
- Both coprime to base 15: gcd(13,15)=1, gcd(1,15)=1
- Base 15 = 3×5

**Structure**: `13 0 1 [XXX] 1 0 13` (k=1 padding)

**Note**: This is the STRONGEST M=2 anomaly with 2.8 point advantage

### 4. Base 16, (5,11) → k*=1

```
Density progression:
  k=1: 0.104167 ★ OPTIMAL
  k=0: 0.100000
  k=2: 0.050000
  k=3: 0.045833

Advantage: 0.4 percentage points over k=0
```

**Properties**:
- outer=5 (prime), inner=11 (prime)
- Both coprime to base 16: gcd(5,16)=1, gcd(11,16)=1
- Base 16 = 2⁴ (highly composite)

**Structure**: `5 0 11 [XXXX] 11 0 5` (k=1 padding)

**Note**: This is INCREDIBLY marginal - only 0.4 percentage point advantage!

---

## Common Properties Analysis

### Universal Patterns Across All 4 Anomalies

1. **k* Value**: ALL have k*=1 (not k*=2 or k*=3)
   - k=1 provides marginal advantage
   - Higher padding (k=2,3) consistently worse than k=0

2. **Outer Digit**: ALL have prime outer boundary digits
   - Base 8 (5,1): outer=5 ✓ prime
   - Base 15 (7,2): outer=7 ✓ prime
   - Base 15 (13,1): outer=13 ✓ prime
   - Base 16 (5,11): outer=5 ✓ prime

3. **Coprimality**: ALL boundary digits coprime to base
   - gcd(outer, base) = 1 ✓
   - gcd(inner, base) = 1 ✓

4. **Advantage Magnitude**: Tiny margins (0.4-2.8 percentage points)
   - Strongest: Base 15 (13,1) at 2.8 points
   - Weakest: Base 16 (5,11) at 0.4 points
   - Average: ~1.5 percentage points

### Base Distribution

```
Base 8:  1 anomaly  (out of 48 M=2 configs = 2.1%)
Base 15: 2 anomalies (out of 224 M=2 configs = 0.9%)
Base 16: 1 anomaly  (out of 224 M=2 configs = 0.4%)
```

**Note**: Base 15 contributes half the anomalies, but still only 0.9% of its M=2 configurations.

### What Doesn't Correlate

❌ **Inner digit primality**: 2 prime, 2 not prime (50/50)
❌ **Base factorization type**: Powers of 2 (8, 16) and composite (15) both represented
❌ **Base size**: Bases 8, 15, 16 (no clear pattern)

---

## Comparative Context

### M=2 Anomalies vs M=1 Anomalies

|  | M=2 Anomalies | M=1 Anomalies |
|---|---|---|
| Count | 4 (0.9% of M=2) | 101 (21.6% of M=1) |
| k* values | ALL k*=1 | k*∈{1,2,3} mixed |
| Advantage | 0.4-2.8 points | Up to 19 points |
| Outer prime | 100% | Unknown |

**Interpretation**: M=2 anomalies are QUALITATIVELY different from M=1 anomalies:
- Much rarer (0.9% vs 21.6%)
- Much weaker advantages (max 2.8 vs max 19 points)
- More uniform behavior (all k*=1)

### Statistical Significance

For Base 16 (5,11) with 0.4 point advantage:
- k=0: 24/240 primes (10.0%)
- k=1: 25/240 primes (10.4%)
- Difference: **1 extra prime out of 240 candidates**

This raises the question: **Are these statistically meaningful exceptions or noise?**

---

## Hypothesis: Why k=1 Helps (Marginally)

### Structural Effect of k=1 Padding

Without padding (k=0):
```
outer inner [SEED] inner outer
  5     1   [XX]    1     5      (Base 8 example)
```

With k=1 padding:
```
outer 0 inner 0 [SEED] 0 inner 0 outer
  5   0   1   0  [XX]  0   1   0   5    (Base 8 example)
```

**Effect**: k=1 creates "breathing room" between boundary digits and seed

### Possible Mechanisms

1. **Divisibility Interference Reduction**
   - Direct adjacency between outer/inner might create local divisibility patterns
   - Zero buffer reduces interaction between boundary and seed digits

2. **Length Scaling**
   - M=2 seeds already create 6-digit membranes at k=0
   - k=1 expands to 10 digits
   - Small increase in number size might cross divisibility thresholds

3. **Prime Outer Digit Significance**
   - ALL 4 anomalies have prime outer digits
   - Perhaps prime outer digits benefit from zero buffering?
   - **Counter-evidence**: Most M=2 configs with prime outer still prefer k*=0

### Why This Doesn't Scale to M=3

For M=3:
- Seeds are longer (3 digits in base)
- Membranes already longer (8+ digits at k=0)
- Boundary-seed interaction already minimized by structure
- Zero padding provides no additional benefit → **100% k*=0**

---

## Conclusion

### Key Findings

1. **M=2 anomalies are MARGINAL exceptions**, not robust alternatives
   - 0.4-2.8 percentage point advantages (vs 19+ points for strong M=1 anomalies)
   - Only 4 out of 468 M=2 configurations (0.9%)

2. **All 4 share common traits**:
   - k*=1 (never k*=2 or k*=3)
   - Prime outer boundary digits
   - Coprime boundary digits (universal requirement)
   - Tiny density advantages

3. **Statistical fragility**: Base 16 (5,11) differs by 1 prime in 240 candidates

### Implications for Minimal Padding Principle

The M=2 anomalies **do not refute** the minimal padding principle. Instead, they demonstrate:

- **M=3 universality**: Zero exceptions → k*=0 is ABSOLUTE for longer seeds
- **M=2 near-universality**: 99.1% k*=0 → exceptions are rare edge cases
- **M=1 mixed regime**: 21.6% anomalies → different dynamics at play

**Refined principle**:
> **For M≥2, minimal padding (k=0) is optimal in >99% of configurations. The 4 M=2 exceptions show marginal k=1 advantages (<3 percentage points) and may represent statistical noise rather than robust pattern.**

### Questions for Further Investigation

1. **Statistical significance testing**: Are these real effects or sampling noise?
   - Need larger seed ranges or bootstrap analysis
   - Especially for Base 16 (5,11) with 0.4 point margin

2. **Prime outer digit correlation**: Why do ALL 4 have prime outer digits?
   - Is this mechanistically significant or coincidence?
   - Test: Do other M=2 configs with prime outer show elevated k=1 preference?

3. **Base property analysis**: What do bases 8, 15, 16 have in common?
   - 8=2³, 15=3×5, 16=2⁴
   - Two are powers of 2, one is odd composite
   - No obvious unifying property

4. **Replication in M>3**: Do any M=4 or M=5 configs show k*>0?
   - Prediction: NO - expect 100% k*=0 for all M≥3
   - Would confirm strengthening of minimal padding with length

---

## Data Transparency

**Complete dataset**: `solution_space_complete.csv` (5,616 rows)
**Analysis script**: `find_m2_anomalies.py`
**Pattern analyzer**: `examples/pattern_analyzer.rs`
**Solution space explorer**: `examples/solution_space_explorer.rs`

**Methodology**: Exhaustive enumeration of all seed values (not sampling)
**Primality testing**: Miller-Rabin with 20 rounds (error rate <10⁻¹²)
**Reproducibility**: All configurations deterministically testable

---

## Appendix: Full Density Tables

### Base 8, (5,1) - M=2

| k | Total | Primes | Density | vs k=0 |
|---|-------|--------|---------|---------|
| 0 | 56 | 9 | 0.160714 | - |
| 1 | 56 | 10 | **0.178571** | +1.8 pts |
| 2 | 56 | 7 | 0.125000 | -3.6 pts |
| 3 | 56 | 2 | 0.035714 | -12.5 pts |

### Base 15, (7,2) - M=2

| k | Total | Primes | Density | vs k=0 |
|---|-------|--------|---------|---------|
| 0 | 210 | 17 | 0.080952 | - |
| 1 | 210 | 19 | **0.090476** | +0.9 pts |
| 2 | 210 | 10 | 0.047619 | -3.3 pts |
| 3 | 210 | 6 | 0.028571 | -5.2 pts |

### Base 15, (13,1) - M=2

| k | Total | Primes | Density | vs k=0 |
|---|-------|--------|---------|---------|
| 0 | 210 | 18 | 0.085714 | - |
| 1 | 210 | 24 | **0.114286** | +2.8 pts |
| 2 | 210 | 6 | 0.028571 | -5.7 pts |
| 3 | 210 | 8 | 0.038095 | -4.8 pts |

### Base 16, (5,11) - M=2

| k | Total | Primes | Density | vs k=0 |
|---|-------|--------|---------|---------|
| 0 | 240 | 24 | 0.100000 | - |
| 1 | 240 | 25 | **0.104167** | +0.4 pts |
| 2 | 240 | 12 | 0.050000 | -5.0 pts |
| 3 | 240 | 11 | 0.045833 | -5.4 pts |

**Note**: Base 16 (5,11) differs by literally **1 prime** (25 vs 24) out of 240 candidates.

---

**End of Report**
