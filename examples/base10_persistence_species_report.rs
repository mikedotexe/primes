//! Focused species report for the base-10 persistence-only lane.
//!
//! The hinge robustness matrix showed that removing base `10` `(3,3)` is the
//! one representative-drop scenario that demotes `overlap_boundary` from
//! `deepest` to `bridge`. This report treats that as a signal worth probing
//! directly.
//!
//! The report asks whether base `10` `(3,3)` is:
//! - merely a singleton crutch, or
//! - a coherent persistence-only species witness.
//!
//! It does that by comparing the anchor against:
//! - structured same-base neighbors from the full base-10 ordered-unit surface
//! - a tiny persistence-only outgroup: base `6` `(5,5)`
//! - a hinge outgroup: base `14` `(D,B)`
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example base10_persistence_species_report
//! cargo run --release --example base10_persistence_species_report -- --out-dir /tmp/primes_base10_persistence_species
//! ```

use plotters::prelude::*;
use primes::validation::{
    bounded_k::{
        analyze_hinge_feature_row, digit_symbol, ordered_unit_pairs, unit_residues,
        HingeFeatureRow, HINGE_CATEGORY_PERSISTENCE_ONLY,
    },
    hinge_atoms::{
        build_hinge_atom_specs_with_policy, default_hinge_atom_catalog_policy,
        run_hinge_rule_search, HingeAtomCatalogPolicy, HingeSearchProblem, HingeThresholdPolicy,
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

const BASE: u32 = 10;
const ANCHOR_OUTER: u32 = 3;
const ANCHOR_INNER: u32 = 3;
const DEFAULT_OUT_DIR: &str = "/tmp/primes_base10_persistence_species";
const REPORT_EXPORT_VERSION: u32 = 1;
const ARTIFACT_ID: &str = "base10_persistence_species_report";
const MAX_RULE_ATOMS: usize = 3;
const EXPORTED_RULE_FRONTIER: usize = 12;
const BEST_RULES_PER_SEARCH: usize = 5;

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    base: u32,
    anchor_pair: String,
    outgroups: Vec<String>,
    max_rule_atoms: usize,
}

#[derive(Debug, Clone)]
struct FocusEntry {
    role: String,
    row: HingeFeatureRow,
    species_positive: bool,
}

#[derive(Debug, Clone, Serialize)]
struct NeighborRow {
    role: String,
    base: u32,
    pair_label: String,
    hinge_category: String,
    gap_bucket: String,
    same_digit: bool,
    same_gap_bucket_as_anchor: bool,
    same_same_digit_as_anchor: bool,
    same_m1_best_k_as_anchor: bool,
    same_m2_best_k_as_anchor: bool,
    m1_active: bool,
    m2_active: bool,
    persistent: bool,
    pair_distance: usize,
    overall_rank: usize,
    m1_anomaly_mass_pp: f64,
    m2_anomaly_mass_pp: f64,
    m2_stable_zero_signal_margin_count: isize,
    m2_boundary_prime_delta_count: isize,
    m2_stable_zero_prime_delta_count: isize,
    m2_stable_zero_support_ratio: f64,
    m2_admissible_overlap_jaccard: f64,
}

#[derive(Debug, Clone, Serialize)]
struct FocusRow {
    role: String,
    scope: String,
    base: u32,
    pair_label: String,
    hinge_category: String,
    same_digit: bool,
    gap_bucket: String,
    unit_distance: usize,
    m1_best_k: String,
    m2_best_k: String,
    m1_active: bool,
    m2_active: bool,
    persistent: bool,
    shared_yield_core: bool,
    m1_anomaly_mass_pp: f64,
    m2_anomaly_mass_pp: f64,
    m1_stable_zero_signal_margin_count: isize,
    m2_stable_zero_signal_margin_count: isize,
    m2_stable_zero_prime_delta_count: isize,
    m2_boundary_prime_delta_count: isize,
    m2_stable_zero_support_ratio: f64,
    m2_admissible_overlap_jaccard: f64,
    m2_signal_source_label: String,
    local_species_positive: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ComparisonRow {
    comparison_role: String,
    base: u32,
    pair_label: String,
    same_base: bool,
    pair_distance: Option<usize>,
    same_gap_bucket: bool,
    same_same_digit: bool,
    same_m1_best_k: bool,
    same_m2_best_k: bool,
    comparison_hinge_category: String,
    anchor_m1_anomaly_mass_pp: f64,
    comparison_m1_anomaly_mass_pp: f64,
    delta_m1_anomaly_mass_pp: f64,
    anchor_m2_anomaly_mass_pp: f64,
    comparison_m2_anomaly_mass_pp: f64,
    delta_m2_anomaly_mass_pp: f64,
    anchor_m2_stable_zero_signal_margin_count: isize,
    comparison_m2_stable_zero_signal_margin_count: isize,
    delta_m2_stable_zero_signal_margin_count: isize,
    anchor_m2_boundary_prime_delta_count: isize,
    comparison_m2_boundary_prime_delta_count: isize,
    delta_m2_boundary_prime_delta_count: isize,
    anchor_m2_stable_zero_support_ratio: f64,
    comparison_m2_stable_zero_support_ratio: f64,
    delta_m2_stable_zero_support_ratio: f64,
}

#[derive(Debug, Clone, Serialize)]
struct LocalRuleRow {
    threshold_policy: String,
    rank: usize,
    status: String,
    exact_match: bool,
    rule_label: String,
    atom_count: usize,
    threshold_free: bool,
    rule_theorem_class: String,
    true_positive: usize,
    false_positive: usize,
    false_negative: usize,
    true_negative: usize,
    f1: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    anchor_pair: String,
    selected_same_base_neighbors: Vec<String>,
    local_exact_rule_observed: String,
    local_exact_rule_min_side_2: String,
    local_mechanism_rule: String,
    main_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    neighbor_rows: Vec<NeighborRow>,
    focus_rows: Vec<FocusRow>,
    comparison_rows: Vec<ComparisonRow>,
    local_rule_rows: Vec<LocalRuleRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let anchor = analyze_hinge_feature_row(BASE, ANCHOR_OUTER, ANCHOR_INNER);
    let base_rows = build_hinge_feature_rows(&[BASE]);
    let neighbor_rows = build_neighbor_rows(&anchor, &base_rows);
    let focus_entries = build_focus_entries(&anchor, &base_rows, &neighbor_rows);
    let focus_rows = focus_entries.iter().map(focus_row).collect::<Vec<_>>();
    let comparison_rows = build_comparison_rows(&anchor, &focus_entries);
    let local_rule_rows = build_local_rule_rows(&focus_entries);

    let neighborhood_path = options.out_dir.join("base10_persistence_neighborhood.png");
    render_neighborhood(&base_rows, &focus_entries, &neighborhood_path);
    let plane_path = options.out_dir.join("base10_persistence_plane.png");
    render_species_plane(&focus_entries, &plane_path);
    let strip_path = options.out_dir.join("base10_persistence_strip.png");
    render_focus_strip(&focus_entries, &strip_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "neighborhood".to_string(),
            label: "Base 10 persistence neighborhood".to_string(),
            path: neighborhood_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "species_plane".to_string(),
            label: "Base 10 persistence plane".to_string(),
            path: plane_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "focus_strip".to_string(),
            label: "Base 10 persistence strip".to_string(),
            path: strip_path.display().to_string(),
        },
    ];

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        base: BASE,
        anchor_pair: anchor.pair_label.clone(),
        outgroups: vec!["base 6 (5,5)".to_string(), "base 14 (D,B)".to_string()],
        max_rule_atoms: MAX_RULE_ATOMS,
    };

    let report_summary = build_report_summary(&focus_entries, &local_rule_rows);
    let observations = derive_observations(&focus_entries, &comparison_rows, &local_rule_rows);

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        neighbor_rows: neighbor_rows.clone(),
        focus_rows: focus_rows.clone(),
        comparison_rows: comparison_rows.clone(),
        local_rule_rows: local_rule_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary,
        observations,
    };

    write_csv_rows(options.out_dir.join("neighbor_rows.csv"), &neighbor_rows)
        .expect("failed to write neighbor_rows.csv");
    write_csv_rows(options.out_dir.join("focus_rows.csv"), &focus_rows)
        .expect("failed to write focus_rows.csv");
    write_csv_rows(
        options.out_dir.join("comparison_rows.csv"),
        &comparison_rows,
    )
    .expect("failed to write comparison_rows.csv");
    write_csv_rows(
        options.out_dir.join("local_rule_rows.csv"),
        &local_rule_rows,
    )
    .expect("failed to write local_rule_rows.csv");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle)
        .expect("failed to write summary.json");
    write_text_file(options.out_dir.join("report.md"), &render_markdown(&bundle))
        .expect("failed to write report.md");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo".to_string(),
            args: vec![
                "run".to_string(),
                "--release".to_string(),
                "--example".to_string(),
                "base10_persistence_species_report".to_string(),
                "--".to_string(),
                "--out-dir".to_string(),
                options.out_dir.display().to_string(),
            ],
            upstream_inputs: vec![],
            expected_outputs: vec![
                "neighbor_rows.csv".to_string(),
                "focus_rows.csv".to_string(),
                "comparison_rows.csv".to_string(),
                "local_rule_rows.csv".to_string(),
                "summary.json".to_string(),
                "report.md".to_string(),
                "artifact_manifest.json".to_string(),
                "base10_persistence_neighborhood.png".to_string(),
                "base10_persistence_plane.png".to_string(),
                "base10_persistence_strip.png".to_string(),
            ],
        },
    )
    .expect("failed to write artifact manifest");

    println!("base10 persistence species report");
    println!("  output dir: {}", options.out_dir.display());
    println!("  anchor: {}", anchor.pair_label);
    for row in &focus_rows {
        println!(
            "  {:<28} | {:<8} | {:<8} | m1 {:>6.2}pp | m2 {:>6.2}pp | margin {:>3} | boundary {:>3}",
            row.role,
            format!("base {}", row.base),
            row.pair_label,
            row.m1_anomaly_mass_pp,
            row.m2_anomaly_mass_pp,
            row.m2_stable_zero_signal_margin_count,
            row.m2_boundary_prime_delta_count,
        );
    }
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => {
                let value = args
                    .next()
                    .expect("--out-dir requires a directory argument");
                out_dir = PathBuf::from(value);
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => panic!("unrecognized argument: {arg}"),
        }
    }
    Options { out_dir }
}

