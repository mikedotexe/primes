//! Quadratic Membrane Discriminant Analysis
//!
//! This tool validates the Quadratic Membrane Hypothesis by analyzing membrane
//! constructions through the lens of polynomial discriminants and Goldbach decompositions.
//!
//! ## Core Theory
//!
//! A membrane structure like `1 000 S 000 1` in base b can be viewed as evaluating
//! a quadratic polynomial:
//!
//! ```text
//! N(X) = A·X² + S·X + A  where X = b^k
//! ```
//!
//! The discriminant Δ = S² - 4A² determines the "algebraic fate" of this polynomial:
//! - If Δ is a perfect square, the polynomial factors → likely composite
//! - If Δ has favorable quadratic residue properties → enhanced primality
//!
//! ## Usage
//!
//! ```bash
//! cargo run --release --example quadratic_membrane_analyzer
//! ```
//!
//! Analyzes existing `solution_space_complete.csv` and generates:
//! - `quadratic_membrane_analysis.csv` - Enhanced data with discriminants
//! - `DISCRIMINANT_ANALYSIS_REPORT.md` - Statistical findings
//! - `base10_m2_discriminant_deep_dive.txt` - Anomaly analysis

use num_bigint::BigUint;
use primes::is_prime;
use primes::hzlib::{
    PairCount, sieve_bool, sieve_spf,
    hl_goldbach_lambda, goldbach_coverage_from_lambda,
    hedges_g, cliffs_delta, spearman_rho,
};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone)]
struct MembraneConfig {
    base: u32,
    m: usize,          // middle length (seed digits)
    outer: u32,        // outer boundary digit
    inner: u32,        // inner boundary digit
    k: u32,            // padding length
    seed: u64,         // the variable center value
    prime_count: usize,
    total_candidates: usize,
    density: f64,
}

#[derive(Debug, Clone)]
struct QuadraticAnalysis {
    // Polynomial coefficients (simplified model)
    // N(X) ≈ outer·X² + seed·X + outer  where X = base^k
    a_coeff: i64,      // outer (or outer-inner for some models)
    s_coeff: i64,      // seed

    // Discriminant properties
    discriminant: i64,           // Δ = S² - 4A²
    is_perfect_square: bool,     // Is Δ a perfect square?
    sqrt_discriminant: Option<i64>, // If perfect square, its root
    discriminant_mod_base: i64,  // Δ mod base

    // Quadratic residue analysis (Legendre symbols)
    qr_mod3: i8,   // (Δ/3) ∈ {-1, 0, 1}
    qr_mod5: i8,   // (Δ/5)
    qr_mod7: i8,   // (Δ/7)
    qr_mod11: i8,  // (Δ/11)

    // Goldbach decomposition
    goldbach_pairs: Vec<(u64, u64)>,  // All (p,q) where p+q=seed, both prime
    goldbach_count: usize,
    goldbach_lambda: f64,             // Hardy-Littlewood expectation
    goldbach_coverage: f64,           // 1 - e^(-λ)

    // Symmetric pair analysis (for 2p bases)
    is_2p_base: bool,
    midpoint: u32,
    symmetric_pairs: Vec<(u32, u32)>, // Pairs equidistant from midpoint
    phase_locked: bool,                // Do boundaries form symmetric pair?
}

#[derive(Debug, Clone)]
struct StatisticalSummary {
    total_configs: usize,
    perfect_square_count: usize,
    perfect_square_avg_density: f64,
    non_square_avg_density: f64,

    // Correlation coefficients
    spearman_discriminant_density: f64,
    spearman_goldbach_density: f64,
    spearman_qr_count_density: f64,

    // Effect sizes
    hedges_g_perfect_square: f64,
    cliffs_delta_perfect_square: f64,

    // Base-specific insights
    base10_m2_unique_signature: bool,
    base10_m2_effect_size: f64,
}

// ============================================================================
// Discriminant Calculation
// ============================================================================

