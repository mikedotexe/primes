import Mathlib
import PrimeArithmetic.Sieve.RuntimeCollection
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

/-- The singleton grouped plan attached to one bounded runtime step. -/
def singletonSegmentRuntimeStepPlan {lo limit p segLo : ℕ}
    (stepCoord : SegmentRuntimeStep lo limit p segLo) :
    SegmentRuntimeStepPlan lo limit p segLo :=
  singletonCoordinatePlan segmentRuntimeStepMark stepCoord

/-- Runtime-coordinate image of a bounded-step family. -/
def segmentRuntimeCoordsOfBoundedSteps {lo limit p segLo : ℕ}
    (steps : List (SegmentRuntimeStep lo limit p segLo)) : List (SegmentRuntimeCoord lo limit) :=
  steps.map segmentRuntimeCoordOfBoundedStep

/-- Forget bounded-step payloads inside one grouped plan and keep the induced coordinates. -/
def SegmentRuntimeStepPlan.toCoordPlan {lo limit p segLo : ℕ}
    (plan : SegmentRuntimeStepPlan lo limit p segLo) : SegmentRuntimePlan lo limit :=
  CoordinatePlan.map segmentRuntimeCoordOfBoundedStep plan

@[simp] theorem SegmentRuntimeStepPlan.toCoordPlan_fst {lo limit p segLo : ℕ}
    (plan : SegmentRuntimeStepPlan lo limit p segLo) :
    plan.toCoordPlan.1 = plan.1 := by
  simp [SegmentRuntimeStepPlan.toCoordPlan]

@[simp] theorem SegmentRuntimeStepPlan.toCoordPlan_snd {lo limit p segLo : ℕ}
    (plan : SegmentRuntimeStepPlan lo limit p segLo) :
    plan.toCoordPlan.2 = plan.2.map segmentRuntimeCoordOfBoundedStep := by
  simp [SegmentRuntimeStepPlan.toCoordPlan]

/-- The first two bounded steps in one executable cross-off prefix. -/
def segmentRuntimeConsecutiveBoundedSteps {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit) :
    List (SegmentRuntimeStep lo limit p segLo) :=
  [⟨step, ⟨hLo, hHi⟩⟩, ⟨step + 1, ⟨hLoSucc, hHiSucc⟩⟩]

/-- The bounded-step family with two named runtime cross-off steps. -/
def segmentRuntimeBoundedStepPair {lo limit p segLo step₁ step₂ : ℕ}
    (hLo₁ : lo ≤ runtimeMarkedBy p segLo step₁)
    (hHi₁ : runtimeMarkedBy p segLo step₁ ≤ rawSegmentHi lo limit)
    (hLo₂ : lo ≤ runtimeMarkedBy p segLo step₂)
    (hHi₂ : runtimeMarkedBy p segLo step₂ ≤ rawSegmentHi lo limit) :
    List (SegmentRuntimeStep lo limit p segLo) :=
  [⟨step₁, ⟨hLo₁, hHi₁⟩⟩, ⟨step₂, ⟨hLo₂, hHi₂⟩⟩]

/-- Distinct targeted byte slots for grouped bounded-step plans. -/
def segmentRuntimeStepPlansHaveDistinctByteSlots {lo limit p segLo : ℕ}
    (plans : List (SegmentRuntimeStepPlan lo limit p segLo)) : Prop :=
  coordinatePlansHaveDistinctByteSlots segmentRuntimeStepMark plans

/-- The grouped step plans attached to two bounded runtime steps. -/
def segmentRuntimeBoundedStepPairPlans {lo limit p segLo step₁ step₂ : ℕ}
    (hLo₁ : lo ≤ runtimeMarkedBy p segLo step₁)
    (hHi₁ : runtimeMarkedBy p segLo step₁ ≤ rawSegmentHi lo limit)
    (hLo₂ : lo ≤ runtimeMarkedBy p segLo step₂)
    (hHi₂ : runtimeMarkedBy p segLo step₂ ≤ rawSegmentHi lo limit) :
    List (SegmentRuntimeStepPlan lo limit p segLo) :=
  coordinatePlanPair segmentRuntimeStepMark
    (⟨step₁, ⟨hLo₁, hHi₁⟩⟩ : SegmentRuntimeStep lo limit p segLo)
    (⟨step₂, ⟨hLo₂, hHi₂⟩⟩ : SegmentRuntimeStep lo limit p segLo)

/-- The grouped step plans attached to two consecutive bounded runtime steps. -/
def segmentRuntimeConsecutiveBoundedStepPlans {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit) :
    List (SegmentRuntimeStepPlan lo limit p segLo) :=
  segmentRuntimeBoundedStepPairPlans hLo hHi hLoSucc hHiSucc

@[simp] theorem segmentRuntimeBoundedStepPairPlans_toCoordPlans
    {lo limit p segLo step₁ step₂ : ℕ}
    (hLo₁ : lo ≤ runtimeMarkedBy p segLo step₁)
    (hHi₁ : runtimeMarkedBy p segLo step₁ ≤ rawSegmentHi lo limit)
    (hLo₂ : lo ≤ runtimeMarkedBy p segLo step₂)
    (hHi₂ : runtimeMarkedBy p segLo step₂ ≤ rawSegmentHi lo limit) :
    (segmentRuntimeBoundedStepPairPlans hLo₁ hHi₁ hLo₂ hHi₂).map
        (fun plan => plan.toCoordPlan) =
      segmentRuntimeStepPairPlans hLo₁ hHi₁ hLo₂ hHi₂ := by
  simpa [segmentRuntimeBoundedStepPairPlans, segmentRuntimeStepPairPlans,
    SegmentRuntimeStepPlan.toCoordPlan] using
    (coordinatePlanPair_map_eq_of_mark_eq
      (mark₁ := segmentRuntimeStepMark)
      (mark₂ := segmentRuntimeMark)
      (f := segmentRuntimeCoordOfBoundedStep)
      (hMark := fun stepCoord => by
        simp [segmentRuntimeStepMark, segmentRuntimeCoordOfBoundedStep])
      (coord₁ := (⟨step₁, ⟨hLo₁, hHi₁⟩⟩ : SegmentRuntimeStep lo limit p segLo))
      (coord₂ := (⟨step₂, ⟨hLo₂, hHi₂⟩⟩ : SegmentRuntimeStep lo limit p segLo)))

@[simp] theorem segmentRuntimeConsecutiveBoundedStepPlans_toCoordPlans
    {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit) :
    (segmentRuntimeConsecutiveBoundedStepPlans hLo hHi hLoSucc hHiSucc).map
        (fun plan => plan.toCoordPlan) =
      segmentRuntimeConsecutivePlans hLo hHi hLoSucc hHiSucc := by
  rw [segmentRuntimeConsecutiveBoundedStepPlans, segmentRuntimeConsecutivePlans]
  exact segmentRuntimeBoundedStepPairPlans_toCoordPlans
    (hLo₁ := hLo) (hHi₁ := hHi) (hLo₂ := hLoSucc) (hHi₂ := hHiSucc)

theorem segmentRuntimeStepRead_eq_byteMarkRead (bytes : SegmentByteState)
    {lo limit p segLo : ℕ}
    (stepCoord : SegmentRuntimeStep lo limit p segLo) :
    segmentRuntimeStepRead bytes stepCoord =
      byteMarkRead bytes (segmentRuntimeStepMark stepCoord) := by
  simpa [segmentRuntimeStepRead, segmentRuntimeStepMark,
    segmentRuntimeCoordOfBoundedStep] using
    (segmentRuntimeRead_eq_byteMarkRead bytes
      (coord := segmentRuntimeCoordOfBoundedStep stepCoord))

theorem segmentByteRead_proof_irrel (bytes : SegmentByteState)
    {lo limit n : ℕ}
    {hLo₁ hLo₂ : lo ≤ n}
    {hHi₁ hHi₂ : n ≤ rawSegmentHi lo limit} :
    segmentByteRead bytes hLo₁ hHi₁ = segmentByteRead bytes hLo₂ hHi₂ := by
  unfold segmentByteRead segmentByteSlot
  simp

theorem segmentRuntimeStepRead_singleton (bytes : SegmentByteState)
    {lo limit p segLo : ℕ}
    (stepCoord : SegmentRuntimeStep lo limit p segLo) :
    segmentRuntimeStepRead
        (segmentRuntimeStepWriteMany bytes [singletonSegmentRuntimeStepPlan stepCoord])
        stepCoord = 1 := by
  simpa [segmentRuntimeStepWriteMany, singletonSegmentRuntimeStepPlan] using
    (coordRead_singleton
      (read := fun bytes stepCoord => segmentRuntimeStepRead bytes stepCoord)
      (mark := segmentRuntimeStepMark)
      (hRead := fun bytes stepCoord => segmentRuntimeStepRead_eq_byteMarkRead bytes stepCoord)
      (bytes := bytes) (coord := stepCoord))

