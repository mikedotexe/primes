//! Direct `B = 2p` family test against nearby foil bases.
//!
//! This report asks a deliberately narrow question:
//! are `M=2` boundary persistence and shared-admissible yield structure
//! elevated in the `2p` bases compared with foils?
//!
//! Tested bases:
//! - `2p` family: `6, 10, 14, 22, 26`
//! - foil family: `12, 18, 30`
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example two_p_family_report
//! cargo run --release --example two_p_family_report -- --smoke --out-dir /tmp/primes_two_p_family_smoke
//! ```

use plotters::prelude::*;
use primes::validation::{
    bounded_k::{
        digit_symbol, evaluate_pair_row, format_k, ordered_unit_pairs, parse_k_label,
        scan_k_config_mask_profile, select_smoke_pairs, KDominancePairRow, DEFAULT_BOUNDED_K_GRID,
    },
    reporting::{
        ensure_dir, export_timestamp_utc, write_csv_rows, write_json_pretty, write_text_file,
    },
};
use rayon::prelude::*;
use serde::Serialize;
use std::{collections::BTreeMap, env, path::PathBuf};

const TWO_P_BASES: &[u32] = &[6, 10, 14, 22, 26];
const FOIL_BASES: &[u32] = &[12, 18, 30];
const BASES: &[u32] = &[6, 10, 12, 14, 18, 22, 26, 30];
const M1: usize = 1;
const M2: usize = 2;
const M3: usize = 3;
const DEFAULT_OUT_DIR: &str = "/tmp/primes_two_p_family";
const REPORT_EXPORT_VERSION: u32 = 1;
const SMOKE_MAX_ORDERED_PAIRS_PER_BASE: usize = 10;
const SMOKE_PAIR_ANCHORS: &[(u32, u32, u32)] =
    &[(6, 1, 5), (10, 3, 3), (10, 3, 7), (14, 13, 11), (30, 11, 7)];

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    smoke_catalog: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    bases: Vec<u32>,
    two_p_bases: Vec<u32>,
    foil_bases: Vec<u32>,
    pair_catalog_mode: String,
    max_ordered_pairs_per_base: Option<usize>,
    middle_lengths: Vec<usize>,
    k_grid: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct PairAuditRow {
    family: String,
    base: u32,
    outer: u32,
    inner: u32,
    pair_label: String,
    best_k_m1: String,
    best_k_m2: String,
    best_k_m3: String,
    anomaly_m1_pp: f64,
    anomaly_m2_pp: f64,
    anomaly_m3_pp: f64,
    m1_anomalous: bool,
    m2_active: bool,
    m2_persistent: bool,
    m2_emergent: bool,
    boundary_class: String,
    shared_admissible_count: Option<usize>,
    shared_prime_delta_count: Option<isize>,
    overlap_prime_delta_count: Option<isize>,
    shared_prime_rate_k00_pp: Option<f64>,
    shared_prime_rate_best_pp: Option<f64>,
    shared_prime_rate_delta_pp: Option<f64>,
    admissible_set_effect_pp: Option<f64>,
    prime_yield_effect_pp: Option<f64>,
    positive_shared_yield: Option<bool>,
    shared_yield_core: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
struct BaseSummaryRow {
    family: String,
    base: u32,
    ordered_pair_count: usize,
    m1_anomalous_pairs: usize,
    m2_active_pairs: usize,
    m2_persistent_pairs: usize,
    m2_emergent_pairs: usize,
    m3_active_pairs: usize,
    m1_anomaly_mass_pp: f64,
    m2_anomaly_mass_pp: f64,
    m3_anomaly_mass_pp: f64,
    persistence_rate_given_m1: Option<f64>,
    positive_shared_yield_pairs: usize,
    shared_yield_core_pairs: usize,
    positive_shared_yield_share_given_m2: Option<f64>,
    shared_yield_core_share_given_m2: Option<f64>,
    mean_shared_prime_delta_count: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
struct FamilySummaryRow {
    family: String,
    base_count: usize,
    bases: String,
    ordered_pair_count: usize,
    m1_anomalous_pairs: usize,
    m2_active_pairs: usize,
    m2_persistent_pairs: usize,
    m2_emergent_pairs: usize,
    m3_active_pairs: usize,
    weighted_persistence_rate_given_m1: Option<f64>,
    weighted_positive_shared_yield_share_given_m2: Option<f64>,
    weighted_shared_yield_core_share_given_m2: Option<f64>,
    mean_base_persistence_rate_given_m1: Option<f64>,
    mean_base_shared_yield_core_share_given_m2: Option<f64>,
    mean_shared_prime_delta_count: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
struct HypothesisRow {
    metric: String,
    two_p_value: Option<f64>,
    foil_value: Option<f64>,
    difference_two_p_minus_foil: Option<f64>,
    supporting_bases: String,
    contradicting_bases: String,
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
    two_p_pairs: usize,
    foil_pairs: usize,
    two_p_bases_with_m2_activity: String,
    foil_bases_with_m2_activity: String,
    main_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    pair_audit_rows: Vec<PairAuditRow>,
    base_summary_rows: Vec<BaseSummaryRow>,
    family_summary_rows: Vec<FamilySummaryRow>,
    hypothesis_rows: Vec<HypothesisRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

#[derive(Debug, Clone)]
struct SharedYieldMetrics {
    shared_admissible_count: usize,
    shared_prime_delta_count: isize,
    overlap_prime_delta_count: isize,
    shared_prime_rate_k00_pp: f64,
    shared_prime_rate_best_pp: f64,
    shared_prime_rate_delta_pp: f64,
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
        two_p_bases: TWO_P_BASES.to_vec(),
        foil_bases: FOIL_BASES.to_vec(),
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
        middle_lengths: vec![M1, M2, M3],
        k_grid: DEFAULT_BOUNDED_K_GRID
            .iter()
            .map(|&config| format_k(config))
            .collect(),
    };

    let pair_audit_rows = build_pair_audit_rows(options.smoke_catalog);
    let base_summary_rows = build_base_summary_rows(&pair_audit_rows);
    let family_summary_rows = build_family_summary_rows(&base_summary_rows, &pair_audit_rows);
    let hypothesis_rows = build_hypothesis_rows(&base_summary_rows, &family_summary_rows);

    let scatter_path = options
        .out_dir
        .join("base_scatter_persistence_vs_shared_yield.png");
    render_base_scatter(&base_summary_rows, &scatter_path);
    let family_bar_path = options.out_dir.join("family_metric_bars.png");
    render_family_bar_chart(&family_summary_rows, &family_bar_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "base_scatter".to_string(),
            label: "Base persistence vs shared-yield-core scatter".to_string(),
            path: scatter_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "family_bars".to_string(),
            label: "2p family vs foil metrics".to_string(),
            path: family_bar_path.display().to_string(),
        },
    ];
    let report_summary =
        build_report_summary(&pair_audit_rows, &base_summary_rows, &family_summary_rows);
    let observations =
        derive_observations(&base_summary_rows, &family_summary_rows, &hypothesis_rows);

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        pair_audit_rows: pair_audit_rows.clone(),
        base_summary_rows: base_summary_rows.clone(),
        family_summary_rows: family_summary_rows.clone(),
        hypothesis_rows: hypothesis_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary,
        observations,
    };

    write_csv_rows(
        options.out_dir.join("pair_audit_rows.csv"),
        &pair_audit_rows,
    )
    .expect("failed to write pair_audit_rows.csv");
    write_csv_rows(
        options.out_dir.join("base_summary_rows.csv"),
        &base_summary_rows,
    )
    .expect("failed to write base_summary_rows.csv");
    write_csv_rows(
        options.out_dir.join("family_summary_rows.csv"),
        &family_summary_rows,
    )
    .expect("failed to write family_summary_rows.csv");
    write_csv_rows(
        options.out_dir.join("hypothesis_rows.csv"),
        &hypothesis_rows,
    )
    .expect("failed to write hypothesis_rows.csv");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle)
        .expect("failed to write summary.json");
    write_text_file(options.out_dir.join("report.md"), &render_markdown(&bundle))
        .expect("failed to write report.md");

    println!("2p family report");
    println!("  output dir: {}", options.out_dir.display());
    for row in &family_summary_rows {
        println!(
            "  {} | persistence {:?} | shared_yield_core {:?}",
            row.family,
            row.weighted_persistence_rate_given_m1,
            row.weighted_shared_yield_core_share_given_m2
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
                    .unwrap_or_else(|| panic!("missing value after --out-dir"));
                out_dir = PathBuf::from(value);
            }
            "--smoke" => smoke_catalog = true,
            "--help" | "-h" => {
                print_help_and_exit();
            }
            other => panic!("unrecognized argument: {other}"),
        }
    }
    Options {
        out_dir,
        smoke_catalog,
    }
}

