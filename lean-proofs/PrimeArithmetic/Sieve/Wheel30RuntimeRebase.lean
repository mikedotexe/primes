import Mathlib
import PrimeArithmetic.Sieve.BoundedByteCoordinates
import PrimeArithmetic.Sieve.Wheel30Indexing
import PrimeArithmetic.Sieve.Wheel30RuntimePlans

namespace PrimeArithmetic.Sieve

/-!
Base-invariant transport for grouped wheel30 runtime families.

`Wheel30RuntimePlans.lean` already proves that wheel30 grouped writes depend
only on `(cycle, slot)` data, not on the phantom `base` parameter. This module
packages the two corollaries that later executable-agreement arguments usually
need:

- rebasing a grouped runtime-coordinate family does not change readback
- rebasing an already-grouped runtime-plan family does not change the final
  byte write or its distinct-plan readback route
- membership witnesses transport in both directions between the original and
  rebased families

The goal is to keep later proofs from rebuilding `List.mem_map` witnesses or
repeating the same base-invariance argument by hand.
-/

/-- Reinterpret one grouped wheel30 runtime plan at a different phantom base. -/
def Wheel30RuntimePlan.rebase {base₁ base₂ : ℕ}
    (plan : Wheel30RuntimePlan base₁) : Wheel30RuntimePlan base₂ :=
  CoordinatePlan.map (fun coord => coord.rebase (base₂ := base₂)) plan

@[simp] theorem Wheel30RuntimePlan.rebase_fst {base₁ base₂ : ℕ}
    (plan : Wheel30RuntimePlan base₁) :
    (plan.rebase (base₂ := base₂)).1 = plan.1 := by
  simp [Wheel30RuntimePlan.rebase]

@[simp] theorem Wheel30RuntimePlan.rebase_snd {base₁ base₂ : ℕ}
    (plan : Wheel30RuntimePlan base₁) :
    (plan.rebase (base₂ := base₂)).2 =
      plan.2.map fun coord => coord.rebase (base₂ := base₂) := by
  simp [Wheel30RuntimePlan.rebase]

@[simp] theorem Wheel30RuntimePlan.rebase_id {base : ℕ}
    (plan : Wheel30RuntimePlan base) :
    plan.rebase (base₂ := base) = plan := by
  cases plan with
  | mk slot coords =>
      simp [Wheel30RuntimePlan.rebase, CoordinatePlan.map]

@[simp] theorem Wheel30RuntimePlan.rebase_rebase {base₁ base₂ base₃ : ℕ}
    (plan : Wheel30RuntimePlan base₁) :
    (plan.rebase (base₂ := base₂)).rebase (base₂ := base₃) =
      plan.rebase (base₂ := base₃) := by
  cases plan with
  | mk slot coords =>
      simp [Wheel30RuntimePlan.rebase, CoordinatePlan.map, List.map_map]

@[simp] theorem Wheel30RuntimeCoord.rebase_roundtrip {base₁ base₂ : ℕ}
    (coord : Wheel30RuntimeCoord base₁) :
    (coord.rebase (base₂ := base₂)).rebase (base₂ := base₁) = coord := by
  calc
    (coord.rebase (base₂ := base₂)).rebase (base₂ := base₁)
        = coord.rebase (base₂ := base₁) :=
          Wheel30RuntimeCoord.rebase_rebase
            (base₂ := base₂) (base₃ := base₁) coord
    _ = coord := by simp

theorem Wheel30RuntimeCoord.rebase_leftInverse {base₁ base₂ : ℕ} :
    Function.LeftInverse
      (fun coord : Wheel30RuntimeCoord base₂ => coord.rebase (base₂ := base₁))
      (fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂)) := by
  intro coord
  exact Wheel30RuntimeCoord.rebase_roundtrip (base₂ := base₂) coord

theorem Wheel30RuntimeCoord.rebase_injective {base₁ base₂ : ℕ} :
    Function.Injective
      (fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂)) := by
  exact (Wheel30RuntimeCoord.rebase_leftInverse (base₁ := base₁) (base₂ := base₂)).injective

theorem Wheel30RuntimeCoord.rebase_surjective {base₁ base₂ : ℕ} :
    Function.Surjective
      (fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂)) := by
  intro coord
  exact ⟨coord.rebase (base₂ := base₁), by simp⟩

theorem Wheel30RuntimeCoord.rebase_bijective {base₁ base₂ : ℕ} :
    Function.Bijective
      (fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂)) := by
  exact ⟨
    Wheel30RuntimeCoord.rebase_injective (base₁ := base₁) (base₂ := base₂),
    Wheel30RuntimeCoord.rebase_surjective (base₁ := base₁) (base₂ := base₂)
  ⟩

@[simp] theorem Wheel30RuntimeCoord.rebase_eq_iff {base₁ base₂ : ℕ}
    {coord₁ : Wheel30RuntimeCoord base₁} {coord₂ : Wheel30RuntimeCoord base₂} :
    coord₁.rebase (base₂ := base₂) = coord₂ ↔
      coord₁ = coord₂.rebase (base₂ := base₁) := by
  cases coord₁
  cases coord₂
  simp [Wheel30RuntimeCoord.rebase]

@[simp] theorem Wheel30RuntimePlan.rebase_roundtrip {base₁ base₂ : ℕ}
    (plan : Wheel30RuntimePlan base₁) :
    (plan.rebase (base₂ := base₂)).rebase (base₂ := base₁) = plan := by
  calc
    (plan.rebase (base₂ := base₂)).rebase (base₂ := base₁)
        = plan.rebase (base₂ := base₁) :=
          Wheel30RuntimePlan.rebase_rebase
            (base₂ := base₂) (base₃ := base₁) plan
    _ = plan := by simp

theorem Wheel30RuntimePlan.rebase_leftInverse {base₁ base₂ : ℕ} :
    Function.LeftInverse
      (fun plan : Wheel30RuntimePlan base₂ => plan.rebase (base₂ := base₁))
      (fun plan : Wheel30RuntimePlan base₁ => plan.rebase (base₂ := base₂)) := by
  intro plan
  exact Wheel30RuntimePlan.rebase_roundtrip (base₂ := base₂) plan

theorem Wheel30RuntimePlan.rebase_injective {base₁ base₂ : ℕ} :
    Function.Injective
      (fun plan : Wheel30RuntimePlan base₁ => plan.rebase (base₂ := base₂)) := by
  exact CoordinatePlan.map_injective
    (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
    (g := fun coord : Wheel30RuntimeCoord base₂ => coord.rebase (base₂ := base₁))
    (hLeft := Wheel30RuntimeCoord.rebase_leftInverse (base₁ := base₁) (base₂ := base₂))

theorem Wheel30RuntimePlan.rebase_surjective {base₁ base₂ : ℕ} :
    Function.Surjective
      (fun plan : Wheel30RuntimePlan base₁ => plan.rebase (base₂ := base₂)) := by
  exact CoordinatePlan.map_surjective
    (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
    (g := fun coord : Wheel30RuntimeCoord base₂ => coord.rebase (base₂ := base₁))
    (hRight := Wheel30RuntimeCoord.rebase_leftInverse (base₁ := base₂) (base₂ := base₁))

theorem Wheel30RuntimePlan.rebase_bijective {base₁ base₂ : ℕ} :
    Function.Bijective
      (fun plan : Wheel30RuntimePlan base₁ => plan.rebase (base₂ := base₂)) := by
  exact CoordinatePlan.map_bijective
    (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
    (g := fun coord : Wheel30RuntimeCoord base₂ => coord.rebase (base₂ := base₁))
    (hLeft := Wheel30RuntimeCoord.rebase_leftInverse (base₁ := base₁) (base₂ := base₂))
    (hRight := Wheel30RuntimeCoord.rebase_leftInverse (base₁ := base₂) (base₂ := base₁))

@[simp] theorem Wheel30RuntimePlan.rebase_eq_iff {base₁ base₂ : ℕ}
    {plan₁ : Wheel30RuntimePlan base₁} {plan₂ : Wheel30RuntimePlan base₂} :
    plan₁.rebase (base₂ := base₂) = plan₂ ↔
      plan₁ = plan₂.rebase (base₂ := base₁) := by
  simpa [Wheel30RuntimePlan.rebase] using
    (CoordinatePlan.map_eq_iff_of_inverse
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (g := fun coord : Wheel30RuntimeCoord base₂ => coord.rebase (base₂ := base₁))
      (hLeft := Wheel30RuntimeCoord.rebase_leftInverse (base₁ := base₁) (base₂ := base₂))
      (hRight := Wheel30RuntimeCoord.rebase_leftInverse (base₁ := base₂) (base₂ := base₁))
      (plan₁ := plan₁) (plan₂ := plan₂))

theorem wheel30CandidateRead_eq_one_of_base_invariant
    (bytes : Wheel30ByteState)
    {base₁ base₂ cycle : ℕ} (slot : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes)
    (hRead : wheel30CandidateRead bytes base₁ cycle slot hCycle = 1) :
    wheel30CandidateRead bytes base₂ cycle slot hCycle = 1 := by
  rw [wheel30CandidateRead_base_invariant
      (bytes := bytes) (base₁ := base₂) (base₂ := base₁)
      (cycle := cycle) (slot := slot) (hCycle := hCycle)]
  exact hRead

theorem wheel30RuntimeRead_eq_one_of_base_invariant
    (bytes : Wheel30ByteState)
    {coordBase readBase₁ readBase₂ : ℕ}
    (coord : Wheel30RuntimeCoord coordBase)
    (hRead :
      wheel30CandidateRead bytes readBase₁ coord.cycle coord.slot coord.hCycle = 1) :
    wheel30CandidateRead bytes readBase₂ coord.cycle coord.slot coord.hCycle = 1 := by
  exact wheel30CandidateRead_eq_one_of_base_invariant
    (bytes := bytes) (base₁ := readBase₁) (base₂ := readBase₂)
    (slot := coord.slot) (hCycle := coord.hCycle) hRead

theorem wheel30RuntimeRead_eq_one_of_rebased
    (bytes : Wheel30ByteState)
    {coordBase₁ coordBase₂ readBase₁ readBase₂ : ℕ}
    (coord : Wheel30RuntimeCoord coordBase₂)
    (hRead :
      wheel30CandidateRead bytes readBase₁
        (coord.rebase (base₂ := coordBase₁)).cycle
        (coord.rebase (base₂ := coordBase₁)).slot
        (coord.rebase (base₂ := coordBase₁)).hCycle = 1) :
    wheel30CandidateRead bytes readBase₂ coord.cycle coord.slot coord.hCycle = 1 := by
  simpa [Wheel30RuntimeCoord.rebase] using
    (wheel30RuntimeRead_eq_one_of_base_invariant
      (bytes := bytes) (readBase₁ := readBase₁) (readBase₂ := readBase₂)
      (coord := coord.rebase (base₂ := coordBase₁)) hRead)

theorem wheel30BoundedCycleSlotRead_eq_one_of_base_invariant
    (bytes : Wheel30ByteState)
    {base₁ base₂ : ℕ} (cycleSlot : Wheel30BoundedCycleSlot)
    (hRead : wheel30BoundedCycleSlotRead bytes base₁ cycleSlot = 1) :
    wheel30BoundedCycleSlotRead bytes base₂ cycleSlot = 1 := by
  rw [wheel30BoundedCycleSlotRead_base_invariant
      (bytes := bytes) (base₁ := base₂) (base₂ := base₁)
      (cycleSlot := cycleSlot)]
  exact hRead

theorem wheel30BoundedCycleSlotRead_eq_one_of_runtimeRead
    (bytes : Wheel30ByteState)
    {base₁ base₂ : ℕ} (cycleSlot : Wheel30BoundedCycleSlot)
    (hRead :
      wheel30CandidateRead bytes base₁ cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2 = 1) :
    wheel30BoundedCycleSlotRead bytes base₂ cycleSlot = 1 := by
  rw [wheel30CandidateRead_eq_boundedCycleSlotRead
      (bytes := bytes) (base := base₁) (cycleSlot := cycleSlot)] at hRead
  exact wheel30BoundedCycleSlotRead_eq_one_of_base_invariant
    (bytes := bytes) (base₁ := base₁) (base₂ := base₂)
    (cycleSlot := cycleSlot) hRead

theorem wheel30RuntimeCoord_rebase_mem_map {base₁ base₂ : ℕ}
    (coords : List (Wheel30RuntimeCoord base₁))
    {coord : Wheel30RuntimeCoord base₁} (hCoord : coord ∈ coords) :
    coord.rebase (base₂ := base₂) ∈
      coords.map fun coord => coord.rebase (base₂ := base₂) := by
  exact List.mem_map.mpr ⟨coord, hCoord, rfl⟩

theorem wheel30RuntimeCoord_mem_of_rebased_mem_map {base₁ base₂ : ℕ}
    (coords : List (Wheel30RuntimeCoord base₁))
    {coord : Wheel30RuntimeCoord base₂}
    (hCoord : coord ∈ coords.map fun coord => coord.rebase (base₂ := base₂)) :
    coord.rebase (base₂ := base₁) ∈ coords := by
  exact list_mem_of_mem_map_of_leftInverse
    (fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
    (fun coord : Wheel30RuntimeCoord base₂ => coord.rebase (base₂ := base₁))
    (Wheel30RuntimeCoord.rebase_leftInverse (base₁ := base₁) (base₂ := base₂))
    coords hCoord

@[simp] theorem wheel30RuntimeCoord_rebase_mem_map_iff {base₁ base₂ : ℕ}
    (coords : List (Wheel30RuntimeCoord base₁))
    {coord : Wheel30RuntimeCoord base₁} :
    coord.rebase (base₂ := base₂) ∈
        coords.map (fun coord => coord.rebase (base₂ := base₂)) ↔
      coord ∈ coords := by
  exact
    (list_mem_map_iff_of_leftInverse
      (fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (fun coord : Wheel30RuntimeCoord base₂ => coord.rebase (base₂ := base₁))
      (Wheel30RuntimeCoord.rebase_leftInverse (base₁ := base₁) (base₂ := base₂))
      coords
      (x := coord))

@[simp] theorem wheel30RuntimeCoordsOfCycleSlots_eq_map_rebase
    {base₁ base₂ : ℕ} (cycleSlots : List (ℕ × Fin 8))
    (hCycles : ∀ cycleSlot ∈ cycleSlots, cycleSlot.1 < wheel30SegmentBytes) :
    wheel30RuntimeCoordsOfCycleSlots (base := base₂) cycleSlots hCycles =
      (wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles).map
        (fun coord => coord.rebase (base₂ := base₂)) := by
  unfold wheel30RuntimeCoordsOfCycleSlots
  simp [List.map_map, Wheel30RuntimeCoord.rebase]

theorem wheel30RuntimeCoord_mem_rebasedCoordsOfCycleSlots
    {base₁ : ℕ} (base₂ : ℕ)
    (cycleSlots : List (ℕ × Fin 8))
    (hCycles : ∀ cycleSlot ∈ cycleSlots, cycleSlot.1 < wheel30SegmentBytes)
    {cycle : ℕ} {slot : Fin 8} (hCycleSlot : (cycle, slot) ∈ cycleSlots) :
    (⟨cycle, slot, hCycles (cycle, slot) hCycleSlot⟩ : Wheel30RuntimeCoord base₁).rebase
        (base₂ := base₂) ∈
      wheel30RuntimeCoordsOfCycleSlots (base := base₂) cycleSlots hCycles := by
  rw [wheel30RuntimeCoordsOfCycleSlots_eq_map_rebase
      (base₁ := base₁) (base₂ := base₂)]
  exact wheel30RuntimeCoord_rebase_mem_map
    (coords := wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles)
    (coord := (⟨cycle, slot, hCycles (cycle, slot) hCycleSlot⟩ : Wheel30RuntimeCoord base₁))
    (wheel30RuntimeCoord_mem_coordsOfCycleSlots cycleSlots hCycles hCycleSlot)

theorem wheel30RuntimeCoord_mem_of_mem_rebasedCoordsOfCycleSlots
    {base₁ : ℕ} (base₂ : ℕ)
    (cycleSlots : List (ℕ × Fin 8))
    (hCycles : ∀ cycleSlot ∈ cycleSlots, cycleSlot.1 < wheel30SegmentBytes)
    {coord : Wheel30RuntimeCoord base₂}
    (hCoord : coord ∈ wheel30RuntimeCoordsOfCycleSlots (base := base₂) cycleSlots hCycles) :
    coord.rebase (base₂ := base₁) ∈
      wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles := by
  rw [wheel30RuntimeCoordsOfCycleSlots_eq_map_rebase
      (base₁ := base₁) (base₂ := base₂) (cycleSlots := cycleSlots) (hCycles := hCycles)] at hCoord
  exact wheel30RuntimeCoord_mem_of_rebased_mem_map
    (base₁ := base₁) (base₂ := base₂)
    (coords := wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles) hCoord

@[simp] theorem wheel30RuntimeCoord_rebase_mem_coordsOfCycleSlots_iff
    {base₁ base₂ : ℕ}
    (cycleSlots : List (ℕ × Fin 8))
    (hCycles : ∀ cycleSlot ∈ cycleSlots, cycleSlot.1 < wheel30SegmentBytes)
    {coord : Wheel30RuntimeCoord base₁} :
    coord.rebase (base₂ := base₂) ∈
        wheel30RuntimeCoordsOfCycleSlots (base := base₂) cycleSlots hCycles ↔
      coord ∈ wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles := by
  rw [wheel30RuntimeCoordsOfCycleSlots_eq_map_rebase
      (base₁ := base₁) (base₂ := base₂)]
  exact wheel30RuntimeCoord_rebase_mem_map_iff
    (base₂ := base₂)
    (coords := wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles)

theorem wheel30RuntimeWriteByByte_cycleSlots_base_invariant
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (cycleSlots : List (ℕ × Fin 8))
    (hCycles : ∀ cycleSlot ∈ cycleSlots, cycleSlot.1 < wheel30SegmentBytes) :
    wheel30RuntimeWriteByByte (base := base₁) bytes
        (wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles) =
      wheel30RuntimeWriteByByte (base := base₂) bytes
        (wheel30RuntimeCoordsOfCycleSlots (base := base₂) cycleSlots hCycles) := by
  rw [wheel30RuntimeCoordsOfCycleSlots_eq_map_rebase
      (base₁ := base₁) (base₂ := base₂)]
  exact wheel30RuntimeWriteByByte_eq_rebasedCoordsWriteByByte
    (bytes := bytes) (base₁ := base₁) (base₂ := base₂)
    (coords := wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles)

theorem wheel30RuntimeRead_of_mem_rebasedCycleSlots_byByte
    {base₁ : ℕ} (base₂ : ℕ)
    (cycleSlots : List (ℕ × Fin 8))
    (hCycles : ∀ cycleSlot ∈ cycleSlots, cycleSlot.1 < wheel30SegmentBytes)
    (bytes : Wheel30ByteState)
    {cycle : ℕ} {slot : Fin 8} (hCycleSlot : (cycle, slot) ∈ cycleSlots) :
    wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base₂) bytes
          ((wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles).map
            (fun coord => coord.rebase (base₂ := base₂))))
        base₂ cycle slot (hCycles (cycle, slot) hCycleSlot) = 1 := by
  rw [← wheel30RuntimeCoordsOfCycleSlots_eq_map_rebase
      (base₁ := base₁) (base₂ := base₂) (cycleSlots := cycleSlots) (hCycles := hCycles)]
  simpa using
    (wheel30RuntimeRead_of_mem_groupedCoords
      (base := base₂)
      (coords := wheel30RuntimeCoordsOfCycleSlots (base := base₂) cycleSlots hCycles)
      (bytes := bytes)
      (coord := (⟨cycle, slot, hCycles (cycle, slot) hCycleSlot⟩ : Wheel30RuntimeCoord base₂))
      (hCoord := wheel30RuntimeCoord_mem_coordsOfCycleSlots cycleSlots hCycles hCycleSlot))

theorem wheel30RuntimeRead_of_mem_cycleSlots_byByte_of_rebased
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (cycleSlots : List (ℕ × Fin 8))
    (hCycles : ∀ cycleSlot ∈ cycleSlots, cycleSlot.1 < wheel30SegmentBytes)
    (bytes : Wheel30ByteState)
    {coord : Wheel30RuntimeCoord base₂}
    (hCoord : coord ∈ wheel30RuntimeCoordsOfCycleSlots (base := base₂) cycleSlots hCycles) :
    wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base₁) bytes
          (wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles))
        base₃ coord.cycle coord.slot coord.hCycle = 1 := by
  rw [wheel30CandidateRead_base_invariant
      (bytes := wheel30RuntimeWriteByByte (base := base₁) bytes
        (wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles))
      (base₁ := base₃) (base₂ := base₁) (cycle := coord.cycle)
      (slot := coord.slot) (hCycle := coord.hCycle)]
  have hCoordOrig :
      coord.rebase (base₂ := base₁) ∈
        wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles := by
    exact wheel30RuntimeCoord_mem_of_mem_rebasedCoordsOfCycleSlots
      (base₂ := base₂) (cycleSlots := cycleSlots) (hCycles := hCycles) hCoord
  simpa [Wheel30RuntimeCoord.rebase] using
    (wheel30RuntimeRead_of_mem_groupedCoords
      (base := base₁)
      (coords := wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles)
      (bytes := bytes)
      (coord := coord.rebase (base₂ := base₁)) hCoordOrig)

@[simp] theorem wheel30RuntimeCoordOfBoundedCycleSlot_rebase {base₁ base₂ : ℕ}
    (cycleSlot : Wheel30BoundedCycleSlot) :
    (wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot).rebase (base₂ := base₂) =
      wheel30RuntimeCoordOfBoundedCycleSlot (base := base₂) cycleSlot := by
  rfl

@[simp] theorem Wheel30BoundedCycleSlotPlan.toRuntimePlan_rebase {base₁ base₂ : ℕ}
    (plan : Wheel30BoundedCycleSlotPlan) :
    Wheel30RuntimePlan.rebase (base₂ := base₂)
        (Wheel30BoundedCycleSlotPlan.toRuntimePlan (base := base₁) plan) =
      Wheel30BoundedCycleSlotPlan.toRuntimePlan (base := base₂) plan := by
  rcases plan with ⟨slot, cycleSlots⟩
  change CoordinatePlan.map (fun coord : Wheel30RuntimeCoord base₁ =>
      coord.rebase (base₂ := base₂))
        (CoordinatePlan.map (wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁))
          (slot, cycleSlots)) =
    CoordinatePlan.map (wheel30RuntimeCoordOfBoundedCycleSlot (base := base₂))
      (slot, cycleSlots)
  simp [CoordinatePlan.map, List.map_map]

