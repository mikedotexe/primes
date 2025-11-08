use num_bigint::BigUint;
use prime_physics_engine::is_prime;

fn main() {
    println!("🔍 Verifying updated documentation claim...\n");
    
    let prime1 = "10301";
    let prime2 = "3007003007003";
    
    // Check both are prime
    println!("Prime 1: {} → {}", prime1, 
        if is_prime(&prime1.parse::<BigUint>().unwrap()) { "✅ PRIME" } else { "❌ NOT PRIME" });
    println!("Prime 2: {} → {}", prime2,
        if is_prime(&prime2.parse::<BigUint>().unwrap()) { "✅ PRIME" } else { "❌ NOT PRIME" });
    
    // Test with 5 zeros
    let with_zeros = "10301000003007003007003";
    println!("\nWith 5 zeros: {}", with_zeros);
    println!("Is prime: {}", 
        if is_prime(&with_zeros.parse::<BigUint>().unwrap()) { "✅ YES" } else { "❌ NO" });
    
    // Test position 1, digit 2
    let with_digit = "10301020003007003007003";
    println!("\nWith digit 2 at position 1: {}", with_digit);
    println!("Is prime: {}", 
        if is_prime(&with_digit.parse::<BigUint>().unwrap()) { "✅ YES!" } else { "❌ NO" });
    
    // Let's find all Lagrange points for this pair
    println!("\n🎯 Finding ALL Lagrange points for this pair...\n");
    
    for position in 0..5 {
        for digit in 1..=9 {
            let mut zeros = "00000".to_string();
            let bytes = unsafe { zeros.as_bytes_mut() };
            bytes[position] = b'0' + digit as u8;
            
            let test = format!("{}{}{}", prime1, zeros, prime2);
            if is_prime(&test.parse::<BigUint>().unwrap()) {
                println!("✅ Position {}, digit {} → PRIME!", position, digit);
            }
        }
    }
}