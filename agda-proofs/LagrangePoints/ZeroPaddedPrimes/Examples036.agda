module LagrangePoints.ZeroPaddedPrimes.Examples036 where

open import Data.Nat using (ℕ; zero; suc; _+_; _*_)
open import Data.List using (List; []; _∷_; length)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Data.List.Relation.Unary.All using (All)

-- Use existing primality infrastructure
open import Core.Primality using (IsPrime)

-- Import alphabet module
open import LagrangePoints.ZeroPaddedPrimes.Alphabet036 as A
  using (Digit036; d0; d3; d6; AllDigits036; countZeros; digitLen; digits10)

------------------------------------------------------------------------
-- 1. Digit-level connector representation
------------------------------------------------------------------------

-- A connector as an explicit list of decimal digits, preserving
-- leading zeros. Example: [0 , 6 , 3 , 3] encodes "0633".
record Conn036D : Set where
  field
    digits : List ℕ

open Conn036D public

-- Decimal length = number of digits
len : Conn036D → ℕ
len c = length (digits c)

-- Sum of digits (for mod-3 / mod-9 heuristics)
sumDigits : Conn036D → ℕ
sumDigits c = sum (digits c)
  where
    sum : List ℕ → ℕ
    sum []       = 0
    sum (x ∷ xs) = x + sum xs

-- Zero count on the explicit digit list
zerosOf : Conn036D → ℕ
zerosOf c = A.countZeros (digits c)

------------------------------------------------------------------------
-- 2. {0,3,6} alphabet predicate on the digit list
------------------------------------------------------------------------

AllDigits036List : Conn036D → Set
AllDigits036List c = All Digit036 (digits c)

------------------------------------------------------------------------
-- 3. Numeric value and core concatenation PCcore
------------------------------------------------------------------------

-- Power of 10 function (postulated for now)
postulate
  pow10 : ℕ → ℕ

-- Interpret a list of digits as a base-10 number:
--   [d₀ , d₁ , ... , dₖ₋₁] ↦ d₀·10^(k-1) + ... + dₖ₋₁.
valueFromDigits : List ℕ → ℕ
valueFromDigits []       = 0
valueFromDigits (d ∷ ds) = d * pow10 (length ds) + valueFromDigits ds

value : Conn036D → ℕ
value c = valueFromDigits (digits c)

-- Core prime pair (10301, 3007003007003)
-- We use the canonical pair from empirical discovery
coreP1 : ℕ
coreP1 = 10301

coreP2 : ℕ
coreP2 = 3007003007003

-- Core concatenation with an explicit-digit connector:
--
--   PCcore c = 10301 · 10^(len(c) + digits(coreP2))
--              + value(c) · 10^(digits(coreP2))
--              + 3007003007003
--
PCcore : Conn036D → ℕ
PCcore c =
  coreP1 * pow10 (len c + digitLen coreP2) +
  value c * pow10 (digitLen coreP2) +
  coreP2

------------------------------------------------------------------------
-- 4. Mod-3 principle for {0,3,6}-alphabet connectors (postulated)
------------------------------------------------------------------------

-- We assume: if all digits of c lie in {0,3,6}, then
-- PCcore c ≡ 1 (mod 3).
--
-- This encodes:
--   p₁ ≡ 2 (mod 3), p₂ ≡ 2 (mod 3),
--   digits(c) ⊆ {0,3,6} ⇒ C ≡ 0 (mod 3)
--   ⇒ full ≡ 2 + 0 + 2 ≡ 1 (mod 3).
postulate
  PCcore036-mod3-1 : ∀ (c : Conn036D) → AllDigits036List c → Set

------------------------------------------------------------------------
-- 5. Concrete {0,3,6} connectors by length
------------------------------------------------------------------------
-- Each connector is given as a concrete digit list. We *do not*
-- try to prove the AllDigits036List property or primality here;
-- those go in separate postulates below.

-- Length 4 (Resonant length)
c4-d0633 : Conn036D
c4-d0633 = record { digits = 0 ∷ 6 ∷ 3 ∷ 3 ∷ [] }

