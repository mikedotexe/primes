{-# OPTIONS --safe #-}

module Tests.DevProofs where

open import Agda.Builtin.Nat       using (Nat; _+_)
open import Agda.Builtin.Bool      using (Bool; true; false)
open import Agda.Builtin.List      using (List; []; _∷_)
open import Data.Bool.Base         using (if_then_else_)
open import Data.List.Base         using (_++_)
open import Core.ResidueFold       using (Slot; Pattern; FixedZero; Open
                                         ; Counts; delta0; convFold; eqCounts
                                         ; countsDPConv; countsDP)
open import Theorems.MirrorObstruction using (TestMirror₁)
open import Core.CRTVector         using (TestCRT₁; TestCRT₂)

------------------------------------------------------------------------
-- Identity / associativity on concrete residues

idL : Bool
idL =
  let m    = 11
      acc  = delta0 m
  in eqCounts (convFold m acc []) acc

assoc₁ : Bool
assoc₁ =
  let m   = 11
      acc = delta0 m
      xs  = 1 ∷ 4 ∷ 2 ∷ []
      ys  = 0 ∷ 3 ∷ []
  in eqCounts (convFold m (convFold m acc xs) ys)
              (convFold m acc (xs ++ ys))

------------------------------------------------------------------------
-- DP equivalence demonstrations

eqDP₁ : Bool
eqDP₁ =
  let base = 10
      m    = 11
      pat  = FixedZero ∷ Open (1 ∷ 3 ∷ 7 ∷ 9 ∷ []) ∷ FixedZero ∷ []
  in eqCounts (countsDPConv base m pat) (countsDP base m pat)

eqDP₂ : Bool
eqDP₂ =
  let base = 10
      m    = 7
      pat  = Open (0 ∷ 1 ∷ 2 ∷ []) ∷ FixedZero ∷ Open (3 ∷ 4 ∷ []) ∷ []
  in eqCounts (countsDPConv base m pat) (countsDP base m pat)

------------------------------------------------------------------------
-- Mirror obstruction

mirrorOK : Bool
mirrorOK = TestMirror₁

------------------------------------------------------------------------
-- CRT/LCM pushforward verification

crt₁ : Bool
crt₁ = TestCRT₁

crt₂ : Bool
crt₂ = TestCRT₂

------------------------------------------------------------------------
-- Aggregate test suite

All : Bool
All = if idL
      then if assoc₁
           then if eqDP₁
                then if eqDP₂
                     then if mirrorOK
                          then if crt₁
                               then crt₂
                               else false
                          else false
                     else false
                else false
           else false
      else false
