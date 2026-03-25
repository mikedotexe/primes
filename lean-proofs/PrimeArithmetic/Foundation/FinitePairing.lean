import Mathlib

namespace PrimeArithmetic.Foundation

/-!
Generic finite symmetry data and pairing witnesses for the certification lane.
-/

structure SymmetryData (α : Type*) where
  mid : α
  inv : α → α
  invInvolutive : Function.Involutive inv
  invMid : inv mid = mid

namespace SymmetryData

variable {α : Type*} (symmetry : SymmetryData α)

@[simp] theorem inv_inv (a : α) : symmetry.inv (symmetry.inv a) = a :=
  symmetry.invInvolutive a

@[simp] theorem inv_mid : symmetry.inv symmetry.mid = symmetry.mid :=
  symmetry.invMid

end SymmetryData

structure PerfectPairing {α ι : Type*} (symmetry : SymmetryData α) (residue : ι → α) where
  mate : ι → ι
  mateInvolutive : Function.Involutive mate
  noFixed : ∀ i, mate i ≠ i
  equivariant : ∀ i, symmetry.inv (residue i) = residue (mate i)
  residueDistinct : ∀ i, residue (mate i) ≠ residue i

namespace PerfectPairing

variable {α ι : Type*} {symmetry : SymmetryData α} {residue : ι → α}
variable (pairing : PerfectPairing symmetry residue)

@[simp] theorem mate_mate (i : ι) : pairing.mate (pairing.mate i) = i :=
  pairing.mateInvolutive i

theorem mate_ne (i : ι) : pairing.mate i ≠ i :=
  pairing.noFixed i

theorem residue_ne_self (i : ι) : residue (pairing.mate i) ≠ residue i :=
  pairing.residueDistinct i

end PerfectPairing

end PrimeArithmetic.Foundation