@[simp] theorem wheel30BoundedCycleSlotPlans_toRuntimePlans_rebase {base₁ base₂ : ℕ}
    (plans : List Wheel30BoundedCycleSlotPlan) :
    (plans.map fun plan =>
        Wheel30RuntimePlan.rebase (base₂ := base₂)
          (Wheel30BoundedCycleSlotPlan.toRuntimePlan (base := base₁) plan)) =
      plans.map (fun plan =>
        Wheel30BoundedCycleSlotPlan.toRuntimePlan (base := base₂) plan) := by
  induction plans with
  | nil =>
      rfl
  | cons plan plans ih =>
      simp

@[simp] theorem wheel30RuntimeCoordsOfBoundedCycleSlots_eq_map_rebase
    {base₁ base₂ : ℕ} (cycleSlots : List Wheel30BoundedCycleSlot) :
    wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₂) cycleSlots =
      (wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots).map
        (fun coord => coord.rebase (base₂ := base₂)) := by
  simp [wheel30RuntimeCoordsOfBoundedCycleSlots, List.map_map]

theorem wheel30BoundedCycleSlotWriteByByte_eq_rebasedCoordsWriteByByte
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (cycleSlots : List Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotWriteByByte bytes cycleSlots =
      wheel30RuntimeWriteByByte (base := base₂) bytes
        ((wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots).map
          (fun coord => coord.rebase (base₂ := base₂))) := by
  simpa [wheel30RuntimeCoordsOfBoundedCycleSlots_eq_map_rebase
      (base₁ := base₁) (base₂ := base₂) (cycleSlots := cycleSlots)] using
    (wheel30BoundedCycleSlotWriteByByte_eq_mappedCoordsWriteByByte
      (base := base₂) (bytes := bytes) (cycleSlots := cycleSlots))

theorem wheel30RuntimeCoordOfBoundedCycleSlot_mem_rebasedCoordsOfBoundedCycleSlots
    {base₁ : ℕ} (base₂ : ℕ) (cycleSlots : List Wheel30BoundedCycleSlot)
    {cycleSlot : Wheel30BoundedCycleSlot} (hCycleSlot : cycleSlot ∈ cycleSlots) :
    (wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot).rebase (base₂ := base₂) ∈
      wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₂) cycleSlots := by
  rw [wheel30RuntimeCoordsOfBoundedCycleSlots_eq_map_rebase
      (base₁ := base₁) (base₂ := base₂)]
  exact wheel30RuntimeCoord_rebase_mem_map
    (coords := wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots)
    (coord := wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot)
    (wheel30RuntimeCoordOfBoundedCycleSlot_mem_coordsOfBoundedCycleSlots
      (base := base₁) cycleSlots hCycleSlot)

theorem wheel30RuntimeCoord_mem_of_mem_rebasedCoordsOfBoundedCycleSlots
    {base₁ : ℕ} (base₂ : ℕ) (cycleSlots : List Wheel30BoundedCycleSlot)
    {coord : Wheel30RuntimeCoord base₂}
    (hCoord : coord ∈ wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₂) cycleSlots) :
    coord.rebase (base₂ := base₁) ∈
      wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots := by
  rw [wheel30RuntimeCoordsOfBoundedCycleSlots_eq_map_rebase
      (base₁ := base₁) (base₂ := base₂)] at hCoord
  exact wheel30RuntimeCoord_mem_of_rebased_mem_map
    (base₁ := base₁) (base₂ := base₂)
    (coords := wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots) hCoord

@[simp] theorem wheel30RuntimeCoord_rebase_mem_coordsOfBoundedCycleSlots_iff
    {base₁ base₂ : ℕ} (cycleSlots : List Wheel30BoundedCycleSlot)
    {coord : Wheel30RuntimeCoord base₁} :
    coord.rebase (base₂ := base₂) ∈
        wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₂) cycleSlots ↔
      coord ∈ wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots := by
  rw [wheel30RuntimeCoordsOfBoundedCycleSlots_eq_map_rebase
      (base₁ := base₁) (base₂ := base₂)]
  exact wheel30RuntimeCoord_rebase_mem_map_iff
    (base₂ := base₂)
    (coords := wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots)

theorem wheel30RuntimePlan_rebase_mem_map {base₁ base₂ : ℕ}
    (plans : List (Wheel30RuntimePlan base₁))
    {plan : Wheel30RuntimePlan base₁} (hPlan : plan ∈ plans) :
    plan.rebase (base₂ := base₂) ∈
      plans.map fun plan => plan.rebase (base₂ := base₂) := by
  simpa [Wheel30RuntimePlan.rebase] using
    (coordinatePlan_mem_mappedPlans
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (plans := plans) hPlan)

theorem wheel30RuntimePlan_mem_of_rebased_mem_map {base₁ base₂ : ℕ}
    (plans : List (Wheel30RuntimePlan base₁))
    {plan : Wheel30RuntimePlan base₂}
    (hPlan : plan ∈ plans.map fun plan => plan.rebase (base₂ := base₂)) :
    plan.rebase (base₂ := base₁) ∈ plans := by
  exact coordinatePlan_mem_of_mem_mappedPlans
    (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
    (g := fun coord : Wheel30RuntimeCoord base₂ => coord.rebase (base₂ := base₁))
    (hLeft := Wheel30RuntimeCoord.rebase_leftInverse (base₁ := base₁) (base₂ := base₂))
    (plans := plans) hPlan

@[simp] theorem wheel30RuntimePlan_rebase_mem_map_iff {base₁ base₂ : ℕ}
    (plans : List (Wheel30RuntimePlan base₁))
    {plan : Wheel30RuntimePlan base₁} :
    plan.rebase (base₂ := base₂) ∈
        plans.map (fun plan => plan.rebase (base₂ := base₂)) ↔
      plan ∈ plans := by
  simpa [Wheel30RuntimePlan.rebase] using
    (coordinatePlan_mem_mappedPlans_iff
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (g := fun coord : Wheel30RuntimeCoord base₂ => coord.rebase (base₂ := base₁))
      (hLeft := Wheel30RuntimeCoord.rebase_leftInverse (base₁ := base₁) (base₂ := base₂))
      (plans := plans)
      (plan := plan))

theorem wheel30RuntimeCoord_rebase_mem_rebasedPlan {base₁ base₂ : ℕ}
    (plan : Wheel30RuntimePlan base₁)
    {coord : Wheel30RuntimeCoord base₁} (hCoord : coord ∈ plan.2) :
    coord.rebase (base₂ := base₂) ∈ (plan.rebase (base₂ := base₂)).2 := by
  simpa [Wheel30RuntimePlan.rebase] using
    (CoordinatePlan.mem_map_snd
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (plan := plan) (coord := coord) hCoord)

theorem wheel30RuntimeCoord_mem_of_mem_rebasedPlan {base₁ base₂ : ℕ}
    (plan : Wheel30RuntimePlan base₁)
    {coord : Wheel30RuntimeCoord base₂}
    (hCoord : coord ∈ (plan.rebase (base₂ := base₂)).2) :
    coord.rebase (base₂ := base₁) ∈ plan.2 := by
  exact CoordinatePlan.mem_of_mem_map_snd
    (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
    (g := fun coord : Wheel30RuntimeCoord base₂ => coord.rebase (base₂ := base₁))
    (hLeft := Wheel30RuntimeCoord.rebase_leftInverse (base₁ := base₁) (base₂ := base₂))
    (plan := plan) hCoord

@[simp] theorem wheel30RuntimeCoord_rebase_mem_rebasedPlan_iff {base₁ base₂ : ℕ}
    (plan : Wheel30RuntimePlan base₁)
    {coord : Wheel30RuntimeCoord base₁} :
    coord.rebase (base₂ := base₂) ∈ (plan.rebase (base₂ := base₂)).2 ↔
      coord ∈ plan.2 := by
  rw [Wheel30RuntimePlan.rebase]
  exact CoordinatePlan.mem_map_snd_iff
    (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
    (g := fun coord : Wheel30RuntimeCoord base₂ => coord.rebase (base₂ := base₁))
    (hLeft := Wheel30RuntimeCoord.rebase_leftInverse (base₁ := base₁) (base₂ := base₂))
    (plan := plan)
    (coord := coord)

theorem wheel30RuntimeCoordOfBoundedCycleSlot_mem_rebasedPlan {base₁ base₂ : ℕ}
    (plan : Wheel30RuntimePlan base₁)
    {cycleSlot : Wheel30BoundedCycleSlot}
    (hCoord :
      wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot ∈ plan.2) :
    wheel30RuntimeCoordOfBoundedCycleSlot (base := base₂) cycleSlot ∈
      (plan.rebase (base₂ := base₂)).2 := by
  simpa [wheel30RuntimeCoordOfBoundedCycleSlot_rebase] using
    (wheel30RuntimeCoord_rebase_mem_rebasedPlan
      (base₂ := base₂) (plan := plan)
      (coord := wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot)
      hCoord)

theorem wheel30RuntimeCoordOfBoundedCycleSlot_mem_of_mem_rebasedPlan {base₁ base₂ : ℕ}
    (plan : Wheel30RuntimePlan base₁)
    {cycleSlot : Wheel30BoundedCycleSlot}
    (hCoord :
      wheel30RuntimeCoordOfBoundedCycleSlot (base := base₂) cycleSlot ∈
        (plan.rebase (base₂ := base₂)).2) :
    wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot ∈ plan.2 := by
  simpa [wheel30RuntimeCoordOfBoundedCycleSlot_rebase] using
    (wheel30RuntimeCoord_mem_of_mem_rebasedPlan
      (base₂ := base₂) (plan := plan)
      (coord := wheel30RuntimeCoordOfBoundedCycleSlot (base := base₂) cycleSlot)
      hCoord)

@[simp] theorem wheel30RuntimeCoordOfBoundedCycleSlot_mem_rebasedPlan_iff
    {base₁ base₂ : ℕ}
    (plan : Wheel30RuntimePlan base₁)
    {cycleSlot : Wheel30BoundedCycleSlot} :
    wheel30RuntimeCoordOfBoundedCycleSlot (base := base₂) cycleSlot ∈
        (plan.rebase (base₂ := base₂)).2 ↔
      wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot ∈ plan.2 := by
  exact
    (wheel30RuntimeCoord_rebase_mem_rebasedPlan_iff
      (base₁ := base₁) (base₂ := base₂) (plan := plan)
      (coord := wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot))

theorem wheel30RuntimePlan_rebase_toBytePlan {base₁ base₂ : ℕ}
    (plan : Wheel30RuntimePlan base₁) :
    coordinatePlanToBytePlan (wheel30RuntimeMark (base := base₁)) plan =
      coordinatePlanToBytePlan (wheel30RuntimeMark (base := base₂))
        (plan.rebase (base₂ := base₂)) := by
  simpa [Wheel30RuntimePlan.rebase] using
    (coordinatePlanToBytePlan_map_eq_of_mark_eq
      (mark₁ := wheel30RuntimeMark (base := base₁))
      (mark₂ := wheel30RuntimeMark (base := base₂))
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (hMark := fun coord => wheel30RuntimeMark_rebase (base₂ := base₂) coord)
      plan)

theorem wheel30RuntimePlans_rebase_toBytePlans {base₁ base₂ : ℕ}
    (plans : List (Wheel30RuntimePlan base₁)) :
    plans.map (coordinatePlanToBytePlan (wheel30RuntimeMark (base := base₁))) =
      (plans.map fun plan => plan.rebase (base₂ := base₂)).map
        (coordinatePlanToBytePlan (wheel30RuntimeMark (base := base₂))) := by
  simpa [Wheel30RuntimePlan.rebase] using
    (coordinatePlans_map_toBytePlans_eq_of_mark_eq
      (mark₁ := wheel30RuntimeMark (base := base₁))
      (mark₂ := wheel30RuntimeMark (base := base₂))
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (hMark := fun coord => wheel30RuntimeMark_rebase (base₂ := base₂) coord)
      plans)

theorem wheel30RuntimePlansByByte_rebase_eq {base₁ base₂ : ℕ}
    (coords : List (Wheel30RuntimeCoord base₁)) :
    (coordinatePlansByByte (wheel30RuntimeMark (base := base₁)) coords).map
        (fun plan => Wheel30RuntimePlan.rebase (base₂ := base₂) plan) =
      coordinatePlansByByte (wheel30RuntimeMark (base := base₂))
        (coords.map fun coord => coord.rebase (base₂ := base₂)) := by
  simpa [Wheel30RuntimePlan.rebase] using
    (coordinatePlansByByte_map_eq_of_mark_eq
      (mark₁ := wheel30RuntimeMark (base := base₁))
      (mark₂ := wheel30RuntimeMark (base := base₂))
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (hMark := fun coord => wheel30RuntimeMark_rebase (base₂ := base₂) coord)
      (coords := coords))

theorem wheel30RuntimeCycleSlotsPlansByByte_rebase_eq
    {base₁ base₂ : ℕ}
    (cycleSlots : List (ℕ × Fin 8))
    (hCycles : ∀ cycleSlot ∈ cycleSlots, cycleSlot.1 < wheel30SegmentBytes) :
    (coordinatePlansByByte (wheel30RuntimeMark (base := base₁))
        (wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles)).map
        (fun plan => Wheel30RuntimePlan.rebase (base₂ := base₂) plan) =
      coordinatePlansByByte (wheel30RuntimeMark (base := base₂))
        (wheel30RuntimeCoordsOfCycleSlots (base := base₂) cycleSlots hCycles) := by
  rw [wheel30RuntimePlansByByte_rebase_eq
      (base₁ := base₁) (base₂ := base₂)
      (coords := wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles)]
  rw [wheel30RuntimeCoordsOfCycleSlots_eq_map_rebase
      (base₁ := base₁) (base₂ := base₂) (cycleSlots := cycleSlots) (hCycles := hCycles)]

theorem wheel30RuntimeBoundedCycleSlotsPlansByByte_rebase_eq
    {base₁ base₂ : ℕ}
    (cycleSlots : List Wheel30BoundedCycleSlot) :
    (coordinatePlansByByte (wheel30RuntimeMark (base := base₁))
        (wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots)).map
        (fun plan => Wheel30RuntimePlan.rebase (base₂ := base₂) plan) =
      coordinatePlansByByte (wheel30RuntimeMark (base := base₂))
        (wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₂) cycleSlots) := by
  rw [wheel30RuntimePlansByByte_rebase_eq
      (base₁ := base₁) (base₂ := base₂)
      (coords := wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots)]
  rw [wheel30RuntimeCoordsOfBoundedCycleSlots_eq_map_rebase
      (base₁ := base₁) (base₂ := base₂) (cycleSlots := cycleSlots)]

theorem wheel30RuntimeWriteMany_rebasedPlansByByte_eq_rebasedCoordsWriteByByte
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (coords : List (Wheel30RuntimeCoord base₁)) :
    wheel30RuntimeWriteMany (base := base₂) bytes
      ((coordinatePlansByByte (wheel30RuntimeMark (base := base₁)) coords).map
        (fun plan => Wheel30RuntimePlan.rebase (base₂ := base₂) plan)) =
      wheel30RuntimeWriteByByte (base := base₂) bytes
        (coords.map fun coord => coord.rebase (base₂ := base₂)) := by
  unfold wheel30RuntimeWriteMany wheel30RuntimeWriteByByte coordinateWriteMany
  rw [wheel30RuntimePlansByByte_rebase_eq (base₁ := base₁) (base₂ := base₂) (coords := coords)]

theorem wheel30RuntimeWriteMany_rebasedCycleSlotsPlansByByte_eq
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (cycleSlots : List (ℕ × Fin 8))
    (hCycles : ∀ cycleSlot ∈ cycleSlots, cycleSlot.1 < wheel30SegmentBytes) :
    wheel30RuntimeWriteMany (base := base₂) bytes
      ((coordinatePlansByByte (wheel30RuntimeMark (base := base₁))
          (wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles)).map
        (fun plan => Wheel30RuntimePlan.rebase (base₂ := base₂) plan)) =
      wheel30RuntimeWriteByByte (base := base₂) bytes
        (wheel30RuntimeCoordsOfCycleSlots (base := base₂) cycleSlots hCycles) := by
  unfold wheel30RuntimeWriteMany wheel30RuntimeWriteByByte coordinateWriteMany
  rw [wheel30RuntimeCycleSlotsPlansByByte_rebase_eq
      (base₁ := base₁) (base₂ := base₂) (cycleSlots := cycleSlots) (hCycles := hCycles)]

theorem wheel30RuntimeWriteMany_rebasedBoundedCycleSlotsPlansByByte_eq
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (cycleSlots : List Wheel30BoundedCycleSlot) :
    wheel30RuntimeWriteMany (base := base₂) bytes
      ((coordinatePlansByByte (wheel30RuntimeMark (base := base₁))
          (wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots)).map
        (fun plan => Wheel30RuntimePlan.rebase (base₂ := base₂) plan)) =
      wheel30RuntimeWriteByByte (base := base₂) bytes
        (wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₂) cycleSlots) := by
  unfold wheel30RuntimeWriteMany wheel30RuntimeWriteByByte coordinateWriteMany
  rw [wheel30RuntimeBoundedCycleSlotsPlansByByte_rebase_eq
      (base₁ := base₁) (base₂ := base₂) (cycleSlots := cycleSlots)]

theorem wheel30RuntimeWriteMany_eq_rebasedPlansWriteMany
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (plans : List (Wheel30RuntimePlan base₁)) :
    wheel30RuntimeWriteMany (base := base₁) bytes plans =
      wheel30RuntimeWriteMany (base := base₂) bytes
        (plans.map fun plan => plan.rebase (base₂ := base₂)) := by
  simpa [wheel30RuntimeWriteMany, coordinateWriteMany, Wheel30RuntimePlan.rebase] using
    (coordinatePlanWriteMany_eq_mappedPlans_of_mark_eq
      (mark₁ := wheel30RuntimeMark (base := base₁))
      (mark₂ := wheel30RuntimeMark (base := base₂))
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (hMark := fun coord => wheel30RuntimeMark_rebase (base₂ := base₂) coord)
      (bytes := bytes) (plans := plans))

theorem wheel30BoundedCycleSlotWriteMany_eq_rebasedRuntimePlansWriteMany
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (plans : List Wheel30BoundedCycleSlotPlan) :
    wheel30BoundedCycleSlotWriteMany bytes plans =
      wheel30RuntimeWriteMany (base := base₂) bytes
        ((plans.map fun plan =>
            Wheel30BoundedCycleSlotPlan.toRuntimePlan (base := base₁) plan).map
          (fun plan => Wheel30RuntimePlan.rebase (base₂ := base₂) plan)) := by
  calc
    wheel30BoundedCycleSlotWriteMany bytes plans =
        wheel30RuntimeWriteMany (base := base₁) bytes
          (plans.map fun plan =>
            Wheel30BoundedCycleSlotPlan.toRuntimePlan (base := base₁) plan) := by
          exact wheel30BoundedCycleSlotWriteMany_eq_mappedRuntimePlansWriteMany
            (base := base₁) (bytes := bytes) (plans := plans)
    _ = wheel30RuntimeWriteMany (base := base₂) bytes
          ((plans.map fun plan =>
              Wheel30BoundedCycleSlotPlan.toRuntimePlan (base := base₁) plan).map
            (fun plan => Wheel30RuntimePlan.rebase (base₂ := base₂) plan)) := by
          exact wheel30RuntimeWriteMany_eq_rebasedPlansWriteMany
            (bytes := bytes) (base₁ := base₁) (base₂ := base₂)
            (plans := plans.map fun plan =>
              Wheel30BoundedCycleSlotPlan.toRuntimePlan (base := base₁) plan)

theorem wheel30RuntimePlans_rebase_aligned_iff {base₁ base₂ : ℕ}
    (plans : List (Wheel30RuntimePlan base₁)) :
    (∀ plan ∈ plans.map (fun plan => plan.rebase (base₂ := base₂)),
        ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1) ↔
      (∀ plan ∈ plans, ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1) := by
  simpa [Wheel30RuntimePlan.rebase] using
    (coordinatePlans_mapped_aligned_iff_of_mark_eq
      (mark₁ := wheel30RuntimeMark (base := base₁))
      (mark₂ := wheel30RuntimeMark (base := base₂))
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (hMark := fun coord => wheel30RuntimeMark_rebase (base₂ := base₂) coord)
      (plans := plans))

theorem wheel30RuntimePlans_rebase_aligned {base₁ base₂ : ℕ}
    (plans : List (Wheel30RuntimePlan base₁))
    (hAligned :
      ∀ plan ∈ plans, ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1) :
    ∀ plan ∈ plans.map (fun plan => plan.rebase (base₂ := base₂)),
      ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1 := by
  exact (wheel30RuntimePlans_rebase_aligned_iff
    (base₁ := base₁) (base₂ := base₂) plans).2 hAligned

theorem wheel30RuntimePlans_aligned_of_rebased {base₁ base₂ : ℕ}
    (plans : List (Wheel30RuntimePlan base₁))
    (hAligned :
      ∀ plan ∈ plans.map (fun plan => plan.rebase (base₂ := base₂)),
        ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1) :
    ∀ plan ∈ plans, ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1 := by
  exact (wheel30RuntimePlans_rebase_aligned_iff
    (base₁ := base₁) (base₂ := base₂) plans).1 hAligned

theorem wheel30RuntimePlans_rebase_distinctByteSlots_iff {base₁ base₂ : ℕ}
    (plans : List (Wheel30RuntimePlan base₁)) :
    coordinatePlansHaveDistinctByteSlots (wheel30RuntimeMark (base := base₂))
        (plans.map fun plan => plan.rebase (base₂ := base₂)) ↔
      coordinatePlansHaveDistinctByteSlots (wheel30RuntimeMark (base := base₁)) plans := by
  simpa [Wheel30RuntimePlan.rebase] using
    (coordinatePlans_mapped_distinctByteSlots_iff_of_mark_eq
      (mark₁ := wheel30RuntimeMark (base := base₁))
      (mark₂ := wheel30RuntimeMark (base := base₂))
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (hMark := fun coord => wheel30RuntimeMark_rebase (base₂ := base₂) coord)
      (plans := plans))

theorem wheel30RuntimePlans_rebase_distinctByteSlots {base₁ base₂ : ℕ}
    (plans : List (Wheel30RuntimePlan base₁))
    (hDistinct :
      coordinatePlansHaveDistinctByteSlots (wheel30RuntimeMark (base := base₁)) plans) :
    coordinatePlansHaveDistinctByteSlots (wheel30RuntimeMark (base := base₂))
      (plans.map fun plan => plan.rebase (base₂ := base₂)) := by
  exact (wheel30RuntimePlans_rebase_distinctByteSlots_iff
    (base₁ := base₁) (base₂ := base₂) plans).2 hDistinct

theorem wheel30RuntimePlans_distinctByteSlots_of_rebased {base₁ base₂ : ℕ}
    (plans : List (Wheel30RuntimePlan base₁))
    (hDistinct :
      coordinatePlansHaveDistinctByteSlots (wheel30RuntimeMark (base := base₂))
        (plans.map fun plan => plan.rebase (base₂ := base₂))) :
    coordinatePlansHaveDistinctByteSlots (wheel30RuntimeMark (base := base₁)) plans := by
  exact (wheel30RuntimePlans_rebase_distinctByteSlots_iff
    (base₁ := base₁) (base₂ := base₂) plans).1 hDistinct

theorem wheel30RuntimeRead_rebasedCoords_base_invariant
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (coords : List (Wheel30RuntimeCoord base₁))
    (coord : Wheel30RuntimeCoord base₁) :
    wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base₁) bytes coords)
        base₁ coord.cycle coord.slot coord.hCycle =
      wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base₂) bytes
          (coords.map fun coord => coord.rebase (base₂ := base₂)))
        base₂ coord.cycle coord.slot coord.hCycle := by
  simpa [wheel30RuntimeWriteByByte, Wheel30RuntimeCoord.rebase] using
    (coordRead_coordinatePlansByByte_eq_of_mark_eq
      (read₁ := fun bytes coord =>
        wheel30CandidateRead bytes base₁ coord.cycle coord.slot coord.hCycle)
      (read₂ := fun bytes coord =>
        wheel30CandidateRead bytes base₂ coord.cycle coord.slot coord.hCycle)
      (mark₁ := wheel30RuntimeMark (base := base₁))
      (mark₂ := wheel30RuntimeMark (base := base₂))
      (hRead₁ := fun bytes coord => wheel30RuntimeRead_eq_byteMarkRead bytes base₁ coord)
      (hRead₂ := fun bytes coord => wheel30RuntimeRead_eq_byteMarkRead bytes base₂ coord)
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (hMark := fun coord => wheel30RuntimeMark_rebase (base₂ := base₂) coord)
      (bytes := bytes) (coords := coords) (coord := coord))

