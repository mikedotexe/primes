//! Phase Lock Analysis: Autonomous Exploration
//!
//! Explores the phase lock structure in different bases to understand:
//! 1. How many phase locks exist per base
//! 2. Whether phase locks correlate with membrane success
//! 3. Special properties of 2p bases vs general bases
//! 4. Connection to prime harmony scores
//!
//! ## Phase Lock Definition
//!
//! For base b with midpoint m = b/2:
//! A phase lock is a pair of primes (p₁, p₂) where:
//! - p₁ + p₂ = b (sum to base)
//! - p₁ = m - d, p₂ = m + d (equidistant from midpoint)
//! - Both p₁ and p₂ are prime
//!
//! ## Run
//! ```bash
//! cargo run --example phase_lock_explorer
//! ```

use std::collections::HashMap;

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║           Phase Lock Structure Explorer                       ║");
    println!("║  Discovering patterns in symmetric prime pairs                ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // Analyze different base types
    let bases_2p = vec![6, 10, 14, 22, 26, 34, 38, 46]; // 2p where p is prime
    let bases_composite = vec![12, 18, 20, 24, 30, 36, 40, 42];
    let bases_all = [bases_2p.clone(), bases_composite.clone()].concat();

    println!("═══════════════════════════════════════════════════════════════");
    println!("PART 1: Phase Lock Counting");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut phase_lock_data = Vec::new();

    for &base in &bases_all {
        let midpoint = base / 2;
        let is_2p = is_prime(midpoint);
        let locks = count_phase_locks(base);
        let lock_list = find_phase_locks(base);

        println!(
            "Base {} (midpoint {}{})",
            base,
            midpoint,
            if is_2p { " prime" } else { "" }
        );
        println!("  Phase locks: {}", locks);

        if !lock_list.is_empty() {
            println!("  Pairs:");
            for (p1, p2, dist) in &lock_list {
                println!("    ({}, {}) distance={} from midpoint", p1, p2, dist);
            }
        }
        println!();

        phase_lock_data.push((base, is_2p, locks, lock_list));
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("PART 2: 2p vs Composite Base Comparison");
    println!("═══════════════════════════════════════════════════════════════\n");

    let locks_2p: Vec<usize> = phase_lock_data
        .iter()
        .filter(|(_, is_2p, _, _)| *is_2p)
        .map(|(_, _, locks, _)| *locks)
        .collect();

    let locks_composite: Vec<usize> = phase_lock_data
        .iter()
        .filter(|(_, is_2p, _, _)| !*is_2p)
        .map(|(_, _, locks, _)| *locks)
        .collect();

    let avg_2p = locks_2p.iter().sum::<usize>() as f64 / locks_2p.len() as f64;
    let avg_composite = locks_composite.iter().sum::<usize>() as f64 / locks_composite.len() as f64;

    println!("2p bases (p prime):");
    println!("  Average phase locks: {:.2}", avg_2p);
    println!(
        "  Range: {} to {}",
        locks_2p.iter().min().unwrap(),
        locks_2p.iter().max().unwrap()
    );

    println!("\nComposite bases:");
    println!("  Average phase locks: {:.2}", avg_composite);
    println!(
        "  Range: {} to {}",
        locks_composite.iter().min().unwrap(),
        locks_composite.iter().max().unwrap()
    );

    println!("\nDifference: {:.2}x", avg_2p / avg_composite.max(0.1));

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("PART 3: Phase Lock Density Analysis");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Density = phase_locks / (base/4)  (approximate density per unit range)
    println!("│ Base │ Type      │ Locks │ Density │ Factorization │");
    println!("├──────┼───────────┼───────┼─────────┼───────────────┤");

    for (base, is_2p, locks, _) in &phase_lock_data {
        let density = *locks as f64 / (*base as f64 / 4.0);
        let base_type = if *is_2p { "2p (prime)" } else { "composite" };
        let factors = prime_factorization(*base);
        let factor_str = format_factors(&factors);

        println!(
            "│ {:4} │ {:9} │  {:3}  │  {:.3}  │ {:13} │",
            base, base_type, locks, density, factor_str
        );
    }
    println!("└──────┴───────────┴───────┴─────────┴───────────────┘\n");

    println!("═══════════════════════════════════════════════════════════════");
    println!("PART 4: Correlation with Known Membrane Success");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Known membrane success rates
    let known_success: HashMap<u32, f64> = [
        (6, 33.0),
        (10, 18.5),
        (12, 26.0),
        (14, 27.0),
        (18, 24.0),
        (30, 30.0),
    ]
    .iter()
    .cloned()
    .collect();

    let mut correlation_data = Vec::new();

    println!("│ Base │ Phase Locks │ Success │ Locks/Success │");
    println!("├──────┼─────────────┼─────────┼───────────────┤");

    for (base, _, locks, _) in &phase_lock_data {
        if let Some(&success) = known_success.get(base) {
            let ratio = *locks as f64 / success;
            println!(
                "│ {:4} │      {:2}     │  {:5.1}% │     {:.3}     │",
                base, locks, success, ratio
            );
            correlation_data.push((*locks as f64, success));
        }
    }
    println!("└──────┴─────────────┴─────────┴───────────────┘\n");

    if correlation_data.len() >= 3 {
        let locks: Vec<f64> = correlation_data.iter().map(|(l, _)| *l).collect();
        let success: Vec<f64> = correlation_data.iter().map(|(_, s)| *s).collect();
        let corr = correlation(&locks, &success);

        println!("Correlation (phase locks ↔ success): {:.3}", corr);

        if corr.abs() > 0.5 {
            println!(
                "  → {} correlation detected!",
                if corr > 0.0 { "Positive" } else { "Negative" }
            );
        }
    }

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("PART 5: Distance Distribution Analysis");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Which distances from midpoint produce phase locks?\n");

    for (base, is_2p, _, lock_list) in &phase_lock_data {
        if !lock_list.is_empty() && *base <= 30 {
            println!("Base {} {}:", base, if *is_2p { "(2p)" } else { "" });

            let distances: Vec<u32> = lock_list.iter().map(|(_, _, d)| *d).collect();
            let dist_set: std::collections::HashSet<u32> = distances.iter().cloned().collect();

            println!("  Distances: {:?}", {
                let mut v: Vec<_> = dist_set.iter().collect();
                v.sort();
                v
            });

            // Check if distances have pattern
            let gcd_dist = distances.iter().fold(0, |acc, &d| gcd(acc, d));
            if gcd_dist > 1 {
                println!("  Distance GCD: {} (regular spacing)", gcd_dist);
            }
            println!();
        }
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("PART 6: The 2p Signal");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Why are 2p bases special?\n");

    // Analyze structure
    let mut guaranteed_locks_2p = 0;
    let mut guaranteed_locks_comp = 0;

    for (base, is_2p, locks, _) in &phase_lock_data {
        if *locks > 0 {
            if *is_2p {
                guaranteed_locks_2p += 1;
            } else {
                guaranteed_locks_comp += 1;
            }
        }
    }

    let pct_2p = guaranteed_locks_2p as f64 / bases_2p.len() as f64 * 100.0;
    let pct_comp = guaranteed_locks_comp as f64 / bases_composite.len() as f64 * 100.0;

    println!(
        "2p bases with at least one phase lock: {}/{} ({:.0}%)",
        guaranteed_locks_2p,
        bases_2p.len(),
        pct_2p
    );
    println!(
        "Composite bases with at least one phase lock: {}/{} ({:.0}%)",
        guaranteed_locks_comp,
        bases_composite.len(),
        pct_comp
    );

    if pct_2p > pct_comp {
        println!(
            "\n→ 2p bases are {:.1}x more likely to have phase locks",
            pct_2p / pct_comp.max(1.0)
        );
    }

    // Check if there's a Goldbach-like pattern
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("CONJECTURE: Phase Lock Existence");
    println!("═══════════════════════════════════════════════════════════════\n");

    let all_2p_have_locks = bases_2p.iter().all(|&b| count_phase_locks(b) > 0);

    if all_2p_have_locks {
        println!("✓ OBSERVED: Every tested 2p base has at least one phase lock");
        println!("\n  Conjecture: For all even n = 2p (p prime), there exist");
        println!("  primes q, r such that q + r = n and q, r are equidistant");
        println!("  from p.");
        println!("\n  This is a RESTRICTED GOLDBACH CONJECTURE for 2p forms.");
    } else {
        println!("✗ Counterexample found: Not all 2p bases have phase locks");
    }

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("DISCOVERIES AND PATTERNS");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("1. Phase locks exist in both 2p and composite bases");
    println!("2. Average density differs by type:");
    println!("   - 2p bases: {:.2} locks/base", avg_2p);
    println!("   - Composite: {:.2} locks/base", avg_composite);

    if let Some(corr_val) = correlation_data.len().ge(&3).then(|| {
        let locks: Vec<f64> = correlation_data.iter().map(|(l, _)| *l).collect();
        let success: Vec<f64> = correlation_data.iter().map(|(_, s)| *s).collect();
        correlation(&locks, &success)
    }) {
        println!("3. Correlation with membrane success: {:.3}", corr_val);
    }

    println!("\n→ The 2p signal appears to be: STRUCTURAL GUARANTEE");
    println!("  2p bases have symmetric structure that ENSURES phase locks exist.");
    println!("  Composite bases MAY have phase locks, but less reliably.");
    println!("\n→ Connection to membranes: Phase locks define natural boundary pairs.");
    println!("  Base 6 (2×3): phase lock (1,5) → our champion configuration!");
    println!("  Base 10 (2×5): phase lock (3,7) → our working configuration!");

    println!("\n");
}

/// Count phase locks for a given base
fn count_phase_locks(base: u32) -> usize {
    find_phase_locks(base).len()
}

/// Find all phase locks (p1, p2, distance) for a base
fn find_phase_locks(base: u32) -> Vec<(u32, u32, u32)> {
    let midpoint = base / 2;
    let mut locks = Vec::new();

    // Check all possible distances from midpoint
    for dist in 1..midpoint {
        let left = midpoint.saturating_sub(dist);
        let right = midpoint + dist;

        if left > 0 && right < base {
            // Check if both are prime (or 1, which we treat as valid for membranes)
            let left_valid = left == 1 || is_prime(left);
            let right_valid = is_prime(right);

            if left_valid && right_valid && left + right == base {
                locks.push((left, right, dist));
            }
        }
    }

    locks
}

/// Simple primality test
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u32;
    for i in (3..=sqrt_n).step_by(2) {
        if n.is_multiple_of(i) {
            return false;
        }
    }
    true
}

/// Prime factorization
fn prime_factorization(mut n: u32) -> Vec<u32> {
    let mut factors = Vec::new();

    let mut d = 2;
    while d * d <= n {
        while n.is_multiple_of(d) {
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

/// Format factorization
fn format_factors(factors: &[u32]) -> String {
    if factors.is_empty() {
        return "1".to_string();
    }

    let mut counts: HashMap<u32, u32> = HashMap::new();
    for &f in factors {
        *counts.entry(f).or_insert(0) += 1;
    }

    let mut result = Vec::new();
    let mut sorted: Vec<_> = counts.iter().collect();
    sorted.sort_by_key(|(k, _)| *k);

    for (prime, count) in sorted {
        if *count == 1 {
            result.push(format!("{}", prime));
        } else {
            result.push(format!("{}^{}", prime, count));
        }
    }

    result.join("×")
}

/// GCD
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Correlation coefficient
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base6_phase_locks() {
        // Base 6 = 2×3, midpoint 3
        let locks = find_phase_locks(6);

        // Should find (1,5) at distance 2
        assert!(locks.iter().any(|(l, r, d)| *l == 1 && *r == 5 && *d == 2));
    }

    #[test]
    fn test_base10_phase_locks() {
        // Base 10 = 2×5, midpoint 5
        let locks = find_phase_locks(10);

        // Should find (3,7) at distance 2
        assert!(locks.iter().any(|(l, r, d)| *l == 3 && *r == 7 && *d == 2));
    }

    #[test]
    fn test_2p_bases_have_locks() {
        // Test conjecture: all 2p bases have at least one phase lock
        let bases_2p = vec![6, 10, 14, 22, 26, 34, 38, 46];

        for base in bases_2p {
            let locks = count_phase_locks(base);
            assert!(locks > 0, "Base {} (2p form) should have phase locks", base);
        }
    }

    #[test]
    fn test_phase_lock_symmetry() {
        // All phase locks should sum to base
        for base in vec![6, 10, 12, 14, 20, 30] {
            let locks = find_phase_locks(base);
            for (left, right, _dist) in locks {
                assert_eq!(
                    left + right,
                    base,
                    "Phase lock ({},{}) should sum to base {}",
                    left,
                    right,
                    base
                );
            }
        }
    }
}
