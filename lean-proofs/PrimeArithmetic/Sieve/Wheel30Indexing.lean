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

theorem wheel30SlotIndex_some_of_mem_wheel30Residues {r : ℕ}
    (hr : r ∈ wheel30Residues) :
    ∃ slot : Fin 8, wheel30SlotIndex r = some slot ∧ wheel30Slot slot = r := by
  have hr' :
      r = 1 ∨ r = 7 ∨ r = 11 ∨ r = 13 ∨ r = 17 ∨ r = 19 ∨ r = 23 ∨ r = 29 := by
    simpa [wheel30Residues] using hr
  rcases hr' with rfl | rfl | rfl | rfl | rfl | rfl | rfl | rfl
  all_goals
    refine ⟨_, rfl, rfl⟩

theorem wheel30SlotIndex_eq_some_iff_of_lt_thirty {r : ℕ}
    (hr : r < 30) (slot : Fin 8) :
    wheel30SlotIndex r = some slot ↔ wheel30Slot slot = r := by
  interval_cases r <;> fin_cases slot <;> decide

theorem wheel30LinearIndex_byte (cycle : ℕ) (slot : Fin 8) :
    wheel30LinearIndex cycle slot / 8 = cycle := by
  unfold wheel30LinearIndex
  omega

theorem wheel30LinearIndex_bit (cycle : ℕ) (slot : Fin 8) :
    wheel30LinearIndex cycle slot % 8 = slot.1 := by
  unfold wheel30LinearIndex
  omega

theorem wheel30Candidate_ge_base (base cycle : ℕ) (slot : Fin 8) :
    base ≤ wheel30Candidate base cycle slot := by
  unfold wheel30Candidate
  omega

theorem wheel30Candidate_sub_base (base cycle : ℕ) (slot : Fin 8) :
    wheel30Candidate base cycle slot - base = 30 * cycle + wheel30Slot slot := by
  unfold wheel30Candidate
  omega

theorem wheel30Candidate_sub_lt_thirty_of_same_cycle {base cycle : ℕ}
    (slot₁ slot₂ : Fin 8) :
    wheel30Candidate base cycle slot₂ - wheel30Candidate base cycle slot₁ < 30 := by
  have hLe :
      wheel30Candidate base cycle slot₂ - wheel30Candidate base cycle slot₁ ≤
        wheel30Slot slot₂ := by
    unfold wheel30Candidate
    omega
  exact lt_of_le_of_lt hLe (wheel30Slot_lt_thirty slot₂)

theorem wheel30Candidate_cycle_ne_of_thirty_le_sub {base cycle₁ cycle₂ : ℕ}
    (slot₁ slot₂ : Fin 8)
    (hSep :
      30 ≤ wheel30Candidate base cycle₂ slot₂ - wheel30Candidate base cycle₁ slot₁) :
    cycle₁ ≠ cycle₂ := by
  intro hCycle
  subst hCycle
  exact (not_lt_of_ge hSep)
    (wheel30Candidate_sub_lt_thirty_of_same_cycle (base := base) (cycle := cycle₁) slot₁ slot₂)

theorem wheel30Candidate_offset_mod (cycle : ℕ) (slot : Fin 8) :
    (30 * cycle + wheel30Slot slot) % 30 = wheel30Slot slot := by
  calc
    (30 * cycle + wheel30Slot slot) % 30
        = ((30 * cycle) % 30 + wheel30Slot slot % 30) % 30 := by
            rw [Nat.add_mod]
    _ = wheel30Slot slot := by
          simp [Nat.mod_eq_of_lt (wheel30Slot_lt_thirty slot)]

theorem wheel30Candidate_offset_div (cycle : ℕ) (slot : Fin 8) :
    (30 * cycle + wheel30Slot slot) / 30 = cycle := by
  rw [Nat.add_comm, Nat.mul_comm, Nat.add_mul_div_right _ _ (by decide : 0 < 30)]
  simp [Nat.div_eq_of_lt (wheel30Slot_lt_thirty slot)]

