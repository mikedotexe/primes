import Mathlib
import PrimeArithmetic.Foundation.FinitePairing

namespace PrimeArithmetic.Symmetry.ModularReflection

open PrimeArithmetic.Foundation

/-!
Concrete modular reflection on `Fin base`.

For a positive modulus `base`, the canonical midpoint is `base / 2`, and the
mirror involution is the modular complement `r ↦ -r mod base`, represented on
natural residues as `r ↦ (base - r) % base`.

When `base` is even, the fixed points of this involution are exactly `0` and the
midpoint `base / 2`.
-/

def midpoint (base : ℕ) [NeZero base] : Fin base :=
  ⟨base / 2, Nat.div_lt_self (Nat.pos_of_ne_zero (NeZero.ne base)) (by decide : 1 < 2)⟩

def reflect (base : ℕ) [NeZero base] : Fin base → Fin base
  | r => ⟨(base - r.1) % base, Nat.mod_lt _ (Nat.pos_of_ne_zero (NeZero.ne base))⟩

@[simp] theorem reflect_zero (base : ℕ) [NeZero base] :
    reflect base (0 : Fin base) = 0 := by
  apply Fin.ext
  simp [reflect]

theorem reflect_val_of_pos {base : ℕ} [NeZero base] {r : Fin base}
    (hPos : 0 < r.1) :
    (reflect base r).1 = base - r.1 := by
  have hLt : base - r.1 < base := by
    omega
  simp [reflect, Nat.mod_eq_of_lt hLt]

theorem reflect_involutive (base : ℕ) [NeZero base] :
    Function.Involutive (reflect base) := by
  intro r
  by_cases hZero : r.1 = 0
  · apply Fin.ext
    simp [reflect, hZero]
  · have hPos : 0 < r.1 := Nat.pos_of_ne_zero hZero
    have hVal : (reflect base r).1 = base - r.1 :=
      reflect_val_of_pos (base := base) hPos
    have hPos' : 0 < (reflect base r).1 := by
      rw [hVal]
      omega
    have hVal' : (reflect base (reflect base r)).1 = base - (reflect base r).1 :=
      reflect_val_of_pos (base := base) hPos'
    apply Fin.ext
    rw [hVal', hVal]
    omega

theorem reflect_midpoint {base : ℕ} [NeZero base] (hEven : Even base) :
    reflect base (midpoint base) = midpoint base := by
  have hBasePos : 0 < base := Nat.pos_of_ne_zero (NeZero.ne base)
  rcases hEven with ⟨k, hk⟩
  subst hk
  apply Fin.ext
  have hkPos : 0 < k := by
    omega
  have hLt : k + k - (k + k) / 2 < k + k := by
    omega
  simp [reflect, midpoint, Nat.mod_eq_of_lt hLt]
  omega

def symmetryData (base : ℕ) [NeZero base] (hEven : Even base) : SymmetryData (Fin base) where
  mid := midpoint base
  inv := reflect base
  invInvolutive := reflect_involutive base
  invMid := reflect_midpoint (base := base) hEven

theorem eq_zero_or_eq_midpoint_of_fixed {base : ℕ} [NeZero base]
    (r : Fin base) (hFix : reflect base r = r) :
    r = 0 ∨ r = midpoint base := by
  by_cases hZero : r.1 = 0
  · left
    apply Fin.ext
    simp [hZero]
  · right
    have hPos : 0 < r.1 := Nat.pos_of_ne_zero hZero
    have hVal : (reflect base r).1 = base - r.1 :=
      reflect_val_of_pos (base := base) hPos
    have hEq : base - r.1 = r.1 := by
      have hFixVal : (reflect base r).1 = r.1 := congrArg Fin.val hFix
      rw [hVal] at hFixVal
      exact hFixVal
    apply Fin.ext
    simp [midpoint]
    omega

theorem reflect_ne_self_of_ne_zero_ne_midpoint {base : ℕ} [NeZero base]
    (r : Fin base) (hZero : r ≠ 0) (hMid : r ≠ midpoint base) :
    reflect base r ≠ r := by
  intro hFix
  rcases eq_zero_or_eq_midpoint_of_fixed (base := base) r hFix with h | h
  · exact hZero h
  · exact hMid h

end PrimeArithmetic.Symmetry.ModularReflection
