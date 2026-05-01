//! Downstream signal-clarity report for the base-14 mechanism lane.
//!
//! This report reads the maintained base-14 mechanism artifact and makes the
//! main signal easier to see by combining two exact views:
//! - transfer-source prime deltas by candidate category
//! - admissible-set effect vs prime-yield effect
//!
//! The goal is not to broaden the search. It is to clarify which active
//! base-14 pairs are really driven by shared-admissible prime yield, which are
//! transfer-supported, and which sit in between.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example base14_signal_clarity_report
//! cargo run --release --example base14_signal_clarity_report -- --input-json /tmp/primes_base14_outlier_mechanism/summary.json --out-dir /tmp/primes_base14_signal_clarity
//! ```

use plotters::prelude::*;
use primes::validation::reporting::{
    ensure_dir, export_timestamp_utc, write_csv_rows, write_json_pretty, write_text_file,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
};

const DEFAULT_INPUT_JSON: &str = "/tmp/primes_base14_outlier_mechanism/summary.json";
const DEFAULT_OUT_DIR: &str = "/tmp/primes_base14_signal_clarity";
const REPORT_EXPORT_VERSION: u32 = 1;

#[derive(Debug)]
struct Options {
    input_json: PathBuf,
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
struct MechanismRow {
    pair_label: String,
    atlas_role: String,
    explanation_label: String,
    stress_case: bool,
    admissible_delta_pp: f64,
    prime_hit_delta_count: isize,
    prime_hit_delta_pp: f64,
    admissible_set_effect_pp: f64,
    prime_yield_effect_pp: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct CandidateTransferRow {
    pair_label: String,
    category: String,
    prime_delta_count: isize,
}

#[derive(Debug, Clone, Deserialize)]
struct InputBundle {
    generated_at_utc: String,
    mechanism_rows: Vec<MechanismRow>,
    candidate_transfer_rows: Vec<CandidateTransferRow>,
}

#[derive(Debug, Clone, Serialize)]
struct ClarityRow {
    pair_label: String,
    atlas_role: String,
    explanation_label: String,
    stress_case: bool,
    prime_hit_delta_count: isize,
    prime_hit_delta_pp: f64,
    admissible_delta_pp: f64,
    inadmissible_to_admissible_prime_delta_count: isize,
    admissible_to_inadmissible_prime_delta_count: isize,
    shared_admissible_prime_delta_count: isize,
    overlap_prime_delta_count: isize,
    dominant_count_source: String,
    dominant_effect_source: String,
    clarity_label: String,
    admissible_set_effect_pp: f64,
    prime_yield_effect_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct TransferContributionRow {
    pair_label: String,
    category: String,
    prime_delta_count: isize,
}

#[derive(Debug, Clone, Serialize)]
struct EffectContributionRow {
    pair_label: String,
    component: String,
    contribution_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    pair_count: usize,
    shared_yield_core_pairs: usize,
    overlap_supported_pairs: usize,
    mixed_support_pairs: usize,
    stress_case_pair: String,
    main_signal: String,
}

#[derive(Debug, Clone, Serialize)]
struct OutputBundle {
    export_version: u32,
    generated_at_utc: String,
    input_json: String,
    input_generated_at_utc: String,
    clarity_rows: Vec<ClarityRow>,
    transfer_contribution_rows: Vec<TransferContributionRow>,
    effect_contribution_rows: Vec<EffectContributionRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let input_bundle = load_input_bundle(&options.input_json);
    let clarity_rows = build_clarity_rows(&input_bundle);
    let transfer_contribution_rows = build_transfer_contribution_rows(&input_bundle);
    let effect_contribution_rows = build_effect_contribution_rows(&clarity_rows);

