import Mathlib
import PrimeArithmetic.Density.ZModUnitNegation
import PrimeArithmetic.Density.ZModUnitOrbits

namespace PrimeArithmetic.Density

/-!
Negation as an order-two group action on `(ZMod n)ˣ`.

`ZModUnitNegation` states the midpoint/complement story through the involution
`u ↦ -u`, and `ZModUnitOrbits` packages the same picture through canonical
pair representatives. This module inserts the standard group-action layer
between them by letting the order-two subgroup `{1, -1}` act on `(ZMod n)ˣ`
by multiplication.

The resulting orbit quotient is equivalent both to the canonical pair-representative
set and to the earlier quotient by negation-orbit representatives, so its
cardinality is again `φ(n) / 2`.
-/

def negationSubgroup (base : ℕ) : Subgroup (ZMod base)ˣ where
  carrier := { u | u = 1 ∨ u = -1 }
  one_mem' := Or.inl rfl
  mul_mem' := by
    intro a b ha hb
    rcases ha with rfl | rfl <;> rcases hb with rfl | rfl <;> simp
  inv_mem' := by
    intro a ha
    rcases ha with rfl | rfl <;> simp

theorem mem_negationSubgroup_iff {base : ℕ} {u : (ZMod base)ˣ} :
    u ∈ negationSubgroup base ↔ u = 1 ∨ u = -1 := by
  rfl

abbrev negationActionOrbitQuotient (base : ℕ) : Type :=
  Quotient (MulAction.orbitRel (negationSubgroup base) (ZMod base)ˣ)

noncomputable instance instFintypeNegationActionOrbitQuotient (base : ℕ) [NeZero base] :
    Fintype (negationActionOrbitQuotient base) := by
  classical
  infer_instance

noncomputable instance instFintypeNegationSubgroup (base : ℕ) [NeZero base] :
    Fintype (negationSubgroup base) := by
  classical
  infer_instance

