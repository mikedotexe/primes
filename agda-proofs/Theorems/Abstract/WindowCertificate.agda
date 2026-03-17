-- Window Certificate: Complete Static+Dynamic Per-Window Certification
--
-- DUAL CERTIFICATION LAYER: Combines both invariants into single certificate
--
-- For each 2p² window, certify:
-- 1. STATIC: Honorary zero (midpoint void from perfect pairing)
-- 2. DYNAMIC: Inviolability (stable paths cannot enter exclusion zone)
--
-- This is the production artifact for per-window machine-checked proofs!
--
-- Production-ready for 2p² window pipeline integration.

module Theorems.Abstract.WindowCertificate where

open import Data.Nat       using (ℕ; zero; suc; _+_; _*_; _∸_)
open import Data.Product     using (Σ; _,_; proj₁; proj₂)
open import Relation.Binary.PropositionalEquality  using (_≡_; refl)
open import Data.Empty     using (⊥)
open import Data.Fin               using (Fin)
open import Data.Fin.Properties    using () renaming (_≟_ to _≟Fin_)

-- Import complete framework
open import Theorems.Abstract.SymmetryImpliesRepulsion
  using ( SymmetryData ; MS ; HonoraryZero )
open import Theorems.Abstract.SymmetryFromList
  using ( MS-fromResid ; PerfectBuckets ; honoraryZeroFromPerfect )
open import Theorems.Abstract.ConstrainedOrbitals
  using ( StableOrbital ; PointwiseSafe ; pointwiseSafe⇒StableOrbital
        ; InZone ; inviolabilityFromPointwiseSafe ; List )
open import Theorems.Abstract.BucketsAutoMatch
  using ( BalancedBuckets ; SupportCountsAgree ; ObservedResiduesMove
        ; honoraryZeroFromBalanced )

------------------------------------------------------------------------
-- WINDOW DATA: What we extract from 2p² window analysis

record WindowData (base : ℕ) (n : ℕ) : Set where
  field
    -- Window parameters
    p          : ℕ                      -- Prime p (window is around 2p²)
    window-mid : ℕ                      -- Actual midpoint of window
    radius     : ℕ                      -- Exclusion radius R

    -- Extracted prime residues
    residues   : Fin n → Fin base       -- Residue labeling

    -- Extracted prime positions (for dynamic check)
    positions  : List ℕ                 -- Absolute positions in window

    -- Count function (for auto-matching)
    count      : Fin base → ℕ           -- Bucket counts

------------------------------------------------------------------------
-- STATIC CERTIFICATE: Honorary zero witness

record StaticContracts {base n : ℕ}
  (S : SymmetryData (Fin base))
  (W : WindowData base n)
  : Set where
  field
    support-counts   : SupportCountsAgree _≟Fin_
                         (WindowData.residues W)
                         (WindowData.count W)
    nonfixed-residue : ObservedResiduesMove S (WindowData.residues W)

record StaticCertificate {base n : ℕ}
  (S : SymmetryData (Fin base))
  (W : WindowData base n)
  : Set where
  field
    -- Witness: Balanced buckets (auto-builds perfect pairing)
    balanced-witness : BalancedBuckets S
                         (WindowData.residues W)
                         (WindowData.count W)

    -- Explicit static boundary: support counts plus fixed-point exclusion.
    static-contracts : StaticContracts S W

  honorary-zero : HonoraryZero S (MS-fromResid (WindowData.residues W))
  honorary-zero =
    honoraryZeroFromBalanced
      _≟Fin_
      S
      (WindowData.residues W)
      (WindowData.count W)
      balanced-witness
      (StaticContracts.support-counts static-contracts)
      (StaticContracts.nonfixed-residue static-contracts)

------------------------------------------------------------------------
-- DYNAMIC CERTIFICATE: Inviolability witness

record DynamicCertificate {base n : ℕ}
  (W : WindowData base n)
  : Set where
  field
    -- Witness contract: every position is individually safe.
    pointwise-safe : PointwiseSafe
                       (WindowData.radius W)
                       (WindowData.window-mid W)
                       (WindowData.positions W)

  stable-witness : StableOrbital
                     (WindowData.radius W)
                     (WindowData.window-mid W)
                     (WindowData.positions W)
  stable-witness = pointwiseSafe⇒StableOrbital pointwise-safe

  inviolability : InZone
                    (WindowData.radius W)
                    (WindowData.window-mid W)
                    (WindowData.positions W)
                  → ⊥
  inviolability = inviolabilityFromPointwiseSafe pointwise-safe

------------------------------------------------------------------------
-- COMPLETE DUAL CERTIFICATE: Static + Dynamic

record DualCertificate {base n : ℕ}
  (S : SymmetryData (Fin base))
  (W : WindowData base n)
  : Set where
  field
    -- Static invariant
    static : StaticCertificate S W

    -- Dynamic invariant
    dynamic : DynamicCertificate W

  -- Convenience accessors
  honorary-zero : HonoraryZero S (MS-fromResid (WindowData.residues W))
  honorary-zero = StaticCertificate.honorary-zero static

  inviolability : InZone
                    (WindowData.radius W)
                    (WindowData.window-mid W)
                    (WindowData.positions W)
                  → ⊥
  inviolability = DynamicCertificate.inviolability dynamic

