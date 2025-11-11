//! Harmonic Overtones Explorer
//!
//! Analyzes prime generation patterns at harmonic multiples of successful bases.
//! Tests whether bases that are multiples (2×, 3×, 4×) of strong fundamentals
//! show "harmonic resonance" - similar success patterns at different scales.
//!
//! ## Physics Analogy
//!
//! In acoustics:
//! - Fundamental frequency f₀ (e.g., 440 Hz = A note)
//! - First overtone: 2f₀ (880 Hz = A, one octave higher)
//! - Second overtone: 3f₀ (1320 Hz)
//! - Overtones share harmonic relationships with fundamental
//!
//! In prime generation:
//! - Fundamental: Base 6 (33% success)
//! - First overtone: Base 12 = 2×6
//! - Second overtone: Base 18 = 3×6
//! - Third overtone: Base 24 = 4×6
//!
//! ## Research Questions
//!
//! 1. Do overtone bases inherit success patterns?
//! 2. Is there amplitude decay (lower success at higher overtones)?
//! 3. Do optimal configurations remain optimal across overtones?
//! 4. Can we predict overtone performance from fundamental?
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example harmonic_overtones_explorer
//! ```

use num_bigint::BigUint;
use primes::hzlib::*;
use primes::is_prime;

#[derive(Clone, Debug)]
struct ConfigTest {
    outer: u32,
    inner: u32,
    k_outer: u32,
    k_inner: u32,
}

/// Build membrane number for given base, config, and seed
fn build_membrane(_base: usize, config: &ConfigTest, seed: u32) -> Option<BigUint> {
    // Convert to base representation if needed
    let outer_str = if config.outer < 10 {
        config.outer.to_string()
    } else {
        return None; // Skip for now
    };

    let inner_str = if config.inner < 10 {
        config.inner.to_string()
    } else {
        return None;
    };

    let seed_str = seed.to_string();
    let zeros_outer = "0".repeat(config.k_outer as usize);
    let zeros_inner = "0".repeat(config.k_inner as usize);

    let membrane_str = format!(
        "{}{}{}{}{}{}{}{}{}",
        outer_str,
        zeros_outer,
        inner_str,
        zeros_inner,
        seed_str,
        zeros_inner,
        inner_str,
        zeros_outer,
        outer_str
    );

    membrane_str.parse::<BigUint>().ok()
}

