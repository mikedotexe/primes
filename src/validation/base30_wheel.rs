//! Base-30 compact wheel report helpers.
//!
//! This module is deliberately narrow: it explains the maintained base-30
//! compact lane as a classical wheel-compressed affine candidate surface, not
//! as a new residual density claim.

use crate::validation::{
    bounded_k::{
        digit_symbol, evaluate_pair_row, ordered_unit_pairs, scan_k_config_examples,
        KDominancePairRow, DEFAULT_BOUNDED_K_GRID,
    },
    fast_affine::{build_fast_affine_lane, FastLaneConfig},
};
use serde::Serialize;
use std::collections::BTreeSet;

pub const BASE30: u32 = 30;
pub const BASE30_TARGET_OUTER: u32 = 11;
pub const BASE30_TARGET_INNER: u32 = 7;
pub const BASE30_TARGET_MIDDLE_LENGTH: usize = 2;
pub const BASE30_TARGET_K: (u32, u32) = (0, 0);
pub const BASE30_RESIDUE_MODULI: &[u32] = &[7, 11, 13, 17, 19, 23, 29, 31];
pub const DEFAULT_BASE30_MIN_MIDDLE_LENGTH: usize = 1;
pub const DEFAULT_BASE30_MAX_MIDDLE_LENGTH: usize = 3;
pub const DEFAULT_BASE30_WITNESS_LIMIT: usize = 5;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Base30WheelSettings {
    pub min_middle_length: usize,
    pub max_middle_length: usize,
    pub witness_limit: usize,
}