theorem wheel30RuntimeRead_rebasedCoords_eq_one_iff
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (coords : List (Wheel30RuntimeCoord base₁))
    (coord : Wheel30RuntimeCoord base₁) :
    wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base₂) bytes
          (coords.map fun c => c.rebase (base₂ := base₂)))
        base₂ coord.cycle coord.slot coord.hCycle = 1 ↔
      wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base₁) bytes coords)
        base₁ coord.cycle coord.slot coord.hCycle = 1 := by
  simpa [wheel30RuntimeWriteByByte, Wheel30RuntimeCoord.rebase] using
    (coordRead_coordinatePlansByByte_eq_iff_of_mark_eq
      (read₁ := fun bytes coord =>
        wheel30CandidateRead bytes base₁ coord.cycle coord.slot coord.hCycle)
      (read₂ := fun bytes coord =>
        wheel30CandidateRead bytes base₂ coord.cycle coord.slot coord.hCycle)
      (mark₁ := wheel30RuntimeMark (base := base₁))
      (mark₂ := wheel30RuntimeMark (base := base₂))
      (hRead₁ := fun bytes coord => wheel30RuntimeRead_eq_byteMarkRead bytes base₁ coord)
      (hRead₂ := fun bytes coord => wheel30RuntimeRead_eq_byteMarkRead bytes base₂ coord)
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (hMark := fun coord => wheel30RuntimeMark_rebase (base₂ := base₂) coord)
      (bytes := bytes) (coords := coords) (coord := coord) (target := 1)).symm

theorem wheel30RuntimeRead_rebasedCycleSlots_eq_one_iff
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (cycleSlots : List (ℕ × Fin 8))
    (hCycles : ∀ cycleSlot ∈ cycleSlots, cycleSlot.1 < wheel30SegmentBytes)
    (coord : Wheel30RuntimeCoord base₁) :
    wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base₂) bytes
          (wheel30RuntimeCoordsOfCycleSlots (base := base₂) cycleSlots hCycles))
        base₂ coord.cycle coord.slot coord.hCycle = 1 ↔
      wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base₁) bytes
          (wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles))
        base₁ coord.cycle coord.slot coord.hCycle = 1 := by
  rw [wheel30RuntimeCoordsOfCycleSlots_eq_map_rebase
      (base₁ := base₁) (base₂ := base₂) (cycleSlots := cycleSlots) (hCycles := hCycles)]
  exact wheel30RuntimeRead_rebasedCoords_eq_one_iff
    (bytes := bytes) (base₁ := base₁) (base₂ := base₂)
    (coords := wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles)
    (coord := coord)

theorem wheel30BoundedCycleSlotRead_rebasedBoundedCycleSlots_eq_one_iff
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (cycleSlots : List Wheel30BoundedCycleSlot)
    (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteByByte (base := base₂) bytes
          (wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₂) cycleSlots))
        base₂ cycleSlot = 1 ↔
      wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteByByte (base := base₁) bytes
          (wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots))
        base₁ cycleSlot = 1 := by
  rw [wheel30RuntimeCoordsOfBoundedCycleSlots_eq_map_rebase
      (base₁ := base₁) (base₂ := base₂) (cycleSlots := cycleSlots)]
  rw [← wheel30CandidateRead_eq_boundedCycleSlotRead
      (bytes := wheel30RuntimeWriteByByte (base := base₂) bytes
        ((wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots).map
          (fun coord => coord.rebase (base₂ := base₂))))
      (base := base₂) (cycleSlot := cycleSlot)]
  rw [← wheel30CandidateRead_eq_boundedCycleSlotRead
      (bytes := wheel30RuntimeWriteByByte (base := base₁) bytes
        (wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots))
      (base := base₁) (cycleSlot := cycleSlot)]
  exact wheel30RuntimeRead_rebasedCoords_eq_one_iff
    (bytes := bytes) (base₁ := base₁) (base₂ := base₂)
    (coords := wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots)
    (coord := wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot)

theorem wheel30RuntimeRead_rebasedPlans_base_invariant
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (plans : List (Wheel30RuntimePlan base₁))
    (coord : Wheel30RuntimeCoord base₁) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany (base := base₁) bytes plans)
        base₁ coord.cycle coord.slot coord.hCycle =
      wheel30CandidateRead
        (wheel30RuntimeWriteMany (base := base₂) bytes
          (plans.map fun plan => plan.rebase (base₂ := base₂)))
        base₂ coord.cycle coord.slot coord.hCycle := by
  simpa [wheel30RuntimeWriteMany, coordinateWriteMany, Wheel30RuntimeCoord.rebase] using
    (coordRead_mappedPlans_eq_of_mark_eq
      (read₁ := fun bytes coord =>
        wheel30CandidateRead bytes base₁ coord.cycle coord.slot coord.hCycle)
      (read₂ := fun bytes coord =>
        wheel30CandidateRead bytes base₂ coord.cycle coord.slot coord.hCycle)
      (mark₁ := wheel30RuntimeMark (base := base₁))
      (mark₂ := wheel30RuntimeMark (base := base₂))
      (hRead₁ := fun bytes coord => wheel30RuntimeRead_eq_byteMarkRead bytes base₁ coord)
      (hRead₂ := fun bytes coord => wheel30RuntimeRead_eq_byteMarkRead bytes base₂ coord)
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (hMark := fun coord => wheel30RuntimeMark_rebase (base₂ := base₂) coord)
      (bytes := bytes) (plans := plans) (coord := coord))

theorem wheel30RuntimeRead_rebasedPlans_eq_one_iff
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (plans : List (Wheel30RuntimePlan base₁))
    (coord : Wheel30RuntimeCoord base₁) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany (base := base₂) bytes
          (plans.map fun plan => plan.rebase (base₂ := base₂)))
        base₂ coord.cycle coord.slot coord.hCycle = 1 ↔
      wheel30CandidateRead
        (wheel30RuntimeWriteMany (base := base₁) bytes plans)
        base₁ coord.cycle coord.slot coord.hCycle = 1 := by
  simpa [wheel30RuntimeWriteMany, coordinateWriteMany, Wheel30RuntimeCoord.rebase] using
    (coordRead_mappedPlans_eq_iff_of_mark_eq
      (read₁ := fun bytes coord =>
        wheel30CandidateRead bytes base₁ coord.cycle coord.slot coord.hCycle)
      (read₂ := fun bytes coord =>
        wheel30CandidateRead bytes base₂ coord.cycle coord.slot coord.hCycle)
      (mark₁ := wheel30RuntimeMark (base := base₁))
      (mark₂ := wheel30RuntimeMark (base := base₂))
      (hRead₁ := fun bytes coord => wheel30RuntimeRead_eq_byteMarkRead bytes base₁ coord)
      (hRead₂ := fun bytes coord => wheel30RuntimeRead_eq_byteMarkRead bytes base₂ coord)
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (hMark := fun coord => wheel30RuntimeMark_rebase (base₂ := base₂) coord)
      (bytes := bytes) (plans := plans) (coord := coord) (target := 1)).symm

theorem wheel30BoundedCycleSlotRead_rebasedPlans_eq_one_iff
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (plans : List (Wheel30RuntimePlan base₁))
    (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteMany (base := base₂) bytes
          (plans.map fun plan => plan.rebase (base₂ := base₂)))
        base₂ cycleSlot = 1 ↔
      wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteMany (base := base₁) bytes plans)
        base₁ cycleSlot = 1 := by
  rw [← wheel30CandidateRead_eq_boundedCycleSlotRead
      (bytes := wheel30RuntimeWriteMany (base := base₂) bytes
        (plans.map fun plan => plan.rebase (base₂ := base₂)))
      (base := base₂) (cycleSlot := cycleSlot)]
  rw [← wheel30CandidateRead_eq_boundedCycleSlotRead
      (bytes := wheel30RuntimeWriteMany (base := base₁) bytes plans)
      (base := base₁) (cycleSlot := cycleSlot)]
  exact wheel30RuntimeRead_rebasedPlans_eq_one_iff
    (bytes := bytes) (base₁ := base₁) (base₂ := base₂) (plans := plans)
    (coord := wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot)

theorem wheel30RuntimeRead_of_mem_rebasedCoords_byByte
    {base₁ : ℕ} (base₂ : ℕ) (coords : List (Wheel30RuntimeCoord base₁))
    (bytes : Wheel30ByteState)
    {coord : Wheel30RuntimeCoord base₁} (hCoord : coord ∈ coords) :
    wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base₂) bytes
          (coords.map fun coord => coord.rebase (base₂ := base₂)))
        base₂ coord.cycle coord.slot coord.hCycle = 1 := by
  simpa [wheel30RuntimeWriteByByte] using
    (read_of_mem_coordinatePlansByByte_mapped_of_mark_eq
      (Coord₁ := Wheel30RuntimeCoord base₁)
      (Coord₂ := Wheel30RuntimeCoord base₂)
      (mark₁ := wheel30RuntimeMark (base := base₁))
      (mark₂ := wheel30RuntimeMark (base := base₂))
      (read := fun bytes (coord : Wheel30RuntimeCoord base₁) =>
        wheel30CandidateRead bytes base₂ coord.cycle coord.slot coord.hCycle)
      (f := fun (coord : Wheel30RuntimeCoord base₁) => coord.rebase (base₂ := base₂))
      (hRead := fun bytes (coord : Wheel30RuntimeCoord base₁) => by
        simpa [wheel30RuntimeMark] using
          (wheel30CandidateRead_eq_byteMarkRead
            bytes base₂ coord.cycle coord.slot coord.hCycle))
      (hMark := fun (coord : Wheel30RuntimeCoord base₁) => by
        simp)
      (bytes := bytes) (coords := coords) (coord := coord) hCoord)

theorem wheel30RuntimeRead_of_mem_rebasedPlansByByte
    {base₁ : ℕ} (base₂ : ℕ) (coords : List (Wheel30RuntimeCoord base₁))
    (bytes : Wheel30ByteState)
    {coord : Wheel30RuntimeCoord base₁} (hCoord : coord ∈ coords) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany (base := base₂) bytes
          ((coordinatePlansByByte (wheel30RuntimeMark (base := base₁)) coords).map
            (fun plan => Wheel30RuntimePlan.rebase (base₂ := base₂) plan)))
        base₂ coord.cycle coord.slot coord.hCycle = 1 := by
  rw [wheel30RuntimeWriteMany_rebasedPlansByByte_eq_rebasedCoordsWriteByByte
      (base₁ := base₁) (base₂ := base₂) (bytes := bytes) (coords := coords)]
  exact wheel30RuntimeRead_of_mem_rebasedCoords_byByte
    (base₂ := base₂) (coords := coords) (bytes := bytes) hCoord

theorem wheel30RuntimeRead_of_mem_rebasedCycleSlotsPlansByByte
    {base₁ : ℕ} (base₂ : ℕ)
    (cycleSlots : List (ℕ × Fin 8))
    (hCycles : ∀ cycleSlot ∈ cycleSlots, cycleSlot.1 < wheel30SegmentBytes)
    (bytes : Wheel30ByteState)
    {cycle : ℕ} {slot : Fin 8} (hCycleSlot : (cycle, slot) ∈ cycleSlots) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany (base := base₂) bytes
          ((coordinatePlansByByte (wheel30RuntimeMark (base := base₁))
              (wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles)).map
            (fun plan => Wheel30RuntimePlan.rebase (base₂ := base₂) plan)))
        base₂ cycle slot (hCycles (cycle, slot) hCycleSlot) = 1 := by
  exact wheel30RuntimeRead_of_mem_rebasedPlansByByte
    (base₂ := base₂)
    (coords := wheel30RuntimeCoordsOfCycleSlots (base := base₁) cycleSlots hCycles)
    (bytes := bytes)
    (coord := (⟨cycle, slot, hCycles (cycle, slot) hCycleSlot⟩ : Wheel30RuntimeCoord base₁))
    (hCoord := wheel30RuntimeCoord_mem_coordsOfCycleSlots cycleSlots hCycles hCycleSlot)

theorem wheel30RuntimeRead_of_mem_rebasedBoundedCycleSlotsPlansByByte
    {base₁ : ℕ} (base₂ : ℕ)
    (cycleSlots : List Wheel30BoundedCycleSlot)
    (bytes : Wheel30ByteState)
    {cycleSlot : Wheel30BoundedCycleSlot} (hCycleSlot : cycleSlot ∈ cycleSlots) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany (base := base₂) bytes
          ((coordinatePlansByByte (wheel30RuntimeMark (base := base₁))
              (wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots)).map
            (fun plan => Wheel30RuntimePlan.rebase (base₂ := base₂) plan)))
        base₂ cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2 = 1 := by
  simpa [wheel30RuntimeCoordOfBoundedCycleSlot] using
    (wheel30RuntimeRead_of_mem_rebasedPlansByByte
      (base₂ := base₂)
      (coords := wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots)
      (bytes := bytes)
      (coord := wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot)
      (hCoord := wheel30RuntimeCoordOfBoundedCycleSlot_mem_coordsOfBoundedCycleSlots
        (base := base₁) cycleSlots hCycleSlot))

theorem wheel30RuntimeRead_of_mem_byByte_of_rebasedCoords
    {base₁ : ℕ} (base₂ base₃ : ℕ) (coords : List (Wheel30RuntimeCoord base₁))
    (bytes : Wheel30ByteState)
    {coord : Wheel30RuntimeCoord base₂}
    (hCoord : coord ∈ coords.map fun coord => coord.rebase (base₂ := base₂)) :
    wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base₁) bytes coords)
        base₃ coord.cycle coord.slot coord.hCycle = 1 := by
  rw [wheel30CandidateRead_base_invariant
      (bytes := wheel30RuntimeWriteByByte (base := base₁) bytes coords)
      (base₁ := base₃) (base₂ := base₁) (cycle := coord.cycle)
      (slot := coord.slot) (hCycle := coord.hCycle)]
  simpa [wheel30RuntimeWriteByByte, Wheel30RuntimeCoord.rebase] using
    (read_of_mem_coordinatePlansByByte_of_eq_of_leftInverse
      (mark := wheel30RuntimeMark (base := base₁))
      (read := fun bytes (coord : Wheel30RuntimeCoord base₁) =>
        wheel30CandidateRead bytes base₁ coord.cycle coord.slot coord.hCycle)
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (g := fun coord : Wheel30RuntimeCoord base₂ => coord.rebase (base₂ := base₁))
      (hRead := fun bytes coord => wheel30RuntimeRead_eq_byteMarkRead bytes base₁ coord)
      (hLeft := Wheel30RuntimeCoord.rebase_leftInverse (base₁ := base₁) (base₂ := base₂))
      (bytes := bytes) (coords := coords) (coord := coord) hCoord)

theorem wheel30RuntimeRead_of_mem_rebasedCoords_byByte_of_rebased
    {base₁ : ℕ} (base₂ base₃ : ℕ) (coords : List (Wheel30RuntimeCoord base₁))
    (bytes : Wheel30ByteState)
    {coord : Wheel30RuntimeCoord base₂}
    (hCoord : coord ∈ coords.map fun coord => coord.rebase (base₂ := base₂)) :
    wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base₂) bytes
          (coords.map fun coord => coord.rebase (base₂ := base₂)))
        base₃ coord.cycle coord.slot coord.hCycle = 1 := by
  rw [wheel30CandidateRead_base_invariant
      (bytes := wheel30RuntimeWriteByByte (base := base₂) bytes
        (coords.map fun coord => coord.rebase (base₂ := base₂)))
      (base₁ := base₃) (base₂ := base₂) (cycle := coord.cycle)
      (slot := coord.slot) (hCycle := coord.hCycle)]
  simpa [wheel30RuntimeWriteByByte, Wheel30RuntimeCoord.rebase] using
    (read_of_mem_coordinatePlansByByte_mapped_of_mark_eq_of_leftInverse
      (mark₁ := wheel30RuntimeMark (base := base₁))
      (mark₂ := wheel30RuntimeMark (base := base₂))
      (read := fun bytes (coord : Wheel30RuntimeCoord base₁) =>
        wheel30CandidateRead bytes base₂ coord.cycle coord.slot coord.hCycle)
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (g := fun coord : Wheel30RuntimeCoord base₂ => coord.rebase (base₂ := base₁))
      (hRead := fun bytes (coord : Wheel30RuntimeCoord base₁) => by
        simpa [wheel30RuntimeMark] using
          (wheel30CandidateRead_eq_byteMarkRead
            bytes base₂ coord.cycle coord.slot coord.hCycle))
      (hMark := fun coord => by
        simp [wheel30RuntimeMark_rebase])
      (hLeft := Wheel30RuntimeCoord.rebase_leftInverse (base₁ := base₁) (base₂ := base₂))
      (bytes := bytes) (coords := coords) (coord := coord) hCoord)

