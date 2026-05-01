------------------------------------------------------------------------
-- Lagrange example shell: canonical connector case study
--
-- Strongest live signal:
-- 1. the repo has one canonical connector pair with two reported width-5
--    insertion cases in the source shell
-- 2. reflection and center-void questions remain useful structure around that
--    pair, but they are still example-level questions rather than settled
--    general theory
-- 3. the membrane-like second prime remains part of the story, but only as a
--    narrow connector enhancement shell
--
-- Arithmetic-first reading:
-- - the reported points below are connector hits first
-- - the reflection shell is a position-analysis helper, not a proof of a
--   general Lagrange law
------------------------------------------------------------------------

module LagrangePoints.Examples where

open import Data.Bool using (Bool; true)
open import Data.List using (List; []; _∷_)
open import Data.Nat using (ℕ; _∸_)
open import Data.Maybe.Base using (Maybe; just; nothing)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

open import Core.LagrangePoints using
  ( ConcatenatedStructureShell
  ; ConnectorHitShell
  ; LagrangePointShell
  ; MembraneConnectionShell
  ; canonical-example
  ; canonical-L1
  ; canonical-L2
  ; canonical-connector-hits
  ; canonical-points
  ; canonical-point-count
  ; canonical-membrane-connection
  )

------------------------------------------------------------------------
-- Canonical pair and buffer geometry
------------------------------------------------------------------------

buffer-length : ℕ
buffer-length = ConcatenatedStructureShell.buffer-length canonical-example

buffer-length-check : buffer-length ≡ 5
buffer-length-check = refl

buffer-reflect : ℕ -> ℕ
buffer-reflect pos = buffer-length ∸ pos ∸ 1

reflect-0 : buffer-reflect 0 ≡ 4
reflect-0 = refl

reflect-1 : buffer-reflect 1 ≡ 3
reflect-1 = refl

reflect-2 : buffer-reflect 2 ≡ 2
reflect-2 = refl

reflect-3 : buffer-reflect 3 ≡ 1
reflect-3 = refl

reflect-4 : buffer-reflect 4 ≡ 0
reflect-4 = refl

------------------------------------------------------------------------
-- Reported connector hits
------------------------------------------------------------------------

canonical-L1-position : LagrangePointShell.position canonical-L1 ≡ 1
canonical-L1-position = refl

canonical-L1-digit : LagrangePointShell.digit canonical-L1 ≡ 6
canonical-L1-digit = refl

canonical-L2-position : LagrangePointShell.position canonical-L2 ≡ 4
canonical-L2-position = refl

canonical-L2-digit : LagrangePointShell.digit canonical-L2 ≡ 6
canonical-L2-digit = refl

canonical-two-point-count : canonical-point-count ≡ 2
canonical-two-point-count = refl

------------------------------------------------------------------------
-- Position-status shell
------------------------------------------------------------------------

data PositionStatus : Set where
  reported-hit : ℕ -> PositionStatus
  center-void-candidate : PositionStatus
  unresolved : PositionStatus

record PositionShell : Set where
  field
    position : ℕ
    reflected-position : ℕ
    status : PositionStatus

position-0-shell : PositionShell
position-0-shell = record
  { position = 0
  ; reflected-position = buffer-reflect 0
  ; status = unresolved
  }

position-1-shell : PositionShell
position-1-shell = record
  { position = 1
  ; reflected-position = buffer-reflect 1
  ; status = reported-hit 6
  }

position-2-shell : PositionShell
position-2-shell = record
  { position = 2
  ; reflected-position = buffer-reflect 2
  ; status = center-void-candidate
  }

position-3-shell : PositionShell
position-3-shell = record
  { position = 3
  ; reflected-position = buffer-reflect 3
  ; status = unresolved
  }

position-4-shell : PositionShell
position-4-shell = record
  { position = 4
  ; reflected-position = buffer-reflect 4
  ; status = reported-hit 6
  }

all-position-shells : List PositionShell
all-position-shells =
  position-0-shell ∷
  position-1-shell ∷
  position-2-shell ∷
  position-3-shell ∷
  position-4-shell ∷
  []

------------------------------------------------------------------------
-- Canonical example summary
------------------------------------------------------------------------

record ExampleCaseShell : Set1 where
  field
    structure : ConcatenatedStructureShell
    reported-points : List ConnectorHitShell
    reported-count : ℕ
    membrane-connection : MembraneConnectionShell
    center-position : Maybe ℕ
    reflected-open-positions : List ℕ
    pairing-still-open : Bool

canonical-case : ExampleCaseShell
canonical-case = record
  { structure = canonical-example
  ; reported-points = canonical-connector-hits
  ; reported-count = canonical-point-count
  ; membrane-connection = canonical-membrane-connection
  ; center-position = just 2
  ; reflected-open-positions = 0 ∷ 3 ∷ []
  ; pairing-still-open = true
  }

------------------------------------------------------------------------
-- Open scan / theory shell
------------------------------------------------------------------------

postulate
  insertDigit : ConcatenatedStructureShell -> ℕ -> ℕ -> ℕ
  residueEquilibrium : ConcatenatedStructureShell -> ℕ -> ℕ -> Set
  reflectedHit : PositionShell -> Set
  centerVoid : ExampleCaseShell -> Set
  membraneEnhancement : ExampleCaseShell -> Set
  fullScan : ExampleCaseShell -> List PositionShell