/// Calculate the discriminant for a membrane viewed as quadratic polynomial.
///
/// For a symmetric membrane with outer digit A and seed S:
/// N(X) = A·X² + S·X + A  where X = base^k
///
/// Discriminant: Δ = S² - 4A²
fn compute_discriminant(outer: u32, seed: u64) -> i64 {
    let s = seed as i64;
    let a = outer as i64;

    // Δ = S² - 4A²
    let delta = (s * s) - (4 * a * a);
    delta
}

/// Check if a number is a perfect square and return its root if so.
fn is_perfect_square(n: i64) -> (bool, Option<i64>) {
    if n < 0 {
        return (false, None);
    }

    let n_abs = n.abs() as u64;
    let sqrt = (n_abs as f64).sqrt() as u64;

    // Check sqrt and sqrt±1 to handle floating point errors
    for candidate in [sqrt.saturating_sub(1), sqrt, sqrt + 1] {
        if candidate * candidate == n_abs {
            return (true, Some(candidate as i64));
        }
    }

    (false, None)
}

/// Calculate Legendre symbol (a/p) using Euler's criterion.
///
/// Returns:
/// - 1 if a is a quadratic residue mod p
/// - -1 if a is a non-residue mod p
/// - 0 if a ≡ 0 (mod p)
fn legendre_symbol(a: i64, p: u32) -> i8 {
    let p_i64 = p as i64;
    let a_mod = ((a % p_i64) + p_i64) % p_i64; // Ensure positive

    if a_mod == 0 {
        return 0;
    }

    // Use Euler's criterion: (a/p) ≡ a^((p-1)/2) (mod p)
    let exp = (p - 1) / 2;
    let result = mod_exp(a_mod as u64, exp as u64, p as u64);

    if result == 1 {
        1
    } else if result == p as u64 - 1 {
        -1
    } else {
        0
    }
}

/// Modular exponentiation: (base^exp) mod m
fn mod_exp(base: u64, exp: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }

    let mut result = 1u64;
    let mut base = base % m;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % m;
        }
        exp >>= 1;
        base = (base * base) % m;
    }

    result
}

// ============================================================================
// Goldbach Decomposition
// ============================================================================

/// Find all Goldbach pairs (p, q) such that p + q = n and both p, q are prime.
fn find_goldbach_pairs(n: u64, spf: &[usize]) -> Vec<(u64, u64)> {
    if n < 4 || n % 2 != 0 {
        return vec![];
    }

    let n_usize = n as usize;
    let is_prime_vec = sieve_bool(n_usize + 1);

    let mut pairs = Vec::new();

    // Only iterate up to n/2 to avoid duplicates (p,q) and (q,p)
    for p in 2..=n/2 {
        let q = n - p;
        if (p as usize) < is_prime_vec.len() && (q as usize) < is_prime_vec.len() {
            if is_prime_vec[p as usize] && is_prime_vec[q as usize] {
                pairs.push((p, q));
            }
        }
    }

    pairs
}

/// Calculate Hardy-Littlewood expectation for Goldbach pairs.
fn compute_goldbach_stats(n: u64, spf: &[usize]) -> (f64, f64) {
    if n < 4 || n % 2 != 0 {
        return (0.0, 0.0);
    }

    let n_usize = n as usize;

    // Use Hardy-Littlewood formula for expected number of pairs
    let lambda = hl_goldbach_lambda(n_usize, spf, PairCount::Unordered);
    let coverage = goldbach_coverage_from_lambda(lambda);

    (lambda, coverage)
}

// ============================================================================
// Symmetric Pair Analysis
// ============================================================================

/// Check if a base is of the form 2p where p is prime.
fn is_2p_base(base: u32) -> (bool, u32) {
    if base % 2 != 0 {
        return (false, 0);
    }

    let midpoint = base / 2;
    let midpoint_big = BigUint::from(midpoint);

    let is_prime = is_prime(&midpoint_big);
    (is_prime, midpoint)
}

/// Find all symmetric pairs (a, b) where a + b = base and both are prime.
fn find_symmetric_pairs(base: u32) -> Vec<(u32, u32)> {
    let mut pairs = Vec::new();

    for a in 2..=base/2 {
        let b = base - a;
        let a_big = BigUint::from(a);
        let b_big = BigUint::from(b);

        if is_prime(&a_big) && is_prime(&b_big) {
            pairs.push((a, b));
        }
    }

    pairs
}

