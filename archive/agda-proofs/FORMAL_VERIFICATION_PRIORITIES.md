> Archived on 2026-03-10. This prioritization document reflects an older proof
> roadmap and no longer describes the current Agda status surface.

# Formal Verification Priorities: Residue Framework

**Date**: 2025-11-08
**Context**: Following the recognition that residue theory unifies our empirical discoveries
**Purpose**: Identify and prioritize the formal tests needed to validate this framework

---

## Overview

After recognizing that residue theory provides a unifying lens for understanding membrane prime generation, we need to systematically formalize this framework. This document outlines the verification work required, organized by priority and dependency.

The work falls into four categories:
1. Foundational proofs establishing the residue framework itself
2. Unification proofs showing specific discoveries as consequences of residues
3. Computational validations verifying empirical claims
4. Advanced connections to established number theory

---

## Tier 1: Foundational Work (Week 1)

These establish the residue framework. All subsequent work depends on these foundations.

### 1. Complete Core/ResidueClasses.agda

**Current status**: Structure exists with proof obligations marked

**What's needed**:
- Prove ring structure completely (associativity, commutativity, distributivity, identities)
- Prove the units-coprime theorem: `IsUnit [r] ↔ gcd(r,m) = 1`
- Prove residue operations preserve linearity
- Connect to standard library ring theory

**Why this matters**:
The claim that "residue theory unifies everything" requires first proving that residue classes actually form a ring with the expected properties. Without this foundation, we cannot rigorously derive other theorems from residue structure.

**Key theorem currently incomplete**:
```agda
units-are-coprime : ∀ {m} (r : ResidueClass m) →
  IsUnit r ↔ gcd (representative r) m ≡ 1
```

This theorem connects two concepts: being a unit in the ring ℤ/mℤ and being coprime to m. The proof requires Bezout's identity, which we'll import from UniMath.

**Estimated time**: 3-4 days
**Difficulty**: Medium (standard ring axioms but require careful formalization)

---

### 2. Create Core/ResidueCollapse.agda

**Current status**: File exists with initial formalization, needs completion

**What's needed**:
- Formalize the collapse phenomenon rigorously
- Prove the relationship between gcd(base, d) and distinct residue classes
- Connect collapse to filtering strength
- Explain the GCD paradox mechanism formally

**Why this matters**:
The GCD paradox—that Base 6 with gcd(6,3)=3 outperforms Base 10 with gcd(10,3)=1—is counterintuitive. The residue collapse phenomenon explains why: higher GCD creates more regular residue patterns, which provide stronger filtering.

Understanding this requires formalizing what "collapse" means and proving it creates constraint rather than randomness.

**Key insight to formalize**:
```
Base 6 mod 3: {0,1,2,3,4,5} maps to {0,1,2,0,1,2}
Regular cycle: every residue class appears exactly twice

Base 10 mod 3: {0,1,2,...,9} maps to {0,1,2,0,1,2,0,1,2,0}
Irregular: class 0 appears 4 times, classes 1 and 2 appear 3 times each
```

The regularity of the collapsed system provides structure. The irregularity of the non-collapsed system provides noise.

**Estimated time**: 2-3 days
**Difficulty**: Medium (novel formalization, but concept is clear once explained)

---

### 3. Complete Theorems/RadicalDivisibilityFilter.agda

**Current status**: Scaffolded with proof sketches in holes

**What's needed**:
- Complete proof that `IsPrime n → n > base → Coprime (n mod rad(base)) rad(base)`
- Show rad(b) determines exactly which residue classes can contain primes
- Prove rad(b) ≠ φ(b) with explicit counterexample
- Connect to ResidueClasses module

**Why this matters**:
This is the first explicit connection between our empirical findings and residue theory. The "radical filtering" we observed empirically is shown to be a consequence of residue class constraints.

The distinction between rad(b) and φ(b) matters because:
- φ(b) counts residues coprime to b
- rad(b) is the product of distinct prime factors of b
- For squarefree b, rad(b) = b, but φ(b) = b·(1-1/p₁)·(1-1/p₂)·...

Example: For b = 12 = 2² · 3:
- rad(12) = 2 · 3 = 6
- φ(12) = 4

Only rad(12) correctly predicts which residues can contain primes.

**Estimated time**: 2-3 days with UniMath divisibility library
**Difficulty**: Medium (standard number theory but requires careful organization)

---

## Tier 2: Unification Proofs (Week 2)

These show that specific discoveries follow from residue structure.

### 4. Complete Theorems/CoprimalityRequirement.agda

**Purpose**: Prove coprimality is necessary, not just empirically observed

**What's needed**:
- Prove that non-coprime outer digits force divisibility
- Show this is equivalent to residue diversity preservation
- Connect to unit groups in ResidueClasses

