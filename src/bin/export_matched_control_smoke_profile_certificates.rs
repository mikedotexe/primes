//! Export maintained smoke-profile certificate metadata.

use clap::Parser;
use primes::validation::{
    matched_control::{
        build_matched_control_smoke_profile_certificate_metadata,
        render_matched_control_smoke_profile_certificate_lean_candidates,
        render_matched_control_smoke_profile_certificate_lean_checks,
        render_matched_control_smoke_profile_certificate_lean_module,
        render_matched_control_smoke_profile_certificate_lean_silent_check_shards,
        render_matched_control_smoke_profile_certificate_lean_silent_checks,
    },
    reporting::write_text_file,
};
use std::{
    error::Error,
    io,
    path::{Path, PathBuf},
    process,
};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Export maintained matched-control smoke-profile certificate metadata"
)]
struct Args {
    /// Output format: json, lean-candidates, lean-checks, lean-silent-checks, or lean-module.
    #[arg(long, default_value = "json")]
    format: String,

    /// Optional output path. If omitted, output is printed to stdout.
    #[arg(long)]
    out: Option<PathBuf>,

    /// Optional number of smoke-profile certificate rows per generated silent-check shard.
    #[arg(long)]
    shard_size: Option<usize>,

    /// Lean module prefix used by generated shard imports.
    #[arg(long, default_value = "PrimeArithmetic.Generated")]
    module_prefix: String,

    /// Optional directory for shard files. Defaults to the parent of --out.
    #[arg(long)]
    shard_out_dir: Option<PathBuf>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("export-matched-control-smoke-profile-certificates error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    if args.shard_size.is_some() && args.format != "lean-silent-checks" {
        return Err("--shard-size is only valid with --format lean-silent-checks".into());
    }

    if let Some(shard_size) = args.shard_size {
        let Some(path) = &args.out else {
            return Err("--shard-size requires --out".into());
        };
        let output_path = absolute_output_path(path);
        let umbrella_stem = output_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "--out must have a UTF-8 file stem when --shard-size is set",
                )
            })?;
        let shard_out_dir = args
            .shard_out_dir
            .as_deref()
            .map(absolute_output_path)
            .unwrap_or_else(|| {
                output_path
                    .parent()
                    .map(Path::to_path_buf)
                    .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
            });
        let bundle = render_matched_control_smoke_profile_certificate_lean_silent_check_shards(
            &args.module_prefix,
            umbrella_stem,
            shard_size,
        )
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
        write_text_file(&output_path, &bundle.umbrella_contents)?;
        for shard in &bundle.shards {
            write_text_file(shard_out_dir.join(&shard.file_name), &shard.contents)?;
        }
        println!(
            "Wrote smoke-profile certificate Lean check umbrella: {}",
            display_path(&output_path)
        );
        println!(
            "Wrote {} smoke-profile certificate Lean check shard(s) under {}",
            bundle.shards.len(),
            display_path(&shard_out_dir)
        );
        return Ok(());
    }

    let output = match args.format.as_str() {
        "json" => serde_json::to_string_pretty(
            &build_matched_control_smoke_profile_certificate_metadata(),
        )?,
        "lean-candidates" => render_matched_control_smoke_profile_certificate_lean_candidates(),
        "lean-checks" => render_matched_control_smoke_profile_certificate_lean_checks(),
        "lean-silent-checks" => {
            render_matched_control_smoke_profile_certificate_lean_silent_checks()
        }
        "lean-module" => render_matched_control_smoke_profile_certificate_lean_module(),
        _ => return Err(
            "invalid --format value; expected json, lean-candidates, lean-checks, lean-silent-checks, or lean-module"
                .into(),
        ),
    };

    if let Some(path) = &args.out {
        let output_path = absolute_output_path(path);
        write_text_file(&output_path, &output)?;
        println!(
            "Wrote smoke-profile certificate metadata: {}",
            display_path(&output_path)
        );
    } else {
        println!("{output}");
    }

    Ok(())
}

fn absolute_output_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path)
    }
}

fn display_path(path: &Path) -> String {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.strip_prefix(&repo_root)
        .unwrap_or(path)
        .display()
        .to_string()
}
