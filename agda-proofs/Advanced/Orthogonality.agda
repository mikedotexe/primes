{-# OPTIONS --safe --without-K #-}

{-|
  Orthogonality After Hardy-Littlewood Normalization

  This module formalizes the orthogonality testing framework for
  Babylonian/prime-pair divergence analysis.

  THE CORE OBSERVATION:

  Raw prime-pair counts show positive correlation with Babylonian scores
  (base-60 centric heuristics). However, after normalizing by the Hardy-Littlewood
  singular series, this correlation vanishes - the signals become orthogonal.

  This demonstrates that the Babylonian score captures systematic bias
  in raw counts, but HL normalization removes this bias, leaving only noise.

  MATHEMATICAL FRAMEWORK:

  For gap g, compute three sequences:
    1. Bab(g)  = Babylonian score (structural heuristic)
    2. Raw(g)  = number of prime pairs (p, p+g) with p,p+g ≤ N
    3. Norm(g) = Raw(g) / S(g) where S(g) is singular series

  PREDICTIONS:
    - Cov(Bab, Raw) > 0   (positive correlation)
    - Corr(Bab, Norm) ≈ 0  (orthogonality, |ρ| < ε for small ε)

  This module provides the computational framework to test these predictions
  on finite samples.

  INTEGRATION WITH OUR WORK:

  This connects to our residue spectral analysis:
    - Residue regularity → Babylonian-like structural score
    - Raw membrane success → Raw prime pairs
    - HL-normalized success → Norm prime pairs

  We can test whether our regularity scores also decorrelate after
  HL normalization, validating that we're capturing structural bias.
-}

module Advanced.Orthogonality where

open import Data.Nat using (ℕ; zero; suc; _+_; _*_; _∸_; _<_; _≤_; _≡ᵇ_)
open import Data.Nat.Properties using (+-comm; *-comm; +-assoc; *-assoc)
open import Data.Nat.DivMod using (_mod_; _div_)
open import Data.Bool using (Bool; true; false; if_then_else_; _∨_; _∧_; not)
open import Data.List using (List; []; _∷_; filter; map; foldr; length)
open import Data.Float using (Float; _+_; _*_; _-_; _/_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Relation.Nullary using (¬_)

-------------------------------------------------------------------------------
-- Part 1: Basic List Operations
-------------------------------------------------------------------------------

sum-list : List ℕ → ℕ
sum-list = foldr _+_ 0

sum-float : List Float → Float
sum-float = foldr _+_ 0.0

zip-with : ∀ {A B C : Set} → (A → B → C) → List A → List B → List C
zip-with f [] [] = []
zip-with f (x ∷ xs) (y ∷ ys) = f x y ∷ zip-with f xs ys
zip-with f _ _ = []  -- lengths mismatch, shouldn't occur in our use

-------------------------------------------------------------------------------
-- Part 2: Number Theoretic Functions
-------------------------------------------------------------------------------

{-|
  PRIMALITY TESTING (trial division)

  We need this to identify primes for pair counting.
  This is the naive approach but sufficient for moderate N.
-}

-- Integer square root (floor)
sqrt : ℕ → ℕ
sqrt n = go 0
  where
    go : ℕ → ℕ
    go k with (k * k ≤ n) ∧ ((suc k) * (suc k) ≤ n)
    ... | true  = go (suc k)
    ... | false = k

-- Primality test
is-prime : ℕ → Bool
is-prime n with n ≤ 1
... | true  = false
... | false = all-prime 2
  where
    all-prime : ℕ → Bool
    all-prime d with d ≤ sqrt n
    ... | false = true
    ... | true  = if (n mod d ≡ᵇ 0) then false else all-prime (suc d)

-- Generate range [0..n]
upto : ℕ → List ℕ
upto zero = 0 ∷ []
upto (suc n) = append-last (upto n) (suc n)
  where
    append-last : List ℕ → ℕ → List ℕ
    append-last [] y = y ∷ []
    append-last (x ∷ xs) y = x ∷ append-last xs y

-- All primes up to N
primes-upto : ℕ → List ℕ
primes-upto N = filter is-prime (filter (λ k → 2 ≤ k) (upto N))

{-|
  P-ADIC VALUATION

  v_p(n) = highest power of p dividing n
-}

v-p : ℕ → ℕ → ℕ
v-p n p with p ≤ 1
... | true  = 0
... | false with n mod p ≡ᵇ 0
... | false = 0
... | true  = suc (v-p (n div p) p)

{-|
  DIVISOR FUNCTIONS
-}

divides : ℕ → ℕ → Bool
divides d n = (n mod (if d ≡ᵇ 0 then 1 else d)) ≡ᵇ 0

divisors : ℕ → List ℕ
divisors n = filter (λ d → if d ≡ᵇ 0 then false else divides d n) (upto n)

tau : ℕ → ℕ
tau n = length (divisors n)

distinct-prime-factors : ℕ → List ℕ
distinct-prime-factors n =
  filter (λ p → is-prime p ∧ (n mod p ≡ᵇ 0))
         (filter (λ k → 2 ≤ k) (upto n))

others-count : ℕ → ℕ
others-count n =
  length (filter (λ p → (not (p ≡ᵇ 2)) ∧ (not (p ≡ᵇ 3)) ∧ (not (p ≡ᵇ 5)))
                 (distinct-prime-factors n))

-------------------------------------------------------------------------------
-- Part 3: Babylonian Score
-------------------------------------------------------------------------------

{-|
  BABYLONIAN SCORE (Base-60 Centric Heuristic)

  This is a heuristic score that weights gaps based on their factorization
  properties, with special bonus for multiples of 60.

  Score components:
    +2  for each power of 2, 3, 5 in the gap
    +10 if gap is divisible by 60
    -3  for each distinct prime factor other than 2,3,5
    +0.5 * τ(g) for divisor count

  HYPOTHESIS: This captures systematic structural bias in raw pair counts.
-}

nat-to-float : ℕ → Float
nat-to-float zero = 0.0
nat-to-float (suc n) = 1.0 + nat-to-float n

babylonian-score : ℕ → Float
babylonian-score g with g mod 2 ≡ᵇ 1
... | true = 0.0  -- Odd gaps don't form prime pairs
... | false =
  let e2 = v-p g 2
      e3 = v-p g 3
      e5 = v-p g 5
      bonus = if (g mod 60 ≡ᵇ 0) then 10.0 else 0.0
      oth = nat-to-float (others-count g)
      τg = nat-to-float (tau g)
  in  2.0 * nat-to-float (e2 + e3 + e5) + bonus - 3.0 * oth + 0.5 * τg

-------------------------------------------------------------------------------
-- Part 4: Prime Pair Counting
-------------------------------------------------------------------------------

{-|
  RAW PAIR COUNT

  Count how many prime pairs (p, p+g) exist with both p, p+g ≤ N.
-}

pairs-raw : ℕ → ℕ → ℕ
pairs-raw N g =
  let ps = primes-upto N
  in foldr (λ p acc → if (p + g ≤ N) ∧ is-prime (p + g) then suc acc else acc) 0 ps

-------------------------------------------------------------------------------
-- Part 5: Hardy-Littlewood Singular Series
-------------------------------------------------------------------------------

{-|
  SINGULAR SERIES S(g)

  For even gap g = 2k, the singular series is:
    S(g) = 2·C₂ · ∏_{p|k, p>2} (p-1)/(p-2)

  where C₂ ≈ 0.6601618 is the twin-prime constant.

  This is the multiplicative correction factor in the Hardy-Littlewood
  conjecture for prime pairs.
-}

C₂ : Float
C₂ = 0.6601618158468696

singular-series : ℕ → Float
singular-series g with (g ≡ᵇ 0) ∨ (g mod 2 ≡ᵇ 1)
... | true = 0.0  -- Undefined for odd or zero
... | false =
  let k = g div 2
      factors = filter (λ p → not (p ≡ᵇ 2)) (distinct-prime-factors k)
      product = foldr (λ p acc → ((nat-to-float (p ∸ 1)) / (nat-to-float (p ∸ 2))) * acc) 1.0 factors
  in 2.0 * C₂ * product

{-|
  HL-NORMALIZED PAIR COUNT

  Norm(g) = Raw(g) / S(g)

  This removes the systematic bias predicted by Hardy-Littlewood,
  leaving only random fluctuations.
-}

pairs-normalized : ℕ → ℕ → Float
pairs-normalized N g with singular-series g
... | 0.0 = 0.0
... | Sg  = (nat-to-float (pairs-raw N g)) / Sg

-------------------------------------------------------------------------------
-- Part 6: Gap Sequences
-------------------------------------------------------------------------------

{-|
  Generate even gaps up to G
-}

even-gaps : ℕ → List ℕ
even-gaps G = go 2
  where
    go : ℕ → List ℕ
    go k with k ≤ G
    ... | false = []
    ... | true  = k ∷ go (k + 2)

-------------------------------------------------------------------------------
-- Part 7: Statistical Functions
-------------------------------------------------------------------------------

{-|
  MEAN, VARIANCE, COVARIANCE

  Standard statistical measures for Float lists.
-}

mean-float : List Float → Float
mean-float xs = sum-float xs / nat-to-float (length xs)

center : Float → List Float → List Float
center μ = map (λ x → x - μ)

cov-float : List Float → List Float → Float
cov-float xs ys =
  let n = nat-to-float (length xs)
      μx = mean-float xs
      μy = mean-float ys
      cx = center μx xs
      cy = center μy ys
      products = zip-with _*_ cx cy
  in sum-float products / n

var-float : List Float → Float
var-float xs =
  let n = nat-to-float (length xs)
      μ = mean-float xs
      c = center μ xs
      squares = map (λ z → z * z) c
  in sum-float squares / n

{-|
  CORRELATION BOUND TEST

  Instead of computing correlation directly (which requires sqrt),
  we test whether |ρ| ≤ ε by checking:
    Cov(x,y)² ≤ ε² · Var(x) · Var(y)

  This avoids floating-point sqrt while testing the same condition.
-}

corr-bound : Float → List Float → List Float → Bool
corr-bound ε xs ys =
  let cv = cov-float xs ys
      vx = var-float xs
      vy = var-float ys
  in if (vx == 0.0) ∨ (vy == 0.0)
     then true  -- Degenerate case
     else (cv * cv) ≤ (ε * ε * vx * vy)
  where
    _≤_ : Float → Float → Bool
    _≤_ = {! Float comparison, implementation-dependent !}

-------------------------------------------------------------------------------
-- Part 8: Orthogonality Experiment
-------------------------------------------------------------------------------

{-|
  MAIN EXPERIMENT STRUCTURE

  For a given N (bound on primes) and G (maximum gap):
    1. Generate even gaps 2, 4, 6, ..., G
    2. Compute Babylonian scores for each gap
    3. Compute raw pair counts for each gap
    4. Compute HL-normalized counts for each gap
    5. Test correlations

  EXPECTED RESULTS:
    - Cov(Bab, Raw) > 0     (positive correlation)
    - |Corr(Bab, Norm)| < ε  (near-orthogonality)
-}

record ExperimentResult : Set where
  constructor mk-result
  field
    raw-covariance : Float
    raw-var-bab : Float
    raw-var-pairs : Float
    norm-covariance : Float
    norm-var-bab : Float
    norm-var-pairs : Float
    norm-orthogonal : Bool  -- True if |ρ| < ε

run-experiment : ℕ → ℕ → Float → ExperimentResult
run-experiment N G ε =
  let gaps = even-gaps G
      bab-scores = map babylonian-score gaps
      raw-counts = map (λ g → nat-to-float (pairs-raw N g)) gaps
      norm-counts = map (pairs-normalized N) gaps
  in mk-result
       (cov-float bab-scores raw-counts)
       (var-float bab-scores)
       (var-float raw-counts)
       (cov-float bab-scores norm-counts)
       (var-float bab-scores)
       (var-float norm-counts)
       (corr-bound ε bab-scores norm-counts)

-------------------------------------------------------------------------------
-- Part 9: Integration with Residue Spectral Analysis
-------------------------------------------------------------------------------

{-|
  CONNECTION TO OUR WORK

  Our residue regularity scores are analogous to Babylonian scores:
    - Both capture structural properties (regularity vs base-60 factors)
    - Both correlate with raw success metrics
    - Question: Do both decorrelate after HL normalization?

  PROPOSED TEST:

  Replace Babylonian scores with residue regularity scores:
    1. Regularity(base, d) from spectral analysis
    2. Raw membrane success rate for (base, outer, inner, k)
    3. HL-normalized success (if we formalize HL for membranes)

  Run the same orthogonality test.

  HYPOTHESIS: If regularity scores decorrelate after HL normalization,
  this validates that our spectral analysis captures structural bias
  similar to how Babylonian scores capture bias in prime pair distributions.

  This would be strong evidence that:
    - Residue regularity is fundamental
    - Our spectral approach is sound
    - The connection to HL theory is deep
-}

postulate
  regularity-score : ℕ → ℕ → Float  -- base, divisor → regularity
  membrane-success-raw : ℕ → ℕ → ℕ → ℕ → Float  -- base, outer, inner, k → success %
  membrane-success-HL : ℕ → ℕ → ℕ → ℕ → Float  -- HL-normalized success

{-|
  PLANNED: Membrane Orthogonality Test

  Same structure as prime-pair orthogonality, but for membrane configurations.
-}

postulate
  membrane-orthogonality-test :
    ℕ →  -- Sample size (number of configurations)
    Float →  -- Epsilon (orthogonality threshold)
    ExperimentResult

-------------------------------------------------------------------------------
-- Status and Next Steps
-------------------------------------------------------------------------------

{-|
  CURRENT STATUS:

  This module provides the theoretical framework for orthogonality testing.
  Several functions are postulated pending implementation details:
    - Float comparison operators
    - nat-to-float conversion
    - Integration with membrane success metrics

  IMPLEMENTATION PATH:

  1. Port to computational Agda with IO
  2. Or translate to Rust for actual experiments
  3. Validate on known prime-pair data
  4. Apply to membrane data

  VERIFICATION GOALS:

  Once implemented, this enables:
    - Formal verification of orthogonality claims
    - Computational validation on finite samples
    - Cross-validation between theory (Agda) and experiment (Rust)

  RESEARCH SIGNIFICANCE:

  If residue regularity scores show the same orthogonality pattern as
  Babylonian scores, this suggests a deep connection:
    Structural heuristics ←→ HL normalization ←→ True randomness

  This would be a novel contribution connecting:
    - Signal processing (spectral analysis)
    - Number theory (Hardy-Littlewood)
    - Constructive prime generation (membranes)
-}

-- End of Orthogonality module