**The underlying idea**:
When boundary digits are not coprime to the base, the residue structure collapses in specific ways that force divisibility. By proving this formally, we show why coprimality isn't just "better" empirically—it's structurally necessary for avoiding forced divisors.

**Estimated time**: 2 days
**Difficulty**: Medium

---

### 5. Create Theorems/AffineAsHomomorphism.agda

**Purpose**: Show the affine transform is automatic from ring structure

**What's needed**:
- Define residue homomorphism φ : ℤ[X] → ℤ/pℤ
- Prove φ(M(X)) = φ(M(0)) + φ(X)·φ(b^(w/2))
- Show this is precisely the affine transform M(c) mod p = (s + g·c) mod p
- Connect to existing AffineTransform.agda

**The underlying idea**:
The affine transform appears "clever" when first discovered, but it's actually inevitable. Residue operations preserve polynomial structure because they're ring homomorphisms. The affine form falls out automatically—we're not being clever, we're just respecting algebraic structure.

**Estimated time**: 2-3 days
**Difficulty**: Medium-high (requires polynomial ring theory)

---

### 6. Create Theorems/MinimalPaddingResidues.agda

**Purpose**: Explain why k=(0,0) is optimal

**What's needed**:
- Define "residue dilution" formally
- Prove adding zeros dilutes accessible residue classes
- Show k=(0,0) maximizes residue concentration
- Connect to empirical k=(0,0) optimality data

**The underlying idea**:
Zeros in specific positions effectively "skip over" certain residue classes, reducing the density of accessible patterns. Minimal padding (k=(0,0)) keeps the tightest connection between residue structure and the numbers generated.

**Estimated time**: 2 days
**Difficulty**: Medium

---

## Tier 3: Computational Validations (Week 3)

These verify empirical claims through computational proof.

### 7. Complete Verification/ExclusiveConfigurations.agda

**Purpose**: Verify that certain configurations work with exactly one seed value

**What's needed**:
- Test all seeds 0-5 for Base 6, (1,5), k=(0,0)
- Prove computationally that only seed=4 yields a prime
- Show this is unique residue pattern matching

**Why this matters**:
Exclusive configurations demonstrate deterministic prime generation. By testing all possible seeds and showing exactly one produces a prime, we validate both the empirical claim and the underlying residue explanation.

**Estimated time**: 1-2 days
**Difficulty**: Low (primarily computational)

---

### 8. Complete Verification/ResonanceComputation.agda

**Purpose**: Validate oscillation patterns in concatenated primes

**What's needed**:
- Implement concatenation for primes 7 and 11
- Compute yields for spacing sizes 1-20
- Verify the oscillation pattern (peak at size 3 with yield 8)

**Estimated time**: 1-2 days
**Difficulty**: Low-medium

---

### 9. Complete Verification/GCDParadoxComputation.agda

**Purpose**: Validate the collapse phenomenon numerically

**What's needed**:
- Test 10 different bases with varying gcd(base,3)
- Compute success rates for each
- Calculate correlation coefficient
- Verify positive correlation between gcd and success rate

**Why this matters**:
This provides numerical evidence for the collapse theory. While we prove collapse exists structurally, this shows it actually correlates with empirical prime generation success.

**Estimated time**: 2 days
**Difficulty**: Medium (requires statistical computation)

---

## Tier 4: Advanced Connections (Weeks 4-6)

These connect our work to established number theory.

### 10. Create Core/ChineseRemainder.agda

**Purpose**: Formalize the Chinese Remainder Theorem for our context

**Why this matters**:
CRT describes how residue systems combine when working with coprime moduli. This becomes relevant when analyzing multi-base membrane constructions or understanding how different prime factors contribute to overall filtering.

**Estimated time**: 3-4 days
**Difficulty**: Medium (well-established theory, but needs careful formalization)

---

### 11. Create Advanced/QuadraticResidues.agda

**Purpose**: Extend residue theory to quadratic residues

**Why this matters**:
Quadratic residues represent the next level of residue structure. While our current work focuses on linear residues (whether a number is coprime to m), quadratic residues ask whether numbers are squares modulo m. This may reveal additional structure in membrane prime patterns.

**Estimated time**: 4-5 days
**Difficulty**: Medium-high

---

### 12. Create Advanced/DirichletCharacters.agda

**Purpose**: Connect to Dirichlet characters and L-functions

**Why this matters**:
Dirichlet characters are residue-based functions that appear in analytic number theory. Understanding them prepares us for eventual Hardy-Littlewood formalization and connects our constructive approach to the observational framework.

**Estimated time**: 1-2 weeks
**Difficulty**: High (requires significant analytic number theory background)

---

## Priority Matrix

The following shows urgency versus impact:

