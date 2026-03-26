import Mathlib

namespace PrimeArithmetic.Sieve

/-!
Generic bounded byte-array mark families for sieve-style bitsets.

This module abstracts the last exact step shared by the odd-only and wheel30
runtime layouts:

- a bounded family of bytes indexed by `Fin byteCount`
- a mark given by a byte slot and one bit in that byte
- a one-byte update that ORs in the corresponding mask
- a readback that extracts the selected bit

The theorems stay intentionally small and exact. They cover:

- one-mark correctness
- preservation under writes to other byte slots
- finite families of marks whose byte slots are pairwise distinct

This gives a short but real whole-segment agreement surface without yet taking
on same-byte collision combinatorics or full mutable-array semantics.
-/

/-- Functional model of a bounded family of bytes. -/
abbrev BoundedByteState (byteCount : ℕ) := Fin byteCount → ℕ

/-- A single bit mark in a bounded byte family. -/
abbrev ByteMark (byteCount : ℕ) := Fin byteCount × Fin 8

/-- Numeric readback of a marked bit. -/
def byteMarkRead {byteCount : ℕ} (bytes : BoundedByteState byteCount)
    (mark : ByteMark byteCount) : ℕ :=
  ((bytes mark.1) >>> mark.2.1) &&& 1

/-- One-byte update at the selected slot and bit. -/
def byteMarkWrite {byteCount : ℕ} (bytes : BoundedByteState byteCount)
    (mark : ByteMark byteCount) : BoundedByteState byteCount :=
  Function.update bytes mark.1 (bytes mark.1 ||| (1 <<< mark.2.1))

/-- Left-to-right family update for a finite list of marks. -/
def byteMarkWriteMany {byteCount : ℕ} (bytes : BoundedByteState byteCount) :
    List (ByteMark byteCount) → BoundedByteState byteCount
  | [] => bytes
  | mark :: marks => byteMarkWriteMany (byteMarkWrite bytes mark) marks

/-- Pairwise disjointness of the touched byte slots. -/
def marksHaveDistinctByteSlots {byteCount : ℕ} (marks : List (ByteMark byteCount)) : Prop :=
  marks.Pairwise fun a b => a.1 ≠ b.1

theorem byteMarkRead_eq_toNat_testBit {byteCount : ℕ}
    (bytes : BoundedByteState byteCount) (mark : ByteMark byteCount) :
    byteMarkRead bytes mark = ((bytes mark.1).testBit mark.2.1).toNat := by
  unfold byteMarkRead
  simpa [Nat.testBit] using (Nat.and_two_pow ((bytes mark.1) >>> mark.2.1) 0)

theorem byteMarkRead_written {byteCount : ℕ} (bytes : BoundedByteState byteCount)
    (mark : ByteMark byteCount) :
    byteMarkRead (byteMarkWrite bytes mark) mark = 1 := by
  rw [byteMarkRead_eq_toNat_testBit]
  unfold byteMarkWrite
  simp [Function.update, Nat.shiftLeft_eq]

theorem byteMarkRead_preserved_by_write_other_byte {byteCount : ℕ}
    (bytes : BoundedByteState byteCount) (target update : ByteMark byteCount)
    (hByte : target.1 ≠ update.1) (hRead : byteMarkRead bytes target = 1) :
    byteMarkRead (byteMarkWrite bytes update) target = 1 := by
  unfold byteMarkRead byteMarkWrite at *
  simp [Function.update, hByte, hRead]

theorem byteMarkRead_preserved_by_writeMany_other_bytes {byteCount : ℕ}
    (marks : List (ByteMark byteCount)) (bytes : BoundedByteState byteCount)
    (target : ByteMark byteCount)
    (hOther : ∀ mark ∈ marks, target.1 ≠ mark.1)
    (hRead : byteMarkRead bytes target = 1) :
    byteMarkRead (byteMarkWriteMany bytes marks) target = 1 := by
  induction marks generalizing bytes with
  | nil =>
      simpa [byteMarkWriteMany] using hRead
  | cons mark marks ih =>
      simp [byteMarkWriteMany]
      apply ih
      · intro mark' hMem
        exact hOther mark' (List.mem_cons_of_mem _ hMem)
      · exact byteMarkRead_preserved_by_write_other_byte bytes target mark
          (hOther mark (by simp)) hRead

theorem byteMarkRead_of_mem_writeMany_distinct {byteCount : ℕ}
    (marks : List (ByteMark byteCount)) (bytes : BoundedByteState byteCount)
    (target : ByteMark byteCount) (hDistinct : marksHaveDistinctByteSlots marks)
    (hMem : target ∈ marks) :
    byteMarkRead (byteMarkWriteMany bytes marks) target = 1 := by
  induction marks generalizing bytes with
  | nil =>
      cases hMem
  | cons mark marks ih =>
      rw [List.mem_cons] at hMem
      cases hDistinct with
      | cons hHead hTail =>
          cases hMem with
          | inl hEq =>
              subst hEq
              simp [byteMarkWriteMany]
              exact byteMarkRead_preserved_by_writeMany_other_bytes marks _ target
                (fun mark' hMem' => hHead mark' hMem')
                (byteMarkRead_written bytes target)
          | inr hTailMem =>
              simp [byteMarkWriteMany]
              exact ih _ hTail hTailMem

end PrimeArithmetic.Sieve
