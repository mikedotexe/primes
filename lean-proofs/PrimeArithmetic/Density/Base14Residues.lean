import Mathlib
import PrimeArithmetic.Density.RadicalFilter
import PrimeArithmetic.Density.UnitResiduePairs
import PrimeArithmetic.Density.UnitResidues
import PrimeArithmetic.Density.WheelBases
import PrimeArithmetic.Density.WheelResidueClassifier

namespace PrimeArithmetic.Density.Base14Residues

def expected : Finset ℕ := ({1, 3, 5, 9, 11, 13} : Finset ℕ)
def expectedPairReps : Finset ℕ := ({1, 3, 5} : Finset ℕ)

theorem radical_base : PrimeArithmetic.Density.radical 14 = 14 := by
  native_decide

theorem unitResidues_eq_expected :
    PrimeArithmetic.Density.unitResidues 14 = expected := by
  native_decide

theorem unitResidues_card : (PrimeArithmetic.Density.unitResidues 14).card = 6 := by
  native_decide

theorem totient_base : Nat.totient 14 = 6 := by
  native_decide

theorem unitResiduePairReps_eq_expectedPairReps :
    PrimeArithmetic.Density.unitResiduePairReps 14 = expectedPairReps := by
  native_decide

theorem unitResiduePairReps_card :
    (PrimeArithmetic.Density.unitResiduePairReps 14).card = 3 := by
  native_decide

theorem wheelBase_two_seven :
    PrimeArithmetic.Density.wheelBase ({2, 7} : Finset ℕ) = 14 := by
  native_decide

theorem memExpected_iff_localUnitResidues {a : ℕ} :
    a ∈ expected ↔
      a < 14 ∧
        a % 2 ∈ PrimeArithmetic.Density.unitResidues 2 ∧
        a % 7 ∈ PrimeArithmetic.Density.unitResidues 7 := by
  simpa [expected, unitResidues_eq_expected, wheelBase_two_seven] using
    (PrimeArithmetic.Density.mem_unitResidues_wheelBase_iff_primeUnitResidues
      (primes := ({2, 7} : Finset ℕ))
      (by decide)
      (a := a))

theorem memExpected_iff_mod_ne_zero {a : ℕ} :
    a ∈ expected ↔
      a < 14 ∧ a % 2 ≠ 0 ∧ a % 7 ≠ 0 := by
  simpa [expected, unitResidues_eq_expected, wheelBase_two_seven] using
    (PrimeArithmetic.Density.mem_unitResidues_wheelBase_iff_mod_ne_zero
      (primes := ({2, 7} : Finset ℕ))
      (by decide)
      (a := a))

theorem wheelCRTRepresentative_memExpected_of_localNonzeroResidues
    {residue : ℕ → ℕ}
    (h2 : residue 2 % 2 ≠ 0)
    (h7 : residue 7 % 7 ≠ 0) :
    (PrimeArithmetic.Density.wheelCRTRepresentative
      (primes := ({2, 7} : Finset ℕ))
      residue
      (by decide) : ℕ) ∈ expected := by
  have hMem :
      (PrimeArithmetic.Density.wheelCRTRepresentative
        (primes := ({2, 7} : Finset ℕ))
        residue
        (by decide) : ℕ) ∈
        PrimeArithmetic.Density.unitResidues
          (PrimeArithmetic.Density.wheelBase ({2, 7} : Finset ℕ)) := by
    apply PrimeArithmetic.Density.wheelCRTRepresentative_mem_unitResidues_of_mod_ne_zero
      (primes := ({2, 7} : Finset ℕ))
      (residue := residue)
      (by decide)
    intro p hp
    rcases (by simpa using hp : p = 2 ∨ p = 7) with rfl | rfl
    · exact h2
    · exact h7
  simpa [expected, unitResidues_eq_expected, wheelBase_two_seven] using hMem

theorem wheelCRTRepresentative_memExpected_of_localUnitResidues
    {residue : ℕ → ℕ}
    (h2 : residue 2 ∈ PrimeArithmetic.Density.unitResidues 2)
    (h7 : residue 7 ∈ PrimeArithmetic.Density.unitResidues 7) :
    (PrimeArithmetic.Density.wheelCRTRepresentative
      (primes := ({2, 7} : Finset ℕ))
      residue
      (by decide) : ℕ) ∈ expected := by
  have hMem :
      (PrimeArithmetic.Density.wheelCRTRepresentative
        (primes := ({2, 7} : Finset ℕ))
        residue
        (by decide) : ℕ) ∈
        PrimeArithmetic.Density.unitResidues
          (PrimeArithmetic.Density.wheelBase ({2, 7} : Finset ℕ)) := by
    apply PrimeArithmetic.Density.wheelCRTRepresentative_mem_unitResidues
      (primes := ({2, 7} : Finset ℕ))
      (residue := residue)
      (by decide)
    intro p hp
    rcases (by simpa using hp : p = 2 ∨ p = 7) with rfl | rfl
    · simpa using h2
    · simpa using h7
  simpa [expected, unitResidues_eq_expected, wheelBase_two_seven] using hMem

theorem wheelCRTRepresentative_residueMap_eq_of_memExpected
    {a : ℕ} (ha : a ∈ expected) :
    PrimeArithmetic.Density.wheelCRTRepresentative
      (primes := ({2, 7} : Finset ℕ))
      (fun p => a % p)
      (by decide) = a := by
  have hMem : a ∈ PrimeArithmetic.Density.unitResidues 14 := by
    simpa [expected, unitResidues_eq_expected] using ha
  simpa [wheelBase_two_seven] using
    (PrimeArithmetic.Density.wheelCRTRepresentative_residueMap_eq_of_mem_unitResidues
      (primes := ({2, 7} : Finset ℕ))
      (hPrimes := by decide)
      (a := a)
      hMem)

theorem primeGtFourteenMod_memExpected
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 14 < p) :
    p % 14 ∈ expected := by
  simpa [expected, unitResidues_eq_expected] using
    (PrimeArithmetic.Density.primeGtBaseMod_memUnitResidues
      (base := 14) (hBase := by decide) hPrime hGt)

theorem primeGtFourteenMod_memExpectedPairReps_or_complement
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 14 < p) :
    p % 14 ∈ expectedPairReps ∨ 14 - (p % 14) ∈ expectedPairReps := by
  simpa [expectedPairReps, unitResiduePairReps_eq_expectedPairReps] using
    (PrimeArithmetic.Density.primeGtBaseMod_mem_unitResiduePairReps_or_complement
      (base := 14) (hBase := by decide) hPrime hGt)

theorem primeGtFourteenMod_xor_memExpectedPairReps_complement
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 14 < p) :
    Xor' (p % 14 ∈ expectedPairReps) (14 - (p % 14) ∈ expectedPairReps) := by
  simpa [expectedPairReps, unitResiduePairReps_eq_expectedPairReps] using
    (PrimeArithmetic.Density.primeGtBaseMod_xor_mem_unitResiduePairReps_complement
      (base := 14) (hBase := by decide) hPrime hGt)

end PrimeArithmetic.Density.Base14Residues
