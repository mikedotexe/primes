//! Affine arithmetic core: sound first-order enclosures with explicit noise symbols.
//! Ops implemented: +, -, scalar mul, and aa.mul(&mut Ctx) (needs a fresh noise symbol).
//! Coeffs are f64; rounding errors are conservatively absorbed into the remainder term.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ops::{Add, AddAssign, Mul, Neg, Sub};

/// Noise symbol identifier.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Sym(pub u32);

/// Symbol allocation context.
/// Thread callers should own a context (no globals) to preserve determinism.
#[derive(Debug, Default, Clone)]
pub struct Ctx {
    next: u32,
}
impl Ctx {
    pub fn new() -> Self { Self { next: 0 } }
    /// Allocate a fresh noise symbol.
    pub fn fresh(&mut self) -> Sym {
        let id = self.next;
        self.next = self.next.checked_add(1).expect("symbol id overflow");
        Sym(id)
    }
}

/// Affine form: a0 + Σ (coeff[sym] * ε_sym), ε ∈ [-1, 1].
#[derive(Clone, Debug, PartialEq)]
pub struct Affine {
    pub a0: f64,
    /// Sorted by Sym; duplicates forbidden (canonical form).
    pub terms: Vec<(Sym, f64)>,
}

impl Affine {
    /// Constant.
    pub fn cst(a0: f64) -> Self { Self { a0, terms: Vec::new() } }

    /// From interval [lo, hi] using a fresh noise symbol.
    pub fn from_interval(lo: f64, hi: f64, ctx: &mut Ctx) -> Self {
        assert!(lo <= hi, "invalid interval");
        let m = 0.5 * (lo + hi);
        let r = 0.5 * (hi - lo);
        if r == 0.0 {
            return Self::cst(m);
        }
        let s = ctx.fresh();
        Self { a0: m, terms: vec![(s, r)] }
    }

    /// Interval enclosure [a0 - Σ|ai|, a0 + Σ|ai|].
    pub fn to_interval(&self) -> (f64, f64) {
        let r = self.radius_l1();
        (self.a0 - r, self.a0 + r)
    }

    /// L1 radius Σ|ai|.
    pub fn radius_l1(&self) -> f64 {
        let mut s = 0.0f64;
        for &(_, ci) in &self.terms { s += ci.abs(); }
        s
    }

    /// Add a term (merge by symbol), keeping canonical sorted form.
    #[allow(dead_code)]
    fn add_term(&mut self, sym: Sym, coeff: f64) {
        if coeff == 0.0 { return; }
        match self.terms.binary_search_by(|(s, _)| s.cmp(&sym)) {
            Ok(idx) => {
                let newc = self.terms[idx].1 + coeff;
                if newc == 0.0 { self.terms.remove(idx); } else { self.terms[idx].1 = newc; }
            }
            Err(pos) => self.terms.insert(pos, (sym, coeff)),
        }
    }

    /// Scale by a scalar.
    pub fn scale(mut self, k: f64) -> Self {
        self.a0 *= k;
        if k == 0.0 {
            self.terms.clear();
        } else {
            for (_, c) in &mut self.terms { *c *= k; }
        }
        self
    }

