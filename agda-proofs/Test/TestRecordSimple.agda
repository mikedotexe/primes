module Test.TestRecordSimple where

open import Relation.Binary.PropositionalEquality  using (_≡_)
open import Data.Empty     using (⊥)

_≢_ : ∀ {A : Set} → A → A → Set
x ≢ y = x ≡ y → ⊥

record SymData (B : Set) : Set where
  field
    mid : B
    inv : B → B

record MS (B : Set) : Set₁ where
  field
    X : Set
    res : X → B

record Pairing {B : Set} (S : SymData B) (M : MS B) : Set where
  private
    X' = MS.X M
    res' = MS.res M

  field
    π               : X' → X'
    residue-distinct: (x : X') → res' (π x) ≢ res' x

  -- Now we can open after fields
  open SymData S
  open MS      M
