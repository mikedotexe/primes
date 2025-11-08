//! Prime lookup table with reciprocals for fast modulo

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct SigRowRecip {
    pub s: u32,  // signature
    pub g: u32,  // generator
    pub p: u32,  // prime
    pub q: u32,  // reciprocal for fast modulo
}

/// Generate signature table with reciprocals
pub fn generate_signatures_with_reciprocals(base: u32, l: u32, r: u32, width: u32) -> Vec<SigRowRecip> {
    let primes = SMALL_PRIMES_100;
    
    primes.iter().map(|&p| {
        // Compute signature components
        let b = base as u64;
        let s = (l as u64 * b.pow(width - 1) + 
                r as u64 * b.pow(width - 2) +
                r as u64 * b +
                l as u64) % p as u64;
        let g = b % p as u64;
        
        // Compute reciprocal: ceil(2^32 / p)
        let q = ((u64::MAX / p as u64) + 1) as u32;
        
        SigRowRecip {
            s: s as u32,
            g: g as u32,
            p,
            q,
        }
    }).collect()
}

// First 100 odd primes for sieving
pub const SMALL_PRIMES_100: [u32; 100] = [
    3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
    79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157,
    163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239,
    241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331,
    337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509,
    521, 523, 541, 547
];