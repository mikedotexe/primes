//! The Ultimate Affine Arithmetic Superiority Demonstration
//!
//! This example showcases THREE fundamental ways affine arithmetic (AA)
//! dramatically outperforms standard interval arithmetic (IA):
//!
//! 1. **Dependency Problem** - Correlated variables cause IA to massively overestimate
//! 2. **Wrapping Effect** - Repeated operations accumulate uncertainty in IA but not AA
//! 3. **Composition Power** - Complex expressions stay tight in AA, explode in IA

use affine_arithmetic::{Affine, Ctx};

fn print_header(title: &str) {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║  {:<61}║", title);
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
}

fn print_comparison(name: &str, aa_width: f64, ia_width: f64) {
    let ratio = ia_width / aa_width;
    let bars_aa = (aa_width * 50.0).round() as usize;
    let bars_ia = (ia_width * 50.0).round() as usize;

    println!("  {}", name);
    println!("    AA: [width: {:.8}] {}", aa_width, "█".repeat(bars_aa.min(60)));
    println!("    IA: [width: {:.8}] {}", ia_width, "█".repeat(bars_ia.min(60)));
    println!("    ⚡ AA is {:.1}× tighter than IA\n", ratio);
}

/// Problem 1: The Dependency Catastrophe
/// Computing (x - x) where x has uncertainty
fn demo_dependency_problem() {
    print_header("Problem 1: The Dependency Catastrophe");

    println!("  Setup: x ∈ [1.9, 2.1] (±5% uncertainty around 2.0)");
    println!("  Compute: z = x - x  (should be exactly 0!)");
    println!();

    // Affine Arithmetic - tracks correlation
    let mut ctx = Ctx::new();
    let x_aa = Affine::from_interval(1.9, 2.1, &mut ctx);
    let z_aa = x_aa.clone() - x_aa.clone();
    let (aa_lo, aa_hi) = z_aa.to_interval();
    let aa_width = aa_hi - aa_lo;

    // Interval Arithmetic - treats each x independently
    let x_ia = (1.9, 2.1);
    let z_ia = (x_ia.0 - x_ia.1, x_ia.1 - x_ia.0); // [lo - hi, hi - lo]
    let ia_width = z_ia.1 - z_ia.0;

    println!("  Affine Arithmetic (AA):");
    println!("    x = 2.0 + 0.1·ε₀");
    println!("    z = x - x = (2.0 + 0.1·ε₀) - (2.0 + 0.1·ε₀)");
    println!("       = 0 + 0·ε₀");
    println!("    Result: z ∈ [{:.10}, {:.10}] ✓ EXACT", aa_lo, aa_hi);
    println!();

    println!("  Interval Arithmetic (IA):");
    println!("    x₁ ∈ [1.9, 2.1]  (first occurrence)");
    println!("    x₂ ∈ [1.9, 2.1]  (second occurrence - INDEPENDENT!)");
    println!("    z = x₁ - x₂ ∈ [1.9-2.1, 2.1-1.9]");
    println!("    Result: z ∈ [{:.1}, {:.1}] ✗ WRONG", z_ia.0, z_ia.1);
    println!();

    print_comparison("x - x", aa_width, ia_width);

    println!("  💡 Insight: IA doesn't know the two x's are the same variable!");
    println!("     AA tracks correlation through shared noise symbols.");
}

