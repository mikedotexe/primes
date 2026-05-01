import Mathlib
import PrimeArithmetic.Sieve.BoundedByteMasks

namespace PrimeArithmetic.Sieve

/-!
Grouped multi-byte mark plans for sieve-style bitsets.

`BoundedByteFamilies.lean` handles disjoint single-bit writes across bytes, and
`BoundedByteMasks.lean` handles repeated writes within one byte by collapsing
them into one OR-mask update. This module combines those two routes into one
cleaner abstraction: a finite list of per-byte plans.

Each plan names:

- one bounded byte slot
- a finite list of bits to set in that byte

The resulting theorem family shows:

- one grouped plan is equivalent to its flattened single-bit write trace
- a whole grouped plan family is equivalent to the corresponding flattened
  write trace
- grouped plan families concatenate exactly, both before and after flattening
- if the planned byte slots are pairwise distinct, then every planned bit reads
  back as `1` after the whole family update

This gives a short exact whole-segment surface that already matches the natural
runtime idea of “touch these bytes, and within each byte set these bits”.
-/

/-- A grouped write plan for one bounded byte slot. -/
abbrev BytePlan (byteCount : ℕ) := Fin byteCount × List (Fin 8)

/-- Apply one grouped byte plan by ORing in its aggregated mask. -/
def bytePlanWrite {byteCount : ℕ} (bytes : BoundedByteState byteCount)
    (plan : BytePlan byteCount) : BoundedByteState byteCount :=
  byteMaskWrite bytes plan.1 (bitsMask plan.2)

/-- Flatten one grouped byte plan into its single-bit marks. -/
def bytePlanMarks {byteCount : ℕ} (plan : BytePlan byteCount) : List (ByteMark byteCount) :=
  byteMarksAt plan.1 plan.2

/-- Left-to-right application of a finite list of grouped byte plans. -/
def bytePlanWriteMany {byteCount : ℕ} (bytes : BoundedByteState byteCount) :
    List (BytePlan byteCount) → BoundedByteState byteCount
  | [] => bytes
  | plan :: plans => bytePlanWriteMany (bytePlanWrite bytes plan) plans

/-- Flatten a list of grouped byte plans into a single-bit write trace. -/
def bytePlanMarkFamily {byteCount : ℕ} : List (BytePlan byteCount) → List (ByteMark byteCount)
  | [] => []
  | plan :: plans => bytePlanMarks plan ++ bytePlanMarkFamily plans

/-- Pairwise disjointness of the byte slots touched by grouped plans. -/
def plansHaveDistinctByteSlots {byteCount : ℕ} (plans : List (BytePlan byteCount)) : Prop :=
  plans.Pairwise fun a b => a.1 ≠ b.1

theorem bytePlanMarkFamily_append {byteCount : ℕ}
    (xs ys : List (BytePlan byteCount)) :
    bytePlanMarkFamily (xs ++ ys) = bytePlanMarkFamily xs ++ bytePlanMarkFamily ys := by
  induction xs with
  | nil =>
      simp [bytePlanMarkFamily]
  | cons plan plans ih =>
      simp [bytePlanMarkFamily, ih, List.append_assoc]

theorem bytePlanWriteMany_append {byteCount : ℕ} (bytes : BoundedByteState byteCount)
    (xs ys : List (BytePlan byteCount)) :
    bytePlanWriteMany bytes (xs ++ ys) = bytePlanWriteMany (bytePlanWriteMany bytes xs) ys := by
  induction xs generalizing bytes with
  | nil =>
      simp [bytePlanWriteMany]
  | cons plan plans ih =>
      simp [bytePlanWriteMany, ih]

theorem bytePlanWrite_eq_byteMarkWriteMany {byteCount : ℕ}
    (bytes : BoundedByteState byteCount) (plan : BytePlan byteCount) :
    bytePlanWrite bytes plan = byteMarkWriteMany bytes (bytePlanMarks plan) := by
  cases plan with
  | mk slot bits =>
      simpa [bytePlanWrite, bytePlanMarks] using
        (byteMarkWriteMany_at_eq_byteMaskWrite bytes slot bits).symm

theorem bytePlanWriteMany_eq_byteMarkWriteMany {byteCount : ℕ}
    (bytes : BoundedByteState byteCount) (plans : List (BytePlan byteCount)) :
    bytePlanWriteMany bytes plans = byteMarkWriteMany bytes (bytePlanMarkFamily plans) := by
  induction plans generalizing bytes with
  | nil =>
      simp [bytePlanWriteMany, bytePlanMarkFamily, byteMarkWriteMany]
  | cons plan plans ih =>
      simp [bytePlanWriteMany, bytePlanMarkFamily]
      rw [bytePlanWrite_eq_byteMarkWriteMany, ih, ← byteMarkWriteMany_append]

