-- Certified Resonance Param Dyn: Parameterized Dual Certificate
--
-- COMPLETE DUAL CERTIFICATION: Static + Dynamic in one call
--
-- This is the production interface combining:
-- 1. STATIC: Honorary Zero (midpoint void via perfect pairing)
-- 2. DYNAMIC: Inviolability (exclusion zone structurally enforced)
--
-- ONE CALL returns:
--   - S: Modular reflection at mid
--   - buckets: Perfect pairing witness
--   - voidOK: Honorary Zero certificate
--   - inviolability: Pre-applied adapter (PointwiseSafe → ¬InZone)
--
-- This bridges mechanical causality (symmetry forces void) with
-- statistical validation (RMT/spectral analysis).
--
-- Ready for week-1 validation plan and production deployment!

module Examples.CertifiedResonanceParamDyn where

open import Data.Product     using (Σ; _,_; proj₁; proj₂)
open import Relation.Binary.PropositionalEquality  using (_≡_; refl; cong; trans)
open import Data.Empty     using (⊥)
open import Data.Nat       using (ℕ ; zero ; suc ; NonZero ; _+_ ; _<_ ; z≤n ; s≤s)
open import Data.Bool      using (Bool; true; false)

open import Data.Fin               using (Fin ; zero ; suc ; toℕ)
open import Data.Fin.Properties    using () renaming (_≟_ to _≟Fin_)
open import Data.Vec               using (Vec ; [] ; _∷_)
open import Data.List              using (List) renaming ([] to []L ; _∷_ to _∷L_)
open import Function               using (_∘_)
open import Relation.Nullary       using (Dec; yes; no)

-- Import complete framework
open import Theorems.Abstract.SymmetryImpliesRepulsion
  using ( SymmetryData
        ; HonoraryZero
        )
open import Theorems.Abstract.SymmetryFromList
  using ( MS-fromResid
        ; PerfectBuckets
        ; honoraryZeroFromPerfect
        )
open import Theorems.Abstract.BucketsAutoMatch as BAM
  using ( BalancedBuckets
        ; SupportCountsAgree
        ; indices-with-residue
        ; length-lift-fin-list
        ; perfectFromBalancedWithSupport
        ; _∨_
        )
open import Theorems.Abstract.SymmetryFiniteReflect
  using ( mkSymReflect
        ; ObservedFixedPointClassifier
        ; canonicalEvenMidpoint
        ; canonicalEvenObservedFixedPointClassifier
        ; ObservedFixedPointExclusion
        ; observedResiduesMoveFromObservedSupportExclusion
        )
open import Theorems.Abstract.ConstrainedOrbitals as C
  using ( PointwiseSafe ; InZone ; inviolabilityFromPointwiseSafe )

_≢_ : ∀ {A : Set} → A → A → Set
x ≢ y = (x ≡ y) → ⊥

------------------------------------------------------------------------
-- BRIDGE FUNCTIONS: Fin ↔ Nat conversion for orbital predicates

-- Vector indexing
indexer : ∀ {A : Set} {n} → Vec A n → Fin n → A
indexer {A} {zero}     []       ()
indexer {A} {suc n}    (x ∷ xs) zero    = x
indexer {A} {suc n}    (x ∷ xs) (suc i) = indexer xs i

-- Convert Fin list to ℕ list for PointwiseSafe/InZone predicates
mapFin : ∀ {m} → C.List (Fin m) → C.List ℕ
mapFin C.[]         = C.[]
mapFin (C._∷_ x xs) = C._∷_ (toℕ x) (mapFin xs)

------------------------------------------------------------------------
-- RESIDUE COUNTING (from CertifiedResonanceParam)

-- Count occurrences of residue b in f : Fin n → Fin m
countResid : ∀ {m n} → (Fin n → Fin m) → Fin m → ℕ
countResid {m} {zero}  f b = zero
countResid {m} {suc n} f b with (f zero) ≟Fin b
... | yes _ = suc (countResid (f ∘ suc) b)
... | no  _ = countResid (f ∘ suc) b

count-positive-or-zero
  : ∀ {m n}
  → (f : Fin n → Fin m)
  → (b : Fin m)
  → (0 < countResid f b) ∨ (countResid f b ≡ zero)
count-positive-or-zero f b with countResid f b
... | zero  = false , refl
... | suc k = true , s≤s z≤n

balancedBucketsFromCounts
  : ∀ {m n}
  → (S : SymmetryData (Fin m))
  → (f : Fin n → Fin m)
  → (balanced : ∀ b → countResid f b ≡ countResid f (SymmetryData.inv S b))
  → BalancedBuckets S f (countResid f)
