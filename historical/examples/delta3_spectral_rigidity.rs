// Δ₃ Spectral Rigidity Analysis for Coordinate Constellation Primes
//
// Computes discrete Δ₃ (spectral rigidity) and small-s repulsion exponent β
// for coordinate constellation primes vs random primes.
//
// HYPOTHESIS: Coordinate constellations show LOWER rigidity (more random)
// than Poisson baseline, due to φ-constraint creating gaps NOT correlations.
//
// OUTPUT: Rationalized results (num/den with scale 10⁶) for Agda verification
//
// Usage: cargo run --example delta3_spectral_rigidity --release

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime;

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

fn collect_primes_sorted(base: u32, middle_values: &[u32], limit: u64) -> Vec<BigUint> {
    let mut primes = Vec::new();

    for &middle in middle_values {
        for z in 1..base {
            for y in 1..base {
                for x in 1..base {
                    let candidate = septuplet_membrane(middle, x, y, z, base);

                    if candidate > BigUint::from(limit) {
                        continue;
                    }

                    if is_prime(&candidate) {
                        primes.push(candidate);
                    }
                }
            }
        }
    }

    primes.sort();
    primes.dedup();
    primes
}

// Normalize gaps: divide by mean gap
fn normalize_gaps(primes: &[BigUint]) -> Vec<f64> {
    if primes.len() < 2 {
        return Vec::new();
    }

    let mut gaps = Vec::new();
    for i in 0..primes.len() - 1 {
        let gap = (&primes[i + 1] - &primes[i])
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);
        gaps.push(gap);
    }

    let mean_gap = gaps.iter().sum::<f64>() / gaps.len() as f64;
    gaps.iter().map(|&g| g / mean_gap).collect()
}

// Δ₃ discrete proxy: MSE of best linear fit on cumulative sums
fn delta3_discrete(normalized_gaps: &[f64], window_len: usize) -> f64 {
    if normalized_gaps.len() < window_len + 2 {
        return f64::NAN;
    }

    // Cumulative sum
    let mut cumsum = vec![0.0; normalized_gaps.len() + 1];
    for i in 0..normalized_gaps.len() {
        cumsum[i + 1] = cumsum[i] + normalized_gaps[i];
    }

    let mut mses = Vec::new();

    // Slide window across cumulative sum
    for start in 0..=normalized_gaps.len() - window_len {
        // Fit linear: y_j = A*j + B where j=0..window_len
        let mut sum_j = 0.0;
        let mut sum_y = 0.0;
        let mut sum_jj = 0.0;
        let mut sum_jy = 0.0;

        for j in 0..=window_len {
            let jf = j as f64;
            let y = cumsum[start + j];
            sum_j += jf;
            sum_y += y;
            sum_jj += jf * jf;
            sum_jy += jf * y;
        }

        let n = (window_len + 1) as f64;
        let denom = n * sum_jj - sum_j * sum_j;

        let (a, b) = if denom.abs() < 1e-12 {
            (0.0, sum_y / n)
        } else {
            let a = (n * sum_jy - sum_j * sum_y) / denom;
            let b = (sum_y - a * sum_j) / n;
            (a, b)
        };

        // Compute MSE of fit
        let mut mse = 0.0;
        for j in 0..=window_len {
            let jf = j as f64;
            let y = cumsum[start + j];
            let yhat = a * jf + b;
            mse += (y - yhat).powi(2);
        }
        mse /= (window_len + 1) as f64;
        mses.push(mse);
    }

    // Average MSE across all windows
    if mses.is_empty() {
        f64::NAN
    } else {
        mses.iter().sum::<f64>() / mses.len() as f64
    }
}

// Small-s repulsion exponent: fit log P(s) ≈ c + β log s
fn repulsion_beta(normalized_gaps: &[f64], smax: f64, bins: usize) -> f64 {
    if normalized_gaps.len() < 10 {
        return f64::NAN;
    }

    // Create histogram for s ∈ (0, smax]
    let mut hist = vec![0usize; bins];
    for &s in normalized_gaps.iter() {
        if s > 0.0 && s <= smax {
            let idx = ((s / smax * bins as f64).floor() as usize).min(bins - 1);
            hist[idx] += 1;
        }
    }

    // Log-log regression
    let mut xs = Vec::new();
    let mut ys = Vec::new();

    for i in 0..bins {
        let center = (i as f64 + 0.5) * (smax / bins as f64);
        let count = hist[i] as f64;

        if count > 0.0 {
            xs.push(center.ln());
            ys.push(count.ln());
        }
    }

    if xs.len() < 3 {
        return f64::NAN;
    }

    // Linear regression: β is the slope
    let n = xs.len() as f64;
    let sum_x = xs.iter().sum::<f64>();
    let sum_y = ys.iter().sum::<f64>();
    let sum_xx = xs.iter().map(|x| x * x).sum::<f64>();
    let sum_xy = xs.iter().zip(ys.iter()).map(|(x, y)| x * y).sum::<f64>();

    let denom = n * sum_xx - sum_x * sum_x;
    if denom.abs() < 1e-12 {
        return f64::NAN;
    }

    (n * sum_xy - sum_x * sum_y) / denom
}

