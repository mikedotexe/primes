//! Basic membrane prime generation example

use primes::is_prime_miller_rabin;
use num_bigint::BigUint;
fn main() {
    println!("=== Basic Membrane Prime Generation ===\n");
    
    // Configuration
    let base = 6;
    let width = 3;
    let (l, r) = (5, 5);  // Boundary digits
    let (r1, r2) = (0, 0); // Zero padding
    println!("Configuration:");
    println!("  Base: {base}");
    println!("  Width: {width}");
    println!("  Boundary: L={l}, R={r}");
    println!("  Padding: r1={r1}, r2={r2}");
    println!();
    // Generate and test first 20 candidates
    let mut primes_found = 0;
    println!("Testing candidates 0-19:");
    for c in 0..20 {
        let value = compute_membrane(base, width, l, r, r1, r2, c);
        let is_prime = is_prime_miller_rabin(&value);
        
        println!("  C={:2} → {:10} {}", 
            c, 
            value, 
            if is_prime { "✓ PRIME" } else { "" }
        );
        if is_prime {
            primes_found += 1;
        }
    }
    println!("\nFound {} primes ({}% density)", 
        primes_found, 
        primes_found * 100 / 20
    );
}
fn compute_membrane(base: u32, w: u32, l: u32, r: u32, r1: u32, r2: u32, c: u64) -> BigUint {
    let b = BigUint::from(base);
    let l = BigUint::from(l);
    let r = BigUint::from(r);
    let c = BigUint::from(c);
    // Membrane formula: L*b^(w-1) + R*b^(w-2-r1) + C*b^(w/2) + R*b^(r2+1) + L
    &l * b.pow(w - 1) +
    &r * b.pow(w - 2 - r1) +
    &c * b.pow(w / 2) +
    &r * b.pow(r2 + 1) +
    &l
}
