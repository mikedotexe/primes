use num_bigint::BigUint;
use prime_physics_engine::is_prime;

fn test_lagrange_verbose(prime1: &str, prime2: &str, zero_count: usize) {
    println!("\n{}", "=".repeat(80));
    println!("🌌 LAGRANGE POINT ANALYSIS");
    println!("{}", "=".repeat(80));

    let p1 = prime1.parse::<BigUint>().unwrap();
    let p2 = prime2.parse::<BigUint>().unwrap();

    println!("\n📍 BODY 1: {}", prime1);
    println!(
        "   Status: {}",
        if is_prime(&p1) {
            "✅ VERIFIED PRIME"
        } else {
            "❌ NOT PRIME"
        }
    );

    println!("\n📍 BODY 2: {}", prime2);
    println!(
        "   Status: {}",
        if is_prime(&p2) {
            "✅ VERIFIED PRIME"
        } else {
            "❌ NOT PRIME"
        }
    );

    println!("\n🚀 SPACE BETWEEN BODIES: {} zeros", zero_count);

    // Baseline test
    let zeros = "0".repeat(zero_count);
    let baseline = format!("{}{}{}", prime1, zeros, prime2);
    let baseline_num = baseline.parse::<BigUint>().unwrap();

    println!("\n🔬 BASELINE (just zeros):");
    println!("   Full number: {}", baseline);
    println!("   Length: {} digits", baseline.len());
    println!(
        "   Primality: {}",
        if is_prime(&baseline_num) {
            "✅ PRIME!"
        } else {
            "❌ Not prime"
        }
    );

    // Test all positions and digits
    println!("\n🎯 SEARCHING FOR LAGRANGE POINTS...");
    println!("{}", "=".repeat(80));

    let mut total_found = 0;

    for position in 0..zero_count {
        let mut position_primes = Vec::new();

        for digit in 1..=9 {
            let mut test_str = zeros.clone();
            let bytes = unsafe { test_str.as_bytes_mut() };
            bytes[position] = b'0' + digit as u8;

            let full_number = format!("{}{}{}", prime1, test_str, prime2);
            let num = full_number.parse::<BigUint>().unwrap();

            if is_prime(&num) {
                position_primes.push((digit, full_number.clone()));
            }
        }

        if !position_primes.is_empty() {
            println!("\n⚡ LAGRANGE POINT AT POSITION {} ⚡", position);
            for (digit, prime_str) in position_primes {
                total_found += 1;
                println!("\n   🌟 Digit {}: CREATES PRIME!", digit);
                println!("   Full number: {}", prime_str);
                println!("   Length: {} digits", prime_str.len());

                // Show the structure visually
                let visual = format!(
                    "{} {} {} {} {}",
                    prime1,
                    &zeros[..position],
                    digit,
                    &zeros[position + 1..],
                    prime2
                );
                println!("   Structure: {}", visual);
            }
        }
    }

    println!("\n{}", "=".repeat(80));
    println!("📊 TOTAL LAGRANGE POINTS FOUND: {}", total_found);
    println!("{}", "=".repeat(80));
}

fn find_membrane_primes() {
    println!("\n🔍 FINDING ACTUAL MEMBRANE PRIMES FOR TESTING...");

    // Test some membrane structures to find primes
    let patterns = [
        ("1", "3", "5"),
        ("3", "7", "1"),
        ("7", "1", "3"),
        ("1", "5", "7"),
    ];

    let mut found = Vec::new();

    for (outer, inner, middle) in patterns {
        for zeros1 in 0..=2 {
            for zeros2 in 0..=2 {
                let z1 = "0".repeat(zeros1);
                let z2 = "0".repeat(zeros2);

                // Build double membrane
                let membrane = format!(
                    "{}{}{}{}{}{}{}{}{}",
                    outer, z1, inner, z2, middle, z2, inner, z1, outer
                );

                let num = membrane.parse::<BigUint>().unwrap();
                if is_prime(&num) && membrane.len() > 5 {
                    found.push((membrane.clone(), outer, inner, middle, zeros1, zeros2));
                    println!("\n✅ Found membrane prime: {}", membrane);
                    println!(
                        "   Structure: {}-{}-{}-{}-{}-{}-{}-{}-{}",
                        outer,
                        if zeros1 > 0 { &z1 } else { "" },
                        inner,
                        if zeros2 > 0 { &z2 } else { "" },
                        middle,
                        if zeros2 > 0 { &z2 } else { "" },
                        inner,
                        if zeros1 > 0 { &z1 } else { "" },
                        outer
                    );
                }
            }
        }
    }

    if found.len() >= 2 {
        println!("\n🌌 TESTING LAGRANGE POINTS BETWEEN TWO MEMBRANE PRIMES:");
        test_lagrange_verbose(&found[0].0, &found[1].0, 5);
    }
}

fn main() {
    println!("{}", "=".repeat(80));
    println!("🚀 LAGRANGE POINT SUPER VERBOSE VERIFICATION");
    println!("{}", "=".repeat(80));

    // Test original claim
    println!("\n1️⃣ ORIGINAL DOCUMENTATION CLAIM:");
    test_lagrange_verbose("10301", "30305070305070303", 5);

    // Test with verified primes
    println!("\n\n2️⃣ TWO VERIFIED PRIMES:");
    test_lagrange_verbose("101", "30103", 4);

    // Test with simple primes and more space
    println!("\n\n3️⃣ SIMPLE PRIMES WITH MORE SPACE:");
    test_lagrange_verbose("11", "13", 10);

    // Find and test actual membrane primes
    println!("\n\n4️⃣ FINDING MEMBRANE PRIMES:");
    find_membrane_primes();

    // Test a known membrane prime with itself
    println!("\n\n5️⃣ MEMBRANE PRIME WITH ITSELF:");
    test_lagrange_verbose("30703", "30703", 7);

    println!("\n{}", "=".repeat(80));
    println!("✨ VERIFICATION COMPLETE ✨");
    println!("{}", "=".repeat(80));
}
