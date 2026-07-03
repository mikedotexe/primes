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
    fmt::Write as _,
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
pub const MATCHED_CONTROL_COMPARISON_BATCH_EXPORT_VERSION: u32 = 1;
pub const MATCHED_CONTROL_ATLAS_SCHEMA_VERSION: &str = "matched-control-atlas-v1";
pub const DEFAULT_COMPARE_LIFT_THRESHOLD: f64 = 0.25;
pub const DEFAULT_COMPARE_Q_THRESHOLD: f64 = 0.10;
pub const MATCHED_CONTROL_COMPARE_FLAG_EXIT_CODE: i32 = 3;
pub const MATCHED_CONTROL_SMOKE_PANEL_ID: &str = "canonical-smoke-v1";
pub const MATCHED_CONTROL_AUDIT_PANEL_ID: &str = "canonical-audit-v1";
pub const MATCHED_CONTROL_LEAN_LANE_MODULE: &str =
    "PrimeArithmetic.Generated.MatchedControlFamilyLanes";
pub const MATCHED_CONTROL_SMOKE_PROFILE_MODULE: &str =
    "PrimeArithmetic.Density.CanonicalSmokeLaneProfiles";
pub const MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE: &str =
    "PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates";
pub const MATCHED_CONTROL_BASE10_SEPARATION_MODULE: &str =
    "PrimeArithmetic.Density.Base10SeedClassSeparation";
pub const MATCHED_CONTROL_BASE10_M2_SMOKE_SEPARATION_THEOREM: &str =
    "forbiddenSeedMask_base10BreathingM2_ne_base10SymmetricM2_mod7";
pub const MATCHED_CONTROL_BASE10_M2_SMOKE_FORBIDDEN_RESIDUES_THEOREM: &str =
    "forbiddenResidues_base10BreathingM2_ne_base10SymmetricM2_mod7";
pub const MATCHED_CONTROL_BASE10_M2_SMOKE_EQUAL_SURVIVOR_THEOREM: &str =
    "survivorResidueCount_base10BreathingM2_eq_base10SymmetricM2_mod7";
pub const MATCHED_CONTROL_BASE10_CLASSIC_M2_SMOKE_SEPARATION_THEOREM: &str =
    "forbiddenSeedMask_base10ClassicM2_ne_base10ExclusiveM2_mod7";
pub const MATCHED_CONTROL_BASE10_CLASSIC_M2_SMOKE_FORBIDDEN_RESIDUES_THEOREM: &str =
    "forbiddenResidues_base10ClassicM2_ne_base10ExclusiveM2_mod7";
pub const MATCHED_CONTROL_BASE10_CLASSIC_M2_SMOKE_EQUAL_SURVIVOR_THEOREM: &str =
    "survivorResidueCount_base10ClassicM2_eq_base10ExclusiveM2_mod7";
pub const MATCHED_CONTROL_BASE10_BREATHING_CLASSIC_M2_SMOKE_SEPARATION_THEOREM: &str =
    "forbiddenSeedMask_base10BreathingM2_ne_base10ClassicM2_mod7";
pub const MATCHED_CONTROL_BASE10_BREATHING_CLASSIC_M2_SMOKE_FORBIDDEN_RESIDUES_THEOREM: &str =
    "forbiddenResidues_base10BreathingM2_ne_base10ClassicM2_mod7";
pub const MATCHED_CONTROL_BASE10_BREATHING_CLASSIC_M2_SMOKE_EQUAL_SURVIVOR_THEOREM: &str =
    "survivorResidueCount_base10BreathingM2_eq_base10ClassicM2_mod7";
pub const MATCHED_CONTROL_BASE10_SYMMETRIC_EXCLUSIVE_M2_SMOKE_SEPARATION_THEOREM: &str =
    "forbiddenSeedMask_base10SymmetricM2_ne_base10ExclusiveM2_mod7";
pub const MATCHED_CONTROL_BASE10_SYMMETRIC_EXCLUSIVE_M2_SMOKE_FORBIDDEN_RESIDUES_THEOREM: &str =
    "forbiddenResidues_base10SymmetricM2_ne_base10ExclusiveM2_mod7";
pub const MATCHED_CONTROL_BASE10_SYMMETRIC_EXCLUSIVE_M2_SMOKE_EQUAL_SURVIVOR_THEOREM: &str =
    "survivorResidueCount_base10SymmetricM2_eq_base10ExclusiveM2_mod7";
pub const MATCHED_CONTROL_BASE10_BREATHING_EXCLUSIVE_M2_SMOKE_SEPARATION_THEOREM: &str =
    "forbiddenSeedMask_base10BreathingM2_ne_base10ExclusiveM2_mod7";
pub const MATCHED_CONTROL_BASE10_BREATHING_EXCLUSIVE_M2_SMOKE_FORBIDDEN_RESIDUES_THEOREM: &str =
    "forbiddenResidues_base10BreathingM2_ne_base10ExclusiveM2_mod7";
pub const MATCHED_CONTROL_BASE10_BREATHING_EXCLUSIVE_M2_SMOKE_EQUAL_SURVIVOR_THEOREM: &str =
    "survivorResidueCount_base10BreathingM2_eq_base10ExclusiveM2_mod7";
pub const MATCHED_CONTROL_BASE10_SYMMETRIC_CLASSIC_M2_SMOKE_SEPARATION_THEOREM: &str =
    "forbiddenSeedMask_base10SymmetricM2_ne_base10ClassicM2_mod3";
pub const MATCHED_CONTROL_BASE10_SYMMETRIC_CLASSIC_M2_SMOKE_FORBIDDEN_RESIDUES_THEOREM: &str =
    "forbiddenResidues_base10SymmetricM2_ne_base10ClassicM2_mod3";
pub const MATCHED_CONTROL_BASE10_SYMMETRIC_CLASSIC_M2_SMOKE_EQUAL_SURVIVOR_THEOREM: &str =
    "survivorResidueCount_base10SymmetricM2_eq_base10ClassicM2_mod3";
pub const MATCHED_CONTROL_BASE10_M1_BREATHING_SYMMETRIC_SEPARATION_THEOREM: &str =
    "forbiddenSeedMask_breathingM1_ne_symmetricM1_mod11";
pub const MATCHED_CONTROL_BASE10_M1_BREATHING_SYMMETRIC_FORBIDDEN_RESIDUES_THEOREM: &str =
    "forbiddenResidues_breathingM1_ne_symmetricM1_mod11";
pub const MATCHED_CONTROL_BASE10_M1_BREATHING_SYMMETRIC_EQUAL_SURVIVOR_THEOREM: &str =
    "survivorResidueCount_breathingM1_eq_symmetricM1_mod11";
pub const MATCHED_CONTROL_BASE10_M1_BREATHING_EXCLUSIVE_SEPARATION_THEOREM: &str =
    "forbiddenSeedMask_breathingM1_ne_exclusiveM1_mod11";
pub const MATCHED_CONTROL_BASE10_M1_BREATHING_EXCLUSIVE_FORBIDDEN_RESIDUES_THEOREM: &str =
    "forbiddenResidues_breathingM1_ne_exclusiveM1_mod11";
pub const MATCHED_CONTROL_BASE10_M1_BREATHING_EXCLUSIVE_EQUAL_SURVIVOR_THEOREM: &str =
    "survivorResidueCount_breathingM1_eq_exclusiveM1_mod11";
pub const MATCHED_CONTROL_BASE10_M1_BREATHING_CLASSIC_SEPARATION_THEOREM: &str =
    "forbiddenSeedMask_breathingM1_ne_classicM1_mod11";
pub const MATCHED_CONTROL_BASE10_M1_BREATHING_CLASSIC_FORBIDDEN_RESIDUES_THEOREM: &str =
    "forbiddenResidues_breathingM1_ne_classicM1_mod11";
pub const MATCHED_CONTROL_BASE10_M1_BREATHING_CLASSIC_EQUAL_SURVIVOR_THEOREM: &str =
    "survivorResidueCount_breathingM1_eq_classicM1_mod11";
pub const MATCHED_CONTROL_BASE10_M1_CLASSIC_EXCLUSIVE_SEPARATION_THEOREM: &str =
    "forbiddenSeedMask_classicM1_ne_exclusiveM1_mod11";
pub const MATCHED_CONTROL_BASE10_M1_CLASSIC_EXCLUSIVE_FORBIDDEN_RESIDUES_THEOREM: &str =
    "forbiddenResidues_classicM1_ne_exclusiveM1_mod11";
pub const MATCHED_CONTROL_BASE10_M1_CLASSIC_EXCLUSIVE_EQUAL_SURVIVOR_THEOREM: &str =
    "survivorResidueCount_classicM1_eq_exclusiveM1_mod11";
pub const MATCHED_CONTROL_BASE10_M1_SYMMETRIC_CLASSIC_SEPARATION_THEOREM: &str =
    "forbiddenSeedMask_symmetricM1_ne_classicM1_mod11";
pub const MATCHED_CONTROL_BASE10_M1_SYMMETRIC_CLASSIC_FORBIDDEN_RESIDUES_THEOREM: &str =
    "forbiddenResidues_symmetricM1_ne_classicM1_mod11";
pub const MATCHED_CONTROL_BASE10_M1_SYMMETRIC_CLASSIC_EQUAL_SURVIVOR_THEOREM: &str =
    "survivorResidueCount_symmetricM1_eq_classicM1_mod11";
pub const MATCHED_CONTROL_BASE10_M1_SYMMETRIC_EXCLUSIVE_SEPARATION_THEOREM: &str =
    "forbiddenSeedMask_symmetricM1_ne_exclusiveM1_mod11";
pub const MATCHED_CONTROL_BASE10_M1_SYMMETRIC_EXCLUSIVE_FORBIDDEN_RESIDUES_THEOREM: &str =
    "forbiddenResidues_symmetricM1_ne_exclusiveM1_mod11";
pub const MATCHED_CONTROL_BASE10_M1_SYMMETRIC_EXCLUSIVE_EQUAL_SURVIVOR_THEOREM: &str =
    "survivorResidueCount_symmetricM1_eq_exclusiveM1_mod11";

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

