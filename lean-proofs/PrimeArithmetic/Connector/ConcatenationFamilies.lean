import PrimeArithmetic.Connector.ConcatenationFilters

namespace PrimeArithmetic.Connector

/-!
Reusable residue-class families for fixed-width concatenation scans.

`ConcatenationFilters` proves the canonical decimal pair results directly.
This module extracts the general family pattern:

- fix a base and modulus with `base ≡ 1 (mod modulus)`,
- fix a left/right pair with known residue class,
- then forward and reverse concatenations reduce to the same connector-shifted
  residue class.

This gives reusable admissibility lemmas for entire connector families rather
than only one maintained pair.
-/

structure PairResidueProfile where
  base : ℕ
  modulus : ℕ
  left : ℕ
  right : ℕ
  leftWidth : ℕ
  rightWidth : ℕ
  pairResidue : ℕ
  baseModOne : base ≡ 1 [MOD modulus]
  pairSum : left + right ≡ pairResidue [MOD modulus]

theorem PairResidueProfile.concatForward_modEq_connector_shift
    (profile : PairResidueProfile) (connector connWidth : ℕ) :
    concatForward profile.base profile.left profile.right connector profile.rightWidth connWidth ≡
      connector + profile.pairResidue [MOD profile.modulus] := by
  refine
    (concatForward_modEq_sum_of_base_modEq_one
      (base := profile.base) (modulus := profile.modulus)
      (left := profile.left) (right := profile.right)
      (connector := connector) (rightWidth := profile.rightWidth)
      (connWidth := connWidth) profile.baseModOne).trans ?_
  simpa [Nat.add_assoc, Nat.add_left_comm, Nat.add_comm] using
    Nat.ModEq.add_left connector profile.pairSum

theorem PairResidueProfile.concatReverse_modEq_connector_shift
    (profile : PairResidueProfile) (connector connWidth : ℕ) :
    concatReverse profile.base profile.left profile.right connector profile.leftWidth connWidth ≡
      connector + profile.pairResidue [MOD profile.modulus] := by
  refine
    (concatReverse_modEq_sum_of_base_modEq_one
      (base := profile.base) (modulus := profile.modulus)
      (left := profile.left) (right := profile.right)
      (connector := connector) (leftWidth := profile.leftWidth)
      (connWidth := connWidth) profile.baseModOne).trans ?_
  simpa [Nat.add_assoc, Nat.add_left_comm, Nat.add_comm] using
    Nat.ModEq.add_left connector profile.pairSum

theorem PairResidueProfile.forward_reverse_same_mod
    (profile : PairResidueProfile) (connector connWidth : ℕ) :
    concatForward profile.base profile.left profile.right connector profile.rightWidth connWidth ≡
      concatReverse profile.base profile.left profile.right connector profile.leftWidth connWidth
        [MOD profile.modulus] := by
  exact
    (profile.concatForward_modEq_connector_shift connector connWidth).trans
      (profile.concatReverse_modEq_connector_shift connector connWidth).symm

theorem PairResidueProfile.concatForward_modEq_target_iff_connector_class
    (profile : PairResidueProfile) {target shift : ℕ}
    (hShift : shift + profile.pairResidue ≡ target [MOD profile.modulus])
    (connector connWidth : ℕ) :
    concatForward profile.base profile.left profile.right connector profile.rightWidth connWidth ≡
      target [MOD profile.modulus] ↔
      connector ≡ shift [MOD profile.modulus] := by
  constructor
  · intro h
    have hsum : connector + profile.pairResidue ≡ target [MOD profile.modulus] :=
      (profile.concatForward_modEq_connector_shift connector connWidth).symm.trans h
    have hCancel :
        connector + profile.pairResidue ≡ shift + profile.pairResidue [MOD profile.modulus] :=
      hsum.trans hShift.symm
    simpa [Nat.add_assoc, Nat.add_left_comm, Nat.add_comm] using
      Nat.ModEq.add_right_cancel' profile.pairResidue hCancel
  · intro h
    have hShifted :
        connector + profile.pairResidue ≡ shift + profile.pairResidue [MOD profile.modulus] :=
      Nat.ModEq.add_right profile.pairResidue h
    exact (profile.concatForward_modEq_connector_shift connector connWidth).trans
      (hShifted.trans hShift)