balancedBucketsFromCounts {n = n} S f balanced = record
  { balanced = balanced
  ; total    = n , refl
  ; positive = count-positive-or-zero f
  }

supportCountsAgreeCountResid
  : ∀ {m n}
  → (f : Fin n → Fin m)
  → SupportCountsAgree _≟Fin_ f (countResid f)
supportCountsAgreeCountResid {m} {zero}  f b = refl
supportCountsAgreeCountResid {m} {suc n} f b with (f zero) ≟Fin b
... | yes _ =
  cong suc
    (trans
       (length-lift-fin-list (indices-with-residue _≟Fin_ (f ∘ suc) b))
       (supportCountsAgreeCountResid (f ∘ suc) b))
... | no  _ =
  trans
    (length-lift-fin-list (indices-with-residue _≟Fin_ (f ∘ suc) b))
    (supportCountsAgreeCountResid (f ∘ suc) b)

autoPerfectBuckets
  : ∀ {m n}
  → .⦃ _ : NonZero m ⦄
  → (mid : Fin m)
  → (f : Fin n → Fin m)
  → ObservedFixedPointClassifier mid f
  → ObservedFixedPointExclusion mid f
  → (balanced : (S : SymmetryData (Fin m))
               → ∀ b → countResid f b ≡ countResid f (SymmetryData.inv S b))
  → PerfectBuckets (mkSymReflect mid) f
autoPerfectBuckets mid f classify supportExcl balanced =
  let S* = mkSymReflect mid
  in perfectFromBalancedWithSupport _≟Fin_ S* f (countResid f)
       (balancedBucketsFromCounts S* f (balanced S*))
       (supportCountsAgreeCountResid f)
       (observedResiduesMoveFromObservedSupportExclusion mid f classify supportExcl)

------------------------------------------------------------------------
-- DUAL CERTIFICATE: Static + Dynamic combined
--
-- This is the complete production artifact!

record ResonanceCertificateDyn {m n : ℕ}
       (mid : Fin m) (f : Fin n → Fin m) : Set where
  field
    -- STATIC CERTIFICATE
    S            : SymmetryData (Fin m)
    buckets      : PerfectBuckets S f
    voidOK       : HonoraryZero S (MS-fromResid f)

    -- DYNAMIC CERTIFICATE (pre-applied)
    -- Given any path xs and pointwise safe-position evidence, forbids InZone
    -- in one step.
    inviolability
      : ∀ {R} (xs : C.List (Fin m))
      → PointwiseSafe R (toℕ mid) (mapFin xs)
      → InZone        R (toℕ mid) (mapFin xs)
      → ⊥

------------------------------------------------------------------------
-- CORE CONSTRUCTOR: Function source (Fin n → Fin m)
--
-- ONE-SHOT DUAL CERTIFICATION:
-- Input:  mid, residue function, and two static witness families
--         (classify, supportExcl, balanced)
-- Output: Complete dual certificate (static + dynamic)

certifyWithDynamicsFromResid
  : ∀ {m n}
  → .⦃ _ : NonZero m ⦄
  → (mid : Fin m)
  → (f   : Fin n → Fin m)
  → ObservedFixedPointClassifier mid f
  → ObservedFixedPointExclusion mid f
  → (balanced : (S : SymmetryData (Fin m))
               → ∀ b → countResid f b ≡ countResid f (SymmetryData.inv S b))
  → ResonanceCertificateDyn mid f
certifyWithDynamicsFromResid {m} {n} mid f classify supportExcl balanced =
  let S*  = mkSymReflect mid
      PB  = autoPerfectBuckets mid f classify supportExcl balanced
      HZ  = honoraryZeroFromPerfect S* f PB
  in record
       { S        = S*
       ; buckets  = PB
       ; voidOK   = HZ
       ; inviolability = λ {R} xs safe iz →
           C.inviolabilityFromPointwiseSafe safe iz
       }

------------------------------------------------------------------------
-- CONVENIENCE CONSTRUCTOR: Vector source (Vec (Fin m) n)
--
-- For cases where residues come as a vector instead of a function.

certifyWithDynamicsFromVec
  : ∀ {m n}
  → .⦃ _ : NonZero m ⦄
  → (mid : Fin m)
  → (xs  : Vec (Fin m) n)
  → ObservedFixedPointClassifier mid (indexer xs)
  → ObservedFixedPointExclusion mid (indexer xs)
  → (balanced : (S : SymmetryData (Fin m))
               → ∀ b → countResid (indexer xs) b
                       ≡ countResid (indexer xs) (SymmetryData.inv S b))
  → ResonanceCertificateDyn mid (indexer xs)
