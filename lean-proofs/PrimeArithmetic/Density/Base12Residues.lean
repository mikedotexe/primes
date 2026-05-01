import Mathlib
import PrimeArithmetic.Density.RadicalFilter
import PrimeArithmetic.Density.UnitResiduePairs
import PrimeArithmetic.Density.UnitResidues

namespace PrimeArithmetic.Density.Base12Residues

def expected : Finset ℕ := ({1, 5, 7, 11} : Finset ℕ)
def expectedPairReps : Finset ℕ := ({1, 5} : Finset ℕ)

theorem radical_base : PrimeArithmetic.Density.radical 12 = 6 := by
  native_decide

theorem unitResidues_eq_expected :
    PrimeArithmetic.Density.unitResidues 12 = expected := by
  native_decide

theorem unitResidues_card : (PrimeArithmetic.Density.unitResidues 12).card = 4 := by
  native_decide

theorem totient_base : Nat.totient 12 = 4 := by
  native_decide

theorem unitResiduePairReps_eq_expectedPairReps :
    PrimeArithmetic.Density.unitResiduePairReps 12 = expectedPairReps := by
  native_decide

theorem unitResiduePairReps_card :
    (PrimeArithmetic.Density.unitResiduePairReps 12).card = 2 := by
  native_decide

theorem primeGtTwelveMod_memExpected
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 12 < p) :
    p % 12 ∈ expected := by
  simpa [expected, unitResidues_eq_expected] using
    (PrimeArithmetic.Density.primeGtBaseMod_memUnitResidues
      (base := 12) (hBase := by decide) hPrime hGt)

theorem primeGtTwelveMod_memExpectedPairReps_or_complement
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 12 < p) :
    p % 12 ∈ expectedPairReps ∨ 12 - (p % 12) ∈ expectedPairReps := by
  simpa [expectedPairReps, unitResiduePairReps_eq_expectedPairReps] using
    (PrimeArithmetic.Density.primeGtBaseMod_mem_unitResiduePairReps_or_complement
      (base := 12) (hBase := by decide) hPrime hGt)

theorem primeGtTwelveMod_xor_memExpectedPairReps_complement
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 12 < p) :
    Xor' (p % 12 ∈ expectedPairReps) (12 - (p % 12) ∈ expectedPairReps) := by
  simpa [expectedPairReps, unitResiduePairReps_eq_expectedPairReps] using
    (PrimeArithmetic.Density.primeGtBaseMod_xor_mem_unitResiduePairReps_complement
      (base := 12) (hBase := by decide) hPrime hGt)

end PrimeArithmetic.Density.Base12Residues
