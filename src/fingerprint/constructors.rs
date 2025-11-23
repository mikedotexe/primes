//! Prime Construction Methods
//!
//! Defines various methods for constructing prime candidates:
//! - Membrane configurations
//! - Belphegor-style palindromes
//! - Connectors between prime pairs
//! - Random baseline

use num_bigint::BigUint;
use rand::{Rng, SeedableRng};
use rand::seq::SliceRandom;

/// Trait for prime construction methods
pub trait PrimeConstructor: Send + Sync {
    /// Generate a candidate number from a seed
    fn generate(&self, seed: u64) -> BigUint;

    /// Human-readable name
    fn name(&self) -> String;

    /// Generate many candidates
    fn generate_many(&self, count: usize) -> Vec<BigUint> {
        (0..count as u64)
            .map(|seed| self.generate(seed))
            .collect()
    }

    /// Generate many primes (filter candidates through primality test)
    fn generate_primes(&self, count: usize, max_candidates: usize) -> Vec<BigUint> {
        let mut primes = Vec::new();
        let mut seed = 0u64;

        while primes.len() < count && (seed as usize) < max_candidates {
            let candidate = self.generate(seed);
            if is_probably_prime(&candidate, 10) {
                primes.push(candidate);
            }
            seed += 1;
        }

        primes
    }
}

/// Membrane prime constructor
#[derive(Debug, Clone)]
pub struct MembraneConstructor {
    pub base: u32,
    pub outer: u8,
    pub inner: u8,
    pub k_outer: usize,
    pub k_inner: usize,
}

impl MembraneConstructor {
    pub fn new(base: u32, outer: u8, inner: u8, k_outer: usize, k_inner: usize) -> Self {
        MembraneConstructor {
            base,
            outer,
            inner,
            k_outer,
            k_inner,
        }
    }

    /// Build membrane number from seed
    fn construct(&self, seed: u64) -> BigUint {
        // Structure: outer - 0^k_outer - inner - 0^k_inner - SEED - 0^k_inner - inner - 0^k_outer - outer
        let seed_str = format_in_base(seed, self.base);

        let mut s = String::new();
        s.push_str(&format!("{}", self.outer));
        s.push_str(&"0".repeat(self.k_outer));
        s.push_str(&format!("{}", self.inner));
        s.push_str(&"0".repeat(self.k_inner));
        s.push_str(&seed_str);
        s.push_str(&"0".repeat(self.k_inner));
        s.push_str(&format!("{}", self.inner));
        s.push_str(&"0".repeat(self.k_outer));
        s.push_str(&format!("{}", self.outer));

        // Convert from base representation to decimal
        from_base_string(&s, self.base)
    }
}

impl PrimeConstructor for MembraneConstructor {
    fn generate(&self, seed: u64) -> BigUint {
        self.construct(seed)
    }

    fn name(&self) -> String {
        format!(
            "membrane_b{}_({}_{})_k({}_{})",
            self.base, self.outer, self.inner, self.k_outer, self.k_inner
        )
    }
}

/// Belphegor-style palindrome constructor
#[derive(Debug, Clone)]
pub struct BelphegorConstructor {
    pub outer: u8,
    pub padding: usize,
}

impl BelphegorConstructor {
    pub fn new(outer: u8, padding: usize) -> Self {
        BelphegorConstructor { outer, padding }
    }

    fn construct(&self, seed: u64) -> BigUint {
        // Structure: outer - 0^padding - seed - 0^padding - outer
        let seed_digits = seed.to_string().len();
        let base = BigUint::from(10u32);

        // Seed position: padding + 1 (for right outer)
        let seed_position = self.padding + 1;
        // Left outer position: seed_position + seed_digits + padding
        let left_position = seed_position + seed_digits + self.padding;

        let left_outer = BigUint::from(self.outer) * base.pow(left_position as u32);
        let middle_seed = BigUint::from(seed) * base.pow(seed_position as u32);
        let right_outer = BigUint::from(self.outer);

        left_outer + middle_seed + right_outer
    }
}

impl PrimeConstructor for BelphegorConstructor {
    fn generate(&self, seed: u64) -> BigUint {
        self.construct(seed)
    }

    fn name(&self) -> String {
        format!("belphegor_outer{}_pad{}", self.outer, self.padding)
    }
}

/// Connector between two fixed primes
#[derive(Debug, Clone)]
pub struct ConnectorConstructor {
    pub prime1: BigUint,
    pub prime2: BigUint,
    pub connector_length: usize,
}

impl ConnectorConstructor {
    pub fn new(prime1: BigUint, prime2: BigUint, connector_length: usize) -> Self {
        ConnectorConstructor {
            prime1,
            prime2,
            connector_length,
        }
    }

    fn construct(&self, seed: u64) -> BigUint {
        // Zero-pad seed to connector_length
        let connector = format!("{:0width$}", seed, width = self.connector_length);

        // Concatenate
        let full_str = format!("{}{}{}", self.prime1, connector, self.prime2);
        BigUint::parse_bytes(full_str.as_bytes(), 10).unwrap()
    }
}

impl PrimeConstructor for ConnectorConstructor {
    fn generate(&self, seed: u64) -> BigUint {
        self.construct(seed)
    }

    fn name(&self) -> String {
        format!(
            "connector_{}_{}_len{}",
            self.prime1, self.prime2, self.connector_length
        )
    }
}

/// Zero-heavy connector restricted to {0,3,6} digits
/// Demonstrates rare structural patterns in connector space
#[derive(Debug, Clone)]
pub struct ZeroHeavyConnectorConstructor {
    pub prime1: BigUint,
    pub prime2: BigUint,
    pub connector_length: usize,
}

