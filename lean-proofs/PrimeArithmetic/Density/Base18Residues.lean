import Mathlib
import PrimeArithmetic.Density.RadicalFilter
import PrimeArithmetic.Density.UnitResiduePairs
import PrimeArithmetic.Density.UnitResidues

namespace PrimeArithmetic.Density.Base18Residues

def expected : Finset ℕ := ({1, 5, 7, 11, 13, 17} : Finset ℕ)
def expectedPairReps : Finset ℕ := ({1, 5, 7} : Finset ℕ)

theorem radical_base : PrimeArithmetic.Density.radical 18 = 6 := by
  native_decide

theorem unitResidues_eq_expected :
    PrimeArithmetic.Density.unitResidues 18 = expected := by
  native_decide

theorem unitResidues_card : (PrimeArithmetic.Density.unitResidues 18).card = 6 := by
  native_decide

theorem totient_base : Nat.totient 18 = 6 := by
  native_decide

theorem unitResiduePairReps_eq_expectedPairReps :
    PrimeArithmetic.Density.unitResiduePairReps 18 = expectedPairReps := by
  native_decide

theorem unitResiduePairReps_card :
    (PrimeArithmetic.Density.unitResiduePairReps 18).card = 3 := by
  native_decide

theorem primeGtEighteenMod_memExpected
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 18 < p) :
    p % 18 ∈ expected := by
  simpa [expected, unitResidues_eq_expected] using
    (PrimeArithmetic.Density.primeGtBaseMod_memUnitResidues
      (base := 18) (hBase := by decide) hPrime hGt)

theorem primeGtEighteenMod_memExpectedPairReps_or_complement
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 18 < p) :
    p % 18 ∈ expectedPairReps ∨ 18 - (p % 18) ∈ expectedPairReps := by
  simpa [expectedPairReps, unitResiduePairReps_eq_expectedPairReps] using
    (PrimeArithmetic.Density.primeGtBaseMod_mem_unitResiduePairReps_or_complement
      (base := 18) (hBase := by decide) hPrime hGt)

theorem primeGtEighteenMod_xor_memExpectedPairReps_complement
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 18 < p) :
    Xor' (p % 18 ∈ expectedPairReps) (18 - (p % 18) ∈ expectedPairReps) := by
  simpa [expectedPairReps, unitResiduePairReps_eq_expectedPairReps] using
    (PrimeArithmetic.Density.primeGtBaseMod_xor_mem_unitResiduePairReps_complement
      (base := 18) (hBase := by decide) hPrime hGt)

end PrimeArithmetic.Density.Base18Residues
