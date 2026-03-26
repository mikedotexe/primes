//! Export a finite prime window as a Lean certificate artifact.
//!
//! The emitted file targets the maintained generated-data symmetry shell:
//! `GeneratedWindowPayload`, `GeneratedDualEvidence`, and the corresponding
//! dual certificate layer in the Lean package.

use clap::Parser;
use num_bigint::BigUint;
use primes::{is_prime, BitSieve};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Export a prime window as a Lean window-certificate artifact"
)]
struct Args {
    /// Prime parameter p for the window midpoint 2 * p^2
    #[arg(long)]
    p: u64,

    /// Even modulus used for residue buckets
    #[arg(long)]
    base: u64,

    /// Half-width of the absolute window around 2 * p^2
    #[arg(long)]
    window_span: u64,

    /// Midpoint exclusion radius used in the dynamic certificate
    #[arg(long, default_value_t = 1)]
    exclude_radius: u64,

    /// Output path under lean-proofs/PrimeArithmetic/Generated/
    #[arg(long)]
    out: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    validate_args(&args)?;

    let midpoint = checked_midpoint(args.p)?;
    let lower = midpoint.saturating_sub(args.window_span);
    let upper = midpoint
        .checked_add(args.window_span)
        .ok_or("window upper bound overflowed u64")?;
    let limit = usize::try_from(upper).map_err(|_| "window upper bound does not fit in usize")?;

    let positions = extract_prime_positions(limit, lower, upper);
    if positions.is_empty() {
        return Err("no primes were found in the requested window".into());
    }

    let residues = extract_residues(&positions, args.base);
    validate_fixed_point_exclusion(&residues, args.base)?;
    let counts = residue_counts(&residues, args.base)?;
    validate_balanced_counts(&counts, args.base)?;
    validate_pointwise_safety(&positions, midpoint, args.exclude_radius)?;

    let output_path = resolve_output_path(&args)?;
    let module_name = lean_module_name(&output_path)?;
    let artifact = render_artifact(
        &args,
        midpoint,
        &positions,
        &residues,
        &counts,
        &module_name,
    )?;

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output_path, artifact)?;

    let display_path = display_path(&output_path);
    println!("Wrote Lean artifact: {display_path}");
    println!("Module: {module_name}");
    println!("Window midpoint: {midpoint}");
    println!("Prime positions: {:?}", positions);
    println!("Residues mod {}: {:?}", args.base, residues);
    println!("Residue counts: {}", format_residue_counts(&counts));

    Ok(())
}

fn validate_args(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    if args.base < 2 {
        return Err("--base must be at least 2".into());
    }
    if !args.base.is_multiple_of(2) {
        return Err("--base must be even so the midpoint residue is defined".into());
    }
    if args.p < 2 {
        return Err("--p must be at least 2".into());
    }
    if !is_prime(&BigUint::from(args.p)) {
        return Err("--p must be prime".into());
    }
    Ok(())
}

fn checked_midpoint(p: u64) -> Result<u64, Box<dyn std::error::Error>> {
    p.checked_mul(p)
        .and_then(|n| n.checked_mul(2))
        .ok_or_else(|| "2 * p^2 overflowed u64".into())
}

fn extract_prime_positions(limit: usize, lower: u64, upper: u64) -> Vec<u64> {
    BitSieve::new(limit)
        .primes()
        .into_iter()
        .map(|n| n as u64)
        .filter(|&n| lower <= n && n <= upper)
        .collect()
}

fn extract_residues(positions: &[u64], base: u64) -> Vec<u64> {
    positions.iter().map(|&n| n % base).collect()
}

fn residue_counts(residues: &[u64], base: u64) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let base_usize = usize::try_from(base).map_err(|_| "base does not fit in usize")?;
    let mut counts = vec![0usize; base_usize];
    for &r in residues {
        let idx = usize::try_from(r).map_err(|_| "residue does not fit in usize")?;
        counts[idx] += 1;
    }
    Ok(counts)
}

