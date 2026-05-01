//! Focused species report for the base-34 boundary-led lane.
//!
//! Base 34 does not look like a new hinge bridge at the base-wide level, but
//! it does have a few `M=2` pockets. This report asks whether those pockets
//! form a coherent non-hinge species rather than merely failing the base-14
//! pattern.
//!
//! The species candidate is deliberately narrow:
//! - base `34`
//! - `M=2`
//! - compare `k=(1,0)` against `k=(0,0)`
//! - active pair means positive `k=(1,0)` anomaly
//! - boundary-led means the gain/loss-zero lanes dominate the stable-zero lane
//!
//! The report then compares the active pairs against nearby same-base dead
//! pairs under the exact same `k=(1,0)` lane.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example base34_boundary_species_report
//! cargo run --release --example base34_boundary_species_report -- --out-dir /tmp/primes_base34_boundary_species
//! ```

use plotters::prelude::*;
use primes::validation::{
    bounded_k::{
        digit_symbol, evaluate_pair_row, ordered_unit_pairs, scan_k_config_mask_profile,
        scan_k_config_transfer_profile, unit_residues, KDominancePairRow, DEFAULT_BOUNDED_K_GRID,
    },
    reporting::{
        ensure_dir, export_timestamp_utc, write_csv_rows, write_json_pretty, write_text_file,
    },
};
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    path::{Path, PathBuf},
};

const BASE: u32 = 34;
const M1: usize = 1;
const M2: usize = 2;
const LANE_K: (u32, u32) = (1, 0);
const LANE_K_LABEL: &str = "k=(1,0)";
const DEFAULT_OUT_DIR: &str = "/tmp/primes_base34_boundary_species";
const REPORT_EXPORT_VERSION: u32 = 1;
const NEARBY_CONTROL_LIMIT: usize = 3;

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    nearby_control_limit: usize,
}

#[derive(Debug, Clone)]
struct BoundaryScan {
    outer: u32,
    inner: u32,
    pair_label: String,
    same_digit: bool,
    unit_index_outer: usize,
    unit_index_inner: usize,
    gap_bucket: String,
    row_m1: KDominancePairRow,
    row_m2: KDominancePairRow,
    boundary_class: String,
    lane_metrics: LaneMetrics,
}

#[derive(Debug, Clone)]
struct LaneMetrics {
    lane_minus_k00_pp: f64,
    admissible_delta_pp: f64,
    shared_admissible_count: usize,
    shared_prime_rate_delta_pp: f64,
    stable_zero_prime_delta_pp: f64,
    boundary_prime_delta_pp: f64,
    gain_zero_count: usize,
    loss_zero_count: usize,
    zero_mask_net_transfer_pp: f64,
    signal_source_label: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    base: u32,
    middle_lengths: Vec<usize>,
    lane_k: String,
    nearby_control_limit: usize,
}

