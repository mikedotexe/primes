//! Property-based tests for affine arithmetic soundness using proptest.
//!
//! These tests verify that all operations maintain sound enclosures across
//! random inputs, ensuring the library's core invariants hold universally.

use affine_arithmetic::{Affine, Ctx};
use proptest::prelude::*;

/// Strategy for generating valid intervals [lo, hi] where lo <= hi
fn valid_interval() -> impl Strategy<Value = (f64, f64)> {
    (-100.0_f64..100.0_f64).prop_flat_map(|lo| (Just(lo), lo..=100.0_f64))
}

/// Strategy for positive intervals (for log domain)
fn positive_interval() -> impl Strategy<Value = (f64, f64)> {
    (0.01_f64..100.0_f64).prop_flat_map(|lo| (Just(lo), lo..=100.0_f64))
}

/// Helper: check if affine form's interval encloses [img_lo, img_hi]
fn encloses(img_lo: f64, img_hi: f64, aff: &Affine) -> bool {
    let (alo, ahi) = aff.to_interval();
    // Use relative epsilon for large magnitudes, absolute for small
    let magnitude = img_lo.abs().max(img_hi.abs()).max(1.0);
    let epsilon = magnitude * 1e-10 + 1e-12;
    alo <= img_lo + epsilon && ahi + epsilon >= img_hi
}

