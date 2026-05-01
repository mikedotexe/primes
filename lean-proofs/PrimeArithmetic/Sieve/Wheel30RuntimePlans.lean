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

/-- Reinterpret a runtime wheel30 coordinate at a different phantom base. -/
def Wheel30RuntimeCoord.rebase {base₁ base₂ : ℕ}
    (coord : Wheel30RuntimeCoord base₁) : Wheel30RuntimeCoord base₂ :=
  ⟨coord.cycle, coord.slot, coord.hCycle⟩

@[simp] theorem Wheel30RuntimeCoord.rebase_id {base : ℕ}
    (coord : Wheel30RuntimeCoord base) :
    coord.rebase (base₂ := base) = coord := by
  cases coord
  rfl

@[simp] theorem Wheel30RuntimeCoord.rebase_rebase {base₁ base₂ base₃ : ℕ}
    (coord : Wheel30RuntimeCoord base₁) :
    (coord.rebase (base₂ := base₂)).rebase (base₂ := base₃) =
      coord.rebase (base₂ := base₃) := by
  cases coord
  rfl

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

/-- A raw runtime `(cycle, slot)` input whose cycle is known to stay in range. -/
abbrev Wheel30BoundedCycleSlot := { cycleSlot : ℕ × Fin 8 // cycleSlot.1 < wheel30SegmentBytes }

/-- Candidate attached to one bounded raw wheel30 input. -/
def wheel30BoundedCycleSlotCandidate (base : ℕ)
    (cycleSlot : Wheel30BoundedCycleSlot) : ℕ :=
  wheel30Candidate base cycleSlot.1.1 cycleSlot.1.2

/-- Linear wheel index attached to one bounded raw wheel30 input. -/
def wheel30BoundedCycleSlotLinearIndex
    (cycleSlot : Wheel30BoundedCycleSlot) : ℕ :=
  wheel30LinearIndex cycleSlot.1.1 cycleSlot.1.2

/-- Convert a bounded raw wheel30 input into the runtime coordinate structure. -/
def wheel30RuntimeCoordOfBoundedCycleSlot {base : ℕ}
    (cycleSlot : Wheel30BoundedCycleSlot) : Wheel30RuntimeCoord base :=
  ⟨cycleSlot.1.1, cycleSlot.1.2, cycleSlot.2⟩

/-- Forget the phantom base and recover the bounded raw wheel30 input. -/
def wheel30BoundedCycleSlotOfRuntimeCoord {base : ℕ}
    (coord : Wheel30RuntimeCoord base) : Wheel30BoundedCycleSlot :=
  ⟨(coord.cycle, coord.slot), coord.hCycle⟩

@[simp] theorem wheel30BoundedCycleSlotOfRuntimeCoord_coordOfBoundedCycleSlot {base : ℕ}
    (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotOfRuntimeCoord
        (wheel30RuntimeCoordOfBoundedCycleSlot (base := base) cycleSlot) =
      cycleSlot := by
  cases cycleSlot
  rfl

@[simp] theorem wheel30RuntimeCoordOfBoundedCycleSlot_boundedCycleSlotOfRuntimeCoord {base : ℕ}
    (coord : Wheel30RuntimeCoord base) :
    wheel30RuntimeCoordOfBoundedCycleSlot (base := base)
        (wheel30BoundedCycleSlotOfRuntimeCoord coord) =
      coord := by
  cases coord
  rfl

theorem wheel30RuntimeCoordOfBoundedCycleSlot_leftInverse {base : ℕ} :
    Function.LeftInverse
      wheel30BoundedCycleSlotOfRuntimeCoord
      (wheel30RuntimeCoordOfBoundedCycleSlot (base := base)) := by
  intro cycleSlot
  exact wheel30BoundedCycleSlotOfRuntimeCoord_coordOfBoundedCycleSlot
    (base := base) cycleSlot

theorem wheel30RuntimeCoordOfBoundedCycleSlot_injective {base : ℕ} :
    Function.Injective (wheel30RuntimeCoordOfBoundedCycleSlot (base := base)) := by
  exact (wheel30RuntimeCoordOfBoundedCycleSlot_leftInverse (base := base)).injective

/-- Byte-mark view of a bounded raw wheel30 input. -/
def wheel30BoundedCycleSlotMark
    (cycleSlot : Wheel30BoundedCycleSlot) : ByteMark wheel30SegmentBytes :=
  wheel30CandidateMark cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2

@[simp] theorem wheel30RuntimeMark_coordOfBoundedCycleSlot {base : ℕ}
    (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30RuntimeMark (wheel30RuntimeCoordOfBoundedCycleSlot (base := base) cycleSlot) =
      wheel30BoundedCycleSlotMark cycleSlot := by
  rfl

/-- Readback at the runtime wheel30 candidate named by a bounded raw input. -/
def wheel30BoundedCycleSlotRead (bytes : Wheel30ByteState) (base : ℕ)
    (cycleSlot : Wheel30BoundedCycleSlot) : ℕ :=
  wheel30CandidateRead bytes base cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2

@[simp] theorem wheel30CandidateRead_eq_boundedCycleSlotRead
    (bytes : Wheel30ByteState) (base : ℕ)
    (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30CandidateRead bytes base cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2 =
      wheel30BoundedCycleSlotRead bytes base cycleSlot := by
  cases cycleSlot
  rfl

/-- Canonical per-byte bucketing for a finite bounded `(cycle, slot)` family. -/
def wheel30BoundedCycleSlotWriteByByte (bytes : Wheel30ByteState)
    (cycleSlots : List Wheel30BoundedCycleSlot) : Wheel30ByteState :=
  coordinatePlanWriteMany wheel30BoundedCycleSlotMark bytes
    (coordinatePlansByByte wheel30BoundedCycleSlotMark cycleSlots)

/-- Runtime-coordinate image of a bounded `(cycle, slot)` family. -/
def wheel30RuntimeCoordsOfBoundedCycleSlots {base : ℕ}
    (cycleSlots : List Wheel30BoundedCycleSlot) : List (Wheel30RuntimeCoord base) :=
  cycleSlots.map wheel30RuntimeCoordOfBoundedCycleSlot

/-- Grouped plans on bounded raw wheel30 inputs. -/
abbrev Wheel30BoundedCycleSlotPlan :=
  CoordinatePlan Wheel30BoundedCycleSlot wheel30SegmentBytes

/-- Apply a finite family of grouped bounded `(cycle, slot)` plans. -/
def wheel30BoundedCycleSlotWriteMany (bytes : Wheel30ByteState)
    (plans : List Wheel30BoundedCycleSlotPlan) : Wheel30ByteState :=
  coordinatePlanWriteMany wheel30BoundedCycleSlotMark bytes plans

/-- Forget bounded raw inputs inside one grouped plan and keep runtime coordinates. -/
def Wheel30BoundedCycleSlotPlan.toRuntimePlan {base : ℕ}
    (plan : Wheel30BoundedCycleSlotPlan) : Wheel30RuntimePlan base :=
  CoordinatePlan.map (wheel30RuntimeCoordOfBoundedCycleSlot (base := base)) plan

@[simp] theorem Wheel30BoundedCycleSlotPlan.toRuntimePlan_fst {base : ℕ}
    (plan : Wheel30BoundedCycleSlotPlan) :
    plan.toRuntimePlan (base := base).1 = plan.1 := by
  simp [Wheel30BoundedCycleSlotPlan.toRuntimePlan]

@[simp] theorem Wheel30BoundedCycleSlotPlan.toRuntimePlan_snd {base : ℕ}
    (plan : Wheel30BoundedCycleSlotPlan) :
    plan.toRuntimePlan (base := base).2 =
      plan.2.map (wheel30RuntimeCoordOfBoundedCycleSlot (base := base)) := by
  simp [Wheel30BoundedCycleSlotPlan.toRuntimePlan]

theorem wheel30BoundedCycleSlot_mem_of_mem_toRuntimePlan {base : ℕ}
    (plan : Wheel30BoundedCycleSlotPlan)
    {coord : Wheel30RuntimeCoord base}
    (hCoord : coord ∈ (plan.toRuntimePlan (base := base)).2) :
    wheel30BoundedCycleSlotOfRuntimeCoord coord ∈ plan.2 := by
  exact CoordinatePlan.mem_of_mem_map_snd
    (f := wheel30RuntimeCoordOfBoundedCycleSlot (base := base))
    (g := wheel30BoundedCycleSlotOfRuntimeCoord)
    (hLeft := wheel30RuntimeCoordOfBoundedCycleSlot_leftInverse (base := base))
    (plan := plan) hCoord

@[simp] theorem wheel30RuntimeCoordOfBoundedCycleSlot_mem_toRuntimePlan_iff {base : ℕ}
    (plan : Wheel30BoundedCycleSlotPlan)
    {cycleSlot : Wheel30BoundedCycleSlot} :
    wheel30RuntimeCoordOfBoundedCycleSlot (base := base) cycleSlot ∈
        (plan.toRuntimePlan (base := base)).2 ↔
      cycleSlot ∈ plan.2 := by
  simpa [Wheel30BoundedCycleSlotPlan.toRuntimePlan] using
    (CoordinatePlan.mem_map_snd_iff
      (f := wheel30RuntimeCoordOfBoundedCycleSlot (base := base))
      (g := wheel30BoundedCycleSlotOfRuntimeCoord)
      (hLeft := wheel30RuntimeCoordOfBoundedCycleSlot_leftInverse (base := base))
      (plan := plan)
      (coord := cycleSlot))

theorem Wheel30BoundedCycleSlotPlan.toRuntimePlan_mem_map {base : ℕ}
    (plans : List Wheel30BoundedCycleSlotPlan)
    {plan : Wheel30BoundedCycleSlotPlan} (hPlan : plan ∈ plans) :
    plan.toRuntimePlan (base := base) ∈
      plans.map (fun plan => plan.toRuntimePlan (base := base)) := by
  exact coordinatePlan_mem_mappedPlans
    (f := wheel30RuntimeCoordOfBoundedCycleSlot (base := base))
    (plans := plans) hPlan

theorem Wheel30BoundedCycleSlotPlan.mem_of_mem_toRuntimePlan_map {base : ℕ}
    (plans : List Wheel30BoundedCycleSlotPlan)
    {plan : Wheel30RuntimePlan base}
    (hPlan : plan ∈ plans.map (fun plan => plan.toRuntimePlan (base := base))) :
    CoordinatePlan.map wheel30BoundedCycleSlotOfRuntimeCoord plan ∈ plans := by
  exact coordinatePlan_mem_of_mem_mappedPlans
    (f := wheel30RuntimeCoordOfBoundedCycleSlot (base := base))
    (g := wheel30BoundedCycleSlotOfRuntimeCoord)
    (hLeft := wheel30RuntimeCoordOfBoundedCycleSlot_leftInverse (base := base))
    (plans := plans) hPlan

@[simp] theorem Wheel30BoundedCycleSlotPlan.toRuntimePlan_mem_map_iff {base : ℕ}
    (plans : List Wheel30BoundedCycleSlotPlan)
    {plan : Wheel30BoundedCycleSlotPlan} :
    plan.toRuntimePlan (base := base) ∈
        plans.map (fun plan => plan.toRuntimePlan (base := base)) ↔
      plan ∈ plans := by
  simpa [Wheel30BoundedCycleSlotPlan.toRuntimePlan] using
    (coordinatePlan_mem_mappedPlans_iff
      (f := wheel30RuntimeCoordOfBoundedCycleSlot (base := base))
      (g := wheel30BoundedCycleSlotOfRuntimeCoord)
      (hLeft := wheel30RuntimeCoordOfBoundedCycleSlot_leftInverse (base := base))
      (plans := plans)
      (plan := plan))

theorem wheel30BoundedCycleSlotWriteMany_eq_mappedRuntimePlansWriteMany
    (base : ℕ) (bytes : Wheel30ByteState)
    (plans : List Wheel30BoundedCycleSlotPlan) :
    wheel30BoundedCycleSlotWriteMany bytes plans =
      wheel30RuntimeWriteMany (base := base) bytes
        (plans.map fun plan => plan.toRuntimePlan (base := base)) := by
  simpa [wheel30BoundedCycleSlotWriteMany, wheel30RuntimeWriteMany,
    Wheel30BoundedCycleSlotPlan.toRuntimePlan, coordinateWriteMany] using
    (coordinatePlanWriteMany_eq_mappedPlans_of_mark_eq
      (mark₁ := wheel30BoundedCycleSlotMark)
      (mark₂ := wheel30RuntimeMark (base := base))
      (f := wheel30RuntimeCoordOfBoundedCycleSlot (base := base))
      (hMark := fun cycleSlot => by
        simp [wheel30RuntimeCoordOfBoundedCycleSlot, wheel30RuntimeMark,
          wheel30BoundedCycleSlotMark])
      (bytes := bytes) (plans := plans))

theorem wheel30BoundedCycleSlotPlans_toRuntimePlans_aligned_iff {base : ℕ}
    (plans : List Wheel30BoundedCycleSlotPlan) :
    (∀ plan ∈ plans.map (fun plan => plan.toRuntimePlan (base := base)),
        ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1) ↔
      (∀ plan ∈ plans, ∀ cycleSlot ∈ plan.2,
        (wheel30BoundedCycleSlotMark cycleSlot).1 = plan.1) := by
  simpa [Wheel30BoundedCycleSlotPlan.toRuntimePlan] using
    (coordinatePlans_mapped_aligned_iff_of_mark_eq
      (mark₁ := wheel30BoundedCycleSlotMark)
      (mark₂ := wheel30RuntimeMark (base := base))
      (f := wheel30RuntimeCoordOfBoundedCycleSlot (base := base))
      (hMark := fun cycleSlot => by
        simp [wheel30RuntimeCoordOfBoundedCycleSlot, wheel30RuntimeMark,
          wheel30BoundedCycleSlotMark])
      (plans := plans))

theorem wheel30BoundedCycleSlotPlans_toRuntimePlans_aligned {base : ℕ}
    (plans : List Wheel30BoundedCycleSlotPlan)
    (hAligned :
      ∀ plan ∈ plans, ∀ cycleSlot ∈ plan.2,
        (wheel30BoundedCycleSlotMark cycleSlot).1 = plan.1) :
    ∀ plan ∈ plans.map (fun plan => plan.toRuntimePlan (base := base)),
      ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1 := by
  exact (wheel30BoundedCycleSlotPlans_toRuntimePlans_aligned_iff
    (base := base) plans).2 hAligned

theorem wheel30BoundedCycleSlotPlans_aligned_of_toRuntimePlans {base : ℕ}
    (plans : List Wheel30BoundedCycleSlotPlan)
    (hAligned :
      ∀ plan ∈ plans.map (fun plan => plan.toRuntimePlan (base := base)),
        ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1) :
    ∀ plan ∈ plans, ∀ cycleSlot ∈ plan.2,
      (wheel30BoundedCycleSlotMark cycleSlot).1 = plan.1 := by
  exact (wheel30BoundedCycleSlotPlans_toRuntimePlans_aligned_iff
    (base := base) plans).1 hAligned

theorem wheel30BoundedCycleSlotPlans_toRuntimePlans_distinctByteSlots_iff {base : ℕ}
    (plans : List Wheel30BoundedCycleSlotPlan) :
    coordinatePlansHaveDistinctByteSlots (wheel30RuntimeMark (base := base))
        (plans.map fun plan => plan.toRuntimePlan (base := base)) ↔
      coordinatePlansHaveDistinctByteSlots wheel30BoundedCycleSlotMark plans := by
  simpa [Wheel30BoundedCycleSlotPlan.toRuntimePlan] using
    (coordinatePlans_mapped_distinctByteSlots_iff_of_mark_eq
      (mark₁ := wheel30BoundedCycleSlotMark)
      (mark₂ := wheel30RuntimeMark (base := base))
      (f := wheel30RuntimeCoordOfBoundedCycleSlot (base := base))
      (hMark := fun cycleSlot => by
        simp [wheel30RuntimeCoordOfBoundedCycleSlot, wheel30RuntimeMark,
          wheel30BoundedCycleSlotMark])
      (plans := plans))

theorem wheel30BoundedCycleSlotPlans_toRuntimePlans_distinctByteSlots {base : ℕ}
    (plans : List Wheel30BoundedCycleSlotPlan)
    (hDistinct :
      coordinatePlansHaveDistinctByteSlots wheel30BoundedCycleSlotMark plans) :
    coordinatePlansHaveDistinctByteSlots (wheel30RuntimeMark (base := base))
      (plans.map fun plan => plan.toRuntimePlan (base := base)) := by
  exact (wheel30BoundedCycleSlotPlans_toRuntimePlans_distinctByteSlots_iff
    (base := base) plans).2 hDistinct

theorem wheel30BoundedCycleSlotPlans_distinctByteSlots_of_toRuntimePlans {base : ℕ}
    (plans : List Wheel30BoundedCycleSlotPlan)
    (hDistinct :
      coordinatePlansHaveDistinctByteSlots (wheel30RuntimeMark (base := base))
        (plans.map fun plan => plan.toRuntimePlan (base := base))) :
    coordinatePlansHaveDistinctByteSlots wheel30BoundedCycleSlotMark plans := by
  exact (wheel30BoundedCycleSlotPlans_toRuntimePlans_distinctByteSlots_iff
    (base := base) plans).1 hDistinct

theorem wheel30BoundedCycleSlotRead_of_mem_boundedPlans_distinct
    (base : ℕ) (plans : List Wheel30BoundedCycleSlotPlan) (bytes : Wheel30ByteState)
    (hAligned :
      ∀ plan ∈ plans, ∀ cycleSlot ∈ plan.2,
        (wheel30BoundedCycleSlotMark cycleSlot).1 = plan.1)
    (hDistinct : coordinatePlansHaveDistinctByteSlots wheel30BoundedCycleSlotMark plans)
    {plan : Wheel30BoundedCycleSlotPlan} (hPlan : plan ∈ plans)
    {cycleSlot : Wheel30BoundedCycleSlot} (hCoord : cycleSlot ∈ plan.2) :
    wheel30BoundedCycleSlotRead
        (wheel30BoundedCycleSlotWriteMany bytes plans)
        base cycleSlot = 1 := by
  exact read_of_mem_coordinatePlans_distinct_of_eq
    wheel30BoundedCycleSlotMark
    (fun bytes cycleSlot => wheel30BoundedCycleSlotRead bytes base cycleSlot)
    bytes plans
    (fun bytes cycleSlot => by
      unfold wheel30BoundedCycleSlotRead wheel30BoundedCycleSlotMark
      exact wheel30CandidateRead_eq_byteMarkRead
        bytes base cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2)
    hAligned hDistinct hPlan hCoord

theorem wheel30BoundedCycleSlotRead_of_mem_mappedPlans_distinct
    (base : ℕ) (plans : List Wheel30BoundedCycleSlotPlan) (bytes : Wheel30ByteState)
    (hAligned :
      ∀ plan ∈ plans, ∀ cycleSlot ∈ plan.2,
        (wheel30BoundedCycleSlotMark cycleSlot).1 = plan.1)
    (hDistinct : coordinatePlansHaveDistinctByteSlots wheel30BoundedCycleSlotMark plans)
    {plan : Wheel30BoundedCycleSlotPlan} (hPlan : plan ∈ plans)
    {cycleSlot : Wheel30BoundedCycleSlot} (hCoord : cycleSlot ∈ plan.2) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteMany (base := base) bytes
          (plans.map fun plan => plan.toRuntimePlan (base := base)))
        base cycleSlot = 1 := by
  simpa [wheel30RuntimeWriteMany, Wheel30BoundedCycleSlotPlan.toRuntimePlan,
    coordinateWriteMany] using
    (coordRead_of_mem_mappedPlans_distinct_of_mark_eq
      (read := fun bytes cycleSlot => wheel30BoundedCycleSlotRead bytes base cycleSlot)
      (mark₁ := wheel30BoundedCycleSlotMark)
      (mark₂ := wheel30RuntimeMark (base := base))
      (hRead := fun bytes cycleSlot => by
        unfold wheel30BoundedCycleSlotRead wheel30BoundedCycleSlotMark
        exact wheel30CandidateRead_eq_byteMarkRead
          bytes base cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2)
      (f := wheel30RuntimeCoordOfBoundedCycleSlot (base := base))
      (hMark := fun cycleSlot => by
        simp [wheel30RuntimeCoordOfBoundedCycleSlot, wheel30RuntimeMark,
          wheel30BoundedCycleSlotMark])
      (bytes := bytes) (plans := plans)
      hAligned hDistinct hPlan hCoord)

theorem wheel30RuntimeRead_of_mem_boundedCycleSlotPlans_distinct
    (base : ℕ) (plans : List Wheel30BoundedCycleSlotPlan) (bytes : Wheel30ByteState)
    (hAligned :
      ∀ plan ∈ plans, ∀ cycleSlot ∈ plan.2,
        (wheel30BoundedCycleSlotMark cycleSlot).1 = plan.1)
    (hDistinct : coordinatePlansHaveDistinctByteSlots wheel30BoundedCycleSlotMark plans)
    {plan : Wheel30BoundedCycleSlotPlan} (hPlan : plan ∈ plans)
    {cycleSlot : Wheel30BoundedCycleSlot} (hCoord : cycleSlot ∈ plan.2) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany (base := base) bytes
          (plans.map fun plan => plan.toRuntimePlan (base := base)))
        base cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2 = 1 := by
  simpa using
    (wheel30BoundedCycleSlotRead_of_mem_mappedPlans_distinct
      (base := base) (plans := plans) (bytes := bytes)
      hAligned hDistinct hPlan hCoord)

@[simp] theorem wheel30BoundedCycleSlotCandidate_mk
    (base cycle : ℕ) (slot : Fin 8) (hCycle : cycle < wheel30SegmentBytes) :
    wheel30BoundedCycleSlotCandidate base ⟨(cycle, slot), hCycle⟩ =
      wheel30Candidate base cycle slot := by
  rfl

@[simp] theorem wheel30BoundedCycleSlotLinearIndex_mk
    (cycle : ℕ) (slot : Fin 8) (hCycle : cycle < wheel30SegmentBytes) :
    wheel30BoundedCycleSlotLinearIndex ⟨(cycle, slot), hCycle⟩ =
      wheel30LinearIndex cycle slot := by
  rfl

theorem wheel30BoundedCycleSlot_candidate_ge_base
    (base : ℕ) (cycleSlot : Wheel30BoundedCycleSlot) :
    base ≤ wheel30BoundedCycleSlotCandidate base cycleSlot := by
  exact wheel30Candidate_ge_base base cycleSlot.1.1 cycleSlot.1.2

theorem wheel30BoundedCycleSlot_candidate_sub_base
    (base : ℕ) (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotCandidate base cycleSlot - base =
      30 * cycleSlot.1.1 + wheel30Slot cycleSlot.1.2 := by
  exact wheel30Candidate_sub_base base cycleSlot.1.1 cycleSlot.1.2

theorem wheel30BoundedCycleSlotLinearIndex_byte
    (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotLinearIndex cycleSlot / 8 = cycleSlot.1.1 := by
  exact wheel30LinearIndex_byte cycleSlot.1.1 cycleSlot.1.2

theorem wheel30BoundedCycleSlotLinearIndex_bit
    (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotLinearIndex cycleSlot % 8 = cycleSlot.1.2.1 := by
  exact wheel30LinearIndex_bit cycleSlot.1.1 cycleSlot.1.2

theorem wheel30BoundedCycleSlot_candidate_lt_base_plus_segmentSpan
    (base : ℕ) (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotCandidate base cycleSlot < base + wheel30SegmentSpan := by
  exact wheel30Candidate_lt_base_plus_segmentSpan_of_cycle_lt
    cycleSlot.1.2 cycleSlot.2

theorem wheel30BoundedCycleSlot_candidate_mod
    {base : ℕ} (cycleSlot : Wheel30BoundedCycleSlot)
    (hBase : base % 30 = 0) :
    wheel30BoundedCycleSlotCandidate base cycleSlot % 30 =
      wheel30Slot cycleSlot.1.2 := by
  exact wheel30Candidate_mod cycleSlot.1.2 hBase

theorem wheel30BoundedCycleSlot_representableCandidate
    {base : ℕ} (hBase : base % 30 = 0)
    (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30Representable (wheel30BoundedCycleSlotCandidate base cycleSlot) := by
  exact wheel30Representable_candidate cycleSlot.1.2 hBase

@[simp] theorem wheel30Index_boundedCycleSlotCandidate
    (base : ℕ) (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30Index base (wheel30BoundedCycleSlotCandidate base cycleSlot) =
      some (wheel30BoundedCycleSlotLinearIndex cycleSlot) := by
  exact wheel30Index_candidate (base := base) (cycle := cycleSlot.1.1)
    cycleSlot.1.2 cycleSlot.2

theorem exists_wheel30BoundedCycleSlot_of_index_eq_some {base n idx : ℕ}
    (hIndex : wheel30Index base n = some idx) :
    ∃ cycleSlot : Wheel30BoundedCycleSlot,
      n = wheel30BoundedCycleSlotCandidate base cycleSlot ∧
      idx = wheel30BoundedCycleSlotLinearIndex cycleSlot := by
  obtain ⟨cycle, slot, hCycle, hCand, hIdx⟩ :=
    exists_wheel30Candidate_of_index_eq_some hIndex
  exact ⟨⟨(cycle, slot), hCycle⟩, hCand, hIdx⟩

theorem wheel30Index_eq_some_iff_exists_boundedCycleSlot {base n idx : ℕ} :
    wheel30Index base n = some idx ↔
      ∃ cycleSlot : Wheel30BoundedCycleSlot,
        n = wheel30BoundedCycleSlotCandidate base cycleSlot ∧
        idx = wheel30BoundedCycleSlotLinearIndex cycleSlot := by
  constructor
  · exact exists_wheel30BoundedCycleSlot_of_index_eq_some
  · rintro ⟨cycleSlot, hCand, hIdx⟩
    rw [hCand, hIdx]
    exact wheel30Index_boundedCycleSlotCandidate base cycleSlot

theorem wheel30BoundedCycleSlot_eq_of_linearIndex_eq
    {cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot}
    (hIdx :
      wheel30BoundedCycleSlotLinearIndex cycleSlot₁ =
        wheel30BoundedCycleSlotLinearIndex cycleSlot₂) :
    cycleSlot₁ = cycleSlot₂ := by
  apply Subtype.ext
  apply Prod.ext
  · have hByte := congrArg (fun idx => idx / 8) hIdx
    simpa [wheel30BoundedCycleSlotLinearIndex, wheel30LinearIndex_byte] using hByte
  · apply Fin.ext
    have hBit := congrArg (fun idx => idx % 8) hIdx
    simpa [wheel30BoundedCycleSlotLinearIndex, wheel30LinearIndex_bit] using hBit

theorem wheel30BoundedCycleSlot_eq_of_index_eq_some
    {base n : ℕ}
    {cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot}
    (hIdx₁ :
      wheel30Index base n = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₁))
    (hIdx₂ :
      wheel30Index base n = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₂)) :
    cycleSlot₁ = cycleSlot₂ := by
  apply wheel30BoundedCycleSlot_eq_of_linearIndex_eq
  exact Option.some.inj (hIdx₁.symm.trans hIdx₂)

theorem wheel30Index_eq_some_iff_existsUnique_boundedCycleSlot {base n idx : ℕ} :
    wheel30Index base n = some idx ↔
      ∃! cycleSlot : Wheel30BoundedCycleSlot,
        n = wheel30BoundedCycleSlotCandidate base cycleSlot ∧
        idx = wheel30BoundedCycleSlotLinearIndex cycleSlot := by
  constructor
  · intro hIndex
    obtain ⟨cycleSlot, hCand, hIdx⟩ :=
      exists_wheel30BoundedCycleSlot_of_index_eq_some hIndex
    refine ⟨cycleSlot, ⟨hCand, hIdx⟩, ?_⟩
    intro other hOther
    exact wheel30BoundedCycleSlot_eq_of_index_eq_some
      (hIdx₁ := by simpa [hOther.2] using hIndex)
      (hIdx₂ := by simpa [hIdx] using hIndex)
  · rintro ⟨cycleSlot, hWitness, _⟩
    rw [hWitness.1, hWitness.2]
    exact wheel30Index_boundedCycleSlotCandidate base cycleSlot

theorem wheel30RuntimeRead_eq_byteMarkRead (bytes : Wheel30ByteState) (base : ℕ)
    (coord : Wheel30RuntimeCoord base) :
    wheel30CandidateRead bytes base coord.cycle coord.slot coord.hCycle =
      byteMarkRead bytes (wheel30RuntimeMark coord) := by
  exact wheel30CandidateRead_eq_byteMarkRead bytes base coord.cycle coord.slot coord.hCycle

@[simp] theorem wheel30RuntimeMark_rebase {base₁ base₂ : ℕ}
    (coord : Wheel30RuntimeCoord base₁) :
    wheel30RuntimeMark (coord.rebase (base₂ := base₂)) = wheel30RuntimeMark coord := by
  cases coord
  rfl

/-- Rebased runtime-coordinate families induce the same grouped byte update. -/
theorem wheel30RuntimeWriteByByte_eq_rebasedCoordsWriteByByte
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (coords : List (Wheel30RuntimeCoord base₁)) :
    wheel30RuntimeWriteByByte (base := base₁) bytes coords =
      wheel30RuntimeWriteByByte (base := base₂) bytes
        (coords.map fun coord => coord.rebase (base₂ := base₂)) := by
  simpa [wheel30RuntimeWriteByByte] using
    (coordinatePlanWriteMany_coordinatePlansByByte_eq_of_mark_eq
      (fun coord : Wheel30RuntimeCoord base₁ => wheel30RuntimeMark coord)
      (fun coord : Wheel30RuntimeCoord base₂ => wheel30RuntimeMark coord)
      (fun coord => coord.rebase (base₂ := base₂))
      (fun coord => wheel30RuntimeMark_rebase (base₂ := base₂) coord)
      bytes coords)

theorem wheel30BoundedCycleSlotRead_eq_byteMarkRead
    (bytes : Wheel30ByteState) (base : ℕ)
    (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotRead bytes base cycleSlot =
      byteMarkRead bytes (wheel30BoundedCycleSlotMark cycleSlot) := by
  exact wheel30CandidateRead_eq_byteMarkRead bytes base cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2

theorem wheel30BoundedCycleSlotRead_base_invariant
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotRead bytes base₁ cycleSlot =
      wheel30BoundedCycleSlotRead bytes base₂ cycleSlot := by
  rw [wheel30BoundedCycleSlotRead_eq_byteMarkRead
      (bytes := bytes) (base := base₁) (cycleSlot := cycleSlot)]
  rw [wheel30BoundedCycleSlotRead_eq_byteMarkRead
      (bytes := bytes) (base := base₂) (cycleSlot := cycleSlot)]

theorem wheel30RuntimeCoordOfBoundedCycleSlot_mem_coordsOfBoundedCycleSlots {base : ℕ}
    (cycleSlots : List Wheel30BoundedCycleSlot)
    {cycleSlot : Wheel30BoundedCycleSlot} (hCycleSlot : cycleSlot ∈ cycleSlots) :
    wheel30RuntimeCoordOfBoundedCycleSlot (base := base) cycleSlot ∈
      wheel30RuntimeCoordsOfBoundedCycleSlots (base := base) cycleSlots := by
  unfold wheel30RuntimeCoordsOfBoundedCycleSlots
  exact List.mem_map.mpr ⟨cycleSlot, hCycleSlot, rfl⟩

theorem wheel30BoundedCycleSlotRead_of_mem_mappedCoords_byByte (base : ℕ)
    (cycleSlots : List Wheel30BoundedCycleSlot) (bytes : Wheel30ByteState)
    {cycleSlot : Wheel30BoundedCycleSlot} (hCycleSlot : cycleSlot ∈ cycleSlots) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteByByte (base := base) bytes
          (wheel30RuntimeCoordsOfBoundedCycleSlots (base := base) cycleSlots))
        base cycleSlot = 1 := by
  simpa [wheel30RuntimeWriteByByte, wheel30RuntimeCoordsOfBoundedCycleSlots] using
    (read_of_mem_coordinatePlansByByte_mapped_of_mark_eq
      wheel30BoundedCycleSlotMark
      (wheel30RuntimeMark (base := base))
      (fun bytes cycleSlot => wheel30BoundedCycleSlotRead bytes base cycleSlot)
      (wheel30RuntimeCoordOfBoundedCycleSlot (base := base))
      (fun bytes cycleSlot => wheel30BoundedCycleSlotRead_eq_byteMarkRead bytes base cycleSlot)
      (fun cycleSlot => by
        simp [wheel30BoundedCycleSlotMark, wheel30RuntimeCoordOfBoundedCycleSlot,
          wheel30RuntimeMark])
      bytes cycleSlots hCycleSlot)

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

theorem wheel30BoundedCycleSlotRead_of_mem_byByte (base : ℕ)
    (cycleSlots : List Wheel30BoundedCycleSlot) (bytes : Wheel30ByteState)
    {cycleSlot : Wheel30BoundedCycleSlot} (hCycleSlot : cycleSlot ∈ cycleSlots) :
    wheel30BoundedCycleSlotRead (wheel30BoundedCycleSlotWriteByByte bytes cycleSlots) base cycleSlot = 1 := by
  simpa [wheel30BoundedCycleSlotWriteByByte] using
    (read_of_mem_coordinatePlansByByte_of_eq
      wheel30BoundedCycleSlotMark
      (fun bytes cycleSlot => wheel30BoundedCycleSlotRead bytes base cycleSlot)
      bytes cycleSlots
      (fun bytes cycleSlot => wheel30BoundedCycleSlotRead_eq_byteMarkRead bytes base cycleSlot)
      hCycleSlot)

theorem wheel30RuntimeRead_of_mem_boundedCycleSlots_mappedCoords_byByte (base : ℕ)
    (cycleSlots : List Wheel30BoundedCycleSlot) (bytes : Wheel30ByteState)
    {cycleSlot : Wheel30BoundedCycleSlot} (hCycleSlot : cycleSlot ∈ cycleSlots) :
    wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base) bytes
          (wheel30RuntimeCoordsOfBoundedCycleSlots (base := base) cycleSlots))
        base cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2 = 1 := by
  simpa using
    (wheel30BoundedCycleSlotRead_of_mem_mappedCoords_byByte
      (base := base) (cycleSlots := cycleSlots) (bytes := bytes) hCycleSlot)

theorem wheel30BoundedCycleSlotWriteByByte_eq_mappedCoordsWriteByByte (base : ℕ)
    (bytes : Wheel30ByteState) (cycleSlots : List Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotWriteByByte bytes cycleSlots =
      wheel30RuntimeWriteByByte (base := base) bytes
        (wheel30RuntimeCoordsOfBoundedCycleSlots (base := base) cycleSlots) := by
  simpa [wheel30BoundedCycleSlotWriteByByte, wheel30RuntimeWriteByByte,
    wheel30RuntimeCoordsOfBoundedCycleSlots] using
    (coordinatePlanWriteMany_coordinatePlansByByte_eq_of_mark_eq
      wheel30BoundedCycleSlotMark
      (wheel30RuntimeMark (base := base))
      (wheel30RuntimeCoordOfBoundedCycleSlot (base := base))
      (fun cycleSlot => by
        exact wheel30RuntimeMark_coordOfBoundedCycleSlot (base := base) cycleSlot)
      bytes cycleSlots)

/-- Transporting bounded wheel30 inputs to runtime coordinates is base-invariant. -/
theorem wheel30RuntimeWriteByByte_mappedCoords_base_invariant
    (bytes : Wheel30ByteState) (cycleSlots : List Wheel30BoundedCycleSlot)
    {base₁ base₂ : ℕ} :
    wheel30RuntimeWriteByByte (base := base₁) bytes
        (wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots) =
      wheel30RuntimeWriteByByte (base := base₂) bytes
        (wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₂) cycleSlots) := by
  rw [← wheel30BoundedCycleSlotWriteByByte_eq_mappedCoordsWriteByByte
      (base := base₁) (bytes := bytes) (cycleSlots := cycleSlots)]
  rw [← wheel30BoundedCycleSlotWriteByByte_eq_mappedCoordsWriteByByte
      (base := base₂) (bytes := bytes) (cycleSlots := cycleSlots)]

theorem wheel30BoundedCycleSlotRead_mappedCoords_base_invariant
    (bytes : Wheel30ByteState) (cycleSlots : List Wheel30BoundedCycleSlot)
    {base₁ base₂ : ℕ} (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteByByte (base := base₁) bytes
          (wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots))
        base₁ cycleSlot =
      wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteByByte (base := base₂) bytes
          (wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₂) cycleSlots))
        base₂ cycleSlot := by
  rw [wheel30RuntimeWriteByByte_mappedCoords_base_invariant
      (bytes := bytes) (cycleSlots := cycleSlots)
      (base₁ := base₁) (base₂ := base₂)]
  exact wheel30BoundedCycleSlotRead_base_invariant _ cycleSlot

/-- Bounded raw-input readback is unchanged after transport to runtime coordinates. -/
theorem wheel30BoundedCycleSlotRead_writeByByte_eq_mappedCoordsWriteByByte (base : ℕ)
    (cycleSlots : List Wheel30BoundedCycleSlot) (bytes : Wheel30ByteState)
    (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotRead (wheel30BoundedCycleSlotWriteByByte bytes cycleSlots) base cycleSlot =
      wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteByByte (base := base) bytes
          (wheel30RuntimeCoordsOfBoundedCycleSlots (base := base) cycleSlots))
        base cycleSlot := by
  rw [wheel30BoundedCycleSlotWriteByByte_eq_mappedCoordsWriteByByte]

/-- The executable wheel30 read is unchanged after bounded raw-input transport. -/
theorem wheel30RuntimeRead_boundedCycleSlotWriteByByte_eq_mappedCoordsWriteByByte (base : ℕ)
    (cycleSlots : List Wheel30BoundedCycleSlot) (bytes : Wheel30ByteState)
    (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30CandidateRead
        (wheel30BoundedCycleSlotWriteByByte bytes cycleSlots)
        base cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2 =
      wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base) bytes
          (wheel30RuntimeCoordsOfBoundedCycleSlots (base := base) cycleSlots))
        base cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2 := by
  simpa using
    (wheel30BoundedCycleSlotRead_writeByByte_eq_mappedCoordsWriteByByte
      (base := base) (cycleSlots := cycleSlots) (bytes := bytes) cycleSlot)

theorem wheel30RuntimeRead_of_mem_boundedCycleSlots_byByte (base : ℕ)
    (cycleSlots : List Wheel30BoundedCycleSlot) (bytes : Wheel30ByteState)
    {cycleSlot : Wheel30BoundedCycleSlot} (hCycleSlot : cycleSlot ∈ cycleSlots) :
    wheel30CandidateRead
        (wheel30BoundedCycleSlotWriteByByte bytes cycleSlots)
        base cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2 = 1 := by
  simpa using
    (wheel30BoundedCycleSlotRead_of_mem_byByte
      (base := base) (cycleSlots := cycleSlots) (bytes := bytes) hCycleSlot)

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

theorem wheel30BoundedCycleSlotRead_of_mem_plans_distinct
    {base₁ : ℕ} (base₂ : ℕ)
    (plans : List (Wheel30RuntimePlan base₁)) (bytes : Wheel30ByteState)
    (hAligned :
      ∀ plan ∈ plans, ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1)
    (hDistinct :
      coordinatePlansHaveDistinctByteSlots (wheel30RuntimeMark (base := base₁)) plans)
    {plan : Wheel30RuntimePlan base₁} (hPlan : plan ∈ plans)
    {cycleSlot : Wheel30BoundedCycleSlot}
    (hCoord :
      wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot ∈ plan.2) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteMany (base := base₁) bytes plans)
        base₂ cycleSlot = 1 := by
  rw [wheel30BoundedCycleSlotRead_base_invariant
      (bytes := wheel30RuntimeWriteMany (base := base₁) bytes plans)
      (base₁ := base₂) (base₂ := base₁) (cycleSlot := cycleSlot)]
  rw [← wheel30CandidateRead_eq_boundedCycleSlotRead
      (bytes := wheel30RuntimeWriteMany (base := base₁) bytes plans)
      (base := base₁) (cycleSlot := cycleSlot)]
  exact wheel30RuntimeRead_of_mem_plans_distinct (base := base₁)
    (plans := plans) (bytes := bytes) hAligned hDistinct
    (plan := plan) hPlan
    (coord := wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot) hCoord

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

