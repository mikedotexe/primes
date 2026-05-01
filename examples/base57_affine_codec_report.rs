//! Base57 affine codec experiment report.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example base57_affine_codec_report -- --out-dir /tmp/primes_base57_affine_codec
//! ```

use plotters::prelude::*;
use primes::validation::{
    base57_affine_codec::{
        encode_affine, encode_bytes, format_hex, parse_hex, residue_moduli_for_affine_base,
        AffineChunkRecord, AffineCodecMode, CodecAlphabet, AFFINE_CHUNK_DIGITS, AFFINE_INNER,
        AFFINE_MIDDLE_LENGTH, AFFINE_NONCE_BITS, AFFINE_NONCE_SPACE, AFFINE_OUTER,
        AFFINE_PAYLOAD_BYTES_PER_CHUNK, BASE57_ALPHABET, BASE57_DROPPED_CHAR,
        BITCOIN_BASE58_ALPHABET,
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

const DEFAULT_OUT_DIR: &str = "/tmp/primes_base57_affine_codec";
const ARTIFACT_ID: &str = "base57_affine_codec_report";
const EXPORT_VERSION: u32 = 1;

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
}

#[derive(Debug, Clone)]
struct SamplePayload {
    name: String,
    payload: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
struct BaselineCodecRow {
    sample_name: String,
    payload_len: usize,
    payload_hex: String,
    base58: String,
    base57: String,
    base58_len: usize,
    base57_len: usize,
    base57_minus_base58_chars: isize,
    base57_vs_base58_ratio: f64,
    base58_roundtrip_ok: bool,
    base57_roundtrip_ok: bool,
}

#[derive(Debug, Clone, Serialize)]
struct AffineCodecRow {
    sample_name: String,
    mode: String,
    payload_len: usize,
    notation: String,
    notation_len: usize,
    body_len: usize,
    chunk_count: usize,
    baseline_base58_len: usize,
    baseline_base57_len: usize,
    overhead_vs_base58_chars: isize,
    overhead_vs_base57_chars: isize,
    total_attempts: u64,
    average_attempts_per_chunk: f64,
    max_attempts_for_chunk: u16,
    prime_chunk_count: usize,
    residue_admissible_chunk_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct ChunkRow {
    sample_name: String,
    mode: String,
    chunk_index: usize,
    payload_hex: String,
    payload_value: u32,
    nonce: u16,
    attempts: u16,
    seed: u64,
    middle_digits: String,
    chunk_text: String,
    candidate_value: u64,
    residue_admissible: bool,
    prime: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct BaseInvariantRow {
    sample_name: String,
    payload_len: usize,
    leading_zero_bytes: usize,
    canonical_hex: String,
    decimal_value: String,
    base58: String,
    base57: String,
    affine_residue: String,
    affine_prime: String,
    base58_len: usize,
    base57_len: usize,
    affine_residue_len: usize,
    affine_prime_len: usize,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    sample_count: usize,
    total_payload_bytes: usize,
    base58_alphabet_len: usize,
    base57_alphabet_len: usize,
    dropped_char: char,
    mean_base57_vs_base58_ratio: f64,
    max_base57_minus_base58_chars: isize,
    residue_notation_mean_overhead_vs_base58: f64,
    prime_notation_mean_overhead_vs_base58: f64,
    residue_total_chunks: usize,
    prime_total_chunks: usize,
    residue_average_attempts_per_chunk: f64,
    prime_average_attempts_per_chunk: f64,
    affine_chunk_digits: usize,
    affine_payload_bytes_per_chunk: usize,
    affine_nonce_bits: u32,
    affine_nonce_space: u16,
    residue_moduli_label: String,
    strong_line: String,
    caution_line: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    summary: ReportSummary,
    baseline_codec_rows: Vec<BaselineCodecRow>,
    affine_codec_rows: Vec<AffineCodecRow>,
    base_invariant_rows: Vec<BaseInvariantRow>,
    chunk_rows: Vec<ChunkRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("create output directory");

    let samples = sample_payloads();
    let baseline_rows = build_baseline_rows(&samples);
    let (affine_rows, chunk_rows) = build_affine_rows(&samples, &baseline_rows);
    let base_invariant_rows = build_base_invariant_rows(&baseline_rows, &affine_rows);
    let summary = build_summary(&baseline_rows, &affine_rows);

    let expansion_path = options.out_dir.join("base58_vs_base57_expansion.png");
    render_codec_comparison_panel(&baseline_rows, &affine_rows, &expansion_path);
    let funnel_path = options.out_dir.join("affine_chunk_funnel.png");
    render_affine_chunk_funnel(&affine_rows, &funnel_path);
    let anatomy_path = options.out_dir.join("notation_anatomy_strip.png");
    render_notation_anatomy_strip(&affine_rows, &anatomy_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "base58_vs_base57_expansion".to_string(),
            label: "Codec delta and affine envelope".to_string(),
            path: expansion_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "affine_chunk_funnel".to_string(),
            label: "Affine chunk nonce funnel".to_string(),
            path: funnel_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "notation_anatomy_strip".to_string(),
            label: "Notation anatomy strip".to_string(),
            path: anatomy_path.display().to_string(),
        },
    ];

    let bundle = ReportBundle {
        export_version: EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        summary,
        baseline_codec_rows: baseline_rows,
        affine_codec_rows: affine_rows,
        base_invariant_rows,
        chunk_rows,
        image_artifact_rows,
    };

    write_csv_rows(
        options.out_dir.join("baseline_codec_rows.csv"),
        &bundle.baseline_codec_rows,
    )
    .expect("write baseline codec rows");
    write_csv_rows(
        options.out_dir.join("affine_codec_rows.csv"),
        &bundle.affine_codec_rows,
    )
    .expect("write affine codec rows");
    write_csv_rows(
        options.out_dir.join("base_invariant_rows.csv"),
        &bundle.base_invariant_rows,
    )
    .expect("write base-invariant rows");
    write_csv_rows(options.out_dir.join("chunk_rows.csv"), &bundle.chunk_rows)
        .expect("write chunk rows");
    write_csv_rows(
        options.out_dir.join("image_artifact_rows.csv"),
        &bundle.image_artifact_rows,
    )
    .expect("write image artifact rows");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle).expect("write summary json");
    write_text_file(options.out_dir.join("report.md"), &render_report(&bundle))
        .expect("write report markdown");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo run --release --example base57_affine_codec_report".to_string(),
            args: env::args().skip(1).collect(),
            upstream_inputs: vec![
                "src/validation/base57_affine_codec.rs".to_string(),
                "src/validation/fast_affine.rs".to_string(),
                "src/bin/base57-affine-codec.rs".to_string(),
            ],
            expected_outputs: vec![
                "report.md".to_string(),
                "summary.json".to_string(),
                "baseline_codec_rows.csv".to_string(),
                "affine_codec_rows.csv".to_string(),
                "base_invariant_rows.csv".to_string(),
                "chunk_rows.csv".to_string(),
                "image_artifact_rows.csv".to_string(),
                "base58_vs_base57_expansion.png".to_string(),
                "affine_chunk_funnel.png".to_string(),
                "notation_anatomy_strip.png".to_string(),
                "artifact_manifest.json".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote base57 affine codec report to {}",
        options.out_dir.display()
    );
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => {
                out_dir = PathBuf::from(args.next().unwrap_or_else(|| {
                    eprintln!("missing value for --out-dir");
                    std::process::exit(2);
                }));
            }
            "--help" | "-h" => {
                println!(
                    "Usage: cargo run --release --example base57_affine_codec_report -- --out-dir {DEFAULT_OUT_DIR}"
                );
                std::process::exit(0);
            }
            _ => {
                eprintln!("unknown argument: {arg}");
                std::process::exit(2);
            }
        }
    }
    Options { out_dir }
}

