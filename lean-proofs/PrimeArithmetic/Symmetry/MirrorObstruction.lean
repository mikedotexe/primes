import PrimeArithmetic.Symmetry.CertificateReflection

namespace PrimeArithmetic.Symmetry.MirrorObstruction

open PrimeArithmetic.Symmetry.ModularReflection
open PrimeArithmetic.Symmetry.CertificateReflection

/-!
Mirror-symmetric residue families indexed by list reversal.

For an even-length family `residue : Fin (n + n) → Fin base`, if reversing the
index list corresponds to modular reflection `r ↦ -r mod base`, then excluding
the fixed residues `0` and `base / 2` yields the midpoint obstruction.

This is the Lean counterpart of the executable mirror-obstruction signal kept in
`agda-proofs/Theorems/MirrorObstruction.agda`, stated as an exact finite
symmetry theorem rather than as a boolean test.
-/

def mirrorCertificate {base n : ℕ} [NeZero base] (hEven : Even base)
    (residue : Fin (n + n) → Fin base)
    (hMirror : ∀ i, reflect base (residue i) = residue i.rev)
    (support : ObservedFixedPointExclusion residue) :
    ReflectionCertificate hEven residue where
  mate := Fin.rev
  mateInvolutive := by
    intro i
    simp
  equivariant := hMirror
  fixedPointExclusion := support

theorem midpoint_not_in_range_of_mirrorFamily {base n : ℕ} [NeZero base]
    (hEven : Even base) (residue : Fin (n + n) → Fin base)
    (hMirror : ∀ i, reflect base (residue i) = residue i.rev)
    (support : ObservedFixedPointExclusion residue) :
    midpoint base ∉ Set.range residue :=
  midpoint_not_in_range (mirrorCertificate hEven residue hMirror support)

theorem midpoint_not_visited_of_mirrorFamily {base n : ℕ} [NeZero base]
    (hEven : Even base) (residue : Fin (n + n) → Fin base)
    (hMirror : ∀ i, reflect base (residue i) = residue i.rev)
    (support : ObservedFixedPointExclusion residue) (i : Fin (n + n)) :
    residue i ≠ midpoint base :=
  midpoint_not_visited (mirrorCertificate hEven residue hMirror support) i

namespace Base10Example

def residues : Fin 4 → Fin 10
  | 0 => 1
  | 1 => 3
  | 2 => 7
  | _ => 9

theorem mirror_equivariant : ∀ i, reflect 10 (residues i) = residues i.rev := by
  intro i
  fin_cases i <;> decide

def support : ObservedFixedPointExclusion residues where
  zeroVoid := by
    intro i
    fin_cases i <;> decide
  midpointVoid := by
    intro i
    fin_cases i <;> decide

theorem midpoint_not_in_range : midpoint 10 ∉ Set.range residues :=
  midpoint_not_in_range_of_mirrorFamily (base := 10) (n := 2)
    (show Even 10 by decide) residues mirror_equivariant support

end Base10Example

end PrimeArithmetic.Symmetry.MirrorObstruction