certifyWithDynamicsFromVec mid xs classify supportExcl balanced =
  certifyWithDynamicsFromResid mid (indexer xs) classify supportExcl balanced

------------------------------------------------------------------------
-- CANONICAL EVEN-BASE CONVENIENCE
--
-- For the standard half-turn choice `mid = base / 2` in an even base `2h`,
-- callers do not need to supply the midpoint witness manually.

certifyWithDynamicsFromResidCanonicalEven
  : ∀ {k n}
  → (f : Fin n → Fin (suc k + suc k))
  → ObservedFixedPointExclusion (canonicalEvenMidpoint {k}) f
  → (balanced : (S : SymmetryData (Fin (suc k + suc k)))
               → ∀ b → countResid f b ≡ countResid f (SymmetryData.inv S b))
  → ResonanceCertificateDyn (canonicalEvenMidpoint {k}) f
certifyWithDynamicsFromResidCanonicalEven {k} f supportExcl balanced =
  certifyWithDynamicsFromResid
    (canonicalEvenMidpoint {k})
    f
    (canonicalEvenObservedFixedPointClassifier f)
    supportExcl
    balanced

certifyWithDynamicsFromVecCanonicalEven
  : ∀ {k n}
  → (xs : Vec (Fin (suc k + suc k)) n)
  → ObservedFixedPointExclusion (canonicalEvenMidpoint {k}) (indexer xs)
  → (balanced : (S : SymmetryData (Fin (suc k + suc k)))
               → ∀ b → countResid (indexer xs) b
                       ≡ countResid (indexer xs) (SymmetryData.inv S b))
  → ResonanceCertificateDyn (canonicalEvenMidpoint {k}) (indexer xs)
certifyWithDynamicsFromVecCanonicalEven {k} xs supportExcl balanced =
  certifyWithDynamicsFromVec
    (canonicalEvenMidpoint {k})
    xs
    (canonicalEvenObservedFixedPointClassifier (indexer xs))
    supportExcl
    balanced

------------------------------------------------------------------------
-- USAGE PATTERN (complete workflow)
------------------------------------------------------------------------

{-
INTEGRATION WITH 2p² WINDOW PIPELINE:

STEP 1: RUST ANALYSIS
  - Analyze window around 2p²
  - Extract primes: [p₁, p₂, ..., pₙ]
  - Compute residues: rᵢ = pᵢ mod base
  - Compute positions: [pos₁, pos₂, ..., posₙ]
  - Determine midpoint: mid = base / 2
  - Determine radius: R (exclusion zone size)

STEP 2: RUST VERIFICATION (decidable checks)
  Static checks:
    ✓ canonical even-base classifier is derived constructively for `mid = base / 2`
    ✓ noncanonical midpoint choices provide `proof-classify`
    ✓ supportExcl.zeroVoid: no residue equals 0
    ✓ supportExcl.midVoid: no residue equals mid
    ✓ balanced: symmetric bucket counts

  Dynamic checks:
    ✓ pointwiseSafe: all |posᵢ - window_mid| ≥ R

STEP 3: RUST CODE GENERATION
  Generate window_p{p}_base{base}.agda:

  ```agda
  module Window_p{p}_base{base} where

  open import CertifiedResonanceParamDyn

  -- Concrete data
  mid-val : Fin {base}
  mid-val = fromℕ< {base/2} proof

  residues-vec : Vec (Fin {base}) {n}
  residues-vec = r₁ ∷ r₂ ∷ ... ∷ rₙ ∷ []

  positions-list : List (Fin {window-size})
  positions-list = pos₁ ∷L pos₂ ∷L ... ∷L posₙ ∷L []L

  -- Static witnesses (auto-generated)
  proof-classify : ObservedFixedPointClassifier mid-val (indexer residues-vec)
  proof-classify = ... -- Generated for noncanonical midpoint choices only

  proof-supportExcl : ObservedFixedPointExclusion mid-val (indexer residues-vec)
  proof-supportExcl = record
    {
    ; zeroVoid   = ...
    ; midVoid    = ...
    }

  proof-balanced : (S : SymmetryData (Fin {base}))
                 → ∀ b → countResid (indexer residues-vec) b
                        ≡ countResid (indexer residues-vec) (SymmetryData.inv S b)
  proof-balanced = ...

  -- Dynamic witness (auto-generated)
  proof-pointwiseSafe : PointwiseSafe {R} (toℕ mid-val) (mapFin positions-list)
  proof-pointwiseSafe = ...

  -- ONE-LINE DUAL CERTIFICATION
  certificate : ResonanceCertificateDyn mid-val (indexer residues-vec)
  certificate = certifyWithDynamicsFromVec mid-val residues-vec
                  proof-classify
                  proof-supportExcl
                  proof-balanced

  -- EXTRACT BOTH PROOFS
  static-proof : HonoraryZero
                   (ResonanceCertificateDyn.S certificate)
                   (MS-fromResid (indexer residues-vec))
  static-proof = ResonanceCertificateDyn.voidOK certificate

  dynamic-proof : InZone {R} (toℕ mid-val) (mapFin positions-list) → ⊥
  dynamic-proof = ResonanceCertificateDyn.inviolability certificate positions-list proof-pointwiseSafe
  ```

STEP 4: AGDA TYPE-CHECK
  $ agda --safe window_p{p}_base{base}.agda

  If successful:
    ✓ Static certificate: Midpoint void proven
    ✓ Dynamic certificate: Exclusion zone inviolable
    ✓ Dual certification complete
    ✓ Machine-checked appendix ready

BENEFITS OF DUAL CERTIFICATION:

1. **Mechanical Causality** (Static)
   - Symmetry FORCES midpoint void
   - Constructive witness (PerfectBuckets)
   - Explains WHY void exists

2. **Structural Enforcement** (Dynamic)
   - Exclusion zone is IMPOSSIBLE to violate
  - Type-level guarantee (PointwiseSafe)
   - Explains HOW void is maintained

3. **Statistical Validation** (RMT Integration)
   - Correlate dual certs with Δ₃/β
   - Compare predicted (HL) vs proven (certs)
   - Unified mechanical + statistical framework

4. **Publication Ready**
   - Machine-checked static proof
   - Machine-checked dynamic proof
   - Complete causal explanation
   - Independent verification possible

This is the complete production interface for week-1 validation!
-}

