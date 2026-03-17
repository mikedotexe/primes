-- Symmetry Finite Reflect: Concrete Modular Involution
--
-- INSTANTIATES: Abstract SymmetryData with Fin m and reflection involution
--
-- This module provides the concrete modular arithmetic implementation
-- for the abstract framework, specifically the reflection involution
-- r ↦ (2·mid - r) mod m used in coordinate constellation analysis.
--
-- Production-ready for 2p² window certification.

module Theorems.Abstract.SymmetryFiniteReflect where

open import Data.Product     using (Σ; _,_; proj₁; proj₂)
open import Relation.Binary.PropositionalEquality  using (_≡_; refl; sym; cong; trans; subst)
open import Data.Empty     using (⊥)
open import Data.Nat       using (ℕ; zero; suc; _+_; _*_ ; _∸_; NonZero)
open import Data.Nat               using (_<_ ; _≤_ ; z≤n ; s≤s)
open import Data.Nat.DivMod        using (_%_; _/_; _mod_; m%n<n; m<n⇒m%n≡m; [m+n]%n≡m%n; [m+kn]%n≡m%n; %-congˡ; n%n≡0; m≡m%n+[m/n]*n)
open import Data.Nat.Properties    using (+-identityʳ; *-identityˡ; 0<1+n; +-assoc; +-comm; m+n∸n≡m; m+n∸m≡n; m∸n+n≡m; ≤-trans; m≤n+m; <⇒≤; n≢0⇒n>0; m<n+m; +-∸-assoc; *-cancelˡ-≡; m<n+o⇒m∸n<o)
open import Data.Fin               using (Fin; toℕ; fromℕ<; _↑ʳ_) renaming (zero to fzero)
open import Data.Fin.Properties    using (_≟_; toℕ-injective; toℕ-fromℕ<; toℕ-↑ʳ; toℕ<n)
open import Data.Nat.Base          using (≢-nonZero⁻¹) renaming (NonZero to NonZero-inst)
open import Data.Sum               using (_⊎_; inj₁; inj₂)
open import Relation.Nullary       using (Dec; yes; no)
open import Relation.Binary.PropositionalEquality.Properties using (module ≡-Reasoning)

-- Import abstract framework
open import Theorems.Abstract.SymmetryImpliesRepulsion
  using ( SymmetryData ; MS ; Pairing ; HonoraryZero
        ; SymmetryImpliesRepulsion )
open import Theorems.Abstract.SymmetryFromList
  using ( PerfectBuckets ; pairingFromPerfect ; MS-fromResid )
open import Theorems.Abstract.BucketsAutoMatch
  using (ObservedResiduesMove)

open ≡-Reasoning

------------------------------------------------------------------------
-- FINITE-BASE REFLECTION: inv r = (2·mid - r) mod m on B = Fin m
------------------------------------------------------------------------

-- The reflection involution: r ↦ (2·mid - r) mod m
-- This is the concrete implementation used in coordinate constellation analysis
-- Note: In stdlib 2.3, _mod_ returns Fin m directly, so no conversion needed!
reflect : ∀ {m} → .⦃ _ : NonZero m ⦄ → (mid : Fin m) → Fin m → Fin m
reflect {m} mid r =
  let a   = 2 * toℕ mid + m      -- Add m to ensure non-negative
      raw = a ∸ toℕ r            -- Compute 2·mid - r
  in raw mod m                   -- Returns Fin m directly in stdlib 2.3

_≢_ : ∀ {A : Set} → A → A → Set
x ≢ y = x ≡ y → ⊥

finZero : ∀ {m} → .⦃ _ : NonZero m ⦄ → Fin m
finZero {m} = zero mod m

HalfTurnMidpoint : ∀ {m} → .⦃ _ : NonZero m ⦄ → Fin m → Set
HalfTurnMidpoint mid = reflect mid finZero ≡ finZero

-- Canonical midpoint for the standard even-base case m = 2h.
canonicalEvenMidpoint
  : ∀ {k}
  → Fin (suc k + suc k)
canonicalEvenMidpoint {k} = suc k ↑ʳ (fzero {n = k})

