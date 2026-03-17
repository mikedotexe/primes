------------------------------------------------------------------------
-- Residue Symmetry Shell
--
-- This module preserves the symmetric-window residue story in a compilable
-- shell. The current live signal is:
--
-- 1. symmetric windows around a center are the intended residue-level analogue
--    of the abstract symmetry theorem
-- 2. the `2p²` window story is still meaningful as a named target
-- 3. the actual constructive instantiation proofs remain open here
------------------------------------------------------------------------

module Theorems.ResidueSymmetry where

open import Data.Nat using (ℕ)

record SymmetricWindowShell : Set where
  constructor window
  field
    base : ℕ
    center : ℕ
    width : ℕ

record ResidueSymmetryWitness : Set where
  constructor witness
  field
    window-data : SymmetricWindowShell
    midpoint-residue : ℕ

base14-window : SymmetricWindowShell
base14-window = window 14 98 7

base18-window : SymmetricWindowShell
base18-window = window 18 162 9

base14-residue-witness : ResidueSymmetryWitness
base14-residue-witness = witness base14-window 7

base18-residue-witness : ResidueSymmetryWitness
base18-residue-witness = witness base18-window 9

postulate
  symmetryDataModShell : SymmetricWindowShell → Set
  symmetricPairingWitness : SymmetricWindowShell → Set
  symmetricWindowHonoraryZero : SymmetricWindowShell → Set
  window2p²HonoraryZero : ℕ → Set
  base14-window-example : Set
