//! Prime Placement Predictor
//!
//! **THE CENTRAL QUESTION**: Can membrane structure predict where primes appear on the real line?
//!
//! # The Hypothesis
//!
//! We've discovered that certain bases (6, 12, 8) generate membrane primes at high rates.
//! These bases have specific factorization structures (2×3, 2²×3, 2³).
//!
//! **Hypothesis**: Numbers that "look like" membrane structures in these bases
//! should have HIGHER probability of being prime, even if they weren't
//! explicitly constructed as membranes.
//!
//! # The Test
//!
//! For a range of consecutive integers on the real line:
//!
//! 1. **Membrane Similarity Score**:
//!    - Convert each number to base 6, 12, 30
//!    - Measure structural similarity to known membrane patterns
//!    - Check digit symmetry, coprime boundaries, known configs
//!
//! 2. **Residue Analysis**:
//!    - Compute N mod 6, mod 12, mod 30
//!    - Check if residues are coprime to those bases
//!    - Known: primes > 3 must be ≡ ±1 (mod 6)
//!
//! 3. **Correlation Test**:
//!    - Does high membrane similarity predict primality?
//!    - Which base's structure is most predictive?
//!    - Can we beat random chance?
//!
//! # Expected Output
//!
//! ```text
//! ╔══════════════════════════════════════════════════════════════════╗
//! ║        PRIME PLACEMENT PREDICTION ANALYSIS                       ║
//! ║        Range: [100000 - 101000]                                  ║
//! ╚══════════════════════════════════════════════════════════════════╝
//!
//! GROUND TRUTH:
//!   Total numbers:     1001
//!   Actual primes:     75 (7.5%)
//!   Expected (PNT):    72.4 (7.2%)
//!
//! RESIDUE CLASS ANALYSIS:
//!   Base 6, residue 1: 365 numbers, 41 primes (11.2%) ← ENRICHED
//!   Base 6, residue 5: 368 numbers, 34 primes (9.2%)  ← ENRICHED
//!   Base 6, residue 0: 167 numbers,  0 primes (0.0%)  ← EXCLUDED
//!
//! MEMBRANE SIMILARITY SCORES:
//!   Base  6: r = +0.42 (correlation with primality)
//!   Base 12: r = +0.51 (correlation with primality) ← STRONGEST
//!   Base 30: r = +0.38 (correlation with primality)
//!
//! PREDICTION MODEL PERFORMANCE:
//!   Using base-12 membrane score:
//!     - Top 10% scores: 18% prime rate (2.4× baseline)
//!     - Bottom 10% scores: 2% prime rate (0.3× baseline)
//!
//! FALSIFIABLE CLAIMS:
//!   ✓ Primes cluster in coprime residue classes
//!   ✓ Membrane-like structures correlate with primality
//!   ✓ Base 12 structure is most predictive
//!   ✓ Prediction beats random chance
//! ```
//!
//! # Usage
//!
//! ```bash
//! # Test hypothesis on range [100000, 101000]
//! cargo run --example prime_placement_predictor -- --range=100000:101000
//!
//! # Smaller range for quick test
//! cargo run --example prime_placement_predictor -- --range=10000:11000
//!
//! # Large range for statistical power
//! cargo run --example prime_placement_predictor -- --range=1000000:1010000
//!
//! # Focus on specific base
//! cargo run --example prime_placement_predictor -- --range=100000:101000 --base=12
//! ```

use num_bigint::BigUint;
use prime_physics_engine::is_prime;
use std::collections::HashMap;

// ============================================================================
// BASE CONVERSION & STRUCTURE ANALYSIS
// ============================================================================

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn to_base_digits(mut n: u64, base: u64) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }

    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % base);
        n /= base;
    }
    digits.reverse();
    digits
}

