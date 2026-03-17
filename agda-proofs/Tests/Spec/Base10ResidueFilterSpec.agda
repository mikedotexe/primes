{-# OPTIONS --safe --without-K #-}

{-|
  Test specification: base-10 prime filter.

  This module applies the maintained base-10 theorem to concrete primes and
  checks that each invocation normalizes to `refl`.

  Pattern:
  - derive prime witnesses from `isPrime?`
  - feed them into `prime-residue-theorem`
  - confirm the executable filter returns `true`
-}

module Tests.Spec.Base10ResidueFilterSpec where

open import Data.Nat using (ℕ; zero; suc; _<_; _<ᵇ_)
open import Data.Nat.Properties using (<ᵇ⇒<)
open import Relation.Binary.PropositionalEquality using (_≡_)
open import Data.Bool using (Bool; true; false)
open import Data.Unit.Base using (tt)
open import Relation.Nullary.Decidable.Core using (from-yes)

-- Import the module under test
open import Examples.Base10ResidueFilter
  renaming (IsPrime to Prime)

-------------------------------------------------------------------------------
-- Primality Facts
-------------------------------------------------------------------------------

prime-11 : Prime 11
prime-11 = from-yes (isPrime? 11)

prime-13 : Prime 13
prime-13 = from-yes (isPrime? 13)

prime-17 : Prime 17
prime-17 = from-yes (isPrime? 17)

prime-19 : Prime 19
prime-19 = from-yes (isPrime? 19)

prime-23 : Prime 23
prime-23 = from-yes (isPrime? 23)

prime-29 : Prime 29
prime-29 = from-yes (isPrime? 29)

prime-31 : Prime 31
prime-31 = from-yes (isPrime? 31)

prime-37 : Prime 37
prime-37 = from-yes (isPrime? 37)

-------------------------------------------------------------------------------
-- Inequality Proofs
-------------------------------------------------------------------------------

-- Proofs that 10 < n for each test case
-- These are explicit successor chains: 0 < 1 < 2 < ... < 10 < n

10<11 : 10 < 11
10<11 = <ᵇ⇒< 10 11 tt

10<13 : 10 < 13
10<13 = <ᵇ⇒< 10 13 tt

10<17 : 10 < 17
10<17 = <ᵇ⇒< 10 17 tt

10<19 : 10 < 19
10<19 = <ᵇ⇒< 10 19 tt

10<23 : 10 < 23
10<23 = <ᵇ⇒< 10 23 tt

10<29 : 10 < 29
10<29 = <ᵇ⇒< 10 29 tt

10<31 : 10 < 31
10<31 = <ᵇ⇒< 10 31 tt

10<37 : 10 < 37
10<37 = <ᵇ⇒< 10 37 tt

-------------------------------------------------------------------------------
-- Test Cases
-------------------------------------------------------------------------------

-- Each test applies prime-residue-theorem to a specific prime
-- When the theorem is complete, these will normalize to refl

-- Test: 11 is prime and 11 > 10, so valid-prime-residue 11 must be true
test-11 : valid-prime-residue 11 ≡ true
test-11 = prime-residue-theorem 11 prime-11 10<11

-- Test: 13 is prime and 13 > 10, so valid-prime-residue 13 must be true
test-13 : valid-prime-residue 13 ≡ true
test-13 = prime-residue-theorem 13 prime-13 10<13

-- Test: 17 is prime and 17 > 10, so valid-prime-residue 17 must be true
test-17 : valid-prime-residue 17 ≡ true
test-17 = prime-residue-theorem 17 prime-17 10<17

-- Test: 19 is prime and 19 > 10, so valid-prime-residue 19 must be true
test-19 : valid-prime-residue 19 ≡ true
test-19 = prime-residue-theorem 19 prime-19 10<19

-- Test: 23 is prime and 23 > 10, so valid-prime-residue 23 must be true
test-23 : valid-prime-residue 23 ≡ true
test-23 = prime-residue-theorem 23 prime-23 10<23

-- Test: 29 is prime and 29 > 10, so valid-prime-residue 29 must be true
test-29 : valid-prime-residue 29 ≡ true
test-29 = prime-residue-theorem 29 prime-29 10<29

-- Test: 31 is prime and 31 > 10, so valid-prime-residue 31 must be true
test-31 : valid-prime-residue 31 ≡ true
test-31 = prime-residue-theorem 31 prime-31 10<31

-- Test: 37 is prime and 37 > 10, so valid-prime-residue 37 must be true
test-37 : valid-prime-residue 37 ≡ true
test-37 = prime-residue-theorem 37 prime-37 10<37

-------------------------------------------------------------------------------
-- Verification Notes
-------------------------------------------------------------------------------

{-
  WHAT THESE TESTS VALIDATE:

  1. The current theorem path is live: concrete prime witnesses from `isPrime?`
     flow through `prime-residue-theorem` and normalize successfully.

  2. The executable filter matches the expected base-10 examples:
     11, 13, 17, 19, 23, 29, 31, and 37 all pass.

  3. This stays a useful regression surface even while the stronger explicit
     equivalence to the digit set {1,3,7,9} remains documented mainly through
     the example module rather than a dedicated theorem.
-}

-- End of Base10ResidueFilterSpec
