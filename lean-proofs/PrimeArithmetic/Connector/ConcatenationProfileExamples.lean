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

theorem zeroPaddedMembraneProfileMod3_forward_reverse_survivor_count_eq
    (connectors : List ℕ) (connWidth : ℕ) :
    (connectors.filter (fun connector =>
      decide (¬ concatForward 10 canonicalLeft zeroPaddedRight connector 17 connWidth ≡
        0 [MOD 3]))).length =
    (connectors.filter (fun connector =>
      decide (¬ concatReverse 10 canonicalLeft zeroPaddedRight connector 5 connWidth ≡
        0 [MOD 3]))).length := by
  simpa [zeroPaddedMembraneProfileMod3] using
    PairResidueProfile.forward_reverse_survivor_count_eq
      zeroPaddedMembraneProfileMod3 connectors connWidth

theorem zeroPaddedMembraneProfileMod9_forward_reverse_survivor_count_eq
    (connectors : List ℕ) (connWidth : ℕ) :
    (connectors.filter (fun connector =>
      decide (¬ concatForward 10 canonicalLeft zeroPaddedRight connector 17 connWidth ≡
        0 [MOD 9]))).length =
    (connectors.filter (fun connector =>
      decide (¬ concatReverse 10 canonicalLeft zeroPaddedRight connector 5 connWidth ≡
        0 [MOD 9]))).length := by
  simpa [zeroPaddedMembraneProfileMod9] using
    PairResidueProfile.forward_reverse_survivor_count_eq
      zeroPaddedMembraneProfileMod9 connectors connWidth

theorem twinSmallProfileMod3_forward_reverse_survivor_count_eq
    (connectors : List ℕ) (connWidth : ℕ) :
    (connectors.filter (fun connector =>
      decide (¬ concatForward 10 twinSmallLeft twinSmallRight connector 2 connWidth ≡
        0 [MOD 3]))).length =
    (connectors.filter (fun connector =>
      decide (¬ concatReverse 10 twinSmallLeft twinSmallRight connector 2 connWidth ≡
        0 [MOD 3]))).length := by
  simpa [twinSmallProfileMod3] using
    PairResidueProfile.forward_reverse_survivor_count_eq
      twinSmallProfileMod3 connectors connWidth

theorem twinSmallProfileMod9_forward_reverse_survivor_count_eq
    (connectors : List ℕ) (connWidth : ℕ) :
    (connectors.filter (fun connector =>
      decide (¬ concatForward 10 twinSmallLeft twinSmallRight connector 2 connWidth ≡
        0 [MOD 9]))).length =
    (connectors.filter (fun connector =>
      decide (¬ concatReverse 10 twinSmallLeft twinSmallRight connector 2 connWidth ≡
        0 [MOD 9]))).length := by
  simpa [twinSmallProfileMod9] using
    PairResidueProfile.forward_reverse_survivor_count_eq
      twinSmallProfileMod9 connectors connWidth

theorem twinSmallProfileMod3_mod9_forward_reverse_survivor_count_eq
    (connectors : List ℕ) (connWidth : ℕ) :
    ((connectors.filter (fun connector =>
      decide (¬ concatForward 10 twinSmallLeft twinSmallRight connector 2 connWidth ≡
        0 [MOD 3]))).length =
    (connectors.filter (fun connector =>
      decide (¬ concatReverse 10 twinSmallLeft twinSmallRight connector 2 connWidth ≡
        0 [MOD 3]))).length) ∧
    ((connectors.filter (fun connector =>
      decide (¬ concatForward 10 twinSmallLeft twinSmallRight connector 2 connWidth ≡
        0 [MOD 9]))).length =
    (connectors.filter (fun connector =>
      decide (¬ concatReverse 10 twinSmallLeft twinSmallRight connector 2 connWidth ≡
        0 [MOD 9]))).length) := by
  exact
    ⟨twinSmallProfileMod3_forward_reverse_survivor_count_eq connectors connWidth,
      twinSmallProfileMod9_forward_reverse_survivor_count_eq connectors connWidth⟩

