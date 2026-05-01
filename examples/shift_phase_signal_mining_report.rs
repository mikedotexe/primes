//! Shift-phase signal mining report.
//!
//! This report is the curated follow-up to the broad affine phase residual
//! atlas. It asks a reduced question:
//!
//! same slope, different intercept, different residue weather.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example shift_phase_signal_mining_report -- --out-dir /tmp/primes_shift_phase_signal_mining
//! ```

use plotters::prelude::*;
use primes::validation::{
    affine_phase_residual::{
        build_shift_phase_signal_mining_report, AffinePhaseResidualRow, ShiftPhaseFoilRow,
        ShiftPhaseMaturityRow, ShiftPhaseResidueGateRow, ShiftPhaseSignalMiningReport,
        ShiftPhaseSignalMiningSettings, DEFAULT_PHASE_RESIDUAL_BASES,
        DEFAULT_PHASE_RESIDUAL_MAX_MIDDLE_LENGTH, DEFAULT_PHASE_RESIDUAL_MIN_MIDDLE_LENGTH,
        DEFAULT_SHIFT_PHASE_FOLLOWUP_MIDDLE_LENGTH, DEFAULT_SHIFT_PHASE_TOP_LIMIT,
        DEFAULT_SHIFT_PHASE_WITNESS_LIMIT,
    },
    reporting::{
        ensure_dir, export_timestamp_utc, write_artifact_manifest, write_csv_rows,
        write_json_pretty, write_text_file, ArtifactManifest,
    },
};
use serde::Serialize;
use std::{
    env,
    path::{Path, PathBuf},
};

