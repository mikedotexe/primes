import Mathlib
import PrimeArithmetic.Density.Base30Residues
import PrimeArithmetic.Density.UnitResidues

namespace PrimeArithmetic.Sieve

def wheel30Residues : Finset ℕ :=
  ({1, 7, 11, 13, 17, 19, 23, 29} : Finset ℕ)

theorem wheel30Residues_eq_unitResidues :
    wheel30Residues = PrimeArithmetic.Density.unitResidues 30 := by
  simpa [wheel30Residues, PrimeArithmetic.Density.Base30Residues.expected] using
    PrimeArithmetic.Density.Base30Residues.unitResidues_eq_expected.symm

theorem card_wheel30Residues :
    wheel30Residues.card = 8 := by
  simpa [wheel30Residues_eq_unitResidues] using
    PrimeArithmetic.Density.Base30Residues.unitResidues_card

theorem mem_wheel30Residues_iff {a : ℕ} :
    a ∈ wheel30Residues ↔ a < 30 ∧ (30).Coprime a := by
  rw [wheel30Residues_eq_unitResidues]
  simp [PrimeArithmetic.Density.mem_unitResidues]

theorem mod_mem_wheel30Residues_iff (n : ℕ) :
    n % 30 ∈ wheel30Residues ↔ n % 2 = 1 ∧ n % 3 ≠ 0 ∧ n % 5 ≠ 0 := by
  have hmod2 : (n % 30) % 2 = n % 2 := by
    exact Nat.mod_mod_of_dvd n (show 2 ∣ 30 by simp)
  have hmod3 : (n % 30) % 3 = n % 3 := by
    exact Nat.mod_mod_of_dvd n (show 3 ∣ 30 by simp)
  have hmod5 : (n % 30) % 5 = n % 5 := by
    exact Nat.mod_mod_of_dvd n (show 5 ∣ 30 by simp)
  have hfin :
      ∀ m : Fin 30,
        m.1 ∈ wheel30Residues ↔
          m.1 % 2 = 1 ∧ m.1 % 3 ≠ 0 ∧ m.1 % 5 ≠ 0 := by
    intro m
    fin_cases m <;> native_decide
  have hcase := hfin ⟨n % 30, Nat.mod_lt _ (by decide)⟩
  simpa [hmod2, hmod3, hmod5] using hcase

theorem primeGtThirty_mod_mem_wheel30Residues {p : ℕ}
    (hPrime : Nat.Prime p) (hGt : 30 < p) :
    p % 30 ∈ wheel30Residues := by
  simpa [wheel30Residues, PrimeArithmetic.Density.Base30Residues.expected] using
    PrimeArithmetic.Density.Base30Residues.primeGtThirtyMod_memExpected hPrime hGt

end PrimeArithmetic.Sieve
