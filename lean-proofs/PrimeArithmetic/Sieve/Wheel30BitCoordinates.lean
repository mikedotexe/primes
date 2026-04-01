import Mathlib
import PrimeArithmetic.Sieve.Wheel30Indexing

namespace PrimeArithmetic.Sieve

/-!
Byte/bit coordinates for the wheel30 segment in `src/prime_sieve/wheel30.rs`.

This is the exact arithmetic content behind the shared runtime formulas:

- `wheel_idx = cycle * 8 + slot`
- `byte = wheel_idx / 8`
- `bit = wheel_idx % 8`
-/

/-- Total number of wheel30 slots represented in the runtime bit array. -/
def wheel30SegmentSlots : ℕ := wheel30SegmentCycles * 8

/-- Byte coordinate extracted from the runtime wheel30 index. -/
def wheel30ByteIndex (base n : ℕ) : Option ℕ :=
  Option.map (fun idx => idx / 8) (wheel30Index base n)

/-- Bit coordinate extracted from the runtime wheel30 index. -/
def wheel30BitIndex (base n : ℕ) : Option ℕ :=
  Option.map (fun idx => idx % 8) (wheel30Index base n)

/-- Combined byte/bit coordinates extracted from the runtime wheel30 index. -/
def wheel30BitCoordinates (base n : ℕ) : Option (ℕ × ℕ) :=
  Option.map (fun idx => (idx / 8, idx % 8)) (wheel30Index base n)

@[simp] theorem wheel30SegmentSlots_eq : wheel30SegmentSlots = 32768 := by
  native_decide

theorem wheel30LinearIndex_lt_segmentSlots {cycle : ℕ} (slot : Fin 8)
    (hCycle : cycle < wheel30SegmentCycles) :
    wheel30LinearIndex cycle slot < wheel30SegmentSlots := by
  have hSlot : slot.1 < 8 := slot.2
  unfold wheel30LinearIndex wheel30SegmentSlots
  have h1 : cycle * 8 + slot.1 < cycle * 8 + 8 := by
    exact Nat.add_lt_add_left hSlot (cycle * 8)
  have h2 : cycle * 8 + 8 ≤ wheel30SegmentCycles * 8 := by
    calc
      cycle * 8 + 8 = (cycle + 1) * 8 := by omega
      _ ≤ wheel30SegmentCycles * 8 := by
        exact Nat.mul_le_mul_right 8 (Nat.succ_le_of_lt hCycle)
  exact lt_of_lt_of_le h1 h2

theorem wheel30ByteIndex_candidate {base cycle : ℕ} (slot : Fin 8)
    (hCycle : cycle < wheel30SegmentCycles) :
    wheel30ByteIndex base (wheel30Candidate base cycle slot) = some cycle := by
  rw [wheel30ByteIndex]
  rw [wheel30Index_candidate (base := base) (cycle := cycle) slot hCycle]
  simp [wheel30LinearIndex_byte]

theorem wheel30BitIndex_candidate {base cycle : ℕ} (slot : Fin 8)
    (hCycle : cycle < wheel30SegmentCycles) :
    wheel30BitIndex base (wheel30Candidate base cycle slot) = some slot.1 := by
  rw [wheel30BitIndex]
  rw [wheel30Index_candidate (base := base) (cycle := cycle) slot hCycle]
  simp [wheel30LinearIndex_bit]

theorem wheel30BitCoordinates_candidate {base cycle : ℕ} (slot : Fin 8)
    (hCycle : cycle < wheel30SegmentCycles) :
    wheel30BitCoordinates base (wheel30Candidate base cycle slot) = some (cycle, slot.1) := by
  rw [wheel30BitCoordinates]
  rw [wheel30Index_candidate (base := base) (cycle := cycle) slot hCycle]
  simp [wheel30LinearIndex_byte, wheel30LinearIndex_bit]

theorem wheel30BitIndex_lt_eight {base n bit : ℕ}
    (hBit : wheel30BitIndex base n = some bit) :
    bit < 8 := by
  unfold wheel30BitIndex at hBit
  cases hIdx : wheel30Index base n with
  | none =>
      simp [hIdx] at hBit
  | some idx =>
      simp [hIdx] at hBit
      subst hBit
      exact Nat.mod_lt _ (by decide)

theorem wheel30MarkQuery_same_coordinates_candidate {base cycle : ℕ} (slot : Fin 8)
    (hCycle : cycle < wheel30SegmentCycles) :
    wheel30BitCoordinates base (wheel30Candidate base cycle slot) =
      some (wheel30LinearIndex cycle slot / 8, wheel30LinearIndex cycle slot % 8) := by
  simpa [wheel30LinearIndex_byte, wheel30LinearIndex_bit] using
    (wheel30BitCoordinates_candidate (base := base) (cycle := cycle) slot hCycle)

end PrimeArithmetic.Sieve
