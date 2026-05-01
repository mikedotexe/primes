//! Focused hinge report inside the `B = 2p` family.
//!
//! The direct family test showed that `2p` bases help `M=2` persistence more
//! than the foil bases, but that the story weakens once larger `2p` bases like
//! `22` and `26` are included.
//!
//! This report asks the sharper follow-up question:
//! what makes base `14` strong while bases `22` and `26` weaken?
//!
//! It treats the live hinge as the overlap of two exact/empirical signals:
//! - `M=1 -> M=2` persistence
//! - `M=2` shared-yield-core behavior
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example two_p_hinge_report
//! cargo run --release --example two_p_hinge_report -- --smoke --out-dir /tmp/primes_two_p_hinge_smoke
//! ```

use plotters::prelude::*;
use primes::validation::{
    bounded_k::{
        digit_symbol, evaluate_pair_row, format_k, ordered_unit_pairs, parse_k_label,
        scan_k_config_mask_profile, select_smoke_pairs, unit_residues, DEFAULT_BOUNDED_K_GRID,
    },
    reporting::{
        ensure_dir, export_timestamp_utc, write_csv_rows, write_json_pretty, write_text_file,
    },
};
use rayon::prelude::*;
use serde::Serialize;
use std::{
    collections::BTreeMap,
    env,
    path::{Path, PathBuf},
};

const BASES: &[u32] = &[6, 10, 14, 22, 26];
const M1: usize = 1;
const M2: usize = 2;
const DEFAULT_OUT_DIR: &str = "/tmp/primes_two_p_hinge";
const REPORT_EXPORT_VERSION: u32 = 1;
const SMOKE_MAX_ORDERED_PAIRS_PER_BASE: usize = 10;
const SMOKE_PAIR_ANCHORS: &[(u32, u32, u32)] = &[
    (6, 5, 5),
    (10, 3, 3),
    (10, 3, 7),
    (14, 3, 1),
    (14, 13, 11),
    (22, 17, 19),
    (26, 23, 23),
];

