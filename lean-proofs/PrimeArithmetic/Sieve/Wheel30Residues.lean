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

theorem mod_mem_wheel30Residues_iff_coprime (n : ℕ) :
    n % 30 ∈ wheel30Residues ↔ n.Coprime 30 := by
  rw [mem_wheel30Residues_iff]
  constructor
  · intro h
    exact (ZMod.coprime_mod_iff_coprime n 30).1 h.2.symm
  · intro h
    exact ⟨Nat.mod_lt _ (by decide), ((ZMod.coprime_mod_iff_coprime n 30).2 h).symm⟩

theorem mod_mem_wheel30Residues_iff_gcd_eq_one (n : ℕ) :
    n % 30 ∈ wheel30Residues ↔ Nat.gcd n 30 = 1 := by
  rw [mod_mem_wheel30Residues_iff_coprime, Nat.coprime_iff_gcd_eq_one]

theorem primeGtFive_mod_mem_wheel30Residues {p : ℕ}
    (hPrime : Nat.Prime p) (hGt : 5 < p) :
    p % 30 ∈ wheel30Residues := by
  refine (mod_mem_wheel30Residues_iff_coprime p).2 ?_
  have hNotDvd2 : ¬ p ∣ 2 := by
    exact Nat.not_dvd_of_pos_of_lt (by decide : 0 < 2) (lt_trans (by decide : 2 < 5) hGt)
  have hNotDvd3 : ¬ p ∣ 3 := by
    exact Nat.not_dvd_of_pos_of_lt (by decide : 0 < 3) (lt_trans (by decide : 3 < 5) hGt)
  have hNotDvd5 : ¬ p ∣ 5 := by
    exact Nat.not_dvd_of_pos_of_lt (by decide : 0 < 5) hGt
  have hNotDvd30 : ¬ p ∣ 30 := by
    change ¬ p ∣ 2 * (3 * 5)
    exact hPrime.not_dvd_mul hNotDvd2 (hPrime.not_dvd_mul hNotDvd3 hNotDvd5)
  exact (hPrime.coprime_iff_not_dvd).2 hNotDvd30

theorem primeGtThirty_mod_mem_wheel30Residues {p : ℕ}
    (hPrime : Nat.Prime p) (hGt : 30 < p) :
    p % 30 ∈ wheel30Residues := by
  exact primeGtFive_mod_mem_wheel30Residues hPrime <|
    lt_trans (by decide : 5 < 30) hGt

end PrimeArithmetic.Sieve
