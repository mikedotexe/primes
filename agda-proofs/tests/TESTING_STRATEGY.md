# Testing Strategy: Computational Validation of Formal Proofs

**Date**: 2025-11-08
**Purpose**: Document how test specifications validate our formal verification work
**Context**: Integration of executable test modules inspired by ZetaWalker's approach

---

## Overview

We've integrated three test specification modules that validate our formal proofs through computation. This document explains what these tests do, why they matter, and how they fit into our verification strategy.

---

## The Core Idea

**Traditional proof approach**: Write theorems, prove them abstractly, hope they're correct.

**Our enhanced approach**: Write theorems, prove them abstractly, then validate they compute correctly on concrete examples.

The difference: computational validation catches errors that purely abstract reasoning might miss. If a theorem doesn't normalize to `refl` on actual values, something is wrong—either with the proof, the theorem statement, or our understanding of what should be true.

---

## What We're Testing

### 1. Base10ResidueFilterSpec.agda

**What it tests**: The theorem that all primes greater than 10 end in {1,3,7,9}.

**How it tests**:
```agda
test-11 : valid-prime-residue 11 ≡ true
test-11 = prime-residue-theorem 11 prime-11 10<11
```

This applies the actual theorem to the number 11, with proofs that:
- 11 is prime (`prime-11`)
- 10 < 11 (`10<11`)

If the theorem is correct and computes properly, this normalizes to:
```
valid-prime-residue 11
  = (11 mod 10 ≡ᵇ 1) ∨ (11 mod 10 ≡ᵇ 3) ∨ (11 mod 10 ≡ᵇ 7) ∨ (11 mod 10 ≡ᵇ 9)
  = (1 ≡ᵇ 1) ∨ ...
  = true ∨ ...
  = true
```

The equality `test-11 : true ≡ true` normalizes to `refl`.

**Why it matters**: This validates that our theorem actually computes the right answer for specific primes. We test eight different primes (11, 13, 17, 19, 23, 29, 31, 37), covering all four valid residues (1, 3, 7, 9).

**Current status**: Awaiting completion of `prime-residue-theorem` proof. Once the proof is finished (no holes), these tests should compile and pass.

---

### 2. ResidueClassesRingSpec.agda

**What it tests**: The ring axioms for ℤ/mℤ.

**How it tests**:
```agda
test-add-assoc-10 : (A10 ⊕ B10) ⊕ C10 ≡ᵣ A10 ⊕ (B10 ⊕ C10)
test-add-assoc-10 = ⊕-assoc A10 B10 C10
```

This constructs three specific residue classes in ℤ/10ℤ:
- A10 = [3]₁₀
- B10 = [7]₁₀
- C10 = [9]₁₀

Then applies the associativity theorem and verifies both sides compute to the same value.

**Why it matters**: Ring structure is foundational. Everything builds on the claim that ℤ/mℤ is a commutative ring. If ring axioms don't hold computationally, nothing built on top can be trusted.

We test:
- Associativity (addition and multiplication)
- Commutativity (addition and multiplication)
- Identity elements (zero and one)
- Distributivity

We test multiple moduli (ℤ/10ℤ and ℤ/7ℤ) to ensure proofs work generally.

**Current status**: Should compile once ResidueClassesComplete.agda is finalized. Ring structure proofs are complete, though some underlying mod distribution properties are postulated.

---

### 3. ResidueCollapseSpec.agda

**What it tests**: Frequency distributions of residues, validating our understanding of the collapse phenomenon.

**How it tests**:
```agda
test-freqs-6-3 : freqs 6 3 ≡ (2 ∷ 2 ∷ 2 ∷ [])
test-freqs-6-3 = refl
```

This computes the frequency vector for Base 6, divisor 3:
1. Generate digits: [0, 1, 2, 3, 4, 5]
2. Map to residues mod 3: [0, 1, 2, 0, 1, 2]
3. Count frequencies: [2, 2, 2]
4. Verify this matches our prediction

**Why it matters**: This validates our refined understanding of collapse. The key insight is that collapse isn't about fewer residues appearing (they all appear), but about HOW REGULARLY they appear.

Comparing Base 6 vs Base 10 (both mod 3):
- Base 6: [2, 2, 2] — perfectly regular
- Base 10: [4, 3, 3] — slightly irregular

The regularity difference explains why Base 6 outperforms Base 10.

We test four different (base, divisor) pairs to verify the pattern holds generally and matches theoretical predictions about coset structure.

**Current status**: These are computational proofs that normalize to `refl` immediately. They're complete and validate our frequency computation logic.

---

## The Testing Pattern

All three specs follow the same pattern:

**1. Construct concrete values**
```agda
A10 : ResidueClass 10
A10 = [ 3 ]mod 10 ⦃ proof-3<10 ⦄
```

**2. Apply the theorem**
```agda
test : some-property A10 ≡ expected-result
test = theorem A10 additional-proofs
```

**3. Verify normalization**

If the theorem computes correctly, both sides reduce to the same normal form, and the equality is witnessed by `refl`.

If normalization fails, we've found an error.

---

## Why This Approach Works

### Catching Errors

**Proof errors**: If a proof has a logical error, it might type-check but compute incorrectly. Tests catch this.

