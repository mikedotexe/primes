//! Focused atlas for the base-14 boundary layer.
//!
//! This report narrows the transition story onto the richest surviving boundary
//! base from the full catalog. It computes the exact bounded-`k` scans for base
//! 14 at `M=1..3`, isolates the four `M=2`-active pairs, compares them against
//! nearby `m1_only` neighbors, and renders an atlas with three views:
//! - pair-space lattice colored by anomaly species
//! - local transition strip for survivors plus nearby dead neighbors
//! - `M=2` residue-relief heatmap for the same local neighborhood
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example base14_survivor_atlas_report
//! cargo run --release --example base14_survivor_atlas_report -- --out-dir /tmp/primes_base14_survivor_atlas
//! ```

use plotters::prelude::*;
use primes::validation::{
    bounded_k::{
        digit_symbol, evaluate_pair_row, format_k, ordered_unit_pairs, scan_k_config_profile,
        unit_residues, KConfigModulusDivisibilityRow, KDominancePairRow, DEFAULT_BOUNDED_K_GRID,
        DEFAULT_PREFILTER_PRIMES,
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

const BASE: u32 = 14;
const M1: usize = 1;
const M2: usize = 2;
const M3: usize = 3;
const DEFAULT_OUT_DIR: &str = "/tmp/primes_base14_survivor_atlas";
const REPORT_EXPORT_VERSION: u32 = 1;
const NEARBY_CONTROL_LIMIT: usize = 2;
const TOP_MODULUS_LIMIT: usize = 4;

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
    complement_pair: bool,
    unit_index_outer: usize,
    unit_index_inner: usize,
    unit_gap_bucket: String,
    row_m1: KDominancePairRow,
    row_m2: KDominancePairRow,
    row_m3: KDominancePairRow,
    boundary_class: String,
}

#[derive(Debug, Clone)]
struct ProfileDeltaSummary {
    admissible_delta_pp: f64,
    top_moduli_summary: String,
    modulus_relief_rows: Vec<ModulusReliefRow>,
}

#[derive(Debug, Clone)]
struct ModulusReliefRow {
    modulus: u32,
    k00_divisible_pp: f64,
    best_divisible_pp: f64,
    relief_pp: f64,
}

#[derive(Debug, Clone)]
struct AtlasEntry {
    pair_label: String,
    atlas_role: String,
    anchor_survivor: String,
    rank_within_anchor: usize,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    base: u32,
    middle_lengths: Vec<usize>,
    nearby_control_limit: usize,
    top_modulus_limit: usize,
    prefilter_moduli: Vec<u32>,
    k_grid: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct AtlasRow {
    pair_label: String,
    atlas_role: String,
    anchor_survivor: String,
    rank_within_anchor: usize,
    species: String,
    same_digit: bool,
    complement_pair: bool,
    unit_gap_bucket: String,
    unit_index_outer: usize,
    unit_index_inner: usize,
    best_k_m1: String,
    best_k_m2: String,
    anomaly_m1_pp: f64,
    anomaly_m2_pp: f64,
    anomaly_m3_pp: f64,
    admissible_delta_m2_pp: f64,
    top_moduli_m2: String,
}

#[derive(Debug, Clone, Serialize)]
struct NeighborRow {
    survivor_pair: String,
    survivor_species: String,
    control_pair: String,
    control_species: String,
    comparison_rank: usize,
    unit_distance: usize,
    same_gap_bucket: bool,
    same_same_digit: bool,
    same_best_k_m1: bool,
    survivor_anomaly_m1_pp: f64,
    survivor_anomaly_m2_pp: f64,
    control_anomaly_m1_pp: f64,
    control_anomaly_m2_pp: f64,
    survivor_top_moduli_m2: String,
    control_top_moduli_m2: String,
}

#[derive(Debug, Clone, Serialize)]
struct TransitionCellRow {
    pair_label: String,
    atlas_role: String,
    anchor_survivor: String,
    middle_length: usize,
    best_k: String,
    anomaly_mass_pp: f64,
    phase_state: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReliefHeatmapRow {
    pair_label: String,
    atlas_role: String,
    anchor_survivor: String,
    modulus: u32,
    relief_pp: f64,
    k00_divisible_pp: f64,
    best_divisible_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    total_base14_pairs: usize,
    m1_only_pairs: usize,
    m1_to_m2_pairs: usize,
    m2_only_pairs: usize,
    never_anomalous_pairs: usize,
    selected_atlas_rows: usize,
    selected_control_rows: usize,
    m2_active_pairs: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    atlas_rows: Vec<AtlasRow>,
    neighbor_rows: Vec<NeighborRow>,
    transition_cell_rows: Vec<TransitionCellRow>,
    relief_heatmap_rows: Vec<ReliefHeatmapRow>,
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
        middle_lengths: vec![M1, M2, M3],
        nearby_control_limit: options.nearby_control_limit,
        top_modulus_limit: TOP_MODULUS_LIMIT,
        prefilter_moduli: DEFAULT_PREFILTER_PRIMES.to_vec(),
        k_grid: DEFAULT_BOUNDED_K_GRID
            .iter()
            .map(|&config| format_k(config))
            .collect(),
    };

    let scans = build_boundary_scans();
    let survivors = scans
        .iter()
        .filter(|scan| anomaly_mass(&scan.row_m2) > 0.0)
        .collect::<Vec<_>>();
    let controls = scans
        .iter()
        .filter(|scan| scan.boundary_class == "m1_only")
        .collect::<Vec<_>>();
    let atlas_entries = build_atlas_entries(&survivors, &controls, options.nearby_control_limit);
    let atlas_rows = build_atlas_rows(&atlas_entries, &scans);
    let neighbor_rows = build_neighbor_rows(&survivors, &controls, options.nearby_control_limit);
    let transition_cell_rows = build_transition_cell_rows(&atlas_entries, &scans);
    let relief_heatmap_rows = build_relief_heatmap_rows(&atlas_entries, &scans);
    let report_summary = build_report_summary(&scans, &atlas_rows);

    let lattice_path = options.out_dir.join("base14_species_lattice.png");
    render_species_lattice(&scans, &lattice_path);
    let strip_path = options.out_dir.join("base14_local_transition_strip.png");
    render_local_transition_strip(&transition_cell_rows, &atlas_rows, &strip_path);
    let heatmap_path = options.out_dir.join("base14_residue_relief_heatmap_m2.png");
    render_relief_heatmap(&relief_heatmap_rows, &atlas_rows, &heatmap_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "species_lattice".to_string(),
            label: "Base 14 species lattice".to_string(),
            path: lattice_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "transition_strip".to_string(),
            label: "Base 14 local transition strip".to_string(),
            path: strip_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "relief_heatmap".to_string(),
            label: "Base 14 M=2 residue-relief heatmap".to_string(),
            path: heatmap_path.display().to_string(),
        },
    ];
    let observations = derive_observations(&atlas_rows, &neighbor_rows, &relief_heatmap_rows);

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        atlas_rows,
        neighbor_rows,
        transition_cell_rows,
        relief_heatmap_rows,
        image_artifact_rows,
        report_summary,
        observations,
    };

    write_csv_rows(options.out_dir.join("atlas_rows.csv"), &bundle.atlas_rows)
        .expect("failed to write atlas rows");
    write_csv_rows(
        options.out_dir.join("neighbor_rows.csv"),
        &bundle.neighbor_rows,
    )
    .expect("failed to write neighbor rows");
    write_csv_rows(
        options.out_dir.join("transition_cell_rows.csv"),
        &bundle.transition_cell_rows,
    )
    .expect("failed to write transition cell rows");
    write_csv_rows(
        options.out_dir.join("relief_heatmap_rows.csv"),
        &bundle.relief_heatmap_rows,
    )
    .expect("failed to write relief heatmap rows");
    write_csv_rows(
        options.out_dir.join("image_artifact_rows.csv"),
        &bundle.image_artifact_rows,
    )
    .expect("failed to write image artifact rows");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle)
        .expect("failed to write summary json");
    write_text_file(
        options.out_dir.join("report.md"),
        &render_markdown_report(&bundle),
    )
    .expect("failed to write markdown report");

    print_summary(&bundle);
}

