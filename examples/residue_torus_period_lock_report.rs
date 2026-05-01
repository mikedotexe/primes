//! Visual-intuition report for the affine period-lock residue torus.
//!
//! This report promotes the residue torus from a side plot into a maintained
//! affine artifact. It reuses the exact period-lock scan surface and adds a
//! skeptical-audience walkthrough: what the torus means, how to read the
//! base-22 / mod-5 pocket, and where prime witnesses enter without implying a
//! density theorem.
//!
//! The explanatory anchor is: we are not visualizing where primes magically
//! appear; we are visualizing the arithmetic surface that a fixed symmetric
//! template forces every candidate to live on. The affine core is
//! `N(s) = A + G*s`.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example residue_torus_period_lock_report
//! cargo run --release --example residue_torus_period_lock_report -- --out-dir /tmp/primes_residue_torus_period_lock
//! ```

use num_bigint::BigUint;
use plotters::prelude::*;
use primes::validation::{
    affine_period_lock::{
        scan_k_config_affine_period_lock_comparison, AffinePeriodLockComparison,
        AffinePeriodLockModulusRow,
    },
    bounded_k::{
        digit_symbol, format_k, ordered_unit_pairs, scan_k_config_examples, to_base_string_fixed,
        BoundedKConfig,
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

const MAIN_BASES: &[u32] = &[10, 14, 22, 26];
const APPENDIX_BASES: &[u32] = &[34, 6];
const MIDDLE_LENGTHS: &[usize] = &[1, 2, 3];
const FROM_K: BoundedKConfig = (0, 0);
const NONCOMPACT_LANES: &[BoundedKConfig] = &[(0, 1), (1, 0), (1, 1), (2, 2)];
const DEFAULT_OUT_DIR: &str = "/tmp/primes_residue_torus_period_lock";
const REPORT_EXPORT_VERSION: u32 = 1;
const ARTIFACT_ID: &str = "residue_torus_period_lock_report";

const CANONICAL_BASE: u32 = 22;
const CANONICAL_OUTER: u32 = 17;
const CANONICAL_INNER: u32 = 19;
const CANONICAL_MIDDLE_LENGTH: usize = 2;
const CANONICAL_TO_K: BoundedKConfig = (2, 2);
const CANONICAL_MODULUS: u32 = 5;

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
}

#[derive(Debug, Clone)]
struct ComparisonBundle {
    comparison: AffinePeriodLockComparison,
}

#[derive(Debug, Clone, Copy)]
enum ComparisonSide {
    From,
    To,
}

#[derive(Debug, Clone, Copy)]
struct WitnessSpec {
    role: &'static str,
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    k: BoundedKConfig,
    comparison_to_k: BoundedKConfig,
    modulus: u32,
    side: ComparisonSide,
    note: &'static str,
}

const WITNESS_SPECS: &[WitnessSpec] = &[
    WitnessSpec {
        role: "decimal_visible_zero_run_k21",
        base: 10,
        middle_length: 2,
        outer: 3,
        inner: 7,
        k: (2, 1),
        comparison_to_k: (2, 1),
        modulus: 3,
        side: ComparisonSide::To,
        note: "Teaching witness with visibly mirrored zero runs: 3 00 7 0 seed 0 7 00 3.",
    },
    WitnessSpec {
        role: "decimal_visible_zero_run_k11",
        base: 10,
        middle_length: 2,
        outer: 3,
        inner: 7,
        k: (1, 1),
        comparison_to_k: (1, 1),
        modulus: 11,
        side: ComparisonSide::To,
        note: "Compact decimal teaching witness with one mirrored zero run on each side.",
    },
    WitnessSpec {
        role: "decimal_deep_zero_run_k22",
        base: 10,
        middle_length: 2,
        outer: 1,
        inner: 7,
        k: (2, 2),
        comparison_to_k: (2, 2),
        modulus: 3,
        side: ComparisonSide::To,
        note:
            "Deep decimal zero-run witness showing the construction without base-letter notation.",
    },
    WitnessSpec {
        role: "decimal_m3_nonpal_center",
        base: 10,
        middle_length: 3,
        outer: 3,
        inner: 1,
        k: (2, 2),
        comparison_to_k: (2, 2),
        modulus: 3,
        side: ComparisonSide::To,
        note: "M=3 decimal witness: the center block need not itself be palindromic.",
    },
    WitnessSpec {
        role: "canonical_compact_lane",
        base: CANONICAL_BASE,
        middle_length: CANONICAL_MIDDLE_LENGTH,
        outer: CANONICAL_OUTER,
        inner: CANONICAL_INNER,
        k: FROM_K,
        comparison_to_k: CANONICAL_TO_K,
        modulus: CANONICAL_MODULUS,
        side: ComparisonSide::From,
        note: "Compact side of the base-22/mod-5 walkthrough.",
    },
    WitnessSpec {
        role: "base_22_higher_order_side_pocket",
        base: CANONICAL_BASE,
        middle_length: CANONICAL_MIDDLE_LENGTH,
        outer: CANONICAL_OUTER,
        inner: CANONICAL_INNER,
        k: CANONICAL_TO_K,
        comparison_to_k: CANONICAL_TO_K,
        modulus: CANONICAL_MODULUS,
        side: ComparisonSide::To,
        note: "Period-locked gradients with separated shifts: gradient_only.",
    },
    WitnessSpec {
        role: "base_10_persistence_identity",
        base: 10,
        middle_length: 2,
        outer: 3,
        inner: 3,
        k: (1, 0),
        comparison_to_k: (1, 0),
        modulus: 3,
        side: ComparisonSide::To,
        note: "Low-order locked behavior that collapses to identity locally.",
    },
    WitnessSpec {
        role: "base_14_persistent_core",
        base: 14,
        middle_length: 2,
        outer: 13,
        inner: 11,
        k: (0, 1),
        comparison_to_k: (0, 1),
        modulus: 13,
        side: ComparisonSide::To,
        note: "Persistent core example on a nondecimal base surface.",
    },
    WitnessSpec {
        role: "base_6_base_aware_witness",
        base: 6,
        middle_length: 1,
        outer: 1,
        inner: 5,
        k: FROM_K,
        comparison_to_k: FROM_K,
        modulus: 5,
        side: ComparisonSide::To,
        note: "Accessible bridge: 15451 in base 6 is 2551 decimal and prime.",
    },
];

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    main_bases: Vec<u32>,
    appendix_bases: Vec<u32>,
    middle_lengths: Vec<usize>,
    from_k: String,
    noncompact_lanes: Vec<String>,
    canonical_base: u32,
    canonical_modulus: u32,
}

