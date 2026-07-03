import Mathlib
import PrimeArithmetic.Generated.MatchedControlFamilyLanes
import PrimeArithmetic.Structure.BoundedKResidueProfile

namespace PrimeArithmetic.Density.Base10SeedClassSeparation

open PrimeArithmetic.Structure

/-!
Exact base-10 forbidden seed-class separation for maintained smoke-panel
lane pairs.

This module compares stable same-boundary and cross-boundary, `M = 1` base-10
lane pairs from the canonical smoke panel at the coprime modulus `11`. The
results are purely local arithmetic statements: divisibility by `11` cuts out
different seed congruence classes in the fixed affine templates.

No density claim is made here.
-/

abbrev breathingM1Lane : BoundedKFamilyLane :=
  PrimeArithmetic.Generated.MatchedControlFamilyLanes.base10BreathingM1Lane

abbrev symmetricM1Lane : BoundedKFamilyLane :=
  PrimeArithmetic.Generated.MatchedControlFamilyLanes.base10SymmetricM1Lane

abbrev classicM1Lane : BoundedKFamilyLane :=
  PrimeArithmetic.Generated.MatchedControlFamilyLanes.base10ClassicM1Lane

abbrev exclusiveM1Lane : BoundedKFamilyLane :=
  PrimeArithmetic.Generated.MatchedControlFamilyLanes.base10ExclusiveM1Lane

abbrev breathingM1Config : SymmetricTemplateConfig :=
  breathingM1Lane.toSymmetricTemplateConfig

abbrev symmetricM1Config : SymmetricTemplateConfig :=
  symmetricM1Lane.toSymmetricTemplateConfig

abbrev classicM1Config : SymmetricTemplateConfig :=
  classicM1Lane.toSymmetricTemplateConfig

abbrev exclusiveM1Config : SymmetricTemplateConfig :=
  exclusiveM1Lane.toSymmetricTemplateConfig

theorem breathingM1Lane_familyCode :
    breathingM1Lane.familyCode = "B10 ( 3, 3) k=(0,1) M=1" := rfl

theorem symmetricM1Lane_familyCode :
    symmetricM1Lane.familyCode = "B10 ( 3, 3) k=(1,1) M=1" := rfl

theorem classicM1Lane_familyCode :
    classicM1Lane.familyCode = "B10 ( 3, 7) k=(0,0) M=1" := rfl

theorem exclusiveM1Lane_familyCode :
    exclusiveM1Lane.familyCode = "B10 ( 3, 7) k=(1,1) M=1" := rfl

theorem breathingM1Config_eq_lane_config :
    breathingM1Config =
      ({ kOuter := 0, kInner := 1 } : BoundedKConfig).toSymmetricTemplateConfig 10 3 3 1 := rfl

theorem symmetricM1Config_eq_lane_config :
    symmetricM1Config =
      ({ kOuter := 1, kInner := 1 } : BoundedKConfig).toSymmetricTemplateConfig 10 3 3 1 := rfl

theorem classicM1Config_eq_lane_config :
    classicM1Config =
      ({ kOuter := 0, kInner := 0 } : BoundedKConfig).toSymmetricTemplateConfig 10 3 7 1 := rfl

theorem exclusiveM1Config_eq_lane_config :
    exclusiveM1Config =
      ({ kOuter := 1, kInner := 1 } : BoundedKConfig).toSymmetricTemplateConfig 10 3 7 1 := rfl

theorem modElevenCoprime_breathingM1 :
    (11).Coprime breathingM1Lane.base := by
  native_decide

theorem modElevenCoprime_symmetricM1 :
    (11).Coprime symmetricM1Lane.base := by
  native_decide

theorem modElevenCoprime_classicM1 :
    (11).Coprime classicM1Lane.base := by
  native_decide

theorem modElevenCoprime_exclusiveM1 :
    (11).Coprime exclusiveM1Lane.base := by
  native_decide

theorem zeroSeedClass_breathingM1_mod11 :
    breathingM1Lane.zeroSeedClassAt 11 modElevenCoprime_breathingM1 = 0 := by
  exact BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    breathingM1Lane modElevenCoprime_breathingM1 (by native_decide)

theorem zeroSeedClass_symmetricM1_mod11 :
    symmetricM1Lane.zeroSeedClassAt 11 modElevenCoprime_symmetricM1 = 10 := by
  exact BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    symmetricM1Lane modElevenCoprime_symmetricM1 (by native_decide)

