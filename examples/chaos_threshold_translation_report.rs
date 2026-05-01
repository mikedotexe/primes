//! Chaos-to-threshold translation report for the bounded-`k` transition lane.
//!
//! This report deliberately leaves `src/chaos/*` on the metaphor side.
//! Instead, it asks what arithmetic threshold is actually present once we
//! translate the maintained bounded-`k` transition artifacts into exact
//! transition, admissible-set, and shared-prime-yield language.
//!
//! Prime density appears here only as a guardrail:
//! raw prime-rate changes are shown so we do not confuse anomaly mass with
//! admissible-set change, but this report does not claim a density theorem.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example chaos_threshold_translation_report
//! cargo run --release --example chaos_threshold_translation_report -- --full
//! ```

use plotters::prelude::*;
use primes::validation::{
    bounded_k::{
        analyze_pair_row_best_vs_k00, evaluate_pair_row, format_k, ordered_unit_pairs,
        select_smoke_pairs, DEFAULT_BOUNDED_K_GRID,
    },
    reporting::{
        ensure_dir, export_timestamp_utc, write_artifact_manifest, write_csv_rows,
        write_json_pretty, write_text_file, ArtifactManifest,
    },
};
use rayon::prelude::*;
use serde::Serialize;
use std::{
    collections::BTreeMap,
    env,
    path::{Path, PathBuf},
};

const BASES: &[u32] = &[6, 10, 12, 14, 22, 26, 30, 34];
const DEFAULT_MIN_MIDDLE_LENGTH: usize = 1;
const DEFAULT_MAX_MIDDLE_LENGTH: usize = 3;
const DEFAULT_OUT_DIR: &str = "/tmp/primes_chaos_threshold_translation";
const REPORT_EXPORT_VERSION: u32 = 1;
const ARTIFACT_ID: &str = "chaos_threshold_translation_report";
const SMOKE_MAX_ORDERED_PAIRS_PER_BASE: usize = 8;
const SMOKE_PAIR_ANCHORS: &[(u32, u32, u32)] = &[
    (6, 5, 5),
    (6, 5, 1),
    (10, 3, 3),
    (10, 3, 7),
    (12, 11, 1),
    (12, 1, 1),
    (14, 3, 1),
    (14, 13, 11),
    (22, 17, 19),
    (22, 1, 3),
    (26, 23, 23),
    (26, 1, 3),
    (30, 11, 7),
    (30, 17, 13),
    (34, 25, 9),
    (34, 27, 3),
    (34, 15, 13),
];

const REGIME_STABLE: &str = "stable_regime";
const REGIME_BOUNDARY: &str = "boundary_layer";
const REGIME_ANOMALY_RICH: &str = "anomaly_rich";

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    full_catalog: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    bases: Vec<u32>,
    pair_catalog_mode: String,
    max_ordered_pairs_per_base: Option<usize>,
    min_middle_length: usize,
    max_middle_length: usize,
    k_grid: Vec<String>,
    density_role: String,
}

#[derive(Debug, Clone, Serialize)]
struct PairThresholdRow {
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    pair_label: String,
    best_k: String,
    k00_noninferior: bool,
    active: bool,
    best_minus_k00_pp: f64,
    anomaly_mass_pp: f64,
    prime_hits_k00: usize,
    prime_hits_best: usize,
    rate_k00_pct: f64,
    rate_best_pct: f64,
    admissible_delta_pp: f64,
    stable_zero_prime_delta_pp: f64,
    boundary_prime_delta_pp: f64,
    shared_prime_rate_delta_pp: f64,
    signal_source_label: String,
}

#[derive(Debug, Clone, Serialize)]
struct BaseMRow {
    base: u32,
    middle_length: usize,
    ordered_pair_count: usize,
    active_pair_count: usize,
    active_pair_share: f64,
    k00_noninferior_share: f64,
    anomaly_mass_pp: f64,
    mean_anomaly_mass_pp_given_active: Option<f64>,
    mean_admissible_delta_pp_given_active: Option<f64>,
    mean_stable_zero_prime_delta_pp_given_active: Option<f64>,
    mean_boundary_prime_delta_pp_given_active: Option<f64>,
    mean_shared_prime_rate_delta_pp_given_active: Option<f64>,
    mean_rate_k00_pct_given_active: Option<f64>,
    mean_rate_best_pct_given_active: Option<f64>,
    dominant_signal_source_label: String,
    leading_pair: String,
    leading_best_k: String,
    leading_signal_source_label: String,
    regime_label: String,
}

