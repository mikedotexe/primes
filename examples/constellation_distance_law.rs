// Constellation Distance Law Analysis
//
// Quantifies the relationship between phase lock distance and membrane success rate.
//
// Data collected:
//   - Twin primes (distance 1): ~40%+ (expected from theory)
//   - Cousin primes (distance 2): ~20% (validated)
//   - Sexy primes (distance 3): ~13% (just tested)
//
// Hypotheses to test:
//   H1: Linear decay: success(d) = a - b×d
//   H2: Inverse: success(d) = k/d
//   H3: Exponential: success(d) = a × exp(-b×d)
//   H4: Power law: success(d) = a × d^(-b)
//
// This analysis will determine which mathematical model best describes
// how constellation success rate depends on phase lock distance.

use num_bigint::BigUint;
use num_traits::Zero;
use prime_physics_engine::is_prime;

#[derive(Debug, Clone)]
struct ConstellationData {
    name: &'static str,
    gap: u32,
    distance: u32,
    example_base: u32,
    example_pair: (u32, u32),
    observed_success: f64,
    num_samples: usize,
}

// Generate membrane for constellation
fn constellation_membrane(left: u32, right: u32, seed: u32, base: u32) -> BigUint {
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    result = result * &base_big + BigUint::from(left);
    result = result * &base_big + BigUint::from(right);
    result = result * &base_big + BigUint::from(seed);
    result = result * &base_big + BigUint::from(right);
    result = result * &base_big + BigUint::from(left);

    result
}

// Quick test of a constellation configuration
fn quick_test(left: u32, right: u32, base: u32, num_seeds: u32) -> f64 {
    let mut primes = 0;
    for seed in 1..=num_seeds {
        let n = constellation_membrane(left, right, seed, base);
        if is_prime(&n) {
            primes += 1;
        }
    }
    (primes as f64) / (num_seeds as f64) * 100.0
}

// Model fitting functions
fn linear_model(distance: f64, a: f64, b: f64) -> f64 {
    a - b * distance
}

fn inverse_model(distance: f64, k: f64) -> f64 {
    k / distance
}

fn exponential_model(distance: f64, a: f64, b: f64) -> f64 {
    a * (-b * distance).exp()
}

fn power_law_model(distance: f64, a: f64, b: f64) -> f64 {
    a * distance.powf(-b)
}

// Simple least squares fit for y = a - b*x
fn fit_linear(distances: &[f64], successes: &[f64]) -> (f64, f64) {
    let n = distances.len() as f64;
    let sum_x: f64 = distances.iter().sum();
    let sum_y: f64 = successes.iter().sum();
    let sum_xx: f64 = distances.iter().map(|x| x * x).sum();
    let sum_xy: f64 = distances.iter().zip(successes).map(|(x, y)| x * y).sum();

    let b = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x);
    let a = (sum_y - b * sum_x) / n;

    (a, b)
}

// Fit y = k/x by minimizing sum of (y - k/x)^2
fn fit_inverse(distances: &[f64], successes: &[f64]) -> f64 {
    let sum_y_over_x: f64 = successes.iter().zip(distances).map(|(y, x)| y / x).sum();
    let sum_inv_x_sq: f64 = distances.iter().map(|x| 1.0 / (x * x)).sum();

    sum_y_over_x / sum_inv_x_sq
}