fn validate_fixed_point_exclusion(
    residues: &[u64],
    base: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let midpoint = base / 2;
    if residues.contains(&0) {
        return Err(
            "the extracted residue list contains 0, so no fixed-point exclusion proof exists"
                .into(),
        );
    }
    if residues.contains(&midpoint) {
        return Err(format!(
            "the extracted residue list contains the midpoint residue {}, so no fixed-point exclusion proof exists",
            midpoint
        )
        .into());
    }
    Ok(())
}

fn validate_balanced_counts(counts: &[usize], base: u64) -> Result<(), Box<dyn std::error::Error>> {
    for r in 0..base {
        let s = (base - r) % base;
        let r_idx = usize::try_from(r).map_err(|_| "residue index does not fit in usize")?;
        let s_idx = usize::try_from(s).map_err(|_| "residue index does not fit in usize")?;
        if counts[r_idx] != counts[s_idx] {
            return Err(format!(
                "balanced reflected bucket counts failed at residues {} and {} ({} != {})",
                r, s, counts[r_idx], counts[s_idx]
            )
            .into());
        }
    }
    Ok(())
}

fn validate_pointwise_safety(
    positions: &[u64],
    midpoint: u64,
    radius: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    for &x in positions {
        if x.abs_diff(midpoint) < radius {
            return Err(format!(
                "pointwise midpoint-radius safety failed at position {} (distance {} < radius {})",
                x,
                x.abs_diff(midpoint),
                radius
            )
            .into());
        }
    }
    Ok(())
}