fn sample_payloads() -> Vec<SamplePayload> {
    let mut samples = vec![
        SamplePayload {
            name: "empty".to_string(),
            payload: Vec::new(),
        },
        SamplePayload {
            name: "leading_zero_bytes".to_string(),
            payload: vec![0, 0, 0, 1, 2, 3],
        },
        SamplePayload {
            name: "hello".to_string(),
            payload: b"hello".to_vec(),
        },
    ];
    let mut state = 0x57_58_13_21u64;
    for len in 1..=64 {
        let mut payload = Vec::with_capacity(len);
        for _ in 0..len {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            payload.push((state >> 32) as u8);
        }
        samples.push(SamplePayload {
            name: format!("deterministic_random_{len:02}"),
            payload,
        });
    }
    samples
}

fn build_baseline_rows(samples: &[SamplePayload]) -> Vec<BaselineCodecRow> {
    samples
        .iter()
        .map(|sample| {
            let base58 = encode_bytes(&sample.payload, CodecAlphabet::Base58).expect("base58");
            let base57 = encode_bytes(&sample.payload, CodecAlphabet::Base57).expect("base57");
            let decoded58 = primes::validation::base57_affine_codec::decode_to_bytes(
                &base58,
                CodecAlphabet::Base58,
            )
            .expect("decode base58");
            let decoded57 = primes::validation::base57_affine_codec::decode_to_bytes(
                &base57,
                CodecAlphabet::Base57,
            )
            .expect("decode base57");
            BaselineCodecRow {
                sample_name: sample.name.clone(),
                payload_len: sample.payload.len(),
                payload_hex: format_hex(&sample.payload),
                base58_len: base58.len(),
                base57_len: base57.len(),
                base57_minus_base58_chars: base57.len() as isize - base58.len() as isize,
                base57_vs_base58_ratio: ratio(base57.len(), base58.len()),
                base58_roundtrip_ok: decoded58 == sample.payload,
                base57_roundtrip_ok: decoded57 == sample.payload,
                base58,
                base57,
            }
        })
        .collect()
}

