//! Phase Shift Analysis: Why does optimal mod6 vary by base?
//!
//! OBSERVATION:
//!   Base 210 (7):        optimal at mod6 ≡ 2
//!   Base 2310 (7,11):    optimal at mod6 ≡ 3
//!   Base 30030 (7,11,13): optimal at mod6 ≡ 2
//!
//! HYPOTHESIS: The optimal phase is determined by how ord(10) mod p
//! combines across the odd prime factors of the base.
//!
//! This explores whether we can PREDICT the optimal phase from
//! number-theoretic properties alone.

use num_bigint::BigUint;
use primes::is_prime_miller_rabin;
use std::collections::HashMap;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % m; }
        exp >>= 1;
        base = base * base % m;
    }
    result
}

fn multiplicative_order(a: u64, n: u64) -> u64 {
    if gcd(a, n) != 1 { return 0; }
    let mut ord = 1;
    let mut power = a % n;
    while power != 1 {
        power = power * a % n;
        ord += 1;
        if ord > n { return 0; } // Safety
    }
    ord
}

fn factor_odd_primes(mut n: u64) -> Vec<u64> {
    // Remove factors of 2 and 5
    while n % 2 == 0 { n /= 2; }
    while n % 5 == 0 { n /= 5; }

    let mut primes = vec![];
    let mut d = 3;
    while d * d <= n {
        if n % d == 0 {
            primes.push(d);
            while n % d == 0 { n /= d; }
        }
        d += 2;
    }
    if n > 1 { primes.push(n); }
    primes
}

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

fn first_coprime(base: u64) -> u64 {
    let factors: Vec<u64> = {
        let mut n = base;
        let mut fs = vec![];
        for p in [2, 3, 5, 7, 11, 13, 17, 19] {
            if n % p == 0 { fs.push(p); while n % p == 0 { n /= p; } }
        }
        fs
    };
    (1..base).find(|&d| factors.iter().all(|&p| d % p != 0)).unwrap_or(1)
}

