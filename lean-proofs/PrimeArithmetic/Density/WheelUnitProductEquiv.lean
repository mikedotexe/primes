import Mathlib
import PrimeArithmetic.Density.WheelUnitCRT

namespace PrimeArithmetic.Density

/-!
Canonical finite-family CRT on wheel-base unit groups.

`WheelUnitCRT` already proves the wheel-base CRT theorem using an iterated tuple
indexed by `primes.toList`. This module repackages the same theorem in a more
standard finite-family form:

`(ZMod (wheelBase primes))ˣ ≃ ∀ p ∈ primes, (ZMod p)ˣ`.

That keeps the statement closer to textbook CRT language while reusing the
existing exact arithmetic spine.
-/

abbrev wheelUnitFamily (primes : Finset ℕ) : Type :=
  ∀ p : primes, (ZMod (p : ℕ))ˣ

noncomputable def singletonWheelUnitEquiv (p : ℕ) :
    (∀ q : ({p} : Finset ℕ), (ZMod (q : ℕ))ˣ) ≃ (ZMod p)ˣ where
  toFun f := by
    simpa using f ⟨p, by simp⟩
  invFun u := by
    intro q
    rcases q with ⟨q, hq⟩
    simp at hq
    subst hq
    simpa using u
  left_inv f := by
    funext q
    rcases q with ⟨q, hq⟩
    simp at hq
    subst hq
    simp
  right_inv u := by
    simp

noncomputable def wheelUnitFamilyInsertEquiv (p : ℕ) (primes : Finset ℕ) (hp : p ∉ primes) :
    (ZMod p)ˣ × wheelUnitFamily primes ≃ wheelUnitFamily (insert p primes) := by
  let hdisj : Disjoint ({p} : Finset ℕ) primes := Finset.disjoint_singleton_left.mpr hp
  let e1 :
      (ZMod p)ˣ × wheelUnitFamily primes ≃
        (∀ q : ({p} : Finset ℕ), (ZMod (q : ℕ))ˣ) × wheelUnitFamily primes :=
    Equiv.prodCongr (singletonWheelUnitEquiv p).symm (Equiv.refl _)
  have hEq : ({p} ∪ primes : Finset ℕ) = insert p primes := by
    rw [Finset.insert_eq]
  let e2 :
      (∀ q : ({p} ∪ primes : Finset ℕ), (ZMod (q : ℕ))ˣ) ≃ wheelUnitFamily (insert p primes) :=
    Equiv.cast (congrArg (fun s : Finset ℕ => ∀ q : s, (ZMod (q : ℕ))ˣ) hEq)
  exact e1.trans ((Equiv.piFinsetUnion (fun q : ℕ => (ZMod q)ˣ) hdisj).trans e2)

noncomputable def zmodUnitsWheelBaseFamilyEquiv
    {primes : Finset ℕ} (hPrimes : ∀ p ∈ primes, p.Prime) :
    (ZMod (wheelBase primes))ˣ ≃ wheelUnitFamily primes := by
  classical
  refine (primes.strongInductionOn
    (p := fun s => (∀ q ∈ s, q.Prime) → (ZMod (wheelBase s))ˣ ≃ wheelUnitFamily s) ?_) hPrimes
  intro s ih hsPrimes
  by_cases hsEmpty : s = ∅
  · subst hsEmpty
    simpa [wheelUnitFamily, wheelBase] using
      (Equiv.ofUnique (ZMod 1)ˣ (wheelUnitFamily (∅ : Finset ℕ)))
  · let p : ℕ := Classical.choose (Finset.nonempty_iff_ne_empty.mpr hsEmpty)
    have hp : p ∈ s := Classical.choose_spec (Finset.nonempty_iff_ne_empty.mpr hsEmpty)
    let t := s.erase p
    have hpPrime : p.Prime := hsPrimes p hp
    have hpNotMem : p ∉ t := by
      simp [t]
    have hsEq : insert p t = s := by
      simp [t, hp]
    have hsPrimes' : ∀ q ∈ t, q.Prime := by
      intro q hq
      exact hsPrimes q (by simpa [t] using Finset.mem_of_mem_erase hq)
    have hsubset : t ⊂ s := by
      simpa [t] using Finset.erase_ssubset hp
    have hcop : p.Coprime (wheelBase t) := by
      refine Nat.coprime_prod_right_iff.2 ?_
      intro q hq
      refine (Nat.coprime_primes hpPrime (hsPrimes' q hq)).2 ?_
      intro hpq
      exact hpNotMem (hpq ▸ hq)
    haveI : NeZero p := ⟨hpPrime.ne_zero⟩
    haveI : NeZero (wheelBase t) := ⟨wheelBase_ne_zero hsPrimes'⟩
    have hBaseEq : wheelBase s = wheelBase (insert p t) := by
      simp [hsEq]
    let eBase :
        (ZMod (wheelBase s))ˣ ≃ (ZMod (wheelBase (insert p t)))ˣ :=
      Equiv.cast (congrArg (fun n : ℕ => (ZMod n)ˣ) hBaseEq)
    have eInsert :
        (ZMod (wheelBase (insert p t)))ˣ ≃ wheelUnitFamily (insert p t) := by
      have hWheelEq : wheelBase (insert p t) = p * wheelBase t := by
        simp [wheelBase, Finset.prod_insert, hpNotMem]
      calc
        (ZMod (wheelBase (insert p t)))ˣ ≃ (ZMod (p * wheelBase t))ˣ :=
          Equiv.cast (congrArg (fun n : ℕ => (ZMod n)ˣ) hWheelEq)
        _ ≃ (ZMod p)ˣ × (ZMod (wheelBase t))ˣ := zmodUnitsMulEquiv hcop
        _ ≃ (ZMod p)ˣ × wheelUnitFamily t := by
          exact Equiv.prodCongr (Equiv.refl _) (ih t hsubset hsPrimes')
        _ ≃ wheelUnitFamily (insert p t) := wheelUnitFamilyInsertEquiv p t hpNotMem
    let eFamily :
        wheelUnitFamily (insert p t) ≃ wheelUnitFamily s :=
      Equiv.cast (congrArg wheelUnitFamily hsEq)
    exact eBase.trans (eInsert.trans eFamily)

noncomputable def fintypeWheelUnitFamily
    {primes : Finset ℕ} (hPrimes : ∀ p ∈ primes, p.Prime) :
    Fintype (wheelUnitFamily primes) := by
  haveI : NeZero (wheelBase primes) := ⟨wheelBase_ne_zero hPrimes⟩
  exact Fintype.ofEquiv (ZMod (wheelBase primes))ˣ (zmodUnitsWheelBaseFamilyEquiv hPrimes)

theorem card_wheelUnitFamily
    {primes : Finset ℕ} (hPrimes : ∀ p ∈ primes, p.Prime) :
    @Fintype.card (wheelUnitFamily primes) (fintypeWheelUnitFamily hPrimes) =
      ∏ p ∈ primes, (p - 1) := by
  letI := fintypeWheelUnitFamily hPrimes
  haveI : NeZero (wheelBase primes) := ⟨wheelBase_ne_zero hPrimes⟩
  rw [← Fintype.card_congr (zmodUnitsWheelBaseFamilyEquiv hPrimes)]
  rw [ZMod.card_units_eq_totient, totient_wheelBase hPrimes]

end PrimeArithmetic.Density
