import PrimeArithmetic.Structure.BoundedKTemplate
import PrimeArithmetic.Structure.AffineSeedClasses

namespace PrimeArithmetic.Structure

open scoped BigOperators

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

structure BoundedKFamilyLane where
  familyCode : String
  base : ℕ
  outer : ℕ
  inner : ℕ
  middleWidth : ℕ
  cfg : BoundedKConfig
deriving DecidableEq, Repr

def BoundedKFamilyLane.toSymmetricTemplateConfig
    (lane : BoundedKFamilyLane) : SymmetricTemplateConfig :=
  lane.cfg.toSymmetricTemplateConfig lane.base lane.outer lane.inner lane.middleWidth

@[simp] theorem BoundedKFamilyLane.toSymmetricTemplateConfig_base
    (lane : BoundedKFamilyLane) :
    lane.toSymmetricTemplateConfig.base = lane.base := rfl

noncomputable def BoundedKFamilyLane.zeroSeedClassAt
    (lane : BoundedKFamilyLane) (modulus : ℕ)
    (hcop : modulus.Coprime lane.base) :
    ZMod modulus :=
  zeroSeedClass lane.toSymmetricTemplateConfig modulus (by
    simpa [BoundedKFamilyLane.toSymmetricTemplateConfig] using hcop)

def BoundedKFamilyLane.forbiddenSeedMaskAt
    (lane : BoundedKFamilyLane) (modulus : ℕ) : Set ℕ :=
  forbiddenSeedMask lane.toSymmetricTemplateConfig modulus

noncomputable def BoundedKFamilyLane.forbiddenResiduesAt
    (lane : BoundedKFamilyLane) (modulus : ℕ)
    [NeZero modulus]
    (hcop : modulus.Coprime lane.base) :
    Finset (ZMod modulus) :=
  {lane.zeroSeedClassAt modulus hcop}

noncomputable def BoundedKFamilyLane.survivorResiduesAt
    (lane : BoundedKFamilyLane) (modulus : ℕ)
    [NeZero modulus]
    (hcop : modulus.Coprime lane.base) :
    Finset (ZMod modulus) :=
  (Finset.univ : Finset (ZMod modulus)).erase (lane.zeroSeedClassAt modulus hcop)

noncomputable def BoundedKFamilyLane.sharedSurvivorResiduesAt
    (left right : BoundedKFamilyLane) (modulus : ℕ)
    [NeZero modulus]
    (hcopLeft : modulus.Coprime left.base)
    (hcopRight : modulus.Coprime right.base) :
    Finset (ZMod modulus) :=
  left.survivorResiduesAt modulus hcopLeft ∩
    right.survivorResiduesAt modulus hcopRight

noncomputable def BoundedKFamilyLane.survivorResidueProductAt
    (lane : BoundedKFamilyLane)
    (moduli : Finset ℕ)
    (hne : ∀ modulus ∈ moduli, NeZero modulus)
    (hcop : ∀ modulus ∈ moduli, modulus.Coprime lane.base) :
    ℕ :=
  moduli.attach.prod fun modulus => by
    letI : NeZero modulus.1 := hne modulus.1 modulus.2
    exact (lane.survivorResiduesAt modulus.1 (hcop modulus.1 modulus.2)).card

noncomputable def BoundedKFamilyLane.sharedSurvivorResidueProductAt
    (left right : BoundedKFamilyLane)
    (moduli : Finset ℕ)
    (hne : ∀ modulus ∈ moduli, NeZero modulus)
    (hcopLeft : ∀ modulus ∈ moduli, modulus.Coprime left.base)
    (hcopRight : ∀ modulus ∈ moduli, modulus.Coprime right.base) :
    ℕ :=
  moduli.attach.prod fun modulus => by
    letI : NeZero modulus.1 := hne modulus.1 modulus.2
    exact (left.sharedSurvivorResiduesAt right modulus.1
      (hcopLeft modulus.1 modulus.2) (hcopRight modulus.1 modulus.2)).card

noncomputable def BoundedKFamilyLane.sharedSurvivorFactorAt
    (left right : BoundedKFamilyLane) (modulus : ℕ)
    [NeZero modulus]
    (hcopLeft : modulus.Coprime left.base)
    (hcopRight : modulus.Coprime right.base) :
    ℕ :=
  if left.zeroSeedClassAt modulus hcopLeft =
      right.zeroSeedClassAt modulus hcopRight then
    modulus - 1
  else
    modulus - 2