impl Default for Base30WheelSettings {
    fn default() -> Self {
        Self {
            min_middle_length: DEFAULT_BASE30_MIN_MIDDLE_LENGTH,
            max_middle_length: DEFAULT_BASE30_MAX_MIDDLE_LENGTH,
            witness_limit: DEFAULT_BASE30_WITNESS_LIMIT,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30PairHeatmapRow {
    pub base: u32,
    pub middle_length: usize,
    pub outer: u32,
    pub inner: u32,
    pub pair_label: String,
    pub candidates_per_config: usize,
    pub k00_rank: usize,
    pub rate_k00: f64,
    pub rate_k01: f64,
    pub rate_k10: f64,
    pub rate_k11: f64,
    pub rate_k22: f64,
    pub prime_hits_k00: usize,
    pub best_k: String,
    pub best_rate: f64,
    pub k00_noninferior: bool,
    pub best_minus_k00_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30ResidueFunnelRow {
    pub stage_index: usize,
    pub modulus_set_label: String,
    pub added_modulus: Option<u32>,
    pub added_excluded_seed_classes: String,
    pub survivor_count: usize,
    pub survivor_share: f64,
    pub prime_count: usize,
    pub prime_rate_among_survivors: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30TopPairRow {
    pub selection_reason: String,
    pub k00_rank: usize,
    pub pair_label: String,
    pub outer: u32,
    pub inner: u32,
    pub rate_k00: f64,
    pub prime_hits_k00: usize,
    pub best_k: String,
    pub best_rate: f64,
    pub k00_noninferior: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30WitnessRow {
    pub pair_label: String,
    pub outer: u32,
    pub inner: u32,
    pub middle_length: usize,
    pub k_label: String,
    pub seed: u32,
    pub middle_digits: String,
    pub template_digits: String,
    pub decimal_value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30LengthSummaryRow {
    pub base: u32,
    pub middle_length: usize,
    pub ordered_pair_count: usize,
    pub mean_k00_rate: f64,
    pub median_k00_rate: f64,
    pub min_k00_rate: f64,
    pub max_k00_rate: f64,
    pub target_pair_rank: usize,
    pub target_pair_rate_k00: f64,
    pub noncompact_counterexample_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30WheelSummary {
    pub target_pair_label: String,
    pub target_shift: u64,
    pub target_gradient: u64,
    pub target_seed_capacity: u64,
    pub target_m2_rate_k00: f64,
    pub target_m2_rank: usize,
    pub top_m2_pair: String,
    pub top_m2_rate_k00: f64,
    pub m2_noncompact_counterexamples: usize,
    pub m3_noncompact_counterexamples: usize,
    pub strong_line: String,
    pub caution_line: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30WheelReport {
    pub settings: Base30WheelSettings,
    pub summary: Base30WheelSummary,
    pub length_summary_rows: Vec<Base30LengthSummaryRow>,
    pub pair_heatmap_rows: Vec<Base30PairHeatmapRow>,
    pub residue_funnel_rows: Vec<Base30ResidueFunnelRow>,
    pub top_pair_rows: Vec<Base30TopPairRow>,
    pub witness_rows: Vec<Base30WitnessRow>,
}

pub fn build_base30_wheel_report(settings: Base30WheelSettings) -> Base30WheelReport {
    let pair_heatmap_rows = build_pair_heatmap_rows(settings);
    let length_summary_rows = build_length_summary_rows(&pair_heatmap_rows);
    let residue_funnel_rows = build_base30_residue_funnel_rows();
    let top_pair_rows = build_top_pair_rows(&pair_heatmap_rows);
    let witness_rows = build_witness_rows(&top_pair_rows, settings.witness_limit);
    let summary = build_summary(&pair_heatmap_rows, &length_summary_rows);

    Base30WheelReport {
        settings,
        summary,
        length_summary_rows,
        pair_heatmap_rows,
        residue_funnel_rows,
        top_pair_rows,
        witness_rows,
    }
}

pub fn base30_target_lane() -> crate::validation::fast_affine::FastAffineLane {
    build_fast_affine_lane(FastLaneConfig::new(
        BASE30,
        BASE30_TARGET_OUTER,
        BASE30_TARGET_INNER,
        BASE30_TARGET_MIDDLE_LENGTH,
        BASE30_TARGET_K,
    ))
    .expect("base-30 target lane should fit u64")
}

fn build_pair_heatmap_rows(settings: Base30WheelSettings) -> Vec<Base30PairHeatmapRow> {
    let mut rows = Vec::new();
    for middle_length in settings.min_middle_length..=settings.max_middle_length {
        let pair_rows = ordered_unit_pairs(BASE30)
            .into_iter()
            .map(|(outer, inner)| {
                evaluate_pair_row(BASE30, middle_length, outer, inner, DEFAULT_BOUNDED_K_GRID)
            })
            .collect::<Vec<_>>();
        rows.extend(rank_pair_rows(&pair_rows));
    }
    rows
}

fn rank_pair_rows(rows: &[KDominancePairRow]) -> Vec<Base30PairHeatmapRow> {
    let mut ranked = rows.iter().collect::<Vec<_>>();
    ranked.sort_by(|left, right| {
        right
            .rate_k00
            .total_cmp(&left.rate_k00)
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });

    ranked
        .into_iter()
        .enumerate()
        .map(|(index, row)| Base30PairHeatmapRow {
            base: row.base,
            middle_length: row.middle_length,
            outer: row.outer,
            inner: row.inner,
            pair_label: row.pair_label.clone(),
            candidates_per_config: row.candidates_per_config,
            k00_rank: index + 1,
            rate_k00: row.rate_k00,
            rate_k01: row.rate_k01,
            rate_k10: row.rate_k10,
            rate_k11: row.rate_k11,
            rate_k22: row.rate_k22,
            prime_hits_k00: row.prime_hits_k00,
            best_k: row.best_k.clone(),
            best_rate: row.best_rate,
            k00_noninferior: row.k00_noninferior,
            best_minus_k00_pp: row.best_minus_k00_pp,
        })
        .collect()
}

fn build_length_summary_rows(rows: &[Base30PairHeatmapRow]) -> Vec<Base30LengthSummaryRow> {
    let mut out = Vec::new();
    let mut middle_lengths = rows
        .iter()
        .map(|row| row.middle_length)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    middle_lengths.sort_unstable();

    for middle_length in middle_lengths {
        let group = rows
            .iter()
            .filter(|row| row.middle_length == middle_length)
            .collect::<Vec<_>>();
        let rates = group.iter().map(|row| row.rate_k00).collect::<Vec<_>>();
        let target = group
            .iter()
            .find(|row| row.outer == BASE30_TARGET_OUTER && row.inner == BASE30_TARGET_INNER)
            .expect("target pair should be present");
        out.push(Base30LengthSummaryRow {
            base: BASE30,
            middle_length,
            ordered_pair_count: group.len(),
            mean_k00_rate: mean(&rates),
            median_k00_rate: median(rates),
            min_k00_rate: group
                .iter()
                .map(|row| row.rate_k00)
                .min_by(f64::total_cmp)
                .unwrap_or(0.0),
            max_k00_rate: group
                .iter()
                .map(|row| row.rate_k00)
                .max_by(f64::total_cmp)
                .unwrap_or(0.0),
            target_pair_rank: target.k00_rank,
            target_pair_rate_k00: target.rate_k00,
            noncompact_counterexample_count: group
                .iter()
                .filter(|row| row.best_minus_k00_pp > 0.0)
                .count(),
        });
    }

    out
}

pub fn build_base30_residue_funnel_rows() -> Vec<Base30ResidueFunnelRow> {
    let lane = base30_target_lane();
    let mut rows = Vec::new();
    let mut active_moduli = Vec::new();

    rows.push(residue_funnel_row(0, &lane, &active_moduli, None));
    for &modulus in BASE30_RESIDUE_MODULI {
        active_moduli.push(modulus);
        rows.push(residue_funnel_row(
            active_moduli.len(),
            &lane,
            &active_moduli,
            Some(modulus),
        ));
    }

    rows
}

fn residue_funnel_row(
    stage_index: usize,
    lane: &crate::validation::fast_affine::FastAffineLane,
    moduli: &[u32],
    added_modulus: Option<u32>,
) -> Base30ResidueFunnelRow {
    let survivor_seeds = (0..lane.seed_capacity)
        .filter(|&seed| seed_survives_moduli(lane, seed, moduli))
        .collect::<Vec<_>>();
    let prime_count = survivor_seeds
        .iter()
        .filter(|&&seed| lane.candidate_value(seed).is_some_and(primal::is_prime))
        .count();
    let survivor_count = survivor_seeds.len();
    Base30ResidueFunnelRow {
        stage_index,
        modulus_set_label: if moduli.is_empty() {
            "none".to_string()
        } else {
            moduli
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("|")
        },
        added_modulus,
        added_excluded_seed_classes: added_modulus
            .map(|modulus| excluded_seed_classes_label(lane, modulus))
            .unwrap_or_else(|| "none".to_string()),
        survivor_count,
        survivor_share: survivor_count as f64 / lane.seed_capacity as f64,
        prime_count,
        prime_rate_among_survivors: if survivor_count == 0 {
            0.0
        } else {
            prime_count as f64 / survivor_count as f64
        },
    }
}

fn seed_survives_moduli(
    lane: &crate::validation::fast_affine::FastAffineLane,
    seed: u64,
    moduli: &[u32],
) -> bool {
    moduli.iter().copied().all(|modulus| {
        !((lane.shift % u64::from(modulus))
            + (lane.gradient % u64::from(modulus)) * (seed % u64::from(modulus)))
        .is_multiple_of(u64::from(modulus))
    })
}

fn excluded_seed_classes_label(
    lane: &crate::validation::fast_affine::FastAffineLane,
    modulus: u32,
) -> String {
    let classes = (0..modulus)
        .filter(|&seed_class| {
            ((lane.shift % u64::from(modulus))
                + (lane.gradient % u64::from(modulus)) * u64::from(seed_class))
            .is_multiple_of(u64::from(modulus))
        })
        .map(|seed_class| seed_class.to_string())
        .collect::<Vec<_>>();
    if classes.is_empty() {
        "none".to_string()
    } else {
        classes.join("|")
    }
}

fn build_top_pair_rows(rows: &[Base30PairHeatmapRow]) -> Vec<Base30TopPairRow> {
    let m2_rows = rows
        .iter()
        .filter(|row| row.middle_length == BASE30_TARGET_MIDDLE_LENGTH)
        .collect::<Vec<_>>();
    let mut selected_keys = BTreeSet::new();
    let mut selected = Vec::new();

    for row in m2_rows.iter().take(8) {
        selected_keys.insert((row.outer, row.inner));
        selected.push(top_pair_row(row, "top8"));
    }

    if !selected_keys.contains(&(BASE30_TARGET_OUTER, BASE30_TARGET_INNER)) {
        let target = m2_rows
            .iter()
            .find(|row| row.outer == BASE30_TARGET_OUTER && row.inner == BASE30_TARGET_INNER)
            .expect("target pair should be present");
        selected.push(top_pair_row(target, "canonical_walkthrough"));
    }

    selected
}

fn top_pair_row(row: &Base30PairHeatmapRow, reason: &str) -> Base30TopPairRow {
    Base30TopPairRow {
        selection_reason: reason.to_string(),
        k00_rank: row.k00_rank,
        pair_label: row.pair_label.clone(),
        outer: row.outer,
        inner: row.inner,
        rate_k00: row.rate_k00,
        prime_hits_k00: row.prime_hits_k00,
        best_k: row.best_k.clone(),
        best_rate: row.best_rate,
        k00_noninferior: row.k00_noninferior,
    }
}

fn build_witness_rows(
    top_pair_rows: &[Base30TopPairRow],
    witness_limit: usize,
) -> Vec<Base30WitnessRow> {
    let mut rows = Vec::new();
    for pair in top_pair_rows {
        let (_, examples) = scan_k_config_examples(
            BASE30,
            BASE30_TARGET_MIDDLE_LENGTH,
            pair.outer,
            pair.inner,
            BASE30_TARGET_K,
            witness_limit,
        );
        for example in examples {
            rows.push(Base30WitnessRow {
                pair_label: pair.pair_label.clone(),
                outer: pair.outer,
                inner: pair.inner,
                middle_length: BASE30_TARGET_MIDDLE_LENGTH,
                k_label: "k=(0,0)".to_string(),
                seed: example.middle_index,
                middle_digits: example.middle_digits.clone(),
                template_digits: template_digits(pair.outer, pair.inner, &example.middle_digits),
                decimal_value: example.decimal_value,
            });
        }
    }
    rows
}

fn template_digits(outer: u32, inner: u32, middle_digits: &str) -> String {
    format!(
        "{}{}{}{}{}",
        digit_symbol(outer),
        digit_symbol(inner),
        middle_digits,
        digit_symbol(inner),
        digit_symbol(outer)
    )
}

fn build_summary(
    rows: &[Base30PairHeatmapRow],
    length_summary_rows: &[Base30LengthSummaryRow],
) -> Base30WheelSummary {
    let lane = base30_target_lane();
    let m2_rows = rows
        .iter()
        .filter(|row| row.middle_length == BASE30_TARGET_MIDDLE_LENGTH)
        .collect::<Vec<_>>();
    let target_m2 = m2_rows
        .iter()
        .find(|row| row.outer == BASE30_TARGET_OUTER && row.inner == BASE30_TARGET_INNER)
        .expect("target pair should be present");
    let top_m2 = m2_rows
        .iter()
        .min_by_key(|row| row.k00_rank)
        .expect("M=2 rows should exist");
    let counterexamples = |middle_length| {
        length_summary_rows
            .iter()
            .find(|row| row.middle_length == middle_length)
            .map(|row| row.noncompact_counterexample_count)
            .unwrap_or(0)
    };

    Base30WheelSummary {
        target_pair_label: target_m2.pair_label.clone(),
        target_shift: lane.shift,
        target_gradient: lane.gradient,
        target_seed_capacity: lane.seed_capacity,
        target_m2_rate_k00: target_m2.rate_k00,
        target_m2_rank: target_m2.k00_rank,
        top_m2_pair: top_m2.pair_label.clone(),
        top_m2_rate_k00: top_m2.rate_k00,
        m2_noncompact_counterexamples: counterexamples(2),
        m3_noncompact_counterexamples: counterexamples(3),
        strong_line: "base 30 is a clean wheel-compressed affine candidate surface.".to_string(),
        caution_line: "this is a gorgeous classical wheel effect, not yet residual density magic."
            .to_string(),
    }
}

fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<f64>() / values.len() as f64
    }
}

fn median(mut values: Vec<f64>) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.sort_by(f64::total_cmp);
    let middle = values.len() / 2;
    if values.len().is_multiple_of(2) {
        (values[middle - 1] + values[middle]) / 2.0
    } else {
        values[middle]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::bounded_k::to_base_string_fixed;

    #[test]
    fn base30_canonical_affine_arithmetic_matches_witness() {
        let lane = base30_target_lane();
        assert_eq!(lane.shift, 272_970_221);
        assert_eq!(lane.gradient, 900);
        assert_eq!(lane.seed_capacity, 900);
        assert_eq!(lane.middle_digits(1), "01");
        assert_eq!(lane.template_digits(1), "B7017B");
        assert_eq!(lane.candidate_value(1), Some(272_971_121));
        assert!(primal::is_prime(272_971_121));
        assert_eq!(to_base_string_fixed(1, BASE30, 2), "01");
    }

    #[test]
    fn residue_funnel_preserves_prime_witnesses() {
        let rows = build_base30_residue_funnel_rows();
        let first_five = rows
            .iter()
            .find(|row| row.modulus_set_label == "7|11|13|17|19")
            .expect("first five row");
        assert_eq!(first_five.survivor_count, 580);
        assert_eq!(first_five.prime_count, 171);

        let full = rows.last().expect("full row");
        assert_eq!(full.modulus_set_label, "7|11|13|17|19|23|29|31");
        assert_eq!(full.survivor_count, 516);
        assert_eq!(full.prime_count, 171);
    }

    #[test]
    fn pair_ranking_covers_base30_surface_and_keeps_k00_noninferior() {
        let report = build_base30_wheel_report(Base30WheelSettings {
            min_middle_length: 2,
            max_middle_length: 2,
            witness_limit: 1,
        });
        let m2_rows = report
            .pair_heatmap_rows
            .iter()
            .filter(|row| row.middle_length == 2)
            .collect::<Vec<_>>();
        assert_eq!(m2_rows.len(), 64);

        let target = m2_rows
            .iter()
            .find(|row| row.outer == BASE30_TARGET_OUTER && row.inner == BASE30_TARGET_INNER)
            .expect("(B,7) row");
        assert!(target.k00_noninferior);
        assert_eq!(target.prime_hits_k00, 171);

        assert!(m2_rows.iter().all(|row| row.best_minus_k00_pp == 0.0));
    }

    #[test]
    #[ignore = "exact full M=3 base-30 surface is release-report scale"]
    fn exact_m3_full_surface_has_no_noncompact_anomaly() {
        let report = build_base30_wheel_report(Base30WheelSettings {
            min_middle_length: 3,
            max_middle_length: 3,
            witness_limit: 1,
        });
        assert!(report
            .pair_heatmap_rows
            .iter()
            .all(|row| row.best_minus_k00_pp == 0.0));
    }
}
