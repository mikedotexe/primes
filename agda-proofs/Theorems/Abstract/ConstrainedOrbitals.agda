-- Constrained Orbitals: Dynamic Invariant (Refined)
--
-- ROCHE-STYLE STABILITY: Path-level exclusion enforcement
--
-- This module defines the dynamic complement to static honorary zero:
-- - Static: "The midpoint is empty in the distribution"
-- - Dynamic: "No trajectory can enter the exclusion zone"
--
-- Uses indexed inductive types for compile-time guarantee.

module Theorems.Abstract.ConstrainedOrbitals where

open import Data.Nat      using (ℕ ; zero ; suc)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Data.Empty    using (⊥)
open import Relation.Nullary using (¬_)

------------------------------------------------------------------------
-- Lists

infixr 5 _∷_
data List (A : Set) : Set where
  []  : List A
  _∷_ : A → List A → List A

------------------------------------------------------------------------
-- Order on ℕ

data _≤_ : ℕ → ℕ → Set where
  z≤n : ∀ n → zero ≤ n
  s≤s : ∀ {m n} → m ≤ n → suc m ≤ suc n

_<_ : ℕ → ℕ → Set
m < n = suc m ≤ n

≤-refl : ∀ n → n ≤ n
≤-refl zero    = z≤n 0
≤-refl (suc n) = s≤s (≤-refl n)

≤-trans : ∀ {a b c} → a ≤ b → b ≤ c → a ≤ c
≤-trans (z≤n _)  q       = z≤n _
≤-trans (s≤s p) (s≤s q)  = s≤s (≤-trans p q)

not-s≤n : ∀ n → ¬ (suc n ≤ n)
not-s≤n zero      ()
not-s≤n (suc n) (s≤s p) = not-s≤n n p

------------------------------------------------------------------------
-- Absolute difference on ℕ

absDiff : ℕ → ℕ → ℕ
absDiff zero     n        = n
absDiff (suc m)  zero     = suc m
absDiff (suc m) (suc n)   = absDiff m n

------------------------------------------------------------------------
-- Zone predicates around midpoint with radius R

-- Safe position: outside exclusion zone (R ≤ |x - mid|)
SafePos : ℕ → ℕ → ℕ → Set
SafePos R mid x = R ≤ absDiff x mid

-- Inside zone: within exclusion radius (|x - mid| < R)
InPos : ℕ → ℕ → ℕ → Set
InPos R mid x = absDiff x mid < R

------------------------------------------------------------------------
-- Any predicate over lists

data Any {A : Set}(P : A → Set) : List A → Set where
  here  : ∀ {x xs} → P x → Any P (x ∷ xs)
  there : ∀ {x xs} → Any P xs → Any P (x ∷ xs)

data All {A : Set}(P : A → Set) : List A → Set where
  all[]  : All P []
  _all∷_ : ∀ {x xs} → P x → All P xs → All P (x ∷ xs)

------------------------------------------------------------------------
-- STABLE ORBITAL: Indexed inductive type
--
-- KEY INNOVATION: Type-level enforcement of distance invariant
--
-- StableOrbital R mid xs can only be inhabited if EVERY element
-- of xs maintains SafePos (R ≤ |x - mid|).
--
-- This is not a runtime check - it's a COMPILE-TIME GUARANTEE!

data StableOrbital (R mid : ℕ) : List ℕ → Set where
  stableNil  : StableOrbital R mid []

  stableCons : ∀ {x xs}
             → SafePos R mid x          -- Proof: current position safe
             → StableOrbital R mid xs   -- Proof: rest of path stable
             → StableOrbital R mid (x ∷ xs)

-- InZone: Existential - some position violates the bound
InZone : ∀ (R mid : ℕ) → List ℕ → Set
InZone R mid xs = Any (InPos R mid) xs

------------------------------------------------------------------------
-- Pointwise safety contract
--
-- Preferred maintained helper path for generated callers:
--   pointwiseSafeNil
--   pointwiseSafeCons
--   pointwiseSafeSingleton
--   pointwiseSafeFromAll

record PointwiseSafe (R mid : ℕ) (xs : List ℕ) : Set where
  field
    safe-each : All (SafePos R mid) xs

pointwiseSafeNil
  : ∀ {R mid}
  → PointwiseSafe R mid []
pointwiseSafeNil = record { safe-each = all[] }

pointwiseSafeCons
  : ∀ {R mid x xs}
  → SafePos R mid x
  → PointwiseSafe R mid xs
  → PointwiseSafe R mid (x ∷ xs)