#[derive(Debug, Clone, Serialize)]
struct MetaphorTranslationRow {
    phrase: String,
    status: String,
    arithmetic_meaning: String,
    maintained_verdict: String,
    evidence_anchor: String,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    total_pair_rows: usize,
    active_pair_rows: usize,
    first_stable_length_all_bases: Option<usize>,
    main_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    pair_threshold_rows: Vec<PairThresholdRow>,
    base_m_rows: Vec<BaseMRow>,
    metaphor_translation_rows: Vec<MetaphorTranslationRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        bases: BASES.to_vec(),
        pair_catalog_mode: if options.full_catalog {
            "full".to_string()
        } else {
            "smoke".to_string()
        },
        max_ordered_pairs_per_base: if options.full_catalog {
            None
        } else {
            Some(SMOKE_MAX_ORDERED_PAIRS_PER_BASE)
        },
        min_middle_length: DEFAULT_MIN_MIDDLE_LENGTH,
        max_middle_length: DEFAULT_MAX_MIDDLE_LENGTH,
        k_grid: DEFAULT_BOUNDED_K_GRID
            .iter()
            .map(|&config| format_k(config))
            .collect(),
        density_role: "guardrail_only".to_string(),
    };

    let pair_threshold_rows = build_pair_threshold_rows(options.full_catalog);
    let base_m_rows = build_base_m_rows(&pair_threshold_rows);
    let metaphor_translation_rows = build_metaphor_translation_rows();

    let threshold_curve_path = options.out_dir.join("threshold_curve.png");
    render_threshold_curve(&base_m_rows, &threshold_curve_path);
    let threshold_decomposition_path = options.out_dir.join("threshold_decomposition_bars.png");
    render_threshold_decomposition_bars(&base_m_rows, &threshold_decomposition_path);
    let threshold_regime_grid_path = options.out_dir.join("threshold_regime_grid.png");
    render_threshold_regime_grid(&base_m_rows, &threshold_regime_grid_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "threshold_curve".to_string(),
            label: "Threshold curve".to_string(),
            path: threshold_curve_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "threshold_decomposition_bars".to_string(),
            label: "Threshold decomposition bars".to_string(),
            path: threshold_decomposition_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "threshold_regime_grid".to_string(),
            label: "Threshold regime grid".to_string(),
            path: threshold_regime_grid_path.display().to_string(),
        },
    ];

    let report_summary = build_report_summary(&base_m_rows, &pair_threshold_rows);
    let observations = derive_observations(&base_m_rows, &pair_threshold_rows);

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        pair_threshold_rows,
        base_m_rows,
        metaphor_translation_rows,
        image_artifact_rows,
        report_summary,
        observations,
    };

    write_csv_rows(
        options.out_dir.join("pair_threshold_rows.csv"),
        &bundle.pair_threshold_rows,
    )
    .expect("failed to write pair threshold rows");
    write_csv_rows(options.out_dir.join("base_m_rows.csv"), &bundle.base_m_rows)
        .expect("failed to write base-m rows");
    write_csv_rows(
        options.out_dir.join("metaphor_translation_rows.csv"),
        &bundle.metaphor_translation_rows,
    )
    .expect("failed to write metaphor translation rows");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle)
        .expect("failed to write summary json");
    write_text_file(options.out_dir.join("report.md"), &render_markdown(&bundle))
        .expect("failed to write report markdown");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo".to_string(),
            args: report_manifest_args(&options),
            upstream_inputs: Vec::new(),
            expected_outputs: vec![
                "pair_threshold_rows.csv".to_string(),
                "base_m_rows.csv".to_string(),
                "metaphor_translation_rows.csv".to_string(),
                "summary.json".to_string(),
                "report.md".to_string(),
                "threshold_curve.png".to_string(),
                "threshold_decomposition_bars.png".to_string(),
                "threshold_regime_grid.png".to_string(),
            ],
        },
    )
    .expect("failed to write artifact manifest");

    print_summary(&bundle);
}

fn report_manifest_args(options: &Options) -> Vec<String> {
    let mut args = vec![
        "run".to_string(),
        "--release".to_string(),
        "--example".to_string(),
        "chaos_threshold_translation_report".to_string(),
        "--".to_string(),
        "--out-dir".to_string(),
        options.out_dir.display().to_string(),
    ];
    if options.full_catalog {
        args.push("--full".to_string());
    }
    args
}

fn parse_args() -> Options {
    let mut args = env::args().skip(1);
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut full_catalog = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => {
                out_dir = PathBuf::from(parse_next::<String>(&mut args, "--out-dir"));
            }
            "--full" => {
                full_catalog = true;
            }
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            other => {
                eprintln!("Unrecognized argument: {other}");
                print_usage();
                std::process::exit(1);
            }
        }
    }

    Options {
        out_dir,
        full_catalog,
    }
}

fn print_usage() {
    println!("Usage:");
    println!("  cargo run --release --example chaos_threshold_translation_report -- [options]");
    println!();
    println!("Options:");
    println!("  --out-dir <path>   Output directory (default: {DEFAULT_OUT_DIR})");
    println!("  --full             Use the full ordered-pair catalog instead of the smoke catalog");
    println!("  -h, --help         Show this help text");
}

