import Mathlib
import PrimeArithmetic.Density.UnitResiduePairs
import PrimeArithmetic.Density.WheelUnitCRT

namespace PrimeArithmetic.Density

/-!
Explicit negation-orbit quotient on `ZMod` units.

The pair-representative equivalence from `UnitResiduePairs` gives a canonical
quotient of `(ZMod n)ˣ` into two-element complement/negation classes. This
module packages that quotient explicitly and records its cardinality.
-/

def unitNegationOrbitRep
    (base : ℕ) [NeZero base] (hBase : 2 < base) (u : (ZMod base)ˣ) :
    { a : ℕ // a ∈ unitResiduePairReps base } :=
  (unitResiduePairEquivUnits base hBase).symm u |>.1

def unitNegationOrbitSetoid
    (base : ℕ) [NeZero base] (hBase : 2 < base) : Setoid (ZMod base)ˣ where
  r u v := unitNegationOrbitRep base hBase u = unitNegationOrbitRep base hBase v
  iseqv := by
    refine ⟨?_, ?_, ?_⟩
    · intro u
      rfl
    · intro u v h
      simpa using h.symm
    · intro u v w huv hvw
      exact huv.trans hvw

noncomputable instance instFintypeUnitNegationOrbitQuotient
    (base : ℕ) [NeZero base] (hBase : 2 < base) :
    Fintype (Quotient (unitNegationOrbitSetoid base hBase)) := by
  classical
  infer_instance

noncomputable def unitNegationOrbitQuotientEquivPairReps
    (base : ℕ) [NeZero base] (hBase : 2 < base) :
    Quotient (unitNegationOrbitSetoid base hBase) ≃
      { a : ℕ // a ∈ unitResiduePairReps base } where
  toFun :=
    Quotient.lift (unitNegationOrbitRep base hBase) (by
      intro u v h
      exact h)
  invFun r :=
    Quotient.mk'' (unitResiduePairEquivUnits base hBase (r, false))
  left_inv := by
    intro q
    refine Quotient.inductionOn q ?_
    intro u
    apply Quotient.sound
    change
      unitNegationOrbitRep base hBase
          ((unitResiduePairEquivUnits base hBase) (unitNegationOrbitRep base hBase u, false)) =
        unitNegationOrbitRep base hBase u
    simp [unitNegationOrbitRep]
  right_inv r := by
    simp [unitNegationOrbitRep]

theorem card_unitNegationOrbitQuotient_eq_pairReps
    (base : ℕ) [NeZero base] (hBase : 2 < base) :
    Fintype.card (Quotient (unitNegationOrbitSetoid base hBase)) =
      (unitResiduePairReps base).card := by
  rw [Fintype.card_congr (unitNegationOrbitQuotientEquivPairReps base hBase), Fintype.card_coe]

theorem card_unitNegationOrbitQuotient_eq_totient_div_two
    (base : ℕ) [NeZero base] (hBase : 2 < base) :
    Fintype.card (Quotient (unitNegationOrbitSetoid base hBase)) = Nat.totient base / 2 := by
  rw [card_unitNegationOrbitQuotient_eq_pairReps (base := base) (hBase := hBase),
    card_unitResiduePairReps_eq_totient_div_two hBase]

theorem card_unitNegationOrbitQuotient_twoHundredTen :
    Fintype.card (Quotient (unitNegationOrbitSetoid 210 (by decide))) = 24 := by
  haveI : NeZero 210 := ⟨by decide⟩
  rw [card_unitNegationOrbitQuotient_eq_pairReps (base := 210) (hBase := by decide)]
  exact card_unitResiduePairReps_twoHundredTen

theorem card_unitNegationOrbitQuotient_twoThousandThreeHundredTen :
    Fintype.card (Quotient (unitNegationOrbitSetoid 2310 (by decide))) = 240 := by
  haveI : NeZero 2310 := ⟨by decide⟩
  rw [card_unitNegationOrbitQuotient_eq_pairReps (base := 2310) (hBase := by decide)]
  exact card_unitResiduePairReps_twoThousandThreeHundredTen

end PrimeArithmetic.Density