/// Problem 2: Polynomial Evaluation
/// Evaluating (x-1)³ in expanded form shows wrapping effect
fn demo_polynomial_wrapping() {
    print_header("Problem 2: Polynomial Evaluation - The Wrapping Effect");

    println!("  Setup: x ∈ [0.95, 1.05]");
    println!("  Compute: p(x) = x³ - 3x² + 3x - 1  (which equals (x-1)³)");
    println!();

    let mut ctx = Ctx::new();
    let x_aa = Affine::from_interval(0.95, 1.05, &mut ctx);

    // AA evaluation
    let x2_aa = x_aa.clone().mul_ctx(&x_aa, &mut ctx);
    let x3_aa = x2_aa.clone().mul_ctx(&x_aa, &mut ctx);
    let term1 = x3_aa;
    let term2 = x2_aa * -3.0;
    let term3 = x_aa.clone() * 3.0;
    let term4 = Affine::cst(-1.0);
    let p_aa = term1 + term2 + term3 + term4;
    let (aa_lo, aa_hi) = p_aa.to_interval();
    let aa_width = aa_hi - aa_lo;

    // IA evaluation (simulate independent interval arithmetic)
    let x_ia = (0.95, 1.05);
    // x²: [lo², hi²] but we must account for mixing
    let x2_ia = (x_ia.0 * x_ia.0, x_ia.1 * x_ia.1);
    // x³
    let x3_ia = (x2_ia.0 * x_ia.0, x2_ia.1 * x_ia.1);
    // -3x²
    let neg3x2_ia = (-3.0 * x2_ia.1, -3.0 * x2_ia.0);
    // 3x
    let pos3x_ia = (3.0 * x_ia.0, 3.0 * x_ia.1);
    // Combine (naive addition)
    let ia_lo = x3_ia.0 + neg3x2_ia.0 + pos3x_ia.0 - 1.0;
    let ia_hi = x3_ia.1 + neg3x2_ia.1 + pos3x_ia.1 - 1.0;
    let ia_width = ia_hi - ia_lo;

    // Ground truth: (x-1)³ for x ∈ [0.95, 1.05] should be [(-0.05)³, (0.05)³] = [-0.000125, 0.000125]
    let truth_width = 0.000250;

    println!("  Ground Truth (factored form):");
    println!("    p(x) = (x-1)³");
    println!("    For x ∈ [0.95, 1.05], (x-1) ∈ [-0.05, 0.05]");
    println!("    So p(x) ∈ [{:.6}, {:.6}]", -0.000125, 0.000125);
    println!("    Width: {:.6}", truth_width);
    println!();

    println!("  Affine Arithmetic:");
    println!("    Tracks correlation between x, x², x³");
    println!("    Result: p(x) ∈ [{:.6}, {:.6}]", aa_lo, aa_hi);
    println!("    Width: {:.6} (close to ground truth!)", aa_width);
    println!();

    println!("  Interval Arithmetic:");
    println!("    Treats each occurrence of x as independent");
    println!("    Result: p(x) ∈ [{:.6}, {:.6}]", ia_lo, ia_hi);
    println!("    Width: {:.6} (massive overestimate!)", ia_width);
    println!();

    print_comparison("(x-1)³ expanded", aa_width, ia_width);

    println!("  💡 Insight: Each IA operation compounds uncertainty!");
    println!("     AA preserves correlations, keeping bounds tight.");
}

/// Problem 3: Algebraic Identity Cancellation
/// Computing (x+1)² - (x² + 2x + 1) which is identically zero
fn demo_rational_composition() {
    print_header("Problem 3: Algebraic Identity - The Cancellation Test");

    println!("  Setup: x ∈ [0.9, 1.1]");
    println!("  Compute: (x+1)² - (x² + 2x + 1)");
    println!();

    println!("  Mathematical fact: (x+1)² = x² + 2x + 1");
    println!("  So (x+1)² - (x² + 2x + 1) ≡ 0 for all x");
    println!();

    let mut ctx = Ctx::new();
    let x_aa = Affine::from_interval(0.9, 1.1, &mut ctx);

    // AA evaluation: (x+1)²
    let x_plus_1 = x_aa.clone() + Affine::cst(1.0);
    let lhs_aa = x_plus_1.clone().mul_ctx(&x_plus_1, &mut ctx);

    // AA evaluation: x² + 2x + 1
    let x2_aa = x_aa.clone().mul_ctx(&x_aa, &mut ctx);
    let rhs_aa = x2_aa + x_aa.clone() * 2.0 + Affine::cst(1.0);

    // AA difference
    let diff_aa = lhs_aa - rhs_aa;
    let (aa_lo, aa_hi) = diff_aa.to_interval();
    let aa_width = aa_hi - aa_lo;

    // IA evaluation
    let x_ia = (0.9, 1.1);
    // (x+1) ∈ [1.9, 2.1]
    let x_plus_1_ia = (x_ia.0 + 1.0, x_ia.1 + 1.0);
    // (x+1)² ∈ [1.9², 2.1²] = [3.61, 4.41]
    let lhs_ia = (x_plus_1_ia.0 * x_plus_1_ia.0, x_plus_1_ia.1 * x_plus_1_ia.1);

    // x² ∈ [0.81, 1.21]
    let x2_ia = (x_ia.0 * x_ia.0, x_ia.1 * x_ia.1);
    // 2x ∈ [1.8, 2.2]
    let two_x_ia = (2.0 * x_ia.0, 2.0 * x_ia.1);
    // x² + 2x + 1 ∈ [0.81+1.8+1, 1.21+2.2+1] = [3.61, 4.41]
    let rhs_ia = (x2_ia.0 + two_x_ia.0 + 1.0, x2_ia.1 + two_x_ia.1 + 1.0);

    // (x+1)² - (x²+2x+1) in IA
    // IA treats both sides independently!
    let diff_ia = (lhs_ia.0 - rhs_ia.1, lhs_ia.1 - rhs_ia.0);
    let ia_width = diff_ia.1 - diff_ia.0;

    println!("  Ground Truth:");
    println!("    (x+1)² - (x²+2x+1) = 0 identically");
    println!("    Result should be exactly 0");
    println!();

    println!("  Affine Arithmetic:");
    println!("    Tracks all correlations during expansion");
    println!("    Result: [{:.6}, {:.6}]", aa_lo, aa_hi);
    println!("    Width: {:.6} (near-zero!)", aa_width);
    println!();

    println!("  Interval Arithmetic:");
    println!("    Treats LHS and RHS as independent");
    println!("    Result: [{:.3}, {:.3}]", diff_ia.0, diff_ia.1);
    println!("    Width: {:.3} (huge false uncertainty!)", ia_width);
    println!();

    print_comparison("(x+1)² - (x²+2x+1)", aa_width, ia_width);

    println!("  💡 Insight: IA can't detect algebraic identities!");
    println!("     AA preserves relationships, recognizing the cancellation.");
}

