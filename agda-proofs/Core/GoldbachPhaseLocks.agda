{-# OPTIONS --without-K #-}

{-|
  Goldbach / phase-lock bridge shell.

  Strongest live signal:
  1. the repo keeps treating symmetric prime pairs in base 2p as the same
     search object as Goldbach pairs for 2p
  2. that bridge is conceptually important even though the older core imports
     (`TwoPBase`, `PhaseLocks`, `Spectral`) are still unstable
  3. this file now preserves that vocabulary and a few concrete shell values
     without pretending the full bridge is already reconstructed here
-}

module Core.GoldbachPhaseLocks where

open import Data.List using (List; []; _∷_)
open import Data.Nat using (ℕ; _+_; _<_; _>_)
open import Data.Product using (_×_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

------------------------------------------------------------------------
-- Base and pair shells
------------------------------------------------------------------------

record TwoPBaseShell : Set where
  field
    prime-center : ℕ
    even-base : ℕ

base22 : TwoPBaseShell
base22 = record
  { prime-center = 11
  ; even-base = 22
  }

base26 : TwoPBaseShell
base26 = record
  { prime-center = 13
  ; even-base = 26
  }

record PhaseLockShell : Set where
  field
    left-prime : ℕ
    right-prime : ℕ
    base-sum : ℕ
    distance : ℕ

record GoldbachPairShell : Set where
  field
    left-prime : ℕ
    right-prime : ℕ
    even-sum : ℕ

phase-lock-22 : PhaseLockShell
phase-lock-22 = record
  { left-prime = 5
  ; right-prime = 17
  ; base-sum = 22
  ; distance = 6
  }

goldbach-22 : GoldbachPairShell
goldbach-22 = record
  { left-prime = 5
  ; right-prime = 17
  ; even-sum = 22
  }

phase-lock-26 : PhaseLockShell
phase-lock-26 = record
  { left-prime = 7
  ; right-prime = 19
  ; base-sum = 26
  ; distance = 6
  }

goldbach-26 : GoldbachPairShell
goldbach-26 = record
  { left-prime = 7
  ; right-prime = 19
  ; even-sum = 26
  }

------------------------------------------------------------------------
-- Spectral and distance shell
------------------------------------------------------------------------

data SpectralType : Set where
  qr-distance : SpectralType
  nqr-distance : SpectralType

record PhaseLockCounts : Set where
  field
    total : ℕ
    qr-count : ℕ
    nqr-count : ℕ
    count-sum : qr-count + nqr-count ≡ total

valid-phase-lock-distance : TwoPBaseShell -> ℕ -> Set
valid-phase-lock-distance shell d =
  (d < TwoPBaseShell.prime-center shell) ×
  (d > 0)

sample-distances-22 : List ℕ
sample-distances-22 = 1 ∷ 3 ∷ 5 ∷ 7 ∷ 9 ∷ []

sample-distances-26 : List ℕ
sample-distances-26 = 1 ∷ 3 ∷ 5 ∷ 7 ∷ 9 ∷ 11 ∷ []

------------------------------------------------------------------------
-- Open bridge shell
------------------------------------------------------------------------

postulate
  phaseLocksEquivalentGoldbach : TwoPBaseShell -> Set
  spectralType : PhaseLockShell -> SpectralType
  possibleDistances : TwoPBaseShell -> List ℕ
  goldbachFor2p : TwoPBaseShell -> Set
  goldbachEquivalence : TwoPBaseShell -> Set
  expectedSymmetry : TwoPBaseShell -> Set
  phaseLockResidue : TwoPBaseShell -> Set