noncomputable def BoundedKFamilyLane.residueProfile
    (lane : BoundedKFamilyLane)
    (moduli : Finset ℕ)
    (hcop : ∀ modulus ∈ moduli, modulus.Coprime lane.base) :
    BoundedKResidueProfile :=
  buildResidueProfile lane.cfg lane.base lane.outer lane.inner lane.middleWidth moduli hcop

noncomputable def BoundedKFamilyLane.residueProfileAt
    (lane : BoundedKFamilyLane) (modulus : ℕ)
    (hcop : modulus.Coprime lane.base) :
    BoundedKResidueProfile :=
  lane.residueProfile ({modulus} : Finset ℕ) (by
    intro m hm
    rcases Finset.mem_singleton.mp hm with rfl
    exact hcop)

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

theorem BoundedKFamilyLane.residueProfile_excludedSeedClass
    (lane : BoundedKFamilyLane)
    (moduli : Finset ℕ)
    (hcop : ∀ modulus ∈ moduli, modulus.Coprime lane.base)
    {modulus : ℕ} (hmod : modulus ∈ moduli) :
    (lane.residueProfile moduli hcop).excludedSeedClass modulus =
      (lane.zeroSeedClassAt modulus (hcop modulus hmod)).val := by
  simp [BoundedKFamilyLane.residueProfile, BoundedKFamilyLane.zeroSeedClassAt,
    BoundedKFamilyLane.toSymmetricTemplateConfig, buildResidueProfile_excludedSeedClass,
    hmod]

theorem BoundedKFamilyLane.residueProfileAt_excludedSeedClass
    (lane : BoundedKFamilyLane) (modulus : ℕ)
    (hcop : modulus.Coprime lane.base) :
    (lane.residueProfileAt modulus hcop).excludedSeedClass modulus =
      (lane.zeroSeedClassAt modulus hcop).val := by
  simp [BoundedKFamilyLane.residueProfileAt,
    BoundedKFamilyLane.residueProfile_excludedSeedClass]

theorem BoundedKFamilyLane.templateValue_mod_eq_zero_iff_seed_mod_eq_zeroSeedClassAt
    (lane : BoundedKFamilyLane) {modulus seed : ℕ}
    [NeZero modulus]
    (hcop : modulus.Coprime lane.base) :
    templateValue lane.toSymmetricTemplateConfig seed % modulus = 0 ↔
      seed % modulus = (lane.zeroSeedClassAt modulus hcop).val := by
  simpa [BoundedKFamilyLane.zeroSeedClassAt] using
    (templateValue_mod_eq_zero_iff_seed_mod_eq_zeroSeedClass
      (conf := lane.toSymmetricTemplateConfig) (modulus := modulus) (seed := seed)
      (by simpa [BoundedKFamilyLane.toSymmetricTemplateConfig] using hcop))

@[simp] theorem BoundedKFamilyLane.forbiddenResiduesAt_card
    (lane : BoundedKFamilyLane) (modulus : ℕ)
    [NeZero modulus]
    (hcop : modulus.Coprime lane.base) :
    (lane.forbiddenResiduesAt modulus hcop).card = 1 := by
  simp [BoundedKFamilyLane.forbiddenResiduesAt]

theorem BoundedKFamilyLane.forbiddenResiduesAt_eq_iff_zeroSeedClassAt_eq
    (left right : BoundedKFamilyLane) {modulus : ℕ}
    [NeZero modulus]
    (hcopLeft : modulus.Coprime left.base)
    (hcopRight : modulus.Coprime right.base) :
    left.forbiddenResiduesAt modulus hcopLeft =
        right.forbiddenResiduesAt modulus hcopRight ↔
      left.zeroSeedClassAt modulus hcopLeft =
        right.zeroSeedClassAt modulus hcopRight := by
  simp [BoundedKFamilyLane.forbiddenResiduesAt]

theorem BoundedKFamilyLane.forbiddenResiduesAt_ne_of_zeroSeedClassAt_ne
    (left right : BoundedKFamilyLane) {modulus : ℕ}
    [NeZero modulus]
    (hcopLeft : modulus.Coprime left.base)
    (hcopRight : modulus.Coprime right.base)
    (hne :
      left.zeroSeedClassAt modulus hcopLeft ≠ right.zeroSeedClassAt modulus hcopRight) :
    left.forbiddenResiduesAt modulus hcopLeft ≠
      right.forbiddenResiduesAt modulus hcopRight := by
  intro h
  exact hne
    ((BoundedKFamilyLane.forbiddenResiduesAt_eq_iff_zeroSeedClassAt_eq
      left right hcopLeft hcopRight).1 h)

