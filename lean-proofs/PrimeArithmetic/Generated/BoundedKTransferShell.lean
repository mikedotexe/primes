import PrimeArithmetic.Structure.FiniteMaskTransfer

namespace PrimeArithmetic.Generated.BoundedKTransferShell

open PrimeArithmetic.Structure

/-!
Generated-data shell for exact bounded-`k` mask-transfer witnesses.

This mirrors the existing generated window-certificate lane, but for aligned
`k=(0,0) → best_k` transfer rows:

- each row stores the exact `from` and `to` divisibility masks,
- admissibility is the empty-mask condition,
- prime flags are kept only as booleans,
- all exported conclusions remain finite exact counts or integer sign facts.

Report-layer labels such as "stable_zero_led" or "boundary_led" stay outside
this module. Lean only sees the exact bucket counts and signed deltas.
-/

structure GeneratedTransferRow where
  middleIndex : ℕ
  maskFrom : Nat
  maskTo : Nat
  primeFrom : Bool
  primeTo : Bool
deriving DecidableEq, Repr

structure GeneratedTransferPayload where
  base : ℕ
  middleLength : ℕ
  outer : ℕ
  inner : ℕ
  fromKOuter : ℕ
  fromKInner : ℕ
  toKOuter : ℕ
  toKInner : ℕ
  prefilterPrimes : List ℕ
  rows : List GeneratedTransferRow
deriving DecidableEq, Repr

abbrev GeneratedTransferPayload.rowIndex (payload : GeneratedTransferPayload) :=
  Fin payload.rows.length

def GeneratedTransferPayload.row
    (payload : GeneratedTransferPayload) (i : payload.rowIndex) : GeneratedTransferRow :=
  payload.rows.get i

def GeneratedTransferPayload.decodeMask
    (payload : GeneratedTransferPayload) (mask : Nat) :
    DivMask (Fin payload.prefilterPrimes.length) :=
  Finset.univ.filter fun i => Nat.testBit mask i.1

def GeneratedTransferPayload.maskFromFn
    (payload : GeneratedTransferPayload) (i : payload.rowIndex) :
    DivMask (Fin payload.prefilterPrimes.length) :=
  payload.decodeMask (payload.row i).maskFrom

def GeneratedTransferPayload.maskToFn
    (payload : GeneratedTransferPayload) (i : payload.rowIndex) :
    DivMask (Fin payload.prefilterPrimes.length) :=
  payload.decodeMask (payload.row i).maskTo

def GeneratedTransferPayload.primeFromFn
    (payload : GeneratedTransferPayload) (i : payload.rowIndex) : Bool :=
  (payload.row i).primeFrom

def GeneratedTransferPayload.primeToFn
    (payload : GeneratedTransferPayload) (i : payload.rowIndex) : Bool :=
  (payload.row i).primeTo

def GeneratedTransferPayload.candidateSet
    (payload : GeneratedTransferPayload) : Finset payload.rowIndex :=
  Finset.univ

def GeneratedTransferPayload.transferBucketCount
    (payload : GeneratedTransferPayload) (bucket : TransferBucket) : ℕ :=
  PrimeArithmetic.Structure.bucketCount
    payload.candidateSet payload.maskFromFn payload.maskToFn bucket

def GeneratedTransferPayload.stableZeroCount
    (payload : GeneratedTransferPayload) : ℕ :=
  payload.transferBucketCount .stableZero

def GeneratedTransferPayload.gainZeroCount
    (payload : GeneratedTransferPayload) : ℕ :=
  payload.transferBucketCount .gainZero

def GeneratedTransferPayload.lossZeroCount
    (payload : GeneratedTransferPayload) : ℕ :=
  payload.transferBucketCount .lossZero

def GeneratedTransferPayload.stableNonzeroCount
    (payload : GeneratedTransferPayload) : ℕ :=
  payload.transferBucketCount .stableNonzero

def GeneratedTransferPayload.nonzeroChurnCount
    (payload : GeneratedTransferPayload) : ℕ :=
  payload.transferBucketCount .nonzeroChurn

def GeneratedTransferPayload.sharedAdmissibleCount
    (payload : GeneratedTransferPayload) : ℕ :=
  PrimeArithmetic.Structure.sharedAdmissibleCount
    payload.candidateSet payload.maskFromFn payload.maskToFn