private
  two-times≡double : ∀ n → 2 * n ≡ n + n
  two-times≡double n = begin
    2 * n       ≡⟨⟩
    n + (1 * n) ≡⟨ cong (n +_) (*-identityˡ n) ⟩
    n + n       ∎

  canonicalEvenMidpoint-value
    : ∀ {k}
    → toℕ (canonicalEvenMidpoint {k}) ≡ suc k
  canonicalEvenMidpoint-value {k} = begin
    toℕ (canonicalEvenMidpoint {k}) ≡⟨ toℕ-↑ʳ (suc k) (fzero {n = k}) ⟩
    suc k + toℕ (fzero {n = k})     ≡⟨⟩
    suc k + zero                    ≡⟨ +-identityʳ (suc k) ⟩
    suc k                           ∎

  canonicalEvenMidpoint-double
    : ∀ {k}
    → 2 * toℕ (canonicalEvenMidpoint {k}) ≡ suc k + suc k
  canonicalEvenMidpoint-double {k} = begin
    2 * toℕ (canonicalEvenMidpoint {k})
      ≡⟨ cong (λ n → 2 * n) canonicalEvenMidpoint-value ⟩
    2 * suc k     ≡⟨ two-times≡double (suc k) ⟩
    suc k + suc k ∎

toℕ-finZero
  : ∀ {m}
  → .⦃ _ : NonZero m ⦄
  → toℕ (finZero {m}) ≡ zero
toℕ-finZero {m} = begin
  toℕ (finZero {m})
    ≡⟨ toℕ-fromℕ< (m%n<n zero m) ⟩
  zero % m
    ≡⟨ m<n⇒m%n≡m (n≢0⇒n>0 (≢-nonZero⁻¹ m)) ⟩
  zero
  ∎

canonicalEvenHalfTurnMidpoint
  : ∀ {k}
  → .⦃ _ : NonZero (suc k + suc k) ⦄
  → HalfTurnMidpoint (canonicalEvenMidpoint {k})
canonicalEvenHalfTurnMidpoint {k} = toℕ-injective (begin
  toℕ (reflect (canonicalEvenMidpoint {k}) (finZero {suc k + suc k}))
    ≡⟨ toℕ-fromℕ<
         (m%n<n
           (2 * toℕ (canonicalEvenMidpoint {k}) + (suc k + suc k) ∸ toℕ (finZero {suc k + suc k}))
           (suc k + suc k))
      ⟩
  (2 * toℕ (canonicalEvenMidpoint {k}) + (suc k + suc k) ∸ toℕ (finZero {suc k + suc k})) % (suc k + suc k)
    ≡⟨ cong
         (λ z → (2 * toℕ (canonicalEvenMidpoint {k}) + (suc k + suc k) ∸ z) % (suc k + suc k))
         (toℕ-finZero {suc k + suc k})
      ⟩
  (2 * toℕ (canonicalEvenMidpoint {k}) + (suc k + suc k) ∸ zero) % (suc k + suc k)
    ≡⟨⟩
  (2 * toℕ (canonicalEvenMidpoint {k}) + (suc k + suc k)) % (suc k + suc k)
    ≡⟨ [m+n]%n≡m%n (2 * toℕ (canonicalEvenMidpoint {k})) (suc k + suc k) ⟩
  (2 * toℕ (canonicalEvenMidpoint {k})) % (suc k + suc k)
    ≡⟨ %-congˡ (canonicalEvenMidpoint-double {k}) ⟩
  (suc k + suc k) % (suc k + suc k)
    ≡⟨ n%n≡0 (suc k + suc k) ⟩
  zero
    ≡⟨ sym (toℕ-finZero {suc k + suc k}) ⟩
  toℕ (finZero {suc k + suc k})
  ∎)

reflect-value
  : ∀ {m}
  → .⦃ _ : NonZero m ⦄
  → (mid r : Fin m)
  → toℕ (reflect mid r) ≡ ((2 * toℕ mid + m) ∸ toℕ r) % m
reflect-value {m} mid r =
  toℕ-fromℕ< (m%n<n ((2 * toℕ mid + m) ∸ toℕ r) m)

ObservedFixedPointClassifier
  : ∀ {m n} → .⦃ _ : NonZero m ⦄ → (mid : Fin m) → (f : Fin n → Fin m) → Set
ObservedFixedPointClassifier mid f =
  ∀ i → reflect mid (f i) ≡ f i → (f i ≡ finZero) ⊎ (f i ≡ mid)

NonzeroFixedPointClassifier
  : ∀ {m}
  → .⦃ _ : NonZero m ⦄
  → (mid : Fin m)
  → Set
NonzeroFixedPointClassifier mid =
  ∀ r → r ≢ finZero → reflect mid r ≡ r → r ≡ mid

-- On the observed support side, the certification lane only needs the residue
-- scan to exclude those two classified fixed residues.
record ObservedFixedPointExclusion {m n : ℕ}
       .⦃ _ : NonZero m ⦄
       (mid : Fin m)
       (f : Fin n → Fin m)
       : Set where
  field
    zeroVoid   : ∀ i → f i ≢ finZero
    midVoid    : ∀ i → f i ≢ mid