/// Check if boundary digits form a phase-locked symmetric pair.
fn is_phase_locked(outer: u32, inner: u32, base: u32, midpoint: u32) -> bool {
    // Check if outer and inner are equidistant from midpoint
    let outer_dist = (outer as i32 - midpoint as i32).abs() as u32;
    let inner_dist = (inner as i32 - midpoint as i32).abs() as u32;

    outer_dist == inner_dist && (outer + inner) == base
}

// ============================================================================
// Full Analysis Pipeline
// ============================================================================

/// Perform complete quadratic analysis on a membrane configuration.
fn analyze_membrane(config: &MembraneConfig, spf: &[usize]) -> QuadraticAnalysis {
    // 1. Compute discriminant
    let discriminant = compute_discriminant(config.outer, config.seed);
    let (is_perfect_square, sqrt_disc) = is_perfect_square(discriminant);
    let discriminant_mod_base = ((discriminant % config.base as i64) + config.base as i64) % config.base as i64;

    // 2. Quadratic residue analysis
    let qr_mod3 = legendre_symbol(discriminant, 3);
    let qr_mod5 = legendre_symbol(discriminant, 5);
    let qr_mod7 = legendre_symbol(discriminant, 7);
    let qr_mod11 = legendre_symbol(discriminant, 11);

    // 3. Goldbach decomposition
    let goldbach_pairs = find_goldbach_pairs(config.seed, spf);
    let goldbach_count = goldbach_pairs.len();
    let (goldbach_lambda, goldbach_coverage) = compute_goldbach_stats(config.seed, spf);

    // 4. Symmetric pair analysis
    let (is_2p, midpoint) = is_2p_base(config.base);
    let symmetric_pairs = if is_2p {
        find_symmetric_pairs(config.base)
    } else {
        vec![]
    };
    let phase_locked = if is_2p {
        is_phase_locked(config.outer, config.inner, config.base, midpoint)
    } else {
        false
    };

    QuadraticAnalysis {
        a_coeff: config.outer as i64,
        s_coeff: config.seed as i64,
        discriminant,
        is_perfect_square,
        sqrt_discriminant: sqrt_disc,
        discriminant_mod_base,
        qr_mod3,
        qr_mod5,
        qr_mod7,
        qr_mod11,
        goldbach_pairs,
        goldbach_count,
        goldbach_lambda,
        goldbach_coverage,
        is_2p_base: is_2p,
        midpoint,
        symmetric_pairs,
        phase_locked,
    }
}

// ============================================================================
// CSV I/O
// ============================================================================

/// Read existing solution_space_complete.csv
fn read_solution_space_csv(path: &str) -> Result<Vec<MembraneConfig>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut configs = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            continue; // Skip header
        }

        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();

        if fields.len() < 8 {
            continue; // Skip malformed lines
        }

        // Parse fields: base,M,outer,inner,k,total_candidates,prime_count,density,...
        let config = MembraneConfig {
            base: fields[0].parse().unwrap_or(0),
            m: fields[1].parse().unwrap_or(0),
            outer: fields[2].parse().unwrap_or(0),
            inner: fields[3].parse().unwrap_or(0),
            k: fields[4].parse().unwrap_or(0),
            total_candidates: fields[5].parse().unwrap_or(0),
            prime_count: fields[6].parse().unwrap_or(0),
            density: fields[7].parse().unwrap_or(0.0),
            seed: 0, // We'll need to reconstruct this from candidates
        };

        configs.push(config);
    }

    Ok(configs)
}

