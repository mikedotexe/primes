//! Large affine witness ladder report.
//!
//! This report demonstrates the maintained large-prime witness path:
//! symmetric visible decimal constructions are compiled into affine lanes,
//! small residue gates skip exact obstructions, and fixed-base Miller-Rabin
//! confirmation is run only on survivors.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example large_affine_witness_ladder_report -- \
//!   --profile smoke \
//!   --out-dir /tmp/primes_large_affine_witness_ladder_smoke
//! ```

use plotters::prelude::*;
use primes::validation::{
    large_affine_witness::{
        build_large_witness_report, middle_length_for_visible_digits, AffineWitnessRow, BackendRow,
        ControlRow, LargeWitnessReportData, LargeWitnessSettings, RarityRow, WitnessGalleryRow,
        PROBABLE_PRIME_BASES,
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
    process::Command,
    time::Instant,
};

const DEFAULT_OUT_DIR: &str = "/tmp/primes_large_affine_witness_ladder";
const ARTIFACT_ID: &str = "large_affine_witness_ladder_report";
const EXPORT_VERSION: u32 = 1;
const SMOKE_VISIBLE_DIGITS: &[usize] = &[22, 38];
const RELEASE_VISIBLE_DIGITS: &[usize] = &[22, 28, 38, 50, 75, 100, 128];
const SMOKE_SEED_COUNT: u64 = 2_000;
const RELEASE_SEED_COUNT: u64 = 20_000;
const SMOKE_CONTROL_SAMPLE_COUNT: u64 = 300;
const RELEASE_CONTROL_SAMPLE_COUNT: u64 = 2_000;
const DEFAULT_MAX_WITNESSES: usize = 5;
const SMOKE_OPENSSL_COUNT: u64 = 1;
const RELEASE_OPENSSL_COUNT: u64 = 3;

#[derive(Debug, Clone)]
struct Options {
    out_dir: PathBuf,
    profile: String,
    visible_digits: Vec<usize>,
    seed_count: u64,
    control_sample_count: u64,
    max_witnesses: usize,
    openssl_count: u64,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    profile: String,
    visible_digits: Vec<usize>,
    middle_lengths: Vec<usize>,
    seed_count: u64,
    control_sample_count: u64,
    max_witnesses: usize,
    openssl_count: u64,
    confirmation_scope: String,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    rung_count: usize,
    largest_visible_digits: usize,
    total_affine_witnesses: u64,
    fastest_time_to_first_role: String,
    fastest_time_to_first_seconds: f64,
    strongest_raw_hit_rate_role: String,
    strongest_raw_hit_rate: f64,
    widest_first_witness_role: String,
    widest_first_witness_digits: usize,
    strongest_line: String,
    caution_line: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    summary: ReportSummary,
    affine_witness_rows: Vec<AffineWitnessRow>,
    backend_rows: Vec<BackendRow>,
    control_rows: Vec<ControlRow>,
    rarity_rows: Vec<RarityRow>,
    witness_gallery_rows: Vec<WitnessGalleryRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let middle_lengths = options
        .visible_digits
        .iter()
        .map(|&digits| {
            middle_length_for_visible_digits(digits).unwrap_or_else(|| {
                panic!("visible digit target {digits} is below fixed template width")
            })
        })
        .collect::<Vec<_>>();
    let settings = LargeWitnessSettings {
        profile: options.profile.clone(),
        seed_count: options.seed_count,
        control_sample_count: options.control_sample_count,
        max_witnesses: options.max_witnesses,
        middle_lengths: middle_lengths.clone(),
        probable_prime_bases: PROBABLE_PRIME_BASES.to_vec(),
    };

    let mut data = build_large_witness_report(settings);
    data.control_rows
        .extend(build_external_control_rows(&data, options.openssl_count));

    let report_settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        profile: options.profile.clone(),
        visible_digits: options.visible_digits.clone(),
        middle_lengths,
        seed_count: options.seed_count,
        control_sample_count: options.control_sample_count,
        max_witnesses: options.max_witnesses,
        openssl_count: options.openssl_count,
        confirmation_scope:
            "BigUint fixed-base Miller-Rabin probable-prime confirmation above u64; deterministic primality only where candidates fit u64"
                .to_string(),
    };
    let summary = build_summary(&data.affine_witness_rows);
    let observations = build_observations(&data);

    let image_artifact_rows = render_images(&options.out_dir, &data);
    let report_text = render_report(&report_settings, &summary, &data, &observations);
    let bundle = ReportBundle {
        export_version: EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings: report_settings,
        summary: summary.clone(),
        affine_witness_rows: data.affine_witness_rows.clone(),
        backend_rows: data.backend_rows.clone(),
        control_rows: data.control_rows.clone(),
        rarity_rows: data.rarity_rows.clone(),
        witness_gallery_rows: data.witness_gallery_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        observations,
    };

    write_csv_rows(
        options.out_dir.join("affine_witness_rows.csv"),
        &data.affine_witness_rows,
    )
    .expect("write affine witness rows");
    write_csv_rows(options.out_dir.join("backend_rows.csv"), &data.backend_rows)
        .expect("write backend rows");
    write_csv_rows(options.out_dir.join("control_rows.csv"), &data.control_rows)
        .expect("write control rows");
    write_csv_rows(options.out_dir.join("rarity_rows.csv"), &data.rarity_rows)
        .expect("write rarity rows");
    write_csv_rows(
        options.out_dir.join("witness_gallery_rows.csv"),
        &data.witness_gallery_rows,
    )
    .expect("write witness gallery rows");
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
                "large_affine_witness_ladder_report".to_string(),
            ],
            upstream_inputs: vec![
                "examples/large_affine_witness_ladder_report.rs".to_string(),
                "src/validation/large_affine_witness.rs".to_string(),
            ],
            expected_outputs: vec![
                "report.md".to_string(),
                "summary.json".to_string(),
                "affine_witness_rows.csv".to_string(),
                "control_rows.csv".to_string(),
                "backend_rows.csv".to_string(),
                "rarity_rows.csv".to_string(),
                "witness_gallery_rows.csv".to_string(),
                "witness_ladder.png".to_string(),
                "time_to_first.png".to_string(),
                "seeds_per_witness.png".to_string(),
                "residue_funnel.png".to_string(),
                "control_comparison.png".to_string(),
                "semantic_rarity_strip.png".to_string(),
                "artifact_manifest.json".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote large affine witness ladder bundle to {}",
        options.out_dir.display()
    );
    println!("{}", summary.strongest_line);
}

