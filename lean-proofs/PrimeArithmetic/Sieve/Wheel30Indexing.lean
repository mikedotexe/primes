import Mathlib
import PrimeArithmetic.Sieve.Wheel30Agreement

namespace PrimeArithmetic.Sieve

/-!
Runtime-facing arithmetic for the wheel30 linear index used by
`src/prime_sieve/wheel30.rs`.

This module matches the concrete layout of the runtime code:

- one byte / wheel cycle for exactly `4096` runtime cycles
- a total numeric span of `4096 * 30`
- a slot number for each admissible residue modulo `30`
- the candidate formula `base + 30 * cycle + residue`
- the linear wheel index `cycle * 8 + slot`
- the byte/bit split `idx / 8`, `idx % 8`
- the runtime segment-bound check on the recovered cycle
-/

/-- Byte capacity of the runtime wheel30 bit array. -/
def wheel30SegmentBytes : ℕ := 4096

/-- Number of wheel cycles represented in one runtime wheel30 segment. -/
def wheel30SegmentCycles : ℕ := wheel30SegmentBytes

/-- Numeric span covered by one runtime wheel30 segment. -/
def wheel30SegmentSpan : ℕ := wheel30SegmentCycles * 30

/-- The runtime slot order for the admissible wheel30 residues. -/
def wheel30Slot : Fin 8 → ℕ
  | ⟨0, _⟩ => 1
  | ⟨1, _⟩ => 7
  | ⟨2, _⟩ => 11
  | ⟨3, _⟩ => 13
  | ⟨4, _⟩ => 17
  | ⟨5, _⟩ => 19
  | ⟨6, _⟩ => 23
  | ⟨7, _⟩ => 29

/-- The inverse slot lookup used by the runtime wheel indexer. -/
def wheel30SlotIndex : ℕ → Option (Fin 8)
  | 1 => some ⟨0, by decide⟩
  | 7 => some ⟨1, by decide⟩
  | 11 => some ⟨2, by decide⟩
  | 13 => some ⟨3, by decide⟩
  | 17 => some ⟨4, by decide⟩
  | 19 => some ⟨5, by decide⟩
  | 23 => some ⟨6, by decide⟩
  | 29 => some ⟨7, by decide⟩
  | _ => none

/-- Candidate reconstruction from the runtime base / cycle / slot triple. -/
def wheel30Candidate (base cycle : ℕ) (slot : Fin 8) : ℕ :=
  base + 30 * cycle + wheel30Slot slot

/-- Linear wheel index used by the runtime bit array. -/
def wheel30LinearIndex (cycle : ℕ) (slot : Fin 8) : ℕ :=
  cycle * 8 + slot.1

/-- Exact arithmetic version of the runtime wheel index function. -/
def wheel30Index (base n : ℕ) : Option ℕ :=
  if _ : base ≤ n then
    let offset := n - base
    match wheel30SlotIndex (offset % 30) with
    | some slot =>
        let cycle := offset / 30
        if _ : cycle < wheel30SegmentCycles then
          some (wheel30LinearIndex cycle slot)
        else
          none
    | none => none
  else
    none

@[simp] theorem wheel30SegmentBytes_eq : wheel30SegmentBytes = 4096 := by
  native_decide

@[simp] theorem wheel30SegmentCycles_eq : wheel30SegmentCycles = 4096 := by
  native_decide

@[simp] theorem wheel30SegmentSpan_eq : wheel30SegmentSpan = 122880 := by
  native_decide

theorem wheel30Slot_mem (slot : Fin 8) :
    wheel30Slot slot ∈ wheel30Residues := by
  fin_cases slot <;> native_decide

theorem wheel30Slot_lt_thirty (slot : Fin 8) :
    wheel30Slot slot < 30 := by
  fin_cases slot <;> decide

theorem wheel30SlotIndex_wheel30Slot (slot : Fin 8) :
    wheel30SlotIndex (wheel30Slot slot) = some slot := by
  fin_cases slot <;> rfl

theorem wheel30LinearIndex_byte (cycle : ℕ) (slot : Fin 8) :
    wheel30LinearIndex cycle slot / 8 = cycle := by
  unfold wheel30LinearIndex
  omega

theorem wheel30LinearIndex_bit (cycle : ℕ) (slot : Fin 8) :
    wheel30LinearIndex cycle slot % 8 = slot.1 := by
  unfold wheel30LinearIndex
  omega

