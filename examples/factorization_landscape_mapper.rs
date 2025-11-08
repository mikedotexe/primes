//! Factorization Landscape Mapper
//!
//! **THE CENTRAL QUESTION**: Why does base 6 win?
//!
//! # The Goldilocks Hypothesis
//!
//! We observe:
//! - Base 6 (2×3): 33% success - CHAMPION
//! - Base 10 (2×5): 18.5% success - Good
//! - Base 30 (2×3×5): 30% success - Excellent
//!
//! But WHY? Is there an optimal "factorization structure" that predicts success?
//!
//! # The Experiment
//!
//! Test bases 2-50 systematically and measure:
//!
//! 1. **Factorization Properties**:
//!    - ω(n): Number of distinct prime factors
//!    - Ω(n): Total prime factors (with multiplicity)
//!    - τ(n): Number of divisors
//!    - Largest prime factor
//!    - Is semiprime? Is prime power?
//!
//! 2. **Membrane Success**:
//!    - Test standard coprime boundary digit pairs
//!    - Measure success rate across multiple seeds
//!    - Record optimal configuration
//!
//! 3. **Correlation Analysis**:
//!    - Does ω(n) predict success?
//!    - Do semiprimes (ω=2) dominate?
//!    - Is there a divisor count sweet spot?
//!    - Can we predict success from structure alone?
//!
//! # Example Output
//!
//! ```text
//! ╔══════════════════════════════════════════════════════════════════╗
//! ║           FACTORIZATION LANDSCAPE ANALYSIS                       ║
//! ╚══════════════════════════════════════════════════════════════════╝
//!
//! Base  Factors      ω  Ω  τ   Success  Best Config
//! ────────────────────────────────────────────────────────────────
//!   6   2×3          2  2  4    33.0%   (1,5) k=(0,0)  🏆
//!  10   2×5          2  2  4    18.5%   (3,7) k=(0,0)
//!  30   2×3×5        3  3  8    30.0%   (11,7) k=(0,0)
//!  12   2²×3         2  3  6    ??%     Testing...
//!
//! CORRELATIONS:
//!   ω(n) vs success:  r = +0.42
//!   τ(n) vs success:  r = -0.31
//!   Semiprime bonus:  +12% average
//!
//! PREDICTION MODEL:
//!   success ≈ f(ω, τ, largest_factor)
//! ```
//!
//! # Usage
//!
//! ```bash
//! # Full landscape scan (bases 2-50)
//! cargo run --example factorization_landscape_mapper
//!
//! # Focus on specific range
//! cargo run --example factorization_landscape_mapper -- --range=2:20
//!
//! # More seeds for accuracy
//! cargo run --example factorization_landscape_mapper -- --seeds=100
//!
//! # Quick scan (fewer seeds)
//! cargo run --example factorization_landscape_mapper -- --seeds=10 --range=2:30
//! ```

use prime_physics_engine::is_prime;
use num_bigint::BigUint;
use std::collections::HashMap;

// ============================================================================
// FACTORIZATION ANALYSIS
// ============================================================================

#[derive(Debug, Clone)]
struct BaseFactorization {
    base: u64,
    prime_factors: Vec<u64>,          // List of prime factors with multiplicity
    distinct_factors: Vec<u64>,       // Unique prime factors
    omega: usize,                      // ω(n): count of distinct prime factors
    big_omega: usize,                  // Ω(n): count with multiplicity
    tau: usize,                        // τ(n): count of divisors
    largest_prime_factor: u64,
    is_prime: bool,
    is_semiprime: bool,                // Exactly two prime factors (counted with multiplicity)
    is_prime_power: bool,              // p^k for some prime p
}

fn prime_factorize(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();

    // Handle 2
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    // Handle odd factors
    let mut d = 3;
    while d * d <= n {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
        d += 2;
    }

    // If n > 1, it's a prime factor
    if n > 1 {
        factors.push(n);
    }

    factors
}