fn build_affine_rows(
    samples: &[SamplePayload],
    baseline_rows: &[BaselineCodecRow],
) -> (Vec<AffineCodecRow>, Vec<ChunkRow>) {
    let mut affine_rows = Vec::new();
    let mut chunk_rows = Vec::new();
    for sample in samples {
        let baseline = baseline_rows
            .iter()
            .find(|row| row.sample_name == sample.name)
            .expect("baseline row should exist");
        for mode in [AffineCodecMode::Residue, AffineCodecMode::Prime] {
            let encoded = encode_affine(&sample.payload, mode).expect("affine encode");
            let total_attempts = encoded
                .chunks
                .iter()
                .map(|chunk| u64::from(chunk.attempts))
                .sum::<u64>();
            let max_attempts_for_chunk = encoded
                .chunks
                .iter()
                .map(|chunk| chunk.attempts)
                .max()
                .unwrap_or(0);
            let prime_chunk_count = encoded.chunks.iter().filter(|chunk| chunk.prime).count();
            let residue_admissible_chunk_count = encoded
                .chunks
                .iter()
                .filter(|chunk| chunk.residue_admissible)
                .count();
            affine_rows.push(AffineCodecRow {
                sample_name: sample.name.clone(),
                mode: mode_label(mode).to_string(),
                payload_len: sample.payload.len(),
                notation_len: encoded.notation.len(),
                body_len: encoded.body_len,
                chunk_count: encoded.chunk_count,
                baseline_base58_len: baseline.base58_len,
                baseline_base57_len: baseline.base57_len,
                overhead_vs_base58_chars: encoded.notation.len() as isize
                    - baseline.base58_len as isize,
                overhead_vs_base57_chars: encoded.notation.len() as isize
                    - baseline.base57_len as isize,
                total_attempts,
                average_attempts_per_chunk: ratio(total_attempts as usize, encoded.chunk_count),
                max_attempts_for_chunk,
                prime_chunk_count,
                residue_admissible_chunk_count,
                notation: encoded.notation.clone(),
            });
            chunk_rows.extend(
                encoded
                    .chunks
                    .iter()
                    .map(|chunk| chunk_row(&sample.name, mode, chunk)),
            );
        }
    }
    (affine_rows, chunk_rows)
}

fn chunk_row(sample_name: &str, mode: AffineCodecMode, chunk: &AffineChunkRecord) -> ChunkRow {
    ChunkRow {
        sample_name: sample_name.to_string(),
        mode: mode_label(mode).to_string(),
        chunk_index: chunk.chunk_index,
        payload_hex: chunk.payload_hex.clone(),
        payload_value: chunk.payload_value,
        nonce: chunk.nonce,
        attempts: chunk.attempts,
        seed: chunk.seed,
        middle_digits: chunk.middle_digits.clone(),
        chunk_text: chunk.chunk_text.clone(),
        candidate_value: chunk.candidate_value,
        residue_admissible: chunk.residue_admissible,
        prime: chunk.prime,
    }
}

