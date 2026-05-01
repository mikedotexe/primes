//! Matched-control report helpers for maintained membrane families.
//!
//! This module supports the empirical "Gate A" surface:
//! compare documented membrane families against random decimal controls matched
//! on exact decimal digit count and coprimality to the same base, then summarize
//! the results with effect sizes, confidence intervals, and BH-adjusted
//! multiplicity decisions.

use crate::{
    hzlib::{benjamini_hochberg, hedges_g},
    is_prime,
};
use chrono::{SecondsFormat, Utc};
use num_bigint::BigUint;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use statrs::distribution::{ContinuousCDF, Normal};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};
use thiserror::Error;

pub const DEFAULT_SAMPLES: usize = 100;
pub const DEFAULT_MIN_SEED_LEN: usize = 1;
pub const DEFAULT_MAX_SEED_LEN: usize = 2;
pub const DEFAULT_FDR: f64 = 0.05;
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;
pub const MATCHED_CONTROL_EXPORT_VERSION: u32 = 1;
pub const MATCHED_CONTROL_COMPARISON_EXPORT_VERSION: u32 = 1;
pub const DEFAULT_COMPARE_LIFT_THRESHOLD: f64 = 0.25;
pub const DEFAULT_COMPARE_Q_THRESHOLD: f64 = 0.10;
pub const MATCHED_CONTROL_COMPARE_FLAG_EXIT_CODE: i32 = 3;

pub const MAINTAINED_MATCHED_CONTROL_FAMILIES: [MatchedControlFamily; 8] = [
    MatchedControlFamily {
        label: "Base 6 champion",
        base: 6,
        outer: 1,
        inner: 5,
        k_outer: 0,
        k_inner: 0,
    },
    MatchedControlFamily {
        label: "Base 10 classic",
        base: 10,
        outer: 3,
        inner: 7,
        k_outer: 0,
        k_inner: 0,
    },
    MatchedControlFamily {
        label: "Base 10 breathing",
        base: 10,
        outer: 3,
        inner: 3,
        k_outer: 0,
        k_inner: 1,
    },
    MatchedControlFamily {
        label: "Base 10 symmetric",
        base: 10,
        outer: 3,
        inner: 3,
        k_outer: 1,
        k_inner: 1,
    },
    MatchedControlFamily {
        label: "Base 10 exclusive",
        base: 10,
        outer: 3,
        inner: 7,
        k_outer: 1,
        k_inner: 1,
    },
    MatchedControlFamily {
        label: "Base 12 compact",
        base: 12,
        outer: 1,
        inner: 1,
        k_outer: 0,
        k_inner: 0,
    },
    MatchedControlFamily {
        label: "Base 14 offset",
        base: 14,
        outer: 1,
        inner: 3,
        k_outer: 0,
        k_inner: 0,
    },
    MatchedControlFamily {
        label: "Base 30 wheel-like",
        base: 30,
        outer: 11,
        inner: 7,
        k_outer: 0,
        k_inner: 0,
    },
];

#[derive(Debug, Clone, Copy)]
pub struct MatchedControlFamily {
    pub label: &'static str,
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub k_outer: u32,
    pub k_inner: u32,
}

