import Mathlib

namespace PrimeArithmetic.Structure

open scoped BigOperators

/-!
Exact finite combinatorics for aligned divisibility-mask transfer.

This is the arithmetic core beneath the maintained bounded-`k` transfer lane:

- each aligned candidate carries a `from` and `to` divisibility mask,
- admissibility means the mask is empty,
- the transfer buckets record whether emptiness is shared, gained, lost, or
  whether the nonzero mask stays the same or churns.

The statements here are fully finite and exact. They do not refer to primality
or any threshold theorem by themselves.
-/

abbrev DivMask (ι : Type*) := Finset ι

inductive TransferBucket
  | stableZero
  | gainZero
  | lossZero
  | stableNonzero
  | nonzeroChurn
  deriving DecidableEq, Fintype, Repr

def transferBucket [DecidableEq ι] (fromMask toMask : DivMask ι) : TransferBucket :=
  if _hFrom : fromMask = ∅ then
    if _hTo : toMask = ∅ then .stableZero else .lossZero
  else if _hTo : toMask = ∅ then
    .gainZero
  else if _hEq : fromMask = toMask then
    .stableNonzero
  else
    .nonzeroChurn

theorem transferBucket_eq_stableZero_iff [DecidableEq ι]
    {fromMask toMask : DivMask ι} :
    transferBucket fromMask toMask = .stableZero ↔ fromMask = ∅ ∧ toMask = ∅ := by
  by_cases hFrom : fromMask = ∅ <;> by_cases hTo : toMask = ∅ <;> by_cases hEq : fromMask = toMask <;>
    simp [transferBucket, hFrom, hTo, hEq]

theorem transferBucket_eq_gainZero_iff [DecidableEq ι]
    {fromMask toMask : DivMask ι} :
    transferBucket fromMask toMask = .gainZero ↔ fromMask ≠ ∅ ∧ toMask = ∅ := by
  by_cases hFrom : fromMask = ∅ <;> by_cases hTo : toMask = ∅ <;> by_cases hEq : fromMask = toMask <;>
    simp [transferBucket, hFrom, hTo, hEq]

theorem transferBucket_eq_lossZero_iff [DecidableEq ι]
    {fromMask toMask : DivMask ι} :
    transferBucket fromMask toMask = .lossZero ↔ fromMask = ∅ ∧ toMask ≠ ∅ := by
  by_cases hFrom : fromMask = ∅ <;> by_cases hTo : toMask = ∅ <;> by_cases hEq : fromMask = toMask <;>
    simp [transferBucket, hFrom, hTo, hEq]

theorem transferBucket_eq_stableNonzero_iff [DecidableEq ι]
    {fromMask toMask : DivMask ι} :
    transferBucket fromMask toMask = .stableNonzero ↔
      fromMask ≠ ∅ ∧ toMask ≠ ∅ ∧ fromMask = toMask := by
  by_cases hFrom : fromMask = ∅ <;> by_cases hTo : toMask = ∅ <;> by_cases hEq : fromMask = toMask <;>
    simp [transferBucket, hFrom, hTo, hEq]

theorem transferBucket_eq_nonzeroChurn_iff [DecidableEq ι]
    {fromMask toMask : DivMask ι} :
    transferBucket fromMask toMask = .nonzeroChurn ↔
      fromMask ≠ ∅ ∧ toMask ≠ ∅ ∧ fromMask ≠ toMask := by
  by_cases hFrom : fromMask = ∅ <;> by_cases hTo : toMask = ∅ <;> by_cases hEq : fromMask = toMask <;>
    simp [transferBucket, hFrom, hTo, hEq]

def bucketSet [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι) (bucket : TransferBucket) :
    Finset α :=
  s.filter fun a => transferBucket (maskFrom a) (maskTo a) = bucket

def bucketCount [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι) (bucket : TransferBucket) : ℕ :=
  (bucketSet s maskFrom maskTo bucket).card