const DEFAULT_OUT_DIR: &str = "/tmp/primes_shift_phase_signal_mining";
const ARTIFACT_ID: &str = "shift_phase_signal_mining_report";
const EXPORT_VERSION: u32 = 1;

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    settings: ShiftPhaseSignalMiningSettings,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    report: ShiftPhaseSignalMiningReport,
    image_artifact_rows: Vec<ImageArtifactRow>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("create output directory");

    let report = build_shift_phase_signal_mining_report(options.settings.clone());
    let story_path = options.out_dir.join("shift_phase_story_panel.png");
    render_shift_phase_story_panel(&story_path);
    let line_pair_path = options.out_dir.join("same_gradient_line_pair.png");
    render_same_gradient_line_pair(&report.shift_phase_rows, &line_pair_path);
    let residue_comb_path = options.out_dir.join("residue_gate_comb.png");
    render_residue_gate_comb(&report.residue_gate_rows, &residue_comb_path);
    let waterfall_path = options.out_dir.join("survivor_yield_waterfall.png");
    render_survivor_yield_waterfall(&report.maturity_rows, &waterfall_path);
    let maturity_path = options.out_dir.join("m_maturity_strip.png");
    render_maturity_strip(&report.maturity_rows, &maturity_path);
    let gallery_path = options.out_dir.join("lead_vs_foil_gallery.png");
    render_lead_vs_foil_gallery(&report.maturity_rows, &report.foil_rows, &gallery_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "shift_phase_story_panel".to_string(),
            label: "Shift-phase story panel".to_string(),
            path: story_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "same_gradient_line_pair".to_string(),
            label: "Same-gradient line pair".to_string(),
            path: line_pair_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "residue_gate_comb".to_string(),
            label: "Residue gate comb".to_string(),
            path: residue_comb_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "survivor_yield_waterfall".to_string(),
            label: "Survivor-yield waterfall".to_string(),
            path: waterfall_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "m_maturity_strip".to_string(),
            label: "M-maturity strip".to_string(),
            path: maturity_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "lead_vs_foil_gallery".to_string(),
            label: "Lead versus foil gallery".to_string(),
            path: gallery_path.display().to_string(),
        },
    ];

    write_csv_rows(
        options.out_dir.join("shift_phase_rows.csv"),
        &report.shift_phase_rows,
    )
    .expect("write shift phase rows");
    write_csv_rows(
        options.out_dir.join("maturity_rows.csv"),
        &report.maturity_rows,
    )
    .expect("write maturity rows");
    write_csv_rows(options.out_dir.join("foil_rows.csv"), &report.foil_rows)
        .expect("write foil rows");
    write_csv_rows(
        options.out_dir.join("residue_gate_rows.csv"),
        &report.residue_gate_rows,
    )
    .expect("write residue gate rows");
    write_csv_rows(
        options.out_dir.join("witness_rows.csv"),
        &report.witness_rows,
    )
    .expect("write witness rows");
    write_csv_rows(
        options.out_dir.join("image_artifact_rows.csv"),
        &image_artifact_rows,
    )
    .expect("write image artifact rows");

    let bundle = ReportBundle {
        export_version: EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        report,
        image_artifact_rows,
    };
    write_json_pretty(options.out_dir.join("summary.json"), &bundle).expect("write summary json");
    write_text_file(options.out_dir.join("report.md"), &render_report(&bundle))
        .expect("write report markdown");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo run --release --example shift_phase_signal_mining_report"
                .to_string(),
            args: env::args().skip(1).collect(),
            upstream_inputs: vec![
                "src/validation/affine_phase_residual.rs".to_string(),
                "src/validation/fast_affine.rs".to_string(),
                "src/validation/bounded_k.rs".to_string(),
            ],
            expected_outputs: vec![
                "report.md".to_string(),
                "summary.json".to_string(),
                "shift_phase_rows.csv".to_string(),
                "maturity_rows.csv".to_string(),
                "foil_rows.csv".to_string(),
                "residue_gate_rows.csv".to_string(),
                "witness_rows.csv".to_string(),
                "image_artifact_rows.csv".to_string(),
                "shift_phase_story_panel.png".to_string(),
                "same_gradient_line_pair.png".to_string(),
                "residue_gate_comb.png".to_string(),
                "survivor_yield_waterfall.png".to_string(),
                "m_maturity_strip.png".to_string(),
                "lead_vs_foil_gallery.png".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote shift-phase signal mining report to {}",
        options.out_dir.display()
    );
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut settings = ShiftPhaseSignalMiningSettings::default();
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => out_dir = PathBuf::from(parse_next::<String>(&mut args, "--out-dir")),
            "--bases" => {
                settings.base_settings.bases = parse_next::<String>(&mut args, "--bases")
                    .split(',')
                    .map(|part| {
                        part.parse::<u32>().unwrap_or_else(|_| {
                            eprintln!("invalid base in --bases: {part}");
                            std::process::exit(2);
                        })
                    })
                    .collect();
            }
            "--min-middle-length" => {
                settings.base_settings.min_middle_length =
                    parse_next(&mut args, "--min-middle-length");
            }
            "--max-middle-length" => {
                settings.base_settings.max_middle_length =
                    parse_next(&mut args, "--max-middle-length");
            }
            "--followup-middle-length" => {
                settings.followup_middle_length = parse_next(&mut args, "--followup-middle-length");
            }
            "--top-limit" => {
                let value = parse_next(&mut args, "--top-limit");
                settings.top_limit = value;
                settings.base_settings.top_limit = value;
            }
            "--witness-limit" => {
                let value = parse_next(&mut args, "--witness-limit");
                settings.witness_limit = value;
                settings.base_settings.witness_limit = value;
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => {
                eprintln!("unknown argument: {arg}");
                print_help();
                std::process::exit(2);
            }
        }
    }

    if settings.base_settings.bases.is_empty() {
        eprintln!("--bases must contain at least one base");
        std::process::exit(2);
    }
    if settings.base_settings.min_middle_length == 0
        || settings.base_settings.max_middle_length < settings.base_settings.min_middle_length
    {
        eprintln!("middle length range must be nonempty and start at least at 1");
        std::process::exit(2);
    }
    if settings.followup_middle_length == 0 {
        eprintln!("--followup-middle-length must be at least 1");
        std::process::exit(2);
    }
    if settings.top_limit == 0 || settings.witness_limit == 0 {
        eprintln!("--top-limit and --witness-limit must be at least 1");
        std::process::exit(2);
    }

    Options { out_dir, settings }
}

fn parse_next<T: std::str::FromStr>(args: &mut impl Iterator<Item = String>, flag: &str) -> T {
    args.next()
        .unwrap_or_else(|| {
            eprintln!("{flag} requires a value");
            std::process::exit(2);
        })
        .parse::<T>()
        .unwrap_or_else(|_| {
            eprintln!("invalid value for {flag}");
            std::process::exit(2);
        })
}

fn print_help() {
    println!("Shift-Phase Signal Mining Report");
    println!();
    println!("Options:");
    println!("  --out-dir <path>                Output directory (default: {DEFAULT_OUT_DIR})");
    println!(
        "  --bases <csv>                   Bases to sweep (default: {})",
        DEFAULT_PHASE_RESIDUAL_BASES
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(",")
    );
    println!(
        "  --min-middle-length <n>         Minimum broad-atlas M (default: {DEFAULT_PHASE_RESIDUAL_MIN_MIDDLE_LENGTH})"
    );
    println!(
        "  --max-middle-length <n>         Maximum broad-atlas M (default: {DEFAULT_PHASE_RESIDUAL_MAX_MIDDLE_LENGTH})"
    );
    println!(
        "  --followup-middle-length <n>    Mature follow-up M (default: {DEFAULT_SHIFT_PHASE_FOLLOWUP_MIDDLE_LENGTH})"
    );
    println!(
        "  --top-limit <n>                 Top mature atlas leads to follow (default: {DEFAULT_SHIFT_PHASE_TOP_LIMIT})"
    );
    println!(
        "  --witness-limit <n>             Witnesses per selected direction (default: {DEFAULT_SHIFT_PHASE_WITNESS_LIMIT})"
    );
}

