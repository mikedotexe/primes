{-# OPTIONS --without-K #-}
------------------------------------------------------------------------
-- Lagrange-point shell for prime concatenation
--
-- Strongest live signal:
-- 1. the repo has a concrete canonical pair with two reported width-5
--    insertion cases that remain part of the maintained source shell
-- 2. that canonical pair is worth keeping as a formal shell even though the
--    general insertion/existence theory is still open
-- 3. digit-bias and clustering stories remain empirical; the canonical pair
--    itself already shows the claims need to stay narrow
--
-- Arithmetic-first reading:
-- - ConnectorHitShell is the preferred neutral term for one reported hit
-- - Residue and asymmetry questions should be stated separately
-- - "Lagrange point" remains as repository shorthand only
------------------------------------------------------------------------

module Core.LagrangePoints where

open import Data.Bool using (Bool; true; false)
open import Data.List using (List; []; _∷_)
open import Data.Nat using (ℕ; zero; suc; _+_; _≟_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Relation.Nullary using (Dec; yes; no)
open import Theorems.RationalStatistics using (ℚ; _/_)

------------------------------------------------------------------------
-- Concatenation shell
------------------------------------------------------------------------

record ConcatenatedStructureShell : Set where
  field
    prime1 : ℕ
    prime2 : ℕ
    buffer-length : ℕ

canonical-example : ConcatenatedStructureShell
canonical-example = record
  { prime1 = 10301
  ; prime2 = 3007003007003
  ; buffer-length = 5
  }

record LagrangePointShell : Set where
  field
    position : ℕ
    digit : ℕ
    result : ℕ
    reported-prime : Bool

ConnectorHitShell : Set
ConnectorHitShell = LagrangePointShell

canonical-L1 : LagrangePointShell
canonical-L1 = record
  { position = 1
  ; digit = 6
  ; result = 10301060003007003007003
  ; reported-prime = true
  }

canonical-L2 : LagrangePointShell
canonical-L2 = record
  { position = 4
  ; digit = 6
  ; result = 10301000063007003007003
  ; reported-prime = true
  }

canonical-points : List LagrangePointShell
canonical-points = canonical-L1 ∷ canonical-L2 ∷ []

canonical-connector-hits : List ConnectorHitShell
canonical-connector-hits = canonical-points

------------------------------------------------------------------------
-- Empirical shell summaries
------------------------------------------------------------------------

record EmpiricalCoverageShell : Set where
  field
    tested-pairs : ℕ
    successful-pairs : ℕ
    reported-success-rate : ℚ

reported-coverage : EmpiricalCoverageShell
reported-coverage = record
  { tested-pairs = 24
  ; successful-pairs = 24
  ; reported-success-rate = 1 / 1
  }

data EdgeRelativePosition : Set where
  near-prime1 : EdgeRelativePosition
  middle : EdgeRelativePosition
  near-prime2 : EdgeRelativePosition

record PositionedShell : Set where
  field
    point : LagrangePointShell
    region : EdgeRelativePosition

canonical-L1-positioned : PositionedShell
canonical-L1-positioned = record
  { point = canonical-L1
  ; region = near-prime1
  }

canonical-L2-positioned : PositionedShell
canonical-L2-positioned = record
  { point = canonical-L2
  ; region = near-prime2
  }

canonical-positioned : List PositionedShell
canonical-positioned = canonical-L1-positioned ∷ canonical-L2-positioned ∷ []

------------------------------------------------------------------------
-- Small constructive counting helpers
------------------------------------------------------------------------

digitFrequency : List LagrangePointShell -> ℕ -> ℕ
digitFrequency [] target = 0
digitFrequency (p ∷ ps) target with LagrangePointShell.digit p ≟ target
... | yes _ = suc (digitFrequency ps target)
... | no _ = digitFrequency ps target

pointCount : List LagrangePointShell -> ℕ
pointCount [] = 0
pointCount (_ ∷ ps) = suc (pointCount ps)

canonical-point-count : ℕ
canonical-point-count = pointCount canonical-points

canonical-six-frequency : ℕ
canonical-six-frequency = digitFrequency canonical-points 6

canonical-has-two-points : canonical-point-count ≡ 2
canonical-has-two-points = refl

canonical-digit-is-six-twice : canonical-six-frequency ≡ 2
canonical-digit-is-six-twice = refl

------------------------------------------------------------------------
-- Membrane connection shell
------------------------------------------------------------------------

record MembraneConnectionShell : Set1 where
  field
    prime2-is-membrane-like : Bool
    reported-point-count : ℕ
    enhancement-shape : Set

canonical-membrane-connection : MembraneConnectionShell
canonical-membrane-connection = record
  { prime2-is-membrane-like = true
  ; reported-point-count = 2
  ; enhancement-shape = List ℕ
  }

------------------------------------------------------------------------
-- Open theorem shell
------------------------------------------------------------------------

record LagrangeTheoryShell : Set1 where
  field
    insertion-shape : Set
    clustering-shape : Set
    balance-shape : Set
    enhancement-shape : Set

postulate
  insertDigit : ConcatenatedStructureShell -> ℕ -> ℕ -> ℕ
  lagrange-point-existence : ConcatenatedStructureShell -> Set
  lagrange-clustering : ConcatenatedStructureShell -> Set
  buffer-length-correlation : Set
  divisibility-balance : Set
  membrane-prime-enhancement : Set
  lagrange-theory : LagrangeTheoryShell
