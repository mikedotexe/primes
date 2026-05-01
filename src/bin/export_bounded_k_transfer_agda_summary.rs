//! Export the maintained bounded-`k` witness catalog as an Agda summary module.
//!
//! The emitted file targets:
//! `agda-proofs/Examples/Generated/BoundedKTransferWitnessCatalog.agda`

use clap::Parser;
use primes::validation::bounded_k::{
    analyze_best_vs_k00_decomposition, digit_symbol, evaluate_pair_row, parse_k_label,
    scan_k_config_transfer_profile, DEFAULT_BOUNDED_K_GRID,
};
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Export the maintained bounded-k witness catalog as an Agda summary module"
)]
struct Args {
    /// Output path under agda-proofs/Examples/Generated/
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy)]
struct WitnessSpec {
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
}

const MAINTAINED_WITNESSES: &[WitnessSpec] = &[
    WitnessSpec {
        base: 14,
        middle_length: 2,
        outer: 13,
        inner: 11,
    },
    WitnessSpec {
        base: 10,
        middle_length: 2,
        outer: 3,
        inner: 3,
    },
    WitnessSpec {
        base: 34,
        middle_length: 2,
        outer: 25,
        inner: 9,
    },
    WitnessSpec {
        base: 22,
        middle_length: 2,
        outer: 17,
        inner: 19,
    },
];

#[derive(Debug, Clone)]
struct TransferStats {
    stable_zero_count: usize,
    gain_zero_count: usize,
    loss_zero_count: usize,
    stable_nonzero_count: usize,
    nonzero_churn_count: usize,
    stable_zero_prime_delta_count: isize,
    boundary_prime_delta_count: isize,
}

#[derive(Debug, Clone)]
struct WitnessRow {
    spec: WitnessSpec,
    binding_name: String,
    best_k: (u32, u32),
    best_k_label: String,
    stats: TransferStats,
    overlap_led: bool,
    boundary_led: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let output_path = resolve_output_path(&args)?;

    let mut rows = Vec::with_capacity(MAINTAINED_WITNESSES.len());
    for &spec in MAINTAINED_WITNESSES {
        let pair_row = evaluate_pair_row(
            spec.base,
            spec.middle_length,
            spec.outer,
            spec.inner,
            DEFAULT_BOUNDED_K_GRID,
        );
        let best_k = parse_k_label(&pair_row.best_k);
        let transfer_profile = scan_k_config_transfer_profile(
            spec.base,
            spec.middle_length,
            spec.outer,
            spec.inner,
            (0, 0),
            best_k,
        );
        let decomposition = analyze_best_vs_k00_decomposition(
            spec.base,
            spec.middle_length,
            spec.outer,
            spec.inner,
            DEFAULT_BOUNDED_K_GRID,
        );
        let stats = derive_transfer_stats(&transfer_profile);

        rows.push(WitnessRow {
            spec,
            binding_name: agda_binding_name(spec),
            best_k,
            best_k_label: pair_row.best_k,
            stats,
            overlap_led: decomposition.signal_source_label == "stable_zero_led",
            boundary_led: decomposition.signal_source_label == "boundary_led",
        });
    }

    let module_text = render_agda_catalog(&rows)?;
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output_path, module_text)?;

    println!("Wrote Agda witness catalog: {}", output_path.display());
    println!("Cases: {}", rows.len());
    for row in &rows {
        println!(
            "  - {}: base {} pair ({}, {}) M={} best {}",
            row.binding_name,
            row.spec.base,
            digit_symbol(row.spec.outer),
            digit_symbol(row.spec.inner),
            row.spec.middle_length,
            row.best_k_label
        );
    }

    Ok(())
}