theorem wheel30RuntimeRead_of_mem_rebasedBoundedCycleSlots_byByte
    {base₁ : ℕ} (base₂ : ℕ) (cycleSlots : List Wheel30BoundedCycleSlot)
    (bytes : Wheel30ByteState)
    {cycleSlot : Wheel30BoundedCycleSlot} (hCycleSlot : cycleSlot ∈ cycleSlots) :
    wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base₂) bytes
          ((wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots).map
            (fun coord => coord.rebase (base₂ := base₂))))
        base₂ cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2 = 1 := by
  simpa [wheel30RuntimeCoordOfBoundedCycleSlot_rebase] using
    (wheel30RuntimeRead_of_mem_rebasedCoords_byByte_of_rebased
      (base₂ := base₂) (base₃ := base₂)
      (coords := wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots)
      (bytes := bytes)
      (coord := wheel30RuntimeCoordOfBoundedCycleSlot (base := base₂) cycleSlot)
      (hCoord := by
        rw [← wheel30RuntimeCoordsOfBoundedCycleSlots_eq_map_rebase
          (base₁ := base₁) (base₂ := base₂) (cycleSlots := cycleSlots)]
        simpa [wheel30RuntimeCoordOfBoundedCycleSlot_rebase] using
          wheel30RuntimeCoordOfBoundedCycleSlot_mem_rebasedCoordsOfBoundedCycleSlots
            (base₂ := base₂) (base₁ := base₁) cycleSlots hCycleSlot))

theorem wheel30BoundedCycleSlotRead_of_mem_rebasedBoundedCycleSlots_byByte
    {base₁ : ℕ} (base₂ base₃ : ℕ) (cycleSlots : List Wheel30BoundedCycleSlot)
    (bytes : Wheel30ByteState)
    {cycleSlot : Wheel30BoundedCycleSlot} (hCycleSlot : cycleSlot ∈ cycleSlots) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteByByte (base := base₂) bytes
          ((wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots).map
            (fun coord => coord.rebase (base₂ := base₂))))
        base₃ cycleSlot = 1 := by
  rw [wheel30BoundedCycleSlotRead_base_invariant
      (bytes := wheel30RuntimeWriteByByte (base := base₂) bytes
        ((wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots).map
          (fun coord => coord.rebase (base₂ := base₂))))
      (base₁ := base₃) (base₂ := base₂) (cycleSlot := cycleSlot)]
  rw [← wheel30CandidateRead_eq_boundedCycleSlotRead
      (bytes := wheel30RuntimeWriteByByte (base := base₂) bytes
        ((wheel30RuntimeCoordsOfBoundedCycleSlots (base := base₁) cycleSlots).map
          (fun coord => coord.rebase (base₂ := base₂))))
      (base := base₂) (cycleSlot := cycleSlot)]
  exact wheel30RuntimeRead_of_mem_rebasedBoundedCycleSlots_byByte
    (base₂ := base₂) (cycleSlots := cycleSlots) (bytes := bytes)
    (cycleSlot := cycleSlot) hCycleSlot

theorem wheel30RuntimeRead_of_mem_rebasedPlans_distinct
    {base₁ : ℕ} (base₂ : ℕ)
    (plans : List (Wheel30RuntimePlan base₁)) (bytes : Wheel30ByteState)
    (hAligned :
      ∀ plan ∈ plans, ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1)
    (hDistinct :
      coordinatePlansHaveDistinctByteSlots (wheel30RuntimeMark (base := base₁)) plans)
    {plan : Wheel30RuntimePlan base₁} (hPlan : plan ∈ plans)
    {coord : Wheel30RuntimeCoord base₁} (hCoord : coord ∈ plan.2) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany (base := base₂) bytes
          (plans.map fun plan => plan.rebase (base₂ := base₂)))
        base₂ coord.cycle coord.slot coord.hCycle = 1 := by
  simpa [wheel30RuntimeWriteMany, coordinateWriteMany, Wheel30RuntimePlan.rebase] using
    (coordRead_of_mem_mappedPlans_distinct_of_mark_eq
      (read := fun bytes (coord : Wheel30RuntimeCoord base₁) =>
        wheel30CandidateRead bytes base₂ coord.cycle coord.slot coord.hCycle)
      (mark₁ := wheel30RuntimeMark (base := base₁))
      (mark₂ := wheel30RuntimeMark (base := base₂))
      (hRead := fun bytes coord => by
        simpa [wheel30RuntimeMark] using
          (wheel30CandidateRead_eq_byteMarkRead
            bytes base₂ coord.cycle coord.slot coord.hCycle))
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (hMark := fun coord => wheel30RuntimeMark_rebase (base₂ := base₂) coord)
      (bytes := bytes) (plans := plans)
      (hAligned := hAligned) (hDistinct := hDistinct)
      (plan := plan) (coord := coord) hPlan hCoord)

theorem wheel30RuntimeRead_of_mem_rebasedPlan_distinct
    {base₁ : ℕ} (base₂ : ℕ)
    (plans : List (Wheel30RuntimePlan base₁)) (bytes : Wheel30ByteState)
    (hAligned :
      ∀ plan ∈ plans, ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1)
    (hDistinct :
      coordinatePlansHaveDistinctByteSlots (wheel30RuntimeMark (base := base₁)) plans)
    {plan : Wheel30RuntimePlan base₁} (hPlan : plan ∈ plans)
    {coord : Wheel30RuntimeCoord base₂}
    (hCoord : coord ∈ (plan.rebase (base₂ := base₂)).2) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany (base := base₂) bytes
          (plans.map fun plan => plan.rebase (base₂ := base₂)))
        base₂ coord.cycle coord.slot coord.hCycle = 1 := by
  exact wheel30RuntimeRead_of_mem_plans_distinct (base := base₂)
    (plans := plans.map fun plan => plan.rebase (base₂ := base₂))
    (bytes := bytes)
    (hAligned := wheel30RuntimePlans_rebase_aligned
      (base₂ := base₂) plans hAligned)
    (hDistinct := wheel30RuntimePlans_rebase_distinctByteSlots
      (base₂ := base₂) plans hDistinct)
    (plan := plan.rebase (base₂ := base₂))
    (coord := coord)
    (hPlan := wheel30RuntimePlan_rebase_mem_map
      (base₂ := base₂) plans hPlan)
    hCoord

theorem wheel30RuntimeRead_of_mem_plans_distinct_of_rebased
    {base₁ : ℕ} (base₂ : ℕ)
    (plans : List (Wheel30RuntimePlan base₁)) (bytes : Wheel30ByteState)
    (hAligned :
      ∀ plan ∈ plans.map (fun plan => plan.rebase (base₂ := base₂)),
        ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1)
    (hDistinct :
      coordinatePlansHaveDistinctByteSlots (wheel30RuntimeMark (base := base₂))
        (plans.map fun plan => plan.rebase (base₂ := base₂)))
    {plan : Wheel30RuntimePlan base₁} (hPlan : plan ∈ plans)
    {coord : Wheel30RuntimeCoord base₁} (hCoord : coord ∈ plan.2) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany (base := base₁) bytes plans)
        base₁ coord.cycle coord.slot coord.hCycle = 1 := by
  exact wheel30RuntimeRead_of_mem_plans_distinct (base := base₁)
    (plans := plans) (bytes := bytes)
    (hAligned := wheel30RuntimePlans_aligned_of_rebased
      (base₂ := base₂) plans hAligned)
    (hDistinct := wheel30RuntimePlans_distinctByteSlots_of_rebased
      (base₂ := base₂) plans hDistinct)
    (plan := plan) (coord := coord) hPlan hCoord

theorem wheel30RuntimeRead_of_mem_plan_distinct_of_rebasedFamily
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (plans : List (Wheel30RuntimePlan base₁)) (bytes : Wheel30ByteState)
    (hAligned :
      ∀ plan ∈ plans.map (fun plan => plan.rebase (base₂ := base₂)),
        ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1)
    (hDistinct :
      coordinatePlansHaveDistinctByteSlots (wheel30RuntimeMark (base := base₂))
        (plans.map fun plan => plan.rebase (base₂ := base₂)))
    {plan : Wheel30RuntimePlan base₂}
    (hPlan : plan ∈ plans.map (fun plan => plan.rebase (base₂ := base₂)))
    {coord : Wheel30RuntimeCoord base₂}
    (hCoord : coord ∈ plan.2) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany (base := base₁) bytes plans)
        base₃ coord.cycle coord.slot coord.hCycle = 1 := by
  have hPreimage :=
    coordinatePlan_preimage_membership_of_mem_mappedPlans
      (f := fun coord : Wheel30RuntimeCoord base₁ =>
        coord.rebase (base₂ := base₂))
      (g := fun coord : Wheel30RuntimeCoord base₂ =>
        coord.rebase (base₂ := base₁))
      (hLeft := Wheel30RuntimeCoord.rebase_leftInverse
        (base₁ := base₁) (base₂ := base₂))
      (plans := plans)
      (plan := plan) hPlan
      (coord := coord) hCoord
  have hPlanOrig : plan.rebase (base₂ := base₁) ∈ plans := by
    simpa [Wheel30RuntimePlan.rebase] using hPreimage.1
  have hCoordOrig :
      coord.rebase (base₂ := base₁) ∈ (plan.rebase (base₂ := base₁)).2 := by
    simpa [Wheel30RuntimePlan.rebase] using hPreimage.2
  have hReadBase₁ :
      wheel30CandidateRead
          (wheel30RuntimeWriteMany (base := base₁) bytes plans)
          base₁ (coord.rebase (base₂ := base₁)).cycle
          (coord.rebase (base₂ := base₁)).slot
          (coord.rebase (base₂ := base₁)).hCycle = 1 := by
    exact wheel30RuntimeRead_of_mem_plans_distinct_of_rebased
      (base₂ := base₂)
      (plans := plans) (bytes := bytes)
      (hAligned := hAligned) (hDistinct := hDistinct)
      (plan := plan.rebase (base₂ := base₁)) hPlanOrig
      (coord := coord.rebase (base₂ := base₁)) hCoordOrig
  exact wheel30RuntimeRead_eq_one_of_rebased
    (bytes := wheel30RuntimeWriteMany (base := base₁) bytes plans)
    (readBase₁ := base₁) (readBase₂ := base₃)
    (coord := coord) hReadBase₁

theorem wheel30RuntimeRead_of_mem_rebasedPlan_distinct_of_rebased
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (plans : List (Wheel30RuntimePlan base₁)) (bytes : Wheel30ByteState)
    (hAligned :
      ∀ plan ∈ plans.map (fun plan => plan.rebase (base₂ := base₂)),
        ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1)
    (hDistinct :
      coordinatePlansHaveDistinctByteSlots (wheel30RuntimeMark (base := base₂))
        (plans.map fun plan => plan.rebase (base₂ := base₂)))
    {plan : Wheel30RuntimePlan base₁} (hPlan : plan ∈ plans)
    {coord : Wheel30RuntimeCoord base₂}
    (hCoord : coord ∈ (plan.rebase (base₂ := base₂)).2) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany (base := base₁) bytes plans)
        base₃ coord.cycle coord.slot coord.hCycle = 1 := by
  exact wheel30RuntimeRead_of_mem_plan_distinct_of_rebasedFamily
    (base₂ := base₂) (base₃ := base₃)
    (plans := plans) (bytes := bytes)
    (hAligned := hAligned) (hDistinct := hDistinct)
    (plan := plan.rebase (base₂ := base₂))
    (hPlan := wheel30RuntimePlan_rebase_mem_map (base₂ := base₂) plans hPlan)
    (coord := coord) hCoord

theorem wheel30BoundedCycleSlotRead_of_mem_rebasedPlans_distinct
    {base₁ : ℕ} (base₂ base₃ : ℕ)
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
        (wheel30RuntimeWriteMany (base := base₂) bytes
          (plans.map fun plan => plan.rebase (base₂ := base₂)))
        base₃ cycleSlot = 1 := by
  exact wheel30BoundedCycleSlotRead_of_mem_plans_distinct
    (base₂ := base₃)
    (plans := plans.map fun plan => plan.rebase (base₂ := base₂))
    (bytes := bytes)
    (hAligned := wheel30RuntimePlans_rebase_aligned
      (base₂ := base₂) plans hAligned)
    (hDistinct := wheel30RuntimePlans_rebase_distinctByteSlots
      (base₂ := base₂) plans hDistinct)
    (plan := plan.rebase (base₂ := base₂))
    (hPlan := wheel30RuntimePlan_rebase_mem_map
      (base₂ := base₂) plans hPlan)
    (cycleSlot := cycleSlot)
    (hCoord := wheel30RuntimeCoordOfBoundedCycleSlot_mem_rebasedPlan
      (base₂ := base₂) (plan := plan) hCoord)

theorem wheel30BoundedCycleSlotRead_of_mem_plans_distinct_of_rebased
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (plans : List (Wheel30RuntimePlan base₁)) (bytes : Wheel30ByteState)
    (hAligned :
      ∀ plan ∈ plans.map (fun plan => plan.rebase (base₂ := base₂)),
        ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1)
    (hDistinct :
      coordinatePlansHaveDistinctByteSlots (wheel30RuntimeMark (base := base₂))
        (plans.map fun plan => plan.rebase (base₂ := base₂)))
    {plan : Wheel30RuntimePlan base₁} (hPlan : plan ∈ plans)
    {cycleSlot : Wheel30BoundedCycleSlot}
    (hCoord :
      wheel30RuntimeCoordOfBoundedCycleSlot (base := base₂) cycleSlot ∈
        (plan.rebase (base₂ := base₂)).2) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteMany (base := base₁) bytes plans)
        base₃ cycleSlot = 1 := by
  exact wheel30BoundedCycleSlotRead_of_mem_plans_distinct
    (base₂ := base₃)
    (plans := plans) (bytes := bytes)
    (hAligned := wheel30RuntimePlans_aligned_of_rebased
      (base₂ := base₂) plans hAligned)
    (hDistinct := wheel30RuntimePlans_distinctByteSlots_of_rebased
      (base₂ := base₂) plans hDistinct)
    (plan := plan) hPlan
    (cycleSlot := cycleSlot)
    (hCoord := wheel30RuntimeCoordOfBoundedCycleSlot_mem_of_mem_rebasedPlan
      (base₂ := base₂) (plan := plan) hCoord)

theorem exists_wheel30BoundedCycleSlot_of_segmentRepresentable
    {base n : ℕ} (hBase : base % 30 = 0)
    (hGe : base ≤ n) (hLt : n < base + wheel30SegmentSpan)
    (hRep : wheel30Representable n) :
    ∃ cycleSlot : Wheel30BoundedCycleSlot,
      n = wheel30BoundedCycleSlotCandidate base cycleSlot ∧
      wheel30Index base n = some (wheel30BoundedCycleSlotLinearIndex cycleSlot) := by
  obtain ⟨idx, hIdx⟩ :=
    exists_wheel30Index_of_segmentRepresentable hBase hGe hLt hRep
  obtain ⟨cycleSlot, hCand, hIdxEq⟩ :=
    (wheel30Index_eq_some_iff_exists_boundedCycleSlot (base := base) (n := n) (idx := idx)).mp hIdx
  exact ⟨cycleSlot, hCand, by simpa [hIdxEq] using hIdx⟩

theorem with_wheel30BoundedCycleSlot_of_segmentRepresentable
    {base n : ℕ} {α : Prop} (hBase : base % 30 = 0)
    (hGe : base ≤ n) (hLt : n < base + wheel30SegmentSpan)
    (hRep : wheel30Representable n)
    (k : (cycleSlot : Wheel30BoundedCycleSlot) →
      n = wheel30BoundedCycleSlotCandidate base cycleSlot →
      wheel30Index base n = some (wheel30BoundedCycleSlotLinearIndex cycleSlot) →
      α) :
    α := by
  obtain ⟨cycleSlot, hCand, hIdx⟩ :=
    exists_wheel30BoundedCycleSlot_of_segmentRepresentable
      (hBase := hBase) (hGe := hGe) (hLt := hLt) (hRep := hRep)
  exact k cycleSlot hCand hIdx

theorem with_wheel30BoundedCycleSlotPair_of_segmentRepresentable
    {base n₁ n₂ : ℕ} {α : Prop} (hBase : base % 30 = 0)
    (hGe₁ : base ≤ n₁) (hLt₁ : n₁ < base + wheel30SegmentSpan)
    (hRep₁ : wheel30Representable n₁)
    (hGe₂ : base ≤ n₂) (hLt₂ : n₂ < base + wheel30SegmentSpan)
    (hRep₂ : wheel30Representable n₂)
    (k : (cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot) →
      n₁ = wheel30BoundedCycleSlotCandidate base cycleSlot₁ →
      n₂ = wheel30BoundedCycleSlotCandidate base cycleSlot₂ →
      wheel30Index base n₁ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₁) →
      wheel30Index base n₂ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₂) →
      α) :
    α := by
  exact with_wheel30BoundedCycleSlot_of_segmentRepresentable
    (hBase := hBase) (hGe := hGe₁) (hLt := hLt₁) (hRep := hRep₁)
    (fun cycleSlot₁ hCand₁ hIdx₁ =>
      with_wheel30BoundedCycleSlot_of_segmentRepresentable
        (hBase := hBase) (hGe := hGe₂) (hLt := hLt₂) (hRep := hRep₂)
        (fun cycleSlot₂ hCand₂ hIdx₂ =>
          k cycleSlot₁ cycleSlot₂ hCand₁ hCand₂ hIdx₁ hIdx₂))

theorem wheel30BoundedCycleSlot_cycle_ne_of_thirty_le_sub
    {base : ℕ} {cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot}
    (hSep :
      30 ≤ wheel30BoundedCycleSlotCandidate base cycleSlot₂ -
        wheel30BoundedCycleSlotCandidate base cycleSlot₁) :
    cycleSlot₁.1.1 ≠ cycleSlot₂.1.1 := by
  rcases cycleSlot₁ with ⟨⟨cycle₁, slot₁⟩, hCycle₁⟩
  rcases cycleSlot₂ with ⟨⟨cycle₂, slot₂⟩, hCycle₂⟩
  exact wheel30Candidate_cycle_ne_of_thirty_le_sub
    (base := base) (slot₁ := slot₁) (slot₂ := slot₂) <|
      by simpa [wheel30BoundedCycleSlotCandidate] using hSep

/-- Rebased singleton write on the grouped wheel30 runtime-plan surface. -/
def wheel30RuntimeWriteRebasedSingleton
    {base₁ base₂ : ℕ} (bytes : Wheel30ByteState) (coord : Wheel30RuntimeCoord base₁) :
    Wheel30ByteState :=
  wheel30RuntimeWriteMany (base := base₂) bytes
    [singletonWheel30RuntimePlan (coord.rebase (base₂ := base₂))]

/-- Rebased singleton write on the byte-bucketed wheel30 runtime surface. -/
def wheel30RuntimeWriteRebasedSingletonByByte
    {base₁ base₂ : ℕ} (bytes : Wheel30ByteState) (coord : Wheel30RuntimeCoord base₁) :
    Wheel30ByteState :=
  wheel30RuntimeWriteByByte (base := base₂) bytes [coord.rebase (base₂ := base₂)]

/-- Rebased singleton write for one bounded raw wheel30 input. -/
def wheel30BoundedCycleSlotWriteRebasedSingleton
    {base₁ base₂ : ℕ} (bytes : Wheel30ByteState)
    (cycleSlot : Wheel30BoundedCycleSlot) : Wheel30ByteState :=
  wheel30RuntimeWriteRebasedSingleton (base₁ := base₁) (base₂ := base₂) bytes
    (wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot)

/-- Rebased singleton byte-bucket write for one bounded raw wheel30 input. -/
def wheel30BoundedCycleSlotWriteRebasedSingletonByByte
    {base₁ base₂ : ℕ} (bytes : Wheel30ByteState)
    (cycleSlot : Wheel30BoundedCycleSlot) : Wheel30ByteState :=
  wheel30RuntimeWriteRebasedSingletonByByte (base₁ := base₁) (base₂ := base₂) bytes
    (wheel30RuntimeCoordOfBoundedCycleSlot (base := base₁) cycleSlot)

@[simp] theorem wheel30RuntimeWriteRebasedSingleton_eq_runtimeWriteMany_singleton
    {base₁ base₂ : ℕ} (bytes : Wheel30ByteState) (coord : Wheel30RuntimeCoord base₁) :
    wheel30RuntimeWriteRebasedSingleton (base₁ := base₁) (base₂ := base₂) bytes coord =
      wheel30RuntimeWriteMany (base := base₂) bytes
        [singletonWheel30RuntimePlan (coord.rebase (base₂ := base₂))] := rfl

@[simp] theorem wheel30RuntimeWriteRebasedSingletonByByte_eq_runtimeWriteByByte_singleton
    {base₁ base₂ : ℕ} (bytes : Wheel30ByteState) (coord : Wheel30RuntimeCoord base₁) :
    wheel30RuntimeWriteRebasedSingletonByByte
        (base₁ := base₁) (base₂ := base₂) bytes coord =
      wheel30RuntimeWriteByByte (base := base₂) bytes [coord.rebase (base₂ := base₂)] := rfl

@[simp] theorem singletonWheel30RuntimePlan_rebase_eq {base₁ base₂ : ℕ}
    (coord : Wheel30RuntimeCoord base₁) :
    (singletonWheel30RuntimePlan coord).rebase (base₂ := base₂) =
      singletonWheel30RuntimePlan (coord.rebase (base₂ := base₂)) := by
  simpa [Wheel30RuntimePlan.rebase] using
    (CoordinatePlan.map_singletonCoordinatePlan
      (mark₁ := wheel30RuntimeMark (base := base₁))
      (mark₂ := wheel30RuntimeMark (base := base₂))
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (hMark := fun coord => wheel30RuntimeMark_rebase (base₂ := base₂) coord)
      (coord := coord))