fn parse_args() -> Options {
    let mut profile = "smoke".to_string();
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut explicit_visible_digits = None;
    let mut explicit_seed_count = None;
    let mut explicit_control_sample_count = None;
    let mut max_witnesses = DEFAULT_MAX_WITNESSES;
    let mut explicit_openssl_count = None;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => out_dir = PathBuf::from(args.next().expect("missing --out-dir value")),
            "--profile" => profile = args.next().expect("missing --profile value"),
            "--visible-digits" => {
                explicit_visible_digits = Some(parse_usize_list(
                    &args.next().expect("missing --visible-digits value"),
                ))
            }
            "--seed-count" => {
                explicit_seed_count = Some(
                    args.next()
                        .expect("missing --seed-count value")
                        .parse()
                        .expect("invalid --seed-count"),
                )
            }
            "--control-sample-count" => {
                explicit_control_sample_count = Some(
                    args.next()
                        .expect("missing --control-sample-count value")
                        .parse()
                        .expect("invalid --control-sample-count"),
                )
            }
            "--max-witnesses" => {
                max_witnesses = args
                    .next()
                    .expect("missing --max-witnesses value")
                    .parse()
                    .expect("invalid --max-witnesses")
            }
            "--openssl-count" => {
                explicit_openssl_count = Some(
                    args.next()
                        .expect("missing --openssl-count value")
                        .parse()
                        .expect("invalid --openssl-count"),
                )
            }
            _ => panic!("unrecognized argument: {arg}"),
        }
    }

    let profile_defaults = match profile.as_str() {
        "smoke" => (
            SMOKE_VISIBLE_DIGITS.to_vec(),
            SMOKE_SEED_COUNT,
            SMOKE_CONTROL_SAMPLE_COUNT,
            SMOKE_OPENSSL_COUNT,
        ),
        "release" => (
            RELEASE_VISIBLE_DIGITS.to_vec(),
            RELEASE_SEED_COUNT,
            RELEASE_CONTROL_SAMPLE_COUNT,
            RELEASE_OPENSSL_COUNT,
        ),
        _ => panic!("unknown --profile `{profile}`; expected smoke or release"),
    };

    Options {
        out_dir,
        profile,
        visible_digits: explicit_visible_digits.unwrap_or(profile_defaults.0),
        seed_count: explicit_seed_count.unwrap_or(profile_defaults.1),
        control_sample_count: explicit_control_sample_count.unwrap_or(profile_defaults.2),
        max_witnesses,
        openssl_count: explicit_openssl_count.unwrap_or(profile_defaults.3),
    }
}

