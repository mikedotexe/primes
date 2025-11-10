use crate::{Affine, Ctx, Sym};

impl Affine {
    pub fn condense(&mut self, max_terms: usize, ctx: &mut Ctx) {
        assert!(max_terms >= 1, "max_terms must be >= 1");
        if self.terms.len() <= max_terms { return; }

        let mut idx_abs: Vec<(usize, f64)> = self.terms
            .iter()
            .enumerate()
            .map(|(i, &(_, c))| (i, c.abs()))
            .collect();

        idx_abs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let keep_n = max_terms.saturating_sub(1);
        let mut keep_mask = vec![false; self.terms.len()];
        for (k, _) in idx_abs.iter().take(keep_n) {
            keep_mask[*k] = true;
        }

        let mut new_terms: Vec<(Sym, f64)> = Vec::with_capacity(max_terms);
        let mut tail_sum = 0.0f64;

        for (i, &(sym, c)) in self.terms.iter().enumerate() {
            if keep_mask[i] {
                new_terms.push((sym, c));
            } else {
                tail_sum += c.abs();
            }
        }

        if tail_sum != 0.0 {
            let s = ctx.fresh();
            new_terms.push((s, tail_sum));
        }

        new_terms.sort_by_key(|(s, _)| s.0);
        self.terms = new_terms;
    }
}

#[cfg(test)]
mod tests {
    use crate::{Affine, Ctx};

    #[test]
    fn condense_reduces_terms_and_encloses() {
        let mut ctx = Ctx::new();
        // a0 = 0, terms: many small ones
        let mut a = Affine { a0: 0.0, terms: Vec::new() };
        for _ in 0..20 {
            let s = ctx.fresh();
            a.terms.push((s, 0.01));
        }
        let (lo0, hi0) = a.to_interval(); // [-0.2, 0.2]

        a.condense(5, &mut ctx);
        assert!(a.terms.len() <= 5);

        let (lo1, hi1) = a.to_interval();
        assert!(lo1 <= lo0 + 1e-12 && hi1 >= hi0 - 1e-12);
    }

    #[test]
    fn condense_keeps_largest() {
        let mut ctx = Ctx::new();
        let mut a = Affine { a0: 1.0, terms: Vec::new() };
        let s_big = ctx.fresh();
        a.terms.push((s_big, 0.5));
        for _ in 0..10 {
            let s = ctx.fresh();
            a.terms.push((s, 0.01));
        }
        a.condense(3, &mut ctx);
        assert!(a.terms.iter().any(|(s, c)| *s == s_big && c.abs() == 0.5));
    }
}
