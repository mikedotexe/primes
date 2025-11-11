//! Harmonic Lagrange Explorer
//!
//! Tests whether Lagrange point positions in concatenated primes follow
//! harmonic ratios when the primes come from harmonically-related bases.
//!
//! ## The Beautiful Hypothesis
//!
//! Musical harmony emerges from simple integer ratios (1/2, 2/3, 3/4...).
//! Lagrange points are positions of gravitational equilibrium.
//! If we concatenate primes from harmonically-related bases (e.g., 6 and 12),
//! do the equilibrium positions follow these same harmonic ratios?
//!
//! This would suggest that:
//! 1. Mathematical harmony transcends domains (music, physics, primes)
//! 2. Spatial equilibrium and base relationships are deeply connected
//! 3. Nature's mathematics follows universal harmonic principles
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example harmonic_lagrange_explorer
//! ```

use num_bigint::BigUint;
use primes::hzlib::*;
use primes::is_prime;
use std::str::FromStr;

/// Generate simple membrane prime for testing
fn generate_test_prime(base: usize, seed: u32) -> Option<String> {
    // Use (1, base-1) configuration for simplicity
    let outer = 1;
    let inner = base - 1;

    let membrane_str = format!("{}{}{}", outer, seed, inner);

    // Quick validation
    if let Ok(num) = membrane_str.parse::<BigUint>() {
        if is_prime(&num) {
            return Some(membrane_str);
        }
    }

    None
}

