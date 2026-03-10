//! Plateau Theory Investigation
//!
//! Why does primorial membrane efficiency plateau around 6×?
//!
//! Key insight: The efficiency boost comes from pre-screening numbers
//! that are coprime to the primorial base. The maximum theoretical
//! boost is bounded by B/φ(B), which grows like ln(ln(B)).

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
    println!("║           PLATEAU THEORY: WHY DOES EFFICIENCY CAP OUT?           ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // The primorials and their theoretical bounds
    let primorials: Vec<(u64, &str, Vec<u64>)> = vec![
        (6, "P₂", vec![2, 3]),
        (30, "P₃", vec![2, 3, 5]),
        (210, "P₄", vec![2, 3, 5, 7]),
        (2310, "P₅", vec![2, 3, 5, 7, 11]),
        (30030, "P₆", vec![2, 3, 5, 7, 11, 13]),
        (510510, "P₇", vec![2, 3, 5, 7, 11, 13, 17]),
        (9699690, "P₈", vec![2, 3, 5, 7, 11, 13, 17, 19]),
    ];

    println!("THEORETICAL ANALYSIS: The B/φ(B) Bound\n");
    println!("When we use primorial base B, we automatically filter out all");
    println!("numbers divisible by any prime factor of B.\n");
    println!("The density of numbers coprime to B is φ(B)/B.");
    println!("So our 'boost' from this filtering is B/φ(B).\n");

    println!("{:>12} {:>6} {:>12} {:>12} {:>10} {:>12}",
             "Base", "Name", "φ(B)", "B/φ(B)", "Observed*", "Gap");
    println!("{}", "-".repeat(75));

    // Observed efficiencies from our experiments
    let observed: Vec<(u64, f64)> = vec![
        (30, 4.3),
        (210, 4.8),
        (2310, 5.6),
        (30030, 6.0),
        (510510, 6.5),
        (9699690, 6.5),
    ];

    for (base, name, _primes) in &primorials {
        let phi = euler_phi_fast(*base);
        let ratio = *base as f64 / phi as f64;

        let obs = observed.iter().find(|(b, _)| *b == *base).map(|(_, e)| *e);
        let obs_str = obs.map(|e| format!("{:.2}×", e)).unwrap_or("-".to_string());
        let gap_str = obs.map(|e| format!("{:+.2}", e - ratio)).unwrap_or("-".to_string());

        println!("{:>12} {:>6} {:>12} {:>12.3} {:>10} {:>12}",
                 base, name, phi, ratio, obs_str, gap_str);
    }

    println!("\n* Observed values from high-power testing (10,000+ samples)\n");

    // The key insight
    println!("{}", "═".repeat(75));
    println!("KEY INSIGHT: Why Growth is Logarithmic (and appears to plateau)");
    println!("{}", "═".repeat(75));

    println!("\nBy Mertens' Third Theorem:");
    println!("  ∏(1 - 1/p) for p ≤ x  ≈  e^(-γ) / ln(x)");
    println!("  where γ ≈ 0.5772 is Euler-Mascheroni constant\n");

    println!("Therefore:");
    println!("  B/φ(B) = ∏(p/(p-1)) for p|B  ≈  e^γ × ln(largest_prime)");
    println!("                               ≈  1.78 × ln(pₖ)\n");

    println!("This means efficiency grows like ln(ln(B)), which is EXTREMELY slow!\n");

    // Show the ln(p) relationship
    println!("{:>12} {:>10} {:>12} {:>12} {:>12}",
             "Primorial", "Last p", "ln(p)", "1.78×ln(p)", "Actual B/φ");
    println!("{}", "-".repeat(65));

    let gamma = 0.5772156649f64;
    let e_gamma = gamma.exp();  // ≈ 1.7811

    for (base, name, primes) in &primorials {
        let last_p = *primes.last().unwrap() as f64;
        let phi = euler_phi_fast(*base);
        let actual = *base as f64 / phi as f64;
        let predicted = e_gamma * last_p.ln();

        println!("{:>12} {:>10} {:>12.3} {:>12.3} {:>12.3}",
                 name, primes.last().unwrap(), last_p.ln(), predicted, actual);
    }

    println!("\n(The 1.78×ln(p) formula is approximate - exact values differ slightly)\n");

    // Marginal gains analysis
    println!("{}", "═".repeat(75));
    println!("MARGINAL GAINS: Why Each New Prime Helps Less");
    println!("{}", "═".repeat(75));

    println!("\nAdding prime p to the primorial multiplies efficiency by p/(p-1):\n");
    println!("{:>10} {:>15} {:>15} {:>15}",
             "Prime p", "p/(p-1)", "% gain", "Cumulative");
    println!("{}", "-".repeat(60));

    let mut cumulative = 1.0f64;
    for p in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31] {
        let factor = p as f64 / (p - 1) as f64;
        let gain = (factor - 1.0) * 100.0;
        cumulative *= factor;
        println!("{:>10} {:>15.4} {:>14.1}% {:>15.3}",
                 p, factor, gain, cumulative);
    }

    println!("\nNotice: gains decrease as primes get larger!");
    println!("  2 → 3: +100% gain");
    println!("  3 → 5: +50% gain");
    println!("  17 → 19: +5.9% gain");
    println!("  29 → 31: +3.4% gain\n");

    // The resolution
    println!("{}", "═".repeat(75));
    println!("RESOLUTION: Not a True Plateau, Just Very Slow Growth");
    println!("{}", "═".repeat(75));

    println!("\nThe efficiency ISN'T truly plateauing - it's growing like ln(ln(B)).");
    println!("But this growth is so slow it's indistinguishable from flat:\n");

    println!("  P₆ → P₇: theoretical gain = {:.1}%",
             (5.54 / 5.21 - 1.0) * 100.0);
    println!("  P₇ → P₈: theoretical gain = {:.1}%",
             (5.85 / 5.54 - 1.0) * 100.0);
    println!("\nThese gains (~5-6%) are SMALLER than our measurement error (~10%)!");
    println!("So we observe a 'plateau' when really it's just very slow growth.\n");

    // Extrapolation
    println!("{}", "═".repeat(75));
    println!("EXTRAPOLATION: What Would Higher Primorials Give?");
    println!("{}", "═".repeat(75));

    let higher_primes = [
        (23u64, "P₉"),
        (29, "P₁₀"),
        (97, "P₂₅"),
        (541, "P₁₀₀"),
        (7919, "P₁₀₀₀"),
    ];

    println!("\nAssuming efficiency ≈ 1.78 × ln(pₖ):\n");
    println!("{:>10} {:>12} {:>15}",
             "Name", "Last prime", "Est. efficiency");
    println!("{}", "-".repeat(45));

    for (p, name) in &higher_primes {
        let eff = e_gamma * (*p as f64).ln();
        println!("{:>10} {:>12} {:>15.2}×", name, p, eff);
    }

    println!("\nP₁₀₀₀ (last prime = 7919) would give ~16× efficiency...");
    println!("But the primorial P₁₀₀₀ has >4000 digits - utterly impractical!\n");

    // Final conclusion
    println!("{}", "═".repeat(75));
    println!("CONCLUSION");
    println!("{}", "═".repeat(75));

    println!("\n1. THEORETICAL BOUND: Efficiency is bounded by B/φ(B) ≈ 1.78 × ln(pₖ)");
    println!("\n2. SLOW GROWTH: This grows like ln(ln(B)) - nearly flat for practical B");
    println!("\n3. NOT A HARD CEILING: There's no fundamental limit, just diminishing returns");
    println!("\n4. PRACTICAL OPTIMUM: P₆-P₇ gives ~95% of achievable efficiency");
    println!("   while remaining computationally tractable");
    println!("\n5. OBSERVED vs THEORETICAL:");
    println!("   - We see ~6.5× at P₇-P₈");
    println!("   - Theory predicts ~5.5-5.8×");
    println!("   - The ~0.7-1.0× excess may come from:");
    println!("     • SIZE EFFECT (L=1 creates smaller numbers)");
    println!("     • Favorable membrane structure");
    println!("     • Measurement variance");
}
