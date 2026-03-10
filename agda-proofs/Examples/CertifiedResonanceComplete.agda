-- Certified Resonance: Complete Base 6 Concrete Example
--
-- CONCRETE INSTANTIATION: Fully verified Base 6 resonance with all proofs
--
-- This module demonstrates the complete certification pipeline:
-- 1. Define concrete base (B=6) with explicit involution
-- 2. Provide concrete residue list {1, 5, 2, 4}
-- 3. Construct pairing witness with all proofs filled in
-- 4. Get HonoraryZero certificate (machine-checked!)
--
-- This closes the loop: theory -> pairing -> void
-- Ready for 2p^2 window verification pipeline.
--
-- STATUS: All proofs machine-checked. No postulates.
-- (Previous version used postulates due to #_ operator not computing in
-- pattern matching. Fixed March 2026 by using explicit fzero/fsuc patterns.)

module Examples.CertifiedResonanceComplete where

open import Data.Product     using (Σ; _,_)
open import Relation.Binary.PropositionalEquality  using (_≡_; refl)
open import Data.Empty     using (⊥)
open import Data.Nat       using (ℕ; zero; suc)
open import Data.Fin       using (Fin) renaming (zero to fzero; suc to fsuc)

open import Theorems.Abstract.SymmetryImpliesRepulsion
  using ( SymmetryData
        ; HonoraryZero
        )
open import Theorems.Abstract.SymmetryFromList
  using ( MS-fromResid
        ; PerfectBuckets
        ; honoraryZeroFromPerfect
        )

_≢_ : ∀ {A : Set} → A → A → Set
x ≢ y = (x ≡ y) → ⊥

------------------------------------------------------------------------
-- CONCRETE BASE: B = 6

B : ℕ
B = 6

Fin6 = Fin B

-- Canonical inhabitants of Fin 6 (using explicit fzero/fsuc for computation)
f0 : Fin6 ; f0 = fzero
f1 : Fin6 ; f1 = fsuc fzero
f2 : Fin6 ; f2 = fsuc (fsuc fzero)
f3 : Fin6 ; f3 = fsuc (fsuc (fsuc fzero))
f4 : Fin6 ; f4 = fsuc (fsuc (fsuc (fsuc fzero)))
f5 : Fin6 ; f5 = fsuc (fsuc (fsuc (fsuc (fsuc fzero))))

------------------------------------------------------------------------
-- MIDPOINT AND REFLECTION INVOLUTION (mid = 3)
--
-- The reflection: r |-> (2*3 - r) mod 6 = (6 - r) mod 6
-- Explicit computation for each residue:
--   inv(0) = 0  (6-0 = 6 = 0 mod 6)
--   inv(1) = 5  (6-1 = 5)
--   inv(2) = 4  (6-2 = 4)
--   inv(3) = 3  (6-3 = 3)  <- midpoint fixed
--   inv(4) = 2  (6-4 = 2)
--   inv(5) = 1  (6-5 = 1)

midpoint-fin : Fin6
midpoint-fin = f3

inv-fn : Fin6 → Fin6
inv-fn fzero                                         = f0
inv-fn (fsuc fzero)                                  = f5
inv-fn (fsuc (fsuc fzero))                           = f4
inv-fn (fsuc (fsuc (fsuc fzero)))                    = f3
inv-fn (fsuc (fsuc (fsuc (fsuc fzero))))             = f2
inv-fn (fsuc (fsuc (fsuc (fsuc (fsuc fzero)))))      = f1

-- PROOF: inv is involutive (inv(inv(r)) = r)
-- Proven by case analysis (6 cases, all reduce to refl)
inv-involutive-proof : ∀ r → inv-fn (inv-fn r) ≡ r
inv-involutive-proof fzero                                         = refl
inv-involutive-proof (fsuc fzero)                                  = refl
inv-involutive-proof (fsuc (fsuc fzero))                           = refl
inv-involutive-proof (fsuc (fsuc (fsuc fzero)))                    = refl
inv-involutive-proof (fsuc (fsuc (fsuc (fsuc fzero))))             = refl
inv-involutive-proof (fsuc (fsuc (fsuc (fsuc (fsuc fzero)))))      = refl

-- PROOF: midpoint is fixed by involution
inv-mid-proof : inv-fn midpoint-fin ≡ midpoint-fin
inv-mid-proof = refl

-- Construct SymmetryData for Base 6
S : SymmetryData Fin6
S = record
  { mid            = midpoint-fin
  ; inv            = inv-fn
  ; inv-involutive = inv-involutive-proof
  ; inv-mid        = inv-mid-proof
  }

------------------------------------------------------------------------
-- CONCRETE SAMPLE: n = 4, residues {1, 5, 2, 4} (perfectly paired)
--
-- This represents 4 occurrences with residues:
--   Occurrence 0 -> residue 1
--   Occurrence 1 -> residue 5
--   Occurrence 2 -> residue 2
--   Occurrence 3 -> residue 4
--
-- Perfect pairing:
--   1 <-> 5  (inv(1) = 5, inv(5) = 1)
--   2 <-> 4  (inv(2) = 4, inv(4) = 2)
--
-- Notice: residue 3 (midpoint) is ABSENT!

