import PrimeArithmetic.Structure.BoundedKResidueProfile
import PrimeArithmetic.Structure.FiniteMaskTransfer

namespace PrimeArithmetic.Structure

/-!
Transfer-collapse criteria for direct bounded-`k` lane comparisons.

This module turns local residue-profile agreement into exact candidatewise mask
agreement, and then into exact transfer-bucket consequences.

The statements remain purely arithmetic:
- no primality,
- no empirical threshold claim,
- only masks, admissibility, and exact bucket counts.
-/

open scoped BigOperators

noncomputable def seedMask
    (cfg : BoundedKConfig) (base outer inner middleWidth : ℕ)
    (moduli : Finset ℕ) (seed : ℕ) :
    DivMask ℕ :=
  moduli.filter fun modulus =>
    templateValue (cfg.toSymmetricTemplateConfig base outer inner middleWidth) seed % modulus = 0

theorem seedMask_eq_of_profileAgreementOn
    (cfgFrom cfgTo : BoundedKConfig)
    (base outer inner middleWidth : ℕ)
    (moduli : Finset ℕ)
    (hcop : ∀ modulus ∈ moduli, modulus.Coprime base)
    (hmod : ∀ modulus ∈ moduli, modulus ≠ 0)
    (hagree : profileAgreementOn cfgFrom cfgTo base outer inner middleWidth moduli hcop) :
    ∀ seed,
      seedMask cfgFrom base outer inner middleWidth moduli seed =
        seedMask cfgTo base outer inner middleWidth moduli seed := by
  intro seed
  ext modulus
  by_cases hm : modulus ∈ moduli
  · have hclass : (zeroSeedClass
        (cfgFrom.toSymmetricTemplateConfig base outer inner middleWidth) modulus
        (hcop modulus hm)).val =
      (zeroSeedClass
        (cfgTo.toSymmetricTemplateConfig base outer inner middleWidth) modulus
        (hcop modulus hm)).val := by
      calc
        (zeroSeedClass
          (cfgFrom.toSymmetricTemplateConfig base outer inner middleWidth) modulus
          (hcop modulus hm)).val
            =
          (buildResidueProfile cfgFrom base outer inner middleWidth moduli hcop).excludedSeedClass modulus := by
              symm
              exact buildResidueProfile_excludedSeedClass
                cfgFrom base outer inner middleWidth moduli hcop hm
        _ =
          (buildResidueProfile cfgTo base outer inner middleWidth moduli hcop).excludedSeedClass modulus := by
              exact hagree.2 modulus hm
        _ =
          (zeroSeedClass
            (cfgTo.toSymmetricTemplateConfig base outer inner middleWidth) modulus
            (hcop modulus hm)).val := by
              exact buildResidueProfile_excludedSeedClass
                cfgTo base outer inner middleWidth moduli hcop hm
    have hdiv :
        templateValue (cfgFrom.toSymmetricTemplateConfig base outer inner middleWidth) seed % modulus = 0 ↔
          templateValue (cfgTo.toSymmetricTemplateConfig base outer inner middleWidth) seed % modulus = 0 := by
      letI : NeZero modulus := ⟨hmod modulus hm⟩
      exact divisibilityFlag_eq_of_zeroSeedClass_val_eq
        cfgFrom cfgTo base outer inner middleWidth
        (modulus := modulus) (seed := seed) (hcop modulus hm) hclass
    simp [seedMask, hm, hdiv]
  · simp [seedMask, hm]

theorem transferBucket_ne_gainZero_self [DecidableEq ι] (mask : DivMask ι) :
    transferBucket mask mask ≠ .gainZero := by
  by_cases hEmpty : mask = ∅ <;> simp [transferBucket, hEmpty]

theorem transferBucket_ne_lossZero_self [DecidableEq ι] (mask : DivMask ι) :
    transferBucket mask mask ≠ .lossZero := by
  by_cases hEmpty : mask = ∅ <;> simp [transferBucket, hEmpty]

theorem transferBucket_ne_nonzeroChurn_self [DecidableEq ι] (mask : DivMask ι) :
    transferBucket mask mask ≠ .nonzeroChurn := by
  by_cases hEmpty : mask = ∅ <;> simp [transferBucket, hEmpty]

