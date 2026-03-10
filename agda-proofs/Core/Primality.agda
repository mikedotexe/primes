{-# OPTIONS --without-K #-}

-- Core Primality module
-- This module provides primality predicates and computational checks
-- using the standard library where possible

module Core.Primality where

open import Data.Nat using (ℕ; zero; suc; _≡ᵇ_)
open import Data.Nat.Primality using (Prime)
open import Data.Nat.Primality as StdPrimality
open import Data.Bool using (Bool; true; false)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Relation.Nullary using (Dec; yes; no)
open import Relation.Nullary.Decidable.Core using (from-yes)

-- Use stdlib's Prime as our canonical primality predicate
IsPrime : ℕ → Set
IsPrime = Prime

-- Decidable primality check
isPrime? : (n : ℕ) → Dec (IsPrime n)
isPrime? = StdPrimality.prime?

-- Boolean version for compatibility with existing code
primeBool : ℕ → Bool
primeBool n with isPrime? n
... | yes _ = true
... | no  _ = false

-- Bridge lemmas connecting Bool version to proof version
primeBool-sound : ∀ {n} → primeBool n ≡ true → IsPrime n
primeBool-sound {n} eq with isPrime? n
... | yes p = p
... | no ¬p = ⊥-elim (false≢true eq)
  where
    open import Data.Empty using (⊥; ⊥-elim)
    false≢true : false ≡ true → ⊥
    false≢true ()

primeBool-complete : ∀ {n} → IsPrime n → primeBool n ≡ true
primeBool-complete {n} p with isPrime? n
... | yes _ = refl
... | no ¬p = ⊥-elim (¬p p)
  where
    open import Data.Empty using (⊥-elim)

-- Small prime witnesses from stdlib
2-is-prime : IsPrime 2
2-is-prime = StdPrimality.prime[2]

-- For 3, 5, 7 we use the from-yes pattern from stdlib
3-is-prime : IsPrime 3
3-is-prime = from-yes (isPrime? 3)

5-is-prime : IsPrime 5
5-is-prime = from-yes (isPrime? 5)

7-is-prime : IsPrime 7
7-is-prime = from-yes (isPrime? 7)

-- Note: Use primeBool instead of prime? to avoid conflict with stdlib