-- Properties of reflection
--
-- Public certification consumers only need the observed-only classifier helper
-- below. The broader half-turn arithmetic classifier is kept private because
-- no downstream module consumes it directly.
reflect-involutive : ∀ {m} → .⦃ _ : NonZero m ⦄ → (mid : Fin m) (r : Fin m)
                   → reflect mid (reflect mid r) ≡ r
reflect-involutive {m} mid r = toℕ-injective (begin
  toℕ (reflect mid (reflect mid r))
    ≡⟨ reflect-value mid (reflect mid r) ⟩
  (a ∸ toℕ (reflect mid r)) % m
    ≡⟨ cong (λ z → (a ∸ z) % m) (reflect-value mid r) ⟩
  (a ∸ (A % m)) % m
    ≡⟨ %-congˡ roundtrip-raw ⟩
  ((A / m) * m + toℕ r) % m
    ≡⟨ %-congˡ (+-comm ((A / m) * m) (toℕ r)) ⟩
  (toℕ r + (A / m) * m) % m
    ≡⟨ [m+kn]%n≡m%n (toℕ r) (A / m) m ⟩
  toℕ r % m
    ≡⟨ m<n⇒m%n≡m (toℕ<n r) ⟩
  toℕ r
  ∎)
  where
    a : ℕ
    a = 2 * toℕ mid + m

    A : ℕ
    A = a ∸ toℕ r

    r≤a : toℕ r ≤ a
    r≤a = ≤-trans (<⇒≤ (toℕ<n r)) (m≤n+m m (2 * toℕ mid))

    expand-a : a ≡ A % m + ((A / m) * m + toℕ r)
    expand-a = begin
      a
        ≡⟨ sym (m∸n+n≡m r≤a) ⟩
      A + toℕ r
        ≡⟨ cong (λ z → z + toℕ r) (m≡m%n+[m/n]*n A m) ⟩
      (A % m + (A / m) * m) + toℕ r
        ≡⟨ +-assoc (A % m) ((A / m) * m) (toℕ r) ⟩
      A % m + ((A / m) * m + toℕ r)
      ∎

    roundtrip-raw : a ∸ (A % m) ≡ (A / m) * m + toℕ r
    roundtrip-raw = begin
      a ∸ (A % m)
        ≡⟨ cong (λ z → z ∸ (A % m)) expand-a ⟩
      (A % m + ((A / m) * m + toℕ r)) ∸ (A % m)
        ≡⟨ m+n∸m≡n (A % m) ((A / m) * m + toℕ r) ⟩
      (A / m) * m + toℕ r
      ∎

reflect-mid : ∀ {m} → .⦃ _ : NonZero m ⦄ → (mid : Fin m)
            → reflect mid mid ≡ mid
reflect-mid {m} mid = toℕ-injective (begin
  toℕ (reflect mid mid)
    ≡⟨ toℕ-fromℕ<
         (m%n<n
           ((2 * toℕ mid + m) ∸ toℕ mid)
           m)
      ⟩
  ((2 * toℕ mid + m) ∸ toℕ mid) % m
    ≡⟨ %-congˡ (begin
         (2 * toℕ mid + m) ∸ toℕ mid
           ≡⟨ cong (λ h → (h + m) ∸ toℕ mid) (two-times≡double (toℕ mid)) ⟩
         ((toℕ mid + toℕ mid) + m) ∸ toℕ mid
           ≡⟨ cong (_∸ toℕ mid) (+-assoc (toℕ mid) (toℕ mid) m) ⟩
         (toℕ mid + (toℕ mid + m)) ∸ toℕ mid
           ≡⟨ cong (_∸ toℕ mid) (cong (toℕ mid +_) (+-comm (toℕ mid) m)) ⟩
         (toℕ mid + (m + toℕ mid)) ∸ toℕ mid
           ≡⟨ cong (_∸ toℕ mid) (sym (+-assoc (toℕ mid) m (toℕ mid))) ⟩
         ((toℕ mid + m) + toℕ mid) ∸ toℕ mid
           ≡⟨ m+n∸n≡m (toℕ mid + m) (toℕ mid) ⟩
         toℕ mid + m
         ∎)
      ⟩
  (toℕ mid + m) % m
    ≡⟨ [m+n]%n≡m%n (toℕ mid) m ⟩
  toℕ mid % m
    ≡⟨ m<n⇒m%n≡m (toℕ<n mid) ⟩
  toℕ mid
  ∎)

