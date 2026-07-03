//! Exact residue-mask scanner for maintained matched-control lanes.
//!
//! This module ranks local forbidden seed-class distinctions for future Lean
//! work. It is an arithmetic scanner over affine template lanes, not a density
//! or residual-mechanism claim.

use crate::validation::{
    fast_affine::{build_fast_affine_lane, FastLaneConfig},
    matched_control::{
        build_matched_control_atlas_manifest, matched_control_lean_lane_name,
        matched_control_smoke_pair_certificate_metadata_for, MatchedControlAtlasProofStatus,
        MatchedControlPanel, MatchedControlSmokePairCertificateMetadata,
        MAINTAINED_MATCHED_CONTROL_FAMILIES,
    },
};
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};
use thiserror::Error;

pub const MATCHED_CONTROL_RESIDUE_MASK_SCHEMA_VERSION: &str = "matched-control-residue-masks-v4";
pub const DEFAULT_RESIDUE_MASK_PRIME_BOUND: u32 = 31;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchedControlResidueMaskSettings {
    pub panel: MatchedControlPanel,
    pub prime_bound: u32,
}

impl Default for MatchedControlResidueMaskSettings {
    fn default() -> Self {
        Self {
            panel: MatchedControlPanel::Smoke,
            prime_bound: DEFAULT_RESIDUE_MASK_PRIME_BOUND,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchedControlResidueMaskReportSettings {
    pub panel: String,
    pub panel_id: String,
    pub prime_bound: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchedControlResidueMaskSummary {
    pub panel: String,
    pub panel_id: String,
    pub prime_bound: u32,
    pub lane_count: usize,
    pub lane_modulus_row_count: usize,
    pub pair_candidate_count: usize,
    pub pair_fingerprint_row_count: usize,
    pub pair_certified_count: usize,
    pub pair_uncertified_count: usize,
    pub same_boundary_candidate_count: usize,
    pub same_boundary_k_distinction_candidate_count: usize,
    pub top_theorem_candidate: Option<MatchedControlResidueMaskTopTheoremCandidate>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MatchedControlResidueMaskTopTheoremCandidateKind {
    UncertifiedPairFingerprint,
    CertifiedFollowOnFingerprint,
}

impl MatchedControlResidueMaskTopTheoremCandidateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UncertifiedPairFingerprint => "uncertified-pair-fingerprint",
            Self::CertifiedFollowOnFingerprint => "certified-follow-on-fingerprint",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchedControlResidueMaskTopTheoremCandidate {
    pub rank: usize,
    pub selection_kind: MatchedControlResidueMaskTopTheoremCandidateKind,
    pub selection_reason: String,
    pub left_family_code: String,
    pub right_family_code: String,
    pub base: u32,
    pub middle_width: usize,
    pub same_boundary_digits: bool,
    pub k_distinction: bool,
    pub rank_bucket: u8,
    pub rank_bucket_label: String,
    pub common_moduli: Vec<u32>,
    pub distinct_excluded_class_count: usize,
    pub overlap_ratio_fraction: String,
    pub proof_status_pair: String,
    pub pair_certified: bool,
    pub separation_theorem: Option<String>,
    pub forbidden_residue_set_theorem: Option<String>,
    pub equal_survivor_theorem: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchedControlResidueMaskLaneRow {
    pub family_label: String,
    pub family_code: String,
    pub lean_lane_constant: String,
    pub proof_status: MatchedControlAtlasProofStatus,
    pub separation_theorem: Option<String>,
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub k_outer: u32,
    pub k_inner: u32,
    pub middle_width: usize,
    pub modulus: u32,
    pub shift_residue: u32,
    pub gradient_residue: u32,
    pub excluded_seed_class: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchedControlResidueMaskPairCandidateRow {
    pub left_family_code: String,
    pub right_family_code: String,
    pub left_lean_lane_constant: String,
    pub right_lean_lane_constant: String,
    pub base: u32,
    pub middle_width: usize,
    pub modulus: u32,
    pub left_excluded_seed_class: u32,
    pub right_excluded_seed_class: u32,
    pub same_boundary_digits: bool,
    pub k_distinction: bool,
    pub rank_bucket: u8,
    pub candidate_reason: String,
    pub left_proof_status: MatchedControlAtlasProofStatus,
    pub right_proof_status: MatchedControlAtlasProofStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchedControlResidueMaskDisplacementRow {
    pub modulus: u32,
    pub left_excluded_seed_class: u32,
    pub right_excluded_seed_class: u32,
    pub excluded_classes_equal: bool,
    pub forward_displacement: u32,
    pub circular_distance: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchedControlResidueMaskPairFingerprintRow {
    pub left_family_label: String,
    pub right_family_label: String,
    pub left_family_code: String,
    pub right_family_code: String,
    pub left_lean_lane_constant: String,
    pub right_lean_lane_constant: String,
    pub base: u32,
    pub left_outer: u32,
    pub left_inner: u32,
    pub right_outer: u32,
    pub right_inner: u32,
    pub left_k_outer: u32,
    pub left_k_inner: u32,
    pub right_k_outer: u32,
    pub right_k_inner: u32,
    pub middle_width: usize,
    pub same_boundary_digits: bool,
    pub k_distinction: bool,
    pub rank_bucket: u8,
    pub rank_bucket_label: String,
    pub common_moduli: Vec<u32>,
    pub common_modulus_count: usize,
    pub equal_excluded_class_count: usize,
    pub distinct_excluded_class_count: usize,
    pub displacements: Vec<MatchedControlResidueMaskDisplacementRow>,
    pub left_individual_survivor_count: String,
    pub right_individual_survivor_count: String,
    pub individual_survivor_counts_equal: bool,
    pub shared_survivor_count: String,
    pub overlap_ratio: f64,
    pub overlap_ratio_fraction: String,
    pub left_proof_status: MatchedControlAtlasProofStatus,
    pub right_proof_status: MatchedControlAtlasProofStatus,
    pub proof_status_pair: String,
    pub pair_certificate: Option<MatchedControlSmokePairCertificateMetadata>,
    pub separation_theorem: Option<String>,
    pub forbidden_residue_set_theorem: Option<String>,
    pub equal_survivor_theorem: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchedControlResidueMaskReport {
    pub schema_version: String,
    pub settings: MatchedControlResidueMaskReportSettings,
    pub summary: MatchedControlResidueMaskSummary,
    pub lane_modulus_rows: Vec<MatchedControlResidueMaskLaneRow>,
    pub pair_candidate_rows: Vec<MatchedControlResidueMaskPairCandidateRow>,
    pub pair_fingerprint_rows: Vec<MatchedControlResidueMaskPairFingerprintRow>,
}

#[derive(Debug, Error)]
pub enum MatchedControlResidueMaskError {
    #[error("prime bound must be at least 2, got {0}")]
    InvalidPrimeBound(u32),
    #[error("failed to build affine lane for {family_code}: {source}")]
    LaneBuild {
        family_code: String,
        source: crate::validation::fast_affine::FastPrimeError,
    },
    #[error("modulus {modulus} is not invertible for lane {family_code}")]
    NonInvertibleGradient { family_code: String, modulus: u32 },
    #[error("I/O error while exporting residue-mask report: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON serialization failed for residue-mask report: {0}")]
    Json(#[from] serde_json::Error),
}

pub fn build_matched_control_residue_mask_report(
    settings: MatchedControlResidueMaskSettings,
) -> Result<MatchedControlResidueMaskReport, MatchedControlResidueMaskError> {
    if settings.prime_bound < 2 {
        return Err(MatchedControlResidueMaskError::InvalidPrimeBound(
            settings.prime_bound,
        ));
    }

    let panel_settings = settings.panel.settings();
    let atlas = build_matched_control_atlas_manifest(settings.panel);
    let proof_status_by_code: BTreeMap<String, MatchedControlAtlasProofStatus> = atlas
        .families
        .iter()
        .map(|row| (row.family_code.clone(), row.proof_status))
        .collect();
    let separation_theorem_by_code: BTreeMap<String, Option<String>> = atlas
        .families
        .iter()
        .map(|row| {
            (
                row.family_code.clone(),
                row.proof_certificate
                    .as_ref()
                    .and_then(|certificate| certificate.separation_theorem.clone()),
            )
        })
        .collect();
    let prime_moduli = primes_up_to(settings.prime_bound);
    let mut lane_count = 0usize;
    let mut lane_rows = Vec::new();

    for family in MAINTAINED_MATCHED_CONTROL_FAMILIES {
        for middle_width in panel_settings.min_seed_len..=panel_settings.max_seed_len {
            let family_code = family.code(middle_width);
            let lean_lane_constant = matched_control_lean_lane_name(&family, middle_width);
            let proof_status = proof_status_by_code
                .get(&family_code)
                .copied()
                .unwrap_or(MatchedControlAtlasProofStatus::LaneGeneratedOnly);
            let separation_theorem = separation_theorem_by_code
                .get(&family_code)
                .cloned()
                .flatten();
            let lane = build_fast_affine_lane(FastLaneConfig::new(
                family.base,
                family.outer,
                family.inner,
                middle_width,
                (family.k_outer, family.k_inner),
            ))
            .map_err(|source| MatchedControlResidueMaskError::LaneBuild {
                family_code: family_code.clone(),
                source,
            })?;

            lane_count += 1;

            for &modulus in &prime_moduli {
                if gcd_u32(family.base, modulus) != 1 {
                    continue;
                }
                let shift_residue = (lane.shift % modulus as u64) as u32;
                let gradient_residue = (lane.gradient % modulus as u64) as u32;
                let inverse = mod_inverse_u32(gradient_residue, modulus).ok_or(
                    MatchedControlResidueMaskError::NonInvertibleGradient {
                        family_code: family_code.clone(),
                        modulus,
                    },
                )?;
                let excluded_seed_class = ((modulus - shift_residue) % modulus * inverse) % modulus;

                lane_rows.push(MatchedControlResidueMaskLaneRow {
                    family_label: family.label.to_string(),
                    family_code: family_code.clone(),
                    lean_lane_constant: lean_lane_constant.clone(),
                    proof_status,
                    separation_theorem: separation_theorem.clone(),
                    base: family.base,
                    outer: family.outer,
                    inner: family.inner,
                    k_outer: family.k_outer,
                    k_inner: family.k_inner,
                    middle_width,
                    modulus,
                    shift_residue,
                    gradient_residue,
                    excluded_seed_class,
                });
            }
        }
    }

    lane_rows.sort_by_key(lane_row_sort_key);
    let mut pair_candidate_rows = build_pair_candidate_rows(&lane_rows);
    pair_candidate_rows.sort_by_key(pair_candidate_sort_key);
    let mut pair_fingerprint_rows = build_pair_fingerprint_rows(&lane_rows);
    pair_fingerprint_rows.sort_by(pair_fingerprint_sort_order);

    let pair_certified_count = pair_fingerprint_rows
        .iter()
        .filter(|row| row.pair_certificate.is_some())
        .count();
    let pair_uncertified_count = pair_fingerprint_rows.len() - pair_certified_count;
    let top_theorem_candidate = build_top_theorem_candidate(&pair_fingerprint_rows);
    let same_boundary_candidate_count = pair_candidate_rows
        .iter()
        .filter(|row| row.same_boundary_digits)
        .count();
    let same_boundary_k_distinction_candidate_count = pair_candidate_rows
        .iter()
        .filter(|row| row.same_boundary_digits && row.k_distinction)
        .count();
    Ok(MatchedControlResidueMaskReport {
        schema_version: MATCHED_CONTROL_RESIDUE_MASK_SCHEMA_VERSION.to_string(),
        settings: MatchedControlResidueMaskReportSettings {
            panel: settings.panel.as_str().to_string(),
            panel_id: settings.panel.panel_id().to_string(),
            prime_bound: settings.prime_bound,
        },
        summary: MatchedControlResidueMaskSummary {
            panel: settings.panel.as_str().to_string(),
            panel_id: settings.panel.panel_id().to_string(),
            prime_bound: settings.prime_bound,
            lane_count,
            lane_modulus_row_count: lane_rows.len(),
            pair_candidate_count: pair_candidate_rows.len(),
            pair_fingerprint_row_count: pair_fingerprint_rows.len(),
            pair_certified_count,
            pair_uncertified_count,
            same_boundary_candidate_count,
            same_boundary_k_distinction_candidate_count,
            top_theorem_candidate,
        },
        lane_modulus_rows: lane_rows,
        pair_candidate_rows,
        pair_fingerprint_rows,
    })
}

pub fn render_matched_control_residue_mask_markdown(
    report: &MatchedControlResidueMaskReport,
) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# Matched-Control Residue-Mask Scanner");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "This deterministic scan ranks exact local forbidden seed-class distinctions. It is not a density or residual-mechanism claim."
    );
    let _ = writeln!(out);
    let _ = writeln!(out, "- panel: `{}`", report.summary.panel);
    let _ = writeln!(out, "- panel id: `{}`", report.summary.panel_id);
    let _ = writeln!(out, "- prime bound: `{}`", report.summary.prime_bound);
    let _ = writeln!(out, "- lanes: `{}`", report.summary.lane_count);
    let _ = writeln!(
        out,
        "- lane/modulus rows: `{}`",
        report.summary.lane_modulus_row_count
    );
    let _ = writeln!(
        out,
        "- pair candidates: `{}`",
        report.summary.pair_candidate_count
    );
    let _ = writeln!(
        out,
        "- pair fingerprints: `{}`",
        report.summary.pair_fingerprint_row_count
    );
    let _ = writeln!(
        out,
        "- pair-certified fingerprints: `{}`",
        report.summary.pair_certified_count
    );
    let _ = writeln!(
        out,
        "- pair-uncertified fingerprints: `{}`",
        report.summary.pair_uncertified_count
    );
    let _ = writeln!(
        out,
        "- same-boundary candidates: `{}`",
        report.summary.same_boundary_candidate_count
    );
    let _ = writeln!(
        out,
        "- same-boundary k-distinction candidates: `{}`",
        report.summary.same_boundary_k_distinction_candidate_count
    );
    if let Some(candidate) = &report.summary.top_theorem_candidate {
        let _ = writeln!(out);
        let _ = writeln!(out, "## Top Theorem Candidate");
        let _ = writeln!(out);
        let _ = writeln!(out, "- rank: `{}`", candidate.rank);
        let _ = writeln!(
            out,
            "- selection kind: `{}`",
            candidate.selection_kind.as_str()
        );
        let _ = writeln!(out, "- selection reason: {}", candidate.selection_reason);
        let _ = writeln!(
            out,
            "- pair: `{}` vs `{}`",
            candidate.left_family_code, candidate.right_family_code
        );
        let _ = writeln!(
            out,
            "- proof links: seed-mask `{}`, residue-set `{}`, equal-survivor `{}`",
            theorem_short_name(candidate.separation_theorem.as_deref()),
            theorem_short_name(candidate.forbidden_residue_set_theorem.as_deref()),
            theorem_short_name(candidate.equal_survivor_theorem.as_deref())
        );
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "## Ranked Pair Fingerprints");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "| rank | base | M | left | right | common moduli | distinct masks | shared / individual | overlap | proof statuses | seed-mask theorem | residue-set theorem | equal-survivor theorem |"
    );
    let _ = writeln!(
        out,
        "|---:|---:|---:|---|---|---:|---:|---|---:|---|---|---|---|"
    );
    for (index, row) in report.pair_fingerprint_rows.iter().take(25).enumerate() {
        let _ = writeln!(
            out,
            "| {} | {} | {} | `{}` | `{}` | {} | {} | `{}` / `{}` | {:.6} | `{}` | `{}` | `{}` | `{}` |",
            index + 1,
            row.base,
            row.middle_width,
            row.left_family_code,
            row.right_family_code,
            row.common_modulus_count,
            row.distinct_excluded_class_count,
            row.shared_survivor_count,
            row.left_individual_survivor_count,
            row.overlap_ratio,
            row.proof_status_pair,
            theorem_short_name(row.separation_theorem.as_deref()),
            theorem_short_name(row.forbidden_residue_set_theorem.as_deref()),
            theorem_short_name(row.equal_survivor_theorem.as_deref()),
        );
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "## Ranked Pair Candidates");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "| rank | modulus | base | M | left | right | classes | reason | proof statuses |"
    );
    let _ = writeln!(out, "|---:|---:|---:|---:|---|---|---|---|---|");
    for (index, row) in report.pair_candidate_rows.iter().take(25).enumerate() {
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} | `{}` | `{}` | `{}` vs `{}` | {} | `{}` / `{}` |",
            index + 1,
            row.modulus,
            row.base,
            row.middle_width,
            row.left_family_code,
            row.right_family_code,
            row.left_excluded_seed_class,
            row.right_excluded_seed_class,
            row.candidate_reason,
            proof_status_label(row.left_proof_status),
            proof_status_label(row.right_proof_status),
        );
    }
    out
}

pub fn render_matched_control_residue_mask_top_candidate_lean_checks(
    report: &MatchedControlResidueMaskReport,
) -> String {
    let mut out = String::new();
    let Some(candidate) = &report.summary.top_theorem_candidate else {
        out.push_str("/-!\n");
        out.push_str("No residue-mask top theorem candidate was selected.\n");
        out.push_str("-/\n");
        return out;
    };

    let theorem_links = [
        ("seed-mask theorem", candidate.separation_theorem.as_deref()),
        (
            "residue-set theorem",
            candidate.forbidden_residue_set_theorem.as_deref(),
        ),
        (
            "equal-survivor theorem",
            candidate.equal_survivor_theorem.as_deref(),
        ),
    ];
    let imports: BTreeSet<&str> = theorem_links
        .iter()
        .filter_map(|(_, theorem)| theorem.and_then(lean_module_path))
        .collect();

    for module in imports {
        let _ = writeln!(out, "import {module}");
    }
    out.push_str("\n/-!\n");
    out.push_str("Lean checks for the current residue-mask top theorem candidate.\n");
    out.push_str("This file is generated from `summary.top_theorem_candidate`; it is a\n");
    out.push_str("proof-catalog drift check, not a density or residual-mechanism claim.\n");
    let _ = writeln!(out, "- panel id: `{}`", report.summary.panel_id);
    let _ = writeln!(out, "- prime bound: `{}`", report.summary.prime_bound);
    let _ = writeln!(
        out,
        "- selected pair: `{}` vs `{}`",
        candidate.left_family_code, candidate.right_family_code
    );
    let _ = writeln!(
        out,
        "- selection kind: `{}`",
        candidate.selection_kind.as_str()
    );
    out.push_str("-/\n\n");

    for (label, theorem) in theorem_links {
        let _ = writeln!(out, "-- {label}");
        if let Some(theorem) = theorem {
            let _ = writeln!(out, "#check {theorem}");
        } else {
            out.push_str("-- no theorem link recorded\n");
        }
        out.push('\n');
    }

    out
}

pub fn render_matched_control_residue_mask_top_candidate_lean_silent_checks(
    report: &MatchedControlResidueMaskReport,
) -> String {
    let mut out = String::new();
    let Some(candidate) = &report.summary.top_theorem_candidate else {
        out.push_str("/-!\n");
        out.push_str("No residue-mask top theorem candidate was selected.\n");
        out.push_str("-/\n");
        return out;
    };

    let theorem_links = [
        ("seed-mask theorem", candidate.separation_theorem.as_deref()),
        (
            "residue-set theorem",
            candidate.forbidden_residue_set_theorem.as_deref(),
        ),
        (
            "equal-survivor theorem",
            candidate.equal_survivor_theorem.as_deref(),
        ),
    ];
    let imports: BTreeSet<&str> = theorem_links
        .iter()
        .filter_map(|(_, theorem)| theorem.and_then(lean_module_path))
        .collect();

    for module in imports {
        let _ = writeln!(out, "import {module}");
    }
    out.push_str("\n/-!\n");
    out.push_str("Silent Lean checks for the current residue-mask top theorem candidate.\n");
    out.push_str("This file is generated from `summary.top_theorem_candidate`; it is a\n");
    out.push_str("proof-catalog drift check, not a density or residual-mechanism claim.\n");
    let _ = writeln!(out, "- panel id: `{}`", report.summary.panel_id);
    let _ = writeln!(out, "- prime bound: `{}`", report.summary.prime_bound);
    let _ = writeln!(
        out,
        "- selected pair: `{}` vs `{}`",
        candidate.left_family_code, candidate.right_family_code
    );
    let _ = writeln!(
        out,
        "- selection kind: `{}`",
        candidate.selection_kind.as_str()
    );
    out.push_str("-/\n\n");

    for (label, theorem) in theorem_links {
        let _ = writeln!(out, "-- {label}");
        if let Some(theorem) = theorem {
            write_silent_lean_declaration_check(&mut out, theorem);
        } else {
            out.push_str("-- no theorem link recorded\n");
        }
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

pub fn render_matched_control_residue_mask_theorem_queue_markdown(
    report: &MatchedControlResidueMaskReport,
) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# Matched-Control Theorem Queue");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "This queue is generated from the residue-mask scanner summary. It is a theorem-planning surface, not a density or residual-mechanism claim."
    );
    let _ = writeln!(out);
    let _ = writeln!(out, "- schema version: `{}`", report.schema_version);
    let _ = writeln!(out, "- panel: `{}`", report.summary.panel);
    let _ = writeln!(out, "- panel id: `{}`", report.summary.panel_id);
    let _ = writeln!(out, "- prime bound: `{}`", report.summary.prime_bound);
    let _ = writeln!(
        out,
        "- pair fingerprints: `{}`",
        report.summary.pair_fingerprint_row_count
    );
    let _ = writeln!(
        out,
        "- pair-certified fingerprints: `{}`",
        report.summary.pair_certified_count
    );
    let _ = writeln!(
        out,
        "- pair-uncertified fingerprints: `{}`",
        report.summary.pair_uncertified_count
    );

    let _ = writeln!(out);
    let _ = writeln!(out, "## Top Candidate");
    let _ = writeln!(out);
    if let Some(candidate) = &report.summary.top_theorem_candidate {
        let _ = writeln!(out, "- rank: `{}`", candidate.rank);
        let _ = writeln!(
            out,
            "- selection kind: `{}`",
            candidate.selection_kind.as_str()
        );
        let _ = writeln!(out, "- selection reason: {}", candidate.selection_reason);
        let _ = writeln!(
            out,
            "- pair: `{}` vs `{}`",
            candidate.left_family_code, candidate.right_family_code
        );
        let _ = writeln!(out, "- base: `{}`", candidate.base);
        let _ = writeln!(out, "- middle width: `{}`", candidate.middle_width);
        let _ = writeln!(
            out,
            "- same boundary digits: `{}`",
            candidate.same_boundary_digits
        );
        let _ = writeln!(
            out,
            "- bounded-k distinction: `{}`",
            candidate.k_distinction
        );
        let _ = writeln!(out, "- rank bucket: `{}`", candidate.rank_bucket);
        let _ = writeln!(
            out,
            "- rank bucket label: `{}`",
            candidate.rank_bucket_label
        );
        let _ = writeln!(
            out,
            "- common moduli: `{}`",
            format_u32_list(&candidate.common_moduli)
        );
        let _ = writeln!(
            out,
            "- distinct excluded-class count: `{}`",
            candidate.distinct_excluded_class_count
        );
        let _ = writeln!(
            out,
            "- overlap ratio fraction: `{}`",
            candidate.overlap_ratio_fraction
        );
        let _ = writeln!(
            out,
            "- proof status pair: `{}`",
            candidate.proof_status_pair
        );
        let _ = writeln!(out, "- pair certified: `{}`", candidate.pair_certified);

        let _ = writeln!(out);
        let _ = writeln!(out, "## Proof Links");
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "- seed-mask separation: {}",
            markdown_theorem(candidate.separation_theorem.as_deref())
        );
        let _ = writeln!(
            out,
            "- finite residue-set separation: {}",
            markdown_theorem(candidate.forbidden_residue_set_theorem.as_deref())
        );
        let _ = writeln!(
            out,
            "- equal survivor count: {}",
            markdown_theorem(candidate.equal_survivor_theorem.as_deref())
        );
    } else {
        let _ = writeln!(out, "No theorem candidate is currently selected.");
    }

    let _ = writeln!(out);
    let _ = writeln!(out, "## Queue Semantics");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "- `uncertified-pair-fingerprint`: add or repair Lean proof metadata before promoting a new exact theorem target."
    );
    let _ = writeln!(
        out,
        "- `certified-follow-on-fingerprint`: proof links already elaborate; use the row as the maintained planning anchor for the next explanatory theorem layer."
    );
    let _ = writeln!(
        out,
        "- The queue ranks exact local mask geometry only. It does not assert a prime-density mechanism."
    );

    out
}

pub fn write_matched_control_residue_mask_report_json(
    path: impl AsRef<Path>,
    report: &MatchedControlResidueMaskReport,
) -> Result<(), MatchedControlResidueMaskError> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, report)?;
    Ok(())
}

pub fn write_matched_control_residue_mask_report_markdown(
    path: impl AsRef<Path>,
    report: &MatchedControlResidueMaskReport,
) -> Result<(), MatchedControlResidueMaskError> {
    let mut file = File::create(path)?;
    file.write_all(render_matched_control_residue_mask_markdown(report).as_bytes())?;
    Ok(())
}

pub fn write_matched_control_residue_mask_theorem_queue_markdown(
    path: impl AsRef<Path>,
    report: &MatchedControlResidueMaskReport,
) -> Result<(), MatchedControlResidueMaskError> {
    let mut file = File::create(path)?;
    file.write_all(render_matched_control_residue_mask_theorem_queue_markdown(report).as_bytes())?;
    Ok(())
}

fn theorem_short_name(theorem: Option<&str>) -> &str {
    theorem
        .and_then(|name| name.rsplit('.').next())
        .unwrap_or("-")
}

fn markdown_theorem(theorem: Option<&str>) -> String {
    theorem
        .map(|name| format!("`{name}`"))
        .unwrap_or_else(|| "`-`".to_string())
}

fn format_u32_list(values: &[u32]) -> String {
    values
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

fn lean_module_path(qualified_theorem: &str) -> Option<&str> {
    qualified_theorem.rsplit_once('.').map(|(module, _)| module)
}

fn build_top_theorem_candidate(
    rows: &[MatchedControlResidueMaskPairFingerprintRow],
) -> Option<MatchedControlResidueMaskTopTheoremCandidate> {
    rows.iter()
        .enumerate()
        .find(|(_, row)| row.pair_certificate.is_none())
        .map(|(index, row)| {
            top_theorem_candidate_from_row(
                index,
                row,
                MatchedControlResidueMaskTopTheoremCandidateKind::UncertifiedPairFingerprint,
            )
        })
        .or_else(|| {
            rows.iter()
                .enumerate()
                .find(|(_, row)| row.distinct_excluded_class_count > 0)
                .map(|(index, row)| {
                    top_theorem_candidate_from_row(
                        index,
                        row,
                        MatchedControlResidueMaskTopTheoremCandidateKind::CertifiedFollowOnFingerprint,
                    )
                })
        })
}

fn top_theorem_candidate_from_row(
    index: usize,
    row: &MatchedControlResidueMaskPairFingerprintRow,
    selection_kind: MatchedControlResidueMaskTopTheoremCandidateKind,
) -> MatchedControlResidueMaskTopTheoremCandidate {
    let selection_reason = match selection_kind {
        MatchedControlResidueMaskTopTheoremCandidateKind::UncertifiedPairFingerprint => {
            "highest-ranked fingerprint without pair-certificate metadata"
        }
        MatchedControlResidueMaskTopTheoremCandidateKind::CertifiedFollowOnFingerprint => {
            "all fingerprints are pair-certified; highest-ranked fingerprint remains the maintained follow-on target"
        }
    };

    MatchedControlResidueMaskTopTheoremCandidate {
        rank: index + 1,
        selection_kind,
        selection_reason: selection_reason.to_string(),
        left_family_code: row.left_family_code.clone(),
        right_family_code: row.right_family_code.clone(),
        base: row.base,
        middle_width: row.middle_width,
        same_boundary_digits: row.same_boundary_digits,
        k_distinction: row.k_distinction,
        rank_bucket: row.rank_bucket,
        rank_bucket_label: row.rank_bucket_label.clone(),
        common_moduli: row.common_moduli.clone(),
        distinct_excluded_class_count: row.distinct_excluded_class_count,
        overlap_ratio_fraction: row.overlap_ratio_fraction.clone(),
        proof_status_pair: row.proof_status_pair.clone(),
        pair_certified: row.pair_certificate.is_some(),
        separation_theorem: row.separation_theorem.clone(),
        forbidden_residue_set_theorem: row.forbidden_residue_set_theorem.clone(),
        equal_survivor_theorem: row.equal_survivor_theorem.clone(),
    }
}

fn build_pair_candidate_rows(
    lane_rows: &[MatchedControlResidueMaskLaneRow],
) -> Vec<MatchedControlResidueMaskPairCandidateRow> {
    let mut by_key: BTreeMap<(u32, usize, u32), Vec<&MatchedControlResidueMaskLaneRow>> =
        BTreeMap::new();
    for row in lane_rows {
        by_key
            .entry((row.base, row.middle_width, row.modulus))
            .or_default()
            .push(row);
    }

    let mut candidates = Vec::new();
    for ((_base, _middle_width, _modulus), rows) in by_key {
        for left_index in 0..rows.len() {
            for right_index in (left_index + 1)..rows.len() {
                let left = rows[left_index];
                let right = rows[right_index];
                if left.excluded_seed_class == right.excluded_seed_class {
                    continue;
                }
                let same_boundary_digits = left.outer == right.outer && left.inner == right.inner;
                let k_distinction = left.k_outer != right.k_outer || left.k_inner != right.k_inner;
                let rank_bucket = match (same_boundary_digits, k_distinction) {
                    (true, true) => 0,
                    (true, false) => 1,
                    _ => 2,
                };

                candidates.push(MatchedControlResidueMaskPairCandidateRow {
                    left_family_code: left.family_code.clone(),
                    right_family_code: right.family_code.clone(),
                    left_lean_lane_constant: left.lean_lane_constant.clone(),
                    right_lean_lane_constant: right.lean_lane_constant.clone(),
                    base: left.base,
                    middle_width: left.middle_width,
                    modulus: left.modulus,
                    left_excluded_seed_class: left.excluded_seed_class,
                    right_excluded_seed_class: right.excluded_seed_class,
                    same_boundary_digits,
                    k_distinction,
                    rank_bucket,
                    candidate_reason: candidate_reason(rank_bucket),
                    left_proof_status: left.proof_status,
                    right_proof_status: right.proof_status,
                });
            }
        }
    }

    candidates
}

fn build_pair_fingerprint_rows(
    lane_rows: &[MatchedControlResidueMaskLaneRow],
) -> Vec<MatchedControlResidueMaskPairFingerprintRow> {
    let mut by_lane_group: BTreeMap<
        (u32, usize),
        BTreeMap<String, BTreeMap<u32, &MatchedControlResidueMaskLaneRow>>,
    > = BTreeMap::new();

    for row in lane_rows {
        by_lane_group
            .entry((row.base, row.middle_width))
            .or_default()
            .entry(row.family_code.clone())
            .or_default()
            .insert(row.modulus, row);
    }

    let mut fingerprints = Vec::new();
    for ((_base, _middle_width), lanes) in by_lane_group {
        let lanes: Vec<_> = lanes.into_values().collect();
        for left_index in 0..lanes.len() {
            for right_index in (left_index + 1)..lanes.len() {
                let left_by_modulus = &lanes[left_index];
                let right_by_modulus = &lanes[right_index];
                let Some(left_representative) = left_by_modulus.values().next().copied() else {
                    continue;
                };
                let Some(right_representative) = right_by_modulus.values().next().copied() else {
                    continue;
                };
                let common_moduli: Vec<u32> = left_by_modulus
                    .keys()
                    .filter(|modulus| right_by_modulus.contains_key(modulus))
                    .copied()
                    .collect();
                if common_moduli.is_empty() {
                    continue;
                }

                let mut equal_excluded_class_count = 0usize;
                let mut distinct_excluded_class_count = 0usize;
                let mut displacements = Vec::with_capacity(common_moduli.len());
                for modulus in &common_moduli {
                    let left = left_by_modulus
                        .get(modulus)
                        .expect("common modulus should exist on left");
                    let right = right_by_modulus
                        .get(modulus)
                        .expect("common modulus should exist on right");
                    let excluded_classes_equal =
                        left.excluded_seed_class == right.excluded_seed_class;
                    if excluded_classes_equal {
                        equal_excluded_class_count += 1;
                    } else {
                        distinct_excluded_class_count += 1;
                    }
                    let forward_displacement =
                        (right.excluded_seed_class + modulus - left.excluded_seed_class) % modulus;
                    let circular_distance =
                        forward_displacement.min((modulus - forward_displacement) % modulus);
                    displacements.push(MatchedControlResidueMaskDisplacementRow {
                        modulus: *modulus,
                        left_excluded_seed_class: left.excluded_seed_class,
                        right_excluded_seed_class: right.excluded_seed_class,
                        excluded_classes_equal,
                        forward_displacement,
                        circular_distance,
                    });
                }

                let individual_survivor_count = individual_survivor_product(&common_moduli);
                let shared_survivor_count = shared_survivor_product(&displacements);
                let same_boundary_digits = left_representative.outer == right_representative.outer
                    && left_representative.inner == right_representative.inner;
                let k_distinction = left_representative.k_outer != right_representative.k_outer
                    || left_representative.k_inner != right_representative.k_inner;
                let rank_bucket = match (same_boundary_digits, k_distinction) {
                    (true, true) => 0,
                    (true, false) => 1,
                    _ => 2,
                };
                let individual_survivor_count_string = individual_survivor_count.to_str_radix(10);
                let shared_survivor_count_string = shared_survivor_count.to_str_radix(10);
                let pair_certificate = matched_control_smoke_pair_certificate_metadata_for(
                    &left_representative.family_code,
                    &right_representative.family_code,
                );
                let separation_theorem = pair_certificate
                    .as_ref()
                    .map(|certificate| certificate.separation_theorem_qualified.clone())
                    .or_else(|| {
                        shared_separation_theorem(
                            &left_representative.separation_theorem,
                            &right_representative.separation_theorem,
                        )
                    });
                let forbidden_residue_set_theorem =
                    pair_certificate.as_ref().and_then(|certificate| {
                        certificate.forbidden_residue_set_theorem_qualified.clone()
                    });
                let equal_survivor_theorem = pair_certificate
                    .as_ref()
                    .map(|certificate| certificate.equal_survivor_theorem_qualified.clone());

                fingerprints.push(MatchedControlResidueMaskPairFingerprintRow {
                    left_family_label: left_representative.family_label.clone(),
                    right_family_label: right_representative.family_label.clone(),
                    left_family_code: left_representative.family_code.clone(),
                    right_family_code: right_representative.family_code.clone(),
                    left_lean_lane_constant: left_representative.lean_lane_constant.clone(),
                    right_lean_lane_constant: right_representative.lean_lane_constant.clone(),
                    base: left_representative.base,
                    left_outer: left_representative.outer,
                    left_inner: left_representative.inner,
                    right_outer: right_representative.outer,
                    right_inner: right_representative.inner,
                    left_k_outer: left_representative.k_outer,
                    left_k_inner: left_representative.k_inner,
                    right_k_outer: right_representative.k_outer,
                    right_k_inner: right_representative.k_inner,
                    middle_width: left_representative.middle_width,
                    same_boundary_digits,
                    k_distinction,
                    rank_bucket,
                    rank_bucket_label: candidate_reason(rank_bucket),
                    common_moduli: common_moduli.clone(),
                    common_modulus_count: common_moduli.len(),
                    equal_excluded_class_count,
                    distinct_excluded_class_count,
                    displacements,
                    left_individual_survivor_count: individual_survivor_count_string.clone(),
                    right_individual_survivor_count: individual_survivor_count_string.clone(),
                    individual_survivor_counts_equal: true,
                    shared_survivor_count: shared_survivor_count_string.clone(),
                    overlap_ratio: biguint_ratio_to_f64(
                        &shared_survivor_count,
                        &individual_survivor_count,
                    ),
                    overlap_ratio_fraction: format!(
                        "{shared_survivor_count_string}/{individual_survivor_count_string}"
                    ),
                    left_proof_status: left_representative.proof_status,
                    right_proof_status: right_representative.proof_status,
                    proof_status_pair: format!(
                        "{}/{}",
                        proof_status_label(left_representative.proof_status),
                        proof_status_label(right_representative.proof_status)
                    ),
                    pair_certificate,
                    separation_theorem,
                    forbidden_residue_set_theorem,
                    equal_survivor_theorem,
                });
            }
        }
    }

    fingerprints
}

fn shared_separation_theorem(left: &Option<String>, right: &Option<String>) -> Option<String> {
    match (left, right) {
        (Some(left), Some(right)) if left == right => Some(left.clone()),
        _ => None,
    }
}

fn individual_survivor_product(moduli: &[u32]) -> BigUint {
    moduli.iter().fold(BigUint::one(), |acc, modulus| {
        acc * BigUint::from(modulus - 1)
    })
}

fn shared_survivor_product(displacements: &[MatchedControlResidueMaskDisplacementRow]) -> BigUint {
    displacements.iter().fold(BigUint::one(), |acc, row| {
        let factor = if row.excluded_classes_equal {
            row.modulus - 1
        } else {
            row.modulus - 2
        };
        acc * BigUint::from(factor)
    })
}

fn biguint_ratio_to_f64(numerator: &BigUint, denominator: &BigUint) -> f64 {
    let numerator = numerator.to_f64().unwrap_or(f64::INFINITY);
    let denominator = denominator.to_f64().unwrap_or(f64::INFINITY);
    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}

fn candidate_reason(rank_bucket: u8) -> String {
    match rank_bucket {
        0 => "same boundary digits; distinct bounded-k profile".to_string(),
        1 => "same boundary digits; distinct seed width or lane identity".to_string(),
        _ => "same base and middle width; distinct forbidden seed class".to_string(),
    }
}

fn lane_row_sort_key(
    row: &MatchedControlResidueMaskLaneRow,
) -> (u32, usize, String, u32, u32, u32) {
    (
        row.base,
        row.middle_width,
        row.family_code.clone(),
        row.modulus,
        row.outer,
        row.inner,
    )
}

fn pair_candidate_sort_key(
    row: &MatchedControlResidueMaskPairCandidateRow,
) -> (u8, u32, usize, u32, String, String) {
    (
        row.rank_bucket,
        row.base,
        row.middle_width,
        row.modulus,
        row.left_family_code.clone(),
        row.right_family_code.clone(),
    )
}

fn pair_fingerprint_sort_order(
    left: &MatchedControlResidueMaskPairFingerprintRow,
    right: &MatchedControlResidueMaskPairFingerprintRow,
) -> Ordering {
    left.rank_bucket
        .cmp(&right.rank_bucket)
        .then_with(|| compare_fingerprint_overlap_ratio(left, right))
        .then_with(|| {
            right
                .distinct_excluded_class_count
                .cmp(&left.distinct_excluded_class_count)
        })
        .then_with(|| left.base.cmp(&right.base))
        .then_with(|| left.middle_width.cmp(&right.middle_width))
        .then_with(|| left.left_family_code.cmp(&right.left_family_code))
        .then_with(|| left.right_family_code.cmp(&right.right_family_code))
        .then_with(|| left.common_moduli.cmp(&right.common_moduli))
}

fn compare_fingerprint_overlap_ratio(
    left: &MatchedControlResidueMaskPairFingerprintRow,
    right: &MatchedControlResidueMaskPairFingerprintRow,
) -> Ordering {
    let left_shared = parse_decimal_biguint(&left.shared_survivor_count);
    let left_individual = parse_decimal_biguint(&left.left_individual_survivor_count);
    let right_shared = parse_decimal_biguint(&right.shared_survivor_count);
    let right_individual = parse_decimal_biguint(&right.left_individual_survivor_count);

    (left_shared * right_individual).cmp(&(right_shared * left_individual))
}

fn parse_decimal_biguint(value: &str) -> BigUint {
    BigUint::parse_bytes(value.as_bytes(), 10)
        .expect("residue-mask exact count fields should contain decimal integers")
}

fn primes_up_to(bound: u32) -> Vec<u32> {
    (2..=bound)
        .filter(|&candidate| is_prime_u32(candidate))
        .collect()
}

fn is_prime_u32(value: u32) -> bool {
    if value < 2 {
        return false;
    }
    if value == 2 {
        return true;
    }
    if value % 2 == 0 {
        return false;
    }
    let mut divisor = 3u32;
    while divisor.saturating_mul(divisor) <= value {
        if value % divisor == 0 {
            return false;
        }
        divisor += 2;
    }
    true
}

fn mod_inverse_u32(value: u32, modulus: u32) -> Option<u32> {
    let mut t = 0i64;
    let mut new_t = 1i64;
    let mut r = modulus as i64;
    let mut new_r = value as i64;

    while new_r != 0 {
        let quotient = r / new_r;
        (t, new_t) = (new_t, t - quotient * new_t);
        (r, new_r) = (new_r, r - quotient * new_r);
    }

    if r != 1 {
        return None;
    }
    if t < 0 {
        t += modulus as i64;
    }
    Some(t as u32)
}

fn gcd_u32(mut left: u32, mut right: u32) -> u32 {
    while right != 0 {
        let tmp = right;
        right = left % right;
        left = tmp;
    }
    left
}

fn proof_status_label(status: MatchedControlAtlasProofStatus) -> &'static str {
    match status {
        MatchedControlAtlasProofStatus::LaneGeneratedOnly => "lane-generated-only",
        MatchedControlAtlasProofStatus::ExactResidueProfile => "exact-residue-profile",
        MatchedControlAtlasProofStatus::ExactSeedClassSeparation => "exact-seed-class-separation",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn smoke_report() -> MatchedControlResidueMaskReport {
        build_matched_control_residue_mask_report(MatchedControlResidueMaskSettings::default())
            .expect("smoke residue-mask report should build")
    }

    fn lane_class(
        report: &MatchedControlResidueMaskReport,
        family_code: &str,
        modulus: u32,
    ) -> u32 {
        report
            .lane_modulus_rows
            .iter()
            .find(|row| row.family_code == family_code && row.modulus == modulus)
            .unwrap_or_else(|| panic!("missing lane row {family_code} mod {modulus}"))
            .excluded_seed_class
    }

    fn fingerprint_row<'a>(
        report: &'a MatchedControlResidueMaskReport,
        left_family_code: &str,
        right_family_code: &str,
    ) -> &'a MatchedControlResidueMaskPairFingerprintRow {
        report
            .pair_fingerprint_rows
            .iter()
            .find(|row| {
                row.left_family_code == left_family_code
                    && row.right_family_code == right_family_code
            })
            .unwrap_or_else(|| {
                panic!("missing fingerprint row {left_family_code} vs {right_family_code}")
            })
    }

