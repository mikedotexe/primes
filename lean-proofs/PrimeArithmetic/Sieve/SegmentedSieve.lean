import Mathlib

namespace PrimeArithmetic.Sieve

/-!
This module isolates the exact arithmetic used by the odd-only segmented sieve.

It does not formalize the imperative bit-array implementation. Instead it proves
the arithmetic invariants that make that implementation correct:

- odd candidates are encoded by `lo + 2 * idx`
- the inverse index is `(n - lo) / 2`
- marking starts at the first odd multiple of `p` at or above `segLo`
- later marks advance by steps of `2 * p`
-/

def oddSieveCandidate (n : ℕ) : Prop :=
  3 ≤ n ∧ Odd n

def oddSegmentNumber (lo idx : ℕ) : ℕ :=
  lo + 2 * idx

def oddSegmentIndex (lo n : ℕ) : ℕ :=
  (n - lo) / 2

def firstMultipleAtOrAbove (p segLo : ℕ) : ℕ :=
  (segLo ⌈/⌉ p) * p

def firstOddMultipleAtOrAbove (p segLo : ℕ) : ℕ :=
  let start := firstMultipleAtOrAbove p segLo
  if start % 2 = 0 then start + p else start

def markedBy (p segLo step : ℕ) : ℕ :=
  firstOddMultipleAtOrAbove p segLo + step * (2 * p)

theorem oddSegmentIndex_oddSegmentNumber (lo idx : ℕ) :
    oddSegmentIndex lo (oddSegmentNumber lo idx) = idx := by
  simp [oddSegmentIndex, oddSegmentNumber]

theorem oddSegmentNumber_odd {lo idx : ℕ} (hLoOdd : Odd lo) :
    Odd (oddSegmentNumber lo idx) := by
  rcases hLoOdd with ⟨k, hk⟩
  rw [oddSegmentNumber, hk]
  refine ⟨k + idx, ?_⟩
  omega

theorem oddSegmentNumber_ge_lo (lo idx : ℕ) :
    lo ≤ oddSegmentNumber lo idx := by
  simp [oddSegmentNumber]

theorem oddSegmentNumber_succ (lo idx : ℕ) :
    oddSegmentNumber lo (idx + 1) = oddSegmentNumber lo idx + 2 := by
  unfold oddSegmentNumber
  omega

theorem oddSegmentNumber_strictMono (lo : ℕ) :
    StrictMono (oddSegmentNumber lo) := by
  intro idx₁ idx₂ hIdx
  unfold oddSegmentNumber
  omega

theorem oddSegmentNumber_injective (lo : ℕ) :
    Function.Injective (oddSegmentNumber lo) :=
  (oddSegmentNumber_strictMono lo).injective

theorem oddSegmentNumber_oddSegmentIndex {lo n : ℕ}
    (hLoLe : lo ≤ n) (hLoOdd : Odd lo) (hNOdd : Odd n) :
    oddSegmentNumber lo (oddSegmentIndex lo n) = n := by
  rcases (hNOdd.tsub_odd hLoOdd) with ⟨k, hk⟩
  calc
    oddSegmentNumber lo (oddSegmentIndex lo n)
        = lo + 2 * ((n - lo) / 2) := by
          simp [oddSegmentNumber, oddSegmentIndex]
    _ = lo + 2 * k := by
          rw [hk]
          omega
    _ = n := by omega

theorem dvd_firstMultipleAtOrAbove (p segLo : ℕ) :
    p ∣ firstMultipleAtOrAbove p segLo := by
  simp [firstMultipleAtOrAbove]

theorem le_firstMultipleAtOrAbove {p segLo : ℕ} (hp : 0 < p) :
    segLo ≤ firstMultipleAtOrAbove p segLo := by
  simpa [firstMultipleAtOrAbove, Nat.mul_comm] using
    (ceilDiv_le_iff_le_mul hp).1 le_rfl

theorem dvd_firstOddMultipleAtOrAbove (p segLo : ℕ) :
    p ∣ firstOddMultipleAtOrAbove p segLo := by
  by_cases h : firstMultipleAtOrAbove p segLo % 2 = 0
  · simp [firstOddMultipleAtOrAbove, h, dvd_firstMultipleAtOrAbove]
  · simp [firstOddMultipleAtOrAbove, h, dvd_firstMultipleAtOrAbove]