#[derive(Debug, Clone, Serialize)]
struct TorusPhaseRow {
    multiplicative_order: u32,
    delta_mod_order: u32,
    phase: f64,
    row_count: usize,
    locked_count: usize,
    locked_share: f64,
    observed_gradient_equal_count: usize,
    observed_gradient_equal_share: f64,
    identity_count: usize,
    gradient_only_count: usize,
    mismatch_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct ConstructionWitnessRow {
    role: String,
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    pair_label: String,
    k_label: String,
    comparison_side: String,
    modulus: u32,
    multiplicative_order: u32,
    gradient_position_delta: i32,
    delta_mod_order: u32,
    local_relation_label: String,
    shift_modulus: u32,
    gradient_modulus: u32,
    zero_seed_class: u32,
    middle_index: u32,
    middle_digits: String,
    template_digits: String,
    decimal_value: String,
    prime: bool,
    affine_shift: String,
    affine_gradient: String,
    affine_shift_modulus: u32,
    affine_gradient_modulus: u32,
    note: String,
}

#[derive(Debug, Clone, Serialize)]
struct CanonicalWalkthrough {
    base: u32,
    modulus: u32,
    pair_label: String,
    from_k: String,
    to_k: String,
    multiplicative_order: u32,
    gradient_position_delta: i32,
    delta_mod_order: u32,
    gradient_equal: bool,
    shift_equal: bool,
    local_relation_label: String,
    compact_lane_equation: String,
    side_lane_equation: String,
    compact_lane_mod_relation: String,
    side_lane_mod_relation: String,
}

#[derive(Debug, Clone, Serialize)]
struct HeadlineMetrics {
    comparison_rows: usize,
    modulus_rows: usize,
    phase_cells: usize,
    locked_rows: usize,
    locked_share: f64,
    mismatch_rows: usize,
    mismatch_share: f64,
    canonical_relation: String,
    exact_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct SummaryJson {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    headline_metrics: HeadlineMetrics,
    canonical_walkthrough: CanonicalWalkthrough,
    construction_witness_rows: Vec<ConstructionWitnessRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    observations: Vec<String>,
}

#[derive(Debug, Clone)]
struct AffineFormula {
    shift: BigUint,
    gradient: BigUint,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let bundles = build_comparison_bundles();
    let modulus_rows = bundles
        .iter()
        .flat_map(|bundle| bundle.comparison.modulus_rows.iter())
        .collect::<Vec<_>>();
    let phase_rows = build_torus_phase_rows(&modulus_rows);
    let witness_rows = build_witness_rows();
    let canonical = build_canonical_walkthrough(&witness_rows);
    let headline_metrics = build_headline_metrics(&bundles, &phase_rows, &canonical);
    let observations = derive_observations(&phase_rows, &witness_rows, &canonical);

    let construction_template_path = options.out_dir.join("construction_template.png");
    render_construction_template(&witness_rows, &construction_template_path);
    let affine_line_path = options.out_dir.join("affine_line.png");
    render_affine_line(&witness_rows, &affine_line_path);
    let residue_filter_path = options.out_dir.join("residue_filter_wheel.png");
    render_residue_filter_wheel(&canonical, &witness_rows, &residue_filter_path);
    let residue_torus_path = options.out_dir.join("residue_torus.png");
    render_residue_torus(&phase_rows, &residue_torus_path);
    let unwrapped_torus_path = options.out_dir.join("unwrapped_torus.png");
    render_unwrapped_torus(&phase_rows, &unwrapped_torus_path);
    let walkthrough_path = options.out_dir.join("canonical_walkthrough.png");
    render_canonical_walkthrough(&canonical, &witness_rows, &walkthrough_path);
    let gallery_path = options.out_dir.join("example_gallery.png");
    render_example_gallery(&witness_rows, &gallery_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "construction_template".to_string(),
            label: "Visible symmetric zero-run construction diagram".to_string(),
            path: construction_template_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "affine_line".to_string(),
            label: "Affine seed line view".to_string(),
            path: affine_line_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "residue_filter_wheel".to_string(),
            label: "Residue filter wheel for canonical mod-5 lanes".to_string(),
            path: residue_filter_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "residue_torus".to_string(),
            label: "Residue torus phase view".to_string(),
            path: residue_torus_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "unwrapped_torus".to_string(),
            label: "Unwrapped order/residue phase chart".to_string(),
            path: unwrapped_torus_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "canonical_walkthrough".to_string(),
            label: "Base-22/mod-5 canonical walkthrough panel".to_string(),
            path: walkthrough_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "example_gallery".to_string(),
            label: "Compact prime construction gallery".to_string(),
            path: gallery_path.display().to_string(),
        },
    ];

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        main_bases: MAIN_BASES.to_vec(),
        appendix_bases: APPENDIX_BASES.to_vec(),
        middle_lengths: MIDDLE_LENGTHS.to_vec(),
        from_k: format_k(FROM_K),
        noncompact_lanes: NONCOMPACT_LANES.iter().map(|&k| format_k(k)).collect(),
        canonical_base: CANONICAL_BASE,
        canonical_modulus: CANONICAL_MODULUS,
    };
    let report_text = render_report(
        &settings,
        &headline_metrics,
        &canonical,
        &witness_rows,
        &observations,
    );
    let summary = SummaryJson {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        headline_metrics: headline_metrics.clone(),
        canonical_walkthrough: canonical,
        construction_witness_rows: witness_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        observations,
    };