theorem BoundedKFamilyLane.survivorResiduesAt_card
    (lane : BoundedKFamilyLane) (modulus : ℕ)
    [NeZero modulus]
    (hcop : modulus.Coprime lane.base) :
    (lane.survivorResiduesAt modulus hcop).card = modulus - 1 := by
  rw [BoundedKFamilyLane.survivorResiduesAt]
  rw [Finset.card_erase_of_mem (Finset.mem_univ _)]
  simp [ZMod.card]

theorem BoundedKFamilyLane.survivorResiduesAt_card_eq
    (left right : BoundedKFamilyLane) {modulus : ℕ}
    [NeZero modulus]
    (hcopLeft : modulus.Coprime left.base)
    (hcopRight : modulus.Coprime right.base) :
    (left.survivorResiduesAt modulus hcopLeft).card =
      (right.survivorResiduesAt modulus hcopRight).card := by
  rw [BoundedKFamilyLane.survivorResiduesAt_card left modulus hcopLeft,
    BoundedKFamilyLane.survivorResiduesAt_card right modulus hcopRight]

theorem erased_univ_inter_erased_univ_card_eq_of_eq
    {modulus : ℕ} [NeZero modulus] {leftClass rightClass : ZMod modulus}
    (heq : leftClass = rightClass) :
    (((Finset.univ : Finset (ZMod modulus)).erase leftClass) ∩
        ((Finset.univ : Finset (ZMod modulus)).erase rightClass)).card =
      modulus - 1 := by
  subst rightClass
  rw [Finset.inter_self]
  rw [Finset.card_erase_of_mem (Finset.mem_univ leftClass)]
  simp [ZMod.card]

theorem erased_univ_inter_erased_univ_card_eq_of_ne
    {modulus : ℕ} [NeZero modulus] {leftClass rightClass : ZMod modulus}
    (hne : leftClass ≠ rightClass) :
    (((Finset.univ : Finset (ZMod modulus)).erase leftClass) ∩
        ((Finset.univ : Finset (ZMod modulus)).erase rightClass)).card =
      modulus - 2 := by
  have hset :
      ((Finset.univ : Finset (ZMod modulus)).erase leftClass) ∩
          ((Finset.univ : Finset (ZMod modulus)).erase rightClass) =
        ((Finset.univ : Finset (ZMod modulus)).erase leftClass).erase rightClass := by
    ext residue
    simp [Finset.mem_erase, and_comm]
  rw [hset]
  rw [Finset.card_erase_of_mem]
  · rw [Finset.card_erase_of_mem (Finset.mem_univ leftClass)]
    rw [Nat.sub_sub]
    simp [ZMod.card]
  · simp [Finset.mem_erase, hne.symm]

theorem BoundedKFamilyLane.sharedSurvivorResiduesAt_card_eq_of_zeroSeedClassAt_eq
    (left right : BoundedKFamilyLane) {modulus : ℕ}
    [NeZero modulus]
    (hcopLeft : modulus.Coprime left.base)
    (hcopRight : modulus.Coprime right.base)
    (heq :
      left.zeroSeedClassAt modulus hcopLeft =
        right.zeroSeedClassAt modulus hcopRight) :
    (left.sharedSurvivorResiduesAt right modulus hcopLeft hcopRight).card =
      modulus - 1 := by
  simpa [BoundedKFamilyLane.sharedSurvivorResiduesAt,
    BoundedKFamilyLane.survivorResiduesAt] using
    (erased_univ_inter_erased_univ_card_eq_of_eq
      (modulus := modulus) (leftClass := left.zeroSeedClassAt modulus hcopLeft)
      (rightClass := right.zeroSeedClassAt modulus hcopRight) heq)

theorem BoundedKFamilyLane.sharedSurvivorResiduesAt_card_eq_of_zeroSeedClassAt_ne
    (left right : BoundedKFamilyLane) {modulus : ℕ}
    [NeZero modulus]
    (hcopLeft : modulus.Coprime left.base)
    (hcopRight : modulus.Coprime right.base)
    (hne :
      left.zeroSeedClassAt modulus hcopLeft ≠
        right.zeroSeedClassAt modulus hcopRight) :
    (left.sharedSurvivorResiduesAt right modulus hcopLeft hcopRight).card =
      modulus - 2 := by
  simpa [BoundedKFamilyLane.sharedSurvivorResiduesAt,
    BoundedKFamilyLane.survivorResiduesAt] using
    (erased_univ_inter_erased_univ_card_eq_of_ne
      (modulus := modulus) (leftClass := left.zeroSeedClassAt modulus hcopLeft)
      (rightClass := right.zeroSeedClassAt modulus hcopRight) hne)