fn parse_args() -> Options {
    let mut args = env::args().skip(1);
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut nearby_control_limit = NEARBY_CONTROL_LIMIT;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => {
                out_dir = PathBuf::from(parse_next::<String>(&mut args, "--out-dir"));
            }
            "--nearby-control-limit" => {
                nearby_control_limit = parse_next::<usize>(&mut args, "--nearby-control-limit");
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
        nearby_control_limit,
    }
}

fn parse_next<T>(args: &mut impl Iterator<Item = String>, flag: &str) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let raw = args.next().unwrap_or_else(|| {
        eprintln!("Missing value for {flag}");
        print_usage();
        std::process::exit(1);
    });
    raw.parse::<T>().unwrap_or_else(|err| {
        eprintln!("Invalid value for {flag}: {err}");
        print_usage();
        std::process::exit(1);
    })
}

fn print_usage() {
    println!("Base 14 survivor atlas report");
    println!();
    println!("Usage:");
    println!("  cargo run --release --example base14_survivor_atlas_report -- [options]");
    println!();
    println!("Options:");
    println!(
        "  --out-dir <path>              Output directory for images and summary files (default: {DEFAULT_OUT_DIR})"
    );
    println!(
        "  --nearby-control-limit <n>    Nearby m1-only controls per survivor (default: {NEARBY_CONTROL_LIMIT})"
    );
}

fn build_boundary_scans() -> Vec<BoundaryScan> {
    let units = unit_residues(BASE);
    let mut scans = ordered_unit_pairs(BASE)
        .into_iter()
        .map(|(outer, inner)| {
            let row_m1 = evaluate_pair_row(BASE, M1, outer, inner, DEFAULT_BOUNDED_K_GRID);
            let row_m2 = evaluate_pair_row(BASE, M2, outer, inner, DEFAULT_BOUNDED_K_GRID);
            let row_m3 = evaluate_pair_row(BASE, M3, outer, inner, DEFAULT_BOUNDED_K_GRID);
            let unit_index_outer = units
                .iter()
                .position(|&digit| digit == outer)
                .expect("outer digit should be a unit residue");
            let unit_index_inner = units
                .iter()
                .position(|&digit| digit == inner)
                .expect("inner digit should be a unit residue");
            BoundaryScan {
                outer,
                inner,
                pair_label: row_m1.pair_label.clone(),
                same_digit: outer == inner,
                complement_pair: (outer + inner).is_multiple_of(BASE),
                unit_index_outer,
                unit_index_inner,
                unit_gap_bucket: unit_gap_bucket(unit_index_outer.abs_diff(unit_index_inner)),
                boundary_class: boundary_class(
                    anomaly_mass_bool(&row_m1),
                    anomaly_mass_bool(&row_m2),
                    anomaly_mass_bool(&row_m3),
                ),
                row_m1,
                row_m2,
                row_m3,
            }
        })
        .collect::<Vec<_>>();
    scans.sort_by(|left, right| {
        left.outer
            .cmp(&right.outer)
            .then_with(|| left.inner.cmp(&right.inner))
    });
    scans
}

