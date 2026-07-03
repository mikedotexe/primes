import Mathlib
import PrimeArithmetic.Structure.AffineSeedClasses

namespace PrimeArithmetic.Structure

/-!
Exact local affine lane-comparison language.

For one modulus coprime to the base, a fixed symmetric template determines:

- a local affine shift
- a local affine gradient
- a unique zero-seed class

This module packages that vocabulary for lane comparisons and records the basic
exact consequences needed by the affine hinge atlas.
-/

structure AffineLocalProfile (modulus : ℕ) where
  shift : ZMod modulus
  gradient : ZMod modulus
  zeroSeedClass : ZMod modulus

noncomputable def localAffineProfile
    (conf : SymmetricTemplateConfig) (modulus : ℕ)
    (hcop : modulus.Coprime conf.base) :
    AffineLocalProfile modulus where
  shift := templateShift conf
  gradient := templateGradient conf
  zeroSeedClass := zeroSeedClass conf modulus hcop

def localAffineMap {modulus : ℕ} (profile : AffineLocalProfile modulus) :
    ZMod modulus → ZMod modulus :=
  fun seed => profile.shift + seed * profile.gradient

theorem localAffineMap_eq_iff
    {modulus : ℕ} (left right : AffineLocalProfile modulus) :
    localAffineMap left = localAffineMap right ↔
      left.shift = right.shift ∧ left.gradient = right.gradient := by
  constructor
  · intro h
    have h0 := congrFun h 0
    have h1 := congrFun h 1
    have hShift : left.shift = right.shift := by
      simpa [localAffineMap] using h0
    have hGrad : left.gradient = right.gradient := by
      simpa [localAffineMap, hShift] using h1
    exact ⟨hShift, hGrad⟩
  · rintro ⟨hShift, hGrad⟩
    funext seed
    simp [localAffineMap, hShift, hGrad]

theorem zeroSeedClass_eq_of_shift_eq_and_gradient_eq
    (conf₁ conf₂ : SymmetricTemplateConfig) (modulus : ℕ)
    [NeZero modulus]
    (hcop₁ : modulus.Coprime conf₁.base) (hcop₂ : modulus.Coprime conf₂.base)
    (hShift : (templateShift conf₁ : ZMod modulus) = templateShift conf₂)
    (hGrad : (templateGradient conf₁ : ZMod modulus) = templateGradient conf₂) :
    zeroSeedClass conf₁ modulus hcop₁ = zeroSeedClass conf₂ modulus hcop₂ := by
  let seed := (zeroSeedClass conf₁ modulus hcop₁).val
  have hz₁ : (templateValue conf₁ seed : ZMod modulus) = 0 := by
    have hseed :
        (seed : ZMod modulus) = zeroSeedClass conf₁ modulus hcop₁ := by
      simp [seed]
    exact
      (templateValue_eq_zero_iff_seed_eq_zeroSeedClass conf₁ modulus seed hcop₁).2 hseed
  have hz₁' :
      (templateShift conf₁ : ZMod modulus) + (seed : ZMod modulus) * templateGradient conf₁ = 0 := by
    simpa [templateValue_eq_shift_add_gradient] using hz₁
  have hz₂ : (templateValue conf₂ seed : ZMod modulus) = 0 := by
    calc
      (templateValue conf₂ seed : ZMod modulus)
          = (templateShift conf₂ : ZMod modulus) + (seed : ZMod modulus) * templateGradient conf₂ := by
              simp [templateValue_eq_shift_add_gradient]
      _ = (templateShift conf₁ : ZMod modulus) + (seed : ZMod modulus) * templateGradient conf₁ := by
            simp [hShift, hGrad]
      _ = 0 := hz₁'
  have hseed₂ :
      (seed : ZMod modulus) = zeroSeedClass conf₂ modulus hcop₂ :=
    (templateValue_eq_zero_iff_seed_eq_zeroSeedClass conf₂ modulus seed hcop₂).1 hz₂
  calc
    zeroSeedClass conf₁ modulus hcop₁ = (seed : ZMod modulus) := by
      simp [seed]
    _ = zeroSeedClass conf₂ modulus hcop₂ := hseed₂

