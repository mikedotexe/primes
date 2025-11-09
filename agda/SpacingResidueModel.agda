{-# OPTIONS --safe #-}

------------------------------------------------------------------------
-- Spacing-Based Residue Model
--
-- ✓  SCOPE: This module formalizes the DEFAULT construction:
--            symmetric spacing with INDEPENDENT digit sampling
--
-- Core insight: Spacing alone (exponent patterns in base expansion)
--               creates modular traps that shift with midpoint length
--
-- Key properties:
--   • Open slots sample digits independently (no mirroring)
--   • Layout is symmetric (midpoint + zero runs + slot widths)
--   • Residue distribution P(n ≡ r mod m) determined by:
--       - Positions (exponents) of open slots
--       - Allowed digits in each slot
--       - Base and modulus interaction
--
-- This is the mathematical foundation for the DP residue model
-- implemented in tools/density-explorer/src/main.rs
------------------------------------------------------------------------

module SpacingResidueModel where

open import Data.Nat using (ℕ; zero; suc; _+_; _*_; _^_; _≤_; _%_)
open import Data.Nat.Properties as ℕₚ
open import Data.List using (List; []; _∷_; length; foldr; map)
open import Data.Vec using (Vec; []; _∷_)
open import Data.Fin using (Fin)
open import Relation.Binary.PropositionalEquality using (_≡_; refl; cong; sym; trans)
open import Data.Product using (Σ; Σ-syntax; _,_; proj₁; proj₂; _×_)
open import Data.Rational using (ℚ; 0ℚ; 1ℚ; _+_; _*_; _/_)
open import Data.Integer using (ℤ; +_)

------------------------------------------------------------------------
-- Pattern specification (spacing-symmetric layout)

-- Midpoint specification
data MidpointSpec : Set where
  Free  : ℕ → MidpointSpec  -- Free digits of given length
  Zeros : ℕ → MidpointSpec  -- Fixed zeros of given length

-- Layer specification (symmetric around midpoint)
record Layer : Set where
  field
    zeroCount : ℕ    -- Fixed zeros
    slotCount : ℕ    -- Open slots (independently sampled)

-- Complete pattern (spacing-symmetric, no digit mirroring)
record Pattern : Set where
  field
    base     : ℕ
    midpoint : MidpointSpec
    layers   : List Layer    -- Inner to outer

------------------------------------------------------------------------
-- Open slot specification

-- Position (exponent) and allowed digits for one open slot
record OpenSlot : Set where
  field
    exponent      : ℕ        -- Position in base expansion
    allowedDigits : List ℕ   -- Digits that can appear here
    isLeading     : Bool     -- True if this is leading position (no zero allowed)
    isLastDigit   : Bool     -- True if this determines last digit (coprimality filter)

-- Extract all open slots from a pattern
-- This is the key data structure for the residue model
postulate
  extractOpenSlots : Pattern → List OpenSlot

------------------------------------------------------------------------
-- Residue distribution via DP

-- Probability distribution over residues modulo m
ResidueDistribution : ℕ → Set
ResidueDistribution m = Vec ℚ m

-- Base case: start with all probability at residue 0
initialDistribution : (m : ℕ) → ResidueDistribution m
initialDistribution zero    = []
initialDistribution (suc n) = 1ℚ ∷ (foldr (λ _ acc → 0ℚ ∷ acc) [] (replicate n 0))
  where
  open import Data.Vec using (replicate)

-- Update distribution by adding one open slot
-- For each current residue r with probability p:
--   For each allowed digit d:
--     Transfer probability p/|allowed| to residue (r + d·b^exp) mod m
postulate
  updateWithSlot
    : ∀ {m} → ResidueDistribution m → OpenSlot → ℕ → ResidueDistribution m
  -- Parameters: current distribution, slot spec, modulus
  -- Returns: updated distribution after processing this slot

-- Fold over all slots to compute final distribution
computeResidueDistribution
  : Pattern → ℕ → ResidueDistribution
computeResidueDistribution pattern modulus =
    foldr (updateWithSlot modulus) (initialDistribution modulus) slots
  where
  slots = extractOpenSlots pattern
  postulate updateWithSlot : ℕ → OpenSlot → ResidueDistribution modulus → ResidueDistribution modulus

-- Extract probability of being divisible by m
-- (probability that n ≡ 0 mod m)
postulate
  getProbabilityDivisible : ∀ {m} → ResidueDistribution (suc m) → ℚ
  -- Returns: dist[0], the probability at residue 0

------------------------------------------------------------------------
-- Key theorems about spacing-driven filtering

-- Theorem 1: Spacing can create non-uniform residue distributions
-- (even with uniform digit sampling)
postulate
  spacing-creates-bias
    : ∀ (pattern : Pattern) (modulus : ℕ)
    → (m≥2 : 2 ≤ modulus)
    → let dist = computeResidueDistribution pattern modulus
          p0   = getProbabilityDivisible dist
          uniform = 1ℚ / (+ modulus)
      in ¬ (p0 ≡ uniform)  -- Probability at 0 deviates from uniform

-- Theorem 2: GCD between base and modulus amplifies bias
-- When gcd(base, modulus) > 1, certain exponent patterns create "traps"
postulate
  gcd-amplifies-spacing-bias
    : ∀ (base modulus : ℕ)
    → (gcd : ℕ) → gcd ≡ ℕₚ.gcd base modulus
    → gcd > 1
    → ∃ λ (positions : List ℕ) →
        ∃ λ (bias : ℚ) →
          bias > 0ℚ ∧ bias ≠ 1ℚ / (+ modulus)

-- Theorem 3: Midpoint length shifts modular traps
-- Changing midpoint length changes exponent patterns, shifting residue bias
postulate
  midpoint-shifts-traps
    : ∀ (pattern1 pattern2 : Pattern)
    → (sameLayout : sameLayers pattern1 pattern2)
    → (diffMidpoint : midpointLength pattern1 ≠ midpointLength pattern2)
    → ∃ λ (modulus : ℕ) →
        let dist1 = computeResidueDistribution pattern1 modulus
            dist2 = computeResidueDistribution pattern2 modulus
        in getProbabilityDivisible dist1 ≠ getProbabilityDivisible dist2
  where
  postulate
    sameLayers      : Pattern → Pattern → Set
    midpointLength  : Pattern → ℕ

-- Theorem 4: Independence preserves no (b+1) divisibility guarantee
-- Unlike palindromes, spacing-symmetric patterns with independent slots
-- do NOT automatically divide by (b+1)
postulate
  spacing-symmetric-not-universally-divisible
    : ∀ (pattern : Pattern)
    → (isSpacingSymmetric : IsSpacingSymmetric pattern)
    → ¬ (∀ (digits : SampleFromPattern pattern) →
           (Pattern.base pattern + 1) ∣ evalDigits pattern digits)
  where
  postulate
    IsSpacingSymmetric : Pattern → Set
    SampleFromPattern  : Pattern → Set
    evalDigits         : (p : Pattern) → SampleFromPattern p → ℕ

------------------------------------------------------------------------
-- Connection to local-factors baseline

-- The local-factors baseline uses the exact residue model:
--   density ≈ ∏_{p ∈ track} (1 - P(n ≡ 0 mod p)) / ln(x)
--
-- This is MORE accurate than conditional PNT when:
--   • Base has non-trivial factorization
--   • Spacing creates modular traps
--   • Midpoint length interacts with prime moduli

postulate
  local-factors-from-residue-model
    : ∀ (pattern : Pattern) (trackedPrimes : List ℕ)
    → ℚ  -- Predicted prime density
    -- Implementation:
    -- prod = ∏_{p ∈ tracked} (1 - getProbabilityDivisible (computeResidueDistribution pattern p))
    -- density = prod / ln(length)

-- Correctness: local-factors baseline converges to true density
-- as we track more small primes
postulate
  local-factors-convergence
    : ∀ (pattern : Pattern) (trackedPrimes : List ℕ)
    → (allPrime : AllPrime trackedPrimes)
    → (sorted : Sorted trackedPrimes)
    → let predicted = local-factors-from-residue-model pattern trackedPrimes
          observed  = empiricalDensity pattern  -- from sampling
      in |predicted - observed| ≤ O(1 / sqrt(sampledCount))
  where
  postulate
    AllPrime        : List ℕ → Set
    Sorted          : List ℕ → Set
    empiricalDensity : Pattern → ℚ
    sampledCount    : ℕ
    _-_             : ℚ → ℚ → ℚ
    |_|             : ℚ → ℚ
    O               : ℚ → ℚ
    sqrt            : ℕ → ℚ

------------------------------------------------------------------------
-- Example: Base 10, midpoint shifts mod-3 trap

-- In base 10, positions with exponent ≡ 0 mod 2 interact with mod-2 trap
-- Positions with exponent pattern affect mod-3 differently based on 10 ≡ 1 (mod 3)

exampleBase10Mod3Shift : Set
exampleBase10Mod3Shift =
  let pattern1 = record {
        base = 10 ;
        midpoint = Free 3 ;  -- Odd midpoint length
        layers = []
      }
      pattern2 = record {
        base = 10 ;
        midpoint = Free 4 ;  -- Even midpoint length
        layers = []
      }
      dist1 = computeResidueDistribution pattern1 3
      dist2 = computeResidueDistribution pattern2 3
      p0₁   = getProbabilityDivisible dist1
      p0₂   = getProbabilityDivisible dist2
  in p0₁ ≠ p0₂  -- Different divisibility probabilities!

------------------------------------------------------------------------
-- Concrete counterexamples: Spacing-symmetric ≠ Palindrome

-- Example 1: Base 10, spacing-symmetric but NOT divisible by 11
--
-- Pattern: 3 zeros, 1 free digit (midpoint), 3 zeros
-- Layout:  0 0 0 [d] 0 0 0  (spacing is symmetric)
--
-- Possible values: 0001000, 0002000, ..., 0009000
-- In base 10: 1000, 2000, ..., 9000
--
-- Check divisibility by (base+1) = 11:
--   1000 mod 11 = 10  ✗
--   2000 mod 11 = 9   ✗
--   3000 mod 11 = 8   ✗
--   ...
-- None are divisible by 11, even though layout is symmetric!

counterexample1-base10-not-div-11 : Set
counterexample1-base10-not-div-11 =
  let pattern = record {
        base = 10 ;
        midpoint = Free 1 ;
        layers = record { zeroCount = 3 ; slotCount = 0 } ∷ []
      }
      -- Generated number: d₀ × 10³ where d₀ ∈ {1..9}
      -- For d₀=1: 1000 mod 11 = 10 ≠ 0
  in ∃ λ (n : ℕ) →
       (generatedBy pattern n) ∧ ¬((10 + 1) ∣ n)
  where
  postulate generatedBy : Pattern → ℕ → Set

-- Example 2: Base 6, spacing-symmetric with independent sampling
--
-- Pattern: [d₁] 0 0 [d₂] (spacing is symmetric: 1 slot, 2 zeros, 1 slot)
-- Palindrome would force d₂ = d₁
-- Spacing-symmetric allows d₁ ≠ d₂
--
-- Palindrome:  1001₆ = 217₁₀ = 7 × 31  (divisible by 7 = 6+1) ✓
-- Non-mirror:  1002₆ = 218₁₀ = 2 × 109 (NOT divisible by 7) ✗
--              2001₆ = 433₁₀ = prime   (NOT divisible by 7) ✗
--
-- Same spacing, different divisibility!

counterexample2-base6-independence-breaks-div : Set
counterexample2-base6-independence-breaks-div =
  let pattern = record {
        base = 6 ;
        midpoint = Zeros 2 ;
        layers = record { zeroCount = 0 ; slotCount = 1 } ∷ []
      }
      -- Layout: [d₁] 0 0 [d₂]
      -- Palindrome: d₁ = d₂ → always divisible by 7
      -- Independent: d₁ ≠ d₂ possible → NOT always divisible by 7
      n1 = 1 * 6^3 + 0 * 6^2 + 0 * 6^1 + 2 * 6^0  -- 1002₆ = 218₁₀
      n2 = 2 * 6^3 + 0 * 6^2 + 0 * 6^1 + 1 * 6^0  -- 2001₆ = 433₁₀
  in (¬((6 + 1) ∣ n1)) ∧ (¬((6 + 1) ∣ n2))

-- Example 3: Explicit comparison - Palindrome vs Spacing-Symmetric
--
-- Base 10, length 6
-- Layout: [a][b][c] [c][b][a]  (3 open slots, symmetric positions)
--
-- PALINDROME construction:
--   Sample a,b,c independently
--   Mirror: rightmost 3 digits = reverse of leftmost 3
--   Result: always length-6 even palindrome
--   Property: ALWAYS divisible by 11 = (10+1) ✓
--   Example: 123321, 456654, 789987
--
-- SPACING-SYMMETRIC construction:
--   Sample all 6 digits independently
--   No mirroring enforced
--   Same layout (positions symmetric) but values independent
--   Property: NOT always divisible by 11 ✗
--   Counterexample: 123456 mod 11 = 3 ≠ 0

comparison-palindrome-vs-spacing : Set
comparison-palindrome-vs-spacing =
  let layout = record {
        base = 10 ;
        midpoint = Zeros 0 ;  -- No midpoint
        layers = record { zeroCount = 0 ; slotCount = 3 } ∷ []
      }

      -- Palindrome: 123321
      palindrome = 1*10^5 + 2*10^4 + 3*10^3 + 3*10^2 + 2*10^1 + 1*10^0
      -- Check: 123321 mod 11 = 0 ✓

      -- Spacing-symmetric (independent): 123456
      independent = 1*10^5 + 2*10^4 + 3*10^3 + 4*10^2 + 5*10^1 + 6*10^0
      -- Check: 123456 mod 11 = 3 ✗

  in ((10 + 1) ∣ palindrome) ∧ ¬((10 + 1) ∣ independent)

------------------------------------------------------------------------
-- Visualization: Why spacing-symmetry ≠ palindrome

-- Palindrome divisibility proof relies on PAIRING symmetric digit VALUES:
--   d_i × b^i + d_j × b^j  where d_i = d_j (mirroring!)
--   = d_i × (b^i + b^j)
--   = d_i × b^i × (1 + b^(j-i))
--   When j-i is odd, (b+1) ∣ (1 + b^(j-i)) by factorization
--
-- Spacing-symmetric with independence breaks this:
--   d_i × b^i + d_j × b^j  where d_i ≠ d_j (independent!)
--   Cannot factor out common digit
--   No guaranteed divisibility by (b+1)

postulate
  palindrome-pairing-requires-equal-digits
    : ∀ (base : ℕ) (i j : ℕ) (d_i d_j : ℕ)
    → (odd-gap : Odd (j - i))
    → (mirrored : d_i ≡ d_j)  -- PALINDROME property
    → (base + 1) ∣ (d_i * base^i + d_j * base^j)

  spacing-symmetric-independence-breaks-divisibility
    : ∀ (base : ℕ) (i j : ℕ) (d_i d_j : ℕ)
    → (symmetric-positions : i + j ≡ totalLen - 1)  -- Positions symmetric
    → (independent : d_i ≠ d_j)  -- But values INDEPENDENT
    → ¬ (∀ d_i d_j → (base + 1) ∣ (d_i * base^i + d_j * base^j))
  where
  postulate
    totalLen : ℕ
    Odd : ℕ → Set

------------------------------------------------------------------------
-- Key insight: Spacing creates DIFFERENT filtering mechanism

-- Palindromes: Universal (b+1) divisibility wall → systematic filtering
-- Spacing-symmetric: Modular traps from exponent patterns → selective filtering
--
-- The spacing-based approach is MORE FLEXIBLE:
--   • Can avoid (b+1) trap entirely
--   • Can target specific moduli via exponent engineering
--   • Midpoint tuning shifts which primes are filtered

postulate
  spacing-offers-flexible-filtering
    : ∀ (base : ℕ) (targetModulus : ℕ)
    → ∃ λ (pattern : Pattern) →
        let dist = computeResidueDistribution pattern targetModulus
            p0   = getProbabilityDivisible dist
        in (p0 > 1ℚ / (+ targetModulus))  -- Enhanced filtering for target
           ∧ (pattern is not palindromic)  -- But avoids (b+1) wall
  where
  postulate _is_not_palindromic : Pattern → Set

------------------------------------------------------------------------
-- Future work: Connect to empirical findings

-- These postulates should be proven or validated against:
--
-- 1. Rust implementation in tools/density-explorer
--    • DP residue computation (residue_null_probability)
--    • Local-factors baseline (expected_density_local)
--
-- 2. Empirical data from grid sweeps
--    • Midpoint length vs prime density plots
--    • Observed vs predicted divisibility rates
--
-- 3. GCD paradox data
--    • Higher gcd → better filtering (via stronger traps)
--    • Correlation between gcd and success rate