pub fn matched_control_lean_lane_name(
    family: &MatchedControlFamily,
    middle_width: usize,
) -> String {
    let mut name = String::new();
    for (index, token) in identifier_tokens(family.label).iter().enumerate() {
        if index == 0 {
            name.push_str(&token.to_ascii_lowercase());
        } else {
            name.push_str(&upper_camel_token(token));
        }
    }
    name.push('M');
    name.push_str(&middle_width.to_string());
    name.push_str("Lane");
    name
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchedControlPanel {
    Smoke,
    Audit,
}

impl MatchedControlPanel {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "smoke" => Some(Self::Smoke),
            "audit" => Some(Self::Audit),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Smoke => "smoke",
            Self::Audit => "audit",
        }
    }

    pub fn panel_id(self) -> &'static str {
        match self {
            Self::Smoke => MATCHED_CONTROL_SMOKE_PANEL_ID,
            Self::Audit => MATCHED_CONTROL_AUDIT_PANEL_ID,
        }
    }

    pub fn settings(self) -> MatchedControlRunSettings {
        match self {
            Self::Smoke => MatchedControlRunSettings {
                samples: 250,
                min_seed_len: 1,
                max_seed_len: 2,
                fdr: DEFAULT_FDR,
                confidence_level: DEFAULT_CONFIDENCE_LEVEL,
            },
            Self::Audit => MatchedControlRunSettings {
                samples: 5000,
                min_seed_len: 1,
                max_seed_len: 4,
                fdr: DEFAULT_FDR,
                confidence_level: DEFAULT_CONFIDENCE_LEVEL,
            },
        }
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

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MatchedControlBatchError {
    #[error("matched-control comparison batch requires at least one input")]
    EmptyInput,
    #[error("comparison export {index} has different compare settings than the first input")]
    CompareSettingsMismatch { index: usize },
    #[error("comparison export {index} has different compare policy than the first input")]
    ComparePolicyMismatch { index: usize },
    #[error("comparison export {index} compares different panel ids before vs after")]
    PanelMismatchWithinComparison { index: usize },
    #[error("comparison export {index} has a different panel id than the first input")]
    PanelMismatchAcrossBatch { index: usize },
    #[error("comparison export {index} lacks compared-family snapshots; regenerate it with the current compare helper")]
    MissingComparedFamilySnapshots { index: usize },
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
    #[serde(default)]
    pub panel_id: Option<String>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MatchedControlAuditSeverity {
    Clear,
    Info,
    Error,
}

impl MatchedControlAuditSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::Info => "info",
            Self::Error => "error",
        }
    }

    fn is_error(self) -> bool {
        matches!(self, Self::Error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchedControlAuditCondition {
    pub active: bool,
    pub severity: MatchedControlAuditSeverity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchedControlComparisonAuditConditions {
    pub residual_criterion_changed: MatchedControlAuditCondition,
    pub material_family_change: MatchedControlAuditCondition,
    pub sampling_plan_drift: MatchedControlAuditCondition,
    pub added_families: MatchedControlAuditCondition,
    pub removed_families: MatchedControlAuditCondition,
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
    #[serde(default)]
    pub before_panel_id: Option<String>,
    #[serde(default)]
    pub after_panel_id: Option<String>,
    pub families_compared: usize,
    #[serde(default)]
    pub compared_families: Vec<MatchedControlFamilySnapshot>,
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
    pub conditions: MatchedControlComparisonAuditConditions,
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

#[derive(Debug, Clone)]
pub struct MatchedControlComparisonBatchInput {
    pub source_path: String,
    pub bundle: MatchedControlComparisonExportBundle,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchedControlAuditSeverityTally {
    pub clear: usize,
    pub info: usize,
    pub error: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchedControlComparisonBatchConditionTallies {
    pub residual_criterion_changed: MatchedControlAuditSeverityTally,
    pub material_family_change: MatchedControlAuditSeverityTally,
    pub sampling_plan_drift: MatchedControlAuditSeverityTally,
    pub added_families: MatchedControlAuditSeverityTally,
    pub removed_families: MatchedControlAuditSeverityTally,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MatchedControlBatchFamilyStatus {
    Stable,
    Drifting,
}

impl MatchedControlBatchFamilyStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Drifting => "drifting",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedControlComparisonBatchRunRow {
    pub source_path: String,
    pub generated_at_utc: String,
    pub before_path: String,
    pub after_path: String,
    pub before_panel_id: Option<String>,
    pub after_panel_id: Option<String>,
    pub flagged: bool,
    pub residual_criterion_changed: bool,
    pub material_family_change_count: usize,
    pub added_family_count: usize,
    pub removed_family_count: usize,
    pub residual_criterion_changed_severity: MatchedControlAuditSeverity,
    pub material_family_change_severity: MatchedControlAuditSeverity,
    pub sampling_plan_drift_severity: MatchedControlAuditSeverity,
    pub added_families_severity: MatchedControlAuditSeverity,
    pub removed_families_severity: MatchedControlAuditSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedControlComparisonBatchFamilyRow {
    pub family_label: String,
    pub family_code: String,
    pub status: MatchedControlBatchFamilyStatus,
    pub material_change_count: usize,
    pub decision_change_count: usize,
    pub added_count: usize,
    pub removed_count: usize,
    pub max_abs_lift_delta: Option<f64>,
    pub max_abs_q_delta: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedControlComparisonBatchSummary {
    pub export_version: u32,
    pub generated_at_utc: String,
    pub panel_id: Option<String>,
    pub compare_settings: MatchedControlCompareSettings,
    pub compare_policy: MatchedControlComparePolicy,
    pub run_count: usize,
    pub flagged_run_count: usize,
    pub residual_criterion_flip_count: usize,
    pub condition_tallies: MatchedControlComparisonBatchConditionTallies,
    pub stable_family_count: usize,
    pub drifting_family_count: usize,
    pub run_rows: Vec<MatchedControlComparisonBatchRunRow>,
    pub family_rows: Vec<MatchedControlComparisonBatchFamilyRow>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MatchedControlAtlasClaimStatus {
    NoDensityMechanismClaim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MatchedControlAtlasProofStatus {
    LaneGeneratedOnly,
    ExactResidueProfile,
    ExactSeedClassSeparation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchedControlAtlasLeanLink {
    pub module: String,
    pub lane_constant: String,
    pub lane_constant_qualified: String,
    pub lookup_theorem: String,
    pub lookup_theorem_qualified: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchedControlAtlasProofCertificate {
    pub module: String,
    pub certificate_constant: Option<String>,
    pub modulus: u32,
    pub excluded_seed_class: u32,
    pub excluded_seed_class_theorem: String,
    pub divisibility_iff_theorem: String,
    pub separation_theorem: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchedControlAtlasFamilyRow {
    pub family_label: String,
    pub family_code: String,
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub k_outer: u32,
    pub k_inner: u32,
    pub middle_width: usize,
    pub lean: MatchedControlAtlasLeanLink,
    pub proof_status: MatchedControlAtlasProofStatus,
    pub proof_certificate: Option<MatchedControlAtlasProofCertificate>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchedControlAtlasManifest {
    pub schema_version: String,
    pub panel_id: String,
    pub panel: String,
    pub settings: MatchedControlRunSettings,
    pub family_count: usize,
    pub lane_count: usize,
    pub lean_lane_module: String,
    pub claim_status: MatchedControlAtlasClaimStatus,
    pub families: Vec<MatchedControlAtlasFamilyRow>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchedControlSmokeProfileCertificateSpec {
    pub family_index: usize,
    pub middle_width: usize,
    pub modulus: u32,
    pub excluded_seed_class: u32,
    pub certificate_constant: &'static str,
    pub coprime_theorem: &'static str,
    pub zero_seed_class_theorem: &'static str,
    pub profile_excluded_seed_class_theorem: &'static str,
    pub divisibility_iff_theorem: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchedControlSmokeProfileCertificateMetadata {
    pub family_label: String,
    pub family_code: String,
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub k_outer: u32,
    pub k_inner: u32,
    pub middle_width: usize,
    pub lean_module: String,
    pub lean_lane_constant: String,
    pub lean_lookup_theorem: String,
    pub certificate_constant: String,
    pub modulus: u32,
    pub excluded_seed_class: u32,
    pub coprime_theorem: String,
    pub zero_seed_class_theorem: String,
    pub profile_excluded_seed_class_theorem: String,
    pub divisibility_iff_theorem: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchedControlSmokePairCertificateSpec {
    pub left_family_index: usize,
    pub right_family_index: usize,
    pub middle_width: usize,
    pub modulus: u32,
    pub left_excluded_seed_class: u32,
    pub right_excluded_seed_class: u32,
    pub lean_module: &'static str,
    pub zero_seed_class_ne_theorem: &'static str,
    pub separation_theorem: &'static str,
    pub forbidden_residue_set_theorem: Option<&'static str>,
    pub equal_survivor_theorem: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchedControlSmokePairCertificateMetadata {
    pub left_family_label: String,
    pub right_family_label: String,
    pub left_family_code: String,
    pub right_family_code: String,
    pub left_lean_lane_constant: String,
    pub right_lean_lane_constant: String,
    pub middle_width: usize,
    pub modulus: u32,
    pub left_excluded_seed_class: u32,
    pub right_excluded_seed_class: u32,
    pub lean_module: String,
    pub zero_seed_class_ne_theorem: String,
    pub zero_seed_class_ne_theorem_qualified: String,
    pub separation_theorem: String,
    pub separation_theorem_qualified: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbidden_residue_set_theorem: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbidden_residue_set_theorem_qualified: Option<String>,
    pub equal_survivor_theorem: String,
    pub equal_survivor_theorem_qualified: String,
}

pub const MAINTAINED_MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATES:
    [MatchedControlSmokeProfileCertificateSpec; 14] = [
    MatchedControlSmokeProfileCertificateSpec {
        family_index: 0,
        middle_width: 1,
        modulus: 7,
        excluded_seed_class: 1,
        certificate_constant: "base6ChampionM1Mod7Certificate",
        coprime_theorem: "modSevenCoprime_base6ChampionM1",
        zero_seed_class_theorem: "zeroSeedClass_base6ChampionM1_mod7",
        profile_excluded_seed_class_theorem: "base6ChampionM1ProfileAt_mod7_excludedSeedClass",
        divisibility_iff_theorem: "templateValue_base6ChampionM1_mod7_eq_zero_iff_seed_mod_eq_one",
    },
    MatchedControlSmokeProfileCertificateSpec {
        family_index: 0,
        middle_width: 2,
        modulus: 7,
        excluded_seed_class: 0,
        certificate_constant: "base6ChampionM2Mod7Certificate",
        coprime_theorem: "modSevenCoprime_base6ChampionM2",
        zero_seed_class_theorem: "zeroSeedClass_base6ChampionM2_mod7",
        profile_excluded_seed_class_theorem: "base6ChampionM2ProfileAt_mod7_excludedSeedClass",
        divisibility_iff_theorem: "templateValue_base6ChampionM2_mod7_eq_zero_iff_seed_mod_eq_zero",
    },
    MatchedControlSmokeProfileCertificateSpec {
        family_index: 2,
        middle_width: 2,
        modulus: 7,
        excluded_seed_class: 3,
        certificate_constant: "base10BreathingM2Mod7Certificate",
        coprime_theorem: "modSevenCoprime_base10BreathingM2",
        zero_seed_class_theorem: "zeroSeedClass_base10BreathingM2_mod7",
        profile_excluded_seed_class_theorem: "base10BreathingM2ProfileAt_mod7_excludedSeedClass",
        divisibility_iff_theorem:
            "templateValue_base10BreathingM2_mod7_eq_zero_iff_seed_mod_eq_three",
    },
    MatchedControlSmokeProfileCertificateSpec {
        family_index: 1,
        middle_width: 2,
        modulus: 7,
        excluded_seed_class: 5,
        certificate_constant: "base10ClassicM2Mod7Certificate",
        coprime_theorem: "modSevenCoprime_base10ClassicM2",
        zero_seed_class_theorem: "zeroSeedClass_base10ClassicM2_mod7",
        profile_excluded_seed_class_theorem: "base10ClassicM2ProfileAt_mod7_excludedSeedClass",
        divisibility_iff_theorem: "templateValue_base10ClassicM2_mod7_eq_zero_iff_seed_mod_eq_five",
    },
    MatchedControlSmokeProfileCertificateSpec {
        family_index: 3,
        middle_width: 2,
        modulus: 7,
        excluded_seed_class: 5,
        certificate_constant: "base10SymmetricM2Mod7Certificate",
        coprime_theorem: "modSevenCoprime_base10SymmetricM2",
        zero_seed_class_theorem: "zeroSeedClass_base10SymmetricM2_mod7",
        profile_excluded_seed_class_theorem: "base10SymmetricM2ProfileAt_mod7_excludedSeedClass",
        divisibility_iff_theorem:
            "templateValue_base10SymmetricM2_mod7_eq_zero_iff_seed_mod_eq_five",
    },
    MatchedControlSmokeProfileCertificateSpec {
        family_index: 4,
        middle_width: 2,
        modulus: 7,
        excluded_seed_class: 0,
        certificate_constant: "base10ExclusiveM2Mod7Certificate",
        coprime_theorem: "modSevenCoprime_base10ExclusiveM2",
        zero_seed_class_theorem: "zeroSeedClass_base10ExclusiveM2_mod7",
        profile_excluded_seed_class_theorem: "base10ExclusiveM2ProfileAt_mod7_excludedSeedClass",
        divisibility_iff_theorem:
            "templateValue_base10ExclusiveM2_mod7_eq_zero_iff_seed_mod_eq_zero",
    },
    MatchedControlSmokeProfileCertificateSpec {
        family_index: 5,
        middle_width: 1,
        modulus: 5,
        excluded_seed_class: 2,
        certificate_constant: "base12CompactM1Mod5Certificate",
        coprime_theorem: "modFiveCoprime_base12CompactM1",
        zero_seed_class_theorem: "zeroSeedClass_base12CompactM1_mod5",
        profile_excluded_seed_class_theorem: "base12CompactM1ProfileAt_mod5_excludedSeedClass",
        divisibility_iff_theorem: "templateValue_base12CompactM1_mod5_eq_zero_iff_seed_mod_eq_two",
    },
    MatchedControlSmokeProfileCertificateSpec {
        family_index: 5,
        middle_width: 2,
        modulus: 5,
        excluded_seed_class: 1,
        certificate_constant: "base12CompactM2Mod5Certificate",
        coprime_theorem: "modFiveCoprime_base12CompactM2",
        zero_seed_class_theorem: "zeroSeedClass_base12CompactM2_mod5",
        profile_excluded_seed_class_theorem: "base12CompactM2ProfileAt_mod5_excludedSeedClass",
        divisibility_iff_theorem: "templateValue_base12CompactM2_mod5_eq_zero_iff_seed_mod_eq_one",
    },
    MatchedControlSmokeProfileCertificateSpec {
        family_index: 6,
        middle_width: 1,
        modulus: 5,
        excluded_seed_class: 4,
        certificate_constant: "base14OffsetM1Mod5Certificate",
        coprime_theorem: "modFiveCoprime_base14OffsetM1",
        zero_seed_class_theorem: "zeroSeedClass_base14OffsetM1_mod5",
        profile_excluded_seed_class_theorem: "base14OffsetM1ProfileAt_mod5_excludedSeedClass",
        divisibility_iff_theorem: "templateValue_base14OffsetM1_mod5_eq_zero_iff_seed_mod_eq_four",
    },
    MatchedControlSmokeProfileCertificateSpec {
        family_index: 6,
        middle_width: 2,
        modulus: 5,
        excluded_seed_class: 0,
        certificate_constant: "base14OffsetM2Mod5Certificate",
        coprime_theorem: "modFiveCoprime_base14OffsetM2",
        zero_seed_class_theorem: "zeroSeedClass_base14OffsetM2_mod5",
        profile_excluded_seed_class_theorem: "base14OffsetM2ProfileAt_mod5_excludedSeedClass",
        divisibility_iff_theorem: "templateValue_base14OffsetM2_mod5_eq_zero_iff_seed_mod_eq_zero",
    },
    MatchedControlSmokeProfileCertificateSpec {
        family_index: 7,
        middle_width: 1,
        modulus: 7,
        excluded_seed_class: 4,
        certificate_constant: "base30WheelLikeM1Mod7Certificate",
        coprime_theorem: "modSevenCoprime_base30WheelLikeM1",
        zero_seed_class_theorem: "zeroSeedClass_base30WheelLikeM1_mod7",
        profile_excluded_seed_class_theorem: "base30WheelLikeM1ProfileAt_mod7_excludedSeedClass",
        divisibility_iff_theorem:
            "templateValue_base30WheelLikeM1_mod7_eq_zero_iff_seed_mod_eq_four",
    },
    MatchedControlSmokeProfileCertificateSpec {
        family_index: 7,
        middle_width: 2,
        modulus: 7,
        excluded_seed_class: 2,
        certificate_constant: "base30WheelLikeM2Mod7Certificate",
        coprime_theorem: "modSevenCoprime_base30WheelLikeM2",
        zero_seed_class_theorem: "zeroSeedClass_base30WheelLikeM2_mod7",
        profile_excluded_seed_class_theorem: "base30WheelLikeM2ProfileAt_mod7_excludedSeedClass",
        divisibility_iff_theorem:
            "templateValue_base30WheelLikeM2_mod7_eq_zero_iff_seed_mod_eq_two",
    },
    MatchedControlSmokeProfileCertificateSpec {
        family_index: 1,
        middle_width: 2,
        modulus: 3,
        excluded_seed_class: 1,
        certificate_constant: "base10ClassicM2Mod3Certificate",
        coprime_theorem: "modThreeCoprime_base10ClassicM2",
        zero_seed_class_theorem: "zeroSeedClass_base10ClassicM2_mod3",
        profile_excluded_seed_class_theorem: "base10ClassicM2ProfileAt_mod3_excludedSeedClass",
        divisibility_iff_theorem: "templateValue_base10ClassicM2_mod3_eq_zero_iff_seed_mod_eq_one",
    },
    MatchedControlSmokeProfileCertificateSpec {
        family_index: 3,
        middle_width: 2,
        modulus: 3,
        excluded_seed_class: 0,
        certificate_constant: "base10SymmetricM2Mod3Certificate",
        coprime_theorem: "modThreeCoprime_base10SymmetricM2",
        zero_seed_class_theorem: "zeroSeedClass_base10SymmetricM2_mod3",
        profile_excluded_seed_class_theorem: "base10SymmetricM2ProfileAt_mod3_excludedSeedClass",
        divisibility_iff_theorem:
            "templateValue_base10SymmetricM2_mod3_eq_zero_iff_seed_mod_eq_zero",
    },
];

pub const MAINTAINED_MATCHED_CONTROL_SMOKE_PAIR_CERTIFICATES:
    [MatchedControlSmokePairCertificateSpec; 12] = [
    MatchedControlSmokePairCertificateSpec {
        left_family_index: 2,
        right_family_index: 3,
        middle_width: 1,
        modulus: 11,
        left_excluded_seed_class: 0,
        right_excluded_seed_class: 10,
        lean_module: MATCHED_CONTROL_BASE10_SEPARATION_MODULE,
        zero_seed_class_ne_theorem: "zeroSeedClass_breathingM1_ne_symmetricM1_mod11",
        separation_theorem: MATCHED_CONTROL_BASE10_M1_BREATHING_SYMMETRIC_SEPARATION_THEOREM,
        forbidden_residue_set_theorem: Some(
            MATCHED_CONTROL_BASE10_M1_BREATHING_SYMMETRIC_FORBIDDEN_RESIDUES_THEOREM,
        ),
        equal_survivor_theorem:
            MATCHED_CONTROL_BASE10_M1_BREATHING_SYMMETRIC_EQUAL_SURVIVOR_THEOREM,
    },
    MatchedControlSmokePairCertificateSpec {
        left_family_index: 2,
        right_family_index: 3,
        middle_width: 2,
        modulus: 7,
        left_excluded_seed_class: 3,
        right_excluded_seed_class: 5,
        lean_module: MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE,
        zero_seed_class_ne_theorem: "zeroSeedClass_base10BreathingM2_ne_base10SymmetricM2_mod7",
        separation_theorem: MATCHED_CONTROL_BASE10_M2_SMOKE_SEPARATION_THEOREM,
        forbidden_residue_set_theorem: Some(
            MATCHED_CONTROL_BASE10_M2_SMOKE_FORBIDDEN_RESIDUES_THEOREM,
        ),
        equal_survivor_theorem: MATCHED_CONTROL_BASE10_M2_SMOKE_EQUAL_SURVIVOR_THEOREM,
    },
    MatchedControlSmokePairCertificateSpec {
        left_family_index: 1,
        right_family_index: 4,
        middle_width: 1,
        modulus: 11,
        left_excluded_seed_class: 8,
        right_excluded_seed_class: 2,
        lean_module: MATCHED_CONTROL_BASE10_SEPARATION_MODULE,
        zero_seed_class_ne_theorem: "zeroSeedClass_classicM1_ne_exclusiveM1_mod11",
        separation_theorem: MATCHED_CONTROL_BASE10_M1_CLASSIC_EXCLUSIVE_SEPARATION_THEOREM,
        forbidden_residue_set_theorem: Some(
            MATCHED_CONTROL_BASE10_M1_CLASSIC_EXCLUSIVE_FORBIDDEN_RESIDUES_THEOREM,
        ),
        equal_survivor_theorem: MATCHED_CONTROL_BASE10_M1_CLASSIC_EXCLUSIVE_EQUAL_SURVIVOR_THEOREM,
    },
    MatchedControlSmokePairCertificateSpec {
        left_family_index: 2,
        right_family_index: 4,
        middle_width: 1,
        modulus: 11,
        left_excluded_seed_class: 0,
        right_excluded_seed_class: 2,
        lean_module: MATCHED_CONTROL_BASE10_SEPARATION_MODULE,
        zero_seed_class_ne_theorem: "zeroSeedClass_breathingM1_ne_exclusiveM1_mod11",
        separation_theorem: MATCHED_CONTROL_BASE10_M1_BREATHING_EXCLUSIVE_SEPARATION_THEOREM,
        forbidden_residue_set_theorem: Some(
            MATCHED_CONTROL_BASE10_M1_BREATHING_EXCLUSIVE_FORBIDDEN_RESIDUES_THEOREM,
        ),
        equal_survivor_theorem:
            MATCHED_CONTROL_BASE10_M1_BREATHING_EXCLUSIVE_EQUAL_SURVIVOR_THEOREM,
    },
    MatchedControlSmokePairCertificateSpec {
        left_family_index: 2,
        right_family_index: 1,
        middle_width: 1,
        modulus: 11,
        left_excluded_seed_class: 0,
        right_excluded_seed_class: 8,
        lean_module: MATCHED_CONTROL_BASE10_SEPARATION_MODULE,
        zero_seed_class_ne_theorem: "zeroSeedClass_breathingM1_ne_classicM1_mod11",
        separation_theorem: MATCHED_CONTROL_BASE10_M1_BREATHING_CLASSIC_SEPARATION_THEOREM,
        forbidden_residue_set_theorem: Some(
            MATCHED_CONTROL_BASE10_M1_BREATHING_CLASSIC_FORBIDDEN_RESIDUES_THEOREM,
        ),
        equal_survivor_theorem: MATCHED_CONTROL_BASE10_M1_BREATHING_CLASSIC_EQUAL_SURVIVOR_THEOREM,
    },
    MatchedControlSmokePairCertificateSpec {
        left_family_index: 1,
        right_family_index: 4,
        middle_width: 2,
        modulus: 7,
        left_excluded_seed_class: 5,
        right_excluded_seed_class: 0,
        lean_module: MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE,
        zero_seed_class_ne_theorem: "zeroSeedClass_base10ClassicM2_ne_base10ExclusiveM2_mod7",
        separation_theorem: MATCHED_CONTROL_BASE10_CLASSIC_M2_SMOKE_SEPARATION_THEOREM,
        forbidden_residue_set_theorem: Some(
            MATCHED_CONTROL_BASE10_CLASSIC_M2_SMOKE_FORBIDDEN_RESIDUES_THEOREM,
        ),
        equal_survivor_theorem: MATCHED_CONTROL_BASE10_CLASSIC_M2_SMOKE_EQUAL_SURVIVOR_THEOREM,
    },
    MatchedControlSmokePairCertificateSpec {
        left_family_index: 2,
        right_family_index: 1,
        middle_width: 2,
        modulus: 7,
        left_excluded_seed_class: 3,
        right_excluded_seed_class: 5,
        lean_module: MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE,
        zero_seed_class_ne_theorem: "zeroSeedClass_base10BreathingM2_ne_base10ClassicM2_mod7",
        separation_theorem: MATCHED_CONTROL_BASE10_BREATHING_CLASSIC_M2_SMOKE_SEPARATION_THEOREM,
        forbidden_residue_set_theorem: Some(
            MATCHED_CONTROL_BASE10_BREATHING_CLASSIC_M2_SMOKE_FORBIDDEN_RESIDUES_THEOREM,
        ),
        equal_survivor_theorem:
            MATCHED_CONTROL_BASE10_BREATHING_CLASSIC_M2_SMOKE_EQUAL_SURVIVOR_THEOREM,
    },
    MatchedControlSmokePairCertificateSpec {
        left_family_index: 3,
        right_family_index: 4,
        middle_width: 2,
        modulus: 7,
        left_excluded_seed_class: 5,
        right_excluded_seed_class: 0,
        lean_module: MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE,
        zero_seed_class_ne_theorem: "zeroSeedClass_base10SymmetricM2_ne_base10ExclusiveM2_mod7",
        separation_theorem: MATCHED_CONTROL_BASE10_SYMMETRIC_EXCLUSIVE_M2_SMOKE_SEPARATION_THEOREM,
        forbidden_residue_set_theorem: Some(
            MATCHED_CONTROL_BASE10_SYMMETRIC_EXCLUSIVE_M2_SMOKE_FORBIDDEN_RESIDUES_THEOREM,
        ),
        equal_survivor_theorem:
            MATCHED_CONTROL_BASE10_SYMMETRIC_EXCLUSIVE_M2_SMOKE_EQUAL_SURVIVOR_THEOREM,
    },
    MatchedControlSmokePairCertificateSpec {
        left_family_index: 2,
        right_family_index: 4,
        middle_width: 2,
        modulus: 7,
        left_excluded_seed_class: 3,
        right_excluded_seed_class: 0,
        lean_module: MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE,
        zero_seed_class_ne_theorem: "zeroSeedClass_base10BreathingM2_ne_base10ExclusiveM2_mod7",
        separation_theorem: MATCHED_CONTROL_BASE10_BREATHING_EXCLUSIVE_M2_SMOKE_SEPARATION_THEOREM,
        forbidden_residue_set_theorem: Some(
            MATCHED_CONTROL_BASE10_BREATHING_EXCLUSIVE_M2_SMOKE_FORBIDDEN_RESIDUES_THEOREM,
        ),
        equal_survivor_theorem:
            MATCHED_CONTROL_BASE10_BREATHING_EXCLUSIVE_M2_SMOKE_EQUAL_SURVIVOR_THEOREM,
    },
    MatchedControlSmokePairCertificateSpec {
        left_family_index: 3,
        right_family_index: 1,
        middle_width: 2,
        modulus: 3,
        left_excluded_seed_class: 0,
        right_excluded_seed_class: 1,
        lean_module: MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE,
        zero_seed_class_ne_theorem: "zeroSeedClass_base10SymmetricM2_ne_base10ClassicM2_mod3",
        separation_theorem: MATCHED_CONTROL_BASE10_SYMMETRIC_CLASSIC_M2_SMOKE_SEPARATION_THEOREM,
        forbidden_residue_set_theorem: Some(
            MATCHED_CONTROL_BASE10_SYMMETRIC_CLASSIC_M2_SMOKE_FORBIDDEN_RESIDUES_THEOREM,
        ),
        equal_survivor_theorem:
            MATCHED_CONTROL_BASE10_SYMMETRIC_CLASSIC_M2_SMOKE_EQUAL_SURVIVOR_THEOREM,
    },
    MatchedControlSmokePairCertificateSpec {
        left_family_index: 3,
        right_family_index: 4,
        middle_width: 1,
        modulus: 11,
        left_excluded_seed_class: 10,
        right_excluded_seed_class: 2,
        lean_module: MATCHED_CONTROL_BASE10_SEPARATION_MODULE,
        zero_seed_class_ne_theorem: "zeroSeedClass_symmetricM1_ne_exclusiveM1_mod11",
        separation_theorem: MATCHED_CONTROL_BASE10_M1_SYMMETRIC_EXCLUSIVE_SEPARATION_THEOREM,
        forbidden_residue_set_theorem: Some(
            MATCHED_CONTROL_BASE10_M1_SYMMETRIC_EXCLUSIVE_FORBIDDEN_RESIDUES_THEOREM,
        ),
        equal_survivor_theorem:
            MATCHED_CONTROL_BASE10_M1_SYMMETRIC_EXCLUSIVE_EQUAL_SURVIVOR_THEOREM,
    },
    MatchedControlSmokePairCertificateSpec {
        left_family_index: 3,
        right_family_index: 1,
        middle_width: 1,
        modulus: 11,
        left_excluded_seed_class: 10,
        right_excluded_seed_class: 8,
        lean_module: MATCHED_CONTROL_BASE10_SEPARATION_MODULE,
        zero_seed_class_ne_theorem: "zeroSeedClass_symmetricM1_ne_classicM1_mod11",
        separation_theorem: MATCHED_CONTROL_BASE10_M1_SYMMETRIC_CLASSIC_SEPARATION_THEOREM,
        forbidden_residue_set_theorem: Some(
            MATCHED_CONTROL_BASE10_M1_SYMMETRIC_CLASSIC_FORBIDDEN_RESIDUES_THEOREM,
        ),
        equal_survivor_theorem: MATCHED_CONTROL_BASE10_M1_SYMMETRIC_CLASSIC_EQUAL_SURVIVOR_THEOREM,
    },
];

#[derive(Default)]
struct AggregateAccumulator {
    membrane_primes: usize,
    control_primes: usize,
    samples: usize,
    families: usize,
    positive_q: usize,
}

#[derive(Default)]
struct BatchFamilyAccumulator {
    family_label: String,
    family_code: String,
    material_change_count: usize,
    decision_change_count: usize,
    added_count: usize,
    removed_count: usize,
    max_abs_lift_delta: Option<f64>,
    max_abs_q_delta: Option<f64>,
}

impl MatchedControlAuditSeverityTally {
    fn observe(&mut self, severity: MatchedControlAuditSeverity) {
        match severity {
            MatchedControlAuditSeverity::Clear => self.clear += 1,
            MatchedControlAuditSeverity::Info => self.info += 1,
            MatchedControlAuditSeverity::Error => self.error += 1,
        }
    }
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

pub fn read_comparison_json_export(
    path: impl AsRef<Path>,
) -> Result<MatchedControlComparisonExportBundle, MatchedControlExportError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

pub fn build_export_bundle(
    reports: &[MatchedControlReport],
    summary: &MatchedControlSummary,
    settings: MatchedControlRunSettings,
) -> MatchedControlExportBundle {
    build_export_bundle_with_panel(reports, summary, settings, None)
}

pub fn build_export_bundle_with_panel(
    reports: &[MatchedControlReport],
    summary: &MatchedControlSummary,
    settings: MatchedControlRunSettings,
    panel_id: Option<&str>,
) -> MatchedControlExportBundle {
    MatchedControlExportBundle {
        export_version: MATCHED_CONTROL_EXPORT_VERSION,
        generated_at_utc: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
        panel_id: panel_id.map(str::to_string),
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
    write_json_export_with_panel(path, reports, summary, settings, None)
}

pub fn write_json_export_with_panel(
    path: impl AsRef<Path>,
    reports: &[MatchedControlReport],
    summary: &MatchedControlSummary,
    settings: MatchedControlRunSettings,
    panel_id: Option<&str>,
) -> Result<(), MatchedControlExportError> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(
        writer,
        &build_export_bundle_with_panel(reports, summary, settings, panel_id),
    )?;
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

pub fn build_matched_control_atlas_manifest(
    panel: MatchedControlPanel,
) -> MatchedControlAtlasManifest {
    let settings = panel.settings();
    let families = MAINTAINED_MATCHED_CONTROL_FAMILIES
        .iter()
        .flat_map(|family| {
            (settings.min_seed_len..=settings.max_seed_len)
                .map(move |middle_width| matched_control_atlas_family_row(family, middle_width))
        })
        .collect::<Vec<_>>();

    MatchedControlAtlasManifest {
        schema_version: MATCHED_CONTROL_ATLAS_SCHEMA_VERSION.to_string(),
        panel_id: panel.panel_id().to_string(),
        panel: panel.as_str().to_string(),
        settings,
        family_count: MAINTAINED_MATCHED_CONTROL_FAMILIES.len(),
        lane_count: families.len(),
        lean_lane_module: MATCHED_CONTROL_LEAN_LANE_MODULE.to_string(),
        claim_status: MatchedControlAtlasClaimStatus::NoDensityMechanismClaim,
        families,
    }
}

pub fn build_matched_control_smoke_profile_certificate_metadata(
) -> Vec<MatchedControlSmokeProfileCertificateMetadata> {
    MAINTAINED_MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATES
        .iter()
        .map(smoke_profile_certificate_metadata_from_spec)
        .collect()
}

pub fn build_matched_control_smoke_pair_certificate_metadata(
) -> Vec<MatchedControlSmokePairCertificateMetadata> {
    MAINTAINED_MATCHED_CONTROL_SMOKE_PAIR_CERTIFICATES
        .iter()
        .map(smoke_pair_certificate_metadata_from_spec)
        .collect()
}

pub fn matched_control_smoke_pair_certificate_metadata_for(
    left_family_code: &str,
    right_family_code: &str,
) -> Option<MatchedControlSmokePairCertificateMetadata> {
    build_matched_control_smoke_pair_certificate_metadata()
        .into_iter()
        .find(|row| {
            row.left_family_code == left_family_code && row.right_family_code == right_family_code
        })
}

fn smoke_profile_certificate_metadata_for_family_code_modulus(
    family_code: &str,
    middle_width: usize,
    modulus: u32,
) -> Option<MatchedControlSmokeProfileCertificateMetadata> {
    build_matched_control_smoke_profile_certificate_metadata()
        .into_iter()
        .find(|row| {
            row.family_code == family_code
                && row.middle_width == middle_width
                && row.modulus == modulus
        })
}

pub fn render_matched_control_smoke_profile_certificate_lean_candidates() -> String {
    let mut out = String::new();
    out.push_str("/-!\n");
    out.push_str("Lean smoke-profile certificate candidates generated from Rust metadata.\n");
    out.push_str("These declarations assume the lane aliases, lookup theorems, coprime proofs,\n");
    out.push_str("and zero-seed-class theorems already exist in the target Lean module.\n");
    out.push_str("-/\n\n");

    for row in build_matched_control_smoke_profile_certificate_metadata() {
        let _ = writeln!(
            out,
            "def {} : SmokeLaneProfileCertificate :=",
            row.certificate_constant
        );
        let _ = writeln!(out, "  SmokeLaneProfileCertificate.ofZeroSeedClass");
        let _ = writeln!(
            out,
            "    {} {} {} {} {}",
            lean_string_literal(&row.family_code),
            row.middle_width,
            row.lean_lane_constant,
            row.modulus,
            row.excluded_seed_class
        );
        let _ = writeln!(out, "    {}", row.coprime_theorem);
        let _ = writeln!(out, "    {}", row.lean_lookup_theorem);
        let _ = writeln!(out, "    {}", row.zero_seed_class_theorem);
        let _ = writeln!(out, "    (by native_decide)");
        out.push('\n');
    }

    out
}

pub fn render_matched_control_smoke_profile_certificate_lean_module() -> String {
    let mut out = String::new();
    out.push_str("import ");
    out.push_str(MATCHED_CONTROL_SMOKE_PROFILE_MODULE);
    out.push_str("\n\nnamespace ");
    out.push_str(MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE);
    out.push_str("\n\nopen PrimeArithmetic.Structure\n");
    out.push_str("open PrimeArithmetic.Density.CanonicalSmokeLaneProfiles\n\n");
    out.push_str("/-!\n");
    out.push_str("Generated smoke-profile certificate declarations.\n\n");
    out.push_str("Generated by:\n");
    out.push_str("`cargo run --bin export_matched_control_smoke_profile_certificates -- --format lean-module --out lean-proofs/PrimeArithmetic/Generated/MatchedControlSmokeProfileCertificates.lean`\n\n");
    out.push_str("These are exact local residue-profile facts, not density claims.\n");
    out.push_str("-/\n\n");

    write_smoke_profile_certificate_declarations(&mut out);
    write_smoke_profile_certificate_profile_theorems(&mut out);
    write_smoke_profile_certificate_divisibility_theorems(&mut out);
    write_smoke_profile_pair_separation_theorems(&mut out);

    out.push_str("end ");
    out.push_str(MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE);
    out.push('\n');

    out
}

pub fn render_matched_control_smoke_profile_certificate_lean_checks() -> String {
    let mut out = String::new();
    out.push_str("import ");
    out.push_str(MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE);
    out.push('\n');
    out.push_str("import ");
    out.push_str(MATCHED_CONTROL_BASE10_SEPARATION_MODULE);
    out.push_str("\n\n/-!\n");
    out.push_str("Lean smoke-profile certificate existence checks generated from Rust metadata.\n");
    out.push_str("This file is intended for drift checks; it should elaborate if every metadata\n");
    out.push_str("row still points at an existing maintained Lean declaration.\n");
    out.push_str("-/\n\n");

    for row in build_matched_control_smoke_profile_certificate_metadata() {
        let support_names = [
            row.lean_lane_constant.as_str(),
            row.lean_lookup_theorem.as_str(),
            row.coprime_theorem.as_str(),
            row.zero_seed_class_theorem.as_str(),
        ];
        let certificate_names = [
            row.certificate_constant.as_str(),
            row.profile_excluded_seed_class_theorem.as_str(),
            row.divisibility_iff_theorem.as_str(),
        ];

        let _ = writeln!(out, "-- {}", row.family_code);
        for name in support_names {
            let _ = writeln!(
                out,
                "#check {}",
                qualified_lean_name(MATCHED_CONTROL_SMOKE_PROFILE_MODULE, name)
            );
        }
        for name in certificate_names {
            let _ = writeln!(
                out,
                "#check {}",
                qualified_lean_name(MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE, name)
            );
        }
        out.push('\n');
    }

    for row in build_matched_control_smoke_pair_certificate_metadata() {
        let _ = writeln!(
            out,
            "-- pair {} vs {}",
            row.left_family_code, row.right_family_code
        );
        let _ = writeln!(out, "#check {}", row.zero_seed_class_ne_theorem_qualified);
        let _ = writeln!(out, "#check {}", row.separation_theorem_qualified);
        if let Some(theorem) = &row.forbidden_residue_set_theorem_qualified {
            let _ = writeln!(out, "#check {theorem}");
        }
        let _ = writeln!(out, "#check {}", row.equal_survivor_theorem_qualified);
        out.push('\n');
    }

    out
}

pub fn render_matched_control_smoke_profile_certificate_lean_silent_checks() -> String {
    let profile_rows = build_matched_control_smoke_profile_certificate_metadata();
    let pair_rows = build_matched_control_smoke_pair_certificate_metadata();
    render_matched_control_smoke_profile_certificate_lean_silent_checks_for_rows(
        &profile_rows,
        &pair_rows,
    )
}

pub struct MatchedControlSmokeProfileCertificateLeanCheckShard {
    pub module_name: String,
    pub file_name: String,
    pub contents: String,
}

pub struct MatchedControlSmokeProfileCertificateLeanCheckBundle {
    pub umbrella_contents: String,
    pub shards: Vec<MatchedControlSmokeProfileCertificateLeanCheckShard>,
}

pub fn render_matched_control_smoke_profile_certificate_lean_silent_check_shards(
    module_prefix: &str,
    umbrella_stem: &str,
    profile_shard_size: usize,
) -> Result<MatchedControlSmokeProfileCertificateLeanCheckBundle, String> {
    if profile_shard_size == 0 {
        return Err("profile shard size must be positive".to_string());
    }
    if umbrella_stem.is_empty() {
        return Err("umbrella module stem must not be empty".to_string());
    }

    let profile_rows = build_matched_control_smoke_profile_certificate_metadata();
    let pair_rows = build_matched_control_smoke_pair_certificate_metadata();
    let mut shards = Vec::new();

    for (index, chunk) in profile_rows.chunks(profile_shard_size).enumerate() {
        let file_stem = format!("{umbrella_stem}Shard{:02}", index + 1);
        let module_name = format!("{module_prefix}.{file_stem}");
        shards.push(MatchedControlSmokeProfileCertificateLeanCheckShard {
            module_name,
            file_name: format!("{file_stem}.lean"),
            contents: render_matched_control_smoke_profile_certificate_lean_silent_checks_for_rows(
                chunk,
                &[],
            ),
        });
    }

    if !pair_rows.is_empty() {
        let file_stem = format!("{umbrella_stem}Shard{:02}", shards.len() + 1);
        let module_name = format!("{module_prefix}.{file_stem}");
        shards.push(MatchedControlSmokeProfileCertificateLeanCheckShard {
            module_name,
            file_name: format!("{file_stem}.lean"),
            contents: render_matched_control_smoke_profile_certificate_lean_silent_checks_for_rows(
                &[],
                &pair_rows,
            ),
        });
    }

    let mut umbrella_contents = String::new();
    for shard in &shards {
        let _ = writeln!(umbrella_contents, "import {}", shard.module_name);
    }
    umbrella_contents.push_str("\n/-!\n");
    umbrella_contents.push_str(
        "Silent Lean smoke-profile certificate check umbrella generated from Rust metadata.\n",
    );
    umbrella_contents.push_str(
        "Each imported shard should elaborate if its certificate and pair theorem links resolve.\n",
    );
    umbrella_contents.push_str("-/\n");

    Ok(MatchedControlSmokeProfileCertificateLeanCheckBundle {
        umbrella_contents,
        shards,
    })
}

fn render_matched_control_smoke_profile_certificate_lean_silent_checks_for_rows(
    profile_rows: &[MatchedControlSmokeProfileCertificateMetadata],
    pair_rows: &[MatchedControlSmokePairCertificateMetadata],
) -> String {
    let mut out = String::new();
    out.push_str("import ");
    out.push_str(MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE);
    out.push('\n');
    out.push_str("import ");
    out.push_str(MATCHED_CONTROL_BASE10_SEPARATION_MODULE);
    out.push_str("\n\n/-!\n");
    out.push_str(
        "Silent Lean smoke-profile certificate existence checks generated from Rust metadata.\n",
    );
    out.push_str(
        "This file is intended for CI/drift checks; it should elaborate if every metadata\n",
    );
    out.push_str("row still points at an existing maintained Lean declaration.\n");
    out.push_str("-/\n\n");

    for row in profile_rows {
        let support_names = [
            row.lean_lane_constant.as_str(),
            row.lean_lookup_theorem.as_str(),
            row.coprime_theorem.as_str(),
            row.zero_seed_class_theorem.as_str(),
        ];
        let certificate_names = [
            row.certificate_constant.as_str(),
            row.profile_excluded_seed_class_theorem.as_str(),
            row.divisibility_iff_theorem.as_str(),
        ];

        let _ = writeln!(out, "-- {}", row.family_code);
        for name in support_names {
            write_silent_lean_declaration_check(
                &mut out,
                &qualified_lean_name(MATCHED_CONTROL_SMOKE_PROFILE_MODULE, name),
            );
        }
        for name in certificate_names {
            write_silent_lean_declaration_check(
                &mut out,
                &qualified_lean_name(MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE, name),
            );
        }
        out.push('\n');
    }

    for row in pair_rows {
        let _ = writeln!(
            out,
            "-- pair {} vs {}",
            row.left_family_code, row.right_family_code
        );
        write_silent_lean_declaration_check(&mut out, &row.zero_seed_class_ne_theorem_qualified);
        write_silent_lean_declaration_check(&mut out, &row.separation_theorem_qualified);
        if let Some(theorem) = &row.forbidden_residue_set_theorem_qualified {
            write_silent_lean_declaration_check(&mut out, theorem);
        }
        write_silent_lean_declaration_check(&mut out, &row.equal_survivor_theorem_qualified);
        out.push('\n');
    }

    while out.ends_with("\n\n") {
        out.pop();
    }
    out
}

fn write_silent_lean_declaration_check(out: &mut String, name: &str) {
    let _ = writeln!(out, "example : True := by");
    let _ = writeln!(out, "  have _ := {name}");
    let _ = writeln!(out, "  trivial");
}

fn write_smoke_profile_certificate_declarations(out: &mut String) {
    for row in build_matched_control_smoke_profile_certificate_metadata() {
        let _ = writeln!(
            out,
            "def {} : SmokeLaneProfileCertificate :=",
            row.certificate_constant
        );
        let _ = writeln!(out, "  SmokeLaneProfileCertificate.ofZeroSeedClass");
        let _ = writeln!(
            out,
            "    {} {} {} {} {}",
            lean_string_literal(&row.family_code),
            row.middle_width,
            row.lean_lane_constant,
            row.modulus,
            row.excluded_seed_class
        );
        let _ = writeln!(out, "    {}", row.coprime_theorem);
        let _ = writeln!(out, "    {}", row.lean_lookup_theorem);
        let _ = writeln!(out, "    {}", row.zero_seed_class_theorem);
        let _ = writeln!(out, "    (by native_decide)");
        out.push('\n');
    }
}

fn write_smoke_profile_certificate_profile_theorems(out: &mut String) {
    for row in build_matched_control_smoke_profile_certificate_metadata() {
        let _ = writeln!(out, "theorem {} :", row.profile_excluded_seed_class_theorem);
        let _ = writeln!(
            out,
            "    ({}.residueProfileAt {} {}).excludedSeedClass {} = {} := by",
            row.lean_lane_constant,
            row.modulus,
            row.coprime_theorem,
            row.modulus,
            row.excluded_seed_class
        );
        let _ = writeln!(out, "  simpa [SmokeLaneProfileCertificate.profile,");
        let _ = writeln!(
            out,
            "    BoundedKFamilyLaneProfileCertificate.profile, {}] using",
            row.certificate_constant
        );
        let _ = writeln!(
            out,
            "    {}.profile_excludedSeedClass",
            row.certificate_constant
        );
        out.push('\n');
    }
}

fn write_smoke_profile_certificate_divisibility_theorems(out: &mut String) {
    for row in build_matched_control_smoke_profile_certificate_metadata() {
        let config_name = smoke_profile_lane_config_name(&row.lean_lane_constant);
        let _ = writeln!(out, "theorem {}", row.divisibility_iff_theorem);
        let _ = writeln!(out, "    (seed : ℕ) :");
        let _ = writeln!(
            out,
            "    templateValue {} seed % {} = 0 ↔ seed % {} = {} := by",
            config_name, row.modulus, row.modulus, row.excluded_seed_class
        );
        let _ = writeln!(
            out,
            "  simpa [{}, {}] using",
            config_name, row.certificate_constant
        );
        let _ = writeln!(
            out,
            "    {}.templateValue_mod_eq_zero_iff_seed_mod_eq seed",
            row.certificate_constant
        );
        out.push('\n');
    }
}

fn write_smoke_profile_pair_separation_theorems(out: &mut String) {
    for pair in build_matched_control_smoke_pair_certificate_metadata()
        .into_iter()
        .filter(|pair| pair.lean_module == MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE)
    {
        let left = smoke_profile_certificate_metadata_for_family_code_modulus(
            &pair.left_family_code,
            pair.middle_width,
            pair.modulus,
        )
        .expect("generated pair certificate should have left profile metadata");
        let right = smoke_profile_certificate_metadata_for_family_code_modulus(
            &pair.right_family_code,
            pair.middle_width,
            pair.modulus,
        )
        .expect("generated pair certificate should have right profile metadata");

        assert_eq!(
            left.excluded_seed_class, pair.left_excluded_seed_class,
            "left pair/profile excluded seed class mismatch for {}",
            pair.left_family_code
        );
        assert_eq!(
            right.excluded_seed_class, pair.right_excluded_seed_class,
            "right pair/profile excluded seed class mismatch for {}",
            pair.right_family_code
        );

        let _ = writeln!(out, "theorem {} :", pair.zero_seed_class_ne_theorem);
        let _ = writeln!(
            out,
            "    {}.zeroSeedClassAt {} {} ≠",
            left.lean_lane_constant, pair.modulus, left.coprime_theorem
        );
        let _ = writeln!(
            out,
            "      {}.zeroSeedClassAt {} {} := by",
            right.lean_lane_constant, pair.modulus, right.coprime_theorem
        );
        let _ = writeln!(
            out,
            "  rw [{}, {}]",
            left.zero_seed_class_theorem, right.zero_seed_class_theorem
        );
        out.push_str("  native_decide\n\n");

        let _ = writeln!(out, "theorem {} :", pair.separation_theorem);
        let _ = writeln!(
            out,
            "    {}.forbiddenSeedMaskAt {} ≠ {}.forbiddenSeedMaskAt {} := by",
            left.lean_lane_constant, pair.modulus, right.lean_lane_constant, pair.modulus
        );
        out.push_str("  exact BoundedKFamilyLane.forbiddenSeedMaskAt_ne_of_zeroSeedClassAt_ne\n");
        let _ = writeln!(
            out,
            "    {} {}",
            left.lean_lane_constant, right.lean_lane_constant
        );
        let _ = writeln!(
            out,
            "    {} {}",
            left.coprime_theorem, right.coprime_theorem
        );
        let _ = writeln!(out, "    {}\n", pair.zero_seed_class_ne_theorem);

        if let Some(theorem) = &pair.forbidden_residue_set_theorem {
            let _ = writeln!(out, "theorem {theorem} :");
            let _ = writeln!(
                out,
                "    {}.forbiddenResiduesAt {} {} ≠",
                left.lean_lane_constant, pair.modulus, left.coprime_theorem
            );
            let _ = writeln!(
                out,
                "      {}.forbiddenResiduesAt {} {} := by",
                right.lean_lane_constant, pair.modulus, right.coprime_theorem
            );
            out.push_str(
                "  exact BoundedKFamilyLane.forbiddenResiduesAt_ne_of_zeroSeedClassAt_ne\n",
            );
            let _ = writeln!(
                out,
                "    {} {}",
                left.lean_lane_constant, right.lean_lane_constant
            );
            let _ = writeln!(
                out,
                "    {} {}",
                left.coprime_theorem, right.coprime_theorem
            );
            let _ = writeln!(out, "    {}\n", pair.zero_seed_class_ne_theorem);
        }

        let _ = writeln!(out, "theorem {} :", pair.equal_survivor_theorem);
        let _ = writeln!(
            out,
            "    ({}.survivorResiduesAt {} {}).card =",
            left.lean_lane_constant, pair.modulus, left.coprime_theorem
        );
        let _ = writeln!(
            out,
            "      ({}.survivorResiduesAt {} {}).card := by",
            right.lean_lane_constant, pair.modulus, right.coprime_theorem
        );
        out.push_str("  exact BoundedKFamilyLane.survivorResiduesAt_card_eq\n");
        let _ = writeln!(
            out,
            "    {} {}",
            left.lean_lane_constant, right.lean_lane_constant
        );
        let _ = writeln!(
            out,
            "    {} {}\n",
            left.coprime_theorem, right.coprime_theorem
        );
    }
}

fn smoke_profile_lane_config_name(lane_constant: &str) -> String {
    let prefix = lane_constant.strip_suffix("Lane").unwrap_or(lane_constant);
    format!("{prefix}Config")
}

pub fn write_matched_control_atlas_manifest_json(
    path: impl AsRef<Path>,
    panel: MatchedControlPanel,
) -> Result<(), MatchedControlExportError> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &build_matched_control_atlas_manifest(panel))?;
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
    let mut compared_families = Vec::new();
    let mut added_families = Vec::new();
    let mut removed_families = Vec::new();
    let mut families_compared = 0usize;

    for (&family_code, before_row) in &before_by_code {
        match after_by_code.get(family_code) {
            Some(after_row) => {
                families_compared += 1;
                compared_families.push(snapshot_from_row(after_row));

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
        before_panel_id: before.panel_id.clone(),
        after_panel_id: after.panel_id.clone(),
        families_compared,
        compared_families,
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
    let conditions = MatchedControlComparisonAuditConditions {
        residual_criterion_changed: audit_condition(
            residual_criterion_changed,
            MatchedControlAuditSeverity::Error,
            MatchedControlAuditSeverity::Clear,
        ),
        material_family_change: audit_condition(
            material_family_change_count > 0,
            MatchedControlAuditSeverity::Error,
            MatchedControlAuditSeverity::Clear,
        ),
        sampling_plan_drift: audit_condition(
            sampling_plan_changed,
            if policy.flag_sampling_plan_drift {
                MatchedControlAuditSeverity::Error
            } else {
                MatchedControlAuditSeverity::Info
            },
            MatchedControlAuditSeverity::Clear,
        ),
        added_families: audit_condition(
            added_family_count > 0,
            if policy.flag_added_families {
                MatchedControlAuditSeverity::Error
            } else {
                MatchedControlAuditSeverity::Info
            },
            MatchedControlAuditSeverity::Clear,
        ),
        removed_families: audit_condition(
            removed_family_count > 0,
            if policy.flag_removed_families {
                MatchedControlAuditSeverity::Error
            } else {
                MatchedControlAuditSeverity::Info
            },
            MatchedControlAuditSeverity::Clear,
        ),
    };
    let reasons = error_reasons_from_conditions(conditions);

    MatchedControlComparisonAudit {
        flagged: conditions_have_error(conditions),
        sampling_plan_changed,
        residual_criterion_changed,
        material_family_change_count,
        added_family_count,
        removed_family_count,
        reasons,
        conditions,
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

pub fn summarize_comparison_batch(
    inputs: &[MatchedControlComparisonBatchInput],
) -> Result<MatchedControlComparisonBatchSummary, MatchedControlBatchError> {
    let first = inputs.first().ok_or(MatchedControlBatchError::EmptyInput)?;
    let expected_settings = first.bundle.compare_settings;
    let expected_policy = first.bundle.compare_policy;
    let expected_panel_id = comparison_panel_identity(&first.bundle, 0)?;
    let mut condition_tallies = MatchedControlComparisonBatchConditionTallies::default();
    let mut run_rows = Vec::with_capacity(inputs.len());
    let mut family_acc: BTreeMap<String, BatchFamilyAccumulator> = BTreeMap::new();
    let mut flagged_run_count = 0usize;
    let mut residual_criterion_flip_count = 0usize;

    for (index, input) in inputs.iter().enumerate() {
        let bundle = &input.bundle;
        if bundle.compare_settings != expected_settings {
            return Err(MatchedControlBatchError::CompareSettingsMismatch { index });
        }
        if bundle.compare_policy != expected_policy {
            return Err(MatchedControlBatchError::ComparePolicyMismatch { index });
        }
        if comparison_panel_identity(bundle, index)? != expected_panel_id {
            return Err(MatchedControlBatchError::PanelMismatchAcrossBatch { index });
        }
        if bundle.comparison.families_compared > 0 && bundle.comparison.compared_families.is_empty()
        {
            return Err(MatchedControlBatchError::MissingComparedFamilySnapshots { index });
        }

        observe_condition_tallies(&mut condition_tallies, bundle.audit.conditions);

        flagged_run_count += usize::from(bundle.audit.flagged);
        residual_criterion_flip_count += usize::from(bundle.comparison.residual_criterion_changed);

        for family in &bundle.comparison.compared_families {
            record_family_snapshot(&mut family_acc, family);
        }
        for family in &bundle.comparison.added_families {
            record_family_snapshot(&mut family_acc, family).added_count += 1;
        }
        for family in &bundle.comparison.removed_families {
            record_family_snapshot(&mut family_acc, family).removed_count += 1;
        }
        for delta in &bundle.comparison.materially_changed_families {
            let acc = record_family_delta(&mut family_acc, delta);
            acc.material_change_count += 1;
            acc.decision_change_count += usize::from(delta.decision_changed);
            update_max_abs(&mut acc.max_abs_lift_delta, delta.lift_delta);
            update_max_abs(&mut acc.max_abs_q_delta, delta.q_delta);
        }

        run_rows.push(MatchedControlComparisonBatchRunRow {
            source_path: input.source_path.clone(),
            generated_at_utc: bundle.generated_at_utc.clone(),
            before_path: bundle.before_path.clone(),
            after_path: bundle.after_path.clone(),
            before_panel_id: bundle.comparison.before_panel_id.clone(),
            after_panel_id: bundle.comparison.after_panel_id.clone(),
            flagged: bundle.audit.flagged,
            residual_criterion_changed: bundle.comparison.residual_criterion_changed,
            material_family_change_count: bundle.audit.material_family_change_count,
            added_family_count: bundle.audit.added_family_count,
            removed_family_count: bundle.audit.removed_family_count,
            residual_criterion_changed_severity: bundle
                .audit
                .conditions
                .residual_criterion_changed
                .severity,
            material_family_change_severity: bundle
                .audit
                .conditions
                .material_family_change
                .severity,
            sampling_plan_drift_severity: bundle.audit.conditions.sampling_plan_drift.severity,
            added_families_severity: bundle.audit.conditions.added_families.severity,
            removed_families_severity: bundle.audit.conditions.removed_families.severity,
        });
    }

    let family_rows = family_acc
        .into_values()
        .map(|acc| {
            let status = if acc.material_change_count == 0 {
                MatchedControlBatchFamilyStatus::Stable
            } else {
                MatchedControlBatchFamilyStatus::Drifting
            };
            MatchedControlComparisonBatchFamilyRow {
                family_label: acc.family_label,
                family_code: acc.family_code,
                status,
                material_change_count: acc.material_change_count,
                decision_change_count: acc.decision_change_count,
                added_count: acc.added_count,
                removed_count: acc.removed_count,
                max_abs_lift_delta: acc.max_abs_lift_delta,
                max_abs_q_delta: acc.max_abs_q_delta,
            }
        })
        .collect::<Vec<_>>();
    let stable_family_count = family_rows
        .iter()
        .filter(|row| row.status == MatchedControlBatchFamilyStatus::Stable)
        .count();
    let drifting_family_count = family_rows.len() - stable_family_count;

    Ok(MatchedControlComparisonBatchSummary {
        export_version: MATCHED_CONTROL_COMPARISON_BATCH_EXPORT_VERSION,
        generated_at_utc: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
        panel_id: expected_panel_id,
        compare_settings: expected_settings,
        compare_policy: expected_policy,
        run_count: inputs.len(),
        flagged_run_count,
        residual_criterion_flip_count,
        condition_tallies,
        stable_family_count,
        drifting_family_count,
        run_rows,
        family_rows,
    })
}

pub fn format_p_like(value: f64) -> String {
    if value < 0.001 {
        format!("{value:.1e}")
    } else {
        format!("{value:.3}")
    }
}

fn matched_control_atlas_family_row(
    family: &MatchedControlFamily,
    middle_width: usize,
) -> MatchedControlAtlasFamilyRow {
    let lane_constant = matched_control_lean_lane_name(family, middle_width);
    let proof_certificate = matched_control_atlas_proof_certificate(family, middle_width);
    let proof_status = proof_certificate
        .as_ref()
        .map(|certificate| {
            if certificate.certificate_constant.is_some() {
                MatchedControlAtlasProofStatus::ExactResidueProfile
            } else {
                MatchedControlAtlasProofStatus::ExactSeedClassSeparation
            }
        })
        .unwrap_or(MatchedControlAtlasProofStatus::LaneGeneratedOnly);

    MatchedControlAtlasFamilyRow {
        family_label: family.label.to_string(),
        family_code: family.code(middle_width),
        base: family.base,
        outer: family.outer,
        inner: family.inner,
        k_outer: family.k_outer,
        k_inner: family.k_inner,
        middle_width,
        lean: matched_control_atlas_lean_link(&lane_constant),
        proof_status,
        proof_certificate,
    }
}

fn smoke_profile_certificate_metadata_from_spec(
    spec: &MatchedControlSmokeProfileCertificateSpec,
) -> MatchedControlSmokeProfileCertificateMetadata {
    let family = MAINTAINED_MATCHED_CONTROL_FAMILIES[spec.family_index];
    let lean_lane_constant = matched_control_lean_lane_name(&family, spec.middle_width);

    MatchedControlSmokeProfileCertificateMetadata {
        family_label: family.label.to_string(),
        family_code: family.code(spec.middle_width),
        base: family.base,
        outer: family.outer,
        inner: family.inner,
        k_outer: family.k_outer,
        k_inner: family.k_inner,
        middle_width: spec.middle_width,
        lean_module: MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE.to_string(),
        lean_lane_constant: lean_lane_constant.clone(),
        lean_lookup_theorem: format!("{lean_lane_constant}_lookup"),
        certificate_constant: spec.certificate_constant.to_string(),
        modulus: spec.modulus,
        excluded_seed_class: spec.excluded_seed_class,
        coprime_theorem: spec.coprime_theorem.to_string(),
        zero_seed_class_theorem: spec.zero_seed_class_theorem.to_string(),
        profile_excluded_seed_class_theorem: spec.profile_excluded_seed_class_theorem.to_string(),
        divisibility_iff_theorem: spec.divisibility_iff_theorem.to_string(),
    }
}

fn smoke_pair_certificate_metadata_from_spec(
    spec: &MatchedControlSmokePairCertificateSpec,
) -> MatchedControlSmokePairCertificateMetadata {
    let left_family = MAINTAINED_MATCHED_CONTROL_FAMILIES[spec.left_family_index];
    let right_family = MAINTAINED_MATCHED_CONTROL_FAMILIES[spec.right_family_index];
    let left_lean_lane_constant = matched_control_lean_lane_name(&left_family, spec.middle_width);
    let right_lean_lane_constant = matched_control_lean_lane_name(&right_family, spec.middle_width);

    MatchedControlSmokePairCertificateMetadata {
        left_family_label: left_family.label.to_string(),
        right_family_label: right_family.label.to_string(),
        left_family_code: left_family.code(spec.middle_width),
        right_family_code: right_family.code(spec.middle_width),
        left_lean_lane_constant,
        right_lean_lane_constant,
        middle_width: spec.middle_width,
        modulus: spec.modulus,
        left_excluded_seed_class: spec.left_excluded_seed_class,
        right_excluded_seed_class: spec.right_excluded_seed_class,
        lean_module: spec.lean_module.to_string(),
        zero_seed_class_ne_theorem: spec.zero_seed_class_ne_theorem.to_string(),
        zero_seed_class_ne_theorem_qualified: qualified_lean_name(
            spec.lean_module,
            spec.zero_seed_class_ne_theorem,
        ),
        separation_theorem: spec.separation_theorem.to_string(),
        separation_theorem_qualified: qualified_lean_name(
            spec.lean_module,
            spec.separation_theorem,
        ),
        forbidden_residue_set_theorem: spec.forbidden_residue_set_theorem.map(str::to_string),
        forbidden_residue_set_theorem_qualified: spec
            .forbidden_residue_set_theorem
            .map(|theorem| qualified_lean_name(spec.lean_module, theorem)),
        equal_survivor_theorem: spec.equal_survivor_theorem.to_string(),
        equal_survivor_theorem_qualified: qualified_lean_name(
            spec.lean_module,
            spec.equal_survivor_theorem,
        ),
    }
}

fn smoke_profile_certificate_metadata_for(
    family: &MatchedControlFamily,
    middle_width: usize,
) -> Option<MatchedControlSmokeProfileCertificateMetadata> {
    MAINTAINED_MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATES
        .iter()
        .find(|spec| {
            let spec_family = MAINTAINED_MATCHED_CONTROL_FAMILIES[spec.family_index];
            spec.middle_width == middle_width
                && spec_family.base == family.base
                && spec_family.outer == family.outer
                && spec_family.inner == family.inner
                && spec_family.k_outer == family.k_outer
                && spec_family.k_inner == family.k_inner
        })
        .map(smoke_profile_certificate_metadata_from_spec)
}

fn matched_control_atlas_lean_link(lane_constant: &str) -> MatchedControlAtlasLeanLink {
    let lookup_theorem = format!("{lane_constant}_lookup");
    MatchedControlAtlasLeanLink {
        module: MATCHED_CONTROL_LEAN_LANE_MODULE.to_string(),
        lane_constant: lane_constant.to_string(),
        lane_constant_qualified: qualified_lean_name(
            MATCHED_CONTROL_LEAN_LANE_MODULE,
            lane_constant,
        ),
        lookup_theorem: lookup_theorem.clone(),
        lookup_theorem_qualified: qualified_lean_name(
            MATCHED_CONTROL_LEAN_LANE_MODULE,
            &lookup_theorem,
        ),
    }
}

fn matched_control_atlas_proof_certificate(
    family: &MatchedControlFamily,
    middle_width: usize,
) -> Option<MatchedControlAtlasProofCertificate> {
    if let Some(metadata) = smoke_profile_certificate_metadata_for(family, middle_width) {
        return Some(profile_certificate_from_metadata(&metadata));
    }

    match (
        family.base,
        family.outer,
        family.inner,
        family.k_outer,
        family.k_inner,
        middle_width,
    ) {
        (10, 3, 3, 0, 1, 1) => Some(seed_class_separation(
            11,
            0,
            "breathingM1ProfileAt_mod11_excludedSeedClass",
            "templateValue_breathingM1_mod11_eq_zero_iff_seed_mod_eq_zero",
            "forbiddenSeedMask_breathingM1_ne_symmetricM1_mod11",
        )),
        (10, 3, 3, 1, 1, 1) => Some(seed_class_separation(
            11,
            10,
            "symmetricM1ProfileAt_mod11_excludedSeedClass",
            "templateValue_symmetricM1_mod11_eq_zero_iff_seed_mod_eq_ten",
            "forbiddenSeedMask_breathingM1_ne_symmetricM1_mod11",
        )),
        (10, 3, 7, 0, 0, 1) => Some(seed_class_separation(
            11,
            8,
            "classicM1ProfileAt_mod11_excludedSeedClass",
            "templateValue_classicM1_mod11_eq_zero_iff_seed_mod_eq_eight",
            "forbiddenSeedMask_classicM1_ne_exclusiveM1_mod11",
        )),
        (10, 3, 7, 1, 1, 1) => Some(seed_class_separation(
            11,
            2,
            "exclusiveM1ProfileAt_mod11_excludedSeedClass",
            "templateValue_exclusiveM1_mod11_eq_zero_iff_seed_mod_eq_two",
            "forbiddenSeedMask_classicM1_ne_exclusiveM1_mod11",
        )),
        _ => None,
    }
}

fn profile_certificate_from_metadata(
    metadata: &MatchedControlSmokeProfileCertificateMetadata,
) -> MatchedControlAtlasProofCertificate {
    MatchedControlAtlasProofCertificate {
        module: MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE.to_string(),
        certificate_constant: Some(metadata.certificate_constant.clone()),
        modulus: metadata.modulus,
        excluded_seed_class: metadata.excluded_seed_class,
        excluded_seed_class_theorem: qualified_lean_name(
            MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE,
            &metadata.profile_excluded_seed_class_theorem,
        ),
        divisibility_iff_theorem: qualified_lean_name(
            MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE,
            &metadata.divisibility_iff_theorem,
        ),
        separation_theorem: smoke_profile_pair_separation_theorem_for(metadata),
    }
}

fn smoke_profile_pair_separation_theorem_for(
    metadata: &MatchedControlSmokeProfileCertificateMetadata,
) -> Option<String> {
    match metadata.family_code.as_str() {
        "B10 ( 3, 3) k=(0,1) M=2" | "B10 ( 3, 3) k=(1,1) M=2" => Some(qualified_lean_name(
            MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE,
            MATCHED_CONTROL_BASE10_M2_SMOKE_SEPARATION_THEOREM,
        )),
        "B10 ( 3, 7) k=(0,0) M=2" | "B10 ( 3, 7) k=(1,1) M=2" => Some(qualified_lean_name(
            MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE,
            MATCHED_CONTROL_BASE10_CLASSIC_M2_SMOKE_SEPARATION_THEOREM,
        )),
        _ => None,
    }
}

fn seed_class_separation(
    modulus: u32,
    excluded_seed_class: u32,
    excluded_seed_class_theorem: &str,
    divisibility_iff_theorem: &str,
    separation_theorem: &str,
) -> MatchedControlAtlasProofCertificate {
    MatchedControlAtlasProofCertificate {
        module: MATCHED_CONTROL_BASE10_SEPARATION_MODULE.to_string(),
        certificate_constant: None,
        modulus,
        excluded_seed_class,
        excluded_seed_class_theorem: qualified_lean_name(
            MATCHED_CONTROL_BASE10_SEPARATION_MODULE,
            excluded_seed_class_theorem,
        ),
        divisibility_iff_theorem: qualified_lean_name(
            MATCHED_CONTROL_BASE10_SEPARATION_MODULE,
            divisibility_iff_theorem,
        ),
        separation_theorem: Some(qualified_lean_name(
            MATCHED_CONTROL_BASE10_SEPARATION_MODULE,
            separation_theorem,
        )),
    }
}

fn qualified_lean_name(module: &str, name: &str) -> String {
    format!("{module}.{name}")
}

fn lean_string_literal(value: &str) -> String {
    let escaped = value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n");
    format!("\"{escaped}\"")
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

fn identifier_tokens(label: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    for ch in label.chars() {
        if ch.is_ascii_alphanumeric() {
            current.push(ch);
        } else if !current.is_empty() {
            tokens.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

fn upper_camel_token(token: &str) -> String {
    let mut chars = token.chars();
    match chars.next() {
        Some(first) => {
            let mut out = String::new();
            out.push(first.to_ascii_uppercase());
            out.push_str(&chars.as_str().to_ascii_lowercase());
            out
        }
        None => String::new(),
    }
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

fn audit_condition(
    active: bool,
    active_severity: MatchedControlAuditSeverity,
    inactive_severity: MatchedControlAuditSeverity,
) -> MatchedControlAuditCondition {
    MatchedControlAuditCondition {
        active,
        severity: if active {
            active_severity
        } else {
            inactive_severity
        },
    }
}

fn error_reasons_from_conditions(
    conditions: MatchedControlComparisonAuditConditions,
) -> Vec<String> {
    let mut reasons = Vec::new();

    if conditions.residual_criterion_changed.severity.is_error() {
        reasons.push("residual-criterion-changed".to_string());
    }
    if conditions.material_family_change.severity.is_error() {
        reasons.push("material-family-change".to_string());
    }
    if conditions.sampling_plan_drift.severity.is_error() {
        reasons.push("sampling-plan-drift".to_string());
    }
    if conditions.added_families.severity.is_error() {
        reasons.push("added-families".to_string());
    }
    if conditions.removed_families.severity.is_error() {
        reasons.push("removed-families".to_string());
    }

    reasons
}

fn conditions_have_error(conditions: MatchedControlComparisonAuditConditions) -> bool {
    [
        conditions.residual_criterion_changed.severity,
        conditions.material_family_change.severity,
        conditions.sampling_plan_drift.severity,
        conditions.added_families.severity,
        conditions.removed_families.severity,
    ]
    .into_iter()
    .any(MatchedControlAuditSeverity::is_error)
}

fn comparison_panel_identity(
    bundle: &MatchedControlComparisonExportBundle,
    index: usize,
) -> Result<Option<String>, MatchedControlBatchError> {
    if bundle.comparison.before_panel_id == bundle.comparison.after_panel_id {
        Ok(bundle.comparison.before_panel_id.clone())
    } else {
        Err(MatchedControlBatchError::PanelMismatchWithinComparison { index })
    }
}

fn observe_condition_tallies(
    tallies: &mut MatchedControlComparisonBatchConditionTallies,
    conditions: MatchedControlComparisonAuditConditions,
) {
    tallies
        .residual_criterion_changed
        .observe(conditions.residual_criterion_changed.severity);
    tallies
        .material_family_change
        .observe(conditions.material_family_change.severity);
    tallies
        .sampling_plan_drift
        .observe(conditions.sampling_plan_drift.severity);
    tallies
        .added_families
        .observe(conditions.added_families.severity);
    tallies
        .removed_families
        .observe(conditions.removed_families.severity);
}

fn record_family_snapshot<'a>(
    families: &'a mut BTreeMap<String, BatchFamilyAccumulator>,
    snapshot: &MatchedControlFamilySnapshot,
) -> &'a mut BatchFamilyAccumulator {
    families
        .entry(snapshot.family_code.clone())
        .or_insert_with(|| BatchFamilyAccumulator {
            family_label: snapshot.family_label.clone(),
            family_code: snapshot.family_code.clone(),
            ..BatchFamilyAccumulator::default()
        })
}

fn record_family_delta<'a>(
    families: &'a mut BTreeMap<String, BatchFamilyAccumulator>,
    delta: &MatchedControlFamilyDelta,
) -> &'a mut BatchFamilyAccumulator {
    families
        .entry(delta.family_code.clone())
        .or_insert_with(|| BatchFamilyAccumulator {
            family_label: delta.family_label.clone(),
            family_code: delta.family_code.clone(),
            ..BatchFamilyAccumulator::default()
        })
}

fn update_max_abs(max_abs: &mut Option<f64>, delta: Option<f64>) {
    if let Some(abs_delta) = delta.map(f64::abs) {
        if max_abs.is_none_or(|current| abs_delta > current) {
            *max_abs = Some(abs_delta);
        }
    }
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

    fn blank_export_bundle(
        samples: usize,
        residual_criterion_met: bool,
    ) -> MatchedControlExportBundle {
        MatchedControlExportBundle {
            export_version: MATCHED_CONTROL_EXPORT_VERSION,
            generated_at_utc: "2026-04-02T16:00:00Z".to_string(),
            panel_id: None,
            settings: MatchedControlRunSettings {
                samples,
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
                residual_criterion_met,
                base_summaries: vec![],
            },
        }
    }

    fn blank_comparison() -> MatchedControlComparison {
        MatchedControlComparison {
            before_generated_at_utc: "2026-04-02T16:00:00Z".to_string(),
            after_generated_at_utc: "2026-04-02T17:00:00Z".to_string(),
            before_export_version: MATCHED_CONTROL_EXPORT_VERSION,
            after_export_version: MATCHED_CONTROL_EXPORT_VERSION,
            before_panel_id: None,
            after_panel_id: None,
            families_compared: 0,
            compared_families: vec![],
            materially_changed_families: vec![],
            added_families: vec![],
            removed_families: vec![],
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
        }
    }

    fn family_snapshot(label: &str, code: &str) -> MatchedControlFamilySnapshot {
        MatchedControlFamilySnapshot {
            family_label: label.to_string(),
            family_code: code.to_string(),
        }
    }

    fn family_delta(
        code: &str,
        label: &str,
        lift_delta: f64,
        q_delta: f64,
    ) -> MatchedControlFamilyDelta {
        MatchedControlFamilyDelta {
            family_label: label.to_string(),
            family_code: code.to_string(),
            lift_before: Some(1.0),
            lift_after: Some(1.0 + lift_delta),
            lift_delta: Some(lift_delta),
            q_before: Some(0.4),
            q_after: Some(0.4 + q_delta),
            q_delta: Some(q_delta),
            decision_before: "ns".to_string(),
            decision_after: "positive-raw".to_string(),
            material_lift_change: true,
            material_q_change: true,
            decision_changed: true,
        }
    }

    fn batch_bundle(
        changed: bool,
        residual_flip: bool,
        panel_id: Option<&str>,
        policy: MatchedControlComparePolicy,
    ) -> MatchedControlComparisonExportBundle {
        let mut before = blank_export_bundle(20, false);
        let mut after = blank_export_bundle(20, residual_flip);
        before.panel_id = panel_id.map(str::to_string);
        after.panel_id = panel_id.map(str::to_string);

        let stable = family_snapshot("Stable family", "B 6 ( 1, 5) k=(0,0) M=1");
        let drifting = family_snapshot("Drifting family", "B10 ( 3, 7) k=(0,0) M=1");
        let mut comparison = blank_comparison();
        comparison.before_panel_id = before.panel_id.clone();
        comparison.after_panel_id = after.panel_id.clone();
        comparison.families_compared = 2;
        comparison.compared_families = vec![stable, drifting.clone()];
        comparison.residual_criterion_after = residual_flip;
        comparison.residual_criterion_changed = residual_flip;

        if changed {
            comparison.materially_changed_families = vec![family_delta(
                &drifting.family_code,
                &drifting.family_label,
                0.35,
                -0.20,
            )];
        }

        let audit = summarize_comparison_audit(&before, &after, &comparison, policy);
        build_comparison_export_bundle(
            "before.json",
            "after.json",
            MatchedControlCompareSettings::default(),
            policy,
            &comparison,
            &audit,
        )
    }

    fn batch_input(
        source_path: &str,
        bundle: MatchedControlComparisonExportBundle,
    ) -> MatchedControlComparisonBatchInput {
        MatchedControlComparisonBatchInput {
            source_path: source_path.to_string(),
            bundle,
        }
    }

    fn atlas_row<'a>(
        manifest: &'a MatchedControlAtlasManifest,
        family_code: &str,
    ) -> &'a MatchedControlAtlasFamilyRow {
        manifest
            .families
            .iter()
            .find(|row| row.family_code == family_code)
            .expect("atlas family row should exist")
    }

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
    fn lean_lane_names_are_shared_with_generated_artifact() {
        assert_eq!(
            matched_control_lean_lane_name(&MAINTAINED_MATCHED_CONTROL_FAMILIES[0], 1),
            "base6ChampionM1Lane"
        );
        assert_eq!(
            matched_control_lean_lane_name(&MAINTAINED_MATCHED_CONTROL_FAMILIES[5], 1),
            "base12CompactM1Lane"
        );
        assert_eq!(
            matched_control_lean_lane_name(&MAINTAINED_MATCHED_CONTROL_FAMILIES[6], 1),
            "base14OffsetM1Lane"
        );
        assert_eq!(
            matched_control_lean_lane_name(&MAINTAINED_MATCHED_CONTROL_FAMILIES[7], 1),
            "base30WheelLikeM1Lane"
        );
    }

    #[test]
    fn atlas_manifest_builds_deterministic_smoke_panel() {
        let manifest = build_matched_control_atlas_manifest(MatchedControlPanel::Smoke);
        let encoded = serde_json::to_string(&manifest).expect("atlas manifest should serialize");
        let decoded: MatchedControlAtlasManifest =
            serde_json::from_str(&encoded).expect("atlas manifest should deserialize");

        assert_eq!(
            manifest.schema_version,
            MATCHED_CONTROL_ATLAS_SCHEMA_VERSION
        );
        assert_eq!(manifest.panel_id, MatchedControlPanel::Smoke.panel_id());
        assert_eq!(manifest.panel, "smoke");
        assert_eq!(manifest.settings, MatchedControlPanel::Smoke.settings());
        assert_eq!(
            manifest.family_count,
            MAINTAINED_MATCHED_CONTROL_FAMILIES.len()
        );
        assert_eq!(
            manifest.lane_count,
            MAINTAINED_MATCHED_CONTROL_FAMILIES.len() * 2
        );
        assert_eq!(manifest.families.len(), manifest.lane_count);
        assert_eq!(decoded, manifest);
        assert!(encoded.contains("\"schema_version\":\"matched-control-atlas-v1\""));
        assert!(encoded.contains("\"claim_status\":\"no-density-mechanism-claim\""));
    }

    #[test]
    fn atlas_manifest_links_known_exact_profile_certificates() {
        let manifest = build_matched_control_atlas_manifest(MatchedControlPanel::Smoke);
        let cases = [
            (
                "B 6 ( 1, 5) k=(0,0) M=1",
                "base6ChampionM1Lane",
                "base6ChampionM1Mod7Certificate",
                7,
                1,
            ),
            (
                "B 6 ( 1, 5) k=(0,0) M=2",
                "base6ChampionM2Lane",
                "base6ChampionM2Mod7Certificate",
                7,
                0,
            ),
            (
                "B10 ( 3, 3) k=(0,1) M=2",
                "base10BreathingM2Lane",
                "base10BreathingM2Mod7Certificate",
                7,
                3,
            ),
            (
                "B10 ( 3, 7) k=(0,0) M=2",
                "base10ClassicM2Lane",
                "base10ClassicM2Mod7Certificate",
                7,
                5,
            ),
            (
                "B10 ( 3, 3) k=(1,1) M=2",
                "base10SymmetricM2Lane",
                "base10SymmetricM2Mod7Certificate",
                7,
                5,
            ),
            (
                "B10 ( 3, 7) k=(1,1) M=2",
                "base10ExclusiveM2Lane",
                "base10ExclusiveM2Mod7Certificate",
                7,
                0,
            ),
            (
                "B12 ( 1, 1) k=(0,0) M=1",
                "base12CompactM1Lane",
                "base12CompactM1Mod5Certificate",
                5,
                2,
            ),
            (
                "B12 ( 1, 1) k=(0,0) M=2",
                "base12CompactM2Lane",
                "base12CompactM2Mod5Certificate",
                5,
                1,
            ),
            (
                "B14 ( 1, 3) k=(0,0) M=1",
                "base14OffsetM1Lane",
                "base14OffsetM1Mod5Certificate",
                5,
                4,
            ),
            (
                "B14 ( 1, 3) k=(0,0) M=2",
                "base14OffsetM2Lane",
                "base14OffsetM2Mod5Certificate",
                5,
                0,
            ),
            (
                "B30 (11, 7) k=(0,0) M=1",
                "base30WheelLikeM1Lane",
                "base30WheelLikeM1Mod7Certificate",
                7,
                4,
            ),
            (
                "B30 (11, 7) k=(0,0) M=2",
                "base30WheelLikeM2Lane",
                "base30WheelLikeM2Mod7Certificate",
                7,
                2,
            ),
        ];

        for (family_code, lane_constant, certificate_constant, modulus, excluded) in cases {
            let row = atlas_row(&manifest, family_code);
            let certificate = row
                .proof_certificate
                .as_ref()
                .expect("profile certificate should be linked");

            assert_eq!(
                row.proof_status,
                MatchedControlAtlasProofStatus::ExactResidueProfile
            );
            assert_eq!(row.lean.lane_constant, lane_constant);
            assert_eq!(
                row.lean.lookup_theorem_qualified,
                format!("{MATCHED_CONTROL_LEAN_LANE_MODULE}.{lane_constant}_lookup")
            );
            assert_eq!(
                certificate.module,
                MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE
            );
            assert_eq!(
                certificate.certificate_constant.as_deref(),
                Some(certificate_constant)
            );
            assert_eq!(certificate.modulus, modulus);
            assert_eq!(certificate.excluded_seed_class, excluded);
            if matches!(
                family_code,
                "B10 ( 3, 3) k=(0,1) M=2" | "B10 ( 3, 3) k=(1,1) M=2"
            ) {
                let expected_separation = format!(
                    "{MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE}.{MATCHED_CONTROL_BASE10_M2_SMOKE_SEPARATION_THEOREM}"
                );
                assert_eq!(
                    certificate.separation_theorem.as_deref(),
                    Some(expected_separation.as_str())
                );
            }
            if matches!(
                family_code,
                "B10 ( 3, 7) k=(0,0) M=2" | "B10 ( 3, 7) k=(1,1) M=2"
            ) {
                let expected_separation = format!(
                    "{MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE}.{MATCHED_CONTROL_BASE10_CLASSIC_M2_SMOKE_SEPARATION_THEOREM}"
                );
                assert_eq!(
                    certificate.separation_theorem.as_deref(),
                    Some(expected_separation.as_str())
                );
            }
        }
    }

    #[test]
    fn smoke_profile_certificate_metadata_is_generated_from_one_table() {
        let rows = build_matched_control_smoke_profile_certificate_metadata();
        let mut keys = std::collections::BTreeSet::new();

        assert_eq!(rows.len(), 14);
        for row in &rows {
            assert!(keys.insert((row.family_code.clone(), row.modulus)));
            assert!(matches!(row.middle_width, 1 | 2));
            assert_eq!(
                row.lean_module,
                MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE
            );
            assert_eq!(
                row.lean_lookup_theorem,
                format!("{}_lookup", row.lean_lane_constant)
            );
        }

        let base6_m2 = rows
            .iter()
            .find(|row| row.family_code == "B 6 ( 1, 5) k=(0,0) M=2")
            .expect("base-6 M2 metadata row");
        assert_eq!(
            base6_m2.certificate_constant,
            "base6ChampionM2Mod7Certificate"
        );
        assert_eq!(base6_m2.modulus, 7);
        assert_eq!(base6_m2.excluded_seed_class, 0);

        let base10_breathing_m2 = rows
            .iter()
            .find(|row| row.family_code == "B10 ( 3, 3) k=(0,1) M=2")
            .expect("base-10 breathing M2 metadata row");
        assert_eq!(
            base10_breathing_m2.certificate_constant,
            "base10BreathingM2Mod7Certificate"
        );
        assert_eq!(base10_breathing_m2.modulus, 7);
        assert_eq!(base10_breathing_m2.excluded_seed_class, 3);

        let base10_classic_m2 = rows
            .iter()
            .find(|row| row.family_code == "B10 ( 3, 7) k=(0,0) M=2")
            .expect("base-10 classic M2 metadata row");
        assert_eq!(
            base10_classic_m2.certificate_constant,
            "base10ClassicM2Mod7Certificate"
        );
        assert_eq!(base10_classic_m2.modulus, 7);
        assert_eq!(base10_classic_m2.excluded_seed_class, 5);

        let base10_symmetric_m2 = rows
            .iter()
            .find(|row| row.family_code == "B10 ( 3, 3) k=(1,1) M=2")
            .expect("base-10 symmetric M2 metadata row");
        assert_eq!(
            base10_symmetric_m2.certificate_constant,
            "base10SymmetricM2Mod7Certificate"
        );
        assert_eq!(base10_symmetric_m2.modulus, 7);
        assert_eq!(base10_symmetric_m2.excluded_seed_class, 5);

        let base10_symmetric_m2_mod3 = rows
            .iter()
            .find(|row| row.family_code == "B10 ( 3, 3) k=(1,1) M=2" && row.modulus == 3)
            .expect("base-10 symmetric M2 mod3 metadata row");
        assert_eq!(
            base10_symmetric_m2_mod3.certificate_constant,
            "base10SymmetricM2Mod3Certificate"
        );
        assert_eq!(base10_symmetric_m2_mod3.excluded_seed_class, 0);

        let base10_classic_m2_mod3 = rows
            .iter()
            .find(|row| row.family_code == "B10 ( 3, 7) k=(0,0) M=2" && row.modulus == 3)
            .expect("base-10 classic M2 mod3 metadata row");
        assert_eq!(
            base10_classic_m2_mod3.certificate_constant,
            "base10ClassicM2Mod3Certificate"
        );
        assert_eq!(base10_classic_m2_mod3.excluded_seed_class, 1);

        let base10_exclusive_m2 = rows
            .iter()
            .find(|row| row.family_code == "B10 ( 3, 7) k=(1,1) M=2")
            .expect("base-10 exclusive M2 metadata row");
        assert_eq!(
            base10_exclusive_m2.certificate_constant,
            "base10ExclusiveM2Mod7Certificate"
        );
        assert_eq!(base10_exclusive_m2.modulus, 7);
        assert_eq!(base10_exclusive_m2.excluded_seed_class, 0);

        let pair_rows = build_matched_control_smoke_pair_certificate_metadata();
        assert_eq!(pair_rows.len(), 12);
        assert!(pair_rows
            .iter()
            .all(|row| row.forbidden_residue_set_theorem.is_some()));
        assert!(pair_rows
            .iter()
            .all(|row| row.forbidden_residue_set_theorem_qualified.is_some()));
        let rank_one_same_boundary_pair = pair_rows
            .iter()
            .find(|row| {
                row.left_family_code == "B10 ( 3, 3) k=(0,1) M=1"
                    && row.right_family_code == "B10 ( 3, 3) k=(1,1) M=1"
            })
            .expect("base-10 M1 breathing/symmetric pair metadata row");
        assert_eq!(rank_one_same_boundary_pair.modulus, 11);
        assert_eq!(rank_one_same_boundary_pair.left_excluded_seed_class, 0);
        assert_eq!(rank_one_same_boundary_pair.right_excluded_seed_class, 10);
        assert_eq!(
            rank_one_same_boundary_pair.separation_theorem_qualified,
            format!(
                "{MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_BREATHING_SYMMETRIC_SEPARATION_THEOREM}"
            )
        );
        assert_eq!(
            rank_one_same_boundary_pair
                .forbidden_residue_set_theorem
                .as_deref(),
            Some(MATCHED_CONTROL_BASE10_M1_BREATHING_SYMMETRIC_FORBIDDEN_RESIDUES_THEOREM)
        );
        assert_eq!(
            rank_one_same_boundary_pair
                .forbidden_residue_set_theorem_qualified
                .as_deref(),
            Some(
                "PrimeArithmetic.Density.Base10SeedClassSeparation.forbiddenResidues_breathingM1_ne_symmetricM1_mod11"
            )
        );
        assert_eq!(
            rank_one_same_boundary_pair.equal_survivor_theorem_qualified,
            format!(
                "{MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_BREATHING_SYMMETRIC_EQUAL_SURVIVOR_THEOREM}"
            )
        );

        let breathing_exclusive_m1_pair = pair_rows
            .iter()
            .find(|row| {
                row.left_family_code == "B10 ( 3, 3) k=(0,1) M=1"
                    && row.right_family_code == "B10 ( 3, 7) k=(1,1) M=1"
            })
            .expect("base-10 M1 breathing/exclusive pair metadata row");
        assert_eq!(breathing_exclusive_m1_pair.modulus, 11);
        assert_eq!(breathing_exclusive_m1_pair.left_excluded_seed_class, 0);
        assert_eq!(breathing_exclusive_m1_pair.right_excluded_seed_class, 2);
        assert_eq!(
            breathing_exclusive_m1_pair.separation_theorem_qualified,
            format!(
                "{MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_BREATHING_EXCLUSIVE_SEPARATION_THEOREM}"
            )
        );
        assert_eq!(
            breathing_exclusive_m1_pair.equal_survivor_theorem_qualified,
            format!(
                "{MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_BREATHING_EXCLUSIVE_EQUAL_SURVIVOR_THEOREM}"
            )
        );

        let breathing_classic_m1_pair = pair_rows
            .iter()
            .find(|row| {
                row.left_family_code == "B10 ( 3, 3) k=(0,1) M=1"
                    && row.right_family_code == "B10 ( 3, 7) k=(0,0) M=1"
            })
            .expect("base-10 M1 breathing/classic pair metadata row");
        assert_eq!(breathing_classic_m1_pair.modulus, 11);
        assert_eq!(breathing_classic_m1_pair.left_excluded_seed_class, 0);
        assert_eq!(breathing_classic_m1_pair.right_excluded_seed_class, 8);
        assert_eq!(
            breathing_classic_m1_pair.separation_theorem_qualified,
            format!(
                "{MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_BREATHING_CLASSIC_SEPARATION_THEOREM}"
            )
        );
        assert_eq!(
            breathing_classic_m1_pair.equal_survivor_theorem_qualified,
            format!(
                "{MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_BREATHING_CLASSIC_EQUAL_SURVIVOR_THEOREM}"
            )
        );

        let classic_exclusive_m1_pair = pair_rows
            .iter()
            .find(|row| {
                row.left_family_code == "B10 ( 3, 7) k=(0,0) M=1"
                    && row.right_family_code == "B10 ( 3, 7) k=(1,1) M=1"
            })
            .expect("base-10 M1 classic/exclusive pair metadata row");
        assert_eq!(classic_exclusive_m1_pair.modulus, 11);
        assert_eq!(classic_exclusive_m1_pair.left_excluded_seed_class, 8);
        assert_eq!(classic_exclusive_m1_pair.right_excluded_seed_class, 2);
        assert_eq!(
            classic_exclusive_m1_pair.separation_theorem_qualified,
            format!(
                "{MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_CLASSIC_EXCLUSIVE_SEPARATION_THEOREM}"
            )
        );
        assert_eq!(
            classic_exclusive_m1_pair.equal_survivor_theorem_qualified,
            format!(
                "{MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_CLASSIC_EXCLUSIVE_EQUAL_SURVIVOR_THEOREM}"
            )
        );

        let lowest_overlap_cross_boundary_pair = pair_rows
            .iter()
            .find(|row| {
                row.left_family_code == "B10 ( 3, 3) k=(1,1) M=1"
                    && row.right_family_code == "B10 ( 3, 7) k=(1,1) M=1"
            })
            .expect("base-10 M1 symmetric/exclusive pair metadata row");
        assert_eq!(lowest_overlap_cross_boundary_pair.modulus, 11);
        assert_eq!(
            lowest_overlap_cross_boundary_pair.left_excluded_seed_class,
            10
        );
        assert_eq!(
            lowest_overlap_cross_boundary_pair.right_excluded_seed_class,
            2
        );
        assert_eq!(
            lowest_overlap_cross_boundary_pair.separation_theorem_qualified,
            format!(
                "{MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_SYMMETRIC_EXCLUSIVE_SEPARATION_THEOREM}"
            )
        );
        assert_eq!(
            lowest_overlap_cross_boundary_pair.equal_survivor_theorem_qualified,
            format!(
                "{MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_SYMMETRIC_EXCLUSIVE_EQUAL_SURVIVOR_THEOREM}"
            )
        );

        let cross_boundary_pair = pair_rows
            .iter()
            .find(|row| {
                row.left_family_code == "B10 ( 3, 3) k=(1,1) M=1"
                    && row.right_family_code == "B10 ( 3, 7) k=(0,0) M=1"
            })
            .expect("base-10 M1 symmetric/classic pair metadata row");
        assert_eq!(cross_boundary_pair.modulus, 11);
        assert_eq!(cross_boundary_pair.left_excluded_seed_class, 10);
        assert_eq!(cross_boundary_pair.right_excluded_seed_class, 8);
        assert_eq!(
            cross_boundary_pair.separation_theorem_qualified,
            format!(
                "{MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_SYMMETRIC_CLASSIC_SEPARATION_THEOREM}"
            )
        );
        assert_eq!(
            cross_boundary_pair.equal_survivor_theorem_qualified,
            format!(
                "{MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_SYMMETRIC_CLASSIC_EQUAL_SURVIVOR_THEOREM}"
            )
        );

        let generated_m2_pair = pair_rows
            .iter()
            .find(|row| {
                row.left_family_code == "B10 ( 3, 3) k=(0,1) M=2"
                    && row.right_family_code == "B10 ( 3, 3) k=(1,1) M=2"
            })
            .expect("base-10 M2 breathing/symmetric generated pair metadata row");
        assert_eq!(
            generated_m2_pair.forbidden_residue_set_theorem.as_deref(),
            Some(MATCHED_CONTROL_BASE10_M2_SMOKE_FORBIDDEN_RESIDUES_THEOREM)
        );
        assert_eq!(
            generated_m2_pair
                .forbidden_residue_set_theorem_qualified
                .as_deref(),
            Some(
                "PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates.forbiddenResidues_base10BreathingM2_ne_base10SymmetricM2_mod7"
            )
        );

        let candidates = render_matched_control_smoke_profile_certificate_lean_candidates();
        assert!(candidates.contains("def base6ChampionM2Mod7Certificate"));
        assert!(candidates.contains("\"B10 ( 3, 3) k=(0,1) M=2\" 2 base10BreathingM2Lane 7 3"));
        assert!(candidates.contains("\"B10 ( 3, 7) k=(0,0) M=2\" 2 base10ClassicM2Lane 7 5"));
        assert!(candidates.contains("\"B10 ( 3, 3) k=(1,1) M=2\" 2 base10SymmetricM2Lane 7 5"));
        assert!(candidates.contains("\"B10 ( 3, 7) k=(1,1) M=2\" 2 base10ExclusiveM2Lane 7 0"));
        assert!(candidates.contains("\"B30 (11, 7) k=(0,0) M=2\" 2 base30WheelLikeM2Lane 7 2"));

        let module = render_matched_control_smoke_profile_certificate_lean_module();
        assert!(module.contains("import PrimeArithmetic.Density.CanonicalSmokeLaneProfiles"));
        assert!(module.contains(
            "namespace PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates"
        ));
        for row in &rows {
            assert!(module.contains(&format!("def {}", row.certificate_constant)));
            assert!(module.contains(&format!(
                "theorem {}",
                row.profile_excluded_seed_class_theorem
            )));
            assert!(module.contains(&format!("theorem {}", row.divisibility_iff_theorem)));
        }
        assert!(module.contains(&format!(
            "theorem {MATCHED_CONTROL_BASE10_M2_SMOKE_SEPARATION_THEOREM}"
        )));
        assert!(module.contains(&format!(
            "theorem {MATCHED_CONTROL_BASE10_M2_SMOKE_FORBIDDEN_RESIDUES_THEOREM}"
        )));
        assert!(module.contains(&format!(
            "theorem {MATCHED_CONTROL_BASE10_CLASSIC_M2_SMOKE_SEPARATION_THEOREM}"
        )));
        assert!(module.contains(&format!(
            "theorem {MATCHED_CONTROL_BASE10_BREATHING_CLASSIC_M2_SMOKE_SEPARATION_THEOREM}"
        )));
        assert!(module.contains(&format!(
            "theorem {MATCHED_CONTROL_BASE10_SYMMETRIC_EXCLUSIVE_M2_SMOKE_SEPARATION_THEOREM}"
        )));
        assert!(module.contains(&format!(
            "theorem {MATCHED_CONTROL_BASE10_BREATHING_EXCLUSIVE_M2_SMOKE_SEPARATION_THEOREM}"
        )));
        assert!(module.contains(&format!(
            "theorem {MATCHED_CONTROL_BASE10_SYMMETRIC_CLASSIC_M2_SMOKE_SEPARATION_THEOREM}"
        )));

        let checks = render_matched_control_smoke_profile_certificate_lean_checks();
        assert!(checks
            .contains("import PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates"));
        assert!(checks.contains("import PrimeArithmetic.Density.Base10SeedClassSeparation"));
        for row in &rows {
            assert!(checks.contains(&format!(
                "#check {MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE}.{}",
                row.certificate_constant
            )));
            assert!(checks.contains(&format!(
                "#check {MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE}.{}",
                row.divisibility_iff_theorem
            )));
        }
        assert!(checks.contains(&format!(
            "#check {MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE}.{MATCHED_CONTROL_BASE10_M2_SMOKE_SEPARATION_THEOREM}"
        )));
        assert!(checks.contains(&format!(
            "#check {MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE}.{MATCHED_CONTROL_BASE10_M2_SMOKE_FORBIDDEN_RESIDUES_THEOREM}"
        )));
        assert!(checks.contains(&format!(
            "#check {MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE}.{MATCHED_CONTROL_BASE10_CLASSIC_M2_SMOKE_SEPARATION_THEOREM}"
        )));
        assert!(checks.contains(&format!(
            "#check {MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_BREATHING_SYMMETRIC_SEPARATION_THEOREM}"
        )));
        assert!(checks.contains(&format!(
            "#check {MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_BREATHING_SYMMETRIC_FORBIDDEN_RESIDUES_THEOREM}"
        )));
        assert!(checks.contains(&format!(
            "#check {MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_BREATHING_SYMMETRIC_EQUAL_SURVIVOR_THEOREM}"
        )));
        assert!(checks.contains(&format!(
            "#check {MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_BREATHING_EXCLUSIVE_SEPARATION_THEOREM}"
        )));
        assert!(checks.contains(&format!(
            "#check {MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_BREATHING_EXCLUSIVE_EQUAL_SURVIVOR_THEOREM}"
        )));
        assert!(checks.contains(&format!(
            "#check {MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_BREATHING_CLASSIC_SEPARATION_THEOREM}"
        )));
        assert!(checks.contains(&format!(
            "#check {MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_BREATHING_CLASSIC_EQUAL_SURVIVOR_THEOREM}"
        )));
        assert!(checks.contains(&format!(
            "#check {MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_CLASSIC_EXCLUSIVE_SEPARATION_THEOREM}"
        )));
        assert!(checks.contains(&format!(
            "#check {MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_CLASSIC_EXCLUSIVE_EQUAL_SURVIVOR_THEOREM}"
        )));
        assert!(checks.contains(&format!(
            "#check {MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_SYMMETRIC_CLASSIC_SEPARATION_THEOREM}"
        )));
        assert!(checks.contains(&format!(
            "#check {MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_SYMMETRIC_CLASSIC_EQUAL_SURVIVOR_THEOREM}"
        )));
        assert!(checks.contains(&format!(
            "#check {MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_SYMMETRIC_EXCLUSIVE_SEPARATION_THEOREM}"
        )));
        assert!(checks.contains(&format!(
            "#check {MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_SYMMETRIC_EXCLUSIVE_EQUAL_SURVIVOR_THEOREM}"
        )));

        let silent_checks = render_matched_control_smoke_profile_certificate_lean_silent_checks();
        assert!(silent_checks
            .contains("import PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates"));
        assert!(silent_checks.contains("import PrimeArithmetic.Density.Base10SeedClassSeparation"));
        assert!(!silent_checks.contains("#check "));
        assert!(silent_checks.contains("example : True := by"));
        assert!(silent_checks.contains(&format!(
            "have _ := {MATCHED_CONTROL_SMOKE_PROFILE_CERTIFICATE_MODULE}.{}",
            rows[0].certificate_constant
        )));
        assert!(silent_checks.contains(&format!(
            "have _ := {MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_BREATHING_SYMMETRIC_SEPARATION_THEOREM}"
        )));

        let sharded_checks =
            render_matched_control_smoke_profile_certificate_lean_silent_check_shards(
                "PrimeArithmetic.Generated",
                "MatchedControlSmokeProfileCertificateChecks",
                4,
            )
            .expect("render sharded silent checks");
        assert_eq!(sharded_checks.shards.len(), 5);
        assert!(sharded_checks.umbrella_contents.contains(
            "import PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificateChecksShard01"
        ));
        assert!(sharded_checks.umbrella_contents.contains(
            "import PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificateChecksShard05"
        ));
        assert!(sharded_checks.shards[0].contents.contains(
            "have _ := PrimeArithmetic.Density.CanonicalSmokeLaneProfiles.base6ChampionM1Lane"
        ));
        assert!(sharded_checks.shards[2]
            .contents
            .contains("have _ := PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates.base30WheelLikeM2Mod7Certificate"));
        assert!(sharded_checks.shards[3]
            .contents
            .contains("have _ := PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates.base10SymmetricM2Mod3Certificate"));
        assert!(sharded_checks.shards[4].contents.contains(&format!(
            "have _ := {MATCHED_CONTROL_BASE10_SEPARATION_MODULE}.{MATCHED_CONTROL_BASE10_M1_BREATHING_SYMMETRIC_SEPARATION_THEOREM}"
        )));
        assert!(!sharded_checks
            .umbrella_contents
            .contains("example : True := by"));
    }

    #[test]
    fn atlas_manifest_marks_all_smoke_rows_with_proof_claims() {
        let manifest = build_matched_control_atlas_manifest(MatchedControlPanel::Smoke);
        assert!(manifest
            .families
            .iter()
            .all(|row| row.proof_status != MatchedControlAtlasProofStatus::LaneGeneratedOnly));
        assert!(manifest
            .families
            .iter()
            .all(|row| row.proof_certificate.is_some()));
    }

    #[test]
    fn atlas_manifest_links_base10_seed_class_separation_facts() {
        let manifest = build_matched_control_atlas_manifest(MatchedControlPanel::Smoke);
        let row = atlas_row(&manifest, "B10 ( 3, 3) k=(0,1) M=1");
        let certificate = row
            .proof_certificate
            .as_ref()
            .expect("base-10 separation fact should be linked");

        assert_eq!(
            row.proof_status,
            MatchedControlAtlasProofStatus::ExactSeedClassSeparation
        );
        assert_eq!(row.lean.lane_constant, "base10BreathingM1Lane");
        assert_eq!(certificate.module, MATCHED_CONTROL_BASE10_SEPARATION_MODULE);
        assert_eq!(certificate.certificate_constant, None);
        assert_eq!(certificate.modulus, 11);
        assert_eq!(certificate.excluded_seed_class, 0);
        assert_eq!(
            certificate.separation_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Density.Base10SeedClassSeparation.forbiddenSeedMask_breathingM1_ne_symmetricM1_mod11"
            )
        );
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
            panel_id: None,
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
            panel_id: None,
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
        let before = blank_export_bundle(20, false);
        let mut after = blank_export_bundle(24, true);
        after.generated_at_utc = "2026-04-02T17:00:00Z".to_string();
        after.summary.positive_q = 1;
        after.summary.positive_q_bases = vec![10];
        after.summary.pooled_lift = Some(1.4);
        after.summary.pooled_lift_ci_lo = Some(1.1);
        after.summary.pooled_lift_ci_hi = Some(1.8);

        let mut comparison = blank_comparison();
        comparison.before_generated_at_utc = before.generated_at_utc.clone();
        comparison.after_generated_at_utc = after.generated_at_utc.clone();
        comparison.families_compared = 1;
        comparison.materially_changed_families = vec![MatchedControlFamilyDelta {
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
        }];
        comparison.residual_criterion_after = true;
        comparison.residual_criterion_changed = true;
        comparison.pooled_lift_after = Some(1.4);
        comparison.pooled_lift_delta = Some(0.4);
        comparison.positive_q_after = 1;

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
        assert!(audit.conditions.residual_criterion_changed.active);
        assert_eq!(
            audit.conditions.residual_criterion_changed.severity,
            MatchedControlAuditSeverity::Error
        );
        assert!(audit.conditions.material_family_change.active);
        assert_eq!(
            audit.conditions.material_family_change.severity,
            MatchedControlAuditSeverity::Error
        );
        assert!(audit.conditions.sampling_plan_drift.active);
        assert_eq!(
            audit.conditions.sampling_plan_drift.severity,
            MatchedControlAuditSeverity::Info
        );
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
    fn comparison_audit_default_policy_keeps_sampling_drift_informational() {
        let before = blank_export_bundle(20, false);
        let after = blank_export_bundle(24, false);
        let comparison = blank_comparison();
        let default_audit = summarize_comparison_audit(
            &before,
            &after,
            &comparison,
            MatchedControlComparePolicy::default(),
        );

        assert!(!default_audit.flagged);
        assert!(default_audit.sampling_plan_changed);
        assert!(default_audit.reasons.is_empty());
        assert!(default_audit.conditions.sampling_plan_drift.active);
        assert_eq!(
            default_audit.conditions.sampling_plan_drift.severity,
            MatchedControlAuditSeverity::Info
        );
    }

    #[test]
    fn comparison_audit_promotes_sampling_plan_drift_when_policy_requests_it() {
        let before = blank_export_bundle(20, false);
        let after = blank_export_bundle(24, false);
        let comparison = blank_comparison();
        let audit = summarize_comparison_audit(
            &before,
            &after,
            &comparison,
            MatchedControlComparePolicy {
                flag_sampling_plan_drift: true,
                ..MatchedControlComparePolicy::default()
            },
        );

        assert!(audit.flagged);
        assert_eq!(audit.reasons, vec!["sampling-plan-drift".to_string()]);
        assert!(audit.conditions.sampling_plan_drift.active);
        assert_eq!(
            audit.conditions.sampling_plan_drift.severity,
            MatchedControlAuditSeverity::Error
        );
    }

    #[test]
    fn comparison_audit_promotes_family_set_changes_only_when_policy_requests_it() {
        let cases = vec![
            (
                vec![MatchedControlFamilySnapshot {
                    family_label: "Base 6 champion".to_string(),
                    family_code: "B 6 ( 1, 5) k=(0,0) M=1".to_string(),
                }],
                vec![],
                MatchedControlComparePolicy {
                    flag_added_families: true,
                    ..MatchedControlComparePolicy::default()
                },
                vec!["added-families".to_string()],
            ),
            (
                vec![],
                vec![MatchedControlFamilySnapshot {
                    family_label: "Base 30 wheel-like".to_string(),
                    family_code: "B30 (11, 7) k=(0,0) M=1".to_string(),
                }],
                MatchedControlComparePolicy {
                    flag_removed_families: true,
                    ..MatchedControlComparePolicy::default()
                },
                vec!["removed-families".to_string()],
            ),
            (
                vec![MatchedControlFamilySnapshot {
                    family_label: "Base 6 champion".to_string(),
                    family_code: "B 6 ( 1, 5) k=(0,0) M=1".to_string(),
                }],
                vec![MatchedControlFamilySnapshot {
                    family_label: "Base 30 wheel-like".to_string(),
                    family_code: "B30 (11, 7) k=(0,0) M=1".to_string(),
                }],
                MatchedControlComparePolicy {
                    flag_added_families: true,
                    flag_removed_families: true,
                    ..MatchedControlComparePolicy::default()
                },
                vec!["added-families".to_string(), "removed-families".to_string()],
            ),
        ];

        for (added_families, removed_families, policy, expected_reasons) in cases {
            let before = blank_export_bundle(20, false);
            let after = blank_export_bundle(20, false);
            let mut comparison = blank_comparison();
            comparison.added_families = added_families.clone();
            comparison.removed_families = removed_families.clone();

            let default_audit = summarize_comparison_audit(
                &before,
                &after,
                &comparison,
                MatchedControlComparePolicy::default(),
            );
            let strict_audit = summarize_comparison_audit(&before, &after, &comparison, policy);

            assert!(!default_audit.flagged);
            assert_eq!(default_audit.reasons, Vec::<String>::new());
            assert_eq!(
                default_audit.conditions.added_families.severity,
                if added_families.is_empty() {
                    MatchedControlAuditSeverity::Clear
                } else {
                    MatchedControlAuditSeverity::Info
                }
            );
            assert_eq!(
                default_audit.conditions.removed_families.severity,
                if removed_families.is_empty() {
                    MatchedControlAuditSeverity::Clear
                } else {
                    MatchedControlAuditSeverity::Info
                }
            );

            assert!(strict_audit.flagged);
            assert_eq!(strict_audit.reasons, expected_reasons);
            assert_eq!(
                strict_audit.conditions.added_families.severity,
                if added_families.is_empty() {
                    MatchedControlAuditSeverity::Clear
                } else if policy.flag_added_families {
                    MatchedControlAuditSeverity::Error
                } else {
                    MatchedControlAuditSeverity::Info
                }
            );
            assert_eq!(
                strict_audit.conditions.removed_families.severity,
                if removed_families.is_empty() {
                    MatchedControlAuditSeverity::Clear
                } else if policy.flag_removed_families {
                    MatchedControlAuditSeverity::Error
                } else {
                    MatchedControlAuditSeverity::Info
                }
            );
        }
    }

    #[test]
    fn comparison_export_bundle_round_trips_structured_conditions() {
        let before = blank_export_bundle(20, false);
        let after = blank_export_bundle(24, false);
        let comparison = blank_comparison();
        let policy = MatchedControlComparePolicy {
            flag_sampling_plan_drift: true,
            ..MatchedControlComparePolicy::default()
        };
        let audit = summarize_comparison_audit(&before, &after, &comparison, policy);
        let bundle = build_comparison_export_bundle(
            "before.json",
            "after.json",
            MatchedControlCompareSettings::default(),
            policy,
            &comparison,
            &audit,
        );
        let encoded = serde_json::to_string(&bundle).expect("comparison bundle should serialize");

        assert!(encoded.contains("\"compare_policy\""));
        assert!(encoded.contains("\"conditions\""));
        assert!(encoded.contains("\"sampling_plan_drift\""));
        assert!(encoded.contains("\"severity\":\"error\""));

        let decoded: MatchedControlComparisonExportBundle =
            serde_json::from_str(&encoded).expect("comparison bundle should deserialize");
        assert_eq!(decoded.compare_policy, policy);
        assert!(decoded.audit.flagged);
        assert_eq!(
            decoded.audit.conditions.sampling_plan_drift.severity,
            MatchedControlAuditSeverity::Error
        );
    }

    #[test]
    fn panel_metadata_round_trips_raw_export_json() {
        let reports = vec![MatchedControlReport {
            family: MAINTAINED_MATCHED_CONTROL_FAMILIES[0],
            seed_len: 1,
            mean_digits: 4.0,
            membrane: MatchedControlArmStats {
                primes: 5,
                samples: 20,
                rate: 0.25,
                ci: (0.11, 0.47),
            },
            control: MatchedControlArmStats {
                primes: 4,
                samples: 20,
                rate: 0.20,
                ci: (0.08, 0.42),
            },
            diff: 0.05,
            diff_ci: (-0.20, 0.30),
            lift: 1.25,
            lift_ci: (0.40, 3.20),
            hedges_g: 0.1,
            p_value: 0.70,
            q_value: 0.70,
            decision: MatchedControlDecision::NotSignificant,
        }];
        let summary = summarize_reports(&reports, MatchedControlPanel::Smoke.settings());
        let bundle = build_export_bundle_with_panel(
            &reports,
            &summary,
            MatchedControlPanel::Smoke.settings(),
            Some(MatchedControlPanel::Smoke.panel_id()),
        );
        let encoded = serde_json::to_string(&bundle).expect("raw export should serialize");

        assert!(encoded.contains("\"panel_id\":\"canonical-smoke-v1\""));

        let decoded: MatchedControlExportBundle =
            serde_json::from_str(&encoded).expect("raw export should deserialize");
        assert_eq!(
            decoded.panel_id.as_deref(),
            Some(MatchedControlPanel::Smoke.panel_id())
        );
    }

    #[test]
    fn comparison_export_round_trips_panel_ids_and_compared_families() {
        let mut before = blank_export_bundle(20, false);
        let mut after = blank_export_bundle(20, false);
        before.panel_id = Some(MatchedControlPanel::Smoke.panel_id().to_string());
        after.panel_id = Some(MatchedControlPanel::Smoke.panel_id().to_string());
        before.reports = vec![MatchedControlExportRow::from_report(
            &MatchedControlReport {
                family: MAINTAINED_MATCHED_CONTROL_FAMILIES[0],
                seed_len: 1,
                mean_digits: 4.0,
                membrane: MatchedControlArmStats {
                    primes: 5,
                    samples: 20,
                    rate: 0.25,
                    ci: (0.11, 0.47),
                },
                control: MatchedControlArmStats {
                    primes: 4,
                    samples: 20,
                    rate: 0.20,
                    ci: (0.08, 0.42),
                },
                diff: 0.05,
                diff_ci: (-0.20, 0.30),
                lift: 1.25,
                lift_ci: (0.40, 3.20),
                hedges_g: 0.1,
                p_value: 0.70,
                q_value: 0.70,
                decision: MatchedControlDecision::NotSignificant,
            },
            MatchedControlPanel::Smoke.settings(),
        )];
        after.reports = before.reports.clone();

        let comparison =
            compare_export_bundles(&before, &after, MatchedControlCompareSettings::default());
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
        let encoded = serde_json::to_string(&bundle).expect("comparison should serialize");
        let decoded: MatchedControlComparisonExportBundle =
            serde_json::from_str(&encoded).expect("comparison should deserialize");

        assert_eq!(
            decoded.comparison.before_panel_id.as_deref(),
            Some(MatchedControlPanel::Smoke.panel_id())
        );
        assert_eq!(
            decoded.comparison.after_panel_id.as_deref(),
            Some(MatchedControlPanel::Smoke.panel_id())
        );
        assert_eq!(decoded.comparison.compared_families.len(), 1);
    }

    #[test]
    fn comparison_batch_aggregates_homogeneous_inputs() {
        let inputs = vec![
            batch_input(
                "diff-a.json",
                batch_bundle(
                    true,
                    true,
                    Some(MatchedControlPanel::Smoke.panel_id()),
                    MatchedControlComparePolicy::default(),
                ),
            ),
            batch_input(
                "diff-b.json",
                batch_bundle(
                    false,
                    false,
                    Some(MatchedControlPanel::Smoke.panel_id()),
                    MatchedControlComparePolicy::default(),
                ),
            ),
        ];
        let summary = summarize_comparison_batch(&inputs).expect("batch should summarize");

        assert_eq!(summary.run_count, 2);
        assert_eq!(summary.flagged_run_count, 1);
        assert_eq!(summary.residual_criterion_flip_count, 1);
        assert_eq!(
            summary.panel_id.as_deref(),
            Some(MatchedControlPanel::Smoke.panel_id())
        );
        assert_eq!(
            summary.condition_tallies.residual_criterion_changed,
            MatchedControlAuditSeverityTally {
                clear: 1,
                info: 0,
                error: 1,
            }
        );
        assert_eq!(
            summary.condition_tallies.material_family_change,
            MatchedControlAuditSeverityTally {
                clear: 1,
                info: 0,
                error: 1,
            }
        );
        assert_eq!(
            summary.condition_tallies.sampling_plan_drift,
            MatchedControlAuditSeverityTally {
                clear: 2,
                info: 0,
                error: 0,
            }
        );
        assert_eq!(summary.stable_family_count, 1);
        assert_eq!(summary.drifting_family_count, 1);

        let drifting = summary
            .family_rows
            .iter()
            .find(|row| row.status == MatchedControlBatchFamilyStatus::Drifting)
            .expect("drifting family row");
        assert_eq!(drifting.material_change_count, 1);
        assert_eq!(drifting.decision_change_count, 1);
        assert_eq!(drifting.max_abs_lift_delta, Some(0.35));
        assert_eq!(drifting.max_abs_q_delta, Some(0.20));
    }

    #[test]
    fn comparison_batch_rejects_mixed_settings_policy_and_panel() {
        let first = batch_input(
            "diff-a.json",
            batch_bundle(
                false,
                false,
                Some(MatchedControlPanel::Smoke.panel_id()),
                MatchedControlComparePolicy::default(),
            ),
        );

        let mut settings_mismatch = batch_bundle(
            false,
            false,
            Some(MatchedControlPanel::Smoke.panel_id()),
            MatchedControlComparePolicy::default(),
        );
        settings_mismatch.compare_settings.lift_threshold = 0.99;
        assert!(matches!(
            summarize_comparison_batch(&[
                first.clone(),
                batch_input("diff-settings.json", settings_mismatch)
            ]),
            Err(MatchedControlBatchError::CompareSettingsMismatch { index: 1 })
        ));

        let policy = MatchedControlComparePolicy {
            flag_sampling_plan_drift: true,
            ..MatchedControlComparePolicy::default()
        };
        assert!(matches!(
            summarize_comparison_batch(&[
                first.clone(),
                batch_input(
                    "diff-policy.json",
                    batch_bundle(
                        false,
                        false,
                        Some(MatchedControlPanel::Smoke.panel_id()),
                        policy
                    )
                )
            ]),
            Err(MatchedControlBatchError::ComparePolicyMismatch { index: 1 })
        ));

        assert!(matches!(
            summarize_comparison_batch(&[
                first,
                batch_input(
                    "diff-panel.json",
                    batch_bundle(
                        false,
                        false,
                        Some(MatchedControlPanel::Audit.panel_id()),
                        MatchedControlComparePolicy::default()
                    )
                )
            ]),
            Err(MatchedControlBatchError::PanelMismatchAcrossBatch { index: 1 })
        ));
    }

    #[test]
    fn comparison_batch_rejects_missing_compared_family_snapshots() {
        let mut bundle = batch_bundle(
            false,
            false,
            Some(MatchedControlPanel::Smoke.panel_id()),
            MatchedControlComparePolicy::default(),
        );
        bundle.comparison.compared_families.clear();

        assert!(matches!(
            summarize_comparison_batch(&[batch_input("diff-old.json", bundle)]),
            Err(MatchedControlBatchError::MissingComparedFamilySnapshots { index: 0 })
        ));
    }
}