fn parse_next<T>(args: &mut impl Iterator<Item = String>, flag: &str) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let value = args
        .next()
        .unwrap_or_else(|| panic!("missing value for {flag}"));
    value
        .parse::<T>()
        .unwrap_or_else(|err| panic!("invalid value for {flag}: {err}"))
}

fn build_pair_threshold_rows(full_catalog: bool) -> Vec<PairThresholdRow> {
    let mut rows = BASES
        .par_iter()
        .map(|&base| {
            let pairs = pair_catalog_for_base(base, full_catalog);
            let mut rows = Vec::with_capacity(
                pairs.len() * (DEFAULT_MAX_MIDDLE_LENGTH - DEFAULT_MIN_MIDDLE_LENGTH + 1),
            );
            for middle_length in DEFAULT_MIN_MIDDLE_LENGTH..=DEFAULT_MAX_MIDDLE_LENGTH {
                for (outer, inner) in pairs.iter().copied() {
                    let pair_row = evaluate_pair_row(
                        base,
                        middle_length,
                        outer,
                        inner,
                        DEFAULT_BOUNDED_K_GRID,
                    );
                    let decomposition =
                        analyze_pair_row_best_vs_k00(base, middle_length, outer, inner, &pair_row);
                    rows.push(PairThresholdRow {
                        base,
                        middle_length,
                        outer,
                        inner,
                        pair_label: pair_row.pair_label.clone(),
                        best_k: pair_row.best_k.clone(),
                        k00_noninferior: pair_row.k00_noninferior,
                        active: decomposition.anomaly_mass_pp > 0.0,
                        best_minus_k00_pp: pair_row.best_minus_k00_pp,
                        anomaly_mass_pp: decomposition.anomaly_mass_pp,
                        prime_hits_k00: pair_row.prime_hits_k00,
                        prime_hits_best: pair_row.best_prime_hits,
                        rate_k00_pct: pair_row.rate_k00 * 100.0,
                        rate_best_pct: pair_row.best_rate * 100.0,
                        admissible_delta_pp: decomposition.admissible_delta_pp,
                        stable_zero_prime_delta_pp: decomposition.stable_zero_prime_delta_pp,
                        boundary_prime_delta_pp: decomposition.boundary_prime_delta_pp,
                        shared_prime_rate_delta_pp: decomposition.shared_prime_rate_delta_pp,
                        signal_source_label: decomposition.signal_source_label,
                    });
                }
            }
            rows
        })
        .flatten()
        .collect::<Vec<_>>();

    rows.sort_by(|left, right| {
        left.base
            .cmp(&right.base)
            .then_with(|| left.middle_length.cmp(&right.middle_length))
            .then_with(|| left.outer.cmp(&right.outer))
            .then_with(|| left.inner.cmp(&right.inner))
    });
    rows
}

fn pair_catalog_for_base(base: u32, full_catalog: bool) -> Vec<(u32, u32)> {
    if full_catalog {
        ordered_unit_pairs(base)
    } else {
        let anchors = SMOKE_PAIR_ANCHORS
            .iter()
            .filter_map(|&(anchor_base, outer, inner)| {
                (anchor_base == base).then_some((outer, inner))
            })
            .collect::<Vec<_>>();
        select_smoke_pairs(base, SMOKE_MAX_ORDERED_PAIRS_PER_BASE, &anchors)
    }
}

fn build_base_m_rows(pair_rows: &[PairThresholdRow]) -> Vec<BaseMRow> {
    let mut by_group = BTreeMap::<(u32, usize), Vec<&PairThresholdRow>>::new();
    for row in pair_rows {
        by_group
            .entry((row.base, row.middle_length))
            .or_default()
            .push(row);
    }

    by_group
        .into_iter()
        .map(|((base, middle_length), rows)| {
            let ordered_pair_count = rows.len();
            let active_rows = rows
                .iter()
                .copied()
                .filter(|row| row.active)
                .collect::<Vec<_>>();
            let active_pair_count = active_rows.len();
            let leading_row = rows
                .iter()
                .copied()
                .max_by(|left, right| {
                    left.anomaly_mass_pp
                        .total_cmp(&right.anomaly_mass_pp)
                        .then_with(|| left.pair_label.cmp(&right.pair_label))
                })
                .expect("base/m group should not be empty");

            BaseMRow {
                base,
                middle_length,
                ordered_pair_count,
                active_pair_count,
                active_pair_share: ratio(active_pair_count, ordered_pair_count),
                k00_noninferior_share: ratio(
                    rows.iter().filter(|row| row.k00_noninferior).count(),
                    ordered_pair_count,
                ),
                anomaly_mass_pp: rows.iter().map(|row| row.anomaly_mass_pp).sum::<f64>(),
                mean_anomaly_mass_pp_given_active: mean_option(
                    active_rows.iter().map(|row| row.anomaly_mass_pp),
                ),
                mean_admissible_delta_pp_given_active: mean_option(
                    active_rows.iter().map(|row| row.admissible_delta_pp),
                ),
                mean_stable_zero_prime_delta_pp_given_active: mean_option(
                    active_rows.iter().map(|row| row.stable_zero_prime_delta_pp),
                ),
                mean_boundary_prime_delta_pp_given_active: mean_option(
                    active_rows.iter().map(|row| row.boundary_prime_delta_pp),
                ),
                mean_shared_prime_rate_delta_pp_given_active: mean_option(
                    active_rows.iter().map(|row| row.shared_prime_rate_delta_pp),
                ),
                mean_rate_k00_pct_given_active: mean_option(
                    active_rows.iter().map(|row| row.rate_k00_pct),
                ),
                mean_rate_best_pct_given_active: mean_option(
                    active_rows.iter().map(|row| row.rate_best_pct),
                ),
                dominant_signal_source_label: dominant_label(
                    active_rows
                        .iter()
                        .map(|row| row.signal_source_label.as_str()),
                    "none",
                ),
                leading_pair: leading_row.pair_label.clone(),
                leading_best_k: leading_row.best_k.clone(),
                leading_signal_source_label: leading_row.signal_source_label.clone(),
                regime_label: regime_label(
                    active_pair_count,
                    ordered_pair_count,
                    rows.iter().filter(|row| row.k00_noninferior).count(),
                )
                .to_string(),
            }
        })
        .collect()
}