theorem wheel30RuntimeRead_rebasedSingleton_eq_one_iff
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (coord : Wheel30RuntimeCoord base₁) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedSingleton (base₁ := base₁) (base₂ := base₂) bytes coord)
        base₂ coord.cycle coord.slot coord.hCycle = 1 ↔
      wheel30CandidateRead
        (wheel30RuntimeWriteMany (base := base₁) bytes [singletonWheel30RuntimePlan coord])
        base₁ coord.cycle coord.slot coord.hCycle = 1 := by
  simpa [wheel30RuntimeWriteRebasedSingleton] using
    (wheel30RuntimeRead_rebasedPlans_eq_one_iff
      (bytes := bytes) (base₁ := base₁) (base₂ := base₂)
      (plans := [singletonWheel30RuntimePlan coord]) (coord := coord))

theorem wheel30RuntimeRead_rebasedSingleton_byByte_eq_one_iff
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (coord : Wheel30RuntimeCoord base₁) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedSingletonByByte
          (base₁ := base₁) (base₂ := base₂) bytes coord)
        base₂ coord.cycle coord.slot coord.hCycle = 1 ↔
      wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base₁) bytes [coord])
        base₁ coord.cycle coord.slot coord.hCycle = 1 := by
  simpa [wheel30RuntimeWriteRebasedSingletonByByte] using
    (wheel30RuntimeRead_rebasedCoords_eq_one_iff
      (bytes := bytes) (base₁ := base₁) (base₂ := base₂)
      (coords := [coord]) (coord := coord))

theorem wheel30RuntimeRead_of_rebasedSingleton_byByte
    {base₁ : ℕ} (base₂ : ℕ) (bytes : Wheel30ByteState)
    (coord : Wheel30RuntimeCoord base₁) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedSingletonByByte
          (base₁ := base₁) (base₂ := base₂) bytes coord)
        base₂ coord.cycle coord.slot coord.hCycle = 1 := by
  simpa using
    (wheel30RuntimeRead_singleton_byByte (base := base₂) (bytes := bytes)
      (coord := coord.rebase (base₂ := base₂)))

theorem wheel30RuntimeRead_of_rebasedSingletonByByte_base_invariant
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    (coord : Wheel30RuntimeCoord base₁) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedSingletonByByte
          (base₁ := base₁) (base₂ := base₂) bytes coord)
        base₃ coord.cycle coord.slot coord.hCycle = 1 := by
  exact wheel30RuntimeRead_eq_one_of_base_invariant
    (bytes := wheel30RuntimeWriteRebasedSingletonByByte
      (base₁ := base₁) (base₂ := base₂) bytes coord)
    (readBase₁ := base₂) (readBase₂ := base₃)
    (coord := coord)
    (wheel30RuntimeRead_of_rebasedSingleton_byByte
      (base₁ := base₁) (base₂ := base₂) (bytes := bytes) coord)

theorem wheel30BoundedCycleSlotRead_of_rebasedSingleton_byByte
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotRead
        (wheel30BoundedCycleSlotWriteRebasedSingletonByByte
          (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
        base₃ cycleSlot = 1 := by
  simpa [wheel30BoundedCycleSlotWriteRebasedSingletonByByte,
      wheel30RuntimeCoordOfBoundedCycleSlot_rebase] using
    (wheel30BoundedCycleSlotRead_singleton_byByte
      (base₁ := base₂) (base₂ := base₃) (bytes := bytes) cycleSlot)

theorem wheel30RuntimeRead_of_rebasedSingleton
    {base₁ : ℕ} (base₂ : ℕ) (bytes : Wheel30ByteState)
    (coord : Wheel30RuntimeCoord base₁) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedSingleton (base₁ := base₁) (base₂ := base₂) bytes coord)
        base₂ coord.cycle coord.slot coord.hCycle = 1 := by
  simpa using
    (wheel30RuntimeRead_singleton (base := base₂) (bytes := bytes)
      (coord := coord.rebase (base₂ := base₂)))

theorem wheel30RuntimeRead_of_rebasedSingleton_base_invariant
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    (coord : Wheel30RuntimeCoord base₁) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedSingleton
          (base₁ := base₁) (base₂ := base₂) bytes coord)
        base₃ coord.cycle coord.slot coord.hCycle = 1 := by
  exact wheel30RuntimeRead_eq_one_of_base_invariant
    (bytes := wheel30RuntimeWriteRebasedSingleton
      (base₁ := base₁) (base₂ := base₂) bytes coord)
    (readBase₁ := base₂) (readBase₂ := base₃)
    (coord := coord)
    (wheel30RuntimeRead_of_rebasedSingleton
      (base₁ := base₁) (base₂ := base₂) (bytes := bytes) coord)

theorem wheel30BoundedCycleSlotRead_of_rebasedSingleton
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    (cycleSlot : Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotRead
        (wheel30BoundedCycleSlotWriteRebasedSingleton
          (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
        base₃ cycleSlot = 1 := by
  simpa [wheel30BoundedCycleSlotWriteRebasedSingleton,
      wheel30RuntimeCoordOfBoundedCycleSlot_rebase] using
    (wheel30BoundedCycleSlotRead_singleton
      (base₁ := base₂) (base₂ := base₃) (bytes := bytes) cycleSlot)

theorem with_wheel30BoundedCycleSlot_indexedReadback_of_segmentRepresentable
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n : ℕ} {α : Prop} (hBase : base₁ % 30 = 0)
    (hGe : base₁ ≤ n) (hLt : n < base₁ + wheel30SegmentSpan)
    (hRep : wheel30Representable n)
    (k : (cycleSlot : Wheel30BoundedCycleSlot) →
      n = wheel30BoundedCycleSlotCandidate base₁ cycleSlot →
      wheel30Index base₁ n = some (wheel30BoundedCycleSlotLinearIndex cycleSlot) →
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotWriteRebasedSingleton
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
          base₃ cycleSlot = 1 →
      α) :
    α := by
  exact with_wheel30BoundedCycleSlot_of_segmentRepresentable
    (hBase := hBase) (hGe := hGe) (hLt := hLt) (hRep := hRep)
    (fun cycleSlot hCand hIdx =>
      k cycleSlot hCand hIdx
        (wheel30BoundedCycleSlotRead_of_rebasedSingleton
          (base₁ := base₁) (base₂ := base₂) (base₃ := base₃)
          (bytes := bytes) cycleSlot))

theorem exists_wheel30BoundedCycleSlot_indexedReadback_of_segmentRepresentable
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n : ℕ} (hBase : base₁ % 30 = 0)
    (hGe : base₁ ≤ n) (hLt : n < base₁ + wheel30SegmentSpan)
    (hRep : wheel30Representable n) :
    ∃ cycleSlot : Wheel30BoundedCycleSlot,
      n = wheel30BoundedCycleSlotCandidate base₁ cycleSlot ∧
      wheel30Index base₁ n = some (wheel30BoundedCycleSlotLinearIndex cycleSlot) ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotWriteRebasedSingleton
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
          base₃ cycleSlot = 1 := by
  exact with_wheel30BoundedCycleSlot_indexedReadback_of_segmentRepresentable
    (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
    (hBase := hBase) (hGe := hGe) (hLt := hLt) (hRep := hRep)
    (fun cycleSlot hCand hIdx hRead =>
      ⟨cycleSlot, hCand, hIdx, hRead⟩)

theorem exists_wheel30BoundedCycleSlot_readback_of_segmentRepresentable
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n : ℕ} (hBase : base₁ % 30 = 0)
    (hGe : base₁ ≤ n) (hLt : n < base₁ + wheel30SegmentSpan)
    (hRep : wheel30Representable n) :
    ∃ cycleSlot : Wheel30BoundedCycleSlot,
      n = wheel30BoundedCycleSlotCandidate base₁ cycleSlot ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotWriteRebasedSingleton
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
          base₃ cycleSlot = 1 := by
  obtain ⟨cycleSlot, hCand, _, hRead⟩ :=
    exists_wheel30BoundedCycleSlot_indexedReadback_of_segmentRepresentable
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase) (hGe := hGe) (hLt := hLt) (hRep := hRep)
  exact ⟨cycleSlot, hCand, hRead⟩

theorem exists_wheel30BoundedCycleSlot_indexedReadback_of_segmentPrimeGtThirty
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p : ℕ} (hBase : base₁ % 30 = 0)
    (hGe : base₁ ≤ p) (hLt : p < base₁ + wheel30SegmentSpan)
    (hPrime : Nat.Prime p) (hGt : 30 < p) :
    ∃ cycleSlot : Wheel30BoundedCycleSlot,
      p = wheel30BoundedCycleSlotCandidate base₁ cycleSlot ∧
      wheel30Index base₁ p = some (wheel30BoundedCycleSlotLinearIndex cycleSlot) ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotWriteRebasedSingleton
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
          base₃ cycleSlot = 1 := by
  exact exists_wheel30BoundedCycleSlot_indexedReadback_of_segmentRepresentable
    (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
    (hBase := hBase) (hGe := hGe) (hLt := hLt)
    (hRep := primeGtThirty_wheel30Representable hPrime hGt)

theorem exists_wheel30BoundedCycleSlot_readback_of_segmentPrimeGtThirty
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p : ℕ} (hBase : base₁ % 30 = 0)
    (hGe : base₁ ≤ p) (hLt : p < base₁ + wheel30SegmentSpan)
    (hPrime : Nat.Prime p) (hGt : 30 < p) :
    ∃ cycleSlot : Wheel30BoundedCycleSlot,
      p = wheel30BoundedCycleSlotCandidate base₁ cycleSlot ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotWriteRebasedSingleton
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
          base₃ cycleSlot = 1 := by
  obtain ⟨cycleSlot, hCand, _, hRead⟩ :=
    exists_wheel30BoundedCycleSlot_indexedReadback_of_segmentPrimeGtThirty
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase) (hGe := hGe) (hLt := hLt)
      (hPrime := hPrime) (hGt := hGt)
  exact ⟨cycleSlot, hCand, hRead⟩

theorem exists_wheel30BoundedCycleSlot_indexedRuntimeRead_of_segmentRepresentable
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n : ℕ} (hBase : base₁ % 30 = 0)
    (hGe : base₁ ≤ n) (hLt : n < base₁ + wheel30SegmentSpan)
    (hRep : wheel30Representable n) :
    ∃ cycleSlot : Wheel30BoundedCycleSlot,
      n = wheel30BoundedCycleSlotCandidate base₁ cycleSlot ∧
      wheel30Index base₁ n = some (wheel30BoundedCycleSlotLinearIndex cycleSlot) ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotWriteRebasedSingleton
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
          base₃ cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2 = 1 := by
  exact with_wheel30BoundedCycleSlot_indexedReadback_of_segmentRepresentable
    (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
    (hBase := hBase) (hGe := hGe) (hLt := hLt) (hRep := hRep)
    (fun cycleSlot hCand hIdx hRead =>
      ⟨cycleSlot, hCand, hIdx, by
        rw [wheel30CandidateRead_eq_boundedCycleSlotRead
            (bytes := wheel30BoundedCycleSlotWriteRebasedSingleton
              (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
            (base := base₃) (cycleSlot := cycleSlot)]
        exact hRead⟩)

theorem exists_wheel30BoundedCycleSlot_indexedRuntimeRead_of_segmentPrimeGtThirty
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p : ℕ} (hBase : base₁ % 30 = 0)
    (hGe : base₁ ≤ p) (hLt : p < base₁ + wheel30SegmentSpan)
    (hPrime : Nat.Prime p) (hGt : 30 < p) :
    ∃ cycleSlot : Wheel30BoundedCycleSlot,
      p = wheel30BoundedCycleSlotCandidate base₁ cycleSlot ∧
      wheel30Index base₁ p = some (wheel30BoundedCycleSlotLinearIndex cycleSlot) ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotWriteRebasedSingleton
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
          base₃ cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2 = 1 := by
  exact exists_wheel30BoundedCycleSlot_indexedRuntimeRead_of_segmentRepresentable
    (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
    (hBase := hBase) (hGe := hGe) (hLt := hLt)
    (hRep := primeGtThirty_wheel30Representable hPrime hGt)

theorem exists_wheel30BoundedCycleSlot_runtimeRead_of_segmentRepresentable
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n : ℕ} (hBase : base₁ % 30 = 0)
    (hGe : base₁ ≤ n) (hLt : n < base₁ + wheel30SegmentSpan)
    (hRep : wheel30Representable n) :
    ∃ cycleSlot : Wheel30BoundedCycleSlot,
      n = wheel30BoundedCycleSlotCandidate base₁ cycleSlot ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotWriteRebasedSingleton
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
          base₃ cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2 = 1 := by
  obtain ⟨cycleSlot, hCand, _, hRead⟩ :=
    exists_wheel30BoundedCycleSlot_indexedRuntimeRead_of_segmentRepresentable
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase) (hGe := hGe) (hLt := hLt) (hRep := hRep)
  exact ⟨cycleSlot, hCand, hRead⟩

theorem exists_wheel30BoundedCycleSlot_runtimeRead_of_segmentPrimeGtThirty
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p : ℕ} (hBase : base₁ % 30 = 0)
    (hGe : base₁ ≤ p) (hLt : p < base₁ + wheel30SegmentSpan)
    (hPrime : Nat.Prime p) (hGt : 30 < p) :
    ∃ cycleSlot : Wheel30BoundedCycleSlot,
      p = wheel30BoundedCycleSlotCandidate base₁ cycleSlot ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotWriteRebasedSingleton
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
          base₃ cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2 = 1 := by
  obtain ⟨cycleSlot, hCand, _, hRead⟩ :=
    exists_wheel30BoundedCycleSlot_indexedRuntimeRead_of_segmentPrimeGtThirty
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase) (hGe := hGe) (hLt := hLt)
      (hPrime := hPrime) (hGt := hGt)
  exact ⟨cycleSlot, hCand, hRead⟩

theorem with_wheel30BoundedCycleSlot_indexedReadback_byByte_of_segmentRepresentable
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n : ℕ} {α : Prop} (hBase : base₁ % 30 = 0)
    (hGe : base₁ ≤ n) (hLt : n < base₁ + wheel30SegmentSpan)
    (hRep : wheel30Representable n)
    (k : (cycleSlot : Wheel30BoundedCycleSlot) →
      n = wheel30BoundedCycleSlotCandidate base₁ cycleSlot →
      wheel30Index base₁ n = some (wheel30BoundedCycleSlotLinearIndex cycleSlot) →
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotWriteRebasedSingletonByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
          base₃ cycleSlot = 1 →
      α) :
    α := by
  exact with_wheel30BoundedCycleSlot_of_segmentRepresentable
    (hBase := hBase) (hGe := hGe) (hLt := hLt) (hRep := hRep)
    (fun cycleSlot hCand hIdx =>
      k cycleSlot hCand hIdx
        (wheel30BoundedCycleSlotRead_of_rebasedSingleton_byByte
          (base₁ := base₁) (base₂ := base₂) (base₃ := base₃)
          (bytes := bytes) cycleSlot))

theorem exists_wheel30BoundedCycleSlot_readback_byByte_of_segmentRepresentable
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n : ℕ} (hBase : base₁ % 30 = 0)
    (hGe : base₁ ≤ n) (hLt : n < base₁ + wheel30SegmentSpan)
    (hRep : wheel30Representable n) :
    ∃ cycleSlot : Wheel30BoundedCycleSlot,
      n = wheel30BoundedCycleSlotCandidate base₁ cycleSlot ∧
      wheel30Index base₁ n = some (wheel30BoundedCycleSlotLinearIndex cycleSlot) ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotWriteRebasedSingletonByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
          base₃ cycleSlot = 1 := by
  exact with_wheel30BoundedCycleSlot_indexedReadback_byByte_of_segmentRepresentable
    (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
    (hBase := hBase) (hGe := hGe) (hLt := hLt) (hRep := hRep)
    (fun cycleSlot hCand hIdx hRead =>
      ⟨cycleSlot, hCand, hIdx, hRead⟩)

theorem exists_wheel30BoundedCycleSlot_readback_byByte_of_segmentPrimeGtThirty
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p : ℕ} (hBase : base₁ % 30 = 0)
    (hGe : base₁ ≤ p) (hLt : p < base₁ + wheel30SegmentSpan)
    (hPrime : Nat.Prime p) (hGt : 30 < p) :
    ∃ cycleSlot : Wheel30BoundedCycleSlot,
      p = wheel30BoundedCycleSlotCandidate base₁ cycleSlot ∧
      wheel30Index base₁ p = some (wheel30BoundedCycleSlotLinearIndex cycleSlot) ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotWriteRebasedSingletonByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
          base₃ cycleSlot = 1 := by
  exact exists_wheel30BoundedCycleSlot_readback_byByte_of_segmentRepresentable
    (base₁ := base₁) (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
    (hBase := hBase) (hGe := hGe) (hLt := hLt)
    (hRep := primeGtThirty_wheel30Representable hPrime hGt)

theorem exists_wheel30BoundedCycleSlot_indexedRuntimeRead_byByte_of_segmentRepresentable
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n : ℕ} (hBase : base₁ % 30 = 0)
    (hGe : base₁ ≤ n) (hLt : n < base₁ + wheel30SegmentSpan)
    (hRep : wheel30Representable n) :
    ∃ cycleSlot : Wheel30BoundedCycleSlot,
      n = wheel30BoundedCycleSlotCandidate base₁ cycleSlot ∧
      wheel30Index base₁ n = some (wheel30BoundedCycleSlotLinearIndex cycleSlot) ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotWriteRebasedSingletonByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
          base₃ cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2 = 1 := by
  exact with_wheel30BoundedCycleSlot_indexedReadback_byByte_of_segmentRepresentable
    (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
    (hBase := hBase) (hGe := hGe) (hLt := hLt) (hRep := hRep)
    (fun cycleSlot hCand hIdx hRead =>
      ⟨cycleSlot, hCand, hIdx, by
        rw [wheel30CandidateRead_eq_boundedCycleSlotRead
            (bytes := wheel30BoundedCycleSlotWriteRebasedSingletonByByte
              (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
            (base := base₃) (cycleSlot := cycleSlot)]
        exact hRead⟩)

theorem exists_wheel30BoundedCycleSlot_indexedRuntimeRead_byByte_of_segmentPrimeGtThirty
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p : ℕ} (hBase : base₁ % 30 = 0)
    (hGe : base₁ ≤ p) (hLt : p < base₁ + wheel30SegmentSpan)
    (hPrime : Nat.Prime p) (hGt : 30 < p) :
    ∃ cycleSlot : Wheel30BoundedCycleSlot,
      p = wheel30BoundedCycleSlotCandidate base₁ cycleSlot ∧
      wheel30Index base₁ p = some (wheel30BoundedCycleSlotLinearIndex cycleSlot) ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotWriteRebasedSingletonByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
          base₃ cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2 = 1 := by
  exact exists_wheel30BoundedCycleSlot_indexedRuntimeRead_byByte_of_segmentRepresentable
    (base₁ := base₁) (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
    (hBase := hBase) (hGe := hGe) (hLt := hLt)
    (hRep := primeGtThirty_wheel30Representable hPrime hGt)

theorem exists_wheel30BoundedCycleSlot_runtimeRead_byByte_of_segmentRepresentable
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n : ℕ} (hBase : base₁ % 30 = 0)
    (hGe : base₁ ≤ n) (hLt : n < base₁ + wheel30SegmentSpan)
    (hRep : wheel30Representable n) :
    ∃ cycleSlot : Wheel30BoundedCycleSlot,
      n = wheel30BoundedCycleSlotCandidate base₁ cycleSlot ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotWriteRebasedSingletonByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
          base₃ cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2 = 1 := by
  obtain ⟨cycleSlot, hCand, _, hRead⟩ :=
    exists_wheel30BoundedCycleSlot_indexedRuntimeRead_byByte_of_segmentRepresentable
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase) (hGe := hGe) (hLt := hLt) (hRep := hRep)
  exact ⟨cycleSlot, hCand, hRead⟩

theorem exists_wheel30BoundedCycleSlot_runtimeRead_byByte_of_segmentPrimeGtThirty
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p : ℕ} (hBase : base₁ % 30 = 0)
    (hGe : base₁ ≤ p) (hLt : p < base₁ + wheel30SegmentSpan)
    (hPrime : Nat.Prime p) (hGt : 30 < p) :
    ∃ cycleSlot : Wheel30BoundedCycleSlot,
      p = wheel30BoundedCycleSlotCandidate base₁ cycleSlot ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotWriteRebasedSingletonByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot)
          base₃ cycleSlot.1.1 cycleSlot.1.2 cycleSlot.2 = 1 := by
  obtain ⟨cycleSlot, hCand, _, hRead⟩ :=
    exists_wheel30BoundedCycleSlot_indexedRuntimeRead_byByte_of_segmentPrimeGtThirty
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase) (hGe := hGe) (hLt := hLt)
      (hPrime := hPrime) (hGt := hGt)
  exact ⟨cycleSlot, hCand, hRead⟩

@[simp] theorem wheel30RuntimeCycleSlotPair_rebase_eq {base₁ base₂ : ℕ}
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    ((wheel30RuntimeCycleSlotPair (base := base₁)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂).map
      fun coord => coord.rebase (base₂ := base₂)) =
      wheel30RuntimeCycleSlotPair (base := base₂)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ := by
  simp [wheel30RuntimeCycleSlotPair, Wheel30RuntimeCoord.rebase]

