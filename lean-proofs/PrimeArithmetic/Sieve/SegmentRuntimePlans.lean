import Mathlib
import PrimeArithmetic.Sieve.BoundedByteCoordinates
import PrimeArithmetic.Sieve.RuntimeCrossOff
import PrimeArithmetic.Sieve.SegmentByteArray

namespace PrimeArithmetic.Sieve

/-!
Short odd-only runtime mark families on the grouped byte-plan layer.

This module keeps the runtime-facing surface close to the segmented sieve:

- a runtime coordinate is an in-range odd candidate in the current segment
- such a coordinate maps to the same `ByteMark` already used by the
  single-candidate byte-array shell
- grouped runtime plans are lists of per-byte coordinate buckets

The main theorem is a direct instantiation of `BoundedByteCoordinates.lean`:
if the grouped plans are aligned with the byte slots they claim to target and
their byte slots are pairwise distinct, then every planned runtime coordinate
reads back as marked after the whole family update.
-/

/-- Runtime odd-only coordinates for one bounded segment. -/
abbrev SegmentRuntimeCoord (lo limit : ℕ) := {n : ℕ // lo ≤ n ∧ n ≤ rawSegmentHi lo limit}

/-- Runtime-step coordinates produced by the executable cross-off progression. -/
def segmentRuntimeCoordOfStep {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit) :
    SegmentRuntimeCoord lo limit :=
  ⟨runtimeMarkedBy p segLo step, ⟨hLo, hHi⟩⟩

/-- Byte-mark view of an in-range runtime odd-only coordinate. -/
def segmentRuntimeMark {lo limit : ℕ}
    (coord : SegmentRuntimeCoord lo limit) : ByteMark segBytes :=
  segmentByteMark coord.2.1 coord.2.2

/-- Grouped runtime odd-only plans bucketed by target byte slot. -/
abbrev SegmentRuntimePlan (lo limit : ℕ) := CoordinatePlan (SegmentRuntimeCoord lo limit) segBytes

theorem segmentRuntimeRead_eq_byteMarkRead (bytes : SegmentByteState) {lo limit : ℕ}
    (coord : SegmentRuntimeCoord lo limit) :
    segmentByteRead bytes coord.2.1 coord.2.2 = byteMarkRead bytes (segmentRuntimeMark coord) := by
  exact segmentByteRead_eq_byteMarkRead bytes coord.2.1 coord.2.2

theorem segmentRuntimeRead_of_mem_plans_distinct {lo limit : ℕ}
    (plans : List (SegmentRuntimePlan lo limit)) (bytes : SegmentByteState)
    (hAligned :
      ∀ plan ∈ plans, ∀ coord ∈ plan.2, (segmentRuntimeMark coord).1 = plan.1)
    (hDistinct :
      plansHaveDistinctByteSlots (plans.map (coordinatePlanToBytePlan segmentRuntimeMark)))
    {plan : SegmentRuntimePlan lo limit} (hPlan : plan ∈ plans)
    {coord : SegmentRuntimeCoord lo limit} (hCoord : coord ∈ plan.2) :
    segmentByteRead
        (bytePlanWriteMany bytes (plans.map (coordinatePlanToBytePlan segmentRuntimeMark)))
        coord.2.1 coord.2.2 = 1 := by
  rw [segmentRuntimeRead_eq_byteMarkRead]
  exact byteMarkRead_of_mem_coordinatePlans_distinct
    segmentRuntimeMark bytes plans hAligned hDistinct hPlan hCoord

end PrimeArithmetic.Sieve