theorem gainZeroCount_eq_zero_of_mask_eq [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι)
    (hEq : ∀ a, maskFrom a = maskTo a) :
    bucketCount s maskFrom maskTo .gainZero = 0 := by
  classical
  unfold bucketCount bucketSet
  rw [Finset.card_eq_zero]
  ext a
  constructor
  · intro ha
    rcases Finset.mem_filter.mp ha with ⟨_, hbucket⟩
    exact False.elim <|
      (transferBucket_ne_gainZero_self (maskFrom a)) (by simpa [hEq a] using hbucket)
  · intro ha
    simp at ha

theorem lossZeroCount_eq_zero_of_mask_eq [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι)
    (hEq : ∀ a, maskFrom a = maskTo a) :
    bucketCount s maskFrom maskTo .lossZero = 0 := by
  classical
  unfold bucketCount bucketSet
  rw [Finset.card_eq_zero]
  ext a
  constructor
  · intro ha
    rcases Finset.mem_filter.mp ha with ⟨_, hbucket⟩
    exact False.elim <|
      (transferBucket_ne_lossZero_self (maskFrom a)) (by simpa [hEq a] using hbucket)
  · intro ha
    simp at ha

theorem nonzeroChurnCount_eq_zero_of_mask_eq [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι)
    (hEq : ∀ a, maskFrom a = maskTo a) :
    bucketCount s maskFrom maskTo .nonzeroChurn = 0 := by
  classical
  unfold bucketCount bucketSet
  rw [Finset.card_eq_zero]
  ext a
  constructor
  · intro ha
    rcases Finset.mem_filter.mp ha with ⟨_, hbucket⟩
    exact False.elim <|
      (transferBucket_ne_nonzeroChurn_self (maskFrom a)) (by simpa [hEq a] using hbucket)
  · intro ha
    simp at ha

theorem admissibleCount_eq_of_mask_eq [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι)
    (hEq : ∀ a, maskFrom a = maskTo a) :
    admissibleCountFrom s maskFrom maskTo = admissibleCountTo s maskFrom maskTo := by
  simp [admissibleCountFrom, admissibleCountTo,
    gainZeroCount_eq_zero_of_mask_eq s maskFrom maskTo hEq,
    lossZeroCount_eq_zero_of_mask_eq s maskFrom maskTo hEq]

theorem admissibleDeltaCount_eq_zero_of_mask_eq [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι)
    (hEq : ∀ a, maskFrom a = maskTo a) :
    admissibleDeltaCount s maskFrom maskTo = 0 := by
  simp [admissibleDeltaCount,
    gainZeroCount_eq_zero_of_mask_eq s maskFrom maskTo hEq,
    lossZeroCount_eq_zero_of_mask_eq s maskFrom maskTo hEq]

theorem no_positive_admissibleDelta_of_mask_eq [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι)
    (hEq : ∀ a, maskFrom a = maskTo a) :
    admissibleDeltaCount s maskFrom maskTo ≤ 0 := by
  have hZero : admissibleDeltaCount s maskFrom maskTo = 0 :=
    admissibleDeltaCount_eq_zero_of_mask_eq s maskFrom maskTo hEq
  rw [hZero]

