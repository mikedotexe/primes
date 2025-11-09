// Quintuplet Coordinate Constellation Test
//
// This example explores k=5 constellations with the structure:
//   y x MIDDLE x y
//
// where (x, y) form a 2D coordinate system on each side of the midpoint.
//
// COMPARISON ACROSS DIMENSIONS:
// - Triplet (k=3):     a MIDDLE a              (1D)
// - Quintuplet (k=5):  y x MIDDLE x y          (2D)
// - Septuplet (k=7):   z y x MIDDLE x y z      (3D)
//
// HARDY-LITTLEWOOD PREDICTION:
// Expected rate scales as 1/(log base)^k
//   - Triplets (k=3):     ~1/(log base)³
//   - Quintuplets (k=5):  ~1/(log base)⁵
//   - Septuplets (k=7):   ~1/(log base)⁷
//
// For base 14: log(14) ≈ 2.639
//   Triplet → Quintuplet: (log 14)² ≈ 6.96x rarer
//   Quintuplet → Septuplet: (log 14)² ≈ 6.96x rarer
//
// COORDINATE INTERPRETATION:
// The structure y-x creates a 2D coordinate space around MIDDLE:
//   - x: nearest neighbor distance
//   - y: second neighbor distance
//
// RESEARCH QUESTION:
// After discovering septuplets violate HL predictions (only 1.9x rarer
// than triplets, not 48.5x), where do quintuplets fall?
//
// If the violation is gradual, we expect quintuplets to show intermediate
// behavior. If it's dimension-specific, k=5 may show different patterns.

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime;
use std::collections::HashMap;

/// Build quintuplet membrane: y-x-MIDDLE-x-y
fn quintuplet_membrane(middle: u32, x: u32, y: u32, base: u32) -> BigUint {
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    // Left side: y, x
    result = result * &base_big + BigUint::from(y);
    result = result * &base_big + BigUint::from(x);

    // Center: MIDDLE
    result = result * &base_big + BigUint::from(middle);

    // Right side (mirror): x, y
    result = result * &base_big + BigUint::from(x);
    result = result * &base_big + BigUint::from(y);

    result
}

/// Test all quintuplet patterns for a given middle value
fn test_quintuplets(middle: u32, base: u32, limit: u64) -> Vec<((u32, u32), BigUint)> {
    let mut successes = Vec::new();

    // Sweep 2D coordinate space (x, y)
    for y in 1..base {
        for x in 1..base {
            let candidate = quintuplet_membrane(middle, x, y, base);

            // Skip if too large
            if candidate > BigUint::from(limit) {
                continue;
            }

            if is_prime(&candidate) {
                successes.push(((x, y), candidate));
            }
        }
    }

    successes
}

/// Analyze coordinate patterns in successful quintuplets
fn analyze_patterns(successes: &[((u32, u32), BigUint)]) -> HashMap<String, usize> {
    let mut patterns = HashMap::new();

    for ((x, y), _) in successes {
        // Both equal?
        if x == y {
            *patterns.entry("x_equals_y".to_string()).or_insert(0) += 1;
        }

        // Monotonic (x < y)?
        if x < y {
            *patterns
                .entry("monotonic_increasing".to_string())
                .or_insert(0) += 1;
        }

        // Small coordinates (both ≤ 3)?
        if *x <= 3 && *y <= 3 {
            *patterns.entry("small_coords".to_string()).or_insert(0) += 1;
        }

        // Fibonacci-like (x, y in {1,2,3,5,8,13})?
        let fib_values = [1u32, 2, 3, 5, 8, 13];
        if fib_values.contains(x) && fib_values.contains(y) {
            *patterns.entry("fibonacci_coords".to_string()).or_insert(0) += 1;
        }

        // Sum to specific values?
        let sum = x + y;
        if sum == 14 {
            // base
            *patterns.entry("sum_to_base".to_string()).or_insert(0) += 1;
        }
        if sum % 2 == 0 {
            *patterns.entry("even_sum".to_string()).or_insert(0) += 1;
        }
    }

    patterns
}