/// Find Lagrange points in concatenation
fn find_lagrange_points(prime1: &str, prime2: &str, buffer_size: usize) -> Vec<LagrangePoint> {
    let mut points = Vec::new();
    let zeros = "0".repeat(buffer_size);

    for position in 0..buffer_size {
        for digit in 1..=9 {
            let mut test_str = zeros.clone();
            let bytes = unsafe { test_str.as_bytes_mut() };
            bytes[position] = b'0' + digit as u8;

            let full_number = format!("{}{}{}", prime1, test_str, prime2);

            if let Ok(num) = BigUint::from_str(&full_number) {
                if is_prime(&num) {
                    points.push(LagrangePoint {
                        position,
                        digit,
                        buffer_size,
                    });
                }
            }
        }
    }

    points
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║        HARMONIC LAGRANGE POINT EXPLORER                   ║");
    println!("║        Where Music Meets Mathematics                      ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("Research Question:");
    println!("  When concatenating primes from harmonically-related bases,");
    println!("  do Lagrange points cluster at harmonic positions?");
    println!();
    println!("Harmonic Ratios (from music theory):");
    for (ratio, name) in HARMONIC_RATIOS.iter().take(6) {
        println!("  • {:.3} = {}", ratio, name);
    }
    println!();

    // Test cases: harmonic and non-harmonic pairs
    let test_cases = vec![
        ("Harmonic: Base 6 + Base 12 (2× fundamental)", 6, 12, true),
        ("Harmonic: Base 6 + Base 18 (3× fundamental)", 6, 18, true),
        ("Harmonic: Base 5 + Base 10 (2× fundamental)", 5, 10, true),
        ("Non-harmonic: Base 6 + Base 7 (coprime bases)", 6, 7, false),
        (
            "Non-harmonic: Base 6 + Base 11 (coprime bases)",
            6,
            11,
            false,
        ),
    ];

    let buffer_sizes = vec![5, 7, 11]; // Different buffer sizes to test
    let mut comparator = HarmonicComparator::new();

    println!("═══════════════════════════════════════════════════════════════");
    println!("TESTING HARMONIC PAIRS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    for (description, base1, base2, _is_harmonic) in &test_cases {
        println!("─────────────────────────────────────────────────────────────");
        println!("{}", description);
        println!("─────────────────────────────────────────────────────────────");
        println!();

        // Generate test primes from each base
        let mut prime1_str: Option<String> = None;
        let mut prime2_str: Option<String> = None;

        // Try to find working primes
        for seed in 1..20 {
            if prime1_str.is_none() {
                prime1_str = generate_test_prime(*base1, seed);
            }
            if prime2_str.is_none() {
                prime2_str = generate_test_prime(*base2, seed);
            }

            if prime1_str.is_some() && prime2_str.is_some() {
                break;
            }
        }

        if prime1_str.is_none() || prime2_str.is_none() {
            println!("  ⚠ Could not generate test primes for this pair");
            println!();
            continue;
        }

        let prime1 = prime1_str.unwrap();
        let prime2 = prime2_str.unwrap();

        println!("  Prime from base {}: {}", base1, prime1);
        println!("  Prime from base {}: {}", base2, prime2);
        println!();

        // Test each buffer size
        for &buffer_size in &buffer_sizes {
            println!("  Buffer size: {}", buffer_size);

            let pair = HarmonicLagrangePair::new(prime1.clone(), *base1, prime2.clone(), *base2);

            let mut analysis = PositionalAnalysis::new(pair.clone(), buffer_size);

            let lagrange_points = find_lagrange_points(&prime1, &prime2, buffer_size);

            analysis.total_tested = buffer_size * 9; // 9 digits per position

            for lp in lagrange_points {
                analysis.add_lagrange_point(lp.position, lp.digit);
            }

            println!(
                "    Found {} Lagrange points ({:.1}% success)",
                analysis.lagrange_points.len(),
                analysis.success_rate() * 100.0
            );

            if !analysis.lagrange_points.is_empty() {
                // Harmonic clustering analysis
                let tolerance = 0.1; // Within 10% of harmonic ratio
                let (clustered, expected, enrichment) =
                    analysis.harmonic_clustering_test(tolerance);

                println!(
                    "    Harmonic clustering: {}/{} points near harmonics",
                    clustered,
                    analysis.lagrange_points.len()
                );
                println!(
                    "    Expected by chance: {:.1}, Enrichment: {:.2}×",
                    expected, enrichment
                );

                if enrichment > 1.5 {
                    println!("    ✓ SIGNIFICANT HARMONIC CLUSTERING!");
                } else if enrichment > 1.0 {
                    println!("    → Moderate clustering");
                } else {
                    println!("    → No clustering (random distribution)");
                }

                // Dominant harmonic
                if let Some((name, count, ratio)) = analysis.dominant_harmonic(tolerance) {
                    println!(
                        "    Dominant harmonic: {} ({:.3}) with {} points",
                        name, ratio, count
                    );
                }

                // Show position distribution
                println!("    Positions:");
                let mut pos_display = vec!['·'; buffer_size];
                for lp in &analysis.lagrange_points {
                    pos_display[lp.position] = '●';
                }
                print!("      ");
                for (i, &c) in pos_display.iter().enumerate() {
                    print!("{}", c);
                    if (i + 1) % 5 == 0 {
                        print!(" ");
                    }
                }
                println!();

                // Show harmonic positions
                print!("      ");
                for i in 0..buffer_size {
                    let frac = i as f64 / buffer_size as f64;
                    let mut is_harmonic = false;

                    for (ratio, _) in HARMONIC_RATIOS.iter().take(6) {
                        if (frac - ratio).abs() < tolerance {
                            is_harmonic = true;
                            break;
                        }
                    }

                    print!("{}", if is_harmonic { '│' } else { ' ' });
                    if (i + 1) % 5 == 0 {
                        print!(" ");
                    }
                }
                println!(" ← harmonic positions");

                println!();
            }

            comparator.add_analysis(analysis);
        }
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("COMPARATIVE ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("Harmonic vs Non-Harmonic Pairs:");
    println!();

    let tolerance = 0.1;
    let (harmonic_enrich, non_harmonic_enrich, ratio) = comparator.compare_clustering(tolerance);

    println!("  Harmonic Pairs:");
    println!("    Average enrichment: {:.2}×", harmonic_enrich);
    println!(
        "    Sample size: {} analyses",
        comparator.harmonic_analyses.len()
    );
    println!();

    println!("  Non-Harmonic Pairs:");
    println!("    Average enrichment: {:.2}×", non_harmonic_enrich);
    println!(
        "    Sample size: {} analyses",
        comparator.non_harmonic_analyses.len()
    );
    println!();

    println!("  Ratio (Harmonic / Non-Harmonic): {:.2}×", ratio);
    println!();

    if ratio > 1.3 {
        println!("  ✓ HARMONIC PAIRS SHOW STRONGER CLUSTERING!");
        println!("    This suggests Lagrange positions follow harmonic ratios");
        println!("    when bases have harmonic relationships.");
    } else if ratio > 0.7 && ratio < 1.3 {
        println!("  → No significant difference between groups");
        println!("    Clustering may be independent of base harmonics");
    } else {
        println!("  ✗ Non-harmonic pairs show stronger clustering");
        println!("    Unexpected result - may indicate other patterns");
    }
    println!();

    // Variance comparison
    let (harmonic_var, non_harmonic_var) = comparator.compare_variance();

    println!("Position Variance:");
    println!("  Harmonic pairs: {:.3}", harmonic_var);
    println!("  Non-harmonic pairs: {:.3}", non_harmonic_var);
    println!();

    if harmonic_var < non_harmonic_var * 0.8 {
        println!("  ✓ Harmonic pairs show MORE CONCENTRATED positions");
        println!("    Lagrange points cluster tightly at specific ratios");
    } else if harmonic_var > non_harmonic_var * 1.2 {
        println!("  → Harmonic pairs show MORE SPREAD positions");
        println!("    May indicate multiple harmonic attractors");
    } else {
        println!("  → Similar spread in both groups");
    }
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("KEY INSIGHTS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("1. HARMONIC POSITIONING:");
    println!("   Do Lagrange points cluster at musical harmonic ratios?");
    println!("   Octave (1/2), fifth (2/3), fourth (3/4), golden ratio (φ)");
    println!();

    println!("2. BASE RELATIONSHIP MATTERS:");
    println!("   Comparing harmonic pairs (6+12, 6+18) vs non-harmonic (6+7)");
    println!("   Tests if base multiplication relationship affects positions");
    println!();

    println!("3. UNIVERSAL HARMONY:");
    println!("   If enrichment > 1.5×, positions follow harmonic mathematics");
    println!("   Suggests deep connection: music ↔ gravity ↔ primes");
    println!();

    println!("4. GOLDEN RATIO CONNECTION:");
    println!("   φ ≈ 0.618 appears throughout nature");
    println!("   Do Lagrange points cluster at this divine proportion?");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("THEORETICAL CONNECTIONS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("MUSIC THEORY ↔ LAGRANGE MECHANICS:");
    println!("  Harmonic series: 1/1, 1/2, 1/3, 1/4, ...");
    println!("  L-points: Equilibrium positions between masses");
    println!("  Question: Are these the same mathematical structure?");
    println!();

    println!("OVERTONE BASES ↔ SPATIAL EQUILIBRIUM:");
    println!("  Base 12 = 2 × Base 6 (first overtone)");
    println!("  Do primes from 6+12 create equilibrium at 1/2?");
    println!("  Do primes from 6+18 create equilibrium at 1/3?");
    println!();

    println!("BABYLONIAN DIVERGENCE REVISITED:");
    println!("  Babylonian math: optimize for humans (base 60)");
    println!("  Prime harmony: optimize for nature (harmonic ratios)");
    println!("  Lagrange points may reveal nature's preferred positions");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("RECOMMENDATIONS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("1. EXPAND BASE SERIES:");
    println!("   Test complete harmonic series: 6, 12, 18, 24, 30");
    println!("   Look for systematic position progression");
    println!();

    println!("2. TEST GOLDEN RATIO:");
    println!("   φ ≈ 0.618 appears in nature, Fibonacci, art");
    println!("   Special analysis: do L-points favor φ position?");
    println!();

    println!("3. FOURIER ANALYSIS:");
    println!("   Treat position distribution as signal");
    println!("   FFT to find dominant frequencies = harmonic modes");
    println!();

    println!("4. CROSS-REFERENCE DENSITY:");
    println!("   Combine with midpoint density analysis");
    println!("   Do harmonic pairs show different density patterns?");
    println!();

    println!("5. AGDA CERTIFICATION:");
    println!("   Harmonic ratios are algebraic (certifiable!)");
    println!("   Prove: \"If bases are harmonic, positions cluster\"");
    println!();
}
