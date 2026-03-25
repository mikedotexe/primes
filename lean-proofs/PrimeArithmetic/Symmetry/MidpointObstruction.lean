import PrimeArithmetic.Foundation.FinitePairing

namespace PrimeArithmetic.Symmetry

open PrimeArithmetic.Foundation

/-!
If a residue family is perfectly paired under an involution and every mate has a
distinct residue, the midpoint fixed by the involution cannot occur.
-/

section

variable {α ι : Type*}
variable {symmetry : SymmetryData α} {residue : ι → α}

theorem midpointNotVisited
    (pairing : PerfectPairing symmetry residue) (i : ι) :
    residue i ≠ symmetry.mid := by
  intro hMid
  have hMateMid : residue (pairing.mate i) = symmetry.mid := by
    calc
      residue (pairing.mate i) = symmetry.inv (residue i) := by
        symm
        exact pairing.equivariant i
      _ = symmetry.inv symmetry.mid := by simp [hMid]
      _ = symmetry.mid := symmetry.invMid
  exact pairing.residueDistinct i (hMateMid.trans hMid.symm)

theorem midpointNotInRange
    (pairing : PerfectPairing symmetry residue) :
    symmetry.mid ∉ Set.range residue := by
  intro hMid
  rcases hMid with ⟨i, hi⟩
  exact midpointNotVisited pairing i hi

end

end PrimeArithmetic.Symmetry
