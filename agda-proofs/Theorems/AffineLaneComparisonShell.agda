{-# OPTIONS --without-K #-}
------------------------------------------------------------------------
-- Lean-led mirror shell for local affine lane comparison
--
-- Purpose:
-- 1. preserve the affine local-profile vocabulary used by the Rust atlas
-- 2. keep shift / gradient / zero-seed terminology stable in Agda
-- 3. record the intended local relation labels without claiming proof parity
--
-- The theorem engine for this lane currently lives in Lean 4. This Agda file
-- is a concept mirror rather than a parity-forcing proof target.
------------------------------------------------------------------------

module Theorems.AffineLaneComparisonShell where

open import Data.Bool using (Bool; true; false)
open import Data.Nat using (ℕ)
open import Data.Nat.Base using (NonZero)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

open import Theorems.AffineTransform using
  ( AffineConfig
  ; affine-gradient
  ; affine-shift
  ; base6-15
  ; base10-37
  )

record LocalAffineProfile : Set where
  field
    modulus : ℕ
    shift : ℕ
    gradient : ℕ
    zero-seed-class : ℕ

mk-local-profile :
  (conf : AffineConfig) ->
  (modulus zeroSeed : ℕ) .{{_ : NonZero modulus}} ->
  LocalAffineProfile
mk-local-profile conf modulus zeroSeed = record
  { modulus = modulus
  ; shift = affine-shift conf modulus
  ; gradient = affine-gradient conf modulus
  ; zero-seed-class = zeroSeed
  }

data LocalRelationLabel : Set where
  identity : LocalRelationLabel
  shift-only : LocalRelationLabel
  gradient-only : LocalRelationLabel
  shift-and-gradient : LocalRelationLabel

local-relation : Bool -> Bool -> LocalRelationLabel
local-relation true true = identity
local-relation true false = shift-only
local-relation false true = gradient-only
local-relation false false = shift-and-gradient

base6-mod7-profile : LocalAffineProfile
base6-mod7-profile = mk-local-profile base6-15 7 1

base10-mod11-profile : LocalAffineProfile
base10-mod11-profile = mk-local-profile base10-37 11 2

base6-mod7-shift : LocalAffineProfile.shift base6-mod7-profile ≡ 6
base6-mod7-shift = refl

base6-mod7-gradient : LocalAffineProfile.gradient base6-mod7-profile ≡ 1
base6-mod7-gradient = refl

base6-mod7-zero-seed : LocalAffineProfile.zero-seed-class base6-mod7-profile ≡ 1
base6-mod7-zero-seed = refl

base10-mod11-shift : LocalAffineProfile.shift base10-mod11-profile ≡ 9
base10-mod11-shift = refl

base10-mod11-gradient : LocalAffineProfile.gradient base10-mod11-profile ≡ 1
base10-mod11-gradient = refl

base10-mod11-zero-seed : LocalAffineProfile.zero-seed-class base10-mod11-profile ≡ 2
base10-mod11-zero-seed = refl

identity-example : local-relation true true ≡ identity
identity-example = refl

shift-only-example : local-relation true false ≡ shift-only
shift-only-example = refl

gradient-only-example : local-relation false true ≡ gradient-only
gradient-only-example = refl

shift-and-gradient-example : local-relation false false ≡ shift-and-gradient
shift-and-gradient-example = refl
