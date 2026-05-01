{-# OPTIONS --safe --without-K #-}
------------------------------------------------------------------------
-- Bounded-k compactness shell
--
-- Strongest live signal:
-- 1. bounded-k compactness can be stated exactly as padding/diameter arithmetic
-- 2. the compact configuration k = (0,0) is the exact diameter minimizer
-- 3. increasing either padding coordinate shifts the template positions in the
--    expected monotone way without importing any prime-density language
------------------------------------------------------------------------

module Theorems.BoundedKCompactness where

open import Data.Nat using (ℕ; _+_; _*_; _≤_; z≤n)
open import Data.Nat.Properties as Nat using
  ( +-assoc
  ; +-identityʳ
  ; +-comm
  ; *-distribˡ-+
  ; m≤m+n
  )
open import Relation.Binary.PropositionalEquality as Eq using (_≡_; refl; cong; sym)

open Eq.≡-Reasoning

------------------------------------------------------------------------
-- Compactness data
------------------------------------------------------------------------

record BoundedKConfig : Set where
  constructor kConfig
  field
    kOuter : ℕ
    kInner : ℕ

open BoundedKConfig public

compact₀ : BoundedKConfig
compact₀ = kConfig 0 0

paddingWeight : BoundedKConfig → ℕ
paddingWeight cfg = kOuter cfg + kInner cfg

seed-position : BoundedKConfig → ℕ
seed-position cfg = paddingWeight cfg + 2

outer-left-position : BoundedKConfig → ℕ
outer-left-position cfg = 2 * seed-position cfg

inner-left-position : BoundedKConfig → ℕ
inner-left-position cfg = seed-position cfg + kInner cfg + 1

inner-right-position : BoundedKConfig → ℕ
inner-right-position cfg = kInner cfg + 1

diameter : ℕ → BoundedKConfig → ℕ
diameter middleWidth cfg = middleWidth + 4 + 2 * paddingWeight cfg

------------------------------------------------------------------------
-- Exact compactness identities
------------------------------------------------------------------------

paddingWeight-compact₀ : paddingWeight compact₀ ≡ 0
paddingWeight-compact₀ = refl

diameter-compact₀ : ∀ middleWidth → diameter middleWidth compact₀ ≡ middleWidth + 4
diameter-compact₀ middleWidth = begin
  diameter middleWidth compact₀ ≡⟨ refl ⟩
  middleWidth + 4 + 2 * 0      ≡⟨ refl ⟩
  middleWidth + 4 + 0          ≡⟨ +-identityʳ (middleWidth + 4) ⟩
  middleWidth + 4              ∎

compact₀-minimizes-padding : ∀ cfg → 0 ≤ paddingWeight cfg
compact₀-minimizes-padding cfg = z≤n

compact₀-minimizes-diameter : ∀ middleWidth cfg → middleWidth + 4 ≤ diameter middleWidth cfg
compact₀-minimizes-diameter middleWidth cfg =
  m≤m+n (middleWidth + 4) (2 * paddingWeight cfg)

------------------------------------------------------------------------
-- Monotone shifts in each coordinate
------------------------------------------------------------------------

paddingWeight-outer-step
  : ∀ kO kI d
  → paddingWeight (kConfig (kO + d) kI) ≡ paddingWeight (kConfig kO kI) + d
paddingWeight-outer-step kO kI d = begin
  paddingWeight (kConfig (kO + d) kI) ≡⟨ refl ⟩
  (kO + d) + kI                       ≡⟨ +-assoc kO d kI ⟩
  kO + (d + kI)                       ≡⟨ cong (kO +_) (+-comm d kI) ⟩
  kO + (kI + d)                       ≡⟨ sym (+-assoc kO kI d) ⟩
  (kO + kI) + d                       ≡⟨ refl ⟩
  paddingWeight (kConfig kO kI) + d   ∎

paddingWeight-inner-step
  : ∀ kO kI d
  → paddingWeight (kConfig kO (kI + d)) ≡ paddingWeight (kConfig kO kI) + d
paddingWeight-inner-step kO kI d = begin
  paddingWeight (kConfig kO (kI + d)) ≡⟨ refl ⟩
  kO + (kI + d)                       ≡⟨ sym (+-assoc kO kI d) ⟩
  (kO + kI) + d                       ≡⟨ refl ⟩
  paddingWeight (kConfig kO kI) + d   ∎

seed-position-outer-step
  : ∀ kO kI d
  → seed-position (kConfig (kO + d) kI) ≡ seed-position (kConfig kO kI) + d
seed-position-outer-step kO kI d = begin
  seed-position (kConfig (kO + d) kI) ≡⟨ cong (_+ 2) (paddingWeight-outer-step kO kI d) ⟩
  (paddingWeight (kConfig kO kI) + d) + 2 ≡⟨ +-assoc (paddingWeight (kConfig kO kI)) d 2 ⟩
  paddingWeight (kConfig kO kI) + (d + 2) ≡⟨ cong (paddingWeight (kConfig kO kI) +_) (+-comm d 2) ⟩
  paddingWeight (kConfig kO kI) + (2 + d) ≡⟨ sym (+-assoc (paddingWeight (kConfig kO kI)) 2 d) ⟩
  (paddingWeight (kConfig kO kI) + 2) + d ≡⟨ refl ⟩
  seed-position (kConfig kO kI) + d       ∎

seed-position-inner-step
  : ∀ kO kI d
  → seed-position (kConfig kO (kI + d)) ≡ seed-position (kConfig kO kI) + d
seed-position-inner-step kO kI d = begin
  seed-position (kConfig kO (kI + d)) ≡⟨ cong (_+ 2) (paddingWeight-inner-step kO kI d) ⟩
  (paddingWeight (kConfig kO kI) + d) + 2 ≡⟨ +-assoc (paddingWeight (kConfig kO kI)) d 2 ⟩
  paddingWeight (kConfig kO kI) + (d + 2) ≡⟨ cong (paddingWeight (kConfig kO kI) +_) (+-comm d 2) ⟩
  paddingWeight (kConfig kO kI) + (2 + d) ≡⟨ sym (+-assoc (paddingWeight (kConfig kO kI)) 2 d) ⟩
  (paddingWeight (kConfig kO kI) + 2) + d ≡⟨ refl ⟩
  seed-position (kConfig kO kI) + d       ∎

inner-right-position-outer-invariant
  : ∀ kO kI d
  → inner-right-position (kConfig (kO + d) kI) ≡ inner-right-position (kConfig kO kI)
inner-right-position-outer-invariant kO kI d = refl

inner-right-position-inner-step
  : ∀ kO kI d
  → inner-right-position (kConfig kO (kI + d)) ≡ inner-right-position (kConfig kO kI) + d
inner-right-position-inner-step kO kI d = begin
  inner-right-position (kConfig kO (kI + d)) ≡⟨ refl ⟩
  (kI + d) + 1                               ≡⟨ +-assoc kI d 1 ⟩
  kI + (d + 1)                               ≡⟨ cong (kI +_) (+-comm d 1) ⟩
  kI + (1 + d)                               ≡⟨ sym (+-assoc kI 1 d) ⟩
  (kI + 1) + d                               ≡⟨ refl ⟩
  inner-right-position (kConfig kO kI) + d   ∎

diameter-outer-step
  : ∀ middleWidth kO kI d
  → diameter middleWidth (kConfig (kO + d) kI) ≡ diameter middleWidth (kConfig kO kI) + 2 * d
diameter-outer-step middleWidth kO kI d = begin
  diameter middleWidth (kConfig (kO + d) kI) ≡⟨ cong (middleWidth + 4 +_) (cong (2 *_) (paddingWeight-outer-step kO kI d)) ⟩
  middleWidth + 4 + 2 * (paddingWeight (kConfig kO kI) + d) ≡⟨ cong (middleWidth + 4 +_) (*-distribˡ-+ 2 (paddingWeight (kConfig kO kI)) d) ⟩
  middleWidth + 4 + (2 * paddingWeight (kConfig kO kI) + 2 * d) ≡⟨ sym (+-assoc (middleWidth + 4) (2 * paddingWeight (kConfig kO kI)) (2 * d)) ⟩
  (middleWidth + 4 + 2 * paddingWeight (kConfig kO kI)) + 2 * d ≡⟨ refl ⟩
  diameter middleWidth (kConfig kO kI) + 2 * d ∎

diameter-inner-step
  : ∀ middleWidth kO kI d
  → diameter middleWidth (kConfig kO (kI + d)) ≡ diameter middleWidth (kConfig kO kI) + 2 * d
diameter-inner-step middleWidth kO kI d = begin
  diameter middleWidth (kConfig kO (kI + d)) ≡⟨ cong (middleWidth + 4 +_) (cong (2 *_) (paddingWeight-inner-step kO kI d)) ⟩
  middleWidth + 4 + 2 * (paddingWeight (kConfig kO kI) + d) ≡⟨ cong (middleWidth + 4 +_) (*-distribˡ-+ 2 (paddingWeight (kConfig kO kI)) d) ⟩
  middleWidth + 4 + (2 * paddingWeight (kConfig kO kI) + 2 * d) ≡⟨ sym (+-assoc (middleWidth + 4) (2 * paddingWeight (kConfig kO kI)) (2 * d)) ⟩
  (middleWidth + 4 + 2 * paddingWeight (kConfig kO kI)) + 2 * d ≡⟨ refl ⟩
  diameter middleWidth (kConfig kO kI) + 2 * d ∎

------------------------------------------------------------------------
-- Small maintained examples
------------------------------------------------------------------------

base14-adjacent : BoundedKConfig
base14-adjacent = kConfig 0 1

base34-boundary-release : BoundedKConfig
base34-boundary-release = kConfig 1 0

base14-adjacent-diameter : diameter 2 base14-adjacent ≡ 8
base14-adjacent-diameter = refl

base34-boundary-release-diameter : diameter 2 base34-boundary-release ≡ 8
base34-boundary-release-diameter = refl