private
  toℕ-nonzero
    : ∀ {m}
    → .⦃ _ : NonZero m ⦄
    → (r : Fin m)
    → r ≢ finZero
    → toℕ r ≢ zero
  toℕ-nonzero {m} r r≢zero rℕ≡0 =
    r≢zero
      (toℕ-injective
         (begin
            toℕ r
              ≡⟨ rℕ≡0 ⟩
            zero
              ≡⟨ sym (toℕ-finZero {m}) ⟩
            toℕ (finZero {m})
          ∎))

  double-injective
    : ∀ {a b}
    → a + a ≡ b + b
    → a ≡ b
  double-injective {a} {b} eq =
    *-cancelˡ-≡ a b 2
      (begin
         2 * a
           ≡⟨ two-times≡double a ⟩
         a + a
           ≡⟨ eq ⟩
         b + b
           ≡⟨ sym (two-times≡double b) ⟩
         2 * b
       ∎)

  canonicalEvenNonzeroFixedPointClassifier
    : ∀ {k}
    → NonzeroFixedPointClassifier (canonicalEvenMidpoint {k})
  canonicalEvenNonzeroFixedPointClassifier {k} r r≢zero fixed =
    toℕ-injective (double-injective double-eq-mid)
    where
      m : ℕ
      m = suc k + suc k

      x : ℕ
      x = toℕ r

      x≤m : x ≤ m
      x≤m = <⇒≤ (toℕ<n r)

      x>0 : zero < x
      x>0 = n≢0⇒n>0 (toℕ-nonzero r r≢zero)

      m∸x<m : m ∸ x < m
      m∸x<m =
        m<n+o⇒m∸n<o m x
          (m<n+m m x>0)

      fixedNat : x ≡ m ∸ x
      fixedNat = begin
        x
          ≡⟨ sym (cong toℕ fixed) ⟩
        toℕ (reflect (canonicalEvenMidpoint {k}) r)
          ≡⟨ reflect-value (canonicalEvenMidpoint {k}) r ⟩
        ((2 * toℕ (canonicalEvenMidpoint {k}) + m) ∸ x) % m
          ≡⟨ cong (λ z → ((z + m) ∸ x) % m) (canonicalEvenMidpoint-double {k}) ⟩
        ((m + m) ∸ x) % m
          ≡⟨ cong (_% m) (+-∸-assoc m x≤m) ⟩
        (m + (m ∸ x)) % m
          ≡⟨ cong (_% m) (+-comm m (m ∸ x)) ⟩
        ((m ∸ x) + m) % m
          ≡⟨ [m+n]%n≡m%n (m ∸ x) m ⟩
        (m ∸ x) % m
          ≡⟨ m<n⇒m%n≡m m∸x<m ⟩
        m ∸ x
        ∎

      double-eq : x + x ≡ suc k + suc k
      double-eq = begin
        x + x
          ≡⟨ cong (_+ x) fixedNat ⟩
        (m ∸ x) + x
          ≡⟨ m∸n+n≡m x≤m ⟩
        suc k + suc k
        ∎

      double-eq-mid : x + x ≡ toℕ (canonicalEvenMidpoint {k}) + toℕ (canonicalEvenMidpoint {k})
      double-eq-mid =
        trans
          double-eq
          (sym (cong (λ n → n + n) (canonicalEvenMidpoint-value {k})))

  half-turn-fixed-point-case
                      : ∀ {m} → .⦃ _ : NonZero m ⦄ → (mid : Fin m)
                      → NonzeroFixedPointClassifier mid
                      → (r : Fin m)
                      → reflect mid r ≡ r
                      → (r ≡ finZero) ⊎ (r ≡ mid)
  half-turn-fixed-point-case mid classify r fixed with r ≟ finZero
  ... | yes r≡zero = inj₁ r≡zero
  ... | no  r≢zero = inj₂ (classify r r≢zero fixed)

------------------------------------------------------------------------
-- CONSTRUCT CONCRETE SYMMETRY DATA
--
-- This is the canonical SymmetryData for modular arithmetic!

mkSymReflect : ∀ {m} → .⦃ _ : NonZero m ⦄ → (mid : Fin m) → SymmetryData (Fin m)
mkSymReflect mid =
  record
    { mid            = mid
    ; inv            = reflect mid
    ; inv-involutive = reflect-involutive mid
    ; inv-mid        = reflect-mid mid
    }

observedFixedPointClassifierFromNonzero
  : ∀ {m n}
  → .⦃ _ : NonZero m ⦄
  → (mid : Fin m)
  → (f : Fin n → Fin m)
  → NonzeroFixedPointClassifier mid
  → ObservedFixedPointClassifier mid f
