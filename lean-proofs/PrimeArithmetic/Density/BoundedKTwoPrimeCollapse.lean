import PrimeArithmetic.Structure.BoundedKTransferCollapse

namespace PrimeArithmetic.Density

open PrimeArithmetic.Structure

/-!
Even squarefree `2p`-style transfer-collapse wrappers.

This module does **not** prove a universal `M >= 3` theorem. Instead it fixes
the theorem-family shape for the maintained even squarefree `2p` track:

- if a direct lane comparison on a base of the form `2 * p` satisfies the exact
  residue-profile agreement criterion,
- then the corresponding transfer comparison collapses to identity.

The stage-1 audit determines whether any stronger public theorem is warranted.
-/

theorem profileAgreementOn_twoPrime_implies_transferIdentity
    {p : ℕ} (hp : Nat.Prime p) (hpNeTwo : p ≠ 2)
    (cfgFrom cfgTo : BoundedKConfig)
    (outer inner middleWidth : ℕ)
    (moduli : Finset ℕ)
    (hcop : ∀ modulus ∈ moduli, modulus.Coprime (2 * p))
    (hmod : ∀ modulus ∈ moduli, modulus ≠ 0)
    (s : Finset ℕ)
    (hagree : profileAgreementOn cfgFrom cfgTo (2 * p) outer inner middleWidth moduli hcop) :
    bucketCount s
        (seedMask cfgFrom (2 * p) outer inner middleWidth moduli)
        (seedMask cfgTo (2 * p) outer inner middleWidth moduli)
        .gainZero = 0
      ∧ bucketCount s
          (seedMask cfgFrom (2 * p) outer inner middleWidth moduli)
          (seedMask cfgTo (2 * p) outer inner middleWidth moduli)
          .lossZero = 0
      ∧ bucketCount s
          (seedMask cfgFrom (2 * p) outer inner middleWidth moduli)
          (seedMask cfgTo (2 * p) outer inner middleWidth moduli)
          .nonzeroChurn = 0
      ∧ admissibleCountFrom s
          (seedMask cfgFrom (2 * p) outer inner middleWidth moduli)
          (seedMask cfgTo (2 * p) outer inner middleWidth moduli) =
        admissibleCountTo s
          (seedMask cfgFrom (2 * p) outer inner middleWidth moduli)
          (seedMask cfgTo (2 * p) outer inner middleWidth moduli)
      ∧ admissibleDeltaCount s
          (seedMask cfgFrom (2 * p) outer inner middleWidth moduli)
          (seedMask cfgTo (2 * p) outer inner middleWidth moduli) = 0 := by
  let _ := hp
  let _ := hpNeTwo
  exact profileAgreement_implies_transferIdentity
    cfgFrom cfgTo (2 * p) outer inner middleWidth moduli hcop hmod s hagree

theorem profileAgreementOn_baseSix_implies_transferIdentity
    (cfgFrom cfgTo : BoundedKConfig)
    (outer inner middleWidth : ℕ)
    (moduli : Finset ℕ)
    (hcop : ∀ modulus ∈ moduli, modulus.Coprime 6)
    (hmod : ∀ modulus ∈ moduli, modulus ≠ 0)
    (s : Finset ℕ)
    (hagree : profileAgreementOn cfgFrom cfgTo 6 outer inner middleWidth moduli hcop) :
    bucketCount s
        (seedMask cfgFrom 6 outer inner middleWidth moduli)
        (seedMask cfgTo 6 outer inner middleWidth moduli)
        .gainZero = 0 := by
  exact (profileAgreementOn_twoPrime_implies_transferIdentity
    (p := 3) (by decide) (by decide)
    cfgFrom cfgTo outer inner middleWidth moduli hcop hmod s hagree).1

end PrimeArithmetic.Density