theorem sophieSmallProfileMod3_forward_reverse_survivor_count_eq
    (connectors : List ℕ) (connWidth : ℕ) :
    (connectors.filter (fun connector =>
      decide (¬ concatForward 10 sophieSmallLeft sophieSmallRight connector 2 connWidth ≡
        0 [MOD 3]))).length =
    (connectors.filter (fun connector =>
      decide (¬ concatReverse 10 sophieSmallLeft sophieSmallRight connector 2 connWidth ≡
        0 [MOD 3]))).length := by
  simpa [sophieSmallProfileMod3] using
    PairResidueProfile.forward_reverse_survivor_count_eq
      sophieSmallProfileMod3 connectors connWidth

theorem sophieSmallProfileMod9_forward_reverse_survivor_count_eq
    (connectors : List ℕ) (connWidth : ℕ) :
    (connectors.filter (fun connector =>
      decide (¬ concatForward 10 sophieSmallLeft sophieSmallRight connector 2 connWidth ≡
        0 [MOD 9]))).length =
    (connectors.filter (fun connector =>
      decide (¬ concatReverse 10 sophieSmallLeft sophieSmallRight connector 2 connWidth ≡
        0 [MOD 9]))).length := by
  simpa [sophieSmallProfileMod9] using
    PairResidueProfile.forward_reverse_survivor_count_eq
      sophieSmallProfileMod9 connectors connWidth

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

theorem twinPrimeAboveThree_left_mod3_eq_two
    {p : ℕ} (hp : Nat.Prime p) (hp2 : Nat.Prime (p + 2)) (hp3 : 3 < p) :
    p ≡ 2 [MOD 3] := by
  have hmodlt : p % 3 < 3 := Nat.mod_lt p (by decide)
  interval_cases hmod : p % 3
  · have hdvd : 3 ∣ p := Nat.dvd_of_mod_eq_zero hmod
    have hcases := hp.eq_one_or_self_of_dvd 3 hdvd
    omega
  · have hsum_mod : (p + 2) % 3 = 0 := by omega
    have hdvd : 3 ∣ p + 2 := Nat.dvd_of_mod_eq_zero hsum_mod
    have hcases := hp2.eq_one_or_self_of_dvd 3 hdvd
    omega
  · exact hmod

theorem twinPrimeAboveThree_pair_sum_mod3_eq_zero
    {p : ℕ} (hp : Nat.Prime p) (hp2 : Nat.Prime (p + 2)) (hp3 : 3 < p) :
    p + (p + 2) ≡ 0 [MOD 3] := by
  have hpmod : p ≡ 2 [MOD 3] :=
    twinPrimeAboveThree_left_mod3_eq_two hp hp2 hp3
  change (p + (p + 2)) % 3 = 0 % 3
  change p % 3 = 2 % 3 at hpmod
  norm_num at hpmod ⊢
  omega

theorem twinPrimeAboveThree_decimal_connector_mod3_forward_blocked
    {p connector rightWidth connWidth : ℕ}
    (hp : Nat.Prime p) (hp2 : Nat.Prime (p + 2)) (hp3 : 3 < p)
    (hConnector : connector ≡ 0 [MOD 3]) :
    concatForward 10 p (p + 2) connector rightWidth connWidth ≡ 0 [MOD 3] := by
  have hPair : p + (p + 2) ≡ 0 [MOD 3] :=
    twinPrimeAboveThree_pair_sum_mod3_eq_zero hp hp2 hp3
  refine
    (concatForward_modEq_sum_of_base_modEq_one
      (base := 10) (modulus := 3) (left := p) (right := p + 2)
      (connector := connector) (rightWidth := rightWidth) (connWidth := connWidth)
      base10_modEq_one_mod3).trans ?_
  have hsum : connector + (p + (p + 2)) ≡ 0 + 0 [MOD 3] :=
    hConnector.add hPair
  simpa [Nat.add_assoc, Nat.add_left_comm, Nat.add_comm] using hsum

