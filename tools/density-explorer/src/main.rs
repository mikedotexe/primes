use clap::{Parser, Subcommand};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::str::FromStr;

const LCM_CAP: u32 = 500_000;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, default_value_t = 42u64)]
    seed: u64,

    #[arg(long, default_value_t = 10u32)]
    base: u32,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Sample {
        #[arg(long)]
        midpoint: String,
        #[arg(long, num_args = 1.., value_delimiter = ' ')]
        layers: Vec<String>,
        #[arg(long, default_value_t = 100_000usize)]
        samples: usize,
        #[arg(long, default_value = "1,3,7,9")]
        allowed_last_digits: String,
        #[arg(long)]
        out_json: Option<String>,
        #[arg(long)]
        out_csv: Option<String>,
        #[arg(long, default_value_t = true)]
        parallel: bool,
        #[arg(long, default_value_t = false)]
        mirror: bool,
        #[arg(long, visible_alias = "track_primes", default_value = "")]
        track_moduli: String,
        #[arg(long, default_value_t = false)]
        auto_track: bool,
        #[arg(long, default_value_t = 97u32)]
        auto_pmax: u32,
        #[arg(long, default_value_t = 8usize)]
        auto_k: usize,
        #[arg(long, default_value_t = 0.03f64)]
        auto_min_p0: f64,
        #[arg(long, default_value_t = true)]
        pre_sieve: bool,
    },

    Grid {
        #[arg(long, default_value = "free")]
        mid_kind: String,
        #[arg(long)]
        mid_len_range: String,
        #[arg(long)]
        inner_zero_range: String,
        #[arg(long, default_value_t = 1usize)]
        inner_slot: usize,
        #[arg(long, num_args = 0.., value_delimiter = ' ')]
        outer_layers: Vec<String>,
        #[arg(long, default_value_t = 50_000usize)]
        samples: usize,
        #[arg(long, default_value = "1,3,7,9")]
        allowed_last_digits: String,
        #[arg(long)]
        out_csv: String,
        #[arg(long, default_value_t = true)]
        parallel: bool,
        #[arg(long, default_value_t = false)]
        mirror: bool,
        #[arg(long, visible_alias = "track_primes", default_value = "")]
        track_moduli: String,
        #[arg(long, default_value_t = false)]
        auto_track: bool,
        #[arg(long, default_value_t = 97u32)]
        auto_pmax: u32,
        #[arg(long, default_value_t = 8usize)]
        auto_k: usize,
        #[arg(long, default_value_t = 0.03f64)]
        auto_min_p0: f64,
        #[arg(long, default_value = "per-cell")]
        auto_mode: String, // per-cell | global
        #[arg(long, default_value_t = true)]
        pre_sieve: bool,
    },

    #[command(name = "model-only")]
    ModelOnly {
        #[arg(long, default_value = "free")]
        mid_kind: String,
        #[arg(long)]
        mid_len_range: String,
        #[arg(long)]
        inner_zero_range: String,
        #[arg(long, default_value_t = 1usize)]
        inner_slot: usize,
        #[arg(long, num_args = 0.., value_delimiter = ' ')]
        outer_layers: Vec<String>,
        #[arg(long, default_value = "1,3,7,9")]
        allowed_last_digits: String,
        #[arg(long)]
        out_csv: String,
        #[arg(long, default_value_t = false)]
        mirror: bool,
        #[arg(long, visible_alias = "track_primes", default_value = "3,5,7,11")]
        track_moduli: String,
        #[arg(long, default_value_t = false)]
        auto_track: bool,
        #[arg(long, default_value_t = 97u32)]
        auto_pmax: u32,
        #[arg(long, default_value_t = 8usize)]
        auto_k: usize,
        #[arg(long, default_value_t = 0.03f64)]
        auto_min_p0: f64,
        #[arg(long, default_value = "per-cell")]
        auto_mode: String, // per-cell | global
    },

    Explain {
        #[arg(long)]
        midpoint: String,
        #[arg(long, num_args = 1.., value_delimiter = ' ')]
        layers: Vec<String>,
        #[arg(long, default_value = "1,3,7,9")]
        allowed_last_digits: String,
        #[arg(long, default_value_t = false)]
        mirror: bool,
        #[arg(long, visible_alias = "track_primes", default_value = "3,5,7,11")]
        track_moduli: String,
        #[arg(long, default_value_t = false)]
        auto_track: bool,
        #[arg(long, default_value_t = 97u32)]
        auto_pmax: u32,
        #[arg(long, default_value_t = 8usize)]
        auto_k: usize,
        #[arg(long, default_value_t = 0.03f64)]
        auto_min_p0: f64,
        #[arg(long)]
        out_json: Option<String>,
    },

    #[command(name = "explain-grid")]
    ExplainGrid {
        #[arg(long, default_value = "free")]
        mid_kind: String,
        #[arg(long)]
        mid_len_range: String,
        #[arg(long)]
        inner_zero_range: String,
        #[arg(long, default_value_t = 1usize)]
        inner_slot: usize,
        #[arg(long, num_args = 0.., value_delimiter = ' ')]
        outer_layers: Vec<String>,
        #[arg(long, default_value = "1,3,7,9")]
        allowed_last_digits: String,
        #[arg(long)]
        out_json: String,
        #[arg(long, default_value_t = false)]
        mirror: bool,
        #[arg(long, visible_alias = "track_primes", default_value = "3,5,7,11")]
        track_moduli: String,
        #[arg(long, default_value_t = false)]
        auto_track: bool,
        #[arg(long, default_value_t = 97u32)]
        auto_pmax: u32,
        #[arg(long, default_value_t = 8usize)]
        auto_k: usize,
        #[arg(long, default_value_t = 0.03f64)]
        auto_min_p0: f64,
        #[arg(long, default_value = "per-cell")]
        auto_mode: String, // per-cell | global
    },

    Lineout {
        #[arg(long, default_value = "mid")] // "mid" or "iz"
        axis: String,
        #[arg(long, default_value = "free")]
        mid_kind: String,
        #[arg(long)]
        range: String,
        #[arg(long, default_value_t = 0usize)]
        inner_zero: usize, // used when axis="mid"
        #[arg(long, default_value_t = 1usize)]
        mid_len: usize,    // used when axis="iz"
        #[arg(long, default_value_t = 1usize)]
        inner_slot: usize,
        #[arg(long, num_args = 0.., value_delimiter = ' ')]
        outer_layers: Vec<String>,
        #[arg(long, default_value_t = 50_000usize)]
        samples: usize,
        #[arg(long, default_value = "1,3,7,9")]
        allowed_last_digits: String,
        #[arg(long)]
        out_csv: String,
        #[arg(long, default_value_t = true)]
        parallel: bool,
        #[arg(long, default_value_t = false)]
        mirror: bool,
        #[arg(long, default_value = "")]
        track_moduli: String,
        #[arg(long, default_value_t = false)]
        auto_track: bool,
        #[arg(long, default_value_t = 97u32)]
        auto_pmax: u32,
        #[arg(long, default_value_t = 8usize)]
        auto_k: usize,
        #[arg(long, default_value_t = 0.03f64)]
        auto_min_p0: f64,
        #[arg(long, default_value = "per-cell")]
        auto_mode: String, // per-cell | global
        #[arg(long, default_value_t = true)]
        pre_sieve: bool,
        #[arg(long, default_value_t = false)]
        model_only: bool,
    },

    Ridge {
        #[arg(long, default_value = "free")]
        mid_kind: String,
        #[arg(long)]
        mid_len_range: String,
        #[arg(long)]
        inner_zero_range: String,
        #[arg(long, default_value_t = 1usize)]
        inner_slot: usize,
        #[arg(long, num_args = 0.., value_delimiter = ' ')]
        outer_layers: Vec<String>,
        #[arg(long, default_value = "1,3,7,9")]
        allowed_last_digits: String,
        #[arg(long)]
        out_csv: String,
        #[arg(long, default_value_t = false)]
        mirror: bool,
        #[arg(long, visible_alias = "track_primes", default_value = "")]
        track_moduli: String,
        #[arg(long, default_value_t = false)]
        auto_track: bool,
        #[arg(long, default_value_t = 97u32)]
        auto_pmax: u32,
        #[arg(long, default_value_t = 8usize)]
        auto_k: usize,
        #[arg(long, default_value_t = 0.03f64)]
        auto_min_p0: f64,
        #[arg(long, default_value = "per-cell")]
        auto_mode: String, // per-cell | global
        #[arg(long, default_value = "exact")]   // exact | prod
        objective: String,
    },

    Run { #[arg(long)] config: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum Midpoint { Free(usize), Zeros(usize) }

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Layer { zero: usize, slot: usize }

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Constraints {
    allowed_last_digits: Vec<u32>,
    forbid_leading_zero: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Pattern {
    base: u32,
    midpoint: Midpoint,
    layers: Vec<Layer>,
    constraints: Constraints,
    mirror: bool,
}
impl Pattern {
    fn total_len(&self) -> usize {
        let mid = match self.midpoint { Midpoint::Free(l) | Midpoint::Zeros(l) => l };
        let side: usize = self.layers.iter().map(|L| L.zero + L.slot).sum();
        mid + 2 * side
    }
}

// ---------------- parsing helpers ----------------

fn parse_midpoint(s: &str) -> Midpoint {
    let parts: Vec<&str> = s.split(':').collect();
    assert!(parts.len() == 2, "midpoint must be free:<len> or zeros:<len>");
    let len = usize::from_str(parts[1]).expect("bad midpoint len");
    match parts[0].to_lowercase().as_str() {
        "free" => Midpoint::Free(len),
        "zeros" => Midpoint::Zeros(len),
        _ => panic!("midpoint kind must be free or zeros"),
    }
}
fn parse_layers(vecs: &[String]) -> Vec<Layer> {
    vecs.iter().map(|s| {
        let parts: Vec<&str> = s.split(':').collect();
        assert!(parts.len() == 2, "layer must be ZERO:SLOT");
        let zero = usize::from_str(parts[0]).expect("bad zero");
        let slot = usize::from_str(parts[1]).expect("bad slot");
        Layer { zero, slot }
    }).collect()
}
fn parse_u32_list(s: &str) -> Vec<u32> {
    if s.trim().is_empty() { vec![] } else { s.split(',').map(|d| u32::from_str(d.trim()).unwrap()).collect() }
}
fn parse_allowed_last_digits(base: u32, s: &str) -> Vec<u32> {
    let t = s.trim();
    if t.is_empty() { return vec![]; }
    if t.eq_ignore_ascii_case("auto") { return coprime_digits(base); }
    s.split(',').map(|d| u32::from_str(d.trim()).unwrap()).collect()
}
fn validate_allowed_last_digits(base: u32, allowed: &[u32]) {
    for &d in allowed { assert!(d < base, "allowed_last_digits contains digit {} >= base {}", d, base); }
}
fn parse_range(s: &str) -> (usize, usize) {
    let parts: Vec<&str> = s.split("..").collect();
    assert!(parts.len() == 2, "range must be a..b");
    let a = usize::from_str(parts[0]).unwrap();
    let b = usize::from_str(parts[1]).unwrap();
    assert!(a <= b);
    (a, b)
}

// ---------------- digit spec (spacing; optional mirror) ----------------

fn build_digit_spec(p: &Pattern) -> Vec<Option<Vec<u32>>> {
    let n = p.total_len();
    let mut spec: Vec<Option<Vec<u32>>> = vec![None; n];
    let base_digits: Vec<u32> = (0..p.base).collect();
    validate_allowed_last_digits(p.base, &p.constraints.allowed_last_digits);

    fn set_open_slots(
        spec: &mut [Option<Vec<u32>>],
        base_digits: &[u32],
        range: std::ops::Range<usize>,
        mirror: bool,
        allowed_last_digits: &[u32],
        total_len: usize,
    ) {
        use std::collections::HashSet;
        let last_set: Option<HashSet<u32>> =
            if allowed_last_digits.is_empty() { None } else { Some(allowed_last_digits.iter().cloned().collect()) };

        for idx in range {
            let mut allowed = base_digits.to_vec();
            if idx == 0 {
                allowed.retain(|&d| d != 0);
                if mirror {
                    if let Some(ref set) = last_set { allowed.retain(|d| set.contains(d)); }
                }
            }
            if idx + 1 == total_len && !allowed_last_digits.is_empty() {
                allowed = allowed_last_digits.to_vec();
            }
            spec[idx] = Some(allowed);
        }
    }

    let mid_len = match p.midpoint { Midpoint::Free(l) | Midpoint::Zeros(l) => l };
    let mid_start = (n - mid_len)/2;
    let mid_end = mid_start + mid_len;

    match p.midpoint {
        Midpoint::Zeros(_) => { for i in mid_start..mid_end { spec[i] = None; } }
        Midpoint::Free(_)  => { set_open_slots(&mut spec, &base_digits, mid_start..mid_end, p.mirror, &p.constraints.allowed_last_digits, n); }
    }

    let mut left_end = mid_start;
    let mut right_start = mid_end;

    for layer in p.layers.iter() {
        if layer.zero > 0 {
            let lz_start = left_end - layer.zero;
            for i in lz_start..left_end { spec[i] = None; }
            left_end = lz_start;

            let rz_end = right_start + layer.zero;
            for i in right_start..rz_end { spec[i] = None; }
            right_start = rz_end;
        }
        if layer.slot > 0 {
            let ls_start = left_end - layer.slot;
            let rs_end  = right_start + layer.slot;

            set_open_slots(&mut spec, &base_digits, ls_start..left_end, p.mirror, &p.constraints.allowed_last_digits, n);
            set_open_slots(&mut spec, &base_digits, right_start..rs_end, p.mirror, &p.constraints.allowed_last_digits, n);

            left_end = ls_start;
            right_start = rs_end;
        }
    }

    assert_eq!(left_end, 0, "left_end not at 0");
    assert_eq!(right_start, n, "right_start not at n");
    spec
}

// ---------------- number + primality ----------------

fn digits_to_biguint(digits: &[u32], base: u32) -> BigUint {
    let mut acc = BigUint::zero();
    let b = BigUint::from(base);
    for &d in digits { acc = &acc * &b + BigUint::from(d); }
    acc
}

// Optimization (A): Sample digits first, delay BigUint construction
fn sample_digits<R: Rng>(spec: &[Option<Vec<u32>>], mirror: bool, rng: &mut R) -> Vec<u32> {
    let n = spec.len();
    let mut digits: Vec<u32> = vec![0; n];
    for (i, allowed_opt) in spec.iter().enumerate() {
        match allowed_opt {
            None => digits[i] = 0,
            Some(allowed) => {
                let idx = rng.gen_range(0..allowed.len());
                digits[i] = allowed[idx];
            }
        }
    }
    if mirror {
        for i in 0..(n / 2) {
            let j = n - 1 - i;
            digits[j] = digits[i];
        }
    }
    digits
}

#[inline]
fn horner_mod(digits: &[u32], base: u32, m: u32) -> u32 {
    if m == 0 { return 0; }
    if m == 1 { return 0; }
    let mut acc: u64 = 0;
    let b = (base as u64) % (m as u64);
    for &d in digits {
        acc = ((acc * b) + (d as u64)) % (m as u64);
    }
    acc as u32
}

// ---------------- Optimization (C): Spec-aware model helpers ----------------

// Optimization (D): O(n) weight generation (replaces pow_mod_u32 calls)
fn weights_streaming(n: usize, base_mod: u32, m: u32) -> Vec<u32> {
    let mut w = vec![0u32; n];
    if m == 1 { return w; }
    let b = base_mod % m;
    let mut cur = 1u32 % m; // base^0
    // w[i] = base^(n-1-i)
    for i in (0..n).rev() {
        w[i] = cur;
        cur = ((cur as u64 * b as u64) % (m as u64)) as u32;
    }
    w
}

#[derive(Clone)]
struct DigitSpec {
    slots: Vec<Option<Vec<u32>>>,
}

fn build_spec(p: &Pattern) -> DigitSpec {
    DigitSpec { slots: build_digit_spec(p) }
}

fn residue_null_probability_with_spec(spec: &DigitSpec, base: u32, modm: u32) -> f64 {
    if modm < 2 { return 0.0; }
    let n = spec.slots.len();
    let base_mod = base % modm;
    let m = modm as usize;

    // Optimization (D): O(n) weight generation
    let weights = weights_streaming(n, base_mod, modm);

    // Optimization (E): Reuse DP buffers
    let mut dist = vec![0.0f64; m];
    let mut next = vec![0.0f64; m];
    dist[0] = 1.0;
    for (i, allowed_opt) in spec.slots.iter().enumerate() {
        if let Some(allowed) = allowed_opt {
            if allowed.is_empty() { continue; }
            let mul = weights[i] as usize;
            let w = 1.0 / (allowed.len() as f64);

            next.fill(0.0);
            for r in 0..m {
                let v = dist[r];
                if v == 0.0 { continue; }
                let base_r = r;
                for &d in allowed {
                    let delta = ((d as usize * mul) % m) as usize;
                    next[(base_r + delta) % m] += v * w;
                }
            }
            std::mem::swap(&mut dist, &mut next);
        }
    }
    dist[0]
}

fn union_null_probability_lcm_with_spec(spec: &DigitSpec, base: u32, track: &[u32]) -> (u32, f64) {
    let mods: Vec<u32> = track.iter().copied().filter(|&m| m >= 2).collect();
    if mods.is_empty() { return (1, 0.0); }
    let l = lcm_list(&mods);
    if l > LCM_CAP {
        let mut prod = 1.0;
        for &m in &mods { prod *= 1.0 - residue_null_probability_with_spec(spec, base, m); }
        return (0, (1.0 - prod).clamp(0.0, 1.0));
    }

    let n = spec.slots.len();
    let base_mod = base % l;
    let m = l as usize;

    // Optimization (D): O(n) weight generation
    let weights = weights_streaming(n, base_mod, l);

    // Optimization (E): Reuse DP buffers
    let mut dist = vec![0.0f64; m];
    let mut next = vec![0.0f64; m];
    dist[0] = 1.0;
    for (i, allowed_opt) in spec.slots.iter().enumerate() {
        if let Some(allowed) = allowed_opt {
            if allowed.is_empty() { continue; }
            let mul = weights[i] as usize;
            let w = 1.0 / (allowed.len() as f64);

            next.fill(0.0);
            for r in 0..m {
                let v = dist[r];
                if v == 0.0 { continue; }
                for &d in allowed {
                    let delta = ((d as usize * mul) % m) as usize;
                    next[(r + delta) % m] += v * w;
                }
            }
            std::mem::swap(&mut dist, &mut next);
        }
    }

    let mut is_bad = vec![false; m];
    for &pmod in &mods { for r in (0..m).step_by(pmod as usize) { is_bad[r] = true; } }
    let mut p_any = 0.0;
    for r in 0..m { if is_bad[r] { p_any += dist[r]; } }
    (l, p_any.clamp(0.0, 1.0))
}

// Optimization (F): Single-DP path to get all P0(p) when LCM(track) ≤ LCM_CAP
fn residue_null_vector_via_lcm_with_spec(
    spec: &DigitSpec,
    base: u32,
    track: &[u32],
) -> Option<(u32, Vec<(u32, f64)>)> {
    let mods: Vec<u32> = track.iter().copied().filter(|&m| m >= 2).collect();
    if mods.is_empty() { return Some((1, vec![])); }
    let l = lcm_list(&mods);
    if l == 0 || l > LCM_CAP { return None; }

    let n = spec.slots.len();
    let base_mod = base % l;
    let m = l as usize;

    let weights = weights_streaming(n, base_mod, l);

    let mut dist = vec![0.0f64; m];
    let mut next = vec![0.0f64; m];
    dist[0] = 1.0;
    for (i, allowed_opt) in spec.slots.iter().enumerate() {
        if let Some(allowed) = allowed_opt {
            if allowed.is_empty() { continue; }
            let mul = weights[i] as usize;
            let w = 1.0 / (allowed.len() as f64);

            next.fill(0.0);
            for r in 0..m {
                let v = dist[r];
                if v == 0.0 { continue; }
                for &d in allowed {
                    let delta = ((d as usize * mul) % m) as usize;
                    next[(r + delta) % m] += v * w;
                }
            }
            std::mem::swap(&mut dist, &mut next);
        }
    }

    let mut out = Vec::with_capacity(mods.len());
    for &p in &mods {
        let step = p as usize;
        let mut sum = 0.0;
        for r in (0..m).step_by(step) { sum += dist[r]; }
        out.push((p, sum.clamp(0.0, 1.0)));
    }
    Some((l, out))
}

fn expected_density_local_with_spec(p: &Pattern, spec: &DigitSpec, track: &[u32]) -> f64 {
    let l = p.total_len() as f64;
    let ln_x = (l - 0.5) * (p.base as f64).ln();
    if ln_x <= 0.0 { return 0.0; }
    if p.mirror && (p.total_len() % 2 == 0) { return 0.0; }
    let mut prod = 1.0;
    for &m in track { if m >= 2 { prod *= 1.0 - residue_null_probability_with_spec(spec, p.base, m); } }
    prod * (1.0 / ln_x)
}

fn expected_density_local_exact_with_spec(p: &Pattern, spec: &DigitSpec, track: &[u32]) -> f64 {
    let l = p.total_len() as f64;
    let ln_x = (l - 0.5) * (p.base as f64).ln();
    if ln_x <= 0.0 { return 0.0; }
    if p.mirror && (p.total_len() % 2 == 0) { return 0.0; }
    let (_, p_any) = union_null_probability_lcm_with_spec(spec, p.base, track);
    (1.0 - p_any) * (1.0 / ln_x)
}

fn sample_number<R: Rng>(p: &Pattern, spec: &[Option<Vec<u32>>], rng: &mut R) -> BigUint {
    let mut digits: Vec<u32> = vec![0; spec.len()];
    for (i, allowed_opt) in spec.iter().enumerate() {
        match allowed_opt {
            None => digits[i] = 0,
            Some(allowed) => { let idx = rng.gen_range(0..allowed.len()); digits[i] = allowed[idx]; }
        }
    }
    if p.mirror {
        let n = digits.len();
        for i in 0..(n/2) { digits[n-1-i] = digits[i]; }
    }
    digits_to_biguint(&digits, p.base)
}
fn modexp(mut base: BigUint, mut exp: BigUint, modu: &BigUint) -> BigUint {
    let mut result = BigUint::one();
    base %= modu;
    while exp > BigUint::zero() {
        if &exp & BigUint::one() == BigUint::one() { result = (result * &base) % modu; }
        exp >>= 1;
        base = (&base * &base) % modu;
    }
    result
}
fn miller_rabin_round(n: &BigUint, d: &BigUint, s: u32, a: &BigUint) -> bool {
    if a >= n { return true; }
    let one = BigUint::one();
    let n_minus_one = n - &one;
    let mut x = modexp(a.clone(), d.clone(), n);
    if x == one || x == n_minus_one { return true; }
    for _ in 1..s {
        x = (&x * &x) % n;
        if x == n_minus_one { return true; }
    }
    false
}
fn is_probable_prime(n: &BigUint) -> bool {
    let two = BigUint::from(2u32);
    let three = BigUint::from(3u32);
    if *n < two { return false; }
    if *n == two || *n == three { return true; }
    if (n % &two).is_zero() { return false; }

    let one = BigUint::one();
    let n_minus_one = n - &one;
    let mut d = n_minus_one.clone();
    let mut s = 0u32;
    while (&d & BigUint::from(1u32)).is_zero() { d >>= 1; s += 1; }

    let n_bits = n.bits();
    if n_bits <= 64 {
        let bases_u128: [u128; 7] = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
        for &a in &bases_u128 { if !miller_rabin_round(n, &d, s, &BigUint::from(a)) { return false; } }
        return true;
    }
    let bases: [u64; 12] = [2,3,5,7,11,13,17,19,23,29,31,37];
    for &a in &bases { if !miller_rabin_round(n, &d, s, &BigUint::from(a)) { return false; } }
    true
}

// ---------------- residue DP + tools ----------------

fn pow_mod_u32(a: u32, mut e: usize, m: u32) -> u32 {
    if m == 1 { return 0; }
    let mut r: u64 = 1 % (m as u64);
    let mut base = (a as u64) % (m as u64);
    while e > 0 {
        if (e & 1) == 1 { r = (r * base) % (m as u64); }
        base = (base * base) % (m as u64);
        e >>= 1;
    }
    r as u32
}
fn gcd_u32(mut a: u32, mut b: u32) -> u32 { while b != 0 { let t = a % b; a = b; b = t; } a }
fn coprime_digits(base: u32) -> Vec<u32> {
    (0..base).filter(|&d| gcd_u32(d, base) == 1).collect()
}
fn lcm_u32(a: u32, b: u32) -> u32 {
    if a == 0 || b == 0 { return 0; }
    let g = gcd_u32(a, b);
    if g == 0 { return 0; }
    let l = (a as u64 / g as u64).saturating_mul(b as u64);
    if l > u32::MAX as u64 { 0 } else { l as u32 }
}
fn lcm_list(mods: &[u32]) -> u32 { mods.iter().copied().filter(|&m| m>=2).fold(1u32, |acc,m| lcm_u32(acc,m)) }

// exact P(n ≡ 0 mod m)
fn residue_null_probability(p: &Pattern, modm: u32) -> f64 {
    if modm < 2 { return 0.0; }
    let spec = build_digit_spec(p);
    let n = spec.len();
    let base = p.base % modm;
    let m = modm as usize;

    let mut weights: Vec<u32> = vec![0; n];
    for i in 0..n { let exp = n - 1 - i; weights[i] = pow_mod_u32(base, exp, modm); }

    let mut dist = vec![0.0f64; m];
    dist[0] = 1.0;
    for (i, allowed_opt) in spec.iter().enumerate() {
        if let Some(allowed) = allowed_opt {
            if allowed.is_empty() { continue; }
            let mul = weights[i];
            let w = 1.0 / (allowed.len() as f64);

            let mut next = vec![0.0f64; m];
            for r in 0..m {
                let v = dist[r];
                if v == 0.0 { continue; }
                for &d in allowed {
                    let delta = ((d as u64 * mul as u64) % (modm as u64)) as usize;
                    next[(r + delta) % m] += v * w;
                }
            }
            dist = next;
        }
    }
    dist[0]
}

// union probability P(any p in track divides n) via DP modulo L, fallback when L too large
fn union_null_probability_lcm(p: &Pattern, track: &[u32]) -> (u32, f64) {
    let mods: Vec<u32> = track.iter().copied().filter(|&m| m >= 2).collect();
    if mods.is_empty() { return (1, 0.0); }
    let l = lcm_list(&mods);
    if l > LCM_CAP {
        let mut prod = 1.0;
        for &m in &mods { prod *= 1.0 - residue_null_probability(p, m); }
        return (0, (1.0 - prod).clamp(0.0, 1.0));
    }

    let spec = build_digit_spec(p);
    let n = spec.len();
    let base_mod = p.base % l;
    let m = l as usize;

    let mut weights: Vec<u32> = vec![0; n];
    for i in 0..n { let exp = n - 1 - i; weights[i] = pow_mod_u32(base_mod, exp, l); }

    let mut dist = vec![0.0f64; m];
    dist[0] = 1.0;
    for (i, allowed_opt) in spec.iter().enumerate() {
        if let Some(allowed) = allowed_opt {
            if allowed.is_empty() { continue; }
            let mul = weights[i];
            let w = 1.0 / (allowed.len() as f64);

            let mut next = vec![0.0f64; m];
            for r in 0..m {
                let v = dist[r];
                if v == 0.0 { continue; }
                for &d in allowed {
                    let delta = ((d as u64 * mul as u64) % (l as u64)) as usize;
                    next[(r + delta) % m] += v * w;
                }
            }
            dist = next;
        }
    }

    let mut is_bad = vec![false; m];
    for &pmod in &mods { for r in (0..m).step_by(pmod as usize) { is_bad[r] = true; } }
    let mut p_any = 0.0;
    for r in 0..m { if is_bad[r] { p_any += dist[r]; } }
    (l, p_any.clamp(0.0, 1.0))
}

// baselines
fn euler_phi(mut n: u32) -> u32 {
    if n <= 1 { return n; }
    let mut res = n;
    let mut p = 2u32;
    while (p as u64) * (p as u64) <= n as u64 {
        if n % p == 0 {
            while n % p == 0 { n /= p; }
            res = res / p * (p - 1);
        }
        p += 1;
    }
    if n > 1 { res = res / n * (n - 1); }
    res
}
fn expected_density_pnt_conditional(p: &Pattern) -> f64 {
    let l = p.total_len() as f64;
    let ln_x = (l - 0.5) * (p.base as f64).ln();
    if ln_x <= 0.0 { return 0.0; }
    if p.mirror && (p.total_len() % 2 == 0) { return 0.0; }

    let b = p.base;
    let phi_b = euler_phi(b) as usize;
    let s: Vec<u32> = if p.constraints.allowed_last_digits.is_empty() { (0..b).collect() } else { p.constraints.allowed_last_digits.clone() };
    let k = s.len().max(1);
    let a = s.iter().filter(|&&d| gcd_u32(d, b) == 1).count().max(1);
    (a as f64 * b as f64) / (k as f64 * phi_b as f64) * (1.0 / ln_x)
}
fn expected_density_local(p: &Pattern, track: &[u32]) -> f64 {
    let l = p.total_len() as f64;
    let ln_x = (l - 0.5) * (p.base as f64).ln();
    if ln_x <= 0.0 { return 0.0; }
    if p.mirror && (p.total_len() % 2 == 0) { return 0.0; }
    let mut prod = 1.0;
    for &m in track { if m >= 2 { prod *= 1.0 - residue_null_probability(p, m); } }
    prod * (1.0 / ln_x)
}
fn expected_density_local_exact(p: &Pattern, track: &[u32]) -> f64 {
    let l = p.total_len() as f64;
    let ln_x = (l - 0.5) * (p.base as f64).ln();
    if ln_x <= 0.0 { return 0.0; }
    if p.mirror && (p.total_len() % 2 == 0) { return 0.0; }
    let (_, p_any) = union_null_probability_lcm(p, track);
    (1.0 - p_any) * (1.0 / ln_x)
}
fn wilson_ci(primes: usize, n: usize, z: f64) -> (f64, f64) {
    let phat = primes as f64 / n as f64;
    let denom = 1.0 + z*z / n as f64;
    let center = phat + z*z / (2.0*n as f64);
    let margin = z * ((phat*(1.0-phat)/n as f64) + (z*z)/(4.0*n as f64)).sqrt();
    (((center - margin) / denom).max(0.0), ((center + margin) / denom).min(1.0))
}
fn format_pattern(p: &Pattern) -> String {
    let mid = match p.midpoint { Midpoint::Free(l) => format!("free:{l}"), Midpoint::Zeros(l) => format!("zeros:{l}") };
    let layers = p.layers.iter().map(|L| format!("{}:{}", L.zero, L.slot)).collect::<Vec<_>>().join(",");
    format!("mid={mid}|layers=[{layers}]|base={}", p.base)
}

// ---------------- auto‑track ----------------

#[derive(Deserialize, Clone, Default, Debug)]
struct AutoTrackCfg { enable: Option<bool>, mode: Option<String>, pmax: Option<u32>, k: Option<usize>, min_p0: Option<f64> }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AutoMode { PerCell, Global }
fn auto_mode_from_str(s: Option<&str>) -> AutoMode {
    match s.unwrap_or("per-cell").to_lowercase().as_str() {
        "global" => AutoMode::Global,
        _ => AutoMode::PerCell,
    }
}

fn sieve_primes_u32(n: u32) -> Vec<u32> {
    if n < 2 { return vec![]; }
    let mut is_prime = vec![true; (n as usize)+1];
    is_prime[0] = false; is_prime[1] = false;
    let mut p = 2usize;
    while p*p <= n as usize {
        if is_prime[p] { let mut j = p*p; while j <= n as usize { is_prime[j] = false; j += p; } }
        p += 1;
    }
    (2..=n).filter(|&x| is_prime[x as usize]).collect()
}

// per‑pattern selection
fn auto_select_moduli(pat: &Pattern, pmax: u32, k: usize, min_p0: f64, include_b1: bool) -> Vec<u32> {
    let mut cand = Vec::new();
    for m in sieve_primes_u32(pmax) { cand.push((m, residue_null_probability(pat, m))); }
    cand.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut out: Vec<u32> = cand.iter().filter(|(_,p0)| *p0 >= min_p0).take(k).map(|(m,_)| *m).collect();
    if out.len() < k { for (m,_) in cand.iter() { if out.len()>=k {break;} if !out.contains(m){ out.push(*m); } } }
    if include_b1 { let b1 = pat.base + 1; if !out.contains(&b1) { out.push(b1); } }
    out.sort_unstable(); out.dedup(); out
}

// grid‑global selection (aggregate sum of P0)
fn auto_select_moduli_global_grid(
    base: u32, mid_kind: &str, mid: (usize,usize), iz: (usize,usize),
    inner_slot: usize, outer: &[Layer], allowed: &[u32], mirror: bool,
    pmax: u32, k: usize, min_p0: f64
) -> Vec<u32> {
    let primes = sieve_primes_u32(pmax);
    let mut score: HashMap<u32, f64> = HashMap::new();
    let mut cells = 0.0;
    for mlen in mid.0..=mid.1 {
        for iz0 in iz.0..=iz.1 {
            let mut layers = vec![Layer { zero: iz0, slot: inner_slot }];
            layers.extend_from_slice(outer);
            let p = Pattern {
                base,
                midpoint: if mid_kind=="free" { Midpoint::Free(mlen) } else { Midpoint::Zeros(mlen) },
                layers,
                constraints: Constraints { allowed_last_digits: allowed.to_vec(), forbid_leading_zero: true },
                mirror,
            };
            for &q in &primes {
                let p0 = residue_null_probability(&p, q);
                *score.entry(q).or_insert(0.0) += p0;
            }
            cells += 1.0;
        }
    }
    let mut vec_sc: Vec<(u32, f64)> = score.into_iter().collect();
    vec_sc.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut out: Vec<u32> = vec_sc.into_iter().filter(|(_,s)| *s >= min_p0*cells).take(k).map(|(m,_)| m).collect();
    if mirror { let b1 = base + 1; if !out.contains(&b1) { out.push(b1); } }
    out.sort_unstable(); out.dedup(); out
}

// line‑global selection over the set of points in the lineout
fn auto_select_moduli_global_line(
    base: u32, mid_kind: &str, axis: &str, vals: &[usize], fixed_mid: usize, fixed_iz: usize,
    inner_slot: usize, outer: &[Layer], allowed: &[u32], mirror: bool,
    pmax: u32, k: usize, min_p0: f64
) -> Vec<u32> {
    let primes = sieve_primes_u32(pmax);
    let mut score: HashMap<u32, f64> = HashMap::new();
    let mut cells = 0.0;
    for &v in vals {
        let mut layers = vec![Layer { zero: if axis=="mid" { fixed_iz } else { v }, slot: inner_slot }];
        layers.extend_from_slice(outer);
        let p = Pattern {
            base,
            midpoint: if axis=="mid" {
                if mid_kind=="free" { Midpoint::Free(v) } else { Midpoint::Zeros(v) }
            } else {
                if mid_kind=="free" { Midpoint::Free(fixed_mid) } else { Midpoint::Zeros(fixed_mid) }
            },
            layers,
            constraints: Constraints { allowed_last_digits: allowed.to_vec(), forbid_leading_zero: true },
            mirror,
        };
        for &q in &primes {
            let p0 = residue_null_probability(&p, q);
            *score.entry(q).or_insert(0.0) += p0;
        }
        cells += 1.0;
    }
    let mut vec_sc: Vec<(u32, f64)> = score.into_iter().collect();
    vec_sc.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut out: Vec<u32> = vec_sc.into_iter().filter(|(_,s)| *s >= min_p0*cells).take(k).map(|(m,_)| m).collect();
    if mirror { let b1 = base + 1; if !out.contains(&b1) { out.push(b1); } }
    out.sort_unstable(); out.dedup(); out
}

// ---------------- reporting ----------------

#[derive(Serialize)]
struct SampleReport {
    pattern: String,
    base: u32,
    total_len: usize,
    mid_len: usize,
    inner_zero: usize,
    samples: usize,
    primes: usize,
    prime_density: f64,
    ci_lo: f64,
    ci_hi: f64,
    expected_density_pnt_cond: f64,
    expected_density_local: f64,
    expected_density_local_exact: f64,
    enrichment_vs_pnt_cond: f64,
    enrichment_vs_local: f64,
    enrichment_vs_local_exact: f64,
    elapsed_ms: u128,
    tracked_moduli: Vec<u32>,
    divisible_counts: Vec<usize>,
    model_p0: Vec<f64>,
}
fn write_csv_header(w: &mut dyn Write) {
    writeln!(
        w,
        "pattern,base,total_len,mid_len,inner_zero,samples,primes,prime_density,ci_lo,ci_hi,expected_density_pnt_cond,expected_density_local,expected_density_local_exact,enrichment_vs_pnt_cond,enrichment_vs_local,enrichment_vs_local_exact,elapsed_ms,tracked_moduli,divisible_counts,model_p0"
    ).unwrap();
}
fn write_csv_row(w: &mut dyn Write, r: &SampleReport) {
    writeln!(
        w,
        "{},{},{},{},{},{},{},{:.8},{:.8},{:.8},{:.8},{:.8},{:.8},{:.8},{:.8},{:.8},{},{:?},{:?},{:?}",
        r.pattern, r.base, r.total_len, r.mid_len, r.inner_zero, r.samples, r.primes, r.prime_density,
        r.ci_lo, r.ci_hi, r.expected_density_pnt_cond, r.expected_density_local, r.expected_density_local_exact,
        r.enrichment_vs_pnt_cond, r.enrichment_vs_local, r.enrichment_vs_local_exact, r.elapsed_ms,
        r.tracked_moduli, r.divisible_counts, r.model_p0
    ).unwrap();
}

fn write_model_csv_header(w: &mut dyn Write) {
    writeln!(
        w,
        "pattern,base,total_len,mid_len,inner_zero,expected_density_pnt_cond,expected_density_local,expected_density_local_exact,tracked_moduli,model_p0"
    ).unwrap();
}
fn write_model_csv_row(
    w: &mut dyn Write,
    pattern: &str, base: u32, total_len: usize, mid_len: usize, inner_zero: usize,
    expected_pnt: f64, expected_local: f64, expected_local_exact: f64,
    tracked: &[u32], model_p0: &[f64],
) {
    writeln!(
        w,
        "{},{},{},{},{},{:.8},{:.8},{:.8},{:?},{:?}",
        pattern, base, total_len, mid_len, inner_zero,
        expected_pnt, expected_local, expected_local_exact, tracked, model_p0
    ).unwrap();
}

// Ridge CSV
fn write_ridge_header(w: &mut dyn Write) {
    writeln!(
        w,
        "base,mid_len,iz_best,objective,p_any_exact,p_any_prod,expected_local_exact,expected_local,lcm_modulus,tracked_moduli"
    ).unwrap();
}
fn write_ridge_row(
    w: &mut dyn Write,
    base: u32, mid_len: usize, iz_best: usize, objective: &str,
    p_any_exact: f64, p_any_prod: f64, e_local_exact: f64, e_local: f64,
    lcm_modulus: u32, tracked_moduli: &[u32],
) {
    writeln!(
        w,
        "{},{},{},{},{:.8},{:.8},{:.8},{:.8},{},{:?}",
        base, mid_len, iz_best, objective, p_any_exact, p_any_prod, e_local_exact, e_local, lcm_modulus, tracked_moduli
    ).unwrap();
}

// ---------------- explanation ----------------

#[derive(Serialize, Deserialize, Clone)]
struct ExplainReport {
    pattern: String,
    base: u32,
    total_len: usize,
    mid_len: usize,
    inner_zero: usize,
    open_mask: Vec<u8>,
    allowed_lens: Vec<usize>,
    last_digit_set: Vec<u32>,
    weights: Vec<(u32, Vec<u32>)>,
    model_p0: Vec<(u32, f64)>,
    lcm_modulus: Option<u32>,
    union_p_any: Option<f64>,
    orders: Vec<(u32, Option<u32>)>,
    tracked_moduli: Vec<u32>,
}

fn multiplicative_order(base: u32, m: u32) -> Option<u32> {
    if gcd_u32(base, m) != 1 { return None; }
    let mut x = base as u64 % m as u64;
    let mut k: u32 = 1;
    while x != 1 {
        x = (x * base as u64) % m as u64;
        k = k.saturating_add(1);
        if k > m { return None; }
    }
    Some(k)
}

fn explain_for_pattern(p: &Pattern, track: &[u32]) -> ExplainReport {
    let spec = build_digit_spec(p);
    let n = spec.len();
    let open_mask: Vec<u8> = spec.iter().map(|s| if s.is_some() { 1 } else { 0 }).collect();
    let allowed_lens: Vec<usize> = spec.iter().map(|s| s.as_ref().map(|v| v.len()).unwrap_or(0)).collect();

    let spec_wrapped = build_spec(p);

    // Use LCM vector path for model_p0 when possible (Optimization F)
    let model_p0_vec: Vec<(u32, f64)> = if let Some((_l, pairs)) =
        residue_null_vector_via_lcm_with_spec(&spec_wrapped, p.base, track)
    {
        pairs
    } else {
        // Fallback: per-prime DP when LCM too large
        track.iter().filter(|&&m| m >= 2)
            .map(|&m| (m, residue_null_probability_with_spec(&spec_wrapped, p.base, m)))
            .collect()
    };

    let mut weights: Vec<(u32, Vec<u32>)> = Vec::new();
    let mut orders: Vec<(u32, Option<u32>)> = Vec::new();
    for &m in track {
        if m >= 2 {
            let base_mod = p.base % m;
            // Use streaming weights (Optimization D) instead of pow_mod_u32
            let wvec = weights_streaming(n, base_mod, m);
            weights.push((m, wvec));
            orders.push((m, multiplicative_order(p.base % m, m)));
        }
    }

    let model_p0: Vec<(u32, f64)> = model_p0_vec;

    let (l, p_any) = union_null_probability_lcm(p, track);
    let l_opt = if l == 0 { None } else { Some(l) };
    let p_opt = if track.is_empty() { None } else { Some(p_any) };

    ExplainReport {
        pattern: format_pattern(p),
        base: p.base,
        total_len: p.total_len(),
        mid_len: match p.midpoint { Midpoint::Free(l) | Midpoint::Zeros(l) => l },
        inner_zero: p.layers.first().map(|L| L.zero).unwrap_or(0),
        open_mask,
        allowed_lens,
        last_digit_set: p.constraints.allowed_last_digits.clone(),
        weights,
        model_p0,
        lcm_modulus: l_opt,
        union_p_any: p_opt,
        orders,
        tracked_moduli: track.to_vec(),
    }
}

// ---------------- core sampling ----------------

fn do_sample(p: &Pattern, samples: usize, seed: u64, parallel: bool, track: &[u32], pre_sieve: bool) -> SampleReport {
    let total_len = p.total_len();
    let mid_len = match p.midpoint { Midpoint::Free(l) | Midpoint::Zeros(l) => l };
    let inner_zero = p.layers.first().map(|L| L.zero).unwrap_or(0);
    let spec = build_digit_spec(p);
    let spec_wrapped = build_spec(p);

    // Use LCM vector path when possible (Optimization F)
    let model_p0: Vec<f64> = if let Some((_l, pairs)) =
        residue_null_vector_via_lcm_with_spec(&spec_wrapped, p.base, track)
    {
        track.iter().map(|&m|
            if m < 2 { 0.0 }
            else { pairs.iter().find(|(pm,_)| *pm==m).map(|(_,q)| *q).unwrap_or(0.0) }
        ).collect()
    } else {
        // Fallback: per-prime DP when LCM too large
        track.iter().map(|&m| if m>=2 {
            residue_null_probability_with_spec(&spec_wrapped, p.base, m)
        } else { 0.0 }).collect()
    };
    let expected_pnt = expected_density_pnt_conditional(p);
    let expected_local = expected_density_local(p, track);
    let expected_local_exact = expected_density_local_exact(p, track);

    let start = std::time::Instant::now();
    let (primes, counts): (usize, Vec<usize>) = if parallel {
        (0..samples).into_par_iter().map(|i| {
            let mut rng = StdRng::seed_from_u64(seed ^ (i as u64).wrapping_mul(0x9E3779B97F4A7C15));
            let digits = sample_digits(&spec, p.mirror, &mut rng);

            // Compute residues first (no BigUint yet)
            let mut div = vec![0usize; track.len()];
            let mut blocked = false;
            for (j, &m) in track.iter().enumerate() {
                if m >= 2 {
                    if horner_mod(&digits, p.base, m) == 0 {
                        div[j] = 1;
                        if pre_sieve { blocked = true; }
                    }
                }
            }

            // Only construct BigUint if passes pre-sieve
            let is_prime = if pre_sieve && blocked {
                0usize
            } else {
                let n = digits_to_biguint(&digits, p.base);
                is_probable_prime(&n) as usize
            };
            (is_prime, div)
        }).reduce(|| (0usize, vec![0usize; track.len()]), |(p1, mut c1), (p2, c2)| {
            for (i, v) in c2.iter().enumerate() { c1[i] += *v; }
            (p1 + p2, c1)
        })
    } else {
        let mut primes = 0usize;
        let mut counts = vec![0usize; track.len()];
        for i in 0..samples {
            let mut rng = StdRng::seed_from_u64(seed ^ (i as u64).wrapping_mul(0x9E3779B97F4A7C15));
            let digits = sample_digits(&spec, p.mirror, &mut rng);

            // Compute residues first (no BigUint yet)
            let mut blocked = false;
            for (j, &m) in track.iter().enumerate() {
                if m >= 2 {
                    if horner_mod(&digits, p.base, m) == 0 {
                        counts[j] += 1;
                        if pre_sieve { blocked = true; }
                    }
                }
            }

            // Only construct BigUint if passes pre-sieve
            if !(pre_sieve && blocked) {
                let n = digits_to_biguint(&digits, p.base);
                if is_probable_prime(&n) { primes += 1; }
            }
        }
        (primes, counts)
    };
    let elapsed = start.elapsed().as_millis();

    let density = primes as f64 / samples as f64;
    let (ci_lo, ci_hi) = wilson_ci(primes, samples, 1.96);
    let enrichment_vs_pnt = if expected_pnt > 0.0 { density / expected_pnt } else if primes == 0 { 0.0 } else { f64::INFINITY };
    let enrichment_vs_local = if expected_local > 0.0 { density / expected_local } else if primes == 0 { 0.0 } else { f64::INFINITY };
    let enrichment_vs_local_exact = if expected_local_exact > 0.0 { density / expected_local_exact } else if primes == 0 { 0.0 } else { f64::INFINITY };

    SampleReport {
        pattern: format_pattern(p),
        base: p.base,
        total_len,
        mid_len,
        inner_zero,
        samples,
        primes,
        prime_density: density,
        ci_lo, ci_hi,
        expected_density_pnt_cond: expected_pnt,
        expected_density_local: expected_local,
        expected_density_local_exact: expected_local_exact,
        enrichment_vs_pnt_cond: enrichment_vs_pnt,
        enrichment_vs_local,
        enrichment_vs_local_exact,
        elapsed_ms: elapsed,
        tracked_moduli: track.to_vec(),
        divisible_counts: counts,
        model_p0,
    }
}

// ---------------- TOML runner types ----------------

#[derive(Deserialize)]
struct RunConfig {
    base: Option<u32>,
    seed: Option<u64>,
    jobs: Vec<Job>,
}
#[derive(Deserialize)]
#[serde(tag = "kind")]
enum Job {
    #[serde(rename = "sample")]
    Sample {
        base: Option<u32>,
        seed: Option<u64>,
        midpoint: String,
        layers: Vec<String>,
        samples: usize,
        allowed_last_digits: String,
        out_json: Option<String>,
        out_csv: Option<String>,
        parallel: Option<bool>,
        mirror: Option<bool>,
        track_moduli: String,
        pre_sieve: Option<bool>,
        auto_track: Option<AutoTrackCfg>,
    },
    #[serde(rename = "grid")]
    Grid {
        base: Option<u32>,
        seed: Option<u64>,
        mid_kind: String,
        mid_len_range: String,
        inner_zero_range: String,
        inner_slot: usize,
        outer_layers: Vec<String>,
        samples: usize,
        allowed_last_digits: String,
        out_csv: String,
        parallel: Option<bool>,
        mirror: Option<bool>,
        track_moduli: String,
        pre_sieve: Option<bool>,
        auto_track: Option<AutoTrackCfg>,
    },
    #[serde(rename = "model-only")]
    ModelOnly {
        base: Option<u32>,
        mid_kind: String,
        mid_len_range: String,
        inner_zero_range: String,
        inner_slot: usize,
        outer_layers: Vec<String>,
        allowed_last_digits: String,
        out_csv: String,
        mirror: Option<bool>,
        track_moduli: String,
        auto_track: Option<AutoTrackCfg>,
    },
    #[serde(rename = "explain-grid")]
    ExplainGrid {
        base: Option<u32>,
        mid_kind: String,
        mid_len_range: String,
        inner_zero_range: String,
        inner_slot: usize,
        outer_layers: Vec<String>,
        allowed_last_digits: String,
        out_json: String,
        mirror: Option<bool>,
        track_moduli: String,
        auto_track: Option<AutoTrackCfg>,
    },
    #[serde(rename = "lineout")]
    Lineout {
        base: Option<u32>,
        seed: Option<u64>,
        axis: String,
        mid_kind: String,
        range: String,
        inner_zero: Option<usize>,
        mid_len: Option<usize>,
        inner_slot: usize,
        outer_layers: Vec<String>,
        samples: usize,
        allowed_last_digits: String,
        out_csv: String,
        parallel: Option<bool>,
        mirror: Option<bool>,
        track_moduli: String,
        pre_sieve: Option<bool>,
        auto_track: Option<AutoTrackCfg>,
        model_only: Option<bool>,
    },
    #[serde(rename = "ridge")]
    Ridge {
        base: Option<u32>,
        mid_kind: String,
        mid_len_range: String,
        inner_zero_range: String,
        inner_slot: usize,
        outer_layers: Vec<String>,
        allowed_last_digits: String,
        out_csv: String,
        mirror: Option<bool>,
        track_moduli: String,
        auto_track: Option<AutoTrackCfg>,
        objective: Option<String>,
    },
}
fn read_toml(path: &str) -> RunConfig {
    let mut f = BufReader::new(File::open(path).expect("open config"));
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    toml::from_str::<RunConfig>(&s).expect("parse toml")
}

// ---------------- main ----------------

fn main() {
    let cli = Cli::parse();
    match cli.command {
        // Sample
        Commands::Sample { midpoint, layers, samples, allowed_last_digits, out_json, out_csv, parallel, mirror, track_moduli, auto_track, auto_pmax, auto_k, auto_min_p0, pre_sieve } => {
            let p = Pattern {
                base: cli.base,
                midpoint: parse_midpoint(&midpoint),
                layers: parse_layers(&layers),
                constraints: Constraints { allowed_last_digits: parse_allowed_last_digits(cli.base, &allowed_last_digits), forbid_leading_zero: true },
                mirror,
            };
            let mut track = if auto_track {
                auto_select_moduli(&p, auto_pmax, auto_k, auto_min_p0, mirror)
            } else {
                let mut t = parse_u32_list(&track_moduli);
                if mirror { let m = p.base + 1; if !t.contains(&m) { t.push(m); } }
                t
            };
            track.sort_unstable(); track.dedup();

            eprintln!(
                "[density-explorer] base={} allowed_last_digits={:?} auto_track={} track={:?}  LCM={}{}",
                cli.base,
                p.constraints.allowed_last_digits,
                auto_track,
                track,
                lcm_list(&track),
                if lcm_list(&track) > LCM_CAP { "  (over cap → prod fallback)" } else { "" }
            );

            let report = do_sample(&p, samples, cli.seed, parallel, &track, pre_sieve);
            if let Some(path) = out_json {
                let f = File::create(path).expect("create json");
                serde_json::to_writer_pretty(f, &report).expect("write json");
            } else {
                println!("{}", serde_json::to_string_pretty(&report).unwrap());
            }
            if let Some(path) = out_csv {
                let f = File::create(path).expect("create csv");
                let mut w = BufWriter::new(f);
                write_csv_header(&mut w);
                write_csv_row(&mut w, &report);
                w.flush().unwrap();
            }
        }

        // Grid
        Commands::Grid { mid_kind, mid_len_range, inner_zero_range, inner_slot, outer_layers, samples, allowed_last_digits, out_csv, parallel, mirror, track_moduli, auto_track, auto_pmax, auto_k, auto_min_p0, auto_mode, pre_sieve } => {
            let (mid_a, mid_b) = parse_range(&mid_len_range);
            let (iz_a, iz_b) = parse_range(&inner_zero_range);
            let kind = mid_kind.to_lowercase(); assert!(kind == "free" || kind == "zeros");
            let outer = parse_layers(&outer_layers);
            let explicit_track = parse_u32_list(&track_moduli);
            let at_mode = auto_mode_from_str(Some(&auto_mode));

            let global_track = if auto_track && at_mode==AutoMode::Global {
                Some(auto_select_moduli_global_grid(
                    cli.base, &kind, (mid_a,mid_b), (iz_a,iz_b),
                    inner_slot, &outer, &parse_allowed_last_digits(cli.base, &allowed_last_digits),
                    mirror, auto_pmax, auto_k, auto_min_p0
                ))
            } else { None };

            if let Some(ref gt) = global_track {
                eprintln!(
                    "[density-explorer] base={} allowed_last_digits={:?} auto_track(mode={:?}): {:?}  LCM={}{}",
                    cli.base,
                    parse_allowed_last_digits(cli.base, &allowed_last_digits),
                    at_mode,
                    gt,
                    lcm_list(gt),
                    if lcm_list(gt) > LCM_CAP { "  (over cap → prod fallback)" } else { "" }
                );
            } else if auto_track {
                eprintln!(
                    "[density-explorer] base={} allowed_last_digits={:?} auto_track(mode={:?}): tracks will vary per cell",
                    cli.base,
                    parse_allowed_last_digits(cli.base, &allowed_last_digits),
                    at_mode
                );
            } else {
                eprintln!(
                    "[density-explorer] base={} allowed_last_digits={:?} auto_track=false track={:?}  LCM={}{}",
                    cli.base,
                    parse_allowed_last_digits(cli.base, &allowed_last_digits),
                    explicit_track,
                    lcm_list(&explicit_track),
                    if lcm_list(&explicit_track) > LCM_CAP { "  (over cap → prod fallback)" } else { "" }
                );
            }

            let f = File::create(out_csv).expect("create csv");
            let mut w = BufWriter::new(f); write_csv_header(&mut w);

            let combos: Vec<(usize, usize)> = (mid_a..=mid_b).flat_map(|m| (iz_a..=iz_b).map(move |z| (m, z))).collect();

            let results: Vec<SampleReport> = if parallel {
                combos.par_iter().map(|(m, z)| {
                    let mut layers = vec![Layer { zero: *z, slot: inner_slot }]; layers.extend_from_slice(&outer);
                    let p = Pattern {
                        base: cli.base,
                        midpoint: if kind=="free" { Midpoint::Free(*m) } else { Midpoint::Zeros(*m) },
                        layers,
                        constraints: Constraints { allowed_last_digits: parse_allowed_last_digits(cli.base, &allowed_last_digits), forbid_leading_zero: true },
                        mirror,
                    };
                    let mut track = if auto_track {
                        if let Some(gt) = &global_track {
                            gt.clone()
                        } else {
                            auto_select_moduli(&p, auto_pmax, auto_k, auto_min_p0, mirror)
                        }
                    } else {
                        let mut t = explicit_track.clone();
                        if mirror { let m = p.base + 1; if !t.contains(&m) { t.push(m); } }
                        t
                    };
                    track.sort_unstable(); track.dedup();
                    do_sample(&p, samples, cli.seed ^ ((*m as u64) << 32) ^ (*z as u64), true, &track, pre_sieve)
                }).collect()
            } else {
                combos.iter().map(|(m, z)| {
                    let mut layers = vec![Layer { zero: *z, slot: inner_slot }]; layers.extend_from_slice(&outer);
                    let p = Pattern {
                        base: cli.base,
                        midpoint: if kind=="free" { Midpoint::Free(*m) } else { Midpoint::Zeros(*m) },
                        layers,
                        constraints: Constraints { allowed_last_digits: parse_allowed_last_digits(cli.base, &allowed_last_digits), forbid_leading_zero: true },
                        mirror,
                    };
                    let mut track = if auto_track {
                        if let Some(gt) = &global_track {
                            gt.clone()
                        } else {
                            auto_select_moduli(&p, auto_pmax, auto_k, auto_min_p0, mirror)
                        }
                    } else {
                        let mut t = explicit_track.clone();
                        if mirror { let m = p.base + 1; if !t.contains(&m) { t.push(m); } }
                        t
                    };
                    track.sort_unstable(); track.dedup();
                    do_sample(&p, samples, cli.seed ^ ((*m as u64) << 32) ^ (*z as u64), false, &track, pre_sieve)
                }).collect()
            };

            for r in &results { write_csv_row(&mut w, r); }
            w.flush().unwrap();
        }

        // Model-only
        Commands::ModelOnly { mid_kind, mid_len_range, inner_zero_range, inner_slot, outer_layers, allowed_last_digits, out_csv, mirror, track_moduli, auto_track, auto_pmax, auto_k, auto_min_p0, auto_mode } => {
            let (mid_a, mid_b) = parse_range(&mid_len_range);
            let (iz_a, iz_b) = parse_range(&inner_zero_range);
            let kind = mid_kind.to_lowercase(); assert!(kind == "free" || kind == "zeros");
            let outer = parse_layers(&outer_layers);
            let explicit_track = parse_u32_list(&track_moduli);
            let at_mode = auto_mode_from_str(Some(&auto_mode));

            let global_track = if auto_track && at_mode==AutoMode::Global {
                Some(auto_select_moduli_global_grid(
                    cli.base, &kind, (mid_a,mid_b), (iz_a,iz_b),
                    inner_slot, &outer, &parse_allowed_last_digits(cli.base, &allowed_last_digits),
                    mirror, auto_pmax, auto_k, auto_min_p0
                ))
            } else { None };

            if let Some(ref gt) = global_track {
                eprintln!(
                    "[density-explorer] base={} allowed_last_digits={:?} auto_track(mode={:?}): {:?}  LCM={}{}",
                    cli.base,
                    parse_allowed_last_digits(cli.base, &allowed_last_digits),
                    at_mode,
                    gt,
                    lcm_list(gt),
                    if lcm_list(gt) > LCM_CAP { "  (over cap → prod fallback)" } else { "" }
                );
            } else if auto_track {
                eprintln!(
                    "[density-explorer] base={} allowed_last_digits={:?} auto_track(mode={:?}): tracks will vary per cell",
                    cli.base,
                    parse_allowed_last_digits(cli.base, &allowed_last_digits),
                    at_mode
                );
            } else {
                eprintln!(
                    "[density-explorer] base={} allowed_last_digits={:?} auto_track=false track={:?}  LCM={}{}",
                    cli.base,
                    parse_allowed_last_digits(cli.base, &allowed_last_digits),
                    explicit_track,
                    lcm_list(&explicit_track),
                    if lcm_list(&explicit_track) > LCM_CAP { "  (over cap → prod fallback)" } else { "" }
                );
            }

            let f = File::create(out_csv).expect("create csv");
            let mut w = BufWriter::new(f); write_model_csv_header(&mut w);

            for mid_len in mid_a..=mid_b {
                for iz in iz_a..=iz_b {
                    let mut layers = vec![Layer { zero: iz, slot: inner_slot }]; layers.extend_from_slice(&outer);
                    let p = Pattern {
                        base: cli.base,
                        midpoint: if kind=="free" { Midpoint::Free(mid_len) } else { Midpoint::Zeros(mid_len) },
                        layers,
                        constraints: Constraints { allowed_last_digits: parse_allowed_last_digits(cli.base, &allowed_last_digits), forbid_leading_zero: true },
                        mirror,
                    };
                    let mut track = if auto_track {
                        if let Some(gt) = &global_track {
                            gt.clone()
                        } else {
                            auto_select_moduli(&p, auto_pmax, auto_k, auto_min_p0, mirror)
                        }
                    } else {
                        let mut t = explicit_track.clone();
                        if mirror { let m = p.base + 1; if !t.contains(&m) { t.push(m); } }
                        t
                    };
                    track.sort_unstable(); track.dedup();

                    // Use LCM vector path when possible (Optimization F)
                    let spec_wrapped = build_spec(&p);
                    let model_p0: Vec<f64> = if let Some((_l, pairs)) =
                        residue_null_vector_via_lcm_with_spec(&spec_wrapped, p.base, &track)
                    {
                        track.iter().map(|&m|
                            if m < 2 { 0.0 }
                            else { pairs.iter().find(|(pm,_)| *pm==m).map(|(_,q)| *q).unwrap_or(0.0) }
                        ).collect()
                    } else {
                        // Fallback: per-prime DP when LCM too large
                        track.iter().map(|&m| if m>=2 {
                            residue_null_probability_with_spec(&spec_wrapped, p.base, m)
                        } else { 0.0 }).collect()
                    };
                    let expected_pnt = expected_density_pnt_conditional(&p);
                    let expected_local = expected_density_local(&p, &track);
                    let expected_local_exact = expected_density_local_exact(&p, &track);

                    write_model_csv_row(&mut w, &format_pattern(&p), p.base, p.total_len(), mid_len, iz, expected_pnt, expected_local, expected_local_exact, &track, &model_p0);
                }
            }
            w.flush().unwrap();
        }

        // Explain
        Commands::Explain { midpoint, layers, allowed_last_digits, mirror, track_moduli, auto_track, auto_pmax, auto_k, auto_min_p0, out_json } => {
            let p = Pattern {
                base: cli.base,
                midpoint: parse_midpoint(&midpoint),
                layers: parse_layers(&layers),
                constraints: Constraints { allowed_last_digits: parse_allowed_last_digits(cli.base, &allowed_last_digits), forbid_leading_zero: true },
                mirror,
            };
            let mut track = if auto_track {
                auto_select_moduli(&p, auto_pmax, auto_k, auto_min_p0, mirror)
            } else {
                let mut t = parse_u32_list(&track_moduli);
                if mirror { let m = p.base + 1; if !t.contains(&m) { t.push(m); } }
                t
            };
            track.sort_unstable(); track.dedup();

            eprintln!(
                "[density-explorer] base={} allowed_last_digits={:?} auto_track={} track={:?}  LCM={}{}",
                cli.base,
                p.constraints.allowed_last_digits,
                auto_track,
                track,
                lcm_list(&track),
                if lcm_list(&track) > LCM_CAP { "  (over cap → prod fallback)" } else { "" }
            );

            let report = explain_for_pattern(&p, &track);
            if let Some(path) = out_json {
                let f = File::create(path).expect("create json");
                serde_json::to_writer_pretty(f, &report).expect("write json");
            } else {
                println!("{}", serde_json::to_string_pretty(&report).unwrap());
            }
        }

        // Explain-grid
        Commands::ExplainGrid { mid_kind, mid_len_range, inner_zero_range, inner_slot, outer_layers, allowed_last_digits, out_json, mirror, track_moduli, auto_track, auto_pmax, auto_k, auto_min_p0, auto_mode } => {
            let (mid_a, mid_b) = parse_range(&mid_len_range);
            let (iz_a, iz_b) = parse_range(&inner_zero_range);
            let kind = mid_kind.to_lowercase(); assert!(kind == "free" || kind == "zeros");
            let outer = parse_layers(&outer_layers);
            let explicit_track = parse_u32_list(&track_moduli);
            let at_mode = auto_mode_from_str(Some(&auto_mode));

            let global_track = if auto_track && at_mode==AutoMode::Global {
                Some(auto_select_moduli_global_grid(
                    cli.base, &kind, (mid_a,mid_b), (iz_a,iz_b),
                    inner_slot, &outer, &parse_allowed_last_digits(cli.base, &allowed_last_digits),
                    mirror, auto_pmax, auto_k, auto_min_p0
                ))
            } else { None };

            if let Some(ref gt) = global_track {
                eprintln!(
                    "[density-explorer] base={} allowed_last_digits={:?} auto_track(mode={:?}): {:?}  LCM={}{}",
                    cli.base,
                    parse_allowed_last_digits(cli.base, &allowed_last_digits),
                    at_mode,
                    gt,
                    lcm_list(gt),
                    if lcm_list(gt) > LCM_CAP { "  (over cap → prod fallback)" } else { "" }
                );
            } else if auto_track {
                eprintln!(
                    "[density-explorer] base={} allowed_last_digits={:?} auto_track(mode={:?}): tracks will vary per cell",
                    cli.base,
                    parse_allowed_last_digits(cli.base, &allowed_last_digits),
                    at_mode
                );
            } else {
                eprintln!(
                    "[density-explorer] base={} allowed_last_digits={:?} auto_track=false track={:?}  LCM={}{}",
                    cli.base,
                    parse_allowed_last_digits(cli.base, &allowed_last_digits),
                    explicit_track,
                    lcm_list(&explicit_track),
                    if lcm_list(&explicit_track) > LCM_CAP { "  (over cap → prod fallback)" } else { "" }
                );
            }

            let mut entries: Vec<ExplainReport> = Vec::new();
            for mid_len in mid_a..=mid_b {
                for iz in iz_a..=iz_b {
                    let mut layers = vec![Layer { zero: iz, slot: inner_slot }]; layers.extend_from_slice(&outer);
                    let p = Pattern {
                        base: cli.base,
                        midpoint: if kind=="free" { Midpoint::Free(mid_len) } else { Midpoint::Zeros(mid_len) },
                        layers,
                        constraints: Constraints { allowed_last_digits: parse_allowed_last_digits(cli.base, &allowed_last_digits), forbid_leading_zero: true },
                        mirror,
                    };
                    let mut track = if auto_track {
                        if let Some(gt) = &global_track {
                            gt.clone()
                        } else {
                            auto_select_moduli(&p, auto_pmax, auto_k, auto_min_p0, mirror)
                        }
                    } else {
                        let mut t = explicit_track.clone();
                        if mirror { let m = p.base + 1; if !t.contains(&m) { t.push(m); } }
                        t
                    };
                    track.sort_unstable(); track.dedup();
                    entries.push(explain_for_pattern(&p, &track));
                }
            }
            let f = File::create(out_json).expect("create json");
            serde_json::to_writer_pretty(f, &entries).expect("write json");
        }

        // Lineout
        Commands::Lineout { axis, mid_kind, range, inner_zero, mid_len, inner_slot, outer_layers, samples, allowed_last_digits, out_csv, parallel, mirror, track_moduli, auto_track, auto_pmax, auto_k, auto_min_p0, auto_mode, pre_sieve, model_only } => {
            let (a, b) = parse_range(&range);
            let kind = mid_kind.to_lowercase(); assert!(kind == "free" || kind == "zeros");
            let outer = parse_layers(&outer_layers);

            let explicit_track = parse_u32_list(&track_moduli);
            let fixed_iz = inner_zero;
            let fixed_mid = mid_len;
            let at_mode = auto_mode_from_str(Some(&auto_mode));

            let vals: Vec<usize> = (a..=b).collect();
            let global_track = if auto_track && at_mode==AutoMode::Global {
                Some(auto_select_moduli_global_line(
                    cli.base, &kind, &axis, &vals,
                    fixed_mid, fixed_iz, inner_slot, &outer,
                    &parse_allowed_last_digits(cli.base, &allowed_last_digits), mirror,
                    auto_pmax, auto_k, auto_min_p0
                ))
            } else { None };

            let f = File::create(out_csv).expect("create csv");
            let mut w = BufWriter::new(f); write_csv_header(&mut w);

            let results: Vec<SampleReport> = if parallel {
                vals.par_iter().map(|&v| {
                    let mut layers = vec![Layer {
                        zero: if axis=="mid" { fixed_iz } else { v },
                        slot: inner_slot
                    }];
                    layers.extend_from_slice(&outer);
                    let p = Pattern {
                        base: cli.base,
                        midpoint: if axis=="mid" {
                            if kind=="free" { Midpoint::Free(v) } else { Midpoint::Zeros(v) }
                        } else {
                            if kind=="free" { Midpoint::Free(fixed_mid) } else { Midpoint::Zeros(fixed_mid) }
                        },
                        layers,
                        constraints: Constraints { allowed_last_digits: parse_allowed_last_digits(cli.base, &allowed_last_digits), forbid_leading_zero: true },
                        mirror,
                    };
                    let mut track = if auto_track {
                        if let Some(gt) = &global_track { gt.clone() }
                        else { auto_select_moduli(&p, auto_pmax, auto_k, auto_min_p0, mirror) }
                    } else {
                        let mut t = explicit_track.clone();
                        if mirror { let m = p.base + 1; if !t.contains(&m) { t.push(m); } }
                        t
                    };
                    track.sort_unstable(); track.dedup();

                    if model_only {
                        // Use LCM vector path when possible (Optimization F)
                        let spec_wrapped = build_spec(&p);
                        let model_p0: Vec<f64> = if let Some((_l, pairs)) =
                            residue_null_vector_via_lcm_with_spec(&spec_wrapped, p.base, &track)
                        {
                            track.iter().map(|&m|
                                if m < 2 { 0.0 }
                                else { pairs.iter().find(|(pm,_)| *pm==m).map(|(_,q)| *q).unwrap_or(0.0) }
                            ).collect()
                        } else {
                            // Fallback: per-prime DP when LCM too large
                            track.iter().map(|&m| if m>=2 {
                                residue_null_probability_with_spec(&spec_wrapped, p.base, m)
                            } else { 0.0 }).collect()
                        };
                        let expected_pnt = expected_density_pnt_conditional(&p);
                        let expected_local = expected_density_local(&p, &track);
                        let expected_local_exact = expected_density_local_exact(&p, &track);
                        SampleReport{
                            pattern: format_pattern(&p),
                            base: p.base,
                            total_len: p.total_len(),
                            mid_len: match p.midpoint { Midpoint::Free(l)|Midpoint::Zeros(l)=>l },
                            inner_zero: p.layers.first().map(|L| L.zero).unwrap_or(0),
                            samples: 0, primes: 0,
                            prime_density: 0.0, ci_lo: 0.0, ci_hi: 0.0,
                            expected_density_pnt_cond: expected_pnt,
                            expected_density_local: expected_local,
                            expected_density_local_exact: expected_local_exact,
                            enrichment_vs_pnt_cond: 0.0,
                            enrichment_vs_local: 0.0,
                            enrichment_vs_local_exact: 0.0,
                            elapsed_ms: 0,
                            tracked_moduli: track,
                            divisible_counts: vec![],
                            model_p0,
                        }
                    } else {
                        do_sample(&p, samples, cli.seed ^ (v as u64), true, &track, pre_sieve)
                    }
                }).collect()
            } else {
                vals.iter().map(|&v| {
                    let mut layers = vec![Layer {
                        zero: if axis=="mid" { fixed_iz } else { v },
                        slot: inner_slot
                    }];
                    layers.extend_from_slice(&outer);
                    let p = Pattern {
                        base: cli.base,
                        midpoint: if axis=="mid" {
                            if kind=="free" { Midpoint::Free(v) } else { Midpoint::Zeros(v) }
                        } else {
                            if kind=="free" { Midpoint::Free(fixed_mid) } else { Midpoint::Zeros(fixed_mid) }
                        },
                        layers,
                        constraints: Constraints { allowed_last_digits: parse_allowed_last_digits(cli.base, &allowed_last_digits), forbid_leading_zero: true },
                        mirror,
                    };
                    let mut track = if auto_track {
                        if let Some(gt) = &global_track { gt.clone() }
                        else { auto_select_moduli(&p, auto_pmax, auto_k, auto_min_p0, mirror) }
                    } else {
                        let mut t = explicit_track.clone();
                        if mirror { let m = p.base + 1; if !t.contains(&m) { t.push(m); } }
                        t
                    };
                    track.sort_unstable(); track.dedup();

                    if model_only {
                        // Use LCM vector path when possible (Optimization F)
                        let spec_wrapped = build_spec(&p);
                        let model_p0: Vec<f64> = if let Some((_l, pairs)) =
                            residue_null_vector_via_lcm_with_spec(&spec_wrapped, p.base, &track)
                        {
                            track.iter().map(|&m|
                                if m < 2 { 0.0 }
                                else { pairs.iter().find(|(pm,_)| *pm==m).map(|(_,q)| *q).unwrap_or(0.0) }
                            ).collect()
                        } else {
                            // Fallback: per-prime DP when LCM too large
                            track.iter().map(|&m| if m>=2 {
                                residue_null_probability_with_spec(&spec_wrapped, p.base, m)
                            } else { 0.0 }).collect()
                        };
                        let expected_pnt = expected_density_pnt_conditional(&p);
                        let expected_local = expected_density_local(&p, &track);
                        let expected_local_exact = expected_density_local_exact(&p, &track);
                        SampleReport{
                            pattern: format_pattern(&p),
                            base: p.base,
                            total_len: p.total_len(),
                            mid_len: match p.midpoint { Midpoint::Free(l)|Midpoint::Zeros(l)=>l },
                            inner_zero: p.layers.first().map(|L| L.zero).unwrap_or(0),
                            samples: 0, primes: 0,
                            prime_density: 0.0, ci_lo: 0.0, ci_hi: 0.0,
                            expected_density_pnt_cond: expected_pnt,
                            expected_density_local: expected_local,
                            expected_density_local_exact: expected_local_exact,
                            enrichment_vs_pnt_cond: 0.0,
                            enrichment_vs_local: 0.0,
                            enrichment_vs_local_exact: 0.0,
                            elapsed_ms: 0,
                            tracked_moduli: track,
                            divisible_counts: vec![],
                            model_p0,
                        }
                    } else {
                        do_sample(&p, samples, cli.seed ^ (v as u64), false, &track, pre_sieve)
                    }
                }).collect()
            };

            for r in &results { write_csv_row(&mut w, r); }
            w.flush().unwrap();
        }

        // Ridge
        Commands::Ridge { mid_kind, mid_len_range, inner_zero_range, inner_slot, outer_layers, allowed_last_digits, out_csv, mirror, track_moduli, auto_track, auto_pmax, auto_k, auto_min_p0, auto_mode, objective } => {
            let (mid_a, mid_b) = parse_range(&mid_len_range);
            let (iz_a, iz_b) = parse_range(&inner_zero_range);
            let kind = mid_kind.to_lowercase(); assert!(kind == "free" || kind == "zeros");
            let outer = parse_layers(&outer_layers);
            let allowed = parse_allowed_last_digits(cli.base, &allowed_last_digits);
            let explicit_track = parse_u32_list(&track_moduli);
            let at_mode = auto_mode_from_str(Some(&auto_mode));
            let obj = objective.to_lowercase();
            assert!(obj=="exact" || obj=="prod");

            let global_track = if auto_track && at_mode==AutoMode::Global {
                Some(auto_select_moduli_global_grid(
                    cli.base, &kind, (mid_a,mid_b), (iz_a,iz_b),
                    inner_slot, &outer, &allowed, mirror,
                    auto_pmax, auto_k, auto_min_p0
                ))
            } else { None };

            let f = File::create(out_csv).expect("create csv");
            let mut w = BufWriter::new(f); write_ridge_header(&mut w);

            for mid_len in mid_a..=mid_b {
                let mut best_iz = iz_a;
                let mut best_score = f64::INFINITY;
                let mut best_lcm = 0u32;
                let mut best_track: Vec<u32> = vec![];
                let mut best_exact = 1.0;
                let mut best_prod = 1.0;

                for iz in iz_a..=iz_b {
                    let mut layers = vec![Layer { zero: iz, slot: inner_slot }];
                    layers.extend_from_slice(&outer);
                    let p = Pattern {
                        base: cli.base,
                        midpoint: if kind=="free" { Midpoint::Free(mid_len) } else { Midpoint::Zeros(mid_len) },
                        layers,
                        constraints: Constraints { allowed_last_digits: allowed.clone(), forbid_leading_zero: true },
                        mirror,
                    };

                    let track = if auto_track {
                        if let Some(gt) = &global_track { gt.clone() }
                        else { auto_select_moduli(&p, auto_pmax, auto_k, auto_min_p0, mirror) }
                    } else {
                        let mut t = explicit_track.clone();
                        if mirror { let m = p.base + 1; if !t.contains(&m) { t.push(m); } }
                        t.sort_unstable(); t.dedup(); t
                    };

                    let (_, p_any_prod) = (0, {
                        let mut prod = 1.0;
                        for &m in &track { if m>=2 { prod *= 1.0 - residue_null_probability(&p, m); } }
                        1.0 - prod
                    });
                    let (lcm, p_any_exact) = union_null_probability_lcm(&p, &track);

                    let score = if obj=="exact" { p_any_exact } else { p_any_prod };
                    if score < best_score {
                        best_score = score;
                        best_iz = iz;
                        best_lcm = lcm;
                        best_track = track.clone();
                        best_exact = p_any_exact;
                        best_prod  = p_any_prod;
                    }
                }

                let mut layers = vec![Layer { zero: best_iz, slot: inner_slot }];
                layers.extend_from_slice(&outer);
                let p_best = Pattern {
                    base: cli.base,
                    midpoint: if kind=="free" { Midpoint::Free(mid_len) } else { Midpoint::Zeros(mid_len) },
                    layers,
                    constraints: Constraints { allowed_last_digits: allowed.clone(), forbid_leading_zero: true },
                    mirror,
                };
                let e_local_exact = expected_density_local_exact(&p_best, &best_track);
                let e_local = expected_density_local(&p_best, &best_track);

                write_ridge_row(&mut w, cli.base, mid_len, best_iz, &obj, best_exact, best_prod, e_local_exact, e_local, best_lcm, &best_track);
            }
            w.flush().unwrap();
        }

        // Run (TOML)
        Commands::Run { config } => {
            let cfg = read_toml(&config);
            let base_default = cfg.base.unwrap_or(cli.base);
            let seed_default = cfg.seed.unwrap_or(cli.seed);

            for job in cfg.jobs {
                match job {
                    Job::Sample { base, seed, midpoint, layers, samples, allowed_last_digits, out_json, out_csv, parallel, mirror, track_moduli, pre_sieve, auto_track } => {
                        let job_base = base.unwrap_or(base_default);
                        let job_seed = seed.unwrap_or(seed_default);
                        let mirror = mirror.unwrap_or(false);
                        let pre_sieve = pre_sieve.unwrap_or(true);
                        let parallel = parallel.unwrap_or(true);
                        let at = auto_track.unwrap_or_default();
                        let enable_at = at.enable.unwrap_or(false);
                        let pmax = at.pmax.unwrap_or(97);
                        let k = at.k.unwrap_or(8);
                        let min = at.min_p0.unwrap_or(0.03);

                        // Parse allowed digits with logging
                        let allowed = parse_allowed_last_digits(cli.base, &allowed_last_digits);
                        if allowed_last_digits.trim().eq_ignore_ascii_case("auto") {
                            eprintln!("Auto-selected coprime digits for base {}: {:?}", job_base, allowed);
                        }

                        let p = Pattern {
                            base: job_base,
                            midpoint: parse_midpoint(&midpoint),
                            layers: parse_layers(&layers),
                            constraints: Constraints { allowed_last_digits: allowed, forbid_leading_zero: true },
                            mirror,
                        };
                        let mut track = if enable_at {
                            auto_select_moduli(&p, pmax, k, min, mirror)
                        } else {
                            let mut t = parse_u32_list(&track_moduli);
                            if mirror { let m = p.base + 1; if !t.contains(&m) { t.push(m); } }
                            t
                        };
                        track.sort_unstable(); track.dedup();

                        // Log auto-track selection
                        if enable_at {
                            eprintln!("Auto-track: selected {} moduli up to p={}", track.len(), pmax);
                            eprintln!("Tracked moduli: {:?}", track);
                        }

                        let report = do_sample(&p, samples, job_seed, parallel, &track, pre_sieve);
                        if let Some(path) = out_json.clone() {
                            let f = File::create(path).expect("create json");
                            serde_json::to_writer_pretty(f, &report).expect("write json");
                        }
                        if let Some(path) = out_csv.clone() {
                            let f = File::create(path).expect("create csv");
                            let mut w = BufWriter::new(f);
                            write_csv_header(&mut w);
                            write_csv_row(&mut w, &report);
                            w.flush().unwrap();
                        }
                    }
                    Job::Grid { base, seed, mid_kind, mid_len_range, inner_zero_range, inner_slot, outer_layers, samples, allowed_last_digits, out_csv, parallel, mirror, track_moduli, pre_sieve, auto_track } => {
                        let job_base = base.unwrap_or(base_default);
                        let job_seed = seed.unwrap_or(seed_default);
                        let (mid_a, mid_b) = parse_range(&mid_len_range);
                        let (iz_a, iz_b) = parse_range(&inner_zero_range);
                        let kind = mid_kind.to_lowercase(); assert!(kind == "free" || kind == "zeros");
                        let outer = parse_layers(&outer_layers);
                        let mirror = mirror.unwrap_or(false);
                        let pre_sieve = pre_sieve.unwrap_or(true);
                        let parallel = parallel.unwrap_or(true);
                        let at = auto_track.unwrap_or_default();
                        let enable_at = at.enable.unwrap_or(false);
                        let at_mode = auto_mode_from_str(at.mode.as_deref());
                        let pmax = at.pmax.unwrap_or(97);
                        let k = at.k.unwrap_or(8);
                        let min = at.min_p0.unwrap_or(0.03);
                        let explicit_track = parse_u32_list(&track_moduli);

                        // Log auto-selected digits
                        let allowed = parse_allowed_last_digits(cli.base, &allowed_last_digits);
                        if allowed_last_digits.trim().eq_ignore_ascii_case("auto") {
                            eprintln!("Auto-selected coprime digits for base {}: {:?}", job_base, allowed);
                        }

                        let global_track = if enable_at && at_mode==AutoMode::Global {
                            let track = auto_select_moduli_global_grid(
                                job_base, &kind, (mid_a,mid_b), (iz_a,iz_b),
                                inner_slot, &outer, &allowed,
                                mirror, pmax, k, min
                            );
                            eprintln!("Auto-track (global mode): selected {} moduli up to p={}", track.len(), pmax);
                            eprintln!("Tracked moduli: {:?}", track);
                            Some(track)
                        } else { None };

                        let f = File::create(out_csv).expect("create csv");
                        let mut w = BufWriter::new(f); write_csv_header(&mut w);

                        let combos: Vec<(usize, usize)> = (mid_a..=mid_b).flat_map(|m| (iz_a..=iz_b).map(move |z| (m, z))).collect();
                        let results: Vec<SampleReport> = if parallel {
                            combos.par_iter().map(|(m, z)| {
                                let mut layers = vec![Layer { zero: *z, slot: inner_slot }]; layers.extend_from_slice(&outer);
                                let p = Pattern {
                                    base: job_base,
                                    midpoint: if kind=="free" { Midpoint::Free(*m) } else { Midpoint::Zeros(*m) },
                                    layers,
                                    constraints: Constraints { allowed_last_digits: allowed.clone(), forbid_leading_zero: true },
                                    mirror,
                                };
                                let mut track = if enable_at {
                                    if let Some(gt) = &global_track { gt.clone() }
                                    else { auto_select_moduli(&p, pmax, k, min, mirror) }
                                } else {
                                    let mut t = explicit_track.clone();
                                    if mirror { let m = p.base + 1; if !t.contains(&m) { t.push(m); } }
                                    t
                                };
                                track.sort_unstable(); track.dedup();
                                do_sample(&p, samples, job_seed ^ ((*m as u64) << 32) ^ (*z as u64), true, &track, pre_sieve)
                            }).collect()
                        } else {
                            combos.iter().map(|(m, z)| {
                                let mut layers = vec![Layer { zero: *z, slot: inner_slot }]; layers.extend_from_slice(&outer);
                                let p = Pattern {
                                    base: job_base,
                                    midpoint: if kind=="free" { Midpoint::Free(*m) } else { Midpoint::Zeros(*m) },
                                    layers,
                                    constraints: Constraints { allowed_last_digits: allowed.clone(), forbid_leading_zero: true },
                                    mirror,
                                };
                                let mut track = if enable_at {
                                    if let Some(gt) = &global_track { gt.clone() }
                                    else { auto_select_moduli(&p, pmax, k, min, mirror) }
                                } else {
                                    let mut t = explicit_track.clone();
                                    if mirror { let m = p.base + 1; if !t.contains(&m) { t.push(m); } }
                                    t
                                };
                                track.sort_unstable(); track.dedup();
                                do_sample(&p, samples, job_seed ^ ((*m as u64) << 32) ^ (*z as u64), false, &track, pre_sieve)
                            }).collect()
                        };
                        for r in &results { write_csv_row(&mut w, r); }
                        w.flush().unwrap();
                    }
                    Job::ModelOnly { base, mid_kind, mid_len_range, inner_zero_range, inner_slot, outer_layers, allowed_last_digits, out_csv, mirror, track_moduli, auto_track } => {
                        let job_base = base.unwrap_or(base_default);
                        let (mid_a, mid_b) = parse_range(&mid_len_range);
                        let (iz_a, iz_b) = parse_range(&inner_zero_range);
                        let kind = mid_kind.to_lowercase(); assert!(kind == "free" || kind == "zeros");
                        let outer = parse_layers(&outer_layers);
                        let mirror = mirror.unwrap_or(false);
                        let at = auto_track.unwrap_or_default();
                        let enable_at = at.enable.unwrap_or(false);
                        let at_mode = auto_mode_from_str(at.mode.as_deref());
                        let pmax = at.pmax.unwrap_or(97);
                        let k = at.k.unwrap_or(8);
                        let min = at.min_p0.unwrap_or(0.03);
                        let explicit_track = parse_u32_list(&track_moduli);

                        let global_track = if enable_at && at_mode==AutoMode::Global {
                            Some(auto_select_moduli_global_grid(
                                job_base, &kind, (mid_a,mid_b), (iz_a,iz_b),
                                inner_slot, &outer, &parse_allowed_last_digits(cli.base, &allowed_last_digits),
                                mirror, pmax, k, min
                            ))
                        } else { None };

                        let f = File::create(out_csv).expect("create csv");
                        let mut w = BufWriter::new(f); write_model_csv_header(&mut w);

                        for mid_len in mid_a..=mid_b {
                            for iz in iz_a..=iz_b {
                                let mut layers = vec![Layer { zero: iz, slot: inner_slot }]; layers.extend_from_slice(&outer);
                                let p = Pattern {
                                    base: job_base,
                                    midpoint: if kind=="free" { Midpoint::Free(mid_len) } else { Midpoint::Zeros(mid_len) },
                                    layers,
                                    constraints: Constraints { allowed_last_digits: parse_allowed_last_digits(cli.base, &allowed_last_digits), forbid_leading_zero: true },
                                    mirror,
                                };
                                let mut track = if enable_at {
                                    if let Some(gt) = &global_track { gt.clone() }
                                    else { auto_select_moduli(&p, pmax, k, min, mirror) }
                                } else {
                                    let mut t = explicit_track.clone();
                                    if mirror { let m = p.base + 1; if !t.contains(&m) { t.push(m); } }
                                    t
                                };
                                track.sort_unstable(); track.dedup();

                                // Use LCM vector path when possible (Optimization F)
                                let spec_wrapped = build_spec(&p);
                                let model_p0: Vec<f64> = if let Some((_l, pairs)) =
                                    residue_null_vector_via_lcm_with_spec(&spec_wrapped, p.base, &track)
                                {
                                    track.iter().map(|&m|
                                        if m < 2 { 0.0 }
                                        else { pairs.iter().find(|(pm,_)| *pm==m).map(|(_,q)| *q).unwrap_or(0.0) }
                                    ).collect()
                                } else {
                                    // Fallback: per-prime DP when LCM too large
                                    track.iter().map(|&m| if m>=2 {
                                        residue_null_probability_with_spec(&spec_wrapped, p.base, m)
                                    } else { 0.0 }).collect()
                                };
                                let expected_pnt = expected_density_pnt_conditional(&p);
                                let expected_local = expected_density_local(&p, &track);
                                let expected_local_exact = expected_density_local_exact(&p, &track);

                                write_model_csv_row(&mut w, &format_pattern(&p), p.base, p.total_len(), mid_len, iz, expected_pnt, expected_local, expected_local_exact, &track, &model_p0);
                            }
                        }
                        w.flush().unwrap();
                    }
                    Job::ExplainGrid { base, mid_kind, mid_len_range, inner_zero_range, inner_slot, outer_layers, allowed_last_digits, out_json, mirror, track_moduli, auto_track } => {
                        let job_base = base.unwrap_or(base_default);
                        let (mid_a, mid_b) = parse_range(&mid_len_range);
                        let (iz_a, iz_b) = parse_range(&inner_zero_range);
                        let kind = mid_kind.to_lowercase(); assert!(kind == "free" || kind == "zeros");
                        let outer = parse_layers(&outer_layers);
                        let mirror = mirror.unwrap_or(false);
                        let at = auto_track.unwrap_or_default();
                        let enable_at = at.enable.unwrap_or(false);
                        let at_mode = auto_mode_from_str(at.mode.as_deref());
                        let pmax = at.pmax.unwrap_or(97);
                        let k = at.k.unwrap_or(8);
                        let min = at.min_p0.unwrap_or(0.03);
                        let explicit_track = parse_u32_list(&track_moduli);

                        let global_track = if enable_at && at_mode==AutoMode::Global {
                            Some(auto_select_moduli_global_grid(
                                job_base, &kind, (mid_a,mid_b), (iz_a,iz_b),
                                inner_slot, &outer, &parse_allowed_last_digits(cli.base, &allowed_last_digits),
                                mirror, pmax, k, min
                            ))
                        } else { None };

                        let mut entries: Vec<ExplainReport> = Vec::new();
                        for mid_len in mid_a..=mid_b {
                            for iz in iz_a..=iz_b {
                                let mut layers = vec![Layer { zero: iz, slot: inner_slot }]; layers.extend_from_slice(&outer);
                                let p = Pattern {
                                    base: job_base,
                                    midpoint: if kind=="free" { Midpoint::Free(mid_len) } else { Midpoint::Zeros(mid_len) },
                                    layers,
                                    constraints: Constraints { allowed_last_digits: parse_allowed_last_digits(cli.base, &allowed_last_digits), forbid_leading_zero: true },
                                    mirror,
                                };
                                let mut track = if enable_at {
                                    if let Some(gt) = &global_track { gt.clone() }
                                    else { auto_select_moduli(&p, pmax, k, min, mirror) }
                                } else {
                                    let mut t = explicit_track.clone();
                                    if mirror { let m = p.base + 1; if !t.contains(&m) { t.push(m); } }
                                    t
                                };
                                track.sort_unstable(); track.dedup();
                                entries.push(explain_for_pattern(&p, &track));
                            }
                        }
                        let f = File::create(out_json).expect("create json");
                        serde_json::to_writer_pretty(f, &entries).expect("write json");
                    }
                    Job::Lineout { base, seed, axis, mid_kind, range, inner_zero, mid_len, inner_slot, outer_layers, samples, allowed_last_digits, out_csv, parallel, mirror, track_moduli, pre_sieve, auto_track, model_only } => {
                        let job_base = base.unwrap_or(base_default);
                        let job_seed = seed.unwrap_or(seed_default);
                        let (a, b) = parse_range(&range);
                        let kind = mid_kind.to_lowercase(); assert!(kind == "free" || kind == "zeros");
                        let mirror = mirror.unwrap_or(false);
                        let pre_sieve = pre_sieve.unwrap_or(true);
                        let parallel = parallel.unwrap_or(true);
                        let at = auto_track.unwrap_or_default();
                        let enable_at = at.enable.unwrap_or(false);
                        let at_mode = auto_mode_from_str(at.mode.as_deref());
                        let pmax = at.pmax.unwrap_or(97);
                        let k = at.k.unwrap_or(8);
                        let min = at.min_p0.unwrap_or(0.03);
                        let explicit_track = parse_u32_list(&track_moduli);
                        let fixed_iz = inner_zero.unwrap_or(0);
                        let fixed_mid = mid_len.unwrap_or(1);
                        let outer = parse_layers(&outer_layers);
                        let model_only = model_only.unwrap_or(false);

                        let vals: Vec<usize> = (a..=b).collect();
                        let global_track = if enable_at && at_mode==AutoMode::Global {
                            Some(auto_select_moduli_global_line(
                                job_base, &kind, &axis, &vals,
                                fixed_mid, fixed_iz, inner_slot, &outer,
                                &parse_allowed_last_digits(cli.base, &allowed_last_digits), mirror,
                                pmax, k, min
                            ))
                        } else { None };

                        let f = File::create(out_csv).expect("create csv");
                        let mut w = BufWriter::new(f); write_csv_header(&mut w);

                        let results: Vec<SampleReport> = if parallel {
                            vals.par_iter().map(|&v| {
                                let mut layers = vec![Layer { zero: if axis=="mid" { fixed_iz } else { v }, slot: inner_slot }];
                                layers.extend_from_slice(&outer);
                                let p = Pattern {
                                    base: job_base,
                                    midpoint: if axis=="mid" {
                                        if kind=="free" { Midpoint::Free(v) } else { Midpoint::Zeros(v) }
                                    } else {
                                        if kind=="free" { Midpoint::Free(fixed_mid) } else { Midpoint::Zeros(fixed_mid) }
                                    },
                                    layers,
                                    constraints: Constraints { allowed_last_digits: parse_allowed_last_digits(cli.base, &allowed_last_digits), forbid_leading_zero: true },
                                    mirror,
                                };
                                let mut track = if enable_at {
                                    if let Some(gt) = &global_track { gt.clone() }
                                    else { auto_select_moduli(&p, pmax, k, min, mirror) }
                                } else {
                                    let mut t = explicit_track.clone();
                                    if mirror { let m = p.base + 1; if !t.contains(&m) { t.push(m); } }
                                    t
                                };
                                track.sort_unstable(); track.dedup();

                                if model_only {
                                    // Use LCM vector path when possible (Optimization F)
                                    let spec_wrapped = build_spec(&p);
                                    let model_p0: Vec<f64> = if let Some((_l, pairs)) =
                                        residue_null_vector_via_lcm_with_spec(&spec_wrapped, p.base, &track)
                                    {
                                        track.iter().map(|&m|
                                            if m < 2 { 0.0 }
                                            else { pairs.iter().find(|(pm,_)| *pm==m).map(|(_,q)| *q).unwrap_or(0.0) }
                                        ).collect()
                                    } else {
                                        // Fallback: per-prime DP when LCM too large
                                        track.iter().map(|&m| if m>=2 {
                                            residue_null_probability_with_spec(&spec_wrapped, p.base, m)
                                        } else { 0.0 }).collect()
                                    };
                                    let expected_pnt = expected_density_pnt_conditional(&p);
                                    let expected_local = expected_density_local(&p, &track);
                                    let expected_local_exact = expected_density_local_exact(&p, &track);
                                    SampleReport{
                                        pattern: format_pattern(&p),
                                        base: p.base,
                                        total_len: p.total_len(),
                                        mid_len: match p.midpoint { Midpoint::Free(l)|Midpoint::Zeros(l)=>l },
                                        inner_zero: p.layers.first().map(|L| L.zero).unwrap_or(0),
                                        samples: 0, primes: 0,
                                        prime_density: 0.0, ci_lo: 0.0, ci_hi: 0.0,
                                        expected_density_pnt_cond: expected_pnt,
                                        expected_density_local: expected_local,
                                        expected_density_local_exact: expected_local_exact,
                                        enrichment_vs_pnt_cond: 0.0,
                                        enrichment_vs_local: 0.0,
                                        enrichment_vs_local_exact: 0.0,
                                        elapsed_ms: 0,
                                        tracked_moduli: track,
                                        divisible_counts: vec![],
                                        model_p0,
                                    }
                                } else {
                                    do_sample(&p, samples, job_seed ^ (v as u64), true, &track, pre_sieve)
                                }
                            }).collect()
                        } else {
                            vals.iter().map(|&v| {
                                let mut layers = vec![Layer { zero: if axis=="mid" { fixed_iz } else { v }, slot: inner_slot }];
                                layers.extend_from_slice(&outer);
                                let p = Pattern {
                                    base: job_base,
                                    midpoint: if axis=="mid" {
                                        if kind=="free" { Midpoint::Free(v) } else { Midpoint::Zeros(v) }
                                    } else {
                                        if kind=="free" { Midpoint::Free(fixed_mid) } else { Midpoint::Zeros(fixed_mid) }
                                    },
                                    layers,
                                    constraints: Constraints { allowed_last_digits: parse_allowed_last_digits(cli.base, &allowed_last_digits), forbid_leading_zero: true },
                                    mirror,
                                };
                                let mut track = if enable_at {
                                    if let Some(gt) = &global_track { gt.clone() }
                                    else { auto_select_moduli(&p, pmax, k, min, mirror) }
                                } else {
                                    let mut t = explicit_track.clone();
                                    if mirror { let m = p.base + 1; if !t.contains(&m) { t.push(m); } }
                                    t
                                };
                                track.sort_unstable(); track.dedup();

                                if model_only {
                                    // Use LCM vector path when possible (Optimization F)
                                    let spec_wrapped = build_spec(&p);
                                    let model_p0: Vec<f64> = if let Some((_l, pairs)) =
                                        residue_null_vector_via_lcm_with_spec(&spec_wrapped, p.base, &track)
                                    {
                                        track.iter().map(|&m|
                                            if m < 2 { 0.0 }
                                            else { pairs.iter().find(|(pm,_)| *pm==m).map(|(_,q)| *q).unwrap_or(0.0) }
                                        ).collect()
                                    } else {
                                        // Fallback: per-prime DP when LCM too large
                                        track.iter().map(|&m| if m>=2 {
                                            residue_null_probability_with_spec(&spec_wrapped, p.base, m)
                                        } else { 0.0 }).collect()
                                    };
                                    let expected_pnt = expected_density_pnt_conditional(&p);
                                    let expected_local = expected_density_local(&p, &track);
                                    let expected_local_exact = expected_density_local_exact(&p, &track);
                                    SampleReport{
                                        pattern: format_pattern(&p),
                                        base: p.base,
                                        total_len: p.total_len(),
                                        mid_len: match p.midpoint { Midpoint::Free(l)|Midpoint::Zeros(l)=>l },
                                        inner_zero: p.layers.first().map(|L| L.zero).unwrap_or(0),
                                        samples: 0, primes: 0,
                                        prime_density: 0.0, ci_lo: 0.0, ci_hi: 0.0,
                                        expected_density_pnt_cond: expected_pnt,
                                        expected_density_local: expected_local,
                                        expected_density_local_exact: expected_local_exact,
                                        enrichment_vs_pnt_cond: 0.0,
                                        enrichment_vs_local: 0.0,
                                        enrichment_vs_local_exact: 0.0,
                                        elapsed_ms: 0,
                                        tracked_moduli: track,
                                        divisible_counts: vec![],
                                        model_p0,
                                    }
                                } else {
                                    do_sample(&p, samples, job_seed ^ (v as u64), false, &track, pre_sieve)
                                }
                            }).collect()
                        };

                        for r in &results { write_csv_row(&mut w, r); }
                        w.flush().unwrap();
                    }
                    Job::Ridge { base, mid_kind, mid_len_range, inner_zero_range, inner_slot, outer_layers, allowed_last_digits, out_csv, mirror, track_moduli, auto_track, objective } => {
                        let job_base = base.unwrap_or(base_default);
                        let (mid_a, mid_b) = parse_range(&mid_len_range);
                        let (iz_a, iz_b) = parse_range(&inner_zero_range);
                        let kind = mid_kind.to_lowercase(); assert!(kind == "free" || kind == "zeros");
                        let outer = parse_layers(&outer_layers);
                        let mirror = mirror.unwrap_or(false);
                        let at = auto_track.unwrap_or_default();
                        let enable_at = at.enable.unwrap_or(false);
                        let at_mode = auto_mode_from_str(at.mode.as_deref());
                        let pmax = at.pmax.unwrap_or(97);
                        let k = at.k.unwrap_or(8);
                        let min = at.min_p0.unwrap_or(0.03);
                        let explicit_track = parse_u32_list(&track_moduli);
                        let allowed = parse_allowed_last_digits(cli.base, &allowed_last_digits);
                        let obj = objective.unwrap_or_else(|| "exact".to_string());
                        let obj = obj.to_lowercase();
                        assert!(obj=="exact" || obj=="prod");

                        let global_track = if enable_at && at_mode==AutoMode::Global {
                            Some(auto_select_moduli_global_grid(
                                job_base, &kind, (mid_a,mid_b), (iz_a,iz_b),
                                inner_slot, &outer, &allowed, mirror,
                                pmax, k, min
                            ))
                        } else { None };

                        let f = File::create(out_csv).expect("create csv");
                        let mut w = BufWriter::new(f); write_ridge_header(&mut w);

                        for mid_len in mid_a..=mid_b {
                            let mut best_iz = iz_a;
                            let mut best_score = f64::INFINITY;
                            let mut best_lcm = 0u32;
                            let mut best_track: Vec<u32> = vec![];
                            let mut best_exact = 1.0;
                            let mut best_prod = 1.0;

                            for iz in iz_a..=iz_b {
                                let mut layers = vec![Layer { zero: iz, slot: inner_slot }];
                                layers.extend_from_slice(&outer);
                                let p = Pattern {
                                    base: job_base,
                                    midpoint: if kind=="free" { Midpoint::Free(mid_len) } else { Midpoint::Zeros(mid_len) },
                                    layers,
                                    constraints: Constraints { allowed_last_digits: allowed.clone(), forbid_leading_zero: true },
                                    mirror,
                                };
                                let track = if enable_at {
                                    if let Some(gt) = &global_track { gt.clone() }
                                    else { auto_select_moduli(&p, pmax, k, min, mirror) }
                                } else {
                                    let mut t = explicit_track.clone();
                                    if mirror { let m = p.base + 1; if !t.contains(&m) { t.push(m); } }
                                    t.sort_unstable(); t.dedup(); t
                                };

                                let (_, p_any_prod) = (0, {
                                    let mut prod = 1.0;
                                    for &m in &track { if m>=2 { prod *= 1.0 - residue_null_probability(&p, m); } }
                                    1.0 - prod
                                });
                                let (lcm, p_any_exact) = union_null_probability_lcm(&p, &track);

                                let score = if obj=="exact" { p_any_exact } else { p_any_prod };
                                if score < best_score {
                                    best_score = score;
                                    best_iz = iz;
                                    best_lcm = lcm;
                                    best_track = track.clone();
                                    best_exact = p_any_exact;
                                    best_prod  = p_any_prod;
                                }
                            }

                            let mut layers = vec![Layer { zero: best_iz, slot: inner_slot }];
                            layers.extend_from_slice(&outer);
                            let p_best = Pattern {
                                base: job_base,
                                midpoint: if kind=="free" { Midpoint::Free(mid_len) } else { Midpoint::Zeros(mid_len) },
                                layers,
                                constraints: Constraints { allowed_last_digits: allowed.clone(), forbid_leading_zero: true },
                                mirror,
                            };
                            let e_local_exact = expected_density_local_exact(&p_best, &best_track);
                            let e_local = expected_density_local(&p_best, &best_track);

                            write_ridge_row(&mut w, job_base, mid_len, best_iz, &obj, best_exact, best_prod, e_local_exact, e_local, best_lcm, &best_track);
                        }
                        w.flush().unwrap();
                    }
                }
            }
        }
    }
}

// ---------------- tests ----------------

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn residue_model_matches_mc_mod3() {
        let p = Pattern {
            base: 10,
            midpoint: Midpoint::Free(1),
            layers: vec![Layer { zero: 0, slot: 1 }],
            constraints: Constraints { allowed_last_digits: vec![1,3,7,9], forbid_leading_zero: true },
            mirror: false,
        };
        let p0 = residue_null_probability(&p, 3);
        let spec = build_digit_spec(&p);
        let mut rng = StdRng::seed_from_u64(123);
        let mut hits = 0usize;
        let trials = 5000;
        for _ in 0..trials {
            let n = super::sample_number(&p, &spec, &mut rng);
            if (&n % BigUint::from(3u32)).is_zero() { hits += 1; }
        }
        let freq = hits as f64 / trials as f64;
        assert!((freq - p0).abs() < 0.02, "freq={} p0={}", freq, p0);
    }

    #[test]
    fn mirror_even_length_divisible_by_b_plus_1() {
        let p = Pattern {
            base: 12,
            midpoint: Midpoint::Free(0),
            layers: vec![Layer { zero: 0, slot: 3 }], // total_len = 6 (even)
            constraints: Constraints { allowed_last_digits: vec![1,5,7,11], forbid_leading_zero: true },
            mirror: true,
        };
        let spec = build_digit_spec(&p);
        let mut rng = StdRng::seed_from_u64(999);
        for _ in 0..1000 {
            let n = super::sample_number(&p, &spec, &mut rng);
            assert!((&n % BigUint::from(13u32)).is_zero(), "palindrome not divisible by b+1");
        }
    }

    #[test]
    fn mirror_even_length_expected_density_zero() {
        // any even-length mirrored pattern should have predicted density 0 by our baseline
        let p = Pattern {
            base: 10,
            midpoint: Midpoint::Free(2),
            layers: vec![Layer { zero: 0, slot: 2 }], // total_len = 6 (even)
            constraints: Constraints { allowed_last_digits: vec![1,3,7,9], forbid_leading_zero: true },
            mirror: true,
        };
        let e1 = expected_density_pnt_conditional(&p);
        let e2 = expected_density_local(&p, &[]);
        let e3 = expected_density_local_exact(&p, &[]);
        assert_eq!(e1, 0.0);
        assert_eq!(e2, 0.0);
        assert_eq!(e3, 0.0);
    }

    #[test]
    fn mirror_odd_length_not_always_divisible_by_b_plus_1() {
        let p = Pattern {
            base: 12,
            midpoint: Midpoint::Free(1),
            layers: vec![Layer { zero: 0, slot: 3 }], // total_len = 7 (odd)
            constraints: Constraints { allowed_last_digits: vec![1,5,7,11], forbid_leading_zero: true },
            mirror: true,
        };
        let spec = build_digit_spec(&p);
        let mut rng = StdRng::seed_from_u64(12345);
        let mut all_divisible = true;
        for _ in 0..200 {
            let n = super::sample_number(&p, &spec, &mut rng);
            if !(&n % BigUint::from(13u32)).is_zero() { all_divisible = false; break; }
        }
        assert!(!all_divisible, "odd-length palindromes should not all be divisible by b+1");
    }

    #[test]
    fn weights_streaming_matches_pow() {
        for &(base, m, n) in &[(10u32,3u32,9usize),(12,5,11),(10,97,17)] {
            let w = weights_streaming(n, base % m, m);
            for i in 0..n {
                let exp = (n-1-i) as usize;
                let slow = pow_mod_u32(base % m, exp, m);
                assert_eq!(w[i], slow, "m={}, i={}", m, i);
            }
        }
    }

    #[test]
    fn p0_from_lcm_equals_per_prime_when_under_cap() {
        let p = Pattern {
            base: 10,
            midpoint: Midpoint::Free(3),
            layers: vec![Layer { zero: 2, slot: 1 }],
            constraints: Constraints { allowed_last_digits: vec![1,3,7,9], forbid_leading_zero: true },
            mirror: false,
        };
        let spec = build_spec(&p);
        let track = vec![3u32,5u32];
        let (l, vecp) = residue_null_vector_via_lcm_with_spec(&spec, p.base, &track).expect("under cap");
        assert_eq!(l, 15);
        for (pm, p0) in vecp {
            let solo = residue_null_probability_with_spec(&spec, p.base, pm);
            assert!((solo - p0).abs() < 1e-12, "pm={} solo={} lcm={}", pm, solo, p0);
        }
    }
}
