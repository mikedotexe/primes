import Mathlib
import PrimeArithmetic.Sieve.BoundedByteCoordinates
import PrimeArithmetic.Sieve.SegmentBitMasks

namespace PrimeArithmetic.Sieve

/-!
Bounded single-byte array semantics for the odd-only runtime segment.

The runtime sieve does not only compute a byte/bit coordinate and a mask. It
then performs a concrete write into the selected byte of a bounded bit array.
This module packages that last exact step in a purely functional form:

- pick the unique in-range byte slot for the candidate
- update only that byte with the proved odd-only mask operation
- read back from the same slot using the proved odd-only readback formula

This still avoids full imperative-array reasoning, but it closes the gap
between arithmetic bit identities and the executable one-byte update pattern.
-/

/-- Functional model of the odd-only segment byte array. -/
abbrev SegmentByteState := BoundedByteState segBytes

/-- The unique bounded byte slot used for an in-range odd candidate. -/
def segmentByteSlot {lo limit n : ℕ}
    (hLo : lo ≤ n) (hN : n ≤ rawSegmentHi lo limit) : Fin segBytes :=
  ⟨segmentByteIndex lo n, segmentByteIndex_lt_segBytes_of_le_rawSegmentHi hLo hN⟩

/-- Readback of the target bit from the selected segment byte. -/
def segmentByteRead (bytes : SegmentByteState) {lo limit n : ℕ}
    (hLo : lo ≤ n) (hN : n ≤ rawSegmentHi lo limit) : ℕ :=
  segmentReadValue (bytes (segmentByteSlot hLo hN)) lo n

/-- Single-byte update performed at the selected segment slot. -/
def segmentByteWrite (bytes : SegmentByteState) {lo limit n : ℕ}
    (hLo : lo ≤ n) (hN : n ≤ rawSegmentHi lo limit) : SegmentByteState :=
  let slot := segmentByteSlot hLo hN
  Function.update bytes slot (segmentMarkByte (bytes slot) lo n)

/-- Generic byte-mark view of an in-range odd candidate. -/
def segmentByteMark {lo limit n : ℕ}
    (hLo : lo ≤ n) (hN : n ≤ rawSegmentHi lo limit) : ByteMark segBytes :=
  (segmentByteSlot hLo hN, ⟨segmentBitIndex lo n, segmentBitIndex_lt_eight lo n⟩)

theorem segmentByteRead_eq_byteMarkRead (bytes : SegmentByteState) {lo limit n : ℕ}
    (hLo : lo ≤ n) (hN : n ≤ rawSegmentHi lo limit) :
    segmentByteRead bytes hLo hN = byteMarkRead bytes (segmentByteMark hLo hN) := by
  unfold segmentByteRead segmentByteMark byteMarkRead
  simp [segmentReadValue]

theorem segmentByteWrite_eq_byteMarkWrite (bytes : SegmentByteState) {lo limit n : ℕ}
    (hLo : lo ≤ n) (hN : n ≤ rawSegmentHi lo limit) :
    segmentByteWrite bytes hLo hN = byteMarkWrite bytes (segmentByteMark hLo hN) := by
  funext slot
  unfold segmentByteWrite segmentByteMark byteMarkWrite
  simp [segmentMarkByte, segmentBitMask]

theorem segmentByteRead_written (bytes : SegmentByteState) {lo limit n : ℕ}
    (hLo : lo ≤ n) (hN : n ≤ rawSegmentHi lo limit) :
    segmentByteRead (segmentByteWrite bytes hLo hN) hLo hN = 1 := by
  exact fixedRead_written_of_eq
    (read := fun bytes => segmentByteRead bytes hLo hN)
    (write := fun bytes => segmentByteWrite bytes hLo hN)
    (mark := segmentByteMark hLo hN)
    (hRead := fun bytes => segmentByteRead_eq_byteMarkRead bytes hLo hN)
    (hWrite := fun bytes => segmentByteWrite_eq_byteMarkWrite bytes hLo hN)
    bytes

theorem segmentByteSlot_ne_of_byteIndex_ne {lo limit n m : ℕ}
    (hLoN : lo ≤ n) (hN : n ≤ rawSegmentHi lo limit)
    (hLoM : lo ≤ m) (hM : m ≤ rawSegmentHi lo limit)
    (hByte : segmentByteIndex lo n ≠ segmentByteIndex lo m) :
    segmentByteSlot hLoN hN ≠ segmentByteSlot hLoM hM := by
  intro hEq
  apply hByte
  simpa [segmentByteSlot] using congrArg Fin.val hEq

theorem segmentByteSlot_eq_of_byteIndex_eq {lo limit n m : ℕ}
    (hLoN : lo ≤ n) (hN : n ≤ rawSegmentHi lo limit)
    (hLoM : lo ≤ m) (hM : m ≤ rawSegmentHi lo limit)
    (hByte : segmentByteIndex lo n = segmentByteIndex lo m) :
    segmentByteSlot hLoN hN = segmentByteSlot hLoM hM := by
  apply Fin.ext
  simpa [segmentByteSlot] using hByte