fn build_atlas_entries(
    survivors: &[&BoundaryScan],
    controls: &[&BoundaryScan],
    nearby_control_limit: usize,
) -> Vec<AtlasEntry> {
    let mut entries = Vec::new();
    let mut seen = BTreeSet::new();

    let mut sorted_survivors = survivors.to_vec();
    sorted_survivors.sort_by(|left, right| {
        species_rank(&left.boundary_class)
            .cmp(&species_rank(&right.boundary_class))
            .then_with(|| anomaly_mass(&right.row_m2).total_cmp(&anomaly_mass(&left.row_m2)))
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });

    for survivor in sorted_survivors {
        entries.push(AtlasEntry {
            pair_label: survivor.pair_label.clone(),
            atlas_role: survivor.boundary_class.clone(),
            anchor_survivor: survivor.pair_label.clone(),
            rank_within_anchor: 0,
        });
        seen.insert(survivor.pair_label.clone());
        for (rank, control) in nearest_controls(survivor, controls, nearby_control_limit)
            .into_iter()
            .enumerate()
        {
            if seen.insert(control.pair_label.clone()) {
                entries.push(AtlasEntry {
                    pair_label: control.pair_label.clone(),
                    atlas_role: "m1_only_control".to_string(),
                    anchor_survivor: survivor.pair_label.clone(),
                    rank_within_anchor: rank + 1,
                });
            }
        }
    }

    entries
}

