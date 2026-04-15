import Mathlib
import PrimeArithmetic.Sieve.SegmentRuntimePlans

namespace PrimeArithmetic.Sieve

/-!
Step-indexed odd-only runtime mark families.

`SegmentRuntimePlans.lean` already packages grouped in-range odd candidates on
the shared coordinate-plan layer. This module moves one step closer to the
executable cross-off loop in `src/prime_sieve.rs`, which naturally iterates by
`step` in the arithmetic progression `runtimeMarkedBy p segLo step`.

Here we package a bounded step together with the proof that its runtime mark
stays inside the current segment, then reuse the same grouped coordinate-plan
surface without manually converting each step into a candidate coordinate first.
-/

/-- A runtime cross-off step whose marked odd candidate lies in the segment. -/
abbrev SegmentRuntimeStep (lo limit p segLo : ℕ) :=
  { step : ℕ // lo ≤ runtimeMarkedBy p segLo step ∧
      runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit }

/-- Convert a bounded runtime step into the corresponding in-range odd coordinate. -/
def segmentRuntimeCoordOfBoundedStep {lo limit p segLo : ℕ}
    (stepCoord : SegmentRuntimeStep lo limit p segLo) : SegmentRuntimeCoord lo limit :=
  segmentRuntimeCoordOfStep stepCoord.2.1 stepCoord.2.2

/-- Byte-mark view of a bounded runtime cross-off step. -/
def segmentRuntimeStepMark {lo limit p segLo : ℕ}
    (stepCoord : SegmentRuntimeStep lo limit p segLo) : ByteMark segBytes :=
  segmentRuntimeMark (segmentRuntimeCoordOfBoundedStep stepCoord)

/-- Grouped bounded-step plans bucketed by target byte slot. -/
abbrev SegmentRuntimeStepPlan (lo limit p segLo : ℕ) :=
  CoordinatePlan (SegmentRuntimeStep lo limit p segLo) segBytes

/-- Readback at the runtime-marked odd candidate named by a bounded step. -/
def segmentRuntimeStepRead (bytes : SegmentByteState) {lo limit p segLo : ℕ}
    (stepCoord : SegmentRuntimeStep lo limit p segLo) : ℕ :=
  segmentByteRead bytes stepCoord.2.1 stepCoord.2.2

/-- Grouped bounded-step writes on the shared coordinate-plan layer. -/
def segmentRuntimeStepWriteMany {lo limit p segLo : ℕ}
    (bytes : SegmentByteState)
    (plans : List (SegmentRuntimeStepPlan lo limit p segLo)) : SegmentByteState :=
  coordinatePlanWriteMany segmentRuntimeStepMark bytes plans

/-- Canonical per-byte bucketing for a finite bounded-step family. -/
def segmentRuntimeStepWriteByByte {lo limit p segLo : ℕ}
    (bytes : SegmentByteState)
    (steps : List (SegmentRuntimeStep lo limit p segLo)) : SegmentByteState :=
  coordinatePlanWriteMany segmentRuntimeStepMark bytes
    (coordinatePlansByByte segmentRuntimeStepMark steps)

/-- The first two bounded steps in one executable cross-off prefix. -/
def segmentRuntimeConsecutiveBoundedSteps {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit) :
    List (SegmentRuntimeStep lo limit p segLo) :=
  [⟨step, ⟨hLo, hHi⟩⟩, ⟨step + 1, ⟨hLoSucc, hHiSucc⟩⟩]

/-- Distinct targeted byte slots for grouped bounded-step plans. -/
def segmentRuntimeStepPlansHaveDistinctByteSlots {lo limit p segLo : ℕ}
    (plans : List (SegmentRuntimeStepPlan lo limit p segLo)) : Prop :=
  coordinatePlansHaveDistinctByteSlots segmentRuntimeStepMark plans

theorem segmentRuntimeStepRead_eq_byteMarkRead (bytes : SegmentByteState)
    {lo limit p segLo : ℕ}
    (stepCoord : SegmentRuntimeStep lo limit p segLo) :
    segmentRuntimeStepRead bytes stepCoord =
      byteMarkRead bytes (segmentRuntimeStepMark stepCoord) := by
  simpa [segmentRuntimeStepRead, segmentRuntimeStepMark,
    segmentRuntimeCoordOfBoundedStep] using
    (segmentRuntimeRead_eq_byteMarkRead bytes
      (coord := segmentRuntimeCoordOfBoundedStep stepCoord))

