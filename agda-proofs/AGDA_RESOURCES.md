# Agda Formalization Resources

This document catalogs high-quality Agda resources that can strengthen our formal verification work.

## 📚 Core Resources

### 1. UniMath Agda Library - Elementary Number Theory
**URL**: https://unimath.github.io/agda-unimath/elementary-number-theory.prime-numbers.html

**Relevance**: ⭐⭐⭐⭐⭐ (Highest Priority)

**What It Provides**:
- Formal definitions of prime numbers in dependently-typed setting
- Divisibility proofs and GCD theory
- Fundamental theorem of arithmetic
- Modular arithmetic foundations

**How We Can Use It**:
```agda
-- Instead of postulating IsPrime, we can import from UniMath:
open import elementary-number-theory.prime-numbers
open import elementary-number-theory.divisibility
open import elementary-number-theory.modular-arithmetic

-- This gives us proven properties like:
-- - is-prime-ℕ : ℕ → UU
-- - is-divisible-by-ℕ : ℕ → ℕ → UU
-- - mod-ℕ : ℕ → ℕ → ℕ
```

**Integration Strategy**:
1. **Replace postulates in `PrimeConcepts.agda`**
   - Current: `postulate IsPrime : ℕ → Bool`
   - Replace with: `open import elementary-number-theory.prime-numbers`

2. **Strengthen `Radical.agda`**
   - Use UniMath's divisibility proofs for `radical-divides`
   - Import GCD properties for `radical-gcd-relationship`

3. **Improve `AffineTransform.agda`**
   - Use proven mod distributivity: `mod-+-dist`, `mod-*-dist`
   - Leverage proven properties of exponentiation mod p

**Action Items**:
- [ ] Review UniMath's prime number definitions
- [ ] Map our postulates to UniMath theorems
- [ ] Refactor `Core/Radical.agda` to use UniMath
- [ ] Replace `mod` postulates in `AffineTransform.agda`

---

### 2. Real Numbers in Agda (Lundfall)
**URL**: https://ncatlab.org/nlab/files/Lundfall-RealNumbersInAgda.pdf

**Relevance**: ⭐⭐ (Lower Priority for Now)

**What It Provides**:
- Construction of real numbers in type theory
- Cauchy sequences and completeness proofs
- Rigorous foundations for analysis

**How We Can Use It**:
- **Future Hardy-Littlewood Work**: When formalizing HL constants (C₂ ≈ 0.6601618)
- **Density Analysis**: Prime density functions need real-valued limits
- **Not Immediate**: Our current work focuses on natural numbers and mod arithmetic

**Potential Applications**:
```agda
-- For Hardy-Littlewood formalization:
C₂ : ℝ  -- Twin prime constant
C₂-approximation : 0.66 < C₂ < 0.67

-- For density theorems:
prime-density : ℕ → ℝ
density-limit : lim (λ n → prime-density n) ≈ 0  -- via PNT
```

**Action Items**:
- [ ] Study when we move to HL formalization (Phase 2+)
- [ ] Prepare for real-valued constants and limits

---

### 3. Primes in Agda (Doisinkidney Blog)
**URL**: https://doisinkidney.com/posts/2018-12-14-primes-in-agda.html

**Relevance**: ⭐⭐⭐⭐ (High Priority)

**What It Provides**:
- Practical prime number implementations in Agda
- Sieve of Eratosthenes formalization
- Primality testing with proofs
- Performance-oriented constructive approaches

**How We Can Use It**:
- **Computational Proof Strategy**: Practical techniques for Strategy 3
- **Test Case Generation**: How to compute primes efficiently in Agda
- **Primality Certificates**: Verified primality checking

**Integration Strategy**:
```agda
-- In AffineTransformComputation.agda, we can use:
open import Sieve  -- From blog's implementation

-- Generate verified prime list
verified-primes : Vec Prime 100
verified-primes = sieve-eratosthenes 100

-- Use for test case expansion
test-with-all-primes : ∀ p ∈ verified-primes →
  verify-affine-transform base conf seed p
```

**Action Items**:
- [ ] Study sieve implementation for efficiency
- [ ] Integrate primality checking into test suite
- [ ] Use for automatic test case generation

---

### 4. Agda Language Documentation
**URL**: https://agda.readthedocs.io/en/v2.6.1/language/index.html

**Relevance**: ⭐⭐⭐⭐⭐ (Essential Reference)

**What It Provides**:
- Complete language specification
- Pattern matching and dependent types
- Termination checking and positivity
- Module system and imports

**How We Can Use It**:
- **Reference for Advanced Features**: When implementing complex proofs
- **Performance Optimization**: Understanding Agda's evaluation model
- **Debugging**: When proofs don't type-check

**Key Sections for Our Work**:
1. **Pattern Matching** (for case analysis in proofs)
2. **Termination Checking** (ensure our recursive functions are total)
3. **Pragma System** (use `--safe` flag appropriately)
4. **Mixfix Operators** (clean notation for membrane operations)

**Action Items**:
- [ ] Review pattern matching for complex proofs
- [ ] Ensure all functions pass termination checking
- [ ] Use `--safe` flag for all theorem files

---

### 5. Agda Standard Library v2.3
**URL**: https://agda.github.io/agda-stdlib/v2.3/

**Relevance**: ⭐⭐⭐⭐⭐ (Foundation of Everything)