theorem wheel30BoundedCycleSlotRead_singleton
    {base₁ : ℕ} (base₂ : ℕ) (bytes : Wheel30ByteState)
    (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteMany (base := base₁) bytes
          [singletonWheel30RuntimePlan
            (wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot)])
        base₂ cycleSlot = 1 := by
  rw [wheel30BoundedCycleSlotRead_base_invariant
      (bytes := wheel30RuntimeWriteMany (base := base₁) bytes
        [singletonWheel30RuntimePlan
          (wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot)])
      (base₁ := base₂) (base₂ := base₁) (cycleSlot := cycleSlot)]
  rw [← wheel30CandidateRead_eq_boundedCycleSlotRead
      (bytes := wheel30RuntimeWriteMany (base := base₁) bytes
        [singletonWheel30RuntimePlan
          (wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot)])
      (base := base₁) (cycleSlot := cycleSlot)]
  simpa [wheel30RuntimeCoordOfBoundedCycleSlot] using
    (wheel30RuntimeRead_singleton (base := base₁) (bytes := bytes)
      (coord := wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot))

theorem wheel30RuntimeRead_singleton_byByte (base : ℕ) (bytes : Wheel30ByteState)
    (coord : Wheel30RuntimeCoord base) :
    wheel30CandidateRead
        (wheel30RuntimeWriteByByte bytes [coord])
        base coord.cycle coord.slot coord.hCycle = 1 := by
  simpa [wheel30RuntimeWriteByByte] using
    (coordRead_singleton_byByte
      (read := fun bytes coord =>
        wheel30CandidateRead bytes base coord.cycle coord.slot coord.hCycle)
      (mark := wheel30RuntimeMark)
      (hRead := fun bytes coord => wheel30RuntimeRead_eq_byteMarkRead bytes base coord)
      (bytes := bytes) (coord := coord))

