{-# OPTIONS --guardedness #-}

-- Complete orthogonality testing framework using Float arithmetic
-- Implements Hardy-Littlewood orthogonality pattern for prime pairs
--
-- This module is COMPLETE (no postulates for core logic) and compiles
-- with the GHC backend to produce executable validation.
--
-- Based on: Babylonian score studies showing orthogonality after HL normalization
-- Adapted from: Complete implementation by collaborator

module Complete.OrthogonalityFloat where

open import Agda.Builtin.Nat       using (Nat; suc; zero; _+_)
open import Data.Bool      using (Bool; true; false; if_then_else_)
open import Agda.Builtin.Int       using (Int)
open import Data.Float.Base using (Float)
  renaming
  ( _+_   to _+f_
  ; _-_   to _-f_
  ; _*_   to _*f_
  ; _÷_   to _/f_
  ; _<ᵇ_  to _<f_
  ; sqrt  to sqrtF
  ; show  to showF
  ; fromℕ to primNatToFloat
  )
open import Data.String.Base using (String; _++_)
open import Agda.Builtin.String using (primShowNat)
open import Data.Unit.Polymorphic.Base using (⊤; tt)
open import Data.Product   using (_×_; _,_; proj₁; proj₂)
open import IO                     using (IO; putStrLn)
open import Level                  using (0ℓ)

-- Basic list
infixr 5 _∷_
data List (A : Set) : Set where
  []  : List A
  _∷_ : A → List A → List A

map : {A B : Set} → (A → B) → List A → List B
map f []       = []
map f (x ∷ xs) = f x ∷ map f xs

foldr : {A B : Set} → (A → B → B) → B → List A → B
foldr f z []       = z
foldr f z (x ∷ xs) = f x (foldr f z xs)

filter : {A : Set} → (A → Bool) → List A → List A
filter p []       = []
filter p (x ∷ xs) = if p x then x ∷ filter p xs else filter p xs

infixr 3 _∧ᵇ_
infixr 2 _∨ᵇ_

notB : Bool → Bool
notB true  = false
notB false = true

_∧ᵇ_ : Bool → Bool → Bool
true ∧ᵇ b = b
false ∧ᵇ _ = false

_∨ᵇ_ : Bool → Bool → Bool
true ∨ᵇ _ = true
false ∨ᵇ b = b

length : {A : Set} → List A → Nat
length []       = 0
length (_ ∷ xs) = suc (length xs)

-- Nat utilities
infix  4 _≤_ _<_ _==_
infixl 6 _-_
infixl 7 _*_ _÷_ _%_

_≤_ : Nat → Nat → Bool
zero  ≤ _        = true
suc _ ≤ zero     = false
suc m ≤ suc n    = m ≤ n

_<_ : Nat → Nat → Bool
m < n = suc m ≤ n

_==_ : Nat → Nat → Bool
zero  == zero    = true
zero  == suc _   = false
suc _ == zero    = false
suc m == suc n   = m == n

_-_ : Nat → Nat → Nat
m     - zero   = m
zero  - suc _  = 0
suc m - suc n  = m - n

-- Integer division and modulus for Nat (total when divisor > 0)
{-# TERMINATING #-}
divMod : Nat → Nat → Nat × Nat
divMod m n with n
... | 0       = 0 , m
... | suc n'  = go m 0
  where
    go : Nat → Nat → Nat × Nat
    go k q with k ≤ n'
    ... | true  = q , k
    ... | false = go (k - suc n') (suc q)

_÷_ : Nat → Nat → Nat
m ÷ n = proj₁ (divMod m n)

_%_ : Nat → Nat → Nat
m % n = proj₂ (divMod m n)

-- sqrt floor (for primality)
_*_ : Nat → Nat → Nat
zero  * _      = 0
suc m * n      = n + (m * n)

{-# TERMINATING #-}
sqrt : Nat → Nat
sqrt n = go 0
  where
    sq : Nat → Nat
    sq k = k * k

    go : Nat → Nat
    go k with (sq k ≤ n) , (sq (suc k) ≤ n)
    ... | true  , true  = go (suc k)
    ... | true  , false = k
    ... | false , _     = 0

-- primes
{-# TERMINATING #-}
isPrime : Nat → Bool
isPrime n with n ≤ 1
... | true  = false
... | false = loop 2
  where
    loop : Nat → Bool
    loop d with d ≤ sqrt n
    ... | false = true
    ... | true  = if (n % d == 0) then false else loop (suc d)

appendLast : {A : Set} → List A → A → List A
appendLast []       y = y ∷ []
appendLast (x ∷ xs) y = x ∷ appendLast xs y

upto : Nat → List Nat
upto 0       = 0 ∷ []
upto (suc n) = appendLast (upto n) (suc n)

primesUpTo : Nat → List Nat
primesUpTo n = filter isPrime (filter (λ k → 2 ≤ k) (upto n))

-- divisors and τ
divides : Nat → Nat → Bool
divides d n = if d == 0 then false else (n % d == 0)

divisors : Nat → List Nat
divisors n = filter (λ d → divides d n) (upto n)

tau : Nat → Nat
tau n = length (divisors n)

-- v_p
{-# TERMINATING #-}
vp : Nat → Nat → Nat
vp n p with p ≤ 1
... | true  = 0
... | false with n % p == 0
... | true  = suc (vp (n ÷ p) p)
... | false = 0

distinctPrimeFactors : Nat → List Nat
distinctPrimeFactors n =
  filter (λ p → isPrime p ∧ (n % p == 0))
         (filter (λ k → 2 ≤ k) (upto n))
  where
    _∧_ : Bool → Bool → Bool
    true  ∧ b = b
    false ∧ _ = false

othersCount : Nat → Nat
othersCount n =
  let ps = distinctPrimeFactors n in
  length (filter (λ p → notB (p == 2) ∧ᵇ notB (p == 3) ∧ᵇ notB (p == 5)) ps)

toF : Nat → Float
toF = primNatToFloat

-- Babylonian score (base-60 centric)
-- This is the structural heuristic from HL prime pair literature
-- Formula: 2(e₂ + e₃ + e₅) + bonus - 3·others + 0.5·τ
babylonian : Nat → Float
babylonian g with g % 2 == 1
... | true  = 0.0
... | false =
  let e2 = vp g 2
      e3 = vp g 3
      e5 = vp g 5
      bon = if (g % 60 == 0) then 10.0 else 0.0
      oth = toF (othersCount g)
      τ   = toF (tau g)
  in  (((2.0 *f toF (e2 + e3 + e5)) +f bon) -f (3.0 *f oth)) +f (0.5 *f τ)

-- singular series S(g) = 2*C₂ * ∏_{p | g/2, p>2} (p-1)/(p-2)
-- This is the Hardy-Littlewood correction factor
C2 : Float
C2 = 0.6601618158468696

singSeries : Nat → Float
singSeries g with (g == 0) ∨ᵇ (g % 2 == 1)
... | true  = 0.0
... | false =
  let k   = g ÷ 2
      ps  = filter (λ p → notB (p == 2)) (distinctPrimeFactors k)
      prod = foldr (λ p acc → ((toF (p - 1)) /f (toF (p - 2))) *f acc) 1.0 ps
  in  (2.0 *f C2) *f prod

-- Count prime pairs (p, p+g) up to N
pairsRaw : Nat → Nat → Nat
pairsRaw N g =
  let ps = primesUpTo N in
  foldr (λ p acc → if (p + g ≤ N) ∧ isPrime (p + g) then suc acc else acc) 0 ps
  where
    _∧_ : Bool → Bool → Bool
    true  ∧ b = b
    false ∧ _ = false

-- Generate even gaps 2, 4, 6, ..., G
{-# TERMINATING #-}
gaps : Nat → List Nat
gaps G = step 2
  where
    step : Nat → List Nat
    step k with k ≤ G
    ... | false = []
    ... | true  = k ∷ step (k + 2)

-- Statistical functions (Float)
sumF : List Float → Float
sumF = foldr (λ x acc → x +f acc) 0.0

meanF : List Float → Float
meanF xs = sumF xs /f primNatToFloat (length xs)

zipWithF : List Float → List Float → List Float
zipWithF []       []       = []
zipWithF (x ∷ xs) (y ∷ ys) = (x *f y) ∷ zipWithF xs ys
zipWithF _        _        = []

center : Float → List Float → List Float
center μ = map (λ x → x -f μ)

-- Covariance
covF : List Float → List Float → Float
covF xs ys =
  let n  = primNatToFloat (length xs)
      μx = meanF xs
      μy = meanF ys
      cx = center μx xs
      cy = center μy ys
  in  sumF (zipWithF cx cy) /f n

-- Variance
varF : List Float → Float
varF xs =
  let n  = primNatToFloat (length xs)
      μ  = meanF xs
      c  = center μ xs
  in  sumF (map (λ z → z *f z) c) /f n

-- Pearson correlation coefficient
corrF : List Float → List Float → Float
corrF xs ys =
  let cv = covF xs ys
      vx = varF xs
      vy = varF ys
  in  if (0.0 <f vx) ∧ᵇ (0.0 <f vy)
      then cv /f sqrtF (vx *f vy)
      else 0.0

showN : Nat → String
showN = primShowNat

-- One-shot run with constants; tweak N/G here:
-- N = search range for primes
-- G = maximum gap to test
N : Nat
N = 1000000

G : Nat
G = 300

-- Main execution: test orthogonality pattern on prime pairs
-- Computes correlation before and after HL normalization
-- Expected: r(raw) > 0, r(norm) ≈ 0 (orthogonality)
main : IO {a = 0ℓ} ⊤
main =
  let gs     = gaps G
      bab    = map babylonian gs
      raw    = map (λ g → primNatToFloat (pairsRaw N g)) gs
      norm   = map (λ g → let s = singSeries g in if 0.0 <f s then (primNatToFloat (pairsRaw N g)) /f s else 0.0) gs
      rRaw   = corrF bab raw
      rNorm  = corrF bab norm
      out    = "N=" ++ showN N ++ " G=" ++ showN G ++ "\n"
             ++ "r(raw)  = " ++ showF rRaw  ++ "\n"
             ++ "r(norm) = " ++ showF rNorm ++ "\n"
  in putStrLn out
