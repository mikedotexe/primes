use affine_arithmetic::{Affine, Ctx};

#[test]
fn sqrt_hybrid_at_zero_matches_regular() {
    let mut ctx = Ctx::new();
    let x = Affine::from_interval(0.0, 4.0, &mut ctx);

    let y1 = x.clone().sqrt_ctx(&mut ctx);
    let y2 = x.sqrt_hybrid(&mut ctx, 10);

    let (lo1, hi1) = y1.to_interval();
    let (lo2, hi2) = y2.to_interval();

    // Both should enclose [0, 2]
    assert!(lo1 <= 1e-12);
    assert!(hi1 >= 2.0 - 1e-12);
    assert!(lo2 <= 1e-12);
    assert!(hi2 >= 2.0 - 1e-12);

    // Enclosures should be similar (within reasonable bounds)
    assert!((lo1 - lo2).abs() < 0.1);
    assert!((hi1 - hi2).abs() < 0.1);
}
