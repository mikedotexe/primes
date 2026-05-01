//! Direct bounded-`k` transfer-criterion audit.
//!
//! This report exists for theorem work, not hinge interpretation.
//! It compares `k=(0,0)` against each maintained noncompact lane directly,
//! rather than collapsing through `best_k`.
//!
//! The governing ladder is:
//! 1. `profile_agreement`
//! 2. `admissible_equality_only`
//! 3. `no_positive_admissible_delta_only`

use primes::validation::{
    bounded_k::{
        coprime_prefilter_moduli, format_k, ordered_unit_pairs,
        scan_k_config_lane_profile_comparison, BoundedKConfig, KConfigLaneProfileComparison,
        DEFAULT_BOUNDED_K_GRID,
    },
    reporting::{
        ensure_dir, export_timestamp_utc, write_artifact_manifest, write_csv_rows,
        write_json_pretty, write_text_file, ArtifactManifest,
    },
};
use rayon::prelude::*;
use serde::Serialize;
use std::{collections::BTreeMap, env, path::PathBuf};

const BASES: &[u32] = &[6, 10, 12, 14, 22, 26, 30, 34];
const TWO_PRIME_BASES: &[u32] = &[6, 10, 14, 22, 26, 34];
const WHEEL_BASES: &[u32] = &[30];
const DEFAULT_OUT_DIR: &str = "/tmp/primes_bounded_k_transfer_criterion";
const ARTIFACT_ID: &str = "bounded_k_transfer_criterion_report";
const REPORT_EXPORT_VERSION: u32 = 1;
const MIDDLE_LENGTHS: &[usize] = &[2, 3];

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    include_base_210: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    bases: Vec<u32>,
    middle_lengths: Vec<usize>,
    lane_grid: Vec<String>,
    include_base_210: bool,
}

#[derive(Debug, Clone, Serialize)]
struct LaneProfileAgreementRow {
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    pair_label: String,
    from_k: String,
    to_k: String,
    compared_moduli_count: usize,
    coprime_prefilter_moduli: String,
    agreeing_modulus_count: usize,
    disagreeing_modulus_count: usize,
    disagreeing_moduli: String,
    all_singleton_profiles: bool,
    profile_agreement: bool,
}

#[derive(Debug, Clone, Serialize)]
struct LaneAdmissibleEqualityRow {
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    pair_label: String,
    from_k: String,
    to_k: String,
    admissible_set_equal: bool,
    stable_zero_count: usize,
    gain_zero_count: usize,
    loss_zero_count: usize,
    stable_nonzero_count: usize,
    nonzero_churn_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct LaneAdmissibleDeltaRow {
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    pair_label: String,
    from_k: String,
    to_k: String,
    admissible_delta_count: isize,
    no_positive_admissible_delta: bool,
    theorem_rung_label: String,
}

#[derive(Debug, Clone, Serialize)]
struct LaneRungSummaryRow {
    group_kind: String,
    group_label: String,
    middle_length: usize,
    comparison_count: usize,
    profile_agreement_count: usize,
    admissible_equality_only_count: usize,
    no_positive_admissible_delta_only_count: usize,
    fails_all_three_count: usize,
    strongest_surviving_rung_label: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    total_comparisons: usize,
    m2_strongest_surviving_rung: String,
    m3_strongest_surviving_rung: String,
    two_prime_m3_strongest_surviving_rung: String,
    wheel_m3_strongest_surviving_rung: String,
    main_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    lane_profile_agreement_rows: Vec<LaneProfileAgreementRow>,
    lane_admissible_equality_rows: Vec<LaneAdmissibleEqualityRow>,
    lane_admissible_delta_rows: Vec<LaneAdmissibleDeltaRow>,
    lane_rung_summary_rows: Vec<LaneRungSummaryRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let bases = report_bases(options.include_base_210);
    let comparisons = build_lane_comparisons(&bases);
    let lane_profile_agreement_rows = build_profile_rows(&comparisons);
    let lane_admissible_equality_rows = build_admissible_equality_rows(&comparisons);
    let lane_admissible_delta_rows = build_admissible_delta_rows(&comparisons);
    let lane_rung_summary_rows = build_summary_rows(&comparisons, options.include_base_210);
    let report_summary = build_report_summary(&comparisons, options.include_base_210);
    let observations = build_observations(&comparisons, &lane_rung_summary_rows, &report_summary);

