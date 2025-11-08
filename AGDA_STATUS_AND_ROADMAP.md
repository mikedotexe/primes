# Agda Formalization: Status and Roadmap

**Last Updated**: November 2025
**Agda Version Required**: 2.6.3+
**Standard Library**: 2.0+

---

## Current Status ✅

### Existing Formalization Work

#### 1. **`agda/PrimeConcepts.agda`** - Core Framework (394 lines)

**Status**: ✅ Comprehensive structure defined, many postulates awaiting proofs

**Contents**:
- ✅ Basic number theory foundations (GCD, coprimality, primality predicate)
- ✅ Membrane structure formalization (MembraneConfig, boundary digits, padding)
- ✅ GCD constraint paradox framework
- ✅ Resonance theory (PrimeBody, concatenation, yield oscillation)
- ✅ Perturbation and stability theory
- ✅ Hardy-Littlewood coverage theory (C₂, singular series, Goldbach)
- ✅ Unified framework (PrimeMembranePhysics)
- ✅ Verification standards (VerifiedClaim, FalsifiableClaim, StatisticalClaim)

**Key Postulates to Prove**:
```agda
- coprimality-essential : Coprime configs outperform non-coprime
- minimal-padding-optimal : k=(0,0) always best
- base6-optimal : No base ≤30 beats 33%
- fragility-theorem : >90% primes have stability <0.1
```

#### 2. **`agda/EmpiricalEvidence.agda`** - Data Encoding (390 lines)

**Status**: ✅ Real experimental data formalized

**Contents**:
- ✅ Resonance data for bodies (7,11) - 27 space sizes
- ✅ Perturbation fragility data - 100% failure rate
- ✅ GCD paradox empirical data - 10 bases tested
- ✅ Optimal configurations (base 6: 33%, base 30: 30%)
- ✅ Coprimality requirement data (100% correlation)
- ✅ Minimal padding dominance data
- ✅ Verification metadata (286,200 primality checks)

**Key Verified Claims**:
```agda
- oscillation-verified : Resonance peaks at sizes 3, 11, 21
- perturbation-fragility-verified : stability-score = 0/1
- gcd-paradox-trend-verified : gcd=3 > gcd=1 success rate
- all-top-configs-coprime : 100% of top configs coprime
```

#### 3. **`docs/agda/BabylonianPrimeDivergence.agda`** - Orthogonality Framework (352 lines)

**Status**: ✅ Philosophical framework, SKETCH status

**Contents**:
- ✅ Babylonian score definitions (base-60, divisibility)
- ✅ Hardy-Littlewood singular series formalization
- ✅ Orthogonality thesis (raw vs normalized correlation)
- ✅ Coprimality and membrane connections
- ✅ Meta-theorems on mathematical duality

**Key Theorems to Prove**:
```agda
- normalized-correlation-zero : After HL normalization, corr ≈ 0
- membrane-success-coprime-biased : Membranes align with nature, not convenience
- coprime-necessary-for-prime : gcd(n, rad(b)) = 1 required
```

---

## Gaps Identified from FORMAL_VERIFICATION_ASSESSMENT.md

### Priority 1: Affine Transform Theorem ⭐⭐⭐⭐⭐

**File**: Not yet created
**Target**: `agda-proofs/Theorems/AffineTransform.agda`

**Claim to Prove**:
```agda
affine-transform-correct :
  ∀ (base : ℕ) (conf : Config) (seed : ℕ) (p : ℕ)
  → Prime p
  → (membrane base conf seed mod p)
    ≡ ((membrane base conf 0 mod p) + seed * (base ^ (width conf / 2) mod p)) mod p
```

**Status**: ❌ Not started
**Effort**: 2-3 months
**Impact**: Very High - enables O(1) computation instead of O(n) polynomial evaluation

---

### Priority 2: Radical Properties ⭐⭐⭐⭐

**File**: Not yet created
**Target**: `agda-proofs/Core/Radical.agda`

**Claims to Prove**:
```agda
-- Define radical
radical : ℕ → ℕ
radical n = product (distinct-prime-factors n)

-- Properties
radical-idempotent : ∀ n → radical (radical n) ≡ radical n
radical-multiplicative : ∀ a b → coprime a b → radical (a * b) ≡ radical a * radical b
radical-not-totient : ∃[ n ] radical n ≢ totient n  -- e.g., 12

-- Connection to primality
prime-requires-coprime-to-radical :
  ∀ (n b : ℕ) → prime n → gcd n (radical b) ≡ 1
```

**Status**: ❌ Not started
**Effort**: 1-2 weeks (straightforward number theory)
**Impact**: High - clarifies confusing rad vs φ distinction

---

### Priority 3: Coprimality Necessity ⭐⭐⭐⭐

**File**: Partially in PrimeConcepts.agda
**Target**: `agda-proofs/Theorems/Coprimality.agda`

**Claim to Prove**:
```agda
coprimality-necessary-for-optimal :
  ∀ (b : ℕ) (outer inner : ℕ)
  → is-optimal-config b outer inner
  → gcd outer b ≡ 1 ∧ gcd inner b ≡ 1

-- Or prove contrapositive
non-coprime-suboptimal :
  ∀ (b outer inner : ℕ)
  → (gcd outer b > 1 ∨ gcd inner b > 1)
  → success-rate b outer inner < threshold
```

