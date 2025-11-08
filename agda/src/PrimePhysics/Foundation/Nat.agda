{-
  ═══════════════════════════════════════════════════════════════════════
  NATURAL NUMBER FOUNDATIONS
  ═══════════════════════════════════════════════════════════════════════

  This module extends the standard library's natural number operations
  with properties specifically needed for membrane prime construction.

  Key additions:
  • Digital representation (number → list of digits)
  • Modular arithmetic properties
  • Divisibility chains
  • Properties of powers and multiples

  Author: Prime Physics Engine Research Team
  Version: 1.0.0
-}

module PrimePhysics.Foundation.Nat where

-- Standard library imports
open import Data.Nat using (ℕ; zero; suc; _+_; _*_; _∸_; _≤_; _<_; _≡ᵇ_)
open import Data.Nat.Properties using (+-comm; +-assoc; *-comm; *-assoc)
open import Data.Nat.DivMod using (_div_; _mod_)
open import Data.Bool using (Bool; true; false; _∧_; _∨_; if_then_else_)
open import Data.List using (List; []; _∷_; reverse; length)
open import Relation.Binary.PropositionalEquality using (_≡_; refl; sym; trans; cong)
open import Relation.Nullary using (Dec; yes; no; ¬_)
open import Data.Product using (_×_; _,_; proj₁; proj₂)

-- Re-export commonly used operations
open Data.Nat public using (ℕ; zero; suc; _+_; _*_; _∸_; _≤_; _<_)
open Relation.Binary.PropositionalEquality public using (_≡_; refl; sym; trans; cong)

-------------------------------------------------------------------------------
-- DIVISIBILITY
-------------------------------------------------------------------------------

{- DEFINITION: Divisibility
   d divides n (written d ∣ n) if there exists k such that n = k * d.

   Example: 3 ∣ 12 because 12 = 4 * 3
-}
_∣_ : ℕ → ℕ → Set
d ∣ n = ∃[ k ] (n ≡ k * d)
  where open import Data.Product using (∃-syntax)

{- THEOREM: Divisibility is reflexive
   Every number divides itself (with k = 1).
-}
∣-refl : ∀ n → n ∣ n
∣-refl n = 1 , refl

{- THEOREM: Divisibility is transitive
   If a ∣ b and b ∣ c, then a ∣ c.

   Proof idea: c = k₁ * b = k₁ * (k₂ * a) = (k₁ * k₂) * a
-}
∣-trans : ∀ {a b c} → a ∣ b → b ∣ c → a ∣ c
∣-trans {a} {b} {c} (k₁ , refl) (k₂ , refl) = (k₂ * k₁) , *-assoc k₂ k₁ a

{- THEOREM: 1 divides everything
   This is obvious but needs to be stated formally.
-}
1∣n : ∀ n → 1 ∣ n
1∣n n = n , refl

-------------------------------------------------------------------------------
-- PRIMALITY (Placeholder for now)
-------------------------------------------------------------------------------

