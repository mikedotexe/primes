//! Grid analysis functions for CSV-based prime density data
//!
//! Provides data structures and functions for loading, joining, and analyzing
//! sample and model grid outputs from density-explorer.

use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Axis along which to perform analysis
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Axis {
    Mid,       // mid_len axis
    InnerZero, // inner_zero axis
}

/// Joined grid combining sample and model data with enrichment calculations
#[derive(Debug, Clone)]
pub struct JoinedGrid {
    pub rows: Vec<GridRow>,
}

/// Single row in a joined grid
#[derive(Debug, Clone)]
pub struct GridRow {
    pub base: Option<u32>,
    pub mid_len: Option<usize>,
    pub inner_zero: Option<usize>,
    pub obs: Option<f64>,      // observed density (from sample)
    pub pred: Option<f64>,     // predicted density (from model)
    pub ci_lo: Option<f64>,    // confidence interval low
    pub ci_hi: Option<f64>,    // confidence interval high
    pub enrichment: Option<f64>, // obs/pred - 1
}

/// Load sample CSV (observed densities)
pub fn load_sample_csv<P: AsRef<Path>>(path: P) -> io::Result<Vec<HashMap<String, String>>> {
    load_csv(path)
}

/// Load model CSV (predicted densities)
pub fn load_model_csv<P: AsRef<Path>>(path: P) -> io::Result<Vec<HashMap<String, String>>> {
    load_csv(path)
}

/// Load explain JSON (per-cell diagnostics)
/// Returns a map from cell key (e.g., "10_5_3" for base=10, mid_len=5, inner_zero=3) to JSON data
pub fn load_explain_json<P: AsRef<Path>>(path: P) -> io::Result<HashMap<String, Value>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let data: HashMap<String, Value> = serde_json::from_reader(reader)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(data)
}

/// Generic CSV loader - returns vec of hashmaps (column_name -> value)
fn load_csv<P: AsRef<Path>>(path: P) -> io::Result<Vec<HashMap<String, String>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Parse header
    let header = lines.next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Empty CSV"))??;
    let cols: Vec<String> = header.split(',').map(|s| s.trim().to_string()).collect();

    // Parse rows
    let mut rows = Vec::new();
    for line in lines {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let values: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
        let mut row = HashMap::new();
        for (i, col) in cols.iter().enumerate() {
            if let Some(val) = values.get(i) {
                row.insert(col.clone(), val.clone());
            }
        }
        rows.push(row);
    }
    Ok(rows)
}

/// Join sample and model data into a unified grid
pub fn join_sample_and_model(
    sample: &[HashMap<String, String>],
    model: &[HashMap<String, String>],
) -> JoinedGrid {
    // Build lookup map for model data by (base, mid_len, inner_zero)
    let mut model_map: HashMap<(u32, usize, usize), &HashMap<String, String>> = HashMap::new();
    for row in model {
        if let (Some(b), Some(m), Some(z)) = (
            parse_u32(row, "base"),
            parse_usize(row, "mid_len"),
            parse_usize(row, "inner_zero"),
        ) {
            model_map.insert((b, m, z), row);
        }
    }

    // Join sample with model
    let mut rows = Vec::new();
    for srow in sample {
        let base = parse_u32(srow, "base");
        let mid_len = parse_usize(srow, "mid_len");
        let inner_zero = parse_usize(srow, "inner_zero");

        let obs = parse_f64(srow, "prime_density");
        let ci_lo = parse_f64(srow, "ci_lo");
        let ci_hi = parse_f64(srow, "ci_hi");

        // Look up model prediction
        let pred = if let (Some(b), Some(m), Some(z)) = (base, mid_len, inner_zero) {
            model_map.get(&(b, m, z)).and_then(|mrow| {
                parse_f64(mrow, "expected_density_local_exact")
                    .or_else(|| parse_f64(mrow, "expected_density_local"))
            })
        } else {
            None
        };

        // Calculate enrichment
        let enrichment = match (obs, pred) {
            (Some(o), Some(p)) if p > 0.0 => Some(o / p - 1.0),
            _ => None,
        };

        rows.push(GridRow {
            base,
            mid_len,
            inner_zero,
            obs,
            pred,
            ci_lo,
            ci_hi,
            enrichment,
        });
    }

    JoinedGrid { rows }
}

/// Extract a lineout (1D slice) from the grid
/// Returns vec of (x, obs, pred) where x is the axis coordinate
pub fn lineout(grid: &JoinedGrid, axis: Axis, fixed_mid: usize, fixed_iz: usize) -> Vec<(usize, f64, f64)> {
    let mut result = Vec::new();

    for row in &grid.rows {
        let matches = match axis {
            Axis::Mid => row.inner_zero == Some(fixed_iz),
            Axis::InnerZero => row.mid_len == Some(fixed_mid),
        };

        if matches {
            if let (Some(obs), Some(pred)) = (row.obs, row.pred) {
                let x = match axis {
                    Axis::Mid => row.mid_len.unwrap_or(0),
                    Axis::InnerZero => row.inner_zero.unwrap_or(0),
                };
                result.push((x, obs, pred));
            }
        }
    }

    // Sort by x coordinate
    result.sort_by_key(|(x, _, _)| *x);
    result
}

/// Calculate enrichment metric: obs/pred - 1
pub fn enrichment(obs: f64, pred: f64) -> f64 {
    if pred > 0.0 {
        obs / pred - 1.0
    } else {
        f64::INFINITY
    }
}

// Helper parsers
fn parse_f64(map: &HashMap<String, String>, key: &str) -> Option<f64> {
    map.get(key).and_then(|v| v.parse::<f64>().ok())
}

fn parse_usize(map: &HashMap<String, String>, key: &str) -> Option<usize> {
    map.get(key).and_then(|v| v.parse::<usize>().ok())
}

fn parse_u32(map: &HashMap<String, String>, key: &str) -> Option<u32> {
    map.get(key).and_then(|v| v.parse::<u32>().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enrichment() {
        assert!((enrichment(0.12, 0.10) - 0.2).abs() < 1e-10);
        assert!((enrichment(0.10, 0.12) - (-1.0/6.0)).abs() < 1e-10);
        assert!(enrichment(0.10, 0.0).is_infinite());
    }
}
