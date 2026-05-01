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

theorem wheel30Representable_iff_modEq {m n : ℕ} (h : m ≡ n [MOD 30]) :
    wheel30Representable m ↔ wheel30Representable n := by
  rw [wheel30Representable_iff_mod_mem, wheel30Representable_iff_mod_mem]
  have hMod :
      m % 30 ≡ n % 30 [MOD 30] := by
    exact (Nat.mod_modEq m 30).trans (h.trans (Nat.mod_modEq n 30).symm)
  have hEq : m % 30 = n % 30 := by
    exact hMod.eq_of_lt_of_lt
      (Nat.mod_lt _ (by decide : 0 < 30))
      (Nat.mod_lt _ (by decide : 0 < 30))
  simp [hEq]

theorem wheel30Representable_iff_modThirty (n : ℕ) :
    wheel30Representable (n % 30) ↔ wheel30Representable n :=
  wheel30Representable_iff_modEq (Nat.mod_modEq n 30)

theorem wheel30Representable_sub_base_iff {base n : ℕ}
    (hBase : base % 30 = 0) (hGe : base ≤ n) :
    wheel30Representable (n - base) ↔ wheel30Representable n := by
  rw [wheel30Representable_iff_mod_mem, wheel30Representable_iff_mod_mem]
  have hOffsetMod : (n - base) % 30 = n % 30 := by
    calc
      (n - base) % 30 = (((n - base) % 30) + (base % 30)) % 30 := by simp [hBase]
      _ = ((n - base) + base) % 30 := by
        symm
        exact Nat.add_mod (n - base) base 30
      _ = n % 30 := by rw [Nat.sub_add_cancel hGe]
  simp [hOffsetMod]

theorem wheel30Representable_iff_mod_mem_unitResidues (n : ℕ) :
    wheel30Representable n ↔ n % 30 ∈ PrimeArithmetic.Density.unitResidues 30 := by
  rw [wheel30Representable_iff_mod_mem, wheel30Residues_eq_unitResidues]

theorem wheel30Representable_iff_coprime (n : ℕ) :
    wheel30Representable n ↔ n.Coprime 30 := by
  rw [wheel30Representable_iff_mod_mem, mod_mem_wheel30Residues_iff_coprime]

theorem wheel30Representable_iff_gcd_eq_one (n : ℕ) :
    wheel30Representable n ↔ Nat.gcd n 30 = 1 := by
  rw [wheel30Representable_iff_coprime, Nat.coprime_iff_gcd_eq_one]

theorem wheel30Representable_iff_mod_coprime (n : ℕ) :
    wheel30Representable n ↔ (n % 30).Coprime 30 := by
  exact (wheel30Representable_iff_modThirty n).symm.trans
    (by simpa [Nat.mod_mod] using (wheel30Representable_iff_coprime (n := n % 30)))

theorem wheel30Representable_iff_mod_gcd_eq_one (n : ℕ) :
    wheel30Representable n ↔ Nat.gcd (n % 30) 30 = 1 := by
  exact (wheel30Representable_iff_modThirty n).symm.trans
    (by simpa [Nat.mod_mod] using (wheel30Representable_iff_gcd_eq_one (n := n % 30)))

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

theorem wheel30Representable_of_mem_wheel30Residues {a : ℕ}
    (ha : a ∈ wheel30Residues) :
    wheel30Representable a := by
  refine (wheel30Representable_iff_mod_mem a).2 ?_
  have haLt : a < 30 := (mem_wheel30Residues_iff.1 ha).1
  simpa [Nat.mod_eq_of_lt haLt] using ha

theorem primeGtThirty_wheel30Representable {p : ℕ}
    (hPrime : Nat.Prime p) (hGt : 30 < p) :
    wheel30Representable p := by
  exact (wheel30Representable_iff_mod_mem _).2 <|
    primeGtThirty_mod_mem_wheel30Residues hPrime hGt

theorem wheelCRTRepresentative_wheel30Representable_of_localUnitResidues
    {residue : ℕ → ℕ}
    (h2 : residue 2 ∈ PrimeArithmetic.Density.unitResidues 2)
    (h3 : residue 3 ∈ PrimeArithmetic.Density.unitResidues 3)
    (h5 : residue 5 ∈ PrimeArithmetic.Density.unitResidues 5) :
    wheel30Representable
      (PrimeArithmetic.Density.wheelCRTRepresentative
        (primes := ({2, 3, 5} : Finset ℕ))
        residue
        (by decide) : ℕ) := by
  apply wheel30Representable_of_mem_wheel30Residues
  simpa [wheel30Residues, PrimeArithmetic.Density.Base30Residues.expected] using
    PrimeArithmetic.Density.Base30Residues.wheelCRTRepresentative_memExpected_of_localUnitResidues
      (residue := residue) h2 h3 h5

theorem wheelCRTRepresentative_mods_eq_modThirty (n : ℕ) :
    PrimeArithmetic.Density.wheelCRTRepresentative
      (primes := ({2, 3, 5} : Finset ℕ))
      (fun p => n % p)
      (by decide) = n % 30 := by
  apply PrimeArithmetic.Density.wheelCRTRepresentative_eq_of_forall_modEq
    (primes := ({2, 3, 5} : Finset ℕ))
    (hPrimes := by decide)
    (a := n % 30)
  · simpa [PrimeArithmetic.Density.wheelBase_two_three_five] using
      (Nat.mod_lt n (by decide : 0 < 30))
  · intro p hp
    rcases (by simpa using hp : p = 2 ∨ p = 3 ∨ p = 5) with rfl | rfl | rfl
    · simpa [Nat.mod_mod_of_dvd n (show 2 ∣ 30 by simp)] using
        ((Nat.mod_modEq (n % 30) 2).symm)
    · simpa [Nat.mod_mod_of_dvd n (show 3 ∣ 30 by simp)] using
        ((Nat.mod_modEq (n % 30) 3).symm)
    · simpa [Nat.mod_mod_of_dvd n (show 5 ∣ 30 by simp)] using
        ((Nat.mod_modEq (n % 30) 5).symm)

theorem wheel30Representable_iff_wheelCRTRepresentative (n : ℕ) :
    wheel30Representable
      (PrimeArithmetic.Density.wheelCRTRepresentative
        (primes := ({2, 3, 5} : Finset ℕ))
        (fun p => n % p)
        (by decide) : ℕ) ↔
      wheel30Representable n := by
  simpa [wheelCRTRepresentative_mods_eq_modThirty] using
    (wheel30Representable_iff_modThirty n)

theorem wheel30Representable_iff_wheelCRTRepresentative_mem_wheel30Residues (n : ℕ) :
    wheel30Representable n ↔
      (PrimeArithmetic.Density.wheelCRTRepresentative
        (primes := ({2, 3, 5} : Finset ℕ))
        (fun p => n % p)
        (by decide) : ℕ) ∈ wheel30Residues := by
  rw [← wheel30Representable_iff_wheelCRTRepresentative n, wheel30Representable_iff_mod_mem]
  have hLt :
      (PrimeArithmetic.Density.wheelCRTRepresentative
        (primes := ({2, 3, 5} : Finset ℕ))
        (fun p => n % p)
        (by decide) : ℕ) < 30 := by
    simpa [PrimeArithmetic.Density.wheelBase_two_three_five] using
      (PrimeArithmetic.Density.wheelCRTRepresentative_lt
        (primes := ({2, 3, 5} : Finset ℕ))
        (residue := fun p => n % p)
        (by decide))
  simp [Nat.mod_eq_of_lt hLt]

end PrimeArithmetic.Sieve
