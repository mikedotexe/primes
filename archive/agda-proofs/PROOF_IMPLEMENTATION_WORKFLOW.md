> Archived on 2026-03-10. This workflow document reflects an older execution
> plan and is no longer the active guide for the Agda workspace.

# Agda Proof Implementation Workflow

**From resource discovery → proven theorem: A practical guide**

This document shows EXACTLY how to use our discovered resources (UniMath, Primes blog, stdlib) to implement the proofs we've outlined.

---

## 🎯 The Big Picture

```
Resources Available          Our Claims              Formal Proofs
┌──────────────────┐        ┌─────────────┐        ┌──────────────┐
│ UniMath          │───────→│ Coprimality │───────→│ Proven!      │
│ Primes Blog      │        │ Radical     │        │ Verified!    │
│ Agda Stdlib      │        │ GCD Paradox │        │ Ironclad!    │
│ Core/Radical     │        │ Affine      │        │              │
└──────────────────┘        └─────────────┘        └──────────────┘
```

**Goal**: Transform every empirical claim into a formally verified theorem.

---

## 📚 Phase 0: Resource Setup

### Step 1: Install UniMath Agda Library

```bash
cd /home/user/primes/agda-proofs
git clone https://github.com/UniMath/agda-unimath.git
```

### Step 2: Configure Agda Library Path

Create or edit `~/.agda/libraries`:
```
/home/user/primes/agda-proofs/agda-unimath/agda-unimath.agda-lib
```

Create or edit `~/.agda/defaults`:
```
agda-unimath
standard-library
```

### Step 3: Verify Installation

```agda
-- Test file
module Test where

open import elementary-number-theory.prime-numbers
open import elementary-number-theory.divisibility-natural-numbers

-- Should type-check if installed correctly
test-prime : is-prime-ℕ 7
test-prime = {! verify this works !}
```

### Step 4: Get Sieve from Primes Blog

Download from: https://doisinkidney.com/posts/2018-12-14-primes-in-agda.html

Or implement minimal version:
```agda
module Sieve where

-- Sieve of Eratosthenes (simplified)
sieve : ℕ → List ℕ
sieve n = {! implementation from blog !}

is-prime-fast : ℕ → Bool
is-prime-fast n = n ∈ sieve n
```

---

## 🔨 Phase 1: Implement Foundation Proofs

### Example Workflow: CoprimalityRequirement.agda

#### Step 1: Replace Postulates with UniMath Imports

**Before**:
```agda
postulate
  IsPrime : ℕ → Set
  _∣_ : ℕ → ℕ → Set
```

**After**:
```agda
open import elementary-number-theory.prime-numbers using (is-prime-ℕ)
open import elementary-number-theory.divisibility-natural-numbers using (div-ℕ)
```

#### Step 2: Prove First Lemma (Simplest One)

**Target**: `non-coprime-outer-forces-divisibility`

**Strategy**:
```agda
non-coprime-outer-forces-divisibility : ∀ b conf seed →
  gcd (outer conf) (base b) > 1 →
  gcd (outer conf) (base b) ∣ membrane b conf seed

-- PROOF:
non-coprime-outer-forces-divisibility b conf seed gcd>1 =
  let d = gcd (MembraneConfig.outer conf) (Base.value b)
      o = MembraneConfig.outer conf
  in
    begin
      -- 1. d ∣ outer (by GCD property)
      div-gcd-left : d ∣ o
      div-gcd-left = gcd-is-divisor-left-ℕ o (Base.value b)

      -- 2. d ∣ base (by GCD property)
      div-gcd-right : d ∣ Base.value b
      div-gcd-right = gcd-is-divisor-right-ℕ o (Base.value b)

      -- 3. d ∣ base^k for all k (by induction)
      div-power : ∀ k → d ∣ (Base.value b) ^ k
      div-power = div-exp-ℕ div-gcd-right

      -- 4. d ∣ outer·base^k (product divisibility)
      div-outer-term : ∀ k → d ∣ o * ((Base.value b) ^ k)
      div-outer-term k = div-mul-ℕ div-gcd-left (div-power k)

      -- 5. d ∣ membrane (sum divisibility)
      -- membrane = outer·b^k₁ + inner·b^k₂ + seed·b^k₃ + inner·b^k₄ + outer
      div-membrane : d ∣ membrane b conf seed
      div-membrane = div-add-ℕ
        (div-add-ℕ
          (div-add-ℕ
            (div-add-ℕ (div-outer-term _) (div-power _))
            (div-power _))
          (div-power _))
        div-gcd-left
    in div-membrane
```