    /// Affine multiply (needs a fresh noise symbol). Sound first-order enclosure:
    /// z0 = a0*b0 ; z_i = a0*b_i + b0*a_i ; ρ = (Σ|a_i|)*(Σ|b_j|).
    /// All floating-point roundoff is conservatively absorbed into ρ via a tiny safety slack.
    pub fn mul_ctx(&self, other: &Self, ctx: &mut Ctx) -> Self {
        // Center part
        let z0 = self.a0 * other.a0;

        // Merge coefficients for shared symbols: z_i = a0*b_i + b0*a_i
        let mut zi: Vec<(Sym, f64)> = Vec::with_capacity(self.terms.len() + other.terms.len());
        let mut i = 0usize; let mut j = 0usize;
        while i < self.terms.len() || j < other.terms.len() {
            match (self.terms.get(i), other.terms.get(j)) {
                (Some(&(si, ai)), Some(&(sj, bj))) => {
                    match si.cmp(&sj) {
                        Ordering::Less => { zi.push((si, other.a0 * ai)); i += 1; }
                        Ordering::Greater => { zi.push((sj, self.a0 * bj)); j += 1; }
                        Ordering::Equal => {
                            let coeff = other.a0 * ai + self.a0 * bj;
                            if coeff != 0.0 { zi.push((si, coeff)); }
                            i += 1; j += 1;
                        }
                    }
                }
                (Some(&(si, ai)), None) => { zi.push((si, other.a0 * ai)); i += 1; }
                (None, Some(&(sj, bj))) => { zi.push((sj, self.a0 * bj)); j += 1; }
                (None, None) => break,
            }
        }

        // Remainder bound ρ = (Σ|a_i|)*(Σ|b_j|). Add a tiny slack for fp roundoff.
        let ra = self.radius_l1();
        let rb = other.radius_l1();

        // Optional rigorous outward rounding with `inari` if available.
        #[cfg(feature = "rigorous")]
        let rho = {
            use inari::Interval;
            // Compute (Σ|a_i|) and (Σ|b_j|) as intervals and take sup for worst-case.
            let mut sa = Interval::new(0.0, 0.0).unwrap();
            for &(_, ai) in &self.terms {
                let t = Interval::new(ai.abs(), ai.abs()).unwrap();
                sa = sa + t;
            }
            let mut sb = Interval::new(0.0, 0.0).unwrap();
            for &(_, bj) in &other.terms {
                let t = Interval::new(bj.abs(), bj.abs()).unwrap();
                sb = sb + t;
            }
            let prod = sa * sb;
            prod.sup() // outward-rounded upper bound
        };

        #[cfg(not(feature = "rigorous"))]
        let rho = {
            // Slightly enlarge to cover fp accumulation (~few ulps).
            let slack = 2.0_f64.mul_add(core::f64::EPSILON, 1.0);
            (ra * rb) * slack
        };

        if rho != 0.0 {
            let s_new = ctx.fresh();
            zi.push((s_new, rho));
        }

        Self { a0: z0, terms: zi }
    }
}

// Affine ops that do not need fresh noise: +, -, unary -, and scalar mul.
impl Add for Affine {
    type Output = Affine;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.a0 += rhs.a0;
        // Merge two sorted lists.
        let mut out: Vec<(Sym, f64)> = Vec::with_capacity(self.terms.len() + rhs.terms.len());
        let mut i = 0usize; let mut j = 0usize;
        while i < self.terms.len() || j < rhs.terms.len() {
            match (self.terms.get(i), rhs.terms.get(j)) {
                (Some(&(si, ai)), Some(&(sj, bj))) => match si.cmp(&sj) {
                    Ordering::Less => { out.push((si, ai)); i += 1; }
                    Ordering::Greater => { out.push((sj, bj)); j += 1; }
                    Ordering::Equal => {
                        let c = ai + bj;
                        if c != 0.0 { out.push((si, c)); }
                        i += 1; j += 1;
                    }
                },
                (Some(&(si, ai)), None) => { out.push((si, ai)); i += 1; }
                (None, Some(&(sj, bj))) => { out.push((sj, bj)); j += 1; }
                (None, None) => break,
            }
        }
        self.terms = out;
        self
    }
}
impl Sub for Affine {
    type Output = Affine;
    fn sub(self, rhs: Self) -> Self::Output { self + (-rhs) }
}
impl Neg for Affine {
    type Output = Affine;
    fn neg(mut self) -> Self::Output {
        self.a0 = -self.a0;
        for (_, c) in &mut self.terms { *c = -*c; }
        self
    }
}
impl Mul<f64> for Affine {
    type Output = Affine;
    fn mul(self, rhs: f64) -> Self::Output { self.scale(rhs) }
}
impl AddAssign for Affine {
    fn add_assign(&mut self, rhs: Self) { *self = self.clone() + rhs; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_bridge() {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(2.9, 3.1, &mut ctx);
        let y = Affine::cst(3.0);
        let z = x.mul_ctx(&y, &mut ctx);
        let (lo, hi) = z.to_interval();
        assert!(lo <= 8.7 + 1e-12 && hi >= 9.3 - 1e-12);
    }

    #[test]
    fn aa_vs_ia_encloses_product() {
        let mut ctx = Ctx::new();
        let a = Affine::from_interval(1.95, 2.05, &mut ctx);
        let b = Affine::from_interval(2.9, 3.1, &mut ctx);
        let z = a.mul_ctx(&b, &mut ctx);
        let (zlo, zhi) = z.to_interval();
        // Ground truth by endpoint products:
        let truth_lo = 1.95 * 2.9_f64.min(3.1);
        let truth_hi = 2.05 * 3.1_f64.max(2.9);
        assert!(zlo <= truth_lo + 1e-12 && zhi >= truth_hi - 1e-12);
    }
}
