//! Nonlinear unary ops via derivative-range linearization with rigorous remainder:
//! For `y=f(x)`, with `x∈[lo,hi]`, slope `s = (min f', max f')` average,
//! center `f(a0)`, remainder `ρ = (max f' - min f') * (Σ|ai|)/2`.

use crate::{Affine, Ctx};

pub(crate) const PI: f64 = core::f64::consts::PI;
pub(crate) const TAU: f64 = core::f64::consts::PI * 2.0;

// Range helpers for sin/cos derivatives (periodic extrema)
pub(crate) fn range_contains(base: f64, period: f64, lo: f64, hi: f64) -> bool {
    if hi < lo { return false; }
    let k_min = ((lo - base) / period).ceil();
    let k_max = ((hi - base) / period).floor();
    k_min <= k_max
}
pub(crate) fn range_sin(lo: f64, hi: f64) -> (f64, f64) {
    if hi - lo >= TAU { return (-1.0, 1.0); }
    let s_lo = lo.sin();
    let s_hi = hi.sin();
    let mut mn = s_lo.min(s_hi);
    let mut mx = s_lo.max(s_hi);
    if range_contains(PI * 0.5, TAU, lo, hi) { mx = 1.0; }
    if range_contains(PI * 1.5, TAU, lo, hi) { mn = -1.0; }
    (mn, mx)
}
pub(crate) fn range_cos(lo: f64, hi: f64) -> (f64, f64) {
    if hi - lo >= TAU { return (-1.0, 1.0); }
    let c_lo = lo.cos();
    let c_hi = hi.cos();
    let mut mn = c_lo.min(c_hi);
    let mut mx = c_lo.max(c_hi);
    if range_contains(0.0, TAU, lo, hi) { mx = 1.0; }
    if range_contains(PI, TAU, lo, hi) { mn = -1.0; }
    (mn, mx)
}

// Core enclosure primitive: linear map with derivative range and remainder.
fn cheb_linear_map(
    a: &Affine,
    ctx: &mut Ctx,
    f_center: impl Fn(f64) -> f64,
    d_range: impl Fn(f64, f64) -> (f64, f64),
) -> Affine {
    let (lo, hi) = a.to_interval();
    let y0 = f_center(a.a0);
    let (dmin, dmax) = d_range(lo, hi);
    let s = 0.5 * (dmin + dmax);
    let r = a.radius_l1();
    let rho = 0.5 * (dmax - dmin) * r;

    let mut terms = Vec::with_capacity(a.terms.len() + if rho != 0.0 { 1 } else { 0 });
    for &(sym, c) in &a.terms {
        terms.push((sym, s * c));
    }
    if rho != 0.0 {
        let sn = ctx.fresh();
        terms.push((sn, rho));
    }
    Affine { a0: y0, terms }
}

