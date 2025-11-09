// Prime Gap Analysis in Coordinate Constellations
//
// Inspired by the connection between RMT spacing distributions and
// prime gap distributions (twin primes, cousin primes, sexy primes).
//
// RESEARCH QUESTION:
// Do coordinate constellation primes exhibit different gap statistics
// than random primes? Do hexagonal structures (φ(base)=6) favor
// certain gap sizes?
//
// GAP TYPES:
// - Twin primes:  gap = 2  (minimal for odd primes)
// - Cousin primes: gap = 4
// - Sexy primes:  gap = 6  (connection to perfect number?)
// - Gap = 8, 10, 12, ... (even gaps)
//
// HYPOTHESIS:
// Hexagonal structure with φ(base)=6 may enhance gap=6 occurrence
// (sexy primes) due to the perfect number connection.

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime;
use std::collections::HashMap;

fn is_coprime(a: u32, b: u32) -> bool {
    fn gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    gcd(a, b) == 1
}

fn septuplet_membrane(middle: u32, x: u32, y: u32, z: u32, base: u32) -> BigUint {
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    result = result * &base_big + BigUint::from(z);
    result = result * &base_big + BigUint::from(y);
    result = result * &base_big + BigUint::from(x);
    result = result * &base_big + BigUint::from(middle);
    result = result * &base_big + BigUint::from(x);
    result = result * &base_big + BigUint::from(y);
    result = result * &base_big + BigUint::from(z);

    result
}

fn collect_primes(base: u32, middle_values: &[u32], limit: u64) -> Vec<BigUint> {
    let mut primes = Vec::new();

    for &middle in middle_values {
        for z in 1..base {
            for y in 1..base {
                for x in 1..base {
                    let candidate = septuplet_membrane(middle, x, y, z, base);

                    if candidate > BigUint::from(limit) {
                        continue;
                    }

                    if is_prime(&candidate) {
                        primes.push(candidate);
                    }
                }
            }
        }
    }

    primes.sort();
    primes.dedup();
    primes
}