fn build_metaphor_translation_rows() -> Vec<MetaphorTranslationRow> {
    vec![
        MetaphorTranslationRow {
            phrase: "stable regime".to_string(),
            status: "live".to_string(),
            arithmetic_meaning:
                "k=(0,0) is noninferior and positive anomaly mass has collapsed to zero."
                    .to_string(),
            maintained_verdict:
                "Use only for bounded-k transition rows where the arithmetic artifact shows no active counterexample pairs."
                    .to_string(),
            evidence_anchor:
                "examples/m_transition_curve_report.rs; examples/chaos_threshold_translation_report.rs"
                    .to_string(),
        },
        MetaphorTranslationRow {
            phrase: "boundary layer / edge of chaos".to_string(),
            status: "live".to_string(),
            arithmetic_meaning:
                "Positive anomaly mass survives only at short middle lengths and only in sparse pair classes."
                    .to_string(),
            maintained_verdict:
                "Allowed only when tied to bounded-k transition artifacts and exact mask/yield decomposition, not to Lyapunov outputs."
                    .to_string(),
            evidence_anchor:
                "examples/m2_m3_transition_report.rs; examples/m_transition_phase_map_report.rs; examples/chaos_threshold_translation_report.rs"
                    .to_string(),
        },
        MetaphorTranslationRow {
            phrase: "deep chaos regime / midpoint threshold claims".to_string(),
            status: "retired".to_string(),
            arithmetic_meaning:
                "No maintained arithmetic replacement. Midpoint-style chaos claims are not part of the bounded-k evidence lane."
                    .to_string(),
            maintained_verdict:
                "Retired from maintained claim language; historical interest only."
                    .to_string(),
            evidence_anchor:
                "VERIFIED_FACTS_VS_SPECULATION.md; src/chaos/mod.rs".to_string(),
        },
        MetaphorTranslationRow {
            phrase: "chaos storm / attractor language".to_string(),
            status: "retired".to_string(),
            arithmetic_meaning:
                "No current arithmetic object in the maintained transition lane justifies attractor-style wording."
                    .to_string(),
            maintained_verdict:
                "Retire unless a future arithmetic classifier replaces the metaphor directly."
                    .to_string(),
            evidence_anchor:
                "src/chaos/mod.rs; collab/CHAOS_THRESHOLD_TRANSLATION.md".to_string(),
        },
    ]
}

fn build_report_summary(base_rows: &[BaseMRow], pair_rows: &[PairThresholdRow]) -> ReportSummary {
    let first_stable_length_all_bases = (DEFAULT_MIN_MIDDLE_LENGTH..=DEFAULT_MAX_MIDDLE_LENGTH)
        .find(|&middle_length| {
            base_rows
                .iter()
                .filter(|row| row.middle_length == middle_length)
                .all(|row| row.active_pair_count == 0 && row.k00_noninferior_share >= 1.0)
        });

    ReportSummary {
        total_pair_rows: pair_rows.len(),
        active_pair_rows: pair_rows.iter().filter(|row| row.active).count(),
        first_stable_length_all_bases,
        main_takeaway: "The maintained threshold is arithmetic, not simulation-based: M=1 is anomaly-rich, M=2 is a sparse boundary layer, and M=3 is stable on this catalog, while base 14 stays overlap-positive at M=2 and base 34 stays boundary-led.".to_string(),
    }
}

