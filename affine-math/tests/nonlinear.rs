use affine_math::{Affine, Ctx};

#[test]
fn exp_log_sin_cos_sqrt() {
    let mut ctx = Ctx::new();
    let x = Affine::from_interval(0.9, 1.1, &mut ctx);

    // exp
    let e = x.exp_ctx(&mut ctx);
    let (lo, hi) = x.to_interval();
    assert!(e.to_interval().0 <= lo.exp() + 1e-12);
    assert!(e.to_interval().1 >= hi.exp() - 1e-12);

    // log
    let l = x.log_ctx(&mut ctx);
    let (a, b) = (lo.ln(), hi.ln());
    let (tlo, thi) = (a.min(b), a.max(b));
    let (ylo, yhi) = l.to_interval();
    assert!(ylo <= tlo + 1e-12 && yhi >= thi - 1e-12);

    // sin
    let s = x.sin_ctx(&mut ctx);
    let (slo, shi) = s.to_interval();
    let (truth_lo, truth_hi) = (lo.sin().min(hi.sin()), lo.sin().max(hi.sin()));
    assert!(slo <= truth_lo + 1e-12 && shi >= truth_hi - 1e-12);

    // cos
    let c = x.cos_ctx(&mut ctx);
    let (clo, chi) = c.to_interval();
    let (tlo2, thi2) = (lo.cos().min(hi.cos()), lo.cos().max(hi.cos()));
    assert!(clo <= tlo2 + 1e-12 && chi >= thi2 - 1e-12);

    // sqrt
    let y = Affine::from_interval(1.0, 4.0, &mut ctx).sqrt_ctx(&mut ctx);
    let (ylo, yhi) = y.to_interval();
    assert!(ylo <= 1.0 + 1e-12 && yhi >= 2.0 - 1e-12);
}

#[test]
#[should_panic]
fn log_domain_panics() {
    let mut ctx = Ctx::new();
    let x = Affine::from_interval(-0.1, 0.2, &mut ctx);
    let _ = x.log_ctx(&mut ctx);
}