fn resolve_output_path(args: &Args) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let default_output = PathBuf::from("agda-proofs")
        .join("Examples")
        .join("Generated")
        .join("BoundedKTransferWitnessCatalog.agda");
    let output = args.out.clone().unwrap_or(default_output);
    let absolute = if output.is_absolute() {
        output
    } else {
        repo_root.join(output)
    };

    let generated_root = repo_root
        .join("agda-proofs")
        .join("Examples")
        .join("Generated");
    if !absolute.starts_with(&generated_root) {
        return Err(format!("output path must live under {}", generated_root.display()).into());
    }
    if absolute.extension().and_then(|ext| ext.to_str()) != Some("agda") {
        return Err("output path must end with .agda".into());
    }
    Ok(absolute)
}

fn derive_transfer_stats(
    transfer_profile: &primes::validation::bounded_k::KConfigTransferProfile,
) -> TransferStats {
    let bucket_count = |bucket: &str| {
        transfer_profile
            .candidate_rows
            .iter()
            .filter(|row| row.transfer_bucket == bucket)
            .count()
    };
    let bucket_prime_delta = |bucket: &str| {
        transfer_profile
            .transfer_histogram_rows
            .iter()
            .filter(|row| row.transfer_bucket == bucket)
            .map(|row| row.prime_delta_count)
            .sum::<isize>()
    };

    TransferStats {
        stable_zero_count: bucket_count("stable_zero"),
        gain_zero_count: bucket_count("gain_zero"),
        loss_zero_count: bucket_count("loss_zero"),
        stable_nonzero_count: bucket_count("stable_nonzero"),
        nonzero_churn_count: bucket_count("nonzero_churn"),
        stable_zero_prime_delta_count: bucket_prime_delta("stable_zero"),
        boundary_prime_delta_count: bucket_prime_delta("gain_zero")
            + bucket_prime_delta("loss_zero"),
    }
}

fn agda_binding_name(spec: WitnessSpec) -> String {
    format!(
        "base{}-{}{}",
        spec.base,
        digit_symbol(spec.outer),
        digit_symbol(spec.inner)
    )
}

fn agda_theorem_stem(spec: WitnessSpec) -> String {
    format!(
        "base{}-{}{}",
        spec.base,
        digit_symbol(spec.outer),
        digit_symbol(spec.inner)
    )
}

fn render_signed_delta(delta: isize) -> String {
    match delta.cmp(&0) {
        std::cmp::Ordering::Less => format!("negativeΔ {}", delta.unsigned_abs()),
        std::cmp::Ordering::Equal => "zeroΔ".to_string(),
        std::cmp::Ordering::Greater => format!("positiveΔ {}", delta),
    }
}

