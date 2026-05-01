//! Exploratory report for period-locked affine gradient agreement.
//!
//! This pass chases the hypothesis suggested by the base-22 / mod-5 residual
//! pocket:
//!
//! - local affine gradient equality is governed by a period lock
//! - the lock is `Δposition ≡ 0 mod ord_p(base)` on coprime moduli
//! - the remaining identity vs `gradient_only` split is then a shift question
//!
//! The report treats this as a reusable local classifier surface rather than a
//! new public claim.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example affine_period_lock_report
//! cargo run --release --example affine_period_lock_report -- --out-dir /tmp/primes_affine_period_lock_alt
//! ```

use plotters::prelude::*;
use primes::validation::{
    affine_period_lock::{scan_k_config_affine_period_lock_comparison, AffinePeriodLockComparison},
    bounded_k::{digit_symbol, format_k, ordered_unit_pairs, BoundedKConfig},
    reporting::{
        ensure_dir, export_timestamp_utc, write_artifact_manifest, write_csv_rows,
        write_json_pretty, write_text_file, ArtifactManifest,
    },
};
use rayon::prelude::*;
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    path::{Path, PathBuf},
};

const MAIN_BASES: &[u32] = &[10, 14, 22, 26];
const APPENDIX_BASES: &[u32] = &[34, 6];
const MIDDLE_LENGTHS: &[usize] = &[1, 2, 3];
const FROM_K: BoundedKConfig = (0, 0);
const NONCOMPACT_LANES: &[BoundedKConfig] = &[(0, 1), (1, 0), (1, 1), (2, 2)];
const DEFAULT_OUT_DIR: &str = "/tmp/primes_affine_period_lock";
const REPORT_EXPORT_VERSION: u32 = 1;
const ARTIFACT_ID: &str = "affine_period_lock_report";

const REPRESENTATIVES: &[RepresentativeSpec] = &[
    RepresentativeSpec {
        role: "persistent_core",
        base: 14,
        outer: 13,
        inner: 11,
    },
    RepresentativeSpec {
        role: "persistence_only",
        base: 10,
        outer: 3,
        inner: 3,
    },
    RepresentativeSpec {
        role: "active_neither_pocket",
        base: 22,
        outer: 17,
        inner: 19,
    },
    RepresentativeSpec {
        role: "collapsed_column",
        base: 22,
        outer: 17,
        inner: 15,
    },
    RepresentativeSpec {
        role: "appendix_outgroup",
        base: 34,
        outer: 25,
        inner: 9,
    },
    RepresentativeSpec {
        role: "same_base_dead_control",
        base: 10,
        outer: 9,
        inner: 9,
    },
];

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Copy)]
struct RepresentativeSpec {
    role: &'static str,
    base: u32,
    outer: u32,
    inner: u32,
}

type ModulusSummaryKey = (String, u32, usize, String, u32, i32, u32);