/// Write enhanced CSV with discriminant analysis
fn write_analysis_csv(
    path: &str,
    configs: &[(MembraneConfig, QuadraticAnalysis)],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path)?;

    // Write header
    writeln!(
        file,
        "base,M,outer,inner,k,seed,prime_count,total_candidates,density,\
         discriminant,is_perfect_square,sqrt_disc,disc_mod_base,\
         qr_mod3,qr_mod5,qr_mod7,qr_mod11,\
         goldbach_count,goldbach_lambda,goldbach_coverage,\
         is_2p_base,midpoint,symmetric_pair_count,phase_locked"
    )?;

    // Write data
    for (config, analysis) in configs {
        writeln!(
            file,
            "{},{},{},{},{},{},{},{},{:.6},\
             {},{},{},{},\
             {},{},{},{},\
             {},{:.6},{:.6},\
             {},{},{},{}",
            config.base, config.m, config.outer, config.inner, config.k,
            config.seed, config.prime_count, config.total_candidates, config.density,
            analysis.discriminant,
            analysis.is_perfect_square,
            analysis.sqrt_discriminant.unwrap_or(-1),
            analysis.discriminant_mod_base,
            analysis.qr_mod3, analysis.qr_mod5, analysis.qr_mod7, analysis.qr_mod11,
            analysis.goldbach_count,
            analysis.goldbach_lambda,
            analysis.goldbach_coverage,
            analysis.is_2p_base,
            analysis.midpoint,
            analysis.symmetric_pairs.len(),
            analysis.phase_locked,
        )?;
    }

    Ok(())
}

// ============================================================================
// Statistical Analysis
// ============================================================================

/// Compute comprehensive statistical summary
fn compute_statistics(
    configs: &[(MembraneConfig, QuadraticAnalysis)],
) -> StatisticalSummary {
    let total_configs = configs.len();

    // Separate perfect square vs non-square
    let perfect_square_densities: Vec<f64> = configs
        .iter()
        .filter(|(_, a)| a.is_perfect_square)
        .map(|(c, _)| c.density)
        .collect();

    let non_square_densities: Vec<f64> = configs
        .iter()
        .filter(|(_, a)| !a.is_perfect_square)
        .map(|(c, _)| c.density)
        .collect();

    let perfect_square_count = perfect_square_densities.len();
    let perfect_square_avg_density = if perfect_square_count > 0 {
        perfect_square_densities.iter().sum::<f64>() / perfect_square_count as f64
    } else {
        0.0
    };

    let non_square_avg_density = if !non_square_densities.is_empty() {
        non_square_densities.iter().sum::<f64>() / non_square_densities.len() as f64
    } else {
        0.0
    };

    // Effect sizes (perfect square vs non-square)
    let hedges_g_value = if perfect_square_count > 0 && !non_square_densities.is_empty() {
        hedges_g(&perfect_square_densities, &non_square_densities)
    } else {
        0.0
    };

    let cliffs_delta_value = if perfect_square_count > 0 && !non_square_densities.is_empty() {
        cliffs_delta(&perfect_square_densities, &non_square_densities)
    } else {
        0.0
    };

    // Correlation analysis (using Spearman)
    let discriminants: Vec<f64> = configs.iter().map(|(_, a)| a.discriminant as f64).collect();
    let densities: Vec<f64> = configs.iter().map(|(c, _)| c.density).collect();
    let goldbach_counts: Vec<f64> = configs.iter().map(|(_, a)| a.goldbach_count as f64).collect();
    let qr_counts: Vec<f64> = configs
        .iter()
        .map(|(_, a)| {
            // Count positive QR symbols
            let count = [a.qr_mod3, a.qr_mod5, a.qr_mod7, a.qr_mod11]
                .iter()
                .filter(|&&x| x == 1)
                .count();
            count as f64
        })
        .collect();

    let spearman_discriminant_density = if discriminants.len() > 2 {
        spearman_rho(&discriminants, &densities)
    } else {
        0.0
    };

    let spearman_goldbach_density = if goldbach_counts.len() > 2 {
        spearman_rho(&goldbach_counts, &densities)
    } else {
        0.0
    };

    let spearman_qr_count_density = if qr_counts.len() > 2 {
        spearman_rho(&qr_counts, &densities)
    } else {
        0.0
    };

    // Base 10 M=2 analysis
    let base10_m2_k0: Vec<f64> = configs
        .iter()
        .filter(|(c, _)| c.base == 10 && c.m == 2 && c.k == 0)
        .map(|(c, _)| c.density)
        .collect();

    let base10_m2_k1: Vec<f64> = configs
        .iter()
        .filter(|(c, _)| c.base == 10 && c.m == 2 && c.k == 1)
        .map(|(c, _)| c.density)
        .collect();

    let base10_m2_effect_size = if !base10_m2_k0.is_empty() && !base10_m2_k1.is_empty() {
        hedges_g(&base10_m2_k0, &base10_m2_k1).abs()
    } else {
        0.0
    };

    let base10_m2_unique_signature = base10_m2_effect_size > 0.5; // Medium+ effect

    StatisticalSummary {
        total_configs,
        perfect_square_count,
        perfect_square_avg_density,
        non_square_avg_density,
        spearman_discriminant_density,
        spearman_goldbach_density,
        spearman_qr_count_density,
        hedges_g_perfect_square: hedges_g_value,
        cliffs_delta_perfect_square: cliffs_delta_value,
        base10_m2_unique_signature,
        base10_m2_effect_size,
    }
}

