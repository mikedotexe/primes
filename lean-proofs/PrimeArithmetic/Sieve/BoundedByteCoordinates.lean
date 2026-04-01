import Mathlib
import PrimeArithmetic.Sieve.BoundedBytePlans

namespace PrimeArithmetic.Sieve

/-!
Tiny coordinate-to-byte-plan bridges for sieve-style bitsets.

This module is intentionally small. It does not introduce new arithmetic; it
only packages several recurring proof patterns:

- a local read/write shell may already be known to agree with one fixed
  `ByteMark`, so written-readback can be discharged once and reused
- one explicit coordinate may be packaged as a singleton grouped plan, together
  with its immediate slot-alignment witness
- a two-coordinate family on distinct byte slots may be discharged without
  rebuilding the grouped-plan proof shell by hand
- a finite coordinate family may be bucketed by byte slot in a canonical way,
  with automatic alignment, distinct-slot, and membership-recovery facts
- one grouped runtime or offline byte bucket may already be available as a
  single coordinate plan, so same-byte readback can be discharged directly
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

/-- Apply a finite family of grouped coordinate plans via their byte-plan image. -/
def coordinatePlanWriteMany {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (bytes : BoundedByteState byteCount)
    (plans : List (CoordinatePlan Coord byteCount)) : BoundedByteState byteCount :=
  bytePlanWriteMany bytes (plans.map (coordinatePlanToBytePlan mark))

/-- Backward-compatible shorter name for grouped coordinate writes. -/
def coordinateWriteMany {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (bytes : BoundedByteState byteCount)
    (plans : List (CoordinatePlan Coord byteCount)) : BoundedByteState byteCount :=
  coordinatePlanWriteMany mark bytes plans

/-- Distinct targeted byte slots for a grouped coordinate family. -/
def coordinatePlansHaveDistinctByteSlots {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount)
    (plans : List (CoordinatePlan Coord byteCount)) : Prop :=
  plansHaveDistinctByteSlots (plans.map (coordinatePlanToBytePlan mark))

/-- Package one coordinate as a one-bucket grouped plan. -/
def singletonCoordinatePlan {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (coord : Coord) : CoordinatePlan Coord byteCount :=
  ((mark coord).1, [coord])

theorem mem_singletonCoordinatePlan {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (coord : Coord) :
    coord ∈ (singletonCoordinatePlan mark coord).2 := by
  simp [singletonCoordinatePlan]

theorem singletonCoordinatePlan_aligned {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (coord : Coord) :
    ∀ coord' ∈ (singletonCoordinatePlan mark coord).2,
      (mark coord').1 = (singletonCoordinatePlan mark coord).1 := by
  intro coord' hCoord'
  simp [singletonCoordinatePlan] at hCoord' ⊢
  subst hCoord'
  rfl

theorem singletonCoordinatePlans_aligned {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (coord : Coord) :
    ∀ plan ∈ [singletonCoordinatePlan mark coord],
      ∀ coord' ∈ plan.2, (mark coord').1 = plan.1 := by
  intro plan hPlan coord' hCoord'
  have hPlanEq : plan = singletonCoordinatePlan mark coord := by
    simpa using hPlan
  subst hPlanEq
  exact singletonCoordinatePlan_aligned mark coord coord' hCoord'

/-- The two-plan family attached to a pair of coordinates. -/
def coordinatePlanPair {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (coord₁ coord₂ : Coord) :
    List (CoordinatePlan Coord byteCount) :=
  [singletonCoordinatePlan mark coord₁, singletonCoordinatePlan mark coord₂]

theorem coordinatePlanPair_aligned {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (coord₁ coord₂ : Coord) :
    ∀ plan ∈ coordinatePlanPair mark coord₁ coord₂,
      ∀ coord ∈ plan.2, (mark coord).1 = plan.1 := by
  intro plan hPlan coord hCoord
  have hPlanCases :
      plan = singletonCoordinatePlan mark coord₁ ∨
        plan = singletonCoordinatePlan mark coord₂ := by
    simpa [coordinatePlanPair] using hPlan
  cases hPlanCases with
  | inl hPlanEq =>
      subst hPlanEq
      exact singletonCoordinatePlan_aligned mark coord₁ coord hCoord
  | inr hPlanEq =>
      subst hPlanEq
      exact singletonCoordinatePlan_aligned mark coord₂ coord hCoord

theorem coordinatePlanPair_distinct_of_byte_ne {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) {coord₁ coord₂ : Coord}
    (hByte : (mark coord₁).1 ≠ (mark coord₂).1) :
    coordinatePlansHaveDistinctByteSlots mark (coordinatePlanPair mark coord₁ coord₂) := by
  simpa [coordinatePlansHaveDistinctByteSlots] using
    (show plansHaveDistinctByteSlots
        ((coordinatePlanPair mark coord₁ coord₂).map (coordinatePlanToBytePlan mark)) from by
      simpa [coordinatePlanPair, singletonCoordinatePlan, plansHaveDistinctByteSlots,
        coordinatePlanToBytePlan] using hByte)

/-- The bucket for one fixed byte slot inside a finite coordinate family. -/
def coordinatePlanBucket {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (coords : List Coord)
    (slot : Fin byteCount) : CoordinatePlan Coord byteCount :=
  (slot, coords.filter fun coord => decide ((mark coord).1 = slot))

/-- Canonical grouped plans obtained by bucketing coordinates by byte slot. -/
def coordinatePlansByByte {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (coords : List Coord) :
    List (CoordinatePlan Coord byteCount) :=
  (List.finRange byteCount).map (coordinatePlanBucket mark coords)

theorem coordinatePlanBucket_aligned {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (coords : List Coord)
    (slot : Fin byteCount) :
    ∀ coord ∈ (coordinatePlanBucket mark coords slot).2, (mark coord).1 = slot := by
  intro coord hCoord
  simp [coordinatePlanBucket] at hCoord
  exact hCoord.2

theorem coordinatePlanBucket_mem_coordinatePlansByByte {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (coords : List Coord)
    (slot : Fin byteCount) :
    coordinatePlanBucket mark coords slot ∈ coordinatePlansByByte mark coords := by
  unfold coordinatePlansByByte
  exact List.mem_map.mpr ⟨slot, by simp [List.mem_finRange], rfl⟩

theorem mem_coordinatePlanBucket_of_mem {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (coords : List Coord)
    {coord : Coord} (hCoord : coord ∈ coords) :
    coord ∈ (coordinatePlanBucket mark coords (mark coord).1).2 := by
  simp [coordinatePlanBucket, hCoord]

theorem coordinatePlansByByte_aligned {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (coords : List Coord) :
    ∀ plan ∈ coordinatePlansByByte mark coords,
      ∀ coord ∈ plan.2, (mark coord).1 = plan.1 := by
  intro plan hPlan coord hCoord
  unfold coordinatePlansByByte at hPlan
  rcases List.mem_map.mp hPlan with ⟨slot, -, rfl⟩
  exact coordinatePlanBucket_aligned mark coords slot coord hCoord

theorem coordinatePlansByByte_distinct {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (coords : List Coord) :
    coordinatePlansHaveDistinctByteSlots mark (coordinatePlansByByte mark coords) := by
  simpa [coordinatePlansHaveDistinctByteSlots] using
    (show plansHaveDistinctByteSlots
        ((coordinatePlansByByte mark coords).map (coordinatePlanToBytePlan mark)) from by
      unfold plansHaveDistinctByteSlots coordinatePlansByByte
      rw [List.pairwise_map, List.pairwise_map]
      refine (List.nodup_finRange byteCount).pairwise_of_forall_ne ?_
      intro a ha b hb hne
      simpa [coordinatePlanBucket, coordinatePlanToBytePlan] using hne)

theorem exists_coordinatePlan_mem_of_mem_coordinatePlansByByte
    {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (coords : List Coord)
    {coord : Coord} (hCoord : coord ∈ coords) :
    ∃ plan ∈ coordinatePlansByByte mark coords, coord ∈ plan.2 := by
  refine ⟨coordinatePlanBucket mark coords (mark coord).1, ?_, ?_⟩
  · exact coordinatePlanBucket_mem_coordinatePlansByByte mark coords (mark coord).1
  · exact mem_coordinatePlanBucket_of_mem mark coords hCoord

theorem byteMarkRead_of_mem_coordinatePlan {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (bytes : BoundedByteState byteCount)
    (plan : CoordinatePlan Coord byteCount)
    (hAligned : ∀ coord ∈ plan.2, (mark coord).1 = plan.1)
    {coord : Coord} (hCoord : coord ∈ plan.2) :
    byteMarkRead (bytePlanWrite bytes (coordinatePlanToBytePlan mark plan)) (mark coord) = 1 := by
  have hByte : (mark coord).1 = plan.1 := hAligned coord hCoord
  have hBitMem : (mark coord).2 ∈ (coordinatePlanToBytePlan mark plan).2 := by
    exact List.mem_map.mpr ⟨coord, hCoord, rfl⟩
  cases hMark : mark coord with
  | mk byteSlot bit =>
      simp only [hMark] at hByte hBitMem
      cases hByte
      simpa [coordinatePlanToBytePlan, bytePlanWrite] using
        (byteMarkRead_written_by_bits_of_mem bytes plan.1
          (plan.2.map fun c => (mark c).2) bit hBitMem)

theorem byteMarkRead_of_mem_coordinatePlans_distinct {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (bytes : BoundedByteState byteCount)
    (plans : List (CoordinatePlan Coord byteCount))
    (hAligned : ∀ plan ∈ plans, ∀ coord ∈ plan.2, (mark coord).1 = plan.1)
    (hDistinct : coordinatePlansHaveDistinctByteSlots mark plans)
    {plan : CoordinatePlan Coord byteCount} (hPlan : plan ∈ plans)
    {coord : Coord} (hCoord : coord ∈ plan.2) :
    byteMarkRead (coordinatePlanWriteMany mark bytes plans) (mark coord) = 1 := by
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
      simpa [coordinatePlanWriteMany, coordinatePlansHaveDistinctByteSlots,
        coordinatePlanToBytePlan] using
        (byteMarkRead_of_mem_planWriteMany_distinct
          (plans := plans.map (coordinatePlanToBytePlan mark))
          (bytes := bytes) (slot := plan.1)
          (bits := plan.2.map fun c => (mark c).2) (target := bit)
          hDistinct hPlanMem hBitMem)

theorem byteMarkRead_of_mem_coordinatePlansByByte {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount) (bytes : BoundedByteState byteCount)
    (coords : List Coord) {coord : Coord} (hCoord : coord ∈ coords) :
    byteMarkRead (coordinatePlanWriteMany mark bytes (coordinatePlansByByte mark coords)) (mark coord) = 1 := by
  rcases exists_coordinatePlan_mem_of_mem_coordinatePlansByByte mark coords hCoord with
    ⟨plan, hPlan, hCoordPlan⟩
  exact byteMarkRead_of_mem_coordinatePlans_distinct
    mark bytes (coordinatePlansByByte mark coords)
    (coordinatePlansByByte_aligned mark coords)
    (coordinatePlansByByte_distinct mark coords)
    hPlan hCoordPlan

theorem read_of_mem_coordinatePlans_distinct_of_eq {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount)
    (read : BoundedByteState byteCount → Coord → ℕ)
    (bytes : BoundedByteState byteCount)
    (plans : List (CoordinatePlan Coord byteCount))
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark coord))
    (hAligned : ∀ plan ∈ plans, ∀ coord ∈ plan.2, (mark coord).1 = plan.1)
    (hDistinct : coordinatePlansHaveDistinctByteSlots mark plans)
    {plan : CoordinatePlan Coord byteCount} (hPlan : plan ∈ plans)
    {coord : Coord} (hCoord : coord ∈ plan.2) :
    read (coordinatePlanWriteMany mark bytes plans) coord = 1 := by
  rw [hRead]
  exact byteMarkRead_of_mem_coordinatePlans_distinct
    mark bytes plans hAligned hDistinct hPlan hCoord

theorem coordRead_of_mem_coordinatePlans_distinct {Coord : Type} {byteCount : ℕ}
    (read : BoundedByteState byteCount → Coord → ℕ)
    (mark : Coord → ByteMark byteCount)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark coord))
    (bytes : BoundedByteState byteCount)
    (plans : List (CoordinatePlan Coord byteCount))
    (hAligned : ∀ plan ∈ plans, ∀ coord ∈ plan.2, (mark coord).1 = plan.1)
    (hDistinct : coordinatePlansHaveDistinctByteSlots mark plans)
    {plan : CoordinatePlan Coord byteCount} (hPlan : plan ∈ plans)
    {coord : Coord} (hCoord : coord ∈ plan.2) :
    read (coordinateWriteMany mark bytes plans) coord = 1 := by
  simpa [coordinateWriteMany] using
    (read_of_mem_coordinatePlans_distinct_of_eq
      mark read bytes plans hRead hAligned hDistinct hPlan hCoord)

theorem coordRead_singleton {Coord : Type} {byteCount : ℕ}
    (read : BoundedByteState byteCount → Coord → ℕ)
    (mark : Coord → ByteMark byteCount)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark coord))
    (bytes : BoundedByteState byteCount) (coord : Coord) :
    read (coordinateWriteMany mark bytes [singletonCoordinatePlan mark coord]) coord = 1 := by
  simpa [coordinateWriteMany] using
    (coordRead_of_mem_coordinatePlans_distinct
      (read := read) (mark := mark) (hRead := hRead) (bytes := bytes)
      (plans := [singletonCoordinatePlan mark coord])
      (hAligned := singletonCoordinatePlans_aligned mark coord)
      (hDistinct := by
        simp [coordinatePlansHaveDistinctByteSlots, plansHaveDistinctByteSlots,
          singletonCoordinatePlan, coordinatePlanToBytePlan])
      (plan := singletonCoordinatePlan mark coord) (coord := coord)
      (by simp [singletonCoordinatePlan])
      (by simp [singletonCoordinatePlan]))

theorem coordRead_first_of_pair {Coord : Type} {byteCount : ℕ}
    (read : BoundedByteState byteCount → Coord → ℕ)
    (mark : Coord → ByteMark byteCount)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark coord))
    (bytes : BoundedByteState byteCount) (coord₁ coord₂ : Coord)
    (hByte : (mark coord₁).1 ≠ (mark coord₂).1) :
    read (coordinateWriteMany mark bytes (coordinatePlanPair mark coord₁ coord₂)) coord₁ = 1 := by
  simpa [coordinateWriteMany] using
    (coordRead_of_mem_coordinatePlans_distinct
      (read := read) (mark := mark) (hRead := hRead) (bytes := bytes)
      (plans := coordinatePlanPair mark coord₁ coord₂)
      (hAligned := coordinatePlanPair_aligned mark coord₁ coord₂)
      (hDistinct := coordinatePlanPair_distinct_of_byte_ne mark hByte)
      (plan := singletonCoordinatePlan mark coord₁) (coord := coord₁)
      (by simp [coordinatePlanPair, singletonCoordinatePlan])
      (by simp [singletonCoordinatePlan]))

theorem coordRead_second_of_pair {Coord : Type} {byteCount : ℕ}
    (read : BoundedByteState byteCount → Coord → ℕ)
    (mark : Coord → ByteMark byteCount)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark coord))
    (bytes : BoundedByteState byteCount) (coord₁ coord₂ : Coord)
    (hByte : (mark coord₁).1 ≠ (mark coord₂).1) :
    read (coordinateWriteMany mark bytes (coordinatePlanPair mark coord₁ coord₂)) coord₂ = 1 := by
  simpa [coordinateWriteMany] using
    (coordRead_of_mem_coordinatePlans_distinct
      (read := read) (mark := mark) (hRead := hRead) (bytes := bytes)
      (plans := coordinatePlanPair mark coord₁ coord₂)
      (hAligned := coordinatePlanPair_aligned mark coord₁ coord₂)
      (hDistinct := coordinatePlanPair_distinct_of_byte_ne mark hByte)
      (plan := singletonCoordinatePlan mark coord₂) (coord := coord₂)
      (by simp [coordinatePlanPair, singletonCoordinatePlan])
      (by simp [singletonCoordinatePlan]))

end PrimeArithmetic.Sieve