theorem breathingM1ProfileAt_mod11_excludedSeedClass :
    (breathingM1Lane.residueProfileAt 11 modElevenCoprime_breathingM1).excludedSeedClass 11 = 0 := by
  simpa [zeroSeedClass_breathingM1_mod11] using
    (BoundedKFamilyLane.residueProfileAt_excludedSeedClass
      breathingM1Lane 11 modElevenCoprime_breathingM1)

theorem symmetricM1ProfileAt_mod11_excludedSeedClass :
    (symmetricM1Lane.residueProfileAt 11 modElevenCoprime_symmetricM1).excludedSeedClass 11 = 10 := by
  simpa [zeroSeedClass_symmetricM1_mod11] using
    (BoundedKFamilyLane.residueProfileAt_excludedSeedClass
      symmetricM1Lane 11 modElevenCoprime_symmetricM1)

theorem zeroSeedClass_breathingM1_ne_symmetricM1_mod11 :
    breathingM1Lane.zeroSeedClassAt 11 modElevenCoprime_breathingM1 ≠
      symmetricM1Lane.zeroSeedClassAt 11 modElevenCoprime_symmetricM1 := by
  rw [zeroSeedClass_breathingM1_mod11, zeroSeedClass_symmetricM1_mod11]
  native_decide

theorem forbiddenSeedMask_breathingM1_ne_symmetricM1_mod11 :
    breathingM1Lane.forbiddenSeedMaskAt 11 ≠ symmetricM1Lane.forbiddenSeedMaskAt 11 := by
  exact BoundedKFamilyLane.forbiddenSeedMaskAt_ne_of_zeroSeedClassAt_ne
    breathingM1Lane symmetricM1Lane
    modElevenCoprime_breathingM1 modElevenCoprime_symmetricM1
    zeroSeedClass_breathingM1_ne_symmetricM1_mod11

theorem forbiddenResidues_breathingM1_ne_symmetricM1_mod11 :
    breathingM1Lane.forbiddenResiduesAt 11 modElevenCoprime_breathingM1 ≠
      symmetricM1Lane.forbiddenResiduesAt 11 modElevenCoprime_symmetricM1 := by
  exact BoundedKFamilyLane.forbiddenResiduesAt_ne_of_zeroSeedClassAt_ne
    breathingM1Lane symmetricM1Lane
    modElevenCoprime_breathingM1 modElevenCoprime_symmetricM1
    zeroSeedClass_breathingM1_ne_symmetricM1_mod11

theorem survivorResidueCount_breathingM1_eq_symmetricM1_mod11 :
    (breathingM1Lane.survivorResiduesAt 11 modElevenCoprime_breathingM1).card =
      (symmetricM1Lane.survivorResiduesAt 11 modElevenCoprime_symmetricM1).card := by
  exact BoundedKFamilyLane.survivorResiduesAt_card_eq
    breathingM1Lane symmetricM1Lane
    modElevenCoprime_breathingM1 modElevenCoprime_symmetricM1

theorem zeroSeedClass_classicM1_mod11 :
    classicM1Lane.zeroSeedClassAt 11 modElevenCoprime_classicM1 = 8 := by
  exact BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    classicM1Lane modElevenCoprime_classicM1 (by native_decide)

theorem zeroSeedClass_exclusiveM1_mod11 :
    exclusiveM1Lane.zeroSeedClassAt 11 modElevenCoprime_exclusiveM1 = 2 := by
  exact BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    exclusiveM1Lane modElevenCoprime_exclusiveM1 (by native_decide)

theorem classicM1ProfileAt_mod11_excludedSeedClass :
    (classicM1Lane.residueProfileAt 11 modElevenCoprime_classicM1).excludedSeedClass 11 = 8 := by
  simpa [zeroSeedClass_classicM1_mod11] using
    (BoundedKFamilyLane.residueProfileAt_excludedSeedClass
      classicM1Lane 11 modElevenCoprime_classicM1)

theorem exclusiveM1ProfileAt_mod11_excludedSeedClass :
    (exclusiveM1Lane.residueProfileAt 11 modElevenCoprime_exclusiveM1).excludedSeedClass 11 = 2 := by
  simpa [zeroSeedClass_exclusiveM1_mod11] using
    (BoundedKFamilyLane.residueProfileAt_excludedSeedClass
      exclusiveM1Lane 11 modElevenCoprime_exclusiveM1)

