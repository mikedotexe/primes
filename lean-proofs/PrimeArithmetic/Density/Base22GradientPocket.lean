import Mathlib
import PrimeArithmetic.Density.Base22Residues
import PrimeArithmetic.Structure.AffineLaneComparison
import PrimeArithmetic.Structure.BoundedKTemplate

namespace PrimeArithmetic.Density.Base22GradientPocket

open PrimeArithmetic.Structure

/-!
The focused base-22 residual pocket from the affine hinge atlas has a clean
exact local explanation at modulus `5`.

For middle width `M = 2`, compare the compact lane `k = (0, 0)` against the
offset lane `k = (2, 2)`. Modulo `5`:

- the local gradients agree,
- the local shifts differ by `-inner`,
- so the local affine relation is `identity` exactly when `5 ∣ inner`,
  and `gradientOnly` otherwise.

This is the exact Lean version of the base-22 / mod-5 side-pocket isolated by
the maintained affine exploration reports.
-/

def k00 : BoundedKConfig := { kOuter := 0, kInner := 0 }

def k22 : BoundedKConfig := { kOuter := 2, kInner := 2 }

def k00Config (outer inner : ℕ) : SymmetricTemplateConfig :=
  k00.toSymmetricTemplateConfig 22 outer inner 2

def k22Config (outer inner : ℕ) : SymmetricTemplateConfig :=
  k22.toSymmetricTemplateConfig 22 outer inner 2

theorem modFiveCoprime_k00 (outer inner : ℕ) :
    (5).Coprime (k00Config outer inner).base := by
  simpa [k00Config, k00, BoundedKConfig.toSymmetricTemplateConfig] using
    (show Nat.Coprime 5 22 by decide)

theorem modFiveCoprime_k22 (outer inner : ℕ) :
    (5).Coprime (k22Config outer inner).base := by
  simpa [k22Config, k22, BoundedKConfig.toSymmetricTemplateConfig] using
    (show Nat.Coprime 5 22 by decide)

noncomputable def k00Mod5Profile (outer inner : ℕ) : AffineLocalProfile 5 :=
  localAffineProfile (k00Config outer inner) 5 (modFiveCoprime_k00 outer inner)

noncomputable def k22Mod5Profile (outer inner : ℕ) : AffineLocalProfile 5 :=
  localAffineProfile (k22Config outer inner) 5 (modFiveCoprime_k22 outer inner)

theorem shift_k00_mod5 (outer inner : ℕ) :
    (templateShift (k00Config outer inner) : ZMod 5) =
      (3 : ZMod 5) * outer + (3 : ZMod 5) * inner := by
  rw [templateShift, templateValue]
  push_cast
  have hLO : leftOuterPosition (k00Config outer inner) = 5 := by
    simp [k00Config, k00, leftOuterPosition, BoundedKConfig.toSymmetricTemplateConfig]
  have hLI : leftInnerPosition (k00Config outer inner) = 4 := by
    simp [k00Config, k00, leftInnerPosition, BoundedKConfig.toSymmetricTemplateConfig]
  have hRI : rightInnerPosition (k00Config outer inner) = 1 := by
    simp [k00Config, k00, rightInnerPosition, BoundedKConfig.toSymmetricTemplateConfig]
  have hMid : middlePosition (k00Config outer inner) = 2 := by
    simp [k00Config, k00, middlePosition, BoundedKConfig.toSymmetricTemplateConfig]
  rw [hLO, hLI, hRI, hMid]
  have hBase : (((k00Config outer inner).base : ℕ) : ZMod 5) = 22 := by
    rfl
  have hOuter : (((k00Config outer inner).outer : ℕ) : ZMod 5) = outer := by
    rfl
  have hInner : (((k00Config outer inner).inner : ℕ) : ZMod 5) = inner := by
    rfl
  rw [hBase, hOuter, hInner]
  simp
  have h5 : ((22 : ZMod 5) ^ 5) = 2 := by native_decide
  have h4 : ((22 : ZMod 5) ^ 4) = 1 := by native_decide
  have h22 : (22 : ZMod 5) = 2 := by native_decide
  rw [h5, h4, h22]
  ring

