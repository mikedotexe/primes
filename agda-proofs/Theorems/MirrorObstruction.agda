{-# OPTIONS --safe #-}

module Theorems.MirrorObstruction where

open import Agda.Builtin.Nat       using (Nat; zero; suc; _+_; mod-helper)
open import Agda.Builtin.Bool      using (Bool; true; false)
open import Agda.Builtin.Equality  using (_≡_; refl)
open import Agda.Builtin.List      using (List; []; _∷_)
open import Data.Bool.Base         using (if_then_else_)
open import Data.List.Base         using (reverse; length)
open import Data.Nat               using (_≟_)
open import Relation.Nullary       using (yes; no)
open import Core.ResidueFold       using (Slot; Pattern; FixedZero; Open
                                         ; Counts; countsDPConv; delta0; eqCounts)

-- Use built-in modulo
_mod_ : Nat → Nat → Nat
n mod m = mod-helper 0 m n m

------------------------------------------------------------------------
-- List equality helper

eqNat : Nat → Nat → Bool
eqNat m n with m ≟ n
... | yes _ = true
... | no  _ = false

eqList : {A : Set} → (A → A → Bool) → List A → List A → Bool
eqList eq []       []       = true
eqList eq []       (_ ∷ _)  = false
eqList eq (_ ∷ _)  []       = false
eqList eq (x ∷ xs) (y ∷ ys) = if eq x y then eqList eq xs ys else false

eqSlot : Slot → Slot → Bool
eqSlot FixedZero FixedZero = true
eqSlot (Open ds₁) (Open ds₂) = eqList eqNat ds₁ ds₂
eqSlot _ _ = false

------------------------------------------------------------------------
-- Even length and mirror checks

evenLen : {A : Set} → List A → Bool
evenLen xs with (length xs) mod 2
... | 0 = true
... | _ = false

mirrorSlots : Pattern → Bool
mirrorSlots ps = eqList eqSlot ps (reverse ps)

------------------------------------------------------------------------
-- Executable DP invariant: even-length mirror ⇒ DP collapses to δ₀ at m=b+1

obstructionAt_b+1? : Nat → Pattern → Bool
obstructionAt_b+1? base pat =
  if evenLen pat then
    if mirrorSlots pat then
      eqCounts (countsDPConv base (base + 1) pat) (delta0 (base + 1))
    else true
  else true

------------------------------------------------------------------------
-- Smoke test

TestMirror₁ : Bool
TestMirror₁ =
  let base = 10
      pat  =
        Open (1 ∷ 3 ∷ 7 ∷ 9 ∷ []) ∷
        FixedZero ∷
        FixedZero ∷
        Open (1 ∷ 3 ∷ 7 ∷ 9 ∷ []) ∷
        []
  in obstructionAt_b+1? base pat
