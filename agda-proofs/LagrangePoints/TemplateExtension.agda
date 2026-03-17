------------------------------------------------------------------------
-- Lagrange template-extension shell: asymmetric membrane wrapper
--
-- Strongest live signal:
-- 1. the canonical connector can be described as an asymmetric extension of
--    the repo's membrane vocabulary
-- 2. the buffer itself carries a reflection structure even though the whole
--    concatenation is not palindromic
-- 3. the abstract symmetry -> honorary-zero theorem is relevant here as a
--    wrapper target, but the concrete pairing bridge remains open
------------------------------------------------------------------------

module LagrangePoints.TemplateExtension where

open import Data.Bool using (Bool; true; false)
open import Data.List using (List; []; _∷_)
open import Data.Maybe.Base using (Maybe; just; nothing)
open import Data.Nat using (ℕ; _∸_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

open import Core.LagrangePoints using
  ( ConcatenatedStructureShell
  ; canonical-example
  ; canonical-point-count
  )
open import LagrangePoints.Examples using (canonical-case)
open import LagrangePoints.ResidueField using (canonical-residue-case)
open import Theorems.Abstract.SymmetryImpliesRepulsion using
  ( SymmetryData
  ; MS
  ; Pairing
  ; HonoraryZero
  ; SymmetryImpliesRepulsion
  )

------------------------------------------------------------------------
-- Symmetric vs asymmetric shell
------------------------------------------------------------------------

record SymmetricTemplateShell : Set where
  field
    outer : ℕ
    inner : ℕ
    seed : ℕ
    perfectly-mirrored : Bool

record AsymmetricTemplateShell : Set where
  field
    structure : ConcatenatedStructureShell
    left-boundary : ℕ
    right-boundary : ℕ
    buffer-zeros : ℕ
    perfectly-mirrored : Bool

canonical-template : AsymmetricTemplateShell
canonical-template = record
  { structure = canonical-example
  ; left-boundary = ConcatenatedStructureShell.prime1 canonical-example
  ; right-boundary = ConcatenatedStructureShell.prime2 canonical-example
  ; buffer-zeros = ConcatenatedStructureShell.buffer-length canonical-example
  ; perfectly-mirrored = false
  }

------------------------------------------------------------------------
-- Buffer reflection shell
------------------------------------------------------------------------

buffer-reflection : AsymmetricTemplateShell -> ℕ -> ℕ
buffer-reflection template pos =
  AsymmetricTemplateShell.buffer-zeros template ∸ pos ∸ 1

canonical-buffer-check : AsymmetricTemplateShell.buffer-zeros canonical-template ≡ 5
canonical-buffer-check = refl

canonical-reflect-0 : buffer-reflection canonical-template 0 ≡ 4
canonical-reflect-0 = refl

canonical-reflect-1 : buffer-reflection canonical-template 1 ≡ 3
canonical-reflect-1 = refl

canonical-reflect-2 : buffer-reflection canonical-template 2 ≡ 2
canonical-reflect-2 = refl

canonical-reflect-4 : buffer-reflection canonical-template 4 ≡ 0
canonical-reflect-4 = refl

buffer-center : AsymmetricTemplateShell -> Maybe ℕ
buffer-center template = just (AsymmetricTemplateShell.buffer-zeros template ∸ 3)

canonical-center : buffer-center canonical-template ≡ just 2
canonical-center = refl

------------------------------------------------------------------------
-- Wrapper interpretation shell
------------------------------------------------------------------------

record TemplateExtensionCaseShell : Set1 where
  field
    asymmetric-template : AsymmetricTemplateShell
    example-shell-ready : Bool
    residue-shell-ready : Bool
    reported-hit-count : ℕ
    center-still-open : Bool

canonical-template-case : TemplateExtensionCaseShell
canonical-template-case = record
  { asymmetric-template = canonical-template
  ; example-shell-ready = true
  ; residue-shell-ready = true
  ; reported-hit-count = canonical-point-count
  ; center-still-open = true
  }

canonical-hit-count : canonical-point-count ≡ 2
canonical-hit-count = refl

------------------------------------------------------------------------
-- Open abstract symmetry bridge
------------------------------------------------------------------------

record TemplateTheoryShell : Set1 where
  field
    reflection-shape : Set
    abstract-symmetry-shape : Set
    honorary-zero-shape : Set
    membrane-enhancement-shape : Set

postulate
  bufferSymmetry : AsymmetricTemplateShell -> SymmetryData ℕ
  lagrangeMultiset : TemplateExtensionCaseShell -> MS ℕ
  lagrangePairing : (tc : TemplateExtensionCaseShell) ->
                    Pairing (bufferSymmetry (TemplateExtensionCaseShell.asymmetric-template tc))
                            (lagrangeMultiset tc)
  centerVoid : (tc : TemplateExtensionCaseShell) ->
               HonoraryZero (bufferSymmetry (TemplateExtensionCaseShell.asymmetric-template tc))
                            (lagrangeMultiset tc)
  membraneEnhancement : TemplateExtensionCaseShell -> Set
  template-theory : TemplateTheoryShell

abstract-center-void :
  (tc : TemplateExtensionCaseShell) ->
  Pairing (bufferSymmetry (TemplateExtensionCaseShell.asymmetric-template tc))
          (lagrangeMultiset tc) ->
  HonoraryZero (bufferSymmetry (TemplateExtensionCaseShell.asymmetric-template tc))
               (lagrangeMultiset tc)
abstract-center-void tc pairing =
  SymmetryImpliesRepulsion
    (bufferSymmetry (TemplateExtensionCaseShell.asymmetric-template tc))
    (lagrangeMultiset tc)
    pairing