theorem wheel30Candidate_mod {base cycle : ℕ} (slot : Fin 8)
    (hBase : base % 30 = 0) :
    wheel30Candidate base cycle slot % 30 = wheel30Slot slot := by
  have hSlot : wheel30Slot slot < 30 := wheel30Slot_lt_thirty slot
  unfold wheel30Candidate
  calc
    (base + 30 * cycle + wheel30Slot slot) % 30
        = (((base + 30 * cycle) % 30) + wheel30Slot slot % 30) % 30 := by
            rw [Nat.add_mod]
    _ = (0 + wheel30Slot slot) % 30 := by
          simp [Nat.add_mod, hBase]
    _ = wheel30Slot slot := by
          simp [Nat.mod_eq_of_lt hSlot]

theorem wheel30Representable_candidate {base cycle : ℕ} (slot : Fin 8)
    (hBase : base % 30 = 0) :
    wheel30Representable (wheel30Candidate base cycle slot) := by
  have hSlot : wheel30Slot slot ∈ wheel30Residues := wheel30Slot_mem slot
  refine (wheel30Representable_iff_mod_mem _).2 ?_
  simp [wheel30Candidate_mod slot hBase] at hSlot ⊢
  exact hSlot

theorem wheel30Index_candidate {base cycle : ℕ} (slot : Fin 8)
    (hCycleBound : cycle < wheel30SegmentCycles) :
    wheel30Index base (wheel30Candidate base cycle slot) =
      some (wheel30LinearIndex cycle slot) := by
  unfold wheel30Index
  have hGe : base ≤ wheel30Candidate base cycle slot := by
    unfold wheel30Candidate
    omega
  simp [hGe]
  have hOffset :
      wheel30Candidate base cycle slot - base = 30 * cycle + wheel30Slot slot := by
    unfold wheel30Candidate
    omega
  rw [hOffset]
  have hMod : (30 * cycle + wheel30Slot slot) % 30 = wheel30Slot slot := by
    calc
      (30 * cycle + wheel30Slot slot) % 30
          = ((30 * cycle) % 30 + wheel30Slot slot % 30) % 30 := by
              rw [Nat.add_mod]
      _ = wheel30Slot slot := by
            simp [Nat.mod_eq_of_lt (wheel30Slot_lt_thirty slot)]
  rw [hMod, wheel30SlotIndex_wheel30Slot]
  have hDiv :
      (30 * cycle + wheel30Slot slot) / 30 = cycle := by
    rw [Nat.add_comm, Nat.mul_comm, Nat.add_mul_div_right _ _ (by decide : 0 < 30)]
    simp [Nat.div_eq_of_lt (wheel30Slot_lt_thirty slot)]
  rw [hDiv]
  have hCycle4096 : cycle < 4096 := by
    simpa [wheel30SegmentCycles, wheel30SegmentBytes] using hCycleBound
  simp [hCycle4096]

theorem wheel30Index_candidate_none_of_segmentCycles_le {base cycle : ℕ} (slot : Fin 8)
    (hCycleBound : wheel30SegmentCycles ≤ cycle) :
    wheel30Index base (wheel30Candidate base cycle slot) = none := by
  unfold wheel30Index
  have hGe : base ≤ wheel30Candidate base cycle slot := by
    unfold wheel30Candidate
    omega
  simp [hGe]
  have hOffset :
      wheel30Candidate base cycle slot - base = 30 * cycle + wheel30Slot slot := by
    unfold wheel30Candidate
    omega
  rw [hOffset]
  have hMod : (30 * cycle + wheel30Slot slot) % 30 = wheel30Slot slot := by
    calc
      (30 * cycle + wheel30Slot slot) % 30
          = ((30 * cycle) % 30 + wheel30Slot slot % 30) % 30 := by
              rw [Nat.add_mod]
      _ = wheel30Slot slot := by
            simp [Nat.mod_eq_of_lt (wheel30Slot_lt_thirty slot)]
  rw [hMod, wheel30SlotIndex_wheel30Slot]
  have hDiv :
      (30 * cycle + wheel30Slot slot) / 30 = cycle := by
    rw [Nat.add_comm, Nat.mul_comm, Nat.add_mul_div_right _ _ (by decide : 0 < 30)]
    simp [Nat.div_eq_of_lt (wheel30Slot_lt_thirty slot)]
  rw [hDiv]
  have hCycle4096 : 4096 ≤ cycle := by
    simpa [wheel30SegmentCycles, wheel30SegmentBytes] using hCycleBound
  have hNot : ¬ cycle < 4096 := Nat.not_lt.mpr hCycle4096
  simp [hNot]

end PrimeArithmetic.Sieve
