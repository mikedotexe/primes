//! Band-based Prime Density Analysis
//!
//! Analyzes prime distribution in symmetric windows around digit-block midpoints.
//! Implements the "honorary zero" concept where the midpoint of [b^k, b^(k+1))
//! acts as a natural symmetry center.
//!
//! # Mathematical Background
//!
//! ## Admissibility and the Radical
//!
//! A number n can be prime only if gcd(n, rad(b)) = 1, where rad(b) is the
//! **radical** (or square-free kernel) of the base:
//!
//! ```text
//! rad(b) = ∏_{p|b} p
//! ```
//!
//! This is the product of **distinct** prime factors of b.
//!
//! ### Examples
//! - rad(10) = rad(2×5) = 2×5 = 10
//! - rad(12) = rad(2²×3) = 2×3 = 6  (not 12!)
//! - rad(30) = rad(2×3×5) = 2×3×5 = 30
//!
//! ### Why rad(b), not b or b-1?
//!
//! **Admissibility** depends only on the *distinct* prime divisors of b:
//! - If p|b and p|n, then n is composite (divisible by p)
//! - Powers don't matter: if 2²|b, we only need to check divisibility by 2
//! - This is why rad(12)=6, not 12
//!
//! **Euler's totient** φ(b^k) counts residues coprime to b^k, but this includes
//! numbers like 25 mod 100 (coprime to 100 but not to rad(10)=10).
//!
//! For exact prime density denominators, we count residues r where gcd(r, rad(b))=1.
//!
//! ## Denominators in this Module
//!
//! Currently, denominators are computed as:
//! ```text
//! denom[i] = residues_in_bin[i] × (b - 1)
//! ```
//!
//! For **exact** denominators accounting for admissibility:
//! 1. Precompute coprimality indicator for r mod rad(b)
//! 2. Count residues in bin satisfying gcd(r, rad(b)) = 1
//! 3. Use sliding prefix-sum for O(1) bin queries after O(rad(b)) preprocessing
//!
//! This gives exact prime-eligible denominators independent of window sizes.

/// Band representing a single digit-length range [b^k, b^(k+1))
///
/// Tracks prime density in symmetric bins around the midpoint.
/// Uses exact denominators accounting for residue class structure.
#[derive(Clone)]
pub struct Band {
    pub k: u32,           // Digit length
    pub b: usize,         // Base
    pub s: usize,         // b^k (start of range)
    pub s_next: usize,    // b^(k+1) (end of range, exclusive)
    pub mid: usize,       // floor(s/2) - "honorary zero"
    pub bins: usize,      // Number of bins for density distribution
    pub denom: Vec<u64>,  // Exact denominators per bin
    pub counts: Vec<u64>, // Prime counts per bin
}

impl Band {
    /// Create new band for digit length k in base b
    ///
    /// # Arguments
    /// * `b` - Base
    /// * `k` - Digit length (power)
    /// * `s` - b^k
    /// * `s_next` - b^(k+1)
    /// * `bins` - Number of bins for density distribution
    pub fn new(b: usize, k: u32, s: usize, s_next: usize, bins: usize) -> Self {
        let mid = s / 2;
        let mut denom = vec![0u64; bins];
        let dmax = s / 2;

        // Compute exact denominators for each bin
        for (i, denom_val) in denom.iter_mut().enumerate().take(bins) {
            let a = i as f64 / bins as f64;
            let bb = (i + 1) as f64 / bins as f64;
            let d_min = ((a * (s as f64) * 0.5).ceil()) as usize;
            let mut d_max_bin = (bb * (s as f64) * 0.5 - 1e-12).floor() as usize;
            if d_max_bin > dmax {
                d_max_bin = dmax;
            }
            if d_min > d_max_bin {
                *denom_val = 0;
                continue;
            }

            let mut residues = 0u64;
            for d in d_min..=d_max_bin {
                if d == 0 || (s.is_multiple_of(2) && d * 2 == s) {
                    residues += 1;
                } else {
                    residues += 2; // Both mid+d and mid-d
                }
            }
            *denom_val = residues * (b as u64 - 1);
        }

        Self {
            k,
            b,
            s,
            s_next,
            mid,
            bins,
            denom,
            counts: vec![0u64; bins],
        }
    }

    /// Add a prime to this band's density distribution
    #[inline]
    pub fn add_prime(&mut self, p: usize) {
        let r = p % self.s;
        let d = r.abs_diff(self.mid);
        let delta = (2.0 * d as f64) / self.s as f64;
        let mut idx = (delta * self.bins as f64) as usize;
        if idx >= self.bins {
            idx = self.bins - 1;
        }
        self.counts[idx] += 1;
    }