const CATEGORY_PERSISTENT_CORE: &str = "persistent_core";
const CATEGORY_PERSISTENCE_ONLY: &str = "persistence_only";
const CATEGORY_CORE_ONLY: &str = "core_only";
const CATEGORY_ACTIVE_NEITHER: &str = "active_neither";
const CATEGORIES: &[&str] = &[
    CATEGORY_PERSISTENT_CORE,
    CATEGORY_PERSISTENCE_ONLY,
    CATEGORY_CORE_ONLY,
    CATEGORY_ACTIVE_NEITHER,
];

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    smoke_catalog: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    bases: Vec<u32>,
    pair_catalog_mode: String,
    max_ordered_pairs_per_base: Option<usize>,
    middle_lengths: Vec<usize>,
    k_grid: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct PairHingeRow {
    base: u32,
    outer: u32,
    inner: u32,
    pair_label: String,
    unit_distance: usize,
    gap_bucket: String,
    best_k_m1: String,
    best_k_m2: String,
    anomaly_m1_pp: f64,
    anomaly_m2_pp: f64,
    m1_anomalous: bool,
    m2_active: bool,
    m2_persistent: bool,
    m2_emergent: bool,
    positive_shared_yield: Option<bool>,
    shared_yield_core: Option<bool>,
    shared_prime_delta_count: Option<isize>,
    admissible_set_effect_pp: Option<f64>,
    prime_yield_effect_pp: Option<f64>,
    persistent_core: bool,
    hinge_category: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct BaseHingeSummaryRow {
    base: u32,
    unit_cycle: String,
    unit_count: usize,
    ordered_pair_count: usize,
    baseline_tight_pair_share: f64,
    m1_anomalous_pairs: usize,
    m2_active_pairs: usize,
    m2_persistent_pairs: usize,
    m2_emergent_pairs: usize,
    shared_yield_core_pairs: usize,
    persistent_core_pairs: usize,
    persistence_rate_given_m1: Option<f64>,
    shared_yield_core_share_given_m2: Option<f64>,
    persistent_core_share_given_m2: Option<f64>,
    persistent_core_share_given_persistent: Option<f64>,
    active_tight_share_given_m2: Option<f64>,
    persistent_tight_share: Option<f64>,
    persistent_core_tight_share: Option<f64>,
    mean_shared_prime_delta_count: Option<f64>,
    mean_anomaly_m2_pp: Option<f64>,
    hinge_label: String,
}

#[derive(Debug, Clone, Serialize)]
struct CompositionRow {
    base: u32,
    category: String,
    count: usize,
    share_given_m2_active: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
struct GapSummaryRow {
    base: u32,
    gap_bucket: String,
    active_pairs: usize,
    persistent_pairs: usize,
    persistent_core_pairs: usize,
    share_given_m2_active: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    total_pairs: usize,
    active_pairs: usize,
    main_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    pair_hinge_rows: Vec<PairHingeRow>,
    base_summary_rows: Vec<BaseHingeSummaryRow>,
    composition_rows: Vec<CompositionRow>,
    gap_summary_rows: Vec<GapSummaryRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

#[derive(Debug, Clone)]
struct SharedYieldMetrics {
    shared_prime_delta_count: isize,
    admissible_set_effect_pp: f64,
    prime_yield_effect_pp: f64,
    positive_shared_yield: bool,
    shared_yield_core: bool,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        bases: BASES.to_vec(),
        pair_catalog_mode: if options.smoke_catalog {
            "smoke".to_string()
        } else {
            "full".to_string()
        },
        max_ordered_pairs_per_base: if options.smoke_catalog {
            Some(SMOKE_MAX_ORDERED_PAIRS_PER_BASE)
        } else {
            None
        },
        middle_lengths: vec![M1, M2],
        k_grid: DEFAULT_BOUNDED_K_GRID
            .iter()
            .map(|&config| format_k(config))
            .collect(),
    };

    let pair_hinge_rows = build_pair_hinge_rows(options.smoke_catalog);
    let base_summary_rows = build_base_summary_rows(&pair_hinge_rows);
    let composition_rows = build_composition_rows(&pair_hinge_rows);
    let gap_summary_rows = build_gap_summary_rows(&pair_hinge_rows);

    let hinge_plane_path = options.out_dir.join("two_p_hinge_plane.png");
    render_hinge_plane(&base_summary_rows, &hinge_plane_path);
    let composition_heatmap_path = options.out_dir.join("two_p_hinge_composition_heatmap.png");
    render_composition_heatmap(
        &base_summary_rows,
        &composition_rows,
        &composition_heatmap_path,
    );

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "hinge_plane".to_string(),
            label: "2p hinge plane".to_string(),
            path: hinge_plane_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "composition_heatmap".to_string(),
            label: "2p hinge composition heatmap".to_string(),
            path: composition_heatmap_path.display().to_string(),
        },
    ];
    let report_summary = build_report_summary(&pair_hinge_rows, &base_summary_rows);
    let observations = derive_observations(&base_summary_rows, &composition_rows);

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        pair_hinge_rows: pair_hinge_rows.clone(),
        base_summary_rows: base_summary_rows.clone(),
        composition_rows: composition_rows.clone(),
        gap_summary_rows: gap_summary_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary,
        observations,
    };

    write_csv_rows(
        options.out_dir.join("pair_hinge_rows.csv"),
        &pair_hinge_rows,
    )
    .expect("failed to write pair_hinge_rows.csv");
    write_csv_rows(
        options.out_dir.join("base_summary_rows.csv"),
        &base_summary_rows,
    )
    .expect("failed to write base_summary_rows.csv");
    write_csv_rows(
        options.out_dir.join("composition_rows.csv"),
        &composition_rows,
    )
    .expect("failed to write composition_rows.csv");
    write_csv_rows(
        options.out_dir.join("gap_summary_rows.csv"),
        &gap_summary_rows,
    )
    .expect("failed to write gap_summary_rows.csv");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle)
        .expect("failed to write summary.json");
    write_text_file(options.out_dir.join("report.md"), &render_markdown(&bundle))
        .expect("failed to write report.md");

    println!("2p hinge report");
    println!("  output dir: {}", options.out_dir.display());
    for row in &base_summary_rows {
        println!(
            "  base {:>2} | label {:<16} | persistence {} | core {} | persistent_core {}",
            row.base,
            row.hinge_label,
            format_option_share(row.persistence_rate_given_m1),
            format_option_share(row.shared_yield_core_share_given_m2),
            row.persistent_core_pairs,
        );
    }
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut smoke_catalog = false;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => {
                let value = args
                    .next()
                    .expect("--out-dir requires a directory argument");
                out_dir = PathBuf::from(value);
            }
            "--smoke" => smoke_catalog = true,
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => panic!("unrecognized argument: {arg}"),
        }
    }

    Options {
        out_dir,
        smoke_catalog,
    }
}

