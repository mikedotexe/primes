use affine_arithmetic::{Affine, Ctx};

#[test]
fn condense_never_shrinks_enclosure() {
    let mut ctx = Ctx::new();

    // Create an affine form with many small terms
    let mut a = Affine {
        a0: 5.0,
        terms: Vec::new(),
    };
    for _ in 0..50 {
        a.terms.push((ctx.fresh(), 0.01));
    }

    let (lo0, hi0) = a.to_interval();
    let r0 = a.radius_l1();

    // Condense to 10 terms
    let mut a_condensed = a.clone();
    a_condensed.condense(10, &mut ctx);

    let (lo1, hi1) = a_condensed.to_interval();
    let r1 = a_condensed.radius_l1();

    // Radius should not decrease (may stay same or increase conservatively)
    assert!(r1 >= r0 - 1e-12, "radius shrank: {} -> {}", r0, r1);

    // Interval enclosure must not shrink
    assert!(
        lo1 <= lo0 + 1e-12,
        "lower bound increased: {} -> {}",
        lo0,
        lo1
    );
    assert!(
        hi1 >= hi0 - 1e-12,
        "upper bound decreased: {} -> {}",
        hi0,
        hi1
    );
}
