import Mathlib
import PrimeArithmetic.Structure.AffineResidueSearch

namespace PrimeArithmetic.Structure

/-!
Explicit seed classes induced by the affine residue map.

For any modulus coprime to the base, every target residue determines a unique
seed class modulo that modulus. In particular, divisibility by such a modulus
cuts out a single congruence class of seeds.
-/

noncomputable def seedClassForResidue
    (conf : SymmetricTemplateConfig) (modulus : ℕ)
    (hcop : modulus.Coprime conf.base) (target : ZMod modulus) :
    ZMod modulus :=
  (residueAffineEquiv conf modulus hcop).symm target

noncomputable def zeroSeedClass
    (conf : SymmetricTemplateConfig) (modulus : ℕ)
    (hcop : modulus.Coprime conf.base) :
    ZMod modulus :=
  seedClassForResidue conf modulus hcop 0

theorem residueAffineEquiv_seedClassForResidue
    (conf : SymmetricTemplateConfig) (modulus : ℕ)
    (hcop : modulus.Coprime conf.base) (target : ZMod modulus) :
    residueAffineEquiv conf modulus hcop (seedClassForResidue conf modulus hcop target) = target := by
  simp [seedClassForResidue]

theorem templateValue_eq_target_iff_seed_eq_seedClass
    (conf : SymmetricTemplateConfig) (modulus seed : ℕ)
    (hcop : modulus.Coprime conf.base) (target : ZMod modulus) :
    (templateValue conf seed : ZMod modulus) = target ↔
      (seed : ZMod modulus) = seedClassForResidue conf modulus hcop target := by
  rw [← residueAffineEquiv_apply_seed conf modulus seed hcop]
  exact (residueAffineEquiv conf modulus hcop).apply_eq_iff_eq_symm_apply

theorem templateValue_eq_zero_iff_seed_eq_zeroSeedClass
    (conf : SymmetricTemplateConfig) (modulus seed : ℕ)
    (hcop : modulus.Coprime conf.base) :
    (templateValue conf seed : ZMod modulus) = 0 ↔
      (seed : ZMod modulus) = zeroSeedClass conf modulus hcop := by
  simpa [zeroSeedClass] using
    templateValue_eq_target_iff_seed_eq_seedClass conf modulus seed hcop (0 : ZMod modulus)

theorem templateValue_modEq_target_iff_seed_modEq_seedClass
    (conf : SymmetricTemplateConfig) {modulus seed target : ℕ}
    [NeZero modulus]
    (hcop : modulus.Coprime conf.base) :
    templateValue conf seed ≡ target [MOD modulus] ↔
      seed ≡ (seedClassForResidue conf modulus hcop target).val [MOD modulus] := by
  constructor
  · intro h
    have h' : (templateValue conf seed : ZMod modulus) = target := by
      exact (ZMod.natCast_eq_natCast_iff _ _ _).2 h
    have hs : (seed : ZMod modulus) = seedClassForResidue conf modulus hcop target :=
      (templateValue_eq_target_iff_seed_eq_seedClass conf modulus seed hcop target).1 h'
    exact (ZMod.natCast_eq_natCast_iff _ _ _).1 <|
      hs.trans (ZMod.natCast_zmod_val _).symm
  · intro h
    have hs : (seed : ZMod modulus) = seedClassForResidue conf modulus hcop target := by
      exact ((ZMod.natCast_eq_natCast_iff _ _ _).2 h).trans (ZMod.natCast_zmod_val _)
    have h' : (templateValue conf seed : ZMod modulus) = target :=
      (templateValue_eq_target_iff_seed_eq_seedClass conf modulus seed hcop target).2 hs
    exact (ZMod.natCast_eq_natCast_iff _ _ _).1 h'

theorem templateValue_modEq_zero_iff_seed_modEq_zeroSeedClass
    (conf : SymmetricTemplateConfig) {modulus seed : ℕ}
    [NeZero modulus]
    (hcop : modulus.Coprime conf.base) :
    templateValue conf seed ≡ 0 [MOD modulus] ↔
      seed ≡ (zeroSeedClass conf modulus hcop).val [MOD modulus] := by
  simpa [zeroSeedClass] using
    templateValue_modEq_target_iff_seed_modEq_seedClass
      (conf := conf) (modulus := modulus) (seed := seed) (target := 0) hcop

theorem zeroSeedClass_base6_15_mod7 :
    zeroSeedClass base6_15 7 (by decide) = 1 := by
  have h : (templateValue base6_15 1 : ZMod 7) = 0 := by native_decide
  simpa [zeroSeedClass] using
    ((templateValue_eq_zero_iff_seed_eq_zeroSeedClass base6_15 7 1 (by decide)).1 h).symm

theorem zeroSeedClass_base10_37_mod11 :
    zeroSeedClass base10_37 11 (by decide) = 2 := by
  have h : (templateValue base10_37 2 : ZMod 11) = 0 := by native_decide
  simpa [zeroSeedClass] using
    ((templateValue_eq_zero_iff_seed_eq_zeroSeedClass base10_37 11 2 (by decide)).1 h).symm

theorem seedClassForResidue_base6_15_mod7_res3 :
    seedClassForResidue base6_15 7 (by decide) 3 = 4 := by
  have h : (templateValue base6_15 4 : ZMod 7) = 3 := by native_decide
  simpa using
    ((templateValue_eq_target_iff_seed_eq_seedClass base6_15 7 4 (by decide) (3 : ZMod 7)).1 h).symm

theorem seedClassForResidue_base10_37_mod11_res3 :
    seedClassForResidue base10_37 11 (by decide) 3 = 5 := by
  have h : (templateValue base10_37 5 : ZMod 11) = 3 := by native_decide
  simpa using
    ((templateValue_eq_target_iff_seed_eq_seedClass base10_37 11 5 (by decide) (3 : ZMod 11)).1 h).symm

end PrimeArithmetic.Structure