impl ZeroHeavyConnectorConstructor {
    pub fn new(prime1: BigUint, prime2: BigUint, connector_length: usize) -> Self {
        ZeroHeavyConnectorConstructor {
            prime1,
            prime2,
            connector_length,
        }
    }

    fn construct(&self, seed: u64) -> BigUint {
        // Generate zero-heavy pattern using only {0, 3, 6}
        // Strategy: Start with zeros, place a few {3,6} digits at random positions
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        let mut connector = vec![0u8; self.connector_length];

        // Determine how many nonzero digits (biased toward sparse patterns)
        let num_nonzero = if self.connector_length <= 5 {
            1 // For short connectors, use minimal patterns
        } else if self.connector_length <= 7 {
            rng.gen_range(1..=3) // 1-3 nonzero digits
        } else {
            rng.gen_range(1..=4) // 1-4 nonzero digits
        };

        // Place nonzero digits at random positions
        let mut positions: Vec<usize> = (0..self.connector_length).collect();
        positions.shuffle(&mut rng);

        for i in 0..num_nonzero {
            let pos = positions[i];
            // Choose from {3, 6} with equal probability
            connector[pos] = if rng.gen_bool(0.5) { 3 } else { 6 };
        }

        // Convert to string
        let connector_str: String = connector.iter()
            .map(|&d| char::from_digit(d as u32, 10).unwrap())
            .collect();

        // Concatenate
        let full_str = format!("{}{}{}", self.prime1, connector_str, self.prime2);
        BigUint::parse_bytes(full_str.as_bytes(), 10).unwrap()
    }
}

impl PrimeConstructor for ZeroHeavyConnectorConstructor {
    fn generate(&self, seed: u64) -> BigUint {
        self.construct(seed)
    }

    fn name(&self) -> String {
        format!(
            "connector_zeroheavy_{}_{}_len{}",
            self.prime1, self.prime2, self.connector_length
        )
    }
}

/// Random baseline (uniform random large integers)
#[derive(Debug, Clone)]
pub struct RandomConstructor {
    pub digit_count: usize,
}

impl RandomConstructor {
    pub fn new(digit_count: usize) -> Self {
        RandomConstructor { digit_count }
    }
}

impl PrimeConstructor for RandomConstructor {
    fn generate(&self, seed: u64) -> BigUint {
        // Use seed to initialize RNG for reproducibility
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        // Generate random digits
        let mut s = String::new();
        // First digit: 1-9
        s.push_str(&format!("{}", rng.gen_range(1..=9)));
        // Rest: 0-9
        for _ in 1..self.digit_count {
            s.push_str(&format!("{}", rng.gen_range(0..=9)));
        }

        BigUint::parse_bytes(s.as_bytes(), 10).unwrap()
    }

    fn name(&self) -> String {
        format!("random_{}_digits", self.digit_count)
    }
}

// ========================================
// Helper Functions
// ========================================

/// Format number in given base
fn format_in_base(mut n: u64, base: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let mut digits = Vec::new();
    while n > 0 {
        digits.push((n % base as u64) as u8);
        n /= base as u64;
    }
    digits.reverse();

    digits.iter().map(|&d| format!("{}", d)).collect()
}

/// Parse base-b string to BigUint
fn from_base_string(s: &str, base: u32) -> BigUint {
    let mut result = BigUint::from(0u32);
    let base_big = BigUint::from(base);

    for ch in s.chars() {
        let digit = ch.to_digit(10).unwrap_or(0);
        result = result * &base_big + BigUint::from(digit);
    }

    result
}

/// Miller-Rabin primality test (simplified, reused from other modules)
fn is_probably_prime(n: &BigUint, rounds: u32) -> bool {
    use num_traits::{One, Zero};

    let two = BigUint::from(2u32);
    let three = BigUint::from(3u32);

    if *n < two {
        return false;
    }
    if *n == two || *n == three {
        return true;
    }
    if n % 2u32 == BigUint::zero() {
        return false;
    }

    let one = BigUint::one();
    let n_minus_one = n - &one;

    // Write n - 1 = d * 2^s with d odd
    let mut d = n_minus_one.clone();
    let mut s: u32 = 0;
    while &d % 2u32 == BigUint::zero() {
        d /= 2u32;
        s += 1;
    }

    // Deterministic bases
    let bases: [u32; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    let mut used_rounds = 0u32;
    for &a_u32 in bases.iter() {
        if used_rounds >= rounds {
            break;
        }
        used_rounds += 1;

        if BigUint::from(a_u32) >= n_minus_one {
            continue;
        }

        let mut x = BigUint::from(a_u32).modpow(&d, n);
        if x == one || x == n_minus_one {
            continue;
        }

        let mut composite = true;
        for _ in 1..s {
            x = x.modpow(&two, n);
            if x == n_minus_one {
                composite = false;
                break;
            }
        }

        if composite {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_membrane_constructor() {
        let mem = MembraneConstructor::new(6, 1, 5, 0, 0);
        let candidate = mem.generate(0);
        assert!(candidate > BigUint::from(0u32));
    }

    #[test]
    fn test_belphegor_constructor() {
        let belph = BelphegorConstructor::new(1, 13);
        let candidate = belph.generate(666);
        // Should produce Belphegor's prime
        let expected_str = "1000000000000066600000000000001";
        let expected = BigUint::parse_bytes(expected_str.as_bytes(), 10).unwrap();
        assert_eq!(candidate, expected);
    }

    #[test]
    fn test_random_constructor() {
        let rand_const = RandomConstructor::new(10);
        let n1 = rand_const.generate(42);
        let n2 = rand_const.generate(42);
        assert_eq!(n1, n2); // Same seed = same result
        assert_eq!(n1.to_string().len(), 10);
    }
}
