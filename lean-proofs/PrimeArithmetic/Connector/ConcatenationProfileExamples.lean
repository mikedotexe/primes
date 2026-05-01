import PrimeArithmetic.Connector.ConcatenationFamilies

namespace PrimeArithmetic.Connector

/-!
Maintained examples for the generic connector-family residue API.

`ConcatenationFamilies` proves the reusable profile theorems abstractly.
This module exercises that API on several maintained pairs already present in
the repository's connector/TUI surface:

- the zero-padded membrane pair `10301 ∘ 30305070305070303`,
- the small twin-prime pair `11 ∘ 13`,
- the small Sophie Germain pair `23 ∘ 47`.

These examples keep the family layer honest by showing that it applies beyond
the canonical pair `10301 ∘ 3007003007003`.

They should be read as same-language comparison profiles, not as evidence that
the canonical pair's empirical asymmetry already generalizes.
-/

def zeroPaddedRight : ℕ := 30305070305070303

def twinSmallLeft : ℕ := 11
def twinSmallRight : ℕ := 13

def sophieSmallLeft : ℕ := 23
def sophieSmallRight : ℕ := 47

def zeroPaddedMembraneProfileMod3 : PairResidueProfile where
  base := 10
  modulus := 3
  left := canonicalLeft
  right := zeroPaddedRight
  leftWidth := 5
  rightWidth := 17
  pairResidue := 2
  baseModOne := base10_modEq_one_mod3
  pairSum := by native_decide

def zeroPaddedMembraneProfileMod9 : PairResidueProfile where
  base := 10
  modulus := 9
  left := canonicalLeft
  right := zeroPaddedRight
  leftWidth := 5
  rightWidth := 17
  pairResidue := 8
  baseModOne := base10_modEq_one_mod9
  pairSum := by native_decide

def twinSmallProfileMod3 : PairResidueProfile where
  base := 10
  modulus := 3
  left := twinSmallLeft
  right := twinSmallRight
  leftWidth := 2
  rightWidth := 2
  pairResidue := 0
  baseModOne := base10_modEq_one_mod3
  pairSum := by native_decide

def twinSmallProfileMod9 : PairResidueProfile where
  base := 10
  modulus := 9
  left := twinSmallLeft
  right := twinSmallRight
  leftWidth := 2
  rightWidth := 2
  pairResidue := 6
  baseModOne := base10_modEq_one_mod9
  pairSum := by native_decide

def sophieSmallProfileMod3 : PairResidueProfile where
  base := 10
  modulus := 3
  left := sophieSmallLeft
  right := sophieSmallRight
  leftWidth := 2
  rightWidth := 2
  pairResidue := 1
  baseModOne := base10_modEq_one_mod3
  pairSum := by native_decide

def sophieSmallProfileMod9 : PairResidueProfile where
  base := 10
  modulus := 9
  left := sophieSmallLeft
  right := sophieSmallRight
  leftWidth := 2
  rightWidth := 2
  pairResidue := 7
  baseModOne := base10_modEq_one_mod9
  pairSum := by native_decide

theorem zeroPaddedMembrane_forward_reverse_same_mod3
    (connector connWidth : ℕ) :
    concatForward 10 canonicalLeft zeroPaddedRight connector 17 connWidth ≡
      concatReverse 10 canonicalLeft zeroPaddedRight connector 5 connWidth [MOD 3] := by
  simpa [zeroPaddedMembraneProfileMod3] using
    PairResidueProfile.forward_reverse_same_mod
      zeroPaddedMembraneProfileMod3 connector connWidth

theorem zeroPaddedMembrane_forward_divisibleBy3_iff_connector_mod1
    (connector connWidth : ℕ) :
    concatForward 10 canonicalLeft zeroPaddedRight connector 17 connWidth ≡ 0 [MOD 3] ↔
      connector ≡ 1 [MOD 3] := by
  simpa [zeroPaddedMembraneProfileMod3] using
    PairResidueProfile.concatForward_divisible_iff_connector_class
      zeroPaddedMembraneProfileMod3
      (shift := 1)
      (by native_decide :
        1 + zeroPaddedMembraneProfileMod3.pairResidue ≡ 0 [MOD zeroPaddedMembraneProfileMod3.modulus])
      connector connWidth

theorem zeroPaddedMembrane_forward_divisibleBy3_iff_across_widths
    (connector connWidth₁ connWidth₂ : ℕ) :
    concatForward 10 canonicalLeft zeroPaddedRight connector 17 connWidth₁ ≡ 0 [MOD 3] ↔
      concatForward 10 canonicalLeft zeroPaddedRight connector 17 connWidth₂ ≡ 0 [MOD 3] := by
  simpa [zeroPaddedMembraneProfileMod3] using
    PairResidueProfile.concatForward_modEq_target_iff_across_widths
      zeroPaddedMembraneProfileMod3 connector connWidth₁ connWidth₂ 0

