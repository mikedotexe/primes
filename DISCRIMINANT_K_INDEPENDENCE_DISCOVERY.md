# Critical Discovery: Discriminants are K-Independent

**Date**: November 19, 2025
**Status**: ❌ Coupling hypothesis REFUTED → ✅ K-independence discovered
**Significance**: Major theoretical clarification

---

## Executive Summary

Phase 1 coupling analysis **refuted** the hypothesis that k=1 selects seeds with better discriminants. Instead, it revealed a fundamental property:

**Discriminant Δ = S² - 4A² depends ONLY on (seed, outer shell), NOT on padding k.**

This means:
- k=0 and k=1 sample **identical** discriminant distributions
- The k-dependent density difference must arise from a **different mechanism**
- Discriminant and residue constraints are **orthogonal** to k-optimization

---

## Experimental Results

### Configuration: Base 10 (3,7) M=2

**Test**: Compare discriminant quality between k=0 and k=1

**Results**:

| Metric | k=0 | k=1 | Difference |
|--------|-----|-----|------------|
| Mean quality score | -8.011 | -8.011 | **0.000** |
| Perfect squares | 1/90 | 1/90 | **0** |
| Favorable residues | 100% | 100% | **0%** |
| Mod 5 pattern | All ≡3 | All ≡3 | **Identical** |
| **Prime density** | **21.1%** | **10.0%** | **-11.1pp** |

### Critical Observation

**Discriminants are identical, but densities differ by 2×!**

This conclusively proves that the k-dependent density effect operates through a mechanism **independent of discriminant properties**.

---

## Why Discriminants Are K-Independent

### Mathematical Proof

The discriminant formula is:

```
Δ = S² - 4A²
```

where:
- S = seed (middle value)
- A = outer shell (boundary digit)

**Padding k appears nowhere in this formula.**

Therefore:
- For seed S=23, outer A=3: Δ = 23² - 4(3²) = 529 - 36 = 493
- This is **the same** whether k=0 or k=1 or k=100

### Membrane Structure Reality Check

```
k=0: 3 7 23 7 3         → Δ(3, 23) = 493
k=1: 3 0 7 0 23 0 7 0 3 → Δ(3, 23) = 493  (same!)
```

The zeros change the **number's value** (decimal representation), but not the **discriminant** of the underlying polynomial N(X) = A·X² + S·X + A.

---

## Why Residues Are Also K-Independent (Almost)

### The (3,7) Universal Pattern

With boundaries (3,7), the membrane structure creates:

```
Membrane = 3 + ... + 7 (in specific positions)
```

For Base 10, this **always** produces:
- **Odd** numbers (last digit determined by seed ending + contributions)
- **≡3 (mod 5)** for all seeds in M=2 range

This is true for **both** k=0 and k=1!

The mod 5 pattern is locked in by the (3,7) boundaries and base 10 structure, independent of padding.

---

## Implications

### 1. The Collaborator's Hypothesis Was Wrong (But Instructive)

**Original hypothesis**:
> "Seeds that would produce discriminants with many small prime factors or that are perfect squares get filtered out by the residue requirements"

**Reality**:
- Residue requirements (odd, not div by 5) are **universal** for (3,7) in base 10
- They don't "filter" different discriminants for different k values
- Both k=0 and k=1 see the same discriminant pool

**Value**: Testing this hypothesis revealed k-independence, which is more important!

### 2. The K-Dependent Mechanism Must Be Elsewhere

Since discriminants and basic residues don't differ between k=0 and k=1, the 21.1% vs 10.0% density gap must arise from:

**Candidate mechanisms**:

1. **Length penalty** (PNT):
   - k=0: 6-digit numbers
   - k=1: 10-digit numbers
   - Expected density ratio: ~10/6 = 1.67× (close to observed 2.1×!)

2. **Higher-order modular structure**:
   - Not just mod 2, mod 5, but mod 3, mod 7, mod 11, etc.
   - Perhaps k=1 creates systematic obstructions mod small primes

3. **Mirror symmetry effects**:
   - k=1 adds zeros, increasing palindromic regularity
   - May trigger mirror obstruction factors

4. **Goldbach reflection**:
   - The k=1 structure might interact differently with Goldbach pair constraints

