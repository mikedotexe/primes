//! Construction-density atlas helpers.
//!
//! This module measures density drift across affine membrane prime families.
//! It keeps the arithmetic and control generation in the library so the report
//! example is presentation-only.

use crate::validation::{
    bounded_k::{digit_symbol, format_k},
    fast_affine::{
        build_fast_affine_lane, build_residue_wheel, FastAffineLane, FastLaneConfig,
        FastPrimeError, ResidueWheel,
    },
};
use serde::Serialize;
use std::collections::BTreeSet;

pub const DEFAULT_DENSITY_SAMPLE_COUNT: usize = 50_000;
pub const DEFAULT_DENSITY_EXACT_SEED_CAP: u64 = 10_000;
pub const DEFAULT_DENSITY_MAX_WITNESSES: usize = 8;
pub const DEFAULT_DENSITY_WHEEL_PERIOD_CAP: u64 = 1_000_000;
pub const DEFAULT_DENSITY_SAMPLE_SEED: u64 = 0xD3A5_1A7A_5EED_2026;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ConstructionDensitySettings {
    pub sample_count: usize,
    pub exact_seed_cap: u64,
    pub max_witnesses: usize,
    pub wheel_period_cap: u64,
    pub sample_seed: u64,
}

impl Default for ConstructionDensitySettings {
    fn default() -> Self {
        Self {
            sample_count: DEFAULT_DENSITY_SAMPLE_COUNT,
            exact_seed_cap: DEFAULT_DENSITY_EXACT_SEED_CAP,
            max_witnesses: DEFAULT_DENSITY_MAX_WITNESSES,
            wheel_period_cap: DEFAULT_DENSITY_WHEEL_PERIOD_CAP,
            sample_seed: DEFAULT_DENSITY_SAMPLE_SEED,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ConstructionFamilySpec {
    pub role: &'static str,
    pub category: &'static str,
    pub expected_quality: &'static str,
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub middle_length: usize,
    pub k_outer: u32,
    pub k_inner: u32,
    pub note: &'static str,
}

impl ConstructionFamilySpec {
    pub fn pair_label(self) -> String {
        format!(
            "({},{})",
            digit_symbol(self.outer),
            digit_symbol(self.inner)
        )
    }

    pub fn k_label(self) -> String {
        format_k((self.k_outer, self.k_inner))
    }

    pub fn fast_config(self) -> FastLaneConfig {
        FastLaneConfig::new(
            self.base,
            self.outer,
            self.inner,
            self.middle_length,
            (self.k_outer, self.k_inner),
        )
    }
}

pub const DEFAULT_CONSTRUCTION_DENSITY_SPECS: &[ConstructionFamilySpec] = &[
    ConstructionFamilySpec {
        role: "base6_champion_compact",
        category: "maintained",
        expected_quality: "good",
        base: 6,
        outer: 1,
        inner: 5,
        middle_length: 1,
        k_outer: 0,
        k_inner: 0,
        note: "base-aware compact bridge witness family",
    },
    ConstructionFamilySpec {
        role: "base10_classic_compact",
        category: "maintained",
        expected_quality: "good",
        base: 10,
        outer: 3,
        inner: 7,
        middle_length: 2,
        k_outer: 0,
        k_inner: 0,
        note: "classic compact decimal membrane lane",
    },
    ConstructionFamilySpec {
        role: "base10_visible_zero_run_k21",
        category: "maintained",
        expected_quality: "good",
        base: 10,
        outer: 3,
        inner: 7,
        middle_length: 2,
        k_outer: 2,
        k_inner: 1,
        note: "human-readable decimal teaching lane",
    },
    ConstructionFamilySpec {
        role: "base14_persistent_core",
        category: "maintained",
        expected_quality: "good",
        base: 14,
        outer: 13,
        inner: 11,
        middle_length: 2,
        k_outer: 0,
        k_inner: 0,
        note: "base-14 persistent-core representative",
    },
    ConstructionFamilySpec {
        role: "base22_compact_pocket",
        category: "maintained",
        expected_quality: "good",
        base: 22,
        outer: 17,
        inner: 19,
        middle_length: 2,
        k_outer: 0,
        k_inner: 0,
        note: "compact side of the base-22 period-lock pocket",
    },
    ConstructionFamilySpec {
        role: "base22_side_pocket_k22",
        category: "maintained",
        expected_quality: "good",
        base: 22,
        outer: 17,
        inner: 19,
        middle_length: 2,
        k_outer: 2,
        k_inner: 2,
        note: "higher-order base-22 period-lock side pocket",
    },
    ConstructionFamilySpec {
        role: "base30_wheel_like_compact",
        category: "maintained",
        expected_quality: "good",
        base: 30,
        outer: 11,
        inner: 7,
        middle_length: 2,
        k_outer: 0,
        k_inner: 0,
        note: "wheel-like base with many built-in small-prime exclusions",
    },
    ConstructionFamilySpec {
        role: "base10_under_padded_long_seed",
        category: "stress",
        expected_quality: "mixed",
        base: 10,
        outer: 3,
        inner: 7,
        middle_length: 6,
        k_outer: 0,
        k_inner: 0,
        note: "longer middle with too little zero separation",
    },
    ConstructionFamilySpec {
        role: "base10_over_padded_k33",
        category: "stress",
        expected_quality: "mixed",
        base: 10,
        outer: 3,
        inner: 7,
        middle_length: 2,
        k_outer: 3,
        k_inner: 3,
        note: "same visible pair with a heavy zero budget",
    },
    ConstructionFamilySpec {
        role: "base10_lopsided_k30",
        category: "stress",
        expected_quality: "mixed",
        base: 10,
        outer: 3,
        inner: 7,
        middle_length: 2,
        k_outer: 3,
        k_inner: 0,
        note: "lopsided zero-run stress lane",
    },
    ConstructionFamilySpec {
        role: "base10_nearby_weak_pair",
        category: "stress",
        expected_quality: "weak",
        base: 10,
        outer: 9,
        inner: 9,
        middle_length: 2,
        k_outer: 0,
        k_inner: 0,
        note: "same-base nearby weak-pair sanity lane",
    },
    ConstructionFamilySpec {
        role: "base10_nonunit_outer_sanity",
        category: "stress",
        expected_quality: "poor",
        base: 10,
        outer: 2,
        inner: 7,
        middle_length: 2,
        k_outer: 0,
        k_inner: 0,
        note: "non-unit outer digit; nearly all candidates are structurally composite",
    },
];

#[derive(Debug, Clone, Serialize)]
pub struct DensityAtlasRow {
    pub role: String,
    pub category: String,
    pub expected_quality: String,
    pub base: u32,
    pub pair_label: String,
    pub outer: u32,
    pub inner: u32,
    pub k_label: String,
    pub k_outer: u32,
    pub k_inner: u32,
    pub middle_length: usize,
    pub total_zero_budget: u32,
    pub zero_lopsidedness: u32,
    pub total_base_digits: usize,
    pub compactness_proxy: f64,
    pub boundary_unit_viable: bool,
    pub deterministic_u64_scope: bool,
    pub measurement_mode: String,
    pub sample_seed: u64,
    pub seed_capacity: u64,
    pub sample_count: usize,
    pub wheel_period: u64,
    pub wheel_moduli_label: String,
    pub raw_candidate_count: usize,
    pub residue_admissible_count: usize,
    pub residue_admissible_share: f64,
    pub prime_count: usize,
    pub raw_prime_rate: f64,
    pub prime_rate_among_admissible: f64,
    pub average_decimal_digits: f64,
    pub pnt_expected_density: f64,
    pub coprime_adjusted_expected_density: f64,
    pub lift_vs_pnt: Option<f64>,
    pub lift_vs_coprime_expected: Option<f64>,
    pub shift: Option<u64>,
    pub gradient: Option<u64>,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ControlAtlasRow {
    pub role: String,
    pub category: String,
    pub control_kind: String,
    pub available: bool,
    pub base: u32,
    pub pair_label: String,
    pub k_label: String,
    pub middle_length: usize,
    pub measurement_mode: String,
    pub sample_seed: u64,
    pub sample_count: usize,
    pub candidate_count: usize,
    pub prime_count: usize,
    pub prime_rate: Option<f64>,
    pub membrane_prime_rate: f64,
    pub membrane_lift_vs_control: Option<f64>,
    pub average_decimal_digits: Option<f64>,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WitnessAtlasRow {
    pub role: String,
    pub category: String,
    pub base: u32,
    pub pair_label: String,
    pub k_label: String,
    pub middle_length: usize,
    pub seed: u64,
    pub middle_digits: String,
    pub template_digits: String,
    pub decimal_value: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConstructionDensitySummary {
    pub lane_count: usize,
    pub in_scope_lane_count: usize,
    pub exact_lane_count: usize,
    pub sampled_lane_count: usize,
    pub best_lane: String,
    pub best_raw_prime_rate: f64,
    pub weakest_lane: String,
    pub weakest_raw_prime_rate: f64,
    pub average_membrane_prime_rate: f64,
    pub average_coprime_control_rate: f64,
    pub average_residue_admissible_share: f64,
    pub conservative_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConstructionDensityAtlas {
    pub settings: ConstructionDensitySettings,
    pub summary: ConstructionDensitySummary,
    pub density_rows: Vec<DensityAtlasRow>,
    pub control_rows: Vec<ControlAtlasRow>,
    pub witness_rows: Vec<WitnessAtlasRow>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ControlKind {
    RawRandomSameDigits,
    CoprimeRandomSameDigits,
    SameSlotRandom,
    SameBudgetScaffold,
}

impl ControlKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::RawRandomSameDigits => "raw_random_same_digits",
            Self::CoprimeRandomSameDigits => "coprime_random_same_digits",
            Self::SameSlotRandom => "same_slot_random",
            Self::SameBudgetScaffold => "same_budget_scaffold",
        }
    }
}

#[derive(Debug, Clone)]
struct CandidateSample {
    seeds: Vec<u64>,
    measurement_mode: &'static str,
}

#[derive(Debug, Clone)]
struct SlotRandomCandidate {
    value: u64,
    template_digits: String,
}

pub fn measure_construction_density_atlas(
    specs: &[ConstructionFamilySpec],
    settings: ConstructionDensitySettings,
) -> ConstructionDensityAtlas {
    let mut density_rows = Vec::new();
    let mut control_rows = Vec::new();
    let mut witness_rows = Vec::new();

    for spec in specs {
        let measured = measure_family(*spec, settings);
        control_rows.extend(measured.control_rows);
        witness_rows.extend(measured.witness_rows);
        density_rows.push(measured.density_row);
    }

    let summary = summarize_density_atlas(&density_rows, &control_rows);

    ConstructionDensityAtlas {
        settings,
        summary,
        density_rows,
        control_rows,
        witness_rows,
    }
}

struct MeasuredFamily {
    density_row: DensityAtlasRow,
    control_rows: Vec<ControlAtlasRow>,
    witness_rows: Vec<WitnessAtlasRow>,
}

fn measure_family(
    spec: ConstructionFamilySpec,
    settings: ConstructionDensitySettings,
) -> MeasuredFamily {
    let lane = match build_fast_affine_lane(spec.fast_config()) {
        Ok(lane) => lane,
        Err(err) => {
            return MeasuredFamily {
                density_row: out_of_scope_row(spec, settings, err),
                control_rows: Vec::new(),
                witness_rows: Vec::new(),
            };
        }
    };
    let wheel = match build_residue_wheel(&lane, settings.wheel_period_cap) {
        Ok(wheel) => wheel,
        Err(err) => {
            return MeasuredFamily {
                density_row: out_of_scope_row(spec, settings, err),
                control_rows: Vec::new(),
                witness_rows: Vec::new(),
            };
        }
    };
    let sample = seed_sample(&lane, spec, settings);
    let wheel_mask = wheel_mask(&wheel);
    let mut raw_candidate_count = 0usize;
    let mut residue_admissible_count = 0usize;
    let mut prime_count = 0usize;
    let mut sum_digits = 0.0;
    let mut sum_ln = 0.0;
    let mut witness_rows = Vec::new();

    for &seed in &sample.seeds {
        let Some(value) = lane.candidate_value(seed) else {
            continue;
        };
        raw_candidate_count += 1;
        sum_digits += decimal_digits(value) as f64;
        sum_ln += (value as f64).ln();

        if is_wheel_admissible(seed, &wheel, &wheel_mask) {
            residue_admissible_count += 1;
        }

        if primal::is_prime(value) {
            prime_count += 1;
            if witness_rows.len() < settings.max_witnesses {
                witness_rows.push(WitnessAtlasRow {
                    role: spec.role.to_string(),
                    category: spec.category.to_string(),
                    base: spec.base,
                    pair_label: spec.pair_label(),
                    k_label: spec.k_label(),
                    middle_length: spec.middle_length,
                    seed,
                    middle_digits: lane.middle_digits(seed),
                    template_digits: lane.template_digits(seed),
                    decimal_value: value,
                });
            }
        }
    }

    let raw_prime_rate = rate(prime_count, raw_candidate_count);
    let average_ln = if raw_candidate_count == 0 {
        0.0
    } else {
        sum_ln / raw_candidate_count as f64
    };
    let pnt_expected_density = if average_ln > 0.0 {
        1.0 / average_ln
    } else {
        0.0
    };
    let coprime_adjusted_expected_density =
        (pnt_expected_density * coprime_density_boost(spec.base)).min(1.0);
    let density_row = DensityAtlasRow {
        role: spec.role.to_string(),
        category: spec.category.to_string(),
        expected_quality: spec.expected_quality.to_string(),
        base: spec.base,
        pair_label: spec.pair_label(),
        outer: spec.outer,
        inner: spec.inner,
        k_label: spec.k_label(),
        k_outer: spec.k_outer,
        k_inner: spec.k_inner,
        middle_length: spec.middle_length,
        total_zero_budget: 2 * (spec.k_outer + spec.k_inner),
        zero_lopsidedness: spec.k_outer.abs_diff(spec.k_inner),
        total_base_digits: total_base_digits(spec),
        compactness_proxy: compactness_proxy(spec),
        boundary_unit_viable: boundary_unit_viable(spec),
        deterministic_u64_scope: true,
        measurement_mode: sample.measurement_mode.to_string(),
        sample_seed: settings.sample_seed,
        seed_capacity: lane.seed_capacity,
        sample_count: sample.seeds.len(),
        wheel_period: wheel.period,
        wheel_moduli_label: wheel
            .moduli
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("|"),
        raw_candidate_count,
        residue_admissible_count,
        residue_admissible_share: rate(residue_admissible_count, raw_candidate_count),
        prime_count,
        raw_prime_rate,
        prime_rate_among_admissible: rate(prime_count, residue_admissible_count),
        average_decimal_digits: if raw_candidate_count == 0 {
            0.0
        } else {
            sum_digits / raw_candidate_count as f64
        },
        pnt_expected_density,
        coprime_adjusted_expected_density,
        lift_vs_pnt: ratio(raw_prime_rate, pnt_expected_density),
        lift_vs_coprime_expected: ratio(raw_prime_rate, coprime_adjusted_expected_density),
        shift: Some(lane.shift),
        gradient: Some(lane.gradient),
        note: spec.note.to_string(),
    };

    let control_rows = measure_controls(spec, &lane, &sample, raw_prime_rate, settings);

    MeasuredFamily {
        density_row,
        control_rows,
        witness_rows,
    }
}

fn out_of_scope_row(
    spec: ConstructionFamilySpec,
    settings: ConstructionDensitySettings,
    err: FastPrimeError,
) -> DensityAtlasRow {
    DensityAtlasRow {
        role: spec.role.to_string(),
        category: spec.category.to_string(),
        expected_quality: spec.expected_quality.to_string(),
        base: spec.base,
        pair_label: spec.pair_label(),
        outer: spec.outer,
        inner: spec.inner,
        k_label: spec.k_label(),
        k_outer: spec.k_outer,
        k_inner: spec.k_inner,
        middle_length: spec.middle_length,
        total_zero_budget: 2 * (spec.k_outer + spec.k_inner),
        zero_lopsidedness: spec.k_outer.abs_diff(spec.k_inner),
        total_base_digits: total_base_digits(spec),
        compactness_proxy: compactness_proxy(spec),
        boundary_unit_viable: boundary_unit_viable(spec),
        deterministic_u64_scope: false,
        measurement_mode: "out_of_scope".to_string(),
        sample_seed: settings.sample_seed,
        seed_capacity: 0,
        sample_count: 0,
        wheel_period: 0,
        wheel_moduli_label: String::new(),
        raw_candidate_count: 0,
        residue_admissible_count: 0,
        residue_admissible_share: 0.0,
        prime_count: 0,
        raw_prime_rate: 0.0,
        prime_rate_among_admissible: 0.0,
        average_decimal_digits: 0.0,
        pnt_expected_density: 0.0,
        coprime_adjusted_expected_density: 0.0,
        lift_vs_pnt: None,
        lift_vs_coprime_expected: None,
        shift: None,
        gradient: None,
        note: format!("out of deterministic u64 scope: {err}"),
    }
}

fn measure_controls(
    spec: ConstructionFamilySpec,
    lane: &FastAffineLane,
    sample: &CandidateSample,
    membrane_prime_rate: f64,
    settings: ConstructionDensitySettings,
) -> Vec<ControlAtlasRow> {
    [
        ControlKind::RawRandomSameDigits,
        ControlKind::CoprimeRandomSameDigits,
        ControlKind::SameSlotRandom,
        ControlKind::SameBudgetScaffold,
    ]
    .into_iter()
    .map(|kind| measure_control(spec, lane, sample, kind, membrane_prime_rate, settings))
    .collect()
}

fn measure_control(
    spec: ConstructionFamilySpec,
    lane: &FastAffineLane,
    sample: &CandidateSample,
    kind: ControlKind,
    membrane_prime_rate: f64,
    settings: ConstructionDensitySettings,
) -> ControlAtlasRow {
    let mut rng = control_seed(spec, settings.sample_seed, kind);
    let scaffold_gaps = if kind == ControlKind::SameBudgetScaffold {
        alternate_scaffold_gaps(spec)
    } else {
        None
    };

    if kind == ControlKind::SameBudgetScaffold && scaffold_gaps.is_none() {
        return control_unavailable_row(
            spec,
            kind,
            sample,
            settings,
            membrane_prime_rate,
            "no noncanonical same-budget scaffold exists for zero budget",
        );
    }

    let mut candidate_count = 0usize;
    let mut prime_count = 0usize;
    let mut sum_digits = 0.0;

    for &seed in &sample.seeds {
        let value = match kind {
            ControlKind::RawRandomSameDigits => {
                let Some(reference) = lane.candidate_value(seed) else {
                    continue;
                };
                random_decimal_with_digits(decimal_digits(reference), &mut rng)
            }
            ControlKind::CoprimeRandomSameDigits => {
                let Some(reference) = lane.candidate_value(seed) else {
                    continue;
                };
                random_coprime_decimal_with_digits(decimal_digits(reference), spec.base, &mut rng)
            }
            ControlKind::SameSlotRandom => {
                same_slot_random_candidate(spec, &mut rng).map(|candidate| {
                    debug_assert_eq!(
                        candidate.template_digits.chars().count(),
                        total_base_digits(spec)
                    );
                    candidate.value
                })
            }
            ControlKind::SameBudgetScaffold => same_budget_scaffold_candidate(
                spec,
                seed,
                scaffold_gaps.expect("checked scaffold availability"),
            ),
        };

        let Some(value) = value else {
            continue;
        };
        candidate_count += 1;
        sum_digits += decimal_digits(value) as f64;
        if primal::is_prime(value) {
            prime_count += 1;
        }
    }

    let control_rate = rate(prime_count, candidate_count);
    ControlAtlasRow {
        role: spec.role.to_string(),
        category: spec.category.to_string(),
        control_kind: kind.as_str().to_string(),
        available: candidate_count > 0,
        base: spec.base,
        pair_label: spec.pair_label(),
        k_label: spec.k_label(),
        middle_length: spec.middle_length,
        measurement_mode: sample.measurement_mode.to_string(),
        sample_seed: settings.sample_seed,
        sample_count: sample.seeds.len(),
        candidate_count,
        prime_count,
        prime_rate: if candidate_count > 0 {
            Some(control_rate)
        } else {
            None
        },
        membrane_prime_rate,
        membrane_lift_vs_control: ratio(membrane_prime_rate, control_rate),
        average_decimal_digits: if candidate_count > 0 {
            Some(sum_digits / candidate_count as f64)
        } else {
            None
        },
        note: control_note(kind),
    }
}

fn control_unavailable_row(
    spec: ConstructionFamilySpec,
    kind: ControlKind,
    sample: &CandidateSample,
    settings: ConstructionDensitySettings,
    membrane_prime_rate: f64,
    note: &str,
) -> ControlAtlasRow {
    ControlAtlasRow {
        role: spec.role.to_string(),
        category: spec.category.to_string(),
        control_kind: kind.as_str().to_string(),
        available: false,
        base: spec.base,
        pair_label: spec.pair_label(),
        k_label: spec.k_label(),
        middle_length: spec.middle_length,
        measurement_mode: sample.measurement_mode.to_string(),
        sample_seed: settings.sample_seed,
        sample_count: sample.seeds.len(),
        candidate_count: 0,
        prime_count: 0,
        prime_rate: None,
        membrane_prime_rate,
        membrane_lift_vs_control: None,
        average_decimal_digits: None,
        note: note.to_string(),
    }
}

fn summarize_density_atlas(
    density_rows: &[DensityAtlasRow],
    control_rows: &[ControlAtlasRow],
) -> ConstructionDensitySummary {
    let in_scope = density_rows
        .iter()
        .filter(|row| row.deterministic_u64_scope)
        .collect::<Vec<_>>();
    let exact_lane_count = in_scope
        .iter()
        .filter(|row| row.measurement_mode == "exact")
        .count();
    let sampled_lane_count = in_scope
        .iter()
        .filter(|row| row.measurement_mode == "sampled")
        .count();

    let best = in_scope
        .iter()
        .max_by(|left, right| left.raw_prime_rate.total_cmp(&right.raw_prime_rate));
    let weakest = in_scope
        .iter()
        .min_by(|left, right| left.raw_prime_rate.total_cmp(&right.raw_prime_rate));
    let average_membrane_prime_rate = mean(in_scope.iter().map(|row| row.raw_prime_rate));
    let average_residue_admissible_share =
        mean(in_scope.iter().map(|row| row.residue_admissible_share));
    let average_coprime_control_rate = mean(control_rows.iter().filter_map(|row| {
        if row.control_kind == ControlKind::CoprimeRandomSameDigits.as_str() {
            row.prime_rate
        } else {
            None
        }
    }));

    ConstructionDensitySummary {
        lane_count: density_rows.len(),
        in_scope_lane_count: in_scope.len(),
        exact_lane_count,
        sampled_lane_count,
        best_lane: best
            .map(|row| row.role.clone())
            .unwrap_or_else(|| "none".to_string()),
        best_raw_prime_rate: best.map(|row| row.raw_prime_rate).unwrap_or(0.0),
        weakest_lane: weakest
            .map(|row| row.role.clone())
            .unwrap_or_else(|| "none".to_string()),
        weakest_raw_prime_rate: weakest.map(|row| row.raw_prime_rate).unwrap_or(0.0),
        average_membrane_prime_rate,
        average_coprime_control_rate,
        average_residue_admissible_share,
        conservative_takeaway: "Density drift is measured across controlled affine membrane families; high-yield lanes are candidate surfaces, not density theorems.".to_string(),
    }
}

fn seed_sample(
    lane: &FastAffineLane,
    spec: ConstructionFamilySpec,
    settings: ConstructionDensitySettings,
) -> CandidateSample {
    if lane.seed_capacity <= settings.exact_seed_cap {
        let seeds = (0..lane.seed_capacity).collect::<Vec<_>>();
        return CandidateSample {
            seeds,
            measurement_mode: "exact",
        };
    }

    let count = settings.sample_count.min(lane.seed_capacity as usize);
    let mut seeds = Vec::with_capacity(count);
    let start = control_seed(spec, settings.sample_seed, ControlKind::RawRandomSameDigits)
        % lane.seed_capacity;
    let mut stride = (control_seed(spec, settings.sample_seed, ControlKind::SameSlotRandom)
        % lane.seed_capacity)
        .max(1);
    while gcd_u64(stride, lane.seed_capacity) != 1 {
        stride = stride.saturating_add(1);
        if stride >= lane.seed_capacity {
            stride = 1;
            break;
        }
    }

    let mut seen = BTreeSet::new();
    let mut index = 0u64;
    while seeds.len() < count {
        let seed = (start + stride.saturating_mul(index)) % lane.seed_capacity;
        if seen.insert(seed) {
            seeds.push(seed);
        }
        index += 1;
    }

    CandidateSample {
        seeds,
        measurement_mode: "sampled",
    }
}

fn wheel_mask(wheel: &ResidueWheel) -> Vec<bool> {
    let mut mask = vec![false; wheel.period as usize];
    for &residue in &wheel.admissible_residues {
        mask[residue as usize] = true;
    }
    mask
}

fn is_wheel_admissible(seed: u64, wheel: &ResidueWheel, mask: &[bool]) -> bool {
    mask[(seed % wheel.period) as usize]
}

fn random_decimal_with_digits(digits: usize, rng: &mut u64) -> Option<u64> {
    if digits == 0 || digits > 19 {
        return None;
    }
    let mut value = u64::from(next_bounded(rng, 9) + 1);
    for _ in 1..digits {
        value = value
            .checked_mul(10)?
            .checked_add(u64::from(next_bounded(rng, 10)))?;
    }
    Some(value)
}

fn random_coprime_decimal_with_digits(digits: usize, base: u32, rng: &mut u64) -> Option<u64> {
    let factors = prime_divisors(base);
    for _ in 0..10_000 {
        let value = random_decimal_with_digits(digits, rng)?;
        if factors.iter().all(|&factor| value % u64::from(factor) != 0) {
            return Some(value);
        }
    }
    None
}

fn same_slot_random_candidate(
    spec: ConstructionFamilySpec,
    rng: &mut u64,
) -> Option<SlotRandomCandidate> {
    let units = unit_digits(spec.base);
    if units.is_empty() {
        return None;
    }
    let left_outer = units[next_bounded(rng, units.len() as u32) as usize];
    let right_outer = units[next_bounded(rng, units.len() as u32) as usize];
    let left_inner = next_bounded(rng, spec.base);
    let right_inner = next_bounded(rng, spec.base);
    let seed_digits = (0..spec.middle_length)
        .map(|_| next_bounded(rng, spec.base))
        .collect::<Vec<_>>();

    let mut digits = Vec::with_capacity(total_base_digits(spec));
    digits.push(left_outer);
    digits.extend(std::iter::repeat_n(0, spec.k_outer as usize));
    digits.push(left_inner);
    digits.extend(std::iter::repeat_n(0, spec.k_inner as usize));
    digits.extend_from_slice(&seed_digits);
    digits.extend(std::iter::repeat_n(0, spec.k_inner as usize));
    digits.push(right_inner);
    digits.extend(std::iter::repeat_n(0, spec.k_outer as usize));
    digits.push(right_outer);

    Some(SlotRandomCandidate {
        value: digits_to_u64(spec.base, &digits)?,
        template_digits: digits_to_string(&digits),
    })
}

fn same_budget_scaffold_candidate(
    spec: ConstructionFamilySpec,
    seed: u64,
    gaps: [u32; 4],
) -> Option<u64> {
    let seed_digits = seed_digits_from_index(seed, spec.base, spec.middle_length)?;
    let mut digits = Vec::with_capacity(total_base_digits(spec));
    digits.push(spec.outer);
    digits.extend(std::iter::repeat_n(0, gaps[0] as usize));
    digits.push(spec.inner);
    digits.extend(std::iter::repeat_n(0, gaps[1] as usize));
    digits.extend_from_slice(&seed_digits);
    digits.extend(std::iter::repeat_n(0, gaps[2] as usize));
    digits.push(spec.inner);
    digits.extend(std::iter::repeat_n(0, gaps[3] as usize));
    digits.push(spec.outer);
    digits_to_u64(spec.base, &digits)
}

fn alternate_scaffold_gaps(spec: ConstructionFamilySpec) -> Option<[u32; 4]> {
    let total = 2 * (spec.k_outer + spec.k_inner);
    if total == 0 {
        return None;
    }
    let canonical = [spec.k_outer, spec.k_inner, spec.k_inner, spec.k_outer];
    [[0, total, 0, 0], [0, 0, total, 0], [total, 0, 0, 0]]
        .into_iter()
        .find(|candidate| *candidate != canonical)
}

fn seed_digits_from_index(mut seed: u64, base: u32, width: usize) -> Option<Vec<u32>> {
    let mut digits = vec![0; width];
    for digit in digits.iter_mut().rev() {
        *digit = (seed % u64::from(base)).try_into().ok()?;
        seed /= u64::from(base);
    }
    Some(digits)
}

fn digits_to_u64(base: u32, digits: &[u32]) -> Option<u64> {
    let mut value = 0u128;
    for &digit in digits {
        if digit >= base {
            return None;
        }
        value = value
            .checked_mul(u128::from(base))?
            .checked_add(u128::from(digit))?;
    }
    value.try_into().ok()
}

fn digits_to_string(digits: &[u32]) -> String {
    digits.iter().map(|&digit| digit_char(digit)).collect()
}

fn digit_char(digit: u32) -> char {
    if digit < 10 {
        char::from_digit(digit, 10).expect("decimal digit")
    } else {
        char::from_u32('A' as u32 + digit - 10).expect("uppercase digit")
    }
}

fn control_note(kind: ControlKind) -> String {
    match kind {
        ControlKind::RawRandomSameDigits => {
            "uniform decimal integers matched to candidate decimal digit counts".to_string()
        }
        ControlKind::CoprimeRandomSameDigits => {
            "uniform decimal integers matched to digit count and coprime to rad(base)".to_string()
        }
        ControlKind::SameSlotRandom => {
            "same zero-slot grammar with randomized boundary, inner, and seed digits".to_string()
        }
        ControlKind::SameBudgetScaffold => {
            "same anchors and zero budget with a noncanonical zero placement".to_string()
        }
    }
}

fn total_base_digits(spec: ConstructionFamilySpec) -> usize {
    4 + spec.middle_length + (2 * (spec.k_outer + spec.k_inner)) as usize
}

fn compactness_proxy(spec: ConstructionFamilySpec) -> f64 {
    1.0 / total_base_digits(spec) as f64
}

fn boundary_unit_viable(spec: ConstructionFamilySpec) -> bool {
    gcd_u32(spec.outer, spec.base) == 1 && gcd_u32(spec.inner, spec.base) == 1
}

fn decimal_digits(value: u64) -> usize {
    value.to_string().len()
}

fn rate(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn ratio(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator > 0.0 && numerator.is_finite() && denominator.is_finite() {
        Some(numerator / denominator)
    } else {
        None
    }
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

fn coprime_density_boost(base: u32) -> f64 {
    prime_divisors(base)
        .into_iter()
        .map(|p| p as f64 / (p as f64 - 1.0))
        .product()
}

fn prime_divisors(mut value: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let mut p = 2u32;
    while p * p <= value {
        if value.is_multiple_of(p) {
            factors.push(p);
            while value.is_multiple_of(p) {
                value /= p;
            }
        }
        p += 1;
    }
    if value > 1 {
        factors.push(value);
    }
    factors
}

fn unit_digits(base: u32) -> Vec<u32> {
    (1..base)
        .filter(|&digit| gcd_u32(digit, base) == 1)
        .collect()
}

fn control_seed(spec: ConstructionFamilySpec, sample_seed: u64, kind: ControlKind) -> u64 {
    let kind_salt = match kind {
        ControlKind::RawRandomSameDigits => 0xA11C_E001_u64,
        ControlKind::CoprimeRandomSameDigits => 0xC0F1_E001_u64,
        ControlKind::SameSlotRandom => 0x5107_E001_u64,
        ControlKind::SameBudgetScaffold => 0x5CAF_F01D_u64,
    };
    sample_seed
        ^ kind_salt
        ^ (u64::from(spec.base) << 48)
        ^ (u64::from(spec.outer) << 32)
        ^ (u64::from(spec.inner) << 24)
        ^ (u64::from(spec.k_outer) << 16)
        ^ (u64::from(spec.k_inner) << 8)
        ^ spec.middle_length as u64
}

fn next_u64(rng: &mut u64) -> u64 {
    *rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
    *rng
}

fn next_bounded(rng: &mut u64, modulus: u32) -> u32 {
    ((next_u64(rng) >> 32) % u64::from(modulus)) as u32
}

fn gcd_u32(mut left: u32, mut right: u32) -> u32 {
    while right != 0 {
        let tmp = left % right;
        left = right;
        right = tmp;
    }
    left
}

fn gcd_u64(mut left: u64, mut right: u64) -> u64 {
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

    fn tiny_settings() -> ConstructionDensitySettings {
        ConstructionDensitySettings {
            sample_count: 25,
            exact_seed_cap: 1_000,
            max_witnesses: 5,
            wheel_period_cap: 1_000_000,
            sample_seed: DEFAULT_DENSITY_SAMPLE_SEED,
        }
    }

    #[test]
    fn known_visible_lanes_produce_density_rows_and_witnesses() {
        let specs = [
            ConstructionFamilySpec {
                role: "base6_test",
                category: "test",
                expected_quality: "good",
                base: 6,
                outer: 1,
                inner: 5,
                middle_length: 1,
                k_outer: 0,
                k_inner: 0,
                note: "test",
            },
            ConstructionFamilySpec {
                role: "base10_test",
                category: "test",
                expected_quality: "good",
                base: 10,
                outer: 3,
                inner: 7,
                middle_length: 2,
                k_outer: 2,
                k_inner: 1,
                note: "test",
            },
            ConstructionFamilySpec {
                role: "base22_test",
                category: "test",
                expected_quality: "good",
                base: 22,
                outer: 17,
                inner: 19,
                middle_length: 2,
                k_outer: 2,
                k_inner: 2,
                note: "test",
            },
        ];
        let atlas = measure_construction_density_atlas(&specs, tiny_settings());

        assert_eq!(atlas.density_rows.len(), 3);
        assert!(atlas
            .density_rows
            .iter()
            .all(|row| row.deterministic_u64_scope));
        assert!(atlas
            .density_rows
            .iter()
            .all(|row| row.measurement_mode == "exact"));
        assert!(atlas
            .witness_rows
            .iter()
            .any(|row| row.decimal_value == 2551));
        assert!(atlas
            .witness_rows
            .iter()
            .any(|row| row.decimal_value == 300_702_007_003));
        assert!(atlas
            .witness_rows
            .iter()
            .any(|row| row.decimal_value == 4_808_275_624_019_584_921));
    }

    #[test]
    fn controls_preserve_requested_constraints() {
        let spec = ConstructionFamilySpec {
            role: "control_test",
            category: "test",
            expected_quality: "mixed",
            base: 10,
            outer: 3,
            inner: 7,
            middle_length: 2,
            k_outer: 2,
            k_inner: 1,
            note: "test",
        };
        let mut rng = 12345;
        let decimal = random_decimal_with_digits(12, &mut rng).expect("decimal");
        assert_eq!(decimal_digits(decimal), 12);

        let coprime =
            random_coprime_decimal_with_digits(12, spec.base, &mut rng).expect("coprime decimal");
        assert_ne!(coprime % 2, 0);
        assert_ne!(coprime % 5, 0);

        let slot = same_slot_random_candidate(spec, &mut rng).expect("slot candidate");
        assert_eq!(
            slot.template_digits.chars().count(),
            total_base_digits(spec)
        );
        assert!(slot
            .template_digits
            .chars()
            .nth(1)
            .is_some_and(|ch| ch == '0'));

        let gaps = alternate_scaffold_gaps(spec).expect("alternate scaffold");
        assert_eq!(gaps.iter().sum::<u32>(), 2 * (spec.k_outer + spec.k_inner));
        assert_ne!(
            gaps,
            [spec.k_outer, spec.k_inner, spec.k_inner, spec.k_outer]
        );
        assert!(same_budget_scaffold_candidate(spec, 20, gaps).is_some());
    }

    #[test]
    fn hybrid_measurement_labels_exact_sampled_and_out_of_scope() {
        let exact_spec = ConstructionFamilySpec {
            role: "exact",
            category: "test",
            expected_quality: "good",
            base: 6,
            outer: 1,
            inner: 5,
            middle_length: 1,
            k_outer: 0,
            k_inner: 0,
            note: "test",
        };
        let sampled_spec = ConstructionFamilySpec {
            role: "sampled",
            category: "test",
            expected_quality: "mixed",
            base: 10,
            outer: 3,
            inner: 7,
            middle_length: 6,
            k_outer: 0,
            k_inner: 0,
            note: "test",
        };
        let out_of_scope_spec = ConstructionFamilySpec {
            role: "too_large",
            category: "test",
            expected_quality: "out",
            base: 10,
            outer: 3,
            inner: 7,
            middle_length: 20,
            k_outer: 0,
            k_inner: 0,
            note: "test",
        };
        let atlas = measure_construction_density_atlas(
            &[exact_spec, sampled_spec, out_of_scope_spec],
            ConstructionDensitySettings {
                sample_count: 25,
                exact_seed_cap: 100,
                max_witnesses: 3,
                wheel_period_cap: 1_000_000,
                sample_seed: DEFAULT_DENSITY_SAMPLE_SEED,
            },
        );

        let exact = atlas
            .density_rows
            .iter()
            .find(|row| row.role == "exact")
            .unwrap();
        assert_eq!(exact.measurement_mode, "exact");
        assert_eq!(exact.sample_count as u64, exact.seed_capacity);

        let sampled = atlas
            .density_rows
            .iter()
            .find(|row| row.role == "sampled")
            .unwrap();
        assert_eq!(sampled.measurement_mode, "sampled");
        assert_eq!(sampled.sample_count, 25);

        let out = atlas
            .density_rows
            .iter()
            .find(|row| row.role == "too_large")
            .unwrap();
        assert!(!out.deterministic_u64_scope);
        assert_eq!(out.measurement_mode, "out_of_scope");
    }
}
