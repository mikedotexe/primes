//! Phase Lock Analysis v4: Edge Proximity Hypothesis
//!
//! v1 RESULT: Total lock count correlation = 0.208 (weak)
//! v2 RESULT: Distance, coprimality, quality - all ≈0 (rejected)
//! v3 RESULT: Digit ratio = -0.453 (moderate but inconsistent)
//!
//! NEW HYPOTHESIS (v4):
//! H9: "Edge proximity" - Success correlates with digits near base boundaries
//!     Champion Base 6 (1,5): digit 1 = near-zero, digit 5 = near-base (6)
//!     Runner-up Base 30 (13,17): both centered near midpoint (15)
//!
//! Maybe there are TWO winning strategies:
//!   A) "Boundary strategy": Small digit (1,3) + Large digit (base-1, base-2)
//!   B) "Midpoint strategy": Both digits clustered near honorary zero
//!
//! Let's test: min(left, base-right) as "edge proximity score"
//!
//! ## Run
//! ```bash
//! cargo run --example phase_lock_explorer_v4
//! ```

use std::collections::HashMap;

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║    Phase Lock Explorer v4: Edge Proximity Hypothesis        ║");
    println!("║    Testing if digits near base edges correlate with success  ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    let known_success: HashMap<u32, f64> = [
        (6, 33.0),   // (1,5): left_edge=1, right_edge=1 → edge_score=1
        (10, 18.5),  // (3,7): left_edge=3, right_edge=3 → edge_score=3
        (12, 26.0),  // (5,7): left_edge=5, right_edge=5 → edge_score=5
        (14, 27.0),  // (3,11): left_edge=3, right_edge=3 → edge_score=3
        (18, 24.0),  // (7,11): left_edge=7, right_edge=7 → edge_score=7
        (30, 30.0),  // (13,17): left_edge=13, right_edge=13 → edge_score=13
    ]
    .iter()
    .cloned()
    .collect();

    let bases: Vec<u32> = known_success.keys().cloned().collect();

    println!("═══════════════════════════════════════════════════════════════");
    println!("PART 1: Edge Proximity Analysis");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut analysis_data = Vec::new();

    println!("│ Base │ Lock    │ LeftEdge │ RightEdge │ MinEdge │ MidDist │ Success │");
    println!("├──────┼─────────┼──────────┼───────────┼─────────┼─────────┼─────────┤");

    for &base in &bases {
        let locks = find_phase_locks(base);
        let success = known_success[&base];

        if let Some((left, right, dist)) = locks.first() {
            let midpoint = base / 2;

            // Edge distances
            let left_from_edge = *left;  // Distance from 0
            let right_from_edge = base.saturating_sub(*right);  // Distance from base

            // Minimum edge distance (closest to ANY edge)
            let min_edge_dist = std::cmp::min(left_from_edge, right_from_edge);

            // Distance from midpoint (we already know this from v2)
            let midpoint_dist = *dist;

            // Compute "centrality" - how close digits cluster to midpoint
            let left_mid_dist = (*left as i32 - midpoint as i32).abs() as u32;
            let right_mid_dist = (*right as i32 - midpoint as i32).abs() as u32;
            let avg_mid_dist = (left_mid_dist + right_mid_dist) as f64 / 2.0;

            println!(
                "│ {:4} │ ({:2},{:2}) │    {:2}    │     {:2}    │    {:2}   │   {:4.1}  │ {:5.1}% │",
                base, left, right, left_from_edge, right_from_edge, min_edge_dist, avg_mid_dist, success
            );

            analysis_data.push((
                base,
                success,
                left_from_edge as f64,
                right_from_edge as f64,
                min_edge_dist as f64,
                midpoint_dist as f64,
                avg_mid_dist,
            ));
        }
    }
    println!("└──────┴─────────┴──────────┴───────────┴─────────┴─────────┴─────────┘\n");

    println!("═══════════════════════════════════════════════════════════════");
    println!("PART 2: Correlation Tests");
    println!("═══════════════════════════════════════════════════════════════\n");

    let successes: Vec<f64> = analysis_data.iter().map(|(_, s, _, _, _, _, _)| *s).collect();

    let left_edges: Vec<f64> = analysis_data.iter().map(|(_, _, le, _, _, _, _)| *le).collect();
    let right_edges: Vec<f64> = analysis_data.iter().map(|(_, _, _, re, _, _, _)| *re).collect();
    let min_edges: Vec<f64> = analysis_data.iter().map(|(_, _, _, _, me, _, _)| *me).collect();
    let mid_dists: Vec<f64> = analysis_data.iter().map(|(_, _, _, _, _, md, _)| *md).collect();
    let avg_centrality: Vec<f64> = analysis_data.iter().map(|(_, _, _, _, _, _, ac)| *ac).collect();

    let corr_left_edge = correlation(&left_edges, &successes);
    let corr_right_edge = correlation(&right_edges, &successes);
    let corr_min_edge = correlation(&min_edges, &successes);
    let corr_mid_dist = correlation(&mid_dists, &successes);
    let corr_centrality = correlation(&avg_centrality, &successes);

    println!("Correlation with membrane success:\n");
    println!("H9a (left edge distance):     {:+.3}", corr_left_edge);
    println!("H9b (right edge distance):    {:+.3}", corr_right_edge);
    println!("H9c (min edge distance):      {:+.3}", corr_min_edge);
    println!("H9d (midpoint distance):      {:+.3}", corr_mid_dist);
    println!("H9e (avg centrality):         {:+.3}", corr_centrality);

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("PART 3: Strategy Classification");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Classify strategies
    println!("Strategy classification:\n");

    for (base, success, left_edge, right_edge, min_edge, mid_dist, avg_cent) in &analysis_data {
        let strategy = if *min_edge <= 3.0 {
            "BOUNDARY"
        } else if *avg_cent <= 3.0 {
            "MIDPOINT"
        } else {
            "MIXED"
        };

        let locks = find_phase_locks(*base);
        if let Some((left, right, _)) = locks.first() {
            println!("Base {:2} ({:2},{:2}): {:.1}% → {} strategy",
                     base, left, right, success, strategy);
            println!("  min_edge={:.0}, avg_centrality={:.1}", min_edge, avg_cent);
        }
    }

    // Count strategies
    let boundary_count = analysis_data.iter().filter(|(_, _, _, _, me, _, _)| *me <= 3.0).count();
    let midpoint_count = analysis_data.iter().filter(|(_, _, _, _, me, _, ac)| *me > 3.0 && *ac <= 3.0).count();

    println!("\nStrategy distribution:");
    println!("  BOUNDARY strategy: {} bases", boundary_count);
    println!("  MIDPOINT strategy: {} bases", midpoint_count);
    println!("  MIXED strategy: {} bases", analysis_data.len() - boundary_count - midpoint_count);

    // Average success by strategy
    let boundary_avg = analysis_data
        .iter()
        .filter(|(_, _, _, _, me, _, _)| *me <= 3.0)
        .map(|(_, s, _, _, _, _, _)| *s)
        .sum::<f64>() / boundary_count.max(1) as f64;

    let midpoint_avg = if midpoint_count > 0 {
        analysis_data
            .iter()
            .filter(|(_, _, _, _, me, _, ac)| *me > 3.0 && *ac <= 3.0)
            .map(|(_, s, _, _, _, _, _)| *s)
            .sum::<f64>() / midpoint_count as f64
    } else {
        0.0
    };

    if boundary_count > 0 {
        println!("\nBOUNDARY strategy average success: {:.1}%", boundary_avg);
    }
    if midpoint_count > 0 {
        println!("MIDPOINT strategy average success: {:.1}%", midpoint_avg);
    }

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("CONCLUSIONS");
    println!("═══════════════════════════════════════════════════════════════\n");

    let best_corr = [
        ("left edge", corr_left_edge),
        ("right edge", corr_right_edge),
        ("min edge", corr_min_edge),
        ("midpoint dist", corr_mid_dist),
        ("centrality", corr_centrality),
    ];

    let strongest = best_corr
        .iter()
        .max_by(|a, b| a.1.abs().partial_cmp(&b.1.abs()).unwrap())
        .unwrap();

    println!("Strongest correlation: {} ({:+.3})", strongest.0, strongest.1);

    if strongest.1.abs() > 0.5 {
        println!("  → STRONG correlation! ✓");
    } else if strongest.1.abs() > 0.3 {
        println!("  → MODERATE correlation");
    } else {
        println!("  → WEAK correlation");
    }

    println!("\n🎯 EMERGING INSIGHT:");
    println!("Looking at the champion (Base 6, 33%) and runner-up (Base 30, 30%):");
    println!("  • Base 6: (1,5) - EXTREME positioning (1 near zero, 5 near base)");
    println!("  • Base 30: (13,17) - CENTERED positioning (both near midpoint 15)");
    println!("\nThese represent DIFFERENT successful strategies!");
    println!("Perhaps the common factor isn't geometric position,");
    println!("but something about NUMBER-THEORETIC properties specific to each base...");

    println!("\n💡 NEXT ITERATION:");
    println!("v5 should investigate BASE-SPECIFIC number theory:");
    println!("  • How do these digits interact with the base's prime factorization?");
    println!("  • Do they create favorable residue patterns mod various small primes?");
    println!("  • Is there something special about their multiplicative order?");

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
