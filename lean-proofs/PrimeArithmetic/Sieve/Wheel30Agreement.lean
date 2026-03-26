import Mathlib
import PrimeArithmetic.Sieve.SegmentedSieve
import PrimeArithmetic.Sieve.Wheel30Residues

namespace PrimeArithmetic.Sieve

def wheel30Representable (n : ℕ) : Prop :=
  ∃ q r, r ∈ wheel30Residues ∧ n = 30 * q + r

def survivesWheel30 (n : ℕ) : Prop :=
  oddSieveCandidate n ∧ n % 3 ≠ 0 ∧ n % 5 ≠ 0

theorem wheel30Representable_iff_mod_mem (n : ℕ) :
    wheel30Representable n ↔ n % 30 ∈ wheel30Residues := by
  constructor
  · rintro ⟨q, r, hr, rfl⟩
    have hrlt : r < 30 := (mem_wheel30Residues_iff.1 hr).1
    simpa [Nat.add_mod, Nat.mod_eq_of_lt hrlt] using hr
  · intro h
    refine ⟨n / 30, n % 30, h, ?_⟩
    simpa [Nat.mul_comm, Nat.add_comm, Nat.add_left_comm, Nat.add_assoc] using
      (Nat.mod_add_div n 30).symm

theorem wheel30Representable_iff_filters (n : ℕ) :
    wheel30Representable n ↔ Odd n ∧ n % 3 ≠ 0 ∧ n % 5 ≠ 0 := by
  rw [wheel30Representable_iff_mod_mem, mod_mem_wheel30Residues_iff]
  rw [Nat.odd_iff]

theorem wheel30Representable_iff_survivesWheel30 {n : ℕ} (hGe : 5 ≤ n) :
    wheel30Representable n ↔ survivesWheel30 n := by
  rw [wheel30Representable_iff_filters, survivesWheel30, oddSieveCandidate]
  constructor
  · intro h
    exact ⟨⟨le_trans (by decide) hGe, h.1⟩, h.2.1, h.2.2⟩
  · intro h
    exact ⟨h.1.2, h.2.1, h.2.2⟩

theorem wheel30Representable_implies_oddSieveCandidate {n : ℕ}
    (hGe : 5 ≤ n) (hRep : wheel30Representable n) :
    oddSieveCandidate n :=
  (wheel30Representable_iff_survivesWheel30 hGe).1 hRep |>.1

theorem oddSieveCandidate_filters_imply_wheel30Representable {n : ℕ}
    (hGe : 5 ≤ n) (hOdd : oddSieveCandidate n)
    (h3 : n % 3 ≠ 0) (h5 : n % 5 ≠ 0) :
    wheel30Representable n := by
  exact (wheel30Representable_iff_survivesWheel30 hGe).2 ⟨hOdd, h3, h5⟩

end PrimeArithmetic.Sieve
