import Mathlib
import PrimeArithmetic.Density.SquarefreeBases

namespace PrimeArithmetic.Density

/-!
Finite prime-product bases.

`wheelBase primes` packages the common “product of distinct small primes”
construction into a reusable Lean surface. This lets the density lane talk
about whole families of bases, not only hand-written concrete examples.
-/

def wheelBase (primes : Finset ℕ) : ℕ :=
  ∏ p ∈ primes, p

theorem wheelBase_ne_zero {primes : Finset ℕ} (hPrimes : ∀ p ∈ primes, p.Prime) :
    wheelBase primes ≠ 0 := by
  refine Finset.prod_ne_zero_iff.2 ?_
  intro p hp
  exact (hPrimes p hp).ne_zero

theorem wheelBase_pos {primes : Finset ℕ} (hPrimes : ∀ p ∈ primes, p.Prime) :
    0 < wheelBase primes :=
  Nat.pos_of_ne_zero (wheelBase_ne_zero hPrimes)

theorem primeFactors_wheelBase {primes : Finset ℕ} (hPrimes : ∀ p ∈ primes, p.Prime) :
    (wheelBase primes).primeFactors = primes := by
  simpa [wheelBase] using Nat.primeFactors_prod hPrimes

theorem squarefree_wheelBase {primes : Finset ℕ} (hPrimes : ∀ p ∈ primes, p.Prime) :
    Squarefree (wheelBase primes) := by
  induction primes using Finset.cons_induction with
  | empty =>
      simp [wheelBase]
  | cons p primes hp ih =>
      have hpPrime : p.Prime := hPrimes p (by simp)
      have hPrimes' : ∀ q ∈ primes, q.Prime := by
        intro q hq
        exact hPrimes q (by simp [hq])
      have hcop : p.Coprime (wheelBase primes) := by
        refine Nat.coprime_prod_right_iff.2 ?_
        intro q hq
        refine (Nat.coprime_primes hpPrime (hPrimes' q hq)).2 ?_
        intro hpq
        exact hp (hpq ▸ hq)
      have hsq : Squarefree (p * wheelBase primes) := by
        exact (Nat.squarefree_mul hcop).2 ⟨hpPrime.squarefree, ih hPrimes'⟩
      simpa [wheelBase, Finset.prod_cons, hp] using hsq

theorem radical_wheelBase {primes : Finset ℕ} (hPrimes : ∀ p ∈ primes, p.Prime) :
    radical (wheelBase primes) = wheelBase primes := by
  exact radical_eq_self_of_squarefree (squarefree_wheelBase hPrimes)

theorem totient_wheelBase {primes : Finset ℕ} (hPrimes : ∀ p ∈ primes, p.Prime) :
    Nat.totient (wheelBase primes) = ∏ p ∈ primes, (p - 1) := by
  induction primes using Finset.cons_induction with
  | empty =>
      simp [wheelBase]
  | cons p primes hp ih =>
      have hpPrime : p.Prime := hPrimes p (by simp)
      have hPrimes' : ∀ q ∈ primes, q.Prime := by
        intro q hq
        exact hPrimes q (by simp [hq])
      have hcop : p.Coprime (wheelBase primes) := by
        refine Nat.coprime_prod_right_iff.2 ?_
        intro q hq
        refine (Nat.coprime_primes hpPrime (hPrimes' q hq)).2 ?_
        intro hpq
        exact hp (hpq ▸ hq)
      have htot :
          Nat.totient (p * wheelBase primes) = (p - 1) * ∏ q ∈ primes, (q - 1) := by
        rw [Nat.totient_mul hcop, Nat.totient_prime hpPrime, ih hPrimes']
      simpa [wheelBase, Finset.prod_cons, hp] using htot

theorem card_unitResidues_wheelBase {primes : Finset ℕ} (hPrimes : ∀ p ∈ primes, p.Prime) :
    (unitResidues (wheelBase primes)).card = ∏ p ∈ primes, (p - 1) := by
  rw [card_unitResidues, totient_wheelBase hPrimes]

theorem two_le_wheelBase {primes : Finset ℕ} (hPrimes : ∀ p ∈ primes, p.Prime)
    (hne : primes.Nonempty) : 2 ≤ wheelBase primes := by
  rcases hne with ⟨p, hp⟩
  have hpPrime : p.Prime := hPrimes p hp
  have hdiv : p ∣ wheelBase primes := by
    simpa [wheelBase] using Finset.dvd_prod_of_mem (fun q => q) hp
  exact le_trans hpPrime.two_le <|
    Nat.le_of_dvd (wheelBase_pos hPrimes) hdiv

theorem primeGtWheelBaseMod_memUnitResidues
    {primes : Finset ℕ} (hPrimes : ∀ q ∈ primes, q.Prime) (hne : primes.Nonempty)
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : wheelBase primes < p) :
    p % wheelBase primes ∈ unitResidues (wheelBase primes) := by
  exact primeGtBaseMod_memUnitResidues (base := wheelBase primes)
    (two_le_wheelBase hPrimes hne) hPrime hGt

theorem primeGtWheelBaseModGcdEqOne
    {primes : Finset ℕ} (hPrimes : ∀ q ∈ primes, q.Prime) (hne : primes.Nonempty)
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : wheelBase primes < p) :
    Nat.gcd (p % wheelBase primes) (wheelBase primes) = 1 := by
  exact primeGtBaseModGcdEqOne_of_squarefree
    (base := wheelBase primes)
    (squarefree_wheelBase hPrimes)
    (two_le_wheelBase hPrimes hne)
    hPrime hGt

theorem wheelBase_two_three : wheelBase ({2, 3} : Finset ℕ) = 6 := by
  native_decide

theorem wheelBase_two_five : wheelBase ({2, 5} : Finset ℕ) = 10 := by
  native_decide

theorem wheelBase_two_three_five : wheelBase ({2, 3, 5} : Finset ℕ) = 30 := by
  native_decide

theorem wheelBase_two_three_five_seven : wheelBase ({2, 3, 5, 7} : Finset ℕ) = 210 := by
  native_decide

theorem wheelBase_two_three_five_seven_eleven :
    wheelBase ({2, 3, 5, 7, 11} : Finset ℕ) = 2310 := by
  native_decide

theorem card_unitResidues_twoHundredTen : (unitResidues 210).card = 48 := by
  calc
    (unitResidues 210).card = (unitResidues (wheelBase ({2, 3, 5, 7} : Finset ℕ))).card := by
      rw [wheelBase_two_three_five_seven]
    _ = ∏ p ∈ ({2, 3, 5, 7} : Finset ℕ), (p - 1) := by
      simpa using
        card_unitResidues_wheelBase
          (primes := ({2, 3, 5, 7} : Finset ℕ))
          (by decide)
    _ = 48 := by
      native_decide

theorem primeGtTwoHundredTenMod_memUnitResidues
    {p : ℕ} (hPrime : Nat.Prime p) (hGt : 210 < p) :
    p % 210 ∈ unitResidues 210 := by
  simpa [wheelBase_two_three_five_seven] using
    primeGtWheelBaseMod_memUnitResidues
      (primes := ({2, 3, 5, 7} : Finset ℕ))
      (by decide)
      (by simp)
      hPrime
      hGt

theorem card_unitResidues_twoThousandThreeHundredTen : (unitResidues 2310).card = 480 := by
  calc
    (unitResidues 2310).card =
        (unitResidues (wheelBase ({2, 3, 5, 7, 11} : Finset ℕ))).card := by
      rw [wheelBase_two_three_five_seven_eleven]
    _ = ∏ p ∈ ({2, 3, 5, 7, 11} : Finset ℕ), (p - 1) := by
      simpa using
        card_unitResidues_wheelBase
          (primes := ({2, 3, 5, 7, 11} : Finset ℕ))
          (by decide)
    _ = 480 := by
      native_decide

end PrimeArithmetic.Density
