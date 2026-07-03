import Mathlib
import PrimeArithmetic.Generated.MatchedControlFamilyLanes
import PrimeArithmetic.Structure.BoundedKResidueProfile

namespace PrimeArithmetic.Density.CanonicalSmokeLaneProfiles

open PrimeArithmetic.Structure

/-!
Canonical smoke-panel lane profiles.

This module records maintained smoke-panel lanes through the `BoundedKFamilyLane`
wrapper. The statements are exact local residue-profile bookkeeping facts, not
density claims.
-/

structure SmokeLaneProfileCertificate where
  familyCode : String
  middleWidth : ℕ
  profileCert : BoundedKFamilyLaneProfileCertificate
  lookup_eq :
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.lookupByFamilyCodeAndMiddleWidth
      familyCode middleWidth = some profileCert.lane

noncomputable def SmokeLaneProfileCertificate.profile
    (cert : SmokeLaneProfileCertificate) :
    BoundedKResidueProfile :=
  cert.profileCert.profile

theorem SmokeLaneProfileCertificate.profile_excludedSeedClass
    (cert : SmokeLaneProfileCertificate) :
    cert.profile.excludedSeedClass cert.profileCert.modulus =
      cert.profileCert.excludedSeedClass :=
  cert.profileCert.profile_excludedSeedClass

theorem SmokeLaneProfileCertificate.templateValue_mod_eq_zero_iff_seed_mod_eq
    (cert : SmokeLaneProfileCertificate) (seed : ℕ) :
    templateValue cert.profileCert.lane.toSymmetricTemplateConfig seed %
        cert.profileCert.modulus = 0 ↔
      seed % cert.profileCert.modulus = cert.profileCert.excludedSeedClass :=
  cert.profileCert.templateValue_mod_eq_zero_iff_seed_mod_eq seed

def SmokeLaneProfileCertificate.ofZeroSeedClass
    (familyCode : String)
    (middleWidth : ℕ)
    (lane : BoundedKFamilyLane)
    (modulus excludedSeedClass : ℕ)
    [modulus_neZero : NeZero modulus]
    (hcop : modulus.Coprime lane.base)
    (lookup_eq :
      PrimeArithmetic.Generated.MatchedControlFamilyLanes.lookupByFamilyCodeAndMiddleWidth
        familyCode middleWidth = some lane)
    (zeroSeedClass_eq : lane.zeroSeedClassAt modulus hcop = excludedSeedClass)
    (excludedSeedClass_lt : excludedSeedClass < modulus) :
    SmokeLaneProfileCertificate where
  familyCode := familyCode
  middleWidth := middleWidth
  profileCert := {
    lane := lane
    modulus := modulus
    excludedSeedClass := excludedSeedClass
    modulus_neZero := modulus_neZero
    hcop := hcop
    zeroSeedClass_val_eq := by
      calc
        (lane.zeroSeedClassAt modulus hcop).val = ((excludedSeedClass : ZMod modulus).val) := by
          rw [zeroSeedClass_eq]
        _ = excludedSeedClass := ZMod.val_natCast_of_lt excludedSeedClass_lt
  }
  lookup_eq := lookup_eq

abbrev base12CompactM1Lane : BoundedKFamilyLane :=
  PrimeArithmetic.Generated.MatchedControlFamilyLanes.base12CompactM1Lane

abbrev base12CompactM2Lane : BoundedKFamilyLane :=
  PrimeArithmetic.Generated.MatchedControlFamilyLanes.base12CompactM2Lane

abbrev base14OffsetM1Lane : BoundedKFamilyLane :=
  PrimeArithmetic.Generated.MatchedControlFamilyLanes.base14OffsetM1Lane

abbrev base14OffsetM2Lane : BoundedKFamilyLane :=
  PrimeArithmetic.Generated.MatchedControlFamilyLanes.base14OffsetM2Lane

abbrev base30WheelLikeM1Lane : BoundedKFamilyLane :=
  PrimeArithmetic.Generated.MatchedControlFamilyLanes.base30WheelLikeM1Lane

abbrev base30WheelLikeM2Lane : BoundedKFamilyLane :=
  PrimeArithmetic.Generated.MatchedControlFamilyLanes.base30WheelLikeM2Lane

abbrev base6ChampionM1Lane : BoundedKFamilyLane :=
  PrimeArithmetic.Generated.MatchedControlFamilyLanes.base6ChampionM1Lane

