{-# OPTIONS --safe --without-K #-}
------------------------------------------------------------------------
-- Bounded-k transfer witness shell
--
-- Strongest live signal:
-- 1. this module keeps the exact Agda-side witness vocabulary stable
-- 2. generated witness catalogs can import it without duplicating shell code
-- 3. the exact arithmetic remains separate from the empirical threshold prose
------------------------------------------------------------------------

module Examples.BoundedKTransferWitnessShell where

open import Data.Bool using (Bool; true; false)
open import Data.Nat using (ℕ; zero; suc; _+_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

open import Theorems.BoundedKCompactness using
  ( BoundedKConfig
  ; compact₀
  ; kConfig
  ; paddingWeight
  ; diameter
  )

------------------------------------------------------------------------
-- Small exact signed-count shell
------------------------------------------------------------------------

data SignedDelta : Set where
  zeroΔ : SignedDelta
  positiveΔ : ℕ → SignedDelta
  negativeΔ : ℕ → SignedDelta

deltaMagnitude : SignedDelta → ℕ
deltaMagnitude zeroΔ = 0
deltaMagnitude (positiveΔ n) = n
deltaMagnitude (negativeΔ n) = n

deltaPositive? : SignedDelta → Bool
deltaPositive? zeroΔ = false
deltaPositive? (positiveΔ _) = true
deltaPositive? (negativeΔ _) = false

deltaNonpositive? : SignedDelta → Bool
deltaNonpositive? zeroΔ = true
deltaNonpositive? (positiveΔ _) = false
deltaNonpositive? (negativeΔ _) = true

_<?_ : ℕ → ℕ → Bool
zero <? zero = false
zero <? suc n = true
suc m <? zero = false
suc m <? suc n = m <? n

_∧ᵇ_ : Bool → Bool → Bool
true ∧ᵇ true = true
_ ∧ᵇ _ = false

dominatesAbs? : SignedDelta → SignedDelta → Bool
dominatesAbs? δ₁ δ₂ = deltaMagnitude δ₂ <? deltaMagnitude δ₁

------------------------------------------------------------------------
-- Generated-style witness shell
------------------------------------------------------------------------

record TransferWitnessSummary : Set where
  field
    base : ℕ
    middleLength : ℕ
    outer : ℕ
    inner : ℕ
    fromConfig : BoundedKConfig
    toConfig : BoundedKConfig

    stableZeroCount : ℕ
    gainZeroCount : ℕ
    lossZeroCount : ℕ
    stableNonzeroCount : ℕ
    nonzeroChurnCount : ℕ

    stableZeroPrimeDelta : SignedDelta
    boundaryPrimeDelta : SignedDelta

open TransferWitnessSummary public

sharedAdmissibleCount : TransferWitnessSummary → ℕ
sharedAdmissibleCount summary = stableZeroCount summary

admissibleCountFrom : TransferWitnessSummary → ℕ
admissibleCountFrom summary = stableZeroCount summary + lossZeroCount summary

admissibleCountTo : TransferWitnessSummary → ℕ
admissibleCountTo summary = stableZeroCount summary + gainZeroCount summary

sameMaskCount : TransferWitnessSummary → ℕ
sameMaskCount summary = stableZeroCount summary + stableNonzeroCount summary

zeroUnionCount : TransferWitnessSummary → ℕ
zeroUnionCount summary =
  stableZeroCount summary + gainZeroCount summary + lossZeroCount summary

overlapLed? : TransferWitnessSummary → Bool
overlapLed? summary =
  deltaPositive? (stableZeroPrimeDelta summary) ∧ᵇ
  dominatesAbs? (stableZeroPrimeDelta summary) (boundaryPrimeDelta summary)

boundaryLed? : TransferWitnessSummary → Bool
boundaryLed? summary =
  deltaPositive? (boundaryPrimeDelta summary) ∧ᵇ
  dominatesAbs? (boundaryPrimeDelta summary) (stableZeroPrimeDelta summary)

sharedAdmissible-identity : ∀ summary → sharedAdmissibleCount summary ≡ stableZeroCount summary
sharedAdmissible-identity summary = refl

admissibleFrom-identity
  : ∀ summary
  → admissibleCountFrom summary ≡ stableZeroCount summary + lossZeroCount summary
admissibleFrom-identity summary = refl

admissibleTo-identity
  : ∀ summary
  → admissibleCountTo summary ≡ stableZeroCount summary + gainZeroCount summary
admissibleTo-identity summary = refl

sameMask-identity
  : ∀ summary
  → sameMaskCount summary ≡ stableZeroCount summary + stableNonzeroCount summary
sameMask-identity summary = refl

zeroUnion-identity
  : ∀ summary
  → zeroUnionCount summary ≡ stableZeroCount summary + gainZeroCount summary + lossZeroCount summary
zeroUnion-identity summary = refl