    write_csv_rows(
        options.out_dir.join("lane_profile_agreement_rows.csv"),
        &lane_profile_agreement_rows,
    )
    .expect("failed to write lane_profile_agreement_rows.csv");
    write_csv_rows(
        options.out_dir.join("lane_admissible_equality_rows.csv"),
        &lane_admissible_equality_rows,
    )
    .expect("failed to write lane_admissible_equality_rows.csv");
    write_csv_rows(
        options.out_dir.join("lane_admissible_delta_rows.csv"),
        &lane_admissible_delta_rows,
    )
    .expect("failed to write lane_admissible_delta_rows.csv");
    write_csv_rows(
        options.out_dir.join("lane_rung_summary_rows.csv"),
        &lane_rung_summary_rows,
    )
    .expect("failed to write lane_rung_summary_rows.csv");

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        bases: bases.clone(),
        middle_lengths: MIDDLE_LENGTHS.to_vec(),
        lane_grid: DEFAULT_BOUNDED_K_GRID
            .iter()
            .map(|&k| format_k(k))
            .collect(),
        include_base_210: options.include_base_210,
    };
    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        lane_profile_agreement_rows,
        lane_admissible_equality_rows,
        lane_admissible_delta_rows,
        lane_rung_summary_rows,
        report_summary,
        observations,
    };
    write_json_pretty(options.out_dir.join("summary.json"), &bundle)
        .expect("failed to write summary.json");
    write_text_file(options.out_dir.join("report.md"), &render_markdown(&bundle))
        .expect("failed to write report.md");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo".to_string(),
            args: artifact_args(&options),
            upstream_inputs: vec![
                "src/validation/bounded_k.rs".to_string(),
                "lean-proofs/PrimeArithmetic/Structure/BoundedKTemplate.lean".to_string(),
                "lean-proofs/PrimeArithmetic/Structure/FiniteMaskTransfer.lean".to_string(),
            ],
            expected_outputs: vec![
                "lane_profile_agreement_rows.csv".to_string(),
                "lane_admissible_equality_rows.csv".to_string(),
                "lane_admissible_delta_rows.csv".to_string(),
                "lane_rung_summary_rows.csv".to_string(),
                "summary.json".to_string(),
                "report.md".to_string(),
            ],
        },
    )
    .expect("failed to write artifact_manifest.json");

    let m2_summary = find_summary_row(&bundle.lane_rung_summary_rows, "all", MIDDLE_LENGTHS[0]);
    let m3_summary = find_summary_row(&bundle.lane_rung_summary_rows, "all", MIDDLE_LENGTHS[1]);
    println!("bounded-k transfer criterion report");
    println!("  output dir: {}", options.out_dir.display());
    println!(
        "  M2 strongest rung | {} | comparisons {}",
        m2_summary.strongest_surviving_rung_label, m2_summary.comparison_count
    );
    println!(
        "  M3 strongest rung | {} | comparisons {}",
        m3_summary.strongest_surviving_rung_label, m3_summary.comparison_count
    );
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut include_base_210 = false;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => {
                let value = args.next().expect("--out-dir requires a directory path");
                out_dir = PathBuf::from(value);
            }
            "--include-base-210" => include_base_210 = true,
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            other => panic!("unrecognized argument: {other}"),
        }
    }
    Options {
        out_dir,
        include_base_210,
    }
}