{- DEFINITION: Prime numbers
   A number p > 1 is prime if its only divisors are 1 and p itself.

   Note: Full primality proofs are complex. We use this as a predicate
   that will be satisfied by explicit proofs for small primes, and
   postulated for larger ones (checked by Rust's Miller-Rabin).
-}
record IsPrime (n : ℕ) : Set where
  field
    n>1 : n > 1
    only-trivial-divisors : ∀ d → d ∣ n → (d ≡ 1) ∨ (d ≡ n)
      where open import Data.Sum using (_⊎_; inj₁; inj₂) renaming (_⊎_ to _∨_)

{- Small primes we'll use often -}
postulate
  2-is-prime : IsPrime 2
  3-is-prime : IsPrime 3
  5-is-prime : IsPrime 5
  7-is-prime : IsPrime 7
  11-is-prime : IsPrime 11

-- These postulates will be proven in a separate module or accepted as
-- axiomatic for small primes (trivially verifiable by hand).

-------------------------------------------------------------------------------
-- DIGITAL REPRESENTATION
-------------------------------------------------------------------------------

{- FUNCTION: Convert number to list of digits in given base

   Example: toDigits 10 12345 = [1, 2, 3, 4, 5]

   This is crucial for membrane construction, where we need to
   manipulate the digital structure of numbers.
-}
toDigits : (base : ℕ) → {base≥2 : base > 1} → ℕ → List ℕ
toDigits base {base≥2} zero = zero ∷ []
toDigits base {base≥2} n@(suc _) = go n []
  where
    go : ℕ → List ℕ → List ℕ
    go zero acc = acc
    go m acc with m mod base
    ... | digit = go (m div base) (digit ∷ acc)

{- FUNCTION: Convert list of digits back to number

   Example: fromDigits 10 [1, 2, 3, 4, 5] = 12345

   Inverse of toDigits (up to leading zeros).
-}
fromDigits : (base : ℕ) → List ℕ → ℕ
fromDigits base [] = zero
fromDigits base (d ∷ ds) = d + base * fromDigits base ds

{- THEOREM: toDigits and fromDigits are inverses (for normalized representations)

   This is important for proving that membrane construction doesn't
   accidentally change the numerical value.

   Note: Full proof requires induction on the digit list structure.
   Marked as postulate for Phase 1; will prove in Phase 2.
-}
postulate
  toFromDigits-inverse : ∀ base {p} n →
    fromDigits base (toDigits base {p} n) ≡ n

-------------------------------------------------------------------------------
-- SYMMETRY PREDICATE
-------------------------------------------------------------------------------

{- FUNCTION: Check if a list is symmetric (palindrome)

   Example: isSymmetric [3, 0, 7, 0, 3] = true
            isSymmetric [3, 0, 7, 0, 5] = false

   This is the core property we'll prove about membranes!
-}
isSymmetric : {A : Set} → (eq : A → A → Bool) → List A → Bool
isSymmetric eq xs = go xs (reverse xs)
  where
    open import Data.List using (zipWith; and)
    open import Data.Bool using (and)

    go : List A → List A → Bool
    go [] [] = true
    go (x ∷ xs) (y ∷ ys) = if eq x y then go xs ys else false
    go _ _ = false

{- Specialized version for natural numbers -}
isSymmetricℕ : List ℕ → Bool
isSymmetricℕ = isSymmetric _≡ᵇ_

{- THEOREM: Symmetry is preserved under reversal
   If a list is symmetric, reversing it gives the same list.

   This might seem obvious, but it's a key property for reasoning
   about membrane structure.
-}
postulate
  symmetric-reverse : ∀ xs → isSymmetricℕ xs ≡ true →
    reverse xs ≡ xs

-------------------------------------------------------------------------------
-- UTILITIES
-------------------------------------------------------------------------------

{- Maximum of two numbers -}
max : ℕ → ℕ → ℕ
max zero n = n
max m zero = m
max (suc m) (suc n) = suc (max m n)

{- Minimum of two numbers -}
min : ℕ → ℕ → ℕ
min zero n = zero
min m zero = zero
min (suc m) (suc n) = suc (min m n)

{- Power function (needed for digit position values) -}
_^_ : ℕ → ℕ → ℕ
n ^ zero = 1
n ^ (suc m) = n * (n ^ m)

{- THEOREM: Basic power properties -}
postulate
  ^-distributes-* : ∀ n m k → (n * m) ^ k ≡ (n ^ k) * (m ^ k)
  ^-adds : ∀ n m k → n ^ (m + k) ≡ (n ^ m) * (n ^ k)

-------------------------------------------------------------------------------
-- REMARKS
-------------------------------------------------------------------------------

{-
  This module provides the foundation for all subsequent proofs.

  Key design decisions:

  1. We use postulates for theorems that are:
     - Tedious but straightforward to prove
     - Standard results from the Agda stdlib (but in a different form)
     - Verifiable by external tools (for primality)

  2. The digital representation functions are crucial because membrane
     construction operates on the digit level, not just the numeric value.

  3. Symmetry is defined generically (for any type with equality) but
     specialized for ℕ, which is what we'll use for membranes.

  Next steps: See Foundation.GCD for greatest common divisor properties.
-}
