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

As in `SegmentRuntimePlans.lean`, the main constructor-driven theorem packages
finite runtime-coordinate families either as explicit grouped plans or as
canonical per-byte buckets, then reuses `BoundedByteCoordinates.lean` to show
that every planned runtime coordinate reads back as marked after the whole
family update.
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

/-- Apply a finite family of grouped wheel30 runtime plans. -/
def wheel30RuntimeWriteMany {base : ℕ} (bytes : Wheel30ByteState)
    (plans : List (Wheel30RuntimePlan base)) : Wheel30ByteState :=
  coordinateWriteMany wheel30RuntimeMark bytes plans

/-- Grouped byte-slot write induced by a finite runtime wheel30 coordinate family. -/
def wheel30RuntimeWriteByByte {base : ℕ}
    (bytes : Wheel30ByteState) (coords : List (Wheel30RuntimeCoord base)) :
    Wheel30ByteState :=
  coordinatePlanWriteMany wheel30RuntimeMark bytes (coordinatePlansByByte wheel30RuntimeMark coords)

/-- The singleton grouped runtime plan attached to one wheel30 coordinate. -/
def singletonWheel30RuntimePlan {base : ℕ}
    (coord : Wheel30RuntimeCoord base) : Wheel30RuntimePlan base :=
  singletonCoordinatePlan wheel30RuntimeMark coord

/-- Runtime coordinates induced by a finite list of bounded `(cycle, slot)` inputs. -/
def wheel30RuntimeCoordsOfCycleSlots {base : ℕ}
    (cycleSlots : List (ℕ × Fin 8))
    (hCycles : ∀ cycleSlot ∈ cycleSlots, cycleSlot.1 < wheel30SegmentBytes) :
    List (Wheel30RuntimeCoord base) :=
  cycleSlots.attach.map fun cycleSlot =>
    ⟨cycleSlot.1.1, cycleSlot.1.2, hCycles cycleSlot.1 cycleSlot.2⟩

/-- A short wheel30 runtime family with two bounded `(cycle, slot)` inputs. -/
def wheel30RuntimeCycleSlotPair {base : ℕ}
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    List (Wheel30RuntimeCoord base) :=
  [⟨cycle₁, slot₁, hCycle₁⟩, ⟨cycle₂, slot₂, hCycle₂⟩]

theorem wheel30RuntimeRead_eq_byteMarkRead (bytes : Wheel30ByteState) (base : ℕ)
    (coord : Wheel30RuntimeCoord base) :
    wheel30CandidateRead bytes base coord.cycle coord.slot coord.hCycle =
      byteMarkRead bytes (wheel30RuntimeMark coord) := by
  exact wheel30CandidateRead_eq_byteMarkRead bytes base coord.cycle coord.slot coord.hCycle

theorem wheel30RuntimeCoord_mem_coordsOfCycleSlots {base : ℕ}
    (cycleSlots : List (ℕ × Fin 8))
    (hCycles : ∀ cycleSlot ∈ cycleSlots, cycleSlot.1 < wheel30SegmentBytes)
    {cycle : ℕ} {slot : Fin 8} (hCycleSlot : (cycle, slot) ∈ cycleSlots) :
    (⟨cycle, slot, hCycles (cycle, slot) hCycleSlot⟩ : Wheel30RuntimeCoord base) ∈
      wheel30RuntimeCoordsOfCycleSlots cycleSlots hCycles := by
  unfold wheel30RuntimeCoordsOfCycleSlots
  exact List.mem_map.mpr
    ⟨⟨(cycle, slot), hCycleSlot⟩, List.mem_attach cycleSlots ⟨(cycle, slot), hCycleSlot⟩, rfl⟩

theorem wheel30RuntimeRead_of_mem_groupedCoords (base : ℕ)
    (coords : List (Wheel30RuntimeCoord base)) (bytes : Wheel30ByteState)
    {coord : Wheel30RuntimeCoord base} (hCoord : coord ∈ coords) :
    wheel30CandidateRead
        (wheel30RuntimeWriteByByte bytes coords)
        base coord.cycle coord.slot coord.hCycle = 1 := by
  rw [wheel30RuntimeRead_eq_byteMarkRead
      (bytes := wheel30RuntimeWriteByByte bytes coords) (base := base) (coord := coord)]
  simpa [wheel30RuntimeWriteByByte] using
    (byteMarkRead_of_mem_coordinatePlansByByte
      wheel30RuntimeMark bytes coords hCoord)

theorem wheel30RuntimeRead_of_mem_plan (base : ℕ)
    (plan : Wheel30RuntimePlan base) (bytes : Wheel30ByteState)
    (hAligned : ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1)
    {coord : Wheel30RuntimeCoord base} (hCoord : coord ∈ plan.2) :
    wheel30CandidateRead
        (bytePlanWrite bytes (coordinatePlanToBytePlan wheel30RuntimeMark plan))
        base coord.cycle coord.slot coord.hCycle = 1 := by
  rw [wheel30RuntimeRead_eq_byteMarkRead]
  exact byteMarkRead_of_mem_coordinatePlan
    wheel30RuntimeMark bytes plan hAligned hCoord