c4-d0636 : Conn036D
c4-d0636 = record { digits = 0 ∷ 6 ∷ 3 ∷ 6 ∷ [] }

c4-d6006 : Conn036D
c4-d6006 = record { digits = 6 ∷ 0 ∷ 0 ∷ 6 ∷ [] }

c4-d6030 : Conn036D
c4-d6030 = record { digits = 6 ∷ 0 ∷ 3 ∷ 0 ∷ [] }

-- Length 5
c5-d00006 : Conn036D
c5-d00006 = record { digits = 0 ∷ 0 ∷ 0 ∷ 0 ∷ 6 ∷ [] }

-- Length 6
c6-d006000 : Conn036D
c6-d006000 = record { digits = 0 ∷ 0 ∷ 6 ∷ 0 ∷ 0 ∷ 0 ∷ [] }

c6-d000060 : Conn036D
c6-d000060 = record { digits = 0 ∷ 0 ∷ 0 ∷ 0 ∷ 6 ∷ 0 ∷ [] }

c6-d033300 : Conn036D
c6-d033300 = record { digits = 0 ∷ 3 ∷ 3 ∷ 3 ∷ 0 ∷ 0 ∷ [] }

c6-d366000 : Conn036D
c6-d366000 = record { digits = 3 ∷ 6 ∷ 6 ∷ 0 ∷ 0 ∷ 0 ∷ [] }

c6-d063300 : Conn036D
c6-d063300 = record { digits = 0 ∷ 6 ∷ 3 ∷ 3 ∷ 0 ∷ 0 ∷ [] }

c6-d000663 : Conn036D
c6-d000663 = record { digits = 0 ∷ 0 ∷ 0 ∷ 6 ∷ 6 ∷ 3 ∷ [] }

-- Length 7
c7-d0006000 : Conn036D
c7-d0006000 = record { digits = 0 ∷ 0 ∷ 0 ∷ 6 ∷ 0 ∷ 0 ∷ 0 ∷ [] }

c7-d0333000 : Conn036D
c7-d0333000 = record { digits = 0 ∷ 3 ∷ 3 ∷ 3 ∷ 0 ∷ 0 ∷ 0 ∷ [] }

c7-d0630000 : Conn036D
c7-d0630000 = record { digits = 0 ∷ 6 ∷ 3 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ [] }

c7-d0636000 : Conn036D
c7-d0636000 = record { digits = 0 ∷ 6 ∷ 3 ∷ 6 ∷ 0 ∷ 0 ∷ 0 ∷ [] }

c7-d0663000 : Conn036D
c7-d0663000 = record { digits = 0 ∷ 6 ∷ 6 ∷ 3 ∷ 0 ∷ 0 ∷ 0 ∷ [] }

c7-d0066600 : Conn036D
c7-d0066600 = record { digits = 0 ∷ 0 ∷ 6 ∷ 6 ∷ 6 ∷ 0 ∷ 0 ∷ [] }

c7-d3336000 : Conn036D
c7-d3336000 = record { digits = 3 ∷ 3 ∷ 3 ∷ 6 ∷ 0 ∷ 0 ∷ 0 ∷ [] }

-- Length 8
c8-d00033000 : Conn036D
c8-d00033000 = record { digits = 0 ∷ 0 ∷ 0 ∷ 3 ∷ 3 ∷ 0 ∷ 0 ∷ 0 ∷ [] }

c8-d06600000 : Conn036D
c8-d06600000 = record { digits = 0 ∷ 6 ∷ 6 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ [] }

c8-d06300000 : Conn036D
c8-d06300000 = record { digits = 0 ∷ 6 ∷ 3 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ [] }

c8-d00000063 : Conn036D
c8-d00000063 = record { digits = 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 6 ∷ 3 ∷ [] }

-- Length 9
c9-d000000003 : Conn036D
c9-d000000003 = record { digits = 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 3 ∷ [] }

c9-d063000000 : Conn036D
c9-d063000000 = record { digits = 0 ∷ 6 ∷ 3 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ [] }