fn resolve_output_path(args: &Args) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let default_name = format!(
        "WindowP{}Base{}Span{}.lean",
        args.p, args.base, args.window_span
    );
    let output = args.out.clone().unwrap_or_else(|| {
        PathBuf::from("lean-proofs")
            .join("PrimeArithmetic")
            .join("Generated")
            .join("Runtime")
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
    midpoint: u64,
    positions: &[u64],
    residues: &[u64],
    counts: &[usize],
    module_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut out = String::new();
    let command = format!(
        "cargo run --bin export_window_certificate -- --p {} --base {} --window-span {} --exclude-radius {} --out {}",
        args.p,
        args.base,
        args.window_span,
        args.exclude_radius,
        default_display_output(args)?
    );
    let namespace = module_name;

    writeln!(
        out,
        "import PrimeArithmetic.Symmetry.WindowCertificateErgonomics"
    )?;
    writeln!(out)?;
    writeln!(out, "namespace {namespace}")?;
    writeln!(out)?;
    writeln!(out, "open PrimeArithmetic.Symmetry.ModularReflection")?;
    writeln!(out, "open PrimeArithmetic.Symmetry.WindowCertificate")?;
    writeln!(
        out,
        "open PrimeArithmetic.Symmetry.WindowCertificateGenerated"
    )?;
    writeln!(
        out,
        "open PrimeArithmetic.Symmetry.WindowCertificateErgonomics"
    )?;
    writeln!(out)?;
    writeln!(out, "/-!")?;
    writeln!(out, "Runtime-exported finite window certificate artifact.")?;
    writeln!(out)?;
    writeln!(out, "Generated by:")?;
    writeln!(out, "`{command}`")?;
    writeln!(out)?;
    writeln!(out, "- midpoint: `{midpoint}`")?;
    writeln!(out, "- window span: `{}`", args.window_span)?;
    writeln!(out, "- exclusion radius: `{}`", args.exclude_radius)?;
    writeln!(out, "- prime positions: `{}`", format_lean_list(positions))?;
    writeln!(
        out,
        "- residues mod {}: `{}`",
        args.base,
        format_lean_list(residues)
    )?;
    writeln!(out, "- residue counts: `{}`", format_residue_counts(counts))?;
    writeln!(out, "-/")?;
    writeln!(out)?;
    writeln!(
        out,
        "def payload : GeneratedWindowPayload {} where",
        args.base
    )?;
    writeln!(out, "  p := {}", args.p)?;
    writeln!(out, "  windowMid := {midpoint}")?;
    writeln!(out, "  radius := {}", args.exclude_radius)?;
    writeln!(out, "  residues := {}", format_lean_list(residues))?;
    writeln!(out, "  positions := {}", format_lean_list(positions))?;
    writeln!(out)?;
    writeln!(
        out,
        "abbrev window : WindowData {} payload.residues.length := payload.windowData",
        args.base
    )?;
    writeln!(out)?;
    writeln!(out, "theorem balanced :")?;
    writeln!(
        out,
        "    ∀ r, payload.derivedCount r = payload.derivedCount (reflect {} r) := by",
        args.base
    )?;
    writeln!(out, "  intro r")?;
    writeln!(out, "  fin_cases r <;> native_decide")?;
    writeln!(out)?;
    writeln!(
        out,
        "def fixedPointExclusion : PrimeArithmetic.Symmetry.CertificateReflection.ObservedFixedPointExclusion payload.residueFn where"
    )?;
    writeln!(out, "  zeroVoid := by")?;
    writeln!(out, "    intro i")?;
    writeln!(out, "    fin_cases i <;> native_decide")?;
    writeln!(out, "  midpointVoid := by")?;
    writeln!(out, "    intro i")?;
    writeln!(out, "    fin_cases i <;> native_decide")?;
    writeln!(out)?;
    writeln!(out, "theorem pointwiseSafe :")?;
    writeln!(
        out,
        "    PointwiseSafe payload.radius payload.windowMid payload.positions := by"
    )?;
    writeln!(out, "  intro x hx")?;
    writeln!(out, "  simp [payload, SafePos] at hx ⊢")?;
    writeln!(out, "  rcases hx with {}", rcases_pattern(positions.len()))?;
    writeln!(out, "  all_goals native_decide")?;
    writeln!(out)?;
    writeln!(out, "def evidence : GeneratedDualEvidence payload where")?;
    writeln!(out, "  balanced := balanced")?;
    writeln!(out, "  fixedPointExclusion := fixedPointExclusion")?;
    writeln!(out, "  pointwiseSafe := pointwiseSafe")?;
    writeln!(out)?;
    writeln!(out, "theorem midpoint_residue_not_in_range :")?;
    writeln!(
        out,
        "    midpoint {} ∉ Set.range window.residue :=",
        args.base
    )?;
    writeln!(
        out,
        "  evidence.midpoint_not_in_range (hEven := show Even {} by native_decide)",
        args.base
    )?;
    writeln!(out)?;
    writeln!(out, "theorem zero_residue_not_in_range :")?;
    writeln!(
        out,
        "    (0 : Fin {}) ∉ Set.range window.residue :=",
        args.base
    )?;
    writeln!(
        out,
        "  evidence.zero_not_in_range (hEven := show Even {} by native_decide)",
        args.base
    )?;
    writeln!(out)?;
    writeln!(out, "theorem window_inviolability :")?;
    writeln!(
        out,
        "    InZone payload.radius payload.windowMid payload.positions → False :="
    )?;
    writeln!(
        out,
        "  evidence.inviolability (hEven := show Even {} by native_decide)",
        args.base
    )?;
    writeln!(out)?;
    writeln!(out, "end {namespace}")?;
    Ok(out)
}

fn default_display_output(args: &Args) -> Result<String, Box<dyn std::error::Error>> {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = resolve_output_path(args)?;
    Ok(path
        .strip_prefix(&repo_root)
        .unwrap_or(&path)
        .display()
        .to_string())
}

fn format_lean_list(values: &[u64]) -> String {
    let body = values
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{body}]")
}

fn format_residue_counts(counts: &[usize]) -> String {
    let mut active = BTreeMap::new();
    for (residue, &count) in counts.iter().enumerate() {
        if count != 0 {
            active.insert(residue, count);
        }
    }
    active
        .into_iter()
        .map(|(residue, count)| format!("{residue}->{count}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn rcases_pattern(length: usize) -> String {
    assert!(
        length > 0,
        "generated artifacts require at least one position"
    );
    std::iter::repeat_n("rfl", length)
        .collect::<Vec<_>>()
        .join(" | ")
}

fn display_path(path: &Path) -> String {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.strip_prefix(&repo_root)
        .unwrap_or(path)
        .display()
        .to_string()
}
