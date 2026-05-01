{-# OPTIONS --safe --without-K #-}
------------------------------------------------------------------------
-- Auto-generated bounded-k transfer witness catalog
--
-- Source of truth: `cargo run --bin export_bounded_k_transfer_agda_summary`
------------------------------------------------------------------------

module Examples.Generated.BoundedKTransferWitnessCatalog where

open import Data.Bool using (true; false)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

open import Theorems.BoundedKCompactness using (compact₀; kConfig; paddingWeight; diameter)
open import Examples.BoundedKTransferWitnessShell using
  ( TransferWitnessSummary
  ; zeroΔ
  ; positiveΔ
  ; negativeΔ
  ; overlapLed?
  ; boundaryLed?
  ; fromConfig
  ; toConfig
  )

-- Base 14, pair (D, B) = (13,11), M = 2, best k=(0,1)
base14-DB : TransferWitnessSummary
base14-DB = record
  { base = 14
  ; middleLength = 2
  ; outer = 13
  ; inner = 11
  ; fromConfig = compact₀
  ; toConfig = kConfig 0 1
  ; stableZeroCount = 49
  ; gainZeroCount = 20
  ; lossZeroCount = 21
  ; stableNonzeroCount = 53
  ; nonzeroChurnCount = 53
  ; stableZeroPrimeDelta = positiveΔ 12
  ; boundaryPrimeDelta = negativeΔ 6
  }

base14-DB-overlap-led : overlapLed? base14-DB ≡ true
base14-DB-overlap-led = refl
base14-DB-boundary-not-led : boundaryLed? base14-DB ≡ false
base14-DB-boundary-not-led = refl
base14-DB-diameter-from : diameter 2 (fromConfig base14-DB) ≡ 6
base14-DB-diameter-from = refl
base14-DB-diameter-to : diameter 2 (toConfig base14-DB) ≡ 8
base14-DB-diameter-to = refl
base14-DB-padding-step : paddingWeight (toConfig base14-DB) ≡ 1
base14-DB-padding-step = refl

-- Base 10, pair (3, 3) = (3,3), M = 2, best k=(1,0)
base10-33 : TransferWitnessSummary
base10-33 = record
  { base = 10
  ; middleLength = 2
  ; outer = 3
  ; inner = 3
  ; fromConfig = compact₀
  ; toConfig = kConfig 1 0
  ; stableZeroCount = 29
  ; gainZeroCount = 8
  ; lossZeroCount = 7
  ; stableNonzeroCount = 35
  ; nonzeroChurnCount = 21
  ; stableZeroPrimeDelta = zeroΔ
  ; boundaryPrimeDelta = positiveΔ 2
  }

base10-33-overlap-not-led : overlapLed? base10-33 ≡ false
base10-33-overlap-not-led = refl
base10-33-boundary-led : boundaryLed? base10-33 ≡ true
base10-33-boundary-led = refl
base10-33-diameter-from : diameter 2 (fromConfig base10-33) ≡ 6
base10-33-diameter-from = refl
base10-33-diameter-to : diameter 2 (toConfig base10-33) ≡ 8
base10-33-diameter-to = refl
base10-33-padding-step : paddingWeight (toConfig base10-33) ≡ 1
base10-33-padding-step = refl

-- Base 34, pair (P, 9) = (25,9), M = 2, best k=(1,0)
base34-P9 : TransferWitnessSummary
base34-P9 = record
  { base = 34
  ; middleLength = 2
  ; outer = 25
  ; inner = 9
  ; fromConfig = compact₀
  ; toConfig = kConfig 1 0
  ; stableZeroCount = 289
  ; gainZeroCount = 88
  ; lossZeroCount = 86
  ; stableNonzeroCount = 410
  ; nonzeroChurnCount = 283
  ; stableZeroPrimeDelta = positiveΔ 1
  ; boundaryPrimeDelta = positiveΔ 8
  }

base34-P9-overlap-not-led : overlapLed? base34-P9 ≡ false
base34-P9-overlap-not-led = refl
base34-P9-boundary-led : boundaryLed? base34-P9 ≡ true
base34-P9-boundary-led = refl
base34-P9-diameter-from : diameter 2 (fromConfig base34-P9) ≡ 6
base34-P9-diameter-from = refl
base34-P9-diameter-to : diameter 2 (toConfig base34-P9) ≡ 8
base34-P9-diameter-to = refl
base34-P9-padding-step : paddingWeight (toConfig base34-P9) ≡ 1
base34-P9-padding-step = refl

-- Base 22, pair (H, J) = (17,19), M = 2, best k=(0,1)
base22-HJ : TransferWitnessSummary
base22-HJ = record
  { base = 22
  ; middleLength = 2
  ; outer = 17
  ; inner = 19
  ; fromConfig = compact₀
  ; toConfig = kConfig 0 1
  ; stableZeroCount = 87
  ; gainZeroCount = 77
  ; lossZeroCount = 72
  ; stableNonzeroCount = 82
  ; nonzeroChurnCount = 166
  ; stableZeroPrimeDelta = negativeΔ 2
  ; boundaryPrimeDelta = positiveΔ 5
  }

base22-HJ-overlap-not-led : overlapLed? base22-HJ ≡ false
base22-HJ-overlap-not-led = refl
base22-HJ-boundary-led : boundaryLed? base22-HJ ≡ true
base22-HJ-boundary-led = refl
base22-HJ-diameter-from : diameter 2 (fromConfig base22-HJ) ≡ 6
base22-HJ-diameter-from = refl
base22-HJ-diameter-to : diameter 2 (toConfig base22-HJ) ≡ 8
base22-HJ-diameter-to = refl
base22-HJ-padding-step : paddingWeight (toConfig base22-HJ) ≡ 1
base22-HJ-padding-step = refl
