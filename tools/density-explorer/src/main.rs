use clap::{Parser, Subcommand};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::str::FromStr;

// ====================== CLI ======================

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
    /// Sample a single template and estimate prime density
    Sample {
        #[arg(long)]
        midpoint: String, // free:<len> | zeros:<len>

        #[arg(long, num_args = 1.., value_delimiter = ' ')]
        layers: Vec<String>, // ZERO:SLOT inner->outer

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

        // Optional exploration only; default is spacing-only (open slots independent)
        #[arg(long, default_value_t = false)]
        mirror: bool,

        #[arg(long, visible_alias = "track_primes", default_value = "")]
        track_moduli: String,

        #[arg(long, default_value_t = true)]
        pre_sieve: bool,
    },

    /// Sweep grid over midpoint length and inner zero padding; outer layers fixed
    Grid {
        #[arg(long, default_value = "free")]
        mid_kind: String, // free | zeros

        #[arg(long)]
        mid_len_range: String, // a..b inclusive

        #[arg(long)]
        inner_zero_range: String, // a..b inclusive

        #[arg(long, default_value_t = 1usize)]
        inner_slot: usize,

        #[arg(long, num_args = 0.., value_delimiter = ' ')]
        outer_layers: Vec<String>, // ZERO:SLOT ...

        #[arg(long, default_value_t = 50_000usize)]
        samples: usize,

        #[arg(long, default_value = "1,3,7,9")]
        allowed_last_digits: String,

        #[arg(long)]
        out_csv: String,

        #[arg(long, default_value_t = true)]
        parallel: bool,

        // Optional exploration only; default is spacing-only (open slots independent)
        #[arg(long, default_value_t = false)]
        mirror: bool,

        #[arg(long, visible_alias = "track_primes", default_value = "")]
        track_moduli: String,

        #[arg(long, default_value_t = true)]
        pre_sieve: bool,
    },

    /// Compute residue-model predictions only (no sampling) across a grid.
    /// Prints P0(p) = P(n ≡ 0 mod p) per tracked modulus, plus baselines.
    #[command(name = "model-only")]
    ModelOnly {
        #[arg(long, default_value = "free")]
        mid_kind: String, // free | zeros

        #[arg(long)]
        mid_len_range: String, // a..b inclusive

        #[arg(long)]
        inner_zero_range: String, // a..b inclusive

        #[arg(long, default_value_t = 1usize)]
        inner_slot: usize,

        #[arg(long, num_args = 0.., value_delimiter = ' ')]
        outer_layers: Vec<String>, // ZERO:SLOT ...

        #[arg(long, default_value = "1,3,7,9")]
        allowed_last_digits: String,

        #[arg(long)]
        out_csv: String,

        // Optional exploration only; default is spacing-only (open slots independent)
        #[arg(long, default_value_t = false)]
        mirror: bool,

        #[arg(long, visible_alias = "track_primes", default_value = "3,5,7,11")]
        track_moduli: String,
    },
}

// ====================== Pattern model ======================

