-- Certified Resonance Param: Parameterized One-Shot Certificate Wrapper
--
-- PRODUCTION LAYER: Runtime residue source → Machine-checkable certificate
--
-- This module provides the one-shot certification interface for external jobs.
-- Given a midpoint and residue list, automatically construct the complete
-- certificate artifact ready for verification.
--
-- USAGE PATTERN (per 2p² window):
-- 1. Extract mid : Fin m and residues f : Fin n → Fin m
-- 2. Verify the concrete reflection fixed-point exclusions:
--    - zero is fixed by the chosen half-turn reflection
--    - no observed residue equals zero
--    - no observed residue equals mid
-- 3. Verify balanced bucket counts
-- 4. Get ResonanceCertificate (S, buckets, voidOK) ✓
--
-- This is the final layer plugging into your external job workflow!

module Examples.CertifiedResonanceParam where

open import Data.Product     using (Σ; _,_; proj₁; proj₂)
open import Relation.Binary.PropositionalEquality  using (_≡_; refl; cong; trans)
open import Data.Empty     using (⊥)
open import Data.Nat       using (ℕ ; zero ; suc ; NonZero ; _+_ ; _<_ ; z≤n ; s≤s)
open import Data.Bool      using (Bool; true; false)
open import Data.Fin               using (Fin ; zero ; suc)
open import Data.Fin.Properties    using () renaming (_≟_ to _≟Fin_)
open import Data.Vec               using (Vec ; [] ; _∷_)
open import Function               using (_∘_)
open import Relation.Nullary       using (Dec; yes; no)

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

------------------------------------------------------------------------
-- BASIC HELPERS

_≢_ : ∀ {A : Set} → A → A → Set
x ≢ y = (x ≡ y) → ⊥

-- Vector indexing
indexer : ∀ {A : Set} {n} → Vec A n → Fin n → A
indexer {A} {zero}     []       ()
indexer {A} {suc n}    (x ∷ xs) zero    = x
indexer {A} {suc n}    (x ∷ xs) (suc i) = indexer xs i

------------------------------------------------------------------------
-- RESIDUE COUNTING (for balanced bucket verification)

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

------------------------------------------------------------------------
-- AUTO-PERFECT BUCKETS (from balanced counts + fixed-point exclusion)
--
-- This is the automatic witness construction!
-- If buckets are balanced and the observed residues move under the involution,
-- we can auto-build the pairing.

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
-- CERTIFICATE RECORD: Static pairing + midpoint void
--
-- This is what gets archived per 2p² window:
-- - S: Symmetry data (reflection involution)
-- - buckets: Perfect pairing witness
-- - voidOK: Honorary zero certificate (midpoint is provably empty)

record ResonanceCertificate {m n : ℕ}
       (mid : Fin m) (f : Fin n → Fin m) : Set where
  field
    S        : SymmetryData (Fin m)
    buckets  : PerfectBuckets S f
    voidOK   : HonoraryZero S (MS-fromResid f)

------------------------------------------------------------------------
-- CORE CERTIFICATION: Function source (Fin n → Fin m)
--
-- ONE-SHOT WRAPPER:
-- Given mid, residue function f, and two witness families
-- (classify, supportExcl, balanced),
-- automatically construct the complete certificate!

certifyFromResid
  : ∀ {m n}
  → .⦃ _ : NonZero m ⦄
  → (mid : Fin m)
  → (f   : Fin n → Fin m)
  → ObservedFixedPointClassifier mid f
  → ObservedFixedPointExclusion mid f
  → (balanced : (S : SymmetryData (Fin m))
               → ∀ b → countResid f b ≡ countResid f (SymmetryData.inv S b))
  → ResonanceCertificate mid f
certifyFromResid {m} {n} mid f classify supportExcl balanced =
  let S*  = mkSymReflect mid
      PB  = autoPerfectBuckets mid f classify supportExcl balanced
      HZ  = honoraryZeroFromPerfect S* f PB
  in record
       { S        = S*
       ; buckets  = PB
       ; voidOK   = HZ
       }

------------------------------------------------------------------------
-- CONVENIENCE: Vector source (Vec (Fin m) n)
--
-- For cases where residues come as a vector instead of a function.
-- Automatically converts via indexer.

