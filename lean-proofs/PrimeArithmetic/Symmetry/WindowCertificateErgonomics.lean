import PrimeArithmetic.Symmetry.WindowCertificateGenerated

namespace PrimeArithmetic.Symmetry.WindowCertificateErgonomics

open PrimeArithmetic.Symmetry.ModularReflection
open PrimeArithmetic.Symmetry.WindowCertificate
open PrimeArithmetic.Symmetry.WindowCertificateGenerated

/-!
Small ergonomics wrappers for generated window certificates.

`WindowCertificateGenerated` already gives the arithmetic bridge from residue
lists and position lists into the maintained certificate shell. This module
adds one further packaging layer for exported artifacts:

- static evidence is bundled once as balanced counts plus fixed-point exclusion,
- dynamic evidence extends it with pointwise midpoint-radius safety,
- the corresponding static or dual certificates are rebuilt from that single
  evidence object.

This keeps the theorem boundary unchanged while making generated certificate
files or imported artifacts less verbose.
-/

structure GeneratedStaticEvidence {base : ℕ} [NeZero base]
    (payload : GeneratedWindowPayload base) where
  balanced : ∀ r, payload.derivedCount r = payload.derivedCount (reflect base r)
  fixedPointExclusion :
    PrimeArithmetic.Symmetry.CertificateReflection.ObservedFixedPointExclusion
      payload.residueFn

def GeneratedStaticEvidence.staticCertificate {base : ℕ} [NeZero base]
    {payload : GeneratedWindowPayload base} {hEven : Even base}
    (evidence : GeneratedStaticEvidence payload) :
    StaticCertificate hEven payload.windowData :=
  payload.staticCertificate hEven evidence.balanced evidence.fixedPointExclusion

theorem GeneratedStaticEvidence.midpoint_not_in_range {base : ℕ} [NeZero base]
    {payload : GeneratedWindowPayload base} {hEven : Even base}
    (evidence : GeneratedStaticEvidence payload) :
    midpoint base ∉ Set.range payload.windowData.residue :=
  WindowCertificate.midpoint_not_in_range (evidence.staticCertificate (hEven := hEven))

theorem GeneratedStaticEvidence.zero_not_in_range {base : ℕ} [NeZero base]
    {payload : GeneratedWindowPayload base} {hEven : Even base}
    (evidence : GeneratedStaticEvidence payload) :
    (0 : Fin base) ∉ Set.range payload.windowData.residue :=
  WindowCertificate.zero_not_in_range (evidence.staticCertificate (hEven := hEven))

structure GeneratedDualEvidence {base : ℕ} [NeZero base]
    (payload : GeneratedWindowPayload base)
    extends GeneratedStaticEvidence payload where
  pointwiseSafe : PointwiseSafe payload.radius payload.windowMid payload.positions

def GeneratedDualEvidence.dualCertificate {base : ℕ} [NeZero base]
    {payload : GeneratedWindowPayload base} {hEven : Even base}
    (evidence : GeneratedDualEvidence payload) :
    DualCertificate hEven payload.windowData :=
  payload.dualCertificate hEven
    evidence.toGeneratedStaticEvidence.balanced
    evidence.toGeneratedStaticEvidence.fixedPointExclusion
    evidence.pointwiseSafe

theorem GeneratedDualEvidence.midpoint_not_in_range {base : ℕ} [NeZero base]
    {payload : GeneratedWindowPayload base} {hEven : Even base}
    (evidence : GeneratedDualEvidence payload) :
    midpoint base ∉ Set.range payload.windowData.residue :=
  WindowCertificate.dual_midpoint_not_in_range (evidence.dualCertificate (hEven := hEven))

theorem GeneratedDualEvidence.zero_not_in_range {base : ℕ} [NeZero base]
    {payload : GeneratedWindowPayload base} {hEven : Even base}
    (evidence : GeneratedDualEvidence payload) :
    (0 : Fin base) ∉ Set.range payload.windowData.residue :=
  WindowCertificate.zero_not_in_range
    ((evidence.dualCertificate (hEven := hEven)).static)

theorem GeneratedDualEvidence.inviolability {base : ℕ} [NeZero base]
    {payload : GeneratedWindowPayload base} {hEven : Even base}
    (evidence : GeneratedDualEvidence payload) :
    InZone payload.radius payload.windowMid payload.positions → False :=
  WindowCertificate.dual_inviolability (evidence.dualCertificate (hEven := hEven))

end PrimeArithmetic.Symmetry.WindowCertificateErgonomics
