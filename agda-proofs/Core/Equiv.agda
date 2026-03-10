{-# OPTIONS --safe --without-K #-}

{-|
  Core.Equiv: Logical Equivalence and Related Concepts

  This module provides the fundamental equivalence types used throughout
  the phase-locks project. Extracted from Core.ResidueClasses to allow
  reuse across multiple modules.

  Key types:
  - _↔_ : Logical equivalence (biconditional)

  Future additions:
  - _≅_ : Type isomorphism
  - _↪_ : Embedding (injection)
  - _↠_ : Surjection
-}

module Core.Equiv where

open import Level using (Level; _⊔_)

-------------------------------------------------------------------------------
-- LOGICAL EQUIVALENCE
-------------------------------------------------------------------------------

{-|
  Logical equivalence: A ↔ B

  A pair of functions witnessing that A and B are logically equivalent.
  This is the "iff" relation in constructive mathematics.
-}
record _↔_ {ℓ₁ ℓ₂ : Level} (A : Set ℓ₁) (B : Set ℓ₂) : Set (ℓ₁ ⊔ ℓ₂) where
  constructor mk↔
  field
    to   : A → B
    from : B → A

open _↔_ public

-------------------------------------------------------------------------------
-- EQUIVALENCE PROPERTIES
-------------------------------------------------------------------------------

-- Reflexivity: A ↔ A
refl↔ : ∀ {ℓ} {A : Set ℓ} → A ↔ A
refl↔ = mk↔ (λ x → x) (λ x → x)

-- Symmetry: A ↔ B → B ↔ A
sym↔ : ∀ {ℓ₁ ℓ₂} {A : Set ℓ₁} {B : Set ℓ₂} →
       A ↔ B → B ↔ A
sym↔ e = mk↔ (from e) (to e)

-- Transitivity: A ↔ B → B ↔ C → A ↔ C
trans↔ : ∀ {ℓ₁ ℓ₂ ℓ₃} {A : Set ℓ₁} {B : Set ℓ₂} {C : Set ℓ₃} →
         A ↔ B → B ↔ C → A ↔ C
trans↔ e₁ e₂ = mk↔
  (λ a → to e₂ (to e₁ a))
  (λ c → from e₁ (from e₂ c))

-------------------------------------------------------------------------------
-- USAGE PATTERNS
-------------------------------------------------------------------------------

{-|
  Usage in other modules:

  ```agda
  open import Core.Equiv using (_↔_; mk↔; refl↔; sym↔; trans↔)
  open _↔_ public  -- if you want to use .to/.from via projection syntax
  ```

  Then you can:
  - Define equivalences: `A↔B : A ↔ B; A↔B = mk↔ f g`
  - Apply them: `to A↔B a` or `from A↔B b`
  - Compose them: `trans↔ A↔B B↔C`
-}

-- End of Core.Equiv module