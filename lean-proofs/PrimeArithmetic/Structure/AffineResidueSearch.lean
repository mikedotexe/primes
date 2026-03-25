import Mathlib
import PrimeArithmetic.Density.UnitResidues
import PrimeArithmetic.Structure.AffineTemplate

namespace PrimeArithmetic.Structure

open PrimeArithmetic.Density

/-!
Modular search consequences of the affine template theorem.

There are two complementary regimes.

1. For any modulus dividing the base, the template residue is independent of the
   seed and is determined by the outer digit.
2. For any modulus coprime to the base, the template residue varies affinely in
   the seed with invertible slope, so the seed-to-residue map is a permutation
   of `ZMod modulus`.

Together these facts turn the exact affine form into a reusable residue-search
surface.
-/

theorem cast_templateValue_eq_outer_of_dvd_base
    (conf : SymmetricTemplateConfig) (seed modulus : ℕ) (hmod : modulus ∣ conf.base) :
    ((templateValue conf seed : ℕ) : ZMod modulus) = conf.outer := by
  have hbase : ((conf.base : ℕ) : ZMod modulus) = 0 := by
    rw [ZMod.natCast_eq_zero_iff]
    exact hmod
  unfold templateValue rightInnerPosition middlePosition leftInnerPosition leftOuterPosition
  simp [pow_succ, pow_add, hbase, Nat.add_assoc, Nat.add_left_comm, Nat.add_comm]

theorem templateValue_modEq_outer_of_dvd_base
    (conf : SymmetricTemplateConfig) (seed modulus : ℕ) (hmod : modulus ∣ conf.base) :
    templateValue conf seed ≡ conf.outer [MOD modulus] := by
  rw [← ZMod.natCast_eq_natCast_iff]
  simpa using cast_templateValue_eq_outer_of_dvd_base conf seed modulus hmod

theorem templateValue_modEq_outer_base
    (conf : SymmetricTemplateConfig) (seed : ℕ) :
    templateValue conf seed ≡ conf.outer [MOD conf.base] := by
  exact templateValue_modEq_outer_of_dvd_base conf seed conf.base (dvd_rfl)

theorem templateValue_mod_base_eq_outer_mod
    (conf : SymmetricTemplateConfig) (seed : ℕ) :
    templateValue conf seed % conf.base = conf.outer % conf.base := by
  simpa [Nat.ModEq] using (templateValue_modEq_outer_base conf seed)

theorem templateValue_mod_base_eq_outer
    (conf : SymmetricTemplateConfig) (seed : ℕ) (hOuter : conf.outer < conf.base) :
    templateValue conf seed % conf.base = conf.outer := by
  rw [templateValue_mod_base_eq_outer_mod conf seed, Nat.mod_eq_of_lt hOuter]

theorem gcd_templateValue_base_eq_gcd_outer_base
    (conf : SymmetricTemplateConfig) (seed : ℕ) :
    Nat.gcd (templateValue conf seed) conf.base = Nat.gcd conf.outer conf.base := by
  exact Nat.ModEq.gcd_eq (templateValue_modEq_outer_base conf seed)

theorem templateValue_coprime_base_iff_outer_coprime_base
    (conf : SymmetricTemplateConfig) (seed : ℕ) :
    (templateValue conf seed).Coprime conf.base ↔ conf.outer.Coprime conf.base := by
  rw [Nat.coprime_iff_gcd_eq_one, Nat.coprime_iff_gcd_eq_one,
    gcd_templateValue_base_eq_gcd_outer_base]

theorem templateValue_coprime_base_of_outer_coprime_base
    (conf : SymmetricTemplateConfig) (seed : ℕ)
    (hOuter : conf.outer.Coprime conf.base) :
    (templateValue conf seed).Coprime conf.base :=
  (templateValue_coprime_base_iff_outer_coprime_base conf seed).2 hOuter

theorem outer_memUnitResidues_iff_templateValue_mod_memUnitResidues
    (conf : SymmetricTemplateConfig) (seed : ℕ) (hOuter : conf.outer < conf.base) :
    conf.outer ∈ unitResidues conf.base ↔
      templateValue conf seed % conf.base ∈ unitResidues conf.base := by
  rw [mem_unitResidues, mem_unitResidues, templateValue_mod_base_eq_outer conf seed hOuter]

theorem templateGradient_coprime_of_base_coprime
    (conf : SymmetricTemplateConfig) {modulus : ℕ}
    (hcop : modulus.Coprime conf.base) :
    (templateGradient conf).Coprime modulus := by
  unfold templateGradient
  simpa [Nat.coprime_comm] using hcop.pow_right (middlePosition conf)

noncomputable def templateGradientUnit
    (conf : SymmetricTemplateConfig) (modulus : ℕ)
    (hcop : modulus.Coprime conf.base) :
    (ZMod modulus)ˣ :=
  ZMod.unitOfCoprime (templateGradient conf)
    (templateGradient_coprime_of_base_coprime conf hcop)

noncomputable def residueAffineEquiv
    (conf : SymmetricTemplateConfig) (modulus : ℕ)
    (hcop : modulus.Coprime conf.base) :
    ZMod modulus ≃ ZMod modulus :=
  let u := templateGradientUnit conf modulus hcop
  { toFun := fun z => templateShift conf + z * (u : ZMod modulus)
    invFun := fun z => (z - templateShift conf) * ↑u⁻¹
    left_inv := by
      intro z
      simp [u, mul_assoc]
    right_inv := by
      intro z
      simp [u, mul_assoc] }

theorem residueAffineEquiv_apply_seed
    (conf : SymmetricTemplateConfig) (modulus seed : ℕ)
    (hcop : modulus.Coprime conf.base) :
    residueAffineEquiv conf modulus hcop seed = (templateValue conf seed : ZMod modulus) := by
  rw [show ((templateValue conf seed : ℕ) : ZMod modulus) =
      templateShift conf + seed * templateGradient conf by
      simpa using congrArg (fun n : ℕ => (n : ZMod modulus))
        (templateValue_eq_shift_add_gradient conf seed)]
  simp [residueAffineEquiv, templateGradientUnit]

theorem templateValue_modEq_iff_seed_modEq_of_coprime
    (conf : SymmetricTemplateConfig) {seed₁ seed₂ modulus : ℕ}
    (hcop : modulus.Coprime conf.base) :
    templateValue conf seed₁ ≡ templateValue conf seed₂ [MOD modulus] ↔
      seed₁ ≡ seed₂ [MOD modulus] := by
  constructor
  · intro h
    rw [← ZMod.natCast_eq_natCast_iff] at h
    have h' :
        residueAffineEquiv conf modulus hcop (seed₁ : ZMod modulus) =
          residueAffineEquiv conf modulus hcop (seed₂ : ZMod modulus) := by
      simpa [residueAffineEquiv_apply_seed] using h
    rw [← ZMod.natCast_eq_natCast_iff]
    exact (residueAffineEquiv conf modulus hcop).injective h'
  · intro h
    rw [← ZMod.natCast_eq_natCast_iff] at h
    rw [← ZMod.natCast_eq_natCast_iff]
    have h' :
        residueAffineEquiv conf modulus hcop (seed₁ : ZMod modulus) =
          residueAffineEquiv conf modulus hcop (seed₂ : ZMod modulus) := by
      exact congrArg (residueAffineEquiv conf modulus hcop) h
    simpa [residueAffineEquiv_apply_seed] using h'

end PrimeArithmetic.Structure