theorem twinPrimeAboveThree_decimal_connector_mod3_reverse_blocked
    {p connector leftWidth connWidth : ℕ}
    (hp : Nat.Prime p) (hp2 : Nat.Prime (p + 2)) (hp3 : 3 < p)
    (hConnector : connector ≡ 0 [MOD 3]) :
    concatReverse 10 p (p + 2) connector leftWidth connWidth ≡ 0 [MOD 3] := by
  have hPair : p + (p + 2) ≡ 0 [MOD 3] :=
    twinPrimeAboveThree_pair_sum_mod3_eq_zero hp hp2 hp3
  refine
    (concatReverse_modEq_sum_of_base_modEq_one
      (base := 10) (modulus := 3) (left := p) (right := p + 2)
      (connector := connector) (leftWidth := leftWidth) (connWidth := connWidth)
      base10_modEq_one_mod3).trans ?_
  have hsum : connector + (p + (p + 2)) ≡ 0 + 0 [MOD 3] :=
    hConnector.add hPair
  simpa [Nat.add_assoc, Nat.add_left_comm, Nat.add_comm] using hsum

/-!
The following bounded classifiers are tied to the tracked connector stress
artifact for replicated digit-8 edge cells. They classify the currently scanned
twin-prime left endpoints only; they are not general connector laws.
-/

def digit8LeadingWidth6Connector : ℕ := 800000

def digit8LeadingWidth6ScannedTwinLefts : List ℕ :=
  [3, 5, 11, 17, 29, 41, 59, 71, 101, 107, 137, 149]

def digit8LeadingWidth6ReverseOnlyTwinLefts : List ℕ :=
  [3, 17, 71]

def digit8LeadingWidth6Mod17ReverseOnlyClassifier (p : ℕ) : Bool :=
  p % 17 == 0 || p % 17 == 3

theorem digit8LeadingWidth6_reverseOnlyLefts_eq_filter_mod17 :
    digit8LeadingWidth6ScannedTwinLefts.filter digit8LeadingWidth6Mod17ReverseOnlyClassifier =
      digit8LeadingWidth6ReverseOnlyTwinLefts := by
  native_decide

theorem digit8LeadingWidth6_reverseOnly_mem_iff_mod17 (p : ℕ) :
    p ∈ digit8LeadingWidth6ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧ (p % 17 = 0 ∨ p % 17 = 3) := by
  rw [← digit8LeadingWidth6_reverseOnlyLefts_eq_filter_mod17]
  simp [digit8LeadingWidth6Mod17ReverseOnlyClassifier]

def digit8LeadingWidth6Mod23ReverseOnlyClassifier (p : ℕ) : Bool :=
  p % 23 == 2 || p % 23 == 3 || p % 23 == 17

theorem digit8LeadingWidth6_reverseOnlyLefts_eq_filter_mod23 :
    digit8LeadingWidth6ScannedTwinLefts.filter digit8LeadingWidth6Mod23ReverseOnlyClassifier =
      digit8LeadingWidth6ReverseOnlyTwinLefts := by
  native_decide

theorem digit8LeadingWidth6_reverseOnly_mem_iff_mod23 (p : ℕ) :
    p ∈ digit8LeadingWidth6ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 23 = 2 ∨ p % 23 = 3 ∨ p % 23 = 17) := by
  rw [← digit8LeadingWidth6_reverseOnlyLefts_eq_filter_mod23]
  simp [digit8LeadingWidth6Mod23ReverseOnlyClassifier]
  tauto

def digit8LeadingWidth6Mod29ReverseOnlyClassifier (p : ℕ) : Bool :=
  p % 29 == 3 || p % 29 == 13 || p % 29 == 17

