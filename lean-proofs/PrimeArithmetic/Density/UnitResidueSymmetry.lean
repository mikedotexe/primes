import Mathlib
import PrimeArithmetic.Density.UnitResidues

namespace PrimeArithmetic.Density

/-!
Complement symmetry on unit residues.

On natural representatives below `base`, the complement map `a ↦ base - a`
models negation in `ZMod base`. For `base > 2`, unit residues are stable under
this map and never sit at its fixed midpoint, so admissible residues come in
complement pairs.
-/

theorem unitResidue_pos {base a : ℕ} (hBase : 1 < base) (ha : a ∈ unitResidues base) :
    0 < a := by
  rcases (mem_unitResidues.1 ha) with ⟨_, hcop⟩
  by_contra hzero
  have ha0 : a = 0 := Nat.eq_zero_of_le_zero (not_lt.1 hzero)
  subst ha0
  have hbase1 : base = 1 := by
    simpa using hcop
  exact hBase.ne' hbase1

theorem complement_mem_unitResidues {base a : ℕ} (hBase : 1 < base)
    (ha : a ∈ unitResidues base) :
    base - a ∈ unitResidues base := by
  rcases (mem_unitResidues.1 ha) with ⟨haLt, hcop⟩
  refine mem_unitResidues.2 ?_
  constructor
  · exact Nat.sub_lt (Nat.zero_lt_one.trans hBase) (unitResidue_pos hBase ha)
  · have hcop₁ : a.Coprime (base - a) := by
      exact (Nat.coprime_sub_self_right (le_of_lt haLt)).2 hcop.symm
    have hcop₂ : (base - a).Coprime base := by
      refine (Nat.coprime_sub_self_right (Nat.sub_le _ _)).1 ?_
      simpa [Nat.sub_sub_self (le_of_lt haLt)] using hcop₁.symm
    exact hcop₂.symm

theorem complement_ne_self {base a : ℕ} (hBase : 2 < base) (ha : a ∈ unitResidues base) :
    base - a ≠ a := by
  rcases (mem_unitResidues.1 ha) with ⟨haLt, hcop⟩
  intro hfix
  have hbaseEq : base = a + a := by
    exact (Nat.sub_eq_iff_eq_add (le_of_lt haLt)).1 hfix
  have haGtOne : 1 < a := by
    omega
  have hadivBase : a ∣ base := by
    use 2
    omega
  exact (Nat.not_coprime_of_dvd_of_dvd haGtOne hadivBase dvd_rfl) hcop

def unitResidueComplementEquiv (base : ℕ) (hBase : 2 < base) :
    { a : ℕ // a ∈ unitResidues base } ≃ { a : ℕ // a ∈ unitResidues base } where
  toFun x := ⟨base - x.1, complement_mem_unitResidues (lt_trans one_lt_two hBase) x.2⟩
  invFun := fun x => ⟨base - x.1, complement_mem_unitResidues (lt_trans one_lt_two hBase) x.2⟩
  left_inv x := by
    apply Subtype.ext
    simp [Nat.sub_sub_self (le_of_lt (mem_unitResidues.1 x.2).1)]
  right_inv x := by
    apply Subtype.ext
    simp [Nat.sub_sub_self (le_of_lt (mem_unitResidues.1 x.2).1)]

theorem unitResidueComplementEquiv_ne_self
    {base : ℕ} (hBase : 2 < base) (x : { a : ℕ // a ∈ unitResidues base }) :
    unitResidueComplementEquiv base hBase x ≠ x := by
  intro hx
  exact complement_ne_self hBase x.2 (congrArg Subtype.val hx)

theorem card_unitResidues_even_of_two_lt {base : ℕ} (hBase : 2 < base) :
    Even (unitResidues base).card := by
  rw [card_unitResidues]
  exact Nat.totient_even hBase

theorem primeGtBaseMod_complement_memUnitResidues
    {p base : ℕ} (hBase : 2 < base) (hPrime : Nat.Prime p) (hGt : base < p) :
    base - (p % base) ∈ unitResidues base := by
  exact complement_mem_unitResidues (lt_trans one_lt_two hBase) <|
    primeGtBaseMod_memUnitResidues (le_of_lt hBase) hPrime hGt

theorem primeGtBaseMod_complement_ne_self
    {p base : ℕ} (hBase : 2 < base) (hPrime : Nat.Prime p) (hGt : base < p) :
    base - (p % base) ≠ p % base := by
  exact complement_ne_self hBase <|
    primeGtBaseMod_memUnitResidues (le_of_lt hBase) hPrime hGt

end PrimeArithmetic.Density
