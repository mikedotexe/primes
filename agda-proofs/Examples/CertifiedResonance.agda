------------------------------------------------------------------------
-- Certified-resonance shell: generated wrapper over the base-6 certificate
--
-- Strongest live signal:
-- 1. the repo already has a fully constructive base-6 certificate in
--    `CertifiedResonanceComplete.agda`
-- 2. this module's remaining value is as a runtime-generation / export wrapper
--    around that live certificate, not as a second copy of the proof
-- 3. the open gap is the external code-generation bridge, not the concrete
--    honorary-zero proof for the base-6 case
------------------------------------------------------------------------

module Examples.CertifiedResonance where

open import Data.Nat using (ℕ)
open import Data.Bool using (Bool; true)
open import Data.Fin using (Fin)

open import Theorems.Abstract.SymmetryImpliesRepulsion using
  ( SymmetryData
  ; HonoraryZero
  )
open import Theorems.Abstract.SymmetryFromList using
  ( MS-fromResid
  ; PerfectBuckets
  )
open import Examples.CertifiedResonanceComplete as Complete using
  ( B
  ; Fin6
  ; midpoint-fin
  ; S
  ; res-list
  ; PBuckets
  ; CertifiedHonoraryZero
  )

------------------------------------------------------------------------
-- Runtime wrapper shell
------------------------------------------------------------------------

record RuntimeResidueShell : Set where
  field
    base : ℕ
    midpoint : Fin base
    occurrence-count : ℕ

base6-runtime : RuntimeResidueShell
base6-runtime = record
  { base = B
  ; midpoint = midpoint-fin
  ; occurrence-count = 4
  }

record GeneratedCertificateShell : Set where
  field
    runtime : RuntimeResidueShell
    symmetry-ready : Bool
    pairing-ready : Bool
    void-ready : Bool

base6-generated-shell : GeneratedCertificateShell
base6-generated-shell = record
  { runtime = base6-runtime
  ; symmetry-ready = true
  ; pairing-ready = true
  ; void-ready = true
  }

------------------------------------------------------------------------
-- Live certificate surface
------------------------------------------------------------------------

GeneratedSymmetry : SymmetryData Fin6
GeneratedSymmetry = S

GeneratedBuckets : PerfectBuckets GeneratedSymmetry res-list
GeneratedBuckets = PBuckets

GeneratedHonoraryZero : HonoraryZero GeneratedSymmetry (MS-fromResid res-list)
GeneratedHonoraryZero = CertifiedHonoraryZero

------------------------------------------------------------------------
-- Open external-generation bridge
------------------------------------------------------------------------

record GeneratedPipelineShell : Set1 where
  field
    rust-export-shape : Set
    codegen-shape : Set
    verification-log-shape : Set

postulate
  runtimeResidueExport : RuntimeResidueShell -> Set
  generateAgdaCertificate : RuntimeResidueShell -> Set
  verifyGeneratedArtifact : GeneratedCertificateShell -> Set
  generated-pipeline : GeneratedPipelineShell