#[derive(Clone, Debug, Serialize, Deserialize)]
enum Midpoint {
    Free(usize),
    Zeros(usize),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Layer {
    zero: usize,
    slot: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Constraints {
    allowed_last_digits: Vec<u32>,
    forbid_leading_zero: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Pattern {
    base: u32,
    midpoint: Midpoint,
    layers: Vec<Layer>, // inner -> outer
    constraints: Constraints,
    mirror: bool, // default false => spacing-only
}

impl Pattern {
    fn total_len(&self) -> usize {
        let mid = match self.midpoint {
            Midpoint::Free(l) | Midpoint::Zeros(l) => l,
        };
        let side: usize = self.layers.iter().map(|L| L.zero + L.slot).sum();
        mid + 2 * side
    }
}

// ====================== Parsing helpers ======================

fn parse_midpoint(s: &str) -> Midpoint {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        panic!("midpoint must be free:<len> or zeros:<len>");
    }
    let len = usize::from_str(parts[1]).expect("bad midpoint len");
    match parts[0].to_lowercase().as_str() {
        "free" => Midpoint::Free(len),
        "zeros" => Midpoint::Zeros(len),
        _ => panic!("midpoint kind must be free or zeros"),
    }
}
fn parse_layers(vecs: &[String]) -> Vec<Layer> {
    vecs.iter()
        .map(|s| {
            let parts: Vec<&str> = s.split(':').collect();
            if parts.len() != 2 {
                panic!("layer must be ZERO:SLOT");
            }
            let zero = usize::from_str(parts[0]).expect("bad zero");
            let slot = usize::from_str(parts[1]).expect("bad slot");
            Layer { zero, slot }
        })
        .collect()
}
fn parse_u32_list(s: &str) -> Vec<u32> {
    if s.trim().is_empty() {
        vec![]
    } else {
        s.split(',').map(|d| u32::from_str(d.trim()).unwrap()).collect()
    }
}

// ====================== Spec build (spacing symmetric; independent slots) ======================

fn build_digit_spec(p: &Pattern) -> Vec<Option<Vec<u32>>> {
    let n = p.total_len();
    let mut spec: Vec<Option<Vec<u32>>> = vec![None; n];
    let base_digits: Vec<u32> = (0..p.base).collect();

    fn set_open_slots(
        spec: &mut [Option<Vec<u32>>],
        base_digits: &[u32],
        range: std::ops::Range<usize>,
        is_leading: bool,
        is_last_digit_range: bool,
        mirror: bool,
        allowed_last_digits: &[u32],
    ) {
        for idx in range.clone() {
            let mut allowed = base_digits.to_vec();
            if is_leading {
                allowed = allowed.into_iter().filter(|&d| d != 0).collect();
                if mirror && !allowed_last_digits.is_empty() {
                    use std::collections::HashSet;
                    let set: HashSet<u32> = allowed_last_digits.iter().cloned().collect();
                    allowed = allowed.into_iter().filter(|d| set.contains(d)).collect();
                }
            }
            if is_last_digit_range && !allowed_last_digits.is_empty() {
                allowed = allowed_last_digits.to_vec();
            }
            spec[idx] = Some(allowed);
        }
    }

    let mid_len = match p.midpoint {
        Midpoint::Free(l) | Midpoint::Zeros(l) => l,
    };
    let mid_start = (n - mid_len) / 2;
    let mid_end = mid_start + mid_len;

    match p.midpoint {
        Midpoint::Zeros(_) => {
            for i in mid_start..mid_end {
                spec[i] = None;
            }
        }
        Midpoint::Free(_) => {
            let is_leading = mid_start == 0;
            let is_last = mid_end == n;
            set_open_slots(&mut spec, &base_digits, mid_start..mid_end, is_leading, is_last, p.mirror, &p.constraints.allowed_last_digits);
        }
    }

    let mut left_end = mid_start;
    let mut right_start = mid_end;

    for layer in p.layers.iter() {
        if layer.zero > 0 {
            let lz_start = left_end - layer.zero;
            for i in lz_start..left_end {
                spec[i] = None;
            }
            left_end = lz_start;

            let rz_end = right_start + layer.zero;
            for i in right_start..rz_end {
                spec[i] = None;
            }
            right_start = rz_end;
        }
        if layer.slot > 0 {
            let ls_start = left_end - layer.slot;
            let rs_end = right_start + layer.slot;

            let left_leading = ls_start == 0;
            let right_is_last = rs_end == n;

            set_open_slots(&mut spec, &base_digits, ls_start..left_end, left_leading, false, p.mirror, &p.constraints.allowed_last_digits);
            set_open_slots(&mut spec, &base_digits, right_start..rs_end, false, right_is_last, p.mirror, &p.constraints.allowed_last_digits);

            left_end = ls_start;
            right_start = rs_end;
        }
    }

    assert_eq!(left_end, 0, "left_end not at 0");
    assert_eq!(right_start, n, "right_start not at n");
    spec
}

// ====================== Number build & primality ======================

fn digits_to_biguint(digits: &[u32], base: u32) -> BigUint {
    let mut acc = BigUint::zero();
    let b = BigUint::from(base);
    for &d in digits {
        acc = &acc * &b + BigUint::from(d);
    }
    acc
}

fn sample_number<R: Rng>(p: &Pattern, spec: &[Option<Vec<u32>>], rng: &mut R) -> BigUint {
    let mut digits: Vec<u32> = vec![0; spec.len()];
    for (i, allowed_opt) in spec.iter().enumerate() {
        match allowed_opt {
            None => digits[i] = 0,
            Some(allowed) => {
                let idx = rng.gen_range(0..allowed.len());
                digits[i] = allowed[idx];
            }
        }
    }
    if p.mirror {
        let n = digits.len();
        for i in 0..(n / 2) {
            digits[n - 1 - i] = digits[i];
        }
    }
    digits_to_biguint(&digits, p.base)
}

fn modexp(mut base: BigUint, mut exp: BigUint, modu: &BigUint) -> BigUint {
    let mut result = BigUint::one();
    base %= modu;
    while exp > BigUint::zero() {
        if &exp & BigUint::one() == BigUint::one() {
            result = (result * &base) % modu;
        }
        exp >>= 1;
        base = (&base * &base) % modu;
    }
    result
}

fn miller_rabin_round(n: &BigUint, d: &BigUint, s: u32, a: &BigUint) -> bool {
    if a >= n {
        return true;
    }
    let one = BigUint::one();
    let n_minus_one = n - &one;
    let mut x = modexp(a.clone(), d.clone(), n);
    if x == one || x == n_minus_one {
        return true;
    }
    for _ in 1..s {
        x = (&x * &x) % n;
        if x == n_minus_one {
            return true;
        }
    }
    false
}

fn is_probable_prime(n: &BigUint) -> bool {
    let two = BigUint::from(2u32);
    let three = BigUint::from(3u32);
    if *n < two {
        return false;
    }
    if *n == two || *n == three {
        return true;
    }
    if (n % &two).is_zero() {
        return false;
    }

    let one = BigUint::one();
    let n_minus_one = n - &one;
    let mut d = n_minus_one.clone();
    let mut s = 0u32;
    while (&d & BigUint::from(1u32)).is_zero() {
        d >>= 1;
        s += 1;
    }

    let n_bits = n.bits();
    if n_bits <= 64 {
        let bases_u128: [u128; 7] = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
        for &a in &bases_u128 {
            if !miller_rabin_round(n, &d, s, &BigUint::from(a)) {
                return false;
            }
        }
        return true;
    }

    let bases: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &a in &bases {
        if !miller_rabin_round(n, &d, s, &BigUint::from(a)) {
            return false;
        }
    }
    true
}

// ====================== Small-mod residue model (spacing-only) ======================

fn pow_mod_u32(a: u32, mut e: usize, m: u32) -> u32 {
    if m == 1 {
        return 0;
    }
    let mut r: u64 = 1 % (m as u64);
    let mut base = (a as u64) % (m as u64);
    while e > 0 {
        if (e & 1) == 1 {
            r = (r * base) % (m as u64);
        }
        base = (base * base) % (m as u64);
        e >>= 1;
    }
    r as u32
}

// Exact P(n ≡ 0 mod m) from spacing + allowed digits via DP over residues.
fn residue_null_probability(p: &Pattern, modm: u32) -> f64 {
    if modm < 2 {
        return 0.0;
    }
    let spec = build_digit_spec(p);
    let n = spec.len();
    let base = p.base % modm;

    // dist[r] = probability sum ≡ r (mod m)
    let m = modm as usize;
    let mut dist = vec![0.0f64; m];
    dist[0] = 1.0;

    for (i, allowed_opt) in spec.iter().enumerate() {
        if let Some(allowed) = allowed_opt {
            if allowed.is_empty() {
                continue;
            }
            let exp = n - 1 - i; // positional exponent
            let mul = pow_mod_u32(base, exp, modm);
            let w = 1.0 / (allowed.len() as f64);

            let mut next = vec![0.0f64; m];
            for r in 0..m {
                let v = dist[r];
                if v == 0.0 {
                    continue;
                }
                for &d in allowed {
                    let delta = ((d as u64 * mul as u64) % (modm as u64)) as usize;
                    let to = (r + delta) % m;
                    next[to] += v * w;
                }
            }
            dist = next;
        } else {
            // fixed zero contributes nothing
        }
    }
    dist[0]
}

// ====================== Baselines & stats ======================

fn gcd_u32(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}
fn euler_phi(mut n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    let mut res = n;
    let mut p = 2u32;
    while (p as u64) * (p as u64) <= n as u64 {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            res = res / p * (p - 1);
        }
        p += 1;
    }
    if n > 1 {
        res = res / n * (n - 1);
    }
    res
}

// Conditional PNT baseline accounting for last-digit constraints; 0 if mirror & even length.
fn expected_density_pnt_conditional(p: &Pattern) -> f64 {
    let l = p.total_len() as f64;
    let ln_x = (l - 0.5) * (p.base as f64).ln();
    if ln_x <= 0.0 {
        return 0.0;
    }
    if p.mirror && (p.total_len() % 2 == 0) {
        return 0.0;
    }

    let b = p.base;
    let phi_b = euler_phi(b) as usize;

    let s: Vec<u32> = if p.constraints.allowed_last_digits.is_empty() {
        (0..b).collect()
    } else {
        p.constraints.allowed_last_digits.clone()
    };
    let k = s.len().max(1);
    let a = s
        .iter()
        .filter(|&&d| gcd_u32(d, b) == 1)
        .count()
        .max(1);

    (a as f64 * b as f64) / (k as f64 * phi_b as f64) * (1.0 / ln_x)
}

// Local-factors baseline using exact residue model on tracked set.
fn expected_density_local(p: &Pattern, track: &[u32]) -> f64 {
    let l = p.total_len() as f64;
    let ln_x = (l - 0.5) * (p.base as f64).ln();
    if ln_x <= 0.0 {
        return 0.0;
    }
    if p.mirror && (p.total_len() % 2 == 0) {
        return 0.0;
    }
    let mut prod = 1.0;
    for &m in track {
        if m >= 2 {
            let p0 = residue_null_probability(p, m);
            prod *= 1.0 - p0;
        }
    }
    prod * (1.0 / ln_x)
}

fn wilson_ci(primes: usize, n: usize, z: f64) -> (f64, f64) {
    let phat = primes as f64 / n as f64;
    let denom = 1.0 + z * z / n as f64;
    let center = phat + z * z / (2.0 * n as f64);
    let margin = z * ((phat * (1.0 - phat) / n as f64) + (z * z) / (4.0 * n as f64)).sqrt();
    let lo = (center - margin) / denom;
    let hi = (center + margin) / denom;
    (lo.max(0.0), hi.min(1.0))
}

fn format_pattern(p: &Pattern) -> String {
    let mid = match p.midpoint {
        Midpoint::Free(l) => format!("free:{l}"),
        Midpoint::Zeros(l) => format!("zeros:{l}"),
    };
    let layers = p
        .layers
        .iter()
        .map(|L| format!("{}:{}", L.zero, L.slot))
        .collect::<Vec<_>>()
        .join(",");
    format!("mid={mid}|layers=[{layers}]|base={}", p.base)
}

// ====================== Reporting ======================

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
    enrichment_vs_pnt_cond: f64,
    enrichment_vs_local: f64,
    elapsed_ms: u128,
    tracked_moduli: Vec<u32>,
    divisible_counts: Vec<usize>, // observed counts for each tracked modulus
    model_p0: Vec<f64>,           // exact P(n ≡ 0 mod m) from residue model
}