fn print_help_and_exit() -> ! {
    println!("Usage:");
    println!("  cargo run --release --example two_p_family_report -- [options]");
    println!();
    println!("Options:");
    println!("  --out-dir <path>   Write output bundle to this directory");
    println!(
        "  --smoke            Run a reduced smoke catalog instead of the full ordered-pair catalog"
    );
    println!("  -h, --help         Show this help");
    std::process::exit(0);
}

fn build_pair_audit_rows(smoke_catalog: bool) -> Vec<PairAuditRow> {
    BASES
        .par_iter()
        .flat_map_iter(|&base| {
            let pairs = if smoke_catalog {
                let anchors = SMOKE_PAIR_ANCHORS
                    .iter()
                    .filter(|&&(anchor_base, _, _)| anchor_base == base)
                    .map(|&(_, outer, inner)| (outer, inner))
                    .collect::<Vec<_>>();
                select_smoke_pairs(base, SMOKE_MAX_ORDERED_PAIRS_PER_BASE, &anchors)
            } else {
                ordered_unit_pairs(base)
            };
            pairs
                .into_iter()
                .map(move |(outer, inner)| build_pair_audit_row(base, outer, inner))
        })
        .collect::<Vec<_>>()
}

fn build_pair_audit_row(base: u32, outer: u32, inner: u32) -> PairAuditRow {
    let row_m1 = evaluate_pair_row(base, M1, outer, inner, DEFAULT_BOUNDED_K_GRID);
    let row_m2 = evaluate_pair_row(base, M2, outer, inner, DEFAULT_BOUNDED_K_GRID);
    let row_m3 = evaluate_pair_row(base, M3, outer, inner, DEFAULT_BOUNDED_K_GRID);
    let anomaly_m1 = anomaly_mass(&row_m1);
    let anomaly_m2 = anomaly_mass(&row_m2);
    let anomaly_m3 = anomaly_mass(&row_m3);
    let m1_anomalous = anomaly_m1 > 0.0;
    let m2_active = anomaly_m2 > 0.0;
    let m3_active = anomaly_m3 > 0.0;
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

    PairAuditRow {
        family: family_label(base).to_string(),
        base,
        outer,
        inner,
        pair_label: format!("({},{})", digit_symbol(outer), digit_symbol(inner)),
        best_k_m1: row_m1.best_k,
        best_k_m2: row_m2.best_k.clone(),
        best_k_m3: row_m3.best_k,
        anomaly_m1_pp: anomaly_m1,
        anomaly_m2_pp: anomaly_m2,
        anomaly_m3_pp: anomaly_m3,
        m1_anomalous,
        m2_active,
        m2_persistent: m1_anomalous && m2_active,
        m2_emergent: !m1_anomalous && m2_active,
        boundary_class: boundary_class(m1_anomalous, m2_active, m3_active),
        shared_admissible_count: shared_metrics
            .as_ref()
            .map(|metrics| metrics.shared_admissible_count),
        shared_prime_delta_count: shared_metrics
            .as_ref()
            .map(|metrics| metrics.shared_prime_delta_count),
        overlap_prime_delta_count: shared_metrics
            .as_ref()
            .map(|metrics| metrics.overlap_prime_delta_count),
        shared_prime_rate_k00_pp: shared_metrics
            .as_ref()
            .map(|metrics| metrics.shared_prime_rate_k00_pp),
        shared_prime_rate_best_pp: shared_metrics
            .as_ref()
            .map(|metrics| metrics.shared_prime_rate_best_pp),
        shared_prime_rate_delta_pp: shared_metrics
            .as_ref()
            .map(|metrics| metrics.shared_prime_rate_delta_pp),
        admissible_set_effect_pp: shared_metrics
            .as_ref()
            .map(|metrics| metrics.admissible_set_effect_pp),
        prime_yield_effect_pp: shared_metrics
            .as_ref()
            .map(|metrics| metrics.prime_yield_effect_pp),
        positive_shared_yield: shared_metrics
            .as_ref()
            .map(|metrics| metrics.positive_shared_yield),
        shared_yield_core: shared_metrics
            .as_ref()
            .map(|metrics| metrics.shared_yield_core),
    }
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
    let mut shared_admissible_count = 0usize;
    let mut shared_prime_hits_k00 = 0usize;
    let mut shared_prime_hits_best = 0usize;

    for (k00_row, best_row) in k00_profile
        .candidate_rows
        .iter()
        .zip(&best_profile.candidate_rows)
    {
        match (k00_row.admissible, best_row.admissible) {
            (true, true) => {
                shared_admissible_count += 1;
                if k00_row.prime {
                    shared_prime_hits_k00 += 1;
                    shared_prime_delta_count -= 1;
                }
                if best_row.prime {
                    shared_prime_hits_best += 1;
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
        shared_admissible_count,
        shared_prime_delta_count,
        overlap_prime_delta_count,
        shared_prime_rate_k00_pp: ratio(shared_prime_hits_k00, shared_admissible_count) * 100.0,
        shared_prime_rate_best_pp: ratio(shared_prime_hits_best, shared_admissible_count) * 100.0,
        shared_prime_rate_delta_pp: (ratio(shared_prime_hits_best, shared_admissible_count)
            - ratio(shared_prime_hits_k00, shared_admissible_count))
            * 100.0,
        admissible_set_effect_pp,
        prime_yield_effect_pp,
        positive_shared_yield: shared_prime_delta_count > 0,
        shared_yield_core: shared_prime_delta_count > overlap_prime_delta_count.abs()
            && shared_prime_delta_count > 0
            && prime_yield_effect_pp.abs() > admissible_set_effect_pp.abs(),
    }
}

fn build_base_summary_rows(rows: &[PairAuditRow]) -> Vec<BaseSummaryRow> {
    let mut by_base = BTreeMap::<u32, Vec<&PairAuditRow>>::new();
    for row in rows {
        by_base.entry(row.base).or_default().push(row);
    }

    by_base
        .into_iter()
        .map(|(base, group)| {
            let family = family_label(base).to_string();
            let ordered_pair_count = group.len();
            let m1_anomalous_pairs = group.iter().filter(|row| row.m1_anomalous).count();
            let m2_active_pairs = group.iter().filter(|row| row.m2_active).count();
            let m2_persistent_pairs = group.iter().filter(|row| row.m2_persistent).count();
            let m2_emergent_pairs = group.iter().filter(|row| row.m2_emergent).count();
            let m3_active_pairs = group.iter().filter(|row| row.anomaly_m3_pp > 0.0).count();
            let m1_anomaly_mass_pp = group.iter().map(|row| row.anomaly_m1_pp).sum();
            let m2_anomaly_mass_pp = group.iter().map(|row| row.anomaly_m2_pp).sum();
            let m3_anomaly_mass_pp = group.iter().map(|row| row.anomaly_m3_pp).sum();
            let positive_shared_yield_pairs = group
                .iter()
                .filter(|row| row.positive_shared_yield == Some(true))
                .count();
            let shared_yield_core_pairs = group
                .iter()
                .filter(|row| row.shared_yield_core == Some(true))
                .count();
            let shared_prime_deltas = group
                .iter()
                .filter_map(|row| row.shared_prime_delta_count.map(|value| value as f64))
                .collect::<Vec<_>>();

            BaseSummaryRow {
                family,
                base,
                ordered_pair_count,
                m1_anomalous_pairs,
                m2_active_pairs,
                m2_persistent_pairs,
                m2_emergent_pairs,
                m3_active_pairs,
                m1_anomaly_mass_pp,
                m2_anomaly_mass_pp,
                m3_anomaly_mass_pp,
                persistence_rate_given_m1: ratio_option(m2_persistent_pairs, m1_anomalous_pairs),
                positive_shared_yield_pairs,
                shared_yield_core_pairs,
                positive_shared_yield_share_given_m2: ratio_option(
                    positive_shared_yield_pairs,
                    m2_active_pairs,
                ),
                shared_yield_core_share_given_m2: ratio_option(
                    shared_yield_core_pairs,
                    m2_active_pairs,
                ),
                mean_shared_prime_delta_count: mean(&shared_prime_deltas),
            }
        })
        .collect()
}

fn build_family_summary_rows(
    base_rows: &[BaseSummaryRow],
    pair_rows: &[PairAuditRow],
) -> Vec<FamilySummaryRow> {
    let mut by_family_base = BTreeMap::<String, Vec<&BaseSummaryRow>>::new();
    for row in base_rows {
        by_family_base
            .entry(row.family.clone())
            .or_default()
            .push(row);
    }

    let mut by_family_pairs = BTreeMap::<String, Vec<&PairAuditRow>>::new();
    for row in pair_rows {
        by_family_pairs
            .entry(row.family.clone())
            .or_default()
            .push(row);
    }

    by_family_base
        .into_iter()
        .map(|(family, base_group)| {
            let pair_group = by_family_pairs
                .get(&family)
                .expect("family pair group should exist");
            let ordered_pair_count = pair_group.len();
            let m1_anomalous_pairs = pair_group.iter().filter(|row| row.m1_anomalous).count();
            let m2_active_pairs = pair_group.iter().filter(|row| row.m2_active).count();
            let m2_persistent_pairs = pair_group.iter().filter(|row| row.m2_persistent).count();
            let m2_emergent_pairs = pair_group.iter().filter(|row| row.m2_emergent).count();
            let m3_active_pairs = pair_group
                .iter()
                .filter(|row| row.anomaly_m3_pp > 0.0)
                .count();
            let positive_shared_yield_pairs = pair_group
                .iter()
                .filter(|row| row.positive_shared_yield == Some(true))
                .count();
            let shared_yield_core_pairs = pair_group
                .iter()
                .filter(|row| row.shared_yield_core == Some(true))
                .count();
            let base_persistence = base_group
                .iter()
                .filter_map(|row| row.persistence_rate_given_m1)
                .collect::<Vec<_>>();
            let base_core_share = base_group
                .iter()
                .filter_map(|row| row.shared_yield_core_share_given_m2)
                .collect::<Vec<_>>();
            let shared_prime_deltas = pair_group
                .iter()
                .filter_map(|row| row.shared_prime_delta_count.map(|value| value as f64))
                .collect::<Vec<_>>();

            FamilySummaryRow {
                family: family.clone(),
                base_count: base_group.len(),
                bases: base_group
                    .iter()
                    .map(|row| row.base.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                ordered_pair_count,
                m1_anomalous_pairs,
                m2_active_pairs,
                m2_persistent_pairs,
                m2_emergent_pairs,
                m3_active_pairs,
                weighted_persistence_rate_given_m1: ratio_option(
                    m2_persistent_pairs,
                    m1_anomalous_pairs,
                ),
                weighted_positive_shared_yield_share_given_m2: ratio_option(
                    positive_shared_yield_pairs,
                    m2_active_pairs,
                ),
                weighted_shared_yield_core_share_given_m2: ratio_option(
                    shared_yield_core_pairs,
                    m2_active_pairs,
                ),
                mean_base_persistence_rate_given_m1: mean(&base_persistence),
                mean_base_shared_yield_core_share_given_m2: mean(&base_core_share),
                mean_shared_prime_delta_count: mean(&shared_prime_deltas),
            }
        })
        .collect()
}

fn build_hypothesis_rows(
    base_rows: &[BaseSummaryRow],
    family_rows: &[FamilySummaryRow],
) -> Vec<HypothesisRow> {
    let two_p = family_rows
        .iter()
        .find(|row| row.family == "2p")
        .expect("2p family row should exist");
    let foil = family_rows
        .iter()
        .find(|row| row.family == "foil")
        .expect("foil family row should exist");

    vec![
        HypothesisRow {
            metric: "weighted_persistence_rate_given_m1".to_string(),
            two_p_value: two_p.weighted_persistence_rate_given_m1,
            foil_value: foil.weighted_persistence_rate_given_m1,
            difference_two_p_minus_foil: subtract_options(
                two_p.weighted_persistence_rate_given_m1,
                foil.weighted_persistence_rate_given_m1,
            ),
            supporting_bases: supporting_bases(base_rows, |row| row.persistence_rate_given_m1),
            contradicting_bases: contradicting_bases(base_rows, |row| {
                row.persistence_rate_given_m1
            }),
        },
        HypothesisRow {
            metric: "weighted_shared_yield_core_share_given_m2".to_string(),
            two_p_value: two_p.weighted_shared_yield_core_share_given_m2,
            foil_value: foil.weighted_shared_yield_core_share_given_m2,
            difference_two_p_minus_foil: subtract_options(
                two_p.weighted_shared_yield_core_share_given_m2,
                foil.weighted_shared_yield_core_share_given_m2,
            ),
            supporting_bases: supporting_bases(base_rows, |row| {
                row.shared_yield_core_share_given_m2
            }),
            contradicting_bases: contradicting_bases(base_rows, |row| {
                row.shared_yield_core_share_given_m2
            }),
        },
        HypothesisRow {
            metric: "mean_shared_prime_delta_count".to_string(),
            two_p_value: two_p.mean_shared_prime_delta_count,
            foil_value: foil.mean_shared_prime_delta_count,
            difference_two_p_minus_foil: subtract_options(
                two_p.mean_shared_prime_delta_count,
                foil.mean_shared_prime_delta_count,
            ),
            supporting_bases: supporting_bases(base_rows, |row| row.mean_shared_prime_delta_count),
            contradicting_bases: contradicting_bases(base_rows, |row| {
                row.mean_shared_prime_delta_count
            }),
        },
    ]
}

fn render_base_scatter(rows: &[BaseSummaryRow], path: &std::path::Path) {
    let root = BitMapBackend::new(path, (980, 760)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill scatter canvas");
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Base-Level M=2 Persistence vs Shared-Yield-Core Share",
            ("sans-serif", 24),
        )
        .margin(24)
        .x_label_area_size(56)
        .y_label_area_size(64)
        .build_cartesian_2d(-0.02f64..1.02f64, -0.02f64..1.02f64)
        .expect("failed to build base scatter");

    chart
        .configure_mesh()
        .x_desc("persistence rate given M=1")
        .y_desc("shared-yield-core share given M=2")
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .light_line_style(RGBColor(214, 207, 196))
        .draw()
        .expect("failed to draw scatter mesh");

    for row in rows {
        let x = row.persistence_rate_given_m1.unwrap_or(0.0);
        let y = row.shared_yield_core_share_given_m2.unwrap_or(0.0);
        let color = family_color(&row.family);
        chart
            .draw_series(std::iter::once(Circle::new(
                (x, y),
                8,
                ShapeStyle::from(&color).filled(),
            )))
            .expect("failed to draw scatter point");
        chart
            .draw_series(std::iter::once(Text::new(
                row.base.to_string(),
                (x + 0.015, y + 0.015),
                ("sans-serif", 18).into_font().color(&BLACK),
            )))
            .expect("failed to draw scatter label");
    }

    root.present().expect("failed to present base scatter");
}

fn render_family_bar_chart(rows: &[FamilySummaryRow], path: &std::path::Path) {
    let families = ["2p", "foil"];
    let root = BitMapBackend::new(path, (980, 620)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill family bar canvas");
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "2p Family vs Foils  (pair-weighted metrics)",
            ("sans-serif", 24),
        )
        .margin(24)
        .x_label_area_size(56)
        .y_label_area_size(64)
        .build_cartesian_2d(0f64..(families.len() as f64), 0f64..1.0f64)
        .expect("failed to build family bar chart");

    chart
        .configure_mesh()
        .x_desc("family")
        .y_desc("share")
        .x_labels(families.len())
        .x_label_formatter(&move |value| {
            let index = value.floor() as usize;
            families.get(index).copied().unwrap_or("").to_string()
        })
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .light_line_style(RGBColor(214, 207, 196))
        .draw()
        .expect("failed to draw family bar mesh");

    for (index, family) in families.iter().enumerate() {
        let row = rows
            .iter()
            .find(|row| row.family == *family)
            .unwrap_or_else(|| panic!("missing family row for {family}"));
        let x = index as f64;
        let persistence = row.weighted_persistence_rate_given_m1.unwrap_or(0.0);
        let core_share = row.weighted_shared_yield_core_share_given_m2.unwrap_or(0.0);
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x + 0.08, 0.0), (x + 0.40, persistence)],
                ShapeStyle::from(&RGBColor(69, 129, 190)).filled(),
            )))
            .expect("failed to draw persistence bar");
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x + 0.54, 0.0), (x + 0.86, core_share)],
                ShapeStyle::from(&RGBColor(191, 82, 32)).filled(),
            )))
            .expect("failed to draw shared-yield-core bar");
    }

    root.present().expect("failed to present family bar chart");
}

