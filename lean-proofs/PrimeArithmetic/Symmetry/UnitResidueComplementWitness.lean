import PrimeArithmetic.Symmetry.MidpointObstruction
import PrimeArithmetic.Density.UnitResidueSymmetry

namespace PrimeArithmetic.Symmetry.UnitResidueComplementWitness

open PrimeArithmetic.Foundation

/-!
Generic symmetry witness from complement-paired unit residues.

For an even base `> 2`, the admissible residues below the base are closed under
the complement map `a ↦ base - a`. This produces a concrete `PerfectPairing`
that plugs directly into the abstract midpoint-obstruction theorem.
-/

def reflectFin (base : ℕ) : Fin (base + 1) → Fin (base + 1)
  | x => ⟨base - x.1, Nat.lt_succ_of_le (Nat.sub_le _ _)⟩

theorem reflectFin_involutive (base : ℕ) : Function.Involutive (reflectFin base) := by
  intro x
  apply Fin.ext
  simp [reflectFin, Nat.sub_sub_self (Nat.le_of_lt_succ x.is_lt)]

def midpointFin (base : ℕ) : Fin (base + 1) :=
  ⟨base / 2, Nat.lt_succ_of_le (Nat.div_le_self _ _)⟩

theorem reflectFin_midpoint {base : ℕ} (hEven : Even base) :
    reflectFin base (midpointFin base) = midpointFin base := by
  apply Fin.ext
  rcases hEven with ⟨k, rfl⟩
  have hk : (k + k) / 2 = k := by
    rw [← Nat.two_mul, Nat.mul_div_right _ (by decide : 0 < 2)]
  simp [reflectFin, midpointFin, hk]

def symmetryData (base : ℕ) (hEven : Even base) : SymmetryData (Fin (base + 1)) where
  mid := midpointFin base
  inv := reflectFin base
  invInvolutive := reflectFin_involutive base
  invMid := reflectFin_midpoint hEven

def asFin {base : ℕ} :
    { a : ℕ // a ∈ PrimeArithmetic.Density.unitResidues base } → Fin (base + 1)
  | x => ⟨x.1, Nat.lt_succ_of_lt (PrimeArithmetic.Density.mem_unitResidues.1 x.2).1⟩

def mate {base : ℕ} (hBase : 2 < base) :
    { a : ℕ // a ∈ PrimeArithmetic.Density.unitResidues base } →
      { a : ℕ // a ∈ PrimeArithmetic.Density.unitResidues base }
  | x => ⟨base - x.1,
      PrimeArithmetic.Density.complement_mem_unitResidues (lt_trans one_lt_two hBase) x.2⟩

theorem mate_involutive {base : ℕ} (hBase : 2 < base) :
    Function.Involutive (mate hBase) := by
  intro x
  apply Subtype.ext
  simp [mate, Nat.sub_sub_self, le_of_lt, PrimeArithmetic.Density.mem_unitResidues.1 x.2 |>.1]

theorem mate_noFixed {base : ℕ} (hBase : 2 < base) :
    ∀ x : { a : ℕ // a ∈ PrimeArithmetic.Density.unitResidues base }, mate hBase x ≠ x := by
  intro x hx
  exact PrimeArithmetic.Density.complement_ne_self hBase x.2 (congrArg Subtype.val hx)

theorem residue_equivariant {base : ℕ} (hBase : 2 < base) (hEven : Even base) :
    ∀ x : { a : ℕ // a ∈ PrimeArithmetic.Density.unitResidues base },
      (symmetryData base hEven).inv (asFin x) = asFin (mate hBase x) := by
  intro x
  apply Fin.ext
  rfl

theorem residue_distinct {base : ℕ} (hBase : 2 < base) :
    ∀ x : { a : ℕ // a ∈ PrimeArithmetic.Density.unitResidues base },
      asFin (mate hBase x) ≠ asFin x := by
  intro x hx
  exact PrimeArithmetic.Density.complement_ne_self hBase x.2 (congrArg Fin.val hx)

def pairing {base : ℕ} (hBase : 2 < base) (hEven : Even base) :
    PerfectPairing (symmetryData base hEven) (asFin (base := base)) where
  mate := mate hBase
  mateInvolutive := mate_involutive hBase
  noFixed := mate_noFixed hBase
  equivariant := residue_equivariant hBase hEven
  residueDistinct := residue_distinct hBase

theorem midpoint_not_in_unitResidueRange {base : ℕ} (hBase : 2 < base) (hEven : Even base) :
    midpointFin base ∉ Set.range (asFin (base := base)) :=
  PrimeArithmetic.Symmetry.midpointNotInRange (pairing hBase hEven)

theorem midpoint_not_mem_unitResidues {base : ℕ} (hBase : 2 < base) (hEven : Even base) :
    base / 2 ∉ PrimeArithmetic.Density.unitResidues base := by
  intro hMid
  have hRange : midpointFin base ∈ Set.range (asFin (base := base)) := by
    refine ⟨⟨base / 2, hMid⟩, ?_⟩
    apply Fin.ext
    rfl
  exact midpoint_not_in_unitResidueRange hBase hEven hRange

theorem midpoint_not_mem_unitResidues_210 : 105 ∉ PrimeArithmetic.Density.unitResidues 210 := by
  simpa using midpoint_not_mem_unitResidues (base := 210) (by decide) (show Even 210 by native_decide)

theorem midpoint_not_mem_unitResidues_2310 : 1155 ∉ PrimeArithmetic.Density.unitResidues 2310 := by
  simpa using midpoint_not_mem_unitResidues (base := 2310) (by decide) (show Even 2310 by native_decide)

end PrimeArithmetic.Symmetry.UnitResidueComplementWitness
