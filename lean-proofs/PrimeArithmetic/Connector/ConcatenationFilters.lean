import Mathlib

namespace PrimeArithmetic.Connector

/-!
Exact fixed-width connector arithmetic.

This module formalizes the arithmetic core behind connector scans:

- forward and reverse base-`b` concatenation with a fixed connector width,
- residue reduction when `base ≡ 1 (mod m)`,
- decimal `mod 3` and `mod 9` filters for the canonical pair
  `(10301, 3007003007003)`.

Arithmetic-first connector vocabulary:

- a bounded scan entry is a connector hit `(pair, width, position, digit, direction)`,
- this file covers the exact `ResidueAdmissible` layer,
- broader `DirectionalAsymmetry` claims live above this file and remain open.

The statements are purely arithmetic. They do not depend on any physics or
Lagrange-language layer.
-/

/-- `left || connector || right` in base `base`, with fixed right width and connector width. -/
def concatForward (base left right connector rightWidth connWidth : ℕ) : ℕ :=
  left * base ^ (connWidth + rightWidth) + connector * base ^ rightWidth + right

/-- `right || connector || left` in base `base`, with fixed left width and connector width. -/
def concatReverse (base left right connector leftWidth connWidth : ℕ) : ℕ :=
  right * base ^ (connWidth + leftWidth) + connector * base ^ leftWidth + left

theorem concatForward_modEq_sum_of_base_modEq_one
    {base modulus left right connector rightWidth connWidth : ℕ}
    (hBase : base ≡ 1 [MOD modulus]) :
    concatForward base left right connector rightWidth connWidth ≡
      left + connector + right [MOD modulus] := by
  unfold concatForward
  have hPow1 : base ^ (connWidth + rightWidth) ≡ 1 [MOD modulus] := by
    simpa using hBase.pow (connWidth + rightWidth)
  have hPow2 : base ^ rightWidth ≡ 1 [MOD modulus] := by
    simpa using hBase.pow rightWidth
  calc
    left * base ^ (connWidth + rightWidth) + connector * base ^ rightWidth + right
      ≡ left * 1 + (connector * 1 + right) [MOD modulus] := by
        simpa [Nat.add_assoc] using
          (((Nat.ModEq.refl left).mul hPow1).add
            (((Nat.ModEq.refl connector).mul hPow2).add (Nat.ModEq.refl right)))
    _ ≡ left + connector + right [MOD modulus] := by
        simpa [Nat.mul_one, Nat.add_assoc] using
          (Nat.ModEq.refl (left + (connector + right)))

theorem concatReverse_modEq_sum_of_base_modEq_one
    {base modulus left right connector leftWidth connWidth : ℕ}
    (hBase : base ≡ 1 [MOD modulus]) :
    concatReverse base left right connector leftWidth connWidth ≡
      left + connector + right [MOD modulus] := by
  unfold concatReverse
  have hPow1 : base ^ (connWidth + leftWidth) ≡ 1 [MOD modulus] := by
    simpa using hBase.pow (connWidth + leftWidth)
  have hPow2 : base ^ leftWidth ≡ 1 [MOD modulus] := by
    simpa using hBase.pow leftWidth
  calc
    right * base ^ (connWidth + leftWidth) + connector * base ^ leftWidth + left
      ≡ right * 1 + (connector * 1 + left) [MOD modulus] := by
        simpa [Nat.add_assoc] using
          (((Nat.ModEq.refl right).mul hPow1).add
            (((Nat.ModEq.refl connector).mul hPow2).add (Nat.ModEq.refl left)))
    _ ≡ left + connector + right [MOD modulus] := by
        simpa [Nat.mul_one, Nat.add_assoc, Nat.add_left_comm, Nat.add_comm] using
          (Nat.ModEq.refl (left + (right + connector)))

theorem forward_reverse_modEq_of_base_modEq_one
    {base modulus left right connector leftWidth rightWidth connWidth : ℕ}
    (hBase : base ≡ 1 [MOD modulus]) :
    concatForward base left right connector rightWidth connWidth ≡
      concatReverse base left right connector leftWidth connWidth [MOD modulus] := by
  exact (concatForward_modEq_sum_of_base_modEq_one hBase).trans
    (concatReverse_modEq_sum_of_base_modEq_one hBase).symm

