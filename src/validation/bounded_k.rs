//! Shared bounded-`k` membrane scans for cross-base and anomaly reports.
//!
//! This lane studies a fixed small `k` grid:
//! - `k=(0,0)`
//! - `k=(0,1)`
//! - `k=(1,0)`
//! - `k=(1,1)`
//! - `k=(2,2)`
//!
//! The helpers here keep exact prime counting, smoke-pair sampling, and
//! summary logic consistent across maintained and exploratory reports.

use crate::is_prime;
use num_bigint::BigUint;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

pub type BoundedKConfig = (u32, u32);

pub const DEFAULT_BOUNDED_K_GRID: &[BoundedKConfig] = &[(0, 0), (0, 1), (1, 0), (1, 1), (2, 2)];
pub const DEFAULT_PREFILTER_PRIMES: &[u32] = &[3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
pub const HINGE_CATEGORY_PERSISTENT_CORE: &str = "persistent_core";
pub const HINGE_CATEGORY_PERSISTENCE_ONLY: &str = "persistence_only";
pub const HINGE_CATEGORY_CORE_ONLY: &str = "core_only";
pub const HINGE_CATEGORY_ACTIVE_NEITHER: &str = "active_neither";

#[derive(Debug, Clone, Serialize)]
pub struct KDominancePairRow {
    pub base: u32,
    pub middle_length: usize,
    pub outer: u32,
    pub inner: u32,
    pub pair_label: String,
    pub candidates_per_config: usize,
    pub prime_hits_k00: usize,
    pub prime_hits_k01: usize,
    pub prime_hits_k10: usize,
    pub prime_hits_k11: usize,
    pub prime_hits_k22: usize,
    pub rate_k00: f64,
    pub rate_k01: f64,
    pub rate_k10: f64,
    pub rate_k11: f64,
    pub rate_k22: f64,
    pub best_k: String,
    pub best_prime_hits: usize,
    pub best_rate: f64,
    pub k00_strict_best: bool,
    pub k00_tied_best: bool,
    pub k00_noninferior: bool,
    pub best_minus_k00_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct KDominanceSummaryRow {
    pub base: u32,
    pub middle_length: usize,
    pub ordered_pair_count: usize,
    pub k00_strict_best_pairs: usize,
    pub k00_tied_best_pairs: usize,
    pub k00_noninferior_pairs: usize,
    pub k00_best_share: f64,
    pub k00_noninferior_share: f64,
    pub median_k00_advantage_pp: f64,
    pub strongest_counterexample_pair: String,
    pub strongest_counterexample_best_k: String,
    pub strongest_counterexample_margin_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct KConfigPrimeExample {
    pub middle_index: u32,
    pub middle_digits: String,
    pub decimal_value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct KConfigModulusDivisibilityRow {
    pub modulus: u32,
    pub divisible_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct KConfigProfile {
    pub candidates_per_config: usize,
    pub admissible_count: usize,
    pub prime_hits: usize,
    pub modulus_divisibility_rows: Vec<KConfigModulusDivisibilityRow>,
}

#[derive(Debug, Clone, Serialize)]
pub struct KConfigCandidateMaskRow {
    pub middle_index: u32,
    pub middle_digits: String,
    pub decimal_value: String,
    pub divisibility_mask: u16,
    pub mask_label: String,
    pub admissible: bool,
    pub prime: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct KConfigMaskHistogramRow {
    pub divisibility_mask: u16,
    pub mask_label: String,
    pub count: usize,
    pub prime_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct KConfigMaskProfile {
    pub candidates_per_config: usize,
    pub admissible_count: usize,
    pub prime_hits: usize,
    pub modulus_divisibility_rows: Vec<KConfigModulusDivisibilityRow>,
    pub mask_histogram_rows: Vec<KConfigMaskHistogramRow>,
    pub candidate_rows: Vec<KConfigCandidateMaskRow>,
}

#[derive(Debug, Clone, Serialize)]
pub struct KConfigTransferRow {
    pub middle_index: u32,
    pub middle_digits: String,
    pub decimal_value_from: String,
    pub decimal_value_to: String,
    pub divisibility_mask_from: u16,
    pub mask_label_from: String,
    pub divisibility_mask_to: u16,
    pub mask_label_to: String,
    pub admissible_from: bool,
    pub admissible_to: bool,
    pub prime_from: bool,
    pub prime_to: bool,
    pub transfer_bucket: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct KConfigTransferHistogramRow {
    pub divisibility_mask_from: u16,
    pub mask_label_from: String,
    pub divisibility_mask_to: u16,
    pub mask_label_to: String,
    pub transfer_bucket: String,
    pub count: usize,
    pub prime_count_from: usize,
    pub prime_count_to: usize,
    pub prime_delta_count: isize,
}

#[derive(Debug, Clone, Serialize)]
pub struct KConfigTransferProfile {
    pub candidates_per_config: usize,
    pub transfer_histogram_rows: Vec<KConfigTransferHistogramRow>,
    pub candidate_rows: Vec<KConfigTransferRow>,
}

#[derive(Debug, Clone, Serialize)]
pub struct KConfigResidueProfileModulusRow {
    pub modulus: u32,
    pub excluded_seed_classes: Vec<u32>,
    pub excluded_seed_class_count: usize,
    pub excluded_seed_class_label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct KConfigResidueProfile {
    pub base: u32,
    pub middle_length: usize,
    pub outer: u32,
    pub inner: u32,
    pub k_label: String,
    pub candidates_per_config: usize,
    pub compared_moduli_count: usize,
    pub all_singleton_profiles: bool,
    pub modulus_rows: Vec<KConfigResidueProfileModulusRow>,
}

#[derive(Debug, Clone, Serialize)]
pub struct KConfigAffineModulusRow {
    pub modulus: u32,
    pub shift_modulus: u32,
    pub gradient_modulus: u32,
    pub zero_seed_class: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct KConfigAffineProfile {
    pub base: u32,
    pub middle_length: usize,
    pub outer: u32,
    pub inner: u32,
    pub k_label: String,
    pub candidates_per_config: usize,
    pub compared_moduli_count: usize,
    pub modulus_rows: Vec<KConfigAffineModulusRow>,
}

#[derive(Debug, Clone, Serialize)]
pub struct KConfigLaneProfileComparisonModulusRow {
    pub modulus: u32,
    pub excluded_seed_classes_from: Vec<u32>,
    pub excluded_seed_class_label_from: String,
    pub excluded_seed_classes_to: Vec<u32>,
    pub excluded_seed_class_label_to: String,
    pub profile_agreement: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct KConfigLaneProfileComparison {
    pub base: u32,
    pub middle_length: usize,
    pub outer: u32,
    pub inner: u32,
    pub pair_label: String,
    pub from_k: String,
    pub to_k: String,
    pub candidates_per_config: usize,
    pub compared_moduli_count: usize,
    pub all_singleton_profiles: bool,
    pub profile_agreement: bool,
    pub admissible_set_equal: bool,
    pub no_positive_admissible_delta: bool,
    pub admissible_delta_count: isize,
    pub theorem_rung_label: String,
    pub stable_zero_count: usize,
    pub gain_zero_count: usize,
    pub loss_zero_count: usize,
    pub stable_nonzero_count: usize,
    pub nonzero_churn_count: usize,
    pub modulus_rows: Vec<KConfigLaneProfileComparisonModulusRow>,
}

#[derive(Debug, Clone, Serialize)]
pub struct KConfigAffineLaneComparisonModulusRow {
    pub modulus: u32,
    pub shift_modulus_from: u32,
    pub shift_modulus_to: u32,
    pub gradient_modulus_from: u32,
    pub gradient_modulus_to: u32,
    pub zero_seed_class_from: u32,
    pub zero_seed_class_to: u32,
    pub shift_equal: bool,
    pub gradient_equal: bool,
    pub zero_seed_equal: bool,
    pub local_relation_label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct KConfigAffineLaneComparison {
    pub base: u32,
    pub middle_length: usize,
    pub outer: u32,
    pub inner: u32,
    pub pair_label: String,
    pub from_k: String,
    pub to_k: String,
    pub candidates_per_config: usize,
    pub compared_moduli_count: usize,
    pub same_shift_count: usize,
    pub same_gradient_count: usize,
    pub same_zero_seed_count: usize,
    pub identity_count: usize,
    pub shift_only_count: usize,
    pub gradient_only_count: usize,
    pub shift_and_gradient_count: usize,
    pub same_shift_share: f64,
    pub same_gradient_share: f64,
    pub same_zero_seed_share: f64,
    pub identity_share: f64,
    pub shift_only_share: f64,
    pub gradient_only_share: f64,
    pub shift_and_gradient_share: f64,
    pub modulus_rows: Vec<KConfigAffineLaneComparisonModulusRow>,
}

#[derive(Debug, Clone, Serialize)]
pub struct KBestVsK00Decomposition {
    pub best_k: String,
    pub anomaly_mass_pp: f64,
    pub admissible_delta_pp: f64,
    pub stable_zero_prime_delta_pp: f64,
    pub boundary_prime_delta_pp: f64,
    pub shared_prime_rate_delta_pp: f64,
    pub signal_source_label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BestVsK00FeatureRow {
    pub base: u32,
    pub middle_length: usize,
    pub outer: u32,
    pub inner: u32,
    pub pair_label: String,
    pub same_digit: bool,
    pub unit_distance: usize,
    pub gap_bucket: String,
    pub candidates_per_config: usize,
    pub best_k: String,
    pub active: bool,
    pub anomaly_mass_pp: f64,
    pub admissible_delta_pp: f64,
    pub admissible_set_effect_pp: f64,
    pub prime_yield_effect_pp: f64,
    pub shared_admissible_count: usize,
    pub stable_zero_prime_delta_count: isize,
    pub boundary_prime_delta_count: isize,
    pub stable_zero_prime_delta_pp: f64,
    pub boundary_prime_delta_pp: f64,
    pub shared_prime_rate_k00_pp: f64,
    pub shared_prime_rate_best_pp: f64,
    pub shared_prime_rate_delta_pp: f64,
    pub stable_zero_signal_margin_count: isize,
    pub stable_zero_signal_margin_pp: f64,
    pub stable_zero_support_ratio: f64,
    pub mask_stability_share: f64,
    pub admissible_overlap_jaccard: f64,
    pub nonzero_churn_share: f64,
    pub stable_zero_count: usize,
    pub gain_zero_count: usize,
    pub loss_zero_count: usize,
    pub stable_nonzero_count: usize,
    pub nonzero_churn_count: usize,
    pub positive_shared_yield: bool,
    pub shared_yield_core: bool,
    pub signal_source_label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HingeFeatureRow {
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub pair_label: String,
    pub same_digit: bool,
    pub unit_distance: usize,
    pub gap_bucket: String,
    pub m1_active: bool,
    pub m2_active: bool,
    pub m1_to_m2_persistent: bool,
    pub m1_best_k: String,
    pub m2_best_k: String,
    pub m1_anomaly_mass_pp: f64,
    pub m2_anomaly_mass_pp: f64,
    pub m1_admissible_delta_pp: f64,
    pub m2_admissible_delta_pp: f64,
    pub m1_stable_zero_prime_delta_count: isize,
    pub m2_stable_zero_prime_delta_count: isize,
    pub m1_boundary_prime_delta_count: isize,
    pub m2_boundary_prime_delta_count: isize,
    pub m1_stable_zero_prime_delta_pp: f64,
    pub m2_stable_zero_prime_delta_pp: f64,
    pub m1_boundary_prime_delta_pp: f64,
    pub m2_boundary_prime_delta_pp: f64,
    pub m1_shared_prime_rate_delta_pp: f64,
    pub m2_shared_prime_rate_delta_pp: f64,
    pub m1_stable_zero_signal_margin_count: isize,
    pub m2_stable_zero_signal_margin_count: isize,
    pub m1_stable_zero_signal_margin_pp: f64,
    pub m2_stable_zero_signal_margin_pp: f64,
    pub m1_stable_zero_support_ratio: f64,
    pub m2_stable_zero_support_ratio: f64,
    pub m1_mask_stability_share: f64,
    pub m2_mask_stability_share: f64,
    pub m1_admissible_overlap_jaccard: f64,
    pub m2_admissible_overlap_jaccard: f64,
    pub m1_nonzero_churn_share: f64,
    pub m2_nonzero_churn_share: f64,
    pub m2_stable_zero_count: usize,
    pub m2_gain_zero_count: usize,
    pub m2_loss_zero_count: usize,
    pub m2_stable_nonzero_count: usize,
    pub m2_nonzero_churn_count: usize,
    pub m1_signal_source_label: String,
    pub m2_signal_source_label: String,
    pub shared_yield_core: bool,
    pub hinge_category: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffineHingeFeatureRow {
    #[serde(flatten)]
    pub hinge_row: HingeFeatureRow,
    pub m1_affine_compared_moduli_count: usize,
    pub m1_affine_same_shift_count: usize,
    pub m1_affine_same_gradient_count: usize,
    pub m1_affine_same_zero_seed_count: usize,
    pub m1_affine_identity_count: usize,
    pub m1_affine_shift_only_count: usize,
    pub m1_affine_gradient_only_count: usize,
    pub m1_affine_shift_and_gradient_count: usize,
    pub m1_affine_same_shift_share: f64,
    pub m1_affine_same_gradient_share: f64,
    pub m1_affine_same_zero_seed_share: f64,
    pub m1_affine_identity_share: f64,
    pub m1_affine_shift_only_share: f64,
    pub m1_affine_gradient_only_share: f64,
    pub m1_affine_shift_and_gradient_share: f64,
    pub m2_affine_compared_moduli_count: usize,
    pub m2_affine_same_shift_count: usize,
    pub m2_affine_same_gradient_count: usize,
    pub m2_affine_same_zero_seed_count: usize,
    pub m2_affine_identity_count: usize,
    pub m2_affine_shift_only_count: usize,
    pub m2_affine_gradient_only_count: usize,
    pub m2_affine_shift_and_gradient_count: usize,
    pub m2_affine_same_shift_share: f64,
    pub m2_affine_same_gradient_share: f64,
    pub m2_affine_same_zero_seed_share: f64,
    pub m2_affine_identity_share: f64,
    pub m2_affine_shift_only_share: f64,
    pub m2_affine_gradient_only_share: f64,
    pub m2_affine_shift_and_gradient_share: f64,
}

#[derive(Debug, Clone)]
struct ResidueFilterState {
    residue: u32,
    step_residue: u32,
    modulus: u32,
}

#[derive(Debug, Clone)]
struct KConfigScanSetup {
    candidate_base: BigUint,
    step: BigUint,
    candidates_per_config: usize,
    residue_filters: Vec<ResidueFilterState>,
}

pub fn digit_symbol(digit: u32) -> String {
    if digit < 10 {
        digit.to_string()
    } else {
        char::from_u32('A' as u32 + digit - 10)
            .expect("digit should fit uppercase alphabet")
            .to_string()
    }
}

pub fn format_k((k_outer, k_inner): BoundedKConfig) -> String {
    format!("k=({k_outer},{k_inner})")
}

pub fn parse_k_label(label: &str) -> BoundedKConfig {
    DEFAULT_BOUNDED_K_GRID
        .iter()
        .copied()
        .find(|&config| format_k(config) == label)
        .unwrap_or_else(|| panic!("unrecognized k label: {label}"))
}

pub fn unit_residues(base: u32) -> Vec<u32> {
    (1..base)
        .filter(|&digit| gcd_u32(digit, base) == 1)
        .collect()
}

pub fn ordered_unit_pairs(base: u32) -> Vec<(u32, u32)> {
    let units = unit_residues(base);
    units
        .iter()
        .copied()
        .flat_map(|outer| units.iter().copied().map(move |inner| (outer, inner)))
        .collect()
}

pub fn cyclic_unit_distance(base: u32, outer: u32, inner: u32) -> usize {
    let units = unit_residues(base);
    let outer_index = units
        .iter()
        .position(|&digit| digit == outer)
        .expect("outer digit should be a unit");
    let inner_index = units
        .iter()
        .position(|&digit| digit == inner)
        .expect("inner digit should be a unit");
    let direct = outer_index.abs_diff(inner_index);
    direct.min(units.len() - direct)
}

pub fn gap_bucket(base: u32, outer: u32, inner: u32) -> &'static str {
    match cyclic_unit_distance(base, outer, inner) {
        0 => "same",
        1 => "adjacent",
        _ => "wide",
    }
}

pub fn bounded_k_hinge_category(m2_persistent: bool, shared_yield_core: bool) -> &'static str {
    match (m2_persistent, shared_yield_core) {
        (true, true) => HINGE_CATEGORY_PERSISTENT_CORE,
        (true, false) => HINGE_CATEGORY_PERSISTENCE_ONLY,
        (false, true) => HINGE_CATEGORY_CORE_ONLY,
        (false, false) => HINGE_CATEGORY_ACTIVE_NEITHER,
    }
}

pub fn select_smoke_pairs(base: u32, max_pairs: usize, anchors: &[(u32, u32)]) -> Vec<(u32, u32)> {
    let all_pairs = ordered_unit_pairs(base);
    if all_pairs.len() <= max_pairs {
        return all_pairs;
    }

    let anchor_indices = anchors
        .iter()
        .filter_map(|&(outer, inner)| {
            all_pairs
                .iter()
                .position(|&(pair_outer, pair_inner)| pair_outer == outer && pair_inner == inner)
        })
        .collect::<BTreeSet<_>>();

    let mut selected = anchor_indices.iter().copied().collect::<Vec<_>>();
    let mut seen = anchor_indices;
    let evenly_spaced = (0..max_pairs)
        .map(|slot| ((slot * all_pairs.len()) + all_pairs.len() / 2) / max_pairs)
        .map(|index| index.min(all_pairs.len() - 1))
        .collect::<Vec<_>>();

    for index in evenly_spaced {
        if selected.len() >= max_pairs {
            break;
        }
        if seen.insert(index) {
            selected.push(index);
        }
    }

    if selected.len() < max_pairs {
        for index in 0..all_pairs.len() {
            if selected.len() >= max_pairs {
                break;
            }
            if seen.insert(index) {
                selected.push(index);
            }
        }
    }

    selected.sort_unstable();
    selected.into_iter().map(|index| all_pairs[index]).collect()
}

pub fn evaluate_pair_row(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    k_grid: &[BoundedKConfig],
) -> KDominancePairRow {
    assert!(
        k_grid == DEFAULT_BOUNDED_K_GRID,
        "evaluate_pair_row currently assumes the maintained five-lane k-grid",
    );

    let candidates_per_config = (base as usize).pow(middle_length as u32);
    let hits: Vec<_> = k_grid
        .iter()
        .map(|&(k_outer, k_inner)| {
            exact_prime_hits(base, middle_length, outer, inner, k_outer, k_inner)
        })
        .collect();
    let best_prime_hits = *hits.iter().max().expect("k-grid hits should not be empty");
    let best_index = hits
        .iter()
        .position(|&count| count == best_prime_hits)
        .expect("best hit index should exist");
    let prime_hits_k00 = hits[0];
    let k00_strict_best = prime_hits_k00 == best_prime_hits
        && hits
            .iter()
            .filter(|&&value| value == best_prime_hits)
            .count()
            == 1;
    let k00_tied_best = prime_hits_k00 == best_prime_hits && !k00_strict_best;
    let rate = |count: usize| count as f64 / candidates_per_config as f64;

    KDominancePairRow {
        base,
        middle_length,
        outer,
        inner,
        pair_label: format!("({},{})", digit_symbol(outer), digit_symbol(inner)),
        candidates_per_config,
        prime_hits_k00: hits[0],
        prime_hits_k01: hits[1],
        prime_hits_k10: hits[2],
        prime_hits_k11: hits[3],
        prime_hits_k22: hits[4],
        rate_k00: rate(hits[0]),
        rate_k01: rate(hits[1]),
        rate_k10: rate(hits[2]),
        rate_k11: rate(hits[3]),
        rate_k22: rate(hits[4]),
        best_k: format_k(k_grid[best_index]),
        best_prime_hits,
        best_rate: rate(best_prime_hits),
        k00_strict_best,
        k00_tied_best,
        k00_noninferior: prime_hits_k00 == best_prime_hits,
        best_minus_k00_pp: (best_prime_hits as f64 - prime_hits_k00 as f64) * 100.0
            / candidates_per_config as f64,
    }
}

pub fn summarize_pair_rows(rows: &[KDominancePairRow]) -> Vec<KDominanceSummaryRow> {
    let mut by_group: BTreeMap<(u32, usize), Vec<&KDominancePairRow>> = BTreeMap::new();
    for row in rows {
        by_group
            .entry((row.base, row.middle_length))
            .or_default()
            .push(row);
    }

    by_group
        .into_iter()
        .map(|((base, middle_length), group_rows)| {
            let ordered_pair_count = group_rows.len();
            let k00_strict_best_pairs = group_rows.iter().filter(|row| row.k00_strict_best).count();
            let k00_tied_best_pairs = group_rows.iter().filter(|row| row.k00_tied_best).count();
            let k00_noninferior_pairs = group_rows.iter().filter(|row| row.k00_noninferior).count();
            let strongest_counterexample = group_rows
                .iter()
                .max_by(|left, right| {
                    left.best_minus_k00_pp
                        .total_cmp(&right.best_minus_k00_pp)
                        .then_with(|| left.pair_label.cmp(&right.pair_label))
                })
                .expect("summary rows should have at least one pair");

            KDominanceSummaryRow {
                base,
                middle_length,
                ordered_pair_count,
                k00_strict_best_pairs,
                k00_tied_best_pairs,
                k00_noninferior_pairs,
                k00_best_share: (k00_strict_best_pairs + k00_tied_best_pairs) as f64
                    / ordered_pair_count as f64,
                k00_noninferior_share: k00_noninferior_pairs as f64 / ordered_pair_count as f64,
                median_k00_advantage_pp: median(
                    group_rows
                        .iter()
                        .map(|row| -row.best_minus_k00_pp)
                        .collect(),
                ),
                strongest_counterexample_pair: strongest_counterexample.pair_label.clone(),
                strongest_counterexample_best_k: strongest_counterexample.best_k.clone(),
                strongest_counterexample_margin_pp: strongest_counterexample.best_minus_k00_pp,
            }
        })
        .collect()
}

pub fn exact_prime_hits(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    k_outer: u32,
    k_inner: u32,
) -> usize {
    scan_k_config_examples(base, middle_length, outer, inner, (k_outer, k_inner), 0).0
}

pub fn scan_k_config_examples(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    (k_outer, k_inner): BoundedKConfig,
    example_limit: usize,
) -> (usize, Vec<KConfigPrimeExample>) {
    let setup = build_k_config_scan_setup(base, middle_length, outer, inner, (k_outer, k_inner));
    let mut residue_filters = setup.residue_filters;

    let mut hits = 0usize;
    let mut examples = Vec::new();
    for middle_index in 0..setup.candidates_per_config as u32 {
        let admissible = residue_filters.iter().all(|state| state.residue != 0);
        if admissible {
            let candidate = if middle_index == 0 {
                setup.candidate_base.clone()
            } else {
                &setup.candidate_base + (&setup.step * middle_index)
            };
            if is_prime(&candidate) {
                hits += 1;
                if examples.len() < example_limit {
                    examples.push(KConfigPrimeExample {
                        middle_index,
                        middle_digits: to_base_string_fixed(middle_index, base, middle_length),
                        decimal_value: candidate.to_string(),
                    });
                }
            }
        }
        for state in &mut residue_filters {
            state.residue += state.step_residue;
            state.residue %= state.modulus;
        }
    }

    (hits, examples)
}

pub fn scan_k_config_profile(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    (k_outer, k_inner): BoundedKConfig,
) -> KConfigProfile {
    let setup = build_k_config_scan_setup(base, middle_length, outer, inner, (k_outer, k_inner));
    let mut residue_filters = setup
        .residue_filters
        .into_iter()
        .map(|state| (state, 0usize))
        .collect::<Vec<_>>();

    let mut admissible_count = 0usize;
    let mut prime_hits = 0usize;
    for middle_index in 0..setup.candidates_per_config as u32 {
        let mut admissible = true;
        for (state, divisible_count) in &mut residue_filters {
            if state.residue == 0 {
                *divisible_count += 1;
                admissible = false;
            }
        }
        if admissible {
            admissible_count += 1;
            let candidate = if middle_index == 0 {
                setup.candidate_base.clone()
            } else {
                &setup.candidate_base + (&setup.step * middle_index)
            };
            if is_prime(&candidate) {
                prime_hits += 1;
            }
        }
        for (state, _) in &mut residue_filters {
            state.residue += state.step_residue;
            state.residue %= state.modulus;
        }
    }

    KConfigProfile {
        candidates_per_config: setup.candidates_per_config,
        admissible_count,
        prime_hits,
        modulus_divisibility_rows: residue_filters
            .into_iter()
            .map(|(state, divisible_count)| KConfigModulusDivisibilityRow {
                modulus: state.modulus,
                divisible_count,
            })
            .collect(),
    }
}

pub fn scan_k_config_mask_profile(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    (k_outer, k_inner): BoundedKConfig,
) -> KConfigMaskProfile {
    assert!(
        DEFAULT_PREFILTER_PRIMES.len() <= u16::BITS as usize,
        "mask profile requires the prefilter prime list to fit in a u16 bitmask",
    );

    let setup = build_k_config_scan_setup(base, middle_length, outer, inner, (k_outer, k_inner));
    let mut residue_filters = setup.residue_filters;
    let mut admissible_count = 0usize;
    let mut prime_hits = 0usize;
    let mut candidate_rows = Vec::with_capacity(setup.candidates_per_config);
    let mut mask_histogram = BTreeMap::<u16, (usize, usize)>::new();

    for middle_index in 0..setup.candidates_per_config as u32 {
        let mut divisibility_mask = 0u16;
        for (bit_index, state) in residue_filters.iter().enumerate() {
            if state.residue == 0 {
                divisibility_mask |= 1u16 << bit_index;
            }
        }
        let admissible = divisibility_mask == 0;
        let candidate = if middle_index == 0 {
            setup.candidate_base.clone()
        } else {
            &setup.candidate_base + (&setup.step * middle_index)
        };
        let prime = is_prime(&candidate);
        if admissible {
            admissible_count += 1;
            if prime {
                prime_hits += 1;
            }
        }

        candidate_rows.push(KConfigCandidateMaskRow {
            middle_index,
            middle_digits: to_base_string_fixed(middle_index, base, middle_length),
            decimal_value: candidate.to_string(),
            divisibility_mask,
            mask_label: render_divisibility_mask(divisibility_mask),
            admissible,
            prime,
        });
        let entry = mask_histogram
            .entry(divisibility_mask)
            .or_insert((0usize, 0usize));
        entry.0 += 1;
        if prime {
            entry.1 += 1;
        }

        for state in &mut residue_filters {
            state.residue += state.step_residue;
            state.residue %= state.modulus;
        }
    }

    let modulus_divisibility_rows = DEFAULT_PREFILTER_PRIMES
        .iter()
        .enumerate()
        .map(|(bit_index, &modulus)| KConfigModulusDivisibilityRow {
            modulus,
            divisible_count: candidate_rows
                .iter()
                .filter(|row| row.divisibility_mask & (1u16 << bit_index) != 0)
                .count(),
        })
        .collect::<Vec<_>>();

    let mask_histogram_rows = mask_histogram
        .into_iter()
        .map(
            |(divisibility_mask, (count, prime_count))| KConfigMaskHistogramRow {
                divisibility_mask,
                mask_label: render_divisibility_mask(divisibility_mask),
                count,
                prime_count,
            },
        )
        .collect::<Vec<_>>();

    KConfigMaskProfile {
        candidates_per_config: setup.candidates_per_config,
        admissible_count,
        prime_hits,
        modulus_divisibility_rows,
        mask_histogram_rows,
        candidate_rows,
    }
}

pub fn scan_k_config_transfer_profile(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    from_k: BoundedKConfig,
    to_k: BoundedKConfig,
) -> KConfigTransferProfile {
    let from_profile = scan_k_config_mask_profile(base, middle_length, outer, inner, from_k);
    let to_profile = if from_k == to_k {
        from_profile.clone()
    } else {
        scan_k_config_mask_profile(base, middle_length, outer, inner, to_k)
    };

    assert_eq!(
        from_profile.candidates_per_config, to_profile.candidates_per_config,
        "transfer profile requires matching candidate counts"
    );

    let mut candidate_rows = Vec::with_capacity(from_profile.candidates_per_config);
    let mut histogram = BTreeMap::<(u16, u16), (usize, usize, usize)>::new();

    for (from_row, to_row) in from_profile
        .candidate_rows
        .iter()
        .zip(&to_profile.candidate_rows)
    {
        assert_eq!(
            from_row.middle_index, to_row.middle_index,
            "transfer profile requires aligned middle indices"
        );
        assert_eq!(
            from_row.middle_digits, to_row.middle_digits,
            "transfer profile requires aligned middle digits"
        );

        let transfer_bucket =
            classify_transfer_bucket(from_row.divisibility_mask, to_row.divisibility_mask)
                .to_string();
        candidate_rows.push(KConfigTransferRow {
            middle_index: from_row.middle_index,
            middle_digits: from_row.middle_digits.clone(),
            decimal_value_from: from_row.decimal_value.clone(),
            decimal_value_to: to_row.decimal_value.clone(),
            divisibility_mask_from: from_row.divisibility_mask,
            mask_label_from: from_row.mask_label.clone(),
            divisibility_mask_to: to_row.divisibility_mask,
            mask_label_to: to_row.mask_label.clone(),
            admissible_from: from_row.admissible,
            admissible_to: to_row.admissible,
            prime_from: from_row.prime,
            prime_to: to_row.prime,
            transfer_bucket: transfer_bucket.clone(),
        });

        let entry = histogram
            .entry((from_row.divisibility_mask, to_row.divisibility_mask))
            .or_insert((0usize, 0usize, 0usize));
        entry.0 += 1;
        if from_row.prime {
            entry.1 += 1;
        }
        if to_row.prime {
            entry.2 += 1;
        }
    }

    let transfer_histogram_rows = histogram
        .into_iter()
        .map(
            |(
                (divisibility_mask_from, divisibility_mask_to),
                (count, prime_count_from, prime_count_to),
            )| {
                KConfigTransferHistogramRow {
                    divisibility_mask_from,
                    mask_label_from: render_divisibility_mask(divisibility_mask_from),
                    divisibility_mask_to,
                    mask_label_to: render_divisibility_mask(divisibility_mask_to),
                    transfer_bucket: classify_transfer_bucket(
                        divisibility_mask_from,
                        divisibility_mask_to,
                    )
                    .to_string(),
                    count,
                    prime_count_from,
                    prime_count_to,
                    prime_delta_count: prime_count_to as isize - prime_count_from as isize,
                }
            },
        )
        .collect::<Vec<_>>();

    KConfigTransferProfile {
        candidates_per_config: from_profile.candidates_per_config,
        transfer_histogram_rows,
        candidate_rows,
    }
}

pub fn coprime_prefilter_moduli(base: u32) -> Vec<u32> {
    DEFAULT_PREFILTER_PRIMES
        .iter()
        .copied()
        .filter(|&modulus| gcd_u32(base, modulus) == 1)
        .collect()
}

pub fn scan_k_config_residue_profile(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    k: BoundedKConfig,
) -> KConfigResidueProfile {
    let setup = build_k_config_scan_setup(base, middle_length, outer, inner, k);
    let modulus_rows = setup
        .residue_filters
        .iter()
        .filter(|state| gcd_u32(base, state.modulus) == 1)
        .map(|state| {
            let excluded_seed_classes =
                excluded_seed_classes_for_modulus(state.residue, state.step_residue, state.modulus);
            KConfigResidueProfileModulusRow {
                modulus: state.modulus,
                excluded_seed_class_count: excluded_seed_classes.len(),
                excluded_seed_class_label: render_seed_class_profile(&excluded_seed_classes),
                excluded_seed_classes,
            }
        })
        .collect::<Vec<_>>();

    KConfigResidueProfile {
        base,
        middle_length,
        outer,
        inner,
        k_label: format_k(k),
        candidates_per_config: setup.candidates_per_config,
        compared_moduli_count: modulus_rows.len(),
        all_singleton_profiles: modulus_rows
            .iter()
            .all(|row| row.excluded_seed_class_count == 1),
        modulus_rows,
    }
}

pub fn scan_k_config_affine_profile(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    k: BoundedKConfig,
) -> KConfigAffineProfile {
    let setup = build_k_config_scan_setup(base, middle_length, outer, inner, k);
    let modulus_rows = setup
        .residue_filters
        .iter()
        .filter(|state| gcd_u32(base, state.modulus) == 1)
        .map(|state| {
            let excluded_seed_classes =
                excluded_seed_classes_for_modulus(state.residue, state.step_residue, state.modulus);
            let zero_seed_class = *excluded_seed_classes
                .first()
                .expect("coprime affine residue profile should have a singleton zero-seed class");
            KConfigAffineModulusRow {
                modulus: state.modulus,
                shift_modulus: state.residue,
                gradient_modulus: state.step_residue,
                zero_seed_class,
            }
        })
        .collect::<Vec<_>>();

    KConfigAffineProfile {
        base,
        middle_length,
        outer,
        inner,
        k_label: format_k(k),
        candidates_per_config: setup.candidates_per_config,
        compared_moduli_count: modulus_rows.len(),
        modulus_rows,
    }
}

pub fn classify_theorem_rung(
    profile_agreement: bool,
    admissible_set_equal: bool,
    no_positive_admissible_delta: bool,
) -> &'static str {
    if profile_agreement {
        "profile_agreement"
    } else if admissible_set_equal {
        "admissible_equality_only"
    } else if no_positive_admissible_delta {
        "no_positive_admissible_delta_only"
    } else {
        "fails_all_three"
    }
}

pub fn scan_k_config_lane_profile_comparison(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    from_k: BoundedKConfig,
    to_k: BoundedKConfig,
) -> KConfigLaneProfileComparison {
    let from_profile = scan_k_config_residue_profile(base, middle_length, outer, inner, from_k);
    let to_profile = if from_k == to_k {
        from_profile.clone()
    } else {
        scan_k_config_residue_profile(base, middle_length, outer, inner, to_k)
    };
    assert_eq!(
        from_profile.compared_moduli_count, to_profile.compared_moduli_count,
        "lane profile comparison requires matching coprime-modulus families"
    );
    let transfer_profile =
        scan_k_config_transfer_profile(base, middle_length, outer, inner, from_k, to_k);
    let stable_zero_count = transfer_profile
        .candidate_rows
        .iter()
        .filter(|row| row.transfer_bucket == "stable_zero")
        .count();
    let gain_zero_count = transfer_profile
        .candidate_rows
        .iter()
        .filter(|row| row.transfer_bucket == "gain_zero")
        .count();
    let loss_zero_count = transfer_profile
        .candidate_rows
        .iter()
        .filter(|row| row.transfer_bucket == "loss_zero")
        .count();
    let stable_nonzero_count = transfer_profile
        .candidate_rows
        .iter()
        .filter(|row| row.transfer_bucket == "stable_nonzero")
        .count();
    let nonzero_churn_count = transfer_profile
        .candidate_rows
        .iter()
        .filter(|row| row.transfer_bucket == "nonzero_churn")
        .count();
    let admissible_delta_count = gain_zero_count as isize - loss_zero_count as isize;
    let admissible_set_equal = gain_zero_count == 0 && loss_zero_count == 0;
    let no_positive_admissible_delta = admissible_delta_count <= 0;

    let modulus_rows = from_profile
        .modulus_rows
        .iter()
        .zip(&to_profile.modulus_rows)
        .map(|(from_row, to_row)| {
            assert_eq!(
                from_row.modulus, to_row.modulus,
                "lane profile comparison requires aligned modulus rows"
            );
            KConfigLaneProfileComparisonModulusRow {
                modulus: from_row.modulus,
                excluded_seed_class_label_from: from_row.excluded_seed_class_label.clone(),
                excluded_seed_classes_from: from_row.excluded_seed_classes.clone(),
                excluded_seed_class_label_to: to_row.excluded_seed_class_label.clone(),
                excluded_seed_classes_to: to_row.excluded_seed_classes.clone(),
                profile_agreement: from_row.excluded_seed_classes == to_row.excluded_seed_classes,
            }
        })
        .collect::<Vec<_>>();
    let profile_agreement = modulus_rows.iter().all(|row| row.profile_agreement);

    KConfigLaneProfileComparison {
        base,
        middle_length,
        outer,
        inner,
        pair_label: format!("({},{})", digit_symbol(outer), digit_symbol(inner)),
        from_k: format_k(from_k),
        to_k: format_k(to_k),
        candidates_per_config: transfer_profile.candidates_per_config,
        compared_moduli_count: modulus_rows.len(),
        all_singleton_profiles: from_profile.all_singleton_profiles
            && to_profile.all_singleton_profiles,
        profile_agreement,
        admissible_set_equal,
        no_positive_admissible_delta,
        admissible_delta_count,
        theorem_rung_label: classify_theorem_rung(
            profile_agreement,
            admissible_set_equal,
            no_positive_admissible_delta,
        )
        .to_string(),
        stable_zero_count,
        gain_zero_count,
        loss_zero_count,
        stable_nonzero_count,
        nonzero_churn_count,
        modulus_rows,
    }
}

pub fn local_affine_relation_label(shift_equal: bool, gradient_equal: bool) -> &'static str {
    match (shift_equal, gradient_equal) {
        (true, true) => "identity",
        (true, false) => "shift_only",
        (false, true) => "gradient_only",
        (false, false) => "shift_and_gradient",
    }
}

pub fn scan_k_config_affine_lane_comparison(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    from_k: BoundedKConfig,
    to_k: BoundedKConfig,
) -> KConfigAffineLaneComparison {
    let from_profile = scan_k_config_affine_profile(base, middle_length, outer, inner, from_k);
    let to_profile = if from_k == to_k {
        from_profile.clone()
    } else {
        scan_k_config_affine_profile(base, middle_length, outer, inner, to_k)
    };

    assert_eq!(
        from_profile.compared_moduli_count, to_profile.compared_moduli_count,
        "affine lane comparison requires matching coprime-modulus families",
    );

    let modulus_rows = from_profile
        .modulus_rows
        .iter()
        .zip(&to_profile.modulus_rows)
        .map(|(from_row, to_row)| {
            assert_eq!(
                from_row.modulus, to_row.modulus,
                "affine lane comparison requires aligned modulus rows",
            );
            let shift_equal = from_row.shift_modulus == to_row.shift_modulus;
            let gradient_equal = from_row.gradient_modulus == to_row.gradient_modulus;
            let zero_seed_equal = from_row.zero_seed_class == to_row.zero_seed_class;
            KConfigAffineLaneComparisonModulusRow {
                modulus: from_row.modulus,
                shift_modulus_from: from_row.shift_modulus,
                shift_modulus_to: to_row.shift_modulus,
                gradient_modulus_from: from_row.gradient_modulus,
                gradient_modulus_to: to_row.gradient_modulus,
                zero_seed_class_from: from_row.zero_seed_class,
                zero_seed_class_to: to_row.zero_seed_class,
                shift_equal,
                gradient_equal,
                zero_seed_equal,
                local_relation_label: local_affine_relation_label(shift_equal, gradient_equal)
                    .to_string(),
            }
        })
        .collect::<Vec<_>>();

    let same_shift_count = modulus_rows.iter().filter(|row| row.shift_equal).count();
    let same_gradient_count = modulus_rows.iter().filter(|row| row.gradient_equal).count();
    let same_zero_seed_count = modulus_rows
        .iter()
        .filter(|row| row.zero_seed_equal)
        .count();
    let identity_count = modulus_rows
        .iter()
        .filter(|row| row.local_relation_label == "identity")
        .count();
    let shift_only_count = modulus_rows
        .iter()
        .filter(|row| row.local_relation_label == "shift_only")
        .count();
    let gradient_only_count = modulus_rows
        .iter()
        .filter(|row| row.local_relation_label == "gradient_only")
        .count();
    let shift_and_gradient_count = modulus_rows
        .iter()
        .filter(|row| row.local_relation_label == "shift_and_gradient")
        .count();
    let compared_moduli_count = modulus_rows.len();

    KConfigAffineLaneComparison {
        base,
        middle_length,
        outer,
        inner,
        pair_label: format!("({},{})", digit_symbol(outer), digit_symbol(inner)),
        from_k: format_k(from_k),
        to_k: format_k(to_k),
        candidates_per_config: from_profile.candidates_per_config,
        compared_moduli_count,
        same_shift_count,
        same_gradient_count,
        same_zero_seed_count,
        identity_count,
        shift_only_count,
        gradient_only_count,
        shift_and_gradient_count,
        same_shift_share: ratio(same_shift_count, compared_moduli_count),
        same_gradient_share: ratio(same_gradient_count, compared_moduli_count),
        same_zero_seed_share: ratio(same_zero_seed_count, compared_moduli_count),
        identity_share: ratio(identity_count, compared_moduli_count),
        shift_only_share: ratio(shift_only_count, compared_moduli_count),
        gradient_only_share: ratio(gradient_only_count, compared_moduli_count),
        shift_and_gradient_share: ratio(shift_and_gradient_count, compared_moduli_count),
        modulus_rows,
    }
}

pub fn bounded_k_signal_source_label(
    stable_zero_prime_delta_pp: f64,
    boundary_prime_delta_pp: f64,
) -> &'static str {
    const EPS: f64 = 1e-9;
    let stable_abs = stable_zero_prime_delta_pp.abs();
    let boundary_abs = boundary_prime_delta_pp.abs();
    if stable_zero_prime_delta_pp > 0.0 && stable_abs > boundary_abs + EPS {
        "stable_zero_led"
    } else if boundary_abs > stable_abs + EPS {
        "boundary_led"
    } else {
        "mixed_or_flat"
    }
}

pub fn analyze_best_vs_k00_feature_row(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
) -> BestVsK00FeatureRow {
    let pair_row = evaluate_pair_row(base, middle_length, outer, inner, DEFAULT_BOUNDED_K_GRID);
    analyze_pair_row_best_vs_k00_feature_row(base, middle_length, outer, inner, &pair_row)
}

fn analyze_pair_row_best_vs_k00_feature_row(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    pair_row: &KDominancePairRow,
) -> BestVsK00FeatureRow {
    let best_k = parse_k_label(&pair_row.best_k);
    let k00_profile = scan_k_config_mask_profile(base, middle_length, outer, inner, (0, 0));
    let best_profile = if best_k == (0, 0) {
        k00_profile.clone()
    } else {
        scan_k_config_mask_profile(base, middle_length, outer, inner, best_k)
    };
    let transfer_profile =
        scan_k_config_transfer_profile(base, middle_length, outer, inner, (0, 0), best_k);

    let mut shared_admissible_count = 0usize;
    let mut shared_prime_hits_k00 = 0usize;
    let mut shared_prime_hits_best = 0usize;
    let mut stable_zero_prime_delta_count = 0isize;
    let mut boundary_prime_delta_count = 0isize;
    let mut bucket_counts = BTreeMap::<&'static str, usize>::new();

    for row in &transfer_profile.candidate_rows {
        *bucket_counts
            .entry(classify_transfer_bucket(
                row.divisibility_mask_from,
                row.divisibility_mask_to,
            ))
            .or_insert(0) += 1;
        match (row.admissible_from, row.admissible_to) {
            (true, true) => {
                shared_admissible_count += 1;
                if row.prime_from {
                    shared_prime_hits_k00 += 1;
                    stable_zero_prime_delta_count -= 1;
                }
                if row.prime_to {
                    shared_prime_hits_best += 1;
                    stable_zero_prime_delta_count += 1;
                }
            }
            (false, true) => {
                if row.prime_to {
                    boundary_prime_delta_count += 1;
                }
            }
            (true, false) => {
                if row.prime_from {
                    boundary_prime_delta_count -= 1;
                }
            }
            (false, false) => {}
        }
    }

    let stable_zero_count = *bucket_counts.get("stable_zero").unwrap_or(&0);
    let gain_zero_count = *bucket_counts.get("gain_zero").unwrap_or(&0);
    let loss_zero_count = *bucket_counts.get("loss_zero").unwrap_or(&0);
    let stable_nonzero_count = *bucket_counts.get("stable_nonzero").unwrap_or(&0);
    let nonzero_churn_count = *bucket_counts.get("nonzero_churn").unwrap_or(&0);
    let same_mask_count = stable_zero_count + stable_nonzero_count;
    let zero_union_count = stable_zero_count + gain_zero_count + loss_zero_count;

    let admissible_share_k00 = ratio(
        k00_profile.admissible_count,
        k00_profile.candidates_per_config,
    );
    let admissible_share_best = ratio(
        best_profile.admissible_count,
        best_profile.candidates_per_config,
    );
    let prime_yield_k00 = ratio(k00_profile.prime_hits, k00_profile.admissible_count);
    let prime_yield_best = ratio(best_profile.prime_hits, best_profile.admissible_count);
    let admissible_set_effect_pp =
        (admissible_share_best - admissible_share_k00) * prime_yield_k00 * 100.0;
    let prime_yield_effect_pp =
        admissible_share_best * (prime_yield_best - prime_yield_k00) * 100.0;
    let stable_zero_prime_delta_pp = count_delta_pp_signed(
        stable_zero_prime_delta_count,
        best_profile.candidates_per_config,
    );
    let boundary_prime_delta_pp = count_delta_pp_signed(
        boundary_prime_delta_count,
        best_profile.candidates_per_config,
    );
    let stable_zero_signal_margin_count =
        stable_zero_prime_delta_count - boundary_prime_delta_count.abs();
    let stable_zero_signal_margin_pp = stable_zero_prime_delta_pp - boundary_prime_delta_pp.abs();
    let anomaly_mass_pp = pair_row.best_minus_k00_pp.max(0.0);
    let shared_prime_rate_k00_pp = ratio(shared_prime_hits_k00, shared_admissible_count) * 100.0;
    let shared_prime_rate_best_pp = ratio(shared_prime_hits_best, shared_admissible_count) * 100.0;
    let shared_prime_rate_delta_pp = shared_prime_rate_best_pp - shared_prime_rate_k00_pp;

    BestVsK00FeatureRow {
        base,
        middle_length,
        outer,
        inner,
        pair_label: format!("({},{})", digit_symbol(outer), digit_symbol(inner)),
        same_digit: outer == inner,
        unit_distance: cyclic_unit_distance(base, outer, inner),
        gap_bucket: gap_bucket(base, outer, inner).to_string(),
        candidates_per_config: best_profile.candidates_per_config,
        best_k: pair_row.best_k.clone(),
        active: anomaly_mass_pp > 0.0,
        anomaly_mass_pp,
        admissible_delta_pp: count_delta_pp(
            best_profile.admissible_count,
            k00_profile.admissible_count,
            best_profile.candidates_per_config,
        ),
        admissible_set_effect_pp,
        prime_yield_effect_pp,
        shared_admissible_count,
        stable_zero_prime_delta_count,
        boundary_prime_delta_count,
        stable_zero_prime_delta_pp,
        boundary_prime_delta_pp,
        shared_prime_rate_k00_pp,
        shared_prime_rate_best_pp,
        shared_prime_rate_delta_pp,
        stable_zero_signal_margin_count,
        stable_zero_signal_margin_pp,
        stable_zero_support_ratio: if anomaly_mass_pp > 0.0 {
            stable_zero_prime_delta_pp / anomaly_mass_pp
        } else {
            0.0
        },
        mask_stability_share: same_mask_count as f64 / best_profile.candidates_per_config as f64,
        admissible_overlap_jaccard: ratio(stable_zero_count, zero_union_count),
        nonzero_churn_share: nonzero_churn_count as f64 / best_profile.candidates_per_config as f64,
        stable_zero_count,
        gain_zero_count,
        loss_zero_count,
        stable_nonzero_count,
        nonzero_churn_count,
        positive_shared_yield: stable_zero_prime_delta_count > 0,
        shared_yield_core: stable_zero_prime_delta_count > boundary_prime_delta_count.abs()
            && stable_zero_prime_delta_count > 0
            && prime_yield_effect_pp.abs() > admissible_set_effect_pp.abs(),
        signal_source_label: bounded_k_signal_source_label(
            stable_zero_prime_delta_pp,
            boundary_prime_delta_pp,
        )
        .to_string(),
    }
}

pub fn analyze_hinge_feature_row(base: u32, outer: u32, inner: u32) -> HingeFeatureRow {
    let m1 = analyze_best_vs_k00_feature_row(base, 1, outer, inner);
    let m2 = analyze_best_vs_k00_feature_row(base, 2, outer, inner);
    let m1_to_m2_persistent = m1.active && m2.active;
    let shared_yield_core = m2.shared_yield_core;

    HingeFeatureRow {
        base,
        outer,
        inner,
        pair_label: m2.pair_label.clone(),
        same_digit: m2.same_digit,
        unit_distance: m2.unit_distance,
        gap_bucket: m2.gap_bucket.clone(),
        m1_active: m1.active,
        m2_active: m2.active,
        m1_to_m2_persistent,
        m1_best_k: m1.best_k.clone(),
        m2_best_k: m2.best_k.clone(),
        m1_anomaly_mass_pp: m1.anomaly_mass_pp,
        m2_anomaly_mass_pp: m2.anomaly_mass_pp,
        m1_admissible_delta_pp: m1.admissible_delta_pp,
        m2_admissible_delta_pp: m2.admissible_delta_pp,
        m1_stable_zero_prime_delta_count: m1.stable_zero_prime_delta_count,
        m2_stable_zero_prime_delta_count: m2.stable_zero_prime_delta_count,
        m1_boundary_prime_delta_count: m1.boundary_prime_delta_count,
        m2_boundary_prime_delta_count: m2.boundary_prime_delta_count,
        m1_stable_zero_prime_delta_pp: m1.stable_zero_prime_delta_pp,
        m2_stable_zero_prime_delta_pp: m2.stable_zero_prime_delta_pp,
        m1_boundary_prime_delta_pp: m1.boundary_prime_delta_pp,
        m2_boundary_prime_delta_pp: m2.boundary_prime_delta_pp,
        m1_shared_prime_rate_delta_pp: m1.shared_prime_rate_delta_pp,
        m2_shared_prime_rate_delta_pp: m2.shared_prime_rate_delta_pp,
        m1_stable_zero_signal_margin_count: m1.stable_zero_signal_margin_count,
        m2_stable_zero_signal_margin_count: m2.stable_zero_signal_margin_count,
        m1_stable_zero_signal_margin_pp: m1.stable_zero_signal_margin_pp,
        m2_stable_zero_signal_margin_pp: m2.stable_zero_signal_margin_pp,
        m1_stable_zero_support_ratio: m1.stable_zero_support_ratio,
        m2_stable_zero_support_ratio: m2.stable_zero_support_ratio,
        m1_mask_stability_share: m1.mask_stability_share,
        m2_mask_stability_share: m2.mask_stability_share,
        m1_admissible_overlap_jaccard: m1.admissible_overlap_jaccard,
        m2_admissible_overlap_jaccard: m2.admissible_overlap_jaccard,
        m1_nonzero_churn_share: m1.nonzero_churn_share,
        m2_nonzero_churn_share: m2.nonzero_churn_share,
        m2_stable_zero_count: m2.stable_zero_count,
        m2_gain_zero_count: m2.gain_zero_count,
        m2_loss_zero_count: m2.loss_zero_count,
        m2_stable_nonzero_count: m2.stable_nonzero_count,
        m2_nonzero_churn_count: m2.nonzero_churn_count,
        m1_signal_source_label: m1.signal_source_label.clone(),
        m2_signal_source_label: m2.signal_source_label.clone(),
        shared_yield_core,
        hinge_category: bounded_k_hinge_category(m1_to_m2_persistent, shared_yield_core)
            .to_string(),
    }
}

pub fn analyze_affine_hinge_feature_row(
    base: u32,
    outer: u32,
    inner: u32,
) -> AffineHingeFeatureRow {
    let hinge_row = analyze_hinge_feature_row(base, outer, inner);
    let m1_affine = scan_k_config_affine_lane_comparison(
        base,
        1,
        outer,
        inner,
        (0, 0),
        parse_k_label(&hinge_row.m1_best_k),
    );
    let m2_affine = scan_k_config_affine_lane_comparison(
        base,
        2,
        outer,
        inner,
        (0, 0),
        parse_k_label(&hinge_row.m2_best_k),
    );

    AffineHingeFeatureRow {
        hinge_row,
        m1_affine_compared_moduli_count: m1_affine.compared_moduli_count,
        m1_affine_same_shift_count: m1_affine.same_shift_count,
        m1_affine_same_gradient_count: m1_affine.same_gradient_count,
        m1_affine_same_zero_seed_count: m1_affine.same_zero_seed_count,
        m1_affine_identity_count: m1_affine.identity_count,
        m1_affine_shift_only_count: m1_affine.shift_only_count,
        m1_affine_gradient_only_count: m1_affine.gradient_only_count,
        m1_affine_shift_and_gradient_count: m1_affine.shift_and_gradient_count,
        m1_affine_same_shift_share: m1_affine.same_shift_share,
        m1_affine_same_gradient_share: m1_affine.same_gradient_share,
        m1_affine_same_zero_seed_share: m1_affine.same_zero_seed_share,
        m1_affine_identity_share: m1_affine.identity_share,
        m1_affine_shift_only_share: m1_affine.shift_only_share,
        m1_affine_gradient_only_share: m1_affine.gradient_only_share,
        m1_affine_shift_and_gradient_share: m1_affine.shift_and_gradient_share,
        m2_affine_compared_moduli_count: m2_affine.compared_moduli_count,
        m2_affine_same_shift_count: m2_affine.same_shift_count,
        m2_affine_same_gradient_count: m2_affine.same_gradient_count,
        m2_affine_same_zero_seed_count: m2_affine.same_zero_seed_count,
        m2_affine_identity_count: m2_affine.identity_count,
        m2_affine_shift_only_count: m2_affine.shift_only_count,
        m2_affine_gradient_only_count: m2_affine.gradient_only_count,
        m2_affine_shift_and_gradient_count: m2_affine.shift_and_gradient_count,
        m2_affine_same_shift_share: m2_affine.same_shift_share,
        m2_affine_same_gradient_share: m2_affine.same_gradient_share,
        m2_affine_same_zero_seed_share: m2_affine.same_zero_seed_share,
        m2_affine_identity_share: m2_affine.identity_share,
        m2_affine_shift_only_share: m2_affine.shift_only_share,
        m2_affine_gradient_only_share: m2_affine.gradient_only_share,
        m2_affine_shift_and_gradient_share: m2_affine.shift_and_gradient_share,
    }
}

pub fn analyze_best_vs_k00_decomposition(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    k_grid: &[BoundedKConfig],
) -> KBestVsK00Decomposition {
    let pair_row = evaluate_pair_row(base, middle_length, outer, inner, k_grid);
    analyze_pair_row_best_vs_k00(base, middle_length, outer, inner, &pair_row)
}

pub fn analyze_pair_row_best_vs_k00(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    pair_row: &KDominancePairRow,
) -> KBestVsK00Decomposition {
    let feature_row =
        analyze_pair_row_best_vs_k00_feature_row(base, middle_length, outer, inner, pair_row);

    KBestVsK00Decomposition {
        best_k: feature_row.best_k,
        anomaly_mass_pp: feature_row.anomaly_mass_pp,
        admissible_delta_pp: feature_row.admissible_delta_pp,
        stable_zero_prime_delta_pp: feature_row.stable_zero_prime_delta_pp,
        boundary_prime_delta_pp: feature_row.boundary_prime_delta_pp,
        shared_prime_rate_delta_pp: feature_row.shared_prime_rate_delta_pp,
        signal_source_label: feature_row.signal_source_label,
    }
}

pub fn to_base_string_fixed(mut value: u32, base: u32, width: usize) -> String {
    let mut digits = vec!['0'; width];
    for index in (0..width).rev() {
        let digit = value % base;
        digits[index] = if digit < 10 {
            char::from_digit(digit, 10).expect("digit should fit decimal")
        } else {
            char::from_u32('A' as u32 + digit - 10).expect("digit should fit uppercase alphabet")
        };
        value /= base;
    }
    digits.into_iter().collect()
}

pub fn mask_moduli(divisibility_mask: u16) -> Vec<u32> {
    DEFAULT_PREFILTER_PRIMES
        .iter()
        .enumerate()
        .filter_map(|(bit_index, &modulus)| {
            if divisibility_mask & (1u16 << bit_index) != 0 {
                Some(modulus)
            } else {
                None
            }
        })
        .collect()
}

pub fn render_divisibility_mask(divisibility_mask: u16) -> String {
    let moduli = mask_moduli(divisibility_mask);
    if moduli.is_empty() {
        "zero_mask".to_string()
    } else {
        moduli
            .into_iter()
            .map(|modulus| format!("p{modulus}"))
            .collect::<Vec<_>>()
            .join("|")
    }
}

fn render_seed_class_profile(seed_classes: &[u32]) -> String {
    if seed_classes.is_empty() {
        "none".to_string()
    } else {
        seed_classes
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("|")
    }
}

fn excluded_seed_classes_for_modulus(
    start_residue: u32,
    step_residue: u32,
    modulus: u32,
) -> Vec<u32> {
    (0..modulus)
        .filter(|&seed_class| {
            (start_residue as u64 + step_residue as u64 * seed_class as u64)
                .is_multiple_of(modulus as u64)
        })
        .collect()
}

fn classify_transfer_bucket(
    divisibility_mask_from: u16,
    divisibility_mask_to: u16,
) -> &'static str {
    match (divisibility_mask_from, divisibility_mask_to) {
        (0, 0) => "stable_zero",
        (0, _) => "loss_zero",
        (_, 0) => "gain_zero",
        (left, right) if left == right => "stable_nonzero",
        _ => "nonzero_churn",
    }
}

fn build_k_config_scan_setup(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    (k_outer, k_inner): BoundedKConfig,
) -> KConfigScanSetup {
    let prefix_digits = {
        let mut digits = Vec::with_capacity((2 + k_outer + k_inner) as usize);
        digits.push(outer);
        digits.extend(std::iter::repeat_n(0, k_outer as usize));
        digits.push(inner);
        digits.extend(std::iter::repeat_n(0, k_inner as usize));
        digits
    };
    let suffix_digits = {
        let mut digits = Vec::with_capacity((2 + k_outer + k_inner) as usize);
        digits.extend(std::iter::repeat_n(0, k_inner as usize));
        digits.push(inner);
        digits.extend(std::iter::repeat_n(0, k_outer as usize));
        digits.push(outer);
        digits
    };
    let suffix_len = suffix_digits.len() as u32;
    let base_big = BigUint::from(base);
    let step = base_big.pow(suffix_len);
    let prefix_shift = base_big.pow((middle_length as u32) + suffix_len);
    let prefix_value = digits_to_biguint(base, &prefix_digits);
    let suffix_value = digits_to_biguint(base, &suffix_digits);
    let candidate_base = prefix_value * prefix_shift + suffix_value;
    let candidates_per_config = (base as usize).pow(middle_length as u32);
    let residue_filters = DEFAULT_PREFILTER_PRIMES
        .iter()
        .copied()
        .map(|modulus| {
            let base_mod = base % modulus;
            let prefix_mod = digits_to_mod(base, &prefix_digits, modulus);
            let suffix_mod = digits_to_mod(base, &suffix_digits, modulus);
            let start_residue = (prefix_mod
                * pow_mod(base_mod, (middle_length as u32) + suffix_len, modulus)
                + suffix_mod)
                % modulus;
            let step_residue = pow_mod(base_mod, suffix_len, modulus);
            ResidueFilterState {
                residue: start_residue,
                step_residue,
                modulus,
            }
        })
        .collect::<Vec<_>>();

    KConfigScanSetup {
        candidate_base,
        step,
        candidates_per_config,
        residue_filters,
    }
}

fn digits_to_biguint(base: u32, digits: &[u32]) -> BigUint {
    let base_big = BigUint::from(base);
    let mut value = BigUint::from(0u32);
    for &digit in digits {
        value *= &base_big;
        value += digit;
    }
    value
}

fn digits_to_mod(base: u32, digits: &[u32], modulus: u32) -> u32 {
    let mut value = 0u32;
    for &digit in digits {
        value = (value * (base % modulus) + digit % modulus) % modulus;
    }
    value
}

fn pow_mod(mut base: u32, mut exp: u32, modulus: u32) -> u32 {
    let mut result = 1 % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }
    result
}

fn gcd_u32(mut left: u32, mut right: u32) -> u32 {
    while right != 0 {
        let next = left % right;
        left = right;
        right = next;
    }
    left
}

fn median(mut values: Vec<f64>) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.sort_by(|left, right| left.total_cmp(right));
    let middle = values.len() / 2;
    if values.len() % 2 == 1 {
        values[middle]
    } else {
        (values[middle - 1] + values[middle]) / 2.0
    }
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn count_delta_pp(left: usize, right: usize, total: usize) -> f64 {
    (left as f64 - right as f64) * 100.0 / total as f64
}

fn count_delta_pp_signed(delta: isize, total: usize) -> f64 {
    delta as f64 * 100.0 / total as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base6_counterexample_counts_match_smoke_run() {
        let row = evaluate_pair_row(6, 2, 5, 5, DEFAULT_BOUNDED_K_GRID);
        assert_eq!(row.prime_hits_k00, 8);
        assert_eq!(row.best_k, "k=(1,0)");
        assert_eq!(row.best_prime_hits, 11);
        assert!((row.best_minus_k00_pp - 8.333333333333334).abs() < 1e-12);
    }

    #[test]
    fn smoke_pair_selection_keeps_anchor_pairs() {
        let pairs = select_smoke_pairs(10, 4, &[(3, 3), (3, 7)]);
        assert!(pairs.contains(&(3, 3)));
        assert!(pairs.contains(&(3, 7)));
        assert_eq!(pairs.len(), 4);
    }

    #[test]
    fn config_profile_prime_hits_match_exact_scan() {
        let profile = scan_k_config_profile(6, 2, 5, 5, (1, 0));
        assert_eq!(profile.prime_hits, 11);
        assert_eq!(profile.candidates_per_config, 36);
        assert!(profile.admissible_count >= profile.prime_hits);
        assert_eq!(
            profile.modulus_divisibility_rows.len(),
            DEFAULT_PREFILTER_PRIMES.len()
        );
    }

    #[test]
    fn mask_profile_reproduces_lightweight_profile_and_marginals() {
        let lightweight = scan_k_config_profile(6, 2, 5, 5, (1, 0));
        let detailed = scan_k_config_mask_profile(6, 2, 5, 5, (1, 0));

        assert_eq!(
            detailed.candidates_per_config,
            lightweight.candidates_per_config
        );
        assert_eq!(detailed.admissible_count, lightweight.admissible_count);
        assert_eq!(detailed.prime_hits, lightweight.prime_hits);

        let modulus_counts = lightweight
            .modulus_divisibility_rows
            .iter()
            .map(|row| (row.modulus, row.divisible_count))
            .collect::<BTreeMap<_, _>>();
        let collapsed_counts = DEFAULT_PREFILTER_PRIMES
            .iter()
            .enumerate()
            .map(|(bit_index, &modulus)| {
                let count = detailed
                    .candidate_rows
                    .iter()
                    .filter(|row| row.divisibility_mask & (1u16 << bit_index) != 0)
                    .count();
                (modulus, count)
            })
            .collect::<BTreeMap<_, _>>();

        assert_eq!(collapsed_counts, modulus_counts);
    }

    #[test]
    fn zero_mask_histogram_matches_admissible_count() {
        let detailed = scan_k_config_mask_profile(14, 2, 13, 11, (0, 1));
        let zero_mask_count = detailed
            .mask_histogram_rows
            .iter()
            .find(|row| row.divisibility_mask == 0)
            .map(|row| row.count)
            .unwrap_or(0);

        assert_eq!(zero_mask_count, detailed.admissible_count);
    }

    #[test]
    fn admissible_prime_flags_match_prime_hits() {
        let detailed = scan_k_config_mask_profile(14, 2, 13, 11, (0, 1));
        let admissible_prime_count = detailed
            .candidate_rows
            .iter()
            .filter(|row| row.admissible && row.prime)
            .count();

        assert_eq!(admissible_prime_count, detailed.prime_hits);
    }

    #[test]
    fn transfer_profile_preserves_from_and_to_mask_marginals() {
        let from_profile = scan_k_config_mask_profile(14, 2, 13, 11, (0, 0));
        let to_profile = scan_k_config_mask_profile(14, 2, 13, 11, (0, 1));
        let transfer_profile = scan_k_config_transfer_profile(14, 2, 13, 11, (0, 0), (0, 1));

        let collapse = |is_from: bool| {
            let mut collapsed = BTreeMap::<u16, (usize, usize)>::new();
            for row in &transfer_profile.candidate_rows {
                let mask = if is_from {
                    row.divisibility_mask_from
                } else {
                    row.divisibility_mask_to
                };
                let prime = if is_from {
                    row.prime_from
                } else {
                    row.prime_to
                };
                let entry = collapsed.entry(mask).or_insert((0usize, 0usize));
                entry.0 += 1;
                if prime {
                    entry.1 += 1;
                }
            }
            collapsed
        };

        let from_expected = from_profile
            .mask_histogram_rows
            .iter()
            .map(|row| (row.divisibility_mask, (row.count, row.prime_count)))
            .collect::<BTreeMap<_, _>>();
        let to_expected = to_profile
            .mask_histogram_rows
            .iter()
            .map(|row| (row.divisibility_mask, (row.count, row.prime_count)))
            .collect::<BTreeMap<_, _>>();

        assert_eq!(collapse(true), from_expected);
        assert_eq!(collapse(false), to_expected);
        assert_eq!(
            transfer_profile
                .transfer_histogram_rows
                .iter()
                .map(|row| row.count)
                .sum::<usize>(),
            transfer_profile.candidates_per_config
        );
    }

    #[test]
    fn transfer_profile_stable_zero_matches_shared_admissible_overlap() {
        let transfer_profile = scan_k_config_transfer_profile(14, 2, 13, 11, (0, 0), (0, 1));
        let stable_zero_count = transfer_profile
            .candidate_rows
            .iter()
            .filter(|row| row.transfer_bucket == "stable_zero")
            .count();

        let shared_admissible_count = transfer_profile
            .candidate_rows
            .iter()
            .filter(|row| row.admissible_from && row.admissible_to)
            .count();

        assert_eq!(stable_zero_count, shared_admissible_count);
    }

    #[test]
    fn transfer_profile_stable_zero_prime_flags_match_shared_overlap_counts() {
        let transfer_profile = scan_k_config_transfer_profile(14, 2, 13, 11, (0, 0), (0, 1));
        let stable_zero_prime_from = transfer_profile
            .candidate_rows
            .iter()
            .filter(|row| row.transfer_bucket == "stable_zero" && row.prime_from)
            .count();
        let stable_zero_prime_to = transfer_profile
            .candidate_rows
            .iter()
            .filter(|row| row.transfer_bucket == "stable_zero" && row.prime_to)
            .count();

        let manual_prime_from = transfer_profile
            .candidate_rows
            .iter()
            .filter(|row| row.admissible_from && row.admissible_to && row.prime_from)
            .count();
        let manual_prime_to = transfer_profile
            .candidate_rows
            .iter()
            .filter(|row| row.admissible_from && row.admissible_to && row.prime_to)
            .count();

        assert_eq!(stable_zero_prime_from, manual_prime_from);
        assert_eq!(stable_zero_prime_to, manual_prime_to);
    }

    #[test]
    fn best_vs_k00_decomposition_matches_pair_row_anomaly_mass() {
        let row = evaluate_pair_row(6, 2, 5, 5, DEFAULT_BOUNDED_K_GRID);
        let decomposition = analyze_pair_row_best_vs_k00(6, 2, 5, 5, &row);

        assert_eq!(decomposition.best_k, row.best_k);
        assert!((decomposition.anomaly_mass_pp - row.best_minus_k00_pp.max(0.0)).abs() < 1e-12);
    }

    #[test]
    fn best_vs_k00_decomposition_matches_manual_transfer_accounting() {
        let row = evaluate_pair_row(14, 2, 13, 11, DEFAULT_BOUNDED_K_GRID);
        let decomposition = analyze_pair_row_best_vs_k00(14, 2, 13, 11, &row);
        let transfer_profile = scan_k_config_transfer_profile(14, 2, 13, 11, (0, 0), (0, 1));
        let k00_profile = scan_k_config_mask_profile(14, 2, 13, 11, (0, 0));
        let best_profile = scan_k_config_mask_profile(14, 2, 13, 11, (0, 1));

        let mut shared_admissible_count = 0usize;
        let mut shared_prime_hits_k00 = 0usize;
        let mut shared_prime_hits_best = 0usize;
        let mut stable_zero_prime_delta_count = 0isize;
        let mut boundary_prime_delta_count = 0isize;

        for transfer_row in &transfer_profile.candidate_rows {
            match (transfer_row.admissible_from, transfer_row.admissible_to) {
                (true, true) => {
                    shared_admissible_count += 1;
                    if transfer_row.prime_from {
                        shared_prime_hits_k00 += 1;
                        stable_zero_prime_delta_count -= 1;
                    }
                    if transfer_row.prime_to {
                        shared_prime_hits_best += 1;
                        stable_zero_prime_delta_count += 1;
                    }
                }
                (false, true) => {
                    if transfer_row.prime_to {
                        boundary_prime_delta_count += 1;
                    }
                }
                (true, false) => {
                    if transfer_row.prime_from {
                        boundary_prime_delta_count -= 1;
                    }
                }
                (false, false) => {}
            }
        }

        let admissible_delta_pp = count_delta_pp(
            best_profile.admissible_count,
            k00_profile.admissible_count,
            best_profile.candidates_per_config,
        );
        let stable_zero_prime_delta_pp = count_delta_pp_signed(
            stable_zero_prime_delta_count,
            best_profile.candidates_per_config,
        );
        let boundary_prime_delta_pp = count_delta_pp_signed(
            boundary_prime_delta_count,
            best_profile.candidates_per_config,
        );
        let shared_prime_rate_delta_pp = (ratio(shared_prime_hits_best, shared_admissible_count)
            - ratio(shared_prime_hits_k00, shared_admissible_count))
            * 100.0;

        assert!((decomposition.admissible_delta_pp - admissible_delta_pp).abs() < 1e-12);
        assert!(
            (decomposition.stable_zero_prime_delta_pp - stable_zero_prime_delta_pp).abs() < 1e-12
        );
        assert!((decomposition.boundary_prime_delta_pp - boundary_prime_delta_pp).abs() < 1e-12);
        assert!(
            (decomposition.shared_prime_rate_delta_pp - shared_prime_rate_delta_pp).abs() < 1e-12
        );
    }

    #[test]
    fn best_vs_k00_decomposition_signal_source_matches_representative_pairs() {
        let stable_zero_led =
            analyze_best_vs_k00_decomposition(14, 2, 13, 11, DEFAULT_BOUNDED_K_GRID);
        let boundary_led = analyze_best_vs_k00_decomposition(10, 2, 3, 3, DEFAULT_BOUNDED_K_GRID);

        assert_eq!(stable_zero_led.signal_source_label, "stable_zero_led");
        assert_eq!(boundary_led.signal_source_label, "boundary_led");
    }

    #[test]
    fn representative_best_vs_k00_feature_rows_match_known_counts() {
        let base14_db = analyze_best_vs_k00_feature_row(14, 2, 13, 11);
        assert_eq!(base14_db.best_k, "k=(0,1)");
        assert_eq!(base14_db.stable_zero_prime_delta_count, 12);
        assert_eq!(base14_db.boundary_prime_delta_count, -6);
        assert_eq!(base14_db.stable_zero_signal_margin_count, 6);
        assert_eq!(base14_db.signal_source_label, "stable_zero_led");
        assert!(base14_db.shared_yield_core);

        let base10_33 = analyze_best_vs_k00_feature_row(10, 2, 3, 3);
        assert_eq!(base10_33.best_k, "k=(1,0)");
        assert_eq!(base10_33.stable_zero_prime_delta_count, 0);
        assert_eq!(base10_33.boundary_prime_delta_count, 2);
        assert_eq!(base10_33.stable_zero_signal_margin_count, -2);
        assert_eq!(base10_33.signal_source_label, "boundary_led");
        assert!(!base10_33.shared_yield_core);

        let base26_nn = analyze_best_vs_k00_feature_row(26, 2, 23, 23);
        assert_eq!(base26_nn.best_k, "k=(0,1)");
        assert_eq!(base26_nn.stable_zero_prime_delta_count, 5);
        assert_eq!(base26_nn.boundary_prime_delta_count, 3);
        assert_eq!(base26_nn.stable_zero_signal_margin_count, 2);
        assert_eq!(base26_nn.signal_source_label, "stable_zero_led");
        assert!(base26_nn.shared_yield_core);

        let base22_hj = analyze_best_vs_k00_feature_row(22, 2, 17, 19);
        assert_eq!(base22_hj.best_k, "k=(0,1)");
        assert_eq!(base22_hj.stable_zero_prime_delta_count, -2);
        assert_eq!(base22_hj.boundary_prime_delta_count, 5);
        assert_eq!(base22_hj.stable_zero_signal_margin_count, -7);
        assert_eq!(base22_hj.signal_source_label, "boundary_led");
        assert!(!base22_hj.shared_yield_core);
    }

    #[test]
    fn hinge_feature_rows_reproduce_representative_categories() {
        let base14_db = analyze_hinge_feature_row(14, 13, 11);
        assert_eq!(base14_db.hinge_category, HINGE_CATEGORY_PERSISTENT_CORE);
        assert!(base14_db.m1_to_m2_persistent);
        assert_eq!(base14_db.m2_signal_source_label, "stable_zero_led");

        let base10_33 = analyze_hinge_feature_row(10, 3, 3);
        assert_eq!(base10_33.hinge_category, HINGE_CATEGORY_PERSISTENCE_ONLY);
        assert!(base10_33.m1_to_m2_persistent);
        assert_eq!(base10_33.m2_signal_source_label, "boundary_led");

        let base26_nn = analyze_hinge_feature_row(26, 23, 23);
        assert_eq!(base26_nn.hinge_category, HINGE_CATEGORY_CORE_ONLY);
        assert!(!base26_nn.m1_to_m2_persistent);
        assert_eq!(base26_nn.m2_signal_source_label, "stable_zero_led");

        let base22_hj = analyze_hinge_feature_row(22, 17, 19);
        assert_eq!(base22_hj.hinge_category, HINGE_CATEGORY_ACTIVE_NEITHER);
        assert!(!base22_hj.m1_to_m2_persistent);
        assert_eq!(base22_hj.m2_signal_source_label, "boundary_led");
    }

    #[test]
    fn cyclic_gap_and_bucket_match_unit_cycle_geometry() {
        assert_eq!(cyclic_unit_distance(14, 13, 11), 1);
        assert_eq!(gap_bucket(14, 13, 11), "adjacent");
        assert_eq!(cyclic_unit_distance(10, 3, 3), 0);
        assert_eq!(gap_bucket(10, 3, 3), "same");
        assert_eq!(cyclic_unit_distance(34, 25, 9), 7);
        assert_eq!(gap_bucket(34, 25, 9), "wide");
    }

    #[test]
    fn residue_profiles_are_singleton_on_coprime_prefilter_moduli() {
        let profile = scan_k_config_residue_profile(14, 3, 13, 11, (0, 1));
        assert!(profile.compared_moduli_count > 0);
        assert!(profile.all_singleton_profiles);
        assert!(profile
            .modulus_rows
            .iter()
            .all(|row| row.excluded_seed_class_count == 1));
    }

    #[test]
    fn affine_profile_rows_match_residue_profile_singletons() {
        let residue_profile = scan_k_config_residue_profile(10, 2, 3, 7, (0, 0));
        let affine_profile = scan_k_config_affine_profile(10, 2, 3, 7, (0, 0));
        assert_eq!(
            residue_profile.compared_moduli_count,
            affine_profile.compared_moduli_count
        );
        for (residue_row, affine_row) in residue_profile
            .modulus_rows
            .iter()
            .zip(&affine_profile.modulus_rows)
        {
            assert_eq!(residue_row.modulus, affine_row.modulus);
            assert_eq!(residue_row.excluded_seed_class_count, 1);
            assert_eq!(
                residue_row.excluded_seed_classes,
                vec![affine_row.zero_seed_class]
            );
        }
    }

    #[test]
    fn affine_lane_relation_counts_partition_compared_moduli() {
        let comparison = scan_k_config_affine_lane_comparison(14, 2, 13, 11, (0, 0), (0, 1));
        assert_eq!(
            comparison.identity_count
                + comparison.shift_only_count
                + comparison.gradient_only_count
                + comparison.shift_and_gradient_count,
            comparison.compared_moduli_count
        );
    }

    #[test]
    fn affine_hinge_feature_rows_preserve_representative_categories() {
        let base14_db = analyze_affine_hinge_feature_row(14, 13, 11);
        assert_eq!(
            base14_db.hinge_row.hinge_category,
            HINGE_CATEGORY_PERSISTENT_CORE
        );

        let base10_33 = analyze_affine_hinge_feature_row(10, 3, 3);
        assert_eq!(
            base10_33.hinge_row.hinge_category,
            HINGE_CATEGORY_PERSISTENCE_ONLY
        );
    }

    #[test]
    fn affine_shift_and_gradient_match_direct_template_arithmetic_examples() {
        let base6_profile = scan_k_config_affine_profile(6, 1, 1, 5, (0, 0));
        let mod7 = base6_profile
            .modulus_rows
            .iter()
            .find(|row| row.modulus == 7)
            .expect("base 6 profile should include modulus 7");
        assert_eq!(mod7.shift_modulus, 6);
        assert_eq!(mod7.gradient_modulus, 1);
        assert_eq!(mod7.zero_seed_class, 1);

        let base10_profile = scan_k_config_affine_profile(10, 1, 3, 7, (1, 1));
        let mod11 = base10_profile
            .modulus_rows
            .iter()
            .find(|row| row.modulus == 11)
            .expect("base 10 profile should include modulus 11");
        assert_eq!(mod11.shift_modulus, 9);
        assert_eq!(mod11.gradient_modulus, 1);
        assert_eq!(mod11.zero_seed_class, 2);
    }

    #[test]
    fn direct_lane_counterexamples_pin_stage1_boundary() {
        let base6 = scan_k_config_lane_profile_comparison(6, 3, 1, 5, (0, 0), (0, 1));
        assert_eq!(base6.theorem_rung_label, "fails_all_three");
        assert!(!base6.profile_agreement);
        assert!(!base6.no_positive_admissible_delta);
        assert_eq!(base6.admissible_delta_count, 7);
        assert_eq!(base6.compared_moduli_count, 9);

        let base30 = scan_k_config_lane_profile_comparison(30, 3, 1, 1, (0, 0), (0, 1));
        assert_eq!(base30.theorem_rung_label, "fails_all_three");
        assert!(!base30.profile_agreement);
        assert!(!base30.no_positive_admissible_delta);
        assert_eq!(base30.admissible_delta_count, 3);
        assert_eq!(base30.compared_moduli_count, 8);
    }
}
