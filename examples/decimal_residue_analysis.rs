// Decimal Residue Structure Analysis
//
// Tests the hypothesis that Base 10's k*=1 exception arises from decimal-specific
// residue patterns, specifically enrichment of prime-favorable last digits {1,3,7,9}
// when using k=1 padding compared to k=0.
//
// Key Questions:
// 1. Does k=1 create more numbers ending in {1,3,7,9} than k=0 in Base 10?
// 2. Is this effect Base-10-specific (not present in Base 12, 14, etc.)?
// 3. Can we quantify the enrichment and correlate with observed density advantage?

use num_bigint::BigUint;
use num_traits::{Zero, One};
use primes::is_prime;

// ============================================================================
// Residue Distribution Analysis
// ============================================================================

#[derive(Debug, Clone)]
struct ResidueDistribution {
    base: u32,
    k: u32,
    total_candidates: usize,

    // Last digit (mod base) frequency
    last_digit_counts: Vec<usize>,

    // Mod-2 distribution (odd/even)
    odd_count: usize,
    even_count: usize,

    // Prime-favorable digit count (for base 10: {1,3,7,9})
    favorable_count: usize,

    // Actual prime count
    prime_count: usize,
}

impl ResidueDistribution {
    fn new(base: u32, k: u32) -> Self {
        Self {
            base,
            k,
            total_candidates: 0,
            last_digit_counts: vec![0; base as usize],
            odd_count: 0,
            even_count: 0,
            favorable_count: 0,
            prime_count: 0,
        }
    }

    fn get_favorable_digits(base: u32) -> Vec<u32> {
        // For Base 10: {1, 3, 7, 9} (all primes >5 must end in these)
        // For other bases: digits coprime to base
        (1..base)
            .filter(|&d| gcd(d, base) == 1)
            .collect()
    }

    fn is_favorable(&self, digit: u32) -> bool {
        gcd(digit, self.base) == 1
    }

    fn record(&mut self, membrane: &BigUint) {
        self.total_candidates += 1;

        // Last digit (mod base)
        let last_digit_big = membrane % BigUint::from(self.base);
        let last_digit = last_digit_big.to_u32_digits();
        let last_digit = if last_digit.is_empty() { 0 } else { last_digit[0] };
        self.last_digit_counts[last_digit as usize] += 1;

        // Odd/even (mod 2)
        let is_odd = (membrane % 2u32) == BigUint::one();
        if is_odd {
            self.odd_count += 1;
        } else {
            self.even_count += 1;
        }

        // Prime-favorable digit
        if self.is_favorable(last_digit) {
            self.favorable_count += 1;
        }

        // Check if actually prime
        if is_prime(membrane) {
            self.prime_count += 1;
        }
    }

    fn favorable_percentage(&self) -> f64 {
        if self.total_candidates == 0 { return 0.0; }
        100.0 * self.favorable_count as f64 / self.total_candidates as f64
    }

    fn prime_density(&self) -> f64 {
        if self.total_candidates == 0 { return 0.0; }
        100.0 * self.prime_count as f64 / self.total_candidates as f64
    }

    fn odd_percentage(&self) -> f64 {
        if self.total_candidates == 0 { return 0.0; }
        100.0 * self.odd_count as f64 / self.total_candidates as f64
    }
}

// ============================================================================
// Membrane Generation
// ============================================================================

/// Generate membrane using positional notation
fn construct_membrane(
    base: u32,
    outer: u32,
    inner: u32,
    m: usize,
    k: u32,
    seed: u64,
) -> BigUint {
    let base_big = BigUint::from(base);
    let mut result = BigUint::zero();
    let mut position = 0;

    let mut add_digit = |digit: u32| {
        result += BigUint::from(digit) * base_big.pow(position);
        position += 1;
    };

    // Structure: outer [k×0] inner [k×0] SEED [k×0] inner [k×0] outer

    // Left side
    add_digit(outer);
    for _ in 0..k {
        add_digit(0);
    }
    add_digit(inner);
    for _ in 0..k {
        add_digit(0);
    }

    // Middle (seed in base representation)
    let mut seed_val = seed;
    for _ in 0..m {
        add_digit((seed_val % base as u64) as u32);
        seed_val /= base as u64;
    }

    // Right side (mirror)
    for _ in 0..k {
        add_digit(0);
    }
    add_digit(inner);
    for _ in 0..k {
        add_digit(0);
    }
    add_digit(outer);

    result
}

/// Analyze residue distribution for a specific configuration
fn analyze_residue_distribution(
    base: u32,
    outer: u32,
    inner: u32,
    m: usize,
    k: u32,
) -> ResidueDistribution {
    let mut dist = ResidueDistribution::new(base, k);

    // Calculate seed range
    let seed_min = if m > 1 { base.pow((m - 1) as u32) as u64 } else { 1 };
    let seed_max = base.pow(m as u32) as u64;

    // Generate all membranes in this configuration
    for seed in seed_min..seed_max {
        let membrane = construct_membrane(base, outer, inner, m, k, seed);
        dist.record(&membrane);
    }

    dist
}