fn derive_observations(base_rows: &[BaseMRow], pair_rows: &[PairThresholdRow]) -> Vec<String> {
    let global_by_m = (DEFAULT_MIN_MIDDLE_LENGTH..=DEFAULT_MAX_MIDDLE_LENGTH)
        .map(|middle_length| {
            let rows = base_rows
                .iter()
                .filter(|row| row.middle_length == middle_length)
                .collect::<Vec<_>>();
            let anomaly_mass_pp = rows.iter().map(|row| row.anomaly_mass_pp).sum::<f64>();
            let active_pair_count = rows.iter().map(|row| row.active_pair_count).sum::<usize>();
            let ordered_pair_count = rows.iter().map(|row| row.ordered_pair_count).sum::<usize>();
            (
                middle_length,
                anomaly_mass_pp,
                active_pair_count,
                ordered_pair_count,
            )
        })
        .collect::<Vec<_>>();

    let m2_base14 = base_rows
        .iter()
        .find(|row| row.base == 14 && row.middle_length == 2)
        .expect("base 14 M=2 row should exist");
    let m2_base34 = base_rows
        .iter()
        .find(|row| row.base == 34 && row.middle_length == 2)
        .expect("base 34 M=2 row should exist");

    let strongest_difference_row = pair_rows
        .iter()
        .filter(|row| row.active)
        .max_by(|left, right| {
            (left.anomaly_mass_pp - left.admissible_delta_pp)
                .abs()
                .total_cmp(&(right.anomaly_mass_pp - right.admissible_delta_pp).abs())
        })
        .expect("at least one active pair row should exist");

    let mut observations = vec![];
    for (middle_length, anomaly_mass_pp, active_pair_count, ordered_pair_count) in global_by_m {
        let regime = if active_pair_count == 0 {
            "stable"
        } else if ratio(active_pair_count, ordered_pair_count) >= 0.20 {
            "anomaly-rich"
        } else {
            "boundary-layer"
        };
        observations.push(format!(
            "Across all maintained bases, M={} is `{}` with total anomaly mass {:.2}pp and {}/{} active ordered pairs.",
            middle_length, regime, anomaly_mass_pp, active_pair_count, ordered_pair_count
        ));
    }

    observations.push(format!(
        "Base 14 and base 34 stay visibly different at M=2: base 14 has mean stable-zero delta {} with signal source `{}`, while base 34 has mean stable-zero delta {} with signal source `{}`.",
        format_option_pp(m2_base14.mean_stable_zero_prime_delta_pp_given_active),
        m2_base14.dominant_signal_source_label,
        format_option_pp(m2_base34.mean_stable_zero_prime_delta_pp_given_active),
        m2_base34.dominant_signal_source_label,
    ));

    observations.push(format!(
        "This lane treats density as a guardrail only: the active row with the largest separation between raw anomaly and admissible change is base {} {} at M={}, where anomaly mass is {:.2}pp but admissible delta is {:.2}pp.",
        strongest_difference_row.base,
        strongest_difference_row.pair_label,
        strongest_difference_row.middle_length,
        strongest_difference_row.anomaly_mass_pp,
        strongest_difference_row.admissible_delta_pp,
    ));

    observations.push(
        "The maintained threshold statement is purely arithmetic: it is tied to bounded-k transition artifacts, admissible-set deltas, and shared-prime-yield structure, not to simulation Lyapunov outputs."
            .to_string(),
    );

    observations
}

fn render_threshold_curve(rows: &[BaseMRow], path: &Path) {
    let root = BitMapBackend::new(path, (1360, 860)).into_drawing_area();
    root.fill(&WHITE)
        .expect("failed to clear threshold curve background");

    let y_max = rows
        .iter()
        .map(|row| row.anomaly_mass_pp)
        .fold(0.0_f64, f64::max)
        + 1.0;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Arithmetic Threshold Curve  (total anomaly mass by base)",
            ("sans-serif", 28),
        )
        .margin(28)
        .x_label_area_size(56)
        .y_label_area_size(72)
        .build_cartesian_2d(
            (DEFAULT_MIN_MIDDLE_LENGTH as f64 - 0.2)..(DEFAULT_MAX_MIDDLE_LENGTH as f64 + 0.2),
            0.0f64..y_max.max(1.0),
        )
        .expect("failed to build threshold curve chart");

    chart
        .configure_mesh()
        .x_desc("Middle length M")
        .y_desc("Total positive anomaly mass (pp)")
        .x_labels(DEFAULT_MAX_MIDDLE_LENGTH - DEFAULT_MIN_MIDDLE_LENGTH + 1)
        .x_label_formatter(&|value| format!("M{}", value.round() as usize))
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(90, 84, 76))
        .light_line_style(RGBColor(224, 218, 209))
        .draw()
        .expect("failed to draw threshold curve mesh");

    for (index, &base) in BASES.iter().enumerate() {
        let color = base_color(index);
        let base_rows = rows
            .iter()
            .filter(|row| row.base == base)
            .collect::<Vec<_>>();
        let points = base_rows
            .iter()
            .map(|row| (row.middle_length as f64, row.anomaly_mass_pp))
            .collect::<Vec<_>>();

        chart
            .draw_series(LineSeries::new(
                points.clone(),
                ShapeStyle::from(&color).stroke_width(3),
            ))
            .expect("failed to draw threshold curve line")
            .label(format!("base {base}"))
            .legend(move |(x, y)| {
                PathElement::new(
                    vec![(x, y), (x + 24, y)],
                    ShapeStyle::from(&color).stroke_width(3),
                )
            });

        chart
            .draw_series(
                points
                    .into_iter()
                    .map(|point| Circle::new(point, 5, ShapeStyle::from(&color).filled())),
            )
            .expect("failed to draw threshold curve points");
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.9))
        .border_style(BLACK.mix(0.2))
        .label_font(("sans-serif", 14))
        .draw()
        .expect("failed to draw threshold curve legend");

    root.present().expect("failed to present threshold curve");
}

