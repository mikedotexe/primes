//! Phase Lock Analysis v5: Residue Theory Deep Dive
//!
//! v1-v4 SUMMARY:
//!   • Geometric factors (distance, position) show weak correlations
//!   • Two viable strategies exist (boundary vs midpoint) - both ~26-27%
//!   • Base 6 (33%) is an OUTLIER even within its strategy
//!
//! NEW HYPOTHESIS (v5):
//! H10: Success depends on RESIDUE SYSTEM properties
//!      Base 6 (1,5) creates special residue patterns because:
//!        - Base 6 = 2×3 (two prime factors)
//!        - Digit 1 ≡ 1 mod everything (universal)
//!        - Digit 5 ≡ 1 mod 2, ≡ 2 mod 3
//!      Maybe the membrane's residue pattern mod base's factors matters?
//!
//! ANALYSIS APPROACH:
//!   1. Compute residues of boundary digits mod each prime factor of base
//!   2. Check if patterns like (1,1) or (-1,-1) appear
//!   3. Investigate quadratic residue status
//!   4. Test "residue coverage" - do digits span different residue classes?
//!
//! ## Run
//! ```bash
//! cargo run --example phase_lock_explorer_v5
//! ```

use std::collections::HashMap;

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║   Phase Lock Explorer v5: Residue Theory Deep Dive          ║");
    println!("║   Investigating number-theoretic properties                  ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    let known_success: HashMap<u32, f64> = [
        (6, 33.0),   // 2×3: (1,5) → residues mod 2: (1,1), mod 3: (1,2)
        (10, 18.5),  // 2×5: (3,7) → residues mod 2: (1,1), mod 5: (3,2)
        (12, 26.0),  // 2²×3: (5,7) → residues mod 2: (1,1), mod 3: (2,1)
        (14, 27.0),  // 2×7: (3,11) → residues mod 2: (1,1), mod 7: (3,4)
        (18, 24.0),  // 2×3²: (7,11) → residues mod 2: (1,1), mod 3: (1,2)
        (30, 30.0),  // 2×3×5: (13,17) → residues mod 2: (1,1), mod 3: (1,2), mod 5: (3,2)
    ]
    .iter()
    .cloned()
    .collect();

    let bases: Vec<u32> = known_success.keys().cloned().collect();

    println!("═══════════════════════════════════════════════════════════════");
    println!("PART 1: Residue Pattern Analysis");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Analyzing residues mod each prime factor of the base:\n");

    let mut analysis_data = Vec::new();

    for &base in &bases {
        let locks = find_phase_locks(base);
        let success = known_success[&base];

        if let Some((left, right, _dist)) = locks.first() {
            let factors = prime_factorization(base);

            println!("Base {:2} = {} → ({:2},{:2}) → {:.1}%",
                     base, format_factorization(&factors), left, right, success);

            // Compute residues mod each distinct prime factor
            let distinct_primes: Vec<u32> = factors.iter().map(|&(p, _)| p).collect();

            print!("  Residues: ");
            let mut residue_features = Vec::new();

            for &p in &distinct_primes {
                let left_res = left % p;
                let right_res = right % p;
                print!("mod {}: ({},{}), ", p, left_res, right_res);

                // Feature: Are both ≡ 1?
                let both_one = left_res == 1 && right_res == 1;
                // Feature: Are both ≡ -1?
                let both_minus_one = left_res == p - 1 && right_res == p - 1;
                // Feature: Different residue classes?
                let different = left_res != right_res;
                // Feature: Sum of residues
                let res_sum = (left_res + right_res) % p;

                residue_features.push((p, left_res, right_res, both_one, both_minus_one, different, res_sum));
            }
            println!();

            // Check for special patterns
            let all_odd = distinct_primes.iter().all(|&p| {
                (left % p) % 2 == 1 && (right % p) % 2 == 1
            });

            // "Residue diversity" - do digits span different residue classes mod each factor?
            let avg_diversity = residue_features.iter()
                .filter(|(_, _, _, _, _, diff, _)| *diff)
                .count() as f64 / residue_features.len() as f64;

            // "Unit-like" - are many residues ≡ 1?
            let unit_count = residue_features.iter()
                .filter(|(_, l, r, _, _, _, _)| *l == 1 || *r == 1)
                .count();

            println!("  All odd: {}, Diversity: {:.2}, Unit-like: {}",
                     if all_odd { "✓" } else { "✗" }, avg_diversity, unit_count);
            println!();

            analysis_data.push((
                base,
                success,
                distinct_primes.len() as f64,
                avg_diversity,
                unit_count as f64,
                all_odd,
            ));
        }
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("PART 2: Correlation Analysis");
    println!("═══════════════════════════════════════════════════════════════\n");

    let successes: Vec<f64> = analysis_data.iter().map(|(_, s, _, _, _, _)| *s).collect();
    let prime_counts: Vec<f64> = analysis_data.iter().map(|(_, _, pc, _, _, _)| *pc).collect();
    let diversities: Vec<f64> = analysis_data.iter().map(|(_, _, _, d, _, _)| *d).collect();
    let unit_counts: Vec<f64> = analysis_data.iter().map(|(_, _, _, _, uc, _)| *uc).collect();
    let all_odd_numeric: Vec<f64> = analysis_data.iter()
        .map(|(_, _, _, _, _, ao)| if *ao { 1.0 } else { 0.0 })
        .collect();

    let corr_primes = correlation(&prime_counts, &successes);
    let corr_diversity = correlation(&diversities, &successes);
    let corr_units = correlation(&unit_counts, &successes);
    let corr_all_odd = correlation(&all_odd_numeric, &successes);

    println!("Correlation with membrane success:\n");
    println!("H10a (# prime factors):      {:+.3}", corr_primes);
    println!("H10b (residue diversity):    {:+.3}", corr_diversity);
    println!("H10c (unit-like count):      {:+.3}", corr_units);
    println!("H10d (all residues odd):     {:+.3}", corr_all_odd);

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("PART 3: The Base 6 Anomaly");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Why is Base 6 (33%) an outlier?\n");

    // Compare Base 6 to other BOUNDARY strategy bases
    let boundary_bases = vec![6, 10, 14];

    println!("BOUNDARY strategy bases comparison:\n");
    for &base in &boundary_bases {
        let locks = find_phase_locks(base);
        let success = known_success[&base];

        if let Some((left, right, _)) = locks.first() {
            let factors = prime_factorization(base);
            println!("Base {:2} ({:2},{:2}) → {:.1}%", base, left, right, success);

            // Special properties
            let has_digit_one = *left == 1 || *right == 1;
            let has_base_minus_one = *left == base - 1 || *right == base - 1;
            let smallest_base = base <= 10;

            println!("  Factors: {}", format_factorization(&factors));
            println!("  Has digit 1: {}", if has_digit_one { "✓" } else { "✗" });
            println!("  Has base-1: {}", if has_base_minus_one { "✓" } else { "✗" });
            println!("  Smallest base: {}", if smallest_base { "✓" } else { "✗" });
            println!();
        }
    }

    println!("🎯 OBSERVATION:");
    println!("Base 6 is UNIQUE among boundary strategy bases:");
    println!("  1. Uses digit 1 (universal element)");
    println!("  2. Uses digit base-1 (5 = 6-1)");
    println!("  3. Smallest base in the comparison");
    println!("  4. Simplest factorization structure (2×3)");

    println!("\n💡 HYPOTHESIS:");
    println!("The (1, base-1) configuration in the smallest composite base");
    println!("creates optimal conditions because:");
    println!("  • Digit 1 generates all residues via multiplication");
    println!("  • Digit base-1 ≡ -1 mod base (additive inverse structure)");
    println!("  • Small base = small residue system = less 'noise'");

    println!("\n🔬 TEST:");
    println!("Check if other bases with (1, base-1) configurations perform well...");

    // Test hypothesis: does (1, base-1) exist for other bases and perform well?
    let test_bases = vec![8, 16, 20, 22, 26, 28];
    println!("\nChecking bases for (1, base-1) phase locks:\n");

    for &base in &test_bases {
        let locks = find_phase_locks(base);
        if let Some((left, right, dist)) = locks.first() {
            if (*left == 1 && *right == base - 1) || (*left == base - 1 && *right == 1) {
                println!("  Base {:2}: Has (1,{}) lock at distance={} ✓", base, base-1, dist);
            } else {
                println!("  Base {:2}: Primary lock ({},{}) - no (1,{}) pattern", base, left, right, base-1);
            }
        } else {
            println!("  Base {:2}: No phase locks found", base);
        }
    }

    println!("\n");
}

