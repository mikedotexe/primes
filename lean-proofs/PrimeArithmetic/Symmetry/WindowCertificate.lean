import PrimeArithmetic.Symmetry.BalancedBucketReflection

namespace PrimeArithmetic.Symmetry.WindowCertificate

open PrimeArithmetic.Symmetry.ModularReflection
open PrimeArithmetic.Symmetry.CertificateReflection
open PrimeArithmetic.Symmetry.BalancedBucketReflection

/-!
Window-level static and dynamic certificate shells.

This module keeps the Lean window certificate surface deliberately narrow:

- the static side is a balanced-bucket reflection certificate on the extracted
  residues,
- the dynamic side is a pointwise midpoint-radius safety contract on positions,
- the combined certificate exposes midpoint exclusion and inviolability
  together.

This mirrors the clean builder role of the Agda `WindowCertificate` layer
without importing a larger orbital API than the current Lean package needs.
-/

def SafePos (radius midpoint x : ℕ) : Prop :=
  radius ≤ Nat.dist x midpoint

def PointwiseSafe (radius midpoint : ℕ) (positions : List ℕ) : Prop :=
  ∀ ⦃x : ℕ⦄, x ∈ positions → SafePos radius midpoint x

def InZone (radius midpoint : ℕ) (positions : List ℕ) : Prop :=
  ∃ x, x ∈ positions ∧ Nat.dist x midpoint < radius

theorem pointwiseSafeNil {radius midpoint : ℕ} :
    PointwiseSafe radius midpoint [] := by
  intro x hx
  cases hx

theorem pointwiseSafeCons {radius midpoint x : ℕ} {positions : List ℕ}
    (hx : SafePos radius midpoint x)
    (hrest : PointwiseSafe radius midpoint positions) :
    PointwiseSafe radius midpoint (x :: positions) := by
  intro y hy
  rcases List.mem_cons.1 hy with rfl | hyTail
  · exact hx
  · exact hrest hyTail

theorem pointwiseSafeSingleton {radius midpoint x : ℕ}
    (hx : SafePos radius midpoint x) :
    PointwiseSafe radius midpoint [x] :=
  pointwiseSafeCons hx pointwiseSafeNil

theorem pointwiseSafeFromAll {radius midpoint : ℕ} {positions : List ℕ}
    (hAll : ∀ ⦃x : ℕ⦄, x ∈ positions → SafePos radius midpoint x) :
    PointwiseSafe radius midpoint positions :=
  hAll

theorem inviolabilityFromPointwiseSafe {radius midpoint : ℕ} {positions : List ℕ}
    (safe : PointwiseSafe radius midpoint positions) :
    InZone radius midpoint positions → False := by
  rintro ⟨x, hx, hlt⟩
  exact Nat.not_lt_of_ge (safe hx) hlt

structure WindowData (base n : ℕ) [NeZero base] where
  p : ℕ
  windowMid : ℕ
  radius : ℕ
  residue : Fin n → Fin base
  positions : List ℕ
  count : Fin base → ℕ

structure StaticContracts {base n : ℕ} [NeZero base]
    (W : WindowData base n) where
  supportCounts : SupportCountsAgree W.residue W.count
  fixedPointExclusion : ObservedFixedPointExclusion W.residue

structure StaticCertificate {base n : ℕ} [NeZero base]
    (hEven : Even base) (W : WindowData base n) where
  balancedWitness : BalancedBuckets W.residue W.count
  staticContracts : StaticContracts W

def staticReflectionCertificate {base n : ℕ} [NeZero base] {hEven : Even base}
    {W : WindowData base n} (cert : StaticCertificate hEven W) :
    BalancedBucketReflectionCertificate hEven W.residue where
  count := W.count
  balancedWitness := cert.balancedWitness
  supportCounts := cert.staticContracts.supportCounts
  fixedPointExclusion := cert.staticContracts.fixedPointExclusion

theorem midpoint_not_in_range {base n : ℕ} [NeZero base] {hEven : Even base}
    {W : WindowData base n} (cert : StaticCertificate hEven W) :
    midpoint base ∉ Set.range W.residue :=
  BalancedBucketReflection.midpoint_not_in_range (staticReflectionCertificate cert)

theorem zero_not_in_range {base n : ℕ} [NeZero base] {hEven : Even base}
    {W : WindowData base n} (cert : StaticCertificate hEven W) :
    (0 : Fin base) ∉ Set.range W.residue :=
  BalancedBucketReflection.zero_not_in_range (staticReflectionCertificate cert)

structure DynamicCertificate {base n : ℕ} [NeZero base]
    (W : WindowData base n) where
  pointwiseSafe : PointwiseSafe W.radius W.windowMid W.positions

theorem inviolability {base n : ℕ} [NeZero base]
    {W : WindowData base n} (cert : DynamicCertificate W) :
    InZone W.radius W.windowMid W.positions → False :=
  inviolabilityFromPointwiseSafe cert.pointwiseSafe

structure DualCertificate {base n : ℕ} [NeZero base]
    (hEven : Even base) (W : WindowData base n) where
  static : StaticCertificate hEven W
  dynamic : DynamicCertificate W

theorem dual_midpoint_not_in_range {base n : ℕ} [NeZero base] {hEven : Even base}
    {W : WindowData base n} (cert : DualCertificate hEven W) :
    midpoint base ∉ Set.range W.residue :=
  midpoint_not_in_range cert.static

theorem dual_inviolability {base n : ℕ} [NeZero base] {hEven : Even base}
    {W : WindowData base n} (cert : DualCertificate hEven W) :
    InZone W.radius W.windowMid W.positions → False :=
  inviolability cert.dynamic

def buildDualCertificate {base n : ℕ} [NeZero base]
    (hEven : Even base) (W : WindowData base n)
    (contracts : StaticContracts W)
    (balanced : BalancedBuckets W.residue W.count)
    (safe : PointwiseSafe W.radius W.windowMid W.positions) :
    DualCertificate hEven W where
  static := {
    balancedWitness := balanced
    staticContracts := contracts
  }
  dynamic := {
    pointwiseSafe := safe
  }

end PrimeArithmetic.Symmetry.WindowCertificate
