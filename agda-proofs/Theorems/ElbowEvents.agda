{-# OPTIONS --safe #-}

-- ElbowEvents: Formalizing "elbow" behaviour in membrane prime densities
-- (Self-contained version - HZBase bridge postulates are commented out for --safe compatibility)
--
-- INTENT:
--   - Give a small, clean Agda interface for "elbow events":
--       * The same (base, outer, inner) template
--       * A step in M (usually M → M + 1)
--       * The best padding k* jumps outward (k-from < k-to)
--       * The prime density at the new point is >= the old one
--
--   - Also support "contrarian elbows":
--       * k jumps outward (k-from < k-to)
--       * but density drops (to ≤ from)
--
--   - Keep this empirical / combinatorial:
--       * Densities are rational ℚ (num/den) from RationalStatistics.
--       * Comparisons use the Bool-valued predicate _≤ℚ_.
--
--   - Provide small lemmas linking elbows to the "honorary zero" base
--     notion, by showing that any elbow with an even base admits an
--     HZBase from Core.HonoraryZero.

module Theorems.ElbowEvents where

open import Data.Nat using (ℕ; zero; suc; _+_; _≤_; _<_; z≤n; s≤s)
open import Data.Bool using (Bool; true; false)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Data.Product using (_×_; _,_)

-- We reuse the constructive ℚ from Theorems.RationalStatistics:
--   record ℚ where constructor _/_; field num den : ℕ; ...
--   _≤ℚ_ : ℚ → ℚ → Bool   -- defines ≤ via cross-multiplication
open import Theorems.RationalStatistics using (ℚ; _/_; _≤ℚ_)

-- Honorary-zero base infrastructure
-- Note: This self-contained version omits the Core.HonoraryZero bridge.
-- For integration with the honorary zero framework, uncomment the import below.
-- open import Core.HonoraryZero using (HZBase)

------------------------------------------------------------------------
-- 0. Evenness predicate and bridge to HZBase
------------------------------------------------------------------------

-- A simple inductive "even" predicate on ℕ:
--
--   Even 0          holds
--   Even (suc (suc n)) holds if Even n holds
--
-- This is the standard 2-step pattern.
data Even : ℕ → Set where
  even-zero    : Even 0
  even-suc-suc : ∀ {n} → Even n → Even (suc (suc n))

-- We postulate a bridge from an even base to an honorary-zero base.
-- Conceptually, this belongs in Core.HonoraryZero; for now we keep it
-- here with a postulate so elbows can refer to HZBase without knowing
-- the internal details of that record.
--
-- Note: Commented out for self-contained version
-- postulate
--   hzBaseFromEven : (b : ℕ) → Even b → HZBase

------------------------------------------------------------------------
-- 1. Generic notion of an elbow configuration
------------------------------------------------------------------------

-- A purely combinatorial description of "where" the elbow happens.
--
-- Interpretation (all fields are ℕ parameters, no arithmetic yet):
--   base      : the positional base b
--   outer     : outer shell digit in the membrane
--   inner     : inner "shoulder" digit
--   M-from    : old seed length (middle size)
--   M-to      : new seed length (typically suc M-from)
--   k-from    : old optimal padding
--   k-to      : new optimal padding (typically > k-from)
--
-- This is intentionally agnostic about symmetry details,
-- but it is exactly the tuple you export in ridge_elbows.csv.
record ElbowConfig : Set where
  constructor mkElbowConfig
  field
    base   : ℕ
    outer  : ℕ
    inner  : ℕ
    M-from : ℕ
    M-to   : ℕ
    k-from : ℕ
    k-to   : ℕ

open ElbowConfig public

------------------------------------------------------------------------
-- 2. Evidence for a "positive" elbow: density improves
------------------------------------------------------------------------

-- An "elbow" is not just a tuple of parameters:
-- we also want to remember the RATIONAL densities at the ridge points,
-- and minimal monotonicity facts:
--   - M-to is the next step after M-from
--   - k-to is strictly larger than k-from
--   - density-to ≥ density-from (weak improvement in prime density)
--
-- We stay Bool-valued for comparisons (like RationalStatistics), and
-- use ≡ true as the propositional wrapper.
record ElbowEvidence : Set where
  constructor mkElbowEvidence
  field
    cfg : ElbowConfig

    -- Prime densities at the two ridge points:
    density-from : ℚ
    density-to   : ℚ

    -- M-step: usually M-to = suc M-from
    --
    -- NOTE: We encode this as a propositional equality, so the caller
    --       can choose whether they want strictly adjacent M (suc) or
    --       something looser. For the flagship elbow, we use suc.
    M-step : M-to cfg ≡ suc (M-from cfg)

    -- k-step: the ridge moves OUTWARD in padding
    --
    -- We use the standard < on ℕ (suc-based).
    k-increases : k-from cfg < k-to cfg

    -- Density monotonicity at the ridge points:
    --   density-from ≤ℚ density-to
    --
    -- We phrase it as "_≤ℚ_ density-from density-to ≡ true"
    -- to match the RationalStatistics style.
    density-weakly-improves :
      _≤ℚ_ density-from density-to ≡ true

open ElbowEvidence public

-- For convenience, a synonym: an "elbow event" is just some evidence.
ElbowEvent : Set
ElbowEvent = ElbowEvidence

------------------------------------------------------------------------
-- 3. Evidence for a "contrarian" elbow: k jumps but density drops
------------------------------------------------------------------------

-- A contrarian elbow has the same combinatorial structure as an elbow
-- (same ElbowConfig, same sort of k-step and M-step), but the density
-- goes DOWN instead of up:
--
--   - density-to ≤ℚ density-from
--
-- We use the same Bool-valued comparison and wrap it as ≡ true.
-- If you want strictness, you can later extend this with an explicit
-- inequality witness (e.g. density-from ≡ density-to → ⊥).
record ContrarianElbowEvidence : Set where
  constructor mkContrarianElbowEvidence
  field
    cfg : ElbowConfig

    density-from : ℚ
    density-to   : ℚ

    M-step : M-to cfg ≡ suc (M-from cfg)

    k-increases : k-from cfg < k-to cfg

    density-weakly-decreases :
      _≤ℚ_ density-to density-from ≡ true

open ContrarianElbowEvidence public

ContrarianElbowEvent : Set
ContrarianElbowEvent = ContrarianElbowEvidence

------------------------------------------------------------------------
-- 4. Example: Base 15, outer=13, inner=1 elbow (flagship event)
------------------------------------------------------------------------

-- This is the canonical positive elbow observed in the Rust sweeps:
--
--   base  = 15
--   outer = 13
--   inner = 1
--
--   M: 1 → 2
--   k: 0 → 1
--
--   ρ(M=1, k=0) ≈ 0.0714
--   ρ(M=2, k=1) ≈ 0.1143
--
-- Here we encode specific rational approximations that:
--   - Are compatible with the experimental values
--   - Make the inequality ρ-from ≤ℚ ρ-to definitional, so refl works.
--
-- Concretely we choose:
--   ρ-from = 1 / 14
--   ρ-to   = 4 / 35
--
-- since 1/14 ≈ 0.071428 … and 4/35 ≈ 0.114285 …,
-- which matches the documented densities to within rounding.
--
-- Check of the comparison:
--   ρ-from ≤ℚ ρ-to
--   ⇔ (1 * 35) ≤ (4 * 14)   by definition of ≤ℚ
--   ⇔ 35 ≤ 56               which is provably true in Data.Nat
--
-- Therefore "_≤ℚ_ (1/14) (4/35)" reduces to "true" and we can use refl
-- as the equality witness.

base15-13-1-elbow-config : ElbowConfig
base15-13-1-elbow-config = mkElbowConfig
  15    -- base
  13    -- outer
  1     -- inner
  1     -- M-from
  2     -- M-to
  0     -- k-from
  1     -- k-to

-- Rational densities at the two ridge points
ρ-base15-M1-k0 : ℚ
ρ-base15-M1-k0 = 1 / 14

ρ-base15-M2-k1 : ℚ
ρ-base15-M2-k1 = 4 / 35

-- Generic "successor" inequality: ∀ n, n < suc n
--
-- This covers all padding steps k: n → n+1, so we don't need to
-- hard-code individual proofs for each transition (0→1, 1→2, 2→3, ...).
--
-- Proof by induction:
--   Base case:    0 < 1       (by s≤s z≤n)
--   Inductive:    n < suc n → suc n < suc (suc n)  (by s≤s)
k-step-suc : ∀ (n : ℕ) → n < suc n
k-step-suc zero    = s≤s z≤n            -- 0 < 1
k-step-suc (suc n) = s≤s (k-step-suc n) -- lift: if n < suc n then suc n < suc (suc n)

-- Special case for 0 < 1 (used in examples below):
k-0<1 : 0 < 1
k-0<1 = k-step-suc 0

-- The flagship elbow event:
--
--   - Step in M:     1 → 2   (encoded by refl on suc 1)
--   - Step in k:     0 → 1   (k-0<1)
--   - ρ increases:   1/14 ≤ℚ 4/35  (definitionally true, refl)
base15-13-1-elbow : ElbowEvent
base15-13-1-elbow = mkElbowEvidence
  base15-13-1-elbow-config
  ρ-base15-M1-k0
  ρ-base15-M2-k1
  refl        -- M-step: 2 ≡ suc 1
  k-0<1       -- k-increases: 0 < 1
  refl        -- density-weakly-improves: _≤ℚ_ ρ-from ρ-to ≡ true

------------------------------------------------------------------------
-- 5. Example stub: Base 16, outer=5, inner=11 contrarian elbow
------------------------------------------------------------------------

-- Rust data shows a "contrarian" elbow in base 16 with outer=5, inner=11:
--
--   base  = 16
--   outer = 5
--   inner = 11
--
--   M: 1 → 2
--   k: 0 → 1
--
--   ρ(M=1, k=0) > ρ(M=2, k=1)
--
-- We do not hard-code the exact experimental densities here; instead we
-- choose simple rationals consistent with "density lowers":
--
--   ρ-from = 2 / 35
--   ρ-to   = 1 / 35
--
-- so that:
--   ρ-to ≤ℚ ρ-from   (definitionally true).
--
-- You can later refine ρ-from / ρ-to to match your CSV exactly, as long
-- as they satisfy the same inequality.
base16-5-11-contrarian-config : ElbowConfig
base16-5-11-contrarian-config = mkElbowConfig
  16    -- base
  5     -- outer
  11    -- inner
  1     -- M-from
  2     -- M-to
  0     -- k-from
  1     -- k-to

ρ-base16-M1-k0 : ℚ
ρ-base16-M1-k0 = 2 / 35

ρ-base16-M2-k1 : ℚ
ρ-base16-M2-k1 = 1 / 35

base16-5-11-contrarian : ContrarianElbowEvent
base16-5-11-contrarian = mkContrarianElbowEvidence
  base16-5-11-contrarian-config
  ρ-base16-M1-k0
  ρ-base16-M2-k1
  refl        -- M-step: 2 ≡ suc 1
  k-0<1       -- k-increases: 0 < 1
  refl        -- density-weakly-decreases: _≤ℚ_ ρ-to ρ-from ≡ true

------------------------------------------------------------------------
-- 6. Elbows live over honorary-zero bases
------------------------------------------------------------------------

-- Note: This section is commented out for self-contained --safe version
--
-- Now that we have Even and hzBaseFromEven, we can say:
--
--   "Any elbow with an even base admits an HZBase"
--
-- This is a very light connection to the honorary-zero layer; all the
-- geometry / midpoint details remain in Core.HonoraryZero. We just
-- show that the base field of an ElbowConfig can be turned into an
-- HZBase whenever it is even.

-- elbowHasHZBase : (e : ElbowConfig) → Even (base e) → HZBase
-- elbowHasHZBase e even-base = hzBaseFromEven (base e) even-base

-- contrarianElbowHasHZBase : (e : ElbowConfig) → Even (base e) → HZBase
-- contrarianElbowHasHZBase e even-base = hzBaseFromEven (base e) even-base

------------------------------------------------------------------------
-- 7. How to extend this module
------------------------------------------------------------------------
-- Suggested pattern for future elbows:
--
-- 1. Add a new ElbowConfig for the (base, outer, inner, M-from, M-to, k-from, k-to).
-- 2. Add ℚ constants for density-from and density-to, as num/den fractions.
-- 3. Check (on paper or via Rust) that:
--       _≤ℚ_ density-from density-to
--    (for positive elbows) or
--       _≤ℚ_ density-to density-from
--    (for contrarian elbows)
--    definitionally reduces to true (via the cross-multiplication rule).
-- 4. Provide either:
--       ElbowEvent             (positive elbow)
--    or ContrarianElbowEvent  (contrarian elbow)
--    with:
--       M-step            : either refl (if M-to = suc M-from) or a proof you want
--       k-increases       : proof of k-from < k-to (often s≤s ... chain)
--       density-weakly-improves / density-weakly-decreases : refl
--
-- That gives you a SMALL, TIGHT Agda summary of each elbow,
-- which you can later:
--   - connect to HonoraryZero / PhaseLocks for geometric interpretation
--   - reference from a higher-level "membrane ridge" theory
--   - use as input to future Orthogonality / Statistics modules.
