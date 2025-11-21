{-# OPTIONS --safe #-}

-- GlobalElbowFacts:
--   First small global facts about the currently known elbow frontier.
--
--   This module is intentionally tiny and concrete:
--     - It talks about the *current* CSV-generated elbow
--       (base 15, outer 13, inner 1, M:1→2, k:0→1).
--     - It packages that into named lemmas you can later
--       generalize, or quantify over once you have more
--       events and a roster (allElbowEvents, etc.).
--
--   It depends only on:
--     - Theorems.RationalStatistics   (ℚ + comparisons)
--     - Theorems.ElbowEvents          (ElbowConfig / ElbowEvent)
--     - Theorems.ElbowsFromCSV        (auto-generated from ridge_elbows.csv)

module Theorems.GlobalElbowFacts where

open import Data.Nat using (ℕ; zero; suc)
open import Data.Bool using (Bool; true; false)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

open import Theorems.RationalStatistics using (ℚ; _≤ℚ_)
open import Theorems.ElbowEvents
open import Theorems.ElbowsFromCSV

-- Bring record projections into scope
open ElbowConfig   using (base; outer; inner; M-from; M-to; k-from; k-to)
open ElbowEvidence using (cfg; density-from; density-to; M-step; k-increases; density-weakly-improves)

------------------------------------------------------------------------
-- 1. Designate the current frontier elbow from the CSV
------------------------------------------------------------------------

-- This name is exactly what generate_elbow_agda_from_csv.py produces
-- for the sample event in ridge_elbows.csv:  base=15, outer=13, inner=1,
-- M=1→2, k=0→1.
--
-- If later your CSV has many rows, you can keep this as the "flagship"
-- elbow and add roster-level facts on top.

frontierElbow : ElbowEvent
frontierElbow = elbow_b15_o13_i1_M1_to_M2_k0_to_k1

frontierCfg : ElbowConfig
frontierCfg = cfg frontierElbow

------------------------------------------------------------------------
-- 2. Structural facts: (base, outer, inner) and steps in M, k
------------------------------------------------------------------------

-- Base / outer / inner for the current elbow

frontier-base : base frontierCfg ≡ 15
frontier-base = refl

frontier-outer : outer frontierCfg ≡ 13
frontier-outer = refl

frontier-inner : inner frontierCfg ≡ 1
frontier-inner = refl

-- Middle-length (M) transition:
--   M-from = 1
--   M-to   = 2
--   and the evidence encodes M-to ≡ suc M-from (M-step = refl).

frontier-M-from : M-from frontierCfg ≡ 1
frontier-M-from = refl

frontier-M-to : M-to frontierCfg ≡ 2
frontier-M-to = refl

frontier-M-step-is-suc :
  M-step frontierElbow ≡ refl
frontier-M-step-is-suc = refl

-- Padding (k) transition:
--   k-from = 0
--   k-to   = 1
--   and the generator used the generic lemma k-step-suc 0 as the proof
--   that 0 < 1 (n < suc n).

frontier-k-from : k-from frontierCfg ≡ 0
frontier-k-from = refl

frontier-k-to : k-to frontierCfg ≡ 1
frontier-k-to = refl

frontier-k-increases :
  k-increases frontierElbow ≡ k-step-suc 0
frontier-k-increases = refl

------------------------------------------------------------------------
-- 3. Density behaviour: this elbow genuinely improves ρ
------------------------------------------------------------------------

-- We don't pin the densities to particular numerators/denominators
-- here (ElbowsFromCSV.agda already chose rationals like 1/7 and
-- 24/133 via Fraction.limit_denominator).
-- Instead we just expose what the evidence guarantees:
--
--   _≤ℚ_ (density-from e) (density-to e) ≡ true
--
-- i.e. the elbow is a genuine density improvement.

frontier-ρ-from : ℚ
frontier-ρ-from = density-from frontierElbow

frontier-ρ-to : ℚ
frontier-ρ-to = density-to frontierElbow

frontier-density-weakly-improves :
  _≤ℚ_ frontier-ρ-from frontier-ρ-to ≡ true
frontier-density-weakly-improves = density-weakly-improves frontierElbow

-- If you later extend RationalStatistics with a strict comparison
-- _<ℚ_, you can add:
--
--   currentElbow-density-strictly-improves :
--     _<ℚ_ (density-from currentElbow) (density-to currentElbow) ≡ true
--   currentElbow-density-strictly-improves = refl
--
-- once you pick a representation for ρ-from / ρ-to where that
-- definitional equality holds.

------------------------------------------------------------------------
-- 4. "Dataset-level" statements you can build on
------------------------------------------------------------------------

-- With just one configuration in ridge_statistics.txt, we know:
--
--   - There exists at least one positive elbow event:
--         frontierElbow : ElbowEvent
--     and it lives at (base,outer,inner) = (15,13,1) with M:1→2, k:0→1.
--
-- As you expand the scan and add a roster (e.g.
--   allElbowEvents : List ElbowEvent
-- in ElbowsFromCSV.agda), you can extend this module to prove
-- things like:
--
--   - totalElbows ≡ length allElbowEvents
--   - all current elbows have odd base
--   - there are currently no ContrarianElbowEvent instances
--
-- simply by pattern-matching on that list.