fn print_help() {
    println!("Usage:");
    println!("  cargo run --release --example base10_persistence_species_report -- [options]");
    println!();
    println!("Options:");
    println!("  --out-dir <dir>   Output directory (default: {DEFAULT_OUT_DIR})");
    println!("  -h, --help        Show this help message");
}

fn build_hinge_feature_rows(bases: &[u32]) -> Vec<HingeFeatureRow> {
    bases
        .par_iter()
        .copied()
        .flat_map(|base| {
            ordered_unit_pairs(base)
                .into_par_iter()
                .map(move |(outer, inner)| analyze_hinge_feature_row(base, outer, inner))
        })
        .collect()
}

fn build_neighbor_rows(
    anchor: &HingeFeatureRow,
    base_rows: &[HingeFeatureRow],
) -> Vec<NeighborRow> {
    let mut rows = base_rows
        .iter()
        .filter(|row| !(row.outer == anchor.outer && row.inner == anchor.inner))
        .map(|row| NeighborRow {
            role: "same_base_neighbor".to_string(),
            base: row.base,
            pair_label: row.pair_label.clone(),
            hinge_category: row.hinge_category.clone(),
            gap_bucket: row.gap_bucket.clone(),
            same_digit: row.same_digit,
            same_gap_bucket_as_anchor: row.gap_bucket == anchor.gap_bucket,
            same_same_digit_as_anchor: row.same_digit == anchor.same_digit,
            same_m1_best_k_as_anchor: row.m1_best_k == anchor.m1_best_k,
            same_m2_best_k_as_anchor: row.m2_best_k == anchor.m2_best_k,
            m1_active: row.m1_active,
            m2_active: row.m2_active,
            persistent: row.m1_to_m2_persistent,
            pair_distance: pair_distance(BASE, anchor.outer, anchor.inner, row.outer, row.inner),
            overall_rank: 0,
            m1_anomaly_mass_pp: row.m1_anomaly_mass_pp,
            m2_anomaly_mass_pp: row.m2_anomaly_mass_pp,
            m2_stable_zero_signal_margin_count: row.m2_stable_zero_signal_margin_count,
            m2_boundary_prime_delta_count: row.m2_boundary_prime_delta_count,
            m2_stable_zero_prime_delta_count: row.m2_stable_zero_prime_delta_count,
            m2_stable_zero_support_ratio: row.m2_stable_zero_support_ratio,
            m2_admissible_overlap_jaccard: row.m2_admissible_overlap_jaccard,
        })
        .collect::<Vec<_>>();

    rows.sort_by(|left, right| {
        right
            .same_same_digit_as_anchor
            .cmp(&left.same_same_digit_as_anchor)
            .then_with(|| {
                right
                    .same_gap_bucket_as_anchor
                    .cmp(&left.same_gap_bucket_as_anchor)
            })
            .then_with(|| {
                right
                    .same_m1_best_k_as_anchor
                    .cmp(&left.same_m1_best_k_as_anchor)
            })
            .then_with(|| right.m1_active.cmp(&left.m1_active))
            .then_with(|| right.m2_active.cmp(&left.m2_active))
            .then_with(|| left.pair_distance.cmp(&right.pair_distance))
            .then_with(|| {
                (left.m1_anomaly_mass_pp - anchor.m1_anomaly_mass_pp)
                    .abs()
                    .total_cmp(&(right.m1_anomaly_mass_pp - anchor.m1_anomaly_mass_pp).abs())
            })
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });
    for (index, row) in rows.iter_mut().enumerate() {
        row.overall_rank = index + 1;
    }
    rows
}

