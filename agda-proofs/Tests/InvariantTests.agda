------------------------------------------------------------------------
-- Invariant test shell: stable-orbital regression cases
--
-- Strongest live signal:
-- 1. the repo still benefits from a lightweight regression surface for the
--    "avoid the midpoint / stay outside a radius" intuition
-- 2. the stable-orbital examples for bases 7, 14, and 18 are worth keeping as
--    executable checks even without the older hand-written proof script
-- 3. this module is a test shell, not a replacement for the abstract theorem
--    stack in `Theorems/Abstract/*`
------------------------------------------------------------------------

module Tests.InvariantTests where

open import Data.Bool using (Bool; true; false)
open import Data.List using (List; []; _∷_)
open import Data.Nat using (ℕ; zero; suc; _+_; _∸_; _≤?_)
open import Data.Empty using (⊥)
open import Relation.Nullary using (yes; no)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Theorems.Abstract.ConstrainedOrbitals as C
  using ()

------------------------------------------------------------------------
-- Small executable helpers
------------------------------------------------------------------------

absDiff : ℕ -> ℕ -> ℕ
absDiff a b = (a ∸ b) + (b ∸ a)

isSafePos : ℕ -> ℕ -> ℕ -> Bool
isSafePos radius mid x with radius ≤? absDiff x mid
... | yes _ = true
... | no _ = false

allSafe : ℕ -> ℕ -> List ℕ -> Bool
allSafe radius mid [] = true
allSafe radius mid (x ∷ xs) with isSafePos radius mid x
... | true = allSafe radius mid xs
... | false = false

------------------------------------------------------------------------
-- Test-case shell
------------------------------------------------------------------------

record InvariantCase : Set where
  field
    base : ℕ
    midpoint : ℕ
    radius : ℕ
    residues : List ℕ
    expected-safe : Bool

base7-pair-16 : InvariantCase
base7-pair-16 = record
  { base = 7
  ; midpoint = 3
  ; radius = 2
  ; residues = 1 ∷ 6 ∷ []
  ; expected-safe = true
  }

base7-avoid-mid : InvariantCase
base7-avoid-mid = record
  { base = 7
  ; midpoint = 3
  ; radius = 1
  ; residues = 1 ∷ 2 ∷ 4 ∷ 5 ∷ 6 ∷ []
  ; expected-safe = true
  }

base14-all-coprime : InvariantCase
base14-all-coprime = record
  { base = 14
  ; midpoint = 7
  ; radius = 2
  ; residues = 1 ∷ 3 ∷ 5 ∷ 9 ∷ 11 ∷ 13 ∷ []
  ; expected-safe = true
  }

base18-all-coprime : InvariantCase
base18-all-coprime = record
  { base = 18
  ; midpoint = 9
  ; radius = 2
  ; residues = 1 ∷ 5 ∷ 7 ∷ 11 ∷ 13 ∷ 17 ∷ []
  ; expected-safe = true
  }

midpoint-violates : InvariantCase
midpoint-violates = record
  { base = 14
  ; midpoint = 7
  ; radius = 1
  ; residues = 7 ∷ []
  ; expected-safe = false
  }

------------------------------------------------------------------------
-- Regression checks
------------------------------------------------------------------------

case-holds : InvariantCase -> Bool
case-holds tc =
  allSafe
    (InvariantCase.radius tc)
    (InvariantCase.midpoint tc)
    (InvariantCase.residues tc)

base7-pair-16-check : case-holds base7-pair-16 ≡ true
base7-pair-16-check = refl

base7-avoid-mid-check : case-holds base7-avoid-mid ≡ true
base7-avoid-mid-check = refl

base14-all-coprime-check : case-holds base14-all-coprime ≡ true
base14-all-coprime-check = refl

base18-all-coprime-check : case-holds base18-all-coprime ≡ true
base18-all-coprime-check = refl

midpoint-violates-check : case-holds midpoint-violates ≡ false
midpoint-violates-check = refl

------------------------------------------------------------------------
-- Narrow bridge to the abstract dynamic contract
------------------------------------------------------------------------

base7-pair-16-orbit : C.List ℕ
base7-pair-16-orbit = 1 C.∷ 6 C.∷ C.[]

two≤two : C._≤_ 2 2
two≤two = C.s≤s (C.s≤s (C.z≤n 0))

two≤three : C._≤_ 2 3
two≤three = C.s≤s (C.s≤s (C.z≤n 1))

-- Regression guard: keep this witness on the maintained helper path
-- (`pointwiseSafeCons` / `pointwiseSafeNil`) so helper-name drift is caught
-- here as well as in the active theorem modules.
base7-pair-16-pointwise : C.PointwiseSafe 2 3 base7-pair-16-orbit
base7-pair-16-pointwise =
  C.pointwiseSafeCons two≤two
    (C.pointwiseSafeCons two≤three
      C.pointwiseSafeNil)

base7-pair-16-not-in-zone : C.InZone 2 3 base7-pair-16-orbit → ⊥
base7-pair-16-not-in-zone =
  C.inviolabilityFromPointwiseSafe base7-pair-16-pointwise

midpoint-violates-orbit : C.List ℕ
midpoint-violates-orbit = 7 C.∷ C.[]

zero<one : C._<_ 0 1
zero<one = C.s≤s (C.z≤n 0)

midpoint-violates-in-zone : C.InZone 1 7 midpoint-violates-orbit
midpoint-violates-in-zone = C.here zero<one

-- Negative regression guard: this branch stays helper-agnostic on purpose.
-- The point is that any claimed PointwiseSafe witness must collapse against
-- an explicit InZone counterexample, regardless of how that witness was built.
midpoint-violates-not-pointwise : C.PointwiseSafe 1 7 midpoint-violates-orbit → ⊥
midpoint-violates-not-pointwise safe =
  C.inviolabilityFromPointwiseSafe safe midpoint-violates-in-zone
