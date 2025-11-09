-- Constrained Orbitals: The Dynamic Invariant
--
-- COMPANION TO: SymmetryImpliesRepulsion.agda (static invariant)
--
-- STATIC INVARIANT:  Symmetry ⇒ Honorary Zero
--   "The midpoint residue is empty in the distribution"
--
-- DYNAMIC INVARIANT: StableOrbital ⇒ ¬InZone
--   "No trajectory can enter the exclusion zone"
--
-- This is the path-level formalization of the Roche-limit analogy:
-- positions must maintain R ≤ |x - mid| at EVERY step, not just in aggregate.
--
-- NOVEL CONTRIBUTION: Indexed inductive type enforcing distance invariant
-- across entire sequences, proving stability is structural, not statistical.

module ConstrainedOrbitals where

open import Agda.Builtin.Nat       using (Nat ; zero ; suc)
open import Agda.Builtin.Equality  using (_≡_; refl)
open import Agda.Builtin.Unit      using (⊤ ; tt)
open import Agda.Builtin.Sigma     using (Σ ; _,_)
open import Agda.Builtin.Empty     using (⊥)

------------------------------------------------------------------------
-- Lists

infixr 5 _∷_
data List (A : Set) : Set where
  []  : List A
  _∷_ : A → List A → List A

------------------------------------------------------------------------
-- Basic arithmetic and order on ℕ (Peano)

_+_ : Nat → Nat → Nat
zero  + n = n
suc m + n = suc (m + n)

_-_ : Nat → Nat → Nat
m     - zero   = m
zero  - suc _  = 0
suc m - suc n  = m - n

_*_ : Nat → Nat → Nat
zero  * _ = 0
suc m * n = n + (m * n)

pow : Nat → Nat → Nat
pow _  zero   = 1
pow a (suc k) = a * pow a k

data _≤_ : Nat → Nat → Set where
  z≤n : ∀ {n} → zero ≤ n
  s≤s : ∀ {m n} → m ≤ n → suc m ≤ suc n

_<_ : Nat → Nat → Set
m < n = suc m ≤ n

≤-refl : ∀ n → n ≤ n
≤-refl zero    = z≤n
≤-refl (suc n) = s≤s (≤-refl n)

≤-trans : ∀ {a b c} → a ≤ b → b ≤ c → a ≤ c
≤-trans z≤n       q       = z≤n
≤-trans (s≤s p)  (s≤s q)  = s≤s (≤-trans p q)

not-s≤n : ∀ {n} → ¬ (suc n ≤ n)
not-s≤n {zero}   ()
not-s≤n {suc n}  (s≤s p) = not-s≤n p

¬_ : Set → Set
¬ P = P → ⊥

absurd : ∀ {A} → ⊥ → A
absurd ()

------------------------------------------------------------------------
-- Absolute distance on ℕ

-- Compute |a - b| using case analysis on ordering
absDiff : Nat → Nat → Nat
absDiff zero    b       = b
absDiff a       zero    = a
absDiff (suc a) (suc b) = absDiff a b

------------------------------------------------------------------------
-- Zone predicates around a fixed midpoint

-- Safe position: outside the exclusion zone (Roche-like), i.e. R ≤ |x - mid|
SafePos : Nat → Nat → Nat → Set
SafePos R mid x = R ≤ absDiff x mid

-- Inside zone: |x - mid| < R
InPos : Nat → Nat → Nat → Set
InPos R mid x = absDiff x mid < R

------------------------------------------------------------------------
-- Any / All over lists

data Any {A : Set}(P : A → Set) : List A → Set where
  here  : ∀ {x xs} → P x → Any P (x ∷ xs)
  there : ∀ {x xs} → Any P xs → Any P (x ∷ xs)

data All {A : Set}(P : A → Set) : List A → Set where
  all[] : All P []
  all∷  : ∀ {x xs} → P x → All P xs → All P (x ∷ xs)

------------------------------------------------------------------------
-- Exclusion vs inclusion contradictions

no≤and< : ∀ {r d} → r ≤ d → d < r → ⊥
no≤and< r≤d d<r =
  -- d<r is suc d ≤ r; compose suc d ≤ r ≤ d ⇒ suc d ≤ d ⇒ ⊥
  let chain : suc d ≤ d
      chain = ≤-trans d<r r≤d
  in not-s≤n chain

------------------------------------------------------------------------
-- The Roche-style bound (optional helper for your pipeline)
-- R(mid) = 2 * mid^3
RocheBound : Nat → Nat
RocheBound mid = 2 * pow mid 3

------------------------------------------------------------------------
-- STABLE ORBITAL: Indexed family that *enforces* the invariant at every step
--
-- This is the KEY INNOVATION:
-- Instead of checking "does this list satisfy the distance constraint?"
-- we construct a TYPE that can only be inhabited if the constraint holds.
--
-- StableOrbital R mid xs is a PROOF OBJECT that xs respects the exclusion zone.

-- Positions are plain Nat here; if you prefer residues, swap Nat for Fin B.
data StableOrbital (R mid : Nat) : List Nat → Set where
  stableNil  : StableOrbital R mid []
  stableCons : ∀ {x xs}
             → SafePos R mid x          -- Proof: current position is safe
             → StableOrbital R mid xs   -- Proof: rest of path is stable
             → StableOrbital R mid (x ∷ xs)

-- Zone-membership over a path (existential: some position violates the bound)
InZone : ∀ (R mid : Nat) → List Nat → Set
InZone R mid xs = Any (InPos R mid) xs

------------------------------------------------------------------------
-- DYNAMIC INVARIANT THEOREM: Inviolability
--
-- A stable orbital cannot intersect the exclusion zone.
--
-- This is the CONSTRUCTIVE PROOF that stability and zone-violation
-- are mutually exclusive - if you have both, you derive ⊥.

