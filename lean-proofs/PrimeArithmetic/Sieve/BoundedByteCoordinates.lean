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
- when a proof starts from membership in an already-mapped grouped family, a
  left inverse transports the plan witness and the local coordinate witness
  back to the original family

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

/-- Remap the payload of a grouped coordinate plan without changing its slot. -/
def CoordinatePlan.map {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (f : Coord₁ → Coord₂) (plan : CoordinatePlan Coord₁ byteCount) :
    CoordinatePlan Coord₂ byteCount :=
  (plan.1, plan.2.map f)

@[simp] theorem CoordinatePlan.map_fst {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (f : Coord₁ → Coord₂) (plan : CoordinatePlan Coord₁ byteCount) :
    (CoordinatePlan.map f plan).1 = plan.1 := by
  rfl

@[simp] theorem CoordinatePlan.map_snd {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (f : Coord₁ → Coord₂) (plan : CoordinatePlan Coord₁ byteCount) :
    (CoordinatePlan.map f plan).2 = plan.2.map f := by
  rfl

theorem CoordinatePlan.mem_map_snd {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (f : Coord₁ → Coord₂) (plan : CoordinatePlan Coord₁ byteCount)
    {coord : Coord₁} (hCoord : coord ∈ plan.2) :
    f coord ∈ (CoordinatePlan.map f plan).2 := by
  simpa [CoordinatePlan.map] using List.mem_map.mpr ⟨coord, hCoord, rfl⟩

theorem list_mem_of_mem_map_of_leftInverse {α β : Type}
    (f : α → β) (g : β → α) (hLeft : Function.LeftInverse g f)
    (xs : List α) {y : β} (hy : y ∈ xs.map f) :
    g y ∈ xs := by
  rcases List.mem_map.mp hy with ⟨x, hx, hEq⟩
  have hEq' : g y = x := by
    rw [← hEq]
    exact hLeft x
  simpa [hEq'] using hx

@[simp] theorem list_mem_map_iff_of_leftInverse {α β : Type}
    (f : α → β) (g : β → α) (hLeft : Function.LeftInverse g f)
    (xs : List α) {x : α} :
    f x ∈ xs.map f ↔ x ∈ xs := by
  constructor
  · intro hx
    simpa [hLeft x] using
      (list_mem_of_mem_map_of_leftInverse f g hLeft xs
        (y := f x) hx)
  · intro hx
    exact List.mem_map.mpr ⟨x, hx, rfl⟩

@[simp] theorem list_mem_map_iff_of_injective {α β : Type}
    (f : α → β) (hInj : Function.Injective f)
    (xs : List α) {x : α} :
    f x ∈ xs.map f ↔ x ∈ xs := by
  constructor
  · intro hx
    rcases List.mem_map.mp hx with ⟨y, hy, hEq⟩
    have hxy : y = x := hInj hEq
    simpa [hxy] using hy
  · intro hx
    exact List.mem_map.mpr ⟨x, hx, rfl⟩

theorem CoordinatePlan.map_leftInverse {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (f : Coord₁ → Coord₂) (g : Coord₂ → Coord₁)
    (hLeft : Function.LeftInverse g f) :
    Function.LeftInverse
      (fun plan : CoordinatePlan Coord₂ byteCount => CoordinatePlan.map g plan)
      (fun plan : CoordinatePlan Coord₁ byteCount => CoordinatePlan.map f plan) := by
  intro plan
  cases plan with
  | mk slot coords =>
      simp [CoordinatePlan.map, List.map_map]
      induction coords with
      | nil =>
          rfl
      | cons coord coords ih =>
          simp [hLeft coord, ih]

theorem CoordinatePlan.map_injective {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (f : Coord₁ → Coord₂) (g : Coord₂ → Coord₁)
    (hLeft : Function.LeftInverse g f) :
    Function.Injective
      (fun plan : CoordinatePlan Coord₁ byteCount => CoordinatePlan.map f plan) := by
  exact (CoordinatePlan.map_leftInverse f g hLeft).injective

theorem CoordinatePlan.map_surjective {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (f : Coord₁ → Coord₂) (g : Coord₂ → Coord₁)
    (hRight : Function.LeftInverse f g) :
    Function.Surjective
      (fun plan : CoordinatePlan Coord₁ byteCount => CoordinatePlan.map f plan) := by
  intro plan
  exact ⟨CoordinatePlan.map g plan, CoordinatePlan.map_leftInverse g f hRight plan⟩

theorem CoordinatePlan.map_bijective {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (f : Coord₁ → Coord₂) (g : Coord₂ → Coord₁)
    (hLeft : Function.LeftInverse g f)
    (hRight : Function.LeftInverse f g) :
    Function.Bijective
      (fun plan : CoordinatePlan Coord₁ byteCount => CoordinatePlan.map f plan) := by
  exact ⟨
    CoordinatePlan.map_injective f g hLeft,
    CoordinatePlan.map_surjective f g hRight
  ⟩

@[simp] theorem CoordinatePlan.map_eq_iff_of_inverse
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (f : Coord₁ → Coord₂) (g : Coord₂ → Coord₁)
    (hLeft : Function.LeftInverse g f)
    (hRight : Function.LeftInverse f g)
    {plan₁ : CoordinatePlan Coord₁ byteCount} {plan₂ : CoordinatePlan Coord₂ byteCount} :
    CoordinatePlan.map f plan₁ = plan₂ ↔
      plan₁ = CoordinatePlan.map g plan₂ := by
  constructor
  · intro h
    have h' :
        CoordinatePlan.map g (CoordinatePlan.map f plan₁) =
          CoordinatePlan.map g plan₂ := by
      simpa using
        congrArg
          (fun plan : CoordinatePlan Coord₂ byteCount => CoordinatePlan.map g plan)
          h
    calc
      plan₁ = CoordinatePlan.map g (CoordinatePlan.map f plan₁) := by
        symm
        exact CoordinatePlan.map_leftInverse f g hLeft plan₁
      _ = CoordinatePlan.map g plan₂ := h'
  · intro h
    have h' :
        CoordinatePlan.map f plan₁ =
          CoordinatePlan.map f (CoordinatePlan.map g plan₂) := by
      simpa using
        congrArg
          (fun plan : CoordinatePlan Coord₁ byteCount => CoordinatePlan.map f plan)
          h
    calc
      CoordinatePlan.map f plan₁ =
          CoordinatePlan.map f (CoordinatePlan.map g plan₂) := h'
      _ = plan₂ := by
        simpa using CoordinatePlan.map_leftInverse g f hRight plan₂

theorem CoordinatePlan.mem_of_mem_map_snd {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (f : Coord₁ → Coord₂) (g : Coord₂ → Coord₁)
    (hLeft : Function.LeftInverse g f)
    (plan : CoordinatePlan Coord₁ byteCount)
    {coord : Coord₂} (hCoord : coord ∈ (CoordinatePlan.map f plan).2) :
    g coord ∈ plan.2 := by
  rw [CoordinatePlan.map_snd] at hCoord
  exact list_mem_of_mem_map_of_leftInverse f g hLeft plan.2 hCoord

@[simp] theorem CoordinatePlan.mem_map_snd_iff
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (f : Coord₁ → Coord₂) (g : Coord₂ → Coord₁)
    (hLeft : Function.LeftInverse g f)
    (plan : CoordinatePlan Coord₁ byteCount)
    {coord : Coord₁} :
    f coord ∈ (CoordinatePlan.map f plan).2 ↔ coord ∈ plan.2 := by
  rw [CoordinatePlan.map_snd]
  exact list_mem_map_iff_of_leftInverse f g hLeft plan.2

@[simp] theorem CoordinatePlan.mem_map_snd_iff_of_injective
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (f : Coord₁ → Coord₂) (hInj : Function.Injective f)
    (plan : CoordinatePlan Coord₁ byteCount)
    {coord : Coord₁} :
    f coord ∈ (CoordinatePlan.map f plan).2 ↔ coord ∈ plan.2 := by
  rw [CoordinatePlan.map_snd]
  exact list_mem_map_iff_of_injective f hInj plan.2

@[simp] theorem CoordinatePlan.map_id {Coord : Type} {byteCount : ℕ}
    (plan : CoordinatePlan Coord byteCount) :
    CoordinatePlan.map (fun coord => coord) plan = plan := by
  cases plan with
  | mk slot coords =>
      simp [CoordinatePlan.map]

@[simp] theorem CoordinatePlan.map_map {Coord₁ Coord₂ Coord₃ : Type} {byteCount : ℕ}
    (f : Coord₁ → Coord₂) (g : Coord₂ → Coord₃)
    (plan : CoordinatePlan Coord₁ byteCount) :
    CoordinatePlan.map g (CoordinatePlan.map f plan) =
      CoordinatePlan.map (fun coord => g (f coord)) plan := by
  cases plan with
  | mk slot coords =>
      simp [CoordinatePlan.map, List.map_map]

@[simp] theorem CoordinatePlan.map_singletonCoordinatePlan
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (coord : Coord₁) :
    CoordinatePlan.map f (singletonCoordinatePlan mark₁ coord) =
      singletonCoordinatePlan mark₂ (f coord) := by
  simp [singletonCoordinatePlan, CoordinatePlan.map, hMark coord]

@[simp] theorem coordinatePlanPair_map_eq_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (coord₁ coord₂ : Coord₁) :
    (coordinatePlanPair mark₁ coord₁ coord₂).map (fun plan => CoordinatePlan.map f plan) =
      coordinatePlanPair mark₂ (f coord₁) (f coord₂) := by
  simp [coordinatePlanPair]
  constructor
  · exact CoordinatePlan.map_singletonCoordinatePlan mark₁ mark₂ f hMark coord₁
  · exact CoordinatePlan.map_singletonCoordinatePlan mark₁ mark₂ f hMark coord₂

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

theorem coordinatePlanBucket_map_eq_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (coords : List Coord₁)
    (slot : Fin byteCount) :
    CoordinatePlan.map f (coordinatePlanBucket mark₁ coords slot) =
      coordinatePlanBucket mark₂ (coords.map f) slot := by
  induction coords with
  | nil =>
      rfl
  | cons coord coords ih =>
      have ih' :
          List.map f (List.filter (fun coord => decide ((mark₁ coord).1 = slot)) coords) =
            List.filter (fun coord => decide ((mark₂ coord).1 = slot)) (List.map f coords) := by
        exact congrArg Prod.snd ih
      by_cases hCoord : (mark₁ coord).1 = slot
      · have hCoord' : (mark₂ (f coord)).1 = slot := by
          simpa [hMark coord] using hCoord
        simp [coordinatePlanBucket, CoordinatePlan.map, hCoord, hCoord', ih']
      · have hCoord' : ¬ (mark₂ (f coord)).1 = slot := by
          simpa [hMark coord] using hCoord
        simp [coordinatePlanBucket, CoordinatePlan.map, hCoord, hCoord', ih']

theorem coordinatePlanToBytePlan_map_eq_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (plan : CoordinatePlan Coord₁ byteCount) :
    coordinatePlanToBytePlan mark₁ plan =
      coordinatePlanToBytePlan mark₂ (CoordinatePlan.map f plan) := by
  cases plan with
  | mk slot coords =>
      simp [CoordinatePlan.map, coordinatePlanToBytePlan, List.map_map, hMark]

theorem coordinatePlans_map_toBytePlans_eq_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (plans : List (CoordinatePlan Coord₁ byteCount)) :
    plans.map (coordinatePlanToBytePlan mark₁) =
      (plans.map fun plan => CoordinatePlan.map f plan).map
        (coordinatePlanToBytePlan mark₂) := by
  induction plans with
  | nil =>
      rfl
  | cons plan plans ih =>
      simp only [List.map_cons]
      rw [coordinatePlanToBytePlan_map_eq_of_mark_eq mark₁ mark₂ f hMark plan, ih]

theorem coordinatePlan_mem_mappedPlans {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (f : Coord₁ → Coord₂)
    (plans : List (CoordinatePlan Coord₁ byteCount))
    {plan : CoordinatePlan Coord₁ byteCount} (hPlan : plan ∈ plans) :
    CoordinatePlan.map f plan ∈ plans.map (fun plan => CoordinatePlan.map f plan) := by
  exact List.mem_map.mpr ⟨plan, hPlan, rfl⟩

theorem coordinatePlan_mem_of_mem_mappedPlans {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (f : Coord₁ → Coord₂) (g : Coord₂ → Coord₁)
    (hLeft : Function.LeftInverse g f)
    (plans : List (CoordinatePlan Coord₁ byteCount))
    {plan : CoordinatePlan Coord₂ byteCount}
    (hPlan : plan ∈ plans.map (fun plan => CoordinatePlan.map f plan)) :
    CoordinatePlan.map g plan ∈ plans := by
  exact list_mem_of_mem_map_of_leftInverse
    (fun plan : CoordinatePlan Coord₁ byteCount => CoordinatePlan.map f plan)
    (fun plan : CoordinatePlan Coord₂ byteCount => CoordinatePlan.map g plan)
    (CoordinatePlan.map_leftInverse f g hLeft) plans hPlan

@[simp] theorem coordinatePlan_mem_mappedPlans_iff
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (f : Coord₁ → Coord₂) (g : Coord₂ → Coord₁)
    (hLeft : Function.LeftInverse g f)
    (plans : List (CoordinatePlan Coord₁ byteCount))
    {plan : CoordinatePlan Coord₁ byteCount} :
    CoordinatePlan.map f plan ∈ plans.map (fun plan => CoordinatePlan.map f plan) ↔
      plan ∈ plans := by
  simpa using
    (list_mem_map_iff_of_leftInverse
      (fun plan : CoordinatePlan Coord₁ byteCount => CoordinatePlan.map f plan)
      (fun plan : CoordinatePlan Coord₂ byteCount => CoordinatePlan.map g plan)
      (CoordinatePlan.map_leftInverse f g hLeft)
      plans
      (x := plan))

theorem coordinatePlan_preimage_membership_of_mem_mappedPlans
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (f : Coord₁ → Coord₂) (g : Coord₂ → Coord₁)
    (hLeft : Function.LeftInverse g f)
    (plans : List (CoordinatePlan Coord₁ byteCount))
    {plan : CoordinatePlan Coord₂ byteCount}
    (hPlan : plan ∈ plans.map (fun plan => CoordinatePlan.map f plan))
    {coord : Coord₂} (hCoord : coord ∈ plan.2) :
    CoordinatePlan.map g plan ∈ plans ∧
      g coord ∈ (CoordinatePlan.map g plan).2 := by
  constructor
  · exact coordinatePlan_mem_of_mem_mappedPlans
      (f := f) (g := g) (hLeft := hLeft) (plans := plans) hPlan
  · exact CoordinatePlan.mem_map_snd (f := g) (plan := plan) hCoord

theorem coordinatePlanWriteMany_eq_mappedPlans_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (bytes : BoundedByteState byteCount)
    (plans : List (CoordinatePlan Coord₁ byteCount)) :
    coordinatePlanWriteMany mark₁ bytes plans =
      coordinatePlanWriteMany mark₂ bytes
        (plans.map fun plan => CoordinatePlan.map f plan) := by
  unfold coordinatePlanWriteMany
  rw [coordinatePlans_map_toBytePlans_eq_of_mark_eq mark₁ mark₂ f hMark plans]

theorem coordinatePlans_map_aligned_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (plans : List (CoordinatePlan Coord₁ byteCount))
    (hAligned : ∀ plan ∈ plans, ∀ coord ∈ plan.2, (mark₁ coord).1 = plan.1) :
    ∀ plan ∈ plans.map (fun plan => CoordinatePlan.map f plan),
      ∀ coord ∈ plan.2, (mark₂ coord).1 = plan.1 := by
  intro plan' hPlan' coord' hCoord'
  rcases List.mem_map.mp hPlan' with ⟨plan, hPlan, rfl⟩
  rcases List.mem_map.mp hCoord' with ⟨coord, hCoord, rfl⟩
  simpa [CoordinatePlan.map, hMark coord] using hAligned plan hPlan coord hCoord

theorem coordinatePlans_aligned_of_mapped_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (plans : List (CoordinatePlan Coord₁ byteCount))
    (hAligned :
      ∀ plan ∈ plans.map (fun plan => CoordinatePlan.map f plan),
        ∀ coord ∈ plan.2, (mark₂ coord).1 = plan.1) :
    ∀ plan ∈ plans, ∀ coord ∈ plan.2, (mark₁ coord).1 = plan.1 := by
  intro plan hPlan coord hCoord
  have hPlanMapped :
      CoordinatePlan.map f plan ∈ plans.map (fun plan => CoordinatePlan.map f plan) := by
    exact coordinatePlan_mem_mappedPlans (f := f) (plans := plans) hPlan
  have hCoordMapped : f coord ∈ (CoordinatePlan.map f plan).2 := by
    exact CoordinatePlan.mem_map_snd (f := f) (plan := plan) hCoord
  simpa [CoordinatePlan.map, hMark coord] using
    hAligned (CoordinatePlan.map f plan) hPlanMapped (f coord) hCoordMapped

theorem coordinatePlans_mapped_aligned_iff_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (plans : List (CoordinatePlan Coord₁ byteCount)) :
    (∀ plan ∈ plans.map (fun plan => CoordinatePlan.map f plan),
        ∀ coord ∈ plan.2, (mark₂ coord).1 = plan.1) ↔
      (∀ plan ∈ plans, ∀ coord ∈ plan.2, (mark₁ coord).1 = plan.1) := by
  constructor
  · intro hAligned
    exact coordinatePlans_aligned_of_mapped_of_mark_eq
      (mark₁ := mark₁) (mark₂ := mark₂) (f := f) (hMark := hMark)
      (plans := plans) hAligned
  · intro hAligned
    exact coordinatePlans_map_aligned_of_mark_eq
      (mark₁ := mark₁) (mark₂ := mark₂) (f := f) (hMark := hMark)
      (plans := plans) hAligned

theorem coordinatePlans_map_distinctByteSlots_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (plans : List (CoordinatePlan Coord₁ byteCount))
    (hDistinct : coordinatePlansHaveDistinctByteSlots mark₁ plans) :
    coordinatePlansHaveDistinctByteSlots mark₂
      (plans.map fun plan => CoordinatePlan.map f plan) := by
  unfold coordinatePlansHaveDistinctByteSlots at hDistinct ⊢
  exact coordinatePlans_map_toBytePlans_eq_of_mark_eq mark₁ mark₂ f hMark plans ▸ hDistinct

theorem coordinatePlans_distinctByteSlots_of_mapped_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (plans : List (CoordinatePlan Coord₁ byteCount))
    (hDistinct : coordinatePlansHaveDistinctByteSlots mark₂
      (plans.map fun plan => CoordinatePlan.map f plan)) :
    coordinatePlansHaveDistinctByteSlots mark₁ plans := by
  unfold coordinatePlansHaveDistinctByteSlots at hDistinct ⊢
  exact (coordinatePlans_map_toBytePlans_eq_of_mark_eq mark₁ mark₂ f hMark plans).symm ▸
    hDistinct

theorem coordinatePlans_mapped_distinctByteSlots_iff_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (plans : List (CoordinatePlan Coord₁ byteCount)) :
    coordinatePlansHaveDistinctByteSlots mark₂
        (plans.map fun plan => CoordinatePlan.map f plan) ↔
      coordinatePlansHaveDistinctByteSlots mark₁ plans := by
  constructor
  · intro hDistinct
    exact coordinatePlans_distinctByteSlots_of_mapped_of_mark_eq
      (mark₁ := mark₁) (mark₂ := mark₂) (f := f) (hMark := hMark)
      (plans := plans) hDistinct
  · intro hDistinct
    exact coordinatePlans_map_distinctByteSlots_of_mark_eq
      (mark₁ := mark₁) (mark₂ := mark₂) (f := f) (hMark := hMark)
      (plans := plans) hDistinct

theorem coordinatePlansByByte_map_eq_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (coords : List Coord₁) :
    (coordinatePlansByByte mark₁ coords).map (CoordinatePlan.map f) =
      coordinatePlansByByte mark₂ (coords.map f) := by
  unfold coordinatePlansByByte
  apply List.ext_getElem?
  intro i
  rw [List.getElem?_map, List.getElem?_map]
  cases hSlot : (List.finRange byteCount)[i]? with
  | none =>
      simp [hSlot]
  | some slot =>
      simp [hSlot]
      exact coordinatePlanBucket_map_eq_of_mark_eq mark₁ mark₂ f hMark coords slot

theorem coordinatePlanWriteMany_coordinatePlansByByte_eq_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (bytes : BoundedByteState byteCount) (coords : List Coord₁) :
    coordinatePlanWriteMany mark₁ bytes (coordinatePlansByByte mark₁ coords) =
      coordinatePlanWriteMany mark₂ bytes (coordinatePlansByByte mark₂ (coords.map f)) := by
  rw [← coordinatePlansByByte_map_eq_of_mark_eq mark₁ mark₂ f hMark coords]
  exact coordinatePlanWriteMany_eq_mappedPlans_of_mark_eq
    mark₁ mark₂ f hMark bytes (coordinatePlansByByte mark₁ coords)

theorem coordRead_mappedPlans_eq_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (read₁ : BoundedByteState byteCount → Coord₁ → ℕ)
    (read₂ : BoundedByteState byteCount → Coord₂ → ℕ)
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (hRead₁ : ∀ bytes coord, read₁ bytes coord = byteMarkRead bytes (mark₁ coord))
    (hRead₂ : ∀ bytes coord, read₂ bytes coord = byteMarkRead bytes (mark₂ coord))
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (bytes : BoundedByteState byteCount)
    (plans : List (CoordinatePlan Coord₁ byteCount))
    (coord : Coord₁) :
    read₁ (coordinateWriteMany mark₁ bytes plans) coord =
      read₂ (coordinateWriteMany mark₂ bytes
        (plans.map fun plan => CoordinatePlan.map f plan)) (f coord) := by
  rw [coordinateWriteMany, coordinateWriteMany,
    ← coordinatePlanWriteMany_eq_mappedPlans_of_mark_eq
      mark₁ mark₂ f hMark bytes plans]
  rw [hRead₁, hRead₂, hMark coord]

theorem coordRead_mappedPlans_eq_iff_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (read₁ : BoundedByteState byteCount → Coord₁ → ℕ)
    (read₂ : BoundedByteState byteCount → Coord₂ → ℕ)
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (hRead₁ : ∀ bytes coord, read₁ bytes coord = byteMarkRead bytes (mark₁ coord))
    (hRead₂ : ∀ bytes coord, read₂ bytes coord = byteMarkRead bytes (mark₂ coord))
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (bytes : BoundedByteState byteCount)
    (plans : List (CoordinatePlan Coord₁ byteCount))
    (coord : Coord₁) (target : ℕ) :
    read₁ (coordinateWriteMany mark₁ bytes plans) coord = target ↔
      read₂ (coordinateWriteMany mark₂ bytes
        (plans.map fun plan => CoordinatePlan.map f plan)) (f coord) = target := by
  rw [coordRead_mappedPlans_eq_of_mark_eq
    read₁ read₂ mark₁ mark₂ hRead₁ hRead₂ f hMark bytes plans coord]

theorem coordRead_coordinatePlansByByte_eq_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (read₁ : BoundedByteState byteCount → Coord₁ → ℕ)
    (read₂ : BoundedByteState byteCount → Coord₂ → ℕ)
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (hRead₁ : ∀ bytes coord, read₁ bytes coord = byteMarkRead bytes (mark₁ coord))
    (hRead₂ : ∀ bytes coord, read₂ bytes coord = byteMarkRead bytes (mark₂ coord))
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (bytes : BoundedByteState byteCount)
    (coords : List Coord₁)
    (coord : Coord₁) :
    read₁
        (coordinatePlanWriteMany mark₁ bytes
          (coordinatePlansByByte mark₁ coords))
        coord =
      read₂
        (coordinatePlanWriteMany mark₂ bytes
          (coordinatePlansByByte mark₂ (coords.map f)))
        (f coord) := by
  rw [← coordinatePlanWriteMany_coordinatePlansByByte_eq_of_mark_eq
    mark₁ mark₂ f hMark bytes coords]
  rw [hRead₁, hRead₂, hMark coord]

theorem coordRead_coordinatePlansByByte_eq_iff_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (read₁ : BoundedByteState byteCount → Coord₁ → ℕ)
    (read₂ : BoundedByteState byteCount → Coord₂ → ℕ)
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (hRead₁ : ∀ bytes coord, read₁ bytes coord = byteMarkRead bytes (mark₁ coord))
    (hRead₂ : ∀ bytes coord, read₂ bytes coord = byteMarkRead bytes (mark₂ coord))
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (bytes : BoundedByteState byteCount)
    (coords : List Coord₁)
    (coord : Coord₁) (target : ℕ) :
    read₁
        (coordinatePlanWriteMany mark₁ bytes
          (coordinatePlansByByte mark₁ coords))
        coord = target ↔
      read₂
        (coordinatePlanWriteMany mark₂ bytes
          (coordinatePlansByByte mark₂ (coords.map f)))
        (f coord) = target := by
  rw [coordRead_coordinatePlansByByte_eq_of_mark_eq
    read₁ read₂ mark₁ mark₂ hRead₁ hRead₂ f hMark bytes coords coord]

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

theorem read_of_mem_coordinatePlansByByte_of_eq {Coord : Type} {byteCount : ℕ}
    (mark : Coord → ByteMark byteCount)
    (read : BoundedByteState byteCount → Coord → ℕ)
    (bytes : BoundedByteState byteCount) (coords : List Coord)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark coord))
    {coord : Coord} (hCoord : coord ∈ coords) :
    read (coordinatePlanWriteMany mark bytes (coordinatePlansByByte mark coords)) coord = 1 := by
  rw [hRead]
  exact byteMarkRead_of_mem_coordinatePlansByByte mark bytes coords hCoord

theorem read_of_mem_coordinatePlansByByte_mapped_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (read : BoundedByteState byteCount → Coord₁ → ℕ)
    (f : Coord₁ → Coord₂)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark₁ coord))
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (bytes : BoundedByteState byteCount) (coords : List Coord₁)
    {coord : Coord₁} (hCoord : coord ∈ coords) :
    read
        (coordinatePlanWriteMany mark₂ bytes
          (coordinatePlansByByte mark₂ (coords.map f)))
        coord = 1 := by
  rw [← coordinatePlanWriteMany_coordinatePlansByByte_eq_of_mark_eq
    mark₁ mark₂ f hMark bytes coords]
  exact read_of_mem_coordinatePlansByByte_of_eq mark₁ read bytes coords hRead hCoord

theorem coordRead_singleton_byByte {Coord : Type} {byteCount : ℕ}
    (read : BoundedByteState byteCount → Coord → ℕ)
    (mark : Coord → ByteMark byteCount)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark coord))
    (bytes : BoundedByteState byteCount) (coord : Coord) :
    read (coordinatePlanWriteMany mark bytes (coordinatePlansByByte mark [coord])) coord = 1 := by
  simpa using
    (read_of_mem_coordinatePlansByByte_of_eq
      (mark := mark) (read := read) (bytes := bytes) (coords := [coord])
      (hRead := hRead) (coord := coord) (by simp))

theorem coordRead_mappedSingleton_byByte_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (read : BoundedByteState byteCount → Coord₁ → ℕ)
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark₁ coord))
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (bytes : BoundedByteState byteCount) (coord : Coord₁) :
    read (coordinatePlanWriteMany mark₂ bytes (coordinatePlansByByte mark₂ [f coord])) coord = 1 := by
  simpa using
    (read_of_mem_coordinatePlansByByte_mapped_of_mark_eq
      (mark₁ := mark₁) (mark₂ := mark₂) (read := read) (f := f)
      (hRead := hRead) (hMark := hMark)
      (bytes := bytes) (coords := [coord]) (coord := coord) (by simp))

theorem coordRead_first_of_mappedPair_byByte_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (read : BoundedByteState byteCount → Coord₁ → ℕ)
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark₁ coord))
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (bytes : BoundedByteState byteCount) (coord₁ coord₂ : Coord₁) :
    read (coordinatePlanWriteMany mark₂ bytes
      (coordinatePlansByByte mark₂ [f coord₁, f coord₂])) coord₁ = 1 := by
  simpa using
    (read_of_mem_coordinatePlansByByte_mapped_of_mark_eq
      (mark₁ := mark₁) (mark₂ := mark₂) (read := read) (f := f)
      (hRead := hRead) (hMark := hMark)
      (bytes := bytes) (coords := [coord₁, coord₂]) (coord := coord₁) (by simp))

theorem coordRead_second_of_mappedPair_byByte_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (read : BoundedByteState byteCount → Coord₁ → ℕ)
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark₁ coord))
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (bytes : BoundedByteState byteCount) (coord₁ coord₂ : Coord₁) :
    read (coordinatePlanWriteMany mark₂ bytes
      (coordinatePlansByByte mark₂ [f coord₁, f coord₂])) coord₂ = 1 := by
  simpa using
    (read_of_mem_coordinatePlansByByte_mapped_of_mark_eq
      (mark₁ := mark₁) (mark₂ := mark₂) (read := read) (f := f)
      (hRead := hRead) (hMark := hMark)
      (bytes := bytes) (coords := [coord₁, coord₂]) (coord := coord₂) (by simp))