-- Public entry point for the fixed-point classifier lane used by the
-- certification wrappers. The broader arithmetic burden remains pointwise;
-- callers only see the observed-support classifier they actually need.
observedFixedPointClassifierFromNonzero mid f classify i fixed =
  half-turn-fixed-point-case mid classify (f i) fixed

canonicalEvenObservedFixedPointClassifier
  : ∀ {k n}
  → (f : Fin n → Fin (suc k + suc k))
  → ObservedFixedPointClassifier (canonicalEvenMidpoint {k}) f
canonicalEvenObservedFixedPointClassifier {k} f =
  observedFixedPointClassifierFromNonzero
    (canonicalEvenMidpoint {k})
    f
    canonicalEvenNonzeroFixedPointClassifier

observedFixedPointExclusion
  : ∀ {m n}
  → .⦃ _ : NonZero m ⦄
  → (mid : Fin m)
  → (f : Fin n → Fin m)
  → ObservedFixedPointClassifier mid f
  → ObservedFixedPointExclusion mid f
  → ∀ i → reflect mid (f i) ≡ f i → ⊥
observedFixedPointExclusion mid f classify support i fixed
  with classify i fixed
... | inj₁ fi≡zero =
  ObservedFixedPointExclusion.zeroVoid support i fi≡zero
... | inj₂ fi≡mid =
  ObservedFixedPointExclusion.midVoid support i fi≡mid

observedResiduesMoveFromFixedPointContracts
  : ∀ {m n}
  → .⦃ _ : NonZero m ⦄
  → (mid : Fin m)
  → (f : Fin n → Fin m)
  → ObservedFixedPointClassifier mid f
  → ObservedFixedPointExclusion mid f
  → ObservedResiduesMove (mkSymReflect mid) f
observedResiduesMoveFromFixedPointContracts mid f classify support i fixed =
  observedFixedPointExclusion mid f classify support i fixed

observedResiduesMoveFromObservedSupportExclusion
  : ∀ {m n}
  → .⦃ _ : NonZero m ⦄
  → (mid : Fin m)
  → (f : Fin n → Fin m)
  → ObservedFixedPointClassifier mid f
  → ObservedFixedPointExclusion mid f
  → ObservedResiduesMove (mkSymReflect mid) f
observedResiduesMoveFromObservedSupportExclusion mid f classify support =
  observedResiduesMoveFromFixedPointContracts
    mid
    f
    classify
    support

------------------------------------------------------------------------
-- CONVENIENCE: Build Pairing from permutation witness on indices
--
-- A permutation-level witness is exactly PerfectBuckets; reuse it.

pairingFromPermutation
  : ∀ {m n}
  → (S  : SymmetryData (Fin m))
  → (f  : Fin n → Fin m)
  → PerfectBuckets S f
  → Pairing S (MS-fromResid f)
pairingFromPermutation = pairingFromPerfect

-- Ready-to-use honorary zero certificate from a permutation witness
honoraryZeroFromPermutation
  : ∀ {m n}
  → (S  : SymmetryData (Fin m))
  → (f  : Fin n → Fin m)
  → PerfectBuckets S f
  → HonoraryZero S (MS-fromResid f)
honoraryZeroFromPermutation S f pb =
  SymmetryImpliesRepulsion S (MS-fromResid f) (pairingFromPerfect S f pb)

------------------------------------------------------------------------
-- USAGE NOTES
------------------------------------------------------------------------

{-
CONCRETE INSTANTIATION FOR 2p² WINDOWS:

1. Choose base m (e.g., m=14 for φ(14)=6)
2. Set midpoint: mid = fromℕ< (m div 2) proof
3. Build SymmetryData: S = mkSymReflect mid
4. Extract residues: f : Fin n → Fin m
5. Construct PerfectBuckets witness
6. Get HonoraryZero automatically!

EXAMPLE (Base 14):
  m = 14, mid = 7
  S = mkSymReflect (fromℕ< 7 proof)
  f : Fin 6 → Fin 14  -- Six coprime residues
  f 0 = 1, f 1 = 3, f 2 = 5, f 3 = 9, f 4 = 11, f 5 = 13

  Verify: reflect 7 1 = 13 ✓
          reflect 7 3 = 11 ✓
          reflect 7 5 = 9  ✓
          fixed residues are 0 and 7 ✓

  In the canonical even-base half-turn cases, the certification lane separates
  into:
    - constructive arithmetic classifier: any fixed residue is 0 or mid
    - observed support exclusion: no observed residue equals 0 or mid

  The generic wrappers now consume an explicit observed fixed-point classifier.
  The canonical even-base helper here builds that classifier constructively.

This is the concrete arithmetic that makes the abstract framework
work for real coordinate constellation data!
-}