theorem le_firstOddMultipleAtOrAbove {p segLo : ℕ} (hp : 0 < p) :
    segLo ≤ firstOddMultipleAtOrAbove p segLo := by
  by_cases h : firstMultipleAtOrAbove p segLo % 2 = 0
  · simpa [firstOddMultipleAtOrAbove, h] using
      le_trans (le_firstMultipleAtOrAbove hp) (Nat.le_add_right _ _)
  · simpa [firstOddMultipleAtOrAbove, h] using le_firstMultipleAtOrAbove hp

theorem odd_firstOddMultipleAtOrAbove {p segLo : ℕ}
    (hpOdd : Odd p) :
    Odd (firstOddMultipleAtOrAbove p segLo) := by
  by_cases h : firstMultipleAtOrAbove p segLo % 2 = 0
  · have hStartEven : Even (firstMultipleAtOrAbove p segLo) := by
      exact Nat.even_iff.mpr h
    simpa [firstOddMultipleAtOrAbove, h] using hStartEven.add_odd hpOdd
  · have hStartOdd : Odd (firstMultipleAtOrAbove p segLo) := by
      refine (Nat.not_even_iff_odd).mp ?_
      intro hEven
      exact h (Nat.even_iff.mp hEven)
    simpa [firstOddMultipleAtOrAbove, h] using hStartOdd

@[simp] theorem markedBy_zero (p segLo : ℕ) :
    markedBy p segLo 0 = firstOddMultipleAtOrAbove p segLo := by
  simp [markedBy]

theorem markedBy_succ (p segLo step : ℕ) :
    markedBy p segLo (step + 1) = markedBy p segLo step + 2 * p := by
  unfold markedBy
  ring

theorem markedBy_add (p segLo step k : ℕ) :
    markedBy p segLo (step + k) =
      markedBy p segLo step + k * (2 * p) := by
  unfold markedBy
  ring

theorem markedBy_strictMono {p segLo : ℕ} (hp : 0 < p) :
    StrictMono (markedBy p segLo) := by
  intro step₁ step₂ hStep
  unfold markedBy
  have hFactor : 0 < 2 * p := by
    omega
  have hMul :
      step₁ * (2 * p) < step₂ * (2 * p) :=
    Nat.mul_lt_mul_of_pos_right hStep hFactor
  exact Nat.add_lt_add_left hMul (firstOddMultipleAtOrAbove p segLo)

theorem markedBy_injective {p segLo : ℕ} (hp : 0 < p) :
    Function.Injective (markedBy p segLo) :=
  (markedBy_strictMono (p := p) (segLo := segLo) hp).injective

theorem markedBy_add_two_mul (p segLo step k : ℕ) :
    markedBy p segLo (step + k) =
      markedBy p segLo step + 2 * (k * p) := by
  simpa [Nat.mul_comm, Nat.mul_left_comm, Nat.mul_assoc] using
    markedBy_add p segLo step k

theorem dvd_markedBy (p segLo step : ℕ) :
    p ∣ markedBy p segLo step := by
  unfold markedBy
  have hTail : p ∣ step * (2 * p) := by
    refine ⟨step * 2, ?_⟩
    ring
  exact dvd_add (dvd_firstOddMultipleAtOrAbove p segLo) hTail

theorem odd_markedBy {p segLo step : ℕ}
    (hpOdd : Odd p) :
    Odd (markedBy p segLo step) := by
  unfold markedBy
  have hBase : Odd (firstOddMultipleAtOrAbove p segLo) :=
    odd_firstOddMultipleAtOrAbove hpOdd
  have hTailEven : Even (step * (2 * p)) := by
    refine ⟨step * p, ?_⟩
    ring
  exact hBase.add_even hTailEven

theorem le_markedBy_firstOddMultipleAtOrAbove (p segLo step : ℕ) :
    firstOddMultipleAtOrAbove p segLo ≤ markedBy p segLo step := by
  simp [markedBy]

end PrimeArithmetic.Sieve