fn write_csv_header(w: &mut dyn Write) {
    writeln!(
        w,
        "pattern,base,total_len,mid_len,inner_zero,samples,primes,prime_density,ci_lo,ci_hi,expected_density_pnt_cond,expected_density_local,enrichment_vs_pnt_cond,enrichment_vs_local,elapsed_ms,tracked_moduli,divisible_counts,model_p0"
    )
    .unwrap();
}
fn write_csv_row(w: &mut dyn Write, r: &SampleReport) {
    writeln!(
        w,
        "{},{},{},{},{},{},{},{:.8},{:.8},{:.8},{:.8},{:.8},{:.8},{:.8},{},{:?},{:?},{:?}",
        r.pattern,
        r.base,
        r.total_len,
        r.mid_len,
        r.inner_zero,
        r.samples,
        r.primes,
        r.prime_density,
        r.ci_lo,
        r.ci_hi,
        r.expected_density_pnt_cond,
        r.expected_density_local,
        r.enrichment_vs_pnt_cond,
        r.enrichment_vs_local,
        r.elapsed_ms,
        r.tracked_moduli,
        r.divisible_counts,
        r.model_p0
    )
    .unwrap();
}

// Model-only CSV (no sampling fields)
fn write_model_csv_header(w: &mut dyn Write) {
    writeln!(
        w,
        "pattern,base,total_len,mid_len,inner_zero,expected_density_pnt_cond,expected_density_local,tracked_moduli,model_p0"
    )
    .unwrap();
}
fn write_model_csv_row(
    w: &mut dyn Write,
    pattern: &str,
    base: u32,
    total_len: usize,
    mid_len: usize,
    inner_zero: usize,
    expected_pnt: f64,
    expected_local: f64,
    tracked: &[u32],
    model_p0: &[f64],
) {
    writeln!(
        w,
        "{},{},{},{},{},{:.8},{:.8},{:?},{:?}",
        pattern, base, total_len, mid_len, inner_zero, expected_pnt, expected_local, tracked, model_p0
    )
    .unwrap();
}