theorem segmentRuntimeRead_singleton_of_boundedStep (bytes : SegmentByteState)
    {lo limit p segLo : ℕ}
    (stepCoord : SegmentRuntimeStep lo limit p segLo) :
    segmentByteRead
        (segmentRuntimeStepWriteMany bytes [singletonSegmentRuntimeStepPlan stepCoord])
        stepCoord.2.1 stepCoord.2.2 = 1 := by
  simpa [segmentRuntimeStepRead] using
    (segmentRuntimeStepRead_singleton (bytes := bytes) stepCoord)

theorem segmentRuntimeStepRead_singleton_byByte (bytes : SegmentByteState)
    {lo limit p segLo : ℕ}
    (stepCoord : SegmentRuntimeStep lo limit p segLo) :
    segmentRuntimeStepRead
        (segmentRuntimeStepWriteByByte bytes [stepCoord])
        stepCoord = 1 := by
  simpa [segmentRuntimeStepWriteByByte] using
    (coordRead_singleton_byByte
      (fun bytes stepCoord => segmentRuntimeStepRead bytes stepCoord)
      segmentRuntimeStepMark
      (bytes := bytes) (coord := stepCoord)
      (fun bytes stepCoord => segmentRuntimeStepRead_eq_byteMarkRead bytes stepCoord)
      )

theorem segmentRuntimeRead_singleton_of_boundedStep_byByte
    (bytes : SegmentByteState)
    {lo limit p segLo : ℕ}
    (stepCoord : SegmentRuntimeStep lo limit p segLo) :
    segmentByteRead
        (segmentRuntimeStepWriteByByte bytes [stepCoord])
        stepCoord.2.1 stepCoord.2.2 = 1 := by
  simpa [segmentRuntimeStepRead] using
    (segmentRuntimeStepRead_singleton_byByte (bytes := bytes) stepCoord)

/-- The first bounded runtime step, stated directly at the actual cross-off start. -/
def segmentRuntimeFirstBoundedStep {lo limit p segLo : ℕ}
    (hLo : lo ≤ runtimeCrossOffStart p segLo)
    (hHi : runtimeCrossOffStart p segLo ≤ rawSegmentHi lo limit) :
    SegmentRuntimeStep lo limit p segLo :=
  ⟨0, by simpa using And.intro hLo hHi⟩

@[simp] theorem segmentRuntimeFirstBoundedStep_val
    {lo limit p segLo : ℕ}
    (hLo : lo ≤ runtimeCrossOffStart p segLo)
    (hHi : runtimeCrossOffStart p segLo ≤ rawSegmentHi lo limit) :
    (segmentRuntimeFirstBoundedStep hLo hHi).1 = 0 := rfl

@[simp] theorem segmentRuntimeCoordOfFirstBoundedStep_val
    {lo limit p segLo : ℕ}
    (hLo : lo ≤ runtimeCrossOffStart p segLo)
    (hHi : runtimeCrossOffStart p segLo ≤ rawSegmentHi lo limit) :
    (segmentRuntimeCoordOfBoundedStep (segmentRuntimeFirstBoundedStep hLo hHi)).1 =
      runtimeCrossOffStart p segLo := by
  simp [segmentRuntimeFirstBoundedStep, segmentRuntimeCoordOfBoundedStep,
    segmentRuntimeCoordOfStep, runtimeMarkedBy_zero]

theorem segmentRuntimeRead_firstBoundedStep
    (bytes : SegmentByteState)
    {lo limit p segLo : ℕ}
    (hLo : lo ≤ runtimeCrossOffStart p segLo)
    (hHi : runtimeCrossOffStart p segLo ≤ rawSegmentHi lo limit) :
    segmentByteRead
        (segmentRuntimeStepWriteMany bytes
          [singletonSegmentRuntimeStepPlan (segmentRuntimeFirstBoundedStep hLo hHi)])
        hLo hHi = 1 := by
  simpa [segmentRuntimeFirstBoundedStep] using
    (segmentRuntimeRead_singleton_of_boundedStep (bytes := bytes)
      (stepCoord := segmentRuntimeFirstBoundedStep hLo hHi))

theorem segmentRuntimeRead_firstBoundedStep_byByte
    (bytes : SegmentByteState)
    {lo limit p segLo : ℕ}
    (hLo : lo ≤ runtimeCrossOffStart p segLo)
    (hHi : runtimeCrossOffStart p segLo ≤ rawSegmentHi lo limit) :
    segmentByteRead
        (segmentRuntimeStepWriteByByte bytes
          [segmentRuntimeFirstBoundedStep hLo hHi])
        hLo hHi = 1 := by
  simpa [segmentRuntimeFirstBoundedStep] using
    (segmentRuntimeRead_singleton_of_boundedStep_byByte (bytes := bytes)
      (stepCoord := segmentRuntimeFirstBoundedStep hLo hHi))

/-- The second bounded runtime step, stated directly at the next executable mark. -/
def segmentRuntimeSecondBoundedStep {lo limit p segLo : ℕ}
    (hLo : lo ≤ runtimeCrossOffStart p segLo + 2 * p)
    (hHi : runtimeCrossOffStart p segLo + 2 * p ≤ rawSegmentHi lo limit) :
    SegmentRuntimeStep lo limit p segLo :=
  ⟨1, by
    simpa [runtimeMarkedBy_succ, runtimeMarkedBy_zero] using And.intro hLo hHi⟩

@[simp] theorem segmentRuntimeSecondBoundedStep_val
    {lo limit p segLo : ℕ}
    (hLo : lo ≤ runtimeCrossOffStart p segLo + 2 * p)
    (hHi : runtimeCrossOffStart p segLo + 2 * p ≤ rawSegmentHi lo limit) :
    (segmentRuntimeSecondBoundedStep hLo hHi).1 = 1 := rfl

@[simp] theorem segmentRuntimeCoordOfSecondBoundedStep_val
    {lo limit p segLo : ℕ}
    (hLo : lo ≤ runtimeCrossOffStart p segLo + 2 * p)
    (hHi : runtimeCrossOffStart p segLo + 2 * p ≤ rawSegmentHi lo limit) :
    (segmentRuntimeCoordOfBoundedStep (segmentRuntimeSecondBoundedStep hLo hHi)).1 =
      runtimeCrossOffStart p segLo + 2 * p := by
  simp [segmentRuntimeSecondBoundedStep, segmentRuntimeCoordOfBoundedStep,
    segmentRuntimeCoordOfStep, runtimeMarkedBy_succ, runtimeMarkedBy_zero]

/-- The initial two-step bounded runtime prefix, stated in executable arithmetic. -/
def segmentRuntimeInitialConsecutiveBoundedSteps {lo limit p segLo : ℕ}
    (hLo : lo ≤ runtimeCrossOffStart p segLo)
    (hHi : runtimeCrossOffStart p segLo ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeCrossOffStart p segLo + 2 * p)
    (hHiSucc : runtimeCrossOffStart p segLo + 2 * p ≤ rawSegmentHi lo limit) :
    List (SegmentRuntimeStep lo limit p segLo) :=
  [segmentRuntimeFirstBoundedStep hLo hHi,
    segmentRuntimeSecondBoundedStep hLoSucc hHiSucc]

/-- Grouped plans for the initial two-step runtime prefix. -/
def segmentRuntimeInitialConsecutiveBoundedStepPlans {lo limit p segLo : ℕ}
    (hLo : lo ≤ runtimeCrossOffStart p segLo)
    (hHi : runtimeCrossOffStart p segLo ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeCrossOffStart p segLo + 2 * p)
    (hHiSucc : runtimeCrossOffStart p segLo + 2 * p ≤ rawSegmentHi lo limit) :
    List (SegmentRuntimeStepPlan lo limit p segLo) :=
  segmentRuntimeConsecutiveBoundedStepPlans
    (step := 0)
    (by simpa [runtimeMarkedBy_zero] using hLo)
    (by simpa [runtimeMarkedBy_zero] using hHi)
    (by simpa [runtimeMarkedBy_succ, runtimeMarkedBy_zero] using hLoSucc)
    (by simpa [runtimeMarkedBy_succ, runtimeMarkedBy_zero] using hHiSucc)

@[simp] theorem segmentRuntimeInitialConsecutiveBoundedSteps_eq_consecutiveBoundedSteps
    {lo limit p segLo : ℕ}
    (hLo : lo ≤ runtimeCrossOffStart p segLo)
    (hHi : runtimeCrossOffStart p segLo ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeCrossOffStart p segLo + 2 * p)
    (hHiSucc : runtimeCrossOffStart p segLo + 2 * p ≤ rawSegmentHi lo limit) :
    segmentRuntimeInitialConsecutiveBoundedSteps hLo hHi hLoSucc hHiSucc =
      segmentRuntimeConsecutiveBoundedSteps
        (step := 0)
        (by simpa [runtimeMarkedBy_zero] using hLo)
        (by simpa [runtimeMarkedBy_zero] using hHi)
        (by simpa [runtimeMarkedBy_succ, runtimeMarkedBy_zero] using hLoSucc)
        (by simpa [runtimeMarkedBy_succ, runtimeMarkedBy_zero] using hHiSucc) := by
  simp [segmentRuntimeInitialConsecutiveBoundedSteps,
    segmentRuntimeConsecutiveBoundedSteps, segmentRuntimeFirstBoundedStep,
    segmentRuntimeSecondBoundedStep]

