//! Goldbach N× Transform Explorer
//!
//! **THE BIG IDEA**: Can we use N× transform theory to CONSTRUCT Goldbach pairs
//! instead of just searching for them?
//!
//! # The Goldbach Attack
//!
//! Goldbach conjecture: Every even number ≥ 4 is the sum of two primes.
//!
//! Traditional approach: Search randomly for p₁, p₂ where p₁ + p₂ = T
//!
//! Our approach: Use membrane structures + N× transform to ENGINEER primes that sum to T
//!
//! # Phase 1: Pattern Discovery
//!
//! This tool analyzes existing Goldbach pairs through the lens of N× transform:
//!
//! For each pair (p₁, p₂) where p₁ + p₂ = T:
//! 1. Decompose p₁ as (r₁ + k₁·B) / N across multiple bases B
//! 2. Decompose p₂ as (r₂ + k₂·B) / N for the same bases
//! 3. Look for "resonance" - bases where BOTH primes fit clean N× patterns
//! 4. Measure correlations between r₁ and r₂ residue structures
//!
//! # Key Questions
//!
//! - Do certain bases (like 30, which has 30% membrane success) capture most pairs?
//! - Are r₁ and r₂ correlated when both primes decompose in the same base?
//! - Can we predict k_int₁ from k_int₂?
//! - Does "trio universality" (N=3, gcd(B,3)=1) help?
//!
//! # Success Criteria
//!
//! If we find that base B systematically decomposes both primes in many pairs,
//! we can REVERSE the process:
//! 1. Pick target T
//! 2. Choose optimal base B (e.g., 30)
//! 3. Use N× formula k_int ≡ -r·B⁻¹ (mod N) to engineer r₁, r₂
//! 4. Apply membrane constraints to ensure primality
//! 5. Generate p₁, p₂ ON DEMAND
//!
//! # Usage
//!
//! ```bash
//! # Analyze small target
//! cargo run --example goldbach_ntransform_explorer -- --target=100
//!
//! # Larger target with more bases
//! cargo run --example goldbach_ntransform_explorer -- --target=1000 --bases=6,10,30,106
//!
//! # Full exploration
//! cargo run --example goldbach_ntransform_explorer -- --target=10000 --N=3
//! ```
//!
//! # Output
//!
//! - Console: Summary statistics and resonance patterns
//! - CSV: Full decomposition data for further analysis
//!
//! # Relationship to Main Research
//!
//! - Combines membrane generation (33% success in base 6)
//! - With N× transform theory (deterministic k_int control)
//! - To attack additive number theory's white whale
//!
//! If successful, this transforms prime generation from probabilistic to constructive.

use prime_physics_engine::is_prime;
use num_bigint::BigUint;
use std::collections::HashMap;

// ============================================================================
// N× TRANSFORM CORE (ported from Python reference)
// ============================================================================

/// Extended GCD for modular inverse
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x1, y1) = egcd(b, a % b);
        (g, y1, x1 - (a / b) * y1)
    }
}

/// Modular inverse: returns Some(inv) where a*inv ≡ 1 (mod n), or None if gcd(a,n) ≠ 1
fn inv_mod(a: u64, n: u64) -> Option<u64> {
    let a_signed = (a % n) as i64;
    let n_signed = n as i64;
    let (g, x, _) = egcd(a_signed, n_signed);
    if g != 1 {
        return None;
    }
    Some(((x % n_signed + n_signed) % n_signed) as u64)
}

/// Compute residues for k = 0..N-1 in the N× transform
fn residues_after_transform(b: u64, n: u64, r: u64) -> Vec<u64> {
    let a = r % n;
    let b_mod = b % n;
    (0..n).map(|k| (a + k * b_mod) % n).collect()
}