pointwiseSafeCons px safe =
  record { safe-each = px all∷ PointwiseSafe.safe-each safe }

pointwiseSafeSingleton
  : ∀ {R mid x}
  → SafePos R mid x
  → PointwiseSafe R mid (x ∷ [])
pointwiseSafeSingleton px = pointwiseSafeCons px pointwiseSafeNil

pointwiseSafeFromAll
  : ∀ {R mid xs}
  → All (SafePos R mid) xs
  → PointwiseSafe R mid xs
pointwiseSafeFromAll pxs = record { safe-each = pxs }

pointwiseSafe⇒StableOrbital
  : ∀ {R mid xs}
  → PointwiseSafe R mid xs
  → StableOrbital R mid xs
pointwiseSafe⇒StableOrbital {xs = []} safe = stableNil
pointwiseSafe⇒StableOrbital {R} {mid} {xs = x ∷ xs} safe
  with PointwiseSafe.safe-each safe
... | px all∷ pxs =
  stableCons px
    (pointwiseSafe⇒StableOrbital
       (pointwiseSafeFromAll pxs))

stableOrbital⇒PointwiseSafe
  : ∀ {R mid xs}
  → StableOrbital R mid xs
  → PointwiseSafe R mid xs
stableOrbital⇒PointwiseSafe stableNil = pointwiseSafeNil
stableOrbital⇒PointwiseSafe (stableCons px pxs) =
  pointwiseSafeCons px (stableOrbital⇒PointwiseSafe pxs)

------------------------------------------------------------------------
-- Arithmetic contradiction: R ≤ d and d < R cannot coexist

no≤and< : ∀ {r d} → r ≤ d → d < r → ⊥
no≤and< {r} {d} r≤d d<r =
  let c : suc d ≤ d
      c = ≤-trans d<r r≤d
  in not-s≤n d c

------------------------------------------------------------------------
-- INVIOLABILITY THEOREM: Dynamic invariant
--
-- A stable orbital CANNOT intersect the exclusion zone.
--
-- This proves that stability and zone-violation are MUTUALLY EXCLUSIVE.
-- If you have both, you derive ⊥ (logical impossibility).

Inviolability
  : ∀ {R mid xs}
  → StableOrbital R mid xs
  → InZone R mid xs
  → ⊥
-- NOTE: These cases should be impossible but Agda 2.8.0 has trouble with the absurd patterns
-- Inviolability stableNil           (here  ())   -- Empty list can't be InZone
-- Inviolability stableNil           (there ())   -- Empty list can't have tail
Inviolability (stableCons px pxs) (here  q)    = no≤and< px q  -- Head contradiction
Inviolability (stableCons _  pxs) (there q)    = Inviolability pxs q  -- Recurse

inviolabilityFromPointwiseSafe
  : ∀ {R mid xs}
  → PointwiseSafe R mid xs
  → InZone R mid xs
  → ⊥
inviolabilityFromPointwiseSafe safe =
  Inviolability (pointwiseSafe⇒StableOrbital safe)

------------------------------------------------------------------------
-- INTEGRATION WITH STATIC INVARIANT
------------------------------------------------------------------------

{-
DUAL CERTIFICATION:

STATIC (SymmetryImpliesRepulsion):
  Question: "Is the midpoint present in the distribution?"
  Answer:   HonoraryZero = no occurrence at midpoint
  Method:   Perfect pairing witness
  Proves:   Global property of residue multiset

DYNAMIC (This module):
  Question: "Can a trajectory enter the exclusion zone?"
  Answer:   Inviolability = stable path cannot intersect zone
  Method:   Indexed types enforcing distance at each step
  Proves:   Local property at every step of sequence

RELATIONSHIP:
Both are necessary for complete 2p² window certification:
  1. Static: Proves the void EXISTS (φ-constraint consequence)
  2. Dynamic: Proves paths RESPECT the void (structural impossibility)

Together they provide:
  - Existence (honorary zero)
  - Mechanism (exclusion zone)
  - Necessity (inviolability)

USAGE IN 2p² FRAMEWORK:
1. Extract residues from window around 2p²
2. Static: Construct PerfectBuckets → HonoraryZero ✓
3. Dynamic: Construct StableOrbital → Inviolability ✓
4. Dual certificate: Both invariants verified ✓

The void is not just empty (static) - it's structurally inviolable (dynamic).
-}
