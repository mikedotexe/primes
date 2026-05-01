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
  unfold wheel30CandidateRead wheel30CandidateMark byteMarkRead wheel30ReadValue
  rw [wheel30BitIndex_candidate (base := base) (cycle := cycle) slot hCycle]
  simp

theorem wheel30CandidateRead_base_invariant (bytes : Wheel30ByteState)
    {base₁ base₂ cycle : ℕ} (slot : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes) :
    wheel30CandidateRead bytes base₁ cycle slot hCycle =
      wheel30CandidateRead bytes base₂ cycle slot hCycle := by
  rw [wheel30CandidateRead_eq_byteMarkRead (bytes := bytes) (base := base₁)
    (cycle := cycle) (slot := slot) (hCycle := hCycle)]
  rw [wheel30CandidateRead_eq_byteMarkRead (bytes := bytes) (base := base₂)
    (cycle := cycle) (slot := slot) (hCycle := hCycle)]

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

theorem wheel30CandidateByteSlot_ne_of_cycle_ne {cycle₁ cycle₂ : ℕ}
    (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂) :
    wheel30CandidateByteSlot cycle₁ hCycle₁ ≠ wheel30CandidateByteSlot cycle₂ hCycle₂ := by
  intro hEq
  apply hCycle
  simpa [wheel30CandidateByteSlot] using congrArg Fin.val hEq

theorem wheel30CandidateRead_write_other_slot_eq (bytes : Wheel30ByteState)
    {base cycleWrite cycleRead : ℕ} (slotWrite slotRead : Fin 8)
    (hCycleWrite : cycleWrite < wheel30SegmentBytes)
    (hCycleRead : cycleRead < wheel30SegmentBytes)
    (hSlot :
      wheel30CandidateByteSlot cycleWrite hCycleWrite ≠
        wheel30CandidateByteSlot cycleRead hCycleRead) :
    wheel30CandidateRead
        (wheel30CandidateWrite bytes cycleWrite slotWrite hCycleWrite)
        base cycleRead slotRead hCycleRead =
      wheel30CandidateRead bytes base cycleRead slotRead hCycleRead := by
  have hSlot' :
      wheel30CandidateByteSlot cycleRead hCycleRead ≠
        wheel30CandidateByteSlot cycleWrite hCycleWrite := hSlot.symm
  unfold wheel30CandidateRead wheel30CandidateWrite
  simp [Function.update, hSlot']

theorem wheel30CandidateRead_write_other_cycle_eq (bytes : Wheel30ByteState)
    {base cycleWrite cycleRead : ℕ} (slotWrite slotRead : Fin 8)
    (hCycleWrite : cycleWrite < wheel30SegmentBytes)
    (hCycleRead : cycleRead < wheel30SegmentBytes)
    (hCycle : cycleWrite ≠ cycleRead) :
    wheel30CandidateRead
        (wheel30CandidateWrite bytes cycleWrite slotWrite hCycleWrite)
        base cycleRead slotRead hCycleRead =
      wheel30CandidateRead bytes base cycleRead slotRead hCycleRead := by
  exact wheel30CandidateRead_write_other_slot_eq bytes slotWrite slotRead
    hCycleWrite hCycleRead
    (wheel30CandidateByteSlot_ne_of_cycle_ne hCycleWrite hCycleRead hCycle)

theorem wheel30CandidateRead_write_other_slot_same_cycle_eq
    (bytes : Wheel30ByteState) (base cycle : ℕ)
    (slotWrite slotRead : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes)
    (hSlot : slotRead ≠ slotWrite) :
    wheel30CandidateRead
        (wheel30CandidateWrite bytes cycle slotWrite hCycle)
        base cycle slotRead hCycle =
      wheel30CandidateRead bytes base cycle slotRead hCycle := by
  unfold wheel30CandidateRead wheel30CandidateWrite
  simpa [Function.update] using
    congrArg (fun value => Option.getD value 0)
      (wheel30ReadValue_marked_other_candidate_eq
        (byte := bytes (wheel30CandidateByteSlot cycle hCycle))
        (base := base) (cycle := cycle)
        (slot := slotWrite) (target := slotRead) hCycle hSlot)

theorem wheel30CandidateRead_first_of_sequentialSameCycleWrites
    (bytes : Wheel30ByteState) (base cycle : ℕ)
    (slot₁ slot₂ : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes)
    (hDistinct : slot₁ ≠ slot₂) :
    wheel30CandidateRead
        (wheel30CandidateWrite
          (wheel30CandidateWrite bytes cycle slot₁ hCycle)
          cycle slot₂ hCycle)
        base cycle slot₁ hCycle = 1 := by
  rw [wheel30CandidateRead_write_other_slot_same_cycle_eq
      (bytes := wheel30CandidateWrite bytes cycle slot₁ hCycle)
      (base := base) (cycle := cycle)
      (slotWrite := slot₂) (slotRead := slot₁)
      (hCycle := hCycle) (hSlot := hDistinct)]
  exact wheel30CandidateRead_written bytes base cycle slot₁ hCycle

theorem wheel30CandidateRead_second_of_sequentialSameCycleWrites
    (bytes : Wheel30ByteState) (base cycle : ℕ)
    (slot₁ slot₂ : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes) :
    wheel30CandidateRead
        (wheel30CandidateWrite
          (wheel30CandidateWrite bytes cycle slot₁ hCycle)
          cycle slot₂ hCycle)
        base cycle slot₂ hCycle = 1 := by
  exact wheel30CandidateRead_written
    (bytes := wheel30CandidateWrite bytes cycle slot₁ hCycle)
    (base := base) (cycle := cycle) (slot := slot₂) (hCycle := hCycle)

theorem wheel30CandidateReads_of_sequentialSameCycleWrites
    (bytes : Wheel30ByteState) (base cycle : ℕ)
    (slot₁ slot₂ : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes)
    (hDistinct : slot₁ ≠ slot₂) :
    wheel30CandidateRead
        (wheel30CandidateWrite
          (wheel30CandidateWrite bytes cycle slot₁ hCycle)
          cycle slot₂ hCycle)
        base cycle slot₁ hCycle = 1 ∧
      wheel30CandidateRead
        (wheel30CandidateWrite
          (wheel30CandidateWrite bytes cycle slot₁ hCycle)
          cycle slot₂ hCycle)
        base cycle slot₂ hCycle = 1 := by
  constructor
  · exact wheel30CandidateRead_first_of_sequentialSameCycleWrites
      bytes base cycle slot₁ slot₂ hCycle hDistinct
  · exact wheel30CandidateRead_second_of_sequentialSameCycleWrites
      bytes base cycle slot₁ slot₂ hCycle

end PrimeArithmetic.Sieve
