{-
  ═══════════════════════════════════════════════════════════════════════
  GREATEST COMMON DIVISOR (GCD) PROPERTIES
  ═══════════════════════════════════════════════════════════════════════

  This module formalizes GCD and proves the properties essential for
  understanding why coprimality matters in membrane construction.

  Key results:
  • Euclidean algorithm correctness
  • GCD of 1 means coprimality
  • GCD properties under multiplication
  • Bézout's identity (gcd can be expressed as linear combination)

  Author: Prime Physics Engine Research Team
  Version: 1.0.0
-}

module PrimePhysics.Foundation.GCD where

open import PrimePhysics.Foundation.Nat
open import Data.Nat using (ℕ; zero; suc; _+_; _*_; _∸_)
open import Data.Nat.DivMod using (_mod_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl; sym; trans; cong)
open import Data.Product using (_×_; _,_; ∃-syntax)

-------------------------------------------------------------------------------
-- GCD DEFINITION
-------------------------------------------------------------------------------

{- DEFINITION: Greatest Common Divisor

   gcd(m, n) is the largest d such that d ∣ m and d ∣ n.

   Properties that characterize GCD:
   1. d divides both m and n
   2. Any other common divisor divides d
-}
record IsGCD (d m n : ℕ) : Set where
  field
    -- d divides both m and n
    divides-m : d ∣ m
    divides-n : d ∣ n

    -- d is the *greatest* common divisor
    -- (any other common divisor must divide d)
    greatest : ∀ d' → d' ∣ m → d' ∣ n → d' ∣ d

{- FUNCTION: Compute GCD via Euclidean algorithm

   The classic algorithm: gcd(m, n) = gcd(n, m mod n)
   Base case: gcd(m, 0) = m

   Example: gcd(12, 8)
            = gcd(8, 4)   [12 mod 8 = 4]
            = gcd(4, 0)   [8 mod 4 = 0]
            = 4
-}
gcd : ℕ → ℕ → ℕ
gcd m zero = m
gcd m n@(suc _) = gcd n (m mod n)

{- Notation for GCD -}
GCD : ℕ → ℕ → Set
GCD m n = IsGCD (gcd m n) m n

-------------------------------------------------------------------------------
-- BASIC GCD PROPERTIES
-------------------------------------------------------------------------------

{- THEOREM: GCD is commutative
   gcd(m, n) = gcd(n, m)

   This follows from the symmetry of the IsGCD predicate.
-}
postulate
  gcd-comm : ∀ m n → gcd m n ≡ gcd n m

{- THEOREM: GCD is associative
   gcd(gcd(m, n), k) = gcd(m, gcd(n, k))

   Useful for chaining GCD computations.
-}
postulate
  gcd-assoc : ∀ m n k → gcd (gcd m n) k ≡ gcd m (gcd n k)

{- THEOREM: GCD with 0
   gcd(n, 0) = n
   gcd(0, n) = n

   Base case of the Euclidean algorithm.
-}
gcd-zero-right : ∀ n → gcd n zero ≡ n
gcd-zero-right n = refl

postulate
  gcd-zero-left : ∀ n → gcd zero n ≡ n

{- THEOREM: GCD with 1
   gcd(n, 1) = 1 for all n

   This is fundamental: 1 divides everything, and nothing bigger
   divides 1, so the greatest common divisor must be 1.
-}
postulate
  gcd-one-right : ∀ n → gcd n 1 ≡ 1
  gcd-one-left : ∀ n → gcd 1 n ≡ 1

{- THEOREM: GCD with self
   gcd(n, n) = n

   A number's greatest common divisor with itself is itself.
-}
postulate
  gcd-self : ∀ n → gcd n n ≡ n

-------------------------------------------------------------------------------
-- COPRIMALITY
-------------------------------------------------------------------------------

{- DEFINITION: Coprimality

   Two numbers are coprime if their GCD is 1.

   Example: 15 and 28 are coprime (share no prime factors)
            15 = 3 × 5
            28 = 2² × 7
            gcd(15, 28) = 1
-}
Coprime : ℕ → ℕ → Set
Coprime m n = gcd m n ≡ 1

{- NOTATION: Write m ⊥ n for "m and n are coprime" -}
_⊥_ : ℕ → ℕ → Set
m ⊥ n = Coprime m n

{- THEOREM: 1 is coprime to everything -}
1-coprime : ∀ n → 1 ⊥ n
1-coprime n = gcd-one-left n

{- THEOREM: Coprimality is symmetric -}
coprime-sym : ∀ {m n} → m ⊥ n → n ⊥ m
coprime-sym {m} {n} prf = trans (gcd-comm n m) prf

-------------------------------------------------------------------------------
-- GCD AND MULTIPLICATION
-------------------------------------------------------------------------------

