//! Export a bounded-`k` transfer witness as a Lean artifact.
//!
//! The emitted file targets the maintained generated-data transfer shell:
//! `GeneratedTransferPayload` in
//! `PrimeArithmetic.Generated.BoundedKTransferShell`.

use clap::Parser;
use primes::validation::bounded_k::{
    analyze_best_vs_k00_decomposition, digit_symbol, evaluate_pair_row, parse_k_label,
    scan_k_config_mask_profile, scan_k_config_transfer_profile, DEFAULT_BOUNDED_K_GRID,
    DEFAULT_PREFILTER_PRIMES,
};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Export a bounded-k transfer witness as a Lean artifact"
)]
struct Args {
    /// Base for the symmetric template lane
    #[arg(long)]
    base: u32,

    /// Middle block width M
    #[arg(long = "middle-length")]
    middle_length: usize,

    /// Outer boundary digit
    #[arg(long)]
    outer: u32,

    /// Inner boundary digit
    #[arg(long)]
    inner: u32,

    /// Output path under lean-proofs/PrimeArithmetic/Generated/
    #[arg(long)]
    out: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    validate_args(&args)?;

    let pair_row = evaluate_pair_row(
        args.base,
        args.middle_length,
        args.outer,
        args.inner,
        DEFAULT_BOUNDED_K_GRID,
    );
    let best_k = parse_k_label(&pair_row.best_k);
    let from_k = (0, 0);
    let k00_profile = scan_k_config_mask_profile(
        args.base,
        args.middle_length,
        args.outer,
        args.inner,
        from_k,
    );
    let best_profile = scan_k_config_mask_profile(
        args.base,
        args.middle_length,
        args.outer,
        args.inner,
        best_k,
    );
    let transfer_profile = scan_k_config_transfer_profile(
        args.base,
        args.middle_length,
        args.outer,
        args.inner,
        from_k,
        best_k,
    );
    let decomposition = analyze_best_vs_k00_decomposition(
        args.base,
        args.middle_length,
        args.outer,
        args.inner,
        DEFAULT_BOUNDED_K_GRID,
    );

    let stats = derive_transfer_stats(&transfer_profile);
    let output_path = resolve_output_path(&args)?;
    let module_name = lean_module_name(&output_path)?;
    let artifact = render_artifact(RenderContext {
        args: &args,
        module_name: &module_name,
        best_k_label: &pair_row.best_k,
        best_k,
        k00_profile: &k00_profile,
        best_profile: &best_profile,
        stats: &stats,
        transfer_profile: &transfer_profile,
        signal_source_label: &decomposition.signal_source_label,
    })?;

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output_path, artifact)?;

    println!("Wrote Lean artifact: {}", display_path(&output_path));
    println!("Module: {module_name}");
    println!(
        "Pair: ({}, {}) at base {} M={}",
        digit_symbol(args.outer),
        digit_symbol(args.inner),
        args.base,
        args.middle_length
    );
    println!("Best k: {}", pair_row.best_k);
    println!(
        "Counts: stable_zero={}, gain_zero={}, loss_zero={}, stable_nonzero={}, nonzero_churn={}",
        stats.stable_zero_count,
        stats.gain_zero_count,
        stats.loss_zero_count,
        stats.stable_nonzero_count,
        stats.nonzero_churn_count
    );
    println!(
        "Prime deltas: stable_zero={}, boundary={}",
        stats.stable_zero_prime_delta_count, stats.boundary_prime_delta_count
    );

    Ok(())
}

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

struct RenderContext<'a> {
    args: &'a Args,
    module_name: &'a str,
    best_k_label: &'a str,
    best_k: (u32, u32),
    k00_profile: &'a primes::validation::bounded_k::KConfigMaskProfile,
    best_profile: &'a primes::validation::bounded_k::KConfigMaskProfile,
    stats: &'a TransferStats,
    transfer_profile: &'a primes::validation::bounded_k::KConfigTransferProfile,
    signal_source_label: &'a str,
}

