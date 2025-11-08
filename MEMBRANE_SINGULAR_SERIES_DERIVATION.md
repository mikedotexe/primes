# Membrane Singular Series: Hardy-Littlewood Derivation

**Date**: 2025-11-08
**Goal**: Derive theoretical formula for membrane success rate from phase lock structure
**Approach**: Adapt gap singular series to membrane configuration constraints

---

## Motivation

We've empirically validated that membrane success rate follows:
```
success ≈ 50 × density
where density = phase_locks / (base/4)
```

With correlation r = 0.996 across five 2p bases.

**Question**: Can we derive this relationship from first principles using Hardy-Littlewood theory?

If successful, this would:
1. Explain WHY phase lock density predicts success
2. Provide theoretical correction factors (explaining the slight overestimation)
3. Enable a priori predictions without empirical testing
4. Connect membrane generation to established number theory

---

## Hardy-Littlewood Background

### Gap Singular Series

For prime gaps of size g, the singular series is:
```
S(g) = 2·C₂ · ∏_{p|g/2, p>2} (p-1)/(p-2)
```

where:
- C₂ ≈ 0.660 is the twin prime constant
- Product is over odd primes dividing g/2
- Formula estimates density of prime pairs with gap g

### Key Insight

Membranes impose **multiple constraints simultaneously**:
1. Specific residue classes (from base-N representation)
2. Symmetric structure (left boundary = right boundary reflected)
3. Phase lock positions (primes at specific distances from midpoint)
4. Coprimality requirements (boundaries coprime to base)

Each constraint reduces the "probability" that a random number is prime.

---

## Membrane Configuration Anatomy

### Structure

For base b, boundaries (outer, inner), seed S:
```
M = outer·b^k₁ + inner·b^k₂ + S·b^k₃ + inner·b^k₄ + outer·b^k₅
```

In base-b notation:
```
M = [outer, 0^k₁, inner, 0^k₂, S, 0^k₂, inner, 0^k₁, outer]_b
```

### Constraints

**C1. Base-b residue structure**:
- Outer digit at specific positions
- Inner digit at specific positions
- Seed in middle
- Symmetric structure

**C2. Phase lock constraint**:
- outer + inner ≡ 0 (mod something?)
- Actually: phase lock means outer and inner are specific primes
- They're not arbitrary - they're the phase lock pair for base b

**C3. Coprimality**:
- gcd(outer, b) = 1
- gcd(inner, b) = 1
- This is automatic for phase locks in 2p bases

**C4. Symmetry**:
- Left half mirrors right half
- This is a structural constraint on the number's form

---

## Probability Model

### Heuristic Approach

The probability that M is prime can be estimated as:
```
Pr[M is prime] ≈ (1/ln M) × S_membrane(b, lock, k)
```

where:
- 1/ln M is the Prime Number Theorem baseline
- S_membrane is the singular series accounting for constraints

### Decomposing S_membrane

We need to account for:

1. **Base divisibility**: Numbers of form M in base b have specific divisibility properties
2. **Phase lock structure**: The boundaries impose residue class constraints
3. **Symmetric structure**: Mirror property creates additional constraints
4. **Coprimality**: Boundaries coprime to base

---

## Derivation Attempt 1: Residue Class Approach

### Step 1: Count Valid Residue Classes

For a number M constructed as membrane with seed S:
- M mod p depends on (outer, inner, S, k₁, k₂) and p
- For M to be prime, M must be coprime to all small primes p

**For each prime p**, what fraction of seeds S give M ≢ 0 (mod p)?

If b ≡ r (mod p), then:
```
M ≡ outer·r^k₁ + inner·r^k₂ + S·r^k₃ + inner·r^k₄ + outer·r^k₅ (mod p)
```

The symmetric structure means:
```
M ≡ outer·(r^k₁ + r^k₅) + inner·(r^k₂ + r^k₄) + S·r^k₃ (mod p)
```

### Step 2: Probability for Prime p

For fixed (outer, inner, k₁, k₂, k₃), as S varies from 1 to p:
- M takes on at most p distinct values mod p
- Exactly one value gives M ≡ 0 (mod p) (assuming S varies freely)