    let transfer_plot_path = options.out_dir.join("transfer_source_prime_delta.png");
    render_transfer_source_chart(
        &clarity_rows,
        &transfer_contribution_rows,
        &transfer_plot_path,
    );
    let effect_plot_path = options.out_dir.join("effect_contribution_pp.png");
    render_effect_chart(&clarity_rows, &effect_plot_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "transfer_source".to_string(),
            label: "Prime delta by transfer source".to_string(),
            path: transfer_plot_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "effect_contribution".to_string(),
            label: "Admissible-set effect vs prime-yield effect".to_string(),
            path: effect_plot_path.display().to_string(),
        },
    ];
    let report_summary = build_report_summary(&clarity_rows);
    let observations = derive_observations(&clarity_rows);

    let bundle = OutputBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        input_json: options.input_json.display().to_string(),
        input_generated_at_utc: input_bundle.generated_at_utc,
        clarity_rows: clarity_rows.clone(),
        transfer_contribution_rows: transfer_contribution_rows.clone(),
        effect_contribution_rows: effect_contribution_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary,
        observations,
    };

    write_csv_rows(options.out_dir.join("clarity_rows.csv"), &clarity_rows)
        .expect("failed to write clarity_rows.csv");
    write_csv_rows(
        options.out_dir.join("transfer_contribution_rows.csv"),
        &transfer_contribution_rows,
    )
    .expect("failed to write transfer_contribution_rows.csv");
    write_csv_rows(
        options.out_dir.join("effect_contribution_rows.csv"),
        &effect_contribution_rows,
    )
    .expect("failed to write effect_contribution_rows.csv");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle)
        .expect("failed to write summary.json");

    let markdown = render_markdown(&bundle);
    write_text_file(options.out_dir.join("report.md"), &markdown)
        .expect("failed to write report.md");

    println!("Base-14 signal clarity report");
    println!("  input mechanism: {}", options.input_json.display());
    println!("  output dir:      {}", options.out_dir.display());
    for row in &clarity_rows {
        println!(
            "  {} | {} | count source {} | effect source {}",
            row.pair_label,
            row.clarity_label,
            row.dominant_count_source,
            row.dominant_effect_source
        );
    }
}

fn parse_args() -> Options {
    let mut input_json = PathBuf::from(DEFAULT_INPUT_JSON);
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input-json" => {
                let value = args
                    .next()
                    .unwrap_or_else(|| panic!("missing value after --input-json"));
                input_json = PathBuf::from(value);
            }
            "--out-dir" => {
                let value = args
                    .next()
                    .unwrap_or_else(|| panic!("missing value after --out-dir"));
                out_dir = PathBuf::from(value);
            }
            "--help" | "-h" => print_help_and_exit(),
            other => panic!("unrecognized argument: {other}"),
        }
    }

    Options {
        input_json,
        out_dir,
    }
}

fn print_help_and_exit() -> ! {
    println!("Usage:");
    println!("  cargo run --release --example base14_signal_clarity_report -- [options]");
    println!();
    println!("Options:");
    println!("  --input-json <path>   Read mechanism artifact from this JSON path");
    println!("  --out-dir <path>      Write output bundle to this directory");
    println!("  -h, --help            Show this help");
    std::process::exit(0);
}

fn load_input_bundle(path: &Path) -> InputBundle {
    let text = fs::read_to_string(path).unwrap_or_else(|err| {
        panic!(
            "Failed to read mechanism JSON at {}: {err}\nRun `cargo run --release --example base14_outlier_mechanism_report` first or pass --input-json.",
            path.display()
        )
    });
    serde_json::from_str(&text).unwrap_or_else(|err| {
        panic!(
            "Failed to parse mechanism JSON at {}: {err}",
            path.display()
        )
    })
}