theorem wheel30BoundedCycleSlotRead_singleton_byByte
    {base₁ : ℕ} (base₂ : ℕ) (bytes : Wheel30ByteState)
    (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteByByte (base := base₁) bytes
          [wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot])
        base₂ cycleSlot = 1 := by
  rw [wheel30BoundedCycleSlotRead_base_invariant
      (bytes := wheel30RuntimeWriteByByte (base := base₁) bytes
        [wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot])
      (base₁ := base₂) (base₂ := base₁) (cycleSlot := cycleSlot)]
  rw [← wheel30CandidateRead_eq_boundedCycleSlotRead
      (bytes := wheel30RuntimeWriteByByte (base := base₁) bytes
        [wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot])
      (base := base₁) (cycleSlot := cycleSlot)]
  simpa [wheel30RuntimeCoordOfBoundedCycleSlot] using
    (wheel30RuntimeRead_singleton_byByte (base := base₁) (bytes := bytes)
      (coord := wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot))

/-- The grouped runtime plans attached to two bounded `(cycle, slot)` inputs. -/
def wheel30RuntimePairPlans {base : ℕ}
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    List (Wheel30RuntimePlan base) :=
  coordinatePlanPair wheel30RuntimeMark
    (⟨cycle₁, slot₁, hCycle₁⟩ : Wheel30RuntimeCoord base)
    (⟨cycle₂, slot₂, hCycle₂⟩ : Wheel30RuntimeCoord base)

