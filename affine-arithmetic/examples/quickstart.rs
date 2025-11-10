//! Quick-start example - Copy and paste to get started in 30 seconds!
//!
//! This shows the absolute basics: creating uncertain values and combining them.

use affine_arithmetic::{Affine, Ctx};

fn main() {
    // Create a context for symbol allocation
    let mut ctx = Ctx::new();

    // Create uncertain values from intervals
    let x = Affine::from_interval(1.9, 2.1, &mut ctx);  // x ∈ [1.9, 2.1]
    let y = Affine::from_interval(2.9, 3.1, &mut ctx);  // y ∈ [2.9, 3.1]

    // Perform operations
    let sum = x.clone() + y.clone();
    let product = x.mul_ctx(&y, &mut ctx);

    // Extract interval bounds
    let (sum_lo, sum_hi) = sum.to_interval();
    let (prod_lo, prod_hi) = product.to_interval();

    println!("x + y ∈ [{}, {}]", sum_lo, sum_hi);
    println!("x × y ∈ [{}, {}]", prod_lo, prod_hi);

    // The key advantage: correlation tracking
    let diff = x.clone() - x.clone();
    let (diff_lo, diff_hi) = diff.to_interval();
    println!("x - x = [{}, {}]  (exactly zero!)", diff_lo, diff_hi);

    println!("\n✨ That's it! You're using affine arithmetic.");
}