/// Find k ∈ [0, N-1] where (r + k·B) ≡ 0 (mod N) - the integer vertex
fn vertex_integer_k(b: u64, n: u64, r: u64) -> Option<u64> {
    let g = gcd(b % n, n);
    if r % g != 0 {
        return None;
    }
    if g == n {
        return Some(0); // everything ≡ r (mod N), and r ≡ 0 (mod N)
    }

    let b_reduced = (b / g) % (n / g);
    let r_reduced = (r / g) % (n / g);
    let n_reduced = n / g;

    let inv = inv_mod(b_reduced, n_reduced)?;
    let k0 = ((n_reduced - r_reduced) * inv) % n_reduced;

    Some(k0)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// Full N× transform analysis for a prime
#[derive(Debug, Clone)]
struct NTransformDecomposition {
    prime: u64,
    base: u64,
    n: u64,
    r: u64,
    k_int: u64,
    residues: Vec<u64>,
    gcd_bn: u64,
    trio_universal: bool, // For N=3, gcd(B,N)=1 → residues are always {0,1,2}
}

/// Try to decompose prime p as (r + k·B) / N for given base B and N
fn try_decompose(p: u64, b: u64, n: u64) -> Option<NTransformDecomposition> {
    // We want to find r, k such that (r + k·B) / N = p
    // So r + k·B = p·N
    // For various k, compute r = p·N - k·B and check if valid

    for k in 0..n {
        let product = p * n;
        let term = k * b;

        if term > product {
            continue;
        }

        let r = product - term;

        // Check if this r gives us an integer vertex at k
        if let Some(k_vertex) = vertex_integer_k(b, n, r) {
            if k_vertex == k {
                let residues = residues_after_transform(b, n, r);
                let gcd_bn = gcd(b, n);
                let trio_universal = n == 3 && gcd_bn == 1 && residues.iter().collect::<std::collections::HashSet<_>>().len() == 3;

                return Some(NTransformDecomposition {
                    prime: p,
                    base: b,
                    n,
                    r,
                    k_int: k,
                    residues,
                    gcd_bn,
                    trio_universal,
                });
            }
        }
    }

    None
}

// ============================================================================
// GOLDBACH PAIR ENUMERATION
// ============================================================================

/// Find all Goldbach pairs for target T
fn find_goldbach_pairs(target: u64) -> Vec<(u64, u64)> {
    if target < 4 || target % 2 != 0 {
        return vec![];
    }

    let mut pairs = Vec::new();

    // Only need to check up to T/2 (by symmetry)
    for p1 in 2..=(target / 2) {
        let p2 = target - p1;

        if is_prime(&BigUint::from(p1)) && is_prime(&BigUint::from(p2)) {
            pairs.push((p1, p2));
        }
    }

    pairs
}

// ============================================================================
// PATTERN CORRELATION
// ============================================================================

#[derive(Debug, Clone)]
struct ResonancePattern {
    target: u64,
    pair: (u64, u64),
    base: u64,
    n: u64,
    decomp1: NTransformDecomposition,
    decomp2: NTransformDecomposition,
    r_correlation: f64, // Simple correlation measure
    k_relationship: String, // e.g., "k1 = k2", "k1 + k2 = N", etc.
}

/// Analyze correlations between two N× decompositions
fn analyze_resonance(
    target: u64,
    pair: (u64, u64),
    decomp1: &NTransformDecomposition,
    decomp2: &NTransformDecomposition,
) -> ResonancePattern {
    // Simple r correlation: how similar are the residue patterns?
    let r_corr = if decomp1.residues == decomp2.residues {
        1.0
    } else if decomp1.residues.iter().all(|&r| decomp2.residues.contains(&r)) {
        0.5
    } else {
        0.0
    };

    // k_int relationship
    let k_rel = if decomp1.k_int == decomp2.k_int {
        "equal".to_string()
    } else if (decomp1.k_int + decomp2.k_int) % decomp1.n == 0 {
        format!("sum≡0(mod{})", decomp1.n)
    } else if decomp1.k_int + decomp2.k_int == decomp1.n {
        format!("sum={}", decomp1.n)
    } else {
        format!("{}+{}", decomp1.k_int, decomp2.k_int)
    };

    ResonancePattern {
        target,
        pair,
        base: decomp1.base,
        n: decomp1.n,
        decomp1: decomp1.clone(),
        decomp2: decomp2.clone(),
        r_correlation: r_corr,
        k_relationship: k_rel,
    }
}

// ============================================================================
// SUMMARY REPORTING
// ============================================================================

fn print_summary(
    target: u64,
    label: &str,
    pairs: &[(u64, u64)],
    resonances: &[ResonancePattern],
    base_success_counts: &HashMap<u64, usize>,
    bases: &[u64],
) {
    println!("┌──────────────────────────────────────────────────────────────────┐");
    println!("│ SUMMARY for T={} ({})                                      ", target, label);
    println!("└──────────────────────────────────────────────────────────────────┘");
    println!();
    println!("Total Goldbach pairs: {}", pairs.len());
    println!("Total resonances found: {}", resonances.len());
    println!();

    if !resonances.is_empty() {
        println!("Base Success Rates:");
        for &b in bases {
            let count = base_success_counts.get(&b).unwrap_or(&0);
            let rate = (*count as f64) / (pairs.len() as f64) * 100.0;
            println!("  Base {:3}: {:4}/{} pairs ({:5.1}%)", b, count, pairs.len(), rate);
        }
    }
}

// ============================================================================
// MAIN EXPLORATION
// ============================================================================

fn parse_arg(args: &[String], flag: &str) -> Option<String> {
    // Support both --flag=value and --flag value formats
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

    // Parse arguments with both --flag=value and --flag value support
    let target_arg = parse_arg(&args, "--target")
        .and_then(|s| s.parse::<u64>().ok());

    let bases_str = parse_arg(&args, "--bases")
        .unwrap_or_else(|| "6,10,30,106".to_string());

    let bases: Vec<u64> = bases_str
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let n = parse_arg(&args, "--N")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(3);

    // NEW: Scale mode for base-proportional analysis
    let scale_mode = parse_arg(&args, "--scale-mode")
        .unwrap_or_else(|| "fixed".to_string());

    let scale_mode = scale_mode.as_str();

    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           GOLDBACH N× TRANSFORM EXPLORER                        ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("Bases to test: {:?}", bases);
    println!("N× transform: N={}", n);
    println!("Scale mode: {}", scale_mode);
    println!();

    // Determine targets based on mode
    let targets_to_test: Vec<(u64, String)> = match scale_mode {
        "proportional" => {
            let mut targets = Vec::new();
            for &b in &bases {
                let b2 = b * b;
                let b3 = b * b * b;

                // Ensure even targets
                targets.push((if b2 % 2 == 0 { b2 } else { 2 * b2 }, format!("B²={}", b)));
                targets.push((if b3 % 2 == 0 { b3 } else { 2 * b3 }, format!("B³={}", b)));
            }
            targets
        },
        "both" => {
            let mut targets = Vec::new();
            // Fixed mode
            if let Some(t) = target_arg {
                targets.push((t, "fixed".to_string()));
            } else {
                targets.push((100, "fixed".to_string()));
            }
            // Plus proportional for each base
            for &b in &bases {
                let b2 = b * b;
                targets.push((if b2 % 2 == 0 { b2 } else { 2 * b2 }, format!("B²={}", b)));
            }
            targets
        },
        _ => { // "fixed" or default
            let t = target_arg.unwrap_or(100);
            vec![(t, "fixed".to_string())]
        }
    };

    // Run analysis for each target
    let mut all_results: HashMap<u64, (Vec<ResonancePattern>, HashMap<u64, usize>)> = HashMap::new();

    for (target, label) in &targets_to_test {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🎯 ANALYZING T={} ({})", target, label);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!();

        // Step 1: Find all Goldbach pairs
        println!("🔍 Finding Goldbach pairs for T={}...", target);
        let pairs = find_goldbach_pairs(*target);
        println!("   Found {} pairs", pairs.len());
        println!();

        if pairs.is_empty() {
            println!("❌ No Goldbach pairs found (is T even and ≥ 4?)");
            continue;
        }

        // Step 2: For each pair, try to decompose both primes in each base
        println!("🧪 Analyzing N× decompositions...");
        println!();

        let mut resonances: Vec<ResonancePattern> = Vec::new();
        let mut base_success_counts: HashMap<u64, usize> = HashMap::new();

        for (p1, p2) in &pairs {
            let verbose = pairs.len() <= 10; // Only show details for small target sets

            if verbose {
                println!("Pair: {} + {} = {}", p1, p2, target);
            }

            for &b in &bases {
                if let Some(d1) = try_decompose(*p1, b, n) {
                    if let Some(d2) = try_decompose(*p2, b, n) {
                        let resonance = analyze_resonance(*target, (*p1, *p2), &d1, &d2);

                        if verbose {
                            println!("  ✨ RESONANCE in base {}", b);
                            println!("     p₁={}: r₁={}, k_int={}, residues={:?}",
                                     p1, d1.r, d1.k_int, d1.residues);
                            println!("     p₂={}: r₂={}, k_int={}, residues={:?}",
                                     p2, d2.r, d2.k_int, d2.residues);
                            println!("     k relationship: {}", resonance.k_relationship);
                            println!("     Trio universal: {} & {}", d1.trio_universal, d2.trio_universal);
                        }

                        *base_success_counts.entry(b).or_insert(0) += 1;
                        resonances.push(resonance);
                    }
                }
            }
            if verbose {
                println!();
            }
        }

        all_results.insert(*target, (resonances.clone(), base_success_counts.clone()));

        // Show summary for this target
        print_summary(*target, label, &pairs, &resonances, &base_success_counts, &bases);
        println!();
    }

    // Step 3: Comparative analysis (if multiple targets tested)
    if all_results.len() > 1 {
        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║                  COMPARATIVE ANALYSIS                            ║");
        println!("╚══════════════════════════════════════════════════════════════════╝");
        println!();
        println!("Comparing resonance rates across scales:");
        println!();

        // Group by base
        for &b in &bases {
            println!("BASE {}:", b);
            for (target, label) in &targets_to_test {
                if let Some((resonances, base_counts)) = all_results.get(target) {
                    let pairs_count = find_goldbach_pairs(*target).len();
                    if pairs_count > 0 {
                        let count = base_counts.get(&b).unwrap_or(&0);
                        let rate = (*count as f64) / (pairs_count as f64) * 100.0;
                        let indicator = if label.contains(&format!("B²={}", b)) || label.contains(&format!("B³={}", b)) {
                            "★" // Star for base-proportional
                        } else {
                            " "
                        };
                        println!("  {} T={:6} ({:12}): {:4}/{} pairs ({:5.1}%)",
                                 indicator, target, label, count, pairs_count, rate);
                    }
                }
            }
            println!();
        }

        println!("★ = base-proportional target (B² or B³)");
        println!();
        println!("🔬 HYPOTHESIS TEST:");
        println!("   If ★ targets show HIGHER resonance rates,");
        println!("   this confirms base-scale harmonic structure!");
    }

    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                        IMPLICATIONS                              ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    if all_results.values().all(|(r, _)| r.is_empty()) {
        println!("❌ No resonances found. Try:");
        println!("   - Larger target T");
        println!("   - More bases (add membrane-optimal bases)");
        println!("   - Different N values");
        println!("   - --scale-mode=proportional to test natural scales");
    } else {
        println!("✨ Resonances detected!");
        println!();
        println!("NEXT STEPS:");
        println!("1. If base-proportional targets show higher rates → confirms harmonic structure");
        println!("2. Look for k_int patterns to predict complementary primes");
        println!("3. Combine with membrane constraints (e.g., base 6 (1,5) k=(0,0))");
        println!("4. Use N× formula k_int ≡ -r·B⁻¹ (mod N) to engineer r values");
        println!();
        println!("🎯 ULTIMATE GOAL: Given T, compute r₁ such that:");
        println!("   - p₁ = (r₁ + k₁·B)/N is membrane-prime");
        println!("   - p₂ = T - p₁ = (r₂ + k₂·B)/N is membrane-prime");
        println!("   - Goldbach pairs CONSTRUCTED, not found!");
    }
}