    write_csv_rows(options.out_dir.join("torus_phase_rows.csv"), &phase_rows)
        .expect("write torus phase rows");
    write_csv_rows(
        options.out_dir.join("construction_witness_rows.csv"),
        &witness_rows,
    )
    .expect("write construction witness rows");
    write_json_pretty(options.out_dir.join("summary.json"), &summary).expect("write summary json");
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
                "residue_torus_period_lock_report".to_string(),
            ],
            upstream_inputs: vec![
                "src/validation/affine_period_lock.rs".to_string(),
                "src/validation/bounded_k.rs".to_string(),
            ],
            expected_outputs: vec![
                "report.md".to_string(),
                "summary.json".to_string(),
                "torus_phase_rows.csv".to_string(),
                "construction_witness_rows.csv".to_string(),
                "artifact_manifest.json".to_string(),
                "construction_template.png".to_string(),
                "affine_line.png".to_string(),
                "residue_filter_wheel.png".to_string(),
                "residue_torus.png".to_string(),
                "unwrapped_torus.png".to_string(),
                "canonical_walkthrough.png".to_string(),
                "example_gallery.png".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote residue torus period-lock bundle to {}",
        options.out_dir.display()
    );
    println!("{}", headline_metrics.exact_takeaway);
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
            NONCOMPACT_LANES
                .iter()
                .copied()
                .map(|to_k| ComparisonBundle {
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

fn build_torus_phase_rows(rows: &[&AffinePeriodLockModulusRow]) -> Vec<TorusPhaseRow> {
    #[derive(Default)]
    struct Accumulator {
        row_count: usize,
        locked_count: usize,
        observed_gradient_equal_count: usize,
        identity_count: usize,
        gradient_only_count: usize,
        mismatch_count: usize,
    }

    let mut accumulators: BTreeMap<(u32, u32), Accumulator> = BTreeMap::new();
    for row in rows {
        let key = (row.multiplicative_order, row.delta_mod_order);
        let entry = accumulators.entry(key).or_default();
        entry.row_count += 1;
        entry.locked_count += usize::from(row.period_lock_expected);
        entry.observed_gradient_equal_count += usize::from(row.observed_gradient_equal);
        entry.identity_count += usize::from(row.local_relation_label == "identity");
        entry.gradient_only_count += usize::from(row.local_relation_label == "gradient_only");
        entry.mismatch_count += usize::from(!row.expected_matches_observation);
    }

    accumulators
        .into_iter()
        .map(|((multiplicative_order, delta_mod_order), acc)| {
            let phase = if multiplicative_order == 0 {
                0.0
            } else {
                delta_mod_order as f64 / multiplicative_order as f64
            };
            TorusPhaseRow {
                multiplicative_order,
                delta_mod_order,
                phase,
                row_count: acc.row_count,
                locked_count: acc.locked_count,
                locked_share: ratio(acc.locked_count, acc.row_count),
                observed_gradient_equal_count: acc.observed_gradient_equal_count,
                observed_gradient_equal_share: ratio(
                    acc.observed_gradient_equal_count,
                    acc.row_count,
                ),
                identity_count: acc.identity_count,
                gradient_only_count: acc.gradient_only_count,
                mismatch_count: acc.mismatch_count,
            }
        })
        .collect()
}

fn build_witness_rows() -> Vec<ConstructionWitnessRow> {
    WITNESS_SPECS.iter().map(build_witness_row).collect()
}

fn build_witness_row(spec: &WitnessSpec) -> ConstructionWitnessRow {
    let comparison = scan_k_config_affine_period_lock_comparison(
        spec.base,
        spec.middle_length,
        spec.outer,
        spec.inner,
        FROM_K,
        spec.comparison_to_k,
    );
    let modulus_row = comparison
        .modulus_rows
        .iter()
        .find(|row| row.modulus == spec.modulus)
        .unwrap_or_else(|| panic!("missing modulus {} for {}", spec.modulus, spec.role));
    let (_, examples) = scan_k_config_examples(
        spec.base,
        spec.middle_length,
        spec.outer,
        spec.inner,
        spec.k,
        1,
    );
    let example = examples
        .first()
        .unwrap_or_else(|| panic!("missing prime example for {}", spec.role));
    let formula = affine_formula(
        spec.base,
        spec.middle_length,
        spec.outer,
        spec.inner,
        spec.k,
    );
    let template_digits = template_digits(
        spec.base,
        spec.middle_length,
        spec.outer,
        spec.inner,
        spec.k,
        example.middle_index,
    );
    let (comparison_side, shift_modulus, gradient_modulus, zero_seed_class) = match spec.side {
        ComparisonSide::From => (
            "from",
            modulus_row.shift_modulus_from,
            modulus_row.gradient_modulus_from,
            modulus_row.zero_seed_class_from,
        ),
        ComparisonSide::To => (
            "to",
            modulus_row.shift_modulus_to,
            modulus_row.gradient_modulus_to,
            modulus_row.zero_seed_class_to,
        ),
    };

    ConstructionWitnessRow {
        role: spec.role.to_string(),
        base: spec.base,
        middle_length: spec.middle_length,
        outer: spec.outer,
        inner: spec.inner,
        pair_label: format!(
            "({},{})",
            digit_symbol(spec.outer),
            digit_symbol(spec.inner)
        ),
        k_label: format_k(spec.k),
        comparison_side: comparison_side.to_string(),
        modulus: modulus_row.modulus,
        multiplicative_order: modulus_row.multiplicative_order,
        gradient_position_delta: modulus_row.gradient_position_delta,
        delta_mod_order: modulus_row.delta_mod_order,
        local_relation_label: modulus_row.local_relation_label.clone(),
        shift_modulus,
        gradient_modulus,
        zero_seed_class,
        middle_index: example.middle_index,
        middle_digits: example.middle_digits.clone(),
        template_digits,
        decimal_value: example.decimal_value.clone(),
        prime: true,
        affine_shift: formula.shift.to_string(),
        affine_gradient: formula.gradient.to_string(),
        affine_shift_modulus: (&formula.shift % modulus_row.modulus).try_into().unwrap(),
        affine_gradient_modulus: (&formula.gradient % modulus_row.modulus)
            .try_into()
            .unwrap(),
        note: spec.note.to_string(),
    }
}

fn build_canonical_walkthrough(witness_rows: &[ConstructionWitnessRow]) -> CanonicalWalkthrough {
    let compact = witness_rows
        .iter()
        .find(|row| row.role == "canonical_compact_lane")
        .expect("compact canonical witness row");
    let side = witness_rows
        .iter()
        .find(|row| row.role == "base_22_higher_order_side_pocket")
        .expect("side canonical witness row");

    CanonicalWalkthrough {
        base: CANONICAL_BASE,
        modulus: CANONICAL_MODULUS,
        pair_label: compact.pair_label.clone(),
        from_k: format_k(FROM_K),
        to_k: format_k(CANONICAL_TO_K),
        multiplicative_order: side.multiplicative_order,
        gradient_position_delta: side.gradient_position_delta,
        delta_mod_order: side.delta_mod_order,
        gradient_equal: compact.gradient_modulus == side.gradient_modulus,
        shift_equal: compact.shift_modulus == side.shift_modulus,
        local_relation_label: side.local_relation_label.clone(),
        compact_lane_equation: format!(
            "N(s) = {} + {}*s",
            compact.affine_shift, compact.affine_gradient
        ),
        side_lane_equation: format!("N(s) = {} + {}*s", side.affine_shift, side.affine_gradient),
        compact_lane_mod_relation: format!(
            "N(s) = {} + {}*s (mod {})",
            compact.affine_shift_modulus, compact.affine_gradient_modulus, compact.modulus
        ),
        side_lane_mod_relation: format!(
            "N(s) = {} + {}*s (mod {})",
            side.affine_shift_modulus, side.affine_gradient_modulus, side.modulus
        ),
    }
}

fn build_headline_metrics(
    bundles: &[ComparisonBundle],
    phase_rows: &[TorusPhaseRow],
    canonical: &CanonicalWalkthrough,
) -> HeadlineMetrics {
    let modulus_rows = bundles
        .iter()
        .map(|bundle| bundle.comparison.modulus_rows.len())
        .sum::<usize>();
    let locked_rows = phase_rows.iter().map(|row| row.locked_count).sum::<usize>();
    let mismatch_rows = phase_rows
        .iter()
        .map(|row| row.mismatch_count)
        .sum::<usize>();
    let exact_takeaway = if mismatch_rows == 0 {
        "period lock exactly matches observed affine gradient equality on the maintained coprime-modulus surface".to_string()
    } else {
        format!(
            "period lock misses {mismatch_rows} of {modulus_rows} modulus rows on the maintained coprime-modulus surface"
        )
    };

    HeadlineMetrics {
        comparison_rows: bundles.len(),
        modulus_rows,
        phase_cells: phase_rows.len(),
        locked_rows,
        locked_share: ratio(locked_rows, modulus_rows),
        mismatch_rows,
        mismatch_share: ratio(mismatch_rows, modulus_rows),
        canonical_relation: canonical.local_relation_label.clone(),
        exact_takeaway,
    }
}

fn derive_observations(
    phase_rows: &[TorusPhaseRow],
    witness_rows: &[ConstructionWitnessRow],
    canonical: &CanonicalWalkthrough,
) -> Vec<String> {
    let lock_meridian_rows = phase_rows
        .iter()
        .filter(|row| row.delta_mod_order == 0)
        .map(|row| row.row_count)
        .sum::<usize>();
    let off_meridian_gradient_rows = phase_rows
        .iter()
        .filter(|row| row.delta_mod_order != 0)
        .map(|row| row.observed_gradient_equal_count)
        .sum::<usize>();
    let side = witness_rows
        .iter()
        .find(|row| row.role == "base_22_higher_order_side_pocket")
        .expect("side witness row");
    let decimal = witness_rows
        .iter()
        .find(|row| row.role == "decimal_visible_zero_run_k21")
        .expect("decimal visible witness row");

    vec![
        format!(
            "The visible decimal witness {} shows the maintained construction grammar directly: mirrored zero-run lengths around a center block.",
            decimal.template_digits
        ),
        format!(
            "The torus lock lives on residue class 0: {lock_meridian_rows} scanned modulus rows sit on that meridian, and {off_meridian_gradient_rows} off-meridian rows show gradient equality."
        ),
        format!(
            "The canonical base-22/mod-5 pocket has ord_5(22) = {}, gradient-position delta {}, and delta mod order {}; the gradients match while shifts differ.",
            canonical.multiplicative_order,
            canonical.gradient_position_delta,
            canonical.delta_mod_order
        ),
        format!(
            "The side-pocket witness is {} (base {}) = {} decimal, found by the repo scanner at seed {}.",
            side.template_digits, side.base, side.decimal_value, side.middle_index
        ),
        "Prime witnesses demonstrate that the construction is real; they do not by themselves claim a global prime-density theorem.".to_string(),
    ]
}

fn render_report(
    settings: &ReportSettings,
    metrics: &HeadlineMetrics,
    canonical: &CanonicalWalkthrough,
    witness_rows: &[ConstructionWitnessRow],
    observations: &[String],
) -> String {
    let mut lines = Vec::new();
    lines.push("# Residue Torus Period-Lock Report".to_string());
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
    lines.push("## Affine Core".to_string());
    lines.push("- We are not visualizing where primes magically appear; we are visualizing the arithmetic surface that a fixed symmetric template forces every candidate to live on.".to_string());
    lines.push("- Once the base, boundary digits, middle length, and `k` lane are fixed, the seed is the only moving part.".to_string());
    lines.push("- Each lane is `N(s) = A + G*s`; modulo a coprime prime `p`, it becomes `N(s) = A_p + G_p*s`.".to_string());
    lines.push("- The residue torus visualizes when two such lanes have the same local gradient, while the shift decides whether the relation is `identity` or `gradient_only`.".to_string());
    lines.push(String::new());
    lines.push("## Construction Grammar".to_string());
    lines.push("The maintained membrane construction is:".to_string());
    lines.push(String::new());
    lines.push("```text".to_string());
    lines.push(
        "outer + 0^k_outer + inner + 0^k_inner + seed + 0^k_inner + inner + 0^k_outer + outer"
            .to_string(),
    );
    lines.push("```".to_string());
    lines.push(String::new());
    lines.push("- The zero run lengths are symmetric around the seed.".to_string());
    lines.push(
        "- In the current maintained grammar, the boundary digits are mirrored too.".to_string(),
    );
    lines.push("- A number with symmetric zero runs but non-mirrored boundary digits is a nearby scaffold generalization, not this exact membrane lane.".to_string());
    lines.push("- For `M > 1`, the middle seed block does not need to be palindromic; this is why these are not just palindromic primes.".to_string());
    lines.push(String::new());
    lines.push("## Visual Chain".to_string());
    lines.push("| Step | Visual artifact | What it teaches |".to_string());
    lines.push("|---|---|---|".to_string());
    lines.push("| construction | `construction_template.png` | where the mirrored zero runs and seed block live |".to_string());
    lines.push(
        "| affine line | `affine_line.png` | fixing the template leaves `N(s)=A+G*s` |".to_string(),
    );
    lines.push("| residue filters | `residue_filter_wheel.png` | each coprime modulus excludes an exact seed class |".to_string());
    lines.push("| lane comparison | `canonical_walkthrough.png` | same gradient plus different shift gives `gradient_only` |".to_string());
    lines.push("| period lock | `unwrapped_torus.png` | gradient agreement is `delta = 0 mod ord_p(base)` |".to_string());
    lines.push("| torus visualization | `residue_torus.png` | the lock meridian is the cyclic residue-0 line |".to_string());
    lines.push("| prime witnesses | `example_gallery.png` | the construction produces real prime examples |".to_string());
    lines.push(String::new());
    lines.push("## What The Torus Means".to_string());
    lines.push(
        "- A bounded template lane is an affine seed line: `N(s) = shift + gradient * s`."
            .to_string(),
    );
    lines.push("- For each coprime modulus, the gradient is a power of the base. Moving from one k-lane to another changes the exponent by a position delta.".to_string());
    lines.push("- The torus unwraps that cyclic condition: the horizontal phase is `delta mod ord_p(base)`, and the lock meridian is phase 0.".to_string());
    lines.push("- Points on the lock meridian have matching affine gradients. Whether they become identity or `gradient_only` depends on the shift residue.".to_string());
    lines.push(String::new());
    lines.push("## Headline".to_string());
    lines.push(format!("- {}", metrics.exact_takeaway));
    lines.push(format!(
        "- comparison rows: {}, modulus rows: {}, phase cells: {}",
        metrics.comparison_rows, metrics.modulus_rows, metrics.phase_cells
    ));
    lines.push(format!(
        "- locked rows: {} ({:.2}%), mismatches: {} ({:.2}%)",
        metrics.locked_rows,
        metrics.locked_share * 100.0,
        metrics.mismatch_rows,
        metrics.mismatch_share * 100.0
    ));
    lines.push(String::new());
    lines.push("## Canonical Walkthrough".to_string());
    lines.push(format!(
        "- pocket: base {}, mod {}, pair {}, M=2, `{}` -> `{}`",
        canonical.base, canonical.modulus, canonical.pair_label, canonical.from_k, canonical.to_k
    ));
    lines.push(format!(
        "- `ord_{}({}) = {}`, position delta = {}, delta mod order = {}",
        canonical.modulus,
        canonical.base,
        canonical.multiplicative_order,
        canonical.gradient_position_delta,
        canonical.delta_mod_order
    ));
    lines.push(format!(
        "- compact lane: `{}`",
        canonical.compact_lane_equation
    ));
    lines.push(format!("- side lane: `{}`", canonical.side_lane_equation));
    lines.push(format!(
        "- compact residue: `{}`",
        canonical.compact_lane_mod_relation
    ));
    lines.push(format!(
        "- side residue: `{}`",
        canonical.side_lane_mod_relation
    ));
    lines.push(format!(
        "- reading: gradient equality is `{}`, shift equality is `{}`, so the local relation is `{}`.",
        canonical.gradient_equal, canonical.shift_equal, canonical.local_relation_label
    ));
    lines.push(String::new());
    lines.push("## Example Gallery".to_string());
    lines.push("| Role | Template | Decimal prime | Lane | Local relation |".to_string());
    lines.push("|---|---:|---:|---|---|".to_string());
    for row in witness_rows {
        lines.push(format!(
            "| `{}` | `{}` base {} | `{}` | `{}` | `{}` |",
            row.role,
            row.template_digits,
            row.base,
            row.decimal_value,
            row.k_label,
            row.local_relation_label
        ));
    }
    lines.push(String::new());
    lines.push("## Observations".to_string());
    for observation in observations {
        lines.push(format!("- {}", observation));
    }
    lines.push(String::new());
    lines.push("## What This Does Not Claim".to_string());
    lines.push("- It does not claim a new density theorem.".to_string());
    lines.push("- It does not claim that gradient agreement alone generates primes.".to_string());
    lines.push("- It claims an exact local affine decomposition: period lock explains when affine gradients can agree; prime generation still depends on residue filters, shift alignment, and ordinary primality.".to_string());
    lines.push(String::new());
    lines.push("## Artifacts".to_string());
    lines.push(
        "- `torus_phase_rows.csv`: order/residue aggregate rows for the residue torus.".to_string(),
    );
    lines.push("- `construction_witness_rows.csv`: selected construction witnesses computed from repo code.".to_string());
    lines.push("- `construction_template.png`, `affine_line.png`, `residue_filter_wheel.png`, `residue_torus.png`, `unwrapped_torus.png`, `canonical_walkthrough.png`, `example_gallery.png`: visual reading aids.".to_string());
    lines.join("\n")
}

fn render_construction_template(rows: &[ConstructionWitnessRow], path: &Path) {
    let row = rows
        .iter()
        .find(|row| row.role == "decimal_visible_zero_run_k21")
        .expect("visible decimal construction row");
    let root = BitMapBackend::new(path, (1500, 760)).into_drawing_area();
    root.fill(&RGBColor(250, 249, 246)).unwrap();

    root.draw(&Text::new(
        "Construction grammar: mirrored zero runs around a seed",
        (60, 72),
        ("sans-serif", 34).into_font().color(&RGBColor(38, 44, 51)),
    ))
    .unwrap();
    root.draw(&Text::new(
        "outer + 0^k_outer + inner + 0^k_inner + seed + 0^k_inner + inner + 0^k_outer + outer",
        (60, 122),
        ("monospace", 22).into_font().color(&RGBColor(82, 88, 96)),
    ))
    .unwrap();

    let segments = [
        ("outer", "3", 110, RGBColor(47, 103, 168)),
        ("k_outer", "00", 145, RGBColor(215, 222, 230)),
        ("inner", "7", 110, RGBColor(24, 118, 117)),
        ("k_inner", "0", 130, RGBColor(225, 230, 235)),
        ("seed", &row.middle_digits, 145, RGBColor(191, 61, 56)),
        ("k_inner", "0", 130, RGBColor(225, 230, 235)),
        ("inner", "7", 110, RGBColor(24, 118, 117)),
        ("k_outer", "00", 145, RGBColor(215, 222, 230)),
        ("outer", "3", 110, RGBColor(47, 103, 168)),
    ];

    let mut x = 80;
    let y = 250;
    for (label, value, width, color) in segments {
        draw_template_segment(&root, (x, y), width, label, value, color);
        x += width + 12;
    }

    root.draw(&PathElement::new(
        vec![(225, 470), (1020, 470)],
        ShapeStyle::from(&RGBColor(82, 88, 96)).stroke_width(2),
    ))
    .unwrap();
    root.draw(&Text::new(
        "the zero-run lengths mirror: 2, then 1, then seed, then 1, then 2",
        (265, 510),
        ("sans-serif", 22).into_font().color(&RGBColor(82, 88, 96)),
    ))
    .unwrap();
    root.draw(&Text::new(
        format!(
            "{} base {} = {} decimal (prime)",
            row.template_digits, row.base, row.decimal_value
        ),
        (60, 620),
        ("monospace", 27).into_font().color(&RGBColor(24, 118, 117)),
    ))
    .unwrap();
    root.present().unwrap();
}

fn draw_template_segment<DB: DrawingBackend>(
    root: &DrawingArea<DB, plotters::coord::Shift>,
    origin: (i32, i32),
    width: i32,
    label: &str,
    value: &str,
    color: RGBColor,
) {
    let (x, y) = origin;
    root.draw(&Rectangle::new(
        [(x, y), (x + width, y + 130)],
        ShapeStyle::from(&WHITE).filled(),
    ))
    .unwrap();
    root.draw(&Rectangle::new(
        [(x, y), (x + width, y + 130)],
        ShapeStyle::from(&color).stroke_width(3),
    ))
    .unwrap();
    root.draw(&Text::new(
        value.to_string(),
        (x + 24, y + 54),
        ("monospace", 34).into_font().color(&RGBColor(38, 44, 51)),
    ))
    .unwrap();
    root.draw(&Text::new(
        label.to_string(),
        (x + 16, y + 104),
        ("sans-serif", 17).into_font().color(&RGBColor(82, 88, 96)),
    ))
    .unwrap();
}

fn render_affine_line(rows: &[ConstructionWitnessRow], path: &Path) {
    let row = rows
        .iter()
        .find(|row| row.role == "decimal_visible_zero_run_k21")
        .expect("visible decimal construction row");
    let shift = row.affine_shift.parse::<f64>().unwrap();
    let gradient = row.affine_gradient.parse::<f64>().unwrap();
    let y_min = shift;
    let y_max = shift + gradient * 99.0;
    let (_, examples) = scan_k_config_examples(10, 2, 3, 7, (2, 1), 5);

    let root = BitMapBackend::new(path, (1300, 820)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Affine seed line after the template is fixed",
            ("sans-serif", 30),
        )
        .margin(35)
        .x_label_area_size(55)
        .y_label_area_size(95)
        .build_cartesian_2d(0u32..99u32, y_min..y_max)
        .unwrap();
    chart
        .configure_mesh()
        .x_desc("seed s")
        .y_desc("candidate N(s)")
        .light_line_style(RGBColor(232, 235, 238))
        .draw()
        .unwrap();
    chart
        .draw_series(LineSeries::new(
            (0u32..=99u32).map(|seed| (seed, shift + gradient * seed as f64)),
            ShapeStyle::from(&RGBColor(47, 103, 168)).stroke_width(4),
        ))
        .unwrap();
    chart
        .draw_series(examples.iter().map(|example| {
            let y = example.decimal_value.parse::<f64>().unwrap();
            Circle::new(
                (example.middle_index, y),
                7,
                ShapeStyle::from(&RGBColor(191, 61, 56)).filled(),
            )
        }))
        .unwrap();
    root.draw(&Text::new(
        format!(
            "{}: N(s) = {} + {}*s",
            row.k_label, row.affine_shift, row.affine_gradient
        ),
        (90, 745),
        ("monospace", 23).into_font().color(&RGBColor(38, 44, 51)),
    ))
    .unwrap();
    root.draw(&Text::new(
        "red points are the first prime witnesses found on this decimal teaching lane",
        (90, 780),
        ("sans-serif", 19).into_font().color(&RGBColor(82, 88, 96)),
    ))
    .unwrap();
    root.present().unwrap();
}

fn render_residue_filter_wheel(
    canonical: &CanonicalWalkthrough,
    rows: &[ConstructionWitnessRow],
    path: &Path,
) {
    let compact = rows
        .iter()
        .find(|row| row.role == "canonical_compact_lane")
        .expect("compact canonical row");
    let side = rows
        .iter()
        .find(|row| row.role == "base_22_higher_order_side_pocket")
        .expect("side canonical row");
    let root = BitMapBackend::new(path, (1300, 780)).into_drawing_area();
    root.fill(&RGBColor(250, 249, 246)).unwrap();
    root.draw(&Text::new(
        "Residue filters: each affine lane excludes an exact seed class",
        (60, 70),
        ("sans-serif", 32).into_font().color(&RGBColor(38, 44, 51)),
    ))
    .unwrap();
    root.draw(&Text::new(
        "The canonical lanes have the same mod-5 gradient but different shift, so the excluded seed class moves.",
        (60, 116),
        ("sans-serif", 20).into_font().color(&RGBColor(82, 88, 96)),
    ))
    .unwrap();

    draw_residue_wheel(
        &root,
        (360, 420),
        "k=(0,0)",
        &canonical.compact_lane_mod_relation,
        compact.zero_seed_class,
        RGBColor(47, 103, 168),
    );
    draw_residue_wheel(
        &root,
        (930, 420),
        "k=(2,2)",
        &canonical.side_lane_mod_relation,
        side.zero_seed_class,
        RGBColor(24, 118, 117),
    );
    root.draw(&Text::new(
        "same step 4*s mod 5; different starts 3 vs 4",
        (395, 705),
        ("sans-serif", 25).into_font().color(&RGBColor(38, 44, 51)),
    ))
    .unwrap();
    root.present().unwrap();
}

fn draw_residue_wheel<DB: DrawingBackend>(
    root: &DrawingArea<DB, plotters::coord::Shift>,
    center: (i32, i32),
    title: &str,
    equation: &str,
    excluded_class: u32,
    accent: RGBColor,
) {
    root.draw(&Text::new(
        title.to_string(),
        (center.0 - 95, center.1 - 220),
        ("sans-serif", 27).into_font().color(&RGBColor(38, 44, 51)),
    ))
    .unwrap();
    root.draw(&Text::new(
        equation.to_string(),
        (center.0 - 165, center.1 - 178),
        ("monospace", 21).into_font().color(&RGBColor(82, 88, 96)),
    ))
    .unwrap();
    root.draw(&Circle::new(
        center,
        145,
        ShapeStyle::from(&RGBColor(205, 209, 214)).stroke_width(2),
    ))
    .unwrap();
    for residue in 0..5u32 {
        let angle = std::f64::consts::PI / 2.0 - residue as f64 / 5.0 * 2.0 * std::f64::consts::PI;
        let x = center.0 as f64 + angle.cos() * 145.0;
        let y = center.1 as f64 - angle.sin() * 145.0;
        let color = if residue == excluded_class {
            RGBColor(191, 61, 56)
        } else {
            accent
        };
        root.draw(&Circle::new(
            (x.round() as i32, y.round() as i32),
            25,
            ShapeStyle::from(&color).filled(),
        ))
        .unwrap();
        root.draw(&Text::new(
            residue.to_string(),
            (x.round() as i32 - 7, y.round() as i32 + 8),
            ("sans-serif", 22).into_font().color(&WHITE),
        ))
        .unwrap();
    }
    root.draw(&Text::new(
        format!("excluded seed class: {excluded_class}"),
        (center.0 - 132, center.1 + 205),
        ("sans-serif", 21).into_font().color(&RGBColor(191, 61, 56)),
    ))
    .unwrap();
}

fn render_residue_torus(rows: &[TorusPhaseRow], path: &Path) {
    let root = BitMapBackend::new(path, (1200, 980)).into_drawing_area();
    root.fill(&RGBColor(250, 249, 246)).unwrap();

    let max_order = rows
        .iter()
        .map(|row| row.multiplicative_order)
        .max()
        .unwrap_or(1) as f64;
    let center = (600i32, 520i32);
    let max_radius = 370.0;

    root.draw(&Text::new(
        "Residue Torus: period lock lives on the residue-0 meridian",
        (70, 70),
        ("sans-serif", 32).into_font().color(&RGBColor(38, 44, 51)),
    ))
    .unwrap();
    root.draw(&Text::new(
        "angle = delta mod ord_p(base); radius = multiplicative order",
        (70, 112),
        ("sans-serif", 20).into_font().color(&RGBColor(82, 88, 96)),
    ))
    .unwrap();

    for fraction in [0.25_f64, 0.5, 0.75, 1.0] {
        let radius = (max_radius * fraction).round() as i32;
        root.draw(&Circle::new(
            center,
            radius,
            ShapeStyle::from(&RGBColor(205, 209, 214)).stroke_width(1),
        ))
        .unwrap();
    }
    root.draw(&PathElement::new(
        vec![
            (center.0, (center.1 as f64 - max_radius) as i32),
            (center.0, (center.1 as f64 + max_radius) as i32),
        ],
        ShapeStyle::from(&RGBColor(191, 61, 56)).stroke_width(3),
    ))
    .unwrap();
    root.draw(&Text::new(
        "lock meridian",
        (center.0 + 14, (center.1 as f64 - max_radius) as i32 + 8),
        ("sans-serif", 18).into_font().color(&RGBColor(150, 42, 39)),
    ))
    .unwrap();

    for row in rows {
        let order_scale = row.multiplicative_order as f64 / max_order;
        let radius = 45.0 + order_scale * (max_radius - 45.0);
        let angle = std::f64::consts::PI / 2.0 - row.phase * 2.0 * std::f64::consts::PI;
        let x = center.0 as f64 + angle.cos() * radius;
        let y = center.1 as f64 - angle.sin() * radius;
        let point_radius = 4 + (row.row_count as f64).sqrt().round() as i32;
        let color = if row.locked_share > 0.0 {
            RGBColor(24, 118, 117)
        } else if row.mismatch_count > 0 {
            RGBColor(196, 75, 64)
        } else {
            RGBColor(124, 139, 153)
        };
        root.draw(&Circle::new(
            (x.round() as i32, y.round() as i32),
            point_radius,
            ShapeStyle::from(&color.mix(0.84)).filled(),
        ))
        .unwrap();
    }

    root.draw(&Text::new(
        "A point is a residue phase cell aggregated over scanned bases, pairs, M, and k-lanes.",
        (70, 910),
        ("sans-serif", 18).into_font().color(&RGBColor(82, 88, 96)),
    ))
    .unwrap();
    root.present().unwrap();
}

fn render_unwrapped_torus(rows: &[TorusPhaseRow], path: &Path) {
    let root = BitMapBackend::new(path, (1200, 760)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let max_order = rows
        .iter()
        .map(|row| row.multiplicative_order)
        .max()
        .unwrap_or(1);
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Unwrapped residue torus: phase by multiplicative order",
            ("sans-serif", 30),
        )
        .margin(35)
        .x_label_area_size(50)
        .y_label_area_size(70)
        .build_cartesian_2d(0f64..1f64, 0u32..(max_order + 1))
        .unwrap();
    chart
        .configure_mesh()
        .x_desc("phase = delta mod order / order")
        .y_desc("ord_p(base)")
        .light_line_style(RGBColor(232, 235, 238))
        .draw()
        .unwrap();
    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(0.0, 0), (0.0, max_order + 1)],
            ShapeStyle::from(&RGBColor(191, 61, 56)).stroke_width(3),
        )))
        .unwrap();
    chart
        .draw_series(rows.iter().map(|row| {
            let color = if row.locked_share > 0.0 {
                RGBColor(24, 118, 117)
            } else {
                RGBColor(92, 107, 122)
            };
            let size = 3 + (row.row_count as f64).sqrt().round() as i32;
            Circle::new(
                (row.phase, row.multiplicative_order),
                size,
                ShapeStyle::from(&color.mix(0.78)).filled(),
            )
        }))
        .unwrap();
    root.present().unwrap();
}