impl MatchedControlFamily {
    pub fn code(&self, seed_len: usize) -> String {
        format!(
            "B{:>2} ({:>2},{:>2}) k=({},{}) M={}",
            self.base, self.outer, self.inner, self.k_outer, self.k_inner, seed_len
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MatchedControlRunSettings {
    pub samples: usize,
    pub min_seed_len: usize,
    pub max_seed_len: usize,
    pub fdr: f64,
    pub confidence_level: f64,
}

impl Default for MatchedControlRunSettings {
    fn default() -> Self {
        Self {
            samples: DEFAULT_SAMPLES,
            min_seed_len: DEFAULT_MIN_SEED_LEN,
            max_seed_len: DEFAULT_MAX_SEED_LEN,
            fdr: DEFAULT_FDR,
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MatchedControlArmStats {
    pub primes: usize,
    pub samples: usize,
    pub rate: f64,
    pub ci: (f64, f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchedControlDecision {
    PositiveQ,
    NegativeQ,
    PositiveRaw,
    NegativeRaw,
    NotSignificant,
}

impl MatchedControlDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PositiveQ => "positive-q",
            Self::NegativeQ => "negative-q",
            Self::PositiveRaw => "positive-raw",
            Self::NegativeRaw => "negative-raw",
            Self::NotSignificant => "ns",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MatchedControlReport {
    pub family: MatchedControlFamily,
    pub seed_len: usize,
    pub mean_digits: f64,
    pub membrane: MatchedControlArmStats,
    pub control: MatchedControlArmStats,
    pub diff: f64,
    pub diff_ci: (f64, f64),
    pub lift: f64,
    pub lift_ci: (f64, f64),
    pub hedges_g: f64,
    pub p_value: f64,
    pub q_value: f64,
    pub decision: MatchedControlDecision,
}

#[derive(Debug, Clone, Copy)]
pub struct MatchedControlBaseSummary {
    pub base: u32,
    pub families: usize,
    pub membrane: MatchedControlArmStats,
    pub control: MatchedControlArmStats,
    pub lift: f64,
    pub lift_ci: (f64, f64),
    pub positive_q_families: usize,
}

#[derive(Debug, Clone)]
pub struct MatchedControlSummary {
    pub total_families: usize,
    pub positive_q: usize,
    pub negative_q: usize,
    pub positive_raw: usize,
    pub negative_raw: usize,
    pub positive_q_bases: Vec<u32>,
    pub pooled_membrane: MatchedControlArmStats,
    pub pooled_control: MatchedControlArmStats,
    pub pooled_lift: f64,
    pub pooled_lift_ci: (f64, f64),
    pub residual_criterion_met: bool,
    pub base_summaries: Vec<MatchedControlBaseSummary>,
}

#[derive(Debug, Error)]
pub enum MatchedControlExportError {
    #[error("I/O error while exporting matched-control report: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON serialization failed for matched-control report: {0}")]
    Json(#[from] serde_json::Error),
    #[error("CSV serialization failed for matched-control report: {0}")]
    Csv(#[from] csv::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedControlExportRow {
    pub family_label: String,
    pub family_code: String,
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub k_outer: u32,
    pub k_inner: u32,
    pub seed_len: usize,
    pub mean_digits: f64,
    pub samples_per_arm: usize,
    pub confidence_level: f64,
    pub fdr_threshold: f64,
    pub membrane_primes: usize,
    pub membrane_rate: f64,
    pub membrane_ci_lo: f64,
    pub membrane_ci_hi: f64,
    pub control_primes: usize,
    pub control_rate: f64,
    pub control_ci_lo: f64,
    pub control_ci_hi: f64,
    pub diff: f64,
    pub diff_ci_lo: f64,
    pub diff_ci_hi: f64,
    pub lift: Option<f64>,
    pub lift_ci_lo: Option<f64>,
    pub lift_ci_hi: Option<f64>,
    pub hedges_g: Option<f64>,
    pub p_value: Option<f64>,
    pub q_value: Option<f64>,
    pub decision: String,
}

impl MatchedControlExportRow {
    fn from_report(report: &MatchedControlReport, settings: MatchedControlRunSettings) -> Self {
        Self {
            family_label: report.family.label.to_string(),
            family_code: report.family.code(report.seed_len),
            base: report.family.base,
            outer: report.family.outer,
            inner: report.family.inner,
            k_outer: report.family.k_outer,
            k_inner: report.family.k_inner,
            seed_len: report.seed_len,
            mean_digits: report.mean_digits,
            samples_per_arm: report.membrane.samples,
            confidence_level: settings.confidence_level,
            fdr_threshold: settings.fdr,
            membrane_primes: report.membrane.primes,
            membrane_rate: report.membrane.rate,
            membrane_ci_lo: report.membrane.ci.0,
            membrane_ci_hi: report.membrane.ci.1,
            control_primes: report.control.primes,
            control_rate: report.control.rate,
            control_ci_lo: report.control.ci.0,
            control_ci_hi: report.control.ci.1,
            diff: report.diff,
            diff_ci_lo: report.diff_ci.0,
            diff_ci_hi: report.diff_ci.1,
            lift: finite_or_none(report.lift),
            lift_ci_lo: finite_or_none(report.lift_ci.0),
            lift_ci_hi: finite_or_none(report.lift_ci.1),
            hedges_g: finite_or_none(report.hedges_g),
            p_value: finite_or_none(report.p_value),
            q_value: finite_or_none(report.q_value),
            decision: report.decision.as_str().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedControlExportBaseSummary {
    pub base: u32,
    pub families: usize,
    pub membrane_primes: usize,
    pub membrane_samples: usize,
    pub membrane_rate: f64,
    pub membrane_ci_lo: f64,
    pub membrane_ci_hi: f64,
    pub control_primes: usize,
    pub control_samples: usize,
    pub control_rate: f64,
    pub control_ci_lo: f64,
    pub control_ci_hi: f64,
    pub lift: Option<f64>,
    pub lift_ci_lo: Option<f64>,
    pub lift_ci_hi: Option<f64>,
    pub positive_q_families: usize,
}

impl From<&MatchedControlBaseSummary> for MatchedControlExportBaseSummary {
    fn from(summary: &MatchedControlBaseSummary) -> Self {
        Self {
            base: summary.base,
            families: summary.families,
            membrane_primes: summary.membrane.primes,
            membrane_samples: summary.membrane.samples,
            membrane_rate: summary.membrane.rate,
            membrane_ci_lo: summary.membrane.ci.0,
            membrane_ci_hi: summary.membrane.ci.1,
            control_primes: summary.control.primes,
            control_samples: summary.control.samples,
            control_rate: summary.control.rate,
            control_ci_lo: summary.control.ci.0,
            control_ci_hi: summary.control.ci.1,
            lift: finite_or_none(summary.lift),
            lift_ci_lo: finite_or_none(summary.lift_ci.0),
            lift_ci_hi: finite_or_none(summary.lift_ci.1),
            positive_q_families: summary.positive_q_families,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedControlExportSummary {
    pub total_families: usize,
    pub positive_q: usize,
    pub negative_q: usize,
    pub positive_raw: usize,
    pub negative_raw: usize,
    pub positive_q_bases: Vec<u32>,
    pub pooled_membrane_primes: usize,
    pub pooled_membrane_samples: usize,
    pub pooled_membrane_rate: f64,
    pub pooled_membrane_ci_lo: f64,
    pub pooled_membrane_ci_hi: f64,
    pub pooled_control_primes: usize,
    pub pooled_control_samples: usize,
    pub pooled_control_rate: f64,
    pub pooled_control_ci_lo: f64,
    pub pooled_control_ci_hi: f64,
    pub pooled_lift: Option<f64>,
    pub pooled_lift_ci_lo: Option<f64>,
    pub pooled_lift_ci_hi: Option<f64>,
    pub residual_criterion_met: bool,
    pub base_summaries: Vec<MatchedControlExportBaseSummary>,
}

impl From<&MatchedControlSummary> for MatchedControlExportSummary {
    fn from(summary: &MatchedControlSummary) -> Self {
        Self {
            total_families: summary.total_families,
            positive_q: summary.positive_q,
            negative_q: summary.negative_q,
            positive_raw: summary.positive_raw,
            negative_raw: summary.negative_raw,
            positive_q_bases: summary.positive_q_bases.clone(),
            pooled_membrane_primes: summary.pooled_membrane.primes,
            pooled_membrane_samples: summary.pooled_membrane.samples,
            pooled_membrane_rate: summary.pooled_membrane.rate,
            pooled_membrane_ci_lo: summary.pooled_membrane.ci.0,
            pooled_membrane_ci_hi: summary.pooled_membrane.ci.1,
            pooled_control_primes: summary.pooled_control.primes,
            pooled_control_samples: summary.pooled_control.samples,
            pooled_control_rate: summary.pooled_control.rate,
            pooled_control_ci_lo: summary.pooled_control.ci.0,
            pooled_control_ci_hi: summary.pooled_control.ci.1,
            pooled_lift: finite_or_none(summary.pooled_lift),
            pooled_lift_ci_lo: finite_or_none(summary.pooled_lift_ci.0),
            pooled_lift_ci_hi: finite_or_none(summary.pooled_lift_ci.1),
            residual_criterion_met: summary.residual_criterion_met,
            base_summaries: summary
                .base_summaries
                .iter()
                .map(MatchedControlExportBaseSummary::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedControlExportBundle {
    pub export_version: u32,
    pub generated_at_utc: String,
    pub settings: MatchedControlRunSettings,
    pub reports: Vec<MatchedControlExportRow>,
    pub summary: MatchedControlExportSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MatchedControlCompareSettings {
    pub lift_threshold: f64,
    pub q_threshold: f64,
}

impl Default for MatchedControlCompareSettings {
    fn default() -> Self {
        Self {
            lift_threshold: DEFAULT_COMPARE_LIFT_THRESHOLD,
            q_threshold: DEFAULT_COMPARE_Q_THRESHOLD,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchedControlComparePolicy {
    pub flag_sampling_plan_drift: bool,
    pub flag_added_families: bool,
    pub flag_removed_families: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchedControlFamilySnapshot {
    pub family_label: String,
    pub family_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedControlFamilyDelta {
    pub family_label: String,
    pub family_code: String,
    pub lift_before: Option<f64>,
    pub lift_after: Option<f64>,
    pub lift_delta: Option<f64>,
    pub q_before: Option<f64>,
    pub q_after: Option<f64>,
    pub q_delta: Option<f64>,
    pub decision_before: String,
    pub decision_after: String,
    pub material_lift_change: bool,
    pub material_q_change: bool,
    pub decision_changed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedControlComparison {
    pub before_generated_at_utc: String,
    pub after_generated_at_utc: String,
    pub before_export_version: u32,
    pub after_export_version: u32,
    pub families_compared: usize,
    pub materially_changed_families: Vec<MatchedControlFamilyDelta>,
    pub added_families: Vec<MatchedControlFamilySnapshot>,
    pub removed_families: Vec<MatchedControlFamilySnapshot>,
    pub residual_criterion_before: bool,
    pub residual_criterion_after: bool,
    pub residual_criterion_changed: bool,
    pub pooled_lift_before: Option<f64>,
    pub pooled_lift_after: Option<f64>,
    pub pooled_lift_delta: Option<f64>,
    pub positive_q_before: usize,
    pub positive_q_after: usize,
    pub negative_q_before: usize,
    pub negative_q_after: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedControlComparisonAudit {
    pub flagged: bool,
    pub sampling_plan_changed: bool,
    pub residual_criterion_changed: bool,
    pub material_family_change_count: usize,
    pub added_family_count: usize,
    pub removed_family_count: usize,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedControlComparisonExportBundle {
    pub export_version: u32,
    pub generated_at_utc: String,
    pub before_path: String,
    pub after_path: String,
    pub compare_settings: MatchedControlCompareSettings,
    pub compare_policy: MatchedControlComparePolicy,
    pub comparison: MatchedControlComparison,
    pub audit: MatchedControlComparisonAudit,
}

#[derive(Default)]
struct AggregateAccumulator {
    membrane_primes: usize,
    control_primes: usize,
    samples: usize,
    families: usize,
    positive_q: usize,
}

pub fn run_cross_family_report(
    families: &[MatchedControlFamily],
    settings: MatchedControlRunSettings,
) -> Vec<MatchedControlReport> {
    run_cross_family_report_with_progress(families, settings, false)
}

pub fn run_cross_family_report_with_progress(
    families: &[MatchedControlFamily],
    settings: MatchedControlRunSettings,
    progress: bool,
) -> Vec<MatchedControlReport> {
    let tasks = families
        .iter()
        .copied()
        .flat_map(|family| {
            (settings.min_seed_len..=settings.max_seed_len).map(move |seed_len| (family, seed_len))
        })
        .collect::<Vec<_>>();
    let completed = AtomicUsize::new(0);
    let total = tasks.len();
    let mut reports = tasks
        .par_iter()
        .map(|&(family, seed_len)| {
            let report = run_family_report(family, seed_len, settings);
            if progress {
                let done = completed.fetch_add(1, Ordering::Relaxed) + 1;
                eprintln!("[matched-control {done}/{total}] {}", family.code(seed_len));
            }
            report
        })
        .collect::<Vec<_>>();

    let adjusted = benjamini_hochberg(
        &reports
            .iter()
            .map(|report| report.p_value)
            .collect::<Vec<_>>(),
        settings.fdr,
    );

    for (report, q_value) in reports.iter_mut().zip(adjusted) {
        report.q_value = q_value;
        report.decision =
            classify_decision(report.diff, report.p_value, report.q_value, settings.fdr);
    }

    reports
}

pub fn summarize_reports(
    reports: &[MatchedControlReport],
    settings: MatchedControlRunSettings,
) -> MatchedControlSummary {
    let mut overall = AggregateAccumulator::default();
    let mut by_base: BTreeMap<u32, AggregateAccumulator> = BTreeMap::new();
    let mut positive_q_bases = BTreeSet::new();
    let mut positive_q = 0usize;
    let mut negative_q = 0usize;
    let mut positive_raw = 0usize;
    let mut negative_raw = 0usize;

    for report in reports {
        accumulate(&mut overall, report);
        accumulate(by_base.entry(report.family.base).or_default(), report);

        match report.decision {
            MatchedControlDecision::PositiveQ => {
                positive_q += 1;
                positive_q_bases.insert(report.family.base);
            }
            MatchedControlDecision::NegativeQ => negative_q += 1,
            MatchedControlDecision::PositiveRaw => positive_raw += 1,
            MatchedControlDecision::NegativeRaw => negative_raw += 1,
            MatchedControlDecision::NotSignificant => {}
        }
    }

    let pooled_membrane = arm_stats(
        overall.membrane_primes,
        overall.samples,
        settings.confidence_level,
    );
    let pooled_control = arm_stats(
        overall.control_primes,
        overall.samples,
        settings.confidence_level,
    );
    let pooled_lift = pooled_membrane.rate / pooled_control.rate;
    let pooled_lift_ci = rate_ratio_ci(
        overall.membrane_primes,
        overall.samples,
        overall.control_primes,
        overall.samples,
        settings.confidence_level,
    );

    let mut base_summaries = Vec::new();
    for (base, acc) in by_base {
        let membrane = arm_stats(acc.membrane_primes, acc.samples, settings.confidence_level);
        let control = arm_stats(acc.control_primes, acc.samples, settings.confidence_level);
        let lift = membrane.rate / control.rate;
        let lift_ci = rate_ratio_ci(
            acc.membrane_primes,
            acc.samples,
            acc.control_primes,
            acc.samples,
            settings.confidence_level,
        );

        base_summaries.push(MatchedControlBaseSummary {
            base,
            families: acc.families,
            membrane,
            control,
            lift,
            lift_ci,
            positive_q_families: acc.positive_q,
        });
    }

    let positive_q_base_count = positive_q_bases.len();
    let positive_q_bases_vec: Vec<u32> = positive_q_bases.into_iter().collect();

    MatchedControlSummary {
        total_families: reports.len(),
        positive_q,
        negative_q,
        positive_raw,
        negative_raw,
        positive_q_bases: positive_q_bases_vec,
        pooled_membrane,
        pooled_control,
        pooled_lift,
        pooled_lift_ci,
        residual_criterion_met: pooled_lift_ci.0 > 1.0
            && negative_q == 0
            && positive_q_base_count >= 2,
        base_summaries,
    }
}

pub fn read_json_export(
    path: impl AsRef<Path>,
) -> Result<MatchedControlExportBundle, MatchedControlExportError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

pub fn build_export_bundle(
    reports: &[MatchedControlReport],
    summary: &MatchedControlSummary,
    settings: MatchedControlRunSettings,
) -> MatchedControlExportBundle {
    MatchedControlExportBundle {
        export_version: MATCHED_CONTROL_EXPORT_VERSION,
        generated_at_utc: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
        settings,
        reports: reports
            .iter()
            .map(|report| MatchedControlExportRow::from_report(report, settings))
            .collect(),
        summary: MatchedControlExportSummary::from(summary),
    }
}

pub fn write_json_export(
    path: impl AsRef<Path>,
    reports: &[MatchedControlReport],
    summary: &MatchedControlSummary,
    settings: MatchedControlRunSettings,
) -> Result<(), MatchedControlExportError> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &build_export_bundle(reports, summary, settings))?;
    Ok(())
}

pub fn write_csv_export(
    path: impl AsRef<Path>,
    reports: &[MatchedControlReport],
    settings: MatchedControlRunSettings,
) -> Result<(), MatchedControlExportError> {
    let mut writer = csv::Writer::from_path(path)?;
    for row in reports
        .iter()
        .map(|report| MatchedControlExportRow::from_report(report, settings))
    {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub fn compare_export_bundles(
    before: &MatchedControlExportBundle,
    after: &MatchedControlExportBundle,
    settings: MatchedControlCompareSettings,
) -> MatchedControlComparison {
    let before_by_code: BTreeMap<&str, &MatchedControlExportRow> = before
        .reports
        .iter()
        .map(|report| (report.family_code.as_str(), report))
        .collect();
    let after_by_code: BTreeMap<&str, &MatchedControlExportRow> = after
        .reports
        .iter()
        .map(|report| (report.family_code.as_str(), report))
        .collect();

    let mut materially_changed_families = Vec::new();
    let mut added_families = Vec::new();
    let mut removed_families = Vec::new();
    let mut families_compared = 0usize;

    for (&family_code, before_row) in &before_by_code {
        match after_by_code.get(family_code) {
            Some(after_row) => {
                families_compared += 1;

                let lift_delta = option_delta(before_row.lift, after_row.lift);
                let q_delta = option_delta(before_row.q_value, after_row.q_value);
                let material_lift_change =
                    lift_delta.is_some_and(|delta| delta.abs() >= settings.lift_threshold);
                let material_q_change =
                    q_delta.is_some_and(|delta| delta.abs() >= settings.q_threshold);
                let decision_changed = before_row.decision != after_row.decision;

                if material_lift_change || material_q_change || decision_changed {
                    materially_changed_families.push(MatchedControlFamilyDelta {
                        family_label: after_row.family_label.clone(),
                        family_code: after_row.family_code.clone(),
                        lift_before: before_row.lift,
                        lift_after: after_row.lift,
                        lift_delta,
                        q_before: before_row.q_value,
                        q_after: after_row.q_value,
                        q_delta,
                        decision_before: before_row.decision.clone(),
                        decision_after: after_row.decision.clone(),
                        material_lift_change,
                        material_q_change,
                        decision_changed,
                    });
                }
            }
            None => removed_families.push(snapshot_from_row(before_row)),
        }
    }

    for (&family_code, after_row) in &after_by_code {
        if !before_by_code.contains_key(family_code) {
            added_families.push(snapshot_from_row(after_row));
        }
    }

    MatchedControlComparison {
        before_generated_at_utc: before.generated_at_utc.clone(),
        after_generated_at_utc: after.generated_at_utc.clone(),
        before_export_version: before.export_version,
        after_export_version: after.export_version,
        families_compared,
        materially_changed_families,
        added_families,
        removed_families,
        residual_criterion_before: before.summary.residual_criterion_met,
        residual_criterion_after: after.summary.residual_criterion_met,
        residual_criterion_changed: before.summary.residual_criterion_met
            != after.summary.residual_criterion_met,
        pooled_lift_before: before.summary.pooled_lift,
        pooled_lift_after: after.summary.pooled_lift,
        pooled_lift_delta: option_delta(before.summary.pooled_lift, after.summary.pooled_lift),
        positive_q_before: before.summary.positive_q,
        positive_q_after: after.summary.positive_q,
        negative_q_before: before.summary.negative_q,
        negative_q_after: after.summary.negative_q,
    }
}

pub fn summarize_comparison_audit(
    before: &MatchedControlExportBundle,
    after: &MatchedControlExportBundle,
    comparison: &MatchedControlComparison,
    policy: MatchedControlComparePolicy,
) -> MatchedControlComparisonAudit {
    let sampling_plan_changed = before.settings != after.settings;
    let residual_criterion_changed = comparison.residual_criterion_changed;
    let material_family_change_count = comparison.materially_changed_families.len();
    let added_family_count = comparison.added_families.len();
    let removed_family_count = comparison.removed_families.len();
    let mut reasons = Vec::new();

    if residual_criterion_changed {
        reasons.push("residual-criterion-changed".to_string());
    }
    if material_family_change_count > 0 {
        reasons.push("material-family-change".to_string());
    }
    if policy.flag_sampling_plan_drift && sampling_plan_changed {
        reasons.push("sampling-plan-drift".to_string());
    }
    if policy.flag_added_families && added_family_count > 0 {
        reasons.push("added-families".to_string());
    }
    if policy.flag_removed_families && removed_family_count > 0 {
        reasons.push("removed-families".to_string());
    }

    MatchedControlComparisonAudit {
        flagged: !reasons.is_empty(),
        sampling_plan_changed,
        residual_criterion_changed,
        material_family_change_count,
        added_family_count,
        removed_family_count,
        reasons,
    }
}

pub fn build_comparison_export_bundle(
    before_path: impl AsRef<Path>,
    after_path: impl AsRef<Path>,
    compare_settings: MatchedControlCompareSettings,
    compare_policy: MatchedControlComparePolicy,
    comparison: &MatchedControlComparison,
    audit: &MatchedControlComparisonAudit,
) -> MatchedControlComparisonExportBundle {
    MatchedControlComparisonExportBundle {
        export_version: MATCHED_CONTROL_COMPARISON_EXPORT_VERSION,
        generated_at_utc: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
        before_path: before_path.as_ref().display().to_string(),
        after_path: after_path.as_ref().display().to_string(),
        compare_settings,
        compare_policy,
        comparison: comparison.clone(),
        audit: audit.clone(),
    }
}

pub fn write_comparison_json_export(
    path: impl AsRef<Path>,
    before_path: impl AsRef<Path>,
    after_path: impl AsRef<Path>,
    compare_settings: MatchedControlCompareSettings,
    compare_policy: MatchedControlComparePolicy,
    comparison: &MatchedControlComparison,
    audit: &MatchedControlComparisonAudit,
) -> Result<(), MatchedControlExportError> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(
        writer,
        &build_comparison_export_bundle(
            before_path,
            after_path,
            compare_settings,
            compare_policy,
            comparison,
            audit,
        ),
    )?;
    Ok(())
}

pub fn format_p_like(value: f64) -> String {
    if value < 0.001 {
        format!("{value:.1e}")
    } else {
        format!("{value:.3}")
    }
}

fn run_family_report(
    family: MatchedControlFamily,
    seed_len: usize,
    settings: MatchedControlRunSettings,
) -> MatchedControlReport {
    let divisors = prime_divisors(family.base);
    let mut membrane_rng = seeded_rng(family, seed_len, 0x1357_2468_9abc_def0);
    let mut control_rng = seeded_rng(family, seed_len, 0xfedc_ba98_7654_3210);

    let mut membrane_scores = Vec::with_capacity(settings.samples);
    let mut control_scores = Vec::with_capacity(settings.samples);
    let mut membrane_primes = 0usize;
    let mut control_primes = 0usize;
    let mut total_digits = 0.0_f64;

    for _ in 0..settings.samples {
        let seed = random_base_string_with_length(family.base, seed_len, &mut membrane_rng);
        let membrane = membrane_value(family, &seed);
        let digits = membrane.to_string().len();
        total_digits += digits as f64;

        let membrane_is_prime = is_prime(&membrane);
        membrane_primes += usize::from(membrane_is_prime);
        membrane_scores.push(if membrane_is_prime { 1.0 } else { 0.0 });

        let control = random_coprime_decimal_number(digits, &divisors, &mut control_rng);
        let control_is_prime = is_prime(&control);
        control_primes += usize::from(control_is_prime);
        control_scores.push(if control_is_prime { 1.0 } else { 0.0 });
    }

    let membrane = arm_stats(membrane_primes, settings.samples, settings.confidence_level);
    let control = arm_stats(control_primes, settings.samples, settings.confidence_level);
    let diff = membrane.rate - control.rate;
    let diff_ci = difference_ci(
        membrane_primes,
        settings.samples,
        control_primes,
        settings.samples,
        settings.confidence_level,
    );
    let lift = if control.rate > 0.0 {
        membrane.rate / control.rate
    } else {
        f64::INFINITY
    };
    let lift_ci = rate_ratio_ci(
        membrane_primes,
        settings.samples,
        control_primes,
        settings.samples,
        settings.confidence_level,
    );

    MatchedControlReport {
        family,
        seed_len,
        mean_digits: total_digits / settings.samples as f64,
        membrane,
        control,
        diff,
        diff_ci,
        lift,
        lift_ci,
        hedges_g: hedges_g(&membrane_scores, &control_scores),
        p_value: two_proportion_p_value(
            membrane_primes,
            settings.samples,
            control_primes,
            settings.samples,
        ),
        q_value: f64::NAN,
        decision: MatchedControlDecision::NotSignificant,
    }
}

fn arm_stats(primes: usize, samples: usize, confidence_level: f64) -> MatchedControlArmStats {
    let rate = primes as f64 / samples as f64;
    MatchedControlArmStats {
        primes,
        samples,
        rate,
        ci: wilson_ci(primes, samples, confidence_level),
    }
}

fn seeded_rng(family: MatchedControlFamily, seed_len: usize, salt: u64) -> u64 {
    salt ^ ((family.base as u64) << 40)
        ^ ((family.outer as u64) << 24)
        ^ ((family.inner as u64) << 16)
        ^ ((family.k_outer as u64) << 8)
        ^ ((family.k_inner as u64) << 4)
        ^ seed_len as u64
}

fn next_u64(rng: &mut u64) -> u64 {
    *rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
    *rng
}

fn next_bounded(rng: &mut u64, modulus: u32) -> u32 {
    ((next_u64(rng) >> 32) % u64::from(modulus)) as u32
}

fn random_base_string_with_length(base: u32, length: usize, rng: &mut u64) -> String {
    let mut digits = String::with_capacity(length.max(1));
    let first = next_bounded(rng, base - 1) + 1;
    digits.push(digit_char(first));

    for _ in 1..length {
        let digit = next_bounded(rng, base);
        digits.push(digit_char(digit));
    }

    digits
}

fn membrane_value(family: MatchedControlFamily, seed: &str) -> BigUint {
    let outer = to_base_string(family.outer, family.base);
    let inner = to_base_string(family.inner, family.base);
    let encoded = format!(
        "{}{}{}{}{}{}{}{}{}",
        outer,
        "0".repeat(family.k_outer as usize),
        inner,
        "0".repeat(family.k_inner as usize),
        seed,
        "0".repeat(family.k_inner as usize),
        inner,
        "0".repeat(family.k_outer as usize),
        outer
    );

    BigUint::parse_bytes(encoded.as_bytes(), family.base)
        .unwrap_or_else(|| panic!("failed to parse base-{} membrane: {}", family.base, encoded))
}

fn random_coprime_decimal_number(
    decimal_digits: usize,
    divisors: &[u32],
    rng: &mut u64,
) -> BigUint {
    loop {
        let first = next_bounded(rng, 9) + 1;
        let mut n = BigUint::from(first);

        for _ in 1..decimal_digits {
            n = n * 10u32 + next_bounded(rng, 10);
        }

        if is_coprime_to_base(&n, divisors) {
            return n;
        }
    }
}

fn is_coprime_to_base(n: &BigUint, divisors: &[u32]) -> bool {
    divisors.iter().all(|&p| n % p != BigUint::from(0u32))
}

fn prime_divisors(mut base: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let mut p = 2u32;

    while p * p <= base {
        if base.is_multiple_of(p) {
            factors.push(p);
            while base.is_multiple_of(p) {
                base /= p;
            }
        }
        p += 1;
    }

    if base > 1 {
        factors.push(base);
    }

    factors
}

fn digit_char(digit: u32) -> char {
    if digit < 10 {
        char::from_digit(digit, 10).unwrap()
    } else {
        char::from_u32(u32::from(b'A') + digit - 10).unwrap()
    }
}

fn to_base_string(mut value: u32, base: u32) -> String {
    if value == 0 {
        return "0".to_string();
    }

    let mut out = String::new();
    while value > 0 {
        out.insert(0, digit_char(value % base));
        value /= base;
    }
    out
}

fn normal_quantile(confidence_level: f64) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    normal.inverse_cdf(0.5 + confidence_level / 2.0)
}

fn wilson_ci(successes: usize, trials: usize, confidence_level: f64) -> (f64, f64) {
    let n = trials as f64;
    let p = successes as f64 / n;
    let z = normal_quantile(confidence_level);
    let z2 = z * z;
    let denom = 1.0 + z2 / n;
    let center = (p + z2 / (2.0 * n)) / denom;
    let radius = z * ((p * (1.0 - p) / n + z2 / (4.0 * n * n)).sqrt()) / denom;

    ((center - radius).max(0.0), (center + radius).min(1.0))
}

fn difference_ci(
    successes_a: usize,
    trials_a: usize,
    successes_b: usize,
    trials_b: usize,
    confidence_level: f64,
) -> (f64, f64) {
    let p1 = successes_a as f64 / trials_a as f64;
    let p2 = successes_b as f64 / trials_b as f64;
    let diff = p1 - p2;
    let z = normal_quantile(confidence_level);
    let se = (p1 * (1.0 - p1) / trials_a as f64 + p2 * (1.0 - p2) / trials_b as f64).sqrt();
    let margin = z * se;

    (diff - margin, diff + margin)
}

fn rate_ratio_ci(
    successes_a: usize,
    trials_a: usize,
    successes_b: usize,
    trials_b: usize,
    confidence_level: f64,
) -> (f64, f64) {
    let a = successes_a as f64 + 0.5;
    let c = successes_b as f64 + 0.5;
    let n1 = trials_a as f64 + 1.0;
    let n2 = trials_b as f64 + 1.0;
    let log_rr = (a / n1).ln() - (c / n2).ln();
    let se = (1.0 / a - 1.0 / n1 + 1.0 / c - 1.0 / n2).sqrt();
    let z = normal_quantile(confidence_level);

    ((log_rr - z * se).exp(), (log_rr + z * se).exp())
}

fn two_proportion_p_value(
    successes_a: usize,
    trials_a: usize,
    successes_b: usize,
    trials_b: usize,
) -> f64 {
    let n1 = trials_a as f64;
    let n2 = trials_b as f64;
    let x1 = successes_a as f64;
    let x2 = successes_b as f64;
    let p1 = x1 / n1;
    let p2 = x2 / n2;
    let pooled = (x1 + x2) / (n1 + n2);
    let se = (pooled * (1.0 - pooled) * (1.0 / n1 + 1.0 / n2)).sqrt();

    if se == 0.0 {
        return 1.0;
    }

    let z = (p1 - p2) / se;
    let normal = Normal::new(0.0, 1.0).unwrap();
    2.0 * (1.0 - normal.cdf(z.abs()))
}

fn classify_decision(diff: f64, p_value: f64, q_value: f64, fdr: f64) -> MatchedControlDecision {
    if q_value < fdr {
        if diff > 0.0 {
            MatchedControlDecision::PositiveQ
        } else {
            MatchedControlDecision::NegativeQ
        }
    } else if p_value < 0.05 {
        if diff > 0.0 {
            MatchedControlDecision::PositiveRaw
        } else {
            MatchedControlDecision::NegativeRaw
        }
    } else {
        MatchedControlDecision::NotSignificant
    }
}

fn finite_or_none(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

fn option_delta(before: Option<f64>, after: Option<f64>) -> Option<f64> {
    Some(after? - before?)
}

fn snapshot_from_row(row: &MatchedControlExportRow) -> MatchedControlFamilySnapshot {
    MatchedControlFamilySnapshot {
        family_label: row.family_label.clone(),
        family_code: row.family_code.clone(),
    }
}

fn accumulate(acc: &mut AggregateAccumulator, report: &MatchedControlReport) {
    acc.membrane_primes += report.membrane.primes;
    acc.control_primes += report.control.primes;
    acc.samples += report.membrane.samples;
    acc.families += 1;

    if report.decision == MatchedControlDecision::PositiveQ {
        acc.positive_q += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maintained_family_catalog_extends_beyond_three_k00_lanes() {
        assert!(
            MAINTAINED_MATCHED_CONTROL_FAMILIES.len() > 3,
            "expected widened maintained family set"
        );
        assert!(MAINTAINED_MATCHED_CONTROL_FAMILIES
            .iter()
            .any(|family| family.k_outer != 0 || family.k_inner != 0));
    }

    #[test]
    fn maintained_family_catalog_covers_base12_and_base14() {
        assert!(MAINTAINED_MATCHED_CONTROL_FAMILIES
            .iter()
            .any(|family| family.base == 12));
        assert!(MAINTAINED_MATCHED_CONTROL_FAMILIES
            .iter()
            .any(|family| family.base == 14));
    }

    #[test]
    fn prime_divisors_strip_repeated_factors() {
        assert_eq!(prime_divisors(30), vec![2, 3, 5]);
        assert_eq!(prime_divisors(18), vec![2, 3]);
    }

    #[test]
    fn decimal_control_rng_reaches_base10_unit_digits_for_even_lengths() {
        let family = MAINTAINED_MATCHED_CONTROL_FAMILIES[1];
        let divisors = prime_divisors(family.base);
        let mut rng = seeded_rng(family, 2, 0xfedc_ba98_7654_3210);

        let n = random_coprime_decimal_number(6, &divisors, &mut rng);

        assert_eq!(n.to_string().len(), 6);
        assert!(is_coprime_to_base(&n, &divisors));
    }

    #[test]
    fn positive_q_summary_marks_residual_gate_inputs() {
        let family = MAINTAINED_MATCHED_CONTROL_FAMILIES[0];
        let report = MatchedControlReport {
            family,
            seed_len: 1,
            mean_digits: 4.0,
            membrane: MatchedControlArmStats {
                primes: 30,
                samples: 50,
                rate: 0.6,
                ci: (0.46, 0.73),
            },
            control: MatchedControlArmStats {
                primes: 10,
                samples: 50,
                rate: 0.2,
                ci: (0.11, 0.34),
            },
            diff: 0.4,
            diff_ci: (0.2, 0.6),
            lift: 3.0,
            lift_ci: (1.7, 5.2),
            hedges_g: 0.8,
            p_value: 0.001,
            q_value: 0.002,
            decision: MatchedControlDecision::PositiveQ,
        };

        let summary = summarize_reports(
            &[report],
            MatchedControlRunSettings {
                samples: 50,
                min_seed_len: 1,
                max_seed_len: 1,
                fdr: 0.05,
                confidence_level: 0.95,
            },
        );

        assert_eq!(summary.positive_q, 1);
        assert_eq!(summary.negative_q, 0);
        assert_eq!(summary.positive_q_bases, vec![family.base]);
    }

    #[test]
    fn export_rows_sanitize_non_finite_metrics() {
        let family = MAINTAINED_MATCHED_CONTROL_FAMILIES[0];
        let row = MatchedControlExportRow::from_report(
            &MatchedControlReport {
                family,
                seed_len: 1,
                mean_digits: 4.0,
                membrane: MatchedControlArmStats {
                    primes: 5,
                    samples: 10,
                    rate: 0.5,
                    ci: (0.24, 0.76),
                },
                control: MatchedControlArmStats {
                    primes: 0,
                    samples: 10,
                    rate: 0.0,
                    ci: (0.0, 0.28),
                },
                diff: 0.5,
                diff_ci: (0.2, 0.8),
                lift: f64::INFINITY,
                lift_ci: (1.2, 8.4),
                hedges_g: 1.0,
                p_value: 0.03,
                q_value: f64::NAN,
                decision: MatchedControlDecision::PositiveRaw,
            },
            MatchedControlRunSettings::default(),
        );

        assert_eq!(row.lift, None);
        assert_eq!(row.q_value, None);
        assert_eq!(row.decision, "positive-raw");
    }

    #[test]
    fn compare_export_bundles_marks_material_family_and_residual_changes() {
        let before = MatchedControlExportBundle {
            export_version: MATCHED_CONTROL_EXPORT_VERSION,
            generated_at_utc: "2026-04-02T16:00:00Z".to_string(),
            settings: MatchedControlRunSettings::default(),
            reports: vec![
                MatchedControlExportRow {
                    family_label: "Base 10 classic".to_string(),
                    family_code: "B10 ( 3, 7) k=(0,0) M=1".to_string(),
                    base: 10,
                    outer: 3,
                    inner: 7,
                    k_outer: 0,
                    k_inner: 0,
                    seed_len: 1,
                    mean_digits: 5.0,
                    samples_per_arm: 100,
                    confidence_level: 0.95,
                    fdr_threshold: 0.05,
                    membrane_primes: 10,
                    membrane_rate: 0.10,
                    membrane_ci_lo: 0.05,
                    membrane_ci_hi: 0.18,
                    control_primes: 10,
                    control_rate: 0.10,
                    control_ci_lo: 0.05,
                    control_ci_hi: 0.18,
                    diff: 0.0,
                    diff_ci_lo: -0.08,
                    diff_ci_hi: 0.08,
                    lift: Some(1.0),
                    lift_ci_lo: Some(0.6),
                    lift_ci_hi: Some(1.6),
                    hedges_g: Some(0.0),
                    p_value: Some(0.9),
                    q_value: Some(0.40),
                    decision: "ns".to_string(),
                },
                MatchedControlExportRow {
                    family_label: "Base 30 wheel-like".to_string(),
                    family_code: "B30 (11, 7) k=(0,0) M=1".to_string(),
                    base: 30,
                    outer: 11,
                    inner: 7,
                    k_outer: 0,
                    k_inner: 0,
                    seed_len: 1,
                    mean_digits: 7.0,
                    samples_per_arm: 100,
                    confidence_level: 0.95,
                    fdr_threshold: 0.05,
                    membrane_primes: 20,
                    membrane_rate: 0.20,
                    membrane_ci_lo: 0.13,
                    membrane_ci_hi: 0.29,
                    control_primes: 18,
                    control_rate: 0.18,
                    control_ci_lo: 0.11,
                    control_ci_hi: 0.27,
                    diff: 0.02,
                    diff_ci_lo: -0.06,
                    diff_ci_hi: 0.10,
                    lift: Some(1.11),
                    lift_ci_lo: Some(0.70),
                    lift_ci_hi: Some(1.76),
                    hedges_g: Some(0.05),
                    p_value: Some(0.7),
                    q_value: Some(0.45),
                    decision: "ns".to_string(),
                },
            ],
            summary: MatchedControlExportSummary {
                total_families: 2,
                positive_q: 0,
                negative_q: 0,
                positive_raw: 0,
                negative_raw: 0,
                positive_q_bases: vec![],
                pooled_membrane_primes: 30,
                pooled_membrane_samples: 200,
                pooled_membrane_rate: 0.15,
                pooled_membrane_ci_lo: 0.11,
                pooled_membrane_ci_hi: 0.21,
                pooled_control_primes: 28,
                pooled_control_samples: 200,
                pooled_control_rate: 0.14,
                pooled_control_ci_lo: 0.10,
                pooled_control_ci_hi: 0.20,
                pooled_lift: Some(1.07),
                pooled_lift_ci_lo: Some(0.77),
                pooled_lift_ci_hi: Some(1.49),
                residual_criterion_met: false,
                base_summaries: vec![],
            },
        };

        let after = MatchedControlExportBundle {
            export_version: MATCHED_CONTROL_EXPORT_VERSION,
            generated_at_utc: "2026-04-02T17:00:00Z".to_string(),
            settings: MatchedControlRunSettings::default(),
            reports: vec![
                MatchedControlExportRow {
                    family_label: "Base 10 classic".to_string(),
                    family_code: "B10 ( 3, 7) k=(0,0) M=1".to_string(),
                    base: 10,
                    outer: 3,
                    inner: 7,
                    k_outer: 0,
                    k_inner: 0,
                    seed_len: 1,
                    mean_digits: 5.0,
                    samples_per_arm: 100,
                    confidence_level: 0.95,
                    fdr_threshold: 0.05,
                    membrane_primes: 19,
                    membrane_rate: 0.19,
                    membrane_ci_lo: 0.12,
                    membrane_ci_hi: 0.28,
                    control_primes: 10,
                    control_rate: 0.10,
                    control_ci_lo: 0.05,
                    control_ci_hi: 0.18,
                    diff: 0.09,
                    diff_ci_lo: 0.01,
                    diff_ci_hi: 0.17,
                    lift: Some(1.45),
                    lift_ci_lo: Some(1.02),
                    lift_ci_hi: Some(2.05),
                    hedges_g: Some(0.32),
                    p_value: Some(0.03),
                    q_value: Some(0.08),
                    decision: "positive-raw".to_string(),
                },
                MatchedControlExportRow {
                    family_label: "Base 6 champion".to_string(),
                    family_code: "B 6 ( 1, 5) k=(0,0) M=1".to_string(),
                    base: 6,
                    outer: 1,
                    inner: 5,
                    k_outer: 0,
                    k_inner: 0,
                    seed_len: 1,
                    mean_digits: 4.0,
                    samples_per_arm: 100,
                    confidence_level: 0.95,
                    fdr_threshold: 0.05,
                    membrane_primes: 24,
                    membrane_rate: 0.24,
                    membrane_ci_lo: 0.17,
                    membrane_ci_hi: 0.33,
                    control_primes: 16,
                    control_rate: 0.16,
                    control_ci_lo: 0.10,
                    control_ci_hi: 0.24,
                    diff: 0.08,
                    diff_ci_lo: 0.00,
                    diff_ci_hi: 0.16,
                    lift: Some(1.50),
                    lift_ci_lo: Some(1.00),
                    lift_ci_hi: Some(2.24),
                    hedges_g: Some(0.28),
                    p_value: Some(0.05),
                    q_value: Some(0.12),
                    decision: "ns".to_string(),
                },
            ],
            summary: MatchedControlExportSummary {
                total_families: 2,
                positive_q: 1,
                negative_q: 0,
                positive_raw: 1,
                negative_raw: 0,
                positive_q_bases: vec![10],
                pooled_membrane_primes: 43,
                pooled_membrane_samples: 200,
                pooled_membrane_rate: 0.215,
                pooled_membrane_ci_lo: 0.16,
                pooled_membrane_ci_hi: 0.28,
                pooled_control_primes: 26,
                pooled_control_samples: 200,
                pooled_control_rate: 0.13,
                pooled_control_ci_lo: 0.09,
                pooled_control_ci_hi: 0.19,
                pooled_lift: Some(1.65),
                pooled_lift_ci_lo: Some(1.12),
                pooled_lift_ci_hi: Some(2.42),
                residual_criterion_met: true,
                base_summaries: vec![],
            },
        };

        let comparison = compare_export_bundles(
            &before,
            &after,
            MatchedControlCompareSettings {
                lift_threshold: 0.25,
                q_threshold: 0.10,
            },
        );

        assert!(comparison.residual_criterion_changed);
        assert_eq!(comparison.families_compared, 1);
        assert_eq!(comparison.materially_changed_families.len(), 1);
        assert_eq!(comparison.added_families.len(), 1);
        assert_eq!(comparison.removed_families.len(), 1);

        let delta = &comparison.materially_changed_families[0];
        assert_eq!(delta.family_code, "B10 ( 3, 7) k=(0,0) M=1");
        assert!(delta.material_lift_change);
        assert!(delta.material_q_change);
        assert!(delta.decision_changed);
    }

    #[test]
    fn comparison_audit_flags_residual_and_material_changes() {
        let before = MatchedControlExportBundle {
            export_version: MATCHED_CONTROL_EXPORT_VERSION,
            generated_at_utc: "2026-04-02T16:00:00Z".to_string(),
            settings: MatchedControlRunSettings {
                samples: 20,
                ..MatchedControlRunSettings::default()
            },
            reports: vec![],
            summary: MatchedControlExportSummary {
                total_families: 0,
                positive_q: 0,
                negative_q: 0,
                positive_raw: 0,
                negative_raw: 0,
                positive_q_bases: vec![],
                pooled_membrane_primes: 0,
                pooled_membrane_samples: 0,
                pooled_membrane_rate: 0.0,
                pooled_membrane_ci_lo: 0.0,
                pooled_membrane_ci_hi: 0.0,
                pooled_control_primes: 0,
                pooled_control_samples: 0,
                pooled_control_rate: 0.0,
                pooled_control_ci_lo: 0.0,
                pooled_control_ci_hi: 0.0,
                pooled_lift: Some(1.0),
                pooled_lift_ci_lo: Some(0.7),
                pooled_lift_ci_hi: Some(1.4),
                residual_criterion_met: false,
                base_summaries: vec![],
            },
        };
        let after = MatchedControlExportBundle {
            export_version: MATCHED_CONTROL_EXPORT_VERSION,
            generated_at_utc: "2026-04-02T17:00:00Z".to_string(),
            settings: MatchedControlRunSettings {
                samples: 24,
                ..MatchedControlRunSettings::default()
            },
            reports: vec![],
            summary: MatchedControlExportSummary {
                total_families: 0,
                positive_q: 1,
                negative_q: 0,
                positive_raw: 0,
                negative_raw: 0,
                positive_q_bases: vec![10],
                pooled_membrane_primes: 0,
                pooled_membrane_samples: 0,
                pooled_membrane_rate: 0.0,
                pooled_membrane_ci_lo: 0.0,
                pooled_membrane_ci_hi: 0.0,
                pooled_control_primes: 0,
                pooled_control_samples: 0,
                pooled_control_rate: 0.0,
                pooled_control_ci_lo: 0.0,
                pooled_control_ci_hi: 0.0,
                pooled_lift: Some(1.4),
                pooled_lift_ci_lo: Some(1.1),
                pooled_lift_ci_hi: Some(1.8),
                residual_criterion_met: true,
                base_summaries: vec![],
            },
        };
        let comparison = MatchedControlComparison {
            before_generated_at_utc: before.generated_at_utc.clone(),
            after_generated_at_utc: after.generated_at_utc.clone(),
            before_export_version: before.export_version,
            after_export_version: after.export_version,
            families_compared: 1,
            materially_changed_families: vec![MatchedControlFamilyDelta {
                family_label: "Base 10 classic".to_string(),
                family_code: "B10 ( 3, 7) k=(0,0) M=1".to_string(),
                lift_before: Some(1.0),
                lift_after: Some(1.4),
                lift_delta: Some(0.4),
                q_before: Some(0.4),
                q_after: Some(0.1),
                q_delta: Some(-0.3),
                decision_before: "ns".to_string(),
                decision_after: "positive-raw".to_string(),
                material_lift_change: true,
                material_q_change: true,
                decision_changed: true,
            }],
            added_families: vec![],
            removed_families: vec![],
            residual_criterion_before: false,
            residual_criterion_after: true,
            residual_criterion_changed: true,
            pooled_lift_before: Some(1.0),
            pooled_lift_after: Some(1.4),
            pooled_lift_delta: Some(0.4),
            positive_q_before: 0,
            positive_q_after: 1,
            negative_q_before: 0,
            negative_q_after: 0,
        };

        let audit = summarize_comparison_audit(
            &before,
            &after,
            &comparison,
            MatchedControlComparePolicy::default(),
        );
        let bundle = build_comparison_export_bundle(
            "before.json",
            "after.json",
            MatchedControlCompareSettings::default(),
            MatchedControlComparePolicy::default(),
            &comparison,
            &audit,
        );

        assert!(audit.flagged);
        assert!(audit.sampling_plan_changed);
        assert_eq!(
            audit.reasons,
            vec![
                "residual-criterion-changed".to_string(),
                "material-family-change".to_string()
            ]
        );
        assert_eq!(
            bundle.export_version,
            MATCHED_CONTROL_COMPARISON_EXPORT_VERSION
        );
        assert_eq!(bundle.before_path, "before.json");
        assert_eq!(bundle.after_path, "after.json");
        assert!(bundle.audit.flagged);
    }

    #[test]
    fn comparison_audit_promotes_sampling_and_family_set_changes_only_when_policy_requests_it() {
        let before = MatchedControlExportBundle {
            export_version: MATCHED_CONTROL_EXPORT_VERSION,
            generated_at_utc: "2026-04-02T16:00:00Z".to_string(),
            settings: MatchedControlRunSettings {
                samples: 20,
                ..MatchedControlRunSettings::default()
            },
            reports: vec![],
            summary: MatchedControlExportSummary {
                total_families: 0,
                positive_q: 0,
                negative_q: 0,
                positive_raw: 0,
                negative_raw: 0,
                positive_q_bases: vec![],
                pooled_membrane_primes: 0,
                pooled_membrane_samples: 0,
                pooled_membrane_rate: 0.0,
                pooled_membrane_ci_lo: 0.0,
                pooled_membrane_ci_hi: 0.0,
                pooled_control_primes: 0,
                pooled_control_samples: 0,
                pooled_control_rate: 0.0,
                pooled_control_ci_lo: 0.0,
                pooled_control_ci_hi: 0.0,
                pooled_lift: Some(1.0),
                pooled_lift_ci_lo: Some(0.8),
                pooled_lift_ci_hi: Some(1.2),
                residual_criterion_met: false,
                base_summaries: vec![],
            },
        };
        let after = MatchedControlExportBundle {
            export_version: MATCHED_CONTROL_EXPORT_VERSION,
            generated_at_utc: "2026-04-02T17:00:00Z".to_string(),
            settings: MatchedControlRunSettings {
                samples: 24,
                ..MatchedControlRunSettings::default()
            },
            reports: vec![],
            summary: MatchedControlExportSummary {
                total_families: 0,
                positive_q: 0,
                negative_q: 0,
                positive_raw: 0,
                negative_raw: 0,
                positive_q_bases: vec![],
                pooled_membrane_primes: 0,
                pooled_membrane_samples: 0,
                pooled_membrane_rate: 0.0,
                pooled_membrane_ci_lo: 0.0,
                pooled_membrane_ci_hi: 0.0,
                pooled_control_primes: 0,
                pooled_control_samples: 0,
                pooled_control_rate: 0.0,
                pooled_control_ci_lo: 0.0,
                pooled_control_ci_hi: 0.0,
                pooled_lift: Some(1.0),
                pooled_lift_ci_lo: Some(0.8),
                pooled_lift_ci_hi: Some(1.2),
                residual_criterion_met: false,
                base_summaries: vec![],
            },
        };
        let comparison = MatchedControlComparison {
            before_generated_at_utc: before.generated_at_utc.clone(),
            after_generated_at_utc: after.generated_at_utc.clone(),
            before_export_version: before.export_version,
            after_export_version: after.export_version,
            families_compared: 0,
            materially_changed_families: vec![],
            added_families: vec![MatchedControlFamilySnapshot {
                family_label: "Base 6 champion".to_string(),
                family_code: "B 6 ( 1, 5) k=(0,0) M=1".to_string(),
            }],
            removed_families: vec![MatchedControlFamilySnapshot {
                family_label: "Base 30 wheel-like".to_string(),
                family_code: "B30 (11, 7) k=(0,0) M=1".to_string(),
            }],
            residual_criterion_before: false,
            residual_criterion_after: false,
            residual_criterion_changed: false,
            pooled_lift_before: Some(1.0),
            pooled_lift_after: Some(1.0),
            pooled_lift_delta: Some(0.0),
            positive_q_before: 0,
            positive_q_after: 0,
            negative_q_before: 0,
            negative_q_after: 0,
        };

        let default_audit = summarize_comparison_audit(
            &before,
            &after,
            &comparison,
            MatchedControlComparePolicy::default(),
        );
        let strict_audit = summarize_comparison_audit(
            &before,
            &after,
            &comparison,
            MatchedControlComparePolicy {
                flag_sampling_plan_drift: true,
                flag_added_families: true,
                flag_removed_families: true,
            },
        );

        assert!(!default_audit.flagged);
        assert!(default_audit.sampling_plan_changed);
        assert!(default_audit.reasons.is_empty());

        assert!(strict_audit.flagged);
        assert_eq!(
            strict_audit.reasons,
            vec![
                "sampling-plan-drift".to_string(),
                "added-families".to_string(),
                "removed-families".to_string()
            ]
        );
    }
}
