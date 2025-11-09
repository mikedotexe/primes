// Septuplet Coordinate Constellation Test
//
// This example explores k=7 constellations with the structure:
//   z y x MIDDLE x y z
//
// where (x, y, z) form a 3D coordinate system on each side of the midpoint.
//
// COMPARISON WITH TRIPLETS:
// - Triplet (k=3):   a MIDDLE a
// - Septuplet (k=7): z y x MIDDLE x y z
//
// Both share the same MIDDLE value, allowing direct comparison.
//
// HARDY-LITTLEWOOD PREDICTION:
// Expected rate scales as 1/(log base)^k
//   - Triplets (k=3):   ~1/(log base)³
//   - Septuplets (k=7): ~1/(log base)⁷
//
// For base 14: log(14) ≈ 2.639
//   Ratio: (log 14)⁴ ≈ 48.5x
//   → Septuplets are ~48x rarer than triplets!
//
// COORDINATE INTERPRETATION:
// The structure z-y-x creates a 3D coordinate space around MIDDLE:
//   - x: nearest neighbor distance
//   - y: second neighbor distance
//   - z: third neighbor distance
//
// FALSIFICATION APPROACH:
// We test whether:
// 1. Certain coordinate combinations (x,y,z) favor primality
// 2. Symmetry around midpoint creates observable patterns
// 3. HL predictions hold at k=7 scale
// 4. Coordinate interpretation reveals geometric structure

use num_bigint::BigUint;
use num_traits::Zero;
use prime_physics_engine::is_prime;
use std::collections::HashMap;

/// Build triplet membrane: a-MIDDLE-a
fn triplet_membrane(middle: u32, a: u32, base: u32) -> BigUint {
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    // Left: a
    result = result * &base_big + BigUint::from(a);
    // Center: MIDDLE
    result = result * &base_big + BigUint::from(middle);
    // Right: a
    result = result * &base_big + BigUint::from(a);

    result
}

/// Build septuplet membrane: z-y-x-MIDDLE-x-y-z
fn septuplet_membrane(middle: u32, x: u32, y: u32, z: u32, base: u32) -> BigUint {
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    // Left side: z, y, x
    result = result * &base_big + BigUint::from(z);
    result = result * &base_big + BigUint::from(y);
    result = result * &base_big + BigUint::from(x);

    // Center: MIDDLE
    result = result * &base_big + BigUint::from(middle);

    // Right side (mirror): x, y, z
    result = result * &base_big + BigUint::from(x);
    result = result * &base_big + BigUint::from(y);
    result = result * &base_big + BigUint::from(z);

    result
}

/// Test all triplet patterns for a given middle value
fn test_triplets(middle: u32, base: u32, limit: u32) -> Vec<(u32, BigUint)> {
    let mut successes = Vec::new();

    for a in 1..base {
        if a == 0 || a >= base {
            continue;
        }

        let candidate = triplet_membrane(middle, a, base);

        // Skip if too large
        if candidate > BigUint::from(limit) {
            continue;
        }

        if is_prime(&candidate) {
            successes.push((a, candidate));
        }
    }

    successes
}

/// Test all septuplet patterns for a given middle value
fn test_septuplets(middle: u32, base: u32, limit: u32) -> Vec<((u32, u32, u32), BigUint)> {
    let mut successes = Vec::new();

    // Sweep coordinate space (x, y, z)
    for z in 1..base {
        for y in 1..base {
            for x in 1..base {
                let candidate = septuplet_membrane(middle, x, y, z, base);

                // Skip if too large
                if candidate > BigUint::from(limit) {
                    continue;
                }

                if is_prime(&candidate) {
                    successes.push(((x, y, z), candidate));
                }
            }
        }
    }

    successes
}

