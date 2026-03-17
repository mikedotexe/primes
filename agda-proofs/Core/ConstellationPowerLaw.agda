{-# OPTIONS --without-K #-}
------------------------------------------------------------------------
-- Constellation power-law shell
--
-- Strongest live signal:
-- 1. the membrane constellation experiments recorded a decay profile that was
--    well fit by an exponent near -1/2
-- 2. the observed twin / cousin / sexy ordering is still visible in the
--    reported data
-- 3. the general bridge from those observations to a proved universal law
--    remains open here and should be treated as an empirical shell
------------------------------------------------------------------------

module Core.ConstellationPowerLaw where

open import Data.Nat using (ℕ)
open import Data.Product using (_×_; _,_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Theorems.RationalStatistics using (ℚ; _/_)

------------------------------------------------------------------------
-- Core constellation vocabulary
------------------------------------------------------------------------

data ConstellationType : Set where
  twin : ConstellationType
  cousin : ConstellationType
  sexy : ConstellationType
  custom : ℕ -> ConstellationType

constellation-gap : ConstellationType -> ℕ
constellation-gap twin = 2
constellation-gap cousin = 4
constellation-gap sexy = 6
constellation-gap (custom n) = n

record PrimeConstellation : Set where
  field
    constellation-type : ConstellationType
    first-prime : ℕ
    second-prime : ℕ
    gap : ℕ

twin-example : PrimeConstellation
twin-example = record
  { constellation-type = twin
  ; first-prime = 11
  ; second-prime = 13
  ; gap = 2
  }

cousin-example : PrimeConstellation
cousin-example = record
  { constellation-type = cousin
  ; first-prime = 7
  ; second-prime = 11
  ; gap = 4
  }

sexy-example : PrimeConstellation
sexy-example = record
  { constellation-type = sexy
  ; first-prime = 5
  ; second-prime = 11
  ; gap = 6
  }

record PhaseLockShell : Set where
  field
    base : ℕ
    midpoint : ℕ
    distance : ℕ

twin-phase-lock : PhaseLockShell
twin-phase-lock = record
  { base = 24
  ; midpoint = 12
  ; distance = 1
  }

cousin-phase-lock : PhaseLockShell
cousin-phase-lock = record
  { base = 18
  ; midpoint = 9
  ; distance = 2
  }

sexy-phase-lock : PhaseLockShell
sexy-phase-lock = record
  { base = 16
  ; midpoint = 8
  ; distance = 3
  }

------------------------------------------------------------------------
-- Empirical fit surface
------------------------------------------------------------------------

record PowerLawFit : Set where
  field
    coefficient : ℚ
    exponent-magnitude : ℚ
    fit-r² : ℚ

empirical-power-law : PowerLawFit
empirical-power-law = record
  { coefficient = 2521 / 100
  ; exponent-magnitude = 53 / 100
  ; fit-r² = 8549 / 10000
  }

inverse-square-root-target : ℚ
inverse-square-root-target = 1 / 2

record SuccessObservation : Set where
  field
    constellation : PrimeConstellation
    phase-lock : PhaseLockShell
    observed-success : ℚ
    predicted-success : ℚ

twin-observation : SuccessObservation
twin-observation = record
  { constellation = twin-example
  ; phase-lock = twin-phase-lock
  ; observed-success = 24 / 100
  ; predicted-success = 252 / 1000
  }

cousin-observation : SuccessObservation
cousin-observation = record
  { constellation = cousin-example
  ; phase-lock = cousin-phase-lock
  ; observed-success = 20 / 100
  ; predicted-success = 175 / 1000
  }

sexy-observation : SuccessObservation
sexy-observation = record
  { constellation = sexy-example
  ; phase-lock = sexy-phase-lock
  ; observed-success = 13 / 100
  ; predicted-success = 141 / 1000
  }

record PredictionShell : Set where
  field
    distance : ℕ
    predicted-success : ℚ

distance-4-prediction : PredictionShell
distance-4-prediction = record
  { distance = 4
  ; predicted-success = 122 / 10
  }

distance-5-prediction : PredictionShell
distance-5-prediction = record
  { distance = 5
  ; predicted-success = 108 / 10
  }

distance-6-prediction : PredictionShell
distance-6-prediction = record
  { distance = 6
  ; predicted-success = 98 / 10
  }

------------------------------------------------------------------------
-- Open theorem shell
------------------------------------------------------------------------

record UnifiedConstellationTheory : Set1 where
  field
    gap-midpoint-formula : PrimeConstellation -> PhaseLockShell
    universality-shape : Set

postulate
  distance-as-rational : ℕ -> ℚ
  power-law-function : ℚ -> ℚ -> ℚ -> ℚ
  constellation-success-rate : PrimeConstellation -> PhaseLockShell -> ℚ
  empirical-fit-near-negative-half : Set
  inverse-sqrt-interpretation : Set
  power-law-monotonic : Set
  twin-better-than-cousin : Set
  cousin-better-than-sexy : Set
  unified-constellation-theory : UnifiedConstellationTheory
