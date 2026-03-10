//! Material Landscape Sweep: Cross-Base Analysis
//!
//! Explores how material properties (purity, utilization, slippage) vary
//! across different number bases to find signal about membrane-friendly bases.
//!
//! Key question: Do bases with high membrane success rates (6, 12, 30) have
//! systematically different material landscapes than other bases?

use primes::hzlib::num_theory::{factor, gcd, Material};
use std::collections::HashMap;

/// Statistics for a base's material landscape
#[derive(Debug, Clone)]
struct BaseLandscape {
    base: u64,
    membrane_success: Option<f64>, // Known membrane success rate if available

    // Purity statistics
    mean_purity: f64,
    full_purity_count: usize,      // Cores with purity = 1.0
    full_purity_fraction: f64,

    // Utilization statistics
    mean_utilization: f64,
    full_util_count: usize,        // Cores with utilization = 1.0

    // Slippage statistics
    mean_slippage: f64,
    min_slippage: f64,

    // Primitive root statistics
    primitive_root_primes: usize,  // Primes p where base is primitive root mod p
    tested_primes: usize,
    primitive_root_fraction: f64,
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut d = 3;
    while d * d <= n {
        if n % d == 0 { return false; }
        d += 2;
    }
    true
}

fn analyze_base(base: u64, limit: u64) -> BaseLandscape {
    let mut purities = Vec::new();
    let mut utilizations = Vec::new();
    let mut slippages = Vec::new();
    let mut full_purity = 0usize;
    let mut full_util = 0usize;
    let mut primitive_root_primes = 0usize;
    let mut tested_primes = 0usize;

    // Get base's prime factors to skip
    let base_primes: Vec<u64> = factor(base).iter().map(|(p, _)| *p).collect();

    for q in 2..=limit {
        // Skip if q shares factors with base (would give core=1 or partial)
        if base_primes.iter().any(|&p| q % p == 0) {
            continue;
        }

        let m = Material::for_base(q, base);

        if m.core > 1 {
            purities.push(m.purity);
            utilizations.push(m.utilization);
            slippages.push(m.slippage);

            if (m.purity - 1.0).abs() < 1e-10 {
                full_purity += 1;
            }
            if (m.utilization - 1.0).abs() < 1e-10 {
                full_util += 1;
            }

            // For primes, check if base is primitive root
            if is_prime(q) {
                tested_primes += 1;
                if (m.purity - 1.0).abs() < 1e-10 {
                    primitive_root_primes += 1;
                }
            }
        }
    }

    let n = purities.len();
    let mean_purity = if n > 0 { purities.iter().sum::<f64>() / n as f64 } else { 0.0 };
    let mean_utilization = if n > 0 { utilizations.iter().sum::<f64>() / n as f64 } else { 0.0 };
    let mean_slippage = if n > 0 { slippages.iter().sum::<f64>() / n as f64 } else { 0.0 };
    let min_slippage = slippages.iter().cloned().fold(f64::INFINITY, f64::min);

    BaseLandscape {
        base,
        membrane_success: match base {
            6 => Some(0.33),
            10 => Some(0.185),
            12 => Some(0.30),
            30 => Some(0.30),
            _ => None,
        },
        mean_purity,
        full_purity_count: full_purity,
        full_purity_fraction: full_purity as f64 / n as f64,
        mean_utilization,
        full_util_count: full_util,
        mean_slippage,
        min_slippage,
        primitive_root_primes,
        tested_primes,
        primitive_root_fraction: if tested_primes > 0 {
            primitive_root_primes as f64 / tested_primes as f64
        } else {
            0.0
        },
    }
}

/// Artin's conjecture predicts the density of primes for which g is a primitive root
fn artin_constant() -> f64 {
    // C = prod_{p prime} (1 - 1/(p(p-1))) ≈ 0.3739558136...
    0.3739558136
}