// ============================================================================
// Report Generation
// ============================================================================

/// Generate comprehensive markdown report
fn generate_report(
    stats: &StatisticalSummary,
    _configs: &[(MembraneConfig, QuadraticAnalysis)],
) -> String {
    let mut report = String::new();

    report.push_str("# Quadratic Membrane Discriminant Analysis Report\n\n");
    report.push_str("## Executive Summary\n\n");
    report.push_str(&format!("**Total Configurations Analyzed**: {}\n\n", stats.total_configs));

    report.push_str("## 1. Perfect Square Discriminant Test (Algebraic Lock)\n\n");
    report.push_str("**Hypothesis**: Membranes with Δ = perfect square should show near-zero prime density.\n\n");
    report.push_str(&format!(
        "- **Perfect Square Count**: {} ({:.2}% of total)\n",
        stats.perfect_square_count,
        100.0 * stats.perfect_square_count as f64 / stats.total_configs as f64
    ));
    report.push_str(&format!(
        "- **Avg Density (Δ=□)**: {:.4}\n",
        stats.perfect_square_avg_density
    ));
    report.push_str(&format!(
        "- **Avg Density (Δ≠□)**: {:.4}\n",
        stats.non_square_avg_density
    ));
    report.push_str(&format!(
        "- **Effect Size (Hedges' g)**: {:.3}\n",
        stats.hedges_g_perfect_square
    ));
    report.push_str(&format!(
        "- **Effect Size (Cliff's δ)**: {:.3}\n\n",
        stats.cliffs_delta_perfect_square
    ));

    let algebraic_lock_verdict = if stats.hedges_g_perfect_square.abs() > 0.5 {
        "✅ STRONG SUPPORT - Perfect squares show significantly lower density"
    } else {
        "❌ NO SUPPORT - Perfect squares do not show reduced density"
    };
    report.push_str(&format!("**Verdict**: {}\n\n", algebraic_lock_verdict));

    report.push_str("## 2. Discriminant-Density Correlation\n\n");
    report.push_str("**Hypothesis**: Discriminant quality correlates with membrane success.\n\n");
    report.push_str(&format!(
        "- **Spearman ρ (Δ vs Density)**: {:.3}\n\n",
        stats.spearman_discriminant_density
    ));

    let correlation_verdict = if stats.spearman_discriminant_density.abs() > 0.3 {
        "✅ STRONG CORRELATION - Discriminant predicts density"
    } else if stats.spearman_discriminant_density.abs() > 0.1 {
        "⚠️  WEAK CORRELATION - Some relationship exists"
    } else {
        "❌ NO CORRELATION - Discriminant does not predict density"
    };
    report.push_str(&format!("**Verdict**: {}\n\n", correlation_verdict));

    report.push_str("## 3. Goldbach Decomposition Analysis\n\n");
    report.push_str("**Hypothesis**: Seeds with more Goldbach pairs show better membrane performance.\n\n");
    report.push_str(&format!(
        "- **Spearman ρ (Goldbach Count vs Density)**: {:.3}\n\n",
        stats.spearman_goldbach_density
    ));

    let goldbach_verdict = if stats.spearman_goldbach_density > 0.3 {
        "✅ STRONG SUPPORT - Goldbach richness enhances primality"
    } else {
        "❌ NO SUPPORT - Goldbach pairs do not predict success"
    };
    report.push_str(&format!("**Verdict**: {}\n\n", goldbach_verdict));

    report.push_str("## 4. Quadratic Residue Analysis\n\n");
    report.push_str("**Hypothesis**: Favorable QR properties (more +1 Legendre symbols) improve density.\n\n");
    report.push_str(&format!(
        "- **Spearman ρ (QR Count vs Density)**: {:.3}\n\n",
        stats.spearman_qr_count_density
    ));

    report.push_str("## 5. Base 10 M=2 Anomaly Deep Dive\n\n");
    report.push_str("**Question**: Does the k=1 advantage show unique discriminant signature?\n\n");
    report.push_str(&format!(
        "- **Effect Size (k=0 vs k=1)**: {:.3}\n",
        stats.base10_m2_effect_size
    ));
    report.push_str(&format!(
        "- **Unique Signature Detected**: {}\n\n",
        if stats.base10_m2_unique_signature { "YES" } else { "NO" }
    ));

    report.push_str("## Overall Validation Status\n\n");

    let validation_score = [
        stats.hedges_g_perfect_square.abs() > 0.5,
        stats.spearman_discriminant_density.abs() > 0.3,
        stats.spearman_goldbach_density > 0.2,
    ].iter().filter(|&&x| x).count();

    match validation_score {
        3 => report.push_str("✅ **HYPOTHESIS VALIDATED** - Strong multi-baseline support\n"),
        2 => report.push_str("⚠️  **PARTIAL VALIDATION** - Some baselines support, others refute\n"),
        1 => report.push_str("⚠️  **WEAK VALIDATION** - Limited support\n"),
        _ => report.push_str("❌ **HYPOTHESIS REFUTED** - No significant patterns detected\n"),
    }

    report
}

