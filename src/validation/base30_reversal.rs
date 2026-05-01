//! Base-30 ordered-pair reversal asymmetry helpers.
//!
//! This module studies the compact `k=(0,0)` base-30 surface by comparing each
//! ordered pair `(outer, inner)` with its reversal `(inner, outer)`. Swapping
//! roles preserves the compact grammar and gradient, but changes the affine
//! shift, so the two lanes can enter later residue gates in different phases.

use crate::validation::{
    base30_wheel::BASE30,
    bounded_k::{digit_symbol, unit_residues},
    fast_affine::{build_fast_affine_lane, FastAffineLane, FastLaneConfig},
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

pub const BASE30_REVERSAL_FOCUS_OUTER: u32 = 1;
pub const BASE30_REVERSAL_FOCUS_INNER: u32 = 11;
pub const BASE30_REVERSAL_K: (u32, u32) = (0, 0);
pub const BASE30_REVERSAL_RESIDUE_MODULI: &[u32] = &[7, 11, 13, 17, 19, 23, 29, 31];
pub const DEFAULT_BASE30_REVERSAL_MIN_MIDDLE_LENGTH: usize = 1;
pub const DEFAULT_BASE30_REVERSAL_MAX_MIDDLE_LENGTH: usize = 3;
pub const DEFAULT_BASE30_REVERSAL_WITNESS_LIMIT: usize = 4;
pub const DEFAULT_BASE30_REVERSAL_TOP_LIMIT: usize = 8;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Base30ReversalSettings {
    pub min_middle_length: usize,
    pub max_middle_length: usize,
    pub witness_limit: usize,
    pub top_limit: usize,
}

impl Default for Base30ReversalSettings {
    fn default() -> Self {
        Self {
            min_middle_length: DEFAULT_BASE30_REVERSAL_MIN_MIDDLE_LENGTH,
            max_middle_length: DEFAULT_BASE30_REVERSAL_MAX_MIDDLE_LENGTH,
            witness_limit: DEFAULT_BASE30_REVERSAL_WITNESS_LIMIT,
            top_limit: DEFAULT_BASE30_REVERSAL_TOP_LIMIT,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30OrderedPairRateRow {
    pub base: u32,
    pub middle_length: usize,
    pub outer: u32,
    pub inner: u32,
    pub pair_label: String,
    pub seed_capacity: u64,
    pub compact_rank: usize,
    pub compact_prime_hits: usize,
    pub compact_prime_rate: f64,
    pub shift: u64,
    pub gradient: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30ReversalPairRow {
    pub base: u32,
    pub middle_length: usize,
    pub outer: u32,
    pub inner: u32,
    pub pair_label: String,
    pub reverse_pair_label: String,
    pub seed_capacity: u64,
    pub forward_hits: usize,
    pub reverse_hits: usize,
    pub forward_rate: f64,
    pub reverse_rate: f64,
    pub hit_delta: i64,
    pub rate_delta_pp: f64,
    pub abs_rate_delta_pp: f64,
    pub forward_rank: usize,
    pub reverse_rank: usize,
    pub shift_delta: i128,
    pub formula_shift_delta: i128,
    pub dominant_pair_label: String,
    pub weaker_pair_label: String,
    pub is_diagonal: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30ReversalLengthSummaryRow {
    pub base: u32,
    pub middle_length: usize,
    pub ordered_pair_count: usize,
    pub unique_reversal_pair_count: usize,
    pub mean_abs_delta_pp: f64,
    pub median_abs_delta_pp: f64,
    pub max_abs_delta_pp: f64,
    pub strongest_dominant_pair: String,
    pub strongest_weaker_pair: String,
    pub strongest_delta_pp: f64,
    pub focus_pair_label: String,
    pub focus_reverse_pair_label: String,
    pub focus_forward_hits: usize,
    pub focus_reverse_hits: usize,
    pub focus_forward_rate: f64,
    pub focus_reverse_rate: f64,
    pub focus_delta_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30TopAsymmetryRow {
    pub selection_reason: String,
    pub middle_length: usize,
    pub pair_label: String,
    pub reverse_pair_label: String,
    pub dominant_pair_label: String,
    pub weaker_pair_label: String,
    pub forward_hits: usize,
    pub reverse_hits: usize,
    pub forward_rate: f64,
    pub reverse_rate: f64,
    pub rate_delta_pp: f64,
    pub abs_rate_delta_pp: f64,
    pub shift_delta: i128,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30ResiduePhaseRow {
    pub middle_length: usize,
    pub pair_label: String,
    pub outer: u32,
    pub inner: u32,
    pub modulus: u32,
    pub shift_modulus: u32,
    pub gradient_modulus: u32,
    pub excluded_seed_classes: String,
    pub survivor_count: usize,
    pub survivor_share: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30ReversalWitnessRow {
    pub selection_reason: String,
    pub middle_length: usize,
    pub pair_label: String,
    pub outer: u32,
    pub inner: u32,
    pub seed: u64,
    pub middle_digits: String,
    pub template_digits: String,
    pub decimal_value: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30ReversalSummary {
    pub focus_pair_label: String,
    pub focus_reverse_pair_label: String,
    pub focus_m2_delta_pp: f64,
    pub focus_m3_delta_pp: f64,
    pub focus_m3_forward_hits: usize,
    pub focus_m3_reverse_hits: usize,
    pub strongest_m3_dominant_pair: String,
    pub strongest_m3_weaker_pair: String,
    pub strongest_m3_abs_delta_pp: f64,
    pub strong_line: String,
    pub caution_line: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30ReversalReport {
    pub settings: Base30ReversalSettings,
    pub summary: Base30ReversalSummary,
    pub ordered_pair_rows: Vec<Base30OrderedPairRateRow>,
    pub reversal_pair_rows: Vec<Base30ReversalPairRow>,
    pub length_summary_rows: Vec<Base30ReversalLengthSummaryRow>,
    pub top_asymmetry_rows: Vec<Base30TopAsymmetryRow>,
    pub residue_phase_rows: Vec<Base30ResiduePhaseRow>,
    pub witness_rows: Vec<Base30ReversalWitnessRow>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30ReversalResidualRow {
    pub base: u32,
    pub middle_length: usize,
    pub low_digit: u32,
    pub high_digit: u32,
    pub low_high_pair_label: String,
    pub high_low_pair_label: String,
    pub seed_capacity: u64,
    pub gradient: u64,
    pub low_high_shift: u64,
    pub high_low_shift: u64,
    pub shift_delta: i128,
    pub formula_shift_delta: i128,
    pub low_high_prime_hits: usize,
    pub high_low_prime_hits: usize,
    pub low_high_prime_rate: f64,
    pub high_low_prime_rate: f64,
    pub raw_delta_pp: f64,
    pub abs_raw_delta_pp: f64,
    pub low_high_average_ln: f64,
    pub high_low_average_ln: f64,
    pub low_high_pnt_expected_density: f64,
    pub high_low_pnt_expected_density: f64,
    pub size_expected_delta_pp: f64,
    pub residual_after_size_pp: f64,
    pub abs_residual_after_size_pp: f64,
    pub residue_moduli_label: String,
    pub low_high_survivor_count: usize,
    pub high_low_survivor_count: usize,
    pub low_high_survivor_share: f64,
    pub high_low_survivor_share: f64,
    pub residue_survivor_delta_pp: f64,
    pub low_high_prime_rate_among_survivors: f64,
    pub high_low_prime_rate_among_survivors: f64,
    pub survivor_prime_residual_delta_pp: f64,
    pub first_low_high_witness: String,
    pub first_high_low_witness: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30ReversalResidueDeltaRow {
    pub base: u32,
    pub middle_length: usize,
    pub low_digit: u32,
    pub high_digit: u32,
    pub low_high_pair_label: String,
    pub high_low_pair_label: String,
    pub modulus_set_label: String,
    pub low_high_excluded_seed_classes: String,
    pub high_low_excluded_seed_classes: String,
    pub low_high_survivor_count: usize,
    pub high_low_survivor_count: usize,
    pub low_high_survivor_share: f64,
    pub high_low_survivor_share: f64,
    pub survivor_delta_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30TopResidualRow {
    pub selection_reason: String,
    pub middle_length: usize,
    pub low_digit: u32,
    pub high_digit: u32,
    pub low_high_pair_label: String,
    pub high_low_pair_label: String,
    pub raw_delta_pp: f64,
    pub size_expected_delta_pp: f64,
    pub residual_after_size_pp: f64,
    pub survivor_prime_residual_delta_pp: f64,
    pub low_high_prime_hits: usize,
    pub high_low_prime_hits: usize,
    pub low_high_prime_rate: f64,
    pub high_low_prime_rate: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30ReversalResidualSummary {
    pub focus_pair_label: String,
    pub focus_reverse_pair_label: String,
    pub focus_m2_raw_delta_pp: f64,
    pub focus_m3_raw_delta_pp: f64,
    pub focus_m3_size_expected_delta_pp: f64,
    pub focus_m3_residual_after_size_pp: f64,
    pub focus_m3_survivor_prime_residual_delta_pp: f64,
    pub strongest_m3_residual_pair: String,
    pub strongest_m3_reverse_pair: String,
    pub strongest_m3_residual_after_size_pp: f64,
    pub mean_abs_m3_raw_delta_pp: f64,
    pub mean_abs_m3_residual_after_size_pp: f64,
    pub strong_line: String,
    pub caution_line: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base30ReversalResidualReport {
    pub settings: Base30ReversalSettings,
    pub summary: Base30ReversalResidualSummary,
    pub residual_rows: Vec<Base30ReversalResidualRow>,
    pub residue_delta_rows: Vec<Base30ReversalResidueDeltaRow>,
    pub top_residual_rows: Vec<Base30TopResidualRow>,
    pub witness_rows: Vec<Base30ReversalWitnessRow>,
}

pub fn build_base30_reversal_report(settings: Base30ReversalSettings) -> Base30ReversalReport {
    let ordered_pair_rows = build_ordered_pair_rows(settings);
    let reversal_pair_rows = build_reversal_pair_rows(&ordered_pair_rows);
    let length_summary_rows = build_length_summary_rows(&reversal_pair_rows);
    let top_asymmetry_rows = build_top_asymmetry_rows(&reversal_pair_rows, settings);
    let residue_phase_rows = build_residue_phase_rows(&top_asymmetry_rows);
    let witness_rows = build_witness_rows(&top_asymmetry_rows, settings.witness_limit);
    let summary = build_summary(&length_summary_rows);

    Base30ReversalReport {
        settings,
        summary,
        ordered_pair_rows,
        reversal_pair_rows,
        length_summary_rows,
        top_asymmetry_rows,
        residue_phase_rows,
        witness_rows,
    }
}

pub fn build_base30_reversal_residual_report(
    settings: Base30ReversalSettings,
) -> Base30ReversalResidualReport {
    let residual_rows = build_reversal_residual_rows(settings);
    let residue_delta_rows = build_reversal_residue_delta_rows(&residual_rows);
    let top_residual_rows = build_top_residual_rows(&residual_rows, settings);
    let witness_rows = build_residual_witness_rows(&top_residual_rows, settings.witness_limit);
    let summary = build_residual_summary(&residual_rows);

    Base30ReversalResidualReport {
        settings,
        summary,
        residual_rows,
        residue_delta_rows,
        top_residual_rows,
        witness_rows,
    }
}

pub fn base30_compact_prime_hits(middle_length: usize, outer: u32, inner: u32) -> usize {
    compact_prime_scan(middle_length, outer, inner, 0).0
}

pub fn compact_reversal_shift_delta(middle_length: usize, outer: u32, inner: u32) -> i128 {
    let base = i128::from(BASE30);
    let digit_delta = i128::from(outer) - i128::from(inner);
    digit_delta * (base - 1) * (pow_i128(base, middle_length + 2) - 1)
}

fn build_ordered_pair_rows(settings: Base30ReversalSettings) -> Vec<Base30OrderedPairRateRow> {
    let units = unit_residues(BASE30);
    let mut rows = Vec::new();

    for middle_length in settings.min_middle_length..=settings.max_middle_length {
        let mut group = Vec::new();
        for &outer in &units {
            for &inner in &units {
                let lane = compact_lane(middle_length, outer, inner);
                let hits = compact_prime_scan(middle_length, outer, inner, 0).0;
                group.push(Base30OrderedPairRateRow {
                    base: BASE30,
                    middle_length,
                    outer,
                    inner,
                    pair_label: pair_label(outer, inner),
                    seed_capacity: lane.seed_capacity,
                    compact_rank: 0,
                    compact_prime_hits: hits,
                    compact_prime_rate: hits as f64 / lane.seed_capacity as f64,
                    shift: lane.shift,
                    gradient: lane.gradient,
                });
            }
        }

        group.sort_by(|left, right| {
            right
                .compact_prime_rate
                .total_cmp(&left.compact_prime_rate)
                .then_with(|| left.pair_label.cmp(&right.pair_label))
        });
        for (idx, row) in group.iter_mut().enumerate() {
            row.compact_rank = idx + 1;
        }
        rows.extend(group);
    }

    rows
}

fn build_reversal_pair_rows(
    ordered_rows: &[Base30OrderedPairRateRow],
) -> Vec<Base30ReversalPairRow> {
    let by_key = ordered_rows
        .iter()
        .map(|row| ((row.middle_length, row.outer, row.inner), row))
        .collect::<BTreeMap<_, _>>();

    ordered_rows
        .iter()
        .map(|row| {
            let reverse = by_key
                .get(&(row.middle_length, row.inner, row.outer))
                .expect("reverse row should exist");
            let hit_delta = row.compact_prime_hits as i64 - reverse.compact_prime_hits as i64;
            let rate_delta_pp = (row.compact_prime_rate - reverse.compact_prime_rate) * 100.0;
            let shift_delta = row.shift as i128 - reverse.shift as i128;
            let formula_shift_delta =
                compact_reversal_shift_delta(row.middle_length, row.outer, row.inner);
            let (dominant_pair_label, weaker_pair_label) = if hit_delta >= 0 {
                (row.pair_label.clone(), reverse.pair_label.clone())
            } else {
                (reverse.pair_label.clone(), row.pair_label.clone())
            };
            Base30ReversalPairRow {
                base: BASE30,
                middle_length: row.middle_length,
                outer: row.outer,
                inner: row.inner,
                pair_label: row.pair_label.clone(),
                reverse_pair_label: reverse.pair_label.clone(),
                seed_capacity: row.seed_capacity,
                forward_hits: row.compact_prime_hits,
                reverse_hits: reverse.compact_prime_hits,
                forward_rate: row.compact_prime_rate,
                reverse_rate: reverse.compact_prime_rate,
                hit_delta,
                rate_delta_pp,
                abs_rate_delta_pp: rate_delta_pp.abs(),
                forward_rank: row.compact_rank,
                reverse_rank: reverse.compact_rank,
                shift_delta,
                formula_shift_delta,
                dominant_pair_label,
                weaker_pair_label,
                is_diagonal: row.outer == row.inner,
            }
        })
        .collect()
}

fn build_length_summary_rows(
    reversal_rows: &[Base30ReversalPairRow],
) -> Vec<Base30ReversalLengthSummaryRow> {
    let middle_lengths = reversal_rows
        .iter()
        .map(|row| row.middle_length)
        .collect::<BTreeSet<_>>();
    let mut rows = Vec::new();

    for middle_length in middle_lengths {
        let unique_rows = unique_non_diagonal_rows(reversal_rows, middle_length);
        let mut abs_deltas = unique_rows
            .iter()
            .map(|row| row.abs_rate_delta_pp)
            .collect::<Vec<_>>();
        let strongest = unique_rows
            .iter()
            .max_by(|left, right| left.abs_rate_delta_pp.total_cmp(&right.abs_rate_delta_pp))
            .expect("unique reversal rows should exist");
        let focus = reversal_rows
            .iter()
            .find(|row| {
                row.middle_length == middle_length
                    && row.outer == BASE30_REVERSAL_FOCUS_OUTER
                    && row.inner == BASE30_REVERSAL_FOCUS_INNER
            })
            .expect("focus row should exist");
        let unique_reversal_pair_count = unique_rows.len();
        rows.push(Base30ReversalLengthSummaryRow {
            base: BASE30,
            middle_length,
            ordered_pair_count: reversal_rows
                .iter()
                .filter(|row| row.middle_length == middle_length)
                .count(),
            unique_reversal_pair_count,
            mean_abs_delta_pp: mean(&abs_deltas),
            median_abs_delta_pp: median(abs_deltas.split_off(0)),
            max_abs_delta_pp: strongest.abs_rate_delta_pp,
            strongest_dominant_pair: strongest.dominant_pair_label.clone(),
            strongest_weaker_pair: strongest.weaker_pair_label.clone(),
            strongest_delta_pp: strongest.rate_delta_pp,
            focus_pair_label: focus.pair_label.clone(),
            focus_reverse_pair_label: focus.reverse_pair_label.clone(),
            focus_forward_hits: focus.forward_hits,
            focus_reverse_hits: focus.reverse_hits,
            focus_forward_rate: focus.forward_rate,
            focus_reverse_rate: focus.reverse_rate,
            focus_delta_pp: focus.rate_delta_pp,
        });
    }

    rows
}

fn build_top_asymmetry_rows(
    reversal_rows: &[Base30ReversalPairRow],
    settings: Base30ReversalSettings,
) -> Vec<Base30TopAsymmetryRow> {
    let middle_length = settings.max_middle_length;
    let mut unique_rows = unique_non_diagonal_rows(reversal_rows, middle_length);
    unique_rows.sort_by(|left, right| {
        right
            .abs_rate_delta_pp
            .total_cmp(&left.abs_rate_delta_pp)
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });

    let mut selected_keys = BTreeSet::new();
    let mut selected = Vec::new();
    for row in unique_rows.iter().take(settings.top_limit) {
        selected_keys.insert(unordered_pair_key(row.outer, row.inner));
        selected.push(top_asymmetry_row(row, "top_abs_delta"));
    }

    let focus_key = unordered_pair_key(BASE30_REVERSAL_FOCUS_OUTER, BASE30_REVERSAL_FOCUS_INNER);
    if !selected_keys.contains(&focus_key) {
        let focus = reversal_rows
            .iter()
            .find(|row| {
                row.middle_length == middle_length
                    && row.outer == BASE30_REVERSAL_FOCUS_OUTER
                    && row.inner == BASE30_REVERSAL_FOCUS_INNER
            })
            .expect("focus reversal row should exist");
        selected.push(top_asymmetry_row(focus, "focus_pair"));
    }

    selected
}

fn top_asymmetry_row(row: &Base30ReversalPairRow, reason: &str) -> Base30TopAsymmetryRow {
    Base30TopAsymmetryRow {
        selection_reason: reason.to_string(),
        middle_length: row.middle_length,
        pair_label: row.pair_label.clone(),
        reverse_pair_label: row.reverse_pair_label.clone(),
        dominant_pair_label: row.dominant_pair_label.clone(),
        weaker_pair_label: row.weaker_pair_label.clone(),
        forward_hits: row.forward_hits,
        reverse_hits: row.reverse_hits,
        forward_rate: row.forward_rate,
        reverse_rate: row.reverse_rate,
        rate_delta_pp: row.rate_delta_pp,
        abs_rate_delta_pp: row.abs_rate_delta_pp,
        shift_delta: row.shift_delta,
    }
}

fn build_residue_phase_rows(top_rows: &[Base30TopAsymmetryRow]) -> Vec<Base30ResiduePhaseRow> {
    let mut rows = Vec::new();
    let mut seen = BTreeSet::new();
    for top_row in top_rows {
        let Some((outer, inner)) = parse_base30_pair_label(&top_row.pair_label) else {
            continue;
        };
        for (pair_outer, pair_inner) in [(outer, inner), (inner, outer)] {
            if !seen.insert((top_row.middle_length, pair_outer, pair_inner)) {
                continue;
            }
            let lane = compact_lane(top_row.middle_length, pair_outer, pair_inner);
            for &modulus in BASE30_REVERSAL_RESIDUE_MODULI {
                let survivor_count = (0..lane.seed_capacity)
                    .filter(|&seed| seed_survives_modulus(&lane, seed, modulus))
                    .count();
                rows.push(Base30ResiduePhaseRow {
                    middle_length: top_row.middle_length,
                    pair_label: pair_label(pair_outer, pair_inner),
                    outer: pair_outer,
                    inner: pair_inner,
                    modulus,
                    shift_modulus: (lane.shift % u64::from(modulus)) as u32,
                    gradient_modulus: (lane.gradient % u64::from(modulus)) as u32,
                    excluded_seed_classes: excluded_seed_classes_label(&lane, modulus),
                    survivor_count,
                    survivor_share: survivor_count as f64 / lane.seed_capacity as f64,
                });
            }
        }
    }
    rows
}

fn build_witness_rows(
    top_rows: &[Base30TopAsymmetryRow],
    witness_limit: usize,
) -> Vec<Base30ReversalWitnessRow> {
    let mut rows = Vec::new();
    let mut seen = BTreeSet::new();
    for top_row in top_rows {
        let Some((outer, inner)) = parse_base30_pair_label(&top_row.pair_label) else {
            continue;
        };
        for (pair_outer, pair_inner) in [(outer, inner), (inner, outer)] {
            if !seen.insert((top_row.middle_length, pair_outer, pair_inner)) {
                continue;
            }
            let (_, witnesses) =
                compact_prime_scan(top_row.middle_length, pair_outer, pair_inner, witness_limit);
            for witness in witnesses {
                rows.push(Base30ReversalWitnessRow {
                    selection_reason: top_row.selection_reason.clone(),
                    middle_length: top_row.middle_length,
                    pair_label: pair_label(pair_outer, pair_inner),
                    outer: pair_outer,
                    inner: pair_inner,
                    seed: witness.seed,
                    middle_digits: witness.middle_digits,
                    template_digits: witness.template_digits,
                    decimal_value: witness.decimal_value,
                });
            }
        }
    }
    rows
}

fn build_summary(length_rows: &[Base30ReversalLengthSummaryRow]) -> Base30ReversalSummary {
    let focus_m2 = length_rows
        .iter()
        .find(|row| row.middle_length == 2)
        .or_else(|| length_rows.first())
        .expect("at least one summary row should exist");
    let focus_m3 = length_rows
        .iter()
        .find(|row| row.middle_length == 3)
        .or_else(|| length_rows.last())
        .expect("at least one summary row should exist");
    Base30ReversalSummary {
        focus_pair_label: focus_m3.focus_pair_label.clone(),
        focus_reverse_pair_label: focus_m3.focus_reverse_pair_label.clone(),
        focus_m2_delta_pp: focus_m2.focus_delta_pp,
        focus_m3_delta_pp: focus_m3.focus_delta_pp,
        focus_m3_forward_hits: focus_m3.focus_forward_hits,
        focus_m3_reverse_hits: focus_m3.focus_reverse_hits,
        strongest_m3_dominant_pair: focus_m3.strongest_dominant_pair.clone(),
        strongest_m3_weaker_pair: focus_m3.strongest_weaker_pair.clone(),
        strongest_m3_abs_delta_pp: focus_m3.max_abs_delta_pp,
        strong_line:
            "base-30 ordered pairs can share a wheel alphabet while entering later residue gates in different affine phases."
                .to_string(),
        caution_line:
            "reversal asymmetry is a local residue-phase signal, not a standalone density theorem."
                .to_string(),
    }
}

#[derive(Debug, Clone)]
struct CompactWitness {
    seed: u64,
    middle_digits: String,
    template_digits: String,
    decimal_value: u64,
}

#[derive(Debug, Clone)]
struct ResidualLaneStats {
    lane: FastAffineLane,
    prime_hits: usize,
    prime_rate: f64,
    average_ln: f64,
    pnt_expected_density: f64,
    survivor_count: usize,
    survivor_share: f64,
    prime_rate_among_survivors: f64,
    first_witness: String,
}

fn build_reversal_residual_rows(
    settings: Base30ReversalSettings,
) -> Vec<Base30ReversalResidualRow> {
    let units = unit_residues(BASE30);
    let residue_moduli_label = moduli_label(BASE30_REVERSAL_RESIDUE_MODULI);
    let mut rows = Vec::new();

    for middle_length in settings.min_middle_length..=settings.max_middle_length {
        for (left_idx, &low_digit) in units.iter().enumerate() {
            for &high_digit in units.iter().skip(left_idx + 1) {
                let low_high = residual_lane_stats(middle_length, low_digit, high_digit, 1);
                let high_low = residual_lane_stats(middle_length, high_digit, low_digit, 1);
                let raw_delta_pp = (low_high.prime_rate - high_low.prime_rate) * 100.0;
                let size_expected_delta_pp =
                    (low_high.pnt_expected_density - high_low.pnt_expected_density) * 100.0;
                let residual_after_size_pp = raw_delta_pp - size_expected_delta_pp;
                let survivor_prime_residual_delta_pp = (low_high.prime_rate_among_survivors
                    - high_low.prime_rate_among_survivors)
                    * 100.0;

                rows.push(Base30ReversalResidualRow {
                    base: BASE30,
                    middle_length,
                    low_digit,
                    high_digit,
                    low_high_pair_label: pair_label(low_digit, high_digit),
                    high_low_pair_label: pair_label(high_digit, low_digit),
                    seed_capacity: low_high.lane.seed_capacity,
                    gradient: low_high.lane.gradient,
                    low_high_shift: low_high.lane.shift,
                    high_low_shift: high_low.lane.shift,
                    shift_delta: low_high.lane.shift as i128 - high_low.lane.shift as i128,
                    formula_shift_delta: compact_reversal_shift_delta(
                        middle_length,
                        low_digit,
                        high_digit,
                    ),
                    low_high_prime_hits: low_high.prime_hits,
                    high_low_prime_hits: high_low.prime_hits,
                    low_high_prime_rate: low_high.prime_rate,
                    high_low_prime_rate: high_low.prime_rate,
                    raw_delta_pp,
                    abs_raw_delta_pp: raw_delta_pp.abs(),
                    low_high_average_ln: low_high.average_ln,
                    high_low_average_ln: high_low.average_ln,
                    low_high_pnt_expected_density: low_high.pnt_expected_density,
                    high_low_pnt_expected_density: high_low.pnt_expected_density,
                    size_expected_delta_pp,
                    residual_after_size_pp,
                    abs_residual_after_size_pp: residual_after_size_pp.abs(),
                    residue_moduli_label: residue_moduli_label.clone(),
                    low_high_survivor_count: low_high.survivor_count,
                    high_low_survivor_count: high_low.survivor_count,
                    low_high_survivor_share: low_high.survivor_share,
                    high_low_survivor_share: high_low.survivor_share,
                    residue_survivor_delta_pp: (low_high.survivor_share - high_low.survivor_share)
                        * 100.0,
                    low_high_prime_rate_among_survivors: low_high.prime_rate_among_survivors,
                    high_low_prime_rate_among_survivors: high_low.prime_rate_among_survivors,
                    survivor_prime_residual_delta_pp,
                    first_low_high_witness: low_high.first_witness,
                    first_high_low_witness: high_low.first_witness,
                });
            }
        }
    }

    rows
}

fn residual_lane_stats(
    middle_length: usize,
    outer: u32,
    inner: u32,
    witness_limit: usize,
) -> ResidualLaneStats {
    let lane = compact_lane(middle_length, outer, inner);
    let mut prime_hits = 0usize;
    let mut survivor_count = 0usize;
    let mut survivor_prime_hits = 0usize;
    let mut sum_ln = 0.0;
    let mut first_witness = String::new();

    for seed in 0..lane.seed_capacity {
        let value = lane
            .candidate_value(seed)
            .expect("compact base-30 report lane should fit u64");
        sum_ln += (value as f64).ln();
        let is_survivor = seed_survives_moduli(&lane, seed, BASE30_REVERSAL_RESIDUE_MODULI);
        if is_survivor {
            survivor_count += 1;
        }
        if primal::is_prime(value) {
            prime_hits += 1;
            if is_survivor {
                survivor_prime_hits += 1;
            }
            if first_witness.is_empty() && witness_limit > 0 {
                first_witness = lane.template_digits(seed);
            }
        }
    }

    let seed_capacity = lane.seed_capacity as f64;
    let average_ln = if lane.seed_capacity == 0 {
        0.0
    } else {
        sum_ln / seed_capacity
    };
    let pnt_expected_density = if average_ln > 0.0 {
        1.0 / average_ln
    } else {
        0.0
    };

    ResidualLaneStats {
        lane,
        prime_hits,
        prime_rate: prime_hits as f64 / seed_capacity,
        average_ln,
        pnt_expected_density,
        survivor_count,
        survivor_share: survivor_count as f64 / seed_capacity,
        prime_rate_among_survivors: if survivor_count == 0 {
            0.0
        } else {
            survivor_prime_hits as f64 / survivor_count as f64
        },
        first_witness,
    }
}

fn build_reversal_residue_delta_rows(
    residual_rows: &[Base30ReversalResidualRow],
) -> Vec<Base30ReversalResidueDeltaRow> {
    let mut rows = Vec::new();
    for row in residual_rows {
        let low_high_lane = compact_lane(row.middle_length, row.low_digit, row.high_digit);
        let high_low_lane = compact_lane(row.middle_length, row.high_digit, row.low_digit);

        for &modulus in BASE30_REVERSAL_RESIDUE_MODULI {
            rows.push(residue_delta_row(
                row,
                &low_high_lane,
                &high_low_lane,
                &[modulus],
                &modulus.to_string(),
            ));
        }

        rows.push(residue_delta_row(
            row,
            &low_high_lane,
            &high_low_lane,
            BASE30_REVERSAL_RESIDUE_MODULI,
            &moduli_label(BASE30_REVERSAL_RESIDUE_MODULI),
        ));
    }

    rows
}

fn residue_delta_row(
    row: &Base30ReversalResidualRow,
    low_high_lane: &FastAffineLane,
    high_low_lane: &FastAffineLane,
    moduli: &[u32],
    modulus_set_label: &str,
) -> Base30ReversalResidueDeltaRow {
    let low_high_survivor_count = survivor_count_for_moduli(low_high_lane, moduli);
    let high_low_survivor_count = survivor_count_for_moduli(high_low_lane, moduli);
    let seed_capacity = row.seed_capacity as f64;
    let low_high_survivor_share = low_high_survivor_count as f64 / seed_capacity;
    let high_low_survivor_share = high_low_survivor_count as f64 / seed_capacity;

    Base30ReversalResidueDeltaRow {
        base: BASE30,
        middle_length: row.middle_length,
        low_digit: row.low_digit,
        high_digit: row.high_digit,
        low_high_pair_label: row.low_high_pair_label.clone(),
        high_low_pair_label: row.high_low_pair_label.clone(),
        modulus_set_label: modulus_set_label.to_string(),
        low_high_excluded_seed_classes: excluded_seed_classes_for_moduli(low_high_lane, moduli),
        high_low_excluded_seed_classes: excluded_seed_classes_for_moduli(high_low_lane, moduli),
        low_high_survivor_count,
        high_low_survivor_count,
        low_high_survivor_share,
        high_low_survivor_share,
        survivor_delta_pp: (low_high_survivor_share - high_low_survivor_share) * 100.0,
    }
}

fn build_top_residual_rows(
    residual_rows: &[Base30ReversalResidualRow],
    settings: Base30ReversalSettings,
) -> Vec<Base30TopResidualRow> {
    let middle_length = settings.max_middle_length;
    let mut ranked = residual_rows
        .iter()
        .filter(|row| row.middle_length == middle_length)
        .collect::<Vec<_>>();
    ranked.sort_by(|left, right| {
        right
            .abs_residual_after_size_pp
            .total_cmp(&left.abs_residual_after_size_pp)
            .then_with(|| left.low_high_pair_label.cmp(&right.low_high_pair_label))
    });

    let mut selected_keys = BTreeSet::new();
    let mut selected = Vec::new();
    for row in ranked.iter().take(settings.top_limit) {
        selected_keys.insert((row.low_digit, row.high_digit));
        selected.push(top_residual_row(row, "top_size_residual"));
    }

    let focus_key = unordered_pair_key(BASE30_REVERSAL_FOCUS_OUTER, BASE30_REVERSAL_FOCUS_INNER);
    if !selected_keys.contains(&focus_key) {
        let focus = residual_rows
            .iter()
            .find(|row| {
                row.middle_length == middle_length
                    && row.low_digit == focus_key.0
                    && row.high_digit == focus_key.1
            })
            .expect("focus residual row should exist");
        selected.push(top_residual_row(focus, "focus_pair"));
    }

    selected
}

fn top_residual_row(row: &Base30ReversalResidualRow, reason: &str) -> Base30TopResidualRow {
    Base30TopResidualRow {
        selection_reason: reason.to_string(),
        middle_length: row.middle_length,
        low_digit: row.low_digit,
        high_digit: row.high_digit,
        low_high_pair_label: row.low_high_pair_label.clone(),
        high_low_pair_label: row.high_low_pair_label.clone(),
        raw_delta_pp: row.raw_delta_pp,
        size_expected_delta_pp: row.size_expected_delta_pp,
        residual_after_size_pp: row.residual_after_size_pp,
        survivor_prime_residual_delta_pp: row.survivor_prime_residual_delta_pp,
        low_high_prime_hits: row.low_high_prime_hits,
        high_low_prime_hits: row.high_low_prime_hits,
        low_high_prime_rate: row.low_high_prime_rate,
        high_low_prime_rate: row.high_low_prime_rate,
    }
}

fn build_residual_witness_rows(
    top_rows: &[Base30TopResidualRow],
    witness_limit: usize,
) -> Vec<Base30ReversalWitnessRow> {
    let mut rows = Vec::new();
    let mut seen = BTreeSet::new();
    for top_row in top_rows {
        for (outer, inner) in [
            (top_row.low_digit, top_row.high_digit),
            (top_row.high_digit, top_row.low_digit),
        ] {
            if !seen.insert((top_row.middle_length, outer, inner)) {
                continue;
            }
            let (_, witnesses) =
                compact_prime_scan(top_row.middle_length, outer, inner, witness_limit);
            for witness in witnesses {
                rows.push(Base30ReversalWitnessRow {
                    selection_reason: top_row.selection_reason.clone(),
                    middle_length: top_row.middle_length,
                    pair_label: pair_label(outer, inner),
                    outer,
                    inner,
                    seed: witness.seed,
                    middle_digits: witness.middle_digits,
                    template_digits: witness.template_digits,
                    decimal_value: witness.decimal_value,
                });
            }
        }
    }
    rows
}

fn build_residual_summary(rows: &[Base30ReversalResidualRow]) -> Base30ReversalResidualSummary {
    let focus_key = unordered_pair_key(BASE30_REVERSAL_FOCUS_OUTER, BASE30_REVERSAL_FOCUS_INNER);
    let focus_m2 = rows
        .iter()
        .find(|row| {
            row.middle_length == 2 && row.low_digit == focus_key.0 && row.high_digit == focus_key.1
        })
        .or_else(|| rows.first())
        .expect("at least one residual row should exist");
    let focus_m3 = rows
        .iter()
        .find(|row| {
            row.middle_length == 3 && row.low_digit == focus_key.0 && row.high_digit == focus_key.1
        })
        .or_else(|| rows.last())
        .expect("at least one residual row should exist");
    let m3_rows = rows
        .iter()
        .filter(|row| row.middle_length == focus_m3.middle_length)
        .collect::<Vec<_>>();
    let strongest = m3_rows
        .iter()
        .max_by(|left, right| {
            left.abs_residual_after_size_pp
                .total_cmp(&right.abs_residual_after_size_pp)
        })
        .expect("residual rows should exist");

    Base30ReversalResidualSummary {
        focus_pair_label: focus_m3.low_high_pair_label.clone(),
        focus_reverse_pair_label: focus_m3.high_low_pair_label.clone(),
        focus_m2_raw_delta_pp: focus_m2.raw_delta_pp,
        focus_m3_raw_delta_pp: focus_m3.raw_delta_pp,
        focus_m3_size_expected_delta_pp: focus_m3.size_expected_delta_pp,
        focus_m3_residual_after_size_pp: focus_m3.residual_after_size_pp,
        focus_m3_survivor_prime_residual_delta_pp: focus_m3.survivor_prime_residual_delta_pp,
        strongest_m3_residual_pair: strongest.low_high_pair_label.clone(),
        strongest_m3_reverse_pair: strongest.high_low_pair_label.clone(),
        strongest_m3_residual_after_size_pp: strongest.residual_after_size_pp,
        mean_abs_m3_raw_delta_pp: mean(
            &m3_rows
                .iter()
                .map(|row| row.abs_raw_delta_pp)
                .collect::<Vec<_>>(),
        ),
        mean_abs_m3_residual_after_size_pp: mean(
            &m3_rows
                .iter()
                .map(|row| row.abs_residual_after_size_pp)
                .collect::<Vec<_>>(),
        ),
        strong_line: "base-30 reversal asymmetry is a measurable local affine phase effect."
            .to_string(),
        caution_line: "the first question is how much is ordinary size and exact residue survival."
            .to_string(),
    }
}

fn compact_prime_scan(
    middle_length: usize,
    outer: u32,
    inner: u32,
    witness_limit: usize,
) -> (usize, Vec<CompactWitness>) {
    let lane = compact_lane(middle_length, outer, inner);
    let mut hits = 0usize;
    let mut witnesses = Vec::new();
    for seed in 0..lane.seed_capacity {
        let value = lane
            .candidate_value(seed)
            .expect("compact base-30 report lane should fit u64");
        if primal::is_prime(value) {
            hits += 1;
            if witnesses.len() < witness_limit {
                witnesses.push(CompactWitness {
                    seed,
                    middle_digits: lane.middle_digits(seed),
                    template_digits: lane.template_digits(seed),
                    decimal_value: value,
                });
            }
        }
    }
    (hits, witnesses)
}

fn compact_lane(middle_length: usize, outer: u32, inner: u32) -> FastAffineLane {
    build_fast_affine_lane(FastLaneConfig::new(
        BASE30,
        outer,
        inner,
        middle_length,
        BASE30_REVERSAL_K,
    ))
    .expect("compact base-30 lane should fit u64 for report scope")
}

fn seed_survives_modulus(lane: &FastAffineLane, seed: u64, modulus: u32) -> bool {
    !((lane.shift % u64::from(modulus))
        + (lane.gradient % u64::from(modulus)) * (seed % u64::from(modulus)))
    .is_multiple_of(u64::from(modulus))
}

fn seed_survives_moduli(lane: &FastAffineLane, seed: u64, moduli: &[u32]) -> bool {
    moduli
        .iter()
        .all(|&modulus| seed_survives_modulus(lane, seed, modulus))
}

fn survivor_count_for_moduli(lane: &FastAffineLane, moduli: &[u32]) -> usize {
    (0..lane.seed_capacity)
        .filter(|&seed| seed_survives_moduli(lane, seed, moduli))
        .count()
}

fn excluded_seed_classes_label(lane: &FastAffineLane, modulus: u32) -> String {
    let classes = (0..modulus)
        .filter(|&seed_class| !seed_survives_modulus(lane, u64::from(seed_class), modulus))
        .map(|seed_class| seed_class.to_string())
        .collect::<Vec<_>>();
    if classes.is_empty() {
        "none".to_string()
    } else {
        classes.join("|")
    }
}

fn excluded_seed_classes_for_moduli(lane: &FastAffineLane, moduli: &[u32]) -> String {
    if moduli.len() == 1 {
        return excluded_seed_classes_label(lane, moduli[0]);
    }
    moduli
        .iter()
        .map(|&modulus| format!("{modulus}:{}", excluded_seed_classes_label(lane, modulus)))
        .collect::<Vec<_>>()
        .join(";")
}

fn moduli_label(moduli: &[u32]) -> String {
    moduli
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn unique_non_diagonal_rows(
    reversal_rows: &[Base30ReversalPairRow],
    middle_length: usize,
) -> Vec<&Base30ReversalPairRow> {
    reversal_rows
        .iter()
        .filter(|row| row.middle_length == middle_length && row.outer < row.inner)
        .collect()
}

fn unordered_pair_key(left: u32, right: u32) -> (u32, u32) {
    if left <= right {
        (left, right)
    } else {
        (right, left)
    }
}

fn pair_label(outer: u32, inner: u32) -> String {
    format!("({},{})", digit_symbol(outer), digit_symbol(inner))
}

fn parse_base30_pair_label(label: &str) -> Option<(u32, u32)> {
    let trimmed = label.strip_prefix('(')?.strip_suffix(')')?;
    let (left, right) = trimmed.split_once(',')?;
    Some((parse_base30_digit(left)?, parse_base30_digit(right)?))
}

fn parse_base30_digit(label: &str) -> Option<u32> {
    let mut chars = label.chars();
    let ch = chars.next()?;
    if chars.next().is_some() {
        return None;
    }
    ch.to_digit(30)
}

fn pow_i128(base: i128, exp: usize) -> i128 {
    let mut value = 1i128;
    for _ in 0..exp {
        value *= base;
    }
    value
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

    #[test]
    fn compact_reversal_shift_delta_matches_affine_shifts() {
        let lane_1b = compact_lane(3, 1, 11);
        let lane_b1 = compact_lane(3, 11, 1);
        let direct = lane_1b.shift as i128 - lane_b1.shift as i128;
        assert_eq!(direct, compact_reversal_shift_delta(3, 1, 11));
        assert_eq!(direct, -7_046_999_710);
        assert_eq!(lane_1b.gradient, lane_b1.gradient);
    }

    #[test]
    fn focus_pair_rates_match_exact_compact_scan() {
        assert_eq!(base30_compact_prime_hits(1, 1, 11), 7);
        assert_eq!(base30_compact_prime_hits(1, 11, 1), 5);
        assert_eq!(base30_compact_prime_hits(2, 1, 11), 191);
        assert_eq!(base30_compact_prime_hits(2, 11, 1), 194);
        assert_eq!(base30_compact_prime_hits(3, 1, 11), 4877);
        assert_eq!(base30_compact_prime_hits(3, 11, 1), 4451);
    }

    #[test]
    fn report_covers_all_ordered_pairs_and_reversal_symmetry() {
        let report = build_base30_reversal_report(Base30ReversalSettings {
            min_middle_length: 2,
            max_middle_length: 2,
            witness_limit: 1,
            top_limit: 3,
        });
        assert_eq!(report.ordered_pair_rows.len(), 64);
        assert_eq!(report.reversal_pair_rows.len(), 64);

        let one_b = report
            .reversal_pair_rows
            .iter()
            .find(|row| row.middle_length == 2 && row.outer == 1 && row.inner == 11)
            .expect("(1,B) row");
        let b_one = report
            .reversal_pair_rows
            .iter()
            .find(|row| row.middle_length == 2 && row.outer == 11 && row.inner == 1)
            .expect("(B,1) row");
        assert_eq!(one_b.hit_delta, -b_one.hit_delta);
        assert_eq!(one_b.shift_delta, -b_one.shift_delta);
        assert_eq!(one_b.forward_hits, 191);
        assert_eq!(one_b.reverse_hits, 194);
    }

    #[test]
    fn residual_report_has_28_non_diagonal_rows_per_m() {
        let report = build_base30_reversal_residual_report(Base30ReversalSettings {
            min_middle_length: 2,
            max_middle_length: 2,
            witness_limit: 1,
            top_limit: 3,
        });
        assert_eq!(report.residual_rows.len(), 28);
        let focus = report
            .residual_rows
            .iter()
            .find(|row| row.middle_length == 2 && row.low_digit == 1 && row.high_digit == 11)
            .expect("(1,B) residual row");
        assert_eq!(focus.low_high_prime_hits, 191);
        assert_eq!(focus.high_low_prime_hits, 194);
        assert_eq!(focus.seed_capacity, 900);
        assert_eq!(focus.gradient, 900);
        assert_eq!(focus.shift_delta, focus.formula_shift_delta);
    }

    #[test]
    fn residual_pnt_size_fields_are_directionally_sane_for_focus() {
        let report = build_base30_reversal_residual_report(Base30ReversalSettings {
            min_middle_length: 3,
            max_middle_length: 3,
            witness_limit: 1,
            top_limit: 3,
        });
        let focus = report
            .residual_rows
            .iter()
            .find(|row| row.middle_length == 3 && row.low_digit == 1 && row.high_digit == 11)
            .expect("(1,B) residual row");

        assert!(focus.low_high_average_ln < focus.high_low_average_ln);
        assert!(focus.low_high_pnt_expected_density > focus.high_low_pnt_expected_density);
        assert!(focus.size_expected_delta_pp > 0.0);
        assert!(focus.residual_after_size_pp.is_finite());
        assert!(focus.survivor_prime_residual_delta_pp.is_finite());
    }

    #[test]
    fn residual_combined_survivor_counts_match_exhaustive_filtering() {
        let report = build_base30_reversal_residual_report(Base30ReversalSettings {
            min_middle_length: 3,
            max_middle_length: 3,
            witness_limit: 1,
            top_limit: 3,
        });
        let focus = report
            .residual_rows
            .iter()
            .find(|row| row.middle_length == 3 && row.low_digit == 1 && row.high_digit == 11)
            .expect("(1,B) residual row");
        let low_high_lane = compact_lane(3, 1, 11);
        let high_low_lane = compact_lane(3, 11, 1);
        let low_high_exhaustive = (0..low_high_lane.seed_capacity)
            .filter(|&seed| {
                BASE30_REVERSAL_RESIDUE_MODULI
                    .iter()
                    .all(|&modulus| seed_survives_modulus(&low_high_lane, seed, modulus))
            })
            .count();
        let high_low_exhaustive = (0..high_low_lane.seed_capacity)
            .filter(|&seed| {
                BASE30_REVERSAL_RESIDUE_MODULI
                    .iter()
                    .all(|&modulus| seed_survives_modulus(&high_low_lane, seed, modulus))
            })
            .count();

        assert_eq!(focus.low_high_survivor_count, low_high_exhaustive);
        assert_eq!(focus.high_low_survivor_count, high_low_exhaustive);

        let combined = report
            .residue_delta_rows
            .iter()
            .find(|row| {
                row.middle_length == 3
                    && row.low_digit == 1
                    && row.high_digit == 11
                    && row.modulus_set_label == moduli_label(BASE30_REVERSAL_RESIDUE_MODULI)
            })
            .expect("combined residue row");
        assert_eq!(combined.low_high_survivor_count, low_high_exhaustive);
        assert_eq!(combined.high_low_survivor_count, high_low_exhaustive);
    }
}
