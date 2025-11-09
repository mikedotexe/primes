//! Prime lookup table with compile-time generation and unrolled loops

use once_cell::sync::Lazy;

/// Pre-computed small primes for fast filtering
pub static SMALL_PRIMES: Lazy<Vec<u32>> = Lazy::new(|| {
    vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
        509, 521, 523, 541,
    ]
});

/// Signature row for affine sieve
#[repr(C, align(16))] // 16-byte aligned as recommended
pub struct SigRow {
    pub s: u32,   // signature
    pub g: u32,   // generator
    pub p: u32,   // prime
    pub pad: u32, // padding for alignment
}

/// Pre-computed signatures for small primes
pub static SIGNATURES: Lazy<Vec<SigRow>> = Lazy::new(|| {
    SMALL_PRIMES
        .iter()
        .map(|&p| {
            // Simple signature generation (can be optimized based on base)
            SigRow {
                s: 1,
                g: primitive_root(p),
                p,
                pad: 0,
            }
        })
        .collect()
});

/// Find a primitive root modulo p (simplified version)
fn primitive_root(p: u32) -> u32 {
    if p == 2 {
        return 1;
    }

    // For small primes, use known primitive roots
    match p {
        3 => 2,
        5 => 2,
        7 => 3,
        11 => 2,
        13 => 2,
        17 => 3,
        19 => 2,
        23 => 5,
        29 => 2,
        31 => 3,
        _ => {
            // Simple search for primitive root
            for g in 2..p {
                if is_primitive_root(g, p) {
                    return g;
                }
            }
            2 // fallback
        }
    }
}

/// Check if g is a primitive root mod p
fn is_primitive_root(g: u32, p: u32) -> bool {
    let mut seen = vec![false; p as usize];
    let mut power = 1u64;

    for _ in 0..p - 1 {
        power = (power * g as u64) % p as u64;
        if seen[power as usize] {
            return false;
        }
        seen[power as usize] = true;
    }

    true
}

/// Fast modular exponentiation
#[inline(always)]
pub fn mod_pow(base: u32, exp: u32, modulus: u32) -> u32 {
    let mut result = 1u64;
    let mut base = base as u64;
    let mut exp = exp;
    let modulus = modulus as u64;

    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp >>= 1;
    }

    result as u32
}

/// Unrolled prime check using chunks of 4
pub fn quick_composite_check_unrolled(value: u64) -> bool {
    let sigs = &SIGNATURES[..];

    // Process in chunks of 4 for ILP
    for chunk in sigs.chunks_exact(4) {
        let p0 = chunk[0].p as u64;
        let p1 = chunk[1].p as u64;
        let p2 = chunk[2].p as u64;
        let p3 = chunk[3].p as u64;

        // Check divisibility by all 4 primes at once
        if value.is_multiple_of(p0) && value != p0 {
            return true;
        }
        if value.is_multiple_of(p1) && value != p1 {
            return true;
        }
        if value.is_multiple_of(p2) && value != p2 {
            return true;
        }
        if value.is_multiple_of(p3) && value != p3 {
            return true;
        }
    }

    // Handle remainder
    for sig in sigs.chunks_exact(4).remainder() {
        let p = sig.p as u64;
        if value.is_multiple_of(p) && value != p {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_primes() {
        assert!(SMALL_PRIMES.len() >= 100);
        assert_eq!(SMALL_PRIMES[0], 2);
        assert_eq!(SMALL_PRIMES[1], 3);
    }

    #[test]
    fn test_signatures() {
        assert_eq!(SIGNATURES.len(), SMALL_PRIMES.len());
        for (i, sig) in SIGNATURES.iter().enumerate() {
            assert_eq!(sig.p, SMALL_PRIMES[i]);
        }
    }

    #[test]
    fn test_composite_check() {
        assert!(quick_composite_check_unrolled(15)); // 3 * 5
        assert!(quick_composite_check_unrolled(21)); // 3 * 7
        assert!(!quick_composite_check_unrolled(17)); // prime
        assert!(!quick_composite_check_unrolled(2)); // prime
    }
}
