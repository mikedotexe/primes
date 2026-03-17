{-# OPTIONS --without-K #-}
------------------------------------------------------------------------
-- Spectral shell: quadratic characters and QR/NQR vocabulary
--
-- Strongest live signal:
-- 1. the phase-lock story repeatedly uses QR/NQR language and the `p mod 4`
--    split into two spectral families
-- 2. the concrete supplements at `-1` and `2` matter more here than a full
--    constructive Legendre-symbol development
-- 3. the primitive-root and Euler bridges remain open and should be exposed as
--    shell assumptions rather than as broken low-level proof scripts
------------------------------------------------------------------------

module Core.Spectral where

open import Data.Nat using (ℕ; zero; suc)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

------------------------------------------------------------------------
-- ±1 shell
------------------------------------------------------------------------

data ±1 : Set where
  +1# : ±1
  -1# : ±1

_⊗_ : ±1 -> ±1 -> ±1
+1# ⊗ x = x
-1# ⊗ +1# = -1#
-1# ⊗ -1# = +1#

_^#_ : ±1 -> ℕ -> ±1
x ^# zero = +1#
x ^# (suc n) = x ⊗ (x ^# n)

toExp : ±1 -> ℕ
toExp +1# = 0
toExp -1# = 1

------------------------------------------------------------------------
-- Spectral families
------------------------------------------------------------------------

data Epsilon : Set where
  ε+1 : Epsilon
  ε-1 : Epsilon

record QuadraticCharacterShell : Set where
  field
    prime : ℕ
    epsilon : Epsilon
    χ-minus-one : ±1
    χ-two : ±1

prime5-shell : QuadraticCharacterShell
prime5-shell = record
  { prime = 5
  ; epsilon = ε+1
  ; χ-minus-one = +1#
  ; χ-two = -1#
  }

prime7-shell : QuadraticCharacterShell
prime7-shell = record
  { prime = 7
  ; epsilon = ε-1
  ; χ-minus-one = -1#
  ; χ-two = +1#
  }

prime13-shell : QuadraticCharacterShell
prime13-shell = record
  { prime = 13
  ; epsilon = ε+1
  ; χ-minus-one = +1#
  ; χ-two = -1#
  }

classifyPrimeShell : QuadraticCharacterShell -> Epsilon
classifyPrimeShell = QuadraticCharacterShell.epsilon

prime5-classification : classifyPrimeShell prime5-shell ≡ ε+1
prime5-classification = refl

prime7-classification : classifyPrimeShell prime7-shell ≡ ε-1
prime7-classification = refl

------------------------------------------------------------------------
-- QR/NQR shell
------------------------------------------------------------------------

data SpectralTag : Set where
  qr : SpectralTag
  nqr : SpectralTag

record SpectralObservation : Set where
  field
    prime : ℕ
    distance : ℕ
    tag : SpectralTag

distance-1-at-5 : SpectralObservation
distance-1-at-5 = record
  { prime = 5
  ; distance = 1
  ; tag = qr
  }

distance-3-at-7 : SpectralObservation
distance-3-at-7 = record
  { prime = 7
  ; distance = 3
  ; tag = nqr
  }

------------------------------------------------------------------------
-- Primitive-root shell
------------------------------------------------------------------------

record PrimitiveRootShell : Set where
  field
    prime : ℕ
    generator : ℕ

primitive-root-5 : PrimitiveRootShell
primitive-root-5 = record
  { prime = 5
  ; generator = 2
  }

primitive-root-7 : PrimitiveRootShell
primitive-root-7 = record
  { prime = 7
  ; generator = 3
  }

------------------------------------------------------------------------
-- Open theorem shell
------------------------------------------------------------------------

record SpectralTheoryShell : Set1 where
  field
    legendre-shape : Set
    euler-shape : Set
    primitive-root-shape : Set
    phase-lock-shape : Set

postulate
  legendre : ℕ -> ℕ -> ±1
  legendreMinus1 : ℕ -> ±1
  IsQR : QuadraticCharacterShell -> ℕ -> Set
  IsNQR : QuadraticCharacterShell -> ℕ -> Set
  qr-or-nqr : QuadraticCharacterShell -> ℕ -> Set
  qr-iff-even-index : Set
  epsilon-chi-minus-one : QuadraticCharacterShell -> Set
  spectral-theory : SpectralTheoryShell