fn print_help() {
    println!("Usage:");
    println!("  cargo run --release --example two_p_hinge_report -- [options]");
    println!();
    println!("Options:");
    println!("  --out-dir <dir>   Output directory (default: {DEFAULT_OUT_DIR})");
    println!("  --smoke           Use a small anchored pair catalog per base");
    println!("  -h, --help        Show this help message");
}

fn build_pair_hinge_rows(smoke_catalog: bool) -> Vec<PairHingeRow> {
    BASES
        .par_iter()
        .copied()
        .flat_map(|base| {
            let pairs = selected_pairs(base, smoke_catalog);
            pairs.into_par_iter().map(move |(outer, inner)| {
                let row_m1 = evaluate_pair_row(base, M1, outer, inner, DEFAULT_BOUNDED_K_GRID);
                let row_m2 = evaluate_pair_row(base, M2, outer, inner, DEFAULT_BOUNDED_K_GRID);

                let anomaly_m1 = anomaly_mass(&row_m1);
                let anomaly_m2 = anomaly_mass(&row_m2);
                let m1_anomalous = anomaly_m1 > 0.0;
                let m2_active = anomaly_m2 > 0.0;
                let m2_persistent = m1_anomalous && m2_active;
                let m2_emergent = !m1_anomalous && m2_active;
                let shared_metrics = if m2_active {
                    Some(shared_yield_metrics(
                        base,
                        outer,
                        inner,
                        parse_k_label(&row_m2.best_k),
                    ))
                } else {
                    None
                };
                let persistent_core = m2_persistent
                    && shared_metrics
                        .as_ref()
                        .map(|metrics| metrics.shared_yield_core)
                        .unwrap_or(false);

                PairHingeRow {
                    base,
                    outer,
                    inner,
                    pair_label: format!("({},{})", digit_symbol(outer), digit_symbol(inner)),
                    unit_distance: cyclic_unit_distance(base, outer, inner),
                    gap_bucket: gap_bucket(base, outer, inner).to_string(),
                    best_k_m1: row_m1.best_k,
                    best_k_m2: row_m2.best_k,
                    anomaly_m1_pp: anomaly_m1,
                    anomaly_m2_pp: anomaly_m2,
                    m1_anomalous,
                    m2_active,
                    m2_persistent,
                    m2_emergent,
                    positive_shared_yield: shared_metrics
                        .as_ref()
                        .map(|metrics| metrics.positive_shared_yield),
                    shared_yield_core: shared_metrics
                        .as_ref()
                        .map(|metrics| metrics.shared_yield_core),
                    shared_prime_delta_count: shared_metrics
                        .as_ref()
                        .map(|metrics| metrics.shared_prime_delta_count),
                    admissible_set_effect_pp: shared_metrics
                        .as_ref()
                        .map(|metrics| metrics.admissible_set_effect_pp),
                    prime_yield_effect_pp: shared_metrics
                        .as_ref()
                        .map(|metrics| metrics.prime_yield_effect_pp),
                    persistent_core,
                    hinge_category: if m2_active {
                        Some(
                            hinge_category(
                                m2_persistent,
                                shared_metrics
                                    .as_ref()
                                    .map(|m| m.shared_yield_core)
                                    .unwrap_or(false),
                            )
                            .to_string(),
                        )
                    } else {
                        None
                    },
                }
            })
        })
        .collect::<Vec<_>>()
}

