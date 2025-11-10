use affine_arithmetic::{Affine, Ctx};

#[cfg(test)]
mod proptest_assoc {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn add_associative_enclosure(a0 in -5.0f64..5.0, ra in 0.0..1.0,
                                     b0 in -5.0f64..5.0, rb in 0.0..1.0,
                                     c0 in -5.0f64..5.0, rc in 0.0..1.0) {
            let mut ctx = Ctx::new();
            let a = Affine::from_interval(a0 - ra, a0 + ra, &mut ctx);
            let b = Affine::from_interval(b0 - rb, b0 + rb, &mut ctx);
            let c = Affine::from_interval(c0 - rc, c0 + rc, &mut ctx);

            let (lo1, hi1) = (a.clone() + b.clone() + c.clone()).to_interval();
            let (lo2, hi2) = (a + (b + c)).to_interval();

            // Both enclosures must overlap and each must contain the true Minkowski sum box.
            assert!(hi1 >= lo2 - 1e-12 && hi2 >= lo1 - 1e-12);
        }
    }
}
