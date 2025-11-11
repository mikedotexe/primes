use affine_arithmetic::{Affine, Ctx, Sym};
use proptest::prelude::*;

/// Strategy for generating arbitrary Affine forms with controlled complexity
fn arbitrary_affine(max_terms: usize, max_coeff: f64) -> impl Strategy<Value = (Affine, Ctx)> {
    (
        -100.0..100.0, // a0
        prop::collection::vec(
            (-max_coeff..max_coeff), // coefficients
            0..=max_terms,
        ),
    )
        .prop_map(|(a0, coeffs)| {
            let mut ctx = Ctx::new();
            let terms: Vec<(Sym, f64)> = coeffs
                .into_iter()
                .filter(|&c| c.abs() > 1e-10) // Skip near-zero coeffs
                .map(|c| (ctx.fresh(), c))
                .collect();

            (Affine { a0, terms }, ctx)
        })
}

/// Strategy for smaller affine forms (faster tests)
fn small_affine() -> impl Strategy<Value = (Affine, Ctx)> {
    arbitrary_affine(10, 10.0)
}

/// Strategy for medium affine forms (thorough tests)
fn medium_affine() -> impl Strategy<Value = (Affine, Ctx)> {
    arbitrary_affine(30, 100.0)
}

proptest! {
    /// THE BIG ONE: Condense never shrinks the reachable set
    ///
    /// Mathematical property: For all ε ∈ [-1,1]^n, if value V is reachable
    /// in the original affine form, then V is reachable in the condensed form.
    ///
    /// We verify this by checking interval enclosure:
    ///   [lo_condensed, hi_condensed] ⊇ [lo_original, hi_original]
    #[test]
    fn condense_preserves_all_reachable_values(
        (affine, mut ctx) in medium_affine(),
        max_terms in 1..20usize
    ) {
        let (lo_orig, hi_orig) = affine.to_interval();

        let mut condensed = affine.clone();
        condensed.condense(max_terms, &mut ctx);

        let (lo_cond, hi_cond) = condensed.to_interval();

        // The condensed interval must contain the original
        prop_assert!(
            lo_cond <= lo_orig + 1e-10,
            "Condense shrank lower bound: {} -> {} (original interval: [{}, {}])",
            lo_orig, lo_cond, lo_orig, hi_orig
        );
        prop_assert!(
            hi_cond >= hi_orig - 1e-10,
            "Condense shrank upper bound: {} -> {} (original interval: [{}, {}])",
            hi_orig, hi_cond, lo_orig, hi_orig
        );
    }

    /// Multiplication produces a sound enclosure
    ///
    /// For any two affine forms a and b, the product a*b must enclose
    /// all possible values of the true product.
    #[test]
    fn multiplication_is_sound(
        a0 in -50.0..50.0f64,
        alo_offset in 0.0..10.0f64,
        ahi_offset in 0.0..10.0f64,
        b0 in -50.0..50.0f64,
        blo_offset in 0.0..10.0f64,
        bhi_offset in 0.0..10.0f64,
    ) {
        let mut ctx = Ctx::new();

        // Create two independent affine forms
        let a = Affine::from_interval(a0 - alo_offset, a0 + ahi_offset, &mut ctx);
        let b = Affine::from_interval(b0 - blo_offset, b0 + bhi_offset, &mut ctx);

        let (alo, ahi) = a.to_interval();
        let (blo, bhi) = b.to_interval();

        // True product interval (using interval arithmetic)
        let products = [alo*blo, alo*bhi, ahi*blo, ahi*bhi];
        let true_lo = products.iter().copied().fold(f64::INFINITY, f64::min);
        let true_hi = products.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        // Affine arithmetic product
        let z = a.mul_ctx(&b, &mut ctx);
        let (zlo, zhi) = z.to_interval();

        // Account for both affine arithmetic approximation and FP rounding
        let tolerance = 1e-6 * (true_hi.abs().max(true_lo.abs()).max(1.0));

        prop_assert!(
            zlo <= true_lo + tolerance,
            "Multiplication lower bound too high: {} vs true {} (tol: {})", zlo, true_lo, tolerance
        );
        prop_assert!(
            zhi >= true_hi - tolerance,
            "Multiplication upper bound too low: {} vs true {} (tol: {})", zhi, true_hi, tolerance
        );
    }

    /// Addition is exact (no approximation error)
    ///
    /// Unlike multiplication, addition should produce exact results
    /// with no interval growth beyond the Minkowski sum.
    /// We test with independent noise symbols to avoid correlation effects.
    #[test]
    fn addition_is_exact(
        a0 in -50.0..50.0f64,
        ar in 0.0..10.0f64,
        b0 in -50.0..50.0f64,
        br in 0.0..10.0f64,
    ) {
        let mut ctx = Ctx::new();

        // Create two independent affine forms with different noise symbols
        let a = Affine::from_interval(a0 - ar, a0 + ar, &mut ctx);
        let b = Affine::from_interval(b0 - br, b0 + br, &mut ctx);

        let (alo, ahi) = a.to_interval();
        let (blo, bhi) = b.to_interval();

        let expected_lo = alo + blo;
        let expected_hi = ahi + bhi;

        let z = a + b;
        let (zlo, zhi) = z.to_interval();

        // Should be exact up to floating-point epsilon for simple sums
        let tolerance = 1e-10 * expected_hi.abs().max(1.0);

        prop_assert!(
            (zlo - expected_lo).abs() < tolerance,
            "Addition lower bound inexact: {} vs expected {} (diff: {}, tol: {})",
            zlo, expected_lo, (zlo - expected_lo).abs(), tolerance
        );
        prop_assert!(
            (zhi - expected_hi).abs() < tolerance,
            "Addition upper bound inexact: {} vs expected {} (diff: {}, tol: {})",
            zhi, expected_hi, (zhi - expected_hi).abs(), tolerance
        );
    }

    /// Correlation cancellation in subtraction
    ///
    /// When subtracting an affine form from itself, the result should be
    /// exactly zero (not just a small interval around zero).
    #[test]
    fn self_subtraction_is_exact_zero((affine, _) in small_affine()) {
        let z = affine.clone() - affine;
        let (lo, hi) = z.to_interval();

        prop_assert_eq!(lo, 0.0, "Self-subtraction lower bound not zero");
        prop_assert_eq!(hi, 0.0, "Self-subtraction upper bound not zero");
    }

    /// Scaling preserves interval bounds exactly
    #[test]
    fn scaling_is_exact(
        (affine, _) in small_affine(),
        k in -10.0..10.0f64
    ) {
        let (lo, hi) = affine.to_interval();
        let scaled = affine.scale(k);
        let (slo, shi) = scaled.to_interval();

        let expected_lo = if k >= 0.0 { k * lo } else { k * hi };
        let expected_hi = if k >= 0.0 { k * hi } else { k * lo };

        prop_assert!(
            (slo - expected_lo).abs() < 1e-10,
            "Scaling lower bound inexact: {} vs {}", slo, expected_lo
        );
        prop_assert!(
            (shi - expected_hi).abs() < 1e-10,
            "Scaling upper bound inexact: {} vs {}", shi, expected_hi
        );
    }

    /// Nonlinear operations produce sound enclosures
    #[test]
    fn exp_is_sound((affine, mut ctx) in small_affine()) {
        let (lo, hi) = affine.to_interval();

        // Only test if interval is not too extreme (avoid overflow)
        if lo > -50.0 && hi < 50.0 {
            let y = affine.exp_ctx(&mut ctx);
            let (ylo, yhi) = y.to_interval();

            let true_lo = lo.exp();
            let true_hi = hi.exp();

            prop_assert!(
                ylo <= true_lo + 1e-6,
                "exp lower bound too high: {} vs {}", ylo, true_lo
            );
            prop_assert!(
                yhi >= true_hi - 1e-6,
                "exp upper bound too low: {} vs {}", yhi, true_hi
            );
        }
    }

    /// Prune never shrinks enclosure
    #[test]
    fn prune_preserves_enclosure(
        (affine, mut ctx) in medium_affine(),
        eps in 0.0..5.0f64
    ) {
        let (lo_orig, hi_orig) = affine.to_interval();

        let mut pruned = affine.clone();
        pruned.prune_below(eps, &mut ctx);

        let (lo_pruned, hi_pruned) = pruned.to_interval();

        prop_assert!(
            lo_pruned <= lo_orig + 1e-10,
            "Prune shrank lower bound: {} -> {}", lo_orig, lo_pruned
        );
        prop_assert!(
            hi_pruned >= hi_orig - 1e-10,
            "Prune shrank upper bound: {} -> {}", hi_orig, hi_pruned
        );
    }

    /// Radius is always non-negative
    #[test]
    fn radius_is_nonnegative((affine, _) in small_affine()) {
        let r = affine.radius_l1();
        prop_assert!(r >= 0.0, "Radius is negative: {}", r);
    }

    /// Interval bounds are consistent (lo <= hi)
    #[test]
    fn interval_bounds_are_ordered((affine, _) in small_affine()) {
        let (lo, hi) = affine.to_interval();
        prop_assert!(
            lo <= hi,
            "Interval bounds inverted: [{}, {}]", lo, hi
        );
    }

    /// Zero-radius affine forms are point values
    #[test]
    fn zero_radius_is_point_value(a0 in -100.0..100.0f64) {
        let affine = Affine::cst(a0);
        let (lo, hi) = affine.to_interval();

        prop_assert_eq!(lo, a0, "Constant lower bound wrong");
        prop_assert_eq!(hi, a0, "Constant upper bound wrong");
        prop_assert_eq!(affine.radius_l1(), 0.0, "Constant has nonzero radius");
    }
}

