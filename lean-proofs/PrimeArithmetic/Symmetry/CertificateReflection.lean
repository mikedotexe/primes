import PrimeArithmetic.Symmetry.ModularReflection
import PrimeArithmetic.Symmetry.MidpointObstruction

namespace PrimeArithmetic.Symmetry.CertificateReflection

open PrimeArithmetic.Foundation
open PrimeArithmetic.Symmetry.ModularReflection

/-!
Reusable symmetry certificates for modular reflection.

This layer packages the concrete reflection arithmetic on `Fin base` into a
smaller certificate surface than `PerfectPairing`. A caller provides:

- a mate involution on the observed indices,
- equivariance with modular reflection on residues,
- exclusion of the fixed residues `0` and `base / 2`.

From these data we derive the full `PerfectPairing` witness and the abstract
midpoint obstruction automatically.
-/

structure ObservedFixedPointExclusion {base n : ℕ} [NeZero base]
    (residue : Fin n → Fin base) where
  zeroVoid : ∀ i, residue i ≠ 0
  midpointVoid : ∀ i, residue i ≠ midpoint base

theorem observedResiduesMove {base n : ℕ} [NeZero base]
    {residue : Fin n → Fin base} (support : ObservedFixedPointExclusion residue) :
    ∀ i, reflect base (residue i) ≠ residue i := by
  intro i hFix
  rcases eq_zero_or_eq_midpoint_of_fixed (base := base) (residue i) hFix with h | h
  · exact support.zeroVoid i h
  · exact support.midpointVoid i h

structure ReflectionCertificate {base n : ℕ} [NeZero base]
    (hEven : Even base) (residue : Fin n → Fin base) where
  mate : Fin n → Fin n
  mateInvolutive : Function.Involutive mate
  equivariant : ∀ i, reflect base (residue i) = residue (mate i)
  fixedPointExclusion : ObservedFixedPointExclusion residue

def pairing {base n : ℕ} [NeZero base] {hEven : Even base} {residue : Fin n → Fin base}
    (cert : ReflectionCertificate hEven residue) :
    PerfectPairing (symmetryData base hEven) residue where
  mate := cert.mate
  mateInvolutive := cert.mateInvolutive
  noFixed := by
    intro i hMate
    have hFix : reflect base (residue i) = residue i := by
      simpa [hMate] using cert.equivariant i
    exact observedResiduesMove cert.fixedPointExclusion i hFix
  equivariant := cert.equivariant
  residueDistinct := by
    intro i hResidue
    have hFix : reflect base (residue i) = residue i :=
      (cert.equivariant i).trans hResidue
    exact observedResiduesMove cert.fixedPointExclusion i hFix

theorem midpoint_not_visited {base n : ℕ} [NeZero base] {hEven : Even base}
    {residue : Fin n → Fin base} (cert : ReflectionCertificate hEven residue) (i : Fin n) :
    residue i ≠ midpoint base :=
  PrimeArithmetic.Symmetry.midpointNotVisited (pairing cert) i

theorem midpoint_not_in_range {base n : ℕ} [NeZero base] {hEven : Even base}
    {residue : Fin n → Fin base} (cert : ReflectionCertificate hEven residue) :
    midpoint base ∉ Set.range residue :=
  PrimeArithmetic.Symmetry.midpointNotInRange (pairing cert)

theorem zero_not_in_range {base n : ℕ} [NeZero base] {hEven : Even base}
    {residue : Fin n → Fin base} (cert : ReflectionCertificate hEven residue) :
    (0 : Fin base) ∉ Set.range residue := by
  intro hZero
  rcases hZero with ⟨i, hi⟩
  exact cert.fixedPointExclusion.zeroVoid i hi

end PrimeArithmetic.Symmetry.CertificateReflection
