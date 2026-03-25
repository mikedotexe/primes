import Mathlib
import PrimeArithmetic.Density.RadicalFilter
import PrimeArithmetic.Density.UnitResidues

namespace PrimeArithmetic.Density.Base30Residues

def expected : Finset ℕ := ({1, 7, 11, 13, 17, 19, 23, 29} : Finset ℕ)

theorem radical_base : PrimeArithmetic.Density.radical 30 = 30 := by
  native_decide

theorem unitResidues_eq_expected :
    PrimeArithmetic.Density.unitResidues 30 = expected := by
  native_decide

theorem unitResidues_card : (PrimeArithmetic.Density.unitResidues 30).card = 8 := by
  native_decide

theorem totient_base : Nat.totient 30 = 8 := by
  native_decide

theorem primeGtThirtyMod_memExpected
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 30 < p) :
    p % 30 ∈ expected := by
  simpa [expected, unitResidues_eq_expected] using
    (PrimeArithmetic.Density.primeGtBaseMod_memUnitResidues
      (base := 30) (hBase := by decide) hPrime hGt)

end PrimeArithmetic.Density.Base30Residues