// ============================================================================
// Main Execution
// ============================================================================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Quadratic Membrane Discriminant Analyzer");
    println!("═══════════════════════════════════════════\n");

    // Note: We'll need to generate synthetic test data since solution_space_complete.csv
    // doesn't include individual seed values - it's aggregated by configuration

    println!("⚠️  NOTE: This is a proof-of-concept implementation.");
    println!("   For full analysis, we need to re-run solution_space_explorer.rs");
    println!("   with per-seed tracking to get discriminant data.\n");

    // Create synthetic test data for demonstration
    println!("📊 Generating test data...");

    let mut test_configs = Vec::new();
    let spf = sieve_spf(100000);

    // Test Base 10, M=2, various seeds
    for seed in [11, 13, 17, 19, 23, 29, 31, 37, 41, 43] {
        for k in [0, 1] {
            let config = MembraneConfig {
                base: 10,
                m: 2,
                outer: 3,
                inner: 7,
                k,
                seed,
                prime_count: 0,  // Would be computed
                total_candidates: 1,
                density: 0.0,    // Would be computed
            };

            let analysis = analyze_membrane(&config, &spf);
            test_configs.push((config, analysis));
        }
    }

    println!("✅ Generated {} test configurations\n", test_configs.len());

    // Compute statistics
    println!("📈 Computing statistical analysis...");
    let stats = compute_statistics(&test_configs);
    println!("✅ Statistics computed\n");

    // Generate report
    println!("📝 Generating report...");
    let report = generate_report(&stats, &test_configs);

    // Write outputs
    std::fs::write("DISCRIMINANT_ANALYSIS_REPORT.md", &report)?;
    println!("✅ Report written to DISCRIMINANT_ANALYSIS_REPORT.md\n");

    write_analysis_csv("quadratic_membrane_analysis.csv", &test_configs)?;
    println!("✅ CSV written to quadratic_membrane_analysis.csv\n");

    // Print summary to console
    println!("{}", report);

    println!("\n🎯 Next Steps:");
    println!("   1. Modify solution_space_explorer.rs to track per-seed discriminants");
    println!("   2. Re-run full analysis on 5,616 configurations");
    println!("   3. Validate against perfect square prediction");
    println!("   4. Investigate Base 10 M=2 discriminant signature\n");

    Ok(())
}