@[simp] theorem segmentRuntimeInitialConsecutiveBoundedStepPlans_eq_consecutiveBoundedStepPlans
    {lo limit p segLo : ℕ}
    (hLo : lo ≤ runtimeCrossOffStart p segLo)
    (hHi : runtimeCrossOffStart p segLo ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeCrossOffStart p segLo + 2 * p)
    (hHiSucc : runtimeCrossOffStart p segLo + 2 * p ≤ rawSegmentHi lo limit) :
    segmentRuntimeInitialConsecutiveBoundedStepPlans hLo hHi hLoSucc hHiSucc =
      segmentRuntimeConsecutiveBoundedStepPlans
        (step := 0)
        (by simpa [runtimeMarkedBy_zero] using hLo)
        (by simpa [runtimeMarkedBy_zero] using hHi)
        (by simpa [runtimeMarkedBy_succ, runtimeMarkedBy_zero] using hLoSucc)
        (by simpa [runtimeMarkedBy_succ, runtimeMarkedBy_zero] using hHiSucc) := by
  rfl

theorem segmentRuntimeStepWriteMany_eq_mappedCoordPlansWriteMany
    {lo limit p segLo : ℕ}
    (bytes : SegmentByteState)
    (plans : List (SegmentRuntimeStepPlan lo limit p segLo)) :
    segmentRuntimeStepWriteMany bytes plans =
      segmentRuntimeWriteMany bytes (plans.map fun plan => plan.toCoordPlan) := by
  simpa [segmentRuntimeStepWriteMany, segmentRuntimeWriteMany,
    SegmentRuntimeStepPlan.toCoordPlan, coordinateWriteMany] using
    (coordinatePlanWriteMany_eq_mappedPlans_of_mark_eq
      (mark₁ := segmentRuntimeStepMark)
      (mark₂ := segmentRuntimeMark)
      (f := segmentRuntimeCoordOfBoundedStep)
      (hMark := fun stepCoord => by
        simp [segmentRuntimeStepMark, segmentRuntimeCoordOfBoundedStep])
      (bytes := bytes) (plans := plans))

theorem segmentRuntimeStepWriteMany_boundedStepPairPlans_eq_stepPairPlans
    (bytes : SegmentByteState)
    {lo limit p segLo step₁ step₂ : ℕ}
    (hLo₁ : lo ≤ runtimeMarkedBy p segLo step₁)
    (hHi₁ : runtimeMarkedBy p segLo step₁ ≤ rawSegmentHi lo limit)
    (hLo₂ : lo ≤ runtimeMarkedBy p segLo step₂)
    (hHi₂ : runtimeMarkedBy p segLo step₂ ≤ rawSegmentHi lo limit) :
    segmentRuntimeStepWriteMany bytes
        (segmentRuntimeBoundedStepPairPlans hLo₁ hHi₁ hLo₂ hHi₂) =
      segmentRuntimeWriteMany bytes
        (segmentRuntimeStepPairPlans hLo₁ hHi₁ hLo₂ hHi₂) := by
  rw [segmentRuntimeStepWriteMany_eq_mappedCoordPlansWriteMany]
  rw [segmentRuntimeBoundedStepPairPlans_toCoordPlans]

theorem segmentRuntimeStepWriteMany_consecutiveBoundedStepPlans_eq_consecutivePlans
    (bytes : SegmentByteState)
    {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit) :
    segmentRuntimeStepWriteMany bytes
        (segmentRuntimeConsecutiveBoundedStepPlans hLo hHi hLoSucc hHiSucc) =
      segmentRuntimeWriteMany bytes
        (segmentRuntimeConsecutivePlans hLo hHi hLoSucc hHiSucc) := by
  rw [segmentRuntimeStepWriteMany_eq_mappedCoordPlansWriteMany]
  rw [segmentRuntimeConsecutiveBoundedStepPlans_toCoordPlans]

theorem segmentRuntimeStepPlans_toCoordPlans_aligned_iff
    {lo limit p segLo : ℕ}
    (plans : List (SegmentRuntimeStepPlan lo limit p segLo)) :
    (∀ plan ∈ plans.map (fun plan => plan.toCoordPlan),
        ∀ coord ∈ plan.2, (segmentRuntimeMark coord).1 = plan.1) ↔
      (∀ plan ∈ plans, ∀ stepCoord ∈ plan.2, (segmentRuntimeStepMark stepCoord).1 = plan.1) := by
  simpa [SegmentRuntimeStepPlan.toCoordPlan] using
    (coordinatePlans_mapped_aligned_iff_of_mark_eq
      (mark₁ := segmentRuntimeStepMark)
      (mark₂ := segmentRuntimeMark)
      (f := segmentRuntimeCoordOfBoundedStep)
      (hMark := fun stepCoord => by
        simp [segmentRuntimeStepMark, segmentRuntimeCoordOfBoundedStep])
      (plans := plans))

theorem segmentRuntimeStepPlans_toCoordPlans_aligned
    {lo limit p segLo : ℕ}
    (plans : List (SegmentRuntimeStepPlan lo limit p segLo))
    (hAligned :
      ∀ plan ∈ plans, ∀ stepCoord ∈ plan.2, (segmentRuntimeStepMark stepCoord).1 = plan.1) :
    ∀ plan ∈ plans.map (fun plan => plan.toCoordPlan),
      ∀ coord ∈ plan.2, (segmentRuntimeMark coord).1 = plan.1 := by
  exact (segmentRuntimeStepPlans_toCoordPlans_aligned_iff
    (lo := lo) (limit := limit) (p := p) (segLo := segLo) plans).2 hAligned

theorem segmentRuntimeStepPlans_aligned_of_toCoordPlans
    {lo limit p segLo : ℕ}
    (plans : List (SegmentRuntimeStepPlan lo limit p segLo))
    (hAligned :
      ∀ plan ∈ plans.map (fun plan => plan.toCoordPlan),
        ∀ coord ∈ plan.2, (segmentRuntimeMark coord).1 = plan.1) :
    ∀ plan ∈ plans, ∀ stepCoord ∈ plan.2, (segmentRuntimeStepMark stepCoord).1 = plan.1 := by
  exact (segmentRuntimeStepPlans_toCoordPlans_aligned_iff
    (lo := lo) (limit := limit) (p := p) (segLo := segLo) plans).1 hAligned

theorem segmentRuntimeStepPlans_toCoordPlans_distinctByteSlots_iff
    {lo limit p segLo : ℕ}
    (plans : List (SegmentRuntimeStepPlan lo limit p segLo)) :
    coordinatePlansHaveDistinctByteSlots segmentRuntimeMark
        (plans.map fun plan => plan.toCoordPlan) ↔
      segmentRuntimeStepPlansHaveDistinctByteSlots plans := by
  simpa [segmentRuntimeStepPlansHaveDistinctByteSlots, SegmentRuntimeStepPlan.toCoordPlan] using
    (coordinatePlans_mapped_distinctByteSlots_iff_of_mark_eq
      (mark₁ := segmentRuntimeStepMark)
      (mark₂ := segmentRuntimeMark)
      (f := segmentRuntimeCoordOfBoundedStep)
      (hMark := fun stepCoord => by
        simp [segmentRuntimeStepMark, segmentRuntimeCoordOfBoundedStep])
      (plans := plans))

theorem segmentRuntimeStepPlans_toCoordPlans_distinctByteSlots
    {lo limit p segLo : ℕ}
    (plans : List (SegmentRuntimeStepPlan lo limit p segLo))
    (hDistinct : segmentRuntimeStepPlansHaveDistinctByteSlots plans) :
    coordinatePlansHaveDistinctByteSlots segmentRuntimeMark
      (plans.map fun plan => plan.toCoordPlan) := by
  exact (segmentRuntimeStepPlans_toCoordPlans_distinctByteSlots_iff
    (lo := lo) (limit := limit) (p := p) (segLo := segLo) plans).2 hDistinct

theorem segmentRuntimeStepPlans_distinctByteSlots_of_toCoordPlans
    {lo limit p segLo : ℕ}
    (plans : List (SegmentRuntimeStepPlan lo limit p segLo))
    (hDistinct :
      coordinatePlansHaveDistinctByteSlots segmentRuntimeMark
        (plans.map fun plan => plan.toCoordPlan)) :
    segmentRuntimeStepPlansHaveDistinctByteSlots plans := by
  exact (segmentRuntimeStepPlans_toCoordPlans_distinctByteSlots_iff
    (lo := lo) (limit := limit) (p := p) (segLo := segLo) plans).1 hDistinct