fn render_canonical_walkthrough(
    canonical: &CanonicalWalkthrough,
    witness_rows: &[ConstructionWitnessRow],
    path: &Path,
) {
    let root = BitMapBackend::new(path, (1400, 860)).into_drawing_area();
    root.fill(&RGBColor(250, 249, 246)).unwrap();
    let compact = witness_rows
        .iter()
        .find(|row| row.role == "canonical_compact_lane")
        .expect("compact row");
    let side = witness_rows
        .iter()
        .find(|row| row.role == "base_22_higher_order_side_pocket")
        .expect("side row");

    root.draw(&Text::new(
        "Canonical walkthrough: base-22 / mod-5 period lock",
        (60, 70),
        ("sans-serif", 34).into_font().color(&RGBColor(38, 44, 51)),
    ))
    .unwrap();
    root.draw(&Text::new(
        format!(
            "ord_{}({}) = {}; delta = {}; delta mod order = {}",
            canonical.modulus,
            canonical.base,
            canonical.multiplicative_order,
            canonical.gradient_position_delta,
            canonical.delta_mod_order
        ),
        (60, 116),
        ("sans-serif", 22).into_font().color(&RGBColor(82, 88, 96)),
    ))
    .unwrap();

    draw_lane_panel(
        &root,
        (80, 190),
        "compact lane k=(0,0)",
        &canonical.compact_lane_equation,
        &canonical.compact_lane_mod_relation,
        &format!(
            "{} = {} decimal",
            compact.template_digits, compact.decimal_value
        ),
        RGBColor(47, 103, 168),
    );
    draw_lane_panel(
        &root,
        (80, 500),
        "side lane k=(2,2)",
        &canonical.side_lane_equation,
        &canonical.side_lane_mod_relation,
        &format!("{} = {} decimal", side.template_digits, side.decimal_value),
        RGBColor(24, 118, 117),
    );

    root.draw(&PathElement::new(
        vec![(830, 300), (1160, 300)],
        ShapeStyle::from(&RGBColor(47, 103, 168)).stroke_width(5),
    ))
    .unwrap();
    root.draw(&PathElement::new(
        vec![(830, 610), (1160, 610)],
        ShapeStyle::from(&RGBColor(24, 118, 117)).stroke_width(5),
    ))
    .unwrap();
    root.draw(&Text::new(
        "same gradient mod 5",
        (875, 382),
        ("sans-serif", 24).into_font().color(&RGBColor(38, 44, 51)),
    ))
    .unwrap();
    root.draw(&Text::new(
        "different shift mod 5",
        (865, 430),
        ("sans-serif", 24).into_font().color(&RGBColor(191, 61, 56)),
    ))
    .unwrap();
    root.draw(&Text::new(
        "local relation: gradient_only",
        (850, 486),
        ("sans-serif", 26).into_font().color(&RGBColor(38, 44, 51)),
    ))
    .unwrap();
    root.present().unwrap();
}