**What It Provides**:
- `Data.Nat`: Natural number arithmetic
- `Data.Nat.DivMod`: Division and modulo with proofs
- `Data.Nat.Primality`: Prime number predicates
- `Relation.Binary.PropositionalEquality`: Equality reasoning
- `Data.Fin`: Finite types (for digits in base b)

**Current Usage**:
```agda
-- Already using in our files:
open import Data.Nat using (ℕ; _+_; _*_; _^_)
open import Data.Nat.DivMod using (_mod_; _div_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
```

**Additional Useful Modules**:
```agda
-- For membrane proofs:
open import Data.Nat.Properties using (
  +-comm; +-assoc; *-distribˡ-+;
  +-mono-≤; *-mono-≤
  )

-- For mod arithmetic:
open import Data.Nat.DivMod.Properties using (
  m%n≡m∸m/n*n;  -- Fundamental mod property
  %-distribˡ-+;  -- Mod distributivity
  %-distribˡ-*   -- Mod distributivity for multiplication
  )

-- For finite types (digits):
open import Data.Fin.Properties using (
  toℕ<n;  -- Digit is less than base
  toℕ-injective  -- Digit equality
  )
```

**Action Items**:
- [ ] Review `Data.Nat.DivMod.Properties` for mod lemmas
- [ ] Import proven properties instead of postulating
- [ ] Use `Data.Vec` for membrane digit sequences

---

## 🎯 Integration Roadmap

### Phase 1: Replace Postulates (Week 1-2)
**Goal**: Replace our postulates with proven theorems from libraries

1. **UniMath Integration**
   ```agda
   -- In PrimeConcepts.agda
   - postulate IsPrime : ℕ → Bool
   + open import elementary-number-theory.prime-numbers
   ```

2. **Stdlib Properties**
   ```agda
   -- In AffineTransform.agda
   - postulate mod-+-dist : (a + b) mod p ≡ ((a mod p) + (b mod p)) mod p
   + open import Data.Nat.DivMod.Properties using (%-distribˡ-+)
   ```

3. **Verify Imports**
   - Ensure all files type-check with real proofs
   - No postulates except for empirically verified claims

### Phase 2: Computational Efficiency (Week 3-4)
**Goal**: Use efficient primality testing from blog post

1. **Sieve Integration**
   - Import sieve implementation
   - Generate test primes efficiently
   - Expand test suite to 50-100 cases

2. **Verified Computation**
   - Use blog's primality certificates
   - Prove test cases with concrete primes
   - Cross-check with Rust implementation

### Phase 3: Advanced Formalization (Month 2+)
**Goal**: Prepare for Hardy-Littlewood work

1. **Real Numbers**
   - Study Lundfall's construction
   - Formalize C₂ constant
   - Prove convergence properties

2. **Density Functions**
   - Define prime density formally
   - Use real numbers for limits
   - Connect to PNT framework

---

## 📖 Learning Path

### For Newcomers to Agda
1. **Start**: Agda Language Documentation (basics)
2. **Practice**: Primes in Agda blog (concrete examples)
3. **Reference**: Standard Library v2.3 (daily use)
4. **Advanced**: UniMath (when ready for abstract algebra)

### For Our Project
1. **Immediate**: UniMath elementary number theory
2. **Next**: Primes in Agda blog (computational strategy)
3. **Soon**: Agda stdlib DivMod properties
4. **Later**: Real numbers in Agda (HL formalization)

---

## 🔗 Quick Reference Links

| Resource | Use For | Priority |
|----------|---------|----------|
| [UniMath](https://unimath.github.io/agda-unimath/elementary-number-theory.prime-numbers.html) | Prime definitions, GCD proofs | ⭐⭐⭐⭐⭐ |
| [Primes Blog](https://doisinkidney.com/posts/2018-12-14-primes-in-agda.html) | Sieve, efficiency, testing | ⭐⭐⭐⭐ |
| [Agda Docs](https://agda.readthedocs.io/en/v2.6.1/language/index.html) | Language features | ⭐⭐⭐⭐⭐ |
| [Agda Stdlib](https://agda.github.io/agda-stdlib/v2.3/) | Foundation libraries | ⭐⭐⭐⭐⭐ |
| [Real Numbers](https://ncatlab.org/nlab/files/Lundfall-RealNumbersInAgda.pdf) | HL constants, analysis | ⭐⭐ |

---

## ✅ Action Items Summary

**Immediate (This Week)**:
- [ ] Review UniMath prime number module
- [ ] Replace `IsPrime` postulate with UniMath import
- [ ] Import mod distributivity from stdlib
- [ ] Study primes blog for sieve implementation

**Short-term (This Month)**:
- [ ] Integrate sieve for test case generation
- [ ] Expand computational verification to 50+ cases
- [ ] Refactor `Radical.agda` with UniMath divisibility
- [ ] Remove all non-empirical postulates

**Long-term (Next Quarter)**:
- [ ] Study real numbers for HL formalization
- [ ] Formalize C₂ constant with Lundfall's approach
- [ ] Prepare density function formalization
- [ ] Cross-validate with Rust QuickCheck

---

**Last Updated**: 2025-11-08
**Status**: Resource catalog complete, ready for integration
**Next Step**: Begin UniMath integration in `PrimeConcepts.agda`