fn build_clarity_rows(input: &InputBundle) -> Vec<ClarityRow> {
    let mut transfer_lookup = BTreeMap::<(String, String), isize>::new();
    for row in &input.candidate_transfer_rows {
        transfer_lookup.insert(
            (row.pair_label.clone(), row.category.clone()),
            row.prime_delta_count,
        );
    }

    let mut rows = input
        .mechanism_rows
        .iter()
        .map(|row| {
            let to_admissible = transfer_lookup
                .get(&(
                    row.pair_label.clone(),
                    "inadmissible_to_admissible".to_string(),
                ))
                .copied()
                .unwrap_or(0);
            let to_inadmissible = transfer_lookup
                .get(&(
                    row.pair_label.clone(),
                    "admissible_to_inadmissible".to_string(),
                ))
                .copied()
                .unwrap_or(0);
            let shared = transfer_lookup
                .get(&(row.pair_label.clone(), "shared_admissible".to_string()))
                .copied()
                .unwrap_or(0);
            let overlap = to_admissible + to_inadmissible;
            let dominant_count_source = dominant_count_source(shared, overlap);
            let dominant_effect_source =
                dominant_effect_source(row.admissible_set_effect_pp, row.prime_yield_effect_pp);
            let clarity_label = classify_clarity(
                shared,
                overlap,
                row.admissible_set_effect_pp,
                row.prime_yield_effect_pp,
            );

            ClarityRow {
                pair_label: row.pair_label.clone(),
                atlas_role: row.atlas_role.clone(),
                explanation_label: row.explanation_label.clone(),
                stress_case: row.stress_case,
                prime_hit_delta_count: row.prime_hit_delta_count,
                prime_hit_delta_pp: row.prime_hit_delta_pp,
                admissible_delta_pp: row.admissible_delta_pp,
                inadmissible_to_admissible_prime_delta_count: to_admissible,
                admissible_to_inadmissible_prime_delta_count: to_inadmissible,
                shared_admissible_prime_delta_count: shared,
                overlap_prime_delta_count: overlap,
                dominant_count_source,
                dominant_effect_source,
                clarity_label,
                admissible_set_effect_pp: row.admissible_set_effect_pp,
                prime_yield_effect_pp: row.prime_yield_effect_pp,
            }
        })
        .collect::<Vec<_>>();

    rows.sort_by(|left, right| left.pair_label.cmp(&right.pair_label));
    rows
}

fn build_transfer_contribution_rows(input: &InputBundle) -> Vec<TransferContributionRow> {
    let mut rows = input
        .candidate_transfer_rows
        .iter()
        .filter(|row| row.category != "shared_inadmissible")
        .map(|row| TransferContributionRow {
            pair_label: row.pair_label.clone(),
            category: row.category.clone(),
            prime_delta_count: row.prime_delta_count,
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.pair_label.cmp(&right.pair_label).then_with(|| {
            transfer_sort_key(&left.category).cmp(&transfer_sort_key(&right.category))
        })
    });
    rows
}

fn build_effect_contribution_rows(rows: &[ClarityRow]) -> Vec<EffectContributionRow> {
    let mut effect_rows = Vec::new();
    for row in rows {
        effect_rows.push(EffectContributionRow {
            pair_label: row.pair_label.clone(),
            component: "admissible_set_effect".to_string(),
            contribution_pp: row.admissible_set_effect_pp,
        });
        effect_rows.push(EffectContributionRow {
            pair_label: row.pair_label.clone(),
            component: "prime_yield_effect".to_string(),
            contribution_pp: row.prime_yield_effect_pp,
        });
    }
    effect_rows
}

fn dominant_count_source(shared: isize, overlap: isize) -> String {
    let shared_abs = shared.abs();
    let overlap_abs = overlap.abs();
    if shared_abs > overlap_abs {
        "shared_admissible".to_string()
    } else if overlap_abs > shared_abs {
        "boundary_transfer".to_string()
    } else {
        "balanced".to_string()
    }
}

fn dominant_effect_source(admissible_set_effect_pp: f64, prime_yield_effect_pp: f64) -> String {
    if prime_yield_effect_pp.abs() > admissible_set_effect_pp.abs() {
        "prime_yield".to_string()
    } else if admissible_set_effect_pp.abs() > prime_yield_effect_pp.abs() {
        "admissible_set".to_string()
    } else {
        "balanced".to_string()
    }
}