#[derive(Debug, Clone, Serialize)]
struct SpeciesRow {
    role: String,
    anchor_active_pair: String,
    control_rank: usize,
    pair_label: String,
    outer: u32,
    inner: u32,
    gap_bucket: String,
    same_digit: bool,
    unit_index_outer: usize,
    unit_index_inner: usize,
    best_k_m1: String,
    best_k_m2: String,
    boundary_class: String,
    m1_anomalous: bool,
    m2_active: bool,
    m2_emergent: bool,
    lane_is_best_at_m2: bool,
    boundary_led: bool,
    pair_distance_from_anchor: usize,
    lane_minus_k00_pp: f64,
    stable_zero_prime_delta_pp: f64,
    boundary_prime_delta_pp: f64,
    admissible_delta_pp: f64,
    shared_prime_rate_delta_pp: f64,
    zero_mask_net_transfer_pp: f64,
    gain_zero_count: usize,
    loss_zero_count: usize,
    shared_admissible_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct ComparisonRow {
    active_pair: String,
    control_pair: String,
    control_rank: usize,
    same_gap_bucket: bool,
    same_same_digit: bool,
    same_best_k_m1: bool,
    pair_distance: usize,
    active_boundary_class: String,
    control_boundary_class: String,
    active_lane_minus_k00_pp: f64,
    control_lane_minus_k00_pp: f64,
    active_boundary_prime_delta_pp: f64,
    control_boundary_prime_delta_pp: f64,
    active_stable_zero_prime_delta_pp: f64,
    control_stable_zero_prime_delta_pp: f64,
    active_admissible_delta_pp: f64,
    control_admissible_delta_pp: f64,
    active_shared_prime_rate_delta_pp: f64,
    control_shared_prime_rate_delta_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct RoleSummaryRow {
    role: String,
    pair_count: usize,
    same_gap_pairs: usize,
    adjacent_gap_pairs: usize,
    wide_gap_pairs: usize,
    same_digit_pairs: usize,
    mean_lane_minus_k00_pp: Option<f64>,
    mean_boundary_prime_delta_pp: Option<f64>,
    mean_stable_zero_prime_delta_pp: Option<f64>,
    mean_admissible_delta_pp: Option<f64>,
    mean_shared_prime_rate_delta_pp: Option<f64>,
    mean_zero_mask_net_transfer_pp: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    active_species_pairs: usize,
    nearby_dead_controls: usize,
    main_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    species_rows: Vec<SpeciesRow>,
    comparison_rows: Vec<ComparisonRow>,
    role_summary_rows: Vec<RoleSummaryRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        base: BASE,
        middle_lengths: vec![M1, M2],
        lane_k: LANE_K_LABEL.to_string(),
        nearby_control_limit: options.nearby_control_limit,
    };

    let scans = build_boundary_scans();
    let actives = scans
        .iter()
        .filter(|scan| is_species_active(scan))
        .collect::<Vec<_>>();
    let dead_controls = scans
        .iter()
        .filter(|scan| anomaly_mass(&scan.row_m2) == 0.0)
        .collect::<Vec<_>>();

    let (species_rows, comparison_rows) =
        build_species_rows(&actives, &dead_controls, options.nearby_control_limit);
    let role_summary_rows = build_role_summary_rows(&species_rows);

    let lattice_path = options.out_dir.join("base34_species_lattice.png");
    render_species_lattice(&scans, &species_rows, &lattice_path);
    let plane_path = options.out_dir.join("base34_boundary_species_plane.png");
    render_species_plane(&species_rows, &plane_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "species_lattice".to_string(),
            label: "Base 34 species lattice".to_string(),
            path: lattice_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "species_plane".to_string(),
            label: "Base 34 boundary-species plane".to_string(),
            path: plane_path.display().to_string(),
        },
    ];

    let report_summary = build_report_summary(&species_rows);
    let observations = derive_observations(&species_rows, &comparison_rows, &role_summary_rows);

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        species_rows: species_rows.clone(),
        comparison_rows: comparison_rows.clone(),
        role_summary_rows: role_summary_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary,
        observations,
    };

    write_csv_rows(options.out_dir.join("species_rows.csv"), &species_rows)
        .expect("failed to write species_rows.csv");
    write_csv_rows(
        options.out_dir.join("comparison_rows.csv"),
        &comparison_rows,
    )
    .expect("failed to write comparison_rows.csv");
    write_csv_rows(
        options.out_dir.join("role_summary_rows.csv"),
        &role_summary_rows,
    )
    .expect("failed to write role_summary_rows.csv");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle)
        .expect("failed to write summary.json");
    write_text_file(options.out_dir.join("report.md"), &render_markdown(&bundle))
        .expect("failed to write report.md");

    println!("base34 boundary species report");
    println!("  output dir: {}", options.out_dir.display());
    for row in &role_summary_rows {
        println!(
            "  {:<22} | n {:>2} | lane {} | boundary {} | stable_zero {}",
            row.role,
            row.pair_count,
            format_option_float(row.mean_lane_minus_k00_pp),
            format_option_float(row.mean_boundary_prime_delta_pp),
            format_option_float(row.mean_stable_zero_prime_delta_pp),
        );
    }
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut nearby_control_limit = NEARBY_CONTROL_LIMIT;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => {
                let value = args.next().expect("--out-dir requires a directory");
                out_dir = PathBuf::from(value);
            }
            "--nearby-control-limit" => {
                let value = args
                    .next()
                    .expect("--nearby-control-limit requires a numeric argument");
                nearby_control_limit = value
                    .parse()
                    .expect("--nearby-control-limit must be a positive integer");
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => panic!("unrecognized argument: {arg}"),
        }
    }

    Options {
        out_dir,
        nearby_control_limit,
    }
}

fn print_help() {
    println!("Usage:");
    println!("  cargo run --release --example base34_boundary_species_report -- [options]");
    println!();
    println!("Options:");
    println!("  --out-dir <dir>                 Output directory (default: {DEFAULT_OUT_DIR})");
    println!("  --nearby-control-limit <n>      Controls per active pair (default: {NEARBY_CONTROL_LIMIT})");
    println!("  -h, --help                      Show this help");
}