@[simp] theorem wheel30RuntimePairPlans_rebase_eq {base₁ base₂ : ℕ}
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    ((coordinatePlanPair (wheel30RuntimeMark (base := base₁))
        (⟨cycle₁, slot₁, hCycle₁⟩ : Wheel30RuntimeCoord base₁)
        (⟨cycle₂, slot₂, hCycle₂⟩ : Wheel30RuntimeCoord base₁)).map
      fun plan => Wheel30RuntimePlan.rebase (base₂ := base₂) plan) =
      wheel30RuntimePairPlans (base := base₂)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ := by
  simpa [wheel30RuntimePairPlans] using
    (coordinatePlanPair_map_eq_of_mark_eq
      (mark₁ := wheel30RuntimeMark (base := base₁))
      (mark₂ := wheel30RuntimeMark (base := base₂))
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (hMark := fun coord => wheel30RuntimeMark_rebase (base₂ := base₂) coord)
      (coord₁ := (⟨cycle₁, slot₁, hCycle₁⟩ : Wheel30RuntimeCoord base₁))
      (coord₂ := (⟨cycle₂, slot₂, hCycle₂⟩ : Wheel30RuntimeCoord base₁)))

/-- Rebased pair write on the grouped wheel30 runtime-plan surface. -/
def wheel30RuntimeWriteRebasedPairPlans
    {base₁ base₂ : ℕ} (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    Wheel30ByteState :=
  wheel30RuntimeWriteMany (base := base₂) bytes
    ((wheel30RuntimePairPlans (base := base₁)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂).map
      fun plan => Wheel30RuntimePlan.rebase (base₂ := base₂) plan)

/-- Rebased pair write on the byte-bucketed wheel30 runtime surface. -/
def wheel30RuntimeWriteRebasedCycleSlotPairByByte
    {base₁ base₂ : ℕ} (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    Wheel30ByteState :=
  wheel30RuntimeWriteByByte (base := base₂) bytes
    ((wheel30RuntimeCycleSlotPair (base := base₁)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂).map
      fun coord => coord.rebase (base₂ := base₂))

/-- Rebased grouped-plan write for one bounded raw wheel30 pair. -/
def wheel30BoundedCycleSlotPairWriteRebasedPairPlans
    {base₁ base₂ : ℕ} (bytes : Wheel30ByteState)
    (cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot) : Wheel30ByteState :=
  wheel30RuntimeWriteRebasedPairPlans (base₁ := base₁) (base₂ := base₂) bytes
    cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2
    cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2

/-- Rebased byte-bucket write for one bounded raw wheel30 pair. -/
def wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
    {base₁ base₂ : ℕ} (bytes : Wheel30ByteState)
    (cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot) : Wheel30ByteState :=
  wheel30RuntimeWriteRebasedCycleSlotPairByByte (base₁ := base₁) (base₂ := base₂) bytes
    cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2
    cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2

@[simp] theorem wheel30RuntimeWriteRebasedPairPlans_eq
    {base₁ base₂ : ℕ} (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    wheel30RuntimeWriteRebasedPairPlans (base₁ := base₁) (base₂ := base₂) bytes
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ =
      wheel30RuntimeWriteMany (base := base₂) bytes
        (wheel30RuntimePairPlans (base := base₂)
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂) := by
  have hPlans :
      (wheel30RuntimePairPlans (base := base₁)
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂).map
        (fun plan => plan.rebase (base₂ := base₂)) =
      wheel30RuntimePairPlans (base := base₂)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ := by
    rw [wheel30RuntimePairPlans]
    exact wheel30RuntimePairPlans_rebase_eq
      (base₁ := base₁) (base₂ := base₂)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂
  simpa [wheel30RuntimeWriteRebasedPairPlans] using
    congrArg
      (fun plans =>
        wheel30RuntimeWriteMany (base := base₂) bytes plans)
      hPlans

@[simp] theorem wheel30RuntimeWriteRebasedCycleSlotPairByByte_eq
    {base₁ base₂ : ℕ} (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    wheel30RuntimeWriteRebasedCycleSlotPairByByte (base₁ := base₁) (base₂ := base₂) bytes
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ =
      wheel30RuntimeWriteByByte (base := base₂) bytes
        (wheel30RuntimeCycleSlotPair (base := base₂)
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂) := by
  rw [wheel30RuntimeWriteRebasedCycleSlotPairByByte, wheel30RuntimeCycleSlotPair_rebase_eq]

@[simp] theorem wheel30BoundedCycleSlotPairWriteRebasedPairPlans_eq
    {base₁ base₂ : ℕ} (bytes : Wheel30ByteState)
    (cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotPairWriteRebasedPairPlans
        (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂ =
      wheel30RuntimeWriteMany (base := base₂) bytes
        (wheel30RuntimePairPlans (base := base₂)
          cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2
          cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2) := by
  simp [wheel30BoundedCycleSlotPairWriteRebasedPairPlans]

@[simp] theorem wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte_eq
    {base₁ base₂ : ℕ} (bytes : Wheel30ByteState)
    (cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot) :
    wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
        (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂ =
      wheel30RuntimeWriteByByte (base := base₂) bytes
        (wheel30RuntimeCycleSlotPair (base := base₂)
          cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2
          cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2) := by
  simp [wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte]

theorem wheel30RuntimeRead_rebasedPairPlans_eq_one_iff
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (coord : Wheel30RuntimeCoord base₁) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₂ coord.cycle coord.slot coord.hCycle = 1 ↔
      wheel30CandidateRead
        (wheel30RuntimeWriteMany (base := base₁) bytes
          (wheel30RuntimePairPlans (base := base₁)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        base₁ coord.cycle coord.slot coord.hCycle = 1 := by
  simpa [wheel30RuntimeWriteRebasedPairPlans] using
    (wheel30RuntimeRead_rebasedPlans_eq_one_iff
      (bytes := bytes) (base₁ := base₁) (base₂ := base₂)
      (plans := wheel30RuntimePairPlans (base := base₁)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
      (coord := coord))

theorem wheel30RuntimeRead_rebasedCycleSlotPair_byByte_eq_one_iff
    (bytes : Wheel30ByteState) {base₁ base₂ : ℕ}
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (coord : Wheel30RuntimeCoord base₁) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedCycleSlotPairByByte
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₂ coord.cycle coord.slot coord.hCycle = 1 ↔
      wheel30CandidateRead
        (wheel30RuntimeWriteByByte (base := base₁) bytes
          (wheel30RuntimeCycleSlotPair (base := base₁)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        base₁ coord.cycle coord.slot coord.hCycle = 1 := by
  simpa [wheel30RuntimeWriteRebasedCycleSlotPairByByte] using
    (wheel30RuntimeRead_rebasedCoords_eq_one_iff
      (bytes := bytes) (base₁ := base₁) (base₂ := base₂)
      (coords := wheel30RuntimeCycleSlotPair (base := base₁)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
      (coord := coord))

theorem wheel30RuntimePairPlans_aligned {base : ℕ}
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    ∀ plan ∈ wheel30RuntimePairPlans (base := base)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂,
      ∀ coord ∈ plan.2, (wheel30RuntimeMark coord).1 = plan.1 := by
  simpa [wheel30RuntimePairPlans] using
    (coordinatePlanPair_aligned
      (mark := wheel30RuntimeMark (base := base))
      (coord₁ := (⟨cycle₁, slot₁, hCycle₁⟩ : Wheel30RuntimeCoord base))
      (coord₂ := (⟨cycle₂, slot₂, hCycle₂⟩ : Wheel30RuntimeCoord base)))

theorem wheel30RuntimePairPlans_distinctByteSlots {base : ℕ}
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂) :
    coordinatePlansHaveDistinctByteSlots (wheel30RuntimeMark (base := base))
      (wheel30RuntimePairPlans (base := base)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂) := by
  have hMarkByte :
      (wheel30RuntimeMark (⟨cycle₁, slot₁, hCycle₁⟩ : Wheel30RuntimeCoord base)).1 ≠
        (wheel30RuntimeMark (⟨cycle₂, slot₂, hCycle₂⟩ : Wheel30RuntimeCoord base)).1 := by
    simpa [wheel30RuntimeMark, wheel30CandidateMark] using
      (wheel30CandidateByteSlot_ne_of_cycle_ne hCycle₁ hCycle₂ hCycle)
  simpa [wheel30RuntimePairPlans] using
    (coordinatePlanPair_distinct_of_byte_ne
      (mark := wheel30RuntimeMark (base := base))
      (coord₁ := (⟨cycle₁, slot₁, hCycle₁⟩ : Wheel30RuntimeCoord base))
      (coord₂ := (⟨cycle₂, slot₂, hCycle₂⟩ : Wheel30RuntimeCoord base))
      hMarkByte)

theorem wheel30RuntimeRead_of_mem_rebasedPairPlan
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂)
    {plan : Wheel30RuntimePlan base₂}
    (hPlan :
      plan ∈ (wheel30RuntimePairPlans (base := base₁)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂).map
          (fun plan => plan.rebase (base₂ := base₂)))
    {coord : Wheel30RuntimeCoord base₂}
    (hCoord : coord ∈ plan.2) :
    wheel30CandidateRead
        (wheel30RuntimeWriteMany (base := base₁) bytes
          (wheel30RuntimePairPlans (base := base₁)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        base₃ coord.cycle coord.slot coord.hCycle = 1 := by
  let coord₁ : Wheel30RuntimeCoord base₁ := ⟨cycle₁, slot₁, hCycle₁⟩
  let coord₂ : Wheel30RuntimeCoord base₁ := ⟨cycle₂, slot₂, hCycle₂⟩
  have hRead :
      ∀ bytes (coord : Wheel30RuntimeCoord base₁),
        wheel30CandidateRead bytes base₃ coord.cycle coord.slot coord.hCycle =
          byteMarkRead bytes (wheel30RuntimeMark coord) := by
    intro bytes coord
    simpa [wheel30RuntimeMark] using
      (wheel30CandidateRead_eq_byteMarkRead
        bytes base₃ coord.cycle coord.slot coord.hCycle)
  have hMarkByte :
      (wheel30RuntimeMark coord₁).1 ≠ (wheel30RuntimeMark coord₂).1 := by
    simpa [coord₁, coord₂, wheel30RuntimeMark, wheel30CandidateMark] using
      (wheel30CandidateByteSlot_ne_of_cycle_ne hCycle₁ hCycle₂ hCycle)
  simpa [coord₁, coord₂, wheel30RuntimeWriteMany, wheel30RuntimePairPlans,
    coordinateWriteMany, Wheel30RuntimeCoord.rebase] using
    (coordRead_of_mem_mappedPair_distinct_of_mark_eq_of_leftInverse
      (read := fun bytes (coord : Wheel30RuntimeCoord base₁) =>
        wheel30CandidateRead bytes base₃ coord.cycle coord.slot coord.hCycle)
      (mark₁ := wheel30RuntimeMark (base := base₁))
      (mark₂ := wheel30RuntimeMark (base := base₂))
      (hRead := hRead)
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (g := fun coord : Wheel30RuntimeCoord base₂ => coord.rebase (base₂ := base₁))
      (hMark := fun coord => wheel30RuntimeMark_rebase (base₂ := base₂) coord)
      (hLeft := Wheel30RuntimeCoord.rebase_leftInverse
        (base₁ := base₁) (base₂ := base₂))
      (bytes := bytes) (coord₁ := coord₁) (coord₂ := coord₂)
      (hByte := hMarkByte) (plan := plan) (coord := coord) hPlan hCoord)

theorem wheel30BoundedCycleSlotRead_of_mem_rebasedPairPlan
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂)
    {plan : Wheel30RuntimePlan base₂}
    (hPlan :
      plan ∈ (wheel30RuntimePairPlans (base := base₁)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂).map
          (fun plan => plan.rebase (base₂ := base₂)))
    {cycleSlot : Wheel30BoundedCycleSlot}
    (hCoord :
      wheel30RuntimeCoordOfBoundedCycleSlot (base := base₂) cycleSlot ∈ plan.2) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteMany (base := base₁) bytes
          (wheel30RuntimePairPlans (base := base₁)
            cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
        base₃ cycleSlot = 1 := by
  exact wheel30BoundedCycleSlotRead_eq_one_of_runtimeRead
    (bytes := wheel30RuntimeWriteMany (base := base₁) bytes
      (wheel30RuntimePairPlans (base := base₁)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂))
    (base₁ := base₃) (base₂ := base₃) (cycleSlot := cycleSlot)
    (wheel30RuntimeRead_of_mem_rebasedPairPlan
      (base₂ := base₂) (base₃ := base₃)
      (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle
      (plan := plan) hPlan
      (coord := wheel30RuntimeCoordOfBoundedCycleSlot (base := base₂) cycleSlot)
      hCoord)

theorem wheel30RuntimeRead_first_of_rebasedPairPlans
    {base₁ : ℕ} (base₂ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₂ cycle₁ slot₁ hCycle₁ = 1 := by
  simpa [wheel30RuntimeWriteRebasedPairPlans] using
    (wheel30RuntimeRead_first_of_cycleSlotPair (base := base₂)
      (bytes := bytes) cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle)

theorem wheel30RuntimeRead_second_of_rebasedPairPlans
    {base₁ : ℕ} (base₂ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₂ cycle₂ slot₂ hCycle₂ = 1 := by
  simpa [wheel30RuntimeWriteRebasedPairPlans] using
    (wheel30RuntimeRead_second_of_cycleSlotPair (base := base₂)
      (bytes := bytes) cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle)

theorem wheel30RuntimeReads_of_rebasedPairPlans
    {base₁ : ℕ} (base₂ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₂ cycle₁ slot₁ hCycle₁ = 1 ∧
      wheel30CandidateRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₂ cycle₂ slot₂ hCycle₂ = 1 := by
  constructor
  · exact wheel30RuntimeRead_first_of_rebasedPairPlans
      (base₁ := base₁) (base₂ := base₂) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle
  · exact wheel30RuntimeRead_second_of_rebasedPairPlans
      (base₁ := base₁) (base₂ := base₂) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle

theorem wheel30RuntimeRead_first_of_rebasedSameCyclePairPlans
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle : ℕ) (slot₁ slot₂ : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes)
    (hDistinct : slot₁ ≠ slot₂) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle slot₁ hCycle cycle slot₂ hCycle)
        base₃ cycle slot₁ hCycle = 1 := by
  exact wheel30RuntimeRead_eq_one_of_base_invariant
    (bytes := wheel30RuntimeWriteRebasedPairPlans
      (base₁ := base₁) (base₂ := base₂) bytes
      cycle slot₁ hCycle cycle slot₂ hCycle)
    (readBase₁ := base₂) (readBase₂ := base₃)
    (coord := (⟨cycle, slot₁, hCycle⟩ : Wheel30RuntimeCoord base₂))
    (by
      simpa [wheel30RuntimeWriteRebasedPairPlans] using
        (wheel30RuntimeRead_first_of_sameCyclePairPlans
          (base := base₂) (bytes := bytes) (cycle := cycle)
          (slot₁ := slot₁) (slot₂ := slot₂) (hCycle := hCycle) hDistinct))

theorem wheel30RuntimeRead_second_of_rebasedSameCyclePairPlans
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle : ℕ) (slot₁ slot₂ : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle slot₁ hCycle cycle slot₂ hCycle)
        base₃ cycle slot₂ hCycle = 1 := by
  exact wheel30RuntimeRead_eq_one_of_base_invariant
    (bytes := wheel30RuntimeWriteRebasedPairPlans
      (base₁ := base₁) (base₂ := base₂) bytes
      cycle slot₁ hCycle cycle slot₂ hCycle)
    (readBase₁ := base₂) (readBase₂ := base₃)
    (coord := (⟨cycle, slot₂, hCycle⟩ : Wheel30RuntimeCoord base₂))
    (by
      simpa [wheel30RuntimeWriteRebasedPairPlans] using
        (wheel30RuntimeRead_second_of_sameCyclePairPlans
          (base := base₂) (bytes := bytes) (cycle := cycle)
          (slot₁ := slot₁) (slot₂ := slot₂) (hCycle := hCycle)))

theorem wheel30RuntimeReads_of_rebasedSameCyclePairPlans
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle : ℕ) (slot₁ slot₂ : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes)
    (hDistinct : slot₁ ≠ slot₂) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle slot₁ hCycle cycle slot₂ hCycle)
        base₃ cycle slot₁ hCycle = 1 ∧
      wheel30CandidateRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle slot₁ hCycle cycle slot₂ hCycle)
        base₃ cycle slot₂ hCycle = 1 := by
  constructor
  · exact wheel30RuntimeRead_first_of_rebasedSameCyclePairPlans
      (base₁ := base₁) (base₂ := base₂) (base₃ := base₃)
      (bytes := bytes) (cycle := cycle) (slot₁ := slot₁) (slot₂ := slot₂)
      (hCycle := hCycle) hDistinct
  · exact wheel30RuntimeRead_second_of_rebasedSameCyclePairPlans
      (base₁ := base₁) (base₂ := base₂) (base₃ := base₃)
      (bytes := bytes) (cycle := cycle) (slot₁ := slot₁) (slot₂ := slot₂)
      (hCycle := hCycle)

theorem wheel30BoundedCycleSlotRead_first_of_rebasedSameCyclePairPlans
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle : ℕ) (slot₁ slot₂ : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes)
    (hDistinct : slot₁ ≠ slot₂) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle slot₁ hCycle cycle slot₂ hCycle)
        base₃ ⟨(cycle, slot₁), hCycle⟩ = 1 := by
  exact wheel30BoundedCycleSlotRead_eq_one_of_runtimeRead
    (bytes := wheel30RuntimeWriteRebasedPairPlans
      (base₁ := base₁) (base₂ := base₂) bytes
      cycle slot₁ hCycle cycle slot₂ hCycle)
    (base₁ := base₂) (base₂ := base₃)
    (cycleSlot := ⟨(cycle, slot₁), hCycle⟩)
    (wheel30RuntimeRead_first_of_rebasedSameCyclePairPlans
      (base₁ := base₁) (base₂ := base₂) (base₃ := base₂)
      (bytes := bytes) (cycle := cycle) (slot₁ := slot₁) (slot₂ := slot₂)
      (hCycle := hCycle) hDistinct)

theorem wheel30BoundedCycleSlotRead_second_of_rebasedSameCyclePairPlans
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle : ℕ) (slot₁ slot₂ : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle slot₁ hCycle cycle slot₂ hCycle)
        base₃ ⟨(cycle, slot₂), hCycle⟩ = 1 := by
  exact wheel30BoundedCycleSlotRead_eq_one_of_runtimeRead
    (bytes := wheel30RuntimeWriteRebasedPairPlans
      (base₁ := base₁) (base₂ := base₂) bytes
      cycle slot₁ hCycle cycle slot₂ hCycle)
    (base₁ := base₂) (base₂ := base₃)
    (cycleSlot := ⟨(cycle, slot₂), hCycle⟩)
    (wheel30RuntimeRead_second_of_rebasedSameCyclePairPlans
      (base₁ := base₁) (base₂ := base₂) (base₃ := base₂)
      (bytes := bytes) (cycle := cycle) (slot₁ := slot₁) (slot₂ := slot₂)
      (hCycle := hCycle))

theorem wheel30BoundedCycleSlotReads_of_rebasedSameCyclePairPlans
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle : ℕ) (slot₁ slot₂ : Fin 8)
    (hCycle : cycle < wheel30SegmentBytes)
    (hDistinct : slot₁ ≠ slot₂) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle slot₁ hCycle cycle slot₂ hCycle)
        base₃ ⟨(cycle, slot₁), hCycle⟩ = 1 ∧
      wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle slot₁ hCycle cycle slot₂ hCycle)
        base₃ ⟨(cycle, slot₂), hCycle⟩ = 1 := by
  constructor
  · exact wheel30BoundedCycleSlotRead_first_of_rebasedSameCyclePairPlans
      (base₁ := base₁) (base₂ := base₂) (base₃ := base₃)
      (bytes := bytes) (cycle := cycle) (slot₁ := slot₁) (slot₂ := slot₂)
      (hCycle := hCycle) hDistinct
  · exact wheel30BoundedCycleSlotRead_second_of_rebasedSameCyclePairPlans
      (base₁ := base₁) (base₂ := base₂) (base₃ := base₃)
      (bytes := bytes) (cycle := cycle) (slot₁ := slot₁) (slot₂ := slot₂)
      (hCycle := hCycle)

theorem wheel30BoundedCycleSlotRead_first_of_rebasedPairPlans
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₃ ⟨(cycle₁, slot₁), hCycle₁⟩ = 1 := by
  exact wheel30BoundedCycleSlotRead_eq_one_of_runtimeRead
    (bytes := wheel30RuntimeWriteRebasedPairPlans
      (base₁ := base₁) (base₂ := base₂) bytes
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
    (base₁ := base₂) (base₂ := base₃) (cycleSlot := ⟨(cycle₁, slot₁), hCycle₁⟩)
    (wheel30RuntimeRead_first_of_rebasedPairPlans
      (base₁ := base₁) (base₂ := base₂) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle)

theorem wheel30BoundedCycleSlotRead_second_of_rebasedPairPlans
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₃ ⟨(cycle₂, slot₂), hCycle₂⟩ = 1 := by
  exact wheel30BoundedCycleSlotRead_eq_one_of_runtimeRead
    (bytes := wheel30RuntimeWriteRebasedPairPlans
      (base₁ := base₁) (base₂ := base₂) bytes
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
    (base₁ := base₂) (base₂ := base₃) (cycleSlot := ⟨(cycle₂, slot₂), hCycle₂⟩)
    (wheel30RuntimeRead_second_of_rebasedPairPlans
      (base₁ := base₁) (base₂ := base₂) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle)

theorem wheel30BoundedCycleSlotReads_of_rebasedPairPlans
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₃ ⟨(cycle₁, slot₁), hCycle₁⟩ = 1 ∧
      wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₃ ⟨(cycle₂, slot₂), hCycle₂⟩ = 1 := by
  constructor
  · exact wheel30BoundedCycleSlotRead_first_of_rebasedPairPlans
      (base₁ := base₁) (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle
  · exact wheel30BoundedCycleSlotRead_second_of_rebasedPairPlans
      (base₁ := base₁) (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle

theorem wheel30RuntimeRead_first_of_rebasedCycleSlotPair_byByte
    {base₁ : ℕ} (base₂ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedCycleSlotPairByByte
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₂ cycle₁ slot₁ hCycle₁ = 1 := by
  let coord₁ : Wheel30RuntimeCoord base₁ := ⟨cycle₁, slot₁, hCycle₁⟩
  let coord₂ : Wheel30RuntimeCoord base₁ := ⟨cycle₂, slot₂, hCycle₂⟩
  have hRead :
      ∀ bytes (coord : Wheel30RuntimeCoord base₁),
        wheel30CandidateRead bytes base₂ coord.cycle coord.slot coord.hCycle =
          byteMarkRead bytes (wheel30RuntimeMark coord) := by
    intro bytes coord
    simpa [wheel30RuntimeMark] using
      (wheel30CandidateRead_eq_byteMarkRead
        bytes base₂ coord.cycle coord.slot coord.hCycle)
  simpa [coord₁, coord₂, wheel30RuntimeWriteRebasedCycleSlotPairByByte,
    wheel30RuntimeCycleSlotPair, Wheel30RuntimeCoord.rebase] using
    (coordRead_first_of_mappedPair_byByte_of_mark_eq
      (read := fun bytes (coord : Wheel30RuntimeCoord base₁) =>
        wheel30CandidateRead bytes base₂ coord.cycle coord.slot coord.hCycle)
      (mark₁ := wheel30RuntimeMark (base := base₁))
      (mark₂ := wheel30RuntimeMark (base := base₂))
      (hRead := hRead)
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (hMark := fun coord => wheel30RuntimeMark_rebase (base₂ := base₂) coord)
      (bytes := bytes) (coord₁ := coord₁) (coord₂ := coord₂))

theorem wheel30RuntimeRead_second_of_rebasedCycleSlotPair_byByte
    {base₁ : ℕ} (base₂ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedCycleSlotPairByByte
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₂ cycle₂ slot₂ hCycle₂ = 1 := by
  let coord₁ : Wheel30RuntimeCoord base₁ := ⟨cycle₁, slot₁, hCycle₁⟩
  let coord₂ : Wheel30RuntimeCoord base₁ := ⟨cycle₂, slot₂, hCycle₂⟩
  have hRead :
      ∀ bytes (coord : Wheel30RuntimeCoord base₁),
        wheel30CandidateRead bytes base₂ coord.cycle coord.slot coord.hCycle =
          byteMarkRead bytes (wheel30RuntimeMark coord) := by
    intro bytes coord
    simpa [wheel30RuntimeMark] using
      (wheel30CandidateRead_eq_byteMarkRead
        bytes base₂ coord.cycle coord.slot coord.hCycle)
  simpa [coord₁, coord₂, wheel30RuntimeWriteRebasedCycleSlotPairByByte,
    wheel30RuntimeCycleSlotPair, Wheel30RuntimeCoord.rebase] using
    (coordRead_second_of_mappedPair_byByte_of_mark_eq
      (read := fun bytes (coord : Wheel30RuntimeCoord base₁) =>
        wheel30CandidateRead bytes base₂ coord.cycle coord.slot coord.hCycle)
      (mark₁ := wheel30RuntimeMark (base := base₁))
      (mark₂ := wheel30RuntimeMark (base := base₂))
      (hRead := hRead)
      (f := fun coord : Wheel30RuntimeCoord base₁ => coord.rebase (base₂ := base₂))
      (hMark := fun coord => wheel30RuntimeMark_rebase (base₂ := base₂) coord)
      (bytes := bytes) (coord₁ := coord₁) (coord₂ := coord₂))

theorem wheel30RuntimeReads_of_rebasedCycleSlotPair_byByte
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedCycleSlotPairByByte
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₃ cycle₁ slot₁ hCycle₁ = 1 ∧
      wheel30CandidateRead
        (wheel30RuntimeWriteRebasedCycleSlotPairByByte
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₃ cycle₂ slot₂ hCycle₂ = 1 := by
  constructor
  · exact wheel30RuntimeRead_eq_one_of_base_invariant
      (bytes := wheel30RuntimeWriteRebasedCycleSlotPairByByte
        (base₁ := base₁) (base₂ := base₂) bytes
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
      (readBase₁ := base₂) (readBase₂ := base₃)
      (coord := (⟨cycle₁, slot₁, hCycle₁⟩ : Wheel30RuntimeCoord base₂))
      (wheel30RuntimeRead_first_of_rebasedCycleSlotPair_byByte
        (base₁ := base₁) (base₂ := base₂) (bytes := bytes)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
  · exact wheel30RuntimeRead_eq_one_of_base_invariant
      (bytes := wheel30RuntimeWriteRebasedCycleSlotPairByByte
        (base₁ := base₁) (base₂ := base₂) bytes
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
      (readBase₁ := base₂) (readBase₂ := base₃)
      (coord := (⟨cycle₂, slot₂, hCycle₂⟩ : Wheel30RuntimeCoord base₂))
      (wheel30RuntimeRead_second_of_rebasedCycleSlotPair_byByte
        (base₁ := base₁) (base₂ := base₂) (bytes := bytes)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)

theorem wheel30RuntimeRead_first_of_rebasedCycleSlotPair
    {base₁ : ℕ} (base₂ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₂ cycle₁ slot₁ hCycle₁ = 1 := by
  simpa [wheel30RuntimeWriteRebasedPairPlans] using
    (wheel30RuntimeRead_first_of_rebasedPairPlans
      (base₁ := base₁) (base₂ := base₂) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle)

theorem wheel30RuntimeRead_second_of_rebasedCycleSlotPair
    {base₁ : ℕ} (base₂ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₂ cycle₂ slot₂ hCycle₂ = 1 := by
  simpa [wheel30RuntimeWriteRebasedPairPlans] using
    (wheel30RuntimeRead_second_of_rebasedPairPlans
      (base₁ := base₁) (base₂ := base₂) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle)

theorem wheel30RuntimeReads_of_rebasedCycleSlotPair
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂) :
    wheel30CandidateRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₃ cycle₁ slot₁ hCycle₁ = 1 ∧
      wheel30CandidateRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₃ cycle₂ slot₂ hCycle₂ = 1 := by
  constructor
  · exact wheel30RuntimeRead_eq_one_of_base_invariant
      (bytes := wheel30RuntimeWriteRebasedPairPlans
        (base₁ := base₁) (base₂ := base₂) bytes
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
      (readBase₁ := base₂) (readBase₂ := base₃)
      (coord := (⟨cycle₁, slot₁, hCycle₁⟩ : Wheel30RuntimeCoord base₂))
      (wheel30RuntimeRead_first_of_rebasedCycleSlotPair
        (base₁ := base₁) (base₂ := base₂) (bytes := bytes)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle)
  · exact wheel30RuntimeRead_eq_one_of_base_invariant
      (bytes := wheel30RuntimeWriteRebasedPairPlans
        (base₁ := base₁) (base₂ := base₂) bytes
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
      (readBase₁ := base₂) (readBase₂ := base₃)
      (coord := (⟨cycle₂, slot₂, hCycle₂⟩ : Wheel30RuntimeCoord base₂))
      (wheel30RuntimeRead_second_of_rebasedCycleSlotPair
        (base₁ := base₁) (base₂ := base₂) (bytes := bytes)
        cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle)

theorem wheel30BoundedCycleSlotRead_first_of_rebasedCycleSlotPair_byByte
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteRebasedCycleSlotPairByByte
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₃ ⟨(cycle₁, slot₁), hCycle₁⟩ = 1 := by
  exact wheel30BoundedCycleSlotRead_eq_one_of_runtimeRead
    (bytes := wheel30RuntimeWriteRebasedCycleSlotPairByByte
      (base₁ := base₁) (base₂ := base₂) bytes
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
    (base₁ := base₂) (base₂ := base₃)
    (cycleSlot := ⟨(cycle₁, slot₁), hCycle₁⟩)
    (wheel30RuntimeRead_first_of_rebasedCycleSlotPair_byByte
      (base₁ := base₁) (base₂ := base₂) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)

theorem wheel30BoundedCycleSlotRead_second_of_rebasedCycleSlotPair_byByte
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteRebasedCycleSlotPairByByte
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₃ ⟨(cycle₂, slot₂), hCycle₂⟩ = 1 := by
  exact wheel30BoundedCycleSlotRead_eq_one_of_runtimeRead
    (bytes := wheel30RuntimeWriteRebasedCycleSlotPairByByte
      (base₁ := base₁) (base₂ := base₂) bytes
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
    (base₁ := base₂) (base₂ := base₃)
    (cycleSlot := ⟨(cycle₂, slot₂), hCycle₂⟩)
    (wheel30RuntimeRead_second_of_rebasedCycleSlotPair_byByte
      (base₁ := base₁) (base₂ := base₂) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)

theorem wheel30BoundedCycleSlotReads_of_rebasedCycleSlotPair_byByte
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteRebasedCycleSlotPairByByte
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₃ ⟨(cycle₁, slot₁), hCycle₁⟩ = 1 ∧
      wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteRebasedCycleSlotPairByByte
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₃ ⟨(cycle₂, slot₂), hCycle₂⟩ = 1 := by
  constructor
  · exact wheel30BoundedCycleSlotRead_first_of_rebasedCycleSlotPair_byByte
      (base₁ := base₁) (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂
  · exact wheel30BoundedCycleSlotRead_second_of_rebasedCycleSlotPair_byByte
      (base₁ := base₁) (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂

theorem wheel30BoundedCycleSlotRead_first_of_rebasedCycleSlotPair
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₃ ⟨(cycle₁, slot₁), hCycle₁⟩ = 1 := by
  exact wheel30BoundedCycleSlotRead_eq_one_of_runtimeRead
    (bytes := wheel30RuntimeWriteRebasedPairPlans
      (base₁ := base₁) (base₂ := base₂) bytes
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
    (base₁ := base₂) (base₂ := base₃)
    (cycleSlot := ⟨(cycle₁, slot₁), hCycle₁⟩)
    (wheel30RuntimeRead_first_of_rebasedCycleSlotPair
      (base₁ := base₁) (base₂ := base₂) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle)

theorem wheel30BoundedCycleSlotRead_second_of_rebasedCycleSlotPair
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₃ ⟨(cycle₂, slot₂), hCycle₂⟩ = 1 := by
  exact wheel30BoundedCycleSlotRead_eq_one_of_runtimeRead
    (bytes := wheel30RuntimeWriteRebasedPairPlans
      (base₁ := base₁) (base₂ := base₂) bytes
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
    (base₁ := base₂) (base₂ := base₃)
    (cycleSlot := ⟨(cycle₂, slot₂), hCycle₂⟩)
    (wheel30RuntimeRead_second_of_rebasedCycleSlotPair
      (base₁ := base₁) (base₂ := base₂) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle)

theorem wheel30BoundedCycleSlotReads_of_rebasedCycleSlotPair
    {base₁ : ℕ} (base₂ base₃ : ℕ)
    (bytes : Wheel30ByteState)
    (cycle₁ : ℕ) (slot₁ : Fin 8) (hCycle₁ : cycle₁ < wheel30SegmentBytes)
    (cycle₂ : ℕ) (slot₂ : Fin 8) (hCycle₂ : cycle₂ < wheel30SegmentBytes)
    (hCycle : cycle₁ ≠ cycle₂) :
    wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₃ ⟨(cycle₁, slot₁), hCycle₁⟩ = 1 ∧
      wheel30BoundedCycleSlotRead
        (wheel30RuntimeWriteRebasedPairPlans
          (base₁ := base₁) (base₂ := base₂) bytes
          cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂)
        base₃ ⟨(cycle₂, slot₂), hCycle₂⟩ = 1 := by
  constructor
  · exact wheel30BoundedCycleSlotRead_first_of_rebasedCycleSlotPair
      (base₁ := base₁) (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle
  · exact wheel30BoundedCycleSlotRead_second_of_rebasedCycleSlotPair
      (base₁ := base₁) (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      cycle₁ slot₁ hCycle₁ cycle₂ slot₂ hCycle₂ hCycle

theorem exists_wheel30BoundedCycleSlotPair_indexedReadback_byByte_of_segmentRepresentable
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n₁ n₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ n₁) (hLt₁ : n₁ < base₁ + wheel30SegmentSpan)
    (hRep₁ : wheel30Representable n₁)
    (hGe₂ : base₁ ≤ n₂) (hLt₂ : n₂ < base₁ + wheel30SegmentSpan)
    (hRep₂ : wheel30Representable n₂) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      n₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      n₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30Index base₁ n₁ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₁) ∧
      wheel30Index base₁ n₂ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₂) ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁ = 1 ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂ = 1 := by
  exact with_wheel30BoundedCycleSlotPair_of_segmentRepresentable
    (hBase := hBase)
    (hGe₁ := hGe₁) (hLt₁ := hLt₁) (hRep₁ := hRep₁)
    (hGe₂ := hGe₂) (hLt₂ := hLt₂) (hRep₂ := hRep₂)
    (fun cycleSlot₁ cycleSlot₂ hCand₁ hCand₂ hIdx₁ hIdx₂ =>
      ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, hIdx₁, hIdx₂,
        by
          simpa [wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte] using
            (wheel30BoundedCycleSlotReads_of_rebasedCycleSlotPair_byByte
              (base₁ := base₁) (base₂ := base₂) (base₃ := base₃)
              (bytes := bytes)
              cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2
              cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2)⟩)

theorem exists_wheel30BoundedCycleSlotPair_readback_byByte_of_segmentRepresentable
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n₁ n₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ n₁) (hLt₁ : n₁ < base₁ + wheel30SegmentSpan)
    (hRep₁ : wheel30Representable n₁)
    (hGe₂ : base₁ ≤ n₂) (hLt₂ : n₂ < base₁ + wheel30SegmentSpan)
    (hRep₂ : wheel30Representable n₂) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      n₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      n₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁ = 1 ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂ = 1 := by
  obtain ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, _, _, hRead⟩ :=
    exists_wheel30BoundedCycleSlotPair_indexedReadback_byByte_of_segmentRepresentable
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase)
      (hGe₁ := hGe₁) (hLt₁ := hLt₁) (hRep₁ := hRep₁)
      (hGe₂ := hGe₂) (hLt₂ := hLt₂) (hRep₂ := hRep₂)
  exact ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, hRead⟩

theorem exists_wheel30BoundedCycleSlotPair_indexedRuntimeRead_byByte_of_segmentRepresentable
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n₁ n₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ n₁) (hLt₁ : n₁ < base₁ + wheel30SegmentSpan)
    (hRep₁ : wheel30Representable n₁)
    (hGe₂ : base₁ ≤ n₂) (hLt₂ : n₂ < base₁ + wheel30SegmentSpan)
    (hRep₂ : wheel30Representable n₂) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      n₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      n₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30Index base₁ n₁ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₁) ∧
      wheel30Index base₁ n₂ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₂) ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2 = 1 ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2 = 1 := by
  obtain ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, hIdx₁, hIdx₂, _, _⟩ :=
    exists_wheel30BoundedCycleSlotPair_indexedReadback_byByte_of_segmentRepresentable
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase)
      (hGe₁ := hGe₁) (hLt₁ := hLt₁) (hRep₁ := hRep₁)
      (hGe₂ := hGe₂) (hLt₂ := hLt₂) (hRep₂ := hRep₂)
  refine ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, hIdx₁, hIdx₂, ?_⟩
  simpa [wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte] using
    (wheel30RuntimeReads_of_rebasedCycleSlotPair_byByte
      (base₁ := base₁) (base₂ := base₂) (base₃ := base₃)
      (bytes := bytes)
      cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2
      cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2)

theorem exists_wheel30BoundedCycleSlotPair_indexedReadback_of_segmentRepresentable_of_thirty_le_sub
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n₁ n₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ n₁) (hLt₁ : n₁ < base₁ + wheel30SegmentSpan)
    (hRep₁ : wheel30Representable n₁)
    (hGe₂ : base₁ ≤ n₂) (hLt₂ : n₂ < base₁ + wheel30SegmentSpan)
    (hRep₂ : wheel30Representable n₂)
    (hSep : 30 ≤ n₂ - n₁) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      n₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      n₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30Index base₁ n₁ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₁) ∧
      wheel30Index base₁ n₂ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₂) ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedPairPlans
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁ = 1 ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedPairPlans
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂ = 1 := by
  exact with_wheel30BoundedCycleSlotPair_of_segmentRepresentable
    (hBase := hBase)
    (hGe₁ := hGe₁) (hLt₁ := hLt₁) (hRep₁ := hRep₁)
    (hGe₂ := hGe₂) (hLt₂ := hLt₂) (hRep₂ := hRep₂)
    (fun cycleSlot₁ cycleSlot₂ hCand₁ hCand₂ hIdx₁ hIdx₂ =>
      let hCycle : cycleSlot₁.1.1 ≠ cycleSlot₂.1.1 := by
        exact wheel30BoundedCycleSlot_cycle_ne_of_thirty_le_sub
          (base := base₁) <| by simpa [hCand₁, hCand₂] using hSep
      ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, hIdx₁, hIdx₂,
        by
          simpa [wheel30BoundedCycleSlotPairWriteRebasedPairPlans] using
            (wheel30BoundedCycleSlotReads_of_rebasedPairPlans
              (base₁ := base₁) (base₂ := base₂) (base₃ := base₃)
              (bytes := bytes)
              cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2
              cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2 hCycle)⟩)

theorem exists_wheel30BoundedCycleSlotPair_readback_of_segmentRepresentable_of_thirty_le_sub
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n₁ n₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ n₁) (hLt₁ : n₁ < base₁ + wheel30SegmentSpan)
    (hRep₁ : wheel30Representable n₁)
    (hGe₂ : base₁ ≤ n₂) (hLt₂ : n₂ < base₁ + wheel30SegmentSpan)
    (hRep₂ : wheel30Representable n₂)
    (hSep : 30 ≤ n₂ - n₁) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      n₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      n₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedPairPlans
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁ = 1 ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedPairPlans
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂ = 1 := by
  obtain ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, _, _, hRead⟩ :=
    exists_wheel30BoundedCycleSlotPair_indexedReadback_of_segmentRepresentable_of_thirty_le_sub
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase)
      (hGe₁ := hGe₁) (hLt₁ := hLt₁) (hRep₁ := hRep₁)
      (hGe₂ := hGe₂) (hLt₂ := hLt₂) (hRep₂ := hRep₂)
      (hSep := hSep)
  exact ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, hRead⟩

theorem exists_wheel30BoundedCycleSlotPair_indexedReadback_byByte_of_segmentPrimesGtThirty
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p₁ p₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ p₁) (hLt₁ : p₁ < base₁ + wheel30SegmentSpan)
    (hPrime₁ : Nat.Prime p₁) (hGt₁ : 30 < p₁)
    (hGe₂ : base₁ ≤ p₂) (hLt₂ : p₂ < base₁ + wheel30SegmentSpan)
    (hPrime₂ : Nat.Prime p₂) (hGt₂ : 30 < p₂) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      p₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      p₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30Index base₁ p₁ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₁) ∧
      wheel30Index base₁ p₂ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₂) ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁ = 1 ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂ = 1 := by
  exact exists_wheel30BoundedCycleSlotPair_indexedReadback_byByte_of_segmentRepresentable
    (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
    (hBase := hBase)
    (hGe₁ := hGe₁) (hLt₁ := hLt₁)
    (hRep₁ := primeGtThirty_wheel30Representable hPrime₁ hGt₁)
    (hGe₂ := hGe₂) (hLt₂ := hLt₂)
    (hRep₂ := primeGtThirty_wheel30Representable hPrime₂ hGt₂)

theorem exists_wheel30BoundedCycleSlotPair_readback_byByte_of_segmentPrimesGtThirty
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p₁ p₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ p₁) (hLt₁ : p₁ < base₁ + wheel30SegmentSpan)
    (hPrime₁ : Nat.Prime p₁) (hGt₁ : 30 < p₁)
    (hGe₂ : base₁ ≤ p₂) (hLt₂ : p₂ < base₁ + wheel30SegmentSpan)
    (hPrime₂ : Nat.Prime p₂) (hGt₂ : 30 < p₂) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      p₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      p₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁ = 1 ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂ = 1 := by
  obtain ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, _, _, hRead⟩ :=
    exists_wheel30BoundedCycleSlotPair_indexedReadback_byByte_of_segmentPrimesGtThirty
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase)
      (hGe₁ := hGe₁) (hLt₁ := hLt₁) (hPrime₁ := hPrime₁) (hGt₁ := hGt₁)
      (hGe₂ := hGe₂) (hLt₂ := hLt₂) (hPrime₂ := hPrime₂) (hGt₂ := hGt₂)
  exact ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, hRead⟩

theorem exists_wheel30BoundedCycleSlotPair_indexedRuntimeRead_byByte_of_segmentPrimesGtThirty
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p₁ p₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ p₁) (hLt₁ : p₁ < base₁ + wheel30SegmentSpan)
    (hPrime₁ : Nat.Prime p₁) (hGt₁ : 30 < p₁)
    (hGe₂ : base₁ ≤ p₂) (hLt₂ : p₂ < base₁ + wheel30SegmentSpan)
    (hPrime₂ : Nat.Prime p₂) (hGt₂ : 30 < p₂) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      p₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      p₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30Index base₁ p₁ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₁) ∧
      wheel30Index base₁ p₂ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₂) ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2 = 1 ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2 = 1 := by
  exact exists_wheel30BoundedCycleSlotPair_indexedRuntimeRead_byByte_of_segmentRepresentable
    (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
    (hBase := hBase)
    (hGe₁ := hGe₁) (hLt₁ := hLt₁)
    (hRep₁ := primeGtThirty_wheel30Representable hPrime₁ hGt₁)
    (hGe₂ := hGe₂) (hLt₂ := hLt₂)
    (hRep₂ := primeGtThirty_wheel30Representable hPrime₂ hGt₂)

theorem exists_wheel30BoundedCycleSlotPair_runtimeRead_byByte_of_segmentRepresentable
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n₁ n₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ n₁) (hLt₁ : n₁ < base₁ + wheel30SegmentSpan)
    (hRep₁ : wheel30Representable n₁)
    (hGe₂ : base₁ ≤ n₂) (hLt₂ : n₂ < base₁ + wheel30SegmentSpan)
    (hRep₂ : wheel30Representable n₂) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      n₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      n₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2 = 1 ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2 = 1 := by
  obtain ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, _, _, hRead⟩ :=
    exists_wheel30BoundedCycleSlotPair_indexedRuntimeRead_byByte_of_segmentRepresentable
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase)
      (hGe₁ := hGe₁) (hLt₁ := hLt₁) (hRep₁ := hRep₁)
      (hGe₂ := hGe₂) (hLt₂ := hLt₂) (hRep₂ := hRep₂)
  exact ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, hRead⟩

theorem exists_wheel30BoundedCycleSlotPair_runtimeRead_byByte_of_segmentPrimesGtThirty
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p₁ p₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ p₁) (hLt₁ : p₁ < base₁ + wheel30SegmentSpan)
    (hPrime₁ : Nat.Prime p₁) (hGt₁ : 30 < p₁)
    (hGe₂ : base₁ ≤ p₂) (hLt₂ : p₂ < base₁ + wheel30SegmentSpan)
    (hPrime₂ : Nat.Prime p₂) (hGt₂ : 30 < p₂) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      p₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      p₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2 = 1 ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2 = 1 := by
  obtain ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, _, _, hRead⟩ :=
    exists_wheel30BoundedCycleSlotPair_indexedRuntimeRead_byByte_of_segmentPrimesGtThirty
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase)
      (hGe₁ := hGe₁) (hLt₁ := hLt₁) (hPrime₁ := hPrime₁) (hGt₁ := hGt₁)
      (hGe₂ := hGe₂) (hLt₂ := hLt₂) (hPrime₂ := hPrime₂) (hGt₂ := hGt₂)
  exact ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, hRead⟩

theorem exists_wheel30BoundedCycleSlotPair_indexedReadback_byByte_of_segmentRepresentable_of_thirty_le_sub
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n₁ n₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ n₁) (hLt₁ : n₁ < base₁ + wheel30SegmentSpan)
    (hRep₁ : wheel30Representable n₁)
    (hGe₂ : base₁ ≤ n₂) (hLt₂ : n₂ < base₁ + wheel30SegmentSpan)
    (hRep₂ : wheel30Representable n₂)
    (hSep : 30 ≤ n₂ - n₁) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      n₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      n₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30Index base₁ n₁ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₁) ∧
      wheel30Index base₁ n₂ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₂) ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁ = 1 ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂ = 1 := by
  let _ := hSep
  exact exists_wheel30BoundedCycleSlotPair_indexedReadback_byByte_of_segmentRepresentable
    (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
    (hBase := hBase)
    (hGe₁ := hGe₁) (hLt₁ := hLt₁) (hRep₁ := hRep₁)
    (hGe₂ := hGe₂) (hLt₂ := hLt₂) (hRep₂ := hRep₂)

theorem exists_wheel30BoundedCycleSlotPair_readback_byByte_of_segmentRepresentable_of_thirty_le_sub
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n₁ n₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ n₁) (hLt₁ : n₁ < base₁ + wheel30SegmentSpan)
    (hRep₁ : wheel30Representable n₁)
    (hGe₂ : base₁ ≤ n₂) (hLt₂ : n₂ < base₁ + wheel30SegmentSpan)
    (hRep₂ : wheel30Representable n₂)
    (hSep : 30 ≤ n₂ - n₁) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      n₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      n₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁ = 1 ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂ = 1 := by
  obtain ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, _, _, hRead⟩ :=
    exists_wheel30BoundedCycleSlotPair_indexedReadback_byByte_of_segmentRepresentable_of_thirty_le_sub
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase)
      (hGe₁ := hGe₁) (hLt₁ := hLt₁) (hRep₁ := hRep₁)
      (hGe₂ := hGe₂) (hLt₂ := hLt₂) (hRep₂ := hRep₂)
      (hSep := hSep)
  exact ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, hRead⟩

