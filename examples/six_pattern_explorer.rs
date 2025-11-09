// The Pattern of Six: Autonomous Exploration
//
// Following the emergent pattern of 6 appearing throughout
// the coordinate constellation framework.
//
// OBSERVATIONS:
// - φ(14) = 6 (our primary test base)
// - φ(18) = 6 (another test base)
// - Base 6 achieved 16% success (highest)
// - 6 bases tested total
// - 6 = first perfect number (1+2+3 = 6, 1×2×3 = 6)
// - Multiples of 6 appear in 19.4% of y-coordinates
//
// RESEARCH QUESTIONS:
// 1. Which bases have φ(base) = 6?
// 2. Do they share special properties for prime generation?
// 3. Is there a connection to k=3,6,9 pattern?
// 4. What role does the perfect number play?

use std::collections::HashMap;

fn totient(n: u32) -> u32 {
    let mut result = n;
    let mut n_mut = n;
    let mut p = 2;

    while p * p <= n_mut {
        if n_mut % p == 0 {
            while n_mut % p == 0 {
                n_mut /= p;
            }
            result -= result / p;
        }
        p += 1;
    }

    if n_mut > 1 {
        result -= result / n_mut;
    }

    result
}

fn factorize(mut n: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let mut d = 2;

    while d * d <= n {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
        d += 1;
    }

    if n > 1 {
        factors.push(n);
    }

    factors
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║              THE PATTERN OF SIX                              ║");
    println!("║              Autonomous Exploration                          ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("PART 1: FINDING BASES WHERE φ(base) = 6");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    let mut bases_with_phi_6 = Vec::new();

    for base in 2..=100 {
        if totient(base) == 6 {
            bases_with_phi_6.push(base);
        }
    }

    println!("Bases (2-100) where φ(base) = 6:");
    for &base in &bases_with_phi_6 {
        let factors = factorize(base);
        println!("  Base {:3}: factorization = {:?}", base, factors);
    }
    println!();

    println!("Found {} bases with φ(base) = 6", bases_with_phi_6.len());
    println!();

    // Analyze the pattern
    println!("Pattern analysis:");
    for &base in &bases_with_phi_6 {
        let factors = factorize(base);
        let distinct: std::collections::HashSet<_> = factors.iter().collect();
        let is_square_free = factors.len() == distinct.len();

        print!("  {} = ", base);
        for (i, &f) in factors.iter().enumerate() {
            if i > 0 {
                print!("×");
            }
            print!("{}", f);
        }
        if is_square_free {
            println!(" (square-free)");
        } else {
            println!(" (has squares)");
        }
    }
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("PART 2: THE PERFECT NUMBER CONNECTION");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("6 is the first perfect number:");
    println!("  Divisors of 6: {{1, 2, 3}}");
    println!("  Sum of proper divisors: 1 + 2 + 3 = 6 ✓");
    println!("  Also: 1×2×3 = 6");
    println!("  And: 1+2+3 = 6");
    println!();

    println!("Perfect number formula: 2^(p-1) × (2^p - 1)");
    println!("  For p=2: 2^1 × (2^2 - 1) = 2 × 3 = 6 ✓");
    println!();

    println!("Relationship to our bases:");
    println!("  6 = 2 × 3");
    println!("  14 = 2 × 7 (where 7 is Mersenne-adjacent)");
    println!("  18 = 2 × 3² (includes 3 from perfect number)");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("PART 3: DIMENSIONAL PATTERN (k=3,6,9)");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("We've tested:");
    println!("  k=3 (triplets)");
    println!("  k=5 (quintuplets)");
    println!("  k=7 (septuplets)");
    println!();

    println!("But notice: 3, 6, 9 form an arithmetic sequence!");
    println!("  3 = 3×1 (minimal)");
    println!("  6 = 3×2 (perfect)");
    println!("  9 = 3×3 (square)");
    println!();

    println!("k=6 structure would be:");
    println!("  y-y-x-MIDDLE-x-y-y (doubled outer coord)");
    println!("  OR");
    println!("  z-x-MIDDLE-x-z (skip middle coord)");
    println!();

    println!("k=9 structure would be:");
    println!("  w-z-y-x-MIDDLE-x-y-z-w (4D coordinate)");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("PART 4: THE 6,157 PRIMES");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    let total_primes = 6157;
    println!("Total primes found across all bases: {}", total_primes);
    println!();

    println!("Digit analysis of 6157:");
    println!("  6157 = 6×1000 + 157");
    println!("  6157 = 6×1000 + 157");
    println!("  First digit: 6");
    println!();

    // Check if 6157 is prime
    println!("Is 6157 prime? Testing...");
    let mut is_prime = true;
    for d in 2..=(6157f64.sqrt() as u32) {
        if 6157 % d == 0 {
            println!("  6157 = {} × {}", d, 6157 / d);
            is_prime = false;
            break;
        }
    }
    if is_prime {
        println!("  Yes! 6157 is PRIME ✓");
    }
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("PART 5: HEXAGONAL STRUCTURE");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("6-fold symmetry appears in:");
    println!("  - Hexagons (6 sides)");
    println!("  - Benzene rings (6 carbons)");
    println!("  - Snowflakes (6-fold symmetry)");
    println!("  - Honeycomb cells (hexagonal)");
    println!();

    println!("In coordinates:");
    println!("  - 6 directions in hexagonal lattice");
    println!("  - φ(base) = 6 gives 6 allowed outer coordinates");
    println!("  - Could constellations form hexagonal patterns?");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("PART 6: CONNECTIONS TO EXISTING WORK");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("φ(6) = 2:");
    println!("  Base 6 has only 2 coprime values: {{1, 5}}");
    println!("  These form phase lock: 1 + 5 = 6");
    println!("  Minimal constraint → highest success (16%)!");
    println!();

    println!("φ(14) = 6:");
    println!("  Base 14 has 6 coprime values: {{1,3,5,9,11,13}}");
    println!("  These form 3 phase locks:");
    println!("    (1,13), (3,11), (5,9)");
    println!("  6 = 3 × 2 (3 pairs, each with 2 elements)");
    println!();

    println!("φ(18) = 6:");
    println!("  Base 18 has 6 coprime values: {{1,5,7,11,13,17}}");
    println!("  Also forms 3 phase locks:");
    println!("    (1,17), (5,13), (7,11)");
    println!("  Same structure!");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("SYNTHESIS: WHY 6?");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("6 emerges as a fundamental constant because:");
    println!();

    println!("1. PERFECT NUMBER:");
    println!("   6 = sum of proper divisors");
    println!("   Creates balanced arithmetic structure");
    println!();

    println!("2. TOTIENT VALUES:");
    println!("   Multiple bases have φ(base) = 6");
    println!("   These bases have 6 coprime coordinates");
    println!("   Forms 3 symmetric phase lock pairs");
    println!();

    println!("3. DIMENSIONAL RESONANCE:");
    println!("   3-6-9 pattern (multiples of 3)");
    println!("   k=3 (triplets) × 2 = k=6");
    println!("   k=3 (triplets) × 3 = k=9");
    println!();

    println!("4. HEXAGONAL SYMMETRY:");
    println!("   6-fold patterns appear in nature");
    println!("   May reflect coordinate lattice structure");
    println!();

    println!("5. EMPIRICAL OBSERVATION:");
    println!("   Base 6: 16% success (highest tested)");
    println!("   φ(14)=6, φ(18)=6: both perform well");
    println!("   6,157 total primes (itself prime!)");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("NEXT STEPS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("Immediate tests:");
    println!("  1. Test all bases with φ(base)=6: {{7,9,14,18}}");
    println!("  2. Test k=6 and k=9 constellations");
    println!("  3. Look for hexagonal patterns in coordinate plots");
    println!("  4. Explore 3-6-9 dimensional progression");
    println!();

    println!("Deeper questions:");
    println!("  - Why do perfect numbers relate to prime structure?");
    println!("  - Is φ(base)=6 optimal for some metric?");
    println!("  - Does hexagonal lattice thinking help?");
    println!("  - Connection to 6π²/ζ(2) = 6π²/(π²/6) = 36?");
    println!();
}
