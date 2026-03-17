------------------------------------------------------------------------
-- Lagrange residue-field shell: small-prime screening for the canonical pair
--
-- Strongest live signal:
-- 1. the residue-side story can already be stated honestly as a small-prime
--    screening lane for the canonical connector pair
-- 2. the two reported insertion hits remain the clearest residue-compatible
--    points, while the other buffer positions stay open rather than silently
--    assumed solved
-- 3. the hard part is the general CRT/search/primality bridge, not the basic
--    shape of the residue-screen interface
------------------------------------------------------------------------

module LagrangePoints.ResidueField where

open import Data.Bool using (Bool; true; false)
open import Data.List using (List; []; _∷_)
open import Data.Maybe.Base using (Maybe; just; nothing)
open import Data.Nat using (ℕ)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

open import Core.LagrangePoints using
  ( ConcatenatedStructureShell
  ; LagrangePointShell
  ; canonical-example
  ; canonical-L1
  ; canonical-L2
  ; canonical-point-count
  )

------------------------------------------------------------------------
-- Small-prime screen
------------------------------------------------------------------------

small-primes : List ℕ
small-primes =
  2 ∷ 3 ∷ 5 ∷ 7 ∷ 11 ∷ 13 ∷ 17 ∷ 19 ∷ 23 ∷ 29 ∷
  31 ∷ 37 ∷ 41 ∷ 43 ∷ 47 ∷ 53 ∷ 59 ∷ 61 ∷ 67 ∷ 71 ∷
  73 ∷ 79 ∷ 83 ∷ 89 ∷ 97 ∷ []

small-prime-count : ℕ
small-prime-count = 25

small-prime-count-check : small-prime-count ≡ 25
small-prime-count-check = refl

record ResiduePositionShell : Set where
  field
    position : ℕ
    candidate-digit : Maybe ℕ
    checked-primes : List ℕ
    search-complete : Bool
    equilibrium-reported : Bool

canonical-pos0-screen : ResiduePositionShell
canonical-pos0-screen = record
  { position = 0
  ; candidate-digit = nothing
  ; checked-primes = small-primes
  ; search-complete = false
  ; equilibrium-reported = false
  }

canonical-pos1-screen : ResiduePositionShell
canonical-pos1-screen = record
  { position = 1
  ; candidate-digit = just 6
  ; checked-primes = small-primes
  ; search-complete = true
  ; equilibrium-reported = true
  }

canonical-pos2-screen : ResiduePositionShell
canonical-pos2-screen = record
  { position = 2
  ; candidate-digit = nothing
  ; checked-primes = small-primes
  ; search-complete = false
  ; equilibrium-reported = false
  }

canonical-pos3-screen : ResiduePositionShell
canonical-pos3-screen = record
  { position = 3
  ; candidate-digit = nothing
  ; checked-primes = small-primes
  ; search-complete = false
  ; equilibrium-reported = false
  }

canonical-pos4-screen : ResiduePositionShell
canonical-pos4-screen = record
  { position = 4
  ; candidate-digit = just 6
  ; checked-primes = small-primes
  ; search-complete = true
  ; equilibrium-reported = true
  }

all-canonical-screens : List ResiduePositionShell
all-canonical-screens =
  canonical-pos0-screen ∷
  canonical-pos1-screen ∷
  canonical-pos2-screen ∷
  canonical-pos3-screen ∷
  canonical-pos4-screen ∷
  []

------------------------------------------------------------------------
-- Canonical residue case
------------------------------------------------------------------------

record ResidueFieldCaseShell : Set where
  field
    structure : ConcatenatedStructureShell
    reported-hit-count : ℕ
    reported-L1 : LagrangePointShell
    reported-L2 : LagrangePointShell
    screen : List ResiduePositionShell
    center-position : Maybe ℕ
    residue-mechanism-ready : Bool

canonical-residue-case : ResidueFieldCaseShell
canonical-residue-case = record
  { structure = canonical-example
  ; reported-hit-count = canonical-point-count
  ; reported-L1 = canonical-L1
  ; reported-L2 = canonical-L2
  ; screen = all-canonical-screens
  ; center-position = just 2
  ; residue-mechanism-ready = true
  }

canonical-hit-count-check : canonical-point-count ≡ 2
canonical-hit-count-check = refl

canonical-L1-position : LagrangePointShell.position canonical-L1 ≡ 1
canonical-L1-position = refl

canonical-L2-position : LagrangePointShell.position canonical-L2 ≡ 4
canonical-L2-position = refl

------------------------------------------------------------------------
-- Open residue bridge
------------------------------------------------------------------------

record ResidueTheoryShell : Set1 where
  field
    residue-vector-shape : Set
    crt-shape : Set
    equilibrium-search-shape : Set
    primality-bridge-shape : Set

postulate
  residueVector : ConcatenatedStructureShell -> ℕ -> ℕ -> Set
  residueCompatible : ConcatenatedStructureShell -> ℕ -> ℕ -> Set
  findEquilibriumDigit : ConcatenatedStructureShell -> ℕ -> Set
  scanAllPositions : ConcatenatedStructureShell -> Set
  lagrangeExistence : ConcatenatedStructureShell -> Set
  residue-theory : ResidueTheoryShell
