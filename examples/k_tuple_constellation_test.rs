// k-Tuple Constellation Test
//
// GOAL: Extend beyond pairs (k=2) to test triplets (k=3) and quadruplets (k=4)
//
// THEORY: Hardy-Littlewood predicts for k-tuple (p, p+g₁, p+g₂, ...):
//   Count ≈ S(pattern) × x/(log x)^k
//
// For membranes, we can generalize to multi-prime configurations:
//   - Pairs (k=2): We've validated extensively
//   - Triplets (k=3): NEW - tests if HL generalizes
//   - Quadruplets (k=4): Further validation
//
// HYPOTHESIS: If HL is correct, triplets should show:
//   1. Lower success than pairs (harder to find 3 primes than 2)
//   2. Success depends on S(pattern) singular series
//   3. Some patterns work better than others (admissibility)
//
// FALSIFICATION: If triplets DON'T follow HL predictions, our framework fails!

use num_bigint::BigUint;
use num_traits::Zero;
use prime_physics_engine::is_prime;

// Prime triplet patterns (examples from OEIS and HL theory)
#[derive(Debug, Clone)]
struct TripletPattern {
    name: &'static str,
    gaps: &'static [u32],  // Gaps from first prime
    base_multiplier: u32,  // How to compute base from p
}

// Standard admissible triplet patterns
const TRIPLET_PATTERNS: &[TripletPattern] = &[
    TripletPattern {
        name: "Cousin triplet (0,2,6)",
        gaps: &[0, 2, 6],
        base_multiplier: 3, // base ≈ 3p for symmetry
    },
    TripletPattern {
        name: "Sexy triplet (0,6,12)",
        gaps: &[0, 6, 12],
        base_multiplier: 3,
    },
    TripletPattern {
        name: "Prime triplet (0,4,6)",
        gaps: &[0, 4, 6],
        base_multiplier: 3,
    },
];

// Generate k-tuple membrane number
// Structure: boundary-...-p1-...-p2-...-p3-...-seed-...-p3-...-p2-...-p1-...-boundary
fn k_tuple_membrane(primes: &[u32], seed: u32, base: u32) -> BigUint {
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    // Left half: boundary, then primes in order
    for &p in primes {
        result = result * &base_big + BigUint::from(p);
    }

    // Center seed
    result = result * &base_big + BigUint::from(seed);

    // Right half: primes in reverse
    for &p in primes.iter().rev() {
        result = result * &base_big + BigUint::from(p);
    }

    result
}

// Test if a triplet pattern works for given first prime
fn test_triplet_pattern(
    pattern: &TripletPattern,
    first_prime: u32,
    num_seeds: u32,
) -> (usize, usize) {
    let base = first_prime * pattern.base_multiplier;

    // Compute the three primes in pattern
    let primes: Vec<u32> = pattern.gaps.iter().map(|&gap| first_prime + gap).collect();

    // Check all primes are actually prime (simple check)
    for &p in &primes {
        if !is_prime_simple(p) {
            return (0, 0); // Pattern invalid for this first prime
        }
    }

    let mut successes = 0;
    let mut total = 0;

    for seed in 1..=num_seeds {
        let n = k_tuple_membrane(&primes, seed, base);
        total += 1;

        if is_prime(&n) {
            successes += 1;
        }
    }

    (successes, total)
}

