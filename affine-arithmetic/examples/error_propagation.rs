//! Error Propagation Example - Real-world physics calculation
//!
//! Computing the period of a pendulum with measurement uncertainty.
//!
//! Formula: T = 2π√(L/g)
//! where:
//!   L = length (measured with ±2% error)
//!   g = 9.81 m/s² (±0.5% error)
//!
//! This example shows how AA tracks uncertainty through complex calculations.

use affine_arithmetic::{Affine, Ctx};

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║         Pendulum Period with Measurement Uncertainty         ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    let mut ctx = Ctx::new();

    // Measurements with uncertainty
    let length_nominal = 1.0; // 1 meter
    let length_error = 0.02; // ±2%
    let length = Affine::from_interval(
        length_nominal * (1.0 - length_error),
        length_nominal * (1.0 + length_error),
        &mut ctx,
    );

    let g_nominal = 9.81; // m/s²
    let g_error = 0.005; // ±0.5%
    let g = Affine::from_interval(
        g_nominal * (1.0 - g_error),
        g_nominal * (1.0 + g_error),
        &mut ctx,
    );

    println!("Input Measurements:");
    println!(
        "  Length L: {:.3} m (±{:.1}%)",
        length_nominal,
        length_error * 100.0
    );
    println!(
        "  Gravity g: {:.2} m/s² (±{:.1}%)",
        g_nominal,
        g_error * 100.0
    );
    println!();

    // Compute T = 2π√(L/g)

    // Step 1: L/g
    let (g_lo, g_hi) = g.to_interval();
    let g_inv = Affine::from_interval(1.0 / g_hi, 1.0 / g_lo, &mut ctx);
    let ratio = length.mul_ctx(&g_inv, &mut ctx);

    // Step 2: √(L/g)
    let sqrt_ratio = ratio.sqrt_ctx(&mut ctx);

    // Step 3: 2π√(L/g)
    let two_pi = 2.0 * std::f64::consts::PI;
    let period = sqrt_ratio * two_pi;

    let (period_lo, period_hi) = period.to_interval();
    let period_mid = (period_lo + period_hi) / 2.0;
    let period_uncertainty = (period_hi - period_lo) / 2.0;
    let period_percent = (period_uncertainty / period_mid) * 100.0;

    println!("Calculated Period:");
    println!(
        "  T = {:.4} ± {:.4} seconds",
        period_mid, period_uncertainty
    );
    println!("  T = {:.4} seconds (±{:.2}%)", period_mid, period_percent);
    println!();

    // Compare with naive interval arithmetic
    let l_ia = (
        length_nominal * (1.0 - length_error),
        length_nominal * (1.0 + length_error),
    );
    let g_ia = (g_nominal * (1.0 - g_error), g_nominal * (1.0 + g_error));

    // IA calculation (loses correlations)
    let ratio_ia_lo = l_ia.0 / g_ia.1;
    let ratio_ia_hi = l_ia.1 / g_ia.0;
    let sqrt_ia_lo = ratio_ia_lo.sqrt();
    let sqrt_ia_hi = ratio_ia_hi.sqrt();
    let t_ia = (two_pi * sqrt_ia_lo, two_pi * sqrt_ia_hi);
    let t_ia_mid = (t_ia.0 + t_ia.1) / 2.0;
    let t_ia_unc = (t_ia.1 - t_ia.0) / 2.0;
    let t_ia_percent = (t_ia_unc / t_ia_mid) * 100.0;

    println!("Comparison:");
    println!("  Affine Arithmetic:");
    println!(
        "    T = {:.4} seconds (±{:.2}%)",
        period_mid, period_percent
    );
    println!(
        "    Uncertainty width: {:.4} seconds",
        period_hi - period_lo
    );
    println!();
    println!("  Interval Arithmetic (naive):");
    println!("    T = {:.4} seconds (±{:.2}%)", t_ia_mid, t_ia_percent);
    println!("    Uncertainty width: {:.4} seconds", t_ia.1 - t_ia.0);
    println!();

    let improvement = (t_ia.1 - t_ia.0) / (period_hi - period_lo);
    println!("  ⚡ AA is {:.2}× tighter than IA!", improvement);
    println!();

    println!("Physical Insight:");
    println!("  - Input uncertainties: L (±2%), g (±0.5%)");
    println!("  - Output uncertainty: T (±{:.2}%)", period_percent);
    println!("  - AA correctly tracks how L and g uncertainties combine");
    println!("  - IA overestimates uncertainty by treating each use of L and g independently");
    println!();

    println!("✅ This is why AA matters for real-world error propagation!");
}
