//! Seed-to-witness demo helpers.
//!
//! A seed in this module is a starting point on an affine membrane lane, not a
//! promise that the exact seed is prime. The search walks forward until the
//! first residue-admissible probable-prime witness is found.

use crate::validation::large_affine_witness::{
    build_big_affine_lane, candidate_value, classify_mersenne, compact_description,
    is_probable_prime_fixed_bases, middle_digits, residue_allows_seed, residue_moduli,
    template_digits, BigAffineLane, PRIMARY_BASE, PRIMARY_INNER, PRIMARY_K, PRIMARY_OUTER,
    PROBABLE_PRIME_BASES,
};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt,
    time::Instant,
};

pub const DEFAULT_VISIBLE_DIGITS: usize = 128;
pub const DEFAULT_MAX_STEPS: u64 = 20_000;
pub const PROOF_CARRYING_WITNESS_SCHEMA_VERSION: &str = "proof-carrying-witness-v1";
pub const PROOF_CARRYING_WITNESS_MANIFEST_SCHEMA_VERSION: &str =
    "proof-carrying-witness-manifest-v1";
pub const PROOF_CARRYING_WITNESS_LEAN_CATALOG_SCHEMA_VERSION: &str =
    "proof-carrying-witness-lean-catalog-v1";
pub const PROOF_CARRYING_WITNESS_SEARCH_POLICY_ATLAS_SCHEMA_VERSION: &str =
    "proof-carrying-witness-search-policy-atlas-v1";
pub const PROOF_CARRYING_WITNESS_POLICY_MATRIX_SCHEMA_VERSION: &str =
    "proof-carrying-witness-policy-matrix-v1";
pub const PROOF_CARRYING_WITNESS_POLICY_MATRIX_ATLAS_SCHEMA_VERSION: &str =
    "proof-carrying-witness-policy-matrix-atlas-v1";
pub const PROOF_CARRYING_WITNESS_ARTIFACT_SET_ID: &str = "canonical-proof-carrying-witnesses-v1";
pub const PROOF_CARRYING_WITNESS_POLICY_MATRIX_ID: &str =
    "proof-carrying-witness-policy-matrix-smoke-v1";
pub const PROOF_CARRYING_WITNESS_POLICY_MATRIX_LEAN_CATALOG_MANIFEST: &str =
    "docs/witness/witness_policy_matrix_lean_catalog_manifest.json";
pub const PROBABLE_PRIME_NOT_PROOF_CERTIFIED: &str = "probable-prime-not-proof-certified";
const DEFAULT_REJECTION_EXAMPLE_COUNT: usize = 3;
const DEFAULT_REJECTION_SCAN_EXTRA: u64 = 32;

#[derive(Debug, Clone, Serialize)]
pub struct SeedToWitnessConfig {
    pub input_seed: u64,
    pub max_steps: u64,
    pub exact_seed_only: bool,
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub k_outer: u32,
    pub k_inner: u32,
    pub visible_digits: usize,
    pub probable_prime_bases: Vec<u64>,
}

impl SeedToWitnessConfig {
    pub fn default_for_seed(input_seed: u64) -> Self {
        Self {
            input_seed,
            max_steps: DEFAULT_MAX_STEPS,
            exact_seed_only: false,
            base: PRIMARY_BASE,
            outer: PRIMARY_OUTER,
            inner: PRIMARY_INNER,
            k_outer: PRIMARY_K.0,
            k_inner: PRIMARY_K.1,
            visible_digits: DEFAULT_VISIBLE_DIGITS,
            probable_prime_bases: PROBABLE_PRIME_BASES.to_vec(),
        }
    }

    pub fn with_visible_digits(mut self, visible_digits: usize) -> Self {
        self.visible_digits = visible_digits;
        self
    }

    pub fn with_exact_seed_only(mut self, exact_seed_only: bool) -> Self {
        self.exact_seed_only = exact_seed_only;
        self
    }

    pub fn with_max_steps(mut self, max_steps: u64) -> Self {
        self.max_steps = max_steps;
        self
    }