theorem shift_k22_mod5 (outer inner : ℕ) :
    (templateShift (k22Config outer inner) : ZMod 5) =
      (3 : ZMod 5) * outer + (2 : ZMod 5) * inner := by
  rw [templateShift, templateValue]
  push_cast
  have hLO : leftOuterPosition (k22Config outer inner) = 13 := by
    simp [k22Config, k22, leftOuterPosition, BoundedKConfig.toSymmetricTemplateConfig]
  have hLI : leftInnerPosition (k22Config outer inner) = 10 := by
    simp [k22Config, k22, leftInnerPosition, BoundedKConfig.toSymmetricTemplateConfig]
  have hRI : rightInnerPosition (k22Config outer inner) = 3 := by
    simp [k22Config, k22, rightInnerPosition, BoundedKConfig.toSymmetricTemplateConfig]
  have hMid : middlePosition (k22Config outer inner) = 6 := by
    simp [k22Config, k22, middlePosition, BoundedKConfig.toSymmetricTemplateConfig]
  rw [hLO, hLI, hRI, hMid]
  have hBase : (((k22Config outer inner).base : ℕ) : ZMod 5) = 22 := by
    rfl
  have hOuter : (((k22Config outer inner).outer : ℕ) : ZMod 5) = outer := by
    rfl
  have hInner : (((k22Config outer inner).inner : ℕ) : ZMod 5) = inner := by
    rfl
  rw [hBase, hOuter, hInner]
  simp
  have h13 : ((22 : ZMod 5) ^ 13) = 2 := by native_decide
  have h10 : ((22 : ZMod 5) ^ 10) = 4 := by native_decide
  have h3 : ((22 : ZMod 5) ^ 3) = 3 := by native_decide
  rw [h13, h10, h3]
  ring_nf
  have h7 : (7 : ZMod 5) = 2 := by native_decide
  simp [h7]

theorem gradient_eq_mod5 (outer inner : ℕ) :
    (templateGradient (k00Config outer inner) : ZMod 5) =
      templateGradient (k22Config outer inner) := by
  rw [templateGradient, templateGradient]
  have hM00 : middlePosition (k00Config outer inner) = 2 := by
    simp [k00Config, k00, middlePosition, BoundedKConfig.toSymmetricTemplateConfig]
  have hM22 : middlePosition (k22Config outer inner) = 6 := by
    simp [k22Config, k22, middlePosition, BoundedKConfig.toSymmetricTemplateConfig]
  rw [hM00, hM22]
  push_cast
  have hBase00 : (((k00Config outer inner).base : ℕ) : ZMod 5) = 22 := by
    rfl
  have hBase22 : (((k22Config outer inner).base : ℕ) : ZMod 5) = 22 := by
    rfl
  rw [hBase00, hBase22]
  have h2 : ((22 : ZMod 5) ^ 2) = 4 := by native_decide
  have h6 : ((22 : ZMod 5) ^ 6) = 4 := by native_decide
  rw [h2, h6]

theorem shift_diff_mod5 (outer inner : ℕ) :
    (templateShift (k22Config outer inner) : ZMod 5) -
        templateShift (k00Config outer inner) =
      -(inner : ZMod 5) := by
  rw [shift_k22_mod5, shift_k00_mod5]
  ring

theorem shift_eq_iff_five_dvd_inner (outer inner : ℕ) :
    (templateShift (k00Config outer inner) : ZMod 5) =
        templateShift (k22Config outer inner) ↔
      5 ∣ inner := by
  constructor
  · intro h
    have hdiff0 :
        ((templateShift (k22Config outer inner) : ZMod 5) -
            templateShift (k00Config outer inner)) = 0 := by
      simpa [sub_eq_zero] using h.symm
    have hneg : -(inner : ZMod 5) = 0 := by
      simpa [shift_diff_mod5 outer inner] using hdiff0
    have hzero : (inner : ZMod 5) = 0 := by
      simpa using congrArg Neg.neg hneg
    exact (ZMod.natCast_eq_zero_iff inner 5).1 hzero
  · intro hdiv
    have hzero : (inner : ZMod 5) = 0 := (ZMod.natCast_eq_zero_iff inner 5).2 hdiv
    have hdiff0 :
        ((templateShift (k22Config outer inner) : ZMod 5) -
            templateShift (k00Config outer inner)) = 0 := by
      rw [shift_diff_mod5 outer inner, hzero]
      simp
    exact (sub_eq_zero.mp hdiff0).symm

