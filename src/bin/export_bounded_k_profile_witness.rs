//! Export a direct bounded-`k` residue-profile witness as a Lean artifact.

use clap::Parser;
use primes::validation::bounded_k::{
    digit_symbol, format_k, scan_k_config_lane_profile_comparison, DEFAULT_BOUNDED_K_GRID,
};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Export a direct bounded-k residue-profile witness as a Lean artifact"
)]
struct Args {
    #[arg(long)]
    base: u32,

    #[arg(long = "middle-length")]
    middle_length: usize,

    #[arg(long)]
    outer: u32,

    #[arg(long)]
    inner: u32,

    #[arg(long = "from-k-outer", default_value_t = 0)]
    from_k_outer: u32,

    #[arg(long = "from-k-inner", default_value_t = 0)]
    from_k_inner: u32,

    #[arg(long = "to-k-outer")]
    to_k_outer: u32,

    #[arg(long = "to-k-inner")]
    to_k_inner: u32,

    #[arg(long)]
    out: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    validate_args(&args)?;

    let from_k = (args.from_k_outer, args.from_k_inner);
    let to_k = (args.to_k_outer, args.to_k_inner);
    let comparison = scan_k_config_lane_profile_comparison(
        args.base,
        args.middle_length,
        args.outer,
        args.inner,
        from_k,
        to_k,
    );
    let output_path = resolve_output_path(&args)?;
    let module_name = lean_module_name(&output_path)?;
    let artifact = render_artifact(&args, &comparison, &module_name)?;

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
    println!("Comparison: {} -> {}", format_k(from_k), format_k(to_k));
    println!(
        "Rung: {} | compared moduli {} | admissible delta {}",
        comparison.theorem_rung_label,
        comparison.compared_moduli_count,
        comparison.admissible_delta_count
    );

    Ok(())
}

fn validate_args(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    if args.base < 2 {
        return Err("--base must be at least 2".into());
    }
    if args.outer >= args.base || args.inner >= args.base {
        return Err("--outer and --inner must be valid digits in the selected base".into());
    }
    let from_k = (args.from_k_outer, args.from_k_inner);
    let to_k = (args.to_k_outer, args.to_k_inner);
    if !DEFAULT_BOUNDED_K_GRID.contains(&from_k) || !DEFAULT_BOUNDED_K_GRID.contains(&to_k) {
        return Err("from_k and to_k must lie in DEFAULT_BOUNDED_K_GRID".into());
    }
    Ok(())
}