fn render_threshold_decomposition_bars(rows: &[BaseMRow], path: &Path) {
    let root = BitMapBackend::new(path, (2200, 760)).into_drawing_area();
    root.fill(&WHITE)
        .expect("failed to clear decomposition background");
    let panels = root.split_evenly((1, 3));

    let mut metric_values = vec![];
    for row in rows {
        metric_values.push(row.mean_admissible_delta_pp_given_active.unwrap_or(0.0));
        metric_values.push(
            row.mean_stable_zero_prime_delta_pp_given_active
                .unwrap_or(0.0),
        );
        metric_values.push(row.mean_boundary_prime_delta_pp_given_active.unwrap_or(0.0));
        metric_values.push(
            row.mean_shared_prime_rate_delta_pp_given_active
                .unwrap_or(0.0),
        );
    }
    let y_min = metric_values.iter().copied().fold(0.0_f64, f64::min) - 0.8;
    let y_max = metric_values.iter().copied().fold(0.0_f64, f64::max) + 0.8;

    let metric_offsets = [-0.27, -0.09, 0.09, 0.27];
    let metric_width = 0.16;
    let metric_colors = [
        RGBColor(201, 157, 86),
        RGBColor(58, 125, 144),
        RGBColor(197, 94, 74),
        RGBColor(121, 91, 184),
    ];
    let metric_labels = [
        "admissible delta",
        "stable-zero delta",
        "boundary delta",
        "shared rate delta",
    ];

    for (panel_index, panel) in panels.into_iter().enumerate() {
        let middle_length = DEFAULT_MIN_MIDDLE_LENGTH + panel_index;
        let mut chart = ChartBuilder::on(&panel)
            .caption(
                format!("M={middle_length} decomposition"),
                ("sans-serif", 24),
            )
            .margin(24)
            .x_label_area_size(42)
            .y_label_area_size(70)
            .build_cartesian_2d(-0.6f64..(BASES.len() as f64 - 0.4), y_min..y_max)
            .expect("failed to build decomposition panel");

        chart
            .configure_mesh()
            .x_desc("Base")
            .y_desc("Mean delta over active pairs (pp)")
            .x_labels(BASES.len())
            .x_label_formatter(&|value| {
                let index = value.round().clamp(0.0, (BASES.len() - 1) as f64) as usize;
                BASES[index].to_string()
            })
            .label_style(("sans-serif", 14))
            .axis_style(RGBColor(90, 84, 76))
            .light_line_style(RGBColor(226, 220, 212))
            .draw()
            .expect("failed to draw decomposition mesh");

        chart
            .draw_series(LineSeries::new(
                vec![(-0.6, 0.0), (BASES.len() as f64 - 0.4, 0.0)],
                ShapeStyle::from(&BLACK.mix(0.35)).stroke_width(1),
            ))
            .expect("failed to draw zero line");

        for (metric_index, ((&offset, color), label)) in metric_offsets
            .iter()
            .zip(metric_colors.iter())
            .zip(metric_labels.iter())
            .enumerate()
        {
            let series = BASES.iter().enumerate().map(|(base_index, &base)| {
                let row = rows
                    .iter()
                    .find(|row| row.base == base && row.middle_length == middle_length)
                    .expect("base/m row should exist");
                let value = match metric_index {
                    0 => row.mean_admissible_delta_pp_given_active.unwrap_or(0.0),
                    1 => row
                        .mean_stable_zero_prime_delta_pp_given_active
                        .unwrap_or(0.0),
                    2 => row.mean_boundary_prime_delta_pp_given_active.unwrap_or(0.0),
                    3 => row
                        .mean_shared_prime_rate_delta_pp_given_active
                        .unwrap_or(0.0),
                    _ => unreachable!(),
                };
                let center = base_index as f64 + offset;
                Rectangle::new(
                    [
                        (center - metric_width / 2.0, 0.0),
                        (center + metric_width / 2.0, value),
                    ],
                    ShapeStyle::from(color).filled(),
                )
            });

            chart
                .draw_series(series)
                .expect("failed to draw decomposition bars")
                .label(*label)
                .legend(move |(x, y)| {
                    Rectangle::new(
                        [(x, y - 5), (x + 16, y + 5)],
                        ShapeStyle::from(color).filled(),
                    )
                });
        }

        if middle_length == DEFAULT_MIN_MIDDLE_LENGTH {
            chart
                .configure_series_labels()
                .background_style(WHITE.mix(0.92))
                .border_style(BLACK.mix(0.2))
                .label_font(("sans-serif", 14))
                .draw()
                .expect("failed to draw decomposition legend");
        }
    }

    root.present()
        .expect("failed to present decomposition chart");
}

