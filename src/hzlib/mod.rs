//! # Hardy-Littlewood Analysis Framework
//!
//! **Layer**: Math core (verified, tested)
//!
//! Number-theoretic, asymptotic, and statistical tools for analyzing prime
//! distributions. This module is the main mathematical support layer for the
//! repository and is independent of the optional metaphor-oriented APIs.
//!
//! ## Submodules
//!
//! | Module | Purpose |
//! |--------|---------|
//! | [`hardy_littlewood`] | Singular series, Goldbach lambda, truncated expectations |
//! | [`sieve`] | Smallest-prime-factor sieve, boolean sieve, prime enumeration |
//! | [`stats`] | Hedges' g, Cliff's delta, Spearman rho, Benjamini-Hochberg, linear regression |
//! | [`num_theory`] | GCD, LCM, Euler phi, Carmichael lambda, multiplicative order |
//! | [`density`] | Prime density prediction and band analysis |
//! | [`orthogonality`] | Babylonian-prime divergence analysis |
//! | [`crt_patterns`] | Chinese Remainder Theorem pattern detection |
//!
//! ## Quick Start
//!
//! ```rust
//! use primes::hzlib::*;
//!
//! let spf = sieve_spf(10_000);
//! let lambda = hl_goldbach_lambda(1000, &spf, PairCount::Unordered);
//! let restricted = hl_goldbach_lambda_truncated(1000, 100, &spf, PairCount::Unordered);
//! let coverage = goldbach_coverage_from_lambda(lambda);
//! let material = Material::for_base10(7);
//! let is_prime = sieve_bool(1000);
//! let actual_restricted_pairs = count_pairs_for_n(1000, 100, &is_prime);
//!
//! assert!(lambda > 0.0);
//! assert!(restricted > 0.0);
//! assert!(coverage > 0.0 && coverage < 1.0);
//! assert_eq!(material.core, 7);
//! assert_eq!(material.ord, 6);
//! assert!(actual_restricted_pairs > 0);
//! ```

pub mod harmonic_lagrange;
pub mod harmonic_overtones;
pub mod symmetry_breaking;
pub mod verification;

// Statistical and mathematical utilities
pub mod crt_patterns;
pub mod density;
pub mod hardy_littlewood;
pub mod num_theory;
pub mod orthogonality;
pub mod sieve;
pub mod stats;

// Re-export commonly used items
pub use hardy_littlewood::{
    count_pairs_for_n, goldbach_coverage_from_lambda, hl_goldbach_lambda,
    hl_goldbach_lambda_truncated, kappa, singular_series_goldbach_multiplicative, PairCount, C2,
};

pub use sieve::{segmented_sieve, sieve_bool, sieve_primes, sieve_spf};

pub use stats::{
    benjamini_hochberg, cliffs_delta, hedges_g, linreg, linreg_with_ci, spearman_rho, welch_t,
};

pub use num_theory::{
    carmichael_lambda_from_factor, factor, gcd, lcm, multiplicative_order, phi_from_factor,
    pow_mod, strip_factors, Material,
};

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Map from (mid_len, inner_zero) to list of (modulus, probability) pairs
pub type ExplainMap = HashMap<(usize, usize), Vec<(u32, f64)>>;

#[derive(Clone, Debug)]
pub struct Pair {
    pub base: u32,
    pub mid_len: usize,
    pub inner_zero: usize,
    pub obs_density: Option<f64>,
    pub ci_lo: Option<f64>,
    pub ci_hi: Option<f64>,
    pub pred_local: Option<f64>,
    pub pred_local_exact: Option<f64>,
    pub tracked_moduli: Vec<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    Mid,
    InnerZero,
}

#[derive(Clone, Debug)]
pub struct JoinedGrid {
    pub mids: Vec<usize>,
    pub izs: Vec<usize>,
    pub pairs: Vec<Pair>, // row-major over iz × mid
}

impl JoinedGrid {
    pub fn idx(&self, mid: usize, iz: usize) -> Option<usize> {
        let i = self.mids.iter().position(|&m| m == mid)?;
        let j = self.izs.iter().position(|&z| z == iz)?;
        Some(j * self.mids.len() + i)
    }
}

pub fn load_sample_csv<P: AsRef<Path>>(path: P) -> io::Result<Vec<HashMap<String, String>>> {
    load_csv_internal(path)
}
pub fn load_model_csv<P: AsRef<Path>>(path: P) -> io::Result<Vec<HashMap<String, String>>> {
    load_csv_internal(path)
}