fn parse_usize_list(value: &str) -> Vec<usize> {
    value
        .split(',')
        .filter(|part| !part.trim().is_empty())
        .map(|part| part.trim().parse().expect("invalid visible digit value"))
        .collect()
}

fn build_summary(rows: &[AffineWitnessRow]) -> ReportSummary {
    let largest_visible_digits = rows
        .iter()
        .map(|row| row.visible_digits)
        .max()
        .unwrap_or_default();
    let total_affine_witnesses = rows.iter().map(|row| row.witnesses_found).sum();
    let fastest_first = rows
        .iter()
        .filter(|row| row.first_witness_seed.is_some())
        .min_by(|left, right| {
            left.time_to_first_witness_seconds
                .partial_cmp(&right.time_to_first_witness_seconds)
                .unwrap()
        })
        .or_else(|| rows.first())
        .expect("at least one rung");
    let strongest_rate = rows
        .iter()
        .max_by(|left, right| left.raw_hit_rate.partial_cmp(&right.raw_hit_rate).unwrap())
        .expect("at least one rung");
    let widest_first = rows
        .iter()
        .filter(|row| !row.first_witness_value.is_empty())
        .max_by_key(|row| row.decimal_digits)
        .or_else(|| rows.first())
        .expect("at least one rung");

    ReportSummary {
        rung_count: rows.len(),
        largest_visible_digits,
        total_affine_witnesses,
        fastest_time_to_first_role: fastest_first.role.clone(),
        fastest_time_to_first_seconds: fastest_first.time_to_first_witness_seconds,
        strongest_raw_hit_rate_role: strongest_rate.role.clone(),
        strongest_raw_hit_rate: strongest_rate.raw_hit_rate,
        widest_first_witness_role: widest_first.role.clone(),
        widest_first_witness_digits: widest_first.decimal_digits,
        strongest_line:
            "This engine generates large, human-readable prime witnesses by compiling symmetric digit constructions into affine search lanes with cheap residue filtering."
                .to_string(),
        caution_line:
            "The comparison claim is not that this beats general-purpose prime generators; it targets a named readable construction family that ordinary tools do not preserve."
                .to_string(),
    }
}

fn build_observations(data: &LargeWitnessReportData) -> Vec<String> {
    let best_survivor = data
        .affine_witness_rows
        .iter()
        .max_by(|left, right| {
            left.survivor_hit_rate
                .partial_cmp(&right.survivor_hit_rate)
                .unwrap()
        })
        .expect("at least one affine row");
    let largest = data
        .affine_witness_rows
        .iter()
        .max_by_key(|row| row.visible_digits)
        .expect("at least one affine row");
    let unavailable_backends = data
        .backend_rows
        .iter()
        .filter(|row| row.status != "ok")
        .count();

    vec![
        format!(
            "The largest rung scanned here is `{}` visible digits; its first witness is `{}` and reported as `{}` confirmation.",
            largest.visible_digits, largest.first_witness_mersenne_class, largest.confirmation
        ),
        format!(
            "The strongest survivor-yield row is `{}` at {:.2}% witnesses among residue survivors.",
            best_survivor.role,
            best_survivor.survivor_hit_rate * 100.0
        ),
        format!(
            "{} fixed-width backend rows are explicitly labeled out of scope instead of silently changing semantics.",
            unavailable_backends
        ),
        "OpenSSL calibration rows generate ordinary random probable primes at comparable bit widths; they are a capability baseline, not a same-family control.".to_string(),
        "Primesieve is only a fair interval-count calibration inside u64; larger witness rungs are reported as out of scope for it.".to_string(),
    ]
}