    #[test]
    fn matched_control_residue_scanner_reproduces_known_anchors() {
        let report = smoke_report();

        assert_eq!(lane_class(&report, "B 6 ( 1, 5) k=(0,0) M=1", 7), 1);
        assert_eq!(lane_class(&report, "B 6 ( 1, 5) k=(0,0) M=2", 7), 0);
        assert_eq!(lane_class(&report, "B10 ( 3, 7) k=(0,0) M=1", 11), 8);
        assert_eq!(lane_class(&report, "B10 ( 3, 7) k=(1,1) M=1", 11), 2);
    }

    #[test]
    fn matched_control_residue_scanner_emits_deterministic_sorted_rows() {
        let report = smoke_report();
        let mut resorted = report.lane_modulus_rows.clone();
        resorted.sort_by_key(lane_row_sort_key);
        assert_eq!(report.lane_modulus_rows, resorted);

        let mut pair_resorted = report.pair_candidate_rows.clone();
        pair_resorted.sort_by_key(pair_candidate_sort_key);
        assert_eq!(report.pair_candidate_rows, pair_resorted);

        let mut fingerprint_resorted = report.pair_fingerprint_rows.clone();
        fingerprint_resorted.sort_by(pair_fingerprint_sort_order);
        assert_eq!(report.pair_fingerprint_rows, fingerprint_resorted);
    }

