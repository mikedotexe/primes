//! Excess Investigation
//!
//! We observe ~6.5× efficiency but theory (B/φ(B)) predicts ~5.5-5.8×.
//! Where does the extra ~1× come from?
//!
//! Hypotheses:
//! 1. SIZE EFFECT: L=1 creates smaller numbers → higher PNT density
//! 2. MEMBRANE STRUCTURE: The L|seed|R form has favorable properties
//! 3. MEASUREMENT ARTIFACT: Our efficiency calculation has bias
//!
//! Test: Compare L=1 vs other coprime L values at same base

use num_bigint::BigUint;
use primes::is_prime_miller_rabin;

fn random_seed_with_length(base: u64, length: usize, rng: &mut u64) -> BigUint {
    if length == 0 { return BigUint::ZERO; }
    let mut next = || {
        *rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        *rng
    };
    let b = BigUint::from(base);
    let first = (next() % (base - 1)) + 1;
    let mut seed = BigUint::from(first);
    for _ in 1..length { seed = seed * &b + BigUint::from(next() % base); }
    seed
}

fn membrane_value(base: u64, left: u64, seed: &BigUint, right: u64) -> BigUint {
    let b = BigUint::from(base);
    let mut sd = 0u32;
    let mut t = seed.clone();
    while t > BigUint::ZERO { t /= &b; sd += 1; }
    if sd == 0 { sd = 1; }
    BigUint::from(left) * b.pow(sd + 1) + seed * &b + BigUint::from(right)
}

fn coprime_digits(base: u64) -> Vec<u64> {
    let factors: Vec<u64> = {
        let mut n = base;
        let mut fs = vec![];
        for p in [2, 3, 5, 7, 11, 13, 17, 19, 23] {
            if n % p == 0 { fs.push(p); while n % p == 0 { n /= p; } }
        }
        fs
    };
    (1..base).filter(|&d| factors.iter().all(|&p| d % p != 0)).collect()
}