// ====================== Core sampling ======================

fn do_sample(
    p: &Pattern,
    samples: usize,
    seed: u64,
    parallel: bool,
    track: &[u32],
    pre_sieve: bool,
) -> SampleReport {
    let total_len = p.total_len();
    let mid_len = match p.midpoint {
        Midpoint::Free(l) | Midpoint::Zeros(l) => l,
    };
    let inner_zero = p.layers.first().map(|L| L.zero).unwrap_or(0);
    let spec = build_digit_spec(p);

    // Residue model predictions for diagnostics visualized against observations
    let model_p0: Vec<f64> = track
        .iter()
        .map(|&m| if m >= 2 { residue_null_probability(p, m) } else { 0.0 })
        .collect();

    let expected_pnt = expected_density_pnt_conditional(p);
    let expected_local = expected_density_local(p, track);

    let start = std::time::Instant::now();
    let (primes, counts): (usize, Vec<usize>) = if parallel {
        (0..samples)
            .into_par_iter()
            .map(|i| {
                let mut rng = StdRng::seed_from_u64(
                    seed ^ (i as u64).wrapping_mul(0x9E3779B97F4A7C15),
                );
                let n = sample_number(p, &spec, &mut rng);

                // track divisibility & optional presieve
                let mut div = vec![0usize; track.len()];
                let mut blocked = false;
                for (j, &m) in track.iter().enumerate() {
                    if m >= 2 && (&n % BigUint::from(m)).is_zero() {
                        div[j] = 1;
                        if pre_sieve {
                            blocked = true;
                        }
                    }
                }
                let is_prime = if pre_sieve && blocked {
                    0usize
                } else {
                    is_probable_prime(&n) as usize
                };
                (is_prime, div)
            })
            .reduce(
                || (0usize, vec![0usize; track.len()]),
                |(p1, mut c1), (p2, c2)| {
                    for (i, v) in c2.iter().enumerate() {
                        c1[i] += *v;
                    }
                    (p1 + p2, c1)
                },
            )
    } else {
        let mut primes = 0usize;
        let mut counts = vec![0usize; track.len()];
        for i in 0..samples {
            let mut rng = StdRng::seed_from_u64(
                seed ^ (i as u64).wrapping_mul(0x9E3779B97F4A7C15),
            );
            let n = sample_number(p, &spec, &mut rng);
            let mut blocked = false;
            for (j, &m) in track.iter().enumerate() {
                if m >= 2 && (&n % BigUint::from(m)).is_zero() {
                    counts[j] += 1;
                    if pre_sieve {
                        blocked = true;
                    }
                }
            }
            if !(pre_sieve && blocked) && is_probable_prime(&n) {
                primes += 1;
            }
        }
        (primes, counts)
    };
    let elapsed = start.elapsed().as_millis();

    let density = primes as f64 / samples as f64;
    let (ci_lo, ci_hi) = wilson_ci(primes, samples, 1.96);
    let enrichment_vs_pnt = if expected_pnt > 0.0 {
        density / expected_pnt
    } else if primes == 0 {
        0.0
    } else {
        f64::INFINITY
    };
    let enrichment_vs_local = if expected_local > 0.0 {
        density / expected_local
    } else if primes == 0 {
        0.0
    } else {
        f64::INFINITY
    };

    SampleReport {
        pattern: format_pattern(p),
        base: p.base,
        total_len,
        mid_len,
        inner_zero,
        samples,
        primes,
        prime_density: density,
        ci_lo,
        ci_hi,
        expected_density_pnt_cond: expected_pnt,
        expected_density_local: expected_local,
        enrichment_vs_pnt_cond: enrichment_vs_pnt,
        enrichment_vs_local,
        elapsed_ms: elapsed,
        tracked_moduli: track.to_vec(),
        divisible_counts: counts,
        model_p0,
    }
}

