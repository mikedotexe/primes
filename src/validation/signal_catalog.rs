//! Lightweight top-level index over maintained signal/proof artifacts.

use crate::validation::connector_signal::build_connector_width6_stress_report;
use serde::{Deserialize, Serialize};
use std::{
    path::Path,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

pub const SIGNAL_CATALOG_SCHEMA_VERSION: &str = "signal-catalog-v76";
pub const SIGNAL_CATALOG_ARTIFACT_ID: &str = "signal-catalog";
pub const SIGNAL_CATALOG_GENERATOR_COMMAND: &str =
    "cargo run --bin export_signal_catalog -- --out-dir docs/signal_catalog";
pub const SIGNAL_CATALOG_DRIFT_CHECK_COMMAND: &str = "scripts/signal_catalog.sh verify";
pub const KNOWN_SIGNAL_CATALOG_DRIFT_CHECK_COMMANDS: &[&str] = &[
    "scripts/matched_control_atlas_bridge.sh verify",
    "scripts/proof_carrying_witness.sh verify",
    "scripts/lean_proof_carrying_witness_certificate.sh verify",
    "scripts/connector_signal_atlas.sh verify",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalCatalog {
    pub schema_version: String,
    pub artifact_id: String,
    pub generator_command: String,
    pub drift_check_command: String,
    pub summary: SignalCatalogSummary,
    pub connector_digit8_classifier_family: SignalCatalogConnectorDigit8ClassifierFamily,
    pub rows: Vec<SignalCatalogRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalCatalogSummary {
    pub row_count: usize,
    pub matched_control_rows: usize,
    pub witness_rows: usize,
    pub connector_rows: usize,
    pub proof_carrying_rows: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalCatalogRow {
    pub signal_id: String,
    pub domain: String,
    pub artifact_path: String,
    pub drift_check_command: String,
    pub claim_status: String,
    pub proof_status: String,
    pub empirical_status: String,
    pub next_theorem_target: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalCatalogConnectorDigit8ClassifierFamily {
    pub source_artifact_path: String,
    pub source_schema_version: String,
    pub surface_status: String,
    pub theorem_backed_multi_modulus_cell_count: usize,
    pub unclassified_exact_separator_count: usize,
    pub replication_selection_rule: String,
    pub outside_ladder_replication_decision: String,
    pub outside_ladder_baseline_pair_count: usize,
    pub outside_ladder_widened_pair_count: usize,
    pub outside_ladder_added_pair_count: usize,
    pub outside_ladder_tested_cell_count: usize,
    pub outside_ladder_retained_cell_count: usize,
    pub outside_ladder_split_cell_count: usize,
    pub outside_ladder_collapsed_cell_count: usize,
    pub split_follow_up_decision: String,
    pub split_follow_up_source_row_count: usize,
    pub split_follow_up_tested_row_count: usize,
    pub split_follow_up_stabilized_row_count: usize,
    pub split_follow_up_split_again_row_count: usize,
    pub split_follow_up_collapsed_row_count: usize,
    pub branch_picker_decision: String,
    pub selected_next_branch_id: Option<String>,
    pub selected_next_branch_status: Option<String>,
    pub selected_next_branch_target: Option<String>,
    pub selected_branch_independent_replication_decision: Option<String>,
    pub selected_branch_independent_replication_next_target: Option<String>,
    pub retired_branch_id: Option<String>,
    pub non_mod3_candidate_picker_decision: Option<String>,
    pub retired_non_mod3_candidate_count: usize,
    pub retired_non_mod3_candidate_ids: Vec<String>,
    pub selected_non_mod3_candidate_id: Option<String>,
    pub selected_non_mod3_candidate_target: Option<String>,
    pub non_mod3_second_replication_decision: Option<String>,
    pub non_mod3_second_replication_next_target: Option<String>,
    pub non_mod3_residue_profile_decision: Option<String>,
    pub non_mod3_residue_profile_best_modulus: Option<u32>,
    pub non_mod3_residue_profile_next_target: Option<String>,
    pub non_mod3_residue_separator_replication_decision: Option<String>,
    pub non_mod3_residue_separator_replication_status: Option<String>,
    pub non_mod3_residue_separator_replication_next_target: Option<String>,
    pub non_mod3_mutated_residue_separator_replication_decision: Option<String>,
    pub non_mod3_mutated_residue_separator_replication_status: Option<String>,
    pub non_mod3_mutated_residue_separator_replication_next_target: Option<String>,
    pub next_non_mod3_candidate_picker_decision: Option<String>,
    pub selected_next_non_mod3_candidate_id: Option<String>,
    pub selected_next_non_mod3_candidate_target: Option<String>,
    pub next_non_mod3_independent_replication_decision: Option<String>,
    pub next_non_mod3_independent_replication_target: Option<String>,
    pub non_mod3_retired_edge_candidate_count: usize,
    pub non_mod3_retired_edge_candidate_ids: Vec<String>,
    pub interior_non_mod3_family_picker_decision: Option<String>,
    pub selected_interior_non_mod3_candidate_id: Option<String>,
    pub selected_interior_non_mod3_candidate_target: Option<String>,
    pub interior_non_mod3_family_independent_replication_decision: Option<String>,
    pub interior_non_mod3_family_independent_replication_target: Option<String>,
    pub interior_non_mod3_residue_profile_decision: Option<String>,
    pub interior_non_mod3_residue_profile_best_modulus: Option<u32>,
    pub interior_non_mod3_residue_profile_target: Option<String>,
    pub interior_non_mod3_residue_separator_replication_decision: Option<String>,
    pub interior_non_mod3_residue_separator_replication_status: Option<String>,
    pub interior_non_mod3_residue_separator_replication_target: Option<String>,
    pub interior_non_mod3_retired_candidate_count: usize,
    pub interior_non_mod3_retired_candidate_ids: Vec<String>,
    pub interior_non_mod3_next_family_picker_decision: Option<String>,
    pub selected_next_interior_non_mod3_candidate_id: Option<String>,
    pub selected_next_interior_non_mod3_candidate_target: Option<String>,
    pub interior_non_mod3_next_family_independent_replication_decision: Option<String>,
    pub interior_non_mod3_next_family_independent_replication_target: Option<String>,
    pub interior_non_mod3_next_residue_profile_decision: Option<String>,
    pub interior_non_mod3_next_residue_profile_best_modulus: Option<u32>,
    pub interior_non_mod3_next_residue_profile_target: Option<String>,
    pub interior_non_mod3_next_residue_separator_replication_decision: Option<String>,
    pub interior_non_mod3_next_residue_separator_replication_status: Option<String>,
    pub interior_non_mod3_next_residue_separator_replication_target: Option<String>,
    pub interior_non_mod3_post_retirement_family_picker_decision: Option<String>,
    pub selected_post_retirement_interior_non_mod3_candidate_id: Option<String>,
    pub selected_post_retirement_interior_non_mod3_candidate_target: Option<String>,
    pub interior_non_mod3_post_retirement_family_independent_replication_decision: Option<String>,
    pub interior_non_mod3_post_retirement_family_independent_replication_target: Option<String>,
    pub interior_non_mod3_post_retirement_residue_profile_decision: Option<String>,
    pub interior_non_mod3_post_retirement_residue_profile_best_modulus: Option<u32>,
    pub interior_non_mod3_post_retirement_residue_profile_target: Option<String>,
    pub interior_non_mod3_post_retirement_residue_separator_replication_decision: Option<String>,
    pub interior_non_mod3_post_retirement_residue_separator_replication_status: Option<String>,
    pub interior_non_mod3_post_retirement_residue_separator_replication_target: Option<String>,
    pub interior_non_mod3_after_third_retirement_family_picker_decision: Option<String>,
    pub selected_after_third_retirement_interior_non_mod3_candidate_id: Option<String>,
    pub selected_after_third_retirement_interior_non_mod3_candidate_target: Option<String>,
    pub interior_non_mod3_after_third_retirement_family_independent_replication_decision:
        Option<String>,
    pub interior_non_mod3_after_third_retirement_family_independent_replication_target:
        Option<String>,
    pub interior_non_mod3_after_third_retirement_residue_profile_decision: Option<String>,
    pub interior_non_mod3_after_third_retirement_residue_profile_best_modulus: Option<u32>,
    pub interior_non_mod3_after_third_retirement_residue_profile_target: Option<String>,
    pub interior_non_mod3_after_third_retirement_residue_separator_replication_decision:
        Option<String>,
    pub interior_non_mod3_after_third_retirement_residue_separator_replication_status:
        Option<String>,
    pub interior_non_mod3_after_third_retirement_residue_separator_replication_target:
        Option<String>,
    pub interior_non_mod3_after_fourth_retirement_family_picker_decision: Option<String>,
    pub selected_after_fourth_retirement_interior_non_mod3_candidate_id: Option<String>,
    pub selected_after_fourth_retirement_interior_non_mod3_candidate_target: Option<String>,
    pub interior_non_mod3_after_fourth_retirement_family_independent_replication_decision:
        Option<String>,
    pub interior_non_mod3_after_fourth_retirement_family_independent_replication_target:
        Option<String>,
    pub interior_non_mod3_after_fourth_retirement_residue_profile_decision: Option<String>,
    pub interior_non_mod3_after_fourth_retirement_residue_profile_best_modulus: Option<u32>,
    pub interior_non_mod3_after_fourth_retirement_residue_profile_target: Option<String>,
    pub interior_non_mod3_after_fourth_retirement_residue_separator_replication_decision:
        Option<String>,
    pub interior_non_mod3_after_fourth_retirement_residue_separator_replication_status:
        Option<String>,
    pub interior_non_mod3_after_fourth_retirement_residue_separator_replication_target:
        Option<String>,
    pub interior_non_mod3_after_fifth_retirement_family_picker_decision: Option<String>,
    pub selected_after_fifth_retirement_interior_non_mod3_candidate_id: Option<String>,
    pub selected_after_fifth_retirement_interior_non_mod3_candidate_target: Option<String>,
    pub interior_non_mod3_after_fifth_retirement_family_independent_replication_decision:
        Option<String>,
    pub interior_non_mod3_after_fifth_retirement_family_independent_replication_target:
        Option<String>,
    pub interior_non_mod3_after_sixth_retirement_family_picker_decision: Option<String>,
    pub selected_after_sixth_retirement_interior_non_mod3_candidate_id: Option<String>,
    pub selected_after_sixth_retirement_interior_non_mod3_candidate_target: Option<String>,
    pub interior_non_mod3_after_sixth_retirement_family_independent_replication_decision:
        Option<String>,
    pub interior_non_mod3_after_sixth_retirement_family_independent_replication_target:
        Option<String>,
    pub single_digit_interior_pivot_decision: Option<String>,
    pub multi_digit_motif_family_picker_decision: Option<String>,
    pub selected_multi_digit_motif_id: Option<String>,
    pub selected_multi_digit_motif_target: Option<String>,
    pub multi_digit_motif_family_independent_replication_decision: Option<String>,
    pub multi_digit_motif_family_independent_replication_target: Option<String>,
    pub multi_digit_motif_residue_profile_decision: Option<String>,
    pub multi_digit_motif_residue_profile_best_modulus: Option<u32>,
    pub multi_digit_motif_residue_profile_target: Option<String>,
    pub multi_digit_motif_residue_separator_replication_decision: Option<String>,
    pub multi_digit_motif_residue_separator_replication_status: Option<String>,
    pub multi_digit_motif_residue_separator_replication_target: Option<String>,
    pub multi_digit_motif_retired_count: usize,
    pub retired_multi_digit_motif_ids: Vec<String>,
    pub orthogonal_pair_family_retired_count: usize,
    pub retired_orthogonal_pair_family_branch_ids: Vec<String>,
    pub orthogonal_pair_family_control_matrix_decision: Option<String>,
    pub selected_orthogonal_pair_family_branch_id: Option<String>,
    pub selected_orthogonal_pair_family: Option<String>,
    pub selected_orthogonal_pair_family_connector: Option<String>,
    pub selected_orthogonal_pair_family_target: Option<String>,
    pub orthogonal_pair_family_residue_profile_decision: Option<String>,
    pub orthogonal_pair_family_residue_profile_best_modulus: Option<u32>,
    pub orthogonal_pair_family_residue_profile_target: Option<String>,
    pub orthogonal_pair_family_residue_separator_replication_decision: Option<String>,
    pub orthogonal_pair_family_residue_separator_replication_status: Option<String>,
    pub orthogonal_pair_family_residue_separator_replication_target: Option<String>,
    pub orthogonal_compact_three_digit_control_decision: Option<String>,
    pub selected_orthogonal_compact_three_digit_branch_id: Option<String>,
    pub selected_orthogonal_compact_three_digit_family: Option<String>,
    pub selected_orthogonal_compact_three_digit_connector: Option<String>,
    pub selected_orthogonal_compact_three_digit_target: Option<String>,
    pub orthogonal_compact_three_digit_residue_profile_decision: Option<String>,
    pub orthogonal_compact_three_digit_residue_profile_best_modulus: Option<u32>,
    pub orthogonal_compact_three_digit_residue_profile_target: Option<String>,
    pub orthogonal_compact_three_digit_residue_separator_replication_decision: Option<String>,
    pub orthogonal_compact_three_digit_residue_separator_replication_status: Option<String>,
    pub orthogonal_compact_three_digit_residue_separator_replication_target: Option<String>,
    pub orthogonal_nonadjacent_two_digit_control_decision: Option<String>,
    pub selected_orthogonal_nonadjacent_two_digit_branch_id: Option<String>,
    pub selected_orthogonal_nonadjacent_two_digit_family: Option<String>,
    pub selected_orthogonal_nonadjacent_two_digit_connector: Option<String>,
    pub selected_orthogonal_nonadjacent_two_digit_target: Option<String>,
    pub orthogonal_nonadjacent_two_digit_residue_profile_decision: Option<String>,
    pub orthogonal_nonadjacent_two_digit_residue_profile_best_modulus: Option<u32>,
    pub orthogonal_nonadjacent_two_digit_residue_profile_target: Option<String>,
    pub orthogonal_nonadjacent_two_digit_residue_separator_replication_decision: Option<String>,
    pub orthogonal_nonadjacent_two_digit_residue_separator_replication_status: Option<String>,
    pub orthogonal_nonadjacent_two_digit_residue_separator_replication_target: Option<String>,
    pub orthogonal_edge_plus_interior_control_decision: Option<String>,
    pub selected_orthogonal_edge_plus_interior_branch_id: Option<String>,
    pub selected_orthogonal_edge_plus_interior_family: Option<String>,
    pub selected_orthogonal_edge_plus_interior_connector: Option<String>,
    pub selected_orthogonal_edge_plus_interior_target: Option<String>,
    pub orthogonal_edge_plus_interior_residue_profile_decision: Option<String>,
    pub orthogonal_edge_plus_interior_residue_profile_best_modulus: Option<u32>,
    pub orthogonal_edge_plus_interior_residue_profile_target: Option<String>,
    pub orthogonal_edge_plus_interior_residue_separator_replication_decision: Option<String>,
    pub orthogonal_edge_plus_interior_residue_separator_replication_status: Option<String>,
    pub orthogonal_edge_plus_interior_residue_separator_replication_target: Option<String>,
    pub orthogonal_repeated_block_control_decision: Option<String>,
    pub selected_orthogonal_repeated_block_branch_id: Option<String>,
    pub selected_orthogonal_repeated_block_family: Option<String>,
    pub selected_orthogonal_repeated_block_connector: Option<String>,
    pub selected_orthogonal_repeated_block_target: Option<String>,
    pub orthogonal_repeated_block_residue_profile_decision: Option<String>,
    pub orthogonal_repeated_block_residue_profile_best_modulus: Option<u32>,
    pub orthogonal_repeated_block_residue_profile_target: Option<String>,
    pub orthogonal_repeated_block_residue_separator_replication_decision: Option<String>,
    pub orthogonal_repeated_block_residue_separator_replication_status: Option<String>,
    pub orthogonal_repeated_block_residue_separator_replication_target: Option<String>,
    pub orthogonal_arithmetic_connector_control_decision: Option<String>,
    pub selected_orthogonal_arithmetic_connector_branch_id: Option<String>,
    pub selected_orthogonal_arithmetic_connector_family: Option<String>,
    pub selected_orthogonal_arithmetic_connector_connector: Option<String>,
    pub selected_orthogonal_arithmetic_connector_target: Option<String>,
    pub orthogonal_arithmetic_connector_residue_profile_decision: Option<String>,
    pub orthogonal_arithmetic_connector_residue_profile_best_modulus: Option<u32>,
    pub orthogonal_arithmetic_connector_residue_profile_target: Option<String>,
    pub orthogonal_arithmetic_connector_residue_separator_replication_decision: Option<String>,
    pub orthogonal_arithmetic_connector_residue_separator_replication_status: Option<String>,
    pub orthogonal_arithmetic_connector_residue_separator_replication_target: Option<String>,
    pub orthogonal_residue_lattice_connector_control_decision: Option<String>,
    pub selected_orthogonal_residue_lattice_connector_branch_id: Option<String>,
    pub selected_orthogonal_residue_lattice_connector_family: Option<String>,
    pub selected_orthogonal_residue_lattice_connector_connector: Option<String>,
    pub selected_orthogonal_residue_lattice_connector_target: Option<String>,
    pub orthogonal_residue_lattice_connector_residue_profile_decision: Option<String>,
    pub orthogonal_residue_lattice_connector_residue_profile_best_modulus: Option<u32>,
    pub orthogonal_residue_lattice_connector_residue_profile_target: Option<String>,
    pub orthogonal_residue_lattice_connector_residue_separator_replication_decision: Option<String>,
    pub orthogonal_residue_lattice_connector_residue_separator_replication_status: Option<String>,
    pub orthogonal_residue_lattice_connector_residue_separator_replication_target: Option<String>,
    pub orthogonal_modular_walk_connector_control_decision: Option<String>,
    pub selected_orthogonal_modular_walk_connector_branch_id: Option<String>,
    pub selected_orthogonal_modular_walk_connector_family: Option<String>,
    pub selected_orthogonal_modular_walk_connector_connector: Option<String>,
    pub selected_orthogonal_modular_walk_connector_target: Option<String>,
    pub orthogonal_modular_walk_connector_residue_profile_decision: Option<String>,
    pub orthogonal_modular_walk_connector_residue_profile_best_modulus: Option<u32>,
    pub orthogonal_modular_walk_connector_residue_profile_target: Option<String>,
    pub orthogonal_modular_walk_connector_residue_separator_replication_decision: Option<String>,
    pub orthogonal_modular_walk_connector_residue_separator_replication_status: Option<String>,
    pub orthogonal_modular_walk_connector_residue_separator_replication_target: Option<String>,
    pub orthogonal_arithmetic_family_registry_decision: Option<String>,
    pub orthogonal_arithmetic_family_registry_retired_count: Option<usize>,
    pub orthogonal_arithmetic_family_registry_selected_family: Option<String>,
    pub orthogonal_arithmetic_family_registry_selected_target: Option<String>,
    pub orthogonal_crt_paired_connector_control_decision: Option<String>,
    pub selected_orthogonal_crt_paired_connector_branch_id: Option<String>,
    pub selected_orthogonal_crt_paired_connector_family: Option<String>,
    pub selected_orthogonal_crt_paired_connector_connector: Option<String>,
    pub selected_orthogonal_crt_paired_connector_target: Option<String>,
    pub orthogonal_crt_paired_connector_residue_profile_decision: Option<String>,
    pub orthogonal_crt_paired_connector_residue_profile_best_modulus: Option<u32>,
    pub orthogonal_crt_paired_connector_residue_profile_target: Option<String>,
    pub orthogonal_crt_paired_connector_residue_separator_replication_decision: Option<String>,
    pub orthogonal_crt_paired_connector_residue_separator_replication_status: Option<String>,
    pub orthogonal_crt_paired_connector_residue_separator_replication_target: Option<String>,
    pub orthogonal_multiplicative_order_connector_control_decision: Option<String>,
    pub selected_orthogonal_multiplicative_order_connector_branch_id: Option<String>,
    pub selected_orthogonal_multiplicative_order_connector_family: Option<String>,
    pub selected_orthogonal_multiplicative_order_connector_connector: Option<String>,
    pub selected_orthogonal_multiplicative_order_connector_target: Option<String>,
    pub orthogonal_multiplicative_order_connector_residue_profile_decision: Option<String>,
    pub orthogonal_multiplicative_order_connector_residue_profile_best_modulus: Option<u32>,
    pub orthogonal_multiplicative_order_connector_residue_profile_target: Option<String>,
    pub orthogonal_multiplicative_order_connector_residue_separator_replication_decision:
        Option<String>,
    pub orthogonal_multiplicative_order_connector_residue_separator_replication_status:
        Option<String>,
    pub orthogonal_multiplicative_order_connector_residue_separator_replication_target:
        Option<String>,
    pub orthogonal_automorphic_repunit_connector_control_decision: Option<String>,
    pub selected_orthogonal_automorphic_repunit_connector_branch_id: Option<String>,
    pub selected_orthogonal_automorphic_repunit_connector_family: Option<String>,
    pub selected_orthogonal_automorphic_repunit_connector_connector: Option<String>,
    pub selected_orthogonal_automorphic_repunit_connector_target: Option<String>,
    pub orthogonal_automorphic_repunit_connector_residue_profile_decision: Option<String>,
    pub orthogonal_automorphic_repunit_connector_residue_profile_best_modulus: Option<u32>,
    pub orthogonal_automorphic_repunit_connector_residue_profile_target: Option<String>,
    pub orthogonal_automorphic_repunit_connector_residue_separator_replication_decision:
        Option<String>,
    pub orthogonal_automorphic_repunit_connector_residue_separator_replication_status:
        Option<String>,
    pub orthogonal_automorphic_repunit_connector_residue_separator_replication_target:
        Option<String>,
    pub orthogonal_cyclic_reptend_connector_control_decision: Option<String>,
    pub selected_orthogonal_cyclic_reptend_connector_branch_id: Option<String>,
    pub selected_orthogonal_cyclic_reptend_connector_family: Option<String>,
    pub selected_orthogonal_cyclic_reptend_connector_connector: Option<String>,
    pub selected_orthogonal_cyclic_reptend_connector_target: Option<String>,
    pub orthogonal_cyclic_reptend_connector_residue_profile_decision: Option<String>,
    pub orthogonal_cyclic_reptend_connector_residue_profile_best_modulus: Option<u32>,
    pub orthogonal_cyclic_reptend_connector_residue_profile_target: Option<String>,
    pub orthogonal_cyclic_reptend_connector_residue_separator_replication_decision: Option<String>,
    pub orthogonal_cyclic_reptend_connector_residue_separator_replication_status: Option<String>,
    pub orthogonal_cyclic_reptend_connector_residue_separator_replication_target: Option<String>,
    pub orthogonal_carry_chain_connector_control_decision: Option<String>,
    pub selected_orthogonal_carry_chain_connector_branch_id: Option<String>,
    pub selected_orthogonal_carry_chain_connector_family: Option<String>,
    pub selected_orthogonal_carry_chain_connector_connector: Option<String>,
    pub selected_orthogonal_carry_chain_connector_target: Option<String>,
    pub orthogonal_carry_chain_connector_residue_profile_decision: Option<String>,
    pub orthogonal_carry_chain_connector_residue_profile_best_modulus: Option<u32>,
    pub orthogonal_carry_chain_connector_residue_profile_target: Option<String>,
    pub orthogonal_carry_chain_connector_residue_separator_replication_decision: Option<String>,
    pub orthogonal_carry_chain_connector_residue_separator_replication_status: Option<String>,
    pub orthogonal_carry_chain_connector_residue_separator_replication_target: Option<String>,
    pub orthogonal_base_mixed_connector_control_decision: Option<String>,
    pub orthogonal_base_mixed_connector_control_target: Option<String>,
    pub selected_orthogonal_base_mixed_connector_branch_id: Option<String>,
    pub selected_orthogonal_base_mixed_connector_family: Option<String>,
    pub selected_orthogonal_base_mixed_connector_connector: Option<String>,
    pub selected_orthogonal_base_mixed_connector_target: Option<String>,
    pub orthogonal_base_mixed_connector_residue_profile_decision: Option<String>,
    pub orthogonal_base_mixed_connector_residue_profile_best_modulus: Option<u32>,
    pub orthogonal_base_mixed_connector_residue_profile_target: Option<String>,
    pub orthogonal_base_mixed_connector_residue_separator_replication_decision: Option<String>,
    pub orthogonal_base_mixed_connector_residue_separator_replication_status: Option<String>,
    pub orthogonal_base_mixed_connector_residue_separator_replication_target: Option<String>,
    pub connector_stress_meta_atlas_decision: Option<String>,
    pub connector_stress_meta_atlas_retired_count: Option<usize>,
    pub connector_stress_meta_atlas_selected_surface: Option<String>,
    pub connector_stress_meta_atlas_selected_target: Option<String>,
    pub pair_family_gap_portfolio_control_decision: Option<String>,
    pub pair_family_gap_portfolio_control_target: Option<String>,
    pub selected_pair_family_gap_portfolio_branch_id: Option<String>,
    pub selected_pair_family_gap_portfolio_family: Option<String>,
    pub selected_pair_family_gap_portfolio_connector: Option<String>,
    pub selected_pair_family_gap_portfolio_target: Option<String>,
    pub pair_family_gap_portfolio_residue_profile_decision: Option<String>,
    pub pair_family_gap_portfolio_residue_profile_best_modulus: Option<u32>,
    pub pair_family_gap_portfolio_residue_profile_target: Option<String>,
    pub pair_family_gap_portfolio_residue_separator_replication_decision: Option<String>,
    pub pair_family_gap_portfolio_residue_separator_replication_status: Option<String>,
    pub pair_family_gap_portfolio_residue_separator_replication_target: Option<String>,
    pub pair_family_gap_extension_control_decision: Option<String>,
    pub pair_family_gap_extension_control_target: Option<String>,
    pub selected_pair_family_gap_extension_branch_id: Option<String>,
    pub selected_pair_family_gap_extension_family: Option<String>,
    pub selected_pair_family_gap_extension_connector: Option<String>,
    pub selected_pair_family_gap_extension_target: Option<String>,
    pub pair_family_gap_extension_residue_profile_decision: Option<String>,
    pub pair_family_gap_extension_residue_profile_best_modulus: Option<u32>,
    pub pair_family_gap_extension_residue_profile_target: Option<String>,
    pub pair_family_gap_extension_residue_separator_replication_decision: Option<String>,
    pub pair_family_gap_extension_residue_separator_replication_status: Option<String>,
    pub pair_family_gap_extension_residue_separator_replication_target: Option<String>,
    pub pair_family_size_band_control_decision: Option<String>,
    pub pair_family_size_band_control_target: Option<String>,
    pub selected_pair_family_size_band_branch_id: Option<String>,
    pub selected_pair_family_size_band_family: Option<String>,
    pub selected_pair_family_size_band_connector: Option<String>,
    pub selected_pair_family_size_band_target: Option<String>,
    pub pair_family_size_band_residue_profile_decision: Option<String>,
    pub pair_family_size_band_residue_profile_best_modulus: Option<u32>,
    pub pair_family_size_band_residue_profile_target: Option<String>,
    pub pair_family_size_band_residue_separator_replication_decision: Option<String>,
    pub pair_family_size_band_residue_separator_replication_status: Option<String>,
    pub pair_family_size_band_residue_separator_replication_target: Option<String>,
    pub replication_null_atlas_schema_version: String,
    pub replication_null_atlas_status: String,
    pub replication_null_atlas_branch_row_count: usize,
    pub replication_null_atlas_retained_separator_count: usize,
    pub replication_null_atlas_split_separator_count: usize,
    pub replication_null_atlas_collapsed_separator_count: usize,
    pub replication_null_atlas_theorem_candidate_count: usize,
    pub replication_null_atlas_next_target: String,
    pub pair_family_cohort_retention_picker_decision: Option<String>,
    pub selected_pair_family_cohort_id: Option<String>,
    pub selected_pair_family_cohort_connector: Option<String>,
    pub selected_pair_family_cohort_target: Option<String>,
    pub pair_family_cohort_residue_profile_decision: Option<String>,
    pub pair_family_cohort_residue_profile_exact_separator_count: Option<usize>,
    pub pair_family_cohort_residue_profile_best_modulus: Option<u32>,
    pub pair_family_cohort_residue_profile_target: Option<String>,
    pub pair_family_cohort_residue_separator_replication_decision: Option<String>,
    pub pair_family_cohort_residue_separator_replication_status: Option<String>,
    pub pair_family_cohort_residue_separator_replication_target: Option<String>,
    pub pair_family_surface_picker_decision: Option<String>,
    pub selected_pair_family_surface_id: Option<String>,
    pub selected_pair_family_surface_label: Option<String>,
    pub selected_pair_family_surface_target: Option<String>,
    pub pair_family_surface_residue_profile_decision: Option<String>,
    pub pair_family_surface_residue_profile_exact_separator_count: Option<usize>,
    pub pair_family_surface_residue_profile_best_modulus: Option<u32>,
    pub pair_family_surface_residue_profile_target: Option<String>,
    pub pair_family_topn_motif_surface_profile_decision: Option<String>,
    pub pair_family_topn_motif_surface_profile_top_n: Option<usize>,
    pub pair_family_topn_motif_surface_profile_source_motif_count: Option<usize>,
    pub pair_family_topn_motif_surface_profile_fresh_survivor_count: Option<usize>,
    pub pair_family_topn_motif_surface_profile_exact_separator_count: Option<usize>,
    pub pair_family_topn_motif_surface_profile_best_modulus: Option<u32>,
    pub pair_family_topn_motif_surface_profile_target: Option<String>,
    pub pair_family_gap_cohort_geometry_control_decision: Option<String>,
    pub pair_family_gap_cohort_geometry_control_top_n: Option<usize>,
    pub pair_family_gap_cohort_geometry_control_source_motif_count: Option<usize>,
    pub pair_family_gap_cohort_geometry_control_retained_geometry_count: Option<usize>,
    pub pair_family_gap_cohort_geometry_control_selected_connector: Option<String>,
    pub pair_family_gap_cohort_geometry_control_target: Option<String>,
    pub pair_family_gap_cohort_residue_profile_decision: Option<String>,
    pub pair_family_gap_cohort_residue_profile_exact_separator_count: Option<usize>,
    pub pair_family_gap_cohort_residue_profile_best_modulus: Option<u32>,
    pub pair_family_gap_cohort_residue_profile_target: Option<String>,
    pub pair_family_gap_cohort_residue_separator_replication_decision: Option<String>,
    pub pair_family_gap_cohort_residue_separator_replication_status: Option<String>,
    pub pair_family_gap_cohort_residue_separator_replication_target: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_control_decision: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_control_selected_connector: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_control_selected_bias: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_control_target: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_replication_decision: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_replication_status: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_replication_target: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_expansion_decision: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_expansion_status: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_expansion_target: Option<String>,
    pub pair_family_gap_cohort_ratio_correction_bound_stability_decision: Option<String>,
    pub pair_family_gap_cohort_ratio_correction_bound_stability_status: Option<String>,
    pub pair_family_gap_cohort_ratio_correction_bound_stable_bound_count: Option<usize>,
    pub pair_family_gap_cohort_ratio_correction_bound_stability_target: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_atlas_decision: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_atlas_status: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_atlas_target: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_picker_decision: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_picker_stable_candidate_count: Option<usize>,
    pub pair_family_gap_cohort_ratio_geometry_picker_selected_connector: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_picker_selected_direction: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_picker_target: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_residue_profile_decision: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_residue_profile_status: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_residue_profile_best_modulus: Option<u32>,
    pub pair_family_gap_cohort_ratio_geometry_residue_profile_target: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_next_picker_decision: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_next_picker_excluded_profile_count: Option<usize>,
    pub pair_family_gap_cohort_ratio_geometry_next_picker_selected_connector: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_next_picker_selected_direction: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_next_picker_target: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_next_residue_profile_decision: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_next_residue_profile_status: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_next_residue_profile_best_modulus: Option<u32>,
    pub pair_family_gap_cohort_ratio_geometry_next_residue_profile_target: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_post_two_null_picker_decision: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_post_two_null_picker_excluded_profile_count:
        Option<usize>,
    pub pair_family_gap_cohort_ratio_geometry_post_two_null_picker_selected_connector:
        Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_post_two_null_picker_selected_direction:
        Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_post_two_null_picker_target: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile_decision:
        Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile_status: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile_best_modulus:
        Option<u32>,
    pub pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile_target: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_three_null_conclusion_decision: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_three_null_conclusion_status: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_three_null_conclusion_collapsed_profile_count:
        Option<usize>,
    pub pair_family_gap_cohort_ratio_geometry_three_null_conclusion_selected_connector:
        Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_three_null_conclusion_selected_direction:
        Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_three_null_conclusion_target: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_forward_residue_profile_decision: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_forward_residue_profile_status: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_forward_residue_profile_best_modulus: Option<u32>,
    pub pair_family_gap_cohort_ratio_geometry_forward_residue_profile_target: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_decision: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_status: Option<String>,
    pub pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_collapsed_profile_count:
        Option<usize>,
    pub pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_remaining_candidate_count:
        Option<usize>,
    pub pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_target: Option<String>,
    pub pair_family_gap_cohort_window_consensus_surface_decision: Option<String>,
    pub pair_family_gap_cohort_window_consensus_surface_candidate_count: Option<usize>,
    pub pair_family_gap_cohort_window_consensus_surface_selected_connector: Option<String>,
    pub pair_family_gap_cohort_window_consensus_surface_selected_direction: Option<String>,
    pub pair_family_gap_cohort_window_consensus_surface_selected_status: Option<String>,
    pub pair_family_gap_cohort_window_consensus_surface_selected_consensus_window_count:
        Option<usize>,
    pub pair_family_gap_cohort_window_consensus_surface_target: Option<String>,
    pub pair_family_gap_cohort_window_consensus_stress_status: Option<String>,
    pub pair_family_gap_cohort_window_consensus_stress_decision: Option<String>,
    pub pair_family_gap_cohort_window_consensus_stress_retained_surface_count: Option<usize>,
    pub pair_family_gap_cohort_window_consensus_stress_split_surface_count: Option<usize>,
    pub pair_family_gap_cohort_window_consensus_stress_collapsed_surface_count: Option<usize>,
    pub pair_family_gap_cohort_window_consensus_stress_target: Option<String>,
    pub pair_family_gap_cohort_sign_persistence_picker_decision: Option<String>,
    pub pair_family_gap_cohort_sign_persistence_picker_candidate_count: Option<usize>,
    pub pair_family_gap_cohort_sign_persistence_picker_persistent_candidate_count: Option<usize>,
    pub pair_family_gap_cohort_sign_persistence_picker_selected_connector: Option<String>,
    pub pair_family_gap_cohort_sign_persistence_picker_selected_direction: Option<String>,
    pub pair_family_gap_cohort_sign_persistence_picker_selected_status: Option<String>,
    pub pair_family_gap_cohort_sign_persistence_picker_selected_surface_count: Option<usize>,
    pub pair_family_gap_cohort_sign_persistence_picker_selected_volatility_score: Option<usize>,
    pub pair_family_gap_cohort_sign_persistence_picker_target: Option<String>,
    pub pair_family_gap_cohort_sign_persistence_stress_status: Option<String>,
    pub pair_family_gap_cohort_sign_persistence_stress_decision: Option<String>,
    pub pair_family_gap_cohort_sign_persistence_stress_retained_surface_count: Option<usize>,
    pub pair_family_gap_cohort_sign_persistence_stress_split_surface_count: Option<usize>,
    pub pair_family_gap_cohort_sign_persistence_stress_neutral_surface_count: Option<usize>,
    pub pair_family_gap_cohort_sign_persistence_stress_retained_window_count: Option<usize>,
    pub pair_family_gap_cohort_sign_persistence_stress_opposite_window_count: Option<usize>,
    pub pair_family_gap_cohort_sign_persistence_stress_target: Option<String>,
    pub pair_family_gap_cohort_volatility_ensemble_picker_decision: Option<String>,
    pub pair_family_gap_cohort_volatility_ensemble_picker_ensemble_count: Option<usize>,
    pub pair_family_gap_cohort_volatility_ensemble_picker_qualifying_ensemble_count: Option<usize>,
    pub pair_family_gap_cohort_volatility_ensemble_picker_selected_direction: Option<String>,
    pub pair_family_gap_cohort_volatility_ensemble_picker_selected_connector_count: Option<usize>,
    pub pair_family_gap_cohort_volatility_ensemble_picker_selected_supported_surface_count:
        Option<usize>,
    pub pair_family_gap_cohort_volatility_ensemble_picker_target: Option<String>,
    pub pair_family_gap_cohort_volatility_ensemble_stress_status: Option<String>,
    pub pair_family_gap_cohort_volatility_ensemble_stress_decision: Option<String>,
    pub pair_family_gap_cohort_volatility_ensemble_stress_selected_direction: Option<String>,
    pub pair_family_gap_cohort_volatility_ensemble_stress_selected_connector_count: Option<usize>,
    pub pair_family_gap_cohort_volatility_ensemble_stress_retained_surface_count: Option<usize>,
    pub pair_family_gap_cohort_volatility_ensemble_stress_mixed_retained_surface_count:
        Option<usize>,
    pub pair_family_gap_cohort_volatility_ensemble_stress_split_surface_count: Option<usize>,
    pub pair_family_gap_cohort_volatility_ensemble_stress_collapsed_surface_count: Option<usize>,
    pub pair_family_gap_cohort_volatility_ensemble_stress_retained_window_count: Option<usize>,
    pub pair_family_gap_cohort_volatility_ensemble_stress_opposite_window_count: Option<usize>,
    pub pair_family_gap_cohort_volatility_ensemble_stress_target: Option<String>,
    pub pair_family_gap_cohort_surface_family_contrast_picker_decision: Option<String>,
    pub pair_family_gap_cohort_surface_family_contrast_picker_status: Option<String>,
    pub pair_family_gap_cohort_surface_family_contrast_picker_selected_family: Option<String>,
    pub pair_family_gap_cohort_surface_family_contrast_picker_opposite_family: Option<String>,
    pub pair_family_gap_cohort_surface_family_contrast_picker_retained_family_count: Option<usize>,
    pub pair_family_gap_cohort_surface_family_contrast_picker_split_family_count: Option<usize>,
    pub pair_family_gap_cohort_surface_family_contrast_picker_target: Option<String>,
    pub pair_family_gap_cohort_surface_family_contrast_stress_status: Option<String>,
    pub pair_family_gap_cohort_surface_family_contrast_stress_decision: Option<String>,
    pub pair_family_gap_cohort_surface_family_contrast_stress_selected_family: Option<String>,
    pub pair_family_gap_cohort_surface_family_contrast_stress_opposite_family: Option<String>,
    pub pair_family_gap_cohort_surface_family_contrast_stress_retained_family_count: Option<usize>,
    pub pair_family_gap_cohort_surface_family_contrast_stress_split_family_count: Option<usize>,
    pub pair_family_gap_cohort_surface_family_contrast_stress_retained_window_count: Option<usize>,
    pub pair_family_gap_cohort_surface_family_contrast_stress_opposite_window_count: Option<usize>,
    pub pair_family_gap_cohort_surface_family_contrast_stress_target: Option<String>,
    pub pair_family_gap_cohort_surface_family_contrast_anatomy_concentration_status: Option<String>,
    pub pair_family_gap_cohort_surface_family_contrast_anatomy_decision: Option<String>,
    pub pair_family_gap_cohort_surface_family_contrast_anatomy_full_driver_count: Option<usize>,
    pub pair_family_gap_cohort_surface_family_contrast_anatomy_gap_only_driver_count: Option<usize>,
    pub pair_family_gap_cohort_surface_family_contrast_anatomy_size_only_driver_count:
        Option<usize>,
    pub pair_family_gap_cohort_surface_family_contrast_anatomy_top_driver_share_basis_points:
        Option<usize>,
    pub pair_family_gap_cohort_surface_family_contrast_anatomy_target: Option<String>,
    pub pair_family_gap_cohort_surface_family_driver_cohort_stress_status: Option<String>,
    pub pair_family_gap_cohort_surface_family_driver_cohort_stress_decision: Option<String>,
    pub pair_family_gap_cohort_surface_family_driver_cohort_stress_driver_count: Option<usize>,
    pub pair_family_gap_cohort_surface_family_driver_cohort_stress_retained_family_count:
        Option<usize>,
    pub pair_family_gap_cohort_surface_family_driver_cohort_stress_split_family_count:
        Option<usize>,
    pub pair_family_gap_cohort_surface_family_driver_cohort_stress_retained_window_count:
        Option<usize>,
    pub pair_family_gap_cohort_surface_family_driver_cohort_stress_opposite_window_count:
        Option<usize>,
    pub pair_family_gap_cohort_surface_family_driver_cohort_stress_target: Option<String>,
    pub pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_status:
        Option<String>,
    pub pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_decision:
        Option<String>,
    pub pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_control_count:
        Option<usize>,
    pub pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_retained_family_count:
        Option<usize>,
    pub pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_split_family_count:
        Option<usize>,
    pub pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_retained_window_count:
        Option<usize>,
    pub pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_opposite_window_count:
        Option<usize>,
    pub pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_target:
        Option<String>,
    pub pair_family_gap_cohort_surface_agnostic_ensemble_picker_decision: Option<String>,
    pub pair_family_gap_cohort_surface_agnostic_ensemble_picker_candidate_count: Option<usize>,
    pub pair_family_gap_cohort_surface_agnostic_ensemble_picker_stable_connector_count:
        Option<usize>,
    pub pair_family_gap_cohort_surface_agnostic_ensemble_picker_selected_direction: Option<String>,
    pub pair_family_gap_cohort_surface_agnostic_ensemble_picker_selected_connector_count:
        Option<usize>,
    pub pair_family_gap_cohort_surface_agnostic_ensemble_picker_selected_supported_surface_count:
        Option<usize>,
    pub pair_family_gap_cohort_surface_agnostic_ensemble_picker_retained_window_count:
        Option<usize>,
    pub pair_family_gap_cohort_surface_agnostic_ensemble_picker_opposite_window_count:
        Option<usize>,
    pub pair_family_gap_cohort_surface_agnostic_ensemble_picker_target: Option<String>,
    pub proof_status: String,
    pub claim_status: String,
    pub cells: Vec<SignalCatalogConnectorDigit8ClassifierCell>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalCatalogConnectorDigit8ClassifierCell {
    pub edge: String,
    pub width: u32,
    pub connector: String,
    pub moduli: Vec<u32>,
    pub reverse_only_pair_count: usize,
    pub comparison_pair_count: usize,
    pub lean_module: String,
    pub lean_summary_theorem: String,
    pub outside_ladder_reverse_only_pair_count: usize,
    pub outside_ladder_retained_modulus_count: usize,
    pub outside_ladder_split_modulus_count: usize,
    pub outside_ladder_collapsed_modulus_count: usize,
    pub outside_ladder_cell_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalCatalogVerification {
    pub ok: bool,
    pub checked_rows: usize,
    pub failures: Vec<SignalCatalogVerificationFailure>,
    pub gate_results: Vec<SignalCatalogGateResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalCatalogVerificationFailure {
    pub signal_id: String,
    pub field: String,
    pub value: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalCatalogGateResult {
    pub signal_id: String,
    pub drift_check_command: String,
    pub status: String,
    pub ok: bool,
    pub timed_out: bool,
    pub exit_code: Option<i32>,
    pub duration_ms: u64,
    pub error_message: Option<String>,
}

pub fn build_signal_catalog() -> SignalCatalog {
    let connector_digit8_classifier_family = build_connector_digit8_classifier_family_summary();
    let connector_width6_next_target = vec![
        connector_digit8_classifier_family
            .pair_family_gap_cohort_surface_agnostic_ensemble_picker_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_surface_family_driver_cohort_stress_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_surface_family_contrast_anatomy_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_surface_family_contrast_stress_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_surface_family_contrast_picker_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_volatility_ensemble_stress_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_volatility_ensemble_picker_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_sign_persistence_stress_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_sign_persistence_picker_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_window_consensus_stress_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_window_consensus_surface_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_ratio_geometry_forward_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_ratio_geometry_three_null_conclusion_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_ratio_geometry_post_two_null_picker_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_ratio_geometry_next_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_ratio_geometry_next_picker_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_ratio_geometry_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_ratio_geometry_picker_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_ratio_geometry_atlas_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_ratio_correction_bound_stability_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_ratio_geometry_expansion_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_ratio_geometry_replication_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_ratio_geometry_control_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_cohort_geometry_control_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_topn_motif_surface_profile_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_surface_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_pair_family_surface_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_cohort_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_cohort_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_pair_family_cohort_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_size_band_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_size_band_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_pair_family_size_band_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_size_band_control_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_extension_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_extension_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_pair_family_gap_extension_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_extension_control_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_portfolio_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_portfolio_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_pair_family_gap_portfolio_target
            .clone(),
        connector_digit8_classifier_family
            .pair_family_gap_portfolio_control_target
            .clone(),
        connector_digit8_classifier_family
            .connector_stress_meta_atlas_selected_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_base_mixed_connector_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_base_mixed_connector_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_orthogonal_base_mixed_connector_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_base_mixed_connector_control_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_carry_chain_connector_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_carry_chain_connector_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_orthogonal_carry_chain_connector_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_arithmetic_family_registry_selected_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_cyclic_reptend_connector_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_cyclic_reptend_connector_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_orthogonal_cyclic_reptend_connector_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_automorphic_repunit_connector_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_automorphic_repunit_connector_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_orthogonal_automorphic_repunit_connector_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_multiplicative_order_connector_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_multiplicative_order_connector_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_orthogonal_multiplicative_order_connector_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_crt_paired_connector_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_crt_paired_connector_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_orthogonal_crt_paired_connector_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_modular_walk_connector_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_modular_walk_connector_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_orthogonal_modular_walk_connector_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_residue_lattice_connector_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_residue_lattice_connector_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_orthogonal_residue_lattice_connector_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_arithmetic_connector_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_arithmetic_connector_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_orthogonal_arithmetic_connector_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_repeated_block_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_repeated_block_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_orthogonal_repeated_block_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_edge_plus_interior_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_edge_plus_interior_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_orthogonal_edge_plus_interior_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_nonadjacent_two_digit_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_nonadjacent_two_digit_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_orthogonal_nonadjacent_two_digit_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_compact_three_digit_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_compact_three_digit_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_orthogonal_compact_three_digit_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_pair_family_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .orthogonal_pair_family_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .selected_orthogonal_pair_family_target
            .clone(),
        connector_digit8_classifier_family
            .multi_digit_motif_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .multi_digit_motif_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .multi_digit_motif_family_independent_replication_target
            .clone(),
        connector_digit8_classifier_family
            .selected_multi_digit_motif_target
            .clone(),
        connector_digit8_classifier_family
            .interior_non_mod3_after_sixth_retirement_family_independent_replication_target
            .clone(),
        connector_digit8_classifier_family
            .selected_after_sixth_retirement_interior_non_mod3_candidate_target
            .clone(),
        connector_digit8_classifier_family
            .interior_non_mod3_after_fifth_retirement_family_independent_replication_target
            .clone(),
        connector_digit8_classifier_family
            .selected_after_fifth_retirement_interior_non_mod3_candidate_target
            .clone(),
        connector_digit8_classifier_family
            .interior_non_mod3_after_fourth_retirement_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .interior_non_mod3_after_fourth_retirement_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .interior_non_mod3_after_fourth_retirement_family_independent_replication_target
            .clone(),
        connector_digit8_classifier_family
            .selected_after_fourth_retirement_interior_non_mod3_candidate_target
            .clone(),
        connector_digit8_classifier_family
            .interior_non_mod3_after_third_retirement_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .interior_non_mod3_after_third_retirement_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .interior_non_mod3_after_third_retirement_family_independent_replication_target
            .clone(),
        connector_digit8_classifier_family
            .selected_after_third_retirement_interior_non_mod3_candidate_target
            .clone(),
        connector_digit8_classifier_family
            .interior_non_mod3_post_retirement_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .interior_non_mod3_post_retirement_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .interior_non_mod3_post_retirement_family_independent_replication_target
            .clone(),
        connector_digit8_classifier_family
            .selected_post_retirement_interior_non_mod3_candidate_target
            .clone(),
        connector_digit8_classifier_family
            .interior_non_mod3_next_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .interior_non_mod3_next_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .interior_non_mod3_next_family_independent_replication_target
            .clone(),
        connector_digit8_classifier_family
            .selected_next_interior_non_mod3_candidate_target
            .clone(),
        connector_digit8_classifier_family
            .interior_non_mod3_residue_separator_replication_target
            .clone(),
        connector_digit8_classifier_family
            .interior_non_mod3_residue_profile_target
            .clone(),
        connector_digit8_classifier_family
            .interior_non_mod3_family_independent_replication_target
            .clone(),
        connector_digit8_classifier_family
            .selected_interior_non_mod3_candidate_target
            .clone(),
        connector_digit8_classifier_family
            .next_non_mod3_independent_replication_target
            .clone(),
        connector_digit8_classifier_family
            .selected_next_non_mod3_candidate_target
            .clone(),
        connector_digit8_classifier_family
            .non_mod3_mutated_residue_separator_replication_next_target
            .clone(),
        connector_digit8_classifier_family
            .non_mod3_residue_separator_replication_next_target
            .clone(),
        connector_digit8_classifier_family
            .non_mod3_residue_profile_next_target
            .clone(),
        connector_digit8_classifier_family
            .non_mod3_second_replication_next_target
            .clone(),
        connector_digit8_classifier_family
            .selected_non_mod3_candidate_target
            .clone(),
        connector_digit8_classifier_family
            .selected_branch_independent_replication_next_target
            .clone(),
        connector_digit8_classifier_family
            .selected_next_branch_target
            .clone(),
    ]
    .into_iter()
    .flatten()
    .next()
    .unwrap_or_else(|| "no non-collapsed connector stress branch selected".to_string());
    let rows = vec![
        SignalCatalogRow {
            signal_id: "matched-control-smoke-atlas".to_string(),
            domain: "matched-control".to_string(),
            artifact_path: "docs/atlas/matched_control_smoke_atlas_manifest.json".to_string(),
            drift_check_command: "scripts/matched_control_atlas_bridge.sh verify".to_string(),
            claim_status: "matched-control stability/proof index; no residual mechanism claimed"
                .to_string(),
            proof_status: "generated Lean lane and smoke-profile certificate links".to_string(),
            empirical_status: "canonical smoke-panel maintained-family surface".to_string(),
            next_theorem_target: Some(
                "use residue-mask theorem queue for non-duplicative finite-mask facts".to_string(),
            ),
        },
        SignalCatalogRow {
            signal_id: "witness-search-policy-atlas".to_string(),
            domain: "witness".to_string(),
            artifact_path: "docs/witness/witness_search_policy_atlas.json".to_string(),
            drift_check_command: "scripts/proof_carrying_witness.sh verify".to_string(),
            claim_status: "construction/replay certificate catalog; probable-prime status is not a primality proof"
                .to_string(),
            proof_status: "generated Lean witness replay catalog links for promoted rows".to_string(),
            empirical_status: "canonical deterministic witness bundle and search-policy replay surface"
                .to_string(),
            next_theorem_target: Some(
                "extend replay accounting only where policy-matrix atlas selects a new row".to_string(),
            ),
        },
        SignalCatalogRow {
            signal_id: "witness-lean-catalog".to_string(),
            domain: "witness".to_string(),
            artifact_path: "docs/witness/witness_lean_catalog_manifest.json".to_string(),
            drift_check_command: "scripts/lean_proof_carrying_witness_certificate.sh verify"
                .to_string(),
            claim_status: "Lean declaration-link manifest for canonical witness certificates"
                .to_string(),
            proof_status: "machine-checked declaration resolution through generated check modules"
                .to_string(),
            empirical_status: "not an empirical report; proof-catalog bridge metadata".to_string(),
            next_theorem_target: None,
        },
        SignalCatalogRow {
            signal_id: "witness-policy-matrix-lean-catalog".to_string(),
            domain: "witness".to_string(),
            artifact_path: "docs/witness/witness_policy_matrix_lean_catalog_manifest.json"
                .to_string(),
            drift_check_command: "scripts/lean_proof_carrying_witness_certificate.sh verify"
                .to_string(),
            claim_status: "promoted policy-matrix replay certificates without primality-proof promotion"
                .to_string(),
            proof_status: "machine-checked generated Lean replay theorem links".to_string(),
            empirical_status: "policy-matrix promoted-row proof coverage".to_string(),
            next_theorem_target: Some(
                "use policy-matrix atlas to choose future large-row replay promotions".to_string(),
            ),
        },
        SignalCatalogRow {
            signal_id: "connector-signal-atlas".to_string(),
            domain: "connector".to_string(),
            artifact_path: "docs/connector/connector_signal_atlas.json".to_string(),
            drift_check_command: "scripts/connector_signal_atlas.sh verify".to_string(),
            claim_status:
                "exact residue guardrails plus empirical residual reporting; no connector law claimed"
                    .to_string(),
            proof_status: "Lean connector residue filters, finite survivor-count null theorem, and Hardy-Littlewood coverage transform guardrail".to_string(),
            empirical_status: "maintained same-budget connector-pair residual sweep".to_string(),
            next_theorem_target: Some(
                "use the theorem-backed residue-null layer to isolate any future connector residual target"
                    .to_string(),
            ),
        },
        SignalCatalogRow {
            signal_id: "connector-width6-stress".to_string(),
            domain: "connector".to_string(),
            artifact_path: "docs/connector/connector_width6_stress.json".to_string(),
            drift_check_command: "scripts/connector_signal_atlas.sh verify".to_string(),
            claim_status:
                "bounded connector stress microscope; no connector law or density mechanism claimed"
                    .to_string(),
            proof_status:
                "three theorem-backed digit-8 multi-modulus finite classifier cells; no visible unclassified exact separators"
                    .to_string(),
            empirical_status:
                "width-6 twin-prime ladder stress surface with branch-status routing after digit-8 collapse"
                    .to_string(),
            next_theorem_target: Some(connector_width6_next_target),
        },
        SignalCatalogRow {
            signal_id: "connector-replication-null-atlas".to_string(),
            domain: "connector".to_string(),
            artifact_path: "docs/connector/connector_replication_null_atlas.json".to_string(),
            drift_check_command: "scripts/connector_signal_atlas.sh verify".to_string(),
            claim_status:
                "connector stress replication accounting; no connector law or density mechanism claimed"
                    .to_string(),
            proof_status: format!(
                "single-branch separator stability status {}; theorem candidates {}",
                connector_digit8_classifier_family.replication_null_atlas_status,
                connector_digit8_classifier_family.replication_null_atlas_theorem_candidate_count
            ),
            empirical_status: format!(
                "retained/split/collapsed separator outcomes {}/{}/{} across normalized branch rows",
                connector_digit8_classifier_family.replication_null_atlas_retained_separator_count,
                connector_digit8_classifier_family.replication_null_atlas_split_separator_count,
                connector_digit8_classifier_family.replication_null_atlas_collapsed_separator_count
            ),
            next_theorem_target: Some(
                connector_digit8_classifier_family
                    .replication_null_atlas_next_target
                    .clone(),
            ),
        },
    ];

    SignalCatalog {
        schema_version: SIGNAL_CATALOG_SCHEMA_VERSION.to_string(),
        artifact_id: SIGNAL_CATALOG_ARTIFACT_ID.to_string(),
        generator_command: SIGNAL_CATALOG_GENERATOR_COMMAND.to_string(),
        drift_check_command: SIGNAL_CATALOG_DRIFT_CHECK_COMMAND.to_string(),
        summary: signal_catalog_summary(&rows),
        connector_digit8_classifier_family,
        rows,
    }
}

fn build_connector_digit8_classifier_family_summary() -> SignalCatalogConnectorDigit8ClassifierFamily
{
    let report = build_connector_width6_stress_report();
    let screen = report
        .ladder_peak_matched_control_screen
        .as_ref()
        .expect("maintained connector stress report includes matched-control screen");
    let probe = screen
        .digit8_edge_zoom_probe
        .as_ref()
        .expect("maintained connector stress report includes digit-8 edge zoom probe");
    let branch_picker = screen
        .branch_status_picker
        .as_ref()
        .expect("maintained connector stress report includes branch status picker");
    let selected_branch = branch_picker.selected_branch.as_ref();
    let independent_replication = screen.selected_branch_independent_replication.as_ref();
    let non_mod3_picker = screen.non_mod3_candidate_picker.as_ref();
    let selected_non_mod3_candidate =
        non_mod3_picker.and_then(|picker| picker.selected_candidate.as_ref());
    let non_mod3_second_replication = screen.non_mod3_candidate_second_replication.as_ref();
    let non_mod3_residue_profile = screen.non_mod3_candidate_residue_profile.as_ref();
    let non_mod3_residue_separator_replication =
        screen.non_mod3_residue_separator_replication.as_ref();
    let non_mod3_mutated_residue_separator_replication = screen
        .non_mod3_mutated_residue_separator_replication
        .as_ref();
    let next_non_mod3_picker = screen.non_mod3_next_candidate_picker.as_ref();
    let selected_next_non_mod3_candidate =
        next_non_mod3_picker.and_then(|picker| picker.selected_candidate.as_ref());
    let next_non_mod3_replication = screen
        .non_mod3_next_candidate_independent_replication
        .as_ref();
    let interior_non_mod3_picker = screen.interior_non_mod3_family_picker.as_ref();
    let selected_interior_non_mod3_candidate =
        interior_non_mod3_picker.and_then(|picker| picker.selected_candidate.as_ref());
    let interior_non_mod3_replication = screen
        .interior_non_mod3_family_independent_replication
        .as_ref();
    let interior_non_mod3_residue_profile = screen.interior_non_mod3_residue_profile.as_ref();
    let interior_non_mod3_residue_separator_replication = screen
        .interior_non_mod3_residue_separator_replication
        .as_ref();
    let interior_non_mod3_next_family_picker = screen.interior_non_mod3_next_family_picker.as_ref();
    let selected_next_interior_non_mod3_candidate =
        interior_non_mod3_next_family_picker.and_then(|picker| picker.selected_candidate.as_ref());
    let interior_non_mod3_next_family_independent_replication = screen
        .interior_non_mod3_next_family_independent_replication
        .as_ref();
    let interior_non_mod3_next_residue_profile =
        screen.interior_non_mod3_next_residue_profile.as_ref();
    let interior_non_mod3_next_residue_separator_replication = screen
        .interior_non_mod3_next_residue_separator_replication
        .as_ref();
    let interior_non_mod3_post_retirement_family_picker = screen
        .interior_non_mod3_post_retirement_family_picker
        .as_ref();
    let selected_post_retirement_interior_non_mod3_candidate =
        interior_non_mod3_post_retirement_family_picker
            .and_then(|picker| picker.selected_candidate.as_ref());
    let interior_non_mod3_post_retirement_family_independent_replication = screen
        .interior_non_mod3_post_retirement_family_independent_replication
        .as_ref();
    let interior_non_mod3_post_retirement_residue_profile = screen
        .interior_non_mod3_post_retirement_residue_profile
        .as_ref();
    let interior_non_mod3_post_retirement_residue_separator_replication = screen
        .interior_non_mod3_post_retirement_residue_separator_replication
        .as_ref();
    let interior_non_mod3_after_third_retirement_family_picker = screen
        .interior_non_mod3_after_third_retirement_family_picker
        .as_ref();
    let selected_after_third_retirement_interior_non_mod3_candidate =
        interior_non_mod3_after_third_retirement_family_picker
            .and_then(|picker| picker.selected_candidate.as_ref());
    let interior_non_mod3_after_third_retirement_family_independent_replication = screen
        .interior_non_mod3_after_third_retirement_family_independent_replication
        .as_ref();
    let interior_non_mod3_after_third_retirement_residue_profile = screen
        .interior_non_mod3_after_third_retirement_residue_profile
        .as_ref();
    let interior_non_mod3_after_third_retirement_residue_separator_replication = screen
        .interior_non_mod3_after_third_retirement_residue_separator_replication
        .as_ref();
    let interior_non_mod3_after_fourth_retirement_family_picker = screen
        .interior_non_mod3_after_fourth_retirement_family_picker
        .as_ref();
    let selected_after_fourth_retirement_interior_non_mod3_candidate =
        interior_non_mod3_after_fourth_retirement_family_picker
            .and_then(|picker| picker.selected_candidate.as_ref());
    let interior_non_mod3_after_fourth_retirement_family_independent_replication = screen
        .interior_non_mod3_after_fourth_retirement_family_independent_replication
        .as_ref();
    let interior_non_mod3_after_fourth_retirement_residue_profile = screen
        .interior_non_mod3_after_fourth_retirement_residue_profile
        .as_ref();
    let interior_non_mod3_after_fourth_retirement_residue_separator_replication = screen
        .interior_non_mod3_after_fourth_retirement_residue_separator_replication
        .as_ref();
    let interior_non_mod3_after_fifth_retirement_family_picker = screen
        .interior_non_mod3_after_fifth_retirement_family_picker
        .as_ref();
    let selected_after_fifth_retirement_interior_non_mod3_candidate =
        interior_non_mod3_after_fifth_retirement_family_picker
            .and_then(|picker| picker.selected_candidate.as_ref());
    let interior_non_mod3_after_fifth_retirement_family_independent_replication = screen
        .interior_non_mod3_after_fifth_retirement_family_independent_replication
        .as_ref();
    let interior_non_mod3_after_sixth_retirement_family_picker = screen
        .interior_non_mod3_after_sixth_retirement_family_picker
        .as_ref();
    let selected_after_sixth_retirement_interior_non_mod3_candidate =
        interior_non_mod3_after_sixth_retirement_family_picker
            .and_then(|picker| picker.selected_candidate.as_ref());
    let interior_non_mod3_after_sixth_retirement_family_independent_replication = screen
        .interior_non_mod3_after_sixth_retirement_family_independent_replication
        .as_ref();
    let multi_digit_motif_family_picker = screen.multi_digit_motif_family_picker.as_ref();
    let selected_multi_digit_motif =
        multi_digit_motif_family_picker.and_then(|picker| picker.selected_motif.as_ref());
    let multi_digit_motif_family_independent_replication = screen
        .multi_digit_motif_family_independent_replication
        .as_ref();
    let multi_digit_motif_residue_profile = screen.multi_digit_motif_residue_profile.as_ref();
    let multi_digit_motif_residue_separator_replication = screen
        .multi_digit_motif_residue_separator_replication
        .as_ref();
    let orthogonal_pair_family_control_matrix =
        screen.orthogonal_pair_family_control_matrix.as_ref();
    let selected_orthogonal_pair_family_branch =
        orthogonal_pair_family_control_matrix.and_then(|matrix| matrix.selected_branch.as_ref());
    let orthogonal_pair_family_residue_profile =
        screen.orthogonal_pair_family_residue_profile.as_ref();
    let orthogonal_pair_family_residue_separator_replication = screen
        .orthogonal_pair_family_residue_separator_replication
        .as_ref();
    let orthogonal_compact_three_digit_control =
        screen.orthogonal_compact_three_digit_control.as_ref();
    let selected_orthogonal_compact_three_digit_branch =
        orthogonal_compact_three_digit_control.and_then(|matrix| matrix.selected_branch.as_ref());
    let orthogonal_compact_three_digit_residue_profile = screen
        .orthogonal_compact_three_digit_residue_profile
        .as_ref();
    let orthogonal_compact_three_digit_residue_separator_replication = screen
        .orthogonal_compact_three_digit_residue_separator_replication
        .as_ref();
    let orthogonal_nonadjacent_two_digit_control =
        screen.orthogonal_nonadjacent_two_digit_control.as_ref();
    let selected_orthogonal_nonadjacent_two_digit_branch =
        orthogonal_nonadjacent_two_digit_control.and_then(|matrix| matrix.selected_branch.as_ref());
    let orthogonal_nonadjacent_two_digit_residue_profile = screen
        .orthogonal_nonadjacent_two_digit_residue_profile
        .as_ref();
    let orthogonal_nonadjacent_two_digit_residue_separator_replication = screen
        .orthogonal_nonadjacent_two_digit_residue_separator_replication
        .as_ref();
    let orthogonal_edge_plus_interior_control =
        screen.orthogonal_edge_plus_interior_control.as_ref();
    let selected_orthogonal_edge_plus_interior_branch =
        orthogonal_edge_plus_interior_control.and_then(|matrix| matrix.selected_branch.as_ref());
    let orthogonal_edge_plus_interior_residue_profile = screen
        .orthogonal_edge_plus_interior_residue_profile
        .as_ref();
    let orthogonal_edge_plus_interior_residue_separator_replication = screen
        .orthogonal_edge_plus_interior_residue_separator_replication
        .as_ref();
    let orthogonal_repeated_block_control = screen.orthogonal_repeated_block_control.as_ref();
    let selected_orthogonal_repeated_block_branch =
        orthogonal_repeated_block_control.and_then(|matrix| matrix.selected_branch.as_ref());
    let orthogonal_repeated_block_residue_profile =
        screen.orthogonal_repeated_block_residue_profile.as_ref();
    let orthogonal_repeated_block_residue_separator_replication = screen
        .orthogonal_repeated_block_residue_separator_replication
        .as_ref();
    let orthogonal_arithmetic_connector_control =
        screen.orthogonal_arithmetic_connector_control.as_ref();
    let selected_orthogonal_arithmetic_connector_branch =
        orthogonal_arithmetic_connector_control.and_then(|matrix| matrix.selected_branch.as_ref());
    let orthogonal_arithmetic_connector_residue_profile = screen
        .orthogonal_arithmetic_connector_residue_profile
        .as_ref();
    let orthogonal_arithmetic_connector_residue_separator_replication = screen
        .orthogonal_arithmetic_connector_residue_separator_replication
        .as_ref();
    let orthogonal_residue_lattice_connector_control =
        screen.orthogonal_residue_lattice_connector_control.as_ref();
    let selected_orthogonal_residue_lattice_connector_branch =
        orthogonal_residue_lattice_connector_control
            .and_then(|matrix| matrix.selected_branch.as_ref());
    let orthogonal_residue_lattice_connector_residue_profile = screen
        .orthogonal_residue_lattice_connector_residue_profile
        .as_ref();
    let orthogonal_residue_lattice_connector_residue_separator_replication = screen
        .orthogonal_residue_lattice_connector_residue_separator_replication
        .as_ref();
    let orthogonal_modular_walk_connector_control =
        screen.orthogonal_modular_walk_connector_control.as_ref();
    let selected_orthogonal_modular_walk_connector_branch =
        orthogonal_modular_walk_connector_control
            .and_then(|matrix| matrix.selected_branch.as_ref());
    let orthogonal_modular_walk_connector_residue_profile = screen
        .orthogonal_modular_walk_connector_residue_profile
        .as_ref();
    let orthogonal_modular_walk_connector_residue_separator_replication = screen
        .orthogonal_modular_walk_connector_residue_separator_replication
        .as_ref();
    let orthogonal_arithmetic_family_registry =
        screen.orthogonal_arithmetic_family_registry.as_ref();
    let orthogonal_crt_paired_connector_control =
        screen.orthogonal_crt_paired_connector_control.as_ref();
    let selected_orthogonal_crt_paired_connector_branch =
        orthogonal_crt_paired_connector_control.and_then(|matrix| matrix.selected_branch.as_ref());
    let orthogonal_crt_paired_connector_residue_profile = screen
        .orthogonal_crt_paired_connector_residue_profile
        .as_ref();
    let orthogonal_crt_paired_connector_residue_separator_replication = screen
        .orthogonal_crt_paired_connector_residue_separator_replication
        .as_ref();
    let orthogonal_multiplicative_order_connector_control = screen
        .orthogonal_multiplicative_order_connector_control
        .as_ref();
    let selected_orthogonal_multiplicative_order_connector_branch =
        orthogonal_multiplicative_order_connector_control
            .and_then(|matrix| matrix.selected_branch.as_ref());
    let orthogonal_multiplicative_order_connector_residue_profile = screen
        .orthogonal_multiplicative_order_connector_residue_profile
        .as_ref();
    let orthogonal_multiplicative_order_connector_residue_separator_replication = screen
        .orthogonal_multiplicative_order_connector_residue_separator_replication
        .as_ref();
    let orthogonal_automorphic_repunit_connector_control = screen
        .orthogonal_automorphic_repunit_connector_control
        .as_ref();
    let selected_orthogonal_automorphic_repunit_connector_branch =
        orthogonal_automorphic_repunit_connector_control
            .and_then(|matrix| matrix.selected_branch.as_ref());
    let orthogonal_automorphic_repunit_connector_residue_profile = screen
        .orthogonal_automorphic_repunit_connector_residue_profile
        .as_ref();
    let orthogonal_automorphic_repunit_connector_residue_separator_replication = screen
        .orthogonal_automorphic_repunit_connector_residue_separator_replication
        .as_ref();
    let orthogonal_cyclic_reptend_connector_control =
        screen.orthogonal_cyclic_reptend_connector_control.as_ref();
    let selected_orthogonal_cyclic_reptend_connector_branch =
        orthogonal_cyclic_reptend_connector_control
            .and_then(|matrix| matrix.selected_branch.as_ref());
    let orthogonal_cyclic_reptend_connector_residue_profile = screen
        .orthogonal_cyclic_reptend_connector_residue_profile
        .as_ref();
    let orthogonal_cyclic_reptend_connector_residue_separator_replication = screen
        .orthogonal_cyclic_reptend_connector_residue_separator_replication
        .as_ref();
    let orthogonal_carry_chain_connector_control =
        screen.orthogonal_carry_chain_connector_control.as_ref();
    let selected_orthogonal_carry_chain_connector_branch =
        orthogonal_carry_chain_connector_control.and_then(|matrix| matrix.selected_branch.as_ref());
    let orthogonal_carry_chain_connector_residue_profile = screen
        .orthogonal_carry_chain_connector_residue_profile
        .as_ref();
    let orthogonal_carry_chain_connector_residue_separator_replication = screen
        .orthogonal_carry_chain_connector_residue_separator_replication
        .as_ref();
    let orthogonal_base_mixed_connector_control =
        screen.orthogonal_base_mixed_connector_control.as_ref();
    let selected_orthogonal_base_mixed_connector_branch =
        orthogonal_base_mixed_connector_control.and_then(|matrix| matrix.selected_branch.as_ref());
    let orthogonal_base_mixed_connector_residue_profile = screen
        .orthogonal_base_mixed_connector_residue_profile
        .as_ref();
    let orthogonal_base_mixed_connector_residue_separator_replication = screen
        .orthogonal_base_mixed_connector_residue_separator_replication
        .as_ref();
    let connector_stress_meta_atlas = screen.connector_stress_meta_atlas.as_ref();
    let pair_family_gap_portfolio_control = screen.pair_family_gap_portfolio_control.as_ref();
    let selected_pair_family_gap_portfolio_branch =
        pair_family_gap_portfolio_control.and_then(|matrix| matrix.selected_branch.as_ref());
    let pair_family_gap_portfolio_residue_profile =
        screen.pair_family_gap_portfolio_residue_profile.as_ref();
    let pair_family_gap_portfolio_residue_separator_replication = screen
        .pair_family_gap_portfolio_residue_separator_replication
        .as_ref();
    let pair_family_gap_extension_control = screen.pair_family_gap_extension_control.as_ref();
    let selected_pair_family_gap_extension_branch =
        pair_family_gap_extension_control.and_then(|matrix| matrix.selected_branch.as_ref());
    let pair_family_gap_extension_residue_profile =
        screen.pair_family_gap_extension_residue_profile.as_ref();
    let pair_family_gap_extension_residue_separator_replication = screen
        .pair_family_gap_extension_residue_separator_replication
        .as_ref();
    let pair_family_size_band_control = screen.pair_family_size_band_control.as_ref();
    let selected_pair_family_size_band_branch =
        pair_family_size_band_control.and_then(|matrix| matrix.selected_branch.as_ref());
    let pair_family_size_band_residue_profile =
        screen.pair_family_size_band_residue_profile.as_ref();
    let pair_family_size_band_residue_separator_replication = screen
        .pair_family_size_band_residue_separator_replication
        .as_ref();
    let pair_family_cohort_retention_picker = screen.pair_family_cohort_retention_picker.as_ref();
    let selected_pair_family_cohort =
        pair_family_cohort_retention_picker.and_then(|picker| picker.selected_cohort.as_ref());
    let pair_family_cohort_residue_profile = screen.pair_family_cohort_residue_profile.as_ref();
    let pair_family_cohort_residue_separator_replication = screen
        .pair_family_cohort_residue_separator_replication
        .as_ref();
    let pair_family_surface_picker = screen.pair_family_surface_picker.as_ref();
    let selected_pair_family_surface =
        pair_family_surface_picker.and_then(|picker| picker.selected_surface.as_ref());
    let pair_family_surface_residue_profile = screen.pair_family_surface_residue_profile.as_ref();
    let pair_family_topn_motif_surface_profile =
        screen.pair_family_topn_motif_surface_profile.as_ref();
    let pair_family_gap_cohort_geometry_control =
        screen.pair_family_gap_cohort_geometry_control.as_ref();
    let pair_family_gap_cohort_residue_profile =
        screen.pair_family_gap_cohort_residue_profile.as_ref();
    let pair_family_gap_cohort_residue_separator_replication = screen
        .pair_family_gap_cohort_residue_separator_replication
        .as_ref();
    let pair_family_gap_cohort_ratio_geometry_control = screen
        .pair_family_gap_cohort_ratio_geometry_control
        .as_ref();
    let pair_family_gap_cohort_ratio_geometry_replication = screen
        .pair_family_gap_cohort_ratio_geometry_replication
        .as_ref();
    let pair_family_gap_cohort_ratio_geometry_expansion = screen
        .pair_family_gap_cohort_ratio_geometry_expansion
        .as_ref();
    let pair_family_gap_cohort_ratio_correction_bound_stability = screen
        .pair_family_gap_cohort_ratio_correction_bound_stability
        .as_ref();
    let pair_family_gap_cohort_ratio_geometry_atlas =
        screen.pair_family_gap_cohort_ratio_geometry_atlas.as_ref();
    let pair_family_gap_cohort_ratio_geometry_picker =
        screen.pair_family_gap_cohort_ratio_geometry_picker.as_ref();
    let pair_family_gap_cohort_ratio_geometry_residue_profile = screen
        .pair_family_gap_cohort_ratio_geometry_residue_profile
        .as_ref();
    let pair_family_gap_cohort_ratio_geometry_next_picker = screen
        .pair_family_gap_cohort_ratio_geometry_next_picker
        .as_ref();
    let pair_family_gap_cohort_ratio_geometry_next_residue_profile = screen
        .pair_family_gap_cohort_ratio_geometry_next_residue_profile
        .as_ref();
    let pair_family_gap_cohort_ratio_geometry_post_two_null_picker = screen
        .pair_family_gap_cohort_ratio_geometry_post_two_null_picker
        .as_ref();
    let pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile = screen
        .pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile
        .as_ref();
    let pair_family_gap_cohort_ratio_geometry_three_null_conclusion = screen
        .pair_family_gap_cohort_ratio_geometry_three_null_conclusion
        .as_ref();
    let pair_family_gap_cohort_ratio_geometry_forward_residue_profile = screen
        .pair_family_gap_cohort_ratio_geometry_forward_residue_profile
        .as_ref();
    let pair_family_gap_cohort_ratio_geometry_forward_null_conclusion = screen
        .pair_family_gap_cohort_ratio_geometry_forward_null_conclusion
        .as_ref();
    let pair_family_gap_cohort_window_consensus_surface = screen
        .pair_family_gap_cohort_window_consensus_surface
        .as_ref();
    let pair_family_gap_cohort_window_consensus_stress = screen
        .pair_family_gap_cohort_window_consensus_stress
        .as_ref();
    let pair_family_gap_cohort_sign_persistence_picker = screen
        .pair_family_gap_cohort_sign_persistence_picker
        .as_ref();
    let pair_family_gap_cohort_sign_persistence_stress = screen
        .pair_family_gap_cohort_sign_persistence_stress
        .as_ref();
    let pair_family_gap_cohort_volatility_ensemble_picker = screen
        .pair_family_gap_cohort_volatility_ensemble_picker
        .as_ref();
    let pair_family_gap_cohort_volatility_ensemble_stress = screen
        .pair_family_gap_cohort_volatility_ensemble_stress
        .as_ref();
    let pair_family_gap_cohort_surface_family_contrast_picker = screen
        .pair_family_gap_cohort_surface_family_contrast_picker
        .as_ref();
    let pair_family_gap_cohort_surface_family_contrast_stress = screen
        .pair_family_gap_cohort_surface_family_contrast_stress
        .as_ref();
    let pair_family_gap_cohort_surface_family_contrast_anatomy = screen
        .pair_family_gap_cohort_surface_family_contrast_anatomy
        .as_ref();
    let pair_family_gap_cohort_surface_family_driver_cohort_stress = screen
        .pair_family_gap_cohort_surface_family_driver_cohort_stress
        .as_ref();
    let pair_family_gap_cohort_surface_family_matched_nondriver_control_stress = screen
        .pair_family_gap_cohort_surface_family_matched_nondriver_control_stress
        .as_ref();
    let pair_family_gap_cohort_surface_agnostic_ensemble_picker = screen
        .pair_family_gap_cohort_surface_agnostic_ensemble_picker
        .as_ref();
    let replication_null_atlas = &report.replication_null_atlas;
    let profile = probe
        .residue_profile
        .as_ref()
        .expect("maintained connector stress report includes digit-8 residue profile");
    let replication = probe
        .classifier_family_replication
        .as_ref()
        .expect("maintained connector stress report includes digit-8 classifier replication");
    let split_follow_up = replication
        .split_follow_up
        .as_ref()
        .expect("maintained connector stress report includes digit-8 split follow-up");

    let cells = profile
        .multi_modulus_summaries
        .iter()
        .filter_map(|summary| {
            let replication_cell = replication
                .cells
                .iter()
                .find(|cell| {
                    cell.edge == summary.edge
                        && cell.width == summary.width
                        && cell.connector == summary.connector
                })
                .expect("replication cell exists for each theorem-backed digit-8 summary cell");
            Some(SignalCatalogConnectorDigit8ClassifierCell {
                edge: summary.edge.clone(),
                width: summary.width,
                connector: summary.connector.clone(),
                moduli: summary.moduli.clone(),
                reverse_only_pair_count: summary.reverse_only_pair_count,
                comparison_pair_count: summary.comparison_pair_count,
                lean_module: summary.lean_module.as_ref()?.clone(),
                lean_summary_theorem: summary.lean_summary_theorem.as_ref()?.clone(),
                outside_ladder_reverse_only_pair_count: replication_cell.reverse_only_pair_count,
                outside_ladder_retained_modulus_count: replication_cell.retained_modulus_count,
                outside_ladder_split_modulus_count: replication_cell.split_modulus_count,
                outside_ladder_collapsed_modulus_count: replication_cell.collapsed_modulus_count,
                outside_ladder_cell_status: replication_cell.cell_status.clone(),
            })
        })
        .collect::<Vec<_>>();
    let unclassified_exact_separator_count = profile
        .cell_profiles
        .iter()
        .flat_map(|cell| cell.separator_rows.iter())
        .filter(|row| {
            row.separator_status == "exact-residue-separator" && row.lean_theorem.is_none()
        })
        .count();
    let surface_status = if unclassified_exact_separator_count == 0 && cells.len() == 3 {
        "complete-visible-digit8-exact-separator-family"
    } else if unclassified_exact_separator_count == 0 {
        "complete-visible-digit8-exact-separators-with-partial-family-summary"
    } else {
        "digit8-exact-separator-family-has-unclassified-rows"
    };

    SignalCatalogConnectorDigit8ClassifierFamily {
        source_artifact_path: "docs/connector/connector_width6_stress.json".to_string(),
        source_schema_version: report.schema_version,
        surface_status: surface_status.to_string(),
        theorem_backed_multi_modulus_cell_count: cells.len(),
        unclassified_exact_separator_count,
        replication_selection_rule: replication.selection_rule.clone(),
        outside_ladder_replication_decision: replication.replication_decision.clone(),
        outside_ladder_baseline_pair_count: replication.baseline_pair_count,
        outside_ladder_widened_pair_count: replication.widened_pair_count,
        outside_ladder_added_pair_count: replication.added_pair_count,
        outside_ladder_tested_cell_count: replication.tested_cell_count,
        outside_ladder_retained_cell_count: replication.retained_cell_count,
        outside_ladder_split_cell_count: replication.split_cell_count,
        outside_ladder_collapsed_cell_count: replication.collapsed_cell_count,
        split_follow_up_decision: split_follow_up.follow_up_decision.clone(),
        split_follow_up_source_row_count: split_follow_up.source_split_row_count,
        split_follow_up_tested_row_count: split_follow_up.tested_split_row_count,
        split_follow_up_stabilized_row_count: split_follow_up.stabilized_row_count,
        split_follow_up_split_again_row_count: split_follow_up.split_again_row_count,
        split_follow_up_collapsed_row_count: split_follow_up.collapsed_row_count,
        branch_picker_decision: branch_picker.picker_decision.clone(),
        selected_next_branch_id: selected_branch.map(|branch| branch.branch_id.clone()),
        selected_next_branch_status: selected_branch.map(|branch| branch.branch_status.clone()),
        selected_next_branch_target: selected_branch
            .map(|branch| branch.next_experiment_target.clone()),
        selected_branch_independent_replication_decision: independent_replication
            .map(|replication| replication.replication_decision.clone()),
        selected_branch_independent_replication_next_target: independent_replication
            .map(|replication| replication.next_experiment_target.clone()),
        retired_branch_id: independent_replication
            .filter(|replication| {
                replication.replication_decision
                    == "retired-all-fresh-independent-rows-theorem-blocked-by-mod3-null-layer"
            })
            .map(|replication| replication.source_branch_id.clone()),
        non_mod3_candidate_picker_decision: non_mod3_picker
            .map(|picker| picker.picker_decision.clone()),
        retired_non_mod3_candidate_count: non_mod3_picker
            .map(|picker| picker.retired_candidate_count)
            .unwrap_or(0),
        retired_non_mod3_candidate_ids: non_mod3_picker
            .map(|picker| picker.retired_candidate_ids.clone())
            .unwrap_or_default(),
        selected_non_mod3_candidate_id: selected_non_mod3_candidate.map(|candidate| {
            format!(
                "{}-edge-width{}-digit{}-connector-{}",
                candidate.edge, candidate.width, candidate.digit, candidate.connector
            )
        }),
        selected_non_mod3_candidate_target: non_mod3_picker
            .map(|picker| picker.next_experiment_target.clone()),
        non_mod3_second_replication_decision: non_mod3_second_replication
            .map(|replication| replication.replication_decision.clone()),
        non_mod3_second_replication_next_target: non_mod3_second_replication
            .map(|replication| replication.next_experiment_target.clone()),
        non_mod3_residue_profile_decision: non_mod3_residue_profile
            .map(|profile| profile.profile_decision.clone()),
        non_mod3_residue_profile_best_modulus: non_mod3_residue_profile
            .and_then(|profile| profile.best_separator.as_ref())
            .map(|separator| separator.modulus),
        non_mod3_residue_profile_next_target: non_mod3_residue_profile
            .map(|profile| profile.next_experiment_target.clone()),
        non_mod3_residue_separator_replication_decision: non_mod3_residue_separator_replication
            .map(|replication| replication.replication_decision.clone()),
        non_mod3_residue_separator_replication_status: non_mod3_residue_separator_replication
            .map(|replication| replication.separator_status.clone()),
        non_mod3_residue_separator_replication_next_target:
            non_mod3_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        non_mod3_mutated_residue_separator_replication_decision:
            non_mod3_mutated_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        non_mod3_mutated_residue_separator_replication_status:
            non_mod3_mutated_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        non_mod3_mutated_residue_separator_replication_next_target:
            non_mod3_mutated_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        next_non_mod3_candidate_picker_decision: next_non_mod3_picker
            .map(|picker| picker.picker_decision.clone()),
        selected_next_non_mod3_candidate_id: selected_next_non_mod3_candidate.map(|candidate| {
            format!(
                "{}-edge-width{}-digit{}-connector-{}",
                candidate.edge, candidate.width, candidate.digit, candidate.connector
            )
        }),
        selected_next_non_mod3_candidate_target: next_non_mod3_picker
            .map(|picker| picker.next_experiment_target.clone()),
        next_non_mod3_independent_replication_decision: next_non_mod3_replication
            .map(|replication| replication.replication_decision.clone()),
        next_non_mod3_independent_replication_target: next_non_mod3_replication
            .map(|replication| replication.next_experiment_target.clone()),
        non_mod3_retired_edge_candidate_count: screen.non_mod3_retirement_summary.len(),
        non_mod3_retired_edge_candidate_ids: screen
            .non_mod3_retirement_summary
            .iter()
            .map(|row| row.candidate_id.clone())
            .collect(),
        interior_non_mod3_family_picker_decision: interior_non_mod3_picker
            .map(|picker| picker.picker_decision.clone()),
        selected_interior_non_mod3_candidate_id: selected_interior_non_mod3_candidate
            .map(|candidate| {
                format!(
                    "interior-width{}-position{}-digit{}-connector-{}",
                    candidate.width, candidate.position, candidate.digit, candidate.connector
                )
            }),
        selected_interior_non_mod3_candidate_target: interior_non_mod3_picker
            .map(|picker| picker.next_experiment_target.clone()),
        interior_non_mod3_family_independent_replication_decision:
            interior_non_mod3_replication
                .map(|replication| replication.replication_decision.clone()),
        interior_non_mod3_family_independent_replication_target: interior_non_mod3_replication
            .map(|replication| replication.next_experiment_target.clone()),
        interior_non_mod3_residue_profile_decision: interior_non_mod3_residue_profile
            .map(|profile| profile.profile_decision.clone()),
        interior_non_mod3_residue_profile_best_modulus: interior_non_mod3_residue_profile
            .and_then(|profile| profile.best_separator.as_ref())
            .map(|separator| separator.modulus),
        interior_non_mod3_residue_profile_target: interior_non_mod3_residue_profile
            .map(|profile| profile.next_experiment_target.clone()),
        interior_non_mod3_residue_separator_replication_decision:
            interior_non_mod3_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        interior_non_mod3_residue_separator_replication_status:
            interior_non_mod3_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        interior_non_mod3_residue_separator_replication_target:
            interior_non_mod3_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        interior_non_mod3_retired_candidate_count: screen
            .interior_non_mod3_retirement_summary
            .len(),
        interior_non_mod3_retired_candidate_ids: screen
            .interior_non_mod3_retirement_summary
            .iter()
            .map(|row| row.candidate_id.clone())
            .collect(),
        interior_non_mod3_next_family_picker_decision: interior_non_mod3_next_family_picker
            .map(|picker| picker.picker_decision.clone()),
        selected_next_interior_non_mod3_candidate_id:
            selected_next_interior_non_mod3_candidate.map(|candidate| {
                format!(
                    "interior-width{}-position{}-digit{}-connector-{}",
                    candidate.width, candidate.position, candidate.digit, candidate.connector
                )
            }),
        selected_next_interior_non_mod3_candidate_target: interior_non_mod3_next_family_picker
            .map(|picker| picker.next_experiment_target.clone()),
        interior_non_mod3_next_family_independent_replication_decision:
            interior_non_mod3_next_family_independent_replication
                .map(|replication| replication.replication_decision.clone()),
        interior_non_mod3_next_family_independent_replication_target:
            interior_non_mod3_next_family_independent_replication
                .map(|replication| replication.next_experiment_target.clone()),
        interior_non_mod3_next_residue_profile_decision: interior_non_mod3_next_residue_profile
            .map(|profile| profile.profile_decision.clone()),
        interior_non_mod3_next_residue_profile_best_modulus:
            interior_non_mod3_next_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        interior_non_mod3_next_residue_profile_target: interior_non_mod3_next_residue_profile
            .map(|profile| profile.next_experiment_target.clone()),
        interior_non_mod3_next_residue_separator_replication_decision:
            interior_non_mod3_next_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        interior_non_mod3_next_residue_separator_replication_status:
            interior_non_mod3_next_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        interior_non_mod3_next_residue_separator_replication_target:
            interior_non_mod3_next_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        interior_non_mod3_post_retirement_family_picker_decision:
            interior_non_mod3_post_retirement_family_picker
                .map(|picker| picker.picker_decision.clone()),
        selected_post_retirement_interior_non_mod3_candidate_id:
            selected_post_retirement_interior_non_mod3_candidate.map(|candidate| {
                format!(
                    "interior-width{}-position{}-digit{}-connector-{}",
                    candidate.width, candidate.position, candidate.digit, candidate.connector
                )
            }),
        selected_post_retirement_interior_non_mod3_candidate_target:
            interior_non_mod3_post_retirement_family_picker
                .map(|picker| picker.next_experiment_target.clone()),
        interior_non_mod3_post_retirement_family_independent_replication_decision:
            interior_non_mod3_post_retirement_family_independent_replication
                .map(|replication| replication.replication_decision.clone()),
        interior_non_mod3_post_retirement_family_independent_replication_target:
            interior_non_mod3_post_retirement_family_independent_replication
                .map(|replication| replication.next_experiment_target.clone()),
        interior_non_mod3_post_retirement_residue_profile_decision:
            interior_non_mod3_post_retirement_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        interior_non_mod3_post_retirement_residue_profile_best_modulus:
            interior_non_mod3_post_retirement_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        interior_non_mod3_post_retirement_residue_profile_target:
            interior_non_mod3_post_retirement_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        interior_non_mod3_post_retirement_residue_separator_replication_decision:
            interior_non_mod3_post_retirement_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        interior_non_mod3_post_retirement_residue_separator_replication_status:
            interior_non_mod3_post_retirement_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        interior_non_mod3_post_retirement_residue_separator_replication_target:
            interior_non_mod3_post_retirement_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        interior_non_mod3_after_third_retirement_family_picker_decision:
            interior_non_mod3_after_third_retirement_family_picker
                .map(|picker| picker.picker_decision.clone()),
        selected_after_third_retirement_interior_non_mod3_candidate_id:
            selected_after_third_retirement_interior_non_mod3_candidate.map(|candidate| {
                format!(
                    "interior-width{}-position{}-digit{}-connector-{}",
                    candidate.width, candidate.position, candidate.digit, candidate.connector
                )
            }),
        selected_after_third_retirement_interior_non_mod3_candidate_target:
            interior_non_mod3_after_third_retirement_family_picker
                .map(|picker| picker.next_experiment_target.clone()),
        interior_non_mod3_after_third_retirement_family_independent_replication_decision:
            interior_non_mod3_after_third_retirement_family_independent_replication
                .map(|replication| replication.replication_decision.clone()),
        interior_non_mod3_after_third_retirement_family_independent_replication_target:
            interior_non_mod3_after_third_retirement_family_independent_replication
                .map(|replication| replication.next_experiment_target.clone()),
        interior_non_mod3_after_third_retirement_residue_profile_decision:
            interior_non_mod3_after_third_retirement_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        interior_non_mod3_after_third_retirement_residue_profile_best_modulus:
            interior_non_mod3_after_third_retirement_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        interior_non_mod3_after_third_retirement_residue_profile_target:
            interior_non_mod3_after_third_retirement_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        interior_non_mod3_after_third_retirement_residue_separator_replication_decision:
            interior_non_mod3_after_third_retirement_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        interior_non_mod3_after_third_retirement_residue_separator_replication_status:
            interior_non_mod3_after_third_retirement_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        interior_non_mod3_after_third_retirement_residue_separator_replication_target:
            interior_non_mod3_after_third_retirement_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        interior_non_mod3_after_fourth_retirement_family_picker_decision:
            interior_non_mod3_after_fourth_retirement_family_picker
                .map(|picker| picker.picker_decision.clone()),
        selected_after_fourth_retirement_interior_non_mod3_candidate_id:
            selected_after_fourth_retirement_interior_non_mod3_candidate.map(|candidate| {
                format!(
                    "interior-width{}-position{}-digit{}-connector-{}",
                    candidate.width, candidate.position, candidate.digit, candidate.connector
                )
            }),
        selected_after_fourth_retirement_interior_non_mod3_candidate_target:
            interior_non_mod3_after_fourth_retirement_family_picker
                .map(|picker| picker.next_experiment_target.clone()),
        interior_non_mod3_after_fourth_retirement_family_independent_replication_decision:
            interior_non_mod3_after_fourth_retirement_family_independent_replication
                .map(|replication| replication.replication_decision.clone()),
        interior_non_mod3_after_fourth_retirement_family_independent_replication_target:
            interior_non_mod3_after_fourth_retirement_family_independent_replication
                .map(|replication| replication.next_experiment_target.clone()),
        interior_non_mod3_after_fourth_retirement_residue_profile_decision:
            interior_non_mod3_after_fourth_retirement_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        interior_non_mod3_after_fourth_retirement_residue_profile_best_modulus:
            interior_non_mod3_after_fourth_retirement_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        interior_non_mod3_after_fourth_retirement_residue_profile_target:
            interior_non_mod3_after_fourth_retirement_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        interior_non_mod3_after_fourth_retirement_residue_separator_replication_decision:
            interior_non_mod3_after_fourth_retirement_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        interior_non_mod3_after_fourth_retirement_residue_separator_replication_status:
            interior_non_mod3_after_fourth_retirement_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        interior_non_mod3_after_fourth_retirement_residue_separator_replication_target:
            interior_non_mod3_after_fourth_retirement_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        interior_non_mod3_after_fifth_retirement_family_picker_decision:
            interior_non_mod3_after_fifth_retirement_family_picker
                .map(|picker| picker.picker_decision.clone()),
        selected_after_fifth_retirement_interior_non_mod3_candidate_id:
            selected_after_fifth_retirement_interior_non_mod3_candidate.map(|candidate| {
                format!(
                    "interior-width{}-position{}-digit{}-connector-{}",
                    candidate.width, candidate.position, candidate.digit, candidate.connector
                )
            }),
        selected_after_fifth_retirement_interior_non_mod3_candidate_target:
            interior_non_mod3_after_fifth_retirement_family_picker
                .map(|picker| picker.next_experiment_target.clone()),
        interior_non_mod3_after_fifth_retirement_family_independent_replication_decision:
            interior_non_mod3_after_fifth_retirement_family_independent_replication
                .map(|replication| replication.replication_decision.clone()),
        interior_non_mod3_after_fifth_retirement_family_independent_replication_target:
            interior_non_mod3_after_fifth_retirement_family_independent_replication
                .map(|replication| replication.next_experiment_target.clone()),
        interior_non_mod3_after_sixth_retirement_family_picker_decision:
            interior_non_mod3_after_sixth_retirement_family_picker
                .map(|picker| picker.picker_decision.clone()),
        selected_after_sixth_retirement_interior_non_mod3_candidate_id:
            selected_after_sixth_retirement_interior_non_mod3_candidate.map(|candidate| {
                format!(
                    "interior-width{}-position{}-digit{}-connector-{}",
                    candidate.width, candidate.position, candidate.digit, candidate.connector
                )
            }),
        selected_after_sixth_retirement_interior_non_mod3_candidate_target:
            interior_non_mod3_after_sixth_retirement_family_picker
                .map(|picker| picker.next_experiment_target.clone()),
        interior_non_mod3_after_sixth_retirement_family_independent_replication_decision:
            interior_non_mod3_after_sixth_retirement_family_independent_replication
                .map(|replication| replication.replication_decision.clone()),
        interior_non_mod3_after_sixth_retirement_family_independent_replication_target:
            interior_non_mod3_after_sixth_retirement_family_independent_replication
                .map(|replication| replication.next_experiment_target.clone()),
        single_digit_interior_pivot_decision: Some(
            screen.single_digit_interior_pivot_decision.clone(),
        ),
        multi_digit_motif_family_picker_decision: multi_digit_motif_family_picker
            .map(|picker| picker.picker_decision.clone()),
        selected_multi_digit_motif_id: selected_multi_digit_motif.map(|motif| {
            format!(
                "multidigit-motif-width{}-start{}-digits{}-connector-{}",
                motif.width,
                motif.start_position,
                motif.motif_digits
                    .iter()
                    .map(|digit| digit.to_string())
                    .collect::<Vec<_>>()
                    .join(""),
                motif.connector
            )
        }),
        selected_multi_digit_motif_target: multi_digit_motif_family_picker
            .map(|picker| picker.next_experiment_target.clone()),
        multi_digit_motif_family_independent_replication_decision:
            multi_digit_motif_family_independent_replication
                .map(|replication| replication.replication_decision.clone()),
        multi_digit_motif_family_independent_replication_target:
            multi_digit_motif_family_independent_replication
                .map(|replication| replication.next_experiment_target.clone()),
        multi_digit_motif_residue_profile_decision: multi_digit_motif_residue_profile
            .map(|profile| profile.profile_decision.clone()),
        multi_digit_motif_residue_profile_best_modulus: multi_digit_motif_residue_profile
            .and_then(|profile| profile.best_separator.as_ref())
            .map(|separator| separator.modulus),
        multi_digit_motif_residue_profile_target: multi_digit_motif_residue_profile
            .map(|profile| profile.next_experiment_target.clone()),
        multi_digit_motif_residue_separator_replication_decision:
            multi_digit_motif_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        multi_digit_motif_residue_separator_replication_status:
            multi_digit_motif_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        multi_digit_motif_residue_separator_replication_target:
            multi_digit_motif_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        multi_digit_motif_retired_count: screen.multi_digit_motif_retirement_summary.len(),
        retired_multi_digit_motif_ids: screen
            .multi_digit_motif_retirement_summary
            .iter()
            .map(|row| row.source_motif_id.clone())
            .collect(),
        orthogonal_pair_family_retired_count: screen
            .orthogonal_pair_family_retirement_summary
            .len(),
        retired_orthogonal_pair_family_branch_ids: screen
            .orthogonal_pair_family_retirement_summary
            .iter()
            .map(|row| row.source_branch_id.clone())
            .collect(),
        orthogonal_pair_family_control_matrix_decision: orthogonal_pair_family_control_matrix
            .map(|matrix| matrix.matrix_decision.clone()),
        selected_orthogonal_pair_family_branch_id: selected_orthogonal_pair_family_branch
            .map(|branch| branch.branch_id.clone()),
        selected_orthogonal_pair_family: selected_orthogonal_pair_family_branch
            .map(|branch| branch.pair_family.clone()),
        selected_orthogonal_pair_family_connector: selected_orthogonal_pair_family_branch
            .map(|branch| branch.connector.clone()),
        selected_orthogonal_pair_family_target: selected_orthogonal_pair_family_branch
            .map(|branch| branch.next_experiment_target.clone()),
        orthogonal_pair_family_residue_profile_decision: orthogonal_pair_family_residue_profile
            .map(|profile| profile.profile_decision.clone()),
        orthogonal_pair_family_residue_profile_best_modulus:
            orthogonal_pair_family_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        orthogonal_pair_family_residue_profile_target: orthogonal_pair_family_residue_profile
            .map(|profile| profile.next_experiment_target.clone()),
        orthogonal_pair_family_residue_separator_replication_decision:
            orthogonal_pair_family_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        orthogonal_pair_family_residue_separator_replication_status:
            orthogonal_pair_family_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        orthogonal_pair_family_residue_separator_replication_target:
            orthogonal_pair_family_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        orthogonal_compact_three_digit_control_decision:
            orthogonal_compact_three_digit_control.map(|matrix| matrix.matrix_decision.clone()),
        selected_orthogonal_compact_three_digit_branch_id:
            selected_orthogonal_compact_three_digit_branch.map(|branch| branch.branch_id.clone()),
        selected_orthogonal_compact_three_digit_family:
            selected_orthogonal_compact_three_digit_branch.map(|branch| branch.pair_family.clone()),
        selected_orthogonal_compact_three_digit_connector:
            selected_orthogonal_compact_three_digit_branch.map(|branch| branch.connector.clone()),
        selected_orthogonal_compact_three_digit_target:
            selected_orthogonal_compact_three_digit_branch
                .map(|branch| branch.next_experiment_target.clone()),
        orthogonal_compact_three_digit_residue_profile_decision:
            orthogonal_compact_three_digit_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        orthogonal_compact_three_digit_residue_profile_best_modulus:
            orthogonal_compact_three_digit_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        orthogonal_compact_three_digit_residue_profile_target:
            orthogonal_compact_three_digit_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        orthogonal_compact_three_digit_residue_separator_replication_decision:
            orthogonal_compact_three_digit_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        orthogonal_compact_three_digit_residue_separator_replication_status:
            orthogonal_compact_three_digit_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        orthogonal_compact_three_digit_residue_separator_replication_target:
            orthogonal_compact_three_digit_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        orthogonal_nonadjacent_two_digit_control_decision:
            orthogonal_nonadjacent_two_digit_control.map(|matrix| matrix.matrix_decision.clone()),
        selected_orthogonal_nonadjacent_two_digit_branch_id:
            selected_orthogonal_nonadjacent_two_digit_branch.map(|branch| branch.branch_id.clone()),
        selected_orthogonal_nonadjacent_two_digit_family:
            selected_orthogonal_nonadjacent_two_digit_branch
                .map(|branch| branch.pair_family.clone()),
        selected_orthogonal_nonadjacent_two_digit_connector:
            selected_orthogonal_nonadjacent_two_digit_branch
                .map(|branch| branch.connector.clone()),
        selected_orthogonal_nonadjacent_two_digit_target:
            selected_orthogonal_nonadjacent_two_digit_branch
                .map(|branch| branch.next_experiment_target.clone()),
        orthogonal_nonadjacent_two_digit_residue_profile_decision:
            orthogonal_nonadjacent_two_digit_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        orthogonal_nonadjacent_two_digit_residue_profile_best_modulus:
            orthogonal_nonadjacent_two_digit_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        orthogonal_nonadjacent_two_digit_residue_profile_target:
            orthogonal_nonadjacent_two_digit_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        orthogonal_nonadjacent_two_digit_residue_separator_replication_decision:
            orthogonal_nonadjacent_two_digit_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        orthogonal_nonadjacent_two_digit_residue_separator_replication_status:
            orthogonal_nonadjacent_two_digit_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        orthogonal_nonadjacent_two_digit_residue_separator_replication_target:
            orthogonal_nonadjacent_two_digit_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        orthogonal_edge_plus_interior_control_decision: orthogonal_edge_plus_interior_control
            .map(|matrix| matrix.matrix_decision.clone()),
        selected_orthogonal_edge_plus_interior_branch_id:
            selected_orthogonal_edge_plus_interior_branch.map(|branch| branch.branch_id.clone()),
        selected_orthogonal_edge_plus_interior_family: selected_orthogonal_edge_plus_interior_branch
            .map(|branch| branch.pair_family.clone()),
        selected_orthogonal_edge_plus_interior_connector:
            selected_orthogonal_edge_plus_interior_branch.map(|branch| branch.connector.clone()),
        selected_orthogonal_edge_plus_interior_target: selected_orthogonal_edge_plus_interior_branch
            .map(|branch| branch.next_experiment_target.clone()),
        orthogonal_edge_plus_interior_residue_profile_decision:
            orthogonal_edge_plus_interior_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        orthogonal_edge_plus_interior_residue_profile_best_modulus:
            orthogonal_edge_plus_interior_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        orthogonal_edge_plus_interior_residue_profile_target:
            orthogonal_edge_plus_interior_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        orthogonal_edge_plus_interior_residue_separator_replication_decision:
            orthogonal_edge_plus_interior_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        orthogonal_edge_plus_interior_residue_separator_replication_status:
            orthogonal_edge_plus_interior_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        orthogonal_edge_plus_interior_residue_separator_replication_target:
            orthogonal_edge_plus_interior_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        orthogonal_repeated_block_control_decision: orthogonal_repeated_block_control
            .map(|matrix| matrix.matrix_decision.clone()),
        selected_orthogonal_repeated_block_branch_id: selected_orthogonal_repeated_block_branch
            .map(|branch| branch.branch_id.clone()),
        selected_orthogonal_repeated_block_family: selected_orthogonal_repeated_block_branch
            .map(|branch| branch.pair_family.clone()),
        selected_orthogonal_repeated_block_connector: selected_orthogonal_repeated_block_branch
            .map(|branch| branch.connector.clone()),
        selected_orthogonal_repeated_block_target: selected_orthogonal_repeated_block_branch
            .map(|branch| branch.next_experiment_target.clone()),
        orthogonal_repeated_block_residue_profile_decision:
            orthogonal_repeated_block_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        orthogonal_repeated_block_residue_profile_best_modulus:
            orthogonal_repeated_block_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        orthogonal_repeated_block_residue_profile_target:
            orthogonal_repeated_block_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        orthogonal_repeated_block_residue_separator_replication_decision:
            orthogonal_repeated_block_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        orthogonal_repeated_block_residue_separator_replication_status:
            orthogonal_repeated_block_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        orthogonal_repeated_block_residue_separator_replication_target:
            orthogonal_repeated_block_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        orthogonal_arithmetic_connector_control_decision: orthogonal_arithmetic_connector_control
            .map(|matrix| matrix.matrix_decision.clone()),
        selected_orthogonal_arithmetic_connector_branch_id:
            selected_orthogonal_arithmetic_connector_branch.map(|branch| branch.branch_id.clone()),
        selected_orthogonal_arithmetic_connector_family:
            selected_orthogonal_arithmetic_connector_branch
                .map(|branch| branch.pair_family.clone()),
        selected_orthogonal_arithmetic_connector_connector:
            selected_orthogonal_arithmetic_connector_branch.map(|branch| branch.connector.clone()),
        selected_orthogonal_arithmetic_connector_target:
            selected_orthogonal_arithmetic_connector_branch
                .map(|branch| branch.next_experiment_target.clone()),
        orthogonal_arithmetic_connector_residue_profile_decision:
            orthogonal_arithmetic_connector_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        orthogonal_arithmetic_connector_residue_profile_best_modulus:
            orthogonal_arithmetic_connector_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        orthogonal_arithmetic_connector_residue_profile_target:
            orthogonal_arithmetic_connector_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        orthogonal_arithmetic_connector_residue_separator_replication_decision:
            orthogonal_arithmetic_connector_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        orthogonal_arithmetic_connector_residue_separator_replication_status:
            orthogonal_arithmetic_connector_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        orthogonal_arithmetic_connector_residue_separator_replication_target:
            orthogonal_arithmetic_connector_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        orthogonal_residue_lattice_connector_control_decision:
            orthogonal_residue_lattice_connector_control
                .map(|matrix| matrix.matrix_decision.clone()),
        selected_orthogonal_residue_lattice_connector_branch_id:
            selected_orthogonal_residue_lattice_connector_branch
                .map(|branch| branch.branch_id.clone()),
        selected_orthogonal_residue_lattice_connector_family:
            selected_orthogonal_residue_lattice_connector_branch
                .map(|branch| branch.pair_family.clone()),
        selected_orthogonal_residue_lattice_connector_connector:
            selected_orthogonal_residue_lattice_connector_branch
                .map(|branch| branch.connector.clone()),
        selected_orthogonal_residue_lattice_connector_target:
            selected_orthogonal_residue_lattice_connector_branch
                .map(|branch| branch.next_experiment_target.clone()),
        orthogonal_residue_lattice_connector_residue_profile_decision:
            orthogonal_residue_lattice_connector_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        orthogonal_residue_lattice_connector_residue_profile_best_modulus:
            orthogonal_residue_lattice_connector_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        orthogonal_residue_lattice_connector_residue_profile_target:
            orthogonal_residue_lattice_connector_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        orthogonal_residue_lattice_connector_residue_separator_replication_decision:
            orthogonal_residue_lattice_connector_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        orthogonal_residue_lattice_connector_residue_separator_replication_status:
            orthogonal_residue_lattice_connector_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        orthogonal_residue_lattice_connector_residue_separator_replication_target:
            orthogonal_residue_lattice_connector_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        orthogonal_modular_walk_connector_control_decision:
            orthogonal_modular_walk_connector_control
                .map(|matrix| matrix.matrix_decision.clone()),
        selected_orthogonal_modular_walk_connector_branch_id:
            selected_orthogonal_modular_walk_connector_branch
                .map(|branch| branch.branch_id.clone()),
        selected_orthogonal_modular_walk_connector_family:
            selected_orthogonal_modular_walk_connector_branch
                .map(|branch| branch.pair_family.clone()),
        selected_orthogonal_modular_walk_connector_connector:
            selected_orthogonal_modular_walk_connector_branch.map(|branch| branch.connector.clone()),
        selected_orthogonal_modular_walk_connector_target:
            selected_orthogonal_modular_walk_connector_branch
                .map(|branch| branch.next_experiment_target.clone()),
        orthogonal_modular_walk_connector_residue_profile_decision:
            orthogonal_modular_walk_connector_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        orthogonal_modular_walk_connector_residue_profile_best_modulus:
            orthogonal_modular_walk_connector_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        orthogonal_modular_walk_connector_residue_profile_target:
            orthogonal_modular_walk_connector_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        orthogonal_modular_walk_connector_residue_separator_replication_decision:
            orthogonal_modular_walk_connector_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        orthogonal_modular_walk_connector_residue_separator_replication_status:
            orthogonal_modular_walk_connector_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        orthogonal_modular_walk_connector_residue_separator_replication_target:
            orthogonal_modular_walk_connector_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        orthogonal_arithmetic_family_registry_decision: orthogonal_arithmetic_family_registry
            .map(|registry| registry.registry_decision.clone()),
        orthogonal_arithmetic_family_registry_retired_count: orthogonal_arithmetic_family_registry
            .map(|registry| registry.retired_family_count),
        orthogonal_arithmetic_family_registry_selected_family: orthogonal_arithmetic_family_registry
            .and_then(|registry| registry.selected_next_family_id.clone()),
        orthogonal_arithmetic_family_registry_selected_target: orthogonal_arithmetic_family_registry
            .and_then(|registry| registry.selected_next_family_target.clone()),
        orthogonal_crt_paired_connector_control_decision:
            orthogonal_crt_paired_connector_control.map(|matrix| matrix.matrix_decision.clone()),
        selected_orthogonal_crt_paired_connector_branch_id:
            selected_orthogonal_crt_paired_connector_branch.map(|branch| branch.branch_id.clone()),
        selected_orthogonal_crt_paired_connector_family:
            selected_orthogonal_crt_paired_connector_branch
                .map(|branch| branch.pair_family.clone()),
        selected_orthogonal_crt_paired_connector_connector:
            selected_orthogonal_crt_paired_connector_branch.map(|branch| branch.connector.clone()),
        selected_orthogonal_crt_paired_connector_target:
            selected_orthogonal_crt_paired_connector_branch
                .map(|branch| branch.next_experiment_target.clone()),
        orthogonal_crt_paired_connector_residue_profile_decision:
            orthogonal_crt_paired_connector_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        orthogonal_crt_paired_connector_residue_profile_best_modulus:
            orthogonal_crt_paired_connector_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        orthogonal_crt_paired_connector_residue_profile_target:
            orthogonal_crt_paired_connector_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        orthogonal_crt_paired_connector_residue_separator_replication_decision:
            orthogonal_crt_paired_connector_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        orthogonal_crt_paired_connector_residue_separator_replication_status:
            orthogonal_crt_paired_connector_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        orthogonal_crt_paired_connector_residue_separator_replication_target:
            orthogonal_crt_paired_connector_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        orthogonal_multiplicative_order_connector_control_decision:
            orthogonal_multiplicative_order_connector_control
                .map(|matrix| matrix.matrix_decision.clone()),
        selected_orthogonal_multiplicative_order_connector_branch_id:
            selected_orthogonal_multiplicative_order_connector_branch
                .map(|branch| branch.branch_id.clone()),
        selected_orthogonal_multiplicative_order_connector_family:
            selected_orthogonal_multiplicative_order_connector_branch
                .map(|branch| branch.pair_family.clone()),
        selected_orthogonal_multiplicative_order_connector_connector:
            selected_orthogonal_multiplicative_order_connector_branch
                .map(|branch| branch.connector.clone()),
        selected_orthogonal_multiplicative_order_connector_target:
            selected_orthogonal_multiplicative_order_connector_branch
                .map(|branch| branch.next_experiment_target.clone()),
        orthogonal_multiplicative_order_connector_residue_profile_decision:
            orthogonal_multiplicative_order_connector_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        orthogonal_multiplicative_order_connector_residue_profile_best_modulus:
            orthogonal_multiplicative_order_connector_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        orthogonal_multiplicative_order_connector_residue_profile_target:
            orthogonal_multiplicative_order_connector_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        orthogonal_multiplicative_order_connector_residue_separator_replication_decision:
            orthogonal_multiplicative_order_connector_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        orthogonal_multiplicative_order_connector_residue_separator_replication_status:
            orthogonal_multiplicative_order_connector_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        orthogonal_multiplicative_order_connector_residue_separator_replication_target:
            orthogonal_multiplicative_order_connector_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        orthogonal_automorphic_repunit_connector_control_decision:
            orthogonal_automorphic_repunit_connector_control
                .map(|matrix| matrix.matrix_decision.clone()),
        selected_orthogonal_automorphic_repunit_connector_branch_id:
            selected_orthogonal_automorphic_repunit_connector_branch
                .map(|branch| branch.branch_id.clone()),
        selected_orthogonal_automorphic_repunit_connector_family:
            selected_orthogonal_automorphic_repunit_connector_branch
                .map(|branch| branch.pair_family.clone()),
        selected_orthogonal_automorphic_repunit_connector_connector:
            selected_orthogonal_automorphic_repunit_connector_branch
                .map(|branch| branch.connector.clone()),
        selected_orthogonal_automorphic_repunit_connector_target:
            selected_orthogonal_automorphic_repunit_connector_branch
                .map(|branch| branch.next_experiment_target.clone()),
        orthogonal_automorphic_repunit_connector_residue_profile_decision:
            orthogonal_automorphic_repunit_connector_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        orthogonal_automorphic_repunit_connector_residue_profile_best_modulus:
            orthogonal_automorphic_repunit_connector_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        orthogonal_automorphic_repunit_connector_residue_profile_target:
            orthogonal_automorphic_repunit_connector_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        orthogonal_automorphic_repunit_connector_residue_separator_replication_decision:
            orthogonal_automorphic_repunit_connector_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        orthogonal_automorphic_repunit_connector_residue_separator_replication_status:
            orthogonal_automorphic_repunit_connector_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        orthogonal_automorphic_repunit_connector_residue_separator_replication_target:
            orthogonal_automorphic_repunit_connector_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        orthogonal_cyclic_reptend_connector_control_decision:
            orthogonal_cyclic_reptend_connector_control.map(|matrix| matrix.matrix_decision.clone()),
        selected_orthogonal_cyclic_reptend_connector_branch_id:
            selected_orthogonal_cyclic_reptend_connector_branch
                .map(|branch| branch.branch_id.clone()),
        selected_orthogonal_cyclic_reptend_connector_family:
            selected_orthogonal_cyclic_reptend_connector_branch
                .map(|branch| branch.pair_family.clone()),
        selected_orthogonal_cyclic_reptend_connector_connector:
            selected_orthogonal_cyclic_reptend_connector_branch
                .map(|branch| branch.connector.clone()),
        selected_orthogonal_cyclic_reptend_connector_target:
            selected_orthogonal_cyclic_reptend_connector_branch
                .map(|branch| branch.next_experiment_target.clone()),
        orthogonal_cyclic_reptend_connector_residue_profile_decision:
            orthogonal_cyclic_reptend_connector_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        orthogonal_cyclic_reptend_connector_residue_profile_best_modulus:
            orthogonal_cyclic_reptend_connector_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        orthogonal_cyclic_reptend_connector_residue_profile_target:
            orthogonal_cyclic_reptend_connector_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        orthogonal_cyclic_reptend_connector_residue_separator_replication_decision:
            orthogonal_cyclic_reptend_connector_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        orthogonal_cyclic_reptend_connector_residue_separator_replication_status:
            orthogonal_cyclic_reptend_connector_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        orthogonal_cyclic_reptend_connector_residue_separator_replication_target:
            orthogonal_cyclic_reptend_connector_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        orthogonal_carry_chain_connector_control_decision:
            orthogonal_carry_chain_connector_control.map(|matrix| matrix.matrix_decision.clone()),
        selected_orthogonal_carry_chain_connector_branch_id:
            selected_orthogonal_carry_chain_connector_branch.map(|branch| branch.branch_id.clone()),
        selected_orthogonal_carry_chain_connector_family:
            selected_orthogonal_carry_chain_connector_branch
                .map(|branch| branch.pair_family.clone()),
        selected_orthogonal_carry_chain_connector_connector:
            selected_orthogonal_carry_chain_connector_branch
                .map(|branch| branch.connector.clone()),
        selected_orthogonal_carry_chain_connector_target:
            selected_orthogonal_carry_chain_connector_branch
                .map(|branch| branch.next_experiment_target.clone()),
        orthogonal_carry_chain_connector_residue_profile_decision:
            orthogonal_carry_chain_connector_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        orthogonal_carry_chain_connector_residue_profile_best_modulus:
            orthogonal_carry_chain_connector_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        orthogonal_carry_chain_connector_residue_profile_target:
            orthogonal_carry_chain_connector_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        orthogonal_carry_chain_connector_residue_separator_replication_decision:
            orthogonal_carry_chain_connector_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        orthogonal_carry_chain_connector_residue_separator_replication_status:
            orthogonal_carry_chain_connector_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        orthogonal_carry_chain_connector_residue_separator_replication_target:
            orthogonal_carry_chain_connector_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        orthogonal_base_mixed_connector_control_decision:
            orthogonal_base_mixed_connector_control.map(|matrix| matrix.matrix_decision.clone()),
        orthogonal_base_mixed_connector_control_target: orthogonal_base_mixed_connector_control
            .map(|matrix| matrix.next_experiment_target.clone()),
        selected_orthogonal_base_mixed_connector_branch_id:
            selected_orthogonal_base_mixed_connector_branch.map(|branch| branch.branch_id.clone()),
        selected_orthogonal_base_mixed_connector_family:
            selected_orthogonal_base_mixed_connector_branch.map(|branch| branch.pair_family.clone()),
        selected_orthogonal_base_mixed_connector_connector:
            selected_orthogonal_base_mixed_connector_branch.map(|branch| branch.connector.clone()),
        selected_orthogonal_base_mixed_connector_target:
            selected_orthogonal_base_mixed_connector_branch
                .map(|branch| branch.next_experiment_target.clone()),
        orthogonal_base_mixed_connector_residue_profile_decision:
            orthogonal_base_mixed_connector_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        orthogonal_base_mixed_connector_residue_profile_best_modulus:
            orthogonal_base_mixed_connector_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        orthogonal_base_mixed_connector_residue_profile_target:
            orthogonal_base_mixed_connector_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        orthogonal_base_mixed_connector_residue_separator_replication_decision:
            orthogonal_base_mixed_connector_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        orthogonal_base_mixed_connector_residue_separator_replication_status:
            orthogonal_base_mixed_connector_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        orthogonal_base_mixed_connector_residue_separator_replication_target:
            orthogonal_base_mixed_connector_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        connector_stress_meta_atlas_decision: connector_stress_meta_atlas
            .map(|atlas| atlas.atlas_decision.clone()),
        connector_stress_meta_atlas_retired_count: connector_stress_meta_atlas
            .map(|atlas| atlas.retired_branch_class_count),
        connector_stress_meta_atlas_selected_surface: connector_stress_meta_atlas
            .and_then(|atlas| atlas.selected_surface.clone()),
        connector_stress_meta_atlas_selected_target: connector_stress_meta_atlas
            .and_then(|atlas| atlas.selected_target.clone()),
        pair_family_gap_portfolio_control_decision: pair_family_gap_portfolio_control
            .map(|matrix| matrix.matrix_decision.clone()),
        pair_family_gap_portfolio_control_target: pair_family_gap_portfolio_control
            .map(|matrix| matrix.next_experiment_target.clone()),
        selected_pair_family_gap_portfolio_branch_id: selected_pair_family_gap_portfolio_branch
            .map(|branch| branch.branch_id.clone()),
        selected_pair_family_gap_portfolio_family: selected_pair_family_gap_portfolio_branch
            .map(|branch| branch.pair_family.clone()),
        selected_pair_family_gap_portfolio_connector: selected_pair_family_gap_portfolio_branch
            .map(|branch| branch.connector.clone()),
        selected_pair_family_gap_portfolio_target: selected_pair_family_gap_portfolio_branch
            .map(|branch| branch.next_experiment_target.clone()),
        pair_family_gap_portfolio_residue_profile_decision:
            pair_family_gap_portfolio_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        pair_family_gap_portfolio_residue_profile_best_modulus:
            pair_family_gap_portfolio_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        pair_family_gap_portfolio_residue_profile_target:
            pair_family_gap_portfolio_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        pair_family_gap_portfolio_residue_separator_replication_decision:
            pair_family_gap_portfolio_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        pair_family_gap_portfolio_residue_separator_replication_status:
            pair_family_gap_portfolio_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        pair_family_gap_portfolio_residue_separator_replication_target:
            pair_family_gap_portfolio_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        pair_family_gap_extension_control_decision: pair_family_gap_extension_control
            .map(|matrix| matrix.matrix_decision.clone()),
        pair_family_gap_extension_control_target: pair_family_gap_extension_control
            .map(|matrix| matrix.next_experiment_target.clone()),
        selected_pair_family_gap_extension_branch_id: selected_pair_family_gap_extension_branch
            .map(|branch| branch.branch_id.clone()),
        selected_pair_family_gap_extension_family: selected_pair_family_gap_extension_branch
            .map(|branch| branch.pair_family.clone()),
        selected_pair_family_gap_extension_connector: selected_pair_family_gap_extension_branch
            .map(|branch| branch.connector.clone()),
        selected_pair_family_gap_extension_target: selected_pair_family_gap_extension_branch
            .map(|branch| branch.next_experiment_target.clone()),
        pair_family_gap_extension_residue_profile_decision:
            pair_family_gap_extension_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        pair_family_gap_extension_residue_profile_best_modulus:
            pair_family_gap_extension_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        pair_family_gap_extension_residue_profile_target:
            pair_family_gap_extension_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        pair_family_gap_extension_residue_separator_replication_decision:
            pair_family_gap_extension_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        pair_family_gap_extension_residue_separator_replication_status:
            pair_family_gap_extension_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        pair_family_gap_extension_residue_separator_replication_target:
            pair_family_gap_extension_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        pair_family_size_band_control_decision: pair_family_size_band_control
            .map(|matrix| matrix.matrix_decision.clone()),
        pair_family_size_band_control_target: pair_family_size_band_control
            .map(|matrix| matrix.next_experiment_target.clone()),
        selected_pair_family_size_band_branch_id: selected_pair_family_size_band_branch
            .map(|branch| branch.branch_id.clone()),
        selected_pair_family_size_band_family: selected_pair_family_size_band_branch
            .map(|branch| branch.pair_family.clone()),
        selected_pair_family_size_band_connector: selected_pair_family_size_band_branch
            .map(|branch| branch.connector.clone()),
        selected_pair_family_size_band_target: selected_pair_family_size_band_branch
            .map(|branch| branch.next_experiment_target.clone()),
        pair_family_size_band_residue_profile_decision: pair_family_size_band_residue_profile
            .map(|profile| profile.profile_decision.clone()),
        pair_family_size_band_residue_profile_best_modulus: pair_family_size_band_residue_profile
            .and_then(|profile| profile.best_separator.as_ref())
            .map(|separator| separator.modulus),
        pair_family_size_band_residue_profile_target: pair_family_size_band_residue_profile
            .map(|profile| profile.next_experiment_target.clone()),
        pair_family_size_band_residue_separator_replication_decision:
            pair_family_size_band_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        pair_family_size_band_residue_separator_replication_status:
            pair_family_size_band_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        pair_family_size_band_residue_separator_replication_target:
            pair_family_size_band_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        replication_null_atlas_schema_version: replication_null_atlas.schema_version.clone(),
        replication_null_atlas_status: replication_null_atlas
            .summary
            .single_branch_separator_stability_status
            .clone(),
        replication_null_atlas_branch_row_count: replication_null_atlas.summary.branch_row_count,
        replication_null_atlas_retained_separator_count: replication_null_atlas
            .summary
            .separator_retained_count,
        replication_null_atlas_split_separator_count: replication_null_atlas
            .summary
            .separator_split_count,
        replication_null_atlas_collapsed_separator_count: replication_null_atlas
            .summary
            .separator_collapsed_count,
        replication_null_atlas_theorem_candidate_count: replication_null_atlas
            .summary
            .theorem_candidate_count,
        replication_null_atlas_next_target: replication_null_atlas
            .summary
            .next_experiment_target
            .clone(),
        pair_family_cohort_retention_picker_decision: pair_family_cohort_retention_picker
            .map(|picker| picker.picker_decision.clone()),
        selected_pair_family_cohort_id: selected_pair_family_cohort
            .map(|cohort| cohort.cohort_id.clone()),
        selected_pair_family_cohort_connector: selected_pair_family_cohort
            .map(|cohort| cohort.connector.clone()),
        selected_pair_family_cohort_target: selected_pair_family_cohort
            .map(|cohort| cohort.next_experiment_target.clone()),
        pair_family_cohort_residue_profile_decision: pair_family_cohort_residue_profile
            .map(|profile| profile.profile_decision.clone()),
        pair_family_cohort_residue_profile_exact_separator_count:
            pair_family_cohort_residue_profile.map(|profile| profile.exact_separator_count),
        pair_family_cohort_residue_profile_best_modulus: pair_family_cohort_residue_profile
            .and_then(|profile| profile.best_separator.as_ref())
            .map(|separator| separator.modulus),
        pair_family_cohort_residue_profile_target: pair_family_cohort_residue_profile
            .map(|profile| profile.next_experiment_target.clone()),
        pair_family_cohort_residue_separator_replication_decision:
            pair_family_cohort_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        pair_family_cohort_residue_separator_replication_status:
            pair_family_cohort_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        pair_family_cohort_residue_separator_replication_target:
            pair_family_cohort_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        pair_family_surface_picker_decision: pair_family_surface_picker
            .map(|picker| picker.picker_decision.clone()),
        selected_pair_family_surface_id: selected_pair_family_surface
            .map(|surface| surface.surface_id.clone()),
        selected_pair_family_surface_label: selected_pair_family_surface
            .map(|surface| surface.surface_label.clone()),
        selected_pair_family_surface_target: selected_pair_family_surface
            .map(|surface| surface.next_experiment_target.clone()),
        pair_family_surface_residue_profile_decision: pair_family_surface_residue_profile
            .map(|profile| profile.profile_decision.clone()),
        pair_family_surface_residue_profile_exact_separator_count:
            pair_family_surface_residue_profile.map(|profile| profile.exact_separator_count),
        pair_family_surface_residue_profile_best_modulus: pair_family_surface_residue_profile
            .and_then(|profile| profile.best_separator.as_ref())
            .map(|separator| separator.modulus),
        pair_family_surface_residue_profile_target: pair_family_surface_residue_profile
            .map(|profile| profile.next_experiment_target.clone()),
        pair_family_topn_motif_surface_profile_decision:
            pair_family_topn_motif_surface_profile
                .map(|profile| profile.profile_decision.clone()),
        pair_family_topn_motif_surface_profile_top_n: pair_family_topn_motif_surface_profile
            .map(|profile| profile.top_n_per_family),
        pair_family_topn_motif_surface_profile_source_motif_count:
            pair_family_topn_motif_surface_profile.map(|profile| profile.source_motif_count),
        pair_family_topn_motif_surface_profile_fresh_survivor_count:
            pair_family_topn_motif_surface_profile
                .map(|profile| profile.fresh_survivor_motif_count),
        pair_family_topn_motif_surface_profile_exact_separator_count:
            pair_family_topn_motif_surface_profile.map(|profile| profile.exact_separator_count),
        pair_family_topn_motif_surface_profile_best_modulus:
            pair_family_topn_motif_surface_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        pair_family_topn_motif_surface_profile_target: pair_family_topn_motif_surface_profile
            .map(|profile| profile.next_experiment_target.clone()),
        pair_family_gap_cohort_geometry_control_decision: pair_family_gap_cohort_geometry_control
            .map(|control| control.control_decision.clone()),
        pair_family_gap_cohort_geometry_control_top_n: pair_family_gap_cohort_geometry_control
            .map(|control| control.top_n_per_family),
        pair_family_gap_cohort_geometry_control_source_motif_count:
            pair_family_gap_cohort_geometry_control.map(|control| control.source_motif_count),
        pair_family_gap_cohort_geometry_control_retained_geometry_count:
            pair_family_gap_cohort_geometry_control.map(|control| control.retained_geometry_count),
        pair_family_gap_cohort_geometry_control_selected_connector:
            pair_family_gap_cohort_geometry_control
                .and_then(|control| control.selected_geometry.as_ref())
                .map(|row| row.connector.clone()),
        pair_family_gap_cohort_geometry_control_target: pair_family_gap_cohort_geometry_control
            .map(|control| control.next_experiment_target.clone()),
        pair_family_gap_cohort_residue_profile_decision:
            pair_family_gap_cohort_residue_profile.map(|profile| profile.profile_decision.clone()),
        pair_family_gap_cohort_residue_profile_exact_separator_count:
            pair_family_gap_cohort_residue_profile.map(|profile| profile.exact_separator_count),
        pair_family_gap_cohort_residue_profile_best_modulus:
            pair_family_gap_cohort_residue_profile
                .and_then(|profile| profile.best_separator.as_ref())
                .map(|separator| separator.modulus),
        pair_family_gap_cohort_residue_profile_target: pair_family_gap_cohort_residue_profile
            .map(|profile| profile.next_experiment_target.clone()),
        pair_family_gap_cohort_residue_separator_replication_decision:
            pair_family_gap_cohort_residue_separator_replication
                .map(|replication| replication.replication_decision.clone()),
        pair_family_gap_cohort_residue_separator_replication_status:
            pair_family_gap_cohort_residue_separator_replication
                .map(|replication| replication.separator_status.clone()),
        pair_family_gap_cohort_residue_separator_replication_target:
            pair_family_gap_cohort_residue_separator_replication
                .map(|replication| replication.next_experiment_target.clone()),
        pair_family_gap_cohort_ratio_geometry_control_decision:
            pair_family_gap_cohort_ratio_geometry_control
                .map(|control| control.control_decision.clone()),
        pair_family_gap_cohort_ratio_geometry_control_selected_connector:
            pair_family_gap_cohort_ratio_geometry_control
                .and_then(|control| control.selected_geometry.as_ref())
                .map(|row| row.connector.clone()),
        pair_family_gap_cohort_ratio_geometry_control_selected_bias:
            pair_family_gap_cohort_ratio_geometry_control
                .and_then(|control| control.selected_geometry.as_ref())
                .map(|row| row.retained_bias_direction.clone()),
        pair_family_gap_cohort_ratio_geometry_control_target:
            pair_family_gap_cohort_ratio_geometry_control
                .map(|control| control.next_experiment_target.clone()),
        pair_family_gap_cohort_ratio_geometry_replication_decision:
            pair_family_gap_cohort_ratio_geometry_replication
                .map(|replication| replication.replication_decision.clone()),
        pair_family_gap_cohort_ratio_geometry_replication_status:
            pair_family_gap_cohort_ratio_geometry_replication
                .map(|replication| replication.ratio_geometry_status.clone()),
        pair_family_gap_cohort_ratio_geometry_replication_target:
            pair_family_gap_cohort_ratio_geometry_replication
                .map(|replication| replication.next_experiment_target.clone()),
        pair_family_gap_cohort_ratio_geometry_expansion_decision:
            pair_family_gap_cohort_ratio_geometry_expansion
                .map(|expansion| expansion.expansion_decision.clone()),
        pair_family_gap_cohort_ratio_geometry_expansion_status:
            pair_family_gap_cohort_ratio_geometry_expansion
                .map(|expansion| expansion.expansion_status.clone()),
        pair_family_gap_cohort_ratio_geometry_expansion_target:
            pair_family_gap_cohort_ratio_geometry_expansion
                .map(|expansion| expansion.next_experiment_target.clone()),
        pair_family_gap_cohort_ratio_correction_bound_stability_decision:
            pair_family_gap_cohort_ratio_correction_bound_stability
                .map(|stability| stability.stability_decision.clone()),
        pair_family_gap_cohort_ratio_correction_bound_stability_status:
            pair_family_gap_cohort_ratio_correction_bound_stability
                .map(|stability| stability.stability_status.clone()),
        pair_family_gap_cohort_ratio_correction_bound_stable_bound_count:
            pair_family_gap_cohort_ratio_correction_bound_stability
                .map(|stability| stability.stable_bound_count),
        pair_family_gap_cohort_ratio_correction_bound_stability_target:
            pair_family_gap_cohort_ratio_correction_bound_stability
                .map(|stability| stability.next_experiment_target.clone()),
        pair_family_gap_cohort_ratio_geometry_atlas_decision:
            pair_family_gap_cohort_ratio_geometry_atlas
                .map(|atlas| atlas.atlas_decision.clone()),
        pair_family_gap_cohort_ratio_geometry_atlas_status:
            pair_family_gap_cohort_ratio_geometry_atlas
                .map(|atlas| atlas.atlas_status.clone()),
        pair_family_gap_cohort_ratio_geometry_atlas_target:
            pair_family_gap_cohort_ratio_geometry_atlas
                .map(|atlas| atlas.next_experiment_target.clone()),
        pair_family_gap_cohort_ratio_geometry_picker_decision:
            pair_family_gap_cohort_ratio_geometry_picker
                .map(|picker| picker.picker_decision.clone()),
        pair_family_gap_cohort_ratio_geometry_picker_stable_candidate_count:
            pair_family_gap_cohort_ratio_geometry_picker
                .map(|picker| picker.stable_candidate_count),
        pair_family_gap_cohort_ratio_geometry_picker_selected_connector:
            pair_family_gap_cohort_ratio_geometry_picker
                .and_then(|picker| picker.selected_candidate.as_ref())
                .map(|candidate| candidate.connector.clone()),
        pair_family_gap_cohort_ratio_geometry_picker_selected_direction:
            pair_family_gap_cohort_ratio_geometry_picker
                .and_then(|picker| picker.selected_candidate.as_ref())
                .map(|candidate| candidate.shared_stable_direction.clone()),
        pair_family_gap_cohort_ratio_geometry_picker_target:
            pair_family_gap_cohort_ratio_geometry_picker
                .map(|picker| picker.next_experiment_target.clone()),
        pair_family_gap_cohort_ratio_geometry_residue_profile_decision:
            pair_family_gap_cohort_ratio_geometry_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        pair_family_gap_cohort_ratio_geometry_residue_profile_status:
            pair_family_gap_cohort_ratio_geometry_residue_profile
                .map(|profile| profile.profile_status.clone()),
        pair_family_gap_cohort_ratio_geometry_residue_profile_best_modulus:
            pair_family_gap_cohort_ratio_geometry_residue_profile
                .and_then(|profile| profile.best_coherent_separator.as_ref())
                .map(|separator| separator.modulus),
        pair_family_gap_cohort_ratio_geometry_residue_profile_target:
            pair_family_gap_cohort_ratio_geometry_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        pair_family_gap_cohort_ratio_geometry_next_picker_decision:
            pair_family_gap_cohort_ratio_geometry_next_picker
                .map(|picker| picker.picker_decision.clone()),
        pair_family_gap_cohort_ratio_geometry_next_picker_excluded_profile_count:
            pair_family_gap_cohort_ratio_geometry_next_picker
                .map(|picker| picker.excluded_profile_count),
        pair_family_gap_cohort_ratio_geometry_next_picker_selected_connector:
            pair_family_gap_cohort_ratio_geometry_next_picker
                .and_then(|picker| picker.selected_candidate.as_ref())
                .map(|candidate| candidate.connector.clone()),
        pair_family_gap_cohort_ratio_geometry_next_picker_selected_direction:
            pair_family_gap_cohort_ratio_geometry_next_picker
                .and_then(|picker| picker.selected_candidate.as_ref())
                .map(|candidate| candidate.shared_stable_direction.clone()),
        pair_family_gap_cohort_ratio_geometry_next_picker_target:
            pair_family_gap_cohort_ratio_geometry_next_picker
                .map(|picker| picker.next_experiment_target.clone()),
        pair_family_gap_cohort_ratio_geometry_next_residue_profile_decision:
            pair_family_gap_cohort_ratio_geometry_next_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        pair_family_gap_cohort_ratio_geometry_next_residue_profile_status:
            pair_family_gap_cohort_ratio_geometry_next_residue_profile
                .map(|profile| profile.profile_status.clone()),
        pair_family_gap_cohort_ratio_geometry_next_residue_profile_best_modulus:
            pair_family_gap_cohort_ratio_geometry_next_residue_profile
                .and_then(|profile| profile.best_coherent_separator.as_ref())
                .map(|separator| separator.modulus),
        pair_family_gap_cohort_ratio_geometry_next_residue_profile_target:
            pair_family_gap_cohort_ratio_geometry_next_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        pair_family_gap_cohort_ratio_geometry_post_two_null_picker_decision:
            pair_family_gap_cohort_ratio_geometry_post_two_null_picker
                .map(|picker| picker.picker_decision.clone()),
        pair_family_gap_cohort_ratio_geometry_post_two_null_picker_excluded_profile_count:
            pair_family_gap_cohort_ratio_geometry_post_two_null_picker
                .map(|picker| picker.excluded_profile_count),
        pair_family_gap_cohort_ratio_geometry_post_two_null_picker_selected_connector:
            pair_family_gap_cohort_ratio_geometry_post_two_null_picker
                .and_then(|picker| picker.selected_candidate.as_ref())
                .map(|candidate| candidate.connector.clone()),
        pair_family_gap_cohort_ratio_geometry_post_two_null_picker_selected_direction:
            pair_family_gap_cohort_ratio_geometry_post_two_null_picker
                .and_then(|picker| picker.selected_candidate.as_ref())
                .map(|candidate| candidate.shared_stable_direction.clone()),
        pair_family_gap_cohort_ratio_geometry_post_two_null_picker_target:
            pair_family_gap_cohort_ratio_geometry_post_two_null_picker
                .map(|picker| picker.next_experiment_target.clone()),
        pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile_decision:
            pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile_status:
            pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile
                .map(|profile| profile.profile_status.clone()),
        pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile_best_modulus:
            pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile
                .and_then(|profile| profile.best_coherent_separator.as_ref())
                .map(|separator| separator.modulus),
        pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile_target:
            pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        pair_family_gap_cohort_ratio_geometry_three_null_conclusion_decision:
            pair_family_gap_cohort_ratio_geometry_three_null_conclusion
                .map(|conclusion| conclusion.conclusion_decision.clone()),
        pair_family_gap_cohort_ratio_geometry_three_null_conclusion_status:
            pair_family_gap_cohort_ratio_geometry_three_null_conclusion
                .map(|conclusion| conclusion.conclusion_status.clone()),
        pair_family_gap_cohort_ratio_geometry_three_null_conclusion_collapsed_profile_count:
            pair_family_gap_cohort_ratio_geometry_three_null_conclusion
                .map(|conclusion| conclusion.collapsed_profile_count),
        pair_family_gap_cohort_ratio_geometry_three_null_conclusion_selected_connector:
            pair_family_gap_cohort_ratio_geometry_three_null_conclusion
                .and_then(|conclusion| conclusion.selected_forward_candidate.as_ref())
                .map(|candidate| candidate.connector.clone()),
        pair_family_gap_cohort_ratio_geometry_three_null_conclusion_selected_direction:
            pair_family_gap_cohort_ratio_geometry_three_null_conclusion
                .and_then(|conclusion| conclusion.selected_forward_candidate.as_ref())
                .map(|candidate| candidate.shared_stable_direction.clone()),
        pair_family_gap_cohort_ratio_geometry_three_null_conclusion_target:
            pair_family_gap_cohort_ratio_geometry_three_null_conclusion
                .map(|conclusion| conclusion.next_experiment_target.clone()),
        pair_family_gap_cohort_ratio_geometry_forward_residue_profile_decision:
            pair_family_gap_cohort_ratio_geometry_forward_residue_profile
                .map(|profile| profile.profile_decision.clone()),
        pair_family_gap_cohort_ratio_geometry_forward_residue_profile_status:
            pair_family_gap_cohort_ratio_geometry_forward_residue_profile
                .map(|profile| profile.profile_status.clone()),
        pair_family_gap_cohort_ratio_geometry_forward_residue_profile_best_modulus:
            pair_family_gap_cohort_ratio_geometry_forward_residue_profile
                .and_then(|profile| profile.best_coherent_separator.as_ref())
                .map(|separator| separator.modulus),
        pair_family_gap_cohort_ratio_geometry_forward_residue_profile_target:
            pair_family_gap_cohort_ratio_geometry_forward_residue_profile
                .map(|profile| profile.next_experiment_target.clone()),
        pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_decision:
            pair_family_gap_cohort_ratio_geometry_forward_null_conclusion
                .map(|conclusion| conclusion.conclusion_decision.clone()),
        pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_status:
            pair_family_gap_cohort_ratio_geometry_forward_null_conclusion
                .map(|conclusion| conclusion.conclusion_status.clone()),
        pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_collapsed_profile_count:
            pair_family_gap_cohort_ratio_geometry_forward_null_conclusion
                .map(|conclusion| conclusion.collapsed_profile_count),
        pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_remaining_candidate_count:
            pair_family_gap_cohort_ratio_geometry_forward_null_conclusion
                .map(|conclusion| conclusion.remaining_stable_candidate_count),
        pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_target:
            pair_family_gap_cohort_ratio_geometry_forward_null_conclusion
                .map(|conclusion| conclusion.next_experiment_target.clone()),
        pair_family_gap_cohort_window_consensus_surface_decision:
            pair_family_gap_cohort_window_consensus_surface
                .map(|surface| surface.surface_decision.clone()),
        pair_family_gap_cohort_window_consensus_surface_candidate_count:
            pair_family_gap_cohort_window_consensus_surface.map(|surface| surface.candidate_count),
        pair_family_gap_cohort_window_consensus_surface_selected_connector:
            pair_family_gap_cohort_window_consensus_surface
                .and_then(|surface| surface.selected_candidate.as_ref())
                .map(|candidate| candidate.connector.clone()),
        pair_family_gap_cohort_window_consensus_surface_selected_direction:
            pair_family_gap_cohort_window_consensus_surface
                .and_then(|surface| surface.selected_candidate.as_ref())
                .map(|candidate| candidate.shared_consensus_direction.clone()),
        pair_family_gap_cohort_window_consensus_surface_selected_status:
            pair_family_gap_cohort_window_consensus_surface
                .and_then(|surface| surface.selected_candidate.as_ref())
                .map(|candidate| candidate.candidate_status.clone()),
        pair_family_gap_cohort_window_consensus_surface_selected_consensus_window_count:
            pair_family_gap_cohort_window_consensus_surface
                .and_then(|surface| surface.selected_candidate.as_ref())
                .map(|candidate| candidate.consensus_window_count),
        pair_family_gap_cohort_window_consensus_surface_target:
            pair_family_gap_cohort_window_consensus_surface
                .map(|surface| surface.next_experiment_target.clone()),
        pair_family_gap_cohort_window_consensus_stress_status:
            pair_family_gap_cohort_window_consensus_stress
                .map(|stress| stress.stress_status.clone()),
        pair_family_gap_cohort_window_consensus_stress_decision:
            pair_family_gap_cohort_window_consensus_stress
                .map(|stress| stress.stress_decision.clone()),
        pair_family_gap_cohort_window_consensus_stress_retained_surface_count:
            pair_family_gap_cohort_window_consensus_stress
                .map(|stress| stress.retained_surface_count),
        pair_family_gap_cohort_window_consensus_stress_split_surface_count:
            pair_family_gap_cohort_window_consensus_stress
                .map(|stress| stress.split_surface_count),
        pair_family_gap_cohort_window_consensus_stress_collapsed_surface_count:
            pair_family_gap_cohort_window_consensus_stress
                .map(|stress| stress.collapsed_surface_count),
        pair_family_gap_cohort_window_consensus_stress_target:
            pair_family_gap_cohort_window_consensus_stress
                .map(|stress| stress.next_experiment_target.clone()),
        pair_family_gap_cohort_sign_persistence_picker_decision:
            pair_family_gap_cohort_sign_persistence_picker
                .map(|picker| picker.picker_decision.clone()),
        pair_family_gap_cohort_sign_persistence_picker_candidate_count:
            pair_family_gap_cohort_sign_persistence_picker.map(|picker| picker.candidate_count),
        pair_family_gap_cohort_sign_persistence_picker_persistent_candidate_count:
            pair_family_gap_cohort_sign_persistence_picker
                .map(|picker| picker.persistent_candidate_count),
        pair_family_gap_cohort_sign_persistence_picker_selected_connector:
            pair_family_gap_cohort_sign_persistence_picker
                .and_then(|picker| picker.selected_candidate.as_ref())
                .map(|candidate| candidate.connector.clone()),
        pair_family_gap_cohort_sign_persistence_picker_selected_direction:
            pair_family_gap_cohort_sign_persistence_picker
                .and_then(|picker| picker.selected_candidate.as_ref())
                .map(|candidate| candidate.persistent_direction.clone()),
        pair_family_gap_cohort_sign_persistence_picker_selected_status:
            pair_family_gap_cohort_sign_persistence_picker
                .and_then(|picker| picker.selected_candidate.as_ref())
                .map(|candidate| candidate.candidate_status.clone()),
        pair_family_gap_cohort_sign_persistence_picker_selected_surface_count:
            pair_family_gap_cohort_sign_persistence_picker
                .and_then(|picker| picker.selected_candidate.as_ref())
                .map(|candidate| candidate.persistent_surface_count),
        pair_family_gap_cohort_sign_persistence_picker_selected_volatility_score:
            pair_family_gap_cohort_sign_persistence_picker
                .and_then(|picker| picker.selected_candidate.as_ref())
                .map(|candidate| candidate.volatility_score),
        pair_family_gap_cohort_sign_persistence_picker_target:
            pair_family_gap_cohort_sign_persistence_picker
                .map(|picker| picker.next_experiment_target.clone()),
        pair_family_gap_cohort_sign_persistence_stress_status:
            pair_family_gap_cohort_sign_persistence_stress
                .map(|stress| stress.stress_status.clone()),
        pair_family_gap_cohort_sign_persistence_stress_decision:
            pair_family_gap_cohort_sign_persistence_stress
                .map(|stress| stress.stress_decision.clone()),
        pair_family_gap_cohort_sign_persistence_stress_retained_surface_count:
            pair_family_gap_cohort_sign_persistence_stress
                .map(|stress| stress.retained_surface_count),
        pair_family_gap_cohort_sign_persistence_stress_split_surface_count:
            pair_family_gap_cohort_sign_persistence_stress
                .map(|stress| stress.split_surface_count),
        pair_family_gap_cohort_sign_persistence_stress_neutral_surface_count:
            pair_family_gap_cohort_sign_persistence_stress
                .map(|stress| stress.neutral_surface_count),
        pair_family_gap_cohort_sign_persistence_stress_retained_window_count:
            pair_family_gap_cohort_sign_persistence_stress
                .map(|stress| stress.retained_window_count),
        pair_family_gap_cohort_sign_persistence_stress_opposite_window_count:
            pair_family_gap_cohort_sign_persistence_stress
                .map(|stress| stress.opposite_window_count),
        pair_family_gap_cohort_sign_persistence_stress_target:
            pair_family_gap_cohort_sign_persistence_stress
                .map(|stress| stress.next_experiment_target.clone()),
        pair_family_gap_cohort_volatility_ensemble_picker_decision:
            pair_family_gap_cohort_volatility_ensemble_picker
                .map(|picker| picker.picker_decision.clone()),
        pair_family_gap_cohort_volatility_ensemble_picker_ensemble_count:
            pair_family_gap_cohort_volatility_ensemble_picker
                .map(|picker| picker.ensemble_count),
        pair_family_gap_cohort_volatility_ensemble_picker_qualifying_ensemble_count:
            pair_family_gap_cohort_volatility_ensemble_picker
                .map(|picker| picker.qualifying_ensemble_count),
        pair_family_gap_cohort_volatility_ensemble_picker_selected_direction:
            pair_family_gap_cohort_volatility_ensemble_picker
                .and_then(|picker| picker.selected_ensemble.as_ref())
                .map(|ensemble| ensemble.direction.clone()),
        pair_family_gap_cohort_volatility_ensemble_picker_selected_connector_count:
            pair_family_gap_cohort_volatility_ensemble_picker
                .and_then(|picker| picker.selected_ensemble.as_ref())
                .map(|ensemble| ensemble.connector_count),
        pair_family_gap_cohort_volatility_ensemble_picker_selected_supported_surface_count:
            pair_family_gap_cohort_volatility_ensemble_picker
                .and_then(|picker| picker.selected_ensemble.as_ref())
                .map(|ensemble| ensemble.supported_surface_count),
        pair_family_gap_cohort_volatility_ensemble_picker_target:
            pair_family_gap_cohort_volatility_ensemble_picker
                .map(|picker| picker.next_experiment_target.clone()),
        pair_family_gap_cohort_volatility_ensemble_stress_status:
            pair_family_gap_cohort_volatility_ensemble_stress
                .map(|stress| stress.stress_status.clone()),
        pair_family_gap_cohort_volatility_ensemble_stress_decision:
            pair_family_gap_cohort_volatility_ensemble_stress
                .map(|stress| stress.stress_decision.clone()),
        pair_family_gap_cohort_volatility_ensemble_stress_selected_direction:
            pair_family_gap_cohort_volatility_ensemble_stress
                .map(|stress| stress.source_direction.clone()),
        pair_family_gap_cohort_volatility_ensemble_stress_selected_connector_count:
            pair_family_gap_cohort_volatility_ensemble_stress
                .map(|stress| stress.source_connector_count),
        pair_family_gap_cohort_volatility_ensemble_stress_retained_surface_count:
            pair_family_gap_cohort_volatility_ensemble_stress
                .map(|stress| stress.retained_surface_count),
        pair_family_gap_cohort_volatility_ensemble_stress_mixed_retained_surface_count:
            pair_family_gap_cohort_volatility_ensemble_stress
                .map(|stress| stress.mixed_retained_surface_count),
        pair_family_gap_cohort_volatility_ensemble_stress_split_surface_count:
            pair_family_gap_cohort_volatility_ensemble_stress
                .map(|stress| stress.split_surface_count),
        pair_family_gap_cohort_volatility_ensemble_stress_collapsed_surface_count:
            pair_family_gap_cohort_volatility_ensemble_stress
                .map(|stress| stress.collapsed_surface_count),
        pair_family_gap_cohort_volatility_ensemble_stress_retained_window_count:
            pair_family_gap_cohort_volatility_ensemble_stress
                .map(|stress| stress.retained_window_total),
        pair_family_gap_cohort_volatility_ensemble_stress_opposite_window_count:
            pair_family_gap_cohort_volatility_ensemble_stress
                .map(|stress| stress.opposite_window_total),
        pair_family_gap_cohort_volatility_ensemble_stress_target:
            pair_family_gap_cohort_volatility_ensemble_stress
                .map(|stress| stress.next_experiment_target.clone()),
        pair_family_gap_cohort_surface_family_contrast_picker_decision:
            pair_family_gap_cohort_surface_family_contrast_picker
                .map(|picker| picker.picker_decision.clone()),
        pair_family_gap_cohort_surface_family_contrast_picker_status:
            pair_family_gap_cohort_surface_family_contrast_picker
                .map(|picker| picker.contrast_status.clone()),
        pair_family_gap_cohort_surface_family_contrast_picker_selected_family:
            pair_family_gap_cohort_surface_family_contrast_picker
                .and_then(|picker| picker.selected_family.clone()),
        pair_family_gap_cohort_surface_family_contrast_picker_opposite_family:
            pair_family_gap_cohort_surface_family_contrast_picker
                .and_then(|picker| picker.opposite_family.clone()),
        pair_family_gap_cohort_surface_family_contrast_picker_retained_family_count:
            pair_family_gap_cohort_surface_family_contrast_picker
                .map(|picker| picker.retained_family_count),
        pair_family_gap_cohort_surface_family_contrast_picker_split_family_count:
            pair_family_gap_cohort_surface_family_contrast_picker
                .map(|picker| picker.split_family_count),
        pair_family_gap_cohort_surface_family_contrast_picker_target:
            pair_family_gap_cohort_surface_family_contrast_picker
                .map(|picker| picker.next_experiment_target.clone()),
        pair_family_gap_cohort_surface_family_contrast_stress_status:
            pair_family_gap_cohort_surface_family_contrast_stress
                .map(|stress| stress.stress_status.clone()),
        pair_family_gap_cohort_surface_family_contrast_stress_decision:
            pair_family_gap_cohort_surface_family_contrast_stress
                .map(|stress| stress.stress_decision.clone()),
        pair_family_gap_cohort_surface_family_contrast_stress_selected_family:
            pair_family_gap_cohort_surface_family_contrast_stress
                .map(|stress| stress.source_selected_family.clone()),
        pair_family_gap_cohort_surface_family_contrast_stress_opposite_family:
            pair_family_gap_cohort_surface_family_contrast_stress
                .map(|stress| stress.source_opposite_family.clone()),
        pair_family_gap_cohort_surface_family_contrast_stress_retained_family_count:
            pair_family_gap_cohort_surface_family_contrast_stress
                .map(|stress| stress.retained_family_count),
        pair_family_gap_cohort_surface_family_contrast_stress_split_family_count:
            pair_family_gap_cohort_surface_family_contrast_stress
                .map(|stress| stress.split_family_count),
        pair_family_gap_cohort_surface_family_contrast_stress_retained_window_count:
            pair_family_gap_cohort_surface_family_contrast_stress
                .map(|stress| stress.retained_window_total),
        pair_family_gap_cohort_surface_family_contrast_stress_opposite_window_count:
            pair_family_gap_cohort_surface_family_contrast_stress
                .map(|stress| stress.opposite_window_total),
        pair_family_gap_cohort_surface_family_contrast_stress_target:
            pair_family_gap_cohort_surface_family_contrast_stress
                .map(|stress| stress.next_experiment_target.clone()),
        pair_family_gap_cohort_surface_family_contrast_anatomy_concentration_status:
            pair_family_gap_cohort_surface_family_contrast_anatomy
                .map(|anatomy| anatomy.concentration_status.clone()),
        pair_family_gap_cohort_surface_family_contrast_anatomy_decision:
            pair_family_gap_cohort_surface_family_contrast_anatomy
                .map(|anatomy| anatomy.anatomy_decision.clone()),
        pair_family_gap_cohort_surface_family_contrast_anatomy_full_driver_count:
            pair_family_gap_cohort_surface_family_contrast_anatomy
                .map(|anatomy| anatomy.full_contrast_driver_count),
        pair_family_gap_cohort_surface_family_contrast_anatomy_gap_only_driver_count:
            pair_family_gap_cohort_surface_family_contrast_anatomy
                .map(|anatomy| anatomy.gap_only_driver_count),
        pair_family_gap_cohort_surface_family_contrast_anatomy_size_only_driver_count:
            pair_family_gap_cohort_surface_family_contrast_anatomy
                .map(|anatomy| anatomy.size_only_driver_count),
        pair_family_gap_cohort_surface_family_contrast_anatomy_top_driver_share_basis_points:
            pair_family_gap_cohort_surface_family_contrast_anatomy
                .map(|anatomy| anatomy.top_driver_share_basis_points),
        pair_family_gap_cohort_surface_family_contrast_anatomy_target:
            pair_family_gap_cohort_surface_family_contrast_anatomy
                .map(|anatomy| anatomy.next_experiment_target.clone()),
        pair_family_gap_cohort_surface_family_driver_cohort_stress_status:
            pair_family_gap_cohort_surface_family_driver_cohort_stress
                .map(|stress| stress.stress_status.clone()),
        pair_family_gap_cohort_surface_family_driver_cohort_stress_decision:
            pair_family_gap_cohort_surface_family_driver_cohort_stress
                .map(|stress| stress.stress_decision.clone()),
        pair_family_gap_cohort_surface_family_driver_cohort_stress_driver_count:
            pair_family_gap_cohort_surface_family_driver_cohort_stress
                .map(|stress| stress.driver_count),
        pair_family_gap_cohort_surface_family_driver_cohort_stress_retained_family_count:
            pair_family_gap_cohort_surface_family_driver_cohort_stress
                .map(|stress| stress.retained_family_count),
        pair_family_gap_cohort_surface_family_driver_cohort_stress_split_family_count:
            pair_family_gap_cohort_surface_family_driver_cohort_stress
                .map(|stress| stress.split_family_count),
        pair_family_gap_cohort_surface_family_driver_cohort_stress_retained_window_count:
            pair_family_gap_cohort_surface_family_driver_cohort_stress
                .map(|stress| stress.retained_window_total),
        pair_family_gap_cohort_surface_family_driver_cohort_stress_opposite_window_count:
            pair_family_gap_cohort_surface_family_driver_cohort_stress
                .map(|stress| stress.opposite_window_total),
        pair_family_gap_cohort_surface_family_driver_cohort_stress_target:
            pair_family_gap_cohort_surface_family_driver_cohort_stress
                .map(|stress| stress.next_experiment_target.clone()),
        pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_status:
            pair_family_gap_cohort_surface_family_matched_nondriver_control_stress
                .map(|stress| stress.control_status.clone()),
        pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_decision:
            pair_family_gap_cohort_surface_family_matched_nondriver_control_stress
                .map(|stress| stress.control_decision.clone()),
        pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_control_count:
            pair_family_gap_cohort_surface_family_matched_nondriver_control_stress
                .map(|stress| stress.control_count),
        pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_retained_family_count:
            pair_family_gap_cohort_surface_family_matched_nondriver_control_stress
                .map(|stress| stress.retained_family_count),
        pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_split_family_count:
            pair_family_gap_cohort_surface_family_matched_nondriver_control_stress
                .map(|stress| stress.split_family_count),
        pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_retained_window_count:
            pair_family_gap_cohort_surface_family_matched_nondriver_control_stress
                .map(|stress| stress.retained_window_total),
        pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_opposite_window_count:
            pair_family_gap_cohort_surface_family_matched_nondriver_control_stress
                .map(|stress| stress.opposite_window_total),
        pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_target:
            pair_family_gap_cohort_surface_family_matched_nondriver_control_stress
                .map(|stress| stress.next_experiment_target.clone()),
        pair_family_gap_cohort_surface_agnostic_ensemble_picker_decision:
            pair_family_gap_cohort_surface_agnostic_ensemble_picker
                .map(|picker| picker.picker_decision.clone()),
        pair_family_gap_cohort_surface_agnostic_ensemble_picker_candidate_count:
            pair_family_gap_cohort_surface_agnostic_ensemble_picker
                .map(|picker| picker.candidate_count),
        pair_family_gap_cohort_surface_agnostic_ensemble_picker_stable_connector_count:
            pair_family_gap_cohort_surface_agnostic_ensemble_picker
                .map(|picker| picker.stable_connector_count),
        pair_family_gap_cohort_surface_agnostic_ensemble_picker_selected_direction:
            pair_family_gap_cohort_surface_agnostic_ensemble_picker
                .and_then(|picker| picker.selected_ensemble.as_ref())
                .map(|ensemble| ensemble.direction.clone()),
        pair_family_gap_cohort_surface_agnostic_ensemble_picker_selected_connector_count:
            pair_family_gap_cohort_surface_agnostic_ensemble_picker
                .and_then(|picker| picker.selected_ensemble.as_ref())
                .map(|ensemble| ensemble.connector_count),
        pair_family_gap_cohort_surface_agnostic_ensemble_picker_selected_supported_surface_count:
            pair_family_gap_cohort_surface_agnostic_ensemble_picker
                .and_then(|picker| picker.selected_ensemble.as_ref())
                .map(|ensemble| ensemble.stable_surface_total),
        pair_family_gap_cohort_surface_agnostic_ensemble_picker_retained_window_count:
            pair_family_gap_cohort_surface_agnostic_ensemble_picker
                .and_then(|picker| picker.selected_ensemble.as_ref())
                .map(|ensemble| ensemble.retained_window_total),
        pair_family_gap_cohort_surface_agnostic_ensemble_picker_opposite_window_count:
            pair_family_gap_cohort_surface_agnostic_ensemble_picker
                .and_then(|picker| picker.selected_ensemble.as_ref())
                .map(|ensemble| ensemble.opposite_window_total),
        pair_family_gap_cohort_surface_agnostic_ensemble_picker_target:
            pair_family_gap_cohort_surface_agnostic_ensemble_picker
                .map(|picker| picker.next_experiment_target.clone()),
        proof_status:
            "theorem-backed finite multi-modulus classifiers for the maintained digit-8 stress cells"
                .to_string(),
        claim_status:
            "bounded stress-artifact classifier family; not a connector law or density mechanism"
                .to_string(),
        cells,
    }
}

pub fn render_signal_catalog_markdown(catalog: &SignalCatalog) -> String {
    let mut out = String::new();
    out.push_str("# Signal Catalog\n\n");
    out.push_str("This index ties the maintained matched-control, witness, and connector atlases into one research-instrument surface. It is a catalog of evidence, proof links, and drift gates; it is not a combined density claim.\n\n");
    out.push_str(&format!("- Schema: `{}`\n", catalog.schema_version));
    out.push_str(&format!("- Artifact: `{}`\n", catalog.artifact_id));
    out.push_str(&format!(
        "- Rows: {} total, {} proof-carrying\n\n",
        catalog.summary.row_count, catalog.summary.proof_carrying_rows
    ));

    out.push_str("| Signal | Domain | Artifact | Claim status | Proof status | Drift gate |\n");
    out.push_str("|---|---|---|---|---|---|\n");
    for row in &catalog.rows {
        out.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} | {} | `{}` |\n",
            row.signal_id,
            row.domain,
            row.artifact_path,
            row.claim_status,
            row.proof_status,
            row.drift_check_command
        ));
    }
    out.push('\n');

    let classifier = &catalog.connector_digit8_classifier_family;
    out.push_str("## Connector Digit-8 Classifier Family\n\n");
    out.push_str("This block summarizes the bounded digit-8 edge-cell stress surface from the connector width-6 stress artifact. It is a finite classifier family over the tracked stress screen, not a connector law or prime-density mechanism.\n\n");
    out.push_str(&format!(
        "- Source artifact: `{}`\n",
        classifier.source_artifact_path
    ));
    out.push_str(&format!(
        "- Source schema: `{}`\n",
        classifier.source_schema_version
    ));
    out.push_str(&format!(
        "- Surface status: `{}`\n",
        classifier.surface_status
    ));
    out.push_str(&format!(
        "- Theorem-backed multi-modulus cells: `{}`\n",
        classifier.theorem_backed_multi_modulus_cell_count
    ));
    out.push_str(&format!(
        "- Unclassified exact separators: `{}`\n\n",
        classifier.unclassified_exact_separator_count
    ));
    out.push_str(&format!(
        "- Outside-ladder replication: `{}`\n",
        classifier.outside_ladder_replication_decision
    ));
    out.push_str(&format!(
        "- Outside-ladder pairs: baseline `{}`, widened `{}`, added `{}`\n",
        classifier.outside_ladder_baseline_pair_count,
        classifier.outside_ladder_widened_pair_count,
        classifier.outside_ladder_added_pair_count
    ));
    out.push_str(&format!(
        "- Outside-ladder cells retained/split/collapsed: `{}` / `{}` / `{}`\n\n",
        classifier.outside_ladder_retained_cell_count,
        classifier.outside_ladder_split_cell_count,
        classifier.outside_ladder_collapsed_cell_count
    ));
    out.push_str(&format!(
        "- Split follow-up: `{}`\n",
        classifier.split_follow_up_decision
    ));
    out.push_str(&format!(
        "- Split follow-up rows stabilized/split-again/collapsed: `{}` / `{}` / `{}`\n\n",
        classifier.split_follow_up_stabilized_row_count,
        classifier.split_follow_up_split_again_row_count,
        classifier.split_follow_up_collapsed_row_count
    ));
    out.push_str(&format!(
        "- Branch picker: `{}`\n",
        classifier.branch_picker_decision
    ));
    out.push_str(&format!(
        "- Selected next branch: `{}` status `{}` target `{}`\n\n",
        classifier
            .selected_next_branch_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_next_branch_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_next_branch_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Independent branch replication: `{}` retired branch `{}` next target `{}`\n\n",
        classifier
            .selected_branch_independent_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier.retired_branch_id.as_deref().unwrap_or("none"),
        classifier
            .selected_branch_independent_replication_next_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Non-mod3 candidate picker: `{}` selected `{}` target `{}`\n\n",
        classifier
            .non_mod3_candidate_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_non_mod3_candidate_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_non_mod3_candidate_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Retired non-mod3 candidates: `{}` `{:?}`\n\n",
        classifier.retired_non_mod3_candidate_count, classifier.retired_non_mod3_candidate_ids
    ));
    out.push_str(&format!(
        "- Non-mod3 second replication: `{}` target `{}`\n\n",
        classifier
            .non_mod3_second_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .non_mod3_second_replication_next_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Non-mod3 residue profile: `{}` best modulus `{}` target `{}`\n\n",
        classifier
            .non_mod3_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .non_mod3_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .non_mod3_residue_profile_next_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Non-mod3 residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .non_mod3_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .non_mod3_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .non_mod3_residue_separator_replication_next_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Non-mod3 mutated residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .non_mod3_mutated_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .non_mod3_mutated_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .non_mod3_mutated_residue_separator_replication_next_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Next non-mod3 candidate picker: `{}` selected `{}` target `{}`\n\n",
        classifier
            .next_non_mod3_candidate_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_next_non_mod3_candidate_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_next_non_mod3_candidate_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Next non-mod3 independent replication: `{}` target `{}`\n\n",
        classifier
            .next_non_mod3_independent_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .next_non_mod3_independent_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Retired non-mod3 edge candidates: `{}` `{:?}`\n",
        classifier.non_mod3_retired_edge_candidate_count,
        classifier.non_mod3_retired_edge_candidate_ids
    ));
    out.push_str(&format!(
        "- Interior non-mod3 family picker: `{}` selected `{}` target `{}`\n",
        classifier
            .interior_non_mod3_family_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_interior_non_mod3_candidate_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_interior_non_mod3_candidate_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 independent replication: `{}` target `{}`\n\n",
        classifier
            .interior_non_mod3_family_independent_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_family_independent_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 residue profile: `{}` best modulus `{}` target `{}`\n\n",
        classifier
            .interior_non_mod3_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .interior_non_mod3_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .interior_non_mod3_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Retired interior non-mod3 candidates: `{}` `{:?}`\n",
        classifier.interior_non_mod3_retired_candidate_count,
        classifier.interior_non_mod3_retired_candidate_ids
    ));
    out.push_str(&format!(
        "- Interior non-mod3 next family picker: `{}` selected `{}` target `{}`\n",
        classifier
            .interior_non_mod3_next_family_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_next_interior_non_mod3_candidate_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_next_interior_non_mod3_candidate_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 next independent replication: `{}` target `{}`\n\n",
        classifier
            .interior_non_mod3_next_family_independent_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_next_family_independent_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 next residue profile: `{}` best modulus `{}` target `{}`\n\n",
        classifier
            .interior_non_mod3_next_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_next_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .interior_non_mod3_next_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 next residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .interior_non_mod3_next_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_next_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_next_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 post-retirement family picker: `{}` selected `{}` target `{}`\n",
        classifier
            .interior_non_mod3_post_retirement_family_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_post_retirement_interior_non_mod3_candidate_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_post_retirement_interior_non_mod3_candidate_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 post-retirement independent replication: `{}` target `{}`\n\n",
        classifier
            .interior_non_mod3_post_retirement_family_independent_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_post_retirement_family_independent_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 post-retirement residue profile: `{}` best modulus `{}` target `{}`\n\n",
        classifier
            .interior_non_mod3_post_retirement_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_post_retirement_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .interior_non_mod3_post_retirement_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 post-retirement residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .interior_non_mod3_post_retirement_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_post_retirement_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_post_retirement_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 after third-retirement family picker: `{}` selected `{}` target `{}`\n",
        classifier
            .interior_non_mod3_after_third_retirement_family_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_after_third_retirement_interior_non_mod3_candidate_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_after_third_retirement_interior_non_mod3_candidate_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 after third-retirement independent replication: `{}` target `{}`\n\n",
        classifier
            .interior_non_mod3_after_third_retirement_family_independent_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_after_third_retirement_family_independent_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 after third-retirement residue profile: `{}` best modulus `{}` target `{}`\n\n",
        classifier
            .interior_non_mod3_after_third_retirement_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_after_third_retirement_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .interior_non_mod3_after_third_retirement_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 after third-retirement residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .interior_non_mod3_after_third_retirement_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_after_third_retirement_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_after_third_retirement_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 after fourth-retirement family picker: `{}` selected `{}` target `{}`\n",
        classifier
            .interior_non_mod3_after_fourth_retirement_family_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_after_fourth_retirement_interior_non_mod3_candidate_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_after_fourth_retirement_interior_non_mod3_candidate_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 after fourth-retirement independent replication: `{}` target `{}`\n\n",
        classifier
            .interior_non_mod3_after_fourth_retirement_family_independent_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_after_fourth_retirement_family_independent_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 after fourth-retirement residue profile: `{}` best modulus `{}` target `{}`\n\n",
        classifier
            .interior_non_mod3_after_fourth_retirement_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_after_fourth_retirement_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .interior_non_mod3_after_fourth_retirement_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 after fourth-retirement residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .interior_non_mod3_after_fourth_retirement_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_after_fourth_retirement_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_after_fourth_retirement_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 after fifth-retirement family picker: `{}` selected `{}` target `{}`\n",
        classifier
            .interior_non_mod3_after_fifth_retirement_family_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_after_fifth_retirement_interior_non_mod3_candidate_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_after_fifth_retirement_interior_non_mod3_candidate_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 after fifth-retirement independent replication: `{}` target `{}`\n\n",
        classifier
            .interior_non_mod3_after_fifth_retirement_family_independent_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_after_fifth_retirement_family_independent_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 after sixth-retirement family picker: `{}` selected `{}` target `{}`\n",
        classifier
            .interior_non_mod3_after_sixth_retirement_family_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_after_sixth_retirement_interior_non_mod3_candidate_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_after_sixth_retirement_interior_non_mod3_candidate_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Interior non-mod3 after sixth-retirement independent replication: `{}` target `{}`\n\n",
        classifier
            .interior_non_mod3_after_sixth_retirement_family_independent_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .interior_non_mod3_after_sixth_retirement_family_independent_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Single-digit interior pivot decision: `{}`\n",
        classifier
            .single_digit_interior_pivot_decision
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Multi-digit motif picker: `{}` selected `{}` target `{}`\n",
        classifier
            .multi_digit_motif_family_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_multi_digit_motif_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_multi_digit_motif_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Multi-digit motif independent replication: `{}` target `{}`\n\n",
        classifier
            .multi_digit_motif_family_independent_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .multi_digit_motif_family_independent_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Multi-digit motif residue profile: `{}` best modulus `{}` target `{}`\n",
        classifier
            .multi_digit_motif_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .multi_digit_motif_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .multi_digit_motif_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Multi-digit motif residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .multi_digit_motif_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .multi_digit_motif_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .multi_digit_motif_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Retired multi-digit motifs: `{}` ids `{:?}`\n",
        classifier.multi_digit_motif_retired_count, classifier.retired_multi_digit_motif_ids
    ));
    out.push_str(&format!(
        "- Retired orthogonal branches: `{}` ids `{:?}`\n",
        classifier.orthogonal_pair_family_retired_count,
        classifier.retired_orthogonal_pair_family_branch_ids
    ));
    out.push_str(&format!(
        "- Orthogonal pair-family control: `{}` selected `{}` family `{}` connector `{}` target `{}`\n\n",
        classifier
            .orthogonal_pair_family_control_matrix_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_pair_family_branch_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_pair_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_pair_family_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_pair_family_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal residue profile: `{}` best modulus `{}` target `{}`\n",
        classifier
            .orthogonal_pair_family_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_pair_family_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .orthogonal_pair_family_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .orthogonal_pair_family_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_pair_family_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_pair_family_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal compact three-digit control: `{}` selected `{}` family `{}` connector `{}` target `{}`\n",
        classifier
            .orthogonal_compact_three_digit_control_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_compact_three_digit_branch_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_compact_three_digit_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_compact_three_digit_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_compact_three_digit_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal compact three-digit residue profile: `{}` best modulus `{}` target `{}`\n",
        classifier
            .orthogonal_compact_three_digit_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_compact_three_digit_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .orthogonal_compact_three_digit_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal compact three-digit residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .orthogonal_compact_three_digit_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_compact_three_digit_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_compact_three_digit_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal non-adjacent two-digit control: `{}` selected `{}` family `{}` connector `{}` target `{}`\n",
        classifier
            .orthogonal_nonadjacent_two_digit_control_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_nonadjacent_two_digit_branch_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_nonadjacent_two_digit_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_nonadjacent_two_digit_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_nonadjacent_two_digit_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal non-adjacent two-digit residue profile: `{}` best modulus `{}` target `{}`\n",
        classifier
            .orthogonal_nonadjacent_two_digit_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_nonadjacent_two_digit_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .orthogonal_nonadjacent_two_digit_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal non-adjacent two-digit residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .orthogonal_nonadjacent_two_digit_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_nonadjacent_two_digit_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_nonadjacent_two_digit_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal edge-plus-interior control: `{}` selected `{}` family `{}` connector `{}` target `{}`\n",
        classifier
            .orthogonal_edge_plus_interior_control_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_edge_plus_interior_branch_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_edge_plus_interior_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_edge_plus_interior_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_edge_plus_interior_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal edge-plus-interior residue profile: `{}` best modulus `{}` target `{}`\n",
        classifier
            .orthogonal_edge_plus_interior_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_edge_plus_interior_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .orthogonal_edge_plus_interior_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal edge-plus-interior residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .orthogonal_edge_plus_interior_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_edge_plus_interior_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_edge_plus_interior_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal repeated-block control: `{}` selected `{}` family `{}` connector `{}` target `{}`\n",
        classifier
            .orthogonal_repeated_block_control_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_repeated_block_branch_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_repeated_block_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_repeated_block_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_repeated_block_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal repeated-block residue profile: `{}` best modulus `{}` target `{}`\n",
        classifier
            .orthogonal_repeated_block_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_repeated_block_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .orthogonal_repeated_block_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal repeated-block residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .orthogonal_repeated_block_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_repeated_block_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_repeated_block_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal arithmetic connector control: `{}` selected `{}` family `{}` connector `{}` target `{}`\n",
        classifier
            .orthogonal_arithmetic_connector_control_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_arithmetic_connector_branch_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_arithmetic_connector_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_arithmetic_connector_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_arithmetic_connector_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal arithmetic connector residue profile: `{}` best modulus `{}` target `{}`\n",
        classifier
            .orthogonal_arithmetic_connector_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_arithmetic_connector_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .orthogonal_arithmetic_connector_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal arithmetic connector residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .orthogonal_arithmetic_connector_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_arithmetic_connector_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_arithmetic_connector_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal residue-lattice connector control: `{}` selected `{}` family `{}` connector `{}` target `{}`\n",
        classifier
            .orthogonal_residue_lattice_connector_control_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_residue_lattice_connector_branch_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_residue_lattice_connector_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_residue_lattice_connector_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_residue_lattice_connector_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal residue-lattice connector residue profile: `{}` best modulus `{}` target `{}`\n",
        classifier
            .orthogonal_residue_lattice_connector_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_residue_lattice_connector_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .orthogonal_residue_lattice_connector_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal residue-lattice connector residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .orthogonal_residue_lattice_connector_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_residue_lattice_connector_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_residue_lattice_connector_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal modular-walk connector control: `{}` selected `{}` family `{}` connector `{}` target `{}`\n",
        classifier
            .orthogonal_modular_walk_connector_control_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_modular_walk_connector_branch_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_modular_walk_connector_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_modular_walk_connector_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_modular_walk_connector_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal modular-walk connector residue profile: `{}` best modulus `{}` target `{}`\n",
        classifier
            .orthogonal_modular_walk_connector_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_modular_walk_connector_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .orthogonal_modular_walk_connector_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal modular-walk connector residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .orthogonal_modular_walk_connector_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_modular_walk_connector_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_modular_walk_connector_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal arithmetic-family registry: `{}` retired `{}` selected `{}` target `{}`\n",
        classifier
            .orthogonal_arithmetic_family_registry_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_arithmetic_family_registry_retired_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .orthogonal_arithmetic_family_registry_selected_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_arithmetic_family_registry_selected_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal CRT-paired connector control: `{}` selected `{}` family `{}` connector `{}` target `{}`\n",
        classifier
            .orthogonal_crt_paired_connector_control_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_crt_paired_connector_branch_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_crt_paired_connector_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_crt_paired_connector_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_crt_paired_connector_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal CRT-paired connector residue profile: `{}` best modulus `{}` target `{}`\n",
        classifier
            .orthogonal_crt_paired_connector_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_crt_paired_connector_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .orthogonal_crt_paired_connector_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal CRT-paired connector residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .orthogonal_crt_paired_connector_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_crt_paired_connector_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_crt_paired_connector_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal multiplicative-order connector control: `{}` selected `{}` family `{}` connector `{}` target `{}`\n",
        classifier
            .orthogonal_multiplicative_order_connector_control_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_multiplicative_order_connector_branch_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_multiplicative_order_connector_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_multiplicative_order_connector_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_multiplicative_order_connector_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal multiplicative-order connector residue profile: `{}` best modulus `{}` target `{}`\n",
        classifier
            .orthogonal_multiplicative_order_connector_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_multiplicative_order_connector_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .orthogonal_multiplicative_order_connector_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal multiplicative-order connector residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .orthogonal_multiplicative_order_connector_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_multiplicative_order_connector_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_multiplicative_order_connector_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal automorphic/repunit connector control: `{}` selected `{}` family `{}` connector `{}` target `{}`\n",
        classifier
            .orthogonal_automorphic_repunit_connector_control_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_automorphic_repunit_connector_branch_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_automorphic_repunit_connector_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_automorphic_repunit_connector_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_automorphic_repunit_connector_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal automorphic/repunit connector residue profile: `{}` best modulus `{}` target `{}`\n",
        classifier
            .orthogonal_automorphic_repunit_connector_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_automorphic_repunit_connector_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .orthogonal_automorphic_repunit_connector_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal automorphic/repunit connector residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .orthogonal_automorphic_repunit_connector_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_automorphic_repunit_connector_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_automorphic_repunit_connector_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal cyclic-reptend connector control: `{}` selected `{}` family `{}` connector `{}` target `{}`\n",
        classifier
            .orthogonal_cyclic_reptend_connector_control_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_cyclic_reptend_connector_branch_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_cyclic_reptend_connector_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_cyclic_reptend_connector_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_cyclic_reptend_connector_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal cyclic-reptend connector residue profile: `{}` best modulus `{}` target `{}`\n",
        classifier
            .orthogonal_cyclic_reptend_connector_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_cyclic_reptend_connector_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .orthogonal_cyclic_reptend_connector_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal cyclic-reptend connector residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .orthogonal_cyclic_reptend_connector_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_cyclic_reptend_connector_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_cyclic_reptend_connector_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal carry-chain connector control: `{}` selected `{}` family `{}` connector `{}` target `{}`\n",
        classifier
            .orthogonal_carry_chain_connector_control_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_carry_chain_connector_branch_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_carry_chain_connector_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_carry_chain_connector_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_carry_chain_connector_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal carry-chain connector residue profile: `{}` best modulus `{}` target `{}`\n",
        classifier
            .orthogonal_carry_chain_connector_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_carry_chain_connector_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .orthogonal_carry_chain_connector_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal carry-chain connector residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .orthogonal_carry_chain_connector_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_carry_chain_connector_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_carry_chain_connector_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal base-mixed connector control: `{}` selected `{}` family `{}` connector `{}` target `{}` control target `{}`\n",
        classifier
            .orthogonal_base_mixed_connector_control_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_base_mixed_connector_branch_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_base_mixed_connector_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_base_mixed_connector_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_orthogonal_base_mixed_connector_target
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_base_mixed_connector_control_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal base-mixed connector residue profile: `{}` best modulus `{}` target `{}`\n",
        classifier
            .orthogonal_base_mixed_connector_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_base_mixed_connector_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .orthogonal_base_mixed_connector_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Orthogonal base-mixed connector residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .orthogonal_base_mixed_connector_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_base_mixed_connector_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .orthogonal_base_mixed_connector_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Connector stress meta-atlas: `{}` retired `{}` selected surface `{}` target `{}`\n",
        classifier
            .connector_stress_meta_atlas_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .connector_stress_meta_atlas_retired_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .connector_stress_meta_atlas_selected_surface
            .as_deref()
            .unwrap_or("none"),
        classifier
            .connector_stress_meta_atlas_selected_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family gap portfolio control: `{}` selected `{}` family `{}` connector `{}` target `{}` control target `{}`\n",
        classifier
            .pair_family_gap_portfolio_control_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_pair_family_gap_portfolio_branch_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_pair_family_gap_portfolio_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_pair_family_gap_portfolio_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_pair_family_gap_portfolio_target
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_portfolio_control_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family gap portfolio residue profile: `{}` best modulus `{}` target `{}`\n",
        classifier
            .pair_family_gap_portfolio_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_portfolio_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_portfolio_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family gap portfolio residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_portfolio_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_portfolio_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_portfolio_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family gap extension control: `{}` selected `{}` family `{}` connector `{}` target `{}` control target `{}`\n",
        classifier
            .pair_family_gap_extension_control_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_pair_family_gap_extension_branch_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_pair_family_gap_extension_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_pair_family_gap_extension_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_pair_family_gap_extension_target
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_extension_control_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family gap extension residue profile: `{}` best modulus `{}` target `{}`\n",
        classifier
            .pair_family_gap_extension_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_extension_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_extension_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family gap extension residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_extension_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_extension_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_extension_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family size-band control: `{}` selected `{}` family `{}` connector `{}` target `{}` control target `{}`\n",
        classifier
            .pair_family_size_band_control_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_pair_family_size_band_branch_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_pair_family_size_band_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_pair_family_size_band_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_pair_family_size_band_target
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_size_band_control_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family size-band residue profile: `{}` best modulus `{}` target `{}`\n",
        classifier
            .pair_family_size_band_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_size_band_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_size_band_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family size-band residue-separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .pair_family_size_band_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_size_band_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_size_band_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Replication null atlas: schema `{}` status `{}` rows `{}` theorem candidates `{}` target `{}`\n",
        classifier.replication_null_atlas_schema_version,
        classifier.replication_null_atlas_status,
        classifier.replication_null_atlas_branch_row_count,
        classifier.replication_null_atlas_theorem_candidate_count,
        classifier.replication_null_atlas_next_target
    ));
    out.push_str(&format!(
        "- Replication null separator outcomes retained/split/collapsed: `{}` / `{}` / `{}`\n\n",
        classifier.replication_null_atlas_retained_separator_count,
        classifier.replication_null_atlas_split_separator_count,
        classifier.replication_null_atlas_collapsed_separator_count
    ));
    out.push_str(&format!(
        "- Pair-family cohort retention picker: `{}` selected `{}` connector `{}` target `{}`\n\n",
        classifier
            .pair_family_cohort_retention_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_pair_family_cohort_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_pair_family_cohort_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_pair_family_cohort_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family cohort residue profile: `{}` exact separators `{}` best modulus `{}` target `{}`\n\n",
        classifier
            .pair_family_cohort_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_cohort_residue_profile_exact_separator_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_cohort_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_cohort_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family surface picker: `{}` selected `{}` label `{}` target `{}`\n",
        classifier
            .pair_family_surface_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_pair_family_surface_id
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_pair_family_surface_label
            .as_deref()
            .unwrap_or("none"),
        classifier
            .selected_pair_family_surface_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family surface residue profile: `{}` exact separators `{}` best modulus `{}` target `{}`\n\n",
        classifier
            .pair_family_surface_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_surface_residue_profile_exact_separator_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_surface_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_surface_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family top-N motif surface profile: `{}` top-N `{}` motifs `{}` fresh survivors `{}` exact separators `{}` best modulus `{}` target `{}`\n\n",
        classifier
            .pair_family_topn_motif_surface_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_topn_motif_surface_profile_top_n
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_topn_motif_surface_profile_source_motif_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_topn_motif_surface_profile_fresh_survivor_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_topn_motif_surface_profile_exact_separator_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_topn_motif_surface_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_topn_motif_surface_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family gap cohort geometry control: `{}` top-N `{}` motifs `{}` retained geometry `{}` selected connector `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_geometry_control_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_geometry_control_top_n
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_geometry_control_source_motif_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_geometry_control_retained_geometry_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_geometry_control_selected_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_geometry_control_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family gap cohort residue profile: `{}` exact separators `{}` best modulus `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_residue_profile_exact_separator_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family gap cohort separator replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_residue_separator_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_residue_separator_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_residue_separator_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family gap cohort ratio geometry control: `{}` connector `{}` bias `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_ratio_geometry_control_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_control_selected_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_control_selected_bias
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_control_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family gap cohort ratio geometry replication: `{}` status `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_ratio_geometry_replication_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_replication_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_replication_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family gap cohort ratio geometry expansion: `{}` status `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_ratio_geometry_expansion_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_expansion_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_expansion_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family gap cohort ratio correction-bound stability: `{}` status `{}` stable bounds `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_ratio_correction_bound_stability_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_correction_bound_stability_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_correction_bound_stable_bound_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_ratio_correction_bound_stability_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Pair-family gap cohort ratio geometry atlas: `{}` status `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_ratio_geometry_atlas_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_atlas_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_atlas_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Frozen portfolio cohort-invariant picker: `{}` stable candidates `{}` selected connector `{}` direction `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_ratio_geometry_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_picker_stable_candidate_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_ratio_geometry_picker_selected_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_picker_selected_direction
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_picker_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Cohort-invariant residue profile: `{}` status `{}` best modulus `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_ratio_geometry_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_residue_profile_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_ratio_geometry_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Next cohort-invariant picker: `{}` excluded profiles `{}` selected connector `{}` direction `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_ratio_geometry_next_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_next_picker_excluded_profile_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_ratio_geometry_next_picker_selected_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_next_picker_selected_direction
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_next_picker_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Next cohort-invariant residue profile: `{}` status `{}` best modulus `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_ratio_geometry_next_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_next_residue_profile_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_next_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_ratio_geometry_next_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Post-two-null cohort-invariant picker: `{}` excluded profiles `{}` selected connector `{}` direction `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_ratio_geometry_post_two_null_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_post_two_null_picker_excluded_profile_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_ratio_geometry_post_two_null_picker_selected_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_post_two_null_picker_selected_direction
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_post_two_null_picker_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Post-two-null cohort-invariant residue profile: `{}` status `{}` best modulus `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Three-null cohort-invariant conclusion: `{}` status `{}` collapsed profiles `{}` selected connector `{}` direction `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_ratio_geometry_three_null_conclusion_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_three_null_conclusion_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_three_null_conclusion_collapsed_profile_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_ratio_geometry_three_null_conclusion_selected_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_three_null_conclusion_selected_direction
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_three_null_conclusion_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Forward cohort-invariant residue profile: `{}` status `{}` best modulus `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_ratio_geometry_forward_residue_profile_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_forward_residue_profile_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_forward_residue_profile_best_modulus
            .map(|modulus| modulus.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_ratio_geometry_forward_residue_profile_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Forward-route cohort-invariant conclusion: `{}` status `{}` collapsed profiles `{}` remaining candidates `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_collapsed_profile_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_remaining_candidate_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Window-consensus cohort-invariant surface: `{}` candidates `{}` selected connector `{}` direction `{}` status `{}` consensus windows `{}` target `{}`\n",
        classifier
            .pair_family_gap_cohort_window_consensus_surface_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_window_consensus_surface_candidate_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_window_consensus_surface_selected_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_window_consensus_surface_selected_direction
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_window_consensus_surface_selected_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_window_consensus_surface_selected_consensus_window_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_window_consensus_surface_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Window-consensus held-out stress: `{}` status `{}` retained/split/collapsed `{}`/`{}`/`{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_window_consensus_stress_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_window_consensus_stress_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_window_consensus_stress_retained_surface_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_window_consensus_stress_split_surface_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_window_consensus_stress_collapsed_surface_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_window_consensus_stress_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Sign-persistence cohort-invariant picker: `{}` candidates `{}` persistent `{}` selected connector `{}` direction `{}` status `{}` surfaces `{}` volatility `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_sign_persistence_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_sign_persistence_picker_candidate_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_sign_persistence_picker_persistent_candidate_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_sign_persistence_picker_selected_connector
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_sign_persistence_picker_selected_direction
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_sign_persistence_picker_selected_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_sign_persistence_picker_selected_surface_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_sign_persistence_picker_selected_volatility_score
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_sign_persistence_picker_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Sign-persistence fresh stress: `{}` decision `{}` retained/split/neutral surfaces `{}/{}/{}` retained/opposite windows `{}/{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_sign_persistence_stress_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_sign_persistence_stress_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_sign_persistence_stress_retained_surface_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_sign_persistence_stress_split_surface_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_sign_persistence_stress_neutral_surface_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_sign_persistence_stress_retained_window_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_sign_persistence_stress_opposite_window_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_sign_persistence_stress_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Volatility/ensemble cohort picker: `{}` ensembles `{}` qualifying `{}` selected direction `{}` connectors `{}` surfaces `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_volatility_ensemble_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_volatility_ensemble_picker_ensemble_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_volatility_ensemble_picker_qualifying_ensemble_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_volatility_ensemble_picker_selected_direction
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_volatility_ensemble_picker_selected_connector_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_volatility_ensemble_picker_selected_supported_surface_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_volatility_ensemble_picker_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Volatility/ensemble cohort stress: `{}` decision `{}` direction `{}` connectors `{}` retained/mixed/split/collapsed surfaces `{}/{}/{}/{}` retained/opposite windows `{}/{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_volatility_ensemble_stress_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_volatility_ensemble_stress_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_volatility_ensemble_stress_selected_direction
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_volatility_ensemble_stress_selected_connector_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_volatility_ensemble_stress_retained_surface_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_volatility_ensemble_stress_mixed_retained_surface_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_volatility_ensemble_stress_split_surface_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_volatility_ensemble_stress_collapsed_surface_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_volatility_ensemble_stress_retained_window_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_volatility_ensemble_stress_opposite_window_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_volatility_ensemble_stress_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Surface-family contrast picker: `{}` status `{}` selected family `{}` opposite family `{}` retained/split families `{}/{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_surface_family_contrast_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_picker_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_picker_selected_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_picker_opposite_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_picker_retained_family_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_picker_split_family_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_picker_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Surface-family contrast stress: `{}` decision `{}` selected family `{}` opposite family `{}` retained/split families `{}/{}` retained/opposite windows `{}/{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_surface_family_contrast_stress_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_stress_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_stress_selected_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_stress_opposite_family
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_stress_retained_family_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_stress_split_family_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_stress_retained_window_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_stress_opposite_window_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_stress_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Surface-family contrast anatomy: `{}` decision `{}` full/gap-only/size-only drivers `{}/{}/{}` top-share-bp `{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_surface_family_contrast_anatomy_concentration_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_anatomy_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_anatomy_full_driver_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_anatomy_gap_only_driver_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_anatomy_size_only_driver_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_anatomy_top_driver_share_basis_points
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_contrast_anatomy_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Surface-family driver-cohort stress: `{}` decision `{}` drivers `{}` retained/split families `{}/{}` retained/opposite windows `{}/{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_surface_family_driver_cohort_stress_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_surface_family_driver_cohort_stress_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_surface_family_driver_cohort_stress_driver_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_driver_cohort_stress_retained_family_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_driver_cohort_stress_split_family_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_driver_cohort_stress_retained_window_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_driver_cohort_stress_opposite_window_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_driver_cohort_stress_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Surface-family matched non-driver control stress: `{}` decision `{}` controls `{}` retained/split families `{}/{}` retained/opposite windows `{}/{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_status
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_control_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_retained_family_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_split_family_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_retained_window_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_opposite_window_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(&format!(
        "- Surface-agnostic ensemble picker: `{}` candidates `{}` stable connectors `{}` selected direction `{}` connectors `{}` stable-surface total `{}` retained/opposite windows `{}/{}` target `{}`\n\n",
        classifier
            .pair_family_gap_cohort_surface_agnostic_ensemble_picker_decision
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_surface_agnostic_ensemble_picker_candidate_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_agnostic_ensemble_picker_stable_connector_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_agnostic_ensemble_picker_selected_direction
            .as_deref()
            .unwrap_or("none"),
        classifier
            .pair_family_gap_cohort_surface_agnostic_ensemble_picker_selected_connector_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_agnostic_ensemble_picker_selected_supported_surface_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_agnostic_ensemble_picker_retained_window_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_agnostic_ensemble_picker_opposite_window_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "none".to_string()),
        classifier
            .pair_family_gap_cohort_surface_agnostic_ensemble_picker_target
            .as_deref()
            .unwrap_or("none")
    ));
    out.push_str(
        "| Edge | Width | Connector | Moduli | Outside-ladder status | Summary theorem |\n",
    );
    out.push_str("|---|---:|---|---|---|---|\n");
    for cell in &classifier.cells {
        out.push_str(&format!(
            "| `{}` | {} | `{}` | `{:?}` | `{}` | `{}` |\n",
            cell.edge,
            cell.width,
            cell.connector,
            cell.moduli,
            cell.outside_ladder_cell_status,
            cell.lean_summary_theorem
        ));
    }
    out.push('\n');

    out.push_str("## Next Theorem Targets\n\n");
    for row in catalog
        .rows
        .iter()
        .filter(|row| row.next_theorem_target.is_some())
    {
        out.push_str(&format!(
            "- `{}`: {}\n",
            row.signal_id,
            row.next_theorem_target
                .as_ref()
                .expect("filtered for target")
        ));
    }
    out
}

pub fn verify_signal_catalog(
    catalog: &SignalCatalog,
    repo_root: impl AsRef<Path>,
) -> SignalCatalogVerification {
    let repo_root = repo_root.as_ref();
    let mut failures = Vec::new();

    for row in &catalog.rows {
        if row.artifact_path.trim().is_empty() {
            failures.push(SignalCatalogVerificationFailure {
                signal_id: row.signal_id.clone(),
                field: "artifact_path".to_string(),
                value: row.artifact_path.clone(),
                message: "artifact_path must be nonempty".to_string(),
            });
        } else {
            let artifact_path = Path::new(&row.artifact_path);
            let absolute_artifact_path = if artifact_path.is_absolute() {
                artifact_path.to_path_buf()
            } else {
                repo_root.join(artifact_path)
            };
            if !absolute_artifact_path.exists() {
                failures.push(SignalCatalogVerificationFailure {
                    signal_id: row.signal_id.clone(),
                    field: "artifact_path".to_string(),
                    value: row.artifact_path.clone(),
                    message: "artifact_path does not exist".to_string(),
                });
            }
        }

        let command = row.drift_check_command.trim();
        if command.is_empty() {
            failures.push(SignalCatalogVerificationFailure {
                signal_id: row.signal_id.clone(),
                field: "drift_check_command".to_string(),
                value: row.drift_check_command.clone(),
                message: "drift_check_command must be nonempty".to_string(),
            });
        } else if !KNOWN_SIGNAL_CATALOG_DRIFT_CHECK_COMMANDS.contains(&command) {
            failures.push(SignalCatalogVerificationFailure {
                signal_id: row.signal_id.clone(),
                field: "drift_check_command".to_string(),
                value: row.drift_check_command.clone(),
                message: "drift_check_command is not in the maintained allow-list".to_string(),
            });
        }
    }

    SignalCatalogVerification {
        ok: failures.is_empty(),
        checked_rows: catalog.rows.len(),
        failures,
        gate_results: Vec::new(),
    }
}

pub fn verify_signal_catalog_deep(
    catalog: &SignalCatalog,
    repo_root: impl AsRef<Path>,
    timeout: Duration,
) -> SignalCatalogVerification {
    let repo_root = repo_root.as_ref();
    let mut verification = verify_signal_catalog(catalog, repo_root);
    if !verification.ok {
        return verification;
    }

    let gate_results: Vec<_> = catalog
        .rows
        .iter()
        .map(|row| run_signal_catalog_gate(row, repo_root, timeout))
        .collect();
    let gates_ok = gate_results.iter().all(|result| result.ok);
    verification.gate_results = gate_results;
    verification.ok = verification.ok && gates_ok;
    verification
}

fn run_signal_catalog_gate(
    row: &SignalCatalogRow,
    repo_root: &Path,
    timeout: Duration,
) -> SignalCatalogGateResult {
    let started = Instant::now();
    let command_parts: Vec<_> = row.drift_check_command.split_whitespace().collect();
    let Some((program, args)) = command_parts.split_first() else {
        return SignalCatalogGateResult {
            signal_id: row.signal_id.clone(),
            drift_check_command: row.drift_check_command.clone(),
            status: "spawn-error".to_string(),
            ok: false,
            timed_out: false,
            exit_code: None,
            duration_ms: 0,
            error_message: Some("drift_check_command is empty".to_string()),
        };
    };

    let mut child = match Command::new(program)
        .args(args)
        .current_dir(repo_root)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(child) => child,
        Err(err) => {
            return SignalCatalogGateResult {
                signal_id: row.signal_id.clone(),
                drift_check_command: row.drift_check_command.clone(),
                status: "spawn-error".to_string(),
                ok: false,
                timed_out: false,
                exit_code: None,
                duration_ms: elapsed_ms(started),
                error_message: Some(err.to_string()),
            };
        }
    };

    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let ok = status.success();
                return SignalCatalogGateResult {
                    signal_id: row.signal_id.clone(),
                    drift_check_command: row.drift_check_command.clone(),
                    status: if ok { "passed" } else { "failed" }.to_string(),
                    ok,
                    timed_out: false,
                    exit_code: status.code(),
                    duration_ms: elapsed_ms(started),
                    error_message: None,
                };
            }
            Ok(None) => {
                if started.elapsed() >= timeout {
                    let kill_error = child.kill().err().map(|err| err.to_string());
                    let _ = child.wait();
                    return SignalCatalogGateResult {
                        signal_id: row.signal_id.clone(),
                        drift_check_command: row.drift_check_command.clone(),
                        status: "timed-out".to_string(),
                        ok: false,
                        timed_out: true,
                        exit_code: None,
                        duration_ms: elapsed_ms(started),
                        error_message: kill_error.or_else(|| {
                            Some(format!("timed out after {} ms", timeout.as_millis()))
                        }),
                    };
                }
                thread::sleep(Duration::from_millis(25));
            }
            Err(err) => {
                let _ = child.kill();
                let _ = child.wait();
                return SignalCatalogGateResult {
                    signal_id: row.signal_id.clone(),
                    drift_check_command: row.drift_check_command.clone(),
                    status: "wait-error".to_string(),
                    ok: false,
                    timed_out: false,
                    exit_code: None,
                    duration_ms: elapsed_ms(started),
                    error_message: Some(err.to_string()),
                };
            }
        }
    }
}

fn elapsed_ms(started: Instant) -> u64 {
    u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX)
}

fn signal_catalog_summary(rows: &[SignalCatalogRow]) -> SignalCatalogSummary {
    SignalCatalogSummary {
        row_count: rows.len(),
        matched_control_rows: rows
            .iter()
            .filter(|row| row.domain == "matched-control")
            .count(),
        witness_rows: rows.iter().filter(|row| row.domain == "witness").count(),
        connector_rows: rows.iter().filter(|row| row.domain == "connector").count(),
        proof_carrying_rows: rows
            .iter()
            .filter(|row| !row.proof_status.contains("not an empirical report"))
            .count(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        path::PathBuf,
        process,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn signal_catalog_has_required_rows_in_stable_order() {
        let catalog = build_signal_catalog();

        assert_eq!(catalog.schema_version, SIGNAL_CATALOG_SCHEMA_VERSION);
        assert_eq!(catalog.schema_version, "signal-catalog-v76");
        assert_eq!(catalog.rows.len(), 7);
        assert_eq!(catalog.rows[0].signal_id, "matched-control-smoke-atlas");
        assert_eq!(catalog.rows[4].signal_id, "connector-signal-atlas");
        assert_eq!(catalog.rows[5].signal_id, "connector-width6-stress");
        assert_eq!(
            catalog.rows[6].signal_id,
            "connector-replication-null-atlas"
        );
        assert_eq!(catalog.summary.matched_control_rows, 1);
        assert_eq!(catalog.summary.witness_rows, 3);
        assert_eq!(catalog.summary.connector_rows, 3);
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .source_artifact_path,
            "docs/connector/connector_width6_stress.json"
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .source_schema_version,
            "connector-width6-stress-v79"
        );
        assert_eq!(
            catalog.connector_digit8_classifier_family.surface_status,
            "complete-visible-digit8-exact-separator-family"
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .theorem_backed_multi_modulus_cell_count,
            3
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .unclassified_exact_separator_count,
            0
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .replication_selection_rule,
            "theorem-backed-digit8-classifier-cells-tested-on-next-twelve-twin-prime-pairs-only"
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .outside_ladder_replication_decision,
            "digit8-classifier-family-partly-collapses-on-outside-ladder"
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .outside_ladder_added_pair_count,
            12
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .outside_ladder_retained_cell_count,
            0
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .outside_ladder_split_cell_count,
            0
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .outside_ladder_collapsed_cell_count,
            3
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .split_follow_up_decision,
            "split-signal-partly-collapses-on-second-outside-ladder"
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .split_follow_up_source_row_count,
            2
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .split_follow_up_tested_row_count,
            2
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .split_follow_up_stabilized_row_count,
            0
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .split_follow_up_split_again_row_count,
            0
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .split_follow_up_collapsed_row_count,
            2
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .branch_picker_decision,
            "all-branches-collapsed-after-independent-mod3-guardrail"
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_next_branch_id
                .as_deref(),
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_next_branch_status
                .as_deref(),
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_next_branch_target
                .as_deref(),
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_branch_independent_replication_decision
                .as_deref(),
            Some("retired-all-fresh-independent-rows-theorem-blocked-by-mod3-null-layer")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .retired_branch_id
                .as_deref(),
            Some("trailing-edge-width8-digit6-connector-00000006")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_branch_independent_replication_next_target
                .as_deref(),
            Some("select-new-non-mod3-connector-stress-family-after-00000006-retirement")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .non_mod3_candidate_picker_decision
                .as_deref(),
            Some("fresh-nonmod3-candidate-selected-after-retiring-collapsed-separator")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .retired_non_mod3_candidate_count,
            1
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .retired_non_mod3_candidate_ids,
            vec!["trailing-edge-width7-digit7-connector-0000007".to_string()]
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_non_mod3_candidate_id
                .as_deref(),
            Some("trailing-edge-width7-digit1-connector-0000001")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_non_mod3_candidate_target
                .as_deref(),
            Some("independently-replicate-nonmod3-0000001-trailing-edge-width7-digit1")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .non_mod3_second_replication_decision
                .as_deref(),
            Some("survived-second-independent-ladder-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .non_mod3_second_replication_next_target
                .as_deref(),
            Some("residue-profile-nonmod3-0000001-trailing-edge-width7-digit1")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .non_mod3_residue_profile_decision
                .as_deref(),
            Some("small-prime-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .non_mod3_residue_profile_best_modulus,
            Some(17)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .non_mod3_residue_profile_next_target
                .as_deref(),
            Some("replicate-nonmod3-0000001-mod17-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .non_mod3_residue_separator_replication_decision
                .as_deref(),
            Some("mod17-residue-separator-split-keep-empirical")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .non_mod3_residue_separator_replication_status
                .as_deref(),
            Some("split-exact-residue-separator-on-third-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .non_mod3_residue_separator_replication_next_target
                .as_deref(),
            Some("replicate-mutated-nonmod3-0000001-mod17-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .non_mod3_mutated_residue_separator_replication_decision
                .as_deref(),
            Some("mod17-mutated-residue-separator-collapsed-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .non_mod3_mutated_residue_separator_replication_status
                .as_deref(),
            Some("collapsed-no-reverse-only-on-fourth-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .non_mod3_mutated_residue_separator_replication_next_target
                .as_deref(),
            Some("select-next-nonmod3-connector-stress-family-after-0000001-retirement")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .next_non_mod3_candidate_picker_decision
                .as_deref(),
            Some("fresh-nonmod3-candidate-selected-after-retiring-collapsed-separator")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_next_non_mod3_candidate_id
                .as_deref(),
            Some("trailing-edge-width8-digit5-connector-00000005")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_next_non_mod3_candidate_target
                .as_deref(),
            Some("independently-replicate-nonmod3-00000005-trailing-edge-width8-digit5")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .next_non_mod3_independent_replication_decision
                .as_deref(),
            Some("collapsed-on-second-independent-ladder-retire-without-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .next_non_mod3_independent_replication_target
                .as_deref(),
            Some("retire-nonmod3-00000005-trailing-edge-width8-digit5")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .non_mod3_retired_edge_candidate_count,
            3
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .non_mod3_retired_edge_candidate_ids,
            vec![
                "trailing-edge-width7-digit1-connector-0000001".to_string(),
                "trailing-edge-width7-digit7-connector-0000007".to_string(),
                "trailing-edge-width8-digit5-connector-00000005".to_string()
            ]
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_family_picker_decision
                .as_deref(),
            Some("interior-nonmod3-family-selected-for-independent-replication")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_interior_non_mod3_candidate_id
                .as_deref(),
            Some("interior-width5-position3-digit1-connector-00010")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_interior_non_mod3_candidate_target
                .as_deref(),
            Some("independently-replicate-interior-nonmod3-00010-width5-position3-digit1")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_family_independent_replication_decision
                .as_deref(),
            Some("survived-interior-independent-ladder-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_family_independent_replication_target
                .as_deref(),
            Some("residue-profile-interior-nonmod3-00010-width5-position3-digit1")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_residue_profile_decision
                .as_deref(),
            Some("small-prime-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_residue_profile_best_modulus,
            Some(19)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_residue_profile_target
                .as_deref(),
            Some("replicate-interior-nonmod3-00010-mod19-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_residue_separator_replication_decision
                .as_deref(),
            Some("mod19-interior-residue-separator-mutated-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_residue_separator_replication_status
                .as_deref(),
            Some("split-exact-residue-separator-on-interior-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_residue_separator_replication_target
                .as_deref(),
            Some("retire-interior-nonmod3-00010-mod19-residue-separator-after-mutation")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_retired_candidate_count,
            7
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_retired_candidate_ids,
            vec![
                "interior-width5-position1-digit4-connector-04000".to_string(),
                "interior-width5-position1-digit5-connector-05000".to_string(),
                "interior-width5-position3-digit1-connector-00010".to_string(),
                "interior-width7-position5-digit7-connector-0000070".to_string(),
                "interior-width9-position4-digit5-connector-000050000".to_string(),
                "interior-width9-position5-digit7-connector-000007000".to_string(),
                "interior-width9-position7-digit7-connector-000000070".to_string()
            ]
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_next_family_picker_decision
                .as_deref(),
            Some("interior-nonmod3-family-selected-for-independent-replication")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_next_interior_non_mod3_candidate_id
                .as_deref(),
            Some("interior-width7-position5-digit7-connector-0000070")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_next_interior_non_mod3_candidate_target
                .as_deref(),
            Some("independently-replicate-interior-nonmod3-0000070-width7-position5-digit7")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_next_family_independent_replication_decision
                .as_deref(),
            Some("survived-interior-independent-ladder-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_next_family_independent_replication_target
                .as_deref(),
            Some("residue-profile-interior-nonmod3-0000070-width7-position5-digit7")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_next_residue_profile_decision
                .as_deref(),
            Some("small-prime-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_next_residue_profile_best_modulus,
            Some(17)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_next_residue_profile_target
                .as_deref(),
            Some("replicate-interior-nonmod3-0000070-mod17-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_next_residue_separator_replication_decision
                .as_deref(),
            Some("mod17-interior-residue-separator-collapsed-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_next_residue_separator_replication_status
                .as_deref(),
            Some("collapsed-overlapping-residue-classes-on-interior-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_next_residue_separator_replication_target
                .as_deref(),
            Some("retire-interior-nonmod3-0000070-mod17-residue-separator-after-mutation")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_post_retirement_family_picker_decision
                .as_deref(),
            Some("interior-nonmod3-family-selected-for-independent-replication")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_post_retirement_interior_non_mod3_candidate_id
                .as_deref(),
            Some("interior-width5-position1-digit4-connector-04000")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_post_retirement_interior_non_mod3_candidate_target
                .as_deref(),
            Some("independently-replicate-interior-nonmod3-04000-width5-position1-digit4")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_post_retirement_family_independent_replication_decision
                .as_deref(),
            Some("survived-interior-independent-ladder-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_post_retirement_family_independent_replication_target
                .as_deref(),
            Some("residue-profile-interior-nonmod3-04000-width5-position1-digit4")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_post_retirement_residue_profile_decision
                .as_deref(),
            Some("small-prime-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_post_retirement_residue_profile_best_modulus,
            Some(17)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_post_retirement_residue_profile_target
                .as_deref(),
            Some("replicate-interior-nonmod3-04000-mod17-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_post_retirement_residue_separator_replication_decision
                .as_deref(),
            Some("mod17-interior-residue-separator-mutated-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_post_retirement_residue_separator_replication_status
                .as_deref(),
            Some("split-exact-residue-separator-on-interior-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_post_retirement_residue_separator_replication_target
                .as_deref(),
            Some("retire-interior-nonmod3-04000-mod17-residue-separator-after-mutation")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_third_retirement_family_picker_decision
                .as_deref(),
            Some("interior-nonmod3-family-selected-for-independent-replication")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_after_third_retirement_interior_non_mod3_candidate_id
                .as_deref(),
            Some("interior-width5-position1-digit5-connector-05000")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_after_third_retirement_interior_non_mod3_candidate_target
                .as_deref(),
            Some("independently-replicate-interior-nonmod3-05000-width5-position1-digit5")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_third_retirement_family_independent_replication_decision
                .as_deref(),
            Some("survived-interior-independent-ladder-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_third_retirement_family_independent_replication_target
                .as_deref(),
            Some("residue-profile-interior-nonmod3-05000-width5-position1-digit5")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_third_retirement_residue_profile_decision
                .as_deref(),
            Some("small-prime-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_third_retirement_residue_profile_best_modulus,
            Some(11)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_third_retirement_residue_profile_target
                .as_deref(),
            Some("replicate-interior-nonmod3-05000-mod11-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_third_retirement_residue_separator_replication_decision
                .as_deref(),
            Some("mod11-interior-residue-separator-collapsed-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_third_retirement_residue_separator_replication_status
                .as_deref(),
            Some("collapsed-no-reverse-only-on-interior-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_third_retirement_residue_separator_replication_target
                .as_deref(),
            Some("retire-interior-nonmod3-05000-mod11-residue-separator-after-mutation")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_fourth_retirement_family_picker_decision
                .as_deref(),
            Some("interior-nonmod3-family-selected-for-independent-replication")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_after_fourth_retirement_interior_non_mod3_candidate_id
                .as_deref(),
            Some("interior-width9-position4-digit5-connector-000050000")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_after_fourth_retirement_interior_non_mod3_candidate_target
                .as_deref(),
            Some("independently-replicate-interior-nonmod3-000050000-width9-position4-digit5")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_fourth_retirement_family_independent_replication_decision
                .as_deref(),
            Some("survived-interior-independent-ladder-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_fourth_retirement_family_independent_replication_target
                .as_deref(),
            Some("residue-profile-interior-nonmod3-000050000-width9-position4-digit5")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_fourth_retirement_residue_profile_decision
                .as_deref(),
            Some("small-prime-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_fourth_retirement_residue_profile_best_modulus,
            Some(13)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_fourth_retirement_residue_profile_target
                .as_deref(),
            Some("replicate-interior-nonmod3-000050000-mod13-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_fourth_retirement_residue_separator_replication_decision
                .as_deref(),
            Some("mod13-interior-residue-separator-collapsed-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_fourth_retirement_residue_separator_replication_status
                .as_deref(),
            Some("collapsed-no-reverse-only-on-interior-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_fourth_retirement_residue_separator_replication_target
                .as_deref(),
            Some("retire-interior-nonmod3-000050000-mod13-residue-separator-after-mutation")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_fifth_retirement_family_picker_decision
                .as_deref(),
            Some("interior-nonmod3-family-selected-for-independent-replication")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_after_fifth_retirement_interior_non_mod3_candidate_id
                .as_deref(),
            Some("interior-width9-position5-digit7-connector-000007000")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_after_fifth_retirement_interior_non_mod3_candidate_target
                .as_deref(),
            Some("independently-replicate-interior-nonmod3-000007000-width9-position5-digit7")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_fifth_retirement_family_independent_replication_decision
                .as_deref(),
            Some("collapsed-interior-independent-ladder-retire-without-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_fifth_retirement_family_independent_replication_target
                .as_deref(),
            Some("retire-interior-nonmod3-000007000-width9-position5-digit7")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_sixth_retirement_family_picker_decision
                .as_deref(),
            Some("interior-nonmod3-family-selected-for-independent-replication")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_after_sixth_retirement_interior_non_mod3_candidate_id
                .as_deref(),
            Some("interior-width9-position7-digit7-connector-000000070")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_after_sixth_retirement_interior_non_mod3_candidate_target
                .as_deref(),
            Some("independently-replicate-interior-nonmod3-000000070-width9-position7-digit7")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_sixth_retirement_family_independent_replication_decision
                .as_deref(),
            Some("collapsed-interior-independent-ladder-retire-without-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .interior_non_mod3_after_sixth_retirement_family_independent_replication_target
                .as_deref(),
            Some("retire-interior-nonmod3-000000070-width9-position7-digit7")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .single_digit_interior_pivot_decision
                .as_deref(),
            Some(
                "pivot-away-from-single-digit-interior-family-after-repeated-fresh-ladder-collapse"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .multi_digit_motif_family_picker_decision
                .as_deref(),
            Some("multi-digit-motif-selected-for-independent-replication")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_multi_digit_motif_id
                .as_deref(),
            Some("multidigit-motif-width5-start1-digits11-connector-01100")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_multi_digit_motif_target
                .as_deref(),
            Some("independently-replicate-multidigit-motif-01100-width5-start1-digits11")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .multi_digit_motif_family_independent_replication_decision
                .as_deref(),
            Some("survived-multidigit-motif-independent-ladder-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .multi_digit_motif_family_independent_replication_target
                .as_deref(),
            Some("residue-profile-multidigit-motif-01100-width5-start1-digits11")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .multi_digit_motif_residue_profile_decision
                .as_deref(),
            Some("small-prime-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .multi_digit_motif_residue_profile_best_modulus,
            Some(11)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .multi_digit_motif_residue_profile_target
                .as_deref(),
            Some("replicate-multidigit-motif-01100-mod11-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .multi_digit_motif_residue_separator_replication_decision
                .as_deref(),
            Some("mod11-multidigit-residue-separator-collapsed-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .multi_digit_motif_residue_separator_replication_status
                .as_deref(),
            Some("collapsed-no-reverse-only-on-multidigit-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .multi_digit_motif_residue_separator_replication_target
                .as_deref(),
            Some("retire-multidigit-motif-01100-mod11-residue-separator-after-mutation")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .multi_digit_motif_retired_count,
            1
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .retired_multi_digit_motif_ids,
            vec!["multidigit-motif-width5-start1-digits11-connector-01100".to_string()]
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_pair_family_retired_count,
            2
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .retired_orthogonal_pair_family_branch_ids,
            vec![
                "orthogonal-gap6-width6-start2-digits52-connector-005200".to_string(),
                "orthogonal-gap6-width6-start3-digits41-connector-000410".to_string()
            ]
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_pair_family_control_matrix_decision
                .as_deref(),
            Some("orthogonal-pair-family-control-survived-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_pair_family_branch_id
                .as_deref(),
            Some("orthogonal-gap4-width5-start1-digits47-connector-04700")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_pair_family
                .as_deref(),
            Some("cousin-prime-gap4")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_pair_family_connector
                .as_deref(),
            Some("04700")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_pair_family_target
                .as_deref(),
            Some("residue-profile-orthogonal-gap4-multidigit-motif-04700-width5-start1-digits47")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_pair_family_residue_profile_decision
                .as_deref(),
            Some("small-prime-orthogonal-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_pair_family_residue_profile_best_modulus,
            Some(13)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_pair_family_residue_profile_target
                .as_deref(),
            Some("replicate-orthogonal-gap4-multidigit-motif-04700-mod13-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_pair_family_residue_separator_replication_decision
                .as_deref(),
            Some("mod13-orthogonal-residue-separator-collapsed-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_pair_family_residue_separator_replication_status
                .as_deref(),
            Some("collapsed-overlapping-residue-classes-on-orthogonal-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_pair_family_residue_separator_replication_target
                .as_deref(),
            Some("pivot-away-from-orthogonal-adjacent-two-digit-motifs-after-repeated-three-ladder-collapse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_compact_three_digit_control_decision
                .as_deref(),
            Some("orthogonal-compact-three-digit-control-survived-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_compact_three_digit_branch_id
                .as_deref(),
            Some("orthogonal-compact3-gap4-width5-start1-digits251-connector-02510")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_compact_three_digit_family
                .as_deref(),
            Some("cousin-prime-gap4")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_compact_three_digit_connector
                .as_deref(),
            Some("02510")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_compact_three_digit_target
                .as_deref(),
            Some("residue-profile-orthogonal-gap4-multidigit-motif-02510-width5-start1-digits251")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_compact_three_digit_residue_profile_decision
                .as_deref(),
            Some("small-prime-orthogonal-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_compact_three_digit_residue_profile_best_modulus,
            Some(29)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_compact_three_digit_residue_profile_target
                .as_deref(),
            Some("replicate-orthogonal-gap4-multidigit-motif-02510-mod29-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_compact_three_digit_residue_separator_replication_decision
                .as_deref(),
            Some("mod29-orthogonal-residue-separator-collapsed-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_compact_three_digit_residue_separator_replication_status
                .as_deref(),
            Some("collapsed-overlapping-residue-classes-on-orthogonal-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_compact_three_digit_residue_separator_replication_target
                .as_deref(),
            Some("select-next-orthogonal-nonadjacent-two-digit-motif-family-after-compact-three-digit-collapse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_nonadjacent_two_digit_control_decision
                .as_deref(),
            Some("orthogonal-nonadjacent-two-digit-control-survived-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_nonadjacent_two_digit_branch_id
                .as_deref(),
            Some("orthogonal-nonadjacent2-gap4-width7-pos25-digits52-connector-0050020")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_nonadjacent_two_digit_family
                .as_deref(),
            Some("cousin-prime-gap4")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_nonadjacent_two_digit_connector
                .as_deref(),
            Some("0050020")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_nonadjacent_two_digit_target
                .as_deref(),
            Some("residue-profile-orthogonal-nonadjacent2-gap4-multidigit-motif-0050020-width7-pos25-digits52")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_nonadjacent_two_digit_residue_profile_decision
                .as_deref(),
            Some("small-prime-orthogonal-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_nonadjacent_two_digit_residue_profile_best_modulus,
            Some(11)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_nonadjacent_two_digit_residue_profile_target
                .as_deref(),
            Some("replicate-orthogonal-nonadjacent2-gap4-multidigit-motif-0050020-pos25-mod11-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_nonadjacent_two_digit_residue_separator_replication_decision
                .as_deref(),
            Some("mod11-orthogonal-residue-separator-mutated-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_nonadjacent_two_digit_residue_separator_replication_status
                .as_deref(),
            Some("split-exact-residue-separator-on-orthogonal-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_nonadjacent_two_digit_residue_separator_replication_target
                .as_deref(),
            Some("pivot-away-from-small-digit-orthogonal-motifs-after-nonadjacent-two-digit-three-ladder-collapse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_edge_plus_interior_control_decision
                .as_deref(),
            Some("orthogonal-edge-plus-interior-control-survived-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_edge_plus_interior_branch_id
                .as_deref(),
            Some("orthogonal-edgeplus2-gap4-width5-pos34-digits22-connector-00022")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_edge_plus_interior_family
                .as_deref(),
            Some("cousin-prime-gap4")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_edge_plus_interior_connector
                .as_deref(),
            Some("00022")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_edge_plus_interior_target
                .as_deref(),
            Some("residue-profile-orthogonal-edgeplus2-gap4-motif-00022-width5-pos34-digits22")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_edge_plus_interior_residue_profile_decision
                .as_deref(),
            Some("small-prime-orthogonal-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_edge_plus_interior_residue_profile_best_modulus,
            Some(13)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_edge_plus_interior_residue_profile_target
                .as_deref(),
            Some("replicate-orthogonal-edgeplus2-gap4-motif-00022-pos34-mod13-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_edge_plus_interior_residue_separator_replication_decision
                .as_deref(),
            Some("mod13-orthogonal-residue-separator-collapsed-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_edge_plus_interior_residue_separator_replication_status
                .as_deref(),
            Some("collapsed-no-reverse-only-on-orthogonal-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_edge_plus_interior_residue_separator_replication_target
                .as_deref(),
            Some(
                "pivot-away-from-edge-plus-interior-orthogonal-motifs-after-three-ladder-collapse"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_repeated_block_control_decision
                .as_deref(),
            Some("orthogonal-repeated-block-control-survived-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_repeated_block_branch_id
                .as_deref(),
            Some("orthogonal-repeatblock-gap4-width9-pos3467-digits5555-connector-000550550")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_repeated_block_family
                .as_deref(),
            Some("cousin-prime-gap4")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_repeated_block_connector
                .as_deref(),
            Some("000550550")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_repeated_block_target
                .as_deref(),
            Some("residue-profile-orthogonal-repeatblock-gap4-motif-000550550-width9-pos3467-digits5555")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_repeated_block_residue_profile_decision
                .as_deref(),
            Some("small-prime-orthogonal-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_repeated_block_residue_profile_best_modulus,
            Some(29)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_repeated_block_residue_profile_target
                .as_deref(),
            Some("replicate-orthogonal-repeatblock-gap4-motif-000550550-pos3467-mod29-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_repeated_block_residue_separator_replication_decision
                .as_deref(),
            Some("mod29-orthogonal-residue-separator-collapsed-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_repeated_block_residue_separator_replication_status
                .as_deref(),
            Some("collapsed-overlapping-residue-classes-on-orthogonal-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_repeated_block_residue_separator_replication_target
                .as_deref(),
            Some(
                "pivot-to-arithmetic-connector-families-after-repeated-block-three-ladder-collapse"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_arithmetic_connector_control_decision
                .as_deref(),
            Some("orthogonal-arithmetic-connector-control-survived-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_arithmetic_connector_branch_id
                .as_deref(),
            Some("orthogonal-arithmetic-gap4-width5-connector-04900")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_arithmetic_connector_family
                .as_deref(),
            Some("cousin-prime-gap4")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_arithmetic_connector_connector
                .as_deref(),
            Some("04900")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_arithmetic_connector_target
                .as_deref(),
            Some("residue-profile-orthogonal-arithmetic-gap4-connector-04900-width5")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_arithmetic_connector_residue_profile_decision
                .as_deref(),
            Some("small-prime-orthogonal-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_arithmetic_connector_residue_profile_best_modulus,
            Some(23)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_arithmetic_connector_residue_profile_target
                .as_deref(),
            Some("replicate-orthogonal-arithmetic-gap4-connector-04900-mod23-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_arithmetic_connector_residue_separator_replication_decision
                .as_deref(),
            Some("mod23-orthogonal-residue-separator-collapsed-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_arithmetic_connector_residue_separator_replication_status
                .as_deref(),
            Some("collapsed-overlapping-residue-classes-on-orthogonal-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_arithmetic_connector_residue_separator_replication_target
                .as_deref(),
            Some("select-next-arithmetic-connector-family-after-square-triangular-three-ladder-collapse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_residue_lattice_connector_control_decision
                .as_deref(),
            Some("orthogonal-residue-lattice-connector-control-survived-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_residue_lattice_connector_branch_id
                .as_deref(),
            Some("orthogonal-residuelattice-gap4-width6-connector-000122")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_residue_lattice_connector_family
                .as_deref(),
            Some("cousin-prime-gap4")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_residue_lattice_connector_connector
                .as_deref(),
            Some("000122")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_residue_lattice_connector_target
                .as_deref(),
            Some("residue-profile-orthogonal-residuelattice-gap4-connector-000122-width6")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_residue_lattice_connector_residue_profile_decision
                .as_deref(),
            Some("small-prime-orthogonal-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_residue_lattice_connector_residue_profile_best_modulus,
            Some(23)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_residue_lattice_connector_residue_profile_target
                .as_deref(),
            Some("replicate-orthogonal-residuelattice-gap4-connector-000122-mod23-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_residue_lattice_connector_residue_separator_replication_decision
                .as_deref(),
            Some("mod23-orthogonal-residue-separator-mutated-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_residue_lattice_connector_residue_separator_replication_status
                .as_deref(),
            Some("split-exact-residue-separator-on-orthogonal-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_residue_lattice_connector_residue_separator_replication_target
                .as_deref(),
            Some("select-next-arithmetic-connector-family-after-residue-lattice-three-ladder-collapse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_modular_walk_connector_control_decision
                .as_deref(),
            Some("orthogonal-modular-walk-connector-control-survived-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_modular_walk_connector_branch_id
                .as_deref(),
            Some("orthogonal-modularwalk-gap4-width6-connector-001139")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_modular_walk_connector_family
                .as_deref(),
            Some("cousin-prime-gap4")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_modular_walk_connector_connector
                .as_deref(),
            Some("001139")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_modular_walk_connector_target
                .as_deref(),
            Some("residue-profile-orthogonal-modularwalk-gap4-connector-001139-width6")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_modular_walk_connector_residue_profile_decision
                .as_deref(),
            Some("small-prime-orthogonal-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_modular_walk_connector_residue_profile_best_modulus,
            Some(29)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_modular_walk_connector_residue_profile_target
                .as_deref(),
            Some("replicate-orthogonal-modularwalk-gap4-connector-001139-mod29-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_modular_walk_connector_residue_separator_replication_decision
                .as_deref(),
            Some("mod29-orthogonal-residue-separator-mutated-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_modular_walk_connector_residue_separator_replication_status
                .as_deref(),
            Some("split-exact-residue-separator-on-orthogonal-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_modular_walk_connector_residue_separator_replication_target
                .as_deref(),
            Some(
                "select-next-arithmetic-connector-family-after-modular-walk-three-ladder-collapse"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_arithmetic_family_registry_decision
                .as_deref(),
            Some("retired-arithmetic-families-recorded-select-base-mixed-connectors-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_arithmetic_family_registry_retired_count,
            Some(8)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_arithmetic_family_registry_selected_family
                .as_deref(),
            Some("base-mixed")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_arithmetic_family_registry_selected_target
                .as_deref(),
            Some("scan-orthogonal-base-mixed-connectors-under-source-fresh-separator-gate")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_crt_paired_connector_control_decision
                .as_deref(),
            Some("orthogonal-crt-paired-connector-control-survived-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_crt_paired_connector_branch_id
                .as_deref(),
            Some("orthogonal-crtpaired-gap4-width6-connector-003727")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_crt_paired_connector_family
                .as_deref(),
            Some("cousin-prime-gap4")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_crt_paired_connector_connector
                .as_deref(),
            Some("003727")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_crt_paired_connector_target
                .as_deref(),
            Some("residue-profile-orthogonal-crtpaired-gap4-connector-003727-width6")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_crt_paired_connector_residue_profile_decision
                .as_deref(),
            Some("small-prime-orthogonal-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_crt_paired_connector_residue_profile_best_modulus,
            Some(23)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_crt_paired_connector_residue_profile_target
                .as_deref(),
            Some("replicate-orthogonal-crtpaired-gap4-connector-003727-mod23-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_crt_paired_connector_residue_separator_replication_decision
                .as_deref(),
            Some("mod23-orthogonal-residue-separator-collapsed-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_crt_paired_connector_residue_separator_replication_status
                .as_deref(),
            Some("collapsed-overlapping-residue-classes-on-orthogonal-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_crt_paired_connector_residue_separator_replication_target
                .as_deref(),
            Some("select-next-arithmetic-connector-family-after-crt-paired-three-ladder-collapse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_multiplicative_order_connector_control_decision
                .as_deref(),
            Some(
                "orthogonal-multiplicative-order-connector-control-survived-residue-profiler-next"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_multiplicative_order_connector_branch_id
                .as_deref(),
            Some("orthogonal-multorder-gap4-width6-connector-001139")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_multiplicative_order_connector_family
                .as_deref(),
            Some("cousin-prime-gap4")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_multiplicative_order_connector_connector
                .as_deref(),
            Some("001139")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_multiplicative_order_connector_target
                .as_deref(),
            Some("residue-profile-orthogonal-multorder-gap4-connector-001139-width6")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_multiplicative_order_connector_residue_profile_decision
                .as_deref(),
            Some("small-prime-orthogonal-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_multiplicative_order_connector_residue_profile_best_modulus,
            Some(29)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_multiplicative_order_connector_residue_profile_target
                .as_deref(),
            Some(
                "replicate-orthogonal-multorder-gap4-connector-001139-mod29-residue-separator-before-lean"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_multiplicative_order_connector_residue_separator_replication_decision
                .as_deref(),
            Some("mod29-orthogonal-residue-separator-mutated-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_multiplicative_order_connector_residue_separator_replication_status
                .as_deref(),
            Some("split-exact-residue-separator-on-orthogonal-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_multiplicative_order_connector_residue_separator_replication_target
                .as_deref(),
            Some(
                "select-next-arithmetic-connector-family-after-multiplicative-order-three-ladder-collapse"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_automorphic_repunit_connector_control_decision
                .as_deref(),
            Some("orthogonal-automorphic-repunit-connector-control-survived-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_automorphic_repunit_connector_branch_id
                .as_deref(),
            Some("orthogonal-automorphic-repunit-gap4-width5-connector-91736")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_automorphic_repunit_connector_family
                .as_deref(),
            Some("cousin-prime-gap4")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_automorphic_repunit_connector_connector
                .as_deref(),
            Some("91736")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_automorphic_repunit_connector_target
                .as_deref(),
            Some("residue-profile-orthogonal-automorphic-repunit-gap4-connector-91736-width5")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_automorphic_repunit_connector_residue_profile_decision
                .as_deref(),
            Some("small-prime-orthogonal-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_automorphic_repunit_connector_residue_profile_best_modulus,
            Some(29)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_automorphic_repunit_connector_residue_profile_target
                .as_deref(),
            Some("replicate-orthogonal-automorphic-repunit-gap4-connector-91736-mod29-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_automorphic_repunit_connector_residue_separator_replication_decision
                .as_deref(),
            Some("mod29-orthogonal-residue-separator-collapsed-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_automorphic_repunit_connector_residue_separator_replication_status
                .as_deref(),
            Some("collapsed-overlapping-residue-classes-on-orthogonal-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_automorphic_repunit_connector_residue_separator_replication_target
                .as_deref(),
            Some(
                "select-next-arithmetic-connector-family-after-automorphic-repunit-three-ladder-collapse"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_cyclic_reptend_connector_control_decision
                .as_deref(),
            Some("orthogonal-cyclic-reptend-connector-control-survived-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_cyclic_reptend_connector_branch_id
                .as_deref(),
            Some("orthogonal-cyclic-reptend-gap6-width5-connector-53191")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_cyclic_reptend_connector_family
                .as_deref(),
            Some("sexy-prime-gap6")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_cyclic_reptend_connector_connector
                .as_deref(),
            Some("53191")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_cyclic_reptend_connector_target
                .as_deref(),
            Some("residue-profile-orthogonal-cyclic-reptend-gap6-connector-53191-width5")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_cyclic_reptend_connector_residue_profile_decision
                .as_deref(),
            Some("small-prime-orthogonal-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_cyclic_reptend_connector_residue_profile_best_modulus,
            Some(13)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_cyclic_reptend_connector_residue_profile_target
                .as_deref(),
            Some("replicate-orthogonal-cyclic-reptend-gap6-connector-53191-mod13-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_cyclic_reptend_connector_residue_separator_replication_decision
                .as_deref(),
            Some("mod13-orthogonal-residue-separator-mutated-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_cyclic_reptend_connector_residue_separator_replication_status
                .as_deref(),
            Some("split-exact-residue-separator-on-orthogonal-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_cyclic_reptend_connector_residue_separator_replication_target
                .as_deref(),
            Some(
                "select-next-arithmetic-connector-family-after-cyclic-reptend-three-ladder-collapse"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_carry_chain_connector_control_decision
                .as_deref(),
            Some("orthogonal-carry-chain-connector-control-survived-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_carry_chain_connector_branch_id
                .as_deref(),
            Some("orthogonal-carry-chain-gap4-width9-connector-900020000")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_carry_chain_connector_family
                .as_deref(),
            Some("cousin-prime-gap4")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_carry_chain_connector_connector
                .as_deref(),
            Some("900020000")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_carry_chain_connector_target
                .as_deref(),
            Some("residue-profile-orthogonal-carry-chain-gap4-connector-900020000-width9")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_carry_chain_connector_residue_profile_decision
                .as_deref(),
            Some("small-prime-orthogonal-residue-separator-found-replicate-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_carry_chain_connector_residue_profile_best_modulus,
            Some(7)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_carry_chain_connector_residue_profile_target
                .as_deref(),
            Some("replicate-orthogonal-carry-chain-gap4-connector-900020000-mod7-residue-separator-before-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_carry_chain_connector_residue_separator_replication_decision
                .as_deref(),
            Some("mod7-orthogonal-residue-separator-collapsed-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_carry_chain_connector_residue_separator_replication_status
                .as_deref(),
            Some("collapsed-no-reverse-only-on-orthogonal-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_carry_chain_connector_residue_separator_replication_target
                .as_deref(),
            Some("select-next-arithmetic-connector-family-after-carry-chain-three-ladder-collapse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_base_mixed_connector_control_decision
                .as_deref(),
            Some("orthogonal-base-mixed-connector-control-collapsed-retire-without-lean")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_base_mixed_connector_control_target
                .as_deref(),
            Some("select-next-connector-surface-after-base-mixed-source-fresh-collapse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_base_mixed_connector_branch_id
                .as_deref(),
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_orthogonal_base_mixed_connector_connector
                .as_deref(),
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_base_mixed_connector_residue_profile_decision
                .as_deref(),
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .orthogonal_base_mixed_connector_residue_separator_replication_decision
                .as_deref(),
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .connector_stress_meta_atlas_decision
                .as_deref(),
            Some("retired-connector-value-branches-recorded-select-pair-family-gap-portfolio")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .connector_stress_meta_atlas_retired_count,
            Some(18)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .connector_stress_meta_atlas_selected_surface
                .as_deref(),
            Some("pair-family-gap-portfolio")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_portfolio_control_decision
                .as_deref(),
            Some("pair-family-gap-portfolio-control-survived-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_pair_family_gap_portfolio_branch_id
                .as_deref(),
            Some("pair-family-gap-portfolio-gap8-width6-connector-003727")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_pair_family_gap_portfolio_family
                .as_deref(),
            Some("prime-gap8")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_pair_family_gap_portfolio_connector
                .as_deref(),
            Some("003727")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_portfolio_residue_profile_best_modulus,
            Some(23)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_portfolio_residue_separator_replication_decision
                .as_deref(),
            Some("mod23-orthogonal-residue-separator-mutated-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_portfolio_residue_separator_replication_status
                .as_deref(),
            Some("split-exact-residue-separator-on-orthogonal-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_extension_control_decision
                .as_deref(),
            Some("pair-family-gap-extension-control-survived-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_pair_family_gap_extension_branch_id
                .as_deref(),
            Some("pair-family-gap-extension-gap16-width6-connector-276061")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_pair_family_gap_extension_family
                .as_deref(),
            Some("prime-gap16")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_pair_family_gap_extension_connector
                .as_deref(),
            Some("276061")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_extension_residue_profile_best_modulus,
            Some(19)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_extension_residue_separator_replication_decision
                .as_deref(),
            Some("mod19-orthogonal-residue-separator-mutated-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_extension_residue_separator_replication_status
                .as_deref(),
            Some("split-exact-residue-separator-on-orthogonal-separator-ladder")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_size_band_control_decision
                .as_deref(),
            Some("pair-family-size-band-control-survived-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_pair_family_size_band_branch_id
                .as_deref(),
            Some("pair-family-size-band-prime-gap8-size120k-gap8-width5-connector-91736")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_pair_family_size_band_family
                .as_deref(),
            Some("prime-gap8-size120k")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_pair_family_size_band_connector
                .as_deref(),
            Some("91736")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_size_band_residue_profile_best_modulus,
            Some(11)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_size_band_residue_separator_replication_decision
                .as_deref(),
            Some("mod11-orthogonal-residue-separator-collapsed-retire-branch")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_size_band_residue_separator_replication_status
                .as_deref(),
            Some("collapsed-overlapping-residue-classes-on-orthogonal-separator-ladder")
        );
        assert_eq!(
            catalog.rows[5].next_theorem_target.as_deref(),
            Some("select-new-cohort-invariant-surface-after-surface-agnostic-ensemble-picker-collapse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .replication_null_atlas_schema_version,
            "connector-replication-null-atlas-v1"
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .replication_null_atlas_status,
            "not-stable-under-current-three-ladder-gate"
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .replication_null_atlas_retained_separator_count,
            0
        );
        assert!(
            catalog
                .connector_digit8_classifier_family
                .replication_null_atlas_split_separator_count
                > 0
        );
        assert!(
            catalog
                .connector_digit8_classifier_family
                .replication_null_atlas_collapsed_separator_count
                > 0
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .replication_null_atlas_theorem_candidate_count,
            0
        );
        assert_eq!(
            catalog.rows[6].artifact_path,
            "docs/connector/connector_replication_null_atlas.json"
        );
        assert_eq!(
            catalog.rows[6].next_theorem_target.as_deref(),
            Some("select-new-cohort-invariant-surface-after-surface-agnostic-ensemble-picker-collapse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .replication_null_atlas_next_target,
            "select-new-cohort-invariant-surface-after-surface-agnostic-ensemble-picker-collapse"
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_cohort_retention_picker_decision
                .as_deref(),
            Some(
                "single-branch-separators-not-stable-select-cohort-level-pair-family-side-control"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_pair_family_cohort_id
                .as_deref(),
            Some("pair-family-cohort-width5-connector-91736")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_pair_family_cohort_connector
                .as_deref(),
            Some("91736")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_pair_family_cohort_target
                .as_deref(),
            Some(
                "cohort-residue-profile-width5-connector-91736-across-related-pair-family-ladders"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_cohort_residue_profile_decision
                .as_deref(),
            Some("no-small-prime-cohort-residue-separator-found")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_cohort_residue_profile_exact_separator_count,
            Some(0)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_cohort_residue_profile_best_modulus,
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_cohort_residue_profile_target
                .as_deref(),
            Some(
                "select-new-family-level-replication-surface-after-cohort-91736-profile-no-small-prime-separator"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_surface_picker_decision
                .as_deref(),
            Some("cohort-connector-profile-failed-select-family-level-pair-surface")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_pair_family_surface_id
                .as_deref(),
            Some("pair-family-size-band")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_pair_family_surface_label
                .as_deref(),
            Some("gap-8 size-band controls")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .selected_pair_family_surface_target
                .as_deref(),
            Some("surface-residue-profile-pair-family-size-band-across-selected-branches")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_surface_residue_profile_decision
                .as_deref(),
            Some("no-small-prime-family-surface-residue-separator-found")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_surface_residue_profile_exact_separator_count,
            Some(0)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_surface_residue_profile_best_modulus,
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_surface_residue_profile_target
                .as_deref(),
            Some(
                "select-new-family-level-replication-surface-after-pair-family-size-band-profile-no-small-prime-separator"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_topn_motif_surface_profile_decision
                .as_deref(),
            Some("no-small-prime-topn-family-surface-residue-separator-found")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_topn_motif_surface_profile_top_n,
            Some(3)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_topn_motif_surface_profile_source_motif_count,
            Some(9)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_topn_motif_surface_profile_fresh_survivor_count,
            Some(8)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_topn_motif_surface_profile_exact_separator_count,
            Some(0)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_topn_motif_surface_profile_best_modulus,
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_topn_motif_surface_profile_target
                .as_deref(),
            Some(
                "select-new-family-level-replication-surface-after-topn-pair-family-size-band-profile-no-small-prime-separator"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_geometry_control_decision
                .as_deref(),
            Some("pair-family-gap-cohort-geometry-retained-residue-profiler-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_geometry_control_top_n,
            Some(10)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_geometry_control_source_motif_count,
            Some(21)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_geometry_control_retained_geometry_count,
            Some(2)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_geometry_control_selected_connector
                .as_deref(),
            Some("0286717")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_geometry_control_target
                .as_deref(),
            Some("residue-profile-gap-cohort-width7-connector-0286717-across-gap20-22-24")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_residue_profile_decision
                .as_deref(),
            Some("no-small-prime-gap-cohort-residue-separator-found")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_residue_profile_exact_separator_count,
            Some(0)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_residue_profile_best_modulus,
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_residue_profile_target
                .as_deref(),
            Some(
                "select-new-family-level-replication-surface-after-gap-cohort-0286717-profile-no-small-prime-separator"
            )
        );
        assert!(catalog
            .connector_digit8_classifier_family
            .pair_family_gap_cohort_residue_separator_replication_decision
            .is_none());
        assert!(catalog
            .connector_digit8_classifier_family
            .pair_family_gap_cohort_residue_separator_replication_status
            .is_none());
        assert!(catalog
            .connector_digit8_classifier_family
            .pair_family_gap_cohort_residue_separator_replication_target
            .is_none());
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_control_decision
                .as_deref(),
            Some("pair-family-gap-cohort-ratio-geometry-retained-replicate-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_control_selected_connector
                .as_deref(),
            Some("001139")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_control_selected_bias
                .as_deref(),
            Some("reverse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_control_target
                .as_deref(),
            Some(
                "replicate-gap-cohort-ratio-geometry-width6-connector-001139-bias-reverse-on-separator-ladders"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_replication_decision
                .as_deref(),
            Some("gap-cohort-ratio-geometry-retained-expand-empirical-surface")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_replication_status
                .as_deref(),
            Some("retained-source-ratio-geometry-on-gap-cohort-separator-ladders")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_replication_target
                .as_deref(),
            Some(
                "expand-gap-cohort-ratio-geometry-width6-connector-001139-bias-reverse-on-new-gap-ladders"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_expansion_decision
                .as_deref(),
            Some("gap-cohort-ratio-geometry-mixed-retained-test-correction-bounds-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_expansion_status
                .as_deref(),
            Some("mixed-retained-ratio-geometry-on-new-gap-ladders")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_expansion_target
                .as_deref(),
            Some(
                "test-correction-bound-stability-gap-cohort-ratio-geometry-width6-connector-001139"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_correction_bound_stability_decision
                .as_deref(),
            Some("correction-bound-stability-retained-expand-ratio-geometry-atlas-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_correction_bound_stability_status
                .as_deref(),
            Some("stable-reverse-correction-bound-geometry-across-default-bounds")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_correction_bound_stable_bound_count,
            Some(9)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_correction_bound_stability_target
                .as_deref(),
            Some("expand-ratio-geometry-001139-to-size-band-and-gap-band-controls")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_atlas_decision
                .as_deref(),
            Some("ratio-geometry-001139-collapsed-as-family-level-invariant-record-falsification")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_atlas_status
                .as_deref(),
            Some("ratio-geometry-001139-not-stable-across-size-and-gap-bands")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_atlas_target
                .as_deref(),
            Some("select-new-cohort-invariant-after-ratio-geometry-001139-size-gap-atlas-collapse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_picker_decision
                .as_deref(),
            Some("frozen-portfolio-cohort-invariant-picker-selected-stable-ratio-geometry")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_picker_stable_candidate_count,
            Some(6)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_picker_selected_connector
                .as_deref(),
            Some("003727")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_picker_selected_direction
                .as_deref(),
            Some("reverse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_picker_target
                .as_deref(),
            Some("residue-profile-cohort-invariant-width6-connector-003727-direction-reverse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_residue_profile_decision
                .as_deref(),
            Some("cohort-invariant-residue-profile-no-small-prime-separator-record-falsification")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_residue_profile_status
                .as_deref(),
            Some("no-small-prime-cohort-invariant-residue-separator")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_residue_profile_best_modulus,
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_residue_profile_target
                .as_deref(),
            Some("select-new-cohort-invariant-after-003727-residue-profile-no-coherent-separator")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_next_picker_decision
                .as_deref(),
            Some("cohort-invariant-next-picker-selected-unprofiled-stable-ratio-geometry")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_next_picker_excluded_profile_count,
            Some(1)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_next_picker_selected_connector
                .as_deref(),
            Some("276061")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_next_picker_selected_direction
                .as_deref(),
            Some("reverse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_next_picker_target
                .as_deref(),
            Some("residue-profile-cohort-invariant-width6-connector-276061-direction-reverse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_next_residue_profile_decision
                .as_deref(),
            Some("cohort-invariant-residue-profile-no-small-prime-separator-record-falsification")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_next_residue_profile_status
                .as_deref(),
            Some("no-small-prime-cohort-invariant-residue-separator")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_next_residue_profile_best_modulus,
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_next_residue_profile_target
                .as_deref(),
            Some("select-new-cohort-invariant-after-276061-residue-profile-no-coherent-separator")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_post_two_null_picker_decision
                .as_deref(),
            Some("cohort-invariant-post-two-null-picker-selected-unprofiled-stable-ratio-geometry")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_post_two_null_picker_excluded_profile_count,
            Some(2)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_post_two_null_picker_selected_connector
                .as_deref(),
            Some("91736")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_post_two_null_picker_selected_direction
                .as_deref(),
            Some("reverse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_post_two_null_picker_target
                .as_deref(),
            Some("residue-profile-cohort-invariant-width5-connector-91736-direction-reverse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile_decision
                .as_deref(),
            Some("cohort-invariant-residue-profile-no-small-prime-separator-record-falsification")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile_status
                .as_deref(),
            Some("no-small-prime-cohort-invariant-residue-separator")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile_best_modulus,
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_post_two_null_residue_profile_target
                .as_deref(),
            Some("select-new-cohort-invariant-after-91736-residue-profile-no-coherent-separator")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_three_null_conclusion_decision
                .as_deref(),
            Some(
                "pivot-to-forward-stable-ratio-cohort-candidates-after-three-reverse-residue-nulls"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_three_null_conclusion_status
                .as_deref(),
            Some(
                "reverse-stable-ratio-cohort-residue-route-collapsed-under-small-prime-exact-mask-rule"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_three_null_conclusion_collapsed_profile_count,
            Some(3)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_three_null_conclusion_selected_connector
                .as_deref(),
            Some("900020000")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_three_null_conclusion_selected_direction
                .as_deref(),
            Some("forward")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_three_null_conclusion_target
                .as_deref(),
            Some("residue-profile-cohort-invariant-width9-connector-900020000-direction-forward")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_forward_residue_profile_decision
                .as_deref(),
            Some("cohort-invariant-residue-profile-no-small-prime-separator-record-falsification")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_forward_residue_profile_status
                .as_deref(),
            Some("no-small-prime-cohort-invariant-residue-separator")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_forward_residue_profile_best_modulus,
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_forward_residue_profile_target
                .as_deref(),
            Some(
                "select-new-cohort-invariant-after-900020000-residue-profile-no-coherent-separator"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_decision
                .as_deref(),
            Some("pivot-to-new-cohort-invariant-surface-after-forward-residue-null")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_status
                .as_deref(),
            Some(
                "forward-stable-ratio-cohort-residue-route-collapsed-under-small-prime-exact-mask-rule"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_collapsed_profile_count,
            Some(1)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_remaining_candidate_count,
            Some(2)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_ratio_geometry_forward_null_conclusion_target
                .as_deref(),
            Some(
                "select-new-cohort-invariant-surface-after-forward-route-small-prime-exact-mask-null"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_window_consensus_surface_decision
                .as_deref(),
            Some("window-consensus-surface-selected-shared-stage-complete-invariant")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_window_consensus_surface_candidate_count,
            Some(14)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_window_consensus_surface_selected_connector
                .as_deref(),
            Some("003727")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_window_consensus_surface_selected_direction
                .as_deref(),
            Some("reverse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_window_consensus_surface_selected_status
                .as_deref(),
            Some("shared-stage-complete-window-consensus")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_window_consensus_surface_selected_consensus_window_count,
            Some(10)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_window_consensus_surface_target
                .as_deref(),
            Some(
                "stress-test-window-consensus-cohort-invariant-width6-connector-003727-direction-reverse"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_window_consensus_stress_status
                .as_deref(),
            Some("window-consensus-collapsed-on-heldout-gap-size-surfaces")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_window_consensus_stress_decision
                .as_deref(),
            Some("window-consensus-heldout-stress-collapsed-record-falsification")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_window_consensus_stress_retained_surface_count,
            Some(0)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_window_consensus_stress_split_surface_count,
            Some(0)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_window_consensus_stress_collapsed_surface_count,
            Some(2)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_window_consensus_stress_target
                .as_deref(),
            Some("select-new-cohort-invariant-surface-after-window-consensus-heldout-collapse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_sign_persistence_picker_decision
                .as_deref(),
            Some("sign-persistence-picker-selected-low-volatility-cohort-invariant")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_sign_persistence_picker_candidate_count,
            Some(14)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_sign_persistence_picker_persistent_candidate_count,
            Some(4)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_sign_persistence_picker_selected_connector
                .as_deref(),
            Some("003727")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_sign_persistence_picker_selected_direction
                .as_deref(),
            Some("reverse")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_sign_persistence_picker_selected_status
                .as_deref(),
            Some("surface-sign-persistent-cohort-invariant")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_sign_persistence_picker_selected_surface_count,
            Some(4)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_sign_persistence_picker_selected_volatility_score,
            Some(6)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_sign_persistence_picker_target
                .as_deref(),
            Some(
                "stress-test-sign-persistence-cohort-invariant-width6-connector-003727-direction-reverse"
            )
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_sign_persistence_stress_status
                .as_deref(),
            Some("sign-persistence-split-on-fresh-surfaces")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_sign_persistence_stress_decision
                .as_deref(),
            Some("sign-persistence-fresh-stress-split-record-falsification")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_sign_persistence_stress_retained_surface_count,
            Some(0)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_sign_persistence_stress_split_surface_count,
            Some(2)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_sign_persistence_stress_neutral_surface_count,
            Some(0)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_sign_persistence_stress_retained_window_count,
            Some(6)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_sign_persistence_stress_opposite_window_count,
            Some(8)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_sign_persistence_stress_target
                .as_deref(),
            Some("select-new-cohort-invariant-surface-after-sign-persistence-fresh-split")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_volatility_ensemble_picker_decision
                .as_deref(),
            Some("volatility-ensemble-picker-selected-shared-direction-cohort")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_volatility_ensemble_picker_ensemble_count,
            Some(2)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_volatility_ensemble_picker_qualifying_ensemble_count,
            Some(2)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_volatility_ensemble_picker_selected_direction
                .as_deref(),
            Some("forward")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_volatility_ensemble_picker_selected_connector_count,
            Some(14)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_volatility_ensemble_picker_selected_supported_surface_count,
            Some(6)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_volatility_ensemble_picker_target
                .as_deref(),
            Some("stress-test-volatility-ensemble-cohort-direction-forward-connector-count-14")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_volatility_ensemble_stress_status
                .as_deref(),
            Some("volatility-ensemble-split-on-fresh-surfaces")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_volatility_ensemble_stress_decision
                .as_deref(),
            Some("volatility-ensemble-fresh-stress-split-record-falsification")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_volatility_ensemble_stress_selected_direction
                .as_deref(),
            Some("forward")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_volatility_ensemble_stress_selected_connector_count,
            Some(14)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_volatility_ensemble_stress_retained_surface_count,
            Some(0)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_volatility_ensemble_stress_mixed_retained_surface_count,
            Some(0)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_volatility_ensemble_stress_split_surface_count,
            Some(2)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_volatility_ensemble_stress_collapsed_surface_count,
            Some(0)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_volatility_ensemble_stress_retained_window_count,
            Some(86)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_volatility_ensemble_stress_opposite_window_count,
            Some(102)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_volatility_ensemble_stress_target
                .as_deref(),
            Some("select-new-cohort-invariant-surface-after-volatility-ensemble-fresh-split")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_picker_decision
                .as_deref(),
            Some("surface-family-contrast-picker-selected-directional-gap-size-contrast")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_picker_status
                .as_deref(),
            Some("surface-family-directional-contrast-found")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_picker_selected_family
                .as_deref(),
            Some("gap-family")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_picker_opposite_family
                .as_deref(),
            Some("size-family")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_picker_retained_family_count,
            Some(1)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_picker_split_family_count,
            Some(1)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_picker_target
                .as_deref(),
            Some("stress-test-surface-family-contrast-gap-family-forward-vs-size-family-opposite")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_stress_status
                .as_deref(),
            Some("surface-family-contrast-retained-on-fresh-surfaces")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_stress_decision
                .as_deref(),
            Some("surface-family-contrast-fresh-stress-retained-anatomy-next")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_stress_selected_family
                .as_deref(),
            Some("gap-family")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_stress_opposite_family
                .as_deref(),
            Some("size-family")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_stress_retained_family_count,
            Some(1)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_stress_split_family_count,
            Some(1)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_stress_retained_window_count,
            Some(82)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_stress_opposite_window_count,
            Some(86)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_stress_target
                .as_deref(),
            Some("analyze-surface-family-contrast-anatomy-gap-family-forward-vs-size-family-opposite")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_anatomy_concentration_status
                .as_deref(),
            Some("distributed-full-driver-cohort")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_anatomy_decision
                .as_deref(),
            Some("surface-family-contrast-anatomy-found-distributed-driver-cohort")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_anatomy_full_driver_count,
            Some(4)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_anatomy_gap_only_driver_count,
            Some(2)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_anatomy_size_only_driver_count,
            Some(4)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_anatomy_top_driver_share_basis_points,
            Some(1591)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_contrast_anatomy_target
                .as_deref(),
            Some("stress-test-surface-family-contrast-driver-cohort-distributed-gap-family-forward-vs-size-family-opposite")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_driver_cohort_stress_status
                .as_deref(),
            Some("driver-cohort-contrast-split-on-fresh-surfaces")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_driver_cohort_stress_decision
                .as_deref(),
            Some("driver-cohort-contrast-fresh-stress-split-record-falsification")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_driver_cohort_stress_driver_count,
            Some(4)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_driver_cohort_stress_retained_family_count,
            Some(0)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_driver_cohort_stress_split_family_count,
            Some(2)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_driver_cohort_stress_retained_window_count,
            Some(22)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_driver_cohort_stress_opposite_window_count,
            Some(34)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_driver_cohort_stress_target
                .as_deref(),
            Some("select-new-cohort-invariant-surface-after-driver-cohort-contrast-fresh-split")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_status
                .as_deref(),
            Some("matched-nondriver-control-split-on-fresh-surfaces")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_decision
                .as_deref(),
            Some("matched-nondriver-control-also-split-record-broad-surface-falsification")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_control_count,
            Some(4)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_retained_family_count,
            Some(0)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_split_family_count,
            Some(1)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_retained_window_count,
            Some(19)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_opposite_window_count,
            Some(27)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_family_matched_nondriver_control_stress_target
                .as_deref(),
            Some("select-new-cohort-invariant-surface-after-driver-and-nondriver-fresh-split")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_agnostic_ensemble_picker_decision
                .as_deref(),
            Some("surface-agnostic-ensemble-picker-found-no-stable-mixed-surface-cohort")
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_agnostic_ensemble_picker_candidate_count,
            Some(14)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_agnostic_ensemble_picker_stable_connector_count,
            Some(4)
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_agnostic_ensemble_picker_selected_direction
                .as_deref(),
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_agnostic_ensemble_picker_selected_connector_count,
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_agnostic_ensemble_picker_selected_supported_surface_count,
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_agnostic_ensemble_picker_retained_window_count,
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_agnostic_ensemble_picker_opposite_window_count,
            None
        );
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .pair_family_gap_cohort_surface_agnostic_ensemble_picker_target
                .as_deref(),
            Some("select-new-cohort-invariant-surface-after-surface-agnostic-ensemble-picker-collapse")
        );
        assert_eq!(catalog.connector_digit8_classifier_family.cells.len(), 3);
        assert_eq!(
            catalog
                .connector_digit8_classifier_family
                .cells
                .iter()
                .map(|cell| (
                    cell.edge.as_str(),
                    cell.width,
                    cell.connector.as_str(),
                    cell.outside_ladder_reverse_only_pair_count,
                    cell.outside_ladder_retained_modulus_count,
                    cell.outside_ladder_split_modulus_count,
                    cell.outside_ladder_collapsed_modulus_count,
                    cell.outside_ladder_cell_status.as_str()
                ))
                .collect::<Vec<_>>(),
            vec![
                (
                    "leading",
                    6,
                    "800000",
                    0,
                    0,
                    0,
                    4,
                    "collapsed-at-some-source-modulus-outside-ladder"
                ),
                (
                    "trailing",
                    6,
                    "000008",
                    0,
                    0,
                    0,
                    4,
                    "collapsed-at-some-source-modulus-outside-ladder"
                ),
                (
                    "trailing",
                    5,
                    "00008",
                    2,
                    0,
                    2,
                    1,
                    "collapsed-at-some-source-modulus-outside-ladder"
                )
            ]
        );
    }

    #[test]
    fn signal_catalog_json_round_trips() {
        let catalog = build_signal_catalog();
        let json = serde_json::to_string_pretty(&catalog).expect("serialize catalog");
        let decoded: SignalCatalog = serde_json::from_str(&json).expect("decode catalog");
        assert_eq!(decoded, catalog);
    }

    #[test]
    fn signal_catalog_markdown_is_an_index_not_a_claim() {
        let catalog = build_signal_catalog();
        let markdown = render_signal_catalog_markdown(&catalog);

        assert!(markdown.contains("research-instrument surface"));
        assert!(markdown.contains("not a combined density claim"));
        assert!(markdown.contains("connector-signal-atlas"));
        assert!(markdown.contains("connector-replication-null-atlas"));
        assert!(markdown.contains("Connector Digit-8 Classifier Family"));
        assert!(markdown.contains("complete-visible-digit8-exact-separator-family"));
        assert!(markdown.contains("Unclassified exact separators: `0`"));
        assert!(markdown.contains(
            "Outside-ladder replication: `digit8-classifier-family-partly-collapses-on-outside-ladder`"
        ));
        assert!(markdown.contains("Outside-ladder cells retained/split/collapsed: `0` / `0` / `3`"));
        assert!(markdown
            .contains("Split follow-up: `split-signal-partly-collapses-on-second-outside-ladder`"));
        assert!(markdown
            .contains("Split follow-up rows stabilized/split-again/collapsed: `0` / `0` / `2`"));
        assert!(markdown
            .contains("Branch picker: `all-branches-collapsed-after-independent-mod3-guardrail`"));
        assert!(markdown.contains("Selected next branch: `none` status `none` target `none`"));
        assert!(markdown.contains(
            "Independent branch replication: `retired-all-fresh-independent-rows-theorem-blocked-by-mod3-null-layer` retired branch `trailing-edge-width8-digit6-connector-00000006` next target `select-new-non-mod3-connector-stress-family-after-00000006-retirement`"
        ));
        assert!(markdown.contains(
            "Replication null atlas: schema `connector-replication-null-atlas-v1` status `not-stable-under-current-three-ladder-gate`"
        ));
        assert!(markdown.contains(
            "Non-mod3 candidate picker: `fresh-nonmod3-candidate-selected-after-retiring-collapsed-separator` selected `trailing-edge-width7-digit1-connector-0000001` target `independently-replicate-nonmod3-0000001-trailing-edge-width7-digit1`"
        ));
        assert!(markdown.contains(
            "Retired non-mod3 candidates: `1` `[\"trailing-edge-width7-digit7-connector-0000007\"]`"
        ));
        assert!(markdown.contains(
            "Non-mod3 second replication: `survived-second-independent-ladder-residue-profiler-next` target `residue-profile-nonmod3-0000001-trailing-edge-width7-digit1`"
        ));
        assert!(markdown.contains(
            "Non-mod3 residue profile: `small-prime-residue-separator-found-replicate-before-lean` best modulus `17` target `replicate-nonmod3-0000001-mod17-residue-separator-before-lean`"
        ));
        assert!(markdown.contains(
            "Non-mod3 residue-separator replication: `mod17-residue-separator-split-keep-empirical` status `split-exact-residue-separator-on-third-ladder` target `replicate-mutated-nonmod3-0000001-mod17-residue-separator-before-lean`"
        ));
        assert!(markdown.contains(
            "Non-mod3 mutated residue-separator replication: `mod17-mutated-residue-separator-collapsed-retire-branch` status `collapsed-no-reverse-only-on-fourth-ladder` target `select-next-nonmod3-connector-stress-family-after-0000001-retirement`"
        ));
        assert!(markdown.contains(
            "Next non-mod3 candidate picker: `fresh-nonmod3-candidate-selected-after-retiring-collapsed-separator` selected `trailing-edge-width8-digit5-connector-00000005` target `independently-replicate-nonmod3-00000005-trailing-edge-width8-digit5`"
        ));
        assert!(markdown.contains(
            "Next non-mod3 independent replication: `collapsed-on-second-independent-ladder-retire-without-lean` target `retire-nonmod3-00000005-trailing-edge-width8-digit5`"
        ));
        assert!(markdown.contains(
            "Retired non-mod3 edge candidates: `3` `[\"trailing-edge-width7-digit1-connector-0000001\", \"trailing-edge-width7-digit7-connector-0000007\", \"trailing-edge-width8-digit5-connector-00000005\"]`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 family picker: `interior-nonmod3-family-selected-for-independent-replication` selected `interior-width5-position3-digit1-connector-00010` target `independently-replicate-interior-nonmod3-00010-width5-position3-digit1`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 independent replication: `survived-interior-independent-ladder-residue-profiler-next` target `residue-profile-interior-nonmod3-00010-width5-position3-digit1`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 residue profile: `small-prime-residue-separator-found-replicate-before-lean` best modulus `19` target `replicate-interior-nonmod3-00010-mod19-residue-separator-before-lean`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 residue-separator replication: `mod19-interior-residue-separator-mutated-retire-branch` status `split-exact-residue-separator-on-interior-separator-ladder` target `retire-interior-nonmod3-00010-mod19-residue-separator-after-mutation`"
        ));
        assert!(markdown.contains(
            "Retired interior non-mod3 candidates: `7` `[\"interior-width5-position1-digit4-connector-04000\", \"interior-width5-position1-digit5-connector-05000\", \"interior-width5-position3-digit1-connector-00010\", \"interior-width7-position5-digit7-connector-0000070\", \"interior-width9-position4-digit5-connector-000050000\", \"interior-width9-position5-digit7-connector-000007000\", \"interior-width9-position7-digit7-connector-000000070\"]`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 next family picker: `interior-nonmod3-family-selected-for-independent-replication` selected `interior-width7-position5-digit7-connector-0000070` target `independently-replicate-interior-nonmod3-0000070-width7-position5-digit7`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 next independent replication: `survived-interior-independent-ladder-residue-profiler-next` target `residue-profile-interior-nonmod3-0000070-width7-position5-digit7`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 next residue profile: `small-prime-residue-separator-found-replicate-before-lean` best modulus `17` target `replicate-interior-nonmod3-0000070-mod17-residue-separator-before-lean`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 next residue-separator replication: `mod17-interior-residue-separator-collapsed-retire-branch` status `collapsed-overlapping-residue-classes-on-interior-separator-ladder` target `retire-interior-nonmod3-0000070-mod17-residue-separator-after-mutation`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 post-retirement family picker: `interior-nonmod3-family-selected-for-independent-replication` selected `interior-width5-position1-digit4-connector-04000` target `independently-replicate-interior-nonmod3-04000-width5-position1-digit4`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 post-retirement independent replication: `survived-interior-independent-ladder-residue-profiler-next` target `residue-profile-interior-nonmod3-04000-width5-position1-digit4`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 post-retirement residue profile: `small-prime-residue-separator-found-replicate-before-lean` best modulus `17` target `replicate-interior-nonmod3-04000-mod17-residue-separator-before-lean`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 post-retirement residue-separator replication: `mod17-interior-residue-separator-mutated-retire-branch` status `split-exact-residue-separator-on-interior-separator-ladder` target `retire-interior-nonmod3-04000-mod17-residue-separator-after-mutation`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 after third-retirement family picker: `interior-nonmod3-family-selected-for-independent-replication` selected `interior-width5-position1-digit5-connector-05000` target `independently-replicate-interior-nonmod3-05000-width5-position1-digit5`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 after third-retirement independent replication: `survived-interior-independent-ladder-residue-profiler-next` target `residue-profile-interior-nonmod3-05000-width5-position1-digit5`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 after third-retirement residue profile: `small-prime-residue-separator-found-replicate-before-lean` best modulus `11` target `replicate-interior-nonmod3-05000-mod11-residue-separator-before-lean`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 after third-retirement residue-separator replication: `mod11-interior-residue-separator-collapsed-retire-branch` status `collapsed-no-reverse-only-on-interior-separator-ladder` target `retire-interior-nonmod3-05000-mod11-residue-separator-after-mutation`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 after fourth-retirement family picker: `interior-nonmod3-family-selected-for-independent-replication` selected `interior-width9-position4-digit5-connector-000050000` target `independently-replicate-interior-nonmod3-000050000-width9-position4-digit5`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 after fourth-retirement independent replication: `survived-interior-independent-ladder-residue-profiler-next` target `residue-profile-interior-nonmod3-000050000-width9-position4-digit5`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 after fourth-retirement residue profile: `small-prime-residue-separator-found-replicate-before-lean` best modulus `13` target `replicate-interior-nonmod3-000050000-mod13-residue-separator-before-lean`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 after fourth-retirement residue-separator replication: `mod13-interior-residue-separator-collapsed-retire-branch` status `collapsed-no-reverse-only-on-interior-separator-ladder` target `retire-interior-nonmod3-000050000-mod13-residue-separator-after-mutation`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 after fifth-retirement family picker: `interior-nonmod3-family-selected-for-independent-replication` selected `interior-width9-position5-digit7-connector-000007000` target `independently-replicate-interior-nonmod3-000007000-width9-position5-digit7`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 after fifth-retirement independent replication: `collapsed-interior-independent-ladder-retire-without-lean` target `retire-interior-nonmod3-000007000-width9-position5-digit7`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 after sixth-retirement family picker: `interior-nonmod3-family-selected-for-independent-replication` selected `interior-width9-position7-digit7-connector-000000070` target `independently-replicate-interior-nonmod3-000000070-width9-position7-digit7`"
        ));
        assert!(markdown.contains(
            "Interior non-mod3 after sixth-retirement independent replication: `collapsed-interior-independent-ladder-retire-without-lean` target `retire-interior-nonmod3-000000070-width9-position7-digit7`"
        ));
        assert!(markdown.contains(
            "Single-digit interior pivot decision: `pivot-away-from-single-digit-interior-family-after-repeated-fresh-ladder-collapse`"
        ));
        assert!(markdown.contains(
            "Multi-digit motif picker: `multi-digit-motif-selected-for-independent-replication` selected `multidigit-motif-width5-start1-digits11-connector-01100` target `independently-replicate-multidigit-motif-01100-width5-start1-digits11`"
        ));
        assert!(markdown.contains(
            "Multi-digit motif independent replication: `survived-multidigit-motif-independent-ladder-residue-profiler-next` target `residue-profile-multidigit-motif-01100-width5-start1-digits11`"
        ));
        assert!(markdown.contains(
            "Multi-digit motif residue profile: `small-prime-residue-separator-found-replicate-before-lean` best modulus `11` target `replicate-multidigit-motif-01100-mod11-residue-separator-before-lean`"
        ));
        assert!(markdown.contains(
            "Multi-digit motif residue-separator replication: `mod11-multidigit-residue-separator-collapsed-retire-branch` status `collapsed-no-reverse-only-on-multidigit-separator-ladder` target `retire-multidigit-motif-01100-mod11-residue-separator-after-mutation`"
        ));
        assert!(markdown.contains(
            "Orthogonal carry-chain connector control: `orthogonal-carry-chain-connector-control-survived-residue-profiler-next` selected `orthogonal-carry-chain-gap4-width9-connector-900020000` family `cousin-prime-gap4` connector `900020000` target `residue-profile-orthogonal-carry-chain-gap4-connector-900020000-width9`"
        ));
        assert!(markdown.contains(
            "Orthogonal carry-chain connector residue profile: `small-prime-orthogonal-residue-separator-found-replicate-before-lean` best modulus `7` target `replicate-orthogonal-carry-chain-gap4-connector-900020000-mod7-residue-separator-before-lean`"
        ));
        assert!(markdown.contains(
            "Orthogonal carry-chain connector residue-separator replication: `mod7-orthogonal-residue-separator-collapsed-retire-branch` status `collapsed-no-reverse-only-on-orthogonal-separator-ladder` target `select-next-arithmetic-connector-family-after-carry-chain-three-ladder-collapse`"
        ));
        assert!(markdown.contains(
            "Orthogonal base-mixed connector control: `orthogonal-base-mixed-connector-control-collapsed-retire-without-lean` selected `none` family `none` connector `none` target `none` control target `select-next-connector-surface-after-base-mixed-source-fresh-collapse`"
        ));
        assert!(markdown.contains(
            "Orthogonal base-mixed connector residue profile: `none` best modulus `none` target `none`"
        ));
        assert!(markdown.contains(
            "Orthogonal base-mixed connector residue-separator replication: `none` status `none` target `none`"
        ));
        assert!(markdown.contains(
            "Connector stress meta-atlas: `retired-connector-value-branches-recorded-select-pair-family-gap-portfolio` retired `18` selected surface `pair-family-gap-portfolio` target `scan-pair-family-gap-portfolio-over-retired-connector-heads`"
        ));
        assert!(markdown.contains(
            "Pair-family gap portfolio control: `pair-family-gap-portfolio-control-survived-residue-profiler-next` selected `pair-family-gap-portfolio-gap8-width6-connector-003727` family `prime-gap8` connector `003727` target `residue-profile-pair-family-gap-portfolio-gap8-connector-003727-width6` control target `residue-profile-pair-family-gap-portfolio-gap8-connector-003727-width6`"
        ));
        assert!(markdown.contains(
            "Pair-family gap portfolio residue profile: `small-prime-orthogonal-residue-separator-found-replicate-before-lean` best modulus `23` target `replicate-pair-family-gap-portfolio-gap8-connector-003727-mod23-residue-separator-before-lean`"
        ));
        assert!(markdown.contains(
            "Pair-family gap portfolio residue-separator replication: `mod23-orthogonal-residue-separator-mutated-retire-branch` status `split-exact-residue-separator-on-orthogonal-separator-ladder` target `select-new-pair-family-side-control-after-gap-portfolio-three-ladder-collapse`"
        ));
        assert!(markdown.contains(
            "Pair-family gap extension control: `pair-family-gap-extension-control-survived-residue-profiler-next` selected `pair-family-gap-extension-gap16-width6-connector-276061` family `prime-gap16` connector `276061` target `residue-profile-pair-family-gap-extension-gap16-connector-276061-width6` control target `residue-profile-pair-family-gap-extension-gap16-connector-276061-width6`"
        ));
        assert!(markdown.contains(
            "Pair-family gap extension residue profile: `small-prime-orthogonal-residue-separator-found-replicate-before-lean` best modulus `19` target `replicate-pair-family-gap-extension-gap16-connector-276061-mod19-residue-separator-before-lean`"
        ));
        assert!(markdown.contains(
            "Pair-family gap extension residue-separator replication: `mod19-orthogonal-residue-separator-mutated-retire-branch` status `split-exact-residue-separator-on-orthogonal-separator-ladder` target `select-new-pair-family-side-control-after-gap-extension-three-ladder-collapse`"
        ));
        assert!(markdown.contains(
            "Pair-family size-band control: `pair-family-size-band-control-survived-residue-profiler-next` selected `pair-family-size-band-prime-gap8-size120k-gap8-width5-connector-91736` family `prime-gap8-size120k` connector `91736` target `residue-profile-pair-family-size-band-prime-gap8-size120k-connector-91736-width5` control target `residue-profile-pair-family-size-band-prime-gap8-size120k-connector-91736-width5`"
        ));
        assert!(markdown.contains(
            "Pair-family size-band residue profile: `small-prime-orthogonal-residue-separator-found-replicate-before-lean` best modulus `11` target `replicate-pair-family-size-band-prime-gap8-size120k-connector-91736-mod11-residue-separator-before-lean`"
        ));
        assert!(markdown.contains(
            "Pair-family size-band residue-separator replication: `mod11-orthogonal-residue-separator-collapsed-retire-branch` status `collapsed-overlapping-residue-classes-on-orthogonal-separator-ladder` target `select-new-pair-family-side-control-after-size-band-three-ladder-collapse`"
        ));
        assert!(markdown.contains(
            "Pair-family cohort retention picker: `single-branch-separators-not-stable-select-cohort-level-pair-family-side-control` selected `pair-family-cohort-width5-connector-91736` connector `91736` target `cohort-residue-profile-width5-connector-91736-across-related-pair-family-ladders`"
        ));
        assert!(markdown.contains(
            "Pair-family cohort residue profile: `no-small-prime-cohort-residue-separator-found` exact separators `0` best modulus `none` target `select-new-family-level-replication-surface-after-cohort-91736-profile-no-small-prime-separator`"
        ));
        assert!(markdown.contains(
            "Pair-family surface picker: `cohort-connector-profile-failed-select-family-level-pair-surface` selected `pair-family-size-band` label `gap-8 size-band controls` target `surface-residue-profile-pair-family-size-band-across-selected-branches`"
        ));
        assert!(markdown.contains(
            "Pair-family surface residue profile: `no-small-prime-family-surface-residue-separator-found` exact separators `0` best modulus `none` target `select-new-family-level-replication-surface-after-pair-family-size-band-profile-no-small-prime-separator`"
        ));
        assert!(markdown.contains(
            "Pair-family top-N motif surface profile: `no-small-prime-topn-family-surface-residue-separator-found` top-N `3` motifs `9` fresh survivors `8` exact separators `0` best modulus `none` target `select-new-family-level-replication-surface-after-topn-pair-family-size-band-profile-no-small-prime-separator`"
        ));
        assert!(markdown.contains(
            "Pair-family gap cohort geometry control: `pair-family-gap-cohort-geometry-retained-residue-profiler-next` top-N `10` motifs `21` retained geometry `2` selected connector `0286717` target `residue-profile-gap-cohort-width7-connector-0286717-across-gap20-22-24`"
        ));
        assert!(markdown.contains(
            "Pair-family gap cohort residue profile: `no-small-prime-gap-cohort-residue-separator-found` exact separators `0` best modulus `none` target `select-new-family-level-replication-surface-after-gap-cohort-0286717-profile-no-small-prime-separator`"
        ));
        assert!(markdown.contains(
            "Pair-family gap cohort separator replication: `none` status `none` target `none`"
        ));
        assert!(markdown.contains(
            "Pair-family gap cohort ratio geometry control: `pair-family-gap-cohort-ratio-geometry-retained-replicate-next` connector `001139` bias `reverse` target `replicate-gap-cohort-ratio-geometry-width6-connector-001139-bias-reverse-on-separator-ladders`"
        ));
        assert!(markdown.contains(
            "Pair-family gap cohort ratio geometry replication: `gap-cohort-ratio-geometry-retained-expand-empirical-surface` status `retained-source-ratio-geometry-on-gap-cohort-separator-ladders` target `expand-gap-cohort-ratio-geometry-width6-connector-001139-bias-reverse-on-new-gap-ladders`"
        ));
        assert!(markdown.contains(
            "Pair-family gap cohort ratio geometry expansion: `gap-cohort-ratio-geometry-mixed-retained-test-correction-bounds-next` status `mixed-retained-ratio-geometry-on-new-gap-ladders` target `test-correction-bound-stability-gap-cohort-ratio-geometry-width6-connector-001139`"
        ));
        assert!(markdown.contains(
            "Pair-family gap cohort ratio correction-bound stability: `correction-bound-stability-retained-expand-ratio-geometry-atlas-next` status `stable-reverse-correction-bound-geometry-across-default-bounds` stable bounds `9` target `expand-ratio-geometry-001139-to-size-band-and-gap-band-controls`"
        ));
        assert!(markdown.contains(
            "Pair-family gap cohort ratio geometry atlas: `ratio-geometry-001139-collapsed-as-family-level-invariant-record-falsification` status `ratio-geometry-001139-not-stable-across-size-and-gap-bands` target `select-new-cohort-invariant-after-ratio-geometry-001139-size-gap-atlas-collapse`"
        ));
        assert!(markdown.contains(
            "Frozen portfolio cohort-invariant picker: `frozen-portfolio-cohort-invariant-picker-selected-stable-ratio-geometry` stable candidates `6` selected connector `003727` direction `reverse` target `residue-profile-cohort-invariant-width6-connector-003727-direction-reverse`"
        ));
        assert!(markdown.contains(
            "Cohort-invariant residue profile: `cohort-invariant-residue-profile-no-small-prime-separator-record-falsification` status `no-small-prime-cohort-invariant-residue-separator` best modulus `none` target `select-new-cohort-invariant-after-003727-residue-profile-no-coherent-separator`"
        ));
        assert!(markdown.contains(
            "Next cohort-invariant picker: `cohort-invariant-next-picker-selected-unprofiled-stable-ratio-geometry` excluded profiles `1` selected connector `276061` direction `reverse` target `residue-profile-cohort-invariant-width6-connector-276061-direction-reverse`"
        ));
        assert!(markdown.contains(
            "Next cohort-invariant residue profile: `cohort-invariant-residue-profile-no-small-prime-separator-record-falsification` status `no-small-prime-cohort-invariant-residue-separator` best modulus `none` target `select-new-cohort-invariant-after-276061-residue-profile-no-coherent-separator`"
        ));
        assert!(markdown.contains(
            "Post-two-null cohort-invariant picker: `cohort-invariant-post-two-null-picker-selected-unprofiled-stable-ratio-geometry` excluded profiles `2` selected connector `91736` direction `reverse` target `residue-profile-cohort-invariant-width5-connector-91736-direction-reverse`"
        ));
        assert!(markdown.contains(
            "Post-two-null cohort-invariant residue profile: `cohort-invariant-residue-profile-no-small-prime-separator-record-falsification` status `no-small-prime-cohort-invariant-residue-separator` best modulus `none` target `select-new-cohort-invariant-after-91736-residue-profile-no-coherent-separator`"
        ));
        assert!(markdown.contains(
            "Three-null cohort-invariant conclusion: `pivot-to-forward-stable-ratio-cohort-candidates-after-three-reverse-residue-nulls` status `reverse-stable-ratio-cohort-residue-route-collapsed-under-small-prime-exact-mask-rule` collapsed profiles `3` selected connector `900020000` direction `forward` target `residue-profile-cohort-invariant-width9-connector-900020000-direction-forward`"
        ));
        assert!(markdown.contains(
            "Forward cohort-invariant residue profile: `cohort-invariant-residue-profile-no-small-prime-separator-record-falsification` status `no-small-prime-cohort-invariant-residue-separator` best modulus `none` target `select-new-cohort-invariant-after-900020000-residue-profile-no-coherent-separator`"
        ));
        assert!(markdown.contains(
            "Forward-route cohort-invariant conclusion: `pivot-to-new-cohort-invariant-surface-after-forward-residue-null` status `forward-stable-ratio-cohort-residue-route-collapsed-under-small-prime-exact-mask-rule` collapsed profiles `1` remaining candidates `2` target `select-new-cohort-invariant-surface-after-forward-route-small-prime-exact-mask-null`"
        ));
        assert!(markdown.contains(
            "Window-consensus cohort-invariant surface: `window-consensus-surface-selected-shared-stage-complete-invariant` candidates `14` selected connector `003727` direction `reverse` status `shared-stage-complete-window-consensus` consensus windows `10` target `stress-test-window-consensus-cohort-invariant-width6-connector-003727-direction-reverse`"
        ));
        assert!(markdown.contains(
            "Window-consensus held-out stress: `window-consensus-heldout-stress-collapsed-record-falsification` status `window-consensus-collapsed-on-heldout-gap-size-surfaces` retained/split/collapsed `0`/`0`/`2` target `select-new-cohort-invariant-surface-after-window-consensus-heldout-collapse`"
        ));
        assert!(markdown.contains(
            "Sign-persistence cohort-invariant picker: `sign-persistence-picker-selected-low-volatility-cohort-invariant` candidates `14` persistent `4` selected connector `003727` direction `reverse` status `surface-sign-persistent-cohort-invariant` surfaces `4` volatility `6` target `stress-test-sign-persistence-cohort-invariant-width6-connector-003727-direction-reverse`"
        ));
        assert!(markdown.contains(
            "Sign-persistence fresh stress: `sign-persistence-split-on-fresh-surfaces` decision `sign-persistence-fresh-stress-split-record-falsification` retained/split/neutral surfaces `0/2/0` retained/opposite windows `6/8` target `select-new-cohort-invariant-surface-after-sign-persistence-fresh-split`"
        ));
        assert!(markdown.contains(
            "Volatility/ensemble cohort picker: `volatility-ensemble-picker-selected-shared-direction-cohort` ensembles `2` qualifying `2` selected direction `forward` connectors `14` surfaces `6` target `stress-test-volatility-ensemble-cohort-direction-forward-connector-count-14`"
        ));
        assert!(markdown.contains(
            "Volatility/ensemble cohort stress: `volatility-ensemble-split-on-fresh-surfaces` decision `volatility-ensemble-fresh-stress-split-record-falsification` direction `forward` connectors `14` retained/mixed/split/collapsed surfaces `0/0/2/0` retained/opposite windows `86/102` target `select-new-cohort-invariant-surface-after-volatility-ensemble-fresh-split`"
        ));
        assert!(markdown.contains(
            "Surface-family contrast picker: `surface-family-contrast-picker-selected-directional-gap-size-contrast` status `surface-family-directional-contrast-found` selected family `gap-family` opposite family `size-family` retained/split families `1/1` target `stress-test-surface-family-contrast-gap-family-forward-vs-size-family-opposite`"
        ));
        assert!(markdown.contains(
            "Surface-family contrast stress: `surface-family-contrast-retained-on-fresh-surfaces` decision `surface-family-contrast-fresh-stress-retained-anatomy-next` selected family `gap-family` opposite family `size-family` retained/split families `1/1` retained/opposite windows `82/86` target `analyze-surface-family-contrast-anatomy-gap-family-forward-vs-size-family-opposite`"
        ));
        assert!(markdown.contains(
            "Surface-family contrast anatomy: `distributed-full-driver-cohort` decision `surface-family-contrast-anatomy-found-distributed-driver-cohort` full/gap-only/size-only drivers `4/2/4` top-share-bp `1591` target `stress-test-surface-family-contrast-driver-cohort-distributed-gap-family-forward-vs-size-family-opposite`"
        ));
        assert!(markdown.contains(
            "Surface-family driver-cohort stress: `driver-cohort-contrast-split-on-fresh-surfaces` decision `driver-cohort-contrast-fresh-stress-split-record-falsification` drivers `4` retained/split families `0/2` retained/opposite windows `22/34` target `select-new-cohort-invariant-surface-after-driver-cohort-contrast-fresh-split`"
        ));
        assert!(markdown.contains(
            "Surface-family matched non-driver control stress: `matched-nondriver-control-split-on-fresh-surfaces` decision `matched-nondriver-control-also-split-record-broad-surface-falsification` controls `4` retained/split families `0/1` retained/opposite windows `19/27` target `select-new-cohort-invariant-surface-after-driver-and-nondriver-fresh-split`"
        ));
        assert!(markdown.contains(
            "Surface-agnostic ensemble picker: `surface-agnostic-ensemble-picker-found-no-stable-mixed-surface-cohort` candidates `14` stable connectors `4` selected direction `none` connectors `none` stable-surface total `none` retained/opposite windows `none/none` target `select-new-cohort-invariant-surface-after-surface-agnostic-ensemble-picker-collapse`"
        ));
        assert!(markdown.contains("collapsed-at-some-source-modulus-outside-ladder"));
        assert!(markdown.contains("digit8LeadingWidth6_reverseOnly_multiModulusClassifier"));
        assert!(markdown.contains("digit8TrailingWidth6_reverseOnly_multiModulusClassifier"));
        assert!(markdown.contains("digit8TrailingWidth5_reverseOnly_multiModulusClassifier"));
        assert!(markdown.contains("not a connector law or prime-density mechanism"));
    }

    #[test]
    fn signal_catalog_verifier_accepts_known_existing_rows() {
        let catalog = build_signal_catalog();
        let root = temp_catalog_root("valid");
        materialize_catalog_artifacts(&root, &catalog);

        let verification = verify_signal_catalog(&catalog, &root);

        assert!(verification.ok);
        assert_eq!(verification.checked_rows, catalog.rows.len());
        assert!(verification.failures.is_empty());

        fs::remove_dir_all(root).expect("cleanup temp dir");
    }

    #[test]
    fn signal_catalog_verifier_rejects_missing_artifacts_and_unknown_commands() {
        let mut catalog = build_signal_catalog();
        let root = temp_catalog_root("invalid");
        materialize_catalog_artifacts(&root, &catalog);
        catalog.rows[0].artifact_path = "docs/missing/not_here.json".to_string();
        catalog.rows[1].drift_check_command = "scripts/unknown.sh verify".to_string();
        catalog.rows[2].drift_check_command = " ".to_string();

        let verification = verify_signal_catalog(&catalog, &root);

        assert!(!verification.ok);
        assert_eq!(verification.checked_rows, catalog.rows.len());
        assert!(verification.failures.iter().any(|failure| {
            failure.signal_id == "matched-control-smoke-atlas"
                && failure.field == "artifact_path"
                && failure.message == "artifact_path does not exist"
        }));
        assert!(verification.failures.iter().any(|failure| {
            failure.signal_id == "witness-search-policy-atlas"
                && failure.field == "drift_check_command"
                && failure.message == "drift_check_command is not in the maintained allow-list"
        }));
        assert!(verification.failures.iter().any(|failure| {
            failure.signal_id == "witness-lean-catalog"
                && failure.field == "drift_check_command"
                && failure.message == "drift_check_command must be nonempty"
        }));

        fs::remove_dir_all(root).expect("cleanup temp dir");
    }

    #[test]
    fn deep_signal_catalog_verifier_records_passing_gate() {
        let root = temp_catalog_root("deep-pass");
        let catalog = single_row_catalog();
        materialize_catalog_artifacts(&root, &catalog);
        write_gate_script(&root, "scripts/connector_signal_atlas.sh", "exit 0\n");

        let verification = verify_signal_catalog_deep(&catalog, &root, Duration::from_secs(2));

        assert!(verification.ok);
        assert_eq!(verification.gate_results.len(), 1);
        assert_eq!(verification.gate_results[0].status, "passed");
        assert_eq!(verification.gate_results[0].exit_code, Some(0));

        fs::remove_dir_all(root).expect("cleanup temp dir");
    }

    #[test]
    fn deep_signal_catalog_verifier_records_failing_gate() {
        let root = temp_catalog_root("deep-fail");
        let catalog = single_row_catalog();
        materialize_catalog_artifacts(&root, &catalog);
        write_gate_script(&root, "scripts/connector_signal_atlas.sh", "exit 7\n");

        let verification = verify_signal_catalog_deep(&catalog, &root, Duration::from_secs(2));

        assert!(!verification.ok);
        assert_eq!(verification.gate_results.len(), 1);
        assert_eq!(verification.gate_results[0].status, "failed");
        assert_eq!(verification.gate_results[0].exit_code, Some(7));

        fs::remove_dir_all(root).expect("cleanup temp dir");
    }

    #[test]
    fn deep_signal_catalog_verifier_records_timed_out_gate() {
        let root = temp_catalog_root("deep-timeout");
        let catalog = single_row_catalog();
        materialize_catalog_artifacts(&root, &catalog);
        write_gate_script(&root, "scripts/connector_signal_atlas.sh", "exec sleep 2\n");

        let verification = verify_signal_catalog_deep(&catalog, &root, Duration::from_millis(100));

        assert!(!verification.ok);
        assert_eq!(verification.gate_results.len(), 1);
        assert_eq!(verification.gate_results[0].status, "timed-out");
        assert!(verification.gate_results[0].timed_out);

        fs::remove_dir_all(root).expect("cleanup temp dir");
    }

    fn materialize_catalog_artifacts(root: &Path, catalog: &SignalCatalog) {
        for row in &catalog.rows {
            let path = root.join(&row.artifact_path);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("create artifact parent");
            }
            fs::write(path, "{}\n").expect("write artifact placeholder");
        }
    }

    fn temp_catalog_root(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "primes-signal-catalog-{label}-{}-{nanos}",
            process::id()
        ))
    }

    fn single_row_catalog() -> SignalCatalog {
        let rows = vec![SignalCatalogRow {
            signal_id: "connector-signal-atlas".to_string(),
            domain: "connector".to_string(),
            artifact_path: "docs/connector/connector_signal_atlas.json".to_string(),
            drift_check_command: "scripts/connector_signal_atlas.sh verify".to_string(),
            claim_status: "test claim".to_string(),
            proof_status: "test proof".to_string(),
            empirical_status: "test empirical".to_string(),
            next_theorem_target: None,
        }];
        SignalCatalog {
            schema_version: SIGNAL_CATALOG_SCHEMA_VERSION.to_string(),
            artifact_id: SIGNAL_CATALOG_ARTIFACT_ID.to_string(),
            generator_command: SIGNAL_CATALOG_GENERATOR_COMMAND.to_string(),
            drift_check_command: SIGNAL_CATALOG_DRIFT_CHECK_COMMAND.to_string(),
            summary: signal_catalog_summary(&rows),
            connector_digit8_classifier_family: build_connector_digit8_classifier_family_summary(),
            rows,
        }
    }

    fn write_gate_script(root: &Path, relative_path: &str, body: &str) {
        let path = root.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create script parent");
        }
        fs::write(&path, format!("#!/bin/sh\n{body}")).expect("write gate script");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = fs::metadata(&path).expect("script metadata").permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(&path, permissions).expect("chmod gate script");
        }
    }
}
