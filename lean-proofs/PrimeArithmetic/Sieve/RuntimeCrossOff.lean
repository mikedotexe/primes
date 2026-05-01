import Mathlib
import PrimeArithmetic.Sieve.SegmentedSieve

namespace PrimeArithmetic.Sieve

/-!
Runtime-facing cross-off arithmetic for `src/prime_sieve.rs`.

This module matches the actual segmented-sieve marking loop more closely than
`SegmentedSieve.lean`:

- the runtime starts from `p^2` once `p^2` enters the segment
- otherwise it starts from the first multiple at or above `segLo`
- even starts are shifted by one extra `p`
- later marks advance by `2 * p`
-/

/-- Arithmetic start used by the runtime before the oddness correction. -/
def runtimeCrossOffBase (p segLo : ℕ) : ℕ :=
  if p * p >= segLo then p * p else firstMultipleAtOrAbove p segLo

/-- First odd cross-off used by the runtime marking loop. -/
def runtimeCrossOffStart (p segLo : ℕ) : ℕ :=
  let start := runtimeCrossOffBase p segLo
  if start % 2 = 0 then start + p else start

/-- Exact arithmetic progression marked by the runtime cross-off loop. -/
def runtimeMarkedBy (p segLo step : ℕ) : ℕ :=
  runtimeCrossOffStart p segLo + step * (2 * p)

theorem runtimeCrossOffBase_eq_sq_of_sq_ge {p segLo : ℕ} (hSq : segLo ≤ p * p) :
    runtimeCrossOffBase p segLo = p * p := by
  simp [runtimeCrossOffBase, hSq]

theorem runtimeCrossOffBase_eq_firstMultiple_of_sq_lt {p segLo : ℕ} (hSq : p * p < segLo) :
    runtimeCrossOffBase p segLo = firstMultipleAtOrAbove p segLo := by
  simp [runtimeCrossOffBase, Nat.not_le.mpr hSq]

theorem runtimeCrossOffStart_eq_sq_of_sq_ge {p segLo : ℕ}
    (hpOdd : Odd p) (hSq : segLo ≤ p * p) :
    runtimeCrossOffStart p segLo = p * p := by
  have hOddSq : Odd (p * p) := hpOdd.mul hpOdd
  have hMod : (p * p) % 2 ≠ 0 := by
    rw [Nat.odd_iff] at hOddSq
    omega
  simp [runtimeCrossOffStart, runtimeCrossOffBase, hSq, hMod]

theorem runtimeCrossOffStart_eq_firstOddMultiple_of_sq_lt {p segLo : ℕ}
    (hSq : p * p < segLo) :
    runtimeCrossOffStart p segLo = firstOddMultipleAtOrAbove p segLo := by
  simp [runtimeCrossOffStart, runtimeCrossOffBase, firstOddMultipleAtOrAbove,
    Nat.not_le.mpr hSq]

theorem runtimeMarkedBy_eq_sq_add_two_mul_of_sq_ge {p segLo step : ℕ}
    (hpOdd : Odd p) (hSq : segLo ≤ p * p) :
    runtimeMarkedBy p segLo step = p * p + 2 * (step * p) := by
  rw [runtimeMarkedBy, runtimeCrossOffStart_eq_sq_of_sq_ge hpOdd hSq]
  ring

theorem runtimeMarkedBy_eq_markedBy_of_sq_lt {p segLo step : ℕ}
    (hSq : p * p < segLo) :
    runtimeMarkedBy p segLo step = markedBy p segLo step := by
  simp [runtimeMarkedBy, markedBy,
    runtimeCrossOffStart_eq_firstOddMultiple_of_sq_lt hSq]

theorem dvd_runtimeCrossOffStart (p segLo : ℕ) :
    p ∣ runtimeCrossOffStart p segLo := by
  by_cases hSq : segLo ≤ p * p
  · by_cases hEven : (p * p) % 2 = 0
    · simp [runtimeCrossOffStart, runtimeCrossOffBase, hSq, hEven]
    · simp [runtimeCrossOffStart, runtimeCrossOffBase, hSq, hEven]
  · have hSqLt : p * p < segLo := Nat.lt_of_not_ge hSq
    simpa [runtimeCrossOffStart_eq_firstOddMultiple_of_sq_lt hSqLt] using
      dvd_firstOddMultipleAtOrAbove p segLo

theorem le_runtimeCrossOffStart {p segLo : ℕ} (hp : 0 < p) :
    segLo ≤ runtimeCrossOffStart p segLo := by
  by_cases hSq : segLo ≤ p * p
  · have hBase : segLo ≤ runtimeCrossOffBase p segLo := by
      simp [runtimeCrossOffBase, hSq]
    by_cases hEven : (runtimeCrossOffBase p segLo) % 2 = 0
    · simp [runtimeCrossOffStart, hEven]
      exact le_trans hBase (Nat.le_add_right _ _)
    · simp [runtimeCrossOffStart, hEven]
      exact hBase
  · have hSqLt : p * p < segLo := Nat.lt_of_not_ge hSq
    simpa [runtimeCrossOffStart_eq_firstOddMultiple_of_sq_lt hSqLt] using
      le_firstOddMultipleAtOrAbove hp