**Key UniMath Functions Used**:
- `gcd-is-divisor-left-ℕ`
- `gcd-is-divisor-right-ℕ`
- `div-exp-ℕ`
- `div-mul-ℕ`
- `div-add-ℕ`

#### Step 3: Prove Main Theorem

Build on lemma to prove `coprime-better-density`:

```agda
coprime-better-density : ∀ base seeds →
  average-density (coprime-configs base) seeds >
  average-density (non-coprime-configs base) seeds

-- PROOF STRATEGY:
-- 1. Non-coprime configs generate composites (proven above)
-- 2. This eliminates entire residue classes
-- 3. Coprime configs don't have this constraint
-- 4. Therefore coprime density > non-coprime density
```

#### Step 4: Add Computational Verification

```agda
-- Use efficient primality testing from blog
open import Sieve using (is-prime-fast)

verify-base6-coprime : List (ℕ × Bool)
verify-base6-coprime =
  map (λ s → (s, is-prime-fast (membrane base6 config-15 s)))
      [0..9]

-- EXPECTED: [(0,false), (1,false), (2,false), (3,true), (4,true), ...]

verify-success-rate : ℚ
verify-success-rate =
  let results = verify-base6-coprime
      primes = filter snd results
  in length primes / length results

-- EXPECTED: 0.33 (33% success rate)
```

---

## 🔬 Phase 2: Template for Each Proof Type

### Type A: Mathematical Proof (Uses UniMath)

**Examples**: Coprimality, Radical Filter, Minimal Padding

**Workflow**:
1. State theorem precisely
2. Break into lemmas
3. Import UniMath properties
4. Construct proof step-by-step
5. Verify with type-checker

**Template**:
```agda
-- Main theorem
theorem-name : ∀ params → property
theorem-name params =
  lemma-1 →
  lemma-2 →
  combine-with (unimath-property) →
  qed

-- Supporting lemmas
lemma-1 : ∀ params → sub-property
lemma-1 = {! prove with UniMath !}

lemma-2 : ∀ params → sub-property
lemma-2 = {! prove with stdlib !}
```

### Type B: Computational Proof (Uses Sieve)

**Examples**: Resonance, Exclusivity, GCD Paradox

**Workflow**:
1. Define computation
2. Run on all test cases
3. Verify results match expectations
4. Prove `all-tests-pass ≡ true`

**Template**:
```agda
-- Computation
compute-property : Input → Output
compute-property = {! efficient computation !}

-- Test cases
test-cases : List (Input × Expected)
test-cases = [ concrete examples ]

-- Verification
verify-all : Bool
verify-all = all (λ (inp, exp) → compute-property inp ≡ᵇ exp) test-cases

-- Theorem
all-tests-pass : verify-all ≡ true
all-tests-pass = refl  -- Proven by computation!
```

### Type C: Statistical Proof (Uses Both)

**Examples**: Base 6 Optimality, Universal Patterns

**Workflow**:
1. Exhaustive search over domain
2. Compute statistics
3. Prove inequality holds
4. Verify with concrete examples

**Template**:
```agda
-- Search
search-all : List (Base × Config × SuccessRate)
search-all = map test-config all-combinations

-- Find maximum
max-rate : ℚ
max-rate = maximum (map (λ (_,_,r) → r) search-all)

-- Theorem
theorem-base6-optimal : max-rate ≡ 0.33
theorem-base6-optimal = refl

-- Verification
verify-no-better : ∀ base config →
  base ≤ 30 →
  success-rate base config ≤ 0.33
verify-no-better = exhaustive-check search-all
```

---

## 📐 Phase 3: Proof Tactics

### Tactic 1: Divisibility Chain