fn build_focus_entries(
    anchor: &HingeFeatureRow,
    _base_rows: &[HingeFeatureRow],
    neighbor_rows: &[NeighborRow],
) -> Vec<FocusEntry> {
    let mut entries = vec![FocusEntry {
        role: "anchor_base10_persistence_only".to_string(),
        row: anchor.clone(),
        species_positive: true,
    }];

    if let Some(row) = select_neighbor(neighbor_rows, |row| row.same_digit && !row.m2_active) {
        entries.push(FocusEntry {
            role: "same_digit_dead_control".to_string(),
            row,
            species_positive: false,
        });
    }

    if let Some(row) = select_neighbor(neighbor_rows, |row| {
        !row.same_digit && row.m1_active && !row.m2_active
    }) {
        entries.push(FocusEntry {
            role: "m1_only_dead_control".to_string(),
            row,
            species_positive: false,
        });
    }

    if let Some(row) = select_neighbor(neighbor_rows, |row| {
        row.m2_active && !row.persistent && row.same_m2_best_k_as_anchor
    }) {
        entries.push(FocusEntry {
            role: "active_same_lane_neighbor".to_string(),
            row,
            species_positive: false,
        });
    }

    if let Some(row) = select_neighbor(neighbor_rows, |row| {
        row.m2_active && !row.persistent && !row.same_m2_best_k_as_anchor
    }) {
        entries.push(FocusEntry {
            role: "active_other_lane_neighbor".to_string(),
            row,
            species_positive: false,
        });
    }

    entries.push(FocusEntry {
        role: "outgroup_tiny_persistence_only".to_string(),
        row: analyze_hinge_feature_row(6, 5, 5),
        species_positive: true,
    });
    entries.push(FocusEntry {
        role: "outgroup_hinge_bridge".to_string(),
        row: analyze_hinge_feature_row(14, 13, 11),
        species_positive: false,
    });

    let mut dedup = BTreeMap::<(u32, u32, u32), FocusEntry>::new();
    for entry in entries {
        dedup
            .entry((entry.row.base, entry.row.outer, entry.row.inner))
            .or_insert(entry);
    }

    let ordered_roles = [
        "anchor_base10_persistence_only",
        "same_digit_dead_control",
        "m1_only_dead_control",
        "active_same_lane_neighbor",
        "active_other_lane_neighbor",
        "outgroup_tiny_persistence_only",
        "outgroup_hinge_bridge",
    ];

    ordered_roles
        .iter()
        .filter_map(|role| dedup.values().find(|entry| entry.role == *role).cloned())
        .collect()
}

fn select_neighbor<F>(neighbor_rows: &[NeighborRow], predicate: F) -> Option<HingeFeatureRow>
where
    F: Fn(&NeighborRow) -> bool,
{
    neighbor_rows
        .iter()
        .find(|row| predicate(row))
        .map(parse_focus_row)
}