@[simp] theorem unitResiduesEquivUnits_apply_val
    {base : ℕ} [NeZero base]
    (x : { a : ℕ // a ∈ unitResidues base }) :
    (((unitResiduesEquivUnits (base := base) x : (ZMod base)ˣ) : ZMod base)) =
      (x.1 : ZMod base) := by
  let y : { z : ZMod base // Nat.Coprime z.val base } := unitResiduesEquivCoprimeZMod x
  have hy : ((ZMod.unitsEquivCoprime.symm y : (ZMod base)ˣ) : ZMod base) = y.1 := by
    exact congrArg Subtype.val (ZMod.unitsEquivCoprime.apply_symm_apply y)
  simpa [unitResiduesEquivUnits, y, unitResiduesEquivCoprimeZMod] using hy

theorem unitResiduePairEquivUnits_neg_switch_false
    {base : ℕ} [NeZero base] (hBase : 2 < base)
    (r : { a : ℕ // a ∈ unitResiduePairReps base }) :
    unitResiduePairEquivUnits base hBase (r, true) =
      - unitResiduePairEquivUnits base hBase (r, false) := by
  apply Units.ext
  have hrLt : r.1 < base := (mem_unitResiduePairReps.1 r.2).1 |> mem_unitResidues.1 |> And.left
  calc
    (((unitResiduePairEquivUnits base hBase (r, true) : (ZMod base)ˣ) : ZMod base)) =
        ((base - r.1 : ℕ) : ZMod base) := by
          simp [unitResiduePairEquivUnits, unitResiduePairEquiv, unitResiduesEquivUnits_apply_val]
    _ = - ((r.1 : ℕ) : ZMod base) := by
          rw [Nat.cast_sub (le_of_lt hrLt), ZMod.natCast_self, zero_sub]
    _ = - (((unitResiduePairEquivUnits base hBase (r, false) : (ZMod base)ˣ) : ZMod base)) := by
          simp [unitResiduePairEquivUnits, unitResiduePairEquiv, unitResiduesEquivUnits_apply_val]

theorem unitResiduePairEquivUnits_neg_switch_true
    {base : ℕ} [NeZero base] (hBase : 2 < base)
    (r : { a : ℕ // a ∈ unitResiduePairReps base }) :
    unitResiduePairEquivUnits base hBase (r, false) =
      - unitResiduePairEquivUnits base hBase (r, true) := by
  apply Units.ext
  have hrLt : r.1 < base := (mem_unitResiduePairReps.1 r.2).1 |> mem_unitResidues.1 |> And.left
  calc
    (((unitResiduePairEquivUnits base hBase (r, false) : (ZMod base)ˣ) : ZMod base)) =
        (r.1 : ZMod base) := by
          simp [unitResiduePairEquivUnits, unitResiduePairEquiv, unitResiduesEquivUnits_apply_val]
    _ = - ((base - r.1 : ℕ) : ZMod base) := by
          rw [Nat.cast_sub (le_of_lt hrLt), ZMod.natCast_self]
          ring
    _ = - (((unitResiduePairEquivUnits base hBase (r, true) : (ZMod base)ˣ) : ZMod base)) := by
          simp [unitResiduePairEquivUnits, unitResiduePairEquiv, unitResiduesEquivUnits_apply_val]

theorem unitNegationOrbitRep_neg
    (base : ℕ) [NeZero base] (hBase : 2 < base) (u : (ZMod base)ˣ) :
    unitNegationOrbitRep base hBase (-u) = unitNegationOrbitRep base hBase u := by
  let x := (unitResiduePairEquivUnits base hBase).symm u
  have hx : (unitResiduePairEquivUnits base hBase).symm u = x := rfl
  rcases x with ⟨r, b⟩
  have hu : unitResiduePairEquivUnits base hBase (r, b) = u := by
    simpa [hx] using (unitResiduePairEquivUnits base hBase).apply_symm_apply u
  cases b
  · have hneg : unitResiduePairEquivUnits base hBase (r, true) = -u := by
      calc
        unitResiduePairEquivUnits base hBase (r, true) =
            - unitResiduePairEquivUnits base hBase (r, false) :=
              unitResiduePairEquivUnits_neg_switch_false hBase r
        _ = -u := by simp [hu]
    have hsymm :
        (unitResiduePairEquivUnits base hBase).symm (-u) = (r, true) := by
      apply (unitResiduePairEquivUnits base hBase).injective
      simpa using hneg.symm
    simp [unitNegationOrbitRep, hx, hsymm]
  · have hneg : unitResiduePairEquivUnits base hBase (r, false) = -u := by
      calc
        unitResiduePairEquivUnits base hBase (r, false) =
            - unitResiduePairEquivUnits base hBase (r, true) :=
              unitResiduePairEquivUnits_neg_switch_true hBase r
        _ = -u := by simp [hu]
    have hsymm :
        (unitResiduePairEquivUnits base hBase).symm (-u) = (r, false) := by
      apply (unitResiduePairEquivUnits base hBase).injective
      simpa using hneg.symm
    simp [unitNegationOrbitRep, hx, hsymm]

theorem unitNegationOrbitRep_smul
    (base : ℕ) [NeZero base] (hBase : 2 < base)
    (g : negationSubgroup base) (u : (ZMod base)ˣ) :
    unitNegationOrbitRep base hBase (g • u) = unitNegationOrbitRep base hBase u := by
  rcases g.property with hg | hg
  · change unitNegationOrbitRep base hBase (((g : (ZMod base)ˣ)) * u) =
        unitNegationOrbitRep base hBase u
    rw [hg, one_mul]
  · change unitNegationOrbitRep base hBase (((g : (ZMod base)ˣ)) * u) =
        unitNegationOrbitRep base hBase u
    rw [hg, neg_one_mul]
    simpa using (unitNegationOrbitRep_neg base hBase u)

noncomputable def negationActionOrbitRepLift
    (base : ℕ) [NeZero base] (hBase : 2 < base) :
    negationActionOrbitQuotient base → { a : ℕ // a ∈ unitResiduePairReps base } :=
  Quotient.lift (unitNegationOrbitRep base hBase) (by
    intro u v h
    rcases h with ⟨g, hg⟩
    rw [← hg]
    exact unitNegationOrbitRep_smul base hBase g v)

noncomputable def negationActionQuotientEquivPairReps
    (base : ℕ) [NeZero base] (hBase : 2 < base) :
    negationActionOrbitQuotient base ≃ { a : ℕ // a ∈ unitResiduePairReps base } where
  toFun := negationActionOrbitRepLift base hBase
  invFun r :=
    Quotient.mk'' (unitResiduePairEquivUnits base hBase (r, false))
  left_inv := by
    intro q
    refine Quotient.inductionOn q ?_
    intro u
    let x := (unitResiduePairEquivUnits base hBase).symm u
    have hx : (unitResiduePairEquivUnits base hBase).symm u = x := rfl
    rcases x with ⟨r, b⟩
    have hu : unitResiduePairEquivUnits base hBase (r, b) = u := by
      simpa [hx] using (unitResiduePairEquivUnits base hBase).apply_symm_apply u
    have hRep : negationActionOrbitRepLift base hBase ⟦u⟧ = r := by
      simp [negationActionOrbitRepLift, unitNegationOrbitRep, hx]
    cases b
    · apply Quotient.sound
      refine ⟨1, ?_⟩
      rw [hRep]
      simp [hu]
    · apply Quotient.sound
      refine ⟨⟨-1, Or.inr rfl⟩, ?_⟩
      rw [hRep]
      calc
        ((⟨-1, Or.inr rfl⟩ : negationSubgroup base) • u) =
            (((⟨-1, Or.inr rfl⟩ : negationSubgroup base) : (ZMod base)ˣ) • u) := by
              rw [Subgroup.smul_def]
        _ = (((⟨-1, Or.inr rfl⟩ : negationSubgroup base) : (ZMod base)ˣ) * u) := by
              simp
        _ = -u := by
              simp
        _ = unitResiduePairEquivUnits base hBase (r, false) := by
          simpa [hu] using (unitResiduePairEquivUnits_neg_switch_true hBase r).symm
  right_inv r := by
    change unitNegationOrbitRep base hBase
        (unitResiduePairEquivUnits base hBase (r, false)) = r
    simp [unitNegationOrbitRep]

noncomputable def negationActionQuotientEquivUnitNegationOrbitQuotient
    (base : ℕ) [NeZero base] (hBase : 2 < base) :
    negationActionOrbitQuotient base ≃ Quotient (unitNegationOrbitSetoid base hBase) :=
  (negationActionQuotientEquivPairReps base hBase).trans
    (unitNegationOrbitQuotientEquivPairReps base hBase).symm

theorem card_negationActionOrbitQuotient_eq_pairReps
    (base : ℕ) [NeZero base] (hBase : 2 < base) :
    Fintype.card (negationActionOrbitQuotient base) = (unitResiduePairReps base).card := by
  rw [Fintype.card_congr (negationActionQuotientEquivPairReps base hBase), Fintype.card_coe]

theorem card_negationActionOrbitQuotient_eq_unitNegationOrbitQuotient
    (base : ℕ) [NeZero base] (hBase : 2 < base) :
    Fintype.card (negationActionOrbitQuotient base) =
      Fintype.card (Quotient (unitNegationOrbitSetoid base hBase)) := by
  rw [Fintype.card_congr (negationActionQuotientEquivUnitNegationOrbitQuotient base hBase)]

theorem card_negationActionOrbitQuotient_eq_totient_div_two
    (base : ℕ) [NeZero base] (hBase : 2 < base) :
    Fintype.card (negationActionOrbitQuotient base) = Nat.totient base / 2 := by
  rw [card_negationActionOrbitQuotient_eq_pairReps (base := base) (hBase := hBase),
    card_unitResiduePairReps_eq_totient_div_two hBase]

theorem card_negationActionOrbitQuotient_twoHundredTen :
    Fintype.card (negationActionOrbitQuotient 210) = 24 := by
  haveI : NeZero 210 := ⟨by decide⟩
  rw [card_negationActionOrbitQuotient_eq_pairReps (base := 210) (hBase := by decide)]
  exact card_unitResiduePairReps_twoHundredTen

theorem card_negationActionOrbitQuotient_twoThousandThreeHundredTen :
    Fintype.card (negationActionOrbitQuotient 2310) = 240 := by
  haveI : NeZero 2310 := ⟨by decide⟩
  rw [card_negationActionOrbitQuotient_eq_pairReps (base := 2310) (hBase := by decide)]
  exact card_unitResiduePairReps_twoThousandThreeHundredTen

end PrimeArithmetic.Density
