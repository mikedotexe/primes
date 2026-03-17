{-# OPTIONS --without-K --safe #-}

{-|
  Residue classes with a current, stable public interface.

  This module now serves as the maintained foundation layer for the residue
  ring API. The data representation and basic operations are executable; the
  basic algebraic laws and the unit/coprime bridge are constructive. The
  residue-ring package itself is maintained directly in this file.
-}

module Core.ResidueClassesComplete where

open import Data.Nat using
  ( ℕ; zero; suc; _+_; _*_; _∸_; _<_; _≤_; _≥_; _>_; _≟_; z≤n; s≤s )
open import Data.Nat.Base using (NonZero; >-nonZero)
open import Data.Nat.Properties using
  (+-comm; +-assoc; +-identityˡ; +-identityʳ
  ; *-comm; *-assoc; *-suc; *-identityˡ; *-identityʳ; *-distribˡ-+
  ; ≤-refl; <⇒≤; m∸n+n≡m )
open import Data.Nat.DivMod using
  ( _%_; _/_; %-congˡ; m%n<n; m%n%n≡m%n; [m+kn]%n≡m%n
  ; m<n⇒m%n≡m; m*n%n≡0; n%n≡0; %-distribˡ-+; %-distribˡ-*; %-pred-≡0
  ; m≡m%n+[m/n]*n )
open import Data.Nat.GCD using (gcd; gcd-GCD; GCD; module Bézout)
open import Data.Nat.Divisibility using (_∣_; ∣1⇒≡1; ∣m+n∣m⇒∣n; ∣m⇒∣m*n; ∣n⇒∣m*n)
open import Data.Nat.Coprimality as NatCoprime using (gcd≡1⇒coprime; coprime-Bézout)
open import Data.Product using (_×_; _,_; ∃; Σ-syntax)
open import Relation.Binary using (IsEquivalence)
open import Relation.Binary.PropositionalEquality using (_≡_; refl; cong; cong₂; subst; sym; trans)
open import Relation.Binary.PropositionalEquality.Properties using (module ≡-Reasoning)
open import Relation.Nullary using (Dec; yes; no)
open import Algebra.Structures using (IsCommutativeRing)

open import Core.Equiv using (_↔_; mk↔)
open ≡-Reasoning

instance
  nonZero-from-positive : ∀ {m} {m>0 : m > 0} → NonZero m
  nonZero-from-positive {m>0 = m>0} = >-nonZero m>0

record ResidueClass (m : ℕ) {m>0 : m > 0} : Set where
  constructor mkResidueClass
  field
    representative : ℕ
    valid : representative < m

open ResidueClass public

[_]mod_ : ∀ {m} {m>0 : m > 0} → (r : ℕ) → (r<m : r < m) → ResidueClass m {m>0}
[ r ]mod r<m = mkResidueClass r r<m

⟦_⟧ : ∀ {m} {m>0 : m > 0} → ResidueClass m {m>0} → ℕ
⟦_⟧ = representative

modulo : ∀ {m} {m>0 : m > 0} → ℕ → ℕ
modulo {m} {m>0} n = _%_ n m {{>-nonZero m>0}}

infix 4 _≡ᵣ_ _≟ᵣ_
infixl 6 _⊕_
infixl 7 _⊗_

_≡ᵣ_ : ∀ {m} {m>0 : m > 0} → ResidueClass m {m>0} → ResidueClass m {m>0} → Set
a ≡ᵣ b = ⟦ a ⟧ ≡ ⟦ b ⟧

_≟ᵣ_ : ∀ {m} {m>0 : m > 0} (a b : ResidueClass m {m>0}) → Dec (a ≡ᵣ b)
mkResidueClass r₁ _ ≟ᵣ mkResidueClass r₂ _ with r₁ ≟ r₂
... | yes eq = yes eq
... | no ne = no ne

_⊕_ : ∀ {m} {m>0 : m > 0} → ResidueClass m {m>0} → ResidueClass m {m>0} → ResidueClass m {m>0}
_⊕_ {m} {m>0} (mkResidueClass r₁ _) (mkResidueClass r₂ _) =
  [ (r₁ + r₂) % m ]mod (m%n<n (r₁ + r₂) m {{>-nonZero m>0}})

_⊗_ : ∀ {m} {m>0 : m > 0} → ResidueClass m {m>0} → ResidueClass m {m>0} → ResidueClass m {m>0}
_⊗_ {m} {m>0} (mkResidueClass r₁ _) (mkResidueClass r₂ _) =
  [ (r₁ * r₂) % m ]mod (m%n<n (r₁ * r₂) m {{>-nonZero m>0}})

0ᵣ : ∀ {m} {m>0 : m > 0} → ResidueClass m {m>0}
0ᵣ {m} {m>0} = [ 0 ]mod m>0

1ᵣ : ∀ {m} {m>0 : m > 0} → m ≥ 2 → ResidueClass m {m>0}
1ᵣ {m} {m>0} m≥2 = [ 1 ]mod m≥2

⊖_ : ∀ {m} {m>0 : m > 0} → ResidueClass m {m>0} → ResidueClass m {m>0}
⊖_ {m} {m>0} a =
  [ (m ∸ ⟦ a ⟧) % m ]mod (m%n<n (m ∸ ⟦ a ⟧) m {{>-nonZero m>0}})

m>1⇒m≥2 : ∀ {m} → m > 1 → m ≥ 2
m>1⇒m≥2 m>1 = m>1

pred<m : ∀ {m} → m > 1 → m ∸ 1 < m
pred<m {suc zero} p with p
... | s≤s ()
pred<m {suc (suc _)} _ = ≤-refl

pred-square-expansion : ∀ k → suc k * suc k ≡ 1 + k * suc (suc k)
pred-square-expansion k = begin
  suc k * suc k         ≡⟨ *-suc (suc k) k ⟩
  suc k + suc k * k     ≡⟨ cong (suc k +_) (*-comm (suc k) k) ⟩
  suc k + k * suc k     ≡⟨⟩
  suc (k + k * suc k)   ≡⟨ cong suc (sym (*-suc k (suc k))) ⟩
  suc (k * suc (suc k)) ≡⟨⟩
  1 + k * suc (suc k)   ∎

neg1-square-mod : ∀ {m} .{{_ : NonZero m}} → m > 1 → ((m ∸ 1) * (m ∸ 1)) % m ≡ 1
neg1-square-mod {suc zero} p with p
... | s≤s ()
neg1-square-mod {suc (suc k)} m>1 = begin
  (suc k * suc k) % suc (suc k)       ≡⟨ %-congˡ {o = suc (suc k)} {{nz-succsucc}} (pred-square-expansion k) ⟩
  (1 + k * suc (suc k)) % suc (suc k) ≡⟨ [m+kn]%n≡m%n 1 k (suc (suc k)) {{nz-succsucc}} ⟩
  1 % suc (suc k)                     ≡⟨ m<n⇒m%n≡m {{nz-succsucc}} m>1 ⟩
  1                                 ∎
  where
    instance
      nz-succsucc : NonZero (suc (suc k))
      nz-succsucc = >-nonZero (s≤s z≤n)

⊕-assoc : ∀ {m} {m>0 : m > 0} (a b c : ResidueClass m {m>0}) →
  (a ⊕ b) ⊕ c ≡ᵣ a ⊕ (b ⊕ c)
⊕-assoc {m} {m>0} (mkResidueClass r₁ r₁<m) (mkResidueClass r₂ _) (mkResidueClass r₃ r₃<m) = begin
  modulo {m} {m>0} (modulo {m} {m>0} (r₁ + r₂) + r₃)
    ≡⟨ cong (λ t → modulo {m} {m>0} (modulo {m} {m>0} (r₁ + r₂) + t))
            (sym (m<n⇒m%n≡m {{>-nonZero m>0}} r₃<m)) ⟩
  modulo {m} {m>0} (modulo {m} {m>0} (r₁ + r₂) + modulo {m} {m>0} r₃)
    ≡⟨ sym (%-distribˡ-+ (r₁ + r₂) r₃ m {{>-nonZero m>0}}) ⟩
  modulo {m} {m>0} ((r₁ + r₂) + r₃)
    ≡⟨ cong (modulo {m} {m>0}) (+-assoc r₁ r₂ r₃) ⟩
  modulo {m} {m>0} (r₁ + (r₂ + r₃))
    ≡⟨ %-distribˡ-+ r₁ (r₂ + r₃) m {{>-nonZero m>0}} ⟩
  modulo {m} {m>0} (modulo {m} {m>0} r₁ + modulo {m} {m>0} (r₂ + r₃))
    ≡⟨ cong (λ t → modulo {m} {m>0} (t + modulo {m} {m>0} (r₂ + r₃)))
            (m<n⇒m%n≡m {{>-nonZero m>0}} r₁<m) ⟩
  modulo {m} {m>0} (r₁ + modulo {m} {m>0} (r₂ + r₃))
  ∎

⊕-comm : ∀ {m} {m>0 : m > 0} (a b : ResidueClass m {m>0}) →
  a ⊕ b ≡ᵣ b ⊕ a
⊕-comm {m} {m>0} (mkResidueClass r₁ _) (mkResidueClass r₂ _) =
  cong (modulo {m} {m>0}) (+-comm r₁ r₂)

⊗-assoc : ∀ {m} {m>0 : m > 0} (a b c : ResidueClass m {m>0}) →
  (a ⊗ b) ⊗ c ≡ᵣ a ⊗ (b ⊗ c)
⊗-assoc {m} {m>0} (mkResidueClass r₁ r₁<m) (mkResidueClass r₂ _) (mkResidueClass r₃ r₃<m) = begin
  modulo {m} {m>0} (modulo {m} {m>0} (r₁ * r₂) * r₃)
    ≡⟨ cong (λ t → modulo {m} {m>0} (modulo {m} {m>0} (r₁ * r₂) * t))
            (sym (m<n⇒m%n≡m {{>-nonZero m>0}} r₃<m)) ⟩
  modulo {m} {m>0} (modulo {m} {m>0} (r₁ * r₂) * modulo {m} {m>0} r₃)
    ≡⟨ sym (%-distribˡ-* (r₁ * r₂) r₃ m {{>-nonZero m>0}}) ⟩
  modulo {m} {m>0} ((r₁ * r₂) * r₃)
    ≡⟨ cong (modulo {m} {m>0}) (*-assoc r₁ r₂ r₃) ⟩
  modulo {m} {m>0} (r₁ * (r₂ * r₃))
    ≡⟨ %-distribˡ-* r₁ (r₂ * r₃) m {{>-nonZero m>0}} ⟩
  modulo {m} {m>0} (modulo {m} {m>0} r₁ * modulo {m} {m>0} (r₂ * r₃))
    ≡⟨ cong (λ t → modulo {m} {m>0} (t * modulo {m} {m>0} (r₂ * r₃)))
            (m<n⇒m%n≡m {{>-nonZero m>0}} r₁<m) ⟩
  modulo {m} {m>0} (r₁ * modulo {m} {m>0} (r₂ * r₃))
  ∎

⊗-comm : ∀ {m} {m>0 : m > 0} (a b : ResidueClass m {m>0}) →
  a ⊗ b ≡ᵣ b ⊗ a
⊗-comm {m} {m>0} (mkResidueClass r₁ _) (mkResidueClass r₂ _) =
  cong (modulo {m} {m>0}) (*-comm r₁ r₂)

⊗-distribˡ-⊕ : ∀ {m} {m>0 : m > 0} (a b c : ResidueClass m {m>0}) →
  a ⊗ (b ⊕ c) ≡ᵣ (a ⊗ b) ⊕ (a ⊗ c)
⊗-distribˡ-⊕ {m} {m>0} (mkResidueClass r₁ r₁<m) (mkResidueClass r₂ _) (mkResidueClass r₃ _) = begin
  modulo {m} {m>0} (r₁ * modulo {m} {m>0} (r₂ + r₃))
    ≡⟨ cong (λ t → modulo {m} {m>0} (t * modulo {m} {m>0} (r₂ + r₃)))
            (sym (m<n⇒m%n≡m {{>-nonZero m>0}} r₁<m)) ⟩
  modulo {m} {m>0} (modulo {m} {m>0} r₁ * modulo {m} {m>0} (r₂ + r₃))
    ≡⟨ sym (%-distribˡ-* r₁ (r₂ + r₃) m {{>-nonZero m>0}}) ⟩
  modulo {m} {m>0} (r₁ * (r₂ + r₃))
    ≡⟨ cong (modulo {m} {m>0}) (*-distribˡ-+ r₁ r₂ r₃) ⟩
  modulo {m} {m>0} ((r₁ * r₂) + (r₁ * r₃))
    ≡⟨ %-distribˡ-+ (r₁ * r₂) (r₁ * r₃) m {{>-nonZero m>0}} ⟩
  modulo {m} {m>0} (modulo {m} {m>0} (r₁ * r₂) + modulo {m} {m>0} (r₁ * r₃))
  ∎

⊕-identityˡ : ∀ {m} {m>0 : m > 0} (a : ResidueClass m {m>0}) →
  0ᵣ ⊕ a ≡ᵣ a
⊕-identityˡ {m} {m>0} (mkResidueClass r r<m) = begin
  modulo {m} {m>0} (0 + r)
    ≡⟨ cong (modulo {m} {m>0}) (+-identityˡ r) ⟩
  modulo {m} {m>0} r
    ≡⟨ m<n⇒m%n≡m {{>-nonZero m>0}} r<m ⟩
  r
  ∎

⊕-identityʳ : ∀ {m} {m>0 : m > 0} (a : ResidueClass m {m>0}) →
  a ⊕ 0ᵣ ≡ᵣ a
⊕-identityʳ {m} {m>0} (mkResidueClass r r<m) = begin
  modulo {m} {m>0} (r + 0)
    ≡⟨ cong (modulo {m} {m>0}) (+-identityʳ r) ⟩
  modulo {m} {m>0} r
    ≡⟨ m<n⇒m%n≡m {{>-nonZero m>0}} r<m ⟩
  r
  ∎

⊗-identityˡ : ∀ {m} {m>0 : m > 0} (m≥2 : m ≥ 2) (a : ResidueClass m {m>0}) →
  (1ᵣ {m = m} {m>0 = m>0} m≥2) ⊗ a ≡ᵣ a
⊗-identityˡ {m} {m>0} _ (mkResidueClass r r<m) = begin
  modulo {m} {m>0} (1 * r)
    ≡⟨ cong (modulo {m} {m>0}) (*-identityˡ r) ⟩
  modulo {m} {m>0} r
    ≡⟨ m<n⇒m%n≡m {{>-nonZero m>0}} r<m ⟩
  r
  ∎

⊗-identityʳ : ∀ {m} {m>0 : m > 0} (m≥2 : m ≥ 2) (a : ResidueClass m {m>0}) →
  a ⊗ (1ᵣ {m = m} {m>0 = m>0} m≥2) ≡ᵣ a
⊗-identityʳ {m} {m>0} _ (mkResidueClass r r<m) = begin
  modulo {m} {m>0} (r * 1)
    ≡⟨ cong (modulo {m} {m>0}) (*-identityʳ r) ⟩
  modulo {m} {m>0} r
    ≡⟨ m<n⇒m%n≡m {{>-nonZero m>0}} r<m ⟩
  r
  ∎

⊕-inverseˡ : ∀ {m} {m>0 : m > 0} (a : ResidueClass m {m>0}) →
  (⊖ a) ⊕ a ≡ᵣ 0ᵣ
⊕-inverseˡ {m} {m>0} (mkResidueClass zero _) = begin
  modulo {m} {m>0} (modulo {m} {m>0} (m ∸ zero) + zero)
    ≡⟨ cong (modulo {m} {m>0}) (+-identityʳ (modulo {m} {m>0} (m ∸ zero))) ⟩
  modulo {m} {m>0} (modulo {m} {m>0} (m ∸ zero))
    ≡⟨ m%n%n≡m%n m m {{>-nonZero m>0}} ⟩
  modulo {m} {m>0} m
    ≡⟨ n%n≡0 m {{>-nonZero m>0}} ⟩
  0
  ∎
⊕-inverseˡ {m} {m>0} (mkResidueClass (suc r) valid-r) = begin
  modulo {m} {m>0} (modulo {m} {m>0} (m ∸ suc r) + suc r)
    ≡⟨ cong (λ t → modulo {m} {m>0} (modulo {m} {m>0} (m ∸ suc r) + t))
            (sym (m<n⇒m%n≡m {{>-nonZero m>0}} valid-r)) ⟩
  modulo {m} {m>0} (modulo {m} {m>0} (m ∸ suc r) + modulo {m} {m>0} (suc r))
    ≡⟨ sym (%-distribˡ-+ (m ∸ suc r) (suc r) m {{>-nonZero m>0}}) ⟩
  modulo {m} {m>0} ((m ∸ suc r) + suc r)
    ≡⟨ cong (modulo {m} {m>0}) (m∸n+n≡m (<⇒≤ valid-r)) ⟩
  modulo {m} {m>0} m
    ≡⟨ n%n≡0 m {{>-nonZero m>0}} ⟩
  0
  ∎

⊕-inverseʳ : ∀ {m} {m>0 : m > 0} (a : ResidueClass m {m>0}) →
  a ⊕ (⊖ a) ≡ᵣ 0ᵣ
⊕-inverseʳ a = trans (⊕-comm a (⊖ a)) (⊕-inverseˡ a)

residue-ring : ∀ (m : ℕ) {m>0 : m > 0} →
  (m>1 : m > 1) →
  IsCommutativeRing _≡ᵣ_ _⊕_ _⊗_ ⊖_
    (0ᵣ {m = m} {m>0 = m>0})
    (1ᵣ {m = m} {m>0 = m>0} (m>1⇒m≥2 m>1))
residue-ring m {m>0} m>1 = record
  { isRing = record
    { +-isAbelianGroup = record
      { isGroup = record
        { isMonoid = record
          { isSemigroup = record
            { isMagma = record
              { isEquivalence = ≡ᵣ-isEquivalence
              ; ∙-cong = λ {x} {y} {u} {v} →
                  ⊕-cong {a = x} {b = y} {c = u} {d = v}
              }
            ; assoc = ⊕-assoc
            }
          ; identity = ⊕-identityˡ , ⊕-identityʳ
          }
        ; inverse = ⊕-inverseˡ , ⊕-inverseʳ
        ; ⁻¹-cong = λ {a} {b} → ⊖-cong {a = a} {b = b}
        }
      ; comm = ⊕-comm
      }
    ; *-cong = λ {x} {y} {u} {v} →
        ⊗-cong {a = x} {b = y} {c = u} {d = v}
    ; *-assoc = ⊗-assoc
    ; *-identity = ⊗-identityˡ m≥2 , ⊗-identityʳ m≥2
    ; distrib = ⊗-distribˡ-⊕ , ⊗-distribʳ-⊕
    }
  ; *-comm = ⊗-comm
  }
  where
    m≥2 : m ≥ 2
    m≥2 = m>1⇒m≥2 m>1

    ≡ᵣ-isEquivalence : IsEquivalence (_≡ᵣ_ {m} {m>0})
    ≡ᵣ-isEquivalence = record
      { refl = refl
      ; sym = sym
      ; trans = trans
      }

    ⊕-cong : ∀ {a b c d : ResidueClass m {m>0}} →
      a ≡ᵣ b → c ≡ᵣ d → (a ⊕ c) ≡ᵣ (b ⊕ d)
    ⊕-cong {a = mkResidueClass r₁ _} {b = mkResidueClass r₂ _}
           {c = mkResidueClass r₃ _} {d = mkResidueClass r₄ _} eq₁ eq₂ =
      cong (modulo {m} {m>0}) (cong₂ _+_ eq₁ eq₂)

    ⊗-cong : ∀ {a b c d : ResidueClass m {m>0}} →
      a ≡ᵣ b → c ≡ᵣ d → (a ⊗ c) ≡ᵣ (b ⊗ d)
    ⊗-cong {a = mkResidueClass r₁ _} {b = mkResidueClass r₂ _}
           {c = mkResidueClass r₃ _} {d = mkResidueClass r₄ _} eq₁ eq₂ =
      cong (modulo {m} {m>0}) (cong₂ _*_ eq₁ eq₂)

    ⊖-cong : ∀ {a b : ResidueClass m {m>0}} → a ≡ᵣ b → (⊖ a) ≡ᵣ (⊖ b)
    ⊖-cong = cong (λ t → modulo {m} {m>0} (m ∸ t))

    ⊗-distribʳ-⊕ : ∀ (x y z : ResidueClass m {m>0}) →
      ((y ⊕ z) ⊗ x) ≡ᵣ ((y ⊗ x) ⊕ (z ⊗ x))
    ⊗-distribʳ-⊕ x y z =
      trans
        (⊗-comm (y ⊕ z) x)
        (trans
          (⊗-distribˡ-⊕ x y z)
          (⊕-cong {a = x ⊗ y} {b = y ⊗ x} {c = x ⊗ z} {d = z ⊗ x}
                  (⊗-comm x y) (⊗-comm x z)))

Coprime : ℕ → ℕ → Set
Coprime m n = gcd m n ≡ 1

IsUnit : ∀ {m} {m>0 : m > 0} → ResidueClass m {m>0} → Set
IsUnit {m} {m>0} r =
  ∃ λ (s : ResidueClass m {m>0}) →
    ∃ λ (m>1 : m > 1) →
      (r ⊗ s) ≡ᵣ (1ᵣ {m = m} {m>0 = m>0} (m>1⇒m≥2 m>1))

coprime-→-unit : ∀ {m} {m>0 : m > 0} (m>1 : m > 1) (r : ResidueClass m {m>0}) →
  Coprime ⟦ r ⟧ m → IsUnit r
coprime-→-unit {m} {m>0 = m>0} m>1 r coprime
  with coprime-Bézout {m = ⟦ r ⟧} {n = m} (gcd≡1⇒coprime {m = ⟦ r ⟧} {n = m} coprime)
... | Bézout.+- x y eq = s , m>1 , rs≡1
  where
    instance
      nz-m : NonZero m
      nz-m = >-nonZero m>0

    a : ℕ
    a = ⟦ r ⟧

    a<m : a < m
    a<m = valid r

    modm : ℕ → ℕ
    modm n = _%_ n m {{nz-m}}

    xm : ℕ
    xm = modm x

    s : ResidueClass m {m>0}
    s = [ xm ]mod (m%n<n x m {{nz-m}})

    rs≡1 : (r ⊗ s) ≡ᵣ (1ᵣ {m = m} {m>0 = m>0} (m>1⇒m≥2 m>1))
    rs≡1 = begin
      modm (a * xm)                    ≡⟨ cong modm (*-comm a xm) ⟩
      modm (xm * a)                    ≡⟨ cong (λ t → modm (xm * t)) (sym (m<n⇒m%n≡m {{nz-m}} a<m)) ⟩
      modm (xm * modm a)               ≡⟨ sym (%-distribˡ-* x a m {{nz-m}}) ⟩
      modm (x * a)                     ≡⟨ %-congˡ {o = m} {{nz-m}} (sym eq) ⟩
      modm (1 + y * m)                 ≡⟨ [m+kn]%n≡m%n 1 y m {{nz-m}} ⟩
      modm 1                           ≡⟨ m<n⇒m%n≡m {{nz-m}} m>1 ⟩
      1                                ∎
... | Bézout.-+ x y eq = s , m>1 , rs≡1
  where
    instance
      nz-m : NonZero m
      nz-m = >-nonZero m>0

    a : ℕ
    a = ⟦ r ⟧

    a<m : a < m
    a<m = valid r

    modm : ℕ → ℕ
    modm n = _%_ n m {{nz-m}}

    neg1 : ℕ
    neg1 = m ∸ 1

    s : ResidueClass m {m>0}
    s = [ modm (neg1 * x) ]mod (m%n<n (neg1 * x) m {{nz-m}})

    xa≡neg1 : modm (x * a) ≡ neg1
    xa≡neg1 =
      %-pred-≡0 {n = m} {{nz-m}}
        (trans (%-congˡ {o = m} {{nz-m}} eq)
               (m*n%n≡0 y m {{nz-m}}))

    rs≡1 : (r ⊗ s) ≡ᵣ (1ᵣ {m = m} {m>0 = m>0} (m>1⇒m≥2 m>1))
    rs≡1 = begin
      modm (a * modm (neg1 * x))              ≡⟨ cong modm (*-comm a (modm (neg1 * x))) ⟩
      modm (modm (neg1 * x) * a)              ≡⟨ cong (λ t → modm (modm (neg1 * x) * t)) (sym (m<n⇒m%n≡m {{nz-m}} a<m)) ⟩
      modm (modm (neg1 * x) * modm a)         ≡⟨ sym (%-distribˡ-* (neg1 * x) a m {{nz-m}}) ⟩
      modm ((neg1 * x) * a)                   ≡⟨ cong modm (*-assoc neg1 x a) ⟩
      modm (neg1 * (x * a))                   ≡⟨ %-distribˡ-* neg1 (x * a) m {{nz-m}} ⟩
      modm (modm neg1 * modm (x * a))         ≡⟨ cong (λ t → modm (t * modm (x * a))) (m<n⇒m%n≡m {{nz-m}} (pred<m m>1)) ⟩
      modm (neg1 * modm (x * a))              ≡⟨ cong (λ t → modm (neg1 * t)) xa≡neg1 ⟩
      modm (neg1 * neg1)                      ≡⟨ neg1-square-mod {{nz-m}} m>1 ⟩
      1                                       ∎

unit-→-coprime : ∀ {m} {m>0 : m > 0} (r : ResidueClass m {m>0}) →
  IsUnit r → Coprime ⟦ r ⟧ m
unit-→-coprime {m} {m>0} r (s , _ , rs≡1) = ∣1⇒≡1 g∣1
  where
    a = ⟦ r ⟧
    b = ⟦ s ⟧

    q : ℕ
    q = _/_ (a * b) m {{>-nonZero m>0}}

    g∣a : gcd a m ∣ a
    g∣a = GCD.gcd∣m (gcd-GCD a m)

    g∣m : gcd a m ∣ m
    g∣m = GCD.gcd∣n (gcd-GCD a m)

    g∣ab : gcd a m ∣ a * b
    g∣ab = ∣m⇒∣m*n b g∣a

    ab≡1+q*m : a * b ≡ 1 + q * m
    ab≡1+q*m =
      trans
        (m≡m%n+[m/n]*n (a * b) m {{>-nonZero m>0}})
        (cong (λ t → t + q * m) rs≡1)

    g∣1+q*m : gcd a m ∣ 1 + q * m
    g∣1+q*m = subst (λ n → gcd a m ∣ n) ab≡1+q*m g∣ab

    g∣q*m : gcd a m ∣ q * m
    g∣q*m = ∣n⇒∣m*n q g∣m

    g∣q*m+1 : gcd a m ∣ q * m + 1
    g∣q*m+1 = subst (λ n → gcd a m ∣ n) (+-comm 1 (q * m)) g∣1+q*m

    g∣1 : gcd a m ∣ 1
    g∣1 = ∣m+n∣m⇒∣n g∣q*m+1 g∣q*m

units-are-coprime : ∀ {m} {m>0 : m > 0} (m>1 : m > 1) (r : ResidueClass m {m>0}) →
  IsUnit r ↔ Coprime ⟦ r ⟧ m
units-are-coprime m>1 r = mk↔ (unit-→-coprime r) (coprime-→-unit m>1 r)