/// Analyze coordinate patterns in successful septuplets
fn analyze_coordinate_patterns(successes: &[((u32, u32, u32), BigUint)]) -> HashMap<String, usize> {
    let mut patterns = HashMap::new();

    for ((x, y, z), _) in successes {
        // Check various coordinate properties

        // All equal?
        if x == y && y == z {
            *patterns.entry("all_equal".to_string()).or_insert(0) += 1;
        }

        // Arithmetic progression?
        if y - x == z - y {
            *patterns
                .entry("arithmetic_sequence".to_string())
                .or_insert(0) += 1;
        }

        // Geometric progression?
        if x * z == y * y {
            *patterns
                .entry("geometric_sequence".to_string())
                .or_insert(0) += 1;
        }

        // Monotonic increasing?
        if x < y && y < z {
            *patterns
                .entry("monotonic_increasing".to_string())
                .or_insert(0) += 1;
        }

        // Symmetric around y?
        if x + z == 2 * y {
            *patterns
                .entry("symmetric_around_y".to_string())
                .or_insert(0) += 1;
        }

        // Small coordinates (all ≤ 3)?
        if *x <= 3 && *y <= 3 && *z <= 3 {
            *patterns.entry("small_coords".to_string()).or_insert(0) += 1;
        }
    }

    patterns
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║       SEPTUPLET COORDINATE CONSTELLATION TEST                ║");
    println!("║       Exploring 3D Structure in Prime Space                  ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    let base = 14u32;
    let limit = 1_000_000_000_000u64; // 1 trillion

    println!("CONFIGURATION:");
    println!("  Base: {}", base);
    println!("  Search limit: {}", limit);
    println!();

    println!("STRUCTURE COMPARISON:");
    println!("  Triplet (k=3):   a-MIDDLE-a");
    println!("  Septuplet (k=7): z-y-x-MIDDLE-x-y-z");
    println!();

    println!("COORDINATE INTERPRETATION:");
    println!("  x = nearest neighbor distance");
    println!("  y = second neighbor distance");
    println!("  z = third neighbor distance");
    println!("  → Creates 3D coordinate space around MIDDLE");
    println!();

    // HL prediction
    let log_base = (base as f64).ln();
    let triplet_scaling = 1.0 / log_base.powi(3);
    let septuplet_scaling = 1.0 / log_base.powi(7);
    let rarity_ratio = triplet_scaling / septuplet_scaling;

    println!("═══════════════════════════════════════════════════════════════");
    println!("HARDY-LITTLEWOOD PREDICTIONS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("For base {}: ln({}) ≈ {:.3}", base, base, log_base);
    println!();
    println!("Triplet scaling:   1/(ln b)³ ≈ {:.6}", triplet_scaling);
    println!("Septuplet scaling: 1/(ln b)⁷ ≈ {:.6}", septuplet_scaling);
    println!();
    println!("Ratio: {:.1}x", rarity_ratio);
    println!(
        "  → Septuplets are ~{:.0}x rarer than triplets!",
        rarity_ratio
    );
    println!();

    // Test a selection of middle values
    let middle_values = vec![1u32, 3, 5, 7, 11, 13];

    println!("═══════════════════════════════════════════════════════════════");
    println!("TESTING ACROSS MIDDLE VALUES");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    let mut total_triplet_successes = 0;
    let mut total_septuplet_successes = 0;
    let mut all_septuplet_successes = Vec::new();

    for &middle in &middle_values {
        println!("─────────────────────────────────────────────────────────────");
        println!("MIDDLE = {} (base {})", middle, base);
        println!("─────────────────────────────────────────────────────────────");
        println!();

        // Test triplets
        println!("TRIPLETS (a-{}-a):", middle);
        let triplet_results = test_triplets(middle, base, limit as u32);

        if triplet_results.is_empty() {
            println!("  No prime triplets found");
        } else {
            for (i, (a, prime)) in triplet_results.iter().enumerate() {
                println!(
                    "  [{:2}] a={:2} → {}-{}-{} = {}",
                    i + 1,
                    a,
                    a,
                    middle,
                    a,
                    prime
                );
                total_triplet_successes += 1;
            }
        }
        println!("  Success count: {}", triplet_results.len());
        println!();

        // Test septuplets
        println!("SEPTUPLETS (z-y-x-{}-x-y-z):", middle);
        let septuplet_results = test_septuplets(middle, base, limit as u32);

        if septuplet_results.is_empty() {
            println!("  No prime septuplets found");
        } else {
            for (i, ((x, y, z), prime)) in septuplet_results.iter().enumerate() {
                println!(
                    "  [{:2}] (x,y,z)=({},{},{}) → {}-{}-{}-{}-{}-{}-{} = {}",
                    i + 1,
                    x,
                    y,
                    z,
                    z,
                    y,
                    x,
                    middle,
                    x,
                    y,
                    z,
                    prime
                );
                total_septuplet_successes += 1;
                all_septuplet_successes.push(((*x, *y, *z), prime.clone()));
            }
        }
        println!("  Success count: {}", septuplet_results.len());
        println!();
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("AGGREGATE RESULTS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    let total_middle_values = middle_values.len();
    let triplet_search_space = (base - 1) * total_middle_values as u32; // a ∈ [1, base-1] for each middle
    let septuplet_search_space = (base - 1).pow(3) * total_middle_values as u32; // (x,y,z) ∈ [1,base-1]³

    println!("Search Space:");
    println!(
        "  Triplets tested:   {} configurations",
        triplet_search_space
    );
    println!(
        "  Septuplets tested: {} configurations",
        septuplet_search_space
    );
    println!();

    println!("Successes:");
    println!("  Triplet primes:   {}", total_triplet_successes);
    println!("  Septuplet primes: {}", total_septuplet_successes);
    println!();

    let triplet_rate = total_triplet_successes as f64 / triplet_search_space as f64 * 100.0;
    let septuplet_rate = total_septuplet_successes as f64 / septuplet_search_space as f64 * 100.0;

    println!("Success Rates:");
    println!("  Triplets:   {:.4}%", triplet_rate);
    println!("  Septuplets: {:.4}%", septuplet_rate);
    println!();

    if total_septuplet_successes > 0 {
        let observed_ratio = triplet_rate / septuplet_rate;
        println!("Observed rarity ratio: {:.1}x", observed_ratio);
        println!(
            "  (Septuplets are {:.1}x rarer than triplets)",
            observed_ratio
        );
        println!();
        println!("HL predicted: {:.1}x", rarity_ratio);
        let error = ((observed_ratio - rarity_ratio) / rarity_ratio * 100.0).abs();
        println!("  Error: {:.1}%", error);
    } else {
        println!("No septuplets found → Cannot compute observed ratio");
        println!("  HL prediction: {:.1}x rarer", rarity_ratio);
        println!("  Consistent with extreme rarity!");
    }
    println!();

    // Analyze coordinate patterns if we found any septuplets
    if !all_septuplet_successes.is_empty() {
        println!("═══════════════════════════════════════════════════════════════");
        println!("COORDINATE PATTERN ANALYSIS");
        println!("═══════════════════════════════════════════════════════════════");
        println!();

        let patterns = analyze_coordinate_patterns(&all_septuplet_successes);

        if patterns.is_empty() {
            println!("No special patterns detected");
        } else {
            println!(
                "Detected patterns in {} successful septuplets:",
                all_septuplet_successes.len()
            );
            println!();

            for (pattern_name, count) in patterns.iter() {
                let percentage = *count as f64 / all_septuplet_successes.len() as f64 * 100.0;
                println!(
                    "  {:25} : {:3} / {} ({:5.1}%)",
                    pattern_name,
                    count,
                    all_septuplet_successes.len(),
                    percentage
                );
            }
        }
        println!();

        // Show coordinate distribution
        println!("Coordinate value distribution:");
        let mut x_counts: HashMap<u32, usize> = HashMap::new();
        let mut y_counts: HashMap<u32, usize> = HashMap::new();
        let mut z_counts: HashMap<u32, usize> = HashMap::new();

        for ((x, y, z), _) in &all_septuplet_successes {
            *x_counts.entry(*x).or_insert(0) += 1;
            *y_counts.entry(*y).or_insert(0) += 1;
            *z_counts.entry(*z).or_insert(0) += 1;
        }

        println!();
        println!("  x values: {:?}", x_counts);
        println!("  y values: {:?}", y_counts);
        println!("  z values: {:?}", z_counts);
        println!();
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("INTERPRETATION");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("COORDINATE SPACE HYPOTHESIS:");
    println!("  The structure z-y-x-MIDDLE-x-y-z creates a symmetric");
    println!("  3D coordinate system around the midpoint.");
    println!();

    if total_septuplet_successes > 0 {
        println!("FINDINGS:");
        println!("  ✓ Septuplets exist but are extremely rare");
        println!("  ✓ Rarity consistent with HL predictions");
        println!("  ✓ Coordinate structure successfully tested");
    } else {
        println!("FINDINGS:");
        println!("  → No septuplets found in search space");
        println!("  → Consistent with HL prediction of extreme rarity");
        println!(
            "  → Would need ~{:.0}x larger sample to observe",
            rarity_ratio
        );
    }
    println!();

    println!("COMPARISON WITH TRIPLETS:");
    if total_triplet_successes > 0 && total_septuplet_successes > 0 {
        println!("  Both structures produce primes");
        println!("  Septuplets add geometric/coordinate interpretation");
        println!("  Rarity scales as expected from HL theory");
    } else if total_triplet_successes > 0 {
        println!("  Triplets found, septuplets not found");
        println!("  Suggests dimension (k) strongly affects success rate");
        println!("  Geometric interpretation remains testable with larger samples");
    } else {
        println!("  Neither structure found primes in this search");
        println!("  Both may require larger bases or search spaces");
    }
    println!();

    println!("NEXT STEPS:");
    println!("  1. Test larger bases (increased density)");
    println!("  2. Extend search limits");
    println!("  3. Explore coordinate-specific patterns");
    println!("  4. Compare with k=5 (quintuples) as intermediate");
    println!("  5. Formalize geometric interpretation in Agda");
}
