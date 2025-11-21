{-# OPTIONS --safe #-}

module Core.CRTVector where

open import Agda.Builtin.Nat       using (Nat; zero; suc; _+_; _*_; mod-helper)
open import Agda.Builtin.Bool      using (Bool; true; false)
open import Agda.Builtin.Equality  using (_≡_; refl)
open import Agda.Builtin.List      using (List; []; _∷_)
open import Data.Bool.Base         using (if_then_else_)
open import Data.List.Base         using (map; foldr; length; _++_)
open import Data.Nat               using (_<?_; _≟_; _∸_)
open import Data.Nat.DivMod        using (_/_; _%_)
open import Data.Nat.GCD           using (gcd)
open import Data.Product           using (_×_; _,_)
open import Relation.Nullary       using (Dec; yes; no)
open import Core.ResidueFold       using
  ( Slot ; Pattern ; FixedZero ; Open
  ; Counts ; zeroCounts ; bump ; eqCounts
  ; countsDPConv ; convFold ; stepResidue ; weights ; filterDigits ; kernelResidues
  ; delta0 ; foldl )

------------------------------------------------------------------------
-- gcd / lcm / lcmList (using stdlib gcd)

_==_ : Nat → Nat → Bool
zero   == zero   = true
zero   == suc _  = false
suc _  == zero   = false
suc m  == suc n  = m == n

-- lcm using stdlib gcd
lcm : Nat → Nat → Nat
lcm zero _    = 0
lcm _    zero = 0
lcm a (suc b) with gcd a (suc b)
... | zero    = 0
... | suc g   = (a / (suc g)) * (suc b)

lcmList : List Nat → Nat
lcmList []       = 1
lcmList (n ∷ ns) = lcm n (lcmList ns)

------------------------------------------------------------------------
-- Pushforward (projection) of a distribution from mod L to mod p
-- by summing classes r ≡ i (mod p)

projectCounts : (L p : Nat) → Counts → Counts
projectCounts L zero     dist = zeroCounts zero
projectCounts L (suc p') dist = foldl step (zeroCounts (suc p')) dist
  where
    step : Counts → (Nat × Nat) → Counts
    step acc (i , c) = bump (i % (suc p')) c acc

-- Read a count at residue r (0 if missing)
getCount : Nat → Counts → Nat
getCount r []               = 0
getCount r ((i , c) ∷ xs) with i ≟ r
... | yes refl              = c
... | no  _                 = getCount r xs

------------------------------------------------------------------------
-- "Vector P0 via L": project to each p and read residue 0

P0viaL : (base : Nat) → (ps : List Nat) → (pat : Pattern) → List (Nat × Nat)
P0viaL base ps pat =
  let L    = lcmList ps
      dist = countsDPConv base L pat
  in map (λ p → (p , getCount 0 (projectCounts L p dist))) ps

------------------------------------------------------------------------
-- CRT check: for L = lcm(ps), projection equals running directly at p
-- (executable property; returns true when equal for all p ∈ ps)

andB : Bool → Bool → Bool
andB true  x = x
andB false _ = false

all : List Bool → Bool
all []       = true
all (b ∷ bs) = andB b (all bs)

CRT-ok? : (base : Nat) → (ps : List Nat) → (pat : Pattern) → Bool
CRT-ok? base ps pat =
  let L    = lcmList ps
      distL = countsDPConv base L pat
  in all (map (λ p → eqCounts (projectCounts L p distL)
                              (countsDPConv base p pat))
               ps)

------------------------------------------------------------------------
-- Examples / Sanity (normalize to 'true')

-- A small mirrored-ish spacing pattern
pat₁ : Pattern
pat₁ = Open (1 ∷ 3 ∷ 7 ∷ 9 ∷ []) ∷ FixedZero ∷ []

TestCRT₁ : Bool
TestCRT₁ =
  let base = 10
      ps   = 3 ∷ 5 ∷ []           -- L = 15
  in CRT-ok? base ps pat₁

TestCRT₂ : Bool
TestCRT₂ =
  let base = 10
      ps   = 3 ∷ 5 ∷ 7 ∷ []       -- L = 105
      pat  = Open (0 ∷ 1 ∷ 2 ∷ 3 ∷ 4 ∷ 5 ∷ 6 ∷ 7 ∷ 8 ∷ 9 ∷ []) ∷ []
  in CRT-ok? base ps pat

-- Show P0(p) vector via projection from L
P0Demo : List (Nat × Nat)
P0Demo =
  let base = 10
      ps   = 3 ∷ 5 ∷ 7 ∷ []
  in P0viaL base ps pat₁