theorem zeroSeedClass_classicM1_ne_exclusiveM1_mod11 :
    classicM1Lane.zeroSeedClassAt 11 modElevenCoprime_classicM1 ≠
      exclusiveM1Lane.zeroSeedClassAt 11 modElevenCoprime_exclusiveM1 := by
  rw [zeroSeedClass_classicM1_mod11, zeroSeedClass_exclusiveM1_mod11]
  native_decide

theorem forbiddenSeedMask_classicM1_ne_exclusiveM1_mod11 :
    classicM1Lane.forbiddenSeedMaskAt 11 ≠ exclusiveM1Lane.forbiddenSeedMaskAt 11 := by
  exact BoundedKFamilyLane.forbiddenSeedMaskAt_ne_of_zeroSeedClassAt_ne
    classicM1Lane exclusiveM1Lane
    modElevenCoprime_classicM1 modElevenCoprime_exclusiveM1
    zeroSeedClass_classicM1_ne_exclusiveM1_mod11

theorem forbiddenResidues_classicM1_ne_exclusiveM1_mod11 :
    classicM1Lane.forbiddenResiduesAt 11 modElevenCoprime_classicM1 ≠
      exclusiveM1Lane.forbiddenResiduesAt 11 modElevenCoprime_exclusiveM1 := by
  exact BoundedKFamilyLane.forbiddenResiduesAt_ne_of_zeroSeedClassAt_ne
    classicM1Lane exclusiveM1Lane
    modElevenCoprime_classicM1 modElevenCoprime_exclusiveM1
    zeroSeedClass_classicM1_ne_exclusiveM1_mod11

theorem survivorResidueCount_classicM1_eq_exclusiveM1_mod11 :
    (classicM1Lane.survivorResiduesAt 11 modElevenCoprime_classicM1).card =
      (exclusiveM1Lane.survivorResiduesAt 11 modElevenCoprime_exclusiveM1).card := by
  exact BoundedKFamilyLane.survivorResiduesAt_card_eq
    classicM1Lane exclusiveM1Lane
    modElevenCoprime_classicM1 modElevenCoprime_exclusiveM1

theorem zeroSeedClass_breathingM1_ne_exclusiveM1_mod11 :
    breathingM1Lane.zeroSeedClassAt 11 modElevenCoprime_breathingM1 ≠
      exclusiveM1Lane.zeroSeedClassAt 11 modElevenCoprime_exclusiveM1 := by
  rw [zeroSeedClass_breathingM1_mod11, zeroSeedClass_exclusiveM1_mod11]
  native_decide

theorem forbiddenSeedMask_breathingM1_ne_exclusiveM1_mod11 :
    breathingM1Lane.forbiddenSeedMaskAt 11 ≠ exclusiveM1Lane.forbiddenSeedMaskAt 11 := by
  exact BoundedKFamilyLane.forbiddenSeedMaskAt_ne_of_zeroSeedClassAt_ne
    breathingM1Lane exclusiveM1Lane
    modElevenCoprime_breathingM1 modElevenCoprime_exclusiveM1
    zeroSeedClass_breathingM1_ne_exclusiveM1_mod11

theorem forbiddenResidues_breathingM1_ne_exclusiveM1_mod11 :
    breathingM1Lane.forbiddenResiduesAt 11 modElevenCoprime_breathingM1 ≠
      exclusiveM1Lane.forbiddenResiduesAt 11 modElevenCoprime_exclusiveM1 := by
  exact BoundedKFamilyLane.forbiddenResiduesAt_ne_of_zeroSeedClassAt_ne
    breathingM1Lane exclusiveM1Lane
    modElevenCoprime_breathingM1 modElevenCoprime_exclusiveM1
    zeroSeedClass_breathingM1_ne_exclusiveM1_mod11

theorem survivorResidueCount_breathingM1_eq_exclusiveM1_mod11 :
    (breathingM1Lane.survivorResiduesAt 11 modElevenCoprime_breathingM1).card =
      (exclusiveM1Lane.survivorResiduesAt 11 modElevenCoprime_exclusiveM1).card := by
  exact BoundedKFamilyLane.survivorResiduesAt_card_eq
    breathingM1Lane exclusiveM1Lane
    modElevenCoprime_breathingM1 modElevenCoprime_exclusiveM1