abbrev base6ChampionM2Lane : BoundedKFamilyLane :=
  PrimeArithmetic.Generated.MatchedControlFamilyLanes.base6ChampionM2Lane

abbrev base10ClassicM2Lane : BoundedKFamilyLane :=
  PrimeArithmetic.Generated.MatchedControlFamilyLanes.base10ClassicM2Lane

abbrev base10BreathingM2Lane : BoundedKFamilyLane :=
  PrimeArithmetic.Generated.MatchedControlFamilyLanes.base10BreathingM2Lane

abbrev base10SymmetricM2Lane : BoundedKFamilyLane :=
  PrimeArithmetic.Generated.MatchedControlFamilyLanes.base10SymmetricM2Lane

abbrev base10ExclusiveM2Lane : BoundedKFamilyLane :=
  PrimeArithmetic.Generated.MatchedControlFamilyLanes.base10ExclusiveM2Lane

abbrev base12CompactM1Config : SymmetricTemplateConfig :=
  base12CompactM1Lane.toSymmetricTemplateConfig

abbrev base12CompactM2Config : SymmetricTemplateConfig :=
  base12CompactM2Lane.toSymmetricTemplateConfig

abbrev base14OffsetM1Config : SymmetricTemplateConfig :=
  base14OffsetM1Lane.toSymmetricTemplateConfig

abbrev base14OffsetM2Config : SymmetricTemplateConfig :=
  base14OffsetM2Lane.toSymmetricTemplateConfig

abbrev base30WheelLikeM1Config : SymmetricTemplateConfig :=
  base30WheelLikeM1Lane.toSymmetricTemplateConfig

abbrev base30WheelLikeM2Config : SymmetricTemplateConfig :=
  base30WheelLikeM2Lane.toSymmetricTemplateConfig

abbrev base6ChampionM1Config : SymmetricTemplateConfig :=
  base6ChampionM1Lane.toSymmetricTemplateConfig

abbrev base6ChampionM2Config : SymmetricTemplateConfig :=
  base6ChampionM2Lane.toSymmetricTemplateConfig

abbrev base10ClassicM2Config : SymmetricTemplateConfig :=
  base10ClassicM2Lane.toSymmetricTemplateConfig

abbrev base10BreathingM2Config : SymmetricTemplateConfig :=
  base10BreathingM2Lane.toSymmetricTemplateConfig

abbrev base10SymmetricM2Config : SymmetricTemplateConfig :=
  base10SymmetricM2Lane.toSymmetricTemplateConfig

abbrev base10ExclusiveM2Config : SymmetricTemplateConfig :=
  base10ExclusiveM2Lane.toSymmetricTemplateConfig

theorem base6ChampionM1Lane_lookup :
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.lookupByFamilyCodeAndMiddleWidth
      "B 6 ( 1, 5) k=(0,0) M=1" 1 = some base6ChampionM1Lane := by
  simpa [base6ChampionM1Lane] using
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.base6ChampionM1Lane_lookup

theorem base6ChampionM2Lane_lookup :
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.lookupByFamilyCodeAndMiddleWidth
      "B 6 ( 1, 5) k=(0,0) M=2" 2 = some base6ChampionM2Lane := by
  simpa [base6ChampionM2Lane] using
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.base6ChampionM2Lane_lookup

theorem base10BreathingM2Lane_lookup :
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.lookupByFamilyCodeAndMiddleWidth
      "B10 ( 3, 3) k=(0,1) M=2" 2 = some base10BreathingM2Lane := by
  simpa [base10BreathingM2Lane] using
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.base10BreathingM2Lane_lookup

theorem base10ClassicM2Lane_lookup :
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.lookupByFamilyCodeAndMiddleWidth
      "B10 ( 3, 7) k=(0,0) M=2" 2 = some base10ClassicM2Lane := by
  simpa [base10ClassicM2Lane] using
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.base10ClassicM2Lane_lookup

theorem base10SymmetricM2Lane_lookup :
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.lookupByFamilyCodeAndMiddleWidth
      "B10 ( 3, 3) k=(1,1) M=2" 2 = some base10SymmetricM2Lane := by
  simpa [base10SymmetricM2Lane] using
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.base10SymmetricM2Lane_lookup

theorem base10ExclusiveM2Lane_lookup :
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.lookupByFamilyCodeAndMiddleWidth
      "B10 ( 3, 7) k=(1,1) M=2" 2 = some base10ExclusiveM2Lane := by
  simpa [base10ExclusiveM2Lane] using
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.base10ExclusiveM2Lane_lookup