theorem wheel30Candidate_lt_base_plus_segmentSpan_of_cycle_lt {base cycle : ℕ}
    (slot : Fin 8) (hCycle : cycle < wheel30SegmentCycles) :
    wheel30Candidate base cycle slot < base + wheel30SegmentSpan := by
  have hSlot : wheel30Slot slot < 30 := wheel30Slot_lt_thirty slot
  unfold wheel30Candidate wheel30SegmentSpan at *
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
  have hGe : base ≤ wheel30Candidate base cycle slot :=
    wheel30Candidate_ge_base base cycle slot
  simp [hGe]
  rw [wheel30Candidate_sub_base base cycle slot]
  rw [wheel30Candidate_offset_mod cycle slot, wheel30SlotIndex_wheel30Slot]
  rw [wheel30Candidate_offset_div cycle slot]
  have hCycle4096 : cycle < 4096 := by
    simpa using hCycleBound
  simp [hCycle4096]

theorem wheel30Index_candidate_none_of_segmentCycles_le {base cycle : ℕ} (slot : Fin 8)
    (hCycleBound : wheel30SegmentCycles ≤ cycle) :
    wheel30Index base (wheel30Candidate base cycle slot) = none := by
  unfold wheel30Index
  have hGe : base ≤ wheel30Candidate base cycle slot :=
    wheel30Candidate_ge_base base cycle slot
  simp [hGe]
  rw [wheel30Candidate_sub_base base cycle slot]
  rw [wheel30Candidate_offset_mod cycle slot, wheel30SlotIndex_wheel30Slot]
  rw [wheel30Candidate_offset_div cycle slot]
  have hCycle4096 : 4096 ≤ cycle := by
    simpa using hCycleBound
  have hNot : ¬ cycle < 4096 := Nat.not_lt.mpr hCycle4096
  simp [hNot]

theorem exists_wheel30Candidate_of_segmentRepresentable
    {base n : ℕ} (hBase : base % 30 = 0)
    (hGe : base ≤ n) (hLt : n < base + wheel30SegmentSpan)
    (hRep : wheel30Representable n) :
    ∃ cycle : ℕ, ∃ slot : Fin 8,
      cycle < wheel30SegmentCycles ∧
      n = wheel30Candidate base cycle slot ∧
      wheel30Index base n = some (wheel30LinearIndex cycle slot) := by
  have hOffsetRep : wheel30Representable (n - base) := by
    exact (wheel30Representable_sub_base_iff hBase hGe).2 hRep
  have hResidueMem : (n - base) % 30 ∈ wheel30Residues := by
    exact (wheel30Representable_iff_mod_mem _).1 hOffsetRep
  obtain ⟨slot, hSlotIndex, hSlotEq⟩ :=
    wheel30SlotIndex_some_of_mem_wheel30Residues hResidueMem
  have hOffsetLt : n - base < wheel30SegmentSpan := by
    omega
  have hCycle : (n - base) / 30 < wheel30SegmentCycles := by
    exact Nat.div_lt_of_lt_mul (by simpa [wheel30SegmentSpan] using hOffsetLt)
  have hCandidate :
      n = wheel30Candidate base ((n - base) / 30) slot := by
    have hOffsetEq :
        n - base = 30 * ((n - base) / 30) + (n - base) % 30 := by
      simpa [Nat.add_comm, Nat.add_left_comm, Nat.add_assoc] using
        (Nat.mod_add_div (n - base) 30).symm
    calc
      n = base + (n - base) := by omega
      _ = base + (30 * ((n - base) / 30) + (n - base) % 30) := by
        nth_rewrite 1 [hOffsetEq]
        rfl
      _ = base + (30 * ((n - base) / 30) + wheel30Slot slot) := by rw [← hSlotEq]
      _ = wheel30Candidate base ((n - base) / 30) slot := by
        unfold wheel30Candidate
        omega
  refine ⟨(n - base) / 30, slot, hCycle, hCandidate, ?_⟩
  conv_lhs => rw [hCandidate]
  exact wheel30Index_candidate (base := base) (cycle := (n - base) / 30) slot hCycle

