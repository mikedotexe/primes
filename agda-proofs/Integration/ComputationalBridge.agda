------------------------------------------------------------------------
-- Computational bridge shell: current Agda core -> external tooling
--
-- Strongest live signal:
-- 1. residue-fold / CRT computations are executable today and are worth
--    exposing as a maintained export surface
-- 2. phase-lock and Lagrange shells already provide concrete canonical
--    examples that downstream tooling can consume without overstating the
--    general theory
-- 3. discriminant observations and residue exports can already be combined
--    into tool-facing summaries even though the old full bridge remains open
------------------------------------------------------------------------

module Integration.ComputationalBridge where

open import Agda.Builtin.String using (String)
open import Data.Bool using (Bool; true; false)
open import Data.Integer using (ℤ)
open import Data.List using (List; []; _∷_)
open import Data.Maybe.Base using (Maybe; just; nothing)
open import Data.Nat using (ℕ)
open import Data.Product using (_×_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

open import Core.CRTVector using (P0viaL; CRT-ok?)
open import Core.Discriminant using
  ( Δ
  ; DiscriminantObservation
  ; base6-15-m2
  ; base6-51-m2
  ; base12-15-m1
  )
open import Core.LagrangePoints using
  ( ConcatenatedStructureShell
  ; LagrangePointShell
  ; canonical-example
  ; canonical-points
  ; canonical-point-count
  )
open import Core.PhaseLocks using
  ( PhaseLockShell
  ; base10
  ; base22
  ; base10-phase-lock
  ; base22-phase-lock
  ; midpoint
  )
open import Core.ResidueFold using (Slot; Pattern; FixedZero; Open)
open import Core.TwoPBase using
  ( TwoPBaseShell
  ; ResidueCountShell
  ; base10-residue-count
  ; base14-residue-count
  )

------------------------------------------------------------------------
-- Live residue / CRT exports
------------------------------------------------------------------------

record ResidueExportShell : Set where
  field
    base : ℕ
    residue-count : ResidueCountShell
    moduli : List ℕ
    slot-pattern : Pattern
    zero-frequencies : List (ℕ × ℕ)
    crt-consistent : Bool

base10-pattern : Pattern
base10-pattern =
  Open (1 ∷ 3 ∷ 7 ∷ 9 ∷ []) ∷
  FixedZero ∷
  Open (1 ∷ 3 ∷ 7 ∷ 9 ∷ []) ∷
  []

base14-pattern : Pattern
base14-pattern =
  Open (1 ∷ 3 ∷ 5 ∷ 9 ∷ 11 ∷ 13 ∷ []) ∷
  FixedZero ∷
  []

base10-moduli : List ℕ
base10-moduli = 3 ∷ 5 ∷ 7 ∷ []

base14-moduli : List ℕ
base14-moduli = 3 ∷ 7 ∷ []

base10-residue-export : ResidueExportShell
base10-residue-export = record
  { base = 10
  ; residue-count = base10-residue-count
  ; moduli = base10-moduli
  ; slot-pattern = base10-pattern
  ; zero-frequencies = P0viaL 10 base10-moduli base10-pattern
  ; crt-consistent = CRT-ok? 10 base10-moduli base10-pattern
  }

base14-residue-export : ResidueExportShell
base14-residue-export = record
  { base = 14
  ; residue-count = base14-residue-count
  ; moduli = base14-moduli
  ; slot-pattern = base14-pattern
  ; zero-frequencies = P0viaL 14 base14-moduli base14-pattern
  ; crt-consistent = CRT-ok? 14 base14-moduli base14-pattern
  }

------------------------------------------------------------------------
-- Phase-lock / Lagrange export shells
------------------------------------------------------------------------

record PhaseLockExportShell : Set where
  field
    base-shell : TwoPBaseShell
    midpoint-value : ℕ
    locks : List PhaseLockShell
    bridge-ready : Bool

base10-phase-export : PhaseLockExportShell
base10-phase-export = record
  { base-shell = base10
  ; midpoint-value = midpoint base10
  ; locks = base10-phase-lock ∷ []
  ; bridge-ready = true
  }

base22-phase-export : PhaseLockExportShell
base22-phase-export = record
  { base-shell = base22
  ; midpoint-value = midpoint base22
  ; locks = base22-phase-lock ∷ []
  ; bridge-ready = true
  }

base10-midpoint-check : midpoint base10 ≡ 5
base10-midpoint-check = refl

base22-midpoint-check : midpoint base22 ≡ 11
base22-midpoint-check = refl

record LagrangeExportShell : Set where
  field
    structure : ConcatenatedStructureShell
    insertion-points : List LagrangePointShell
    reported-point-count : ℕ
    bridge-ready : Bool

canonical-lagrange-export : LagrangeExportShell
canonical-lagrange-export = record
  { structure = canonical-example
  ; insertion-points = canonical-points
  ; reported-point-count = canonical-point-count
  ; bridge-ready = true
  }

------------------------------------------------------------------------
-- Discriminant / tool configuration shells
------------------------------------------------------------------------

record DiscriminantExportShell : Set where
  field
    outer : ℕ
    seed-length : ℕ
    delta : ℤ
    observation : DiscriminantObservation
    score-ready : Bool

base6-15-discriminant-export : DiscriminantExportShell
base6-15-discriminant-export = record
  { outer = 1
  ; seed-length = 2
  ; delta = Δ 1 5
  ; observation = base6-15-m2
  ; score-ready = true
  }

base6-51-discriminant-export : DiscriminantExportShell
base6-51-discriminant-export = record
  { outer = 5
  ; seed-length = 2
  ; delta = Δ 5 1
  ; observation = base6-51-m2
  ; score-ready = true
  }

base12-15-discriminant-export : DiscriminantExportShell
base12-15-discriminant-export = record
  { outer = 1
  ; seed-length = 1
  ; delta = Δ 1 5
  ; observation = base12-15-m1
  ; score-ready = true
  }

record ToolConfig : Set where
  field
    name : String
    residue-export : ResidueExportShell
    phase-export : Maybe PhaseLockExportShell
    lagrange-export : Maybe LagrangeExportShell
    discriminant-export : Maybe DiscriminantExportShell

base10-tool-config : ToolConfig
base10-tool-config = record
  { name = "base10-residue-and-phase"
  ; residue-export = base10-residue-export
  ; phase-export = just base10-phase-export
  ; lagrange-export = nothing
  ; discriminant-export = just base6-15-discriminant-export
  }

canonical-tool-config : ToolConfig
canonical-tool-config = record
  { name = "canonical-lagrange-bridge"
  ; residue-export = base14-residue-export
  ; phase-export = just base22-phase-export
  ; lagrange-export = just canonical-lagrange-export
  ; discriminant-export = just base6-51-discriminant-export
  }

record Benchmark : Set where
  field
    label : String
    config : ToolConfig
    expected-residue-slice : Bool
    expected-phase-export : Bool
    expected-lagrange-export : Bool

crt-benchmark : Benchmark
crt-benchmark = record
  { label = "crt-export-shell"
  ; config = base10-tool-config
  ; expected-residue-slice = true
  ; expected-phase-export = true
  ; expected-lagrange-export = false
  }

canonical-benchmark : Benchmark
canonical-benchmark = record
  { label = "canonical-bridge-shell"
  ; config = canonical-tool-config
  ; expected-residue-slice = true
  ; expected-phase-export = true
  ; expected-lagrange-export = true
  }

------------------------------------------------------------------------
-- Open bridge shell
------------------------------------------------------------------------

record ComputationalBridgeTheoryShell : Set1 where
  field
    residue-export-shape : Set
    phase-export-shape : Set
    lagrange-export-shape : Set
    discriminant-export-shape : Set
    rust-cli-shape : Set
    wasm-export-shape : Set

postulate
  exportResidueShell : ResidueExportShell -> Set
  exportPhaseLockShell : PhaseLockExportShell -> Set
  exportLagrangeShell : LagrangeExportShell -> Set
  exportDiscriminantShell : DiscriminantExportShell -> Set
  benchmarkAgainstRust : Benchmark -> Set
  bridgeToUnifiedCLI : ToolConfig -> Set
  bridgeToWasm : ToolConfig -> Set
  computational-bridge-theory : ComputationalBridgeTheoryShell