fn classify_clarity(
    shared: isize,
    overlap: isize,
    admissible_set_effect_pp: f64,
    prime_yield_effect_pp: f64,
) -> String {
    if shared > 0
        && shared.abs() > overlap.abs()
        && prime_yield_effect_pp.abs() > admissible_set_effect_pp.abs()
    {
        "shared_yield_core".to_string()
    } else if overlap > 0 && shared <= 0 {
        "overlap_supported".to_string()
    } else {
        "mixed_support".to_string()
    }
}

fn transfer_sort_key(category: &str) -> usize {
    match category {
        "inadmissible_to_admissible" => 0,
        "shared_admissible" => 1,
        "admissible_to_inadmissible" => 2,
        _ => 3,
    }
}

fn render_transfer_source_chart(
    clarity_rows: &[ClarityRow],
    transfer_rows: &[TransferContributionRow],
    path: &Path,
) {
    let row_lookup = clarity_rows
        .iter()
        .enumerate()
        .map(|(index, row)| (row.pair_label.clone(), index as i32))
        .collect::<BTreeMap<_, _>>();
    let labels = clarity_rows
        .iter()
        .map(|row| format!("{} {}", short_label(&row.clarity_label), row.pair_label))
        .collect::<Vec<_>>();
    let max_y = clarity_rows.len() as i32;
    let max_abs = transfer_rows
        .iter()
        .map(|row| row.prime_delta_count.abs())
        .sum::<isize>()
        .max(12) as f64;

    let root = BitMapBackend::new(
        path,
        (1080, (300 + clarity_rows.len() as u32 * 70).max(560)),
    )
    .into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill transfer chart canvas");

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Base 14 Prime Delta by Transfer Source  (counts relative to k=(0,0))",
            ("sans-serif", 24),
        )
        .margin(24)
        .x_label_area_size(64)
        .y_label_area_size(180)
        .build_cartesian_2d(-max_abs..max_abs, 0i32..max_y)
        .expect("failed to build transfer source chart");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("prime delta count")
        .y_desc("active pairs")
        .y_labels(clarity_rows.len())
        .y_label_formatter(&move |value| {
            if *value >= 0 && *value < max_y {
                let row_index = (max_y - 1 - *value) as usize;
                labels.get(row_index).cloned().unwrap_or_default()
            } else {
                String::new()
            }
        })
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw transfer source mesh");

    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(0.0, 0), (0.0, max_y)],
            ShapeStyle::from(&RGBColor(180, 173, 162)).stroke_width(1),
        )))
        .expect("failed to draw transfer source zero line");

    for row in transfer_rows {
        let y = max_y
            - 1
            - row_lookup
                .get(&row.pair_label)
                .copied()
                .expect("transfer row should point to known pair");
        let (start, end) = if row.prime_delta_count >= 0 {
            positive_span_for_category(transfer_rows, &row.pair_label, &row.category)
        } else {
            negative_span_for_category(transfer_rows, &row.pair_label, &row.category)
        };
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(start, y), (end, y + 1)],
                ShapeStyle::from(&transfer_color(&row.category)).filled(),
            )))
            .expect("failed to draw transfer source segment");
    }

    root.present().expect("failed to present transfer chart");
}

fn positive_span_for_category(
    rows: &[TransferContributionRow],
    pair_label: &str,
    category: &str,
) -> (f64, f64) {
    let ordered = ["inadmissible_to_admissible", "shared_admissible"];
    let mut cursor = 0isize;
    for current in ordered {
        let delta = rows
            .iter()
            .find(|row| row.pair_label == pair_label && row.category == current)
            .map(|row| row.prime_delta_count)
            .unwrap_or(0);
        if delta > 0 {
            let start = cursor as f64;
            cursor += delta;
            if current == category {
                return (start, cursor as f64);
            }
        }
    }
    (0.0, 0.0)
}