// ====================== Main ======================

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Sample {
            midpoint,
            layers,
            samples,
            allowed_last_digits,
            out_json,
            out_csv,
            parallel,
            mirror,
            track_moduli,
            pre_sieve,
        } => {
            let mut track = parse_u32_list(&track_moduli);
            if mirror {
                // optional: include b+1 when exploring mirror; spacing-only default ignores this
                let m = cli.base + 1;
                if !track.contains(&m) {
                    track.push(m);
                }
            }
            let p = Pattern {
                base: cli.base,
                midpoint: parse_midpoint(&midpoint),
                layers: parse_layers(&layers),
                constraints: Constraints {
                    allowed_last_digits: parse_u32_list(&allowed_last_digits),
                    forbid_leading_zero: true,
                },
                mirror,
            };
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

        Commands::Grid {
            mid_kind,
            mid_len_range,
            inner_zero_range,
            inner_slot,
            outer_layers,
            samples,
            allowed_last_digits,
            out_csv,
            parallel,
            mirror,
            track_moduli,
            pre_sieve,
        } => {
            let parse_range = |s: &str| -> (usize, usize) {
                let parts: Vec<&str> = s.split("..").collect();
                assert!(parts.len() == 2, "range must be a..b");
                let a = usize::from_str(parts[0]).unwrap();
                let b = usize::from_str(parts[1]).unwrap();
                assert!(a <= b);
                (a, b)
            };
            let (mid_a, mid_b) = parse_range(&mid_len_range);
            let (iz_a, iz_b) = parse_range(&inner_zero_range);
            let kind = mid_kind.to_lowercase();
            assert!(kind == "free" || kind == "zeros");

            let outer = parse_layers(&outer_layers);

            let f = File::create(out_csv).expect("create csv");
            let mut w = BufWriter::new(f);
            write_csv_header(&mut w);

            let mut track = parse_u32_list(&track_moduli);
            if mirror {
                let m = cli.base + 1;
                if !track.contains(&m) {
                    track.push(m);
                }
            }

            let combos: Vec<(usize, usize)> = (mid_a..=mid_b)
                .flat_map(|m| (iz_a..=iz_b).map(move |z| (m, z)))
                .collect();

            let results: Vec<SampleReport> = if parallel {
                combos
                    .par_iter()
                    .map(|(m, z)| {
                        let mut layers = vec![Layer {
                            zero: *z,
                            slot: inner_slot,
                        }];
                        layers.extend_from_slice(&outer);
                        let p = Pattern {
                            base: cli.base,
                            midpoint: if kind == "free" {
                                Midpoint::Free(*m)
                            } else {
                                Midpoint::Zeros(*m)
                            },
                            layers,
                            constraints: Constraints {
                                allowed_last_digits: parse_u32_list(&allowed_last_digits),
                                forbid_leading_zero: true,
                            },
                            mirror,
                        };
                        do_sample(
                            &p,
                            samples,
                            cli.seed ^ ((*m as u64) << 32) ^ (*z as u64),
                            true,
                            &track,
                            pre_sieve,
                        )
                    })
                    .collect()
            } else {
                combos
                    .iter()
                    .map(|(m, z)| {
                        let mut layers = vec![Layer {
                            zero: *z,
                            slot: inner_slot,
                        }];
                        layers.extend_from_slice(&outer);
                        let p = Pattern {
                            base: cli.base,
                            midpoint: if kind == "free" {
                                Midpoint::Free(*m)
                            } else {
                                Midpoint::Zeros(*m)
                            },
                            layers,
                            constraints: Constraints {
                                allowed_last_digits: parse_u32_list(&allowed_last_digits),
                                forbid_leading_zero: true,
                            },
                            mirror,
                        };
                        do_sample(
                            &p,
                            samples,
                            cli.seed ^ ((*m as u64) << 32) ^ (*z as u64),
                            false,
                            &track,
                            pre_sieve,
                        )
                    })
                    .collect()
            };

            for r in &results {
                write_csv_row(&mut w, r);
            }
            w.flush().unwrap();
        }

        Commands::ModelOnly {
            mid_kind,
            mid_len_range,
            inner_zero_range,
            inner_slot,
            outer_layers,
            allowed_last_digits,
            out_csv,
            mirror,
            track_moduli,
        } => {
            let parse_range = |s: &str| -> (usize, usize) {
                let parts: Vec<&str> = s.split("..").collect();
                assert!(parts.len() == 2, "range must be a..b");
                let a = usize::from_str(parts[0]).unwrap();
                let b = usize::from_str(parts[1]).unwrap();
                assert!(a <= b);
                (a, b)
            };
            let (mid_a, mid_b) = parse_range(&mid_len_range);
            let (iz_a, iz_b) = parse_range(&inner_zero_range);
            let kind = mid_kind.to_lowercase();
            assert!(kind == "free" || kind == "zeros");

            let outer = parse_layers(&outer_layers);

            let f = File::create(out_csv).expect("create csv");
            let mut w = BufWriter::new(f);
            write_model_csv_header(&mut w);

            let mut track = parse_u32_list(&track_moduli);
            if mirror {
                let m = cli.base + 1;
                if !track.contains(&m) {
                    track.push(m);
                }
            }

            for mid_len in mid_a..=mid_b {
                for iz in iz_a..=iz_b {
                    let mut layers = vec![Layer {
                        zero: iz,
                        slot: inner_slot,
                    }];
                    layers.extend_from_slice(&outer);
                    let p = Pattern {
                        base: cli.base,
                        midpoint: if kind == "free" {
                            Midpoint::Free(mid_len)
                        } else {
                            Midpoint::Zeros(mid_len)
                        },
                        layers,
                        constraints: Constraints {
                            allowed_last_digits: parse_u32_list(&allowed_last_digits),
                            forbid_leading_zero: true,
                        },
                        mirror,
                    };

                    let model_p0: Vec<f64> = track
                        .iter()
                        .map(|&m| if m >= 2 { residue_null_probability(&p, m) } else { 0.0 })
                        .collect();
                    let expected_pnt = expected_density_pnt_conditional(&p);
                    let expected_local = expected_density_local(&p, &track);

                    write_model_csv_row(
                        &mut w,
                        &format_pattern(&p),
                        p.base,
                        p.total_len(),
                        mid_len,
                        iz,
                        expected_pnt,
                        expected_local,
                        &track,
                        &model_p0,
                    );
                }
            }
            w.flush().unwrap();
        }
    }
}
