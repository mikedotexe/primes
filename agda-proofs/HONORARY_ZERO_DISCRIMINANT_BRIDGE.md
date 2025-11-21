# Honorary Zero ↔ Discriminant Integration

**Date**: November 19, 2025
**Status**: Formal frameworks created, integration bridge documented
**Agda Modules**: `Core/HonoraryZero.agda`, `Core/Discriminant.agda`

---

## Overview

This document formalizes the integration between two complementary perspectives on membrane prime generation:

1. **Honorary Zero (HZ)**: Geometric reference frame for symmetry analysis
2. **Discriminant (Δ)**: Algebraic skeleton governing polynomial factorization

**Key insight**: HZ defines WHAT symmetry means; Δ determines WHETHER symmetry helps or hurts.

---

## The Three Centers

### 1. Base Midpoint (HZ in ℤ/2pℤ)

For even base b = 2p, the midpoint m = p = b/2 is the Honorary Zero:

```agda
record HZBase : Set where
  field
    b : ℕ          -- even base
    mid : ℕ        -- midpoint p
    mid-is-half : 2 * mid ≡ b
```

**Properties**:
- In ℤ/2pℤ, p has order 2: p ≡ -p (mod 2p)
- Central involution: digits d and (2p - d) are symmetric around HZ
- Phase-locked pairs: primes summing to 2p are HZ-symmetric

**Examples**:
- Base 10: HZ = 5, pairs like (3,7)
- Base 14: HZ = 7, pairs like (3,11), (1,13)
- Base 6: HZ = 3, pairs like (1,5)

### 2. Pattern Midpoint (Template Axis)

The membrane template has a central axis through the seed or zero-buffer:

```
3 ◯◯ 7 ◯ SEED ◯ 7 ◯◯ 3
         ↑
    Pattern HZ
```

**Formalized** in `SymmetricTemplate` (existing Agda work)

### 3. Polynomial Center (Discriminant Axis)

For quadratic N(X) = A·X² + S·X + A, the discriminant Δ = S² - 4A²:

```agda
discriminant : ℕ → ℕ → ℤ
discriminant A S = (+ (S * S)) - (+ 4) * (+ (A * A))
```

**Properties**:
- Invariant under X ↔ -X reflection
- Controls polynomial factorization
- Determines quadratic residue behavior via Legendre symbols

---

## HZ's Role: Reference Frame, Not Mechanism

### What HZ DOES Provide

#### (a) Geometric Definition of Symmetry

The δ transformation maps digits to signed distances from HZ:

```agda
δ : HZBase → ℕ → ℤ
δ hz d = (+ d) - (+ (mid hz))
```

**Examples** (Base 10, HZ = 5):
- δ(0) = -5
- δ(5) = 0  (the HZ itself)
- δ(9) = +4

**Symmetric digits**:

```agda
symmetricDigits : HZBase → ℕ → ℕ → Set
symmetricDigits hz x y = δ hz x ≡ - δ hz y
```

Equivalently: x + y = 2·mid = b (sum to base, Goldbach reflection)

#### (b) Mirror Obstruction Framework

In `Theorems/MirrorObstruction.agda`:
- Perfect HZ-symmetry + all-zero bridge → composite
- HZ acts as "factor-attracting symmetry axis"
- Lagrange points break HZ-symmetry to restore primality

**Interpretation**: Too much HZ-symmetry = discriminant degeneracy

### What HZ Does NOT Explain

**Critical observation from empirical work**:

| Base | HZ | Phase-Locked Pairs | k*=1 Advantage? |
|------|----|--------------------|-----------------|
| 10   | 5  | None (9 not prime) | ✓ YES (M=2 only)|
| 14   | 7  | (3,11), (1,13)     | ✗ NO (k*=0)     |
| 12   | 6  | (5,7)              | ✗ NO (k*=0)     |
| 22   | 11 | (3,19), (5,17)     | ✗ NO (k*=0)     |

**Conclusion**: HZ symmetry exists in all these bases, but only Base 10 shows k=1 bump. Therefore:

**HZ is the reference frame. The engine is discriminant + residue constraints.**

---

## Discriminant: The Algebraic Engine

### Perfect Square Lock (Algebraic Constraint)

**Theorem** (to be proven in Agda):

```agda
algebraicLockTheorem : ∀ (A S : ℕ)
                     → IsPerfectSquare (Δ A S)
                     → composite (N A S X) for large X
```

**Proof sketch**:
- If Δ = r², polynomial factors: N(X) = A(X - α)(X - β)
- For sufficiently large X, factors are > 1 → composite