theorem segmentRuntimeStepRead_of_mem_mappedPlans_distinct
    {lo limit p segLo : ℕ}
    (plans : List (SegmentRuntimeStepPlan lo limit p segLo)) (bytes : SegmentByteState)
    (hAligned :
      ∀ plan ∈ plans, ∀ stepCoord ∈ plan.2, (segmentRuntimeStepMark stepCoord).1 = plan.1)
    (hDistinct : segmentRuntimeStepPlansHaveDistinctByteSlots plans)
    {plan : SegmentRuntimeStepPlan lo limit p segLo} (hPlan : plan ∈ plans)
    {stepCoord : SegmentRuntimeStep lo limit p segLo} (hStep : stepCoord ∈ plan.2) :
    segmentRuntimeStepRead
        (segmentRuntimeWriteMany bytes
          (plans.map fun plan => plan.toCoordPlan))
        stepCoord = 1 := by
  simpa [segmentRuntimeStepRead, segmentRuntimeWriteMany, SegmentRuntimeStepPlan.toCoordPlan,
    segmentRuntimeCoordOfBoundedStep, coordinateWriteMany] using
    (coordRead_of_mem_mappedPlans_distinct_of_mark_eq
      (read := fun bytes stepCoord => segmentRuntimeStepRead bytes stepCoord)
      (mark₁ := segmentRuntimeStepMark)
      (mark₂ := segmentRuntimeMark)
      (hRead := fun bytes stepCoord => segmentRuntimeStepRead_eq_byteMarkRead bytes stepCoord)
      (f := segmentRuntimeCoordOfBoundedStep)
      (hMark := fun stepCoord => by
        simp [segmentRuntimeStepMark, segmentRuntimeCoordOfBoundedStep])
      (bytes := bytes) (plans := plans)
      (hAligned := hAligned) (hDistinct := hDistinct)
      (plan := plan) (coord := stepCoord) hPlan hStep)

theorem segmentRuntimeRead_of_mem_mappedStepPlans_distinct
    {lo limit p segLo : ℕ}
    (plans : List (SegmentRuntimeStepPlan lo limit p segLo)) (bytes : SegmentByteState)
    (hAligned :
      ∀ plan ∈ plans, ∀ stepCoord ∈ plan.2, (segmentRuntimeStepMark stepCoord).1 = plan.1)
    (hDistinct : segmentRuntimeStepPlansHaveDistinctByteSlots plans)
    {plan : SegmentRuntimeStepPlan lo limit p segLo} (hPlan : plan ∈ plans)
    {stepCoord : SegmentRuntimeStep lo limit p segLo} (hStep : stepCoord ∈ plan.2) :
    segmentByteRead
        (segmentRuntimeWriteMany bytes
          (plans.map fun plan => plan.toCoordPlan))
        stepCoord.2.1 stepCoord.2.2 = 1 := by
  simpa [segmentRuntimeStepRead] using
    (segmentRuntimeStepRead_of_mem_mappedPlans_distinct
      (plans := plans) (bytes := bytes) hAligned hDistinct hPlan hStep)

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

theorem segmentRuntimeRead_first_of_boundedStepPair (bytes : SegmentByteState)
    {lo limit p segLo step₁ step₂ : ℕ}
    (hLo₁ : lo ≤ runtimeMarkedBy p segLo step₁)
    (hHi₁ : runtimeMarkedBy p segLo step₁ ≤ rawSegmentHi lo limit)
    (hLo₂ : lo ≤ runtimeMarkedBy p segLo step₂)
    (hHi₂ : runtimeMarkedBy p segLo step₂ ≤ rawSegmentHi lo limit)
    (hByte :
      segmentByteIndex lo (runtimeMarkedBy p segLo step₁) ≠
        segmentByteIndex lo (runtimeMarkedBy p segLo step₂)) :
    segmentByteRead
        (segmentRuntimeStepWriteMany bytes
          (segmentRuntimeBoundedStepPairPlans hLo₁ hHi₁ hLo₂ hHi₂))
        hLo₁ hHi₁ = 1 := by
  let stepCoord₁ : SegmentRuntimeStep lo limit p segLo := ⟨step₁, ⟨hLo₁, hHi₁⟩⟩
  let stepCoord₂ : SegmentRuntimeStep lo limit p segLo := ⟨step₂, ⟨hLo₂, hHi₂⟩⟩
  have hMarkByte :
      (segmentRuntimeStepMark stepCoord₁).1 ≠ (segmentRuntimeStepMark stepCoord₂).1 := by
    intro hEq
    apply hByte
    simpa [stepCoord₁, stepCoord₂, segmentRuntimeStepMark, segmentRuntimeCoordOfBoundedStep,
      segmentRuntimeMark, segmentRuntimeCoordOfStep, segmentByteMark, segmentByteSlot] using
      congrArg Fin.val hEq
  simpa [segmentRuntimeStepWriteMany, segmentRuntimeBoundedStepPairPlans, stepCoord₁,
    stepCoord₂, segmentRuntimeStepRead, coordinateWriteMany, coordinatePlanPair] using
    (coordRead_first_of_pair
      (read := fun bytes stepCoord => segmentRuntimeStepRead bytes stepCoord)
      (mark := segmentRuntimeStepMark)
      (hRead := fun bytes stepCoord => segmentRuntimeStepRead_eq_byteMarkRead bytes stepCoord)
      (bytes := bytes) (coord₁ := stepCoord₁) (coord₂ := stepCoord₂) hMarkByte)

theorem segmentRuntimeRead_second_of_boundedStepPair (bytes : SegmentByteState)
    {lo limit p segLo step₁ step₂ : ℕ}
    (hLo₁ : lo ≤ runtimeMarkedBy p segLo step₁)
    (hHi₁ : runtimeMarkedBy p segLo step₁ ≤ rawSegmentHi lo limit)
    (hLo₂ : lo ≤ runtimeMarkedBy p segLo step₂)
    (hHi₂ : runtimeMarkedBy p segLo step₂ ≤ rawSegmentHi lo limit)
    (hByte :
      segmentByteIndex lo (runtimeMarkedBy p segLo step₁) ≠
        segmentByteIndex lo (runtimeMarkedBy p segLo step₂)) :
    segmentByteRead
        (segmentRuntimeStepWriteMany bytes
          (segmentRuntimeBoundedStepPairPlans hLo₁ hHi₁ hLo₂ hHi₂))
        hLo₂ hHi₂ = 1 := by
  let stepCoord₁ : SegmentRuntimeStep lo limit p segLo := ⟨step₁, ⟨hLo₁, hHi₁⟩⟩
  let stepCoord₂ : SegmentRuntimeStep lo limit p segLo := ⟨step₂, ⟨hLo₂, hHi₂⟩⟩
  have hMarkByte :
      (segmentRuntimeStepMark stepCoord₁).1 ≠ (segmentRuntimeStepMark stepCoord₂).1 := by
    intro hEq
    apply hByte
    simpa [stepCoord₁, stepCoord₂, segmentRuntimeStepMark, segmentRuntimeCoordOfBoundedStep,
      segmentRuntimeMark, segmentRuntimeCoordOfStep, segmentByteMark, segmentByteSlot] using
      congrArg Fin.val hEq
  simpa [segmentRuntimeStepWriteMany, segmentRuntimeBoundedStepPairPlans, stepCoord₁,
    stepCoord₂, segmentRuntimeStepRead, coordinateWriteMany, coordinatePlanPair] using
    (coordRead_second_of_pair
      (read := fun bytes stepCoord => segmentRuntimeStepRead bytes stepCoord)
      (mark := segmentRuntimeStepMark)
      (hRead := fun bytes stepCoord => segmentRuntimeStepRead_eq_byteMarkRead bytes stepCoord)
      (bytes := bytes) (coord₁ := stepCoord₁) (coord₂ := stepCoord₂) hMarkByte)

theorem segmentRuntimeReads_of_boundedStepPair (bytes : SegmentByteState)
    {lo limit p segLo step₁ step₂ : ℕ}
    (hLo₁ : lo ≤ runtimeMarkedBy p segLo step₁)
    (hHi₁ : runtimeMarkedBy p segLo step₁ ≤ rawSegmentHi lo limit)
    (hLo₂ : lo ≤ runtimeMarkedBy p segLo step₂)
    (hHi₂ : runtimeMarkedBy p segLo step₂ ≤ rawSegmentHi lo limit)
    (hByte :
      segmentByteIndex lo (runtimeMarkedBy p segLo step₁) ≠
        segmentByteIndex lo (runtimeMarkedBy p segLo step₂)) :
    segmentByteRead
        (segmentRuntimeStepWriteMany bytes
          (segmentRuntimeBoundedStepPairPlans hLo₁ hHi₁ hLo₂ hHi₂))
        hLo₁ hHi₁ = 1 ∧
      segmentByteRead
        (segmentRuntimeStepWriteMany bytes
          (segmentRuntimeBoundedStepPairPlans hLo₁ hHi₁ hLo₂ hHi₂))
        hLo₂ hHi₂ = 1 := by
  constructor
  · exact segmentRuntimeRead_first_of_boundedStepPair
      (bytes := bytes) hLo₁ hHi₁ hLo₂ hHi₂ hByte
  · exact segmentRuntimeRead_second_of_boundedStepPair
      (bytes := bytes) hLo₁ hHi₁ hLo₂ hHi₂ hByte

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