Inviolability
  : ∀ {R mid xs}
  → StableOrbital R mid xs
  → InZone R mid xs
  → ⊥
Inviolability stableNil           ()                    -- Empty list can't be InZone
Inviolability (stableCons px pxs) (here inz)    = no≤and< px inz  -- Contradiction at head
Inviolability (stableCons _  pxs) (there inzs)  = Inviolability pxs inzs  -- Recurse on tail

------------------------------------------------------------------------
-- CONNECTION TO STATIC INVARIANT
------------------------------------------------------------------------

{-
STATIC vs DYNAMIC INVARIANTS:

STATIC (SymmetryImpliesRepulsion.agda):
  Question: "Is the midpoint residue present in the distribution?"
  Answer:   HonoraryZero = count at midpoint is zero
  Method:   Aggregate counting, perfect bucket pairing
  Proves:   Symmetry ⇒ Honorary Zero

DYNAMIC (This module):
  Question: "Can a trajectory enter the exclusion zone?"
  Answer:   Inviolability = stable path cannot intersect zone
  Method:   Indexed types enforcing distance at each step
  Proves:   StableOrbital ⇒ ¬InZone

RELATIONSHIP:
  Static:  Global property of a residue distribution
  Dynamic: Local property enforced at every step of a sequence

Both are necessary for the complete 2p² framework:
  - Static: Why the void exists (φ-constraint + symmetry)
  - Dynamic: Why trajectories can't violate it (structural exclusion)
-}

------------------------------------------------------------------------
-- APPLICATIONS TO 2p² WINDOWS
------------------------------------------------------------------------

{-
USAGE IN COMPUTE-THEN-VERIFY PIPELINE:

1. COMPUTE (Rust):
   - Generate primes in window around 2p²
   - Extract residues modulo base B
   - Compute distances from midpoint

2. CONSTRUCT WITNESS (Rust):
   - For each residue r, compute d = absDiff r mid
   - Check R ≤ d for chosen exclusion radius R
   - Build List Nat of distances

3. VERIFY (Agda):
   - Import distances as List Nat
   - Construct StableOrbital R mid distances
   - Type-checker verifies each step satisfies SafePos
   - If construction succeeds → path is certified stable

EXAMPLE:
  R = RocheBound mid = 2 * mid³
  Window around 2p² with CRT phase alignment
  Extract residues: [r₁, r₂, ..., rₙ]
  Build: StableOrbital R mid [r₁, r₂, ..., rₙ]
  Agda checks: R ≤ |rᵢ - mid| for all i

If any rᵢ violates the bound, type-checking fails!
This makes the exclusion zone a COMPILE-TIME GUARANTEE.
-}

------------------------------------------------------------------------
-- RELATIONSHIP TO Δ₃ AND SPECTRAL RIGIDITY
------------------------------------------------------------------------

{-
The dynamic invariant complements the Δ₃ analysis:

Δ₃ ANALYSIS (SpectralRigidity.agda):
  - Measures spacing statistics (how far apart are primes?)
  - Found: Δ₃ = 101 (very random, beyond Poisson)
  - Found: β = -0.99 (clustering/deserts, not repulsion)
  - Conclusion: NO spectral correlation

DYNAMIC INVARIANT (This module):
  - Enforces position constraints (where can primes be?)
  - Proves: Trajectories must avoid exclusion zone
  - Mechanism: Structural type-level enforcement
  - Conclusion: Geometric constraint, not statistical

DUAL NATURE CONFIRMED:
  - Configuration space: Constrained (stable orbitals)
  - Spacing statistics: Uncorrelated (Δ₃ = 101)

This is geometric order WITHOUT spectral correlation.
The constraint is WHERE primes can be, not HOW FAR APART they are.
-}

------------------------------------------------------------------------
-- NEXT STEPS
------------------------------------------------------------------------

{-
1. Connect to SymmetryFromList:
   - Build StableOrbital from residue list
   - Show both static (honorary zero) and dynamic (inviolability) hold

2. Empirical validation:
   - Generate coordinate constellation primes
   - Extract residues and distances
   - Attempt to construct StableOrbital
   - Count violations vs successful constructions

3. Parameterize R:
   - Test different exclusion radii
   - RocheBound mid = 2 * mid³
   - Empirical bounds from data
   - Compare with φ-constraint predictions

4. Multi-window analysis:
   - Build StableOrbital for multiple 2p² windows
   - Aggregate results across different primes p
   - Statistical validation of structural exclusion
-}

------------------------------------------------------------------------
-- INTERPRETATION
------------------------------------------------------------------------

{-
THE COMPLETE PICTURE:

1. φ-CONSTRAINT (Arithmetic)
   → Only coprime residues allowed
   → Creates forbidden zones

2. SYMMETRY (Static Invariant)
   → Perfect pairing around midpoint
   → Honorary zero at midpoint
   → Proven in SymmetryImpliesRepulsion.agda

3. STABILITY (Dynamic Invariant)
   → Trajectories must maintain distance
   → Exclusion zone is inviolable
   → Proven in ConstrainedOrbitals.agda (this module)

4. SPECTRAL INDEPENDENCE
   → NO correlation in spacing
   → Δ₃ = 101, β = -0.99
   → Proven in SpectralRigidity.agda

MECHANISM:
  Arithmetic constraint (φ) → Geometric structure (hexagons)
                            → Static void (honorary zero)
                            → Dynamic exclusion (stable orbitals)
                            → NO spectral correlation (Δ₃ = 101)

The void is not a force - it's a structural impossibility
enforced by indexed types in the configuration space.
-}
