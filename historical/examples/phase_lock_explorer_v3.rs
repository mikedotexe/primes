//! Phase Lock Analysis v3: Seeking the Success Factor
//!
//! v1 RESULT: Total lock count correlation = 0.208 (weak)
//! v2 RESULT: All 4 hypotheses REJECTED (distance, distance=2, coprimality, quality)
//!
//! NEW HYPOTHESES (v3):
//! H5: Base factorization structure (2p vs composite, prime factor count)
//! H6: Absolute digit values (small digits like (1,5) vs large like (13,17))
//! H7: Digit sum/product/ratio patterns
//! H8: Effective seed space size (theoretical capacity)
//!
//! APPROACH: Multi-factor correlation matrix to find which factors matter
//!
//! ## Run
//! ```bash
//! cargo run --example phase_lock_explorer_v3
//! ```

use std::collections::HashMap;

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║    Phase Lock Explorer v3: Multi-Factor Analysis             ║");
    println!("║    Testing structural hypotheses about membrane success       ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // Known membrane success rates
    let known_success: HashMap<u32, f64> = [
        (6, 33.0),   // Champion: (1,5) distance=2
        (10, 18.5),  // Working: (3,7) distance=2
        (12, 26.0),  // (5,7) distance=1
        (14, 27.0),  // (3,11) distance=4
        (18, 24.0),  // (5,13) distance=4
        (30, 30.0),  // (11,19) distance=4 OR (13,17) distance=2
    ]
    .iter()
    .cloned()
    .collect();

    let bases: Vec<u32> = known_success.keys().cloned().collect();

    println!("═══════════════════════════════════════════════════════════════");
    println!("PART 1: Multi-Factor Feature Extraction");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut analysis_data = Vec::new();

    println!("│ Base │ Lock │ 2p? │ #Factors │ DigitSum │ DigitProd │ Ratio │ Success │");
    println!("├──────┼──────┼─────┼──────────┼──────────┼───────────┼───────┼─────────┤");

    for &base in &bases {
        let locks = find_phase_locks(base);
        let success = known_success[&base];

        if let Some((left, right, dist)) = locks.first() {
            // H5: Base factorization structure
            let prime_factors = count_prime_factors(base);
            let is_2p = is_twice_prime(base);

            // H6: Absolute digit values
            let digit_sum = left + right;
            let digit_max = std::cmp::max(*left, *right);
            let digit_min = std::cmp::min(*left, *right);

            // H7: Digit relationships
            let digit_product = left * right;
            let digit_ratio = digit_max as f64 / digit_min as f64;

            // H8: Effective seed space (base^M where M is typical middle length)
            // For now, use base itself as proxy
            let seed_capacity = base as f64;

            let is_2p_str = if is_2p { "✓" } else { " " };

            println!(
                "│ {:4} │ ({:2},{:2}) │  {}  │    {:2}    │    {:3}   │    {:4}   │ {:5.2} │ {:5.1}% │",
                base, left, right, is_2p_str, prime_factors, digit_sum, digit_product, digit_ratio, success
            );

            analysis_data.push((
                base,
                *left,
                *right,
                *dist,
                success,
                is_2p,
                prime_factors as f64,
                digit_sum as f64,
                digit_product as f64,
                digit_ratio,
                seed_capacity,
            ));
        }
    }
    println!("└──────┴──────┴─────┴──────────┴──────────┴───────────┴───────┴─────────┘\n");

    println!("═══════════════════════════════════════════════════════════════");
    println!("PART 2: Correlation Analysis");
    println!("═══════════════════════════════════════════════════════════════\n");

    let successes: Vec<f64> = analysis_data.iter().map(|(_, _, _, _, s, _, _, _, _, _, _)| *s).collect();

    // Extract individual factors
    let is_2p_numeric: Vec<f64> = analysis_data.iter().map(|(_, _, _, _, _, is_2p, _, _, _, _, _)| if *is_2p { 1.0 } else { 0.0 }).collect();
    let prime_factors: Vec<f64> = analysis_data.iter().map(|(_, _, _, _, _, _, pf, _, _, _, _)| *pf).collect();
    let digit_sums: Vec<f64> = analysis_data.iter().map(|(_, _, _, _, _, _, _, ds, _, _, _)| *ds).collect();
    let digit_products: Vec<f64> = analysis_data.iter().map(|(_, _, _, _, _, _, _, _, dp, _, _)| *dp).collect();
    let digit_ratios: Vec<f64> = analysis_data.iter().map(|(_, _, _, _, _, _, _, _, _, dr, _)| *dr).collect();
    let seed_capacities: Vec<f64> = analysis_data.iter().map(|(_, _, _, _, _, _, _, _, _, _, sc)| *sc).collect();

    // Compute correlations
    println!("Correlation with membrane success:\n");

    let corr_2p = correlation(&is_2p_numeric, &successes);
    println!("H5a (is 2p base):          {:+.3}", corr_2p);

    let corr_factors = correlation(&prime_factors, &successes);
    println!("H5b (prime factor count):  {:+.3}", corr_factors);

    let corr_sum = correlation(&digit_sums, &successes);
    println!("H6a (digit sum):           {:+.3}", corr_sum);

    let corr_product = correlation(&digit_products, &successes);
    println!("H6b (digit product):       {:+.3}", corr_product);

    let corr_ratio = correlation(&digit_ratios, &successes);
    println!("H6c (digit ratio):         {:+.3}", corr_ratio);

    let corr_capacity = correlation(&seed_capacities, &successes);
    println!("H7 (seed capacity/base):   {:+.3}", corr_capacity);

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("PART 3: Composite Factor Analysis");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Try composite scores
    let mut composite_small_digits = Vec::new();
    let mut composite_balanced = Vec::new();

    for (_, left, right, _, _, _, _, _, _, ratio, _) in &analysis_data {
        // "Small digit hypothesis": smaller digits = better?
        let max_digit = std::cmp::max(*left, *right);
        composite_small_digits.push(-(max_digit as f64)); // Negative so smaller = higher score

        // "Balanced hypothesis": closer ratio = better?
        composite_balanced.push(-ratio); // Negative so ratio closer to 1.0 = higher score
    }

    let corr_small = correlation(&composite_small_digits, &successes);
    println!("Composite (small digits):  {:+.3}", corr_small);

    let corr_balanced = correlation(&composite_balanced, &successes);
    println!("Composite (balanced):      {:+.3}", corr_balanced);

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("PART 4: Pattern Discovery");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Sort by success and look for patterns
    let mut sorted_data = analysis_data.clone();
    sorted_data.sort_by(|a, b| b.4.partial_cmp(&a.4).unwrap());

    println!("Top 3 performers:\n");
    for (base, left, right, dist, success, is_2p, pf, ds, dp, dr, _) in sorted_data.iter().take(3) {
        println!("  Base {:2}: ({:2},{:2}) → {:.1}%", base, left, right, success);
        println!("    2p={}, factors={:.0}, sum={:.0}, prod={:.0}, ratio={:.2}",
                 if *is_2p { "✓" } else { "✗" }, pf, ds, dp, dr);
        println!("    Distance from midpoint: {}", dist);
        println!();
    }

    println!("Bottom 3 performers:\n");
    for (base, left, right, dist, success, is_2p, pf, ds, dp, dr, _) in sorted_data.iter().rev().take(3) {
        println!("  Base {:2}: ({:2},{:2}) → {:.1}%", base, left, right, success);
        println!("    2p={}, factors={:.0}, sum={:.0}, prod={:.0}, ratio={:.2}",
                 if *is_2p { "✓" } else { "✗" }, pf, ds, dp, dr);
        println!("    Distance from midpoint: {}", dist);
        println!();
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("CONCLUSIONS");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Find strongest correlation
    let correlations = vec![
        ("2p base", corr_2p),
        ("prime factor count", corr_factors),
        ("digit sum", corr_sum),
        ("digit product", corr_product),
        ("digit ratio", corr_ratio),
        ("seed capacity", corr_capacity),
        ("small digits", corr_small),
        ("balanced ratio", corr_balanced),
    ];

    let strongest = correlations
        .iter()
        .max_by(|a, b| a.1.abs().partial_cmp(&b.1.abs()).unwrap())
        .unwrap();

    println!("Strongest correlation: {} ({:+.3})", strongest.0, strongest.1);

    if strongest.1.abs() > 0.5 {
        println!("  → STRONG correlation detected! ✓");
        println!("  → This factor appears predictive of membrane success");
    } else if strongest.1.abs() > 0.3 {
        println!("  → MODERATE correlation");
        println!("  → This factor may have some explanatory power");
    } else {
        println!("  → WEAK correlation");
        println!("  → None of these factors strongly predict success");
        println!("\n🎯 KEY INSIGHT:");
        println!("The success factor may be MORE COMPLEX than these simple metrics.");
        println!("Possibilities:");
        println!("  1. Interaction effects between multiple factors");
        println!("  2. Non-linear relationships (e.g., thresholds)");
        println!("  3. Deeper number-theoretic properties we haven't measured");
        println!("  4. Configuration-specific dynamics (not base properties alone)");
    }

    println!("\n");
}

/// Find all phase locks (p1, p2, distance) for a base, sorted by distance
fn find_phase_locks(base: u32) -> Vec<(u32, u32, u32)> {
    let midpoint = base / 2;
    let mut locks = Vec::new();

    for dist in 1..midpoint {
        let left = midpoint.saturating_sub(dist);
        let right = midpoint + dist;

        if left > 0 && right < base {
            let left_valid = left == 1 || is_prime(left);
            let right_valid = is_prime(right);

            if left_valid && right_valid && left + right == base {
                locks.push((left, right, dist));
            }
        }
    }

    locks.sort_by_key(|(_, _, d)| *d);
    locks
}

/// Simple primality test
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u32;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// Count distinct prime factors
fn count_prime_factors(mut n: u32) -> u32 {
    let mut count = 0;
    let mut d = 2;

    while d * d <= n {
        if n % d == 0 {
            count += 1;
            while n % d == 0 {
                n /= d;
            }
        }
        d += 1;
    }

    if n > 1 {
        count += 1;
    }

    count
}

/// Check if base is of form 2×prime
fn is_twice_prime(n: u32) -> bool {
    if n % 2 != 0 {
        return false;
    }
    let half = n / 2;
    is_prime(half)
}

/// Correlation coefficient
fn correlation(xs: &[f64], ys: &[f64]) -> f64 {
    if xs.len() != ys.len() || xs.is_empty() {
        return 0.0;
    }

    let n = xs.len() as f64;
    let mean_x = xs.iter().sum::<f64>() / n;
    let mean_y = ys.iter().sum::<f64>() / n;

    let mut cov = 0.0;
    let mut var_x = 0.0;
    let mut var_y = 0.0;

    for i in 0..xs.len() {
        let dx = xs[i] - mean_x;
        let dy = ys[i] - mean_y;
        cov += dx * dy;
        var_x += dx * dx;
        var_y += dy * dy;
    }

    if var_x < 1e-10 || var_y < 1e-10 {
        return 0.0;
    }

    cov / (var_x * var_y).sqrt()
}