fn print_help() {
    println!("bounded_k_transfer_criterion_report");
    println!();
    println!("Usage:");
    println!("  cargo run --release --example bounded_k_transfer_criterion_report");
    println!(
        "  cargo run --release --example bounded_k_transfer_criterion_report -- --out-dir /tmp/primes_bounded_k_transfer_criterion_alt"
    );
    println!(
        "  cargo run --release --example bounded_k_transfer_criterion_report -- --include-base-210"
    );
}

fn report_bases(include_base_210: bool) -> Vec<u32> {
    let mut bases = BASES.to_vec();
    if include_base_210 {
        bases.push(210);
    }
    bases
}

fn noncompact_lanes() -> Vec<BoundedKConfig> {
    DEFAULT_BOUNDED_K_GRID
        .iter()
        .copied()
        .filter(|&k| k != (0, 0))
        .collect()
}

fn build_lane_comparisons(bases: &[u32]) -> Vec<KConfigLaneProfileComparison> {
    let tasks = bases
        .iter()
        .copied()
        .flat_map(|base| {
            MIDDLE_LENGTHS
                .iter()
                .copied()
                .flat_map(move |middle_length| {
                    ordered_unit_pairs(base)
                        .into_iter()
                        .flat_map(move |(outer, inner)| {
                            noncompact_lanes()
                                .into_iter()
                                .map(move |to_k| (base, middle_length, outer, inner, to_k))
                        })
                })
        })
        .collect::<Vec<_>>();

    let mut rows = tasks
        .par_iter()
        .map(|&(base, middle_length, outer, inner, to_k)| {
            scan_k_config_lane_profile_comparison(base, middle_length, outer, inner, (0, 0), to_k)
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.base
            .cmp(&right.base)
            .then_with(|| left.middle_length.cmp(&right.middle_length))
            .then_with(|| left.outer.cmp(&right.outer))
            .then_with(|| left.inner.cmp(&right.inner))
            .then_with(|| left.to_k.cmp(&right.to_k))
    });
    rows
}

fn build_profile_rows(
    comparisons: &[KConfigLaneProfileComparison],
) -> Vec<LaneProfileAgreementRow> {
    comparisons
        .iter()
        .map(|row| {
            let disagreeing_moduli = row
                .modulus_rows
                .iter()
                .filter(|entry| !entry.profile_agreement)
                .map(|entry| entry.modulus.to_string())
                .collect::<Vec<_>>();
            LaneProfileAgreementRow {
                base: row.base,
                middle_length: row.middle_length,
                outer: row.outer,
                inner: row.inner,
                pair_label: row.pair_label.clone(),
                from_k: row.from_k.clone(),
                to_k: row.to_k.clone(),
                compared_moduli_count: row.compared_moduli_count,
                coprime_prefilter_moduli: coprime_prefilter_moduli(row.base)
                    .into_iter()
                    .map(|modulus| modulus.to_string())
                    .collect::<Vec<_>>()
                    .join("|"),
                agreeing_modulus_count: row
                    .modulus_rows
                    .iter()
                    .filter(|entry| entry.profile_agreement)
                    .count(),
                disagreeing_modulus_count: disagreeing_moduli.len(),
                disagreeing_moduli: if disagreeing_moduli.is_empty() {
                    "none".to_string()
                } else {
                    disagreeing_moduli.join("|")
                },
                all_singleton_profiles: row.all_singleton_profiles,
                profile_agreement: row.profile_agreement,
            }
        })
        .collect()
}

fn build_admissible_equality_rows(
    comparisons: &[KConfigLaneProfileComparison],
) -> Vec<LaneAdmissibleEqualityRow> {
    comparisons
        .iter()
        .map(|row| LaneAdmissibleEqualityRow {
            base: row.base,
            middle_length: row.middle_length,
            outer: row.outer,
            inner: row.inner,
            pair_label: row.pair_label.clone(),
            from_k: row.from_k.clone(),
            to_k: row.to_k.clone(),
            admissible_set_equal: row.admissible_set_equal,
            stable_zero_count: row.stable_zero_count,
            gain_zero_count: row.gain_zero_count,
            loss_zero_count: row.loss_zero_count,
            stable_nonzero_count: row.stable_nonzero_count,
            nonzero_churn_count: row.nonzero_churn_count,
        })
        .collect()
}