fn family_color(family: &str) -> RGBColor {
    match family {
        "2p" => RGBColor(69, 129, 190),
        "foil" => RGBColor(191, 82, 32),
        _ => RGBColor(110, 110, 110),
    }
}

fn build_report_summary(
    pair_rows: &[PairAuditRow],
    base_rows: &[BaseSummaryRow],
    family_rows: &[FamilySummaryRow],
) -> ReportSummary {
    let two_p = family_rows
        .iter()
        .find(|row| row.family == "2p")
        .expect("2p family row should exist");
    let foil = family_rows
        .iter()
        .find(|row| row.family == "foil")
        .expect("foil family row should exist");

    ReportSummary {
        total_pairs: pair_rows.len(),
        two_p_pairs: pair_rows.iter().filter(|row| row.family == "2p").count(),
        foil_pairs: pair_rows.iter().filter(|row| row.family == "foil").count(),
        two_p_bases_with_m2_activity: base_rows
            .iter()
            .filter(|row| row.family == "2p" && row.m2_active_pairs > 0)
            .map(|row| row.base.to_string())
            .collect::<Vec<_>>()
            .join(", "),
        foil_bases_with_m2_activity: base_rows
            .iter()
            .filter(|row| row.family == "foil" && row.m2_active_pairs > 0)
            .map(|row| row.base.to_string())
            .collect::<Vec<_>>()
            .join(", "),
        main_takeaway: match (
            subtract_options(
                two_p.weighted_persistence_rate_given_m1,
                foil.weighted_persistence_rate_given_m1,
            ),
            subtract_options(
                two_p.weighted_shared_yield_core_share_given_m2,
                foil.weighted_shared_yield_core_share_given_m2,
            ),
        ) {
            (Some(persistence_gap), Some(core_gap)) if persistence_gap > 0.0 && core_gap > 0.0 => {
                "2p bases outperform foils on both M=2 persistence and shared-yield-core share."
                    .to_string()
            }
            _ => "The 2p signal is mixed: at least one family metric fails to clear the foils."
                .to_string(),
        },
    }
}

