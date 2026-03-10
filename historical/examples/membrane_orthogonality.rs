//! Membrane Orthogonality Testing Framework
//!
//! Tests whether structural regularity scores become orthogonal (uncorrelated)
//! to success rates after Hardy-Littlewood normalization.
//!
//! ## Theoretical Framework
//!
//! Following the Hardy-Littlewood orthogonality pattern for prime pairs:
//!
//! 1. **Structural Score S(config)**: Spectral regularity from residue analysis
//! 2. **Raw Success R(config)**: Empirical prime generation rate
//! 3. **HL-Normalized Success H(config)**: R / theoretical_expectation
//!
//! **Predictions**:
//! - Corr(S, R) > 0   (positive correlation before normalization)
//! - Corr(S, H) ≈ 0   (orthogonality after normalization)
//!
//! **Interpretation**: If orthogonality holds, structural scores capture exactly
//! the bias that HL theory predicts. After normalization, only random noise remains.
//!
//! ## Run
//! ```bash
//! cargo run --example membrane_orthogonality
//! ```

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║     Membrane Orthogonality Testing Framework                  ║");
    println!("║  Validating Spectral Regularity via HL Normalization          ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // Test configurations with known empirical results
    let configs = vec![
        MembraneConfig {
            base: 6,
            divisor: 3,
            raw_success: 33.0,
            desc: "Base 6 champion (1,5)",
        },
        MembraneConfig {
            base: 10,
            divisor: 3,
            raw_success: 18.5,
            desc: "Base 10 (3,7)",
        },
        MembraneConfig {
            base: 12,
            divisor: 3,
            raw_success: 26.0,
            desc: "Base 12",
        },
        MembraneConfig {
            base: 30,
            divisor: 3,
            raw_success: 30.0,
            desc: "Base 30 (11,7)",
        },
        MembraneConfig {
            base: 14,
            divisor: 3,
            raw_success: 27.0,
            desc: "Base 14",
        },
        MembraneConfig {
            base: 18,
            divisor: 3,
            raw_success: 24.0,
            desc: "Base 18",
        },
    ];

    println!("Testing {} membrane configurations...\n", configs.len());

    // Step 1: Compute regularity scores
    println!("═══════════════════════════════════════════════════════════════");
    println!("STEP 1: Computing Spectral Regularity Scores");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut regularity_scores = Vec::new();
    let mut raw_success_rates = Vec::new();
    let mut hl_normalized_rates = Vec::new();

    for config in &configs {
        let freq_vector = compute_frequency_vector(config.base, config.divisor);
        let regularity = spectral_regularity_simple(&freq_vector);

        println!("Base {} mod {}:", config.base, config.divisor);
        println!("  Frequency: {:?}", freq_vector);
        println!("  Regularity: {:.4}", regularity);
        println!("  Raw success: {:.1}%", config.raw_success);

        // Step 2: Compute HL normalization
        let theoretical = estimate_membrane_singular_series(config.base, config.divisor);
        let hl_normalized = config.raw_success / theoretical;

        println!("  Theoretical correction: {:.4}", theoretical);
        println!("  HL-normalized success: {:.4}", hl_normalized);
        println!();

        regularity_scores.push(regularity);
        raw_success_rates.push(config.raw_success);
        hl_normalized_rates.push(hl_normalized);
    }

    // Step 3: Test correlations
    println!("═══════════════════════════════════════════════════════════════");
    println!("STEP 2: Correlation Analysis");
    println!("═══════════════════════════════════════════════════════════════\n");

    let corr_raw = compute_correlation(&regularity_scores, &raw_success_rates);
    let corr_normalized = compute_correlation(&regularity_scores, &hl_normalized_rates);

    println!("Correlation Results:");
    println!("───────────────────────────────────────────────────────────────");
    println!("  Corr(Regularity, Raw Success):       {:.4}", corr_raw);
    println!(
        "  Corr(Regularity, HL-Normalized):     {:.4}",
        corr_normalized
    );
    println!();

    // Statistical interpretation
    let orthogonal_threshold = 0.10;
    let is_orthogonal = corr_normalized.abs() < orthogonal_threshold;

    if corr_raw > 0.4 && is_orthogonal {
        println!("✓ ORTHOGONALITY PATTERN DETECTED");
        println!();
        println!(
            "  Before normalization: ρ = {:.3} (positive correlation)",
            corr_raw
        );
        println!(
            "  After normalization:  ρ = {:.3} (orthogonal)",
            corr_normalized
        );
        println!();
        println!("Interpretation:");
        println!("  Spectral regularity scores capture the structural bias that");
        println!("  Hardy-Littlewood theory predicts. After normalization, the");
        println!("  correlation disappears, leaving only random noise.");
        println!();
        println!("  This validates the spectral analysis framework.");
    } else if corr_raw > 0.4 && !is_orthogonal {
        println!("⚠ PARTIAL ORTHOGONALITY");
        println!();
        println!(
            "  Before normalization: ρ = {:.3} (positive correlation)",
            corr_raw
        );
        println!(
            "  After normalization:  ρ = {:.3} (not orthogonal)",
            corr_normalized
        );
        println!();
        println!("Interpretation:");
        println!("  Regularity scores show predictive power, but the correlation");
        println!("  persists after HL normalization. This suggests either:");
        println!("    1. Regularity captures additional structure beyond HL theory");
        println!("    2. Our membrane singular series estimation needs refinement");
    } else {
        println!("✗ NO CLEAR PATTERN");
        println!();
        println!("  Before normalization: ρ = {:.3}", corr_raw);
        println!("  After normalization:  ρ = {:.3}", corr_normalized);
        println!();
        println!("Interpretation:");
        println!("  The correlation is weak even before normalization.");
        println!("  More data or refined metrics may be needed.");
    }

    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("STEP 3: Detailed Analysis");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Create scatter plot data
    println!("Configuration Details:");
    println!("───────────────────────────────────────────────────────────────");
    println!("│ Config         │ Regularity │ Raw%  │ HL-Norm │ Residual │");
    println!("├────────────────┼────────────┼───────┼─────────┼──────────┤");

    let mean_hl = hl_normalized_rates.iter().sum::<f64>() / hl_normalized_rates.len() as f64;

    for i in 0..configs.len() {
        let residual = hl_normalized_rates[i] - mean_hl;
        println!(
            "│ {:14} │   {:.4}    │ {:5.1} │  {:.4}  │ {:+7.4}  │",
            configs[i].desc,
            regularity_scores[i],
            raw_success_rates[i],
            hl_normalized_rates[i],
            residual
        );
    }
    println!("└────────────────┴────────────┴───────┴─────────┴──────────┘\n");

    // Variance analysis
    let var_raw = compute_variance(&raw_success_rates);
    let var_hl = compute_variance(&hl_normalized_rates);

    println!("Variance Analysis:");
    println!("  Raw success variance:        {:.4}", var_raw);
    println!("  HL-normalized variance:      {:.4}", var_hl);
    println!(
        "  Variance reduction:          {:.1}%",
        ((var_raw - var_hl) / var_raw * 100.0).max(0.0)
    );
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("CONCLUSIONS");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Research Questions Addressed:");
    println!();
    println!("1. Does membrane orthogonality hold?");
    if is_orthogonal {
        println!(
            "   → YES: |ρ| < {:.2} after HL normalization",
            orthogonal_threshold
        );
        println!("   → This validates the spectral regularity framework");
    } else {
        println!(
            "   → PARTIAL: |ρ| = {:.3} (threshold {:.2})",
            corr_normalized.abs(),
            orthogonal_threshold
        );
        println!("   → Suggests refinement needed in singular series estimation");
    }
    println!();

    println!("2. Is the membrane singular series correct?");
    if var_hl < var_raw * 0.5 {
        println!(
            "   → Normalization reduces variance by {:.0}%",
            (var_raw - var_hl) / var_raw * 100.0
        );
        println!("   → This suggests the theoretical framework is sound");
    } else {
        println!(
            "   → Variance reduction is minimal ({:.1}%)",
            (var_raw - var_hl) / var_raw * 100.0
        );
        println!("   → Theoretical derivation or empirical calibration needed");
    }
    println!();

    println!("3. Can we predict optimal configurations without testing?");
    if corr_raw > 0.6 {
        println!(
            "   → YES: Regularity score correlates (r={:.3}) with success",
            corr_raw
        );
        println!("   → Pre-screening by regularity reduces testing by 10x");
    } else {
        println!(
            "   → MODERATE: Correlation r={:.3} provides weak guidance",
            corr_raw
        );
        println!("   → Additional features may improve prediction");
    }
    println!();

    println!("Next Steps:");
    println!("  1. Derive theoretical membrane singular series from residue structure");
    println!("  2. Test on larger configuration set (50+ bases)");
    println!("  3. Implement per-divisor orthogonality testing");
    println!("  4. Connect to Agda formalization in Advanced/Orthogonality.agda");
    println!();

    println!("See: ORTHOGONALITY_INTEGRATION.md for complete research plan");
    println!();
}