fn build_admissible_delta_rows(
    comparisons: &[KConfigLaneProfileComparison],
) -> Vec<LaneAdmissibleDeltaRow> {
    comparisons
        .iter()
        .map(|row| LaneAdmissibleDeltaRow {
            base: row.base,
            middle_length: row.middle_length,
            outer: row.outer,
            inner: row.inner,
            pair_label: row.pair_label.clone(),
            from_k: row.from_k.clone(),
            to_k: row.to_k.clone(),
            admissible_delta_count: row.admissible_delta_count,
            no_positive_admissible_delta: row.no_positive_admissible_delta,
            theorem_rung_label: row.theorem_rung_label.clone(),
        })
        .collect()
}

fn build_summary_rows(
    comparisons: &[KConfigLaneProfileComparison],
    include_base_210: bool,
) -> Vec<LaneRungSummaryRow> {
    let mut rows = Vec::new();
    for &middle_length in MIDDLE_LENGTHS {
        rows.push(summary_row(
            "all",
            "all",
            middle_length,
            comparisons
                .iter()
                .filter(|row| row.middle_length == middle_length),
        ));
        rows.push(summary_row(
            "class",
            "two_prime_like",
            middle_length,
            comparisons.iter().filter(|row| {
                row.middle_length == middle_length && TWO_PRIME_BASES.contains(&row.base)
            }),
        ));
        rows.push(summary_row(
            "class",
            "wheel_like",
            middle_length,
            comparisons.iter().filter(|row| {
                row.middle_length == middle_length
                    && (WHEEL_BASES.contains(&row.base) || (include_base_210 && row.base == 210))
            }),
        ));
    }

    let mut by_base: BTreeMap<(u32, usize), Vec<&KConfigLaneProfileComparison>> = BTreeMap::new();
    for row in comparisons {
        by_base
            .entry((row.base, row.middle_length))
            .or_default()
            .push(row);
    }
    for ((base, middle_length), group_rows) in by_base {
        rows.push(summary_row(
            "base",
            &base.to_string(),
            middle_length,
            group_rows.into_iter(),
        ));
    }
    rows
}

fn summary_row<'a>(
    group_kind: &str,
    group_label: &str,
    middle_length: usize,
    rows: impl Iterator<Item = &'a KConfigLaneProfileComparison>,
) -> LaneRungSummaryRow {
    let rows = rows.collect::<Vec<_>>();
    let strongest =
        strongest_surviving_rung(rows.iter().map(|row| row.theorem_rung_label.as_str()));
    LaneRungSummaryRow {
        group_kind: group_kind.to_string(),
        group_label: group_label.to_string(),
        middle_length,
        comparison_count: rows.len(),
        profile_agreement_count: rows
            .iter()
            .filter(|row| row.theorem_rung_label == "profile_agreement")
            .count(),
        admissible_equality_only_count: rows
            .iter()
            .filter(|row| row.theorem_rung_label == "admissible_equality_only")
            .count(),
        no_positive_admissible_delta_only_count: rows
            .iter()
            .filter(|row| row.theorem_rung_label == "no_positive_admissible_delta_only")
            .count(),
        fails_all_three_count: rows
            .iter()
            .filter(|row| row.theorem_rung_label == "fails_all_three")
            .count(),
        strongest_surviving_rung_label: strongest.to_string(),
    }
}

fn strongest_surviving_rung<'a>(labels: impl Iterator<Item = &'a str>) -> &'static str {
    let labels = labels.collect::<Vec<_>>();
    if labels.iter().all(|&label| label == "profile_agreement") {
        "profile_agreement"
    } else if labels
        .iter()
        .all(|&label| matches!(label, "profile_agreement" | "admissible_equality_only"))
    {
        "admissible_equality_only"
    } else if labels.iter().all(|&label| label != "fails_all_three") {
        "no_positive_admissible_delta_only"
    } else {
        "fails_all_three"
    }
}