fn rationalize(x: f64, scale: u64) -> (i64, u64) {
    let num = (x * scale as f64).round() as i64;
    (num, scale)
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║     Δ₃ SPECTRAL RIGIDITY - COORDINATE CONSTELLATIONS        ║");
    println!("║     Compute-then-Verify Pipeline for Agda                   ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    println!("THEORETICAL BACKGROUND:");
    println!("  Δ₃(L): Spectral rigidity statistic");
    println!("    - Measures deviation from best linear fit");
    println!("    - GUE: Δ₃ ≈ (1/π²)[ln L - 0.007]");
    println!("    - Poisson: Δ₃ ≈ L/15");
    println!();
    println!("  β: Small-s repulsion exponent");
    println!("    - From P(s) ~ s^β for small s");
    println!("    - GUE: β ≈ 2 (quadratic repulsion)");
    println!("    - Poisson: β ≈ 0 (no repulsion)");
    println!();

    let base = 14u32; // Hexagonal base (φ=6)
    let limit = 1_000_000_000_000u64;
    let scale = 1_000_000u64;

    let middle_values: Vec<u32> = (1..base).filter(|&m| is_coprime(m, base)).collect();

    println!("═══════════════════════════════════════════════════════════════");
    println!("COORDINATE CONSTELLATION ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("  Base: {}", base);
    println!("  φ(base): {}", middle_values.len());
    println!("  Middle values: {:?}", middle_values);
    println!();

    println!("  Collecting primes...");
    let primes = collect_primes_sorted(base, &middle_values, limit);
    println!("  Found {} primes", primes.len());
    println!();

    if primes.len() < 50 {
        println!("  ⚠ Too few primes for statistical analysis");
        return;
    }

    println!("  Normalizing gaps...");
    let normalized = normalize_gaps(&primes);
    println!("  Computed {} normalized gaps", normalized.len());
    println!();

    // Compute Δ₃
    let window_len = 20;
    let delta3 = delta3_discrete(&normalized, window_len);
    println!("  SPECTRAL RIGIDITY Δ₃:");
    println!("    Window length: {}", window_len);
    println!("    Δ₃ value: {:.6}", delta3);

    let (d3_num, d3_den) = rationalize(delta3, scale);
    println!("    Rationalized: {}/{} (×10⁶)", d3_num, d3_den);
    println!();

    // Compute β
    let smax = 0.6;
    let bins = 24;
    let beta = repulsion_beta(&normalized, smax, bins);
    println!("  SMALL-S REPULSION β:");
    println!("    s_max: {}", smax);
    println!("    Bins: {}", bins);
    println!("    β value: {:.6}", beta);

    let (beta_num, beta_den) = rationalize(beta, scale);
    println!("    Rationalized: {}/{} (×10⁶)", beta_num, beta_den);
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("INTERPRETATION");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("  EXPECTED FOR COORDINATE CONSTELLATIONS:");
    println!("    - Δ₃: Higher than GUE (less rigid, more random)");
    println!("    - β: Close to 0 (minimal repulsion, as we found)");
    println!();

    println!("  WHY: φ-constraint creates GEOMETRIC ORDER (eigenspace)");
    println!("       but NOT SPECTRAL CORRELATION (spacing statistics)");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("CSV OUTPUT FOR AGDA VERIFICATION");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("metric,group,val_num,val_den,count");
    println!(
        "delta3,constellation,{},{},{}",
        d3_num,
        d3_den,
        normalized.len()
    );
    println!(
        "beta,constellation,{},{},{}",
        beta_num,
        beta_den,
        normalized.len()
    );
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("AGDA VERIFICATION TEMPLATE");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("-- Paste these values into CheckD3.agda:");
    println!();
    println!("delta3_constellation : Q");
    println!("delta3_constellation = q {} {}", d3_num, d3_den);
    println!();
    println!("beta_constellation : Q");
    println!("beta_constellation = q {} {}", beta_num, beta_den);
    println!();

    println!("-- Then verify bounds:");
    println!("-- For minimal repulsion: beta < 0.5 (much less than GUE ≈ 2)");
    println!("-- For higher randomness: delta3 > GUE_baseline");
    println!();
}