theorem digit8LeadingWidth6_reverseOnlyLefts_eq_filter_mod29 :
    digit8LeadingWidth6ScannedTwinLefts.filter digit8LeadingWidth6Mod29ReverseOnlyClassifier =
      digit8LeadingWidth6ReverseOnlyTwinLefts := by
  native_decide

theorem digit8LeadingWidth6_reverseOnly_mem_iff_mod29 (p : ℕ) :
    p ∈ digit8LeadingWidth6ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 29 = 3 ∨ p % 29 = 13 ∨ p % 29 = 17) := by
  rw [← digit8LeadingWidth6_reverseOnlyLefts_eq_filter_mod29]
  simp [digit8LeadingWidth6Mod29ReverseOnlyClassifier]
  tauto

def digit8LeadingWidth6Mod31ReverseOnlyClassifier (p : ℕ) : Bool :=
  p % 31 == 3 || p % 31 == 9 || p % 31 == 17

theorem digit8LeadingWidth6_reverseOnlyLefts_eq_filter_mod31 :
    digit8LeadingWidth6ScannedTwinLefts.filter digit8LeadingWidth6Mod31ReverseOnlyClassifier =
      digit8LeadingWidth6ReverseOnlyTwinLefts := by
  native_decide

theorem digit8LeadingWidth6_reverseOnly_mem_iff_mod31 (p : ℕ) :
    p ∈ digit8LeadingWidth6ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 31 = 3 ∨ p % 31 = 9 ∨ p % 31 = 17) := by
  rw [← digit8LeadingWidth6_reverseOnlyLefts_eq_filter_mod31]
  simp [digit8LeadingWidth6Mod31ReverseOnlyClassifier]
  tauto

theorem digit8LeadingWidth6_reverseOnly_multiModulusClassifier (p : ℕ) :
    (p ∈ digit8LeadingWidth6ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧ (p % 17 = 0 ∨ p % 17 = 3)) ∧
    (p ∈ digit8LeadingWidth6ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 23 = 2 ∨ p % 23 = 3 ∨ p % 23 = 17)) ∧
    (p ∈ digit8LeadingWidth6ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 29 = 3 ∨ p % 29 = 13 ∨ p % 29 = 17)) ∧
    (p ∈ digit8LeadingWidth6ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 31 = 3 ∨ p % 31 = 9 ∨ p % 31 = 17)) := by
  exact
    ⟨digit8LeadingWidth6_reverseOnly_mem_iff_mod17 p,
      digit8LeadingWidth6_reverseOnly_mem_iff_mod23 p,
      digit8LeadingWidth6_reverseOnly_mem_iff_mod29 p,
      digit8LeadingWidth6_reverseOnly_mem_iff_mod31 p⟩

def digit8TrailingWidth5Connector : ℕ := 8

def digit8TrailingWidth5ReverseOnlyTwinLefts : List ℕ :=
  [11, 29, 101, 107]

def digit8TrailingWidth5Mod19ReverseOnlyClassifier (p : ℕ) : Bool :=
  p % 19 == 6 || p % 19 == 10 || p % 19 == 11 || p % 19 == 12

theorem digit8TrailingWidth5_reverseOnlyLefts_eq_filter_mod19 :
    digit8LeadingWidth6ScannedTwinLefts.filter digit8TrailingWidth5Mod19ReverseOnlyClassifier =
      digit8TrailingWidth5ReverseOnlyTwinLefts := by
  native_decide

theorem digit8TrailingWidth5_reverseOnly_mem_iff_mod19 (p : ℕ) :
    p ∈ digit8TrailingWidth5ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 19 = 6 ∨ p % 19 = 10 ∨ p % 19 = 11 ∨ p % 19 = 12) := by
  rw [← digit8TrailingWidth5_reverseOnlyLefts_eq_filter_mod19]
  simp [digit8TrailingWidth5Mod19ReverseOnlyClassifier]
  tauto

def digit8TrailingWidth5Mod29ReverseOnlyClassifier (p : ℕ) : Bool :=
  p % 29 == 0 || p % 29 == 11 || p % 29 == 14 || p % 29 == 20

