{-# OPTIONS --without-K #-}
------------------------------------------------------------------------
-- Hardy-Littlewood Singular Series
--
-- This module preserves the repo's Hardy-Littlewood vocabulary in a compilable
-- shell. The strongest honest claim here is structural:
--
-- 1. membrane / connector questions can be organized in the language of prime
--    constellations and local obstruction factors
-- 2. the singular-series side naturally shares Euler-product shape with the
--    totient-density layer
-- 3. the distance dependence seen in the experiments still needs a careful
--    bridge, likely through the critical-line / pair-correlation shell rather
--    than through the local factor alone
------------------------------------------------------------------------

module Theorems.HardyLittlewoodSingularSeries where

open import Data.List using (List; []; _∷_)
open import Data.Nat using (ℕ)
open import Data.Product using (_×_; _,_)
open import Data.Unit using (⊤; tt)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Theorems.RationalStatistics using (ℚ; _/_)
open import Theorems.TotientDensity using (fromℕ; _*ℚ_; _÷ℚ_)
open import Theorems.ConstellationCriticalLine using
  ( CriticalLineHeuristic
  ; PairCorrelationShell
  ; critical-line-heuristic
  ; pair-correlation-shell
  )

------------------------------------------------------------------------
-- Prime constellation shell
------------------------------------------------------------------------

record PrimeConstellation : Set₁ where
  field
    size : ℕ
    gaps : List ℕ
    admissibility-shell : Set

twin-primes : PrimeConstellation
twin-primes = record
  { size = 2
  ; gaps = 0 ∷ 2 ∷ []
  ; admissibility-shell = ⊤
  }

cousin-primes : PrimeConstellation
cousin-primes = record
  { size = 2
  ; gaps = 0 ∷ 4 ∷ []
  ; admissibility-shell = ⊤
  }

sexy-primes : PrimeConstellation
sexy-primes = record
  { size = 2
  ; gaps = 0 ∷ 6 ∷ []
  ; admissibility-shell = ⊤
  }

------------------------------------------------------------------------
-- Local obstruction shell
------------------------------------------------------------------------

record SingularSeriesShell : Set₁ where
  field
    constellation : PrimeConstellation
    local-obstruction : ℚ
    euler-product-shape : Set

twin-singular-series-shell : SingularSeriesShell
twin-singular-series-shell = record
  { constellation = twin-primes
  ; local-obstruction = 132 / 100
  ; euler-product-shape = ⊤
  }

membrane-constellation : ℕ → ℕ → PrimeConstellation
membrane-constellation base distance = record
  { size = 2
  ; gaps = 0 ∷ distance ∷ []
  ; admissibility-shell = ⊤
  }

------------------------------------------------------------------------
-- Named open claims
------------------------------------------------------------------------

postulate
  local-factor-formula : Set
  singular-series : PrimeConstellation → ℚ
  singular-series-product : Set
  singular-series-positive : Set
  twin-prime-constant : ℚ
  twin-constant-euler-product : Set
  hardy-littlewood-asymptotic-shell : Set
  expected-count-formula : Set
  membrane-hl-prediction : Set
  singular-series-distance-dependence : Set
  unified-membrane-prediction : Set
  pair-correlation-is-sqrt : Set
  calibrated-prediction : Set
  golden-ratio-hl-connection : Set
  three-constant-necessity : Set

------------------------------------------------------------------------
-- Concrete shell values
------------------------------------------------------------------------

critical-line-input : CriticalLineHeuristic
critical-line-input = critical-line-heuristic

pair-correlation-input : PairCorrelationShell
pair-correlation-input = pair-correlation-shell

base10-hl-factor : ℚ
base10-hl-factor = fromℕ 1 ÷ℚ fromℕ 10

distance1-shell-prediction : ℚ
distance1-shell-prediction = twin-prime-constant *ℚ base10-hl-factor

distance3-shell-prediction : ℚ
distance3-shell-prediction = twin-prime-constant *ℚ (fromℕ 1 ÷ℚ fromℕ 3)

------------------------------------------------------------------------
-- Validation queue
------------------------------------------------------------------------

data ValidationTask : Set where
  compute-local-factors : ValidationTask
  compare-hl-vs-observed : ValidationTask
  test-multiple-bases : ValidationTask
  isolate-correlation-correction : ValidationTask

validation-checklist : List ValidationTask
validation-checklist =
  compute-local-factors
  ∷ compare-hl-vs-observed
  ∷ test-multiple-bases
  ∷ isolate-correlation-correction
  ∷ []