    pub fn with_lane(
        mut self,
        base: u32,
        outer: u32,
        inner: u32,
        k_outer: u32,
        k_inner: u32,
    ) -> Self {
        self.base = base;
        self.outer = outer;
        self.inner = inner;
        self.k_outer = k_outer;
        self.k_inner = k_inner;
        self
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SeedToWitnessResult {
    pub input_seed: u64,
    pub witness_seed: u64,
    pub steps_to_witness: u64,
    pub exact_seed_only: bool,
    pub max_steps: u64,
    pub scanned_seed_count: u64,
    pub residue_survivor_count: u64,
    pub residue_rejected_count: u64,
    pub probable_prime_tests: u64,
    pub elapsed_seconds: f64,
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub k_outer: u32,
    pub k_inner: u32,
    pub middle_length: usize,
    pub visible_digits: usize,
    pub residue_moduli_label: String,
    pub shift: String,
    pub gradient: String,
    pub affine_line: String,
    pub middle_digits: String,
    pub template_digits: String,
    pub decimal_value: String,
    pub decimal_digits: usize,
    pub compact_description: String,
    pub confirmation: String,
    pub is_mersenne: bool,
    pub mersenne_exponent: Option<u64>,
    pub mersenne_class: String,
    pub verification_snippets: Vec<VerificationSnippet>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VerificationSnippet {
    pub tool: String,
    pub snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessCertificate {
    pub schema_version: String,
    pub settings: ProofCarryingWitnessSettings,
    pub witness: ProofCarryingWitnessIdentity,
    pub affine_construction: AffineConstructionCertificate,
    pub residue_rows: Vec<ResidueCertificateRow>,
    pub rejection_examples: Vec<RejectionExampleRow>,
    #[serde(default)]
    pub search_replay: Option<SearchReplayCertificate>,
    pub confirmation: WitnessConfirmationCertificate,
    pub shape: WitnessShapeCertificate,
    pub verification_snippets: Vec<VerificationSnippet>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessSettings {
    pub input_seed: u64,
    pub max_steps: u64,
    pub exact_seed_only: bool,
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub k_outer: u32,
    pub k_inner: u32,
    pub visible_digits: usize,
    pub probable_prime_bases: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessIdentity {
    pub witness_seed: u64,
    pub steps_to_witness: u64,
    pub scanned_seed_count: u64,
    pub residue_survivor_count: u64,
    pub residue_rejected_count: u64,
    pub probable_prime_tests: u64,
    pub middle_width: usize,
    pub decimal_digits: usize,
    pub compact_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AffineConstructionCertificate {
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub k_outer: u32,
    pub k_inner: u32,
    pub middle_width: usize,
    pub shift: String,
    pub gradient: String,
    pub witness_seed: u64,
    pub decimal_value: String,
    pub template_digits: String,
    pub middle_digits: String,
    pub affine_line: String,
    pub affine_value_matches_decimal: bool,
    pub template_digits_match_result: bool,
    pub middle_digits_match_result: bool,
    pub visible_digit_count_matches_template: bool,
    pub decimal_digit_count_matches_value: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResidueCertificateRow {
    pub modulus: u32,
    pub coprime_to_base: bool,
    pub shift_mod: u32,
    pub gradient_mod: u32,
    pub seed_mod: u32,
    pub value_mod: u32,
    pub affine_residue_mod: u32,
    pub affine_residue_check: bool,
    pub survived: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RejectionExampleRow {
    pub seed: u64,
    pub offset_from_input_seed: u64,
    pub rejected_by_modulus: u32,
    pub shift_mod: u32,
    pub gradient_mod: u32,
    pub seed_mod: u32,
    pub value_mod: u32,
    pub affine_residue_mod: u32,
    pub affine_residue_check: bool,
    pub rejected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchReplayCertificate {
    pub search_order: String,
    pub input_seed: u64,
    pub witness_seed: u64,
    pub witness_offset: u64,
    pub scanned_seed_count: u64,
    pub residue_survivor_count: u64,
    pub residue_rejected_count: u64,
    pub probable_prime_tests: u64,
    pub complete_through_witness: bool,
    pub rows: Vec<SearchReplayRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchReplayRow {
    pub offset_from_input_seed: u64,
    pub seed: u64,
    pub status: SearchReplayRowStatus,
    pub rejected_by_modulus: Option<u32>,
    pub residue_survived: bool,
    pub probable_prime_tested: bool,
    pub probable_prime_result: bool,
    pub accepted_witness: bool,
    pub residue_rows: Vec<ResidueCertificateRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SearchReplayRowStatus {
    ResidueRejected,
    ResidueSurvivorProbablePrimeRejected,
    AcceptedProbablePrimeWitness,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WitnessConfirmationCertificate {
    pub method_label: String,
    pub probable_prime_bases: Vec<u64>,
    pub probable_prime_result: bool,
    pub primality_proof_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WitnessShapeCertificate {
    pub is_mersenne: bool,
    pub mersenne_exponent: Option<u64>,
    pub mersenne_class: String,
    pub exact_not_mersenne: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessVerificationReport {
    pub schema_version: String,
    pub ok: bool,
    pub witness_seed: u64,
    pub checked_residue_row_count: usize,
    pub failures: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ProofCarryingWitnessSpec {
    pub artifact_id: &'static str,
    pub role: &'static str,
    pub file_name: &'static str,
    pub lean_module_stem: &'static str,
    pub config: SeedToWitnessConfig,
}

#[derive(Debug, Clone)]
pub struct ProofCarryingWitnessPolicyMatrixSpec {
    pub artifact_id: &'static str,
    pub role: &'static str,
    pub seed_origin_policy: &'static str,
    pub lane_label: &'static str,
    pub file_name: &'static str,
    pub config: SeedToWitnessConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessManifest {
    pub schema_version: String,
    pub artifact_set_id: String,
    pub verifier_command: String,
    pub ci_gate: String,
    pub ci_status: String,
    pub artifacts: Vec<ProofCarryingWitnessManifestArtifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessManifestArtifact {
    pub artifact_id: String,
    pub role: String,
    pub path: String,
    pub certificate_schema_version: String,
    pub input_seed: u64,
    pub witness_seed: u64,
    pub visible_digits: usize,
    pub middle_width: usize,
    pub residue_row_count: usize,
    pub rejection_example_count: usize,
    pub search_replay_row_count: usize,
    pub confirmation_method: String,
    pub primality_proof_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProofCarryingWitnessLeanCatalogClaimStatus {
    ConstructionAndResidueOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessLeanCatalogManifest {
    pub schema_version: String,
    pub artifact_set_id: String,
    pub witness_manifest_path: String,
    pub generated_lean_dir: String,
    pub generator_command: String,
    pub drift_check_command: String,
    pub ci_gate: String,
    pub ci_status: String,
    pub claim_status: ProofCarryingWitnessLeanCatalogClaimStatus,
    pub artifacts: Vec<ProofCarryingWitnessLeanCatalogArtifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessLeanCatalogArtifact {
    pub artifact_id: String,
    pub role: String,
    pub certificate_path: String,
    pub generated_lean_path: String,
    pub generated_lean_module: String,
    pub theorem_names: ProofCarryingWitnessLeanTheoremNames,
    pub residue_theorem_names: Vec<ProofCarryingWitnessLeanResidueTheoremNames>,
    pub rejection_theorem_names: Vec<ProofCarryingWitnessLeanRejectionTheoremNames>,
    pub search_replay_theorem_names: Vec<ProofCarryingWitnessLeanSearchReplayTheoremNames>,
    pub theorem_wrapper: Option<ProofCarryingWitnessLeanWrapperLink>,
    pub primality_proof_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessLeanTheoremNames {
    pub certificate_schema_version: String,
    pub source_certificate_path: String,
    pub width: String,
    pub shift: String,
    pub gradient: String,
    pub witness_value: String,
    pub witness_value_eq_shift_add_gradient: String,
    pub residue_moduli_length: String,
    pub residue_moduli_nodup: String,
    pub residue_funnel_affine_checks: String,
    pub residue_funnel_survives: String,
    pub rejection_examples_reject: String,
    pub search_replay_seeds_length: String,
    pub search_replay_witness_seed: String,
    pub search_replay_rejections_reject: String,
    pub search_replay_survivors_survive: String,
    pub search_replay_scanned_seed_count: String,
    pub search_replay_residue_rejected_count: String,
    pub search_replay_residue_survivor_count: String,
    pub search_replay_certificate: String,
    pub search_replay_pre_witness_complete: String,
    pub search_replay_witness_survives: String,
    pub search_replay_sound: String,
    pub search_replay_survivor_list_exact: String,
    pub search_replay_partition_exact: String,
    pub search_replay_count_exact: String,
    pub search_replay_accounting_exact: String,
    pub search_replay_survivor_acceptance_exact: String,
    pub search_replay_accepted_survivor_exact: String,
    pub search_replay_pre_witness_survivors_non_accepted: String,
    pub search_replay_first_accepted_survivor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessLeanResidueTheoremNames {
    pub modulus: u32,
    pub residue_row: String,
    pub survives: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessLeanRejectionTheoremNames {
    pub seed: u64,
    pub modulus: u32,
    pub rejection: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessLeanSearchReplayTheoremNames {
    pub seed: u64,
    pub status: SearchReplayRowStatus,
    pub rejected_by_modulus: Option<u32>,
    pub rejection: Option<String>,
    pub survives_residue_funnel: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessLeanWrapperLink {
    pub lean_path: String,
    pub lean_module: String,
    pub theorem_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProofCarryingWitnessSearchPolicyAtlasClaimStatus {
    SearchReplayResidueOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessSearchPolicyAtlas {
    pub schema_version: String,
    pub artifact_set_id: String,
    pub witness_manifest_path: String,
    pub lean_catalog_manifest_path: String,
    pub generator_command: String,
    pub drift_check_command: String,
    pub ci_gate: String,
    pub ci_status: String,
    pub claim_status: ProofCarryingWitnessSearchPolicyAtlasClaimStatus,
    pub summary: ProofCarryingWitnessSearchPolicyAtlasSummary,
    pub coverage_rows: Vec<ProofCarryingWitnessSearchPolicyCoverageRow>,
    pub lane_rows: Vec<ProofCarryingWitnessSearchPolicyLaneRow>,
    pub rejection_modulus_rows: Vec<ProofCarryingWitnessSearchPolicyRejectionModulusRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessSearchPolicyAtlasSummary {
    pub artifact_count: usize,
    pub lane_count: usize,
    pub seed_origin_policy_count: usize,
    pub visible_digit_count: usize,
    pub total_scanned_seed_count: u64,
    pub total_residue_rejected_count: u64,
    pub total_residue_survivor_count: u64,
    pub max_first_accepted_distance: u64,
    pub max_non_accepted_residue_survivor_count: u64,
    pub all_replays_complete_through_witness: bool,
    pub all_have_first_accepted_survivor_theorem: bool,
    pub primality_proof_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessSearchPolicyCoverageRow {
    pub artifact_id: String,
    pub role: String,
    pub certificate_path: String,
    pub seed_origin_policy: String,
    pub lane_id: String,
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub k_outer: u32,
    pub k_inner: u32,
    pub visible_digits: usize,
    pub middle_width: usize,
    pub input_seed: u64,
    pub witness_seed: u64,
    pub first_accepted_distance: u64,
    pub scanned_seed_count: u64,
    pub residue_rejected_count: u64,
    pub residue_survivor_count: u64,
    pub non_accepted_residue_survivor_count: u64,
    pub accepted_residue_survivor_count: u64,
    pub probable_prime_tests: u64,
    pub complete_through_witness: bool,
    pub residue_moduli: Vec<u32>,
    pub rejection_modulus_sequence: Vec<u32>,
    pub rejection_modulus_counts: Vec<ProofCarryingWitnessSearchPolicyModulusCount>,
    pub rejection_example_modulus_counts: Vec<ProofCarryingWitnessSearchPolicyModulusCount>,
    pub rejection_geometry: String,
    pub proof_status: String,
    pub lean_links: ProofCarryingWitnessSearchPolicyLeanLinks,
    pub primality_proof_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessSearchPolicyLaneRow {
    pub lane_id: String,
    pub artifact_count: usize,
    pub visible_digits: Vec<usize>,
    pub seed_origin_policies: Vec<String>,
    pub max_first_accepted_distance: u64,
    pub total_scanned_seed_count: u64,
    pub total_residue_rejected_count: u64,
    pub total_residue_survivor_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessSearchPolicyRejectionModulusRow {
    pub modulus: u32,
    pub replay_rejection_count: u64,
    pub rejection_example_count: u64,
    pub artifact_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessSearchPolicyModulusCount {
    pub modulus: u32,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessSearchPolicyLeanLinks {
    pub generated_lean_module: String,
    pub search_replay_certificate: String,
    pub replay_accounting_exact: String,
    pub first_accepted_survivor: String,
    pub theorem_wrapper_first_accepted_survivor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProofCarryingWitnessPolicyMatrixClaimStatus {
    SearchReplayCertificateCandidatesOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProofCarryingWitnessPolicyMatrixLeanPromotionStatus {
    GeneratedLeanCanonical,
    GeneratedLeanPolicyMatrix,
    LeanCandidateSmallNativeDecide,
    AtlasOnlyLargeCandidate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessPolicyMatrixReport {
    pub schema_version: String,
    pub matrix_id: String,
    pub generator_command: String,
    pub claim_status: ProofCarryingWitnessPolicyMatrixClaimStatus,
    pub summary: ProofCarryingWitnessPolicyMatrixSummary,
    pub rows: Vec<ProofCarryingWitnessPolicyMatrixRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessPolicyMatrixSummary {
    pub row_count: usize,
    pub certificate_count: usize,
    pub lane_count: usize,
    pub seed_origin_policy_count: usize,
    pub visible_digit_count: usize,
    pub canonical_lean_promoted_count: usize,
    pub matrix_lean_promoted_count: usize,
    pub small_lean_candidate_count: usize,
    pub atlas_only_large_candidate_count: usize,
    pub total_scanned_seed_count: u64,
    pub total_residue_rejected_count: u64,
    pub total_residue_survivor_count: u64,
    pub max_first_accepted_distance: u64,
    pub all_rows_found_witness: bool,
    pub primality_proof_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessPolicyMatrixRow {
    pub artifact_id: String,
    pub role: String,
    pub seed_origin_policy: String,
    pub lane_label: String,
    pub lane_id: String,
    pub certificate_path: String,
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub k_outer: u32,
    pub k_inner: u32,
    pub visible_digits: usize,
    pub middle_width: usize,
    pub input_seed: u64,
    pub max_steps: u64,
    pub witness_seed: u64,
    pub first_accepted_distance: u64,
    pub scanned_seed_count: u64,
    pub residue_rejected_count: u64,
    pub residue_survivor_count: u64,
    pub non_accepted_residue_survivor_count: u64,
    pub accepted_residue_survivor_count: u64,
    pub probable_prime_tests: u64,
    pub complete_through_witness: bool,
    pub residue_moduli: Vec<u32>,
    pub rejection_modulus_counts: Vec<ProofCarryingWitnessSearchPolicyModulusCount>,
    pub rejection_geometry: String,
    pub lean_promotion_status: ProofCarryingWitnessPolicyMatrixLeanPromotionStatus,
    pub confirmation_method: String,
    pub primality_proof_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProofCarryingWitnessPolicyMatrixAtlasClaimStatus {
    SearchReplayResidueOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProofCarryingWitnessPolicyMatrixLeanReplayCoverage {
    LeanReplayCertified,
    NotLeanPromoted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProofCarryingWitnessPolicyMatrixNextReplayTargetStatus {
    Selected,
    NoneCurrentSmokeMatrixFullyCovered,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessPolicyMatrixAtlas {
    pub schema_version: String,
    pub matrix_id: String,
    pub source_matrix_schema_version: String,
    pub generator_command: String,
    pub drift_check_command: String,
    pub ci_gate: String,
    pub ci_status: String,
    pub claim_status: ProofCarryingWitnessPolicyMatrixAtlasClaimStatus,
    pub summary: ProofCarryingWitnessPolicyMatrixAtlasSummary,
    pub next_replay_target: ProofCarryingWitnessPolicyMatrixNextReplayTarget,
    pub coverage_rows: Vec<ProofCarryingWitnessPolicyMatrixAtlasCoverageRow>,
    pub promoted_large_replay_geometry_rows:
        Vec<ProofCarryingWitnessPolicyMatrixAtlasPromotedLargeReplayGeometryRow>,
    pub lane_rows: Vec<ProofCarryingWitnessPolicyMatrixAtlasLaneRow>,
    pub rejection_geometry_rows: Vec<ProofCarryingWitnessPolicyMatrixAtlasRejectionGeometryRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessPolicyMatrixAtlasSummary {
    pub row_count: usize,
    pub lane_count: usize,
    pub promoted_replay_certified_count: usize,
    pub unpromoted_replay_candidate_count: usize,
    pub atlas_only_large_candidate_count: usize,
    pub canonical_lean_promoted_count: usize,
    pub matrix_lean_promoted_count: usize,
    pub promoted_large_replay_geometry_count: usize,
    pub max_first_accepted_distance: u64,
    pub max_non_accepted_residue_survivor_count: u64,
    pub all_promoted_have_lean_replay_links: bool,
    pub primality_proof_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessPolicyMatrixAtlasCoverageRow {
    pub artifact_id: String,
    pub role: String,
    pub seed_origin_policy: String,
    pub lane_label: String,
    pub lane_id: String,
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub k_outer: u32,
    pub k_inner: u32,
    pub visible_digits: usize,
    pub middle_width: usize,
    pub witness_seed: u64,
    pub first_accepted_distance: u64,
    pub scanned_seed_count: u64,
    pub residue_rejected_count: u64,
    pub residue_survivor_count: u64,
    pub non_accepted_residue_survivor_count: u64,
    pub probable_prime_tests: u64,
    pub rejection_geometry: String,
    pub lean_promotion_status: ProofCarryingWitnessPolicyMatrixLeanPromotionStatus,
    pub lean_replay_coverage: ProofCarryingWitnessPolicyMatrixLeanReplayCoverage,
    pub lean_links: Option<ProofCarryingWitnessSearchPolicyLeanLinks>,
    pub primality_proof_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessPolicyMatrixAtlasPromotedLargeReplayGeometryRow {
    pub rank: usize,
    pub artifact_id: String,
    pub lane_label: String,
    pub lane_id: String,
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub k_outer: u32,
    pub k_inner: u32,
    pub visible_digits: usize,
    pub first_accepted_distance: u64,
    pub scanned_seed_count: u64,
    pub residue_rejected_count: u64,
    pub residue_survivor_count: u64,
    pub non_accepted_residue_survivor_count: u64,
    pub rejection_geometry: String,
    pub generated_lean_module: String,
    pub replay_accounting_exact: String,
    pub first_accepted_survivor: String,
    pub primality_proof_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessPolicyMatrixAtlasLaneRow {
    pub lane_label: String,
    pub lane_id: String,
    pub artifact_count: usize,
    pub promoted_replay_certified_count: usize,
    pub all_rows_lean_replay_certified: bool,
    pub visible_digits: Vec<usize>,
    pub seed_origin_policies: Vec<String>,
    pub rejection_geometries: Vec<String>,
    pub min_first_accepted_distance: u64,
    pub max_first_accepted_distance: u64,
    pub total_scanned_seed_count: u64,
    pub total_residue_rejected_count: u64,
    pub total_residue_survivor_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessPolicyMatrixAtlasRejectionGeometryRow {
    pub rejection_geometry: String,
    pub artifact_count: usize,
    pub artifact_ids: Vec<String>,
    pub promoted_replay_certified_count: usize,
    pub max_first_accepted_distance: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCarryingWitnessPolicyMatrixNextReplayTarget {
    pub status: ProofCarryingWitnessPolicyMatrixNextReplayTargetStatus,
    pub reason: String,
    pub artifact_id: Option<String>,
    pub lane_label: Option<String>,
    pub lane_id: Option<String>,
    pub visible_digits: Option<usize>,
    pub first_accepted_distance: Option<u64>,
    pub rejection_geometry: Option<String>,
}

#[derive(Debug, Clone)]
struct SearchHitStats {
    steps_to_witness: u64,
    scanned_seed_count: u64,
    residue_survivor_count: u64,
    probable_prime_tests: u64,
    elapsed_seconds: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeedToWitnessError {
    VisibleDigitsTooSmall {
        visible_digits: usize,
        fixed_digits: usize,
    },
    SeedRangeOverflow {
        input_seed: u64,
        max_steps: u64,
    },
    NoWitnessFound {
        input_seed: u64,
        max_steps: u64,
        exact_seed_only: bool,
        scanned_seed_count: u64,
        residue_survivor_count: u64,
    },
}

impl fmt::Display for SeedToWitnessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VisibleDigitsTooSmall {
                visible_digits,
                fixed_digits,
            } => write!(
                f,
                "visible digit target {visible_digits} is below fixed template width {fixed_digits}"
            ),
            Self::SeedRangeOverflow {
                input_seed,
                max_steps,
            } => write!(
                f,
                "seed search range overflows u64: start={input_seed}, max_steps={max_steps}"
            ),
            Self::NoWitnessFound {
                input_seed,
                max_steps,
                exact_seed_only,
                scanned_seed_count,
                residue_survivor_count,
            } => {
                if *exact_seed_only {
                    write!(
                        f,
                        "seed {input_seed} is not a witness in exact-seed-only mode"
                    )
                } else {
                    write!(
                        f,
                        "no witness found from seed {input_seed} within {max_steps} steps ({scanned_seed_count} scanned, {residue_survivor_count} residue survivors)"
                    )
                }
            }
        }
    }
}

impl Error for SeedToWitnessError {}

pub fn find_seed_to_witness(
    config: SeedToWitnessConfig,
) -> Result<SeedToWitnessResult, SeedToWitnessError> {
    let fixed_digits = fixed_template_digits((config.k_outer, config.k_inner));
    let middle_length = config.visible_digits.checked_sub(fixed_digits).ok_or(
        SeedToWitnessError::VisibleDigitsTooSmall {
            visible_digits: config.visible_digits,
            fixed_digits,
        },
    )?;
    let lane = build_big_affine_lane(
        config.base,
        config.outer,
        config.inner,
        middle_length,
        (config.k_outer, config.k_inner),
    );
    let scan_count = if config.exact_seed_only {
        1
    } else {
        config.max_steps
    };
    if scan_count > 0
        && config
            .input_seed
            .checked_add(scan_count.saturating_sub(1))
            .is_none()
    {
        return Err(SeedToWitnessError::SeedRangeOverflow {
            input_seed: config.input_seed,
            max_steps: config.max_steps,
        });
    }

    let moduli = residue_moduli(config.base);
    let started = Instant::now();
    let mut residue_survivors = 0u64;
    let mut probable_prime_tests = 0u64;

    for offset in 0..scan_count {
        let seed = config.input_seed + offset;
        if !residue_allows_seed(&lane, seed, &moduli) {
            continue;
        }
        residue_survivors += 1;
        probable_prime_tests += 1;
        let value = candidate_value(&lane, seed);
        if let Some(confirmation) = confirmation_label(&value, &config.probable_prime_bases) {
            let elapsed = started.elapsed().as_secs_f64().max(1e-12);
            return Ok(build_result(
                &config,
                &lane,
                seed,
                SearchHitStats {
                    steps_to_witness: offset,
                    scanned_seed_count: scan_count.min(offset + 1),
                    residue_survivor_count: residue_survivors,
                    probable_prime_tests,
                    elapsed_seconds: elapsed,
                },
                confirmation,
            ));
        }
    }

    Err(SeedToWitnessError::NoWitnessFound {
        input_seed: config.input_seed,
        max_steps: config.max_steps,
        exact_seed_only: config.exact_seed_only,
        scanned_seed_count: scan_count,
        residue_survivor_count: residue_survivors,
    })
}

pub fn render_seed_to_witness_transcript(result: &SeedToWitnessResult) -> String {
    let mut lines = Vec::new();
    lines.push("# Seed To Witness Transcript".to_string());
    lines.push(String::new());
    lines.push("## Construction".to_string());
    lines.push(format!("- input seed: `{}`", result.input_seed));
    lines.push(format!(
        "- search mode: `{}`",
        if result.exact_seed_only {
            "exact seed only"
        } else {
            "walk forward"
        }
    ));
    lines.push(format!(
        "- lane: base `{}`, pair `({}, {})`, k=`({}, {})`, M=`{}`",
        result.base,
        result.outer,
        result.inner,
        result.k_outer,
        result.k_inner,
        result.middle_length
    ));
    lines.push(format!("- visible digits: `{}`", result.visible_digits));
    lines.push(format!("- affine line: `{}`", result.affine_line));
    lines.push(String::new());
    lines.push("## Residue Funnel".to_string());
    lines.push(format!("- moduli: `{}`", result.residue_moduli_label));
    lines.push(format!(
        "- scanned: `{}` raw seeds -> `{}` residue survivors / probable-prime tests -> `1` witness",
        result.scanned_seed_count, result.residue_survivor_count
    ));
    lines.push(String::new());
    lines.push("## Witness".to_string());
    lines.push(format!("- witness seed: `{}`", result.witness_seed));
    lines.push(format!(
        "- steps from input seed: `{}`",
        result.steps_to_witness
    ));
    lines.push(format!("- middle digits: `{}`", result.middle_digits));
    lines.push(format!("- template: `{}`", result.template_digits));
    lines.push(format!("- decimal digits: `{}`", result.decimal_digits));
    lines.push(format!(
        "- compact description: `{}`",
        result.compact_description
    ));
    lines.push(format!("- Mersenne class: `{}`", result.mersenne_class));
    lines.push(String::new());
    lines.push("```text".to_string());
    lines.push(result.decimal_value.clone());
    lines.push("```".to_string());
    lines.push(String::new());
    lines.push("## Local Confirmation".to_string());
    lines.push(format!("- `{}`", result.confirmation));
    lines.push(
        "- Above `u64`, this repo says probable-prime witness unless a proof certificate is added."
            .to_string(),
    );
    lines.push(String::new());
    lines.push("## Copyable External Checks".to_string());
    for snippet in &result.verification_snippets {
        lines.push(format!("- {}: `{}`", snippet.tool, snippet.snippet));
    }
    lines.join("\n")
}

pub fn build_proof_carrying_witness_certificate(
    result: &SeedToWitnessResult,
    probable_prime_bases: &[u64],
) -> ProofCarryingWitnessCertificate {
    let lane = build_big_affine_lane(
        result.base,
        result.outer,
        result.inner,
        result.middle_length,
        (result.k_outer, result.k_inner),
    );
    let value = candidate_value(&lane, result.witness_seed);
    let decimal_value = value.to_str_radix(10);
    let computed_template_digits = template_digits(&lane, result.witness_seed);
    let computed_middle_digits = middle_digits(lane.base, lane.middle_length, result.witness_seed);
    let residue_rows = build_residue_certificate_rows(&lane, result.witness_seed, &value);
    let rejection_examples = build_rejection_examples(
        &lane,
        result.input_seed,
        result.witness_seed,
        result.max_steps,
        DEFAULT_REJECTION_EXAMPLE_COUNT,
    );
    let search_replay = build_search_replay_certificate(
        &lane,
        result.input_seed,
        result.witness_seed,
        result.scanned_seed_count,
        probable_prime_bases,
    );
    let probable_prime_result = confirmation_label(&value, probable_prime_bases).is_some();
    let exact_not_mersenne = !result.is_mersenne && result.mersenne_class == "not_mersenne";

    ProofCarryingWitnessCertificate {
        schema_version: PROOF_CARRYING_WITNESS_SCHEMA_VERSION.to_string(),
        settings: ProofCarryingWitnessSettings {
            input_seed: result.input_seed,
            max_steps: result.max_steps,
            exact_seed_only: result.exact_seed_only,
            base: result.base,
            outer: result.outer,
            inner: result.inner,
            k_outer: result.k_outer,
            k_inner: result.k_inner,
            visible_digits: result.visible_digits,
            probable_prime_bases: probable_prime_bases.to_vec(),
        },
        witness: ProofCarryingWitnessIdentity {
            witness_seed: result.witness_seed,
            steps_to_witness: result.steps_to_witness,
            scanned_seed_count: result.scanned_seed_count,
            residue_survivor_count: result.residue_survivor_count,
            residue_rejected_count: result.residue_rejected_count,
            probable_prime_tests: result.probable_prime_tests,
            middle_width: result.middle_length,
            decimal_digits: result.decimal_digits,
            compact_description: result.compact_description.clone(),
        },
        affine_construction: AffineConstructionCertificate {
            base: lane.base,
            outer: lane.outer,
            inner: lane.inner,
            k_outer: lane.k_outer,
            k_inner: lane.k_inner,
            middle_width: lane.middle_length,
            shift: lane.shift.to_str_radix(10),
            gradient: lane.gradient.to_str_radix(10),
            witness_seed: result.witness_seed,
            decimal_value: result.decimal_value.clone(),
            template_digits: result.template_digits.clone(),
            middle_digits: result.middle_digits.clone(),
            affine_line: result.affine_line.clone(),
            affine_value_matches_decimal: decimal_value == result.decimal_value,
            template_digits_match_result: computed_template_digits == result.template_digits,
            middle_digits_match_result: computed_middle_digits == result.middle_digits,
            visible_digit_count_matches_template: result.template_digits.chars().count()
                == result.visible_digits,
            decimal_digit_count_matches_value: result.decimal_value.len() == result.decimal_digits,
        },
        residue_rows,
        rejection_examples,
        search_replay: Some(search_replay),
        confirmation: WitnessConfirmationCertificate {
            method_label: result.confirmation.clone(),
            probable_prime_bases: probable_prime_bases.to_vec(),
            probable_prime_result,
            primality_proof_status: PROBABLE_PRIME_NOT_PROOF_CERTIFIED.to_string(),
        },
        shape: WitnessShapeCertificate {
            is_mersenne: result.is_mersenne,
            mersenne_exponent: result.mersenne_exponent,
            mersenne_class: result.mersenne_class.clone(),
            exact_not_mersenne,
        },
        verification_snippets: result.verification_snippets.clone(),
    }
}

pub fn canonical_proof_carrying_witness_specs() -> Vec<ProofCarryingWitnessSpec> {
    vec![
        ProofCarryingWitnessSpec {
            artifact_id: "seed60-canonical-128d",
            role: "canonical_seed60_128d",
            file_name: "seed60_proof_carrying_witness.json",
            lean_module_stem: "Seed60",
            config: SeedToWitnessConfig::default_for_seed(60),
        },
        ProofCarryingWitnessSpec {
            artifact_id: "teaching-seed0-38d",
            role: "teaching_seed0_38d",
            file_name: "teaching38_proof_carrying_witness.json",
            lean_module_stem: "Teaching38",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(38)
                .with_max_steps(100),
        },
        ProofCarryingWitnessSpec {
            artifact_id: "timestamp-policy-trial0-29d",
            role: "timestamp_policy_29d_trial0",
            file_name: "timestamp_policy_29d_trial0_proof_carrying_witness.json",
            lean_module_stem: "TimestampPolicy29Trial0",
            config: SeedToWitnessConfig::default_for_seed(1_777_651_200_000_000_000)
                .with_visible_digits(29)
                .with_max_steps(512),
        },
    ]
}

pub fn proof_carrying_witness_policy_matrix_smoke_specs(
) -> Vec<ProofCarryingWitnessPolicyMatrixSpec> {
    vec![
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-seed60-canonical-128d",
            role: "canonical_seed60_128d",
            seed_origin_policy: "canonical-fixed-seed",
            lane_label: "decimal-readable-k21",
            file_name: "matrix_seed60_canonical_128d.json",
            config: SeedToWitnessConfig::default_for_seed(60),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-teaching-seed0-38d",
            role: "teaching_seed0_38d",
            seed_origin_policy: "teaching-fixed-seed",
            lane_label: "decimal-readable-k21",
            file_name: "matrix_teaching_seed0_38d.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(38)
                .with_max_steps(100),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-timestamp-policy-29d-trial0",
            role: "timestamp_policy_29d_trial0",
            seed_origin_policy: "timestamp-policy",
            lane_label: "decimal-readable-k21",
            file_name: "matrix_timestamp_policy_29d_trial0.json",
            config: SeedToWitnessConfig::default_for_seed(1_777_651_200_000_000_000)
                .with_visible_digits(29)
                .with_max_steps(512),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-decimal-readable-22d-seed0",
            role: "matrix_decimal_readable_22d_seed0",
            seed_origin_policy: "fixed-zero-seed",
            lane_label: "decimal-readable-k21",
            file_name: "matrix_decimal_readable_22d_seed0.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(22)
                .with_max_steps(256),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-decimal-classic-22d-seed0",
            role: "matrix_decimal_classic_22d_seed0",
            seed_origin_policy: "fixed-zero-seed",
            lane_label: "decimal-classic-k00",
            file_name: "matrix_decimal_classic_22d_seed0.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(22)
                .with_max_steps(256)
                .with_lane(10, 3, 7, 0, 0),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-decimal-breathing-22d-seed0",
            role: "matrix_decimal_breathing_22d_seed0",
            seed_origin_policy: "fixed-zero-seed",
            lane_label: "decimal-breathing-k01",
            file_name: "matrix_decimal_breathing_22d_seed0.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(22)
                .with_max_steps(256)
                .with_lane(10, 3, 3, 0, 1),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-base6-compact-18d-seed0",
            role: "matrix_base6_compact_18d_seed0",
            seed_origin_policy: "fixed-zero-seed",
            lane_label: "base6-compact-k00",
            file_name: "matrix_base6_compact_18d_seed0.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(18)
                .with_max_steps(256)
                .with_lane(6, 1, 5, 0, 0),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-base12-compact-18d-seed0",
            role: "matrix_base12_compact_18d_seed0",
            seed_origin_policy: "fixed-zero-seed",
            lane_label: "base12-compact-k00",
            file_name: "matrix_base12_compact_18d_seed0.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(18)
                .with_max_steps(256)
                .with_lane(12, 1, 11, 0, 0),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-base30-wheel-18d-seed0",
            role: "matrix_base30_wheel_18d_seed0",
            seed_origin_policy: "fixed-zero-seed",
            lane_label: "base30-wheel-k00",
            file_name: "matrix_base30_wheel_18d_seed0.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(18)
                .with_max_steps(256)
                .with_lane(30, 11, 7, 0, 0),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-decimal-readable-64d-seed0",
            role: "matrix_decimal_readable_64d_seed0",
            seed_origin_policy: "fixed-zero-seed",
            lane_label: "decimal-readable-k21",
            file_name: "matrix_decimal_readable_64d_seed0.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(64)
                .with_max_steps(4096),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-decimal-classic-64d-seed0",
            role: "matrix_decimal_classic_64d_seed0",
            seed_origin_policy: "fixed-zero-seed",
            lane_label: "decimal-classic-k00",
            file_name: "matrix_decimal_classic_64d_seed0.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(64)
                .with_max_steps(4096)
                .with_lane(10, 3, 7, 0, 0),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-decimal-breathing-64d-seed0",
            role: "matrix_decimal_breathing_64d_seed0",
            seed_origin_policy: "fixed-zero-seed",
            lane_label: "decimal-breathing-k01",
            file_name: "matrix_decimal_breathing_64d_seed0.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(64)
                .with_max_steps(4096)
                .with_lane(10, 3, 3, 0, 1),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-base6-compact-64d-seed0",
            role: "matrix_base6_compact_64d_seed0",
            seed_origin_policy: "fixed-zero-seed",
            lane_label: "base6-compact-k00",
            file_name: "matrix_base6_compact_64d_seed0.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(64)
                .with_max_steps(4096)
                .with_lane(6, 1, 5, 0, 0),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-base12-compact-64d-seed0",
            role: "matrix_base12_compact_64d_seed0",
            seed_origin_policy: "fixed-zero-seed",
            lane_label: "base12-compact-k00",
            file_name: "matrix_base12_compact_64d_seed0.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(64)
                .with_max_steps(4096)
                .with_lane(12, 1, 11, 0, 0),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-base30-wheel-64d-seed0",
            role: "matrix_base30_wheel_64d_seed0",
            seed_origin_policy: "fixed-zero-seed",
            lane_label: "base30-wheel-k00",
            file_name: "matrix_base30_wheel_64d_seed0.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(64)
                .with_max_steps(4096)
                .with_lane(30, 11, 7, 0, 0),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-decimal-readable-96d-seed0",
            role: "matrix_decimal_readable_96d_seed0",
            seed_origin_policy: "fixed-zero-seed",
            lane_label: "decimal-readable-k21",
            file_name: "matrix_decimal_readable_96d_seed0.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(96)
                .with_max_steps(8192),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-decimal-classic-96d-seed0",
            role: "matrix_decimal_classic_96d_seed0",
            seed_origin_policy: "fixed-zero-seed",
            lane_label: "decimal-classic-k00",
            file_name: "matrix_decimal_classic_96d_seed0.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(96)
                .with_max_steps(8192)
                .with_lane(10, 3, 7, 0, 0),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-decimal-breathing-96d-seed0",
            role: "matrix_decimal_breathing_96d_seed0",
            seed_origin_policy: "fixed-zero-seed",
            lane_label: "decimal-breathing-k01",
            file_name: "matrix_decimal_breathing_96d_seed0.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(96)
                .with_max_steps(8192)
                .with_lane(10, 3, 3, 0, 1),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-base6-compact-96d-seed0",
            role: "matrix_base6_compact_96d_seed0",
            seed_origin_policy: "fixed-zero-seed",
            lane_label: "base6-compact-k00",
            file_name: "matrix_base6_compact_96d_seed0.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(96)
                .with_max_steps(8192)
                .with_lane(6, 1, 5, 0, 0),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-base12-compact-96d-seed0",
            role: "matrix_base12_compact_96d_seed0",
            seed_origin_policy: "fixed-zero-seed",
            lane_label: "base12-compact-k00",
            file_name: "matrix_base12_compact_96d_seed0.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(96)
                .with_max_steps(8192)
                .with_lane(12, 1, 11, 0, 0),
        },
        ProofCarryingWitnessPolicyMatrixSpec {
            artifact_id: "matrix-base30-wheel-96d-seed0",
            role: "matrix_base30_wheel_96d_seed0",
            seed_origin_policy: "fixed-zero-seed",
            lane_label: "base30-wheel-k00",
            file_name: "matrix_base30_wheel_96d_seed0.json",
            config: SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(96)
                .with_max_steps(8192)
                .with_lane(30, 11, 7, 0, 0),
        },
    ]
}

pub fn proof_carrying_witness_policy_matrix_promoted_specs(
) -> Vec<ProofCarryingWitnessPolicyMatrixSpec> {
    proof_carrying_witness_policy_matrix_smoke_specs()
        .into_iter()
        .filter(|spec| proof_carrying_witness_policy_matrix_lean_module_stem(spec).is_some())
        .collect()
}

pub fn proof_carrying_witness_lean_output_path(spec: &ProofCarryingWitnessSpec) -> String {
    format!(
        "lean-proofs/PrimeArithmetic/Generated/Witness/{}.lean",
        spec.lean_module_stem
    )
}

pub fn proof_carrying_witness_lean_module_name(spec: &ProofCarryingWitnessSpec) -> String {
    format!(
        "PrimeArithmetic.Generated.Witness.{}",
        spec.lean_module_stem
    )
}

pub fn proof_carrying_witness_policy_matrix_certificate_path(
    spec: &ProofCarryingWitnessPolicyMatrixSpec,
) -> String {
    format!("docs/witness/policy_matrix/{}", spec.file_name)
}

pub fn proof_carrying_witness_policy_matrix_lean_output_path(
    spec: &ProofCarryingWitnessPolicyMatrixSpec,
) -> Option<String> {
    proof_carrying_witness_policy_matrix_lean_module_stem(spec)
        .map(|stem| format!("lean-proofs/PrimeArithmetic/Generated/Witness/{stem}.lean"))
}

pub fn proof_carrying_witness_policy_matrix_lean_module_name(
    spec: &ProofCarryingWitnessPolicyMatrixSpec,
) -> Option<String> {
    proof_carrying_witness_policy_matrix_lean_module_stem(spec)
        .map(|stem| format!("PrimeArithmetic.Generated.Witness.{stem}"))
}

fn proof_carrying_witness_policy_matrix_lean_module_stem(
    spec: &ProofCarryingWitnessPolicyMatrixSpec,
) -> Option<&'static str> {
    match spec.artifact_id {
        "matrix-decimal-readable-22d-seed0" => Some("MatrixDecimalReadable22"),
        "matrix-decimal-classic-22d-seed0" => Some("MatrixDecimalClassic22"),
        "matrix-decimal-breathing-22d-seed0" => Some("MatrixDecimalBreathing22"),
        "matrix-decimal-readable-64d-seed0" => Some("MatrixDecimalReadable64"),
        "matrix-decimal-readable-96d-seed0" => Some("MatrixDecimalReadable96"),
        "matrix-decimal-classic-64d-seed0" => Some("MatrixDecimalClassic64"),
        "matrix-decimal-breathing-64d-seed0" => Some("MatrixDecimalBreathing64"),
        "matrix-decimal-breathing-96d-seed0" => Some("MatrixDecimalBreathing96"),
        "matrix-decimal-classic-96d-seed0" => Some("MatrixDecimalClassic96"),
        "matrix-base30-wheel-64d-seed0" => Some("MatrixBase30Wheel64"),
        "matrix-base30-wheel-96d-seed0" => Some("MatrixBase30Wheel96"),
        "matrix-base6-compact-18d-seed0" => Some("MatrixBase6Compact18"),
        "matrix-base12-compact-18d-seed0" => Some("MatrixBase12Compact18"),
        "matrix-base6-compact-64d-seed0" => Some("MatrixBase6Compact64"),
        "matrix-base6-compact-96d-seed0" => Some("MatrixBase6Compact96"),
        "matrix-base12-compact-64d-seed0" => Some("MatrixBase12Compact64"),
        "matrix-base12-compact-96d-seed0" => Some("MatrixBase12Compact96"),
        "matrix-base30-wheel-18d-seed0" => Some("MatrixBase30Wheel18"),
        _ => None,
    }
}

pub fn build_proof_carrying_witness_certificate_for_config(
    config: SeedToWitnessConfig,
) -> Result<ProofCarryingWitnessCertificate, SeedToWitnessError> {
    let probable_prime_bases = config.probable_prime_bases.clone();
    let result = find_seed_to_witness(config)?;
    Ok(build_proof_carrying_witness_certificate(
        &result,
        &probable_prime_bases,
    ))
}

pub fn build_proof_carrying_witness_manifest(
    artifacts: Vec<ProofCarryingWitnessManifestArtifact>,
) -> ProofCarryingWitnessManifest {
    ProofCarryingWitnessManifest {
        schema_version: PROOF_CARRYING_WITNESS_MANIFEST_SCHEMA_VERSION.to_string(),
        artifact_set_id: PROOF_CARRYING_WITNESS_ARTIFACT_SET_ID.to_string(),
        verifier_command: "cargo run --bin verify-proof-carrying-witness -- <certificate.json>"
            .to_string(),
        ci_gate: "scripts/ci_witness_certificate.sh".to_string(),
        ci_status: "maintained-ci-gate".to_string(),
        artifacts,
    }
}

pub fn proof_carrying_witness_manifest_artifact(
    spec: &ProofCarryingWitnessSpec,
    certificate: &ProofCarryingWitnessCertificate,
) -> ProofCarryingWitnessManifestArtifact {
    ProofCarryingWitnessManifestArtifact {
        artifact_id: spec.artifact_id.to_string(),
        role: spec.role.to_string(),
        path: format!("docs/witness/{}", spec.file_name),
        certificate_schema_version: certificate.schema_version.clone(),
        input_seed: certificate.settings.input_seed,
        witness_seed: certificate.witness.witness_seed,
        visible_digits: certificate.settings.visible_digits,
        middle_width: certificate.witness.middle_width,
        residue_row_count: certificate.residue_rows.len(),
        rejection_example_count: certificate.rejection_examples.len(),
        search_replay_row_count: certificate
            .search_replay
            .as_ref()
            .map(|replay| replay.rows.len())
            .unwrap_or(0),
        confirmation_method: certificate.confirmation.method_label.clone(),
        primality_proof_status: certificate.confirmation.primality_proof_status.clone(),
    }
}

pub fn build_proof_carrying_witness_lean_catalog_manifest(
    artifacts: Vec<ProofCarryingWitnessLeanCatalogArtifact>,
) -> ProofCarryingWitnessLeanCatalogManifest {
    ProofCarryingWitnessLeanCatalogManifest {
        schema_version: PROOF_CARRYING_WITNESS_LEAN_CATALOG_SCHEMA_VERSION.to_string(),
        artifact_set_id: PROOF_CARRYING_WITNESS_ARTIFACT_SET_ID.to_string(),
        witness_manifest_path: "docs/witness/witness_certificate_manifest.json".to_string(),
        generated_lean_dir: "lean-proofs/PrimeArithmetic/Generated/Witness".to_string(),
        generator_command:
            "cargo run --bin export_proof_carrying_witness_lean_certificate -- --catalog --certificate-dir docs/witness --out-dir lean-proofs/PrimeArithmetic/Generated/Witness --manifest-out docs/witness/witness_lean_catalog_manifest.json"
                .to_string(),
        drift_check_command: "scripts/lean_proof_carrying_witness_certificate.sh verify"
            .to_string(),
        ci_gate: "scripts/ci_witness_certificate.sh".to_string(),
        ci_status: "maintained-ci-gate".to_string(),
        claim_status: ProofCarryingWitnessLeanCatalogClaimStatus::ConstructionAndResidueOnly,
        artifacts,
    }
}

pub fn build_proof_carrying_witness_policy_matrix_lean_catalog_manifest(
    artifacts: Vec<ProofCarryingWitnessLeanCatalogArtifact>,
) -> ProofCarryingWitnessLeanCatalogManifest {
    ProofCarryingWitnessLeanCatalogManifest {
        schema_version: PROOF_CARRYING_WITNESS_LEAN_CATALOG_SCHEMA_VERSION.to_string(),
        artifact_set_id: PROOF_CARRYING_WITNESS_POLICY_MATRIX_ID.to_string(),
        witness_manifest_path: "docs/witness/policy_matrix".to_string(),
        generated_lean_dir: "lean-proofs/PrimeArithmetic/Generated/Witness".to_string(),
        generator_command:
            "cargo run --bin export_proof_carrying_witness_lean_certificate -- --policy-matrix-catalog --certificate-dir docs/witness/policy_matrix --out-dir lean-proofs/PrimeArithmetic/Generated/Witness --manifest-out docs/witness/witness_policy_matrix_lean_catalog_manifest.json"
                .to_string(),
        drift_check_command: "scripts/lean_proof_carrying_witness_certificate.sh verify"
            .to_string(),
        ci_gate: "scripts/ci_witness_certificate.sh".to_string(),
        ci_status: "maintained-ci-gate".to_string(),
        claim_status: ProofCarryingWitnessLeanCatalogClaimStatus::ConstructionAndResidueOnly,
        artifacts,
    }
}

pub fn proof_carrying_witness_lean_catalog_artifact(
    spec: &ProofCarryingWitnessSpec,
    certificate: &ProofCarryingWitnessCertificate,
) -> ProofCarryingWitnessLeanCatalogArtifact {
    let module = proof_carrying_witness_lean_module_name(spec);
    ProofCarryingWitnessLeanCatalogArtifact {
        artifact_id: spec.artifact_id.to_string(),
        role: spec.role.to_string(),
        certificate_path: format!("docs/witness/{}", spec.file_name),
        generated_lean_path: proof_carrying_witness_lean_output_path(spec),
        generated_lean_module: module.clone(),
        theorem_names: proof_carrying_witness_lean_theorem_names(&module),
        residue_theorem_names: certificate
            .residue_rows
            .iter()
            .map(|row| ProofCarryingWitnessLeanResidueTheoremNames {
                modulus: row.modulus,
                residue_row: qualified_lean_name(
                    &module,
                    &format!("residue_row_mod{}", row.modulus),
                ),
                survives: qualified_lean_name(&module, &format!("survives_mod{}", row.modulus)),
            })
            .collect(),
        rejection_theorem_names: certificate
            .rejection_examples
            .iter()
            .map(|row| ProofCarryingWitnessLeanRejectionTheoremNames {
                seed: row.seed,
                modulus: row.rejected_by_modulus,
                rejection: qualified_lean_name(
                    &module,
                    &format!("rejection_seed{}_mod{}", row.seed, row.rejected_by_modulus),
                ),
            })
            .collect(),
        search_replay_theorem_names: proof_carrying_witness_lean_search_replay_theorem_names(
            &module,
            certificate,
        ),
        theorem_wrapper: proof_carrying_witness_lean_wrapper_link(spec),
        primality_proof_status: certificate.confirmation.primality_proof_status.clone(),
    }
}

pub fn proof_carrying_witness_policy_matrix_lean_catalog_artifact(
    spec: &ProofCarryingWitnessPolicyMatrixSpec,
    certificate: &ProofCarryingWitnessCertificate,
) -> Option<ProofCarryingWitnessLeanCatalogArtifact> {
    let module = proof_carrying_witness_policy_matrix_lean_module_name(spec)?;
    let generated_lean_path = proof_carrying_witness_policy_matrix_lean_output_path(spec)?;
    Some(ProofCarryingWitnessLeanCatalogArtifact {
        artifact_id: spec.artifact_id.to_string(),
        role: spec.role.to_string(),
        certificate_path: proof_carrying_witness_policy_matrix_certificate_path(spec),
        generated_lean_path,
        generated_lean_module: module.clone(),
        theorem_names: proof_carrying_witness_lean_theorem_names(&module),
        residue_theorem_names: certificate
            .residue_rows
            .iter()
            .map(|row| ProofCarryingWitnessLeanResidueTheoremNames {
                modulus: row.modulus,
                residue_row: qualified_lean_name(
                    &module,
                    &format!("residue_row_mod{}", row.modulus),
                ),
                survives: qualified_lean_name(&module, &format!("survives_mod{}", row.modulus)),
            })
            .collect(),
        rejection_theorem_names: certificate
            .rejection_examples
            .iter()
            .map(|row| ProofCarryingWitnessLeanRejectionTheoremNames {
                seed: row.seed,
                modulus: row.rejected_by_modulus,
                rejection: qualified_lean_name(
                    &module,
                    &format!("rejection_seed{}_mod{}", row.seed, row.rejected_by_modulus),
                ),
            })
            .collect(),
        search_replay_theorem_names: proof_carrying_witness_lean_search_replay_theorem_names(
            &module,
            certificate,
        ),
        theorem_wrapper: None,
        primality_proof_status: certificate.confirmation.primality_proof_status.clone(),
    })
}

pub fn render_proof_carrying_witness_lean_catalog_checks(
    manifest: &ProofCarryingWitnessLeanCatalogManifest,
) -> String {
    render_proof_carrying_witness_lean_catalog_checks_for_artifacts(&manifest.artifacts)
}

pub struct ProofCarryingWitnessLeanCatalogCheckShard {
    pub module_name: String,
    pub file_name: String,
    pub contents: String,
}

pub struct ProofCarryingWitnessLeanCatalogCheckBundle {
    pub umbrella_contents: String,
    pub shards: Vec<ProofCarryingWitnessLeanCatalogCheckShard>,
}

pub fn render_proof_carrying_witness_lean_catalog_check_shards(
    manifest: &ProofCarryingWitnessLeanCatalogManifest,
    module_prefix: &str,
    umbrella_stem: &str,
    shard_size: usize,
) -> Result<ProofCarryingWitnessLeanCatalogCheckBundle, String> {
    use std::fmt::Write as _;

    if shard_size == 0 {
        return Err("shard size must be positive".to_string());
    }
    if umbrella_stem.is_empty() {
        return Err("umbrella module stem must not be empty".to_string());
    }

    let mut shards = Vec::new();
    for (index, chunk) in manifest.artifacts.chunks(shard_size).enumerate() {
        let file_stem = format!("{umbrella_stem}Shard{:02}", index + 1);
        let module_name = format!("{module_prefix}.{file_stem}");
        shards.push(ProofCarryingWitnessLeanCatalogCheckShard {
            module_name,
            file_name: format!("{file_stem}.lean"),
            contents: render_proof_carrying_witness_lean_catalog_checks_for_artifacts(chunk),
        });
    }

    let mut umbrella_contents = String::new();
    for shard in &shards {
        let _ = writeln!(umbrella_contents, "import {}", shard.module_name);
    }
    umbrella_contents.push_str("\n/-!\n");
    umbrella_contents.push_str(
        "Lean declaration check umbrella generated from the proof-carrying witness Lean catalog manifest.\n",
    );
    umbrella_contents
        .push_str("Each imported shard should elaborate if its theorem links resolve.\n");
    umbrella_contents.push_str("-/\n");

    Ok(ProofCarryingWitnessLeanCatalogCheckBundle {
        umbrella_contents,
        shards,
    })
}

fn render_proof_carrying_witness_lean_catalog_checks_for_artifacts(
    artifacts: &[ProofCarryingWitnessLeanCatalogArtifact],
) -> String {
    use std::{collections::BTreeSet, fmt::Write as _};

    let mut imports = BTreeSet::new();
    for artifact in artifacts {
        imports.insert(artifact.generated_lean_module.clone());
        if let Some(wrapper) = &artifact.theorem_wrapper {
            imports.insert(wrapper.lean_module.clone());
        }
    }

    let mut out = String::new();
    for module in imports {
        let _ = writeln!(out, "import {module}");
    }
    out.push_str("\n/-!\n");
    out.push_str(
        "Lean declaration checks generated from the proof-carrying witness Lean catalog manifest.\n",
    );
    out.push_str("This file should elaborate if every machine-readable theorem link resolves.\n");
    out.push_str("-/\n\n");

    for artifact in artifacts {
        let _ = writeln!(out, "-- {}", artifact.artifact_id);
        for theorem in proof_carrying_witness_lean_catalog_theorem_links(artifact) {
            let _ = writeln!(out, "example : True := by");
            let _ = writeln!(out, "  have _ := @{theorem}");
            let _ = writeln!(out, "  trivial");
        }
        out.push('\n');
    }

    while out.ends_with("\n\n") {
        out.pop();
    }
    out
}

fn proof_carrying_witness_lean_catalog_theorem_links(
    artifact: &ProofCarryingWitnessLeanCatalogArtifact,
) -> Vec<&str> {
    let names = &artifact.theorem_names;
    let mut links = vec![
        names.certificate_schema_version.as_str(),
        names.source_certificate_path.as_str(),
        names.width.as_str(),
        names.shift.as_str(),
        names.gradient.as_str(),
        names.witness_value.as_str(),
        names.witness_value_eq_shift_add_gradient.as_str(),
        names.residue_moduli_length.as_str(),
        names.residue_moduli_nodup.as_str(),
        names.residue_funnel_affine_checks.as_str(),
        names.residue_funnel_survives.as_str(),
        names.rejection_examples_reject.as_str(),
        names.search_replay_seeds_length.as_str(),
        names.search_replay_witness_seed.as_str(),
        names.search_replay_rejections_reject.as_str(),
        names.search_replay_survivors_survive.as_str(),
        names.search_replay_scanned_seed_count.as_str(),
        names.search_replay_residue_rejected_count.as_str(),
        names.search_replay_residue_survivor_count.as_str(),
        names.search_replay_certificate.as_str(),
        names.search_replay_pre_witness_complete.as_str(),
        names.search_replay_witness_survives.as_str(),
        names.search_replay_sound.as_str(),
        names.search_replay_survivor_list_exact.as_str(),
        names.search_replay_partition_exact.as_str(),
        names.search_replay_count_exact.as_str(),
        names.search_replay_accounting_exact.as_str(),
        names.search_replay_survivor_acceptance_exact.as_str(),
        names.search_replay_accepted_survivor_exact.as_str(),
        names
            .search_replay_pre_witness_survivors_non_accepted
            .as_str(),
        names.search_replay_first_accepted_survivor.as_str(),
    ];
    for row in &artifact.residue_theorem_names {
        links.push(row.residue_row.as_str());
        links.push(row.survives.as_str());
    }
    for row in &artifact.rejection_theorem_names {
        links.push(row.rejection.as_str());
    }
    for row in &artifact.search_replay_theorem_names {
        if let Some(rejection) = &row.rejection {
            links.push(rejection.as_str());
        }
        if let Some(survives) = &row.survives_residue_funnel {
            links.push(survives.as_str());
        }
    }
    if let Some(wrapper) = &artifact.theorem_wrapper {
        links.extend(wrapper.theorem_names.iter().map(String::as_str));
    }
    links
}

fn proof_carrying_witness_lean_theorem_names(module: &str) -> ProofCarryingWitnessLeanTheoremNames {
    ProofCarryingWitnessLeanTheoremNames {
        certificate_schema_version: qualified_lean_name(module, "certificateSchemaVersion_value"),
        source_certificate_path: qualified_lean_name(module, "sourceCertificatePath_value"),
        width: qualified_lean_name(module, "width_value"),
        shift: qualified_lean_name(module, "shift_value"),
        gradient: qualified_lean_name(module, "gradient_value"),
        witness_value: qualified_lean_name(module, "witness_value"),
        witness_value_eq_shift_add_gradient: qualified_lean_name(
            module,
            "witness_value_eq_shift_add_gradient",
        ),
        residue_moduli_length: qualified_lean_name(module, "residueModuli_length"),
        residue_moduli_nodup: qualified_lean_name(module, "residueModuli_nodup"),
        residue_funnel_affine_checks: qualified_lean_name(module, "residueFunnelAffineChecks"),
        residue_funnel_survives: qualified_lean_name(module, "residueFunnelSurvives"),
        rejection_examples_reject: qualified_lean_name(module, "rejectionExamplesReject"),
        search_replay_seeds_length: qualified_lean_name(module, "searchReplaySeeds_length"),
        search_replay_witness_seed: qualified_lean_name(module, "searchReplayWitnessSeed"),
        search_replay_rejections_reject: qualified_lean_name(
            module,
            "searchReplayResidueRejectionsReject",
        ),
        search_replay_survivors_survive: qualified_lean_name(
            module,
            "searchReplayResidueSurvivorsSurvive",
        ),
        search_replay_scanned_seed_count: qualified_lean_name(
            module,
            "searchReplayScannedSeedCount_value",
        ),
        search_replay_residue_rejected_count: qualified_lean_name(
            module,
            "searchReplayResidueRejectedCount_value",
        ),
        search_replay_residue_survivor_count: qualified_lean_name(
            module,
            "searchReplayResidueSurvivorCount_value",
        ),
        search_replay_certificate: qualified_lean_name(module, "searchReplayCertificate"),
        search_replay_pre_witness_complete: qualified_lean_name(
            module,
            "searchReplayPreWitnessComplete",
        ),
        search_replay_witness_survives: qualified_lean_name(module, "searchReplayWitnessSurvives"),
        search_replay_sound: qualified_lean_name(module, "searchReplaySound"),
        search_replay_survivor_list_exact: qualified_lean_name(
            module,
            "searchReplaySurvivorListExact",
        ),
        search_replay_partition_exact: qualified_lean_name(module, "searchReplayPartitionExact"),
        search_replay_count_exact: qualified_lean_name(module, "searchReplayCountExact"),
        search_replay_accounting_exact: qualified_lean_name(module, "searchReplayAccountingExact"),
        search_replay_survivor_acceptance_exact: qualified_lean_name(
            module,
            "searchReplaySurvivorAcceptanceExact",
        ),
        search_replay_accepted_survivor_exact: qualified_lean_name(
            module,
            "searchReplayAcceptedSurvivorExact",
        ),
        search_replay_pre_witness_survivors_non_accepted: qualified_lean_name(
            module,
            "searchReplayPreWitnessSurvivorsNonAccepted",
        ),
        search_replay_first_accepted_survivor: qualified_lean_name(
            module,
            "searchReplayFirstAcceptedSurvivor",
        ),
    }
}

fn proof_carrying_witness_lean_search_replay_theorem_names(
    module: &str,
    certificate: &ProofCarryingWitnessCertificate,
) -> Vec<ProofCarryingWitnessLeanSearchReplayTheoremNames> {
    if !proof_carrying_witness_emit_individual_search_replay_theorems(certificate) {
        return Vec::new();
    }

    certificate
        .search_replay
        .as_ref()
        .into_iter()
        .flat_map(|replay| replay.rows.iter())
        .map(|row| {
            let rejection = row.rejected_by_modulus.map(|modulus| {
                qualified_lean_name(
                    module,
                    &format!("search_replay_seed{}_rejected_mod{}", row.seed, modulus),
                )
            });
            let survives_residue_funnel = row.residue_survived.then(|| {
                qualified_lean_name(module, &format!("search_replay_seed{}_survives", row.seed))
            });
            ProofCarryingWitnessLeanSearchReplayTheoremNames {
                seed: row.seed,
                status: row.status.clone(),
                rejected_by_modulus: row.rejected_by_modulus,
                rejection,
                survives_residue_funnel,
            }
        })
        .collect()
}

fn proof_carrying_witness_emit_individual_search_replay_theorems(
    certificate: &ProofCarryingWitnessCertificate,
) -> bool {
    certificate.settings.visible_digits <= 38
        || certificate
            .search_replay
            .as_ref()
            .is_some_and(|replay| replay.rows.len() <= 1)
}

fn proof_carrying_witness_lean_wrapper_link(
    spec: &ProofCarryingWitnessSpec,
) -> Option<ProofCarryingWitnessLeanWrapperLink> {
    if spec.lean_module_stem != "Teaching38" {
        return None;
    }

    let module = "PrimeArithmetic.Witness.TeachingSeedCertificate";
    let theorem_names = [
        "teaching38_width",
        "teaching38_shift",
        "teaching38_gradient",
        "teaching38_value",
        "teaching38_value_eq_shift_add_gradient",
        "teaching38_residue_moduli_nodup",
        "teaching38_residue_funnel_affine_checks",
        "teaching38_residue_funnel_survives",
        "teaching38_rejection_examples_reject",
        "teaching38_search_replay_seeds_length",
        "teaching38_search_replay_witness_seed",
        "teaching38_search_replay_rejections_reject",
        "teaching38_search_replay_survivors_survive",
        "teaching38_pre_witness_replay_complete",
        "teaching38_search_replay_witness_survives",
        "teaching38_search_replay_sound",
        "teaching38_search_replay_survivor_list_exact",
        "teaching38_search_replay_first_accepted_survivor",
    ]
    .into_iter()
    .map(|name| qualified_lean_name(module, name))
    .collect();

    Some(ProofCarryingWitnessLeanWrapperLink {
        lean_path: "lean-proofs/PrimeArithmetic/Witness/TeachingSeedCertificate.lean".to_string(),
        lean_module: module.to_string(),
        theorem_names,
    })
}

pub fn proof_carrying_witness_search_policy_coverage_row(
    spec: &ProofCarryingWitnessSpec,
    certificate: &ProofCarryingWitnessCertificate,
) -> ProofCarryingWitnessSearchPolicyCoverageRow {
    let replay = certificate.search_replay.as_ref();
    let module = proof_carrying_witness_lean_module_name(spec);
    let theorem_names = proof_carrying_witness_lean_theorem_names(&module);
    let rejection_modulus_sequence = replay
        .into_iter()
        .flat_map(|replay| replay.rows.iter().filter_map(|row| row.rejected_by_modulus))
        .collect::<Vec<_>>();
    let rejection_modulus_counts =
        proof_carrying_witness_modulus_counts(rejection_modulus_sequence.iter().copied());
    let rejection_example_modulus_counts = proof_carrying_witness_modulus_counts(
        certificate
            .rejection_examples
            .iter()
            .map(|row| row.rejected_by_modulus),
    );
    let non_accepted_residue_survivor_count = replay
        .map(|replay| {
            replay
                .rows
                .iter()
                .filter(|row| row.residue_survived && !row.accepted_witness)
                .count() as u64
        })
        .unwrap_or(0);
    let accepted_residue_survivor_count = replay
        .map(|replay| {
            replay
                .rows
                .iter()
                .filter(|row| row.accepted_witness)
                .count() as u64
        })
        .unwrap_or(0);
    let complete_through_witness = replay
        .map(|replay| replay.complete_through_witness)
        .unwrap_or(false);
    let proof_status = if complete_through_witness {
        "lean-search-replay-certified"
    } else {
        "missing-search-replay"
    };
    let theorem_wrapper_first_accepted_survivor = proof_carrying_witness_lean_wrapper_link(spec)
        .and_then(|wrapper| {
            wrapper
                .theorem_names
                .into_iter()
                .find(|name| name.ends_with("_search_replay_first_accepted_survivor"))
        });

    ProofCarryingWitnessSearchPolicyCoverageRow {
        artifact_id: spec.artifact_id.to_string(),
        role: spec.role.to_string(),
        certificate_path: format!("docs/witness/{}", spec.file_name),
        seed_origin_policy: proof_carrying_witness_seed_origin_policy(spec).to_string(),
        lane_id: proof_carrying_witness_lane_id(&certificate.settings),
        base: certificate.settings.base,
        outer: certificate.settings.outer,
        inner: certificate.settings.inner,
        k_outer: certificate.settings.k_outer,
        k_inner: certificate.settings.k_inner,
        visible_digits: certificate.settings.visible_digits,
        middle_width: certificate.witness.middle_width,
        input_seed: certificate.settings.input_seed,
        witness_seed: certificate.witness.witness_seed,
        first_accepted_distance: replay
            .map(|replay| replay.witness_offset)
            .unwrap_or(certificate.witness.steps_to_witness),
        scanned_seed_count: replay
            .map(|replay| replay.scanned_seed_count)
            .unwrap_or(certificate.witness.scanned_seed_count),
        residue_rejected_count: replay
            .map(|replay| replay.residue_rejected_count)
            .unwrap_or(certificate.witness.residue_rejected_count),
        residue_survivor_count: replay
            .map(|replay| replay.residue_survivor_count)
            .unwrap_or(certificate.witness.residue_survivor_count),
        non_accepted_residue_survivor_count,
        accepted_residue_survivor_count,
        probable_prime_tests: replay
            .map(|replay| replay.probable_prime_tests)
            .unwrap_or(certificate.witness.probable_prime_tests),
        complete_through_witness,
        residue_moduli: certificate
            .residue_rows
            .iter()
            .map(|row| row.modulus)
            .collect(),
        rejection_geometry: proof_carrying_witness_rejection_geometry(&rejection_modulus_counts),
        rejection_modulus_sequence,
        rejection_modulus_counts,
        rejection_example_modulus_counts,
        proof_status: proof_status.to_string(),
        lean_links: ProofCarryingWitnessSearchPolicyLeanLinks {
            generated_lean_module: module,
            search_replay_certificate: theorem_names.search_replay_certificate,
            replay_accounting_exact: theorem_names.search_replay_accounting_exact,
            first_accepted_survivor: theorem_names.search_replay_first_accepted_survivor,
            theorem_wrapper_first_accepted_survivor,
        },
        primality_proof_status: certificate.confirmation.primality_proof_status.clone(),
    }
}

pub fn build_proof_carrying_witness_search_policy_atlas(
    mut coverage_rows: Vec<ProofCarryingWitnessSearchPolicyCoverageRow>,
) -> ProofCarryingWitnessSearchPolicyAtlas {
    coverage_rows.sort_by(|left, right| left.artifact_id.cmp(&right.artifact_id));
    let lane_rows = proof_carrying_witness_search_policy_lane_rows(&coverage_rows);
    let rejection_modulus_rows =
        proof_carrying_witness_search_policy_rejection_modulus_rows(&coverage_rows);
    let lane_count = lane_rows.len();
    let seed_origin_policy_count = coverage_rows
        .iter()
        .map(|row| row.seed_origin_policy.clone())
        .collect::<BTreeSet<_>>()
        .len();
    let visible_digit_count = coverage_rows
        .iter()
        .map(|row| row.visible_digits)
        .collect::<BTreeSet<_>>()
        .len();
    let primality_statuses = coverage_rows
        .iter()
        .map(|row| row.primality_proof_status.clone())
        .collect::<BTreeSet<_>>();
    let primality_proof_status = if primality_statuses.len() == 1 {
        primality_statuses
            .iter()
            .next()
            .cloned()
            .unwrap_or_else(|| "none".to_string())
    } else {
        "mixed".to_string()
    };

    ProofCarryingWitnessSearchPolicyAtlas {
        schema_version: PROOF_CARRYING_WITNESS_SEARCH_POLICY_ATLAS_SCHEMA_VERSION.to_string(),
        artifact_set_id: PROOF_CARRYING_WITNESS_ARTIFACT_SET_ID.to_string(),
        witness_manifest_path: "docs/witness/witness_certificate_manifest.json".to_string(),
        lean_catalog_manifest_path: "docs/witness/witness_lean_catalog_manifest.json".to_string(),
        generator_command:
            "cargo run --bin export_proof_carrying_witness_search_policy_atlas -- --certificate-dir docs/witness --out-dir docs/witness"
                .to_string(),
        drift_check_command: "scripts/proof_carrying_witness.sh verify".to_string(),
        ci_gate: "scripts/ci_witness_certificate.sh".to_string(),
        ci_status: "maintained-ci-gate".to_string(),
        claim_status: ProofCarryingWitnessSearchPolicyAtlasClaimStatus::SearchReplayResidueOnly,
        summary: ProofCarryingWitnessSearchPolicyAtlasSummary {
            artifact_count: coverage_rows.len(),
            lane_count,
            seed_origin_policy_count,
            visible_digit_count,
            total_scanned_seed_count: coverage_rows
                .iter()
                .map(|row| row.scanned_seed_count)
                .sum(),
            total_residue_rejected_count: coverage_rows
                .iter()
                .map(|row| row.residue_rejected_count)
                .sum(),
            total_residue_survivor_count: coverage_rows
                .iter()
                .map(|row| row.residue_survivor_count)
                .sum(),
            max_first_accepted_distance: coverage_rows
                .iter()
                .map(|row| row.first_accepted_distance)
                .max()
                .unwrap_or(0),
            max_non_accepted_residue_survivor_count: coverage_rows
                .iter()
                .map(|row| row.non_accepted_residue_survivor_count)
                .max()
                .unwrap_or(0),
            all_replays_complete_through_witness: coverage_rows
                .iter()
                .all(|row| row.complete_through_witness),
            all_have_first_accepted_survivor_theorem: coverage_rows.iter().all(|row| {
                !row.lean_links.first_accepted_survivor.is_empty()
                    && row.proof_status == "lean-search-replay-certified"
            }),
            primality_proof_status,
        },
        coverage_rows,
        lane_rows,
        rejection_modulus_rows,
    }
}

pub fn render_proof_carrying_witness_search_policy_atlas_markdown(
    atlas: &ProofCarryingWitnessSearchPolicyAtlas,
) -> String {
    use std::fmt::Write as _;

    let mut out = String::new();
    let _ = writeln!(out, "# Proof-Carrying Witness Search-Policy Atlas\n");
    let _ = writeln!(
        out,
        "Schema: `{}`. Claim status: `search-replay-residue-only`.\n",
        atlas.schema_version
    );
    out.push_str(
        "This atlas summarizes deterministic residue-replay coverage for the maintained proof-carrying witness bundle. It is a search-policy and residue-funnel artifact, not a primality proof.\n\n",
    );
    let summary = &atlas.summary;
    let _ = writeln!(out, "## Summary\n");
    let _ = writeln!(out, "- Artifacts: {}", summary.artifact_count);
    let _ = writeln!(out, "- Lanes: {}", summary.lane_count);
    let _ = writeln!(
        out,
        "- Seed-origin policies: {}",
        summary.seed_origin_policy_count
    );
    let _ = writeln!(
        out,
        "- Total scanned/rejected/survivor counts: `{}/{}/{}`",
        summary.total_scanned_seed_count,
        summary.total_residue_rejected_count,
        summary.total_residue_survivor_count
    );
    let _ = writeln!(
        out,
        "- Max first-accepted distance: {}",
        summary.max_first_accepted_distance
    );
    let _ = writeln!(
        out,
        "- First-accepted theorem coverage: {}",
        summary.all_have_first_accepted_survivor_theorem
    );
    let _ = writeln!(
        out,
        "- Primality proof status: `{}`\n",
        summary.primality_proof_status
    );

    out.push_str("## Coverage Rows\n\n");
    out.push_str("| Artifact | Policy | Digits | Input seed | Witness seed | Distance | Replay rejected/survivors | Geometry | Lean first-accepted theorem |\n");
    out.push_str("|---|---|---:|---:|---:|---:|---:|---|---|\n");
    for row in &atlas.coverage_rows {
        let _ = writeln!(
            out,
            "| `{}` | `{}` | {} | {} | {} | {} | {}/{} | `{}` | `{}` |",
            row.artifact_id,
            row.seed_origin_policy,
            row.visible_digits,
            row.input_seed,
            row.witness_seed,
            row.first_accepted_distance,
            row.residue_rejected_count,
            row.residue_survivor_count,
            row.rejection_geometry,
            row.lean_links.first_accepted_survivor
        );
    }

    out.push_str("\n## Lane Rows\n\n");
    out.push_str(
        "| Lane | Artifacts | Digits | Policies | Max distance | Scanned/rejected/survivors |\n",
    );
    out.push_str("|---|---:|---|---|---:|---:|\n");
    for row in &atlas.lane_rows {
        let _ = writeln!(
            out,
            "| `{}` | {} | `{}` | `{}` | {} | {}/{}/{} |",
            row.lane_id,
            row.artifact_count,
            row.visible_digits
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
            row.seed_origin_policies.join(","),
            row.max_first_accepted_distance,
            row.total_scanned_seed_count,
            row.total_residue_rejected_count,
            row.total_residue_survivor_count
        );
    }

    out.push_str("\n## Rejection Moduli\n\n");
    out.push_str("| Modulus | Replay rejections | Rejection examples | Artifacts |\n");
    out.push_str("|---:|---:|---:|---:|\n");
    for row in &atlas.rejection_modulus_rows {
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} |",
            row.modulus,
            row.replay_rejection_count,
            row.rejection_example_count,
            row.artifact_count
        );
    }

    out
}

fn proof_carrying_witness_seed_origin_policy(spec: &ProofCarryingWitnessSpec) -> &'static str {
    if spec.role.starts_with("timestamp") {
        "timestamp-policy"
    } else if spec.role.starts_with("teaching") {
        "teaching-fixed-seed"
    } else if spec.role.starts_with("canonical") {
        "canonical-fixed-seed"
    } else {
        "fixed-seed"
    }
}

fn proof_carrying_witness_lane_id(settings: &ProofCarryingWitnessSettings) -> String {
    format!(
        "base{}_outer{}_inner{}_k{}_{}",
        settings.base, settings.outer, settings.inner, settings.k_outer, settings.k_inner
    )
}

fn proof_carrying_witness_modulus_counts(
    moduli: impl IntoIterator<Item = u32>,
) -> Vec<ProofCarryingWitnessSearchPolicyModulusCount> {
    let mut counts = BTreeMap::<u32, u64>::new();
    for modulus in moduli {
        *counts.entry(modulus).or_default() += 1;
    }
    counts
        .into_iter()
        .map(|(modulus, count)| ProofCarryingWitnessSearchPolicyModulusCount { modulus, count })
        .collect()
}

fn proof_carrying_witness_rejection_geometry(
    counts: &[ProofCarryingWitnessSearchPolicyModulusCount],
) -> String {
    if counts.is_empty() {
        return "none".to_string();
    }
    counts
        .iter()
        .map(|row| format!("mod{}x{}", row.modulus, row.count))
        .collect::<Vec<_>>()
        .join("_")
}

fn proof_carrying_witness_search_policy_lane_rows(
    coverage_rows: &[ProofCarryingWitnessSearchPolicyCoverageRow],
) -> Vec<ProofCarryingWitnessSearchPolicyLaneRow> {
    let mut lanes = BTreeMap::<String, Vec<&ProofCarryingWitnessSearchPolicyCoverageRow>>::new();
    for row in coverage_rows {
        lanes.entry(row.lane_id.clone()).or_default().push(row);
    }

    lanes
        .into_iter()
        .map(|(lane_id, rows)| {
            let visible_digits = rows
                .iter()
                .map(|row| row.visible_digits)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            let seed_origin_policies = rows
                .iter()
                .map(|row| row.seed_origin_policy.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            ProofCarryingWitnessSearchPolicyLaneRow {
                lane_id,
                artifact_count: rows.len(),
                visible_digits,
                seed_origin_policies,
                max_first_accepted_distance: rows
                    .iter()
                    .map(|row| row.first_accepted_distance)
                    .max()
                    .unwrap_or(0),
                total_scanned_seed_count: rows.iter().map(|row| row.scanned_seed_count).sum(),
                total_residue_rejected_count: rows
                    .iter()
                    .map(|row| row.residue_rejected_count)
                    .sum(),
                total_residue_survivor_count: rows
                    .iter()
                    .map(|row| row.residue_survivor_count)
                    .sum(),
            }
        })
        .collect()
}

fn proof_carrying_witness_search_policy_rejection_modulus_rows(
    coverage_rows: &[ProofCarryingWitnessSearchPolicyCoverageRow],
) -> Vec<ProofCarryingWitnessSearchPolicyRejectionModulusRow> {
    let mut replay_counts = BTreeMap::<u32, u64>::new();
    let mut example_counts = BTreeMap::<u32, u64>::new();
    let mut artifact_sets = BTreeMap::<u32, BTreeSet<String>>::new();

    for row in coverage_rows {
        for count in &row.rejection_modulus_counts {
            *replay_counts.entry(count.modulus).or_default() += count.count;
            artifact_sets
                .entry(count.modulus)
                .or_default()
                .insert(row.artifact_id.clone());
        }
        for count in &row.rejection_example_modulus_counts {
            *example_counts.entry(count.modulus).or_default() += count.count;
            artifact_sets
                .entry(count.modulus)
                .or_default()
                .insert(row.artifact_id.clone());
        }
    }

    artifact_sets
        .into_iter()
        .map(
            |(modulus, artifacts)| ProofCarryingWitnessSearchPolicyRejectionModulusRow {
                modulus,
                replay_rejection_count: replay_counts.get(&modulus).copied().unwrap_or(0),
                rejection_example_count: example_counts.get(&modulus).copied().unwrap_or(0),
                artifact_count: artifacts.len(),
            },
        )
        .collect()
}

pub fn proof_carrying_witness_policy_matrix_row(
    spec: &ProofCarryingWitnessPolicyMatrixSpec,
    certificate: &ProofCarryingWitnessCertificate,
    certificate_path: impl Into<String>,
) -> ProofCarryingWitnessPolicyMatrixRow {
    let replay = certificate.search_replay.as_ref();
    let rejection_modulus_counts = proof_carrying_witness_modulus_counts(
        replay
            .into_iter()
            .flat_map(|replay| replay.rows.iter().filter_map(|row| row.rejected_by_modulus)),
    );
    let non_accepted_residue_survivor_count = replay
        .map(|replay| {
            replay
                .rows
                .iter()
                .filter(|row| row.residue_survived && !row.accepted_witness)
                .count() as u64
        })
        .unwrap_or(0);
    let accepted_residue_survivor_count = replay
        .map(|replay| {
            replay
                .rows
                .iter()
                .filter(|row| row.accepted_witness)
                .count() as u64
        })
        .unwrap_or(0);
    let complete_through_witness = replay
        .map(|replay| replay.complete_through_witness)
        .unwrap_or(false);

    ProofCarryingWitnessPolicyMatrixRow {
        artifact_id: spec.artifact_id.to_string(),
        role: spec.role.to_string(),
        seed_origin_policy: spec.seed_origin_policy.to_string(),
        lane_label: spec.lane_label.to_string(),
        lane_id: proof_carrying_witness_lane_id(&certificate.settings),
        certificate_path: certificate_path.into(),
        base: certificate.settings.base,
        outer: certificate.settings.outer,
        inner: certificate.settings.inner,
        k_outer: certificate.settings.k_outer,
        k_inner: certificate.settings.k_inner,
        visible_digits: certificate.settings.visible_digits,
        middle_width: certificate.witness.middle_width,
        input_seed: certificate.settings.input_seed,
        max_steps: certificate.settings.max_steps,
        witness_seed: certificate.witness.witness_seed,
        first_accepted_distance: replay
            .map(|replay| replay.witness_offset)
            .unwrap_or(certificate.witness.steps_to_witness),
        scanned_seed_count: replay
            .map(|replay| replay.scanned_seed_count)
            .unwrap_or(certificate.witness.scanned_seed_count),
        residue_rejected_count: replay
            .map(|replay| replay.residue_rejected_count)
            .unwrap_or(certificate.witness.residue_rejected_count),
        residue_survivor_count: replay
            .map(|replay| replay.residue_survivor_count)
            .unwrap_or(certificate.witness.residue_survivor_count),
        non_accepted_residue_survivor_count,
        accepted_residue_survivor_count,
        probable_prime_tests: replay
            .map(|replay| replay.probable_prime_tests)
            .unwrap_or(certificate.witness.probable_prime_tests),
        complete_through_witness,
        residue_moduli: certificate
            .residue_rows
            .iter()
            .map(|row| row.modulus)
            .collect(),
        rejection_geometry: proof_carrying_witness_rejection_geometry(&rejection_modulus_counts),
        rejection_modulus_counts,
        lean_promotion_status: proof_carrying_witness_policy_matrix_lean_status(spec, certificate),
        confirmation_method: certificate.confirmation.method_label.clone(),
        primality_proof_status: certificate.confirmation.primality_proof_status.clone(),
    }
}

pub fn build_proof_carrying_witness_policy_matrix_report(
    mut rows: Vec<ProofCarryingWitnessPolicyMatrixRow>,
) -> ProofCarryingWitnessPolicyMatrixReport {
    rows.sort_by(|left, right| left.artifact_id.cmp(&right.artifact_id));
    let lane_count = rows
        .iter()
        .map(|row| row.lane_id.clone())
        .collect::<BTreeSet<_>>()
        .len();
    let seed_origin_policy_count = rows
        .iter()
        .map(|row| row.seed_origin_policy.clone())
        .collect::<BTreeSet<_>>()
        .len();
    let visible_digit_count = rows
        .iter()
        .map(|row| row.visible_digits)
        .collect::<BTreeSet<_>>()
        .len();
    let primality_statuses = rows
        .iter()
        .map(|row| row.primality_proof_status.clone())
        .collect::<BTreeSet<_>>();
    let primality_proof_status = if primality_statuses.len() == 1 {
        primality_statuses
            .iter()
            .next()
            .cloned()
            .unwrap_or_else(|| "none".to_string())
    } else {
        "mixed".to_string()
    };

    ProofCarryingWitnessPolicyMatrixReport {
        schema_version: PROOF_CARRYING_WITNESS_POLICY_MATRIX_SCHEMA_VERSION.to_string(),
        matrix_id: PROOF_CARRYING_WITNESS_POLICY_MATRIX_ID.to_string(),
        generator_command:
            "cargo run --bin export_proof_carrying_witness_policy_matrix -- --out-dir <path>"
                .to_string(),
        claim_status:
            ProofCarryingWitnessPolicyMatrixClaimStatus::SearchReplayCertificateCandidatesOnly,
        summary: ProofCarryingWitnessPolicyMatrixSummary {
            row_count: rows.len(),
            certificate_count: rows.len(),
            lane_count,
            seed_origin_policy_count,
            visible_digit_count,
            canonical_lean_promoted_count: rows
                .iter()
                .filter(|row| {
                    row.lean_promotion_status
                        == ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanCanonical
                })
                .count(),
            matrix_lean_promoted_count: rows
                .iter()
                .filter(|row| {
                    row.lean_promotion_status
                        == ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanPolicyMatrix
                })
                .count(),
            small_lean_candidate_count: rows
                .iter()
                .filter(|row| {
                    row.lean_promotion_status
                        == ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::LeanCandidateSmallNativeDecide
                })
                .count(),
            atlas_only_large_candidate_count: rows
                .iter()
                .filter(|row| {
                    row.lean_promotion_status
                        == ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::AtlasOnlyLargeCandidate
                })
                .count(),
            total_scanned_seed_count: rows.iter().map(|row| row.scanned_seed_count).sum(),
            total_residue_rejected_count: rows.iter().map(|row| row.residue_rejected_count).sum(),
            total_residue_survivor_count: rows.iter().map(|row| row.residue_survivor_count).sum(),
            max_first_accepted_distance: rows
                .iter()
                .map(|row| row.first_accepted_distance)
                .max()
                .unwrap_or(0),
            all_rows_found_witness: rows
                .iter()
                .all(|row| row.accepted_residue_survivor_count == 1),
            primality_proof_status,
        },
        rows,
    }
}

pub fn build_proof_carrying_witness_policy_matrix_atlas(
    report: &ProofCarryingWitnessPolicyMatrixReport,
) -> ProofCarryingWitnessPolicyMatrixAtlas {
    let mut coverage_rows = report
        .rows
        .iter()
        .map(proof_carrying_witness_policy_matrix_atlas_coverage_row)
        .collect::<Vec<_>>();
    coverage_rows.sort_by(|left, right| left.artifact_id.cmp(&right.artifact_id));

    let lane_rows = proof_carrying_witness_policy_matrix_atlas_lane_rows(&coverage_rows);
    let rejection_geometry_rows =
        proof_carrying_witness_policy_matrix_atlas_rejection_geometry_rows(&coverage_rows);
    let promoted_large_replay_geometry_rows =
        proof_carrying_witness_policy_matrix_atlas_promoted_large_replay_geometry_rows(
            &coverage_rows,
        );
    let next_replay_target =
        proof_carrying_witness_policy_matrix_next_replay_target(&coverage_rows);
    let promoted_replay_certified_count = coverage_rows
        .iter()
        .filter(|row| {
            row.lean_replay_coverage
                == ProofCarryingWitnessPolicyMatrixLeanReplayCoverage::LeanReplayCertified
        })
        .count();
    let primality_statuses = coverage_rows
        .iter()
        .map(|row| row.primality_proof_status.clone())
        .collect::<BTreeSet<_>>();
    let primality_proof_status = if primality_statuses.len() == 1 {
        primality_statuses
            .iter()
            .next()
            .cloned()
            .unwrap_or_else(|| "none".to_string())
    } else {
        "mixed".to_string()
    };

    ProofCarryingWitnessPolicyMatrixAtlas {
        schema_version: PROOF_CARRYING_WITNESS_POLICY_MATRIX_ATLAS_SCHEMA_VERSION.to_string(),
        matrix_id: report.matrix_id.clone(),
        source_matrix_schema_version: report.schema_version.clone(),
        generator_command:
            "cargo run --bin export_proof_carrying_witness_policy_matrix -- --out-dir <path>"
                .to_string(),
        drift_check_command: "scripts/proof_carrying_witness.sh verify".to_string(),
        ci_gate: "scripts/ci_witness_certificate.sh".to_string(),
        ci_status: "maintained-ci-gate".to_string(),
        claim_status: ProofCarryingWitnessPolicyMatrixAtlasClaimStatus::SearchReplayResidueOnly,
        summary: ProofCarryingWitnessPolicyMatrixAtlasSummary {
            row_count: coverage_rows.len(),
            lane_count: lane_rows.len(),
            promoted_replay_certified_count,
            unpromoted_replay_candidate_count: coverage_rows.len() - promoted_replay_certified_count,
            atlas_only_large_candidate_count: coverage_rows
                .iter()
                .filter(|row| {
                    row.lean_promotion_status
                        == ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::AtlasOnlyLargeCandidate
                })
                .count(),
            canonical_lean_promoted_count: coverage_rows
                .iter()
                .filter(|row| {
                    row.lean_promotion_status
                        == ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanCanonical
                })
                .count(),
            matrix_lean_promoted_count: coverage_rows
                .iter()
                .filter(|row| {
                    row.lean_promotion_status
                        == ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanPolicyMatrix
                })
                .count(),
            promoted_large_replay_geometry_count: promoted_large_replay_geometry_rows.len(),
            max_first_accepted_distance: coverage_rows
                .iter()
                .map(|row| row.first_accepted_distance)
                .max()
                .unwrap_or(0),
            max_non_accepted_residue_survivor_count: coverage_rows
                .iter()
                .map(|row| row.non_accepted_residue_survivor_count)
                .max()
                .unwrap_or(0),
            all_promoted_have_lean_replay_links: coverage_rows.iter().all(|row| {
                if row.lean_replay_coverage
                    == ProofCarryingWitnessPolicyMatrixLeanReplayCoverage::LeanReplayCertified
                {
                    row.lean_links.as_ref().is_some_and(|links| {
                        !links.search_replay_certificate.is_empty()
                            && !links.replay_accounting_exact.is_empty()
                            && !links.first_accepted_survivor.is_empty()
                    })
                } else {
                    true
                }
            }),
            primality_proof_status,
        },
        next_replay_target,
        coverage_rows,
        promoted_large_replay_geometry_rows,
        lane_rows,
        rejection_geometry_rows,
    }
}

pub fn render_proof_carrying_witness_policy_matrix_atlas_markdown(
    atlas: &ProofCarryingWitnessPolicyMatrixAtlas,
) -> String {
    use std::fmt::Write as _;

    let mut out = String::new();
    let _ = writeln!(out, "# Proof-Carrying Witness Policy-Matrix Atlas\n");
    let _ = writeln!(
        out,
        "Schema: `{}`. Matrix: `{}`. Claim status: `search-replay-residue-only`.\n",
        atlas.schema_version, atlas.matrix_id
    );
    out.push_str(
        "This atlas summarizes deterministic policy-matrix replay coverage by lane, first-accepted distance, rejection geometry, and Lean theorem-link coverage. It is a search-policy and residue-replay artifact, not a primality proof and not a prime-density claim.\n\n",
    );

    let summary = &atlas.summary;
    out.push_str("## Summary\n\n");
    let _ = writeln!(out, "- Rows: `{}`", summary.row_count);
    let _ = writeln!(out, "- Lanes: `{}`", summary.lane_count);
    let _ = writeln!(
        out,
        "- Lean replay coverage: promoted `{}`, unpromoted `{}`, atlas-only large `{}`",
        summary.promoted_replay_certified_count,
        summary.unpromoted_replay_candidate_count,
        summary.atlas_only_large_candidate_count
    );
    let _ = writeln!(
        out,
        "- Promotion split: canonical `{}`, matrix `{}`",
        summary.canonical_lean_promoted_count, summary.matrix_lean_promoted_count
    );
    let _ = writeln!(
        out,
        "- Promoted large replay geometry rows: `{}`",
        summary.promoted_large_replay_geometry_count
    );
    let _ = writeln!(
        out,
        "- Max first-accepted distance: `{}`",
        summary.max_first_accepted_distance
    );
    let _ = writeln!(
        out,
        "- All promoted theorem links present: `{}`",
        summary.all_promoted_have_lean_replay_links
    );
    let _ = writeln!(
        out,
        "- Primality proof status: `{}`\n",
        summary.primality_proof_status
    );

    out.push_str("## Next Replay Target\n\n");
    let target = &atlas.next_replay_target;
    let _ = writeln!(
        out,
        "- Status: `{}`",
        proof_carrying_witness_policy_matrix_next_replay_target_status_label(&target.status)
    );
    let _ = writeln!(out, "- Reason: {}\n", target.reason);
    if let Some(artifact_id) = &target.artifact_id {
        let _ = writeln!(out, "| Artifact | Lane | Digits | Distance | Geometry |");
        out.push_str("|---|---|---:|---:|---|\n");
        let _ = writeln!(
            out,
            "| `{}` | `{}` | {} | {} | `{}` |\n",
            artifact_id,
            target.lane_label.as_deref().unwrap_or("unknown"),
            target.visible_digits.unwrap_or(0),
            target.first_accepted_distance.unwrap_or(0),
            target.rejection_geometry.as_deref().unwrap_or("unknown")
        );
    }

    out.push_str("## Promoted Large Replay Geometry\n\n");
    out.push_str("These rows compare generated-Lean large policy-matrix witnesses by replay geometry only; they do not rank primality evidence.\n\n");
    out.push_str("| Rank | Artifact | Lane | Digits | Distance | Replay rejected/survivors | Non-accepted survivors | Geometry | Accounting theorem |\n");
    out.push_str("|---:|---|---|---:|---:|---:|---:|---|---|\n");
    for row in &atlas.promoted_large_replay_geometry_rows {
        let _ = writeln!(
            out,
            "| {} | `{}` | `{}` | {} | {} | {}/{} | {} | `{}` | `{}` |",
            row.rank,
            row.artifact_id,
            row.lane_label,
            row.visible_digits,
            row.first_accepted_distance,
            row.residue_rejected_count,
            row.residue_survivor_count,
            row.non_accepted_residue_survivor_count,
            row.rejection_geometry,
            row.replay_accounting_exact
        );
    }
    out.push('\n');

    out.push_str("## Coverage Rows\n\n");
    out.push_str("| Artifact | Lane | Digits | Distance | Replay rejected/survivors | Geometry | Lean replay | First-accepted theorem |\n");
    out.push_str("|---|---|---:|---:|---:|---|---|---|\n");
    for row in &atlas.coverage_rows {
        let theorem = row
            .lean_links
            .as_ref()
            .map(|links| links.first_accepted_survivor.as_str())
            .unwrap_or("");
        let _ = writeln!(
            out,
            "| `{}` | `{}` | {} | {} | {}/{} | `{}` | `{}` | `{}` |",
            row.artifact_id,
            row.lane_label,
            row.visible_digits,
            row.first_accepted_distance,
            row.residue_rejected_count,
            row.residue_survivor_count,
            row.rejection_geometry,
            proof_carrying_witness_policy_matrix_lean_replay_coverage_label(
                &row.lean_replay_coverage
            ),
            theorem
        );
    }

    out.push_str("\n## Lane Rows\n\n");
    out.push_str(
        "| Lane | Artifacts | Promoted | Digits | Policies | Distance range | Geometries |\n",
    );
    out.push_str("|---|---:|---:|---|---|---:|---|\n");
    for row in &atlas.lane_rows {
        let _ = writeln!(
            out,
            "| `{}` | {} | {} | `{}` | `{}` | {}..{} | `{}` |",
            row.lane_label,
            row.artifact_count,
            row.promoted_replay_certified_count,
            row.visible_digits
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
            row.seed_origin_policies.join(","),
            row.min_first_accepted_distance,
            row.max_first_accepted_distance,
            row.rejection_geometries.join(",")
        );
    }

    out.push_str("\n## Rejection Geometry Rows\n\n");
    out.push_str("| Geometry | Artifacts | Promoted | Max distance |\n");
    out.push_str("|---|---:|---:|---:|\n");
    for row in &atlas.rejection_geometry_rows {
        let _ = writeln!(
            out,
            "| `{}` | {} | {} | {} |",
            row.rejection_geometry,
            row.artifact_count,
            row.promoted_replay_certified_count,
            row.max_first_accepted_distance
        );
    }

    out
}

pub fn render_proof_carrying_witness_policy_matrix_markdown(
    report: &ProofCarryingWitnessPolicyMatrixReport,
) -> String {
    use std::fmt::Write as _;

    let mut out = String::new();
    let _ = writeln!(out, "# Proof-Carrying Witness Policy Matrix\n");
    let _ = writeln!(
        out,
        "Schema: `{}`. Matrix: `{}`. Claim status: `search-replay-certificate-candidates-only`.\n",
        report.schema_version, report.matrix_id
    );
    out.push_str(
        "This matrix runs deterministic witness policies across a small lane/digit surface and emits proof-carrying certificate candidates. Lean status records whether each row is already covered by a generated replay module or remains future atlas work.\n\n",
    );
    let summary = &report.summary;
    out.push_str("## Summary\n\n");
    let _ = writeln!(out, "- Rows/certificates: `{}`", summary.row_count);
    let _ = writeln!(out, "- Lanes: `{}`", summary.lane_count);
    let _ = writeln!(
        out,
        "- Seed-origin policies: `{}`",
        summary.seed_origin_policy_count
    );
    let _ = writeln!(
        out,
        "- Visible digit counts: `{}`",
        summary.visible_digit_count
    );
    let _ = writeln!(
        out,
        "- Lean statuses: canonical `{}`, matrix-promoted `{}`, small candidates `{}`, atlas-only large `{}`",
        summary.canonical_lean_promoted_count,
        summary.matrix_lean_promoted_count,
        summary.small_lean_candidate_count,
        summary.atlas_only_large_candidate_count
    );
    let _ = writeln!(
        out,
        "- Total scanned/rejected/survivors: `{}/{}/{}`",
        summary.total_scanned_seed_count,
        summary.total_residue_rejected_count,
        summary.total_residue_survivor_count
    );
    let _ = writeln!(
        out,
        "- Max first-accepted distance: `{}`",
        summary.max_first_accepted_distance
    );
    let _ = writeln!(
        out,
        "- Primality proof status: `{}`\n",
        summary.primality_proof_status
    );

    out.push_str("## Matrix Rows\n\n");
    out.push_str("| Artifact | Lane | Policy | Digits | Witness seed | Distance | Replay rejected/survivors | Geometry | Lean status |\n");
    out.push_str("|---|---|---|---:|---:|---:|---:|---|---|\n");
    for row in &report.rows {
        let _ = writeln!(
            out,
            "| `{}` | `{}` | `{}` | {} | {} | {} | {}/{} | `{}` | `{}` |",
            row.artifact_id,
            row.lane_label,
            row.seed_origin_policy,
            row.visible_digits,
            row.witness_seed,
            row.first_accepted_distance,
            row.residue_rejected_count,
            row.residue_survivor_count,
            row.rejection_geometry,
            proof_carrying_witness_policy_matrix_lean_status_label(&row.lean_promotion_status)
        );
    }

    out
}

fn proof_carrying_witness_policy_matrix_lean_status(
    spec: &ProofCarryingWitnessPolicyMatrixSpec,
    certificate: &ProofCarryingWitnessCertificate,
) -> ProofCarryingWitnessPolicyMatrixLeanPromotionStatus {
    if matches!(
        spec.role,
        "canonical_seed60_128d" | "teaching_seed0_38d" | "timestamp_policy_29d_trial0"
    ) && certificate.settings.base == 10
        && certificate.settings.outer == 3
        && certificate.settings.inner == 7
        && certificate.settings.k_outer == 2
        && certificate.settings.k_inner == 1
    {
        ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanCanonical
    } else if matches!(
        spec.artifact_id,
        "matrix-decimal-readable-22d-seed0"
            | "matrix-decimal-classic-22d-seed0"
            | "matrix-decimal-breathing-22d-seed0"
            | "matrix-decimal-readable-64d-seed0"
            | "matrix-decimal-readable-96d-seed0"
            | "matrix-decimal-classic-64d-seed0"
            | "matrix-decimal-breathing-64d-seed0"
            | "matrix-decimal-breathing-96d-seed0"
            | "matrix-decimal-classic-96d-seed0"
            | "matrix-base30-wheel-64d-seed0"
            | "matrix-base30-wheel-96d-seed0"
            | "matrix-base6-compact-18d-seed0"
            | "matrix-base12-compact-18d-seed0"
            | "matrix-base6-compact-64d-seed0"
            | "matrix-base6-compact-96d-seed0"
            | "matrix-base12-compact-64d-seed0"
            | "matrix-base12-compact-96d-seed0"
            | "matrix-base30-wheel-18d-seed0"
    ) {
        ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanPolicyMatrix
    } else if certificate.settings.visible_digits <= 38 {
        ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::LeanCandidateSmallNativeDecide
    } else {
        ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::AtlasOnlyLargeCandidate
    }
}

fn proof_carrying_witness_policy_matrix_lean_status_label(
    status: &ProofCarryingWitnessPolicyMatrixLeanPromotionStatus,
) -> &'static str {
    match status {
        ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanCanonical => {
            "generated-lean-canonical"
        }
        ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanPolicyMatrix => {
            "generated-lean-policy-matrix"
        }
        ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::LeanCandidateSmallNativeDecide => {
            "lean-candidate-small-native-decide"
        }
        ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::AtlasOnlyLargeCandidate => {
            "atlas-only-large-candidate"
        }
    }
}

fn proof_carrying_witness_policy_matrix_atlas_coverage_row(
    row: &ProofCarryingWitnessPolicyMatrixRow,
) -> ProofCarryingWitnessPolicyMatrixAtlasCoverageRow {
    let lean_links = proof_carrying_witness_policy_matrix_lean_links_for_row(row);
    let lean_replay_coverage = if lean_links.is_some() {
        ProofCarryingWitnessPolicyMatrixLeanReplayCoverage::LeanReplayCertified
    } else {
        ProofCarryingWitnessPolicyMatrixLeanReplayCoverage::NotLeanPromoted
    };

    ProofCarryingWitnessPolicyMatrixAtlasCoverageRow {
        artifact_id: row.artifact_id.clone(),
        role: row.role.clone(),
        seed_origin_policy: row.seed_origin_policy.clone(),
        lane_label: row.lane_label.clone(),
        lane_id: row.lane_id.clone(),
        base: row.base,
        outer: row.outer,
        inner: row.inner,
        k_outer: row.k_outer,
        k_inner: row.k_inner,
        visible_digits: row.visible_digits,
        middle_width: row.middle_width,
        witness_seed: row.witness_seed,
        first_accepted_distance: row.first_accepted_distance,
        scanned_seed_count: row.scanned_seed_count,
        residue_rejected_count: row.residue_rejected_count,
        residue_survivor_count: row.residue_survivor_count,
        non_accepted_residue_survivor_count: row.non_accepted_residue_survivor_count,
        probable_prime_tests: row.probable_prime_tests,
        rejection_geometry: row.rejection_geometry.clone(),
        lean_promotion_status: row.lean_promotion_status.clone(),
        lean_replay_coverage,
        lean_links,
        primality_proof_status: row.primality_proof_status.clone(),
    }
}

fn proof_carrying_witness_policy_matrix_lean_links_for_row(
    row: &ProofCarryingWitnessPolicyMatrixRow,
) -> Option<ProofCarryingWitnessSearchPolicyLeanLinks> {
    let module = proof_carrying_witness_policy_matrix_lean_module_for_row(row)?;
    let theorem_names = proof_carrying_witness_lean_theorem_names(&module);
    let theorem_wrapper_first_accepted_survivor =
        (row.role == "teaching_seed0_38d").then(|| {
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_search_replay_first_accepted_survivor"
                .to_string()
        });

    Some(ProofCarryingWitnessSearchPolicyLeanLinks {
        generated_lean_module: module,
        search_replay_certificate: theorem_names.search_replay_certificate,
        replay_accounting_exact: theorem_names.search_replay_accounting_exact,
        first_accepted_survivor: theorem_names.search_replay_first_accepted_survivor,
        theorem_wrapper_first_accepted_survivor,
    })
}

fn proof_carrying_witness_policy_matrix_lean_module_for_row(
    row: &ProofCarryingWitnessPolicyMatrixRow,
) -> Option<String> {
    match row.artifact_id.as_str() {
        "matrix-decimal-readable-22d-seed0" => {
            Some("PrimeArithmetic.Generated.Witness.MatrixDecimalReadable22".to_string())
        }
        "matrix-decimal-classic-22d-seed0" => {
            Some("PrimeArithmetic.Generated.Witness.MatrixDecimalClassic22".to_string())
        }
        "matrix-decimal-breathing-22d-seed0" => {
            Some("PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing22".to_string())
        }
        "matrix-decimal-readable-64d-seed0" => {
            Some("PrimeArithmetic.Generated.Witness.MatrixDecimalReadable64".to_string())
        }
        "matrix-decimal-readable-96d-seed0" => {
            Some("PrimeArithmetic.Generated.Witness.MatrixDecimalReadable96".to_string())
        }
        "matrix-decimal-classic-64d-seed0" => {
            Some("PrimeArithmetic.Generated.Witness.MatrixDecimalClassic64".to_string())
        }
        "matrix-decimal-breathing-64d-seed0" => {
            Some("PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing64".to_string())
        }
        "matrix-decimal-breathing-96d-seed0" => {
            Some("PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing96".to_string())
        }
        "matrix-decimal-classic-96d-seed0" => {
            Some("PrimeArithmetic.Generated.Witness.MatrixDecimalClassic96".to_string())
        }
        "matrix-base30-wheel-64d-seed0" => {
            Some("PrimeArithmetic.Generated.Witness.MatrixBase30Wheel64".to_string())
        }
        "matrix-base30-wheel-96d-seed0" => {
            Some("PrimeArithmetic.Generated.Witness.MatrixBase30Wheel96".to_string())
        }
        "matrix-base6-compact-18d-seed0" => {
            Some("PrimeArithmetic.Generated.Witness.MatrixBase6Compact18".to_string())
        }
        "matrix-base12-compact-18d-seed0" => {
            Some("PrimeArithmetic.Generated.Witness.MatrixBase12Compact18".to_string())
        }
        "matrix-base6-compact-64d-seed0" => {
            Some("PrimeArithmetic.Generated.Witness.MatrixBase6Compact64".to_string())
        }
        "matrix-base6-compact-96d-seed0" => {
            Some("PrimeArithmetic.Generated.Witness.MatrixBase6Compact96".to_string())
        }
        "matrix-base12-compact-64d-seed0" => {
            Some("PrimeArithmetic.Generated.Witness.MatrixBase12Compact64".to_string())
        }
        "matrix-base12-compact-96d-seed0" => {
            Some("PrimeArithmetic.Generated.Witness.MatrixBase12Compact96".to_string())
        }
        "matrix-base30-wheel-18d-seed0" => {
            Some("PrimeArithmetic.Generated.Witness.MatrixBase30Wheel18".to_string())
        }
        _ => match row.role.as_str() {
            "canonical_seed60_128d" => Some("PrimeArithmetic.Generated.Witness.Seed60".to_string()),
            "teaching_seed0_38d" => {
                Some("PrimeArithmetic.Generated.Witness.Teaching38".to_string())
            }
            "timestamp_policy_29d_trial0" => {
                Some("PrimeArithmetic.Generated.Witness.TimestampPolicy29Trial0".to_string())
            }
            _ => None,
        },
    }
}

fn proof_carrying_witness_policy_matrix_atlas_lane_rows(
    coverage_rows: &[ProofCarryingWitnessPolicyMatrixAtlasCoverageRow],
) -> Vec<ProofCarryingWitnessPolicyMatrixAtlasLaneRow> {
    let mut lanes =
        BTreeMap::<(String, String), Vec<&ProofCarryingWitnessPolicyMatrixAtlasCoverageRow>>::new();
    for row in coverage_rows {
        lanes
            .entry((row.lane_label.clone(), row.lane_id.clone()))
            .or_default()
            .push(row);
    }

    lanes
        .into_iter()
        .map(|((lane_label, lane_id), rows)| {
            let visible_digits = rows
                .iter()
                .map(|row| row.visible_digits)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            let seed_origin_policies = rows
                .iter()
                .map(|row| row.seed_origin_policy.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            let rejection_geometries = rows
                .iter()
                .map(|row| row.rejection_geometry.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            let promoted_replay_certified_count = rows
                .iter()
                .filter(|row| {
                    row.lean_replay_coverage
                        == ProofCarryingWitnessPolicyMatrixLeanReplayCoverage::LeanReplayCertified
                })
                .count();
            ProofCarryingWitnessPolicyMatrixAtlasLaneRow {
                lane_label,
                lane_id,
                artifact_count: rows.len(),
                promoted_replay_certified_count,
                all_rows_lean_replay_certified: promoted_replay_certified_count == rows.len(),
                visible_digits,
                seed_origin_policies,
                rejection_geometries,
                min_first_accepted_distance: rows
                    .iter()
                    .map(|row| row.first_accepted_distance)
                    .min()
                    .unwrap_or(0),
                max_first_accepted_distance: rows
                    .iter()
                    .map(|row| row.first_accepted_distance)
                    .max()
                    .unwrap_or(0),
                total_scanned_seed_count: rows.iter().map(|row| row.scanned_seed_count).sum(),
                total_residue_rejected_count: rows
                    .iter()
                    .map(|row| row.residue_rejected_count)
                    .sum(),
                total_residue_survivor_count: rows
                    .iter()
                    .map(|row| row.residue_survivor_count)
                    .sum(),
            }
        })
        .collect()
}

fn proof_carrying_witness_policy_matrix_atlas_rejection_geometry_rows(
    coverage_rows: &[ProofCarryingWitnessPolicyMatrixAtlasCoverageRow],
) -> Vec<ProofCarryingWitnessPolicyMatrixAtlasRejectionGeometryRow> {
    let mut geometries =
        BTreeMap::<String, Vec<&ProofCarryingWitnessPolicyMatrixAtlasCoverageRow>>::new();
    for row in coverage_rows {
        geometries
            .entry(row.rejection_geometry.clone())
            .or_default()
            .push(row);
    }

    geometries
        .into_iter()
        .map(|(rejection_geometry, rows)| {
            let promoted_replay_certified_count = rows
                .iter()
                .filter(|row| {
                    row.lean_replay_coverage
                        == ProofCarryingWitnessPolicyMatrixLeanReplayCoverage::LeanReplayCertified
                })
                .count();
            ProofCarryingWitnessPolicyMatrixAtlasRejectionGeometryRow {
                rejection_geometry,
                artifact_count: rows.len(),
                artifact_ids: rows.iter().map(|row| row.artifact_id.clone()).collect(),
                promoted_replay_certified_count,
                max_first_accepted_distance: rows
                    .iter()
                    .map(|row| row.first_accepted_distance)
                    .max()
                    .unwrap_or(0),
            }
        })
        .collect()
}

fn proof_carrying_witness_policy_matrix_atlas_promoted_large_replay_geometry_rows(
    coverage_rows: &[ProofCarryingWitnessPolicyMatrixAtlasCoverageRow],
) -> Vec<ProofCarryingWitnessPolicyMatrixAtlasPromotedLargeReplayGeometryRow> {
    let mut rows = coverage_rows
        .iter()
        .filter(|row| {
            row.visible_digits >= 64
                && row.lean_promotion_status
                    == ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanPolicyMatrix
        })
        .filter_map(|row| {
            let links = row.lean_links.as_ref()?;
            Some(ProofCarryingWitnessPolicyMatrixAtlasPromotedLargeReplayGeometryRow {
                rank: 0,
                artifact_id: row.artifact_id.clone(),
                lane_label: row.lane_label.clone(),
                lane_id: row.lane_id.clone(),
                base: row.base,
                outer: row.outer,
                inner: row.inner,
                k_outer: row.k_outer,
                k_inner: row.k_inner,
                visible_digits: row.visible_digits,
                first_accepted_distance: row.first_accepted_distance,
                scanned_seed_count: row.scanned_seed_count,
                residue_rejected_count: row.residue_rejected_count,
                residue_survivor_count: row.residue_survivor_count,
                non_accepted_residue_survivor_count: row.non_accepted_residue_survivor_count,
                rejection_geometry: row.rejection_geometry.clone(),
                generated_lean_module: links.generated_lean_module.clone(),
                replay_accounting_exact: links.replay_accounting_exact.clone(),
                first_accepted_survivor: links.first_accepted_survivor.clone(),
                primality_proof_status: row.primality_proof_status.clone(),
            })
        })
        .collect::<Vec<_>>();

    rows.sort_by(|left, right| {
        right
            .first_accepted_distance
            .cmp(&left.first_accepted_distance)
            .then_with(|| {
                right
                    .residue_survivor_count
                    .cmp(&left.residue_survivor_count)
            })
            .then_with(|| left.artifact_id.cmp(&right.artifact_id))
    });
    for (index, row) in rows.iter_mut().enumerate() {
        row.rank = index + 1;
    }
    rows
}

fn proof_carrying_witness_policy_matrix_next_replay_target(
    coverage_rows: &[ProofCarryingWitnessPolicyMatrixAtlasCoverageRow],
) -> ProofCarryingWitnessPolicyMatrixNextReplayTarget {
    let mut candidates = coverage_rows
        .iter()
        .filter(|row| {
            row.lean_promotion_status
                == ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::AtlasOnlyLargeCandidate
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        right
            .visible_digits
            .cmp(&left.visible_digits)
            .then_with(|| {
                right
                    .first_accepted_distance
                    .cmp(&left.first_accepted_distance)
            })
            .then_with(|| {
                right
                    .residue_survivor_count
                    .cmp(&left.residue_survivor_count)
            })
            .then_with(|| left.artifact_id.cmp(&right.artifact_id))
    });

    if let Some(row) = candidates.first() {
        return ProofCarryingWitnessPolicyMatrixNextReplayTarget {
            status: ProofCarryingWitnessPolicyMatrixNextReplayTargetStatus::Selected,
            reason:
                "highest-ranked atlas-only large row by visible digits, first-accepted distance, and survivor count"
                    .to_string(),
            artifact_id: Some(row.artifact_id.clone()),
            lane_label: Some(row.lane_label.clone()),
            lane_id: Some(row.lane_id.clone()),
            visible_digits: Some(row.visible_digits),
            first_accepted_distance: Some(row.first_accepted_distance),
            rejection_geometry: Some(row.rejection_geometry.clone()),
        };
    }

    ProofCarryingWitnessPolicyMatrixNextReplayTarget {
        status: ProofCarryingWitnessPolicyMatrixNextReplayTargetStatus::NoneCurrentSmokeMatrixFullyCovered,
        reason:
            "current smoke policy matrix has no atlas-only large replay target; every row is already covered by a generated Lean replay module"
                .to_string(),
        artifact_id: None,
        lane_label: None,
        lane_id: None,
        visible_digits: None,
        first_accepted_distance: None,
        rejection_geometry: None,
    }
}

fn proof_carrying_witness_policy_matrix_lean_replay_coverage_label(
    coverage: &ProofCarryingWitnessPolicyMatrixLeanReplayCoverage,
) -> &'static str {
    match coverage {
        ProofCarryingWitnessPolicyMatrixLeanReplayCoverage::LeanReplayCertified => {
            "lean-replay-certified"
        }
        ProofCarryingWitnessPolicyMatrixLeanReplayCoverage::NotLeanPromoted => "not-lean-promoted",
    }
}

fn proof_carrying_witness_policy_matrix_next_replay_target_status_label(
    status: &ProofCarryingWitnessPolicyMatrixNextReplayTargetStatus,
) -> &'static str {
    match status {
        ProofCarryingWitnessPolicyMatrixNextReplayTargetStatus::Selected => "selected",
        ProofCarryingWitnessPolicyMatrixNextReplayTargetStatus::NoneCurrentSmokeMatrixFullyCovered => {
            "none-current-smoke-matrix-fully-covered"
        }
    }
}

pub fn render_proof_carrying_witness_lean_module(
    certificate: &ProofCarryingWitnessCertificate,
    module_name: &str,
    source_certificate_path: &str,
    generated_by_command: &str,
) -> Result<String, fmt::Error> {
    use std::fmt::Write as _;

    let mut out = String::new();
    let config = &certificate.affine_construction;
    let witness = &certificate.witness;

    writeln!(
        out,
        "import PrimeArithmetic.Witness.SearchReplayCertificate"
    )?;
    writeln!(out)?;
    writeln!(out, "namespace {module_name}")?;
    writeln!(out)?;
    writeln!(out, "open PrimeArithmetic.Structure")?;
    writeln!(out)?;
    writeln!(out, "/-!")?;
    writeln!(
        out,
        "Runtime-exported proof-carrying witness certificate arithmetic."
    )?;
    writeln!(out)?;
    writeln!(out, "Generated by:")?;
    writeln!(out, "`{generated_by_command}`")?;
    writeln!(out)?;
    writeln!(out, "Source certificate: `{}`", source_certificate_path)?;
    writeln!(out)?;
    writeln!(
        out,
        "This module proves exact construction and residue-row arithmetic only;"
    )?;
    writeln!(out, "it does not certify primality.")?;
    writeln!(out, "-/")?;
    writeln!(out)?;
    writeln!(
        out,
        "def certificateSchemaVersion : String := {}",
        lean_string_literal(&certificate.schema_version)
    )?;
    writeln!(
        out,
        "def sourceCertificatePath : String := {}",
        lean_string_literal(source_certificate_path)
    )?;
    writeln!(out)?;
    writeln!(out, "def config : SymmetricTemplateConfig where")?;
    writeln!(out, "  base := {}", config.base)?;
    writeln!(out, "  outer := {}", config.outer)?;
    writeln!(out, "  inner := {}", config.inner)?;
    writeln!(out, "  kOuter := {}", config.k_outer)?;
    writeln!(out, "  kInner := {}", config.k_inner)?;
    writeln!(out, "  middleWidth := {}", config.middle_width)?;
    writeln!(out)?;
    writeln!(
        out,
        "def inputSeed : ℕ := {}",
        certificate.settings.input_seed
    )?;
    writeln!(out, "def witnessSeed : ℕ := {}", witness.witness_seed)?;
    writeln!(out)?;
    writeln!(out, "def residueModuli : List ℕ := [")?;
    for row in &certificate.residue_rows {
        writeln!(out, "  {},", row.modulus)?;
    }
    writeln!(out, "]")?;
    writeln!(out)?;
    writeln!(out, "def rejectionExamples : List (ℕ × ℕ) := [")?;
    for row in &certificate.rejection_examples {
        writeln!(out, "  ({}, {}),", row.seed, row.rejected_by_modulus)?;
    }
    writeln!(out, "]")?;
    writeln!(out)?;
    if let Some(replay) = &certificate.search_replay {
        writeln!(
            out,
            "def searchReplayWitnessOffset : ℕ := {}",
            replay.witness_offset
        )?;
        writeln!(
            out,
            "def searchReplayScannedSeedCount : ℕ := {}",
            replay.scanned_seed_count
        )?;
        writeln!(
            out,
            "def searchReplayResidueRejectedCount : ℕ := {}",
            replay.residue_rejected_count
        )?;
        writeln!(
            out,
            "def searchReplayResidueSurvivorCount : ℕ := {}",
            replay.residue_survivor_count
        )?;
        writeln!(out)?;
        writeln!(out, "def searchReplaySeeds : List ℕ :=")?;
        writeln!(
            out,
            "  PrimeArithmetic.Witness.contiguousReplaySeeds inputSeed searchReplayScannedSeedCount"
        )?;
        writeln!(out)?;
        writeln!(out, "def searchReplayResidueRejections : List (ℕ × ℕ) := [")?;
        for row in replay
            .rows
            .iter()
            .filter(|row| row.rejected_by_modulus.is_some())
        {
            writeln!(
                out,
                "  ({}, {}),",
                row.seed,
                row.rejected_by_modulus
                    .expect("filtered rows have a rejection modulus")
            )?;
        }
        writeln!(out, "]")?;
        writeln!(out)?;
        writeln!(out, "def searchReplayResidueSurvivors : List ℕ := [")?;
        for row in replay.rows.iter().filter(|row| row.residue_survived) {
            writeln!(out, "  {},", row.seed)?;
        }
        writeln!(out, "]")?;
        writeln!(out)?;
        writeln!(
            out,
            "def searchReplayNonAcceptedResidueSurvivors : List ℕ := ["
        )?;
        for row in replay
            .rows
            .iter()
            .filter(|row| row.residue_survived && !row.accepted_witness)
        {
            writeln!(out, "  {},", row.seed)?;
        }
        writeln!(out, "]")?;
        writeln!(out)?;
        writeln!(
            out,
            "def searchReplayAcceptedResidueSurvivors : List ℕ := ["
        )?;
        for row in replay.rows.iter().filter(|row| row.accepted_witness) {
            writeln!(out, "  {},", row.seed)?;
        }
        writeln!(out, "]")?;
        writeln!(out)?;
    }
    writeln!(
        out,
        "theorem certificateSchemaVersion_value : certificateSchemaVersion = {} := rfl",
        lean_string_literal(&certificate.schema_version)
    )?;
    writeln!(out)?;
    writeln!(
        out,
        "theorem sourceCertificatePath_value : sourceCertificatePath = {} := rfl",
        lean_string_literal(source_certificate_path)
    )?;
    writeln!(out)?;
    writeln!(
        out,
        "theorem width_value : width config = {} := by",
        certificate.settings.visible_digits
    )?;
    writeln!(out, "  native_decide")?;
    writeln!(out)?;
    writeln!(
        out,
        "theorem shift_value : templateShift config = {} := by",
        config.shift
    )?;
    writeln!(out, "  native_decide")?;
    writeln!(out)?;
    writeln!(
        out,
        "theorem gradient_value : templateGradient config = {} := by",
        config.gradient
    )?;
    writeln!(out, "  native_decide")?;
    writeln!(out)?;
    writeln!(
        out,
        "theorem witness_value : templateValue config witnessSeed = {} := by",
        config.decimal_value
    )?;
    writeln!(out, "  native_decide")?;
    writeln!(out)?;
    writeln!(out, "theorem witness_value_eq_shift_add_gradient :")?;
    writeln!(
        out,
        "    templateValue config witnessSeed = templateShift config + witnessSeed * templateGradient config := by"
    )?;
    writeln!(
        out,
        "  simpa using templateValue_eq_shift_add_gradient config witnessSeed"
    )?;
    writeln!(out)?;
    writeln!(
        out,
        "theorem residueModuli_length : residueModuli.length = {} := by",
        certificate.residue_rows.len()
    )?;
    writeln!(out, "  native_decide")?;
    writeln!(out)?;
    writeln!(
        out,
        "theorem residueModuli_nodup : residueModuli.Nodup := by"
    )?;
    writeln!(out, "  native_decide")?;
    writeln!(out)?;

    for row in &certificate.residue_rows {
        writeln!(out, "theorem residue_row_mod{} :", row.modulus)?;
        writeln!(
            out,
            "    templateShift config % {} = {} ∧",
            row.modulus, row.shift_mod
        )?;
        writeln!(
            out,
            "      templateGradient config % {} = {} ∧",
            row.modulus, row.gradient_mod
        )?;
        writeln!(
            out,
            "      witnessSeed % {} = {} ∧",
            row.modulus, row.seed_mod
        )?;
        writeln!(
            out,
            "      templateValue config witnessSeed % {} = {} ∧",
            row.modulus, row.value_mod
        )?;
        writeln!(out, "      (templateShift config % {} +", row.modulus)?;
        writeln!(
            out,
            "        (templateGradient config % {}) * (witnessSeed % {})) % {} = {} ∧",
            row.modulus, row.modulus, row.modulus, row.affine_residue_mod
        )?;
        writeln!(
            out,
            "      templateValue config witnessSeed % {} ≠ 0 := by",
            row.modulus
        )?;
        writeln!(out, "  native_decide")?;
        writeln!(out)?;
        writeln!(
            out,
            "theorem survives_mod{} : templateValue config witnessSeed % {} ≠ 0 := by",
            row.modulus, row.modulus
        )?;
        writeln!(out, "  native_decide")?;
        writeln!(out)?;
    }

    writeln!(out, "theorem residueFunnelAffineChecks")?;
    writeln!(out, "    {{modulus : ℕ}} (h : modulus ∈ residueModuli) :")?;
    writeln!(out, "    templateValue config witnessSeed % modulus =")?;
    writeln!(
        out,
        "      (templateShift config % modulus + (templateGradient config % modulus) * (witnessSeed % modulus)) % modulus := by"
    )?;
    writeln!(out, "  simp [residueModuli] at h")?;
    writeln!(
        out,
        "  rcases h with {} <;> native_decide",
        repeated_cases("rfl", certificate.residue_rows.len())
    )?;
    writeln!(out)?;
    writeln!(out, "theorem residueFunnelSurvives")?;
    writeln!(out, "    {{modulus : ℕ}} (h : modulus ∈ residueModuli) :")?;
    writeln!(
        out,
        "    templateValue config witnessSeed % modulus ≠ 0 := by"
    )?;
    writeln!(out, "  simp [residueModuli] at h")?;
    writeln!(
        out,
        "  rcases h with {} <;> native_decide",
        repeated_cases("rfl", certificate.residue_rows.len())
    )?;
    writeln!(out)?;

    for row in &certificate.rejection_examples {
        writeln!(
            out,
            "theorem rejection_seed{}_mod{} :",
            row.seed, row.rejected_by_modulus
        )?;
        writeln!(
            out,
            "    templateValue config {} % {} = 0 ∧",
            row.seed, row.rejected_by_modulus
        )?;
        writeln!(
            out,
            "      (templateShift config % {} +",
            row.rejected_by_modulus
        )?;
        writeln!(
            out,
            "        (templateGradient config % {}) * ({} % {})) % {} = 0 := by",
            row.rejected_by_modulus, row.seed, row.rejected_by_modulus, row.rejected_by_modulus
        )?;
        writeln!(out, "  native_decide")?;
        writeln!(out)?;
    }

    writeln!(out, "theorem rejectionExamplesReject")?;
    writeln!(
        out,
        "    {{seed modulus : ℕ}} (h : (seed, modulus) ∈ rejectionExamples) :"
    )?;
    writeln!(out, "    templateValue config seed % modulus = 0 := by")?;
    writeln!(out, "  simp [rejectionExamples] at h")?;
    writeln!(
        out,
        "  rcases h with {} <;> native_decide",
        repeated_cases("⟨rfl, rfl⟩", certificate.rejection_examples.len())
    )?;
    writeln!(out)?;

    if let Some(replay) = &certificate.search_replay {
        let emit_individual_replay_theorems =
            proof_carrying_witness_emit_individual_search_replay_theorems(certificate);
        writeln!(
            out,
            "theorem searchReplaySeeds_length : searchReplaySeeds.length = {} := by",
            replay.rows.len()
        )?;
        writeln!(
            out,
            "  simpa [searchReplaySeeds, searchReplayScannedSeedCount] using"
        )?;
        writeln!(
            out,
            "    PrimeArithmetic.Witness.contiguousReplaySeeds_length inputSeed searchReplayScannedSeedCount"
        )?;
        writeln!(out)?;
        writeln!(out, "theorem searchReplaySeeds_mem_iff {{seed : ℕ}} :")?;
        writeln!(
            out,
            "    seed ∈ searchReplaySeeds ↔ inputSeed ≤ seed ∧ seed < inputSeed + searchReplayScannedSeedCount := by"
        )?;
        writeln!(out, "  simpa [searchReplaySeeds] using")?;
        writeln!(
            out,
            "    (PrimeArithmetic.Witness.mem_contiguousReplaySeeds (inputSeed := inputSeed) (scannedSeedCount := searchReplayScannedSeedCount) (seed := seed))"
        )?;
        writeln!(out)?;
        writeln!(
            out,
            "theorem searchReplayWitnessSeed : inputSeed + searchReplayWitnessOffset = witnessSeed := by"
        )?;
        writeln!(out, "  native_decide")?;
        writeln!(out)?;
        writeln!(out, "theorem searchReplayScannedSeedCount_value :")?;
        writeln!(
            out,
            "    searchReplayScannedSeedCount = {} := by",
            replay.scanned_seed_count
        )?;
        writeln!(out, "  native_decide")?;
        writeln!(out)?;
        writeln!(out, "theorem searchReplayResidueRejectedCount_value :")?;
        writeln!(
            out,
            "    searchReplayResidueRejectedCount = {} := by",
            replay.residue_rejected_count
        )?;
        writeln!(out, "  native_decide")?;
        writeln!(out)?;
        writeln!(out, "theorem searchReplayResidueSurvivorCount_value :")?;
        writeln!(
            out,
            "    searchReplayResidueSurvivorCount = {} := by",
            replay.residue_survivor_count
        )?;
        writeln!(out, "  native_decide")?;
        writeln!(out)?;

        if emit_individual_replay_theorems {
            for row in &replay.rows {
                if let Some(modulus) = row.rejected_by_modulus {
                    writeln!(
                        out,
                        "theorem search_replay_seed{}_rejected_mod{} :",
                        row.seed, modulus
                    )?;
                    writeln!(
                        out,
                        "    templateValue config {} % {} = 0 ∧",
                        row.seed, modulus
                    )?;
                    writeln!(out, "      (templateShift config % {} +", modulus)?;
                    writeln!(
                        out,
                        "        (templateGradient config % {}) * ({} % {})) % {} = 0 := by",
                        modulus, row.seed, modulus, modulus
                    )?;
                    writeln!(out, "  native_decide")?;
                    writeln!(out)?;
                } else if row.residue_survived {
                    writeln!(out, "theorem search_replay_seed{}_survives", row.seed)?;
                    writeln!(out, "    {{modulus : ℕ}} (h : modulus ∈ residueModuli) :")?;
                    writeln!(
                        out,
                        "    templateValue config {} % modulus ≠ 0 := by",
                        row.seed
                    )?;
                    writeln!(out, "  simp [residueModuli] at h")?;
                    writeln!(
                        out,
                        "  rcases h with {} <;> native_decide",
                        repeated_cases("rfl", certificate.residue_rows.len())
                    )?;
                    writeln!(out)?;
                }
            }
        } else {
            writeln!(
                out,
                "-- Large replay windows omit per-seed theorem wrappers; aggregate replay facts below remain proof-carrying."
            )?;
            writeln!(out)?;
        }

        let replay_rejection_count = replay
            .rows
            .iter()
            .filter(|row| row.rejected_by_modulus.is_some())
            .count();
        writeln!(out, "theorem searchReplayResidueRejectionsReject")?;
        writeln!(
            out,
            "    {{seed modulus : ℕ}} (h : (seed, modulus) ∈ searchReplayResidueRejections) :"
        )?;
        writeln!(out, "    templateValue config seed % modulus = 0 := by")?;
        if replay_rejection_count > 0 {
            writeln!(
                out,
                "  simp only [searchReplayResidueRejections, List.mem_cons, List.not_mem_nil, or_false] at h"
            )?;
            if replay_rejection_count == 1 {
                writeln!(out, "  rcases h with ⟨rfl, rfl⟩")?;
                writeln!(out, "  native_decide")?;
            } else {
                writeln!(
                    out,
                    "  rcases h with {} <;> native_decide",
                    repeated_cases("⟨rfl, rfl⟩", replay_rejection_count)
                )?;
            }
        } else {
            writeln!(
                out,
                "  simp only [searchReplayResidueRejections, List.not_mem_nil] at h"
            )?;
        }
        writeln!(out)?;

        let replay_survivor_count = replay
            .rows
            .iter()
            .filter(|row| row.residue_survived)
            .count();
        writeln!(out, "theorem searchReplayResidueSurvivorsSurvive")?;
        writeln!(
            out,
            "    {{seed modulus : ℕ}} (hSeed : seed ∈ searchReplayResidueSurvivors)"
        )?;
        writeln!(out, "    (hModulus : modulus ∈ residueModuli) :")?;
        writeln!(out, "    templateValue config seed % modulus ≠ 0 := by")?;
        writeln!(
            out,
            "  simp only [searchReplayResidueSurvivors, List.mem_cons, List.not_mem_nil, or_false] at hSeed"
        )?;
        writeln!(
            out,
            "  simp only [residueModuli, List.mem_cons, List.not_mem_nil, or_false] at hModulus"
        )?;
        if replay_survivor_count == 1 {
            writeln!(out, "  rcases hSeed with rfl")?;
            writeln!(
                out,
                "  rcases hModulus with {} <;> native_decide",
                repeated_cases("rfl", certificate.residue_rows.len())
            )?;
        } else {
            writeln!(
                out,
                "  rcases hSeed with {} <;> rcases hModulus with {} <;> native_decide",
                repeated_cases("rfl", replay_survivor_count),
                repeated_cases("rfl", certificate.residue_rows.len())
            )?;
        }
        writeln!(out)?;

        writeln!(
            out,
            "def searchReplayCertificate : PrimeArithmetic.Witness.SearchReplayCertificate where"
        )?;
        writeln!(out, "  config := config")?;
        writeln!(out, "  residueModuli := residueModuli")?;
        writeln!(out, "  replaySeeds := searchReplaySeeds")?;
        writeln!(out, "  witnessSeed := witnessSeed")?;
        writeln!(out, "  residueRejections := searchReplayResidueRejections")?;
        writeln!(out, "  residueSurvivors := searchReplayResidueSurvivors")?;
        writeln!(
            out,
            "  nonAcceptedResidueSurvivors := searchReplayNonAcceptedResidueSurvivors"
        )?;
        writeln!(
            out,
            "  acceptedResidueSurvivors := searchReplayAcceptedResidueSurvivors"
        )?;
        writeln!(out, "  witnessInReplay := by")?;
        writeln!(out, "    native_decide")?;
        writeln!(out, "  witnessSurvivor := by")?;
        writeln!(out, "    native_decide")?;
        writeln!(out, "  witnessAccepted := by")?;
        writeln!(out, "    native_decide")?;
        writeln!(out, "  preWitnessClassified := by")?;
        writeln!(out, "    intro seed hSeed hPre")?;
        writeln!(
            out,
            "    have hSeedBounds := searchReplaySeeds_mem_iff.mp hSeed"
        )?;
        writeln!(
            out,
            "    unfold inputSeed searchReplayScannedSeedCount at hSeedBounds"
        )?;
        writeln!(out, "    rcases hSeedBounds with ⟨hSeedLo, hSeedHi⟩")?;
        writeln!(out, "    interval_cases seed")?;
        for row in &replay.rows {
            if row.seed < witness.witness_seed {
                if let Some(modulus) = row.rejected_by_modulus {
                    writeln!(out, "    · left")?;
                    writeln!(out, "      exact ⟨{modulus}, by native_decide⟩")?;
                } else if row.residue_survived {
                    writeln!(out, "    · right")?;
                    writeln!(out, "      native_decide")?;
                } else {
                    writeln!(out, "    · omega")?;
                }
            } else {
                writeln!(out, "    · unfold witnessSeed at hPre")?;
                writeln!(out, "      omega")?;
            }
        }
        writeln!(out, "  rejectionsReject := by")?;
        writeln!(out, "    intro seed modulus h")?;
        writeln!(out, "    exact searchReplayResidueRejectionsReject h")?;
        writeln!(out, "  survivorsSurvive := by")?;
        writeln!(out, "    intro seed modulus hSeed hModulus")?;
        writeln!(
            out,
            "    exact searchReplayResidueSurvivorsSurvive hSeed hModulus"
        )?;
        writeln!(out, "  survivorsInReplay := by")?;
        writeln!(out, "    intro seed hSeed")?;
        writeln!(
            out,
            "    simp only [searchReplayResidueSurvivors, List.mem_cons, List.not_mem_nil, or_false] at hSeed"
        )?;
        if replay_survivor_count == 0 {
            writeln!(out, "    contradiction")?;
        } else {
            writeln!(
                out,
                "    rcases hSeed with {}",
                repeated_cases("rfl", replay_survivor_count)
            )?;
            for _ in 0..replay_survivor_count {
                writeln!(out, "    · native_decide")?;
            }
        }
        writeln!(out, "  replaySurvivorsComplete := by")?;
        writeln!(out, "    intro seed hSeed hSurvives")?;
        writeln!(
            out,
            "    have hSeedBounds := searchReplaySeeds_mem_iff.mp hSeed"
        )?;
        writeln!(
            out,
            "    unfold inputSeed searchReplayScannedSeedCount at hSeedBounds"
        )?;
        writeln!(out, "    rcases hSeedBounds with ⟨hSeedLo, hSeedHi⟩")?;
        writeln!(out, "    interval_cases seed")?;
        for row in &replay.rows {
            if let Some(modulus) = row.rejected_by_modulus {
                writeln!(out, "    · exfalso")?;
                writeln!(
                    out,
                    "      have hModulus : ({modulus} : ℕ) ∈ residueModuli := by"
                )?;
                writeln!(out, "        native_decide")?;
                if emit_individual_replay_theorems {
                    writeln!(
                        out,
                        "      exact hSurvives hModulus (by exact (search_replay_seed{}_rejected_mod{}).1)",
                        row.seed, modulus
                    )?;
                } else {
                    writeln!(
                        out,
                        "      have hRejected : ({}, {}) ∈ searchReplayResidueRejections := by",
                        row.seed, modulus
                    )?;
                    writeln!(out, "        native_decide")?;
                    writeln!(
                        out,
                        "      exact hSurvives hModulus (searchReplayResidueRejectionsReject hRejected)"
                    )?;
                }
            } else if row.residue_survived {
                writeln!(out, "    · native_decide")?;
            } else {
                writeln!(out, "    · unfold witnessSeed at hPre")?;
                writeln!(out, "      omega")?;
            }
        }
        writeln!(out, "  replayPartition := by")?;
        writeln!(out, "    intro seed")?;
        writeln!(out, "    constructor")?;
        writeln!(out, "    · intro hSeed")?;
        writeln!(
            out,
            "      have hSeedBounds := searchReplaySeeds_mem_iff.mp hSeed"
        )?;
        writeln!(
            out,
            "      unfold inputSeed searchReplayScannedSeedCount at hSeedBounds"
        )?;
        writeln!(out, "      rcases hSeedBounds with ⟨hSeedLo, hSeedHi⟩")?;
        writeln!(out, "      interval_cases seed")?;
        for row in &replay.rows {
            if row.rejected_by_modulus.is_some() {
                writeln!(out, "      · left")?;
                writeln!(out, "        native_decide")?;
            } else if row.residue_survived {
                writeln!(out, "      · right")?;
                writeln!(out, "        native_decide")?;
            } else {
                writeln!(out, "      · native_decide")?;
            }
        }
        writeln!(out, "    · intro hPartition")?;
        writeln!(out, "      rcases hPartition with hRejected | hSurvivor")?;
        writeln!(out, "      · apply searchReplaySeeds_mem_iff.mpr")?;
        writeln!(out, "        revert seed")?;
        writeln!(out, "        native_decide")?;
        writeln!(out, "      · apply searchReplaySeeds_mem_iff.mpr")?;
        writeln!(out, "        revert seed")?;
        writeln!(out, "        native_decide")?;
        writeln!(out, "  replayPartitionDisjoint := by")?;
        writeln!(out, "    intro seed hRejected hSurvivor")?;
        if replay_rejection_count == 0 {
            writeln!(
                out,
                "    simp only [searchReplayResidueRejections, PrimeArithmetic.Witness.RejectedSeeds, List.map_nil, List.not_mem_nil] at hRejected"
            )?;
        } else {
            writeln!(
                out,
                "    simp only [searchReplayResidueRejections, searchReplayResidueSurvivors, PrimeArithmetic.Witness.RejectedSeeds, List.map_cons, List.map_nil, List.mem_cons, List.not_mem_nil, or_false] at hRejected hSurvivor"
            )?;
            writeln!(out, "    omega")?;
        }
        let replay_non_accepted_survivor_count = replay
            .rows
            .iter()
            .filter(|row| row.residue_survived && !row.accepted_witness)
            .count();
        let replay_accepted_survivor_count = replay
            .rows
            .iter()
            .filter(|row| row.accepted_witness)
            .count();
        writeln!(out, "  survivorAcceptancePartition := by")?;
        writeln!(out, "    intro seed")?;
        writeln!(out, "    constructor")?;
        writeln!(out, "    · intro hSurvivor")?;
        writeln!(
            out,
            "      simp only [searchReplayResidueSurvivors, List.mem_cons, List.not_mem_nil, or_false] at hSurvivor"
        )?;
        if replay_survivor_count == 0 {
            writeln!(out, "      contradiction")?;
        } else {
            writeln!(
                out,
                "      rcases hSurvivor with {}",
                repeated_cases("rfl", replay_survivor_count)
            )?;
            for _ in 0..replay_survivor_count {
                writeln!(out, "      · native_decide")?;
            }
        }
        writeln!(out, "    · intro hAccepted")?;
        writeln!(out, "      rcases hAccepted with hNonAccepted | hAccepted")?;
        if replay_non_accepted_survivor_count == 0 {
            writeln!(
                out,
                "      · simp only [searchReplayNonAcceptedResidueSurvivors, List.not_mem_nil] at hNonAccepted"
            )?;
        } else {
            writeln!(
                out,
                "      · simp only [searchReplayNonAcceptedResidueSurvivors, List.mem_cons, List.not_mem_nil, or_false] at hNonAccepted"
            )?;
            writeln!(
                out,
                "        rcases hNonAccepted with {}",
                repeated_cases("rfl", replay_non_accepted_survivor_count)
            )?;
            for _ in 0..replay_non_accepted_survivor_count {
                writeln!(out, "        · native_decide")?;
            }
        }
        if replay_accepted_survivor_count == 0 {
            writeln!(
                out,
                "      · simp only [searchReplayAcceptedResidueSurvivors, List.not_mem_nil] at hAccepted"
            )?;
            writeln!(out, "        contradiction")?;
        } else {
            writeln!(
                out,
                "      · simp only [searchReplayAcceptedResidueSurvivors, List.mem_cons, List.not_mem_nil, or_false] at hAccepted"
            )?;
            writeln!(
                out,
                "        rcases hAccepted with {}",
                repeated_cases("rfl", replay_accepted_survivor_count)
            )?;
            for _ in 0..replay_accepted_survivor_count {
                writeln!(out, "        · native_decide")?;
            }
        }
        writeln!(out, "  survivorAcceptanceDisjoint := by")?;
        writeln!(out, "    intro seed hNonAccepted hAccepted")?;
        if replay_non_accepted_survivor_count == 0 {
            writeln!(
                out,
                "    simp only [searchReplayNonAcceptedResidueSurvivors, List.not_mem_nil] at hNonAccepted"
            )?;
        } else {
            writeln!(
                out,
                "    simp only [searchReplayNonAcceptedResidueSurvivors, searchReplayAcceptedResidueSurvivors, List.mem_cons, List.not_mem_nil, or_false] at hNonAccepted hAccepted"
            )?;
            writeln!(out, "    omega")?;
        }
        writeln!(out, "  preWitnessSurvivorsNonAccepted := by")?;
        writeln!(out, "    intro seed hSurvivor hPre")?;
        writeln!(
            out,
            "    simp only [searchReplayResidueSurvivors, List.mem_cons, List.not_mem_nil, or_false] at hSurvivor"
        )?;
        writeln!(
            out,
            "    rcases hSurvivor with {}",
            repeated_cases("rfl", replay_survivor_count)
        )?;
        for row in replay.rows.iter().filter(|row| row.residue_survived) {
            if row.seed < witness.witness_seed && !row.accepted_witness {
                writeln!(out, "    · native_decide")?;
            } else {
                writeln!(out, "    · unfold witnessSeed at hPre")?;
                writeln!(out, "      omega")?;
            }
        }
        writeln!(out, "  acceptedSurvivorsAreWitness := by")?;
        writeln!(out, "    intro seed hAccepted")?;
        writeln!(
            out,
            "    simp only [searchReplayAcceptedResidueSurvivors, List.mem_cons, List.not_mem_nil, or_false] at hAccepted"
        )?;
        if replay_accepted_survivor_count > 0 {
            writeln!(
                out,
                "    rcases hAccepted with {}",
                repeated_cases("rfl", replay_accepted_survivor_count)
            )?;
            for _ in 0..replay_accepted_survivor_count {
                writeln!(out, "    · rfl")?;
            }
        } else {
            writeln!(out, "    contradiction")?;
        }
        writeln!(out)?;
        writeln!(out, "theorem searchReplayPreWitnessComplete :")?;
        writeln!(out, "    searchReplayCertificate.PreWitnessComplete :=")?;
        writeln!(
            out,
            "  PrimeArithmetic.Witness.SearchReplayCertificate.preWitnessComplete searchReplayCertificate"
        )?;
        writeln!(out)?;
        writeln!(out, "theorem searchReplayWitnessSurvives :")?;
        writeln!(out, "    searchReplayCertificate.WitnessSurvives :=")?;
        writeln!(
            out,
            "  PrimeArithmetic.Witness.SearchReplayCertificate.witnessSurvives searchReplayCertificate"
        )?;
        writeln!(out)?;
        writeln!(out, "theorem searchReplaySound :")?;
        writeln!(out, "    searchReplayCertificate.Sound :=")?;
        writeln!(
            out,
            "  PrimeArithmetic.Witness.SearchReplayCertificate.sound searchReplayCertificate"
        )?;
        writeln!(out)?;
        writeln!(out, "theorem searchReplaySurvivorListExact :")?;
        writeln!(out, "    searchReplayCertificate.SurvivorListExact :=")?;
        writeln!(
            out,
            "  PrimeArithmetic.Witness.SearchReplayCertificate.survivorListExact searchReplayCertificate"
        )?;
        writeln!(out)?;
        writeln!(out, "theorem searchReplayPartitionExact :")?;
        writeln!(out, "    searchReplayCertificate.ReplayPartitionExact :=")?;
        writeln!(
            out,
            "  PrimeArithmetic.Witness.SearchReplayCertificate.replayPartitionExact searchReplayCertificate"
        )?;
        writeln!(out)?;
        writeln!(out, "theorem searchReplayCountExact :")?;
        writeln!(out, "    searchReplayCertificate.ReplayCountExact")?;
        writeln!(
            out,
            "      searchReplayScannedSeedCount searchReplayResidueRejectedCount"
        )?;
        writeln!(out, "      searchReplayResidueSurvivorCount :=")?;
        writeln!(
            out,
            "  PrimeArithmetic.Witness.SearchReplayCertificate.replayCountExact"
        )?;
        writeln!(out, "    searchReplayCertificate (by")?;
        writeln!(out, "      native_decide)")?;
        writeln!(out)?;
        writeln!(out, "theorem searchReplayAccountingExact :")?;
        writeln!(out, "    searchReplayCertificate.ReplayAccountingExact")?;
        writeln!(
            out,
            "      searchReplayScannedSeedCount searchReplayResidueRejectedCount"
        )?;
        writeln!(out, "      searchReplayResidueSurvivorCount :=")?;
        writeln!(
            out,
            "  PrimeArithmetic.Witness.SearchReplayCertificate.replayAccountingExact"
        )?;
        writeln!(out, "    searchReplayCertificate searchReplayCountExact")?;
        writeln!(out)?;
        writeln!(out, "theorem searchReplaySurvivorAcceptanceExact :")?;
        writeln!(
            out,
            "    searchReplayCertificate.SurvivorAcceptanceExact :="
        )?;
        writeln!(
            out,
            "  PrimeArithmetic.Witness.SearchReplayCertificate.survivorAcceptanceExact"
        )?;
        writeln!(out, "    searchReplayCertificate")?;
        writeln!(out)?;
        writeln!(out, "theorem searchReplayAcceptedSurvivorExact :")?;
        writeln!(out, "    searchReplayCertificate.AcceptedSurvivorExact :=")?;
        writeln!(
            out,
            "  PrimeArithmetic.Witness.SearchReplayCertificate.acceptedSurvivorExact"
        )?;
        writeln!(out, "    searchReplayCertificate")?;
        writeln!(out)?;
        writeln!(out, "theorem searchReplayPreWitnessSurvivorsNonAccepted :")?;
        writeln!(
            out,
            "    searchReplayCertificate.PreWitnessSurvivorsNonAccepted :="
        )?;
        writeln!(
            out,
            "  PrimeArithmetic.Witness.SearchReplayCertificate.preWitnessSurvivorsNonAcceptedExact"
        )?;
        writeln!(out, "    searchReplayCertificate")?;
        writeln!(out)?;
        writeln!(out, "theorem searchReplayFirstAcceptedSurvivor :")?;
        writeln!(out, "    searchReplayCertificate.FirstAcceptedSurvivor :=")?;
        writeln!(
            out,
            "  PrimeArithmetic.Witness.SearchReplayCertificate.firstAcceptedSurvivor"
        )?;
        writeln!(out, "    searchReplayCertificate")?;
        writeln!(out)?;
    }

    writeln!(out, "end {module_name}")?;

    Ok(out)
}

pub fn verify_proof_carrying_witness_certificate(
    certificate: &ProofCarryingWitnessCertificate,
) -> ProofCarryingWitnessVerificationReport {
    let mut failures = Vec::new();

    check_eq(
        &mut failures,
        "schema_version",
        PROOF_CARRYING_WITNESS_SCHEMA_VERSION,
        certificate.schema_version.as_str(),
    );

    let expected_middle_width =
        certificate
            .settings
            .visible_digits
            .checked_sub(fixed_template_digits((
                certificate.settings.k_outer,
                certificate.settings.k_inner,
            )));
    check_eq(
        &mut failures,
        "witness.middle_width",
        expected_middle_width,
        Some(certificate.witness.middle_width),
    );

    let lane = build_big_affine_lane(
        certificate.settings.base,
        certificate.settings.outer,
        certificate.settings.inner,
        certificate.witness.middle_width,
        (certificate.settings.k_outer, certificate.settings.k_inner),
    );
    let value = candidate_value(&lane, certificate.witness.witness_seed);
    let decimal_value = value.to_str_radix(10);
    let template = template_digits(&lane, certificate.witness.witness_seed);
    let middle = middle_digits(
        lane.base,
        certificate.witness.middle_width,
        certificate.witness.witness_seed,
    );
    let affine_line = format!(
        "N(s) = {} + {}*s",
        lane.shift.to_str_radix(10),
        lane.gradient.to_str_radix(10)
    );
    let compact = compact_description(&lane, certificate.witness.witness_seed);

    check_eq(
        &mut failures,
        "affine_construction.base",
        certificate.settings.base,
        certificate.affine_construction.base,
    );
    check_eq(
        &mut failures,
        "affine_construction.outer",
        certificate.settings.outer,
        certificate.affine_construction.outer,
    );
    check_eq(
        &mut failures,
        "affine_construction.inner",
        certificate.settings.inner,
        certificate.affine_construction.inner,
    );
    check_eq(
        &mut failures,
        "affine_construction.k_outer",
        certificate.settings.k_outer,
        certificate.affine_construction.k_outer,
    );
    check_eq(
        &mut failures,
        "affine_construction.k_inner",
        certificate.settings.k_inner,
        certificate.affine_construction.k_inner,
    );
    check_eq(
        &mut failures,
        "affine_construction.middle_width",
        certificate.witness.middle_width,
        certificate.affine_construction.middle_width,
    );
    check_eq(
        &mut failures,
        "affine_construction.witness_seed",
        certificate.witness.witness_seed,
        certificate.affine_construction.witness_seed,
    );
    check_eq(
        &mut failures,
        "affine_construction.shift",
        lane.shift.to_str_radix(10),
        certificate.affine_construction.shift.clone(),
    );
    check_eq(
        &mut failures,
        "affine_construction.gradient",
        lane.gradient.to_str_radix(10),
        certificate.affine_construction.gradient.clone(),
    );
    check_eq(
        &mut failures,
        "affine_construction.decimal_value",
        decimal_value.clone(),
        certificate.affine_construction.decimal_value.clone(),
    );
    check_eq(
        &mut failures,
        "affine_construction.template_digits",
        template.clone(),
        certificate.affine_construction.template_digits.clone(),
    );
    check_eq(
        &mut failures,
        "affine_construction.middle_digits",
        middle.clone(),
        certificate.affine_construction.middle_digits.clone(),
    );
    check_eq(
        &mut failures,
        "affine_construction.affine_line",
        affine_line,
        certificate.affine_construction.affine_line.clone(),
    );
    check_eq(
        &mut failures,
        "witness.compact_description",
        compact,
        certificate.witness.compact_description.clone(),
    );
    check_eq(
        &mut failures,
        "witness.decimal_digits",
        decimal_value.len(),
        certificate.witness.decimal_digits,
    );

    let parsed_decimal =
        BigUint::parse_bytes(certificate.affine_construction.decimal_value.as_bytes(), 10);
    check_eq(
        &mut failures,
        "affine_construction.decimal_value_parse_matches_affine_value",
        Some(value.clone()),
        parsed_decimal,
    );

    let expected_witness_seed = certificate
        .settings
        .input_seed
        .checked_add(certificate.witness.steps_to_witness);
    check_eq(
        &mut failures,
        "witness_seed_from_input_plus_steps",
        expected_witness_seed,
        Some(certificate.witness.witness_seed),
    );

    let expected_scanned_seed_count = if certificate.settings.exact_seed_only {
        1
    } else {
        certificate.witness.steps_to_witness.saturating_add(1)
    };
    check_eq(
        &mut failures,
        "witness.scanned_seed_count",
        expected_scanned_seed_count,
        certificate.witness.scanned_seed_count,
    );
    check_eq(
        &mut failures,
        "witness.residue_count_balance",
        certificate.witness.scanned_seed_count,
        certificate
            .witness
            .residue_survivor_count
            .saturating_add(certificate.witness.residue_rejected_count),
    );

    let affine_value_matches_decimal =
        decimal_value == certificate.affine_construction.decimal_value;
    check_eq(
        &mut failures,
        "affine_construction.affine_value_matches_decimal",
        affine_value_matches_decimal,
        certificate.affine_construction.affine_value_matches_decimal,
    );
    let template_digits_match_result = template == certificate.affine_construction.template_digits;
    check_eq(
        &mut failures,
        "affine_construction.template_digits_match_result",
        template_digits_match_result,
        certificate.affine_construction.template_digits_match_result,
    );
    let middle_digits_match_result = middle == certificate.affine_construction.middle_digits;
    check_eq(
        &mut failures,
        "affine_construction.middle_digits_match_result",
        middle_digits_match_result,
        certificate.affine_construction.middle_digits_match_result,
    );
    let visible_digit_count_matches_template = certificate
        .affine_construction
        .template_digits
        .chars()
        .count()
        == certificate.settings.visible_digits;
    check_eq(
        &mut failures,
        "affine_construction.visible_digit_count_matches_template",
        visible_digit_count_matches_template,
        certificate
            .affine_construction
            .visible_digit_count_matches_template,
    );
    let decimal_digit_count_matches_value =
        certificate.affine_construction.decimal_value.len() == certificate.witness.decimal_digits;
    check_eq(
        &mut failures,
        "affine_construction.decimal_digit_count_matches_value",
        decimal_digit_count_matches_value,
        certificate
            .affine_construction
            .decimal_digit_count_matches_value,
    );

    let expected_rows =
        build_residue_certificate_rows(&lane, certificate.witness.witness_seed, &value);
    check_eq(
        &mut failures,
        "residue_rows.len",
        expected_rows.len(),
        certificate.residue_rows.len(),
    );
    for (idx, expected) in expected_rows.iter().enumerate() {
        match certificate.residue_rows.get(idx) {
            Some(actual) => check_eq(
                &mut failures,
                &format!("residue_rows[{idx}]"),
                expected,
                actual,
            ),
            None => failures.push(format!("residue_rows[{idx}]: missing expected row")),
        }
    }
    for idx in expected_rows.len()..certificate.residue_rows.len() {
        failures.push(format!("residue_rows[{idx}]: unexpected extra row"));
    }

    let expected_rejections = build_rejection_examples(
        &lane,
        certificate.settings.input_seed,
        certificate.witness.witness_seed,
        certificate.settings.max_steps,
        DEFAULT_REJECTION_EXAMPLE_COUNT,
    );
    check_eq(
        &mut failures,
        "rejection_examples.len",
        expected_rejections.len(),
        certificate.rejection_examples.len(),
    );
    for (idx, expected) in expected_rejections.iter().enumerate() {
        match certificate.rejection_examples.get(idx) {
            Some(actual) => check_eq(
                &mut failures,
                &format!("rejection_examples[{idx}]"),
                expected,
                actual,
            ),
            None => failures.push(format!(
                "rejection_examples[{idx}]: missing expected rejection example"
            )),
        }
    }
    for idx in expected_rejections.len()..certificate.rejection_examples.len() {
        failures.push(format!(
            "rejection_examples[{idx}]: unexpected extra rejection example"
        ));
    }

    let expected_replay = build_search_replay_certificate(
        &lane,
        certificate.settings.input_seed,
        certificate.witness.witness_seed,
        certificate.witness.scanned_seed_count,
        &certificate.settings.probable_prime_bases,
    );
    match &certificate.search_replay {
        Some(actual) => {
            check_eq(
                &mut failures,
                "search_replay",
                expected_replay.clone(),
                actual.clone(),
            );
        }
        None => failures.push("search_replay: missing replay certificate".to_string()),
    }

    let expected_confirmation =
        confirmation_label(&value, &certificate.settings.probable_prime_bases);
    check_eq(
        &mut failures,
        "confirmation.method_label",
        expected_confirmation
            .clone()
            .unwrap_or_else(|| "composite".to_string()),
        certificate.confirmation.method_label.clone(),
    );
    check_eq(
        &mut failures,
        "confirmation.probable_prime_bases",
        certificate.settings.probable_prime_bases.clone(),
        certificate.confirmation.probable_prime_bases.clone(),
    );
    check_eq(
        &mut failures,
        "confirmation.probable_prime_result",
        expected_confirmation.is_some(),
        certificate.confirmation.probable_prime_result,
    );
    check_eq(
        &mut failures,
        "confirmation.primality_proof_status",
        PROBABLE_PRIME_NOT_PROOF_CERTIFIED,
        certificate.confirmation.primality_proof_status.as_str(),
    );

    let mersenne = classify_mersenne(&value);
    check_eq(
        &mut failures,
        "shape.is_mersenne",
        mersenne.is_mersenne,
        certificate.shape.is_mersenne,
    );
    check_eq(
        &mut failures,
        "shape.mersenne_exponent",
        mersenne.mersenne_exponent,
        certificate.shape.mersenne_exponent,
    );
    check_eq(
        &mut failures,
        "shape.mersenne_class",
        mersenne.mersenne_class.clone(),
        certificate.shape.mersenne_class.clone(),
    );
    check_eq(
        &mut failures,
        "shape.exact_not_mersenne",
        !mersenne.is_mersenne && mersenne.mersenne_class == "not_mersenne",
        certificate.shape.exact_not_mersenne,
    );

    check_eq(
        &mut failures,
        "verification_snippets",
        verification_snippets(&decimal_value),
        certificate.verification_snippets.clone(),
    );

    ProofCarryingWitnessVerificationReport {
        schema_version: certificate.schema_version.clone(),
        ok: failures.is_empty(),
        witness_seed: certificate.witness.witness_seed,
        checked_residue_row_count: expected_rows.len(),
        failures,
    }
}

fn build_result(
    config: &SeedToWitnessConfig,
    lane: &BigAffineLane,
    witness_seed: u64,
    stats: SearchHitStats,
    confirmation: String,
) -> SeedToWitnessResult {
    let value = candidate_value(lane, witness_seed);
    let mersenne = classify_mersenne(&value);
    let decimal_value = value.to_str_radix(10);
    let residue_rejected_count = stats
        .scanned_seed_count
        .saturating_sub(stats.residue_survivor_count);
    SeedToWitnessResult {
        input_seed: config.input_seed,
        witness_seed,
        steps_to_witness: stats.steps_to_witness,
        exact_seed_only: config.exact_seed_only,
        max_steps: config.max_steps,
        scanned_seed_count: stats.scanned_seed_count,
        residue_survivor_count: stats.residue_survivor_count,
        residue_rejected_count,
        probable_prime_tests: stats.probable_prime_tests,
        elapsed_seconds: stats.elapsed_seconds,
        base: lane.base,
        outer: lane.outer,
        inner: lane.inner,
        k_outer: lane.k_outer,
        k_inner: lane.k_inner,
        middle_length: lane.middle_length,
        visible_digits: config.visible_digits,
        residue_moduli_label: join_moduli(&residue_moduli(lane.base)),
        shift: lane.shift.to_str_radix(10),
        gradient: lane.gradient.to_str_radix(10),
        affine_line: format!(
            "N(s) = {} + {}*s",
            lane.shift.to_str_radix(10),
            lane.gradient.to_str_radix(10)
        ),
        middle_digits: middle_digits(lane.base, lane.middle_length, witness_seed),
        template_digits: template_digits(lane, witness_seed),
        decimal_digits: decimal_value.len(),
        compact_description: compact_description(lane, witness_seed),
        is_mersenne: mersenne.is_mersenne,
        mersenne_exponent: mersenne.mersenne_exponent,
        mersenne_class: mersenne.mersenne_class,
        verification_snippets: verification_snippets(&decimal_value),
        decimal_value,
        confirmation,
    }
}

fn confirmation_label(value: &BigUint, bases: &[u64]) -> Option<String> {
    if let Ok(value_u64) = value.to_str_radix(10).parse::<u64>() {
        if primal::is_prime(value_u64) {
            return Some("deterministic_primal_u64".to_string());
        }
        return None;
    }
    if is_probable_prime_fixed_bases(value, bases) {
        Some(format!("probable_prime_fixed_{}_bases", bases.len()))
    } else {
        None
    }
}

fn verification_snippets(decimal_value: &str) -> Vec<VerificationSnippet> {
    vec![
        VerificationSnippet {
            tool: "WolframAlpha".to_string(),
            snippet: format!("isprime({decimal_value})"),
        },
        VerificationSnippet {
            tool: "Mathematica".to_string(),
            snippet: format!("PrimeQ[{decimal_value}]"),
        },
        VerificationSnippet {
            tool: "PARI/GP".to_string(),
            snippet: format!("isprime({decimal_value})"),
        },
        VerificationSnippet {
            tool: "Sage".to_string(),
            snippet: format!("is_prime(Integer(\"{decimal_value}\"))"),
        },
    ]
}

fn fixed_template_digits((k_outer, k_inner): (u32, u32)) -> usize {
    (4 + 2 * (k_outer + k_inner)) as usize
}

fn join_moduli(moduli: &[u32]) -> String {
    moduli
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn build_residue_certificate_rows(
    lane: &BigAffineLane,
    seed: u64,
    value: &BigUint,
) -> Vec<ResidueCertificateRow> {
    residue_moduli(lane.base)
        .into_iter()
        .map(|modulus| {
            let shift_mod = biguint_mod_u32(&lane.shift, modulus);
            let gradient_mod = biguint_mod_u32(&lane.gradient, modulus);
            let seed_mod = (seed % u64::from(modulus)) as u32;
            let value_mod = biguint_mod_u32(value, modulus);
            let affine_residue_mod = ((u64::from(shift_mod)
                + u64::from(gradient_mod) * u64::from(seed_mod))
                % u64::from(modulus)) as u32;
            ResidueCertificateRow {
                modulus,
                coprime_to_base: gcd_u32(lane.base, modulus) == 1,
                shift_mod,
                gradient_mod,
                seed_mod,
                value_mod,
                affine_residue_mod,
                affine_residue_check: value_mod == affine_residue_mod,
                survived: value_mod != 0,
            }
        })
        .collect()
}

fn build_search_replay_certificate(
    lane: &BigAffineLane,
    input_seed: u64,
    witness_seed: u64,
    scanned_seed_count: u64,
    probable_prime_bases: &[u64],
) -> SearchReplayCertificate {
    let rows = (0..scanned_seed_count)
        .map(|offset| {
            let seed = input_seed
                .checked_add(offset)
                .expect("verified scan range does not overflow");
            build_search_replay_row(lane, input_seed, witness_seed, seed, probable_prime_bases)
        })
        .collect::<Vec<_>>();
    let residue_survivor_count = rows.iter().filter(|row| row.residue_survived).count() as u64;
    let residue_rejected_count = rows.iter().filter(|row| !row.residue_survived).count() as u64;
    let probable_prime_tests = rows.iter().filter(|row| row.probable_prime_tested).count() as u64;

    SearchReplayCertificate {
        search_order: "walk-forward-by-one".to_string(),
        input_seed,
        witness_seed,
        witness_offset: witness_seed.saturating_sub(input_seed),
        scanned_seed_count,
        residue_survivor_count,
        residue_rejected_count,
        probable_prime_tests,
        complete_through_witness: rows
            .last()
            .map(|row| row.seed == witness_seed && row.accepted_witness)
            .unwrap_or(false),
        rows,
    }
}

fn build_search_replay_row(
    lane: &BigAffineLane,
    input_seed: u64,
    witness_seed: u64,
    seed: u64,
    probable_prime_bases: &[u64],
) -> SearchReplayRow {
    let value = candidate_value(lane, seed);
    let residue_rows = build_residue_certificate_rows(lane, seed, &value);
    let rejected_by_modulus = residue_rows
        .iter()
        .find(|row| !row.survived)
        .map(|row| row.modulus);
    let residue_survived = rejected_by_modulus.is_none();
    let probable_prime_result =
        residue_survived && confirmation_label(&value, probable_prime_bases).is_some();
    let accepted_witness = seed == witness_seed && probable_prime_result;
    let status = if rejected_by_modulus.is_some() {
        SearchReplayRowStatus::ResidueRejected
    } else if accepted_witness {
        SearchReplayRowStatus::AcceptedProbablePrimeWitness
    } else {
        SearchReplayRowStatus::ResidueSurvivorProbablePrimeRejected
    };

    SearchReplayRow {
        offset_from_input_seed: seed.saturating_sub(input_seed),
        seed,
        status,
        rejected_by_modulus,
        residue_survived,
        probable_prime_tested: residue_survived,
        probable_prime_result,
        accepted_witness,
        residue_rows,
    }
}

fn build_rejection_examples(
    lane: &BigAffineLane,
    input_seed: u64,
    witness_seed: u64,
    max_steps: u64,
    limit: usize,
) -> Vec<RejectionExampleRow> {
    let witness_offset = witness_seed.saturating_sub(input_seed);
    let scan_count = max_steps
        .min(witness_offset.saturating_add(DEFAULT_REJECTION_SCAN_EXTRA))
        .max(DEFAULT_REJECTION_SCAN_EXTRA);
    let mut rows = Vec::new();

    for offset in 0..scan_count {
        let Some(seed) = input_seed.checked_add(offset) else {
            break;
        };
        if seed == witness_seed {
            continue;
        }
        if let Some(row) = first_rejection_example(lane, input_seed, seed) {
            rows.push(row);
            if rows.len() == limit {
                break;
            }
        }
    }

    rows
}

fn first_rejection_example(
    lane: &BigAffineLane,
    input_seed: u64,
    seed: u64,
) -> Option<RejectionExampleRow> {
    let value = candidate_value(lane, seed);
    residue_moduli(lane.base).into_iter().find_map(|modulus| {
        let shift_mod = biguint_mod_u32(&lane.shift, modulus);
        let gradient_mod = biguint_mod_u32(&lane.gradient, modulus);
        let seed_mod = (seed % u64::from(modulus)) as u32;
        let value_mod = biguint_mod_u32(&value, modulus);
        if value_mod != 0 {
            return None;
        }
        let affine_residue_mod = ((u64::from(shift_mod)
            + u64::from(gradient_mod) * u64::from(seed_mod))
            % u64::from(modulus)) as u32;
        Some(RejectionExampleRow {
            seed,
            offset_from_input_seed: seed.saturating_sub(input_seed),
            rejected_by_modulus: modulus,
            shift_mod,
            gradient_mod,
            seed_mod,
            value_mod,
            affine_residue_mod,
            affine_residue_check: value_mod == affine_residue_mod,
            rejected: value_mod == 0,
        })
    })
}

fn biguint_mod_u32(value: &BigUint, modulus: u32) -> u32 {
    (value % BigUint::from(modulus))
        .to_u32()
        .expect("remainder modulo u32 fits in u32")
}

fn gcd_u32(mut left: u32, mut right: u32) -> u32 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

fn check_eq<T>(failures: &mut Vec<String>, field: &str, expected: T, actual: T)
where
    T: PartialEq + fmt::Debug,
{
    if expected != actual {
        failures.push(format!("{field}: expected {expected:?}, found {actual:?}"));
    }
}

fn lean_string_literal(value: &str) -> String {
    let escaped = value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n");
    format!("\"{escaped}\"")
}

fn qualified_lean_name(module: &str, declaration: &str) -> String {
    format!("{module}.{declaration}")
}

fn repeated_cases(pattern: &str, count: usize) -> String {
    (0..count).map(|_| pattern).collect::<Vec<_>>().join(" | ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_sixty_returns_known_128_digit_witness() {
        let result =
            find_seed_to_witness(SeedToWitnessConfig::default_for_seed(60)).expect("witness");
        assert_eq!(result.input_seed, 60);
        assert_eq!(result.witness_seed, 60);
        assert_eq!(result.steps_to_witness, 0);
        assert_eq!(result.visible_digits, 128);
        assert_eq!(result.decimal_digits, 128);
        assert!(result
            .decimal_value
            .starts_with("300700000000000000000000000000"));
        assert!(result.decimal_value.ends_with("0000000006007003"));
        assert_eq!(result.confirmation, "probable_prime_fixed_20_bases");
        assert!(!result.is_mersenne);
        assert_eq!(result.mersenne_class, "not_mersenne");
    }

    #[test]
    fn seed_zero_walks_to_reproducible_38_digit_witness() {
        let config = SeedToWitnessConfig::default_for_seed(0)
            .with_visible_digits(38)
            .with_max_steps(100);
        let result = find_seed_to_witness(config).expect("witness");
        assert_eq!(result.witness_seed, 3);
        assert_eq!(result.steps_to_witness, 3);
        assert_eq!(
            result.decimal_value,
            "30070000000000000000000000000000307003"
        );
        assert!(result.scanned_seed_count >= 4);
    }

    #[test]
    fn teaching_certificate_carries_expected_small_rows() {
        let certificate = build_proof_carrying_witness_certificate_for_config(
            SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(38)
                .with_max_steps(100),
        )
        .expect("teaching witness certificate");

        assert_eq!(certificate.witness.witness_seed, 3);
        assert_eq!(certificate.witness.middle_width, 28);
        assert_eq!(
            certificate.affine_construction.decimal_value,
            "30070000000000000000000000000000307003"
        );
        assert_eq!(certificate.residue_rows.len(), 9);
        assert!(certificate.residue_rows.iter().all(|row| row.survived));
        assert_eq!(
            certificate
                .rejection_examples
                .iter()
                .map(|row| (row.seed, row.rejected_by_modulus))
                .collect::<Vec<_>>(),
            vec![(0, 11), (1, 3), (4, 3)]
        );
        let replay = certificate
            .search_replay
            .as_ref()
            .expect("teaching certificate has search replay");
        assert_eq!(replay.input_seed, 0);
        assert_eq!(replay.witness_seed, 3);
        assert!(replay.complete_through_witness);
        assert_eq!(
            replay
                .rows
                .iter()
                .map(|row| (
                    row.seed,
                    row.status.clone(),
                    row.rejected_by_modulus,
                    row.residue_survived,
                    row.probable_prime_result,
                    row.accepted_witness,
                ))
                .collect::<Vec<_>>(),
            vec![
                (
                    0,
                    SearchReplayRowStatus::ResidueRejected,
                    Some(11),
                    false,
                    false,
                    false,
                ),
                (
                    1,
                    SearchReplayRowStatus::ResidueRejected,
                    Some(3),
                    false,
                    false,
                    false,
                ),
                (
                    2,
                    SearchReplayRowStatus::ResidueSurvivorProbablePrimeRejected,
                    None,
                    true,
                    false,
                    false,
                ),
                (
                    3,
                    SearchReplayRowStatus::AcceptedProbablePrimeWitness,
                    None,
                    true,
                    true,
                    true,
                ),
            ]
        );
    }

    #[test]
    fn canonical_witness_manifest_indexes_three_artifacts() {
        let mut artifacts = Vec::new();
        let specs = canonical_proof_carrying_witness_specs();

        for spec in &specs {
            let certificate =
                build_proof_carrying_witness_certificate_for_config(spec.config.clone())
                    .expect("canonical certificate");
            artifacts.push(proof_carrying_witness_manifest_artifact(spec, &certificate));
        }

        let manifest = build_proof_carrying_witness_manifest(artifacts);
        assert_eq!(
            manifest.schema_version,
            PROOF_CARRYING_WITNESS_MANIFEST_SCHEMA_VERSION
        );
        assert_eq!(
            manifest.artifact_set_id,
            PROOF_CARRYING_WITNESS_ARTIFACT_SET_ID
        );
        assert_eq!(manifest.artifacts.len(), 3);
        assert_eq!(
            manifest
                .artifacts
                .iter()
                .map(|artifact| artifact.path.as_str())
                .collect::<Vec<_>>(),
            vec![
                "docs/witness/seed60_proof_carrying_witness.json",
                "docs/witness/teaching38_proof_carrying_witness.json",
                "docs/witness/timestamp_policy_29d_trial0_proof_carrying_witness.json",
            ]
        );
        assert!(manifest
            .artifacts
            .iter()
            .all(|artifact| artifact.rejection_example_count == 3));
        assert_eq!(
            manifest
                .artifacts
                .iter()
                .map(|artifact| artifact.search_replay_row_count)
                .collect::<Vec<_>>(),
            vec![1, 4, 7]
        );
        assert_eq!(
            specs
                .iter()
                .map(proof_carrying_witness_lean_module_name)
                .collect::<Vec<_>>(),
            vec![
                "PrimeArithmetic.Generated.Witness.Seed60",
                "PrimeArithmetic.Generated.Witness.Teaching38",
                "PrimeArithmetic.Generated.Witness.TimestampPolicy29Trial0",
            ]
        );
        assert_eq!(
            specs
                .iter()
                .map(proof_carrying_witness_lean_output_path)
                .collect::<Vec<_>>(),
            vec![
                "lean-proofs/PrimeArithmetic/Generated/Witness/Seed60.lean",
                "lean-proofs/PrimeArithmetic/Generated/Witness/Teaching38.lean",
                "lean-proofs/PrimeArithmetic/Generated/Witness/TimestampPolicy29Trial0.lean",
            ]
        );

        let json = serde_json::to_string_pretty(&manifest).expect("serialize manifest");
        let roundtrip: ProofCarryingWitnessManifest =
            serde_json::from_str(&json).expect("deserialize manifest");
        assert_eq!(roundtrip, manifest);
    }

    #[test]
    fn witness_search_policy_atlas_summarizes_canonical_bundle() {
        let coverage_rows = canonical_proof_carrying_witness_specs()
            .iter()
            .map(|spec| {
                let certificate =
                    build_proof_carrying_witness_certificate_for_config(spec.config.clone())
                        .expect("canonical certificate");
                proof_carrying_witness_search_policy_coverage_row(spec, &certificate)
            })
            .collect::<Vec<_>>();

        let atlas = build_proof_carrying_witness_search_policy_atlas(coverage_rows);
        assert_eq!(
            atlas.schema_version,
            PROOF_CARRYING_WITNESS_SEARCH_POLICY_ATLAS_SCHEMA_VERSION
        );
        assert_eq!(atlas.summary.artifact_count, 3);
        assert_eq!(atlas.summary.lane_count, 1);
        assert_eq!(atlas.summary.seed_origin_policy_count, 3);
        assert_eq!(atlas.summary.visible_digit_count, 3);
        assert_eq!(atlas.summary.total_scanned_seed_count, 12);
        assert_eq!(atlas.summary.total_residue_rejected_count, 6);
        assert_eq!(atlas.summary.total_residue_survivor_count, 6);
        assert_eq!(atlas.summary.max_first_accepted_distance, 6);
        assert_eq!(atlas.summary.max_non_accepted_residue_survivor_count, 2);
        assert!(atlas.summary.all_replays_complete_through_witness);
        assert!(atlas.summary.all_have_first_accepted_survivor_theorem);
        assert_eq!(
            atlas.summary.primality_proof_status,
            PROBABLE_PRIME_NOT_PROOF_CERTIFIED
        );
        assert_eq!(
            atlas
                .coverage_rows
                .iter()
                .map(|row| row.artifact_id.as_str())
                .collect::<Vec<_>>(),
            vec![
                "seed60-canonical-128d",
                "teaching-seed0-38d",
                "timestamp-policy-trial0-29d",
            ]
        );
        let teaching = atlas
            .coverage_rows
            .iter()
            .find(|row| row.artifact_id == "teaching-seed0-38d")
            .expect("teaching coverage row");
        assert_eq!(teaching.seed_origin_policy, "teaching-fixed-seed");
        assert_eq!(teaching.first_accepted_distance, 3);
        assert_eq!(teaching.rejection_geometry, "mod3x1_mod11x1");
        assert_eq!(teaching.non_accepted_residue_survivor_count, 1);
        assert_eq!(teaching.accepted_residue_survivor_count, 1);
        assert_eq!(
            teaching.lean_links.first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.Teaching38.searchReplayFirstAcceptedSurvivor"
        );
        assert_eq!(
            teaching
                .lean_links
                .theorem_wrapper_first_accepted_survivor
                .as_deref(),
            Some(
                "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_search_replay_first_accepted_survivor"
            )
        );
        assert_eq!(
            atlas
                .lane_rows
                .iter()
                .map(|row| (row.lane_id.as_str(), row.artifact_count))
                .collect::<Vec<_>>(),
            vec![("base10_outer3_inner7_k2_1", 3)]
        );
        assert_eq!(
            atlas
                .rejection_modulus_rows
                .iter()
                .map(|row| (row.modulus, row.replay_rejection_count))
                .collect::<Vec<_>>(),
            vec![(3, 3), (7, 1), (11, 1), (13, 0), (17, 1)]
        );

        let markdown = render_proof_carrying_witness_search_policy_atlas_markdown(&atlas);
        assert!(markdown.contains("not a primality proof"));
        assert!(markdown.contains("searchReplayFirstAcceptedSurvivor"));
        let json = serde_json::to_string_pretty(&atlas).expect("serialize atlas");
        let roundtrip: ProofCarryingWitnessSearchPolicyAtlas =
            serde_json::from_str(&json).expect("deserialize atlas");
        assert_eq!(roundtrip, atlas);
    }

    #[test]
    fn witness_policy_matrix_smoke_specs_build_certificate_candidate_report() {
        let rows = proof_carrying_witness_policy_matrix_smoke_specs()
            .iter()
            .map(|spec| {
                let certificate =
                    build_proof_carrying_witness_certificate_for_config(spec.config.clone())
                        .expect("policy matrix certificate");
                proof_carrying_witness_policy_matrix_row(
                    spec,
                    &certificate,
                    format!("certificates/{}", spec.file_name),
                )
            })
            .collect::<Vec<_>>();

        let report = build_proof_carrying_witness_policy_matrix_report(rows);
        assert_eq!(
            report.schema_version,
            PROOF_CARRYING_WITNESS_POLICY_MATRIX_SCHEMA_VERSION
        );
        assert_eq!(report.matrix_id, PROOF_CARRYING_WITNESS_POLICY_MATRIX_ID);
        assert_eq!(report.summary.row_count, 21);
        assert_eq!(report.summary.certificate_count, 21);
        assert_eq!(report.summary.lane_count, 6);
        assert_eq!(report.summary.seed_origin_policy_count, 4);
        assert_eq!(report.summary.visible_digit_count, 7);
        assert_eq!(report.summary.canonical_lean_promoted_count, 3);
        assert_eq!(report.summary.matrix_lean_promoted_count, 18);
        assert_eq!(report.summary.small_lean_candidate_count, 0);
        assert_eq!(report.summary.atlas_only_large_candidate_count, 0);
        assert_eq!(report.summary.max_first_accepted_distance, 179);
        assert!(report.summary.all_rows_found_witness);
        assert_eq!(
            report.summary.primality_proof_status,
            PROBABLE_PRIME_NOT_PROOF_CERTIFIED
        );

        let distance_by_id = report
            .rows
            .iter()
            .map(|row| (row.artifact_id.as_str(), row.first_accepted_distance))
            .collect::<BTreeMap<_, _>>();
        assert_eq!(distance_by_id["matrix-decimal-classic-22d-seed0"], 6);
        assert_eq!(distance_by_id["matrix-decimal-breathing-22d-seed0"], 10);
        assert_eq!(distance_by_id["matrix-decimal-readable-22d-seed0"], 62);
        assert_eq!(distance_by_id["matrix-base6-compact-18d-seed0"], 4);
        assert_eq!(distance_by_id["matrix-base12-compact-18d-seed0"], 5);
        assert_eq!(distance_by_id["matrix-base30-wheel-18d-seed0"], 15);
        assert_eq!(distance_by_id["matrix-decimal-readable-64d-seed0"], 5);
        assert_eq!(distance_by_id["matrix-decimal-classic-64d-seed0"], 48);
        assert_eq!(distance_by_id["matrix-decimal-breathing-64d-seed0"], 23);
        assert_eq!(distance_by_id["matrix-base6-compact-64d-seed0"], 17);
        assert_eq!(distance_by_id["matrix-base12-compact-64d-seed0"], 18);
        assert_eq!(distance_by_id["matrix-base30-wheel-64d-seed0"], 29);
        assert_eq!(distance_by_id["matrix-decimal-readable-96d-seed0"], 32);
        assert_eq!(distance_by_id["matrix-decimal-classic-96d-seed0"], 179);
        assert_eq!(distance_by_id["matrix-decimal-breathing-96d-seed0"], 58);
        assert_eq!(distance_by_id["matrix-base6-compact-96d-seed0"], 117);
        assert_eq!(distance_by_id["matrix-base12-compact-96d-seed0"], 2);
        assert_eq!(distance_by_id["matrix-base30-wheel-96d-seed0"], 137);

        let base6 = report
            .rows
            .iter()
            .find(|row| row.artifact_id == "matrix-base6-compact-18d-seed0")
            .expect("base6 matrix row");
        assert_eq!(base6.lane_id, "base6_outer1_inner5_k0_0");
        assert_eq!(
            base6.lean_promotion_status,
            ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanPolicyMatrix
        );
        let classic64 = report
            .rows
            .iter()
            .find(|row| row.artifact_id == "matrix-decimal-classic-64d-seed0")
            .expect("decimal classic 64d matrix row");
        assert_eq!(
            classic64.lean_promotion_status,
            ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanPolicyMatrix
        );
        let readable64 = report
            .rows
            .iter()
            .find(|row| row.artifact_id == "matrix-decimal-readable-64d-seed0")
            .expect("decimal readable 64d matrix row");
        assert_eq!(
            readable64.lean_promotion_status,
            ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanPolicyMatrix
        );
        let readable96 = report
            .rows
            .iter()
            .find(|row| row.artifact_id == "matrix-decimal-readable-96d-seed0")
            .expect("decimal readable 96d matrix row");
        assert_eq!(
            readable96.lean_promotion_status,
            ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanPolicyMatrix
        );
        let base30_64 = report
            .rows
            .iter()
            .find(|row| row.artifact_id == "matrix-base30-wheel-64d-seed0")
            .expect("base30 64d matrix row");
        assert_eq!(
            base30_64.lean_promotion_status,
            ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanPolicyMatrix
        );
        let breathing64 = report
            .rows
            .iter()
            .find(|row| row.artifact_id == "matrix-decimal-breathing-64d-seed0")
            .expect("decimal breathing 64d matrix row");
        assert_eq!(
            breathing64.lean_promotion_status,
            ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanPolicyMatrix
        );
        let breathing96 = report
            .rows
            .iter()
            .find(|row| row.artifact_id == "matrix-decimal-breathing-96d-seed0")
            .expect("decimal breathing 96d matrix row");
        assert_eq!(
            breathing96.lean_promotion_status,
            ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanPolicyMatrix
        );
        let base12_64 = report
            .rows
            .iter()
            .find(|row| row.artifact_id == "matrix-base12-compact-64d-seed0")
            .expect("base12 64d matrix row");
        assert_eq!(
            base12_64.lean_promotion_status,
            ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanPolicyMatrix
        );
        let base12_96 = report
            .rows
            .iter()
            .find(|row| row.artifact_id == "matrix-base12-compact-96d-seed0")
            .expect("base12 compact 96d matrix row");
        assert_eq!(
            base12_96.lean_promotion_status,
            ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanPolicyMatrix
        );
        let base6_64 = report
            .rows
            .iter()
            .find(|row| row.artifact_id == "matrix-base6-compact-64d-seed0")
            .expect("base6 64d matrix row");
        assert_eq!(
            base6_64.lean_promotion_status,
            ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanPolicyMatrix
        );
        let base6_96 = report
            .rows
            .iter()
            .find(|row| row.artifact_id == "matrix-base6-compact-96d-seed0")
            .expect("base6 compact 96d matrix row");
        assert_eq!(
            base6_96.lean_promotion_status,
            ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanPolicyMatrix
        );
        let classic96 = report
            .rows
            .iter()
            .find(|row| row.artifact_id == "matrix-decimal-classic-96d-seed0")
            .expect("decimal classic 96d matrix row");
        assert_eq!(
            classic96.lean_promotion_status,
            ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanPolicyMatrix
        );
        let base30_96 = report
            .rows
            .iter()
            .find(|row| row.artifact_id == "matrix-base30-wheel-96d-seed0")
            .expect("base30 wheel 96d matrix row");
        assert_eq!(
            base30_96.lean_promotion_status,
            ProofCarryingWitnessPolicyMatrixLeanPromotionStatus::GeneratedLeanPolicyMatrix
        );

        let markdown = render_proof_carrying_witness_policy_matrix_markdown(&report);
        assert!(markdown.contains("Proof-Carrying Witness Policy Matrix"));
        assert!(markdown.contains("generated-lean-policy-matrix"));
        assert!(!markdown.contains("atlas-only-large-candidate"));
        assert!(!markdown.contains("lean-candidate-small-native-decide"));
        let json = serde_json::to_string_pretty(&report).expect("serialize policy matrix");
        let roundtrip: ProofCarryingWitnessPolicyMatrixReport =
            serde_json::from_str(&json).expect("deserialize policy matrix");
        assert_eq!(roundtrip, report);
    }

    #[test]
    fn witness_policy_matrix_atlas_summarizes_replay_coverage_and_next_target() {
        let rows = proof_carrying_witness_policy_matrix_smoke_specs()
            .iter()
            .map(|spec| {
                let certificate =
                    build_proof_carrying_witness_certificate_for_config(spec.config.clone())
                        .expect("policy matrix certificate");
                proof_carrying_witness_policy_matrix_row(
                    spec,
                    &certificate,
                    format!("certificates/{}", spec.file_name),
                )
            })
            .collect::<Vec<_>>();
        let report = build_proof_carrying_witness_policy_matrix_report(rows);
        let atlas = build_proof_carrying_witness_policy_matrix_atlas(&report);

        assert_eq!(
            atlas.schema_version,
            PROOF_CARRYING_WITNESS_POLICY_MATRIX_ATLAS_SCHEMA_VERSION
        );
        assert_eq!(atlas.matrix_id, PROOF_CARRYING_WITNESS_POLICY_MATRIX_ID);
        assert_eq!(atlas.summary.row_count, 21);
        assert_eq!(atlas.summary.lane_count, 6);
        assert_eq!(atlas.summary.promoted_replay_certified_count, 21);
        assert_eq!(atlas.summary.unpromoted_replay_candidate_count, 0);
        assert_eq!(atlas.summary.atlas_only_large_candidate_count, 0);
        assert_eq!(atlas.summary.canonical_lean_promoted_count, 3);
        assert_eq!(atlas.summary.matrix_lean_promoted_count, 18);
        assert_eq!(atlas.summary.promoted_large_replay_geometry_count, 12);
        assert_eq!(atlas.summary.max_first_accepted_distance, 179);
        assert!(atlas.summary.all_promoted_have_lean_replay_links);
        assert_eq!(
            atlas.next_replay_target.status,
            ProofCarryingWitnessPolicyMatrixNextReplayTargetStatus::NoneCurrentSmokeMatrixFullyCovered
        );
        assert_eq!(atlas.next_replay_target.artifact_id, None);

        let readable_lane = atlas
            .lane_rows
            .iter()
            .find(|row| row.lane_label == "decimal-readable-k21")
            .expect("decimal readable lane row");
        assert_eq!(readable_lane.artifact_count, 6);
        assert_eq!(readable_lane.promoted_replay_certified_count, 6);
        assert_eq!(readable_lane.max_first_accepted_distance, 62);

        let readable = atlas
            .coverage_rows
            .iter()
            .find(|row| row.artifact_id == "matrix-decimal-readable-22d-seed0")
            .expect("decimal readable coverage row");
        assert_eq!(
            readable.lean_replay_coverage,
            ProofCarryingWitnessPolicyMatrixLeanReplayCoverage::LeanReplayCertified
        );
        assert_eq!(
            readable
                .lean_links
                .as_ref()
                .expect("readable Lean links")
                .first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixDecimalReadable22.searchReplayFirstAcceptedSurvivor"
        );
        let readable64 = atlas
            .coverage_rows
            .iter()
            .find(|row| row.artifact_id == "matrix-decimal-readable-64d-seed0")
            .expect("decimal readable 64d coverage row");
        assert_eq!(
            readable64
                .lean_links
                .as_ref()
                .expect("readable64 Lean links")
                .first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixDecimalReadable64.searchReplayFirstAcceptedSurvivor"
        );
        let readable96 = atlas
            .coverage_rows
            .iter()
            .find(|row| row.artifact_id == "matrix-decimal-readable-96d-seed0")
            .expect("decimal readable 96d coverage row");
        assert_eq!(
            readable96
                .lean_links
                .as_ref()
                .expect("readable96 Lean links")
                .first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixDecimalReadable96.searchReplayFirstAcceptedSurvivor"
        );
        let classic64 = atlas
            .coverage_rows
            .iter()
            .find(|row| row.artifact_id == "matrix-decimal-classic-64d-seed0")
            .expect("decimal classic 64d coverage row");
        assert_eq!(
            classic64
                .lean_links
                .as_ref()
                .expect("classic64 Lean links")
                .first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixDecimalClassic64.searchReplayFirstAcceptedSurvivor"
        );
        let base30_64 = atlas
            .coverage_rows
            .iter()
            .find(|row| row.artifact_id == "matrix-base30-wheel-64d-seed0")
            .expect("base30 wheel 64d coverage row");
        assert_eq!(
            base30_64
                .lean_links
                .as_ref()
                .expect("base30 64d Lean links")
                .first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixBase30Wheel64.searchReplayFirstAcceptedSurvivor"
        );
        let breathing64 = atlas
            .coverage_rows
            .iter()
            .find(|row| row.artifact_id == "matrix-decimal-breathing-64d-seed0")
            .expect("decimal breathing 64d coverage row");
        assert_eq!(
            breathing64
                .lean_links
                .as_ref()
                .expect("breathing64 Lean links")
                .first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing64.searchReplayFirstAcceptedSurvivor"
        );
        let breathing96 = atlas
            .coverage_rows
            .iter()
            .find(|row| row.artifact_id == "matrix-decimal-breathing-96d-seed0")
            .expect("decimal breathing 96d coverage row");
        assert_eq!(
            breathing96
                .lean_links
                .as_ref()
                .expect("breathing96 Lean links")
                .first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing96.searchReplayFirstAcceptedSurvivor"
        );
        let base12_64 = atlas
            .coverage_rows
            .iter()
            .find(|row| row.artifact_id == "matrix-base12-compact-64d-seed0")
            .expect("base12 compact 64d coverage row");
        assert_eq!(
            base12_64
                .lean_links
                .as_ref()
                .expect("base12 64d Lean links")
                .first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixBase12Compact64.searchReplayFirstAcceptedSurvivor"
        );
        let base12_96 = atlas
            .coverage_rows
            .iter()
            .find(|row| row.artifact_id == "matrix-base12-compact-96d-seed0")
            .expect("base12 compact 96d coverage row");
        assert_eq!(
            base12_96
                .lean_links
                .as_ref()
                .expect("base12 96d Lean links")
                .first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixBase12Compact96.searchReplayFirstAcceptedSurvivor"
        );
        let base6_64 = atlas
            .coverage_rows
            .iter()
            .find(|row| row.artifact_id == "matrix-base6-compact-64d-seed0")
            .expect("base6 compact 64d coverage row");
        assert_eq!(
            base6_64
                .lean_links
                .as_ref()
                .expect("base6 64d Lean links")
                .first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixBase6Compact64.searchReplayFirstAcceptedSurvivor"
        );
        let base6_96 = atlas
            .coverage_rows
            .iter()
            .find(|row| row.artifact_id == "matrix-base6-compact-96d-seed0")
            .expect("base6 compact 96d coverage row");
        assert_eq!(
            base6_96
                .lean_links
                .as_ref()
                .expect("base6 96d Lean links")
                .first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixBase6Compact96.searchReplayFirstAcceptedSurvivor"
        );
        let classic96 = atlas
            .coverage_rows
            .iter()
            .find(|row| row.artifact_id == "matrix-decimal-classic-96d-seed0")
            .expect("decimal classic 96d coverage row");
        assert_eq!(
            classic96
                .lean_links
                .as_ref()
                .expect("classic96 Lean links")
                .first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixDecimalClassic96.searchReplayFirstAcceptedSurvivor"
        );
        let base30_96 = atlas
            .coverage_rows
            .iter()
            .find(|row| row.artifact_id == "matrix-base30-wheel-96d-seed0")
            .expect("base30 wheel 96d coverage row");
        assert_eq!(
            base30_96
                .lean_links
                .as_ref()
                .expect("base30 96d Lean links")
                .first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixBase30Wheel96.searchReplayFirstAcceptedSurvivor"
        );
        assert_eq!(
            atlas
                .promoted_large_replay_geometry_rows
                .iter()
                .map(|row| (
                    row.rank,
                    row.artifact_id.as_str(),
                    row.first_accepted_distance
                ))
                .collect::<Vec<_>>(),
            vec![
                (1, "matrix-decimal-classic-96d-seed0", 179),
                (2, "matrix-base30-wheel-96d-seed0", 137),
                (3, "matrix-base6-compact-96d-seed0", 117),
                (4, "matrix-decimal-breathing-96d-seed0", 58),
                (5, "matrix-decimal-classic-64d-seed0", 48),
                (6, "matrix-decimal-readable-96d-seed0", 32),
                (7, "matrix-base30-wheel-64d-seed0", 29),
                (8, "matrix-decimal-breathing-64d-seed0", 23),
                (9, "matrix-base12-compact-64d-seed0", 18),
                (10, "matrix-base6-compact-64d-seed0", 17),
                (11, "matrix-decimal-readable-64d-seed0", 5),
                (12, "matrix-base12-compact-96d-seed0", 2),
            ]
        );

        let markdown = render_proof_carrying_witness_policy_matrix_atlas_markdown(&atlas);
        assert!(markdown.contains("Proof-Carrying Witness Policy-Matrix Atlas"));
        assert!(markdown.contains("matrix-base30-wheel-64d-seed0"));
        assert!(markdown.contains("Promoted Large Replay Geometry"));
        assert!(markdown.contains("MatrixDecimalReadable64.searchReplayAccountingExact"));
        assert!(markdown.contains("MatrixDecimalReadable96.searchReplayAccountingExact"));
        assert!(markdown.contains("MatrixDecimalClassic96.searchReplayAccountingExact"));
        assert!(markdown.contains("MatrixDecimalBreathing64.searchReplayAccountingExact"));
        assert!(markdown.contains("MatrixDecimalBreathing96.searchReplayAccountingExact"));
        assert!(markdown.contains("MatrixBase12Compact64.searchReplayAccountingExact"));
        assert!(markdown.contains("MatrixBase12Compact96.searchReplayAccountingExact"));
        assert!(markdown.contains("MatrixBase6Compact64.searchReplayAccountingExact"));
        assert!(markdown.contains("MatrixBase6Compact96.searchReplayAccountingExact"));
        assert!(markdown.contains("MatrixBase30Wheel96.searchReplayAccountingExact"));
        assert!(markdown.contains("lean-replay-certified"));
        assert!(markdown.contains("not a primality proof"));

        let json = serde_json::to_string_pretty(&atlas).expect("serialize policy matrix atlas");
        let roundtrip: ProofCarryingWitnessPolicyMatrixAtlas =
            serde_json::from_str(&json).expect("deserialize policy matrix atlas");
        assert_eq!(roundtrip, atlas);
    }

    #[test]
    fn witness_lean_catalog_manifest_maps_artifacts_to_theorems() {
        let specs = canonical_proof_carrying_witness_specs();
        let artifacts = specs
            .iter()
            .map(|spec| {
                let certificate =
                    build_proof_carrying_witness_certificate_for_config(spec.config.clone())
                        .expect("canonical certificate");
                proof_carrying_witness_lean_catalog_artifact(spec, &certificate)
            })
            .collect::<Vec<_>>();

        let manifest = build_proof_carrying_witness_lean_catalog_manifest(artifacts);
        assert_eq!(
            manifest.schema_version,
            PROOF_CARRYING_WITNESS_LEAN_CATALOG_SCHEMA_VERSION
        );
        assert_eq!(manifest.artifacts.len(), 3);
        assert_eq!(
            manifest.claim_status,
            ProofCarryingWitnessLeanCatalogClaimStatus::ConstructionAndResidueOnly
        );

        let seed60 = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.artifact_id == "seed60-canonical-128d")
            .expect("seed60 artifact row");
        assert_eq!(
            seed60.generated_lean_module,
            "PrimeArithmetic.Generated.Witness.Seed60"
        );
        assert_eq!(seed60.residue_theorem_names.len(), 9);
        assert_eq!(seed60.rejection_theorem_names.len(), 3);
        assert_eq!(seed60.search_replay_theorem_names.len(), 1);
        assert_eq!(
            seed60.theorem_names.residue_funnel_survives,
            "PrimeArithmetic.Generated.Witness.Seed60.residueFunnelSurvives"
        );
        assert!(seed60.theorem_wrapper.is_none());

        let teaching = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.artifact_id == "teaching-seed0-38d")
            .expect("teaching artifact row");
        assert_eq!(teaching.search_replay_theorem_names.len(), 4);
        assert!(teaching
            .search_replay_theorem_names
            .iter()
            .any(|row| row.rejection.as_deref()
                == Some("PrimeArithmetic.Generated.Witness.Teaching38.search_replay_seed0_rejected_mod11")));
        assert!(teaching.search_replay_theorem_names.iter().any(|row| row
            .survives_residue_funnel
            .as_deref()
            == Some("PrimeArithmetic.Generated.Witness.Teaching38.search_replay_seed3_survives")));
        assert!(teaching.theorem_wrapper.is_some());
        assert!(teaching
            .theorem_wrapper
            .as_ref()
            .expect("wrapper")
            .theorem_names
            .contains(&"PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_residue_funnel_survives".to_string()));

        let json = serde_json::to_string_pretty(&manifest).expect("serialize Lean catalog");
        assert!(json.contains("proof-carrying-witness-lean-catalog-v1"));
        assert!(json.contains("construction-and-residue-only"));
        let roundtrip: ProofCarryingWitnessLeanCatalogManifest =
            serde_json::from_str(&json).expect("deserialize Lean catalog");
        assert_eq!(roundtrip, manifest);
    }

    #[test]
    fn witness_policy_matrix_lean_catalog_manifest_maps_promoted_artifacts_to_theorems() {
        let specs = proof_carrying_witness_policy_matrix_promoted_specs();
        assert_eq!(specs.len(), 18);

        let artifacts = specs
            .iter()
            .map(|spec| {
                let certificate =
                    build_proof_carrying_witness_certificate_for_config(spec.config.clone())
                        .expect("policy-matrix certificate");
                proof_carrying_witness_policy_matrix_lean_catalog_artifact(spec, &certificate)
                    .expect("promoted policy-matrix Lean artifact")
            })
            .collect::<Vec<_>>();

        let manifest = build_proof_carrying_witness_policy_matrix_lean_catalog_manifest(artifacts);
        assert_eq!(
            manifest.schema_version,
            PROOF_CARRYING_WITNESS_LEAN_CATALOG_SCHEMA_VERSION
        );
        assert_eq!(
            manifest.artifact_set_id,
            PROOF_CARRYING_WITNESS_POLICY_MATRIX_ID
        );
        assert_eq!(manifest.witness_manifest_path, "docs/witness/policy_matrix");
        assert_eq!(manifest.artifacts.len(), 18);
        assert!(manifest
            .artifacts
            .iter()
            .all(|artifact| artifact.theorem_wrapper.is_none()));

        let readable = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.artifact_id == "matrix-decimal-readable-22d-seed0")
            .expect("decimal readable matrix artifact row");
        assert_eq!(
            readable.generated_lean_module,
            "PrimeArithmetic.Generated.Witness.MatrixDecimalReadable22"
        );
        assert_eq!(
            readable.theorem_names.search_replay_first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixDecimalReadable22.searchReplayFirstAcceptedSurvivor"
        );
        let readable96 = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.artifact_id == "matrix-decimal-readable-96d-seed0")
            .expect("readable96 matrix artifact row");
        assert!(readable96.search_replay_theorem_names.is_empty());
        assert_eq!(
            readable96.theorem_names.search_replay_first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixDecimalReadable96.searchReplayFirstAcceptedSurvivor"
        );

        let base6 = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.artifact_id == "matrix-base6-compact-18d-seed0")
            .expect("base6 matrix artifact row");
        assert_eq!(
            base6.generated_lean_path,
            "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixBase6Compact18.lean"
        );
        let classic64 = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.artifact_id == "matrix-decimal-classic-64d-seed0")
            .expect("classic64 matrix artifact row");
        assert!(classic64.search_replay_theorem_names.is_empty());
        assert_eq!(
            classic64.theorem_names.search_replay_first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixDecimalClassic64.searchReplayFirstAcceptedSurvivor"
        );
        let classic96 = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.artifact_id == "matrix-decimal-classic-96d-seed0")
            .expect("classic96 matrix artifact row");
        assert!(classic96.search_replay_theorem_names.is_empty());
        assert_eq!(
            classic96.theorem_names.search_replay_first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixDecimalClassic96.searchReplayFirstAcceptedSurvivor"
        );
        let base30_96 = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.artifact_id == "matrix-base30-wheel-96d-seed0")
            .expect("base30 96d matrix artifact row");
        assert!(base30_96.search_replay_theorem_names.is_empty());
        assert_eq!(
            base30_96.theorem_names.search_replay_first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixBase30Wheel96.searchReplayFirstAcceptedSurvivor"
        );
        let base6_96 = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.artifact_id == "matrix-base6-compact-96d-seed0")
            .expect("base6 96d matrix artifact row");
        assert!(base6_96.search_replay_theorem_names.is_empty());
        assert_eq!(
            base6_96.theorem_names.search_replay_first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixBase6Compact96.searchReplayFirstAcceptedSurvivor"
        );
        let breathing96 = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.artifact_id == "matrix-decimal-breathing-96d-seed0")
            .expect("breathing96 matrix artifact row");
        assert!(breathing96.search_replay_theorem_names.is_empty());
        assert_eq!(
            breathing96.theorem_names.search_replay_first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing96.searchReplayFirstAcceptedSurvivor"
        );
        let readable64 = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.artifact_id == "matrix-decimal-readable-64d-seed0")
            .expect("readable64 matrix artifact row");
        assert!(readable64.search_replay_theorem_names.is_empty());
        assert_eq!(
            readable64.theorem_names.search_replay_first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixDecimalReadable64.searchReplayFirstAcceptedSurvivor"
        );
        let base30_64 = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.artifact_id == "matrix-base30-wheel-64d-seed0")
            .expect("base30 64d matrix artifact row");
        assert!(base30_64.search_replay_theorem_names.is_empty());
        assert_eq!(
            base30_64.generated_lean_module,
            "PrimeArithmetic.Generated.Witness.MatrixBase30Wheel64"
        );
        let breathing64 = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.artifact_id == "matrix-decimal-breathing-64d-seed0")
            .expect("breathing64 matrix artifact row");
        assert!(breathing64.search_replay_theorem_names.is_empty());
        assert_eq!(
            breathing64.generated_lean_module,
            "PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing64"
        );
        let base12_64 = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.artifact_id == "matrix-base12-compact-64d-seed0")
            .expect("base12 64d matrix artifact row");
        assert!(base12_64.search_replay_theorem_names.is_empty());
        assert_eq!(
            base12_64.generated_lean_module,
            "PrimeArithmetic.Generated.Witness.MatrixBase12Compact64"
        );
        let base12_96 = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.artifact_id == "matrix-base12-compact-96d-seed0")
            .expect("base12 96d matrix artifact row");
        assert!(base12_96.search_replay_theorem_names.is_empty());
        assert_eq!(
            base12_96.theorem_names.search_replay_first_accepted_survivor,
            "PrimeArithmetic.Generated.Witness.MatrixBase12Compact96.searchReplayFirstAcceptedSurvivor"
        );
        let base6_64 = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.artifact_id == "matrix-base6-compact-64d-seed0")
            .expect("base6 64d matrix artifact row");
        assert!(base6_64.search_replay_theorem_names.is_empty());
        assert_eq!(
            base6_64.generated_lean_module,
            "PrimeArithmetic.Generated.Witness.MatrixBase6Compact64"
        );

        let checks = render_proof_carrying_witness_lean_catalog_checks(&manifest);
        assert!(checks.contains("import PrimeArithmetic.Generated.Witness.MatrixDecimalReadable22"));
        assert!(checks.contains("import PrimeArithmetic.Generated.Witness.MatrixDecimalReadable64"));
        assert!(checks.contains("import PrimeArithmetic.Generated.Witness.MatrixDecimalReadable96"));
        assert!(checks.contains("import PrimeArithmetic.Generated.Witness.MatrixDecimalClassic64"));
        assert!(checks.contains("import PrimeArithmetic.Generated.Witness.MatrixDecimalClassic96"));
        assert!(
            checks.contains("import PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing64")
        );
        assert!(
            checks.contains("import PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing96")
        );
        assert!(checks.contains("import PrimeArithmetic.Generated.Witness.MatrixBase30Wheel64"));
        assert!(checks.contains("import PrimeArithmetic.Generated.Witness.MatrixBase30Wheel96"));
        assert!(checks.contains("import PrimeArithmetic.Generated.Witness.MatrixBase6Compact64"));
        assert!(checks.contains("import PrimeArithmetic.Generated.Witness.MatrixBase6Compact96"));
        assert!(checks.contains("import PrimeArithmetic.Generated.Witness.MatrixBase12Compact64"));
        assert!(checks.contains("import PrimeArithmetic.Generated.Witness.MatrixBase12Compact96"));
        assert!(checks.contains("import PrimeArithmetic.Generated.Witness.MatrixBase30Wheel18"));
        assert!(checks.contains(
            "  have _ := @PrimeArithmetic.Generated.Witness.MatrixDecimalClassic22.searchReplayFirstAcceptedSurvivor"
        ));
        assert!(!checks
            .contains("PrimeArithmetic.Generated.Witness.MatrixBase30Wheel64.search_replay_seed"));
        assert!(!checks
            .contains("PrimeArithmetic.Generated.Witness.MatrixBase30Wheel96.search_replay_seed"));
        assert!(!checks.contains(
            "PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing64.search_replay_seed"
        ));
        assert!(!checks.contains(
            "PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing96.search_replay_seed"
        ));
        assert!(!checks.contains(
            "PrimeArithmetic.Generated.Witness.MatrixBase12Compact64.search_replay_seed"
        ));
        assert!(!checks.contains(
            "PrimeArithmetic.Generated.Witness.MatrixBase12Compact96.search_replay_seed"
        ));
        assert!(!checks
            .contains("PrimeArithmetic.Generated.Witness.MatrixBase6Compact64.search_replay_seed"));
        assert!(!checks
            .contains("PrimeArithmetic.Generated.Witness.MatrixBase6Compact96.search_replay_seed"));
        assert!(!checks.contains(
            "PrimeArithmetic.Generated.Witness.MatrixDecimalReadable64.search_replay_seed"
        ));
        assert!(!checks.contains(
            "PrimeArithmetic.Generated.Witness.MatrixDecimalReadable96.search_replay_seed"
        ));
        assert!(!checks.contains(
            "PrimeArithmetic.Generated.Witness.MatrixDecimalClassic96.search_replay_seed"
        ));
        assert!(!checks.contains("PrimeArithmetic.Witness.TeachingSeedCertificate"));

        let json =
            serde_json::to_string_pretty(&manifest).expect("serialize policy-matrix Lean catalog");
        assert!(json.contains("proof-carrying-witness-policy-matrix-smoke-v1"));
        assert!(json.contains("MatrixDecimalBreathing22"));
        assert!(json.contains("MatrixDecimalReadable64"));
        assert!(json.contains("MatrixDecimalReadable96"));
        assert!(json.contains("MatrixDecimalClassic64"));
        assert!(json.contains("MatrixDecimalClassic96"));
        assert!(json.contains("MatrixDecimalBreathing64"));
        assert!(json.contains("MatrixDecimalBreathing96"));
        assert!(json.contains("MatrixBase30Wheel64"));
        assert!(json.contains("MatrixBase30Wheel96"));
        assert!(json.contains("MatrixBase6Compact64"));
        assert!(json.contains("MatrixBase6Compact96"));
        assert!(json.contains("MatrixBase12Compact64"));
        assert!(json.contains("MatrixBase12Compact96"));
        let roundtrip: ProofCarryingWitnessLeanCatalogManifest =
            serde_json::from_str(&json).expect("deserialize policy-matrix Lean catalog");
        assert_eq!(roundtrip, manifest);
    }

    #[test]
    fn witness_lean_catalog_checks_cover_generated_and_wrapper_theorems() {
        let specs = canonical_proof_carrying_witness_specs();
        let artifacts = specs
            .iter()
            .map(|spec| {
                let certificate =
                    build_proof_carrying_witness_certificate_for_config(spec.config.clone())
                        .expect("canonical certificate");
                proof_carrying_witness_lean_catalog_artifact(spec, &certificate)
            })
            .collect::<Vec<_>>();
        let manifest = build_proof_carrying_witness_lean_catalog_manifest(artifacts);
        let checks = render_proof_carrying_witness_lean_catalog_checks(&manifest);

        assert!(checks.contains("import PrimeArithmetic.Generated.Witness.Seed60"));
        assert!(checks.contains("import PrimeArithmetic.Generated.Witness.Teaching38"));
        assert!(checks.contains("import PrimeArithmetic.Generated.Witness.TimestampPolicy29Trial0"));
        assert!(checks.contains("import PrimeArithmetic.Witness.TeachingSeedCertificate"));
        assert!(checks.contains(
            "  have _ := @PrimeArithmetic.Generated.Witness.Seed60.residueFunnelSurvives"
        ));
        assert!(checks.contains(
            "  have _ := @PrimeArithmetic.Generated.Witness.TimestampPolicy29Trial0.rejectionExamplesReject"
        ));
        assert!(checks.contains(
            "  have _ := @PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_residue_funnel_survives"
        ));
        assert!(checks.contains(
            "  have _ := @PrimeArithmetic.Generated.Witness.Teaching38.searchReplayResidueSurvivorsSurvive"
        ));
        assert!(checks.contains(
            "  have _ := @PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_search_replay_survivors_survive"
        ));
        assert!(checks.contains(
            "  have _ := @PrimeArithmetic.Generated.Witness.Seed60.searchReplayCertificate"
        ));
        assert!(checks.contains(
            "  have _ := @PrimeArithmetic.Generated.Witness.TimestampPolicy29Trial0.searchReplaySurvivorListExact"
        ));
        assert!(checks.contains(
            "  have _ := @PrimeArithmetic.Generated.Witness.Teaching38.searchReplayAccountingExact"
        ));
        assert!(checks.contains(
            "  have _ := @PrimeArithmetic.Generated.Witness.TimestampPolicy29Trial0.searchReplayCountExact"
        ));
        assert!(checks.contains(
            "  have _ := @PrimeArithmetic.Generated.Witness.Teaching38.searchReplayFirstAcceptedSurvivor"
        ));
        assert!(checks.contains(
            "  have _ := @PrimeArithmetic.Generated.Witness.TimestampPolicy29Trial0.searchReplayPreWitnessSurvivorsNonAccepted"
        ));
        assert!(checks.contains(
            "  have _ := @PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_search_replay_survivor_list_exact"
        ));
        assert!(checks.contains(
            "  have _ := @PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_search_replay_first_accepted_survivor"
        ));
        assert_eq!(checks.matches("example : True := by").count(), 186);
    }

    #[test]
    fn teaching_certificate_lean_renderer_exposes_aggregate_survival_theorem() {
        let certificate = build_proof_carrying_witness_certificate_for_config(
            SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(38)
                .with_max_steps(100),
        )
        .expect("teaching witness certificate");
        let lean = render_proof_carrying_witness_lean_module(
            &certificate,
            "PrimeArithmetic.Generated.Witness.Teaching38",
            "docs/witness/teaching38_proof_carrying_witness.json",
            "cargo run --bin export_proof_carrying_witness_lean_certificate -- --certificate docs/witness/teaching38_proof_carrying_witness.json --out lean-proofs/PrimeArithmetic/Generated/Witness/Teaching38.lean",
        )
        .expect("render Lean module");

        assert!(lean.contains("namespace PrimeArithmetic.Generated.Witness.Teaching38"));
        assert!(lean.contains("def residueModuli : List ℕ := ["));
        assert!(lean.contains("theorem residueFunnelAffineChecks"));
        assert!(lean.contains("theorem residueFunnelSurvives"));
        assert!(lean.contains("theorem rejectionExamplesReject"));
        assert!(lean.contains("def searchReplaySeeds : List ℕ :="));
        assert!(lean.contains("PrimeArithmetic.Witness.contiguousReplaySeeds"));
        assert!(lean.contains("theorem searchReplaySeeds_mem_iff"));
        assert!(lean.contains("theorem searchReplayResidueRejectionsReject"));
        assert!(lean.contains("theorem searchReplayResidueSurvivorsSurvive"));
        assert!(lean.contains("theorem searchReplayPartitionExact"));
        assert!(lean.contains("theorem searchReplayCountExact"));
        assert!(lean.contains("theorem searchReplayAccountingExact"));
        assert!(lean.contains("theorem search_replay_seed0_rejected_mod11"));
        assert!(lean.contains("theorem search_replay_seed3_survives"));
        assert!(lean.contains("theorem residue_row_mod11"));
        assert!(lean.contains("theorem rejection_seed0_mod11"));
        assert!(lean.contains("templateValue config witnessSeed % modulus ≠ 0"));
    }

    #[test]
    fn exact_seed_only_succeeds_or_fails_cleanly() {
        let success = find_seed_to_witness(
            SeedToWitnessConfig::default_for_seed(60).with_exact_seed_only(true),
        )
        .expect("exact witness");
        assert_eq!(success.witness_seed, 60);

        let failure = find_seed_to_witness(
            SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(38)
                .with_exact_seed_only(true),
        )
        .expect_err("exact seed is not a witness");
        assert!(matches!(
            failure,
            SeedToWitnessError::NoWitnessFound {
                exact_seed_only: true,
                ..
            }
        ));
    }

    #[test]
    fn transcript_contains_local_and_external_verification_hooks() {
        let result =
            find_seed_to_witness(SeedToWitnessConfig::default_for_seed(60)).expect("witness");
        let transcript = render_seed_to_witness_transcript(&result);
        assert!(transcript.contains("N(s) ="));
        assert!(transcript.contains(&result.decimal_value));
        assert!(transcript.contains("probable_prime_fixed_20_bases"));
        assert!(transcript.contains("Mersenne class"));
        assert!(transcript.contains("not_mersenne"));
        assert!(transcript.contains("WolframAlpha"));
        assert!(transcript.contains("PrimeQ["));
        assert!(transcript.contains("PARI/GP"));
        assert!(transcript.contains("Sage"));
    }

    #[test]
    fn seed_sixty_certificate_is_deterministic_and_schema_versioned() {
        let result =
            find_seed_to_witness(SeedToWitnessConfig::default_for_seed(60)).expect("witness");
        let certificate = build_proof_carrying_witness_certificate(&result, PROBABLE_PRIME_BASES);
        let again = build_proof_carrying_witness_certificate(&result, PROBABLE_PRIME_BASES);

        assert_eq!(
            certificate.schema_version,
            PROOF_CARRYING_WITNESS_SCHEMA_VERSION
        );
        assert_eq!(certificate, again);
        assert_eq!(certificate.settings.input_seed, 60);
        assert_eq!(certificate.witness.witness_seed, 60);
        assert_eq!(
            certificate.confirmation.primality_proof_status,
            PROBABLE_PRIME_NOT_PROOF_CERTIFIED
        );
    }

    #[test]
    fn seed_sixty_certificate_residue_rows_are_exact_survivor_checks() {
        let result =
            find_seed_to_witness(SeedToWitnessConfig::default_for_seed(60)).expect("witness");
        let certificate = build_proof_carrying_witness_certificate(&result, PROBABLE_PRIME_BASES);

        assert!(!certificate.residue_rows.is_empty());
        for row in &certificate.residue_rows {
            assert!(row.coprime_to_base);
            assert!(row.survived);
            assert!(row.affine_residue_check);
            assert_eq!(
                row.value_mod,
                ((u64::from(row.shift_mod) + u64::from(row.gradient_mod) * u64::from(row.seed_mod))
                    % u64::from(row.modulus)) as u32
            );
        }
    }

    #[test]
    fn seed_sixty_certificate_carries_rejection_examples() {
        let result =
            find_seed_to_witness(SeedToWitnessConfig::default_for_seed(60)).expect("witness");
        let certificate = build_proof_carrying_witness_certificate(&result, PROBABLE_PRIME_BASES);

        assert_eq!(certificate.rejection_examples.len(), 3);
        for row in &certificate.rejection_examples {
            assert_ne!(row.seed, certificate.witness.witness_seed);
            assert_eq!(row.value_mod, 0);
            assert!(row.rejected);
            assert!(row.affine_residue_check);
        }
    }

    #[test]
    fn seed_sixty_certificate_carries_search_replay() {
        let result =
            find_seed_to_witness(SeedToWitnessConfig::default_for_seed(60)).expect("witness");
        let certificate = build_proof_carrying_witness_certificate(&result, PROBABLE_PRIME_BASES);
        let replay = certificate.search_replay.as_ref().expect("search replay");

        assert_eq!(replay.rows.len(), 1);
        assert_eq!(replay.rows[0].seed, 60);
        assert_eq!(
            replay.rows[0].status,
            SearchReplayRowStatus::AcceptedProbablePrimeWitness
        );
        assert!(replay.rows[0].accepted_witness);
        assert!(replay.rows[0].residue_survived);
        assert!(replay.rows[0].probable_prime_result);
    }

    #[test]
    fn seed_sixty_certificate_construction_checks_are_true() {
        let result =
            find_seed_to_witness(SeedToWitnessConfig::default_for_seed(60)).expect("witness");
        let certificate = build_proof_carrying_witness_certificate(&result, PROBABLE_PRIME_BASES);
        let construction = &certificate.affine_construction;

        assert!(construction.affine_value_matches_decimal);
        assert!(construction.template_digits_match_result);
        assert!(construction.middle_digits_match_result);
        assert!(construction.visible_digit_count_matches_template);
        assert!(construction.decimal_digit_count_matches_value);
        assert!(certificate.confirmation.probable_prime_result);
        assert!(certificate.shape.exact_not_mersenne);
    }

    #[test]
    fn seed_sixty_certificate_json_round_trips_without_elapsed_time() {
        let result =
            find_seed_to_witness(SeedToWitnessConfig::default_for_seed(60)).expect("witness");
        let certificate = build_proof_carrying_witness_certificate(&result, PROBABLE_PRIME_BASES);
        let json = serde_json::to_string_pretty(&certificate).expect("serialize certificate");

        assert!(!json.contains("elapsed_seconds"));
        let roundtrip: ProofCarryingWitnessCertificate =
            serde_json::from_str(&json).expect("deserialize certificate");
        assert_eq!(roundtrip, certificate);
    }

    #[test]
    fn seed_sixty_certificate_verifier_accepts_canonical_certificate() {
        let result =
            find_seed_to_witness(SeedToWitnessConfig::default_for_seed(60)).expect("witness");
        let certificate = build_proof_carrying_witness_certificate(&result, PROBABLE_PRIME_BASES);
        let report = verify_proof_carrying_witness_certificate(&certificate);

        assert!(report.ok);
        assert!(report.failures.is_empty());
        assert_eq!(
            report.checked_residue_row_count,
            certificate.residue_rows.len()
        );
        assert_eq!(report.witness_seed, 60);
    }

    #[test]
    fn seed_sixty_certificate_verifier_rejects_corrupt_residue_row() {
        let result =
            find_seed_to_witness(SeedToWitnessConfig::default_for_seed(60)).expect("witness");
        let mut certificate =
            build_proof_carrying_witness_certificate(&result, PROBABLE_PRIME_BASES);
        certificate.residue_rows[0].value_mod = 0;
        certificate.residue_rows[0].survived = false;

        let report = verify_proof_carrying_witness_certificate(&certificate);

        assert!(!report.ok);
        assert!(report
            .failures
            .iter()
            .any(|failure| failure.contains("residue_rows[0]")));
    }

    #[test]
    fn seed_sixty_certificate_verifier_rejects_corrupt_rejection_example() {
        let result =
            find_seed_to_witness(SeedToWitnessConfig::default_for_seed(60)).expect("witness");
        let mut certificate =
            build_proof_carrying_witness_certificate(&result, PROBABLE_PRIME_BASES);
        certificate.rejection_examples[0].value_mod = 1;
        certificate.rejection_examples[0].rejected = false;

        let report = verify_proof_carrying_witness_certificate(&certificate);

        assert!(!report.ok);
        assert!(report
            .failures
            .iter()
            .any(|failure| failure.contains("rejection_examples[0]")));
    }

    #[test]
    fn seed_sixty_certificate_verifier_rejects_corrupt_search_replay() {
        let result =
            find_seed_to_witness(SeedToWitnessConfig::default_for_seed(60)).expect("witness");
        let mut certificate =
            build_proof_carrying_witness_certificate(&result, PROBABLE_PRIME_BASES);
        certificate
            .search_replay
            .as_mut()
            .expect("search replay")
            .rows[0]
            .accepted_witness = false;

        let report = verify_proof_carrying_witness_certificate(&certificate);

        assert!(!report.ok);
        assert!(report
            .failures
            .iter()
            .any(|failure| failure.contains("search_replay")));
    }
}