theorem coordReads_of_mappedPair_byByte_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (read : BoundedByteState byteCount → Coord₁ → ℕ)
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark₁ coord))
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (bytes : BoundedByteState byteCount) (coord₁ coord₂ : Coord₁) :
    read (coordinatePlanWriteMany mark₂ bytes
      (coordinatePlansByByte mark₂ [f coord₁, f coord₂])) coord₁ = 1 ∧
      read (coordinatePlanWriteMany mark₂ bytes
        (coordinatePlansByByte mark₂ [f coord₁, f coord₂])) coord₂ = 1 := by
  constructor
  · exact coordRead_first_of_mappedPair_byByte_of_mark_eq
      (read := read) (mark₁ := mark₁) (mark₂ := mark₂)
      (hRead := hRead) (f := f) (hMark := hMark)
      (bytes := bytes) (coord₁ := coord₁) (coord₂ := coord₂)
  · exact coordRead_second_of_mappedPair_byByte_of_mark_eq
      (read := read) (mark₁ := mark₁) (mark₂ := mark₂)
      (hRead := hRead) (f := f) (hMark := hMark)
      (bytes := bytes) (coord₁ := coord₁) (coord₂ := coord₂)

theorem read_of_mem_coordinatePlansByByte_of_eq_of_leftInverse
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark : Coord₁ → ByteMark byteCount)
    (read : BoundedByteState byteCount → Coord₁ → ℕ)
    (f : Coord₁ → Coord₂) (g : Coord₂ → Coord₁)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark coord))
    (hLeft : Function.LeftInverse g f)
    (bytes : BoundedByteState byteCount) (coords : List Coord₁)
    {coord : Coord₂} (hCoord : coord ∈ coords.map f) :
    read (coordinatePlanWriteMany mark bytes (coordinatePlansByByte mark coords)) (g coord) = 1 := by
  have hCoordOrig : g coord ∈ coords := by
    exact list_mem_of_mem_map_of_leftInverse f g hLeft coords hCoord
  exact read_of_mem_coordinatePlansByByte_of_eq mark read bytes coords hRead hCoordOrig

