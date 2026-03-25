import Mathlib
import PrimeArithmetic.Density.RadicalFilter
import PrimeArithmetic.Density.UnitResidues

namespace PrimeArithmetic.Density.Base12Residues

def expected : Finset ℕ := ({1, 5, 7, 11} : Finset ℕ)

theorem radical_base : PrimeArithmetic.Density.radical 12 = 6 := by
  native_decide

theorem unitResidues_eq_expected :
    PrimeArithmetic.Density.unitResidues 12 = expected := by
  native_decide

theorem unitResidues_card : (PrimeArithmetic.Density.unitResidues 12).card = 4 := by
  native_decide

theorem totient_base : Nat.totient 12 = 4 := by
  native_decide

theorem primeGtTwelveMod_memExpected
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 12 < p) :
    p % 12 ∈ expected := by
  simpa [expected, unitResidues_eq_expected] using
    (PrimeArithmetic.Density.primeGtBaseMod_memUnitResidues
      (base := 12) (hBase := by decide) hPrime hGt)

end PrimeArithmetic.Density.Base12Residues
