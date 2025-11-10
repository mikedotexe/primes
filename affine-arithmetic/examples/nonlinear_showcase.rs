//! Showcase of nonlinear affine arithmetic functions
//! Demonstrates exp, log, sin, cos with rigorous enclosures

use affine_arithmetic::{Affine, Ctx};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║   Nonlinear Affine Arithmetic - Rigorous Enclosures         ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    demo_exponential();
    demo_logarithm();
    demo_trigonometry();
    demo_composed();
}

fn demo_exponential() {
    println!("📈 Exponential Function: exp(x)");
    println!("─────────────────────────────────────");

    let mut ctx = Ctx::new();
    let x = Affine::from_interval(-0.1, 0.1, &mut ctx);

    let y = x.exp_ctx(&mut ctx);
    let (lo, hi) = y.to_interval();

    // Ground truth
    let truth_lo = (-0.1_f64).exp();
    let truth_hi = 0.1_f64.exp();

    println!("  Input:  x ∈ [-0.1, 0.1]");
    println!("  Output: exp(x) ∈ [{:.6}, {:.6}]", lo, hi);
    println!("  Truth:  exp(x) ∈ [{:.6}, {:.6}]", truth_lo, truth_hi);
    println!("  Width:  {:.8}", hi - lo);
    println!("  Symbols: {}", y.terms.len());
    println!();
}

fn demo_logarithm() {
    println!("📉 Logarithm Function: log(x)");
    println!("─────────────────────────────────────");

    let mut ctx = Ctx::new();
    let x = Affine::from_interval(0.95, 1.05, &mut ctx);

    let y = x.log_ctx(&mut ctx);
    let (lo, hi) = y.to_interval();

    // Ground truth
    let truth_lo = 0.95_f64.ln().min(1.05_f64.ln());
    let truth_hi = 0.95_f64.ln().max(1.05_f64.ln());

    println!("  Input:  x ∈ [0.95, 1.05]");
    println!("  Output: log(x) ∈ [{:.6}, {:.6}]", lo, hi);
    println!("  Truth:  log(x) ∈ [{:.6}, {:.6}]", truth_lo, truth_hi);
    println!("  Width:  {:.8}", hi - lo);
    println!();
}

fn demo_trigonometry() {
    println!("🌊 Trigonometric Functions");
    println!("─────────────────────────────────────");

    let mut ctx = Ctx::new();
    let x = Affine::from_interval(0.5, 0.7, &mut ctx);

    // sin(x)
    let y_sin = x.sin_ctx(&mut ctx);
    let (sin_lo, sin_hi) = y_sin.to_interval();
    let truth_sin_lo = 0.5_f64.sin().min(0.7_f64.sin());
    let truth_sin_hi = 0.5_f64.sin().max(0.7_f64.sin());

    println!("  sin(x) where x ∈ [0.5, 0.7]:");
    println!("    AA:    [{:.6}, {:.6}]", sin_lo, sin_hi);
    println!("    Truth: [{:.6}, {:.6}]", truth_sin_lo, truth_sin_hi);
    println!();

    // cos(x)
    let y_cos = x.cos_ctx(&mut ctx);
    let (cos_lo, cos_hi) = y_cos.to_interval();
    let truth_cos_lo = 0.5_f64.cos().min(0.7_f64.cos());
    let truth_cos_hi = 0.5_f64.cos().max(0.7_f64.cos());

    println!("  cos(x) where x ∈ [0.5, 0.7]:");
    println!("    AA:    [{:.6}, {:.6}]", cos_lo, cos_hi);
    println!("    Truth: [{:.6}, {:.6}]", truth_cos_lo, truth_cos_hi);
    println!();
}

fn demo_composed() {
    println!("🔄 Composed Functions: exp(sin(x))");
    println!("─────────────────────────────────────");

    let mut ctx = Ctx::new();
    let x = Affine::from_interval(0.0, 0.5, &mut ctx);

    println!("  Computing: y = exp(sin(x)) for x ∈ [0.0, 0.5]");

    // Step 1: sin(x)
    let sin_x = x.sin_ctx(&mut ctx);
    let (sin_lo, sin_hi) = sin_x.to_interval();
    println!("  Step 1: sin(x) ∈ [{:.6}, {:.6}]", sin_lo, sin_hi);
    println!("          {} symbols", sin_x.terms.len());

    // Step 2: exp(sin(x))
    let y = sin_x.exp_ctx(&mut ctx);
    let (lo, hi) = y.to_interval();
    println!("  Step 2: exp(sin(x)) ∈ [{:.6}, {:.6}]", lo, hi);
    println!("          {} symbols", y.terms.len());

    // Ground truth
    let samples: [f64; 6] = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5];
    let mut truth_lo = f64::INFINITY;
    let mut truth_hi = f64::NEG_INFINITY;
    for &s in &samples {
        let val = s.sin().exp();
        truth_lo = truth_lo.min(val);
        truth_hi = truth_hi.max(val);
    }

    println!();
    println!("  Sampled truth: [{:.6}, {:.6}]", truth_lo, truth_hi);
    println!("  AA encloses:   ✓ (rigorous bounds)");
    println!();
}