theorem read_of_mem_coordinatePlansByByte_mapped_of_mark_eq_of_leftInverse
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (read : BoundedByteState byteCount → Coord₁ → ℕ)
    (f : Coord₁ → Coord₂) (g : Coord₂ → Coord₁)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark₁ coord))
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (hLeft : Function.LeftInverse g f)
    (bytes : BoundedByteState byteCount) (coords : List Coord₁)
    {coord : Coord₂} (hCoord : coord ∈ coords.map f) :
    read
        (coordinatePlanWriteMany mark₂ bytes
          (coordinatePlansByByte mark₂ (coords.map f)))
        (g coord) = 1 := by
  rw [← coordinatePlanWriteMany_coordinatePlansByByte_eq_of_mark_eq
    mark₁ mark₂ f hMark bytes coords]
  exact read_of_mem_coordinatePlansByByte_of_eq_of_leftInverse
    mark₁ read f g hRead hLeft bytes coords hCoord

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

theorem read_of_mem_mappedPlans_distinct_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (read : BoundedByteState byteCount → Coord₁ → ℕ)
    (f : Coord₁ → Coord₂)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark₁ coord))
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (bytes : BoundedByteState byteCount)
    (plans : List (CoordinatePlan Coord₁ byteCount))
    (hAligned : ∀ plan ∈ plans, ∀ coord ∈ plan.2, (mark₁ coord).1 = plan.1)
    (hDistinct : coordinatePlansHaveDistinctByteSlots mark₁ plans)
    {plan : CoordinatePlan Coord₁ byteCount} (hPlan : plan ∈ plans)
    {coord : Coord₁} (hCoord : coord ∈ plan.2) :
    read
        (coordinatePlanWriteMany mark₂ bytes
          (plans.map fun plan => CoordinatePlan.map f plan))
        coord = 1 := by
  rw [← coordinatePlanWriteMany_eq_mappedPlans_of_mark_eq
    mark₁ mark₂ f hMark bytes plans]
  exact read_of_mem_coordinatePlans_distinct_of_eq
    mark₁ read bytes plans hRead hAligned hDistinct hPlan hCoord

