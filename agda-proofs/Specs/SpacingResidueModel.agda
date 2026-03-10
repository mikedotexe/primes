{-# OPTIONS --safe #-}

module Specs.SpacingResidueModel where

open import Agda.Builtin.Nat       using (Nat; zero; suc; _+_; _*_; mod-helper)
open import Agda.Builtin.Equality  using (_≡_; refl)
open import Agda.Builtin.Bool      using (Bool; true; false)
open import Agda.Builtin.List      using (List; []; _∷_)
open import Agda.Builtin.Maybe     using (Maybe; nothing; just)
open import Data.Bool.Base         using (if_then_else_)
open import Data.List.Base         using (map; foldr; concatMap; length; filter)
open import Data.Nat               using (_^_; _≤?_; _≤_; _<?_; _≟_)
open import Data.Product           using (_×_; _,_)
open import Relation.Nullary       using (Dec; yes; no)

-- Use built-in modulo
_mod_ : Nat → Nat → Nat
n mod m = mod-helper 0 m n m

------------------------------------------------------------------------
-- Slots and Patterns

data Slot : Set where
  FixedZero : Slot           -- digit fixed to 0
  Open      : List Nat → Slot  -- allowed digits for this position

Pattern : Set
Pattern = List Slot          -- left→right, most-significant first

------------------------------------------------------------------------
-- Base evaluation and weights

pow : Nat → Nat → Nat
pow b n = b ^ n

-- Horner evaluation
evalBase : Nat → List Nat → Nat
evalBase b = foldr (λ d acc → d + b * acc) 0

-- weights base m n = [ b^(n-1) , … , b^0 ] reduced mod m, in O(n)
weights : Nat → Nat → Nat → List Nat
weights base m n = go n 1 []
  where
    go : Nat → Nat → List Nat → List Nat
    go zero    cur acc = acc
    go (suc k) cur acc =
      let cur' = (cur * (base mod m)) mod m in
      go k cur' ((cur mod m) ∷ acc)

------------------------------------------------------------------------
-- Residue DP (counts as association list over 0..m-1)

Counts : Set
Counts = List (Nat × Nat)

upto : Nat → List Nat
upto zero    = []
upto (suc n) = 0 ∷ map suc (upto n)

zeroCounts : Nat → Counts
zeroCounts m = map (λ i → (i , 0)) (upto m)

bump : Nat → Nat → Counts → Counts
bump r k []               = []
bump r k ((i , c) ∷ xs) with i ≟ r
... | yes refl            = (i , c + k) ∷ xs
... | no  _               = (i , c)     ∷ bump r k xs

shiftAdd : Nat → Nat → Counts → Counts
shiftAdd m δ []               = []
shiftAdd m δ ((r , c) ∷ xs)   = bump ((r + δ) mod m) c (shiftAdd m δ xs)

plusCounts : Counts → Counts → Counts
plusCounts [] ys = ys
plusCounts xs [] = xs
plusCounts ((i , a) ∷ xs) ((j , b) ∷ ys) with i ≟ j
... | yes refl = (i , a + b) ∷ plusCounts xs ys
... | no  _    = (i , a + b) ∷ plusCounts xs ys  -- structurally aligned by construction

-- One open position with weight w and allowed digits ds
stepOpen : Nat → Nat → List Nat → Counts → Counts
stepOpen m w []       acc = acc
stepOpen m w (d ∷ ds) acc = stepOpen m w ds (plusCounts acc (shiftAdd m ((d * w) mod m) acc))

-- Core DP: build weights once; traverse left→right
countsDP : Nat → Nat → Pattern → Counts
countsDP base m pat =
  go pat (weights base m (length pat)) (zeroCounts m)
  where
    filterLt : List Nat → Nat → List Nat
    filterLt [] _ = []
    filterLt (d ∷ ds) b with d <? b
    ... | yes _ = d ∷ filterLt ds b
    ... | no  _ = filterLt ds b

    go : List Slot → List Nat → Counts → Counts
    go []              _        acc = acc
    go (FixedZero ∷ s) (_ ∷ ws) acc = go s ws acc
    go (Open ds   ∷ s) (w ∷ ws) acc =
      let ds' = filterLt ds base in
      go s ws (stepOpen m w ds' acc)
    go _               []       acc = acc

------------------------------------------------------------------------
-- Enumeration baseline (small specs) and eval mod m

filterLt : List Nat → Nat → List Nat
filterLt [] _ = []
filterLt (d ∷ ds) b with d <? b
... | yes _ = d ∷ filterLt ds b
... | no  _ = filterLt ds b

expandWithBase : Nat → Pattern → List (List Nat)
expandWithBase base []                 = ([] ∷ [])
expandWithBase base (FixedZero ∷ rest) = map (λ tail → 0 ∷ tail) (expandWithBase base rest)
expandWithBase base (Open ds   ∷ rest) =
  let ds' = filterLt ds base in
  concatMap (λ d → map (λ tail → d ∷ tail) (expandWithBase base rest)) ds'

evalMod : Nat → Nat → List Nat → Nat
evalMod base m = foldr (λ d acc → (d + (base * acc) mod m) mod m) 0

countsEnum : Nat → Nat → Pattern → Counts
countsEnum base m pat = tally (expandWithBase base pat) (zeroCounts m)
  where
    tally : List (List Nat) → Counts → Counts
    tally []       c = c
    tally (v ∷ vs) c = tally vs (bump (evalMod base m v) 1 c)

------------------------------------------------------------------------
-- LCM-lift: DP at L, derive counts mod p via class summation

sumEvery : Nat → Counts → Nat
sumEvery p []               = 0
sumEvery p ((i , c) ∷ xs) with (i mod p) ≟ 0
... | yes _                 = c + sumEvery p xs
... | no  _                 = sumEvery p xs

countZeroViaL : Nat → Nat → Pattern → Nat → Nat
countZeroViaL base L pat p = sumEvery p (countsDP base L pat)

------------------------------------------------------------------------
-- Helpers and tests

eqCounts : Counts → Counts → Bool
eqCounts [] [] = true
eqCounts [] (_ ∷ _) = false
eqCounts (_ ∷ _) [] = false
eqCounts ((i , a) ∷ xs) ((j , b) ∷ ys) =
  if check-eq i j then (if check-eq a b then eqCounts xs ys else false) else false
  where
    check-eq : Nat → Nat → Bool
    check-eq m n with m ≟ n
    ... | yes _ = true
    ... | no  _ = false

count0 : Counts → Nat
count0 []               = 0
count0 ((i , c) ∷ xs) with i ≟ 0
... | yes _             = c
... | no  _             = count0 xs

-- DP vs enumeration (two independent specs)
Test₁ : Bool
Test₁ =
  let pat = Open (0 ∷ 1 ∷ 2 ∷ []) ∷ FixedZero ∷ Open (3 ∷ 4 ∷ []) ∷ [] in
  eqCounts (countsDP 10 7 pat) (countsEnum 10 7 pat)

Test₂ : Bool
Test₂ =
  let pat = FixedZero ∷ Open (1 ∷ 3 ∷ 7 ∷ 9 ∷ []) ∷ FixedZero ∷ [] in
  eqCounts (countsDP 10 11 pat) (countsEnum 10 11 pat)

-- LCM-lift sanity: L = 15, p ∈ {3,5}
Test₃ : Bool
Test₃ =
  let pat = Open (1 ∷ 3 ∷ 7 ∷ 9 ∷ []) ∷ FixedZero ∷ [] in
  let c15 = countsDP 10 15 pat in
  let z3  = sumEvery 3 c15 in
  let z5  = sumEvery 5 c15 in
  let c3  = countsDP 10 3  pat in
  let c5  = countsDP 10 5  pat in
  (if if-dec (z3 ≟ count0 c3) then (if-dec (z5 ≟ count0 c5)) else false)
  where
    if-dec : {A : Set} → Dec A → Bool
    if-dec (yes _) = true
    if-dec (no  _) = false