fn analyze_gaps(primes: &[BigUint]) -> HashMap<u64, usize> {
    let mut gap_counts = HashMap::new();

    for i in 0..primes.len() - 1 {
        let gap = (&primes[i + 1] - &primes[i])
            .to_string()
            .parse::<u64>()
            .unwrap_or(0);
        *gap_counts.entry(gap).or_insert(0) += 1;
    }

    gap_counts
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║       PRIME GAP ANALYSIS - COORDINATE CONSTELLATIONS         ║");
    println!("║       Twin/Cousin/Sexy Primes in Hexagonal Structure         ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    println!("RESEARCH MOTIVATION:");
    println!("  RMT studies eigenvalue spacings → Number theory studies prime gaps");
    println!("  N=2 GUE exact result → Twin/cousin/sexy prime distributions?");
    println!();

    println!("GAP TYPES:");
    println!("  Twin primes:   gap = 2  (p, p+2)");
    println!("  Cousin primes: gap = 4  (p, p+4)");
    println!("  Sexy primes:   gap = 6  (p, p+6)  ← perfect number!");
    println!();

    let bases_to_test = vec![
        (6, "Base 6 (φ=2, high success 16%)"),
        (7, "Base 7 (φ=6, record 21.30%)"),
        (14, "Base 14 (φ=6, hexagonal)"),
        (18, "Base 18 (φ=6, hexagonal)"),
    ];

    let limit = 1_000_000_000_000u64;

    println!("═══════════════════════════════════════════════════════════════");
    println!("GAP DISTRIBUTION ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    for (base, description) in &bases_to_test {
        println!("─────────────────────────────────────────────────────────────");
        println!("{}", description);
        println!("─────────────────────────────────────────────────────────────");
        println!();

        // Use all coprime middle values
        let middle_values: Vec<u32> = (1..*base).filter(|&m| is_coprime(m, *base)).collect();

        println!("  Collecting primes...");
        let primes = collect_primes(*base, &middle_values, limit);
        println!("  Found {} primes", primes.len());

        if primes.len() < 2 {
            println!("  ⚠ Too few primes for gap analysis");
            println!();
            continue;
        }

        let gaps = analyze_gaps(&primes);
        let total_gaps = gaps.values().sum::<usize>();

        println!();
        println!("  GAP STATISTICS:");
        println!();

        // Special gap types
        let twin_count = *gaps.get(&2).unwrap_or(&0);
        let cousin_count = *gaps.get(&4).unwrap_or(&0);
        let sexy_count = *gaps.get(&6).unwrap_or(&0);

        println!("  Special Gap Types:");
        println!(
            "    Twin primes   (gap=2): {:4} ({:5.2}%)",
            twin_count,
            twin_count as f64 / total_gaps as f64 * 100.0
        );
        println!(
            "    Cousin primes (gap=4): {:4} ({:5.2}%)",
            cousin_count,
            cousin_count as f64 / total_gaps as f64 * 100.0
        );
        println!(
            "    Sexy primes   (gap=6): {:4} ({:5.2}%)",
            sexy_count,
            sexy_count as f64 / total_gaps as f64 * 100.0
        );
        println!();

        // Top 10 most common gaps
        let mut gap_vec: Vec<_> = gaps.iter().collect();
        gap_vec.sort_by(|a, b| b.1.cmp(a.1));

        println!("  Top 10 Most Common Gaps:");
        println!("    Gap  │ Count │ Percent │ Type");
        println!("  ───────┼───────┼─────────┼──────────────────");

        for (i, (&gap, &count)) in gap_vec.iter().take(10).enumerate() {
            let pct = count as f64 / total_gaps as f64 * 100.0;
            let gap_type = match gap {
                2 => "Twin".to_string(),
                4 => "Cousin".to_string(),
                6 => "Sexy (!)".to_string(),
                g if g % 6 == 0 => format!("×6 ({}×6)", g / 6),
                g if g % 2 == 0 => "Even".to_string(),
                _ => "Odd".to_string(),
            };

            println!("  {:6} │ {:5} │  {:5.2}% │ {}", gap, count, pct, gap_type);
        }
        println!();

        // Analyze multiples of 6
        let mult_6_count: usize = gaps
            .iter()
            .filter(|(&gap, _)| gap > 0 && gap % 6 == 0)
            .map(|(_, &count)| count)
            .sum();

        let mult_6_pct = mult_6_count as f64 / total_gaps as f64 * 100.0;

        println!("  HEXAGONAL CONNECTION (φ(base)=6 hypothesis):");
        println!(
            "    Gaps that are multiples of 6: {:4} ({:5.2}%)",
            mult_6_count, mult_6_pct
        );

        if *base == 6 || *base == 7 || *base == 14 || *base == 18 {
            println!("    φ({}) = {}", base, middle_values.len());
            if middle_values.len() == 6 && mult_6_pct > 20.0 {
                println!(
                    "    ✓ ENHANCED: Multiples of 6 are {:.1}% (>20% threshold)",
                    mult_6_pct
                );
            } else if middle_values.len() == 6 {
                println!("    ~ MODERATE: Multiples of 6 are {:.1}%", mult_6_pct);
            }
        }
        println!();

        // Even vs odd gaps
        let even_count: usize = gaps
            .iter()
            .filter(|(&gap, _)| gap % 2 == 0)
            .map(|(_, &count)| count)
            .sum();
        let odd_count = total_gaps - even_count;

        println!("  PARITY ANALYSIS:");
        println!(
            "    Even gaps: {:4} ({:5.2}%)",
            even_count,
            even_count as f64 / total_gaps as f64 * 100.0
        );
        println!(
            "    Odd gaps:  {:4} ({:5.2}%)",
            odd_count,
            odd_count as f64 / total_gaps as f64 * 100.0
        );

        // For random primes, we expect mostly even gaps (since primes are odd)
        // Odd gaps indicate consecutive primes with different parity (rare except 2,3)
        if odd_count > 0 {
            println!(
                "    ⚠ {} odd gaps detected (indicates prime 2 or computational boundary)",
                odd_count
            );
        }
        println!();

        // Gap distribution histogram (for small gaps)
        println!("  GAP DISTRIBUTION (gaps ≤ 30):");
        println!();

        let max_display_gap = 30;
        let max_count = gap_vec
            .iter()
            .filter(|(&gap, _)| gap <= max_display_gap)
            .map(|(_, &count)| count)
            .max()
            .unwrap_or(1);

        for gap in (2..=max_display_gap).step_by(2) {
            let count = *gaps.get(&gap).unwrap_or(&0);
            let bar_len = (count as f64 / max_count as f64 * 50.0) as usize;
            let bar = "█".repeat(bar_len);

            let marker = match gap {
                2 => "← Twin",
                4 => "← Cousin",
                6 => "← Sexy (!)",
                g if g % 6 == 0 => "← ×6",
                _ => "",
            };

            println!("  {:2}: {:4} │ {} {}", gap, count, bar, marker);
        }
        println!();
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("THEORETICAL CONNECTIONS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("RMT EIGENVALUE SPACING ↔ PRIME GAP DISTRIBUTIONS:");
    println!("  N=2 GUE: exact spacing for 2×2 matrices (constructively formalizable)");
    println!("  Twin/cousin/sexy: exact small gaps in primes");
    println!("  Both study: nearest-neighbor correlations");
    println!();

    println!("HEXAGONAL STRUCTURE ↔ GAP=6 ENHANCEMENT:");
    println!("  φ(base)=6 → perfect number → 6-fold symmetry");
    println!("  Sexy primes (gap=6) → multiples of perfect number");
    println!("  Hypothesis: Hexagonal bases favor gap=6?");
    println!();

    println!("CONSTRUCTIVE FORMALIZATION PATH:");
    println!("  1. Exact gap distributions (finite data) ✓ feasible");
    println!("  2. Twin/cousin/sexy counts (discrete) ✓ feasible");
    println!("  3. Comparison to Poisson baseline ✓ feasible");
    println!("  4. Agda proofs with empirical witnesses ✓ feasible");
    println!();

    println!("CONNECTION TO N=2 GUE:");
    println!("  N=2 matrix → 2 eigenvalues → 1 spacing");
    println!("  Twin primes → 2 primes → gap=2 spacing");
    println!("  Both: exact small-system results (not asymptotic)");
    println!("  Parallel: discrete counting vs continuous density");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("RECOMMENDATIONS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("1. COMPARE TO BASELINE:");
    println!("   Generate random primes in same magnitude range");
    println!("   Compare gap distributions (χ² test)");
    println!();

    println!("2. TEST GAP=6 HYPOTHESIS:");
    println!("   Do φ(base)=6 bases show enhanced sexy prime (gap=6) rates?");
    println!("   Statistical significance testing");
    println!();

    println!("3. FORMALIZE IN AGDA:");
    println!("   Gap counts are discrete → easier than continuous spacings");
    println!("   Twin/cousin/sexy counts → constructive witnesses");
    println!("   Parallel to N=2 GUE exact result");
    println!();

    println!("4. GENERALIZE:");
    println!("   Test other gap types (8, 10, 12, ...)");
    println!("   k-tuple gaps (prime triplets with gaps (2,2), (4,2), etc.)");
    println!("   Connection to admissible gap patterns");
    println!();
}
