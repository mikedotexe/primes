import Mathlib

namespace PrimeArithmetic.Density

/-!
Conservative density-facing prerequisite:

If a prime exceeds the base, it cannot divide the base, so the two are
coprime. This is an exact classical fact and not a membrane-specific theorem.
-/

theorem primeGtBaseCoprime
    {p base : ℕ} (hBase : 2 ≤ base) (hPrime : Nat.Prime p) (hGt : base < p) :
    p.Coprime base := by
  have hBasePos : 0 < base := by
    exact lt_of_lt_of_le Nat.zero_lt_two hBase
  have hNotDvd : ¬ p ∣ base :=
    Nat.not_dvd_of_pos_of_lt hBasePos hGt
  exact (hPrime.coprime_iff_not_dvd).2 hNotDvd

theorem primeGtBaseGcdEqOne
    {p base : ℕ} (hBase : 2 ≤ base) (hPrime : Nat.Prime p) (hGt : base < p) :
    Nat.gcd p base = 1 :=
  (primeGtBaseCoprime hBase hPrime hGt).gcd_eq_one

theorem primeGtTenCoprimeToTen
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 10 < p) :
    p.Coprime 10 :=
  primeGtBaseCoprime (by decide : 2 ≤ 10) hPrime hGt

theorem primeGtTenGcdEqOne
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 10 < p) :
    Nat.gcd p 10 = 1 :=
  (primeGtTenCoprimeToTen hPrime hGt).gcd_eq_one

end PrimeArithmetic.Density