**Empirical validation**:
- Base 6 (1,5): 0/30 perfect squares at M=2, 0/180 at M=3
- Base 12 (1,5): 0/132 at M=2, 0/1584 at M=3
- Lock active at M≥2

### Quality Score via Legendre Symbols

For each prime q, Legendre symbol (Δ/q) determines sieve behavior:

```agda
data LegendreSymbol : Set where
  positive : LegendreSymbol  -- +1: Δ is QR mod q (obstructed)
  negative : LegendreSymbol  -- -1: Δ is NR mod q (admissible)
  zero     : LegendreSymbol  --  0: q | Δ (worst case)
```

**Quality score**:

```agda
score = admissible_count - obstructed_count - 5·divisible_count
```

**Empirical correlation** (discriminant quality ↔ primality):
- Base 6 (1,5) M=2: ρ = +0.39 (strong)
- Base 6 (5,1) M=2: ρ = -0.23 (fails!)
- Base 12 (1,5) M=2: ρ = +0.10 (weak)

---

## Integration: HZ × Δ Interaction

### Configuration-Specific Behavior

**Base 6 (1,5)**: NOT HZ-symmetric (1+5 ≠ 6), small A → success
- Δ(A=1, S) = S² - 4 varies freely
- Small outer shell → discriminant dominated by seed
- ρ = +0.39 at M=2

**Base 6 (5,1)**: NOT HZ-symmetric, large A → failure
- Δ(A=5, S) = S² - 100 has large negative offset
- Large outer shell → discriminant constrained
- ρ = -0.23 at M=2 (wrong sign!)

**Conclusion**: HZ-symmetry vs asymmetry doesn't determine success. Outer shell size A matters algebraically.

### Base-Specific Manifestations

**Collaborator's insight**:
> "The polynomial representation N(X) = A·X² + S·X + A provides the algebraic skeleton regardless of base. But primality depends on how the polynomial evaluates when X = b^k in a given base."

**Three-layer interaction**:

1. **Universal layer** (discriminant):
   - Δ = S² - 4A² computed independent of base
   - Perfect square lock applies universally

2. **Base-specific layer** (evaluation):
   - X = b^k depends on base b and padding k
   - N(A, S, b^k) encounters residue constraints from rad(b)

3. **HZ layer** (geometry):
   - Defines symmetric pairs, midpoint structure
   - Provides analysis framework but not causal mechanism

**Example**: Base 10 k=1 anomaly
- HZ = 5 (nice midpoint)
- Discriminant evaluated at X = 10¹ = 10
- Residue filtering: k=1 constraint enriches {1,3,7,9} endings
- These endings correlate with better discriminant properties
- **Coupling**: k=1 SELECTS seeds with favorable Δ values

