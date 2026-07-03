import PrimeArithmetic.Generated.Witness.Teaching38
import PrimeArithmetic.Witness.SearchReplayCertificate

namespace PrimeArithmetic.Witness.TeachingSeedCertificate

open PrimeArithmetic.Structure

/-!
Theorem-facing wrapper for the generated 38-digit teaching witness certificate.

The generated module is the arithmetic source of truth, emitted from
`docs/witness/teaching38_proof_carrying_witness.json`.  This file keeps only the
human-facing names and compact aggregate statements used by the witness lane.
It proves construction and residue-funnel arithmetic only; it does not certify
primality.
-/

abbrev teaching38Config : SymmetricTemplateConfig :=
  PrimeArithmetic.Generated.Witness.Teaching38.config

abbrev teaching38Seed : ℕ :=
  PrimeArithmetic.Generated.Witness.Teaching38.witnessSeed

abbrev teaching38ResidueModuli : List ℕ :=
  PrimeArithmetic.Generated.Witness.Teaching38.residueModuli

abbrev teaching38RejectionExamples : List (ℕ × ℕ) :=
  PrimeArithmetic.Generated.Witness.Teaching38.rejectionExamples

abbrev teaching38SearchReplaySeeds : List ℕ :=
  PrimeArithmetic.Generated.Witness.Teaching38.searchReplaySeeds

abbrev teaching38SearchReplayResidueRejections : List (ℕ × ℕ) :=
  PrimeArithmetic.Generated.Witness.Teaching38.searchReplayResidueRejections

abbrev teaching38SearchReplayResidueSurvivors : List ℕ :=
  PrimeArithmetic.Generated.Witness.Teaching38.searchReplayResidueSurvivors

theorem teaching38_width : width teaching38Config = 38 :=
  PrimeArithmetic.Generated.Witness.Teaching38.width_value

theorem teaching38_shift :
    templateShift teaching38Config =
      30070000000000000000000000000000007003 :=
  PrimeArithmetic.Generated.Witness.Teaching38.shift_value

theorem teaching38_gradient :
    templateGradient teaching38Config = 100000 :=
  PrimeArithmetic.Generated.Witness.Teaching38.gradient_value

theorem teaching38_value :
    templateValue teaching38Config teaching38Seed =
      30070000000000000000000000000000307003 :=
  PrimeArithmetic.Generated.Witness.Teaching38.witness_value

theorem teaching38_value_eq_shift_add_gradient :
    templateValue teaching38Config teaching38Seed =
      templateShift teaching38Config + teaching38Seed * templateGradient teaching38Config :=
  PrimeArithmetic.Generated.Witness.Teaching38.witness_value_eq_shift_add_gradient

theorem teaching38_residue_moduli_nodup : teaching38ResidueModuli.Nodup :=
  PrimeArithmetic.Generated.Witness.Teaching38.residueModuli_nodup

theorem teaching38_residue_funnel_affine_checks
    {modulus : ℕ} (h : modulus ∈ teaching38ResidueModuli) :
    templateValue teaching38Config teaching38Seed % modulus =
      (templateShift teaching38Config % modulus +
        (templateGradient teaching38Config % modulus) * (teaching38Seed % modulus)) %
          modulus :=
  PrimeArithmetic.Generated.Witness.Teaching38.residueFunnelAffineChecks h

theorem teaching38_residue_funnel_survives
    {modulus : ℕ} (h : modulus ∈ teaching38ResidueModuli) :
    templateValue teaching38Config teaching38Seed % modulus ≠ 0 :=
  PrimeArithmetic.Generated.Witness.Teaching38.residueFunnelSurvives h

theorem teaching38_rejection_examples_reject
    {seed modulus : ℕ} (h : (seed, modulus) ∈ teaching38RejectionExamples) :
    templateValue teaching38Config seed % modulus = 0 :=
  PrimeArithmetic.Generated.Witness.Teaching38.rejectionExamplesReject h

theorem teaching38_search_replay_seeds_length :
    teaching38SearchReplaySeeds.length = 4 :=
  PrimeArithmetic.Generated.Witness.Teaching38.searchReplaySeeds_length

theorem teaching38_search_replay_witness_seed :
    PrimeArithmetic.Generated.Witness.Teaching38.inputSeed +
      PrimeArithmetic.Generated.Witness.Teaching38.searchReplayWitnessOffset =
        teaching38Seed :=
  PrimeArithmetic.Generated.Witness.Teaching38.searchReplayWitnessSeed

theorem teaching38_search_replay_rejections_reject
    {seed modulus : ℕ} (h : (seed, modulus) ∈ teaching38SearchReplayResidueRejections) :
    templateValue teaching38Config seed % modulus = 0 :=
  PrimeArithmetic.Generated.Witness.Teaching38.searchReplayResidueRejectionsReject h

theorem teaching38_search_replay_survivors_survive
    {seed modulus : ℕ} (hSeed : seed ∈ teaching38SearchReplayResidueSurvivors)
    (hModulus : modulus ∈ teaching38ResidueModuli) :
    templateValue teaching38Config seed % modulus ≠ 0 :=
  PrimeArithmetic.Generated.Witness.Teaching38.searchReplayResidueSurvivorsSurvive
    hSeed hModulus

abbrev teaching38SearchReplayCertificate : SearchReplayCertificate :=
  PrimeArithmetic.Generated.Witness.Teaching38.searchReplayCertificate

theorem teaching38_pre_witness_replay_complete :
    teaching38SearchReplayCertificate.PreWitnessComplete :=
  PrimeArithmetic.Generated.Witness.Teaching38.searchReplayPreWitnessComplete

theorem teaching38_search_replay_witness_survives :
    teaching38SearchReplayCertificate.WitnessSurvives :=
  PrimeArithmetic.Generated.Witness.Teaching38.searchReplayWitnessSurvives

theorem teaching38_search_replay_sound :
    teaching38SearchReplayCertificate.Sound :=
  PrimeArithmetic.Generated.Witness.Teaching38.searchReplaySound

theorem teaching38_search_replay_survivor_list_exact :
    teaching38SearchReplayCertificate.SurvivorListExact :=
  PrimeArithmetic.Generated.Witness.Teaching38.searchReplaySurvivorListExact

theorem teaching38_search_replay_first_accepted_survivor :
    teaching38SearchReplayCertificate.FirstAcceptedSurvivor :=
  PrimeArithmetic.Generated.Witness.Teaching38.searchReplayFirstAcceptedSurvivor

end PrimeArithmetic.Witness.TeachingSeedCertificate
