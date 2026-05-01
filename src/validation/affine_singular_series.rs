//! Finite affine singular-profile scout.
//!
//! This module asks a deliberately conservative question: after ordinary size
//! and exact small-prime residue weather are accounted for, which maintained
//! affine membrane lanes still look interesting enough to follow?

use crate::validation::{
    affine_phase_residual::DEFAULT_SHIFT_PHASE_FOCUS_SPECS,
    bounded_k::{digit_symbol, format_k},
    construction_density::DEFAULT_CONSTRUCTION_DENSITY_SPECS,
    fast_affine::{build_fast_affine_lane, FastAffineLane, FastLaneConfig},
    large_affine_witness::is_probable_prime_u128,
};
use serde::Serialize;
use std::collections::BTreeSet;

pub const DEFAULT_SINGULAR_PRIME_BOUND: u32 = 97;
pub const DEFAULT_SINGULAR_EXACT_SEED_CAP: u64 = 1_000_000;
pub const DEFAULT_SINGULAR_SAMPLE_COUNT: usize = 50_000;
pub const DEFAULT_SINGULAR_SAMPLE_SEED: u64 = 0x51A6_2026;
pub const DEFAULT_SINGULAR_MAX_WITNESSES: usize = 5;
pub const DEFAULT_SINGULAR_TOP_LIMIT: usize = 12;

pub const AFFINE_SINGULAR_SERIES_EXPECTED_OUTPUTS: &[&str] = &[
    "report.md",
    "summary.json",
    "lane_rows.csv",
    "modulus_rows.csv",
    "residual_rank_rows.csv",
    "witness_rows.csv",
    "residual_ranking.png",
    "multiplier_decomposition.png",
    "modulus_gate_heatmap.png",
    "lead_gallery.png",
];

#[derive(Debug, Clone, Copy, Serialize)]
pub struct AffineSingularSeriesSettings {
    pub prime_bound: u32,
    pub exact_seed_cap: u64,
    pub sample_count: usize,
    pub sample_seed: u64,
    pub max_witnesses: usize,
    pub top_limit: usize,
}