fn validate_args(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    if args.base < 2 {
        return Err("--base must be at least 2".into());
    }
    if args.outer >= args.base || args.inner >= args.base {
        return Err("--outer and --inner must be valid digits in the selected base".into());
    }
    Ok(())
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

fn resolve_output_path(args: &Args) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let default_name = format!(
        "Base{}M{}Pair{}{}.lean",
        args.base,
        args.middle_length,
        digit_symbol(args.outer),
        digit_symbol(args.inner)
    );
    let output = args.out.clone().unwrap_or_else(|| {
        PathBuf::from("lean-proofs")
            .join("PrimeArithmetic")
            .join("Generated")
            .join("BoundedK")
            .join(default_name)
    });
    let absolute = if output.is_absolute() {
        output
    } else {
        repo_root.join(output)
    };

    let lean_root = repo_root.join("lean-proofs");
    let generated_root = lean_root.join("PrimeArithmetic").join("Generated");
    if !absolute.starts_with(&generated_root) {
        return Err(format!("output path must live under {}", generated_root.display()).into());
    }
    if absolute.extension().and_then(|ext| ext.to_str()) != Some("lean") {
        return Err("output path must end with .lean".into());
    }
    Ok(absolute)
}

fn lean_module_name(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let lean_root = repo_root.join("lean-proofs");
    let relative = path
        .strip_prefix(&lean_root)
        .map_err(|_| "output path is not inside lean-proofs")?;
    let without_ext = relative.with_extension("");
    let components: Vec<String> = without_ext
        .components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect();
    if components.is_empty() {
        return Err("could not derive a Lean module name from the output path".into());
    }
    Ok(components.join("."))
}