theorem zeroSeedClass_breathingM1_ne_classicM1_mod11 :
    breathingM1Lane.zeroSeedClassAt 11 modElevenCoprime_breathingM1 ≠
      classicM1Lane.zeroSeedClassAt 11 modElevenCoprime_classicM1 := by
  rw [zeroSeedClass_breathingM1_mod11, zeroSeedClass_classicM1_mod11]
  native_decide

theorem forbiddenSeedMask_breathingM1_ne_classicM1_mod11 :
    breathingM1Lane.forbiddenSeedMaskAt 11 ≠ classicM1Lane.forbiddenSeedMaskAt 11 := by
  exact BoundedKFamilyLane.forbiddenSeedMaskAt_ne_of_zeroSeedClassAt_ne
    breathingM1Lane classicM1Lane
    modElevenCoprime_breathingM1 modElevenCoprime_classicM1
    zeroSeedClass_breathingM1_ne_classicM1_mod11

theorem forbiddenResidues_breathingM1_ne_classicM1_mod11 :
    breathingM1Lane.forbiddenResiduesAt 11 modElevenCoprime_breathingM1 ≠
      classicM1Lane.forbiddenResiduesAt 11 modElevenCoprime_classicM1 := by
  exact BoundedKFamilyLane.forbiddenResiduesAt_ne_of_zeroSeedClassAt_ne
    breathingM1Lane classicM1Lane
    modElevenCoprime_breathingM1 modElevenCoprime_classicM1
    zeroSeedClass_breathingM1_ne_classicM1_mod11

theorem survivorResidueCount_breathingM1_eq_classicM1_mod11 :
    (breathingM1Lane.survivorResiduesAt 11 modElevenCoprime_breathingM1).card =
      (classicM1Lane.survivorResiduesAt 11 modElevenCoprime_classicM1).card := by
  exact BoundedKFamilyLane.survivorResiduesAt_card_eq
    breathingM1Lane classicM1Lane
    modElevenCoprime_breathingM1 modElevenCoprime_classicM1

theorem zeroSeedClass_symmetricM1_ne_classicM1_mod11 :
    symmetricM1Lane.zeroSeedClassAt 11 modElevenCoprime_symmetricM1 ≠
      classicM1Lane.zeroSeedClassAt 11 modElevenCoprime_classicM1 := by
  rw [zeroSeedClass_symmetricM1_mod11, zeroSeedClass_classicM1_mod11]
  native_decide

theorem forbiddenSeedMask_symmetricM1_ne_classicM1_mod11 :
    symmetricM1Lane.forbiddenSeedMaskAt 11 ≠ classicM1Lane.forbiddenSeedMaskAt 11 := by
  exact BoundedKFamilyLane.forbiddenSeedMaskAt_ne_of_zeroSeedClassAt_ne
    symmetricM1Lane classicM1Lane
    modElevenCoprime_symmetricM1 modElevenCoprime_classicM1
    zeroSeedClass_symmetricM1_ne_classicM1_mod11

theorem forbiddenResidues_symmetricM1_ne_classicM1_mod11 :
    symmetricM1Lane.forbiddenResiduesAt 11 modElevenCoprime_symmetricM1 ≠
      classicM1Lane.forbiddenResiduesAt 11 modElevenCoprime_classicM1 := by
  exact BoundedKFamilyLane.forbiddenResiduesAt_ne_of_zeroSeedClassAt_ne
    symmetricM1Lane classicM1Lane
    modElevenCoprime_symmetricM1 modElevenCoprime_classicM1
    zeroSeedClass_symmetricM1_ne_classicM1_mod11

theorem survivorResidueCount_symmetricM1_eq_classicM1_mod11 :
    (symmetricM1Lane.survivorResiduesAt 11 modElevenCoprime_symmetricM1).card =
      (classicM1Lane.survivorResiduesAt 11 modElevenCoprime_classicM1).card := by
  exact BoundedKFamilyLane.survivorResiduesAt_card_eq
    symmetricM1Lane classicM1Lane
    modElevenCoprime_symmetricM1 modElevenCoprime_classicM1