theorem base12CompactM1Lane_lookup :
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.lookupByFamilyCodeAndMiddleWidth
      "B12 ( 1, 1) k=(0,0) M=1" 1 = some base12CompactM1Lane := by
  simpa [base12CompactM1Lane] using
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.base12CompactM1Lane_lookup

theorem base12CompactM2Lane_lookup :
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.lookupByFamilyCodeAndMiddleWidth
      "B12 ( 1, 1) k=(0,0) M=2" 2 = some base12CompactM2Lane := by
  simpa [base12CompactM2Lane] using
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.base12CompactM2Lane_lookup

theorem base14OffsetM1Lane_lookup :
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.lookupByFamilyCodeAndMiddleWidth
      "B14 ( 1, 3) k=(0,0) M=1" 1 = some base14OffsetM1Lane := by
  simpa [base14OffsetM1Lane] using
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.base14OffsetM1Lane_lookup

theorem base14OffsetM2Lane_lookup :
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.lookupByFamilyCodeAndMiddleWidth
      "B14 ( 1, 3) k=(0,0) M=2" 2 = some base14OffsetM2Lane := by
  simpa [base14OffsetM2Lane] using
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.base14OffsetM2Lane_lookup

theorem base30WheelLikeM1Lane_lookup :
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.lookupByFamilyCodeAndMiddleWidth
      "B30 (11, 7) k=(0,0) M=1" 1 = some base30WheelLikeM1Lane := by
  simpa [base30WheelLikeM1Lane] using
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.base30WheelLikeM1Lane_lookup

theorem base30WheelLikeM2Lane_lookup :
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.lookupByFamilyCodeAndMiddleWidth
      "B30 (11, 7) k=(0,0) M=2" 2 = some base30WheelLikeM2Lane := by
  simpa [base30WheelLikeM2Lane] using
    PrimeArithmetic.Generated.MatchedControlFamilyLanes.base30WheelLikeM2Lane_lookup

theorem base6ChampionM1Lane_familyCode :
    base6ChampionM1Lane.familyCode = "B 6 ( 1, 5) k=(0,0) M=1" := rfl

theorem base6ChampionM2Lane_familyCode :
    base6ChampionM2Lane.familyCode = "B 6 ( 1, 5) k=(0,0) M=2" := rfl

theorem base10BreathingM2Lane_familyCode :
    base10BreathingM2Lane.familyCode = "B10 ( 3, 3) k=(0,1) M=2" := rfl

theorem base10ClassicM2Lane_familyCode :
    base10ClassicM2Lane.familyCode = "B10 ( 3, 7) k=(0,0) M=2" := rfl

theorem base10SymmetricM2Lane_familyCode :
    base10SymmetricM2Lane.familyCode = "B10 ( 3, 3) k=(1,1) M=2" := rfl

theorem base10ExclusiveM2Lane_familyCode :
    base10ExclusiveM2Lane.familyCode = "B10 ( 3, 7) k=(1,1) M=2" := rfl

theorem base12CompactM1Lane_familyCode :
    base12CompactM1Lane.familyCode = "B12 ( 1, 1) k=(0,0) M=1" := rfl

theorem base12CompactM2Lane_familyCode :
    base12CompactM2Lane.familyCode = "B12 ( 1, 1) k=(0,0) M=2" := rfl

theorem base14OffsetM1Lane_familyCode :
    base14OffsetM1Lane.familyCode = "B14 ( 1, 3) k=(0,0) M=1" := rfl

theorem base14OffsetM2Lane_familyCode :
    base14OffsetM2Lane.familyCode = "B14 ( 1, 3) k=(0,0) M=2" := rfl

theorem base30WheelLikeM1Lane_familyCode :
    base30WheelLikeM1Lane.familyCode = "B30 (11, 7) k=(0,0) M=1" := rfl

theorem base30WheelLikeM2Lane_familyCode :
    base30WheelLikeM2Lane.familyCode = "B30 (11, 7) k=(0,0) M=2" := rfl

theorem base6ChampionM1Config_eq_lane_config :
    base6ChampionM1Config =
      ({ kOuter := 0, kInner := 0 } : BoundedKConfig).toSymmetricTemplateConfig 6 1 5 1 := rfl

theorem base6ChampionM2Config_eq_lane_config :
    base6ChampionM2Config =
      ({ kOuter := 0, kInner := 0 } : BoundedKConfig).toSymmetricTemplateConfig 6 1 5 2 := rfl

