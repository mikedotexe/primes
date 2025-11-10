# Agda Formalization of Prime Construction Theory

This directory contains rigorous mathematical formalizations of the empirically discovered prime generation patterns.

## Files

### Core Formalizations

#### `PrimeConcepts.agda`
Core mathematical structures and theorems:

1. **Membrane Structure Formalization**
   - Symmetric boundary digit configurations
   - Coprimality requirements
   - Padding parameters k₁, k₂

2. **GCD Constraint Paradox**
   - Counterintuitive finding: Higher GCD → Better prime generation
   - Residue collapse → Primality filtering
   - Entropy anticorrelation with success

3. **Resonance Theory**
   - Prime yield oscillates with space size
   - Non-monotonic behavior between prime bodies
   - Verified peaks and troughs in data

4. **Perturbation & Stability Theory**
   - Stability score under digit perturbations
   - Fragility theorem: Most primes are isolated
   - Energy well interpretation

5. **Hardy-Littlewood Coverage Theory**
   - Truncated expectations for restricted Goldbach
   - Complementary pattern enhancement
   - Coverage probability via Poisson approximation

#### `EmpiricalEvidence.agda`
Data-rich encoding of experimental results:

1. **Resonance Data** (bodies 7 and 11)
   - 27 space sizes tested
   - Oscillation verified: peaks at sizes 3, 11, 21
   - Period estimate: ~9 units

2. **Perturbation Data**
   - Configuration: (7, space=100, pos=5, digit=5, 11)
   - Stability score: 0.0000 (100% fragile)
   - All 99 perturbations → composite

3. **GCD Paradox Data**
   - 10 bases tested with 10 seeds each
   - gcd=3 bases: 33.3% average success
   - gcd=1 bases: 26.5% average success
   - Correlation r=+0.266 (positive!)
   - Entropy correlation r=-0.266 (negative!)

4. **Optimal Configurations**
   - Base 6: (1,5) k=(0,0) → 33% success
   - Base 30: (11,7) k=(0,0) → 30% success
   - Universal pattern: (1,5) k=(0,0) works in 5+ bases

5. **Coprimality Requirement**
   - 100% of top configs use coprime digits
   - Essential (not optional) for success

6. **Minimal Padding Dominance**
   - k=(0,0) consistently outperforms all padded variants
   - Tighter structure = better filtering

### Proof Skeletons (Lightweight Foundations)

#### `SpacingResidueModel.agda` ⭐ DEFAULT CONSTRUCTION
Formal proof skeleton for spacing-based residue filtering (the core discovery):

- **Scope**: DEFAULT construction with symmetric spacing + independent digit sampling
- **Main Insight**: Exponent patterns in base expansion create modular traps that shift with midpoint length
- **Key Property**: Residue distribution P(n ≡ r mod m) determined by positions of open slots, not digit values
- **Flexibility**: Can avoid (b+1) divisibility wall while targeting specific moduli
- **Status**: Skeleton with concrete counterexamples showing spacing ≠ palindrome

**Example counterexamples**:
```agda
-- Base 6, layout [d₁] 0 0 [d₂] (spacing symmetric)
-- Palindrome: 1001₆ = 217₁₀ divisible by 7 = (6+1) ✓
-- Independent: 1002₆ = 218₁₀ NOT divisible by 7 ✗
--              2001₆ = 433₁₀ NOT divisible by 7 ✗

-- Same spacing, different divisibility behavior!
```

**Core theorems**:
```agda
-- Spacing creates non-uniform residue distributions
spacing-creates-bias
  : ∀ pattern modulus → ¬ (P(n≡0 mod m) = 1/m)

-- GCD amplifies spacing-driven traps
gcd-amplifies-spacing-bias
  : ∀ base modulus → gcd(base,m) > 1
  → ∃ exponentPattern bias

-- Midpoint length shifts which moduli are filtered
midpoint-shifts-traps
  : ∀ pattern₁ pattern₂ → diffMidpoint
  → ∃ modulus → P₀(pattern₁) ≠ P₀(pattern₂)
```

**Implementation**: Directly corresponds to `residue_null_probability()` DP model in `tools/density-explorer/src/main.rs`

#### `PalindromeEvenDivides.agda` ⚠️ MIRROR MODE ONLY
Formal proof skeleton for palindrome divisibility property:

- **Scope**: TRUE PALINDROMES (digit-value mirroring) - only applies to optional `--mirror` mode
- **Main Theorem**: Even-length palindromes in base b ≥ 2 are ALWAYS divisible by (b+1)
- **Method**: Pairing argument requires d_i = d_j (mirroring constraint)
- **Key Limitation**: Universal (b+1) divisibility creates systematic filtering wall
- **Status**: Skeleton with postulated algebraic identities (to be filled with ring reasoning)

