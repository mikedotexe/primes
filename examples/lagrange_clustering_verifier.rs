use num_bigint::BigUint;
use num_traits::One;
fn is_prime(n: &BigUint) -> bool {
    if n <= &BigUint::one() {
        return false;
    }
    if n == &BigUint::from(2u32) {
        return true;
    }
    if !n.bit(0) {
        return false;
    }

    // Simple trial division for verification
    let mut i = BigUint::from(3u32);
    let sqrt = n.sqrt();
    while i <= sqrt {
        if n % &i == BigUint::from(0u32) {
            return false;
        }
        i += 2u32;
    }
    true
}

fn main() {
    println!("🔬 Verifying Lagrange Point Clustering");
    println!("=====================================\n");
    println!("Heuristic scan only: compare equal-size windows before inferring density effects.\n");
    // The two membrane primes from LAGRANGE_PROOF.md
    let prime1 = BigUint::from(303050303u64);
    let prime2 = BigUint::from(307050703u64);
    println!(
        "Membrane Prime 1: {} (verified: {})",
        prime1,
        is_prime(&prime1)
    );
    println!(
        "Membrane Prime 2: {} (verified: {})",
        prime2,
        is_prime(&prime2)
    );
    // Calculate L1 (midpoint)
    let l1 = (&prime1 + &prime2) / 2u32;
    println!("\nL1 Point (midpoint): {}", l1);
    // Check for primes within 1000 units of L1
    let radius = 1000u32;
    let start = &l1 - radius;
    let end = &l1 + radius;
    println!("\nSearching for primes within {} units of L1...", radius);
    let mut primes_found = Vec::new();
    let mut current = start.clone();
    while current <= end {
        if is_prime(&current) {
            let distance = if current >= l1 {
                &current - &l1
            } else {
                &l1 - &current
            };
            primes_found.push((current.clone(), distance));
        }
        current += 1u32;
    }
    println!("\nFound {} primes near L1!", primes_found.len());
    // Show closest primes
    primes_found.sort_by_key(|(_, dist)| dist.clone());
    println!("\nClosest primes to L1:");
    for (prime, dist) in primes_found.iter().take(10) {
        println!("  {} at distance {}", prime, dist);
    }

    // Now let's understand the "desert" concept
    println!("\n🏜️ Understanding Prime Deserts in Membrane Structure");
    println!("===================================================");
    // Analyze the membrane structure
    let membrane1_str = prime1.to_string();
    let membrane2_str = prime2.to_string();
    println!("\nMembrane 1 structure: {}", membrane1_str);
    println!("Pattern: 3-0-3-0-5-0-3-0-3");
    println!("Zeros create 'deserts' where no primes can exist");
    println!("\nMembrane 2 structure: {}", membrane2_str);
    println!("Pattern: 3-0-7-0-5-0-7-0-3");
    // Calculate prime density in different regions
    println!("\n📊 Prime Density Analysis");
    println!("========================");
    // Region 1: Around membrane prime 1 (same radius as midpoint window)
    let region1_start = &prime1 - radius;
    let region1_end = &prime1 + radius;
    let mut region1_primes = 0;
    current = region1_start;
    while current <= region1_end {
        if is_prime(&current) {
            region1_primes += 1;
        }
        current += 1u32;
    }

    // Region 2: Around L1 point
    let region2_primes = primes_found.len();
    // Region 3: Around membrane prime 2 (same radius as midpoint window)
    let region3_start = &prime2 - radius;
    let region3_end = &prime2 + radius;
    let mut region3_primes = 0;
    current = region3_start;
    while current <= region3_end {
        if is_prime(&current) {
            region3_primes += 1;
        }
        current += 1u32;
    }

    let window_size = (radius * 2 + 1) as f64;
    let region1_density = region1_primes as f64 / window_size;
    let region2_density = region2_primes as f64 / window_size;
    let region3_density = region3_primes as f64 / window_size;

    println!("\nPrime count in equal {}-number windows:", window_size as usize);
    println!("  Around Membrane 1: {} primes", region1_primes);
    println!("  Around L1 (oasis): {} primes", region2_primes);
    println!("  Around Membrane 2: {} primes", region3_primes);
    println!("\nPrime densities:");
    println!("  Around Membrane 1: {:.4}", region1_density);
    println!("  Around L1:         {:.4}", region2_density);
    println!("  Around Membrane 2: {:.4}", region3_density);

    println!("\n💡 Current read:");
    let endpoint_density = (region1_density + region3_density) / 2.0;
    let uplift = (region2_density / endpoint_density - 1.0) * 100.0;
    if uplift > 10.0 {
        println!("The midpoint window is meaningfully denser in this scan (+{:.1}%).", uplift);
    } else if uplift > -10.0 {
        println!(
            "The midpoint window is only modestly different from endpoint windows (+{:.1}%).",
            uplift
        );
        println!("This script does not establish a strong clustering effect by itself.");
    } else {
        println!(
            "The midpoint window is not denser than endpoint windows in this scan ({:.1}%).",
            uplift
        );
    }
    // Let's also test the "pressure" or "field strength" metaphor
    println!("\n🌊 Understanding the 'Field' Phenomenon");
    println!("======================================");
    // Calculate "field strength" as inverse of distance to nearest membrane prime
    println!("\nField strength at various points:");
    let l1_minus_1000 = &l1 - 1000u32;
    let l1_minus_500 = &l1 - 500u32;
    let l1_plus_500 = &l1 + 500u32;
    let l1_plus_1000 = &l1 + 1000u32;
    let test_points = vec![
        (&prime1, "Membrane 1"),
        (&l1_minus_1000, "L1 - 1000"),
        (&l1_minus_500, "L1 - 500"),
        (&l1, "L1 center"),
        (&l1_plus_500, "L1 + 500"),
        (&l1_plus_1000, "L1 + 1000"),
        (&prime2, "Membrane 2"),
    ];
    for (point, label) in test_points {
        let dist1 = if point >= &prime1 {
            point - &prime1
        } else {
            &prime1 - point
        };
        let dist2 = if point >= &prime2 {
            point - &prime2
        } else {
            &prime2 - point
        };

        // Field strength as inverse of minimum distance
        let min_dist = std::cmp::min(dist1, dist2);
        let field_strength = if min_dist == BigUint::from(0u32) {
            0.0 // At membrane itself - "desert"
        } else {
            1000.0 / min_dist.to_string().parse::<f64>().unwrap_or(1.0)
        };
        println!("  {}: field strength = {:.3}", label, field_strength);
    }

    println!("\n🎯 Better Metaphor Than 'Gravity':");
    println!("==================================");
    println!("Instead of 'gravitational fields', think of:");
    println!("1. PRIME DENSITY GRADIENTS - probability of finding primes");
    println!("2. STRUCTURAL INTERFERENCE - membrane patterns create voids");
    println!("3. EQUILIBRIUM ZONES - candidate midpoints worth testing");
    println!("4. CONTROLLED FOLLOW-UP - compare equal windows before telling a bigger story");
}
