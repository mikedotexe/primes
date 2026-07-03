//! Deterministic connector signal atlas builders.
//!
//! This module keeps the maintained connector report, archival atlas, and
//! future top-level signal catalog on one shared data surface.
#![cfg_attr(clippy, allow(dead_code, unused_imports))]

use crate::connector::{
    canonical_source_hits, scan_single_digit_hits, small_primes_up_to, ConcatenationSystem,
    ConnectorCandidate, Direction, DirectionScanStats, DirectionSignalStats, DirectionalAsymmetry,
    PairScanSummary, PairSignalAudit, PositionSignalRow, CANONICAL_DOCUMENTED_FORWARD_HITS,
    CANONICAL_LEFT, CANONICAL_RIGHT, CANONICAL_WIDTH5_HITS,
};
use crate::validation::reporting::export_timestamp_utc;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const CONNECTOR_SIGNAL_EXPORT_VERSION: u32 = 1;
pub const CONNECTOR_SIGNAL_ATLAS_SCHEMA_VERSION: &str = "connector-signal-atlas-v2";
pub const CONNECTOR_SIGNAL_ATLAS_ARTIFACT_ID: &str = "connector-signal-atlas";
pub const CONNECTOR_SIGNAL_ATLAS_GENERATOR_COMMAND: &str =
    "cargo run --bin export_connector_signal_atlas -- --out-dir docs/connector";
pub const CONNECTOR_SIGNAL_ATLAS_DRIFT_CHECK_COMMAND: &str =
    "scripts/connector_signal_atlas.sh verify";
pub const CONNECTOR_WIDTH6_STRESS_SCHEMA_VERSION: &str = "connector-width6-stress-v79";
pub const CONNECTOR_WIDTH6_STRESS_ARTIFACT_ID: &str = "connector-width6-stress";
pub const CONNECTOR_WIDTH6_STRESS_GENERATOR_COMMAND: &str =
    "cargo run --bin export_connector_width6_stress -- --out-dir docs/connector";
pub const CONNECTOR_REPLICATION_NULL_ATLAS_SCHEMA_VERSION: &str =
    "connector-replication-null-atlas-v1";
pub const CONNECTOR_REPLICATION_NULL_ATLAS_ARTIFACT_ID: &str = "connector-replication-null-atlas";
pub const CONNECTOR_REPLICATION_NULL_ATLAS_GENERATOR_COMMAND: &str =
    "cargo run --bin export_connector_replication_null_atlas -- --out-dir docs/connector";
pub const DEFAULT_SMALL_PRIME_BOUND: u32 = 19;
pub const DEFAULT_SWEEP_BOUNDS: &[u32] = &[5, 7, 11, 13, 17, 19, 23, 29, 31];
pub const WIDTHS: &[u32] = &[5, 6, 7];
pub const DIGITS: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
pub const RESIDUE_MODULI: &[u32] = &[3, 9];
pub const CONNECTOR_STRESS_WIDTH: u32 = 6;
const CONNECTOR_FAMILIES_MODULE: &str = "PrimeArithmetic.Connector.ConcatenationFamilies";
const CONNECTOR_PROFILE_EXAMPLES_MODULE: &str =
    "PrimeArithmetic.Connector.ConcatenationProfileExamples";
const GENERIC_SURVIVOR_COUNT_THEOREM: &str = "PairResidueProfile.forward_reverse_survivor_count_eq";
const CANONICAL_MOD3_SURVIVOR_COUNT_THEOREM: &str =
    "canonicalProfileMod3_forward_reverse_survivor_count_eq";
const CANONICAL_MOD9_SURVIVOR_COUNT_THEOREM: &str =
    "canonicalProfileMod9_forward_reverse_survivor_count_eq";
const ZERO_PADDED_RIGHT: u128 = 30305070305070303;
const TWIN_SMALL_LEFT: u128 = 11;
const TWIN_SMALL_RIGHT: u128 = 13;
const SOPHIE_SMALL_LEFT: u128 = 23;
const SOPHIE_SMALL_RIGHT: u128 = 47;
const ZERO_PADDED_MOD3_SURVIVOR_COUNT_THEOREM: &str =
    "zeroPaddedMembraneProfileMod3_forward_reverse_survivor_count_eq";
const ZERO_PADDED_MOD9_SURVIVOR_COUNT_THEOREM: &str =
    "zeroPaddedMembraneProfileMod9_forward_reverse_survivor_count_eq";
const TWIN_SMALL_MOD3_SURVIVOR_COUNT_THEOREM: &str =
    "twinSmallProfileMod3_forward_reverse_survivor_count_eq";
const TWIN_SMALL_MOD9_SURVIVOR_COUNT_THEOREM: &str =
    "twinSmallProfileMod9_forward_reverse_survivor_count_eq";
const TWIN_SMALL_MOD3_MOD9_SURVIVOR_COUNT_THEOREM: &str =
    "twinSmallProfileMod3_mod9_forward_reverse_survivor_count_eq";
const SOPHIE_SMALL_MOD3_SURVIVOR_COUNT_THEOREM: &str =
    "sophieSmallProfileMod3_forward_reverse_survivor_count_eq";
const SOPHIE_SMALL_MOD9_SURVIVOR_COUNT_THEOREM: &str =
    "sophieSmallProfileMod9_forward_reverse_survivor_count_eq";
const TWIN_PRIME_ABOVE_THREE_MOD3_FORWARD_BLOCKED_THEOREM: &str =
    "twinPrimeAboveThree_decimal_connector_mod3_forward_blocked";
const TWIN_PRIME_ABOVE_THREE_MOD3_REVERSE_BLOCKED_THEOREM: &str =
    "twinPrimeAboveThree_decimal_connector_mod3_reverse_blocked";
const DIGIT8_LEADING_WIDTH6_MOD17_CLASSIFIER_THEOREM: &str =
    "digit8LeadingWidth6_reverseOnly_mem_iff_mod17";
const DIGIT8_LEADING_WIDTH6_MOD23_CLASSIFIER_THEOREM: &str =
    "digit8LeadingWidth6_reverseOnly_mem_iff_mod23";
const DIGIT8_LEADING_WIDTH6_MOD29_CLASSIFIER_THEOREM: &str =
    "digit8LeadingWidth6_reverseOnly_mem_iff_mod29";
const DIGIT8_LEADING_WIDTH6_MOD31_CLASSIFIER_THEOREM: &str =
    "digit8LeadingWidth6_reverseOnly_mem_iff_mod31";
const DIGIT8_LEADING_WIDTH6_MULTI_MODULUS_CLASSIFIER_THEOREM: &str =
    "digit8LeadingWidth6_reverseOnly_multiModulusClassifier";
const DIGIT8_TRAILING_WIDTH5_MOD19_CLASSIFIER_THEOREM: &str =
    "digit8TrailingWidth5_reverseOnly_mem_iff_mod19";
const DIGIT8_TRAILING_WIDTH5_MOD29_CLASSIFIER_THEOREM: &str =
    "digit8TrailingWidth5_reverseOnly_mem_iff_mod29";
const DIGIT8_TRAILING_WIDTH5_MOD31_CLASSIFIER_THEOREM: &str =
    "digit8TrailingWidth5_reverseOnly_mem_iff_mod31";
const DIGIT8_TRAILING_WIDTH5_MULTI_MODULUS_CLASSIFIER_THEOREM: &str =
    "digit8TrailingWidth5_reverseOnly_multiModulusClassifier";
const DIGIT8_TRAILING_WIDTH6_MOD17_CLASSIFIER_THEOREM: &str =
    "digit8TrailingWidth6_reverseOnly_mem_iff_mod17";
const DIGIT8_TRAILING_WIDTH6_MOD19_CLASSIFIER_THEOREM: &str =
    "digit8TrailingWidth6_reverseOnly_mem_iff_mod19";
const DIGIT8_TRAILING_WIDTH6_MOD29_CLASSIFIER_THEOREM: &str =
    "digit8TrailingWidth6_reverseOnly_mem_iff_mod29";
const DIGIT8_TRAILING_WIDTH6_MOD31_CLASSIFIER_THEOREM: &str =
    "digit8TrailingWidth6_reverseOnly_mem_iff_mod31";
const DIGIT8_TRAILING_WIDTH6_MULTI_MODULUS_CLASSIFIER_THEOREM: &str =
    "digit8TrailingWidth6_reverseOnly_multiModulusClassifier";
const DIGIT8_CLASSIFIER_PROOF_STATUS: &str = "finite-bounded-classifier-theorem-backed";
const EXACT_SEPARATOR_UNBACKED_PROOF_STATUS: &str =
    "exact-residue-separator-no-finite-classifier-theorem";