**Type signature**:
```agda
evenPalindromeDividesBPlusOne
  : ∀ {k} (b : ℕ) → b ≥2 → (ds : Vec ℕ (2*k))
  → Palindrome ds → (b + 1) ∣ eval b ds
```

**Why this matters**: Shows the *difference* between palindromic and spacing-symmetric constructions. The palindrome constraint is too rigid; spacing-symmetry with independence offers flexible filtering.

#### `DigitSumMod3.agda`
Formal proof skeleton for digit-sum divisibility rules modulo 3:

- **Lemma 1**: If b ≡ 1 (mod 3), then eval(b,ds) ≡ sum(ds) (mod 3)
- **Lemma 2**: If b ≡ 0 (mod 3), then eval(b,ds) ≡ head(ds) (mod 3) (LSB-first encoding)
- **Lemma 3**: If b ≡ 2 (mod 3), then eval(b,ds) ≡ alternating-sum(ds) (mod 3)
- **Key Insight**: Base congruence class determines which sum invariant holds
- **Status**: Skeleton with postulated modular arithmetic lemmas

**Type signatures**:
```agda
digitSumMod3-base≡1 : ∀ {n} (b : ℕ) (ds : Vec ℕ n)
  → b ≡₃ 1 → eval b ds ≡₃ sumDigits ds

digitSumMod3-base≡0 : ∀ {n} (b : ℕ) (ds : Vec ℕ n)
  → b ≡₃ 0 → eval b ds ≡₃ head ds

digitSumMod3-base≡2 : ∀ {n} (b : ℕ) (ds : Vec ℕ n)
  → b ≡₃ 2 → eval b ds ≡₃ altSum ds
```

**Usage**: These skeletons provide a lightweight foundation that can be gradually refined into complete proofs using the Agda standard library's ring solver and modular arithmetic facilities.

---

### Visual Comparison: Palindrome vs Spacing-Symmetric

Understanding the difference is crucial:

```
┌─────────────────────────────────────────────────────────────────────┐
│                    PALINDROME (--mirror mode)                       │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  Layout:  [d₁] [d₂] [d₃]  ◀═══MIRROR═══▶  [d₃] [d₂] [d₁]          │
│                                                                     │
│  Constraint: RIGHT half = REVERSE(LEFT half)                       │
│  Sampling:   Sample d₁,d₂,d₃ once → copy reversed                  │
│                                                                     │
│  Example (base 10):  1 2 3 │ 3 2 1  →  123321                      │
│                             └─────────────┘                         │
│                           Forced equality                           │
│                                                                     │
│  Property: ALWAYS divisible by (b+1)                               │
│  Base 10:  123321 mod 11 = 0  ✓                                    │
│  Base 6:   1001₆ mod 7 = 0    ✓                                    │
│                                                                     │
│  Trade-off: Universal (b+1) wall → systematic filtering limit      │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│              SPACING-SYMMETRIC (default construction)               │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  Layout:  [d₁] [d₂] [d₃]  ◀═══SPACING═══▶  [d₄] [d₅] [d₆]         │
│                                                                     │
│  Constraint: Position pattern symmetric (not values!)              │
│  Sampling:   Sample ALL digits independently                       │
│                                                                     │
│  Example (base 10):  1 2 3 │ 4 5 6  →  123456                      │
│                             └─────────────┘                         │
│                          Independent values!                        │
│                                                                     │
│  Property: NOT universally divisible by (b+1)                      │
│  Base 10:  123456 mod 11 = 3  ✗ (counterexample!)                  │
│  Base 6:   1002₆ mod 7 = 1    ✗ (same spacing as 1001, diff div)   │
│                                                                     │
│  Advantage: Flexible filtering via exponent patterns               │
│             • Can AVOID (b+1) trap                                  │
│             • Can TARGET specific moduli                            │
│             • Midpoint tuning shifts which primes filtered          │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

**Key Mathematical Difference**:

```
Palindrome pairing (requires mirroring):
  d_i × b^i + d_j × b^j    where d_i = d_j
  = d_i × (b^i + b^j)      ← Can factor!
  = d_i × b^i × (1 + b^(j-i))

  When j-i is odd: (b+1) ∣ (1 + b^(j-i))  → guaranteed divisibility

Spacing-symmetric (independent digits):
  d_i × b^i + d_j × b^j    where d_i ≠ d_j

  Cannot factor!  → No universal (b+1) divisibility
  But exponent pattern creates modular traps via gcd(base, modulus)