fn build_boundary_scans() -> Vec<BoundaryScan> {
    let mut scans = ordered_unit_pairs(BASE)
        .into_iter()
        .map(|(outer, inner)| build_boundary_scan(outer, inner))
        .collect::<Vec<_>>();
    scans.sort_by(|left, right| {
        left.outer
            .cmp(&right.outer)
            .then_with(|| left.inner.cmp(&right.inner))
    });
    scans
}

fn build_boundary_scan(outer: u32, inner: u32) -> BoundaryScan {
    let row_m1 = evaluate_pair_row(BASE, M1, outer, inner, DEFAULT_BOUNDED_K_GRID);
    let row_m2 = evaluate_pair_row(BASE, M2, outer, inner, DEFAULT_BOUNDED_K_GRID);
    let units = unit_residues(BASE);
    let unit_index_outer = units
        .iter()
        .position(|&digit| digit == outer)
        .expect("outer digit should be a unit");
    let unit_index_inner = units
        .iter()
        .position(|&digit| digit == inner)
        .expect("inner digit should be a unit");

    BoundaryScan {
        outer,
        inner,
        pair_label: format!("({},{})", digit_symbol(outer), digit_symbol(inner)),
        same_digit: outer == inner,
        unit_index_outer,
        unit_index_inner,
        gap_bucket: gap_bucket(unit_index_outer.abs_diff(unit_index_inner), units.len())
            .to_string(),
        boundary_class: boundary_class(anomaly_mass(&row_m1) > 0.0, anomaly_mass(&row_m2) > 0.0)
            .to_string(),
        row_m1,
        row_m2,
        lane_metrics: lane_metrics(outer, inner),
    }
}

fn lane_metrics(outer: u32, inner: u32) -> LaneMetrics {
    let k00_profile = scan_k_config_mask_profile(BASE, M2, outer, inner, (0, 0));
    let lane_profile = scan_k_config_mask_profile(BASE, M2, outer, inner, LANE_K);
    let transfer_profile = scan_k_config_transfer_profile(BASE, M2, outer, inner, (0, 0), LANE_K);

    let mut shared_admissible_count = 0usize;
    let mut shared_prime_hits_k00 = 0usize;
    let mut shared_prime_hits_lane = 0usize;
    let mut stable_zero_prime_delta_count = 0isize;
    let mut boundary_prime_delta_count = 0isize;
    let mut gain_zero_count = 0usize;
    let mut loss_zero_count = 0usize;

    for row in &transfer_profile.candidate_rows {
        match (row.admissible_from, row.admissible_to) {
            (true, true) => {
                shared_admissible_count += 1;
                if row.prime_from {
                    shared_prime_hits_k00 += 1;
                    stable_zero_prime_delta_count -= 1;
                }
                if row.prime_to {
                    shared_prime_hits_lane += 1;
                    stable_zero_prime_delta_count += 1;
                }
            }
            (false, true) => {
                gain_zero_count += 1;
                if row.prime_to {
                    boundary_prime_delta_count += 1;
                }
            }
            (true, false) => {
                loss_zero_count += 1;
                if row.prime_from {
                    boundary_prime_delta_count -= 1;
                }
            }
            (false, false) => {}
        }
    }

    let candidate_count = transfer_profile.candidates_per_config;
    let lane_minus_k00_pp = count_delta_pp(
        lane_profile.prime_hits,
        k00_profile.prime_hits,
        candidate_count,
    );
    let admissible_delta_pp = count_delta_pp(
        lane_profile.admissible_count,
        k00_profile.admissible_count,
        candidate_count,
    );
    let stable_zero_prime_delta_pp =
        stable_zero_prime_delta_count as f64 * 100.0 / candidate_count as f64;
    let boundary_prime_delta_pp =
        boundary_prime_delta_count as f64 * 100.0 / candidate_count as f64;

    LaneMetrics {
        lane_minus_k00_pp,
        admissible_delta_pp,
        shared_admissible_count,
        shared_prime_rate_delta_pp: (ratio(shared_prime_hits_lane, shared_admissible_count)
            - ratio(shared_prime_hits_k00, shared_admissible_count))
            * 100.0,
        stable_zero_prime_delta_pp,
        boundary_prime_delta_pp,
        gain_zero_count,
        loss_zero_count,
        zero_mask_net_transfer_pp: (gain_zero_count as f64 - loss_zero_count as f64) * 100.0
            / candidate_count as f64,
        signal_source_label: signal_source_label(
            stable_zero_prime_delta_pp,
            boundary_prime_delta_pp,
        )
        .to_string(),
    }
}