theorem mem_bucketSet_iff [DecidableEq α] [DecidableEq ι]
    {s : Finset α} {maskFrom maskTo : α → DivMask ι} {bucket : TransferBucket} {a : α} :
    a ∈ bucketSet s maskFrom maskTo bucket ↔
      a ∈ s ∧ transferBucket (maskFrom a) (maskTo a) = bucket := by
  simp [bucketSet]

theorem bucketSet_disjoint_of_ne [DecidableEq α] [DecidableEq ι]
    {s : Finset α} {maskFrom maskTo : α → DivMask ι}
    {bucket₁ bucket₂ : TransferBucket} (hNe : bucket₁ ≠ bucket₂) :
    Disjoint (bucketSet s maskFrom maskTo bucket₁) (bucketSet s maskFrom maskTo bucket₂) := by
  rw [Finset.disjoint_left]
  intro a ha₁ ha₂
  have hb₁ : transferBucket (maskFrom a) (maskTo a) = bucket₁ := (mem_bucketSet_iff.mp ha₁).2
  have hb₂ : transferBucket (maskFrom a) (maskTo a) = bucket₂ := (mem_bucketSet_iff.mp ha₂).2
  exact hNe (hb₁.symm.trans hb₂)

theorem existsUnique_bucket_of_mem [DecidableEq α] [DecidableEq ι]
    {s : Finset α} {maskFrom maskTo : α → DivMask ι} {a : α} (ha : a ∈ s) :
    ∃! bucket, a ∈ bucketSet s maskFrom maskTo bucket := by
  refine ⟨transferBucket (maskFrom a) (maskTo a), ?_, ?_⟩
  · exact mem_bucketSet_iff.mpr ⟨ha, rfl⟩
  · intro bucket hb
    exact (mem_bucketSet_iff.mp hb).2.symm

def sharedAdmissibleCount [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι) : ℕ :=
  bucketCount s maskFrom maskTo .stableZero

def admissibleCountFrom [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι) : ℕ :=
  bucketCount s maskFrom maskTo .stableZero + bucketCount s maskFrom maskTo .lossZero

def admissibleCountTo [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι) : ℕ :=
  bucketCount s maskFrom maskTo .stableZero + bucketCount s maskFrom maskTo .gainZero

def admissibleDeltaCount [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι) : Int :=
  bucketCount s maskFrom maskTo .gainZero - bucketCount s maskFrom maskTo .lossZero

def sameMaskCount [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι) : ℕ :=
  bucketCount s maskFrom maskTo .stableZero + bucketCount s maskFrom maskTo .stableNonzero

def zeroUnionCount [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι) : ℕ :=
  bucketCount s maskFrom maskTo .stableZero
    + bucketCount s maskFrom maskTo .gainZero
    + bucketCount s maskFrom maskTo .lossZero

@[simp] theorem sharedAdmissibleCount_eq_stableZeroCount [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι) :
    sharedAdmissibleCount s maskFrom maskTo =
      bucketCount s maskFrom maskTo .stableZero := rfl

@[simp] theorem admissibleCountFrom_eq_shared_plus_loss [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι) :
    admissibleCountFrom s maskFrom maskTo =
      sharedAdmissibleCount s maskFrom maskTo + bucketCount s maskFrom maskTo .lossZero := by
  simp [admissibleCountFrom, sharedAdmissibleCount]

@[simp] theorem admissibleCountTo_eq_shared_plus_gain [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι) :
    admissibleCountTo s maskFrom maskTo =
      sharedAdmissibleCount s maskFrom maskTo + bucketCount s maskFrom maskTo .gainZero := by
  simp [admissibleCountTo, sharedAdmissibleCount]

@[simp] theorem admissibleDeltaCount_eq_gain_minus_loss [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι) :
    admissibleDeltaCount s maskFrom maskTo =
      bucketCount s maskFrom maskTo .gainZero - bucketCount s maskFrom maskTo .lossZero := rfl

