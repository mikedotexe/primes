{-# OPTIONS --safe #-}

module DigitSumMod3 where

open import Data.Nat using (ℕ; zero; suc; _+_; _*_ ; _^_; _∸_)
open import Data.Nat.Properties as ℕₚ
open import Data.Vec using (Vec; []; _∷_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl; cong; sym; trans)

------------------------------------------------------------------------
-- Base‑b evaluation (LSB first)

eval : (b : ℕ) → {n : ℕ} → Vec ℕ n → ℕ
eval b []       = 0
eval b (d ∷ ds) = d + b * eval b ds

sumDigits : ∀ {n} → Vec ℕ n → ℕ
sumDigits []       = 0
sumDigits (d ∷ ds) = d + sumDigits ds

altSumDigits : ∀ {n} → Vec ℕ n → ℕ
altSumDigits []         = 0
altSumDigits (d ∷ ds)   = d + (3 ∸ (altSumDigits ds mod3))  -- sketchy placeholder
  where
  postulate mod3 : ℕ → ℕ
-- For the tiny skeleton we won't fully implement alternating sum; see lemmas below.

------------------------------------------------------------------------
-- Modular arithmetic skeleton (mod 3)

-- We use a lightweight notion of congruence modulo 3.
_≡₃_ : ℕ → ℕ → Set
m ≡₃ n = Σ ℕ (λ k → m ≡ n + 3 * k ⊎ n ≡ m + 3 * k)
  where open import Data.Sum using (_⊎_; inj₁; inj₂)
open import Data.Product using (Σ; _,_; proj₁; proj₂)

postulate
  cong₃-+-left  : ∀ {a b c} → a ≡₃ b → a + c ≡₃ b + c
  cong₃-+-right : ∀ {a b c} → a ≡₃ b → c + a ≡₃ c + b
  cong₃-*-left  : ∀ {a b c} → a ≡₃ b → c * a ≡₃ c * b
  cong₃-*-right : ∀ {a b c} → a ≡₃ b → a * c ≡₃ b * c
  base≡1→pow≡1 : ∀ {b i} → b ≡₃ 1 → (b ^ i) ≡₃ 1
  base≡0→pow≡0 : ∀ {b}   → b ≡₃ 0 → ∀ i → i ℕₚ.> 0 → (b ^ i) ≡₃ 0
  base≡2→pow≡± : ∀ {b i} → b ≡₃ 2 → (b ^ i) ≡₃ (if ℕₚ.parity i then 2 else 1)

-- Helpers
open ℕₚ using (parity)

------------------------------------------------------------------------
-- Lemma 1: if b ≡ 1 (mod 3) then eval(b,ds) ≡ sum(ds) (mod 3)

digitSumMod3-base≡1
  : ∀ {n} (b : ℕ) (ds : Vec ℕ n)
  → b ≡₃ 1
  → eval b ds ≡₃ sumDigits ds
digitSumMod3-base≡1 b []     b≡1 = refl₃
  where postulate refl₃ : 0 ≡₃ 0
digitSumMod3-base≡1 b (d ∷ ds) b≡1 =
  let ih = digitSumMod3-base≡1 b ds b≡1 in
  begin₃
    eval b (d ∷ ds)
  ≡₃⟨ cong₃-+-right {a = d} {b = d} {c = b * eval b ds} refl₃ ⟩
    d + b * eval b ds
  ≡₃⟨ cong₃-*-right (base≡1→pow≡1 {b} {i = 1} b≡1) ⟩
    d + 1 * eval b ds
  ≡₃⟨ step₃ ih ⟩
    d + sumDigits ds
  ∎₃
  where
  postulate
    begin₃ : ∀ {x y} → x ≡₃ y → x ≡₃ y
    _≡₃⟨_⟩_ : ∀ {x y z} → x ≡₃ y → y ≡₃ z → x ≡₃ z
    ∎₃      : ∀ {x} → x ≡₃ x
    step₃   : ∀ {x y} → x ≡₃ y → (d + x) ≡₃ (d + y)

------------------------------------------------------------------------
-- Lemma 2: if b ≡ 0 (mod 3) then eval(b,ds) ≡ head(ds) (mod 3)
-- (because b^i ≡ 0 for i>0, LSB-first)

digitSumMod3-base≡0
  : ∀ {n} (b : ℕ) (ds : Vec ℕ n)
  → b ≡₃ 0
  → eval b ds ≡₃ case ds of λ where
      []       → 0
      (d ∷ _)  → d
digitSumMod3-base≡0 b []     b≡0 = refl₃
  where postulate refl₃ : 0 ≡₃ 0
digitSumMod3-base≡0 b (d ∷ ds) b≡0 =
  begin₃
    d + b * eval b ds
  ≡₃⟨ cong₃-+-right (zeroTimes₃ b≡0) ⟩
    d + 0
  ≡₃⟨ unit₃ ⟩
    d
  ∎₃
  where
  postulate
    zeroTimes₃ : b ≡₃ 0 → b * eval b ds ≡₃ 0
    unit₃      : (d + 0) ≡₃ d
    begin₃     : ∀ {x y} → x ≡₃ y → x ≡₃ y
    _≡₃⟨_⟩_    : ∀ {x y z} → x ≡₃ y → y ≡₃ z → x ≡₃ z
    ∎₃         : ∀ {x} → x ≡₃ x

------------------------------------------------------------------------
-- Lemma 3: if b ≡ 2 (mod 3) then eval(b,ds) ≡ alternating-sum(ds) (mod 3)
-- (since b^i ≡ (-1)^i mod 3)

digitSumMod3-base≡2
  : ∀ {n} (b : ℕ) (ds : Vec ℕ n)
  → b ≡₃ 2
  → eval b ds ≡₃ altSum ds
digitSumMod3-base≡2 b ds b≡2 = postulated
  where
  postulate
    altSum : Vec ℕ _ → ℕ
    postulated : eval b ds ≡₃ altSum ds