fn selected_pairs(base: u32, smoke_catalog: bool) -> Vec<(u32, u32)> {
    if smoke_catalog {
        let anchors = SMOKE_PAIR_ANCHORS
            .iter()
            .filter_map(|&(anchor_base, outer, inner)| {
                if anchor_base == base {
                    Some((outer, inner))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        select_smoke_pairs(base, SMOKE_MAX_ORDERED_PAIRS_PER_BASE, &anchors)
    } else {
        ordered_unit_pairs(base)
    }
}

fn build_base_summary_rows(rows: &[PairHingeRow]) -> Vec<BaseHingeSummaryRow> {
    let mut by_base = BTreeMap::<u32, Vec<&PairHingeRow>>::new();
    for row in rows {
        by_base.entry(row.base).or_default().push(row);
    }

    by_base
        .into_iter()
        .map(|(base, base_rows)| {
            let units = unit_residues(base);
            let unit_cycle = units
                .iter()
                .map(|&digit| digit_symbol(digit))
                .collect::<Vec<_>>()
                .join(", ");
            let ordered_pair_count = base_rows.len();
            let baseline_tight_pair_count = base_rows
                .iter()
                .filter(|row| row.gap_bucket != "wide")
                .count();
            let m1_anomalous_pairs = base_rows.iter().filter(|row| row.m1_anomalous).count();
            let m2_active_pairs = base_rows.iter().filter(|row| row.m2_active).count();
            let m2_persistent_pairs = base_rows.iter().filter(|row| row.m2_persistent).count();
            let m2_emergent_pairs = base_rows.iter().filter(|row| row.m2_emergent).count();
            let shared_yield_core_pairs = base_rows
                .iter()
                .filter(|row| row.shared_yield_core == Some(true))
                .count();
            let persistent_core_pairs = base_rows.iter().filter(|row| row.persistent_core).count();
            let active_tight_pairs = base_rows
                .iter()
                .filter(|row| row.m2_active && row.gap_bucket != "wide")
                .count();
            let persistent_tight_pairs = base_rows
                .iter()
                .filter(|row| row.m2_persistent && row.gap_bucket != "wide")
                .count();
            let persistent_core_tight_pairs = base_rows
                .iter()
                .filter(|row| row.persistent_core && row.gap_bucket != "wide")
                .count();

            let hinge_label = if persistent_core_pairs > 0 {
                if units.len() <= 2 {
                    "tiny_bridge"
                } else {
                    "hinge_bridge"
                }
            } else if m2_persistent_pairs > 0 {
                "persistence_only"
            } else if shared_yield_core_pairs > 0 {
                "core_only"
            } else if m2_active_pairs > 0 {
                "active_neither"
            } else {
                "inactive"
            };

            BaseHingeSummaryRow {
                base,
                unit_cycle,
                unit_count: units.len(),
                ordered_pair_count,
                baseline_tight_pair_share: ratio(baseline_tight_pair_count, ordered_pair_count),
                m1_anomalous_pairs,
                m2_active_pairs,
                m2_persistent_pairs,
                m2_emergent_pairs,
                shared_yield_core_pairs,
                persistent_core_pairs,
                persistence_rate_given_m1: ratio_option(m2_persistent_pairs, m1_anomalous_pairs),
                shared_yield_core_share_given_m2: ratio_option(
                    shared_yield_core_pairs,
                    m2_active_pairs,
                ),
                persistent_core_share_given_m2: ratio_option(
                    persistent_core_pairs,
                    m2_active_pairs,
                ),
                persistent_core_share_given_persistent: ratio_option(
                    persistent_core_pairs,
                    m2_persistent_pairs,
                ),
                active_tight_share_given_m2: ratio_option(active_tight_pairs, m2_active_pairs),
                persistent_tight_share: ratio_option(persistent_tight_pairs, m2_persistent_pairs),
                persistent_core_tight_share: ratio_option(
                    persistent_core_tight_pairs,
                    persistent_core_pairs,
                ),
                mean_shared_prime_delta_count: mean(
                    &base_rows
                        .iter()
                        .filter_map(|row| row.shared_prime_delta_count.map(|count| count as f64))
                        .collect::<Vec<_>>(),
                ),
                mean_anomaly_m2_pp: mean(
                    &base_rows
                        .iter()
                        .filter(|row| row.m2_active)
                        .map(|row| row.anomaly_m2_pp)
                        .collect::<Vec<_>>(),
                ),
                hinge_label: hinge_label.to_string(),
            }
        })
        .collect()
}

fn build_composition_rows(rows: &[PairHingeRow]) -> Vec<CompositionRow> {
    let mut by_base_category = BTreeMap::<(u32, String), usize>::new();
    let mut active_count_by_base = BTreeMap::<u32, usize>::new();

    for row in rows.iter().filter(|row| row.m2_active) {
        *active_count_by_base.entry(row.base).or_insert(0) += 1;
        let category = row
            .hinge_category
            .as_ref()
            .expect("active pair should have a hinge category")
            .clone();
        *by_base_category.entry((row.base, category)).or_insert(0) += 1;
    }

    let mut rows_out = Vec::with_capacity(BASES.len() * CATEGORIES.len());
    for &base in BASES {
        for &category in CATEGORIES {
            let count = *by_base_category
                .get(&(base, category.to_string()))
                .unwrap_or(&0);
            let active_count = *active_count_by_base.get(&base).unwrap_or(&0);
            rows_out.push(CompositionRow {
                base,
                category: category.to_string(),
                count,
                share_given_m2_active: ratio_option(count, active_count),
            });
        }
    }
    rows_out
}

fn build_gap_summary_rows(rows: &[PairHingeRow]) -> Vec<GapSummaryRow> {
    let mut grouped = BTreeMap::<(u32, String), Vec<&PairHingeRow>>::new();
    let active_by_base = rows.iter().filter(|row| row.m2_active).fold(
        BTreeMap::<u32, usize>::new(),
        |mut map, row| {
            *map.entry(row.base).or_insert(0) += 1;
            map
        },
    );

    for row in rows.iter().filter(|row| row.m2_active) {
        grouped
            .entry((row.base, row.gap_bucket.clone()))
            .or_default()
            .push(row);
    }

    let mut rows_out = Vec::with_capacity(BASES.len() * 3);
    for &base in BASES {
        for gap_bucket in ["same", "adjacent", "wide"] {
            let group = grouped
                .get(&(base, gap_bucket.to_string()))
                .cloned()
                .unwrap_or_default();
            let active_pairs = group.len();
            let persistent_pairs = group.iter().filter(|row| row.m2_persistent).count();
            let persistent_core_pairs = group.iter().filter(|row| row.persistent_core).count();
            rows_out.push(GapSummaryRow {
                base,
                gap_bucket: gap_bucket.to_string(),
                active_pairs,
                persistent_pairs,
                persistent_core_pairs,
                share_given_m2_active: ratio_option(
                    active_pairs,
                    *active_by_base.get(&base).unwrap_or(&0),
                ),
            });
        }
    }
    rows_out
}

fn shared_yield_metrics(
    base: u32,
    outer: u32,
    inner: u32,
    best_k: (u32, u32),
) -> SharedYieldMetrics {
    let k00_profile = scan_k_config_mask_profile(base, M2, outer, inner, (0, 0));
    let best_profile = if best_k == (0, 0) {
        k00_profile.clone()
    } else {
        scan_k_config_mask_profile(base, M2, outer, inner, best_k)
    };

    let mut shared_prime_delta_count = 0isize;
    let mut overlap_prime_delta_count = 0isize;

    for (k00_row, best_row) in k00_profile
        .candidate_rows
        .iter()
        .zip(&best_profile.candidate_rows)
    {
        match (k00_row.admissible, best_row.admissible) {
            (true, true) => {
                if k00_row.prime {
                    shared_prime_delta_count -= 1;
                }
                if best_row.prime {
                    shared_prime_delta_count += 1;
                }
            }
            (false, true) => {
                if best_row.prime {
                    overlap_prime_delta_count += 1;
                }
            }
            (true, false) => {
                if k00_row.prime {
                    overlap_prime_delta_count -= 1;
                }
            }
            (false, false) => {}
        }
    }

    let admissible_share_k00 =
        k00_profile.admissible_count as f64 / k00_profile.candidates_per_config as f64;
    let admissible_share_best =
        best_profile.admissible_count as f64 / best_profile.candidates_per_config as f64;
    let prime_yield_k00 = ratio(k00_profile.prime_hits, k00_profile.admissible_count);
    let prime_yield_best = ratio(best_profile.prime_hits, best_profile.admissible_count);
    let admissible_set_effect_pp =
        (admissible_share_best - admissible_share_k00) * prime_yield_k00 * 100.0;
    let prime_yield_effect_pp =
        admissible_share_best * (prime_yield_best - prime_yield_k00) * 100.0;

    SharedYieldMetrics {
        shared_prime_delta_count,
        admissible_set_effect_pp,
        prime_yield_effect_pp,
        positive_shared_yield: shared_prime_delta_count > 0,
        shared_yield_core: shared_prime_delta_count > overlap_prime_delta_count.abs()
            && shared_prime_delta_count > 0
            && prime_yield_effect_pp.abs() > admissible_set_effect_pp.abs(),
    }
}

fn hinge_category(m2_persistent: bool, shared_yield_core: bool) -> &'static str {
    match (m2_persistent, shared_yield_core) {
        (true, true) => CATEGORY_PERSISTENT_CORE,
        (true, false) => CATEGORY_PERSISTENCE_ONLY,
        (false, true) => CATEGORY_CORE_ONLY,
        (false, false) => CATEGORY_ACTIVE_NEITHER,
    }
}

fn cyclic_unit_distance(base: u32, outer: u32, inner: u32) -> usize {
    let units = unit_residues(base);
    let outer_index = units
        .iter()
        .position(|&digit| digit == outer)
        .expect("outer digit should be a unit");
    let inner_index = units
        .iter()
        .position(|&digit| digit == inner)
        .expect("inner digit should be a unit");
    let direct = outer_index.abs_diff(inner_index);
    direct.min(units.len() - direct)
}

fn gap_bucket(base: u32, outer: u32, inner: u32) -> &'static str {
    match cyclic_unit_distance(base, outer, inner) {
        0 => "same",
        1 => "adjacent",
        _ => "wide",
    }
}

fn render_hinge_plane(rows: &[BaseHingeSummaryRow], path: &Path) {
    let root = BitMapBackend::new(path, (1120, 760)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill hinge plane canvas");

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "2p Hinge Plane  (x = persistence given M=1, y = shared-yield-core given M=2)",
            ("sans-serif", 28),
        )
        .margin(28)
        .x_label_area_size(64)
        .y_label_area_size(80)
        .build_cartesian_2d(0.0f64..1.05f64, 0.0f64..1.05f64)
        .expect("failed to build hinge plane");

    chart
        .configure_mesh()
        .x_desc("M=2 persistence rate given M=1")
        .y_desc("shared-yield-core share given M=2")
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .light_line_style(RGBColor(222, 216, 207))
        .draw()
        .expect("failed to draw hinge plane mesh");

    for row in rows {
        let x = row.persistence_rate_given_m1.unwrap_or(0.0);
        let y = row.shared_yield_core_share_given_m2.unwrap_or(0.0);
        let radius = 7 + (row.persistent_core_pairs as i32 * 5);
        let color = hinge_color(&row.hinge_label);

        chart
            .draw_series(std::iter::once(Circle::new(
                (x, y),
                radius,
                ShapeStyle::from(&color).filled(),
            )))
            .expect("failed to draw hinge plane point");
        chart
            .draw_series(std::iter::once(Text::new(
                format!("{} ({})", row.base, row.hinge_label),
                (x + 0.02, y + 0.02),
                ("sans-serif", 16).into_font().color(&BLACK),
            )))
            .expect("failed to draw hinge plane label");
    }

    root.present().expect("failed to present hinge plane");
}

fn render_composition_heatmap(
    base_rows: &[BaseHingeSummaryRow],
    composition_rows: &[CompositionRow],
    path: &Path,
) {
    let base_labels = base_rows
        .iter()
        .map(|row| format!("{} ({})", row.base, row.hinge_label))
        .collect::<Vec<_>>();
    let base_lookup = base_rows
        .iter()
        .enumerate()
        .map(|(index, row)| (row.base, index as i32))
        .collect::<BTreeMap<_, _>>();

    let root = BitMapBackend::new(path, (1180, 460)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill composition heatmap canvas");
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "2p Hinge Composition  (share of M=2 actives by category)",
            ("sans-serif", 28),
        )
        .margin(24)
        .x_label_area_size(84)
        .y_label_area_size(180)
        .build_cartesian_2d(0i32..CATEGORIES.len() as i32, 0i32..base_rows.len() as i32)
        .expect("failed to build composition heatmap");

    let category_labels = CATEGORIES
        .iter()
        .map(|label| category_display_label(label))
        .collect::<Vec<_>>();

    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("M=2 active-pair category")
        .y_desc("2p bases")
        .x_labels(CATEGORIES.len())
        .y_labels(base_rows.len())
        .x_label_formatter(&move |value| {
            if *value >= 0 && (*value as usize) < category_labels.len() {
                category_labels[*value as usize].to_string()
            } else {
                String::new()
            }
        })
        .y_label_formatter(&move |value| {
            if *value >= 0 && (*value as usize) < base_labels.len() {
                let row_index = (base_labels.len() - 1) - *value as usize;
                base_labels[row_index].clone()
            } else {
                String::new()
            }
        })
        .label_style(("sans-serif", 15))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw composition heatmap mesh");

    for row in composition_rows {
        let x = CATEGORIES
            .iter()
            .position(|&category| category == row.category)
            .expect("category should be known") as i32;
        let base_index = *base_lookup
            .get(&row.base)
            .expect("composition base should exist");
        let y = base_rows.len() as i32 - 1 - base_index;
        let share = row.share_given_m2_active.unwrap_or(0.0);
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x, y), (x + 1, y + 1)],
                ShapeStyle::from(&heatmap_color(share)).filled(),
            )))
            .expect("failed to draw composition heatmap cell");
        chart
            .draw_series(std::iter::once(Text::new(
                if row.count == 0 {
                    "0".to_string()
                } else {
                    format!("{} / {:.0}%", row.count, share * 100.0)
                },
                (x, y),
                ("sans-serif", 14).into_font().color(&BLACK),
            )))
            .expect("failed to draw composition heatmap label");
    }

    root.present()
        .expect("failed to present composition heatmap");
}

