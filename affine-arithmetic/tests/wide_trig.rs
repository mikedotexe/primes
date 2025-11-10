use affine_arithmetic::{Affine, Ctx};
use core::f64::consts::PI;

#[test]
fn sin_cos_wide_are_full_range() {
    let mut ctx = Ctx::new();
    let x = Affine::from_interval(0.0, 10.0 * PI, &mut ctx);
    let s = x.clone().sin_ctx(&mut ctx);
    let c = x.cos_ctx(&mut ctx);
    let (slo, shi) = s.to_interval();
    let (clo, chi) = c.to_interval();

    // Affine arithmetic is conservative - allow enclosure to be slightly larger
    // The important property is that it must CONTAIN [-1, 1]
    assert!(slo <= -1.0 + 1e-10, "sin lower bound too high: {}", slo);
    assert!(shi >= 1.0 - 1e-10, "sin upper bound too low: {}", shi);
    assert!(clo <= -1.0 + 1e-10, "cos lower bound too high: {}", clo);
    assert!(chi >= 1.0 - 1e-10, "cos upper bound too low: {}", chi);
}
