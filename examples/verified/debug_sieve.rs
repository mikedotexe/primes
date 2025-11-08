use prime_physics_engine::prime_sieve::BitSieve;

fn main() {
    // Test different sizes to find where the issue starts
    for limit in [1000, 10_000, 100_000, 1_000_000] {
        let sieve = BitSieve::new(limit);
        let primes = sieve.primes();
        println!("Limit: {}, Found: {} primes", limit, primes.len());
        
        // Known values
        match limit {
            1000 => println!("  Expected: 168"),
            10_000 => println!("  Expected: 1229"),
            100_000 => println!("  Expected: 9592"),
            1_000_000 => println!("  Expected: 78498"),
            _ => {}
        }
    }
    
    // Check around segment boundaries
    println!("\nChecking segment boundaries:");
    let seg_size = 32 * 1024 * 8 * 2; // SEG_BITS * 2 (for odds)
    for i in 0..3 {
        let limit = seg_size * (i + 1);
        let sieve = BitSieve::new(limit);
        let count = sieve.primes().len();
        println!("Segment boundary {}: limit={}, count={}", i+1, limit, count);
    }
}