So:
```
Pr[M coprime to p | config] ≈ (p-1)/p
```

**BUT**: This ignores that outer and inner are NOT random - they're phase locks!

### Step 3: Phase Lock Constraint

**Key realization**: outer and inner are primes (or 1) satisfying outer + inner ≡ b (mod ?).

For 2p bases:
- b = 2p (p prime)
- Phase lock: (outer, inner) with outer + inner = 2p
- Both coprime to 2p (automatic)

**Modified probability**:

For prime q ≠ p (where p is the midpoint of base 2p):
```
M ≡ outer + outer + inner + inner + S + ... (mod q)  [if b ≡ 1 (mod q)]
  ≡ 2(outer + inner) + S (mod q)  [simplified]
  ≡ 2·2p + S (mod q)
  ≡ 4p + S (mod q)
```

Wait, this depends on the specific k values. Let me reconsider.

---

## Derivation Attempt 2: Density of Phase Locks

### Observation from Data

We observed:
```
success ≈ 50 × (locks / (base/4))
```

Rearranging:
```
success ≈ (50/4) × locks / (base/4)
        = 12.5 × locks / (base/4)
        = 200 × locks / base
```

Hmm, that doesn't simplify nicely. Let me try differently.

### Alternative: Direct Density Interpretation

Phase lock density = locks / (base/4)

**Interpretation**: For base b, we have b/4 "natural slots" for phase lock pairs.

**Analogy to gaps**: In gap theory, density of primes p,q with q-p=g is related to 1/g.

Here, "density of membrane primes" might relate to:
- How many phase lock choices exist (locks)
- Normalized by base size (base/4 as natural scale)

### Empirical Formula

```
S_membrane(base, density) ≈ k₀ × density
where k₀ ≈ 50
```

**What is k₀?**

k₀ represents the "base success probability" when density = 1.

If density = 1 (one phase lock per quarter-base), we'd expect ~50% success?

Actually, base 6 has density 0.667 and 33% success, so:
```
33 ≈ k₀ × 0.667
k₀ ≈ 49.5 ≈ 50 ✓
```

This suggests k₀ = 50 is the "fully dense" success rate.

---

## Derivation Attempt 3: Connection to Twin Primes

### Twin Prime Analogy

Twin prime constant C₂ ≈ 0.660 appears in:
```
Density of twin primes ≈ C₂ ∫ dx / ln²x
```

For membranes, we might have:
```
Density of membrane primes ≈ C_membrane × (phase_lock_density) × ∫ dx / ln x
```

### Adaptation

If phase_lock_density represents the "structural availability" (like C₂ for twins), then:
```
Success rate ≈ (constant) × phase_lock_density
```

The constant would depend on:
- Base size (larger bases dilute probability)
- Seed range (affects ln x term)
- Symmetric structure (additional constraint)

**Empirical constant ≈ 50** for our tested range.

---

## Theoretical Framework (Synthesis)

### Singular Series Components

For a membrane configuration in base b with phase lock (left, right):

```
S_membrane(b, lock) = S_base(b) × S_lock(lock) × S_symmetry(k₁, k₂)
```

where:

**S_base(b)**: Base-dependent factor
```
S_base(b) = ∏_{p|b, p>2} f_base(p)
```
Accounts for divisibility by prime factors of b.

For 2p bases: b = 2p, so:
```
S_base(2p) = f_base(2) × f_base(p)
```

**S_lock(lock)**: Phase lock factor
```
S_lock(left, right) = g(left, right, b)
```
Accounts for the specific phase lock choice.

Related to coprimality: both left and right coprime to b enhances probability.

**S_symmetry(k)**: Symmetric structure factor
```
S_symmetry(k₁, k₂) = h(k₁, k₂)
```
Accounts for additional constraints from mirror structure.

Empirically: k₁=k₂=0 (minimal padding) is optimal, so:
```
S_symmetry(0, 0) = 1 (maximal)
S_symmetry(k₁, k₂) < 1 for k₁, k₂ > 0
```

### Density Connection

Phase lock density = locks / (b/4) represents the **structural richness** of base b.