fn build_atlas_rows(entries: &[AtlasEntry], scans: &[BoundaryScan]) -> Vec<AtlasRow> {
    let scan_lookup = scans
        .iter()
        .map(|scan| (scan.pair_label.clone(), scan))
        .collect::<BTreeMap<_, _>>();
    let mut rows = entries
        .iter()
        .map(|entry| {
            let scan = scan_lookup
                .get(&entry.pair_label)
                .expect("atlas entry should point to known scan");
            let m2_summary = profile_delta_summary(scan, M2, &scan.row_m2);
            AtlasRow {
                pair_label: entry.pair_label.clone(),
                atlas_role: entry.atlas_role.clone(),
                anchor_survivor: entry.anchor_survivor.clone(),
                rank_within_anchor: entry.rank_within_anchor,
                species: scan.boundary_class.clone(),
                same_digit: scan.same_digit,
                complement_pair: scan.complement_pair,
                unit_gap_bucket: scan.unit_gap_bucket.clone(),
                unit_index_outer: scan.unit_index_outer,
                unit_index_inner: scan.unit_index_inner,
                best_k_m1: scan.row_m1.best_k.clone(),
                best_k_m2: scan.row_m2.best_k.clone(),
                anomaly_m1_pp: anomaly_mass(&scan.row_m1),
                anomaly_m2_pp: anomaly_mass(&scan.row_m2),
                anomaly_m3_pp: anomaly_mass(&scan.row_m3),
                admissible_delta_m2_pp: m2_summary.admissible_delta_pp,
                top_moduli_m2: m2_summary.top_moduli_summary,
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        atlas_role_rank(&left.atlas_role)
            .cmp(&atlas_role_rank(&right.atlas_role))
            .then_with(|| left.anchor_survivor.cmp(&right.anchor_survivor))
            .then_with(|| left.rank_within_anchor.cmp(&right.rank_within_anchor))
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });
    rows
}

fn build_neighbor_rows(
    survivors: &[&BoundaryScan],
    controls: &[&BoundaryScan],
    nearby_control_limit: usize,
) -> Vec<NeighborRow> {
    let mut rows = Vec::new();
    let mut sorted_survivors = survivors.to_vec();
    sorted_survivors.sort_by(|left, right| left.pair_label.cmp(&right.pair_label));

    for survivor in sorted_survivors {
        let survivor_m2 = profile_delta_summary(survivor, M2, &survivor.row_m2);
        for (rank, control) in nearest_controls(survivor, controls, nearby_control_limit)
            .into_iter()
            .enumerate()
        {
            let control_m2 = profile_delta_summary(control, M2, &control.row_m2);
            rows.push(NeighborRow {
                survivor_pair: survivor.pair_label.clone(),
                survivor_species: survivor.boundary_class.clone(),
                control_pair: control.pair_label.clone(),
                control_species: control.boundary_class.clone(),
                comparison_rank: rank + 1,
                unit_distance: pair_distance(survivor, control),
                same_gap_bucket: survivor.unit_gap_bucket == control.unit_gap_bucket,
                same_same_digit: survivor.same_digit == control.same_digit,
                same_best_k_m1: survivor.row_m1.best_k == control.row_m1.best_k,
                survivor_anomaly_m1_pp: anomaly_mass(&survivor.row_m1),
                survivor_anomaly_m2_pp: anomaly_mass(&survivor.row_m2),
                control_anomaly_m1_pp: anomaly_mass(&control.row_m1),
                control_anomaly_m2_pp: anomaly_mass(&control.row_m2),
                survivor_top_moduli_m2: survivor_m2.top_moduli_summary.clone(),
                control_top_moduli_m2: control_m2.top_moduli_summary,
            });
        }
    }
    rows
}

fn build_transition_cell_rows(
    entries: &[AtlasEntry],
    scans: &[BoundaryScan],
) -> Vec<TransitionCellRow> {
    let scan_lookup = scans
        .iter()
        .map(|scan| (scan.pair_label.clone(), scan))
        .collect::<BTreeMap<_, _>>();
    let mut rows = Vec::new();
    for entry in entries {
        let scan = scan_lookup
            .get(&entry.pair_label)
            .expect("transition entry should point to known scan");
        for (middle_length, row) in [(M1, &scan.row_m1), (M2, &scan.row_m2), (M3, &scan.row_m3)] {
            rows.push(TransitionCellRow {
                pair_label: entry.pair_label.clone(),
                atlas_role: entry.atlas_role.clone(),
                anchor_survivor: entry.anchor_survivor.clone(),
                middle_length,
                best_k: row.best_k.clone(),
                anomaly_mass_pp: anomaly_mass(row),
                phase_state: if row.k00_noninferior {
                    "k00_noninferior".to_string()
                } else {
                    row.best_k.clone()
                },
            });
        }
    }
    rows
}

fn build_relief_heatmap_rows(
    entries: &[AtlasEntry],
    scans: &[BoundaryScan],
) -> Vec<ReliefHeatmapRow> {
    let scan_lookup = scans
        .iter()
        .map(|scan| (scan.pair_label.clone(), scan))
        .collect::<BTreeMap<_, _>>();
    let mut rows = Vec::new();
    for entry in entries {
        let scan = scan_lookup
            .get(&entry.pair_label)
            .expect("heatmap entry should point to known scan");
        let summary = profile_delta_summary(scan, M2, &scan.row_m2);
        let modulus_lookup = summary
            .modulus_relief_rows
            .iter()
            .map(|row| (row.modulus, row))
            .collect::<BTreeMap<_, _>>();
        for &modulus in DEFAULT_PREFILTER_PRIMES {
            let row = modulus_lookup
                .get(&modulus)
                .expect("each modulus should be present");
            rows.push(ReliefHeatmapRow {
                pair_label: entry.pair_label.clone(),
                atlas_role: entry.atlas_role.clone(),
                anchor_survivor: entry.anchor_survivor.clone(),
                modulus,
                relief_pp: row.relief_pp,
                k00_divisible_pp: row.k00_divisible_pp,
                best_divisible_pp: row.best_divisible_pp,
            });
        }
    }
    rows
}

fn build_report_summary(scans: &[BoundaryScan], atlas_rows: &[AtlasRow]) -> ReportSummary {
    ReportSummary {
        total_base14_pairs: scans.len(),
        m1_only_pairs: scans
            .iter()
            .filter(|scan| scan.boundary_class == "m1_only")
            .count(),
        m1_to_m2_pairs: scans
            .iter()
            .filter(|scan| scan.boundary_class == "m1_to_m2")
            .count(),
        m2_only_pairs: scans
            .iter()
            .filter(|scan| scan.boundary_class == "m2_only")
            .count(),
        never_anomalous_pairs: scans
            .iter()
            .filter(|scan| scan.boundary_class == "never_anomalous")
            .count(),
        selected_atlas_rows: atlas_rows.len(),
        selected_control_rows: atlas_rows
            .iter()
            .filter(|row| row.atlas_role == "m1_only_control")
            .count(),
        m2_active_pairs: scans
            .iter()
            .filter(|scan| anomaly_mass(&scan.row_m2) > 0.0)
            .map(|scan| scan.pair_label.clone())
            .collect::<Vec<_>>()
            .join(", "),
    }
}

fn render_species_lattice(scans: &[BoundaryScan], path: &Path) {
    let unit_digits = unit_residues(BASE);
    let unit_symbols = unit_digits
        .iter()
        .copied()
        .map(digit_symbol)
        .collect::<Vec<_>>();
    let max_index = unit_digits.len() as i32;
    let root = BitMapBackend::new(path, (900, 900)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill lattice canvas");

    let x_labels = unit_symbols.clone();
    let y_labels = unit_symbols;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Base 14 Boundary Species Lattice  (outline = M=2 active)",
            ("sans-serif", 26),
        )
        .margin(24)
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(0i32..max_index, 0i32..max_index)
        .expect("failed to build lattice chart");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(unit_digits.len())
        .y_labels(unit_digits.len())
        .x_desc("outer residue")
        .y_desc("inner residue")
        .x_label_formatter(&move |value| {
            if *value >= 0 && (*value as usize) < x_labels.len() {
                x_labels[*value as usize].clone()
            } else {
                String::new()
            }
        })
        .y_label_formatter(&move |value| {
            if *value >= 0 && (*value as usize) < y_labels.len() {
                y_labels[*value as usize].clone()
            } else {
                String::new()
            }
        })
        .label_style(("sans-serif", 18))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw lattice mesh");

    chart
        .draw_series(
            (0..max_index)
                .flat_map(|x| (0..max_index).map(move |y| (x, y)))
                .map(|(x, y)| {
                    Circle::new(
                        (x, y),
                        3,
                        ShapeStyle::from(&RGBColor(223, 217, 207)).filled(),
                    )
                }),
        )
        .expect("failed to draw lattice background");

    for scan in scans {
        let strongest_mass = anomaly_mass(&scan.row_m1).max(anomaly_mass(&scan.row_m2));
        let radius = (5.0 + strongest_mass.min(22.0) / 2.4).round() as i32;
        let color = species_color(&scan.boundary_class);
        chart
            .draw_series(std::iter::once(Circle::new(
                (scan.unit_index_outer as i32, scan.unit_index_inner as i32),
                radius,
                ShapeStyle::from(&color).filled(),
            )))
            .expect("failed to draw lattice point");

        if anomaly_mass(&scan.row_m2) > 0.0 {
            chart
                .draw_series(std::iter::once(Circle::new(
                    (scan.unit_index_outer as i32, scan.unit_index_inner as i32),
                    radius + 2,
                    ShapeStyle::from(&BLACK).stroke_width(2),
                )))
                .expect("failed to draw survivor outline");
        }
    }

    root.present().expect("failed to present lattice image");
}

fn render_local_transition_strip(
    transition_rows: &[TransitionCellRow],
    atlas_rows: &[AtlasRow],
    path: &Path,
) {
    let row_labels = atlas_rows
        .iter()
        .map(|row| format!("{} {}", short_role_label(&row.atlas_role), row.pair_label))
        .collect::<Vec<_>>();
    let row_lookup = atlas_rows
        .iter()
        .enumerate()
        .map(|(index, row)| (row.pair_label.clone(), index as i32))
        .collect::<BTreeMap<_, _>>();
    let max_y = atlas_rows.len() as i32;
    let max_mass = transition_rows
        .iter()
        .map(|row| row.anomaly_mass_pp)
        .fold(0.0_f64, f64::max);

    let root = BitMapBackend::new(path, (980, (240 + atlas_rows.len() as u32 * 42).max(520)))
        .into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill transition strip canvas");

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Base 14 Local Transition Strip  (survivors plus nearby m1-only neighbors)",
            ("sans-serif", 26),
        )
        .margin(24)
        .x_label_area_size(54)
        .y_label_area_size(170)
        .build_cartesian_2d(0i32..3i32, 0i32..max_y)
        .expect("failed to build transition strip chart");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(3)
        .y_labels(atlas_rows.len())
        .x_desc("middle length M")
        .y_desc("atlas rows")
        .x_label_formatter(&move |value| match *value {
            0 => "1".to_string(),
            1 => "2".to_string(),
            2 => "3".to_string(),
            _ => String::new(),
        })
        .y_label_formatter(&move |value| {
            if *value >= 0 && *value < max_y {
                let row_index = (max_y - 1 - *value) as usize;
                row_labels.get(row_index).cloned().unwrap_or_default()
            } else {
                String::new()
            }
        })
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw transition strip mesh");

    for row in transition_rows {
        let x = (row.middle_length - 1) as i32;
        let y = max_y
            - 1
            - row_lookup
                .get(&row.pair_label)
                .copied()
                .expect("transition row should be in atlas");
        let fill = phase_color(&row.best_k, row.anomaly_mass_pp, max_mass);
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x, y), (x + 1, y + 1)],
                ShapeStyle::from(&fill).filled(),
            )))
            .expect("failed to draw transition cell");
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x, y), (x + 1, y + 1)],
                ShapeStyle::from(&RGBColor(198, 191, 182)).stroke_width(1),
            )))
            .expect("failed to draw transition cell outline");
        if row.anomaly_mass_pp > 0.0 {
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(x, y), (x + 1, y + 1)],
                    ShapeStyle::from(&BLACK.mix(0.8)).stroke_width(2),
                )))
                .expect("failed to draw transition anomaly border");
        }
    }

    root.present()
        .expect("failed to present transition strip image");
}

