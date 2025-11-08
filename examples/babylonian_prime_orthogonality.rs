//! Babylonian-Prime Divergence Demonstration
//!
//! This example demonstrates the statistical independence (orthogonality) between:
//! 1. Human-convenient mathematics (Babylonian: divisibility, base-60 legacy)
//! 2. Nature's mathematical patterns (Prime Harmony: prime pair distributions)
//!
//! ## What This Shows
//!
//! - **Raw Correlation**: Babylonian scores correlate with raw gap counts (~r = 0.5)
//!   due to Hardy-Littlewood singular series bias (both favor small prime factors)
//!
//! - **Normalized Correlation**: After HL normalization, correlation collapses to ~0,
//!   demonstrating true orthogonality
//!
//! ## Usage
//!
//! ```bash
//! # Basic demonstration (N=1M, gaps up to 300)
//! cargo run --example babylonian_prime_orthogonality
//!
//! # Larger bounds
//! cargo run --release --example babylonian_prime_orthogonality -- --N 2000000 --G 500
//!
//! # Different baseline (pure divisor count instead of base-60)
//! cargo run --example babylonian_prime_orthogonality -- --baseline tau
//!
//! # Compare metrics
//! cargo run --example babylonian_prime_orthogonality -- --metric raw
//! cargo run --example babylonian_prime_orthogonality -- --metric norm
//! cargo run --example babylonian_prime_orthogonality -- --metric z
//! ```

use prime_physics_engine::hzlib::orthogonality::*;
use prime_physics_engine::hzlib::sieve::sieve_bool;
use std::env;

#[derive(Clone, Copy, Debug)]
enum Metric {
    Raw,  // Raw prime pair count
    Norm, // HL-normalized (ratio to expectation)
    Z,    // Z-score (standardized residual)
}