More phase locks → more choices → higher aggregate success (even if per-configuration success varies).

**Model**:
```
Overall success ≈ average(S_membrane over all locks) × (number of locks)
                ≈ k₀ × (locks / (b/4))
                = k₀ × density
```

where k₀ ≈ 50 is the normalization constant.

---

## Refined Model with Correction

### Systematic Bias Observed

Predictions run 1-4 points high. Possible corrections:

**1. Distance factor**:
```
success = k₀ × density × (1 - α × d_norm)
```
where d_norm = distance / (b/2) is normalized distance, α ≈ 0.1.

Farther phase locks from midpoint reduce success.

**2. Base size factor**:
```
success = k₀ × density × (b₀/b)^β
```
where b₀ = 6 (reference base), β ≈ 0.1-0.2.

Larger bases have slightly lower success due to increased number length.

**3. Sample size factor**:
```
observed = predicted + ε
where ε ~ N(0, σ²)
σ² = predicted × (1 - predicted) / n
```

With n=100 samples, statistical noise is ~3-5 percentage points.

### Testing Corrections

To test these, we'd need:
1. More bases (especially larger 2p bases like 34, 38, 46)
2. Larger sample sizes (200-500 seeds per base)
3. Multiple locks per base tested separately

---

## Connection to Hardy-Littlewood Normalization

### Why Orthogonality Matters

In `membrane_orthogonality.rs`, we found:
```
r(raw) = 0.726  (spectral regularity vs success)
r(HL-normalized) = -0.619  (not orthogonal yet)
```

**Reason for non-orthogonality**: We don't have the membrane singular series yet!

Once we derive S_membrane(b, lock) properly:
```
success_normalized = success_observed / S_membrane(b, lock)
```

Then:
```
r(spectral_regularity, success_normalized) ≈ 0
```

This would confirm that spectral regularity and membrane-specific constraints are orthogonal factors.

### What We Need

Explicit formula for S_membrane(b, left, right, k₁, k₂) in terms of:
- Prime factorization of b
- Coprimality of left, right to b
- Phase lock distance
- Padding parameters k₁, k₂

Then we can compute expected success and normalize observations.

---

## Agda Formalization Path

### Step 1: Define Singular Series Type

```agda
-- Singular series for membrane configuration
S-membrane : (base : ℕ) → PhaseLock base → (k₁ k₂ : ℕ) → ℚ
S-membrane base lock k₁ k₂ = S-base base × S-lock lock × S-symmetry k₁ k₂
```

### Step 2: Derive Components

```agda
-- Base factor (product over prime divisors)
S-base : (base : ℕ) → ℚ
S-base base = ∏ (λ p → f-base p) (prime-factors base)

-- Lock factor (coprimality and distance)
S-lock : {base : ℕ} → PhaseLock base → ℚ
S-lock lock = g (PhaseLock.left lock) (PhaseLock.right lock) (PhaseLock.distance lock)

-- Symmetry factor (padding cost)
S-symmetry : (k₁ k₂ : ℕ) → ℚ
S-symmetry zero zero = 1
S-symmetry k₁ k₂ = (1 - α)^(k₁ + k₂)  where α = 0.05 -- empirical
```

### Step 3: Connect to Empirical

```agda
-- Predicted success rate
predicted-success : (base : ℕ) → PhaseLock base → (k₁ k₂ : ℕ) → ℚ
predicted-success base lock k₁ k₂ = k₀ × S-membrane base lock k₁ k₂

-- Empirical validation
validate-prediction : (base : ℕ) → PhaseLock base → ℚ → Set
validate-prediction base lock observed =
  let predicted = predicted-success base lock 0 0
      error = |observed - predicted|
  in error < tolerance
```

### Step 4: Prove Orthogonality

```agda
-- After normalization, spectral regularity and success are orthogonal
orthogonality-after-normalization :
  ∀ (bases : List ℕ) →
  let spectral = map spectral-regularity bases
      success-raw = map observed-success bases
      success-norm = map (λ b → observed-success b / S-membrane b ...) bases
  in correlation spectral success-norm ≈ 0
```

---

## Empirical Validation Plan

