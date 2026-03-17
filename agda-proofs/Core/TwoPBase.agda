{-# OPTIONS --without-K #-}
------------------------------------------------------------------------
-- TwoP-base shell: bases of the form 2p
--
-- Strongest live signal:
-- 1. bases of the form 2p remain the main phase-lock examples across the repo
-- 2. the concrete residue sets for 6, 10, and 14 are worth keeping live
-- 3. the general radical/totient/framework bridge should stay explicit here
--    until the lower residue machinery is pulled through constructively
------------------------------------------------------------------------

module Core.TwoPBase where

open import Data.Bool using (Bool; true; false)
open import Data.List using (List; []; _∷_)
open import Data.Nat using (ℕ)

------------------------------------------------------------------------
-- Core shell
------------------------------------------------------------------------

record TwoPBaseShell : Set where
  field
    p : ℕ
    even-base : ℕ

base6 : TwoPBaseShell
base6 = record
  { p = 3
  ; even-base = 6
  }

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

------------------------------------------------------------------------
-- Concrete residue shell
------------------------------------------------------------------------

valid-residues-6 : List ℕ
valid-residues-6 = 1 ∷ 5 ∷ []

valid-residues-10 : List ℕ
valid-residues-10 = 1 ∷ 3 ∷ 7 ∷ 9 ∷ []

valid-residues-14 : List ℕ
valid-residues-14 = 1 ∷ 3 ∷ 5 ∷ 9 ∷ 11 ∷ 13 ∷ []

record ResidueCountShell : Set where
  field
    base : TwoPBaseShell
    valid-residues : List ℕ
    totient-count : ℕ

base6-residue-count : ResidueCountShell
base6-residue-count = record
  { base = base6
  ; valid-residues = valid-residues-6
  ; totient-count = 2
  }

base10-residue-count : ResidueCountShell
base10-residue-count = record
  { base = base10
  ; valid-residues = valid-residues-10
  ; totient-count = 4
  }

base14-residue-count : ResidueCountShell
base14-residue-count = record
  { base = base14
  ; valid-residues = valid-residues-14
  ; totient-count = 6
  }

------------------------------------------------------------------------
-- Distance shell
------------------------------------------------------------------------

record DistanceShell : Set where
  field
    base : TwoPBaseShell
    distance : ℕ
    valid-distance : Bool

distance-1-base6 : DistanceShell
distance-1-base6 = record
  { base = base6
  ; distance = 1
  ; valid-distance = true
  }

distance-3-base10 : DistanceShell
distance-3-base10 = record
  { base = base10
  ; distance = 3
  ; valid-distance = true
  }

distance-4-base10 : DistanceShell
distance-4-base10 = record
  { base = base10
  ; distance = 4
  ; valid-distance = false
  }

------------------------------------------------------------------------
-- Open theorem shell
------------------------------------------------------------------------

record TwoPTheoryShell : Set1 where
  field
    radical-shape : Set
    totient-shape : Set
    framework-shape : Set

postulate
  rad-2p : TwoPBaseShell -> Set
  divisors-2p : TwoPBaseShell -> Set
  totient-2p : TwoPBaseShell -> Set
  twoPBaseFramework : TwoPBaseShell -> Set
  twoPBaseFilter : TwoPBaseShell -> Set
  coprime-to-2p : TwoPBaseShell -> ℕ -> Bool
  valid-distance : TwoPBaseShell -> ℕ -> Bool
  twoP-theory : TwoPTheoryShell
