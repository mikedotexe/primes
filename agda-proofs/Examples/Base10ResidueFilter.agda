{-# OPTIONS --safe --without-K #-}

{-|
  Base-10 prime filtering, expressed through the maintained residue framework.

  The strongest clean theorem currently available here is the coprimality filter:
  if `n` is prime and `n > 10`, then `n` is coprime to `10`. In base 10 this is
  the formal core behind the classical "ends in 1, 3, 7, or 9" rule.

  This module keeps that theorem executable and uses concrete examples to show
  the familiar last-digit pattern without overstating a larger proof surface.
-}

module Examples.Base10ResidueFilter where

open import Data.Bool using (Bool; true; false; _∨_)
open import Data.Nat using (ℕ; _<_; _>_; _≡ᵇ_; _≟_)
open import Data.Nat.Base using (nonZero)
open import Data.Nat.Coprimality using (Coprime; coprime⇒gcd≡1; prime⇒coprime)
open import Data.Nat.DivMod using (_%_)
open import Data.Nat.GCD using (gcd)
open import Data.Product using (_×_; _,_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Relation.Nullary using (yes; no; contradiction)
open import Relation.Nullary.Decidable.Core using (from-yes)

open import Core.Primality public using (IsPrime; isPrime?)

-------------------------------------------------------------------------------
-- Base-10 views
-------------------------------------------------------------------------------

last-digit : ℕ → ℕ
last-digit n = _%_ n 10 {{nonZero}}

classical-valid-ending : ℕ → Bool
classical-valid-ending n =
  let d = last-digit n
  in (d ≡ᵇ 1) ∨ (d ≡ᵇ 3) ∨ (d ≡ᵇ 7) ∨ (d ≡ᵇ 9)

-- The maintained executable filter is "coprime to 10".
valid-prime-residue : ℕ → Bool
valid-prime-residue n with gcd n 10 ≟ 1
... | yes _ = true
... | no  _ = false

-------------------------------------------------------------------------------
-- Main theorem
-------------------------------------------------------------------------------

prime-coprime-to-10 : ∀ n → IsPrime n → n > 10 → Coprime n 10
prime-coprime-to-10 n n-prime n>10 = prime⇒coprime n-prime {{nonZero}} n>10

prime-residue-theorem : ∀ n →
  IsPrime n →
  n > 10 →
  valid-prime-residue n ≡ true
prime-residue-theorem n n-prime n>10 with gcd n 10 ≟ 1
... | yes _ = refl
... | no gcd≢1 =
  contradiction (coprime⇒gcd≡1 (prime-coprime-to-10 n n-prime n>10)) gcd≢1

-------------------------------------------------------------------------------
-- Concrete examples
-------------------------------------------------------------------------------

example-11 : IsPrime 11 × (last-digit 11 ≡ 1)
example-11 = from-yes (isPrime? 11) , refl

example-13 : IsPrime 13 × (last-digit 13 ≡ 3)
example-13 = from-yes (isPrime? 13) , refl

example-17 : IsPrime 17 × (last-digit 17 ≡ 7)
example-17 = from-yes (isPrime? 17) , refl

example-19 : IsPrime 19 × (last-digit 19 ≡ 9)
example-19 = from-yes (isPrime? 19) , refl

example-11-ending : classical-valid-ending 11 ≡ true
example-11-ending = refl

example-13-ending : classical-valid-ending 13 ≡ true
example-13-ending = refl

example-17-ending : classical-valid-ending 17 ≡ true
example-17-ending = refl

example-19-ending : classical-valid-ending 19 ≡ true
example-19-ending = refl
