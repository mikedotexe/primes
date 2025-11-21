{-# OPTIONS --safe #-}

-- Discriminant Theory for Quadratic Membrane Polynomials
--
-- This module formalizes the polynomial view of membranes:
--   N(X) = A·X² + S·X + A
-- where X = b^k (base to padding power), A = outer shell, S = seed
--
-- The discriminant Δ = S² - 4A² governs:
-- 1. Algebraic factorization (perfect square → composite)
-- 2. Quadratic residue behavior (Legendre symbols)
-- 3. Sieve pressure (local obstructions mod small primes)
--
-- Key theorem: If Δ is a perfect square, N(X) factors algebraically
-- → membrane is composite for all sufficiently large X

module Core.Discriminant where

open import Data.Nat as ℕ using (ℕ; _+_; _∸_; _<_; _≤_)
open import Data.Nat using () renaming (_*_ to _ℕ*_)
open import Data.Nat.Properties
open import Data.Integer as ℤ using (ℤ; +_; -_; _-_; ∣_∣)
open import Data.Integer using () renaming (_*_ to _ℤ*_)
open import Data.Integer.Properties using (*-comm; +-comm)
open import Relation.Binary.PropositionalEquality using (_≡_; refl; sym; trans; cong)
open import Relation.Nullary using (Dec; yes; no)

------------------------------------------------------------------------
-- Discriminant Computation

-- Compute discriminant Δ = S² - 4A² for quadratic A·X² + S·X + A
-- Parameters:
--   A : outer shell (boundary digit)
--   S : seed (middle value)
-- Returns: discriminant as an integer (can be negative)
discriminant : ℕ → ℕ → ℤ
discriminant A S =
  let S² = + (S ℕ* S)
      A² = + (A ℕ* A)
      fourA² = (+ 4) ℤ* A²
  in S² - fourA²

-- Notation shorthand
Δ : ℕ → ℕ → ℤ
Δ = discriminant

------------------------------------------------------------------------
-- Perfect Square Predicate

-- A discriminant is a perfect square if Δ = r² for some integer r
-- This is the ALGEBRAIC LOCK condition: perfect square → factors
record IsPerfectSquare (d : ℤ) : Set where
  constructor perfectSq
  field
    root : ℤ
    proof : root ℤ* root ≡ d

-- Decision procedure for perfect squares (to be implemented)
-- isPerfectSquare? : ∀ (d : ℤ) → Dec (IsPerfectSquare d)

------------------------------------------------------------------------
-- Polynomial Evaluation

-- Evaluate N(X) = A·X² + S·X + A at a given point
-- This represents the membrane value when padding = k and base = b,
-- with X = b^k
evaluatePolynomial : ℕ → ℕ → ℕ → ℕ
evaluatePolynomial A S X = A ℕ* (X ℕ* X) + S ℕ* X + A

-- Short notation
N : ℕ → ℕ → ℕ → ℕ
N = evaluatePolynomial

------------------------------------------------------------------------
-- Factorization via Perfect Square Discriminant

-- If Δ = r² is a perfect square, the polynomial factors as:
--   N(X) = A·X² + S·X + A = A(X - α)(X - β)
-- where α and β are the roots determined by the quadratic formula
--
-- For integer roots, this means N(X) is composite for X > max(|α|, |β|)

-- Placeholder for divisibility (to be imported from standard library)
-- _∣_ : ℕ → ℕ → Set

-- Theorem (informal statement, to be formalized and proven):
-- If discriminant is a perfect square AND roots are integers,
-- then for sufficiently large X, N(X) is composite
--
-- algebraicLockTheorem : ∀ (A S : ℕ)
--                      → IsPerfectSquare (Δ A S)
--                      → ∃[ d ] (d > 1 × ∀ (X : ℕ) → X > threshold → d ∣ N A S X)

------------------------------------------------------------------------
-- Quality Score via Legendre Symbols

-- The Legendre symbol (a/p) indicates quadratic residue status:
--   +1 : a is a quadratic residue mod p (equation has solutions)
--   -1 : a is a non-residue mod p (no solutions → "admissible")
--    0 : p divides a (degenerate case)
data LegendreSymbol : Set where
  positive : LegendreSymbol  -- +1 (quadratic residue, obstructed)
  negative : LegendreSymbol  -- -1 (non-residue, admissible)
  zero     : LegendreSymbol  --  0 (divisible, worst case)

-- Compute Legendre symbol (Δ/p) for discriminant Δ and prime p
-- Uses Euler's criterion: (a/p) ≡ a^((p-1)/2) (mod p)
-- legendreSymbol : ℤ → ℕ → LegendreSymbol  -- to be implemented

-- Quality score based on Legendre symbols for small primes
-- Higher score → better expected prime density
record DiscriminantQuality (d : ℤ) : Set where
  constructor quality
  field
    leg-3  : LegendreSymbol
    leg-5  : LegendreSymbol
    leg-7  : LegendreSymbol
    leg-11 : LegendreSymbol
    leg-13 : LegendreSymbol

  -- Helper: check if two symbols match
  matches : LegendreSymbol → LegendreSymbol → ℕ
  matches positive positive = 1
  matches negative negative = 1
  matches zero zero = 1
  matches _ _ = 0

  -- Helper: count symbols of each type
  countSymbol : LegendreSymbol → ℕ
  countSymbol target =
    matches leg-3 target +
    matches leg-5 target +
    matches leg-7 target +
    matches leg-11 target +
    matches leg-13 target

  -- Count "admissible" primes (Legendre symbol = -1)
  -- These contribute LESS sieve pressure
  admissible-count : ℕ
  admissible-count = countSymbol negative

  -- Count "obstructed" primes (Legendre symbol = +1)
  -- These contribute MORE sieve pressure
  obstructed-count : ℕ
  obstructed-count = countSymbol positive

  -- Count divisible cases (Legendre symbol = 0)
  -- These are worst case (discriminant shares factor with prime)
  divisible-count : ℕ
  divisible-count = countSymbol zero

  -- Composite quality score: admissible good, obstructed bad, divisible worst
  score : ℤ
  score = (+ admissible-count) - (+ obstructed-count) - (+ (5 ℕ* divisible-count))

-- Compute quality for a given configuration (to be implemented)
-- analyzeQuality : ℕ → ℕ → DiscriminantQuality (Δ A S)
-- analyzeQuality A S =
--   let d = Δ A S
--   in quality
--       (legendreSymbol d 3)
--       (legendreSymbol d 5)
--       (legendreSymbol d 7)
--       (legendreSymbol d 11)
--       (legendreSymbol d 13)

------------------------------------------------------------------------
-- Connection to Honorary Zero Framework

open import Core.HonoraryZero using (HZBase; symmetricDigits; shiftToHZ)

-- Discriminant degeneracy: when boundaries are symmetric around HZ,
-- discriminant behavior may be constrained
--
-- Example: Base 6, HZ = 3
--   (1,5) boundaries: NOT symmetric (1+5 ≠ 6)
--     → Δ(A=1, S) = S² - 4 varies freely with S
--   (2,4) boundaries: symmetric (2+4 = 6)
--     → Δ(A=2, S) = S² - 16 constrained by HZ-symmetry
--
-- Hypothesis: Asymmetric boundaries → better discriminant diversity

-- Predicate: do boundaries exhibit HZ-symmetry?
boundariesSymmetricHZ : HZBase → ℕ → ℕ → Set
boundariesSymmetricHZ hz outer inner = symmetricDigits hz outer inner

-- Observation from empirical data:
-- - Base 6 (1,5): NOT HZ-symmetric, ρ = +0.39 (strong correlation)
-- - Base 6 (5,1): NOT HZ-symmetric, ρ = -0.23 (negative!)
-- - So HZ-symmetry alone doesn't determine discriminant effectiveness
--
-- The actual mechanism is more subtle:
-- - Small A (outer=1) → discriminant dominated by seed
-- - Large A (outer=5) → discriminant has large negative offset

------------------------------------------------------------------------
-- Empirical Findings (from discriminant_scanner.rs results)

-- Configuration-specific behavior:
--
-- Base 6 (1,5) k=0:
--   M=1: ρ(quality, primality) = +0.30
--   M=2: ρ(quality, primality) = +0.39 ← STRONG
--   M=3: ρ(quality, primality) = +0.17 (decays)
--   Perfect squares: 1/5 at M=1, 0/30 at M=2, 0/180 at M=3
--
-- Base 6 (5,1) k=0:
--   M=1: ρ(quality, primality) = +0.20
--   M=2: ρ(quality, primality) = -0.23 ← NEGATIVE (fails!)
--   M=3: ρ(quality, primality) = +0.05 (nearly zero)
--   Perfect squares: 0/5 at M=1, 2/30 at M=2, 0/180 at M=3
--
-- Base 12 (1,5) k=0:
--   M=1: ρ(quality, primality) = +0.32
--   M=2: ρ(quality, primality) = +0.10 (weak)
--   M=3: ρ(quality, primality) = +0.02 (vanishes)
--   Perfect squares: 1/11 at M=1, 0/132 at M=2, 0/1584 at M=3
--
-- Conclusion: Discriminant framework is configuration-dependent
-- - Works for A=1 (minimal outer shell) at M=2
-- - Fails or weakens for A≥2 or M≥3
-- - Perfect square lock is real but rare (affects <2% of seeds)

------------------------------------------------------------------------
-- Integration with Multi-Layer Model

-- Discriminant provides ONE layer of a multi-level architecture:
--
-- 1. Algebraic Layer (this module):
--    - Perfect square lock (hard constraint)
--    - Quality score via Legendre symbols (~15% variance explained)
--
-- 2. Modular Layer (CRTVector, ResidueFold):
--    - Coprimality requirement (80%+ of top configs)
--    - Residue class availability
--
-- 3. Geometric Layer (MirrorObstruction, LagrangePoints):
--    - k*=0 dominance (99-100% at M∈{2,3})
--    - Symmetry breaking via perturbations
--
-- 4. Analytic Layer (Hardy-Littlewood):
--    - Length penalty ~1/(M·ln b)
--    - Density heuristics
--
-- The Honorary Zero provides the REFERENCE FRAME for layers 1-3,
-- but does not itself cause density effects.

------------------------------------------------------------------------
-- Future Work

-- 1. Implement isPerfectSquare? decision procedure
-- 2. Prove algebraicLockTheorem (perfect square → composite)
-- 3. Formalize Legendre symbol computation
-- 4. Connect to CRTVector (discriminant residues mod primes)
-- 5. Prove or refute: symmetric boundaries → discriminant degeneracy
-- 6. Quantify: how much variance does quality score explain?
-- 7. Build composite predictor: algebraic × modular × geometric × analytic

------------------------------------------------------------------------
-- Notes from Collaborator's Framework

-- "The polynomial representation N(X) = A·X² + S·X + A provides
--  the algebraic skeleton that governs all membrane constructions
--  regardless of base. However, primality depends not just on the
--  discriminant itself, but on how the polynomial evaluates when
--  X takes the specific value b^k in a given base."
--
-- This is exactly what we formalize here:
-- - discriminant(A,S) is base-independent (universal)
-- - evaluatePolynomial(A,S,X) where X=b^k is base-specific
-- - The interaction between Δ and residues mod rad(b) determines success
--
-- "Seeds that would produce discriminants with many small prime factors
--  or that are perfect squares get filtered out by the residue requirements"
--
-- This suggests coupling between:
-- - Residue filtering (k=1 in base 10 enriches {1,3,7,9})
-- - Discriminant quality (k=1 seeds have fewer bad discriminants?)
-- → Next step: test this coupling empirically (Phase 1 of integration plan)
