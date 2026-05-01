import Mathlib
import PrimeArithmetic.Density.ZModUnits
import PrimeArithmetic.Density.UnitResidueSymmetry

namespace PrimeArithmetic.Density

/-!
Explicit complement-pair representatives for admissible residues.

For `base > 2`, admissible residues modulo `base` split into two-element
complement pairs `a ↔ base - a`. This module makes that picture explicit by
choosing the smaller element in each pair as a canonical representative.
-/

def unitResiduePairReps (base : ℕ) : Finset ℕ :=
  (unitResidues base).filter fun a => a < base - a

theorem mem_unitResiduePairReps {base a : ℕ} :
    a ∈ unitResiduePairReps base ↔ a ∈ unitResidues base ∧ a < base - a := by
  simp [unitResiduePairReps]

theorem complement_mem_unitResiduePairReps_of_not_lt
    {base a : ℕ} (hBase : 2 < base) (ha : a ∈ unitResidues base)
    (hlt : ¬ a < base - a) :
    base - a ∈ unitResiduePairReps base := by
  rw [mem_unitResiduePairReps]
  refine ⟨complement_mem_unitResidues (lt_trans one_lt_two hBase) ha, ?_⟩
  have hneq : a ≠ base - a := by
    exact fun hEq => complement_ne_self hBase ha hEq.symm
  have hlt' : base - a < a := by
    rcases lt_or_gt_of_ne hneq with hlt' | hgt'
    · exact False.elim (hlt hlt')
    · exact hgt'
  simpa [Nat.sub_sub_self (le_of_lt (mem_unitResidues.1 ha).1)] using hlt'

theorem mem_unitResidues_iff_mem_pairReps_or_complement_mem_pairReps
    {base a : ℕ} (hBase : 2 < base) :
    a ∈ unitResidues base ↔
      a ∈ unitResiduePairReps base ∨ base - a ∈ unitResiduePairReps base := by
  constructor
  · intro ha
    by_cases hlt : a < base - a
    · exact Or.inl <| mem_unitResiduePairReps.2 ⟨ha, hlt⟩
    · exact Or.inr <| complement_mem_unitResiduePairReps_of_not_lt hBase ha hlt
  · intro hPair
    rcases hPair with hPair | hPair
    · exact (mem_unitResiduePairReps.1 hPair).1
    · have hComp : base - a ∈ unitResidues base := (mem_unitResiduePairReps.1 hPair).1
      have haLt : a < base := by
        have hPos : 0 < base - a := unitResidue_pos (lt_trans one_lt_two hBase) hComp
        by_contra haGe
        have hZero : base - a = 0 := Nat.sub_eq_zero_of_le (not_lt.1 haGe)
        exact (Nat.lt_irrefl 0) (hZero ▸ hPos)
      simpa [Nat.sub_sub_self (le_of_lt haLt)] using
        complement_mem_unitResidues (lt_trans one_lt_two hBase) hComp

theorem not_mem_unitResiduePairReps_of_complement_mem_unitResiduePairReps
    {base a : ℕ} (hComp : base - a ∈ unitResiduePairReps base) :
    a ∉ unitResiduePairReps base := by
  intro ha
  have haLt : a < base - a := (mem_unitResiduePairReps.1 ha).2
  have hCompLt : base - a < base - (base - a) := (mem_unitResiduePairReps.1 hComp).2
  have haBase : a < base := (mem_unitResiduePairReps.1 ha).1 |> mem_unitResidues.1 |> And.left
  have hCompLt' : base - a < a := by
    simpa [Nat.sub_sub_self (le_of_lt haBase)] using hCompLt
  exact (not_lt_of_ge (le_of_lt haLt)) hCompLt'

theorem complement_not_mem_unitResiduePairReps_of_mem
    {base a : ℕ} (ha : a ∈ unitResiduePairReps base) :
    base - a ∉ unitResiduePairReps base := by
  intro hComp
  exact not_mem_unitResiduePairReps_of_complement_mem_unitResiduePairReps hComp ha

theorem xor_mem_unitResiduePairReps_complement
    {base a : ℕ} (hBase : 2 < base) (ha : a ∈ unitResidues base) :
    Xor' (a ∈ unitResiduePairReps base) (base - a ∈ unitResiduePairReps base) := by
  by_cases hPair : a ∈ unitResiduePairReps base
  · exact Or.inl ⟨hPair,
      complement_not_mem_unitResiduePairReps_of_mem hPair⟩
  · have hlt : ¬ a < base - a := by
      intro hlt
      exact hPair (mem_unitResiduePairReps.2 ⟨ha, hlt⟩)
    exact Or.inr ⟨complement_mem_unitResiduePairReps_of_not_lt hBase ha hlt, hPair⟩

theorem primeGtBaseMod_mem_unitResiduePairReps_or_complement
    {p base : ℕ} (hBase : 2 < base) (hPrime : Nat.Prime p) (hGt : base < p) :
    p % base ∈ unitResiduePairReps base ∨
      base - (p % base) ∈ unitResiduePairReps base := by
  exact (mem_unitResidues_iff_mem_pairReps_or_complement_mem_pairReps hBase).1 <|
    primeGtBaseMod_memUnitResidues (le_of_lt hBase) hPrime hGt

theorem primeGtBaseMod_xor_mem_unitResiduePairReps_complement
    {p base : ℕ} (hBase : 2 < base) (hPrime : Nat.Prime p) (hGt : base < p) :
    Xor' (p % base ∈ unitResiduePairReps base)
      (base - (p % base) ∈ unitResiduePairReps base) := by
  exact xor_mem_unitResiduePairReps_complement hBase <|
    primeGtBaseMod_memUnitResidues (le_of_lt hBase) hPrime hGt

def unitResiduePairEquiv
    (base : ℕ) (hBase : 2 < base) :
    ({ a : ℕ // a ∈ unitResiduePairReps base } × Bool) ≃
      { a : ℕ // a ∈ unitResidues base } where
  toFun
    | ⟨a, false⟩ => ⟨a.1, (mem_unitResiduePairReps.1 a.2).1⟩
    | ⟨a, true⟩ =>
        ⟨base - a.1, complement_mem_unitResidues (lt_trans one_lt_two hBase) <|
          (mem_unitResiduePairReps.1 a.2).1⟩
  invFun x := by
    by_cases hlt : x.1 < base - x.1
    · exact ⟨⟨x.1, mem_unitResiduePairReps.2 ⟨x.2, hlt⟩⟩, false⟩
    · exact ⟨⟨base - x.1, complement_mem_unitResiduePairReps_of_not_lt hBase x.2 hlt⟩, true⟩
  left_inv
    | ⟨a, false⟩ => by
        have hlt : a.1 < base - a.1 := (mem_unitResiduePairReps.1 a.2).2
        simp [hlt]
    | ⟨a, true⟩ => by
        have ha : a.1 ∈ unitResidues base := (mem_unitResiduePairReps.1 a.2).1
        have hlt : a.1 < base - a.1 := (mem_unitResiduePairReps.1 a.2).2
        have hnot : ¬ base - a.1 < a.1 := not_lt_of_ge (le_of_lt hlt)
        ext
        · simp [hnot,
            Nat.sub_sub_self (le_of_lt (mem_unitResidues.1 ha).1)]
        · simp [hnot,
            Nat.sub_sub_self (le_of_lt (mem_unitResidues.1 ha).1)]
  right_inv x := by
    by_cases hlt : x.1 < base - x.1
    · apply Subtype.ext
      simp [hlt]
    · apply Subtype.ext
      simp [hlt,
        Nat.sub_sub_self (le_of_lt (mem_unitResidues.1 x.2).1)]

def unitResiduePairEquivUnits
    (base : ℕ) [NeZero base] (hBase : 2 < base) :
    ({ a : ℕ // a ∈ unitResiduePairReps base } × Bool) ≃ (ZMod base)ˣ :=
  (unitResiduePairEquiv base hBase).trans (unitResiduesEquivUnits (base := base))

theorem card_unitResiduePairReps_mul_two
    {base : ℕ} (hBase : 2 < base) :
    (unitResiduePairReps base).card * 2 = (unitResidues base).card := by
  have hCard := Fintype.card_congr (unitResiduePairEquiv base hBase)
  simpa [Fintype.card_prod, Fintype.card_bool, Nat.mul_comm, Nat.mul_left_comm,
    Nat.mul_assoc] using hCard

theorem card_unitResiduePairReps_eq_totient_div_two
    {base : ℕ} (hBase : 2 < base) :
    (unitResiduePairReps base).card = Nat.totient base / 2 := by
  have hCard : (unitResiduePairReps base).card * 2 = Nat.totient base := by
    rw [card_unitResiduePairReps_mul_two hBase, card_unitResidues]
  omega

theorem card_zmodUnits_eq_pairReps_mul_two
    {base : ℕ} [NeZero base] (hBase : 2 < base) :
    Fintype.card (ZMod base)ˣ = (unitResiduePairReps base).card * 2 := by
  rw [← card_unitResidues_eq_card_units (base := base)]
  exact (card_unitResiduePairReps_mul_two hBase).symm

theorem card_zmodUnits_eq_totient
    {base : ℕ} [NeZero base] :
    Fintype.card (ZMod base)ˣ = Nat.totient base :=
  card_units_eq_totient_via_unitResidues base

theorem card_zmodUnits_div_two_eq_pairReps
    {base : ℕ} [NeZero base] (hBase : 2 < base) :
    Fintype.card (ZMod base)ˣ / 2 = (unitResiduePairReps base).card := by
  rw [card_zmodUnits_eq_totient]
  symm
  exact card_unitResiduePairReps_eq_totient_div_two hBase

end PrimeArithmetic.Density