@[simp] theorem sameMaskCount_eq_stableZero_plus_stableNonzero [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι) :
    sameMaskCount s maskFrom maskTo =
      bucketCount s maskFrom maskTo .stableZero
        + bucketCount s maskFrom maskTo .stableNonzero := rfl

@[simp] theorem zeroUnionCount_eq_stableZero_plus_gain_plus_loss [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι) :
    zeroUnionCount s maskFrom maskTo =
      bucketCount s maskFrom maskTo .stableZero
        + bucketCount s maskFrom maskTo .gainZero
        + bucketCount s maskFrom maskTo .lossZero := rfl

def signedBoolDelta (goodFrom goodTo : α → Bool) (a : α) : Int :=
  (if goodTo a then 1 else 0) - (if goodFrom a then 1 else 0)

def bucketSignedGoodDelta [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι)
    (goodFrom goodTo : α → Bool) (bucket : TransferBucket) : Int :=
  s.sum fun a =>
    if transferBucket (maskFrom a) (maskTo a) = bucket then
      signedBoolDelta goodFrom goodTo a
    else 0

def totalSignedGoodDelta [DecidableEq α]
    (s : Finset α) (goodFrom goodTo : α → Bool) : Int :=
  s.sum fun a => signedBoolDelta goodFrom goodTo a

theorem totalSignedGoodDelta_eq_sum_bucketSignedGoodDelta [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι)
    (goodFrom goodTo : α → Bool) :
    totalSignedGoodDelta s goodFrom goodTo =
      (Finset.univ : Finset TransferBucket).sum
        (fun bucket => bucketSignedGoodDelta s maskFrom maskTo goodFrom goodTo bucket) := by
  classical
  unfold totalSignedGoodDelta bucketSignedGoodDelta
  rw [Finset.sum_comm]
  refine Finset.sum_congr rfl ?_
  intro a ha
  cases h : transferBucket (maskFrom a) (maskTo a) <;> simp [signedBoolDelta]

namespace Examples

abbrev ExampleMask := DivMask (Fin 2)

def sampleFrom : Fin 5 → ExampleMask
  | ⟨0, _⟩ => ∅
  | ⟨1, _⟩ => ({0} : Finset (Fin 2))
  | ⟨2, _⟩ => ∅
  | ⟨3, _⟩ => ({0} : Finset (Fin 2))
  | ⟨4, _⟩ => ({0, 1} : Finset (Fin 2))

def sampleTo : Fin 5 → ExampleMask
  | ⟨0, _⟩ => ∅
  | ⟨1, _⟩ => ∅
  | ⟨2, _⟩ => ({1} : Finset (Fin 2))
  | ⟨3, _⟩ => ({0} : Finset (Fin 2))
  | ⟨4, _⟩ => ({1} : Finset (Fin 2))

example :
    transferBucket (sampleFrom ⟨0, by decide⟩) (sampleTo ⟨0, by decide⟩) = .stableZero := by
  native_decide

example :
    transferBucket (sampleFrom ⟨1, by decide⟩) (sampleTo ⟨1, by decide⟩) = .gainZero := by
  native_decide

example :
    transferBucket (sampleFrom ⟨2, by decide⟩) (sampleTo ⟨2, by decide⟩) = .lossZero := by
  native_decide

example :
    transferBucket (sampleFrom ⟨3, by decide⟩) (sampleTo ⟨3, by decide⟩) = .stableNonzero := by
  native_decide

example :
    transferBucket (sampleFrom ⟨4, by decide⟩) (sampleTo ⟨4, by decide⟩) = .nonzeroChurn := by
  native_decide

example :
    bucketCount (Finset.univ : Finset (Fin 5)) sampleFrom sampleTo .stableZero = 1 := by
  native_decide

example :
    zeroUnionCount (Finset.univ : Finset (Fin 5)) sampleFrom sampleTo = 3 := by
  native_decide

end Examples

end PrimeArithmetic.Structure
