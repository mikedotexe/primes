import Mathlib
import PrimeArithmetic.Density.RadicalFilter
import PrimeArithmetic.Density.UnitResiduePairs
import PrimeArithmetic.Density.UnitResidues

namespace PrimeArithmetic.Density.Base6Residues

def expected : Finset ℕ := ({1, 5} : Finset ℕ)
def expectedPairReps : Finset ℕ := ({1} : Finset ℕ)

theorem radical_base : PrimeArithmetic.Density.radical 6 = 6 := by
  native_decide

theorem unitResidues_eq_expected :
    PrimeArithmetic.Density.unitResidues 6 = expected := by
  native_decide

theorem unitResidues_card : (PrimeArithmetic.Density.unitResidues 6).card = 2 := by
  native_decide

theorem totient_base : Nat.totient 6 = 2 := by
  native_decide

theorem unitResiduePairReps_eq_expectedPairReps :
    PrimeArithmetic.Density.unitResiduePairReps 6 = expectedPairReps := by
  native_decide

theorem unitResiduePairReps_card :
    (PrimeArithmetic.Density.unitResiduePairReps 6).card = 1 := by
  native_decide

theorem primeGtSixMod_memExpected
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 6 < p) :
    p % 6 ∈ expected := by
  simpa [expected, unitResidues_eq_expected] using
    (PrimeArithmetic.Density.primeGtBaseMod_memUnitResidues
      (base := 6) (hBase := by decide) hPrime hGt)

theorem primeGtSixMod_memExpectedPairReps_or_complement
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 6 < p) :
    p % 6 ∈ expectedPairReps ∨ 6 - (p % 6) ∈ expectedPairReps := by
  simpa [expectedPairReps, unitResiduePairReps_eq_expectedPairReps] using
    (PrimeArithmetic.Density.primeGtBaseMod_mem_unitResiduePairReps_or_complement
      (base := 6) (hBase := by decide) hPrime hGt)

theorem primeGtSixMod_xor_memExpectedPairReps_complement
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 6 < p) :
    Xor' (p % 6 ∈ expectedPairReps) (6 - (p % 6) ∈ expectedPairReps) := by
  simpa [expectedPairReps, unitResiduePairReps_eq_expectedPairReps] using
    (PrimeArithmetic.Density.primeGtBaseMod_xor_mem_unitResiduePairReps_complement
      (base := 6) (hBase := by decide) hPrime hGt)

end PrimeArithmetic.Density.Base6Residues
