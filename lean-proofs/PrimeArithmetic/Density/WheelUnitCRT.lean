import Mathlib
import PrimeArithmetic.Density.ZModUnitCRT
import PrimeArithmetic.Density.UnitResiduePairs
import PrimeArithmetic.Density.WheelResidueClassifier

namespace PrimeArithmetic.Density

/-!
Finite-product CRT on unit groups for wheel bases.

This module upgrades the binary coprime-base CRT theorem to an explicit
finite-family statement, presented as an iterated product over `primes.toList`.
That keeps the theorem close to the standard Chinese remainder decomposition
while staying lightweight in Lean.
-/

def wheelUnitTuple : List ℕ → Type
  | [] => PUnit
  | p :: primes => (ZMod p)ˣ × wheelUnitTuple primes

def zmodUnitsListEquiv
    {primes : List ℕ} (hPairwise : primes.Pairwise Nat.Coprime)
    (hNonzero : ∀ p ∈ primes, p ≠ 0) :
    (ZMod primes.prod)ˣ ≃ wheelUnitTuple primes := by
  revert hPairwise hNonzero
  induction primes with
  | nil =>
      intro hPairwise hNonzero
      simpa using (Equiv.ofUnique (ZMod 1)ˣ PUnit)
  | cons p primes ih =>
      intro hPairwise hNonzero
      have hp0 : p ≠ 0 := hNonzero p (by simp)
      have hNonzero' : ∀ q ∈ primes, q ≠ 0 := by
        intro q hq
        exact hNonzero q (by simp [hq])
      have hcop : p.Coprime primes.prod := by
        exact Nat.coprime_list_prod_right_iff.mpr (List.pairwise_cons.mp hPairwise).1
      haveI : NeZero p := ⟨hp0⟩
      have hprod0 : primes.prod ≠ 0 := by
        intro hprod
        have hmem0 : 0 ∈ primes := List.prod_eq_zero_iff.mp hprod
        exact (hNonzero' 0 hmem0) rfl
      haveI : NeZero primes.prod := ⟨hprod0⟩
      calc
        (ZMod ((p :: primes).prod))ˣ ≃ (ZMod (p * primes.prod))ˣ := by
          simpa using (Equiv.refl (ZMod (p * primes.prod))ˣ)
        _ ≃ (ZMod p)ˣ × (ZMod primes.prod)ˣ :=
          zmodUnitsMulEquiv (m := p) (n := primes.prod) hcop
        _ ≃ (ZMod p)ˣ × wheelUnitTuple primes :=
          Equiv.prodCongr (Equiv.refl _) (ih (List.pairwise_cons.mp hPairwise).2 hNonzero')

noncomputable def zmodUnitsWheelBaseEquiv
    {primes : Finset ℕ} (hPrimes : ∀ p ∈ primes, p.Prime) :
    (ZMod (wheelBase primes))ˣ ≃ wheelUnitTuple primes.toList := by
  have hPairwiseSet : (↑(primes.toList.toFinset) : Set ℕ).Pairwise Nat.Coprime := by
    simpa [Finset.toList_toFinset] using
      (pairwiseCoprime_primes (primes := primes) hPrimes)
  have hPairwiseList : primes.toList.Pairwise Nat.Coprime := by
    simpa using
      List.pairwise_of_coe_toFinset_pairwise
        (l := primes.toList)
        (r := Nat.Coprime)
        hPairwiseSet
        (Finset.nodup_toList primes)
  have hNonzeroList : ∀ p ∈ primes.toList, p ≠ 0 := by
    intro p hp
    exact (hPrimes p (by simpa using hp)).ne_zero
  rw [wheelBase, ← Finset.prod_toList]
  exact zmodUnitsListEquiv hPairwiseList hNonzeroList

theorem card_unitResiduePairReps_wheelBase
    {primes : Finset ℕ} (hPrimes : ∀ p ∈ primes, p.Prime)
    (hBase : 2 < wheelBase primes) :
    (unitResiduePairReps (wheelBase primes)).card = (∏ p ∈ primes, (p - 1)) / 2 := by
  rw [card_unitResiduePairReps_eq_totient_div_two hBase, totient_wheelBase hPrimes]

theorem card_unitResiduePairReps_twoHundredTen : (unitResiduePairReps 210).card = 24 := by
  calc
    (unitResiduePairReps 210).card =
        (unitResiduePairReps (wheelBase ({2, 3, 5, 7} : Finset ℕ))).card := by
          rw [wheelBase_two_three_five_seven]
    _ = (∏ p ∈ ({2, 3, 5, 7} : Finset ℕ), (p - 1)) / 2 := by
          simpa using
            card_unitResiduePairReps_wheelBase
              (primes := ({2, 3, 5, 7} : Finset ℕ))
              (by decide)
              (by decide)
    _ = 24 := by native_decide

theorem card_unitResiduePairReps_twoThousandThreeHundredTen :
    (unitResiduePairReps 2310).card = 240 := by
  calc
    (unitResiduePairReps 2310).card =
        (unitResiduePairReps (wheelBase ({2, 3, 5, 7, 11} : Finset ℕ))).card := by
          rw [wheelBase_two_three_five_seven_eleven]
    _ = (∏ p ∈ ({2, 3, 5, 7, 11} : Finset ℕ), (p - 1)) / 2 := by
          simpa using
            card_unitResiduePairReps_wheelBase
              (primes := ({2, 3, 5, 7, 11} : Finset ℕ))
              (by decide)
              (by decide)
    _ = 240 := by native_decide

end PrimeArithmetic.Density
