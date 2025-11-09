# New Agda Verification Targets

**Strategic shortlist of brand new Agda files to formalize unverified claims**

Using the resources we discovered (UniMath, Primes blog, stdlib), here are the highest-value targets:

---

## 🎯 Priority 1: Core Mathematical Claims (Use UniMath)

### 1. `agda-proofs/Theorems/CoprimalityRequirement.agda`

**Claim to Verify**: "100% of top-performing membrane configurations use coprime boundary digits"

**What It Would Elucidate**:
- WHY coprimality is essential, not just empirically observed
- Mathematical proof that non-coprime configs have divisibility constraints
- Connection to radical theory and residue filtering

**Approach Using Resources**:
```agda
open import elementary-number-theory.divisibility-natural-numbers using (
  div-ℕ; is-coprime-ℕ
  )
open import elementary-number-theory.greatest-common-divisor-natural-numbers using (
  gcd-ℕ
  )

-- THEOREM: If boundary digit shares factor with base, membrane is composite
non-coprime-forces-composite : ∀ base outer inner seed →
  ¬ (is-coprime-ℕ outer base) →
  ∃ λ d → d > 1 × div-ℕ d (membrane base outer inner seed)

-- COROLLARY: Coprime configs have strictly better prime density
coprime-density-theorem : ∀ base →
  density (coprime-configs base) > density (non-coprime-configs base)
```

**Key Insight**: Use UniMath's divisibility theory to PROVE why coprimality works, not just observe it.

**Impact**: ⭐⭐⭐⭐⭐ (Elevates empirical finding to mathematical theorem)

---

### 2. `agda-proofs/Theorems/MinimalPaddingOptimality.agda`

**Claim to Verify**: "k=(0,0) consistently outperforms all padded variants"

**What It Would Elucidate**:
- Why tighter structure filters better
- Trade-off between width and constraint
- Optimal balance point

**Approach Using Resources**:
```agda
-- Use stdlib mod properties to analyze residue space
open import Data.Nat.DivMod.Properties using (
  m%n<n;           -- Bounded residues
  %-distribˡ-+     -- Mod distributivity
  )

-- THEOREM: Minimal padding maximizes residue coverage
minimal-padding-maximal-coverage : ∀ base outer inner →
  residue-coverage base outer inner 0 0 ≥
  residue-coverage base outer inner k₁ k₂

-- Key: More padding = more digits = more divisibility paths
padding-increases-composite-paths : ∀ base config k₁ k₂ →
  k₁ > 0 ∨ k₂ > 0 →
  composite-probability base config k₁ k₂ >
  composite-probability base config 0 0
```

**Key Insight**: Use mod arithmetic properties to show padding dilutes filtering power.

**Impact**: ⭐⭐⭐⭐⭐ (Proves another major empirical finding)

---

### 3. `agda-proofs/Theorems/RadicalDivisibilityFilter.agda`

**Claim to Verify**: "gcd(n, rad(b)) = 1 is required for primality in base b"

**What It Would Elucidate**:
- Exact relationship between radical and prime filtering
- Why rad(b) is the RIGHT measure (not φ(b) or b-1)
- Precise divisibility constraints

**Approach Using Resources**:
```agda
open import Core.Radical using (radical; radical-divides)
open import elementary-number-theory.prime-numbers using (is-prime-ℕ)

-- THEOREM: Prime in base b implies coprime to radical
prime-in-base-coprime-radical : ∀ n base →
  represents-in-base n base →
  is-prime-ℕ n →
  is-coprime-ℕ n (radical base)

-- THEOREM: Exactly φ(rad(b)) residues can be prime
prime-residue-count : ∀ base →
  count (λ r → potentially-prime r base) ≡ totient (radical base)
```

**Key Insight**: Radical.agda already scaffolded - now PROVE the filtering mechanism.

**Impact**: ⭐⭐⭐⭐⭐ (Fundamental to understanding why membranes work)

---

## 🎯 Priority 2: Computational Verification (Use Primes Blog)

### 4. `agda-proofs/Verification/ResonanceComputation.agda`

**Claim to Verify**: "Prime yield oscillates with space size between bodies 7 and 11"

