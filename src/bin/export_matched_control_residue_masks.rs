//! Export exact residue-mask scan reports for maintained matched-control lanes.

use clap::Parser;
use primes::validation::{
    matched_control::MatchedControlPanel,
    matched_control_residue_masks::{
        build_matched_control_residue_mask_report,
        render_matched_control_residue_mask_theorem_queue_markdown,
        render_matched_control_residue_mask_top_candidate_lean_checks,
        render_matched_control_residue_mask_top_candidate_lean_silent_checks,
        write_matched_control_residue_mask_report_json,
        write_matched_control_residue_mask_report_markdown,
        write_matched_control_residue_mask_theorem_queue_markdown,
        MatchedControlResidueMaskSettings, DEFAULT_RESIDUE_MASK_PRIME_BOUND,
    },
};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Export exact local residue-mask scans for maintained matched-control lanes"
)]
struct Args {
    /// Output format when --out-dir is omitted: json, theorem-queue, lean-candidate-checks, or lean-candidate-silent-checks.
    #[arg(long, default_value = "json")]
    format: String,

    /// Canonical matched-control panel to scan.
    #[arg(long, default_value = "smoke")]
    panel: String,

    /// Inclusive bound for prime moduli.
    #[arg(long, default_value_t = DEFAULT_RESIDUE_MASK_PRIME_BOUND)]
    prime_bound: u32,

    /// Optional output directory. Writes residue_masks.json, residue_masks.md, and theorem_queue.md.
    #[arg(long)]
    out_dir: Option<PathBuf>,

    /// Optional output path for single-file formats.
    #[arg(long)]
    out: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let panel = MatchedControlPanel::from_name(&args.panel)
        .ok_or("invalid --panel value; expected smoke or audit")?;
    let report = build_matched_control_residue_mask_report(MatchedControlResidueMaskSettings {
        panel,
        prime_bound: args.prime_bound,
    })?;

    if let Some(out_dir) = &args.out_dir {
        if args.format != "json" {
            return Err("--format cannot be combined with --out-dir".into());
        }
        if args.out.is_some() {
            return Err("--out cannot be combined with --out-dir".into());
        }
        let out_dir = absolute_output_path(out_dir);
        std::fs::create_dir_all(&out_dir)?;
        let json_path = out_dir.join("residue_masks.json");
        let markdown_path = out_dir.join("residue_masks.md");
        let theorem_queue_path = out_dir.join("theorem_queue.md");
        write_matched_control_residue_mask_report_json(&json_path, &report)?;
        write_matched_control_residue_mask_report_markdown(&markdown_path, &report)?;
        write_matched_control_residue_mask_theorem_queue_markdown(&theorem_queue_path, &report)?;
        println!(
            "Matched-control residue masks: {} ({})",
            report.summary.panel_id, report.summary.panel
        );
        println!("Prime bound: {}", report.summary.prime_bound);
        println!("Lanes: {}", report.summary.lane_count);
        println!(
            "Lane/modulus rows: {}",
            report.summary.lane_modulus_row_count
        );
        println!("Pair candidates: {}", report.summary.pair_candidate_count);
        println!(
            "Pair fingerprints: {}",
            report.summary.pair_fingerprint_row_count
        );
        println!(
            "Pair-certified fingerprints: {}",
            report.summary.pair_certified_count
        );
        println!(
            "Pair-uncertified fingerprints: {}",
            report.summary.pair_uncertified_count
        );
        println!(
            "Same-boundary k-distinction candidates: {}",
            report.summary.same_boundary_k_distinction_candidate_count
        );
        if let Some(candidate) = &report.summary.top_theorem_candidate {
            println!(
                "Top theorem candidate: #{} {} vs {} ({})",
                candidate.rank,
                candidate.left_family_code,
                candidate.right_family_code,
                candidate.selection_kind.as_str()
            );
        }
        println!("Wrote JSON: {}", display_path(&json_path));
        println!("Wrote Markdown: {}", display_path(&markdown_path));
        println!("Wrote theorem queue: {}", display_path(&theorem_queue_path));
    } else {
        let output = match args.format.as_str() {
            "json" => serde_json::to_string_pretty(&report)?,
            "theorem-queue" => render_matched_control_residue_mask_theorem_queue_markdown(&report),
            "lean-candidate-checks" => {
                render_matched_control_residue_mask_top_candidate_lean_checks(&report)
            }
            "lean-candidate-silent-checks" => {
                render_matched_control_residue_mask_top_candidate_lean_silent_checks(&report)
            }
            _ => return Err(
                "invalid --format value; expected json, theorem-queue, lean-candidate-checks, or lean-candidate-silent-checks"
                    .into(),
            ),
        };

        if let Some(path) = &args.out {
            let output_path = absolute_output_path(path);
            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&output_path, output)?;
            println!(
                "Wrote matched-control residue-mask output: {}",
                display_path(&output_path)
            );
        } else {
            println!("{output}");
        }
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
