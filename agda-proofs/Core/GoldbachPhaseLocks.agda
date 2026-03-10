{-# OPTIONS --safe --without-K #-}

{-|
  Core.GoldbachPhaseLocks: Bridge between Phase Locks and Goldbach Pairs

  This module establishes the deep connection between phase locks in base 2p
  and Goldbach pairs for the even number 2p. The key insight is that every
  phase lock (symmetric prime pair) is a Goldbach pair and vice versa.

  Key theorems:
  - Phase locks ↔ Goldbach pairs
  - Spectral classification of phase locks
  - Connection to residue structure
-}

module Core.GoldbachPhaseLocks where

open import Data.Nat using (ℕ; zero; suc; _+_; _*_; _∸_; _<_; _>_; _≤_)
open import Data.Nat.Properties using (+-comm; *-comm; +-assoc)
open import Data.Product using (_×_; _,_; ∃; proj₁; proj₂)
open import Data.Sum using (_⊎_; inj₁; inj₂)
open import Data.List using (List; []; _∷_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl; sym; trans)
open import Relation.Nullary using (¬_; Dec; yes; no)

open import Core.Primality using (IsPrime)
open import Core.Equiv using (_↔_; mk↔)
open import Core.TwoPBase using (TwoPBase; base; mkTwoPBase)
open import Core.PhaseLocks using (PhaseLock; GoldbachPair; PhaseLock↔Goldbach;
  mkPhaseLock; mkGoldbachPair; PhaseLockDistance)
open import Core.Spectral using (QuadraticCharacter; IsQR; IsNQR; Epsilon; ε+1; ε-1)
open import Core.ResidueClasses using (ResidueFramework; BaseFilter)

--------------------------------------------------------------------------------
-- Core Equivalence (re-exported for convenience)
--------------------------------------------------------------------------------

-- | Phase locks and Goldbach pairs are the same thing
phaseLocks≡GoldbachPairs : ∀ (B : TwoPBase) → PhaseLock B ↔ GoldbachPair B
phaseLocks≡GoldbachPairs = PhaseLock↔Goldbach

--------------------------------------------------------------------------------
-- Spectral Classification of Phase Locks
--------------------------------------------------------------------------------

-- | A phase lock distance can be classified by its spectral type
data SpectralType : Set where
  QR-distance  : SpectralType  -- Distance is a quadratic residue
  NQR-distance : SpectralType  -- Distance is a non-residue

-- | Extract spectral type from a PhaseLockDistance
spectralType : ∀ {B : TwoPBase} {QC : QuadraticCharacter (TwoPBase.p B)} →
  PhaseLockDistance B QC → SpectralType
spectralType pld with PhaseLockDistance.spectral-tag pld
... | inj₁ qr  = QR-distance
... | inj₂ nqr = NQR-distance

-- | Count phase locks by spectral type
record PhaseLockCounts (B : TwoPBase) : Set where
  field
    total     : ℕ
    qr-count  : ℕ
    nqr-count : ℕ
    count-sum : qr-count + nqr-count ≡ total

--------------------------------------------------------------------------------
-- Connection to Residue Structure
--------------------------------------------------------------------------------

-- | Phase lock distances must be valid residues mod 2p
valid-phase-lock-distance : ∀ (B : TwoPBase) → ℕ → Set
valid-phase-lock-distance B d =
  (d < TwoPBase.p B) ×
  (d > 0) ×
  (d % 2 ≡ 1)  -- Must be odd (coprime to 2)
  where open import Data.Nat using (_%_)

-- | The set of all possible phase lock distances
possible-distances : TwoPBase → List ℕ
possible-distances B = filter-valid (applyUpTo suc (TwoPBase.p B))
  where
    open import Data.List using (filter)
    open import Data.List.Base using (applyUpTo)
    open import Data.Bool using (Bool; true; false)

    filter-valid : List ℕ → List ℕ
    filter-valid = filter (λ d → (d % 2) ≡ᵇ 1)
      where open import Data.Nat using (_≡ᵇ_; _%_)

--------------------------------------------------------------------------------
-- Goldbach's Conjecture for 2p
--------------------------------------------------------------------------------

-- | The restricted Goldbach conjecture for bases 2p
GoldbachFor2p : TwoPBase → Set
GoldbachFor2p B = ∃ λ (pl : PhaseLock B) → ⊤
  where open import Data.Unit using (⊤)

-- | Equivalently: Every even 2p (p ≥ 3) is a sum of two primes
GoldbachFor2p' : TwoPBase → Set
GoldbachFor2p' B = ∃ λ (gb : GoldbachPair B) → ⊤
  where open import Data.Unit using (⊤)

-- | The two formulations are equivalent
goldbach-equiv : ∀ (B : TwoPBase) → GoldbachFor2p B ↔ GoldbachFor2p'  B
goldbach-equiv B = mk↔ to from
  where
    open import Data.Unit using (⊤; tt)

    to : GoldbachFor2p B → GoldbachFor2p' B
    to (pl , tt) = (proj₁ (PhaseLock↔Goldbach B) pl , tt)

    from : GoldbachFor2p' B → GoldbachFor2p B
    from (gb , tt) = (proj₂ (PhaseLock↔Goldbach B) gb , tt)

--------------------------------------------------------------------------------
-- Spectral Symmetry Breaking
--------------------------------------------------------------------------------

{-|
  CONJECTURE: The distribution of phase locks depends on the spectral type of p.

  For p ≡ 1 (mod 4): QR and NQR distances appear symmetrically
  For p ≡ 3 (mod 4): Asymmetry emerges, with preference depending on p
-}

-- | Expected symmetry type based on prime's mod 4 class
expectedSymmetry : ∀ p → IsPrime p → p > 2 → p % 4 ≡ 1 → Set
  where open import Data.Nat using (_%_)
expectedSymmetry p pPrime p>2 p≡1mod4 =
  ∀ (B : TwoPBase) → TwoPBase.p B ≡ p →
  ∀ (counts : PhaseLockCounts B) →
    -- For Type A primes, expect balanced QR/NQR counts
    ∃ λ (ε : ℕ) → ε < TwoPBase.p B ×
      (PhaseLockCounts.qr-count counts ∸ PhaseLockCounts.nqr-count counts) ≤ ε

--------------------------------------------------------------------------------
-- Bridge to ResidueFramework
--------------------------------------------------------------------------------

-- | Phase locks respect the residue framework of base 2p
phaseLockResidue : ∀ {B : TwoPBase} (RF : ResidueFramework (base B)) →
  PhaseLock B → Set
phaseLockResidue {B} RF pl =
  let open PhaseLock pl in
  -- Both primes in the lock are valid residues mod rad(2p)
  ∃ λ (left-valid : left ∈ ResidueFramework.wheel-classes RF) →
  ∃ λ (right-valid : right ∈ ResidueFramework.wheel-classes RF) →
    ⊤
  where
    open import Data.Unit using (⊤)
    open import Data.List.Membership.Propositional using (_∈_)

--------------------------------------------------------------------------------
-- Export Summary
--------------------------------------------------------------------------------

{-|
  This module establishes:

  1. Phase locks = Goldbach pairs (formal equivalence)
  2. Spectral classification of phase locks (QR vs NQR distances)
  3. Connection to residue framework and valid distances
  4. Restricted Goldbach conjecture for bases 2p
  5. Spectral symmetry breaking conjectures

  This bridges the algebraic (residues), spectral (QR/NQR), and
  analytic (Goldbach) perspectives on phase locks.
-}

-- End of Core.GoldbachPhaseLocks module