theorem segmentRuntimeStepRead_of_mem_plans_distinct {lo limit p segLo : ℕ}
    (plans : List (SegmentRuntimeStepPlan lo limit p segLo)) (bytes : SegmentByteState)
    (hAligned :
      ∀ plan ∈ plans, ∀ stepCoord ∈ plan.2, (segmentRuntimeStepMark stepCoord).1 = plan.1)
    (hDistinct : segmentRuntimeStepPlansHaveDistinctByteSlots plans)
    {plan : SegmentRuntimeStepPlan lo limit p segLo} (hPlan : plan ∈ plans)
    {stepCoord : SegmentRuntimeStep lo limit p segLo} (hStep : stepCoord ∈ plan.2) :
    segmentRuntimeStepRead (segmentRuntimeStepWriteMany bytes plans) stepCoord = 1 := by
  exact read_of_mem_coordinatePlans_distinct_of_eq
    segmentRuntimeStepMark
    (fun bytes stepCoord => segmentRuntimeStepRead bytes stepCoord)
    bytes plans
    (fun bytes stepCoord => segmentRuntimeStepRead_eq_byteMarkRead bytes stepCoord)
    hAligned hDistinct hPlan hStep

theorem segmentRuntimeStepRead_of_mem_byByte {lo limit p segLo : ℕ}
    (steps : List (SegmentRuntimeStep lo limit p segLo)) (bytes : SegmentByteState)
    {stepCoord : SegmentRuntimeStep lo limit p segLo} (hStep : stepCoord ∈ steps) :
    segmentRuntimeStepRead (segmentRuntimeStepWriteByByte bytes steps) stepCoord = 1 := by
  simpa [segmentRuntimeStepWriteByByte] using
    (read_of_mem_coordinatePlansByByte_of_eq
      segmentRuntimeStepMark
      (fun bytes stepCoord => segmentRuntimeStepRead bytes stepCoord)
      bytes steps
      (fun bytes stepCoord => segmentRuntimeStepRead_eq_byteMarkRead bytes stepCoord)
      hStep)

theorem segmentRuntimeRead_of_mem_stepPlans_distinct {lo limit p segLo : ℕ}
    (plans : List (SegmentRuntimeStepPlan lo limit p segLo)) (bytes : SegmentByteState)
    (hAligned :
      ∀ plan ∈ plans, ∀ stepCoord ∈ plan.2, (segmentRuntimeStepMark stepCoord).1 = plan.1)
    (hDistinct : segmentRuntimeStepPlansHaveDistinctByteSlots plans)
    {plan : SegmentRuntimeStepPlan lo limit p segLo} (hPlan : plan ∈ plans)
    {stepCoord : SegmentRuntimeStep lo limit p segLo} (hStep : stepCoord ∈ plan.2) :
    segmentByteRead (segmentRuntimeStepWriteMany bytes plans) stepCoord.2.1 stepCoord.2.2 = 1 := by
  simpa [segmentRuntimeStepRead] using
    (segmentRuntimeStepRead_of_mem_plans_distinct
      (plans := plans) (bytes := bytes) hAligned hDistinct hPlan hStep)

theorem segmentRuntimeRead_of_mem_boundedSteps_byByte {lo limit p segLo : ℕ}
    (steps : List (SegmentRuntimeStep lo limit p segLo)) (bytes : SegmentByteState)
    {stepCoord : SegmentRuntimeStep lo limit p segLo} (hStep : stepCoord ∈ steps) :
    segmentByteRead (segmentRuntimeStepWriteByByte bytes steps) stepCoord.2.1 stepCoord.2.2 = 1 := by
  simpa [segmentRuntimeStepRead] using
    (segmentRuntimeStepRead_of_mem_byByte
      (steps := steps) (bytes := bytes) hStep)

theorem segmentRuntimeRead_step_of_consecutiveBoundedSteps_byByte
    (bytes : SegmentByteState) {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit) :
    segmentByteRead
        (segmentRuntimeStepWriteByByte bytes
          (segmentRuntimeConsecutiveBoundedSteps hLo hHi hLoSucc hHiSucc))
        hLo hHi = 1 := by
  simpa [segmentRuntimeConsecutiveBoundedSteps] using
    (segmentRuntimeRead_of_mem_boundedSteps_byByte
      (steps := segmentRuntimeConsecutiveBoundedSteps hLo hHi hLoSucc hHiSucc)
      (bytes := bytes)
      (stepCoord := (⟨step, ⟨hLo, hHi⟩⟩ : SegmentRuntimeStep lo limit p segLo))
      (by simp [segmentRuntimeConsecutiveBoundedSteps]))

theorem segmentRuntimeRead_succ_of_consecutiveBoundedSteps_byByte
    (bytes : SegmentByteState) {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit) :
    segmentByteRead
        (segmentRuntimeStepWriteByByte bytes
          (segmentRuntimeConsecutiveBoundedSteps hLo hHi hLoSucc hHiSucc))
        hLoSucc hHiSucc = 1 := by
  simpa [segmentRuntimeConsecutiveBoundedSteps] using
    (segmentRuntimeRead_of_mem_boundedSteps_byByte
      (steps := segmentRuntimeConsecutiveBoundedSteps hLo hHi hLoSucc hHiSucc)
      (bytes := bytes)
      (stepCoord := (⟨step + 1, ⟨hLoSucc, hHiSucc⟩⟩ : SegmentRuntimeStep lo limit p segLo))
      (by simp [segmentRuntimeConsecutiveBoundedSteps]))

end PrimeArithmetic.Sieve
