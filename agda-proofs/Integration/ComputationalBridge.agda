{-# OPTIONS --safe --without-K #-}

{-|
  Computational Bridge Module

  This module provides computational interfaces that bridge between
  the formal Agda proofs and practical implementations (e.g., in Rust).

  It focuses on:
  - Decidable procedures that can be executed
  - Efficient algorithms with formal correctness guarantees
  - Data structures suitable for computation
-}

module Integration.ComputationalBridge where

open import Data.Nat using (ℕ; zero; suc; _+_; _*_; _<_; _≤_; _%_)
open import Data.Bool using (Bool; true; false; _∧_; _∨_; not)
open import Data.List using (List; []; _∷_; map; filter; length; all; any)
open import Data.Product using (_×_; _,_; proj₁; proj₂)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Relation.Nullary using (Dec; yes; no)

--------------------------------------------------------------------------------
-- Computational Primitives
--------------------------------------------------------------------------------

-- Import computational functions
open import Core.Primality using (prime?; isPrime?)
open import Core.Radical using (radical)
open import Core.PhaseLocks using (findPhaseLocks; isLeftValid)
open import Core.CRTVector using (P0viaL; CRT-ok?)
open import Core.ResidueFold using (Pattern; Counts; countsDPConv)

--------------------------------------------------------------------------------
-- Efficient Algorithms
--------------------------------------------------------------------------------

{-|
  Fast primality testing up to a bound
  Returns list of primes ≤ n
-}
sievePrimes : ℕ → List ℕ
sievePrimes n = filter prime? (range 2 n)
  where
    range : ℕ → ℕ → List ℕ
    range from zero = []
    range from (suc to) with from ≤? suc to
    ... | yes _ = from ∷ range (suc from) to
    ... | no  _ = []

    _≤?_ : ℕ → ℕ → Dec (ℕ._≤_ _ _)
    _≤?_ = Data.Nat._≤?_

{-|
  Compute valid prime residues for a base
-}
computeValidResidues : ℕ → List ℕ
computeValidResidues base =
  let r = radical base
  in filter (λ k → coprime? k r) (range 1 r)
  where
    range : ℕ → ℕ → List ℕ
    range from to with from <? to
    ... | yes _ = from ∷ range (suc from) to
    ... | no  _ = []

    _<?_ : ℕ → ℕ → Dec (ℕ._<_ _ _)
    _<?_ = Data.Nat._<?_

    coprime? : ℕ → ℕ → Bool
    coprime? a b = gcd a b ≡ᵇ 1
      where
        open import Data.Nat.GCD using (gcd)
        open import Data.Nat using (_≡ᵇ_)

{-|
  Check if a number is a valid 2p base
-}
check2pBase : ℕ → Maybe (ℕ × List (ℕ × ℕ × ℕ))
  where
    data Maybe (A : Set) : Set where
      nothing : Maybe A
      just    : A → Maybe A
check2pBase base with base % 2 ≡ᵇ 0
... | false = nothing
... | true =
  let p = base / 2
  in if prime? p ∧ (p ≥ᵇ 3)
     then just (p , findPhaseLocks base)
     else nothing
  where
    _≥ᵇ_ : ℕ → ℕ → Bool
    zero ≥ᵇ zero = true
    zero ≥ᵇ suc _ = false
    suc m ≥ᵇ zero = true
    suc m ≥ᵇ suc n = m ≥ᵇ n

{-|
  Batch CRT computation for multiple primes
-}
batchCRT : ℕ → List ℕ → Pattern → List (ℕ × ℕ)
batchCRT base primes pattern = P0viaL base primes pattern

{-|
  Quick discriminant quality score
-}
quickDiscriminantScore : ℕ → ℕ → ℤ
  where open import Data.Integer using (ℤ)
quickDiscriminantScore A S =
  let quality = analyzeQuality A S
  in DiscriminantQuality.score quality
  where
    open import Core.Discriminant using (analyzeQuality; DiscriminantQuality)

--------------------------------------------------------------------------------
-- Verification Procedures
--------------------------------------------------------------------------------

{-|
  Verify phase lock properties
-}
verifyPhaseLock : ℕ → ℕ × ℕ × ℕ → Bool
verifyPhaseLock base (left , right , dist) =
  (left + right ≡ᵇ base) ∧
  (isLeftValid left) ∧
  (prime? right) ∧
  checkSymmetry
  where
    checkSymmetry : Bool
    checkSymmetry with base % 2 ≡ᵇ 0
    ... | false = false
    ... | true =
      let mid = base / 2
      in (left ≡ᵇ mid ∸ dist) ∧ (right ≡ᵇ mid + dist)

{-|
  Verify CRT optimization correctness
-}
verifyCRTOptimization : ℕ → List ℕ → Pattern → Bool
verifyCRTOptimization base primes pattern = CRT-ok? base primes pattern

--------------------------------------------------------------------------------
-- Data Export Functions
--------------------------------------------------------------------------------

{-|
  Export residue distribution as counts
-}
exportResidueCounts : ℕ → ℕ → Pattern → List (ℕ × ℕ)
exportResidueCounts base modulus pattern =
  let counts = countsDPConv base modulus pattern
  in counts  -- Already in (residue, count) format

{-|
  Export phase lock analysis
-}
record PhaseLockExport : Set where
  field
    base      : ℕ
    prime     : ℕ
    locks     : List (ℕ × ℕ × ℕ)  -- (left, right, distance)
    hasLocks  : Bool
    lockCount : ℕ

exportPhaseLocks : ℕ → PhaseLockExport
exportPhaseLocks base with check2pBase base
... | nothing = record
  { base = base
  ; prime = 0
  ; locks = []
  ; hasLocks = false
  ; lockCount = 0
  }
... | just (p , locks) = record
  { base = base
  ; prime = p
  ; locks = locks
  ; hasLocks = not (null locks)
  ; lockCount = length locks
  }
  where
    null : {A : Set} → List A → Bool
    null [] = true
    null (_ ∷ _) = false

--------------------------------------------------------------------------------
-- Performance Benchmarks
--------------------------------------------------------------------------------

{-|
  Benchmark data structure for comparing approaches
-}
record Benchmark : Set where
  field
    description : String
      where postulate String : Set
    inputSize   : ℕ
    operations  : ℕ
    verified    : Bool

{-|
  CRT optimization benchmark
-}
crtBenchmark : ℕ → List ℕ → Benchmark
crtBenchmark base primes = record
  { description = "CRT vs Direct DP"
  ; inputSize = length primes
  ; operations = lcmList primes  -- LCM is the key metric
  ; verified = CRT-ok? base primes pattern
  }
  where
    open import Core.CRTVector using (lcmList)
    pattern = Open (range 0 base) ∷ []
      where
        Open : List ℕ → Slot
          where open import Core.ResidueFold

        range : ℕ → ℕ → List ℕ
        range from zero = []
        range from (suc to) = from ∷ range (suc from) to

--------------------------------------------------------------------------------
-- Integration with External Tools
--------------------------------------------------------------------------------

{-|
  Configuration export for external tools (e.g., Rust density-explorer)
-}
record ToolConfig : Set where
  field
    base           : ℕ
    validResidues  : List ℕ
    radicalValue   : ℕ
    trackedPrimes  : List ℕ
    useCRT         : Bool
    lcmBound       : ℕ

exportToolConfig : ℕ → List ℕ → ℕ → ToolConfig
exportToolConfig base primes lcmCap = record
  { base = base
  ; validResidues = computeValidResidues base
  ; radicalValue = radical base
  ; trackedPrimes = primes
  ; useCRT = lcm ≤ᵇ lcmCap
  ; lcmBound = lcm
  }
  where
    open import Core.CRTVector using (lcmList)
    lcm = lcmList primes

    _≤ᵇ_ : ℕ → ℕ → Bool
    zero ≤ᵇ _ = true
    suc m ≤ᵇ zero = false
    suc m ≤ᵇ suc n = m ≤ᵇ n

-- End of module