theorem base10BreathingM2Config_eq_lane_config :
    base10BreathingM2Config =
      ({ kOuter := 0, kInner := 1 } : BoundedKConfig).toSymmetricTemplateConfig 10 3 3 2 := rfl

theorem base10ClassicM2Config_eq_lane_config :
    base10ClassicM2Config =
      ({ kOuter := 0, kInner := 0 } : BoundedKConfig).toSymmetricTemplateConfig 10 3 7 2 := rfl

theorem base10SymmetricM2Config_eq_lane_config :
    base10SymmetricM2Config =
      ({ kOuter := 1, kInner := 1 } : BoundedKConfig).toSymmetricTemplateConfig 10 3 3 2 := rfl

theorem base10ExclusiveM2Config_eq_lane_config :
    base10ExclusiveM2Config =
      ({ kOuter := 1, kInner := 1 } : BoundedKConfig).toSymmetricTemplateConfig 10 3 7 2 := rfl

theorem base12CompactM1Config_eq_lane_config :
    base12CompactM1Config =
      ({ kOuter := 0, kInner := 0 } : BoundedKConfig).toSymmetricTemplateConfig 12 1 1 1 := rfl

theorem base12CompactM2Config_eq_lane_config :
    base12CompactM2Config =
      ({ kOuter := 0, kInner := 0 } : BoundedKConfig).toSymmetricTemplateConfig 12 1 1 2 := rfl

theorem base14OffsetM1Config_eq_lane_config :
    base14OffsetM1Config =
      ({ kOuter := 0, kInner := 0 } : BoundedKConfig).toSymmetricTemplateConfig 14 1 3 1 := rfl

theorem base14OffsetM2Config_eq_lane_config :
    base14OffsetM2Config =
      ({ kOuter := 0, kInner := 0 } : BoundedKConfig).toSymmetricTemplateConfig 14 1 3 2 := rfl

theorem base30WheelLikeM1Config_eq_lane_config :
    base30WheelLikeM1Config =
      ({ kOuter := 0, kInner := 0 } : BoundedKConfig).toSymmetricTemplateConfig 30 11 7 1 := rfl

theorem base30WheelLikeM2Config_eq_lane_config :
    base30WheelLikeM2Config =
      ({ kOuter := 0, kInner := 0 } : BoundedKConfig).toSymmetricTemplateConfig 30 11 7 2 := rfl

theorem base6ChampionM1Config_eq_base6_15 :
    base6ChampionM1Config = base6_15 := rfl

theorem modSevenCoprime_base6ChampionM1 :
    (7).Coprime base6ChampionM1Lane.base := by
  native_decide

theorem modSevenCoprime_base6ChampionM2 :
    (7).Coprime base6ChampionM2Lane.base := by
  native_decide

theorem modSevenCoprime_base10BreathingM2 :
    (7).Coprime base10BreathingM2Lane.base := by
  native_decide

theorem modSevenCoprime_base10ClassicM2 :
    (7).Coprime base10ClassicM2Lane.base := by
  native_decide

theorem modSevenCoprime_base10SymmetricM2 :
    (7).Coprime base10SymmetricM2Lane.base := by
  native_decide

theorem modSevenCoprime_base10ExclusiveM2 :
    (7).Coprime base10ExclusiveM2Lane.base := by
  native_decide

theorem modThreeCoprime_base10ClassicM2 :
    (3).Coprime base10ClassicM2Lane.base := by
  native_decide

theorem modThreeCoprime_base10SymmetricM2 :
    (3).Coprime base10SymmetricM2Lane.base := by
  native_decide

theorem modFiveCoprime_base12CompactM1 :
    (5).Coprime base12CompactM1Lane.base := by
  native_decide

theorem modFiveCoprime_base12CompactM2 :
    (5).Coprime base12CompactM2Lane.base := by
  native_decide

theorem modFiveCoprime_base14OffsetM1 :
    (5).Coprime base14OffsetM1Lane.base := by
  native_decide

theorem modFiveCoprime_base14OffsetM2 :
    (5).Coprime base14OffsetM2Lane.base := by
  native_decide

theorem modSevenCoprime_base30WheelLikeM1 :
    (7).Coprime base30WheelLikeM1Lane.base := by
  native_decide

theorem modSevenCoprime_base30WheelLikeM2 :
    (7).Coprime base30WheelLikeM2Lane.base := by
  native_decide