/-- The grouped plans attached to two bounded raw wheel30 inputs. -/
def wheel30BoundedCycleSlotPairPlans
    (cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot) :
    List Wheel30BoundedCycleSlotPlan :=
  coordinatePlanPair wheel30BoundedCycleSlotMark cycleSlot₁ cycleSlot₂

@[simp] theorem wheel30BoundedCycleSlotPairPlans_toRuntimePlans {base : ℕ}
    (cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot) :
    (wheel30BoundedCycleSlotPairPlans cycleSlot₁ cycleSlot₂).map
        (fun plan => plan.toRuntimePlan (base := base)) =
      wheel30RuntimePairPlans (base := base)
        cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2
        cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2 := by
  simpa [wheel30BoundedCycleSlotPairPlans, wheel30RuntimePairPlans,
    Wheel30BoundedCycleSlotPlan.toRuntimePlan] using
    (coordinatePlanPair_map_eq_of_mark_eq
      (mark₁ := wheel30BoundedCycleSlotMark)
      (mark₂ := wheel30RuntimeMark (base := base))
      (f := wheel30RuntimeCoordOfBoundedCycleSlot (base := base))
      (hMark := fun cycleSlot => by
        simp [wheel30RuntimeCoordOfBoundedCycleSlot, wheel30RuntimeMark,
          wheel30BoundedCycleSlotMark])
      (coord₁ := cycleSlot₁) (coord₂ := cycleSlot₂))