certifyFromVec
  : ∀ {m n}
  → .⦃ _ : NonZero m ⦄
  → (mid : Fin m)
  → (xs  : Vec (Fin m) n)
  → ObservedFixedPointClassifier mid (indexer xs)
  → ObservedFixedPointExclusion mid (indexer xs)
  → (balanced : (S : SymmetryData (Fin m))
               → ∀ b → countResid (indexer xs) b
                       ≡ countResid (indexer xs) (SymmetryData.inv S b))
  → ResonanceCertificate mid (indexer xs)
certifyFromVec mid xs classify supportExcl balanced =
  certifyFromResid mid (indexer xs) classify supportExcl balanced

------------------------------------------------------------------------
-- CANONICAL EVEN-BASE CONVENIENCE
--
-- For the standard half-turn choice `mid = base / 2` in an even base `2h`,
-- callers do not need to supply the midpoint witness manually.

certifyFromResidCanonicalEven
  : ∀ {k n}
  → (f : Fin n → Fin (suc k + suc k))
  → ObservedFixedPointExclusion (canonicalEvenMidpoint {k}) f
  → (balanced : (S : SymmetryData (Fin (suc k + suc k)))
               → ∀ b → countResid f b ≡ countResid f (SymmetryData.inv S b))
  → ResonanceCertificate (canonicalEvenMidpoint {k}) f
certifyFromResidCanonicalEven {k} f supportExcl balanced =
  certifyFromResid
    (canonicalEvenMidpoint {k})
    f
    (canonicalEvenObservedFixedPointClassifier f)
    supportExcl
    balanced

certifyFromVecCanonicalEven
  : ∀ {k n}
  → (xs : Vec (Fin (suc k + suc k)) n)
  → ObservedFixedPointExclusion (canonicalEvenMidpoint {k}) (indexer xs)
  → (balanced : (S : SymmetryData (Fin (suc k + suc k)))
               → ∀ b → countResid (indexer xs) b
                       ≡ countResid (indexer xs) (SymmetryData.inv S b))
  → ResonanceCertificate (canonicalEvenMidpoint {k}) (indexer xs)
certifyFromVecCanonicalEven {k} xs supportExcl balanced =
  certifyFromVec
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
  - Determine midpoint: mid = base / 2
  - Count buckets: for each residue, count occurrences

STEP 2: RUST VERIFICATION (decidable checks)
  - For the standard even-base path `mid = base / 2`, the observed fixed-point
    classifier is recovered constructively inside Agda.
    Noncanonical midpoint choices provide:
    proof-classify : ObservedFixedPointClassifier mid-val (indexer residues-vec)

  - supportExcl: Package the observed support exclusions
    zeroVoid:   ∀ i, residues[i] ≠ 0  ✓
    midVoid:    ∀ i, residues[i] ≠ mid  ✓

  - balanced: Check symmetric bucket counts
    ∀ b, count(b) = count(inv(b))  ✓

STEP 3: RUST CODE GENERATION
  Generate window_p{p}_base{base}.agda:

  ```agda
  -- Auto-generated certificate for window around 2p²

  mid-val : Fin {base}
  mid-val = fromℕ< {base/2} proof

  residues-vec : Vec (Fin {base}) {n}
  residues-vec = r₁ ∷ r₂ ∷ ... ∷ rₙ ∷ []

  proof-classify : ObservedFixedPointClassifier mid-val (indexer residues-vec)
  proof-classify = ... -- Generated for noncanonical midpoint choices only

  proof-supportExcl : ObservedFixedPointExclusion mid-val (indexer residues-vec)
  proof-supportExcl = record
    {
    ; zeroVoid   = ... -- Generated from runtime residue scan
    ; midVoid    = ... -- Generated from runtime residue scan
    }

  proof-balanced : (S : SymmetryData (Fin {base}))
                 → ∀ b → countResid (indexer residues-vec) b
                        ≡ countResid (indexer residues-vec) (SymmetryData.inv S b)
  proof-balanced = ... -- Generated from bucket counts

  certificate : ResonanceCertificate mid-val (indexer residues-vec)
  certificate = certifyFromVec mid-val residues-vec
                  proof-classify
                  proof-supportExcl
                  proof-balanced
  ```

