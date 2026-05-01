//! Cross-base affine phase residual atlas helpers.
//!
//! This module generalizes the compact base-30 reversal residual into an exact
//! cross-base scan. Each row compares `(low, high)` with `(high, low)` on the
//! compact `k=(0,0)` membrane lane and separates raw prime-rate differences
//! from size/PNT expectation and exact small-prime residue survival.

use crate::validation::{
    bounded_k::{digit_symbol, unit_residues, DEFAULT_PREFILTER_PRIMES},
    fast_affine::{build_fast_affine_lane, FastAffineLane, FastLaneConfig},
};
use serde::Serialize;
use std::collections::BTreeSet;

pub const DEFAULT_PHASE_RESIDUAL_BASES: &[u32] = &[6, 10, 14, 22, 26, 30, 34];
pub const DEFAULT_PHASE_RESIDUAL_MIN_MIDDLE_LENGTH: usize = 1;
pub const DEFAULT_PHASE_RESIDUAL_MAX_MIDDLE_LENGTH: usize = 3;
pub const DEFAULT_PHASE_RESIDUAL_TOP_LIMIT: usize = 12;
pub const DEFAULT_PHASE_RESIDUAL_WITNESS_LIMIT: usize = 4;
pub const PHASE_RESIDUAL_ANCHOR_BASE: u32 = 30;
pub const PHASE_RESIDUAL_ANCHOR_LOW_DIGIT: u32 = 1;
pub const PHASE_RESIDUAL_ANCHOR_HIGH_DIGIT: u32 = 11;
pub const PHASE_RESIDUAL_K: (u32, u32) = (0, 0);
pub const DEFAULT_SHIFT_PHASE_FOLLOWUP_MIDDLE_LENGTH: usize = 4;
pub const DEFAULT_SHIFT_PHASE_TOP_LIMIT: usize = 12;
pub const DEFAULT_SHIFT_PHASE_WITNESS_LIMIT: usize = 4;

#[derive(Debug, Clone, Serialize)]
pub struct AffinePhaseResidualSettings {
    pub bases: Vec<u32>,
    pub min_middle_length: usize,
    pub max_middle_length: usize,
    pub top_limit: usize,
    pub witness_limit: usize,
}

