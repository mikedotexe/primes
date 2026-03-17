module Test.TestRecord where

open import Data.Product     using (Σ; _,_)
open import Relation.Binary.PropositionalEquality  using (_≡_; refl)
open import Data.Empty     using (⊥)

_≢_ : ∀ {A : Set} → A → A → Set
x ≢ y = x ≡ y → ⊥

record TestData (B : Set) : Set where
  field
    mid : B
    inv : B → B

record MS (B : Set) : Set₁ where
  field
    X : Set
    res : X → B

record TestPairing {B : Set} (TD : TestData B) (M : MS B) : Set where
  open TestData TD using (mid; inv)
  open MS M using (X; res)
  field
    π               : X → X
    residue-distinct : (x : X) → res (π x) ≢ res x