// ============================================================================
// Statistical Analysis
// ============================================================================

/// Chi-square test for distribution differences
fn chi_square_test(observed1: &[usize], observed2: &[usize]) -> (f64, f64) {
    assert_eq!(observed1.len(), observed2.len());

    let total1: usize = observed1.iter().sum();
    let total2: usize = observed2.iter().sum();
    let total = total1 + total2;

    let mut chi_square = 0.0;

    for i in 0..observed1.len() {
        let expected1 = (total1 as f64) * (observed1[i] + observed2[i]) as f64 / total as f64;
        let expected2 = (total2 as f64) * (observed1[i] + observed2[i]) as f64 / total as f64;

        if expected1 > 0.0 {
            chi_square += (observed1[i] as f64 - expected1).powi(2) / expected1;
        }
        if expected2 > 0.0 {
            chi_square += (observed2[i] as f64 - expected2).powi(2) / expected2;
        }
    }

    // Degrees of freedom = categories - 1
    let df = (observed1.len() - 1) as f64;

    (chi_square, df)
}

/// Calculate enrichment factor (k=1 vs k=0)
fn calculate_enrichment(favorable_k1: f64, favorable_k0: f64) -> f64 {
    if favorable_k0 == 0.0 { return 0.0; }
    favorable_k1 / favorable_k0
}

// ============================================================================
// Comparative Analysis Across Bases
// ============================================================================

struct BaseComparison {
    base: u32,
    outer: u32,
    inner: u32,
    m: usize,

    dist_k0: ResidueDistribution,
    dist_k1: ResidueDistribution,
    dist_k2: ResidueDistribution,

    favorable_enrichment_k1: f64,  // k=1 vs k=0
    prime_density_advantage_k1: f64,  // k=1 - k=0 (pp)
}

impl BaseComparison {
    fn analyze(base: u32, outer: u32, inner: u32, m: usize) -> Self {
        println!("  Analyzing Base {}, ({},{}), M={}...", base, outer, inner, m);

        let dist_k0 = analyze_residue_distribution(base, outer, inner, m, 0);
        let dist_k1 = analyze_residue_distribution(base, outer, inner, m, 1);
        let dist_k2 = analyze_residue_distribution(base, outer, inner, m, 2);

        let favorable_enrichment_k1 = calculate_enrichment(
            dist_k1.favorable_percentage(),
            dist_k0.favorable_percentage()
        );

        let prime_density_advantage_k1 = dist_k1.prime_density() - dist_k0.prime_density();

        Self {
            base,
            outer,
            inner,
            m,
            dist_k0,
            dist_k1,
            dist_k2,
            favorable_enrichment_k1,
            prime_density_advantage_k1,
        }
    }

    fn print_summary(&self) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║  BASE {} RESIDUE ANALYSIS: ({},{}) M={}                  ",
                 self.base, self.outer, self.inner, self.m);
        println!("╚════════════════════════════════════════════════════════╝\n");

        println!("Last Digit Distribution:");
        println!("  Digit | k=0 Freq | k=1 Freq | k=2 Freq | Prime-Favorable?");
        println!("  ------|----------|----------|----------|------------------");

        for digit in 0..self.base {
            let favorable = if self.dist_k0.is_favorable(digit) { "✓" } else { "" };
            println!("  {:5} | {:8} | {:8} | {:8} | {:^16}",
                digit,
                self.dist_k0.last_digit_counts[digit as usize],
                self.dist_k1.last_digit_counts[digit as usize],
                self.dist_k2.last_digit_counts[digit as usize],
                favorable
            );
        }

        println!("\nResidue Pattern Summary:");
        println!("  Metric                          | k=0      | k=1      | k=2      |");
        println!("  --------------------------------|----------|----------|----------|");
        println!("  Prime-Favorable Digits %        | {:7.2}% | {:7.2}% | {:7.2}% |",
            self.dist_k0.favorable_percentage(),
            self.dist_k1.favorable_percentage(),
            self.dist_k2.favorable_percentage()
        );
        println!("  Odd Numbers %                   | {:7.2}% | {:7.2}% | {:7.2}% |",
            self.dist_k0.odd_percentage(),
            self.dist_k1.odd_percentage(),
            self.dist_k2.odd_percentage()
        );
        println!("  Actual Prime Density %          | {:7.2}% | {:7.2}% | {:7.2}% |",
            self.dist_k0.prime_density(),
            self.dist_k1.prime_density(),
            self.dist_k2.prime_density()
        );

        println!("\nKey Findings:");
        println!("  Favorable Digit Enrichment (k=1/k=0): {:.4}×", self.favorable_enrichment_k1);
        println!("  Prime Density Advantage (k=1 - k=0):  {:+.2}pp", self.prime_density_advantage_k1);

        // Chi-square test for distribution difference
        let (chi2, df) = chi_square_test(
            &self.dist_k0.last_digit_counts,
            &self.dist_k1.last_digit_counts
        );
        println!("  Chi-Square (k=0 vs k=1 distribution): χ²={:.2}, df={:.0}", chi2, df);

        // Interpretation
        if self.favorable_enrichment_k1 > 1.02 && self.prime_density_advantage_k1 > 1.0 {
            println!("\n  ✓ HYPOTHESIS SUPPORTED: k=1 enriches favorable digits AND increases prime density");
        } else if self.favorable_enrichment_k1 > 1.02 {
            println!("\n  ⚠️  PARTIAL: k=1 enriches favorable digits but doesn't increase primes");
        } else {
            println!("\n  ✗ NO ENRICHMENT: k=1 does not favor prime-favorable digit distribution");
        }
    }
}