theorem mem_bytePlanMarks_iff {byteCount : ℕ}
    (plan : BytePlan byteCount) (target : ByteMark byteCount) :
    target ∈ bytePlanMarks plan ↔ target.1 = plan.1 ∧ target.2 ∈ plan.2 := by
  cases plan with
  | mk slot bits =>
      cases target with
      | mk targetSlot targetBit =>
          induction bits with
          | nil =>
              simp [bytePlanMarks, byteMarksAt]
          | cons bit bits ih =>
              have ih' :
                  (targetSlot, targetBit) ∈ byteMarksAt slot bits ↔
                    targetSlot = slot ∧ targetBit ∈ bits := by
                simpa [bytePlanMarks] using ih
              simp [bytePlanMarks, byteMarksAt, ih', and_or_left]

theorem exists_bytePlan_mem_of_mem_bytePlanMarkFamily {byteCount : ℕ}
    (plans : List (BytePlan byteCount))
    {target : ByteMark byteCount} (hTarget : target ∈ bytePlanMarkFamily plans) :
    ∃ plan ∈ plans, target.1 = plan.1 ∧ target.2 ∈ plan.2 := by
  induction plans with
  | nil =>
      cases hTarget
  | cons plan plans ih =>
      simp only [bytePlanMarkFamily, List.mem_append] at hTarget
      cases hTarget with
      | inl hHead =>
          exact ⟨plan, by simp, (mem_bytePlanMarks_iff plan target).mp hHead⟩
      | inr hTail =>
          rcases ih hTail with ⟨plan', hPlan', hSlot, hBit⟩
          exact ⟨plan', by simp [hPlan'], hSlot, hBit⟩

theorem byteMarkRead_preserved_by_planWrite_other_byte {byteCount : ℕ}
    (bytes : BoundedByteState byteCount) (target : ByteMark byteCount)
    (plan : BytePlan byteCount) (hByte : target.1 ≠ plan.1)
    (hRead : byteMarkRead bytes target = 1) :
    byteMarkRead (bytePlanWrite bytes plan) target = 1 := by
  exact byteMarkRead_preserved_by_mask_other_byte bytes target plan.1 (bitsMask plan.2) hByte hRead

theorem byteMarkRead_preserved_by_planWriteMany_other_bytes {byteCount : ℕ}
    (plans : List (BytePlan byteCount)) (bytes : BoundedByteState byteCount)
    (target : ByteMark byteCount)
    (hOther : ∀ plan ∈ plans, target.1 ≠ plan.1)
    (hRead : byteMarkRead bytes target = 1) :
    byteMarkRead (bytePlanWriteMany bytes plans) target = 1 := by
  induction plans generalizing bytes with
  | nil =>
      simpa [bytePlanWriteMany] using hRead
  | cons plan plans ih =>
      simp [bytePlanWriteMany]
      apply ih
      · intro plan' hMem
        exact hOther plan' (List.mem_cons_of_mem _ hMem)
      · exact byteMarkRead_preserved_by_planWrite_other_byte bytes target plan
          (hOther plan (by simp)) hRead

theorem byteMarkRead_of_mem_planWriteMany_distinct {byteCount : ℕ}
    (plans : List (BytePlan byteCount)) (bytes : BoundedByteState byteCount)
    (slot : Fin byteCount) (bits : List (Fin 8)) (target : Fin 8)
    (hDistinct : plansHaveDistinctByteSlots plans)
    (hPlan : (slot, bits) ∈ plans) (hBit : target ∈ bits) :
    byteMarkRead (bytePlanWriteMany bytes plans) (slot, target) = 1 := by
  induction plans generalizing bytes slot bits target with
  | nil =>
      cases hPlan
  | cons plan plans ih =>
      rw [List.mem_cons] at hPlan
      cases hDistinct with
      | cons hHead hTail =>
          cases hPlan with
          | inl hEq =>
              subst hEq
              simp [bytePlanWriteMany]
              exact byteMarkRead_preserved_by_planWriteMany_other_bytes plans _ (slot, target)
                (fun plan' hMem' => hHead plan' hMem')
                (byteMarkRead_written_by_bits_of_mem bytes slot bits target hBit)
          | inr hTailMem =>
              simp [bytePlanWriteMany]
              exact ih (bytes := bytePlanWrite bytes plan) (slot := slot) (bits := bits)
                (target := target) hTail hTailMem hBit

theorem byteMarkRead_of_mem_bytePlanMarkFamily_distinct {byteCount : ℕ}
    (plans : List (BytePlan byteCount)) (bytes : BoundedByteState byteCount)
    (hDistinct : plansHaveDistinctByteSlots plans)
    {target : ByteMark byteCount} (hTarget : target ∈ bytePlanMarkFamily plans) :
    byteMarkRead (bytePlanWriteMany bytes plans) target = 1 := by
  rcases exists_bytePlan_mem_of_mem_bytePlanMarkFamily (plans := plans) hTarget with
    ⟨plan, hPlan, hSlot, hBit⟩
  cases target with
  | mk slot bit =>
      simp only at hSlot hBit
      subst slot
      simpa using
        (byteMarkRead_of_mem_planWriteMany_distinct
          (plans := plans) (bytes := bytes) (slot := plan.1)
          (bits := plan.2) (target := bit) hDistinct hPlan hBit)

end PrimeArithmetic.Sieve