fn derive_observations(
    base_rows: &[BaseSummaryRow],
    family_rows: &[FamilySummaryRow],
    hypothesis_rows: &[HypothesisRow],
) -> Vec<String> {
    let two_p = family_rows
        .iter()
        .find(|row| row.family == "2p")
        .expect("2p family row should exist");
    let foil = family_rows
        .iter()
        .find(|row| row.family == "foil")
        .expect("foil family row should exist");
    let strongest_two_p = base_rows
        .iter()
        .filter(|row| row.family == "2p")
        .max_by(|left, right| {
            left.shared_yield_core_share_given_m2
                .unwrap_or(0.0)
                .total_cmp(&right.shared_yield_core_share_given_m2.unwrap_or(0.0))
                .then_with(|| left.base.cmp(&right.base))
        })
        .expect("2p base rows should exist");
    let strongest_foil = base_rows
        .iter()
        .filter(|row| row.family == "foil")
        .max_by(|left, right| {
            left.shared_yield_core_share_given_m2
                .unwrap_or(0.0)
                .total_cmp(&right.shared_yield_core_share_given_m2.unwrap_or(0.0))
                .then_with(|| left.base.cmp(&right.base))
        })
        .expect("foil base rows should exist");
    let foil_m2_active_pairs = foil.m2_active_pairs;
    let base22 = base_rows
        .iter()
        .find(|row| row.base == 22)
        .expect("base 22 row should exist");
    let base26 = base_rows
        .iter()
        .find(|row| row.base == 26)
        .expect("base 26 row should exist");

    vec![
        format!(
            "Pair-weighted M=2 persistence given M=1 is {} for the 2p family versus {} for the foils.",
            format_option_share(two_p.weighted_persistence_rate_given_m1),
            format_option_share(foil.weighted_persistence_rate_given_m1)
        ),
        format!(
            "Pair-weighted shared-yield-core share given M=2 is {} for the 2p family versus {} for the foils, but that foil number is based on only {} active M=2 foil pairs.",
            format_option_share(two_p.weighted_shared_yield_core_share_given_m2),
            format_option_share(foil.weighted_shared_yield_core_share_given_m2),
            foil_m2_active_pairs
        ),
        format!(
            "The strongest 2p shared-yield-core base is {}, while the strongest foil is {}.",
            strongest_two_p.base, strongest_foil.base
        ),
        format!(
            "Extending from 14 to the larger 2p bases weakens the naive universal story: base 22 has {} M=2 persistent pairs and base 26 has {}.",
            base22.m2_persistent_pairs, base26.m2_persistent_pairs
        ),
        format!(
            "Metric-by-metric hypothesis rows: {}.",
            hypothesis_rows
                .iter()
                .map(|row| {
                    format!(
                        "{} => {}",
                        row.metric,
                        format_metric_value(&row.metric, row.difference_two_p_minus_foil)
                    )
                })
                .collect::<Vec<_>>()
                .join("; ")
        ),
    ]
}

