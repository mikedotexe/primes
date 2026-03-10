{-# OPTIONS --safe #-}

module Specs.PalindromeEvenDivides where

open import Agda.Builtin.Nat       using (Nat; zero; suc; _+_; _*_; mod-helper)
open import Agda.Builtin.Equality  using (_≡_; refl)
open import Agda.Builtin.Bool      using (Bool; true; false)
open import Agda.Builtin.List      using (List; []; _∷_)
open import Data.Bool.Base         using (if_then_else_)
open import Data.List.Base         using (map; reverse; length; foldr)
open import Data.Nat               using (_<?_; _≟_)
open import Relation.Nullary       using (Dec; yes; no)

-- Use built-in modulo
_mod_ : Nat → Nat → Nat
n mod m = mod-helper 0 m n m

------------------------------------------------------------------------
-- Helpers

eqNat : Nat → Nat → Bool
eqNat m n with m ≟ n
... | yes _ = true
... | no  _ = false

eqList : List Nat → List Nat → Bool
eqList []       []       = true
eqList []       (_ ∷ _)  = false
eqList (_ ∷ _)  []       = false
eqList (x ∷ xs) (y ∷ ys) with eqNat x y
... | true  = eqList xs ys
... | false = false

isPalindrome : List Nat → Bool
isPalindrome xs = eqList xs (reverse xs)

evenLength : List Nat → Bool
evenLength xs with (length xs) mod 2
... | 0 = true
... | _ = false

-- Horner evaluation mod m (mirrors SpacingResidueModel.evalMod)
evalMod : Nat → Nat → List Nat → Nat
evalMod base m = foldr (λ d acc → (d + (base * acc) mod m) mod m) 0

------------------------------------------------------------------------
-- Computable check: even-palindrome ⇒ divisible by (b+1)

divisibleBy : Nat → Nat → Bool
divisibleBy m n with (n mod m) ≟ 0
... | yes _ = true
... | no  _ = false

palEvenDividesCheck : (base : Nat) → (digits : List Nat) → Bool
palEvenDividesCheck base xs =
  let m = base + 1 in
  if (evenLength xs) then
    if (isPalindrome xs) then
      divisibleBy m (evalMod base m xs)
    else true
  else true

-- Concrete sanity for base 10 and 4-digit palindromes
Test₁ : Bool
Test₁ =
  let xs = 2 ∷ 5 ∷ 5 ∷ 2 ∷ [] in
  palEvenDividesCheck 10 xs

Test₂ : Bool
Test₂ =
  let xs = 9 ∷ 0 ∷ 0 ∷ 9 ∷ [] in
  palEvenDividesCheck 10 xs
