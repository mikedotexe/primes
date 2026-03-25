import Mathlib
import PrimeArithmetic.Density.ZModUnits

namespace PrimeArithmetic.Density

/-!
Negation symmetry on units of `ZMod n`.

This module states the midpoint/complement story in the most standard modular
language currently available in the Lean package:

- units of `ZMod n`
- the involution `u ↦ -u`
- fixed-point-freeness for `n > 2`
- the resulting even-cardinality consequence
- midpoint exclusion for even moduli
-/

def negUnitsEquiv (n : ℕ) : (ZMod n)ˣ ≃ (ZMod n)ˣ where
  toFun u := -u
  invFun u := -u
  left_inv u := by simp
  right_inv u := by simp

@[simp] theorem negUnitsEquiv_apply {n : ℕ} (u : (ZMod n)ˣ) :
    negUnitsEquiv n u = -u :=
  rfl

theorem negUnit_ne_self_of_two_lt {n : ℕ} (hn : 2 < n) (u : (ZMod n)ˣ) :
    -u ≠ u := by
  intro hNeg
  have hSum : (u : ZMod n) + u = 0 := by
    exact neg_eq_iff_add_eq_zero.mp (by simpa using congrArg Units.val hNeg)
  have hMul : (2 : ZMod n) * (u : ZMod n) = 0 := by
    simpa [two_mul] using hSum
  have hTwoZero : (2 : ZMod n) = 0 := by
    have hInv := congrArg (fun x : ZMod n => x * ↑(u⁻¹)) hMul
    simpa [mul_assoc] using hInv
  have hVal : (2 : ZMod n).val = 2 := ZMod.val_natCast_of_lt hn
  have hVal' := hVal
  simp [hTwoZero] at hVal'

theorem negUnitsEquiv_ne_self_of_two_lt {n : ℕ} (hn : 2 < n) (u : (ZMod n)ˣ) :
    negUnitsEquiv n u ≠ u := by
  simpa using negUnit_ne_self_of_two_lt hn u

theorem card_units_even_of_two_lt {n : ℕ} [NeZero n] (hn : 2 < n) :
    Even (Fintype.card (ZMod n)ˣ) := by
  rw [ZMod.card_units_eq_totient]
  exact Nat.totient_even hn

theorem midpoint_not_isUnit {n : ℕ} (hn : 2 < n) (hEven : Even n) :
    ¬ IsUnit ((n / 2 : ℕ) : ZMod n) := by
  rcases hEven with ⟨k, rfl⟩
  intro hUnit
  have hk : 1 < k := by
    omega
  have hHalf : (k + k) / 2 = k := by
    rw [← Nat.two_mul, Nat.mul_div_right _ (by decide : 0 < 2)]
  have hUnitk : IsUnit ((k : ℕ) : ZMod (k + k)) := by
    simpa [hHalf] using hUnit
  have hCoprime : Nat.Coprime k (k + k) := by
    exact (ZMod.isUnit_iff_coprime k (k + k)).1 hUnitk
  exact
    (Nat.not_coprime_of_dvd_of_dvd hk dvd_rfl
      (show k ∣ k + k by exact dvd_add dvd_rfl dvd_rfl)) hCoprime

theorem midpoint_not_mem_unitResidues {n : ℕ} (hn : 2 < n) (hEven : Even n) :
    n / 2 ∉ unitResidues n := by
  haveI : NeZero n := ⟨Nat.ne_zero_of_lt (lt_trans Nat.zero_lt_two hn)⟩
  intro hMid
  exact midpoint_not_isUnit hn hEven (unitResidue_isUnit hMid)

theorem midpoint_not_isUnit_210 : ¬ IsUnit ((105 : ℕ) : ZMod 210) := by
  simpa using midpoint_not_isUnit (n := 210) (by decide) (show Even 210 by native_decide)

theorem midpoint_not_isUnit_2310 : ¬ IsUnit ((1155 : ℕ) : ZMod 2310) := by
  simpa using midpoint_not_isUnit (n := 2310) (by decide) (show Even 2310 by native_decide)

end PrimeArithmetic.Density