------------------------------------------------------------------------
-- Example instantiations intentionally live outside this wrapper module.
--
-- See:
--   CERTIFIED_RESONANCE_PARAM_DYN_BASE6_SKETCH.md
--
-- The goal is to keep this file focused on the dual certification API and its
-- clean-local static/runtime composition surface, without bundling a local
-- runtime witness shell into the module itself.

------------------------------------------------------------------------
-- PRODUCTION DEPLOYMENT NOTES
------------------------------------------------------------------------

{-
WEEK-1 VALIDATION PLAN:

DAY 1-2: Static Certification
  - Generate certificates for 100 windows (various bases)
  - Verify static Honorary Zero (midpoint void)
  - Correlate with Δ₃ spectral rigidity
  - Success metric: >80% certification rate

DAY 3-4: Dynamic Certification
  - Add PointwiseSafe witnesses to same 100 windows
  - Verify dynamic Inviolability (exclusion zones)
  - Correlate with β repulsion exponent
  - Success metric: >80% dual certification rate

DAY 5-6: Statistical Integration
  - Compare certified vs non-certified windows
  - Analyze Δ₃/β differences
  - Test HL predictions vs constructive proofs
  - Identify patterns (which bases/radii certify best)

DAY 7: Documentation & Publication Prep
  - Archive all certificate files
  - Generate aggregated statistics
  - Prepare machine-checked appendix
  - Draft publication with dual framework

DEPLOYMENT AUTOMATION:

1. Batch Generation:
   ```bash
   cargo run --example generate_dual_certificates \
       --bases 6,14,18,30 \
       --primes 7,11,13,17,19 \
       --output hz_out/
   ```

2. Batch Verification:
   ```bash
   cd hz_out/
   for f in Window_*.agda; do
       agda --safe "$f" && echo "✓ DUAL: $f" || echo "✗ FAIL: $f"
   done
   ```

3. Statistical Analysis:
   ```bash
   cargo run --example analyze_dual_certificates \
       --cert-dir hz_out/ \
       --stats-dir hz_res/ \
       --output dual_analysis.json
   ```

EXPECTED OUTCOMES:

- Mechanical explanation (symmetry forces void)
- Statistical validation (RMT/spectral correlation)
- Unified framework (constructive + probabilistic)
- Publication-ready artifacts (machine-checked)

This is the complete production stack for dual certification! ✓
-}