fn build_external_control_rows(
    data: &LargeWitnessReportData,
    openssl_count: u64,
) -> Vec<ControlRow> {
    data.affine_witness_rows
        .iter()
        .flat_map(|row| {
            [
                openssl_control_row(row, openssl_count),
                primesieve_control_row(row),
            ]
        })
        .collect()
}

fn openssl_control_row(row: &AffineWitnessRow, openssl_count: u64) -> ControlRow {
    if openssl_count == 0 {
        return ControlRow {
            role: row.role.clone(),
            control_type: "openssl_random_probable_prime".to_string(),
            status: "disabled".to_string(),
            visible_digits: row.visible_digits,
            sample_count: 0,
            candidates_tested: 0,
            witnesses_found: 0,
            raw_hit_rate: 0.0,
            elapsed_seconds: 0.0,
            witnesses_per_second: 0.0,
            first_witness_value: String::new(),
            note: "OpenSSL calibration disabled by --openssl-count 0".to_string(),
        };
    }

    let bits = ((row.visible_digits as f64) * std::f64::consts::LOG2_10).ceil() as u64;
    let started = Instant::now();
    let mut first = String::new();
    let mut ok = 0u64;
    let mut failure = None;
    for _ in 0..openssl_count {
        match Command::new("openssl")
            .args(["prime", "-generate", "-bits", &bits.to_string()])
            .output()
        {
            Ok(output) if output.status.success() => {
                ok += 1;
                if first.is_empty() {
                    first = String::from_utf8_lossy(&output.stdout).trim().to_string();
                }
            }
            Ok(output) => {
                failure = Some(String::from_utf8_lossy(&output.stderr).trim().to_string());
                break;
            }
            Err(err) => {
                failure = Some(err.to_string());
                break;
            }
        }
    }
    let elapsed = started.elapsed().as_secs_f64().max(1e-12);
    let status = if ok == openssl_count {
        "ok"
    } else {
        "unavailable"
    };
    ControlRow {
        role: row.role.clone(),
        control_type: "openssl_random_probable_prime".to_string(),
        status: status.to_string(),
        visible_digits: row.visible_digits,
        sample_count: openssl_count,
        candidates_tested: ok,
        witnesses_found: ok,
        raw_hit_rate: if ok == openssl_count { 1.0 } else { 0.0 },
        elapsed_seconds: elapsed,
        witnesses_per_second: if ok == 0 { 0.0 } else { ok as f64 / elapsed },
        first_witness_value: first,
        note: failure.unwrap_or_else(|| {
            format!("OpenSSL generated probable primes at {bits} bits; this does not preserve the membrane template.")
        }),
    }
}