fn parse_focus_row(row: &NeighborRow) -> HingeFeatureRow {
    let (outer, inner) = parse_pair_label(&row.pair_label);
    analyze_hinge_feature_row(row.base, outer, inner)
}

fn parse_pair_label(label: &str) -> (u32, u32) {
    let trimmed = label.trim_matches(|ch| ch == '(' || ch == ')');
    let mut parts = trimmed.split(',');
    let outer = parse_digit_symbol(parts.next().expect("pair should have outer"));
    let inner = parse_digit_symbol(parts.next().expect("pair should have inner"));
    (outer, inner)
}

fn parse_digit_symbol(symbol: &str) -> u32 {
    let symbol = symbol.trim();
    symbol
        .chars()
        .next()
        .map(|ch| {
            if ch.is_ascii_digit() {
                ch.to_digit(10).expect("digit should parse")
            } else {
                10 + (ch as u32 - 'A' as u32)
            }
        })
        .expect("digit symbol should not be empty")
}

fn focus_row(entry: &FocusEntry) -> FocusRow {
    FocusRow {
        role: entry.role.clone(),
        scope: if entry.row.base == BASE {
            "same_base".to_string()
        } else {
            "outgroup".to_string()
        },
        base: entry.row.base,
        pair_label: entry.row.pair_label.clone(),
        hinge_category: entry.row.hinge_category.clone(),
        same_digit: entry.row.same_digit,
        gap_bucket: entry.row.gap_bucket.clone(),
        unit_distance: entry.row.unit_distance,
        m1_best_k: entry.row.m1_best_k.clone(),
        m2_best_k: entry.row.m2_best_k.clone(),
        m1_active: entry.row.m1_active,
        m2_active: entry.row.m2_active,
        persistent: entry.row.m1_to_m2_persistent,
        shared_yield_core: entry.row.shared_yield_core,
        m1_anomaly_mass_pp: entry.row.m1_anomaly_mass_pp,
        m2_anomaly_mass_pp: entry.row.m2_anomaly_mass_pp,
        m1_stable_zero_signal_margin_count: entry.row.m1_stable_zero_signal_margin_count,
        m2_stable_zero_signal_margin_count: entry.row.m2_stable_zero_signal_margin_count,
        m2_stable_zero_prime_delta_count: entry.row.m2_stable_zero_prime_delta_count,
        m2_boundary_prime_delta_count: entry.row.m2_boundary_prime_delta_count,
        m2_stable_zero_support_ratio: entry.row.m2_stable_zero_support_ratio,
        m2_admissible_overlap_jaccard: entry.row.m2_admissible_overlap_jaccard,
        m2_signal_source_label: entry.row.m2_signal_source_label.clone(),
        local_species_positive: entry.species_positive,
    }
}

fn build_comparison_rows(anchor: &HingeFeatureRow, entries: &[FocusEntry]) -> Vec<ComparisonRow> {
    entries
        .iter()
        .filter(|entry| {
            !(entry.row.base == anchor.base
                && entry.row.outer == anchor.outer
                && entry.row.inner == anchor.inner)
        })
        .map(|entry| ComparisonRow {
            comparison_role: entry.role.clone(),
            base: entry.row.base,
            pair_label: entry.row.pair_label.clone(),
            same_base: entry.row.base == anchor.base,
            pair_distance: (entry.row.base == anchor.base).then(|| {
                pair_distance(
                    BASE,
                    anchor.outer,
                    anchor.inner,
                    entry.row.outer,
                    entry.row.inner,
                )
            }),
            same_gap_bucket: entry.row.gap_bucket == anchor.gap_bucket,
            same_same_digit: entry.row.same_digit == anchor.same_digit,
            same_m1_best_k: entry.row.m1_best_k == anchor.m1_best_k,
            same_m2_best_k: entry.row.m2_best_k == anchor.m2_best_k,
            comparison_hinge_category: entry.row.hinge_category.clone(),
            anchor_m1_anomaly_mass_pp: anchor.m1_anomaly_mass_pp,
            comparison_m1_anomaly_mass_pp: entry.row.m1_anomaly_mass_pp,
            delta_m1_anomaly_mass_pp: entry.row.m1_anomaly_mass_pp - anchor.m1_anomaly_mass_pp,
            anchor_m2_anomaly_mass_pp: anchor.m2_anomaly_mass_pp,
            comparison_m2_anomaly_mass_pp: entry.row.m2_anomaly_mass_pp,
            delta_m2_anomaly_mass_pp: entry.row.m2_anomaly_mass_pp - anchor.m2_anomaly_mass_pp,
            anchor_m2_stable_zero_signal_margin_count: anchor.m2_stable_zero_signal_margin_count,
            comparison_m2_stable_zero_signal_margin_count: entry
                .row
                .m2_stable_zero_signal_margin_count,
            delta_m2_stable_zero_signal_margin_count: entry.row.m2_stable_zero_signal_margin_count
                - anchor.m2_stable_zero_signal_margin_count,
            anchor_m2_boundary_prime_delta_count: anchor.m2_boundary_prime_delta_count,
            comparison_m2_boundary_prime_delta_count: entry.row.m2_boundary_prime_delta_count,
            delta_m2_boundary_prime_delta_count: entry.row.m2_boundary_prime_delta_count
                - anchor.m2_boundary_prime_delta_count,
            anchor_m2_stable_zero_support_ratio: anchor.m2_stable_zero_support_ratio,
            comparison_m2_stable_zero_support_ratio: entry.row.m2_stable_zero_support_ratio,
            delta_m2_stable_zero_support_ratio: entry.row.m2_stable_zero_support_ratio
                - anchor.m2_stable_zero_support_ratio,
        })
        .collect()
}

