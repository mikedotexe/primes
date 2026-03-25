import Mathlib
import PrimeArithmetic.Density.ZModUnits

namespace PrimeArithmetic.Density

/-!
CRT/product structure for coprime bases.

For coprime `m` and `n`, admissible residue classes modulo `m * n` decompose
as pairs of admissible residue classes modulo `m` and modulo `n`.
-/

def unitResiduesMulEquiv
    {m n : ℕ} (h : m.Coprime n) [NeZero m] [NeZero n] [NeZero (m * n)] :
    { a : ℕ // a ∈ unitResidues (m * n) } ≃
      { a : ℕ // a ∈ unitResidues m } × { b : ℕ // b ∈ unitResidues n } :=
  (unitResiduesEquivUnits (base := m * n)).trans
    ((((Units.mapEquiv (ZMod.chineseRemainder h).toMulEquiv).trans MulEquiv.prodUnits).toEquiv).trans
      (Equiv.prodCongr
        (unitResiduesEquivUnits (base := m)).symm
        (unitResiduesEquivUnits (base := n)).symm))

theorem card_unitResidues_mul_of_coprime
    {m n : ℕ} (h : m.Coprime n) [NeZero m] [NeZero n] [NeZero (m * n)] :
    (unitResidues (m * n)).card = (unitResidues m).card * (unitResidues n).card := by
  rw [← Fintype.card_coe (unitResidues (m * n))]
  rw [Fintype.card_congr (unitResiduesMulEquiv h), Fintype.card_prod]
  rw [Fintype.card_coe, Fintype.card_coe]

theorem card_units_mul_of_coprime
    {m n : ℕ} (h : m.Coprime n) [NeZero m] [NeZero n] [NeZero (m * n)] :
    Fintype.card (ZMod (m * n))ˣ = Fintype.card (ZMod m)ˣ * Fintype.card (ZMod n)ˣ := by
  rw [← card_unitResidues_eq_card_units (base := m * n)]
  rw [← card_unitResidues_eq_card_units (base := m)]
  rw [← card_unitResidues_eq_card_units (base := n)]
  exact card_unitResidues_mul_of_coprime h

theorem card_unitResidues_six_split :
    (unitResidues 6).card = (unitResidues 2).card * (unitResidues 3).card := by
  simpa using card_unitResidues_mul_of_coprime (m := 2) (n := 3) (by decide)

theorem card_unitResidues_ten_split :
    (unitResidues 10).card = (unitResidues 2).card * (unitResidues 5).card := by
  simpa using card_unitResidues_mul_of_coprime (m := 2) (n := 5) (by decide)

theorem card_unitResidues_twelve_split :
    (unitResidues 12).card = (unitResidues 3).card * (unitResidues 4).card := by
  simpa using card_unitResidues_mul_of_coprime (m := 3) (n := 4) (by decide)

theorem card_unitResidues_thirty_split :
    (unitResidues 30).card = (unitResidues 6).card * (unitResidues 5).card := by
  simpa using card_unitResidues_mul_of_coprime (m := 6) (n := 5) (by decide)

theorem radical_twelve_split :
    radical 12 = radical 3 * radical 4 := by
  simpa using radical_mul_of_coprime (m := 3) (n := 4) (by decide)

theorem radical_thirty_split :
    radical 30 = radical 6 * radical 5 := by
  simpa using radical_mul_of_coprime (m := 6) (n := 5) (by decide)

end PrimeArithmetic.Density