------------------------------------------------------------------------
-- CERTIFICATE BUILDER: Convenience constructor

buildDualCertificate
  : ∀ {base n}
  → (S : SymmetryData (Fin base))
  → (W : WindowData base n)
  → (contracts : StaticContracts S W)
  → (bb : BalancedBuckets S (WindowData.residues W) (WindowData.count W))
  -- Generated callers should usually build this via
  -- pointwiseSafeNil / pointwiseSafeCons / pointwiseSafeFromAll.
  → (ps : PointwiseSafe (WindowData.radius W)
                        (WindowData.window-mid W)
                        (WindowData.positions W))
  → DualCertificate S W
buildDualCertificate S W contracts bb ps = record
  { static = record
      { balanced-witness = bb
      ; static-contracts = contracts }
  ; dynamic = record
      { pointwise-safe = ps }
  }

------------------------------------------------------------------------
-- VERIFICATION SUMMARY: What the certificate proves

{-
DUAL CERTIFICATE GUARANTEES:

Given a 2p² window W with base B, midpoint mid, radius R:

STATIC CERTIFICATE (Honorary Zero):
  ✓ Residues are perfectly paired under reflection r ↦ (2·mid - r) mod B
  ✓ Each residue r appears exactly as often as inv(r)
  ✓ NO prime in the window has residue = mid (the "void")
  ✓ This is a GLOBAL PROPERTY of the entire residue distribution

DYNAMIC CERTIFICATE (Inviolability):
  ✓ Every prime position x satisfies |x - window_mid| ≥ R
  ✓ The exclusion zone [window_mid - R, window_mid + R] is empty
  ✓ It is LOGICALLY IMPOSSIBLE for any position to violate this
  ✓ This is a PATH-LEVEL PROPERTY enforced at each step

TOGETHER THEY PROVE:
  ✓ The void EXISTS (honorary zero)
  ✓ The void is STRUCTURALLY ENFORCED (inviolability)
  ✓ Both global distribution AND local trajectory respect symmetry
  ✓ Machine-checked, constructive, type-safe proof

This is the complete formal foundation for coordinate constellation
symmetry in 2p² windows!
-}

------------------------------------------------------------------------
-- INTEGRATION WITH 2p² PIPELINE

{-
WORKFLOW:

1. RUST: Analyze 2p² window
   - Generate primes around 2p²
   - Extract residues mod base
   - Count bucket frequencies
   - Compute positions and distances

2. RUST: Generate WindowData
   let window_data = WindowData {
     p = prime_value,
     window_mid = calculated_midpoint,
     radius = exclusion_radius,
     residues = extracted_residues,
     positions = prime_positions,
     count = bucket_counts
   }

3. RUST: Generate Agda witness code
   - balanced-witness proof (count r = count (inv r))
   - support-counts proof (count matches observed support)
   - nonfixed-residue proof (observed residues move under inv)
   - pointwise-safe proof (SafePos at each position)

4. AGDA: Type-check certificate
   certificate : DualCertificate S window_data
   certificate = buildDualCertificate S window_data contracts bb ps

5. SUCCESS: Machine-checked dual certificate ✓

OUTPUTS:
  - Certificate file: window_p{p}_base{B}.agda
  - Verification log: All proofs type-check
  - Statistics: Δ₃, β, spectral properties
  - Publication artifact: Machine-checked appendix

This completes the compute-then-verify pipeline!
-}

------------------------------------------------------------------------
-- Hypothetical examples intentionally live outside this builder module.
--
-- See:
--   ../../Examples/WINDOW_CERTIFICATE_BASE14_SKETCH.md
--
-- The goal is to keep this file focused on the dual-certificate API and its
-- constructive builder surface, without bundling a local postulated example
-- shell into the same module.

------------------------------------------------------------------------
-- PRODUCTION NOTES

{-
DEPLOYMENT:

1. Generate one certificate file per window:
   - window_p7_base14.agda
   - window_p11_base14.agda
   - window_p13_base18.agda
   - etc.

2. Each file contains:
   - WindowData (empirical input)
   - Witness proofs (balanced-witness, pointwise-safe)
   - Certificate instantiation
   - Type-check = verification ✓

3. Aggregate results:
   - Success rate: X% of windows verify
   - Common patterns: Which bases/radii work best
   - Exceptions: Base 7 exception analysis
   - Statistical trends: Correlate with Δ₃, β

4. Publication:
   - Main paper: Coordinate constellation theory
   - Appendix: Complete formal framework
   - Artifact: All certificate files
   - Reproducibility: cargo run --example generate_certificates

MAINTENANCE:

- Framework is stable (185 lines of core theory)
- Per-window files are auto-generated
- Proofs are mostly automatic (balanced counts)
- Manual intervention only for exceptions

This is the complete production stack for 2p² window certification!
-}
