use crate::hzlib::{JoinedGrid, Axis};

pub fn dft_real(xs: &[f64]) -> Vec<(usize, f64, f64)> {
    let n = xs.len();
    let two_pi = std::f64::consts::PI * 2.0;
    let mut out = Vec::with_capacity(n);
    for k in 0..n {
        let mut re = 0.0;
        let mut im = 0.0;
        for (t, &x) in xs.iter().enumerate() {
            let ang = two_pi * (k as f64) * (t as f64) / (n as f64);
            re += x * ang.cos();
            im -= x * ang.sin();
        }
        let amp = (re*re + im*im).sqrt() / (n as f64);
        let phase = im.atan2(re);
        out.push((k, amp, phase));
    }
    out
}

pub fn overtone_spectrum(
    grid: &JoinedGrid,
    axis: Axis,
    fixed_mid: usize,
    fixed_iz: usize,
    quantity: &str,
) -> Vec<(usize, f64)> {
    let series: Vec<f64> = match axis {
        Axis::Mid => {
            grid.mids.iter().map(|&m| {
                let idx = grid.idx(m, fixed_iz).unwrap();
                val_of(&grid.pairs[idx], quantity)
            }).collect()
        }
        Axis::InnerZero => {
            grid.izs.iter().map(|&z| {
                let idx = grid.idx(fixed_mid, z).unwrap();
                val_of(&grid.pairs[idx], quantity)
            }).collect()
        }
    };
    let spec = dft_real(&series);
    let mut amps: Vec<(usize, f64)> = spec.into_iter().map(|(k,a,_)| (k,a)).collect();
    amps.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
    amps
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
