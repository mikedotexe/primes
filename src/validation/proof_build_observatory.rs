//! Local timing-atlas support for proof-build gates.
//!
//! These reports are cache-aware performance observations. They are intended
//! for local hardening decisions, not for tracked mathematical claims.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub const PROOF_BUILD_OBSERVATORY_SCHEMA_VERSION: &str = "proof-build-observatory-v1";
pub const PROOF_BUILD_OBSERVATORY_MANIFEST_SCHEMA_VERSION: &str =
    "proof-build-observatory-artifact-manifest-v1";

#[derive(Debug, Clone)]
pub struct ProofBuildTimingSource {
    pub suite: String,
    pub path: PathBuf,
    pub report: ProofBuildTimingReportInput,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofBuildTimingReportInput {
    pub schema_version: String,
    pub command: String,
    pub repeat_count: usize,
    #[serde(default)]
    pub rows: Vec<ProofBuildTimingRawRow>,
    #[serde(default)]
    pub run_totals: Vec<ProofBuildTimingRunTotal>,
    pub summary_rows: Vec<ProofBuildTimingSummaryInputRow>,
    #[serde(default)]
    pub total_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofBuildTimingRawRow {
    pub run_index: usize,
    pub step: String,
    pub duration_ms: u64,
    pub status: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofBuildTimingRunTotal {
    pub run_index: usize,
    pub total_duration_ms: u64,
    pub status: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofBuildTimingSummaryInputRow {
    pub step: String,
    pub count: usize,
    pub min_duration_ms: u64,
    pub median_duration_ms: Value,
    pub max_duration_ms: u64,
    pub status: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofBuildObservatoryReport {
    pub schema_version: String,
    pub repeat_count: usize,
    pub raw_reports: Vec<ProofBuildObservatoryRawReport>,
    pub summary: ProofBuildObservatorySummary,
    pub target_rows: Vec<ProofBuildObservatoryTargetRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofBuildObservatoryRawReport {
    pub suite: String,
    pub path: String,
    pub schema_version: String,
    pub command: String,
    pub repeat_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofBuildObservatorySummary {
    pub suite_count: usize,
    pub target_count: usize,
    pub failed_target_count: usize,
    pub max_median_duration_ms: f64,
    pub max_relative_spread: f64,
    pub highest_sustained_cost_target: Option<String>,
    pub highest_volatility_target: Option<String>,
    pub highest_combined_score_target: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofBuildObservatoryTargetRow {
    pub suite: String,
    pub target: String,
    pub target_kind: String,
    pub repeat_count: usize,
    pub count: usize,
    pub min_duration_ms: u64,
    pub median_duration_ms: f64,
    pub max_duration_ms: u64,
    pub absolute_spread_ms: u64,
    pub relative_spread: f64,
    pub status: i32,
    pub sustained_cost_rank: usize,
    pub volatility_rank: usize,
    pub combined_rank: usize,
    pub combined_score: f64,
    pub action_hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofBuildObservatoryArtifactManifest {
    pub schema_version: String,
    pub report_schema_version: String,
    pub artifacts: Vec<ProofBuildObservatoryArtifactRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofBuildObservatoryArtifactRow {
    pub kind: String,
    pub path: String,
    pub description: String,
}

pub fn read_timing_source(
    suite: &str,
    path: impl AsRef<Path>,
) -> Result<ProofBuildTimingSource, String> {
    let path = path.as_ref();
    let raw = fs::read_to_string(path)
        .map_err(|err| format!("failed to read timing report {}: {err}", path.display()))?;
    let report: ProofBuildTimingReportInput = serde_json::from_str(&raw)
        .map_err(|err| format!("failed to parse timing report {}: {err}", path.display()))?;
    Ok(ProofBuildTimingSource {
        suite: suite.to_string(),
        path: path.to_path_buf(),
        report,
    })
}

pub fn build_proof_build_observatory_report(
    sources: &[ProofBuildTimingSource],
) -> Result<ProofBuildObservatoryReport, String> {
    if sources.is_empty() {
        return Err("at least one timing source is required".to_string());
    }

    let repeat_count = sources[0].report.repeat_count;
    for source in sources {
        if source.report.repeat_count != repeat_count {
            return Err(format!(
                "mixed repeat counts are not supported: expected {repeat_count}, got {} for {}",
                source.report.repeat_count,
                source.path.display()
            ));
        }
    }

    let raw_reports = sources
        .iter()
        .map(|source| ProofBuildObservatoryRawReport {
            suite: source.suite.clone(),
            path: source.path.display().to_string(),
            schema_version: source.report.schema_version.clone(),
            command: source.report.command.clone(),
            repeat_count: source.report.repeat_count,
        })
        .collect();

    let mut target_rows = Vec::new();
    for source in sources {
        for summary in &source.report.summary_rows {
            let median_duration_ms = duration_value_as_f64(&summary.median_duration_ms)?;
            let absolute_spread_ms = summary
                .max_duration_ms
                .saturating_sub(summary.min_duration_ms);
            let relative_spread = if median_duration_ms > 0.0 {
                absolute_spread_ms as f64 / median_duration_ms
            } else {
                0.0
            };
            let target_kind = infer_target_kind(&source.suite, &summary.step).to_string();
            let combined_score = median_duration_ms * (1.0 + relative_spread);

            target_rows.push(ProofBuildObservatoryTargetRow {
                suite: source.suite.clone(),
                target: summary.step.clone(),
                target_kind,
                repeat_count,
                count: summary.count,
                min_duration_ms: summary.min_duration_ms,
                median_duration_ms,
                max_duration_ms: summary.max_duration_ms,
                absolute_spread_ms,
                relative_spread,
                status: summary.status,
                sustained_cost_rank: 0,
                volatility_rank: 0,
                combined_rank: 0,
                combined_score,
                action_hint: String::new(),
            });
        }
    }

    assign_rank(&mut target_rows, RankingKind::SustainedCost);
    assign_rank(&mut target_rows, RankingKind::Volatility);
    assign_rank(&mut target_rows, RankingKind::Combined);
    for row in &mut target_rows {
        row.action_hint = action_hint(row).to_string();
    }
    target_rows.sort_by(|left, right| {
        left.combined_rank
            .cmp(&right.combined_rank)
            .then_with(|| left.suite.cmp(&right.suite))
            .then_with(|| left.target.cmp(&right.target))
    });

    let summary = build_summary(&target_rows);
    Ok(ProofBuildObservatoryReport {
        schema_version: PROOF_BUILD_OBSERVATORY_SCHEMA_VERSION.to_string(),
        repeat_count,
        raw_reports,
        summary,
        target_rows,
    })
}

pub fn build_proof_build_observatory_manifest(
    out_json: impl AsRef<Path>,
    out_md: impl AsRef<Path>,
    raw_reports: &[(String, PathBuf)],
) -> ProofBuildObservatoryArtifactManifest {
    let mut artifacts = vec![
        ProofBuildObservatoryArtifactRow {
            kind: "source-of-truth-json".to_string(),
            path: out_json.as_ref().display().to_string(),
            description: "Normalized proof-build timing atlas.".to_string(),
        },
        ProofBuildObservatoryArtifactRow {
            kind: "human-summary".to_string(),
            path: out_md.as_ref().display().to_string(),
            description: "Markdown summary of sustained-cost and volatility rankings.".to_string(),
        },
    ];
    for (suite, path) in raw_reports {
        artifacts.push(ProofBuildObservatoryArtifactRow {
            kind: format!("raw-{suite}-timing"),
            path: path.display().to_string(),
            description: format!("Raw repeated timing JSON for {suite}."),
        });
    }
    ProofBuildObservatoryArtifactManifest {
        schema_version: PROOF_BUILD_OBSERVATORY_MANIFEST_SCHEMA_VERSION.to_string(),
        report_schema_version: PROOF_BUILD_OBSERVATORY_SCHEMA_VERSION.to_string(),
        artifacts,
    }
}

pub fn render_proof_build_observatory_markdown(report: &ProofBuildObservatoryReport) -> String {
    let mut lines = vec![
        "# Proof-Build Observatory".to_string(),
        String::new(),
        "- Scope: local cache-aware proof-build timing observations.".to_string(),
        "- Interpretation: use for engineering prioritization, not benchmark claims.".to_string(),
        format!("- Repeats per suite: `{}`", report.repeat_count),
        format!("- Suites: `{}`", report.summary.suite_count),
        format!("- Targets: `{}`", report.summary.target_count),
        format!("- Failed targets: `{}`", report.summary.failed_target_count),
        String::new(),
        "## Sustained Cost".to_string(),
        String::new(),
        "| Rank | Suite | Target | Median ms | Spread ms | Hint |".to_string(),
        "|---:|---|---|---:|---:|---|".to_string(),
    ];

    let mut sustained = report.target_rows.clone();
    sustained.sort_by(|left, right| left.sustained_cost_rank.cmp(&right.sustained_cost_rank));
    for row in sustained.iter().take(10) {
        lines.push(format!(
            "| {} | `{}` | `{}` | {} | {} | `{}` |",
            row.sustained_cost_rank,
            row.suite,
            row.target,
            fmt_f64(row.median_duration_ms),
            row.absolute_spread_ms,
            row.action_hint
        ));
    }

    lines.extend([
        String::new(),
        "## Volatility".to_string(),
        String::new(),
        "| Rank | Suite | Target | Relative spread | Spread ms | Median ms | Hint |".to_string(),
        "|---:|---|---|---:|---:|---:|---|".to_string(),
    ]);
    let mut volatile = report.target_rows.clone();
    volatile.sort_by(|left, right| left.volatility_rank.cmp(&right.volatility_rank));
    for row in volatile.iter().take(10) {
        lines.push(format!(
            "| {} | `{}` | `{}` | {} | {} | {} | `{}` |",
            row.volatility_rank,
            row.suite,
            row.target,
            fmt_f64(row.relative_spread),
            row.absolute_spread_ms,
            fmt_f64(row.median_duration_ms),
            row.action_hint
        ));
    }

    lines.extend([
        String::new(),
        "## Combined Priority".to_string(),
        String::new(),
        "| Rank | Suite | Target | Score | Median ms | Relative spread | Hint |".to_string(),
        "|---:|---|---|---:|---:|---:|---|".to_string(),
    ]);
    let mut combined = report.target_rows.clone();
    combined.sort_by(|left, right| left.combined_rank.cmp(&right.combined_rank));
    for row in combined.iter().take(10) {
        lines.push(format!(
            "| {} | `{}` | `{}` | {} | {} | {} | `{}` |",
            row.combined_rank,
            row.suite,
            row.target,
            fmt_f64(row.combined_score),
            fmt_f64(row.median_duration_ms),
            fmt_f64(row.relative_spread),
            row.action_hint
        ));
    }

    let failed: Vec<_> = report
        .target_rows
        .iter()
        .filter(|row| row.status != 0)
        .collect();
    lines.extend([String::new(), "## Failures".to_string(), String::new()]);
    if failed.is_empty() {
        lines.push("- None.".to_string());
    } else {
        for row in failed {
            lines.push(format!(
                "- `{}/{}` exited with status `{}`.",
                row.suite, row.target, row.status
            ));
        }
    }

    lines.push(String::new());
    lines.join("\n")
}

fn duration_value_as_f64(value: &Value) -> Result<f64, String> {
    value
        .as_f64()
        .ok_or_else(|| format!("duration value is not numeric: {value}"))
}

fn infer_target_kind(suite: &str, target: &str) -> &'static str {
    if target == "total" {
        "suite-total"
    } else if target.starts_with("lake-build:") || suite == "lean-umbrella" {
        "lake-target"
    } else if target.contains("lean-check") || target.contains("candidate-check") {
        "lean-check"
    } else if target.starts_with("export-") || target.contains("manifest") {
        "rust-export"
    } else if target.contains("residue-mask") || target.contains("theorem-queue") {
        "proof-artifact"
    } else {
        "script-step"
    }
}

#[derive(Clone, Copy)]
enum RankingKind {
    SustainedCost,
    Volatility,
    Combined,
}

fn assign_rank(rows: &mut [ProofBuildObservatoryTargetRow], kind: RankingKind) {
    let mut indices: Vec<usize> = (0..rows.len()).collect();
    indices.sort_by(|&left_idx, &right_idx| {
        let left = &rows[left_idx];
        let right = &rows[right_idx];
        rank_bucket(left)
            .cmp(&rank_bucket(right))
            .then_with(|| match kind {
                RankingKind::SustainedCost => {
                    desc_f64(left.median_duration_ms, right.median_duration_ms)
                        .then_with(|| right.absolute_spread_ms.cmp(&left.absolute_spread_ms))
                }
                RankingKind::Volatility => right
                    .absolute_spread_ms
                    .cmp(&left.absolute_spread_ms)
                    .then_with(|| desc_f64(left.relative_spread, right.relative_spread)),
                RankingKind::Combined => desc_f64(left.combined_score, right.combined_score),
            })
            .then_with(|| left.suite.cmp(&right.suite))
            .then_with(|| left.target.cmp(&right.target))
    });

    for (rank, index) in indices.into_iter().enumerate() {
        match kind {
            RankingKind::SustainedCost => rows[index].sustained_cost_rank = rank + 1,
            RankingKind::Volatility => rows[index].volatility_rank = rank + 1,
            RankingKind::Combined => rows[index].combined_rank = rank + 1,
        }
    }
}

fn rank_bucket(row: &ProofBuildObservatoryTargetRow) -> usize {
    usize::from(row.target_kind == "suite-total")
}

fn desc_f64(left: f64, right: f64) -> Ordering {
    right.partial_cmp(&left).unwrap_or(Ordering::Equal)
}

fn action_hint(row: &ProofBuildObservatoryTargetRow) -> &'static str {
    if row.status != 0 || row.count < 2 || row.relative_spread >= 0.50 {
        "rerun-before-concluding"
    } else if row.target_kind == "suite-total" {
        "observe"
    } else if row.suite == "lean-umbrella" && row.sustained_cost_rank <= 3 {
        "umbrella-partition-candidate"
    } else if row.target_kind == "lake-target" && row.median_duration_ms >= 30_000.0 {
        "compression-candidate"
    } else if row.target_kind == "lean-check" && row.median_duration_ms >= 10_000.0 {
        "split-candidate"
    } else {
        "observe"
    }
}

fn build_summary(rows: &[ProofBuildObservatoryTargetRow]) -> ProofBuildObservatorySummary {
    let suite_count = rows
        .iter()
        .map(|row| row.suite.clone())
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let failed_target_count = rows.iter().filter(|row| row.status != 0).count();
    let max_median_duration_ms = rows
        .iter()
        .map(|row| row.median_duration_ms)
        .fold(0.0, f64::max);
    let max_relative_spread = rows
        .iter()
        .map(|row| row.relative_spread)
        .fold(0.0, f64::max);

    let best_by_rank = |ranker: fn(&ProofBuildObservatoryTargetRow) -> usize| {
        rows.iter()
            .min_by_key(|row| ranker(row))
            .map(|row| format!("{}/{}", row.suite, row.target))
    };

    ProofBuildObservatorySummary {
        suite_count,
        target_count: rows.len(),
        failed_target_count,
        max_median_duration_ms,
        max_relative_spread,
        highest_sustained_cost_target: best_by_rank(|row| row.sustained_cost_rank),
        highest_volatility_target: best_by_rank(|row| row.volatility_rank),
        highest_combined_score_target: best_by_rank(|row| row.combined_rank),
    }
}

fn fmt_f64(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        format!("{value:.3}")
    }
}

pub fn raw_report_path_map(report: &ProofBuildObservatoryReport) -> HashMap<String, PathBuf> {
    report
        .raw_reports
        .iter()
        .map(|raw| (raw.suite.clone(), PathBuf::from(&raw.path)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source(suite: &str, json: &str) -> ProofBuildTimingSource {
        ProofBuildTimingSource {
            suite: suite.to_string(),
            path: PathBuf::from(format!("/tmp/{suite}.json")),
            report: serde_json::from_str(json).unwrap(),
        }
    }

    const WITNESS_JSON: &str = r#"{
      "schema_version": "proof-carrying-witness-lean-timing-v2",
      "command": "scripts/lean_proof_carrying_witness_certificate.sh timing",
      "repeat_count": 3,
      "rows": [],
      "run_totals": [],
      "summary_rows": [
        {
          "step": "lake-build:PrimeArithmetic.Generated.Witness.MatrixCatalogChecks",
          "count": 3,
          "min_duration_ms": 1000,
          "median_duration_ms": 1500,
          "max_duration_ms": 2100,
          "status": 0
        },
        {
          "step": "total",
          "count": 3,
          "min_duration_ms": 2000,
          "median_duration_ms": 3000,
          "max_duration_ms": 4200,
          "status": 0
        }
      ],
      "total_duration_ms": 9000
    }"#;

    const MATCHED_JSON: &str = r#"{
      "schema_version": "proof-build-target-timing-v1",
      "command": "scripts/matched_control_atlas_bridge.sh timing",
      "repeat_count": 3,
      "rows": [],
      "run_totals": [],
      "summary_rows": [
        {
          "step": "residue-mask-pair-proof-coverage",
          "count": 3,
          "min_duration_ms": 500,
          "median_duration_ms": 650.5,
          "max_duration_ms": 800,
          "status": 0
        }
      ],
      "total_duration_ms": 1950
    }"#;

    const LEAN_JSON: &str = r#"{
      "schema_version": "proof-build-target-timing-v1",
      "command": "scripts/lean_umbrella_build_timing.sh timing",
      "repeat_count": 3,
      "rows": [],
      "run_totals": [],
      "summary_rows": [
        {
          "step": "lake-build:PrimeArithmetic",
          "count": 3,
          "min_duration_ms": 8000,
          "median_duration_ms": 10000,
          "max_duration_ms": 16000,
          "status": 1
        }
      ],
      "total_duration_ms": 30000
    }"#;

    #[test]
    fn parses_existing_witness_timing_and_ranks_rows() {
        let report = build_proof_build_observatory_report(&[
            source("witness-lean", WITNESS_JSON),
            source("matched-control-atlas", MATCHED_JSON),
            source("lean-umbrella", LEAN_JSON),
        ])
        .unwrap();

        assert_eq!(
            report.schema_version,
            PROOF_BUILD_OBSERVATORY_SCHEMA_VERSION
        );
        assert_eq!(report.repeat_count, 3);
        assert_eq!(report.summary.suite_count, 3);
        assert_eq!(report.summary.target_count, 4);
        assert_eq!(report.summary.failed_target_count, 1);
        assert_eq!(
            report.summary.highest_sustained_cost_target.as_deref(),
            Some("lean-umbrella/lake-build:PrimeArithmetic")
        );

        let matched = report
            .target_rows
            .iter()
            .find(|row| row.target == "residue-mask-pair-proof-coverage")
            .unwrap();
        assert_eq!(matched.median_duration_ms, 650.5);
        assert_eq!(matched.absolute_spread_ms, 300);
        assert_eq!(matched.action_hint, "observe");

        let failed = report
            .target_rows
            .iter()
            .find(|row| row.suite == "lean-umbrella")
            .unwrap();
        assert_eq!(failed.status, 1);
        assert_eq!(failed.action_hint, "rerun-before-concluding");
    }

    #[test]
    fn rejects_mixed_repeat_counts() {
        let mut mismatched = source("lean-umbrella", LEAN_JSON);
        mismatched.report.repeat_count = 2;
        let err = build_proof_build_observatory_report(&[
            source("witness-lean", WITNESS_JSON),
            mismatched,
        ])
        .unwrap_err();
        assert!(err.contains("mixed repeat counts"));
    }

    #[test]
    fn serializes_round_trip_stably() {
        let report =
            build_proof_build_observatory_report(&[source("witness-lean", WITNESS_JSON)]).unwrap();
        let json = serde_json::to_string_pretty(&report).unwrap();
        let reparsed: ProofBuildObservatoryReport = serde_json::from_str(&json).unwrap();
        assert_eq!(report, reparsed);

        let markdown = render_proof_build_observatory_markdown(&report);
        assert!(markdown.contains("# Proof-Build Observatory"));
        assert!(markdown.contains("Sustained Cost"));
    }
}