n : ℕ
n = 4

Fin4 = Fin n

-- Canonical inhabitants of Fin 4
i0 : Fin4 ; i0 = fzero
i1 : Fin4 ; i1 = fsuc fzero
i2 : Fin4 ; i2 = fsuc (fsuc fzero)
i3 : Fin4 ; i3 = fsuc (fsuc (fsuc fzero))

-- Residue list: Fin 4 -> Fin 6
--   0 |-> 1, 1 |-> 5, 2 |-> 2, 3 |-> 4
res-list : Fin4 → Fin6
res-list fzero                          = f1
res-list (fsuc fzero)                   = f5
res-list (fsuc (fsuc fzero))            = f2
res-list (fsuc (fsuc (fsuc fzero)))     = f4

------------------------------------------------------------------------
-- PAIRING FUNCTION: mate : Fin 4 -> Fin 4
--
-- Pairs the occurrence indices:
--   0 <-> 1  (because res(0)=1, res(1)=5, inv(1)=5)
--   2 <-> 3  (because res(2)=2, res(3)=4, inv(2)=4)

mate-fn : Fin4 → Fin4
mate-fn fzero                          = i1
mate-fn (fsuc fzero)                   = i0
mate-fn (fsuc (fsuc fzero))            = i3
mate-fn (fsuc (fsuc (fsuc fzero)))     = i2

------------------------------------------------------------------------
-- PERFECT BUCKETS WITNESS PROOFS
--
-- These are the KEY VERIFICATION OBLIGATIONS that certify the pairing!
-- All proven by exhaustive case analysis with refl or absurd patterns.

-- PROOF 1: mate is involutive (mate(mate(i)) = i)
involutive-mate : ∀ i → mate-fn (mate-fn i) ≡ i
involutive-mate fzero                          = refl
involutive-mate (fsuc fzero)                   = refl
involutive-mate (fsuc (fsuc fzero))            = refl
involutive-mate (fsuc (fsuc (fsuc fzero)))     = refl

-- PROOF 2: mate has no fixed points (mate(i) /= i)
no-fixed-mate : ∀ i → mate-fn i ≢ i
no-fixed-mate fzero                          ()
no-fixed-mate (fsuc fzero)                   ()
no-fixed-mate (fsuc (fsuc fzero))            ()
no-fixed-mate (fsuc (fsuc (fsuc fzero)))     ()

-- PROOF 3: equivariant (inv(res(i)) = res(mate(i)))
-- This is the CORE GEOMETRIC PROPERTY!
equivariant-res : ∀ i → inv-fn (res-list i) ≡ res-list (mate-fn i)
equivariant-res fzero                          = refl
equivariant-res (fsuc fzero)                   = refl
equivariant-res (fsuc (fsuc fzero))            = refl
equivariant-res (fsuc (fsuc (fsuc fzero)))     = refl

-- PROOF 4: residues are distinct (res(mate(i)) /= res(i))
residue-distinct : ∀ i → res-list (mate-fn i) ≢ res-list i
residue-distinct fzero                          ()
residue-distinct (fsuc fzero)                   ()
residue-distinct (fsuc (fsuc fzero))            ()
residue-distinct (fsuc (fsuc (fsuc fzero)))     ()

------------------------------------------------------------------------
-- CONSTRUCT PERFECT BUCKETS WITNESS

PBuckets : PerfectBuckets S res-list
PBuckets = record
  { mate             = mate-fn
  ; involutive       = involutive-mate
  ; no-fixed         = no-fixed-mate
  ; equivariant      = equivariant-res
  ; residue-distinct = residue-distinct
  }

------------------------------------------------------------------------
-- FINAL CERTIFICATE: HonoraryZero
--
-- The mere existence of PBuckets (which we constructed above)
-- FORCES the conclusion that midpoint residue (3) is uninhabited!
--
-- This is a MACHINE-CHECKED PROOF, not a statistical observation!

CertifiedHonoraryZero : HonoraryZero S (MS-fromResid res-list)
CertifiedHonoraryZero = honoraryZeroFromPerfect S res-list PBuckets

------------------------------------------------------------------------
-- VERIFICATION STATUS
------------------------------------------------------------------------

{-
WHAT WE JUST PROVED:

Given:
  - Base B = 6, midpoint = 3
  - Residue list: {1, 5, 2, 4}
  - Pairing: 1<->5, 2<->4

Verified (all by refl or absurd pattern -- no postulates):
  - inv(1) = 5
  - inv(5) = 1
  - inv(2) = 4
  - inv(4) = 2
  - All residues distinct from their mates
  - All pairing properties hold

Concluded:
  - HonoraryZero: NO occurrence can have residue 3
  - Type-checked: Agda verified the proof!
  - Machine-checked appendix for publication

HISTORY:
  - Original version used postulates because #_ operator (Fin literal
    shorthand) did not compute in pattern matching.
  - March 2026: Replaced #_ with explicit fzero/fsuc patterns.
    All 6 postulates became constructive proofs (refl or absurd).
    Module now passes with --safe flag.
-}