    #[test]
    fn matched_control_residue_scanner_ranks_same_boundary_k_distinctions_first() {
        let report = smoke_report();
        let first = report
            .pair_candidate_rows
            .first()
            .expect("smoke report should have pair candidates");

        assert_eq!(first.rank_bucket, 0);
        assert!(first.same_boundary_digits);
        assert!(first.k_distinction);

        let first_fingerprint = report
            .pair_fingerprint_rows
            .first()
            .expect("smoke report should have pair fingerprints");
        assert_eq!(first_fingerprint.rank_bucket, 0);
        assert!(first_fingerprint.same_boundary_digits);
        assert!(first_fingerprint.k_distinction);
    }

    #[test]
    fn matched_control_residue_fingerprints_have_equal_individual_survivor_counts() {
        let report = smoke_report();

        for row in &report.pair_fingerprint_rows {
            assert!(row.individual_survivor_counts_equal);
            assert_eq!(
                row.left_individual_survivor_count,
                row.right_individual_survivor_count
            );
            assert_eq!(
                row.left_individual_survivor_count,
                individual_survivor_product(&row.common_moduli).to_str_radix(10)
            );
            assert_eq!(
                row.shared_survivor_count,
                shared_survivor_product(&row.displacements).to_str_radix(10)
            );
            assert_eq!(
                row.equal_excluded_class_count + row.distinct_excluded_class_count,
                row.common_modulus_count
            );
        }
    }