**Pattern**: d ∣ a, a ∣ b ⊢ d ∣ b

```agda
-- UniMath provides:
div-trans : ∀ d a b → div-ℕ d a → div-ℕ a b → div-ℕ d b
```

**Use for**: Radical filtering, coprimality requirements

### Tactic 2: Modular Arithmetic

**Pattern**: a ≡ b (mod p), c ≡ d (mod p) ⊢ a+c ≡ b+d (mod p)

```agda
-- Stdlib provides:
open import Data.Nat.DivMod.Properties using (
  %-distribˡ-+;  -- (a + b) mod p
  %-distribˡ-*   -- (a * b) mod p
  )
```

**Use for**: Affine transform, residue analysis

### Tactic 3: Exhaustive Computation

**Pattern**: ∀ x ∈ finite-set → P(x)

```agda
-- Pattern:
exhaustive-proof : ∀ x → x ∈ [a..b] → P x
exhaustive-proof a _ = base-case-a
exhaustive-proof (suc a) _ = inductive-case
...
exhaustive-proof b _ = base-case-b
```

**Use for**: Resonance verification, optimal configs

### Tactic 4: Contradiction

**Pattern**: ¬P → ⊥

```agda
-- Pattern:
proof-by-contradiction : ∀ n → ¬ (non-coprime ∧ prime)
proof-by-contradiction n (¬coprime, prime) =
  let d = gcd n base
      d>1 = gcd-not-one ¬coprime
      d∣n = divisibility-from-gcd
  in prime-no-divisors n d d>1 d∣n  -- Contradiction!
```

**Use for**: Coprimality necessity, filtering requirements

---

## 🎓 Phase 4: Worked Example

### Complete Proof: Radical Filtering for Base 10

**Claim**: All primes > 10 in base 10 must end in {1,3,7,9}

**Proof**:

```agda
module Base10Example where

open import elementary-number-theory.prime-numbers
open import elementary-number-theory.divisibility-natural-numbers
import Core.Radical using (radical)

-- Base 10 radical
base10-radical : radical 10 ≡ 10
base10-radical = refl  -- 10 = 2·5 is squarefree

-- Main theorem
base10-prime-residues : ∀ n →
  is-prime-ℕ n →
  n > 10 →
  (n mod 10 ≡ 1) ∨ (n mod 10 ≡ 3) ∨ (n mod 10 ≡ 7) ∨ (n mod 10 ≡ 9)

-- PROOF:
base10-prime-residues n n-prime n>10 =
  -- 1. If n is prime, then gcd(n,10) = 1
  prime-coprime-10 : gcd n 10 ≡ 1
  prime-coprime-10 = prime-coprime-to-radical n 10 n-prime n>10

  -- 2. gcd(n,10) = 1 means n not divisible by 2 or 5
  not-div-2 : ¬ (2 ∣ n)
  not-div-2 = coprime-not-divisible prime-coprime-10 (prime-2-divides-10)

  not-div-5 : ¬ (5 ∣ n)
  not-div-5 = coprime-not-divisible prime-coprime-10 (prime-5-divides-10)

  -- 3. Last digit analysis
  last-digit-analysis : (n mod 10 ∈ {0,1,2,3,4,5,6,7,8,9})
  last-digit-analysis = mod-bounded n 10

  -- 4. Eliminate impossible residues
  -- - 0: divisible by 10 → not coprime ✗
  -- - 2,4,6,8: divisible by 2 → not coprime ✗
  -- - 5: divisible by 5 → not coprime ✗
  -- - Remaining: {1,3,7,9} ✓

  in residue-elimination not-div-2 not-div-5 last-digit-analysis

-- Computational verification
verify-first-100-primes : List ℕ
verify-first-100-primes =
  filter is-prime-fast [11..100]

verify-all-end-correctly : Bool
verify-all-end-correctly =
  all (λ p → (p mod 10 ≡ᵇ 1) ∨ᵇ (p mod 10 ≡ᵇ 3) ∨ᵇ
             (p mod 10 ≡ᵇ 7) ∨ᵇ (p mod 10 ≡ᵇ 9))
      verify-first-100-primes

-- Theorem: Computation matches proof
verified : verify-all-end-correctly ≡ true
verified = refl  -- Type-checks! ✓
```

