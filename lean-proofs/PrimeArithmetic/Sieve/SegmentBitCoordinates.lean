import Mathlib
import PrimeArithmetic.Sieve.RuntimeCollection
import PrimeArithmetic.Sieve.RuntimeCrossOff
import PrimeArithmetic.Sieve.SegmentLayout

namespace PrimeArithmetic.Sieve

/-!
Byte/bit coordinates for the odd-only segment in `src/prime_sieve.rs`.

This is the exact arithmetic content behind the shared writer/reader formulas
used by `mark_composite` and `is_prime`:

- `idx = (odd - lo) / 2`
- `byte = idx / 8`
- `bit = idx % 8`
-/

/-- Byte coordinate used by the runtime odd-only segment. -/
def segmentByteIndex (lo n : ℕ) : ℕ :=
  oddSegmentIndex lo n / 8

/-- Bit coordinate inside the selected byte. -/
def segmentBitIndex (lo n : ℕ) : ℕ :=
  oddSegmentIndex lo n % 8

/-- Combined byte/bit coordinates used by the runtime writer and reader. -/
def segmentBitCoordinates (lo n : ℕ) : ℕ × ℕ :=
  (segmentByteIndex lo n, segmentBitIndex lo n)

theorem segmentBitCoordinates_oddSegmentNumber (lo idx : ℕ) :
    segmentBitCoordinates lo (oddSegmentNumber lo idx) = (idx / 8, idx % 8) := by
  simp [segmentBitCoordinates, segmentByteIndex, segmentBitIndex,
    oddSegmentIndex_oddSegmentNumber]

theorem segmentBitIndex_lt_eight (lo n : ℕ) :
    segmentBitIndex lo n < 8 := by
  unfold segmentBitIndex
  exact Nat.mod_lt _ (by decide)

theorem segmentIndex_eq_byte_mul_add_bit (lo n : ℕ) :
    8 * segmentByteIndex lo n + segmentBitIndex lo n = oddSegmentIndex lo n := by
  unfold segmentByteIndex segmentBitIndex
  simpa [Nat.mul_comm, Nat.add_comm, Nat.add_left_comm, Nat.add_assoc] using
    (Nat.mod_add_div (oddSegmentIndex lo n) 8)

theorem segmentByteIndex_lt_segBytes_of_le_rawSegmentHi {lo limit n : ℕ}
    (hLo : lo ≤ n) (hN : n ≤ rawSegmentHi lo limit) :
    segmentByteIndex lo n < segBytes := by
  have hIdx : oddSegmentIndex lo n < segOdds :=
    oddSegmentIndex_lt_segOdds_of_le_rawSegmentHi hLo hN
  unfold segmentByteIndex
  exact (Nat.div_lt_iff_lt_mul (by decide : 0 < 8)).2 <| by
    simpa [segOdds_eq, segBytes_eq, Nat.mul_comm] using hIdx

theorem segmentByteIndex_lt_segBytes_of_le_runtimeSegmentHi {lo limit n : ℕ}
    (hLo : lo ≤ n) (hN : n ≤ runtimeSegmentHi lo limit) :
    segmentByteIndex lo n < segBytes := by
  have hIdx : oddSegmentIndex lo n < segOdds :=
    oddSegmentIndex_lt_segOdds_of_le_runtimeSegmentHi hLo hN
  unfold segmentByteIndex
  exact (Nat.div_lt_iff_lt_mul (by decide : 0 < 8)).2 <| by
    simpa [segOdds_eq, segBytes_eq, Nat.mul_comm] using hIdx

theorem segmentByteIndex_runtimeSegmentHi_lt_segBytes {lo limit : ℕ}
    (hLoLe : lo ≤ limit) (hLoOdd : Odd lo) :
    segmentByteIndex lo (runtimeSegmentHi lo limit) < segBytes := by
  exact segmentByteIndex_lt_segBytes_of_le_runtimeSegmentHi
    (runtimeSegmentHi_ge_lo hLoLe hLoOdd) le_rfl

theorem segmentBitCoordinates_injective_of_odd
    {lo m n : ℕ} (hLoOdd : Odd lo)
    (hLoM : lo ≤ m) (hMOdd : Odd m)
    (hLoN : lo ≤ n) (hNOdd : Odd n)
    (hEq : segmentBitCoordinates lo m = segmentBitCoordinates lo n) :
    m = n := by
  have hByte : segmentByteIndex lo m = segmentByteIndex lo n := congrArg Prod.fst hEq
  have hBit : segmentBitIndex lo m = segmentBitIndex lo n := congrArg Prod.snd hEq
  have hIdx : oddSegmentIndex lo m = oddSegmentIndex lo n := by
    have hm := segmentIndex_eq_byte_mul_add_bit lo m
    have hn := segmentIndex_eq_byte_mul_add_bit lo n
    omega
  rw [← oddSegmentNumber_oddSegmentIndex hLoM hLoOdd hMOdd,
      ← oddSegmentNumber_oddSegmentIndex hLoN hLoOdd hNOdd,
      hIdx]