fn render_threshold_regime_grid(rows: &[BaseMRow], path: &Path) {
    let root = BitMapBackend::new(path, (1180, 740)).into_drawing_area();
    root.fill(&WHITE)
        .expect("failed to clear regime grid background");

    let mut chart = ChartBuilder::on(&root)
        .caption("Arithmetic Threshold Regime Grid", ("sans-serif", 28))
        .margin(28)
        .x_label_area_size(56)
        .y_label_area_size(76)
        .build_cartesian_2d(
            0.0f64..(DEFAULT_MAX_MIDDLE_LENGTH - DEFAULT_MIN_MIDDLE_LENGTH + 1) as f64,
            0.0f64..BASES.len() as f64,
        )
        .expect("failed to build regime grid");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("Middle length M")
        .y_desc("Base")
        .x_labels(DEFAULT_MAX_MIDDLE_LENGTH - DEFAULT_MIN_MIDDLE_LENGTH + 1)
        .y_labels(BASES.len())
        .x_label_formatter(&|value| {
            let middle_length = DEFAULT_MIN_MIDDLE_LENGTH + value.round() as usize;
            format!("M{middle_length}")
        })
        .y_label_formatter(&|value| {
            let index = value.floor().clamp(0.0, (BASES.len() - 1) as f64) as usize;
            BASES[index].to_string()
        })
        .label_style(("sans-serif", 16))
        .draw()
        .expect("failed to draw regime grid mesh");

    for (base_index, &base) in BASES.iter().enumerate() {
        for middle_length in DEFAULT_MIN_MIDDLE_LENGTH..=DEFAULT_MAX_MIDDLE_LENGTH {
            let row = rows
                .iter()
                .find(|row| row.base == base && row.middle_length == middle_length)
                .expect("base/m row should exist");
            let color = regime_color(&row.regime_label);
            let x0 = (middle_length - DEFAULT_MIN_MIDDLE_LENGTH) as f64;
            let x1 = x0 + 1.0;
            let y0 = base_index as f64;
            let y1 = y0 + 1.0;

            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(x0, y0), (x1, y1)],
                    ShapeStyle::from(&color).filled(),
                )))
                .expect("failed to draw regime cell");
            chart
                .draw_series(std::iter::once(Text::new(
                    format!("{}/{}", row.active_pair_count, row.ordered_pair_count),
                    (x0 + 0.5, y0 + 0.5),
                    ("sans-serif", 15).into_font().color(&BLACK),
                )))
                .expect("failed to draw regime cell label");
        }
    }

    root.present().expect("failed to present regime grid");
}