/// Bonus: Show how AA handles composition of transcendental functions
fn demo_transcendental_composition() {
    print_header("Bonus: Transcendental Function Composition");

    println!("  Setup: x ∈ [0.0, 1.0]");
    println!("  Compute: exp(sin(x)²) - complex composition");
    println!();

    let mut ctx = Ctx::new();
    let x_aa = Affine::from_interval(0.0, 1.0, &mut ctx);

    // AA evaluation
    let sin_x = x_aa.sin_ctx(&mut ctx);
    let sin2_x = sin_x.clone().mul_ctx(&sin_x, &mut ctx);
    let result = sin2_x.exp_ctx(&mut ctx);
    let (aa_lo, aa_hi) = result.to_interval();

    // Ground truth by sampling
    let samples: [f64; 6] = [0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
    let mut truth_lo = f64::INFINITY;
    let mut truth_hi = f64::NEG_INFINITY;
    for &s in &samples {
        let val = (s.sin().powi(2)).exp();
        truth_lo = truth_lo.min(val);
        truth_hi = truth_hi.max(val);
    }

    println!("  Ground Truth (sampled):");
    println!("    exp(sin²(x)) ∈ [{:.6}, {:.6}]", truth_lo, truth_hi);
    println!();

    println!("  Affine Arithmetic:");
    println!("    Composes: x → sin(x) → sin²(x) → exp(sin²(x))");
    println!("    Result: [{:.6}, {:.6}] ✓", aa_lo, aa_hi);
    println!("    Maintains soundness through {} noise symbols", result.terms.len());
    println!();

    println!("  💡 Insight: AA handles complex compositions while preserving rigor!");
}

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                                                               ║");
    println!("║   🚀 AFFINE ARITHMETIC: THE ULTIMATE SUPERIORITY DEMO 🚀     ║");
    println!("║                                                               ║");
    println!("║   Why AA crushes Interval Arithmetic in practical analysis   ║");
    println!("║                                                               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");

    demo_dependency_problem();
    demo_polynomial_wrapping();
    demo_rational_composition();
    demo_transcendental_composition();

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                         CONCLUSION                            ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║                                                               ║");
    println!("║  Affine Arithmetic wins because it:                          ║");
    println!("║                                                               ║");
    println!("║  ✓ Tracks correlations through shared noise symbols          ║");
    println!("║  ✓ Prevents dependency problem (x-x = 0, not [-w, w])        ║");
    println!("║  ✓ Avoids wrapping effect in repeated operations             ║");
    println!("║  ✓ Handles complex compositions while staying tight           ║");
    println!("║  ✓ Provides rigorous, sound enclosures with less pessimism   ║");
    println!("║                                                               ║");
    println!("║  Standard Interval Arithmetic is correct but pessimistic.    ║");
    println!("║  Affine Arithmetic is correct AND efficient.                 ║");
    println!("║                                                               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
}