**Implementation errors**: If we misimplement a function (e.g., wrong arithmetic in residue addition), tests catch this.

**Specification errors**: If our theorem statement doesn't match what we think it says, tests catch this.

### Building Confidence

Each passing test validates both:
- The abstract proof is correct
- The proof computes correctly on concrete values

This is stronger than either alone.

### Documentation by Example

Tests show how to use theorems correctly. They're executable documentation.

---

## Integration with Broader Verification

These Agda tests are the first layer of a multi-level verification strategy:

**Level 1: Agda tests** (this document)
- Validate formal proofs compute correctly
- Test on small, hand-verified examples
- Catch proof errors before deployment

**Level 2: Rust computational tests** (planned)
- Validate empirical claims on larger datasets
- Test membrane generation with actual primality checking
- Cross-validate Agda predictions with computational results

**Level 3: Cross-validation** (planned)
- Agda proves a theorem about frequency distributions
- Rust computes frequency distributions for large bases
- Results match: both approaches validated

---

## Extending the Test Suite

The three current specs establish the pattern. We can extend this to test:

### Planned: ResidueClassesUnitsSpec.agda

Once `units-are-coprime` theorem is proven, test that:
```agda
-- For ℤ/7ℤ, [3] should have inverse [5]
test-inverse-3-7 : [ 3 ]mod 7 ⊗ [ 5 ]mod 7 ≡ᵣ [ 1 ]mod 7
test-inverse-3-7 = compute-via-theorem
```

This validates that:
- The units-are-coprime theorem is correct
- Inverses actually compute correctly
- Our understanding of the unit group is accurate

### Planned: Base6ResidueFilterSpec.agda

Once we prove the base-6 analog of the base-10 theorem:
```agda
test-7 : valid-prime-residue-6 7 ≡ true
test-7 = prime-residue-theorem-6 7 prime-7 6<7
```

This validates our empirical finding that Base 6 primes end in {1, 5}.

### Planned: AffineTransformSpec.agda

Once affine transform is proven:
```agda
test-affine-11-seed-3 : membrane 10 (3,7) 3 mod 11 ≡ affine-eval 10 (3,7) 3 11
test-affine-11-seed-3 = affine-transform-theorem ...
```

This validates the O(1) optimization actually computes the same result as the O(w) membrane construction.

---

## Relationship to Formal Verification

**These tests don't replace formal proofs**. They supplement them.

**Formal proof** tells us: "This theorem is logically correct under these axioms."

**Computational test** tells us: "This theorem computes the expected result for this specific input."

Together: "This theorem is both logically sound and computationally correct."

---

## Running the Tests

Once proofs are complete, running tests is simple:

```bash
# Compile all test specs
agda tests/Spec/Base10ResidueFilterSpec.agda
agda tests/Spec/ResidueClassesRingSpec.agda
agda tests/Spec/ResidueCollapseSpec.agda
```

Success means:
- All modules type-check
- All test equalities hold by `refl`
- No errors in proofs or computations

Failure means:
- Either a proof error
- Or a computation error
- Or a misunderstanding of what should be true

Debugging starts by examining which specific test fails and why normalization doesn't reach `refl`.

---

## Current Test Status

| Module | Status | Depends On | Next Steps |
|--------|--------|------------|------------|
| Base10ResidueFilterSpec | Awaiting proof | prime-residue-theorem | Complete theorem proof |
| ResidueClassesRingSpec | Ready | ResidueClassesComplete | Compile and verify |
| ResidueCollapseSpec | Complete | None (computational) | Extend with more cases |

---

## Expected Timeline

**Week 1**:
- Complete proofs in Base10ResidueFilter and ResidueClassesComplete
- Compile all three test specs
- Verify all tests pass

**Week 2**:
- Add ResidueClassesUnitsSpec
- Add Base6ResidueFilterSpec
- Begin AffineTransformSpec (awaiting theorem proof)

**Week 3-4**:
- Complete AffineTransformSpec
- Add computational tests for exclusivity, resonance, GCD paradox
- Begin parallel Rust test suite

**Month 2**:
- Cross-validation between Agda and Rust
- Systematic testing of all empirical claims
- Publication-ready test coverage

---

## Value Proposition

**For developers**: Tests catch errors early, before publication.

**For reviewers**: Tests provide concrete validation of abstract claims.

**For users**: Tests document correct usage patterns.

**For science**: Tests make claims reproducible and falsifiable.

---

## Philosophical Note

These tests embody a principle: **formal verification should be executable**.

A proof that doesn't compute is suspect. A computation without proof is untrustworthy. Together, they provide strong validation.

This is the synthesis of:
- Formal methods (rigorous proof)
- Software engineering (automated testing)
- Experimental mathematics (computational validation)

The result is verification we can actually trust.

---

## Next Steps

**Immediate**:
1. Complete theorem proofs that tests depend on
2. Compile all test specs and verify they pass
3. Document any failures and fix underlying issues

**Short-term**:
4. Add unit tests for `units-are-coprime`
5. Add tests for remaining empirical claims
6. Establish regression testing workflow

**Long-term**:
7. Create parallel Rust test suite
8. Implement cross-validation framework
9. Achieve comprehensive test coverage of all formal claims

---

The path from theory to trust goes through computation. These tests walk that path systematically.
