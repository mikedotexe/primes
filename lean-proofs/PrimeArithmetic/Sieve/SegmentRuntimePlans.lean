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

The main constructor-driven theorem here packages finite runtime-coordinate
families either as explicit grouped plans or as canonical per-byte buckets, and
then reuses `BoundedByteCoordinates.lean` to show that every planned runtime
coordinate reads back as marked after the whole family update.
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

/-- Apply a finite family of grouped odd-only runtime plans. -/
def segmentRuntimeWriteMany {lo limit : ℕ} (bytes : SegmentByteState)
    (plans : List (SegmentRuntimePlan lo limit)) : SegmentByteState :=
  coordinateWriteMany segmentRuntimeMark bytes plans

/-- Grouped byte-slot write induced by a finite runtime coordinate family. -/
def segmentRuntimeWriteByByte {lo limit : ℕ}
    (bytes : SegmentByteState) (coords : List (SegmentRuntimeCoord lo limit)) :
    SegmentByteState :=
  coordinatePlanWriteMany segmentRuntimeMark bytes (coordinatePlansByByte segmentRuntimeMark coords)

/-- The singleton grouped runtime plan attached to one in-range odd candidate. -/
def singletonSegmentRuntimePlan {lo limit : ℕ}
    (coord : SegmentRuntimeCoord lo limit) : SegmentRuntimePlan lo limit :=
  singletonCoordinatePlan segmentRuntimeMark coord

/-- The singleton grouped runtime plan attached to one executable cross-off step. -/
def singletonSegmentRuntimePlanOfStep {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit) :
    SegmentRuntimePlan lo limit :=
  singletonSegmentRuntimePlan (segmentRuntimeCoordOfStep hLo hHi)

/-- The grouped runtime plans attached to two executable cross-off steps. -/
def segmentRuntimeStepPairPlans {lo limit p segLo step₁ step₂ : ℕ}
    (hLo₁ : lo ≤ runtimeMarkedBy p segLo step₁)
    (hHi₁ : runtimeMarkedBy p segLo step₁ ≤ rawSegmentHi lo limit)
    (hLo₂ : lo ≤ runtimeMarkedBy p segLo step₂)
    (hHi₂ : runtimeMarkedBy p segLo step₂ ≤ rawSegmentHi lo limit) :
    List (SegmentRuntimePlan lo limit) :=
  coordinatePlanPair segmentRuntimeMark
    (segmentRuntimeCoordOfStep hLo₁ hHi₁)
    (segmentRuntimeCoordOfStep hLo₂ hHi₂)

/-- A short executable prefix: the runtime marks at `step` and `step + 1`. -/
def segmentRuntimeConsecutivePlans {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit) :
    List (SegmentRuntimePlan lo limit) :=
  segmentRuntimeStepPairPlans hLo hHi hLoSucc hHiSucc

/-- Runtime coordinates induced by a finite list of bounded cross-off steps. -/
def segmentRuntimeCoordsOfSteps {lo limit p segLo : ℕ}
    (steps : List ℕ)
    (hBounds :
      ∀ step ∈ steps,
        lo ≤ runtimeMarkedBy p segLo step ∧ runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit) :
    List (SegmentRuntimeCoord lo limit) :=
  steps.attach.map fun step =>
    segmentRuntimeCoordOfStep (hBounds step.1 step.2).1 (hBounds step.1 step.2).2

theorem segmentRuntimeRead_eq_byteMarkRead (bytes : SegmentByteState) {lo limit : ℕ}
    (coord : SegmentRuntimeCoord lo limit) :
    segmentByteRead bytes coord.2.1 coord.2.2 = byteMarkRead bytes (segmentRuntimeMark coord) := by
  exact segmentByteRead_eq_byteMarkRead bytes coord.2.1 coord.2.2

