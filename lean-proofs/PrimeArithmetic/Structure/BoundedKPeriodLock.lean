import Mathlib
import PrimeArithmetic.Structure.BoundedKTemplate
import PrimeArithmetic.Structure.AffinePeriodLock

namespace PrimeArithmetic.Structure

/-!
Bounded-`k` specialization of the local affine period-lock criterion.

This module does not claim a class theorem. It simply rewrites the exact local
period-lock language in the maintained bounded-`k` lane vocabulary used by the
Rust atlas:

* the bounded-`k` gradient position is `kOuter + kInner + 2`
* direct lane comparisons inherit the period-lock criterion from
  `AffinePeriodLock`
* in the special case `k = (0, 0) → (kOuter, kInner)`, gradient lock reduces to
  `kOuter + kInner ≡ 0` modulo the multiplicative order of the base unit
* inside the locked regime, local relations reduce to `identity` vs
  `gradientOnly` by shift agreement
* outside the locked regime, they reduce to `shiftOnly` vs `shiftAndGradient`
  by shift agreement
-/

/-- The bounded-`k` gradient position used by the direct lane atlas. -/
def boundedKGradientPosition (cfg : BoundedKConfig) : ℕ :=
  cfg.kOuter + cfg.kInner + 2

@[simp] theorem boundedKGradientPosition_eq_middlePosition
    (cfg : BoundedKConfig) (base outer inner middleWidth : ℕ) :
    boundedKGradientPosition cfg =
      middlePosition (cfg.toSymmetricTemplateConfig base outer inner middleWidth) := by
  simp [boundedKGradientPosition]

theorem templateGradient_eq_iff_gradientPeriodLocked_boundedK
    (left right : BoundedKConfig) (base modulus outer inner middleWidth : ℕ)
    [NeZero modulus] (hcop : modulus.Coprime base) :
    (templateGradient (left.toSymmetricTemplateConfig base outer inner middleWidth) : ZMod modulus) =
        templateGradient (right.toSymmetricTemplateConfig base outer inner middleWidth) ↔
      gradientPeriodLocked base modulus
        (boundedKGradientPosition left) (boundedKGradientPosition right) hcop := by
  simpa [boundedKGradientPosition] using
    templateGradient_eq_iff_gradientPeriodLocked
      (left.toSymmetricTemplateConfig base outer inner middleWidth)
      (right.toSymmetricTemplateConfig base outer inner middleWidth)
      modulus rfl hcop