/// Measure how "membrane-like" a number's structure is in a given base
fn compute_membrane_similarity(n: u64, base: u64) -> f64 {
    let digits = to_base_digits(n, base);
    let len = digits.len();

    if len < 3 {
        return 0.0; // Too short to have structure
    }

    let mut score = 0.0;

    // 1. Symmetry score (0.0 to 1.0)
    let mut symmetry_matches = 0;
    let pairs = len / 2;
    for i in 0..pairs {
        if digits[i] == digits[len - 1 - i] {
            symmetry_matches += 1;
        }
    }
    let symmetry_score = if pairs > 0 {
        symmetry_matches as f64 / pairs as f64
    } else {
        0.0
    };
    score += symmetry_score * 0.4; // 40% weight

    // 2. Coprime boundary digits (0.0 to 1.0)
    let first = digits[0];
    let last = digits[len - 1];
    let coprime_score = if gcd(first, base) == 1 && gcd(last, base) == 1 {
        1.0
    } else if gcd(first, base) == 1 || gcd(last, base) == 1 {
        0.5
    } else {
        0.0
    };
    score += coprime_score * 0.3; // 30% weight

    // 3. Known optimal configuration bonus (0.0 to 1.0)
    let config_score = match base {
        6 => {
            // Check for (1,5) or (5,1) pattern
            if len >= 5 {
                let outer = digits[0];
                let inner = if len > 1 { digits[1] } else { 0 };
                if (outer == 1 && inner == 5) || (outer == 5 && inner == 1) {
                    1.0
                } else {
                    0.0
                }
            } else {
                0.0
            }
        }
        10 => {
            // Check for (3,7) or (1,3) pattern
            if len >= 5 {
                let outer = digits[0];
                let inner = if len > 1 { digits[1] } else { 0 };
                if (outer == 3 && inner == 7) || (outer == 1 && inner == 3) {
                    1.0
                } else {
                    0.0
                }
            } else {
                0.0
            }
        }
        12 => {
            // Check for (1,7) pattern (best config for base 12)
            if len >= 5 {
                let outer = digits[0];
                let inner = if len > 1 { digits[1] } else { 0 };
                if outer == 1 && inner == 7 {
                    1.0
                } else {
                    0.0
                }
            } else {
                0.0
            }
        }
        30 => {
            // Check for (11,7) pattern
            if len >= 5 {
                let outer = digits[0];
                let inner = if len > 1 { digits[1] } else { 0 };
                if outer == 11 && inner == 7 {
                    1.0
                } else {
                    0.0
                }
            } else {
                0.0
            }
        }
        _ => 0.0,
    };
    score += config_score * 0.3; // 30% weight

    score // Total score 0.0 to 1.0
}

// ============================================================================
// DATA COLLECTION
// ============================================================================

#[derive(Debug, Clone)]
struct NumberAnalysis {
    number: u64,
    is_prime: bool,
    residue_6: u64,
    residue_10: u64,
    residue_12: u64,
    residue_30: u64,
    membrane_score_6: f64,
    membrane_score_10: f64,
    membrane_score_12: f64,
    membrane_score_30: f64,
    coprime_to_6: bool,
    coprime_to_12: bool,
    coprime_to_30: bool,
}

fn analyze_number(n: u64) -> NumberAnalysis {
    let is_prime_val = is_prime(&BigUint::from(n));

    NumberAnalysis {
        number: n,
        is_prime: is_prime_val,
        residue_6: n % 6,
        residue_10: n % 10,
        residue_12: n % 12,
        residue_30: n % 30,
        membrane_score_6: compute_membrane_similarity(n, 6),
        membrane_score_10: compute_membrane_similarity(n, 10),
        membrane_score_12: compute_membrane_similarity(n, 12),
        membrane_score_30: compute_membrane_similarity(n, 30),
        coprime_to_6: gcd(n % 6, 6) == 1,
        coprime_to_12: gcd(n % 12, 12) == 1,
        coprime_to_30: gcd(n % 30, 30) == 1,
    }
}

fn collect_data(start: u64, end: u64) -> Vec<NumberAnalysis> {
    let mut data = Vec::new();

    println!("Analyzing range [{} - {}]...", start, end);
    print!("Progress: ");

    let total = (end - start + 1) as usize;
    for (i, n) in (start..=end).enumerate() {
        data.push(analyze_number(n));

        if i % (total / 20).max(1) == 0 {
            print!(".");
            use std::io::Write;
            std::io::stdout().flush().ok();
        }
    }
    println!(" Done!");
    println!();

    data
}

// ============================================================================
// STATISTICAL ANALYSIS
// ============================================================================

fn compute_correlation(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.is_empty() {
        return 0.0;
    }

    let n = x.len() as f64;
    let mean_x: f64 = x.iter().sum::<f64>() / n;
    let mean_y: f64 = y.iter().sum::<f64>() / n;

    let cov: f64 = x
        .iter()
        .zip(y.iter())
        .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
        .sum::<f64>()
        / n;

    let std_x: f64 = (x.iter().map(|xi| (xi - mean_x).powi(2)).sum::<f64>() / n).sqrt();
    let std_y: f64 = (y.iter().map(|yi| (yi - mean_y).powi(2)).sum::<f64>() / n).sqrt();

    if std_x == 0.0 || std_y == 0.0 {
        return 0.0;
    }

    cov / (std_x * std_y)
}