**Status**: ⚠️ Postulated in PrimeConcepts.agda, not proven
**Effort**: 2-3 months (requires deep understanding of membrane filtering)
**Impact**: Very High - explains the "100% of top configs" observation

---

### Priority 4: Trio Universality ⭐⭐⭐

**File**: Not yet created
**Target**: `agda-proofs/Theorems/TrioUniversality.agda`

**Claim to Prove**:
```agda
-- For N=3 and coprime B
trio-universal :
  ∀ (B : ℕ) → gcd B 3 ≡ 1
  → ∀ (r : ℕ)
  → permutation (residues-covered B 3 r) [0, 1, 2]

-- Generalize to any N
n-universal :
  ∀ (B N : ℕ) → gcd B N ≡ 1
  → ∀ (r : ℕ)
  → permutation (residues-covered B N r) (range N)
```

**Status**: ❌ Not started
**Effort**: 1-2 weeks (modular arithmetic)
**Impact**: Medium-High - fundamental to N× transform theory

---

### Priority 5: Exclusive Configuration Proofs ⭐⭐⭐⭐

**File**: Not yet created
**Target**: `agda-proofs/Theorems/ExclusiveConfigs.agda`

**Claims to Prove**:
```agda
-- Specific case: (3,3) k=(1,1) base 10
unique-seed-is-5 :
  ∀ (seed : ℕ) → seed < 10
  → prime (membrane 10 exclusive-config-3-3-1-1 seed)
  → seed ≡ 5

-- Prove seed 5 works
seed-5-is-prime :
  prime (membrane 10 exclusive-config-3-3-1-1 5)

-- Prove others fail (constructively via divisibility rules)
seed-0-composite : composite (membrane 10 exclusive-config-3-3-1-1 0)
-- ... for seeds 1,2,3,4,6,7,8,9
```

**Status**: ❌ Not started
**Effort**: 2-3 weeks (tedious but mechanical)
**Impact**: High - proves deterministic generation

---

## Additional Gaps from Examples Review

### 6. GCD Collapse Mechanism ⭐⭐⭐⭐

**File**: Partially in PrimeConcepts.agda
**Target**: `agda-proofs/Theorems/GCDCollapse.agda`

**From**: `gcd_paradox_resolver.rs`

**Claims**:
```agda
-- Residue collapse theorem
gcd-collapse-constrains :
  ∀ (base n : ℕ) (g : ℕ)
  → g ≡ gcd base n → g > 1
  → |residues-available base n g| < n

-- Connect to membrane success
collapsed-improves-filtering :
  ∀ (b : Base) (conf : MembraneConfig b)
  → gcd (Base.value b) 3 > 1
  → SuccessRate b conf seeds > threshold
```

**Status**: ⚠️ Framework in PrimeConcepts, mechanism not proven
**Effort**: 3-4 months (complex combinatorial argument)
**Impact**: Very High - explains counter-intuitive result

---

### 7. Lagrange Point Properties ⭐⭐⭐

**File**: Partially in PrimeConcepts.agda (concatenated primes)
**Target**: `agda-proofs/Theorems/LagrangePoints.agda`

**From**: `lagrange_full_verification.rs`

**Claims**:
```agda
-- Lagrange position creates prime
is-lagrange-position :
  ∀ (p1 p2 : ℕ) (position digit : ℕ)
  → prime p1 → prime p2
  → prime (concatenate-with-digit-at p1 p2 position digit)
  → is-equilibrium-position position

-- Specific verified case
lagrange-position-4-works :
  is-lagrange-position 10301 3007003007003 4 6 ≡ true
```

**Status**: ⚠️ Concatenation framework exists, equilibrium not formalized
**Effort**: 1-2 months
**Impact**: Medium - interesting but less foundational

---

## Agda Installation and Setup

### Installation (Not Yet Done)

```bash
# Option 1: Via GHCup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://get-ghcup.haskell.org | sh
ghcup install ghc 9.4.7
ghcup install cabal latest
cabal update
cabal install Agda

# Option 2: Via package manager
# macOS:
brew install agda

# Ubuntu/Debian:
sudo apt-get install agda

# Arch:
sudo pacman -S agda
```

### Standard Library Setup

```bash
# Clone
git clone https://github.com/agda/agda-stdlib.git ~/.agda/agda-stdlib
cd ~/.agda/agda-stdlib
git checkout v2.0

# Configure
mkdir -p ~/.agda
echo "~/.agda/agda-stdlib/standard-library.agda-lib" > ~/.agda/libraries
echo "standard-library" > ~/.agda/defaults
```

### Verify Installation

```bash
agda --version
# Should show: Agda version 2.6.x

# Type-check existing files
cd /home/user/primes/agda
agda PrimeConcepts.agda
agda EmpiricalEvidence.agda
```

---

## Recommended Work Order

### Phase 1: Quick Wins (Weeks 1-2) ✅

