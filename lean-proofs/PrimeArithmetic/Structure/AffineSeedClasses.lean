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

theorem templateValue_mod_eq_target_mod_iff_seed_mod_eq_seedClass
    (conf : SymmetricTemplateConfig) {modulus seed target : ℕ}
    [NeZero modulus]
    (hcop : modulus.Coprime conf.base) :
    templateValue conf seed % modulus = target % modulus ↔
      seed % modulus = (seedClassForResidue conf modulus hcop target).val := by
  simpa [Nat.ModEq,
    Nat.mod_eq_of_lt (ZMod.val_lt (seedClassForResidue conf modulus hcop target))] using
    templateValue_modEq_target_iff_seed_modEq_seedClass
      (conf := conf) (modulus := modulus) (seed := seed) (target := target) hcop

theorem templateValue_mod_eq_zero_iff_seed_mod_eq_zeroSeedClass
    (conf : SymmetricTemplateConfig) {modulus seed : ℕ}
    [NeZero modulus]
    (hcop : modulus.Coprime conf.base) :
    templateValue conf seed % modulus = 0 ↔
      seed % modulus = (zeroSeedClass conf modulus hcop).val := by
  simpa [Nat.ModEq,
    Nat.mod_eq_of_lt (ZMod.val_lt (zeroSeedClass conf modulus hcop))] using
    templateValue_modEq_zero_iff_seed_modEq_zeroSeedClass
      (conf := conf) (modulus := modulus) (seed := seed) hcop

theorem templateValue_seedClassForResidue_val_modEq_target
    (conf : SymmetricTemplateConfig) {modulus target : ℕ}
    [NeZero modulus]
    (hcop : modulus.Coprime conf.base) :
    templateValue conf (seedClassForResidue conf modulus hcop target).val ≡ target [MOD modulus] := by
  rw [← ZMod.natCast_eq_natCast_iff]
  exact
    (templateValue_eq_target_iff_seed_eq_seedClass
      conf modulus (seedClassForResidue conf modulus hcop target).val hcop
      (target : ZMod modulus)).2 <|
      ZMod.natCast_zmod_val (seedClassForResidue conf modulus hcop target)

theorem templateValue_zeroSeedClass_val_modEq_zero
    (conf : SymmetricTemplateConfig) {modulus : ℕ}
    [NeZero modulus]
    (hcop : modulus.Coprime conf.base) :
    templateValue conf (zeroSeedClass conf modulus hcop).val ≡ 0 [MOD modulus] := by
  simpa [zeroSeedClass] using
    templateValue_seedClassForResidue_val_modEq_target
      (conf := conf) (modulus := modulus) (target := 0) hcop

@[simp] theorem seedClassForResidue_eq_iff
    (conf : SymmetricTemplateConfig) (modulus : ℕ)
    (hcop : modulus.Coprime conf.base)
    {target₁ target₂ : ZMod modulus} :
    seedClassForResidue conf modulus hcop target₁ =
        seedClassForResidue conf modulus hcop target₂ ↔
      target₁ = target₂ := by
  constructor
  · intro h
    have h' := congrArg (residueAffineEquiv conf modulus hcop) h
    simpa [residueAffineEquiv_seedClassForResidue] using h'
  · intro h
    simp [h]

theorem seedClassForResidue_zero_eq_zeroSeedClass
    (conf : SymmetricTemplateConfig) (modulus : ℕ)
    (hcop : modulus.Coprime conf.base) :
    seedClassForResidue conf modulus hcop 0 = zeroSeedClass conf modulus hcop := rfl