theorem digit8TrailingWidth5_reverseOnlyLefts_eq_filter_mod29 :
    digit8LeadingWidth6ScannedTwinLefts.filter digit8TrailingWidth5Mod29ReverseOnlyClassifier =
      digit8TrailingWidth5ReverseOnlyTwinLefts := by
  native_decide

theorem digit8TrailingWidth5_reverseOnly_mem_iff_mod29 (p : ℕ) :
    p ∈ digit8TrailingWidth5ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 29 = 0 ∨ p % 29 = 11 ∨ p % 29 = 14 ∨ p % 29 = 20) := by
  rw [← digit8TrailingWidth5_reverseOnlyLefts_eq_filter_mod29]
  simp [digit8TrailingWidth5Mod29ReverseOnlyClassifier]
  tauto

def digit8TrailingWidth5Mod31ReverseOnlyClassifier (p : ℕ) : Bool :=
  p % 31 == 8 || p % 31 == 11 || p % 31 == 14 || p % 31 == 29

theorem digit8TrailingWidth5_reverseOnlyLefts_eq_filter_mod31 :
    digit8LeadingWidth6ScannedTwinLefts.filter digit8TrailingWidth5Mod31ReverseOnlyClassifier =
      digit8TrailingWidth5ReverseOnlyTwinLefts := by
  native_decide

theorem digit8TrailingWidth5_reverseOnly_mem_iff_mod31 (p : ℕ) :
    p ∈ digit8TrailingWidth5ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 31 = 8 ∨ p % 31 = 11 ∨ p % 31 = 14 ∨ p % 31 = 29) := by
  rw [← digit8TrailingWidth5_reverseOnlyLefts_eq_filter_mod31]
  simp [digit8TrailingWidth5Mod31ReverseOnlyClassifier]
  tauto

theorem digit8TrailingWidth5_reverseOnly_multiModulusClassifier (p : ℕ) :
    (p ∈ digit8TrailingWidth5ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 19 = 6 ∨ p % 19 = 10 ∨ p % 19 = 11 ∨ p % 19 = 12)) ∧
    (p ∈ digit8TrailingWidth5ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 29 = 0 ∨ p % 29 = 11 ∨ p % 29 = 14 ∨ p % 29 = 20)) ∧
    (p ∈ digit8TrailingWidth5ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 31 = 8 ∨ p % 31 = 11 ∨ p % 31 = 14 ∨ p % 31 = 29)) := by
  exact
    ⟨digit8TrailingWidth5_reverseOnly_mem_iff_mod19 p,
      digit8TrailingWidth5_reverseOnly_mem_iff_mod29 p,
      digit8TrailingWidth5_reverseOnly_mem_iff_mod31 p⟩

def digit8TrailingWidth6Connector : ℕ := 8

def digit8TrailingWidth6ReverseOnlyTwinLefts : List ℕ :=
  [101, 137, 149]

def digit8TrailingWidth6Mod17ReverseOnlyClassifier (p : ℕ) : Bool :=
  p % 17 == 1 || p % 17 == 13 || p % 17 == 16

theorem digit8TrailingWidth6_reverseOnlyLefts_eq_filter_mod17 :
    digit8LeadingWidth6ScannedTwinLefts.filter digit8TrailingWidth6Mod17ReverseOnlyClassifier =
      digit8TrailingWidth6ReverseOnlyTwinLefts := by
  native_decide

theorem digit8TrailingWidth6_reverseOnly_mem_iff_mod17 (p : ℕ) :
    p ∈ digit8TrailingWidth6ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 17 = 1 ∨ p % 17 = 13 ∨ p % 17 = 16) := by
  rw [← digit8TrailingWidth6_reverseOnlyLefts_eq_filter_mod17]
  simp [digit8TrailingWidth6Mod17ReverseOnlyClassifier]
  tauto

