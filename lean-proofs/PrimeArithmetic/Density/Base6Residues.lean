import Mathlib
import PrimeArithmetic.Density.RadicalFilter
import PrimeArithmetic.Density.UnitResidues

namespace PrimeArithmetic.Density.Base6Residues

def expected : Finset ℕ := ({1, 5} : Finset ℕ)

theorem radical_base : PrimeArithmetic.Density.radical 6 = 6 := by
  native_decide

theorem unitResidues_eq_expected :
    PrimeArithmetic.Density.unitResidues 6 = expected := by
  native_decide

theorem unitResidues_card : (PrimeArithmetic.Density.unitResidues 6).card = 2 := by
  native_decide

theorem totient_base : Nat.totient 6 = 2 := by
  native_decide

theorem primeGtSixMod_memExpected
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 6 < p) :
    p % 6 ∈ expected := by
  simpa [expected, unitResidues_eq_expected] using
    (PrimeArithmetic.Density.primeGtBaseMod_memUnitResidues
      (base := 6) (hBase := by decide) hPrime hGt)

end PrimeArithmetic.Density.Base6Residues