/// Robust single-line CSV splitter that keeps bracketed lists `[ ... ]` intact (no quotes used by producer).
fn split_csv_line_keep_brackets(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut depth = 0i32;
    for ch in line.chars() {
        match ch {
            '[' => {
                depth += 1;
                cur.push(ch);
            }
            ']' => {
                depth -= 1;
                cur.push(ch);
            }
            ',' if depth == 0 => {
                out.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(ch),
        }
    }
    out.push(cur.trim().to_string());
    out
}

fn load_csv_internal<P: AsRef<Path>>(path: P) -> io::Result<Vec<HashMap<String, String>>> {
    let f = File::open(path)?;
    let mut it = BufReader::new(f).lines();
    let header = if let Some(Ok(h)) = it.next() {
        h
    } else {
        return Ok(vec![]);
    };
    let cols: Vec<String> = header.split(',').map(|s| s.trim().to_string()).collect();
    let mut rows = Vec::new();
    for line in it {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let parts = split_csv_line_keep_brackets(&line);
        let mut map = HashMap::new();
        for (i, v) in parts.into_iter().enumerate() {
            if i < cols.len() {
                map.insert(cols[i].clone(), v);
            }
        }
        rows.push(map);
    }
    Ok(rows)
}

fn parse_u32_list_field(v: &str) -> Vec<u32> {
    // Accept "[]" or "[1, 3, 7]" or "".
    let s = v.trim();
    if s.is_empty() {
        return vec![];
    }
    let t = s.trim_matches(|c| c == '[' || c == ']' || c == ' ');
    if t.is_empty() {
        return vec![];
    }
    t.split(',')
        .filter_map(|x| x.trim().parse::<u32>().ok())
        .collect()
}

fn as_usize(map: &HashMap<String, String>, key: &str) -> Option<usize> {
    map.get(key)?.parse::<usize>().ok()
}
fn as_u32(map: &HashMap<String, String>, key: &str) -> Option<u32> {
    map.get(key)?.parse::<u32>().ok()
}
fn as_f64(map: &HashMap<String, String>, key: &str) -> Option<f64> {
    map.get(key)?.parse::<f64>().ok()
}

pub fn join_sample_and_model(
    sample_rows: &[HashMap<String, String>],
    model_rows: &[HashMap<String, String>],
) -> JoinedGrid {
    let mut mids = std::collections::BTreeSet::new();
    let mut izs = std::collections::BTreeSet::new();

    for r in sample_rows {
        if let (Some(m), Some(z)) = (as_usize(r, "mid_len"), as_usize(r, "inner_zero")) {
            mids.insert(m);
            izs.insert(z);
        }
    }
    for r in model_rows {
        if let (Some(m), Some(z)) = (as_usize(r, "mid_len"), as_usize(r, "inner_zero")) {
            mids.insert(m);
            izs.insert(z);
        }
    }

    let mids: Vec<usize> = mids.into_iter().collect();
    let izs: Vec<usize> = izs.into_iter().collect();

    let key = |m: usize, z: usize| format!("{m}_{z}");

    let mut smap: HashMap<String, &HashMap<String, String>> = HashMap::new();
    for r in sample_rows {
        if let (Some(m), Some(z)) = (as_usize(r, "mid_len"), as_usize(r, "inner_zero")) {
            smap.insert(key(m, z), r);
        }
    }
    let mut mmap: HashMap<String, &HashMap<String, String>> = HashMap::new();
    for r in model_rows {
        if let (Some(m), Some(z)) = (as_usize(r, "mid_len"), as_usize(r, "inner_zero")) {
            mmap.insert(key(m, z), r);
        }
    }

    let mut pairs = Vec::with_capacity(mids.len() * izs.len());
    for &z in &izs {
        for &m in &mids {
            let k = key(m, z);
            let s = smap.get(&k).copied();
            let t = mmap.get(&k).copied();
            let base = s
                .and_then(|x| as_u32(x, "base"))
                .or_else(|| t.and_then(|x| as_u32(x, "base")))
                .unwrap_or(10);
            let obs = s.and_then(|x| as_f64(x, "prime_density"));
            let ci_lo = s.and_then(|x| as_f64(x, "ci_lo"));
            let ci_hi = s.and_then(|x| as_f64(x, "ci_hi"));
            let pred_exact = s
                .and_then(|x| as_f64(x, "expected_density_local_exact"))
                .or_else(|| t.and_then(|x| as_f64(x, "expected_density_local_exact")));
            let pred_local = s
                .and_then(|x| as_f64(x, "expected_density_local"))
                .or_else(|| t.and_then(|x| as_f64(x, "expected_density_local")));
            let tracked = s
                .and_then(|x| x.get("tracked_moduli").map(|v| parse_u32_list_field(v)))
                .or_else(|| {
                    t.and_then(|x| x.get("tracked_moduli").map(|v| parse_u32_list_field(v)))
                })
                .unwrap_or_default();

            pairs.push(Pair {
                base,
                mid_len: m,
                inner_zero: z,
                obs_density: obs,
                ci_lo,
                ci_hi,
                pred_local,
                pred_local_exact: pred_exact,
                tracked_moduli: tracked,
            });
        }
    }

    JoinedGrid { mids, izs, pairs }
}

/// Extract a 1D series along an axis from a `JoinedGrid`.
///
/// # Example
/// ```
/// use primes::hzlib::{Axis, JoinedGrid, Pair, lineout};
///
/// let grid = JoinedGrid {
///     mids: vec![1, 2],
///     izs: vec![0],
///     pairs: vec![
///         Pair {
///             base: 10,
///             mid_len: 1,
///             inner_zero: 0,
///             obs_density: Some(0.5),
///             ci_lo: None,
///             ci_hi: None,
///             pred_local: Some(0.25),
///             pred_local_exact: None,
///             tracked_moduli: vec![],
///         },
///         Pair {
///             base: 10,
///             mid_len: 2,
///             inner_zero: 0,
///             obs_density: Some(0.25),
///             ci_lo: None,
///             ci_hi: None,
///             pred_local: Some(0.125),
///             pred_local_exact: None,
///             tracked_moduli: vec![],
///         },
///     ],
/// };
///
/// let slice = lineout(&grid, Axis::Mid, 1, 0);
/// assert_eq!(slice.len(), 2);
/// assert_eq!(slice[0], (1, 0.5, 0.25));
/// assert_eq!(slice[1], (2, 0.25, 0.125));
/// ```
pub fn lineout(
    grid: &JoinedGrid,
    axis: Axis,
    fixed_mid: usize,
    fixed_iz: usize,
) -> Vec<(usize, f64, f64)> {
    match axis {
        Axis::Mid => {
            let mut v = Vec::with_capacity(grid.mids.len());
            for &m in &grid.mids {
                let idx = grid.idx(m, fixed_iz).unwrap();
                let p = &grid.pairs[idx];
                let pred = p.pred_local_exact.or(p.pred_local).unwrap_or(0.0);
                let obs = p.obs_density.unwrap_or(0.0);
                v.push((m, obs, pred));
            }
            v
        }
        Axis::InnerZero => {
            let mut v = Vec::with_capacity(grid.izs.len());
            for &z in &grid.izs {
                let idx = grid.idx(fixed_mid, z).unwrap();
                let p = &grid.pairs[idx];
                let pred = p.pred_local_exact.or(p.pred_local).unwrap_or(0.0);
                let obs = p.obs_density.unwrap_or(0.0);
                v.push((z, obs, pred));
            }
            v
        }
    }
}

/// Convenience: enrichment = obs/pred - 1 (clamped if pred≈0).
///
/// # Example
/// ```
/// use primes::hzlib::enrichment;
///
/// assert_eq!(enrichment(0.75, 0.5), 0.5);
/// assert_eq!(enrichment(0.0, 0.0), 0.0);
/// assert!(enrichment(1.0, 0.0).is_infinite());
/// ```
pub fn enrichment(obs: f64, pred: f64) -> f64 {
    if pred <= 0.0 {
        if obs <= 0.0 {
            0.0
        } else {
            f64::INFINITY
        }
    } else {
        obs / pred - 1.0
    }
}

/// Parse ExplainGrid JSON array (optional) and return a map (mid,iz) -> top (p, p0).
pub fn load_explain_json<P: AsRef<Path>>(path: P) -> io::Result<ExplainMap> {
    let f = File::open(path)?;
    let vals: serde_json::Value =
        serde_json::from_reader(f).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let mut out = HashMap::new();
    let arr = vals
        .as_array()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected array"))?;
    for v in arr {
        let mid = v
            .get("mid_len")
            .and_then(|x| x.as_u64())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "mid_len"))?
            as usize;
        let iz = v
            .get("inner_zero")
            .and_then(|x| x.as_u64())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "inner_zero"))?
            as usize;
        let list_opt = v.get("model_p0").and_then(|x| x.as_array());
        let mut pairs = Vec::new();
        if let Some(list) = list_opt {
            for it in list {
                if let (Some(p), Some(p0)) = (
                    it.get(0).and_then(|x| x.as_u64()),
                    it.get(1).and_then(|x| x.as_f64()),
                ) {
                    pairs.push((p as u32, p0));
                }
            }
        }
        pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        out.insert((mid, iz), pairs);
    }
    Ok(out)
}