theorem segmentRuntimeCoordOfStep_mem_coordsOfSteps {lo limit p segLo : ℕ}
    (steps : List ℕ)
    (hBounds :
      ∀ step ∈ steps,
        lo ≤ runtimeMarkedBy p segLo step ∧ runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    {step : ℕ} (hStep : step ∈ steps) :
    segmentRuntimeCoordOfStep (hBounds step hStep).1 (hBounds step hStep).2 ∈
      segmentRuntimeCoordsOfSteps steps hBounds := by
  unfold segmentRuntimeCoordsOfSteps
  exact List.mem_map.mpr
    ⟨⟨step, hStep⟩, List.mem_attach steps ⟨step, hStep⟩, rfl⟩

theorem segmentRuntimeRead_of_mem_groupedCoords {lo limit : ℕ}
    (coords : List (SegmentRuntimeCoord lo limit)) (bytes : SegmentByteState)
    {coord : SegmentRuntimeCoord lo limit} (hCoord : coord ∈ coords) :
    segmentByteRead (segmentRuntimeWriteByByte bytes coords) coord.2.1 coord.2.2 = 1 := by
  rw [segmentRuntimeRead_eq_byteMarkRead
      (bytes := segmentRuntimeWriteByByte bytes coords) (coord := coord)]
  simpa [segmentRuntimeWriteByByte] using
    (byteMarkRead_of_mem_coordinatePlansByByte
      segmentRuntimeMark bytes coords hCoord)

theorem segmentRuntimeRead_of_mem_plan {lo limit : ℕ}
    (plan : SegmentRuntimePlan lo limit) (bytes : SegmentByteState)
    (hAligned : ∀ coord ∈ plan.2, (segmentRuntimeMark coord).1 = plan.1)
    {coord : SegmentRuntimeCoord lo limit} (hCoord : coord ∈ plan.2) :
    segmentByteRead
        (bytePlanWrite bytes (coordinatePlanToBytePlan segmentRuntimeMark plan))
        coord.2.1 coord.2.2 = 1 := by
  rw [segmentRuntimeRead_eq_byteMarkRead]
  exact byteMarkRead_of_mem_coordinatePlan
    segmentRuntimeMark bytes plan hAligned hCoord

theorem segmentRuntimeRead_of_step_mem_plan {lo limit p segLo step : ℕ}
    (plan : SegmentRuntimePlan lo limit) (bytes : SegmentByteState)
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hAligned : ∀ coord ∈ plan.2, (segmentRuntimeMark coord).1 = plan.1)
    (hCoord : segmentRuntimeCoordOfStep hLo hHi ∈ plan.2) :
    segmentByteRead
        (bytePlanWrite bytes (coordinatePlanToBytePlan segmentRuntimeMark plan))
        hLo hHi = 1 := by
  simpa [segmentRuntimeCoordOfStep] using
    (segmentRuntimeRead_of_mem_plan (plan := plan) (bytes := bytes)
      hAligned (coord := segmentRuntimeCoordOfStep hLo hHi) hCoord)

