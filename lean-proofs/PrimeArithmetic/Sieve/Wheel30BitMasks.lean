import Mathlib
import PrimeArithmetic.Sieve.Wheel30BitCoordinates

namespace PrimeArithmetic.Sieve

/-!
Bit-mask semantics for the wheel30 runtime segment in
`src/prime_sieve/wheel30.rs`.

This module matches the executable mask and readback expressions once the
candidate-specific bit index is known:

- writer: `bits[byte_idx] |= 1 << bit_idx`
- reader: `(bits[byte_idx] >> bit_idx) & 1 != 0`

The previous wheel30 modules already fixed the admissible candidate surface, the
slot order, the linear index, and the shared byte/bit coordinates. Here we
prove that the executable mask update and the executable readback test operate
on the same bit.
-/

/-- Bit mask used by the runtime wheel30 writer, when the candidate is encoded. -/
def wheel30BitMask (base n : ℕ) : Option ℕ :=
  Option.map (fun bit => 1 <<< bit) (wheel30BitIndex base n)

/-- Byte update performed by `mark_composite` once the target byte is selected. -/
def wheel30MarkByte (byte base n : ℕ) : Option ℕ :=
  Option.map (fun bit => byte ||| (1 <<< bit)) (wheel30BitIndex base n)

/-- Exact numeric readback performed by the runtime query expression. -/
def wheel30ReadValue (byte base n : ℕ) : Option ℕ :=
  Option.map (fun bit => (byte >>> bit) &&& 1) (wheel30BitIndex base n)

/-- Boolean version of the same readback, phrased as a bit test. -/
def wheel30ReadBit (byte base n : ℕ) : Option Bool :=
  Option.map (fun bit => byte.testBit bit) (wheel30BitIndex base n)

theorem wheel30BitMask_eq_some_of_some {base n bit : ℕ}
    (hBit : wheel30BitIndex base n = some bit) :
    wheel30BitMask base n = some (1 <<< bit) := by
  simp [wheel30BitMask, hBit]

theorem wheel30ReadValue_eq_toNat_readBit_of_some {byte base n bit : ℕ}
    (hBit : wheel30BitIndex base n = some bit) :
    wheel30ReadValue byte base n = some ((byte.testBit bit).toNat) := by
  unfold wheel30ReadValue
  simp [hBit]
  simpa [Nat.testBit] using (Nat.and_two_pow (byte >>> bit) 0)

theorem wheel30MarkByte_sets_target_bit_of_some {byte base n bit : ℕ}
    (hBit : wheel30BitIndex base n = some bit) :
    wheel30ReadBit (byte ||| (1 <<< bit)) base n = some true := by
  simp [wheel30ReadBit, hBit, Nat.shiftLeft_eq]

theorem wheel30MarkByte_preserves_other_bits {byte bit j : ℕ}
    (hNe : j ≠ bit) :
    ((byte ||| (1 <<< bit)).testBit j) = byte.testBit j := by
  rw [Nat.testBit_lor]
  have hpow : ((1 <<< bit).testBit j) = false := by
    simpa [Nat.shiftLeft_eq] using
      (Nat.testBit_two_pow_of_ne hNe.symm : (2 ^ bit).testBit j = false)
  rw [hpow]
  simp

theorem wheel30ReadValue_marked_of_some {byte base n bit : ℕ}
    (hBit : wheel30BitIndex base n = some bit) :
    wheel30ReadValue (byte ||| (1 <<< bit)) base n = some 1 := by
  rw [wheel30ReadValue_eq_toNat_readBit_of_some hBit]
  simp

theorem wheel30BitMask_candidate {base cycle : ℕ} (slot : Fin 8)
    (hCycle : cycle < wheel30SegmentCycles) :
    wheel30BitMask base (wheel30Candidate base cycle slot) = some (1 <<< slot.1) := by
  simpa [wheel30BitMask] using
    congrArg (Option.map fun bit => 1 <<< bit)
      (wheel30BitIndex_candidate (base := base) (cycle := cycle) slot hCycle)

theorem wheel30MarkByte_candidate {byte base cycle : ℕ} (slot : Fin 8)
    (hCycle : cycle < wheel30SegmentCycles) :
    wheel30MarkByte byte base (wheel30Candidate base cycle slot) =
      some (byte ||| (1 <<< slot.1)) := by
  simpa [wheel30MarkByte] using
    congrArg (Option.map fun bit => byte ||| (1 <<< bit))
      (wheel30BitIndex_candidate (base := base) (cycle := cycle) slot hCycle)

theorem wheel30ReadValue_marked_candidate {byte base cycle : ℕ} (slot : Fin 8)
    (hCycle : cycle < wheel30SegmentCycles) :
    wheel30ReadValue (byte ||| (1 <<< slot.1)) base (wheel30Candidate base cycle slot) =
      some 1 := by
  exact wheel30ReadValue_marked_of_some (byte := byte) (base := base)
    (n := wheel30Candidate base cycle slot) (bit := slot.1)
    (wheel30BitIndex_candidate slot hCycle)

end PrimeArithmetic.Sieve
