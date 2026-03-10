use crate::hzlib::{Axis, JoinedGrid};

pub struct Ridge {
    pub key: usize,
    pub argmin: usize,
    pub value: f64,
}

pub fn ridge_trough(grid: &JoinedGrid, axis: Axis, quantity: &str) -> Vec<Ridge> {
    match axis {
        Axis::Mid => {
            let mut out = Vec::with_capacity(grid.izs.len());
            for &z in &grid.izs {
                let mut best = (usize::MAX, f64::INFINITY);
                for &m in &grid.mids {
                    let idx = grid.idx(m, z).unwrap();
                    let v = val(&grid.pairs[idx], quantity);
                    if v < best.1 {
                        best = (m, v);
                    }
                }
                out.push(Ridge {
                    key: z,
                    argmin: best.0,
                    value: best.1,
                });
            }
            out
        }
        Axis::InnerZero => {
            let mut out = Vec::with_capacity(grid.mids.len());
            for &m in &grid.mids {
                let mut best = (usize::MAX, f64::INFINITY);
                for &z in &grid.izs {
                    let idx = grid.idx(m, z).unwrap();
                    let v = val(&grid.pairs[idx], quantity);
                    if v < best.1 {
                        best = (z, v);
                    }
                }
                out.push(Ridge {
                    key: m,
                    argmin: best.0,
                    value: best.1,
                });
            }
            out
        }
    }
}

fn val(p: &crate::hzlib::Pair, quantity: &str) -> f64 {
    match quantity {
        "pred" => p.pred_local_exact.or(p.pred_local).unwrap_or(0.0),
        "obs" => p.obs_density.unwrap_or(0.0),
        "enrichment" => {
            let o = p.obs_density.unwrap_or(0.0);
            let pr = p.pred_local_exact.or(p.pred_local).unwrap_or(0.0);
            super::enrichment(o, pr)
        }
        _ => p.pred_local_exact.or(p.pred_local).unwrap_or(0.0),
    }
}
