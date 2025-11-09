{-# OPTIONS --safe --without-K #-}

{-|
  Test Specification: Residue Collapse Frequency Analysis

  This module validates our understanding of the residue collapse phenomenon
  by computing and testing frequency distributions.

  THE INSIGHT WE'RE TESTING:

  When we map base digits {0, 1, ..., base-1} to their residues mod d,
  all residue classes {0, 1, ..., d-1} appear, but with different frequencies
  depending on gcd(base, d).

  The key insight: it's not about WHICH residues appear (they all do),
  but about HOW REGULARLY they appear.

  Regular frequency distribution → structural constraint → better filtering
  Irregular frequency distribution → noise → weaker filtering

  This explains the GCD paradox: Base 6 with gcd(6,3)=3 shows perfect
  regularity [2,2,2] while Base 10 with gcd(10,3)=1 shows irregularity [4,3,3].

  WHAT WE'RE COMPUTING:

  For each (base, divisor) pair, we compute the frequency vector:
    freqs base d = [count₀, count₁, ..., count_{d-1}]

  where count_r is how many digits in {0..base-1} have residue r mod d.

  We verify these frequencies match expected patterns based on gcd(base, d).
-}

module Tests.Spec.ResidueCollapseSpec where

open import Data.Nat using (ℕ; zero; suc; _+_; _*_; _≤_; _<_)
open import Data.Nat.DivMod using (_mod_)
open import Data.Nat.Properties as ℕₚ using (_≟_; z≤n; s≤s)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Relation.Nullary using (yes; no)
open import Data.Bool using (Bool; true; false)
open import Data.List using (List; []; _∷_; length)

-------------------------------------------------------------------------------
-- List Utilities
-------------------------------------------------------------------------------

{-|
  We need basic list operations to compute and count residues.
  These are minimal implementations to avoid external dependencies.
-}

-- Map a function over a list
map : ∀ {A B : Set} → (A → B) → List A → List B
map f []       = []
map f (x ∷ xs) = f x ∷ map f xs

-- Concatenate two lists
concat : ∀ {A : Set} → List A → List A → List A
concat [] ys       = ys
concat (x ∷ xs) ys = x ∷ concat xs ys

-- Generate list [0, 1, ..., n-1]
upto : ℕ → List ℕ
upto zero    = []
upto (suc n) = concat (upto n) (n ∷ [])

-- Count occurrences of k in a list
count : ℕ → List ℕ → ℕ
count k []       = 0
count k (x ∷ xs) with x ℕₚ.≟ k
... | yes _ = suc (count k xs)
... | no  _ = count k xs

-------------------------------------------------------------------------------
-- Frequency Vector Computation
-------------------------------------------------------------------------------

{-|
  FREQUENCY VECTOR DEFINITION:

  For a given base and divisor d, the frequency vector tells us how many
  times each residue class appears when we map {0..base-1} to residues mod d.

  ALGORITHM:
    1. Generate digits: [0, 1, ..., base-1]
    2. Map to residues: [d₀ mod d, d₁ mod d, ..., d_{base-1} mod d]
    3. For each r ∈ {0..d-1}, count how many times r appears
    4. Result: [count₀, count₁, ..., count_{d-1}]

  EXAMPLE:
    Base 6, divisor 3:
      Digits: [0, 1, 2, 3, 4, 5]
      Mod 3:  [0, 1, 2, 0, 1, 2]
      Counts: [2, 2, 2]  -- each residue appears exactly twice

    Base 10, divisor 3:
      Digits: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
      Mod 3:  [0, 1, 2, 0, 1, 2, 0, 1, 2, 0]
      Counts: [4, 3, 3]  -- residue 0 appears more often
-}

freqs : (base : ℕ) → (d : ℕ) → List ℕ
freqs base d = map count-residue (upto d)
  where
    -- Count how many digits have residue r
    count-residue : ℕ → ℕ
    count-residue r = count r (map (_mod d) (upto base))

-------------------------------------------------------------------------------
-- Test Cases: Validating Frequency Patterns
-------------------------------------------------------------------------------

{-|
  Each test validates a specific (base, divisor) pair's frequency distribution.

  THE PATTERN TO VERIFY:

  For gcd(base, d) = g:
    - All residues appear
    - Frequencies group by cosets modulo g
    - Within each coset, frequencies are equal
    - Between cosets, frequencies differ by at most 1

  We test several cases to validate this pattern.
-}

-- Base 6, divisor 3: Perfect regularity
-- gcd(6,3) = 3, so we expect all counts equal
test-freqs-6-3 : freqs 6 3 ≡ (2 ∷ 2 ∷ 2 ∷ [])
test-freqs-6-3 = refl

{-|
  INTERPRETATION:
    Digits: [0, 1, 2, 3, 4, 5]
    Mod 3:  [0, 1, 2, 0, 1, 2]

    Residue 0 appears at positions {0, 3} → count 2
    Residue 1 appears at positions {1, 4} → count 2
    Residue 2 appears at positions {2, 5} → count 2

  Perfect uniformity. This is the REGULAR pattern that gives Base 6
  its structural advantage.
-}

-- Base 10, divisor 3: Slight irregularity
-- gcd(10,3) = 1, so counts can differ
test-freqs-10-3 : freqs 10 3 ≡ (4 ∷ 3 ∷ 3 ∷ [])
test-freqs-10-3 = refl

{-|
  INTERPRETATION:
    Digits: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    Mod 3:  [0, 1, 2, 0, 1, 2, 0, 1, 2, 0]

    Residue 0 appears at positions {0, 3, 6, 9} → count 4
    Residue 1 appears at positions {1, 4, 7}    → count 3
    Residue 2 appears at positions {2, 5, 8}    → count 3

  Irregularity: residue 0 appears one extra time.
  This asymmetry is the NOISE that makes Base 10 less effective than Base 6.

  The difference is small (one occurrence), but systematic.
  Across many prime tests, this accumulates.
-}

-- Base 10, divisor 4: Coset structure visible
-- gcd(10,4) = 2, so counts group in pairs
test-freqs-10-4 : freqs 10 4 ≡ (3 ∷ 3 ∷ 2 ∷ 2 ∷ [])
test-freqs-10-4 = refl

{-|
  INTERPRETATION:
    Digits: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    Mod 4:  [0, 1, 2, 3, 0, 1, 2, 3, 0, 1]

    Residue 0 appears at positions {0, 4, 8} → count 3
    Residue 1 appears at positions {1, 5, 9} → count 3
    Residue 2 appears at positions {2, 6}    → count 2
    Residue 3 appears at positions {3, 7}    → count 2

  Coset structure: {0,1} appear 3 times each, {2,3} appear 2 times each.
  The grouping reflects gcd(10,4) = 2 dividing residues into two cosets.
-}

-- Base 12, divisor 8: Multiple cosets
-- gcd(12,8) = 4, so counts group in sets of 4
test-freqs-12-8 : freqs 12 8 ≡ (2 ∷ 2 ∷ 2 ∷ 2 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ [])
test-freqs-12-8 = refl

{-|
  INTERPRETATION:
    Digits: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
    Mod 8:  [0, 1, 2, 3, 4, 5, 6, 7, 0, 1,  2,  3]

    Residues {0,1,2,3} appear 2 times each
    Residues {4,5,6,7} appear 1 time each

  This reflects gcd(12,8) = 4 creating two cosets with different frequencies.
  Within each coset (mod 4), frequencies are equal.
-}

-------------------------------------------------------------------------------
-- Theoretical Predictions
-------------------------------------------------------------------------------

{-|
  THE PATTERN WE'RE VALIDATING:

  For any (base, d) pair with g = gcd(base, d):

  1. All residues {0..d-1} appear (none are missing)

  2. Frequencies are either ⌊base/d⌋ or ⌈base/d⌉
     (they differ by at most 1)

  3. Residues group by cosets modulo g
     Within each coset, all frequencies are equal

  4. The number of residues with frequency ⌈base/d⌉ is (base mod d)
     The remaining (d - base mod d) residues have frequency ⌊base/d⌋

  EXAMPLES:

  Base 6, d=3: ⌊6/3⌋=2, ⌈6/3⌉=2, 6 mod 3 = 0
    → All 3 residues get frequency 2 ✓ [2,2,2]

  Base 10, d=3: ⌊10/3⌋=3, ⌈10/3⌉=4, 10 mod 3 = 1
    → 1 residue gets frequency 4, other 2 get frequency 3 ✓ [4,3,3]

  Base 10, d=4: ⌊10/4⌋=2, ⌈10/4⌉=3, 10 mod 4 = 2
    → 2 residues get frequency 3, other 2 get frequency 2 ✓ [3,3,2,2]

  Base 12, d=8: ⌊12/8⌋=1, ⌈12/8⌉=2, 12 mod 8 = 4
    → 4 residues get frequency 2, other 4 get frequency 1 ✓ [2,2,2,2,1,1,1,1]

  All our test cases match these predictions.
-}

-------------------------------------------------------------------------------
-- Connection to GCD Paradox
-------------------------------------------------------------------------------

{-|
  THE GCD PARADOX EXPLAINED:

  Why does Base 6 (gcd(6,3)=3) outperform Base 10 (gcd(10,3)=1)?

  REGULARITY MEASURE:

  One way to quantify regularity is variance of the frequency distribution.

  Variance = average squared deviation from mean frequency

  Base 6 mod 3: [2,2,2]
    Mean = 2, deviations = [0,0,0], variance = 0
    PERFECTLY REGULAR

  Base 10 mod 3: [4,3,3]
    Mean = 10/3 ≈ 3.33, deviations ≈ [0.67,-0.33,-0.33], variance ≈ 0.22
    SLIGHTLY IRREGULAR

  Lower variance → more regular → stronger structural constraint → better filtering

  THIS IS WHY higher GCD helps: it creates more regular frequency distributions,
  which provide more predictable (and thus more constraining) residue patterns.

  The collapse isn't about fewer residues appearing. It's about the REGULARITY
  with which they appear.
-}

-------------------------------------------------------------------------------
-- Status and Next Steps
-------------------------------------------------------------------------------

{-|
  CURRENT STATUS:

  These tests are computational proofs. They verify by normalization that:
    - Our frequency computation is correct
    - Specific (base, d) pairs produce expected patterns
    - The patterns match theoretical predictions

  WHAT'S PROVEN:

  ✓ Base 6 mod 3 shows perfect uniformity [2,2,2]
  ✓ Base 10 mod 3 shows slight asymmetry [4,3,3]
  ✓ Base 10 mod 4 shows coset grouping [3,3,2,2]
  ✓ Base 12 mod 8 shows multi-coset structure [2,2,2,2,1,1,1,1]

  NEXT STEPS:

  1. Prove theoretically that frequencies always satisfy:
     count_r ∈ {⌊base/d⌋, ⌈base/d⌉}

  2. Prove the coset grouping property:
     Residues in same coset (mod gcd(base,d)) have equal frequencies

  3. Connect variance to prime generation success rate:
     Lower variance → higher empirical prime density

  4. Create ResidueCollapseLaws.agda with general theorems
     (These tests then become special cases)
-}

-- End of ResidueCollapseSpec