fn render_report(bundle: &ReportBundle) -> String {
    let report = &bundle.report;
    let summary = &report.summary;
    let mut lines = Vec::new();
    lines.push("# Shift-Phase Signal Mining Report".to_string());
    lines.push(String::new());
    lines.push(summary.strong_line.clone());
    lines.push(summary.caution_line.clone());
    lines.push(String::new());
    lines.push("## Curt-Ready One-Minute Version".to_string());
    lines.push("We build a symmetric digit template, then view the middle seed as the variable in an affine line `N(s)=A+G*s`. A same-gradient pair keeps the slope `G` fixed and swaps the boundary roles, so only the intercept `A` changes. That shifted intercept changes how the line meets small-prime residue gates. The question is not whether this proves a density law; the question is which shifted phases repeatedly leave more prime witnesses after size and residue accounting.".to_string());
    lines.push(String::new());
    lines.push("```text".to_string());
    lines
        .push("construction -> affine line -> same-gradient reversal -> residue gates".to_string());
    lines.push("-> survivor yield -> residual decomposition -> lead queue".to_string());
    lines.push("```".to_string());
    lines.push(String::new());
    lines.push("## Vocabulary".to_string());
    lines.push("- `shift-phase residual`: public phrase for the measured leftover in a same-gradient comparison after obvious explanations are separated.".to_string());
    lines.push("- `same-gradient pair`: two compact reversal lanes with the same `G` but different affine shifts.".to_string());
    lines.push(
        "- `residue gate profile`: the small-prime moduli and excluded seed classes met by a lane."
            .to_string(),
    );
    lines.push("- `survivor yield`: prime rate among seeds that survive the exact residue gates used here.".to_string());
    lines.push("- `mature lane`: a larger seed-length follow-up row, here usually `M=4`, used to reduce tiny-lane volatility.".to_string());
    lines.push(String::new());
    lines.push("## Headline".to_string());
    lines.push(format!(
        "- Broad scout surface: `{}` compact residual rows",
        summary.broad_residual_row_count
    ));
    lines.push(format!(
        "- Tracked leads and foils: `{}` maturity rows, `{}` foil rows, `{}` residue-gate rows",
        summary.maturity_row_count, summary.foil_row_count, summary.residue_gate_row_count
    ));
    lines.push(format!(
        "- Persistent or amplifying tracked rows: `{}`",
        summary.persistent_or_amplifying_count
    ));
    lines.push(format!(
        "- Strongest mature follow-up: `{}` (`{}`) at `{:+.3}` pp survivor-yield residual",
        summary.strongest_followup_track,
        summary.strongest_followup_pair,
        summary.strongest_followup_survivor_prime_residual_pp
    ));
    lines.push(format!(
        "- Base-30 `(1,B)` anchor stability: `{}`",
        summary.base30_anchor_stability_label
    ));
    lines.push(String::new());
    lines.push("## Maturity Lead Queue".to_string());
    lines.push("| track | kind | base | pair | source M | source survivor residual | follow-up M | follow-up survivor residual | hits source | hits follow-up | label |".to_string());
    lines.push("|---|---|---:|---|---:|---:|---:|---:|---|---|---|".to_string());
    for row in report.maturity_rows.iter().take(28) {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` vs `{}` | `{}` | `{:+.3}` pp | `{}` | `{:+.3}` pp | `{}` | `{}` | `{}` |",
            row.track_name,
            row.track_kind,
            row.base,
            row.pair_label,
            row.reverse_pair_label,
            row.source_middle_length,
            row.source_survivor_prime_residual_delta_pp,
            row.followup_middle_length,
            row.followup_survivor_prime_residual_delta_pp,
            row.source_prime_hits,
            row.followup_prime_hits,
            row.stability_label
        ));
    }
    lines.push(String::new());
    lines.push("## Foils".to_string());
    lines.push("Foils stay in the report on purpose. They remind us when a row is short-lane volatility, residue-survival bookkeeping, or a low-residual contrast.".to_string());
    lines.push(String::new());
    lines.push(
        "| foil | kind | base | M | pair | raw-size | residue delta | survivor residual | tag |"
            .to_string(),
    );
    lines.push("|---|---|---:|---:|---|---:|---:|---:|---|".to_string());
    for row in &report.foil_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` vs `{}` | `{:+.3}` pp | `{:+.3}` pp | `{:+.3}` pp | `{}` |",
            row.foil_name,
            row.foil_kind,
            row.base,
            row.middle_length,
            row.pair_label,
            row.reverse_pair_label,
            row.residual_after_size_pp,
            row.residue_survivor_delta_pp,
            row.survivor_prime_residual_delta_pp,
            row.lead_tag
        ));
    }
    lines.push(String::new());
    lines.push("## Residue Gate Snapshot".to_string());
    lines.push("| track | base | M | pair | mod | shift mods | gradient mod | excluded seed classes | survivor delta |".to_string());
    lines.push("|---|---:|---:|---|---:|---|---:|---|---:|".to_string());
    for row in report.residue_gate_rows.iter().take(40) {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` vs `{}` | `{}` | `{}` / `{}` | `{}` | `{}` / `{}` | `{:+.3}` pp |",
            row.track_name,
            row.base,
            row.middle_length,
            row.pair_label,
            row.reverse_pair_label,
            row.modulus,
            row.low_high_shift_modulus,
            row.high_low_shift_modulus,
            row.gradient_modulus,
            row.low_high_excluded_seed_classes,
            row.high_low_excluded_seed_classes,
            row.survivor_delta_pp
        ));
    }
    lines.push(String::new());
    lines.push("## Prime Witnesses".to_string());
    lines.push(
        "| track | base | M | orientation | pair | seed | template | decimal value |".to_string(),
    );
    lines.push("|---|---:|---:|---|---|---:|---|---:|".to_string());
    for row in report.witness_rows.iter().take(80) {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |",
            row.track_name,
            row.base,
            row.middle_length,
            row.orientation,
            row.pair_label,
            row.seed,
            row.template_digits,
            row.decimal_value
        ));
    }
    lines.push(String::new());
    lines.push("## Visuals".to_string());
    for image in &bundle.image_artifact_rows {
        lines.push(format!("- `{}`: `{}`", image.label, image.path));
    }
    lines.push(String::new());
    lines.push("## Claim Boundary".to_string());
    lines.push("This report ranks finite deterministic `u64` lanes. A phase residual lead is a lead, not a theorem. Stronger language requires later controls, larger-lane replication, and a clearer separation between classical wheel effects and anything genuinely leftover.".to_string());
    lines.push(String::new());
    lines.join("\n")
}

fn render_shift_phase_story_panel(path: &Path) {
    let root = BitMapBackend::new(path, (1500, 720)).into_drawing_area();
    root.fill(&WHITE).expect("fill story panel");
    root.draw(&Text::new(
        "Shift-Phase Residual: Same Slope, Different Intercept, Different Residue Weather",
        (42, 48),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw title");

    let steps = [
        ("1. Construction", "outer inner seed", "inner outer"),
        ("2. Affine Line", "N(s) = A + G*s", ""),
        ("3. Same Gradient", "same G", "swapped A"),
        ("4. Residue Gates", "small primes", "exclude classes"),
        ("5. Survivor Yield", "prime witnesses", "among survivors"),
        ("6. Lead Queue", "persistent, fades", "reverses, foils"),
    ];
    let card_w = 220;
    let card_h = 150;
    for (idx, (title, body_line_1, body_line_2)) in steps.iter().enumerate() {
        let x = 55 + idx as i32 * 235;
        let y = 185;
        root.draw(&Rectangle::new(
            [(x, y), (x + card_w, y + card_h)],
            story_fill(idx).filled(),
        ))
        .expect("draw story card");
        root.draw(&Rectangle::new(
            [(x, y), (x + card_w, y + card_h)],
            ShapeStyle::from(&RGBColor(65, 72, 82)).stroke_width(1),
        ))
        .expect("draw story border");
        root.draw(&Text::new(
            *title,
            (x + 16, y + 38),
            ("sans-serif", 20).into_font().style(FontStyle::Bold),
        ))
        .expect("draw story title");
        root.draw(&Text::new(
            *body_line_1,
            (x + 16, y + 85),
            ("sans-serif", 17).into_font(),
        ))
        .expect("draw story body line 1");
        if !body_line_2.is_empty() {
            root.draw(&Text::new(
                *body_line_2,
                (x + 16, y + 113),
                ("sans-serif", 17).into_font(),
            ))
            .expect("draw story body line 2");
        }
        if idx + 1 < steps.len() {
            let x1 = x + card_w + 12;
            let x2 = x + card_w + 44;
            let y_mid = y + card_h / 2;
            root.draw(&PathElement::new(
                vec![(x1, y_mid), (x2, y_mid)],
                ShapeStyle::from(&RGBColor(55, 62, 72)).stroke_width(3),
            ))
            .expect("draw story connector");
            root.draw(&Polygon::new(
                vec![(x2, y_mid), (x2 - 10, y_mid - 7), (x2 - 10, y_mid + 7)],
                RGBColor(55, 62, 72).filled(),
            ))
            .expect("draw story arrow");
        }
    }

    root.draw(&Text::new(
        "The mathematical control is narrow: compare two lanes that share the grammar and gradient, then ask what the shifted intercept does to residue survival and prime yield.",
        (85, 470),
        ("sans-serif", 23).into_font(),
    ))
    .expect("draw story note");
    root.draw(&Text::new(
        "The epistemic control is just as important: size/PNT and residue-survivor explanations come first; residuals become leads only after that accounting.",
        (85, 525),
        ("sans-serif", 23).into_font(),
    ))
    .expect("draw story boundary");
    root.present().expect("present story panel");
}

fn render_same_gradient_line_pair(rows: &[AffinePhaseResidualRow], path: &Path) {
    let row = rows
        .iter()
        .find(|row| {
            row.base == 10 && row.middle_length == 3 && row.low_digit == 1 && row.high_digit == 7
        })
        .or_else(|| rows.first())
        .expect("at least one shift-phase row");
    let step = (row.seed_capacity / 200).max(1);
    let low_points = (0..row.seed_capacity)
        .step_by(step as usize)
        .map(|seed| {
            (
                seed as f64,
                (row.low_high_shift + row.gradient * seed) as f64,
            )
        })
        .collect::<Vec<_>>();
    let high_points = (0..row.seed_capacity)
        .step_by(step as usize)
        .map(|seed| {
            (
                seed as f64,
                (row.high_low_shift + row.gradient * seed) as f64,
            )
        })
        .collect::<Vec<_>>();
    let y_min = low_points
        .iter()
        .chain(high_points.iter())
        .map(|(_, value)| *value)
        .fold(f64::INFINITY, f64::min);
    let y_max = low_points
        .iter()
        .chain(high_points.iter())
        .map(|(_, value)| *value)
        .fold(f64::NEG_INFINITY, f64::max);

    let root = BitMapBackend::new(path, (1280, 820)).into_drawing_area();
    root.fill(&WHITE).expect("fill line pair");
    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!(
                "Same-Gradient Pair: {} vs {} (base {}, M={})",
                row.low_high_pair_label, row.high_low_pair_label, row.base, row.middle_length
            ),
            ("sans-serif", 30).into_font(),
        )
        .margin(35)
        .x_label_area_size(70)
        .y_label_area_size(105)
        .build_cartesian_2d(0.0_f64..row.seed_capacity as f64, y_min..y_max)
        .expect("build line-pair chart");

    chart
        .configure_mesh()
        .x_desc("seed s")
        .y_desc("candidate N(s)")
        .y_label_formatter(&|value| format!("{value:.0}"))
        .draw()
        .expect("draw line-pair mesh");

    chart
        .draw_series(LineSeries::new(
            low_points,
            RGBColor(20, 111, 178).stroke_width(4),
        ))
        .expect("draw low-high line");
    chart
        .draw_series(LineSeries::new(
            high_points,
            RGBColor(201, 83, 63).stroke_width(4),
        ))
        .expect("draw high-low line");
    root.draw(&Rectangle::new(
        [(175, 88), (735, 154)],
        WHITE.mix(0.92).filled(),
    ))
    .expect("draw line-pair note backing");
    root.draw(&Text::new(
        format!(
            "Both lines have G = {}; only A changes: {} vs {}",
            row.gradient, row.low_high_shift, row.high_low_shift
        ),
        (190, 112),
        ("sans-serif", 20).into_font(),
    ))
    .expect("draw same-gradient note");
    root.draw(&Text::new(
        format!(
            "raw-size residual {:+.3} pp; survivor-yield residual {:+.3} pp",
            row.residual_after_size_pp, row.survivor_prime_residual_delta_pp
        ),
        (190, 142),
        ("sans-serif", 20).into_font(),
    ))
    .expect("draw residual note");
    root.present().expect("present line pair");
}

fn render_residue_gate_comb(rows: &[ShiftPhaseResidueGateRow], path: &Path) {
    let selected = residue_gate_focus(rows);
    let (y_min, y_max) = padded_range(selected.iter().map(|row| row.survivor_delta_pp));
    let root = BitMapBackend::new(path, (1280, 820)).into_drawing_area();
    root.fill(&WHITE).expect("fill residue comb");
    let mut chart = ChartBuilder::on(&root)
        .caption("Residue Gate Comb", ("sans-serif", 30).into_font())
        .margin(35)
        .x_label_area_size(70)
        .y_label_area_size(90)
        .build_cartesian_2d(0.5_f64..selected.len() as f64 + 0.5, y_min..y_max)
        .expect("build residue comb");

    chart
        .configure_mesh()
        .x_labels(selected.len())
        .x_desc("small-prime gate")
        .y_desc("survivor-count delta (percentage points)")
        .x_label_formatter(&|value| {
            let idx = value.round() as usize;
            if idx == 0 || idx > selected.len() {
                String::new()
            } else {
                selected[idx - 1].modulus.to_string()
            }
        })
        .y_label_formatter(&|value| format!("{value:+.2}"))
        .draw()
        .expect("draw residue comb mesh");

    chart
        .draw_series(LineSeries::new(
            vec![(0.5, 0.0), (selected.len() as f64 + 0.5, 0.0)],
            RGBColor(120, 120, 120).stroke_width(1),
        ))
        .expect("draw zero line");

    for (idx, row) in selected.iter().enumerate() {
        let x = idx as f64 + 1.0;
        let style = if row.survivor_delta_pp >= 0.0 {
            RGBColor(39, 139, 119).filled()
        } else {
            RGBColor(190, 76, 68).filled()
        };
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x - 0.32, 0.0), (x + 0.32, row.survivor_delta_pp)],
                style,
            )))
            .expect("draw residue bar");
    }

    if let Some(first) = selected.first() {
        root.draw(&Text::new(
            format!(
                "{} M{}: {} vs {}",
                first.track_name, first.middle_length, first.pair_label, first.reverse_pair_label
            ),
            (82, 106),
            ("sans-serif", 20).into_font(),
        ))
        .expect("draw residue context");
        root.draw(&Text::new(
            "Each tooth is one small-prime gate; the shifted intercept changes excluded seed classes.",
            (82, 136),
            ("sans-serif", 20).into_font(),
        ))
        .expect("draw residue note");
    }
    root.present().expect("present residue comb");
}

fn render_survivor_yield_waterfall(rows: &[ShiftPhaseMaturityRow], path: &Path) {
    let row = rows
        .iter()
        .find(|row| row.track_name == "base10_low_outer_17")
        .or_else(|| rows.first())
        .expect("at least one maturity row");
    let values = [
        ("source raw", row.source_raw_delta_pp),
        ("source raw-size", row.source_residual_after_size_pp),
        (
            "source survivor",
            row.source_survivor_prime_residual_delta_pp,
        ),
        ("M4 raw", row.followup_raw_delta_pp),
        ("M4 raw-size", row.followup_residual_after_size_pp),
        ("M4 survivor", row.followup_survivor_prime_residual_delta_pp),
    ];
    let (y_min, y_max) = padded_range(values.iter().map(|(_, value)| *value));
    let root = BitMapBackend::new(path, (1280, 820)).into_drawing_area();
    root.fill(&WHITE).expect("fill waterfall");
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Survivor-Yield Decomposition",
            ("sans-serif", 30).into_font(),
        )
        .margin(35)
        .x_label_area_size(90)
        .y_label_area_size(90)
        .build_cartesian_2d(0.5_f64..values.len() as f64 + 0.5, y_min..y_max)
        .expect("build waterfall");

    chart
        .configure_mesh()
        .x_labels(values.len())
        .x_desc("decomposition layer")
        .y_desc("delta (percentage points)")
        .x_label_formatter(&|value| {
            let idx = value.round() as usize;
            if idx == 0 || idx > values.len() {
                String::new()
            } else {
                values[idx - 1].0.to_string()
            }
        })
        .y_label_formatter(&|value| format!("{value:+.2}"))
        .draw()
        .expect("draw waterfall mesh");

    chart
        .draw_series(LineSeries::new(
            vec![(0.5, 0.0), (values.len() as f64 + 0.5, 0.0)],
            RGBColor(125, 125, 125).stroke_width(1),
        ))
        .expect("draw waterfall zero");

    for (idx, (_, value)) in values.iter().enumerate() {
        let x = idx as f64 + 1.0;
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x - 0.28, 0.0), (x + 0.28, *value)],
                signed_color(*value, y_min.abs().max(y_max.abs())).filled(),
            )))
            .expect("draw waterfall bar");
    }

    root.draw(&Text::new(
        format!(
            "{}: {} vs {} ({})",
            row.track_name, row.pair_label, row.reverse_pair_label, row.stability_label
        ),
        (82, 106),
        ("sans-serif", 20).into_font(),
    ))
    .expect("draw waterfall context");
    root.present().expect("present waterfall");
}

fn render_maturity_strip(rows: &[ShiftPhaseMaturityRow], path: &Path) {
    let selected = rows.iter().take(16).collect::<Vec<_>>();
    let root = BitMapBackend::new(path, (1500, 940)).into_drawing_area();
    root.fill(&WHITE).expect("fill maturity strip");
    root.draw(&Text::new(
        "M-Maturity Strip",
        (42, 42),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw strip title");
    root.draw(&Text::new(
        "left dot = source M; right dot = mature follow-up M. Same sign is persistence; larger distance is amplification.",
        (42, 82),
        ("sans-serif", 20).into_font(),
    ))
    .expect("draw strip note");

    let max_abs = selected
        .iter()
        .flat_map(|row| {
            [
                row.source_survivor_prime_residual_delta_pp.abs(),
                row.followup_survivor_prime_residual_delta_pp.abs(),
            ]
        })
        .fold(0.0_f64, f64::max)
        .max(0.01);
    let x_zero = 760;
    let scale = 560.0 / max_abs;
    root.draw(&PathElement::new(
        vec![(x_zero, 130), (x_zero, 890)],
        ShapeStyle::from(&RGBColor(135, 135, 135)).stroke_width(2),
    ))
    .expect("draw strip zero");

    for (idx, row) in selected.iter().enumerate() {
        let y = 155 + idx as i32 * 45;
        let source_x = x_zero + (row.source_survivor_prime_residual_delta_pp * scale) as i32;
        let follow_x = x_zero + (row.followup_survivor_prime_residual_delta_pp * scale) as i32;
        root.draw(&Text::new(
            format!("{} b{} {}", idx + 1, row.base, row.pair_label),
            (42, y + 6),
            ("sans-serif", 15).into_font(),
        ))
        .expect("draw strip label");
        root.draw(&PathElement::new(
            vec![(source_x, y), (follow_x, y)],
            ShapeStyle::from(&RGBColor(85, 94, 104)).stroke_width(2),
        ))
        .expect("draw strip segment");
        root.draw(&Circle::new(
            (source_x, y),
            7,
            RGBColor(42, 115, 185).filled(),
        ))
        .expect("draw source dot");
        root.draw(&Circle::new(
            (follow_x, y),
            7,
            RGBColor(218, 112, 63).filled(),
        ))
        .expect("draw followup dot");
        root.draw(&Text::new(
            row.stability_label.clone(),
            (1210, y + 6),
            ("sans-serif", 15).into_font(),
        ))
        .expect("draw label");
    }

    root.present().expect("present maturity strip");
}

fn render_lead_vs_foil_gallery(
    maturity_rows: &[ShiftPhaseMaturityRow],
    foil_rows: &[ShiftPhaseFoilRow],
    path: &Path,
) {
    let root = BitMapBackend::new(path, (1500, 980)).into_drawing_area();
    root.fill(&WHITE).expect("fill gallery");
    root.draw(&Text::new(
        "Leads and Foils",
        (38, 40),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw gallery title");

    for (idx, row) in maturity_rows.iter().take(6).enumerate() {
        let x = 35 + (idx % 3) as i32 * 485;
        let y = 95 + (idx / 3) as i32 * 250;
        draw_maturity_card(&root, x, y, row);
    }
    for (idx, row) in foil_rows.iter().take(3).enumerate() {
        let x = 35 + idx as i32 * 485;
        let y = 610;
        draw_foil_card(&root, x, y, row);
    }

    root.present().expect("present lead gallery");
}

fn draw_maturity_card(
    root: &DrawingArea<BitMapBackend<'_>, plotters::coord::Shift>,
    x: i32,
    y: i32,
    row: &ShiftPhaseMaturityRow,
) {
    let card_w = 455;
    let card_h = 220;
    root.draw(&Rectangle::new(
        [(x, y), (x + card_w, y + card_h)],
        RGBColor(231, 246, 241).filled(),
    ))
    .expect("draw maturity card");
    root.draw(&Rectangle::new(
        [(x, y), (x + card_w, y + card_h)],
        ShapeStyle::from(&RGBColor(80, 88, 96)).stroke_width(1),
    ))
    .expect("draw maturity border");
    root.draw(&Text::new(
        format!("{}: b{} {}", row.track_kind, row.base, row.pair_label),
        (x + 16, y + 30),
        ("sans-serif", 18).into_font().style(FontStyle::Bold),
    ))
    .expect("draw maturity title");
    root.draw(&Text::new(
        format!("reverse {}", row.reverse_pair_label),
        (x + 16, y + 60),
        ("sans-serif", 15).into_font(),
    ))
    .expect("draw reverse");
    root.draw(&Text::new(
        format!(
            "source M{}: {:+.3} pp",
            row.source_middle_length, row.source_survivor_prime_residual_delta_pp
        ),
        (x + 16, y + 95),
        ("sans-serif", 16).into_font(),
    ))
    .expect("draw source");
    root.draw(&Text::new(
        format!(
            "follow-up M{}: {:+.3} pp",
            row.followup_middle_length, row.followup_survivor_prime_residual_delta_pp
        ),
        (x + 16, y + 125),
        ("sans-serif", 16).into_font().style(FontStyle::Bold),
    ))
    .expect("draw followup");
    root.draw(&Text::new(
        format!(
            "hits: {} -> {}",
            row.source_prime_hits, row.followup_prime_hits
        ),
        (x + 16, y + 155),
        ("sans-serif", 15).into_font(),
    ))
    .expect("draw hits");
    root.draw(&Text::new(
        format!("label: {}", row.stability_label),
        (x + 16, y + 185),
        ("sans-serif", 15).into_font(),
    ))
    .expect("draw stability");
}

fn draw_foil_card(
    root: &DrawingArea<BitMapBackend<'_>, plotters::coord::Shift>,
    x: i32,
    y: i32,
    row: &ShiftPhaseFoilRow,
) {
    let card_w = 455;
    let card_h = 220;
    root.draw(&Rectangle::new(
        [(x, y), (x + card_w, y + card_h)],
        RGBColor(248, 239, 226).filled(),
    ))
    .expect("draw foil card");
    root.draw(&Rectangle::new(
        [(x, y), (x + card_w, y + card_h)],
        ShapeStyle::from(&RGBColor(94, 82, 70)).stroke_width(1),
    ))
    .expect("draw foil border");
    root.draw(&Text::new(
        format!("foil: b{} {}", row.base, row.pair_label),
        (x + 16, y + 30),
        ("sans-serif", 18).into_font().style(FontStyle::Bold),
    ))
    .expect("draw foil title");
    root.draw(&Text::new(
        row.foil_kind.clone(),
        (x + 16, y + 60),
        ("sans-serif", 15).into_font(),
    ))
    .expect("draw foil kind");
    root.draw(&Text::new(
        format!("M{} vs {}", row.middle_length, row.reverse_pair_label),
        (x + 16, y + 95),
        ("sans-serif", 16).into_font(),
    ))
    .expect("draw foil pair");
    root.draw(&Text::new(
        format!("raw-size: {:+.3} pp", row.residual_after_size_pp),
        (x + 16, y + 125),
        ("sans-serif", 16).into_font(),
    ))
    .expect("draw foil residual");
    root.draw(&Text::new(
        format!("residue delta: {:+.3} pp", row.residue_survivor_delta_pp),
        (x + 16, y + 155),
        ("sans-serif", 16).into_font(),
    ))
    .expect("draw foil residue");
    root.draw(&Text::new(
        format!("tag: {}", row.lead_tag),
        (x + 16, y + 185),
        ("sans-serif", 15).into_font(),
    ))
    .expect("draw foil tag");
}

fn residue_gate_focus(rows: &[ShiftPhaseResidueGateRow]) -> Vec<&ShiftPhaseResidueGateRow> {
    let mut selected = rows
        .iter()
        .filter(|row| row.track_name == "base10_low_outer_17" && row.middle_length == 3)
        .collect::<Vec<_>>();
    if selected.is_empty() {
        selected = rows.iter().take(9).collect();
    }
    selected
}

fn story_fill(idx: usize) -> RGBColor {
    match idx {
        0 => RGBColor(229, 242, 247),
        1 => RGBColor(233, 244, 234),
        2 => RGBColor(247, 239, 224),
        3 => RGBColor(241, 233, 247),
        4 => RGBColor(232, 246, 242),
        _ => RGBColor(245, 237, 232),
    }
}

fn signed_color(delta_pp: f64, max_abs_pp: f64) -> RGBColor {
    if delta_pp.abs() < f64::EPSILON {
        return RGBColor(235, 235, 235);
    }
    let t = (delta_pp.abs() / max_abs_pp).clamp(0.0, 1.0);
    let low = (235.0 - 75.0 * t) as u8;
    if delta_pp > 0.0 {
        RGBColor(238, low, low)
    } else {
        RGBColor(low, low, 238)
    }
}

fn padded_range(values: impl Iterator<Item = f64>) -> (f64, f64) {
    let values = values.collect::<Vec<_>>();
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if !min.is_finite() || !max.is_finite() {
        return (-1.0, 1.0);
    }
    let span = (max - min).abs();
    let pad = if span < 0.001 { 1.0 } else { span * 0.25 };
    let lower = (min - pad).min(0.0);
    let upper = (max + pad).max(0.0);
    (lower, upper)
}
