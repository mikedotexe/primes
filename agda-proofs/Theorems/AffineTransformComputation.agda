{-# OPTIONS --safe --without-K #-}
------------------------------------------------------------------------
-- Affine transform computation shell
--
-- This module keeps the concrete computation side of the affine idea live
-- without overstating it as a finished general proof. The clean signal here is:
--
-- 1. small explicit membrane families can be computed directly
-- 2. the affine residue form agrees with those direct computations in the
--    maintained base-6 examples
-- 3. larger base-10 examples can be recorded as reported observations while the
--    theorem layer remains open in Theorems.AffineTransform
------------------------------------------------------------------------

module Theorems.AffineTransformComputation where

open import Data.List using (List; []; _∷_)
open import Data.Nat using (ℕ; _+_; _*_; _^_; z≤n; s≤s)
open import Data.Nat.Base using (NonZero; >-nonZero)
open import Data.Nat.DivMod using (_mod_)
open import Data.Fin.Base using (toℕ)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

------------------------------------------------------------------------
-- Local computational model
------------------------------------------------------------------------

record AffineConfig : Set where
  field
    base : ℕ
    outer : ℕ
    inner : ℕ
    k1 : ℕ
    k2 : ℕ

seed-position : AffineConfig -> ℕ
seed-position conf = AffineConfig.k1 conf + AffineConfig.k2 conf + 2

outer-left-position : AffineConfig -> ℕ
outer-left-position conf = 2 * seed-position conf

inner-left-position : AffineConfig -> ℕ
inner-left-position conf = seed-position conf + AffineConfig.k2 conf + 1

inner-right-position : AffineConfig -> ℕ
inner-right-position conf = AffineConfig.k2 conf + 1

membrane : AffineConfig -> ℕ -> ℕ
membrane conf seed =
  let b = AffineConfig.base conf
      o = AffineConfig.outer conf
      i = AffineConfig.inner conf
  in o * (b ^ outer-left-position conf)
   + i * (b ^ inner-left-position conf)
   + seed * (b ^ seed-position conf)
   + i * (b ^ inner-right-position conf)
   + o

modNat : (n modulus : ℕ) .{{_ : NonZero modulus}} -> ℕ
modNat n modulus = toℕ (n mod modulus)

affine-shift : (conf : AffineConfig) -> (modulus : ℕ) .{{_ : NonZero modulus}} -> ℕ
affine-shift conf modulus = modNat (membrane conf 0) modulus

affine-gradient : (conf : AffineConfig) -> (modulus : ℕ) .{{_ : NonZero modulus}} -> ℕ
affine-gradient conf modulus =
  modNat (AffineConfig.base conf ^ seed-position conf) modulus

affine-eval : (conf : AffineConfig) -> (seed modulus : ℕ) .{{_ : NonZero modulus}} -> ℕ
affine-eval conf seed modulus =
  let shift = affine-shift conf modulus
      gradient = affine-gradient conf modulus
  in modNat (shift + gradient * seed) modulus

instance
  nz7 : NonZero 7
  nz7 = >-nonZero (s≤s z≤n)

  nz11 : NonZero 11
  nz11 = >-nonZero (s≤s z≤n)

------------------------------------------------------------------------
-- Maintained base-6 regression surface
------------------------------------------------------------------------

base6-15 : AffineConfig
base6-15 = record
  { base = 6
  ; outer = 1
  ; inner = 5
  ; k1 = 0
  ; k2 = 0
  }

base6-seed0-mod7 :
  modNat (membrane base6-15 0) 7 ≡ affine-eval base6-15 0 7
base6-seed0-mod7 = refl

base6-seed1-mod7 :
  modNat (membrane base6-15 1) 7 ≡ affine-eval base6-15 1 7
base6-seed1-mod7 = refl

base6-seed5-mod7 :
  modNat (membrane base6-15 5) 7 ≡ affine-eval base6-15 5 7
base6-seed5-mod7 = refl

base6-seed0-mod11 :
  modNat (membrane base6-15 0) 11 ≡ affine-eval base6-15 0 11
base6-seed0-mod11 = refl

base6-seed5-mod11 :
  modNat (membrane base6-15 5) 11 ≡ affine-eval base6-15 5 11
base6-seed5-mod11 = refl

record ComputedCheck : Set where
  constructor mkComputedCheck
  field
    seed : ℕ
    modulus : ℕ
    direct-residue : ℕ
    affine-residue : ℕ

base6-checks : List ComputedCheck
base6-checks =
  mkComputedCheck 0 7 6 6
  ∷ mkComputedCheck 1 7 0 0
  ∷ mkComputedCheck 5 7 4 4
  ∷ mkComputedCheck 0 11 9 9
  ∷ mkComputedCheck 5 11 2 2
  ∷ []

------------------------------------------------------------------------
-- Reported larger observations
------------------------------------------------------------------------

base10-37 : AffineConfig
base10-37 = record
  { base = 10
  ; outer = 3
  ; inner = 7
  ; k1 = 1
  ; k2 = 1
  }

record ReportedObservation : Set where
  constructor mkReportedObservation
  field
    seed : ℕ
    modulus : ℕ
    reported-shift : ℕ
    reported-gradient : ℕ
    reported-direct-residue : ℕ
    reported-affine-residue : ℕ

base10-observations : List ReportedObservation
base10-observations =
  mkReportedObservation 0 11 9 1 9 9
  ∷ mkReportedObservation 5 11 9 1 3 3
  ∷ mkReportedObservation 0 13 9 3 9 9
  ∷ mkReportedObservation 5 13 9 3 11 11
  ∷ []