fn draw_lane_panel<DB: DrawingBackend>(
    root: &DrawingArea<DB, plotters::coord::Shift>,
    origin: (i32, i32),
    title: &str,
    equation: &str,
    residue: &str,
    witness: &str,
    accent: RGBColor,
) {
    let (x, y) = origin;
    root.draw(&Rectangle::new(
        [(x, y), (x + 640, y + 230)],
        ShapeStyle::from(&WHITE).filled(),
    ))
    .unwrap();
    root.draw(&Rectangle::new(
        [(x, y), (x + 640, y + 230)],
        ShapeStyle::from(&RGBColor(205, 209, 214)).stroke_width(1),
    ))
    .unwrap();
    root.draw(&Rectangle::new(
        [(x, y), (x + 10, y + 230)],
        ShapeStyle::from(&accent).filled(),
    ))
    .unwrap();
    root.draw(&Text::new(
        title.to_string(),
        (x + 28, y + 44),
        ("sans-serif", 25).into_font().color(&RGBColor(38, 44, 51)),
    ))
    .unwrap();
    root.draw(&Text::new(
        equation.to_string(),
        (x + 28, y + 92),
        ("sans-serif", 22).into_font().color(&RGBColor(38, 44, 51)),
    ))
    .unwrap();
    root.draw(&Text::new(
        residue.to_string(),
        (x + 28, y + 136),
        ("sans-serif", 22).into_font().color(&RGBColor(82, 88, 96)),
    ))
    .unwrap();
    root.draw(&Text::new(
        witness.to_string(),
        (x + 28, y + 184),
        ("sans-serif", 20).into_font().color(&RGBColor(82, 88, 96)),
    ))
    .unwrap();
}