def digit8TrailingWidth6Mod19ReverseOnlyClassifier (p : ℕ) : Bool :=
  p % 19 == 4 || p % 19 == 6 || p % 19 == 16

theorem digit8TrailingWidth6_reverseOnlyLefts_eq_filter_mod19 :
    digit8LeadingWidth6ScannedTwinLefts.filter digit8TrailingWidth6Mod19ReverseOnlyClassifier =
      digit8TrailingWidth6ReverseOnlyTwinLefts := by
  native_decide

theorem digit8TrailingWidth6_reverseOnly_mem_iff_mod19 (p : ℕ) :
    p ∈ digit8TrailingWidth6ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 19 = 4 ∨ p % 19 = 6 ∨ p % 19 = 16) := by
  rw [← digit8TrailingWidth6_reverseOnlyLefts_eq_filter_mod19]
  simp [digit8TrailingWidth6Mod19ReverseOnlyClassifier]
  tauto

def digit8TrailingWidth6Mod29ReverseOnlyClassifier (p : ℕ) : Bool :=
  p % 29 == 4 || p % 29 == 14 || p % 29 == 21

theorem digit8TrailingWidth6_reverseOnlyLefts_eq_filter_mod29 :
    digit8LeadingWidth6ScannedTwinLefts.filter digit8TrailingWidth6Mod29ReverseOnlyClassifier =
      digit8TrailingWidth6ReverseOnlyTwinLefts := by
  native_decide

theorem digit8TrailingWidth6_reverseOnly_mem_iff_mod29 (p : ℕ) :
    p ∈ digit8TrailingWidth6ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 29 = 4 ∨ p % 29 = 14 ∨ p % 29 = 21) := by
  rw [← digit8TrailingWidth6_reverseOnlyLefts_eq_filter_mod29]
  simp [digit8TrailingWidth6Mod29ReverseOnlyClassifier]
  tauto

def digit8TrailingWidth6Mod31ReverseOnlyClassifier (p : ℕ) : Bool :=
  p % 31 == 8 || p % 31 == 13 || p % 31 == 25

theorem digit8TrailingWidth6_reverseOnlyLefts_eq_filter_mod31 :
    digit8LeadingWidth6ScannedTwinLefts.filter digit8TrailingWidth6Mod31ReverseOnlyClassifier =
      digit8TrailingWidth6ReverseOnlyTwinLefts := by
  native_decide

theorem digit8TrailingWidth6_reverseOnly_mem_iff_mod31 (p : ℕ) :
    p ∈ digit8TrailingWidth6ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 31 = 8 ∨ p % 31 = 13 ∨ p % 31 = 25) := by
  rw [← digit8TrailingWidth6_reverseOnlyLefts_eq_filter_mod31]
  simp [digit8TrailingWidth6Mod31ReverseOnlyClassifier]
  tauto

theorem digit8TrailingWidth6_reverseOnly_multiModulusClassifier (p : ℕ) :
    (p ∈ digit8TrailingWidth6ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 17 = 1 ∨ p % 17 = 13 ∨ p % 17 = 16)) ∧
    (p ∈ digit8TrailingWidth6ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 19 = 4 ∨ p % 19 = 6 ∨ p % 19 = 16)) ∧
    (p ∈ digit8TrailingWidth6ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 29 = 4 ∨ p % 29 = 14 ∨ p % 29 = 21)) ∧
    (p ∈ digit8TrailingWidth6ReverseOnlyTwinLefts ↔
      p ∈ digit8LeadingWidth6ScannedTwinLefts ∧
        (p % 31 = 8 ∨ p % 31 = 13 ∨ p % 31 = 25)) := by
  exact
    ⟨digit8TrailingWidth6_reverseOnly_mem_iff_mod17 p,
      digit8TrailingWidth6_reverseOnly_mem_iff_mod19 p,
      digit8TrailingWidth6_reverseOnly_mem_iff_mod29 p,
      digit8TrailingWidth6_reverseOnly_mem_iff_mod31 p⟩

end PrimeArithmetic.Connector
