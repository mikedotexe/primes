//! Ridge and trough detection for lineouts

use super::grid_analysis::{Axis, JoinedGrid};

/// Ridge point (local minimum in our case, since we're looking for troughs in density)
#[derive(Debug, Clone)]
pub struct RidgePoint {
    pub key: usize,    // The fixed coordinate (e.g., mid_len for iz-axis sweep)
    pub argmin: usize, // Position of minimum along swept axis
    pub value: f64,    // Minimum value
}

/// Detect ridges (troughs) by sweeping along one axis
/// For each fixed value of the other axis, find the minimum along the sweep axis
pub fn ridge_trough(grid: &JoinedGrid, axis: Axis, quantity: &str) -> Vec<RidgePoint> {
    // Group rows by the fixed coordinate
    let mut groups: std::collections::HashMap<usize, Vec<(usize, f64)>> =
        std::collections::HashMap::new();

    for row in &grid.rows {
        let (key, var, value) = match axis {
            Axis::Mid => {
                // Sweeping along mid_len, fixed inner_zero
                let key = row.inner_zero;
                let var = row.mid_len;
                let val = extract_quantity(row, quantity);
                (key, var, val)
            }
            Axis::InnerZero => {
                // Sweeping along inner_zero, fixed mid_len
                let key = row.mid_len;
                let var = row.inner_zero;
                let val = extract_quantity(row, quantity);
                (key, var, val)
            }
        };

        if let (Some(k), Some(v), Some(val)) = (key, var, value) {
            groups.entry(k).or_insert_with(Vec::new).push((v, val));
        }
    }

    // For each group, find the minimum
    let mut ridges = Vec::new();
    for (key, mut points) in groups {
        if points.is_empty() {
            continue;
        }

        // Sort by variable coordinate
        points.sort_by_key(|(v, _)| *v);

        // Find minimum
        if let Some((argmin, value)) = points.iter().min_by(|(_, a), (_, b)| {
            a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
        }) {
            ridges.push(RidgePoint {
                key,
                argmin: *argmin,
                value: *value,
            });
        }
    }

    // Sort by key
    ridges.sort_by_key(|r| r.key);
    ridges
}

fn extract_quantity(row: &super::grid_analysis::GridRow, quantity: &str) -> Option<f64> {
    match quantity {
        "obs" => row.obs,
        "pred" => row.pred,
        "enrichment" => row.enrichment,
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hzlib::grid_analysis::{GridRow, JoinedGrid};

    #[test]
    fn test_ridge_detection() {
        let rows = vec![
            GridRow {
                base: Some(10),
                mid_len: Some(1),
                inner_zero: Some(0),
                obs: Some(0.15),
                pred: Some(0.12),
                ci_lo: None,
                ci_hi: None,
                enrichment: Some(0.25),
            },
            GridRow {
                base: Some(10),
                mid_len: Some(2),
                inner_zero: Some(0),
                obs: Some(0.10),
                pred: Some(0.11),
                ci_lo: None,
                ci_hi: None,
                enrichment: Some(-0.09),
            },
            GridRow {
                base: Some(10),
                mid_len: Some(3),
                inner_zero: Some(0),
                obs: Some(0.12),
                pred: Some(0.10),
                ci_lo: None,
                ci_hi: None,
                enrichment: Some(0.20),
            },
        ];

        let grid = JoinedGrid { rows };
        let ridges = ridge_trough(&grid, Axis::Mid, "obs");

        assert_eq!(ridges.len(), 1);
        assert_eq!(ridges[0].key, 0); // inner_zero = 0
        assert_eq!(ridges[0].argmin, 2); // mid_len = 2 has minimum obs (0.10)
        assert!((ridges[0].value - 0.10).abs() < 1e-10);
    }
}