theorem concatForward_same_mod_across_widths_of_base_modEq_one
    {base modulus left right connector rightWidth connWidth₁ connWidth₂ : ℕ}
    (hBase : base ≡ 1 [MOD modulus]) :
    concatForward base left right connector rightWidth connWidth₁ ≡
      concatForward base left right connector rightWidth connWidth₂ [MOD modulus] := by
  exact
    (concatForward_modEq_sum_of_base_modEq_one
      (base := base) (modulus := modulus)
      (left := left) (right := right)
      (connector := connector) (rightWidth := rightWidth)
      (connWidth := connWidth₁) hBase).trans
      (concatForward_modEq_sum_of_base_modEq_one
        (base := base) (modulus := modulus)
        (left := left) (right := right)
        (connector := connector) (rightWidth := rightWidth)
        (connWidth := connWidth₂) hBase).symm

theorem concatReverse_same_mod_across_widths_of_base_modEq_one
    {base modulus left right connector leftWidth connWidth₁ connWidth₂ : ℕ}
    (hBase : base ≡ 1 [MOD modulus]) :
    concatReverse base left right connector leftWidth connWidth₁ ≡
      concatReverse base left right connector leftWidth connWidth₂ [MOD modulus] := by
  exact
    (concatReverse_modEq_sum_of_base_modEq_one
      (base := base) (modulus := modulus)
      (left := left) (right := right)
      (connector := connector) (leftWidth := leftWidth)
      (connWidth := connWidth₁) hBase).trans
      (concatReverse_modEq_sum_of_base_modEq_one
        (base := base) (modulus := modulus)
        (left := left) (right := right)
        (connector := connector) (leftWidth := leftWidth)
        (connWidth := connWidth₂) hBase).symm

theorem concatForward_modEq_target_iff_across_widths_of_base_modEq_one
    {base modulus left right connector rightWidth connWidth₁ connWidth₂ target : ℕ}
    (hBase : base ≡ 1 [MOD modulus]) :
    concatForward base left right connector rightWidth connWidth₁ ≡ target [MOD modulus] ↔
      concatForward base left right connector rightWidth connWidth₂ ≡ target [MOD modulus] := by
  constructor
  · intro hWidth₁
    exact
      (concatForward_same_mod_across_widths_of_base_modEq_one
        (base := base) (modulus := modulus)
        (left := left) (right := right)
        (connector := connector) (rightWidth := rightWidth)
        (connWidth₁ := connWidth₂) (connWidth₂ := connWidth₁) hBase).trans hWidth₁
  · intro hWidth₂
    exact
      (concatForward_same_mod_across_widths_of_base_modEq_one
        (base := base) (modulus := modulus)
        (left := left) (right := right)
        (connector := connector) (rightWidth := rightWidth)
        (connWidth₁ := connWidth₁) (connWidth₂ := connWidth₂) hBase).trans hWidth₂

theorem concatReverse_modEq_target_iff_across_widths_of_base_modEq_one
    {base modulus left right connector leftWidth connWidth₁ connWidth₂ target : ℕ}
    (hBase : base ≡ 1 [MOD modulus]) :
    concatReverse base left right connector leftWidth connWidth₁ ≡ target [MOD modulus] ↔
      concatReverse base left right connector leftWidth connWidth₂ ≡ target [MOD modulus] := by
  constructor
  · intro hWidth₁
    exact
      (concatReverse_same_mod_across_widths_of_base_modEq_one
        (base := base) (modulus := modulus)
        (left := left) (right := right)
        (connector := connector) (leftWidth := leftWidth)
        (connWidth₁ := connWidth₂) (connWidth₂ := connWidth₁) hBase).trans hWidth₁
  · intro hWidth₂
    exact
      (concatReverse_same_mod_across_widths_of_base_modEq_one
        (base := base) (modulus := modulus)
        (left := left) (right := right)
        (connector := connector) (leftWidth := leftWidth)
        (connWidth₁ := connWidth₁) (connWidth₂ := connWidth₂) hBase).trans hWidth₂

theorem base10_modEq_one_mod3 : 10 ≡ 1 [MOD 3] := by
  native_decide