fn is_species_active(scan: &BoundaryScan) -> bool {
    anomaly_mass(&scan.row_m1) == 0.0
        && scan.row_m2.best_k == LANE_K_LABEL
        && scan.lane_metrics.lane_minus_k00_pp > 0.0
        && scan.lane_metrics.signal_source_label == "boundary_led"
}

fn build_species_rows(
    actives: &[&BoundaryScan],
    dead_controls: &[&BoundaryScan],
    limit: usize,
) -> (Vec<SpeciesRow>, Vec<ComparisonRow>) {
    let mut species_rows = Vec::new();
    let mut comparison_rows = Vec::new();
    let mut selected_controls = BTreeSet::<(u32, u32)>::new();

    for active in actives {
        species_rows.push(species_row_from_scan(
            active,
            "species_active",
            &active.pair_label,
            0,
            0,
        ));

        for (control_rank, control) in nearest_dead_controls(active, dead_controls, limit)
            .into_iter()
            .enumerate()
        {
            selected_controls.insert((control.outer, control.inner));
            species_rows.push(species_row_from_scan(
                control,
                "nearby_dead_control",
                &active.pair_label,
                control_rank + 1,
                pair_distance(active, control),
            ));

            comparison_rows.push(ComparisonRow {
                active_pair: active.pair_label.clone(),
                control_pair: control.pair_label.clone(),
                control_rank: control_rank + 1,
                same_gap_bucket: active.gap_bucket == control.gap_bucket,
                same_same_digit: active.same_digit == control.same_digit,
                same_best_k_m1: active.row_m1.best_k == control.row_m1.best_k,
                pair_distance: pair_distance(active, control),
                active_boundary_class: active.boundary_class.clone(),
                control_boundary_class: control.boundary_class.clone(),
                active_lane_minus_k00_pp: active.lane_metrics.lane_minus_k00_pp,
                control_lane_minus_k00_pp: control.lane_metrics.lane_minus_k00_pp,
                active_boundary_prime_delta_pp: active.lane_metrics.boundary_prime_delta_pp,
                control_boundary_prime_delta_pp: control.lane_metrics.boundary_prime_delta_pp,
                active_stable_zero_prime_delta_pp: active.lane_metrics.stable_zero_prime_delta_pp,
                control_stable_zero_prime_delta_pp: control.lane_metrics.stable_zero_prime_delta_pp,
                active_admissible_delta_pp: active.lane_metrics.admissible_delta_pp,
                control_admissible_delta_pp: control.lane_metrics.admissible_delta_pp,
                active_shared_prime_rate_delta_pp: active.lane_metrics.shared_prime_rate_delta_pp,
                control_shared_prime_rate_delta_pp: control.lane_metrics.shared_prime_rate_delta_pp,
            });
        }
    }

    for control in dead_controls {
        if !selected_controls.contains(&(control.outer, control.inner)) {
            species_rows.push(species_row_from_scan(
                control,
                "other_dead",
                "-",
                0,
                usize::MAX,
            ));
        }
    }

    species_rows.sort_by(|left, right| {
        role_rank(&left.role)
            .cmp(&role_rank(&right.role))
            .then_with(|| right.lane_minus_k00_pp.total_cmp(&left.lane_minus_k00_pp))
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });
    comparison_rows.sort_by(|left, right| {
        left.active_pair
            .cmp(&right.active_pair)
            .then_with(|| left.control_rank.cmp(&right.control_rank))
            .then_with(|| left.control_pair.cmp(&right.control_pair))
    });

    (species_rows, comparison_rows)
}