fn render_markdown(bundle: &ReportBundle) -> String {
    let mut markdown = String::new();
    markdown.push_str("# 2p Family Report\n\n");
    markdown.push_str("_Generated from `examples/two_p_family_report.rs`._\n\n");
    markdown.push_str(&format!(
        "- Output directory: `{}`\n- Pair catalog mode: `{}`\n\n",
        bundle.settings.out_dir, bundle.settings.pair_catalog_mode
    ));

    markdown.push_str("## Family Summary\n\n");
    markdown.push_str(
        "| Family | Bases | Weighted persistence | Weighted shared-yield-core | Mean shared prime delta |\n",
    );
    markdown.push_str("|---|---|---:|---:|---:|\n");
    for row in &bundle.family_summary_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            row.family,
            row.bases,
            format_option_share(row.weighted_persistence_rate_given_m1),
            format_option_share(row.weighted_shared_yield_core_share_given_m2),
            format_option_float(row.mean_shared_prime_delta_count)
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Hypothesis Rows\n\n");
    markdown.push_str("| Metric | 2p | Foil | Difference |\n");
    markdown.push_str("|---|---:|---:|---:|\n");
    for row in &bundle.hypothesis_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            row.metric,
            format_metric_value(&row.metric, row.two_p_value),
            format_metric_value(&row.metric, row.foil_value),
            format_metric_value(&row.metric, row.difference_two_p_minus_foil)
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

fn family_label(base: u32) -> &'static str {
    if TWO_P_BASES.contains(&base) {
        "2p"
    } else if FOIL_BASES.contains(&base) {
        "foil"
    } else {
        panic!("base {base} is not classified");
    }
}