fn build_base_invariant_rows(
    baseline_rows: &[BaselineCodecRow],
    affine_rows: &[AffineCodecRow],
) -> Vec<BaseInvariantRow> {
    baseline_rows
        .iter()
        .map(|baseline| {
            let affine_residue = affine_rows
                .iter()
                .find(|row| row.sample_name == baseline.sample_name && row.mode == "residue")
                .expect("residue affine row should exist");
            let affine_prime = affine_rows
                .iter()
                .find(|row| row.sample_name == baseline.sample_name && row.mode == "prime")
                .expect("prime affine row should exist");
            let payload = parse_hex(&baseline.payload_hex).expect("baseline payload hex");
            BaseInvariantRow {
                sample_name: baseline.sample_name.clone(),
                payload_len: baseline.payload_len,
                leading_zero_bytes: payload.iter().take_while(|&&byte| byte == 0).count(),
                canonical_hex: baseline.payload_hex.clone(),
                decimal_value: num_bigint::BigUint::from_bytes_be(&payload).to_str_radix(10),
                base58: baseline.base58.clone(),
                base57: baseline.base57.clone(),
                affine_residue: affine_residue.notation.clone(),
                affine_prime: affine_prime.notation.clone(),
                base58_len: baseline.base58_len,
                base57_len: baseline.base57_len,
                affine_residue_len: affine_residue.notation_len,
                affine_prime_len: affine_prime.notation_len,
            }
        })
        .collect()
}

fn build_summary(
    baseline_rows: &[BaselineCodecRow],
    affine_rows: &[AffineCodecRow],
) -> ReportSummary {
    let residue_rows = affine_rows
        .iter()
        .filter(|row| row.mode == "residue")
        .collect::<Vec<_>>();
    let prime_rows = affine_rows
        .iter()
        .filter(|row| row.mode == "prime")
        .collect::<Vec<_>>();
    ReportSummary {
        sample_count: baseline_rows.len(),
        total_payload_bytes: baseline_rows.iter().map(|row| row.payload_len).sum(),
        base58_alphabet_len: BITCOIN_BASE58_ALPHABET.chars().count(),
        base57_alphabet_len: BASE57_ALPHABET.chars().count(),
        dropped_char: BASE57_DROPPED_CHAR,
        mean_base57_vs_base58_ratio: mean(
            baseline_rows
                .iter()
                .map(|row| row.base57_vs_base58_ratio)
                .collect::<Vec<_>>()
                .as_slice(),
        ),
        max_base57_minus_base58_chars: baseline_rows
            .iter()
            .map(|row| row.base57_minus_base58_chars)
            .max()
            .unwrap_or(0),
        residue_notation_mean_overhead_vs_base58: mean_isize(
            &residue_rows
                .iter()
                .map(|row| row.overhead_vs_base58_chars)
                .collect::<Vec<_>>(),
        ),
        prime_notation_mean_overhead_vs_base58: mean_isize(
            &prime_rows
                .iter()
                .map(|row| row.overhead_vs_base58_chars)
                .collect::<Vec<_>>(),
        ),
        residue_total_chunks: residue_rows.iter().map(|row| row.chunk_count).sum(),
        prime_total_chunks: prime_rows.iter().map(|row| row.chunk_count).sum(),
        residue_average_attempts_per_chunk: weighted_attempt_mean(&residue_rows),
        prime_average_attempts_per_chunk: weighted_attempt_mean(&prime_rows),
        affine_chunk_digits: AFFINE_CHUNK_DIGITS,
        affine_payload_bytes_per_chunk: AFFINE_PAYLOAD_BYTES_PER_CHUNK,
        affine_nonce_bits: AFFINE_NONCE_BITS,
        affine_nonce_space: AFFINE_NONCE_SPACE,
        residue_moduli_label: residue_moduli_for_affine_base()
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join("|"),
        strong_line: "base57 affine notation is a structured identifier envelope with fast residue validation and optional prime witnesses, not a shorter replacement for base58".to_string(),
        caution_line: "ordinary base58-to-base57 conversion remains reversible radix transcoding; the novel path starts when identifiers are generated inside the constrained grammar".to_string(),
    }
}