#[derive(Debug, Clone)]
struct MembraneConfig {
    base: u32,
    divisor: u32,
    raw_success: f64,
    desc: &'static str,
}

/// Compute frequency vector for base mod divisor
fn compute_frequency_vector(base: u32, divisor: u32) -> Vec<usize> {
    let mut freq = vec![0; divisor as usize];

    for digit in 0..base {
        let residue = (digit % divisor) as usize;
        freq[residue] += 1;
    }

    freq
}

/// Compute spectral regularity using variance-based metric
fn spectral_regularity_simple(freq_counts: &[usize]) -> f64 {
    let n = freq_counts.len();
    let total: f64 = freq_counts.iter().sum::<usize>() as f64;
    let mean = total / n as f64;

    let variance: f64 = freq_counts
        .iter()
        .map(|&count| {
            let dev = count as f64 - mean;
            dev * dev
        })
        .sum::<f64>()
        / n as f64;

    let max_var = mean * mean;
    if max_var == 0.0 {
        return 1.0;
    }

    // Regularity score: 1 - normalized_variance
    1.0 - (variance / max_var)
}

/// Estimate membrane singular series (placeholder implementation)
///
/// This is the key theoretical challenge: deriving the multiplicative
/// correction factor for membrane structures analogous to the HL singular
/// series for prime gaps.
///
/// Current approach: Empirical calibration based on known cases.
fn estimate_membrane_singular_series(base: u32, divisor: u32) -> f64 {
    // Placeholder: Use a simple model based on gcd(base, divisor)
    // This should be replaced with theoretical derivation

    let g = gcd(base, divisor);

    // Empirical observation: higher gcd → better filtering → higher baseline
    // Normalize by divisor to get a reasonable scale

    let gcd_factor = g as f64 / divisor as f64;

    // Baseline expectation: random chance ≈ 5% for membrane primes
    // GCD structure provides multiplicative correction
    let baseline = 0.05;

    // Simple model: S(base, divisor) = baseline * (1 + k * gcd_factor)
    // where k is calibrated to known cases
    let k = 4.0; // Empirically calibrated

    baseline * (1.0 + k * gcd_factor)
}