**What It Would Elucidate**:
- Concrete verification of resonance peaks
- Pattern detection across multiple prime pairs
- Validates "gravitational membrane" metaphor

**Approach Using Resources**:
```agda
-- Use sieve from Primes in Agda blog for efficient computation
open import Sieve using (primes-up-to; is-prime-fast)

-- Generate all concatenations and test primality
test-space-size : PrimeBody → PrimeBody → ℕ → List Bool
test-space-size b1 b2 size =
  map is-prime-fast (all-concatenations b1 b2 size)

-- VERIFIED: Specific resonance pattern for (7, 11)
resonance-7-11-verified :
  yield 7 11 1 ≡ 2 ×
  yield 7 11 2 ≡ 3 ×
  yield 7 11 3 ≡ 8 ×  -- Peak!
  yield 7 11 4 ≡ 5 ×
  yield 7 11 11 ≡ 9   -- Another peak!
```

**Key Insight**: Use efficient sieve to COMPUTE all cases, not just sample.

**Impact**: ⭐⭐⭐⭐ (Validates empirical resonance findings computationally)

---

### 5. `agda-proofs/Verification/ExclusiveConfigurations.agda`

**Claim to Verify**: "Some configs work with only ONE specific seed (100% exclusive)"

**What It Would Elucidate**:
- Deterministic prime generation patterns
- Uniqueness of seed-config pairs
- Exhaustive verification of exclusivity

**Approach Using Resources**:
```agda
-- Test ALL seeds for a config using efficient primality testing
test-all-seeds : Base → Config → List ℕ → List (ℕ × Bool)
test-all-seeds base config seeds =
  map (λ seed → (seed, is-prime-fast (membrane base config seed))) seeds

-- THEOREM: Exclusive config has exactly one prime seed
exclusive-config : ∀ base config →
  IsExclusive base config →
  ∃! λ seed → is-prime-ℕ (membrane base config seed)

-- Verified examples from EVIDENCE.md
example-exclusive-1 :
  membrane 6 (1,5) (0,0) 4 ≡ prime ×
  ∀ s ≠ 4 → ¬ is-prime (membrane 6 (1,5) (0,0) s)
```

**Key Insight**: Exhaustive search with efficient primality testing.

**Impact**: ⭐⭐⭐⭐ (Proves deterministic generation claim)

---

## 🎯 Priority 3: Statistical Claims (Use Stdlib + Computation)

### 6. `agda-proofs/Verification/GCDParadoxComputation.agda`

**Claim to Verify**: "Higher GCD correlates with better prime success rate"

**What It Would Elucidate**:
- Computational verification of paradox
- Statistical significance of correlation
- Concrete examples across multiple bases

**Approach Using Resources**:
```agda
open import Data.Rational using (ℚ; _/_)
open import Data.List.Statistics using (mean; correlation)

-- Test multiple bases with different GCD values
test-gcd-correlation : List (ℕ × ℕ × ℚ)  -- (base, gcd, success-rate)
test-gcd-correlation =
  [ (6,  3, 0.33)   -- gcd(6,3) = 3, success 33%
  ; (10, 1, 0.185)  -- gcd(10,3) = 1, success 18.5%
  ; (12, 3, 0.28)   -- gcd(12,3) = 3, success 28%
  ; (14, 1, 0.21)   -- gcd(14,3) = 1, success 21%
  ]

-- THEOREM: Positive correlation verified
gcd-success-correlation-positive :
  correlation (map gcd-values test-data)
              (map success-rates test-data) > 0
```

**Key Insight**: Computational verification with statistical analysis.

**Impact**: ⭐⭐⭐⭐ (Confirms counterintuitive finding)

---

### 7. `agda-proofs/Verification/Base6Optimality.agda`

**Claim to Verify**: "No base ≤30 beats base 6's 33% success rate"

**What It Would Elucidate**:
- Exhaustive search across bases 2-30
- Why base 6 is special
- Connection to rad(6) = 6, gcd(6,3) = 3

