import Mathlib
import PrimeArithmetic.Structure.AffineLaneComparison

namespace PrimeArithmetic.Structure

/-!
Exact local period-lock language for affine gradient agreement.

For one modulus coprime to the base, the local affine gradient is the cast of a
power of the base. Gradient equality between two lanes is therefore controlled
by the multiplicative order of the base unit in `ZMod modulus`.

This packages the exact bridge used by the affine period-lock atlas:

* gradient equality iff the middle positions are congruent modulo the base-unit
  order
* once that lock holds, the local relation label is determined purely by shift
  agreement
* once that lock fails, the local relation label is determined purely by shift
  agreement in the complementary `shift_only / shift_and_gradient` regime
-/

/-- The base viewed as a unit of `ZMod modulus` on the coprime-modulus surface. -/
noncomputable def baseUnit (base modulus : ℕ) (hcop : modulus.Coprime base) : (ZMod modulus)ˣ :=
  ZMod.unitOfCoprime base (Nat.coprime_comm.mp hcop)

/-- Period lock for local affine gradients: the two positions agree modulo the
multiplicative order of the base unit. -/
def gradientPeriodLocked
    (base modulus leftPos rightPos : ℕ) (hcop : modulus.Coprime base) : Prop :=
  leftPos ≡ rightPos [MOD orderOf (baseUnit base modulus hcop)]

theorem basePow_eq_iff_gradientPeriodLocked
    (base modulus leftPos rightPos : ℕ) [NeZero modulus]
    (hcop : modulus.Coprime base) :
    ((base ^ leftPos : ℕ) : ZMod modulus) = base ^ rightPos ↔
      gradientPeriodLocked base modulus leftPos rightPos hcop := by
  let u : (ZMod modulus)ˣ := baseUnit base modulus hcop
  have hPowUnits :
      u ^ leftPos = u ^ rightPos ↔ leftPos ≡ rightPos [MOD orderOf u] := by
    simpa [u] using
      (pow_eq_pow_iff_modEq (x := u) (n := leftPos) (m := rightPos))
  have hCast :
      ((base ^ leftPos : ℕ) : ZMod modulus) = base ^ rightPos ↔ u ^ leftPos = u ^ rightPos := by
    constructor
    · intro h
      apply Units.ext
      simpa [u, baseUnit, Units.val_pow_eq_pow_val] using h
    · intro h
      simpa [u, baseUnit, Units.val_pow_eq_pow_val] using
        congrArg (fun z : (ZMod modulus)ˣ => (z : ZMod modulus)) h
  exact hCast.trans <| by simpa [gradientPeriodLocked, u]

theorem templateGradient_eq_iff_gradientPeriodLocked
    (conf₁ conf₂ : SymmetricTemplateConfig) (modulus : ℕ) [NeZero modulus]
    (hbase : conf₁.base = conf₂.base)
    (hcop : modulus.Coprime conf₁.base) :
    (templateGradient conf₁ : ZMod modulus) = templateGradient conf₂ ↔
      gradientPeriodLocked conf₁.base modulus (middlePosition conf₁) (middlePosition conf₂) hcop := by
  simpa [templateGradient, middlePosition, hbase] using
    basePow_eq_iff_gradientPeriodLocked
      conf₁.base modulus (middlePosition conf₁) (middlePosition conf₂) hcop

theorem zeroSeedClass_eq_iff_shift_eq_of_gradientPeriodLocked
    (conf₁ conf₂ : SymmetricTemplateConfig) (modulus : ℕ) [NeZero modulus]
    (hcop₁ : modulus.Coprime conf₁.base) (hcop₂ : modulus.Coprime conf₂.base)
    (hbase : conf₁.base = conf₂.base)
    (hlock : gradientPeriodLocked conf₁.base modulus
      (middlePosition conf₁) (middlePosition conf₂) hcop₁) :
    zeroSeedClass conf₁ modulus hcop₁ = zeroSeedClass conf₂ modulus hcop₂ ↔
      (templateShift conf₁ : ZMod modulus) = templateShift conf₂ := by
  have hGrad : (templateGradient conf₁ : ZMod modulus) = templateGradient conf₂ :=
    (templateGradient_eq_iff_gradientPeriodLocked conf₁ conf₂ modulus hbase hcop₁).2 hlock
  exact zeroSeedClass_eq_iff_shift_eq_of_gradient_eq
    conf₁ conf₂ modulus hcop₁ hcop₂ hGrad

theorem localRelation_eq_identity_iff_shift_eq_of_gradientPeriodLocked
    (conf₁ conf₂ : SymmetricTemplateConfig) (modulus : ℕ) [NeZero modulus]
    (hcop₁ : modulus.Coprime conf₁.base) (hcop₂ : modulus.Coprime conf₂.base)
    (hbase : conf₁.base = conf₂.base)
    (hlock : gradientPeriodLocked conf₁.base modulus
      (middlePosition conf₁) (middlePosition conf₂) hcop₁) :
    localRelation
        (localAffineProfile conf₁ modulus hcop₁)
        (localAffineProfile conf₂ modulus hcop₂) = .identity ↔
      (templateShift conf₁ : ZMod modulus) = templateShift conf₂ := by
  have hGrad : (templateGradient conf₁ : ZMod modulus) = templateGradient conf₂ :=
    (templateGradient_eq_iff_gradientPeriodLocked conf₁ conf₂ modulus hbase hcop₁).2 hlock
  have hProfileGrad :
      (localAffineProfile conf₁ modulus hcop₁).gradient =
        (localAffineProfile conf₂ modulus hcop₂).gradient := by
    simpa [localAffineProfile] using hGrad
  constructor
  · intro hRel
    exact (localRelation_eq_identity_iff
      (localAffineProfile conf₁ modulus hcop₁)
      (localAffineProfile conf₂ modulus hcop₂)).1 hRel |>.1
  · intro hShift
    exact localRelation_identity_of_shift_eq_and_gradient_eq
      (left := localAffineProfile conf₁ modulus hcop₁)
      (right := localAffineProfile conf₂ modulus hcop₂)
      (by simpa [localAffineProfile] using hShift)
      hProfileGrad