fn render_example_gallery(rows: &[ConstructionWitnessRow], path: &Path) {
    let height = 190 + rows.len() as u32 * 132 + 70;
    let root = BitMapBackend::new(path, (1500, height)).into_drawing_area();
    root.fill(&RGBColor(250, 249, 246)).unwrap();
    root.draw(&Text::new(
        "Construction witness gallery",
        (60, 70),
        ("sans-serif", 34).into_font().color(&RGBColor(38, 44, 51)),
    ))
    .unwrap();
    root.draw(&Text::new(
        "Each witness is computed from the repo scanner; primality is example evidence, not a density claim.",
        (60, 112),
        ("sans-serif", 20).into_font().color(&RGBColor(82, 88, 96)),
    ))
    .unwrap();

    let mut y = 170;
    for row in rows {
        root.draw(&Rectangle::new(
            [(60, y), (1440, y + 110)],
            ShapeStyle::from(&WHITE).filled(),
        ))
        .unwrap();
        root.draw(&Rectangle::new(
            [(60, y), (1440, y + 110)],
            ShapeStyle::from(&RGBColor(210, 214, 218)).stroke_width(1),
        ))
        .unwrap();
        root.draw(&Text::new(
            row.role.replace('_', " "),
            (88, y + 34),
            ("sans-serif", 22).into_font().color(&RGBColor(38, 44, 51)),
        ))
        .unwrap();
        root.draw(&Text::new(
            format!(
                "base {} {}, M={}, {}, mod {} -> {}",
                row.base,
                row.pair_label,
                row.middle_length,
                row.k_label,
                row.modulus,
                row.local_relation_label
            ),
            (88, y + 70),
            ("sans-serif", 18).into_font().color(&RGBColor(82, 88, 96)),
        ))
        .unwrap();
        root.draw(&Text::new(
            format!("{} = {}", row.template_digits, row.decimal_value),
            (650, y + 46),
            ("monospace", 24).into_font().color(&RGBColor(24, 118, 117)),
        ))
        .unwrap();
        root.draw(&Text::new(
            format!("seed {} ({})", row.middle_index, row.middle_digits),
            (650, y + 80),
            ("sans-serif", 18).into_font().color(&RGBColor(82, 88, 96)),
        ))
        .unwrap();
        y += 132;
    }
    root.present().unwrap();
}