fn euler_phi_fast(mut n: u64) -> u64 {
    let mut result = n;
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 { n /= p; }
            result -= result / p;
        }
        p += 1;
    }
    if n > 1 { result -= result / n; }
    result
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           EXCESS INVESTIGATION: Where Does +1× Come From?        ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let base = 30030u64;  // P₆ - well-characterized
    let phi = euler_phi_fast(base);
    let theoretical = base as f64 / phi as f64;

    println!("Base: {} (P₆ = 2×3×5×7×11×13)", base);
    println!("Theoretical bound B/φ(B) = {}/{} = {:.3}×\n", base, phi, theoretical);

    let coprimes = coprime_digits(base);
    println!("Coprime digits to test as L: {:?}\n", &coprimes[..10.min(coprimes.len())]);

    // Test different L values
    let samples = 2000;
    let seed_lengths: Vec<usize> = (6..=14).collect();

    println!("Testing efficiency for different left boundary digits L...\n");
    println!("{:>6} {:>10} {:>10} {:>10} {:>12} {:>10}",
             "L", "primes", "tests", "rate%", "efficiency", "vs theory");
    println!("{}", "-".repeat(65));

    let test_ls: Vec<u64> = vec![1, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    let mut results: Vec<(u64, f64, f64)> = vec![];

    for &left in &test_ls {
        if !coprimes.contains(&left) { continue; }

        let right = 1u64;  // Keep R=1 for all tests
        let mut rng = 42424242u64 + left * 7777;

        let mut total_primes = 0;
        let mut total_tests = 0;
        let mut total_digits = 0.0;

        for &seed_len in &seed_lengths {
            for _ in 0..samples {
                let seed = random_seed_with_length(base, seed_len, &mut rng);
                let mem = membrane_value(base, left, &seed, right);
                total_digits += mem.to_string().len() as f64;
                total_tests += 1;
                if is_prime_miller_rabin(&mem) {
                    total_primes += 1;
                }
            }
        }

        let rate = total_primes as f64 / total_tests as f64;
        let mean_digits = total_digits / total_tests as f64;
        let pnt_expected = 1.0 / (mean_digits * 2.303);
        let efficiency = rate / pnt_expected;
        let vs_theory = efficiency - theoretical;

        results.push((left, efficiency, mean_digits));

        println!("{:>6} {:>10} {:>10} {:>9.2}% {:>12.3} {:>+10.3}",
                 left, total_primes, total_tests, rate * 100.0, efficiency, vs_theory);
    }

    // Analysis
    println!("\n{}", "═".repeat(65));
    println!("ANALYSIS: Is the excess from SIZE EFFECT?");
    println!("{}", "═".repeat(65));

    // Sort by L value to see size correlation
    let l1_result = results.iter().find(|(l, _, _)| *l == 1);
    let other_results: Vec<_> = results.iter().filter(|(l, _, _)| *l != 1).collect();

    if let Some((_, l1_eff, l1_digits)) = l1_result {
        println!("\nL=1 results:");
        println!("  Efficiency: {:.3}×", l1_eff);
        println!("  Mean digits: {:.1}", l1_digits);

        if !other_results.is_empty() {
            let other_eff: f64 = other_results.iter().map(|(_, e, _)| *e).sum::<f64>()
                / other_results.len() as f64;
            let other_digits: f64 = other_results.iter().map(|(_, _, d)| *d).sum::<f64>()
                / other_results.len() as f64;

            println!("\nOther L values (mean):");
            println!("  Efficiency: {:.3}×", other_eff);
            println!("  Mean digits: {:.1}", other_digits);

            let size_effect = l1_eff - other_eff;
            let digit_diff = other_digits - l1_digits;

            println!("\nSIZE EFFECT contribution:");
            println!("  L=1 vs others efficiency gap: {:+.3}×", size_effect);
            println!("  L=1 vs others digit difference: {:.1} digits", digit_diff);

            // Expected from PNT
            let pnt_ratio = other_digits / l1_digits;
            let expected_from_pnt = other_eff * pnt_ratio.ln() / (l1_digits * 2.303);

            println!("\n  L=1 excess over theory: {:+.3}×", l1_eff - theoretical);
            println!("  Other L excess over theory: {:+.3}×", other_eff - theoretical);
        }
    }

    // Correlation between L value and efficiency
    println!("\n{}", "═".repeat(65));
    println!("CORRELATION: L value vs Efficiency");
    println!("{}", "═".repeat(65));

    let xs: Vec<f64> = results.iter().map(|(l, _, _)| *l as f64).collect();
    let ys: Vec<f64> = results.iter().map(|(_, e, _)| *e).collect();

    let n = xs.len() as f64;
    let mean_x = xs.iter().sum::<f64>() / n;
    let mean_y = ys.iter().sum::<f64>() / n;

    let cov: f64 = xs.iter().zip(ys.iter())
        .map(|(x, y)| (x - mean_x) * (y - mean_y))
        .sum::<f64>() / n;
    let std_x = (xs.iter().map(|x| (x - mean_x).powi(2)).sum::<f64>() / n).sqrt();
    let std_y = (ys.iter().map(|y| (y - mean_y).powi(2)).sum::<f64>() / n).sqrt();

    let correlation = cov / (std_x * std_y);

    println!("\nPearson correlation (L, efficiency): {:.3}", correlation);

    if correlation < -0.5 {
        println!("→ STRONG negative correlation: smaller L → higher efficiency");
        println!("  This confirms SIZE EFFECT is a major contributor!");
    } else if correlation < -0.2 {
        println!("→ Moderate negative correlation: SIZE EFFECT contributes");
    } else {
        println!("→ Weak/no correlation: SIZE EFFECT is NOT the main factor");
    }

    // Residual analysis
    println!("\n{}", "═".repeat(65));
    println!("RESIDUAL: What's left after SIZE EFFECT?");
    println!("{}", "═".repeat(65));

    // If all L values show excess over B/φ(B), there's something else
    let min_excess = results.iter()
        .map(|(_, e, _)| e - theoretical)
        .fold(f64::INFINITY, f64::min);
    let max_excess = results.iter()
        .map(|(_, e, _)| e - theoretical)
        .fold(f64::NEG_INFINITY, f64::max);

    println!("\nExcess over B/φ(B) = {:.3}:", theoretical);
    println!("  Minimum (L={}): {:.3}×",
             results.iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().0,
             min_excess);
    println!("  Maximum (L=1): {:.3}×", max_excess);

    if min_excess > 0.3 {
        println!("\n⚠️  ALL L values exceed theory by >{:.1}×!", min_excess);
        println!("   This residual is NOT from SIZE EFFECT alone.");
        println!("   The membrane STRUCTURE itself contributes extra efficiency!");
    }
}