fn render_agda_catalog(rows: &[WitnessRow]) -> Result<String, Box<dyn std::error::Error>> {
    let mut out = String::new();
    writeln!(
        out,
        "{{-# OPTIONS --safe --without-K #-}}\n------------------------------------------------------------------------"
    )?;
    writeln!(out, "-- Auto-generated bounded-k transfer witness catalog")?;
    writeln!(out, "--")?;
    writeln!(
        out,
        "-- Source of truth: `cargo run --bin export_bounded_k_transfer_agda_summary`"
    )?;
    writeln!(
        out,
        "------------------------------------------------------------------------\n"
    )?;
    writeln!(
        out,
        "module Examples.Generated.BoundedKTransferWitnessCatalog where\n"
    )?;
    writeln!(out, "open import Data.Bool using (true; false)")?;
    writeln!(
        out,
        "open import Relation.Binary.PropositionalEquality using (_≡_; refl)\n"
    )?;
    writeln!(
        out,
        "open import Theorems.BoundedKCompactness using (compact₀; kConfig; paddingWeight; diameter)"
    )?;
    writeln!(
        out,
        "open import Examples.BoundedKTransferWitnessShell using"
    )?;
    writeln!(out, "  ( TransferWitnessSummary")?;
    writeln!(out, "  ; zeroΔ")?;
    writeln!(out, "  ; positiveΔ")?;
    writeln!(out, "  ; negativeΔ")?;
    writeln!(out, "  ; overlapLed?")?;
    writeln!(out, "  ; boundaryLed?")?;
    writeln!(out, "  ; fromConfig")?;
    writeln!(out, "  ; toConfig")?;
    writeln!(out, "  )\n")?;

    for row in rows {
        let spec = row.spec;
        let stem = agda_theorem_stem(spec);
        let from_diameter = spec.middle_length + 4;
        let to_diameter = spec.middle_length + 4 + 2 * (row.best_k.0 + row.best_k.1) as usize;
        let padding_step = row.best_k.0 + row.best_k.1;

        writeln!(
            out,
            "-- Base {}, pair ({}, {}) = ({},{}), M = {}, best {}",
            spec.base,
            digit_symbol(spec.outer),
            digit_symbol(spec.inner),
            spec.outer,
            spec.inner,
            spec.middle_length,
            row.best_k_label
        )?;
        writeln!(out, "{} : TransferWitnessSummary", row.binding_name)?;
        writeln!(out, "{} = record", row.binding_name)?;
        writeln!(out, "  {{ base = {}", spec.base)?;
        writeln!(out, "  ; middleLength = {}", spec.middle_length)?;
        writeln!(out, "  ; outer = {}", spec.outer)?;
        writeln!(out, "  ; inner = {}", spec.inner)?;
        writeln!(out, "  ; fromConfig = compact₀")?;
        writeln!(
            out,
            "  ; toConfig = kConfig {} {}",
            row.best_k.0, row.best_k.1
        )?;
        writeln!(out, "  ; stableZeroCount = {}", row.stats.stable_zero_count)?;
        writeln!(out, "  ; gainZeroCount = {}", row.stats.gain_zero_count)?;
        writeln!(out, "  ; lossZeroCount = {}", row.stats.loss_zero_count)?;
        writeln!(
            out,
            "  ; stableNonzeroCount = {}",
            row.stats.stable_nonzero_count
        )?;
        writeln!(
            out,
            "  ; nonzeroChurnCount = {}",
            row.stats.nonzero_churn_count
        )?;
        writeln!(
            out,
            "  ; stableZeroPrimeDelta = {}",
            render_signed_delta(row.stats.stable_zero_prime_delta_count)
        )?;
        writeln!(
            out,
            "  ; boundaryPrimeDelta = {}",
            render_signed_delta(row.stats.boundary_prime_delta_count)
        )?;
        writeln!(out, "  }}\n")?;

        if row.overlap_led {
            writeln!(
                out,
                "{}-overlap-led : overlapLed? {} ≡ true",
                stem, row.binding_name
            )?;
            writeln!(out, "{}-overlap-led = refl", stem)?;
        } else {
            writeln!(
                out,
                "{}-overlap-not-led : overlapLed? {} ≡ false",
                stem, row.binding_name
            )?;
            writeln!(out, "{}-overlap-not-led = refl", stem)?;
        }
        if row.boundary_led {
            writeln!(
                out,
                "{}-boundary-led : boundaryLed? {} ≡ true",
                stem, row.binding_name
            )?;
            writeln!(out, "{}-boundary-led = refl", stem)?;
        } else {
            writeln!(
                out,
                "{}-boundary-not-led : boundaryLed? {} ≡ false",
                stem, row.binding_name
            )?;
            writeln!(out, "{}-boundary-not-led = refl", stem)?;
        }
        writeln!(
            out,
            "{}-diameter-from : diameter {} (fromConfig {}) ≡ {}",
            stem, spec.middle_length, row.binding_name, from_diameter
        )?;
        writeln!(out, "{}-diameter-from = refl", stem)?;
        writeln!(
            out,
            "{}-diameter-to : diameter {} (toConfig {}) ≡ {}",
            stem, spec.middle_length, row.binding_name, to_diameter
        )?;
        writeln!(out, "{}-diameter-to = refl", stem)?;
        writeln!(
            out,
            "{}-padding-step : paddingWeight (toConfig {}) ≡ {}",
            stem, row.binding_name, padding_step
        )?;
        writeln!(out, "{}-padding-step = refl\n", stem)?;
    }

    Ok(out)
}