theorem profileAgreement_implies_transferIdentity
    (cfgFrom cfgTo : BoundedKConfig)
    (base outer inner middleWidth : ℕ)
    (moduli : Finset ℕ)
    (hcop : ∀ modulus ∈ moduli, modulus.Coprime base)
    (hmod : ∀ modulus ∈ moduli, modulus ≠ 0)
    (s : Finset ℕ)
    (hagree : profileAgreementOn cfgFrom cfgTo base outer inner middleWidth moduli hcop) :
    bucketCount s
        (seedMask cfgFrom base outer inner middleWidth moduli)
        (seedMask cfgTo base outer inner middleWidth moduli)
        .gainZero = 0
      ∧ bucketCount s
          (seedMask cfgFrom base outer inner middleWidth moduli)
          (seedMask cfgTo base outer inner middleWidth moduli)
          .lossZero = 0
      ∧ bucketCount s
          (seedMask cfgFrom base outer inner middleWidth moduli)
          (seedMask cfgTo base outer inner middleWidth moduli)
          .nonzeroChurn = 0
      ∧ admissibleCountFrom s
          (seedMask cfgFrom base outer inner middleWidth moduli)
          (seedMask cfgTo base outer inner middleWidth moduli) =
        admissibleCountTo s
          (seedMask cfgFrom base outer inner middleWidth moduli)
          (seedMask cfgTo base outer inner middleWidth moduli)
      ∧ admissibleDeltaCount s
          (seedMask cfgFrom base outer inner middleWidth moduli)
          (seedMask cfgTo base outer inner middleWidth moduli) = 0 := by
  let hEq := seedMask_eq_of_profileAgreementOn
    cfgFrom cfgTo base outer inner middleWidth moduli hcop hmod hagree
  refine ⟨?_, ?_, ?_, ?_, ?_⟩
  · exact gainZeroCount_eq_zero_of_mask_eq s _ _ hEq
  · exact lossZeroCount_eq_zero_of_mask_eq s _ _ hEq
  · exact nonzeroChurnCount_eq_zero_of_mask_eq s _ _ hEq
  · exact admissibleCount_eq_of_mask_eq s _ _ hEq
  · exact admissibleDeltaCount_eq_zero_of_mask_eq s _ _ hEq

theorem gainZeroCount_eq_zero_of_admissibleAgreement [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι)
    (hAdmissible : ∀ a, maskFrom a = ∅ ↔ maskTo a = ∅) :
    bucketCount s maskFrom maskTo .gainZero = 0 := by
  classical
  unfold bucketCount bucketSet
  rw [Finset.card_eq_zero]
  ext a
  constructor
  · intro ha
    rcases Finset.mem_filter.mp ha with ⟨_, hbucket⟩
    have hNotEmptyFrom : maskFrom a ≠ ∅ := by
      exact (transferBucket_eq_gainZero_iff.mp hbucket).1
    have hEmptyTo : maskTo a = ∅ := by
      exact (transferBucket_eq_gainZero_iff.mp hbucket).2
    exact False.elim <| hNotEmptyFrom ((hAdmissible a).2 hEmptyTo)
  · intro ha
    simp at ha

theorem lossZeroCount_eq_zero_of_admissibleAgreement [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι)
    (hAdmissible : ∀ a, maskFrom a = ∅ ↔ maskTo a = ∅) :
    bucketCount s maskFrom maskTo .lossZero = 0 := by
  classical
  unfold bucketCount bucketSet
  rw [Finset.card_eq_zero]
  ext a
  constructor
  · intro ha
    rcases Finset.mem_filter.mp ha with ⟨_, hbucket⟩
    have hEmptyFrom : maskFrom a = ∅ := by
      exact (transferBucket_eq_lossZero_iff.mp hbucket).1
    have hNotEmptyTo : maskTo a ≠ ∅ := by
      exact (transferBucket_eq_lossZero_iff.mp hbucket).2
    exact False.elim <| hNotEmptyTo ((hAdmissible a).1 hEmptyFrom)
  · intro ha
    simp at ha

theorem admissibleAgreement_implies_no_positive_admissibleDelta
    [DecidableEq α] [DecidableEq ι]
    (s : Finset α) (maskFrom maskTo : α → DivMask ι)
    (hAdmissible :
      ∀ a, maskFrom a = ∅ ↔ maskTo a = ∅) :
    admissibleDeltaCount s maskFrom maskTo ≤ 0 := by
  have hGain : bucketCount s maskFrom maskTo .gainZero = 0 :=
    gainZeroCount_eq_zero_of_admissibleAgreement s maskFrom maskTo hAdmissible
  have hLoss : bucketCount s maskFrom maskTo .lossZero = 0 :=
    lossZeroCount_eq_zero_of_admissibleAgreement s maskFrom maskTo hAdmissible
  simp [admissibleDeltaCount, hGain, hLoss]

end PrimeArithmetic.Structure
