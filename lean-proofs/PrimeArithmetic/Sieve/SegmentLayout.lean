import Mathlib
import PrimeArithmetic.Sieve.SegmentedSieve

namespace PrimeArithmetic.Sieve

/-!
Runtime-facing arithmetic for the odd-only bit-segment layout used by
`src/prime_sieve.rs`.

This stays on the exact candidate/index side of the implementation:

- the segment constants match the Rust values
- the last valid odd index corresponds to the expected arithmetic span
- indices of candidates inside a segment stay below the segment capacity
-/

/-- Segment byte count used by the runtime odd-only sieve. -/
def segBytes : ℕ := 32 * 1024

/-- Bit count in one segment. -/
def segBits : ℕ := segBytes * 8

/-- Number of odd candidates represented in one segment. -/
def segOdds : ℕ := segBits

/-- Arithmetic span covered by one odd-only segment. -/
def oddSegmentSpan : ℕ := 2 * segOdds - 2

/-- Raw arithmetic upper bound before any separate oddness adjustment. -/
def rawSegmentHi (lo limit : ℕ) : ℕ :=
  min (lo + oddSegmentSpan) limit

@[simp] theorem segBytes_eq : segBytes = 32768 := by
  native_decide

@[simp] theorem segBits_eq : segBits = 262144 := by
  native_decide

@[simp] theorem segOdds_eq : segOdds = 262144 := by
  native_decide

@[simp] theorem oddSegmentSpan_eq : oddSegmentSpan = 524286 := by
  native_decide

theorem oddSegmentNumber_last_index (lo : ℕ) :
    oddSegmentNumber lo (segOdds - 1) = lo + oddSegmentSpan := by
  unfold oddSegmentNumber oddSegmentSpan segOdds segBits segBytes
  omega

theorem oddSegmentNumber_le_hi_of_index_lt_segOdds {lo idx : ℕ}
    (hIdx : idx < segOdds) :
    oddSegmentNumber lo idx ≤ lo + oddSegmentSpan := by
  unfold oddSegmentNumber oddSegmentSpan segOdds segBits segBytes at *
  omega

theorem rawSegmentHi_le_limit (lo limit : ℕ) :
    rawSegmentHi lo limit ≤ limit := by
  unfold rawSegmentHi
  exact Nat.min_le_right _ _

theorem rawSegmentHi_le_lo_plus_span (lo limit : ℕ) :
    rawSegmentHi lo limit ≤ lo + oddSegmentSpan := by
  unfold rawSegmentHi
  exact Nat.min_le_left _ _

theorem rawSegmentHi_ge_lo {lo limit : ℕ} (hLoLe : lo ≤ limit) :
    lo ≤ rawSegmentHi lo limit := by
  unfold rawSegmentHi
  exact le_min (by
    unfold oddSegmentSpan segOdds segBits segBytes
    omega) hLoLe

theorem oddSegmentIndex_lt_segOdds_of_le_span {lo n : ℕ}
    (hLo : lo ≤ n) (hN : n ≤ lo + oddSegmentSpan) :
    oddSegmentIndex lo n < segOdds := by
  unfold oddSegmentIndex oddSegmentSpan segOdds segBits segBytes at *
  omega

theorem oddSegmentIndex_lt_segOdds_of_le_rawSegmentHi {lo limit n : ℕ}
    (hLo : lo ≤ n) (hN : n ≤ rawSegmentHi lo limit) :
    oddSegmentIndex lo n < segOdds := by
  exact oddSegmentIndex_lt_segOdds_of_le_span hLo
    (le_trans hN (rawSegmentHi_le_lo_plus_span lo limit))

theorem rawSegmentHi_eq_last_candidate_of_large_limit {lo limit : ℕ}
    (hLarge : lo + oddSegmentSpan ≤ limit) :
    rawSegmentHi lo limit = oddSegmentNumber lo (segOdds - 1) := by
  rw [show rawSegmentHi lo limit = lo + oddSegmentSpan by
    unfold rawSegmentHi
    exact Nat.min_eq_left hLarge]
  exact (oddSegmentNumber_last_index lo).symm

end PrimeArithmetic.Sieve