theorem gradientPeriodLocked_k00_to_iff
    (cfg : BoundedKConfig) (base modulus : ℕ) [NeZero modulus]
    (hcop : modulus.Coprime base) :
    gradientPeriodLocked base modulus
        (boundedKGradientPosition { kOuter := 0, kInner := 0 }) (boundedKGradientPosition cfg) hcop ↔
      cfg.kOuter + cfg.kInner ≡ 0 [MOD orderOf (baseUnit base modulus hcop)] := by
  constructor
  · intro h
    have h' :
        0 ≡ cfg.kOuter + cfg.kInner [MOD orderOf (baseUnit base modulus hcop)] := by
      simpa [boundedKGradientPosition, Nat.zero_add, Nat.add_assoc] using
        (Nat.ModEq.add_right_cancel' 2 h)
    simpa [Nat.ModEq] using h'.symm
  · intro h
    have h' : cfg.kOuter + cfg.kInner ≡ 0 [MOD orderOf (baseUnit base modulus hcop)] := by
      simpa [Nat.ModEq] using h
    have h'' :
        cfg.kOuter + cfg.kInner + 2 ≡ 0 + 2 [MOD orderOf (baseUnit base modulus hcop)] :=
      Nat.ModEq.add_right 2 h'
    simpa [boundedKGradientPosition, Nat.zero_add, Nat.add_assoc] using h''.symm

theorem zeroSeedClass_eq_iff_shift_eq_of_gradientPeriodLocked_boundedK
    (left right : BoundedKConfig) (base modulus outer inner middleWidth : ℕ)
    [NeZero modulus] (hcop : modulus.Coprime base)
    (hlock : gradientPeriodLocked base modulus
      (boundedKGradientPosition left) (boundedKGradientPosition right) hcop) :
    zeroSeedClass (left.toSymmetricTemplateConfig base outer inner middleWidth) modulus hcop =
        zeroSeedClass (right.toSymmetricTemplateConfig base outer inner middleWidth) modulus hcop ↔
      (templateShift (left.toSymmetricTemplateConfig base outer inner middleWidth) : ZMod modulus) =
        templateShift (right.toSymmetricTemplateConfig base outer inner middleWidth) := by
  simpa [boundedKGradientPosition] using
    zeroSeedClass_eq_iff_shift_eq_of_gradientPeriodLocked
      (left.toSymmetricTemplateConfig base outer inner middleWidth)
      (right.toSymmetricTemplateConfig base outer inner middleWidth)
      modulus hcop hcop rfl hlock

theorem localRelation_eq_identity_iff_shift_eq_of_gradientPeriodLocked_boundedK
    (left right : BoundedKConfig) (base modulus outer inner middleWidth : ℕ)
    [NeZero modulus] (hcop : modulus.Coprime base)
    (hlock : gradientPeriodLocked base modulus
      (boundedKGradientPosition left) (boundedKGradientPosition right) hcop) :
    localRelation
        (localAffineProfile (left.toSymmetricTemplateConfig base outer inner middleWidth) modulus hcop)
        (localAffineProfile (right.toSymmetricTemplateConfig base outer inner middleWidth) modulus hcop) = .identity ↔
      (templateShift (left.toSymmetricTemplateConfig base outer inner middleWidth) : ZMod modulus) =
        templateShift (right.toSymmetricTemplateConfig base outer inner middleWidth) := by
  simpa [boundedKGradientPosition] using
    localRelation_eq_identity_iff_shift_eq_of_gradientPeriodLocked
      (left.toSymmetricTemplateConfig base outer inner middleWidth)
      (right.toSymmetricTemplateConfig base outer inner middleWidth)
      modulus hcop hcop rfl hlock

theorem localRelation_eq_gradientOnly_iff_shift_ne_of_gradientPeriodLocked_boundedK
    (left right : BoundedKConfig) (base modulus outer inner middleWidth : ℕ)
    [NeZero modulus] (hcop : modulus.Coprime base)
    (hlock : gradientPeriodLocked base modulus
      (boundedKGradientPosition left) (boundedKGradientPosition right) hcop) :
    localRelation
        (localAffineProfile (left.toSymmetricTemplateConfig base outer inner middleWidth) modulus hcop)
        (localAffineProfile (right.toSymmetricTemplateConfig base outer inner middleWidth) modulus hcop) = .gradientOnly ↔
      (templateShift (left.toSymmetricTemplateConfig base outer inner middleWidth) : ZMod modulus) ≠
        templateShift (right.toSymmetricTemplateConfig base outer inner middleWidth) := by
  simpa [boundedKGradientPosition] using
    localRelation_eq_gradientOnly_iff_shift_ne_of_gradientPeriodLocked
      (left.toSymmetricTemplateConfig base outer inner middleWidth)
      (right.toSymmetricTemplateConfig base outer inner middleWidth)
      modulus hcop hcop rfl hlock

theorem localRelation_eq_shiftOnly_iff_shift_eq_of_not_gradientPeriodLocked_boundedK
    (left right : BoundedKConfig) (base modulus outer inner middleWidth : ℕ)
    [NeZero modulus] (hcop : modulus.Coprime base)
    (hunlock : ¬ gradientPeriodLocked base modulus
      (boundedKGradientPosition left) (boundedKGradientPosition right) hcop) :
    localRelation
        (localAffineProfile (left.toSymmetricTemplateConfig base outer inner middleWidth) modulus hcop)
        (localAffineProfile (right.toSymmetricTemplateConfig base outer inner middleWidth) modulus hcop) = .shiftOnly ↔
      (templateShift (left.toSymmetricTemplateConfig base outer inner middleWidth) : ZMod modulus) =
        templateShift (right.toSymmetricTemplateConfig base outer inner middleWidth) := by
  simpa [boundedKGradientPosition] using
    localRelation_eq_shiftOnly_iff_shift_eq_of_not_gradientPeriodLocked
      (left.toSymmetricTemplateConfig base outer inner middleWidth)
      (right.toSymmetricTemplateConfig base outer inner middleWidth)
      modulus hcop hcop rfl hunlock

theorem localRelation_eq_shiftAndGradient_iff_shift_ne_of_not_gradientPeriodLocked_boundedK
    (left right : BoundedKConfig) (base modulus outer inner middleWidth : ℕ)
    [NeZero modulus] (hcop : modulus.Coprime base)
    (hunlock : ¬ gradientPeriodLocked base modulus
      (boundedKGradientPosition left) (boundedKGradientPosition right) hcop) :
    localRelation
        (localAffineProfile (left.toSymmetricTemplateConfig base outer inner middleWidth) modulus hcop)
        (localAffineProfile (right.toSymmetricTemplateConfig base outer inner middleWidth) modulus hcop) = .shiftAndGradient ↔
      (templateShift (left.toSymmetricTemplateConfig base outer inner middleWidth) : ZMod modulus) ≠
        templateShift (right.toSymmetricTemplateConfig base outer inner middleWidth) := by
  simpa [boundedKGradientPosition] using
    localRelation_eq_shiftAndGradient_iff_shift_ne_of_not_gradientPeriodLocked
      (left.toSymmetricTemplateConfig base outer inner middleWidth)
      (right.toSymmetricTemplateConfig base outer inner middleWidth)
      modulus hcop hcop rfl hunlock

end PrimeArithmetic.Structure