fn build_report_summary(
    comparisons: &[KConfigLaneProfileComparison],
    include_base_210: bool,
) -> ReportSummary {
    let m2_all = summary_row(
        "all",
        "all",
        2,
        comparisons.iter().filter(|row| row.middle_length == 2),
    );
    let m3_all = summary_row(
        "all",
        "all",
        3,
        comparisons.iter().filter(|row| row.middle_length == 3),
    );
    let two_prime_m3 = summary_row(
        "class",
        "two_prime_like",
        3,
        comparisons
            .iter()
            .filter(|row| row.middle_length == 3 && TWO_PRIME_BASES.contains(&row.base)),
    );
    let wheel_m3 = summary_row(
        "class",
        "wheel_like",
        3,
        comparisons.iter().filter(|row| {
            row.middle_length == 3
                && (WHEEL_BASES.contains(&row.base) || (include_base_210 && row.base == 210))
        }),
    );

    ReportSummary {
        total_comparisons: comparisons.len(),
        m2_strongest_surviving_rung: m2_all.strongest_surviving_rung_label.clone(),
        m3_strongest_surviving_rung: m3_all.strongest_surviving_rung_label.clone(),
        two_prime_m3_strongest_surviving_rung: two_prime_m3
            .strongest_surviving_rung_label
            .clone(),
        wheel_m3_strongest_surviving_rung: wheel_m3.strongest_surviving_rung_label.clone(),
        main_takeaway: format!(
            "Direct lane comparisons show `{}` as the strongest surviving exact rung at M=3 on the maintained audit surface; theorem work should target that rung, not a stronger restatement.",
            m3_all.strongest_surviving_rung_label
        ),
    }
}

fn build_observations(
    comparisons: &[KConfigLaneProfileComparison],
    summary_rows: &[LaneRungSummaryRow],
    report_summary: &ReportSummary,
) -> Vec<String> {
    let m2_all = find_summary_row(summary_rows, "all", 2);
    let m3_all = find_summary_row(summary_rows, "all", 3);
    let m3_profile_share = ratio(
        comparisons
            .iter()
            .filter(|row| row.middle_length == 3 && row.profile_agreement)
            .count(),
        comparisons
            .iter()
            .filter(|row| row.middle_length == 3)
            .count(),
    ) * 100.0;
    let m3_fail_count = comparisons
        .iter()
        .filter(|row| row.middle_length == 3 && row.theorem_rung_label == "fails_all_three")
        .count();
    vec![
        format!(
            "The direct-lane audit keeps the theorem surface honest: `M=2` comparisons survive only to rung `{}`, while `M=3` reaches rung `{}` on the full maintained surface.",
            m2_all.strongest_surviving_rung_label, m3_all.strongest_surviving_rung_label
        ),
        format!(
            "At `M=3`, `{m3_profile_share:.2}%` of direct `k=(0,0) -> lane` comparisons satisfy full profile agreement, and `{}`
 comparisons fall all the way through the ladder.",
            m3_fail_count
        ),
        format!(
            "The parallel class read should be based on the strongest surviving rung, not a stronger wish: `two_prime_like` lands at `{}`, while `wheel_like` lands at `{}`.",
            report_summary.two_prime_m3_strongest_surviving_rung,
            report_summary.wheel_m3_strongest_surviving_rung
        ),
    ]
}

fn find_summary_row<'a>(
    rows: &'a [LaneRungSummaryRow],
    group_label: &str,
    middle_length: usize,
) -> &'a LaneRungSummaryRow {
    rows.iter()
        .find(|row| {
            row.group_label == group_label
                && row.middle_length == middle_length
                && row.group_kind == "all"
        })
        .unwrap_or_else(|| panic!("missing summary row for {group_label} M={middle_length}"))
}