**Test** (collaborator's suggestion):
> "For every seed S tested in Base 10, compute Δ = S² - 4 and factor it. Compare discriminant distributions between k=0 and k=1. The hypothesis is that k=1 seeds will show statistically fewer prime factors in their discriminants."

This is **Phase 1** of the integration plan.

---

## Formal Connection (Agda Roadmap)

### Existing Modules

**`Core/HonoraryZero.agda`**:
- `HZBase` record
- `δ` transformation
- `symmetricDigits` predicate
- `PhaseLockedPair` definition

**`Core/Discriminant.agda`**:
- `discriminant` computation
- `IsPerfectSquare` record
- `DiscriminantQuality` with Legendre symbols
- `evaluatePolynomial` for N(X)

### To Be Created

**`Theorems/AlgebraicModularBridge.agda`**:

```agda
-- Connect discriminant properties to residue class membership
bridgeTheorem : ∀ (hz : HZBase) (A S : ℕ)
              → (symmetricDigits hz A inner)  -- HZ-symmetric boundaries
              → implies? (discriminantDegeneracy A S)  -- Constrained Δ?
```

**Questions to formalize**:
1. Do HZ-symmetric boundaries create discriminant degeneracy?
2. Does perfect HZ-symmetry in pattern imply perfect square Δ?
3. How does base factorization (rad(b)) interact with Legendre symbols (Δ/q)?

**`Specs/DiscriminantResidueCoupling.agda`**:

```agda
-- Executable spec: for given base, config, and k value,
-- compute discriminant distribution and residue class distribution
-- Test if they're correlated
```

---

## Empirical Validation Strategy

**Phase 1**: Discriminant-Residue Coupling (from integration plan)

For Base 10 M=2 (where k=1 shows bump):
1. Generate CSV with columns: `[seed, k, discriminant, residue_mod_2, residue_mod_5, is_prime]`
2. Stratify by k value
3. Test hypothesis: k=1 seeds have better discriminants than k=0

**Prediction**:
- k=0 seeds: discriminants with many small prime factors
- k=1 seeds: discriminants with fewer factors, more non-residues

**Phase 2**: Perfect Square Lock Universal Test

Scan all bases, all configs:
- Count perfect square discriminants
- Check primality rate for Δ=□ cases
- Expected: 0% primality at M≥2 (lock holds)

**Phase 3**: A=1 Universality Test

Test whether minimal outer shell (A=1) enables discriminant correlation:
- Run discriminant scanner on ALL (1,x) configs across bases
- Compare ρ(quality, primality) for A=1 vs A≥2
- Expected: A=1 shows positive ρ; A≥2 shows weak/negative ρ

---

## Synthesis: Multi-Layer Architecture

```
┌─────────────────────────────────────────────────────┐
│ Level 0: HONORARY ZERO (Geometric Reference)        │
│ - Defines symmetry axis in base                     │
│ - Phase-locked pairs, Goldbach reflection           │
│ - Mirror obstruction framework                      │
│ → Provides LANGUAGE, not CAUSE                      │
└─────────────────────────────────────────────────────┘
                    ↓ Coordinates
┌─────────────────────────────────────────────────────┐
│ Level 1: ALGEBRAIC CONSTRAINTS (Discriminants)      │
│ - Perfect square lock (hard barrier)                │
│ - Legendre symbols (local sieve pressure)           │
│ - Quality score (~15% variance for A=1 at M=2)      │
└─────────────────────────────────────────────────────┘
                    ↓ Filters seed space
┌─────────────────────────────────────────────────────┐
│ Level 2: MODULAR STRUCTURE (Residues, Coprimality)  │
│ - gcd(boundaries, base) = 1 requirement             │
│ - Residue class availability mod rad(b)             │
│ - CRT-based coordinate transformations              │
└─────────────────────────────────────────────────────┘
                    ↓ Determines viable configs
┌─────────────────────────────────────────────────────┐
│ Level 3: GEOMETRIC STRUCTURE (Padding, Symmetry)    │
│ - k*=0 minimal complexity principle                 │
│ - Lagrange point perturbations                      │
│ - Pattern length effects                            │
└─────────────────────────────────────────────────────┘
                    ↓ Optimizes within constraints
┌─────────────────────────────────────────────────────┐
│ Level 4: ANALYTIC DENSITY (PNT, Hardy-Littlewood)   │
│ - Length penalty ~1/(M·ln b)                        │
│ - Local density fluctuations                        │
│ - Singular series corrections                       │
└─────────────────────────────────────────────────────┘
```

**HZ sits at Level 0**, providing the coordinate system for all other layers. Discriminants operate at **Level 1**, filtering the algebraic possibility space. Together, they form the foundation for the multi-layer model.

---

## Next Steps (Integration Plan)

1. ✅ **Agda formalization**: `HonoraryZero.agda` and `Discriminant.agda` created
2. ⏳ **Phase 1**: Test discriminant-residue coupling in Base 10 M=2
3. ⏳ **Phase 2**: Perfect square lock comprehensive validation
4. ⏳ **Phase 3**: Build multi-layer composite predictor
5. ⏳ **Phase 4**: Test A=1 universality across all bases
6. ⏳ **Phase 5b**: Prove `algebraicLockTheorem` and bridge theorems
7. ⏳ **Phase 6**: Unified membrane theory synthesis

---

## Conclusion

The Honorary Zero and Discriminant frameworks are **complementary, not competing**:

- **HZ**: Geometric reference frame, defines symmetry, explains mirror obstruction
- **Δ**: Algebraic constraints, determines factorization, predicts local sieve pressure

**Collaborator's key insight validated**:
> "HZ gives you the geometry and the language to speak about symmetry, but the actual bump is driven by base factorization, residue filtering, and discriminants evaluated along X_k = b^(k+1). HZ is the reference frame. The engine is modular arithmetic + polynomial/discriminant."

This integration bridges:
- Empirical findings (coprimality, k*=0, phase lock tests)
- Formal proofs (mirror obstruction, residue fold, CRT)
- Polynomial theory (discriminants, Legendre symbols)
- Hardy-Littlewood density framework

The path to a **unified membrane theory** requires all perspectives working together.