fn resolve_output_path(args: &Args) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let default_name = format!(
        "Base{}M{}Pair{}{}{}To{}.lean",
        args.base,
        args.middle_length,
        digit_symbol(args.outer),
        digit_symbol(args.inner),
        compact_k_label((args.from_k_outer, args.from_k_inner)),
        compact_k_label((args.to_k_outer, args.to_k_inner))
    );
    let output = args.out.clone().unwrap_or_else(|| {
        PathBuf::from("lean-proofs")
            .join("PrimeArithmetic")
            .join("Generated")
            .join("BoundedKProfiles")
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

fn render_artifact(
    args: &Args,
    comparison: &primes::validation::bounded_k::KConfigLaneProfileComparison,
    module_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut out = String::new();
    let command = format!(
        "cargo run --bin export_bounded_k_profile_witness -- --base {} --middle-length {} --outer {} --inner {} --from-k-outer {} --from-k-inner {} --to-k-outer {} --to-k-inner {} --out {}",
        args.base,
        args.middle_length,
        args.outer,
        args.inner,
        args.from_k_outer,
        args.from_k_inner,
        args.to_k_outer,
        args.to_k_inner,
        default_display_output(args)?
    );

    writeln!(
        out,
        "import PrimeArithmetic.Generated.BoundedKResidueProfileShell"
    )?;
    writeln!(out)?;
    writeln!(out, "namespace {module_name}")?;
    writeln!(out)?;
    writeln!(
        out,
        "open PrimeArithmetic.Generated.BoundedKResidueProfileShell"
    )?;
    writeln!(out)?;
    writeln!(out, "/-!")?;
    writeln!(
        out,
        "Runtime-exported bounded-k residue-profile witness artifact."
    )?;
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
    writeln!(
        out,
        "- lane comparison: `{}` -> `{}`",
        format_k((args.from_k_outer, args.from_k_inner)),
        format_k((args.to_k_outer, args.to_k_inner))
    )?;
    writeln!(out, "- theorem rung: `{}`", comparison.theorem_rung_label)?;
    writeln!(
        out,
        "- admissible delta count: `{}`",
        comparison.admissible_delta_count
    )?;
    writeln!(out, "-/")?;
    writeln!(out)?;
    writeln!(out, "def payload : GeneratedResidueProfilePayload where")?;
    writeln!(out, "  base := {}", args.base)?;
    writeln!(out, "  middleLength := {}", args.middle_length)?;
    writeln!(out, "  outer := {}", args.outer)?;
    writeln!(out, "  inner := {}", args.inner)?;
    writeln!(out, "  fromKOuter := {}", args.from_k_outer)?;
    writeln!(out, "  fromKInner := {}", args.from_k_inner)?;
    writeln!(out, "  toKOuter := {}", args.to_k_outer)?;
    writeln!(out, "  toKInner := {}", args.to_k_inner)?;
    writeln!(out, "  rows := [")?;
    for row in &comparison.modulus_rows {
        writeln!(
            out,
            "    {{ modulus := {}, excludedSeedClassesFrom := [{}], excludedSeedClassesTo := [{}] }},",
            row.modulus,
            row.excluded_seed_classes_from
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(", "),
            row.excluded_seed_classes_to
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(", "),
        )?;
    }
    writeln!(out, "  ]")?;
    writeln!(out, "  stableZeroCount := {}", comparison.stable_zero_count)?;
    writeln!(out, "  gainZeroCount := {}", comparison.gain_zero_count)?;
    writeln!(out, "  lossZeroCount := {}", comparison.loss_zero_count)?;
    writeln!(
        out,
        "  stableNonzeroCount := {}",
        comparison.stable_nonzero_count
    )?;
    writeln!(
        out,
        "  nonzeroChurnCount := {}",
        comparison.nonzero_churn_count
    )?;
    writeln!(out)?;
    writeln!(
        out,
        "theorem comparedModulusCount_value : payload.comparedModulusCount = {} := by",
        comparison.compared_moduli_count
    )?;
    writeln!(out, "  native_decide")?;
    writeln!(out)?;
    writeln!(
        out,
        "theorem agreeingModulusCount_value : payload.agreeingModulusCount = {} := by",
        comparison
            .modulus_rows
            .iter()
            .filter(|row| row.profile_agreement)
            .count()
    )?;
    writeln!(out, "  native_decide")?;
    writeln!(out)?;
    writeln!(
        out,
        "theorem admissibleSetEqualBool_value : payload.admissibleSetEqualBool = {} := by",
        if comparison.admissible_set_equal {
            "true"
        } else {
            "false"
        }
    )?;
    writeln!(out, "  native_decide")?;
    writeln!(out)?;
    writeln!(
        out,
        "theorem profileAgreementBool_value : payload.profileAgreementBool = {} := by",
        if comparison.profile_agreement {
            "true"
        } else {
            "false"
        }
    )?;
    writeln!(out, "  native_decide")?;
    writeln!(out)?;
    writeln!(
        out,
        "theorem admissibleDeltaCount_value : payload.admissibleDeltaCount = {} := by",
        comparison.admissible_delta_count
    )?;
    writeln!(out, "  native_decide")?;
    writeln!(out)?;
    writeln!(
        out,
        "theorem noPositiveAdmissibleDeltaBool_value : payload.noPositiveAdmissibleDeltaBool = {} := by",
        if comparison.no_positive_admissible_delta { "true" } else { "false" }
    )?;
    writeln!(out, "  native_decide")?;
    writeln!(out)?;
    writeln!(out, "end {module_name}")?;

    Ok(out)
}

fn compact_k_label((k_outer, k_inner): (u32, u32)) -> String {
    format!("K{k_outer}{k_inner}")
}

fn default_display_output(args: &Args) -> Result<String, Box<dyn std::error::Error>> {
    Ok(format!(
        "lean-proofs/PrimeArithmetic/Generated/BoundedKProfiles/Base{}M{}Pair{}{}{}To{}.lean",
        args.base,
        args.middle_length,
        digit_symbol(args.outer),
        digit_symbol(args.inner),
        compact_k_label((args.from_k_outer, args.from_k_inner)),
        compact_k_label((args.to_k_outer, args.to_k_inner))
    ))
}

fn display_path(path: &Path) -> String {
    path.strip_prefix(env!("CARGO_MANIFEST_DIR"))
        .ok()
        .map(|relative| relative.display().to_string())
        .unwrap_or_else(|| path.display().to_string())
}
