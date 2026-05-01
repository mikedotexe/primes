import PrimeArithmetic.Structure.BoundedKTransferCollapse
import PrimeArithmetic.Density.WheelBases

namespace PrimeArithmetic.Density

open PrimeArithmetic.Structure

/-!
Wheel-base transfer-collapse wrappers.

This is the wheel-like parallel to the even-squarefree `2p` track. It keeps
the theorem family aligned:

- if a direct lane comparison on a wheel base satisfies the exact profile
  agreement criterion,
- then the transfer comparison collapses to identity.

The stage-1 audit decides whether any stronger wheel-base statement survives.
-/

theorem profileAgreementOn_wheelBase_implies_transferIdentity
    {primes : Finset ℕ} (hPrimes : ∀ q ∈ primes, q.Prime)
    (cfgFrom cfgTo : BoundedKConfig)
    (outer inner middleWidth : ℕ)
    (moduli : Finset ℕ)
    (hcop : ∀ modulus ∈ moduli, modulus.Coprime (wheelBase primes))
    (hmod : ∀ modulus ∈ moduli, modulus ≠ 0)
    (s : Finset ℕ)
    (hagree :
      profileAgreementOn cfgFrom cfgTo (wheelBase primes) outer inner middleWidth moduli hcop) :
    bucketCount s
        (seedMask cfgFrom (wheelBase primes) outer inner middleWidth moduli)
        (seedMask cfgTo (wheelBase primes) outer inner middleWidth moduli)
        .gainZero = 0
      ∧ bucketCount s
          (seedMask cfgFrom (wheelBase primes) outer inner middleWidth moduli)
          (seedMask cfgTo (wheelBase primes) outer inner middleWidth moduli)
          .lossZero = 0
      ∧ bucketCount s
          (seedMask cfgFrom (wheelBase primes) outer inner middleWidth moduli)
          (seedMask cfgTo (wheelBase primes) outer inner middleWidth moduli)
          .nonzeroChurn = 0
      ∧ admissibleCountFrom s
          (seedMask cfgFrom (wheelBase primes) outer inner middleWidth moduli)
          (seedMask cfgTo (wheelBase primes) outer inner middleWidth moduli) =
        admissibleCountTo s
          (seedMask cfgFrom (wheelBase primes) outer inner middleWidth moduli)
          (seedMask cfgTo (wheelBase primes) outer inner middleWidth moduli)
      ∧ admissibleDeltaCount s
          (seedMask cfgFrom (wheelBase primes) outer inner middleWidth moduli)
          (seedMask cfgTo (wheelBase primes) outer inner middleWidth moduli) = 0 := by
  let _ := radical_wheelBase hPrimes
  exact profileAgreement_implies_transferIdentity
    cfgFrom cfgTo (wheelBase primes) outer inner middleWidth moduli hcop hmod s hagree

theorem profileAgreementOn_baseThirty_implies_transferIdentity
    (cfgFrom cfgTo : BoundedKConfig)
    (outer inner middleWidth : ℕ)
    (moduli : Finset ℕ)
    (hcop : ∀ modulus ∈ moduli, modulus.Coprime 30)
    (hmod : ∀ modulus ∈ moduli, modulus ≠ 0)
    (s : Finset ℕ)
    (hagree : profileAgreementOn cfgFrom cfgTo 30 outer inner middleWidth moduli hcop) :
    bucketCount s
        (seedMask cfgFrom 30 outer inner middleWidth moduli)
        (seedMask cfgTo 30 outer inner middleWidth moduli)
        .gainZero = 0 := by
  exact (profileAgreementOn_wheelBase_implies_transferIdentity
    (primes := ({2, 3, 5} : Finset ℕ)) (by decide)
    cfgFrom cfgTo outer inner middleWidth moduli
    (by
      intro modulus hmodulus
      simpa [wheelBase_two_three_five] using hcop modulus hmodulus)
    hmod s
    (by simpa [wheelBase_two_three_five] using hagree)).1

theorem profileAgreementOn_baseTwoHundredTen_implies_transferIdentity
    (cfgFrom cfgTo : BoundedKConfig)
    (outer inner middleWidth : ℕ)
    (moduli : Finset ℕ)
    (hcop : ∀ modulus ∈ moduli, modulus.Coprime 210)
    (hmod : ∀ modulus ∈ moduli, modulus ≠ 0)
    (s : Finset ℕ)
    (hagree : profileAgreementOn cfgFrom cfgTo 210 outer inner middleWidth moduli hcop) :
    bucketCount s
        (seedMask cfgFrom 210 outer inner middleWidth moduli)
        (seedMask cfgTo 210 outer inner middleWidth moduli)
        .gainZero = 0 := by
  exact (profileAgreementOn_wheelBase_implies_transferIdentity
    (primes := ({2, 3, 5, 7} : Finset ℕ)) (by decide)
    cfgFrom cfgTo outer inner middleWidth moduli
    (by
      intro modulus hmodulus
      simpa [wheelBase_two_three_five_seven] using hcop modulus hmodulus)
    hmod s
    (by simpa [wheelBase_two_three_five_seven] using hagree)).1

end PrimeArithmetic.Density
