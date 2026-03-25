import PrimeArithmetic.Symmetry.MidpointObstruction
import PrimeArithmetic.Symmetry.UnitResidueComplementWitness
import PrimeArithmetic.Density.ZModUnitNegation

namespace PrimeArithmetic.Symmetry.ZModUnitNegationWitness

open PrimeArithmetic.Foundation
open PrimeArithmetic.Symmetry.UnitResidueComplementWitness

/-!
Direct symmetry witness from negation on units of `ZMod n`.

`UnitResidueComplementWitness` already constructs a midpoint-obstruction witness
from admissible residue representatives. This module transports that witness to
the standard algebraic index set `(ZMod n)ˣ` and uses negation `u ↦ -u` as the
pairing map itself.
-/

def asUnitResidue {base : ℕ} [NeZero base] (u : (ZMod base)ˣ) :
    { a : ℕ // a ∈ PrimeArithmetic.Density.unitResidues base } :=
  (PrimeArithmetic.Density.unitResiduesEquivUnits (base := base)).symm u

def asFin {base : ℕ} [NeZero base] (u : (ZMod base)ˣ) : Fin (base + 1) :=
  UnitResidueComplementWitness.asFin (asUnitResidue u)

@[simp] theorem unitResiduesEquivUnits_apply_val
    {base : ℕ} [NeZero base]
    (x : { a : ℕ // a ∈ PrimeArithmetic.Density.unitResidues base }) :
    (((PrimeArithmetic.Density.unitResiduesEquivUnits (base := base) x : (ZMod base)ˣ) : ZMod base)) =
      (x.1 : ZMod base) := by
  let y : { z : ZMod base // Nat.Coprime z.val base } :=
    PrimeArithmetic.Density.unitResiduesEquivCoprimeZMod x
  have hy : ((ZMod.unitsEquivCoprime.symm y : (ZMod base)ˣ) : ZMod base) = y.1 := by
    exact congrArg Subtype.val (ZMod.unitsEquivCoprime.apply_symm_apply y)
  simpa [PrimeArithmetic.Density.unitResiduesEquivUnits, y,
    PrimeArithmetic.Density.unitResiduesEquivCoprimeZMod] using hy

@[simp] theorem asUnitResidue_image {base : ℕ} [NeZero base] (u : (ZMod base)ˣ) :
    PrimeArithmetic.Density.unitResiduesEquivUnits (base := base) (asUnitResidue u) = u := by
  exact (PrimeArithmetic.Density.unitResiduesEquivUnits (base := base)).apply_symm_apply u

theorem asUnitResidue_cast_eq_unit {base : ℕ} [NeZero base] (u : (ZMod base)ˣ) :
    (((asUnitResidue u).1 : ℕ) : ZMod base) = (u : ZMod base) := by
  calc
    (((asUnitResidue u).1 : ℕ) : ZMod base) =
        (((PrimeArithmetic.Density.unitResiduesEquivUnits (base := base)
          (asUnitResidue u) : (ZMod base)ˣ) : ZMod base)) := by
      symm
      exact unitResiduesEquivUnits_apply_val (x := asUnitResidue u)
    _ = (u : ZMod base) := by
      exact congrArg Units.val (asUnitResidue_image (base := base) u)

theorem asUnitResidue_val_eq_zmod_val {base : ℕ} [NeZero base] (u : (ZMod base)ˣ) :
    (asUnitResidue u).1 = (u : ZMod base).val := by
  have hEq : (((asUnitResidue u).1 : ℕ) : ZMod base) = (u : ZMod base) :=
    asUnitResidue_cast_eq_unit u
  have hLt : (asUnitResidue u).1 < base :=
    (PrimeArithmetic.Density.mem_unitResidues.1 (asUnitResidue u).2).1
  have hValEq := congrArg ZMod.val hEq
  simp [ZMod.val_natCast_of_lt hLt] at hValEq
  exact hValEq

@[simp] theorem asFin_val {base : ℕ} [NeZero base] (u : (ZMod base)ˣ) :
    (asFin u).1 = (u : ZMod base).val := by
  unfold asFin UnitResidueComplementWitness.asFin
  simp [asUnitResidue_val_eq_zmod_val]

theorem neg_eq_mate_image {base : ℕ} [NeZero base] (hBase : 2 < base) (u : (ZMod base)ˣ) :
    PrimeArithmetic.Density.unitResiduesEquivUnits (base := base)
        (UnitResidueComplementWitness.mate hBase (asUnitResidue u)) = -u := by
  apply Units.ext
  have hLt : (asUnitResidue u).1 < base :=
    (PrimeArithmetic.Density.mem_unitResidues.1 (asUnitResidue u).2).1
  calc
    (((PrimeArithmetic.Density.unitResiduesEquivUnits (base := base)
        (UnitResidueComplementWitness.mate hBase (asUnitResidue u)) : (ZMod base)ˣ) : ZMod base)) =
        ((base - (asUnitResidue u).1 : ℕ) : ZMod base) := by
      simp [UnitResidueComplementWitness.mate]
    _ = - (((asUnitResidue u).1 : ℕ) : ZMod base) := by
      rw [Nat.cast_sub (le_of_lt hLt), ZMod.natCast_self, zero_sub]
    _ = - (u : ZMod base) := by
      rw [asUnitResidue_cast_eq_unit]

theorem asUnitResidue_neg_eq_mate {base : ℕ} [NeZero base] (hBase : 2 < base) (u : (ZMod base)ˣ) :
    asUnitResidue (-u) = UnitResidueComplementWitness.mate hBase (asUnitResidue u) := by
  apply (PrimeArithmetic.Density.unitResiduesEquivUnits (base := base)).injective
  calc
    PrimeArithmetic.Density.unitResiduesEquivUnits (base := base) (asUnitResidue (-u)) = -u := by
      exact asUnitResidue_image (-u)
    _ = PrimeArithmetic.Density.unitResiduesEquivUnits (base := base)
          (UnitResidueComplementWitness.mate hBase (asUnitResidue u)) := by
      symm
      exact neg_eq_mate_image (base := base) hBase u

theorem residue_equivariant_neg {base : ℕ} [NeZero base] (hBase : 2 < base) (hEven : Even base) :
    ∀ u : (ZMod base)ˣ, (symmetryData base hEven).inv (asFin u) = asFin (-u) := by
  intro u
  simpa [asFin, asUnitResidue_neg_eq_mate hBase u] using
    (UnitResidueComplementWitness.residue_equivariant
      (base := base) hBase hEven (asUnitResidue u))

theorem residue_distinct_neg {base : ℕ} [NeZero base] (hBase : 2 < base) :
    ∀ u : (ZMod base)ˣ, asFin (-u) ≠ asFin u := by
  intro u hEq
  have hEq' :
      UnitResidueComplementWitness.asFin
          (UnitResidueComplementWitness.mate hBase (asUnitResidue u)) =
        UnitResidueComplementWitness.asFin (asUnitResidue u) := by
    simpa [asFin, asUnitResidue_neg_eq_mate hBase u] using hEq
  exact UnitResidueComplementWitness.residue_distinct (base := base) hBase (asUnitResidue u) hEq'

def pairing {base : ℕ} [NeZero base] (hBase : 2 < base) (hEven : Even base) :
    PerfectPairing (symmetryData base hEven) (asFin (base := base)) where
  mate := fun u => -u
  mateInvolutive := by
    intro u
    simp
  noFixed := PrimeArithmetic.Density.negUnit_ne_self_of_two_lt hBase
  equivariant := residue_equivariant_neg hBase hEven
  residueDistinct := residue_distinct_neg hBase

theorem midpoint_not_in_zmodUnitRange {base : ℕ} [NeZero base]
    (hBase : 2 < base) (hEven : Even base) :
    midpointFin base ∉ Set.range (asFin (base := base)) :=
  PrimeArithmetic.Symmetry.midpointNotInRange (pairing hBase hEven)

theorem midpoint_not_isUnit_via_midpointObstruction {base : ℕ} [NeZero base]
    (hBase : 2 < base) (hEven : Even base) :
    ¬ IsUnit ((base / 2 : ℕ) : ZMod base) := by
  intro hUnit
  rcases hUnit with ⟨u, hu⟩
  have hHalfLt : base / 2 < base := by
    omega
  have huVal : (u : ZMod base).val = base / 2 := by
    have hValEq := congrArg ZMod.val hu
    simp [ZMod.val_natCast_of_lt hHalfLt] at hValEq
    exact hValEq
  have hRange : midpointFin base ∈ Set.range (asFin (base := base)) := by
    refine ⟨u, ?_⟩
    apply Fin.ext
    simp [midpointFin, huVal]
  exact midpoint_not_in_zmodUnitRange hBase hEven hRange

end PrimeArithmetic.Symmetry.ZModUnitNegationWitness