#[derive(Debug, Clone)]
struct ComparisonBundle {
    scope: String,
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    pair_label: String,
    same_digit: bool,
    comparison: AffinePeriodLockComparison,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    main_bases: Vec<u32>,
    appendix_bases: Vec<u32>,
    middle_lengths: Vec<usize>,
    from_k: String,
    noncompact_lanes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct PeriodLockComparisonCsvRow {
    scope: String,
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    pair_label: String,
    same_digit: bool,
    to_k: String,
    gradient_position_from: u32,
    gradient_position_to: u32,
    gradient_position_delta: i32,
    compared_moduli_count: usize,
    period_lock_expected_count: usize,
    observed_gradient_equal_count: usize,
    period_lock_match_count: usize,
    period_lock_mismatch_count: usize,
    period_lock_expected_share: f64,
    observed_gradient_equal_share: f64,
    period_lock_match_share: f64,
    period_lock_perfect: bool,
    same_shift_count: usize,
    same_zero_seed_count: usize,
    identity_count: usize,
    shift_only_count: usize,
    gradient_only_count: usize,
    shift_and_gradient_count: usize,
    identity_share: f64,
    shift_only_share: f64,
    gradient_only_share: f64,
    shift_and_gradient_share: f64,
    period_locked_identity_count: usize,
    period_locked_gradient_only_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct PeriodLockModulusCsvRow {
    scope: String,
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    pair_label: String,
    to_k: String,
    modulus: u32,
    base_modulus: u32,
    gradient_position_from: u32,
    gradient_position_to: u32,
    gradient_position_delta: i32,
    multiplicative_order: u32,
    delta_mod_order: u32,
    period_lock_expected: bool,
    observed_gradient_equal: bool,
    expected_matches_observation: bool,
    shift_equal: bool,
    zero_seed_equal: bool,
    local_relation_label: String,
    shift_modulus_from: u32,
    shift_modulus_to: u32,
    gradient_modulus_from: u32,
    gradient_modulus_to: u32,
    zero_seed_class_from: u32,
    zero_seed_class_to: u32,
}

#[derive(Debug, Clone, Serialize)]
struct PeriodLockModulusSummaryRow {
    scope: String,
    base: u32,
    middle_length: usize,
    to_k: String,
    modulus: u32,
    gradient_position_delta: i32,
    multiplicative_order: u32,
    pair_rows: usize,
    period_lock_expected_pair_share: f64,
    observed_gradient_equal_pair_share: f64,
    perfect_match_pair_share: f64,
    identity_pair_share: f64,
    gradient_only_pair_share: f64,
    shift_only_pair_share: f64,
    shift_and_gradient_pair_share: f64,
}

#[derive(Debug, Clone, Serialize)]
struct RepresentativePeriodLockRow {
    role: String,
    base: u32,
    pair_label: String,
    to_k: String,
    gradient_position_delta: i32,
    period_lock_expected_count: usize,
    observed_gradient_equal_count: usize,
    period_lock_mismatch_count: usize,
    locked_moduli_label: String,
    gradient_equal_moduli_label: String,
    identity_moduli_label: String,
    gradient_only_moduli_label: String,
    note: String,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    comparison_rows: usize,
    modulus_rows: usize,
    mismatch_rows: usize,
    mismatch_share: f64,
    perfect_comparison_share: f64,
    main_m2_positive_hotspot_count: usize,
    exact_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    comparison_rows: Vec<PeriodLockComparisonCsvRow>,
    modulus_rows: Vec<PeriodLockModulusCsvRow>,
    modulus_summary_rows: Vec<PeriodLockModulusSummaryRow>,
    representative_rows: Vec<RepresentativePeriodLockRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let bundles = build_comparison_bundles();
    let comparison_rows = build_comparison_rows(&bundles);
    let modulus_rows = build_modulus_rows(&bundles);
    let modulus_summary_rows = build_modulus_summary_rows(&modulus_rows);
    let representative_rows = build_representative_rows(&bundles);

    let heatmap_path = options.out_dir.join("period_lock_heatmap.png");
    render_period_lock_heatmap(&modulus_summary_rows, &heatmap_path);
    let plane_path = options.out_dir.join("period_lock_residual_plane.png");
    render_period_lock_residual_plane(&comparison_rows, &plane_path);
    let strip_path = options.out_dir.join("period_lock_representative_strip.png");
    render_period_lock_representative_strip(&modulus_rows, &strip_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "period_lock_heatmap".to_string(),
            label: "Period-lock modulus heatmap".to_string(),
            path: heatmap_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "residual_plane".to_string(),
            label: "Period-lock residual plane".to_string(),
            path: plane_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "representative_strip".to_string(),
            label: "Representative period-lock strip".to_string(),
            path: strip_path.display().to_string(),
        },
    ];

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        main_bases: MAIN_BASES.to_vec(),
        appendix_bases: APPENDIX_BASES.to_vec(),
        middle_lengths: MIDDLE_LENGTHS.to_vec(),
        from_k: format_k(FROM_K),
        noncompact_lanes: NONCOMPACT_LANES.iter().map(|&k| format_k(k)).collect(),
    };
    let report_summary =
        build_report_summary(&comparison_rows, &modulus_rows, &modulus_summary_rows);
    let observations = derive_observations(&modulus_summary_rows, &representative_rows);
    let report_text = render_report(&settings, &report_summary, &observations);

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        comparison_rows: comparison_rows.clone(),
        modulus_rows: modulus_rows.clone(),
        modulus_summary_rows: modulus_summary_rows.clone(),
        representative_rows: representative_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary: report_summary.clone(),
        observations: observations.clone(),
    };

    write_csv_rows(
        options.out_dir.join("period_lock_comparison_rows.csv"),
        &comparison_rows,
    )
    .expect("write comparison rows");
    write_csv_rows(
        options.out_dir.join("period_lock_modulus_rows.csv"),
        &modulus_rows,
    )
    .expect("write modulus rows");
    write_csv_rows(
        options.out_dir.join("period_lock_modulus_summary_rows.csv"),
        &modulus_summary_rows,
    )
    .expect("write modulus summary rows");
    write_csv_rows(
        options.out_dir.join("representative_period_lock_rows.csv"),
        &representative_rows,
    )
    .expect("write representative rows");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle).expect("write summary json");
    write_text_file(options.out_dir.join("report.md"), &report_text).expect("write report");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo".to_string(),
            args: vec![
                "run".to_string(),
                "--release".to_string(),
                "--example".to_string(),
                "affine_period_lock_report".to_string(),
            ],
            upstream_inputs: vec![
                "src/validation/affine_period_lock.rs".to_string(),
                "src/validation/bounded_k.rs".to_string(),
            ],
            expected_outputs: vec![
                "period_lock_comparison_rows.csv".to_string(),
                "period_lock_modulus_rows.csv".to_string(),
                "period_lock_modulus_summary_rows.csv".to_string(),
                "representative_period_lock_rows.csv".to_string(),
                "summary.json".to_string(),
                "report.md".to_string(),
                "artifact_manifest.json".to_string(),
                "period_lock_heatmap.png".to_string(),
                "period_lock_residual_plane.png".to_string(),
                "period_lock_representative_strip.png".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote affine period-lock bundle to {}",
        options.out_dir.display()
    );
    println!("{}", report_summary.exact_takeaway);
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--out-dir" {
            let value = args.next().expect("missing value after --out-dir");
            out_dir = PathBuf::from(value);
        } else {
            panic!("unrecognized argument: {arg}");
        }
    }
    Options { out_dir }
}

