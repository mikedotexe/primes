import Mathlib
import PrimeArithmetic.Sieve.BoundedBytePlans

namespace PrimeArithmetic.Sieve

/-!
Tiny coordinate-to-byte-plan bridges for sieve-style bitsets.

This module is intentionally small. It does not introduce new arithmetic; it
only packages two recurring proof patterns:

- a local read/write shell may already be known to agree with one fixed
  `ByteMark`, so written-readback can be discharged once and reused
- a runtime or offline coordinate type may map into `ByteMark`s, and grouped
  coordinate plans can then be pushed through `BoundedBytePlans.lean`

The goal is to keep the odd-only and wheel30 runtime shells small and let later
runtime-family modules speak in their own coordinate language without
duplicating the same byte-plan proof boilerplate.
-/

theorem fixedRead_written_of_eq {byteCount : ℕ}
    (read : BoundedByteState byteCount → ℕ)
    (write : BoundedByteState byteCount → BoundedByteState byteCount)
    (mark : ByteMark byteCount)
    (hRead : ∀ bytes, read bytes = byteMarkRead bytes mark)
    (hWrite : ∀ bytes, write bytes = byteMarkWrite bytes mark)
    (bytes : BoundedByteState byteCount) :
    read (write bytes) = 1 := by
  rw [hRead, hWrite]
  exact byteMarkRead_written bytes mark

/-- A grouped runtime or offline plan described directly in a coordinate type. -/
abbrev CoordinatePlan (Coord : Type) (byteCount : ℕ) := Fin byteCount × List Coord

/-- Forget coordinate payloads and keep only the per-byte target bits. -/
def coordinatePlanToBytePlan {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (plan : CoordinatePlan Coord byteCount) :
    BytePlan byteCount :=
  (plan.1, plan.2.map fun coord => (mark coord).2)

theorem byteMarkRead_of_mem_coordinatePlans_distinct {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (bytes : BoundedByteState byteCount)
    (plans : List (CoordinatePlan Coord byteCount))
    (hAligned : ∀ plan ∈ plans, ∀ coord ∈ plan.2, (mark coord).1 = plan.1)
    (hDistinct : plansHaveDistinctByteSlots (plans.map (coordinatePlanToBytePlan mark)))
    {plan : CoordinatePlan Coord byteCount} (hPlan : plan ∈ plans)
    {coord : Coord} (hCoord : coord ∈ plan.2) :
    byteMarkRead (bytePlanWriteMany bytes (plans.map (coordinatePlanToBytePlan mark))) (mark coord) = 1 := by
  have hByte : (mark coord).1 = plan.1 := hAligned plan hPlan coord hCoord
  have hPlanMem :
      coordinatePlanToBytePlan mark plan ∈ plans.map (coordinatePlanToBytePlan mark) := by
    exact List.mem_map.mpr ⟨plan, hPlan, rfl⟩
  have hBitMem : (mark coord).2 ∈ (coordinatePlanToBytePlan mark plan).2 := by
    exact List.mem_map.mpr ⟨coord, hCoord, rfl⟩
  cases hMark : mark coord with
  | mk byteSlot bit =>
      simp only [hMark] at hByte hBitMem
      cases hByte
      simpa [coordinatePlanToBytePlan] using
        (byteMarkRead_of_mem_planWriteMany_distinct
          (plans := plans.map (coordinatePlanToBytePlan mark))
          (bytes := bytes) (slot := plan.1)
          (bits := plan.2.map fun c => (mark c).2) (target := bit)
          hDistinct hPlanMem hBitMem)

end PrimeArithmetic.Sieve