theorem segmentRuntimeCoordOfBoundedStep_mem_coordsOfBoundedSteps
    {lo limit p segLo : ℕ}
    (steps : List (SegmentRuntimeStep lo limit p segLo))
    {stepCoord : SegmentRuntimeStep lo limit p segLo} (hStep : stepCoord ∈ steps) :
    segmentRuntimeCoordOfBoundedStep stepCoord ∈ segmentRuntimeCoordsOfBoundedSteps steps := by
  unfold segmentRuntimeCoordsOfBoundedSteps
  exact List.mem_map.mpr ⟨stepCoord, hStep, rfl⟩

theorem segmentRuntimeCoordOfBoundedStep_injective
    {lo limit p segLo : ℕ} (hp : 0 < p) :
    Function.Injective
      (segmentRuntimeCoordOfBoundedStep (lo := lo) (limit := limit) (p := p) (segLo := segLo)) := by
  intro stepCoord₁ stepCoord₂ hEq
  apply Subtype.ext
  exact runtimeMarkedBy_injective (p := p) (segLo := segLo) hp (congrArg Subtype.val hEq)

theorem segmentRuntimeStep_mem_of_mem_coordsOfBoundedSteps
    {lo limit p segLo : ℕ} (hp : 0 < p)
    (steps : List (SegmentRuntimeStep lo limit p segLo))
    {stepCoord : SegmentRuntimeStep lo limit p segLo}
    (hCoord :
      segmentRuntimeCoordOfBoundedStep stepCoord ∈ segmentRuntimeCoordsOfBoundedSteps steps) :
    stepCoord ∈ steps := by
  exact
    (list_mem_map_iff_of_injective
      (f := segmentRuntimeCoordOfBoundedStep)
      (hInj := segmentRuntimeCoordOfBoundedStep_injective
        (lo := lo) (limit := limit) (p := p) (segLo := segLo) hp)
      (xs := steps)
      (x := stepCoord)).mp (by
        simpa [segmentRuntimeCoordsOfBoundedSteps] using hCoord)

@[simp] theorem segmentRuntimeCoordOfBoundedStep_mem_coordsOfBoundedSteps_iff
    {lo limit p segLo : ℕ} (hp : 0 < p)
    (steps : List (SegmentRuntimeStep lo limit p segLo))
    {stepCoord : SegmentRuntimeStep lo limit p segLo} :
    segmentRuntimeCoordOfBoundedStep stepCoord ∈
        segmentRuntimeCoordsOfBoundedSteps steps ↔
      stepCoord ∈ steps := by
  constructor
  · exact segmentRuntimeStep_mem_of_mem_coordsOfBoundedSteps
      (lo := lo) (limit := limit) (p := p) (segLo := segLo) hp steps
  · exact segmentRuntimeCoordOfBoundedStep_mem_coordsOfBoundedSteps
      (steps := steps)

theorem segmentRuntimeCoordOfBoundedStep_mem_toCoordPlan
    {lo limit p segLo : ℕ}
    (plan : SegmentRuntimeStepPlan lo limit p segLo)
    {stepCoord : SegmentRuntimeStep lo limit p segLo} (hStep : stepCoord ∈ plan.2) :
    segmentRuntimeCoordOfBoundedStep stepCoord ∈ plan.toCoordPlan.2 := by
  simpa [SegmentRuntimeStepPlan.toCoordPlan] using
    (CoordinatePlan.mem_map_snd
      (f := segmentRuntimeCoordOfBoundedStep)
      (plan := plan) (coord := stepCoord) hStep)

theorem segmentRuntimeStep_mem_of_mem_toCoordPlan
    {lo limit p segLo : ℕ} (hp : 0 < p)
    (plan : SegmentRuntimeStepPlan lo limit p segLo)
    {stepCoord : SegmentRuntimeStep lo limit p segLo}
    (hCoord : segmentRuntimeCoordOfBoundedStep stepCoord ∈ plan.toCoordPlan.2) :
    stepCoord ∈ plan.2 := by
  exact
    (CoordinatePlan.mem_map_snd_iff_of_injective
      (f := segmentRuntimeCoordOfBoundedStep)
      (hInj := segmentRuntimeCoordOfBoundedStep_injective
        (lo := lo) (limit := limit) (p := p) (segLo := segLo) hp)
      (plan := plan)
      (coord := stepCoord)).mp (by
        simpa [SegmentRuntimeStepPlan.toCoordPlan] using hCoord)

@[simp] theorem segmentRuntimeCoordOfBoundedStep_mem_toCoordPlan_iff
    {lo limit p segLo : ℕ} (hp : 0 < p)
    (plan : SegmentRuntimeStepPlan lo limit p segLo)
    {stepCoord : SegmentRuntimeStep lo limit p segLo} :
    segmentRuntimeCoordOfBoundedStep stepCoord ∈ plan.toCoordPlan.2 ↔
      stepCoord ∈ plan.2 := by
  constructor
  · exact segmentRuntimeStep_mem_of_mem_toCoordPlan
      (lo := lo) (limit := limit) (p := p) (segLo := segLo) hp plan
  · exact segmentRuntimeCoordOfBoundedStep_mem_toCoordPlan
      (plan := plan)

theorem segmentRuntimeStepRead_of_mem_mappedCoords_byByte
    {lo limit p segLo : ℕ}
    (steps : List (SegmentRuntimeStep lo limit p segLo)) (bytes : SegmentByteState)
    {stepCoord : SegmentRuntimeStep lo limit p segLo} (hStep : stepCoord ∈ steps) :
    segmentRuntimeStepRead
        (segmentRuntimeWriteByByte bytes (segmentRuntimeCoordsOfBoundedSteps steps))
        stepCoord = 1 := by
  simpa [segmentRuntimeWriteByByte, segmentRuntimeCoordsOfBoundedSteps] using
    (read_of_mem_coordinatePlansByByte_mapped_of_mark_eq
      segmentRuntimeStepMark
      segmentRuntimeMark
      (fun bytes stepCoord => segmentRuntimeStepRead bytes stepCoord)
      segmentRuntimeCoordOfBoundedStep
      (fun bytes stepCoord => segmentRuntimeStepRead_eq_byteMarkRead bytes stepCoord)
      (fun stepCoord => by
        simp [segmentRuntimeStepMark, segmentRuntimeCoordOfBoundedStep])
      bytes steps hStep)

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

theorem segmentRuntimeRead_first_of_boundedStepPair_byByte
    (bytes : SegmentByteState) {lo limit p segLo step₁ step₂ : ℕ}
    (hLo₁ : lo ≤ runtimeMarkedBy p segLo step₁)
    (hHi₁ : runtimeMarkedBy p segLo step₁ ≤ rawSegmentHi lo limit)
    (hLo₂ : lo ≤ runtimeMarkedBy p segLo step₂)
    (hHi₂ : runtimeMarkedBy p segLo step₂ ≤ rawSegmentHi lo limit) :
    segmentByteRead
        (segmentRuntimeStepWriteByByte bytes
          (segmentRuntimeBoundedStepPair hLo₁ hHi₁ hLo₂ hHi₂))
        hLo₁ hHi₁ = 1 := by
  simpa [segmentRuntimeBoundedStepPair] using
    (segmentRuntimeRead_of_mem_boundedSteps_byByte
      (steps := segmentRuntimeBoundedStepPair hLo₁ hHi₁ hLo₂ hHi₂)
      (bytes := bytes)
      (stepCoord := (⟨step₁, ⟨hLo₁, hHi₁⟩⟩ : SegmentRuntimeStep lo limit p segLo))
      (by simp [segmentRuntimeBoundedStepPair]))

theorem segmentRuntimeRead_second_of_boundedStepPair_byByte
    (bytes : SegmentByteState) {lo limit p segLo step₁ step₂ : ℕ}
    (hLo₁ : lo ≤ runtimeMarkedBy p segLo step₁)
    (hHi₁ : runtimeMarkedBy p segLo step₁ ≤ rawSegmentHi lo limit)
    (hLo₂ : lo ≤ runtimeMarkedBy p segLo step₂)
    (hHi₂ : runtimeMarkedBy p segLo step₂ ≤ rawSegmentHi lo limit) :
    segmentByteRead
        (segmentRuntimeStepWriteByByte bytes
          (segmentRuntimeBoundedStepPair hLo₁ hHi₁ hLo₂ hHi₂))
        hLo₂ hHi₂ = 1 := by
  simpa [segmentRuntimeBoundedStepPair] using
    (segmentRuntimeRead_of_mem_boundedSteps_byByte
      (steps := segmentRuntimeBoundedStepPair hLo₁ hHi₁ hLo₂ hHi₂)
      (bytes := bytes)
      (stepCoord := (⟨step₂, ⟨hLo₂, hHi₂⟩⟩ : SegmentRuntimeStep lo limit p segLo))
      (by simp [segmentRuntimeBoundedStepPair]))