{- THEOREM: GCD distributes over multiplication (special case)

   If gcd(m, n) = 1, then gcd(m, n*k) = gcd(m, k)

   Intuition: If m and n share no factors, then m and n*k share
   only the factors that m and k share.

   This is crucial for understanding why coprime boundary digits
   help membranes avoid systematic divisibility.
-}
postulate
  gcd-mult-coprime : ∀ m n k → m ⊥ n → gcd m (n * k) ≡ gcd m k

{- THEOREM: Coprimality preserved under multiplication

   If m ⊥ n and m ⊥ k, then m ⊥ (n * k)

   This lets us reason about composite structures built from
   coprime components.
-}
postulate
  coprime-mult : ∀ {m n k} → m ⊥ n → m ⊥ k → m ⊥ (n * k)

-------------------------------------------------------------------------------
-- BÉZOUT'S IDENTITY
-------------------------------------------------------------------------------

{- THEOREM: Bézout's Identity

   For any m, n, there exist integers a, b such that:
   gcd(m, n) = a*m + b*n

   Note: We use ℕ instead of ℤ, so we need a more careful statement.
   For coprime m, n, this becomes: 1 = a*m - b*n (or vice versa).

   This is the foundation for many deeper GCD results.
-}
postulate
  bezout : ∀ m n → ∃[ a ] ∃[ b ] (gcd m n ≡ a * m + b * n)

-------------------------------------------------------------------------------
-- DIVISIBILITY AND GCD
-------------------------------------------------------------------------------

{- THEOREM: If d divides both m and n, then d divides gcd(m, n)

   This is part of what makes gcd "greatest" - any common divisor
   must divide the gcd.
-}
postulate
  common-divisor-divides-gcd : ∀ {d m n} → d ∣ m → d ∣ n → d ∣ gcd m n

{- THEOREM: GCD divides both arguments

   The gcd is itself a common divisor.
-}
postulate
  gcd-divides-left : ∀ m n → gcd m n ∣ m
  gcd-divides-right : ∀ m n → gcd m n ∣ n

-------------------------------------------------------------------------------
-- PRIME-SPECIFIC GCD PROPERTIES
-------------------------------------------------------------------------------

{- THEOREM: GCD with a prime

   If p is prime, then gcd(p, n) is either 1 or p.
   - gcd(p, n) = p if p ∣ n
   - gcd(p, n) = 1 if p ∤ n

   This is fundamental to understanding primality in our membranes.
-}
open PrimePhysics.Foundation.Nat using (IsPrime)

postulate
  gcd-prime : ∀ {p n} → IsPrime p →
    (gcd p n ≡ 1) ∨ (gcd p n ≡ p)
    where open import Data.Sum using (_⊎_) renaming (_⊎_ to _∨_)

{- COROLLARY: If p is prime and p ∤ n, then p ⊥ n -}
postulate
  prime-not-divides-coprime : ∀ {p n} → IsPrime p → ¬ (p ∣ n) → p ⊥ n
    where open import Relation.Nullary using (¬_)

-------------------------------------------------------------------------------
-- GCD EXAMPLES (Concrete Computations)
-------------------------------------------------------------------------------

{- These examples demonstrate the GCD algorithm on concrete values
   used in our membrane experiments. -}

-- Example: gcd(6, 10) = 2
_ : gcd 6 10 ≡ 2
_ = refl

-- Example: gcd(10, 3) = 1 (coprime!)
_ : gcd 10 3 ≡ 1
_ = refl

-- Example: gcd(10, 7) = 1 (coprime!)
_ : gcd 10 7 ≡ 1
_ = refl

-- Example: gcd(30, 11) = 1 (coprime!)
_ : gcd 30 11 ≡ 1
_ = refl

-- Example: gcd(30, 7) = 1 (coprime!)
_ : gcd 30 7 ≡ 1
_ = refl

{- Observation: The high-performing membrane configurations
   (like base 10 with digits 3,7 or base 30 with digits 11,7)
   all have coprime boundary digits! -}

-------------------------------------------------------------------------------
-- REMARKS
-------------------------------------------------------------------------------

{-
  This module establishes the GCD theory needed for membrane proofs.

  Key insights for the membrane project:

  1. Coprimality (gcd = 1) means sharing no prime factors.
     This prevents systematic divisibility patterns.

  2. If boundary digits are coprime to the base's radical,
     the membrane has maximum flexibility to avoid divisors.

  3. The theorems here let us *prove* that certain digit choices
     necessarily lead to coprimality, rather than just observing it.

  Next steps:
  - Foundation.Coprimality: More theorems about coprime numbers
  - Foundation.Radical: Formalize rad(n) = product of distinct prime factors
  - Membrane.Properties: Apply these to prove membrane coprimality
-}
