{-# OPTIONS --without-K #-}
------------------------------------------------------------------------
-- Advanced orthogonality shell
--
-- Strongest live signal:
-- 1. the prime-pair orthogonality experiment is a higher-level analogue of the
--    membrane orthogonality story
-- 2. the useful part here is the experiment framing and the interpretation
--    split between raw counts, HL normalization, and membrane open questions
-- 3. the actual float backend and large-sample computation should remain
--    explicit shell assumptions rather than parser-drifted pseudo-code
------------------------------------------------------------------------

module Advanced.Orthogonality where

open import Data.List using (List; []; _∷_)
open import Data.Nat using (ℕ)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Core.OrthogonalityFramework using
  ( SignedCorrelation
  ; CorrelationSign
  ; positive
  ; negative
  ; neutral
  ; OrthogonalityStatus
  ; orthogonal
  ; moderately-correlated
  ; strongly-correlated
  ; classifyCorrelation
  ; raw-correlation
  ; hl-normalized-correlation
  ; predicted-full-correlation
  )
open import Theorems.RationalStatistics using (ℚ; _/_)

------------------------------------------------------------------------
-- Prime-pair experiment shell
------------------------------------------------------------------------

record PrimePairExperimentShell : Set where
  field
    prime-bound : ℕ
    max-gap : ℕ
    babylonian-raw-correlation : SignedCorrelation
    babylonian-hl-correlation : SignedCorrelation
    orthogonality-threshold : ℚ

default-prime-pair-experiment : PrimePairExperimentShell
default-prime-pair-experiment = record
  { prime-bound = 100000
  ; max-gap = 200
  ; babylonian-raw-correlation = record
      { sign = positive
      ; magnitude = 726 / 1000
      }
  ; babylonian-hl-correlation = record
      { sign = neutral
      ; magnitude = 8 / 100
      }
  ; orthogonality-threshold = 15 / 100
  }

prime-pair-raw-status :
  classifyCorrelation (PrimePairExperimentShell.babylonian-raw-correlation default-prime-pair-experiment)
  ≡ strongly-correlated
prime-pair-raw-status = refl

prime-pair-hl-status :
  classifyCorrelation (PrimePairExperimentShell.babylonian-hl-correlation default-prime-pair-experiment)
  ≡ orthogonal
prime-pair-hl-status = refl

------------------------------------------------------------------------
-- Membrane comparison shell
------------------------------------------------------------------------

record MembraneComparisonShell : Set where
  field
    membrane-raw-correlation : SignedCorrelation
    membrane-hl-correlation : SignedCorrelation
    membrane-predicted-full-correlation : SignedCorrelation

default-membrane-comparison : MembraneComparisonShell
default-membrane-comparison = record
  { membrane-raw-correlation = raw-correlation
  ; membrane-hl-correlation = hl-normalized-correlation
  ; membrane-predicted-full-correlation = predicted-full-correlation
  }

membrane-raw-status :
  classifyCorrelation (MembraneComparisonShell.membrane-raw-correlation default-membrane-comparison)
  ≡ strongly-correlated
membrane-raw-status = refl

membrane-hl-status :
  classifyCorrelation (MembraneComparisonShell.membrane-hl-correlation default-membrane-comparison)
  ≡ moderately-correlated
membrane-hl-status = refl

------------------------------------------------------------------------
-- Interpretation shell
------------------------------------------------------------------------

data OrthogonalityInterpretation : Set where
  prime-pair-bias-removed : OrthogonalityInterpretation
  membrane-bias-reduced-not-removed : OrthogonalityInterpretation
  membrane-full-normalization-open : OrthogonalityInterpretation

interpretations : List OrthogonalityInterpretation
interpretations =
  prime-pair-bias-removed ∷
  membrane-bias-reduced-not-removed ∷
  membrane-full-normalization-open ∷
  []

record ExperimentComparisonShell : Set where
  field
    prime-pair-experiment : PrimePairExperimentShell
    membrane-comparison : MembraneComparisonShell
    interpretation : OrthogonalityInterpretation

current-comparison : ExperimentComparisonShell
current-comparison = record
  { prime-pair-experiment = default-prime-pair-experiment
  ; membrane-comparison = default-membrane-comparison
  ; interpretation = membrane-full-normalization-open
  }

------------------------------------------------------------------------
-- Open computation shell
------------------------------------------------------------------------

record AdvancedOrthogonalityTheoryShell : Set1 where
  field
    babylonian-backend-shape : Set
    prime-pair-backend-shape : Set
    membrane-backend-shape : Set
    float-alignment-shape : Set

postulate
  babylonian-score : ℕ -> ℚ
  pairs-raw : ℕ -> ℕ -> ℕ
  singular-series : ℕ -> ℚ
  pairs-normalized : ℕ -> ℕ -> ℚ
  run-prime-pair-experiment : ℕ -> ℕ -> PrimePairExperimentShell
  membrane-orthogonality-test : ℕ -> PrimePairExperimentShell
  complete-float-alignment : Set
  advanced-orthogonality-theory : AdvancedOrthogonalityTheoryShell