fn render_relief_heatmap(rows: &[ReliefHeatmapRow], atlas_rows: &[AtlasRow], path: &Path) {
    let row_labels = atlas_rows
        .iter()
        .map(|row| format!("{} {}", short_role_label(&row.atlas_role), row.pair_label))
        .collect::<Vec<_>>();
    let row_lookup = atlas_rows
        .iter()
        .enumerate()
        .map(|(index, row)| (row.pair_label.clone(), index as i32))
        .collect::<BTreeMap<_, _>>();
    let moduli = DEFAULT_PREFILTER_PRIMES.to_vec();
    let max_y = atlas_rows.len() as i32;
    let max_relief = rows
        .iter()
        .map(|row| row.relief_pp.abs())
        .fold(0.0_f64, f64::max)
        .max(0.5);

    let root = BitMapBackend::new(path, (1220, (260 + atlas_rows.len() as u32 * 42).max(560)))
        .into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill relief heatmap canvas");

    let x_labels = moduli.iter().map(u32::to_string).collect::<Vec<_>>();
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Base 14 M=2 Residue-Relief Heatmap  (positive = less divisibility than k=(0,0))",
            ("sans-serif", 24),
        )
        .margin(24)
        .x_label_area_size(54)
        .y_label_area_size(170)
        .build_cartesian_2d(0i32..moduli.len() as i32, 0i32..max_y)
        .expect("failed to build relief heatmap chart");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(moduli.len())
        .y_labels(atlas_rows.len())
        .x_desc("small prime modulus")
        .y_desc("atlas rows")
        .x_label_formatter(&move |value| {
            if *value >= 0 && (*value as usize) < x_labels.len() {
                x_labels[*value as usize].clone()
            } else {
                String::new()
            }
        })
        .y_label_formatter(&move |value| {
            if *value >= 0 && *value < max_y {
                let row_index = (max_y - 1 - *value) as usize;
                row_labels.get(row_index).cloned().unwrap_or_default()
            } else {
                String::new()
            }
        })
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw relief heatmap mesh");

    for row in rows {
        let x = moduli
            .iter()
            .position(|&modulus| modulus == row.modulus)
            .expect("modulus should be in prefilter list") as i32;
        let y = max_y
            - 1
            - row_lookup
                .get(&row.pair_label)
                .copied()
                .expect("relief row should be in atlas");
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x, y), (x + 1, y + 1)],
                ShapeStyle::from(&relief_color(row.relief_pp, max_relief)).filled(),
            )))
            .expect("failed to draw relief cell");
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x, y), (x + 1, y + 1)],
                ShapeStyle::from(&RGBColor(198, 191, 182)).stroke_width(1),
            )))
            .expect("failed to draw relief cell outline");
    }

    root.present().expect("failed to present relief heatmap");
}

