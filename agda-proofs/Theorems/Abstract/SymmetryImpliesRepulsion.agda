-- Symmetry Implies Repulsion: Abstract Static Invariant
--
-- REFINED VERSION: Maximally abstract, parameterized over any type B
--
-- THEOREM: Perfect pairing symmetry → Honorary Zero (midpoint void)
--
-- This is the abstract kernel that can be instantiated for:
-- - Fin B (modular arithmetic)
-- - ℕ (natural numbers with distance)
-- - Any type with an involution
--
-- Suitable for machine-checked appendices in publications.

module Theorems.Abstract.SymmetryImpliesRepulsion where

open import Agda.Builtin.Sigma     using (Σ; _,_; proj₁; proj₂)
open import Agda.Builtin.Equality  using (_≡_; refl; sym; cong)
open import Agda.Builtin.Unit      using (⊤; tt)
open import Agda.Builtin.Empty     using (⊥)

------------------------------------------------------------------------
-- Equality transitivity and negation

infixr 40 _∙_
_∙_ : ∀ {A : Set} {x y z : A} → x ≡ y → y ≡ z → x ≡ z
_∙_ refl q = q

¬_ : Set → Set
¬ P = P → ⊥

_≢_ : ∀ {A : Set} → A → A → Set
x ≢ y = x ≡ y → ⊥

------------------------------------------------------------------------
-- SYMMETRY DATA: Abstract involution structure
--
-- We stay abstract about the carrier type B and the involution inv.
-- Only requirements:
-- 1. inv is involutive: inv (inv r) ≡ r
-- 2. mid is a fixed point: inv mid ≡ mid

record SymmetryData (B : Set) : Set where
  constructor mkSym
  field
    mid            : B
    inv            : B → B
    inv-involutive : ∀ r → inv (inv r) ≡ r
    inv-mid        : inv mid ≡ mid

------------------------------------------------------------------------
-- MULTISET STRUCTURE: Occurrences with residue labels
--
-- MS B represents a multiset of residues:
-- - X is the set of occurrences (indices)
-- - res : X → B labels each occurrence with its residue

record MS (B : Set) : Set where
  constructor mkMS
  field
    X   : Set              -- Occurrence index set
    res : X → B            -- Residue labeling

------------------------------------------------------------------------
-- PAIRING WITNESS: Perfect matching on occurrences
--
-- This is the KEY STRUCTURE that forces the honorary zero.
-- Requirements:
-- 1. π is involutive (perfect matching)
-- 2. π has no fixed points (all paired)
-- 3. Residues are distinct under pairing (no self-pairing)
-- 4. π is equivariant w.r.t. inv (geometric pairing)

record Pairing {B : Set} (S : SymmetryData B) (M : MS B) : Set where
  open SymmetryData S using (mid; inv; inv-involutive; inv-mid)
  open MS           M using (X; res)
  field
    π               : X → X
    involutive      : ∀ x → π (π x) ≡ x
    no-fixed        : ∀ x → π x ≢ x
    residue-distinct: ∀ x → res (π x) ≢ res x
    equivariant     : ∀ x → inv (res x) ≡ res (π x)

------------------------------------------------------------------------
-- HONORARY ZERO: The midpoint void
--
-- Definition: No occurrence has residue equal to the midpoint

HonoraryZero : ∀ {B} → (S : SymmetryData B) → (M : MS B) → Set
HonoraryZero S M =
  ¬ (Σ (MS.X M) (λ x → MS.res M x ≡ SymmetryData.mid S))

------------------------------------------------------------------------
-- MAIN THEOREM: Symmetry Implies Repulsion
--
-- If we have a perfect pairing (no self-pairing, equivariant),
-- then the midpoint CANNOT appear (honorary zero).
--
-- PROOF SKETCH:
-- 1. Assume ∃x. res x ≡ mid
-- 2. By equivariance: inv (res x) ≡ res (π x)
-- 3. Since res x ≡ mid and inv mid ≡ mid:
--    res (π x) ≡ inv mid ≡ mid
-- 4. Therefore: res (π x) ≡ res x
-- 5. But residue-distinct says: res (π x) ≢ res x
-- 6. Contradiction! ⊥

SymmetryImpliesRepulsion
  : ∀ {B} (S : SymmetryData B) (M : MS B)
  → Pairing S M
  → HonoraryZero S M
SymmetryImpliesRepulsion {B} S M P (x , resx≡mid) =
  let open SymmetryData S using (mid; inv; inv-mid)
      open MS           M using (res)
      open Pairing      P using (equivariant; residue-distinct)

      -- Step 1: inv (res x) ≡ inv mid
      h₁ : inv (res x) ≡ inv mid
      h₁ = cong inv resx≡mid

      -- Step 2: inv mid ≡ res (π x) (by equivariance)
      h₂ : inv mid ≡ res (P.π x)
      h₂ = sym h₁ ∙ equivariant x

      -- Step 3: res (π x) ≡ mid (since inv mid ≡ mid)
      e₃ : res (P.π x) ≡ mid
      e₃ = sym h₂ ∙ inv-mid

      -- Step 4: res (π x) ≡ res x (combining)
      e₄ : res (P.π x) ≡ res x
      e₄ = e₃ ∙ sym resx≡mid

  in residue-distinct x e₄  -- Contradiction!

------------------------------------------------------------------------
-- INTERPRETATION
------------------------------------------------------------------------

{-
THE CORE INSIGHT:

This theorem proves that the "honorary zero" (midpoint void) is NOT
a separate phenomenon, but a LOGICAL CONSEQUENCE of perfect pairing.

KEY PROPERTIES:
1. Abstract: Works for any type B with an involution
2. Constructive: No classical logic needed
3. Decidable: All proofs are finite and checkable
4. Compositional: Can be instantiated for specific bases

APPLICATIONS:
- Coordinate constellations: B = Fin base, inv r = 2·mid - r
- Natural numbers: B = ℕ, inv n = 2·c - n (for center c)
- General sequences: Any indexed family with symmetry

USAGE IN 2p² FRAMEWORK:
1. Extract residues from window around 2p²
2. Construct pairing witness (mate function)
3. Verify equivariance and distinctness
4. Conclude: midpoint cannot appear (type-checked!)

The void is not a force - it's a structural necessity.
-}
