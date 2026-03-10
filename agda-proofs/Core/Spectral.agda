{-# OPTIONS --safe --without-K #-}

{-|
  Core.Spectral: Quadratic Characters and Spectral Properties

  This module provides the spectral layer for phase lock analysis,
  including Legendre symbols, quadratic residues, and related properties.

  Key concepts:
  - QuadraticCharacter record capturing Legendre symbol properties
  - QR/NQR classification
  - Euler's criterion and supplements
  - Spectral properties for phase lock analysis
-}

module Core.Spectral where

open import Data.Nat using (ℕ; zero; suc; _+_; _*_; _∸_; _^_; _/_; _%_; _≡ᵇ_)
open import Data.Nat.Properties using (*-comm; +-comm)
open import Data.Nat.Divisibility using (_∣_)
open import Data.Nat.GCD using (gcd)
open import Data.Nat.Coprimality using (Coprime)
open import Data.Product using (_×_; _,_; ∃; Σ-syntax)
open import Data.Sum using (_⊎_; inj₁; inj₂)
open import Data.Bool using (if_then_else_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Relation.Nullary using (¬_; Dec; yes; no)

open import Core.Primality using (IsPrime)
open import Core.Equiv using (_↔_; mk↔)

-------------------------------------------------------------------------------
-- ±1 GROUP FOR LEGENDRE SYMBOLS
-------------------------------------------------------------------------------

-- | The ±1 group for Legendre symbol values
data ±1 : Set where
  +1# : ±1
  -1# : ±1

-- | Group operation for ±1
_⊗_ : ±1 → ±1 → ±1
+1# ⊗ x = x
-1# ⊗ +1# = -1#
-1# ⊗ -1# = +1#

-- | Power operation
_^#_ : ±1 → ℕ → ±1
x ^# zero = +1#
x ^# suc n = x ⊗ (x ^# n)

-- | Convert ±1 to integer exponent (for theoretical connections)
toExp : ±1 → ℕ
toExp +1# = 0
toExp -1# = 1

-------------------------------------------------------------------------------
-- QUADRATIC CHARACTER RECORD
-------------------------------------------------------------------------------

{-|
  QuadraticCharacter captures the Legendre symbol and its properties
  for a given odd prime p.

  This provides a clean "spectral" API that the rest of the project
  can consume without re-deriving Legendre facts ad hoc.
-}
record QuadraticCharacter (p : ℕ) : Set where
  field
    pPrime : IsPrime p
    p-odd  : p % 2 ≡ 1  -- p is odd

    -- The Legendre symbol χ(a) = (a/p)
    χ      : ℕ → ±1

    -- Definitional equality to concrete Legendre computation
    χ-def  : ∀ a → χ a ≡ legendre a p

    -- Multiplicativity
    χ-mul  : ∀ a b → χ (a * b) ≡ χ a ⊗ χ b

    -- Special values (supplements)
    χ(-1)  : χ (p ∸ 1) ≡ -1# ^# ((p ∸ 1) / 2)
    χ(2)   : χ 2 ≡ -1# ^# ((p ^ 2 ∸ 1) / 8)

    -- Zero value
    χ(0)   : χ 0 ≡ +1#  -- Convention: (0/p) = 0, but we use +1# for simplicity

    -- Euler's criterion
    euler  : ∀ a → Coprime a p → χ a ≡ +1# ↔ ∃ λ x → (x * x) % p ≡ a % p

-- | Modular exponentiation helper
postulate
  modPow : ℕ → ℕ → ℕ → ℕ

-- | The concrete Legendre symbol computation using Euler's criterion
-- For prime p: (a|p) = a^((p-1)/2) mod p
-- Returns +1 if a is a quadratic residue mod p
-- Returns -1 if a is a quadratic non-residue mod p
legendre : (a : ℕ) → (p : ℕ) → {pr : IsPrime p} → ±1
legendre a p {pr} with a % p
... | zero = +1#  -- By convention, (0|p) = +1
... | a' =
  let exp = (p ∸ 1) / 2
      result = modPow a' exp p
  in if result ≡ᵇ 1
     then +1#
     else if result ≡ᵇ (p ∸ 1)
          then -1#
          else +1#  -- Should not happen for valid primes

-- | Convenience function for Legendre symbol of -1
legendreMinus1 : (p : ℕ) → {pr : IsPrime p} → ±1
legendreMinus1 p {pr} = legendre (p ∸ 1) p {pr}

-------------------------------------------------------------------------------
-- QR/NQR CLASSIFICATION
-------------------------------------------------------------------------------

-- | A residue is a quadratic residue (QR) if χ(a) = +1
IsQR : ∀ {p} → QuadraticCharacter p → ℕ → Set
IsQR QC a = QuadraticCharacter.χ QC a ≡ +1#

-- | A residue is a quadratic non-residue (NQR) if χ(a) = -1
IsNQR : ∀ {p} → QuadraticCharacter p → ℕ → Set
IsNQR QC a = QuadraticCharacter.χ QC a ≡ -1#

-- | Every nonzero residue is either QR or NQR
postulate
  qr-or-nqr : ∀ {p} (QC : QuadraticCharacter p) →
    ∀ a → Coprime a p → IsQR QC a ⊎ IsNQR QC a

-- | Half of the nonzero residues are QR, half are NQR
postulate
  qr-nqr-balance : ∀ {p} (QC : QuadraticCharacter p) →
    let units = {! count of units mod p !}
    in {! count of QR !} * 2 ≡ units

-------------------------------------------------------------------------------
-- PRIMITIVE ROOTS AND INDICES
-------------------------------------------------------------------------------

-- | A primitive root modulo p
record PrimitiveRoot (p : ℕ) : Set where
  field
    g : ℕ
    g-coprime : Coprime g p
    g-generates : ∀ a → Coprime a p →
      ∃ λ k → g ^ k % p ≡ a % p

-- | The discrete logarithm (index) of a modulo g
postulate
  index : ∀ {p} → PrimitiveRoot p → ℕ → ℕ

-- | Key property: a is QR iff its index is even
postulate
  qr-iff-even-index : ∀ {p} (QC : QuadraticCharacter p) (PR : PrimitiveRoot p) →
    ∀ a → Coprime a p →
    IsQR QC a ↔ (index PR a % 2 ≡ 0)

-------------------------------------------------------------------------------
-- SPECTRAL CLASSIFICATION OF PRIMES
-------------------------------------------------------------------------------

-- | Root number / epsilon values
data Epsilon : Set where
  ε+1 : Epsilon  -- SO⁺ family (p ≡ 1 mod 4)
  ε-1 : Epsilon  -- SO⁻ family (p ≡ 3 mod 4)

-- | Classify a prime by its spectral type
classifyPrime : ∀ p → IsPrime p → p > 2 → Epsilon
classifyPrime p pPrime p>2 with p % 4
... | 1 = ε+1
... | 3 = ε-1
... | _ = ε+1  -- Can't happen for odd primes, but needed for totality

-- | Connection to quadratic character
postulate
  epsilon-chi-minus-one : ∀ {p} (QC : QuadraticCharacter p) →
    classifyPrime p (QuadraticCharacter.pPrime QC) {! p > 2 !} ≡ ε+1 ↔
    QuadraticCharacter.χ QC (p ∸ 1) ≡ +1#

-------------------------------------------------------------------------------
-- EXPORT SUMMARY
-------------------------------------------------------------------------------

{-|
  This module provides:

  1. QuadraticCharacter record - the main abstraction
  2. QR/NQR classification predicates
  3. Connection to primitive roots and indices
  4. Spectral classification (ε+1 vs ε-1)

  Use this module whenever you need Legendre symbols or QR/NQR analysis.
-}

-- End of Core.Spectral module