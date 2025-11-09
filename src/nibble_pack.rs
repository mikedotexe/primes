//! Nibble packing for efficient base-6/12 storage

use rayon::prelude::*;

/// Pack base-12 digits into 4-bit nibbles
pub fn pack_base12(candidates: &[u32]) -> Vec<u32> {
    candidates
        .par_chunks(8)
        .map(|chunk| {
            let mut packed = 0u32;
            for (i, &c) in chunk.iter().enumerate() {
                let digit = c % 12; // Take only valid base-12 digit
                packed |= (digit & 0xF) << (i * 4);
            }
            packed
        })
        .collect()
}

/// Pack base-6 digits into 4-bit nibbles (3 bits would suffice but 4-bit is cache-friendly)
pub fn pack_base6(candidates: &[u32]) -> Vec<u32> {
    candidates
        .par_chunks(8)
        .map(|chunk| {
            let mut packed = 0u32;
            for (i, &c) in chunk.iter().enumerate() {
                let digit = c % 6; // Take only valid base-6 digit
                packed |= (digit & 0xF) << (i * 4);
            }
            packed
        })
        .collect()
}

/// Unpack nibbles back to candidates
pub fn unpack_nibbles(packed: &[u32], base: u32) -> Vec<u32> {
    let mut unpacked = Vec::with_capacity(packed.len() * 8);
    for &p in packed {
        for i in 0..8 {
            let digit = (p >> (i * 4)) & 0xF;
            if digit < base {
                unpacked.push(digit);
            }
        }
    }
    unpacked
}

/// Pack candidates based on base
pub fn pack_candidates(candidates: &[u32], base: u32) -> Vec<u32> {
    match base {
        6 => pack_base6(candidates),
        12 => pack_base12(candidates),
        _ => candidates.to_vec(), // No packing for other bases
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_unpack_base6() {
        let candidates = vec![0, 1, 2, 3, 4, 5, 0, 1];
        let packed = pack_base6(&candidates);
        assert_eq!(packed.len(), 1); // 8 digits -> 1 u32

        let unpacked = unpack_nibbles(&packed, 6);
        assert_eq!(unpacked, candidates);
    }

    #[test]
    fn test_pack_unpack_base12() {
        let candidates = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let packed = pack_base12(&candidates);
        assert_eq!(packed.len(), 2); // 12 digits -> 2 u32s (8 + 4)

        let unpacked = unpack_nibbles(&packed, 12);
        assert_eq!(unpacked[..12], candidates);
    }
}
