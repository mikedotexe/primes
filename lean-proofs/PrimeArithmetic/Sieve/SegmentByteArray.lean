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

end PrimeArithmetic.Sieve
