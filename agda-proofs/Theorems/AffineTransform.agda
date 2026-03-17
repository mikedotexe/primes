{-# OPTIONS --without-K #-}
------------------------------------------------------------------------
-- Affine transform shell for membrane evaluation
--
-- Strongest live signal:
-- 1. the membrane polynomial is linear in the seed once the boundary digits
--    and zero-padding layout are fixed
-- 2. that suggests an affine residue form modulo p:
--      M(seed) mod p = (shift + gradient * seed) mod p
-- 3. the general modular proof is still open here, so this file keeps the
--    concrete structure and theorem interfaces in a compilable shell
------------------------------------------------------------------------

module Theorems.AffineTransform where

open import Data.Nat using (ℕ; _+_; _*_; _^_)
open import Data.Nat.Base using (NonZero)
open import Data.Nat.DivMod using (_mod_)
open import Data.Fin.Base using (toℕ)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

------------------------------------------------------------------------
-- Fixed membrane layout
------------------------------------------------------------------------

record AffineConfig : Set where
  field
    base : ℕ
    outer : ℕ
    inner : ℕ
    k1 : ℕ
    k2 : ℕ

width : AffineConfig -> ℕ
width conf =
  let k1 = AffineConfig.k1 conf
      k2 = AffineConfig.k2 conf
  in 2 * (k1 + k2 + 2) + 1

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

------------------------------------------------------------------------
-- Concrete shell values
------------------------------------------------------------------------

base6-15 : AffineConfig
base6-15 = record
  { base = 6
  ; outer = 1
  ; inner = 5
  ; k1 = 0
  ; k2 = 0
  }

base10-37 : AffineConfig
base10-37 = record
  { base = 10
  ; outer = 3
  ; inner = 7
  ; k1 = 1
  ; k2 = 1
  }

base6-width : width base6-15 ≡ 5
base6-width = refl

base10-width : width base10-37 ≡ 9
base10-width = refl

base6-m0 : membrane base6-15 0 ≡ 2407
base6-m0 = refl

base6-m5 : membrane base6-15 5 ≡ 2587
base6-m5 = refl

base10-m0 : membrane base10-37 0 ≡ 307000703
base10-m0 = refl

base10-m5 : membrane base10-37 5 ≡ 307050703
base10-m5 = refl

record AffineObservation : Set where
  field
    config : AffineConfig
    modulus : ℕ
    seed : ℕ
    reported-shift : ℕ
    reported-gradient : ℕ
    reported-direct-residue : ℕ
    reported-affine-residue : ℕ

base6-mod7-seed5 : AffineObservation
base6-mod7-seed5 = record
  { config = base6-15
  ; modulus = 7
  ; seed = 5
  ; reported-shift = 6
  ; reported-gradient = 1
  ; reported-direct-residue = 4
  ; reported-affine-residue = 4
  }

base10-mod11-seed5 : AffineObservation
base10-mod11-seed5 = record
  { config = base10-37
  ; modulus = 11
  ; seed = 5
  ; reported-shift = 9
  ; reported-gradient = 1
  ; reported-direct-residue = 3
  ; reported-affine-residue = 3
  }

base6-theorem-shell : Set1
base6-theorem-shell = Set

base10-theorem-shell : Set1
base10-theorem-shell = Set

------------------------------------------------------------------------
-- Open theorem layer
------------------------------------------------------------------------

postulate
  membrane-split :
    (conf : AffineConfig) ->
    (seed : ℕ) ->
    membrane conf seed
      ≡ membrane conf 0
      + seed * (AffineConfig.base conf ^ seed-position conf)

  affine-transform-theorem :
    (conf : AffineConfig) ->
    (seed modulus : ℕ) .{{_ : NonZero modulus}} ->
    modNat (membrane conf seed) modulus
      ≡ affine-eval conf seed modulus

  affine-speedup-shell :
    (conf : AffineConfig) ->
    Set
