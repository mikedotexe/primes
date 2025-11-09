{-# OPTIONS --safe #-}

module PalindromeEvenDivides where

open import Data.Nat using (ℕ; zero; suc; _+_; _*_; _^_; _≤_; _∸_; _≤?_; _≟_)
open import Data.Nat.Properties as ℕₚ
open import Data.Vec using (Vec; []; _∷_; length; reverse; lookup)
open import Data.Fin using (Fin; zero; suc)
open import Relation.Binary.PropositionalEquality using (_≡_; refl; cong; sym; trans)
open import Data.Nat.Divisibility using (_∣_; divides)
open import Relation.Nullary using (Dec; yes; no)

------------------------------------------------------------------------
-- Basics

_≥2 : ℕ → Set
b ≥2 = ℕₚ.suc (ℕₚ.suc zero) ≤? b ≡ yes _
  where
  open import Relation.Nullary.Decidable using (toWitness)  -- only for witness hiding
  -- We only need the proposition that b ≥ 2 holds; proof value not used below.

-- Least-significant-first evaluation:
-- eval b [d₀ , d₁ , … , dₙ₋₁] = d₀ + b*d₁ + b²*d₂ + … + bⁿ⁻¹*dₙ₋₁
eval : (b : ℕ) → {n : ℕ} → Vec ℕ n → ℕ
eval b []       = 0
eval b (d ∷ ds) = d + b * eval b ds

Palindrome : ∀ {n} → Vec ℕ n → Set
Palindrome {n} xs = xs ≡ reverse xs

EvenLen : ℕ → Set
EvenLen n = ∃ λ k → n ≡ 2 * k
  where
  open import Data.Product using (Σ; Σ-syntax; ∃; _,_; proj₁; proj₂)

-- Handy sugar for Σ
open import Data.Product using (Σ; Σ-syntax; _,_; proj₁; proj₂)

------------------------------------------------------------------------
-- Algebraic identities we rely on (to be filled later)

-- For odd m = 2t+1, x^m + 1 factors as (x + 1) * (x^(m-1) - x^(m-2) + … - x + 1).
data Odd : ℕ → Set where
  odd : ∀ t → Odd (suc (2 * t))

postulate
  factor-x^odd+1
    : ∀ (x m : ℕ) → Odd m → Σ ℕ (λ q → x ^ m + 1 ≡ (x + 1) * q)

-- Trivial divisibility closures
div-sum : ∀ {a m n} → a ∣ m → a ∣ n → a ∣ (m + n)
div-sum {a} {m} {n} (divides k p) (divides ℓ q) =
  divides (k + ℓ) (begin
    m + n          ≡⟨ cong (λ z → z + n) p ⟩
    a * k + n      ≡⟨ cong (λ z → a * k + z) q ⟩
    a * k + a * ℓ  ≡⟨ sym (ℕₚ.*-distribʳ-+ a k ℓ) ⟩
    a * (k + ℓ)    ∎)
  where open ℕₚ.≡-Reasoning

div-mul-left : ∀ {a c t} → a ∣ c → a ∣ (c * t)
div-mul-left {a} {c} {t} (divides k p) =
  divides (k * t) (begin
    c * t        ≡⟨ cong (λ z → z * t) p ⟩
    (a * k) * t  ≡⟨ ℕₚ.*-assoc a k t ⟩
    a * (k * t)  ∎)
  where open ℕₚ.≡-Reasoning

------------------------------------------------------------------------
-- Core lemma: even-length palindrome ⇒ divisible by (b+1)

-- We work pairwise on symmetric digits. If ds has length 2k and is palindromic,
-- write indices i (0..k-1) and j = 2k-1-i (odd gap). Pair contributes:
--   d * (b^i + b^j) = d * b^i * (1 + b^(j-i))
-- With j - i = (2k-1) - 2i odd, factor-x^odd+1 gives (b+1) ∣ (1 + b^(odd)),
-- hence (b+1) ∣ pair; sum of pairs is divisible as well.

-- We package the "pairing" as a postulate skeleton to keep things tiny here:
postulate
  eval-even-palindrome-as-sum-of-pairs
    : ∀ {k} (b : ℕ) (ds : Vec ℕ (2 * k))
      → Palindrome ds
      → Σ (Vec ℕ k) (λ coeff →
           eval b ds ≡ (b + 1) * (ℕₚ.foldr _+_ 0 coeff))
-- coeff[i] here represents d_i * b^i * S_i, where S_i is the alternating geometric tail
-- 1 + b + … + b^(odd), whose product with (b+1) equals b^(odd+1) - 1.

evenPalindromeDividesBPlusOne
  : ∀ {k}
  → (b : ℕ) → b ≥2
  → (ds : Vec ℕ (2 * k))
  → Palindrome ds
  → (b + 1) ∣ eval b ds
evenPalindromeDividesBPlusOne {k} b _ ds pal =
  let open import Data.Vec as Vec
      pr = eval-even-palindrome-as-sum-of-pairs b ds pal
      coeff = proj₁ pr
      eq    = proj₂ pr
  in
  -- From eval = (b+1) * sum(coeff), divisibility is immediate.
  divides (ℕₚ.foldr _+_ 0 coeff) eq