theorem segmentRuntimeReads_of_boundedStepPair_byByte
    (bytes : SegmentByteState) {lo limit p segLo step₁ step₂ : ℕ}
    (hLo₁ : lo ≤ runtimeMarkedBy p segLo step₁)
    (hHi₁ : runtimeMarkedBy p segLo step₁ ≤ rawSegmentHi lo limit)
    (hLo₂ : lo ≤ runtimeMarkedBy p segLo step₂)
    (hHi₂ : runtimeMarkedBy p segLo step₂ ≤ rawSegmentHi lo limit) :
    segmentByteRead
        (segmentRuntimeStepWriteByByte bytes
          (segmentRuntimeBoundedStepPair hLo₁ hHi₁ hLo₂ hHi₂))
        hLo₁ hHi₁ = 1 ∧
      segmentByteRead
        (segmentRuntimeStepWriteByByte bytes
          (segmentRuntimeBoundedStepPair hLo₁ hHi₁ hLo₂ hHi₂))
        hLo₂ hHi₂ = 1 := by
  constructor
  · exact segmentRuntimeRead_first_of_boundedStepPair_byByte
      (bytes := bytes) hLo₁ hHi₁ hLo₂ hHi₂
  · exact segmentRuntimeRead_second_of_boundedStepPair_byByte
      (bytes := bytes) hLo₁ hHi₁ hLo₂ hHi₂

theorem segmentRuntimeRead_of_mem_boundedSteps_mappedCoords_byByte
    {lo limit p segLo : ℕ}
    (steps : List (SegmentRuntimeStep lo limit p segLo)) (bytes : SegmentByteState)
    {stepCoord : SegmentRuntimeStep lo limit p segLo} (hStep : stepCoord ∈ steps) :
    segmentByteRead
        (segmentRuntimeWriteByByte bytes (segmentRuntimeCoordsOfBoundedSteps steps))
        stepCoord.2.1 stepCoord.2.2 = 1 := by
  simpa [segmentRuntimeStepRead] using
    (segmentRuntimeStepRead_of_mem_mappedCoords_byByte
      (steps := steps) (bytes := bytes) hStep)

theorem segmentRuntimeStepWriteByByte_eq_mappedCoordsWriteByByte
    {lo limit p segLo : ℕ}
    (bytes : SegmentByteState) (steps : List (SegmentRuntimeStep lo limit p segLo)) :
    segmentRuntimeStepWriteByByte bytes steps =
      segmentRuntimeWriteByByte bytes (segmentRuntimeCoordsOfBoundedSteps steps) := by
  simpa [segmentRuntimeStepWriteByByte, segmentRuntimeWriteByByte,
    segmentRuntimeCoordsOfBoundedSteps] using
    (coordinatePlanWriteMany_coordinatePlansByByte_eq_of_mark_eq
      segmentRuntimeStepMark
      segmentRuntimeMark
      segmentRuntimeCoordOfBoundedStep
      (fun stepCoord => by
        simp [segmentRuntimeStepMark, segmentRuntimeCoordOfBoundedStep])
      bytes steps)

theorem segmentRuntimeStepRead_stepWriteByByte_eq_mappedCoordsWriteByByte
    {lo limit p segLo : ℕ}
    (steps : List (SegmentRuntimeStep lo limit p segLo)) (bytes : SegmentByteState)
    (stepCoord : SegmentRuntimeStep lo limit p segLo) :
    segmentRuntimeStepRead (segmentRuntimeStepWriteByByte bytes steps) stepCoord =
      segmentRuntimeStepRead
        (segmentRuntimeWriteByByte bytes (segmentRuntimeCoordsOfBoundedSteps steps))
        stepCoord := by
  rw [segmentRuntimeStepWriteByByte_eq_mappedCoordsWriteByByte]

theorem segmentRuntimeCoordOfBoundedStep_le_runtimeSegmentHi
    {lo limit p segLo : ℕ}
    (hpOdd : Odd p)
    (stepCoord : SegmentRuntimeStep lo limit p segLo) :
    (segmentRuntimeCoordOfBoundedStep stepCoord).1 ≤ runtimeSegmentHi lo limit := by
  exact runtimeMarkedBy_le_runtimeSegmentHi_of_le_rawSegmentHi hpOdd stepCoord.2.2

theorem exists_index_of_segmentRuntimeCoordOfBoundedStep
    {lo limit p segLo : ℕ}
    (hLoLe : lo ≤ limit) (hLoPos : 0 < lo) (hLoOdd : Odd lo) (hpOdd : Odd p)
    (stepCoord : SegmentRuntimeStep lo limit p segLo) :
    ∃ idx, idx ≤ oddSegmentIndex lo (runtimeSegmentHi lo limit) ∧
      oddSegmentNumber lo idx = (segmentRuntimeCoordOfBoundedStep stepCoord).1 := by
  simpa [segmentRuntimeCoordOfBoundedStep] using
    (exists_index_of_runtimeMarkedBy_of_le_rawSegmentHi
      (lo := lo) (limit := limit) (p := p) (segLo := segLo) (step := stepCoord.1)
      hLoLe hLoPos hLoOdd hpOdd stepCoord.2.1 stepCoord.2.2)

theorem segmentRuntimeCoordOfBoundedStep_index_le_runtimeSegmentHi
    {lo limit p segLo : ℕ}
    (hLoLe : lo ≤ limit) (hLoPos : 0 < lo) (hLoOdd : Odd lo) (hpOdd : Odd p)
    (stepCoord : SegmentRuntimeStep lo limit p segLo) :
    oddSegmentIndex lo (segmentRuntimeCoordOfBoundedStep stepCoord).1 ≤
      oddSegmentIndex lo (runtimeSegmentHi lo limit) := by
  simpa [segmentRuntimeCoordOfBoundedStep] using
    (runtimeMarkedBy_index_le_runtimeSegmentHi_of_le_rawSegmentHi
      (lo := lo) (limit := limit) (p := p) (segLo := segLo) (step := stepCoord.1)
      hLoLe hLoPos hLoOdd hpOdd stepCoord.2.1 stepCoord.2.2)

theorem segmentRuntimeCoordOfBoundedStep_byte_lt_segBytes
    {lo limit p segLo : ℕ}
    (hpOdd : Odd p)
    (stepCoord : SegmentRuntimeStep lo limit p segLo) :
    segmentByteIndex lo (segmentRuntimeCoordOfBoundedStep stepCoord).1 < segBytes := by
  exact segmentByteIndex_lt_segBytes_of_le_runtimeSegmentHi stepCoord.2.1
    (segmentRuntimeCoordOfBoundedStep_le_runtimeSegmentHi hpOdd stepCoord)

theorem segmentRuntimeBoundedOffset_byte_separated_of_eight_le_mul
    {lo p segLo step k : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hk : 8 ≤ k * p) :
    segmentByteIndex lo (runtimeMarkedBy p segLo step) ≠
      segmentByteIndex lo (runtimeMarkedBy p segLo (step + k)) := by
  exact segmentByteIndex_runtimeMarkedBy_add_ne_of_eight_le_mul
    (step := step) (k := k) hLo hLoOdd hpOdd hk

theorem segmentRuntimeRead_step_of_boundedOffsetPair_of_eight_le_mul
    (bytes : SegmentByteState) {lo limit p segLo step k : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoOffset : lo ≤ runtimeMarkedBy p segLo (step + k))
    (hHiOffset : runtimeMarkedBy p segLo (step + k) ≤ rawSegmentHi lo limit)
    (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hk : 8 ≤ k * p) :
    segmentByteRead
        (segmentRuntimeStepWriteMany bytes
          (segmentRuntimeBoundedStepPairPlans hLo hHi hLoOffset hHiOffset))
        hLo hHi = 1 := by
  exact segmentRuntimeRead_first_of_boundedStepPair (bytes := bytes)
    (hLo₁ := hLo) (hHi₁ := hHi) (hLo₂ := hLoOffset) (hHi₂ := hHiOffset)
    (segmentRuntimeBoundedOffset_byte_separated_of_eight_le_mul
      (step := step) (k := k) hLo hLoOdd hpOdd hk)

