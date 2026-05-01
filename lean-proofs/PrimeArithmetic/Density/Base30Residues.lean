import Mathlib
import PrimeArithmetic.Density.RadicalFilter
import PrimeArithmetic.Density.UnitResiduePairs
import PrimeArithmetic.Density.UnitResidues
import PrimeArithmetic.Density.WheelBases
import PrimeArithmetic.Density.WheelResidueClassifier

namespace PrimeArithmetic.Density.Base30Residues

def expected : Finset ℕ := ({1, 7, 11, 13, 17, 19, 23, 29} : Finset ℕ)
def expectedPairReps : Finset ℕ := ({1, 7, 11, 13} : Finset ℕ)

theorem radical_base : PrimeArithmetic.Density.radical 30 = 30 := by
  native_decide

theorem unitResidues_eq_expected :
    PrimeArithmetic.Density.unitResidues 30 = expected := by
  native_decide

theorem unitResidues_card : (PrimeArithmetic.Density.unitResidues 30).card = 8 := by
  native_decide

theorem totient_base : Nat.totient 30 = 8 := by
  native_decide

theorem unitResiduePairReps_eq_expectedPairReps :
    PrimeArithmetic.Density.unitResiduePairReps 30 = expectedPairReps := by
  native_decide

theorem unitResiduePairReps_card :
    (PrimeArithmetic.Density.unitResiduePairReps 30).card = 4 := by
  native_decide

theorem memExpected_iff_localUnitResidues {a : ℕ} :
    a ∈ expected ↔
      a < 30 ∧
        a % 2 ∈ PrimeArithmetic.Density.unitResidues 2 ∧
        a % 3 ∈ PrimeArithmetic.Density.unitResidues 3 ∧
        a % 5 ∈ PrimeArithmetic.Density.unitResidues 5 := by
  simpa [expected, unitResidues_eq_expected, PrimeArithmetic.Density.wheelBase_two_three_five] using
    (PrimeArithmetic.Density.mem_unitResidues_wheelBase_iff_primeUnitResidues
      (primes := ({2, 3, 5} : Finset ℕ))
      (by decide)
      (a := a))

theorem memExpected_iff_mod_ne_zero {a : ℕ} :
    a ∈ expected ↔
      a < 30 ∧ a % 2 ≠ 0 ∧ a % 3 ≠ 0 ∧ a % 5 ≠ 0 := by
  simpa [expected, unitResidues_eq_expected, PrimeArithmetic.Density.wheelBase_two_three_five] using
    (PrimeArithmetic.Density.mem_unitResidues_wheelBase_iff_mod_ne_zero
      (primes := ({2, 3, 5} : Finset ℕ))
      (by decide)
      (a := a))

theorem wheelCRTRepresentative_memExpected_of_localNonzeroResidues
    {residue : ℕ → ℕ}
    (h2 : residue 2 % 2 ≠ 0)
    (h3 : residue 3 % 3 ≠ 0)
    (h5 : residue 5 % 5 ≠ 0) :
    (PrimeArithmetic.Density.wheelCRTRepresentative
      (primes := ({2, 3, 5} : Finset ℕ))
      residue
      (by decide) : ℕ) ∈ expected := by
  have hMem :
      (PrimeArithmetic.Density.wheelCRTRepresentative
        (primes := ({2, 3, 5} : Finset ℕ))
        residue
        (by decide) : ℕ) ∈
        PrimeArithmetic.Density.unitResidues
          (PrimeArithmetic.Density.wheelBase ({2, 3, 5} : Finset ℕ)) := by
    apply PrimeArithmetic.Density.wheelCRTRepresentative_mem_unitResidues_of_mod_ne_zero
      (primes := ({2, 3, 5} : Finset ℕ))
      (residue := residue)
      (by decide)
    intro p hp
    rcases (by simpa using hp : p = 2 ∨ p = 3 ∨ p = 5) with rfl | rfl | rfl
    · exact h2
    · exact h3
    · exact h5
  simpa [expected, unitResidues_eq_expected, PrimeArithmetic.Density.wheelBase_two_three_five] using hMem

theorem wheelCRTRepresentative_memExpected_of_localUnitResidues
    {residue : ℕ → ℕ}
    (h2 : residue 2 ∈ PrimeArithmetic.Density.unitResidues 2)
    (h3 : residue 3 ∈ PrimeArithmetic.Density.unitResidues 3)
    (h5 : residue 5 ∈ PrimeArithmetic.Density.unitResidues 5) :
    (PrimeArithmetic.Density.wheelCRTRepresentative
      (primes := ({2, 3, 5} : Finset ℕ))
      residue
      (by decide) : ℕ) ∈ expected := by
  have hMem :
      (PrimeArithmetic.Density.wheelCRTRepresentative
        (primes := ({2, 3, 5} : Finset ℕ))
        residue
        (by decide) : ℕ) ∈
        PrimeArithmetic.Density.unitResidues
          (PrimeArithmetic.Density.wheelBase ({2, 3, 5} : Finset ℕ)) := by
    apply PrimeArithmetic.Density.wheelCRTRepresentative_mem_unitResidues
      (primes := ({2, 3, 5} : Finset ℕ))
      (residue := residue)
      (by decide)
    intro p hp
    rcases (by simpa using hp : p = 2 ∨ p = 3 ∨ p = 5) with rfl | rfl | rfl
    · simpa using h2
    · simpa using h3
    · simpa using h5
  simpa [expected, unitResidues_eq_expected, PrimeArithmetic.Density.wheelBase_two_three_five] using hMem

theorem wheelCRTRepresentative_residueMap_eq_of_memExpected
    {a : ℕ} (ha : a ∈ expected) :
    PrimeArithmetic.Density.wheelCRTRepresentative
      (primes := ({2, 3, 5} : Finset ℕ))
      (fun p => a % p)
      (by decide) = a := by
  have hMem : a ∈ PrimeArithmetic.Density.unitResidues 30 := by
    simpa [expected, unitResidues_eq_expected] using ha
  simpa [PrimeArithmetic.Density.wheelBase_two_three_five] using
    (PrimeArithmetic.Density.wheelCRTRepresentative_residueMap_eq_of_mem_unitResidues
      (primes := ({2, 3, 5} : Finset ℕ))
      (hPrimes := by decide)
      (a := a)
      hMem)

theorem primeGtThirtyMod_memExpected
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 30 < p) :
    p % 30 ∈ expected := by
  simpa [expected, unitResidues_eq_expected] using
    (PrimeArithmetic.Density.primeGtBaseMod_memUnitResidues
      (base := 30) (hBase := by decide) hPrime hGt)

theorem primeGtThirtyMod_memExpectedPairReps_or_complement
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 30 < p) :
    p % 30 ∈ expectedPairReps ∨ 30 - (p % 30) ∈ expectedPairReps := by
  simpa [expectedPairReps, unitResiduePairReps_eq_expectedPairReps] using
    (PrimeArithmetic.Density.primeGtBaseMod_mem_unitResiduePairReps_or_complement
      (base := 30) (hBase := by decide) hPrime hGt)

theorem primeGtThirtyMod_xor_memExpectedPairReps_complement
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 30 < p) :
    Xor' (p % 30 ∈ expectedPairReps) (30 - (p % 30) ∈ expectedPairReps) := by
  simpa [expectedPairReps, unitResiduePairReps_eq_expectedPairReps] using
    (PrimeArithmetic.Density.primeGtBaseMod_xor_mem_unitResiduePairReps_complement
      (base := 30) (hBase := by decide) hPrime hGt)

end PrimeArithmetic.Density.Base30Residues
