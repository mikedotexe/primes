import PrimeArithmetic.Symmetry.WindowCertificate

namespace PrimeArithmetic.Symmetry.WindowCertificateExamples

open PrimeArithmetic.Symmetry.ModularReflection
open PrimeArithmetic.Symmetry.CertificateReflection
open PrimeArithmetic.Symmetry.BalancedBucketReflection
open PrimeArithmetic.Symmetry.WindowCertificate

/-!
Concrete finite certificate examples for the balanced-bucket and window shell.

These are small maintained examples that exercise the full Lean certificate
pipeline on explicit finite data:

- a base-6 residue family aligned with the certified symmetry witness,
- a base-10 residue family with the same reflected-bucket shape.
-/

def base6Residue : Fin 4 → Fin 6
  | i =>
      match i.1 with
      | 0 => 1
      | 1 => 5
      | 2 => 2
      | _ => 4

def base6Count : Fin 6 → ℕ
  | r =>
      match r.1 with
      | 1 => 1
      | 2 => 1
      | 4 => 1
      | 5 => 1
      | _ => 0

theorem base6SupportCounts : SupportCountsAgree base6Residue base6Count := by
  intro r
  fin_cases r <;> native_decide

def base6Balanced : BalancedBuckets base6Residue base6Count where
  balanced := by
    intro r
    fin_cases r <;> native_decide

def base6FixedPointExclusion : ObservedFixedPointExclusion base6Residue where
  zeroVoid := by
    intro i
    fin_cases i <;> native_decide
  midpointVoid := by
    intro i
    fin_cases i <;> native_decide

def base6Window : WindowData 6 4 where
  p := 5
  windowMid := 3
  radius := 2
  residue := base6Residue
  positions := [1, 5, 0, 6]
  count := base6Count

def base6StaticContracts : StaticContracts base6Window where
  supportCounts := base6SupportCounts
  fixedPointExclusion := base6FixedPointExclusion

theorem base6PointwiseSafe :
    PointwiseSafe base6Window.radius base6Window.windowMid base6Window.positions := by
  intro x hx
  simp [base6Window, SafePos] at hx ⊢
  rcases hx with rfl | rfl | rfl | rfl
  all_goals native_decide

def base6DualCertificate : DualCertificate (show Even 6 by native_decide) base6Window :=
  buildDualCertificate
    (show Even 6 by native_decide)
    base6Window
    base6StaticContracts
    base6Balanced
    base6PointwiseSafe

theorem base6_midpoint_not_in_range :
    midpoint 6 ∉ Set.range base6Window.residue :=
  dual_midpoint_not_in_range base6DualCertificate

theorem base6_zero_not_in_range :
    (0 : Fin 6) ∉ Set.range base6Window.residue :=
  zero_not_in_range base6DualCertificate.static

theorem base6_inviolability :
    InZone base6Window.radius base6Window.windowMid base6Window.positions → False :=
  dual_inviolability base6DualCertificate

def base10Residue : Fin 4 → Fin 10
  | i =>
      match i.1 with
      | 0 => 1
      | 1 => 9
      | 2 => 3
      | _ => 7

def base10Count : Fin 10 → ℕ
  | r =>
      match r.1 with
      | 1 => 1
      | 3 => 1
      | 7 => 1
      | 9 => 1
      | _ => 0

theorem base10SupportCounts : SupportCountsAgree base10Residue base10Count := by
  intro r
  fin_cases r <;> native_decide

def base10Balanced : BalancedBuckets base10Residue base10Count where
  balanced := by
    intro r
    fin_cases r <;> native_decide

def base10FixedPointExclusion : ObservedFixedPointExclusion base10Residue where
  zeroVoid := by
    intro i
    fin_cases i <;> native_decide
  midpointVoid := by
    intro i
    fin_cases i <;> native_decide

def base10Window : WindowData 10 4 where
  p := 11
  windowMid := 5
  radius := 2
  residue := base10Residue
  positions := [1, 3, 7, 9]
  count := base10Count

def base10StaticContracts : StaticContracts base10Window where
  supportCounts := base10SupportCounts
  fixedPointExclusion := base10FixedPointExclusion

theorem base10PointwiseSafe :
    PointwiseSafe base10Window.radius base10Window.windowMid base10Window.positions := by
  intro x hx
  simp [base10Window, SafePos] at hx ⊢
  rcases hx with rfl | rfl | rfl | rfl
  all_goals native_decide

def base10DualCertificate : DualCertificate (show Even 10 by native_decide) base10Window :=
  buildDualCertificate
    (show Even 10 by native_decide)
    base10Window
    base10StaticContracts
    base10Balanced
    base10PointwiseSafe

theorem base10_midpoint_not_in_range :
    midpoint 10 ∉ Set.range base10Window.residue :=
  dual_midpoint_not_in_range base10DualCertificate

theorem base10_zero_not_in_range :
    (0 : Fin 10) ∉ Set.range base10Window.residue :=
  zero_not_in_range base10DualCertificate.static

theorem base10_inviolability :
    InZone base10Window.radius base10Window.windowMid base10Window.positions → False :=
  dual_inviolability base10DualCertificate

end PrimeArithmetic.Symmetry.WindowCertificateExamples