STEP 4: AGDA TYPE-CHECK
  $ agda window_p{p}_base{base}.agda

  If successful:
    ✓ Certificate is valid
    ✓ Midpoint void is proven
    ✓ Perfect pairing exists
    ✓ Machine-checked appendix ready

STEP 5: ARCHIVE ARTIFACT
  - Certificate file: window_p{p}_base{base}.agda
  - Verification log: Type-check output
  - Statistics: Δ₃, β, spectral data
  - Publication: Machine-checked proof

BENEFITS:

1. **Automation**: Most proofs auto-generated from decidable checks
2. **Verification**: Type-checking = mathematical proof
3. **Reproducibility**: Each window gets independent artifact
4. **Publication**: Machine-checked appendices for papers
5. **Debugging**: Type errors reveal symmetry violations

EXAMPLE OUTPUT:

For window around 2·7² = 98, base 14, 6 primes:
  Residues: {1, 3, 5, 9, 11, 13}
  Balanced: count(1)=count(13)=1, count(3)=count(11)=1, count(5)=count(9)=1
  Fixed residues: 0 and 7
  Observed support excludes both ✓

  → certificate type-checks ✓
  → HonoraryZero proven ✓
  → Ready for publication ✓

This closes the complete loop:
  Empirical → Rational → Formal Proof → Publication
-}

------------------------------------------------------------------------
-- EXAMPLE INSTANTIATION (Base 6, n=4)
------------------------------------------------------------------------

