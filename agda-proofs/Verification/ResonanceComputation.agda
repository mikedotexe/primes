------------------------------------------------------------------------
-- Resonance-computation shell: reported space-size yield oscillations
--
-- Strongest live signal:
-- 1. the repo has a narrow reported oscillation story for the `(7, 11)` pair
--    as space size varies
-- 2. the key claim is empirical and local: sizes 3 and 11 are reported peaks,
--    with non-monotonic behavior around size 3
-- 3. the open gap is the executable search backend, not the ability to record
--    and compare the reported yields honestly
------------------------------------------------------------------------

module Verification.ResonanceComputation where

open import Data.List using (List; []; _∷_)
open import Data.Nat using (ℕ)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

------------------------------------------------------------------------
-- Reported yield data
------------------------------------------------------------------------

record ResonancePoint : Set where
  field
    space-size : ℕ
    reported-yield : ℕ

yield-1 : ResonancePoint
yield-1 = record { space-size = 1 ; reported-yield = 2 }

yield-2 : ResonancePoint
yield-2 = record { space-size = 2 ; reported-yield = 3 }

yield-3 : ResonancePoint
yield-3 = record { space-size = 3 ; reported-yield = 8 }

yield-11 : ResonancePoint
yield-11 = record { space-size = 11 ; reported-yield = 9 }

reported-yields : List ResonancePoint
reported-yields = yield-1 ∷ yield-2 ∷ yield-3 ∷ yield-11 ∷ []

yield-3-check : ResonancePoint.reported-yield yield-3 ≡ 8
yield-3-check = refl

yield-11-check : ResonancePoint.reported-yield yield-11 ≡ 9
yield-11-check = refl

------------------------------------------------------------------------
-- Oscillation shell
------------------------------------------------------------------------

record OscillationShell : Set where
  field
    left-size : ℕ
    peak-size : ℕ
    right-size : ℕ
    left-yield : ℕ
    peak-yield : ℕ
    right-yield : ℕ

local-peak-at-3 : OscillationShell
local-peak-at-3 = record
  { left-size = 2
  ; peak-size = 3
  ; right-size = 4
  ; left-yield = 3
  ; peak-yield = 8
  ; right-yield = 0
  }

record ResonanceCaseShell : Set where
  field
    left-body : ℕ
    right-body : ℕ
    reported-points : List ResonancePoint
    primary-oscillation : OscillationShell
    larger-peak-size : ℕ
    larger-peak-yield : ℕ

pair-7-11-resonance : ResonanceCaseShell
pair-7-11-resonance = record
  { left-body = 7
  ; right-body = 11
  ; reported-points = reported-yields
  ; primary-oscillation = local-peak-at-3
  ; larger-peak-size = 11
  ; larger-peak-yield = 9
  }

------------------------------------------------------------------------
-- Open computation bridge
------------------------------------------------------------------------

record ResonanceTheoryShell : Set1 where
  field
    concatenate-shape : Set
    primality-backend-shape : Set
    yield-computation-shape : Set
    local-maxima-shape : Set

postulate
  concatenate : ℕ -> ℕ -> ℕ -> ℕ -> ℕ -> ℕ
  isPrimeFast : ℕ -> Set
  computeYield : ℕ -> ℕ -> ℕ -> Set
  localMaximum : ResonanceCaseShell -> Set
  resonance-theory : ResonanceTheoryShell