theorem exists_wheel30Index_of_segmentRepresentable
    {base n : ℕ} (hBase : base % 30 = 0)
    (hGe : base ≤ n) (hLt : n < base + wheel30SegmentSpan)
    (hRep : wheel30Representable n) :
    ∃ idx : ℕ, wheel30Index base n = some idx := by
  obtain ⟨cycle, slot, hCycle, _, hIdx⟩ :=
    exists_wheel30Candidate_of_segmentRepresentable hBase hGe hLt hRep
  exact ⟨wheel30LinearIndex cycle slot, hIdx⟩

theorem exists_wheel30Candidate_of_index_eq_some {base n idx : ℕ}
    (hIndex : wheel30Index base n = some idx) :
    ∃ cycle : ℕ, ∃ slot : Fin 8,
      cycle < wheel30SegmentCycles ∧
      n = wheel30Candidate base cycle slot ∧
      idx = wheel30LinearIndex cycle slot := by
  unfold wheel30Index at hIndex
  by_cases hGe : base ≤ n
  · simp [hGe] at hIndex
    let offset := n - base
    have hOffsetLtThirty : offset % 30 < 30 := Nat.mod_lt _ (by decide)
    cases hSlot : wheel30SlotIndex (offset % 30) with
    | none =>
        rw [hSlot] at hIndex
        cases hIndex
    | some slot =>
        by_cases hCycle : offset / 30 < wheel30SegmentCycles
        · have hIdxPair :
              offset / 30 < wheel30SegmentCycles ∧
                wheel30LinearIndex (offset / 30) slot = idx := by
            simpa [offset, wheel30SegmentCycles, hSlot] using hIndex
          have hIdxEq : wheel30LinearIndex (offset / 30) slot = idx := hIdxPair.2
          have hSlotEq : wheel30Slot slot = offset % 30 := by
            exact (wheel30SlotIndex_eq_some_iff_of_lt_thirty hOffsetLtThirty slot).mp hSlot
          refine ⟨offset / 30, slot, hCycle, ?_, hIdxEq.symm⟩
          calc
            n = base + offset := by
                  omega
            _ = base + (offset % 30 + 30 * (offset / 30)) := by
                  rw [Nat.mod_add_div offset 30]
            _ = base + (30 * (offset / 30) + offset % 30) := by omega
            _ = base + (30 * (offset / 30) + wheel30Slot slot) := by rw [← hSlotEq]
            _ = wheel30Candidate base (offset / 30) slot := by
                  simp [wheel30Candidate, Nat.add_assoc]
        · have hIdxPair :
              offset / 30 < wheel30SegmentCycles ∧
                wheel30LinearIndex (offset / 30) slot = idx := by
            simpa [offset, wheel30SegmentCycles, hSlot] using hIndex
          exact False.elim (hCycle hIdxPair.1)
  · simp [hGe] at hIndex

theorem wheel30Index_eq_some_iff {base n idx : ℕ} :
    wheel30Index base n = some idx ↔
      ∃ cycle : ℕ, ∃ slot : Fin 8,
        cycle < wheel30SegmentCycles ∧
        n = wheel30Candidate base cycle slot ∧
        idx = wheel30LinearIndex cycle slot := by
  constructor
  · exact exists_wheel30Candidate_of_index_eq_some
  · rintro ⟨cycle, slot, hCycle, hCandidate, hIdx⟩
    rw [hCandidate, hIdx]
    exact wheel30Index_candidate (base := base) (cycle := cycle) slot hCycle

theorem exists_wheel30Candidate_of_segmentPrimeGtThirty
    {base p : ℕ} (hBase : base % 30 = 0)
    (hGe : base ≤ p) (hLt : p < base + wheel30SegmentSpan)
    (hPrime : Nat.Prime p) (hGt : 30 < p) :
    ∃ cycle : ℕ, ∃ slot : Fin 8,
      cycle < wheel30SegmentCycles ∧
      p = wheel30Candidate base cycle slot ∧
      wheel30Index base p = some (wheel30LinearIndex cycle slot) := by
  exact exists_wheel30Candidate_of_segmentRepresentable hBase hGe hLt
    (primeGtThirty_wheel30Representable hPrime hGt)

end PrimeArithmetic.Sieve