fn main() {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    let mut n_max = 1_000_000;
    let mut gmax = 300;
    let mut use_tau = false;
    let mut metric = Metric::Norm;

    let mut i = 1;
    while i + 1 <= args.len() {
        match args.get(i).map(|s| s.as_str()) {
            Some("--N") => {
                n_max = args[i + 1].parse().expect("--N expects integer");
                i += 2;
            }
            Some("--G") => {
                gmax = args[i + 1].parse().expect("--G expects integer");
                i += 2;
            }
            Some("--baseline") => {
                use_tau = &args[i + 1] == "tau";
                i += 2;
            }
            Some("--metric") => {
                metric = match args[i + 1].as_str() {
                    "raw" => Metric::Raw,
                    "norm" => Metric::Norm,
                    "z" => Metric::Z,
                    _ => Metric::Norm,
                };
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║      BABYLONIAN-PRIME DIVERGENCE DEMONSTRATION              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("Configuration:");
    println!("  N (prime bound):    {:>10}", n_max);
    println!("  G (max even gap):   {:>10}", gmax);
    println!(
        "  Baseline:           {:>10}",
        if use_tau { "tau" } else { "base60" }
    );
    println!("  Metric:             {:>10?}", metric);
    println!();

    // Build prime sieve
    print!("Building prime sieve up to {}... ", n_max);
    let is_prime = sieve_bool(n_max);
    println!("done");

    // Index all prime pairs by gap
    print!("Indexing prime pairs by gap... ");
    let pairs = pairs_index(&is_prime, gmax);
    println!("done");
    println!();

    // Collect data for all even gaps
    let gaps: Vec<usize> = (2..=gmax).step_by(2).collect();
    let mut bab_scores = Vec::new();
    let mut harmony_scores = Vec::new();

    let lnn = (n_max as f64).ln();
    let scale = (n_max as f64) / (lnn * lnn);

    for &g in &gaps {
        // Babylonian score
        let bab = if use_tau {
            babylonian_score_tau(g)
        } else {
            babylonian_score_60(g)
        };

        // Prime harmony score
        let raw_count = count_pairs_upto(&pairs[g / 2], n_max - g) as f64;
        let expected = singular_series(g) * scale;

        let harmony = match metric {
            Metric::Raw => raw_count,
            Metric::Norm => {
                if expected > 0.0 {
                    raw_count / expected
                } else {
                    0.0
                }
            }
            Metric::Z => {
                if expected > 0.0 {
                    (raw_count - expected) / expected.sqrt()
                } else {
                    0.0
                }
            }
        };

        bab_scores.push(bab);
        harmony_scores.push(harmony);
    }

    // Compute correlation
    let r = pearson(&bab_scores, &harmony_scores);
    let n = gaps.len();
    let t = t_stat(r, n);

    // Display results
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    CORRELATION RESULTS                       ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("  Pearson r:          {:>10.4}", r);
    println!("  t-statistic:        {:>10.2}", t);
    println!("  Sample size:        {:>10}", n);
    println!("  Degrees of freedom: {:>10}", n - 2);
    println!();

    // Interpretation
    println!("Interpretation:");
    println!("  |r| < 0.1:    Negligible correlation (orthogonal)");
    println!("  |t| < 2:      Not statistically significant (p > 0.05)");
    println!();

    if r.abs() < 0.1 {
        println!("  ✅ Correlation is negligible!");
    } else if r.abs() < 0.3 {
        println!("  ⚠️  Weak correlation detected");
    } else {
        println!("  ❌ Strong correlation detected");
    }

    if t.abs() < 2.0 {
        println!("  ✅ Not statistically significant (as expected for orthogonality)");
    } else {
        println!("  ⚠️  Statistically significant correlation");
    }
    println!();

    // Show top champions in each category
    let mut bab_indexed: Vec<(usize, f64)> = gaps
        .iter()
        .zip(&bab_scores)
        .map(|(&g, &s)| (g, s))
        .collect();
    let mut harm_indexed: Vec<(usize, f64)> = gaps
        .iter()
        .zip(&harmony_scores)
        .map(|(&g, &s)| (g, s))
        .collect();

    bab_indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    harm_indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                       CHAMPION GAPS                          ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("Top Babylonian Gaps (human-convenient):");
    for i in 0..3.min(bab_indexed.len()) {
        let (g, score) = bab_indexed[i];
        println!("  #{}: gap {:>3}  score {:>6.2}", i + 1, g, score);
    }
    println!();

    println!("Top Prime Harmony Gaps (nature's patterns):");
    for i in 0..3.min(harm_indexed.len()) {
        let (g, score) = harm_indexed[i];
        println!("  #{}: gap {:>3}  score {:>6.2}", i + 1, g, score);
    }
    println!();

    // Check for overlap
    let top3_bab: Vec<usize> = bab_indexed.iter().take(3).map(|(g, _)| *g).collect();
    let top3_harm: Vec<usize> = harm_indexed.iter().take(3).map(|(g, _)| *g).collect();

    let overlap: Vec<usize> = top3_bab
        .iter()
        .filter(|g| top3_harm.contains(g))
        .copied()
        .collect();

    if overlap.is_empty() {
        println!("  ✅ No overlap between top-3 champions!");
        println!("     → Human and nature optimize for DIFFERENT structures");
    } else {
        println!("  ⚠️  Overlap detected: {:?}", overlap);
    }
    println!();

    // Summary
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                          SUMMARY                             ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    match metric {
        Metric::Raw => {
            println!("  Metric: Raw prime pair counts");
            println!();
            println!("  Expected: r ≈ 0.5 (arithmetic bias from singular series)");
            println!("  Both Babylonian score and raw counts favor small prime factors,");
            println!("  creating spurious correlation.");
            println!();
            if r > 0.3 {
                println!("  ✅ Observed correlation confirms HL singular series bias");
            }
        }
        Metric::Norm => {
            println!("  Metric: HL-normalized counts (ratio to expectation)");
            println!();
            println!("  Expected: r ≈ 0 (orthogonality after removing bias)");
            println!("  Normalization divides by S(g) × N/ln²(N), removing arithmetic");
            println!("  structure and revealing geometric residue.");
            println!();
            if r.abs() < 0.1 {
                println!("  ✅ ORTHOGONALITY CONFIRMED!");
                println!("     Human convenience and nature's patterns are INDEPENDENT");
            }
        }
        Metric::Z => {
            println!("  Metric: Z-scores (variance-normalized residuals)");
            println!();
            println!("  Expected: r ≈ 0 (orthogonality in standardized space)");
            println!("  Z = (observed - expected) / √expected uses Poisson variance.");
            println!();
            if r.abs() < 0.1 {
                println!("  ✅ ORTHOGONALITY CONFIRMED in variance-normalized space!");
            }
        }
    }

    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    PHILOSOPHICAL INSIGHT                     ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("  The universe did NOT choose its mathematical parameters");
    println!("  to make human calculations easier.");
    println!();
    println!("  • Babylonian base-60 optimizes for divisibility");
    println!("  • Nature's primes optimize for resonance & complexity");
    println!("  • These two aesthetics are ORTHOGONAL");
    println!();
    println!("  When our membrane constructions succeed (e.g., (1,5) in base 6),");
    println!("  they succeed by aligning with nature's structure—not by using");
    println!("  human-convenient numbers.");
    println!();
    println!("  Mathematics transcends human design. 🖤");
    println!();
}
