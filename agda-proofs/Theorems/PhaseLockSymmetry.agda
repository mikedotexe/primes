------------------------------------------------------------------------
-- Phase Lock Symmetry Shell
--
-- This module keeps the concrete phase-lock symmetry story in a compilable
-- shell. The current live signal is:
--
-- 1. phase locks naturally come with a two-point left/right pairing
-- 2. that pairing is the intended concrete instance of the abstract symmetry ->
--    honorary-zero theorem
-- 3. the actual proof bridge to the abstract theorem is still open in this file
------------------------------------------------------------------------

module Theorems.PhaseLockSymmetry where

open import Data.Nat using (ℕ)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

record PhaseLockShell : Set where
  constructor lock
  field
    base : ℕ
    midpoint : ℕ
    left-residue : ℕ
    right-residue : ℕ

data LockPosition : Set where
  left-pos : LockPosition
  right-pos : LockPosition

phaseLockPairing : LockPosition → LockPosition
phaseLockPairing left-pos = right-pos
phaseLockPairing right-pos = left-pos

pairing-involutive : ∀ x → phaseLockPairing (phaseLockPairing x) ≡ x
pairing-involutive left-pos = refl
pairing-involutive right-pos = refl

base6-lock : PhaseLockShell
base6-lock = lock 6 3 1 5

base10-lock : PhaseLockShell
base10-lock = lock 10 5 3 7

record PhaseLockSymmetryWitness : Set where
  constructor witness
  field
    lock-data : PhaseLockShell
    left-right-pairing : LockPosition → LockPosition

base6-witness : PhaseLockSymmetryWitness
base6-witness = witness base6-lock phaseLockPairing

base10-witness : PhaseLockSymmetryWitness
base10-witness = witness base10-lock phaseLockPairing

postulate
  phaseLockPairingWitness : PhaseLockShell → Set
  phaseLockImpliesHonoraryZero : PhaseLockShell → Set
  base6-midpoint-void : Set
  base10-midpoint-void : Set
