//! Unit-cycle normalized phase signal helpers.
//!
//! This layer annotates compact same-gradient reversal rows from the affine
//! phase residual atlas with base-normalized unit-cycle geometry. The goal is
//! to rank structural arc patterns, not to claim a density theorem.

use crate::validation::{
    affine_phase_residual::{
        build_affine_phase_residual_atlas, build_affine_phase_residual_row, AffinePhaseResidualRow,
        AffinePhaseResidualSettings, ShiftPhaseTrackSpec, DEFAULT_SHIFT_PHASE_FOIL_SPECS,
        DEFAULT_SHIFT_PHASE_FOLLOWUP_MIDDLE_LENGTH, PHASE_RESIDUAL_K,
    },
    bounded_k::{cyclic_unit_distance, digit_symbol, unit_residues},
    fast_affine::{build_fast_affine_lane, FastLaneConfig},
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

pub const DEFAULT_UNIT_CYCLE_TOP_BUCKET_LIMIT: usize = 8;
pub const DEFAULT_UNIT_CYCLE_REPRESENTATIVES_PER_BUCKET: usize = 2;
pub const DEFAULT_UNIT_CYCLE_PAIR_TOP_LIMIT: usize = 12;
pub const DEFAULT_UNIT_CYCLE_WITNESS_LIMIT: usize = 4;
pub const DEFAULT_UNIT_CYCLE_MIN_BUCKET_ROWS: usize = 3;
pub const DEFAULT_UNIT_CYCLE_MIN_SAME_SIGN_SHARE: f64 = 0.70;
pub const DEFAULT_UNIT_CYCLE_MIN_MEAN_ABS_SURVIVOR_RESIDUAL_PP: f64 = 1.0;

pub const DEFAULT_UNIT_CYCLE_ANCHOR_SPECS: &[ShiftPhaseTrackSpec] = &[
    ShiftPhaseTrackSpec {
        track_name: "base6_bridge_15",
        track_kind: "curated_anchor",
        base: 6,
        low_digit: 1,
        high_digit: 5,
        source_middle_length: 3,
        note: "base-6 complement bridge anchor",
    },
    ShiftPhaseTrackSpec {
        track_name: "base10_low_outer_17",
        track_kind: "curated_anchor",
        base: 10,
        low_digit: 1,
        high_digit: 7,
        source_middle_length: 3,
        note: "decimal low-edge diameter anchor",
    },
    ShiftPhaseTrackSpec {
        track_name: "base10_classic_37",
        track_kind: "curated_anchor",
        base: 10,
        low_digit: 3,
        high_digit: 7,
        source_middle_length: 3,
        note: "classic decimal membrane anchor",
    },
    ShiftPhaseTrackSpec {
        track_name: "base30_anchor_1b",
        track_kind: "curated_anchor",
        base: 30,
        low_digit: 1,
        high_digit: 11,
        source_middle_length: 3,
        note: "base-30 reversal residual anchor",
    },
    ShiftPhaseTrackSpec {
        track_name: "base30_complement_1t",
        track_kind: "curated_anchor",
        base: 30,
        low_digit: 1,
        high_digit: 29,
        source_middle_length: 3,
        note: "base-30 wrap-edge complement anchor",
    },
    ShiftPhaseTrackSpec {
        track_name: "base22_low_outer_1l",
        track_kind: "curated_anchor",
        base: 22,
        low_digit: 1,
        high_digit: 21,
        source_middle_length: 3,
        note: "base-22 complement anchor",
    },
    ShiftPhaseTrackSpec {
        track_name: "base26_low_outer_1p",
        track_kind: "curated_anchor",
        base: 26,
        low_digit: 1,
        high_digit: 25,
        source_middle_length: 3,
        note: "base-26 complement anchor",
    },
    ShiftPhaseTrackSpec {
        track_name: "base34_low_outer_1v",
        track_kind: "curated_anchor",
        base: 34,
        low_digit: 1,
        high_digit: 31,
        source_middle_length: 3,
        note: "base-34 high-edge anchor",
    },
];

#[derive(Debug, Clone, Serialize)]
pub struct UnitCyclePhaseSettings {
    pub base_settings: AffinePhaseResidualSettings,
    pub followup_middle_length: usize,
    pub top_bucket_limit: usize,
    pub representatives_per_bucket: usize,
    pub pair_top_limit: usize,
    pub witness_limit: usize,
    pub min_bucket_rows: usize,
    pub min_same_sign_share: f64,
    pub min_mean_abs_survivor_residual_pp: f64,
}

impl Default for UnitCyclePhaseSettings {
    fn default() -> Self {
        Self {
            base_settings: AffinePhaseResidualSettings::default(),
            followup_middle_length: DEFAULT_SHIFT_PHASE_FOLLOWUP_MIDDLE_LENGTH,
            top_bucket_limit: DEFAULT_UNIT_CYCLE_TOP_BUCKET_LIMIT,
            representatives_per_bucket: DEFAULT_UNIT_CYCLE_REPRESENTATIVES_PER_BUCKET,
            pair_top_limit: DEFAULT_UNIT_CYCLE_PAIR_TOP_LIMIT,
            witness_limit: DEFAULT_UNIT_CYCLE_WITNESS_LIMIT,
            min_bucket_rows: DEFAULT_UNIT_CYCLE_MIN_BUCKET_ROWS,
            min_same_sign_share: DEFAULT_UNIT_CYCLE_MIN_SAME_SIGN_SHARE,
            min_mean_abs_survivor_residual_pp: DEFAULT_UNIT_CYCLE_MIN_MEAN_ABS_SURVIVOR_RESIDUAL_PP,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UnitCycleGeometry {
    pub unit_count: usize,
    pub low_unit_index: usize,
    pub high_unit_index: usize,
    pub low_unit_position: f64,
    pub high_unit_position: f64,
    pub forward_gap: usize,
    pub reverse_gap: usize,
    pub cyclic_distance: usize,
    pub distance_fraction: f64,
    pub distance_label: String,
    pub low_to_high_arc_label: String,
    pub edge_label: String,
    pub edge_pair: bool,
    pub base_complement: bool,
    pub low_edge: bool,
    pub wrap_edge: bool,
    pub cycle_bucket_label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnitCyclePhaseRow {
    pub base: u32,
    pub middle_length: usize,
    pub low_digit: u32,
    pub high_digit: u32,
    pub low_high_pair_label: String,
    pub high_low_pair_label: String,
    pub seed_capacity: u64,
    pub unit_count: usize,
    pub low_unit_index: usize,
    pub high_unit_index: usize,
    pub low_unit_position: f64,
    pub high_unit_position: f64,
    pub forward_gap: usize,
    pub reverse_gap: usize,
    pub cyclic_distance: usize,
    pub distance_fraction: f64,
    pub distance_label: String,
    pub low_to_high_arc_label: String,
    pub edge_label: String,
    pub edge_pair: bool,
    pub base_complement: bool,
    pub low_edge: bool,
    pub wrap_edge: bool,
    pub cycle_bucket_label: String,
    pub raw_delta_pp: f64,
    pub size_expected_delta_pp: f64,
    pub residual_after_size_pp: f64,
    pub residue_survivor_delta_pp: f64,
    pub survivor_prime_residual_delta_pp: f64,
    pub abs_survivor_prime_residual_delta_pp: f64,
    pub low_high_prime_hits: usize,
    pub high_low_prime_hits: usize,
    pub low_high_survivor_count: usize,
    pub high_low_survivor_count: usize,
    pub lead_tag: String,
    pub first_low_high_witness: String,
    pub first_high_low_witness: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnitCycleBucketRow {
    pub middle_length: usize,
    pub cycle_bucket_label: String,
    pub distance_label: String,
    pub low_to_high_arc_label: String,
    pub edge_label: String,
    pub row_count: usize,
    pub positive_count: usize,
    pub negative_count: usize,
    pub zero_count: usize,
    pub same_sign_share: f64,
    pub dominant_sign: String,
    pub mean_abs_residual_after_size_pp: f64,
    pub mean_abs_survivor_prime_residual_delta_pp: f64,
    pub mean_residue_survivor_delta_pp: f64,
    pub strongest_base: u32,
    pub strongest_pair_label: String,
    pub strongest_reverse_pair_label: String,
    pub strongest_survivor_prime_residual_delta_pp: f64,
    pub qualifies_signal_bucket: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnitCycleLeadRow {
    pub selection_reason: String,
    pub track_name: String,
    pub track_kind: String,
    pub note: String,
    pub base: u32,
    pub middle_length: usize,
    pub low_digit: u32,
    pub high_digit: u32,
    pub pair_label: String,
    pub reverse_pair_label: String,
    pub cycle_bucket_label: String,
    pub distance_label: String,
    pub low_to_high_arc_label: String,
    pub edge_label: String,
    pub distance_fraction: f64,
    pub raw_delta_pp: f64,
    pub residual_after_size_pp: f64,
    pub residue_survivor_delta_pp: f64,
    pub survivor_prime_residual_delta_pp: f64,
    pub lead_tag: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnitCycleMaturityRow {
    pub track_name: String,
    pub track_kind: String,
    pub note: String,
    pub base: u32,
    pub low_digit: u32,
    pub high_digit: u32,
    pub pair_label: String,
    pub reverse_pair_label: String,
    pub cycle_bucket_label: String,
    pub distance_label: String,
    pub low_to_high_arc_label: String,
    pub edge_label: String,
    pub source_middle_length: usize,
    pub followup_middle_length: usize,
    pub source_survivor_prime_residual_delta_pp: f64,
    pub followup_survivor_prime_residual_delta_pp: f64,
    pub source_residual_after_size_pp: f64,
    pub followup_residual_after_size_pp: f64,
    pub source_prime_hits: String,
    pub followup_prime_hits: String,
    pub followup_seed_capacity: u64,
    pub stability_label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnitCycleFoilRow {
    pub foil_name: String,
    pub foil_kind: String,
    pub base: u32,
    pub middle_length: usize,
    pub pair_label: String,
    pub reverse_pair_label: String,
    pub cycle_bucket_label: String,
    pub distance_label: String,
    pub edge_label: String,
    pub residual_after_size_pp: f64,
    pub residue_survivor_delta_pp: f64,
    pub survivor_prime_residual_delta_pp: f64,
    pub lead_tag: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnitCycleWitnessRow {
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
pub struct UnitCyclePhaseSummary {
    pub phase_row_count: usize,
    pub bucket_row_count: usize,
    pub qualifying_bucket_count: usize,
    pub lead_row_count: usize,
    pub maturity_row_count: usize,
    pub foil_row_count: usize,
    pub strongest_bucket_label: String,
    pub strongest_bucket_mean_abs_survivor_residual_pp: f64,
    pub strongest_mature_track: String,
    pub strongest_mature_pair: String,
    pub strongest_mature_survivor_prime_residual_pp: f64,
    pub public_phrase: String,
    pub strong_line: String,
    pub caution_line: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnitCyclePhaseSignalReport {
    pub settings: UnitCyclePhaseSettings,
    pub summary: UnitCyclePhaseSummary,
    pub unit_cycle_phase_rows: Vec<UnitCyclePhaseRow>,
    pub cycle_bucket_rows: Vec<UnitCycleBucketRow>,
    pub lead_rows: Vec<UnitCycleLeadRow>,
    pub maturity_rows: Vec<UnitCycleMaturityRow>,
    pub foil_rows: Vec<UnitCycleFoilRow>,
    pub witness_rows: Vec<UnitCycleWitnessRow>,
}

#[derive(Debug, Clone)]
struct SelectedSpec {
    track_name: String,
    track_kind: String,
    note: String,
    base: u32,
    low_digit: u32,
    high_digit: u32,
    source_middle_length: usize,
}

#[derive(Debug, Clone)]
struct CompactWitness {
    seed: u64,
    middle_digits: String,
    template_digits: String,
    decimal_value: u64,
}

pub fn build_unit_cycle_phase_signal_report(
    settings: UnitCyclePhaseSettings,
) -> UnitCyclePhaseSignalReport {
    build_unit_cycle_phase_signal_report_for_specs(
        settings,
        DEFAULT_UNIT_CYCLE_ANCHOR_SPECS,
        DEFAULT_SHIFT_PHASE_FOIL_SPECS,
    )
}

pub fn build_unit_cycle_phase_signal_report_for_specs(
    settings: UnitCyclePhaseSettings,
    anchor_specs: &[ShiftPhaseTrackSpec],
    foil_specs: &[ShiftPhaseTrackSpec],
) -> UnitCyclePhaseSignalReport {
    let atlas = build_affine_phase_residual_atlas(settings.base_settings.clone());
    let unit_cycle_phase_rows = atlas
        .phase_residual_rows
        .iter()
        .map(annotate_unit_cycle_phase_row)
        .collect::<Vec<_>>();
    let cycle_bucket_rows = build_cycle_bucket_rows(&unit_cycle_phase_rows, &settings);
    let lead_rows = build_lead_rows(
        &unit_cycle_phase_rows,
        &cycle_bucket_rows,
        &settings,
        anchor_specs,
        foil_specs,
    );
    let selected_specs = select_maturity_specs(&lead_rows);
    let maturity_rows = build_maturity_rows(&unit_cycle_phase_rows, &selected_specs, &settings);
    let foil_rows = build_foil_rows(&unit_cycle_phase_rows, foil_specs, &settings);
    let witness_rows = build_witness_rows(
        &selected_specs,
        settings.followup_middle_length,
        settings.witness_limit,
    );
    let summary = build_summary(
        &unit_cycle_phase_rows,
        &cycle_bucket_rows,
        &lead_rows,
        &maturity_rows,
        &foil_rows,
    );

    UnitCyclePhaseSignalReport {
        settings,
        summary,
        unit_cycle_phase_rows,
        cycle_bucket_rows,
        lead_rows,
        maturity_rows,
        foil_rows,
        witness_rows,
    }
}

pub fn normalize_unit_cycle_geometry(
    base: u32,
    low_digit: u32,
    high_digit: u32,
) -> UnitCycleGeometry {
    let units = unit_residues(base);
    let unit_count = units.len();
    let low_unit_index = units
        .iter()
        .position(|&digit| digit == low_digit)
        .expect("low digit should be a unit");
    let high_unit_index = units
        .iter()
        .position(|&digit| digit == high_digit)
        .expect("high digit should be a unit");
    let forward_gap = if high_unit_index >= low_unit_index {
        high_unit_index - low_unit_index
    } else {
        unit_count - (low_unit_index - high_unit_index)
    };
    let reverse_gap = unit_count - forward_gap;
    let cyclic_distance = cyclic_unit_distance(base, low_digit, high_digit);
    let distance_fraction = cyclic_distance as f64 / unit_count as f64;
    let distance_label = distance_label(unit_count, cyclic_distance).to_string();
    let low_to_high_arc_label = if forward_gap == reverse_gap {
        "diameter"
    } else if forward_gap > reverse_gap {
        "long_arc"
    } else {
        "short_arc"
    }
    .to_string();
    let edge_pair = low_unit_index == 0 && high_unit_index + 1 == unit_count;
    let base_complement = low_digit + high_digit == base;
    let low_edge = low_unit_index == 0 && !edge_pair;
    let wrap_edge = edge_pair || forward_gap > reverse_gap;
    let edge_label = if base_complement {
        "base_complement"
    } else if edge_pair {
        "edge_pair"
    } else if low_edge {
        "low_edge"
    } else {
        "interior"
    }
    .to_string();
    let cycle_bucket_label = format!("{distance_label}|{low_to_high_arc_label}|{edge_label}");

    UnitCycleGeometry {
        unit_count,
        low_unit_index,
        high_unit_index,
        low_unit_position: low_unit_index as f64 / unit_count as f64,
        high_unit_position: high_unit_index as f64 / unit_count as f64,
        forward_gap,
        reverse_gap,
        cyclic_distance,
        distance_fraction,
        distance_label,
        low_to_high_arc_label,
        edge_label,
        edge_pair,
        base_complement,
        low_edge,
        wrap_edge,
        cycle_bucket_label,
    }
}

pub fn annotate_unit_cycle_phase_row(row: &AffinePhaseResidualRow) -> UnitCyclePhaseRow {
    let geometry = normalize_unit_cycle_geometry(row.base, row.low_digit, row.high_digit);
    UnitCyclePhaseRow {
        base: row.base,
        middle_length: row.middle_length,
        low_digit: row.low_digit,
        high_digit: row.high_digit,
        low_high_pair_label: row.low_high_pair_label.clone(),
        high_low_pair_label: row.high_low_pair_label.clone(),
        seed_capacity: row.seed_capacity,
        unit_count: geometry.unit_count,
        low_unit_index: geometry.low_unit_index,
        high_unit_index: geometry.high_unit_index,
        low_unit_position: geometry.low_unit_position,
        high_unit_position: geometry.high_unit_position,
        forward_gap: geometry.forward_gap,
        reverse_gap: geometry.reverse_gap,
        cyclic_distance: geometry.cyclic_distance,
        distance_fraction: geometry.distance_fraction,
        distance_label: geometry.distance_label,
        low_to_high_arc_label: geometry.low_to_high_arc_label,
        edge_label: geometry.edge_label,
        edge_pair: geometry.edge_pair,
        base_complement: geometry.base_complement,
        low_edge: geometry.low_edge,
        wrap_edge: geometry.wrap_edge,
        cycle_bucket_label: geometry.cycle_bucket_label,
        raw_delta_pp: row.raw_delta_pp,
        size_expected_delta_pp: row.size_expected_delta_pp,
        residual_after_size_pp: row.residual_after_size_pp,
        residue_survivor_delta_pp: row.residue_survivor_delta_pp,
        survivor_prime_residual_delta_pp: row.survivor_prime_residual_delta_pp,
        abs_survivor_prime_residual_delta_pp: row.abs_survivor_prime_residual_delta_pp,
        low_high_prime_hits: row.low_high_prime_hits,
        high_low_prime_hits: row.high_low_prime_hits,
        low_high_survivor_count: row.low_high_survivor_count,
        high_low_survivor_count: row.high_low_survivor_count,
        lead_tag: row.lead_tag.clone(),
        first_low_high_witness: row.first_low_high_witness.clone(),
        first_high_low_witness: row.first_high_low_witness.clone(),
    }
}

fn build_cycle_bucket_rows(
    rows: &[UnitCyclePhaseRow],
    settings: &UnitCyclePhaseSettings,
) -> Vec<UnitCycleBucketRow> {
    let mut groups: BTreeMap<(usize, String, String, String, String), Vec<&UnitCyclePhaseRow>> =
        BTreeMap::new();
    for row in rows {
        groups
            .entry((
                row.middle_length,
                row.cycle_bucket_label.clone(),
                row.distance_label.clone(),
                row.low_to_high_arc_label.clone(),
                row.edge_label.clone(),
            ))
            .or_default()
            .push(row);
    }

    groups
        .into_iter()
        .map(
            |(
                (
                    middle_length,
                    cycle_bucket_label,
                    distance_label,
                    low_to_high_arc_label,
                    edge_label,
                ),
                group,
            )| {
                let positive_count = group
                    .iter()
                    .filter(|row| row.survivor_prime_residual_delta_pp > 1e-9)
                    .count();
                let negative_count = group
                    .iter()
                    .filter(|row| row.survivor_prime_residual_delta_pp < -1e-9)
                    .count();
                let zero_count = group.len() - positive_count - negative_count;
                let dominant_count = positive_count.max(negative_count).max(zero_count);
                let dominant_sign =
                    if positive_count >= negative_count && positive_count >= zero_count {
                        "positive"
                    } else if negative_count >= positive_count && negative_count >= zero_count {
                        "negative"
                    } else {
                        "zero"
                    }
                    .to_string();
                let same_sign_share = dominant_count as f64 / group.len() as f64;
                let strongest = group
                    .iter()
                    .max_by(|left, right| {
                        left.abs_survivor_prime_residual_delta_pp
                            .total_cmp(&right.abs_survivor_prime_residual_delta_pp)
                    })
                    .expect("bucket group should not be empty");
                let mean_abs_survivor = mean(
                    group
                        .iter()
                        .map(|row| row.abs_survivor_prime_residual_delta_pp),
                );
                let qualifies_signal_bucket = group.len() >= settings.min_bucket_rows
                    && same_sign_share >= settings.min_same_sign_share
                    && mean_abs_survivor >= settings.min_mean_abs_survivor_residual_pp;

                UnitCycleBucketRow {
                    middle_length,
                    cycle_bucket_label,
                    distance_label,
                    low_to_high_arc_label,
                    edge_label,
                    row_count: group.len(),
                    positive_count,
                    negative_count,
                    zero_count,
                    same_sign_share,
                    dominant_sign,
                    mean_abs_residual_after_size_pp: mean(
                        group.iter().map(|row| row.residual_after_size_pp.abs()),
                    ),
                    mean_abs_survivor_prime_residual_delta_pp: mean_abs_survivor,
                    mean_residue_survivor_delta_pp: mean(
                        group.iter().map(|row| row.residue_survivor_delta_pp),
                    ),
                    strongest_base: strongest.base,
                    strongest_pair_label: strongest.low_high_pair_label.clone(),
                    strongest_reverse_pair_label: strongest.high_low_pair_label.clone(),
                    strongest_survivor_prime_residual_delta_pp: strongest
                        .survivor_prime_residual_delta_pp,
                    qualifies_signal_bucket,
                }
            },
        )
        .collect()
}

fn build_lead_rows(
    phase_rows: &[UnitCyclePhaseRow],
    bucket_rows: &[UnitCycleBucketRow],
    settings: &UnitCyclePhaseSettings,
    anchor_specs: &[ShiftPhaseTrackSpec],
    foil_specs: &[ShiftPhaseTrackSpec],
) -> Vec<UnitCycleLeadRow> {
    let mut rows = Vec::new();
    let mut seen = BTreeSet::new();
    let mature_m = settings.base_settings.max_middle_length;

    let mut top_pairs = phase_rows
        .iter()
        .filter(|row| row.middle_length == mature_m)
        .collect::<Vec<_>>();
    top_pairs.sort_by(|left, right| {
        right
            .abs_survivor_prime_residual_delta_pp
            .total_cmp(&left.abs_survivor_prime_residual_delta_pp)
    });
    for row in top_pairs.into_iter().take(settings.pair_top_limit) {
        push_lead_row(
            &mut rows,
            &mut seen,
            row,
            "top_pair_survivor_residual",
            &format!(
                "top_pair_base{}_m{}_{}_{}",
                row.base,
                row.middle_length,
                digit_symbol(row.low_digit),
                digit_symbol(row.high_digit)
            ),
            "pair_lead",
            "strong M=3 survivor-prime residual after unit-cycle annotation",
        );
    }

    let mut selected_buckets = bucket_rows
        .iter()
        .filter(|row| row.middle_length == mature_m && row.qualifies_signal_bucket)
        .collect::<Vec<_>>();
    selected_buckets.sort_by(|left, right| {
        right
            .mean_abs_survivor_prime_residual_delta_pp
            .total_cmp(&left.mean_abs_survivor_prime_residual_delta_pp)
    });
    for (bucket_idx, bucket) in selected_buckets
        .into_iter()
        .take(settings.top_bucket_limit)
        .enumerate()
    {
        let mut representatives = phase_rows
            .iter()
            .filter(|row| {
                row.middle_length == bucket.middle_length
                    && row.cycle_bucket_label == bucket.cycle_bucket_label
            })
            .collect::<Vec<_>>();
        representatives.sort_by(|left, right| {
            right
                .abs_survivor_prime_residual_delta_pp
                .total_cmp(&left.abs_survivor_prime_residual_delta_pp)
        });
        for row in representatives
            .into_iter()
            .take(settings.representatives_per_bucket)
        {
            push_lead_row(
                &mut rows,
                &mut seen,
                row,
                &format!("top_bucket_{}", bucket_idx + 1),
                &format!(
                    "bucket{}_base{}_m{}_{}_{}",
                    bucket_idx + 1,
                    row.base,
                    row.middle_length,
                    digit_symbol(row.low_digit),
                    digit_symbol(row.high_digit)
                ),
                "bucket_lead",
                "representative from a coherent unit-cycle bucket",
            );
        }
    }

    for spec in anchor_specs {
        if let Some(row) = phase_rows.iter().find(|row| {
            row.base == spec.base
                && row.middle_length == spec.source_middle_length
                && row.low_digit == spec.low_digit
                && row.high_digit == spec.high_digit
        }) {
            push_lead_row(
                &mut rows,
                &mut seen,
                row,
                "curated_anchor",
                spec.track_name,
                spec.track_kind,
                spec.note,
            );
        }
    }

    for spec in foil_specs {
        if let Some(row) = phase_rows.iter().find(|row| {
            row.base == spec.base
                && row.middle_length == spec.source_middle_length
                && row.low_digit == spec.low_digit
                && row.high_digit == spec.high_digit
        }) {
            push_lead_row(
                &mut rows,
                &mut seen,
                row,
                "curated_foil",
                spec.track_name,
                spec.track_kind,
                spec.note,
            );
        }
    }

    rows
}

fn push_lead_row(
    rows: &mut Vec<UnitCycleLeadRow>,
    seen: &mut BTreeSet<(u32, usize, u32, u32, String)>,
    row: &UnitCyclePhaseRow,
    selection_reason: &str,
    track_name: &str,
    track_kind: &str,
    note: &str,
) {
    if !seen.insert((
        row.base,
        row.middle_length,
        row.low_digit,
        row.high_digit,
        selection_reason.to_string(),
    )) {
        return;
    }
    rows.push(UnitCycleLeadRow {
        selection_reason: selection_reason.to_string(),
        track_name: track_name.to_string(),
        track_kind: track_kind.to_string(),
        note: note.to_string(),
        base: row.base,
        middle_length: row.middle_length,
        low_digit: row.low_digit,
        high_digit: row.high_digit,
        pair_label: row.low_high_pair_label.clone(),
        reverse_pair_label: row.high_low_pair_label.clone(),
        cycle_bucket_label: row.cycle_bucket_label.clone(),
        distance_label: row.distance_label.clone(),
        low_to_high_arc_label: row.low_to_high_arc_label.clone(),
        edge_label: row.edge_label.clone(),
        distance_fraction: row.distance_fraction,
        raw_delta_pp: row.raw_delta_pp,
        residual_after_size_pp: row.residual_after_size_pp,
        residue_survivor_delta_pp: row.residue_survivor_delta_pp,
        survivor_prime_residual_delta_pp: row.survivor_prime_residual_delta_pp,
        lead_tag: row.lead_tag.clone(),
    });
}

fn select_maturity_specs(lead_rows: &[UnitCycleLeadRow]) -> Vec<SelectedSpec> {
    let mut specs = Vec::new();
    let mut seen = BTreeSet::new();
    for row in lead_rows.iter().filter(|row| {
        row.selection_reason.starts_with("top_bucket")
            || row.selection_reason == "curated_anchor"
            || row.selection_reason == "curated_foil"
    }) {
        if seen.insert((row.base, row.middle_length, row.low_digit, row.high_digit)) {
            specs.push(SelectedSpec {
                track_name: row.track_name.clone(),
                track_kind: row.track_kind.clone(),
                note: row.note.clone(),
                base: row.base,
                low_digit: row.low_digit,
                high_digit: row.high_digit,
                source_middle_length: row.middle_length,
            });
        }
    }
    specs
}

fn build_maturity_rows(
    phase_rows: &[UnitCyclePhaseRow],
    specs: &[SelectedSpec],
    settings: &UnitCyclePhaseSettings,
) -> Vec<UnitCycleMaturityRow> {
    specs
        .iter()
        .filter_map(|spec| {
            let source = phase_rows.iter().find(|row| {
                row.base == spec.base
                    && row.middle_length == spec.source_middle_length
                    && row.low_digit == spec.low_digit
                    && row.high_digit == spec.high_digit
            })?;
            let followup = build_affine_phase_residual_row(
                spec.base,
                settings.followup_middle_length,
                spec.low_digit,
                spec.high_digit,
            );
            let followup_annotated = annotate_unit_cycle_phase_row(&followup);
            Some(UnitCycleMaturityRow {
                track_name: spec.track_name.clone(),
                track_kind: spec.track_kind.clone(),
                note: spec.note.clone(),
                base: spec.base,
                low_digit: spec.low_digit,
                high_digit: spec.high_digit,
                pair_label: source.low_high_pair_label.clone(),
                reverse_pair_label: source.high_low_pair_label.clone(),
                cycle_bucket_label: source.cycle_bucket_label.clone(),
                distance_label: source.distance_label.clone(),
                low_to_high_arc_label: source.low_to_high_arc_label.clone(),
                edge_label: source.edge_label.clone(),
                source_middle_length: spec.source_middle_length,
                followup_middle_length: settings.followup_middle_length,
                source_survivor_prime_residual_delta_pp: source.survivor_prime_residual_delta_pp,
                followup_survivor_prime_residual_delta_pp: followup_annotated
                    .survivor_prime_residual_delta_pp,
                source_residual_after_size_pp: source.residual_after_size_pp,
                followup_residual_after_size_pp: followup_annotated.residual_after_size_pp,
                source_prime_hits: format!(
                    "{} / {}",
                    source.low_high_prime_hits, source.high_low_prime_hits
                ),
                followup_prime_hits: format!(
                    "{} / {}",
                    followup_annotated.low_high_prime_hits, followup_annotated.high_low_prime_hits
                ),
                followup_seed_capacity: followup_annotated.seed_capacity,
                stability_label: classify_unit_cycle_stability(source, &followup_annotated),
            })
        })
        .collect()
}

fn build_foil_rows(
    phase_rows: &[UnitCyclePhaseRow],
    foil_specs: &[ShiftPhaseTrackSpec],
    settings: &UnitCyclePhaseSettings,
) -> Vec<UnitCycleFoilRow> {
    foil_specs
        .iter()
        .filter(|spec| settings.base_settings.bases.contains(&spec.base))
        .filter_map(|spec| {
            let row = phase_rows.iter().find(|row| {
                row.base == spec.base
                    && row.middle_length == spec.source_middle_length
                    && row.low_digit == spec.low_digit
                    && row.high_digit == spec.high_digit
            })?;
            Some(UnitCycleFoilRow {
                foil_name: spec.track_name.to_string(),
                foil_kind: spec.track_kind.to_string(),
                base: spec.base,
                middle_length: spec.source_middle_length,
                pair_label: row.low_high_pair_label.clone(),
                reverse_pair_label: row.high_low_pair_label.clone(),
                cycle_bucket_label: row.cycle_bucket_label.clone(),
                distance_label: row.distance_label.clone(),
                edge_label: row.edge_label.clone(),
                residual_after_size_pp: row.residual_after_size_pp,
                residue_survivor_delta_pp: row.residue_survivor_delta_pp,
                survivor_prime_residual_delta_pp: row.survivor_prime_residual_delta_pp,
                lead_tag: row.lead_tag.clone(),
                note: spec.note.to_string(),
            })
        })
        .collect()
}

fn build_witness_rows(
    specs: &[SelectedSpec],
    followup_middle_length: usize,
    witness_limit: usize,
) -> Vec<UnitCycleWitnessRow> {
    let mut rows = Vec::new();
    for spec in specs {
        for middle_length in [spec.source_middle_length, followup_middle_length] {
            for (orientation, outer, inner) in [
                ("low_high", spec.low_digit, spec.high_digit),
                ("high_low", spec.high_digit, spec.low_digit),
            ] {
                for witness in
                    compact_prime_witnesses(spec.base, middle_length, outer, inner, witness_limit)
                {
                    rows.push(UnitCycleWitnessRow {
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

fn build_summary(
    phase_rows: &[UnitCyclePhaseRow],
    bucket_rows: &[UnitCycleBucketRow],
    lead_rows: &[UnitCycleLeadRow],
    maturity_rows: &[UnitCycleMaturityRow],
    foil_rows: &[UnitCycleFoilRow],
) -> UnitCyclePhaseSummary {
    let strongest_bucket = bucket_rows
        .iter()
        .filter(|row| row.qualifies_signal_bucket)
        .max_by(|left, right| {
            left.mean_abs_survivor_prime_residual_delta_pp
                .total_cmp(&right.mean_abs_survivor_prime_residual_delta_pp)
        })
        .or_else(|| {
            bucket_rows.iter().max_by(|left, right| {
                left.mean_abs_survivor_prime_residual_delta_pp
                    .total_cmp(&right.mean_abs_survivor_prime_residual_delta_pp)
            })
        });
    let strongest_mature = maturity_rows.iter().max_by(|left, right| {
        left.followup_survivor_prime_residual_delta_pp
            .abs()
            .total_cmp(&right.followup_survivor_prime_residual_delta_pp.abs())
    });

    UnitCyclePhaseSummary {
        phase_row_count: phase_rows.len(),
        bucket_row_count: bucket_rows.len(),
        qualifying_bucket_count: bucket_rows
            .iter()
            .filter(|row| row.qualifies_signal_bucket)
            .count(),
        lead_row_count: lead_rows.len(),
        maturity_row_count: maturity_rows.len(),
        foil_row_count: foil_rows.len(),
        strongest_bucket_label: strongest_bucket
            .map(|row| row.cycle_bucket_label.clone())
            .unwrap_or_else(|| "none".to_string()),
        strongest_bucket_mean_abs_survivor_residual_pp: strongest_bucket
            .map(|row| row.mean_abs_survivor_prime_residual_delta_pp)
            .unwrap_or(0.0),
        strongest_mature_track: strongest_mature
            .map(|row| row.track_name.clone())
            .unwrap_or_else(|| "none".to_string()),
        strongest_mature_pair: strongest_mature
            .map(|row| format!("{} vs {}", row.pair_label, row.reverse_pair_label))
            .unwrap_or_else(|| "none".to_string()),
        strongest_mature_survivor_prime_residual_pp: strongest_mature
            .map(|row| row.followup_survivor_prime_residual_delta_pp)
            .unwrap_or(0.0),
        public_phrase: "unit-cycle phase signal".to_string(),
        strong_line:
            "unit-cycle normalization turns base-local shift-phase leads into ranked arc-geometry hypotheses."
                .to_string(),
        caution_line:
            "unit-cycle phase leads are empirical search cues, not density laws or base-invariant theorems."
                .to_string(),
    }
}

fn distance_label(unit_count: usize, cyclic_distance: usize) -> &'static str {
    if unit_count.is_multiple_of(2) && cyclic_distance == unit_count / 2 {
        "diameter"
    } else if cyclic_distance == 1 {
        "adjacent"
    } else if (cyclic_distance as f64 / unit_count as f64) <= 0.25 {
        "near"
    } else {
        "wide"
    }
}

fn classify_unit_cycle_stability(
    source: &UnitCyclePhaseRow,
    followup: &UnitCyclePhaseRow,
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
    let lane = build_fast_affine_lane(FastLaneConfig::new(
        base,
        outer,
        inner,
        middle_length,
        PHASE_RESIDUAL_K,
    ))
    .expect("compact unit-cycle lane should fit u64 for v1 scope");
    let mut witnesses = Vec::new();
    for seed in 0..lane.seed_capacity {
        let value = lane
            .candidate_value(seed)
            .expect("compact unit-cycle lane should fit u64");
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

fn pair_label(outer: u32, inner: u32) -> String {
    format!("({},{})", digit_symbol(outer), digit_symbol(inner))
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

#[cfg(test)]
mod tests {
    use super::*;

    fn single_base_settings(base: u32, middle_length: usize) -> UnitCyclePhaseSettings {
        UnitCyclePhaseSettings {
            base_settings: AffinePhaseResidualSettings {
                bases: vec![base],
                min_middle_length: middle_length,
                max_middle_length: middle_length,
                top_limit: 4,
                witness_limit: 2,
            },
            followup_middle_length: middle_length,
            top_bucket_limit: 2,
            representatives_per_bucket: 1,
            pair_top_limit: 4,
            witness_limit: 1,
            min_bucket_rows: 1,
            min_same_sign_share: 0.0,
            min_mean_abs_survivor_residual_pp: 0.0,
        }
    }

    #[test]
    fn unit_cycle_geometry_normalizes_anchor_cases() {
        let base30 = normalize_unit_cycle_geometry(30, 1, 29);
        assert_eq!(base30.unit_count, 8);
        assert_eq!(base30.cyclic_distance, 1);
        assert_eq!(base30.low_to_high_arc_label, "long_arc");
        assert!(base30.base_complement);
        assert!(base30.edge_pair);
        assert_eq!(base30.edge_label, "base_complement");

        let base6 = normalize_unit_cycle_geometry(6, 1, 5);
        assert_eq!(base6.distance_label, "diameter");
        assert!(base6.base_complement);
        assert_eq!(base6.cyclic_distance, cyclic_unit_distance(6, 1, 5));

        let base10 = normalize_unit_cycle_geometry(10, 1, 7);
        assert_eq!(base10.distance_label, "diameter");
        assert!(!base10.base_complement);
        assert_eq!(base10.edge_label, "low_edge");
        assert_eq!(base10.cyclic_distance, cyclic_unit_distance(10, 1, 7));
    }

    #[test]
    fn default_source_surface_has_expected_unit_cycle_rows() {
        let report = build_unit_cycle_phase_signal_report_for_specs(
            UnitCyclePhaseSettings {
                followup_middle_length: 3,
                top_bucket_limit: 0,
                representatives_per_bucket: 1,
                pair_top_limit: 0,
                witness_limit: 1,
                ..UnitCyclePhaseSettings::default()
            },
            &[],
            &[],
        );
        let expected_per_m = crate::validation::affine_phase_residual::DEFAULT_PHASE_RESIDUAL_BASES
            .iter()
            .map(|&base| {
                let unit_count = unit_residues(base).len();
                unit_count * (unit_count - 1) / 2
            })
            .sum::<usize>();
        let expected_total = expected_per_m
            * (report.settings.base_settings.max_middle_length
                - report.settings.base_settings.min_middle_length
                + 1);

        assert_eq!(expected_total, 843);
        assert_eq!(report.unit_cycle_phase_rows.len(), expected_total);
        assert_eq!(report.summary.phase_row_count, expected_total);
    }

    #[test]
    fn bucket_rows_are_deterministic_and_report_representatives() {
        let settings = single_base_settings(30, 3);
        let report = build_unit_cycle_phase_signal_report_for_specs(
            settings,
            &[ShiftPhaseTrackSpec {
                track_name: "base30_anchor_1b",
                track_kind: "curated_anchor",
                base: 30,
                low_digit: 1,
                high_digit: 11,
                source_middle_length: 3,
                note: "test anchor",
            }],
            &[],
        );

        assert!(!report.cycle_bucket_rows.is_empty());
        assert!(report.cycle_bucket_rows.iter().all(|row| {
            !row.cycle_bucket_label.is_empty()
                && row.row_count > 0
                && row.same_sign_share.is_finite()
        }));
        assert!(report
            .lead_rows
            .iter()
            .any(|row| row.selection_reason == "curated_anchor"));
        assert!(report
            .maturity_rows
            .iter()
            .any(|row| row.track_name == "base30_anchor_1b"));
    }

    #[test]
    fn witness_rows_are_prime() {
        let settings = UnitCyclePhaseSettings {
            base_settings: AffinePhaseResidualSettings {
                bases: vec![10],
                min_middle_length: 3,
                max_middle_length: 3,
                top_limit: 2,
                witness_limit: 2,
            },
            followup_middle_length: 3,
            top_bucket_limit: 1,
            representatives_per_bucket: 1,
            pair_top_limit: 1,
            witness_limit: 2,
            min_bucket_rows: 1,
            min_same_sign_share: 0.0,
            min_mean_abs_survivor_residual_pp: 0.0,
        };
        let report = build_unit_cycle_phase_signal_report_for_specs(
            settings,
            &[ShiftPhaseTrackSpec {
                track_name: "base10_low_outer_17",
                track_kind: "curated_anchor",
                base: 10,
                low_digit: 1,
                high_digit: 7,
                source_middle_length: 3,
                note: "test anchor",
            }],
            &[],
        );

        assert!(!report.witness_rows.is_empty());
        for witness in &report.witness_rows {
            assert!(primal::is_prime(witness.decimal_value));
        }
    }
}