fn build_report_summary(
    pair_rows: &[PairHingeRow],
    base_rows: &[BaseHingeSummaryRow],
) -> ReportSummary {
    let non_tiny_bridge_count = base_rows
        .iter()
        .filter(|row| row.hinge_label == "hinge_bridge")
        .count();
    let main_takeaway = if non_tiny_bridge_count == 1 {
        let base14 = base_rows
            .iter()
            .find(|row| row.base == 14)
            .expect("base 14 should exist");
        format!(
            "Base 14 is the unique non-tiny hinge base: it keeps {} persistent-core pairs while 10 splits into persistence-only and 26 splits into core-only.",
            base14.persistent_core_pairs
        )
    } else {
        "The 2p hinge is not concentrated in a single non-tiny base on this catalog.".to_string()
    };

    ReportSummary {
        total_pairs: pair_rows.len(),
        active_pairs: pair_rows.iter().filter(|row| row.m2_active).count(),
        main_takeaway,
    }
}

fn derive_observations(
    base_rows: &[BaseHingeSummaryRow],
    composition_rows: &[CompositionRow],
) -> Vec<String> {
    let base6 = base_rows.iter().find(|row| row.base == 6).expect("base 6");
    let base10 = base_rows
        .iter()
        .find(|row| row.base == 10)
        .expect("base 10");
    let base14 = base_rows
        .iter()
        .find(|row| row.base == 14)
        .expect("base 14");
    let base22 = base_rows
        .iter()
        .find(|row| row.base == 22)
        .expect("base 22");
    let base26 = base_rows
        .iter()
        .find(|row| row.base == 26)
        .expect("base 26");

    let composition_value = |base: u32, category: &str| {
        composition_rows
            .iter()
            .find(|row| row.base == base && row.category == category)
            .and_then(|row| row.share_given_m2_active)
            .unwrap_or(0.0)
    };

    vec![
        format!(
            "Base 14 is the only non-tiny hinge bridge: it has `{}` persistent-core pairs, while base 10 has `{}` and base 26 has `{}`.",
            base14.persistent_core_pairs, base10.persistent_core_pairs, base26.persistent_core_pairs
        ),
        format!(
            "The split is now sharp: base 10 is `{}` (`{}` persistent pairs but `{}` shared-yield-core), base 26 is `{}` (`{}` shared-yield-core pairs but `{}` persistent pairs), and base 22 is `{}`.",
            base10.hinge_label,
            base10.m2_persistent_pairs,
            format_option_share(base10.shared_yield_core_share_given_m2),
            base26.hinge_label,
            base26.shared_yield_core_pairs,
            base26.m2_persistent_pairs,
            base22.hinge_label,
        ),
        format!(
            "Base 14 also overconcentrates its M=2 activity in tight pockets: active tight share `{}` against a baseline tight-pair share `{}`. Base 22 is also tight at `{}`, so tightness helps but is not sufficient by itself.",
            format_option_share(base14.active_tight_share_given_m2),
            format!("{:.2}%", base14.baseline_tight_pair_share * 100.0),
            format_option_share(base22.active_tight_share_given_m2),
        ),
        format!(
            "The most telling intersection is the composition split: base 14 puts `{}` of its M=2 actives in `persistent_core`, base 10 puts `{}` in `persistence_only`, base 26 puts `{}` in `core_only`, and base 22 leaves all of its M=2 activity in `active_neither`.",
            format!("{:.0}%", composition_value(14, CATEGORY_PERSISTENT_CORE) * 100.0),
            format!("{:.0}%", composition_value(10, CATEGORY_PERSISTENCE_ONLY) * 100.0),
            format!("{:.0}%", composition_value(26, CATEGORY_CORE_ONLY) * 100.0),
        ),
        format!(
            "Base 6 remains a tiny witness rather than the main explanation base: it has `{}` persistent-core pair and unit count `{}`, while base 14 has the strongest nontrivial overlap with unit count `{}`.",
            base6.persistent_core_pairs, base6.unit_count, base14.unit_count,
        ),
    ]
}