fn count_divisors(n: u64) -> usize {
    if n == 0 { return 0; }
    if n == 1 { return 1; }

    let factors = prime_factorize(n);
    let mut exponent_map: HashMap<u64, usize> = HashMap::new();

    for &p in &factors {
        *exponent_map.entry(p).or_insert(0) += 1;
    }

    // τ(n) = ∏(e_i + 1) where n = ∏(p_i^e_i)
    exponent_map.values().map(|&exp| exp + 1).product()
}

fn analyze_base(base: u64) -> BaseFactorization {
    let factors = prime_factorize(base);
    let mut distinct: Vec<u64> = factors.clone();
    distinct.sort();
    distinct.dedup();

    let omega = distinct.len();
    let big_omega = factors.len();
    let tau = count_divisors(base);
    let largest = *factors.iter().max().unwrap_or(&base);

    let is_prime = factors.len() == 1 && factors[0] == base;
    let is_semiprime = big_omega == 2;
    let is_prime_power = omega == 1;

    BaseFactorization {
        base,
        prime_factors: factors,
        distinct_factors: distinct,
        omega,
        big_omega,
        tau,
        largest_prime_factor: largest,
        is_prime,
        is_semiprime,
        is_prime_power,
    }
}

// ============================================================================
// MEMBRANE GENERATION & TESTING
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
    if n == 0 { return vec![0]; }

    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % base);
        n /= base;
    }
    digits.reverse();
    digits
}

fn from_base_digits(digits: &[u64], base: u64) -> u64 {
    digits.iter().fold(0u64, |acc, &d| acc * base + d)
}

fn generate_membrane(base: u64, outer: u64, inner: u64, seed: u64, k1: usize, k2: usize) -> Option<u64> {
    if outer >= base || inner >= base || seed >= base {
        return None;
    }

    let mut digits = Vec::new();

    // Left side: outer + k1 zeros + inner + k2 zeros
    digits.push(outer);
    for _ in 0..k1 {
        digits.push(0);
    }
    digits.push(inner);
    for _ in 0..k2 {
        digits.push(0);
    }

    // Seed
    let seed_digits = to_base_digits(seed, base);
    digits.extend(&seed_digits);

    // Right side: k2 zeros + inner + k1 zeros + outer (mirror)
    for _ in 0..k2 {
        digits.push(0);
    }
    digits.push(inner);
    for _ in 0..k1 {
        digits.push(0);
    }
    digits.push(outer);

    Some(from_base_digits(&digits, base))
}

#[derive(Debug, Clone)]
struct MembraneConfig {
    outer: u64,
    inner: u64,
    k1: usize,
    k2: usize,
}

impl std::fmt::Display for MembraneConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{}) k=({},{})", self.outer, self.inner, self.k1, self.k2)
    }
}

fn test_membrane_config(base: u64, config: &MembraneConfig, num_seeds: usize) -> (usize, usize) {
    let mut successes = 0;
    let mut total = 0;

    for seed in 1..=num_seeds as u64 {
        if let Some(n) = generate_membrane(base, config.outer, config.inner, seed, config.k1, config.k2) {
            if n > 1 && is_prime(&BigUint::from(n)) {
                successes += 1;
            }
            total += 1;
        }
    }

    (successes, total)
}

fn find_best_config_for_base(base: u64, num_seeds: usize) -> (MembraneConfig, f64) {
    let mut best_config = MembraneConfig { outer: 1, inner: 1, k1: 0, k2: 0 };
    let mut best_rate = 0.0;

    // Test coprime digit pairs with minimal padding
    for outer in 1..base.min(15) {
        if gcd(outer, base) != 1 { continue; }

        for inner in 1..base.min(15) {
            if gcd(inner, base) != 1 { continue; }
            if inner == outer { continue; }

            // Focus on k=(0,0) as we know it's optimal
            for k1 in 0..=1 {
                for k2 in 0..=1 {
                    let config = MembraneConfig { outer, inner, k1, k2 };
                    let (succ, tot) = test_membrane_config(base, &config, num_seeds);

                    if tot > 0 {
                        let rate = succ as f64 / tot as f64;
                        if rate > best_rate {
                            best_rate = rate;
                            best_config = config.clone();
                        }
                    }
                }
            }
        }
    }

    (best_config, best_rate)
}