**Approach Using Resources**:
```agda
-- Systematic test of all bases
test-all-bases : List (ℕ × Config × ℚ)
test-all-bases = map (λ b → (b, find-best-config b, success-rate b)) [2..30]

-- THEOREM: Base 6 is empirically optimal
base6-max-success : ∀ base config →
  base ≤ 30 →
  success-rate 6 (1,5) (0,0) ≥ success-rate base config

-- WHY base 6? Properties:
-- - rad(6) = 6 (simple radical)
-- - gcd(6,3) = 3 (high constraint)
-- - Small enough to test exhaustively
-- - Large enough to avoid trivial cases
```

**Key Insight**: Exhaustive verification + analysis of why 6 is special.

**Impact**: ⭐⭐⭐⭐ (Validates flagship claim)

---

## 🎯 Priority 4: Pattern Discovery (Advanced)

### 8. `agda-proofs/Theorems/UniversalPatternTheorem.agda`

**Claim to Verify**: "(1,5) k=(0,0) works in 5+ different bases"

**What It Would Elucidate**:
- Universal patterns that transcend specific bases
- Invariant properties of certain configs
- Predictive power for new bases

**Approach Using Resources**:
```agda
-- Test (1,5) k=(0,0) across multiple bases
universal-pattern-test : List (ℕ × ℚ)  -- (base, success-rate)
universal-pattern-test =
  map (λ b → (b, success-rate b (1,5) (0,0))) [6, 10, 14, 18, 22, 26]

-- THEOREM: (1,5) k=(0,0) is universally good
pattern-15-universal : ∀ base →
  base > 5 →
  base ≡ 0 (mod 2) ∨ base ≡ 0 (mod 3) →  -- Even or multiple of 3
  success-rate base (1,5) (0,0) > 0.15

-- WHY universal?
-- - 1 and 5 coprime to most bases
-- - Minimal padding k=(0,0) works everywhere
-- - Symmetric structure
```

**Key Insight**: Identify INVARIANT properties that predict success.

**Impact**: ⭐⭐⭐⭐⭐ (Enables prediction without testing)

---

### 9. `agda-proofs/Theorems/LagrangePointClustering.agda`

**Claim to Verify**: "Primes cluster at calculated Lagrange points between concatenated primes"

**What It Would Elucidate**:
- Gravitational membrane model validation
- Predictive power of Lagrange calculation
- Connection to equilibrium points

**Approach Using Resources**:
```agda
-- Use efficient primality testing for all positions
test-lagrange-positions : PrimeBody → PrimeBody → ℕ → List Bool
test-lagrange-positions b1 b2 space-size =
  let positions = calculate-lagrange-points b1 b2 space-size
  in map (λ pos → is-prime-fast (insert-at-position b1 b2 space-size pos))
         positions

-- THEOREM: Lagrange points have higher prime density
lagrange-clustering : ∀ b1 b2 size →
  density-at-lagrange-points b1 b2 size >
  density-at-random-positions b1 b2 size

-- Verified example from EVIDENCE.md
example-10301-3007003007003 :
  lagrange-point-1 ≡ 1 ×
  lagrange-point-2 ≡ 4 ×
  is-prime (insert-digit 10301 3007003007003 4 6) ≡ true
```

**Key Insight**: Validate "physical" metaphor with computational proof.

**Impact**: ⭐⭐⭐⭐ (Connects empirical finding to theoretical model)

---

## 🎯 Priority 5: Advanced Statistical Claims

### 10. `agda-proofs/Verification/PerturbationStability.agda`

**Claim to Verify**: ">90% of primes have stability score <0.1 (fragile)"

**What It Would Elucidate**:
- Fragility vs stability distribution
- Energy well interpretation
- Rare stable configurations

**Approach Using Resources**:
```agda
-- Test perturbations for verified primes
compute-stability : ConcatenatedConfig → ℚ
compute-stability config =
  let perturbations = all-single-digit-changes config
      survivors = filter is-prime-fast perturbations
  in (length survivors) / (length perturbations)

-- THEOREM: Most primes are fragile
fragility-theorem : ∀ sample-primes →
  (count (λ p → stability p < 0.1) sample-primes) /
  (length sample-primes) > 0.9

-- Verified example from EVIDENCE.md
example-fragile :
  stability config-7-100-5-5-11 ≡ 0.0000  -- 100% fragile
```

**Key Insight**: Large-scale computational verification.

**Impact**: ⭐⭐⭐ (Validates stability theory)