### 3. Discriminant Framework Still Valid (But Scope-Limited)

The discriminant analysis is **not worthless**, it just operates at a different level:

**What discriminants DO explain**:
- Why Base 6 (1,5) shows ρ=0.39 correlation (A=1 minimal shell)
- Why Base 6 (5,1) shows ρ=-0.23 failure (A=5 large shell)
- Perfect square lock (algebraic constraint)

**What discriminants DON'T explain**:
- k=0 vs k=1 density differences (k-independent!)
- Base 10 M=2 "anomaly" (both k values have same Δ distribution)
- Coprimality requirement (not a discriminant property)

---

## Revised Multi-Layer Model

### Original Model (from bridge document)

```
HZ (L0) → Discriminant (L1) → Modular (L2) → Geometric (L3) → Analytic (L4)
```

### Revised Understanding

**Discriminant (L1) is ORTHOGONAL to Geometric (L3)**:

```
         ┌─────────────┐
    ┌────┤ Seed + Shell├────┐
    │    └─────────────┘    │
    ↓                       ↓
Discriminant Δ         Padding k
(k-independent)      (Δ-independent)
    ↓                       ↓
Quality Score          Length, Symmetry
Legendre Symbols       Modular Obstructions
    ↓                       ↓
    └───────→ Combine ←─────┘
              ↓
        Prime Density
```

**Key insight**: You can't understand k-dependent density from discriminants alone. You need BOTH the algebraic layer (Δ) AND the geometric layer (k).

---

## Next Steps

### 1. Length Penalty Analysis

Test if the k=0 vs k=1 density ratio matches PNT predictions:

```
Expected ratio = (# digits at k=0) / (# digits at k=1)
                = 6 / 10 = 1.6×

Observed ratio = 21.1% / 10.0% = 2.1×
```

Close but not exact → suggests length penalty PLUS another factor.

### 2. Higher-Order Modular Tests

Compute membrane values mod 3, mod 7, mod 11, mod 13:
- Do k=0 and k=1 have different distributions?
- Could systematic mod obstructions explain the gap?

### 3. Mirror Symmetry Index

Quantify how "mirror-symmetric" k=0 vs k=1 membranes are:
- k=0: less symmetric (6 digits, seed dominates)
- k=1: more symmetric (10 digits, zeros create regularity)
- Does higher symmetry → more mirror obstruction factors?

### 4. Revised Discriminant Scope

**Use discriminants for**:
- Cross-config comparison (A=1 vs A=5)
- Perfect square lock validation
- Algebraic constraint proofs in Agda

**Don't use discriminants for**:
- Explaining k-dependent effects (orthogonal!)
- Base-specific anomalies (need full residue analysis)
- Padding optimization (geometric layer)

---

## Theoretical Significance

This discovery **clarifies the scope** of each mathematical framework:

**Honorary Zero**: Geometric reference, defines symmetry language
**Discriminant**: Algebraic constraints, k-independent, seed-and-shell only
**Residue Classes**: Modular structure, partially k-independent (depends on specifics)
**Padding Geometry**: Length, symmetry, obstruction patterns (discriminant-independent!)

**The lesson**: Multi-perspective analysis isn't just complementary - it's **necessary** because different phenomena live in orthogonal mathematical spaces!

---

## Conclusion

Phase 1 **refuted** the coupling hypothesis but discovered something more fundamental:

**Discriminants are k-independent.**

This means:
- ✅ Discriminant framework is valid for seed/shell analysis
- ❌ Discriminants cannot explain k-dependent density variations
- ✅ Need geometric layer (length, symmetry) to understand padding effects
- ✅ Multi-layer model confirmed: each layer contributes orthogonally

**The search continues**: What geometric or higher-order modular property makes k=0 consistently outperform k=1 by 2×?

**Artifacts**:
- `discriminant_residue_coupling.rs` - Coupling analyzer (180 tests)
- `discriminant_residue_base10_m2.csv` - Full per-seed data
- `coupling_results.txt` - Test output
- `DISCRIMINANT_K_INDEPENDENCE_DISCOVERY.md` - This report

**Next phase**: Length penalty vs higher-order modular obstruction analysis.