impl Default for AffineSingularSeriesSettings {
    fn default() -> Self {
        Self {
            prime_bound: DEFAULT_SINGULAR_PRIME_BOUND,
            exact_seed_cap: DEFAULT_SINGULAR_EXACT_SEED_CAP,
            sample_count: DEFAULT_SINGULAR_SAMPLE_COUNT,
            sample_seed: DEFAULT_SINGULAR_SAMPLE_SEED,
            max_witnesses: DEFAULT_SINGULAR_MAX_WITNESSES,
            top_limit: DEFAULT_SINGULAR_TOP_LIMIT,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct AffineSingularLaneSpec {
    pub role: &'static str,
    pub category: &'static str,
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub middle_length: usize,
    pub k_outer: u32,
    pub k_inner: u32,
    pub note: &'static str,
}

impl AffineSingularLaneSpec {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ModulusClassification {
    BaseSafe,
    ActiveSeedGate,
    InactiveSafe,
    StructurallyBlocked,
}

impl ModulusClassification {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BaseSafe => "base_safe",
            Self::ActiveSeedGate => "active_seed_gate",
            Self::InactiveSafe => "inactive_safe",
            Self::StructurallyBlocked => "structurally_blocked",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AffineSingularLaneRow {
    pub role: String,
    pub category: String,
    pub base: u32,
    pub pair_label: String,
    pub outer: u32,
    pub inner: u32,
    pub k_label: String,
    pub k_outer: u32,
    pub k_inner: u32,
    pub middle_length: usize,
    pub total_base_digits: usize,
    pub measurement_mode: String,
    pub sample_seed: u64,
    pub seed_capacity: u64,
    pub sample_count: usize,
    pub shift: u64,
    pub gradient: u64,
    pub prime_bound: u32,
    pub included_moduli_label: String,
    pub modulus_count: usize,
    pub base_safe_count: usize,
    pub active_seed_gate_count: usize,
    pub inactive_safe_count: usize,
    pub structurally_blocked_count: usize,
    pub survivor_count: usize,
    pub survivor_share: f64,
    pub observed_prime_count: usize,
    pub observed_prime_rate: f64,
    pub pnt_expected_density: f64,
    pub naive_survivor_expectation: f64,
    pub base_coprime_multiplier: f64,
    pub finite_residue_multiplier: f64,
    pub residue_adjusted_expected_density: f64,
    pub residual_vs_pnt_pp: f64,
    pub residual_vs_residue_expected_pp: f64,
    pub abs_residual_vs_residue_expected_pp: f64,
    pub residual_tag: String,
    pub first_witness_value: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffineSingularModulusRow {
    pub role: String,
    pub base: u32,
    pub pair_label: String,
    pub k_label: String,
    pub middle_length: usize,
    pub modulus: u32,
    pub classification: String,
    pub shift_modulus: u32,
    pub gradient_modulus: u32,
    pub excluded_seed_classes: String,
    pub excluded_seed_count: usize,
    pub survivor_count: usize,
    pub survivor_share: f64,
    pub random_survivor_expectation: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffineSingularRankRow {
    pub selection_reason: String,
    pub rank: usize,
    pub role: String,
    pub category: String,
    pub base: u32,
    pub pair_label: String,
    pub k_label: String,
    pub middle_length: usize,
    pub observed_prime_rate: f64,
    pub residue_adjusted_expected_density: f64,
    pub residual_vs_residue_expected_pp: f64,
    pub finite_residue_multiplier: f64,
    pub survivor_share: f64,
    pub observed_prime_count: usize,
    pub first_witness_value: String,
    pub residual_tag: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffineSingularWitnessRow {
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
pub struct AffineSingularSeriesSummary {
    pub lane_count: usize,
    pub modulus_row_count: usize,
    pub witness_count: usize,
    pub strongest_positive_role: String,
    pub strongest_positive_residual_pp: f64,
    pub strongest_absolute_role: String,
    pub strongest_absolute_residual_pp: f64,
    pub structurally_blocked_lane_count: usize,
    pub strong_line: String,
    pub caution_line: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffineSingularSeriesReport {
    pub settings: AffineSingularSeriesSettings,
    pub summary: AffineSingularSeriesSummary,
    pub lane_rows: Vec<AffineSingularLaneRow>,
    pub modulus_rows: Vec<AffineSingularModulusRow>,
    pub residual_rank_rows: Vec<AffineSingularRankRow>,
    pub witness_rows: Vec<AffineSingularWitnessRow>,
}

#[derive(Debug, Clone)]
struct SeedSample {
    seeds: Vec<u64>,
    measurement_mode: &'static str,
}

#[derive(Debug, Clone)]
struct LaneMeasurement {
    lane_row: AffineSingularLaneRow,
    modulus_rows: Vec<AffineSingularModulusRow>,
    witness_rows: Vec<AffineSingularWitnessRow>,
}

pub fn default_affine_singular_lane_specs() -> Vec<AffineSingularLaneSpec> {
    let mut specs = DEFAULT_CONSTRUCTION_DENSITY_SPECS
        .iter()
        .map(|spec| AffineSingularLaneSpec {
            role: spec.role,
            category: spec.category,
            base: spec.base,
            outer: spec.outer,
            inner: spec.inner,
            middle_length: spec.middle_length,
            k_outer: spec.k_outer,
            k_inner: spec.k_inner,
            note: spec.note,
        })
        .collect::<Vec<_>>();

    specs.extend(
        DEFAULT_SHIFT_PHASE_FOCUS_SPECS
            .iter()
            .map(|spec| AffineSingularLaneSpec {
                role: spec.track_name,
                category: spec.track_kind,
                base: spec.base,
                outer: spec.low_digit,
                inner: spec.high_digit,
                middle_length: spec.source_middle_length,
                k_outer: 0,
                k_inner: 0,
                note: spec.note,
            }),
    );

    specs
}

pub fn build_affine_singular_series_report(
    settings: AffineSingularSeriesSettings,
) -> AffineSingularSeriesReport {
    let specs = default_affine_singular_lane_specs();
    let mut lane_rows = Vec::new();
    let mut modulus_rows = Vec::new();
    let mut witness_rows = Vec::new();

    for spec in specs {
        let measurement = measure_lane(spec, settings);
        modulus_rows.extend(measurement.modulus_rows);
        witness_rows.extend(measurement.witness_rows);
        lane_rows.push(measurement.lane_row);
    }

    let residual_rank_rows = build_rank_rows(&lane_rows, settings.top_limit);
    let summary = build_summary(&lane_rows, &modulus_rows, &witness_rows);

    AffineSingularSeriesReport {
        settings,
        summary,
        lane_rows,
        modulus_rows,
        residual_rank_rows,
        witness_rows,
    }
}

pub fn classify_modulus(lane: &FastAffineLane, modulus: u32) -> ModulusClassification {
    let shift_mod = (lane.shift % modulus as u64) as u32;
    let gradient_mod = (lane.gradient % modulus as u64) as u32;

    if lane.config.base.is_multiple_of(modulus) {
        if shift_mod == 0 {
            ModulusClassification::StructurallyBlocked
        } else {
            ModulusClassification::BaseSafe
        }
    } else if gradient_mod == 0 {
        if shift_mod == 0 {
            ModulusClassification::StructurallyBlocked
        } else {
            ModulusClassification::InactiveSafe
        }
    } else {
        ModulusClassification::ActiveSeedGate
    }
}

pub fn excluded_seed_class(lane: &FastAffineLane, modulus: u32) -> Option<u32> {
    if classify_modulus(lane, modulus) != ModulusClassification::ActiveSeedGate {
        return None;
    }
    let shift = (lane.shift % modulus as u64) as i64;
    let gradient = (lane.gradient % modulus as u64) as i64;
    let inverse = modular_inverse(gradient, modulus as i64)?;
    let excluded = (-shift * inverse).rem_euclid(modulus as i64);
    Some(excluded as u32)
}

fn measure_lane(
    spec: AffineSingularLaneSpec,
    settings: AffineSingularSeriesSettings,
) -> LaneMeasurement {
    let lane = build_fast_affine_lane(spec.fast_config()).unwrap_or_else(|err| {
        panic!("default affine singular lane should fit u64: {spec:?}: {err}")
    });
    let primes = primes_up_to(settings.prime_bound);
    let sample = seed_sample(&lane, settings);
    let mut modulus_rows = Vec::new();

    for &modulus in &primes {
        modulus_rows.push(measure_modulus_row(spec, &lane, modulus, &sample.seeds));
    }

    let mut survivor_count = 0usize;
    let mut observed_prime_count = 0usize;
    let mut sum_pnt_density = 0.0;
    let mut witness_rows = Vec::new();
    let mut first_witness_value = String::new();

    for &seed in &sample.seeds {
        let value = lane
            .candidate_value(seed)
            .expect("sample seed should be inside lane capacity");
        let survives_residue_gates = seed_survives_moduli(&lane, seed, &primes);
        if survives_residue_gates {
            survivor_count += 1;
        }
        if value > 1 {
            sum_pnt_density += 1.0 / (value as f64).ln();
        }
        if (survives_residue_gates || value <= settings.prime_bound as u64)
            && is_probable_prime_u128(value as u128)
        {
            observed_prime_count += 1;
            if first_witness_value.is_empty() {
                first_witness_value = value.to_string();
            }
            if witness_rows.len() < settings.max_witnesses {
                witness_rows.push(AffineSingularWitnessRow {
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

    let sample_count = sample.seeds.len();
    let survivor_share = rate(survivor_count, sample_count);
    let observed_prime_rate = rate(observed_prime_count, sample_count);
    let pnt_expected_density = if sample_count == 0 {
        0.0
    } else {
        sum_pnt_density / sample_count as f64
    };
    let naive_survivor_expectation = primes
        .iter()
        .fold(1.0, |acc, &p| acc * (1.0 - 1.0 / p as f64));
    let finite_residue_multiplier = if naive_survivor_expectation > 0.0 {
        survivor_share / naive_survivor_expectation
    } else {
        0.0
    };
    let residue_adjusted_expected_density =
        (pnt_expected_density * finite_residue_multiplier).min(1.0);
    let base_coprime_multiplier = modulus_rows
        .iter()
        .filter(|row| row.classification == ModulusClassification::BaseSafe.as_str())
        .fold(1.0, |acc, row| acc / (1.0 - 1.0 / row.modulus as f64));
    let structurally_blocked_count = modulus_rows
        .iter()
        .filter(|row| row.classification == ModulusClassification::StructurallyBlocked.as_str())
        .count();
    let residual_vs_pnt_pp = (observed_prime_rate - pnt_expected_density) * 100.0;
    let residual_vs_residue_expected_pp =
        (observed_prime_rate - residue_adjusted_expected_density) * 100.0;

    let lane_row = AffineSingularLaneRow {
        role: spec.role.to_string(),
        category: spec.category.to_string(),
        base: spec.base,
        pair_label: spec.pair_label(),
        outer: spec.outer,
        inner: spec.inner,
        k_label: spec.k_label(),
        k_outer: spec.k_outer,
        k_inner: spec.k_inner,
        middle_length: spec.middle_length,
        total_base_digits: spec.middle_length + 4 + 2 * (spec.k_outer + spec.k_inner) as usize,
        measurement_mode: sample.measurement_mode.to_string(),
        sample_seed: settings.sample_seed,
        seed_capacity: lane.seed_capacity,
        sample_count,
        shift: lane.shift,
        gradient: lane.gradient,
        prime_bound: settings.prime_bound,
        included_moduli_label: primes
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("|"),
        modulus_count: primes.len(),
        base_safe_count: modulus_rows
            .iter()
            .filter(|row| row.classification == ModulusClassification::BaseSafe.as_str())
            .count(),
        active_seed_gate_count: modulus_rows
            .iter()
            .filter(|row| row.classification == ModulusClassification::ActiveSeedGate.as_str())
            .count(),
        inactive_safe_count: modulus_rows
            .iter()
            .filter(|row| row.classification == ModulusClassification::InactiveSafe.as_str())
            .count(),
        structurally_blocked_count,
        survivor_count,
        survivor_share,
        observed_prime_count,
        observed_prime_rate,
        pnt_expected_density,
        naive_survivor_expectation,
        base_coprime_multiplier,
        finite_residue_multiplier,
        residue_adjusted_expected_density,
        residual_vs_pnt_pp,
        residual_vs_residue_expected_pp,
        abs_residual_vs_residue_expected_pp: residual_vs_residue_expected_pp.abs(),
        residual_tag: residual_tag(residual_vs_residue_expected_pp, structurally_blocked_count),
        first_witness_value,
        note: spec.note.to_string(),
    };

    LaneMeasurement {
        lane_row,
        modulus_rows,
        witness_rows,
    }
}

fn measure_modulus_row(
    spec: AffineSingularLaneSpec,
    lane: &FastAffineLane,
    modulus: u32,
    seeds: &[u64],
) -> AffineSingularModulusRow {
    let classification = classify_modulus(lane, modulus);
    let shift_modulus = (lane.shift % modulus as u64) as u32;
    let gradient_modulus = (lane.gradient % modulus as u64) as u32;
    let mut excluded_seed_count = 0usize;
    let mut survivor_count = 0usize;
    for &seed in seeds {
        let divisible = seed_divisible_by_modulus(lane, seed, modulus);
        if divisible {
            excluded_seed_count += 1;
        } else {
            survivor_count += 1;
        }
    }

    AffineSingularModulusRow {
        role: spec.role.to_string(),
        base: spec.base,
        pair_label: spec.pair_label(),
        k_label: spec.k_label(),
        middle_length: spec.middle_length,
        modulus,
        classification: classification.as_str().to_string(),
        shift_modulus,
        gradient_modulus,
        excluded_seed_classes: excluded_seed_classes_label(lane, modulus, classification),
        excluded_seed_count,
        survivor_count,
        survivor_share: rate(survivor_count, seeds.len()),
        random_survivor_expectation: 1.0 - 1.0 / modulus as f64,
    }
}

fn build_rank_rows(rows: &[AffineSingularLaneRow], top_limit: usize) -> Vec<AffineSingularRankRow> {
    let mut rank_rows = Vec::new();
    let mut positive = rows
        .iter()
        .filter(|row| row.residual_vs_residue_expected_pp > 0.0)
        .collect::<Vec<_>>();
    positive.sort_by(|left, right| {
        right
            .residual_vs_residue_expected_pp
            .total_cmp(&left.residual_vs_residue_expected_pp)
    });
    for (index, row) in positive.into_iter().take(top_limit).enumerate() {
        rank_rows.push(rank_row("positive_residual", index + 1, row));
    }

    let mut absolute = rows.iter().collect::<Vec<_>>();
    absolute.sort_by(|left, right| {
        right
            .abs_residual_vs_residue_expected_pp
            .total_cmp(&left.abs_residual_vs_residue_expected_pp)
    });
    for (index, row) in absolute.into_iter().take(top_limit).enumerate() {
        rank_rows.push(rank_row("absolute_mismatch", index + 1, row));
    }

    rank_rows
}

fn rank_row(
    selection_reason: &str,
    rank: usize,
    row: &AffineSingularLaneRow,
) -> AffineSingularRankRow {
    AffineSingularRankRow {
        selection_reason: selection_reason.to_string(),
        rank,
        role: row.role.clone(),
        category: row.category.clone(),
        base: row.base,
        pair_label: row.pair_label.clone(),
        k_label: row.k_label.clone(),
        middle_length: row.middle_length,
        observed_prime_rate: row.observed_prime_rate,
        residue_adjusted_expected_density: row.residue_adjusted_expected_density,
        residual_vs_residue_expected_pp: row.residual_vs_residue_expected_pp,
        finite_residue_multiplier: row.finite_residue_multiplier,
        survivor_share: row.survivor_share,
        observed_prime_count: row.observed_prime_count,
        first_witness_value: row.first_witness_value.clone(),
        residual_tag: row.residual_tag.clone(),
    }
}

fn build_summary(
    lane_rows: &[AffineSingularLaneRow],
    modulus_rows: &[AffineSingularModulusRow],
    witness_rows: &[AffineSingularWitnessRow],
) -> AffineSingularSeriesSummary {
    let strongest_positive = lane_rows.iter().max_by(|left, right| {
        left.residual_vs_residue_expected_pp
            .total_cmp(&right.residual_vs_residue_expected_pp)
    });
    let strongest_absolute = lane_rows.iter().max_by(|left, right| {
        left.abs_residual_vs_residue_expected_pp
            .total_cmp(&right.abs_residual_vs_residue_expected_pp)
    });

    AffineSingularSeriesSummary {
        lane_count: lane_rows.len(),
        modulus_row_count: modulus_rows.len(),
        witness_count: witness_rows.len(),
        strongest_positive_role: strongest_positive
            .map(|row| row.role.clone())
            .unwrap_or_default(),
        strongest_positive_residual_pp: strongest_positive
            .map(|row| row.residual_vs_residue_expected_pp)
            .unwrap_or_default(),
        strongest_absolute_role: strongest_absolute
            .map(|row| row.role.clone())
            .unwrap_or_default(),
        strongest_absolute_residual_pp: strongest_absolute
            .map(|row| row.residual_vs_residue_expected_pp)
            .unwrap_or_default(),
        structurally_blocked_lane_count: lane_rows
            .iter()
            .filter(|row| row.structurally_blocked_count > 0)
            .count(),
        strong_line: "Finite affine singular profiles estimate how much lane yield is explained by exact small-prime residue weather.".to_string(),
        caution_line: "A positive residual is a ranked lead for follow-up, not an asymptotic density theorem.".to_string(),
    }
}

fn seed_sample(lane: &FastAffineLane, settings: AffineSingularSeriesSettings) -> SeedSample {
    if lane.seed_capacity < settings.exact_seed_cap
        || lane.seed_capacity <= settings.sample_count as u64
    {
        return SeedSample {
            seeds: (0..lane.seed_capacity).collect(),
            measurement_mode: "exact",
        };
    }

    let target = settings.sample_count.min(lane.seed_capacity as usize);
    let mut state = settings.sample_seed
        ^ lane.shift
        ^ lane.gradient.rotate_left(17)
        ^ ((lane.config.base as u64) << 48);
    let mut seeds = BTreeSet::new();
    while seeds.len() < target {
        state = xorshift64(state);
        seeds.insert(state % lane.seed_capacity);
    }
    SeedSample {
        seeds: seeds.into_iter().collect(),
        measurement_mode: "sampled",
    }
}

fn seed_survives_moduli(lane: &FastAffineLane, seed: u64, moduli: &[u32]) -> bool {
    moduli
        .iter()
        .copied()
        .all(|modulus| !seed_divisible_by_modulus(lane, seed, modulus))
}

fn seed_divisible_by_modulus(lane: &FastAffineLane, seed: u64, modulus: u32) -> bool {
    ((lane.shift % modulus as u64) + (lane.gradient % modulus as u64) * (seed % modulus as u64))
        .is_multiple_of(modulus as u64)
}

fn excluded_seed_classes_label(
    lane: &FastAffineLane,
    modulus: u32,
    classification: ModulusClassification,
) -> String {
    match classification {
        ModulusClassification::BaseSafe | ModulusClassification::InactiveSafe => "none".to_string(),
        ModulusClassification::StructurallyBlocked => "all".to_string(),
        ModulusClassification::ActiveSeedGate => excluded_seed_class(lane, modulus)
            .map(|class| class.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
    }
}

fn residual_tag(residual_pp: f64, structurally_blocked_count: usize) -> String {
    if structurally_blocked_count > 0 {
        "structurally_blocked".to_string()
    } else if residual_pp >= 1.0 {
        "positive_lead".to_string()
    } else if residual_pp <= -1.0 {
        "negative_foil".to_string()
    } else {
        "near_expected".to_string()
    }
}

fn primes_up_to(bound: u32) -> Vec<u32> {
    (2..=bound)
        .filter(|&candidate| is_prime_u32(candidate))
        .collect()
}

fn is_prime_u32(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }
    let mut divisor = 3;
    while divisor * divisor <= n {
        if n.is_multiple_of(divisor) {
            return false;
        }
        divisor += 2;
    }
    true
}

fn modular_inverse(value: i64, modulus: i64) -> Option<i64> {
    let mut t = 0;
    let mut new_t = 1;
    let mut r = modulus;
    let mut new_r = value.rem_euclid(modulus);
    while new_r != 0 {
        let quotient = r / new_r;
        (t, new_t) = (new_t, t - quotient * new_t);
        (r, new_r) = (new_r, r - quotient * new_r);
    }
    if r > 1 {
        return None;
    }
    Some(t.rem_euclid(modulus))
}

fn xorshift64(mut value: u64) -> u64 {
    if value == 0 {
        value = 0x9E37_79B9_7F4A_7C15;
    }
    value ^= value << 13;
    value ^= value >> 7;
    value ^= value << 17;
    value
}

fn rate(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tiny_settings() -> AffineSingularSeriesSettings {
        AffineSingularSeriesSettings {
            prime_bound: 31,
            exact_seed_cap: 10_000,
            sample_count: 200,
            sample_seed: DEFAULT_SINGULAR_SAMPLE_SEED,
            max_witnesses: 3,
            top_limit: 5,
        }
    }

    fn lane(
        base: u32,
        outer: u32,
        inner: u32,
        middle_length: usize,
        k: (u32, u32),
    ) -> FastAffineLane {
        build_fast_affine_lane(FastLaneConfig::new(base, outer, inner, middle_length, k))
            .expect("lane should build")
    }

    #[test]
    fn modulus_classification_handles_base_safe_active_and_blocked_cases() {
        let decimal_lane = lane(10, 3, 7, 2, (2, 1));
        assert_eq!(
            classify_modulus(&decimal_lane, 2),
            ModulusClassification::BaseSafe
        );
        assert_eq!(
            classify_modulus(&decimal_lane, 5),
            ModulusClassification::BaseSafe
        );

        let base30_lane = lane(30, 11, 7, 2, (0, 0));
        assert_eq!(
            classify_modulus(&base30_lane, 7),
            ModulusClassification::ActiveSeedGate
        );
        let excluded = excluded_seed_class(&base30_lane, 7).expect("active gate");
        for seed in 0..100 {
            assert_eq!(
                seed % 7 == excluded as u64,
                seed_divisible_by_modulus(&base30_lane, seed, 7)
            );
        }

        let blocked_lane = lane(10, 2, 7, 2, (0, 0));
        assert_eq!(
            classify_modulus(&blocked_lane, 2),
            ModulusClassification::StructurallyBlocked
        );
    }

    #[test]
    fn prediction_rows_are_deterministic_and_consistent_for_known_lanes() {
        let report = build_affine_singular_series_report(tiny_settings());
        for role in [
            "base6_champion_compact",
            "base10_visible_zero_run_k21",
            "base30_wheel_like_compact",
        ] {
            let row = report
                .lane_rows
                .iter()
                .find(|row| row.role == role)
                .expect("known row");
            assert!(row.finite_residue_multiplier.is_finite());
            assert!(row.finite_residue_multiplier > 0.0);
            assert!(row.residue_adjusted_expected_density >= 0.0);
            assert!(row.residue_adjusted_expected_density <= 1.0);
            let expected =
                (row.observed_prime_rate - row.residue_adjusted_expected_density) * 100.0;
            assert!((row.residual_vs_residue_expected_pp - expected).abs() < 1e-12);
        }
    }

    #[test]
    fn report_rows_have_prime_witnesses_and_required_outputs() {
        let report = build_affine_singular_series_report(tiny_settings());
        assert!(!report.residual_rank_rows.is_empty());
        assert!(report
            .witness_rows
            .iter()
            .all(|row| primal::is_prime(row.decimal_value)));
        for required in [
            "report.md",
            "summary.json",
            "lane_rows.csv",
            "modulus_rows.csv",
            "residual_rank_rows.csv",
            "witness_rows.csv",
            "residual_ranking.png",
            "multiplier_decomposition.png",
            "modulus_gate_heatmap.png",
            "lead_gallery.png",
        ] {
            assert!(AFFINE_SINGULAR_SERIES_EXPECTED_OUTPUTS.contains(&required));
        }
    }
}