theorem read_of_mem_mappedPlan_distinct_of_mark_eq_of_leftInverse
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (read : BoundedByteState byteCount → Coord₁ → ℕ)
    (f : Coord₁ → Coord₂) (g : Coord₂ → Coord₁)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark₁ coord))
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (hLeft : Function.LeftInverse g f)
    (bytes : BoundedByteState byteCount)
    (plans : List (CoordinatePlan Coord₁ byteCount))
    (hAligned :
      ∀ plan ∈ plans.map (fun plan => CoordinatePlan.map f plan),
        ∀ coord ∈ plan.2, (mark₂ coord).1 = plan.1)
    (hDistinct : coordinatePlansHaveDistinctByteSlots mark₂
      (plans.map fun plan => CoordinatePlan.map f plan))
    {plan : CoordinatePlan Coord₂ byteCount}
    (hPlan : plan ∈ plans.map (fun plan => CoordinatePlan.map f plan))
    {coord : Coord₂} (hCoord : coord ∈ plan.2) :
    read (coordinatePlanWriteMany mark₁ bytes plans) (g coord) = 1 := by
  obtain ⟨hPlanOrig, hCoordOrig⟩ :=
    coordinatePlan_preimage_membership_of_mem_mappedPlans
      (f := f) (g := g) (hLeft := hLeft) (plans := plans)
      (plan := plan) hPlan (coord := coord) hCoord
  have hAlignedOrig :
      ∀ plan ∈ plans, ∀ coord ∈ plan.2, (mark₁ coord).1 = plan.1 := by
    exact (coordinatePlans_mapped_aligned_iff_of_mark_eq
      (mark₁ := mark₁) (mark₂ := mark₂) (f := f) (hMark := hMark)
      (plans := plans)).mp hAligned
  have hDistinctOrig :
      coordinatePlansHaveDistinctByteSlots mark₁ plans := by
    exact (coordinatePlans_mapped_distinctByteSlots_iff_of_mark_eq
      (mark₁ := mark₁) (mark₂ := mark₂) (f := f) (hMark := hMark)
      (plans := plans)).mp hDistinct
  exact read_of_mem_coordinatePlans_distinct_of_eq
    mark₁ read bytes plans hRead hAlignedOrig hDistinctOrig
    hPlanOrig hCoordOrig