theorem zeroSeedClass_base6ChampionM1_mod7 :
    base6ChampionM1Lane.zeroSeedClassAt 7 modSevenCoprime_base6ChampionM1 = 1 := by
  exact BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    base6ChampionM1Lane modSevenCoprime_base6ChampionM1 (by native_decide)

theorem zeroSeedClass_base6ChampionM2_mod7 :
    base6ChampionM2Lane.zeroSeedClassAt 7 modSevenCoprime_base6ChampionM2 = 0 := by
  exact BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    base6ChampionM2Lane modSevenCoprime_base6ChampionM2 (by native_decide)

theorem zeroSeedClass_base10BreathingM2_mod7 :
    base10BreathingM2Lane.zeroSeedClassAt 7 modSevenCoprime_base10BreathingM2 = 3 := by
  exact BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    base10BreathingM2Lane modSevenCoprime_base10BreathingM2 (by native_decide)

theorem zeroSeedClass_base10ClassicM2_mod7 :
    base10ClassicM2Lane.zeroSeedClassAt 7 modSevenCoprime_base10ClassicM2 = 5 := by
  exact BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    base10ClassicM2Lane modSevenCoprime_base10ClassicM2 (by native_decide)

theorem zeroSeedClass_base10SymmetricM2_mod7 :
    base10SymmetricM2Lane.zeroSeedClassAt 7 modSevenCoprime_base10SymmetricM2 = 5 := by
  exact BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    base10SymmetricM2Lane modSevenCoprime_base10SymmetricM2 (by native_decide)

theorem zeroSeedClass_base10ExclusiveM2_mod7 :
    base10ExclusiveM2Lane.zeroSeedClassAt 7 modSevenCoprime_base10ExclusiveM2 = 0 := by
  exact BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    base10ExclusiveM2Lane modSevenCoprime_base10ExclusiveM2 (by native_decide)

theorem zeroSeedClass_base10ClassicM2_mod3 :
    base10ClassicM2Lane.zeroSeedClassAt 3 modThreeCoprime_base10ClassicM2 = 1 := by
  exact BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    base10ClassicM2Lane modThreeCoprime_base10ClassicM2 (by native_decide)

theorem zeroSeedClass_base10SymmetricM2_mod3 :
    base10SymmetricM2Lane.zeroSeedClassAt 3 modThreeCoprime_base10SymmetricM2 = 0 := by
  exact BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    base10SymmetricM2Lane modThreeCoprime_base10SymmetricM2 (by native_decide)

theorem zeroSeedClass_base12CompactM1_mod5 :
    base12CompactM1Lane.zeroSeedClassAt 5 modFiveCoprime_base12CompactM1 = 2 := by
  exact BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    base12CompactM1Lane modFiveCoprime_base12CompactM1 (by native_decide)

theorem zeroSeedClass_base12CompactM2_mod5 :
    base12CompactM2Lane.zeroSeedClassAt 5 modFiveCoprime_base12CompactM2 = 1 := by
  exact BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    base12CompactM2Lane modFiveCoprime_base12CompactM2 (by native_decide)

theorem zeroSeedClass_base14OffsetM1_mod5 :
    base14OffsetM1Lane.zeroSeedClassAt 5 modFiveCoprime_base14OffsetM1 = 4 := by
  exact BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    base14OffsetM1Lane modFiveCoprime_base14OffsetM1 (by native_decide)

theorem zeroSeedClass_base14OffsetM2_mod5 :
    base14OffsetM2Lane.zeroSeedClassAt 5 modFiveCoprime_base14OffsetM2 = 0 := by
  exact BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    base14OffsetM2Lane modFiveCoprime_base14OffsetM2 (by native_decide)

theorem zeroSeedClass_base30WheelLikeM1_mod7 :
    base30WheelLikeM1Lane.zeroSeedClassAt 7 modSevenCoprime_base30WheelLikeM1 = 4 := by
  exact BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    base30WheelLikeM1Lane modSevenCoprime_base30WheelLikeM1 (by native_decide)

theorem zeroSeedClass_base30WheelLikeM2_mod7 :
    base30WheelLikeM2Lane.zeroSeedClassAt 7 modSevenCoprime_base30WheelLikeM2 = 2 := by
  exact BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    base30WheelLikeM2Lane modSevenCoprime_base30WheelLikeM2 (by native_decide)

end PrimeArithmetic.Density.CanonicalSmokeLaneProfiles