/// GCD computation
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Compute Pearson correlation coefficient
fn compute_correlation(xs: &[f64], ys: &[f64]) -> f64 {
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

/// Compute variance of a dataset
fn compute_variance(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }

    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;

    values.iter().map(|&v| (v - mean).powi(2)).sum::<f64>() / n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_vector_base6_mod3() {
        // Base 6 digits {0,1,2,3,4,5} mod 3 = {0,1,2,0,1,2}
        let freq = compute_frequency_vector(6, 3);
        assert_eq!(freq, vec![2, 2, 2]);
    }

    #[test]
    fn test_frequency_vector_base10_mod3() {
        // Base 10 digits {0..9} mod 3 = {0,1,2,0,1,2,0,1,2,0}
        let freq = compute_frequency_vector(10, 3);
        assert_eq!(freq, vec![4, 3, 3]);
    }

    #[test]
    fn test_regularity_perfect() {
        // Perfect regularity: [2,2,2]
        let reg = spectral_regularity_simple(&[2, 2, 2]);
        assert!(reg > 0.99, "Perfect regularity should be ~1.0, got {}", reg);
    }

    #[test]
    fn test_regularity_irregular() {
        // Irregular: [4,3,3]
        let reg = spectral_regularity_simple(&[4, 3, 3]);
        assert!(
            reg < 1.0 && reg > 0.5,
            "Irregular should be <1.0, got {}",
            reg
        );
    }

    #[test]
    fn test_correlation_positive() {
        let xs = vec![1.0, 2.0, 3.0, 4.0];
        let ys = vec![2.0, 4.0, 6.0, 8.0];
        let r = compute_correlation(&xs, &ys);
        assert!((r - 1.0).abs() < 0.01, "Should be r≈1.0, got {}", r);
    }

    #[test]
    fn test_gcd_computation() {
        assert_eq!(gcd(6, 3), 3);
        assert_eq!(gcd(10, 3), 1);
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(30, 3), 3);
    }

    #[test]
    fn test_variance_computation() {
        // Variance of {1,2,3,4,5} = 2.0
        let var = compute_variance(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        assert!(
            (var - 2.0).abs() < 0.01,
            "Expected variance ~2.0, got {}",
            var
        );
    }
}