theorem seedClassForResidue_val_modEq_iff_target_modEq
    (conf : SymmetricTemplateConfig) {modulus target₁ target₂ : ℕ}
    [NeZero modulus]
    (hcop : modulus.Coprime conf.base) :
    (seedClassForResidue conf modulus hcop target₁).val ≡
        (seedClassForResidue conf modulus hcop target₂).val [MOD modulus] ↔
      target₁ ≡ target₂ [MOD modulus] := by
  constructor
  · intro h
    have hSeed :
        seedClassForResidue conf modulus hcop target₁ =
          seedClassForResidue conf modulus hcop target₂ := by
      calc
        seedClassForResidue conf modulus hcop target₁
            = ((seedClassForResidue conf modulus hcop target₁).val : ZMod modulus) := by
                symm
                exact ZMod.natCast_zmod_val _
        _ = ((seedClassForResidue conf modulus hcop target₂).val : ZMod modulus) := by
              exact (ZMod.natCast_eq_natCast_iff _ _ _).2 h
        _ = seedClassForResidue conf modulus hcop target₂ := by
              exact ZMod.natCast_zmod_val _
    exact (ZMod.natCast_eq_natCast_iff _ _ _).1 <|
      (seedClassForResidue_eq_iff conf modulus hcop).1 hSeed
  · intro h
    have hTarget : (target₁ : ZMod modulus) = target₂ := by
      exact (ZMod.natCast_eq_natCast_iff _ _ _).2 h
    have hSeed :
        seedClassForResidue conf modulus hcop target₁ =
          seedClassForResidue conf modulus hcop target₂ :=
      (seedClassForResidue_eq_iff conf modulus hcop).2 hTarget
    exact (ZMod.natCast_eq_natCast_iff _ _ _).1 <|
      calc
        ((seedClassForResidue conf modulus hcop target₁).val : ZMod modulus)
            = seedClassForResidue conf modulus hcop target₁ := by
                exact ZMod.natCast_zmod_val _
        _ = seedClassForResidue conf modulus hcop target₂ := hSeed
        _ = ((seedClassForResidue conf modulus hcop target₂).val : ZMod modulus) := by
              exact (ZMod.natCast_zmod_val _).symm

theorem seedClassForResidue_val_modEq_zeroSeedClass_iff_target_modEq_zero
    (conf : SymmetricTemplateConfig) {modulus target : ℕ}
    [NeZero modulus]
    (hcop : modulus.Coprime conf.base) :
    (seedClassForResidue conf modulus hcop target).val ≡
        (zeroSeedClass conf modulus hcop).val [MOD modulus] ↔
      target ≡ 0 [MOD modulus] := by
  simpa [zeroSeedClass] using
    seedClassForResidue_val_modEq_iff_target_modEq
      (conf := conf) (modulus := modulus) (target₁ := target) (target₂ := 0) hcop

theorem seedClassForResidue_val_eq_iff_target_mod_eq
    (conf : SymmetricTemplateConfig) {modulus target₁ target₂ : ℕ}
    [NeZero modulus]
    (hcop : modulus.Coprime conf.base) :
    (seedClassForResidue conf modulus hcop target₁).val =
        (seedClassForResidue conf modulus hcop target₂).val ↔
      target₁ % modulus = target₂ % modulus := by
  simpa [Nat.ModEq,
    Nat.mod_eq_of_lt (ZMod.val_lt (seedClassForResidue conf modulus hcop target₁)),
    Nat.mod_eq_of_lt (ZMod.val_lt (seedClassForResidue conf modulus hcop target₂))] using
    seedClassForResidue_val_modEq_iff_target_modEq
      (conf := conf) (modulus := modulus) (target₁ := target₁) (target₂ := target₂) hcop

theorem seedClassForResidue_val_eq_zeroSeedClass_iff_target_mod_eq_zero
    (conf : SymmetricTemplateConfig) {modulus target : ℕ}
    [NeZero modulus]
    (hcop : modulus.Coprime conf.base) :
    (seedClassForResidue conf modulus hcop target).val =
        (zeroSeedClass conf modulus hcop).val ↔
      target % modulus = 0 := by
  simpa [Nat.ModEq,
    Nat.mod_eq_of_lt (ZMod.val_lt (seedClassForResidue conf modulus hcop target)),
    Nat.mod_eq_of_lt (ZMod.val_lt (zeroSeedClass conf modulus hcop))] using
    seedClassForResidue_val_modEq_zeroSeedClass_iff_target_modEq_zero
      (conf := conf) (modulus := modulus) (target := target) hcop

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