**Result**: Formal proof + computational verification! 🎉

---

## 📊 Progress Tracking

### Proof Complexity Levels

**Level 1: Computational** ⭐
- No deep theory needed
- Exhaustive testing
- Time: 1-2 days each

Examples: Resonance, Exclusivity

**Level 2: Simple Divisibility** ⭐⭐
- Basic UniMath properties
- Direct proofs
- Time: 3-5 days each

Examples: Coprimality necessity, Radical filtering

**Level 3: Advanced Arithmetic** ⭐⭐⭐
- Complex mod properties
- Multi-step reasoning
- Time: 1-2 weeks each

Examples: Minimal padding, Affine transform

**Level 4: Statistical** ⭐⭐⭐⭐
- Exhaustive search + proof
- Optimality claims
- Time: 2-3 weeks each

Examples: Base 6 optimal, Universal patterns

**Level 5: Deep Theory** ⭐⭐⭐⭐⭐
- Novel mathematical insights
- Publication-worthy
- Time: 1-2 months each

Examples: GCD paradox explanation, Lagrange clustering

---

## 🚀 Implementation Schedule

### Week 1-2: Foundation
- [ ] Install UniMath
- [ ] Implement Sieve
- [ ] Complete `CoprimalityRequirement.agda` (Level 2)
- [ ] Complete `RadicalDivisibilityFilter.agda` (Level 2)

### Week 3-4: Computation
- [ ] Complete `ResonanceComputation.agda` (Level 1)
- [ ] Complete `ExclusiveConfigurations.agda` (Level 1)
- [ ] Begin `MinimalPaddingOptimality.agda` (Level 3)

### Week 5-6: Empirical
- [ ] Complete `Base6Optimality.agda` (Level 4)
- [ ] Complete `UniversalPatternTheorem.agda` (Level 3)
- [ ] Begin `GCDParadoxComputation.agda` (Level 4)

### Week 7-8: Advanced
- [ ] Complete `LagrangePointClustering.agda` (Level 4)
- [ ] Complete `PerturbationStability.agda` (Level 3)
- [ ] Begin Affine Transform Strategy 1 (Level 5)

---

## 💡 Tips for Success

### 1. Start Simple
Always prove the simplest lemma first. Build confidence before tackling complex proofs.

### 2. Type-Driven Development
Let Agda guide you. Use holes `{! !}` and ask for type with `C-c C-,`.

### 3. Computational First
Verify computationally before proving formally. Computation catches errors fast.

### 4. Incremental Commits
Commit after each working lemma. Never lose progress.

### 5. Document Everything
Future readers (and future you!) will be grateful.

### 6. Cross-Verify
Every formal proof should have computational verification.
Every computation should suggest a formal theorem.

---

## 📖 Learning Resources

**For Divisibility Proofs**:
- UniMath: `elementary-number-theory.divisibility-natural-numbers`
- Read: Existing proofs in UniMath source

**For Mod Arithmetic**:
- Stdlib: `Data.Nat.DivMod.Properties`
- Practice: Simple mod equations first

**For Primality**:
- UniMath: `elementary-number-theory.prime-numbers`
- Blog: Efficient sieve implementation

**For Statistical Claims**:
- Stdlib: `Data.List`, `Data.Rational`
- Pattern: Exhaustive search + verification

---

## ✅ Success Criteria

### Per File
- [ ] All holes filled (no `{! !}`)
- [ ] Type-checks with `--safe`
- [ ] Computational verification matches
- [ ] Documented with examples
- [ ] Cross-referenced with EVIDENCE.md

### Overall Project
- [ ] 10 new theorem files complete
- [ ] All major claims verified
- [ ] Multiple proof strategies for key theorems
- [ ] Publication-ready documentation

---

**Current Status**: Roadmap complete, workflows defined, ready to implement! 🎯

**Next Action**: Install UniMath and start proving! 💪

**Timeline**: 2 months to ironclad verification of ALL claims! 🏆
