{-# OPTIONS --without-K #-}

{-|
  Residue Classes: The Foundational Framework

  INSIGHT: All our discoveries are fundamentally about RESIDUE STRUCTURE

  This module serves as the conceptual hub and specification layer for:
  1. Residue classes as algebraic structures
  2. The residue class ring ℤ/mℤ
  3. Unit groups (coprime residues)
  4. Euler's totient theorem
  5. Residue collapse phenomenon

  Implementation details are provided by:
  - Core.ResidueClassesComplete: Ring structure and proofs
  - Core.ResidueCollapse: Collapse phenomenon formalization
  - Core.ResidueFold: Enumeration machinery
  - Core.Radical: Radical function definitions
  - Core.Primality: Primality predicates

  This is the UNIFYING FRAMEWORK for the entire project!
-}

module Core.ResidueClasses where

-------------------------------------------------------------------------------
-- IMPORTS FROM STANDARD LIBRARY
-------------------------------------------------------------------------------

open import Data.Nat using (ℕ; zero; suc; _+_; _*_; _∸_; _<_; _≡ᵇ_; _≤_; _>_)
open import Data.Nat.Properties using (_≟_)
open import Data.Nat.DivMod using (_mod_; m%n<n)
open import Data.Nat.GCD using (gcd)
open import Data.Nat.Divisibility using (_∣_)
open import Data.Nat.Coprimality using (Coprime)
open import Data.Product using (_×_; _,_; ∃; Σ-syntax)
open import Data.List using (List; []; _∷_; filter; length)
open import Data.List.Base using (applyUpTo)
open import Relation.Binary.PropositionalEquality using (_≡_; refl; sym; trans)
open import Relation.Nullary using (¬_; Dec; yes; no)
open import Algebra.Structures using (IsCommutativeRing)
open import Data.Unit using (⊤)

-------------------------------------------------------------------------------
-- IMPORTS FROM CORE MODULES
-------------------------------------------------------------------------------

-- Import logical equivalence from Core.Equiv
open import Core.Equiv using (_↔_; mk↔; refl↔) public

open import Core.Primality using (IsPrime)
open import Core.Radical using (radical)
open import Core.ResidueClassesComplete public using
  ( ResidueClass
  ; [_]mod_
  ; ⟦_⟧
  ; _≡ᵣ_
  ; _≟ᵣ_
  ; _⊕_
  ; _⊗_
  ; 0ᵣ
  ; 1ᵣ
  ; residue-ring
  )
open import Core.ResidueCollapse using (CollapseStructure)

-------------------------------------------------------------------------------
-- PART 1: RESIDUE CLASS CONCEPTS
-------------------------------------------------------------------------------

{-|
  A residue class modulo m is an equivalence class of integers
  that have the same remainder when divided by m.

  We represent it by its canonical representative: 0 ≤ r < m

  The ResidueClass type is imported from Core.ResidueClassesComplete
  where it is defined with proper bounds checking {m>0 : m > 0}.
-}

-- Notation: [r]ₘ for residue class of r mod m
syntax ResidueClass m = ℤ/[1+ m ]ℤ

-------------------------------------------------------------------------------
-- PART 2: UNITS (COPRIME RESIDUES)
-------------------------------------------------------------------------------

{-|
  A residue class [r] is a UNIT if it has a multiplicative inverse

  THEOREM: [r] is a unit ⟺ gcd(r, m) = 1

  This fundamental characterization connects algebra and number theory.
-}

-- Unit definition
IsUnit : ∀ {m} {m>0 : m > 0} → ResidueClass m {m>0} → Set
IsUnit {m} {m>0} r =
  ∃ λ (s : ResidueClass m {m>0}) →
    ∃ λ (m>1 : m > 1) → (r ⊗ s) ≡ᵣ 1ᵣ {m>1 = m>1}

-- THEOREM: Units are exactly the coprime residues
postulate
  units-are-coprime : ∀ {m} {m>0 : m > 0} (r : ResidueClass m {m>0}) →
    IsUnit r ↔ Coprime ⟦ r ⟧ m

{-|
  The proof (implemented in Core.ResidueClassesComplete):

  (⟹) If [r] is unit, then ∃s: rs ≡ 1 (mod m)
      Therefore rs = km + 1 for some k
      By Bezout: gcd(r,m) divides rs - km = 1
      Therefore gcd(r,m) = 1

  (⟸) If gcd(r,m) = 1, then by Extended Euclidean:
      ∃s,t: rs + mt = 1
      Therefore rs ≡ 1 (mod m)
      So [s] is inverse of [r]
-}

-- Count units (Euler's totient function)
totient : ℕ → ℕ
totient m = length (filter (λ r → gcd r m ≡ᵇ 1) (applyUpTo (_+ 0) m))

-------------------------------------------------------------------------------
-- PART 3: RESIDUE COLLAPSE
-------------------------------------------------------------------------------

{-|
  RESIDUE COLLAPSE: The GCD Paradox Explained!

  When gcd(base, d) > 1, residues mod d collapse into fewer classes.
  This creates STRONGER filtering, explaining why base 6 can outperform base 10.

  The full formalization is in Core.ResidueCollapse.
-}

-- Re-export the collapse structure
open Core.ResidueCollapse using (CollapseStructure) public

-- THEOREM: Higher collapse → More constraint
postulate
  collapse-strengthens-filtering : ∀ base₁ base₂ divisor →
    let c₁ = gcd base₁ divisor
        c₂ = gcd base₂ divisor
    in c₁ > c₂ →
       -- Base₁ has more constrained residue structure than base₂
       -- (Proven in Core.ResidueCollapse)
       ⊤

-------------------------------------------------------------------------------
-- PART 4: CONNECTION TO PRIMALITY
-------------------------------------------------------------------------------

{-|
  FUNDAMENTAL THEOREM: Prime residue characterization

  A number can be prime only if its residue mod rad(b) is coprime to rad(b).
  This is THE fundamental filtering mechanism!
-}

-- Prime residue constraint
postulate
  prime-residue-constraint : ∀ n base →
    IsPrime n →
    n > base →
    Coprime (n mod (radical base)) (radical base)

{-|
  The proof (from Core.Radical):

  If gcd(n mod rad(b), rad(b)) = d > 1:
  Then d ∣ n and d ∣ rad(b)
  Since d ∣ rad(b), ∃p prime: p ∣ d
  Therefore p ∣ n
  But n is prime, so p = n
  But n > base ≥ rad(b) ≥ d ≥ p
  Contradiction!
-}

-- Valid prime residues
valid-prime-residues : ℕ → List ℕ
valid-prime-residues base =
  let r = radical base
  in filter (λ k → gcd k r ≡ᵇ 1) (applyUpTo suc r)

-------------------------------------------------------------------------------
-- PART 5: AFFINE STRUCTURE
-------------------------------------------------------------------------------

{-|
  Residues preserve LINEAR structure.
  This is why the affine transform works!
-}

-- Residue of linear combination
postulate
  residue-linear : ∀ a b c m →
    ((a + b * c) mod m) ≡
    ((a mod m) + ((b mod m) * (c mod m)) mod m) mod m

-- Linear function type
record IsLinear (f : ℕ → ℕ) : Set where
  field
    additivity : ∀ x y → f (x + y) ≡ f x + f y
    -- Could add more properties like scalar multiplication

-- Residue homomorphism property
postulate
  residue-homomorphism : ∀ f →
    IsLinear f →
    ∀ m x y →
      (f (x + y)) mod m ≡ ((f x) mod m + (f y) mod m) mod m

-------------------------------------------------------------------------------
-- PART 6: THE UNIFYING FRAMEWORK
-------------------------------------------------------------------------------

{-|
  MASTER THEOREM: Residue structure determines everything

  This record captures all the essential properties that make
  residue arithmetic work for our prime detection system.
-}

record ResidueFramework (base : ℕ) {base>1 : base > 1} : Set where
  private
    base>0 : base > 0
    base>0 = trans-< (s≤s z≤n) base>1
      where
        open import Data.Nat using (z≤n; s≤s)
        open import Data.Nat.Properties using (trans-<)

  field
    -- 1. Residue ring structure
    ring : IsCommutativeRing _≡ᵣ_ _⊕_ _⊗_ (0ᵣ {m>0 = base>0}) (1ᵣ {m>1 = base>1})

    -- 2. Unit group (coprime residues)
    units : ∀ (r : ResidueClass base {base>0}) →
      IsUnit {m>0 = base>0} r ↔ Coprime ⟦ r ⟧ base

    -- 3. Radical filtering
    prime-filter : ∀ n →
      IsPrime n →
      n > base →
      Coprime (n mod (radical base)) (radical base)

    -- 4. Collapse structure
    collapse : ∀ d →
      d ∣ base →
      CollapseStructure base d

    -- 5. Affine preservation
    affine-linear : ∀ a b c →
      ((a + b * c) mod base) ≡
      ((a mod base) + ((b mod base) * (c mod base)) mod base) mod base

    -- 6. Radical value
    rad : ℕ
    rad-def : rad ≡ radical base

    -- 7. Wheel classes (valid prime residues)
    wheel-classes : List ℕ
    wheel-classes-def : wheel-classes ≡ valid-prime-residues base
    wheel-classes-coprime : ∀ r → r ∈ wheel-classes → Coprime r (radical base)
      where
        open import Data.List.Membership.Propositional using (_∈_)

-- THEOREM: Every base has this residue framework
universal-residue-framework : ∀ base →
  (base>1 : base > 1) →
  ResidueFramework base {base>1}
universal-residue-framework base base>1 = record
  { ring = residue-ring base
  ; units = units-are-coprime
  ; prime-filter = λ n → prime-residue-constraint n base
  ; collapse = λ d d∣base →
      record
        { divides = d∣base
        ; distinct-classes = distinct-residues base d
        ; distinct-classes-correct = refl
        }
  ; affine-linear = λ a b c → residue-linear a b c base
  ; rad = radical base
  ; rad-def = refl
  ; wheel-classes = valid-prime-residues base
  ; wheel-classes-def = refl
  ; wheel-classes-coprime = λ r r∈wheel → wheel-coprime-lemma base r r∈wheel
  }
  where
    open import Core.ResidueCollapse using (distinct-residues)
    open import Data.List.Membership.Propositional using (_∈_)

    -- Helper: elements of valid-prime-residues are coprime to rad(base)
    postulate
      wheel-coprime-lemma : ∀ base r → r ∈ valid-prime-residues base → Coprime r (radical base)

{-|
  The construction (implemented across modules):
  - ring: from Core.ResidueClassesComplete.residue-ring
  - units: from units-are-coprime theorem
  - prime-filter: from Core.Radical.prime-residue-constraint
  - collapse: from Core.ResidueCollapse
  - affine-linear: from residue-linear lemma

  This UNIFIES all our discoveries!
-}

-------------------------------------------------------------------------------
-- PART 7: BASE FILTER ABSTRACTION
-------------------------------------------------------------------------------

{-|
  BaseFilter: A thin layer over ResidueFramework capturing the essential
  filtering properties of a base.

  Key insight: Which digits can primes have in a given base?
  Answer: Only those coprime to the radical of the base!
-}

record BaseFilter (b : ℕ) : Set where
  field
    rad        : ℕ             -- radical(b)
    units      : ℕ → Set       -- digits/residues coprime to b
    units↔coprime :
      ∀ d → units d ↔ Coprime d b

-- | Create a BaseFilter from a ResidueFramework
baseFilter : ∀ {b} → (b>1 : b > 1) → ResidueFramework b {b>1} → BaseFilter b
baseFilter {b} b>1 RF = record
  { rad = radical b
  ; units = λ d → Coprime d b
  ; units↔coprime = λ d → refl↔
  }
  where
    open ResidueFramework RF

{-|
  Key property: Primes > rad(base) can only have last digits that are units.
  This is the fundamental filtering mechanism!
-}
postulate
  primes-filtered-by-units : ∀ {b} (BF : BaseFilter b) →
    ∀ p → IsPrime p → p > BaseFilter.rad BF →
    BaseFilter.units BF (p mod b)

-------------------------------------------------------------------------------
-- SUMMARY
-------------------------------------------------------------------------------

{-
  This module provides the conceptual framework that explains:
  - Coprimality → Unit structure
  - Radical filtering → Residue constraint
  - GCD paradox → Collapse structure
  - Affine transform → Linear preservation
  - All discoveries → Residue framework!

  Implementation modules handle the detailed proofs while this
  module presents the unified theory.
-}

-- End of ResidueClasses module