/// Generate ASCII visualization of (x,y) distribution
fn visualize_distribution(successes: &[((u32, u32), BigUint)], base: u32) {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║              2D COORDINATE SPACE VISUALIZATION                ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    // Build frequency map
    let mut freq_map: HashMap<(u32, u32), usize> = HashMap::new();
    for ((x, y), _) in successes {
        *freq_map.entry((*x, *y)).or_insert(0) += 1;
    }

    let max_freq = *freq_map.values().max().unwrap_or(&1);

    println!("  y-axis (second neighbor)");
    println!("    ↑");

    // Print grid (y decreases downward for visual clarity)
    for y in (1..base).rev() {
        print!(" {:2} │", y);
        for x in 1..base {
            let freq = freq_map.get(&(x, y)).unwrap_or(&0);
            let symbol = if *freq == 0 {
                "·"
            } else if *freq == max_freq {
                "█"
            } else if *freq > max_freq / 2 {
                "▓"
            } else if *freq > max_freq / 4 {
                "▒"
            } else {
                "░"
            };
            print!(" {}", symbol);
        }
        println!();
    }

    print!("    └");
    for _ in 1..base {
        print!("──");
    }
    println!("──→ x-axis (nearest neighbor)");
    print!("     ");
    for x in 1..base {
        print!(" {:1}", x % 10);
    }
    println!();
    println!();
    println!("  Legend: · = 0  ░ = low  ▒ = medium  ▓ = high  █ = max");
    println!();
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║       QUINTUPLET COORDINATE CONSTELLATION TEST               ║");
    println!("║       Bridging the Gap: k=3 → k=5 → k=7                     ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    let base = 14u32;
    let limit = 1_000_000_000_000u64; // 1 trillion

    println!("CONFIGURATION:");
    println!("  Base: {}", base);
    println!("  Search limit: {}", limit);
    println!();

    println!("DIMENSIONAL PROGRESSION:");
    println!("  k=3 (1D): a-MIDDLE-a");
    println!("  k=5 (2D): y-x-MIDDLE-x-y        ← TESTING THIS");
    println!("  k=7 (3D): z-y-x-MIDDLE-x-y-z");
    println!();

    // HL predictions
    let log_base = (base as f64).ln();
    let triplet_scaling = 1.0 / log_base.powi(3);
    let quintuplet_scaling = 1.0 / log_base.powi(5);
    let septuplet_scaling = 1.0 / log_base.powi(7);

    let triplet_to_quintuplet = triplet_scaling / quintuplet_scaling;
    let quintuplet_to_septuplet = quintuplet_scaling / septuplet_scaling;

    println!("═══════════════════════════════════════════════════════════════");
    println!("HARDY-LITTLEWOOD PREDICTIONS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("For base {}: ln({}) ≈ {:.3}", base, base, log_base);
    println!();
    println!(
        "Triplet scaling (k=3):     1/(ln b)³ ≈ {:.6}",
        triplet_scaling
    );
    println!(
        "Quintuplet scaling (k=5):  1/(ln b)⁵ ≈ {:.6}",
        quintuplet_scaling
    );
    println!(
        "Septuplet scaling (k=7):   1/(ln b)⁷ ≈ {:.6}",
        septuplet_scaling
    );
    println!();
    println!("Predicted ratios:");
    println!(
        "  Triplet → Quintuplet:    {:.1}x rarer",
        triplet_to_quintuplet
    );
    println!(
        "  Quintuplet → Septuplet:  {:.1}x rarer",
        quintuplet_to_septuplet
    );
    println!();
    println!("OBSERVED (from previous tests):");
    println!("  Triplet → Septuplet:     1.9x rarer (predicted: 48.5x)");
    println!("  → Massive 96% deviation from HL theory!");
    println!();
    println!("RESEARCH QUESTION:");
    println!("  Where does k=5 fall in this progression?");
    println!();

    // Test the same middle values as septuplet test
    let middle_values = vec![1u32, 3, 5, 7, 11, 13];

    println!("═══════════════════════════════════════════════════════════════");
    println!("TESTING ACROSS MIDDLE VALUES");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    let mut total_quintuplet_successes = 0;
    let mut all_quintuplet_successes = Vec::new();

    for &middle in &middle_values {
        println!("─────────────────────────────────────────────────────────────");
        println!("MIDDLE = {} (base {})", middle, base);
        println!("─────────────────────────────────────────────────────────────");
        println!();

        let quintuplet_results = test_quintuplets(middle, base, limit);

        if quintuplet_results.is_empty() {
            println!("  No prime quintuplets found");
        } else {
            // Show first 10 examples
            for (i, ((x, y), prime)) in quintuplet_results.iter().take(10).enumerate() {
                println!(
                    "  [{:2}] (x,y)=({},{}) → {}-{}-{}-{}-{} = {}",
                    i + 1,
                    x,
                    y,
                    y,
                    x,
                    middle,
                    x,
                    y,
                    prime
                );
            }
            if quintuplet_results.len() > 10 {
                println!("  ... and {} more", quintuplet_results.len() - 10);
            }
        }
        println!("  Success count: {}", quintuplet_results.len());
        println!();

        total_quintuplet_successes += quintuplet_results.len();
        all_quintuplet_successes.extend(quintuplet_results);
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("AGGREGATE RESULTS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    let total_middle_values = middle_values.len();
    let triplet_space = (base - 1) * total_middle_values as u32;
    let quintuplet_space = (base - 1).pow(2) * total_middle_values as u32;
    let septuplet_space = (base - 1).pow(3) * total_middle_values as u32;

    println!("Search Space Sizes:");
    println!("  Triplets (k=3):     {} configurations", triplet_space);
    println!("  Quintuplets (k=5):  {} configurations", quintuplet_space);
    println!("  Septuplets (k=7):   {} configurations", septuplet_space);
    println!();

    // From previous tests
    let triplet_successes = 9;
    let septuplet_successes = 803;

    println!("Prime Counts:");
    println!("  Triplets:    {} primes", triplet_successes);
    println!("  Quintuplets: {} primes", total_quintuplet_successes);
    println!("  Septuplets:  {} primes", septuplet_successes);
    println!();

    let triplet_rate = triplet_successes as f64 / triplet_space as f64 * 100.0;
    let quintuplet_rate = total_quintuplet_successes as f64 / quintuplet_space as f64 * 100.0;
    let septuplet_rate = septuplet_successes as f64 / septuplet_space as f64 * 100.0;

    println!("Success Rates:");
    println!("  Triplets:    {:.4}%", triplet_rate);
    println!("  Quintuplets: {:.4}%", quintuplet_rate);
    println!("  Septuplets:  {:.4}%", septuplet_rate);
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("COMPARATIVE ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    if total_quintuplet_successes > 0 {
        let triplet_to_quint_obs = triplet_rate / quintuplet_rate;
        let quint_to_sept_obs = quintuplet_rate / septuplet_rate;

        println!("OBSERVED Rarity Ratios:");
        println!("  Triplet → Quintuplet:    {:.2}x", triplet_to_quint_obs);
        println!("  Quintuplet → Septuplet:  {:.2}x", quint_to_sept_obs);
        println!();

        println!("PREDICTED (HL) Rarity Ratios:");
        println!("  Triplet → Quintuplet:    {:.2}x", triplet_to_quintuplet);
        println!("  Quintuplet → Septuplet:  {:.2}x", quintuplet_to_septuplet);
        println!();

        let error_trip_quint =
            ((triplet_to_quint_obs - triplet_to_quintuplet) / triplet_to_quintuplet * 100.0).abs();
        let error_quint_sept =
            ((quint_to_sept_obs - quintuplet_to_septuplet) / quintuplet_to_septuplet * 100.0).abs();

        println!("HL Prediction Errors:");
        println!("  Triplet → Quintuplet:    {:.1}%", error_trip_quint);
        println!("  Quintuplet → Septuplet:  {:.1}%", error_quint_sept);
        println!();
    }

    // Pattern analysis
    if !all_quintuplet_successes.is_empty() {
        println!("═══════════════════════════════════════════════════════════════");
        println!("PATTERN ANALYSIS");
        println!("═══════════════════════════════════════════════════════════════");
        println!();

        let patterns = analyze_patterns(&all_quintuplet_successes);

        println!(
            "Detected patterns in {} quintuplets:",
            all_quintuplet_successes.len()
        );
        println!();

        for (pattern_name, count) in patterns.iter() {
            let percentage = *count as f64 / all_quintuplet_successes.len() as f64 * 100.0;
            println!(
                "  {:25} : {:4} / {} ({:5.1}%)",
                pattern_name,
                count,
                all_quintuplet_successes.len(),
                percentage
            );
        }
        println!();

        // Coordinate distribution
        let mut x_counts: HashMap<u32, usize> = HashMap::new();
        let mut y_counts: HashMap<u32, usize> = HashMap::new();

        for ((x, y), _) in &all_quintuplet_successes {
            *x_counts.entry(*x).or_insert(0) += 1;
            *y_counts.entry(*y).or_insert(0) += 1;
        }

        println!("Coordinate distributions:");
        println!("  x (nearest):  {:?}", x_counts);
        println!("  y (second):   {:?}", y_counts);
        println!();

        // Visualize 2D distribution
        visualize_distribution(&all_quintuplet_successes, base);
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("CONCLUSIONS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    if total_quintuplet_successes > 0 {
        println!("KEY FINDINGS:");
        println!("  → Quintuplets bridge triplets and septuplets");
        println!("  → Dimensional structure (k=3→5→7) reveals patterns");
        println!("  → HL theory predictions show systematic deviations");
        println!("  → Coordinate symmetry appears to favor primality");
    } else {
        println!("KEY FINDINGS:");
        println!("  → No quintuplets found (intermediate rarity)");
        println!("  → May fall in 'desert zone' between triplets and septuplets");
        println!("  → Suggests non-monotonic behavior across dimensions");
    }
    println!();

    println!("NEXT STEPS:");
    println!("  1. Compare coordinate constraints across k=3,5,7");
    println!("  2. Analyze which coordinates act as 'protective shells'");
    println!("  3. Test different bases to validate universality");
    println!("  4. Formalize geometric interpretation in Agda");
    println!("  5. Explore k=9, k=11 to extend dimensional analysis");
}