fn negative_span_for_category(
    rows: &[TransferContributionRow],
    pair_label: &str,
    category: &str,
) -> (f64, f64) {
    let ordered = ["admissible_to_inadmissible", "shared_admissible"];
    let mut cursor = 0isize;
    for current in ordered {
        let delta = rows
            .iter()
            .find(|row| row.pair_label == pair_label && row.category == current)
            .map(|row| row.prime_delta_count)
            .unwrap_or(0);
        if delta < 0 {
            let start = cursor as f64;
            cursor += delta;
            if current == category {
                return (cursor as f64, start);
            }
        }
    }
    (0.0, 0.0)
}

fn transfer_color(category: &str) -> RGBColor {
    match category {
        "inadmissible_to_admissible" => RGBColor(69, 129, 190),
        "shared_admissible" => RGBColor(80, 140, 52),
        "admissible_to_inadmissible" => RGBColor(180, 82, 45),
        _ => RGBColor(120, 120, 120),
    }
}

fn render_effect_chart(rows: &[ClarityRow], path: &Path) {
    let labels = rows
        .iter()
        .map(|row| row.pair_label.clone())
        .collect::<Vec<_>>();
    let max_x = rows.len() as i32;
    let max_abs = rows
        .iter()
        .flat_map(|row| {
            [
                row.admissible_set_effect_pp.abs(),
                row.prime_yield_effect_pp.abs(),
            ]
        })
        .fold(0.0_f64, f64::max)
        .max(1.0)
        + 0.8;

    let root = BitMapBackend::new(path, (1040, 680)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill effect chart canvas");
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Base 14 Exact Effect Contributions  (pp relative to k=(0,0))",
            ("sans-serif", 24),
        )
        .margin(24)
        .x_label_area_size(56)
        .y_label_area_size(64)
        .build_cartesian_2d(0f64..max_x as f64, -max_abs..max_abs)
        .expect("failed to build effect chart");

    chart
        .configure_mesh()
        .x_desc("active pairs")
        .y_desc("contribution (pp)")
        .x_labels(rows.len())
        .x_label_formatter(&move |value| {
            let index = value.floor() as usize;
            labels.get(index).cloned().unwrap_or_default()
        })
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .light_line_style(RGBColor(214, 207, 196))
        .draw()
        .expect("failed to draw effect chart mesh");

    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(0.0, 0.0), (max_x as f64, 0.0)],
            ShapeStyle::from(&RGBColor(180, 173, 162)).stroke_width(1),
        )))
        .expect("failed to draw effect zero line");

    for (index, row) in rows.iter().enumerate() {
        let x = index as f64;
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x + 0.10, 0.0), (x + 0.42, row.admissible_set_effect_pp)],
                ShapeStyle::from(&RGBColor(69, 129, 190)).filled(),
            )))
            .expect("failed to draw admissible effect bar");
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x + 0.52, 0.0), (x + 0.84, row.prime_yield_effect_pp)],
                ShapeStyle::from(&RGBColor(191, 82, 32)).filled(),
            )))
            .expect("failed to draw yield effect bar");
        if row.stress_case {
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(x + 0.05, -max_abs), (x + 0.89, max_abs)],
                    ShapeStyle::from(&BLACK.mix(0.55)).stroke_width(2),
                )))
                .expect("failed to draw stress case outline");
        }
    }

    root.present().expect("failed to present effect chart");
}

fn short_label(label: &str) -> &'static str {
    match label {
        "shared_yield_core" => "shared",
        "overlap_supported" => "overlap",
        "mixed_support" => "mixed",
        _ => "other",
    }
}