fn primesieve_control_row(row: &AffineWitnessRow) -> ControlRow {
    let Some(center) = row.first_witness_value.parse::<u64>().ok() else {
        return ControlRow {
            role: row.role.clone(),
            control_type: "primesieve_interval_calibration".to_string(),
            status: "out_of_scope_above_u64".to_string(),
            visible_digits: row.visible_digits,
            sample_count: 0,
            candidates_tested: 0,
            witnesses_found: 0,
            raw_hit_rate: 0.0,
            elapsed_seconds: 0.0,
            witnesses_per_second: 0.0,
            first_witness_value: String::new(),
            note: "primesieve interval calibration is only measured for u64-safe rows".to_string(),
        };
    };

    let width = row.seed_count.min(1_000_000);
    let start = center.saturating_sub(width / 2);
    let stop = center.saturating_add(width / 2);
    let started = Instant::now();
    let output = Command::new("primesieve")
        .args([start.to_string(), stop.to_string()])
        .output();
    let elapsed = started.elapsed().as_secs_f64().max(1e-12);
    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let count = stdout
                .split_whitespace()
                .filter_map(|part| part.parse::<u64>().ok())
                .next_back()
                .unwrap_or(0);
            ControlRow {
                role: row.role.clone(),
                control_type: "primesieve_interval_calibration".to_string(),
                status: "ok".to_string(),
                visible_digits: row.visible_digits,
                sample_count: stop.saturating_sub(start).saturating_add(1),
                candidates_tested: stop.saturating_sub(start).saturating_add(1),
                witnesses_found: count,
                raw_hit_rate: count as f64 / stop.saturating_sub(start).saturating_add(1) as f64,
                elapsed_seconds: elapsed,
                witnesses_per_second: count as f64 / elapsed,
                first_witness_value: String::new(),
                note: "Prime count in a local u64 interval; this is not template-preserving."
                    .to_string(),
            }
        }
        Ok(output) => ControlRow {
            role: row.role.clone(),
            control_type: "primesieve_interval_calibration".to_string(),
            status: "unavailable".to_string(),
            visible_digits: row.visible_digits,
            sample_count: 0,
            candidates_tested: 0,
            witnesses_found: 0,
            raw_hit_rate: 0.0,
            elapsed_seconds: elapsed,
            witnesses_per_second: 0.0,
            first_witness_value: String::new(),
            note: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        },
        Err(err) => ControlRow {
            role: row.role.clone(),
            control_type: "primesieve_interval_calibration".to_string(),
            status: "unavailable".to_string(),
            visible_digits: row.visible_digits,
            sample_count: 0,
            candidates_tested: 0,
            witnesses_found: 0,
            raw_hit_rate: 0.0,
            elapsed_seconds: elapsed,
            witnesses_per_second: 0.0,
            first_witness_value: String::new(),
            note: err.to_string(),
        },
    }
}