    #[test]
    fn matched_control_residue_fingerprint_certifies_rank_one_same_boundary_pair() {
        let report = smoke_report();
        let row = fingerprint_row(
            &report,
            "B10 ( 3, 3) k=(0,1) M=1",
            "B10 ( 3, 3) k=(1,1) M=1",
        );

        assert_eq!(row.rank_bucket, 0);
        assert_eq!(
            row.separation_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Density.Base10SeedClassSeparation.forbiddenSeedMask_breathingM1_ne_symmetricM1_mod11"
            )
        );
        assert_eq!(
            row.forbidden_residue_set_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Density.Base10SeedClassSeparation.forbiddenResidues_breathingM1_ne_symmetricM1_mod11"
            )
        );
        assert_eq!(
            row.equal_survivor_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Density.Base10SeedClassSeparation.survivorResidueCount_breathingM1_eq_symmetricM1_mod11"
            )
        );
        let certificate = row
            .pair_certificate
            .as_ref()
            .expect("rank-1 same-boundary target should have pair-certificate metadata");
        assert_eq!(certificate.modulus, 11);
        assert_eq!(certificate.left_excluded_seed_class, 0);
        assert_eq!(certificate.right_excluded_seed_class, 10);
        let mod11 = row
            .displacements
            .iter()
            .find(|displacement| displacement.modulus == 11)
            .expect("rank-1 same-boundary target should include mod11");
        assert_eq!(mod11.left_excluded_seed_class, 0);
        assert_eq!(mod11.right_excluded_seed_class, 10);
    }

    #[test]
    fn matched_control_residue_fingerprint_certifies_breathing_exclusive_cross_boundary_pair() {
        let report = smoke_report();
        let row = fingerprint_row(
            &report,
            "B10 ( 3, 3) k=(0,1) M=1",
            "B10 ( 3, 7) k=(1,1) M=1",
        );

        assert_eq!(row.rank_bucket, 2);
        assert_eq!(
            row.separation_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Density.Base10SeedClassSeparation.forbiddenSeedMask_breathingM1_ne_exclusiveM1_mod11"
            )
        );
        assert_eq!(
            row.equal_survivor_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Density.Base10SeedClassSeparation.survivorResidueCount_breathingM1_eq_exclusiveM1_mod11"
            )
        );
        let certificate = row
            .pair_certificate
            .as_ref()
            .expect("breathing/exclusive M1 target should have pair-certificate metadata");
        assert_eq!(certificate.modulus, 11);
        assert_eq!(certificate.left_excluded_seed_class, 0);
        assert_eq!(certificate.right_excluded_seed_class, 2);
        let mod11 = row
            .displacements
            .iter()
            .find(|displacement| displacement.modulus == 11)
            .expect("breathing/exclusive M1 target should include mod11");
        assert_eq!(mod11.left_excluded_seed_class, 0);
        assert_eq!(mod11.right_excluded_seed_class, 2);
    }

    #[test]
    fn matched_control_residue_fingerprint_certifies_breathing_classic_cross_boundary_pair() {
        let report = smoke_report();
        let row = fingerprint_row(
            &report,
            "B10 ( 3, 3) k=(0,1) M=1",
            "B10 ( 3, 7) k=(0,0) M=1",
        );

        assert_eq!(row.rank_bucket, 2);
        assert_eq!(
            row.separation_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Density.Base10SeedClassSeparation.forbiddenSeedMask_breathingM1_ne_classicM1_mod11"
            )
        );
        assert_eq!(
            row.equal_survivor_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Density.Base10SeedClassSeparation.survivorResidueCount_breathingM1_eq_classicM1_mod11"
            )
        );
        let certificate = row
            .pair_certificate
            .as_ref()
            .expect("breathing/classic M1 target should have pair-certificate metadata");
        assert_eq!(certificate.modulus, 11);
        assert_eq!(certificate.left_excluded_seed_class, 0);
        assert_eq!(certificate.right_excluded_seed_class, 8);
        let mod11 = row
            .displacements
            .iter()
            .find(|displacement| displacement.modulus == 11)
            .expect("breathing/classic M1 target should include mod11");
        assert_eq!(mod11.left_excluded_seed_class, 0);
        assert_eq!(mod11.right_excluded_seed_class, 8);
    }

    #[test]
    fn matched_control_residue_fingerprint_certifies_base10_m2_cross_boundary_pairs() {
        let report = smoke_report();
        let cases = [
            (
                "B10 ( 3, 3) k=(0,1) M=2",
                "B10 ( 3, 7) k=(0,0) M=2",
                7,
                3,
                5,
                "PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates.forbiddenSeedMask_base10BreathingM2_ne_base10ClassicM2_mod7",
                "PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates.survivorResidueCount_base10BreathingM2_eq_base10ClassicM2_mod7",
            ),
            (
                "B10 ( 3, 3) k=(1,1) M=2",
                "B10 ( 3, 7) k=(1,1) M=2",
                7,
                5,
                0,
                "PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates.forbiddenSeedMask_base10SymmetricM2_ne_base10ExclusiveM2_mod7",
                "PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates.survivorResidueCount_base10SymmetricM2_eq_base10ExclusiveM2_mod7",
            ),
            (
                "B10 ( 3, 3) k=(0,1) M=2",
                "B10 ( 3, 7) k=(1,1) M=2",
                7,
                3,
                0,
                "PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates.forbiddenSeedMask_base10BreathingM2_ne_base10ExclusiveM2_mod7",
                "PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates.survivorResidueCount_base10BreathingM2_eq_base10ExclusiveM2_mod7",
            ),
            (
                "B10 ( 3, 3) k=(1,1) M=2",
                "B10 ( 3, 7) k=(0,0) M=2",
                3,
                0,
                1,
                "PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates.forbiddenSeedMask_base10SymmetricM2_ne_base10ClassicM2_mod3",
                "PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates.survivorResidueCount_base10SymmetricM2_eq_base10ClassicM2_mod3",
            ),
        ];

        for (
            left_family_code,
            right_family_code,
            modulus,
            left_excluded_seed_class,
            right_excluded_seed_class,
            separation_theorem,
            equal_survivor_theorem,
        ) in cases
        {
            let row = fingerprint_row(&report, left_family_code, right_family_code);

            assert_eq!(row.rank_bucket, 2);
            assert_eq!(row.separation_theorem.as_deref(), Some(separation_theorem));
            assert_eq!(
                row.equal_survivor_theorem.as_deref(),
                Some(equal_survivor_theorem)
            );
            let certificate = row
                .pair_certificate
                .as_ref()
                .expect("M2 cross-boundary target should have pair-certificate metadata");
            assert_eq!(certificate.modulus, modulus);
            assert_eq!(
                certificate.left_excluded_seed_class,
                left_excluded_seed_class
            );
            assert_eq!(
                certificate.right_excluded_seed_class,
                right_excluded_seed_class
            );
            let displacement = row
                .displacements
                .iter()
                .find(|displacement| displacement.modulus == modulus)
                .expect("M2 cross-boundary target should include certificate modulus");
            assert_eq!(
                displacement.left_excluded_seed_class,
                left_excluded_seed_class
            );
            assert_eq!(
                displacement.right_excluded_seed_class,
                right_excluded_seed_class
            );
        }
    }

    #[test]
    fn matched_control_residue_fingerprint_certifies_classic_m1_same_boundary_pair() {
        let report = smoke_report();
        let row = fingerprint_row(
            &report,
            "B10 ( 3, 7) k=(0,0) M=1",
            "B10 ( 3, 7) k=(1,1) M=1",
        );

        assert_eq!(row.rank_bucket, 0);
        assert_eq!(
            row.separation_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Density.Base10SeedClassSeparation.forbiddenSeedMask_classicM1_ne_exclusiveM1_mod11"
            )
        );
        assert_eq!(
            row.equal_survivor_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Density.Base10SeedClassSeparation.survivorResidueCount_classicM1_eq_exclusiveM1_mod11"
            )
        );
        let certificate = row
            .pair_certificate
            .as_ref()
            .expect("classic/exclusive M1 target should have pair-certificate metadata");
        assert_eq!(certificate.modulus, 11);
        assert_eq!(certificate.left_excluded_seed_class, 8);
        assert_eq!(certificate.right_excluded_seed_class, 2);
        let mod11 = row
            .displacements
            .iter()
            .find(|displacement| displacement.modulus == 11)
            .expect("classic/exclusive M1 target should include mod11");
        assert_eq!(mod11.left_excluded_seed_class, 8);
        assert_eq!(mod11.right_excluded_seed_class, 2);
    }

    #[test]
    fn matched_control_residue_fingerprint_anchor_for_base10_m2_pair() {
        let report = smoke_report();
        let row = fingerprint_row(
            &report,
            "B10 ( 3, 3) k=(0,1) M=2",
            "B10 ( 3, 3) k=(1,1) M=2",
        );

        assert_eq!(
            row.left_proof_status,
            MatchedControlAtlasProofStatus::ExactResidueProfile
        );
        assert_eq!(
            row.right_proof_status,
            MatchedControlAtlasProofStatus::ExactResidueProfile
        );
        assert_eq!(
            row.separation_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates.forbiddenSeedMask_base10BreathingM2_ne_base10SymmetricM2_mod7"
            )
        );
        assert_eq!(
            row.forbidden_residue_set_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates.forbiddenResidues_base10BreathingM2_ne_base10SymmetricM2_mod7"
            )
        );
        assert_eq!(
            row.equal_survivor_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates.survivorResidueCount_base10BreathingM2_eq_base10SymmetricM2_mod7"
            )
        );
        assert!(row.pair_certificate.is_some());
        assert!(row.individual_survivor_counts_equal);
        assert_eq!(row.rank_bucket, 0);
        let mod7 = row
            .displacements
            .iter()
            .find(|displacement| displacement.modulus == 7)
            .expect("base10 M2 fingerprint should include mod7");
        assert_eq!(mod7.left_excluded_seed_class, 3);
        assert_eq!(mod7.right_excluded_seed_class, 5);
        assert!(!mod7.excluded_classes_equal);
        assert_eq!(mod7.forward_displacement, 2);
        assert_eq!(mod7.circular_distance, 2);
    }

    #[test]
    fn matched_control_residue_fingerprint_certifies_classic_m2_pair() {
        let report = smoke_report();
        let row = fingerprint_row(
            &report,
            "B10 ( 3, 7) k=(0,0) M=2",
            "B10 ( 3, 7) k=(1,1) M=2",
        );

        assert_eq!(
            row.left_proof_status,
            MatchedControlAtlasProofStatus::ExactResidueProfile
        );
        assert_eq!(
            row.right_proof_status,
            MatchedControlAtlasProofStatus::ExactResidueProfile
        );
        assert_eq!(
            row.separation_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates.forbiddenSeedMask_base10ClassicM2_ne_base10ExclusiveM2_mod7"
            )
        );
        assert_eq!(
            row.equal_survivor_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates.survivorResidueCount_base10ClassicM2_eq_base10ExclusiveM2_mod7"
            )
        );
        assert!(row.pair_certificate.is_some());
        let mod7 = row
            .displacements
            .iter()
            .find(|displacement| displacement.modulus == 7)
            .expect("classic/exclusive M2 fingerprint should include mod7");
        assert_eq!(mod7.left_excluded_seed_class, 5);
        assert_eq!(mod7.right_excluded_seed_class, 0);
    }

    #[test]
    fn matched_control_residue_fingerprint_certifies_lowest_overlap_cross_boundary_pair() {
        let report = smoke_report();
        let row = fingerprint_row(
            &report,
            "B10 ( 3, 3) k=(1,1) M=1",
            "B10 ( 3, 7) k=(1,1) M=1",
        );

        assert_eq!(row.rank_bucket, 2);
        assert_eq!(
            row.separation_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Density.Base10SeedClassSeparation.forbiddenSeedMask_symmetricM1_ne_exclusiveM1_mod11"
            )
        );
        assert_eq!(
            row.equal_survivor_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Density.Base10SeedClassSeparation.survivorResidueCount_symmetricM1_eq_exclusiveM1_mod11"
            )
        );
        let certificate = row
            .pair_certificate
            .as_ref()
            .expect("cross-boundary target should have pair-certificate metadata");
        assert_eq!(certificate.modulus, 11);
        assert_eq!(certificate.left_excluded_seed_class, 10);
        assert_eq!(certificate.right_excluded_seed_class, 2);
        let mod11 = row
            .displacements
            .iter()
            .find(|displacement| displacement.modulus == 11)
            .expect("cross-boundary target should include mod11");
        assert_eq!(mod11.left_excluded_seed_class, 10);
        assert_eq!(mod11.right_excluded_seed_class, 2);
    }

    #[test]
    fn matched_control_residue_fingerprints_have_no_unresolved_same_boundary_pairs() {
        let report = smoke_report();
        assert!(report.pair_fingerprint_rows.iter().all(|row| {
            row.rank_bucket != 0
                || (row.left_proof_status != MatchedControlAtlasProofStatus::LaneGeneratedOnly
                    && row.right_proof_status != MatchedControlAtlasProofStatus::LaneGeneratedOnly)
        }));
    }

    #[test]
    fn matched_control_residue_fingerprints_expose_top_level_forbidden_residue_set_links() {
        let report = smoke_report();
        for row in report
            .pair_fingerprint_rows
            .iter()
            .filter(|row| row.pair_certificate.is_some())
        {
            let certificate = row
                .pair_certificate
                .as_ref()
                .expect("certified fingerprint should carry pair metadata");
            assert_eq!(
                row.forbidden_residue_set_theorem.as_deref(),
                certificate
                    .forbidden_residue_set_theorem_qualified
                    .as_deref()
            );
            assert!(row.forbidden_residue_set_theorem.is_some());
        }
    }

    #[test]
    fn matched_control_residue_markdown_surfaces_pair_proof_links() {
        let report = smoke_report();
        let markdown = render_matched_control_residue_mask_markdown(&report);

        assert!(markdown.contains("seed-mask theorem"));
        assert!(markdown.contains("residue-set theorem"));
        assert!(markdown.contains("equal-survivor theorem"));
        assert!(markdown.contains("## Top Theorem Candidate"));
        assert!(markdown.contains("selection kind: `certified-follow-on-fingerprint`"));
        assert!(markdown.contains("forbiddenSeedMask_breathingM1_ne_symmetricM1_mod11"));
        assert!(markdown.contains("forbiddenResidues_breathingM1_ne_symmetricM1_mod11"));
        assert!(markdown.contains("survivorResidueCount_breathingM1_eq_symmetricM1_mod11"));
    }

    #[test]
    fn matched_control_residue_top_candidate_lean_checks_follow_summary_selection() {
        let report = smoke_report();
        let checks = render_matched_control_residue_mask_top_candidate_lean_checks(&report);

        assert!(checks.contains("import PrimeArithmetic.Density.Base10SeedClassSeparation"));
        assert!(checks.contains("summary.top_theorem_candidate"));
        assert!(checks
            .contains("selected pair: `B10 ( 3, 3) k=(0,1) M=1` vs `B10 ( 3, 3) k=(1,1) M=1`"));
        assert!(checks.contains("#check PrimeArithmetic.Density.Base10SeedClassSeparation.forbiddenSeedMask_breathingM1_ne_symmetricM1_mod11"));
        assert!(checks.contains("#check PrimeArithmetic.Density.Base10SeedClassSeparation.forbiddenResidues_breathingM1_ne_symmetricM1_mod11"));
        assert!(checks.contains("#check PrimeArithmetic.Density.Base10SeedClassSeparation.survivorResidueCount_breathingM1_eq_symmetricM1_mod11"));

        let silent_checks =
            render_matched_control_residue_mask_top_candidate_lean_silent_checks(&report);
        assert!(silent_checks.contains("import PrimeArithmetic.Density.Base10SeedClassSeparation"));
        assert!(silent_checks.contains("summary.top_theorem_candidate"));
        assert!(!silent_checks.contains("#check "));
        assert!(silent_checks.contains("example : True := by"));
        assert!(silent_checks.contains("have _ := PrimeArithmetic.Density.Base10SeedClassSeparation.forbiddenSeedMask_breathingM1_ne_symmetricM1_mod11"));
        assert!(silent_checks.contains("have _ := PrimeArithmetic.Density.Base10SeedClassSeparation.forbiddenResidues_breathingM1_ne_symmetricM1_mod11"));
        assert!(silent_checks.contains("have _ := PrimeArithmetic.Density.Base10SeedClassSeparation.survivorResidueCount_breathingM1_eq_symmetricM1_mod11"));
    }

    #[test]
    fn matched_control_residue_theorem_queue_surfaces_summary_candidate() {
        let report = smoke_report();
        let queue = render_matched_control_residue_mask_theorem_queue_markdown(&report);

        assert!(queue.contains("# Matched-Control Theorem Queue"));
        assert!(queue.contains("schema version: `matched-control-residue-masks-v4`"));
        assert!(queue.contains("selection kind: `certified-follow-on-fingerprint`"));
        assert!(queue.contains("pair: `B10 ( 3, 3) k=(0,1) M=1` vs `B10 ( 3, 3) k=(1,1) M=1`"));
        assert!(queue.contains("common moduli: `3, 7, 11, 13, 17, 19, 23, 29, 31`"));
        assert!(queue.contains("seed-mask separation: `PrimeArithmetic.Density.Base10SeedClassSeparation.forbiddenSeedMask_breathingM1_ne_symmetricM1_mod11`"));
        assert!(queue.contains("finite residue-set separation: `PrimeArithmetic.Density.Base10SeedClassSeparation.forbiddenResidues_breathingM1_ne_symmetricM1_mod11`"));
        assert!(queue.contains("equal survivor count: `PrimeArithmetic.Density.Base10SeedClassSeparation.survivorResidueCount_breathingM1_eq_symmetricM1_mod11`"));
        assert!(queue.contains("It does not assert a prime-density mechanism."));
    }

    #[test]
    fn matched_control_residue_summary_selects_top_theorem_candidate() {
        let report = smoke_report();
        let candidate = report
            .summary
            .top_theorem_candidate
            .as_ref()
            .expect("smoke scanner should select a theorem candidate");

        assert_eq!(candidate.rank, 1);
        assert_eq!(
            candidate.selection_kind,
            MatchedControlResidueMaskTopTheoremCandidateKind::CertifiedFollowOnFingerprint
        );
        assert!(candidate.pair_certified);
        assert_eq!(candidate.left_family_code, "B10 ( 3, 3) k=(0,1) M=1");
        assert_eq!(candidate.right_family_code, "B10 ( 3, 3) k=(1,1) M=1");
        assert_eq!(candidate.rank_bucket, 0);
        assert!(candidate.same_boundary_digits);
        assert!(candidate.k_distinction);
        assert_eq!(candidate.common_moduli.len(), 9);
        assert_eq!(candidate.distinct_excluded_class_count, 8);
        assert_eq!(candidate.overlap_ratio_fraction, "4151035350/7664025600");
        assert_eq!(
            candidate.separation_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Density.Base10SeedClassSeparation.forbiddenSeedMask_breathingM1_ne_symmetricM1_mod11"
            )
        );
        assert_eq!(
            candidate.forbidden_residue_set_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Density.Base10SeedClassSeparation.forbiddenResidues_breathingM1_ne_symmetricM1_mod11"
            )
        );
        assert_eq!(
            candidate.equal_survivor_theorem.as_deref(),
            Some(
                "PrimeArithmetic.Density.Base10SeedClassSeparation.survivorResidueCount_breathingM1_eq_symmetricM1_mod11"
            )
        );
    }

    #[test]
    fn matched_control_residue_scanner_marks_current_proof_statuses() {
        let report = smoke_report();
        let base6_m2 = report
            .lane_modulus_rows
            .iter()
            .find(|row| row.family_code == "B 6 ( 1, 5) k=(0,0) M=2" && row.modulus == 7)
            .expect("base6 M2 mod7 row");
        let base10_breathing_m2 = report
            .lane_modulus_rows
            .iter()
            .find(|row| row.family_code == "B10 ( 3, 3) k=(0,1) M=2" && row.modulus == 7)
            .expect("base10 breathing M2 mod7 row");
        let base10_symmetric_m2 = report
            .lane_modulus_rows
            .iter()
            .find(|row| row.family_code == "B10 ( 3, 3) k=(1,1) M=2" && row.modulus == 7)
            .expect("base10 symmetric M2 mod7 row");
        let base10_classic_m2 = report
            .lane_modulus_rows
            .iter()
            .find(|row| row.family_code == "B10 ( 3, 7) k=(0,0) M=2" && row.modulus == 7)
            .expect("base10 classic M2 mod7 row");
        let base10_exclusive_m2 = report
            .lane_modulus_rows
            .iter()
            .find(|row| row.family_code == "B10 ( 3, 7) k=(1,1) M=2" && row.modulus == 7)
            .expect("base10 exclusive M2 mod7 row");

        assert_eq!(
            base6_m2.proof_status,
            MatchedControlAtlasProofStatus::ExactResidueProfile
        );
        assert_eq!(
            base10_breathing_m2.proof_status,
            MatchedControlAtlasProofStatus::ExactResidueProfile
        );
        assert_eq!(
            base10_symmetric_m2.proof_status,
            MatchedControlAtlasProofStatus::ExactResidueProfile
        );
        assert_eq!(
            base10_classic_m2.proof_status,
            MatchedControlAtlasProofStatus::ExactResidueProfile
        );
        assert_eq!(
            base10_exclusive_m2.proof_status,
            MatchedControlAtlasProofStatus::ExactResidueProfile
        );
    }

    #[test]
    fn matched_control_residue_scanner_serializes_deterministically() {
        let report = smoke_report();
        let encoded = serde_json::to_string(&report).expect("report should serialize");
        let decoded: MatchedControlResidueMaskReport =
            serde_json::from_str(&encoded).expect("report should deserialize");

        assert_eq!(
            report.schema_version,
            MATCHED_CONTROL_RESIDUE_MASK_SCHEMA_VERSION
        );
        assert_eq!(report.schema_version, "matched-control-residue-masks-v4");
        assert_eq!(report.settings.panel, "smoke");
        assert_eq!(report.settings.prime_bound, 31);
        assert_eq!(
            report.summary.pair_fingerprint_row_count,
            report.pair_fingerprint_rows.len()
        );
        assert_eq!(
            report.summary.pair_certified_count,
            report
                .pair_fingerprint_rows
                .iter()
                .filter(|row| row.pair_certificate.is_some())
                .count()
        );
        assert_eq!(
            report.summary.pair_uncertified_count,
            report
                .pair_fingerprint_rows
                .iter()
                .filter(|row| row.pair_certificate.is_none())
                .count()
        );
        assert_eq!(report.summary.pair_certified_count, 12);
        assert_eq!(report.summary.pair_uncertified_count, 0);
        assert!(encoded.contains("\"pair_certified_count\":12"));
        assert!(encoded.contains("\"pair_uncertified_count\":0"));
        assert!(encoded.contains("\"top_theorem_candidate\""));
        assert!(encoded.contains("\"selection_kind\":\"certified-follow-on-fingerprint\""));
        assert!(encoded.contains("\"forbidden_residue_set_theorem\""));
        assert_eq!(decoded.schema_version, report.schema_version);
        assert_eq!(decoded.summary, report.summary);
        assert_eq!(decoded.lane_modulus_rows, report.lane_modulus_rows);
        assert_eq!(decoded.pair_candidate_rows, report.pair_candidate_rows);
        assert_eq!(
            decoded.pair_fingerprint_rows[0].left_individual_survivor_count,
            report.pair_fingerprint_rows[0].left_individual_survivor_count
        );
        assert_eq!(
            decoded.pair_fingerprint_rows[0].shared_survivor_count,
            report.pair_fingerprint_rows[0].shared_survivor_count
        );
    }
}