theorem base10_modEq_one_mod9 : 10 ≡ 1 [MOD 9] := by
  native_decide

def canonicalLeft : ℕ := 10301

def canonicalRight : ℕ := 3007003007003

theorem canonicalPairSum_mod3 : canonicalLeft + canonicalRight ≡ 1 [MOD 3] := by
  native_decide

theorem canonicalPairSum_mod9 : canonicalLeft + canonicalRight ≡ 1 [MOD 9] := by
  native_decide

theorem canonical_concatForward_mod3 (connector connWidth : ℕ) :
    concatForward 10 canonicalLeft canonicalRight connector 13 connWidth ≡
      connector + 1 [MOD 3] := by
  refine
    (concatForward_modEq_sum_of_base_modEq_one
      (left := canonicalLeft) (right := canonicalRight)
      (connector := connector) (rightWidth := 13) (connWidth := connWidth)
      base10_modEq_one_mod3).trans ?_
  simpa [Nat.add_assoc, Nat.add_left_comm, Nat.add_comm] using
    Nat.ModEq.add_left connector canonicalPairSum_mod3

theorem canonical_concatReverse_mod3 (connector connWidth : ℕ) :
    concatReverse 10 canonicalLeft canonicalRight connector 5 connWidth ≡
      connector + 1 [MOD 3] := by
  refine
    (concatReverse_modEq_sum_of_base_modEq_one
      (left := canonicalLeft) (right := canonicalRight)
      (connector := connector) (leftWidth := 5) (connWidth := connWidth)
      base10_modEq_one_mod3).trans ?_
  simpa [Nat.add_assoc, Nat.add_left_comm, Nat.add_comm] using
    Nat.ModEq.add_left connector canonicalPairSum_mod3

theorem canonical_concatForward_mod9 (connector connWidth : ℕ) :
    concatForward 10 canonicalLeft canonicalRight connector 13 connWidth ≡
      connector + 1 [MOD 9] := by
  refine
    (concatForward_modEq_sum_of_base_modEq_one
      (left := canonicalLeft) (right := canonicalRight)
      (connector := connector) (rightWidth := 13) (connWidth := connWidth)
      base10_modEq_one_mod9).trans ?_
  simpa [Nat.add_assoc, Nat.add_left_comm, Nat.add_comm] using
    Nat.ModEq.add_left connector canonicalPairSum_mod9

theorem canonical_concatReverse_mod9 (connector connWidth : ℕ) :
    concatReverse 10 canonicalLeft canonicalRight connector 5 connWidth ≡
      connector + 1 [MOD 9] := by
  refine
    (concatReverse_modEq_sum_of_base_modEq_one
      (left := canonicalLeft) (right := canonicalRight)
      (connector := connector) (leftWidth := 5) (connWidth := connWidth)
      base10_modEq_one_mod9).trans ?_
  simpa [Nat.add_assoc, Nat.add_left_comm, Nat.add_comm] using
    Nat.ModEq.add_left connector canonicalPairSum_mod9

theorem canonical_concatForward_divisibleBy3_iff_connector_mod2
    (connector connWidth : ℕ) :
    concatForward 10 canonicalLeft canonicalRight connector 13 connWidth ≡ 0 [MOD 3] ↔
      connector ≡ 2 [MOD 3] := by
  constructor
  · intro h
    have hsum : connector + 1 ≡ 0 [MOD 3] :=
      (canonical_concatForward_mod3 connector connWidth).symm.trans h
    have hshift : connector + 1 ≡ 2 + 1 [MOD 3] := by
      exact hsum.trans (by native_decide : 0 ≡ 2 + 1 [MOD 3])
    simpa using Nat.ModEq.add_right_cancel' 1 hshift
  · intro h
    have hshift : connector + 1 ≡ 2 + 1 [MOD 3] := Nat.ModEq.add_right 1 h
    have hsum : connector + 1 ≡ 0 [MOD 3] := by
      exact hshift.trans (by native_decide : 2 + 1 ≡ 0 [MOD 3])
    exact (canonical_concatForward_mod3 connector connWidth).trans hsum