fn main() {
    println!("=== MATERIAL LANDSCAPE SWEEP ===\n");

    let limit = 5000u64;

    // Test membrane-friendly bases vs controls
    let bases = vec![
        // Membrane champions
        6, 12, 30,
        // Standard
        10,
        // Controls (various factorizations)
        7, 8, 9, 11, 14, 15, 16, 18, 20, 22, 24,
    ];

    println!("Analyzing {} bases up to q={}...\n", bases.len(), limit);

    let mut results: Vec<BaseLandscape> = bases.iter()
        .map(|&b| analyze_base(b, limit))
        .collect();

    // Sort by primitive root fraction (our hypothesis: this correlates with membrane success)
    results.sort_by(|a, b| b.primitive_root_fraction.partial_cmp(&a.primitive_root_fraction).unwrap());

    println!("PRIMITIVE ROOT DENSITY (Artin constant ≈ {:.4})\n", artin_constant());
    println!("{:>4} {:>8} {:>8} {:>8} {:>10} {:>10}",
             "Base", "Factors", "PR_frac", "Artin_δ", "FullPurity", "Membrane%");
    println!("{}", "-".repeat(60));

    for r in &results {
        let factors = factor(r.base);
        let factor_str: String = factors.iter()
            .map(|(p, e)| if *e == 1 { format!("{}", p) } else { format!("{}^{}", p, e) })
            .collect::<Vec<_>>()
            .join("*");

        let artin_delta = r.primitive_root_fraction - artin_constant();
        let membrane_str = r.membrane_success.map(|s| format!("{:.1}%", s * 100.0))
            .unwrap_or_else(|| "?".to_string());

        println!("{:>4} {:>8} {:>8.4} {:>+8.4} {:>10.4} {:>10}",
                 r.base, factor_str, r.primitive_root_fraction, artin_delta,
                 r.full_purity_fraction, membrane_str);
    }

    // Now let's look at slippage - how much does λ differ from φ?
    println!("\n\nSLIPPAGE ANALYSIS (λ/φ ratio)\n");

    results.sort_by(|a, b| a.mean_slippage.partial_cmp(&b.mean_slippage).unwrap());

    println!("{:>4} {:>10} {:>10} {:>10}", "Base", "MeanSlip", "MinSlip", "Membrane%");
    println!("{}", "-".repeat(44));

    for r in &results {
        let membrane_str = r.membrane_success.map(|s| format!("{:.1}%", s * 100.0))
            .unwrap_or_else(|| "?".to_string());

        println!("{:>4} {:>10.4} {:>10.4} {:>10}",
                 r.base, r.mean_slippage, r.min_slippage, membrane_str);
    }

    // Direct correlation analysis
    println!("\n\nCORRELATION HUNT\n");

    let membrane_bases: Vec<&BaseLandscape> = results.iter()
        .filter(|r| r.membrane_success.is_some())
        .collect();

    if membrane_bases.len() >= 3 {
        let xs: Vec<f64> = membrane_bases.iter().map(|r| r.primitive_root_fraction).collect();
        let ys: Vec<f64> = membrane_bases.iter().map(|r| r.membrane_success.unwrap()).collect();

        // Simple correlation
        let n = xs.len() as f64;
        let mx = xs.iter().sum::<f64>() / n;
        let my = ys.iter().sum::<f64>() / n;
        let mut num = 0.0;
        let mut dx2 = 0.0;
        let mut dy2 = 0.0;
        for i in 0..xs.len() {
            let dx = xs[i] - mx;
            let dy = ys[i] - my;
            num += dx * dy;
            dx2 += dx * dx;
            dy2 += dy * dy;
        }
        let r_pr_membrane = if dx2 > 0.0 && dy2 > 0.0 { num / (dx2.sqrt() * dy2.sqrt()) } else { 0.0 };

        println!("Correlation(primitive_root_fraction, membrane_success) = {:.4}", r_pr_membrane);

        // Also try slippage
        let xs2: Vec<f64> = membrane_bases.iter().map(|r| r.mean_slippage).collect();
        let mx2 = xs2.iter().sum::<f64>() / n;
        let mut num2 = 0.0;
        let mut dx22 = 0.0;
        for i in 0..xs2.len() {
            let dx = xs2[i] - mx2;
            let dy = ys[i] - my;
            num2 += dx * dy;
            dx22 += dx * dx;
        }
        let r_slip_membrane = if dx22 > 0.0 && dy2 > 0.0 { num2 / (dx22.sqrt() * dy2.sqrt()) } else { 0.0 };

        println!("Correlation(mean_slippage, membrane_success) = {:.4}", r_slip_membrane);

        // Full purity fraction
        let xs3: Vec<f64> = membrane_bases.iter().map(|r| r.full_purity_fraction).collect();
        let mx3 = xs3.iter().sum::<f64>() / n;
        let mut num3 = 0.0;
        let mut dx32 = 0.0;
        for i in 0..xs3.len() {
            let dx = xs3[i] - mx3;
            let dy = ys[i] - my;
            num3 += dx * dy;
            dx32 += dx * dx;
        }
        let r_fp_membrane = if dx32 > 0.0 && dy2 > 0.0 { num3 / (dx32.sqrt() * dy2.sqrt()) } else { 0.0 };

        println!("Correlation(full_purity_fraction, membrane_success) = {:.4}", r_fp_membrane);
    }

    // Most interesting: which cores show up as purity=1 across MULTIPLE bases?
    println!("\n\nUNIVERSAL HIGH-PURITY CORES\n");
    println!("Cores with purity=1.0 in multiple bases (primitive roots are rare!):\n");

    let mut core_base_map: HashMap<u64, Vec<u64>> = HashMap::new();

    for &base in &bases {
        let base_primes: Vec<u64> = factor(base).iter().map(|(p, _)| *p).collect();

        for q in 2..=limit {
            if base_primes.iter().any(|&p| q % p == 0) {
                continue;
            }
            if !is_prime(q) {
                continue; // Focus on prime cores
            }

            let m = Material::for_base(q, base);
            if (m.purity - 1.0).abs() < 1e-10 {
                core_base_map.entry(q).or_default().push(base);
            }
        }
    }

    // Find cores that are primitive roots for many bases
    let mut universal_cores: Vec<(u64, Vec<u64>)> = core_base_map.into_iter()
        .filter(|(_, bases)| bases.len() >= 5)
        .collect();
    universal_cores.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    println!("{:>6} {:>6} Bases where this prime has full period", "Prime", "Count");
    println!("{}", "-".repeat(60));
    for (core, bases) in universal_cores.iter().take(20) {
        let bases_str: String = bases.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(",");
        println!("{:>6} {:>6} {}", core, bases.len(), bases_str);
    }

    // What's special about these primes?
    if !universal_cores.is_empty() {
        println!("\n\nANALYSIS: What structure do universal primitives share?\n");

        let universal_primes: Vec<u64> = universal_cores.iter()
            .take(10)
            .map(|(p, _)| *p)
            .collect();

        println!("Top 10 universal primitives: {:?}", universal_primes);

        // Check: are they all ≡ 1 (mod 4)? Or Sophie Germain? Or...?
        let mod4_1: Vec<u64> = universal_primes.iter().filter(|&&p| p % 4 == 1).cloned().collect();
        let mod4_3: Vec<u64> = universal_primes.iter().filter(|&&p| p % 4 == 3).cloned().collect();

        println!("  ≡ 1 (mod 4): {} primes {:?}", mod4_1.len(), mod4_1);
        println!("  ≡ 3 (mod 4): {} primes {:?}", mod4_3.len(), mod4_3);

        // Check p-1 factorization (smooth = easier to be primitive root for many bases)
        println!("\n  p-1 factorizations:");
        for &p in universal_primes.iter().take(5) {
            let pm1_factors = factor(p - 1);
            let factor_str: String = pm1_factors.iter()
                .map(|(q, e)| if *e == 1 { format!("{}", q) } else { format!("{}^{}", q, e) })
                .collect::<Vec<_>>()
                .join(" * ");
            println!("    {} - 1 = {} = {}", p, p - 1, factor_str);
        }
    }
}
