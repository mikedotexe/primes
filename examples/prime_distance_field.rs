//! Prime Distance Field Visualizer
//!
//! **THE QUESTION**: How far is any integer from primality?
//!
//! # The Concept
//!
//! Instead of binary thinking (prime vs composite), we measure DISTANCE to primality
//! using multiple metrics. This creates a "field" showing:
//! - Where primes cluster (low distance regions)
//! - Prime deserts (high distance regions)
//! - Gradients (how distance changes)
//! - Patterns (membrane structures, palindromes, etc.)
//!
//! # The Metrics
//!
//! 1. **Absolute Distance**: Closest prime (simple)
//! 2. **Factor Distance**: Smallest prime factor
//! 3. **Hamming Distance**: Minimum digit changes to make prime
//! 4. **Coprime Distance**: Count of small prime divisors
//! 5. **Membrane Distance**: Edits needed to fit membrane pattern
//!
//! # Example
//!
//! ```text
//! Number: 100
//!
//! DISTANCE METRICS:
//!   Absolute:  3 (nearest: 97, 103)
//!   Factor:    2 (2² × 5²)
//!   Hamming:   1 (change to 101)
//!   Coprime:   2 (divisible by 2,5)
//!   Membrane:  No pattern match
//!
//! PRIMALITY GRADIENT: ████░░░░░░ 4/10
//! ```
//!
//! # Usage
//!
//! ```bash
//! # Single number analysis
//! cargo run --example prime_distance_field -- --number=100
//!
//! # Range visualization
//! cargo run --example prime_distance_field -- --range=90:110
//!
//! # Detect clusters
//! cargo run --example prime_distance_field -- --range=0:1000 --detect-clusters
//!
//! # Membrane analysis
//! cargo run --example prime_distance_field -- --range=2500:2600 --membrane
//! ```

use num_bigint::BigUint;
use prime_physics_engine::is_prime;

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn smallest_prime_factor(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    if n.is_multiple_of(2) {
        return 2;
    }

    let mut d = 3;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return d;
        }
        d += 2;
    }
    n // n is prime
}

// ============================================================================
// DISTANCE METRICS
// ============================================================================

#[derive(Debug, Clone)]
struct DistanceMetrics {
    number: u64,
    is_prime: bool,
    absolute_below: Option<u64>,
    absolute_above: Option<u64>,
    factor_distance: u64,
    hamming_distance: u64,
    coprime_distance: u64,
    membrane_distance: Option<u64>,
    membrane_match: Option<String>,
}

/// Compute absolute distance to nearest prime
fn compute_absolute_distances(n: u64, max_search: u64) -> (Option<u64>, Option<u64>) {
    let mut dist_below = None;
    let mut dist_above = None;

    // Search below
    for d in 0..=max_search.min(n) {
        if n >= d && is_prime(&BigUint::from(n - d)) {
            dist_below = Some(d);
            break;
        }
    }

    // Search above
    for d in 0..=max_search {
        if is_prime(&BigUint::from(n + d)) {
            dist_above = Some(d);
            break;
        }
    }

    (dist_below, dist_above)
}

/// Compute hamming distance (digit changes to make prime)
fn compute_hamming_distance(n: u64) -> u64 {
    if is_prime(&BigUint::from(n)) {
        return 0;
    }

    let s = n.to_string();

    // Try 1 digit change
    for (i, _) in s.chars().enumerate() {
        for digit in '0'..='9' {
            let mut chars: Vec<char> = s.chars().collect();
            chars[i] = digit;
            let new_str: String = chars.iter().collect();

            if let Ok(num) = new_str.parse::<u64>() {
                if num > 1 && is_prime(&BigUint::from(num)) {
                    return 1;
                }
            }
        }
    }

    // If no 1-change works, return 2 (we don't search exhaustively)
    2
}

