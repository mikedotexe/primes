use crate::prime_sieve::BitSieve;

#[test]
fn debug_sieve_count() {
    // Check small counts
    let sieve = BitSieve::new(10);
    let primes = sieve.primes();
    println!("Primes up to 10: {:?}", primes);
    assert_eq!(primes, vec![2, 3, 5, 7]);
    
    let sieve = BitSieve::new(20);
    let primes = sieve.primes();
    println!("Primes up to 20: {:?}", primes);
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
    
    // Count up to 1000
    let sieve = BitSieve::new(1000);
    let count = sieve.primes().len();
    println!("Primes up to 1000: {}", count);
    assert_eq!(count, 168); // π(1000) = 168
}