fn scope_for_base(base: u32) -> &'static str {
    if MAIN_BASES.contains(&base) {
        "main"
    } else {
        "appendix"
    }
}

fn build_comparison_bundles() -> Vec<ComparisonBundle> {
    let pair_middle_rows = MAIN_BASES
        .iter()
        .chain(APPENDIX_BASES.iter())
        .flat_map(|&base| {
            ordered_unit_pairs(base)
                .into_iter()
                .flat_map(move |(outer, inner)| {
                    MIDDLE_LENGTHS
                        .iter()
                        .copied()
                        .map(move |middle_length| (base, outer, inner, middle_length))
                })
        })
        .collect::<Vec<_>>();

    pair_middle_rows
        .into_par_iter()
        .flat_map(|(base, outer, inner, middle_length)| {
            let scope = scope_for_base(base).to_string();
            let pair_label = format!("({},{})", digit_symbol(outer), digit_symbol(inner));

            NONCOMPACT_LANES
                .iter()
                .copied()
                .map(|to_k| ComparisonBundle {
                    scope: scope.clone(),
                    base,
                    middle_length,
                    outer,
                    inner,
                    pair_label: pair_label.clone(),
                    same_digit: outer == inner,
                    comparison: scan_k_config_affine_period_lock_comparison(
                        base,
                        middle_length,
                        outer,
                        inner,
                        FROM_K,
                        to_k,
                    ),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn build_comparison_rows(bundles: &[ComparisonBundle]) -> Vec<PeriodLockComparisonCsvRow> {
    let mut rows = bundles
        .iter()
        .map(|bundle| PeriodLockComparisonCsvRow {
            scope: bundle.scope.clone(),
            base: bundle.base,
            middle_length: bundle.middle_length,
            outer: bundle.outer,
            inner: bundle.inner,
            pair_label: bundle.pair_label.clone(),
            same_digit: bundle.same_digit,
            to_k: bundle.comparison.to_k.clone(),
            gradient_position_from: bundle.comparison.gradient_position_from,
            gradient_position_to: bundle.comparison.gradient_position_to,
            gradient_position_delta: bundle.comparison.gradient_position_delta,
            compared_moduli_count: bundle.comparison.compared_moduli_count,
            period_lock_expected_count: bundle.comparison.period_lock_expected_count,
            observed_gradient_equal_count: bundle.comparison.observed_gradient_equal_count,
            period_lock_match_count: bundle.comparison.period_lock_match_count,
            period_lock_mismatch_count: bundle.comparison.period_lock_mismatch_count,
            period_lock_expected_share: bundle.comparison.period_lock_expected_share,
            observed_gradient_equal_share: bundle.comparison.observed_gradient_equal_share,
            period_lock_match_share: bundle.comparison.period_lock_match_share,
            period_lock_perfect: bundle.comparison.period_lock_perfect,
            same_shift_count: bundle.comparison.same_shift_count,
            same_zero_seed_count: bundle.comparison.same_zero_seed_count,
            identity_count: bundle.comparison.identity_count,
            shift_only_count: bundle.comparison.shift_only_count,
            gradient_only_count: bundle.comparison.gradient_only_count,
            shift_and_gradient_count: bundle.comparison.shift_and_gradient_count,
            identity_share: ratio(
                bundle.comparison.identity_count,
                bundle.comparison.compared_moduli_count,
            ),
            shift_only_share: ratio(
                bundle.comparison.shift_only_count,
                bundle.comparison.compared_moduli_count,
            ),
            gradient_only_share: ratio(
                bundle.comparison.gradient_only_count,
                bundle.comparison.compared_moduli_count,
            ),
            shift_and_gradient_share: ratio(
                bundle.comparison.shift_and_gradient_count,
                bundle.comparison.compared_moduli_count,
            ),
            period_locked_identity_count: bundle.comparison.period_locked_identity_count,
            period_locked_gradient_only_count: bundle.comparison.period_locked_gradient_only_count,
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.scope
            .cmp(&right.scope)
            .then_with(|| left.base.cmp(&right.base))
            .then_with(|| left.middle_length.cmp(&right.middle_length))
            .then_with(|| left.outer.cmp(&right.outer))
            .then_with(|| left.inner.cmp(&right.inner))
            .then_with(|| left.to_k.cmp(&right.to_k))
    });
    rows
}

fn build_modulus_rows(bundles: &[ComparisonBundle]) -> Vec<PeriodLockModulusCsvRow> {
    let mut rows = bundles
        .iter()
        .flat_map(|bundle| {
            bundle
                .comparison
                .modulus_rows
                .iter()
                .map(|row| PeriodLockModulusCsvRow {
                    scope: bundle.scope.clone(),
                    base: bundle.base,
                    middle_length: bundle.middle_length,
                    outer: bundle.outer,
                    inner: bundle.inner,
                    pair_label: bundle.pair_label.clone(),
                    to_k: bundle.comparison.to_k.clone(),
                    modulus: row.modulus,
                    base_modulus: row.base_modulus,
                    gradient_position_from: row.gradient_position_from,
                    gradient_position_to: row.gradient_position_to,
                    gradient_position_delta: row.gradient_position_delta,
                    multiplicative_order: row.multiplicative_order,
                    delta_mod_order: row.delta_mod_order,
                    period_lock_expected: row.period_lock_expected,
                    observed_gradient_equal: row.observed_gradient_equal,
                    expected_matches_observation: row.expected_matches_observation,
                    shift_equal: row.shift_equal,
                    zero_seed_equal: row.zero_seed_equal,
                    local_relation_label: row.local_relation_label.clone(),
                    shift_modulus_from: row.shift_modulus_from,
                    shift_modulus_to: row.shift_modulus_to,
                    gradient_modulus_from: row.gradient_modulus_from,
                    gradient_modulus_to: row.gradient_modulus_to,
                    zero_seed_class_from: row.zero_seed_class_from,
                    zero_seed_class_to: row.zero_seed_class_to,
                })
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.scope
            .cmp(&right.scope)
            .then_with(|| left.base.cmp(&right.base))
            .then_with(|| left.middle_length.cmp(&right.middle_length))
            .then_with(|| left.outer.cmp(&right.outer))
            .then_with(|| left.inner.cmp(&right.inner))
            .then_with(|| left.to_k.cmp(&right.to_k))
            .then_with(|| left.modulus.cmp(&right.modulus))
    });
    rows
}

fn build_modulus_summary_rows(
    rows: &[PeriodLockModulusCsvRow],
) -> Vec<PeriodLockModulusSummaryRow> {
    #[derive(Default)]
    struct Accumulator {
        pair_rows: usize,
        period_lock_expected_rows: usize,
        observed_gradient_equal_rows: usize,
        perfect_match_rows: usize,
        identity_rows: usize,
        gradient_only_rows: usize,
        shift_only_rows: usize,
        shift_and_gradient_rows: usize,
    }

    let mut accumulators: BTreeMap<ModulusSummaryKey, Accumulator> = BTreeMap::new();
    for row in rows {
        let key = (
            row.scope.clone(),
            row.base,
            row.middle_length,
            row.to_k.clone(),
            row.modulus,
            row.gradient_position_delta,
            row.multiplicative_order,
        );
        let entry = accumulators.entry(key).or_default();
        entry.pair_rows += 1;
        entry.period_lock_expected_rows += usize::from(row.period_lock_expected);
        entry.observed_gradient_equal_rows += usize::from(row.observed_gradient_equal);
        entry.perfect_match_rows += usize::from(row.expected_matches_observation);
        entry.identity_rows += usize::from(row.local_relation_label == "identity");
        entry.gradient_only_rows += usize::from(row.local_relation_label == "gradient_only");
        entry.shift_only_rows += usize::from(row.local_relation_label == "shift_only");
        entry.shift_and_gradient_rows +=
            usize::from(row.local_relation_label == "shift_and_gradient");
    }

    accumulators
        .into_iter()
        .map(
            |(
                (
                    scope,
                    base,
                    middle_length,
                    to_k,
                    modulus,
                    gradient_position_delta,
                    multiplicative_order,
                ),
                acc,
            )| PeriodLockModulusSummaryRow {
                scope,
                base,
                middle_length,
                to_k,
                modulus,
                gradient_position_delta,
                multiplicative_order,
                pair_rows: acc.pair_rows,
                period_lock_expected_pair_share: ratio(
                    acc.period_lock_expected_rows,
                    acc.pair_rows,
                ),
                observed_gradient_equal_pair_share: ratio(
                    acc.observed_gradient_equal_rows,
                    acc.pair_rows,
                ),
                perfect_match_pair_share: ratio(acc.perfect_match_rows, acc.pair_rows),
                identity_pair_share: ratio(acc.identity_rows, acc.pair_rows),
                gradient_only_pair_share: ratio(acc.gradient_only_rows, acc.pair_rows),
                shift_only_pair_share: ratio(acc.shift_only_rows, acc.pair_rows),
                shift_and_gradient_pair_share: ratio(acc.shift_and_gradient_rows, acc.pair_rows),
            },
        )
        .collect()
}

fn build_representative_rows(bundles: &[ComparisonBundle]) -> Vec<RepresentativePeriodLockRow> {
    let mut rows = Vec::new();
    for representative in REPRESENTATIVES {
        for bundle in bundles.iter().filter(|bundle| {
            bundle.base == representative.base
                && bundle.outer == representative.outer
                && bundle.inner == representative.inner
                && bundle.middle_length == 2
        }) {
            let locked_moduli = bundle
                .comparison
                .modulus_rows
                .iter()
                .filter(|row| row.period_lock_expected)
                .map(|row| row.modulus.to_string())
                .collect::<Vec<_>>();
            let gradient_equal_moduli = bundle
                .comparison
                .modulus_rows
                .iter()
                .filter(|row| row.observed_gradient_equal)
                .map(|row| row.modulus.to_string())
                .collect::<Vec<_>>();
            let identity_moduli = bundle
                .comparison
                .modulus_rows
                .iter()
                .filter(|row| row.local_relation_label == "identity")
                .map(|row| row.modulus.to_string())
                .collect::<Vec<_>>();
            let gradient_only_moduli = bundle
                .comparison
                .modulus_rows
                .iter()
                .filter(|row| row.local_relation_label == "gradient_only")
                .map(|row| row.modulus.to_string())
                .collect::<Vec<_>>();

            let note = if bundle.comparison.period_lock_expected_count == 0 {
                "no period lock on the coprime modulus surface".to_string()
            } else if bundle.comparison.gradient_only_count > 0
                && bundle.comparison.identity_count == 0
            {
                "period lock survives as a pure shift-controlled gradient_only pocket".to_string()
            } else if bundle.comparison.identity_count > 0
                && bundle.comparison.gradient_only_count == 0
            {
                "period lock collapses to identity because the local shifts also align".to_string()
            } else if bundle.comparison.gradient_only_count > 0
                && bundle.comparison.identity_count > 0
            {
                "period lock is mixed: some locked moduli collapse to identity and others remain gradient_only".to_string()
            } else {
                "period lock is present but does not drive a surviving relation pocket".to_string()
            };

            rows.push(RepresentativePeriodLockRow {
                role: representative.role.to_string(),
                base: bundle.base,
                pair_label: bundle.pair_label.clone(),
                to_k: bundle.comparison.to_k.clone(),
                gradient_position_delta: bundle.comparison.gradient_position_delta,
                period_lock_expected_count: bundle.comparison.period_lock_expected_count,
                observed_gradient_equal_count: bundle.comparison.observed_gradient_equal_count,
                period_lock_mismatch_count: bundle.comparison.period_lock_mismatch_count,
                locked_moduli_label: label_or_none(&locked_moduli),
                gradient_equal_moduli_label: label_or_none(&gradient_equal_moduli),
                identity_moduli_label: label_or_none(&identity_moduli),
                gradient_only_moduli_label: label_or_none(&gradient_only_moduli),
                note,
            });
        }
    }
    rows.sort_by(|left, right| {
        left.base
            .cmp(&right.base)
            .then_with(|| left.pair_label.cmp(&right.pair_label))
            .then_with(|| left.to_k.cmp(&right.to_k))
    });
    rows
}

fn build_report_summary(
    comparison_rows: &[PeriodLockComparisonCsvRow],
    modulus_rows: &[PeriodLockModulusCsvRow],
    modulus_summary_rows: &[PeriodLockModulusSummaryRow],
) -> ReportSummary {
    let mismatch_rows = modulus_rows
        .iter()
        .filter(|row| !row.expected_matches_observation)
        .count();
    let perfect_comparison_count = comparison_rows
        .iter()
        .filter(|row| row.period_lock_perfect)
        .count();
    let main_m2_positive_hotspot_count = modulus_summary_rows
        .iter()
        .filter(|row| {
            row.scope == "main"
                && row.middle_length == 2
                && row.observed_gradient_equal_pair_share > 0.0
        })
        .count();

    let exact_takeaway = if mismatch_rows == 0 {
        "period lock exactly matches observed affine gradient equality on the maintained coprime-modulus surface".to_string()
    } else {
        format!(
            "period lock misses {} of {} modulus rows on the maintained coprime-modulus surface",
            mismatch_rows,
            modulus_rows.len()
        )
    };

    ReportSummary {
        comparison_rows: comparison_rows.len(),
        modulus_rows: modulus_rows.len(),
        mismatch_rows,
        mismatch_share: ratio(mismatch_rows, modulus_rows.len()),
        perfect_comparison_share: ratio(perfect_comparison_count, comparison_rows.len()),
        main_m2_positive_hotspot_count,
        exact_takeaway,
    }
}

fn derive_observations(
    modulus_summary_rows: &[PeriodLockModulusSummaryRow],
    representative_rows: &[RepresentativePeriodLockRow],
) -> Vec<String> {
    let mut observations = Vec::new();

    let exact_rows = modulus_summary_rows
        .iter()
        .filter(|row| row.perfect_match_pair_share == 1.0)
        .count();
    observations.push(format!(
        "{} aggregated lane/modulus cells have perfect period-lock agreement between expected and observed gradient equality.",
        exact_rows
    ));

    let positive_hotspots = modulus_summary_rows
        .iter()
        .filter(|row| {
            row.scope == "main"
                && row.middle_length == 2
                && row.observed_gradient_equal_pair_share > 0.0
        })
        .collect::<Vec<_>>();
    if !positive_hotspots.is_empty() {
        let labels = positive_hotspots
            .iter()
            .map(|row| {
                format!(
                    "base {} {} mod {} (Δ={}, ord={})",
                    row.base,
                    row.to_k,
                    row.modulus,
                    row.gradient_position_delta,
                    row.multiplicative_order
                )
            })
            .collect::<Vec<_>>();
        observations.push(format!(
            "Positive main-surface M=2 hotspots are localized to: {}.",
            labels.join("; ")
        ));
    }

    if let Some(base22_k22) = modulus_summary_rows.iter().find(|row| {
        row.scope == "main"
            && row.base == 22
            && row.middle_length == 2
            && row.to_k == format_k((2, 2))
            && row.modulus == 5
    }) {
        observations.push(format!(
            "The base-22 anchor cell is exact and sharp: at M=2, {} mod 5 has Δ={} and ord={} with gradient-equality pair share {:.2}% and gradient_only pair share {:.2}%.",
            base22_k22.to_k,
            base22_k22.gradient_position_delta,
            base22_k22.multiplicative_order,
            base22_k22.observed_gradient_equal_pair_share * 100.0,
            base22_k22.gradient_only_pair_share * 100.0
        ));
    }

    if let Some(base22_k11) = modulus_summary_rows.iter().find(|row| {
        row.scope == "main"
            && row.base == 22
            && row.middle_length == 2
            && row.to_k == format_k((1, 1))
            && row.modulus == 5
    }) {
        observations.push(format!(
            "The shorter base-22 comparison {} stays unlocked at mod 5: Δ={} against ord={} gives observed gradient-equality pair share {:.2}%.",
            base22_k11.to_k,
            base22_k11.gradient_position_delta,
            base22_k11.multiplicative_order,
            base22_k11.observed_gradient_equal_pair_share * 100.0
        ));
    }

    if let Some(active_pocket) = representative_rows.iter().find(|row| {
        row.base == 22 && row.role == "active_neither_pocket" && row.to_k == format_k((2, 2))
    }) {
        observations.push(format!(
            "The active base-22 pocket keeps the lock on moduli {} and carries gradient_only on {}.",
            active_pocket.locked_moduli_label, active_pocket.gradient_only_moduli_label
        ));
    }

    observations
}

fn render_report(
    settings: &ReportSettings,
    summary: &ReportSummary,
    observations: &[String],
) -> String {
    let mut lines = Vec::new();
    lines.push("# Affine Period-Lock Report".to_string());
    lines.push(String::new());
    lines.push("## Settings".to_string());
    lines.push(format!("- output dir: `{}`", settings.out_dir));
    lines.push(format!("- main bases: {:?}", settings.main_bases));
    lines.push(format!("- appendix bases: {:?}", settings.appendix_bases));
    lines.push(format!("- middle lengths: {:?}", settings.middle_lengths));
    lines.push(format!("- from lane: `{}`", settings.from_k));
    lines.push(format!(
        "- noncompact lanes: {:?}",
        settings.noncompact_lanes
    ));
    lines.push(String::new());
    lines.push("## Exact Takeaway".to_string());
    lines.push(format!("- {}", summary.exact_takeaway));
    lines.push(format!(
        "- comparison rows: {}, modulus rows: {}, mismatches: {} ({:.2}%)",
        summary.comparison_rows,
        summary.modulus_rows,
        summary.mismatch_rows,
        summary.mismatch_share * 100.0
    ));
    lines.push(format!(
        "- perfect comparison share: {:.2}%",
        summary.perfect_comparison_share * 100.0
    ));
    lines.push(format!(
        "- positive main-surface M=2 hotspot cells: {}",
        summary.main_m2_positive_hotspot_count
    ));
    lines.push(String::new());
    lines.push("## Observations".to_string());
    for observation in observations {
        lines.push(format!("- {}", observation));
    }
    lines.push(String::new());
    lines.push("## Reading".to_string());
    lines.push(
        "- The period lock explains when local affine gradients can agree at all.".to_string(),
    );
    lines.push("- The remaining identity vs `gradient_only` split is then controlled by the local shift comparison, not by the gradient channel.".to_string());
    lines.push("- The base-22 / mod-5 pocket is therefore a canonical example of a period-locked gradient agreement that stays shift-misaligned rather than a one-off anomaly.".to_string());
    lines.join("\n")
}

fn render_period_lock_heatmap(rows: &[PeriodLockModulusSummaryRow], path: &Path) {
    let rows = rows
        .iter()
        .filter(|row| row.scope == "main")
        .collect::<Vec<_>>();
    let row_labels = rows
        .iter()
        .map(|row| format!("B{} M{} {}", row.base, row.middle_length, row.to_k))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let moduli = rows
        .iter()
        .map(|row| row.modulus)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let root = BitMapBackend::new(path, (1200, 760)).into_drawing_area();
    root.fill(&WHITE).expect("fill heatmap");
    root.draw(&Text::new(
        "Observed gradient-equality pair share by base / M / lane / modulus",
        (20, 24),
        ("sans-serif", 24).into_font(),
    ))
    .expect("draw heatmap title");

    let top = 70;
    let left = 210;
    let cell_w = 90;
    let cell_h = 32;
    let row_index = row_labels
        .iter()
        .enumerate()
        .map(|(idx, label)| (label.clone(), idx))
        .collect::<BTreeMap<_, _>>();
    let modulus_index = moduli
        .iter()
        .enumerate()
        .map(|(idx, modulus)| (*modulus, idx))
        .collect::<BTreeMap<_, _>>();

    for (idx, label) in row_labels.iter().enumerate() {
        let y = top + idx as i32 * cell_h + cell_h / 2;
        root.draw(&Text::new(
            label.clone(),
            (left - 10, y),
            ("sans-serif", 14).into_font(),
        ))
        .expect("draw heatmap row label");
    }

    for (idx, modulus) in moduli.iter().enumerate() {
        let x = left + idx as i32 * cell_w + cell_w / 2;
        root.draw(&Text::new(
            format!("mod {}", modulus),
            (x, top - 14),
            ("sans-serif", 14).into_font(),
        ))
        .expect("draw heatmap col label");
    }

    for row in rows {
        let row_label = format!("B{} M{} {}", row.base, row.middle_length, row.to_k);
        let row_idx = row_index[&row_label];
        let col_idx = modulus_index[&row.modulus];
        let x0 = left + col_idx as i32 * cell_w;
        let y0 = top + row_idx as i32 * cell_h;
        let color = heatmap_color(row.observed_gradient_equal_pair_share);
        root.draw(&Rectangle::new(
            [(x0, y0), (x0 + cell_w - 2, y0 + cell_h - 2)],
            ShapeStyle {
                color: color.to_rgba(),
                filled: true,
                stroke_width: 1,
            },
        ))
        .expect("draw heatmap cell");
        root.draw(&Text::new(
            format!("{:.0}", row.observed_gradient_equal_pair_share * 100.0),
            (x0 + 12, y0 + 21),
            ("sans-serif", 12).into_font().color(&BLACK),
        ))
        .expect("draw heatmap text");
    }

    root.present().expect("present heatmap");
}

fn render_period_lock_residual_plane(rows: &[PeriodLockComparisonCsvRow], path: &Path) {
    let rows = rows
        .iter()
        .filter(|row| row.scope == "main")
        .collect::<Vec<_>>();
    let root = BitMapBackend::new(path, (980, 760)).into_drawing_area();
    root.fill(&WHITE).expect("fill plane");

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Period-lock expected share vs gradient_only share",
            ("sans-serif", 24),
        )
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..1.0, 0.0..1.0)
        .expect("build chart");

    chart
        .configure_mesh()
        .x_desc("period_lock_expected_share")
        .y_desc("gradient_only_share")
        .draw()
        .expect("draw mesh");

    for row in rows {
        let radius = 4;
        let color = match row.middle_length {
            1 => RGBColor(0x1f, 0x77, 0xb4),
            2 => RGBColor(0xd6, 0x27, 0x28),
            3 => RGBColor(0x2c, 0xa0, 0x2c),
            _ => BLACK,
        };
        chart
            .draw_series(std::iter::once(Circle::new(
                (row.period_lock_expected_share, row.gradient_only_share),
                radius,
                ShapeStyle {
                    color: color.to_rgba(),
                    filled: true,
                    stroke_width: if row.base == 22 && row.to_k == format_k((2, 2)) {
                        2
                    } else {
                        1
                    },
                },
            )))
            .expect("draw point");
    }

    root.present().expect("present plane");
}

fn render_period_lock_representative_strip(rows: &[PeriodLockModulusCsvRow], path: &Path) {
    let rows = rows
        .iter()
        .filter(|row| {
            row.middle_length == 2
                && REPRESENTATIVES.iter().any(|representative| {
                    representative.base == row.base
                        && representative.outer == row.outer
                        && representative.inner == row.inner
                })
        })
        .collect::<Vec<_>>();
    let row_labels = rows
        .iter()
        .map(|row| format!("B{} {} {}", row.base, row.pair_label, row.to_k))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let moduli = rows
        .iter()
        .map(|row| row.modulus)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let root = BitMapBackend::new(path, (1100, 760)).into_drawing_area();
    root.fill(&WHITE).expect("fill strip");
    root.draw(&Text::new(
        "Representative M=2 local affine relations across moduli",
        (20, 24),
        ("sans-serif", 24).into_font(),
    ))
    .expect("draw strip title");

    let top = 70;
    let left = 230;
    let cell_w = 90;
    let cell_h = 28;
    let row_index = row_labels
        .iter()
        .enumerate()
        .map(|(idx, label)| (label.clone(), idx))
        .collect::<BTreeMap<_, _>>();
    let modulus_index = moduli
        .iter()
        .enumerate()
        .map(|(idx, modulus)| (*modulus, idx))
        .collect::<BTreeMap<_, _>>();

    for (idx, label) in row_labels.iter().enumerate() {
        let y = top + idx as i32 * cell_h + cell_h / 2;
        root.draw(&Text::new(
            label.clone(),
            (left - 10, y),
            ("sans-serif", 13).into_font(),
        ))
        .expect("draw strip row label");
    }

    for (idx, modulus) in moduli.iter().enumerate() {
        let x = left + idx as i32 * cell_w + cell_w / 2;
        root.draw(&Text::new(
            format!("mod {}", modulus),
            (x, top - 14),
            ("sans-serif", 13).into_font(),
        ))
        .expect("draw strip col label");
    }

    for row in rows {
        let row_label = format!("B{} {} {}", row.base, row.pair_label, row.to_k);
        let row_idx = row_index[&row_label];
        let col_idx = modulus_index[&row.modulus];
        let x0 = left + col_idx as i32 * cell_w;
        let y0 = top + row_idx as i32 * cell_h;
        let color = relation_color(&row.local_relation_label);
        root.draw(&Rectangle::new(
            [(x0, y0), (x0 + cell_w - 2, y0 + cell_h - 2)],
            ShapeStyle {
                color: color.to_rgba(),
                filled: true,
                stroke_width: if row.period_lock_expected { 2 } else { 1 },
            },
        ))
        .expect("draw strip cell");
        root.draw(&Text::new(
            row.local_relation_label.clone(),
            (x0 + 4, y0 + 18),
            ("sans-serif", 10).into_font().color(&BLACK),
        ))
        .expect("draw strip text");
    }

    root.present().expect("present strip");
}

fn heatmap_color(value: f64) -> RGBColor {
    let clamped = value.clamp(0.0, 1.0);
    let red = (255.0 * clamped) as u8;
    let blue = (255.0 * (1.0 - clamped)) as u8;
    RGBColor(red, 120, blue)
}

fn relation_color(label: &str) -> RGBColor {
    match label {
        "identity" => RGBColor(0x4d, 0xc4, 0x6c),
        "shift_only" => RGBColor(0xf0, 0xc0, 0x4c),
        "gradient_only" => RGBColor(0xd6, 0x27, 0x28),
        "shift_and_gradient" => RGBColor(0x94, 0x67, 0xbd),
        _ => RGBColor(0xcc, 0xcc, 0xcc),
    }
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn label_or_none(labels: &[String]) -> String {
    if labels.is_empty() {
        "none".to_string()
    } else {
        labels.join(",")
    }
}