fn nearest_controls<'a>(
    survivor: &BoundaryScan,
    controls: &'a [&BoundaryScan],
    limit: usize,
) -> Vec<&'a BoundaryScan> {
    let mut same_base = controls.to_vec();
    same_base.sort_by(|left, right| {
        control_rank_tuple(survivor, left).cmp(&control_rank_tuple(survivor, right))
    });
    same_base.truncate(limit);
    same_base
}

fn control_rank_tuple(
    survivor: &BoundaryScan,
    control: &BoundaryScan,
) -> (usize, usize, usize, usize, String) {
    (
        usize::from(survivor.unit_gap_bucket != control.unit_gap_bucket),
        usize::from(survivor.same_digit != control.same_digit),
        usize::from(survivor.row_m1.best_k != control.row_m1.best_k),
        pair_distance(survivor, control),
        control.pair_label.clone(),
    )
}

fn pair_distance(left: &BoundaryScan, right: &BoundaryScan) -> usize {
    left.unit_index_outer.abs_diff(right.unit_index_outer)
        + left.unit_index_inner.abs_diff(right.unit_index_inner)
}

fn profile_delta_summary(
    scan: &BoundaryScan,
    middle_length: usize,
    row: &KDominancePairRow,
) -> ProfileDeltaSummary {
    let k00_profile = scan_k_config_profile(BASE, middle_length, scan.outer, scan.inner, (0, 0));
    let best_config = parse_k_label(&row.best_k);
    let best_profile = if best_config == (0, 0) {
        k00_profile.clone()
    } else {
        scan_k_config_profile(BASE, middle_length, scan.outer, scan.inner, best_config)
    };
    let modulus_relief_rows = zip_modulus_rows(
        k00_profile.modulus_divisibility_rows,
        best_profile.modulus_divisibility_rows,
        best_profile.candidates_per_config,
    );

    ProfileDeltaSummary {
        admissible_delta_pp: count_delta_pp(
            best_profile.admissible_count,
            k00_profile.admissible_count,
            best_profile.candidates_per_config,
        ),
        top_moduli_summary: render_top_moduli(&modulus_relief_rows),
        modulus_relief_rows,
    }
}

