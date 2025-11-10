//! Demonstrates affine arithmetic vs interval arithmetic
//! Shows how AA tracks correlations to reduce overestimation

use affine_arithmetic::{Affine, Ctx};

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║      Affine Arithmetic vs Interval Arithmetic Demo            ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    demo_correlation_tracking();
    demo_multiplication();
    demo_polynomial();
}

/// Demonstrate correlation tracking: x - x should be 0, not [-width, width]
fn demo_correlation_tracking() {
    println!("📊 Test 1: Correlation Tracking (x - x)");
    println!("─────────────────────────────────────────");

    let mut ctx = Ctx::new();
    let x = Affine::from_interval(1.95, 2.05, &mut ctx);

    // AA: x shares the same noise symbol with itself
    let diff_aa = x.clone() - x.clone();
    let (lo_aa, hi_aa) = diff_aa.to_interval();

    // Naive IA: treats x - x as independent
    let lo_ia = 1.95 - 2.05;
    let hi_ia = 2.05 - 1.95;

    println!("  Input: x ∈ [1.95, 2.05]");
    println!("  Computing: x - x");
    println!();
    println!(
        "  Naive IA:   [{:.2}, {:.2}]  ❌ (spurious uncertainty!)",
        lo_ia, hi_ia
    );
    println!("  AA result:  [{:.2}, {:.2}]  ✓  (exact!)", lo_aa, hi_aa);
    println!("  True value: 0.00\n");
}

/// Demonstrate multiplication with remainder bounds
fn demo_multiplication() {
    println!("📊 Test 2: Multiplication with Error Bounds");
    println!("─────────────────────────────────────────");

    let mut ctx = Ctx::new();
    let x = Affine::from_interval(1.95, 2.05, &mut ctx);
    let y = Affine::from_interval(2.9, 3.1, &mut ctx);

    let product = x.mul_ctx(&y, &mut ctx);
    let (lo, hi) = product.to_interval();

    // Ground truth from endpoint arithmetic
    let truth_lo = 1.95 * 2.9;
    let truth_hi = 2.05 * 3.1;

    println!("  x ∈ [1.95, 2.05]");
    println!("  y ∈ [2.9, 3.1]");
    println!("  Computing: x × y");
    println!();
    println!("  True range:   [{:.4}, {:.4}]", truth_lo, truth_hi);
    println!("  AA enclosure: [{:.4}, {:.4}]", lo, hi);
    println!("  Width:        {:.4}", hi - lo);
    println!(
        "  Symbols used: {} (including remainder)",
        product.terms.len()
    );
    println!();
}

/// Demonstrate polynomial evaluation: (x + 1)²
fn demo_polynomial() {
    println!("📊 Test 3: Polynomial Evaluation (x + 1)²");
    println!("─────────────────────────────────────────");

    let mut ctx = Ctx::new();
    let x = Affine::from_interval(0.95, 1.05, &mut ctx);

    // Compute (x + 1)²
    let x_plus_1 = x.clone() + Affine::cst(1.0);
    let result = x_plus_1.clone().mul_ctx(&x_plus_1, &mut ctx);
    let (lo, hi) = result.to_interval();

    // Ground truth: [(0.95+1)², (1.05+1)²] = [1.95², 2.05²]
    let truth_lo = (0.95_f64 + 1.0).powi(2);
    let truth_hi = (1.05_f64 + 1.0).powi(2);

    // Naive IA expansion: each operation widens independently
    let x_ia_lo = 0.95;
    let x_ia_hi = 1.05;
    let sum_ia_lo = x_ia_lo + 1.0; // 1.95
    let sum_ia_hi = x_ia_hi + 1.0; // 2.05
                                   // Naive squaring: [1.95, 2.05] × [1.95, 2.05]
    let naive_lo = sum_ia_lo * sum_ia_lo;
    let naive_hi = sum_ia_hi * sum_ia_hi;

    println!("  x ∈ [0.95, 1.05]");
    println!("  Computing: (x + 1)²");
    println!();
    println!("  True range:       [{:.4}, {:.4}]", truth_lo, truth_hi);
    println!(
        "  Naive IA:         [{:.4}, {:.4}]  (width: {:.4})",
        naive_lo,
        naive_hi,
        naive_hi - naive_lo
    );
    println!(
        "  AA enclosure:     [{:.4}, {:.4}]  (width: {:.4})",
        lo,
        hi,
        hi - lo
    );
    println!();

    let improvement = ((naive_hi - naive_lo) - (hi - lo)) / (naive_hi - naive_lo) * 100.0;
    println!("  AA tightness improvement: {:.1}%", improvement);
    println!();
}
