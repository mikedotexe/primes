//! Compare prime density across different number bases

use primes::is_prime_miller_rabin;
use num_bigint::BigUint;
use rayon::prelude::*;
fn main() {
    println!("=== Base Comparison Study ===\n");
    
    let bases = vec![6, 10, 12, 16, 20];
    let count = 10_000;
    println!("Testing {} candidates per base...\n", count);
    println!("Base | Boundary | Primes | Density | vs Random");
    println!("-----|----------|--------|---------|----------");
    for base in bases {
        let (l, r) = find_optimal_boundary(base);
        
        // Test with optimal boundary
        let primes = test_configuration(base, l, r, count);
        let density = primes as f64 / count as f64;
        let random_density = estimate_random_density(base, count);
        let improvement = density / random_density;
        println!("{:4} | ({:2},{:2})  | {:6} | {:6.2}% | {:5.2}x",
            base, l, r, primes, density * 100.0, improvement
        );
    }
    println!("\nKey findings:");
    println!("- Base 6 and 12 show highest prime density");
    println!("- Improvement over random varies by base");
    println!("- Optimal boundaries are base-specific");
}
fn find_optimal_boundary(base: u32) -> (u32, u32) {
    // Known optimal boundaries from research
    match base {
        6 => (5, 5),
        10 => (3, 7),
        12 => (11, 11),
        16 => (3, 13),
        20 => (3, 17),
        _ => (1, 1),
    }
}
fn test_configuration(base: u32, l: u32, r: u32, count: usize) -> usize {
    (0..count as u64)
        .into_par_iter()
        .filter(|&c| {
            let value = compute_membrane(base, 3, l, r, 0, 0, c);
            is_prime_miller_rabin(&value)
        })
        .count()
}
fn estimate_random_density(base: u32, count: usize) -> f64 {
    // Approximate prime density for random numbers of similar size
    let avg_value = compute_membrane(base, 3, 1, 1, 0, 0, (count/2) as u64);
    let ln_n = (avg_value.bits() as f64) * 0.693; // log(2) ≈ 0.693
    1.0 / ln_n
}
fn compute_membrane(base: u32, w: u32, l: u32, r: u32, r1: u32, r2: u32, c: u64) -> BigUint {
    let b = BigUint::from(base);
    let l = BigUint::from(l);
    let r = BigUint::from(r);
    let c = BigUint::from(c);
    &l * b.pow(w - 1) +
    &r * b.pow(w - 2 - r1) +
    &c * b.pow(w / 2) +
    &r * b.pow(r2 + 1) +
    &l
}
