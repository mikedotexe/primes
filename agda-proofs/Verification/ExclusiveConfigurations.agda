------------------------------------------------------------------------
-- Exclusive-configuration shell: deterministic single-seed case studies
--
-- Strongest live signal:
-- 1. the repo has at least one narrow case study where a single seed is
--    reported to be the only prime-producing choice in a fixed configuration
-- 2. that is interesting as a structured verification target, but it should
--    stay case-study scoped rather than being framed as a general theorem
-- 3. the open gap is the exhaustive search / uniqueness proof machinery, not
--    the ability to record and reason about the reported example
------------------------------------------------------------------------

module Verification.ExclusiveConfigurations where

open import Data.Bool using (Bool; true; false)
open import Data.List using (List; []; _∷_)
open import Data.Nat using (ℕ)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

------------------------------------------------------------------------
-- Configuration shell
------------------------------------------------------------------------

record ConfigShell : Set where
  field
    base : ℕ
    outer : ℕ
    inner : ℕ
    k₁ : ℕ
    k₂ : ℕ

base6-15 : ConfigShell
base6-15 = record
  { base = 6
  ; outer = 1
  ; inner = 5
  ; k₁ = 0
  ; k₂ = 0
  }

record SeedOutcome : Set where
  field
    seed : ℕ
    value : ℕ
    reported-prime : Bool

base6-15-outcomes : List SeedOutcome
base6-15-outcomes =
  record { seed = 0 ; value = 2407 ; reported-prime = false } ∷
  record { seed = 1 ; value = 2443 ; reported-prime = false } ∷
  record { seed = 2 ; value = 2479 ; reported-prime = false } ∷
  record { seed = 3 ; value = 2515 ; reported-prime = false } ∷
  record { seed = 4 ; value = 2551 ; reported-prime = true } ∷
  record { seed = 5 ; value = 2587 ; reported-prime = false } ∷
  []

------------------------------------------------------------------------
-- Exclusive-case shell
------------------------------------------------------------------------

record ExclusiveCaseShell : Set where
  field
    config : ConfigShell
    outcomes : List SeedOutcome
    unique-seed : ℕ
    unique-value : ℕ
    deterministic-prime : Bool

base6-15-exclusive : ExclusiveCaseShell
base6-15-exclusive = record
  { config = base6-15
  ; outcomes = base6-15-outcomes
  ; unique-seed = 4
  ; unique-value = 2551
  ; deterministic-prime = true
  }

base6-unique-seed-check : ExclusiveCaseShell.unique-seed base6-15-exclusive ≡ 4
base6-unique-seed-check = refl

base6-unique-value-check : ExclusiveCaseShell.unique-value base6-15-exclusive ≡ 2551
base6-unique-value-check = refl

------------------------------------------------------------------------
-- Open verification bridge
------------------------------------------------------------------------

record ExclusiveTheoryShell : Set1 where
  field
    membrane-evaluation-shape : Set
    primality-shape : Set
    uniqueness-shape : Set
    search-shape : Set

postulate
  membraneValue : ConfigShell -> ℕ -> ℕ
  isPrimeValue : ℕ -> Set
  uniquePrimeSeed : ExclusiveCaseShell -> Set
  deterministicGeneration : ExclusiveCaseShell -> Set
  searchExclusiveConfigs : Set
  exclusivity-theory : ExclusiveTheoryShell