fn is_prime_simple(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut d = 3;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║      k-TUPLE CONSTELLATION TEST                              ║");
    println!("║      Extending HL Framework Beyond Pairs                     ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    println!("HYPOTHESIS:");
    println!("  If HL theory is correct, triplet membranes should:");
    println!("  1. Work at reduced success rate (k=3 harder than k=2)");
    println!("  2. Show pattern-dependent success (admissibility matters)");
    println!("  3. Follow similar power law scaling with distance");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("TRIPLET PATTERN TESTS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    // Test various first primes
    let test_first_primes = vec![5u32, 7, 11, 13, 17, 19, 23];
    let num_seeds = 50;

    for pattern in TRIPLET_PATTERNS {
        println!("Pattern: {}", pattern.name);
        println!("  Gaps: {:?}", pattern.gaps);
        println!("─────────────────────────────────────────────────────────");

        let mut total_successes = 0;
        let mut total_tested = 0;
        let mut valid_cases = 0;

        for &p in &test_first_primes {
            let (successes, tested) = test_triplet_pattern(pattern, p, num_seeds);

            if tested > 0 {
                let rate = (successes as f64) / (tested as f64) * 100.0;
                println!(
                    "  First prime p={:2}: triplet ({},{},{}) → {}/{} = {:.1}%",
                    p,
                    p,
                    p + pattern.gaps[1],
                    p + pattern.gaps[2],
                    successes,
                    tested,
                    rate
                );

                total_successes += successes;
                total_tested += tested;
                valid_cases += 1;
            } else {
                println!("  First prime p={:2}: triplet invalid (not all prime)", p);
            }
        }

        if total_tested > 0 {
            let overall_rate = (total_successes as f64) / (total_tested as f64) * 100.0;
            println!();
            println!(
                "  Overall: {}/{} = {:.1}% (across {} valid triplets)",
                total_successes, total_tested, overall_rate, valid_cases
            );
        } else {
            println!();
            println!("  Overall: Pattern has no valid triplets in test range");
        }
        println!();
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("COMPARISON: PAIRS vs TRIPLETS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("Known pair (k=2) success rates:");
    println!("  Twin (gap 2):   ~24%");
    println!("  Cousin (gap 4): ~20%");
    println!("  Sexy (gap 6):   ~13%");
    println!();

    println!("Triplet (k=3) success rates:");
    println!("  (See results above)");
    println!();

    println!("EXPECTED from HL theory:");
    println!("  Pairs scale as 1/(log base)²");
    println!("  Triplets scale as 1/(log base)³");
    println!("  → Triplets should be MUCH rarer");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("THEORETICAL ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("Hardy-Littlewood k-tuple conjecture:");
    println!("  π_k(x) ~ S(pattern) × ∫₂ˣ dt/(log t)^k");
    println!();
    println!("For our membranes:");
    println!("  k=2 (pairs):    Success ∝ S₂ × 1/(log base)²");
    println!("  k=3 (triplets): Success ∝ S₃ × 1/(log base)³");
    println!();
    println!("The extra factor of 1/log makes triplets MUCH harder!");
    println!();

    println!("Singular series S(pattern):");
    println!("  Depends on how many residue classes are blocked");
    println!("  More gaps → more blocking → smaller S");
    println!("  Admissible patterns have S > 0");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("POWER LAW EXTENSION");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("For pairs, we found: success(d) ∝ d^(-0.498) ≈ 1/√d");
    println!();
    println!("For triplets, we PREDICT:");
    println!("  If similar pair correlation effects apply:");
    println!("  success(d) ∝ d^(-α) where α may differ from -1/2");
    println!();
    println!("QUESTION: Does the critical line -1/2 extend to k>2?");
    println!("  Or is it specific to pairs (k=2)?");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("FALSIFICATION CRITERIA");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("If HL framework is CORRECT:");
    println!("  ✓ Triplets should exist (success > 0%)");
    println!("  ✓ Triplets should be rarer than pairs");
    println!("  ✓ Pattern choice should matter (admissibility)");
    println!("  ✓ Success should scale with 1/(log base)³");
    println!();

    println!("If ANY of these fail:");
    println!("  ✗ HL framework needs revision for membranes");
    println!("  ✗ k-tuple conjecture may not apply to construction");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("NEXT STEPS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("1. Test more triplet patterns to find optimal configurations");
    println!("2. Compute HL singular series S(pattern) for each triplet");
    println!("3. Test if triplets follow power law in distance");
    println!("4. Extend to quadruplets (k=4) if triplets succeed");
    println!("5. Compare empirical success with HL predictions");
    println!();

    println!("GOAL: Determine if -1/2 exponent is:");
    println!("  A) Universal for all k (profound!)");
    println!("  B) Specific to pairs k=2 (still interesting)");
    println!("  C) Coincidence (would need new theory)");
}
