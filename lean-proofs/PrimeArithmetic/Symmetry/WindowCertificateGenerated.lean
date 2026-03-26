import PrimeArithmetic.Symmetry.WindowCertificate

namespace PrimeArithmetic.Symmetry.WindowCertificateGenerated

open PrimeArithmetic.Symmetry.ModularReflection
open PrimeArithmetic.Symmetry.BalancedBucketSupport
open PrimeArithmetic.Symmetry.BalancedBucketReflection
open PrimeArithmetic.Symmetry.WindowCertificate

/-!
Generated-data entrypoints for the window-certificate shell.

The core `WindowCertificate` layer is stated in terms of a residue function
`Fin n → Fin base`, a count function, and a list of absolute positions. Runtime
or offline extraction pipelines, however, usually produce:

- a concrete list of observed residues,
- a concrete list of absolute positions,
- scalar window metadata such as the midpoint and exclusion radius.

This module bridges those shapes directly into the maintained Lean shell:

- the residue list is reinterpreted as `Fin length → Fin base`,
- the bucket counts are derived automatically from `supportList`,
- support-count agreement becomes definitional,
- callers only need to supply balanced-count, fixed-point-exclusion, and
  pointwise-safety evidence.
-/

structure GeneratedWindowPayload (base : ℕ) [NeZero base] where
  p : ℕ
  windowMid : ℕ
  radius : ℕ
  residues : List (Fin base)
  positions : List ℕ

def GeneratedWindowPayload.residueFn {base : ℕ} [NeZero base]
    (payload : GeneratedWindowPayload base) :
    Fin payload.residues.length → Fin base :=
  fun i => payload.residues.get i

def GeneratedWindowPayload.derivedCount {base : ℕ} [NeZero base]
    (payload : GeneratedWindowPayload base) :
    Fin base → ℕ :=
  fun r => (supportList payload.residueFn r).length

theorem GeneratedWindowPayload.supportCountsAgree {base : ℕ} [NeZero base]
    (payload : GeneratedWindowPayload base) :
    SupportCountsAgree payload.residueFn payload.derivedCount := by
  intro r
  rfl

def GeneratedWindowPayload.windowData {base : ℕ} [NeZero base]
    (payload : GeneratedWindowPayload base) :
    WindowData base payload.residues.length where
  p := payload.p
  windowMid := payload.windowMid
  radius := payload.radius
  residue := payload.residueFn
  positions := payload.positions
  count := payload.derivedCount

def GeneratedWindowPayload.staticContracts {base : ℕ} [NeZero base]
    (payload : GeneratedWindowPayload base)
    (fixedPointExclusion :
      PrimeArithmetic.Symmetry.CertificateReflection.ObservedFixedPointExclusion
        payload.residueFn) :
    StaticContracts payload.windowData where
  supportCounts := payload.supportCountsAgree
  fixedPointExclusion := fixedPointExclusion

def GeneratedWindowPayload.staticCertificate {base : ℕ} [NeZero base]
    (payload : GeneratedWindowPayload base) (hEven : Even base)
    (balanced :
      ∀ r, payload.derivedCount r = payload.derivedCount (reflect base r))
    (fixedPointExclusion :
      PrimeArithmetic.Symmetry.CertificateReflection.ObservedFixedPointExclusion
        payload.residueFn) :
    StaticCertificate hEven payload.windowData where
  balancedWitness := {
    balanced := balanced
  }
  staticContracts := payload.staticContracts fixedPointExclusion

def GeneratedWindowPayload.dynamicCertificate {base : ℕ} [NeZero base]
    (payload : GeneratedWindowPayload base)
    (safe : PointwiseSafe payload.radius payload.windowMid payload.positions) :
    DynamicCertificate payload.windowData where
  pointwiseSafe := safe

def GeneratedWindowPayload.dualCertificate {base : ℕ} [NeZero base]
    (payload : GeneratedWindowPayload base) (hEven : Even base)
    (balanced :
      ∀ r, payload.derivedCount r = payload.derivedCount (reflect base r))
    (fixedPointExclusion :
      PrimeArithmetic.Symmetry.CertificateReflection.ObservedFixedPointExclusion
        payload.residueFn)
    (safe : PointwiseSafe payload.radius payload.windowMid payload.positions) :
    DualCertificate hEven payload.windowData where
  static := payload.staticCertificate hEven balanced fixedPointExclusion
  dynamic := payload.dynamicCertificate safe

theorem generated_midpoint_not_in_range {base : ℕ} [NeZero base]
    {payload : GeneratedWindowPayload base} {hEven : Even base}
    (balanced :
      ∀ r, payload.derivedCount r = payload.derivedCount (reflect base r))
    (fixedPointExclusion :
      PrimeArithmetic.Symmetry.CertificateReflection.ObservedFixedPointExclusion
        payload.residueFn) :
    midpoint base ∉ Set.range payload.windowData.residue :=
  midpoint_not_in_range (payload.staticCertificate hEven balanced fixedPointExclusion)

theorem generated_zero_not_in_range {base : ℕ} [NeZero base]
    {payload : GeneratedWindowPayload base} {hEven : Even base}
    (balanced :
      ∀ r, payload.derivedCount r = payload.derivedCount (reflect base r))
    (fixedPointExclusion :
      PrimeArithmetic.Symmetry.CertificateReflection.ObservedFixedPointExclusion
        payload.residueFn) :
    (0 : Fin base) ∉ Set.range payload.windowData.residue :=
  zero_not_in_range (payload.staticCertificate hEven balanced fixedPointExclusion)

theorem generated_inviolability {base : ℕ} [NeZero base]
    {payload : GeneratedWindowPayload base}
    (safe : PointwiseSafe payload.radius payload.windowMid payload.positions) :
    InZone payload.radius payload.windowMid payload.positions → False :=
  inviolability (payload.dynamicCertificate safe)

end PrimeArithmetic.Symmetry.WindowCertificateGenerated
