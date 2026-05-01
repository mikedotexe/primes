import Mathlib
import PrimeArithmetic.Sieve.SegmentBitCoordinates

namespace PrimeArithmetic.Sieve

/-!
Bit-mask semantics for the odd-only runtime segment in `src/prime_sieve.rs`.

This module matches the executable writer/readback expressions exactly:

- writer: `bits[idx >> 3] |= 1 << (idx & 7)`
- reader: `(bits[idx >> 3] >> (idx & 7)) & 1`

The previous sieve modules already fixed the odd candidate index
`idx = (n - lo) / 2` and the shared byte/bit coordinates `idx / 8`, `idx % 8`.
Here we prove that the actual mask update and the actual readback test target
the same bit.
-/

/-- Bit mask used by the runtime odd-only segment writer. -/
def segmentBitMask (lo n : ℕ) : ℕ :=
  1 <<< segmentBitIndex lo n

/-- Byte update performed by `mark_composite` once the target byte is selected. -/
def segmentMarkByte (byte lo n : ℕ) : ℕ :=
  byte ||| segmentBitMask lo n

/-- Exact numeric readback performed by the runtime query expression. -/
def segmentReadValue (byte lo n : ℕ) : ℕ :=
  (byte >>> segmentBitIndex lo n) &&& 1

/-- Boolean version of the same readback, phrased as a bit test. -/
def segmentReadBit (byte lo n : ℕ) : Bool :=
  byte.testBit (segmentBitIndex lo n)

theorem segmentBitMask_eq_two_pow (lo n : ℕ) :
    segmentBitMask lo n = 2 ^ segmentBitIndex lo n := by
  simp [segmentBitMask, Nat.shiftLeft_eq]

theorem segmentBitMask_targets_self (lo n : ℕ) :
    (segmentBitMask lo n).testBit (segmentBitIndex lo n) = true := by
  rw [segmentBitMask_eq_two_pow]
  have hdec : decide (segmentBitIndex lo n = segmentBitIndex lo n) = true := by
    simp
  rw [← hdec]
  exact Nat.testBit_two_pow

theorem segmentBitMask_targets_other (lo n j : ℕ)
    (h : j ≠ segmentBitIndex lo n) :
    (segmentBitMask lo n).testBit j = false := by
  rw [segmentBitMask_eq_two_pow]
  exact Nat.testBit_two_pow_of_ne h.symm

theorem segmentReadValue_eq_toNat_readBit (byte lo n : ℕ) :
    segmentReadValue byte lo n = (segmentReadBit byte lo n).toNat := by
  unfold segmentReadValue segmentReadBit
  simpa [Nat.testBit] using
    (Nat.and_two_pow (byte >>> segmentBitIndex lo n) 0)

theorem segmentMarkByte_sets_target_bit (byte lo n : ℕ) :
    segmentReadBit (segmentMarkByte byte lo n) lo n = true := by
  unfold segmentReadBit segmentMarkByte
  rw [Nat.testBit_lor]
  simp [segmentBitMask_targets_self]

theorem segmentMarkByte_preserves_other_bits (byte lo n j : ℕ)
    (h : j ≠ segmentBitIndex lo n) :
    (segmentMarkByte byte lo n).testBit j = byte.testBit j := by
  unfold segmentMarkByte
  rw [Nat.testBit_lor, segmentBitMask_targets_other _ _ _ h]
  simp

theorem segmentReadBit_marked_other_eq (byte lo n m : ℕ)
    (h : segmentBitIndex lo m ≠ segmentBitIndex lo n) :
    segmentReadBit (segmentMarkByte byte lo n) lo m =
      segmentReadBit byte lo m := by
  unfold segmentReadBit
  simpa using
    (segmentMarkByte_preserves_other_bits
      (byte := byte) (lo := lo) (n := n) (j := segmentBitIndex lo m) h)

theorem segmentReadValue_marked_other_eq (byte lo n m : ℕ)
    (h : segmentBitIndex lo m ≠ segmentBitIndex lo n) :
    segmentReadValue (segmentMarkByte byte lo n) lo m =
      segmentReadValue byte lo m := by
  rw [segmentReadValue_eq_toNat_readBit, segmentReadValue_eq_toNat_readBit]
  simp [segmentReadBit_marked_other_eq (byte := byte) (lo := lo) (n := n) (m := m) h]

theorem segmentReadValue_marked (byte lo n : ℕ) :
    segmentReadValue (segmentMarkByte byte lo n) lo n = 1 := by
  rw [segmentReadValue_eq_toNat_readBit]
  simp [segmentMarkByte_sets_target_bit]

end PrimeArithmetic.Sieve
