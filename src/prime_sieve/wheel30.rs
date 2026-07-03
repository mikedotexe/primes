//! Wheel-30 compressed sieve utilities.
//!
//! This module exposes the exact wheel-30 residue surface used elsewhere in the
//! repo together with a correctness-first segmented sieve over those residues.
//! Optional AArch64 NEON helpers are kept for byte-array operations, but the
//! main wheel-30 sieve path is scalar.

#[cfg(all(feature = "wheel30", target_arch = "aarch64"))]
use std::arch::aarch64::*;

/// Residue classes modulo 30 that are coprime to 30
/// These are the only positions that can contain primes > 5
pub const WHEEL30_RESIDUES: [u8; 8] = [1, 7, 11, 13, 17, 19, 23, 29];

/// Number of residue classes coprime to 30
pub const WHEEL30_SIZE: usize = 8;

/// Number of bytes in one wheel-30 segment.
pub const WHEEL30_SEGMENT_BYTES: usize = 4096;

/// Number of wheel cycles represented in one wheel-30 segment.
pub const WHEEL30_SEGMENT_CYCLES: usize = WHEEL30_SEGMENT_BYTES;

/// Total numeric span covered by one wheel-30 segment.
pub const WHEEL30_SEGMENT_SPAN: usize = WHEEL30_SEGMENT_CYCLES * 30;

/// Wheel-30 sieve segment for SIMD operations
#[cfg(feature = "wheel30")]
#[repr(align(16))]
pub struct Wheel30Segment {
    /// Bit-packed array: each byte represents 8 candidates
    /// Only stores residue classes \[1,7,11,13,17,19,23,29\] mod 30
    bits: Box<[u8; WHEEL30_SEGMENT_BYTES]>, // 32,768 candidates per segment
    base: usize,
}

#[cfg(feature = "wheel30")]
impl Wheel30Segment {
    pub fn new(base: usize) -> Self {
        Self {
            bits: Box::new([0; WHEEL30_SEGMENT_BYTES]),
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
        if n < self.base {
            return None;
        }

        let offset = n - self.base;
        let wheel_pos = offset % 30;

        // Find position in WHEEL30_RESIDUES
        for (i, &residue) in WHEEL30_RESIDUES.iter().enumerate() {
            if wheel_pos == residue as usize {
                let wheel_cycle = offset / 30;
                if wheel_cycle >= WHEEL30_SEGMENT_CYCLES {
                    return None;
                }
                return Some(wheel_cycle * 8 + i);
            }
        }
        None // Not in a residue class
    }

    /// SIMD bit-clearing for ARM NEON
    ///
    /// # Safety
    ///
    /// Caller must ensure the CPU supports AArch64 NEON instructions and that
    /// `start_idx..start_idx + count` refers to the current segment's logical
    /// bit range.
    #[cfg(target_arch = "aarch64")]
    pub unsafe fn clear_bits_simd_neon(&mut self, start_idx: usize, count: usize) {
        let start_byte = start_idx / 8;
        let end_byte = start_idx.saturating_add(count).div_ceil(8);

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
    ///
    /// # Safety
    ///
    /// Caller must ensure:
    /// - The function is called on a properly aligned ARM NEON-capable CPU
    /// - The bit array is properly initialized
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
    let mut primes: Vec<usize> = [2usize, 3, 5].into_iter().filter(|&p| p <= limit).collect();

    if limit < 7 {
        return primes;
    }

    let root = (limit as f64).sqrt() as usize + 1;
    let base_primes = super::BitSieve::new(root).primes();
    let mut base = 0usize;

    while base <= limit {
        let mut segment = Wheel30Segment::new(base);
        let seg_limit = base
            .saturating_add(WHEEL30_SEGMENT_SPAN.saturating_sub(1))
            .min(limit);

        // Cross off multiples using primes up to sqrt(limit).
        // We skip 2, 3, and 5 because the wheel already removes them.
        for &p in &base_primes {
            if p <= 5 {
                continue;
            }
            if p * p > seg_limit {
                break;
            }

            let start = if p * p >= base {
                p * p
            } else {
                base.div_ceil(p) * p
            };

            let mut m = start;
            while m <= seg_limit {
                segment.mark_composite(m);
                match m.checked_add(p) {
                    Some(next) => m = next,
                    None => break,
                }
            }
        }

        let segment_cycles = (seg_limit - base) / 30 + 1;
        for wheel_cycle in 0..segment_cycles {
            for (i, &residue) in WHEEL30_RESIDUES.iter().enumerate() {
                let candidate = base + wheel_cycle * 30 + residue as usize;
                if candidate > seg_limit {
                    break;
                }
                if candidate <= 5 {
                    continue;
                }

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

        if seg_limit == limit {
            break;
        }
        base += WHEEL30_SEGMENT_SPAN;
    }

    primes
}

#[cfg(not(feature = "wheel30"))]
pub fn wheel30_sieve(limit: usize) -> Vec<usize> {
    super::BitSieve::new(limit).primes()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prime_sieve::BitSieve;

    #[test]
    fn test_wheel30_residues() {
        for &r in &WHEEL30_RESIDUES {
            assert_eq!(gcd(r as usize, 30), 1, "Residue {} not coprime to 30", r);
        }
        assert_eq!(WHEEL30_RESIDUES.len(), WHEEL30_SIZE);
    }

    #[test]
    fn test_wheel30_segment_constants() {
        assert_eq!(WHEEL30_SEGMENT_BYTES, 4096);
        assert_eq!(WHEEL30_SEGMENT_CYCLES, WHEEL30_SEGMENT_BYTES);
        assert_eq!(WHEEL30_SEGMENT_SPAN, 122_880);
    }

    #[test]
    fn test_wheel30_small_matches_baseline() {
        let primes = wheel30_sieve(100);
        assert_eq!(primes, BitSieve::new(100).primes());
    }

    #[test]
    fn test_wheel30_handles_small_limits() {
        for limit in [0usize, 1, 2, 3, 5, 6, 7, 29, 30] {
            assert_eq!(
                wheel30_sieve(limit),
                BitSieve::new(limit).primes(),
                "limit {limit}"
            );
        }
    }

    #[test]
    fn test_wheel30_matches_baseline_across_ranges() {
        for limit in [100usize, 1_000, 10_000, 100_000] {
            let expected = BitSieve::new(limit).primes();
            let actual = wheel30_sieve(limit);
            assert_eq!(actual, expected, "wheel30 mismatch at limit {limit}");
        }
    }

    #[test]
    fn test_wheel30_rejects_square_of_seven() {
        let primes = wheel30_sieve(100);
        assert!(!primes.contains(&49));
        assert!(!primes.contains(&77));
        assert!(!primes.contains(&91));
    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
}