fn build_local_rule_rows(entries: &[FocusEntry]) -> Vec<LocalRuleRow> {
    let rows = entries.iter().map(|entry| &entry.row).collect::<Vec<_>>();
    let target = entries
        .iter()
        .map(|entry| entry.species_positive)
        .collect::<Vec<_>>();
    let problem = HingeSearchProblem {
        id: "base10_local_persistence_species",
        label: "Local persistence-only species vs focused neighbors/outgroups",
        rows,
        target,
    };

    [
        HingeThresholdPolicy::Observed,
        HingeThresholdPolicy::ObservedMinSide2,
    ]
    .into_iter()
    .flat_map(|threshold_policy| {
        let mut policy: HingeAtomCatalogPolicy = default_hinge_atom_catalog_policy();
        policy.threshold_policy = threshold_policy;
        let atoms = build_hinge_atom_specs_with_policy(&problem, &policy);
        let outcome = run_hinge_rule_search(
            &problem,
            &atoms,
            MAX_RULE_ATOMS,
            EXPORTED_RULE_FRONTIER,
            BEST_RULES_PER_SEARCH,
        );
        outcome
            .best_rows
            .into_iter()
            .enumerate()
            .map(|(index, row)| LocalRuleRow {
                threshold_policy: threshold_policy.as_str().to_string(),
                rank: index + 1,
                status: outcome.summary.best_rule_status.clone(),
                exact_match: row.exact_match,
                rule_label: row.rule_label,
                atom_count: row.atom_count,
                threshold_free: row.threshold_free,
                rule_theorem_class: row.rule_theorem_class.as_str().to_string(),
                true_positive: row.true_positive,
                false_positive: row.false_positive,
                false_negative: row.false_negative,
                true_negative: row.true_negative,
                f1: row.f1,
            })
            .collect::<Vec<_>>()
    })
    .collect()
}

fn build_report_summary(entries: &[FocusEntry], local_rule_rows: &[LocalRuleRow]) -> ReportSummary {
    let selected_same_base_neighbors = entries
        .iter()
        .filter(|entry| {
            entry.row.base == BASE
                && !(entry.row.outer == ANCHOR_OUTER && entry.row.inner == ANCHOR_INNER)
        })
        .map(|entry| format!("{} {}", entry.row.pair_label, entry.role))
        .collect::<Vec<_>>();
    ReportSummary {
        anchor_pair: format!("({},{})", digit_symbol(ANCHOR_OUTER), digit_symbol(ANCHOR_INNER)),
        selected_same_base_neighbors,
        local_exact_rule_observed: best_rule_for_policy(local_rule_rows, HingeThresholdPolicy::Observed),
        local_exact_rule_min_side_2: best_rule_for_policy(
            local_rule_rows,
            HingeThresholdPolicy::ObservedMinSide2,
        ),
        local_mechanism_rule: best_mechanism_rule(local_rule_rows),
        main_takeaway: "Base 10 `(3,3)` looks less like an isolated miracle and more like a narrow persistence-only species witness: the clean local separator is diagnostic (`same_digit` plus a noncompact M2 winner), while the first more mechanism-shaped separator is carry-through plus nonpositive overlap margin.".to_string(),
    }
}

fn best_rule_for_policy(rows: &[LocalRuleRow], policy: HingeThresholdPolicy) -> String {
    rows.iter()
        .find(|row| row.threshold_policy == policy.as_str() && row.rank == 1)
        .map(|row| row.rule_label.clone())
        .unwrap_or_else(|| "none".to_string())
}

fn best_mechanism_rule(rows: &[LocalRuleRow]) -> String {
    rows.iter()
        .find(|row| {
            row.threshold_policy == HingeThresholdPolicy::Observed.as_str()
                && row.rule_label.contains("m1 anomaly_mass_pp")
                && row
                    .rule_label
                    .contains("m2 stable_zero_signal_margin_count")
        })
        .map(|row| row.rule_label.clone())
        .unwrap_or_else(|| "none".to_string())
}