fn render_report(
    settings: &ReportSettings,
    summary: &ReportSummary,
    data: &LargeWitnessReportData,
    observations: &[String],
) -> String {
    let mut lines = Vec::new();
    lines.push("# Large Affine Witness Ladder Report".to_string());
    lines.push(String::new());
    lines.push("## Frame".to_string());
    lines.push(format!("- {}", summary.strongest_line));
    lines.push(format!("- {}", summary.caution_line));
    lines.push("- Above `u64`, `prime` in this report means fixed-base Miller-Rabin probable-prime witness unless explicitly labeled deterministic.".to_string());
    lines.push(String::new());
    lines.push("## Settings".to_string());
    lines.push(format!("- output dir: `{}`", settings.out_dir));
    lines.push(format!("- profile: `{}`", settings.profile));
    lines.push(format!(
        "- visible digit ladder: `{:?}`",
        settings.visible_digits
    ));
    lines.push(format!("- seed count per rung: `{}`", settings.seed_count));
    lines.push(format!(
        "- control sample count per local control: `{}`",
        settings.control_sample_count
    ));
    lines.push(format!(
        "- confirmation scope: `{}`",
        settings.confirmation_scope
    ));
    lines.push(String::new());
    lines.push("## Headline".to_string());
    lines.push(format!(
        "- rungs: `{}`, largest rung: `{}` visible digits, total affine witnesses: `{}`",
        summary.rung_count, summary.largest_visible_digits, summary.total_affine_witnesses
    ));
    lines.push(format!(
        "- fastest time to first witness: `{}` in {:.4}s",
        summary.fastest_time_to_first_role, summary.fastest_time_to_first_seconds
    ));
    lines.push(format!(
        "- strongest raw hit rate: `{}` at {:.2}%",
        summary.strongest_raw_hit_rate_role,
        summary.strongest_raw_hit_rate * 100.0
    ));
    lines.push(format!(
        "- widest first witness row: `{}` at `{}` decimal digits",
        summary.widest_first_witness_role, summary.widest_first_witness_digits
    ));
    lines.push(String::new());
    lines.push("## Affine Ladder".to_string());
    lines.push("| Visible digits | First seed | Witnesses | Raw hit | Survivor share | Survivor hit | Time to first | Seeds/witness | Mersenne class |".to_string());
    lines.push("|---:|---:|---:|---:|---:|---:|---:|---:|---|".to_string());
    for row in &data.affine_witness_rows {
        lines.push(format!(
            "| {} | {} | {} | {:.2}% | {:.2}% | {:.2}% | {:.4}s | {:.1} | `{}` |",
            row.visible_digits,
            row.first_witness_seed
                .map(|seed| seed.to_string())
                .unwrap_or_else(|| "-".to_string()),
            row.witnesses_found,
            row.raw_hit_rate * 100.0,
            row.residue_survivor_share * 100.0,
            row.survivor_hit_rate * 100.0,
            row.time_to_first_witness_seconds,
            row.seeds_per_witness,
            row.first_witness_mersenne_class
        ));
    }
    lines.push(String::new());
    lines.push("## Backend Scope".to_string());
    lines.push("| Role | Backend | Status | Witnesses | Witnesses/s | Note |".to_string());
    lines.push("|---|---|---|---:|---:|---|".to_string());
    for row in &data.backend_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | {} | {:.2} | {} |",
            row.role,
            row.backend,
            row.status,
            row.witnesses_found,
            row.witnesses_per_second,
            row.note
        ));
    }
    lines.push(String::new());
    lines.push("## Controls".to_string());
    lines.push("| Role | Control | Status | Samples | Hits | Hit rate | Note |".to_string());
    lines.push("|---|---|---|---:|---:|---:|---|".to_string());
    for row in &data.control_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | {} | {} | {:.2}% | {} |",
            row.role,
            row.control_type,
            row.status,
            row.sample_count,
            row.witnesses_found,
            row.raw_hit_rate * 100.0,
            row.note
        ));
    }
    lines.push(String::new());
    lines.push("## Semantic Rarity".to_string());
    lines.push("| Visible digits | Template log10 space | Same-digit log10 space | Template share log10 | Description |".to_string());
    lines.push("|---:|---:|---:|---:|---|".to_string());
    for row in &data.rarity_rows {
        lines.push(format!(
            "| {} | {:.2} | {:.2} | {:.2} | `{}` |",
            row.visible_digits,
            row.template_space_log10,
            row.same_digit_space_log10,
            row.template_share_log10,
            row.compact_description_example
        ));
    }
    lines.push(String::new());
    lines.push("## Witness Gallery".to_string());
    lines.push("| Role | Rank | Seed | Mersenne class | Template | Decimal value |".to_string());
    lines.push("|---|---:|---:|---|---|---|".to_string());
    for row in data.witness_gallery_rows.iter().take(20) {
        lines.push(format!(
            "| `{}` | {} | {} | `{}` | `{}` | `{}` |",
            row.role,
            row.rank,
            row.seed,
            row.mersenne_class,
            row.template_digits,
            abbreviate(&row.decimal_value, 48)
        ));
    }
    lines.push(String::new());
    lines.push("## Read Carefully".to_string());
    for observation in observations {
        lines.push(format!("- {}", observation));
    }
    lines.push(String::new());
    lines.push("## Artifacts".to_string());
    lines.push("- `affine_witness_rows.csv`: per-rung witness yield, time-to-first, residue funnel, and first witness.".to_string());
    lines.push(
        "- `backend_rows.csv`: BigUint, u128, and u64 confirmation scope and throughput rows."
            .to_string(),
    );
    lines.push(
        "- `control_rows.csv`: random, same-slot, OpenSSL, and primesieve calibration rows."
            .to_string(),
    );
    lines.push(
        "- `rarity_rows.csv`: template-space share among same-digit decimal strings.".to_string(),
    );
    lines.push(
        "- `witness_gallery_rows.csv`: first witness examples and compact generative descriptions."
            .to_string(),
    );
    lines.push("- PNGs: witness ladder, time-to-first, seeds-per-witness, residue funnel, control comparison, and semantic rarity.".to_string());
    lines.join("\n")
}