### Phase 1: Test Distance Correction

For base 14 (two phase locks at distances 4 and 6):
- Test lock (3,11) at d=4: predict ~28%
- Test lock (1,13) at d=6: predict ~24% (if distance penalty applies)

If second lock shows lower success, validates distance factor.

### Phase 2: Test Padding Effect

For base 6, test multiple k configurations:
- (1,5) k=(0,0): observed 33%
- (1,5) k=(1,1): predict ~28% (if padding penalty applies)
- (1,5) k=(2,2): predict ~24%

If success decreases with padding, validates S_symmetry factor.

### Phase 3: Large Sample Validation

Re-test bases 6, 10, 14 with n=500 seeds:
- Reduce statistical noise (σ ~ 2-3 points instead of 4-5)
- Get more precise observed values
- Check if systematic bias persists

### Phase 4: Extended Base Range

Test 2p bases 34, 38, 46:
- Larger bases with multiple phase locks
- Check if density model holds at larger scales
- Validate base-size correction if needed

---

## Open Questions

### Q1: What is the theoretical value of k₀?

We observe k₀ ≈ 50 empirically. Can we derive this from:
- PNT (1/ln n factor)
- C₂ (twin prime constant)
- Base size (2p form)
- Symmetric structure (mirror constraint)

### Q2: Why do phase locks at even distances matter?

All 2p bases have GCD(distances) = 2 (even distances).

**Hypothesis**: Even distance creates parity structure that enhances primality.

**Test**: If we find a base with odd-distance phase locks, does it underperform?

### Q3: Connection to restricted Goldbach?

If we prove restricted-goldbach-2p, does that immediately give us:
```
∀ (base : 2p), ∃ (lock : PhaseLock), S-membrane(base, lock) > 0
```

In other words, does existence of phase lock guarantee non-zero membrane success?

### Q4: Scaling with seed length?

We observed nested structure emerges at seed length 4.

**Question**: Does S_membrane depend on seed length?

**Hypothesis**:
```
S_membrane(base, lock, k, seed_length) = S_base × S_lock × S_symmetry × S_scale(seed_length)

where S_scale(L) = 1 / ln(10^L) ≈ 1 / (L × ln 10)
```

This would explain success decline as seeds grow, and predict when nesting helps.

---

## Summary

### What We Know

1. **Empirical formula validated**: success ≈ 50 × density (r = 0.996)
2. **Phase locks are fundamental**: They determine density
3. **Systematic bias exists**: Predictions run 1-4 points high
4. **Minimal padding optimal**: k=(0,0) outperforms k>0

### What We Need to Derive

1. **S_membrane formula**: Explicit Hardy-Littlewood singular series for membranes
2. **Component factors**: S_base, S_lock, S_symmetry with theoretical values
3. **Correction terms**: Distance, base size, seed length effects
4. **Orthogonality proof**: Show normalized success independent of spectral regularity

### Path Forward

1. **Empirical**: Test distance, padding, sample size effects
2. **Theoretical**: Derive S_membrane from residue class analysis
3. **Formal**: Implement in Agda and prove orthogonality
4. **Validation**: Re-test all bases with refined predictions

---

## Conclusion

The membrane singular series S_membrane(base, lock, k₁, k₂) is the missing theoretical link between:
- Phase lock structure (fundamental)
- Membrane configuration (design)
- Success rate (observation)

Deriving this formula would:
- Explain the r = 0.996 correlation
- Predict exact success rates a priori
- Guide optimization (which locks, padding, base choices)
- Connect to classical number theory (HL conjectures, Goldbach, twin primes)

**Current status**: Strong empirical validation, theoretical framework sketched, derivation in progress.

**Next step**: Test correction factors (distance, padding, sample size) to refine the model before full theoretical derivation.

---

**References**:
- Hardy-Littlewood conjectures on prime gaps
- Twin prime constant C₂ ≈ 0.660
- Phase lock discoveries (PHASE_LOCK_DISCOVERIES.md)
- Orthogonality framework (membrane_orthogonality.rs)
- Validation session results (PHASE_LOCK_VALIDATION_SESSION.md)
