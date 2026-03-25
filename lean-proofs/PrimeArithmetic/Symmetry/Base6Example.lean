import PrimeArithmetic.Symmetry.MidpointObstruction

namespace PrimeArithmetic.Symmetry.Base6Example

open PrimeArithmetic.Foundation

/-!
Lean analogue of `agda-proofs/Examples/CertifiedResonanceComplete.agda`.

The residue list `{1, 5, 2, 4}` is perfectly paired in base 6, so the midpoint
residue `3` is excluded.
-/

inductive Residue6 where
  | r0 | r1 | r2 | r3 | r4 | r5
  deriving DecidableEq, Fintype, Repr

inductive Occurrence4 where
  | o0 | o1 | o2 | o3
  deriving DecidableEq, Fintype, Repr

def midpoint : Residue6 := .r3

def reflect : Residue6 → Residue6
  | .r0 => .r0
  | .r1 => .r5
  | .r2 => .r4
  | .r3 => .r3
  | .r4 => .r2
  | .r5 => .r1

theorem reflectInvolutive : Function.Involutive reflect := by
  intro residue
  cases residue <;> rfl

def symmetryData : SymmetryData Residue6 where
  mid := midpoint
  inv := reflect
  invInvolutive := reflectInvolutive
  invMid := rfl

def residues : Occurrence4 → Residue6
  | .o0 => .r1
  | .o1 => .r5
  | .o2 => .r2
  | .o3 => .r4

def mate : Occurrence4 → Occurrence4
  | .o0 => .o1
  | .o1 => .o0
  | .o2 => .o3
  | .o3 => .o2

theorem mateInvolutive : Function.Involutive mate := by
  intro occurrence
  cases occurrence <;> rfl

theorem mateNoFixed : ∀ occurrence, mate occurrence ≠ occurrence := by
  intro occurrence
  cases occurrence <;> simp [mate]

theorem residueEquivariant :
    ∀ occurrence, symmetryData.inv (residues occurrence) = residues (mate occurrence) := by
  intro occurrence
  cases occurrence <;> rfl

theorem residueDistinct :
    ∀ occurrence, residues (mate occurrence) ≠ residues occurrence := by
  intro occurrence
  cases occurrence <;> simp [mate, residues]

def pairing : PerfectPairing symmetryData residues where
  mate := mate
  mateInvolutive := mateInvolutive
  noFixed := mateNoFixed
  equivariant := residueEquivariant
  residueDistinct := residueDistinct

theorem certifiedHonoraryZero (occurrence : Occurrence4) :
    residues occurrence ≠ midpoint :=
  PrimeArithmetic.Symmetry.midpointNotVisited pairing occurrence

theorem midpointNotInResidueRange : midpoint ∉ Set.range residues :=
  PrimeArithmetic.Symmetry.midpointNotInRange pairing

end PrimeArithmetic.Symmetry.Base6Example