theorem segmentRuntimeRead_offset_of_boundedOffsetPair_of_eight_le_mul
    (bytes : SegmentByteState) {lo limit p segLo step k : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoOffset : lo ≤ runtimeMarkedBy p segLo (step + k))
    (hHiOffset : runtimeMarkedBy p segLo (step + k) ≤ rawSegmentHi lo limit)
    (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hk : 8 ≤ k * p) :
    segmentByteRead
        (segmentRuntimeStepWriteMany bytes
          (segmentRuntimeBoundedStepPairPlans hLo hHi hLoOffset hHiOffset))
        hLoOffset hHiOffset = 1 := by
  exact segmentRuntimeRead_second_of_boundedStepPair (bytes := bytes)
    (hLo₁ := hLo) (hHi₁ := hHi) (hLo₂ := hLoOffset) (hHi₂ := hHiOffset)
    (segmentRuntimeBoundedOffset_byte_separated_of_eight_le_mul
      (step := step) (k := k) hLo hLoOdd hpOdd hk)

theorem segmentRuntimeReads_of_boundedOffsetPair_of_eight_le_mul
    (bytes : SegmentByteState) {lo limit p segLo step k : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoOffset : lo ≤ runtimeMarkedBy p segLo (step + k))
    (hHiOffset : runtimeMarkedBy p segLo (step + k) ≤ rawSegmentHi lo limit)
    (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hk : 8 ≤ k * p) :
    segmentByteRead
        (segmentRuntimeStepWriteMany bytes
          (segmentRuntimeBoundedStepPairPlans hLo hHi hLoOffset hHiOffset))
        hLo hHi = 1 ∧
      segmentByteRead
        (segmentRuntimeStepWriteMany bytes
          (segmentRuntimeBoundedStepPairPlans hLo hHi hLoOffset hHiOffset))
        hLoOffset hHiOffset = 1 := by
  constructor
  · exact segmentRuntimeRead_step_of_boundedOffsetPair_of_eight_le_mul
      (bytes := bytes) hLo hHi hLoOffset hHiOffset hLoOdd hpOdd hk
  · exact segmentRuntimeRead_offset_of_boundedOffsetPair_of_eight_le_mul
      (bytes := bytes) hLo hHi hLoOffset hHiOffset hLoOdd hpOdd hk

theorem segmentRuntimeRead_step_of_boundedOffsetPair_byByte
    (bytes : SegmentByteState) {lo limit p segLo step k : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoOffset : lo ≤ runtimeMarkedBy p segLo (step + k))
    (hHiOffset : runtimeMarkedBy p segLo (step + k) ≤ rawSegmentHi lo limit) :
    segmentByteRead
        (segmentRuntimeStepWriteByByte bytes
          (segmentRuntimeBoundedStepPair hLo hHi hLoOffset hHiOffset))
        hLo hHi = 1 := by
  exact segmentRuntimeRead_first_of_boundedStepPair_byByte
    (bytes := bytes) hLo hHi hLoOffset hHiOffset

theorem segmentRuntimeRead_offset_of_boundedOffsetPair_byByte
    (bytes : SegmentByteState) {lo limit p segLo step k : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoOffset : lo ≤ runtimeMarkedBy p segLo (step + k))
    (hHiOffset : runtimeMarkedBy p segLo (step + k) ≤ rawSegmentHi lo limit) :
    segmentByteRead
        (segmentRuntimeStepWriteByByte bytes
          (segmentRuntimeBoundedStepPair hLo hHi hLoOffset hHiOffset))
        hLoOffset hHiOffset = 1 := by
  exact segmentRuntimeRead_second_of_boundedStepPair_byByte
    (bytes := bytes) hLo hHi hLoOffset hHiOffset

theorem segmentRuntimeReads_of_boundedOffsetPair_byByte
    (bytes : SegmentByteState) {lo limit p segLo step k : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoOffset : lo ≤ runtimeMarkedBy p segLo (step + k))
    (hHiOffset : runtimeMarkedBy p segLo (step + k) ≤ rawSegmentHi lo limit) :
    segmentByteRead
        (segmentRuntimeStepWriteByByte bytes
          (segmentRuntimeBoundedStepPair hLo hHi hLoOffset hHiOffset))
        hLo hHi = 1 ∧
      segmentByteRead
        (segmentRuntimeStepWriteByByte bytes
          (segmentRuntimeBoundedStepPair hLo hHi hLoOffset hHiOffset))
        hLoOffset hHiOffset = 1 := by
  constructor
  · exact segmentRuntimeRead_step_of_boundedOffsetPair_byByte
      (bytes := bytes) hLo hHi hLoOffset hHiOffset
  · exact segmentRuntimeRead_offset_of_boundedOffsetPair_byByte
      (bytes := bytes) hLo hHi hLoOffset hHiOffset

theorem segmentRuntimeRead_step_of_sequentialBoundedOffsetWrites_of_eight_le_mul
    (bytes : SegmentByteState) {lo limit p segLo step k : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoOffset : lo ≤ runtimeMarkedBy p segLo (step + k))
    (hHiOffset : runtimeMarkedBy p segLo (step + k) ≤ rawSegmentHi lo limit)
    (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hk : 8 ≤ k * p) :
    segmentByteRead
        (segmentByteWrite (segmentByteWrite bytes hLo hHi) hLoOffset hHiOffset)
        hLo hHi = 1 := by
  have hByte :
      segmentByteIndex lo (runtimeMarkedBy p segLo (step + k)) ≠
        segmentByteIndex lo (runtimeMarkedBy p segLo step) := by
    symm
    exact segmentRuntimeBoundedOffset_byte_separated_of_eight_le_mul
      (step := step) (k := k) hLo hLoOdd hpOdd hk
  calc
    segmentByteRead
        (segmentByteWrite (segmentByteWrite bytes hLo hHi) hLoOffset hHiOffset)
        hLo hHi
      = segmentByteRead (segmentByteWrite bytes hLo hHi) hLo hHi := by
          exact segmentByteRead_write_other_byte_eq
            (bytes := segmentByteWrite bytes hLo hHi)
            (hLoN := hLoOffset) (hN := hHiOffset)
            (hLoM := hLo) (hM := hHi) hByte
    _ = 1 := segmentByteRead_written _ hLo hHi

theorem segmentRuntimeRead_offset_of_sequentialBoundedOffsetWrites
    (bytes : SegmentByteState) {lo limit p segLo step k : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoOffset : lo ≤ runtimeMarkedBy p segLo (step + k))
    (hHiOffset : runtimeMarkedBy p segLo (step + k) ≤ rawSegmentHi lo limit) :
    segmentByteRead
        (segmentByteWrite (segmentByteWrite bytes hLo hHi) hLoOffset hHiOffset)
        hLoOffset hHiOffset = 1 := by
  exact segmentByteRead_written (segmentByteWrite bytes hLo hHi) hLoOffset hHiOffset

theorem segmentRuntimeReads_of_sequentialBoundedOffsetWrites_of_eight_le_mul
    (bytes : SegmentByteState) {lo limit p segLo step k : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoOffset : lo ≤ runtimeMarkedBy p segLo (step + k))
    (hHiOffset : runtimeMarkedBy p segLo (step + k) ≤ rawSegmentHi lo limit)
    (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hk : 8 ≤ k * p) :
    segmentByteRead
        (segmentByteWrite (segmentByteWrite bytes hLo hHi) hLoOffset hHiOffset)
        hLo hHi = 1 ∧
      segmentByteRead
        (segmentByteWrite (segmentByteWrite bytes hLo hHi) hLoOffset hHiOffset)
        hLoOffset hHiOffset = 1 := by
  constructor
  · exact segmentRuntimeRead_step_of_sequentialBoundedOffsetWrites_of_eight_le_mul
      (bytes := bytes) hLo hHi hLoOffset hHiOffset hLoOdd hpOdd hk
  · exact segmentRuntimeRead_offset_of_sequentialBoundedOffsetWrites
      (bytes := bytes) hLo hHi hLoOffset hHiOffset

theorem segmentRuntimeConsecutiveBoundedSteps_byte_separated_of_eight_le_p
    {lo p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hP : 8 ≤ p) :
    segmentByteIndex lo (runtimeMarkedBy p segLo step) ≠
      segmentByteIndex lo (runtimeMarkedBy p segLo (step + 1)) := by
  simpa using
    (segmentRuntimeBoundedOffset_byte_separated_of_eight_le_mul
      (step := step) (k := 1) hLo hLoOdd hpOdd (by simpa using hP))

theorem segmentRuntimeRead_step_of_consecutiveBoundedSteps_of_eight_le_p
    (bytes : SegmentByteState) {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit)
    (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hP : 8 ≤ p) :
    segmentByteRead
        (segmentRuntimeStepWriteMany bytes
          (segmentRuntimeConsecutiveBoundedStepPlans hLo hHi hLoSucc hHiSucc))
        hLo hHi = 1 := by
  simpa [segmentRuntimeConsecutiveBoundedStepPlans] using
    (segmentRuntimeRead_step_of_boundedOffsetPair_of_eight_le_mul
      (bytes := bytes) (hLo := hLo) (hHi := hHi)
      (hLoOffset := hLoSucc) (hHiOffset := hHiSucc)
      (hLoOdd := hLoOdd) (hpOdd := hpOdd) (step := step) (k := 1)
      (by simpa using hP))