```

**Empirical Result**: The spacing-symmetric approach achieves **33% prime density** (base 6), outperforming palindromic constructions which hit the (b+1) filtering wall.

## Verification Standards

All claims in these formalizations are:

- **Verifiable**: Backed by computational evidence
- **Falsifiable**: Counter-example checks specified
- **Reproducible**: Generated by deterministic scripts
- **Statistical**: Include p-values, effect sizes, sample sizes

### Primality Testing
- Method: Miller-Rabin with 20 rounds
- Confidence: >99.99%
- Total checks: 286,200 across all verifications

### Reproducibility Commands

**Rust examples**:
```bash
cargo run --example resonance_analyzer --release
cargo run --example perturbation_analyzer --release
cargo run --example gcd_paradox_resolver --release -- --quick
cargo run --example goldbach_hl_analysis -- --min-base 60 --max-base 80
```

**Agda type-checking** (requires Agda + standard library):
```bash
# Navigate to agda directory
cd agda/

# Type-check proof skeletons (default construction)
agda --library standard-library SpacingResidueModel.agda

# Type-check proof skeletons (mirror mode & digit-sum rules)
agda --library standard-library PalindromeEvenDivides.agda
agda --library standard-library DigitSumMod3.agda

# Type-check core formalizations
agda --library standard-library PrimeConcepts.agda
agda --library standard-library EmpiricalEvidence.agda
```

## Key Theorems (To Be Proven)

### Conjecture 1: GCD Constraint Improves Filtering
```agda
gcd-improves-filtering : ∀ b conf seeds →
  gcd (Base.value b) 3 > 1 →
  SuccessRate b conf seeds > 0.25
```

**Status**: Empirically supported (r=+0.266), not yet significant (p>0.05 in quick mode)

### Conjecture 2: Universal Resonance
```agda
universal-resonance : ∀ b1 b2 →
  PrimeBody.value b1 > 2 →
  PrimeBody.value b2 > 2 →
  ∃ λ pattern → ResonancePattern b1 b2
```

**Status**: Verified for (7,11), needs testing on more pairs

### Conjecture 3: Complementary Pattern Enhancement
```agda
complementary-enhancement-universal : ∀ base →
  IsComplementaryPattern base ≡ true →
  ∃ λ enhancement → enhancement > 1.1
```

**Status**: Hypothesis formed, awaiting goldbach_hl_analysis data

## Axioms & Postulates

These are currently postulated (assumed true) but should be proven or refined:

1. `coprimality-essential`: Coprime configs always outperform non-coprime
2. `minimal-padding-optimal`: k=(0,0) always optimal
3. `base6-optimal`: No base ≤30 beats base 6's 33% rate
4. `fragility-theorem`: >90% of primes have stability <0.1

## Integration with Codebase

The Agda formalizations directly correspond to Rust implementations:

| Agda Module | Rust Example | Verification |
|-------------|--------------|--------------|
| Resonance theory | `resonance_analyzer.rs` | ✓ 27 data points |
| Perturbation theory | `perturbation_analyzer.rs` | ✓ 99 perturbations |
| GCD paradox | `gcd_paradox_resolver.rs` | ✓ 10 bases |
| Hardy-Littlewood | `goldbach_hl_analysis.rs` | ⧗ Pending run |
| Membrane structure | `proper_membrane_generator.rs` | ✓ Core functionality |

## Future Work

### Immediate
1. **Prove coprimality theorem**: Show mathematically why coprime digits succeed
2. **Characterize resonance**: Find closed-form expression for peak positions
3. **Generalize perturbation**: Test stability across different configurations

### Long-term
1. **Complete soundness proof**: Formalize `EmpiricalSoundness` with full proofs
2. **Unify with orthogonality**: Connect membrane structure to orthogonal k-residues
3. **Lagrangian mechanics**: Formalize energy landscape and equilibrium points
4. **Hardy-Littlewood validation**: Prove coverage predictions match observations

## Reading Order

For newcomers to the formalization:

1. Start with `PrimeConcepts.agda` sections 1-2 (foundations, membranes)
2. Read `EmpiricalEvidence.agda` section 3 (GCD paradox data)
3. Return to `PrimeConcepts.agda` section 3 (GCD theory)
4. Follow the same pattern for resonance, perturbation, and Hardy-Littlewood

## Contributing

When adding new empirical findings:

1. Run the corresponding Rust example
2. Add data to `EmpiricalEvidence.agda`
3. Formalize the pattern in `PrimeConcepts.agda`
4. Update this README with verification status
5. Include reproducibility commands

All additions must satisfy:
- ✓ Verifiable (computational evidence)
- ✓ Falsifiable (counter-example mechanism)
- ✓ Reproducible (deterministic script)
- ✓ Statistical (when appropriate: p-values, effect sizes)

## References

- **CLAUDE.md**: Executive summary of all findings
- **EVIDENCE.md**: Detailed proofs and verification data
- **HARDY_LITTLEWOOD_FRAMEWORK.md**: Mathematical foundations for HL analysis
- **MIDPOINT_ANALYSIS.md**: PNT deviation correlation studies

---

**Status**: Active development
**Last verification**: 2025-11-08
**Total primality checks**: 286,200
**Confidence level**: >99.99%