fn derive_observations(
    entries: &[FocusEntry],
    comparison_rows: &[ComparisonRow],
    local_rule_rows: &[LocalRuleRow],
) -> Vec<String> {
    let anchor = entries
        .iter()
        .find(|entry| entry.role == "anchor_base10_persistence_only")
        .expect("anchor should exist");
    let same_digit_dead = entries
        .iter()
        .find(|entry| entry.role == "same_digit_dead_control")
        .expect("same-digit dead control should exist");
    let tiny_outgroup = entries
        .iter()
        .find(|entry| entry.role == "outgroup_tiny_persistence_only")
        .expect("tiny persistence-only outgroup should exist");
    let hinge_outgroup = entries
        .iter()
        .find(|entry| entry.role == "outgroup_hinge_bridge")
        .expect("hinge outgroup should exist");
    let same_lane = entries
        .iter()
        .find(|entry| entry.role == "active_same_lane_neighbor")
        .expect("active same-lane neighbor should exist");
    let local_rule_observed = best_rule_for_policy(local_rule_rows, HingeThresholdPolicy::Observed);
    let local_rule_min_side_2 =
        best_rule_for_policy(local_rule_rows, HingeThresholdPolicy::ObservedMinSide2);
    let strongest_m1_only = comparison_rows
        .iter()
        .find(|row| row.comparison_role == "m1_only_dead_control")
        .expect("m1-only control comparison should exist");

    vec![
        format!(
            "The anchor `{}` is the only base-10 row with positive anomaly at both `M=1` (`{:.2}pp`) and `M=2` (`{:.2}pp`), but its `M=2` stable-zero margin stays nonpositive (`{}`), so it survives as boundary-led persistence rather than as a hinge witness.",
            anchor.row.pair_label,
            anchor.row.m1_anomaly_mass_pp,
            anchor.row.m2_anomaly_mass_pp,
            anchor.row.m2_stable_zero_signal_margin_count,
        ),
        format!(
            "The strongest same-digit dead control is `{}`: it matches the anchor's same-digit geometry and `M=1` best-k, has even larger `M=1` anomaly (`{:.2}pp` vs `{:.2}pp`), but collapses to `0.00pp` at `M=2`.",
            same_digit_dead.row.pair_label,
            same_digit_dead.row.m1_anomaly_mass_pp,
            anchor.row.m1_anomaly_mass_pp,
        ),
        format!(
            "The strongest non-same-digit `M=1` dead control is `{}`: it stays alive at `M=1` (`{:.2}pp`) but dies completely at `M=2`, so persistence is not just a function of strong short-length carry-through.",
            strongest_m1_only.pair_label,
            strongest_m1_only.comparison_m1_anomaly_mass_pp,
        ),
        format!(
            "The nearest active same-lane neighbor is `{}`: it shares the anchor's winning `M=2` lane `{}`, but it has no `M=1` carry-through and flips to positive stable-zero margin (`{}`), so it looks like a nearby nonpersistent contrast rather than the same species.",
            same_lane.row.pair_label,
            same_lane.row.m2_best_k,
            same_lane.row.m2_stable_zero_signal_margin_count,
        ),
        format!(
            "The tiny outgroup `{}` in base `6` mirrors the species shape at smaller scale: persistent and boundary-led with `M=2` margin `{}`, while the hinge outgroup `{}` in base `14` separates cleanly by keeping positive overlap margin (`{}`).",
            tiny_outgroup.row.pair_label,
            tiny_outgroup.row.m2_stable_zero_signal_margin_count,
            hinge_outgroup.row.pair_label,
            hinge_outgroup.row.m2_stable_zero_signal_margin_count,
        ),
        format!(
            "The focused local rule search stays exact under both threshold policies. The minimal exact separator is diagnostic: observed `{}`; min-side-2 `{}`. The first more mechanism-shaped exact rule is `{}`.",
            local_rule_observed,
            local_rule_min_side_2,
            best_mechanism_rule(local_rule_rows),
        ),
    ]
}

fn render_neighborhood(base_rows: &[HingeFeatureRow], entries: &[FocusEntry], path: &Path) {
    let units = unit_residues(BASE);
    let unit_count = units.len();
    let focus_map = entries
        .iter()
        .map(|entry| {
            (
                (entry.row.base, entry.row.outer, entry.row.inner),
                entry.role.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();

    let root = BitMapBackend::new(path, (980, 920)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill neighborhood");
    let mut chart = ChartBuilder::on(&root)
        .caption("Base 10 Persistence Neighborhood", ("sans-serif", 28))
        .margin(30)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(0..unit_count, 0..unit_count)
        .expect("failed to build neighborhood");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(unit_count)
        .y_labels(unit_count)
        .x_label_formatter(&{
            let labels = units
                .iter()
                .map(|digit| digit_symbol(*digit))
                .collect::<Vec<_>>();
            move |value| labels.get(*value).cloned().unwrap_or_default()
        })
        .y_label_formatter(&{
            let labels = units
                .iter()
                .map(|digit| digit_symbol(*digit))
                .collect::<Vec<_>>();
            move |value| labels.get(*value).cloned().unwrap_or_default()
        })
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw neighborhood mesh");

    for row in base_rows {
        let x = units
            .iter()
            .position(|&digit| digit == row.outer)
            .expect("outer should be a unit");
        let y = units
            .iter()
            .position(|&digit| digit == row.inner)
            .expect("inner should be a unit");
        let fill = if focus_map.contains_key(&(row.base, row.outer, row.inner)) {
            focus_role_color(
                focus_map
                    .get(&(row.base, row.outer, row.inner))
                    .expect("focus role should exist"),
            )
        } else {
            hinge_category_color(&row.hinge_category)
        };
        chart
            .draw_series(std::iter::once(Circle::new(
                (x, y),
                if focus_map.contains_key(&(row.base, row.outer, row.inner)) {
                    8
                } else {
                    5
                },
                ShapeStyle::from(&fill).filled(),
            )))
            .expect("failed to draw neighborhood point");
        if let Some(role) = focus_map.get(&(row.base, row.outer, row.inner)) {
            chart
                .draw_series(std::iter::once(Text::new(
                    format!("{} {}", row.pair_label, short_role(role)),
                    (x, y),
                    ("sans-serif", 13).into_font().color(&BLACK),
                )))
                .expect("failed to draw focus label");
        }
    }

    root.present().expect("failed to present neighborhood");
}

fn render_species_plane(entries: &[FocusEntry], path: &Path) {
    let x_min = entries
        .iter()
        .map(|entry| entry.row.m1_anomaly_mass_pp)
        .fold(f64::INFINITY, f64::min)
        .min(0.0);
    let x_max = entries
        .iter()
        .map(|entry| entry.row.m1_anomaly_mass_pp)
        .fold(f64::NEG_INFINITY, f64::max)
        .max(1.0);
    let y_min = entries
        .iter()
        .map(|entry| entry.row.m2_stable_zero_signal_margin_count)
        .min()
        .unwrap_or(0)
        - 1;
    let y_max = entries
        .iter()
        .map(|entry| entry.row.m2_stable_zero_signal_margin_count)
        .max()
        .unwrap_or(0)
        + 1;

    let root = BitMapBackend::new(path, (1100, 840)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill species plane");
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Persistence Carry-Through vs M2 Overlap Margin",
            ("sans-serif", 28),
        )
        .margin(30)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d((x_min - 1.0)..(x_max + 2.0), y_min..y_max)
        .expect("failed to build species plane");

    chart
        .configure_mesh()
        .x_desc("M1 anomaly mass (pp)")
        .y_desc("M2 stable-zero signal margin (count)")
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw species plane mesh");

    for entry in entries {
        chart
            .draw_series(std::iter::once(Circle::new(
                (
                    entry.row.m1_anomaly_mass_pp,
                    entry.row.m2_stable_zero_signal_margin_count,
                ),
                (6.0 + entry.row.m2_anomaly_mass_pp.max(0.0)) as i32,
                ShapeStyle::from(&focus_role_color(&entry.role)).filled(),
            )))
            .expect("failed to draw species plane point");
        chart
            .draw_series(std::iter::once(Text::new(
                format!("{} {}", entry.row.pair_label, short_role(&entry.role)),
                (
                    entry.row.m1_anomaly_mass_pp + 0.4,
                    entry.row.m2_stable_zero_signal_margin_count,
                ),
                ("sans-serif", 14).into_font().color(&BLACK),
            )))
            .expect("failed to draw species plane label");
    }

    root.present().expect("failed to present species plane");
}

fn render_focus_strip(entries: &[FocusEntry], path: &Path) {
    let labels = entries
        .iter()
        .map(|entry| format!("{} {}", entry.row.pair_label, short_role(&entry.role)))
        .collect::<Vec<_>>();
    let x_max = entries
        .iter()
        .map(|entry| {
            entry
                .row
                .m1_anomaly_mass_pp
                .max(entry.row.m2_anomaly_mass_pp)
        })
        .fold(0.0, f64::max)
        + 5.0;

    let root = BitMapBackend::new(path, (1280, 900)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill focus strip");
    let mut chart = ChartBuilder::on(&root)
        .caption("Focused Persistence Species Strip", ("sans-serif", 28))
        .margin(28)
        .x_label_area_size(60)
        .y_label_area_size(220)
        .build_cartesian_2d(0.0..x_max, 0..entries.len())
        .expect("failed to build focus strip");

    chart
        .configure_mesh()
        .disable_y_mesh()
        .x_desc("Anomaly mass (pp)")
        .y_labels(entries.len())
        .y_label_formatter(&{ move |value| labels.get(*value).cloned().unwrap_or_default() })
        .label_style(("sans-serif", 14))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw focus strip mesh");

    for (index, entry) in entries.iter().enumerate() {
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(0.0, index), (entry.row.m1_anomaly_mass_pp, index + 1)],
                ShapeStyle::from(&RGBAColor(72, 125, 176, 0.55)).filled(),
            )))
            .expect("failed to draw m1 anomaly bar");
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(0.0, index), (entry.row.m2_anomaly_mass_pp, index + 1)],
                ShapeStyle::from(&RGBAColor(209, 122, 64, 0.8)).filled(),
            )))
            .expect("failed to draw m2 anomaly bar");
        chart
            .draw_series(std::iter::once(Text::new(
                format!(
                    "margin {} | boundary {}",
                    entry.row.m2_stable_zero_signal_margin_count,
                    entry.row.m2_boundary_prime_delta_count
                ),
                (
                    entry
                        .row
                        .m1_anomaly_mass_pp
                        .max(entry.row.m2_anomaly_mass_pp)
                        + 0.5,
                    index,
                ),
                ("sans-serif", 12).into_font().color(&BLACK),
            )))
            .expect("failed to draw focus strip annotation");
    }

    root.present().expect("failed to present focus strip");
}

