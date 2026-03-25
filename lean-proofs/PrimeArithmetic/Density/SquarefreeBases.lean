import Mathlib
import PrimeArithmetic.Density.UnitResidues

namespace PrimeArithmetic.Density

/-!
Squarefree-base simplifications.

For squarefree bases, the exact radical filter collapses to the ordinary base
itself: `rad(base) = base`. This is the right generic layer for wheel-like
bases such as `6`, `10`, `30`, and `210`.
-/

theorem radical_eq_self_of_squarefree {base : ℕ} (hSq : Squarefree base) :
    radical base = base := by
  rcases eq_or_ne base 0 with rfl | hBase
  · simp [radical]
  · rw [radical_eq_prod_primeFactors hBase, Nat.prod_primeFactors_of_squarefree hSq]

theorem coprime_radical_eq_base_of_squarefree {m base : ℕ} (hSq : Squarefree base) :
    m.Coprime (radical base) ↔ m.Coprime base := by
  simp [radical_eq_self_of_squarefree hSq]

theorem gcd_eq_one_radical_eq_base_of_squarefree {m base : ℕ} (hSq : Squarefree base) :
    Nat.gcd m (radical base) = 1 ↔ Nat.gcd m base = 1 := by
  simp [radical_eq_self_of_squarefree hSq]

theorem primeGtBaseGcdEqOne_of_squarefree
    {p base : ℕ} (hSq : Squarefree base) (hBase : 2 ≤ base)
    (hPrime : Nat.Prime p) (hGt : base < p) :
    Nat.gcd p base = 1 := by
  simpa [radical_eq_self_of_squarefree hSq] using
    primeGtBaseGcdRadicalEqOne (base := base) hBase hPrime hGt

theorem primeGtBaseModGcdEqOne_of_squarefree
    {p base : ℕ} (hSq : Squarefree base) (hBase : 2 ≤ base)
    (hPrime : Nat.Prime p) (hGt : base < p) :
    Nat.gcd (p % base) base = 1 := by
  simpa [radical_eq_self_of_squarefree hSq] using
    primeGtBaseModGcdRadicalEqOne (base := base) hBase hPrime hGt

theorem radical_six_eq_self : radical 6 = 6 := by
  simpa using radical_eq_self_of_squarefree (show Squarefree 6 by native_decide)

theorem radical_ten_eq_self : radical 10 = 10 := by
  simpa using radical_eq_self_of_squarefree (show Squarefree 10 by native_decide)

theorem radical_thirty_eq_self : radical 30 = 30 := by
  simpa using radical_eq_self_of_squarefree (show Squarefree 30 by native_decide)

theorem radical_twoHundredTen_eq_self : radical 210 = 210 := by
  simpa using radical_eq_self_of_squarefree (show Squarefree 210 by native_decide)

end PrimeArithmetic.Density