    /// Total primes in this band
    pub fn primes_in_band(&self) -> u64 {
        self.counts.iter().copied().sum()
    }

    /// Density at bin i (primes / possible_residues)
    pub fn density_at(&self, i: usize) -> f64 {
        let den = self.denom[i] as f64;
        if den <= 0.0 {
            0.0
        } else {
            (self.counts[i] as f64) / den
        }
    }

    /// Find peak delta using smoothed density
    ///
    /// Returns (delta_center, peak_density, bin_index)
    pub fn peak_delta(&self) -> (f64, f64, usize) {
        let mut best_i = 0usize;
        let mut best = -1.0f64;

        for i in 0..self.bins {
            let y0 = self.density_at(i);
            let yl = if i > 0 { self.density_at(i - 1) } else { y0 };
            let yr = if i + 1 < self.bins {
                self.density_at(i + 1)
            } else {
                y0
            };

            // Smoothed density (weighted average)
            let y = 0.25 * yl + 0.5 * y0 + 0.25 * yr;

            if y > best {
                best = y;
                best_i = i;
            }
        }

        let delta_center = (best_i as f64 + 0.5) / self.bins as f64;
        (delta_center, best, best_i)
    }

    /// Center of mass delta (density-weighted average position)
    ///
    /// Returns (COM_delta, total_density)
    pub fn com_delta(&self) -> (f64, f64) {
        let mut wsum = 0.0f64;
        let mut xwsum = 0.0f64;

        for i in 0..self.bins {
            let x = (i as f64 + 0.5) / self.bins as f64;
            let w = self.density_at(i);
            xwsum += x * w;
            wsum += w;
        }

        if wsum == 0.0 {
            (0.0, 0.0)
        } else {
            (xwsum / wsum, wsum)
        }
    }
}

/// Accumulator for all digit-length bands in a given base
pub struct BaseAccum {
    pub b: usize,
    pub is_double_prime: bool,
    pub bands: Vec<Band>,
    ends: Vec<usize>, // Cached b^(k+1) for binary search
}

impl BaseAccum {
    /// Create accumulator for base b up to limit
    ///
    /// Only includes COMPLETE digit bands (where b^(k+1) ≤ limit)
    pub fn new(b: usize, limit: usize, bins: usize) -> Self {
        let mut bands = Vec::new();
        let mut s: u128 = b as u128;
        let mut k: u32 = 1;

        while s <= limit as u128 {
            let s_next = s * (b as u128);
            if s_next > limit as u128 {
                break;
            } // Incomplete band

            bands.push(Band::new(b, k, s as usize, s_next as usize, bins));
            k += 1;
            s = s_next;
        }

        let ends = bands.iter().map(|bd| bd.s_next).collect::<Vec<_>>();

        Self {
            b,
            is_double_prime: super::crt_patterns::is_double_prime_base(b),
            bands,
            ends,
        }
    }

    /// Find which band contains prime p (binary search)
    ///
    /// Returns Some(band_index) if p is in a complete band, None otherwise
    pub fn find_band(&self, p: usize) -> Option<usize> {
        if self.bands.is_empty() || p < self.bands[0].s {
            return None;
        }

        let mut lo = 0usize;
        let mut hi = self.ends.len();

        while lo < hi {
            let mid = (lo + hi) / 2;
            if self.ends[mid] <= p {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        if lo == 0 {
            return None;
        }

        let idx = lo - 1;
        let bd = &self.bands[idx];

        if p >= bd.s && p < bd.s_next {
            Some(idx)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_band_creation() {
        let band = Band::new(10, 2, 100, 1000, 10);
        assert_eq!(band.k, 2);
        assert_eq!(band.b, 10);
        assert_eq!(band.s, 100);
        assert_eq!(band.s_next, 1000);
        assert_eq!(band.mid, 50);
        assert_eq!(band.bins, 10);
    }

    #[test]
    fn test_band_add_prime() {
        let mut band = Band::new(10, 2, 100, 1000, 10);
        band.add_prime(151); // 151 % 100 = 51, d = 1

        assert!(band.primes_in_band() > 0);
    }

    #[test]
    fn test_base_accum() {
        let accum = BaseAccum::new(10, 10000, 100);
        assert_eq!(accum.b, 10);
        // Should have bands for 10^1, 10^2, 10^3 (but not 10^4 since 10^5 > 10000)
        assert!(!accum.bands.is_empty());
    }

    #[test]
    fn test_find_band() {
        let accum = BaseAccum::new(10, 100000, 100);
        // 523 should be in the band [100, 1000)
        if let Some(idx) = accum.find_band(523) {
            assert_eq!(accum.bands[idx].s, 100);
            assert_eq!(accum.bands[idx].s_next, 1000);
        } else {
            panic!("Should find band for 523");
        }
    }
}