theorem BoundedKFamilyLane.sharedSurvivorResiduesAt_card_eq_of_forbiddenResiduesAt_eq
    (left right : BoundedKFamilyLane) {modulus : ℕ}
    [NeZero modulus]
    (hcopLeft : modulus.Coprime left.base)
    (hcopRight : modulus.Coprime right.base)
    (heq :
      left.forbiddenResiduesAt modulus hcopLeft =
        right.forbiddenResiduesAt modulus hcopRight) :
    (left.sharedSurvivorResiduesAt right modulus hcopLeft hcopRight).card =
      modulus - 1 :=
  BoundedKFamilyLane.sharedSurvivorResiduesAt_card_eq_of_zeroSeedClassAt_eq
    left right hcopLeft hcopRight
    ((BoundedKFamilyLane.forbiddenResiduesAt_eq_iff_zeroSeedClassAt_eq
      left right hcopLeft hcopRight).1 heq)

theorem BoundedKFamilyLane.sharedSurvivorResiduesAt_card_eq_of_forbiddenResiduesAt_ne
    (left right : BoundedKFamilyLane) {modulus : ℕ}
    [NeZero modulus]
    (hcopLeft : modulus.Coprime left.base)
    (hcopRight : modulus.Coprime right.base)
    (hne :
      left.forbiddenResiduesAt modulus hcopLeft ≠
        right.forbiddenResiduesAt modulus hcopRight) :
    (left.sharedSurvivorResiduesAt right modulus hcopLeft hcopRight).card =
      modulus - 2 :=
  BoundedKFamilyLane.sharedSurvivorResiduesAt_card_eq_of_zeroSeedClassAt_ne
    left right hcopLeft hcopRight (by
      intro hclass
      exact hne
        ((BoundedKFamilyLane.forbiddenResiduesAt_eq_iff_zeroSeedClassAt_eq
          left right hcopLeft hcopRight).2 hclass))

theorem BoundedKFamilyLane.sharedSurvivorResiduesAt_card_eq_factor
    (left right : BoundedKFamilyLane) {modulus : ℕ}
    [NeZero modulus]
    (hcopLeft : modulus.Coprime left.base)
    (hcopRight : modulus.Coprime right.base) :
    (left.sharedSurvivorResiduesAt right modulus hcopLeft hcopRight).card =
      left.sharedSurvivorFactorAt right modulus hcopLeft hcopRight := by
  by_cases hclass :
      left.zeroSeedClassAt modulus hcopLeft =
        right.zeroSeedClassAt modulus hcopRight
  · simp [BoundedKFamilyLane.sharedSurvivorFactorAt, hclass,
      BoundedKFamilyLane.sharedSurvivorResiduesAt_card_eq_of_zeroSeedClassAt_eq
        left right hcopLeft hcopRight hclass]
  · simp [BoundedKFamilyLane.sharedSurvivorFactorAt, hclass,
      BoundedKFamilyLane.sharedSurvivorResiduesAt_card_eq_of_zeroSeedClassAt_ne
        left right hcopLeft hcopRight hclass]

theorem BoundedKFamilyLane.survivorResidueProductAt_eq
    (lane : BoundedKFamilyLane)
    (moduli : Finset ℕ)
    (hne : ∀ modulus ∈ moduli, NeZero modulus)
    (hcop : ∀ modulus ∈ moduli, modulus.Coprime lane.base) :
    lane.survivorResidueProductAt moduli hne hcop =
      moduli.attach.prod (fun modulus => modulus.1 - 1) := by
  simp [BoundedKFamilyLane.survivorResidueProductAt,
    BoundedKFamilyLane.survivorResiduesAt_card]

theorem BoundedKFamilyLane.survivorResidueProductAt_eq_between
    (left right : BoundedKFamilyLane)
    (moduli : Finset ℕ)
    (hne : ∀ modulus ∈ moduli, NeZero modulus)
    (hcopLeft : ∀ modulus ∈ moduli, modulus.Coprime left.base)
    (hcopRight : ∀ modulus ∈ moduli, modulus.Coprime right.base) :
    left.survivorResidueProductAt moduli hne hcopLeft =
      right.survivorResidueProductAt moduli hne hcopRight := by
  rw [BoundedKFamilyLane.survivorResidueProductAt_eq left moduli hne hcopLeft,
    BoundedKFamilyLane.survivorResidueProductAt_eq right moduli hne hcopRight]