fn measure_efficiency_by_mod6(base: u64, samples: usize) -> Vec<(usize, f64)> {
    let right = first_coprime(base);
    let mut rng = 31415u64 + base;
    let test_lengths: Vec<usize> = (6..=30).collect();

    let mut by_mod6: HashMap<usize, Vec<f64>> = HashMap::new();

    for &seed_len in &test_lengths {
        let mut primes = 0;
        let mut total_dig = 0.0;

        for _ in 0..samples {
            let seed = random_seed_with_length(base, seed_len, &mut rng);
            let mem = membrane_value(base, 1, &seed, right);
            total_dig += mem.to_string().len() as f64;
            if is_prime_miller_rabin(&mem) { primes += 1; }
        }

        let rate = primes as f64 / samples as f64;
        let mean_dig = total_dig / samples as f64;
        let eff = rate / (1.0 / (mean_dig * 2.303));

        by_mod6.entry(seed_len % 6).or_default().push(eff);
    }

    let mut results: Vec<(usize, f64)> = vec![];
    for r in 0..6 {
        let effs = by_mod6.get(&r).unwrap();
        let mean = effs.iter().sum::<f64>() / effs.len() as f64;
        results.push((r, mean));
    }

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    results
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║          PHASE SHIFT ANALYSIS: PREDICTING OPTIMAL MOD6          ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let primorials: Vec<(u64, &str)> = vec![
        (30, "P₃ = 2×3×5"),
        (210, "P₄ = 2×3×5×7"),
        (2310, "P₅ = 2×3×5×7×11"),
        (30030, "P₆ = 2×3×5×7×11×13"),
    ];

    println!("PART 1: Number-Theoretic Properties of Each Base\n");
    println!("{:>8} {:>20} {:>12} {:>20}", "Base", "Odd Primes", "ord(10)", "Phase Signature");
    println!("{}", "-".repeat(65));

    let mut predictions: HashMap<u64, u64> = HashMap::new();

    for (base, name) in &primorials {
        let odd_primes = factor_odd_primes(*base);

        // Compute ord(10) mod p for each odd prime
        let mut orders: Vec<(u64, u64)> = vec![];
        let mut phase_sum = 0u64;

        for &p in &odd_primes {
            let ord = multiplicative_order(10, p);
            orders.push((p, ord));
            // Hypothesis: optimal phase relates to sum of orders mod 6
            phase_sum += ord;
        }

        let predicted_phase = phase_sum % 6;
        predictions.insert(*base, predicted_phase);

        let primes_str = odd_primes.iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let orders_str = orders.iter()
            .map(|(p, o)| format!("ord₁₀({})={}", p, o))
            .collect::<Vec<_>>()
            .join(", ");

        println!("{:>8} {:>20} {:>12} {:>20}", base, primes_str,
                 format!("Σ mod 6 = {}", predicted_phase), "");
        println!("         {}: {}", name, orders_str);
        println!();
    }

    println!("\n{}", "=".repeat(70));
    println!("PART 2: Empirical Measurement vs Prediction\n");

    let samples = 600;

    for (base, name) in &primorials {
        println!("Testing {} ({})...", name, base);

        let results = measure_efficiency_by_mod6(*base, samples);
        let observed_best = results[0].0;
        let predicted = predictions.get(base).unwrap_or(&0);

        println!("  Empirical ranking (best to worst):");
        for (i, (r, eff)) in results.iter().enumerate() {
            let marker = if i == 0 { "← BEST" } else { "" };
            println!("    mod6≡{}: efficiency {:.2} {}", r, eff, marker);
        }

        let match_status = if observed_best == *predicted as usize {
            "✓ MATCH"
        } else {
            "✗ MISMATCH"
        };

        println!("  Predicted optimal: mod6≡{}", predicted);
        println!("  Observed optimal:  mod6≡{}", observed_best);
        println!("  Status: {}\n", match_status);
    }

    // Try alternative hypotheses
    println!("\n{}", "=".repeat(70));
    println!("PART 3: Alternative Phase Predictors\n");

    println!("Testing different formulas for predicting optimal phase:\n");

    for (base, name) in &primorials {
        let odd_primes = factor_odd_primes(*base);
        let results = measure_efficiency_by_mod6(*base, 400);
        let observed = results[0].0 as u64;

        // Hypothesis A: Sum of orders mod 6
        let sum_orders: u64 = odd_primes.iter()
            .map(|&p| multiplicative_order(10, p))
            .sum();
        let pred_a = sum_orders % 6;

        // Hypothesis B: Product of orders mod 6
        let prod_orders: u64 = odd_primes.iter()
            .map(|&p| multiplicative_order(10, p))
            .product();
        let pred_b = prod_orders % 6;

        // Hypothesis C: Number of odd primes mod 6
        let pred_c = odd_primes.len() as u64 % 6;

        // Hypothesis D: Sum of (p-1)/ord(10,p) mod 6
        let sum_quotients: u64 = odd_primes.iter()
            .map(|&p| {
                let ord = multiplicative_order(10, p);
                if ord > 0 { (p - 1) / ord } else { 0 }
            })
            .sum();
        let pred_d = sum_quotients % 6;

        // Hypothesis E: XOR of orders mod 6
        let xor_orders: u64 = odd_primes.iter()
            .map(|&p| multiplicative_order(10, p))
            .fold(0, |acc, x| acc ^ x);
        let pred_e = xor_orders % 6;

        println!("{} (observed optimal: mod6≡{}):", name, observed);
        println!("  A. Σ ord(10,p) mod 6       = {} {}",
                 pred_a, if pred_a == observed { "✓" } else { "" });
        println!("  B. Π ord(10,p) mod 6       = {} {}",
                 pred_b, if pred_b == observed { "✓" } else { "" });
        println!("  C. #(odd primes) mod 6     = {} {}",
                 pred_c, if pred_c == observed { "✓" } else { "" });
        println!("  D. Σ (p-1)/ord mod 6       = {} {}",
                 pred_d, if pred_d == observed { "✓" } else { "" });
        println!("  E. XOR of orders mod 6     = {} {}",
                 pred_e, if pred_e == observed { "✓" } else { "" });
        println!();
    }

    println!("\n{}", "=".repeat(70));
    println!("SUMMARY");
    println!("{}", "=".repeat(70));

    println!("\nIf any hypothesis matches all 4 bases, we have a predictive formula!");
    println!("If none match perfectly, the phase shift may involve more complex interactions.");
}
