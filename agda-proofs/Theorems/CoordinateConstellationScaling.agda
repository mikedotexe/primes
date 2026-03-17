{-# OPTIONS --without-K #-}
------------------------------------------------------------------------
-- Coordinate Constellation Scaling Theory
--
-- This module keeps the empirical scaling story for symmetric coordinate
-- constellations in a current-syntax Agda shell. The strongest live signal is
-- still empirical:
--
-- 1. observed success rates for the tested symmetric structures do not match
--    the naive Hardy-Littlewood scaling baseline
-- 2. the decay across k = 3, 5, 7 looked closer to linear than exponential in
--    the original experiments
-- 3. outer-coordinate constraints appear tied to coprimality structure in the
--    tested base-14 examples
--
-- The point of this file is not to pretend those claims are proved here. It is
-- to preserve the vocabulary, concrete observations, and intended theorem
-- interfaces in a compilable form.
------------------------------------------------------------------------

module Theorems.CoordinateConstellationScaling where

open import Data.Nat using (ℕ)
open import Data.List using (List; []; _∷_)
open import Data.Product using (_×_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Theorems.RationalStatistics using (ℚ; _/_)
open import Theorems.TotientDensity using (fromℕ; _*ℚ_; _÷ℚ_; φ)

------------------------------------------------------------------------
-- Core structures
------------------------------------------------------------------------

record SymmetricConstellation (k : ℕ) : Set where
  field
    base : ℕ
    middle : ℕ
    outer-coordinates : List ℕ

triplet-structure : SymmetricConstellation 3
triplet-structure = record
  { base = 14
  ; middle = 7
  ; outer-coordinates = 1 ∷ []
  }

quintuplet-structure : SymmetricConstellation 5
quintuplet-structure = record
  { base = 14
  ; middle = 7
  ; outer-coordinates = 5 ∷ 3 ∷ []
  }

septuplet-structure : SymmetricConstellation 7
septuplet-structure = record
  { base = 14
  ; middle = 11
  ; outer-coordinates = 3 ∷ 5 ∷ 7 ∷ []
  }

postulate
  constellation-success-rate : ∀ {k} → SymmetricConstellation k → ℚ
  hl-predicted-rate : ∀ {k} → SymmetricConstellation k → ℚ

------------------------------------------------------------------------
-- Recorded empirical observations
------------------------------------------------------------------------

record ScalingObservation : Set where
  field
    observed-ratio : ℚ
    predicted-ratio : ℚ
    relative-error : ℚ

k3-to-k5-observation : ScalingObservation
k3-to-k5-observation = record
  { observed-ratio = 16 / 10
  ; predicted-ratio = 70 / 10
  ; relative-error = 77 / 100
  }

k5-to-k7-observation : ScalingObservation
k5-to-k7-observation = record
  { observed-ratio = 12 / 10
  ; predicted-ratio = 70 / 10
  ; relative-error = 83 / 100
  }

record LinearDecayFit : Set where
  field
    baseline : ℚ
    penalty : ℚ

empirical-linear-fit : LinearDecayFit
empirical-linear-fit = record
  { baseline = 115 / 10
  ; penalty = 9 / 10
  }

------------------------------------------------------------------------
-- Constraint shell
------------------------------------------------------------------------

record OuterCoordinateConstraint : Set₁ where
  field
    base : ℕ
    allowed-values : List ℕ
    constraint-size : ℕ
    coprime-filter-shape : Set

base14-outer-constraint : OuterCoordinateConstraint
base14-outer-constraint = record
  { base = 14
  ; allowed-values = 1 ∷ 3 ∷ 5 ∷ 9 ∷ 11 ∷ 13 ∷ []
  ; constraint-size = 6
  ; coprime-filter-shape = φ 14 ≡ 6
  }

base14-outer-constraint-size : OuterCoordinateConstraint.constraint-size base14-outer-constraint ≡ 6
base14-outer-constraint-size = refl

record MonotonicPreference : Set where
  field
    monotonic-count : ℕ
    total-count : ℕ
    preference-ratio : ℚ

base14-monotonic-preference : MonotonicPreference
base14-monotonic-preference = record
  { monotonic-count = 32
  ; total-count = 73
  ; preference-ratio = 438 / 1000
  }

------------------------------------------------------------------------
-- Open theorem shell
------------------------------------------------------------------------

record GlobalConstraints {k : ℕ} (c : SymmetricConstellation k) : Set₁ where
  field
    global-symmetry : Set
    outer-shell-dependence : Set
    nonuniform-monotonicity : Set

record ModifiedScalingModel : Set where
  field
    base-success : ℚ
    protection-factor : ℚ
    dimension-penalty : ℚ

postulate
  hl-scaling-law : ∀ {k} → SymmetricConstellation k → Set
  hl-scaling-violation : ℕ → ScalingObservation × ScalingObservation
  linear-decay-law : ∀ {k} → SymmetricConstellation k → LinearDecayFit
  universal-outer-constraint : ∀ (base : ℕ) → OuterCoordinateConstraint
  outer-coords-are-phase-locks : ℕ → Set
  symmetric-entanglement-theorem :
    ∀ {k} (c : SymmetricConstellation k) →
    GlobalConstraints c →
    ModifiedScalingModel
  modified-scaling-conjecture :
    ∀ {k} (c : SymmetricConstellation k) →
    ModifiedScalingModel
  constraint-totient-relationship :
    ∀ (base : ℕ) →
    let constraint = universal-outer-constraint base
        constraint-fraction = fromℕ (OuterCoordinateConstraint.constraint-size constraint) ÷ℚ fromℕ base
        totient-fraction = fromℕ (φ base) ÷ℚ fromℕ base
    in
    Set

------------------------------------------------------------------------
-- Concrete shell values
------------------------------------------------------------------------

base14-constraint-fraction : ℚ
base14-constraint-fraction = fromℕ 6 ÷ℚ fromℕ 14

base14-totient-fraction : ℚ
base14-totient-fraction = fromℕ (φ 14) ÷ℚ fromℕ 14

postulate
  base14-constraint-fraction-close-to-totient : Set
  symmetry-changes-scaling : Set
