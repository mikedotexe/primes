import Mathlib
import PrimeArithmetic.Density.UnitResidues

namespace PrimeArithmetic.Density

/-!
This module connects the repo-local `unitResidues` finset to the standard Lean
number-theory surface: units of `ZMod base`.
-/

theorem unitResidue_isUnit
    {base a : ℕ} [NeZero base] (hmem : a ∈ unitResidues base) :
    IsUnit (a : ZMod base) := by
  exact (ZMod.isUnit_iff_coprime a base).2 (mem_unitResidues.mp hmem).2.symm

theorem mem_unitResidues_iff_isUnit
    {base a : ℕ} [NeZero base] (hLt : a < base) :
    a ∈ unitResidues base ↔ IsUnit (a : ZMod base) := by
  constructor
  · exact unitResidue_isUnit
  · intro hUnit
    exact mem_unitResidues.2 ⟨hLt, ((ZMod.isUnit_iff_coprime a base).1 hUnit).symm⟩

def unitResiduesEquivCoprimeZMod
    {base : ℕ} [NeZero base] :
    { a : ℕ // a ∈ unitResidues base } ≃ { x : ZMod base // Nat.Coprime x.val base } where
  toFun a := by
    refine ⟨(a.1 : ZMod base), ?_⟩
    have hmem := mem_unitResidues.mp a.2
    rw [ZMod.val_natCast_of_lt hmem.1]
    exact hmem.2.symm
  invFun x := ⟨x.1.val, mem_unitResidues.2 ⟨x.1.val_lt, x.2.symm⟩⟩
  left_inv a := by
    apply Subtype.ext
    exact ZMod.val_natCast_of_lt (mem_unitResidues.mp a.2).1
  right_inv x := by
    apply Subtype.ext
    exact ZMod.natCast_zmod_val x.1

def unitResiduesEquivUnits
    {base : ℕ} [NeZero base] :
    { a : ℕ // a ∈ unitResidues base } ≃ (ZMod base)ˣ :=
  unitResiduesEquivCoprimeZMod.trans (ZMod.unitsEquivCoprime.symm)

theorem card_unitResidues_eq_card_units
    (base : ℕ) [NeZero base] :
    (unitResidues base).card = Fintype.card (ZMod base)ˣ := by
  rw [← Fintype.card_coe (unitResidues base)]
  exact Fintype.card_congr (unitResiduesEquivUnits (base := base))

theorem card_units_eq_totient_via_unitResidues
    (base : ℕ) [NeZero base] :
    Fintype.card (ZMod base)ˣ = Nat.totient base := by
  rw [← card_unitResidues_eq_card_units (base := base), card_unitResidues]

theorem primeGtBase_isUnit
    {p base : ℕ} [NeZero base]
    (hBase : 2 ≤ base) (hPrime : Nat.Prime p) (hGt : base < p) :
    IsUnit (p : ZMod base) := by
  exact (ZMod.isUnit_iff_coprime p base).2 <| primeGtBaseCoprime hBase hPrime hGt

theorem primeGtBaseMod_isUnit
    {p base : ℕ} [NeZero base]
    (hBase : 2 ≤ base) (hPrime : Nat.Prime p) (hGt : base < p) :
    IsUnit ((p % base : ℕ) : ZMod base) := by
  exact unitResidue_isUnit <| primeGtBaseMod_memUnitResidues hBase hPrime hGt

end PrimeArithmetic.Density