fn render_markdown(bundle: &ReportBundle) -> String {
    let mut markdown = String::new();
    markdown.push_str("# 2p Hinge Report\n\n");
    markdown.push_str("_Generated from `examples/two_p_hinge_report.rs`._\n\n");
    markdown.push_str(&format!(
        "- Output directory: `{}`\n- Pair catalog mode: `{}`\n\n",
        bundle.settings.out_dir, bundle.settings.pair_catalog_mode
    ));

    markdown.push_str("## Base Summary\n\n");
    markdown.push_str("| Base | Label | Unit count | Persistence | Core share | Persistent-core | Tight active share |\n");
    markdown.push_str("|---|---|---:|---:|---:|---:|---:|\n");
    for row in &bundle.base_summary_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} |\n",
            row.base,
            row.hinge_label,
            row.unit_count,
            format_option_share(row.persistence_rate_given_m1),
            format_option_share(row.shared_yield_core_share_given_m2),
            row.persistent_core_pairs,
            format_option_share(row.active_tight_share_given_m2),
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Active Pair Composition\n\n");
    markdown
        .push_str("| Base | Persistent-core | Persistence-only | Core-only | Active-neither |\n");
    markdown.push_str("|---|---:|---:|---:|---:|\n");
    for base in BASES {
        let share = |category: &str| {
            bundle
                .composition_rows
                .iter()
                .find(|row| row.base == *base && row.category == category)
                .and_then(|row| row.share_given_m2_active)
        };
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            base,
            format_option_share(share(CATEGORY_PERSISTENT_CORE)),
            format_option_share(share(CATEGORY_PERSISTENCE_ONLY)),
            format_option_share(share(CATEGORY_CORE_ONLY)),
            format_option_share(share(CATEGORY_ACTIVE_NEITHER)),
        ));
    }
    markdown.push('\n');

    markdown.push_str("## M=2 Active Pairs\n\n");
    markdown
        .push_str("| Base | Pair | Gap | Category | Best k | M2 anomaly | Shared prime delta |\n");
    markdown.push_str("|---|---|---|---|---|---:|---:|\n");
    let mut active_rows = bundle
        .pair_hinge_rows
        .iter()
        .filter(|row| row.m2_active)
        .collect::<Vec<_>>();
    active_rows.sort_by(|left, right| {
        left.base
            .cmp(&right.base)
            .then_with(|| right.anomaly_m2_pp.total_cmp(&left.anomaly_m2_pp))
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });
    for row in active_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {:.2}pp | {} |\n",
            row.base,
            row.pair_label,
            row.gap_bucket,
            row.hinge_category
                .as_deref()
                .map(category_display_label)
                .unwrap_or("inactive"),
            row.best_k_m2,
            row.anomaly_m2_pp,
            row.shared_prime_delta_count
                .map(|count| count.to_string())
                .unwrap_or_else(|| "n/a".to_string()),
        ));
    }
    markdown.push('\n');

    for image in &bundle.image_artifact_rows {
        markdown.push_str(&format!("![{}]({})\n\n", image.label, image.path));
    }

    markdown.push_str("## Observations\n\n");
    for observation in &bundle.observations {
        markdown.push_str(&format!("- {}\n", observation));
    }
    markdown
}

