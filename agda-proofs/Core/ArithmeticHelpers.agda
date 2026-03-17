{-# OPTIONS --without-K #-}

{-|
  Arithmetic helpers for divisibility-style proof shells.

  Strongest live signal:
  1. the reusable regrouping identities for bases 6, 10, and 30 are genuinely
     constructive and useful
  2. the file still serves as a staging area for shorter divisibility proofs
  3. the broader division-algorithm and example-template layer is left explicit
     rather than mixed with stale parse-era notation
-}

module Core.ArithmeticHelpers where

open import Data.Nat using (ℕ; _+_; _*_)
open import Data.Product using (Σ; _,_)
open import Data.Nat.Properties as Nat using
  ( +-identityʳ
  ; +-identityˡ
  ; *-assoc
  ; *-comm
  ; *-distribˡ-+
  ; *-identityʳ
  )
open import Relation.Binary.PropositionalEquality as Eq using (_≡_; refl; sym; cong)

open Eq.≡-Reasoning

------------------------------------------------------------------------
-- Factorization helpers
------------------------------------------------------------------------

record BaseFactors (base : ℕ) : Set where
  field
    factor1 : ℕ
    factor2 : ℕ
    factorization : base ≡ factor1 * factor2

factors-6 : BaseFactors 6
factors-6 = record
  { factor1 = 2
  ; factor2 = 3
  ; factorization = refl
  }

factors-10 : BaseFactors 10
factors-10 = record
  { factor1 = 2
  ; factor2 = 5
  ; factorization = refl
  }

factors-30 : BaseFactors 30
factors-30 = record
  { factor1 = 5
  ; factor2 = 6
  ; factorization = refl
  }

------------------------------------------------------------------------
-- Constructive regrouping lemmas
------------------------------------------------------------------------

ten_eq_two_times_five : 10 ≡ 2 * 5
ten_eq_two_times_five = refl

ten_times_q_eq_two_times_five_times_q : ∀ q → 10 * q ≡ 2 * (5 * q)
ten_times_q_eq_two_times_five_times_q q = begin
  10 * q       ≡⟨ cong (λ x → x * q) ten_eq_two_times_five ⟩
  (2 * 5) * q  ≡⟨ *-assoc 2 5 q ⟩
  2 * (5 * q)  ∎

ten_times_q_eq_five_times_two_times_q : ∀ q → 10 * q ≡ 5 * (2 * q)
ten_times_q_eq_five_times_two_times_q q = begin
  10 * q       ≡⟨ cong (λ x → x * q) (sym (*-comm 5 2)) ⟩
  (5 * 2) * q  ≡⟨ *-assoc 5 2 q ⟩
  5 * (2 * q)  ∎

six_eq_two_times_three : 6 ≡ 2 * 3
six_eq_two_times_three = refl

six_times_q_eq_two_times_three_times_q : ∀ q → 6 * q ≡ 2 * (3 * q)
six_times_q_eq_two_times_three_times_q q = begin
  6 * q       ≡⟨ cong (λ x → x * q) six_eq_two_times_three ⟩
  (2 * 3) * q ≡⟨ *-assoc 2 3 q ⟩
  2 * (3 * q) ∎

six_times_q_eq_three_times_two_times_q : ∀ q → 6 * q ≡ 3 * (2 * q)
six_times_q_eq_three_times_two_times_q q = begin
  6 * q       ≡⟨ cong (λ x → x * q) (sym (*-comm 3 2)) ⟩
  (3 * 2) * q ≡⟨ *-assoc 3 2 q ⟩
  3 * (2 * q) ∎

thirty_eq_five_times_six : 30 ≡ 5 * 6
thirty_eq_five_times_six = refl

thirty_times_q_eq_five_times_six_times_q : ∀ q → 30 * q ≡ 5 * (6 * q)
thirty_times_q_eq_five_times_six_times_q q = begin
  30 * q       ≡⟨ cong (λ x → x * q) thirty_eq_five_times_six ⟩
  (5 * 6) * q  ≡⟨ *-assoc 5 6 q ⟩
  5 * (6 * q)  ∎

scale_times_a_plus_scale_eq_scale_times_suc : ∀ d a → d * a + d ≡ d * (a + 1)
scale_times_a_plus_scale_eq_scale_times_suc d a = begin
  d * a + d    ≡⟨ cong (d * a +_) (sym (*-identityʳ d)) ⟩
  d * a + d * 1 ≡⟨ sym (*-distribˡ-+ d a 1) ⟩
  d * (a + 1)  ∎

scale_times_a_plus_scale_times_r_eq_scale_times_sum : ∀ d a r → d * a + d * r ≡ d * (a + r)
scale_times_a_plus_scale_times_r_eq_scale_times_sum d a r =
  sym (*-distribˡ-+ d a r)

times_one_right : ∀ d → d * 1 ≡ d
times_one_right d = *-identityʳ d

add_zero_right : ∀ b q → b * q + 0 ≡ b * q
add_zero_right b q = +-identityʳ (b * q)

add_zero_left : ∀ b q → 0 + b * q ≡ b * q
add_zero_left b q = +-identityˡ (b * q)

------------------------------------------------------------------------
-- Small constant witnesses
------------------------------------------------------------------------

two_times_two : 2 * 2 ≡ 4
two_times_two = refl

two_times_three : 2 * 3 ≡ 6
two_times_three = refl

five_times_two : 5 * 2 ≡ 10
five_times_two = refl

------------------------------------------------------------------------
-- Example shell layer
------------------------------------------------------------------------

_∣_ : ℕ → ℕ → Set
d ∣ n = Σ ℕ (λ k → n ≡ d * k)

record DivisibilityTemplate : Set where
  field
    base : ℕ
    divisor : ℕ
    remainder : ℕ

base10-ends-in-2-template : DivisibilityTemplate
base10-ends-in-2-template = record
  { base = 10
  ; divisor = 2
  ; remainder = 2
  }

base6-ends-in-3-template : DivisibilityTemplate
base6-ends-in-3-template = record
  { base = 6
  ; divisor = 3
  ; remainder = 3
  }

postulate
  divmod-2 : ∀ (n : ℕ) → Set
  divmod-3 : ∀ (n : ℕ) → Set
  divmod-5 : ∀ (n : ℕ) → Set
  divmod-6 : ∀ (n : ℕ) → Set
  divmod-10 : ∀ (n : ℕ) → Set
  divmod-30 : ∀ (n : ℕ) → Set
  last-digit-10 : ℕ → ℕ
  example-ends-in-2-div-2 : Set
  example-ends-in-3-div-3-base6 : Set
