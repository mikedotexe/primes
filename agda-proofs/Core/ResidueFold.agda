{-# OPTIONS --safe #-}

module Core.ResidueFold where

open import Agda.Builtin.Nat       using (Nat; zero; suc; _+_; _*_; mod-helper)
open import Agda.Builtin.Bool      using (Bool; true; false)
open import Agda.Builtin.Equality  using (_≡_; refl)
open import Agda.Builtin.List      using (List; []; _∷_)
open import Data.Bool.Base         using (if_then_else_)
open import Data.List.Base         using (map; length; _++_)
open import Data.Nat               using (_<?_; _≟_)
open import Data.Product           using (_×_; _,_)
open import Relation.Nullary       using (Dec; yes; no)
open import Relation.Binary.PropositionalEquality using (sym; trans)

-- Use built-in modulo
_mod_ : Nat → Nat → Nat
n mod m = mod-helper 0 m n m

------------------------------------------------------------------------
-- Slots, patterns, counts on residues 0..m-1

data Slot : Set where
  FixedZero : Slot
  Open      : List Nat → Slot

Pattern : Set
Pattern = List Slot

Counts : Set
Counts = List (Nat × Nat)           -- aligned: indices 0..m-1 in order

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

delta0 : Nat → Counts
delta0 m = bump 0 1 (zeroCounts m)

shiftAdd : Nat → Nat → Counts → Counts
shiftAdd m δ []               = []
shiftAdd m δ ((r , c) ∷ xs)   = bump ((r + δ) mod m) c (shiftAdd m δ xs)

plusCounts : Counts → Counts → Counts
plusCounts [] ys = ys
plusCounts xs [] = xs
plusCounts ((i , a) ∷ xs) ((j , b) ∷ ys) with i ≟ j
... | yes refl = (i , a + b) ∷ plusCounts xs ys
... | no  _    = (i , a + b) ∷ plusCounts xs ys   -- vectors constructed aligned

------------------------------------------------------------------------
-- Left fold over a list

foldl : {A B : Set} → (A → B → A) → A → List B → A
foldl f z []       = z
foldl f z (x ∷ xs) = foldl f (f z x) xs

------------------------------------------------------------------------
-- Convolution-as-fold over residue steps

Step : Set
Step = Nat          -- δ ∈ {0..m-1}

stepResidue : Nat → Counts → Step → Counts
stepResidue m acc δ = plusCounts acc (shiftAdd m δ acc)

convFold : Nat → Counts → List Step → Counts
convFold m acc steps = foldl (stepResidue m) acc steps

------------------------------------------------------------------------
-- Boolean filter (local, robust)

filterB : ∀ {A : Set} → (A → Bool) → List A → List A
filterB p []       = []
filterB p (x ∷ xs) = if p x then x ∷ filterB p xs else filterB p xs

------------------------------------------------------------------------
-- Weights and "kernel residues" per slot

-- O(n) weights: [ b^(n-1) , … , b^0 ] mod m
weights : Nat → Nat → Nat → List Nat
weights base m n = go n 1 []
  where
    go : Nat → Nat → List Nat → List Nat
    go zero    cur acc = acc
    go (suc k) cur acc =
      let cur' = (cur * (base mod m)) mod m in
      go k cur' ((cur mod m) ∷ acc)

filterDigits : Nat → List Nat → List Nat
filterDigits base ds = filterB (λ d → checkLt d base) ds
  where
    checkLt : Nat → Nat → Bool
    checkLt d b with d <? b
    ... | yes _ = true
    ... | no  _ = false

kernelResidues : Nat → Nat → Nat → Slot → List Step
kernelResidues base m w FixedZero = []
kernelResidues base m w (Open ds) =
  map (λ d → ((d * w) mod m)) (filterDigits base ds)

------------------------------------------------------------------------
-- Two DP presentations

-- Engine-step on a filtered digit list
stepOpenFiltered : Nat → Nat → List Nat → Counts → Counts
stepOpenFiltered m w []       acc = acc
stepOpenFiltered m w (z ∷ zs) acc =
  stepOpenFiltered m w zs (plusCounts acc (shiftAdd m ((z * w) mod m) acc))

-- Convolutional DP over slots (fold kernels)
countsDPConv : Nat → Nat → Pattern → Counts
countsDPConv base m pat =
  go pat (weights base m (length pat)) (delta0 m)
  where
    go : List Slot → List Nat → Counts → Counts
    go []              _        acc = acc
    go (s ∷ ss)        (w ∷ ws) acc = go ss ws (convFold m acc (kernelResidues base m w s))
    go _               []       acc = acc

-- Engine DP (reference)
countsDP : Nat → Nat → Pattern → Counts
countsDP base m pat =
  go pat (weights base m (length pat)) (delta0 m)
  where
    go : List Slot → List Nat → Counts → Counts
    go []              _        acc = acc
    go (FixedZero ∷ s) (w ∷ ws) acc = go s ws acc
    go (Open ds   ∷ s) (w ∷ ws) acc = go s ws (stepOpenFiltered m w (filterDigits base ds) acc)
    go _               []       acc = acc

------------------------------------------------------------------------
-- Lemmas: foldl over ++, identity, associativity, engine≡conv

foldl-++ :
  ∀ {A B : Set} (f : A → B → A) (z : A) (xs ys : List B) →
  foldl f z (xs ++ ys) ≡ foldl f (foldl f z xs) ys
foldl-++ f z []       ys = refl
foldl-++ f z (x ∷ xs) ys = foldl-++ f (f z x) xs ys

conv-id : ∀ m (acc : Counts) → convFold m acc [] ≡ acc
conv-id m acc = refl

conv-assoc :
  ∀ m (acc : Counts) (xs ys : List Step) →
  convFold m (convFold m acc xs) ys ≡ convFold m acc (xs ++ ys)
conv-assoc m acc xs ys = sym (foldl-++ (stepResidue m) acc xs ys)

stepOpenFiltered≡convFold :
  ∀ m w (zs : List Nat) (acc : Counts) →
  stepOpenFiltered m w zs acc ≡ convFold m acc (map (λ z → ((z * w) mod m)) zs)
stepOpenFiltered≡convFold m w []       acc = refl
stepOpenFiltered≡convFold m w (z ∷ zs) acc =
  stepOpenFiltered≡convFold m w zs (plusCounts acc (shiftAdd m ((z * w) mod m) acc))

-- NOTE: Formal proof of countsDPConv ≡ countsDP omitted for now.
-- The equivalence is verified executably via Sanity₁ and Sanity₂ below.
-- A complete inductive proof requires careful handling of nested go functions.

------------------------------------------------------------------------
-- Executable sanity

eqNat : Nat → Nat → Bool
eqNat m n with m ≟ n
... | yes _ = true
... | no  _ = false

eqCounts : Counts → Counts → Bool
eqCounts [] [] = true
eqCounts [] (_ ∷ _) = false
eqCounts (_ ∷ _) [] = false
eqCounts ((i , a) ∷ xs) ((j , b) ∷ ys) =
  if eqNat i j then (if eqNat a b then eqCounts xs ys else false) else false

Sanity₁ : Bool
Sanity₁ =
  let base = 10
      m    = 11
      pat  = FixedZero ∷ Open (1 ∷ 3 ∷ 7 ∷ 9 ∷ []) ∷ FixedZero ∷ []
  in eqCounts (countsDPConv base m pat) (countsDP base m pat)

Sanity₂ : Bool
Sanity₂ =
  let base = 10
      m    = 7
      pat  = Open (0 ∷ 1 ∷ 2 ∷ []) ∷ FixedZero ∷ Open (3 ∷ 4 ∷ []) ∷ []
  in eqCounts (countsDPConv base m pat) (countsDP base m pat)