theorem localRelation_eq_gradientOnly_iff_shift_ne_of_gradientPeriodLocked
    (conf₁ conf₂ : SymmetricTemplateConfig) (modulus : ℕ) [NeZero modulus]
    (hcop₁ : modulus.Coprime conf₁.base) (hcop₂ : modulus.Coprime conf₂.base)
    (hbase : conf₁.base = conf₂.base)
    (hlock : gradientPeriodLocked conf₁.base modulus
      (middlePosition conf₁) (middlePosition conf₂) hcop₁) :
    localRelation
        (localAffineProfile conf₁ modulus hcop₁)
        (localAffineProfile conf₂ modulus hcop₂) = .gradientOnly ↔
      (templateShift conf₁ : ZMod modulus) ≠ templateShift conf₂ := by
  have hGrad : (templateGradient conf₁ : ZMod modulus) = templateGradient conf₂ :=
    (templateGradient_eq_iff_gradientPeriodLocked conf₁ conf₂ modulus hbase hcop₁).2 hlock
  have hProfileGrad :
      (localAffineProfile conf₁ modulus hcop₁).gradient =
        (localAffineProfile conf₂ modulus hcop₂).gradient := by
    simpa [localAffineProfile] using hGrad
  simpa [hProfileGrad] using
    (localRelation_eq_gradientOnly_iff
      (localAffineProfile conf₁ modulus hcop₁)
      (localAffineProfile conf₂ modulus hcop₂))

theorem localRelation_eq_shiftOnly_iff_shift_eq_of_not_gradientPeriodLocked
    (conf₁ conf₂ : SymmetricTemplateConfig) (modulus : ℕ) [NeZero modulus]
    (hcop₁ : modulus.Coprime conf₁.base) (hcop₂ : modulus.Coprime conf₂.base)
    (hbase : conf₁.base = conf₂.base)
    (hunlock : ¬ gradientPeriodLocked conf₁.base modulus
      (middlePosition conf₁) (middlePosition conf₂) hcop₁) :
    localRelation
        (localAffineProfile conf₁ modulus hcop₁)
        (localAffineProfile conf₂ modulus hcop₂) = .shiftOnly ↔
      (templateShift conf₁ : ZMod modulus) = templateShift conf₂ := by
  have hGradNe : (templateGradient conf₁ : ZMod modulus) ≠ templateGradient conf₂ := by
    intro hGrad
    exact hunlock <|
      (templateGradient_eq_iff_gradientPeriodLocked conf₁ conf₂ modulus hbase hcop₁).1 hGrad
  have hProfileGradNe :
      (localAffineProfile conf₁ modulus hcop₁).gradient ≠
        (localAffineProfile conf₂ modulus hcop₂).gradient := by
    simpa [localAffineProfile] using hGradNe
  simpa [hProfileGradNe] using
    (localRelation_eq_shiftOnly_iff
      (localAffineProfile conf₁ modulus hcop₁)
      (localAffineProfile conf₂ modulus hcop₂))

theorem localRelation_eq_shiftAndGradient_iff_shift_ne_of_not_gradientPeriodLocked
    (conf₁ conf₂ : SymmetricTemplateConfig) (modulus : ℕ) [NeZero modulus]
    (hcop₁ : modulus.Coprime conf₁.base) (hcop₂ : modulus.Coprime conf₂.base)
    (hbase : conf₁.base = conf₂.base)
    (hunlock : ¬ gradientPeriodLocked conf₁.base modulus
      (middlePosition conf₁) (middlePosition conf₂) hcop₁) :
    localRelation
        (localAffineProfile conf₁ modulus hcop₁)
        (localAffineProfile conf₂ modulus hcop₂) = .shiftAndGradient ↔
      (templateShift conf₁ : ZMod modulus) ≠ templateShift conf₂ := by
  have hGradNe : (templateGradient conf₁ : ZMod modulus) ≠ templateGradient conf₂ := by
    intro hGrad
    exact hunlock <|
      (templateGradient_eq_iff_gradientPeriodLocked conf₁ conf₂ modulus hbase hcop₁).1 hGrad
  have hProfileGradNe :
      (localAffineProfile conf₁ modulus hcop₁).gradient ≠
        (localAffineProfile conf₂ modulus hcop₂).gradient := by
    simpa [localAffineProfile] using hGradNe
  simpa [hProfileGradNe] using
    (localRelation_eq_shiftAndGradient_iff
      (localAffineProfile conf₁ modulus hcop₁)
      (localAffineProfile conf₂ modulus hcop₂))

end PrimeArithmetic.Structure