theorem coordRead_of_mem_mappedPlans_distinct_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (read : BoundedByteState byteCount → Coord₁ → ℕ)
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark₁ coord))
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (bytes : BoundedByteState byteCount)
    (plans : List (CoordinatePlan Coord₁ byteCount))
    (hAligned : ∀ plan ∈ plans, ∀ coord ∈ plan.2, (mark₁ coord).1 = plan.1)
    (hDistinct : coordinatePlansHaveDistinctByteSlots mark₁ plans)
    {plan : CoordinatePlan Coord₁ byteCount} (hPlan : plan ∈ plans)
    {coord : Coord₁} (hCoord : coord ∈ plan.2) :
    read (coordinateWriteMany mark₂ bytes
      (plans.map fun plan => CoordinatePlan.map f plan)) coord = 1 := by
  simpa [coordinateWriteMany] using
    (read_of_mem_mappedPlans_distinct_of_mark_eq
      mark₁ mark₂ read f hRead hMark bytes plans hAligned hDistinct hPlan hCoord)

theorem coordRead_of_mem_mappedPlan_distinct_of_mark_eq_of_leftInverse
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (read : BoundedByteState byteCount → Coord₁ → ℕ)
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark₁ coord))
    (f : Coord₁ → Coord₂) (g : Coord₂ → Coord₁)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (hLeft : Function.LeftInverse g f)
    (bytes : BoundedByteState byteCount)
    (plans : List (CoordinatePlan Coord₁ byteCount))
    (hAligned :
      ∀ plan ∈ plans.map (fun plan => CoordinatePlan.map f plan),
        ∀ coord ∈ plan.2, (mark₂ coord).1 = plan.1)
    (hDistinct : coordinatePlansHaveDistinctByteSlots mark₂
      (plans.map fun plan => CoordinatePlan.map f plan))
    {plan : CoordinatePlan Coord₂ byteCount}
    (hPlan : plan ∈ plans.map (fun plan => CoordinatePlan.map f plan))
    {coord : Coord₂} (hCoord : coord ∈ plan.2) :
    read (coordinateWriteMany mark₁ bytes plans) (g coord) = 1 := by
  simpa [coordinateWriteMany] using
    (read_of_mem_mappedPlan_distinct_of_mark_eq_of_leftInverse
      mark₁ mark₂ read f g hRead hMark hLeft bytes plans hAligned hDistinct hPlan hCoord)

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

