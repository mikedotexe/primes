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

open import Data.Nat      using (Nat ; zero ; suc)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Data.Empty    using (⊥)

------------------------------------------------------------------------
-- Lists

infixr 5 _∷_
data List (A : Set) : Set where
  []  : List A
  _∷_ : A → List A → List A

------------------------------------------------------------------------
-- Order on ℕ

data _≤_ : Nat → Nat → Set where
  z≤n : ∀ n → zero ≤ n
  s≤s : ∀ {m n} → m ≤ n → suc m ≤ suc n

_<_ : Nat → Nat → Set
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

¬_ : Set → Set
¬ P = P → ⊥

------------------------------------------------------------------------
-- Absolute difference on ℕ

absDiff : Nat → Nat → Nat
absDiff zero     n        = n
absDiff (suc m)  zero     = suc m
absDiff (suc m) (suc n)   = absDiff m n

------------------------------------------------------------------------
-- Zone predicates around midpoint with radius R

-- Safe position: outside exclusion zone (R ≤ |x - mid|)
SafePos : Nat → Nat → Nat → Set
SafePos R mid x = R ≤ absDiff x mid

-- Inside zone: within exclusion radius (|x - mid| < R)
InPos : Nat → Nat → Nat → Set
InPos R mid x = absDiff x mid < R

------------------------------------------------------------------------
-- Any predicate over lists

data Any {A : Set}(P : A → Set) : List A → Set where
  here  : ∀ {x xs} → P x → Any P (x ∷ xs)
  there : ∀ {x xs} → Any P xs → Any P (x ∷ xs)

------------------------------------------------------------------------
-- STABLE ORBITAL: Indexed inductive type
--
-- KEY INNOVATION: Type-level enforcement of distance invariant
--
-- StableOrbital R mid xs can only be inhabited if EVERY element
-- of xs maintains SafePos (R ≤ |x - mid|).
--
-- This is not a runtime check - it's a COMPILE-TIME GUARANTEE!

data StableOrbital (R mid : Nat) : List Nat → Set where
  stableNil  : StableOrbital R mid []

  stableCons : ∀ {x xs}
             → SafePos R mid x          -- Proof: current position safe
             → StableOrbital R mid xs   -- Proof: rest of path stable
             → StableOrbital R mid (x ∷ xs)

-- InZone: Existential - some position violates the bound
InZone : ∀ (R mid : Nat) → List Nat → Set
InZone R mid xs = Any (InPos R mid) xs

------------------------------------------------------------------------
-- Arithmetic contradiction: R ≤ d and d < R cannot coexist

no≤and< : ∀ {r d} → r ≤ d → d < r → ⊥
no≤and< r≤d d<r =
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
Inviolability stableNil           (here  ())   -- Empty list can't be InZone
Inviolability stableNil           (there ())   -- Empty list can't have tail
Inviolability (stableCons px pxs) (here  q)    = no≤and< px q  -- Head contradiction
Inviolability (stableCons _  pxs) (there q)    = Inviolability pxs q  -- Recurse

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
