import PrimeArithmetic.Symmetry.WindowCertificateErgonomics

namespace PrimeArithmetic.Symmetry.WindowCertificateExamples

open PrimeArithmetic.Symmetry.ModularReflection
open PrimeArithmetic.Symmetry.WindowCertificate
open PrimeArithmetic.Symmetry.WindowCertificateGenerated
open PrimeArithmetic.Symmetry.WindowCertificateErgonomics

/-!
Concrete finite certificate examples for the generated-data window shell.

These maintained examples exercise the artifact-facing Lean path:

- residues arrive as a concrete list,
- positions arrive as a concrete list,
- support counts are derived automatically,
- the caller supplies only balanced-count, fixed-point-exclusion, and
  pointwise-safety evidence.
-/

def base6Payload : GeneratedWindowPayload 6 where
  p := 5
  windowMid := 3
  radius := 2
  residues := [1, 5, 2, 4]
  positions := [1, 5, 0, 6]

abbrev base6Window : WindowData 6 base6Payload.residues.length :=
  base6Payload.windowData

theorem base6Balanced :
    ∀ r, base6Payload.derivedCount r = base6Payload.derivedCount (reflect 6 r) := by
  intro r
  fin_cases r <;> native_decide

def base6FixedPointExclusion :
    PrimeArithmetic.Symmetry.CertificateReflection.ObservedFixedPointExclusion
      base6Payload.residueFn where
  zeroVoid := by
    intro i
    fin_cases i <;> native_decide
  midpointVoid := by
    intro i
    fin_cases i <;> native_decide

theorem base6PointwiseSafe :
    PointwiseSafe base6Payload.radius base6Payload.windowMid base6Payload.positions := by
  intro x hx
  simp [base6Payload, SafePos] at hx ⊢
  rcases hx with rfl | rfl | rfl | rfl
  all_goals native_decide

def base6Evidence : GeneratedDualEvidence base6Payload where
  balanced := base6Balanced
  fixedPointExclusion := base6FixedPointExclusion
  pointwiseSafe := base6PointwiseSafe

theorem base6_midpoint_not_in_range :
    midpoint 6 ∉ Set.range base6Window.residue :=
  base6Evidence.midpoint_not_in_range (hEven := show Even 6 by native_decide)

theorem base6_zero_not_in_range :
    (0 : Fin 6) ∉ Set.range base6Window.residue :=
  base6Evidence.zero_not_in_range (hEven := show Even 6 by native_decide)

theorem base6_inviolability :
    InZone base6Window.radius base6Window.windowMid base6Window.positions → False :=
  base6Evidence.inviolability (hEven := show Even 6 by native_decide)

def base10Payload : GeneratedWindowPayload 10 where
  p := 11
  windowMid := 5
  radius := 2
  residues := [1, 9, 3, 7]
  positions := [1, 3, 7, 9]

abbrev base10Window : WindowData 10 base10Payload.residues.length :=
  base10Payload.windowData

theorem base10Balanced :
    ∀ r, base10Payload.derivedCount r = base10Payload.derivedCount (reflect 10 r) := by
  intro r
  fin_cases r <;> native_decide

def base10FixedPointExclusion :
    PrimeArithmetic.Symmetry.CertificateReflection.ObservedFixedPointExclusion
      base10Payload.residueFn where
  zeroVoid := by
    intro i
    fin_cases i <;> native_decide
  midpointVoid := by
    intro i
    fin_cases i <;> native_decide

theorem base10PointwiseSafe :
    PointwiseSafe base10Payload.radius base10Payload.windowMid base10Payload.positions := by
  intro x hx
  simp [base10Payload, SafePos] at hx ⊢
  rcases hx with rfl | rfl | rfl | rfl
  all_goals native_decide

def base10Evidence : GeneratedDualEvidence base10Payload where
  balanced := base10Balanced
  fixedPointExclusion := base10FixedPointExclusion
  pointwiseSafe := base10PointwiseSafe

theorem base10_midpoint_not_in_range :
    midpoint 10 ∉ Set.range base10Window.residue :=
  base10Evidence.midpoint_not_in_range (hEven := show Even 10 by native_decide)

theorem base10_zero_not_in_range :
    (0 : Fin 10) ∉ Set.range base10Window.residue :=
  base10Evidence.zero_not_in_range (hEven := show Even 10 by native_decide)

theorem base10_inviolability :
    InZone base10Window.radius base10Window.windowMid base10Window.positions → False :=
  base10Evidence.inviolability (hEven := show Even 10 by native_decide)

end PrimeArithmetic.Symmetry.WindowCertificateExamples