fn render_images(out_dir: &Path, data: &LargeWitnessReportData) -> Vec<ImageArtifactRow> {
    let specs = [
        (
            "witness_ladder",
            "Witness yield by visible digit rung",
            "witness_ladder.png",
            render_witness_ladder as fn(&Path, &LargeWitnessReportData),
        ),
        (
            "time_to_first",
            "Time to first witness by rung",
            "time_to_first.png",
            render_time_to_first,
        ),
        (
            "seeds_per_witness",
            "Seeds per witness by rung",
            "seeds_per_witness.png",
            render_seeds_per_witness,
        ),
        (
            "residue_funnel",
            "Raw seeds to residue survivors to witnesses",
            "residue_funnel.png",
            render_residue_funnel,
        ),
        (
            "control_comparison",
            "Affine lane compared with local controls",
            "control_comparison.png",
            render_control_comparison,
        ),
        (
            "semantic_rarity",
            "Template share among same-digit strings",
            "semantic_rarity_strip.png",
            render_semantic_rarity,
        ),
    ];

    specs
        .iter()
        .map(|(kind, label, filename, render)| {
            let path = out_dir.join(filename);
            render(&path, data);
            ImageArtifactRow {
                kind: (*kind).to_string(),
                label: (*label).to_string(),
                path: path.display().to_string(),
            }
        })
        .collect()
}

fn render_witness_ladder(path: &Path, data: &LargeWitnessReportData) {
    render_bar_chart(
        path,
        "Large affine witness ladder",
        &data
            .affine_witness_rows
            .iter()
            .map(|row| {
                (
                    format!("{}d", row.visible_digits),
                    row.witnesses_found as f64,
                )
            })
            .collect::<Vec<_>>(),
        "witnesses",
        RGBColor(43, 108, 176),
    );
}

fn render_time_to_first(path: &Path, data: &LargeWitnessReportData) {
    render_bar_chart(
        path,
        "Time to first witness",
        &data
            .affine_witness_rows
            .iter()
            .map(|row| {
                (
                    format!("{}d", row.visible_digits),
                    row.time_to_first_witness_seconds.max(0.000001),
                )
            })
            .collect::<Vec<_>>(),
        "seconds",
        RGBColor(214, 106, 46),
    );
}

fn render_seeds_per_witness(path: &Path, data: &LargeWitnessReportData) {
    render_bar_chart(
        path,
        "Seeds per witness",
        &data
            .affine_witness_rows
            .iter()
            .map(|row| {
                (
                    format!("{}d", row.visible_digits),
                    row.seeds_per_witness.max(0.000001),
                )
            })
            .collect::<Vec<_>>(),
        "seeds",
        RGBColor(38, 139, 112),
    );
}

fn render_residue_funnel(path: &Path, data: &LargeWitnessReportData) {
    let raw: u64 = data
        .affine_witness_rows
        .iter()
        .map(|row| row.seed_count)
        .sum();
    let survivors: u64 = data
        .affine_witness_rows
        .iter()
        .map(|row| row.residue_survivor_count)
        .sum();
    let tests: u64 = data
        .affine_witness_rows
        .iter()
        .map(|row| row.probable_prime_tests)
        .sum();
    let witnesses: u64 = data
        .affine_witness_rows
        .iter()
        .map(|row| row.witnesses_found)
        .sum();
    render_bar_chart(
        path,
        "Residue funnel",
        &[
            ("raw seeds".to_string(), raw as f64),
            ("survivors".to_string(), survivors as f64),
            ("tests".to_string(), tests as f64),
            ("witnesses".to_string(), witnesses as f64),
        ],
        "count",
        RGBColor(105, 88, 158),
    );
}