theorem PairResidueProfile.concatReverse_modEq_target_iff_connector_class
    (profile : PairResidueProfile) {target shift : ℕ}
    (hShift : shift + profile.pairResidue ≡ target [MOD profile.modulus])
    (connector connWidth : ℕ) :
    concatReverse profile.base profile.left profile.right connector profile.leftWidth connWidth ≡
      target [MOD profile.modulus] ↔
      connector ≡ shift [MOD profile.modulus] := by
  constructor
  · intro h
    have hsum : connector + profile.pairResidue ≡ target [MOD profile.modulus] :=
      (profile.concatReverse_modEq_connector_shift connector connWidth).symm.trans h
    have hCancel :
        connector + profile.pairResidue ≡ shift + profile.pairResidue [MOD profile.modulus] :=
      hsum.trans hShift.symm
    simpa [Nat.add_assoc, Nat.add_left_comm, Nat.add_comm] using
      Nat.ModEq.add_right_cancel' profile.pairResidue hCancel
  · intro h
    have hShifted :
        connector + profile.pairResidue ≡ shift + profile.pairResidue [MOD profile.modulus] :=
      Nat.ModEq.add_right profile.pairResidue h
    exact (profile.concatReverse_modEq_connector_shift connector connWidth).trans
      (hShifted.trans hShift)

theorem PairResidueProfile.concatForward_divisible_iff_connector_class
    (profile : PairResidueProfile) {shift : ℕ}
    (hShift : shift + profile.pairResidue ≡ 0 [MOD profile.modulus])
    (connector connWidth : ℕ) :
    concatForward profile.base profile.left profile.right connector profile.rightWidth connWidth ≡
      0 [MOD profile.modulus] ↔
      connector ≡ shift [MOD profile.modulus] :=
  profile.concatForward_modEq_target_iff_connector_class hShift connector connWidth

theorem PairResidueProfile.concatReverse_divisible_iff_connector_class
    (profile : PairResidueProfile) {shift : ℕ}
    (hShift : shift + profile.pairResidue ≡ 0 [MOD profile.modulus])
    (connector connWidth : ℕ) :
    concatReverse profile.base profile.left profile.right connector profile.leftWidth connWidth ≡
      0 [MOD profile.modulus] ↔
      connector ≡ shift [MOD profile.modulus] :=
  profile.concatReverse_modEq_target_iff_connector_class hShift connector connWidth

def canonicalProfileMod3 : PairResidueProfile where
  base := 10
  modulus := 3
  left := canonicalLeft
  right := canonicalRight
  leftWidth := 5
  rightWidth := 13
  pairResidue := 1
  baseModOne := base10_modEq_one_mod3
  pairSum := canonicalPairSum_mod3

def canonicalProfileMod9 : PairResidueProfile where
  base := 10
  modulus := 9
  left := canonicalLeft
  right := canonicalRight
  leftWidth := 5
  rightWidth := 13
  pairResidue := 1
  baseModOne := base10_modEq_one_mod9
  pairSum := canonicalPairSum_mod9

theorem canonicalProfileMod3_forward_divisible_iff_connector_mod2
    (connector connWidth : ℕ) :
    concatForward 10 canonicalLeft canonicalRight connector 13 connWidth ≡ 0 [MOD 3] ↔
      connector ≡ 2 [MOD 3] := by
  simpa [canonicalProfileMod3] using
    PairResidueProfile.concatForward_divisible_iff_connector_class
      canonicalProfileMod3
      (shift := 2)
      (by native_decide : 2 + canonicalProfileMod3.pairResidue ≡ 0 [MOD canonicalProfileMod3.modulus])
      connector connWidth

theorem canonicalProfileMod9_forward_divisible_iff_connector_mod8
    (connector connWidth : ℕ) :
    concatForward 10 canonicalLeft canonicalRight connector 13 connWidth ≡ 0 [MOD 9] ↔
      connector ≡ 8 [MOD 9] := by
  simpa [canonicalProfileMod9] using
    PairResidueProfile.concatForward_divisible_iff_connector_class
      canonicalProfileMod9
      (shift := 8)
      (by native_decide : 8 + canonicalProfileMod9.pairResidue ≡ 0 [MOD canonicalProfileMod9.modulus])
      connector connWidth

end PrimeArithmetic.Connector
