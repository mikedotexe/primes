{-# OPTIONS --without-K #-}
------------------------------------------------------------------------
-- Lean-led mirror shell for local affine period lock
--
-- Purpose:
-- 1. preserve the period-lock vocabulary used by the Rust atlas
-- 2. keep gradient-position / order / locked-vs-unlocked language stable
-- 3. mirror the locked and unlocked local relation split without claiming
--    proof parity with the Lean theorem surface
------------------------------------------------------------------------

module Theorems.AffinePeriodLockShell where

open import Data.Bool using (Bool; true; false)
open import Data.Nat using (ℕ; _+_; _∸_; _≡ᵇ_)
open import Data.Nat.Base using (NonZero)
open import Data.Nat.DivMod using (_%_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

open import Theorems.AffineLaneComparisonShell using
  ( LocalRelationLabel
  ; identity
  ; shift-only
  ; gradient-only
  ; shift-and-gradient
  )

gradient-position : ℕ -> ℕ -> ℕ
gradient-position k-outer k-inner = k-outer + k-inner + 2

position-delta : ℕ -> ℕ -> ℕ
position-delta from to = to ∸ from

period-locked : (leftPos rightPos order : ℕ) .{{_ : NonZero order}} -> Bool
period-locked leftPos rightPos order = (leftPos % order) ≡ᵇ (rightPos % order)

relation-from-lock-and-shift : Bool -> Bool -> LocalRelationLabel
relation-from-lock-and-shift true true = identity
relation-from-lock-and-shift true false = gradient-only
relation-from-lock-and-shift false true = shift-only
relation-from-lock-and-shift false false = shift-and-gradient

k00-position : ℕ
k00-position = gradient-position 0 0

k11-position : ℕ
k11-position = gradient-position 1 1

k22-position : ℕ
k22-position = gradient-position 2 2

base22-mod5-order : ℕ
base22-mod5-order = 4

base22-k00-k22-delta : position-delta k00-position k22-position ≡ 4
base22-k00-k22-delta = refl

base22-k00-k11-delta : position-delta k00-position k11-position ≡ 2
base22-k00-k11-delta = refl

base22-k00-k22-locked : period-locked k00-position k22-position 4 ≡ true
base22-k00-k22-locked = refl

base22-k00-k11-unlocked : period-locked k00-position k11-position 4 ≡ false
base22-k00-k11-unlocked = refl

locked-identity-example :
  relation-from-lock-and-shift true true ≡ identity
locked-identity-example = refl

locked-gradient-only-example :
  relation-from-lock-and-shift true false ≡ gradient-only
locked-gradient-only-example = refl

unlocked-shift-only-example :
  relation-from-lock-and-shift false true ≡ shift-only
unlocked-shift-only-example = refl

unlocked-shift-and-gradient-example :
  relation-from-lock-and-shift false false ≡ shift-and-gradient
unlocked-shift-and-gradient-example = refl
