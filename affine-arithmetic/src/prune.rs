//! Pruning by magnitude: remove small coefficients and merge their absolute sum
//! into one fresh noise term so the enclosure cannot shrink.

use crate::{Affine, Ctx, Sym};

impl Affine {
    /// Prune all coefficients with `|c| < eps`. Their absolute values are summed
    /// and replaced by a single fresh noise term (if non-zero).
    pub fn prune_below(&mut self, eps: f64, ctx: &mut Ctx) {
        if self.terms.is_empty() {
            return;
        }
        let mut kept: Vec<(Sym, f64)> = Vec::with_capacity(self.terms.len());
        let mut tail_sum = 0.0f64;
        for &(s, c) in &self.terms {
            if c.abs() < eps {
                tail_sum += c.abs();
            } else {
                kept.push((s, c));
            }
        }
        if tail_sum != 0.0 {
            kept.push((ctx.fresh(), tail_sum));
        }
        kept.sort_by_key(|(s, _)| s.0);
        self.terms = kept;
    }
}

#[cfg(test)]
mod tests {

    use crate::{Affine, Ctx};

    #[test]
    fn prune_small_terms_keeps_enclosure() {
        let mut ctx = Ctx::new();
        let mut a = Affine {
            a0: 0.0,
            terms: Vec::new(),
        };
        let big = ctx.fresh();
        a.terms.push((big, 0.5));
        for _ in 0..20 {
            a.terms.push((ctx.fresh(), 1e-6));
        }
        let (lo0, hi0) = a.to_interval();
        a.prune_below(1e-5, &mut ctx);
        let (lo1, hi1) = a.to_interval();
        assert!(lo1 <= lo0 + 1e-12 && hi1 >= hi0 - 1e-12);
        assert!(a.terms.iter().any(|(s, c)| *s == big && c.abs() == 0.5));
    }
}