theorem zeroPaddedMembrane_reverse_divisibleBy9_iff_connector_mod1
    (connector connWidth : ℕ) :
    concatReverse 10 canonicalLeft zeroPaddedRight connector 5 connWidth ≡ 0 [MOD 9] ↔
      connector ≡ 1 [MOD 9] := by
  simpa [zeroPaddedMembraneProfileMod9] using
    PairResidueProfile.concatReverse_divisible_iff_connector_class
      zeroPaddedMembraneProfileMod9
      (shift := 1)
      (by native_decide :
        1 + zeroPaddedMembraneProfileMod9.pairResidue ≡ 0 [MOD zeroPaddedMembraneProfileMod9.modulus])
      connector connWidth

theorem zeroPaddedMembrane_reverse_divisibleBy9_iff_across_widths
    (connector connWidth₁ connWidth₂ : ℕ) :
    concatReverse 10 canonicalLeft zeroPaddedRight connector 5 connWidth₁ ≡ 0 [MOD 9] ↔
      concatReverse 10 canonicalLeft zeroPaddedRight connector 5 connWidth₂ ≡ 0 [MOD 9] := by
  simpa [zeroPaddedMembraneProfileMod9] using
    PairResidueProfile.concatReverse_modEq_target_iff_across_widths
      zeroPaddedMembraneProfileMod9 connector connWidth₁ connWidth₂ 0

theorem twinSmall_forward_reverse_same_mod9
    (connector connWidth : ℕ) :
    concatForward 10 twinSmallLeft twinSmallRight connector 2 connWidth ≡
      concatReverse 10 twinSmallLeft twinSmallRight connector 2 connWidth [MOD 9] := by
  simpa [twinSmallProfileMod9] using
    PairResidueProfile.forward_reverse_same_mod
      twinSmallProfileMod9 connector connWidth

theorem twinSmall_forward_divisibleBy3_iff_connector_mod0
    (connector connWidth : ℕ) :
    concatForward 10 twinSmallLeft twinSmallRight connector 2 connWidth ≡ 0 [MOD 3] ↔
      connector ≡ 0 [MOD 3] := by
  simpa [twinSmallProfileMod3] using
    PairResidueProfile.concatForward_divisible_iff_connector_class
      twinSmallProfileMod3
      (shift := 0)
      (by native_decide :
        0 + twinSmallProfileMod3.pairResidue ≡ 0 [MOD twinSmallProfileMod3.modulus])
      connector connWidth

theorem twinSmall_reverse_divisibleBy9_iff_connector_mod3
    (connector connWidth : ℕ) :
    concatReverse 10 twinSmallLeft twinSmallRight connector 2 connWidth ≡ 0 [MOD 9] ↔
      connector ≡ 3 [MOD 9] := by
  simpa [twinSmallProfileMod9] using
    PairResidueProfile.concatReverse_divisible_iff_connector_class
      twinSmallProfileMod9
      (shift := 3)
      (by native_decide :
        3 + twinSmallProfileMod9.pairResidue ≡ 0 [MOD twinSmallProfileMod9.modulus])
      connector connWidth

theorem sophieSmall_forward_reverse_same_mod3
    (connector connWidth : ℕ) :
    concatForward 10 sophieSmallLeft sophieSmallRight connector 2 connWidth ≡
      concatReverse 10 sophieSmallLeft sophieSmallRight connector 2 connWidth [MOD 3] := by
  simpa [sophieSmallProfileMod3] using
    PairResidueProfile.forward_reverse_same_mod
      sophieSmallProfileMod3 connector connWidth

theorem sophieSmall_forward_divisibleBy3_iff_connector_mod2
    (connector connWidth : ℕ) :
    concatForward 10 sophieSmallLeft sophieSmallRight connector 2 connWidth ≡ 0 [MOD 3] ↔
      connector ≡ 2 [MOD 3] := by
  simpa [sophieSmallProfileMod3] using
    PairResidueProfile.concatForward_divisible_iff_connector_class
      sophieSmallProfileMod3
      (shift := 2)
      (by native_decide :
        2 + sophieSmallProfileMod3.pairResidue ≡ 0 [MOD sophieSmallProfileMod3.modulus])
      connector connWidth

theorem sophieSmall_reverse_divisibleBy9_iff_connector_mod2
    (connector connWidth : ℕ) :
    concatReverse 10 sophieSmallLeft sophieSmallRight connector 2 connWidth ≡ 0 [MOD 9] ↔
      connector ≡ 2 [MOD 9] := by
  simpa [sophieSmallProfileMod9] using
    PairResidueProfile.concatReverse_divisible_iff_connector_class
      sophieSmallProfileMod9
      (shift := 2)
      (by native_decide :
        2 + sophieSmallProfileMod9.pairResidue ≡ 0 [MOD sophieSmallProfileMod9.modulus])
      connector connWidth

end PrimeArithmetic.Connector
