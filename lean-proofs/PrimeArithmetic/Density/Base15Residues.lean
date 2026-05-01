import Mathlib
import PrimeArithmetic.Density.RadicalFilter
import PrimeArithmetic.Density.UnitResiduePairs
import PrimeArithmetic.Density.UnitResidues
import PrimeArithmetic.Density.WheelBases
import PrimeArithmetic.Density.WheelResidueClassifier

namespace PrimeArithmetic.Density.Base15Residues

def expected : Finset ℕ := ({1, 2, 4, 7, 8, 11, 13, 14} : Finset ℕ)
def expectedPairReps : Finset ℕ := ({1, 2, 4, 7} : Finset ℕ)

theorem radical_base : PrimeArithmetic.Density.radical 15 = 15 := by
  native_decide

theorem unitResidues_eq_expected :
    PrimeArithmetic.Density.unitResidues 15 = expected := by
  native_decide

theorem unitResidues_card : (PrimeArithmetic.Density.unitResidues 15).card = 8 := by
  native_decide

theorem totient_base : Nat.totient 15 = 8 := by
  native_decide

theorem unitResiduePairReps_eq_expectedPairReps :
    PrimeArithmetic.Density.unitResiduePairReps 15 = expectedPairReps := by
  native_decide

theorem unitResiduePairReps_card :
    (PrimeArithmetic.Density.unitResiduePairReps 15).card = 4 := by
  native_decide

theorem wheelBase_three_five :
    PrimeArithmetic.Density.wheelBase ({3, 5} : Finset ℕ) = 15 := by
  native_decide

theorem memExpected_iff_localUnitResidues {a : ℕ} :
    a ∈ expected ↔
      a < 15 ∧
        a % 3 ∈ PrimeArithmetic.Density.unitResidues 3 ∧
        a % 5 ∈ PrimeArithmetic.Density.unitResidues 5 := by
  simpa [expected, unitResidues_eq_expected, wheelBase_three_five] using
    (PrimeArithmetic.Density.mem_unitResidues_wheelBase_iff_primeUnitResidues
      (primes := ({3, 5} : Finset ℕ))
      (by decide)
      (a := a))

theorem memExpected_iff_mod_ne_zero {a : ℕ} :
    a ∈ expected ↔
      a < 15 ∧ a % 3 ≠ 0 ∧ a % 5 ≠ 0 := by
  simpa [expected, unitResidues_eq_expected, wheelBase_three_five] using
    (PrimeArithmetic.Density.mem_unitResidues_wheelBase_iff_mod_ne_zero
      (primes := ({3, 5} : Finset ℕ))
      (by decide)
      (a := a))

theorem wheelCRTRepresentative_memExpected_of_localNonzeroResidues
    {residue : ℕ → ℕ}
    (h3 : residue 3 % 3 ≠ 0)
    (h5 : residue 5 % 5 ≠ 0) :
    (PrimeArithmetic.Density.wheelCRTRepresentative
      (primes := ({3, 5} : Finset ℕ))
      residue
      (by decide) : ℕ) ∈ expected := by
  have hMem :
      (PrimeArithmetic.Density.wheelCRTRepresentative
        (primes := ({3, 5} : Finset ℕ))
        residue
        (by decide) : ℕ) ∈
        PrimeArithmetic.Density.unitResidues
          (PrimeArithmetic.Density.wheelBase ({3, 5} : Finset ℕ)) := by
    apply PrimeArithmetic.Density.wheelCRTRepresentative_mem_unitResidues_of_mod_ne_zero
      (primes := ({3, 5} : Finset ℕ))
      (residue := residue)
      (by decide)
    intro p hp
    rcases (by simpa using hp : p = 3 ∨ p = 5) with rfl | rfl
    · exact h3
    · exact h5
  simpa [expected, unitResidues_eq_expected, wheelBase_three_five] using hMem

theorem wheelCRTRepresentative_memExpected_of_localUnitResidues
    {residue : ℕ → ℕ}
    (h3 : residue 3 ∈ PrimeArithmetic.Density.unitResidues 3)
    (h5 : residue 5 ∈ PrimeArithmetic.Density.unitResidues 5) :
    (PrimeArithmetic.Density.wheelCRTRepresentative
      (primes := ({3, 5} : Finset ℕ))
      residue
      (by decide) : ℕ) ∈ expected := by
  have hMem :
      (PrimeArithmetic.Density.wheelCRTRepresentative
        (primes := ({3, 5} : Finset ℕ))
        residue
        (by decide) : ℕ) ∈
        PrimeArithmetic.Density.unitResidues
          (PrimeArithmetic.Density.wheelBase ({3, 5} : Finset ℕ)) := by
    apply PrimeArithmetic.Density.wheelCRTRepresentative_mem_unitResidues
      (primes := ({3, 5} : Finset ℕ))
      (residue := residue)
      (by decide)
    intro p hp
    rcases (by simpa using hp : p = 3 ∨ p = 5) with rfl | rfl
    · simpa using h3
    · simpa using h5
  simpa [expected, unitResidues_eq_expected, wheelBase_three_five] using hMem

theorem wheelCRTRepresentative_residueMap_eq_of_memExpected
    {a : ℕ} (ha : a ∈ expected) :
    PrimeArithmetic.Density.wheelCRTRepresentative
      (primes := ({3, 5} : Finset ℕ))
      (fun p => a % p)
      (by decide) = a := by
  have hMem : a ∈ PrimeArithmetic.Density.unitResidues 15 := by
    simpa [expected, unitResidues_eq_expected] using ha
  simpa [wheelBase_three_five] using
    (PrimeArithmetic.Density.wheelCRTRepresentative_residueMap_eq_of_mem_unitResidues
      (primes := ({3, 5} : Finset ℕ))
      (hPrimes := by decide)
      (a := a)
      hMem)

theorem primeGtFifteenMod_memExpected
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 15 < p) :
    p % 15 ∈ expected := by
  simpa [expected, unitResidues_eq_expected] using
    (PrimeArithmetic.Density.primeGtBaseMod_memUnitResidues
      (base := 15) (hBase := by decide) hPrime hGt)

theorem primeGtFifteenMod_memExpectedPairReps_or_complement
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 15 < p) :
    p % 15 ∈ expectedPairReps ∨ 15 - (p % 15) ∈ expectedPairReps := by
  simpa [expectedPairReps, unitResiduePairReps_eq_expectedPairReps] using
    (PrimeArithmetic.Density.primeGtBaseMod_mem_unitResiduePairReps_or_complement
      (base := 15) (hBase := by decide) hPrime hGt)

theorem primeGtFifteenMod_xor_memExpectedPairReps_complement
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 15 < p) :
    Xor' (p % 15 ∈ expectedPairReps) (15 - (p % 15) ∈ expectedPairReps) := by
  simpa [expectedPairReps, unitResiduePairReps_eq_expectedPairReps] using
    (PrimeArithmetic.Density.primeGtBaseMod_xor_mem_unitResiduePairReps_complement
      (base := 15) (hBase := by decide) hPrime hGt)

end PrimeArithmetic.Density.Base15Residues