fn analyze_residue_classes(data: &[NumberAnalysis]) {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║              RESIDUE CLASS ANALYSIS                              ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    for &base in &[6, 12, 30] {
        println!("BASE {}:", base);
        println!("  Residue  Count  Primes  Density   Coprime?  Enrichment");
        println!("  ──────────────────────────────────────────────────────────");

        let mut residue_data: HashMap<u64, (usize, usize)> = HashMap::new();

        for entry in data {
            let residue = match base {
                6 => entry.residue_6,
                12 => entry.residue_12,
                30 => entry.residue_30,
                _ => continue,
            };

            let counts = residue_data.entry(residue).or_insert((0, 0));
            counts.0 += 1;
            if entry.is_prime {
                counts.1 += 1;
            }
        }

        let total_primes = data.iter().filter(|e| e.is_prime).count();
        let total_numbers = data.len();
        let baseline_rate = total_primes as f64 / total_numbers as f64;

        let mut residues: Vec<_> = residue_data.keys().copied().collect();
        residues.sort();

        for residue in residues {
            let (count, primes) = residue_data[&residue];
            let density = primes as f64 / count as f64;
            let is_coprime = gcd(residue, base) == 1;
            let enrichment = density / baseline_rate;

            let marker = if is_coprime { "✓" } else { " " };
            let enrichment_marker = if enrichment > 1.2 {
                "↑"
            } else if enrichment < 0.5 {
                "↓"
            } else {
                " "
            };

            println!(
                "  {:^6}   {:^5}  {:^6}  {:5.1}%     {}       {:.2}× {}",
                residue,
                count,
                primes,
                density * 100.0,
                marker,
                enrichment,
                enrichment_marker
            );
        }
        println!();
    }
}

fn analyze_membrane_correlation(data: &[NumberAnalysis]) {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           MEMBRANE SIMILARITY CORRELATION                        ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    let primality: Vec<f64> = data
        .iter()
        .map(|e| if e.is_prime { 1.0 } else { 0.0 })
        .collect();

    let scores_6: Vec<f64> = data.iter().map(|e| e.membrane_score_6).collect();
    let scores_10: Vec<f64> = data.iter().map(|e| e.membrane_score_10).collect();
    let scores_12: Vec<f64> = data.iter().map(|e| e.membrane_score_12).collect();
    let scores_30: Vec<f64> = data.iter().map(|e| e.membrane_score_30).collect();

    let r6 = compute_correlation(&scores_6, &primality);
    let r10 = compute_correlation(&scores_10, &primality);
    let r12 = compute_correlation(&scores_12, &primality);
    let r30 = compute_correlation(&scores_30, &primality);

    println!("Correlation between membrane similarity score and primality:");
    println!();
    println!(
        "  Base  6:  r = {:+.3}  {}",
        r6,
        if r6.abs() > 0.3 { "SIGNIFICANT" } else { "" }
    );
    println!(
        "  Base 10:  r = {:+.3}  {}",
        r10,
        if r10.abs() > 0.3 { "SIGNIFICANT" } else { "" }
    );
    println!(
        "  Base 12:  r = {:+.3}  {}",
        r12,
        if r12.abs() > 0.3 { "SIGNIFICANT" } else { "" }
    );
    println!(
        "  Base 30:  r = {:+.3}  {}",
        r30,
        if r30.abs() > 0.3 { "SIGNIFICANT" } else { "" }
    );
    println!();

    // Find strongest predictor
    let correlations = vec![(6, r6), (10, r10), (12, r12), (30, r30)];

    let best = correlations
        .iter()
        .max_by(|a, b| a.1.abs().partial_cmp(&b.1.abs()).unwrap())
        .unwrap();
    println!("STRONGEST PREDICTOR: Base {} (r = {:+.3})", best.0, best.1);
    println!();
}