const NON_EXACT_SEPARATOR_PROOF_STATUS: &str = "not-exact-residue-separator";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PairCase {
    pub name: &'static str,
    pub pair: ConcatenationSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NamedPairReport {
    pub name: String,
    pub summary: PairScanSummary,
    pub audit: PairSignalAudit,
    pub sweep: Vec<ResidualSweepRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalReportSettings {
    pub widths: Vec<u32>,
    pub digits: Vec<u8>,
    pub residue_moduli: Vec<u32>,
    pub small_primes: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CanonicalSourceCaseRow {
    pub width: u32,
    pub position: u32,
    pub digit: u8,
    pub direction: Direction,
    pub connector: String,
    pub value: String,
    pub source_class: String,
    pub matched_scan_prime_hit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportedPairReport {
    pub name: String,
    pub summary: PairScanSummary,
    pub audit: PairSignalAudit,
    pub residual_sweep: Vec<ResidualSweepRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComparisonRow {
    pub name: String,
    pub raw_hit_delta: isize,
    pub raw_rate_delta_pp: f64,
    pub corrected_expected_hit_delta: f64,
    pub corrected_residual_ratio_delta: f64,
    pub forward_corrected_ratio: f64,
    pub reverse_corrected_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComparisonVerdict {
    pub raw_broader_law_survives: bool,
    pub corrected_broader_law_survives: bool,
    pub rows: Vec<ComparisonRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResidualSweepRow {
    pub pair_label: String,
    pub bound: u32,
    pub small_primes: Vec<u32>,
    pub forward_corrected_ratio: f64,
    pub reverse_corrected_ratio: f64,
    pub corrected_residual_ratio_delta: f64,
    pub corrected_expected_hit_delta: f64,
    pub forward_corrected_expected_hits: f64,
    pub reverse_corrected_expected_hits: f64,
    pub forward_corrected_poisson_z: f64,
    pub reverse_corrected_poisson_z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResidualSweepExportRow {
    pub pair_label: String,
    pub bound: u32,
    pub small_primes: String,
    pub forward_corrected_ratio: f64,
    pub reverse_corrected_ratio: f64,
    pub corrected_residual_ratio_delta: f64,
    pub corrected_expected_hit_delta: f64,
    pub forward_corrected_expected_hits: f64,
    pub reverse_corrected_expected_hits: f64,
    pub forward_corrected_poisson_z: f64,
    pub reverse_corrected_poisson_z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResidualSweepSummary {
    pub pair_label: String,
    pub negative_bounds: usize,
    pub positive_bounds: usize,
    pub zero_bounds: usize,
    pub min_delta: f64,
    pub max_delta: f64,
    pub sign_stable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignalReportBundle {
    pub export_version: u32,
    pub generated_at_utc: String,
    pub settings: SignalReportSettings,
    pub canonical_source_cases: Vec<CanonicalSourceCaseRow>,
    pub comparison: ComparisonVerdict,
    pub residual_sweep_summary: Vec<ResidualSweepSummary>,
    pub pairs: Vec<ExportedPairReport>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PositionExportRow {
    pub pair_label: String,
    pub left: u128,
    pub right: u128,
    pub direction: Direction,
    pub width: u32,
    pub position: u32,
    pub residue_admissible_candidates: usize,
    pub prime_hits: usize,
    pub working_digits: String,
    pub naive_expected_hits: f64,
    pub small_prime_corrected_expected_hits: f64,
    pub observed_to_corrected_ratio: f64,
    pub direction_prime_hits: usize,
    pub direction_naive_expected_hits: f64,
    pub direction_corrected_expected_hits: f64,
    pub direction_observed_to_corrected_ratio: f64,
    pub direction_corrected_poisson_residual_z: f64,
    pub raw_hit_delta: isize,
    pub raw_rate_delta_pp: f64,
    pub corrected_expected_hit_delta: f64,
    pub corrected_residual_ratio_delta: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorSignalAnalysis {
    pub settings: SignalReportSettings,
    pub canonical_source_cases: Vec<CanonicalSourceCaseRow>,
    pub comparison: ComparisonVerdict,
    pub residual_sweep_summary: Vec<ResidualSweepSummary>,
    pub reports: Vec<NamedPairReport>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorSignalProofLink {
    pub label: String,
    pub lean_module: String,
    pub status: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorSignalClaimStatus {
    pub residue_filter_status: String,
    pub residue_survivor_null_status: String,
    pub analytic_guardrail_status: String,
    pub residual_claim_status: String,
    pub density_mechanism_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorResidueSurvivorRow {
    pub pair_label: String,
    pub left: u128,
    pub right: u128,
    pub modulus: u32,
    pub pair_residue: u32,
    pub blocked_class: u32,
    pub forward_raw_candidates: usize,
    pub reverse_raw_candidates: usize,
    pub forward_survivors: usize,
    pub reverse_survivors: usize,
    pub survivor_delta: isize,
    pub proof_status: String,
    pub lean_module: Option<String>,
    pub lean_theorem: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorResidueSurvivorDelta {
    pub modulus: u32,
    pub survivor_delta: isize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorResidualBoundaryRow {
    pub pair_label: String,
    pub left: u128,
    pub right: u128,
    pub residue_survivor_deltas: Vec<ConnectorResidueSurvivorDelta>,
    pub residue_survivors_equal: bool,
    pub residue_survivor_null_status: String,
    pub empirical_raw_hit_delta: isize,
    pub empirical_corrected_expected_hit_delta: f64,
    pub empirical_corrected_residual_ratio_delta: f64,
    pub boundary_interpretation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorResidualTargetPick {
    pub pair_label: String,
    pub left: u128,
    pub right: u128,
    pub selection_rule: String,
    pub absolute_corrected_residual_ratio_delta: f64,
    pub corrected_residual_ratio_delta: f64,
    pub raw_hit_delta: isize,
    pub residue_survivors_equal: bool,
    pub residue_survivor_deltas: Vec<ConnectorResidueSurvivorDelta>,
    pub target_status: String,
    pub next_step_note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorResidualTargetWidthRow {
    pub direction: Direction,
    pub width: u32,
    pub residue_admissible_candidates: usize,
    pub prime_hits: usize,
    pub small_prime_corrected_expected_hits: f64,
    pub observed_to_corrected_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorResidualTargetWidthContrastPick {
    pub width: u32,
    pub selection_rule: String,
    pub forward_observed_to_corrected_ratio: f64,
    pub reverse_observed_to_corrected_ratio: f64,
    pub signed_ratio_gap: f64,
    pub absolute_ratio_gap: f64,
    pub forward_prime_hits: usize,
    pub reverse_prime_hits: usize,
    pub contrast_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorResidualTargetPositionDigitContrastRow {
    pub rank: usize,
    pub width: u32,
    pub position: u32,
    pub digit: u8,
    pub connector: String,
    pub forward_prime_hit: bool,
    pub reverse_prime_hit: bool,
    pub forward_corrected_expected_hit: f64,
    pub reverse_corrected_expected_hit: f64,
    pub corrected_expected_hit_delta: f64,
    pub forward_observed_to_corrected_ratio: f64,
    pub reverse_observed_to_corrected_ratio: f64,
    pub signed_observed_to_corrected_ratio_gap: f64,
    pub absolute_observed_to_corrected_ratio_gap: f64,
    pub contrast_class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorResidualTargetPositionDigitContrastPick {
    pub selection_rule: String,
    pub row: ConnectorResidualTargetPositionDigitContrastRow,
    pub width_signed_observed_to_corrected_ratio_gap: f64,
    pub top_row_aligns_with_width_gap: bool,
    pub concentration_share_of_absolute_gap: f64,
    pub concentration_status: String,
    pub localization_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorResidualTargetWidthContrastMicroAtlas {
    pub pair_label: String,
    pub left: u128,
    pub right: u128,
    pub width: u32,
    pub selection_rule: String,
    pub residue_survivor_deltas: Vec<ConnectorResidueSurvivorDelta>,
    pub residue_survivors_equal: bool,
    pub residue_null_lean_module: String,
    pub residue_null_lean_theorem: String,
    pub exact_layer_decision: String,
    pub forward_residue_admissible_candidates: usize,
    pub reverse_residue_admissible_candidates: usize,
    pub forward_prime_hits: usize,
    pub reverse_prime_hits: usize,
    pub raw_hit_delta: isize,
    pub forward_corrected_expected_hits: f64,
    pub reverse_corrected_expected_hits: f64,
    pub corrected_expected_hit_delta: f64,
    pub forward_observed_to_corrected_ratio: f64,
    pub reverse_observed_to_corrected_ratio: f64,
    pub signed_observed_to_corrected_ratio_gap: f64,
    pub absolute_observed_to_corrected_ratio_gap: f64,
    pub position_digit_contrast_rows: Vec<ConnectorResidualTargetPositionDigitContrastRow>,
    pub position_digit_contrast_pick: Option<ConnectorResidualTargetPositionDigitContrastPick>,
    pub empirical_status: String,
    pub next_theorem_decision: String,
    pub next_experiment_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorResidualTargetFollowUp {
    pub pair_label: String,
    pub left: u128,
    pub right: u128,
    pub scan_scope: String,
    pub residue_survivor_deltas: Vec<ConnectorResidueSurvivorDelta>,
    pub residue_survivors_equal: bool,
    pub residue_survivor_null_status: String,
    pub residue_null_lean_module: String,
    pub residue_null_lean_theorem: String,
    pub corrected_residual_ratio_delta: f64,
    pub width_rows: Vec<ConnectorResidualTargetWidthRow>,
    pub width_contrast_pick: Option<ConnectorResidualTargetWidthContrastPick>,
    pub width_contrast_micro_atlas: Option<ConnectorResidualTargetWidthContrastMicroAtlas>,
    pub follow_up_status: String,
    pub interpretation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorSignalAtlas {
    pub schema_version: String,
    pub artifact_id: String,
    pub generator_command: String,
    pub drift_check_command: String,
    pub settings: SignalReportSettings,
    pub maintained_pairs: Vec<ExportedPairReport>,
    pub canonical_source_cases: Vec<CanonicalSourceCaseRow>,
    pub comparison: ComparisonVerdict,
    pub residual_sweep_summary: Vec<ResidualSweepSummary>,
    pub residue_survivor_rows: Vec<ConnectorResidueSurvivorRow>,
    pub residual_boundary_rows: Vec<ConnectorResidualBoundaryRow>,
    pub residual_target_pick: Option<ConnectorResidualTargetPick>,
    pub residual_target_follow_up: Option<ConnectorResidualTargetFollowUp>,
    pub proof_links: Vec<ConnectorSignalProofLink>,
    pub claim_status: ConnectorSignalClaimStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6StressSettings {
    pub width: u32,
    pub digits: Vec<u8>,
    pub residue_moduli: Vec<u32>,
    pub small_prime_bounds: Vec<u32>,
    pub control_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6StressRow {
    pub pair_label: String,
    pub left: u128,
    pub right: u128,
    pub small_prime_bound: u32,
    pub small_primes: Vec<u32>,
    pub forward_residue_admissible_candidates: usize,
    pub reverse_residue_admissible_candidates: usize,
    pub forward_prime_hits: usize,
    pub reverse_prime_hits: usize,
    pub raw_hit_delta: isize,
    pub forward_corrected_expected_hits: f64,
    pub reverse_corrected_expected_hits: f64,
    pub corrected_expected_hit_delta: f64,
    pub forward_observed_to_corrected_ratio: f64,
    pub reverse_observed_to_corrected_ratio: f64,
    pub signed_observed_to_corrected_ratio_gap: f64,
    pub absolute_observed_to_corrected_ratio_gap: f64,
    pub residue_survivor_deltas: Vec<ConnectorResidueSurvivorDelta>,
    pub residue_survivors_equal: bool,
    pub position_digit_contrast_pick: Option<ConnectorResidualTargetPositionDigitContrastPick>,
    pub stress_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6StressPairSummary {
    pub pair_label: String,
    pub left: u128,
    pub right: u128,
    pub ladder_index: usize,
    pub median_absolute_gap_rank: usize,
    pub bound_count: usize,
    pub positive_gap_count: usize,
    pub negative_gap_count: usize,
    pub zero_gap_count: usize,
    pub sign_stable: bool,
    pub min_signed_gap: f64,
    pub max_signed_gap: f64,
    pub median_absolute_gap: f64,
    pub localization_statuses: Vec<String>,
    pub top_connectors: Vec<String>,
    pub stress_interpretation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakFollowUpRow {
    pub small_prime_bound: u32,
    pub small_primes: Vec<u32>,
    pub width_signed_observed_to_corrected_ratio_gap: f64,
    pub top_connector: String,
    pub position: u32,
    pub digit: u8,
    pub contrast_class: String,
    pub top_row_signed_observed_to_corrected_ratio_gap: f64,
    pub top_row_absolute_observed_to_corrected_ratio_gap: f64,
    pub top_row_aligns_with_width_gap: bool,
    pub concentration_share_of_absolute_gap: f64,
    pub concentration_status: String,
    pub localization_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakFollowUp {
    pub pair_label: String,
    pub left: u128,
    pub right: u128,
    pub selected_by: String,
    pub median_absolute_gap_rank: usize,
    pub median_absolute_gap: f64,
    pub sign_stable: bool,
    pub signed_gap_range: [f64; 2],
    pub residue_survivor_deltas: Vec<ConnectorResidueSurvivorDelta>,
    pub residue_survivors_equal: bool,
    pub exact_layer_decision: String,
    pub dominant_top_connector: String,
    pub dominant_position: u32,
    pub dominant_digit: u8,
    pub dominant_contrast_class: String,
    pub aligned_bound_count: usize,
    pub unaligned_bound_count: usize,
    pub concentration_statuses: Vec<String>,
    pub localization_statuses: Vec<String>,
    pub feature_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6PeakFollowUpRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakMatchedControlRow {
    pub control_family: String,
    pub pair_label: String,
    pub left: u128,
    pub right: u128,
    pub width: u32,
    pub position: u32,
    pub digit: u8,
    pub connector: String,
    pub forward_residue_admissible: bool,
    pub reverse_residue_admissible: bool,
    pub residue_admissible_in_both_directions: bool,
    pub forward_prime_hit: bool,
    pub reverse_prime_hit: bool,
    pub forward_corrected_expected_hit: f64,
    pub reverse_corrected_expected_hit: f64,
    pub forward_observed_to_corrected_ratio: f64,
    pub reverse_observed_to_corrected_ratio: f64,
    pub signed_observed_to_corrected_ratio_gap: f64,
    pub absolute_observed_to_corrected_ratio_gap: f64,
    pub aligns_with_peak_gap: bool,
    pub contrast_class: String,
    pub is_selected_peak_row: bool,
    pub mod3_exception_class: String,
    pub mod3_theorem_blocked: bool,
    pub mod3_theorem_links: Vec<String>,
    pub residue_survivor_deltas: Vec<ConnectorResidueSurvivorDelta>,
    pub residue_survivors_equal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PeakMatchedControlSummary {
    pub control_family: String,
    pub row_count: usize,
    pub aligned_count: usize,
    pub selected_peak_row_count: usize,
    pub residue_admissible_both_count: usize,
    pub forward_only_hit_count: usize,
    pub reverse_only_hit_count: usize,
    pub both_hit_count: usize,
    pub neither_hit_count: usize,
    pub mod3_exceptional_row_count: usize,
    pub mod3_theorem_blocked_row_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakAdjacentWidthFollowUp {
    pub selection_rule: String,
    pub pair_label: String,
    pub left: u128,
    pub right: u128,
    pub position: u32,
    pub digit: u8,
    pub widths: Vec<u32>,
    pub connectors: Vec<String>,
    pub exact_layer_decision: String,
    pub residue_survivors_equal: bool,
    pub mod3_exception_class: String,
    pub aligned_width_count: usize,
    pub reverse_only_hit_count: usize,
    pub zero_gap_width_count: usize,
    pub strongest_width: u32,
    pub strongest_connector: String,
    pub strongest_signed_observed_to_corrected_ratio_gap: f64,
    pub follow_up_decision: String,
    pub rows: Vec<ConnectorWidth6PeakMatchedControlRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakWidthExtensionProbe {
    pub selection_rule: String,
    pub pair_label: String,
    pub left: u128,
    pub right: u128,
    pub position: u32,
    pub digit: u8,
    pub widths: Vec<u32>,
    pub connectors: Vec<String>,
    pub exact_layer_decision: String,
    pub residue_survivors_equal: bool,
    pub mod3_exception_class: String,
    pub aligned_width_count: usize,
    pub reverse_only_hit_count: usize,
    pub zero_gap_width_count: usize,
    pub strongest_width: u32,
    pub strongest_connector: String,
    pub strongest_signed_observed_to_corrected_ratio_gap: f64,
    pub persistence_decision: String,
    pub rows: Vec<ConnectorWidth6PeakMatchedControlRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakLeadingDigitWidthProbe {
    pub selection_rule: String,
    pub pair_label: String,
    pub left: u128,
    pub right: u128,
    pub position: u32,
    pub widths: Vec<u32>,
    pub digits: Vec<u8>,
    pub connectors: Vec<String>,
    pub exact_layer_decision: String,
    pub residue_survivors_equal: bool,
    pub row_count: usize,
    pub aligned_row_count: usize,
    pub reverse_only_hit_count: usize,
    pub multiple_of3_row_count: usize,
    pub multiple_of3_reverse_only_hit_count: usize,
    pub digit9_reverse_only_hit_count: usize,
    pub other_multiple_of3_reverse_only_hit_count: usize,
    pub non_multiple_of3_reverse_only_hit_count: usize,
    pub zero_gap_row_count: usize,
    pub strongest_width: u32,
    pub strongest_digit: u8,
    pub strongest_connector: String,
    pub strongest_signed_observed_to_corrected_ratio_gap: f64,
    pub digit_pattern_decision: String,
    pub width6_reverse_only_digits: Vec<u8>,
    pub width7_reverse_only_digits: Vec<u8>,
    pub persistent_reverse_only_digits: Vec<u8>,
    pub width6_only_reverse_only_digits: Vec<u8>,
    pub width7_only_reverse_only_digits: Vec<u8>,
    pub neither_reverse_only_digits: Vec<u8>,
    pub heatmap_rows: Vec<ConnectorWidth6PeakLeadingDigitHeatmapRow>,
    pub hypothesis_rankings: Vec<ConnectorWidth6PeakLeadingDigitHypothesisRank>,
    pub top_ranked_hypothesis: String,
    pub rows: Vec<ConnectorWidth6PeakMatchedControlRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakLeadingDigitHeatmapRow {
    pub digit: u8,
    pub width6_connector: String,
    pub width6_contrast_class: String,
    pub width6_signed_observed_to_corrected_ratio_gap: f64,
    pub width6_reverse_only: bool,
    pub width7_connector: String,
    pub width7_contrast_class: String,
    pub width7_signed_observed_to_corrected_ratio_gap: f64,
    pub width7_reverse_only: bool,
    pub persistence_class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakLeadingDigitHypothesisRank {
    pub rank: usize,
    pub hypothesis: String,
    pub score: usize,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakPositionDigitProbe {
    pub selection_rule: String,
    pub pair_label: String,
    pub left: u128,
    pub right: u128,
    pub width: u32,
    pub positions: Vec<u32>,
    pub digits: Vec<u8>,
    pub connectors: Vec<String>,
    pub exact_layer_decision: String,
    pub residue_survivors_equal: bool,
    pub row_count: usize,
    pub aligned_row_count: usize,
    pub reverse_only_hit_count: usize,
    pub position0_reverse_only_hit_count: usize,
    pub non_position0_reverse_only_hit_count: usize,
    pub reverse_only_positions: Vec<u32>,
    pub reverse_only_digits: Vec<u8>,
    pub zero_gap_row_count: usize,
    pub strongest_position: u32,
    pub strongest_digit: u8,
    pub strongest_connector: String,
    pub strongest_signed_observed_to_corrected_ratio_gap: f64,
    pub position_pattern_decision: String,
    pub heatmap_rows: Vec<ConnectorWidth6PeakPositionDigitHeatmapRow>,
    pub hypothesis_rankings: Vec<ConnectorWidth6PeakPositionDigitHypothesisRank>,
    pub top_ranked_hypothesis: String,
    pub rows: Vec<ConnectorWidth6PeakMatchedControlRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakPositionDigitHeatmapRow {
    pub position: u32,
    pub digit6_connector: String,
    pub digit6_contrast_class: String,
    pub digit6_signed_observed_to_corrected_ratio_gap: f64,
    pub digit6_reverse_only: bool,
    pub digit8_connector: String,
    pub digit8_contrast_class: String,
    pub digit8_signed_observed_to_corrected_ratio_gap: f64,
    pub digit8_reverse_only: bool,
    pub digit9_connector: String,
    pub digit9_contrast_class: String,
    pub digit9_signed_observed_to_corrected_ratio_gap: f64,
    pub digit9_reverse_only: bool,
    pub position_class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakPositionDigitHypothesisRank {
    pub rank: usize,
    pub hypothesis: String,
    pub score: usize,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakWidthPositionSpreadComparison {
    pub selection_rule: String,
    pub pair_label: String,
    pub left: u128,
    pub right: u128,
    pub widths: Vec<u32>,
    pub digits: Vec<u8>,
    pub exact_layer_decision: String,
    pub residue_survivors_equal: bool,
    pub comparison_decision: String,
    pub width6_top_ranked_hypothesis: String,
    pub width7_top_ranked_hypothesis: String,
    pub width6_reverse_only_positions: Vec<u32>,
    pub width7_reverse_only_positions: Vec<u32>,
    pub width6_reverse_only_hit_count: usize,
    pub width7_reverse_only_hit_count: usize,
    pub width6_non_position0_reverse_only_hit_count: usize,
    pub width7_non_position0_reverse_only_hit_count: usize,
    pub rows: Vec<ConnectorWidth6PeakWidthPositionSpreadComparisonRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakWidthPositionSpreadComparisonRow {
    pub width: u32,
    pub reverse_only_hit_count: usize,
    pub non_position0_reverse_only_hit_count: usize,
    pub reverse_only_positions: Vec<u32>,
    pub reverse_only_digits: Vec<u8>,
    pub top_ranked_hypothesis: String,
    pub position_pattern_decision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakEdgePositionProbe {
    pub selection_rule: String,
    pub pair_label: String,
    pub left: u128,
    pub right: u128,
    pub widths: Vec<u32>,
    pub digits: Vec<u8>,
    pub exact_layer_decision: String,
    pub residue_survivors_equal: bool,
    pub row_count: usize,
    pub heatmap_cell_count: usize,
    pub leading_reverse_only_hit_count: usize,
    pub trailing_reverse_only_hit_count: usize,
    pub both_edges_reverse_only_count: usize,
    pub leading_only_reverse_only_count: usize,
    pub trailing_only_reverse_only_count: usize,
    pub neither_edge_reverse_only_count: usize,
    pub leading_reverse_only_widths: Vec<u32>,
    pub trailing_reverse_only_widths: Vec<u32>,
    pub reverse_only_digits: Vec<u8>,
    pub strongest_edge: String,
    pub strongest_width: u32,
    pub strongest_digit: u8,
    pub strongest_connector: String,
    pub strongest_signed_observed_to_corrected_ratio_gap: f64,
    pub edge_pattern_decision: String,
    pub heatmap_rows: Vec<ConnectorWidth6PeakEdgePositionHeatmapRow>,
    pub hypothesis_rankings: Vec<ConnectorWidth6PeakEdgePositionHypothesisRank>,
    pub top_ranked_hypothesis: String,
    pub rows: Vec<ConnectorWidth6PeakMatchedControlRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakEdgePositionHeatmapRow {
    pub width: u32,
    pub digit: u8,
    pub leading_connector: String,
    pub leading_contrast_class: String,
    pub leading_signed_observed_to_corrected_ratio_gap: f64,
    pub leading_reverse_only: bool,
    pub trailing_connector: String,
    pub trailing_contrast_class: String,
    pub trailing_signed_observed_to_corrected_ratio_gap: f64,
    pub trailing_reverse_only: bool,
    pub edge_class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakEdgePositionHypothesisRank {
    pub rank: usize,
    pub hypothesis: String,
    pub score: usize,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakEdgePairReplicationProbe {
    pub selection_rule: String,
    pub widths: Vec<u32>,
    pub digits: Vec<u8>,
    pub edges: Vec<String>,
    pub exact_layer_decision: String,
    pub raw_row_count: usize,
    pub nonblocked_row_count: usize,
    pub theorem_blocked_row_count: usize,
    pub ranked_cell_count: usize,
    pub replicated_reverse_only_cell_count: usize,
    pub singleton_reverse_only_cell_count: usize,
    pub neutral_cell_count: usize,
    pub target_connector: String,
    pub target_edge: String,
    pub target_width: u32,
    pub target_digit: u8,
    pub target_nonblocked_pair_count: usize,
    pub target_theorem_blocked_pair_count: usize,
    pub target_reverse_only_pair_count: usize,
    pub target_reverse_only_pair_labels: Vec<String>,
    pub target_replication_status: String,
    pub top_ranked_cell: Option<ConnectorWidth6PeakEdgePairReplicationRankRow>,
    pub replication_decision: String,
    pub rankings: Vec<ConnectorWidth6PeakEdgePairReplicationRankRow>,
    pub rows: Vec<ConnectorWidth6PeakMatchedControlRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakEdgePairReplicationRankRow {
    pub rank: usize,
    pub edge: String,
    pub width: u32,
    pub digit: u8,
    pub connector: String,
    pub nonblocked_pair_count: usize,
    pub theorem_blocked_pair_count: usize,
    pub residue_admissible_pair_count: usize,
    pub reverse_only_pair_count: usize,
    pub forward_only_pair_count: usize,
    pub both_hit_pair_count: usize,
    pub neither_hit_pair_count: usize,
    pub reverse_only_pair_labels: Vec<String>,
    pub strongest_pair_label: String,
    pub strongest_signed_observed_to_corrected_ratio_gap: f64,
    pub replication_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakDigit8EdgeZoomProbe {
    pub selection_rule: String,
    pub digit: u8,
    pub widths: Vec<u32>,
    pub edges: Vec<String>,
    pub widened_pair_count: usize,
    pub widened_pair_labels: Vec<String>,
    pub exact_layer_decision: String,
    pub raw_row_count: usize,
    pub nonblocked_row_count: usize,
    pub theorem_blocked_row_count: usize,
    pub ranked_cell_count: usize,
    pub replicated_reverse_only_cell_count: usize,
    pub singleton_reverse_only_cell_count: usize,
    pub neutral_cell_count: usize,
    pub focus_statuses: Vec<ConnectorWidth6PeakDigit8EdgeFocusStatus>,
    pub top_ranked_cell: Option<ConnectorWidth6PeakEdgePairReplicationRankRow>,
    pub zoom_decision: String,
    pub residue_profile: Option<ConnectorWidth6PeakDigit8ResidueProfile>,
    pub classifier_family_replication: Option<ConnectorWidth6PeakDigit8ClassifierFamilyReplication>,
    pub rankings: Vec<ConnectorWidth6PeakEdgePairReplicationRankRow>,
    pub rows: Vec<ConnectorWidth6PeakMatchedControlRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakDigit8EdgeFocusStatus {
    pub anchor_connector: String,
    pub edge: String,
    pub anchor_width: u32,
    pub digit: u8,
    pub scanned_widths: Vec<u32>,
    pub nonblocked_pair_count: usize,
    pub theorem_blocked_pair_count: usize,
    pub reverse_only_pair_count: usize,
    pub reverse_only_pair_labels: Vec<String>,
    pub strongest_pair_label: String,
    pub strongest_signed_observed_to_corrected_ratio_gap: f64,
    pub replication_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakDigit8ResidueProfile {
    pub selection_rule: String,
    pub moduli: Vec<u32>,
    pub profiled_cell_count: usize,
    pub exact_separator_cell_count: usize,
    pub digit8_best_separator_theorem_backed_count: usize,
    pub digit8_best_separator_unbacked_count: usize,
    pub best_separator: Option<ConnectorWidth6PeakDigit8ResidueSeparatorRow>,
    pub next_unclassified_exact_separator: Option<ConnectorWidth6PeakDigit8ResidueSeparatorRow>,
    pub multi_modulus_summaries: Vec<ConnectorWidth6PeakDigit8ResidueMultiModulusSummary>,
    pub profile_decision: String,
    pub cell_profiles: Vec<ConnectorWidth6PeakDigit8ResidueCellProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakDigit8ResidueMultiModulusSummary {
    pub edge: String,
    pub width: u32,
    pub connector: String,
    pub selection_rule: String,
    pub theorem_backed_separator_count: usize,
    pub moduli: Vec<u32>,
    pub reverse_only_pair_count: usize,
    pub comparison_pair_count: usize,
    pub modulus_rows: Vec<ConnectorWidth6PeakDigit8ResidueMultiModulusRow>,
    pub lean_theorems: Vec<String>,
    pub lean_module: Option<String>,
    pub lean_summary_theorem: Option<String>,
    pub summary_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakDigit8ResidueMultiModulusRow {
    pub modulus: u32,
    pub reverse_only_residues: Vec<u32>,
    pub comparison_residues: Vec<u32>,
    pub lean_theorem: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakDigit8ClassifierFamilyReplication {
    pub selection_rule: String,
    pub baseline_pair_count: usize,
    pub widened_pair_count: usize,
    pub added_pair_count: usize,
    pub source_cell_count: usize,
    pub tested_cell_count: usize,
    pub retained_cell_count: usize,
    pub split_cell_count: usize,
    pub collapsed_cell_count: usize,
    pub replication_decision: String,
    pub split_follow_up: Option<ConnectorWidth6PeakDigit8SplitFollowUp>,
    pub cells: Vec<ConnectorWidth6PeakDigit8ClassifierFamilyReplicationCell>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakDigit8ClassifierFamilyReplicationCell {
    pub edge: String,
    pub width: u32,
    pub connector: String,
    pub source_moduli: Vec<u32>,
    pub lean_module: Option<String>,
    pub lean_summary_theorem: Option<String>,
    pub outside_pair_count: usize,
    pub reverse_only_pair_count: usize,
    pub comparison_pair_count: usize,
    pub retained_modulus_count: usize,
    pub split_modulus_count: usize,
    pub collapsed_modulus_count: usize,
    pub cell_status: String,
    pub modulus_rows: Vec<ConnectorWidth6PeakDigit8ClassifierFamilyReplicationModulusRow>,
    pub rows: Vec<ConnectorWidth6PeakMatchedControlRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakDigit8ClassifierFamilyReplicationModulusRow {
    pub modulus: u32,
    pub source_reverse_only_residues: Vec<u32>,
    pub outside_reverse_only_residues: Vec<u32>,
    pub outside_comparison_residues: Vec<u32>,
    pub shared_residues: Vec<u32>,
    pub modulus_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakDigit8SplitFollowUp {
    pub selection_rule: String,
    pub follow_up_pair_count: usize,
    pub follow_up_pair_labels: Vec<String>,
    pub source_split_row_count: usize,
    pub tested_split_row_count: usize,
    pub stabilized_row_count: usize,
    pub split_again_row_count: usize,
    pub collapsed_row_count: usize,
    pub follow_up_decision: String,
    pub rows: Vec<ConnectorWidth6PeakDigit8SplitFollowUpRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakDigit8SplitFollowUpRow {
    pub edge: String,
    pub width: u32,
    pub connector: String,
    pub modulus: u32,
    pub source_reverse_only_residues: Vec<u32>,
    pub first_outside_reverse_only_residues: Vec<u32>,
    pub follow_up_reverse_only_residues: Vec<u32>,
    pub follow_up_comparison_residues: Vec<u32>,
    pub shared_residues: Vec<u32>,
    pub follow_up_reverse_only_pair_count: usize,
    pub follow_up_comparison_pair_count: usize,
    pub follow_up_status: String,
    pub matched_control_rows: Vec<ConnectorWidth6PeakMatchedControlRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakDigit8ResidueCellProfile {
    pub edge: String,
    pub width: u32,
    pub connector: String,
    pub reverse_only_pair_count: usize,
    pub comparison_pair_count: usize,
    pub best_separator_status: String,
    pub best_separator_modulus: Option<u32>,
    pub best_separator_reverse_only_residues: Vec<u32>,
    pub separator_rows: Vec<ConnectorWidth6PeakDigit8ResidueSeparatorRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakDigit8ResidueSeparatorRow {
    pub edge: String,
    pub width: u32,
    pub connector: String,
    pub modulus: u32,
    pub reverse_only_pair_count: usize,
    pub comparison_pair_count: usize,
    pub reverse_only_residues: Vec<u32>,
    pub comparison_residues: Vec<u32>,
    pub shared_residues: Vec<u32>,
    pub separator_status: String,
    pub proof_status: String,
    pub lean_module: Option<String>,
    pub lean_theorem: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6StressBranchStatusPicker {
    pub selection_rule: String,
    pub branch_count: usize,
    pub live_branch_count: usize,
    pub collapsed_branch_count: usize,
    pub needs_independent_replication_count: usize,
    pub selected_branch: Option<ConnectorWidth6StressBranchStatusRow>,
    pub picker_decision: String,
    pub rows: Vec<ConnectorWidth6StressBranchStatusRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6StressBranchStatusRow {
    pub branch_rank: usize,
    pub source_rank: usize,
    pub branch_id: String,
    pub source_probe: String,
    pub edge: String,
    pub width: u32,
    pub digit: u8,
    pub connector: String,
    pub nonblocked_pair_count: usize,
    pub reverse_only_pair_count: usize,
    pub strongest_signed_observed_to_corrected_ratio_gap: f64,
    pub branch_status: String,
    pub status_reason: String,
    pub next_experiment_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6SelectedBranchIndependentReplication {
    pub selection_rule: String,
    pub source_branch_id: String,
    pub source_branch_status: String,
    pub edge: String,
    pub width: u32,
    pub position: u32,
    pub digit: u8,
    pub connector: String,
    pub fresh_pair_count: usize,
    pub fresh_pair_labels: Vec<String>,
    pub row_count: usize,
    pub theorem_blocked_row_count: usize,
    pub nonblocked_row_count: usize,
    pub residue_admissible_pair_count: usize,
    pub reverse_only_pair_count: usize,
    pub forward_only_pair_count: usize,
    pub both_hit_pair_count: usize,
    pub neither_hit_pair_count: usize,
    pub exact_layer_decision: String,
    pub exact_layer_lean_module: String,
    pub exact_layer_lean_theorems: Vec<String>,
    pub replication_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6PeakMatchedControlRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6RetiredNonMod3Candidate {
    pub candidate_id: String,
    pub edge: String,
    pub width: u32,
    pub position: u32,
    pub digit: u8,
    pub connector: String,
    pub source_modulus: u32,
    pub source_reverse_only_residues: Vec<u32>,
    pub replication_reverse_only_residues: Vec<u32>,
    pub replication_shared_residues: Vec<u32>,
    pub retirement_decision: String,
    pub retirement_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6NonMod3CandidatePicker {
    pub selection_rule: String,
    pub fresh_pair_count: usize,
    pub fresh_pair_labels: Vec<String>,
    pub widths: Vec<u32>,
    pub digits: Vec<u8>,
    pub edges: Vec<String>,
    pub retired_candidate_count: usize,
    pub retired_candidate_ids: Vec<String>,
    pub retired_candidates: Vec<ConnectorWidth6RetiredNonMod3Candidate>,
    pub row_count: usize,
    pub theorem_blocked_row_count: usize,
    pub nonblocked_row_count: usize,
    pub ranked_cell_count: usize,
    pub reverse_only_cell_count: usize,
    pub selected_candidate: Option<ConnectorWidth6PeakEdgePairReplicationRankRow>,
    pub picker_decision: String,
    pub next_experiment_target: String,
    pub rankings: Vec<ConnectorWidth6PeakEdgePairReplicationRankRow>,
    pub rows: Vec<ConnectorWidth6PeakMatchedControlRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6NonMod3CandidateSecondReplication {
    pub selection_rule: String,
    pub source_candidate_id: String,
    pub source_candidate_rank: usize,
    pub edge: String,
    pub width: u32,
    pub position: u32,
    pub digit: u8,
    pub connector: String,
    pub fresh_pair_count: usize,
    pub fresh_pair_labels: Vec<String>,
    pub row_count: usize,
    pub theorem_blocked_row_count: usize,
    pub nonblocked_row_count: usize,
    pub residue_admissible_pair_count: usize,
    pub reverse_only_pair_count: usize,
    pub forward_only_pair_count: usize,
    pub both_hit_pair_count: usize,
    pub neither_hit_pair_count: usize,
    pub reverse_only_pair_labels: Vec<String>,
    pub strongest_pair_label: String,
    pub strongest_signed_observed_to_corrected_ratio_gap: f64,
    pub replication_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6PeakMatchedControlRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6NonMod3RetirementSummaryRow {
    pub candidate_id: String,
    pub edge: String,
    pub width: u32,
    pub position: u32,
    pub digit: u8,
    pub connector: String,
    pub retirement_decision: String,
    pub retirement_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6InteriorNonMod3FamilyPicker {
    pub selection_rule: String,
    pub source_pair_count: usize,
    pub source_pair_labels: Vec<String>,
    pub widths: Vec<u32>,
    pub digits: Vec<u8>,
    pub position_scope: String,
    pub retired_candidate_count: usize,
    pub retired_candidate_ids: Vec<String>,
    pub row_count: usize,
    pub theorem_blocked_row_count: usize,
    pub nonblocked_row_count: usize,
    pub ranked_cell_count: usize,
    pub reverse_only_cell_count: usize,
    pub selected_candidate: Option<ConnectorWidth6InteriorNonMod3FamilyRankRow>,
    pub picker_decision: String,
    pub next_experiment_target: String,
    pub rankings: Vec<ConnectorWidth6InteriorNonMod3FamilyRankRow>,
    pub rows: Vec<ConnectorWidth6PeakMatchedControlRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6InteriorNonMod3FamilyRankRow {
    pub rank: usize,
    pub width: u32,
    pub position: u32,
    pub digit: u8,
    pub connector: String,
    pub nonblocked_pair_count: usize,
    pub theorem_blocked_pair_count: usize,
    pub residue_admissible_pair_count: usize,
    pub reverse_only_pair_count: usize,
    pub forward_only_pair_count: usize,
    pub both_hit_pair_count: usize,
    pub neither_hit_pair_count: usize,
    pub reverse_only_pair_labels: Vec<String>,
    pub strongest_pair_label: String,
    pub strongest_signed_observed_to_corrected_ratio_gap: f64,
    pub replication_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6InteriorNonMod3FamilyReplication {
    pub selection_rule: String,
    pub source_candidate_id: String,
    pub source_candidate_rank: usize,
    pub width: u32,
    pub position: u32,
    pub digit: u8,
    pub connector: String,
    pub fresh_pair_count: usize,
    pub fresh_pair_labels: Vec<String>,
    pub row_count: usize,
    pub theorem_blocked_row_count: usize,
    pub nonblocked_row_count: usize,
    pub residue_admissible_pair_count: usize,
    pub reverse_only_pair_count: usize,
    pub forward_only_pair_count: usize,
    pub both_hit_pair_count: usize,
    pub neither_hit_pair_count: usize,
    pub reverse_only_pair_labels: Vec<String>,
    pub strongest_pair_label: String,
    pub strongest_signed_observed_to_corrected_ratio_gap: f64,
    pub replication_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6PeakMatchedControlRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6MultiDigitMotifRow {
    pub control_family: String,
    pub pair_label: String,
    pub left: u128,
    pub right: u128,
    pub width: u32,
    pub start_position: u32,
    pub motif_positions: Vec<u32>,
    pub motif_digits: Vec<u8>,
    pub connector: String,
    pub connector_value: u128,
    pub forward_value: u128,
    pub reverse_value: u128,
    pub forward_residue_admissible: bool,
    pub reverse_residue_admissible: bool,
    pub residue_admissible_in_both_directions: bool,
    pub forward_prime_hit: bool,
    pub reverse_prime_hit: bool,
    pub contrast_class: String,
    pub mod3_exception_class: String,
    pub mod3_theorem_blocked: bool,
    pub mod3_theorem_links: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6MultiDigitMotifRankRow {
    pub rank: usize,
    pub width: u32,
    pub start_position: u32,
    pub motif_positions: Vec<u32>,
    pub motif_digits: Vec<u8>,
    pub connector: String,
    pub nonblocked_pair_count: usize,
    pub theorem_blocked_pair_count: usize,
    pub residue_admissible_pair_count: usize,
    pub reverse_only_pair_count: usize,
    pub forward_only_pair_count: usize,
    pub both_hit_pair_count: usize,
    pub neither_hit_pair_count: usize,
    pub reverse_only_pair_labels: Vec<String>,
    pub replication_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6MultiDigitMotifFamilyPicker {
    pub selection_rule: String,
    pub source_pair_count: usize,
    pub source_pair_labels: Vec<String>,
    pub widths: Vec<u32>,
    pub motif_width: u32,
    pub digits: Vec<u8>,
    pub position_scope: String,
    pub pivot_from_single_digit_retired_candidate_count: usize,
    pub row_count: usize,
    pub theorem_blocked_row_count: usize,
    pub nonblocked_row_count: usize,
    pub ranked_motif_count: usize,
    pub reverse_only_motif_count: usize,
    pub selected_motif: Option<ConnectorWidth6MultiDigitMotifRankRow>,
    pub picker_decision: String,
    pub next_experiment_target: String,
    pub rankings: Vec<ConnectorWidth6MultiDigitMotifRankRow>,
    pub rows: Vec<ConnectorWidth6MultiDigitMotifRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6MultiDigitMotifReplication {
    pub selection_rule: String,
    pub source_motif_id: String,
    pub source_motif_rank: usize,
    pub width: u32,
    pub start_position: u32,
    pub motif_digits: Vec<u8>,
    pub connector: String,
    pub fresh_pair_count: usize,
    pub fresh_pair_labels: Vec<String>,
    pub row_count: usize,
    pub theorem_blocked_row_count: usize,
    pub nonblocked_row_count: usize,
    pub residue_admissible_pair_count: usize,
    pub reverse_only_pair_count: usize,
    pub forward_only_pair_count: usize,
    pub both_hit_pair_count: usize,
    pub neither_hit_pair_count: usize,
    pub reverse_only_pair_labels: Vec<String>,
    pub replication_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6MultiDigitMotifRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6MultiDigitMotifResidueProfile {
    pub selection_rule: String,
    pub source_motif_id: String,
    pub width: u32,
    pub start_position: u32,
    pub motif_digits: Vec<u8>,
    pub connector: String,
    pub moduli: Vec<u32>,
    pub reverse_only_pair_count: usize,
    pub comparison_pair_count: usize,
    pub profiled_modulus_count: usize,
    pub exact_separator_count: usize,
    pub best_separator: Option<ConnectorWidth6NonMod3CandidateResidueProfileRow>,
    pub profile_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6NonMod3CandidateResidueProfileRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6MultiDigitMotifResidueSeparatorReplication {
    pub selection_rule: String,
    pub source_motif_id: String,
    pub width: u32,
    pub start_position: u32,
    pub motif_digits: Vec<u8>,
    pub connector: String,
    pub source_modulus: u32,
    pub source_reverse_only_residues: Vec<u32>,
    pub fresh_pair_count: usize,
    pub fresh_pair_labels: Vec<String>,
    pub row_count: usize,
    pub reverse_only_pair_count: usize,
    pub comparison_pair_count: usize,
    pub reverse_only_residues: Vec<u32>,
    pub comparison_residues: Vec<u32>,
    pub shared_residues: Vec<u32>,
    pub retained_residue_count: usize,
    pub split_residue_count: usize,
    pub collapsed_residue_count: usize,
    pub separator_status: String,
    pub replication_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6MultiDigitMotifRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6MultiDigitMotifRetirementSummaryRow {
    pub source_motif_id: String,
    pub width: u32,
    pub start_position: u32,
    pub motif_digits: Vec<u8>,
    pub connector: String,
    pub source_modulus: u32,
    pub source_reverse_only_residues: Vec<u32>,
    pub separator_status: String,
    pub retirement_decision: String,
    pub retirement_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6OrthogonalPairFamilyControlMatrix {
    pub selection_rule: String,
    pub pair_family_count: usize,
    pub widths: Vec<u32>,
    pub motif_width: u32,
    pub digits: Vec<u8>,
    pub position_scope: String,
    pub excluded_connector_rule: String,
    pub source_row_count: usize,
    pub fresh_row_count: usize,
    pub selected_branch: Option<ConnectorWidth6OrthogonalPairFamilySelectedBranch>,
    pub matrix_decision: String,
    pub next_experiment_target: String,
    pub pair_families: Vec<ConnectorWidth6OrthogonalPairFamilyControl>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6OrthogonalPairFamilyControl {
    pub pair_family: String,
    pub pair_gap: u32,
    pub source_pair_count: usize,
    pub source_pair_labels: Vec<String>,
    pub fresh_pair_count: usize,
    pub fresh_pair_labels: Vec<String>,
    pub row_count: usize,
    pub theorem_blocked_row_count: usize,
    pub nonblocked_row_count: usize,
    pub ranked_motif_count: usize,
    pub reverse_only_motif_count: usize,
    pub selected_motif: Option<ConnectorWidth6MultiDigitMotifRankRow>,
    pub fresh_replication: Option<ConnectorWidth6OrthogonalPairFamilyReplication>,
    pub picker_decision: String,
    pub next_experiment_target: String,
    pub rankings: Vec<ConnectorWidth6MultiDigitMotifRankRow>,
    pub rows: Vec<ConnectorWidth6MultiDigitMotifRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6OrthogonalPairFamilyReplication {
    pub selection_rule: String,
    pub pair_family: String,
    pub pair_gap: u32,
    pub source_branch_id: String,
    pub source_motif_rank: usize,
    pub width: u32,
    pub start_position: u32,
    pub motif_positions: Vec<u32>,
    pub motif_digits: Vec<u8>,
    pub connector: String,
    pub fresh_pair_count: usize,
    pub fresh_pair_labels: Vec<String>,
    pub row_count: usize,
    pub theorem_blocked_row_count: usize,
    pub nonblocked_row_count: usize,
    pub residue_admissible_pair_count: usize,
    pub reverse_only_pair_count: usize,
    pub forward_only_pair_count: usize,
    pub both_hit_pair_count: usize,
    pub neither_hit_pair_count: usize,
    pub reverse_only_pair_labels: Vec<String>,
    pub replication_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6MultiDigitMotifRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6OrthogonalPairFamilySelectedBranch {
    pub branch_id: String,
    pub pair_family: String,
    pub pair_gap: u32,
    pub width: u32,
    pub start_position: u32,
    pub motif_positions: Vec<u32>,
    pub motif_digits: Vec<u8>,
    pub connector: String,
    pub source_reverse_only_pair_count: usize,
    pub fresh_reverse_only_pair_count: usize,
    pub fresh_forward_only_pair_count: usize,
    pub fresh_both_hit_pair_count: usize,
    pub fresh_neither_hit_pair_count: usize,
    pub branch_status: String,
    pub next_experiment_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6OrthogonalPairFamilyResidueProfile {
    pub selection_rule: String,
    pub source_branch_id: String,
    pub pair_family: String,
    pub pair_gap: u32,
    pub width: u32,
    pub start_position: u32,
    pub motif_positions: Vec<u32>,
    pub motif_digits: Vec<u8>,
    pub connector: String,
    pub moduli: Vec<u32>,
    pub reverse_only_pair_count: usize,
    pub comparison_pair_count: usize,
    pub profiled_modulus_count: usize,
    pub exact_separator_count: usize,
    pub best_separator: Option<ConnectorWidth6NonMod3CandidateResidueProfileRow>,
    pub profile_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6NonMod3CandidateResidueProfileRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6OrthogonalPairFamilyResidueSeparatorReplication {
    pub selection_rule: String,
    pub source_branch_id: String,
    pub pair_family: String,
    pub pair_gap: u32,
    pub width: u32,
    pub start_position: u32,
    pub motif_positions: Vec<u32>,
    pub motif_digits: Vec<u8>,
    pub connector: String,
    pub source_modulus: u32,
    pub source_reverse_only_residues: Vec<u32>,
    pub fresh_pair_count: usize,
    pub fresh_pair_labels: Vec<String>,
    pub row_count: usize,
    pub reverse_only_pair_count: usize,
    pub comparison_pair_count: usize,
    pub reverse_only_residues: Vec<u32>,
    pub comparison_residues: Vec<u32>,
    pub shared_residues: Vec<u32>,
    pub retained_residue_count: usize,
    pub split_residue_count: usize,
    pub collapsed_residue_count: usize,
    pub separator_status: String,
    pub replication_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6MultiDigitMotifRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyCohortRetentionPicker {
    pub selection_rule: String,
    pub source_surface_count: usize,
    pub candidate_cohort_count: usize,
    pub cohort_ready_count: usize,
    pub selected_cohort: Option<ConnectorWidth6PairFamilyCohortRetentionRow>,
    pub picker_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6PairFamilyCohortRetentionRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyCohortRetentionRow {
    pub cohort_id: String,
    pub width: u32,
    pub connector: String,
    pub pair_families: Vec<String>,
    pub source_branch_ids: Vec<String>,
    pub source_selected_count: usize,
    pub source_reverse_only_total: usize,
    pub fresh_survivor_count: usize,
    pub fresh_reverse_only_total: usize,
    pub fresh_forward_only_total: usize,
    pub profiled_branch_count: usize,
    pub separator_retained_count: usize,
    pub separator_split_count: usize,
    pub separator_collapsed_count: usize,
    pub theorem_candidate_count: usize,
    pub cohort_status: String,
    pub next_experiment_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PairFamilyCohortResidueProfile {
    pub selection_rule: String,
    pub cohort_id: String,
    pub width: u32,
    pub connector: String,
    pub pair_families: Vec<String>,
    pub source_branch_ids: Vec<String>,
    pub branch_count: usize,
    pub fresh_survivor_branch_count: usize,
    pub moduli: Vec<u32>,
    pub row_count: usize,
    pub reverse_only_pair_count: usize,
    pub comparison_pair_count: usize,
    pub profiled_modulus_count: usize,
    pub exact_separator_count: usize,
    pub best_separator: Option<ConnectorWidth6NonMod3CandidateResidueProfileRow>,
    pub profile_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6NonMod3CandidateResidueProfileRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyCohortResidueSeparatorReplication {
    pub selection_rule: String,
    pub cohort_id: String,
    pub width: u32,
    pub connector: String,
    pub pair_families: Vec<String>,
    pub source_branch_ids: Vec<String>,
    pub source_modulus: u32,
    pub source_reverse_only_residues: Vec<u32>,
    pub separator_family_count: usize,
    pub separator_pair_count: usize,
    pub separator_pair_labels: Vec<String>,
    pub row_count: usize,
    pub reverse_only_pair_count: usize,
    pub comparison_pair_count: usize,
    pub reverse_only_residues: Vec<u32>,
    pub comparison_residues: Vec<u32>,
    pub shared_residues: Vec<u32>,
    pub retained_residue_count: usize,
    pub split_residue_count: usize,
    pub collapsed_residue_count: usize,
    pub separator_status: String,
    pub replication_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6MultiDigitMotifRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilySurfacePicker {
    pub selection_rule: String,
    pub source_surface_count: usize,
    pub candidate_surface_count: usize,
    pub surface_ready_count: usize,
    pub selected_surface: Option<ConnectorWidth6PairFamilySurfaceRow>,
    pub picker_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6PairFamilySurfaceRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilySurfaceRow {
    pub surface_id: String,
    pub surface_label: String,
    pub pair_families: Vec<String>,
    pub source_branch_ids: Vec<String>,
    pub source_selected_count: usize,
    pub source_reverse_only_total: usize,
    pub fresh_survivor_count: usize,
    pub fresh_reverse_only_total: usize,
    pub fresh_forward_only_total: usize,
    pub surface_status: String,
    pub next_experiment_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PairFamilySurfaceResidueProfile {
    pub selection_rule: String,
    pub surface_id: String,
    pub surface_label: String,
    pub pair_families: Vec<String>,
    pub source_branch_ids: Vec<String>,
    pub branch_count: usize,
    pub fresh_survivor_branch_count: usize,
    pub connectors: Vec<String>,
    pub moduli: Vec<u32>,
    pub row_count: usize,
    pub reverse_only_pair_count: usize,
    pub comparison_pair_count: usize,
    pub profiled_modulus_count: usize,
    pub exact_separator_count: usize,
    pub best_separator: Option<ConnectorWidth6NonMod3CandidateResidueProfileRow>,
    pub profile_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6NonMod3CandidateResidueProfileRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PairFamilyTopNMotifSurfaceProfile {
    pub selection_rule: String,
    pub source_surface_id: String,
    pub source_surface_label: String,
    pub top_n_per_family: usize,
    pub pair_family_count: usize,
    pub pair_families: Vec<String>,
    pub source_motif_count: usize,
    pub fresh_survivor_motif_count: usize,
    pub fresh_reverse_only_total: usize,
    pub fresh_forward_only_total: usize,
    pub connectors: Vec<String>,
    pub moduli: Vec<u32>,
    pub row_count: usize,
    pub reverse_only_pair_count: usize,
    pub comparison_pair_count: usize,
    pub profiled_modulus_count: usize,
    pub exact_separator_count: usize,
    pub best_separator: Option<ConnectorWidth6NonMod3CandidateResidueProfileRow>,
    pub profile_decision: String,
    pub next_experiment_target: String,
    pub motif_rows: Vec<ConnectorWidth6PairFamilyTopNMotifRow>,
    pub rows: Vec<ConnectorWidth6NonMod3CandidateResidueProfileRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyTopNMotifRow {
    pub pair_family: String,
    pub pair_gap: u32,
    pub source_rank: usize,
    pub width: u32,
    pub connector: String,
    pub source_reverse_only_pair_count: usize,
    pub fresh_reverse_only_pair_count: usize,
    pub fresh_forward_only_pair_count: usize,
    pub fresh_both_hit_pair_count: usize,
    pub fresh_neither_hit_pair_count: usize,
    pub motif_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PairFamilyGapCohortGeometryControl {
    pub selection_rule: String,
    pub source_surface_id: String,
    pub top_n_per_family: usize,
    pub pair_family_count: usize,
    pub pair_families: Vec<String>,
    pub source_motif_count: usize,
    pub fresh_motif_count: usize,
    pub fresh_survivor_motif_count: usize,
    pub geometry_row_count: usize,
    pub retained_geometry_count: usize,
    pub selected_geometry: Option<ConnectorWidth6PairFamilyGapCohortGeometryRow>,
    pub control_decision: String,
    pub next_experiment_target: String,
    pub motif_rows: Vec<ConnectorWidth6PairFamilyTopNMotifRow>,
    pub geometry_rows: Vec<ConnectorWidth6PairFamilyGapCohortGeometryRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyGapCohortGeometryRow {
    pub width: u32,
    pub connector: String,
    pub source_selected_count: usize,
    pub source_reverse_only_total: usize,
    pub source_pair_families: Vec<String>,
    pub fresh_survivor_family_count: usize,
    pub fresh_reverse_only_total: usize,
    pub fresh_forward_only_total: usize,
    pub fresh_both_hit_total: usize,
    pub fresh_neither_hit_total: usize,
    pub fresh_pair_families: Vec<String>,
    pub geometry_status: String,
    pub next_experiment_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyGapCohortRatioGeometryControl {
    pub selection_rule: String,
    pub source_profile_id: String,
    pub pair_family_count: usize,
    pub pair_families: Vec<String>,
    pub family_row_count: usize,
    pub geometry_row_count: usize,
    pub retained_geometry_count: usize,
    pub selected_geometry: Option<ConnectorWidth6PairFamilyGapCohortRatioGeometryRow>,
    pub control_decision: String,
    pub next_experiment_target: String,
    pub family_rows: Vec<ConnectorWidth6PairFamilyGapCohortRatioGeometryFamilyRow>,
    pub geometry_rows: Vec<ConnectorWidth6PairFamilyGapCohortRatioGeometryRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyGapCohortRatioGeometryRow {
    pub width: u32,
    pub connector: String,
    pub source_selected_count: usize,
    pub source_pair_families: Vec<String>,
    pub reverse_biased_family_count: usize,
    pub forward_biased_family_count: usize,
    pub neutral_family_count: usize,
    pub retained_bias_direction: String,
    pub retained_bias_family_count: usize,
    pub fresh_reverse_only_total: usize,
    pub fresh_forward_only_total: usize,
    pub fresh_both_hit_total: usize,
    pub fresh_neither_hit_total: usize,
    pub signed_hit_delta_total: i32,
    pub absolute_hit_delta_total: usize,
    pub ratio_geometry_status: String,
    pub next_experiment_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyGapCohortRatioGeometryFamilyRow {
    pub stage: String,
    pub pair_family: String,
    pub pair_gap: u32,
    pub source_rank: usize,
    pub width: u32,
    pub connector: String,
    pub reverse_only_pair_count: usize,
    pub forward_only_pair_count: usize,
    pub both_hit_pair_count: usize,
    pub neither_hit_pair_count: usize,
    pub signed_hit_delta: i32,
    pub bias_direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyGapCohortRatioGeometryReplication {
    pub selection_rule: String,
    pub source_width: u32,
    pub source_connector: String,
    pub source_bias_direction: String,
    pub source_retained_bias_family_count: usize,
    pub pair_family_count: usize,
    pub separator_row_count: usize,
    pub reverse_biased_family_count: usize,
    pub forward_biased_family_count: usize,
    pub neutral_family_count: usize,
    pub retained_direction_family_count: usize,
    pub split_direction_family_count: usize,
    pub separator_reverse_only_total: usize,
    pub separator_forward_only_total: usize,
    pub separator_signed_hit_delta_total: i32,
    pub ratio_geometry_status: String,
    pub replication_decision: String,
    pub next_experiment_target: String,
    pub family_rows: Vec<ConnectorWidth6PairFamilyGapCohortRatioGeometryFamilyRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6GeneratedGapLadderRow {
    pub pair_family: String,
    pub pair_gap: u32,
    pub lower_bound: u128,
    pub pair_count: usize,
    pub source_pair_count: usize,
    pub fresh_pair_count: usize,
    pub separator_pair_count: usize,
    pub first_left: u128,
    pub first_right: u128,
    pub last_left: u128,
    pub last_right: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyGapCohortRatioGeometryExpansion {
    pub selection_rule: String,
    pub source_width: u32,
    pub source_connector: String,
    pub source_bias_direction: String,
    pub pair_family_count: usize,
    pub window_count: usize,
    pub generated_pair_count: usize,
    pub reverse_biased_window_count: usize,
    pub forward_biased_window_count: usize,
    pub neutral_window_count: usize,
    pub retained_direction_window_count: usize,
    pub split_direction_window_count: usize,
    pub expansion_status: String,
    pub expansion_decision: String,
    pub next_experiment_target: String,
    pub generated_ladders: Vec<ConnectorWidth6GeneratedGapLadderRow>,
    pub family_rows: Vec<ConnectorWidth6PairFamilyGapCohortRatioGeometryFamilyRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PairFamilyGapCohortRatioCorrectionBoundStability {
    pub selection_rule: String,
    pub source_width: u32,
    pub source_connector: String,
    pub source_bias_direction: String,
    pub bounds: Vec<u32>,
    pub bound_count: usize,
    pub stable_bound_count: usize,
    pub unstable_bound_count: usize,
    pub pair_family_count: usize,
    pub row_count: usize,
    pub stability_status: String,
    pub stability_decision: String,
    pub next_experiment_target: String,
    pub bound_rows: Vec<ConnectorWidth6RatioCorrectionBoundRow>,
    pub family_rows: Vec<ConnectorWidth6RatioCorrectionBoundFamilyRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6RatioCorrectionBoundRow {
    pub small_prime_bound: u32,
    pub small_primes: Vec<u32>,
    pub pair_family_count: usize,
    pub reverse_positive_family_count: usize,
    pub forward_positive_family_count: usize,
    pub neutral_family_count: usize,
    pub aggregate_reverse_observed_to_corrected_ratio: f64,
    pub aggregate_forward_observed_to_corrected_ratio: f64,
    pub aggregate_signed_observed_to_corrected_ratio_gap: f64,
    pub stability_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6RatioCorrectionBoundFamilyRow {
    pub small_prime_bound: u32,
    pub pair_family: String,
    pub pair_gap: u32,
    pub row_count: usize,
    pub reverse_observed_to_corrected_ratio_sum: f64,
    pub forward_observed_to_corrected_ratio_sum: f64,
    pub signed_observed_to_corrected_ratio_gap: f64,
    pub bias_direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PairFamilyGapCohortRatioGeometryAtlas {
    pub selection_rule: String,
    pub source_width: u32,
    pub source_connector: String,
    pub source_bias_direction: String,
    pub surface_count: usize,
    pub surfaces: Vec<String>,
    pub bounds: Vec<u32>,
    pub bound_count: usize,
    pub family_row_count: usize,
    pub correction_bound_row_count: usize,
    pub gap_band_status: String,
    pub size_band_status: String,
    pub atlas_status: String,
    pub atlas_decision: String,
    pub next_experiment_target: String,
    pub surface_rows: Vec<ConnectorWidth6RatioGeometryAtlasSurfaceRow>,
    pub family_rows: Vec<ConnectorWidth6PairFamilyGapCohortRatioGeometryFamilyRow>,
    pub correction_bound_rows: Vec<ConnectorWidth6RatioGeometryAtlasCorrectionBoundRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6RatioGeometryAtlasSurfaceRow {
    pub surface_id: String,
    pub pair_families: Vec<String>,
    pub pair_family_count: usize,
    pub window_count: usize,
    pub reverse_biased_window_count: usize,
    pub forward_biased_window_count: usize,
    pub neutral_window_count: usize,
    pub retained_direction_window_count: usize,
    pub split_direction_window_count: usize,
    pub raw_geometry_status: String,
    pub correction_bound_status: String,
    pub stable_bound_count: usize,
    pub balanced_bound_count: usize,
    pub concentrated_bound_count: usize,
    pub split_bound_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6RatioGeometryAtlasCorrectionBoundRow {
    pub surface_id: String,
    pub small_prime_bound: u32,
    pub small_primes: Vec<u32>,
    pub pair_family_count: usize,
    pub reverse_positive_family_count: usize,
    pub forward_positive_family_count: usize,
    pub neutral_family_count: usize,
    pub aggregate_reverse_observed_to_corrected_ratio: f64,
    pub aggregate_forward_observed_to_corrected_ratio: f64,
    pub aggregate_signed_observed_to_corrected_ratio_gap: f64,
    pub correction_bound_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PairFamilyGapCohortRatioGeometryPicker {
    pub selection_rule: String,
    pub candidate_count: usize,
    pub surface_count: usize,
    pub surfaces: Vec<String>,
    pub bounds: Vec<u32>,
    pub bound_count: usize,
    pub stable_candidate_count: usize,
    pub selected_candidate: Option<ConnectorWidth6RatioGeometryPickerCandidateRow>,
    pub picker_decision: String,
    pub next_experiment_target: String,
    pub candidate_rows: Vec<ConnectorWidth6RatioGeometryPickerCandidateRow>,
    pub surface_rows: Vec<ConnectorWidth6RatioGeometryPickerSurfaceRow>,
    pub family_rows: Vec<ConnectorWidth6PairFamilyGapCohortRatioGeometryFamilyRow>,
    pub correction_bound_rows: Vec<ConnectorWidth6RatioGeometryPickerCorrectionBoundRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6RatioGeometryPickerCandidateRow {
    pub candidate_rank: usize,
    pub width: u32,
    pub connector: String,
    pub stable_surface_count: usize,
    pub stable_bound_count: usize,
    pub shared_stable_direction: String,
    pub gap_band_status: String,
    pub gap_band_stable_direction: String,
    pub size_band_status: String,
    pub size_band_stable_direction: String,
    pub absolute_aggregate_corrected_ratio_gap: f64,
    pub candidate_status: String,
    pub next_experiment_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6RatioGeometryPickerSurfaceRow {
    pub width: u32,
    pub connector: String,
    pub surface_id: String,
    pub pair_families: Vec<String>,
    pub pair_family_count: usize,
    pub window_count: usize,
    pub reverse_biased_window_count: usize,
    pub forward_biased_window_count: usize,
    pub neutral_window_count: usize,
    pub reverse_stable_bound_count: usize,
    pub forward_stable_bound_count: usize,
    pub balanced_bound_count: usize,
    pub concentrated_bound_count: usize,
    pub split_bound_count: usize,
    pub stable_direction: String,
    pub correction_bound_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6RatioGeometryPickerCorrectionBoundRow {
    pub width: u32,
    pub connector: String,
    pub surface_id: String,
    pub small_prime_bound: u32,
    pub small_primes: Vec<u32>,
    pub pair_family_count: usize,
    pub reverse_positive_family_count: usize,
    pub forward_positive_family_count: usize,
    pub neutral_family_count: usize,
    pub aggregate_reverse_observed_to_corrected_ratio: f64,
    pub aggregate_forward_observed_to_corrected_ratio: f64,
    pub aggregate_signed_observed_to_corrected_ratio_gap: f64,
    pub correction_bound_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6CohortInvariantResidueProfile {
    pub selection_rule: String,
    pub source_width: u32,
    pub source_connector: String,
    pub source_direction: String,
    pub surfaces: Vec<String>,
    pub surface_count: usize,
    pub pair_families: Vec<String>,
    pub moduli: Vec<u32>,
    pub row_count: usize,
    pub target_pair_count: usize,
    pub comparison_pair_count: usize,
    pub profiled_modulus_count: usize,
    pub exact_separator_count: usize,
    pub coherent_separator_count: usize,
    pub best_coherent_separator: Option<ConnectorWidth6CohortInvariantResidueSeparator>,
    pub profile_status: String,
    pub profile_decision: String,
    pub next_experiment_target: String,
    pub surface_rows: Vec<ConnectorWidth6CohortInvariantResidueSurfaceRow>,
    pub residue_rows: Vec<ConnectorWidth6CohortInvariantResidueProfileRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6CohortInvariantResidueSurfaceRow {
    pub surface_id: String,
    pub pair_families: Vec<String>,
    pub row_count: usize,
    pub target_pair_count: usize,
    pub comparison_pair_count: usize,
    pub exact_separator_count: usize,
    pub best_modulus: Option<u32>,
    pub best_target_residues: Vec<u32>,
    pub surface_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6CohortInvariantResidueProfileRow {
    pub surface_id: String,
    pub modulus: u32,
    pub target_pair_count: usize,
    pub comparison_pair_count: usize,
    pub target_residues: Vec<u32>,
    pub comparison_residues: Vec<u32>,
    pub shared_residues: Vec<u32>,
    pub separator_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6CohortInvariantResidueSeparator {
    pub modulus: u32,
    pub target_residues: Vec<u32>,
    pub surface_count: usize,
    pub exact_surface_count: usize,
    pub target_pair_count: usize,
    pub comparison_pair_count: usize,
    pub separator_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6CohortInvariantNextPicker {
    pub selection_rule: String,
    pub excluded_profile_count: usize,
    pub excluded_profile_rows: Vec<ConnectorWidth6CohortInvariantExcludedProfileRow>,
    pub candidate_count: usize,
    pub stable_candidate_count: usize,
    pub selected_candidate: Option<ConnectorWidth6RatioGeometryPickerCandidateRow>,
    pub picker_decision: String,
    pub next_experiment_target: String,
    pub candidate_rows: Vec<ConnectorWidth6RatioGeometryPickerCandidateRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6CohortInvariantExcludedProfileRow {
    pub width: u32,
    pub connector: String,
    pub direction: String,
    pub profile_status: String,
    pub profile_decision: String,
    pub next_experiment_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6CohortInvariantThreeNullConclusion {
    pub selection_rule: String,
    pub collapsed_direction: String,
    pub collapsed_profile_count: usize,
    pub collapsed_profile_rows: Vec<ConnectorWidth6CohortInvariantExcludedProfileRow>,
    pub conclusion_status: String,
    pub conclusion_decision: String,
    pub forward_candidate_count: usize,
    pub selected_forward_candidate: Option<ConnectorWidth6RatioGeometryPickerCandidateRow>,
    pub next_experiment_target: String,
    pub forward_candidate_rows: Vec<ConnectorWidth6RatioGeometryPickerCandidateRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6CohortInvariantForwardNullConclusion {
    pub selection_rule: String,
    pub collapsed_direction: String,
    pub collapsed_profile_count: usize,
    pub collapsed_profile_rows: Vec<ConnectorWidth6CohortInvariantExcludedProfileRow>,
    pub conclusion_status: String,
    pub conclusion_decision: String,
    pub remaining_stable_candidate_count: usize,
    pub next_experiment_target: String,
    pub remaining_stable_candidate_rows: Vec<ConnectorWidth6RatioGeometryPickerCandidateRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyGapCohortWindowConsensusSurface {
    pub selection_rule: String,
    pub source_route_status: String,
    pub candidate_count: usize,
    pub surface_count: usize,
    pub surfaces: Vec<String>,
    pub window_count: usize,
    pub selected_candidate: Option<ConnectorWidth6WindowConsensusCandidateRow>,
    pub surface_decision: String,
    pub next_experiment_target: String,
    pub candidate_rows: Vec<ConnectorWidth6WindowConsensusCandidateRow>,
    pub surface_rows: Vec<ConnectorWidth6WindowConsensusSurfaceRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6WindowConsensusCandidateRow {
    pub candidate_rank: usize,
    pub width: u32,
    pub connector: String,
    pub shared_consensus_direction: String,
    pub consensus_surface_count: usize,
    pub consensus_window_count: usize,
    pub opposite_window_count: usize,
    pub neutral_window_count: usize,
    pub source_consensus_window_count: usize,
    pub fresh_consensus_window_count: usize,
    pub separator_consensus_window_count: usize,
    pub absolute_hit_delta_total: usize,
    pub candidate_status: String,
    pub next_experiment_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6WindowConsensusSurfaceRow {
    pub width: u32,
    pub connector: String,
    pub surface_id: String,
    pub pair_families: Vec<String>,
    pub window_count: usize,
    pub reverse_biased_window_count: usize,
    pub forward_biased_window_count: usize,
    pub neutral_window_count: usize,
    pub consensus_direction: String,
    pub consensus_window_count: usize,
    pub opposite_window_count: usize,
    pub source_consensus_window_count: usize,
    pub fresh_consensus_window_count: usize,
    pub separator_consensus_window_count: usize,
    pub surface_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyGapCohortWindowConsensusStress {
    pub selection_rule: String,
    pub source_width: u32,
    pub source_connector: String,
    pub source_direction: String,
    pub source_target: String,
    pub surface_count: usize,
    pub surfaces: Vec<String>,
    pub pair_family_count: usize,
    pub window_count: usize,
    pub retained_surface_count: usize,
    pub split_surface_count: usize,
    pub collapsed_surface_count: usize,
    pub stress_status: String,
    pub stress_decision: String,
    pub next_experiment_target: String,
    pub surface_rows: Vec<ConnectorWidth6WindowConsensusSurfaceRow>,
    pub family_rows: Vec<ConnectorWidth6PairFamilyGapCohortRatioGeometryFamilyRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyGapCohortSignPersistencePicker {
    pub selection_rule: String,
    pub source_route_status: String,
    pub candidate_count: usize,
    pub surface_count: usize,
    pub surfaces: Vec<String>,
    pub pair_family_count: usize,
    pub window_count: usize,
    pub persistent_candidate_count: usize,
    pub selected_candidate: Option<ConnectorWidth6SignPersistenceCandidateRow>,
    pub picker_decision: String,
    pub next_experiment_target: String,
    pub candidate_rows: Vec<ConnectorWidth6SignPersistenceCandidateRow>,
    pub surface_rows: Vec<ConnectorWidth6SignPersistenceSurfaceRow>,
    pub family_rows: Vec<ConnectorWidth6PairFamilyGapCohortRatioGeometryFamilyRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyGapCohortSignPersistenceStress {
    pub selection_rule: String,
    pub source_width: u32,
    pub source_connector: String,
    pub source_direction: String,
    pub source_target: String,
    pub surface_count: usize,
    pub surfaces: Vec<String>,
    pub pair_family_count: usize,
    pub window_count: usize,
    pub retained_surface_count: usize,
    pub split_surface_count: usize,
    pub neutral_surface_count: usize,
    pub retained_window_count: usize,
    pub opposite_window_count: usize,
    pub neutral_window_count: usize,
    pub signed_hit_delta_total: i32,
    pub absolute_hit_delta_total: usize,
    pub stress_status: String,
    pub stress_decision: String,
    pub next_experiment_target: String,
    pub surface_rows: Vec<ConnectorWidth6SignPersistenceSurfaceRow>,
    pub family_rows: Vec<ConnectorWidth6PairFamilyGapCohortRatioGeometryFamilyRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyGapCohortVolatilityEnsemblePicker {
    pub selection_rule: String,
    pub source_route_status: String,
    pub candidate_count: usize,
    pub surface_count: usize,
    pub surfaces: Vec<String>,
    pub pair_family_count: usize,
    pub window_count: usize,
    pub ensemble_count: usize,
    pub qualifying_ensemble_count: usize,
    pub selected_ensemble: Option<ConnectorWidth6VolatilityEnsembleRow>,
    pub picker_decision: String,
    pub next_experiment_target: String,
    pub candidate_rows: Vec<ConnectorWidth6VolatilityEnsembleCandidateRow>,
    pub ensemble_rows: Vec<ConnectorWidth6VolatilityEnsembleRow>,
    pub surface_rows: Vec<ConnectorWidth6SignPersistenceSurfaceRow>,
    pub family_rows: Vec<ConnectorWidth6PairFamilyGapCohortRatioGeometryFamilyRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyGapCohortVolatilityEnsembleStress {
    pub selection_rule: String,
    pub source_direction: String,
    pub source_connector_count: usize,
    pub source_connectors: Vec<String>,
    pub source_supported_surface_count: usize,
    pub source_target: String,
    pub surface_count: usize,
    pub surfaces: Vec<String>,
    pub pair_family_count: usize,
    pub connector_surface_row_count: usize,
    pub family_window_row_count: usize,
    pub retained_surface_count: usize,
    pub mixed_retained_surface_count: usize,
    pub split_surface_count: usize,
    pub collapsed_surface_count: usize,
    pub retained_connector_total: usize,
    pub opposite_connector_total: usize,
    pub neutral_connector_total: usize,
    pub retained_window_total: usize,
    pub opposite_window_total: usize,
    pub neutral_window_total: usize,
    pub signed_hit_delta_total: i32,
    pub absolute_hit_delta_total: usize,
    pub stress_status: String,
    pub stress_decision: String,
    pub next_experiment_target: String,
    pub surface_rows: Vec<ConnectorWidth6VolatilityEnsembleStressSurfaceRow>,
    pub connector_surface_rows: Vec<ConnectorWidth6SignPersistenceSurfaceRow>,
    pub family_rows: Vec<ConnectorWidth6PairFamilyGapCohortRatioGeometryFamilyRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6VolatilityEnsembleStressSurfaceRow {
    pub surface_id: String,
    pub pair_families: Vec<String>,
    pub connector_count: usize,
    pub retained_connector_count: usize,
    pub opposite_connector_count: usize,
    pub neutral_connector_count: usize,
    pub retained_window_total: usize,
    pub opposite_window_total: usize,
    pub neutral_window_total: usize,
    pub signed_hit_delta_total: i32,
    pub absolute_hit_delta_total: usize,
    pub surface_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyGapCohortSurfaceFamilyContrastPicker {
    pub selection_rule: String,
    pub source_route_status: String,
    pub source_direction: String,
    pub source_connector_count: usize,
    pub source_surface_count: usize,
    pub surface_family_count: usize,
    pub connector_surface_row_count: usize,
    pub retained_family_count: usize,
    pub split_family_count: usize,
    pub mixed_family_count: usize,
    pub selected_family: Option<String>,
    pub opposite_family: Option<String>,
    pub contrast_status: String,
    pub picker_decision: String,
    pub next_experiment_target: String,
    pub family_rows: Vec<ConnectorWidth6SurfaceFamilyContrastFamilyRow>,
    pub surface_rows: Vec<ConnectorWidth6SurfaceFamilyContrastSurfaceRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyGapCohortSurfaceFamilyContrastStress {
    pub selection_rule: String,
    pub source_route_status: String,
    pub source_direction: String,
    pub source_connector_count: usize,
    pub source_selected_family: String,
    pub source_opposite_family: String,
    pub source_target: String,
    pub surface_count: usize,
    pub surface_family_count: usize,
    pub pair_family_count: usize,
    pub connector_surface_row_count: usize,
    pub family_window_row_count: usize,
    pub retained_family_count: usize,
    pub split_family_count: usize,
    pub mixed_family_count: usize,
    pub retained_surface_count: usize,
    pub split_surface_count: usize,
    pub mixed_surface_count: usize,
    pub retained_connector_total: usize,
    pub opposite_connector_total: usize,
    pub neutral_connector_total: usize,
    pub retained_window_total: usize,
    pub opposite_window_total: usize,
    pub neutral_window_total: usize,
    pub signed_hit_delta_total: i32,
    pub absolute_hit_delta_total: usize,
    pub stress_status: String,
    pub stress_decision: String,
    pub next_experiment_target: String,
    pub family_rows: Vec<ConnectorWidth6SurfaceFamilyContrastFamilyRow>,
    pub surface_rows: Vec<ConnectorWidth6SurfaceFamilyContrastSurfaceRow>,
    pub connector_surface_rows: Vec<ConnectorWidth6SignPersistenceSurfaceRow>,
    pub family_window_rows: Vec<ConnectorWidth6PairFamilyGapCohortRatioGeometryFamilyRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyGapCohortSurfaceFamilyContrastAnatomy {
    pub selection_rule: String,
    pub source_status: String,
    pub source_direction: String,
    pub source_connector_count: usize,
    pub source_selected_family: String,
    pub source_opposite_family: String,
    pub source_target: String,
    pub connector_count: usize,
    pub full_contrast_driver_count: usize,
    pub gap_only_driver_count: usize,
    pub size_only_driver_count: usize,
    pub neutral_or_mixed_count: usize,
    pub total_driver_score: usize,
    pub top_driver_score: usize,
    pub top_driver_share_basis_points: usize,
    pub concentration_status: String,
    pub anatomy_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6SurfaceFamilyContrastAnatomyRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyGapCohortSurfaceFamilyDriverCohortStress {
    pub selection_rule: String,
    pub source_status: String,
    pub source_direction: String,
    pub source_selected_family: String,
    pub source_opposite_family: String,
    pub source_target: String,
    pub driver_count: usize,
    pub driver_connectors: Vec<String>,
    pub surface_count: usize,
    pub surface_family_count: usize,
    pub pair_family_count: usize,
    pub connector_surface_row_count: usize,
    pub family_window_row_count: usize,
    pub retained_family_count: usize,
    pub split_family_count: usize,
    pub mixed_family_count: usize,
    pub retained_surface_count: usize,
    pub split_surface_count: usize,
    pub mixed_surface_count: usize,
    pub retained_connector_total: usize,
    pub opposite_connector_total: usize,
    pub neutral_connector_total: usize,
    pub retained_window_total: usize,
    pub opposite_window_total: usize,
    pub neutral_window_total: usize,
    pub signed_hit_delta_total: i32,
    pub absolute_hit_delta_total: usize,
    pub stress_status: String,
    pub stress_decision: String,
    pub next_experiment_target: String,
    pub family_rows: Vec<ConnectorWidth6SurfaceFamilyContrastFamilyRow>,
    pub surface_rows: Vec<ConnectorWidth6SurfaceFamilyContrastSurfaceRow>,
    pub connector_surface_rows: Vec<ConnectorWidth6SignPersistenceSurfaceRow>,
    pub family_window_rows: Vec<ConnectorWidth6PairFamilyGapCohortRatioGeometryFamilyRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyGapCohortSurfaceFamilyMatchedNonDriverControlStress {
    pub selection_rule: String,
    pub source_driver_stress_status: String,
    pub source_driver_stress_decision: String,
    pub source_direction: String,
    pub source_target: String,
    pub driver_count: usize,
    pub driver_connectors: Vec<String>,
    pub control_count: usize,
    pub control_connectors: Vec<String>,
    pub surface_count: usize,
    pub surface_family_count: usize,
    pub pair_family_count: usize,
    pub connector_surface_row_count: usize,
    pub family_window_row_count: usize,
    pub retained_family_count: usize,
    pub split_family_count: usize,
    pub mixed_family_count: usize,
    pub retained_surface_count: usize,
    pub split_surface_count: usize,
    pub mixed_surface_count: usize,
    pub retained_connector_total: usize,
    pub opposite_connector_total: usize,
    pub neutral_connector_total: usize,
    pub retained_window_total: usize,
    pub opposite_window_total: usize,
    pub neutral_window_total: usize,
    pub signed_hit_delta_total: i32,
    pub absolute_hit_delta_total: usize,
    pub control_status: String,
    pub control_decision: String,
    pub next_experiment_target: String,
    pub family_rows: Vec<ConnectorWidth6SurfaceFamilyContrastFamilyRow>,
    pub surface_rows: Vec<ConnectorWidth6SurfaceFamilyContrastSurfaceRow>,
    pub connector_surface_rows: Vec<ConnectorWidth6SignPersistenceSurfaceRow>,
    pub family_window_rows: Vec<ConnectorWidth6PairFamilyGapCohortRatioGeometryFamilyRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6SurfaceFamilyContrastAnatomyRow {
    pub anatomy_rank: usize,
    pub width: u32,
    pub connector: String,
    pub connector_label: String,
    pub gap_surface_direction: String,
    pub size_surface_direction: String,
    pub gap_retained_window_count: usize,
    pub gap_opposite_window_count: usize,
    pub gap_neutral_window_count: usize,
    pub size_retained_window_count: usize,
    pub size_opposite_window_count: usize,
    pub size_neutral_window_count: usize,
    pub gap_driver_score: usize,
    pub size_driver_score: usize,
    pub total_driver_score: usize,
    pub gap_absolute_delta: usize,
    pub size_absolute_delta: usize,
    pub contrast_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6SurfaceFamilyContrastFamilyRow {
    pub surface_family: String,
    pub surface_count: usize,
    pub surfaces: Vec<String>,
    pub connector_surface_count: usize,
    pub retained_surface_count: usize,
    pub split_surface_count: usize,
    pub mixed_surface_count: usize,
    pub retained_connector_total: usize,
    pub opposite_connector_total: usize,
    pub neutral_connector_total: usize,
    pub retained_window_total: usize,
    pub opposite_window_total: usize,
    pub neutral_window_total: usize,
    pub signed_hit_delta_total: i32,
    pub absolute_hit_delta_total: usize,
    pub surface_family_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6SurfaceFamilyContrastSurfaceRow {
    pub surface_id: String,
    pub surface_family: String,
    pub connector_count: usize,
    pub retained_connector_count: usize,
    pub opposite_connector_count: usize,
    pub neutral_connector_count: usize,
    pub retained_window_total: usize,
    pub opposite_window_total: usize,
    pub neutral_window_total: usize,
    pub signed_hit_delta_total: i32,
    pub absolute_hit_delta_total: usize,
    pub surface_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6VolatilityEnsembleCandidateRow {
    pub candidate_rank: usize,
    pub width: u32,
    pub connector: String,
    pub retained_direction: String,
    pub retained_surface_count: usize,
    pub opposite_surface_count: usize,
    pub neutral_surface_count: usize,
    pub retained_window_count: usize,
    pub opposite_window_count: usize,
    pub neutral_window_count: usize,
    pub signed_hit_delta_total: i32,
    pub absolute_hit_delta_total: usize,
    pub volatility_score: usize,
    pub candidate_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6VolatilityEnsembleRow {
    pub ensemble_rank: usize,
    pub direction: String,
    pub supported_surface_count: usize,
    pub supported_surfaces: Vec<String>,
    pub connector_count: usize,
    pub selected_connectors: Vec<String>,
    pub retained_window_total: usize,
    pub opposite_window_total: usize,
    pub neutral_window_total: usize,
    pub signed_hit_delta_total: i32,
    pub absolute_hit_delta_total: usize,
    pub ensemble_status: String,
    pub next_experiment_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6PairFamilyGapCohortSurfaceAgnosticEnsemblePicker {
    pub selection_rule: String,
    pub source_route_status: String,
    pub source_target: String,
    pub candidate_count: usize,
    pub stable_connector_count: usize,
    pub surface_count: usize,
    pub surfaces: Vec<String>,
    pub pair_family_count: usize,
    pub window_count: usize,
    pub ensemble_count: usize,
    pub qualifying_ensemble_count: usize,
    pub selected_ensemble: Option<ConnectorWidth6SurfaceAgnosticEnsembleRow>,
    pub picker_decision: String,
    pub next_experiment_target: String,
    pub candidate_rows: Vec<ConnectorWidth6SurfaceAgnosticCandidateRow>,
    pub ensemble_rows: Vec<ConnectorWidth6SurfaceAgnosticEnsembleRow>,
    pub surface_rows: Vec<ConnectorWidth6SignPersistenceSurfaceRow>,
    pub family_rows: Vec<ConnectorWidth6PairFamilyGapCohortRatioGeometryFamilyRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6SurfaceAgnosticCandidateRow {
    pub candidate_rank: usize,
    pub width: u32,
    pub connector: String,
    pub connector_label: String,
    pub retained_direction: String,
    pub reverse_surface_count: usize,
    pub forward_surface_count: usize,
    pub retained_surface_count: usize,
    pub opposite_surface_count: usize,
    pub neutral_surface_count: usize,
    pub retained_window_count: usize,
    pub opposite_window_count: usize,
    pub neutral_window_count: usize,
    pub signed_hit_delta_total: i32,
    pub absolute_hit_delta_total: usize,
    pub volatility_score: usize,
    pub stability_status: String,
    pub next_experiment_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6SurfaceAgnosticEnsembleRow {
    pub ensemble_rank: usize,
    pub direction: String,
    pub connector_count: usize,
    pub stable_surface_total: usize,
    pub selected_connectors: Vec<String>,
    pub retained_window_total: usize,
    pub opposite_window_total: usize,
    pub neutral_window_total: usize,
    pub signed_hit_delta_total: i32,
    pub absolute_hit_delta_total: usize,
    pub ensemble_status: String,
    pub next_experiment_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6SignPersistenceCandidateRow {
    pub candidate_rank: usize,
    pub width: u32,
    pub connector: String,
    pub persistent_direction: String,
    pub persistent_surface_count: usize,
    pub opposite_surface_count: usize,
    pub neutral_surface_count: usize,
    pub retained_window_count: usize,
    pub opposite_window_count: usize,
    pub neutral_window_count: usize,
    pub signed_hit_delta_total: i32,
    pub absolute_hit_delta_total: usize,
    pub volatility_score: usize,
    pub candidate_status: String,
    pub next_experiment_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6SignPersistenceSurfaceRow {
    pub width: u32,
    pub connector: String,
    pub surface_id: String,
    pub pair_families: Vec<String>,
    pub pair_family_count: usize,
    pub window_count: usize,
    pub reverse_biased_window_count: usize,
    pub forward_biased_window_count: usize,
    pub neutral_window_count: usize,
    pub signed_hit_delta_total: i32,
    pub absolute_hit_delta_total: usize,
    pub surface_direction: String,
    pub surface_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6OrthogonalPairFamilyRetirementSummaryRow {
    pub source_branch_id: String,
    pub pair_family: String,
    pub pair_gap: u32,
    pub width: u32,
    pub start_position: u32,
    pub motif_digits: Vec<u8>,
    pub connector: String,
    pub source_modulus: u32,
    pub source_reverse_only_residues: Vec<u32>,
    pub separator_status: String,
    pub retirement_decision: String,
    pub retirement_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6ArithmeticFamilyRegistry {
    pub selection_rule: String,
    pub retired_family_count: usize,
    pub active_family_count: usize,
    pub selected_next_family_id: Option<String>,
    pub selected_next_family_target: Option<String>,
    pub registry_decision: String,
    pub rows: Vec<ConnectorWidth6ArithmeticFamilyRegistryRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6ArithmeticFamilyRegistryRow {
    pub family_id: String,
    pub family_label: String,
    pub family_class: String,
    pub status: String,
    pub evidence_branch_id: Option<String>,
    pub evidence_modulus: Option<u32>,
    pub evidence_decision: Option<String>,
    pub evidence_status: Option<String>,
    pub next_target: String,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6StressMetaAtlas {
    pub selection_rule: String,
    pub retired_branch_class_count: usize,
    pub active_branch_class_count: usize,
    pub theorem_candidate_branch_class_count: usize,
    pub selected_surface: Option<String>,
    pub selected_target: Option<String>,
    pub atlas_decision: String,
    pub rows: Vec<ConnectorWidth6StressMetaAtlasRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorWidth6StressMetaAtlasRow {
    pub branch_class_id: String,
    pub branch_class_label: String,
    pub surface: String,
    pub status: String,
    pub evidence_branch_id: Option<String>,
    pub evidence_decision: Option<String>,
    pub evidence_status: Option<String>,
    pub next_target: String,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6NonMod3CandidateResidueProfile {
    pub selection_rule: String,
    pub source_candidate_id: String,
    pub edge: String,
    pub width: u32,
    pub position: u32,
    pub digit: u8,
    pub connector: String,
    pub moduli: Vec<u32>,
    pub reverse_only_pair_count: usize,
    pub comparison_pair_count: usize,
    pub profiled_modulus_count: usize,
    pub exact_separator_count: usize,
    pub best_separator: Option<ConnectorWidth6NonMod3CandidateResidueProfileRow>,
    pub profile_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6NonMod3CandidateResidueProfileRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6NonMod3CandidateResidueProfileRow {
    pub modulus: u32,
    pub reverse_only_pair_count: usize,
    pub comparison_pair_count: usize,
    pub reverse_only_residues: Vec<u32>,
    pub comparison_residues: Vec<u32>,
    pub shared_residues: Vec<u32>,
    pub separator_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6NonMod3ResidueSeparatorReplication {
    pub selection_rule: String,
    pub source_candidate_id: String,
    pub edge: String,
    pub width: u32,
    pub position: u32,
    pub digit: u8,
    pub connector: String,
    pub source_modulus: u32,
    pub source_reverse_only_residues: Vec<u32>,
    pub fresh_pair_count: usize,
    pub fresh_pair_labels: Vec<String>,
    pub row_count: usize,
    pub reverse_only_pair_count: usize,
    pub comparison_pair_count: usize,
    pub reverse_only_residues: Vec<u32>,
    pub comparison_residues: Vec<u32>,
    pub shared_residues: Vec<u32>,
    pub retained_residue_count: usize,
    pub split_residue_count: usize,
    pub collapsed_residue_count: usize,
    pub separator_status: String,
    pub replication_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6PeakMatchedControlRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6PeakMatchedControlScreen {
    pub selection_rule: String,
    pub small_prime_bound: u32,
    pub small_primes: Vec<u32>,
    pub peak_pair_label: String,
    pub peak_left: u128,
    pub peak_right: u128,
    pub peak_width: u32,
    pub peak_position: u32,
    pub peak_digit: u8,
    pub peak_connector: String,
    pub peak_signed_observed_to_corrected_ratio_gap: f64,
    pub exact_layer_decision: String,
    pub exact_layer_lean_module: String,
    pub exact_layer_lean_theorems: Vec<String>,
    pub row_count: usize,
    pub aligned_row_count: usize,
    pub aligned_non_peak_row_count: usize,
    pub mod3_exceptional_row_count: usize,
    pub mod3_theorem_blocked_row_count: usize,
    pub control_summaries: Vec<ConnectorWidth6PeakMatchedControlSummary>,
    pub next_nonblocked_candidate_selection_rule: String,
    pub next_nonblocked_candidate: Option<ConnectorWidth6PeakMatchedControlRow>,
    pub adjacent_width_follow_up: Option<ConnectorWidth6PeakAdjacentWidthFollowUp>,
    pub width_extension_probe: Option<ConnectorWidth6PeakWidthExtensionProbe>,
    pub leading_digit_width_probe: Option<ConnectorWidth6PeakLeadingDigitWidthProbe>,
    pub position_digit_probe: Option<ConnectorWidth6PeakPositionDigitProbe>,
    pub width7_position_digit_probe: Option<ConnectorWidth6PeakPositionDigitProbe>,
    pub width_position_spread_comparison: Option<ConnectorWidth6PeakWidthPositionSpreadComparison>,
    pub edge_position_probe: Option<ConnectorWidth6PeakEdgePositionProbe>,
    pub edge_pair_replication_probe: Option<ConnectorWidth6PeakEdgePairReplicationProbe>,
    pub digit8_edge_zoom_probe: Option<ConnectorWidth6PeakDigit8EdgeZoomProbe>,
    pub selected_branch_independent_replication:
        Option<ConnectorWidth6SelectedBranchIndependentReplication>,
    pub non_mod3_candidate_picker: Option<ConnectorWidth6NonMod3CandidatePicker>,
    pub non_mod3_candidate_second_replication:
        Option<ConnectorWidth6NonMod3CandidateSecondReplication>,
    pub non_mod3_candidate_residue_profile: Option<ConnectorWidth6NonMod3CandidateResidueProfile>,
    pub non_mod3_residue_separator_replication:
        Option<ConnectorWidth6NonMod3ResidueSeparatorReplication>,
    pub non_mod3_mutated_residue_separator_replication:
        Option<ConnectorWidth6NonMod3ResidueSeparatorReplication>,
    pub non_mod3_next_candidate_picker: Option<ConnectorWidth6NonMod3CandidatePicker>,
    pub non_mod3_next_candidate_independent_replication:
        Option<ConnectorWidth6NonMod3CandidateSecondReplication>,
    pub non_mod3_retirement_summary: Vec<ConnectorWidth6NonMod3RetirementSummaryRow>,
    pub interior_non_mod3_family_picker: Option<ConnectorWidth6InteriorNonMod3FamilyPicker>,
    pub interior_non_mod3_family_independent_replication:
        Option<ConnectorWidth6InteriorNonMod3FamilyReplication>,
    pub interior_non_mod3_residue_profile: Option<ConnectorWidth6NonMod3CandidateResidueProfile>,
    pub interior_non_mod3_residue_separator_replication:
        Option<ConnectorWidth6NonMod3ResidueSeparatorReplication>,
    pub interior_non_mod3_retirement_summary: Vec<ConnectorWidth6NonMod3RetirementSummaryRow>,
    pub interior_non_mod3_next_family_picker: Option<ConnectorWidth6InteriorNonMod3FamilyPicker>,
    pub interior_non_mod3_next_family_independent_replication:
        Option<ConnectorWidth6InteriorNonMod3FamilyReplication>,
    pub interior_non_mod3_next_residue_profile:
        Option<ConnectorWidth6NonMod3CandidateResidueProfile>,
    pub interior_non_mod3_next_residue_separator_replication:
        Option<ConnectorWidth6NonMod3ResidueSeparatorReplication>,
    pub interior_non_mod3_post_retirement_family_picker:
        Option<ConnectorWidth6InteriorNonMod3FamilyPicker>,
    pub interior_non_mod3_post_retirement_family_independent_replication:
        Option<ConnectorWidth6InteriorNonMod3FamilyReplication>,
    pub interior_non_mod3_post_retirement_residue_profile:
        Option<ConnectorWidth6NonMod3CandidateResidueProfile>,
    pub interior_non_mod3_post_retirement_residue_separator_replication:
        Option<ConnectorWidth6NonMod3ResidueSeparatorReplication>,
    pub interior_non_mod3_after_third_retirement_family_picker:
        Option<ConnectorWidth6InteriorNonMod3FamilyPicker>,
    pub interior_non_mod3_after_third_retirement_family_independent_replication:
        Option<ConnectorWidth6InteriorNonMod3FamilyReplication>,
    pub interior_non_mod3_after_third_retirement_residue_profile:
        Option<ConnectorWidth6NonMod3CandidateResidueProfile>,
    pub interior_non_mod3_after_third_retirement_residue_separator_replication:
        Option<ConnectorWidth6NonMod3ResidueSeparatorReplication>,
    pub interior_non_mod3_after_fourth_retirement_family_picker:
        Option<ConnectorWidth6InteriorNonMod3FamilyPicker>,
    pub interior_non_mod3_after_fourth_retirement_family_independent_replication:
        Option<ConnectorWidth6InteriorNonMod3FamilyReplication>,
    pub interior_non_mod3_after_fourth_retirement_residue_profile:
        Option<ConnectorWidth6NonMod3CandidateResidueProfile>,
    pub interior_non_mod3_after_fourth_retirement_residue_separator_replication:
        Option<ConnectorWidth6NonMod3ResidueSeparatorReplication>,
    pub interior_non_mod3_after_fifth_retirement_family_picker:
        Option<ConnectorWidth6InteriorNonMod3FamilyPicker>,
    pub interior_non_mod3_after_fifth_retirement_family_independent_replication:
        Option<ConnectorWidth6InteriorNonMod3FamilyReplication>,
    pub interior_non_mod3_after_sixth_retirement_family_picker:
        Option<ConnectorWidth6InteriorNonMod3FamilyPicker>,
    pub interior_non_mod3_after_sixth_retirement_family_independent_replication:
        Option<ConnectorWidth6InteriorNonMod3FamilyReplication>,
    pub single_digit_interior_pivot_decision: String,
    pub multi_digit_motif_family_picker: Option<ConnectorWidth6MultiDigitMotifFamilyPicker>,
    pub multi_digit_motif_family_independent_replication:
        Option<ConnectorWidth6MultiDigitMotifReplication>,
    pub multi_digit_motif_residue_profile: Option<ConnectorWidth6MultiDigitMotifResidueProfile>,
    pub multi_digit_motif_residue_separator_replication:
        Option<ConnectorWidth6MultiDigitMotifResidueSeparatorReplication>,
    pub multi_digit_motif_retirement_summary:
        Vec<ConnectorWidth6MultiDigitMotifRetirementSummaryRow>,
    pub orthogonal_pair_family_control_matrix:
        Option<ConnectorWidth6OrthogonalPairFamilyControlMatrix>,
    pub orthogonal_pair_family_residue_profile:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueProfile>,
    pub orthogonal_pair_family_residue_separator_replication:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueSeparatorReplication>,
    pub orthogonal_pair_family_retirement_summary:
        Vec<ConnectorWidth6OrthogonalPairFamilyRetirementSummaryRow>,
    pub orthogonal_compact_three_digit_control:
        Option<ConnectorWidth6OrthogonalPairFamilyControlMatrix>,
    pub orthogonal_compact_three_digit_residue_profile:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueProfile>,
    pub orthogonal_compact_three_digit_residue_separator_replication:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueSeparatorReplication>,
    pub orthogonal_nonadjacent_two_digit_control:
        Option<ConnectorWidth6OrthogonalPairFamilyControlMatrix>,
    pub orthogonal_nonadjacent_two_digit_residue_profile:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueProfile>,
    pub orthogonal_nonadjacent_two_digit_residue_separator_replication:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueSeparatorReplication>,
    pub orthogonal_edge_plus_interior_control:
        Option<ConnectorWidth6OrthogonalPairFamilyControlMatrix>,
    pub orthogonal_edge_plus_interior_residue_profile:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueProfile>,
    pub orthogonal_edge_plus_interior_residue_separator_replication:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueSeparatorReplication>,
    pub orthogonal_repeated_block_control: Option<ConnectorWidth6OrthogonalPairFamilyControlMatrix>,
    pub orthogonal_repeated_block_residue_profile:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueProfile>,
    pub orthogonal_repeated_block_residue_separator_replication:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueSeparatorReplication>,
    pub orthogonal_arithmetic_connector_control:
        Option<ConnectorWidth6OrthogonalPairFamilyControlMatrix>,
    pub orthogonal_arithmetic_connector_residue_profile:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueProfile>,
    pub orthogonal_arithmetic_connector_residue_separator_replication:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueSeparatorReplication>,
    pub orthogonal_residue_lattice_connector_control:
        Option<ConnectorWidth6OrthogonalPairFamilyControlMatrix>,
    pub orthogonal_residue_lattice_connector_residue_profile:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueProfile>,
    pub orthogonal_residue_lattice_connector_residue_separator_replication:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueSeparatorReplication>,
    pub orthogonal_modular_walk_connector_control:
        Option<ConnectorWidth6OrthogonalPairFamilyControlMatrix>,
    pub orthogonal_modular_walk_connector_residue_profile:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueProfile>,
    pub orthogonal_modular_walk_connector_residue_separator_replication:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueSeparatorReplication>,
    pub orthogonal_arithmetic_family_registry: Option<ConnectorWidth6ArithmeticFamilyRegistry>,
    pub orthogonal_crt_paired_connector_control:
        Option<ConnectorWidth6OrthogonalPairFamilyControlMatrix>,
    pub orthogonal_crt_paired_connector_residue_profile:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueProfile>,
    pub orthogonal_crt_paired_connector_residue_separator_replication:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueSeparatorReplication>,
    pub orthogonal_multiplicative_order_connector_control:
        Option<ConnectorWidth6OrthogonalPairFamilyControlMatrix>,
    pub orthogonal_multiplicative_order_connector_residue_profile:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueProfile>,
    pub orthogonal_multiplicative_order_connector_residue_separator_replication:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueSeparatorReplication>,
    pub orthogonal_automorphic_repunit_connector_control:
        Option<ConnectorWidth6OrthogonalPairFamilyControlMatrix>,
    pub orthogonal_automorphic_repunit_connector_residue_profile:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueProfile>,
    pub orthogonal_automorphic_repunit_connector_residue_separator_replication:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueSeparatorReplication>,
    pub orthogonal_cyclic_reptend_connector_control:
        Option<ConnectorWidth6OrthogonalPairFamilyControlMatrix>,
    pub orthogonal_cyclic_reptend_connector_residue_profile:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueProfile>,
    pub orthogonal_cyclic_reptend_connector_residue_separator_replication:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueSeparatorReplication>,
    pub orthogonal_carry_chain_connector_control:
        Option<ConnectorWidth6OrthogonalPairFamilyControlMatrix>,
    pub orthogonal_carry_chain_connector_residue_profile:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueProfile>,
    pub orthogonal_carry_chain_connector_residue_separator_replication:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueSeparatorReplication>,
    pub orthogonal_base_mixed_connector_control:
        Option<ConnectorWidth6OrthogonalPairFamilyControlMatrix>,
    pub orthogonal_base_mixed_connector_residue_profile:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueProfile>,
    pub orthogonal_base_mixed_connector_residue_separator_replication:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueSeparatorReplication>,
    pub connector_stress_meta_atlas: Option<ConnectorWidth6StressMetaAtlas>,
    pub pair_family_gap_portfolio_control: Option<ConnectorWidth6OrthogonalPairFamilyControlMatrix>,
    pub pair_family_gap_portfolio_residue_profile:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueProfile>,
    pub pair_family_gap_portfolio_residue_separator_replication:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueSeparatorReplication>,
    pub pair_family_gap_extension_control: Option<ConnectorWidth6OrthogonalPairFamilyControlMatrix>,
    pub pair_family_gap_extension_residue_profile:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueProfile>,
    pub pair_family_gap_extension_residue_separator_replication:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueSeparatorReplication>,
    pub pair_family_size_band_control: Option<ConnectorWidth6OrthogonalPairFamilyControlMatrix>,
    pub pair_family_size_band_residue_profile:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueProfile>,
    pub pair_family_size_band_residue_separator_replication:
        Option<ConnectorWidth6OrthogonalPairFamilyResidueSeparatorReplication>,
    pub pair_family_cohort_retention_picker: Option<ConnectorWidth6PairFamilyCohortRetentionPicker>,
    pub pair_family_cohort_residue_profile: Option<ConnectorWidth6PairFamilyCohortResidueProfile>,
    pub pair_family_cohort_residue_separator_replication:
        Option<ConnectorWidth6PairFamilyCohortResidueSeparatorReplication>,
    pub pair_family_surface_picker: Option<ConnectorWidth6PairFamilySurfacePicker>,
    pub pair_family_surface_residue_profile: Option<ConnectorWidth6PairFamilySurfaceResidueProfile>,
    pub pair_family_topn_motif_surface_profile:
        Option<ConnectorWidth6PairFamilyTopNMotifSurfaceProfile>,
    pub pair_family_gap_cohort_geometry_control:
        Option<ConnectorWidth6PairFamilyGapCohortGeometryControl>,
    pub pair_family_gap_cohort_residue_profile:
        Option<ConnectorWidth6PairFamilyCohortResidueProfile>,
    pub pair_family_gap_cohort_residue_separator_replication:
        Option<ConnectorWidth6PairFamilyCohortResidueSeparatorReplication>,
    pub pair_family_gap_cohort_ratio_geometry_control:
        Option<ConnectorWidth6PairFamilyGapCohortRatioGeometryControl>,
    pub pair_family_gap_cohort_ratio_geometry_replication:
        Option<ConnectorWidth6PairFamilyGapCohortRatioGeometryReplication>,
    pub pair_family_gap_cohort_ratio_geometry_expansion:
        Option<ConnectorWidth6PairFamilyGapCohortRatioGeometryExpansion>,
    pub pair_family_gap_cohort_ratio_correction_bound_stability:
        Option<ConnectorWidth6PairFamilyGapCohortRatioCorrectionBoundStability>,
    pub pair_family_gap_cohort_ratio_geometry_atlas:
        Option<ConnectorWidth6PairFamilyGapCohortRatioGeometryAtlas>,
    pub pair_family_gap_cohort_ratio_geometry_picker:
        Option<ConnectorWidth6PairFamilyGapCohortRatioGeometryPicker>,
    pub pair_family_gap_cohort_ratio_geometry_residue_profile:
        Option<ConnectorWidth6CohortInvariantResidueProfile>,
    pub pair_family_gap_cohort_ratio_geometry_next_picker:
        Option<ConnectorWidth6CohortInvariantNextPicker>,
    pub pair_family_gap_cohort_ratio_geometry_next_residue_profile:
        Option<ConnectorWidth6CohortInvariantResidueProfile>,
    pub pair_family_gap_cohort_ratio_geometry_post_two_null_picker:
        Option<ConnectorWidth6CohortInvariantNextPicker>,
    pub pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile:
        Option<ConnectorWidth6CohortInvariantResidueProfile>,
    pub pair_family_gap_cohort_ratio_geometry_three_null_conclusion:
        Option<ConnectorWidth6CohortInvariantThreeNullConclusion>,
    pub pair_family_gap_cohort_ratio_geometry_forward_residue_profile:
        Option<ConnectorWidth6CohortInvariantResidueProfile>,
    pub pair_family_gap_cohort_ratio_geometry_forward_null_conclusion:
        Option<ConnectorWidth6CohortInvariantForwardNullConclusion>,
    pub pair_family_gap_cohort_window_consensus_surface:
        Option<ConnectorWidth6PairFamilyGapCohortWindowConsensusSurface>,
    pub pair_family_gap_cohort_window_consensus_stress:
        Option<ConnectorWidth6PairFamilyGapCohortWindowConsensusStress>,
    pub pair_family_gap_cohort_sign_persistence_picker:
        Option<ConnectorWidth6PairFamilyGapCohortSignPersistencePicker>,
    pub pair_family_gap_cohort_sign_persistence_stress:
        Option<ConnectorWidth6PairFamilyGapCohortSignPersistenceStress>,
    pub pair_family_gap_cohort_volatility_ensemble_picker:
        Option<ConnectorWidth6PairFamilyGapCohortVolatilityEnsemblePicker>,
    pub pair_family_gap_cohort_volatility_ensemble_stress:
        Option<ConnectorWidth6PairFamilyGapCohortVolatilityEnsembleStress>,
    pub pair_family_gap_cohort_surface_family_contrast_picker:
        Option<ConnectorWidth6PairFamilyGapCohortSurfaceFamilyContrastPicker>,
    pub pair_family_gap_cohort_surface_family_contrast_stress:
        Option<ConnectorWidth6PairFamilyGapCohortSurfaceFamilyContrastStress>,
    pub pair_family_gap_cohort_surface_family_contrast_anatomy:
        Option<ConnectorWidth6PairFamilyGapCohortSurfaceFamilyContrastAnatomy>,
    pub pair_family_gap_cohort_surface_family_driver_cohort_stress:
        Option<ConnectorWidth6PairFamilyGapCohortSurfaceFamilyDriverCohortStress>,
    pub pair_family_gap_cohort_surface_family_matched_nondriver_control_stress:
        Option<ConnectorWidth6PairFamilyGapCohortSurfaceFamilyMatchedNonDriverControlStress>,
    pub pair_family_gap_cohort_surface_agnostic_ensemble_picker:
        Option<ConnectorWidth6PairFamilyGapCohortSurfaceAgnosticEnsemblePicker>,
    pub branch_status_picker: Option<ConnectorWidth6StressBranchStatusPicker>,
    pub screen_decision: String,
    pub next_experiment_target: String,
    pub rows: Vec<ConnectorWidth6PeakMatchedControlRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorWidth6StressReport {
    pub schema_version: String,
    pub artifact_id: String,
    pub generator_command: String,
    pub settings: ConnectorWidth6StressSettings,
    pub selected_pair_label: String,
    pub selected_pair_summary: ConnectorWidth6StressPairSummary,
    pub selected_vs_controls_status: String,
    pub ladder_pattern_status: String,
    pub median_gap_monotone_by_pair_size: bool,
    pub selected_median_absolute_gap_rank: usize,
    pub signed_gap_sign_sequence: Vec<String>,
    pub ladder_peak_follow_up: Option<ConnectorWidth6PeakFollowUp>,
    pub ladder_peak_matched_control_screen: Option<ConnectorWidth6PeakMatchedControlScreen>,
    pub replication_null_atlas: ConnectorReplicationNullAtlas,
    pub target_decision: String,
    pub pair_summaries: Vec<ConnectorWidth6StressPairSummary>,
    pub rows: Vec<ConnectorWidth6StressRow>,
    pub claim_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorReplicationNullAtlas {
    pub schema_version: String,
    pub artifact_id: String,
    pub source_artifact_id: String,
    pub source_schema_version: String,
    pub generator_command: String,
    pub drift_check_command: String,
    pub summary: ConnectorReplicationNullAtlasSummary,
    pub rows: Vec<ConnectorReplicationNullAtlasRow>,
    pub claim_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorReplicationNullAtlasSummary {
    pub branch_row_count: usize,
    pub class_level_row_count: usize,
    pub source_stage_row_count: usize,
    pub fresh_stage_row_count: usize,
    pub profile_stage_row_count: usize,
    pub separator_stage_row_count: usize,
    pub separator_retained_count: usize,
    pub separator_split_count: usize,
    pub separator_collapsed_count: usize,
    pub theorem_candidate_count: usize,
    pub empirical_only_count: usize,
    pub single_branch_separator_stability_status: String,
    pub next_experiment_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorReplicationNullAtlasRow {
    pub branch_id: String,
    pub branch_class: String,
    pub surface: String,
    pub pair_family: Option<String>,
    pub pair_gap: Option<u32>,
    pub width: Option<u32>,
    pub connector: Option<String>,
    pub source_stage_status: String,
    pub source_reverse_only_pair_count: Option<usize>,
    pub fresh_stage_status: String,
    pub fresh_reverse_only_pair_count: Option<usize>,
    pub fresh_forward_only_pair_count: Option<usize>,
    pub profile_stage_status: String,
    pub best_modulus: Option<u32>,
    pub source_reverse_only_residues: Vec<u32>,
    pub separator_stage_status: String,
    pub separator_reverse_only_residues: Vec<u32>,
    pub shared_residues: Vec<u32>,
    pub replication_decision: Option<String>,
    pub theorem_readiness: String,
    pub next_target: String,
    pub interpretation: String,
}

#[cfg(not(clippy))]
#[path = "connector_signal_impl.rs"]
mod connector_signal_impl;
#[cfg(not(clippy))]
pub use connector_signal_impl::*;

#[cfg(clippy)]
pub fn maintained_connector_pair_cases() -> Vec<PairCase> {
    Vec::new()
}

#[cfg(clippy)]
pub fn connector_width6_stress_pair_cases() -> Vec<PairCase> {
    Vec::new()
}

#[cfg(clippy)]
pub fn connector_signal_settings(_small_prime_bound: u32) -> SignalReportSettings {
    panic!("clippy-only connector signal shim")
}

#[cfg(clippy)]
pub fn build_connector_signal_analysis(_small_prime_bound: u32) -> ConnectorSignalAnalysis {
    panic!("clippy-only connector signal shim")
}

#[cfg(clippy)]
pub fn build_signal_report_bundle(_analysis: &ConnectorSignalAnalysis) -> SignalReportBundle {
    panic!("clippy-only connector signal shim")
}

#[cfg(clippy)]
pub fn build_connector_signal_atlas() -> ConnectorSignalAtlas {
    panic!("clippy-only connector signal shim")
}

#[cfg(clippy)]
pub fn build_connector_width6_stress_report() -> ConnectorWidth6StressReport {
    panic!("clippy-only connector stress shim")
}

#[cfg(clippy)]
pub fn build_connector_replication_null_atlas() -> ConnectorReplicationNullAtlas {
    panic!("clippy-only connector replication-null shim")
}

#[cfg(clippy)]
pub fn connector_signal_proof_links() -> Vec<ConnectorSignalProofLink> {
    Vec::new()
}

#[cfg(clippy)]
pub fn connector_signal_claim_status() -> ConnectorSignalClaimStatus {
    panic!("clippy-only connector signal shim")
}

#[cfg(clippy)]
pub fn build_position_export_rows(_reports: &[NamedPairReport]) -> Vec<PositionExportRow> {
    Vec::new()
}

#[cfg(clippy)]
pub fn build_sweep_export_rows(_reports: &[NamedPairReport]) -> Vec<ResidualSweepExportRow> {
    Vec::new()
}

#[cfg(clippy)]
pub fn build_residue_survivor_rows(
    _reports: &[NamedPairReport],
) -> Vec<ConnectorResidueSurvivorRow> {
    Vec::new()
}

#[cfg(clippy)]
pub fn build_residual_boundary_rows(
    _reports: &[NamedPairReport],
    _residue_survivor_rows: &[ConnectorResidueSurvivorRow],
) -> Vec<ConnectorResidualBoundaryRow> {
    Vec::new()
}

#[cfg(clippy)]
pub fn pick_residual_target(
    _rows: &[ConnectorResidualBoundaryRow],
) -> Option<ConnectorResidualTargetPick> {
    None
}

#[cfg(clippy)]
pub fn build_residual_target_follow_up(
    _reports: &[NamedPairReport],
    _target: Option<&ConnectorResidualTargetPick>,
) -> Option<ConnectorResidualTargetFollowUp> {
    None
}

#[cfg(clippy)]
pub fn pick_width_contrast(
    _rows: &[ConnectorResidualTargetWidthRow],
) -> Option<ConnectorResidualTargetWidthContrastPick> {
    None
}

#[cfg(clippy)]
pub fn build_width_contrast_micro_atlas(
    _target: &ConnectorResidualTargetPick,
    _report: &NamedPairReport,
    _pick: Option<&ConnectorResidualTargetWidthContrastPick>,
) -> Option<ConnectorResidualTargetWidthContrastMicroAtlas> {
    None
}

#[cfg(clippy)]
pub fn build_position_digit_contrast_rows(
    _report: &NamedPairReport,
    _width_pick: &ConnectorResidualTargetWidthContrastPick,
) -> Vec<ConnectorResidualTargetPositionDigitContrastRow> {
    Vec::new()
}

#[cfg(clippy)]
pub fn pick_position_digit_contrast(
    _rows: &[ConnectorResidualTargetPositionDigitContrastRow],
    _direction: Direction,
) -> Option<ConnectorResidualTargetPositionDigitContrastPick> {
    None
}

#[cfg(clippy)]
pub fn summarize_residual_sweep(_report: &NamedPairReport) -> ResidualSweepSummary {
    panic!("clippy-only connector signal shim")
}

#[cfg(clippy)]
pub fn render_connector_signal_atlas_markdown(_atlas: &ConnectorSignalAtlas) -> String {
    String::new()
}

#[cfg(clippy)]
pub fn render_connector_width6_stress_markdown(_report: &ConnectorWidth6StressReport) -> String {
    String::new()
}

#[cfg(clippy)]
pub fn render_connector_replication_null_atlas_markdown(
    _atlas: &ConnectorReplicationNullAtlas,
) -> String {
    String::new()
}

#[cfg(clippy)]
pub fn render_connector_signal_atlas_lean_checks(
    _atlas: &ConnectorSignalAtlas,
) -> Result<String, String> {
    Ok(String::new())
}

#[cfg(clippy)]
pub fn render_connector_width6_stress_lean_checks(
    _report: &ConnectorWidth6StressReport,
) -> Result<String, String> {
    Ok(String::new())
}

#[cfg(clippy)]
pub fn canonical_width5_hits() -> &'static [(u32, u32, u8)] {
    &[]
}
