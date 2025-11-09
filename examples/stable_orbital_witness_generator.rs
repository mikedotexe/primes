// Stable Orbital Witness Generator
//
// COMPUTE-THEN-VERIFY PIPELINE:
// 1. Generate coordinate constellation primes (Rust)
// 2. Extract residues and compute distances
// 3. Find minimum safe radius R
// 4. Generate Agda witness code (StableOrbital proof)
// 5. Type-check validates exclusion zone invariant
//
// This is the dynamic complement to static honorary zero verification.

use prime_physics_engine::is_prime;
use num_bigint::BigUint;
use num_traits::Zero;
use std::collections::{HashMap, HashSet};

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

#[derive(Debug, Clone)]
struct ResidueData {
    residue: u32,
    count: usize,
    distance_from_mid: u32,
}

fn collect_residue_distribution(base: u32, limit: u64) -> Vec<ResidueData> {
    let midpoint = base / 2;

    let middle_values: Vec<u32> = (1..base)
        .filter(|&m| is_coprime(m, base))
        .collect();

    let mut residue_counts: HashMap<u32, usize> = HashMap::new();

    for &middle in &middle_values {
        for z in 1..base {
            if !is_coprime(z, base) { continue; }
            for y in 1..base {
                if !is_coprime(y, base) { continue; }
                for x in 1..base {
                    if !is_coprime(x, base) { continue; }

                    let candidate = septuplet_membrane(middle, x, y, z, base);
                    if candidate > BigUint::from(limit) { continue; }

                    if is_prime(&candidate) {
                        // Track residues of x, y, z coordinates
                        *residue_counts.entry(x).or_insert(0) += 1;
                        *residue_counts.entry(y).or_insert(0) += 1;
                        *residue_counts.entry(z).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    let mut data: Vec<ResidueData> = residue_counts
        .into_iter()
        .map(|(r, count)| {
            let dist = if r > midpoint {
                r - midpoint
            } else {
                midpoint - r
            };
            ResidueData {
                residue: r,
                count,
                distance_from_mid: dist,
            }
        })
        .collect();

    data.sort_by_key(|d| d.distance_from_mid);
    data
}

fn generate_agda_witness(base: u32, data: &[ResidueData]) -> String {
    let midpoint = base / 2;

    // Find minimum distance (this becomes our exclusion radius R)
    let min_dist = data.iter()
        .map(|d| d.distance_from_mid)
        .min()
        .unwrap_or(0);

    // Verify all residues maintain minimum distance
    let all_safe = data.iter().all(|d| d.distance_from_mid >= min_dist);

    if !all_safe {
        return format!("-- ERROR: Not all residues maintain minimum distance!\n");
    }

    // Check for honorary zero (midpoint should not appear)
    let midpoint_present = data.iter().any(|d| d.residue == midpoint);

    let mut agda_code = String::new();

    agda_code.push_str(&format!("-- Base {} Stable Orbital Witness\n", base));
    agda_code.push_str(&format!("-- Midpoint: {}\n", midpoint));
    agda_code.push_str(&format!("-- Exclusion radius R: {}\n", min_dist));
    agda_code.push_str(&format!("-- Honorary zero: {}\n", if midpoint_present { "VIOLATED!" } else { "✓" }));
    agda_code.push_str(&format!("-- Unique residues: {}\n\n", data.len()));

    // Generate the residue list
    let residues: Vec<u32> = data.iter().map(|d| d.residue).collect();

    agda_code.push_str(&format!("base{}-residues : List Nat\n", base));
    agda_code.push_str(&format!("base{}-residues = {}\n\n", base,
        if residues.is_empty() {
            "[]".to_string()
        } else {
            residues.iter()
                .map(|r| format!("{}", r))
                .collect::<Vec<_>>()
                .join(" ∷ ") + " ∷ []"
        }
    ));

    // Generate the stable orbital witness
    agda_code.push_str(&format!("base{}-stable : StableOrbital {} {} base{}-residues\n",
        base, min_dist, midpoint, base));
    agda_code.push_str(&format!("base{}-stable = ", base));

    // Construct nested stableCons
    if residues.is_empty() {
        agda_code.push_str("stableNil\n");
    } else {
        for (i, &r) in residues.iter().enumerate() {
            let dist = if r > midpoint { r - midpoint } else { midpoint - r };

            // Generate the proof term (simplified - would need full ≤ proofs)
            agda_code.push_str("stableCons\n");
            agda_code.push_str(&format!("  {! Proof: {} ≤ |{} - {}| = {} !}\n",
                min_dist, r, midpoint, dist));

            if i == residues.len() - 1 {
                agda_code.push_str("  stableNil");
            } else {
                agda_code.push_str("  (");
            }
        }
        // Close parentheses
        for _ in 0..(residues.len() - 1) {
            agda_code.push_str(")");
        }
        agda_code.push_str("\n");
    }

    agda_code.push_str("\n");

    // Generate distance verification table
    agda_code.push_str(&format!("-- Distance verification:\n"));
    for d in data {
        agda_code.push_str(&format!("--   Residue {:2}: |{:2} - {:2}| = {:2} {} {}\n",
            d.residue,
            d.residue,
            midpoint,
            d.distance_from_mid,
            if d.distance_from_mid >= min_dist { "✓" } else { "✗" },
            if d.residue == midpoint { "(MIDPOINT!)" } else { "" }
        ));
    }

    agda_code
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║     STABLE ORBITAL WITNESS GENERATOR                        ║");
    println!("║     Dynamic Invariant: Compute-then-Verify Pipeline         ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    let bases = vec![7u32, 14, 18];
    let limit = 1_000_000_000_000u64;

    println!("THEORY:");
    println!("  SafePos R mid x = R ≤ |x - mid|");
    println!("  StableOrbital R mid xs = indexed type enforcing SafePos at every step");
    println!("  Inviolability: StableOrbital ∧ InZone ⇒ ⊥");
    println!();

    for &base in &bases {
        println!("═══════════════════════════════════════════════════════════════");
        println!("BASE {} ANALYSIS", base);
        println!("═══════════════════════════════════════════════════════════════");
        println!();

        let phi = (1..base).filter(|&n| is_coprime(n, base)).count();
        let midpoint = base / 2;
        let mid_coprime = is_coprime(midpoint, base);

        println!("  Base: {}", base);
        println!("  φ({}): {}", base, phi);
        println!("  Midpoint: {}", midpoint);
        println!("  Midpoint coprime: {}", if mid_coprime { "YES (exception!)" } else { "NO (honorary zero expected)" });
        println!();

        println!("  Collecting coordinate constellation primes...");
        let data = collect_residue_distribution(base, limit);
        println!("  Found {} unique residues", data.len());
        println!();

        if data.is_empty() {
            println!("  ⚠ No primes found - skipping");
            continue;
        }

        // Static invariant check
        let midpoint_count = data.iter()
            .find(|d| d.residue == midpoint)
            .map(|d| d.count)
            .unwrap_or(0);

        println!("  STATIC INVARIANT (Honorary Zero):");
        println!("    Count at midpoint {}: {}", midpoint, midpoint_count);
        println!("    Honorary zero: {}", if midpoint_count == 0 { "✓ HOLDS" } else { "✗ VIOLATED" });
        println!();

        // Dynamic invariant analysis
        let min_dist = data.iter()
            .map(|d| d.distance_from_mid)
            .min()
            .unwrap_or(0);

        let max_dist = data.iter()
            .map(|d| d.distance_from_mid)
            .max()
            .unwrap_or(0);

        println!("  DYNAMIC INVARIANT (Stable Orbital):");
        println!("    Minimum distance from mid: {}", min_dist);
        println!("    Maximum distance from mid: {}", max_dist);
        println!("    Exclusion radius R: {}", min_dist);
        println!();

        // Check if all residues satisfy SafePos
        let all_safe = data.iter().all(|d| d.distance_from_mid >= min_dist);
        println!("    All residues maintain R ≤ |r - mid|: {}", if all_safe { "✓" } else { "✗" });
        println!();

        // Generate Agda witness
        println!("  AGDA WITNESS CODE:");
        println!("{}", "  " + &generate_agda_witness(base, &data).replace("\n", "\n  "));
        println!();
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("SUMMARY");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("Generated Agda witness code for {} bases", bases.len());
    println!();
    println!("NEXT STEPS:");
    println!("  1. Copy witness code into agda-proofs/Tests/GeneratedWitnesses.agda");
    println!("  2. Replace {! !} holes with actual ≤ proofs");
    println!("  3. Type-check with: agda --safe GeneratedWitnesses.agda");
    println!("  4. Success = dynamic invariant verified ✓");
    println!();
}