theorem segmentRuntimeRead_of_mem_steps_byByte {lo limit p segLo : ℕ}
    (steps : List ℕ) (bytes : SegmentByteState)
    (hBounds :
      ∀ step ∈ steps,
        lo ≤ runtimeMarkedBy p segLo step ∧ runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    {step : ℕ} (hStep : step ∈ steps) :
    segmentByteRead
        (segmentRuntimeWriteByByte bytes (segmentRuntimeCoordsOfSteps steps hBounds))
        (hBounds step hStep).1 (hBounds step hStep).2 = 1 := by
  simpa [segmentRuntimeCoordOfStep] using
    (segmentRuntimeRead_of_mem_groupedCoords
      (coords := segmentRuntimeCoordsOfSteps steps hBounds) (bytes := bytes)
      (coord := segmentRuntimeCoordOfStep (hBounds step hStep).1 (hBounds step hStep).2)
      (segmentRuntimeCoordOfStep_mem_coordsOfSteps steps hBounds hStep))

theorem segmentRuntimeConsecutiveBounds
    {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit) :
    ∀ s ∈ [step, step + 1],
      lo ≤ runtimeMarkedBy p segLo s ∧ runtimeMarkedBy p segLo s ≤ rawSegmentHi lo limit := by
  intro s hs
  simp at hs
  rcases hs with rfl | rfl
  · exact ⟨hLo, hHi⟩
  · exact ⟨hLoSucc, hHiSucc⟩

theorem segmentRuntimeRead_step_of_consecutivePrefix_byByte (bytes : SegmentByteState)
    {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit) :
    segmentByteRead
        (segmentRuntimeWriteByByte bytes
          (segmentRuntimeCoordsOfSteps [step, step + 1]
            (segmentRuntimeConsecutiveBounds hLo hHi hLoSucc hHiSucc)))
        hLo hHi = 1 := by
  simpa [segmentRuntimeConsecutiveBounds] using
    (segmentRuntimeRead_of_mem_steps_byByte
      (steps := [step, step + 1]) (bytes := bytes)
      (hBounds := segmentRuntimeConsecutiveBounds hLo hHi hLoSucc hHiSucc)
      (step := step) (by simp))

theorem segmentRuntimeRead_succ_of_consecutivePrefix_byByte (bytes : SegmentByteState)
    {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit) :
    segmentByteRead
        (segmentRuntimeWriteByByte bytes
          (segmentRuntimeCoordsOfSteps [step, step + 1]
            (segmentRuntimeConsecutiveBounds hLo hHi hLoSucc hHiSucc)))
        hLoSucc hHiSucc = 1 := by
  simpa [segmentRuntimeConsecutiveBounds] using
    (segmentRuntimeRead_of_mem_steps_byByte
      (steps := [step, step + 1]) (bytes := bytes)
      (hBounds := segmentRuntimeConsecutiveBounds hLo hHi hLoSucc hHiSucc)
      (step := step + 1) (by simp))

theorem segmentRuntimeRead_of_mem_plans_distinct {lo limit : ℕ}
    (plans : List (SegmentRuntimePlan lo limit)) (bytes : SegmentByteState)
    (hAligned :
      ∀ plan ∈ plans, ∀ coord ∈ plan.2, (segmentRuntimeMark coord).1 = plan.1)
    (hDistinct : coordinatePlansHaveDistinctByteSlots segmentRuntimeMark plans)
    {plan : SegmentRuntimePlan lo limit} (hPlan : plan ∈ plans)
    {coord : SegmentRuntimeCoord lo limit} (hCoord : coord ∈ plan.2) :
    segmentByteRead (segmentRuntimeWriteMany bytes plans) coord.2.1 coord.2.2 = 1 := by
  simpa [segmentRuntimeWriteMany, coordinateWriteMany] using
    (coordRead_of_mem_coordinatePlans_distinct
      (read := fun bytes coord => segmentByteRead bytes coord.2.1 coord.2.2)
      (mark := segmentRuntimeMark)
      (hRead := fun bytes coord => segmentRuntimeRead_eq_byteMarkRead bytes coord)
      (bytes := bytes) (plans := plans) hAligned hDistinct hPlan hCoord)

theorem segmentRuntimeRead_singleton (bytes : SegmentByteState) {lo limit : ℕ}
    (coord : SegmentRuntimeCoord lo limit) :
    segmentByteRead
        (segmentRuntimeWriteMany bytes [singletonSegmentRuntimePlan coord])
        coord.2.1 coord.2.2 = 1 := by
  simpa [segmentRuntimeWriteMany, singletonSegmentRuntimePlan] using
    (coordRead_singleton
      (read := fun bytes coord => segmentByteRead bytes coord.2.1 coord.2.2)
      (mark := segmentRuntimeMark)
      (hRead := fun bytes coord => segmentRuntimeRead_eq_byteMarkRead bytes coord)
      (bytes := bytes) (coord := coord))

theorem segmentRuntimeRead_singleton_of_step (bytes : SegmentByteState)
    {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit) :
    segmentByteRead
        (segmentRuntimeWriteMany bytes [singletonSegmentRuntimePlanOfStep hLo hHi])
        hLo hHi = 1 := by
  simpa [singletonSegmentRuntimePlanOfStep, segmentRuntimeCoordOfStep] using
    (segmentRuntimeRead_singleton (bytes := bytes)
      (coord := segmentRuntimeCoordOfStep hLo hHi))

theorem segmentRuntimeRead_first_of_stepPair (bytes : SegmentByteState)
    {lo limit p segLo step₁ step₂ : ℕ}
    (hLo₁ : lo ≤ runtimeMarkedBy p segLo step₁)
    (hHi₁ : runtimeMarkedBy p segLo step₁ ≤ rawSegmentHi lo limit)
    (hLo₂ : lo ≤ runtimeMarkedBy p segLo step₂)
    (hHi₂ : runtimeMarkedBy p segLo step₂ ≤ rawSegmentHi lo limit)
    (hByte :
      segmentByteIndex lo (runtimeMarkedBy p segLo step₁) ≠
        segmentByteIndex lo (runtimeMarkedBy p segLo step₂)) :
    segmentByteRead
        (segmentRuntimeWriteMany bytes (segmentRuntimeStepPairPlans hLo₁ hHi₁ hLo₂ hHi₂))
        hLo₁ hHi₁ = 1 := by
  have hMarkByte :
      (segmentRuntimeMark (segmentRuntimeCoordOfStep hLo₁ hHi₁)).1 ≠
        (segmentRuntimeMark (segmentRuntimeCoordOfStep hLo₂ hHi₂)).1 := by
    intro hEq
    apply hByte
    simpa [segmentRuntimeMark, segmentRuntimeCoordOfStep, segmentByteMark, segmentByteSlot] using
      congrArg Fin.val hEq
  simpa [segmentRuntimeWriteMany, segmentRuntimeStepPairPlans,
    singletonSegmentRuntimePlanOfStep, singletonSegmentRuntimePlan,
    segmentRuntimeCoordOfStep, coordinatePlanPair] using
    (coordRead_first_of_pair
      (read := fun bytes coord => segmentByteRead bytes coord.2.1 coord.2.2)
      (mark := segmentRuntimeMark)
      (hRead := fun bytes coord => segmentRuntimeRead_eq_byteMarkRead bytes coord)
      (bytes := bytes)
      (coord₁ := segmentRuntimeCoordOfStep hLo₁ hHi₁)
      (coord₂ := segmentRuntimeCoordOfStep hLo₂ hHi₂)
      hMarkByte)

theorem segmentRuntimeRead_second_of_stepPair (bytes : SegmentByteState)
    {lo limit p segLo step₁ step₂ : ℕ}
    (hLo₁ : lo ≤ runtimeMarkedBy p segLo step₁)
    (hHi₁ : runtimeMarkedBy p segLo step₁ ≤ rawSegmentHi lo limit)
    (hLo₂ : lo ≤ runtimeMarkedBy p segLo step₂)
    (hHi₂ : runtimeMarkedBy p segLo step₂ ≤ rawSegmentHi lo limit)
    (hByte :
      segmentByteIndex lo (runtimeMarkedBy p segLo step₁) ≠
        segmentByteIndex lo (runtimeMarkedBy p segLo step₂)) :
    segmentByteRead
        (segmentRuntimeWriteMany bytes (segmentRuntimeStepPairPlans hLo₁ hHi₁ hLo₂ hHi₂))
        hLo₂ hHi₂ = 1 := by
  have hMarkByte :
      (segmentRuntimeMark (segmentRuntimeCoordOfStep hLo₁ hHi₁)).1 ≠
        (segmentRuntimeMark (segmentRuntimeCoordOfStep hLo₂ hHi₂)).1 := by
    intro hEq
    apply hByte
    simpa [segmentRuntimeMark, segmentRuntimeCoordOfStep, segmentByteMark, segmentByteSlot] using
      congrArg Fin.val hEq
  simpa [segmentRuntimeWriteMany, segmentRuntimeStepPairPlans,
    singletonSegmentRuntimePlanOfStep, singletonSegmentRuntimePlan,
    segmentRuntimeCoordOfStep, coordinatePlanPair] using
    (coordRead_second_of_pair
      (read := fun bytes coord => segmentByteRead bytes coord.2.1 coord.2.2)
      (mark := segmentRuntimeMark)
      (hRead := fun bytes coord => segmentRuntimeRead_eq_byteMarkRead bytes coord)
      (bytes := bytes)
      (coord₁ := segmentRuntimeCoordOfStep hLo₁ hHi₁)
      (coord₂ := segmentRuntimeCoordOfStep hLo₂ hHi₂)
      hMarkByte)

theorem segmentRuntimeRead_step_of_consecutivePrefix (bytes : SegmentByteState)
    {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit)
    (hByte :
      segmentByteIndex lo (runtimeMarkedBy p segLo step) ≠
        segmentByteIndex lo (runtimeMarkedBy p segLo (step + 1))) :
    segmentByteRead
        (segmentRuntimeWriteMany bytes
          (segmentRuntimeConsecutivePlans hLo hHi hLoSucc hHiSucc))
        hLo hHi = 1 := by
  simpa [segmentRuntimeConsecutivePlans] using
    (segmentRuntimeRead_first_of_stepPair (bytes := bytes)
      (hLo₁ := hLo) (hHi₁ := hHi)
      (hLo₂ := hLoSucc) (hHi₂ := hHiSucc)
      hByte)

theorem segmentRuntimeRead_succ_of_consecutivePrefix (bytes : SegmentByteState)
    {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit)
    (hByte :
      segmentByteIndex lo (runtimeMarkedBy p segLo step) ≠
        segmentByteIndex lo (runtimeMarkedBy p segLo (step + 1))) :
    segmentByteRead
        (segmentRuntimeWriteMany bytes
          (segmentRuntimeConsecutivePlans hLo hHi hLoSucc hHiSucc))
        hLoSucc hHiSucc = 1 := by
  simpa [segmentRuntimeConsecutivePlans] using
    (segmentRuntimeRead_second_of_stepPair (bytes := bytes)
      (hLo₁ := hLo) (hHi₁ := hHi)
      (hLo₂ := hLoSucc) (hHi₂ := hHiSucc)
      hByte)

theorem segmentRuntimeConsecutive_byte_separated_of_eight_le_p
    {lo p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hP : 8 ≤ p) :
    segmentByteIndex lo (runtimeMarkedBy p segLo step) ≠
      segmentByteIndex lo (runtimeMarkedBy p segLo (step + 1)) := by
  rw [runtimeMarkedBy_succ]
  exact segmentByteIndex_ne_of_add_two_mul_ge_eight
    hLo hLoOdd (odd_runtimeMarkedBy hpOdd) hP

theorem segmentRuntimeRead_step_of_consecutivePrefix_of_eight_le_p (bytes : SegmentByteState)
    {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit)
    (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hP : 8 ≤ p) :
    segmentByteRead
        (segmentRuntimeWriteMany bytes
          (segmentRuntimeConsecutivePlans hLo hHi hLoSucc hHiSucc))
        hLo hHi = 1 := by
  exact segmentRuntimeRead_step_of_consecutivePrefix
    (bytes := bytes) (hLo := hLo) (hHi := hHi)
    (hLoSucc := hLoSucc) (hHiSucc := hHiSucc)
    (segmentRuntimeConsecutive_byte_separated_of_eight_le_p hLo hLoOdd hpOdd hP)

theorem segmentRuntimeRead_succ_of_consecutivePrefix_of_eight_le_p (bytes : SegmentByteState)
    {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit)
    (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hP : 8 ≤ p) :
    segmentByteRead
        (segmentRuntimeWriteMany bytes
          (segmentRuntimeConsecutivePlans hLo hHi hLoSucc hHiSucc))
        hLoSucc hHiSucc = 1 := by
  exact segmentRuntimeRead_succ_of_consecutivePrefix
    (bytes := bytes) (hLo := hLo) (hHi := hHi)
    (hLoSucc := hLoSucc) (hHiSucc := hHiSucc)
    (segmentRuntimeConsecutive_byte_separated_of_eight_le_p hLo hLoOdd hpOdd hP)

end PrimeArithmetic.Sieve
