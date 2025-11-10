//! Hybrid fallbacks: if symbol count exceeds a threshold, fall back to interval
//! evaluation and return an affine built from the resulting interval.
//! With `--features hybrid` or `rigorous`, inari is used for outward-rounded ops.

use crate::{nonlinear::{range_cos, range_sin}, Affine, Ctx};

impl Affine {
    /// Hybrid multiply: if combined terms > `max_terms`, use IA fallback.
    pub fn mul_hybrid(&self, other: &Self, ctx: &mut Ctx, max_terms: usize) -> Self {
        if self.terms.len() + other.terms.len() <= max_terms {
            return self.mul_ctx(other, ctx);
        }
        let (alo, ahi) = self.to_interval();
        let (blo, bhi) = other.to_interval();

        #[cfg(any(feature = "hybrid", feature = "rigorous"))]
        {
            use inari::Interval;
            let a = Interval::new(alo, ahi).unwrap();
            let b = Interval::new(blo, bhi).unwrap();
            let z = a * b;
            return Affine::from_interval(z.inf(), z.sup(), ctx);
        }

        #[cfg(not(any(feature = "hybrid", feature = "rigorous")))]
        {
            // Endpoint products (safe without inari, but not outward-rounded).
            let cands = [alo*blo, alo*bhi, ahi*blo, ahi*bhi];
            let mut lo = f64::INFINITY;
            let mut hi = f64::NEG_INFINITY;
            for &v in &cands { if v < lo { lo = v; } if v > hi { hi = v; } }
            return Affine::from_interval(lo, hi, ctx);
        }
    }

    /// Hybrid unary: if symbol count > `max_terms`, evaluate f over the interval.
    pub fn exp_hybrid(&self, ctx: &mut Ctx, max_terms: usize) -> Self {
        if self.terms.len() <= max_terms { return self.exp_ctx(ctx); }
        let (lo, hi) = self.to_interval();
        Affine::from_interval(lo.exp(), hi.exp(), ctx)
    }
    pub fn log_hybrid(&self, ctx: &mut Ctx, max_terms: usize) -> Self {
        let (lo, hi) = self.to_interval();
        assert!(lo > 0.0, "log domain requires interval > 0");
        if self.terms.len() <= max_terms { return self.log_ctx(ctx); }
        let a = lo.ln(); let b = hi.ln();
        Affine::from_interval(a.min(b), a.max(b), ctx)
    }
    pub fn sin_hybrid(&self, ctx: &mut Ctx, max_terms: usize) -> Self {
        if self.terms.len() <= max_terms { return self.sin_ctx(ctx); }
        let (lo, hi) = self.to_interval();
        let (mn, mx) = range_sin(lo, hi);
        Affine::from_interval(mn, mx, ctx)
    }
    pub fn cos_hybrid(&self, ctx: &mut Ctx, max_terms: usize) -> Self {
        if self.terms.len() <= max_terms { return self.cos_ctx(ctx); }
        let (lo, hi) = self.to_interval();
        let (mn, mx) = range_cos(lo, hi);
        Affine::from_interval(mn, mx, ctx)
    }
    pub fn sqrt_hybrid(&self, ctx: &mut Ctx, max_terms: usize) -> Self {
        let (lo, hi) = self.to_interval();
        assert!(lo >= 0.0, "sqrt domain requires interval ≥ 0");
        if self.terms.len() <= max_terms { return self.sqrt_ctx(ctx); }
        Affine::from_interval(0.0f64.max(lo).sqrt(), hi.sqrt(), ctx)
    }
}

#[cfg(test)]
mod tests {
    
    use crate::{Affine, Ctx};

    #[test]
    fn mul_hybrid_encloses() {
        let mut ctx = Ctx::new();
        // Create many symbols to trigger hybrid
        let mut a = Affine { a0: 1.0, terms: Vec::new() };
        let mut b = Affine { a0: 2.0, terms: Vec::new() };
        for _ in 0..64 {
            a.terms.push((ctx.fresh(), 0.01));
            b.terms.push((ctx.fresh(), 0.02));
        }
        let z = a.mul_hybrid(&b, &mut ctx, 8);
        let (alo, ahi) = a.to_interval();
        let (blo, bhi) = b.to_interval();
        let cands = [alo*blo, alo*bhi, ahi*blo, ahi*bhi];
        let mut truth_lo = f64::INFINITY;
        let mut truth_hi = f64::NEG_INFINITY;
        for &v in &cands { if v < truth_lo { truth_lo = v; } if v > truth_hi { truth_hi = v; } }
        let (zlo, zhi) = z.to_interval();
        assert!(zlo <= truth_lo + 1e-12 && zhi >= truth_hi - 1e-12);
    }
}