theorem zeroSeedClass_eq_iff_shift_eq_of_gradient_eq
    (conf₁ conf₂ : SymmetricTemplateConfig) (modulus : ℕ)
    [NeZero modulus]
    (hcop₁ : modulus.Coprime conf₁.base) (hcop₂ : modulus.Coprime conf₂.base)
    (hGrad : (templateGradient conf₁ : ZMod modulus) = templateGradient conf₂) :
    zeroSeedClass conf₁ modulus hcop₁ = zeroSeedClass conf₂ modulus hcop₂ ↔
      (templateShift conf₁ : ZMod modulus) = templateShift conf₂ := by
  constructor
  · intro hZero
    let seed := (zeroSeedClass conf₁ modulus hcop₁).val
    have hseed₁ :
        (seed : ZMod modulus) = zeroSeedClass conf₁ modulus hcop₁ := by
      simp [seed]
    have hseed₂ :
        (seed : ZMod modulus) = zeroSeedClass conf₂ modulus hcop₂ := by
      calc
        (seed : ZMod modulus) = zeroSeedClass conf₁ modulus hcop₁ := hseed₁
        _ = zeroSeedClass conf₂ modulus hcop₂ := hZero
    have hz₁ : (templateValue conf₁ seed : ZMod modulus) = 0 :=
      (templateValue_eq_zero_iff_seed_eq_zeroSeedClass conf₁ modulus seed hcop₁).2 hseed₁
    have hz₂ : (templateValue conf₂ seed : ZMod modulus) = 0 :=
      (templateValue_eq_zero_iff_seed_eq_zeroSeedClass conf₂ modulus seed hcop₂).2 hseed₂
    have hz₁' :
        (templateShift conf₁ : ZMod modulus) + (seed : ZMod modulus) * templateGradient conf₁ = 0 := by
      simpa [templateValue_eq_shift_add_gradient] using hz₁
    have hz₂' :
        (templateShift conf₂ : ZMod modulus) + (seed : ZMod modulus) * templateGradient conf₂ = 0 := by
      simpa [templateValue_eq_shift_add_gradient] using hz₂
    have hEq :
        (templateShift conf₁ : ZMod modulus) + (seed : ZMod modulus) * templateGradient conf₁ =
          (templateShift conf₂ : ZMod modulus) + (seed : ZMod modulus) * templateGradient conf₂ :=
      hz₁'.trans hz₂'.symm
    have hEq' :
        (templateShift conf₁ : ZMod modulus) + (seed : ZMod modulus) * templateGradient conf₂ =
          (templateShift conf₂ : ZMod modulus) + (seed : ZMod modulus) * templateGradient conf₂ := by
      simpa [hGrad] using hEq
    exact add_right_cancel hEq'
  · intro hShift
    exact zeroSeedClass_eq_of_shift_eq_and_gradient_eq conf₁ conf₂ modulus hcop₁ hcop₂ hShift hGrad

inductive AffineLocalRelation
  | identity
  | shiftOnly
  | gradientOnly
  | shiftAndGradient
  deriving DecidableEq, Repr

def localRelation {modulus : ℕ}
    (left right : AffineLocalProfile modulus) : AffineLocalRelation :=
  if _ : left.shift = right.shift then
    if _ : left.gradient = right.gradient then
      .identity
    else
      .shiftOnly
  else if _ : left.gradient = right.gradient then
    .gradientOnly
  else
    .shiftAndGradient

theorem localRelation_eq_identity_iff
    {modulus : ℕ} (left right : AffineLocalProfile modulus) :
    localRelation left right = .identity ↔
      left.shift = right.shift ∧ left.gradient = right.gradient := by
  unfold localRelation
  by_cases hShift : left.shift = right.shift <;> by_cases hGrad : left.gradient = right.gradient <;>
    simp [hShift, hGrad]

theorem localRelation_eq_shiftOnly_iff
    {modulus : ℕ} (left right : AffineLocalProfile modulus) :
    localRelation left right = .shiftOnly ↔
      left.shift = right.shift ∧ left.gradient ≠ right.gradient := by
  unfold localRelation
  by_cases hShift : left.shift = right.shift <;> by_cases hGrad : left.gradient = right.gradient <;>
    simp [hShift, hGrad]

theorem localRelation_eq_gradientOnly_iff
    {modulus : ℕ} (left right : AffineLocalProfile modulus) :
    localRelation left right = .gradientOnly ↔
      left.shift ≠ right.shift ∧ left.gradient = right.gradient := by
  unfold localRelation
  by_cases hShift : left.shift = right.shift <;> by_cases hGrad : left.gradient = right.gradient <;>
    simp [hShift, hGrad]

theorem localRelation_eq_shiftAndGradient_iff
    {modulus : ℕ} (left right : AffineLocalProfile modulus) :
    localRelation left right = .shiftAndGradient ↔
      left.shift ≠ right.shift ∧ left.gradient ≠ right.gradient := by
  unfold localRelation
  by_cases hShift : left.shift = right.shift <;> by_cases hGrad : left.gradient = right.gradient <;>
    simp [hShift, hGrad]

theorem localRelation_identity_of_shift_eq_and_gradient_eq
    {modulus : ℕ} {left right : AffineLocalProfile modulus}
    (hShift : left.shift = right.shift) (hGrad : left.gradient = right.gradient) :
    localRelation left right = .identity := by
  exact (localRelation_eq_identity_iff left right).2 ⟨hShift, hGrad⟩

theorem localRelation_base10_37_self_mod11 :
    localRelation
        (localAffineProfile base10_37 11 (by decide))
        (localAffineProfile base10_37 11 (by decide)) = .identity := by
  apply localRelation_identity_of_shift_eq_and_gradient_eq <;> rfl

theorem zeroSeedClass_base10_37_self_mod11 :
    zeroSeedClass base10_37 11 (by decide) = zeroSeedClass base10_37 11 (by decide) := by
  rfl

end PrimeArithmetic.Structure
