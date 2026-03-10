use crate::hzlib::{Axis, JoinedGrid};

pub fn lagrange_interpolate(xs: &[f64], ys: &[f64], x: f64) -> f64 {
    let n = xs.len();
    let mut sum = 0.0;
    for i in 0..n {
        let mut li = 1.0;
        for j in 0..n {
            if i == j {
                continue;
            }
            li *= (x - xs[j]) / (xs[i] - xs[j]);
        }
        sum += ys[i] * li;
    }
    sum
}

pub fn fit_lineout_poly(
    grid: &JoinedGrid,
    axis: Axis,
    fixed_mid: usize,
    fixed_iz: usize,
    degree: usize,
    quantity: &str,
) -> Vec<(usize, f64, f64)> {
    let (xs_u, ys_u) = match axis {
        Axis::Mid => {
            let xs = grid.mids.clone();
            let ys: Vec<f64> = xs
                .iter()
                .map(|&m| {
                    let idx = grid.idx(m, fixed_iz).unwrap();
                    val_of(&grid.pairs[idx], quantity)
                })
                .collect();
            (xs, ys)
        }
        Axis::InnerZero => {
            let xs = grid.izs.clone();
            let ys: Vec<f64> = xs
                .iter()
                .map(|&z| {
                    let idx = grid.idx(fixed_mid, z).unwrap();
                    val_of(&grid.pairs[idx], quantity)
                })
                .collect();
            (xs, ys)
        }
    };

    let n = xs_u.len();
    let d = degree.min(n.saturating_sub(1));
    // choose d+1 Chebyshev-like nodes from the discrete set for stability.
    let mut idxs = Vec::with_capacity(d + 1);
    for i in 0..=d {
        let t = ((2 * i + 1) as f64) / (2 * (d + 1)) as f64;
        let pos = ((n as f64 - 1.0) * 0.5 * (1.0 - (std::f64::consts::PI * (1.0 - t)).cos()))
            .round() as usize;
        idxs.push(pos.min(n - 1));
    }
    idxs.sort_unstable();
    idxs.dedup();

    let xs: Vec<f64> = idxs.iter().map(|&i| xs_u[i] as f64).collect();
    let ys: Vec<f64> = idxs.iter().map(|&i| ys_u[i]).collect();

    let mut out = Vec::with_capacity(n);
    for (k, &xk) in xs_u.iter().enumerate() {
        let est = lagrange_interpolate(&xs, &ys, xk as f64);
        out.push((xk, ys_u[k], est));
    }
    out
}

fn val_of(p: &crate::hzlib::Pair, quantity: &str) -> f64 {
    match quantity {
        "obs" => p.obs_density.unwrap_or(0.0),
        "pred" => p.pred_local_exact.or(p.pred_local).unwrap_or(0.0),
        "enrichment" => {
            let o = p.obs_density.unwrap_or(0.0);
            let pr = p.pred_local_exact.or(p.pred_local).unwrap_or(0.0);
            super::enrichment(o, pr)
        }
        _ => 0.0,
    }
}
