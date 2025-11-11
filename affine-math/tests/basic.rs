use affine_math::{Affine, Ctx};

#[test]
fn add_sub_scalar() {
    let mut ctx = Ctx::new();
    let x = Affine::from_interval(2.0, 4.0, &mut ctx);
    let y = Affine::cst(3.0);
    let z = (x.clone() + y.clone()) - y.clone();
    assert_eq!(x.to_interval(), z.to_interval());
    let w = x.clone() * 2.0;
    let (lo, hi) = w.to_interval();
    assert_eq!((lo, hi), ((3.0 * 2.0 - 1.0 * 2.0), (3.0 * 2.0 + 1.0 * 2.0))); // [2,4] → mid=3,r=1
}

#[test]
fn mul_encloses_endpoints() {
    let mut ctx = Ctx::new();
    let a = Affine::from_interval(1.95, 2.05, &mut ctx);
    let b = Affine::from_interval(2.9, 3.1, &mut ctx);
    let z = a.mul_ctx(&b, &mut ctx);
    let (zlo, zhi) = z.to_interval();
    let endpoints = [1.95 * 2.9, 1.95 * 3.1, 2.05 * 2.9, 2.05 * 3.1];
    let truth_lo = endpoints.iter().cloned().fold(f64::INFINITY, f64::min);
    let truth_hi = endpoints.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    assert!(zlo <= truth_lo + 1e-12 && zhi >= truth_hi - 1e-12);
}

#[test]
fn correlation_cancels() {
    let mut ctx = Ctx::new();
    let s = ctx.fresh();
    let a = Affine {
        a0: 3.0,
        terms: vec![(s, 0.1)],
    };
    let z = a.clone() - a;
    assert_eq!(z.to_interval(), (0.0, 0.0));
}