theorem wheel30BoundedCycleSlotWriteMany_pairPlans_eq_runtimePairPlans
    (base : ℕ) (bytes : Wheel30ByteState)
    (cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotWriteMany bytes
        (wheel30BoundedCycleSlotPairPlans cycleSlot₁ cycleSlot₂) =
      wheel30RuntimeWriteMany (base := base) bytes
        (wheel30RuntimePairPlans (base := base)
          cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2
          cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2) := by
  rw [wheel30BoundedCycleSlotWriteMany_eq_mappedRuntimePlansWriteMany
      (base := base) (bytes := bytes)
      (plans := wheel30BoundedCycleSlotPairPlans cycleSlot₁ cycleSlot₂)]
  rw [wheel30BoundedCycleSlotPairPlans_toRuntimePlans (base := base)]

theorem wheel30BoundedCycleSlotPairPlans_aligned
    (cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot) :
    ∀ plan ∈ wheel30BoundedCycleSlotPairPlans cycleSlot₁ cycleSlot₂,
      ∀ cycleSlot ∈ plan.2, (wheel30BoundedCycleSlotMark cycleSlot).1 = plan.1 := by
  simpa [wheel30BoundedCycleSlotPairPlans] using
    (coordinatePlanPair_aligned
      (mark := wheel30BoundedCycleSlotMark)
      (coord₁ := cycleSlot₁) (coord₂ := cycleSlot₂))

theorem wheel30BoundedCycleSlotPairPlans_distinctByteSlots
    {cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot}
    (hCycle : cycleSlot₁.1.1 ≠ cycleSlot₂.1.1) :
    coordinatePlansHaveDistinctByteSlots wheel30BoundedCycleSlotMark
      (wheel30BoundedCycleSlotPairPlans cycleSlot₁ cycleSlot₂) := by
  have hMarkByte :
      (wheel30BoundedCycleSlotMark cycleSlot₁).1 ≠
        (wheel30BoundedCycleSlotMark cycleSlot₂).1 := by
    simpa [wheel30BoundedCycleSlotMark, wheel30CandidateMark] using
      (wheel30CandidateByteSlot_ne_of_cycle_ne cycleSlot₁.2 cycleSlot₂.2 hCycle)
  simpa [wheel30BoundedCycleSlotPairPlans] using
    (coordinatePlanPair_distinct_of_byte_ne
      (mark := wheel30BoundedCycleSlotMark)
      (coord₁ := cycleSlot₁) (coord₂ := cycleSlot₂) hMarkByte)

theorem wheel30RuntimeRead_first_of_cycleSlotPair (base : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany bytes
          (wheel30RuntimePairPlans (base := base)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        base cycle₁ slot₁ hCycle₁ = 1 := by
  have hMarkByte :
      (wheel30RuntimeMark (⟨cycle₁, slot₁, hCycle₁⟩ : Wheel30RuntimeCoord base)).1 ≠
        (wheel30RuntimeMark (⟨cycle₂, slot₂, hCycle₂⟩ : Wheel30RuntimeCoord base)).1 := by
    intro hEq
    apply hCycle
    simpa [wheel30RuntimeMark, wheel30CandidateMark, wheel30CandidateByteSlot] using
      congrArg Fin.val hEq
  simpa [wheel30RuntimeWriteMany, wheel30RuntimePairPlans, coordinatePlanPair] using
    (coordRead_first_of_pair
      (read := fun bytes coord =>
        wheel30CandidateRead bytes base coord.cycle coord.slot coord.hCycle)
      (mark := wheel30RuntimeMark)
      (hRead := fun bytes coord => wheel30RuntimeRead_eq_byteMarkRead bytes base coord)
      (bytes := bytes)
      (coord₁ := (⟨cycle₁, slot₁, hCycle₁⟩ : Wheel30RuntimeCoord base))
      (coord₂ := (⟨cycle₂, slot₂, hCycle₂⟩ : Wheel30RuntimeCoord base))
      hMarkByte)

theorem wheel30RuntimeRead_second_of_cycleSlotPair (base : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany bytes
          (wheel30RuntimePairPlans (base := base)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        base cycle₂ slot₂ hCycle₂ = 1 := by
  have hMarkByte :
      (wheel30RuntimeMark (⟨cycle₁, slot₁, hCycle₁⟩ : Wheel30RuntimeCoord base)).1 ≠
        (wheel30RuntimeMark (⟨cycle₂, slot₂, hCycle₂⟩ : Wheel30RuntimeCoord base)).1 := by
    intro hEq
    apply hCycle
    simpa [wheel30RuntimeMark, wheel30CandidateMark, wheel30CandidateByteSlot] using
      congrArg Fin.val hEq
  simpa [wheel30RuntimeWriteMany, wheel30RuntimePairPlans, coordinatePlanPair] using
    (coordRead_second_of_pair
      (read := fun bytes coord =>
        wheel30CandidateRead bytes base coord.cycle coord.slot coord.hCycle)
      (mark := wheel30RuntimeMark)
      (hRead := fun bytes coord => wheel30RuntimeRead_eq_byteMarkRead bytes base coord)
      (bytes := bytes)
      (coord₁ := (⟨cycle₁, slot₁, hCycle₁⟩ : Wheel30RuntimeCoord base))
      (coord₂ := (⟨cycle₂, slot₂, hCycle₂⟩ : Wheel30RuntimeCoord base))
      hMarkByte)

theorem wheel30RuntimeWriteMany_sameCyclePairPlans_eq_sequentialWrites
    (base : ℕ) (bytes : Wheel30ByteState)
    (cycle : ℕ) (slot₁ slot₂ : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes) :
    wheel30RuntimeWriteMany bytes
        (wheel30RuntimePairPlans (base := base)
          cycle slot₁ hCycle cycle slot₂ hCycle) =
      wheel30CandidateWrite
        (wheel30CandidateWrite bytes cycle slot₁ hCycle)
        cycle slot₂ hCycle := by
  rw [wheel30RuntimeWriteMany, coordinateWriteMany, coordinatePlanWriteMany,
    bytePlanWriteMany_eq_byteMarkWriteMany]
  rw [wheel30CandidateWrite_eq_byteMarkWrite,
    wheel30CandidateWrite_eq_byteMarkWrite]
  simp [wheel30RuntimePairPlans, coordinatePlanPair, singletonCoordinatePlan,
    coordinatePlanToBytePlan, bytePlanMarkFamily, bytePlanMarks, byteMarksAt,
    wheel30RuntimeMark, wheel30CandidateMark, byteMarkWriteMany]

theorem wheel30RuntimeRead_first_of_sameCyclePairPlans (base : ℕ)
    (bytes : Wheel30ByteState)
    (cycle : ℕ) (slot₁ slot₂ : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes)
    (hDistinct : slot₁ ≠ slot₂) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany bytes
          (wheel30RuntimePairPlans (base := base)
            cycle slot₁ hCycle cycle slot₂ hCycle))
        base cycle slot₁ hCycle = 1 := by
  rw [wheel30RuntimeWriteMany_sameCyclePairPlans_eq_sequentialWrites
      (base := base) (bytes := bytes) (cycle := cycle)
      (slot₁ := slot₁) (slot₂ := slot₂) (hCycle := hCycle)]
  exact wheel30CandidateRead_first_of_sequentialSameCycleWrites
    (bytes := bytes) (base := base) (cycle := cycle)
    (slot₁ := slot₁) (slot₂ := slot₂) (hCycle := hCycle) hDistinct

theorem wheel30RuntimeRead_second_of_sameCyclePairPlans (base : ℕ)
    (bytes : Wheel30ByteState)
    (cycle : ℕ) (slot₁ slot₂ : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany bytes
          (wheel30RuntimePairPlans (base := base)
            cycle slot₁ hCycle cycle slot₂ hCycle))
        base cycle slot₂ hCycle = 1 := by
  rw [wheel30RuntimeWriteMany_sameCyclePairPlans_eq_sequentialWrites
      (base := base) (bytes := bytes) (cycle := cycle)
      (slot₁ := slot₁) (slot₂ := slot₂) (hCycle := hCycle)]
  exact wheel30CandidateRead_second_of_sequentialSameCycleWrites
    (bytes := bytes) (base := base) (cycle := cycle)
    (slot₁ := slot₁) (slot₂ := slot₂) (hCycle := hCycle)

theorem wheel30RuntimeReads_of_sameCyclePairPlans (base : ℕ)
    (bytes : Wheel30ByteState)
    (cycle : ℕ) (slot₁ slot₂ : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes)
    (hDistinct : slot₁ ≠ slot₂) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany bytes
          (wheel30RuntimePairPlans (base := base)
            cycle slot₁ hCycle cycle slot₂ hCycle))
        base cycle slot₁ hCycle = 1 ∧
      wheel30CandidateRead
        (wheel30RuntimeWriteMany bytes
          (wheel30RuntimePairPlans (base := base)
            cycle slot₁ hCycle cycle slot₂ hCycle))
        base cycle slot₂ hCycle = 1 := by
  constructor
  · exact wheel30RuntimeRead_first_of_sameCyclePairPlans
      (base := base) (bytes := bytes) (cycle := cycle)
      (slot₁ := slot₁) (slot₂ := slot₂) (hCycle := hCycle) hDistinct
  · exact wheel30RuntimeRead_second_of_sameCyclePairPlans
      (base := base) (bytes := bytes) (cycle := cycle)
      (slot₁ := slot₁) (slot₂ := slot₂) (hCycle := hCycle)

theorem wheel30BoundedCycleSlotRead_first_of_sameCyclePairPlans (base : ℕ)
    (bytes : Wheel30ByteState)
    (cycle : ℕ) (slot₁ slot₂ : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes)
    (hDistinct : slot₁ ≠ slot₂) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteMany bytes
          (wheel30RuntimePairPlans (base := base)
            cycle slot₁ hCycle cycle slot₂ hCycle))
        base ⟨(cycle, slot₁), hCycle⟩ = 1 := by
  rw [← wheel30CandidateRead_eq_boundedCycleSlotRead
        (bytes := wheel30RuntimeWriteMany bytes
          (wheel30RuntimePairPlans (base := base)
            cycle slot₁ hCycle cycle slot₂ hCycle))
        (base := base) (cycleSlot := ⟨(cycle, slot₁), hCycle⟩)]
  exact wheel30RuntimeRead_first_of_sameCyclePairPlans
    (base := base) (bytes := bytes) (cycle := cycle)
    (slot₁ := slot₁) (slot₂ := slot₂) (hCycle := hCycle) hDistinct

theorem wheel30BoundedCycleSlotRead_second_of_sameCyclePairPlans (base : ℕ)
    (bytes : Wheel30ByteState)
    (cycle : ℕ) (slot₁ slot₂ : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteMany bytes
          (wheel30RuntimePairPlans (base := base)
            cycle slot₁ hCycle cycle slot₂ hCycle))
        base ⟨(cycle, slot₂), hCycle⟩ = 1 := by
  rw [← wheel30CandidateRead_eq_boundedCycleSlotRead
        (bytes := wheel30RuntimeWriteMany bytes
          (wheel30RuntimePairPlans (base := base)
            cycle slot₁ hCycle cycle slot₂ hCycle))
        (base := base) (cycleSlot := ⟨(cycle, slot₂), hCycle⟩)]
  exact wheel30RuntimeRead_second_of_sameCyclePairPlans
    (base := base) (bytes := bytes) (cycle := cycle)
    (slot₁ := slot₁) (slot₂ := slot₂) (hCycle := hCycle)

theorem wheel30BoundedCycleSlotReads_of_sameCyclePairPlans (base : ℕ)
    (bytes : Wheel30ByteState)
    (cycle : ℕ) (slot₁ slot₂ : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes)
    (hDistinct : slot₁ ≠ slot₂) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteMany bytes
          (wheel30RuntimePairPlans (base := base)
            cycle slot₁ hCycle cycle slot₂ hCycle))
        base ⟨(cycle, slot₁), hCycle⟩ = 1 ∧
      wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteMany bytes
          (wheel30RuntimePairPlans (base := base)
            cycle slot₁ hCycle cycle slot₂ hCycle))
        base ⟨(cycle, slot₂), hCycle⟩ = 1 := by
  constructor
  · exact wheel30BoundedCycleSlotRead_first_of_sameCyclePairPlans
      (base := base) (bytes := bytes) (cycle := cycle)
      (slot₁ := slot₁) (slot₂ := slot₂) (hCycle := hCycle) hDistinct
  · exact wheel30BoundedCycleSlotRead_second_of_sameCyclePairPlans
      (base := base) (bytes := bytes) (cycle := cycle)
      (slot₁ := slot₁) (slot₂ := slot₂) (hCycle := hCycle)

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