fn render_report(bundle: &ReportBundle) -> String {
    let mut lines = Vec::new();
    lines.push("# Base57 Affine Codec Report".to_string());
    lines.push(String::new());
    lines.push(bundle.summary.strong_line.clone());
    lines.push(bundle.summary.caution_line.clone());
    lines.push(String::new());
    lines.push("## Setup".to_string());
    lines.push(format!(
        "- Base58 alphabet length: `{}`",
        bundle.summary.base58_alphabet_len
    ));
    lines.push(format!(
        "- Base57 alphabet length: `{}`; dropped character: `{}`",
        bundle.summary.base57_alphabet_len, bundle.summary.dropped_char
    ));
    lines.push(format!(
        "- Affine lane: base `57`, outer `{AFFINE_OUTER}`, inner `{AFFINE_INNER}`, `k=(0,0)`, `M={AFFINE_MIDDLE_LENGTH}`"
    ));
    lines.push(format!(
        "- Chunk payload: `{AFFINE_PAYLOAD_BYTES_PER_CHUNK}` bytes plus `{AFFINE_NONCE_BITS}` nonce bits, rendered as `{AFFINE_CHUNK_DIGITS}` base57 symbols"
    ));
    lines.push(String::new());
    lines.push("## Headline".to_string());
    lines.push(format!(
        "- Samples: `{}` payloads, `{}` total bytes",
        bundle.summary.sample_count, bundle.summary.total_payload_bytes
    ));
    lines.push(format!(
        "- Mean base57/base58 length ratio: `{:.4}`",
        bundle.summary.mean_base57_vs_base58_ratio
    ));
    lines.push(format!(
        "- Worst base57 baseline expansion over base58: `{:+}` characters",
        bundle.summary.max_base57_minus_base58_chars
    ));
    lines.push(format!(
        "- Residue notation mean overhead vs base58: `{:+.1}` characters",
        bundle.summary.residue_notation_mean_overhead_vs_base58
    ));
    lines.push(format!(
        "- Prime notation mean overhead vs base58: `{:+.1}` characters",
        bundle.summary.prime_notation_mean_overhead_vs_base58
    ));
    lines.push(format!(
        "- Average nonce attempts: residue `{:.2}`, prime `{:.2}`",
        bundle.summary.residue_average_attempts_per_chunk,
        bundle.summary.prime_average_attempts_per_chunk
    ));
    lines.push(String::new());
    lines.push("## Baseline Examples".to_string());
    lines
        .push("| sample | bytes | base58 len | base57 len | delta | base58 | base57 |".to_string());
    lines.push("|---|---:|---:|---:|---:|---|---|".to_string());
    for row in bundle.baseline_codec_rows.iter().take(12) {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` | `{:+}` | `{}` | `{}` |",
            row.sample_name,
            row.payload_len,
            row.base58_len,
            row.base57_len,
            row.base57_minus_base58_chars,
            compact(&row.base58, 28),
            compact(&row.base57, 28)
        ));
    }
    lines.push(String::new());
    lines.push("## Base-Invariant Value Map".to_string());
    lines.push(
        "| sample | bytes | hex/base16 | base58 | base57 | residue envelope | prime envelope |"
            .to_string(),
    );
    lines.push("|---|---:|---|---|---|---|---|".to_string());
    for row in bundle.base_invariant_rows.iter().filter(|row| {
        row.sample_name == "hello"
            || row.sample_name == "leading_zero_bytes"
            || row.sample_name == "deterministic_random_16"
    }) {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |",
            row.sample_name,
            row.payload_len,
            compact(&row.canonical_hex, 24),
            compact(&row.base58, 22),
            compact(&row.base57, 22),
            compact(&row.affine_residue, 30),
            compact(&row.affine_prime, 30)
        ));
    }
    lines.push(String::new());
    lines.push("The invariant object is the payload bytes. Hex/base16, base58, and base57 are ordinary renderings of that same value. The affine forms are structured representatives: they preserve the payload while adding fixed chunk grammar and arithmetic validation.".to_string());
    lines.push(String::new());
    lines.push("## Affine Examples".to_string());
    lines.push(
        "| sample | mode | chunks | notation len | overhead vs base58 | avg attempts | notation |"
            .to_string(),
    );
    lines.push("|---|---|---:|---:|---:|---:|---|".to_string());
    for row in bundle
        .affine_codec_rows
        .iter()
        .filter(|row| row.sample_name == "hello" || row.sample_name == "leading_zero_bytes")
    {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` | `{:+}` | `{:.2}` | `{}` |",
            row.sample_name,
            row.mode,
            row.chunk_count,
            row.notation_len,
            row.overhead_vs_base58_chars,
            row.average_attempts_per_chunk,
            compact(&row.notation, 44)
        ));
    }
    lines.push(String::new());
    lines.push("## Claim Boundary".to_string());
    lines.push("The baseline codec is ordinary reversible radix conversion. The affine notation is intentionally longer because it adds structure: fixed chunk grammar, residue validation, and optional prime witnesses. Any efficiency claim should be about validation/filtering and structured generation, not character count.".to_string());
    lines.push(String::new());
    lines.push("## Visuals".to_string());
    for image in &bundle.image_artifact_rows {
        lines.push(format!("- `{}`: `{}`", image.label, image.path));
    }
    lines.push(String::new());
    lines.join("\n")
}