fn find_phase_locks(base: u32) -> Vec<(u32, u32, u32)> {
    let midpoint = base / 2;
    let mut locks = Vec::new();

    for dist in 1..midpoint {
        let left = midpoint.saturating_sub(dist);
        let right = midpoint + dist;

        if left > 0 && right < base {
            let left_valid = left == 1 || is_prime(left);
            let right_valid = is_prime(right);

            if left_valid && right_valid && left + right == base {
                locks.push((left, right, dist));
            }
        }
    }

    locks.sort_by_key(|(_, _, d)| *d);
    locks
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u32;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn prime_factorization(mut n: u32) -> Vec<(u32, u32)> {
    let mut factors = Vec::new();
    let mut d = 2;

    while d * d <= n {
        let mut count = 0;
        while n % d == 0 {
            count += 1;
            n /= d;
        }
        if count > 0 {
            factors.push((d, count));
        }
        d += 1;
    }

    if n > 1 {
        factors.push((n, 1));
    }

    factors
}

fn format_factorization(factors: &[(u32, u32)]) -> String {
    factors
        .iter()
        .map(|(p, e)| {
            if *e == 1 {
                format!("{}", p)
            } else {
                format!("{}^{}", p, e)
            }
        })
        .collect::<Vec<_>>()
        .join("×")
}

fn correlation(xs: &[f64], ys: &[f64]) -> f64 {
    if xs.len() != ys.len() || xs.is_empty() {
        return 0.0;
    }

    let n = xs.len() as f64;
    let mean_x = xs.iter().sum::<f64>() / n;
    let mean_y = ys.iter().sum::<f64>() / n;

    let mut cov = 0.0;
    let mut var_x = 0.0;
    let mut var_y = 0.0;

    for i in 0..xs.len() {
        let dx = xs[i] - mean_x;
        let dy = ys[i] - mean_y;
        cov += dx * dy;
        var_x += dx * dx;
        var_y += dy * dy;
    }

    if var_x < 1e-10 || var_y < 1e-10 {
        return 0.0;
    }

    cov / (var_x * var_y).sqrt()
}
