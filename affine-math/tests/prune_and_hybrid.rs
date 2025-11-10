use affine_math::{Affine, Ctx};

#[test]
fn prune_below_keeps_enclosure_and_largest() {
    let mut ctx = Ctx::new();
    let mut a = Affine { a0: 0.0, terms: Vec::new() };
    let big = ctx.fresh();
    a.terms.push((big, 0.5));
    for _ in 0..50 { a.terms.push((ctx.fresh(), 1e-7)); }
    let (lo0, hi0) = a.to_interval();
    a.prune_below(1e-6, &mut ctx);
    let (lo1, hi1) = a.to_interval();
    assert!(lo1 <= lo0 + 1e-12 && hi1 >= hi0 - 1e-12);
    assert!(a.terms.iter().any(|(s, c)| *s == big && c.abs() == 0.5));
}

#[test]
fn hybrid_mul_encloses() {
    let mut ctx = Ctx::new();
    let mut a = Affine { a0: 1.0, terms: Vec::new() };
    let mut b = Affine { a0: 2.0, terms: Vec::new() };
    for _ in 0..64 {
        a.terms.push((ctx.fresh(), 0.01));
        b.terms.push((ctx.fresh(), 0.02));
    }
    let z = a.mul_hybrid(&b, &mut ctx, 8);
    let (alo, ahi) = a.to_interval();
    let (blo, bhi) = b.to_interval();
    let endpoints = [alo*blo, alo*bhi, ahi*blo, ahi*bhi];
    let truth_lo = endpoints.iter().cloned().fold(f64::INFINITY, f64::min);
    let truth_hi = endpoints.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let (zlo, zhi) = z.to_interval();
    assert!(zlo <= truth_lo + 1e-12 && zhi >= truth_hi - 1e-12);
}