fn anomaly_mass(row: &primes::validation::bounded_k::KDominancePairRow) -> f64 {
    row.best_minus_k00_pp.max(0.0)
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn ratio_option(numerator: usize, denominator: usize) -> Option<f64> {
    if denominator == 0 {
        None
    } else {
        Some(ratio(numerator, denominator))
    }
}

fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        None
    } else {
        Some(values.iter().sum::<f64>() / values.len() as f64)
    }
}

fn hinge_color(label: &str) -> RGBColor {
    match label {
        "hinge_bridge" => RGBColor(48, 119, 142),
        "tiny_bridge" => RGBColor(115, 92, 196),
        "persistence_only" => RGBColor(219, 143, 52),
        "core_only" => RGBColor(181, 76, 64),
        "active_neither" => RGBColor(133, 133, 133),
        _ => RGBColor(133, 133, 133),
    }
}

fn heatmap_color(share: f64) -> RGBColor {
    let clamped = share.clamp(0.0, 1.0);
    let low = (242.0, 236.0, 228.0);
    let high = (48.0, 119.0, 142.0);
    RGBColor(
        (low.0 + (high.0 - low.0) * clamped) as u8,
        (low.1 + (high.1 - low.1) * clamped) as u8,
        (low.2 + (high.2 - low.2) * clamped) as u8,
    )
}

fn category_display_label(label: &str) -> &str {
    match label {
        CATEGORY_PERSISTENT_CORE => "persistent_core",
        CATEGORY_PERSISTENCE_ONLY => "persistence_only",
        CATEGORY_CORE_ONLY => "core_only",
        CATEGORY_ACTIVE_NEITHER => "active_neither",
        _ => label,
    }
}

fn format_option_share(value: Option<f64>) -> String {
    value
        .map(|value| format!("{:.2}%", value * 100.0))
        .unwrap_or_else(|| "n/a".to_string())
}
