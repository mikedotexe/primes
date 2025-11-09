use num_bigint::BigUint;
use prime_physics_engine::is_prime;

fn check_membrane_structure(s: &str) -> bool {
    // Check if it has membrane-like structure with zeros
    s.contains('0') && s.chars().filter(|&c| c == '0').count() >= 4
}

fn main() {
    println!("🔍 Checking claimed membrane prime...\n");

    let claimed = "30305070305070303";
    let num = claimed.parse::<BigUint>().unwrap();

    println!("Number: {}", claimed);
    println!("Length: {} digits", claimed.len());
    println!(
        "Has membrane structure: {}",
        check_membrane_structure(claimed)
    );
    println!(
        "Is prime: {}\n",
        if is_prime(&num) { "✅ YES" } else { "❌ NO" }
    );

    if !is_prime(&num) {
        println!("🔍 Finding similar ACTUAL membrane primes...\n");

        // Try variations
        let variations = [
            "30305070305070307", // Changed last digit
            "30305070305070309",
            "30305070305070311",
            "30307070307070303", // Changed 5s to 7s
            "30301070301070303", // Changed 5s to 1s
            "3030507030507031",  // Removed last digit
            "303050703050703",   // Removed last 2 digits
        ];

        for var in &variations {
            let n = var.parse::<BigUint>().unwrap();
            if is_prime(&n) {
                println!("✅ FOUND PRIME: {}", var);
                println!("   Structure matches original pattern!");
            }
        }

        // Also try some simpler membrane patterns
        println!("\n🔍 Testing simpler membrane patterns...\n");

        let patterns = [
            ("3", "7", "1", 2, 2), // 30070100107003
            ("3", "7", "3", 2, 2), // 30070300307003
            ("7", "3", "1", 2, 2), // 70030100103007
        ];

        for (outer, inner, middle, z1, z2) in patterns {
            let zeros1 = "0".repeat(z1);
            let zeros2 = "0".repeat(z2);
            let membrane = format!(
                "{}{}{}{}{}{}{}{}{}",
                outer, zeros1, inner, zeros2, middle, zeros2, inner, zeros1, outer
            );

            let n = membrane.parse::<BigUint>().unwrap();
            if is_prime(&n) && membrane.len() > 10 {
                println!("✅ Membrane prime: {}", membrane);
                println!(
                    "   Pattern: {}-{}-{} with ({},{}) zeros",
                    outer, inner, middle, z1, z2
                );
            }
        }
    }
}
