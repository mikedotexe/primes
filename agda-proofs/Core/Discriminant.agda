{-# OPTIONS --without-K #-}
------------------------------------------------------------------------
-- Discriminant shell for quadratic membrane polynomials
--
-- Strongest live signal:
-- 1. the discriminant Δ = S^2 - 4A^2 gives a real algebraic lens on membrane
--    families once the outer shell A and seed S are fixed
-- 2. the empirical scans reported configuration-dependent usefulness rather
--    than a universal predictor
-- 3. the perfect-square lock and Legendre-symbol machinery remain important,
--    but should be kept as an explicit open shell here rather than as
--    hole-driven pseudo-proofs
------------------------------------------------------------------------

module Core.Discriminant where

open import Data.Nat using (ℕ; _+_; _*_; _>_)
open import Data.Integer as ℤ using (ℤ; +_; -_; _-_)
open import Data.Integer using () renaming (_*_ to _ℤ*_)
open import Data.Product using (Σ; _×_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Theorems.RationalStatistics using (ℚ; _/_)

------------------------------------------------------------------------
-- Algebraic core
------------------------------------------------------------------------

discriminant : ℕ -> ℕ -> ℤ
discriminant outer seed =
  let seed² = + (seed * seed)
      outer² = + (outer * outer)
      fourOuter² = (+ 4) ℤ* outer²
  in seed² - fourOuter²

Δ : ℕ -> ℕ -> ℤ
Δ = discriminant

evaluatePolynomial : ℕ -> ℕ -> ℕ -> ℕ
evaluatePolynomial outer seed x = outer * (x * x) + seed * x + outer

N : ℕ -> ℕ -> ℕ -> ℕ
N = evaluatePolynomial

record IsPerfectSquare (d : ℤ) : Set where
  constructor perfect-square
  field
    root : ℤ
    witness : root ℤ* root ≡ d

------------------------------------------------------------------------
-- Empirical shell values
------------------------------------------------------------------------

record DiscriminantObservation : Set where
  field
    outer : ℕ
    seed-length : ℕ
    quality-correlation : ℚ
    perfect-square-count : ℕ
    sample-count : ℕ

base6-15-m2 : DiscriminantObservation
base6-15-m2 = record
  { outer = 1
  ; seed-length = 2
  ; quality-correlation = 39 / 100
  ; perfect-square-count = 0
  ; sample-count = 30
  }

base6-51-m2 : DiscriminantObservation
base6-51-m2 = record
  { outer = 5
  ; seed-length = 2
  ; quality-correlation = 23 / 100
  ; perfect-square-count = 2
  ; sample-count = 30
  }

base12-15-m1 : DiscriminantObservation
base12-15-m1 = record
  { outer = 1
  ; seed-length = 1
  ; quality-correlation = 32 / 100
  ; perfect-square-count = 1
  ; sample-count = 11
  }

record BoundarySymmetryShell : Set2 where
  field
    base : ℕ
    outer : ℕ
    inner : ℕ
    hz-symmetry-shape : Set1

base6-asymmetric-shell : BoundarySymmetryShell
base6-asymmetric-shell = record
  { base = 6
  ; outer = 1
  ; inner = 5
  ; hz-symmetry-shape = Set
  }

base6-symmetric-shell : BoundarySymmetryShell
base6-symmetric-shell = record
  { base = 6
  ; outer = 2
  ; inner = 4
  ; hz-symmetry-shape = Set
  }

------------------------------------------------------------------------
-- Legendre/quality shell
------------------------------------------------------------------------

data LegendreSymbol : Set where
  residue : LegendreSymbol
  nonresidue : LegendreSymbol
  divisible : LegendreSymbol

record DiscriminantQuality (d : ℤ) : Set where
  constructor quality
  field
    leg3 : LegendreSymbol
    leg5 : LegendreSymbol
    leg7 : LegendreSymbol
    leg11 : LegendreSymbol
    leg13 : LegendreSymbol

  matches : LegendreSymbol -> LegendreSymbol -> ℕ
  matches residue residue = 1
  matches nonresidue nonresidue = 1
  matches divisible divisible = 1
  matches _ _ = 0

  countSymbol : LegendreSymbol -> ℕ
  countSymbol target =
    matches leg3 target +
    matches leg5 target +
    matches leg7 target +
    matches leg11 target +
    matches leg13 target

  admissible-count : ℕ
  admissible-count = countSymbol nonresidue

  obstructed-count : ℕ
  obstructed-count = countSymbol residue

  divisible-count : ℕ
  divisible-count = countSymbol divisible

------------------------------------------------------------------------
-- Open theorem shell
------------------------------------------------------------------------

postulate
  isPerfectSquare? : (d : ℤ) -> Set
  legendreSymbol : ℤ -> ℕ -> LegendreSymbol
  analyzeQuality : (outer seed : ℕ) -> DiscriminantQuality (Δ outer seed)
  algebraicLockTheorem : (outer seed : ℕ) -> outer > 0 -> IsPerfectSquare (Δ outer seed) -> Set
  discriminant-hz-connection : Set