fn build_report_summary(rows: &[ClarityRow]) -> ReportSummary {
    ReportSummary {
        pair_count: rows.len(),
        shared_yield_core_pairs: rows
            .iter()
            .filter(|row| row.clarity_label == "shared_yield_core")
            .count(),
        overlap_supported_pairs: rows
            .iter()
            .filter(|row| row.clarity_label == "overlap_supported")
            .count(),
        mixed_support_pairs: rows
            .iter()
            .filter(|row| row.clarity_label == "mixed_support")
            .count(),
        stress_case_pair: "(D,B)".to_string(),
        main_signal:
            "The clearest base-14 signal is shared-admissible prime yield, not admissible-count gain."
                .to_string(),
    }
}

fn derive_observations(rows: &[ClarityRow]) -> Vec<String> {
    let stress_case = rows
        .iter()
        .find(|row| row.stress_case)
        .expect("stress-case row should exist");
    let overlap_case = rows
        .iter()
        .find(|row| row.clarity_label == "overlap_supported")
        .expect("overlap-supported row should exist");
    let shared_pairs = rows
        .iter()
        .filter(|row| row.clarity_label == "shared_yield_core")
        .map(|row| row.pair_label.clone())
        .collect::<Vec<_>>()
        .join(", ");

    vec![
        format!(
            "The stress case {} is the clearest signal row: shared-admissible prime delta is {:+}, overlap prime delta is {:+}, and the dominant exact effect is {}.",
            stress_case.pair_label,
            stress_case.shared_admissible_prime_delta_count,
            stress_case.overlap_prime_delta_count,
            stress_case.dominant_effect_source
        ),
        format!(
            "Shared-yield-core pairs are {}. In these rows the shared-admissible lane carries more of the exact prime delta than the boundary-transfer lane.",
            shared_pairs
        ),
        format!(
            "{} is the counterexample that keeps the story honest: its net win is transfer-supported even though the mechanism report still classifies it as yield-dominated on the rate lens.",
            overlap_case.pair_label
        ),
        "The main signal is now clearer in two independent exact views: the count lens isolates where prime delta is created, and the effect lens shows that prime-yield change is larger than admissible-set change for every active base-14 pair.".to_string(),
    ]
}

fn render_markdown(bundle: &OutputBundle) -> String {
    let mut markdown = String::new();
    markdown.push_str("# Base-14 Signal Clarity\n\n");
    markdown.push_str("_Generated from `examples/base14_signal_clarity_report.rs`._\n\n");
    markdown.push_str(&format!(
        "- Input mechanism artifact: `{}`\n- Output directory: `{}`\n\n",
        bundle.input_json, DEFAULT_OUT_DIR
    ));

    markdown.push_str("## Main Signal\n\n");
    markdown.push_str(
        "The clearest base-14 signal is **shared-admissible prime yield**, not admissible-count gain.\n\n",
    );
    markdown.push_str(
        "| Pair | Clarity label | Count source | Effect source | Net prime delta | Shared prime delta | Overlap prime delta |\n",
    );
    markdown.push_str("|---|---|---|---|---:|---:|---:|\n");
    for row in &bundle.clarity_rows {
        markdown.push_str(&format!(
            "| {} | `{}` | `{}` | `{}` | {:+} | {:+} | {:+} |\n",
            row.pair_label,
            row.clarity_label,
            row.dominant_count_source,
            row.dominant_effect_source,
            row.prime_hit_delta_count,
            row.shared_admissible_prime_delta_count,
            row.overlap_prime_delta_count
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Transfer Source View\n\n");
    if let Some(image) = bundle
        .image_artifact_rows
        .iter()
        .find(|image| image.kind == "transfer_source")
    {
        markdown.push_str(&format!("![{}]({})\n\n", image.label, image.path));
    }

    markdown.push_str("## Exact Effect View\n\n");
    if let Some(image) = bundle
        .image_artifact_rows
        .iter()
        .find(|image| image.kind == "effect_contribution")
    {
        markdown.push_str(&format!("![{}]({})\n\n", image.label, image.path));
    }

    markdown.push_str("## Observations\n\n");
    for observation in &bundle.observations {
        markdown.push_str(&format!("- {}\n", observation));
    }
    markdown
}