---

## 📊 Summary Table

| File | Claim | Method | Resources | Impact |
|------|-------|--------|-----------|--------|
| CoprimalityRequirement.agda | 100% coprime | Divisibility proof | UniMath | ⭐⭐⭐⭐⭐ |
| MinimalPaddingOptimality.agda | k=(0,0) best | Mod arithmetic | Stdlib | ⭐⭐⭐⭐⭐ |
| RadicalDivisibilityFilter.agda | rad(b) filter | Radical theory | UniMath + Core | ⭐⭐⭐⭐⭐ |
| ResonanceComputation.agda | Oscillation | Exhaustive test | Primes blog | ⭐⭐⭐⭐ |
| ExclusiveConfigurations.agda | Deterministic | Exhaustive test | Primes blog | ⭐⭐⭐⭐ |
| GCDParadoxComputation.agda | GCD correlation | Statistical | Stdlib | ⭐⭐⭐⭐ |
| Base6Optimality.agda | 33% is max | Exhaustive search | Primes blog | ⭐⭐⭐⭐ |
| UniversalPatternTheorem.agda | (1,5) universal | Pattern analysis | UniMath | ⭐⭐⭐⭐⭐ |
| LagrangePointClustering.agda | L-point density | Computational | Primes blog | ⭐⭐⭐⭐ |
| PerturbationStability.agda | 90% fragile | Statistical | Primes blog | ⭐⭐⭐ |

---

## 🎯 Recommended Priority Order

### Phase 1: Foundation (Weeks 1-2)
1. **CoprimalityRequirement.agda** - Proves WHY coprime works
2. **RadicalDivisibilityFilter.agda** - Fundamental filtering mechanism
3. **MinimalPaddingOptimality.agda** - Proves k=(0,0) optimality

**Why**: These are THEOREMS we can actually prove mathematically using UniMath.

### Phase 2: Computational Validation (Weeks 3-4)
4. **ResonanceComputation.agda** - Verify oscillation pattern
5. **ExclusiveConfigurations.agda** - Verify deterministic generation
6. **GCDParadoxComputation.agda** - Verify correlation

**Why**: Builds confidence through exhaustive computation.

### Phase 3: Empirical Claims (Weeks 5-6)
7. **Base6Optimality.agda** - Verify 33% is best
8. **UniversalPatternTheorem.agda** - Verify (1,5) k=(0,0) works everywhere

**Why**: Validates flagship empirical findings.

### Phase 4: Advanced Topics (Weeks 7-8)
9. **LagrangePointClustering.agda** - Verify gravitational model
10. **PerturbationStability.agda** - Verify fragility

**Why**: Deeper theoretical validation.

---

## 🚀 Quick Start: Template

```agda
{-# OPTIONS --safe --without-K #-}

module TheoremName where

-- 1. Import resources
open import elementary-number-theory.prime-numbers
open import Data.Nat.DivMod.Properties
open import Sieve  -- From primes blog

-- 2. State the claim
postulate
  main-claim : [precise mathematical statement]

-- 3. Break into lemmas
lemma-1 : [supporting fact]
lemma-1 = {! proof !}

lemma-2 : [supporting fact]
lemma-2 = {! proof !}

-- 4. Prove main theorem
main-theorem : main-claim
main-theorem = {! combine lemmas !}

-- 5. Computational verification
test-cases : List (Input × Expected)
test-cases = [concrete examples]

verified : all check test-cases ≡ true
verified = refl
```

---

## 💡 Key Insights

1. **UniMath enables PROOFS** - Use for coprimality, radical, padding
2. **Primes blog enables COMPUTATION** - Use for exhaustive verification
3. **Stdlib provides FOUNDATION** - Use for mod arithmetic properties
4. **Combination is POWERFUL** - Prove theorems AND verify computationally

---

**Next Action**: Pick 1-2 files from Phase 1 and start implementing!

**Estimated Time**:
- Phase 1: 2 weeks (foundational proofs)
- Phase 2: 2 weeks (computational verification)
- Phase 3: 2 weeks (empirical validation)
- Phase 4: 2 weeks (advanced topics)

**Total**: 2 months for all 10 files = ironclad verification of ALL major claims! 🎉
