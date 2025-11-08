use num_bigint::BigUint;
use prime_physics_engine::is_prime;

fn test_lagrange_insertions(prime1: &str, prime2: &str, zero_count: usize) {
    println!("\n🔬 Testing Lagrange points between:");
    println!("   Prime 1: {}", prime1);
    println!("   Prime 2: {}", prime2);
    println!("   Zero buffer size: {}", zero_count);
    
    // First verify the base primes
    let p1 = prime1.parse::<BigUint>().unwrap();
    let p2 = prime2.parse::<BigUint>().unwrap();
    
    println!("\n📊 Base number verification:");
    println!("   {} is prime: {}", prime1, is_prime(&p1));
    println!("   {} is prime: {}", prime2, is_prime(&p2));
    
    // Test with just zeros (baseline)
    let zeros = "0".repeat(zero_count);
    let baseline = format!("{}{}{}", prime1, zeros, prime2);
    let baseline_num = baseline.parse::<BigUint>().unwrap();
    println!("\n   With {} zeros: {} → {}", 
        zero_count,
        if baseline.len() > 50 { format!("{}...{}", &baseline[..25], &baseline[baseline.len()-25..]) } else { baseline.clone() },
        if is_prime(&baseline_num) { "✅ PRIME!" } else { "❌ Not prime" }
    );
    
    // Test inserting different digits at different positions
    println!("\n🎯 Testing digit insertions:");
    let mut found_primes = Vec::new();
    
    for position in 0..zero_count {
        for digit in 1..=9 {
            let mut test_str = zeros.clone();
            let bytes = unsafe { test_str.as_bytes_mut() };
            bytes[position] = b'0' + digit as u8;
            
            let full_number = format!("{}{}{}", prime1, test_str, prime2);
            let num = full_number.parse::<BigUint>().unwrap();
            
            if is_prime(&num) {
                found_primes.push((position, digit, full_number.len()));
                println!("   ✅ Position {}, digit {} → {}-digit PRIME!", position, digit, full_number.len());
            }
        }
    }
    
    if found_primes.is_empty() {
        println!("   ❌ No prime-creating Lagrange points found!");
    } else {
        println!("\n📈 Summary: Found {} Lagrange points!", found_primes.len());
        for (pos, digit, len) in &found_primes {
            println!("   • Position {}, digit {} creates {}-digit prime", pos, digit, len);
        }
    }
}

fn main() {
    println!("🌌 Lagrange Point Verification Tool");
    println!("=====================================");
    
    // Test the specific claim from CLAUDE.md
    println!("\n1️⃣ Testing documented claim:");
    test_lagrange_insertions("10301", "30305070305070303", 5);
    
    // Let's try some other combinations
    println!("\n\n2️⃣ Testing with verified membrane primes:");
    test_lagrange_insertions("101", "30103", 3);
    
    println!("\n\n3️⃣ Testing with simple primes:");
    test_lagrange_insertions("13", "17", 5);
    
    println!("\n\n4️⃣ Testing larger gap:");
    test_lagrange_insertions("10301", "30305070305070303", 10);
}