fn render_markdown(bundle: &ReportBundle) -> String {
    let mut markdown = String::new();
    markdown.push_str("# Chaos-To-Threshold Translation Report\n\n");
    markdown.push_str("_Generated from `examples/chaos_threshold_translation_report.rs`._\n\n");
    markdown.push_str(
        "This report translates the repo's older chaos/stability language into a maintained arithmetic threshold statement. It does **not** treat `src/chaos/*` as evidence. The maintained source of truth is the bounded-`k` transition lane.\n\n",
    );

    markdown.push_str("## Summary\n\n");
    markdown.push_str(&format!(
        "- Main takeaway: {}\n",
        bundle.report_summary.main_takeaway
    ));
    markdown.push_str(
        "- Density guardrail: prime-rate changes are shown so raw anomaly mass, admissible-set change, and shared-prime-yield change do not get conflated. This report does **not** claim a density theorem.\n",
    );
    markdown.push('\n');

    markdown.push_str("## Transition Layer\n\n");
    markdown.push_str("| Base | M | Regime | Active pairs | Active share | k=(0,0) noninferior | Anomaly mass | Leading pair | Leading source |\n");
    markdown.push_str("|---:|---:|---|---:|---:|---:|---:|---|---|\n");
    for row in &bundle.base_m_rows {
        markdown.push_str(&format!(
            "| {} | {} | `{}` | {} | {:.2}% | {:.2}% | {:.2}pp | `{}` | `{}` |\n",
            row.base,
            row.middle_length,
            row.regime_label,
            row.active_pair_count,
            row.active_pair_share * 100.0,
            row.k00_noninferior_share * 100.0,
            row.anomaly_mass_pp,
            row.leading_pair,
            row.leading_signal_source_label,
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Decomposition Layer\n\n");
    markdown.push_str("| Base | M | Mean anomaly | Mean admissible delta | Mean stable-zero delta | Mean boundary delta | Mean shared rate delta | Dominant source |\n");
    markdown.push_str("|---:|---:|---:|---:|---:|---:|---:|---|\n");
    for row in &bundle.base_m_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | `{}` |\n",
            row.base,
            row.middle_length,
            format_option_pp(row.mean_anomaly_mass_pp_given_active),
            format_option_pp(row.mean_admissible_delta_pp_given_active),
            format_option_pp(row.mean_stable_zero_prime_delta_pp_given_active),
            format_option_pp(row.mean_boundary_prime_delta_pp_given_active),
            format_option_pp(row.mean_shared_prime_rate_delta_pp_given_active),
            row.dominant_signal_source_label,
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Metaphor Translation Layer\n\n");
    markdown.push_str("| Phrase | Status | Arithmetic meaning | Maintained verdict |\n");
    markdown.push_str("|---|---|---|---|\n");
    for row in &bundle.metaphor_translation_rows {
        markdown.push_str(&format!(
            "| {} | `{}` | {} | {} |\n",
            row.phrase, row.status, row.arithmetic_meaning, row.maintained_verdict
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Images\n\n");
    for image in &bundle.image_artifact_rows {
        markdown.push_str(&format!("![{}]({})\n\n", image.label, image.path));
    }

    markdown.push_str("## Observations\n\n");
    for observation in &bundle.observations {
        markdown.push_str(&format!("- {}\n", observation));
    }

    markdown
}

fn print_summary(bundle: &ReportBundle) {
    println!("Chaos threshold translation report");
    println!("  Output: {}", bundle.settings.out_dir);
    println!(
        "  Pair rows: {} | active rows: {} | stable-from-length: {}",
        bundle.report_summary.total_pair_rows,
        bundle.report_summary.active_pair_rows,
        bundle
            .report_summary
            .first_stable_length_all_bases
            .map(|value| format!("M{value}"))
            .unwrap_or_else(|| "none".to_string())
    );
    for row in &bundle.base_m_rows {
        println!(
            "  - base {:>2}, M={}: {} | active {}/{} | mass {:.2}pp | source `{}`",
            row.base,
            row.middle_length,
            row.regime_label,
            row.active_pair_count,
            row.ordered_pair_count,
            row.anomaly_mass_pp,
            row.dominant_signal_source_label,
        );
    }
}

fn mean_option(values: impl Iterator<Item = f64>) -> Option<f64> {
    let values = values.collect::<Vec<_>>();
    if values.is_empty() {
        None
    } else {
        Some(values.iter().sum::<f64>() / values.len() as f64)
    }
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn regime_label(
    active_pair_count: usize,
    ordered_pair_count: usize,
    k00_noninferior_count: usize,
) -> &'static str {
    if active_pair_count == 0 && k00_noninferior_count == ordered_pair_count {
        REGIME_STABLE
    } else if ratio(active_pair_count, ordered_pair_count) >= 0.20 {
        REGIME_ANOMALY_RICH
    } else {
        REGIME_BOUNDARY
    }
}

fn dominant_label<'a>(labels: impl Iterator<Item = &'a str>, fallback: &'static str) -> String {
    let mut counts = BTreeMap::<&'a str, usize>::new();
    for label in labels {
        *counts.entry(label).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .max_by(|left, right| left.1.cmp(&right.1).then_with(|| right.0.cmp(left.0)))
        .map(|(label, _)| label.to_string())
        .unwrap_or_else(|| fallback.to_string())
}

fn format_option_pp(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.2}pp"))
        .unwrap_or_else(|| "n/a".to_string())
}

fn regime_color(label: &str) -> RGBColor {
    match label {
        REGIME_STABLE => RGBColor(80, 146, 88),
        REGIME_BOUNDARY => RGBColor(224, 164, 76),
        REGIME_ANOMALY_RICH => RGBColor(194, 88, 70),
        _ => RGBColor(150, 150, 150),
    }
}

fn base_color(index: usize) -> RGBColor {
    const COLORS: [RGBColor; 8] = [
        RGBColor(48, 119, 142),
        RGBColor(218, 143, 53),
        RGBColor(95, 115, 140),
        RGBColor(181, 76, 64),
        RGBColor(102, 163, 86),
        RGBColor(126, 87, 194),
        RGBColor(141, 110, 99),
        RGBColor(76, 148, 167),
    ];
    COLORS[index % COLORS.len()]
}