// Calculate R² (coefficient of determination)
fn r_squared(observed: &[f64], predicted: &[f64]) -> f64 {
    let mean: f64 = observed.iter().sum::<f64>() / observed.len() as f64;

    let ss_tot: f64 = observed.iter().map(|y| (y - mean).powi(2)).sum();
    let ss_res: f64 = observed
        .iter()
        .zip(predicted)
        .map(|(y_obs, y_pred)| (y_obs - y_pred).powi(2))
        .sum();

    1.0 - (ss_res / ss_tot)
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║       CONSTELLATION DISTANCE LAW ANALYSIS                    ║");
    println!("║       Quantifying Success Rate vs Phase Lock Distance        ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    // Known data from tests
    let mut data = vec![
        ConstellationData {
            name: "Twin",
            gap: 2,
            distance: 1,
            example_base: 12,
            example_pair: (5, 7),
            observed_success: 0.0, // Will test
            num_samples: 100,
        },
        ConstellationData {
            name: "Cousin",
            gap: 4,
            distance: 2,
            example_base: 10,
            example_pair: (3, 7),
            observed_success: 20.0, // Known from previous tests
            num_samples: 100,
        },
        ConstellationData {
            name: "Sexy",
            gap: 6,
            distance: 3,
            example_base: 20,
            example_pair: (7, 13),
            observed_success: 13.0, // From previous test
            num_samples: 100,
        },
    ];

    println!("═══════════════════════════════════════════════════════════════");
    println!("DATA COLLECTION");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    // Test twin primes to get empirical data
    println!("Testing twin prime constellation...");
    let twin_rate = quick_test(
        data[0].example_pair.0,
        data[0].example_pair.1,
        data[0].example_base,
        data[0].num_samples as u32,
    );
    data[0].observed_success = twin_rate;
    println!("  Twin (5,7) base 12: {:.1}% success\n", twin_rate);

    // Display collected data
    println!("┌──────────┬─────┬──────────┬─────────────────┬──────────────┐");
    println!("│   Type   │ Gap │ Distance │   Example Pair  │ Success Rate │");
    println!("├──────────┼─────┼──────────┼─────────────────┼──────────────┤");

    for datum in &data {
        println!(
            "│ {:8} │  {:2} │    {:2}    │ ({:2},{:2}) base {:2} │    {:5.1}%    │",
            datum.name,
            datum.gap,
            datum.distance,
            datum.example_pair.0,
            datum.example_pair.1,
            datum.example_base,
            datum.observed_success
        );
    }

    println!("└──────────┴─────┴──────────┴─────────────────┴──────────────┘");
    println!();

    // Extract distances and successes for fitting
    let distances: Vec<f64> = data.iter().map(|d| d.distance as f64).collect();
    let successes: Vec<f64> = data.iter().map(|d| d.observed_success).collect();

    // Model fitting
    println!("═══════════════════════════════════════════════════════════════");
    println!("MODEL FITTING");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    // H1: Linear decay
    let (a_lin, b_lin) = fit_linear(&distances, &successes);
    let predicted_lin: Vec<f64> = distances
        .iter()
        .map(|&d| linear_model(d, a_lin, b_lin))
        .collect();
    let r2_lin = r_squared(&successes, &predicted_lin);

    println!("H1: Linear Decay Model");
    println!("  Formula: success(d) = {:.2} - {:.2}×d", a_lin, b_lin);
    println!("  R² = {:.4}", r2_lin);
    println!("  Predictions:");
    for (i, &d) in distances.iter().enumerate() {
        println!(
            "    d={}: predicted {:.1}%, observed {:.1}%",
            d, predicted_lin[i], successes[i]
        );
    }
    println!();

    // H2: Inverse model
    let k_inv = fit_inverse(&distances, &successes);
    let predicted_inv: Vec<f64> = distances.iter().map(|&d| inverse_model(d, k_inv)).collect();
    let r2_inv = r_squared(&successes, &predicted_inv);

    println!("H2: Inverse Model");
    println!("  Formula: success(d) = {:.2}/d", k_inv);
    println!("  R² = {:.4}", r2_inv);
    println!("  Predictions:");
    for (i, &d) in distances.iter().enumerate() {
        println!(
            "    d={}: predicted {:.1}%, observed {:.1}%",
            d, predicted_inv[i], successes[i]
        );
    }
    println!();

    // H3: Exponential (using rough approximation)
    // log(success) ~ log(a) - b*d
    let log_successes: Vec<f64> = successes.iter().map(|s| s.ln()).collect();
    let (log_a_exp, b_exp) = fit_linear(&distances, &log_successes);
    let a_exp = log_a_exp.exp();
    let predicted_exp: Vec<f64> = distances
        .iter()
        .map(|&d| exponential_model(d, a_exp, b_exp))
        .collect();
    let r2_exp = r_squared(&successes, &predicted_exp);

    println!("H3: Exponential Decay Model");
    println!(
        "  Formula: success(d) = {:.2} × exp(-{:.2}×d)",
        a_exp, b_exp
    );
    println!("  R² = {:.4}", r2_exp);
    println!("  Predictions:");
    for (i, &d) in distances.iter().enumerate() {
        println!(
            "    d={}: predicted {:.1}%, observed {:.1}%",
            d, predicted_exp[i], successes[i]
        );
    }
    println!();

    // H4: Power law (log-log fit)
    let log_distances: Vec<f64> = distances.iter().map(|d| d.ln()).collect();
    let (log_a_pow, b_pow) = fit_linear(&log_distances, &log_successes);
    let a_pow = log_a_pow.exp();
    let predicted_pow: Vec<f64> = distances
        .iter()
        .map(|&d| power_law_model(d, a_pow, -b_pow))
        .collect();
    let r2_pow = r_squared(&successes, &predicted_pow);

    println!("H4: Power Law Model");
    println!("  Formula: success(d) = {:.2} × d^(-{:.2})", a_pow, -b_pow);
    println!("  R² = {:.4}", r2_pow);
    println!("  Predictions:");
    for (i, &d) in distances.iter().enumerate() {
        println!(
            "    d={}: predicted {:.1}%, observed {:.1}%",
            d, predicted_pow[i], successes[i]
        );
    }
    println!();

    // Model comparison
    println!("═══════════════════════════════════════════════════════════════");
    println!("MODEL COMPARISON");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    let models = vec![
        ("Linear", r2_lin),
        ("Inverse", r2_inv),
        ("Exponential", r2_exp),
        ("Power Law", r2_pow),
    ];

    let mut sorted_models = models.clone();
    sorted_models.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("┌────────────────┬─────────┬────────────┐");
    println!("│     Model      │   R²    │   Rank     │");
    println!("├────────────────┼─────────┼────────────┤");

    for (rank, (name, r2)) in sorted_models.iter().enumerate() {
        let rank_str = match rank {
            0 => "★ Best",
            1 => "  2nd ",
            2 => "  3rd ",
            _ => "  4th ",
        };
        println!("│ {:14} │  {:.4}  │ {}   │", name, r2, rank_str);
    }

    println!("└────────────────┴─────────┴────────────┘");
    println!();

    // Determine best model
    let best_model = sorted_models[0].0;
    let best_r2 = sorted_models[0].1;

    println!("═══════════════════════════════════════════════════════════════");
    println!("CONCLUSIONS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("Best fit: {} model (R² = {:.4})", best_model, best_r2);
    println!();

    if best_model == "Linear" {
        println!("The LINEAR decay model best describes the data:");
        println!("  success(d) = {:.2} - {:.2}×d", a_lin, b_lin);
        println!();
        println!("Interpretation:");
        println!("  - Base success rate: ~{:.1}% (intercept)", a_lin);
        println!("  - Each unit of distance costs ~{:.1}% success", b_lin);
        println!(
            "  - At distance 1 (twin): {:.1}%",
            linear_model(1.0, a_lin, b_lin)
        );
        println!(
            "  - At distance 2 (cousin): {:.1}%",
            linear_model(2.0, a_lin, b_lin)
        );
        println!(
            "  - At distance 3 (sexy): {:.1}%",
            linear_model(3.0, a_lin, b_lin)
        );
        println!();
        println!("Extrapolation to larger gaps:");
        println!(
            "  - Distance 4 (gap 8): {:.1}%",
            linear_model(4.0, a_lin, b_lin)
        );
        println!(
            "  - Distance 5 (gap 10): {:.1}%",
            linear_model(5.0, a_lin, b_lin)
        );
        println!(
            "  - Distance 6 (gap 12): {:.1}%",
            linear_model(6.0, a_lin, b_lin)
        );
    } else if best_model == "Inverse" {
        println!("The INVERSE model best describes the data:");
        println!("  success(d) = {:.2}/d", k_inv);
        println!();
        println!("Interpretation:");
        println!("  - Success inversely proportional to distance");
        println!("  - Constant k ≈ {:.1} represents baseline capacity", k_inv);
        println!();
        println!("Extrapolation:");
        println!("  - Distance 4: {:.1}%", inverse_model(4.0, k_inv));
        println!("  - Distance 5: {:.1}%", inverse_model(5.0, k_inv));
        println!("  - Distance 6: {:.1}%", inverse_model(6.0, k_inv));
    } else if best_model == "Exponential" {
        println!("The EXPONENTIAL decay model best describes the data:");
        println!("  success(d) = {:.2} × exp(-{:.2}×d)", a_exp, b_exp);
        println!();
        println!("Interpretation:");
        println!("  - Rapid exponential decay with distance");
        println!("  - Half-life: {:.2} units", 0.693 / b_exp);
        println!();
        println!("Extrapolation:");
        println!(
            "  - Distance 4: {:.1}%",
            exponential_model(4.0, a_exp, b_exp)
        );
        println!(
            "  - Distance 5: {:.1}%",
            exponential_model(5.0, a_exp, b_exp)
        );
        println!(
            "  - Distance 6: {:.1}%",
            exponential_model(6.0, a_exp, b_exp)
        );
    } else {
        println!("The POWER LAW model best describes the data:");
        println!("  success(d) = {:.2} × d^(-{:.2})", a_pow, -b_pow);
        println!();
        println!("Interpretation:");
        println!("  - Scale-free decay (power law)");
        println!("  - Exponent {:.2} indicates decay rate", -b_pow);
        println!();
        println!("Extrapolation:");
        println!(
            "  - Distance 4: {:.1}%",
            power_law_model(4.0, a_pow, -b_pow)
        );
        println!(
            "  - Distance 5: {:.1}%",
            power_law_model(5.0, a_pow, -b_pow)
        );
        println!(
            "  - Distance 6: {:.1}%",
            power_law_model(6.0, a_pow, -b_pow)
        );
    }

    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("TESTABLE PREDICTIONS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!(
        "To validate the {} model, test constellations at:",
        best_model
    );
    println!();
    println!("  Distance 4 (gap 8):");
    println!("    Example: (3, 11) in base 22");
    println!("    Example: (5, 13) in base 26");
    println!();
    println!("  Distance 5 (gap 10):");
    println!("    Example: (3, 13) in base 26");
    println!("    Example: (7, 17) in base 34");
    println!();
    println!("If predictions match observations (within ~20%), the");
    println!(
        "{} law is validated for constellation success rates.",
        best_model
    );
}