theorem wheel30RuntimeRead_of_cycleSlot_mem_plan (base cycle : ℕ)
    (slot : Fin 8) (hCycle : cycle < wheel30SegmentBytes)
    (plan : Wheel30RuntimePlan base) (bytes : Wheel30ByteState)
    (hAligned : ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1)
    (hCoord : (⟨cycle, slot, hCycle⟩ : Wheel30RuntimeCoord base) ∈ plan.2) :
    wheel30CandidateRead
        (bytePlanWrite bytes (coordinatePlanToBytePlan wheel30RuntimeMark plan))
        base cycle slot hCycle = 1 := by
  simpa using
    (wheel30RuntimeRead_of_mem_plan (base := base) (plan := plan) (bytes := bytes)
      hAligned (coord := (⟨cycle, slot, hCycle⟩ : Wheel30RuntimeCoord base)) hCoord)

theorem wheel30RuntimeRead_of_mem_cycleSlots_byByte (base : ℕ)
    (cycleSlots : List (ℕ × Fin 8)) (bytes : Wheel30ByteState)
    (hCycles : ∀ cycleSlot ∈ cycleSlots, cycleSlot.1 < wheel30SegmentBytes)
    {cycle : ℕ} {slot : Fin 8} (hCycleSlot : (cycle, slot) ∈ cycleSlots) :
    wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base) bytes
          (wheel30RuntimeCoordsOfCycleSlots (base := base) cycleSlots hCycles))
        base cycle slot (hCycles (cycle, slot) hCycleSlot) = 1 := by
  simpa using
    (wheel30RuntimeRead_of_mem_groupedCoords (base := base)
      (coords := wheel30RuntimeCoordsOfCycleSlots (base := base) cycleSlots hCycles)
      (bytes := bytes)
      (coord := (⟨cycle, slot, hCycles (cycle, slot) hCycleSlot⟩ : Wheel30RuntimeCoord base))
      (wheel30RuntimeCoord_mem_coordsOfCycleSlots cycleSlots hCycles hCycleSlot))

theorem wheel30RuntimeRead_of_mem_plans_distinct (base : ℕ)
    (plans : List (Wheel30RuntimePlan base)) (bytes : Wheel30ByteState)
    (hAligned :
      ∀ plan ∈ plans, ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1)
    (hDistinct : coordinatePlansHaveDistinctByteSlots wheel30RuntimeMark plans)
    {plan : Wheel30RuntimePlan base} (hPlan : plan ∈ plans)
    {coord : Wheel30RuntimeCoord base} (hCoord : coord ∈ plan.2) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany bytes plans)
        base coord.cycle coord.slot coord.hCycle = 1 := by
  simpa [wheel30RuntimeWriteMany, coordinateWriteMany] using
    (coordRead_of_mem_coordinatePlans_distinct
      (read := fun bytes coord =>
        wheel30CandidateRead bytes base coord.cycle coord.slot coord.hCycle)
      (mark := wheel30RuntimeMark)
      (hRead := fun bytes coord => wheel30RuntimeRead_eq_byteMarkRead bytes base coord)
      (bytes := bytes) (plans := plans) hAligned hDistinct hPlan hCoord)

theorem wheel30RuntimeRead_singleton (base : ℕ) (bytes : Wheel30ByteState)
    (coord : Wheel30RuntimeCoord base) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany bytes [singletonWheel30RuntimePlan coord])
        base coord.cycle coord.slot coord.hCycle = 1 := by
  simpa [wheel30RuntimeWriteMany, singletonWheel30RuntimePlan] using
    (coordRead_singleton
      (read := fun bytes coord =>
        wheel30CandidateRead bytes base coord.cycle coord.slot coord.hCycle)
      (mark := wheel30RuntimeMark)
      (hRead := fun bytes coord => wheel30RuntimeRead_eq_byteMarkRead bytes base coord)
      (bytes := bytes) (coord := coord))

theorem wheel30RuntimeRead_first_of_cycleSlotPair_byByte (base : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base) bytes
          (wheel30RuntimeCycleSlotPair (base := base)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        base cycle₁ slot₁ hCycle₁ = 1 := by
  simpa [wheel30RuntimeCycleSlotPair] using
    (wheel30RuntimeRead_of_mem_groupedCoords (base := base)
      (coords := wheel30RuntimeCycleSlotPair (base := base)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
      (bytes := bytes)
      (coord := (⟨cycle₁, slot₁, hCycle₁⟩ : Wheel30RuntimeCoord base))
      (by simp [wheel30RuntimeCycleSlotPair]))

theorem wheel30RuntimeRead_second_of_cycleSlotPair_byByte (base : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base) bytes
          (wheel30RuntimeCycleSlotPair (base := base)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        base cycle₂ slot₂ hCycle₂ = 1 := by
  simpa [wheel30RuntimeCycleSlotPair] using
    (wheel30RuntimeRead_of_mem_groupedCoords (base := base)
      (coords := wheel30RuntimeCycleSlotPair (base := base)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
      (bytes := bytes)
      (coord := (⟨cycle₂, slot₂, hCycle₂⟩ : Wheel30RuntimeCoord base))
      (by simp [wheel30RuntimeCycleSlotPair]))

end PrimeArithmetic.Sieve