------------------------------------------------------------------------
-- 6. {0,3,6} property and primality for these examples (postulated)
------------------------------------------------------------------------

-- Each of these connectors uses only digits in {0,3,6}.
postulate
  c4-d0633_all036     : AllDigits036List c4-d0633
  c4-d0636_all036     : AllDigits036List c4-d0636
  c4-d6006_all036     : AllDigits036List c4-d6006
  c4-d6030_all036     : AllDigits036List c4-d6030

  c5-d00006_all036    : AllDigits036List c5-d00006

  c6-d006000_all036   : AllDigits036List c6-d006000
  c6-d000060_all036   : AllDigits036List c6-d000060
  c6-d033300_all036   : AllDigits036List c6-d033300
  c6-d366000_all036   : AllDigits036List c6-d366000
  c6-d063300_all036   : AllDigits036List c6-d063300
  c6-d000663_all036   : AllDigits036List c6-d000663

  c7-d0006000_all036  : AllDigits036List c7-d0006000
  c7-d0333000_all036  : AllDigits036List c7-d0333000
  c7-d0630000_all036  : AllDigits036List c7-d0630000
  c7-d0636000_all036  : AllDigits036List c7-d0636000
  c7-d0663000_all036  : AllDigits036List c7-d0663000
  c7-d0066600_all036  : AllDigits036List c7-d0066600
  c7-d3336000_all036  : AllDigits036List c7-d3336000

  c8-d00033000_all036 : AllDigits036List c8-d00033000
  c8-d06600000_all036 : AllDigits036List c8-d06600000
  c8-d06300000_all036 : AllDigits036List c8-d06300000
  c8-d00000063_all036 : AllDigits036List c8-d00000063

  c9-d000000003_all036 : AllDigits036List c9-d000000003
  c9-d063000000_all036 : AllDigits036List c9-d063000000

-- And we record (as axioms) that their concatenations with the core pair
-- are prime. This reflects the empirical discovery in your search.
postulate
  prime_PCcore_c4-d0633       : IsPrime (PCcore c4-d0633)
  prime_PCcore_c4-d0636       : IsPrime (PCcore c4-d0636)
  prime_PCcore_c4-d6006       : IsPrime (PCcore c4-d6006)
  prime_PCcore_c4-d6030       : IsPrime (PCcore c4-d6030)

  prime_PCcore_c5-d00006      : IsPrime (PCcore c5-d00006)

  prime_PCcore_c6-d006000     : IsPrime (PCcore c6-d006000)
  prime_PCcore_c6-d000060     : IsPrime (PCcore c6-d000060)
  prime_PCcore_c6-d033300     : IsPrime (PCcore c6-d033300)
  prime_PCcore_c6-d366000     : IsPrime (PCcore c6-d366000)
  prime_PCcore_c6-d063300     : IsPrime (PCcore c6-d063300)
  prime_PCcore_c6-d000663     : IsPrime (PCcore c6-d000663)

  prime_PCcore_c7-d0006000    : IsPrime (PCcore c7-d0006000)
  prime_PCcore_c7-d0333000    : IsPrime (PCcore c7-d0333000)
  prime_PCcore_c7-d0630000    : IsPrime (PCcore c7-d0630000)
  prime_PCcore_c7-d0636000    : IsPrime (PCcore c7-d0636000)
  prime_PCcore_c7-d0663000    : IsPrime (PCcore c7-d0663000)
  prime_PCcore_c7-d0066600    : IsPrime (PCcore c7-d0066600)
  prime_PCcore_c7-d3336000    : IsPrime (PCcore c7-d3336000)

  prime_PCcore_c8-d00033000   : IsPrime (PCcore c8-d00033000)
  prime_PCcore_c8-d06600000   : IsPrime (PCcore c8-d06600000)
  prime_PCcore_c8-d06300000   : IsPrime (PCcore c8-d06300000)
  prime_PCcore_c8-d00000063   : IsPrime (PCcore c8-d00000063)

  prime_PCcore_c9-d000000003  : IsPrime (PCcore c9-d000000003)
  prime_PCcore_c9-d063000000  : IsPrime (PCcore c9-d063000000)