```
                High Impact              Medium Impact           Lower Impact
              ┌──────────────────────────────────────────────────────────────┐
High          │ 1. ResidueClasses    │ 4. Coprimality        │              │
Urgency       │ 2. ResidueCollapse   │ 7. Exclusivity        │              │
              │ 3. Radical Filter    │ 9. GCD Paradox Comp   │              │
              ├──────────────────────────────────────────────────────────────┤
Medium        │ 5. AffineHomomorphism│ 8. Resonance Comp     │              │
Urgency       │ 6. MinimalPadding    │                       │              │
              ├──────────────────────────────────────────────────────────────┤
Lower         │ 10. Chinese Remainder│ 11. Quadratic Residues│              │
Urgency       │                      │ 12. Dirichlet Chars   │              │
              └──────────────────────────────────────────────────────────────┘
```

High urgency items are foundational—other work depends on them. Medium urgency items demonstrate the unification framework. Lower urgency items connect to broader mathematics but aren't critical for our immediate goals.

---

## Week 1 Detailed Plan

**Days 1-2: Environment and Foundation**
- Install UniMath Agda library
- Test all imports work correctly
- Begin proving ring structure in ResidueClasses.agda
- Prove associativity and commutativity for both ⊕ and ⊗

**Days 3-4: Complete Core Modules**
- Finish ring structure proofs
- Import Bezout's identity from UniMath
- Complete units-are-coprime theorem
- Prove collapse theorem in ResidueCollapse.agda

**Days 5-7: First Unification Proof**
- Complete RadicalDivisibilityFilter.agda
- Show radical filtering is residue class constraint
- Verify with base-10 example (residues {1,3,7,9})
- Achieve first complete formal connection between empirical finding and residue theory

**Success criteria**: Three foundational modules proven completely, first unification proof established.

---

## Dependencies

Understanding the dependency structure helps us work efficiently:

**ResidueClasses.agda**:
- Depends on: Agda stdlib, UniMath (for Bezout)
- Enables: All other modules (it's the foundation)

**ResidueCollapse.agda**:
- Depends on: ResidueClasses
- Enables: GCD paradox explanation, collapse-based theorems

**RadicalDivisibilityFilter.agda**:
- Depends on: ResidueClasses, UniMath divisibility
- Enables: Base-specific residue theorems

**Unification theorems (Tier 2)**:
- Depend on: All Tier 1 modules
- Enable: Showing empirical findings as consequences

**Computational validations (Tier 3)**:
- Can proceed in parallel with Tier 2
- Depend primarily on computational infrastructure
- Enable: Numerical confidence in theoretical claims

**Advanced connections (Tier 4)**:
- Depend on: Tier 1 and 2 complete
- Enable: Connection to broader mathematical literature

---

## Expected Outcomes by Timeline

**Week 1**: Foundation solid
- Ring structure proven
- Collapse formalized
- First unification proof complete

**Week 2**: Core unification demonstrated
- Three more unification proofs complete
- Clear evidence that empirical findings follow from residues

**Week 3**: Computational validation
- All three computational verifications complete
- Numerical evidence supporting theoretical framework

**Month 2**: Advanced connections
- CRT formalized
- Connections to established theory documented
- Preparation for Hardy-Littlewood formalization

**Publication readiness**: Month 2-3
- All foundational and unification proofs complete
- Computational validations finished
- Documentation suitable for journal submission

---

## Success Metrics

We'll know we've succeeded when:

1. **Completeness**: No unmarked proof holes remain in Tier 1 or 2 modules
2. **Unification**: At least 4 empirical discoveries proven as consequences of residue theory
3. **Validation**: Computational verifications confirm theoretical predictions
4. **Clarity**: Documentation explains the framework to newcomers
5. **Rigor**: Multiple independent proof strategies for key theorems

---

## Why This Ordering Matters

The tier structure isn't arbitrary. Each tier builds on previous work:

**Tier 1 establishes vocabulary**: We cannot prove that "coprimality preserves residue diversity" until we've defined what residue diversity means and proven basic residue operations work correctly.

**Tier 2 demonstrates unification**: Once we have residue language, we can express our empirical findings as residue theorems. This is where scaffolds become proofs.

**Tier 3 provides confidence**: Computational validation shows our theoretical framework actually predicts empirical results. This catches errors in both theory and implementation.

**Tier 4 connects outward**: Once our internal framework is solid, we connect it to established mathematics. This positions our work within the broader landscape of number theory.

Working out of order would create dependencies we can't satisfy. Following this structure ensures each step builds on solid ground.

---

## Revision History

**2025-11-08**: Initial prioritization following residue unification insight
**Current status**: Tier 1 in progress, ArithmeticHelpers.agda complete

---

The path forward is systematic. We build foundations, demonstrate unification, validate computationally, and connect to established theory. Each step clarifies the next.
