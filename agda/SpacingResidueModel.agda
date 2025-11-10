{-# OPTIONS --safe #-}

------------------------------------------------------------------------
-- Spacing-Based Residue Model (DEFAULT CONSTRUCTION)
--
-- ✓  SCOPE: Spacing-symmetric layouts with INDEPENDENT digit sampling
--
-- Core insight: Exponent patterns in base expansion create modular
-- traps that shift with template parameters (midpoint length, etc.)
--
-- Key distinction: This does NOT assume digit-value mirroring.
-- Open slots sample digits independently → flexible filtering.
------------------------------------------------------------------------

module SpacingResidueModel where

open import Data.Nat using (ℕ; zero; suc; _+_; _*_; _^_; _∸_; _≟_; _≤_; _<_; _/_; _%_)
open import Data.Nat.Properties as ℕₚ
open import Data.Fin using (Fin; zero; suc; toℕ; fromℕ<)
open import Data.Vec using (Vec; []; _∷_; length; tabulate; lookup; map; foldr)
open import Data.Bool using (Bool; true; false; if_then_else_; not; _∧_; _∨_)
open import Data.List using (List; []; _∷_; map; foldr; concatMap; filter; length) renaming (_++_ to _++ₗ_)
open import Relation.Binary.PropositionalEquality using (_≡_; _≢_; refl; cong; sym; trans)
open import Relation.Nullary using (Dec; yes; no; ¬_)
open import Data.Product using (Σ; Σ-syntax; ∃; _,_; proj₁; proj₂; _×_)
open import Data.Nat.Divisibility using (_∣_; divides)
open import Data.Maybe using (Maybe; just; nothing)
open import Data.Empty using (⊥)
open import Function using (_∘_; id)

------------------------------------------------------------------------
-- Template: spacing-symmetric layout with independent slot sampling

record Template : Set where
  field
    base     : ℕ                          -- b ≥ 2 (assumed externally)
    len      : ℕ                          -- total digit count
    open?    : Fin len → Bool             -- which positions are open (vs fixed zero)
    allow    : (i : Fin len) → List ℕ     -- allowed digits at each position
    noLead0  : Bool                       -- if true, leading position forbids 0
    lastSet  : Maybe (List ℕ)             -- optional last-digit constraint (coprimality)

open Template public

------------------------------------------------------------------------
-- Helper: mirror index (for stating symmetry predicate)

postulate
  mirror : ∀ {n} → Fin (suc n) → Fin (suc n)
  -- Semantics: toℕ (mirror i) = n ∸ toℕ i
  -- Kept as postulate for now; Fin arithmetic is fiddly but computable

------------------------------------------------------------------------
-- Spacing-symmetric predicate (layout only, not values)

SpacingSymmetric : ∀ {n} → (Fin n → Bool) → Set
SpacingSymmetric {zero}  open? = ⊤
  where open import Data.Unit using (⊤)
SpacingSymmetric {suc n} open? = ∀ (i : Fin (suc n)) → open? i ≡ open? (mirror i)

------------------------------------------------------------------------
-- Non-degenerate template (at least one position has choice)

NonDegenerate : Template → Set
NonDegenerate T =
  (base T ≥ 2) × (∃ λ i → 2 ≤ Data.List.length (allow T i))
  where
  open import Data.Nat.Properties using (_≤?_)

------------------------------------------------------------------------
-- Allowed digits at position i (respecting constraints)

AllowedAt : (T : Template) → (i : Fin (len T)) → List ℕ
AllowedAt T i with open? T i
... | false = 0 ∷ []  -- Fixed zero
... | true  = applyConstraints (allow T i) i
  where
  -- Helper: apply leading-zero and last-digit constraints
  postulate
    applyConstraints : List ℕ → Fin (len T) → List ℕ
    -- Implementation sketch:
    --   • If i = 0 and noLead0 T: filter (≢ 0)
    --   • If i = len T - 1 and lastSet T ≢ nothing: intersect with lastSet
    --   • Otherwise: identity

------------------------------------------------------------------------
-- Digit assignment respecting template

DigitsRespect : (T : Template) → Vec ℕ (len T) → Set
DigitsRespect T ds = ∀ (i : Fin (len T)) → lookup ds i ∈ₗ AllowedAt T i
  where
  postulate _∈ₗ_ : ℕ → List ℕ → Set

------------------------------------------------------------------------
-- Evaluation (MSB-first)

eval : (b : ℕ) → ∀ {n} → Vec ℕ n → ℕ
eval b {zero}  []       = 0
eval b {suc n} (d ∷ ds) = d * b ^ n + eval b ds

------------------------------------------------------------------------
-- Modular weight at position i (exponent = len - 1 - i)

weightAt : (b m : ℕ) → ∀ {n} → Fin n → ℕ
weightAt b m {zero}  ()
weightAt b m {suc n} i = (b ^ (n ∸ toℕ i)) % m

------------------------------------------------------------------------
-- Residue set computation via DP
--
-- Key idea: Start with {0}, then for each open position i:
--   R' = { (r + d·wᵢ) mod m | r ∈ R, d ∈ AllowedAt i }
--
-- This computes exactly which residues are reachable.

-- Single DP step: update residue set with one position
stepResidue : (b m : ℕ) → ∀ {n} → Fin n → List ℕ → Template → List ℕ
stepResidue b m {n} i currentRes T =
  let w = weightAt b m i
      allowed = AllowedAt T i
  in removeDuplicates (concatMap (λ r → map (λ d → (r + d * w) % m) allowed) currentRes)
  where
  postulate removeDuplicates : List ℕ → List ℕ

-- Full computation: fold over all positions
Residues : (T : Template) → (m : ℕ) → List ℕ
Residues T m =
  let positions = allFins (len T)
  in Data.List.foldr (stepResidue (base T) m) (0 ∷ []) positions
  where
  postulate allFins : (n : ℕ) → List (Fin n)

------------------------------------------------------------------------
-- Key correctness theorem: DP result ↔ actual divisibility

postulate
  zero-in-Residues↔exists-assignment
    : ∀ (T : Template) (m : ℕ)
    → (0 ∈ₗ Residues T m)
    ↔ (∃ λ (ds : Vec ℕ (len T)) → DigitsRespect T ds × ((eval (base T) ds % m) ≡ 0))
  where
  postulate
    _∈ₗ_ : ℕ → List ℕ → Set
    _↔_ : Set → Set → Set

-- This theorem states: The DP residue computation is correct.
-- Proving this validates the Rust implementation (residue_null_probability).

------------------------------------------------------------------------
-- Probability of divisibility (if uniform sampling from allowed digits)

P[n≡0] : (T : Template) → (m : ℕ) → ℚ
P[n≡0] T m = if zeroPresent then (count0 / totalPaths) else 0ℚ
  where
  open import Data.Rational using (ℚ; 0ℚ; _/_)
  postulate
    zeroPresent : Bool
    count0      : ℕ
    totalPaths  : ℕ
  -- Implementation sketch:
  --   • Track probabilities instead of just residue membership
  --   • dist[r] = probability that sum ≡ r (mod m)
  --   • Update: dist'[r'] = Σ_{d ∈ allowed} (1/|allowed|) · dist[(r' - d·w) mod m]

------------------------------------------------------------------------
-- Contrast: Palindrome constraint vs spacing-only

-- Extra constraint: force digit-value mirroring
record MirrorConstraint (T : Template) : Set where
  field
    evenLen  : Even (len T)
    mirrorEq : ∀ (ds : Vec ℕ (len T)) → DigitsRespect T ds
             → ∀ (i : Fin (len T)) → lookup ds i ≡ lookup ds (mirror i)

open import Data.Product using (∃-syntax)

Even : ℕ → Set
Even n = ∃ λ k → n ≡ 2 * k

postulate
  -- If we also demand value mirroring at even length → (b+1) trap
  PalMirrorImpliesDivBPlus1
    : ∀ (T : Template) → MirrorConstraint T
    → ∀ (ds : Vec ℕ (len T)) → DigitsRespect T ds
    → (base T + 1) ∣ eval (base T) ds

  -- Spacing-only (no mirroring) CAN avoid (b+1) trap
  SpacingOnlyCanAvoidBPlus1
    : ∀ (T : Template)
    → SpacingSymmetric (open? T)
    → NonDegenerate T
    → ∃ λ (ds : Vec ℕ (len T)) →
        DigitsRespect T ds × ((eval (base T) ds % (base T + 1)) ≢ 0)

------------------------------------------------------------------------
-- Concrete counterexamples (computational witnesses)

-- Example 1: Base 10, spacing [0][0][0][d][0][0][0]
-- Generated numbers: 1000, 2000, ..., 9000
-- None divisible by 11 = 10+1

example1-base10-template : Template
example1-base10-template = record
  { base = 10
  ; len = 7
  ; open? = λ i → toℕ i ≟ 3  -- Only position 3 is open
  ; allow = λ i → if toℕ i ≟ 3 then (1 ∷ 2 ∷ 3 ∷ 4 ∷ 5 ∷ 6 ∷ 7 ∷ 8 ∷ 9 ∷ []) else []
  ; noLead0 = true
  ; lastSet = nothing
  }

postulate
  example1-avoids-11
    : ∀ (d : ℕ) → 1 ≤ d → d ≤ 9
    → let n = d * (10 ^ 3)
      in (n % 11) ≢ 0

-- Example 2: Base 6, spacing [d₁][0][0][d₂]
-- Palindrome (d₁=d₂): 1001₆ = 217 → 217 % 7 = 0 ✓
-- Independent (d₁≠d₂): 1002₆ = 218 → 218 % 7 = 1 ✗

example2-base6-template : Template
example2-base6-template = record
  { base = 6
  ; len = 4
  ; open? = λ i → (toℕ i ≟ 0) ∨ (toℕ i ≟ 3)
  ; allow = λ i → if (toℕ i ≟ 0) ∨ (toℕ i ≟ 3)
                  then (1 ∷ 2 ∷ 3 ∷ 4 ∷ 5 ∷ [])
                  else []
  ; noLead0 = true
  ; lastSet = nothing
  }

postulate
  example2-independent-avoids-7
    : let n = 1 * 6^3 + 0 * 6^2 + 0 * 6^1 + 2 * 6^0  -- 1002₆ = 218
      in (n % 7) ≢ 0

  example2-palindrome-hits-7
    : let n = 1 * 6^3 + 0 * 6^2 + 0 * 6^1 + 1 * 6^0  -- 1001₆ = 217
      in (n % 7) ≡ 0

------------------------------------------------------------------------
-- Key insight: Flexible filtering via exponent engineering

postulate
  spacing-offers-flexible-filtering
    : ∀ (base targetModulus : ℕ)
    → base ≥ 2
    → targetModulus ≥ 2
    → ∃ λ (T : Template) →
        (SpacingSymmetric (open? T))
        × (¬ MirrorConstraint T)  -- Not palindromic
        × (P[n≡0] T targetModulus > P[n≡0]-uniform targetModulus)  -- Enhanced filtering
  where
  open import Data.Rational using (ℚ; _>_)
  postulate
    P[n≡0]-uniform : ℕ → ℚ
    -- Uniform expectation: 1/m for modulus m

------------------------------------------------------------------------
-- Midpoint-shift theorem: changing midpoint length shifts traps

postulate
  midpoint-length-shifts-traps
    : ∀ (T1 T2 : Template)
    → (sameBase : base T1 ≡ base T2)
    → (sameOpenPattern : ∀ i j → relativePosition i (len T1) ≡ relativePosition j (len T2)
                                → open? T1 i ≡ open? T2 j)
    → (diffLength : len T1 ≢ len T2)
    → ∃ λ m → P[n≡0] T1 m ≢ P[n≡0] T2 m
  where
  postulate
    relativePosition : ∀ {n} → Fin n → ℕ → ℕ  -- Relative position in layout

------------------------------------------------------------------------
-- GCD amplification: gcd(base, m) > 1 creates stronger traps

postulate
  gcd-amplifies-spacing-bias
    : ∀ (base modulus : ℕ)
    → (g : ℕ) → g ≡ ℕₚ.gcd base modulus
    → g > 1
    → ∃ λ (exponentPattern : List ℕ) →
        ∃ λ (bias : ℚ) →
          (bias > 0ℚ) × (bias ≢ 1/modulus)
  where
  open import Data.Rational using (ℚ; 0ℚ; _>_; _≢_; _/_)
  postulate _/_ : ℕ → ℕ → ℚ

------------------------------------------------------------------------
-- Connection to Rust implementation

-- The functions in tools/density-explorer/src/main.rs implement:
--
--   • residue_null_probability(pattern, modulus)
--       ↔ P[n≡0] (toTemplate pattern) modulus
--
--   • expected_density_local(pattern, tracked_primes)
--       = ∏_{p ∈ tracked} (1 - P[n≡0] pattern p) / ln(length)
--
-- The correctness theorem zero-in-Residues↔exists-assignment
-- validates the DP implementation.
--
-- The counterexamples (example1, example2) correspond to:
--   • base 10: midpoint=free:1, layers=3:0
--   • base 6:  midpoint=zeros:2, layers=0:1

------------------------------------------------------------------------
-- Future work: Complete proofs

-- To prove:
--   1. zero-in-Residues↔exists-assignment (DP correctness)
--   2. SpacingOnlyCanAvoidBPlus1 (explicit construction)
--   3. midpoint-length-shifts-traps (exponent analysis)
--   4. gcd-amplifies-spacing-bias (number theory)
--
-- To implement computationally:
--   1. mirror function (Fin arithmetic)
--   2. applyConstraints (filter/intersect logic)
--   3. allFins (enumerate Fin n)
--   4. P[n≡0] with probability tracking
