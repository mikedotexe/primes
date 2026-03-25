import Mathlib
import PrimeArithmetic.Density.CoprimeFilter

namespace PrimeArithmetic.Density

/-!
Exact `rad(base)` filter facts.

`radical n` is the product of the distinct prime divisors of `n`, with `radical 0 = 0`.
This keeps the divisibility filter story separate from `Nat.totient`, which counts
admissible residue classes rather than prime divisors.
-/

def radical : ℕ → ℕ
  | 0 => 0
  | n + 1 => ∏ p ∈ (n + 1).primeFactors, p

theorem radical_eq_prod_primeFactors {n : ℕ} (hn : n ≠ 0) :
    radical n = ∏ p ∈ n.primeFactors, p := by
  cases n with
  | zero => contradiction
  | succ _ => rfl

@[simp] theorem radical_zero : radical 0 = 0 := rfl

@[simp] theorem radical_one : radical 1 = 1 := by
  simp [radical]

@[simp] theorem radical_prime {p : ℕ} (hp : p.Prime) : radical p = p := by
  rw [radical_eq_prod_primeFactors hp.ne_zero, hp.primeFactors]
  simp

theorem radical_pos {n : ℕ} (hn : n ≠ 0) : 0 < radical n := by
  rw [radical_eq_prod_primeFactors hn]
  exact Finset.prod_pos fun p hp => Nat.pos_of_mem_primeFactors hp

theorem radical_dvd {n : ℕ} : radical n ∣ n := by
  cases n with
  | zero =>
      simp [radical]
  | succ k =>
      simpa [radical] using Nat.prod_primeFactors_dvd (Nat.succ k)

theorem radical_mul_of_coprime {m n : ℕ} (h : m.Coprime n) :
    radical (m * n) = radical m * radical n := by
  by_cases hm : m = 0
  · subst hm
    simp at h
    simp [radical, h]
  by_cases hn : n = 0
  · subst hn
    simp at h
    simp [radical, h]
  rw [radical_eq_prod_primeFactors (mul_ne_zero hm hn), h.primeFactors_mul]
  rw [Finset.prod_union]
  · rw [radical_eq_prod_primeFactors hm, radical_eq_prod_primeFactors hn]
  · exact h.disjoint_primeFactors

theorem primeFactors_radical (n : ℕ) : (radical n).primeFactors = n.primeFactors := by
  cases n with
  | zero =>
      simp [radical]
  | succ k =>
      simpa [radical] using
        (Nat.primeFactors_prod (s := (Nat.succ k).primeFactors)
          fun p hp => Nat.prime_of_mem_primeFactors hp)

theorem prime_dvd_radical_iff {p n : ℕ} (hp : p.Prime) :
    p ∣ radical n ↔ p ∣ n := by
  cases n with
  | zero =>
      simp [radical]
  | succ k =>
      have hrad : radical (Nat.succ k) ≠ 0 := (radical_pos (Nat.succ_ne_zero k)).ne'
      constructor
      · intro h
        have hmem : p ∈ (radical (Nat.succ k)).primeFactors :=
          hp.mem_primeFactors h hrad
        have hmem' : p ∈ (Nat.succ k).primeFactors := by
          simpa [primeFactors_radical] using hmem
        exact Nat.dvd_of_mem_primeFactors hmem'
      · intro h
        have hmem : p ∈ (Nat.succ k).primeFactors :=
          hp.mem_primeFactors h (Nat.succ_ne_zero k)
        have hmem' : p ∈ (radical (Nat.succ k)).primeFactors := by
          simpa [primeFactors_radical] using hmem
        exact Nat.dvd_of_mem_primeFactors hmem'

theorem coprime_radical_iff {m n : ℕ} :
    m.Coprime (radical n) ↔ m.Coprime n := by
  constructor
  · intro h
    by_contra hmn
    rcases Nat.Prime.not_coprime_iff_dvd.mp hmn with ⟨p, hp, hpm, hpn⟩
    exact
      (Nat.Prime.not_coprime_iff_dvd.mpr
        ⟨p, hp, hpm, (prime_dvd_radical_iff hp).2 hpn⟩) h
  · intro h
    by_contra hmr
    rcases Nat.Prime.not_coprime_iff_dvd.mp hmr with ⟨p, hp, hpm, hpr⟩
    exact
      (Nat.Prime.not_coprime_iff_dvd.mpr
        ⟨p, hp, hpm, (prime_dvd_radical_iff hp).1 hpr⟩) h

theorem gcd_eq_one_radical_iff {m n : ℕ} :
    Nat.gcd m (radical n) = 1 ↔ Nat.gcd m n = 1 := by
  rw [← Nat.coprime_iff_gcd_eq_one, coprime_radical_iff, Nat.coprime_iff_gcd_eq_one]

theorem primeGtBaseCoprimeToRadical
    {p base : ℕ} (hBase : 2 ≤ base) (hPrime : Nat.Prime p) (hGt : base < p) :
    p.Coprime (radical base) :=
  (coprime_radical_iff (m := p) (n := base)).2 <| primeGtBaseCoprime hBase hPrime hGt

theorem primeGtBaseGcdRadicalEqOne
    {p base : ℕ} (hBase : 2 ≤ base) (hPrime : Nat.Prime p) (hGt : base < p) :
    Nat.gcd p (radical base) = 1 :=
  (primeGtBaseCoprimeToRadical hBase hPrime hGt).gcd_eq_one

theorem primeGtTenCoprimeToTenRadical
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 10 < p) :
    p.Coprime (radical 10) :=
  primeGtBaseCoprimeToRadical (by decide : 2 ≤ 10) hPrime hGt

theorem primeGtTenGcdRadicalEqOne
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 10 < p) :
    Nat.gcd p (radical 10) = 1 :=
  (primeGtTenCoprimeToTenRadical hPrime hGt).gcd_eq_one

theorem radical_twelve : radical 12 = 6 := by
  native_decide

theorem radical_hundred : radical 100 = 10 := by
  native_decide

theorem radical_twelve_ne_totient_twelve : radical 12 ≠ Nat.totient 12 := by
  native_decide

end PrimeArithmetic.Density