proptest! {
    /// Property: Addition preserves enclosures
    #[test]
    fn add_preserves_enclosure((lo1, hi1) in valid_interval(), (lo2, hi2) in valid_interval()) {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(lo1, hi1, &mut ctx);
        let y = Affine::from_interval(lo2, hi2, &mut ctx);
        let z = x + y;

        // Ground truth: sum of intervals
        let truth_lo = lo1 + lo2;
        let truth_hi = hi1 + hi2;

        prop_assert!(encloses(truth_lo, truth_hi, &z));
    }

    /// Property: Subtraction preserves enclosures
    #[test]
    fn sub_preserves_enclosure((lo1, hi1) in valid_interval(), (lo2, hi2) in valid_interval()) {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(lo1, hi1, &mut ctx);
        let y = Affine::from_interval(lo2, hi2, &mut ctx);
        let z = x - y;

        // Ground truth: difference of intervals
        let truth_lo = lo1 - hi2;
        let truth_hi = hi1 - lo2;

        prop_assert!(encloses(truth_lo, truth_hi, &z));
    }

    /// Property: Scalar multiplication preserves enclosures
    #[test]
    fn scalar_mul_preserves_enclosure((lo, hi) in valid_interval(), k in -10.0_f64..10.0) {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(lo, hi, &mut ctx);
        let z = x * k;

        // Ground truth: scaled interval
        let endpoints = [lo * k, hi * k];
        let truth_lo = endpoints[0].min(endpoints[1]);
        let truth_hi = endpoints[0].max(endpoints[1]);

        prop_assert!(encloses(truth_lo, truth_hi, &z));
    }

    /// Property: Multiplication preserves enclosures
    #[test]
    fn mul_preserves_enclosure((lo1, hi1) in valid_interval(), (lo2, hi2) in valid_interval()) {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(lo1, hi1, &mut ctx);
        let y = Affine::from_interval(lo2, hi2, &mut ctx);
        let z = x.mul_ctx(&y, &mut ctx);

        // Ground truth: all endpoint products
        let products = [lo1 * lo2, lo1 * hi2, hi1 * lo2, hi1 * hi2];
        let truth_lo = products.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let truth_hi = products.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        prop_assert!(encloses(truth_lo, truth_hi, &z));
    }

    /// Property: exp preserves enclosures
    #[test]
    fn exp_preserves_enclosure((lo, hi) in (-5.0_f64..5.0).prop_flat_map(|lo| (Just(lo), lo..=5.0))) {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(lo, hi, &mut ctx);
        let y = x.exp_ctx(&mut ctx);

        // Ground truth: exp is monotonic
        let truth_lo = lo.exp();
        let truth_hi = hi.exp();

        prop_assert!(encloses(truth_lo, truth_hi, &y));
    }

    /// Property: log preserves enclosures (positive domain only)
    #[test]
    fn log_preserves_enclosure((lo, hi) in positive_interval()) {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(lo, hi, &mut ctx);
        let y = x.log_ctx(&mut ctx);

        // Ground truth: log is monotonic on positive reals
        let truth_lo = lo.ln();
        let truth_hi = hi.ln();

        prop_assert!(encloses(truth_lo, truth_hi, &y));
    }

    /// Property: sin preserves enclosures (small intervals)
    #[test]
    fn sin_preserves_enclosure((lo, hi) in (-3.14_f64..3.14).prop_flat_map(|lo| (Just(lo), lo..=(lo+1.0).min(3.14)))) {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(lo, hi, &mut ctx);
        let y = x.sin_ctx(&mut ctx);

        // Ground truth: For small intervals or point values, sin is well-behaved
        // For larger intervals, sin can hit extrema at π/2 or 3π/2
        use std::f64::consts::PI;
        let width = hi - lo;

        if width < 0.01 {
            // Small interval: use endpoint sampling
            let truth_lo = lo.sin().min(hi.sin());
            let truth_hi = lo.sin().max(hi.sin());
            prop_assert!(encloses(truth_lo, truth_hi, &y));
        } else {
            // Larger interval: could contain extrema, so we need conservative bounds
            let samples = [lo, (lo + hi) / 2.0, hi];
            let mut truth_lo = f64::INFINITY;
            let mut truth_hi = f64::NEG_INFINITY;
            for &s in &samples {
                let val = s.sin();
                truth_lo = truth_lo.min(val);
                truth_hi = truth_hi.max(val);
            }

            // Check if interval contains extrema
            let contains_max = ((lo - PI/2.0) / (2.0*PI)).ceil() <= ((hi - PI/2.0) / (2.0*PI)).floor();
            let contains_min = ((lo - 3.0*PI/2.0) / (2.0*PI)).ceil() <= ((hi - 3.0*PI/2.0) / (2.0*PI)).floor();

            if contains_max { truth_hi = 1.0; }
            if contains_min { truth_lo = -1.0; }

            prop_assert!(encloses(truth_lo, truth_hi, &y));
        }
    }

    /// Property: cos preserves enclosures (small intervals)
    #[test]
    fn cos_preserves_enclosure((lo, hi) in (-3.14_f64..3.14).prop_flat_map(|lo| (Just(lo), lo..=(lo+1.0).min(3.14)))) {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(lo, hi, &mut ctx);
        let y = x.cos_ctx(&mut ctx);

        // Ground truth: For small intervals or point values, cos is well-behaved
        use std::f64::consts::PI;
        let width = hi - lo;

        if width < 0.01 {
            // Small interval: use endpoint sampling
            let truth_lo = lo.cos().min(hi.cos());
            let truth_hi = lo.cos().max(hi.cos());
            prop_assert!(encloses(truth_lo, truth_hi, &y));
        } else {
            // Larger interval: could contain extrema
            let samples = [lo, (lo + hi) / 2.0, hi];
            let mut truth_lo = f64::INFINITY;
            let mut truth_hi = f64::NEG_INFINITY;
            for &s in &samples {
                let val = s.cos();
                truth_lo = truth_lo.min(val);
                truth_hi = truth_hi.max(val);
            }

            // Check if interval contains extrema (max at 0, min at π)
            let contains_max = (lo / (2.0*PI)).ceil() <= (hi / (2.0*PI)).floor();
            let contains_min = ((lo - PI) / (2.0*PI)).ceil() <= ((hi - PI) / (2.0*PI)).floor();

            if contains_max { truth_hi = 1.0; }
            if contains_min { truth_lo = -1.0; }

            prop_assert!(encloses(truth_lo, truth_hi, &y));
        }
    }

    /// Property: condense maintains enclosure
    #[test]
    fn condense_maintains_enclosure((lo, hi) in valid_interval(), max_terms in 1_usize..10) {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(lo, hi, &mut ctx);

        // Add some noise terms
        let mut y = x.clone();
        for _ in 0..20 {
            let noise = Affine::from_interval(-0.01, 0.01, &mut ctx);
            y = y + noise;
        }

        let (lo_before, hi_before) = y.to_interval();

        // Condense
        y.condense(max_terms, &mut ctx);

        let (lo_after, hi_after) = y.to_interval();

        // Enclosure should be preserved or slightly widened
        prop_assert!(lo_after <= lo_before + 1e-10);
        prop_assert!(hi_after >= hi_before - 1e-10);

        // Term count should be respected
        prop_assert!(y.terms.len() <= max_terms);
    }

    /// Property: Self-subtraction exploits correlation
    #[test]
    fn correlation_self_subtract((lo, hi) in valid_interval()) {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(lo, hi, &mut ctx);
        let y = x.clone();
        let z = x - y;

        // Should be exactly zero (correlation preserved)
        let (zlo, zhi) = z.to_interval();
        let epsilon = 1e-12;
        prop_assert!((zlo.abs() < epsilon) && (zhi.abs() < epsilon));
    }

    /// Property: Composed operations maintain soundness (exp(sin(x)))
    #[test]
    fn composed_exp_sin_sound((lo, hi) in (-1.0_f64..1.0).prop_flat_map(|lo| (Just(lo), lo..=1.0))) {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(lo, hi, &mut ctx);
        let y = x.sin_ctx(&mut ctx).exp_ctx(&mut ctx);

        // Compute true range by first finding sin(x) range, then applying exp
        use std::f64::consts::PI;

        // Step 1: Find range of sin(x) for x ∈ [lo, hi]
        let sin_vals = [lo.sin(), hi.sin()];
        let mut sin_lo = sin_vals[0].min(sin_vals[1]);
        let mut sin_hi = sin_vals[0].max(sin_vals[1]);

        // Check for extrema in the interval
        let contains_sin_max = ((lo - PI/2.0) / (2.0*PI)).ceil() <= ((hi - PI/2.0) / (2.0*PI)).floor();
        let contains_sin_min = ((lo - 3.0*PI/2.0) / (2.0*PI)).ceil() <= ((hi - 3.0*PI/2.0) / (2.0*PI)).floor();

        if contains_sin_max { sin_hi = 1.0; }
        if contains_sin_min { sin_lo = -1.0; }

        // Step 2: Apply exp to the sin range (exp is monotonic)
        let truth_lo = sin_lo.exp();
        let truth_hi = sin_hi.exp();

        prop_assert!(encloses(truth_lo, truth_hi, &y));
    }

    /// Property: sqrt preserves enclosures (positive intervals only)
    #[test]
    fn sqrt_preserves_enclosure((lo, hi) in (0.01_f64..100.0).prop_flat_map(|lo| (Just(lo), lo..=100.0))) {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(lo, hi, &mut ctx);
        let y = x.sqrt_ctx(&mut ctx);

        // Ground truth: sqrt is monotonic for x ≥ 0
        let truth_lo = lo.sqrt();
        let truth_hi = hi.sqrt();

        prop_assert!(encloses(truth_lo, truth_hi, &y));
    }

    /// Property: powi preserves enclosures for various powers
    #[test]
    fn powi_preserves_enclosure((lo, hi) in valid_interval(), n in 0_i32..=5) {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(lo, hi, &mut ctx);
        let y = x.powi_ctx(n, &mut ctx);

        // Ground truth: compute pow at endpoints and find range
        let vals = [lo.powi(n), hi.powi(n)];
        let mut truth_lo = vals[0].min(vals[1]);
        let truth_hi = vals[0].max(vals[1]);

        // For even powers, check if interval contains 0 (minimum point)
        if n % 2 == 0 && n > 0 && lo <= 0.0 && hi >= 0.0 {
            truth_lo = 0.0;
        }

        prop_assert!(encloses(truth_lo, truth_hi, &y));
    }

    /// Property: powi binary exponentiation is efficient (smoke test)
    #[test]
    fn powi_efficiency_smoke((lo, hi) in (1.0_f64..2.0).prop_flat_map(|lo| (Just(lo), lo..=2.0))) {
        let mut ctx = Ctx::new();
        let x = Affine::from_interval(lo, hi, &mut ctx);

        // High power should still work efficiently (binary exponentiation)
        let y = x.powi_ctx(100, &mut ctx);

        // Just verify it produces sound enclosures
        let truth_lo = lo.powi(100);
        let truth_hi = hi.powi(100);

        prop_assert!(encloses(truth_lo, truth_hi, &y));

        // Verify we didn't create too many symbols (should be ~log₂(100) ≈ 7 new symbols)
        // Start with 1 symbol from x, binary exp adds ~log₂(n) symbols
        prop_assert!(y.terms.len() < 20); // Conservative upper bound
    }
}