fn render_codec_comparison_panel(
    baseline_rows: &[BaselineCodecRow],
    affine_rows: &[AffineCodecRow],
    path: &Path,
) {
    let deterministic = baseline_rows
        .iter()
        .filter(|row| {
            row.sample_name == "empty" || row.sample_name.starts_with("deterministic_random_")
        })
        .collect::<Vec<_>>();
    let mut deterministic = deterministic;
    deterministic.sort_by(|left, right| {
        left.payload_len
            .cmp(&right.payload_len)
            .then_with(|| left.sample_name.cmp(&right.sample_name))
    });
    let root = BitMapBackend::new(path, (1500, 900)).into_drawing_area();
    root.fill(&WHITE).expect("fill codec panel");
    let (top, bottom) = root.split_vertically(390);

    let extra_count = deterministic
        .iter()
        .filter(|row| row.base57_minus_base58_chars > 0)
        .count();
    let mut top_chart = ChartBuilder::on(&top)
        .caption(
            "Base57 Baseline Cost: Extra Characters Are Sparse",
            ("sans-serif", 30).into_font(),
        )
        .margin(32)
        .x_label_area_size(54)
        .y_label_area_size(70)
        .build_cartesian_2d(0i32..65i32, 0.0f64..1.35f64)
        .expect("build delta chart");
    top_chart
        .configure_mesh()
        .x_desc("payload length in bytes")
        .y_desc("base57 len - base58 len")
        .y_labels(2)
        .y_label_formatter(&|value| {
            if (*value - 0.0).abs() < 0.05 {
                "0".to_string()
            } else if (*value - 1.0).abs() < 0.05 {
                "+1".to_string()
            } else {
                String::new()
            }
        })
        .draw()
        .expect("draw delta mesh");
    top_chart
        .draw_series(deterministic.iter().map(|row| {
            let x = row.payload_len as i32;
            let height = if row.base57_minus_base58_chars > 0 {
                row.base57_minus_base58_chars as f64
            } else {
                0.08
            };
            let color = if row.base57_minus_base58_chars > 0 {
                RGBColor(205, 95, 70)
            } else {
                RGBColor(190, 197, 204)
            };
            Rectangle::new([(x, 0.0), (x + 1, height)], color.filled())
        }))
        .expect("draw delta bars");
    top.draw(&Text::new(
        format!(
            "{} of {} deterministic payload lengths cost one extra character; none cost more than one in this sample",
            extra_count,
            deterministic.len()
        ),
        (82, 92),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw delta note");

    let selected_names = [
        ("empty", "empty"),
        ("hello", "hello"),
        ("leading_zero_bytes", "zeros"),
        ("deterministic_random_16", "16B"),
        ("deterministic_random_32", "32B"),
        ("deterministic_random_64", "64B"),
    ];
    let groups = selected_names
        .iter()
        .filter_map(|(name, label)| {
            let baseline = baseline_rows.iter().find(|row| row.sample_name == *name)?;
            let affine = affine_rows
                .iter()
                .find(|row| row.sample_name == *name && row.mode == "residue")?;
            Some((*label, baseline, affine))
        })
        .collect::<Vec<_>>();
    let max_value = groups
        .iter()
        .flat_map(|(_, baseline, affine)| {
            [
                baseline.base58_len,
                baseline.base57_len,
                affine.notation_len,
            ]
        })
        .max()
        .unwrap_or(1) as i32;
    let group_stride = 5i32;
    let x_max = groups.len() as i32 * group_stride;
    let mut bottom_chart = ChartBuilder::on(&bottom)
        .caption(
            "Ordinary Codec Text vs Structured Affine Envelope",
            ("sans-serif", 28).into_font(),
        )
        .margin(32)
        .x_label_area_size(66)
        .y_label_area_size(74)
        .build_cartesian_2d(0i32..x_max, 0i32..(max_value + 18))
        .expect("build envelope chart");
    bottom_chart
        .configure_mesh()
        .disable_x_mesh()
        .x_desc("payload sample")
        .y_desc("characters")
        .x_labels((groups.len() as i32 * group_stride) as usize)
        .x_label_formatter(&|value| {
            groups
                .iter()
                .enumerate()
                .find_map(|(idx, (label, _, _))| {
                    let center = idx as i32 * group_stride + 1;
                    if *value == center {
                        Some((*label).to_string())
                    } else {
                        None
                    }
                })
                .unwrap_or_default()
        })
        .draw()
        .expect("draw envelope mesh");
    let series = [
        ("base58", RGBColor(42, 115, 185)),
        ("base57", RGBColor(205, 95, 70)),
        ("a57r1", RGBColor(62, 151, 116)),
    ];
    for (series_index, (label, color)) in series.into_iter().enumerate() {
        bottom_chart
            .draw_series(groups.iter().enumerate().map(|(group_index, group)| {
                let (_, baseline, affine) = group;
                let value = match label {
                    "base58" => baseline.base58_len,
                    "base57" => baseline.base57_len,
                    _ => affine.notation_len,
                } as i32;
                let x0 = group_index as i32 * group_stride + series_index as i32;
                Rectangle::new([(x0, 0), (x0 + 1, value)], color.filled())
            }))
            .expect("draw envelope bars")
            .label(label)
            .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 18, y + 5)], color.filled()));
    }
    bottom_chart
        .configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.85))
        .draw()
        .expect("draw envelope legend");
    bottom.draw(&Text::new(
        "The affine ID is longer on purpose: it carries chunk grammar, payload length, and residue/prime validation.",
        (82, 86),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw envelope note");
    root.present().expect("present codec panel");
}

fn render_affine_chunk_funnel(rows: &[AffineCodecRow], path: &Path) {
    let residue = rows
        .iter()
        .filter(|row| row.mode == "residue")
        .collect::<Vec<_>>();
    let prime = rows
        .iter()
        .filter(|row| row.mode == "prime")
        .collect::<Vec<_>>();
    let total_chunks = [
        residue.iter().map(|row| row.chunk_count).sum::<usize>(),
        prime.iter().map(|row| row.chunk_count).sum::<usize>(),
    ];
    let total_attempts = [
        residue.iter().map(|row| row.total_attempts).sum::<u64>(),
        prime.iter().map(|row| row.total_attempts).sum::<u64>(),
    ];
    let max_value = total_attempts
        .iter()
        .copied()
        .max()
        .unwrap_or(1)
        .max(total_chunks.iter().copied().max().unwrap_or(1) as u64);
    let root = BitMapBackend::new(path, (1200, 760)).into_drawing_area();
    root.fill(&WHITE).expect("fill funnel");
    root.draw(&Text::new(
        "Affine Chunk Funnel",
        (42, 48),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw title");
    root.draw(&Text::new(
        "chunks are fixed; attempts are nonce probes before acceptance",
        (42, 86),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw subtitle");
    for (idx, label) in ["residue", "prime"].iter().enumerate() {
        let y = 170 + idx as i32 * 230;
        let chunk_w = (700.0 * total_chunks[idx] as f64 / max_value as f64) as i32;
        let attempt_w = (700.0 * total_attempts[idx] as f64 / max_value as f64) as i32;
        root.draw(&Text::new(
            *label,
            (55, y + 32),
            ("sans-serif", 22).into_font().style(FontStyle::Bold),
        ))
        .expect("draw label");
        root.draw(&Rectangle::new(
            [(210, y), (210 + chunk_w, y + 42)],
            RGBColor(66, 132, 178).filled(),
        ))
        .expect("draw chunks");
        root.draw(&Rectangle::new(
            [(210, y + 72), (210 + attempt_w, y + 114)],
            RGBColor(205, 95, 70).filled(),
        ))
        .expect("draw attempts");
        root.draw(&Text::new(
            format!("chunks: {}", total_chunks[idx]),
            (930, y + 30),
            ("sans-serif", 18).into_font(),
        ))
        .expect("draw chunks text");
        root.draw(&Text::new(
            format!("attempts: {}", total_attempts[idx]),
            (930, y + 102),
            ("sans-serif", 18).into_font(),
        ))
        .expect("draw attempts text");
    }
    root.present().expect("present funnel");
}

fn render_notation_anatomy_strip(rows: &[AffineCodecRow], path: &Path) {
    let example = rows
        .iter()
        .find(|row| row.sample_name == "hello" && row.mode == "prime")
        .or_else(|| rows.first())
        .expect("at least one affine row");
    let root = BitMapBackend::new(path, (1500, 620)).into_drawing_area();
    root.fill(&WHITE).expect("fill anatomy");
    root.draw(&Text::new(
        "Base57 Affine Notation Anatomy",
        (42, 48),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw title");
    let parts = [
        ("prefix", "a57p1 / a57r1", RGBColor(66, 132, 178)),
        (
            "payload length",
            "decimal byte count",
            RGBColor(62, 151, 116),
        ),
        (
            "body",
            "fixed 10-symbol membrane chunks",
            RGBColor(205, 95, 70),
        ),
    ];
    for (idx, (title, value, color)) in parts.iter().enumerate() {
        let x = 70 + idx as i32 * 455;
        root.draw(&Rectangle::new(
            [(x, 130), (x + 390, 255)],
            color.mix(0.18).filled(),
        ))
        .expect("draw box");
        root.draw(&Rectangle::new(
            [(x, 130), (x + 390, 255)],
            ShapeStyle::from(color).stroke_width(2),
        ))
        .expect("draw box outline");
        root.draw(&Text::new(
            *title,
            (x + 18, 170),
            ("sans-serif", 22).into_font().style(FontStyle::Bold),
        ))
        .expect("draw anatomy title");
        root.draw(&Text::new(
            *value,
            (x + 18, 215),
            ("sans-serif", 18).into_font(),
        ))
        .expect("draw anatomy value");
    }
    root.draw(&Text::new(
        format!("example: {}", compact(&example.notation, 110)),
        (70, 350),
        ("sans-serif", 20).into_font(),
    ))
    .expect("draw example");
    root.draw(&Text::new(
        format!(
            "chunk grammar: outer {AFFINE_OUTER}, inner {AFFINE_INNER}, M={AFFINE_MIDDLE_LENGTH}, {} payload bytes + {} nonce bits",
            AFFINE_PAYLOAD_BYTES_PER_CHUNK, AFFINE_NONCE_BITS
        ),
        (70, 410),
        ("sans-serif", 19).into_font(),
    ))
    .expect("draw grammar");
    root.present().expect("present anatomy");
}

fn mode_label(mode: AffineCodecMode) -> &'static str {
    match mode {
        AffineCodecMode::Residue => "residue",
        AffineCodecMode::Prime => "prime",
    }
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        if numerator == 0 {
            1.0
        } else {
            f64::INFINITY
        }
    } else {
        numerator as f64 / denominator as f64
    }
}

fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<f64>() / values.len() as f64
    }
}

fn mean_isize(values: &[isize]) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().map(|&value| value as f64).sum::<f64>() / values.len() as f64
    }
}

fn weighted_attempt_mean(rows: &[&AffineCodecRow]) -> f64 {
    let chunks = rows.iter().map(|row| row.chunk_count).sum::<usize>();
    let attempts = rows.iter().map(|row| row.total_attempts).sum::<u64>();
    ratio(attempts as usize, chunks)
}

fn compact(value: &str, limit: usize) -> String {
    if value.len() <= limit {
        value.to_string()
    } else {
        format!("{}...", &value[..limit.saturating_sub(3)])
    }
}