/// Count how many small primes divide n
fn compute_coprime_distance(n: u64) -> u64 {
    if n <= 1 {
        return 0;
    }

    let small_primes = [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    small_primes
        .iter()
        .filter(|&&p| n.is_multiple_of(p))
        .count() as u64
}

/// Check if a number fits a membrane pattern
fn fits_membrane_pattern(n: u64, base: u64) -> Option<String> {
    if n < 10 {
        return None;
    }

    // Convert to base representation (simplified for common bases)
    let digits = to_base_digits(n, base);

    // Check for symmetric pattern: outer-inner-seed-inner-outer
    if digits.len() >= 5 {
        let len = digits.len();

        // Perfect symmetry check
        if digits[0] == digits[len - 1] && digits[1] == digits[len - 2] {
            // Check for known successful patterns
            let outer = digits[0];
            let inner = digits[1];

            // Known optimal configs
            let known_patterns = match base {
                6 => vec![(1, 5)],
                10 => vec![(3, 7), (1, 3)],
                30 => vec![(11, 7)],
                _ => vec![],
            };

            for (opt_outer, opt_inner) in known_patterns {
                if outer as u64 == opt_outer && inner as u64 == opt_inner {
                    return Some(format!("base {} ({},{})", base, outer, inner));
                }
            }

            // Generic symmetric pattern
            return Some(format!("base {} ({},{}) generic", base, outer, inner));
        }
    }

    None
}

/// Convert number to base representation (returns digits)
fn to_base_digits(mut n: u64, base: u64) -> Vec<u32> {
    if n == 0 {
        return vec![0];
    }

    let mut digits = Vec::new();
    while n > 0 {
        digits.push((n % base) as u32);
        n /= base;
    }
    digits.reverse();
    digits
}

/// Compute membrane distance
fn compute_membrane_distance(n: u64) -> (Option<u64>, Option<String>) {
    // Check common bases
    let bases = [6, 10, 30];

    for &base in &bases {
        if let Some(pattern) = fits_membrane_pattern(n, base) {
            return (Some(0), Some(pattern));
        }
    }

    // If doesn't fit any pattern, estimate edits needed
    // For now, return None (could implement edit distance calculation)
    (None, None)
}

/// Compute all distance metrics for a number
fn analyze_number(n: u64) -> DistanceMetrics {
    let is_prime_val = is_prime(&BigUint::from(n));
    let (below, above) = if is_prime_val {
        (Some(0), Some(0))
    } else {
        compute_absolute_distances(n, 100)
    };

    let factor_dist = if is_prime_val {
        0
    } else {
        smallest_prime_factor(n)
    };

    let hamming = compute_hamming_distance(n);
    let coprime = compute_coprime_distance(n);
    let (membrane_dist, membrane_pat) = compute_membrane_distance(n);

    DistanceMetrics {
        number: n,
        is_prime: is_prime_val,
        absolute_below: below,
        absolute_above: above,
        factor_distance: factor_dist,
        hamming_distance: hamming,
        coprime_distance: coprime,
        membrane_distance: membrane_dist,
        membrane_match: membrane_pat,
    }
}

// ============================================================================
// VISUALIZATION
// ============================================================================

fn distance_to_block(dist: Option<u64>) -> &'static str {
    match dist {
        Some(0) => "██", // Prime
        Some(1) => "▓▓", // Distance 1
        Some(2) => "▒▒", // Distance 2
        Some(3) => "░░", // Distance 3
        _ => "  ",       // Far or unknown
    }
}

fn print_single_analysis(metrics: &DistanceMetrics) {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!(
        "║              NUMBER ANALYSIS: {}                        ",
        metrics.number
    );
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    println!(
        "STATUS: {}",
        if metrics.is_prime {
            "PRIME"
        } else {
            "COMPOSITE"
        }
    );
    println!();

    println!("DISTANCE METRICS:");

    if let Some(below) = metrics.absolute_below {
        print!("  Absolute distance below: {}", below);
        if below > 0 {
            println!(" (prime at {})", metrics.number - below);
        } else {
            println!();
        }
    }

    if let Some(above) = metrics.absolute_above {
        print!("  Absolute distance above: {}", above);
        if above > 0 {
            println!(" (prime at {})", metrics.number + above);
        } else {
            println!();
        }
    }

    if !metrics.is_prime {
        println!(
            "  Factor distance:        {} (smallest prime factor)",
            metrics.factor_distance
        );
    }

    println!(
        "  Hamming distance:       {} (digit changes)",
        metrics.hamming_distance
    );
    println!(
        "  Coprime distance:       {} (small prime divisors)",
        metrics.coprime_distance
    );

    if let Some(pattern) = &metrics.membrane_match {
        println!("  Membrane pattern:       MATCH - {}", pattern);
    } else {
        println!("  Membrane pattern:       No match");
    }
    println!();

    // Overall score
    let total = metrics.absolute_below.unwrap_or(10)
        + metrics.absolute_above.unwrap_or(10)
        + metrics.hamming_distance * 2;

    let score = (30_u64.saturating_sub(total)) * 10 / 30;

    print!("PRIMALITY GRADIENT: ");
    for i in 0..10 {
        if i < score {
            print!("█");
        } else {
            print!("░");
        }
    }
    println!(" {}/10", score);
}