theorem segmentRuntimeRead_succ_of_consecutiveBoundedSteps_of_eight_le_p
    (bytes : SegmentByteState) {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit)
    (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hP : 8 ≤ p) :
    segmentByteRead
        (segmentRuntimeStepWriteMany bytes
          (segmentRuntimeConsecutiveBoundedStepPlans hLo hHi hLoSucc hHiSucc))
        hLoSucc hHiSucc = 1 := by
  simpa [segmentRuntimeConsecutiveBoundedStepPlans] using
    (segmentRuntimeRead_offset_of_boundedOffsetPair_of_eight_le_mul
      (bytes := bytes) (hLo := hLo) (hHi := hHi)
      (hLoOffset := hLoSucc) (hHiOffset := hHiSucc)
      (hLoOdd := hLoOdd) (hpOdd := hpOdd) (step := step) (k := 1)
      (by simpa using hP))

theorem segmentRuntimeReads_of_consecutiveBoundedSteps_of_eight_le_p
    (bytes : SegmentByteState) {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit)
    (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hP : 8 ≤ p) :
    segmentByteRead
        (segmentRuntimeStepWriteMany bytes
          (segmentRuntimeConsecutiveBoundedStepPlans hLo hHi hLoSucc hHiSucc))
        hLo hHi = 1 ∧
      segmentByteRead
        (segmentRuntimeStepWriteMany bytes
          (segmentRuntimeConsecutiveBoundedStepPlans hLo hHi hLoSucc hHiSucc))
        hLoSucc hHiSucc = 1 := by
  constructor
  · exact segmentRuntimeRead_step_of_consecutiveBoundedSteps_of_eight_le_p
      (bytes := bytes) hLo hHi hLoSucc hHiSucc hLoOdd hpOdd hP
  · exact segmentRuntimeRead_succ_of_consecutiveBoundedSteps_of_eight_le_p
      (bytes := bytes) hLo hHi hLoSucc hHiSucc hLoOdd hpOdd hP

theorem segmentRuntimeRead_step_of_sequentialConsecutiveBoundedWrites_of_eight_le_p
    (bytes : SegmentByteState) {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit)
    (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hP : 8 ≤ p) :
    segmentByteRead
        (segmentByteWrite (segmentByteWrite bytes hLo hHi) hLoSucc hHiSucc)
        hLo hHi = 1 := by
  simpa using
    (segmentRuntimeRead_step_of_sequentialBoundedOffsetWrites_of_eight_le_mul
      (bytes := bytes) (hLo := hLo) (hHi := hHi)
      (hLoOffset := hLoSucc) (hHiOffset := hHiSucc)
      (hLoOdd := hLoOdd) (hpOdd := hpOdd) (step := step) (k := 1)
      (by simpa using hP))

theorem segmentRuntimeRead_succ_of_sequentialConsecutiveBoundedWrites
    (bytes : SegmentByteState) {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit) :
    segmentByteRead
        (segmentByteWrite (segmentByteWrite bytes hLo hHi) hLoSucc hHiSucc)
        hLoSucc hHiSucc = 1 := by
  simpa using
    (segmentRuntimeRead_offset_of_sequentialBoundedOffsetWrites
      (bytes := bytes) (hLo := hLo) (hHi := hHi)
      (hLoOffset := hLoSucc) (hHiOffset := hHiSucc)
      (step := step) (k := 1))

theorem segmentRuntimeReads_of_sequentialConsecutiveBoundedWrites_of_eight_le_p
    (bytes : SegmentByteState) {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit)
    (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hP : 8 ≤ p) :
    segmentByteRead
        (segmentByteWrite (segmentByteWrite bytes hLo hHi) hLoSucc hHiSucc)
        hLo hHi = 1 ∧
      segmentByteRead
        (segmentByteWrite (segmentByteWrite bytes hLo hHi) hLoSucc hHiSucc)
        hLoSucc hHiSucc = 1 := by
  constructor
  · exact segmentRuntimeRead_step_of_sequentialConsecutiveBoundedWrites_of_eight_le_p
      (bytes := bytes) hLo hHi hLoSucc hHiSucc hLoOdd hpOdd hP
  · exact segmentRuntimeRead_succ_of_sequentialConsecutiveBoundedWrites
      (bytes := bytes) hLo hHi hLoSucc hHiSucc

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

theorem segmentRuntimeReads_of_consecutiveBoundedSteps_byByte
    (bytes : SegmentByteState) {lo limit p segLo step : ℕ}
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeMarkedBy p segLo (step + 1))
    (hHiSucc : runtimeMarkedBy p segLo (step + 1) ≤ rawSegmentHi lo limit) :
    segmentByteRead
        (segmentRuntimeStepWriteByByte bytes
          (segmentRuntimeConsecutiveBoundedSteps hLo hHi hLoSucc hHiSucc))
        hLo hHi = 1 ∧
      segmentByteRead
        (segmentRuntimeStepWriteByByte bytes
          (segmentRuntimeConsecutiveBoundedSteps hLo hHi hLoSucc hHiSucc))
        hLoSucc hHiSucc = 1 := by
  constructor
  · exact segmentRuntimeRead_step_of_consecutiveBoundedSteps_byByte
      (bytes := bytes) hLo hHi hLoSucc hHiSucc
  · exact segmentRuntimeRead_succ_of_consecutiveBoundedSteps_byByte
      (bytes := bytes) hLo hHi hLoSucc hHiSucc

theorem segmentRuntimeReads_initialConsecutiveBoundedSteps_of_eight_le_p
    (bytes : SegmentByteState) {lo limit p segLo : ℕ}
    (hLo : lo ≤ runtimeCrossOffStart p segLo)
    (hHi : runtimeCrossOffStart p segLo ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeCrossOffStart p segLo + 2 * p)
    (hHiSucc : runtimeCrossOffStart p segLo + 2 * p ≤ rawSegmentHi lo limit)
    (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hP : 8 ≤ p) :
    segmentByteRead
        (segmentRuntimeStepWriteMany bytes
          (segmentRuntimeInitialConsecutiveBoundedStepPlans hLo hHi hLoSucc hHiSucc))
        hLo hHi = 1 ∧
      segmentByteRead
        (segmentRuntimeStepWriteMany bytes
          (segmentRuntimeInitialConsecutiveBoundedStepPlans hLo hHi hLoSucc hHiSucc))
        hLoSucc hHiSucc = 1 := by
  simpa [runtimeMarkedBy_zero, runtimeMarkedBy_succ] using
    (segmentRuntimeReads_of_consecutiveBoundedSteps_of_eight_le_p
      (bytes := bytes) (step := 0)
      (hLo := by simpa [runtimeMarkedBy_zero] using hLo)
      (hHi := by simpa [runtimeMarkedBy_zero] using hHi)
      (hLoSucc := by simpa [runtimeMarkedBy_succ, runtimeMarkedBy_zero] using hLoSucc)
      (hHiSucc := by simpa [runtimeMarkedBy_succ, runtimeMarkedBy_zero] using hHiSucc)
      hLoOdd hpOdd hP)

theorem segmentRuntimeReads_initialConsecutiveBoundedSteps_byByte
    (bytes : SegmentByteState) {lo limit p segLo : ℕ}
    (hLo : lo ≤ runtimeCrossOffStart p segLo)
    (hHi : runtimeCrossOffStart p segLo ≤ rawSegmentHi lo limit)
    (hLoSucc : lo ≤ runtimeCrossOffStart p segLo + 2 * p)
    (hHiSucc : runtimeCrossOffStart p segLo + 2 * p ≤ rawSegmentHi lo limit) :
    segmentByteRead
        (segmentRuntimeStepWriteByByte bytes
          (segmentRuntimeInitialConsecutiveBoundedSteps hLo hHi hLoSucc hHiSucc))
        hLo hHi = 1 ∧
      segmentByteRead
        (segmentRuntimeStepWriteByByte bytes
          (segmentRuntimeInitialConsecutiveBoundedSteps hLo hHi hLoSucc hHiSucc))
        hLoSucc hHiSucc = 1 := by
  simpa [runtimeMarkedBy_zero, runtimeMarkedBy_succ] using
    (segmentRuntimeReads_of_consecutiveBoundedSteps_byByte
      (bytes := bytes) (step := 0)
      (hLo := by simpa [runtimeMarkedBy_zero] using hLo)
      (hHi := by simpa [runtimeMarkedBy_zero] using hHi)
      (hLoSucc := by simpa [runtimeMarkedBy_succ, runtimeMarkedBy_zero] using hLoSucc)
      (hHiSucc := by simpa [runtimeMarkedBy_succ, runtimeMarkedBy_zero] using hHiSucc))

end PrimeArithmetic.Sieve
