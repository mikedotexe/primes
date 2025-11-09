use num_bigint::BigUint;
use prime_physics_engine::is_prime;

fn test_space_sizes(body1: &str, body2: &str) {
    println!("\n🔬 Testing: {} ↔ {}", body1, body2);
    println!("{}", "-".repeat(60));

    let p1 = body1.parse::<BigUint>().unwrap();
    let p2 = body2.parse::<BigUint>().unwrap();
    println!(
        "Body 1 prime: {}, Body 2 prime: {}",
        is_prime(&p1),
        is_prime(&p2)
    );

    println!(
        "\n{:<10} {:<15} {:<15} {:<15}",
        "Space", "L-Points", "Tests", "Success%"
    );
    println!("{}", "-".repeat(60));

    for space_size in 1..=20 {
        let zeros = "0".repeat(space_size);
        let mut successes = 0;
        let tests = space_size * 9; // 9 digits per position

        for position in 0..space_size {
            for digit in 1..=9 {
                let mut test_str = zeros.clone();
                let bytes = unsafe { test_str.as_bytes_mut() };
                bytes[position] = b'0' + digit as u8;

                let full_number = format!("{}{}{}", body1, test_str, body2);
                let num = full_number.parse::<BigUint>().unwrap();

                if is_prime(&num) {
                    successes += 1;
                }
            }
        }

        let success_rate = (successes as f64 / tests as f64) * 100.0;
        println!(
            "{:<10} {:<15} {:<15} {:<14.1}",
            space_size, successes, tests, success_rate
        );

        // Show specific primes for interesting cases
        if space_size <= 3 && successes > 0 {
            print!("   Examples: ");
            let mut shown = 0;
            for position in 0..space_size {
                for digit in 1..=9 {
                    if shown >= 2 {
                        break;
                    }
                    let mut test_str = zeros.clone();
                    let bytes = unsafe { test_str.as_bytes_mut() };
                    bytes[position] = b'0' + digit as u8;

                    let full_number = format!("{}{}{}", body1, test_str, body2);
                    let num = full_number.parse::<BigUint>().unwrap();

                    if is_prime(&num) {
                        print!("{} ", full_number);
                        shown += 1;
                    }
                }
            }
            println!();
        }
    }
}

fn main() {
    println!("🌌 LAGRANGE SPACE SIZE ANALYSIS");
    println!("{}", "=".repeat(80));
    println!("\nHow does the space between bodies affect Lagrange point density?");

    // Test different configurations
    test_space_sizes("11", "13");
    test_space_sizes("7", "11");
    test_space_sizes("101", "103");
    test_space_sizes("10301", "10301");
    test_space_sizes("11", "121");

    println!("\n📊 OBSERVATIONS:");
    println!("   1. Success rate generally decreases as space increases");
    println!("   2. Simple numbers maintain higher rates even with larger spaces");
    println!("   3. Some configurations have 'sweet spots' at certain distances");
}