fn render_artifact(ctx: RenderContext<'_>) -> Result<String, Box<dyn std::error::Error>> {
    let RenderContext {
        args,
        module_name,
        best_k_label,
        best_k,
        k00_profile,
        best_profile,
        stats,
        transfer_profile,
        signal_source_label,
    } = ctx;
    let mut out = String::new();
    let command = format!(
        "cargo run --bin export_bounded_k_transfer_witness -- --base {} --middle-length {} --outer {} --inner {} --out {}",
        args.base,
        args.middle_length,
        args.outer,
        args.inner,
        default_display_output(args)?
    );

    writeln!(
        out,
        "import PrimeArithmetic.Generated.BoundedKTransferShell"
    )?;
    writeln!(out)?;
    writeln!(out, "namespace {module_name}")?;
    writeln!(out)?;
    writeln!(out, "open PrimeArithmetic.Generated.BoundedKTransferShell")?;
    writeln!(out)?;
    writeln!(out, "/-!")?;
    writeln!(out, "Runtime-exported bounded-k transfer witness artifact.")?;
    writeln!(out)?;
    writeln!(out, "Generated by:")?;
    writeln!(out, "`{command}`")?;
    writeln!(out)?;
    writeln!(
        out,
        "- pair: `({}, {})` in base `{}` at `M={}`",
        digit_symbol(args.outer),
        digit_symbol(args.inner),
        args.base,
        args.middle_length
    )?;
    writeln!(out, "- baseline lane: `k=(0,0)`")?;
    writeln!(out, "- best lane: `{best_k_label}`")?;
    writeln!(out, "- report-layer source label: `{signal_source_label}`")?;
    writeln!(
        out,
        "- stable-zero prime delta count: `{}`",
        stats.stable_zero_prime_delta_count
    )?;
    writeln!(
        out,
        "- boundary prime delta count: `{}`",
        stats.boundary_prime_delta_count
    )?;
    writeln!(out, "-/")?;
    writeln!(out)?;
    writeln!(out, "def payload : GeneratedTransferPayload where")?;
    writeln!(out, "  base := {}", args.base)?;
    writeln!(out, "  middleLength := {}", args.middle_length)?;
    writeln!(out, "  outer := {}", args.outer)?;
    writeln!(out, "  inner := {}", args.inner)?;
    writeln!(out, "  fromKOuter := 0")?;
    writeln!(out, "  fromKInner := 0")?;
    writeln!(out, "  toKOuter := {}", best_k.0)?;
    writeln!(out, "  toKInner := {}", best_k.1)?;
    writeln!(
        out,
        "  prefilterPrimes := [{}]",
        DEFAULT_PREFILTER_PRIMES
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    )?;
    writeln!(out, "  rows := [")?;
    for row in &transfer_profile.candidate_rows {
        writeln!(
            out,
            "    {{ middleIndex := {}, maskFrom := {}, maskTo := {}, primeFrom := {}, primeTo := {} }},",
            row.middle_index,
            row.divisibility_mask_from,
            row.divisibility_mask_to,
            bool_lit(row.prime_from),
            bool_lit(row.prime_to)
        )?;
    }
    writeln!(out, "  ]")?;
    writeln!(out)?;

    render_value_theorem(&mut out, "stableZeroCount", stats.stable_zero_count)?;
    render_value_theorem(&mut out, "gainZeroCount", stats.gain_zero_count)?;
    render_value_theorem(&mut out, "lossZeroCount", stats.loss_zero_count)?;
    render_value_theorem(&mut out, "stableNonzeroCount", stats.stable_nonzero_count)?;
    render_value_theorem(&mut out, "nonzeroChurnCount", stats.nonzero_churn_count)?;
    render_value_theorem(&mut out, "sharedAdmissibleCount", stats.stable_zero_count)?;
    render_value_theorem(
        &mut out,
        "admissibleCountFrom",
        k00_profile.admissible_count,
    )?;
    render_value_theorem(&mut out, "admissibleCountTo", best_profile.admissible_count)?;
    render_int_value_theorem(
        &mut out,
        "admissibleDeltaCount",
        best_profile.admissible_count as isize - k00_profile.admissible_count as isize,
    )?;
    render_int_value_theorem(
        &mut out,
        "stableZeroPrimeDeltaCount",
        stats.stable_zero_prime_delta_count,
    )?;
    render_int_value_theorem(
        &mut out,
        "boundaryPrimeDeltaCount",
        stats.boundary_prime_delta_count,
    )?;

    if stats.stable_zero_prime_delta_count > 0 {
        render_native_decide_theorem(
            &mut out,
            "stableZeroPrimeDeltaCount_pos",
            "payload.stableZeroPrimeDeltaCount > 0",
        )?;
    }
    if stats.stable_zero_prime_delta_count <= 0 {
        render_native_decide_theorem(
            &mut out,
            "stableZeroPrimeDeltaCount_nonpos",
            "payload.stableZeroPrimeDeltaCount ≤ 0",
        )?;
    }
    if stats.boundary_prime_delta_count > 0 {
        render_native_decide_theorem(
            &mut out,
            "boundaryPrimeDeltaCount_pos",
            "payload.boundaryPrimeDeltaCount > 0",
        )?;
    }
    if stats.stable_zero_prime_delta_count
        > stats.boundary_prime_delta_count.unsigned_abs() as isize
    {
        render_native_decide_theorem(
            &mut out,
            "stableZeroPrimeDeltaCount_gt_natAbs_boundaryPrimeDeltaCount",
            "payload.stableZeroPrimeDeltaCount > Int.natAbs payload.boundaryPrimeDeltaCount",
        )?;
    }
    if stats.boundary_prime_delta_count.unsigned_abs()
        > stats.stable_zero_prime_delta_count.unsigned_abs()
    {
        render_native_decide_theorem(
            &mut out,
            "natAbs_boundaryPrimeDeltaCount_gt_natAbs_stableZeroPrimeDeltaCount",
            "Int.natAbs payload.boundaryPrimeDeltaCount > Int.natAbs payload.stableZeroPrimeDeltaCount",
        )?;
    }

    writeln!(out, "end {module_name}")?;

    Ok(out)
}

fn render_value_theorem(
    out: &mut String,
    field_name: &str,
    value: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(
        out,
        "theorem {field_name}_value : payload.{field_name} = {value} := by"
    )?;
    writeln!(out, "  native_decide")?;
    writeln!(out)?;
    Ok(())
}

fn render_int_value_theorem(
    out: &mut String,
    field_name: &str,
    value: isize,
) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(
        out,
        "theorem {field_name}_value : payload.{field_name} = ({value} : Int) := by"
    )?;
    writeln!(out, "  native_decide")?;
    writeln!(out)?;
    Ok(())
}

fn render_native_decide_theorem(
    out: &mut String,
    theorem_name: &str,
    proposition: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(out, "theorem {theorem_name} : {proposition} := by")?;
    writeln!(out, "  native_decide")?;
    writeln!(out)?;
    Ok(())
}

fn bool_lit(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn display_path(path: &Path) -> String {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.strip_prefix(&repo_root)
        .map(|relative| relative.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
}

fn default_display_output(args: &Args) -> Result<String, Box<dyn std::error::Error>> {
    let path = resolve_output_path(args)?;
    Ok(display_path(&path))
}
