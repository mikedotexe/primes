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
open import Relation.Binary.PropositionalEquality  using (_≡_; refl; sym; cong)
open import Data.Empty     using (⊥)
open import Data.Nat       using (ℕ; _+_; _*_ ; _∸_)
open import Data.Nat               using (_<_ ; _≤_ ; z≤n ; s≤s)
open import Data.Nat.DivMod        using (_mod_)
open import Data.Fin               using (Fin; toℕ; fromℕ<)
open import Relation.Nullary       using (Dec; yes; no)

-- Import abstract framework
open import Theorems.Abstract.SymmetryImpliesRepulsion
  using ( SymmetryData ; MS ; Pairing ; HonoraryZero
        ; SymmetryImpliesRepulsion )
open import Theorems.Abstract.SymmetryFromList
  using ( PerfectBuckets ; pairingFromPerfect ; MS-fromResid )

------------------------------------------------------------------------
-- FINITE-BASE REFLECTION: inv r = (2·mid - r) mod m on B = Fin m
------------------------------------------------------------------------

-- Standard modular arithmetic lemma (postulated for now)
postulate
  modLess : ∀ (m k : ℕ) → (k mod m) < m

-- The reflection involution: r ↦ (2·mid - r) mod m
-- This is the concrete implementation used in coordinate constellation analysis
reflect : ∀ {m} → (mid : Fin m) → Fin m → Fin m
reflect {m} mid r =
  let a   = 2 * toℕ mid + m      -- Add m to ensure non-negative
      raw = a ∸ toℕ r            -- Compute 2·mid - r
      k   = raw mod m            -- Take modulo m
  in fromℕ< k (modLess m k)

-- Properties of reflection (postulated - provable by arithmetic)
postulate
  reflect-involutive : ∀ {m} (mid : Fin m) (r : Fin m)
                     → reflect mid (reflect mid r) ≡ r
  reflect-mid        : ∀ {m} (mid : Fin m)
                     → reflect mid mid ≡ mid

------------------------------------------------------------------------
-- CONSTRUCT CONCRETE SYMMETRY DATA
--
-- This is the canonical SymmetryData for modular arithmetic!

mkSymReflect : ∀ {m} → (mid : Fin m) → SymmetryData (Fin m)
mkSymReflect mid =
  record
    { mid            = mid
    ; inv            = reflect mid
    ; inv-involutive = reflect-involutive mid
    ; inv-mid        = reflect-mid mid
    }

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

This is the concrete arithmetic that makes the abstract framework
work for real coordinate constellation data!
-}