theorem zeroSeedClass_symmetricM1_ne_exclusiveM1_mod11 :
    symmetricM1Lane.zeroSeedClassAt 11 modElevenCoprime_symmetricM1 ≠
      exclusiveM1Lane.zeroSeedClassAt 11 modElevenCoprime_exclusiveM1 := by
  rw [zeroSeedClass_symmetricM1_mod11, zeroSeedClass_exclusiveM1_mod11]
  native_decide

theorem forbiddenSeedMask_symmetricM1_ne_exclusiveM1_mod11 :
    symmetricM1Lane.forbiddenSeedMaskAt 11 ≠ exclusiveM1Lane.forbiddenSeedMaskAt 11 := by
  exact BoundedKFamilyLane.forbiddenSeedMaskAt_ne_of_zeroSeedClassAt_ne
    symmetricM1Lane exclusiveM1Lane
    modElevenCoprime_symmetricM1 modElevenCoprime_exclusiveM1
    zeroSeedClass_symmetricM1_ne_exclusiveM1_mod11

theorem forbiddenResidues_symmetricM1_ne_exclusiveM1_mod11 :
    symmetricM1Lane.forbiddenResiduesAt 11 modElevenCoprime_symmetricM1 ≠
      exclusiveM1Lane.forbiddenResiduesAt 11 modElevenCoprime_exclusiveM1 := by
  exact BoundedKFamilyLane.forbiddenResiduesAt_ne_of_zeroSeedClassAt_ne
    symmetricM1Lane exclusiveM1Lane
    modElevenCoprime_symmetricM1 modElevenCoprime_exclusiveM1
    zeroSeedClass_symmetricM1_ne_exclusiveM1_mod11

theorem survivorResidueCount_symmetricM1_eq_exclusiveM1_mod11 :
    (symmetricM1Lane.survivorResiduesAt 11 modElevenCoprime_symmetricM1).card =
      (exclusiveM1Lane.survivorResiduesAt 11 modElevenCoprime_exclusiveM1).card := by
  exact BoundedKFamilyLane.survivorResiduesAt_card_eq
    symmetricM1Lane exclusiveM1Lane
    modElevenCoprime_symmetricM1 modElevenCoprime_exclusiveM1

theorem templateValue_breathingM1_mod11_eq_zero_iff_seed_mod_eq_zero
    (seed : ℕ) :
    templateValue breathingM1Config seed % 11 = 0 ↔ seed % 11 = 0 := by
  simpa [zeroSeedClass_breathingM1_mod11] using
    (BoundedKFamilyLane.templateValue_mod_eq_zero_iff_seed_mod_eq_zeroSeedClassAt
      (lane := breathingM1Lane) (modulus := 11) (seed := seed)
      modElevenCoprime_breathingM1)

theorem templateValue_symmetricM1_mod11_eq_zero_iff_seed_mod_eq_ten
    (seed : ℕ) :
    templateValue symmetricM1Config seed % 11 = 0 ↔ seed % 11 = 10 := by
  simpa [zeroSeedClass_symmetricM1_mod11] using
    (BoundedKFamilyLane.templateValue_mod_eq_zero_iff_seed_mod_eq_zeroSeedClassAt
      (lane := symmetricM1Lane) (modulus := 11) (seed := seed)
      modElevenCoprime_symmetricM1)

theorem templateValue_classicM1_mod11_eq_zero_iff_seed_mod_eq_eight
    (seed : ℕ) :
    templateValue classicM1Config seed % 11 = 0 ↔ seed % 11 = 8 := by
  simpa [zeroSeedClass_classicM1_mod11] using
    (BoundedKFamilyLane.templateValue_mod_eq_zero_iff_seed_mod_eq_zeroSeedClassAt
      (lane := classicM1Lane) (modulus := 11) (seed := seed)
      modElevenCoprime_classicM1)

theorem templateValue_exclusiveM1_mod11_eq_zero_iff_seed_mod_eq_two
    (seed : ℕ) :
    templateValue exclusiveM1Config seed % 11 = 0 ↔ seed % 11 = 2 := by
  simpa [zeroSeedClass_exclusiveM1_mod11] using
    (BoundedKFamilyLane.templateValue_mod_eq_zero_iff_seed_mod_eq_zeroSeedClassAt
      (lane := exclusiveM1Lane) (modulus := 11) (seed := seed)
      modElevenCoprime_exclusiveM1)

end PrimeArithmetic.Density.Base10SeedClassSeparation
