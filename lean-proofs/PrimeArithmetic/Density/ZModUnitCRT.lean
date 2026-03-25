import Mathlib
import PrimeArithmetic.Density.CoprimeBaseProducts

namespace PrimeArithmetic.Density

/-!
Explicit Chinese remainder structure on unit groups.

`PrimeArithmetic/Density/CoprimeBaseProducts` already established the product
decomposition on admissible residue representatives. This module restates the
same structure directly on the standard algebraic object `(ZMod n)ˣ`.
-/

def zmodUnitsMulEquiv
    {m n : ℕ} [NeZero m] [NeZero n] (h : m.Coprime n) :
    (ZMod (m * n))ˣ ≃ (ZMod m)ˣ × (ZMod n)ˣ := by
  letI : NeZero (m * n) := ⟨Nat.mul_ne_zero (NeZero.ne m) (NeZero.ne n)⟩
  exact (((Units.mapEquiv (ZMod.chineseRemainder h).toMulEquiv).trans
      MulEquiv.prodUnits).toEquiv)

theorem card_zmodUnits_mul_of_coprime
    {m n : ℕ} [NeZero m] [NeZero n] (h : m.Coprime n) :
    Fintype.card (ZMod (m * n))ˣ = Fintype.card (ZMod m)ˣ * Fintype.card (ZMod n)ˣ := by
  rw [Fintype.card_congr (zmodUnitsMulEquiv h), Fintype.card_prod]

def zmodUnits_six_equiv :
    (ZMod 6)ˣ ≃ (ZMod 2)ˣ × (ZMod 3)ˣ := by
  simpa using zmodUnitsMulEquiv (m := 2) (n := 3) (by decide)

def zmodUnits_ten_equiv :
    (ZMod 10)ˣ ≃ (ZMod 2)ˣ × (ZMod 5)ˣ := by
  simpa using zmodUnitsMulEquiv (m := 2) (n := 5) (by decide)

def zmodUnits_twelve_equiv :
    (ZMod 12)ˣ ≃ (ZMod 3)ˣ × (ZMod 4)ˣ := by
  simpa using zmodUnitsMulEquiv (m := 3) (n := 4) (by decide)

def zmodUnits_thirty_equiv :
    (ZMod 30)ˣ ≃ (ZMod 6)ˣ × (ZMod 5)ˣ := by
  simpa using zmodUnitsMulEquiv (m := 6) (n := 5) (by decide)

def zmodUnits_twoHundredTen_equiv :
    (ZMod 210)ˣ ≃ (ZMod 30)ˣ × (ZMod 7)ˣ := by
  simpa using zmodUnitsMulEquiv (m := 30) (n := 7) (by decide)

def zmodUnits_twoThousandThreeHundredTen_equiv :
    (ZMod 2310)ˣ ≃ (ZMod 210)ˣ × (ZMod 11)ˣ := by
  simpa using zmodUnitsMulEquiv (m := 210) (n := 11) (by decide)

theorem card_zmodUnits_twoHundredTen_split :
    Fintype.card (ZMod 210)ˣ = Fintype.card (ZMod 30)ˣ * Fintype.card (ZMod 7)ˣ := by
  simpa using card_zmodUnits_mul_of_coprime (m := 30) (n := 7) (by decide)

theorem card_zmodUnits_twoThousandThreeHundredTen_split :
    Fintype.card (ZMod 2310)ˣ = Fintype.card (ZMod 210)ˣ * Fintype.card (ZMod 11)ˣ := by
  simpa using card_zmodUnits_mul_of_coprime (m := 210) (n := 11) (by decide)

end PrimeArithmetic.Density