theorem segmentMarkQuery_same_coordinates (lo n : ℕ) :
    segmentBitCoordinates lo n =
      (oddSegmentIndex lo n / 8, oddSegmentIndex lo n % 8) := by
  rfl

theorem oddSegmentIndex_add_two_mul_of_odd {lo n k : ℕ}
    (hLo : lo ≤ n) (hLoOdd : Odd lo) (hNOdd : Odd n) :
    oddSegmentIndex lo (n + 2 * k) = oddSegmentIndex lo n + k := by
  rcases (hNOdd.tsub_odd hLoOdd) with ⟨t, ht⟩
  unfold oddSegmentIndex
  rw [ht]
  omega

theorem oddSegmentIndex_runtimeMarkedBy_add {lo p segLo step k : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hLoOdd : Odd lo) (hpOdd : Odd p) :
    oddSegmentIndex lo (runtimeMarkedBy p segLo (step + k)) =
      oddSegmentIndex lo (runtimeMarkedBy p segLo step) + k * p := by
  rw [runtimeMarkedBy_add_two_mul]
  simpa [Nat.mul_comm, Nat.mul_left_comm, Nat.mul_assoc] using
    (oddSegmentIndex_add_two_mul_of_odd
      (n := runtimeMarkedBy p segLo step) (k := k * p)
      hLo hLoOdd (odd_runtimeMarkedBy hpOdd))

theorem oddSegmentIndex_runtimeMarkedBy_succ {lo p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hLoOdd : Odd lo) (hpOdd : Odd p) :
    oddSegmentIndex lo (runtimeMarkedBy p segLo (step + 1)) =
      oddSegmentIndex lo (runtimeMarkedBy p segLo step) + p := by
  simpa using
    (oddSegmentIndex_runtimeMarkedBy_add
      (step := step) (k := 1) hLo hLoOdd hpOdd)

theorem segmentBitCoordinates_runtimeMarkedBy_add {lo p segLo step k : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hLoOdd : Odd lo) (hpOdd : Odd p) :
    segmentBitCoordinates lo (runtimeMarkedBy p segLo (step + k)) =
      ((oddSegmentIndex lo (runtimeMarkedBy p segLo step) + k * p) / 8,
        (oddSegmentIndex lo (runtimeMarkedBy p segLo step) + k * p) % 8) := by
  simp [segmentBitCoordinates, segmentByteIndex, segmentBitIndex,
    oddSegmentIndex_runtimeMarkedBy_add hLo hLoOdd hpOdd]

theorem segmentByteIndex_ne_of_add_two_mul_ge_eight {lo n k : ℕ}
    (hLo : lo ≤ n) (hLoOdd : Odd lo) (hNOdd : Odd n)
    (hk : 8 ≤ k) :
    segmentByteIndex lo n ≠ segmentByteIndex lo (n + 2 * k) := by
  intro hEq
  have hIdx :
      oddSegmentIndex lo (n + 2 * k) = oddSegmentIndex lo n + k :=
    oddSegmentIndex_add_two_mul_of_odd hLo hLoOdd hNOdd
  have hBitLo : segmentBitIndex lo n < 8 := segmentBitIndex_lt_eight lo n
  have hBitHi : segmentBitIndex lo (n + 2 * k) < 8 := segmentBitIndex_lt_eight lo (n + 2 * k)
  have hDecompLo := segmentIndex_eq_byte_mul_add_bit lo n
  have hDecompHi := segmentIndex_eq_byte_mul_add_bit lo (n + 2 * k)
  omega

theorem segmentByteIndex_runtimeMarkedBy_add_ne_of_eight_le_mul
    {lo p segLo step k : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hk : 8 ≤ k * p) :
    segmentByteIndex lo (runtimeMarkedBy p segLo step) ≠
      segmentByteIndex lo (runtimeMarkedBy p segLo (step + k)) := by
  rw [runtimeMarkedBy_add_two_mul]
  exact segmentByteIndex_ne_of_add_two_mul_ge_eight
    hLo hLoOdd (odd_runtimeMarkedBy hpOdd) hk

end PrimeArithmetic.Sieve