impl Affine {
    /// `exp(x)` enclosure.
    pub fn exp_ctx(&self, ctx: &mut Ctx) -> Affine {
        cheb_linear_map(self, ctx, |x| x.exp(), |lo, hi| (lo.exp(), hi.exp()))
    }
    /// `log(x)` enclosure; domain requires `x > 0` on the whole interval.
    pub fn log_ctx(&self, ctx: &mut Ctx) -> Affine {
        let (lo, _) = self.to_interval();
        assert!(lo > 0.0, "log domain requires interval > 0");
        cheb_linear_map(self, ctx, |x| x.ln(), |lo, hi| {
            // f'(x)=1/x monotone; derivative range is [1/hi, 1/lo]
            let dmin = 1.0 / hi;
            let dmax = 1.0 / lo;
            (dmin.min(dmax), dmin.max(dmax))
        })
    }
    /// `sin(x)` enclosure using cos derivative range.
    pub fn sin_ctx(&self, ctx: &mut Ctx) -> Affine {
        cheb_linear_map(self, ctx, |x| x.sin(), |lo, hi| range_cos(lo, hi))
    }
    /// `cos(x)` enclosure using -sin derivative range.
    pub fn cos_ctx(&self, ctx: &mut Ctx) -> Affine {
        cheb_linear_map(self, ctx, |x| x.cos(), |lo, hi| {
            let (smin, smax) = range_sin(lo, hi);
            (-smax, -smin)
        })
    }
    /// `sqrt(x)` enclosure; domain requires `x ≥ 0` on the whole interval.
    /// If `lo == 0`, fall back to interval enclosure `[0, sqrt(hi)]`.
    pub fn sqrt_ctx(&self, ctx: &mut Ctx) -> Affine {
        let (lo, hi) = self.to_interval();
        assert!(lo >= 0.0, "sqrt domain requires interval ≥ 0");
        if lo == 0.0 {
            return Affine::from_interval(0.0, hi.sqrt(), ctx);
        }
        // f'(x) = 1/(2√x), decreasing on (0, ∞)
        cheb_linear_map(self, ctx, |x| x.sqrt(), |lo, hi| {
            let dmin = 1.0 / (2.0 * hi.sqrt());
            let dmax = 1.0 / (2.0 * lo.sqrt());
            (dmin, dmax)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Affine, Ctx};

    fn encloses(img_lo: f64, img_hi: f64, aff: &Affine) -> bool {
        let (alo, ahi) = aff.to_interval();
        alo <= img_lo + 1e-12 && ahi + 1e-12 >= img_hi
    }

    #[test]
    fn exp_encloses() {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(-0.1, 0.2, &mut ctx);
        let y = x.exp_ctx(&mut ctx);
        let (lo, hi) = x.to_interval();
        assert!(encloses(lo.exp(), hi.exp(), &y));
    }

    #[test]
    fn log_encloses() {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(0.9, 1.1, &mut ctx);
        let y = x.log_ctx(&mut ctx);
        let (lo, hi) = x.to_interval();
        let a = lo.ln(); let b = hi.ln();
        assert!(encloses(a.min(b), a.max(b), &y));
    }

    #[test]
    fn sin_encloses_small() {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(0.4, 0.6, &mut ctx);
        let y = x.sin_ctx(&mut ctx);
        let (lo, hi) = x.to_interval();
        let (img_lo, img_hi) = (lo.sin().min(hi.sin()), lo.sin().max(hi.sin()));
        assert!(encloses(img_lo, img_hi, &y));
    }

    #[test]
    fn sin_encloses_wide() {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(-1.0, 5.5, &mut ctx);
        let y = x.sin_ctx(&mut ctx);
        let (mn, mx) = super::range_sin(-1.0, 5.5);
        assert!(encloses(mn, mx, &y));
    }

    #[test]
    fn cos_encloses() {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(2.7, 3.5, &mut ctx);
        let y = x.cos_ctx(&mut ctx);
        let (lo, hi) = x.to_interval();
        let c_lo = lo.cos();
        let c_hi = hi.cos();
        let mut mn = c_lo.min(c_hi);
        let mut mx = c_lo.max(c_hi);
        if (hi - lo) >= TAU || super::range_contains(0.0, TAU, lo, hi) { mx = 1.0; }
        if (hi - lo) >= TAU || super::range_contains(PI, TAU, lo, hi) { mn = -1.0; }
        assert!(encloses(mn, mx, &y));
    }

    #[test]
    fn sqrt_encloses_positive() {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(1.0, 4.0, &mut ctx);
        let y = x.sqrt_ctx(&mut ctx);
        assert!(encloses(1.0, 2.0, &y));
    }

    #[test]
    fn sqrt_handles_zero_lower() {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(0.0, 9.0, &mut ctx);
        let y = x.sqrt_ctx(&mut ctx);
        assert!(encloses(0.0, 3.0, &y));
    }

    #[test]
    #[should_panic]
    fn sqrt_domain_panics() {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(-1.0, 4.0, &mut ctx);
        let _ = x.sqrt_ctx(&mut ctx);
    }
}
