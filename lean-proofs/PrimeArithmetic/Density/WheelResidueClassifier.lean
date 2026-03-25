import Mathlib
import PrimeArithmetic.Density.WheelBases

namespace PrimeArithmetic.Density

/-!
Finite-CRT classification for wheel bases.

For a wheel base built from distinct primes, admissible residues are exactly the
numbers below the base that stay nonzero modulo every prime factor. This module
also exposes the finite-set CRT constructor that assembles a global residue from
local congruence data.
-/

theorem pairwiseCoprime_primes {primes : Finset ℕ} (hPrimes : ∀ p ∈ primes, p.Prime) :
    Set.Pairwise primes fun p q : ℕ => p.Coprime q := by
  intro p hp q hq hpq
  exact (Nat.coprime_primes (hPrimes p hp) (hPrimes q hq)).2 hpq

theorem coprime_wheelBase_iff_forall_mod_ne_zero
    {primes : Finset ℕ} (hPrimes : ∀ p ∈ primes, p.Prime) {a : ℕ} :
    a.Coprime (wheelBase primes) ↔ ∀ p ∈ primes, a % p ≠ 0 := by
  rw [wheelBase, Nat.coprime_prod_right_iff]
  constructor
  · intro h p hp hmod
    exact (((hPrimes p hp).coprime_iff_not_dvd).1 (h p hp).symm) <|
      Nat.dvd_of_mod_eq_zero hmod
  · intro h p hp
    have hnodiv : ¬ p ∣ a := by
      intro hdiv
      exact h p hp (Nat.mod_eq_zero_of_dvd hdiv)
    exact (((hPrimes p hp).coprime_iff_not_dvd).2 hnodiv).symm

theorem mem_unitResidues_wheelBase_iff_mod_ne_zero
    {primes : Finset ℕ} (hPrimes : ∀ p ∈ primes, p.Prime) {a : ℕ} :
    a ∈ unitResidues (wheelBase primes) ↔
      a < wheelBase primes ∧ ∀ p ∈ primes, a % p ≠ 0 := by
  rw [mem_unitResidues]
  constructor
  · rintro ⟨ha, hcop⟩
    exact ⟨ha, (coprime_wheelBase_iff_forall_mod_ne_zero hPrimes).1 hcop.symm⟩
  · rintro ⟨ha, hmods⟩
    exact ⟨ha, ((coprime_wheelBase_iff_forall_mod_ne_zero hPrimes).2 hmods).symm⟩

theorem mod_mem_unitResidues_prime_iff_ne_zero {a p : ℕ} (hp : p.Prime) :
    a % p ∈ unitResidues p ↔ a % p ≠ 0 := by
  rw [mem_unitResidues]
  have hlt : a % p < p := Nat.mod_lt _ hp.pos
  constructor
  · rintro ⟨_, hcop⟩ hzero
    have hcop0 : p.Coprime 0 := by
      simpa [hzero] using hcop
    exact hp.ne_one (by simpa using hcop0)
  · intro hne
    refine ⟨hlt, ?_⟩
    exact (hp.coprime_iff_not_dvd).2 fun hdiv =>
      hne ((Nat.mod_eq_of_lt hlt).symm.trans (Nat.mod_eq_zero_of_dvd hdiv))

theorem mem_unitResidues_wheelBase_iff_primeUnitResidues
    {primes : Finset ℕ} (hPrimes : ∀ p ∈ primes, p.Prime) {a : ℕ} :
    a ∈ unitResidues (wheelBase primes) ↔
      a < wheelBase primes ∧ ∀ p ∈ primes, a % p ∈ unitResidues p := by
  rw [mem_unitResidues_wheelBase_iff_mod_ne_zero hPrimes]
  constructor
  · rintro ⟨ha, hmods⟩
    refine ⟨ha, ?_⟩
    intro p hp
    exact (mod_mem_unitResidues_prime_iff_ne_zero (hPrimes p hp)).2 (hmods p hp)
  · rintro ⟨ha, hmods⟩
    refine ⟨ha, ?_⟩
    intro p hp
    exact (mod_mem_unitResidues_prime_iff_ne_zero (hPrimes p hp)).1 (hmods p hp)