theorem segmentByteRead_write_other_slot_eq (bytes : SegmentByteState)
    {lo limit n m : ℕ}
    (hLoN : lo ≤ n) (hN : n ≤ rawSegmentHi lo limit)
    (hLoM : lo ≤ m) (hM : m ≤ rawSegmentHi lo limit)
    (hSlot : segmentByteSlot hLoN hN ≠ segmentByteSlot hLoM hM) :
    segmentByteRead (segmentByteWrite bytes hLoN hN) hLoM hM =
      segmentByteRead bytes hLoM hM := by
  have hSlot' : segmentByteSlot hLoM hM ≠ segmentByteSlot hLoN hN := hSlot.symm
  unfold segmentByteRead segmentByteWrite
  simp [Function.update, hSlot']

theorem segmentByteRead_write_other_byte_eq (bytes : SegmentByteState)
    {lo limit n m : ℕ}
    (hLoN : lo ≤ n) (hN : n ≤ rawSegmentHi lo limit)
    (hLoM : lo ≤ m) (hM : m ≤ rawSegmentHi lo limit)
    (hByte : segmentByteIndex lo n ≠ segmentByteIndex lo m) :
    segmentByteRead (segmentByteWrite bytes hLoN hN) hLoM hM =
      segmentByteRead bytes hLoM hM := by
  exact segmentByteRead_write_other_slot_eq bytes hLoN hN hLoM hM
    (segmentByteSlot_ne_of_byteIndex_ne hLoN hN hLoM hM hByte)

theorem segmentByteRead_write_other_bit_same_byte_eq (bytes : SegmentByteState)
    {lo limit n m : ℕ}
    (hLoN : lo ≤ n) (hN : n ≤ rawSegmentHi lo limit)
    (hLoM : lo ≤ m) (hM : m ≤ rawSegmentHi lo limit)
    (hByte : segmentByteIndex lo n = segmentByteIndex lo m)
    (hBit : segmentBitIndex lo m ≠ segmentBitIndex lo n) :
    segmentByteRead (segmentByteWrite bytes hLoN hN) hLoM hM =
      segmentByteRead bytes hLoM hM := by
  have hSlot :
      segmentByteSlot hLoN hN = segmentByteSlot hLoM hM :=
    segmentByteSlot_eq_of_byteIndex_eq hLoN hN hLoM hM hByte
  simpa [segmentByteRead, segmentByteWrite, hSlot] using
    (segmentReadValue_marked_other_eq
      (byte := bytes (segmentByteSlot hLoM hM))
      (lo := lo) (n := n) (m := m) hBit)

theorem segmentByteRead_first_of_sequentialSameByteWrites
    (bytes : SegmentByteState)
    {lo limit n m : ℕ}
    (hLoN : lo ≤ n) (hN : n ≤ rawSegmentHi lo limit)
    (hLoM : lo ≤ m) (hM : m ≤ rawSegmentHi lo limit)
    (hByte : segmentByteIndex lo n = segmentByteIndex lo m)
    (hBit : segmentBitIndex lo n ≠ segmentBitIndex lo m) :
    segmentByteRead
        (segmentByteWrite
          (segmentByteWrite bytes hLoN hN)
          hLoM hM)
        hLoN hN = 1 := by
  rw [segmentByteRead_write_other_bit_same_byte_eq
      (bytes := segmentByteWrite bytes hLoN hN)
      (hLoN := hLoM) (hN := hM) (hLoM := hLoN) (hM := hN)
      (hByte := hByte.symm) (hBit := hBit)]
  exact segmentByteRead_written bytes hLoN hN

theorem segmentByteRead_second_of_sequentialSameByteWrites
    (bytes : SegmentByteState)
    {lo limit n m : ℕ}
    (hLoN : lo ≤ n) (hN : n ≤ rawSegmentHi lo limit)
    (hLoM : lo ≤ m) (hM : m ≤ rawSegmentHi lo limit) :
    segmentByteRead
        (segmentByteWrite
          (segmentByteWrite bytes hLoN hN)
          hLoM hM)
        hLoM hM = 1 := by
  exact segmentByteRead_written
    (bytes := segmentByteWrite bytes hLoN hN)
    (hLo := hLoM) (hN := hM)

theorem segmentByteReads_of_sequentialSameByteWrites
    (bytes : SegmentByteState)
    {lo limit n m : ℕ}
    (hLoN : lo ≤ n) (hN : n ≤ rawSegmentHi lo limit)
    (hLoM : lo ≤ m) (hM : m ≤ rawSegmentHi lo limit)
    (hByte : segmentByteIndex lo n = segmentByteIndex lo m)
    (hBit : segmentBitIndex lo n ≠ segmentBitIndex lo m) :
    segmentByteRead
        (segmentByteWrite
          (segmentByteWrite bytes hLoN hN)
          hLoM hM)
        hLoN hN = 1 ∧
      segmentByteRead
        (segmentByteWrite
          (segmentByteWrite bytes hLoN hN)
          hLoM hM)
        hLoM hM = 1 := by
  constructor
  · exact segmentByteRead_first_of_sequentialSameByteWrites
      bytes hLoN hN hLoM hM hByte hBit
  · exact segmentByteRead_second_of_sequentialSameByteWrites
      bytes hLoN hN hLoM hM

end PrimeArithmetic.Sieve
