------------------------------------------------------------------------
-- Coprimality Requirement Shell
--
-- This module preserves the core coprimality claim in a compilable shell:
--
-- 1. top-performing membrane configurations use boundary digits coprime to the
--    base
-- 2. non-coprime boundaries are the intended mechanism for forced
--    divisibility/composite leakage
-- 3. the full proof bridge from membrane structure to density dominance is
--    still open here
------------------------------------------------------------------------

module Theorems.CoprimalityRequirement where

open import Data.Bool using (Bool; true; false)
open import Data.List using (List; []; _∷_)
open import Data.Nat using (ℕ)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

record Base : Set where
  constructor mkBase
  field
    value : ℕ

record MembraneConfig (b : Base) : Set where
  constructor cfg
  field
    outer : ℕ
    inner : ℕ
    k₁ : ℕ
    k₂ : ℕ

record BoundaryCoprimality : Set where
  constructor coprimality
  field
    outer-coprime : Bool
    inner-coprime : Bool

example-base6 : Base
example-base6 = mkBase 6

example-base10 : Base
example-base10 = mkBase 10

example-config-15 : MembraneConfig example-base6
example-config-15 = cfg 1 5 0 0

example-config-24 : MembraneConfig example-base10
example-config-24 = cfg 2 4 0 0

config-15-coprimality : BoundaryCoprimality
config-15-coprimality = coprimality true true

config-24-coprimality : BoundaryCoprimality
config-24-coprimality = coprimality false false

postulate
  membrane : (b : Base) → MembraneConfig b → ℕ → ℕ
  IsPrime : ℕ → Set
  success-rate : (b : Base) → MembraneConfig b → List ℕ → ℕ
  non-coprime-outer-forces-divisibility : (b : Base) → MembraneConfig b → Set
  non-coprime-inner-forces-divisibility : (b : Base) → MembraneConfig b → Set
  non-coprime-generates-composites : (b : Base) → MembraneConfig b → Set
  coprime-better-density : (b : Base) → List ℕ → Set
  top-performing-boundaries-are-coprime : Set
  verify-15-coprime : Set
  verify-24-non-coprime : Set
  verify-24-all-composite : Set