theorem exists_wheel30BoundedCycleSlotPair_indexedReadback_byByte_of_segmentPrimesGtThirty_of_thirty_le_sub
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p₁ p₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ p₁) (hLt₁ : p₁ < base₁ + wheel30SegmentSpan)
    (hPrime₁ : Nat.Prime p₁) (hGt₁ : 30 < p₁)
    (hGe₂ : base₁ ≤ p₂) (hLt₂ : p₂ < base₁ + wheel30SegmentSpan)
    (hPrime₂ : Nat.Prime p₂) (hGt₂ : 30 < p₂)
    (hSep : 30 ≤ p₂ - p₁) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      p₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      p₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30Index base₁ p₁ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₁) ∧
      wheel30Index base₁ p₂ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₂) ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁ = 1 ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂ = 1 := by
  exact exists_wheel30BoundedCycleSlotPair_indexedReadback_byByte_of_segmentRepresentable_of_thirty_le_sub
    (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
    (hBase := hBase)
    (hGe₁ := hGe₁) (hLt₁ := hLt₁)
    (hRep₁ := primeGtThirty_wheel30Representable hPrime₁ hGt₁)
    (hGe₂ := hGe₂) (hLt₂ := hLt₂)
    (hRep₂ := primeGtThirty_wheel30Representable hPrime₂ hGt₂)
    (hSep := hSep)

theorem exists_wheel30BoundedCycleSlotPair_readback_byByte_of_segmentPrimesGtThirty_of_thirty_le_sub
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p₁ p₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ p₁) (hLt₁ : p₁ < base₁ + wheel30SegmentSpan)
    (hPrime₁ : Nat.Prime p₁) (hGt₁ : 30 < p₁)
    (hGe₂ : base₁ ≤ p₂) (hLt₂ : p₂ < base₁ + wheel30SegmentSpan)
    (hPrime₂ : Nat.Prime p₂) (hGt₂ : 30 < p₂)
    (hSep : 30 ≤ p₂ - p₁) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      p₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      p₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁ = 1 ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂ = 1 := by
  obtain ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, _, _, hRead⟩ :=
    exists_wheel30BoundedCycleSlotPair_indexedReadback_byByte_of_segmentPrimesGtThirty_of_thirty_le_sub
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase)
      (hGe₁ := hGe₁) (hLt₁ := hLt₁) (hPrime₁ := hPrime₁) (hGt₁ := hGt₁)
      (hGe₂ := hGe₂) (hLt₂ := hLt₂) (hPrime₂ := hPrime₂) (hGt₂ := hGt₂)
      (hSep := hSep)
  exact ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, hRead⟩

