//! Verification functions for comparing sample vs model predictions

use super::grid_analysis::JoinedGrid;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Write verification table to CSV
/// Columns: base, mid_len, inner_zero, obs, pred, delta, enrichment, ci_lo, ci_hi, ci_width
pub fn verify_to_csv<P: AsRef<Path>>(
    grid: &JoinedGrid,
    _explain: Option<&HashMap<String, Value>>,
    out_path: P,
) -> io::Result<()> {
    let mut file = File::create(out_path)?;

    // Write header
    writeln!(
        file,
        "base,mid_len,inner_zero,obs,pred,delta,enrichment,ci_lo,ci_hi,ci_width"
    )?;

    // Write rows
    for row in &grid.rows {
        let delta = match (row.obs, row.pred) {
            (Some(o), Some(p)) => Some(o - p),
            _ => None,
        };

        let ci_width = match (row.ci_lo, row.ci_hi) {
            (Some(lo), Some(hi)) => Some(hi - lo),
            _ => None,
        };

        writeln!(
            file,
            "{},{},{},{},{},{},{},{},{},{}",
            fmt_opt_u32(row.base),
            fmt_opt_usize(row.mid_len),
            fmt_opt_usize(row.inner_zero),
            fmt_opt_f64(row.obs),
            fmt_opt_f64(row.pred),
            fmt_opt_f64(delta),
            fmt_opt_f64(row.enrichment),
            fmt_opt_f64(row.ci_lo),
            fmt_opt_f64(row.ci_hi),
            fmt_opt_f64(ci_width)
        )?;
    }

    Ok(())
}

fn fmt_opt_f64(x: Option<f64>) -> String {
    match x {
        Some(v) => format!("{:.12}", v),
        None => String::from(""),
    }
}

fn fmt_opt_u32(x: Option<u32>) -> String {
    match x {
        Some(v) => v.to_string(),
        None => String::from(""),
    }
}

fn fmt_opt_usize(x: Option<usize>) -> String {
    match x {
        Some(v) => v.to_string(),
        None => String::from(""),
    }
}