theorem BoundedKFamilyLane.sharedSurvivorResidueProductAt_eq_factorProduct
    (left right : BoundedKFamilyLane)
    (moduli : Finset ℕ)
    (hne : ∀ modulus ∈ moduli, NeZero modulus)
    (hcopLeft : ∀ modulus ∈ moduli, modulus.Coprime left.base)
    (hcopRight : ∀ modulus ∈ moduli, modulus.Coprime right.base) :
    left.sharedSurvivorResidueProductAt right moduli hne hcopLeft hcopRight =
      moduli.attach.prod (fun modulus => by
        letI : NeZero modulus.1 := hne modulus.1 modulus.2
        exact left.sharedSurvivorFactorAt right modulus.1
          (hcopLeft modulus.1 modulus.2) (hcopRight modulus.1 modulus.2)) := by
  simp [BoundedKFamilyLane.sharedSurvivorResidueProductAt,
    BoundedKFamilyLane.sharedSurvivorResiduesAt_card_eq_factor]

theorem BoundedKFamilyLane.zeroSeedClassAt_eq_of_templateValue_zmod_eq_zero
    (lane : BoundedKFamilyLane) {modulus seed : ℕ}
    [NeZero modulus]
    (hcop : modulus.Coprime lane.base)
    (hzero : (templateValue lane.toSymmetricTemplateConfig seed : ZMod modulus) = 0) :
    lane.zeroSeedClassAt modulus hcop = seed := by
  simpa [BoundedKFamilyLane.zeroSeedClassAt] using
    ((templateValue_eq_zero_iff_seed_eq_zeroSeedClass
      lane.toSymmetricTemplateConfig modulus seed
      (by simpa [BoundedKFamilyLane.toSymmetricTemplateConfig] using hcop)).1 hzero).symm

structure BoundedKFamilyLaneProfileCertificate where
  lane : BoundedKFamilyLane
  modulus : ℕ
  excludedSeedClass : ℕ
  modulus_neZero : NeZero modulus
  hcop : modulus.Coprime lane.base
  zeroSeedClass_val_eq : (lane.zeroSeedClassAt modulus hcop).val = excludedSeedClass

noncomputable def BoundedKFamilyLaneProfileCertificate.profile
    (cert : BoundedKFamilyLaneProfileCertificate) :
    BoundedKResidueProfile :=
  cert.lane.residueProfileAt cert.modulus cert.hcop

theorem BoundedKFamilyLaneProfileCertificate.profile_excludedSeedClass
    (cert : BoundedKFamilyLaneProfileCertificate) :
    cert.profile.excludedSeedClass cert.modulus = cert.excludedSeedClass := by
  simpa [BoundedKFamilyLaneProfileCertificate.profile, cert.zeroSeedClass_val_eq] using
    (BoundedKFamilyLane.residueProfileAt_excludedSeedClass
      cert.lane cert.modulus cert.hcop)

theorem BoundedKFamilyLaneProfileCertificate.templateValue_mod_eq_zero_iff_seed_mod_eq
    (cert : BoundedKFamilyLaneProfileCertificate) (seed : ℕ) :
    templateValue cert.lane.toSymmetricTemplateConfig seed % cert.modulus = 0 ↔
      seed % cert.modulus = cert.excludedSeedClass := by
  letI : NeZero cert.modulus := cert.modulus_neZero
  simpa [cert.zeroSeedClass_val_eq] using
    (BoundedKFamilyLane.templateValue_mod_eq_zero_iff_seed_mod_eq_zeroSeedClassAt
      (lane := cert.lane) (modulus := cert.modulus) (seed := seed) cert.hcop)

theorem BoundedKFamilyLane.forbiddenSeedMaskAt_ne_of_zeroSeedClassAt_ne
    (left right : BoundedKFamilyLane) {modulus : ℕ}
    [NeZero modulus]
    (hcopLeft : modulus.Coprime left.base)
    (hcopRight : modulus.Coprime right.base)
    (hne :
      left.zeroSeedClassAt modulus hcopLeft ≠ right.zeroSeedClassAt modulus hcopRight) :
    left.forbiddenSeedMaskAt modulus ≠ right.forbiddenSeedMaskAt modulus := by
  simpa [BoundedKFamilyLane.zeroSeedClassAt, BoundedKFamilyLane.forbiddenSeedMaskAt] using
    (forbiddenSeedMask_ne_of_zeroSeedClass_ne
      left.toSymmetricTemplateConfig right.toSymmetricTemplateConfig
      (by simpa [BoundedKFamilyLane.toSymmetricTemplateConfig] using hcopLeft)
      (by simpa [BoundedKFamilyLane.toSymmetricTemplateConfig] using hcopRight)
      hne)

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