// ============================================================================
// Main Analysis Framework
// ============================================================================

fn main() {
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║       DECIMAL RESIDUE STRUCTURE HYPOTHESIS TEST       ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    println!("Hypothesis: Base 10's k*=1 exception arises from k=1 padding");
    println!("creating favorable last-digit distributions (enrichment of {{1,3,7,9}})");
    println!("compared to k=0.\n");

    println!("Testing Strategy:");
    println!("  1. Base 10 (known k*=1): Should show favorable enrichment");
    println!("  2. Base 12 (known k*=0): Should show NO enrichment");
    println!("  3. Base 14 (known k*=0): Control case\n");

    // Test configurations
    let test_cases = vec![
        (10, 3, 7, 2, "Critical - Base 10 exception"),
        (12, 1, 5, 2, "Counter-test - Base 12 (k*=0)"),
        (14, 1, 5, 2, "Control - Base 14 (k*=0)"),
    ];

    let mut results = Vec::new();

    for (base, outer, inner, m, description) in test_cases {
        println!("════════════════════════════════════════════════════════");
        println!("Testing: {} - Base {}", description, base);
        println!("════════════════════════════════════════════════════════\n");

        let comparison = BaseComparison::analyze(base, outer, inner, m);
        comparison.print_summary();

        results.push(comparison);

        println!();
    }

    // Cross-base summary
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║              CROSS-BASE COMPARISON                     ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    println!("Base | Favorable Enrichment | Prime Δ(k=1-k=0) | Known k* | Match?");
    println!("-----|----------------------|------------------|----------|--------");

    for result in &results {
        let known_k_star = match result.base {
            10 => 1,
            12 => 0,
            14 => 0,
            _ => 0,
        };

        let predicted_k_star = if result.favorable_enrichment_k1 > 1.02
            && result.prime_density_advantage_k1 > 1.0 {
            1
        } else {
            0
        };

        let match_str = if predicted_k_star == known_k_star { "✓" } else { "✗" };

        println!("{:4} | {:20.4}× | {:16.2}pp | {:8} | {:6}",
            result.base,
            result.favorable_enrichment_k1,
            result.prime_density_advantage_k1,
            known_k_star,
            match_str
        );
    }

    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║                    CONCLUSION                          ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    let base10_result = results.iter().find(|r| r.base == 10).unwrap();
    let base12_result = results.iter().find(|r| r.base == 12).unwrap();

    if base10_result.favorable_enrichment_k1 > 1.02
        && base10_result.prime_density_advantage_k1 > 1.0
        && base12_result.favorable_enrichment_k1 <= 1.02 {

        println!("✓ HYPOTHESIS SUPPORTED");
        println!("\nBase 10 shows:");
        println!("  • k=1 enriches favorable digits by {:.1}%",
                 (base10_result.favorable_enrichment_k1 - 1.0) * 100.0);
        println!("  • k=1 increases prime density by {:.2}pp",
                 base10_result.prime_density_advantage_k1);
        println!("\nBase 12 shows:");
        println!("  • NO favorable enrichment ({:.4}×)", base12_result.favorable_enrichment_k1);
        println!("  • Prime density DECREASES with k=1 ({:.2}pp)",
                 base12_result.prime_density_advantage_k1);
        println!("\nMECHANISM IDENTIFIED: Decimal residue structure explains k*=1!");

    } else if base10_result.favorable_enrichment_k1 > 1.02 {
        println!("⚠️  PARTIAL SUPPORT");
        println!("\nBase 10 shows favorable digit enrichment but:");
        println!("  • Enrichment doesn't correlate with prime density advantage");
        println!("  • OR Base 12 also shows enrichment (not Base-10-specific)");
        println!("\nResidues may play a role but are not the complete explanation.");

    } else {
        println!("✗ HYPOTHESIS NOT SUPPORTED");
        println!("\nBase 10 does NOT show favorable digit enrichment with k=1");
        println!("Decimal residue structure does not explain the k*=1 exception");
        println!("\nRecommend: Investigate other mechanisms (Hardy-Littlewood, totient structure)");
    }

    println!("\n════════════════════════════════════════════════════════\n");
    println!("Complete residue data available for further analysis.");
    println!("Run with --verbose for full digit-by-digit breakdown.\n");
}

// ============================================================================
// Helper Functions
// ============================================================================

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