theorem coordRead_mappedSingleton_of_mark_eq
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (read : BoundedByteState byteCount → Coord₁ → ℕ)
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark₁ coord))
    (f : Coord₁ → Coord₂)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (bytes : BoundedByteState byteCount) (coord : Coord₁) :
    read (coordinateWriteMany mark₂ bytes [singletonCoordinatePlan mark₂ (f coord)]) coord = 1 := by
  have hWrite :
      coordinateWriteMany mark₁ bytes [singletonCoordinatePlan mark₁ coord] =
        coordinateWriteMany mark₂ bytes [CoordinatePlan.map f (singletonCoordinatePlan mark₁ coord)] := by
    simpa [coordinateWriteMany] using
      (coordinatePlanWriteMany_eq_mappedPlans_of_mark_eq
        mark₁ mark₂ f hMark bytes [singletonCoordinatePlan mark₁ coord])
  calc
    read (coordinateWriteMany mark₂ bytes [singletonCoordinatePlan mark₂ (f coord)]) coord
        = read (coordinateWriteMany mark₂ bytes
            [CoordinatePlan.map f (singletonCoordinatePlan mark₁ coord)]) coord := by
            rw [CoordinatePlan.map_singletonCoordinatePlan mark₁ mark₂ f hMark coord]
    _ = read (coordinateWriteMany mark₁ bytes [singletonCoordinatePlan mark₁ coord]) coord := by
          rw [← hWrite]
    _ = 1 :=
          coordRead_singleton
            (read := read) (mark := mark₁) (hRead := hRead) (bytes := bytes) (coord := coord)

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