// Additional deterministic edge case tests
#[cfg(test)]
mod edge_cases {
    use super::*;

    #[test]
    fn condense_with_all_equal_coefficients() {
        let mut ctx = Ctx::new();
        let mut a = Affine {
            a0: 0.0,
            terms: Vec::new(),
        };

        // 20 identical coefficients
        for _ in 0..20 {
            a.terms.push((ctx.fresh(), 1.0));
        }

        let (lo0, hi0) = a.to_interval();
        a.condense(5, &mut ctx);
        let (lo1, hi1) = a.to_interval();

        assert!(lo1 <= lo0 + 1e-10);
        assert!(hi1 >= hi0 - 1e-10);
    }

    #[test]
    fn condense_to_single_term() {
        let mut ctx = Ctx::new();
        let mut a = Affine {
            a0: 5.0,
            terms: Vec::new(),
        };

        for i in 0..10 {
            a.terms.push((ctx.fresh(), (i as f64) * 0.1));
        }

        let (lo0, hi0) = a.to_interval();
        a.condense(1, &mut ctx); // Aggressive condensation
        let (lo1, hi1) = a.to_interval();

        assert!(a.terms.len() <= 1);
        assert!(lo1 <= lo0 + 1e-10);
        assert!(hi1 >= hi0 - 1e-10);
    }

    #[test]
    fn multiplication_of_identical_forms() {
        // When multiplying x * x, we should get a sound enclosure of x²
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(2.0, 3.0, &mut ctx);
        let x_squared = x.clone().mul_ctx(&x, &mut ctx);

        let (lo, hi) = x_squared.to_interval();

        // True range is [4.0, 9.0]
        assert!(lo <= 4.0 + 1e-10, "x² lower bound too high: {}", lo);
        assert!(hi >= 9.0 - 1e-10, "x² upper bound too low: {}", hi);
    }
}