theorem exists_wheel30BoundedCycleSlotPair_indexedRuntimeRead_byByte_of_segmentRepresentable_of_thirty_le_sub
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n₁ n₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ n₁) (hLt₁ : n₁ < base₁ + wheel30SegmentSpan)
    (hRep₁ : wheel30Representable n₁)
    (hGe₂ : base₁ ≤ n₂) (hLt₂ : n₂ < base₁ + wheel30SegmentSpan)
    (hRep₂ : wheel30Representable n₂)
    (hSep : 30 ≤ n₂ - n₁) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      n₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      n₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30Index base₁ n₁ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₁) ∧
      wheel30Index base₁ n₂ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₂) ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2 = 1 ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2 = 1 := by
  let _ := hSep
  exact exists_wheel30BoundedCycleSlotPair_indexedRuntimeRead_byByte_of_segmentRepresentable
    (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
    (hBase := hBase)
    (hGe₁ := hGe₁) (hLt₁ := hLt₁) (hRep₁ := hRep₁)
    (hGe₂ := hGe₂) (hLt₂ := hLt₂) (hRep₂ := hRep₂)

theorem exists_wheel30BoundedCycleSlotPair_indexedRuntimeRead_byByte_of_segmentPrimesGtThirty_of_thirty_le_sub
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p₁ p₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ p₁) (hLt₁ : p₁ < base₁ + wheel30SegmentSpan)
    (hPrime₁ : Nat.Prime p₁) (hGt₁ : 30 < p₁)
    (hGe₂ : base₁ ≤ p₂) (hLt₂ : p₂ < base₁ + wheel30SegmentSpan)
    (hPrime₂ : Nat.Prime p₂) (hGt₂ : 30 < p₂)
    (hSep : 30 ≤ p₂ - p₁) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      p₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      p₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30Index base₁ p₁ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₁) ∧
      wheel30Index base₁ p₂ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₂) ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2 = 1 ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2 = 1 := by
  exact exists_wheel30BoundedCycleSlotPair_indexedRuntimeRead_byByte_of_segmentRepresentable_of_thirty_le_sub
    (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
    (hBase := hBase)
    (hGe₁ := hGe₁) (hLt₁ := hLt₁)
    (hRep₁ := primeGtThirty_wheel30Representable hPrime₁ hGt₁)
    (hGe₂ := hGe₂) (hLt₂ := hLt₂)
    (hRep₂ := primeGtThirty_wheel30Representable hPrime₂ hGt₂)
    (hSep := hSep)

theorem exists_wheel30BoundedCycleSlotPair_runtimeRead_byByte_of_segmentRepresentable_of_thirty_le_sub
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n₁ n₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ n₁) (hLt₁ : n₁ < base₁ + wheel30SegmentSpan)
    (hRep₁ : wheel30Representable n₁)
    (hGe₂ : base₁ ≤ n₂) (hLt₂ : n₂ < base₁ + wheel30SegmentSpan)
    (hRep₂ : wheel30Representable n₂)
    (hSep : 30 ≤ n₂ - n₁) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      n₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      n₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2 = 1 ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2 = 1 := by
  obtain ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, _, _, hRead⟩ :=
    exists_wheel30BoundedCycleSlotPair_indexedRuntimeRead_byByte_of_segmentRepresentable_of_thirty_le_sub
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase)
      (hGe₁ := hGe₁) (hLt₁ := hLt₁) (hRep₁ := hRep₁)
      (hGe₂ := hGe₂) (hLt₂ := hLt₂) (hRep₂ := hRep₂)
      (hSep := hSep)
  exact ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, hRead⟩

theorem exists_wheel30BoundedCycleSlotPair_runtimeRead_byByte_of_segmentPrimesGtThirty_of_thirty_le_sub
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p₁ p₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ p₁) (hLt₁ : p₁ < base₁ + wheel30SegmentSpan)
    (hPrime₁ : Nat.Prime p₁) (hGt₁ : 30 < p₁)
    (hGe₂ : base₁ ≤ p₂) (hLt₂ : p₂ < base₁ + wheel30SegmentSpan)
    (hPrime₂ : Nat.Prime p₂) (hGt₂ : 30 < p₂)
    (hSep : 30 ≤ p₂ - p₁) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      p₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      p₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2 = 1 ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedCycleSlotPairByByte
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2 = 1 := by
  obtain ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, _, _, hRead⟩ :=
    exists_wheel30BoundedCycleSlotPair_indexedRuntimeRead_byByte_of_segmentPrimesGtThirty_of_thirty_le_sub
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase)
      (hGe₁ := hGe₁) (hLt₁ := hLt₁) (hPrime₁ := hPrime₁) (hGt₁ := hGt₁)
      (hGe₂ := hGe₂) (hLt₂ := hLt₂) (hPrime₂ := hPrime₂) (hGt₂ := hGt₂)
      (hSep := hSep)
  exact ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, hRead⟩

theorem exists_wheel30BoundedCycleSlotPair_indexedReadback_of_segmentPrimesGtThirty_of_thirty_le_sub
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p₁ p₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ p₁) (hLt₁ : p₁ < base₁ + wheel30SegmentSpan)
    (hPrime₁ : Nat.Prime p₁) (hGt₁ : 30 < p₁)
    (hGe₂ : base₁ ≤ p₂) (hLt₂ : p₂ < base₁ + wheel30SegmentSpan)
    (hPrime₂ : Nat.Prime p₂) (hGt₂ : 30 < p₂)
    (hSep : 30 ≤ p₂ - p₁) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      p₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      p₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30Index base₁ p₁ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₁) ∧
      wheel30Index base₁ p₂ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₂) ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedPairPlans
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁ = 1 ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedPairPlans
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂ = 1 := by
  exact exists_wheel30BoundedCycleSlotPair_indexedReadback_of_segmentRepresentable_of_thirty_le_sub
    (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
    (hBase := hBase)
    (hGe₁ := hGe₁) (hLt₁ := hLt₁)
    (hRep₁ := primeGtThirty_wheel30Representable hPrime₁ hGt₁)
    (hGe₂ := hGe₂) (hLt₂ := hLt₂)
    (hRep₂ := primeGtThirty_wheel30Representable hPrime₂ hGt₂)
    (hSep := hSep)

theorem exists_wheel30BoundedCycleSlotPair_readback_of_segmentPrimesGtThirty_of_thirty_le_sub
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p₁ p₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ p₁) (hLt₁ : p₁ < base₁ + wheel30SegmentSpan)
    (hPrime₁ : Nat.Prime p₁) (hGt₁ : 30 < p₁)
    (hGe₂ : base₁ ≤ p₂) (hLt₂ : p₂ < base₁ + wheel30SegmentSpan)
    (hPrime₂ : Nat.Prime p₂) (hGt₂ : 30 < p₂)
    (hSep : 30 ≤ p₂ - p₁) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      p₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      p₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedPairPlans
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁ = 1 ∧
      wheel30BoundedCycleSlotRead
          (wheel30BoundedCycleSlotPairWriteRebasedPairPlans
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂ = 1 := by
  obtain ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, _, _, hRead⟩ :=
    exists_wheel30BoundedCycleSlotPair_indexedReadback_of_segmentPrimesGtThirty_of_thirty_le_sub
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase)
      (hGe₁ := hGe₁) (hLt₁ := hLt₁) (hPrime₁ := hPrime₁) (hGt₁ := hGt₁)
      (hGe₂ := hGe₂) (hLt₂ := hLt₂) (hPrime₂ := hPrime₂) (hGt₂ := hGt₂)
      (hSep := hSep)
  exact ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, hRead⟩

/-- Direct executable readback for a separated representable wheel30 pair. -/
theorem exists_wheel30BoundedCycleSlotPair_indexedRuntimeRead_of_segmentRepresentable_of_thirty_le_sub
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n₁ n₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ n₁) (hLt₁ : n₁ < base₁ + wheel30SegmentSpan)
    (hRep₁ : wheel30Representable n₁)
    (hGe₂ : base₁ ≤ n₂) (hLt₂ : n₂ < base₁ + wheel30SegmentSpan)
    (hRep₂ : wheel30Representable n₂)
    (hSep : 30 ≤ n₂ - n₁) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      n₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      n₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30Index base₁ n₁ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₁) ∧
      wheel30Index base₁ n₂ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₂) ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedPairPlans
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2 = 1 ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedPairPlans
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2 = 1 := by
  obtain ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, hIdx₁, hIdx₂, _, _⟩ :=
    exists_wheel30BoundedCycleSlotPair_indexedReadback_of_segmentRepresentable_of_thirty_le_sub
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase)
      (hGe₁ := hGe₁) (hLt₁ := hLt₁) (hRep₁ := hRep₁)
      (hGe₂ := hGe₂) (hLt₂ := hLt₂) (hRep₂ := hRep₂)
      (hSep := hSep)
  have hCycle :
      cycleSlot₁.1.1 ≠ cycleSlot₂.1.1 := by
    exact wheel30BoundedCycleSlot_cycle_ne_of_thirty_le_sub
      (base := base₁) <| by simpa [hCand₁, hCand₂] using hSep
  refine ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, hIdx₁, hIdx₂, ?_⟩
  simpa [wheel30BoundedCycleSlotPairWriteRebasedPairPlans] using
    (wheel30RuntimeReads_of_rebasedCycleSlotPair
      (base₁ := base₁) (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2
      cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2 hCycle)

/-- Direct executable readback for a separated pair of primes greater than `30`. -/
theorem exists_wheel30BoundedCycleSlotPair_indexedRuntimeRead_of_segmentPrimesGtThirty_of_thirty_le_sub
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p₁ p₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ p₁) (hLt₁ : p₁ < base₁ + wheel30SegmentSpan)
    (hPrime₁ : Nat.Prime p₁) (hGt₁ : 30 < p₁)
    (hGe₂ : base₁ ≤ p₂) (hLt₂ : p₂ < base₁ + wheel30SegmentSpan)
    (hPrime₂ : Nat.Prime p₂) (hGt₂ : 30 < p₂)
    (hSep : 30 ≤ p₂ - p₁) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      p₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      p₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30Index base₁ p₁ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₁) ∧
      wheel30Index base₁ p₂ = some (wheel30BoundedCycleSlotLinearIndex cycleSlot₂) ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedPairPlans
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2 = 1 ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedPairPlans
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2 = 1 := by
  exact exists_wheel30BoundedCycleSlotPair_indexedRuntimeRead_of_segmentRepresentable_of_thirty_le_sub
    (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
    (hBase := hBase)
    (hGe₁ := hGe₁) (hLt₁ := hLt₁)
    (hRep₁ := primeGtThirty_wheel30Representable hPrime₁ hGt₁)
    (hGe₂ := hGe₂) (hLt₂ := hLt₂)
    (hRep₂ := primeGtThirty_wheel30Representable hPrime₂ hGt₂)
    (hSep := hSep)

theorem exists_wheel30BoundedCycleSlotPair_runtimeRead_of_segmentRepresentable_of_thirty_le_sub
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {n₁ n₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ n₁) (hLt₁ : n₁ < base₁ + wheel30SegmentSpan)
    (hRep₁ : wheel30Representable n₁)
    (hGe₂ : base₁ ≤ n₂) (hLt₂ : n₂ < base₁ + wheel30SegmentSpan)
    (hRep₂ : wheel30Representable n₂)
    (hSep : 30 ≤ n₂ - n₁) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      n₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      n₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedPairPlans
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2 = 1 ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedPairPlans
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2 = 1 := by
  obtain ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, _, _, hRead⟩ :=
    exists_wheel30BoundedCycleSlotPair_indexedRuntimeRead_of_segmentRepresentable_of_thirty_le_sub
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase)
      (hGe₁ := hGe₁) (hLt₁ := hLt₁) (hRep₁ := hRep₁)
      (hGe₂ := hGe₂) (hLt₂ := hLt₂) (hRep₂ := hRep₂)
      (hSep := hSep)
  exact ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, hRead⟩

theorem exists_wheel30BoundedCycleSlotPair_runtimeRead_of_segmentPrimesGtThirty_of_thirty_le_sub
    {base₁ : ℕ} (base₂ base₃ : ℕ) (bytes : Wheel30ByteState)
    {p₁ p₂ : ℕ} (hBase : base₁ % 30 = 0)
    (hGe₁ : base₁ ≤ p₁) (hLt₁ : p₁ < base₁ + wheel30SegmentSpan)
    (hPrime₁ : Nat.Prime p₁) (hGt₁ : 30 < p₁)
    (hGe₂ : base₁ ≤ p₂) (hLt₂ : p₂ < base₁ + wheel30SegmentSpan)
    (hPrime₂ : Nat.Prime p₂) (hGt₂ : 30 < p₂)
    (hSep : 30 ≤ p₂ - p₁) :
    ∃ cycleSlot₁ cycleSlot₂ : Wheel30BoundedCycleSlot,
      p₁ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₁ ∧
      p₂ = wheel30BoundedCycleSlotCandidate base₁ cycleSlot₂ ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedPairPlans
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₁.1.1 cycleSlot₁.1.2 cycleSlot₁.2 = 1 ∧
      wheel30CandidateRead
          (wheel30BoundedCycleSlotPairWriteRebasedPairPlans
            (base₁ := base₁) (base₂ := base₂) bytes cycleSlot₁ cycleSlot₂)
          base₃ cycleSlot₂.1.1 cycleSlot₂.1.2 cycleSlot₂.2 = 1 := by
  obtain ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, _, _, hRead⟩ :=
    exists_wheel30BoundedCycleSlotPair_indexedRuntimeRead_of_segmentPrimesGtThirty_of_thirty_le_sub
      (base₂ := base₂) (base₃ := base₃) (bytes := bytes)
      (hBase := hBase)
      (hGe₁ := hGe₁) (hLt₁ := hLt₁) (hPrime₁ := hPrime₁) (hGt₁ := hGt₁)
      (hGe₂ := hGe₂) (hLt₂ := hLt₂) (hPrime₂ := hPrime₂) (hGt₂ := hGt₂)
      (hSep := hSep)
  exact ⟨cycleSlot₁, cycleSlot₂, hCand₁, hCand₂, hRead⟩

end PrimeArithmetic.Sieve
