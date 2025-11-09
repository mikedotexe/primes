use num_bigint::BigUint;
use prime_physics_engine::is_prime;

fn main() {
    println!("--- is_prime Sanity Check ---");

    let tests = vec![
        (17u64, true),          // Small prime
        (18u64, false),         // Small composite
        (4294967291u64, true),  // Moderately large prime (Fermat number F_4(1))
        (4294967290u64, false), // Composite
    ];

    let mut all_passed = true;

    for (num, expected) in tests {
        let bignum = BigUint::from(num as u64);
        let result = is_prime(&bignum);
        let status = if result == expected { "Ok" } else { "FAIL" };
        if result != expected {
            all_passed = false;
        }
        println!(
            "Test: is_prime({}), Expected: {}, Got: {}, Status: {}",
            num, expected, result, status
        );
    }

    println!("\n--- Result ---");
    if all_passed {
        println!("SUCCESS: The is_prime function behaves as expected.");
    } else {
        println!("FAILURE: The is_prime function returned an unexpected result.");
    }
}
