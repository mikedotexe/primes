use crate::{Affine, Ctx};

const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::PI * 2.0;

fn range_contains(base: f64, period: f64, lo: f64, hi: f64) -> bool {
    if hi < lo { return false; }
    let k_min = ((lo - base) / period).ceil();
    let k_max = ((hi - base) / period).floor();
    k_min <= k_max
}

fn range_sin(lo: f64, hi: f64) -> (f64, f64) {
    if hi - lo >= TAU { return (-1.0, 1.0); }
    let s_lo = lo.sin();
    let s_hi = hi.sin();
    let mut mn = s_lo.min(s_hi);
    let mut mx = s_lo.max(s_hi);
    if range_contains(PI * 0.5, TAU, lo, hi) { mx = 1.0; }
    if range_contains(PI * 1.5, TAU, lo, hi) { mn = -1.0; }
    (mn, mx)
}

fn range_cos(lo: f64, hi: f64) -> (f64, f64) {
    if hi - lo >= TAU { return (-1.0, 1.0); }
    let c_lo = lo.cos();
    let c_hi = hi.cos();
    let mut mn = c_lo.min(c_hi);
    let mut mx = c_lo.max(c_hi);
    if range_contains(0.0, TAU, lo, hi) { mx = 1.0; }
    if range_contains(PI, TAU, lo, hi) { mn = -1.0; }
    (mn, mx)
}

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
    pub fn exp_ctx(&self, ctx: &mut Ctx) -> Affine {
        cheb_linear_map(
            self,
            ctx,
            |x| x.exp(),
            |lo, hi| (lo.exp(), hi.exp()),
        )
    }

    pub fn log_ctx(&self, ctx: &mut Ctx) -> Affine {
        let (lo, _hi) = self.to_interval();
        assert!(lo > 0.0, "log domain requires interval > 0");
        cheb_linear_map(
            self,
            ctx,
            |x| x.ln(),
            |lo, hi| {
                assert!(lo > 0.0, "log' domain requires interval > 0");
                let dmin = 1.0 / hi;
                let dmax = 1.0 / lo;
                (dmin.min(dmax), dmin.max(dmax))
            },
        )
    }

    pub fn sin_ctx(&self, ctx: &mut Ctx) -> Affine {
        cheb_linear_map(
            self,
            ctx,
            |x| x.sin(),
            |lo, hi| range_cos(lo, hi),
        )
    }

    pub fn cos_ctx(&self, ctx: &mut Ctx) -> Affine {
        // f' = -sin, so range is negated sin range
        cheb_linear_map(
            self,
            ctx,
            |x| x.cos(),
            |lo, hi| {
                let (smin, smax) = range_sin(lo, hi);
                (-smax, -smin)
            },
        )
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
        let img_lo = lo.exp();
        let img_hi = hi.exp();
        assert!(encloses(img_lo, img_hi, &y));
    }

    #[test]
    fn log_encloses() {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(0.9, 1.1, &mut ctx);
        let y = x.log_ctx(&mut ctx);
        let (lo, hi) = x.to_interval();
        let img_lo = lo.ln();
        let img_hi = hi.ln();
        let lo_img = img_lo.min(img_hi);
        let hi_img = img_lo.max(img_hi);
        assert!(encloses(lo_img, hi_img, &y));
    }

    #[test]
    fn sin_encloses_small() {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(0.4, 0.6, &mut ctx);
        let y = x.sin_ctx(&mut ctx);
        let (lo, hi) = x.to_interval();
        let s_lo = lo.sin();
        let s_hi = hi.sin();
        let (img_lo, img_hi) = (s_lo.min(s_hi), s_lo.max(s_hi));
        assert!(encloses(img_lo, img_hi, &y));
    }

    #[test]
    fn sin_encloses_wide() {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(-1.0, 5.5, &mut ctx);
        let y = x.sin_ctx(&mut ctx);
        let (lo, hi) = (-1.0f64, 5.5f64);
        let (mn, mx) = super::range_sin(lo, hi);
        assert!(encloses(mn, mx, &y));
    }

    #[test]
    fn cos_encloses() {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(2.7, 3.5, &mut ctx);
        let y = x.cos_ctx(&mut ctx);
        let (lo, hi) = x.to_interval();
        // true cos range on [lo, hi]
        let c_lo = lo.cos();
        let c_hi = hi.cos();
        let mut mn = c_lo.min(c_hi);
        let mut mx = c_lo.max(c_hi);
        if (hi - lo) >= TAU || super::range_contains(0.0, TAU, lo, hi) { mx = 1.0; }
        if (hi - lo) >= TAU || super::range_contains(PI, TAU, lo, hi) { mn = -1.0; }
        assert!(encloses(mn, mx, &y));
    }

    #[test]
    #[should_panic(expected = "log domain requires interval > 0")]
    fn log_panics_on_nonpositive() {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(-1.0, 0.5, &mut ctx);
        let _y = x.log_ctx(&mut ctx); // Should panic
    }
}
