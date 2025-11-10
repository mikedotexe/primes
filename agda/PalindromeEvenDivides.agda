{-# OPTIONS --safe #-}

------------------------------------------------------------------------
-- Palindrome Divisibility Property
--
-- ⚠️  SCOPE: This module formalizes properties of TRUE PALINDROMES
--            (digit-value mirroring), which only applies to:
--
--     • Optional --mirror mode in density-explorer
--     • Theoretical exploration of (b+1) divisibility wall
--
-- ⚠️  NOT APPLICABLE to the default spacing-symmetric construction,
--     where open slots are sampled independently.
--
-- For the default construction, see SpacingResidueModel.agda
------------------------------------------------------------------------

module PalindromeEvenDivides where

open import Data.Nat using (ℕ; zero; suc; _+_; _*_; _^_; _≟_; _≤_; _<_; _∸_)
open import Data.Nat.Properties as ℕₚ
open import Data.Vec using (Vec; []; _∷_; length; reverse)
open import Data.Fin using (Fin)
open import Relation.Binary.PropositionalEquality using (_≡_; refl; cong; sym; trans)
open import Data.Nat.Divisibility using (_∣_; divides)
open import Data.Product using (Σ; Σ-syntax; ∃; _,_; proj₁; proj₂)
open import Relation.Nullary using (Dec; yes; no)

------------------------------------------------------------------------
-- Basic predicates

Even : ℕ → Set
Even n = ∃ λ k → n ≡ 2 * k

Palindrome : ∀ {n} → Vec ℕ n → Set
Palindrome xs = xs ≡ reverse xs

_≥₂ : ℕ → Set
n ≥₂ = 2 ≤ n

------------------------------------------------------------------------
-- Evaluation (MSB-first): eval b [d₀ … dₙ₋₁] = d₀*b^(n-1) + … + dₙ₋₁

eval : (b : ℕ) → ∀ {n} → Vec ℕ n → ℕ
eval b {zero}  []       = 0
eval b {suc n} (d ∷ ds) = d * b ^ n + eval b ds

------------------------------------------------------------------------
-- Algebraic skeletons to keep the file tiny; replace with proper proofs later.

data Odd : ℕ → Set where
  odd : ∀ t → Odd (suc (2 * t))

postulate
  -- Factorization: for odd m, x^m + 1 = (x + 1) * Q
  factor-x^odd+1
    : ∀ (x m : ℕ) → Odd m → Σ ℕ (λ q → x ^ m + 1 ≡ (x + 1) * q)

  -- Pairwise decomposition of an even-length palindrome into (b+1)·S
  eval-even-palindrome-as-(b+1)*S
    : ∀ {k} (b : ℕ) (ds : Vec ℕ (2 * k))
      → Palindrome ds
      → Σ ℕ (λ S → eval b ds ≡ (b + 1) * S)

------------------------------------------------------------------------
-- Main lemma: even-length palindrome ⇒ (b+1) ∣ value (for any base b ≥ 2)

evenPalindromeDividesBPlusOne
  : ∀ {k} (b : ℕ) → b ≥₂
  → (ds : Vec ℕ (2 * k))
  → Palindrome ds
  → (b + 1) ∣ eval b ds
evenPalindromeDividesBPlusOne b _ ds pal =
  let pair = eval-even-palindrome-as-(b+1)*S b ds pal
  in divides (proj₁ pair) (proj₂ pair)

------------------------------------------------------------------------
-- Explanation of the pairing argument (to be formalized):
--
-- For palindrome [d₀ d₁ … d_{k-1} d_{k-1} … d₁ d₀] of length 2k:
--
--   eval b ds = Σᵢ dᵢ·b^(2k-1-i) + Σⱼ dⱼ·b^j    (i,j symmetric pairs)
--             = Σᵢ dᵢ·(b^(2k-1-i) + b^i)         (d_i = d_{mirror(i)} by palindrome)
--             = Σᵢ dᵢ·b^i·(b^(2k-1-2i) + 1)
--
-- Since (2k-1-2i) is odd for all i < k, we have:
--   b^(odd) + 1 = (b + 1)·Q   (by factor-x^odd+1)
--
-- Therefore:
--   eval b ds = Σᵢ dᵢ·b^i·(b+1)·Qᵢ = (b + 1)·S
--
-- where S = Σᵢ dᵢ·b^i·Qᵢ