// ============================================================================
// LANDSCAPE ANALYSIS
// ============================================================================

#[derive(Debug, Clone)]
struct LandscapeEntry {
    factorization: BaseFactorization,
    best_config: MembraneConfig,
    success_rate: f64,
    samples_tested: usize,
}

fn scan_landscape(start_base: u64, end_base: u64, seeds_per_base: usize) -> Vec<LandscapeEntry> {
    let mut entries = Vec::new();

    println!("Scanning bases {} to {} with {} seeds each...\n", start_base, end_base, seeds_per_base);

    for base in start_base..=end_base {
        if base < 2 { continue; }

        print!("Base {:2}: ", base);
        use std::io::Write;
        std::io::stdout().flush().ok();

        let factorization = analyze_base(base);
        let (config, rate) = find_best_config_for_base(base, seeds_per_base);

        println!("{:6.1}% success with {}", rate * 100.0, config);

        entries.push(LandscapeEntry {
            factorization,
            best_config: config,
            success_rate: rate,
            samples_tested: seeds_per_base,
        });
    }

    entries
}

fn compute_correlation(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.is_empty() {
        return 0.0;
    }

    let n = x.len() as f64;
    let mean_x: f64 = x.iter().sum::<f64>() / n;
    let mean_y: f64 = y.iter().sum::<f64>() / n;

    let cov: f64 = x.iter().zip(y.iter())
        .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
        .sum::<f64>() / n;

    let std_x: f64 = (x.iter().map(|xi| (xi - mean_x).powi(2)).sum::<f64>() / n).sqrt();
    let std_y: f64 = (y.iter().map(|yi| (yi - mean_y).powi(2)).sum::<f64>() / n).sqrt();

    if std_x == 0.0 || std_y == 0.0 {
        return 0.0;
    }

    cov / (std_x * std_y)
}

fn analyze_correlations(entries: &[LandscapeEntry]) {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║                  CORRELATION ANALYSIS                            ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let success_rates: Vec<f64> = entries.iter().map(|e| e.success_rate).collect();

    // ω(n) correlation
    let omega_values: Vec<f64> = entries.iter().map(|e| e.factorization.omega as f64).collect();
    let r_omega = compute_correlation(&omega_values, &success_rates);
    println!("ω(n) (distinct factors) vs success:  r = {:+.3}", r_omega);

    // Ω(n) correlation
    let big_omega_values: Vec<f64> = entries.iter().map(|e| e.factorization.big_omega as f64).collect();
    let r_big_omega = compute_correlation(&big_omega_values, &success_rates);
    println!("Ω(n) (total factors)    vs success:  r = {:+.3}", r_big_omega);

    // τ(n) correlation
    let tau_values: Vec<f64> = entries.iter().map(|e| e.factorization.tau as f64).collect();
    let r_tau = compute_correlation(&tau_values, &success_rates);
    println!("τ(n) (divisor count)    vs success:  r = {:+.3}", r_tau);

    // Largest prime factor correlation
    let lpf_values: Vec<f64> = entries.iter().map(|e| e.factorization.largest_prime_factor as f64).collect();
    let r_lpf = compute_correlation(&lpf_values, &success_rates);
    println!("Largest prime factor    vs success:  r = {:+.3}", r_lpf);

    println!();

    // Category analysis
    let semiprimes: Vec<&LandscapeEntry> = entries.iter()
        .filter(|e| e.factorization.is_semiprime)
        .collect();

    let non_semiprimes: Vec<&LandscapeEntry> = entries.iter()
        .filter(|e| !e.factorization.is_semiprime)
        .collect();

    if !semiprimes.is_empty() {
        let semiprime_avg = semiprimes.iter().map(|e| e.success_rate).sum::<f64>() / semiprimes.len() as f64;
        println!("Semiprime bases (ω=2): avg = {:.1}%  (n={})", semiprime_avg * 100.0, semiprimes.len());
    }

    if !non_semiprimes.is_empty() {
        let non_semiprime_avg = non_semiprimes.iter().map(|e| e.success_rate).sum::<f64>() / non_semiprimes.len() as f64;
        println!("Non-semiprime bases:   avg = {:.1}%  (n={})", non_semiprime_avg * 100.0, non_semiprimes.len());
    }

    // Prime power analysis
    let prime_powers: Vec<&LandscapeEntry> = entries.iter()
        .filter(|e| e.factorization.is_prime_power && !e.factorization.is_prime)
        .collect();

    if !prime_powers.is_empty() {
        let pp_avg = prime_powers.iter().map(|e| e.success_rate).sum::<f64>() / prime_powers.len() as f64;
        println!("Prime power bases:     avg = {:.1}%  (n={})", pp_avg * 100.0, prime_powers.len());
    }
}

