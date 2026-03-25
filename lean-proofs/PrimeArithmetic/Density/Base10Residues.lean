import Mathlib
import PrimeArithmetic.Density.RadicalFilter
import PrimeArithmetic.Density.UnitResidues

namespace PrimeArithmetic.Density.Base10Residues

def expected : Finset ℕ := ({1, 3, 7, 9} : Finset ℕ)

theorem radical_base : PrimeArithmetic.Density.radical 10 = 10 := by
  native_decide

theorem unitResidues_eq_expected :
    PrimeArithmetic.Density.unitResidues 10 = expected := by
  native_decide

theorem unitResidues_card : (PrimeArithmetic.Density.unitResidues 10).card = 4 := by
  native_decide

theorem totient_base : Nat.totient 10 = 4 := by
  native_decide

theorem primeGtTenMod_memExpected
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 10 < p) :
    p % 10 ∈ expected := by
  simpa [expected, unitResidues_eq_expected] using
    (PrimeArithmetic.Density.primeGtBaseMod_memUnitResidues
      (base := 10) (hBase := by decide) hPrime hGt)

end PrimeArithmetic.Density.Base10Residues
