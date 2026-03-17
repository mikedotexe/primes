{-# OPTIONS --without-K #-}
------------------------------------------------------------------------
-- Phase-lock shell: symmetric prime pairs in 2p bases
--
-- Strongest live signal:
-- 1. phase locks are the repo's core symmetric-pair vocabulary for bases 2p
-- 2. the concrete relation to Goldbach pairs is real at the shell level even
--    though the general restricted-Goldbach bridge remains open
-- 3. the midpoint/distance structure is the most stable part of the story and
--    is worth keeping live for downstream experimentation
------------------------------------------------------------------------

module Core.PhaseLocks where

open import Data.Nat using (ℕ)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Core.Spectral using (SpectralTag; qr; nqr)
open import Core.TwoPBase using (TwoPBaseShell)

------------------------------------------------------------------------
-- Base and midpoint shell
------------------------------------------------------------------------

base10 : TwoPBaseShell
base10 = record
  { p = 5
  ; even-base = 10
  }

base14 : TwoPBaseShell
base14 = record
  { p = 7
  ; even-base = 14
  }

base22 : TwoPBaseShell
base22 = record
  { p = 11
  ; even-base = 22
  }

midpoint : TwoPBaseShell -> ℕ
midpoint = TwoPBaseShell.p

------------------------------------------------------------------------
-- Goldbach and phase-lock shells
------------------------------------------------------------------------

record GoldbachPairShell : Set where
  field
    left : ℕ
    right : ℕ
    sum : ℕ

record PhaseLockShell : Set where
  field
    left : ℕ
    right : ℕ
    sum : ℕ
    distance : ℕ

phaseLock->Goldbach : PhaseLockShell -> GoldbachPairShell
phaseLock->Goldbach pl = record
  { left = PhaseLockShell.left pl
  ; right = PhaseLockShell.right pl
  ; sum = PhaseLockShell.sum pl
  }

goldbach->PhaseLock : GoldbachPairShell -> ℕ -> PhaseLockShell
goldbach->PhaseLock gb d = record
  { left = GoldbachPairShell.left gb
  ; right = GoldbachPairShell.right gb
  ; sum = GoldbachPairShell.sum gb
  ; distance = d
  }

record BridgeShell : Set1 where
  field
    forward-shape : Set
    backward-shape : Set

phase-lock-goldbach-bridge : BridgeShell
phase-lock-goldbach-bridge = record
  { forward-shape = GoldbachPairShell
  ; backward-shape = PhaseLockShell
  }

------------------------------------------------------------------------
-- Concrete shell examples
------------------------------------------------------------------------

base10-goldbach : GoldbachPairShell
base10-goldbach = record
  { left = 3
  ; right = 7
  ; sum = 10
  }

base10-phase-lock : PhaseLockShell
base10-phase-lock = record
  { left = 3
  ; right = 7
  ; sum = 10
  ; distance = 2
  }

base14-goldbach : GoldbachPairShell
base14-goldbach = record
  { left = 3
  ; right = 11
  ; sum = 14
  }

base14-phase-lock : PhaseLockShell
base14-phase-lock = record
  { left = 3
  ; right = 11
  ; sum = 14
  ; distance = 4
  }

base22-goldbach : GoldbachPairShell
base22-goldbach = record
  { left = 5
  ; right = 17
  ; sum = 22
  }

base22-phase-lock : PhaseLockShell
base22-phase-lock = record
  { left = 5
  ; right = 17
  ; sum = 22
  ; distance = 6
  }

base10-bridge-check : GoldbachPairShell.sum (phaseLock->Goldbach base10-phase-lock) ≡ 10
base10-bridge-check = refl

base22-bridge-check : GoldbachPairShell.sum (phaseLock->Goldbach base22-phase-lock) ≡ 22
base22-bridge-check = refl

------------------------------------------------------------------------
-- Spectral tagging shell
------------------------------------------------------------------------

record PhaseLockDistanceShell : Set where
  field
    base : TwoPBaseShell
    distance : ℕ
    spectral-tag : SpectralTag

base10-distance-shell : PhaseLockDistanceShell
base10-distance-shell = record
  { base = base10
  ; distance = 2
  ; spectral-tag = nqr
  }

base22-distance-shell : PhaseLockDistanceShell
base22-distance-shell = record
  { base = base22
  ; distance = 6
  ; spectral-tag = qr
  }

------------------------------------------------------------------------
-- Open theorem shell
------------------------------------------------------------------------

record PhaseLockTheoryShell : Set1 where
  field
    restricted-goldbach-shape : Set
    spectral-shape : Set
    density-shape : Set
    residue-framework-shape : Set

postulate
  PhaseLock->Goldbach : TwoPBaseShell -> Set
  Goldbach->PhaseLock : TwoPBaseShell -> Set
  phaseLockExists : TwoPBaseShell -> Set
  phaseLockRespectsFramework : TwoPBaseShell -> Set
  spectralConstraint : TwoPBaseShell -> Set
  phaseLockDensity : TwoPBaseShell -> Set
  phase-lock-theory : PhaseLockTheoryShell