fn print_summary_table(entries: &[LandscapeEntry]) {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║              FACTORIZATION LANDSCAPE SUMMARY                     ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    println!("Base  Factorization      ω  Ω  τ    Success   Best Config");
    println!("────────────────────────────────────────────────────────────────────");

    let mut sorted = entries.to_vec();
    sorted.sort_by(|a, b| b.success_rate.partial_cmp(&a.success_rate).unwrap());

    for entry in sorted.iter().take(20) {
        let fact = &entry.factorization;

        // Format factorization string
        let mut factor_counts: HashMap<u64, usize> = HashMap::new();
        for &p in &fact.prime_factors {
            *factor_counts.entry(p).or_insert(0) += 1;
        }

        let mut factors_sorted: Vec<_> = factor_counts.iter().collect();
        factors_sorted.sort_by_key(|&(p, _)| p);

        let factor_str: String = factors_sorted.iter()
            .map(|(p, count)| {
                if **count == 1 {
                    format!("{}", p)
                } else {
                    format!("{}^{}", p, count)
                }
            })
            .collect::<Vec<_>>()
            .join("×");

        let trophy = if entry.success_rate > 0.30 { "🏆" } else { "" };

        println!("{:4}  {:16}  {:2} {:2} {:3}   {:5.1}%   {}  {}",
                 fact.base,
                 factor_str,
                 fact.omega,
                 fact.big_omega,
                 fact.tau,
                 entry.success_rate * 100.0,
                 entry.best_config,
                 trophy);
    }
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
    println!("║        FACTORIZATION LANDSCAPE MAPPER                            ║");
    println!("║                                                                  ║");
    println!("║  Question: Why does base 6 achieve 33% membrane success?        ║");
    println!("║  Method: Map factorization structure → membrane performance     ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    let (start, end) = if let Some(range_str) = parse_arg(&args, "--range") {
        let parts: Vec<&str> = range_str.split(':').collect();
        if parts.len() == 2 {
            if let (Ok(s), Ok(e)) = (parts[0].parse::<u64>(), parts[1].parse::<u64>()) {
                (s, e)
            } else {
                (2, 30)
            }
        } else {
            (2, 30)
        }
    } else {
        (2, 30)
    };

    let seeds = if let Some(s) = parse_arg(&args, "--seeds") {
        s.parse::<usize>().unwrap_or(50)
    } else {
        50
    };

    let entries = scan_landscape(start, end, seeds);

    print_summary_table(&entries);
    analyze_correlations(&entries);

    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║                    KEY INSIGHTS                                  ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    println!("Look for:");
    println!("  - Do semiprimes (p×q) dominate the leaderboard?");
    println!("  - Is there a divisor count sweet spot?");
    println!("  - Does ω(n)=2 predict high success?");
    println!("  - What makes base 6 special structurally?");
}