theorem runtimeCrossOffStart_ge_sq (p segLo : ℕ) :
    p * p ≤ runtimeCrossOffStart p segLo := by
  by_cases hSq : segLo ≤ p * p
  · by_cases hEvenSq : (p * p) % 2 = 0
    · simp [runtimeCrossOffStart, runtimeCrossOffBase, hSq, hEvenSq]
    · simp [runtimeCrossOffStart, runtimeCrossOffBase, hSq, hEvenSq]
  · have hSqLt : p * p < segLo := Nat.lt_of_not_ge hSq
    by_cases hp0 : p = 0
    · subst hp0
      exact Nat.zero_le _
    · exact le_trans (Nat.le_of_lt hSqLt)
        (le_runtimeCrossOffStart (Nat.pos_of_ne_zero hp0))

theorem odd_runtimeCrossOffStart {p segLo : ℕ} (hpOdd : Odd p) :
    Odd (runtimeCrossOffStart p segLo) := by
  by_cases hSq : segLo ≤ p * p
  · simpa [runtimeCrossOffStart_eq_sq_of_sq_ge hpOdd hSq] using hpOdd.mul hpOdd
  · have hSqLt : p * p < segLo := Nat.lt_of_not_ge hSq
    simpa [runtimeCrossOffStart_eq_firstOddMultiple_of_sq_lt hSqLt] using
      odd_firstOddMultipleAtOrAbove hpOdd

@[simp] theorem runtimeMarkedBy_zero (p segLo : ℕ) :
    runtimeMarkedBy p segLo 0 = runtimeCrossOffStart p segLo := by
  simp [runtimeMarkedBy]

theorem runtimeMarkedBy_succ (p segLo step : ℕ) :
    runtimeMarkedBy p segLo (step + 1) = runtimeMarkedBy p segLo step + 2 * p := by
  unfold runtimeMarkedBy
  ring

theorem runtimeMarkedBy_add (p segLo step k : ℕ) :
    runtimeMarkedBy p segLo (step + k) =
      runtimeMarkedBy p segLo step + k * (2 * p) := by
  unfold runtimeMarkedBy
  ring

theorem runtimeMarkedBy_strictMono {p segLo : ℕ} (hp : 0 < p) :
    StrictMono (runtimeMarkedBy p segLo) := by
  intro step₁ step₂ hStep
  unfold runtimeMarkedBy
  have hFactor : 0 < 2 * p := by
    omega
  have hMul :
      step₁ * (2 * p) < step₂ * (2 * p) :=
    Nat.mul_lt_mul_of_pos_right hStep hFactor
  exact Nat.add_lt_add_left hMul (runtimeCrossOffStart p segLo)

theorem runtimeMarkedBy_injective {p segLo : ℕ} (hp : 0 < p) :
    Function.Injective (runtimeMarkedBy p segLo) :=
  (runtimeMarkedBy_strictMono (p := p) (segLo := segLo) hp).injective

theorem runtimeMarkedBy_add_two_mul (p segLo step k : ℕ) :
    runtimeMarkedBy p segLo (step + k) =
      runtimeMarkedBy p segLo step + 2 * (k * p) := by
  simpa [Nat.mul_comm, Nat.mul_left_comm, Nat.mul_assoc] using
    runtimeMarkedBy_add p segLo step k

theorem dvd_runtimeMarkedBy (p segLo step : ℕ) :
    p ∣ runtimeMarkedBy p segLo step := by
  unfold runtimeMarkedBy
  have hTail : p ∣ step * (2 * p) := by
    refine ⟨step * 2, ?_⟩
    ring
  exact dvd_add (dvd_runtimeCrossOffStart p segLo) hTail

theorem odd_runtimeMarkedBy {p segLo step : ℕ}
    (hpOdd : Odd p) :
    Odd (runtimeMarkedBy p segLo step) := by
  unfold runtimeMarkedBy
  have hBase : Odd (runtimeCrossOffStart p segLo) := odd_runtimeCrossOffStart hpOdd
  have hTailEven : Even (step * (2 * p)) := by
    refine ⟨step * p, ?_⟩
    ring
  exact hBase.add_even hTailEven

theorem le_runtimeMarkedBy_start (p segLo step : ℕ) :
    runtimeCrossOffStart p segLo ≤ runtimeMarkedBy p segLo step := by
  simp [runtimeMarkedBy]

end PrimeArithmetic.Sieve