1. ✅ **Radical properties** - `agda-proofs/Core/Radical.agda`
   - Define radical function
   - Prove idempotence, multiplicativity
   - Show radical ≠ totient for n=12

2. ✅ **Trio universality** - `agda-proofs/Theorems/TrioUniversality.agda`
   - Prove for N=3, B coprime to 3
   - Generalize to arbitrary N

### Phase 2: High-Impact Theorems (Months 1-2) 🎯

3. 🎯 **Affine transform** - `agda-proofs/Theorems/AffineTransform.agda`
   - Highest priority, most impact
   - Polynomial expansion lemmas
   - Modular arithmetic properties
   - Main theorem proof

4. 🎯 **Coprimality necessity** - `agda-proofs/Theorems/Coprimality.agda`
   - Explain 100% correlation
   - Connect to radical
   - Filtering mechanism

### Phase 3: Mechanisms (Months 2-4) 📐

5. 📐 **Exclusive configurations** - `agda-proofs/Theorems/ExclusiveConfigs.agda`
   - Prove (3,3) k=(1,1) uniqueness
   - Divisibility rule proofs
   - Generalize pattern

6. 📐 **GCD collapse** - `agda-proofs/Theorems/GCDCollapse.agda`
   - Residue constraint proof
   - Filtering mechanism
   - Success rate connection

### Phase 4: Advanced (Months 4+) 🚀

7. 🚀 **Lagrange points** - `agda-proofs/Theorems/LagrangePoints.agda`
8. 🚀 **Goldbach construction** - (from goldbach_ntransform_explorer.rs)
9. 🚀 **Resonance characterization** - Closed-form peak positions

---

## Current Project Structure

```
/home/user/primes/
├── agda/                           # Main formalization ✅
│   ├── PrimeConcepts.agda         # Core framework (394 lines) ✅
│   ├── EmpiricalEvidence.agda     # Real data (390 lines) ✅
│   └── README.md                  # Documentation ✅
│
├── agda-proofs/                    # Theorem proofs (empty) ⚠️
│   ├── Core/                      # Empty - needs Radical.agda
│   ├── Theorems/                  # Empty - needs all proofs
│   ├── Empirical/                 # Empty
│   ├── Utils/                     # Empty
│   └── README.md                  # Created ✅
│
├── docs/agda/                      # Additional frameworks
│   └── BabylonianPrimeDivergence.agda  # Orthogonality (352 lines) ✅
│
└── FORMAL_VERIFICATION_ASSESSMENT.md    # Assessment & roadmap ✅
```

---

## Next Steps

### Immediate (This Session)

1. ✅ Document current status (this file)
2. 🔄 Create `agda-proofs/Core/Radical.agda` with basic structure
3. 🔄 Create `agda-proofs/Theorems/AffineTransform.agda` scaffolding
4. 🔄 Add examples and test cases

### Short-term (Next Sessions)

1. Implement radical property proofs
2. Implement trio universality proof
3. Begin affine transform proof (most complex)

### Medium-term (Weeks-Months)

1. Complete affine transform
2. Prove coprimality necessity
3. Exclusive configuration proofs
4. GCD collapse mechanism

---

## Integration with Rust Code

| Agda Module | Rust Implementation | Status |
|-------------|---------------------|--------|
| `PrimeConcepts.agda` | Core membrane logic | ✅ Formalized |
| `EmpiricalEvidence.agda` | `resonance_analyzer.rs` | ✅ Data encoded |
| `EmpiricalEvidence.agda` | `gcd_paradox_resolver.rs` | ✅ Data encoded |
| `BabylonianPrimeDivergence.agda` | (theoretical) | ✅ Sketched |
| `Radical.agda` (planned) | `src/hzlib/density.rs` | ❌ Not started |
| `AffineTransform.agda` (planned) | `affine_transform_verifier.rs` | ❌ Not started |
| `Coprimality.agda` (planned) | Throughout codebase | ❌ Not started |
| `TrioUniversality.agda` (planned) | N× transform code | ❌ Not started |
| `ExclusiveConfigs.agda` (planned) | `proper_membrane_generator.rs` | ❌ Not started |

---

## Success Metrics

### Phase 1 Complete When:
- ✅ Radical properties proven
- ✅ Trio universality proven
- ✅ Examples type-check in Agda

### Phase 2 Complete When:
- ✅ Affine transform theorem proven
- ✅ Coprimality necessity proven
- ✅ All proofs type-check

### Phase 3 Complete When:
- ✅ Exclusive configs proven
- ✅ GCD collapse proven
- ✅ Paper-ready formalization

### Ultimate Goal:
- ✅ All postulates in PrimeConcepts.agda replaced with proofs
- ✅ EmpiricalSoundness theorem fully proven
- ✅ Publication-ready formal verification

---

**Status Summary**:
- ✅ **Framework**: Comprehensive (1,136 lines of Agda)
- ⚠️ **Proofs**: Many postulates, few completed proofs
- ❌ **Installation**: Agda not yet installed
- 🎯 **Ready**: Structure in place, ready for proof work

**Recommendation**: Install Agda, then start with Radical.agda (easiest) to build confidence before tackling AffineTransform.agda (hardest).
