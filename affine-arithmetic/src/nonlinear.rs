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

    pub fn tan_ctx(&self, ctx: &mut Ctx) -> Affine {
        // tan' = sec² = 1 + tan²
        // For derivative range, compute tan at endpoints and extrema
        let (lo, hi) = self.to_interval();

        // Check if interval crosses a discontinuity at π/2 + nπ
        let k_min = ((lo - PI/2.0) / PI).ceil();
        let k_max = ((hi - PI/2.0) / PI).floor();

        if k_min <= k_max {
            // Interval crosses discontinuity - tan is unbounded
            panic!("tan domain error: interval [{}, {}] crosses discontinuity", lo, hi);
        }

        cheb_linear_map(
            self,
            ctx,
            |x| x.tan(),
            |lo, hi| {
                // Derivative sec²(x) = 1 + tan²(x)
                let sec2_lo = 1.0 + lo.tan().powi(2);
                let sec2_hi = 1.0 + hi.tan().powi(2);
                (sec2_lo.min(sec2_hi), sec2_lo.max(sec2_hi))
            },
        )
    }

    pub fn atan_ctx(&self, ctx: &mut Ctx) -> Affine {
        // atan' = 1/(1 + x²), always positive, decreasing in |x|
        cheb_linear_map(
            self,
            ctx,
            |x| x.atan(),
            |lo, hi| {
                // Derivative 1/(1+x²) is monotone decreasing in |x|
                let d_lo = 1.0 / (1.0 + lo * lo);
                let d_hi = 1.0 / (1.0 + hi * hi);
                // Find min/max considering 0 might be in interval
                let dmin = d_lo.min(d_hi);
                let dmax = if lo <= 0.0 && hi >= 0.0 {
                    1.0 // Maximum at x=0
                } else {
                    d_lo.max(d_hi)
                };
                (dmin, dmax)
            },
        )
    }

    pub fn sinh_ctx(&self, ctx: &mut Ctx) -> Affine {
        // sinh(x) = (exp(x) - exp(-x))/2
        // sinh'(x) = cosh(x) = (exp(x) + exp(-x))/2
        cheb_linear_map(
            self,
            ctx,
            |x| x.sinh(),
            |lo, hi| {
                // cosh is always positive, minimum at x=0
                let cosh_lo = lo.cosh();
                let cosh_hi = hi.cosh();
                let mut dmin = cosh_lo.min(cosh_hi);
                let dmax = cosh_lo.max(cosh_hi);
                if lo <= 0.0 && hi >= 0.0 {
                    dmin = 1.0; // cosh(0) = 1
                }
                (dmin, dmax)
            },
        )
    }

    pub fn cosh_ctx(&self, ctx: &mut Ctx) -> Affine {
        // cosh'(x) = sinh(x)
        cheb_linear_map(
            self,
            ctx,
            |x| x.cosh(),
            |lo, hi| {
                // sinh is monotonic, ranges from sinh(lo) to sinh(hi)
                let sinh_lo = lo.sinh();
                let sinh_hi = hi.sinh();
                (sinh_lo.min(sinh_hi), sinh_lo.max(sinh_hi))
            },
        )
    }

    pub fn tanh_ctx(&self, ctx: &mut Ctx) -> Affine {
        // tanh'(x) = sech²(x) = 1 - tanh²(x)
        cheb_linear_map(
            self,
            ctx,
            |x| x.tanh(),
            |lo, hi| {
                // sech² is maximum at x=0, decreasing away from 0
                let sech2_lo = {
                    let th = lo.tanh();
                    1.0 - th * th
                };
                let sech2_hi = {
                    let th = hi.tanh();
                    1.0 - th * th
                };
                let dmin = sech2_lo.min(sech2_hi);
                let dmax = if lo <= 0.0 && hi >= 0.0 {
                    1.0 // Maximum at x=0
                } else {
                    sech2_lo.max(sech2_hi)
                };
                (dmin, dmax)
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

    #[test]
    fn tan_encloses() {
        let mut ctx = Ctx::new();
        // Small interval away from discontinuities
        let x = Affine::from_interval(0.3, 0.5, &mut ctx);
        let y = x.tan_ctx(&mut ctx);
        let (lo, hi) = x.to_interval();
        let truth_lo = lo.tan().min(hi.tan());
        let truth_hi = lo.tan().max(hi.tan());
        assert!(encloses(truth_lo, truth_hi, &y));
    }

    #[test]
    #[should_panic(expected = "tan domain error")]
    fn tan_panics_on_discontinuity() {
        let mut ctx = Ctx::new();
        // Interval crosses π/2
        let x = Affine::from_interval(1.0, 2.0, &mut ctx);
        let _y = x.tan_ctx(&mut ctx); // Should panic
    }

    #[test]
    fn atan_encloses() {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(-2.0, 3.0, &mut ctx);
        let y = x.atan_ctx(&mut ctx);
        let (lo, hi) = x.to_interval();
        let truth_lo = lo.atan().min(hi.atan());
        let truth_hi = lo.atan().max(hi.atan());
        assert!(encloses(truth_lo, truth_hi, &y));
    }

    #[test]
    fn sinh_encloses() {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(-1.0, 1.5, &mut ctx);
        let y = x.sinh_ctx(&mut ctx);
        let (lo, hi) = x.to_interval();
        let truth_lo = lo.sinh().min(hi.sinh());
        let truth_hi = lo.sinh().max(hi.sinh());
        assert!(encloses(truth_lo, truth_hi, &y));
    }

    #[test]
    fn cosh_encloses() {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(-0.5, 1.0, &mut ctx);
        let y = x.cosh_ctx(&mut ctx);
        let (lo, hi) = x.to_interval();
        // cosh has minimum at 0
        let truth_lo = if lo <= 0.0 && hi >= 0.0 {
            1.0
        } else {
            lo.cosh().min(hi.cosh())
        };
        let truth_hi = lo.cosh().max(hi.cosh());
        assert!(encloses(truth_lo, truth_hi, &y));
    }

    #[test]
    fn tanh_encloses() {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(-2.0, 2.0, &mut ctx);
        let y = x.tanh_ctx(&mut ctx);
        let (lo, hi) = x.to_interval();
        let truth_lo = lo.tanh().min(hi.tanh());
        let truth_hi = lo.tanh().max(hi.tanh());
        assert!(encloses(truth_lo, truth_hi, &y));
    }
}
