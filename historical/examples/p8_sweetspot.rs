//! P₈ Sweet Spot Verification
//!
//! Verify the 10.43× efficiency at seed_len=18 for P₈

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

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           P₈ SWEET SPOT VERIFICATION                             ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let base = 9699690u64;
    let right = 1u64;  // First coprime

    println!("Testing P₈ = {} with L=1, R={}\n", base, right);

    // Test the sweet spot (seed_len=18) with multiple RNG seeds
    let seed_lens = [16, 17, 18, 19, 20];
    let samples = 2000;
    let trials = 5;

    println!("{:>10} {:>8} {:>8} {:>10} {:>12}",
             "seed_len", "mean%", "std%", "mean_eff", "std_eff");
    println!("{}", "-".repeat(55));

    for &seed_len in &seed_lens {
        let mut efficiencies = vec![];

        for trial in 0..trials {
            let mut rng = 12345678u64 + trial * 99999 + seed_len as u64 * 7777;
            let mut primes = 0;
            let mut digits_sum = 0.0;

            for _ in 0..samples {
                let seed = random_seed_with_length(base, seed_len, &mut rng);
                let mem = membrane_value(base, 1, &seed, right);
                digits_sum += mem.to_string().len() as f64;
                if is_prime_miller_rabin(&mem) {
                    primes += 1;
                }
            }

            let rate = primes as f64 / samples as f64;
            let mean_digits = digits_sum / samples as f64;
            let pnt_expected = 1.0 / (mean_digits * 2.303);
            let eff = rate / pnt_expected;
            efficiencies.push((rate * 100.0, eff));
        }

        let rates: Vec<f64> = efficiencies.iter().map(|(r, _)| *r).collect();
        let effs: Vec<f64> = efficiencies.iter().map(|(_, e)| *e).collect();

        let rate_mean: f64 = rates.iter().sum::<f64>() / trials as f64;
        let rate_std: f64 = (rates.iter().map(|r| (r - rate_mean).powi(2)).sum::<f64>()
                            / (trials - 1) as f64).sqrt();
        let eff_mean: f64 = effs.iter().sum::<f64>() / trials as f64;
        let eff_std: f64 = (effs.iter().map(|e| (e - eff_mean).powi(2)).sum::<f64>()
                           / (trials - 1) as f64).sqrt();

        let marker = if seed_len == 18 { " ◄ TARGET" } else { "" };
        println!("{:>10} {:>7.2}% {:>7.2}% {:>10.3}× {:>10.3}{}",
                 seed_len, rate_mean, rate_std, eff_mean, eff_std, marker);
    }

    // High-confidence test at seed_len=18
    println!("\n{}", "═".repeat(60));
    println!("HIGH-CONFIDENCE TEST: seed_len=18 with 10,000 samples");
    println!("{}", "═".repeat(60));

    let seed_len = 18;
    let samples = 10000;

    let mut rng = 314159265u64;
    let mut primes = 0;
    let mut digits_sum = 0.0;

    for _ in 0..samples {
        let seed = random_seed_with_length(base, seed_len, &mut rng);
        let mem = membrane_value(base, 1, &seed, right);
        digits_sum += mem.to_string().len() as f64;
        if is_prime_miller_rabin(&mem) {
            primes += 1;
        }
    }

    let rate = primes as f64 / samples as f64;
    let mean_digits = digits_sum / samples as f64;
    let pnt_expected = 1.0 / (mean_digits * 2.303);
    let eff = rate / pnt_expected;

    // 95% CI for rate
    let se = (rate * (1.0 - rate) / samples as f64).sqrt();
    let ci_low = rate - 1.96 * se;
    let ci_high = rate + 1.96 * se;

    let eff_low = ci_low / pnt_expected;
    let eff_high = ci_high / pnt_expected;

    println!("\nResults:");
    println!("  Primes found: {} / {}", primes, samples);
    println!("  Prime rate: {:.2}% (95% CI: {:.2}% - {:.2}%)",
             rate * 100.0, ci_low * 100.0, ci_high * 100.0);
    println!("  Mean digits: {:.1}", mean_digits);
    println!("  PNT expected: {:.4}%", pnt_expected * 100.0);
    println!();
    println!("  EFFICIENCY: {:.3}× PNT (95% CI: {:.3}× - {:.3}×)", eff, eff_low, eff_high);

    if eff > 10.0 {
        println!("\n🏆 CONFIRMED: P₈ at seed_len=18 exceeds 10× PNT efficiency!");
    } else if eff > 8.0 {
        println!("\n✓ P₈ at seed_len=18 achieves very high efficiency (>8× PNT)");
    }

    // Compare with baseline
    println!("\n{}", "═".repeat(60));
    println!("COMPARISON WITH BASELINES");
    println!("{}", "═".repeat(60));

    // P₇ baseline at similar digit count
    let p7 = 510510u64;
    let p7_seed_len = 25;  // Approximately same decimal digits

    let mut rng = 314159265u64;
    let mut p7_primes = 0;
    let mut p7_digits_sum = 0.0;

    for _ in 0..samples {
        let seed = random_seed_with_length(p7, p7_seed_len, &mut rng);
        let mem = membrane_value(p7, 1, &seed, 1);
        p7_digits_sum += mem.to_string().len() as f64;
        if is_prime_miller_rabin(&mem) {
            p7_primes += 1;
        }
    }

    let p7_rate = p7_primes as f64 / samples as f64;
    let p7_mean_digits = p7_digits_sum / samples as f64;
    let p7_pnt_expected = 1.0 / (p7_mean_digits * 2.303);
    let p7_eff = p7_rate / p7_pnt_expected;

    println!("\nP₇ at seed_len={} (similar digit count):", p7_seed_len);
    println!("  Rate: {:.2}%, Efficiency: {:.3}×, Mean digits: {:.1}",
             p7_rate * 100.0, p7_eff, p7_mean_digits);
    println!("\nP₈ at seed_len=18:");
    println!("  Rate: {:.2}%, Efficiency: {:.3}×, Mean digits: {:.1}",
             rate * 100.0, eff, mean_digits);

    let gain = (eff - p7_eff) / p7_eff * 100.0;
    println!("\nP₈ advantage over P₇ at comparable scale: {:+.1}%", gain);
}