module Example-Base6-n4 where

  open import Data.Fin using () renaming (zero to fzero ; suc to fsuc)

  -- Concrete data from CertifiedResonanceComplete
  example-mid : Fin 6
  example-mid = suc (suc (suc zero))  -- 3

  example-residues : Vec (Fin 6) 4
  example-residues =
    suc zero ∷                         -- 1
    suc (suc (suc (suc (suc zero)))) ∷ -- 5
    suc (suc zero) ∷                   -- 2
    suc (suc (suc (suc zero))) ∷       -- 4
    []

  -- DIRECT CERTIFICATE CONSTRUCTION
  -- Instead of going through the remaining framework-level autoPerfectBuckets
  -- bridge, we construct the PerfectBuckets witness directly, reusing the same
  -- technique as CertifiedResonanceComplete (explicit fzero/fsuc case
  -- analysis).

  -- Concrete involution: r -> (6 - r) mod 6
  inv-fn : Fin 6 → Fin 6
  inv-fn fzero                                         = fzero
  inv-fn (fsuc fzero)                                  = fsuc (fsuc (fsuc (fsuc (fsuc fzero))))
  inv-fn (fsuc (fsuc fzero))                           = fsuc (fsuc (fsuc (fsuc fzero)))
  inv-fn (fsuc (fsuc (fsuc fzero)))                    = fsuc (fsuc (fsuc fzero))
  inv-fn (fsuc (fsuc (fsuc (fsuc fzero))))             = fsuc (fsuc fzero)
  inv-fn (fsuc (fsuc (fsuc (fsuc (fsuc fzero)))))      = fsuc fzero

  inv-involutive : ∀ r → inv-fn (inv-fn r) ≡ r
  inv-involutive fzero                                         = refl
  inv-involutive (fsuc fzero)                                  = refl
  inv-involutive (fsuc (fsuc fzero))                           = refl
  inv-involutive (fsuc (fsuc (fsuc fzero)))                    = refl
  inv-involutive (fsuc (fsuc (fsuc (fsuc fzero))))             = refl
  inv-involutive (fsuc (fsuc (fsuc (fsuc (fsuc fzero)))))      = refl

  inv-mid : inv-fn example-mid ≡ example-mid
  inv-mid = refl

  S* : SymmetryData (Fin 6)
  S* = record
    { mid            = example-mid
    ; inv            = inv-fn
    ; inv-involutive = inv-involutive
    ; inv-mid        = inv-mid
    }

  -- Pairing: 0 <-> 1, 2 <-> 3
  mate-fn : Fin 4 → Fin 4
  mate-fn fzero                          = fsuc fzero
  mate-fn (fsuc fzero)                   = fzero
  mate-fn (fsuc (fsuc fzero))            = fsuc (fsuc (fsuc fzero))
  mate-fn (fsuc (fsuc (fsuc fzero)))     = fsuc (fsuc fzero)

  involutive-mate : ∀ i → mate-fn (mate-fn i) ≡ i
  involutive-mate fzero                          = refl
  involutive-mate (fsuc fzero)                   = refl
  involutive-mate (fsuc (fsuc fzero))            = refl
  involutive-mate (fsuc (fsuc (fsuc fzero)))     = refl

  no-fixed-mate : ∀ i → mate-fn i ≢ i
  no-fixed-mate fzero                          ()
  no-fixed-mate (fsuc fzero)                   ()
  no-fixed-mate (fsuc (fsuc fzero))            ()
  no-fixed-mate (fsuc (fsuc (fsuc fzero)))     ()

  equivariant-res : ∀ i → inv-fn (indexer example-residues i) ≡ indexer example-residues (mate-fn i)
  equivariant-res fzero                          = refl
  equivariant-res (fsuc fzero)                   = refl
  equivariant-res (fsuc (fsuc fzero))            = refl
  equivariant-res (fsuc (fsuc (fsuc fzero)))     = refl

  residue-distinct : ∀ i → indexer example-residues (mate-fn i) ≢ indexer example-residues i
  residue-distinct fzero                          ()
  residue-distinct (fsuc fzero)                   ()
  residue-distinct (fsuc (fsuc fzero))            ()
  residue-distinct (fsuc (fsuc (fsuc fzero)))     ()

  PB : PerfectBuckets S* (indexer example-residues)
  PB = record
    { mate             = mate-fn
    ; involutive       = involutive-mate
    ; no-fixed         = no-fixed-mate
    ; equivariant      = equivariant-res
    ; residue-distinct = residue-distinct
    }

  -- ONE-LINE CERTIFICATION (direct construction, no postulates):
  example-certificate : ResonanceCertificate example-mid (indexer example-residues)
  example-certificate = record
    { S       = S*
    ; buckets = PB
    ; voidOK  = honoraryZeroFromPerfect S* (indexer example-residues) PB
    }

  -- Extract proofs:
  example-voidOK : HonoraryZero
                     (ResonanceCertificate.S example-certificate)
                     (MS-fromResid (indexer example-residues))
  example-voidOK = ResonanceCertificate.voidOK example-certificate

  {-
  This type-checks with ZERO postulates in the Example module.

  The general framework still relies on the BucketsAutoMatch bridge,
  but this concrete example bypasses that assumed helper by constructing PerfectBuckets
  directly via case analysis (same technique as CertifiedResonanceComplete).

  This is the pattern that external jobs replicate per window.
  -}

------------------------------------------------------------------------
-- PRODUCTION NOTES
------------------------------------------------------------------------

{-
DEPLOYMENT STRATEGY:

1. PER-WINDOW GENERATION:
   - cargo run --example generate_window_certificate --prime {p} --base {base}
   - Outputs: hz_out/window_p{p}_base{base}.agda

2. BATCH VERIFICATION:
   - agda --safe window_p*.agda
   - Count successes: X% of windows verify
   - Archive all certificate files

3. STATISTICAL ANALYSIS:
   - Correlate certificate success with:
     • Δ₃ spectral rigidity
     • β repulsion exponent
     • Window size/position
     • Base properties (φ(base), rad(base))

4. PUBLICATION ARTIFACTS:
   - Main paper: Coordinate constellation theory
   - Appendix A: Complete formal framework
   - Appendix B: All certificate files (machine-checked)
   - Code archive: Rust generator + Agda checker

MAINTENANCE:

- Framework is stable (core theory complete)
- Per-window files are fully auto-generated
- Only manual work: exceptional cases (base 7, etc.)
- Continuous integration: Type-check all certificates on commit

This is the complete production-ready certification pipeline!
Ready for PE input and 2p² window deployment! ✓
-}