fn species_row_from_scan(
    scan: &BoundaryScan,
    role: &str,
    anchor_active_pair: &str,
    control_rank: usize,
    pair_distance_from_anchor: usize,
) -> SpeciesRow {
    SpeciesRow {
        role: role.to_string(),
        anchor_active_pair: anchor_active_pair.to_string(),
        control_rank,
        pair_label: scan.pair_label.clone(),
        outer: scan.outer,
        inner: scan.inner,
        gap_bucket: scan.gap_bucket.clone(),
        same_digit: scan.same_digit,
        unit_index_outer: scan.unit_index_outer,
        unit_index_inner: scan.unit_index_inner,
        best_k_m1: scan.row_m1.best_k.clone(),
        best_k_m2: scan.row_m2.best_k.clone(),
        boundary_class: scan.boundary_class.clone(),
        m1_anomalous: anomaly_mass(&scan.row_m1) > 0.0,
        m2_active: anomaly_mass(&scan.row_m2) > 0.0,
        m2_emergent: anomaly_mass(&scan.row_m1) == 0.0 && anomaly_mass(&scan.row_m2) > 0.0,
        lane_is_best_at_m2: scan.row_m2.best_k == LANE_K_LABEL,
        boundary_led: scan.lane_metrics.signal_source_label == "boundary_led",
        pair_distance_from_anchor,
        lane_minus_k00_pp: scan.lane_metrics.lane_minus_k00_pp,
        stable_zero_prime_delta_pp: scan.lane_metrics.stable_zero_prime_delta_pp,
        boundary_prime_delta_pp: scan.lane_metrics.boundary_prime_delta_pp,
        admissible_delta_pp: scan.lane_metrics.admissible_delta_pp,
        shared_prime_rate_delta_pp: scan.lane_metrics.shared_prime_rate_delta_pp,
        zero_mask_net_transfer_pp: scan.lane_metrics.zero_mask_net_transfer_pp,
        gain_zero_count: scan.lane_metrics.gain_zero_count,
        loss_zero_count: scan.lane_metrics.loss_zero_count,
        shared_admissible_count: scan.lane_metrics.shared_admissible_count,
    }
}

fn nearest_dead_controls<'a>(
    active: &BoundaryScan,
    dead_controls: &'a [&BoundaryScan],
    limit: usize,
) -> Vec<&'a BoundaryScan> {
    let mut same_base = dead_controls.to_vec();
    same_base.sort_by(|left, right| {
        control_rank_tuple(active, left).cmp(&control_rank_tuple(active, right))
    });
    same_base.truncate(limit);
    same_base
}

fn control_rank_tuple(
    active: &BoundaryScan,
    control: &BoundaryScan,
) -> (usize, usize, usize, usize, String) {
    (
        usize::from(active.gap_bucket != control.gap_bucket),
        usize::from(active.same_digit != control.same_digit),
        usize::from(active.row_m1.best_k != control.row_m1.best_k),
        pair_distance(active, control),
        control.pair_label.clone(),
    )
}

fn pair_distance(left: &BoundaryScan, right: &BoundaryScan) -> usize {
    left.unit_index_outer.abs_diff(right.unit_index_outer)
        + left.unit_index_inner.abs_diff(right.unit_index_inner)
}

fn build_role_summary_rows(rows: &[SpeciesRow]) -> Vec<RoleSummaryRow> {
    let mut by_role = BTreeMap::<String, Vec<&SpeciesRow>>::new();
    for row in rows {
        by_role.entry(row.role.clone()).or_default().push(row);
    }

    by_role
        .into_iter()
        .map(|(role, group)| RoleSummaryRow {
            role,
            pair_count: group.len(),
            same_gap_pairs: group.iter().filter(|row| row.gap_bucket == "same").count(),
            adjacent_gap_pairs: group
                .iter()
                .filter(|row| row.gap_bucket == "adjacent")
                .count(),
            wide_gap_pairs: group.iter().filter(|row| row.gap_bucket == "wide").count(),
            same_digit_pairs: group.iter().filter(|row| row.same_digit).count(),
            mean_lane_minus_k00_pp: mean(
                &group
                    .iter()
                    .map(|row| row.lane_minus_k00_pp)
                    .collect::<Vec<_>>(),
            ),
            mean_boundary_prime_delta_pp: mean(
                &group
                    .iter()
                    .map(|row| row.boundary_prime_delta_pp)
                    .collect::<Vec<_>>(),
            ),
            mean_stable_zero_prime_delta_pp: mean(
                &group
                    .iter()
                    .map(|row| row.stable_zero_prime_delta_pp)
                    .collect::<Vec<_>>(),
            ),
            mean_admissible_delta_pp: mean(
                &group
                    .iter()
                    .map(|row| row.admissible_delta_pp)
                    .collect::<Vec<_>>(),
            ),
            mean_shared_prime_rate_delta_pp: mean(
                &group
                    .iter()
                    .map(|row| row.shared_prime_rate_delta_pp)
                    .collect::<Vec<_>>(),
            ),
            mean_zero_mask_net_transfer_pp: mean(
                &group
                    .iter()
                    .map(|row| row.zero_mask_net_transfer_pp)
                    .collect::<Vec<_>>(),
            ),
        })
        .collect()
}

