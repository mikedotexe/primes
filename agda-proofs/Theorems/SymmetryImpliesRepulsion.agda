------------------------------------------------------------------------
-- Symmetry Implies Repulsion Shell
--
-- This module keeps the non-abstract narrative wrapper for the static symmetry
-- story in a compilable form. The strongest live claim is:
--
-- 1. midpoint exclusion is the intended concrete reading of the abstract
--    `Theorems.Abstract.SymmetryImpliesRepulsion` theorem
-- 2. bases 14 and 18 are the canonical blocked-midpoint cases
-- 3. base 7 remains the key counterexample showing that the midpoint void is
--    tied to coprimality status, not a universal repulsion law
------------------------------------------------------------------------

module Theorems.SymmetryImpliesRepulsion where

open import Data.Bool using (Bool; true; false)
open import Data.Nat using (ℕ)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Theorems.Abstract.SymmetryImpliesRepulsion using ()

midpointResidue : ℕ → ℕ
midpointResidue 7 = 3
midpointResidue 14 = 7
midpointResidue 18 = 9
midpointResidue b = b

data MidpointStatus : Set where
  blocked-midpoint : MidpointStatus
  allowed-midpoint : MidpointStatus

record SymmetryWrapper : Set where
  constructor wrapper
  field
    base : ℕ
    midpoint : ℕ
    midpoint-coprime : Bool
    status : MidpointStatus

base7-wrapper : SymmetryWrapper
base7-wrapper = wrapper 7 3 true allowed-midpoint

base14-wrapper : SymmetryWrapper
base14-wrapper = wrapper 14 7 false blocked-midpoint

base18-wrapper : SymmetryWrapper
base18-wrapper = wrapper 18 9 false blocked-midpoint

base14-midpoint-not-coprime : SymmetryWrapper.midpoint-coprime base14-wrapper ≡ false
base14-midpoint-not-coprime = refl

base18-midpoint-not-coprime : SymmetryWrapper.midpoint-coprime base18-wrapper ≡ false
base18-midpoint-not-coprime = refl

base7-midpoint-coprime : SymmetryWrapper.midpoint-coprime base7-wrapper ≡ true
base7-midpoint-coprime = refl

postulate
  φConstraint : ℕ → Set
  midpoint-non-coprime-excluded : ℕ → Set
  base14-honorary-zero : Set
  base18-honorary-zero : Set
  base7-midpoint-allowed : Set
  abstract-instantiation-shell : ℕ → Set