fn render_control_comparison(path: &Path, data: &LargeWitnessReportData) {
    let rows = data
        .affine_witness_rows
        .iter()
        .map(|row| {
            (
                format!("affine {}d", row.visible_digits),
                row.raw_hit_rate * 100.0,
            )
        })
        .chain(
            data.control_rows
                .iter()
                .filter(|row| {
                    row.status == "ok"
                        && matches!(
                            row.control_type.as_str(),
                            "random_odd_same_digits"
                                | "random_coprime_same_digits"
                                | "same_slot_random_membrane"
                        )
                })
                .take(12)
                .map(|row| {
                    (
                        format!(
                            "{} {}d",
                            short_control_label(&row.control_type),
                            row.visible_digits
                        ),
                        row.raw_hit_rate * 100.0,
                    )
                }),
        )
        .collect::<Vec<_>>();
    render_bar_chart(
        path,
        "Affine witness rate and local controls",
        &rows,
        "hit rate (%)",
        RGBColor(170, 77, 57),
    );
}

fn render_semantic_rarity(path: &Path, data: &LargeWitnessReportData) {
    render_bar_chart(
        path,
        "Template share among same-digit strings",
        &data
            .rarity_rows
            .iter()
            .map(|row| {
                (
                    format!("{}d", row.visible_digits),
                    row.template_share_log10.abs(),
                )
            })
            .collect::<Vec<_>>(),
        "-log10 share",
        RGBColor(86, 109, 120),
    );
}

fn render_bar_chart(
    path: &Path,
    title: &str,
    rows: &[(String, f64)],
    y_label: &str,
    color: RGBColor,
) {
    let root = BitMapBackend::new(path, (1400, 820)).into_drawing_area();
    root.fill(&RGBColor(250, 249, 246)).unwrap();
    let max_value = rows
        .iter()
        .map(|(_, value)| *value)
        .fold(0.0_f64, f64::max)
        .max(1.0);
    let mut chart = ChartBuilder::on(&root)
        .caption(
            title,
            ("sans-serif", 34).into_font().color(&RGBColor(38, 44, 51)),
        )
        .margin(32)
        .x_label_area_size(90)
        .y_label_area_size(90)
        .build_cartesian_2d(0..rows.len(), 0.0..(max_value * 1.15))
        .unwrap();
    chart
        .configure_mesh()
        .disable_mesh()
        .y_desc(y_label)
        .x_labels(rows.len())
        .x_label_formatter(&|idx| {
            rows.get(*idx)
                .map(|(label, _)| label.clone())
                .unwrap_or_default()
        })
        .label_style(("sans-serif", 15).into_font().color(&RGBColor(70, 76, 84)))
        .axis_desc_style(("sans-serif", 20).into_font().color(&RGBColor(70, 76, 84)))
        .draw()
        .unwrap();
    chart
        .draw_series(rows.iter().enumerate().map(|(idx, (_, value))| {
            Rectangle::new(
                [(idx, 0.0), (idx + 1, *value)],
                ShapeStyle::from(&color).filled(),
            )
        }))
        .unwrap();
    for (idx, (_, value)) in rows.iter().enumerate() {
        chart
            .draw_series(std::iter::once(Text::new(
                format_value(*value),
                (idx, (*value + max_value * 0.025).min(max_value * 1.12)),
                ("sans-serif", 15).into_font().color(&RGBColor(38, 44, 51)),
            )))
            .unwrap();
    }
    root.present().unwrap();
}

fn short_control_label(control_type: &str) -> &'static str {
    match control_type {
        "random_odd_same_digits" => "odd",
        "random_coprime_same_digits" => "coprime",
        "same_slot_random_membrane" => "slot",
        "openssl_random_probable_prime" => "openssl",
        "primesieve_interval_calibration" => "sieve",
        _ => "control",
    }
}

fn format_value(value: f64) -> String {
    if value >= 1000.0 {
        format!("{value:.0}")
    } else if value >= 10.0 {
        format!("{value:.1}")
    } else {
        format!("{value:.3}")
    }
}

fn abbreviate(value: &str, max_len: usize) -> String {
    if value.len() <= max_len {
        value.to_string()
    } else {
        let head = max_len.saturating_sub(18);
        format!("{}...{}", &value[..head], &value[value.len() - 15..])
    }
}