fn zip_modulus_rows(
    k00_rows: Vec<KConfigModulusDivisibilityRow>,
    best_rows: Vec<KConfigModulusDivisibilityRow>,
    candidates_per_config: usize,
) -> Vec<ModulusReliefRow> {
    let mut rows = k00_rows
        .into_iter()
        .zip(best_rows)
        .map(|(k00_row, best_row)| ModulusReliefRow {
            modulus: k00_row.modulus,
            k00_divisible_pp: count_to_pp(k00_row.divisible_count, candidates_per_config),
            best_divisible_pp: count_to_pp(best_row.divisible_count, candidates_per_config),
            relief_pp: count_delta_pp(
                k00_row.divisible_count,
                best_row.divisible_count,
                candidates_per_config,
            ),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.modulus.cmp(&right.modulus));
    rows
}

fn render_top_moduli(rows: &[ModulusReliefRow]) -> String {
    let mut top_rows = rows
        .iter()
        .filter(|row| row.relief_pp > 0.0)
        .collect::<Vec<_>>();
    top_rows.sort_by(|left, right| {
        right
            .relief_pp
            .total_cmp(&left.relief_pp)
            .then_with(|| left.modulus.cmp(&right.modulus))
    });
    top_rows.truncate(TOP_MODULUS_LIMIT);
    if top_rows.is_empty() {
        "none".to_string()
    } else {
        top_rows
            .into_iter()
            .map(|row| format!("p{}:+{:.2}pp", row.modulus, row.relief_pp))
            .collect::<Vec<_>>()
            .join(";")
    }
}

fn parse_k_label(label: &str) -> (u32, u32) {
    DEFAULT_BOUNDED_K_GRID
        .iter()
        .copied()
        .find(|&config| format_k(config) == label)
        .unwrap_or_else(|| panic!("unrecognized k label: {label}"))
}

fn anomaly_mass(row: &KDominancePairRow) -> f64 {
    if row.best_minus_k00_pp > 0.0 {
        row.best_minus_k00_pp
    } else {
        0.0
    }
}

fn anomaly_mass_bool(row: &KDominancePairRow) -> bool {
    anomaly_mass(row) > 0.0
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

fn unit_gap_bucket(unit_gap: usize) -> String {
    match unit_gap {
        0 => "same".to_string(),
        1 => "adjacent".to_string(),
        _ => "wide".to_string(),
    }
}

fn species_rank(species: &str) -> usize {
    match species {
        "m1_to_m2" => 0,
        "m2_only" => 1,
        "m1_only" => 2,
        "never_anomalous" => 3,
        _ => 4,
    }
}

fn atlas_role_rank(role: &str) -> usize {
    match role {
        "m1_to_m2" => 0,
        "m2_only" => 1,
        "m1_only_control" => 2,
        _ => 3,
    }
}

fn short_role_label(role: &str) -> &'static str {
    match role {
        "m1_to_m2" => "persist",
        "m2_only" => "emerge",
        "m1_only_control" => "ctrl",
        _ => "row",
    }
}

fn species_color(species: &str) -> RGBColor {
    match species {
        "m1_only" => RGBColor(124, 130, 146),
        "m1_to_m2" => RGBColor(214, 104, 36),
        "m2_only" => RGBColor(24, 136, 125),
        "never_anomalous" => RGBColor(214, 208, 198),
        _ => RGBColor(96, 96, 96),
    }
}

fn phase_color(best_k: &str, anomaly_mass_pp: f64, max_mass_pp: f64) -> RGBColor {
    let normalized = if max_mass_pp > 0.0 {
        (anomaly_mass_pp / max_mass_pp).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let (start, end) = match best_k {
        "k=(0,1)" => ((228.0, 237.0, 245.0), (59.0, 99.0, 140.0)),
        "k=(1,0)" => ((250.0, 235.0, 220.0), (214.0, 108.0, 33.0)),
        "k=(1,1)" => ((224.0, 242.0, 238.0), (23.0, 133.0, 123.0)),
        "k=(2,2)" => ((243.0, 231.0, 238.0), (145.0, 68.0, 97.0)),
        _ => ((245.0, 241.0, 235.0), (213.0, 206.0, 197.0)),
    };
    let t = if best_k == "k=(0,0)" {
        0.25
    } else {
        0.25 + 0.75 * normalized
    };
    RGBColor(
        lerp(start.0, end.0, t) as u8,
        lerp(start.1, end.1, t) as u8,
        lerp(start.2, end.2, t) as u8,
    )
}

fn relief_color(relief_pp: f64, max_abs_relief: f64) -> RGBColor {
    let t = if max_abs_relief > 0.0 {
        (relief_pp / max_abs_relief).clamp(-1.0, 1.0)
    } else {
        0.0
    };
    if t >= 0.0 {
        let start = (245.0, 241.0, 235.0);
        let end = (39.0, 120.0, 112.0);
        RGBColor(
            lerp(start.0, end.0, t) as u8,
            lerp(start.1, end.1, t) as u8,
            lerp(start.2, end.2, t) as u8,
        )
    } else {
        let start = (245.0, 241.0, 235.0);
        let end = (202.0, 105.0, 52.0);
        let t = -t;
        RGBColor(
            lerp(start.0, end.0, t) as u8,
            lerp(start.1, end.1, t) as u8,
            lerp(start.2, end.2, t) as u8,
        )
    }
}

fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}

fn count_to_pp(count: usize, total: usize) -> f64 {
    count as f64 * 100.0 / total as f64
}

fn count_delta_pp(left_count: usize, right_count: usize, total: usize) -> f64 {
    (left_count as f64 - right_count as f64) * 100.0 / total as f64
}

fn derive_observations(
    atlas_rows: &[AtlasRow],
    neighbor_rows: &[NeighborRow],
    relief_rows: &[ReliefHeatmapRow],
) -> Vec<String> {
    let survivors = atlas_rows
        .iter()
        .filter(|row| row.atlas_role == "m1_to_m2" || row.atlas_role == "m2_only")
        .collect::<Vec<_>>();
    let persistent = survivors
        .iter()
        .filter(|row| row.atlas_role == "m1_to_m2")
        .map(|row| row.pair_label.clone())
        .collect::<Vec<_>>();
    let emergent = survivors
        .iter()
        .filter(|row| row.atlas_role == "m2_only")
        .map(|row| row.pair_label.clone())
        .collect::<Vec<_>>();
    let strongest_survivor = survivors
        .iter()
        .max_by(|left, right| left.anomaly_m2_pp.total_cmp(&right.anomaly_m2_pp))
        .expect("survivor atlas should have active pairs");
    let strongest_neighbor = neighbor_rows
        .iter()
        .filter(|row| row.survivor_pair == strongest_survivor.pair_label)
        .min_by(|left, right| {
            left.unit_distance
                .cmp(&right.unit_distance)
                .then_with(|| left.comparison_rank.cmp(&right.comparison_rank))
        })
        .expect("strongest survivor should have neighbor rows");
    let mut relief_by_pair = BTreeMap::new();
    for row in relief_rows {
        relief_by_pair
            .entry(row.pair_label.clone())
            .or_insert_with(Vec::new)
            .push(row);
    }
    let best_relief_pair = relief_by_pair
        .iter()
        .max_by(|left, right| {
            left.1
                .iter()
                .map(|row| row.relief_pp)
                .fold(0.0_f64, f64::max)
                .total_cmp(
                    &right
                        .1
                        .iter()
                        .map(|row| row.relief_pp)
                        .fold(0.0_f64, f64::max),
                )
        })
        .expect("relief rows should exist");
    let best_relief_row = best_relief_pair
        .1
        .iter()
        .max_by(|left, right| left.relief_pp.total_cmp(&right.relief_pp))
        .expect("relief pair should have modulus rows");

    vec![
        format!(
            "Base 14 has exactly four `M=2`-active pairs: persistent `{}` and emergent `{}`.",
            persistent.join(", "),
            emergent.join(", ")
        ),
        format!(
            "The strongest `M=2` survivor in this base is `{}` at `{:.2}pp`, and its nearest dead neighbor in the atlas is `{}` at unit distance `{}`.",
            strongest_survivor.pair_label,
            strongest_survivor.anomaly_m2_pp,
            strongest_neighbor.control_pair,
            strongest_neighbor.unit_distance
        ),
        format!(
            "The sharpest single-modulus relief in the atlas is `{}` at modulus `p{}` with `{:+.2}pp`; that gives us a concrete small-prime fingerprint to inspect mathematically.",
            best_relief_pair.0,
            best_relief_row.modulus,
            best_relief_row.relief_pp
        ),
        "The strongest survivor `(D,B)` still matters because it survives at `M=2` while its top-modulus summary is `none`; that suggests the base-14 boundary layer is not exhausted by one-modulus relief alone.".to_string(),
    ]
}

fn render_markdown_report(bundle: &ReportBundle) -> String {
    let mut markdown = String::new();
    markdown.push_str("# Base 14 Survivor Atlas Report\n\n");
    markdown.push_str("_Generated from `examples/base14_survivor_atlas_report.rs`._\n\n");
    markdown.push_str(&format!("- Generated at: `{}`\n", bundle.generated_at_utc));
    markdown.push_str(&format!("- Base: `{}`\n", bundle.settings.base));
    markdown.push_str(&format!(
        "- Nearby controls per survivor: `{}`\n",
        bundle.settings.nearby_control_limit
    ));
    markdown.push_str(&format!(
        "- Middle lengths: `{:?}`\n\n",
        bundle.settings.middle_lengths
    ));

    markdown.push_str("## What To Notice\n\n");
    for observation in &bundle.observations {
        markdown.push_str(&format!("- {observation}\n"));
    }
    markdown.push('\n');

    markdown.push_str("## Image Artifacts\n\n");
    markdown.push_str("| Kind | Label | Path |\n");
    markdown.push_str("|---|---|---|\n");
    for row in &bundle.image_artifact_rows {
        markdown.push_str(&format!(
            "| `{}` | {} | `{}` |\n",
            row.kind, row.label, row.path
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Summary\n\n");
    markdown.push_str(&format!(
        "- Total base-14 ordered pairs: `{}`\n",
        bundle.report_summary.total_base14_pairs
    ));
    markdown.push_str(&format!(
        "- Species counts: `m1_only {}`, `m1_to_m2 {}`, `m2_only {}`, `never_anomalous {}`\n",
        bundle.report_summary.m1_only_pairs,
        bundle.report_summary.m1_to_m2_pairs,
        bundle.report_summary.m2_only_pairs,
        bundle.report_summary.never_anomalous_pairs
    ));
    markdown.push_str(&format!(
        "- M=2 active pairs: `{}`\n",
        bundle.report_summary.m2_active_pairs
    ));
    markdown.push('\n');

    markdown.push_str("## Atlas Rows\n\n");
    markdown.push_str("| Role | Pair | Species | Gap | best k @ M1 | best k @ M2 | M1 anomaly | M2 anomaly | top M2 moduli |\n");
    markdown.push_str("|---|---|---|---|---|---|---:|---:|---|\n");
    for row in &bundle.atlas_rows {
        markdown.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{:.2}pp` | `{:.2}pp` | `{}` |\n",
            row.atlas_role,
            row.pair_label,
            row.species,
            row.unit_gap_bucket,
            row.best_k_m1,
            row.best_k_m2,
            row.anomaly_m1_pp,
            row.anomaly_m2_pp,
            row.top_moduli_m2
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Nearest Dead Neighbors\n\n");
    markdown.push_str("| Survivor | Control | Distance | Same gap | Same same-digit | Same best k @ M1 | Survivor M2 | Control M2 |\n");
    markdown.push_str("|---|---|---:|---|---|---|---:|---:|\n");
    for row in &bundle.neighbor_rows {
        markdown.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{:.2}pp` | `{:.2}pp` |\n",
            row.survivor_pair,
            row.control_pair,
            row.unit_distance,
            row.same_gap_bucket,
            row.same_same_digit,
            row.same_best_k_m1,
            row.survivor_anomaly_m2_pp,
            row.control_anomaly_m2_pp
        ));
    }

    markdown
}

fn print_summary(bundle: &ReportBundle) {
    println!("=== Base 14 Survivor Atlas Report ===");
    println!();
    println!(
        "Base {} | atlas rows {} | controls {} | M=2 active {}",
        bundle.settings.base,
        bundle.report_summary.selected_atlas_rows,
        bundle.report_summary.selected_control_rows,
        bundle.report_summary.m2_active_pairs
    );
    println!(
        "Species counts: m1_only {} | m1_to_m2 {} | m2_only {} | never_anomalous {}",
        bundle.report_summary.m1_only_pairs,
        bundle.report_summary.m1_to_m2_pairs,
        bundle.report_summary.m2_only_pairs,
        bundle.report_summary.never_anomalous_pairs
    );
    for row in bundle
        .atlas_rows
        .iter()
        .filter(|row| row.atlas_role == "m1_to_m2" || row.atlas_role == "m2_only")
    {
        println!(
            "  - {:>6} {:>6}: M2 {:.2}pp | best {} | top moduli {}",
            row.atlas_role, row.pair_label, row.anomaly_m2_pp, row.best_k_m2, row.top_moduli_m2
        );
    }
}
