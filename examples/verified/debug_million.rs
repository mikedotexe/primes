use prime_physics_engine::prime_sieve::BitSieve;

fn count_primes_simple(limit: usize) -> usize {
    // Simple sieve for comparison
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in ((i * i)..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    
    is_prime.iter().filter(|&&x| x).count()
}

fn main() {
    let limit = 1_000_000;
    
    // Our sieve
    let sieve = BitSieve::new(limit);
    let our_primes = sieve.primes();
    let our_count = our_primes.len();
    
    // Simple reference
    let simple_count = count_primes_simple(limit);
    
    println!("BitSieve count: {}", our_count);
    println!("Simple sieve count: {}", simple_count);
    println!("Difference: {}", our_count as i32 - simple_count as i32);
    
    // Check some of the larger primes
    println!("\nLast 10 primes from BitSieve:");
    for &p in our_primes.iter().rev().take(10) {
        println!("  {}", p);
    }
    
    // Verify they're actually prime
    println!("\nChecking if these are actually prime:");
    for &p in our_primes.iter().rev().take(10) {
        let is_prime = (2..((p as f64).sqrt() as usize + 1))
            .all(|d| p % d != 0);
        println!("  {} is prime: {}", p, is_prime);
    }
}