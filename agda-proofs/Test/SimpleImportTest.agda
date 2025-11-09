-- Quick import test to verify Agda 2.8.0 compatibility

module Test.SimpleImportTest where

open import Data.Empty using (⊥)
open import Data.Product using (Σ; _,_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Data.Nat using (ℕ; zero; suc)

-- Simple proof that imports work
test-imports : ℕ
test-imports = zero

-- Simple equality
test-eq : zero ≡ zero
test-eq = refl