theorem shift_eq_iff_inner_mod_five_eq_zero (outer inner : ℕ) :
    (templateShift (k00Config outer inner) : ZMod 5) =
        templateShift (k22Config outer inner) ↔
      inner % 5 = 0 := by
  rw [shift_eq_iff_five_dvd_inner]
  simpa using (Nat.dvd_iff_mod_eq_zero : 5 ∣ inner ↔ inner % 5 = 0)

theorem profile_gradient_eq_mod5 (outer inner : ℕ) :
    (k00Mod5Profile outer inner).gradient = (k22Mod5Profile outer inner).gradient := by
  simpa [k00Mod5Profile, k22Mod5Profile, localAffineProfile] using
    gradient_eq_mod5 outer inner

theorem profile_shift_eq_iff_five_dvd_inner (outer inner : ℕ) :
    (k00Mod5Profile outer inner).shift = (k22Mod5Profile outer inner).shift ↔
      5 ∣ inner := by
  simpa [k00Mod5Profile, k22Mod5Profile, localAffineProfile] using
    shift_eq_iff_five_dvd_inner outer inner

theorem zeroSeedClass_eq_iff_five_dvd_inner (outer inner : ℕ) :
    zeroSeedClass (k00Config outer inner) 5 (modFiveCoprime_k00 outer inner) =
        zeroSeedClass (k22Config outer inner) 5 (modFiveCoprime_k22 outer inner) ↔
      5 ∣ inner := by
  exact
    (zeroSeedClass_eq_iff_shift_eq_of_gradient_eq
      (k00Config outer inner) (k22Config outer inner) 5
      (modFiveCoprime_k00 outer inner) (modFiveCoprime_k22 outer inner)
      (gradient_eq_mod5 outer inner)).trans
      (shift_eq_iff_five_dvd_inner outer inner)

theorem localRelation_eq_identity_iff_five_dvd_inner (outer inner : ℕ) :
    localRelation (k00Mod5Profile outer inner) (k22Mod5Profile outer inner) = .identity ↔
      5 ∣ inner := by
  constructor
  · intro h
    have hrel :=
      (localRelation_eq_identity_iff (k00Mod5Profile outer inner) (k22Mod5Profile outer inner)).1 h
    exact (profile_shift_eq_iff_five_dvd_inner outer inner).1 hrel.1
  · intro hdiv
    apply
      (localRelation_eq_identity_iff
        (k00Mod5Profile outer inner) (k22Mod5Profile outer inner)).2
    exact ⟨(profile_shift_eq_iff_five_dvd_inner outer inner).2 hdiv,
      profile_gradient_eq_mod5 outer inner⟩

theorem localRelation_eq_gradientOnly_iff_not_five_dvd_inner (outer inner : ℕ) :
    localRelation (k00Mod5Profile outer inner) (k22Mod5Profile outer inner) = .gradientOnly ↔
      ¬ 5 ∣ inner := by
  constructor
  · intro h
    have hrel :=
      (localRelation_eq_gradientOnly_iff
        (k00Mod5Profile outer inner) (k22Mod5Profile outer inner)).1 h
    exact mt (profile_shift_eq_iff_five_dvd_inner outer inner).2 hrel.1
  · intro hnot
    apply
      (localRelation_eq_gradientOnly_iff
        (k00Mod5Profile outer inner) (k22Mod5Profile outer inner)).2
    exact ⟨mt (profile_shift_eq_iff_five_dvd_inner outer inner).1 hnot,
      profile_gradient_eq_mod5 outer inner⟩

theorem localRelation_eq_identity_iff_inner_mod_five_eq_zero (outer inner : ℕ) :
    localRelation (k00Mod5Profile outer inner) (k22Mod5Profile outer inner) = .identity ↔
      inner % 5 = 0 := by
  rw [localRelation_eq_identity_iff_five_dvd_inner]
  simpa using (Nat.dvd_iff_mod_eq_zero : 5 ∣ inner ↔ inner % 5 = 0)

theorem localRelation_eq_gradientOnly_iff_inner_mod_five_ne_zero (outer inner : ℕ) :
    localRelation (k00Mod5Profile outer inner) (k22Mod5Profile outer inner) = .gradientOnly ↔
      inner % 5 ≠ 0 := by
  rw [localRelation_eq_gradientOnly_iff_not_five_dvd_inner]
  simp [Nat.dvd_iff_mod_eq_zero]

end PrimeArithmetic.Density.Base22GradientPocket