fn template_digits(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    (k_outer, k_inner): BoundedKConfig,
    middle_index: u32,
) -> String {
    let mut digits = String::new();
    digits.push_str(&digit_symbol(outer));
    digits.extend(std::iter::repeat_n('0', k_outer as usize));
    digits.push_str(&digit_symbol(inner));
    digits.extend(std::iter::repeat_n('0', k_inner as usize));
    digits.push_str(&to_base_string_fixed(middle_index, base, middle_length));
    digits.extend(std::iter::repeat_n('0', k_inner as usize));
    digits.push_str(&digit_symbol(inner));
    digits.extend(std::iter::repeat_n('0', k_outer as usize));
    digits.push_str(&digit_symbol(outer));
    digits
}

fn affine_formula(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    (k_outer, k_inner): BoundedKConfig,
) -> AffineFormula {
    let prefix_digits = {
        let mut digits = Vec::with_capacity((2 + k_outer + k_inner) as usize);
        digits.push(outer);
        digits.extend(std::iter::repeat_n(0, k_outer as usize));
        digits.push(inner);
        digits.extend(std::iter::repeat_n(0, k_inner as usize));
        digits
    };
    let suffix_digits = {
        let mut digits = Vec::with_capacity((2 + k_outer + k_inner) as usize);
        digits.extend(std::iter::repeat_n(0, k_inner as usize));
        digits.push(inner);
        digits.extend(std::iter::repeat_n(0, k_outer as usize));
        digits.push(outer);
        digits
    };
    let suffix_len = suffix_digits.len() as u32;
    let base_big = BigUint::from(base);
    let gradient = base_big.pow(suffix_len);
    let prefix_shift = base_big.pow((middle_length as u32) + suffix_len);
    let prefix_value = digits_to_biguint(base, &prefix_digits);
    let suffix_value = digits_to_biguint(base, &suffix_digits);
    AffineFormula {
        shift: prefix_value * prefix_shift + suffix_value,
        gradient,
    }
}

fn digits_to_biguint(base: u32, digits: &[u32]) -> BigUint {
    let base_big = BigUint::from(base);
    let mut value = BigUint::from(0u32);
    for &digit in digits {
        value *= &base_big;
        value += digit;
    }
    value
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}
