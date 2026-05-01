import PrimeArithmetic.Structure.BoundedKTemplate
import PrimeArithmetic.Structure.AffineSeedClasses

namespace PrimeArithmetic.Structure

/-!
Exact bounded-`k` local residue profiles.

For a fixed bounded-`k` configuration and a finite family of moduli coprime to
the base, each modulus contributes one forbidden seed class: the unique seed
class modulo that modulus for which the affine template value is divisible by
the modulus.

This is the exact local object behind the direct lane-comparison theorem work.
-/

structure BoundedKResidueProfile where
  moduli : Finset ℕ
  excludedSeedClass : ℕ → ℕ

noncomputable def buildResidueProfile
    (cfg : BoundedKConfig) (base outer inner middleWidth : ℕ)
    (moduli : Finset ℕ)
    (hcop : ∀ modulus ∈ moduli, modulus.Coprime base) :
    BoundedKResidueProfile where
  moduli := moduli
  excludedSeedClass := fun modulus =>
    if h : modulus ∈ moduli then
      (zeroSeedClass (cfg.toSymmetricTemplateConfig base outer inner middleWidth) modulus
        (hcop modulus h)).val
    else
      0

def profileAgreement (left right : BoundedKResidueProfile) : Prop :=
  left.moduli = right.moduli ∧
    ∀ modulus ∈ left.moduli,
      left.excludedSeedClass modulus = right.excludedSeedClass modulus

def profileAgreementOn
    (cfgFrom cfgTo : BoundedKConfig)
    (base outer inner middleWidth : ℕ)
    (moduli : Finset ℕ)
    (hcop : ∀ modulus ∈ moduli, modulus.Coprime base) : Prop :=
  profileAgreement
    (buildResidueProfile cfgFrom base outer inner middleWidth moduli hcop)
    (buildResidueProfile cfgTo base outer inner middleWidth moduli hcop)

@[simp] theorem buildResidueProfile_moduli
    (cfg : BoundedKConfig) (base outer inner middleWidth : ℕ)
    (moduli : Finset ℕ)
    (hcop : ∀ modulus ∈ moduli, modulus.Coprime base) :
    (buildResidueProfile cfg base outer inner middleWidth moduli hcop).moduli = moduli := rfl

theorem buildResidueProfile_excludedSeedClass
    (cfg : BoundedKConfig) (base outer inner middleWidth : ℕ)
    (moduli : Finset ℕ)
    (hcop : ∀ modulus ∈ moduli, modulus.Coprime base)
    {modulus : ℕ} (hmod : modulus ∈ moduli) :
    (buildResidueProfile cfg base outer inner middleWidth moduli hcop).excludedSeedClass modulus =
      (zeroSeedClass (cfg.toSymmetricTemplateConfig base outer inner middleWidth) modulus
        (hcop modulus hmod)).val := by
  simp [buildResidueProfile, hmod]

theorem divisibilityFlag_eq_of_zeroSeedClass_val_eq
    (cfgFrom cfgTo : BoundedKConfig)
    (base outer inner middleWidth : ℕ)
    {modulus seed : ℕ} [NeZero modulus]
    (hcop : modulus.Coprime base)
    (hclass :
      (zeroSeedClass (cfgFrom.toSymmetricTemplateConfig base outer inner middleWidth) modulus hcop).val =
      (zeroSeedClass (cfgTo.toSymmetricTemplateConfig base outer inner middleWidth) modulus hcop).val) :
    templateValue (cfgFrom.toSymmetricTemplateConfig base outer inner middleWidth) seed % modulus = 0 ↔
      templateValue (cfgTo.toSymmetricTemplateConfig base outer inner middleWidth) seed % modulus = 0 := by
  rw [templateValue_mod_eq_zero_iff_seed_mod_eq_zeroSeedClass
      (cfgFrom.toSymmetricTemplateConfig base outer inner middleWidth) (modulus := modulus)
      (seed := seed) hcop]
  rw [templateValue_mod_eq_zero_iff_seed_mod_eq_zeroSeedClass
      (cfgTo.toSymmetricTemplateConfig base outer inner middleWidth) (modulus := modulus)
      (seed := seed) hcop]
  simp [hclass]

example :
    profileAgreementOn
      { kOuter := 0, kInner := 0 }
      { kOuter := 0, kInner := 0 }
      10 3 7 2
      ({11, 13} : Finset ℕ)
      (by decide) := by
  refine ⟨rfl, ?_⟩
  intro modulus hmod
  simp [buildResidueProfile]

end PrimeArithmetic.Structure
