import Mathlib
import PrimeArithmetic.Sieve.BoundedByteCoordinates
import PrimeArithmetic.Sieve.Wheel30BitMasks

namespace PrimeArithmetic.Sieve

/-!
Bounded single-byte array semantics for the wheel30 runtime segment.

The runtime wheel30 code stores one byte per wheel cycle and one bit per slot
inside that byte. This module packages the exact one-byte update/readback step
for candidate positions already expressed in the runtime `(cycle, slot)` form.

As in `SegmentByteArray.lean`, this remains purely functional and deliberately
stops before any larger mutable-array proof.
-/

/-- Functional model of the wheel30 segment byte array. -/
abbrev Wheel30ByteState := BoundedByteState wheel30SegmentBytes

/-- The bounded byte slot attached to a runtime wheel cycle. -/
def wheel30CandidateByteSlot (cycle : ℕ)
    (hCycle : cycle < wheel30SegmentBytes) : Fin wheel30SegmentBytes :=
  ⟨cycle, hCycle⟩

/-- Readback of the target wheel30 bit from the selected byte slot. -/
def wheel30CandidateRead (bytes : Wheel30ByteState) (base cycle : ℕ)
    (slot : Fin 8) (hCycle : cycle < wheel30SegmentBytes) : ℕ :=
  Option.getD
    (wheel30ReadValue (bytes (wheel30CandidateByteSlot cycle hCycle)) base
      (wheel30Candidate base cycle slot)) 0

/-- Single-byte update performed at the selected wheel30 cycle slot. -/
def wheel30CandidateWrite (bytes : Wheel30ByteState) (cycle : ℕ)
    (slot : Fin 8) (hCycle : cycle < wheel30SegmentBytes) : Wheel30ByteState :=
  let byteSlot := wheel30CandidateByteSlot cycle hCycle
  Function.update bytes byteSlot (bytes byteSlot ||| (1 <<< slot.1))

/-- Generic byte-mark view of a runtime wheel30 candidate. -/
def wheel30CandidateMark (cycle : ℕ) (slot : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes) : ByteMark wheel30SegmentBytes :=
  (wheel30CandidateByteSlot cycle hCycle, slot)

theorem wheel30CandidateRead_eq_byteMarkRead (bytes : Wheel30ByteState) (base cycle : ℕ)
    (slot : Fin 8) (hCycle : cycle < wheel30SegmentBytes) :
    wheel30CandidateRead bytes base cycle slot hCycle =
      byteMarkRead bytes (wheel30CandidateMark cycle slot hCycle) := by
  unfold wheel30CandidateRead wheel30CandidateMark byteMarkRead
  simp [wheel30ReadValue, wheel30BitIndex_candidate]

theorem wheel30CandidateWrite_eq_byteMarkWrite (bytes : Wheel30ByteState) (cycle : ℕ)
    (slot : Fin 8) (hCycle : cycle < wheel30SegmentBytes) :
    wheel30CandidateWrite bytes cycle slot hCycle =
      byteMarkWrite bytes (wheel30CandidateMark cycle slot hCycle) := by
  funext byteSlot
  unfold wheel30CandidateWrite wheel30CandidateMark byteMarkWrite
  simp [Function.update]

theorem wheel30CandidateRead_written (bytes : Wheel30ByteState) (base cycle : ℕ)
    (slot : Fin 8) (hCycle : cycle < wheel30SegmentBytes) :
    wheel30CandidateRead (wheel30CandidateWrite bytes cycle slot hCycle) base cycle slot hCycle = 1 := by
  exact fixedRead_written_of_eq
    (read := fun bytes => wheel30CandidateRead bytes base cycle slot hCycle)
    (write := fun bytes => wheel30CandidateWrite bytes cycle slot hCycle)
    (mark := wheel30CandidateMark cycle slot hCycle)
    (hRead := fun bytes => wheel30CandidateRead_eq_byteMarkRead bytes base cycle slot hCycle)
    (hWrite := fun bytes => wheel30CandidateWrite_eq_byteMarkWrite bytes cycle slot hCycle)
    bytes

end PrimeArithmetic.Sieve