fn print_range_visualization(start: u64, end: u64) {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║        PRIME DISTANCE FIELD: Range [{}-{}]", start, end);
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    let mut metrics_vec = Vec::new();
    let mut primes = Vec::new();

    print!("Analyzing range");
    for n in start..=end {
        let m = analyze_number(n);
        if m.is_prime {
            primes.push(n);
        }
        metrics_vec.push(m);

        if (n - start).is_multiple_of(10) {
            print!(".");
            use std::io::Write;
            std::io::stdout().flush().ok();
        }
    }
    println!(" Done!");
    println!();

    // Print numbers
    print!("Numbers:  ");
    for n in start..=end {
        print!("{:3} ", n);
    }
    println!();

    // Print absolute distances
    print!("Distance: ");
    for m in &metrics_vec {
        let dist = m
            .absolute_below
            .unwrap_or(99)
            .min(m.absolute_above.unwrap_or(99));
        print!("{:3} ", dist);
    }
    println!();

    // Print heat map
    print!("Heat Map: ");
    for m in &metrics_vec {
        let dist = m
            .absolute_below
            .unwrap_or(99)
            .min(m.absolute_above.unwrap_or(99));
        print!("{} ", distance_to_block(Some(dist)));
    }
    println!();
    println!();

    // Print primes
    print!("Primes:   ");
    for n in start..=end {
        if primes.contains(&n) {
            print!("{}★ ", n);
        }
    }
    println!();
    println!();

    // Statistics
    println!("STATISTICS:");
    println!("  Total numbers: {}", end - start + 1);
    println!(
        "  Primes: {} ({:.1}%)",
        primes.len(),
        (primes.len() as f64) / ((end - start + 1) as f64) * 100.0
    );

    let distances: Vec<u64> = metrics_vec
        .iter()
        .filter(|m| !m.is_prime)
        .map(|m| {
            m.absolute_below
                .unwrap_or(99)
                .min(m.absolute_above.unwrap_or(99))
        })
        .collect();

    if !distances.is_empty() {
        let avg_dist = distances.iter().sum::<u64>() as f64 / distances.len() as f64;
        let max_dist = *distances.iter().max().unwrap();

        println!("  Average distance (composites): {:.1}", avg_dist);
        println!("  Max distance: {}", max_dist);
    }
}

fn detect_clusters(start: u64, end: u64, window_size: usize, threshold: f64) {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           CLUSTER/DESERT DETECTION                               ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    println!(
        "Scanning range [{}-{}] with window size {}...",
        start, end, window_size
    );
    println!();

    let mut metrics_vec = Vec::new();
    for n in start..=end {
        metrics_vec.push(analyze_number(n));
    }

    let mut clusters = Vec::new();
    let mut deserts = Vec::new();

    for window_start in 0..metrics_vec.len().saturating_sub(window_size) {
        let window = &metrics_vec[window_start..window_start + window_size];

        let distances: Vec<u64> = window
            .iter()
            .map(|m| {
                m.absolute_below
                    .unwrap_or(99)
                    .min(m.absolute_above.unwrap_or(99))
            })
            .collect();

        let avg_dist = distances.iter().sum::<u64>() as f64 / distances.len() as f64;
        let prime_count = window.iter().filter(|m| m.is_prime).count();

        if avg_dist < threshold {
            clusters.push((start + window_start as u64, avg_dist, prime_count));
        } else if avg_dist > threshold * 2.0 {
            deserts.push((start + window_start as u64, avg_dist, prime_count));
        }
    }

    if !clusters.is_empty() {
        println!("PRIME CLUSTERS DETECTED: {} regions", clusters.len());
        for (pos, avg_dist, primes) in clusters.iter().take(10) {
            println!(
                "  Position {}: avg_distance={:.1}, primes={}",
                pos, avg_dist, primes
            );
        }
        println!();
    }

    if !deserts.is_empty() {
        println!("PRIME DESERTS DETECTED: {} regions", deserts.len());
        for (pos, avg_dist, primes) in deserts.iter().take(10) {
            println!(
                "  Position {}: avg_distance={:.1}, primes={}",
                pos, avg_dist, primes
            );
        }
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
    println!("║           PRIME DISTANCE FIELD VISUALIZER                        ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    let detect_clusters_flag = args.iter().any(|a| a == "--detect-clusters");

    if let Some(number_str) = parse_arg(&args, "--number") {
        // Single number mode
        if let Ok(n) = number_str.parse::<u64>() {
            let metrics = analyze_number(n);
            print_single_analysis(&metrics);
        } else {
            println!("Error: Invalid number '{}'", number_str);
        }
    } else if let Some(range_str) = parse_arg(&args, "--range") {
        // Range mode
        let parts: Vec<&str> = range_str.split(':').collect();
        if parts.len() == 2 {
            if let (Ok(start), Ok(end)) = (parts[0].parse::<u64>(), parts[1].parse::<u64>()) {
                if detect_clusters_flag {
                    detect_clusters(start, end, 10, 2.5);
                } else {
                    print_range_visualization(start, end);
                }
            } else {
                println!("Error: Invalid range '{}'", range_str);
            }
        } else {
            println!("Error: Range format should be 'start:end'");
        }
    } else {
        // Default: analyze 90-110
        println!("No arguments provided. Running default visualization (90-110).");
        println!("Use --number=N for single number or --range=start:end for range.");
        println!();
        print_range_visualization(90, 110);
    }
}