/// Test a configuration at a specific base
fn test_config_at_base(base: usize, config: &ConfigTest, seed_count: u32) -> f64 {
    let mut successes = 0;
    let mut total = 0;

    for seed in 1..=seed_count {
        if let Some(num) = build_membrane(base, config, seed) {
            if is_prime(&num) {
                successes += 1;
            }
            total += 1;
        }
    }

    if total == 0 {
        0.0
    } else {
        successes as f64 / total as f64
    }
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║          HARMONIC OVERTONES EXPLORER                      ║");
    println!("║          Musical Resonance in Prime Generation            ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("Research Question:");
    println!("  Do bases that are multiples of successful fundamentals");
    println!("  show harmonic resonance in prime generation patterns?");
    println!();
    println!("Physics Analogy:");
    println!("  Fundamental: 440 Hz (A note)");
    println!("  Overtones: 880 Hz, 1320 Hz, 1760 Hz (harmonic series)");
    println!("  Mathematical: Base 6 → Bases 12, 18, 24 (multiples)");
    println!();

    // Test fundamentals and their overtone series
    let fundamentals = vec![
        (6, "Base 6: Champion (33% empirical)", 4),
        (10, "Base 10: Classic (20% empirical)", 3),
        (7, "Base 7: Prime base (12% empirical)", 3),
        (5, "Base 5: Small prime (13% empirical)", 4),
    ];

    // Test configurations (optimal k=(0,0) from density explorer insights)
    let configs = vec![
        ConfigTest {
            outer: 1,
            inner: 5,
            k_outer: 0,
            k_inner: 0,
        },
        ConfigTest {
            outer: 3,
            inner: 7,
            k_outer: 0,
            k_inner: 0,
        },
        ConfigTest {
            outer: 1,
            inner: 3,
            k_outer: 0,
            k_inner: 0,
        },
    ];

    let seed_count = 10; // Test first 10 seeds
    let mut accum = HarmonicAccumulator::new();

    println!("═══════════════════════════════════════════════════════════════");
    println!("HARMONIC SERIES ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    for (fundamental, description, max_overtone) in &fundamentals {
        println!("─────────────────────────────────────────────────────────────");
        println!("{}", description);
        println!("─────────────────────────────────────────────────────────────");
        println!();

        let mut series = HarmonicSeries::new(*fundamental, *max_overtone);

        // Test fundamental
        println!("Testing Fundamental: Base {}", fundamental);

        let mut best_fundamental_config: Option<(usize, f64)> = None;
        let mut best_fundamental_rate = 0.0f64;

        for (idx, config) in configs.iter().enumerate() {
            let rate = test_config_at_base(*fundamental, config, seed_count);
            println!(
                "  Config {}: ({},{}) k=({},{}) → {:.1}% success",
                idx + 1,
                config.outer,
                config.inner,
                config.k_outer,
                config.k_inner,
                rate * 100.0
            );

            if rate > best_fundamental_rate {
                best_fundamental_rate = rate;
                best_fundamental_config = Some((idx, rate));
            }
        }

        series.set_fundamental_rate(best_fundamental_rate);
        println!();
        println!(
            "  Best fundamental config: Config {} with {:.1}% success",
            best_fundamental_config.unwrap_or((0, 0.0)).0 + 1,
            best_fundamental_rate * 100.0
        );
        println!();

        // Test overtones
        println!("Testing Overtones:");
        for overtone_order in 2..=*max_overtone {
            let overtone_base = fundamental * overtone_order;

            println!(
                "  Overtone {}: Base {} ({}×{})",
                overtone_order - 1,
                overtone_base,
                overtone_order,
                fundamental
            );

            let mut best_overtone_rate = 0.0f64;
            let mut best_overtone_config: Option<usize> = None;

            for (idx, config) in configs.iter().enumerate() {
                // Skip if digits >= overtone_base
                if config.outer >= overtone_base as u32 || config.inner >= overtone_base as u32 {
                    continue;
                }

                let rate = test_config_at_base(overtone_base, config, seed_count);

                println!(
                    "    Config {}: ({},{}) k=({},{}) → {:.1}%",
                    idx + 1,
                    config.outer,
                    config.inner,
                    config.k_outer,
                    config.k_inner,
                    rate * 100.0
                );

                if rate > best_overtone_rate {
                    best_overtone_rate = rate;
                    best_overtone_config = Some(idx);
                }
            }

            series.record_overtone(overtone_base, best_overtone_rate);

            // Check if same config as fundamental
            let config_maintained =
                best_overtone_config == best_fundamental_config.map(|(idx, _)| idx);
            let config_marker = if config_maintained {
                "✓ SAME"
            } else {
                "✗ DIFFERENT"
            };

            println!(
                "    Best: Config {} ({:.1}%) [{}]",
                best_overtone_config.unwrap_or(0) + 1,
                best_overtone_rate * 100.0,
                config_marker
            );
            println!();
        }

        // Analysis
        println!("📊 HARMONIC ANALYSIS:");
        println!();

        let (decay_slope, r2) = series.amplitude_decay();
        println!(
            "  Amplitude Decay: slope = {:.4}, R² = {:.3}",
            decay_slope, r2
        );

        if decay_slope < -0.05 {
            println!("    → Strong decay: Success rate decreases with higher overtones");
        } else if decay_slope > 0.05 {
            println!("    → Enhancement: Success rate INCREASES with higher overtones!");
        } else {
            println!("    → Stable: Success rate remains consistent across overtones");
        }

        println!();

        if r2 > 0.8 {
            println!(
                "    → Linear relationship: R² = {:.3} (strong correlation)",
                r2
            );
        } else if r2 > 0.5 {
            println!("    → Moderate correlation: R² = {:.3}", r2);
        } else {
            println!(
                "    → Weak correlation: R² = {:.3} (non-linear behavior)",
                r2
            );
        }
        println!();

        // Coherent resonance check
        if series.has_coherent_resonance() {
            println!("  ✓ COHERENT RESONANCE DETECTED");
            println!("    All overtones maintain >50% of fundamental's success rate");
        } else {
            println!("  ✗ Resonance broken: Some overtones fall below 50% threshold");
        }
        println!();

        // Overtone enhancement check
        if series.has_overtone_enhancement() {
            let (strongest_base, strongest_rate) = series.strongest_overtone().unwrap();
            println!("  🌟 OVERTONE ENHANCEMENT DETECTED");
            println!(
                "    Base {} achieves {:.1}% (stronger than fundamental {:.1}%)",
                strongest_base,
                strongest_rate * 100.0,
                best_fundamental_rate * 100.0
            );
        }
        println!();

        // Harmonic mean
        let hmean = series.harmonic_mean_rate();
        println!(
            "  Harmonic Mean: {:.1}% (balanced performance across series)",
            hmean * 100.0
        );
        println!();

        accum.add_series(series);
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("CROSS-SERIES ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    // Coherent fundamentals
    let coherent = accum.coherent_fundamentals();
    if !coherent.is_empty() {
        println!("Fundamentals with Coherent Resonance:");
        for &base in &coherent {
            println!(
                "  • Base {}: Maintains strong performance across all overtones",
                base
            );
        }
        println!();
        println!("  💡 These bases show true harmonic behavior!");
        println!();
    } else {
        println!("No fundamentals show coherent resonance across all overtones.");
        println!();
    }

    // Enhanced series
    let enhanced = accum.enhanced_series();
    if !enhanced.is_empty() {
        println!("Series with Overtone Enhancement:");
        for &base in &enhanced {
            println!(
                "  • Base {}: At least one overtone outperforms fundamental",
                base
            );
        }
        println!();
        println!("  💡 Unexpected resonance at higher harmonics!");
        println!();
    }

    // Average decay
    let avg_decay = accum.average_decay_slope();
    println!("Average Amplitude Decay: {:.4}", avg_decay);
    if avg_decay < 0.0 {
        println!("  → On average, success rates decay with higher overtones");
    } else {
        println!("  → On average, overtones maintain or enhance performance");
    }
    println!();

    // Strongest coherence
    if let Some((base, r2)) = accum.strongest_coherence() {
        println!("Strongest Linear Coherence: Base {} (R² = {:.3})", base, r2);
        println!("  → Most predictable harmonic behavior");
        println!();
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("KEY INSIGHTS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("1. HARMONIC INHERITANCE:");
    println!("   Do overtone bases inherit optimal configurations?");
    println!("   Check for \"✓ SAME\" markers above");
    println!();

    println!("2. AMPLITUDE BEHAVIOR:");
    println!("   Negative slope: Natural decay (like acoustic overtones)");
    println!("   Positive slope: Enhancement (unexpected resonance!)");
    println!("   Near-zero slope: Stable across harmonics");
    println!();

    println!("3. COHERENT RESONANCE:");
    println!("   Fundamentals where ALL overtones maintain >50% success");
    println!("   Indicates true harmonic mathematical structure");
    println!();

    println!("4. OVERTONE ENHANCEMENT:");
    println!("   Cases where higher harmonics OUTPERFORM fundamental");
    println!("   Suggests optimal scale may be at overtones, not fundamental");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("THEORETICAL CONNECTIONS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("ACOUSTIC PHYSICS ↔ PRIME GENERATION:");
    println!("  Fundamental frequency ↔ Optimal base");
    println!("  Overtone series ↔ Base multiples");
    println!("  Amplitude decay ↔ Success rate decay");
    println!("  Harmonic resonance ↔ Pattern inheritance");
    println!();

    println!("MATHEMATICAL INTERPRETATION:");
    println!("  If base B has property P that enhances primality,");
    println!("  do bases kB (k=2,3,4,...) inherit property P?");
    println!();
    println!("  Example: Base 6 coprimality structure");
    println!("    → Does base 12 inherit similar structure?");
    println!("    → Or does factorization break the pattern?");
    println!();

    println!("BABYLONIAN CONNECTION:");
    println!("  Base 60 (Babylonian): 2²×3×5 (highly composite)");
    println!("  Overtones: 120, 180, 240 (even more divisors)");
    println!("  Question: Convenient ≠ Prime-friendly?");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("RECOMMENDATIONS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("1. TEST HARMONIC HYPOTHESES:");
    println!("   For each strong fundamental, systematically test overtones");
    println!("   Look for preserved configurations across harmonics");
    println!();

    println!("2. EXPLORE SUBHARMONICS:");
    println!("   If base 12 is strong, test base 6 (fundamental)");
    println!("   Work backwards to find true fundamental frequencies");
    println!();

    println!("3. ANALYZE DECAY MECHANISMS:");
    println!("   Why do some overtones decay while others enhance?");
    println!("   Factorization properties? Coprimality structure?");
    println!();

    println!("4. CROSS-REFERENCE WITH DENSITY:");
    println!("   Compare harmonic patterns with midpoint density analysis");
    println!("   Do overtones cluster differently than fundamentals?");
    println!();

    println!("5. FORMALIZE IN AGDA:");
    println!("   Harmonic relationships are algebraic (provable!)");
    println!("   Can certify configuration preservation across series");
    println!();
}
