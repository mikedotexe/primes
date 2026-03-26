import Mathlib
import PrimeArithmetic.Sieve.BoundedByteCoordinates
import PrimeArithmetic.Sieve.Wheel30ByteArray

namespace PrimeArithmetic.Sieve

/-!
Short wheel30 runtime mark families on the grouped byte-plan layer.

The wheel30 runtime already names candidates by `(cycle, slot)`. This module
packages that runtime surface directly:

- a runtime coordinate is one wheel cycle plus one slot, with the usual bounded
  cycle proof
- the coordinate maps to the same `ByteMark` used by the single-candidate
  wheel30 byte-array shell
- grouped runtime plans are lists of per-cycle coordinate buckets

As in `SegmentRuntimePlans.lean`, the main theorem says that aligned grouped
plans with pairwise distinct byte slots read back as marked for every planned
runtime coordinate.
-/

/-- Runtime wheel30 coordinates in the executable `(cycle, slot)` language. -/
structure Wheel30RuntimeCoord (base : ℕ) where
  cycle : ℕ
  slot : Fin 8
  hCycle : cycle < wheel30SegmentBytes

/-- Byte-mark view of a runtime wheel30 coordinate. -/
def wheel30RuntimeMark {base : ℕ}
    (coord : Wheel30RuntimeCoord base) : ByteMark wheel30SegmentBytes :=
  wheel30CandidateMark coord.cycle coord.slot coord.hCycle

/-- Grouped runtime wheel30 plans bucketed by target byte slot. -/
abbrev Wheel30RuntimePlan (base : ℕ) := CoordinatePlan (Wheel30RuntimeCoord base) wheel30SegmentBytes

theorem wheel30RuntimeRead_eq_byteMarkRead (bytes : Wheel30ByteState) (base : ℕ)
    (coord : Wheel30RuntimeCoord base) :
    wheel30CandidateRead bytes base coord.cycle coord.slot coord.hCycle =
      byteMarkRead bytes (wheel30RuntimeMark coord) := by
  exact wheel30CandidateRead_eq_byteMarkRead bytes base coord.cycle coord.slot coord.hCycle

theorem wheel30RuntimeRead_of_mem_plans_distinct (base : ℕ)
    (plans : List (Wheel30RuntimePlan base)) (bytes : Wheel30ByteState)
    (hAligned :
      ∀ plan ∈ plans, ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1)
    (hDistinct :
      plansHaveDistinctByteSlots (plans.map (coordinatePlanToBytePlan wheel30RuntimeMark)))
    {plan : Wheel30RuntimePlan base} (hPlan : plan ∈ plans)
    {coord : Wheel30RuntimeCoord base} (hCoord : coord ∈ plan.2) :
    wheel30CandidateRead
        (bytePlanWriteMany bytes (plans.map (coordinatePlanToBytePlan wheel30RuntimeMark)))
        base coord.cycle coord.slot coord.hCycle = 1 := by
  rw [wheel30RuntimeRead_eq_byteMarkRead]
  exact byteMarkRead_of_mem_coordinatePlans_distinct
    wheel30RuntimeMark bytes plans hAligned hDistinct hPlan hCoord

end PrimeArithmetic.Sieve