fn anomaly_mass(row: &KDominancePairRow) -> f64 {
    row.best_minus_k00_pp.max(0.0)
}

fn boundary_class(m1: bool, m2: bool, m3: bool) -> String {
    match (m1, m2, m3) {
        (false, false, false) => "never_anomalous",
        (true, false, false) => "m1_only",
        (true, true, false) => "m1_to_m2",
        (false, true, false) => "m2_only",
        (true, true, true) => "m1_to_m2_to_m3",
        (false, true, true) => "m2_to_m3",
        (false, false, true) => "m3_only",
        (true, false, true) => "non_monotone",
    }
    .to_string()
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
        Some(numerator as f64 / denominator as f64)
    }
}

fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        None
    } else {
        Some(values.iter().sum::<f64>() / values.len() as f64)
    }
}

fn subtract_options(left: Option<f64>, right: Option<f64>) -> Option<f64> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left - right),
        _ => None,
    }
}

fn supporting_bases(
    rows: &[BaseSummaryRow],
    metric: impl Fn(&BaseSummaryRow) -> Option<f64>,
) -> String {
    rows.iter()
        .filter(|row| row.family == "2p")
        .filter_map(|row| metric(row).map(|value| (row.base, value)))
        .filter(|(_, value)| *value > 0.0)
        .map(|(base, _)| base.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

fn contradicting_bases(
    rows: &[BaseSummaryRow],
    metric: impl Fn(&BaseSummaryRow) -> Option<f64>,
) -> String {
    rows.iter()
        .filter(|row| row.family == "foil")
        .filter_map(|row| metric(row).map(|value| (row.base, value)))
        .filter(|(_, value)| *value > 0.0)
        .map(|(base, _)| base.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_option_share(value: Option<f64>) -> String {
    value
        .map(|value| format!("{:.2}%", value * 100.0))
        .unwrap_or_else(|| "n/a".to_string())
}

fn format_option_float(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.3}"))
        .unwrap_or_else(|| "n/a".to_string())
}

fn format_metric_value(metric: &str, value: Option<f64>) -> String {
    if matches!(
        metric,
        "weighted_persistence_rate_given_m1" | "weighted_shared_yield_core_share_given_m2"
    ) {
        format_option_share(value)
    } else {
        format_option_float(value)
    }
}