fn render_species_lattice(scans: &[BoundaryScan], species_rows: &[SpeciesRow], path: &Path) {
    let units = unit_residues(BASE);
    let root = BitMapBackend::new(path, (960, 960)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill species lattice canvas");
    let mut chart = ChartBuilder::on(&root)
        .caption("Base 34 Boundary Species Lattice", ("sans-serif", 28))
        .margin(24)
        .x_label_area_size(64)
        .y_label_area_size(64)
        .build_cartesian_2d(0..units.len() as i32, 0..units.len() as i32)
        .expect("failed to build species lattice");

    chart
        .configure_mesh()
        .x_desc("outer unit index")
        .y_desc("inner unit index")
        .label_style(("sans-serif", 14))
        .axis_style(RGBColor(92, 86, 78))
        .light_line_style(RGBColor(222, 216, 207))
        .x_labels(units.len())
        .y_labels(units.len())
        .x_label_formatter(&|index| digit_symbol(units[(*index as usize).min(units.len() - 1)]))
        .y_label_formatter(&|index| digit_symbol(units[(*index as usize).min(units.len() - 1)]))
        .draw()
        .expect("failed to draw species lattice mesh");

    let role_lookup = species_rows
        .iter()
        .map(|row| ((row.outer, row.inner), row.role.clone()))
        .collect::<BTreeMap<_, _>>();

    for scan in scans {
        let color = match role_lookup
            .get(&(scan.outer, scan.inner))
            .map(String::as_str)
            .unwrap_or("other_dead")
        {
            "species_active" => RGBColor(196, 94, 49),
            "nearby_dead_control" => RGBColor(60, 110, 113),
            _ => RGBColor(210, 206, 198),
        };
        chart
            .draw_series(std::iter::once(Circle::new(
                (scan.unit_index_outer as i32, scan.unit_index_inner as i32),
                6,
                ShapeStyle::from(&color).filled(),
            )))
            .expect("failed to draw species lattice point");
    }

    for row in species_rows.iter().filter(|row| row.role != "other_dead") {
        chart
            .draw_series(std::iter::once(Text::new(
                row.pair_label.clone(),
                (
                    row.unit_index_outer as i32 + 1,
                    row.unit_index_inner as i32 + 1,
                ),
                ("sans-serif", 15).into_font().color(&BLACK),
            )))
            .expect("failed to draw species lattice label");
    }

    root.present().expect("failed to present species lattice");
}

fn render_species_plane(rows: &[SpeciesRow], path: &Path) {
    let focus_rows = rows
        .iter()
        .filter(|row| row.role != "other_dead")
        .collect::<Vec<_>>();
    let root = BitMapBackend::new(path, (1120, 760)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill species plane canvas");

    let x_min = focus_rows
        .iter()
        .map(|row| row.boundary_prime_delta_pp)
        .fold(0.0, f64::min)
        - 0.3;
    let x_max = focus_rows
        .iter()
        .map(|row| row.boundary_prime_delta_pp)
        .fold(0.0, f64::max)
        + 0.3;
    let y_min = focus_rows
        .iter()
        .map(|row| row.lane_minus_k00_pp)
        .fold(0.0, f64::min)
        - 0.3;
    let y_max = focus_rows
        .iter()
        .map(|row| row.lane_minus_k00_pp)
        .fold(0.0, f64::max)
        + 0.3;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Base 34 Boundary Species Plane  (x = boundary delta, y = k=(1,0) anomaly)",
            ("sans-serif", 28),
        )
        .margin(24)
        .x_label_area_size(72)
        .y_label_area_size(84)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .expect("failed to build species plane");

    chart
        .configure_mesh()
        .x_desc("boundary prime delta (pp)")
        .y_desc("k=(1,0) minus k=(0,0) (pp)")
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .light_line_style(RGBColor(222, 216, 207))
        .draw()
        .expect("failed to draw species plane mesh");

    for row in focus_rows {
        let color = match row.role.as_str() {
            "species_active" => RGBColor(196, 94, 49),
            "nearby_dead_control" => RGBColor(60, 110, 113),
            _ => RGBColor(127, 127, 127),
        };
        chart
            .draw_series(std::iter::once(Circle::new(
                (row.boundary_prime_delta_pp, row.lane_minus_k00_pp),
                8,
                ShapeStyle::from(&color).filled(),
            )))
            .expect("failed to draw species plane point");
        chart
            .draw_series(std::iter::once(Text::new(
                row.pair_label.clone(),
                (
                    row.boundary_prime_delta_pp + 0.05,
                    row.lane_minus_k00_pp + 0.05,
                ),
                ("sans-serif", 15).into_font().color(&BLACK),
            )))
            .expect("failed to draw species plane label");
    }

    root.present().expect("failed to present species plane");
}

fn build_report_summary(rows: &[SpeciesRow]) -> ReportSummary {
    let active_species_pairs = rows
        .iter()
        .filter(|row| row.role == "species_active")
        .count();
    let nearby_dead_controls = rows
        .iter()
        .filter(|row| row.role == "nearby_dead_control")
        .count();
    ReportSummary {
        active_species_pairs,
        nearby_dead_controls,
        main_takeaway: format!(
            "Base 34 shows {} active `k=(1,0)` boundary-led pockets against {} nearby dead controls under the same lane.",
            active_species_pairs, nearby_dead_controls
        ),
    }
}

fn derive_observations(
    rows: &[SpeciesRow],
    comparisons: &[ComparisonRow],
    summaries: &[RoleSummaryRow],
) -> Vec<String> {
    let active_summary = summaries
        .iter()
        .find(|row| row.role == "species_active")
        .expect("species_active summary should exist");
    let control_summary = summaries
        .iter()
        .find(|row| row.role == "nearby_dead_control")
        .expect("nearby_dead_control summary should exist");
    let strongest_active = rows
        .iter()
        .filter(|row| row.role == "species_active")
        .max_by(|left, right| left.lane_minus_k00_pp.total_cmp(&right.lane_minus_k00_pp))
        .expect("at least one active species row should exist");
    let best_control_match = comparisons
        .iter()
        .find(|row| row.active_pair == strongest_active.pair_label && row.control_rank == 1)
        .expect("rank-1 control should exist for strongest active");

    vec![
        format!(
            "All active base-34 pockets are `m2_only`, all pick `{}`, and all are boundary-led rather than shared-overlap-led.",
            LANE_K_LABEL
        ),
        format!(
            "The species shape is narrow but coherent: the active set has mean lane anomaly {}, mean boundary delta {}, and mean stable-zero delta {}, while the nearby dead controls sit at {}, {}, and {}.",
            format_option_float(active_summary.mean_lane_minus_k00_pp),
            format_option_float(active_summary.mean_boundary_prime_delta_pp),
            format_option_float(active_summary.mean_stable_zero_prime_delta_pp),
            format_option_float(control_summary.mean_lane_minus_k00_pp),
            format_option_float(control_summary.mean_boundary_prime_delta_pp),
            format_option_float(control_summary.mean_stable_zero_prime_delta_pp),
        ),
        format!(
            "The gap geometry leans wide: {}/{} active pairs are wide-gap, with the remaining active pair adjacent.",
            active_summary.wide_gap_pairs, active_summary.pair_count
        ),
        format!(
            "The strongest witness is {} at {:.2}pp, and its nearest dead control {} drops to {:.2}pp under the same `{}` lane.",
            strongest_active.pair_label,
            strongest_active.lane_minus_k00_pp,
            best_control_match.control_pair,
            best_control_match.control_lane_minus_k00_pp,
            LANE_K_LABEL
        ),
        "That points to a second non-hinge species candidate: emergent boundary release in the `k=(1,0)` lane, rather than persistent shared-overlap lift.".to_string(),
    ]
}

fn render_markdown(bundle: &ReportBundle) -> String {
    let mut markdown = String::new();
    markdown.push_str("# Base 34 Boundary Species\n\n");
    markdown.push_str("_Generated from `examples/base34_boundary_species_report.rs`._\n\n");
    markdown.push_str(
        "This report asks whether base `34` contains a coherent non-hinge species: emergent `M=2` wins in the `k=(1,0)` lane driven by boundary transfer rather than by the shared-overlap mechanism that makes base `14` interesting.\n\n",
    );

    markdown.push_str("## Summary\n\n");
    markdown.push_str(&format!(
        "- Main takeaway: {}\n",
        bundle.report_summary.main_takeaway
    ));
    markdown.push_str(&format!(
        "- Visuals: ![Species lattice]({}) and ![Species plane]({})\n\n",
        bundle.image_artifact_rows[0].path, bundle.image_artifact_rows[1].path
    ));

    markdown.push_str("## Role Summary\n\n");
    markdown.push_str("| Role | Pairs | Wide | Adjacent | Same | Mean lane anomaly | Mean boundary delta | Mean stable-zero delta |\n");
    markdown.push_str("|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in &bundle.role_summary_rows {
        markdown.push_str(&format!(
            "| `{}` | {} | {} | {} | {} | {} | {} | {} |\n",
            row.role,
            row.pair_count,
            row.wide_gap_pairs,
            row.adjacent_gap_pairs,
            row.same_gap_pairs,
            format_option_float(row.mean_lane_minus_k00_pp),
            format_option_float(row.mean_boundary_prime_delta_pp),
            format_option_float(row.mean_stable_zero_prime_delta_pp),
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Active Species Pairs\n\n");
    markdown.push_str("| Pair | Gap | Best k at M=2 | Lane anomaly | Boundary delta | Stable-zero delta | Shared rate delta |\n");
    markdown.push_str("|---|---|---|---:|---:|---:|---:|\n");
    for row in bundle
        .species_rows
        .iter()
        .filter(|row| row.role == "species_active")
    {
        markdown.push_str(&format!(
            "| `{}` | `{}` | `{}` | {:.2}pp | {:.2}pp | {:.2}pp | {:.2}pp |\n",
            row.pair_label,
            row.gap_bucket,
            row.best_k_m2,
            row.lane_minus_k00_pp,
            row.boundary_prime_delta_pp,
            row.stable_zero_prime_delta_pp,
            row.shared_prime_rate_delta_pp,
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Nearby Dead Controls\n\n");
    markdown.push_str("| Active pair | Control | Rank | Gap match | Pair distance | Control lane anomaly | Control boundary delta |\n");
    markdown.push_str("|---|---|---:|---|---:|---:|---:|\n");
    for row in &bundle.comparison_rows {
        markdown.push_str(&format!(
            "| `{}` | `{}` | {} | {} | {} | {:.2}pp | {:.2}pp |\n",
            row.active_pair,
            row.control_pair,
            row.control_rank,
            if row.same_gap_bucket { "yes" } else { "no" },
            row.pair_distance,
            row.control_lane_minus_k00_pp,
            row.control_boundary_prime_delta_pp,
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Observations\n\n");
    for observation in &bundle.observations {
        markdown.push_str(&format!("- {}\n", observation));
    }
    markdown.push('\n');

    markdown.push_str("## Signal Read\n\n");
    markdown.push_str(
        "Base `34` still does not behave like base `14`. But it no longer looks merely empty or failed. The stronger reading is that it may host a second, weaker species: wide-gap, emergent, `k=(1,0)` boundary release.\n",
    );

    markdown
}

fn boundary_class(m1_active: bool, m2_active: bool) -> &'static str {
    match (m1_active, m2_active) {
        (false, false) => "never_by_m2",
        (true, false) => "m1_only",
        (false, true) => "m2_only",
        (true, true) => "m1_to_m2",
    }
}

fn gap_bucket(unit_gap: usize, unit_count: usize) -> &'static str {
    let cyclic_gap = unit_gap.min(unit_count - unit_gap);
    match cyclic_gap {
        0 => "same",
        1 => "adjacent",
        _ => "wide",
    }
}

fn role_rank(role: &str) -> usize {
    match role {
        "species_active" => 0,
        "nearby_dead_control" => 1,
        _ => 2,
    }
}

fn signal_source_label(
    stable_zero_prime_delta_pp: f64,
    boundary_prime_delta_pp: f64,
) -> &'static str {
    const EPS: f64 = 1e-9;
    let stable_abs = stable_zero_prime_delta_pp.abs();
    let boundary_abs = boundary_prime_delta_pp.abs();
    if stable_zero_prime_delta_pp > 0.0 && stable_abs > boundary_abs + EPS {
        "stable_zero_led"
    } else if boundary_abs > stable_abs + EPS {
        "boundary_led"
    } else {
        "mixed_or_flat"
    }
}

fn count_delta_pp(left: usize, right: usize, total: usize) -> f64 {
    (left as f64 - right as f64) * 100.0 / total as f64
}

fn anomaly_mass(row: &KDominancePairRow) -> f64 {
    row.best_minus_k00_pp.max(0.0)
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        None
    } else {
        Some(values.iter().sum::<f64>() / values.len() as f64)
    }
}

fn format_option_float(value: Option<f64>) -> String {
    value
        .map(|number| format!("{number:.2}pp"))
        .unwrap_or_else(|| "-".to_string())
}