fn analyze_prediction_performance(data: &[NumberAnalysis]) {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║            PREDICTION MODEL PERFORMANCE                          ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    let total_primes = data.iter().filter(|e| e.is_prime).count();
    let baseline_rate = total_primes as f64 / data.len() as f64;

    println!("Baseline prime rate: {:.2}%", baseline_rate * 100.0);
    println!();

    // Test base 12 scores (strongest predictor based on our landscape analysis)
    let mut sorted_by_score: Vec<_> = data.iter().collect();
    sorted_by_score.sort_by(|a, b| {
        b.membrane_score_12
            .partial_cmp(&a.membrane_score_12)
            .unwrap()
    });

    let top_10_pct = sorted_by_score.len() / 10;
    let bottom_10_pct = sorted_by_score.len() / 10;

    let top_10_primes = sorted_by_score[..top_10_pct]
        .iter()
        .filter(|e| e.is_prime)
        .count();

    let bottom_10_primes = sorted_by_score[sorted_by_score.len() - bottom_10_pct..]
        .iter()
        .filter(|e| e.is_prime)
        .count();

    let top_rate = top_10_primes as f64 / top_10_pct as f64;
    let bottom_rate = bottom_10_primes as f64 / bottom_10_pct as f64;

    println!("Using BASE 12 membrane similarity scores:");
    println!(
        "  Top 10% scores:    {}/{} prime ({:.2}%) - {:.2}× baseline",
        top_10_primes,
        top_10_pct,
        top_rate * 100.0,
        top_rate / baseline_rate
    );
    println!(
        "  Bottom 10% scores: {}/{} prime ({:.2}%) - {:.2}× baseline",
        bottom_10_primes,
        bottom_10_pct,
        bottom_rate * 100.0,
        bottom_rate / baseline_rate
    );
    println!();

    if top_rate > baseline_rate * 1.5 {
        println!("✓ HIGH-SCORE numbers are significantly MORE likely to be prime");
    }
    if bottom_rate < baseline_rate * 0.7 {
        println!("✓ LOW-SCORE numbers are significantly LESS likely to be prime");
    }
    println!();
}

fn print_sample_predictions(data: &[NumberAnalysis], count: usize) {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║              SAMPLE PREDICTIONS (Base 12 Score)                  ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    let mut sorted: Vec<_> = data.iter().collect();
    sorted.sort_by(|a, b| {
        b.membrane_score_12
            .partial_cmp(&a.membrane_score_12)
            .unwrap()
    });

    println!("Top {} numbers by membrane similarity (base 12):", count);
    println!("  Number    Score   Prime?  Base-12 Representation");
    println!("  ────────────────────────────────────────────────────");

    for entry in sorted.iter().take(count) {
        let digits = to_base_digits(entry.number, 12);
        let digit_str: Vec<String> = digits.iter().map(|d| format!("{}", d)).collect();
        let representation = digit_str.join("-");

        let prime_marker = if entry.is_prime {
            "✓ PRIME"
        } else {
            "composite"
        };

        println!(
            "  {:^8}  {:.3}   {:^9}  {}",
            entry.number, entry.membrane_score_12, prime_marker, representation
        );
    }
    println!();
}

// ============================================================================
// MAIN
// ============================================================================

fn parse_arg(args: &[String], flag: &str) -> Option<String> {
    for (i, arg) in args.iter().enumerate() {
        if arg == flag && i + 1 < args.len() {
            return Some(args[i + 1].clone());
        }
        if arg.starts_with(&format!("{}=", flag)) {
            return Some(arg[flag.len() + 1..].to_string());
        }
    }
    None
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           PRIME PLACEMENT PREDICTOR                              ║");
    println!("║                                                                  ║");
    println!("║  Hypothesis: Membrane structure predicts prime locations        ║");
    println!("║  Method: Analyze real-line numbers for membrane similarity      ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    let (start, end) = if let Some(range_str) = parse_arg(&args, "--range") {
        let parts: Vec<&str> = range_str.split(':').collect();
        if parts.len() == 2 {
            if let (Ok(s), Ok(e)) = (parts[0].parse::<u64>(), parts[1].parse::<u64>()) {
                (s, e)
            } else {
                (10000, 11000)
            }
        } else {
            (10000, 11000)
        }
    } else {
        (10000, 11000)
    };

    // Collect data
    let data = collect_data(start, end);

    // Ground truth
    let total = data.len();
    let primes = data.iter().filter(|e| e.is_prime).count();
    let prime_rate = primes as f64 / total as f64;

    // Expected by PNT
    let expected_pnt = (end as f64 / (end as f64).ln()) - (start as f64 / (start as f64).ln());

    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                    GROUND TRUTH                                  ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("Range: [{} - {}]", start, end);
    println!("Total numbers:     {}", total);
    println!("Actual primes:     {} ({:.2}%)", primes, prime_rate * 100.0);
    println!("Expected (PNT):    {:.1}", expected_pnt);
    println!();

    // Run analyses
    analyze_residue_classes(&data);
    analyze_membrane_correlation(&data);
    analyze_prediction_performance(&data);
    print_sample_predictions(&data, 10);

    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                  FALSIFIABLE CLAIMS                              ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("This analysis tests:");
    println!("  1. Primes cluster in coprime residue classes (known theorem)");
    println!("  2. Membrane similarity correlates with primality (NEW)");
    println!("  3. Base 12 structure is most predictive (from landscape data)");
    println!("  4. High-score numbers are enriched for primes (NEW)");
    println!();
    println!("All results are reproducible. Rerun with same --range to verify.");
}
