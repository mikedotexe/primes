{-# OPTIONS --safe --without-K #-}
------------------------------------------------------------------------
-- Finite mask-transfer combinatorics
--
-- Strongest live signal:
-- 1. the five transfer buckets form an exact finite partition
-- 2. shared admissible overlap and admissible-count deltas are exact count
--    identities, not narrative labels
-- 3. good-event counts decompose exactly bucketwise on any aligned finite list
------------------------------------------------------------------------

module Theorems.Abstract.FiniteMaskTransfer where

open import Data.Bool using (Bool; true; false)
open import Data.Bool.Base using (if_then_else_)
open import Data.Empty using (⊥; ⊥-elim)
open import Data.List using (List; []; _∷_)
open import Data.Nat using (ℕ; zero; suc; _+_)
open import Data.Nat.Properties as Nat using (_≟_; +-assoc; +-comm)
open import Data.Product using (Σ; _×_; _,_)
open import Relation.Nullary using (yes; no)
open import Relation.Binary.PropositionalEquality as Eq using (_≡_; _≢_; refl; cong; sym)

open Eq.≡-Reasoning

------------------------------------------------------------------------
-- Transfer buckets
------------------------------------------------------------------------

data TransferBucket : Set where
  stableZero    : TransferBucket
  gainZero      : TransferBucket
  lossZero      : TransferBucket
  stableNonzero : TransferBucket
  nonzeroChurn  : TransferBucket

bucketEq : TransferBucket → TransferBucket → Bool
bucketEq stableZero stableZero = true
bucketEq gainZero gainZero = true
bucketEq lossZero lossZero = true
bucketEq stableNonzero stableNonzero = true
bucketEq nonzeroChurn nonzeroChurn = true
bucketEq _ _ = false

transferBucket : ℕ → ℕ → TransferBucket
transferBucket zero zero = stableZero
transferBucket zero (suc maskTo) = lossZero
transferBucket (suc maskFrom) zero = gainZero
transferBucket (suc maskFrom) (suc maskTo) with suc maskFrom ≟ suc maskTo
... | yes _ = stableNonzero
... | no _ = nonzeroChurn

------------------------------------------------------------------------
-- Bucket characterizations
------------------------------------------------------------------------

zero-zero→stableZero : transferBucket zero zero ≡ stableZero
zero-zero→stableZero = refl

nonzero-zero→gainZero : ∀ n → transferBucket (suc n) zero ≡ gainZero
nonzero-zero→gainZero n = refl

zero-nonzero→lossZero : ∀ n → transferBucket zero (suc n) ≡ lossZero
zero-nonzero→lossZero n = refl

stableNonzero-self : ∀ n → transferBucket (suc n) (suc n) ≡ stableNonzero
stableNonzero-self n with suc n ≟ suc n
... | yes _ = refl
... | no neq = ⊥-elim (neq refl)

different-nonzero→nonzeroChurn
  : ∀ {m n}
  → suc m ≢ suc n
  → transferBucket (suc m) (suc n) ≡ nonzeroChurn
different-nonzero→nonzeroChurn {m} {n} m≢n with suc m ≟ suc n
... | yes same = ⊥-elim (m≢n same)
... | no _ = refl

------------------------------------------------------------------------
-- Finite aligned rows
------------------------------------------------------------------------

record TransferRow : Set where
  constructor mkTransferRow
  field
    maskFrom : ℕ
    maskTo   : ℕ
    goodFrom : Bool
    goodTo   : Bool

open TransferRow public

rowBucket : TransferRow → TransferBucket
rowBucket row = transferBucket (maskFrom row) (maskTo row)

bool→ℕ : Bool → ℕ
bool→ℕ true = 1
bool→ℕ false = 0

countBy : ∀ {A : Set} → (A → Bool) → List A → ℕ
countBy pred [] = 0
countBy pred (x ∷ xs) = bool→ℕ (pred x) + countBy pred xs

bucketCount : TransferBucket → List TransferRow → ℕ
bucketCount bucket [] = 0
bucketCount bucket (row ∷ rows) with bucketEq bucket (rowBucket row)
... | true = 1 + bucketCount bucket rows
... | false = bucketCount bucket rows

sharedAdmissibleCount : List TransferRow → ℕ
sharedAdmissibleCount rows = bucketCount stableZero rows

admissibleCountFrom : List TransferRow → ℕ
admissibleCountFrom rows = bucketCount stableZero rows + bucketCount lossZero rows

admissibleCountTo : List TransferRow → ℕ
admissibleCountTo rows = bucketCount stableZero rows + bucketCount gainZero rows

sameMaskCount : List TransferRow → ℕ
sameMaskCount rows = bucketCount stableZero rows + bucketCount stableNonzero rows

zeroUnionCount : List TransferRow → ℕ
zeroUnionCount rows =
  bucketCount stableZero rows + bucketCount gainZero rows + bucketCount lossZero rows

sharedAdmissible-is-stableZero : ∀ rows → sharedAdmissibleCount rows ≡ bucketCount stableZero rows
sharedAdmissible-is-stableZero rows = refl

admissibleFrom-identity
  : ∀ rows
  → admissibleCountFrom rows ≡ bucketCount stableZero rows + bucketCount lossZero rows
admissibleFrom-identity rows = refl

admissibleTo-identity
  : ∀ rows
  → admissibleCountTo rows ≡ bucketCount stableZero rows + bucketCount gainZero rows
admissibleTo-identity rows = refl

sameMask-identity
  : ∀ rows
  → sameMaskCount rows ≡ bucketCount stableZero rows + bucketCount stableNonzero rows
sameMask-identity rows = refl

zeroUnion-identity
  : ∀ rows
  → zeroUnionCount rows ≡ bucketCount stableZero rows + bucketCount gainZero rows + bucketCount lossZero rows
zeroUnion-identity rows = refl

------------------------------------------------------------------------
-- Exact bucketwise good-count decomposition
------------------------------------------------------------------------

bucketGoodFromCount : TransferBucket → List TransferRow → ℕ
bucketGoodFromCount bucket [] = 0
bucketGoodFromCount bucket (row ∷ rows) with bucketEq bucket (rowBucket row)
... | true = bool→ℕ (goodFrom row) + bucketGoodFromCount bucket rows
... | false = bucketGoodFromCount bucket rows

bucketGoodToCount : TransferBucket → List TransferRow → ℕ
bucketGoodToCount bucket [] = 0
bucketGoodToCount bucket (row ∷ rows) with bucketEq bucket (rowBucket row)
... | true = bool→ℕ (goodTo row) + bucketGoodToCount bucket rows
... | false = bucketGoodToCount bucket rows

totalGoodFromCount : List TransferRow → ℕ
totalGoodFromCount [] = 0
totalGoodFromCount (row ∷ rows) = bool→ℕ (goodFrom row) + totalGoodFromCount rows

totalGoodToCount : List TransferRow → ℕ
totalGoodToCount [] = 0
totalGoodToCount (row ∷ rows) = bool→ℕ (goodTo row) + totalGoodToCount rows

sum5 : ℕ → ℕ → ℕ → ℕ → ℕ → ℕ
sum5 a b c d e = a + (b + (c + (d + e)))

sumBucketGoodFromCount : List TransferRow → ℕ
sumBucketGoodFromCount rows =
  sum5
    (bucketGoodFromCount stableZero rows)
    (bucketGoodFromCount gainZero rows)
    (bucketGoodFromCount lossZero rows)
    (bucketGoodFromCount stableNonzero rows)
    (bucketGoodFromCount nonzeroChurn rows)

sumBucketGoodToCount : List TransferRow → ℕ
sumBucketGoodToCount rows =
  sum5
    (bucketGoodToCount stableZero rows)
    (bucketGoodToCount gainZero rows)
    (bucketGoodToCount lossZero rows)
    (bucketGoodToCount stableNonzero rows)
    (bucketGoodToCount nonzeroChurn rows)

sum5-insert₁ : ∀ g a b c d e → sum5 (g + a) b c d e ≡ g + sum5 a b c d e
sum5-insert₁ g a b c d e =
  +-assoc g a (b + (c + (d + e)))

sum5-insert₂ : ∀ g a b c d e → sum5 a (g + b) c d e ≡ g + sum5 a b c d e
sum5-insert₂ g a b c d e = begin
  a + ((g + b) + (c + (d + e))) ≡⟨ cong (a +_) (+-assoc g b (c + (d + e))) ⟩
  a + (g + (b + (c + (d + e)))) ≡⟨ sym (+-assoc a g (b + (c + (d + e)))) ⟩
  (a + g) + (b + (c + (d + e))) ≡⟨ cong (_+ (b + (c + (d + e)))) (+-comm a g) ⟩
  (g + a) + (b + (c + (d + e))) ≡⟨ +-assoc g a (b + (c + (d + e))) ⟩
  g + (a + (b + (c + (d + e)))) ∎

sum5-insert₃ : ∀ g a b c d e → sum5 a b (g + c) d e ≡ g + sum5 a b c d e
sum5-insert₃ g a b c d e = begin
  a + (b + ((g + c) + (d + e))) ≡⟨ cong (a +_) (cong (b +_) (+-assoc g c (d + e))) ⟩
  a + (b + (g + (c + (d + e)))) ≡⟨ cong (a +_) (sym (+-assoc b g (c + (d + e)))) ⟩
  a + ((b + g) + (c + (d + e))) ≡⟨ cong (a +_) (cong (_+ (c + (d + e))) (+-comm b g)) ⟩
  a + ((g + b) + (c + (d + e))) ≡⟨ cong (a +_) (+-assoc g b (c + (d + e))) ⟩
  a + (g + (b + (c + (d + e)))) ≡⟨ sym (+-assoc a g (b + (c + (d + e)))) ⟩
  (a + g) + (b + (c + (d + e))) ≡⟨ cong (_+ (b + (c + (d + e)))) (+-comm a g) ⟩
  (g + a) + (b + (c + (d + e))) ≡⟨ +-assoc g a (b + (c + (d + e))) ⟩
  g + (a + (b + (c + (d + e)))) ∎

sum5-insert₄ : ∀ g a b c d e → sum5 a b c (g + d) e ≡ g + sum5 a b c d e
sum5-insert₄ g a b c d e = begin
  a + (b + (c + ((g + d) + e))) ≡⟨ cong (a +_) (cong (b +_) (cong (c +_) (+-assoc g d e))) ⟩
  a + (b + (c + (g + (d + e)))) ≡⟨ cong (a +_) (cong (b +_) (sym (+-assoc c g (d + e)))) ⟩
  a + (b + ((c + g) + (d + e))) ≡⟨ cong (a +_) (cong (b +_) (cong (_+ (d + e)) (+-comm c g))) ⟩
  a + (b + ((g + c) + (d + e))) ≡⟨ cong (a +_) (cong (b +_) (+-assoc g c (d + e))) ⟩
  a + (b + (g + (c + (d + e)))) ≡⟨ cong (a +_) (sym (+-assoc b g (c + (d + e)))) ⟩
  a + ((b + g) + (c + (d + e))) ≡⟨ cong (a +_) (cong (_+ (c + (d + e))) (+-comm b g)) ⟩
  a + ((g + b) + (c + (d + e))) ≡⟨ cong (a +_) (+-assoc g b (c + (d + e))) ⟩
  a + (g + (b + (c + (d + e)))) ≡⟨ sym (+-assoc a g (b + (c + (d + e)))) ⟩
  (a + g) + (b + (c + (d + e))) ≡⟨ cong (_+ (b + (c + (d + e)))) (+-comm a g) ⟩
  (g + a) + (b + (c + (d + e))) ≡⟨ +-assoc g a (b + (c + (d + e))) ⟩
  g + (a + (b + (c + (d + e)))) ∎

sum5-insert₅ : ∀ g a b c d e → sum5 a b c d (g + e) ≡ g + sum5 a b c d e
sum5-insert₅ g a b c d e = begin
  a + (b + (c + (d + (g + e)))) ≡⟨ cong (a +_) (cong (b +_) (cong (c +_) (sym (+-assoc d g e)))) ⟩
  a + (b + (c + ((d + g) + e))) ≡⟨ cong (a +_) (cong (b +_) (cong (c +_) (cong (_+ e) (+-comm d g)))) ⟩
  a + (b + (c + ((g + d) + e))) ≡⟨ cong (a +_) (cong (b +_) (cong (c +_) (+-assoc g d e))) ⟩
  a + (b + (c + (g + (d + e)))) ≡⟨ cong (a +_) (cong (b +_) (sym (+-assoc c g (d + e)))) ⟩
  a + (b + ((c + g) + (d + e))) ≡⟨ cong (a +_) (cong (b +_) (cong (_+ (d + e)) (+-comm c g))) ⟩
  a + (b + ((g + c) + (d + e))) ≡⟨ cong (a +_) (cong (b +_) (+-assoc g c (d + e))) ⟩
  a + (b + (g + (c + (d + e)))) ≡⟨ cong (a +_) (sym (+-assoc b g (c + (d + e)))) ⟩
  a + ((b + g) + (c + (d + e))) ≡⟨ cong (a +_) (cong (_+ (c + (d + e))) (+-comm b g)) ⟩
  a + ((g + b) + (c + (d + e))) ≡⟨ cong (a +_) (+-assoc g b (c + (d + e))) ⟩
  a + (g + (b + (c + (d + e)))) ≡⟨ sym (+-assoc a g (b + (c + (d + e)))) ⟩
  (a + g) + (b + (c + (d + e))) ≡⟨ cong (_+ (b + (c + (d + e)))) (+-comm a g) ⟩
  (g + a) + (b + (c + (d + e))) ≡⟨ +-assoc g a (b + (c + (d + e))) ⟩
  g + (a + (b + (c + (d + e)))) ∎

totalGoodFromCount-eq-sumBucketGoodFromCount : ∀ rows → totalGoodFromCount rows ≡ sumBucketGoodFromCount rows
totalGoodFromCount-eq-sumBucketGoodFromCount [] = refl
totalGoodFromCount-eq-sumBucketGoodFromCount (row ∷ rows)
  with rowBucket row | totalGoodFromCount-eq-sumBucketGoodFromCount rows
... | stableZero | ih = Eq.trans (cong (bool→ℕ (goodFrom row) +_) ih)
                                  (sym (sum5-insert₁ (bool→ℕ (goodFrom row))
                                    (bucketGoodFromCount stableZero rows)
                                    (bucketGoodFromCount gainZero rows)
                                    (bucketGoodFromCount lossZero rows)
                                    (bucketGoodFromCount stableNonzero rows)
                                    (bucketGoodFromCount nonzeroChurn rows)))
... | gainZero | ih = Eq.trans (cong (bool→ℕ (goodFrom row) +_) ih)
                                (sym (sum5-insert₂ (bool→ℕ (goodFrom row))
                                  (bucketGoodFromCount stableZero rows)
                                  (bucketGoodFromCount gainZero rows)
                                  (bucketGoodFromCount lossZero rows)
                                  (bucketGoodFromCount stableNonzero rows)
                                  (bucketGoodFromCount nonzeroChurn rows)))
... | lossZero | ih = Eq.trans (cong (bool→ℕ (goodFrom row) +_) ih)
                                (sym (sum5-insert₃ (bool→ℕ (goodFrom row))
                                  (bucketGoodFromCount stableZero rows)
                                  (bucketGoodFromCount gainZero rows)
                                  (bucketGoodFromCount lossZero rows)
                                  (bucketGoodFromCount stableNonzero rows)
                                  (bucketGoodFromCount nonzeroChurn rows)))
... | stableNonzero | ih = Eq.trans (cong (bool→ℕ (goodFrom row) +_) ih)
                                     (sym (sum5-insert₄ (bool→ℕ (goodFrom row))
                                       (bucketGoodFromCount stableZero rows)
                                       (bucketGoodFromCount gainZero rows)
                                       (bucketGoodFromCount lossZero rows)
                                       (bucketGoodFromCount stableNonzero rows)
                                       (bucketGoodFromCount nonzeroChurn rows)))
... | nonzeroChurn | ih = Eq.trans (cong (bool→ℕ (goodFrom row) +_) ih)
                                   (sym (sum5-insert₅ (bool→ℕ (goodFrom row))
                                     (bucketGoodFromCount stableZero rows)
                                     (bucketGoodFromCount gainZero rows)
                                     (bucketGoodFromCount lossZero rows)
                                     (bucketGoodFromCount stableNonzero rows)
                                     (bucketGoodFromCount nonzeroChurn rows)))

totalGoodToCount-eq-sumBucketGoodToCount : ∀ rows → totalGoodToCount rows ≡ sumBucketGoodToCount rows
totalGoodToCount-eq-sumBucketGoodToCount [] = refl
totalGoodToCount-eq-sumBucketGoodToCount (row ∷ rows)
  with rowBucket row | totalGoodToCount-eq-sumBucketGoodToCount rows
... | stableZero | ih = Eq.trans (cong (bool→ℕ (goodTo row) +_) ih)
                                  (sym (sum5-insert₁ (bool→ℕ (goodTo row))
                                    (bucketGoodToCount stableZero rows)
                                    (bucketGoodToCount gainZero rows)
                                    (bucketGoodToCount lossZero rows)
                                    (bucketGoodToCount stableNonzero rows)
                                    (bucketGoodToCount nonzeroChurn rows)))
... | gainZero | ih = Eq.trans (cong (bool→ℕ (goodTo row) +_) ih)
                                (sym (sum5-insert₂ (bool→ℕ (goodTo row))
                                  (bucketGoodToCount stableZero rows)
                                  (bucketGoodToCount gainZero rows)
                                  (bucketGoodToCount lossZero rows)
                                  (bucketGoodToCount stableNonzero rows)
                                  (bucketGoodToCount nonzeroChurn rows)))
... | lossZero | ih = Eq.trans (cong (bool→ℕ (goodTo row) +_) ih)
                                (sym (sum5-insert₃ (bool→ℕ (goodTo row))
                                  (bucketGoodToCount stableZero rows)
                                  (bucketGoodToCount gainZero rows)
                                  (bucketGoodToCount lossZero rows)
                                  (bucketGoodToCount stableNonzero rows)
                                  (bucketGoodToCount nonzeroChurn rows)))
... | stableNonzero | ih = Eq.trans (cong (bool→ℕ (goodTo row) +_) ih)
                                     (sym (sum5-insert₄ (bool→ℕ (goodTo row))
                                       (bucketGoodToCount stableZero rows)
                                       (bucketGoodToCount gainZero rows)
                                       (bucketGoodToCount lossZero rows)
                                       (bucketGoodToCount stableNonzero rows)
                                       (bucketGoodToCount nonzeroChurn rows)))
... | nonzeroChurn | ih = Eq.trans (cong (bool→ℕ (goodTo row) +_) ih)
                                   (sym (sum5-insert₅ (bool→ℕ (goodTo row))
                                     (bucketGoodToCount stableZero rows)
                                     (bucketGoodToCount gainZero rows)
                                     (bucketGoodToCount lossZero rows)
                                     (bucketGoodToCount stableNonzero rows)
                                     (bucketGoodToCount nonzeroChurn rows)))

------------------------------------------------------------------------
-- Maintained synthetic example: one row in each bucket
------------------------------------------------------------------------

exampleRows : List TransferRow
exampleRows =
  mkTransferRow 0 0 true false
  ∷ mkTransferRow 1 0 false true
  ∷ mkTransferRow 0 2 false false
  ∷ mkTransferRow 3 3 true true
  ∷ mkTransferRow 4 5 true false
  ∷ []

example-stableZero-count : bucketCount stableZero exampleRows ≡ 1
example-stableZero-count = refl

example-gainZero-count : bucketCount gainZero exampleRows ≡ 1
example-gainZero-count = refl

example-lossZero-count : bucketCount lossZero exampleRows ≡ 1
example-lossZero-count = refl

example-stableNonzero-count : bucketCount stableNonzero exampleRows ≡ 1
example-stableNonzero-count = refl

example-nonzeroChurn-count : bucketCount nonzeroChurn exampleRows ≡ 1
example-nonzeroChurn-count = refl