theorem coordRead_of_mem_mappedPair_distinct_of_mark_eq_of_leftInverse
    {Coord₁ Coord₂ : Type} {byteCount : ℕ}
    (read : BoundedByteState byteCount → Coord₁ → ℕ)
    (mark₁ : Coord₁ → ByteMark byteCount)
    (mark₂ : Coord₂ → ByteMark byteCount)
    (hRead : ∀ bytes coord, read bytes coord = byteMarkRead bytes (mark₁ coord))
    (f : Coord₁ → Coord₂) (g : Coord₂ → Coord₁)
    (hMark : ∀ coord, mark₂ (f coord) = mark₁ coord)
    (hLeft : Function.LeftInverse g f)
    (bytes : BoundedByteState byteCount) (coord₁ coord₂ : Coord₁)
    (hByte : (mark₁ coord₁).1 ≠ (mark₁ coord₂).1)
    {plan : CoordinatePlan Coord₂ byteCount}
    (hPlan :
      plan ∈ (coordinatePlanPair mark₁ coord₁ coord₂).map
        (fun plan => CoordinatePlan.map f plan))
    {coord : Coord₂} (hCoord : coord ∈ plan.2) :
    read (coordinateWriteMany mark₁ bytes (coordinatePlanPair mark₁ coord₁ coord₂)) (g coord) = 1 := by
  exact coordRead_of_mem_mappedPlan_distinct_of_mark_eq_of_leftInverse
    (read := read) (mark₁ := mark₁) (mark₂ := mark₂)
    (hRead := hRead) (f := f) (g := g) (hMark := hMark) (hLeft := hLeft)
    (bytes := bytes)
    (plans := coordinatePlanPair mark₁ coord₁ coord₂)
    (hAligned :=
      coordinatePlans_map_aligned_of_mark_eq
        (mark₁ := mark₁) (mark₂ := mark₂)
        (f := f) (hMark := hMark)
        (plans := coordinatePlanPair mark₁ coord₁ coord₂)
        (coordinatePlanPair_aligned mark₁ coord₁ coord₂))
    (hDistinct :=
      coordinatePlans_map_distinctByteSlots_of_mark_eq
        (mark₁ := mark₁) (mark₂ := mark₂)
        (f := f) (hMark := hMark)
        (plans := coordinatePlanPair mark₁ coord₁ coord₂)
        (coordinatePlanPair_distinct_of_byte_ne mark₁ hByte))
    (plan := plan) hPlan hCoord

end PrimeArithmetic.Sieve