theorem wheel30RuntimeReads_of_cycleSlotPair (base : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany bytes
          (wheel30RuntimePairPlans (base := base)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        base cycle₁ slot₁ hCycle₁ = 1 ∧
      wheel30CandidateRead
        (wheel30RuntimeWriteMany bytes
          (wheel30RuntimePairPlans (base := base)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        base cycle₂ slot₂ hCycle₂ = 1 := by
  constructor
  · exact wheel30RuntimeRead_first_of_cycleSlotPair
      (base := base) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle
  · exact wheel30RuntimeRead_second_of_cycleSlotPair
      (base := base) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle

theorem wheel30RuntimeReads_of_cycleSlotPair_byByte (base : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base) bytes
          (wheel30RuntimeCycleSlotPair (base := base)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        base cycle₁ slot₁ hCycle₁ = 1 ∧
      wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base) bytes
          (wheel30RuntimeCycleSlotPair (base := base)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        base cycle₂ slot₂ hCycle₂ = 1 := by
  constructor
  · exact wheel30RuntimeRead_first_of_cycleSlotPair_byByte
      (base := base) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂
  · exact wheel30RuntimeRead_second_of_cycleSlotPair_byByte
      (base := base) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂

theorem wheel30BoundedCycleSlotReads_of_cycleSlotPair (base : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteMany bytes
          (wheel30RuntimePairPlans (base := base)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        base ⟨(cycle₁, slot₁), hCycle₁⟩ = 1 ∧
      wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteMany bytes
          (wheel30RuntimePairPlans (base := base)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        base ⟨(cycle₂, slot₂), hCycle₂⟩ = 1 := by
  constructor
  · rw [← wheel30CandidateRead_eq_boundedCycleSlotRead
        (bytes := wheel30RuntimeWriteMany bytes
          (wheel30RuntimePairPlans (base := base)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        (base := base) (cycleSlot := ⟨(cycle₁, slot₁), hCycle₁⟩)]
    exact (wheel30RuntimeReads_of_cycleSlotPair
      (base := base) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle).1
  · rw [← wheel30CandidateRead_eq_boundedCycleSlotRead
        (bytes := wheel30RuntimeWriteMany bytes
          (wheel30RuntimePairPlans (base := base)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        (base := base) (cycleSlot := ⟨(cycle₂, slot₂), hCycle₂⟩)]
    exact (wheel30RuntimeReads_of_cycleSlotPair
      (base := base) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle).2

theorem wheel30BoundedCycleSlotReads_of_cycleSlotPair_byByte (base : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteByByte (base := base) bytes
          (wheel30RuntimeCycleSlotPair (base := base)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        base ⟨(cycle₁, slot₁), hCycle₁⟩ = 1 ∧
      wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteByByte (base := base) bytes
          (wheel30RuntimeCycleSlotPair (base := base)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        base ⟨(cycle₂, slot₂), hCycle₂⟩ = 1 := by
  constructor
  · rw [← wheel30CandidateRead_eq_boundedCycleSlotRead
        (bytes := wheel30RuntimeWriteByByte (base := base) bytes
          (wheel30RuntimeCycleSlotPair (base := base)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        (base := base) (cycleSlot := ⟨(cycle₁, slot₁), hCycle₁⟩)]
    exact (wheel30RuntimeReads_of_cycleSlotPair_byByte
      (base := base) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂).1
  · rw [← wheel30CandidateRead_eq_boundedCycleSlotRead
        (bytes := wheel30RuntimeWriteByByte (base := base) bytes
          (wheel30RuntimeCycleSlotPair (base := base)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        (base := base) (cycleSlot := ⟨(cycle₂, slot₂), hCycle₂⟩)]
    exact (wheel30RuntimeReads_of_cycleSlotPair_byByte
      (base := base) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂).2

end PrimeArithmetic.Sieve
