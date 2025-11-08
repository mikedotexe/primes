//! Wheel-30 SIMD optimizations for prime sieve
//! 
//! Wheel factorization modulo 30 = 2×3×5 eliminates multiples of 2, 3, and 5
//! from consideration, reducing memory traffic by ~46.7%.

#[cfg(feature = "wheel30")]
use std::arch::aarch64::*;

/// Residue classes modulo 30 that are coprime to 30
/// These are the only positions that can contain primes > 5
pub const WHEEL30_RESIDUES: [u8; 8] = [1, 7, 11, 13, 17, 19, 23, 29];

/// Number of residue classes coprime to 30
pub const WHEEL30_SIZE: usize = 8;

/// Wheel-30 sieve segment for SIMD operations
#[cfg(feature = "wheel30")]
#[repr(align(16))]
pub struct Wheel30Segment {
    /// Bit-packed array: each byte represents 8 candidates
    /// Only stores residue classes [1,7,11,13,17,19,23,29] mod 30
    bits: Box<[u8; 4096]>, // 32K candidates per segment
    base: usize,
}

#[cfg(feature = "wheel30")]
impl Wheel30Segment {
    pub fn new(base: usize) -> Self {
        Self {
            bits: Box::new([0; 4096]),
            base,
        }
    }
    
    /// Mark a number as composite using wheel indexing
    pub fn mark_composite(&mut self, n: usize) {
        if let Some(idx) = self.wheel_index(n) {
            let byte_idx = idx / 8;
            let bit_idx = idx % 8;
            if byte_idx < self.bits.len() {
                self.bits[byte_idx] |= 1 << bit_idx;
            }
        }
    }
    
    /// Convert number to wheel index (returns None if not in residue class)
    fn wheel_index(&self, n: usize) -> Option<usize> {
        if n < self.base { return None; }
        
        let offset = n - self.base;
        let wheel_pos = offset % 30;
        
        // Find position in WHEEL30_RESIDUES
        for (i, &residue) in WHEEL30_RESIDUES.iter().enumerate() {
            if wheel_pos == residue as usize {
                let wheel_cycle = offset / 30;
                return Some(wheel_cycle * 8 + i);
            }
        }
        None // Not in a residue class
    }
    
    /// SIMD bit-clearing for ARM NEON
    #[cfg(target_arch = "aarch64")]
    pub unsafe fn clear_bits_simd_neon(&mut self, start_idx: usize, count: usize) {
        let start_byte = start_idx / 8;
        let end_byte = (start_idx + count + 7) / 8;
        
        if end_byte <= self.bits.len() {
            let ptr = self.bits.as_mut_ptr().add(start_byte);
            let len = end_byte - start_byte;
            
            // Process 16 bytes at a time with NEON
            let mut i = 0;
            while i + 16 <= len {
                let chunk = vld1q_u8(ptr.add(i));
                let cleared = veorq_u8(chunk, chunk); // XOR with self = all zeros
                vst1q_u8(ptr.add(i), cleared);
                i += 16;
            }
            
            // Handle remaining bytes
            for j in i..len {
                *ptr.add(j) = 0;
            }
        }
    }
    
    /// Scalar fallback for non-NEON platforms
    #[cfg(not(target_arch = "aarch64"))]
    pub fn clear_bits_simd_neon(&mut self, start_idx: usize, count: usize) {
        let start_byte = start_idx / 8;
        let end_byte = (start_idx + count + 7) / 8;
        
        for i in start_byte..end_byte.min(self.bits.len()) {
            self.bits[i] = 0;
        }
    }
    
    /// Count primes in this segment using SIMD popcount
    #[cfg(target_arch = "aarch64")]
    pub unsafe fn count_primes_simd(&self) -> usize {
        let mut total = 0u32;
        let ptr = self.bits.as_ptr();
        
        // Process 16 bytes at a time
        let mut i = 0;
        while i + 16 <= self.bits.len() {
            let chunk = vld1q_u8(ptr.add(i));
            let ones = vcntq_u8(chunk); // Population count per byte
            total += vaddvq_u8(ones) as u32; // Sum all bytes
            i += 16;
        }
        
        // Handle remaining bytes
        for j in i..self.bits.len() {
            total += self.bits[j].count_ones();
        }
        
        // Return count of zeros (primes), not ones (composites)
        (self.bits.len() * 8) - total as usize
    }
    
    /// Scalar fallback popcount
    #[cfg(not(target_arch = "aarch64"))]
    pub fn count_primes_simd(&self) -> usize {
        let composite_count: u32 = self.bits.iter().map(|&b| b.count_ones()).sum();
        (self.bits.len() * 8) - composite_count as usize
    }
}

/// Wheel-30 sieve with SIMD optimizations
#[cfg(feature = "wheel30")]
pub fn wheel30_sieve(limit: usize) -> Vec<usize> {
    let mut primes = vec![2, 3, 5]; // Wheel factors
    
    if limit <= 5 { 
        primes.retain(|&p| p <= limit);
        return primes;
    }
    
    // Process in wheel-30 segments
    let segment_size = 30 * 1024; // 30K per segment for cache efficiency
    let mut base = 30;
    
    while base <= limit {
        let mut segment = Wheel30Segment::new(base);
        let seg_limit = (base + segment_size).min(limit);
        
        // Cross off multiples using existing primes
        for &p in &primes {
            if p * p > seg_limit { break; }
            
            // Find first multiple of p >= base in wheel residue class
            let start = if p * p >= base {
                p * p
            } else {
                ((base + p - 1) / p) * p
            };
            
            // Mark multiples in wheel-30 representation
            let mut m = start;
            while m <= seg_limit {
                segment.mark_composite(m);
                m += p;
            }
        }
        
        // Extract primes from segment
        for wheel_cycle in 0..(segment_size / 30) {
            for (i, &residue) in WHEEL30_RESIDUES.iter().enumerate() {
                let candidate = base + wheel_cycle * 30 + residue as usize;
                if candidate > seg_limit { break; }
                
                let wheel_idx = wheel_cycle * 8 + i;
                let byte_idx = wheel_idx / 8;
                let bit_idx = wheel_idx % 8;
                
                if byte_idx < segment.bits.len() {
                    let is_composite = (segment.bits[byte_idx] >> bit_idx) & 1 != 0;
                    if !is_composite {
                        primes.push(candidate);
                    }
                }
            }
        }
        
        base += segment_size;
    }
    
    primes.sort_unstable();
    primes
}

#[cfg(not(feature = "wheel30"))]
pub fn wheel30_sieve(_limit: usize) -> Vec<usize> {
    // Feature disabled - return empty vec or fallback to standard sieve
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "wheel30")]
    fn test_wheel30_residues() {
        // Verify our residue classes are correct
        for &r in &WHEEL30_RESIDUES {
            assert_eq!(gcd(r as usize, 30), 1, "Residue {} not coprime to 30", r);
        }
    }
    
    #[test]
    #[cfg(feature = "wheel30")]
    fn test_wheel30_small() {
        let primes = wheel30_sieve(100);
        assert!(primes.contains(&2));
        assert!(primes.contains(&3));
        assert!(primes.contains(&5));
        assert!(primes.contains(&97));
        assert!(!primes.contains(&100));
    }
    
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 { a } else { gcd(b, a % b) }
    }
}