def GeneratedTransferPayload.admissibleCountFrom
    (payload : GeneratedTransferPayload) : ℕ :=
  PrimeArithmetic.Structure.admissibleCountFrom
    payload.candidateSet payload.maskFromFn payload.maskToFn

def GeneratedTransferPayload.admissibleCountTo
    (payload : GeneratedTransferPayload) : ℕ :=
  PrimeArithmetic.Structure.admissibleCountTo
    payload.candidateSet payload.maskFromFn payload.maskToFn

def GeneratedTransferPayload.admissibleDeltaCount
    (payload : GeneratedTransferPayload) : Int :=
  PrimeArithmetic.Structure.admissibleDeltaCount
    payload.candidateSet payload.maskFromFn payload.maskToFn

def GeneratedTransferPayload.sameMaskCount
    (payload : GeneratedTransferPayload) : ℕ :=
  PrimeArithmetic.Structure.sameMaskCount
    payload.candidateSet payload.maskFromFn payload.maskToFn

def GeneratedTransferPayload.zeroUnionCount
    (payload : GeneratedTransferPayload) : ℕ :=
  PrimeArithmetic.Structure.zeroUnionCount
    payload.candidateSet payload.maskFromFn payload.maskToFn

def GeneratedTransferPayload.bucketPrimeDeltaCount
    (payload : GeneratedTransferPayload) (bucket : TransferBucket) : Int :=
  PrimeArithmetic.Structure.bucketSignedGoodDelta
    payload.candidateSet payload.maskFromFn payload.maskToFn
    payload.primeFromFn payload.primeToFn bucket

def GeneratedTransferPayload.stableZeroPrimeDeltaCount
    (payload : GeneratedTransferPayload) : Int :=
  payload.bucketPrimeDeltaCount .stableZero

def GeneratedTransferPayload.boundaryPrimeDeltaCount
    (payload : GeneratedTransferPayload) : Int :=
  payload.bucketPrimeDeltaCount .gainZero + payload.bucketPrimeDeltaCount .lossZero

@[simp] theorem GeneratedTransferPayload.sharedAdmissibleCount_eq_stableZeroCount
    (payload : GeneratedTransferPayload) :
    payload.sharedAdmissibleCount = payload.stableZeroCount := rfl

@[simp] theorem GeneratedTransferPayload.admissibleCountFrom_eq_stableZero_plus_loss
    (payload : GeneratedTransferPayload) :
    payload.admissibleCountFrom = payload.stableZeroCount + payload.lossZeroCount := rfl

@[simp] theorem GeneratedTransferPayload.admissibleCountTo_eq_stableZero_plus_gain
    (payload : GeneratedTransferPayload) :
    payload.admissibleCountTo = payload.stableZeroCount + payload.gainZeroCount := rfl

@[simp] theorem GeneratedTransferPayload.admissibleDeltaCount_eq_gain_minus_loss
    (payload : GeneratedTransferPayload) :
    payload.admissibleDeltaCount =
      payload.gainZeroCount - payload.lossZeroCount := rfl

@[simp] theorem GeneratedTransferPayload.sameMaskCount_eq_stableZero_plus_stableNonzero
    (payload : GeneratedTransferPayload) :
    payload.sameMaskCount = payload.stableZeroCount + payload.stableNonzeroCount := rfl

@[simp] theorem GeneratedTransferPayload.zeroUnionCount_eq_stableZero_plus_gain_plus_loss
    (payload : GeneratedTransferPayload) :
    payload.zeroUnionCount =
      payload.stableZeroCount + payload.gainZeroCount + payload.lossZeroCount := rfl

theorem GeneratedTransferPayload.totalPrimeDelta_eq_sum_bucketPrimeDelta
    (payload : GeneratedTransferPayload) :
    PrimeArithmetic.Structure.totalSignedGoodDelta
        payload.candidateSet payload.primeFromFn payload.primeToFn =
      (Finset.univ : Finset TransferBucket).sum
        (fun bucket => payload.bucketPrimeDeltaCount bucket) := by
  simpa [GeneratedTransferPayload.bucketPrimeDeltaCount] using
    PrimeArithmetic.Structure.totalSignedGoodDelta_eq_sum_bucketSignedGoodDelta
      payload.candidateSet payload.maskFromFn payload.maskToFn
      payload.primeFromFn payload.primeToFn

end PrimeArithmetic.Generated.BoundedKTransferShell