fn render_markdown(bundle: &ReportBundle) -> String {
    let mut markdown = String::new();
    markdown.push_str("# Base 10 Persistence-Only Species Report\n\n");
    markdown.push_str("_Generated from `examples/base10_persistence_species_report.rs`._\n\n");
    markdown.push_str(&format!(
        "- Output directory: `{}`\n- Anchor: `{}`\n- Outgroups: `{}`\n- Max rule atoms: `{}`\n\n",
        bundle.settings.out_dir,
        bundle.settings.anchor_pair,
        bundle.settings.outgroups.join(", "),
        bundle.settings.max_rule_atoms,
    ));

    markdown.push_str("## Focus Set\n\n");
    markdown.push_str(
        "| Role | Base | Pair | Category | M1 anomaly | M2 anomaly | M2 margin | M2 boundary |\n",
    );
    markdown.push_str("|---|---:|---|---|---:|---:|---:|---:|\n");
    for row in &bundle.focus_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {:.2}pp | {:.2}pp | {} | {} |\n",
            row.role,
            row.base,
            row.pair_label,
            row.hinge_category,
            row.m1_anomaly_mass_pp,
            row.m2_anomaly_mass_pp,
            row.m2_stable_zero_signal_margin_count,
            row.m2_boundary_prime_delta_count,
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Local Rule Search\n\n");
    markdown.push_str("| Threshold policy | Rank | Status | Exact | Rule | tp/fp/fn |\n");
    markdown.push_str("|---|---:|---|---|---|---|\n");
    for row in &bundle.local_rule_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {}/{}/{} |\n",
            row.threshold_policy,
            row.rank,
            row.status,
            yes_no(row.exact_match),
            row.rule_label,
            row.true_positive,
            row.false_positive,
            row.false_negative,
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Anchor Comparisons\n\n");
    markdown.push_str("| Role | Pair | Same base | Dist | Same gap | Same m1 k | Same m2 k | ΔM1 | ΔM2 | Δmargin |\n");
    markdown.push_str("|---|---|---|---:|---|---|---|---:|---:|---:|\n");
    for row in &bundle.comparison_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {:.2}pp | {:.2}pp | {} |\n",
            row.comparison_role,
            row.pair_label,
            yes_no(row.same_base),
            row.pair_distance
                .map(|value| value.to_string())
                .unwrap_or_else(|| "—".to_string()),
            yes_no(row.same_gap_bucket),
            yes_no(row.same_m1_best_k),
            yes_no(row.same_m2_best_k),
            row.delta_m1_anomaly_mass_pp,
            row.delta_m2_anomaly_mass_pp,
            row.delta_m2_stable_zero_signal_margin_count,
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

fn pair_distance(
    base: u32,
    left_outer: u32,
    left_inner: u32,
    right_outer: u32,
    right_inner: u32,
) -> usize {
    digit_distance(base, left_outer, right_outer) + digit_distance(base, left_inner, right_inner)
}

fn digit_distance(base: u32, left: u32, right: u32) -> usize {
    let units = unit_residues(base);
    let left_index = units
        .iter()
        .position(|&digit| digit == left)
        .expect("left digit should be a unit");
    let right_index = units
        .iter()
        .position(|&digit| digit == right)
        .expect("right digit should be a unit");
    let direct = left_index.abs_diff(right_index);
    direct.min(units.len() - direct)
}

fn focus_role_color(role: &str) -> RGBColor {
    match role {
        "anchor_base10_persistence_only" => RGBColor(210, 97, 64),
        "same_digit_dead_control" => RGBColor(100, 119, 156),
        "m1_only_dead_control" => RGBColor(82, 147, 132),
        "active_same_lane_neighbor" => RGBColor(214, 170, 71),
        "active_other_lane_neighbor" => RGBColor(166, 116, 173),
        "outgroup_tiny_persistence_only" => RGBColor(210, 132, 64),
        "outgroup_hinge_bridge" => RGBColor(58, 134, 102),
        _ => RGBColor(120, 120, 120),
    }
}

fn hinge_category_color(category: &str) -> RGBColor {
    match category {
        HINGE_CATEGORY_PERSISTENCE_ONLY => RGBColor(214, 132, 65),
        "persistent_core" => RGBColor(61, 135, 101),
        "core_only" => RGBColor(83, 118, 178),
        _ => RGBColor(175, 175, 175),
    }
}

fn short_role(role: &str) -> &'static str {
    match role {
        "anchor_base10_persistence_only" => "anchor",
        "same_digit_dead_control" => "same-digit",
        "m1_only_dead_control" => "m1-dead",
        "active_same_lane_neighbor" => "same-lane",
        "active_other_lane_neighbor" => "other-lane",
        "outgroup_tiny_persistence_only" => "tiny-persist",
        "outgroup_hinge_bridge" => "hinge",
        _ => "focus",
    }
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_selection_keeps_expected_anchor_neighbors_and_outgroups() {
        let anchor = analyze_hinge_feature_row(BASE, ANCHOR_OUTER, ANCHOR_INNER);
        let base_rows = build_hinge_feature_rows(&[BASE]);
        let neighbor_rows = build_neighbor_rows(&anchor, &base_rows);
        let focus_entries = build_focus_entries(&anchor, &base_rows, &neighbor_rows);

        assert!(focus_entries
            .iter()
            .any(|entry| entry.role == "anchor_base10_persistence_only"
                && entry.row.pair_label == "(3,3)"));
        assert!(focus_entries.iter().any(
            |entry| entry.role == "same_digit_dead_control" && entry.row.pair_label == "(9,9)"
        ));
        assert!(focus_entries
            .iter()
            .any(|entry| entry.role == "active_same_lane_neighbor"
                && entry.row.pair_label == "(7,3)"));
        assert!(focus_entries
            .iter()
            .any(|entry| entry.role == "active_other_lane_neighbor"
                && entry.row.pair_label == "(3,1)"));
        assert!(focus_entries
            .iter()
            .any(|entry| entry.role == "outgroup_tiny_persistence_only"
                && entry.row.base == 6
                && entry.row.pair_label == "(5,5)"));
        assert!(focus_entries
            .iter()
            .any(|entry| entry.role == "outgroup_hinge_bridge"
                && entry.row.base == 14
                && entry.row.pair_label == "(D,B)"));
    }

    #[test]
    fn focus_roles_keep_expected_species_labels() {
        let anchor = analyze_hinge_feature_row(BASE, ANCHOR_OUTER, ANCHOR_INNER);
        let tiny = analyze_hinge_feature_row(6, 5, 5);
        let hinge = analyze_hinge_feature_row(14, 13, 11);

        assert_eq!(anchor.hinge_category, "persistence_only");
        assert_eq!(tiny.hinge_category, "persistence_only");
        assert_eq!(hinge.hinge_category, "persistent_core");
    }

    #[test]
    fn local_rule_search_keeps_exact_rule_under_both_threshold_policies() {
        let anchor = analyze_hinge_feature_row(BASE, ANCHOR_OUTER, ANCHOR_INNER);
        let base_rows = build_hinge_feature_rows(&[BASE]);
        let neighbor_rows = build_neighbor_rows(&anchor, &base_rows);
        let focus_entries = build_focus_entries(&anchor, &base_rows, &neighbor_rows);
        let local_rule_rows = build_local_rule_rows(&focus_entries);

        let observed = local_rule_rows
            .iter()
            .find(|row| {
                row.threshold_policy == HingeThresholdPolicy::Observed.as_str() && row.rank == 1
            })
            .expect("observed local rule should exist");
        let min_side_2 = local_rule_rows
            .iter()
            .find(|row| {
                row.threshold_policy == HingeThresholdPolicy::ObservedMinSide2.as_str()
                    && row.rank == 1
            })
            .expect("min-side-2 local rule should exist");

        assert!(observed.exact_match);
        assert!(min_side_2.exact_match);
    }
}
