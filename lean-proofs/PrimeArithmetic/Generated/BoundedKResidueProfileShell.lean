import Mathlib

namespace PrimeArithmetic.Generated.BoundedKResidueProfileShell

/-!
Generated-data shell for direct bounded-`k` residue-profile witnesses.

These artifacts do not carry candidate rows. They only store:
- the coprime-modulus residue profiles for two fixed lanes,
- the exact transfer-bucket counts already computed on the Rust side.

That is enough to certify honest positive or negative boundary facts such as:
- profile agreement fails,
- admissible equality fails,
- admissible delta is positive or nonpositive.
-/

structure GeneratedResidueProfileRow where
  modulus : Nat
  excludedSeedClassesFrom : List Nat
  excludedSeedClassesTo : List Nat
deriving DecidableEq, Repr

structure GeneratedResidueProfilePayload where
  base : ℕ
  middleLength : ℕ
  outer : ℕ
  inner : ℕ
  fromKOuter : ℕ
  fromKInner : ℕ
  toKOuter : ℕ
  toKInner : ℕ
  rows : List GeneratedResidueProfileRow
  stableZeroCount : ℕ
  gainZeroCount : ℕ
  lossZeroCount : ℕ
  stableNonzeroCount : ℕ
  nonzeroChurnCount : ℕ
deriving DecidableEq, Repr

def GeneratedResidueProfileRow.profileAgreement
    (row : GeneratedResidueProfileRow) : Bool :=
  row.excludedSeedClassesFrom = row.excludedSeedClassesTo

def GeneratedResidueProfilePayload.comparedModulusCount
    (payload : GeneratedResidueProfilePayload) : ℕ :=
  payload.rows.length

def GeneratedResidueProfilePayload.agreeingModulusCount
    (payload : GeneratedResidueProfilePayload) : ℕ :=
  payload.rows.countP GeneratedResidueProfileRow.profileAgreement

def GeneratedResidueProfilePayload.profileAgreement
    (payload : GeneratedResidueProfilePayload) : Prop :=
  ∀ row ∈ payload.rows, row.profileAgreement

def GeneratedResidueProfilePayload.profileAgreementBool
    (payload : GeneratedResidueProfilePayload) : Bool :=
  payload.rows.all GeneratedResidueProfileRow.profileAgreement

def GeneratedResidueProfilePayload.admissibleSetEqual
    (payload : GeneratedResidueProfilePayload) : Prop :=
  payload.gainZeroCount = 0 ∧ payload.lossZeroCount = 0

def GeneratedResidueProfilePayload.admissibleSetEqualBool
    (payload : GeneratedResidueProfilePayload) : Bool :=
  (payload.gainZeroCount == 0) && (payload.lossZeroCount == 0)

def GeneratedResidueProfilePayload.admissibleDeltaCount
    (payload : GeneratedResidueProfilePayload) : Int :=
  payload.gainZeroCount - payload.lossZeroCount

def GeneratedResidueProfilePayload.noPositiveAdmissibleDelta
    (payload : GeneratedResidueProfilePayload) : Prop :=
  payload.admissibleDeltaCount ≤ 0

def GeneratedResidueProfilePayload.noPositiveAdmissibleDeltaBool
    (payload : GeneratedResidueProfilePayload) : Bool :=
  if payload.admissibleDeltaCount ≤ 0 then true else false

@[simp] theorem GeneratedResidueProfilePayload.admissibleDeltaCount_eq_gain_minus_loss
    (payload : GeneratedResidueProfilePayload) :
    payload.admissibleDeltaCount = payload.gainZeroCount - payload.lossZeroCount := rfl

theorem GeneratedResidueProfilePayload.profileAgreement_iff_forall_rows
    (payload : GeneratedResidueProfilePayload) :
    payload.profileAgreement ↔
      ∀ row ∈ payload.rows, row.excludedSeedClassesFrom = row.excludedSeedClassesTo := by
  simp [GeneratedResidueProfilePayload.profileAgreement, GeneratedResidueProfileRow.profileAgreement]

@[simp] theorem GeneratedResidueProfilePayload.profileAgreementBool_eq_true_iff
    (payload : GeneratedResidueProfilePayload) :
    payload.profileAgreementBool = true ↔ payload.profileAgreement := by
  simp [GeneratedResidueProfilePayload.profileAgreementBool,
    GeneratedResidueProfilePayload.profileAgreement, List.all_eq_true]

@[simp] theorem GeneratedResidueProfilePayload.admissibleSetEqualBool_eq_true_iff
    (payload : GeneratedResidueProfilePayload) :
    payload.admissibleSetEqualBool = true ↔ payload.admissibleSetEqual := by
  simp [GeneratedResidueProfilePayload.admissibleSetEqualBool,
    GeneratedResidueProfilePayload.admissibleSetEqual]

@[simp] theorem GeneratedResidueProfilePayload.noPositiveAdmissibleDeltaBool_eq_true_iff
    (payload : GeneratedResidueProfilePayload) :
    payload.noPositiveAdmissibleDeltaBool = true ↔ payload.noPositiveAdmissibleDelta := by
  by_cases h : payload.admissibleDeltaCount ≤ 0 <;>
    simp [GeneratedResidueProfilePayload.noPositiveAdmissibleDeltaBool,
      GeneratedResidueProfilePayload.noPositiveAdmissibleDelta]

theorem GeneratedResidueProfilePayload.admissibleSetEqual_implies_noPositive
    (payload : GeneratedResidueProfilePayload)
    (hEqual : payload.admissibleSetEqual) :
    payload.noPositiveAdmissibleDelta := by
  rcases hEqual with ⟨hGain, hLoss⟩
  simp [GeneratedResidueProfilePayload.noPositiveAdmissibleDelta,
    GeneratedResidueProfilePayload.admissibleDeltaCount, hGain, hLoss]

end PrimeArithmetic.Generated.BoundedKResidueProfileShell