theorem canonical_concatReverse_divisibleBy3_iff_connector_mod2
    (connector connWidth : ℕ) :
    concatReverse 10 canonicalLeft canonicalRight connector 5 connWidth ≡ 0 [MOD 3] ↔
      connector ≡ 2 [MOD 3] := by
  constructor
  · intro h
    have hsum : connector + 1 ≡ 0 [MOD 3] :=
      (canonical_concatReverse_mod3 connector connWidth).symm.trans h
    have hshift : connector + 1 ≡ 2 + 1 [MOD 3] := by
      exact hsum.trans (by native_decide : 0 ≡ 2 + 1 [MOD 3])
    simpa using Nat.ModEq.add_right_cancel' 1 hshift
  · intro h
    have hshift : connector + 1 ≡ 2 + 1 [MOD 3] := Nat.ModEq.add_right 1 h
    have hsum : connector + 1 ≡ 0 [MOD 3] := by
      exact hshift.trans (by native_decide : 2 + 1 ≡ 0 [MOD 3])
    exact (canonical_concatReverse_mod3 connector connWidth).trans hsum

theorem canonical_concatForward_divisibleBy9_iff_connector_mod8
    (connector connWidth : ℕ) :
    concatForward 10 canonicalLeft canonicalRight connector 13 connWidth ≡ 0 [MOD 9] ↔
      connector ≡ 8 [MOD 9] := by
  constructor
  · intro h
    have hsum : connector + 1 ≡ 0 [MOD 9] :=
      (canonical_concatForward_mod9 connector connWidth).symm.trans h
    have hshift : connector + 1 ≡ 8 + 1 [MOD 9] := by
      exact hsum.trans (by native_decide : 0 ≡ 8 + 1 [MOD 9])
    simpa using Nat.ModEq.add_right_cancel' 1 hshift
  · intro h
    have hshift : connector + 1 ≡ 8 + 1 [MOD 9] := Nat.ModEq.add_right 1 h
    have hsum : connector + 1 ≡ 0 [MOD 9] := by
      exact hshift.trans (by native_decide : 8 + 1 ≡ 0 [MOD 9])
    exact (canonical_concatForward_mod9 connector connWidth).trans hsum

theorem canonical_concatReverse_divisibleBy9_iff_connector_mod8
    (connector connWidth : ℕ) :
    concatReverse 10 canonicalLeft canonicalRight connector 5 connWidth ≡ 0 [MOD 9] ↔
      connector ≡ 8 [MOD 9] := by
  constructor
  · intro h
    have hsum : connector + 1 ≡ 0 [MOD 9] :=
      (canonical_concatReverse_mod9 connector connWidth).symm.trans h
    have hshift : connector + 1 ≡ 8 + 1 [MOD 9] := by
      exact hsum.trans (by native_decide : 0 ≡ 8 + 1 [MOD 9])
    simpa using Nat.ModEq.add_right_cancel' 1 hshift
  · intro h
    have hshift : connector + 1 ≡ 8 + 1 [MOD 9] := Nat.ModEq.add_right 1 h
    have hsum : connector + 1 ≡ 0 [MOD 9] := by
      exact hshift.trans (by native_decide : 8 + 1 ≡ 0 [MOD 9])
    exact (canonical_concatReverse_mod9 connector connWidth).trans hsum

theorem canonical_forward_reverse_same_mod3 (connector connWidth : ℕ) :
    concatForward 10 canonicalLeft canonicalRight connector 13 connWidth ≡
      concatReverse 10 canonicalLeft canonicalRight connector 5 connWidth [MOD 3] := by
  exact forward_reverse_modEq_of_base_modEq_one
    (left := canonicalLeft) (right := canonicalRight)
    (connector := connector) (leftWidth := 5) (rightWidth := 13)
    (connWidth := connWidth) base10_modEq_one_mod3

theorem canonical_forward_reverse_same_mod9 (connector connWidth : ℕ) :
    concatForward 10 canonicalLeft canonicalRight connector 13 connWidth ≡
      concatReverse 10 canonicalLeft canonicalRight connector 5 connWidth [MOD 9] := by
  exact forward_reverse_modEq_of_base_modEq_one
    (left := canonicalLeft) (right := canonicalRight)
    (connector := connector) (leftWidth := 5) (rightWidth := 13)
    (connWidth := connWidth) base10_modEq_one_mod9

end PrimeArithmetic.Connector