/-- The canonical CRT representative below `wheelBase primes` with the requested
local residues modulo each prime factor. -/
def wheelCRTRepresentative {primes : Finset ℕ} (residue : ℕ → ℕ)
    (hPrimes : ∀ p ∈ primes, p.Prime) :
    { k : ℕ // ∀ p ∈ primes, k ≡ residue p [MOD p] } :=
  Nat.chineseRemainderOfFinset (a := residue) (s := fun p : ℕ => p) (t := primes)
    (fun p hp => (hPrimes p hp).ne_zero)
    (pairwiseCoprime_primes hPrimes)

theorem wheelCRTRepresentative_lt {primes : Finset ℕ} (residue : ℕ → ℕ)
    (hPrimes : ∀ p ∈ primes, p.Prime) :
    wheelCRTRepresentative residue hPrimes < wheelBase primes := by
  simpa [wheelCRTRepresentative, wheelBase] using
    Nat.chineseRemainderOfFinset_lt_prod (a := residue) (s := fun p : ℕ => p) (t := primes)
      (fun p hp => (hPrimes p hp).ne_zero)
      (pairwiseCoprime_primes hPrimes)

theorem wheelCRTRepresentative_modEq {primes : Finset ℕ} (residue : ℕ → ℕ)
    (hPrimes : ∀ p ∈ primes, p.Prime) {p : ℕ} (hp : p ∈ primes) :
    wheelCRTRepresentative residue hPrimes ≡ residue p [MOD p] :=
  (wheelCRTRepresentative residue hPrimes).property p hp

theorem wheelCRTRepresentative_mod_eq {primes : Finset ℕ} (residue : ℕ → ℕ)
    (hPrimes : ∀ p ∈ primes, p.Prime) {p : ℕ} (hp : p ∈ primes)
    (hResidueLt : residue p < p) :
    wheelCRTRepresentative residue hPrimes % p = residue p := by
  apply Nat.ModEq.eq_of_lt_of_lt
  · exact (Nat.mod_modEq _ _).trans (wheelCRTRepresentative_modEq residue hPrimes hp)
  · exact Nat.mod_lt _ (hPrimes p hp).pos
  · exact hResidueLt

theorem wheelCRTRepresentative_mem_unitResidues
    {primes : Finset ℕ} {residue : ℕ → ℕ} (hPrimes : ∀ p ∈ primes, p.Prime)
    (hResidue : ∀ p ∈ primes, residue p ∈ unitResidues p) :
    (wheelCRTRepresentative residue hPrimes : ℕ) ∈ unitResidues (wheelBase primes) := by
  rw [mem_unitResidues_wheelBase_iff_primeUnitResidues hPrimes]
  refine ⟨wheelCRTRepresentative_lt residue hPrimes, ?_⟩
  intro p hp
  have hEq :=
    wheelCRTRepresentative_mod_eq residue hPrimes hp ((mem_unitResidues.1 (hResidue p hp)).1)
  simpa [hEq] using hResidue p hp

theorem mem_unitResidues_twoHundredTen_iff {a : ℕ} :
    a ∈ unitResidues 210 ↔
      a < 210 ∧ a % 2 ≠ 0 ∧ a % 3 ≠ 0 ∧ a % 5 ≠ 0 ∧ a % 7 ≠ 0 := by
  simpa [wheelBase_two_three_five_seven] using
    (mem_unitResidues_wheelBase_iff_mod_ne_zero
      (primes := ({2, 3, 5, 7} : Finset ℕ))
      (by decide)
      (a := a))

theorem mem_unitResidues_twoThousandThreeHundredTen_iff {a : ℕ} :
    a ∈ unitResidues 2310 ↔
      a < 2310 ∧ a % 2 ≠ 0 ∧ a % 3 ≠ 0 ∧ a % 5 ≠ 0 ∧ a % 7 ≠ 0 ∧ a % 11 ≠ 0 := by
  simpa [wheelBase_two_three_five_seven_eleven] using
    (mem_unitResidues_wheelBase_iff_mod_ne_zero
      (primes := ({2, 3, 5, 7, 11} : Finset ℕ))
      (by decide)
      (a := a))

end PrimeArithmetic.Density