impl Default for AffinePhaseResidualSettings {
    fn default() -> Self {
        Self {
            bases: DEFAULT_PHASE_RESIDUAL_BASES.to_vec(),
            min_middle_length: DEFAULT_PHASE_RESIDUAL_MIN_MIDDLE_LENGTH,
            max_middle_length: DEFAULT_PHASE_RESIDUAL_MAX_MIDDLE_LENGTH,
            top_limit: DEFAULT_PHASE_RESIDUAL_TOP_LIMIT,
            witness_limit: DEFAULT_PHASE_RESIDUAL_WITNESS_LIMIT,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AffinePhaseResidualRow {
    pub base: u32,
    pub middle_length: usize,
    pub low_digit: u32,
    pub high_digit: u32,
    pub low_high_pair_label: String,
    pub high_low_pair_label: String,
    pub seed_capacity: u64,
    pub measurement_scope: String,
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
    pub abs_survivor_prime_residual_delta_pp: f64,
    pub lead_tag: String,
    pub first_low_high_witness: String,
    pub first_high_low_witness: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffinePhaseModulusRow {
    pub base: u32,
    pub middle_length: usize,
    pub low_digit: u32,
    pub high_digit: u32,
    pub low_high_pair_label: String,
    pub high_low_pair_label: String,
    pub modulus: u32,
    pub low_high_shift_modulus: u32,
    pub high_low_shift_modulus: u32,
    pub low_high_gradient_modulus: u32,
    pub high_low_gradient_modulus: u32,
    pub low_high_excluded_seed_classes: String,
    pub high_low_excluded_seed_classes: String,
    pub low_high_survivor_count: usize,
    pub high_low_survivor_count: usize,
    pub low_high_survivor_share: f64,
    pub high_low_survivor_share: f64,
    pub survivor_delta_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffinePhaseBaseSummaryRow {
    pub base: u32,
    pub middle_length: usize,
    pub unordered_pair_count: usize,
    pub residue_moduli_label: String,
    pub mean_abs_raw_delta_pp: f64,
    pub mean_abs_residual_after_size_pp: f64,
    pub mean_abs_survivor_prime_residual_delta_pp: f64,
    pub strongest_size_residual_pair: String,
    pub strongest_size_residual_reverse_pair: String,
    pub strongest_size_residual_pp: f64,
    pub strongest_survivor_prime_pair: String,
    pub strongest_survivor_prime_reverse_pair: String,
    pub strongest_survivor_prime_residual_pp: f64,
    pub survivor_prime_led_rows: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffinePhaseTopSignalRow {
    pub selection_reason: String,
    pub base: u32,
    pub middle_length: usize,
    pub low_digit: u32,
    pub high_digit: u32,
    pub low_high_pair_label: String,
    pub high_low_pair_label: String,
    pub raw_delta_pp: f64,
    pub size_expected_delta_pp: f64,
    pub residual_after_size_pp: f64,
    pub residue_survivor_delta_pp: f64,
    pub survivor_prime_residual_delta_pp: f64,
    pub low_high_prime_hits: usize,
    pub high_low_prime_hits: usize,
    pub low_high_survivor_count: usize,
    pub high_low_survivor_count: usize,
    pub lead_tag: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffinePhaseWitnessRow {
    pub selection_reason: String,
    pub base: u32,
    pub middle_length: usize,
    pub orientation: String,
    pub pair_label: String,
    pub outer: u32,
    pub inner: u32,
    pub seed: u64,
    pub middle_digits: String,
    pub template_digits: String,
    pub decimal_value: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffinePhaseResidualSummary {
    pub base_count: usize,
    pub residual_row_count: usize,
    pub phase_modulus_row_count: usize,
    pub top_signal_row_count: usize,
    pub anchor_pair_label: String,
    pub anchor_reverse_pair_label: String,
    pub anchor_m3_raw_delta_pp: f64,
    pub anchor_m3_residual_after_size_pp: f64,
    pub anchor_m3_survivor_prime_residual_delta_pp: f64,
    pub strongest_size_residual_base: u32,
    pub strongest_size_residual_pair: String,
    pub strongest_size_residual_reverse_pair: String,
    pub strongest_size_residual_m: usize,
    pub strongest_size_residual_pp: f64,
    pub strongest_survivor_prime_base: u32,
    pub strongest_survivor_prime_pair: String,
    pub strongest_survivor_prime_reverse_pair: String,
    pub strongest_survivor_prime_m: usize,
    pub strongest_survivor_prime_residual_pp: f64,
    pub strong_line: String,
    pub caution_line: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffinePhaseResidualAtlas {
    pub settings: AffinePhaseResidualSettings,
    pub summary: AffinePhaseResidualSummary,
    pub phase_residual_rows: Vec<AffinePhaseResidualRow>,
    pub phase_modulus_rows: Vec<AffinePhaseModulusRow>,
    pub base_summary_rows: Vec<AffinePhaseBaseSummaryRow>,
    pub top_signal_rows: Vec<AffinePhaseTopSignalRow>,
    pub witness_rows: Vec<AffinePhaseWitnessRow>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ShiftPhaseTrackSpec {
    pub track_name: &'static str,
    pub track_kind: &'static str,
    pub base: u32,
    pub low_digit: u32,
    pub high_digit: u32,
    pub source_middle_length: usize,
    pub note: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct ShiftPhaseSignalMiningSettings {
    pub base_settings: AffinePhaseResidualSettings,
    pub followup_middle_length: usize,
    pub top_limit: usize,
    pub witness_limit: usize,
}

impl Default for ShiftPhaseSignalMiningSettings {
    fn default() -> Self {
        Self {
            base_settings: AffinePhaseResidualSettings::default(),
            followup_middle_length: DEFAULT_SHIFT_PHASE_FOLLOWUP_MIDDLE_LENGTH,
            top_limit: DEFAULT_SHIFT_PHASE_TOP_LIMIT,
            witness_limit: DEFAULT_SHIFT_PHASE_WITNESS_LIMIT,
        }
    }
}

pub const DEFAULT_SHIFT_PHASE_FOCUS_SPECS: &[ShiftPhaseTrackSpec] = &[
    ShiftPhaseTrackSpec {
        track_name: "base10_low_outer_17",
        track_kind: "focus_lead",
        base: 10,
        low_digit: 1,
        high_digit: 7,
        source_middle_length: 3,
        note: "mature base-10 lead surfaced by the phase-residual atlas",
    },
    ShiftPhaseTrackSpec {
        track_name: "base10_classic_37",
        track_kind: "focus_lead",
        base: 10,
        low_digit: 3,
        high_digit: 7,
        source_middle_length: 3,
        note: "classic decimal membrane pair treated as same-gradient reversal lead",
    },
    ShiftPhaseTrackSpec {
        track_name: "base30_low_outer_1j",
        track_kind: "focus_lead",
        base: 30,
        low_digit: 1,
        high_digit: 19,
        source_middle_length: 3,
        note: "strongest mature base-30 size residual",
    },
    ShiftPhaseTrackSpec {
        track_name: "base30_low_outer_1t",
        track_kind: "focus_lead",
        base: 30,
        low_digit: 1,
        high_digit: 29,
        source_middle_length: 3,
        note: "base-30 low-outer mature lead",
    },
    ShiftPhaseTrackSpec {
        track_name: "base30_low_outer_1h",
        track_kind: "focus_lead",
        base: 30,
        low_digit: 1,
        high_digit: 17,
        source_middle_length: 3,
        note: "base-30 low-outer mature lead",
    },
    ShiftPhaseTrackSpec {
        track_name: "base30_anchor_1b",
        track_kind: "focus_lead",
        base: 30,
        low_digit: 1,
        high_digit: 11,
        source_middle_length: 3,
        note: "canonical anchor from the base-30 reversal residual report",
    },
    ShiftPhaseTrackSpec {
        track_name: "base22_low_outer_1l",
        track_kind: "focus_lead",
        base: 22,
        low_digit: 1,
        high_digit: 21,
        source_middle_length: 3,
        note: "strong mature survivor-prime residual in base 22",
    },
    ShiftPhaseTrackSpec {
        track_name: "base26_low_outer_1p",
        track_kind: "focus_lead",
        base: 26,
        low_digit: 1,
        high_digit: 25,
        source_middle_length: 3,
        note: "strong mature survivor-prime residual in base 26",
    },
    ShiftPhaseTrackSpec {
        track_name: "base34_low_outer_1v",
        track_kind: "focus_lead",
        base: 34,
        low_digit: 1,
        high_digit: 31,
        source_middle_length: 3,
        note: "strong mature survivor-prime residual in base 34",
    },
    ShiftPhaseTrackSpec {
        track_name: "base6_bridge_15",
        track_kind: "focus_lead",
        base: 6,
        low_digit: 1,
        high_digit: 5,
        source_middle_length: 3,
        note: "small-base bridge witness with mature residual signal",
    },
];

pub const DEFAULT_SHIFT_PHASE_FOIL_SPECS: &[ShiftPhaseTrackSpec] = &[
    ShiftPhaseTrackSpec {
        track_name: "volatile_base26_1f",
        track_kind: "volatile_foil",
        base: 26,
        low_digit: 1,
        high_digit: 15,
        source_middle_length: 1,
        note: "huge M=1 effect kept as a volatility warning",
    },
    ShiftPhaseTrackSpec {
        track_name: "volatile_base22_dj",
        track_kind: "volatile_foil",
        base: 22,
        low_digit: 13,
        high_digit: 19,
        source_middle_length: 1,
        note: "large short-lane reversal effect used as a foil",
    },
    ShiftPhaseTrackSpec {
        track_name: "residue_survival_base10_39",
        track_kind: "residue_only_foil",
        base: 10,
        low_digit: 3,
        high_digit: 9,
        source_middle_length: 1,
        note: "residue-survival-led short-lane foil",
    },
    ShiftPhaseTrackSpec {
        track_name: "low_residual_base30_bj",
        track_kind: "low_residual_foil",
        base: 30,
        low_digit: 11,
        high_digit: 19,
        source_middle_length: 3,
        note: "same-base mature low-residual contrast row",
    },
];

#[derive(Debug, Clone, Serialize)]
pub struct ShiftPhaseMaturityRow {
    pub track_name: String,
    pub track_kind: String,
    pub note: String,
    pub base: u32,
    pub low_digit: u32,
    pub high_digit: u32,
    pub pair_label: String,
    pub reverse_pair_label: String,
    pub source_middle_length: usize,
    pub followup_middle_length: usize,
    pub source_raw_delta_pp: f64,
    pub source_residual_after_size_pp: f64,
    pub source_survivor_prime_residual_delta_pp: f64,
    pub source_residue_survivor_delta_pp: f64,
    pub followup_raw_delta_pp: f64,
    pub followup_residual_after_size_pp: f64,
    pub followup_survivor_prime_residual_delta_pp: f64,
    pub followup_residue_survivor_delta_pp: f64,
    pub source_prime_hits: String,
    pub followup_prime_hits: String,
    pub followup_seed_capacity: u64,
    pub stability_label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ShiftPhaseFoilRow {
    pub foil_name: String,
    pub foil_kind: String,
    pub base: u32,
    pub middle_length: usize,
    pub pair_label: String,
    pub reverse_pair_label: String,
    pub raw_delta_pp: f64,
    pub residual_after_size_pp: f64,
    pub residue_survivor_delta_pp: f64,
    pub survivor_prime_residual_delta_pp: f64,
    pub lead_tag: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ShiftPhaseResidueGateRow {
    pub track_name: String,
    pub track_kind: String,
    pub base: u32,
    pub middle_length: usize,
    pub pair_label: String,
    pub reverse_pair_label: String,
    pub modulus: u32,
    pub low_high_shift_modulus: u32,
    pub high_low_shift_modulus: u32,
    pub gradient_modulus: u32,
    pub low_high_excluded_seed_classes: String,
    pub high_low_excluded_seed_classes: String,
    pub low_high_survivor_count: usize,
    pub high_low_survivor_count: usize,
    pub survivor_delta_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ShiftPhaseWitnessRow {
    pub track_name: String,
    pub track_kind: String,
    pub base: u32,
    pub middle_length: usize,
    pub orientation: String,
    pub pair_label: String,
    pub seed: u64,
    pub middle_digits: String,
    pub template_digits: String,
    pub decimal_value: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ShiftPhaseSignalMiningSummary {
    pub broad_residual_row_count: usize,
    pub tracked_pair_count: usize,
    pub maturity_row_count: usize,
    pub foil_row_count: usize,
    pub residue_gate_row_count: usize,
    pub persistent_or_amplifying_count: usize,
    pub strongest_followup_track: String,
    pub strongest_followup_pair: String,
    pub strongest_followup_survivor_prime_residual_pp: f64,
    pub base30_anchor_stability_label: String,
    pub public_phrase: String,
    pub strong_line: String,
    pub caution_line: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ShiftPhaseSignalMiningReport {
    pub settings: ShiftPhaseSignalMiningSettings,
    pub summary: ShiftPhaseSignalMiningSummary,
    pub shift_phase_rows: Vec<AffinePhaseResidualRow>,
    pub maturity_rows: Vec<ShiftPhaseMaturityRow>,
    pub foil_rows: Vec<ShiftPhaseFoilRow>,
    pub residue_gate_rows: Vec<ShiftPhaseResidueGateRow>,
    pub witness_rows: Vec<ShiftPhaseWitnessRow>,
}

#[derive(Debug, Clone)]
struct LaneStats {
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

#[derive(Debug, Clone)]
struct CompactWitness {
    seed: u64,
    middle_digits: String,
    template_digits: String,
    decimal_value: u64,
}

pub fn build_affine_phase_residual_atlas(
    settings: AffinePhaseResidualSettings,
) -> AffinePhaseResidualAtlas {
    let phase_residual_rows = build_phase_residual_rows(&settings);
    let phase_modulus_rows = build_phase_modulus_rows(&phase_residual_rows);
    let base_summary_rows = build_base_summary_rows(&phase_residual_rows);
    let top_signal_rows = build_top_signal_rows(
        &phase_residual_rows,
        settings.top_limit,
        settings.max_middle_length,
    );
    let witness_rows = build_witness_rows(&top_signal_rows, settings.witness_limit);
    let summary = build_summary(
        &settings,
        &phase_residual_rows,
        &phase_modulus_rows,
        &top_signal_rows,
    );

    AffinePhaseResidualAtlas {
        settings,
        summary,
        phase_residual_rows,
        phase_modulus_rows,
        base_summary_rows,
        top_signal_rows,
        witness_rows,
    }
}

pub fn compact_reversal_shift_delta(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
) -> i128 {
    let base = i128::from(base);
    let digit_delta = i128::from(outer) - i128::from(inner);
    digit_delta * (base - 1) * (pow_i128(base, middle_length + 2) - 1)
}

pub fn residue_moduli_for_base(base: u32) -> Vec<u32> {
    DEFAULT_PREFILTER_PRIMES
        .iter()
        .copied()
        .filter(|&modulus| gcd_u32(base, modulus) == 1)
        .collect()
}

pub fn build_affine_phase_residual_row(
    base: u32,
    middle_length: usize,
    low_digit: u32,
    high_digit: u32,
) -> AffinePhaseResidualRow {
    let moduli = residue_moduli_for_base(base);
    let low_high = lane_stats(base, middle_length, low_digit, high_digit, &moduli);
    let high_low = lane_stats(base, middle_length, high_digit, low_digit, &moduli);
    let raw_delta_pp = (low_high.prime_rate - high_low.prime_rate) * 100.0;
    let size_expected_delta_pp =
        (low_high.pnt_expected_density - high_low.pnt_expected_density) * 100.0;
    let residual_after_size_pp = raw_delta_pp - size_expected_delta_pp;
    let residue_survivor_delta_pp = (low_high.survivor_share - high_low.survivor_share) * 100.0;
    let survivor_prime_residual_delta_pp =
        (low_high.prime_rate_among_survivors - high_low.prime_rate_among_survivors) * 100.0;

    AffinePhaseResidualRow {
        base,
        middle_length,
        low_digit,
        high_digit,
        low_high_pair_label: pair_label(low_digit, high_digit),
        high_low_pair_label: pair_label(high_digit, low_digit),
        seed_capacity: low_high.lane.seed_capacity,
        measurement_scope: "exact_u64".to_string(),
        gradient: low_high.lane.gradient,
        low_high_shift: low_high.lane.shift,
        high_low_shift: high_low.lane.shift,
        shift_delta: low_high.lane.shift as i128 - high_low.lane.shift as i128,
        formula_shift_delta: compact_reversal_shift_delta(
            base,
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
        residue_moduli_label: moduli_label(&moduli),
        low_high_survivor_count: low_high.survivor_count,
        high_low_survivor_count: high_low.survivor_count,
        low_high_survivor_share: low_high.survivor_share,
        high_low_survivor_share: high_low.survivor_share,
        residue_survivor_delta_pp,
        low_high_prime_rate_among_survivors: low_high.prime_rate_among_survivors,
        high_low_prime_rate_among_survivors: high_low.prime_rate_among_survivors,
        survivor_prime_residual_delta_pp,
        abs_survivor_prime_residual_delta_pp: survivor_prime_residual_delta_pp.abs(),
        lead_tag: lead_tag(
            size_expected_delta_pp,
            residue_survivor_delta_pp,
            survivor_prime_residual_delta_pp,
        ),
        first_low_high_witness: low_high.first_witness,
        first_high_low_witness: high_low.first_witness,
    }
}

fn build_phase_residual_rows(
    settings: &AffinePhaseResidualSettings,
) -> Vec<AffinePhaseResidualRow> {
    let mut rows = Vec::new();
    for &base in &settings.bases {
        let units = unit_residues(base);
        for middle_length in settings.min_middle_length..=settings.max_middle_length {
            for (left_idx, &low_digit) in units.iter().enumerate() {
                for &high_digit in units.iter().skip(left_idx + 1) {
                    rows.push(build_affine_phase_residual_row(
                        base,
                        middle_length,
                        low_digit,
                        high_digit,
                    ));
                }
            }
        }
    }
    rows
}

fn lane_stats(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    moduli: &[u32],
) -> LaneStats {
    let lane = compact_lane(base, middle_length, outer, inner);
    let mut prime_hits = 0usize;
    let mut survivor_count = 0usize;
    let mut survivor_prime_hits = 0usize;
    let mut sum_ln = 0.0;
    let mut first_witness = String::new();

    for seed in 0..lane.seed_capacity {
        let value = lane
            .candidate_value(seed)
            .expect("compact phase residual lane should fit u64");
        sum_ln += (value as f64).ln();
        let survivor = seed_survives_moduli(&lane, seed, moduli);
        if survivor {
            survivor_count += 1;
        }
        if primal::is_prime(value) {
            prime_hits += 1;
            if survivor {
                survivor_prime_hits += 1;
            }
            if first_witness.is_empty() {
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

    LaneStats {
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

fn build_phase_modulus_rows(
    residual_rows: &[AffinePhaseResidualRow],
) -> Vec<AffinePhaseModulusRow> {
    let mut rows = Vec::new();
    for row in residual_rows {
        let low_high_lane =
            compact_lane(row.base, row.middle_length, row.low_digit, row.high_digit);
        let high_low_lane =
            compact_lane(row.base, row.middle_length, row.high_digit, row.low_digit);
        for modulus in residue_moduli_for_base(row.base) {
            let low_high_survivor_count = survivor_count_for_moduli(&low_high_lane, &[modulus]);
            let high_low_survivor_count = survivor_count_for_moduli(&high_low_lane, &[modulus]);
            let seed_capacity = row.seed_capacity as f64;
            let low_high_survivor_share = low_high_survivor_count as f64 / seed_capacity;
            let high_low_survivor_share = high_low_survivor_count as f64 / seed_capacity;
            rows.push(AffinePhaseModulusRow {
                base: row.base,
                middle_length: row.middle_length,
                low_digit: row.low_digit,
                high_digit: row.high_digit,
                low_high_pair_label: row.low_high_pair_label.clone(),
                high_low_pair_label: row.high_low_pair_label.clone(),
                modulus,
                low_high_shift_modulus: (low_high_lane.shift % u64::from(modulus)) as u32,
                high_low_shift_modulus: (high_low_lane.shift % u64::from(modulus)) as u32,
                low_high_gradient_modulus: (low_high_lane.gradient % u64::from(modulus)) as u32,
                high_low_gradient_modulus: (high_low_lane.gradient % u64::from(modulus)) as u32,
                low_high_excluded_seed_classes: excluded_seed_classes_label(
                    &low_high_lane,
                    modulus,
                ),
                high_low_excluded_seed_classes: excluded_seed_classes_label(
                    &high_low_lane,
                    modulus,
                ),
                low_high_survivor_count,
                high_low_survivor_count,
                low_high_survivor_share,
                high_low_survivor_share,
                survivor_delta_pp: (low_high_survivor_share - high_low_survivor_share) * 100.0,
            });
        }
    }
    rows
}

fn build_base_summary_rows(
    residual_rows: &[AffinePhaseResidualRow],
) -> Vec<AffinePhaseBaseSummaryRow> {
    let keys = residual_rows
        .iter()
        .map(|row| (row.base, row.middle_length))
        .collect::<BTreeSet<_>>();
    let mut rows = Vec::new();
    for (base, middle_length) in keys {
        let group = residual_rows
            .iter()
            .filter(|row| row.base == base && row.middle_length == middle_length)
            .collect::<Vec<_>>();
        let strongest_size = group
            .iter()
            .max_by(|left, right| {
                left.abs_residual_after_size_pp
                    .total_cmp(&right.abs_residual_after_size_pp)
            })
            .expect("base summary group should not be empty");
        let strongest_survivor = group
            .iter()
            .max_by(|left, right| {
                left.abs_survivor_prime_residual_delta_pp
                    .total_cmp(&right.abs_survivor_prime_residual_delta_pp)
            })
            .expect("base summary group should not be empty");
        rows.push(AffinePhaseBaseSummaryRow {
            base,
            middle_length,
            unordered_pair_count: group.len(),
            residue_moduli_label: moduli_label(&residue_moduli_for_base(base)),
            mean_abs_raw_delta_pp: mean(group.iter().map(|row| row.abs_raw_delta_pp)),
            mean_abs_residual_after_size_pp: mean(
                group.iter().map(|row| row.abs_residual_after_size_pp),
            ),
            mean_abs_survivor_prime_residual_delta_pp: mean(
                group
                    .iter()
                    .map(|row| row.abs_survivor_prime_residual_delta_pp),
            ),
            strongest_size_residual_pair: strongest_size.low_high_pair_label.clone(),
            strongest_size_residual_reverse_pair: strongest_size.high_low_pair_label.clone(),
            strongest_size_residual_pp: strongest_size.residual_after_size_pp,
            strongest_survivor_prime_pair: strongest_survivor.low_high_pair_label.clone(),
            strongest_survivor_prime_reverse_pair: strongest_survivor.high_low_pair_label.clone(),
            strongest_survivor_prime_residual_pp: strongest_survivor
                .survivor_prime_residual_delta_pp,
            survivor_prime_led_rows: group
                .iter()
                .filter(|row| row.lead_tag == "survivor_prime_led")
                .count(),
        });
    }
    rows
}

fn build_top_signal_rows(
    residual_rows: &[AffinePhaseResidualRow],
    top_limit: usize,
    mature_middle_length: usize,
) -> Vec<AffinePhaseTopSignalRow> {
    let mut selected = Vec::new();
    let mut seen = BTreeSet::new();

    let mut by_size = residual_rows.iter().collect::<Vec<_>>();
    by_size.sort_by(|left, right| {
        right
            .abs_residual_after_size_pp
            .total_cmp(&left.abs_residual_after_size_pp)
            .then_with(|| left.base.cmp(&right.base))
            .then_with(|| left.low_high_pair_label.cmp(&right.low_high_pair_label))
    });
    for row in by_size.into_iter().take(top_limit) {
        push_top_signal_row(&mut selected, &mut seen, row, "top_size_residual");
    }

    let mut by_survivor = residual_rows.iter().collect::<Vec<_>>();
    by_survivor.sort_by(|left, right| {
        right
            .abs_survivor_prime_residual_delta_pp
            .total_cmp(&left.abs_survivor_prime_residual_delta_pp)
            .then_with(|| left.base.cmp(&right.base))
            .then_with(|| left.low_high_pair_label.cmp(&right.low_high_pair_label))
    });
    for row in by_survivor.into_iter().take(top_limit) {
        push_top_signal_row(&mut selected, &mut seen, row, "top_survivor_prime_residual");
    }

    let mut mature_by_size = residual_rows
        .iter()
        .filter(|row| row.middle_length == mature_middle_length)
        .collect::<Vec<_>>();
    mature_by_size.sort_by(|left, right| {
        right
            .abs_residual_after_size_pp
            .total_cmp(&left.abs_residual_after_size_pp)
            .then_with(|| left.base.cmp(&right.base))
            .then_with(|| left.low_high_pair_label.cmp(&right.low_high_pair_label))
    });
    for row in mature_by_size.into_iter().take(top_limit) {
        push_top_signal_row(&mut selected, &mut seen, row, "top_mature_size_residual");
    }

    let mut mature_by_survivor = residual_rows
        .iter()
        .filter(|row| row.middle_length == mature_middle_length)
        .collect::<Vec<_>>();
    mature_by_survivor.sort_by(|left, right| {
        right
            .abs_survivor_prime_residual_delta_pp
            .total_cmp(&left.abs_survivor_prime_residual_delta_pp)
            .then_with(|| left.base.cmp(&right.base))
            .then_with(|| left.low_high_pair_label.cmp(&right.low_high_pair_label))
    });
    for row in mature_by_survivor.into_iter().take(top_limit) {
        push_top_signal_row(
            &mut selected,
            &mut seen,
            row,
            "top_mature_survivor_prime_residual",
        );
    }

    if let Some(anchor) = residual_rows.iter().find(|row| {
        row.base == PHASE_RESIDUAL_ANCHOR_BASE
            && row.middle_length == 3
            && row.low_digit == PHASE_RESIDUAL_ANCHOR_LOW_DIGIT
            && row.high_digit == PHASE_RESIDUAL_ANCHOR_HIGH_DIGIT
    }) {
        push_top_signal_row(&mut selected, &mut seen, anchor, "base30_anchor");
    }

    selected
}

fn push_top_signal_row(
    selected: &mut Vec<AffinePhaseTopSignalRow>,
    seen: &mut BTreeSet<(u32, usize, u32, u32)>,
    row: &AffinePhaseResidualRow,
    reason: &str,
) {
    if !seen.insert((row.base, row.middle_length, row.low_digit, row.high_digit)) {
        return;
    }
    selected.push(AffinePhaseTopSignalRow {
        selection_reason: reason.to_string(),
        base: row.base,
        middle_length: row.middle_length,
        low_digit: row.low_digit,
        high_digit: row.high_digit,
        low_high_pair_label: row.low_high_pair_label.clone(),
        high_low_pair_label: row.high_low_pair_label.clone(),
        raw_delta_pp: row.raw_delta_pp,
        size_expected_delta_pp: row.size_expected_delta_pp,
        residual_after_size_pp: row.residual_after_size_pp,
        residue_survivor_delta_pp: row.residue_survivor_delta_pp,
        survivor_prime_residual_delta_pp: row.survivor_prime_residual_delta_pp,
        low_high_prime_hits: row.low_high_prime_hits,
        high_low_prime_hits: row.high_low_prime_hits,
        low_high_survivor_count: row.low_high_survivor_count,
        high_low_survivor_count: row.high_low_survivor_count,
        lead_tag: row.lead_tag.clone(),
    });
}

fn build_witness_rows(
    top_rows: &[AffinePhaseTopSignalRow],
    witness_limit: usize,
) -> Vec<AffinePhaseWitnessRow> {
    let mut rows = Vec::new();
    let mut seen = BTreeSet::new();
    for top_row in top_rows {
        for (orientation, outer, inner) in [
            ("low_high", top_row.low_digit, top_row.high_digit),
            ("high_low", top_row.high_digit, top_row.low_digit),
        ] {
            if !seen.insert((top_row.base, top_row.middle_length, outer, inner)) {
                continue;
            }
            let witnesses = compact_prime_witnesses(
                top_row.base,
                top_row.middle_length,
                outer,
                inner,
                witness_limit,
            );
            for witness in witnesses {
                rows.push(AffinePhaseWitnessRow {
                    selection_reason: top_row.selection_reason.clone(),
                    base: top_row.base,
                    middle_length: top_row.middle_length,
                    orientation: orientation.to_string(),
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

fn build_summary(
    settings: &AffinePhaseResidualSettings,
    residual_rows: &[AffinePhaseResidualRow],
    phase_modulus_rows: &[AffinePhaseModulusRow],
    top_signal_rows: &[AffinePhaseTopSignalRow],
) -> AffinePhaseResidualSummary {
    let anchor = residual_rows
        .iter()
        .find(|row| {
            row.base == PHASE_RESIDUAL_ANCHOR_BASE
                && row.middle_length == 3
                && row.low_digit == PHASE_RESIDUAL_ANCHOR_LOW_DIGIT
                && row.high_digit == PHASE_RESIDUAL_ANCHOR_HIGH_DIGIT
        })
        .or_else(|| residual_rows.first())
        .expect("phase residual atlas should contain rows");
    let strongest_size = residual_rows
        .iter()
        .max_by(|left, right| {
            left.abs_residual_after_size_pp
                .total_cmp(&right.abs_residual_after_size_pp)
        })
        .expect("phase residual atlas should contain rows");
    let strongest_survivor = residual_rows
        .iter()
        .max_by(|left, right| {
            left.abs_survivor_prime_residual_delta_pp
                .total_cmp(&right.abs_survivor_prime_residual_delta_pp)
        })
        .expect("phase residual atlas should contain rows");

    AffinePhaseResidualSummary {
        base_count: settings.bases.len(),
        residual_row_count: residual_rows.len(),
        phase_modulus_row_count: phase_modulus_rows.len(),
        top_signal_row_count: top_signal_rows.len(),
        anchor_pair_label: anchor.low_high_pair_label.clone(),
        anchor_reverse_pair_label: anchor.high_low_pair_label.clone(),
        anchor_m3_raw_delta_pp: anchor.raw_delta_pp,
        anchor_m3_residual_after_size_pp: anchor.residual_after_size_pp,
        anchor_m3_survivor_prime_residual_delta_pp: anchor.survivor_prime_residual_delta_pp,
        strongest_size_residual_base: strongest_size.base,
        strongest_size_residual_pair: strongest_size.low_high_pair_label.clone(),
        strongest_size_residual_reverse_pair: strongest_size.high_low_pair_label.clone(),
        strongest_size_residual_m: strongest_size.middle_length,
        strongest_size_residual_pp: strongest_size.residual_after_size_pp,
        strongest_survivor_prime_base: strongest_survivor.base,
        strongest_survivor_prime_pair: strongest_survivor.low_high_pair_label.clone(),
        strongest_survivor_prime_reverse_pair: strongest_survivor.high_low_pair_label.clone(),
        strongest_survivor_prime_m: strongest_survivor.middle_length,
        strongest_survivor_prime_residual_pp: strongest_survivor
            .survivor_prime_residual_delta_pp,
        strong_line:
            "coherent local affine phase effects can be ranked across compact cross-base reversal lanes."
                .to_string(),
        caution_line:
            "the atlas is a signal-discovery surface, not a density theorem or asymptotic claim."
                .to_string(),
    }
}

pub fn build_shift_phase_signal_mining_report(
    settings: ShiftPhaseSignalMiningSettings,
) -> ShiftPhaseSignalMiningReport {
    build_shift_phase_signal_mining_report_for_specs(
        settings,
        DEFAULT_SHIFT_PHASE_FOCUS_SPECS,
        DEFAULT_SHIFT_PHASE_FOIL_SPECS,
    )
}

pub fn build_shift_phase_signal_mining_report_for_specs(
    settings: ShiftPhaseSignalMiningSettings,
    focus_specs: &[ShiftPhaseTrackSpec],
    foil_specs: &[ShiftPhaseTrackSpec],
) -> ShiftPhaseSignalMiningReport {
    let atlas = build_affine_phase_residual_atlas(settings.base_settings.clone());
    let selected_specs =
        select_shift_phase_specs(&atlas, focus_specs, foil_specs, settings.top_limit);
    let maturity_rows = build_maturity_rows(
        &atlas.phase_residual_rows,
        &selected_specs,
        settings.followup_middle_length,
    );
    let foil_rows = build_foil_rows(&atlas.phase_residual_rows, foil_specs);
    let residue_gate_rows = build_shift_phase_residue_gate_rows(
        &selected_specs,
        &maturity_rows,
        settings.followup_middle_length,
    );
    let witness_rows = build_shift_phase_witness_rows(
        &selected_specs,
        &maturity_rows,
        settings.followup_middle_length,
        settings.witness_limit,
    );
    let summary = build_shift_phase_summary(
        &atlas.phase_residual_rows,
        &maturity_rows,
        &foil_rows,
        &residue_gate_rows,
    );

    ShiftPhaseSignalMiningReport {
        settings,
        summary,
        shift_phase_rows: atlas.phase_residual_rows,
        maturity_rows,
        foil_rows,
        residue_gate_rows,
        witness_rows,
    }
}

#[derive(Debug, Clone)]
struct OwnedTrackSpec {
    track_name: String,
    track_kind: String,
    base: u32,
    low_digit: u32,
    high_digit: u32,
    source_middle_length: usize,
    note: String,
}

impl From<ShiftPhaseTrackSpec> for OwnedTrackSpec {
    fn from(spec: ShiftPhaseTrackSpec) -> Self {
        Self {
            track_name: spec.track_name.to_string(),
            track_kind: spec.track_kind.to_string(),
            base: spec.base,
            low_digit: spec.low_digit,
            high_digit: spec.high_digit,
            source_middle_length: spec.source_middle_length,
            note: spec.note.to_string(),
        }
    }
}

fn select_shift_phase_specs(
    atlas: &AffinePhaseResidualAtlas,
    focus_specs: &[ShiftPhaseTrackSpec],
    foil_specs: &[ShiftPhaseTrackSpec],
    top_limit: usize,
) -> Vec<OwnedTrackSpec> {
    let mut selected = Vec::new();
    let mut seen = BTreeSet::new();

    for spec in focus_specs.iter().chain(foil_specs.iter()) {
        let owned = OwnedTrackSpec::from(*spec);
        if seen.insert((
            owned.base,
            owned.low_digit,
            owned.high_digit,
            owned.source_middle_length,
        )) {
            selected.push(owned);
        }
    }

    for row in atlas
        .top_signal_rows
        .iter()
        .filter(|row| row.selection_reason.starts_with("top_mature"))
        .take(top_limit)
    {
        if seen.insert((row.base, row.low_digit, row.high_digit, row.middle_length)) {
            selected.push(OwnedTrackSpec {
                track_name: format!(
                    "atlas_top_base{}_m{}_{}_{}",
                    row.base,
                    row.middle_length,
                    digit_symbol(row.low_digit),
                    digit_symbol(row.high_digit)
                ),
                track_kind: "atlas_mature_lead".to_string(),
                base: row.base,
                low_digit: row.low_digit,
                high_digit: row.high_digit,
                source_middle_length: row.middle_length,
                note: format!("selected from {}", row.selection_reason),
            });
        }
    }

    selected
}

fn build_maturity_rows(
    source_rows: &[AffinePhaseResidualRow],
    specs: &[OwnedTrackSpec],
    followup_middle_length: usize,
) -> Vec<ShiftPhaseMaturityRow> {
    specs
        .iter()
        .filter_map(|spec| {
            let source = find_residual_row(
                source_rows,
                spec.base,
                spec.source_middle_length,
                spec.low_digit,
                spec.high_digit,
            )?;
            let followup = build_affine_phase_residual_row(
                spec.base,
                followup_middle_length,
                spec.low_digit,
                spec.high_digit,
            );
            Some(ShiftPhaseMaturityRow {
                track_name: spec.track_name.clone(),
                track_kind: spec.track_kind.clone(),
                note: spec.note.clone(),
                base: spec.base,
                low_digit: spec.low_digit,
                high_digit: spec.high_digit,
                pair_label: source.low_high_pair_label.clone(),
                reverse_pair_label: source.high_low_pair_label.clone(),
                source_middle_length: spec.source_middle_length,
                followup_middle_length,
                source_raw_delta_pp: source.raw_delta_pp,
                source_residual_after_size_pp: source.residual_after_size_pp,
                source_survivor_prime_residual_delta_pp: source.survivor_prime_residual_delta_pp,
                source_residue_survivor_delta_pp: source.residue_survivor_delta_pp,
                followup_raw_delta_pp: followup.raw_delta_pp,
                followup_residual_after_size_pp: followup.residual_after_size_pp,
                followup_survivor_prime_residual_delta_pp: followup
                    .survivor_prime_residual_delta_pp,
                followup_residue_survivor_delta_pp: followup.residue_survivor_delta_pp,
                source_prime_hits: format!(
                    "{} / {}",
                    source.low_high_prime_hits, source.high_low_prime_hits
                ),
                followup_prime_hits: format!(
                    "{} / {}",
                    followup.low_high_prime_hits, followup.high_low_prime_hits
                ),
                followup_seed_capacity: followup.seed_capacity,
                stability_label: classify_shift_phase_stability(source, &followup),
            })
        })
        .collect()
}

fn build_foil_rows(
    source_rows: &[AffinePhaseResidualRow],
    foil_specs: &[ShiftPhaseTrackSpec],
) -> Vec<ShiftPhaseFoilRow> {
    foil_specs
        .iter()
        .filter_map(|spec| {
            let row = find_residual_row(
                source_rows,
                spec.base,
                spec.source_middle_length,
                spec.low_digit,
                spec.high_digit,
            )?;
            Some(ShiftPhaseFoilRow {
                foil_name: spec.track_name.to_string(),
                foil_kind: spec.track_kind.to_string(),
                base: spec.base,
                middle_length: spec.source_middle_length,
                pair_label: row.low_high_pair_label.clone(),
                reverse_pair_label: row.high_low_pair_label.clone(),
                raw_delta_pp: row.raw_delta_pp,
                residual_after_size_pp: row.residual_after_size_pp,
                residue_survivor_delta_pp: row.residue_survivor_delta_pp,
                survivor_prime_residual_delta_pp: row.survivor_prime_residual_delta_pp,
                lead_tag: row.lead_tag.clone(),
                note: spec.note.to_string(),
            })
        })
        .collect()
}

fn build_shift_phase_residue_gate_rows(
    specs: &[OwnedTrackSpec],
    maturity_rows: &[ShiftPhaseMaturityRow],
    followup_middle_length: usize,
) -> Vec<ShiftPhaseResidueGateRow> {
    let mut rows = Vec::new();
    for spec in specs {
        for middle_length in [spec.source_middle_length, followup_middle_length] {
            if !maturity_rows.iter().any(|row| {
                row.track_name == spec.track_name
                    && row.source_middle_length == spec.source_middle_length
            }) {
                continue;
            }
            let residual = build_affine_phase_residual_row(
                spec.base,
                middle_length,
                spec.low_digit,
                spec.high_digit,
            );
            rows.extend(build_shift_phase_residue_gate_rows_for_residual(
                spec, &residual,
            ));
        }
    }
    rows
}

fn build_shift_phase_residue_gate_rows_for_residual(
    spec: &OwnedTrackSpec,
    row: &AffinePhaseResidualRow,
) -> Vec<ShiftPhaseResidueGateRow> {
    let low_high_lane = compact_lane(row.base, row.middle_length, row.low_digit, row.high_digit);
    let high_low_lane = compact_lane(row.base, row.middle_length, row.high_digit, row.low_digit);
    residue_moduli_for_base(row.base)
        .into_iter()
        .map(|modulus| {
            let low_high_survivor_count = survivor_count_for_moduli(&low_high_lane, &[modulus]);
            let high_low_survivor_count = survivor_count_for_moduli(&high_low_lane, &[modulus]);
            let seed_capacity = row.seed_capacity as f64;
            ShiftPhaseResidueGateRow {
                track_name: spec.track_name.clone(),
                track_kind: spec.track_kind.clone(),
                base: row.base,
                middle_length: row.middle_length,
                pair_label: row.low_high_pair_label.clone(),
                reverse_pair_label: row.high_low_pair_label.clone(),
                modulus,
                low_high_shift_modulus: (low_high_lane.shift % u64::from(modulus)) as u32,
                high_low_shift_modulus: (high_low_lane.shift % u64::from(modulus)) as u32,
                gradient_modulus: (low_high_lane.gradient % u64::from(modulus)) as u32,
                low_high_excluded_seed_classes: excluded_seed_classes_label(
                    &low_high_lane,
                    modulus,
                ),
                high_low_excluded_seed_classes: excluded_seed_classes_label(
                    &high_low_lane,
                    modulus,
                ),
                low_high_survivor_count,
                high_low_survivor_count,
                survivor_delta_pp: (low_high_survivor_count as f64 / seed_capacity
                    - high_low_survivor_count as f64 / seed_capacity)
                    * 100.0,
            }
        })
        .collect()
}

fn build_shift_phase_witness_rows(
    specs: &[OwnedTrackSpec],
    maturity_rows: &[ShiftPhaseMaturityRow],
    followup_middle_length: usize,
    witness_limit: usize,
) -> Vec<ShiftPhaseWitnessRow> {
    let mut rows = Vec::new();
    for spec in specs {
        if !maturity_rows
            .iter()
            .any(|row| row.track_name == spec.track_name)
        {
            continue;
        }
        for middle_length in [spec.source_middle_length, followup_middle_length] {
            for (orientation, outer, inner) in [
                ("low_high", spec.low_digit, spec.high_digit),
                ("high_low", spec.high_digit, spec.low_digit),
            ] {
                for witness in
                    compact_prime_witnesses(spec.base, middle_length, outer, inner, witness_limit)
                {
                    rows.push(ShiftPhaseWitnessRow {
                        track_name: spec.track_name.clone(),
                        track_kind: spec.track_kind.clone(),
                        base: spec.base,
                        middle_length,
                        orientation: orientation.to_string(),
                        pair_label: pair_label(outer, inner),
                        seed: witness.seed,
                        middle_digits: witness.middle_digits,
                        template_digits: witness.template_digits,
                        decimal_value: witness.decimal_value,
                    });
                }
            }
        }
    }
    rows
}

fn build_shift_phase_summary(
    shift_phase_rows: &[AffinePhaseResidualRow],
    maturity_rows: &[ShiftPhaseMaturityRow],
    foil_rows: &[ShiftPhaseFoilRow],
    residue_gate_rows: &[ShiftPhaseResidueGateRow],
) -> ShiftPhaseSignalMiningSummary {
    let strongest = maturity_rows.iter().max_by(|left, right| {
        left.followup_survivor_prime_residual_delta_pp
            .abs()
            .total_cmp(&right.followup_survivor_prime_residual_delta_pp.abs())
    });
    let anchor_label = maturity_rows
        .iter()
        .find(|row| row.track_name == "base30_anchor_1b")
        .map(|row| row.stability_label.clone())
        .unwrap_or_else(|| "missing".to_string());

    ShiftPhaseSignalMiningSummary {
        broad_residual_row_count: shift_phase_rows.len(),
        tracked_pair_count: maturity_rows.len(),
        maturity_row_count: maturity_rows.len(),
        foil_row_count: foil_rows.len(),
        residue_gate_row_count: residue_gate_rows.len(),
        persistent_or_amplifying_count: maturity_rows
            .iter()
            .filter(|row| {
                row.stability_label == "persistent" || row.stability_label == "amplifies"
            })
            .count(),
        strongest_followup_track: strongest
            .map(|row| row.track_name.clone())
            .unwrap_or_else(|| "none".to_string()),
        strongest_followup_pair: strongest
            .map(|row| format!("{} vs {}", row.pair_label, row.reverse_pair_label))
            .unwrap_or_else(|| "none".to_string()),
        strongest_followup_survivor_prime_residual_pp: strongest
            .map(|row| row.followup_survivor_prime_residual_delta_pp)
            .unwrap_or(0.0),
        base30_anchor_stability_label: anchor_label,
        public_phrase: "shift-phase residual".to_string(),
        strong_line:
            "same slope, different intercept, different residue weather can be mined as a local affine phase signal."
                .to_string(),
        caution_line:
            "ranked phase residual leads are not density laws until they survive controls beyond this report."
                .to_string(),
    }
}

fn find_residual_row(
    rows: &[AffinePhaseResidualRow],
    base: u32,
    middle_length: usize,
    low_digit: u32,
    high_digit: u32,
) -> Option<&AffinePhaseResidualRow> {
    rows.iter().find(|row| {
        row.base == base
            && row.middle_length == middle_length
            && row.low_digit == low_digit
            && row.high_digit == high_digit
    })
}

pub fn classify_shift_phase_stability(
    source: &AffinePhaseResidualRow,
    followup: &AffinePhaseResidualRow,
) -> String {
    let residue_abs = source.residue_survivor_delta_pp.abs();
    let survivor_abs = source.survivor_prime_residual_delta_pp.abs();
    if residue_abs > 1.0 && residue_abs > survivor_abs * 1.25 {
        return "residue_only".to_string();
    }
    if source.middle_length == 1 {
        return "volatile".to_string();
    }

    let source_signal = source.survivor_prime_residual_delta_pp;
    let followup_signal = followup.survivor_prime_residual_delta_pp;
    let source_sign = signed_direction(source_signal);
    let followup_sign = signed_direction(followup_signal);
    if source_sign != 0 && followup_sign != 0 && source_sign != followup_sign {
        return "reverses".to_string();
    }
    if followup_signal.abs() > source_signal.abs() + 0.25 {
        return "amplifies".to_string();
    }
    if followup_signal.abs() + 0.25 < source_signal.abs() {
        return "fades".to_string();
    }
    "persistent".to_string()
}

fn signed_direction(value: f64) -> i8 {
    if value > 1e-9 {
        1
    } else if value < -1e-9 {
        -1
    } else {
        0
    }
}

fn compact_prime_witnesses(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    witness_limit: usize,
) -> Vec<CompactWitness> {
    let lane = compact_lane(base, middle_length, outer, inner);
    let mut witnesses = Vec::new();
    for seed in 0..lane.seed_capacity {
        let value = lane
            .candidate_value(seed)
            .expect("compact phase residual lane should fit u64");
        if primal::is_prime(value) {
            witnesses.push(CompactWitness {
                seed,
                middle_digits: lane.middle_digits(seed),
                template_digits: lane.template_digits(seed),
                decimal_value: value,
            });
            if witnesses.len() >= witness_limit {
                break;
            }
        }
    }
    witnesses
}

fn compact_lane(base: u32, middle_length: usize, outer: u32, inner: u32) -> FastAffineLane {
    build_fast_affine_lane(FastLaneConfig::new(
        base,
        outer,
        inner,
        middle_length,
        PHASE_RESIDUAL_K,
    ))
    .expect("compact phase residual lane should fit u64 for v1 scope")
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

fn lead_tag(
    size_expected_delta_pp: f64,
    residue_survivor_delta_pp: f64,
    survivor_prime_residual_delta_pp: f64,
) -> String {
    let mut components = [
        ("size_led", size_expected_delta_pp.abs()),
        ("residue_survival_led", residue_survivor_delta_pp.abs()),
        ("survivor_prime_led", survivor_prime_residual_delta_pp.abs()),
    ];
    components.sort_by(|left, right| right.1.total_cmp(&left.1));
    if components[0].1 < 0.001 || components[1].1 / components[0].1 >= 0.8 {
        "mixed".to_string()
    } else {
        components[0].0.to_string()
    }
}

fn pair_label(outer: u32, inner: u32) -> String {
    format!("({},{})", digit_symbol(outer), digit_symbol(inner))
}

fn moduli_label(moduli: &[u32]) -> String {
    moduli
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn pow_i128(base: i128, exp: usize) -> i128 {
    let mut value = 1i128;
    for _ in 0..exp {
        value *= base;
    }
    value
}

fn mean(values: impl Iterator<Item = f64>) -> f64 {
    let mut count = 0usize;
    let mut total = 0.0;
    for value in values {
        if value.is_finite() {
            count += 1;
            total += value;
        }
    }
    if count == 0 {
        0.0
    } else {
        total / count as f64
    }
}

fn gcd_u32(mut left: u32, mut right: u32) -> u32 {
    while right != 0 {
        let tmp = left % right;
        left = right;
        right = tmp;
    }
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    fn single_base_settings(base: u32, middle_length: usize) -> AffinePhaseResidualSettings {
        AffinePhaseResidualSettings {
            bases: vec![base],
            min_middle_length: middle_length,
            max_middle_length: middle_length,
            top_limit: 4,
            witness_limit: 2,
        }
    }

    #[test]
    fn default_compact_sweep_has_expected_unordered_count() {
        let atlas = build_affine_phase_residual_atlas(AffinePhaseResidualSettings::default());
        let expected_per_m = DEFAULT_PHASE_RESIDUAL_BASES
            .iter()
            .map(|&base| {
                let unit_count = unit_residues(base).len();
                unit_count * (unit_count - 1) / 2
            })
            .sum::<usize>();
        let expected_total = expected_per_m
            * (DEFAULT_PHASE_RESIDUAL_MAX_MIDDLE_LENGTH - DEFAULT_PHASE_RESIDUAL_MIN_MIDDLE_LENGTH
                + 1);

        assert_eq!(expected_per_m, 281);
        assert_eq!(atlas.phase_residual_rows.len(), expected_total);
        assert_eq!(atlas.summary.residual_row_count, expected_total);
    }

    #[test]
    fn base30_anchor_counts_match_reversal_residual_report() {
        let atlas = build_affine_phase_residual_atlas(AffinePhaseResidualSettings {
            bases: vec![30],
            min_middle_length: 2,
            max_middle_length: 3,
            top_limit: 4,
            witness_limit: 2,
        });

        let m2 = atlas
            .phase_residual_rows
            .iter()
            .find(|row| row.middle_length == 2 && row.low_digit == 1 && row.high_digit == 11)
            .expect("base-30 M=2 anchor row");
        let m3 = atlas
            .phase_residual_rows
            .iter()
            .find(|row| row.middle_length == 3 && row.low_digit == 1 && row.high_digit == 11)
            .expect("base-30 M=3 anchor row");

        assert_eq!(m2.low_high_prime_hits, 191);
        assert_eq!(m2.high_low_prime_hits, 194);
        assert_eq!(m3.low_high_prime_hits, 4877);
        assert_eq!(m3.high_low_prime_hits, 4451);
    }

    #[test]
    fn compact_shift_delta_formula_matches_direct_affine_shifts() {
        for (base, middle_length, low, high) in [(10, 2, 3, 7), (22, 2, 17, 19), (34, 3, 1, 33)] {
            let low_high = compact_lane(base, middle_length, low, high);
            let high_low = compact_lane(base, middle_length, high, low);
            let direct = low_high.shift as i128 - high_low.shift as i128;
            assert_eq!(
                direct,
                compact_reversal_shift_delta(base, middle_length, low, high)
            );
            assert_eq!(low_high.gradient, high_low.gradient);
        }
    }

    #[test]
    fn residue_moduli_filter_primes_dividing_base() {
        assert_eq!(
            residue_moduli_for_base(30),
            vec![7, 11, 13, 17, 19, 23, 29, 31]
        );
        assert_eq!(
            residue_moduli_for_base(14),
            vec![3, 5, 11, 13, 17, 19, 23, 29, 31]
        );
        assert_eq!(
            residue_moduli_for_base(6),
            vec![5, 7, 11, 13, 17, 19, 23, 29, 31]
        );
    }

    #[test]
    fn combined_survivor_counts_match_exhaustive_filtering() {
        let atlas = build_affine_phase_residual_atlas(single_base_settings(22, 2));
        let row = atlas
            .phase_residual_rows
            .iter()
            .find(|row| row.low_digit == 1 && row.high_digit == 3)
            .expect("base-22 test row");
        let moduli = residue_moduli_for_base(row.base);
        let low_high_lane =
            compact_lane(row.base, row.middle_length, row.low_digit, row.high_digit);
        let high_low_lane =
            compact_lane(row.base, row.middle_length, row.high_digit, row.low_digit);
        let low_high_count = (0..low_high_lane.seed_capacity)
            .filter(|&seed| seed_survives_moduli(&low_high_lane, seed, &moduli))
            .count();
        let high_low_count = (0..high_low_lane.seed_capacity)
            .filter(|&seed| seed_survives_moduli(&high_low_lane, seed, &moduli))
            .count();

        assert_eq!(row.low_high_survivor_count, low_high_count);
        assert_eq!(row.high_low_survivor_count, high_low_count);
        assert!(atlas
            .phase_modulus_rows
            .iter()
            .filter(|mod_row| {
                mod_row.base == row.base
                    && mod_row.middle_length == row.middle_length
                    && mod_row.low_digit == row.low_digit
                    && mod_row.high_digit == row.high_digit
            })
            .all(|mod_row| {
                !mod_row.low_high_excluded_seed_classes.is_empty()
                    && !mod_row.high_low_excluded_seed_classes.is_empty()
            }));
    }

    #[test]
    fn top_signal_rows_and_witnesses_are_report_ready() {
        let atlas = build_affine_phase_residual_atlas(single_base_settings(30, 3));
        assert!(!atlas.top_signal_rows.is_empty());
        assert!(atlas.top_signal_rows.iter().any(|row| {
            row.selection_reason == "base30_anchor" || row.low_high_pair_label == "(1,B)"
        }));
        assert!(!atlas.witness_rows.is_empty());
        for witness in &atlas.witness_rows {
            assert!(primal::is_prime(witness.decimal_value));
            assert!(!witness.template_digits.is_empty());
        }
    }

    #[test]
    fn shift_phase_report_reproduces_anchor_counts_without_heavy_followup() {
        let settings = ShiftPhaseSignalMiningSettings {
            base_settings: single_base_settings(30, 3),
            followup_middle_length: 3,
            top_limit: 1,
            witness_limit: 1,
        };
        let focus_specs = [ShiftPhaseTrackSpec {
            track_name: "base30_anchor_1b",
            track_kind: "focus_lead",
            base: 30,
            low_digit: 1,
            high_digit: 11,
            source_middle_length: 3,
            note: "test anchor",
        }];

        let report = build_shift_phase_signal_mining_report_for_specs(settings, &focus_specs, &[]);
        let anchor = report
            .maturity_rows
            .iter()
            .find(|row| row.track_name == "base30_anchor_1b")
            .expect("base-30 shift-phase anchor row");

        assert_eq!(anchor.source_prime_hits, "4877 / 4451");
        assert_eq!(anchor.followup_prime_hits, "4877 / 4451");
        assert_eq!(report.summary.base30_anchor_stability_label, "persistent");
        assert_eq!(
            report.summary.residue_gate_row_count,
            report.residue_gate_rows.len()
        );
    }

    #[test]
    fn shift_phase_base10_mature_followup_is_deterministic_and_u64_safe() {
        let settings = ShiftPhaseSignalMiningSettings {
            base_settings: single_base_settings(10, 3),
            followup_middle_length: 4,
            top_limit: 1,
            witness_limit: 2,
        };
        let focus_specs = [
            ShiftPhaseTrackSpec {
                track_name: "base10_low_outer_17",
                track_kind: "focus_lead",
                base: 10,
                low_digit: 1,
                high_digit: 7,
                source_middle_length: 3,
                note: "test base-10 lead",
            },
            ShiftPhaseTrackSpec {
                track_name: "base10_classic_37",
                track_kind: "focus_lead",
                base: 10,
                low_digit: 3,
                high_digit: 7,
                source_middle_length: 3,
                note: "test classic decimal lead",
            },
        ];

        let report = build_shift_phase_signal_mining_report_for_specs(settings, &focus_specs, &[]);
        let row_17 = report
            .maturity_rows
            .iter()
            .find(|row| row.track_name == "base10_low_outer_17")
            .expect("base-10 (1,7) maturity row");
        let row_37 = report
            .maturity_rows
            .iter()
            .find(|row| row.track_name == "base10_classic_37")
            .expect("base-10 (3,7) maturity row");

        assert_eq!(row_17.source_prime_hits, "183 / 153");
        assert_eq!(row_37.source_prime_hits, "170 / 146");
        assert_eq!(row_17.followup_seed_capacity, 10_000);
        assert_eq!(row_37.followup_seed_capacity, 10_000);
        assert!(!report.witness_rows.is_empty());
        for witness in &report.witness_rows {
            assert!(primal::is_prime(witness.decimal_value));
        }
    }

    #[test]
    fn shift_phase_stability_labels_follow_explicit_rules() {
        let residue_source = build_affine_phase_residual_row(10, 1, 3, 9);
        let residue_followup = build_affine_phase_residual_row(10, 4, 3, 9);
        assert_eq!(
            classify_shift_phase_stability(&residue_source, &residue_followup),
            "residue_only"
        );

        let volatile_source = build_affine_phase_residual_row(10, 1, 1, 7);
        let volatile_followup = build_affine_phase_residual_row(10, 4, 1, 7);
        assert_eq!(
            classify_shift_phase_stability(&volatile_source, &volatile_followup),
            "volatile"
        );
    }
}
