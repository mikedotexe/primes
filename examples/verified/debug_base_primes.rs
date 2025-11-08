use prime_physics_engine::prime_sieve::BitSieve;

fn main() {
    let limit = 1_000_000;
    let root = (limit as f64).sqrt() as usize + 1;
    println!("For limit {}, sqrt is approximately {}", limit, root);
    
    // Check what's happening with base prime generation
    let mut is_comp = vec![false; root + 1];
    let mut base = Vec::new();

    for p in 3..=root {
        if p & 1 == 0 || is_comp[p] { continue }
        base.push(p);
        if p * p <= root {
            for m in (p * p..=root).step_by(p << 1) {
                is_comp[m] = true
            }
        }
    }
    
    println!("\nBase primes computed: {}", base.len());
    println!("First 20 base primes: {:?}", &base[..20.min(base.len())]);
    println!("Last 5 base primes: {:?}", &base[base.len().saturating_sub(5)..]);
    
    // Check if 3 is being used to mark 999993
    let seg_lo = 524289; // Start of segment containing 999993
    let p = 3;
    let start = usize::max(p * p, ((seg_lo + p - 1) / p) * p);
    println!("\nFor p=3, seg_lo={}, start={}", seg_lo, start);
    println!("999993 % 3 = {}", 999993 % 3);
    println!("Should 999993 be marked? {}", 999993 % 3 == 0);
    
    // Check the segment containing 999993
    let sieve = BitSieve::new(limit);
    let primes = sieve.primes();
    let has_999993 = primes.contains(&999993);
    println!("\nDoes BitSieve think 999993 is prime? {}", has_999993);
}