fn render_markdown(bundle: &ReportBundle) -> String {
    let mut out = String::new();
    out.push_str("# Bounded-k Transfer Criterion Report\n\n");
    out.push_str("_Generated from `examples/bounded_k_transfer_criterion_report.rs`._\n\n");
    out.push_str(&format!(
        "- Output directory: `{}`\n- Bases: `{}`\n- Middle lengths: `2, 3`\n- Direct lane comparisons: `k=(0,0)` against `{}`\n\n",
        bundle.settings.out_dir,
        bundle
            .settings
            .bases
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        bundle
            .settings
            .lane_grid
            .iter()
            .filter(|label| label.as_str() != "k=(0,0)")
            .cloned()
            .collect::<Vec<_>>()
            .join(", ")
    ));

    out.push_str("## Strongest Surviving Rungs\n\n");
    out.push_str("| Group | M | Comparisons | Strongest surviving rung | Profile | Equality | No-gain only | Fail |\n");
    out.push_str("|---|---:|---:|---|---:|---:|---:|---:|\n");
    for row in &bundle.lane_rung_summary_rows {
        if row.group_kind == "all" || row.group_kind == "class" {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
                row.group_label,
                row.middle_length,
                row.comparison_count,
                row.strongest_surviving_rung_label,
                row.profile_agreement_count,
                row.admissible_equality_only_count,
                row.no_positive_admissible_delta_only_count,
                row.fails_all_three_count
            ));
        }
    }

    out.push_str("\n## Base Summary\n\n");
    out.push_str("| Base | M | Comparisons | Strongest surviving rung | Profile | Equality | No-gain only | Fail |\n");
    out.push_str("|---|---:|---:|---|---:|---:|---:|---:|\n");
    for row in &bundle.lane_rung_summary_rows {
        if row.group_kind == "base" {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
                row.group_label,
                row.middle_length,
                row.comparison_count,
                row.strongest_surviving_rung_label,
                row.profile_agreement_count,
                row.admissible_equality_only_count,
                row.no_positive_admissible_delta_only_count,
                row.fails_all_three_count
            ));
        }
    }

    out.push_str("\n## Observations\n\n");
    for observation in &bundle.observations {
        out.push_str(&format!("- {observation}\n"));
    }

    out
}

fn artifact_args(options: &Options) -> Vec<String> {
    let mut args = vec![
        "run".to_string(),
        "--release".to_string(),
        "--example".to_string(),
        "bounded_k_transfer_criterion_report".to_string(),
        "--".to_string(),
        "--out-dir".to_string(),
        options.out_dir.display().to_string(),
    ];
    if options.include_base_210 {
        args.push("--include-base-210".to_string());
    }
    args
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strongest_rung_ladder_is_monotone() {
        assert_eq!(
            strongest_surviving_rung(["profile_agreement", "profile_agreement"].into_iter()),
            "profile_agreement"
        );
        assert_eq!(
            strongest_surviving_rung(["profile_agreement", "admissible_equality_only"].into_iter()),
            "admissible_equality_only"
        );
        assert_eq!(
            strongest_surviving_rung(
                [
                    "profile_agreement",
                    "admissible_equality_only",
                    "no_positive_admissible_delta_only"
                ]
                .into_iter()
            ),
            "no_positive_admissible_delta_only"
        );
        assert_eq!(
            strongest_surviving_rung(["profile_agreement", "fails_all_three"].into_iter()),
            "fails_all_three"
        );
    }

    #[test]
    fn rung_classifier_matches_summary_labels() {
        assert_eq!(
            primes::validation::bounded_k::classify_theorem_rung(true, true, true),
            "profile_agreement"
        );
        assert_eq!(
            primes::validation::bounded_k::classify_theorem_rung(false, true, true),
            "admissible_equality_only"
        );
        assert_eq!(
            primes::validation::bounded_k::classify_theorem_rung(false, false, true),
            "no_positive_admissible_delta_only"
        );
        assert_eq!(
            primes::validation::bounded_k::classify_theorem_rung(false, false, false),
            "fails_all_three"
        );
    }
}
