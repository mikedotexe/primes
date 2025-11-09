use clap::{Parser, Subcommand};
use num_bigint::{BigUint};
use num_traits::{Zero, One};
use rand::{rngs::StdRng, SeedableRng, Rng};
use rayon::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::str::FromStr;

/// Symmetric zero-padding template exploration.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Random seed for reproducibility
    #[arg(long, default_value_t = 42u64)]
    seed: u64,

    /// Base for digit construction (default 10)
    #[arg(long, default_value_t = 10u32)]
    base: u32,

    /// Subcommand
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Sample a single template and estimate prime density
    Sample {
        /// Midpoint kind: free:<len> or zeros:<len>
        #[arg(long)]
        midpoint: String,

        /// Layers from inner to outer, each as ZERO:SLOT (e.g. 0:1 1:1 2:1)
        #[arg(long, num_args = 1.., value_delimiter = ' ')]
        layers: Vec<String>,

        /// Number of samples
        #[arg(long, default_value_t = 100_000usize)]
        samples: usize,

        /// Allowed last digits (comma-separated), e.g. 1,3,7,9 ; empty = all digits allowed
        #[arg(long, default_value = "1,3,7,9")]
        allowed_last_digits: String,

        /// Output JSON file (optional)
        #[arg(long)]
        out_json: Option<String>,

        /// Output CSV file (optional)
        #[arg(long)]
        out_csv: Option<String>,

        /// Enable parallel sampling
        #[arg(long, default_value_t = true)]
        parallel: bool,

        /// Enforce palindromic mirroring of digits (left half mirrors to right)
        #[arg(long, default_value_t = false)]
        mirror: bool,

        /// Track divisibility by these small moduli (comma-separated), e.g. 3,5,7,11
        /// Alias kept for compatibility with earlier docs: --track_primes
        #[arg(long, visible_alias = "track_primes", default_value = "")]
        track_moduli: String,

        /// If set, skip Miller–Rabin when divisible by any tracked modulus
        #[arg(long, default_value_t = true)]
        pre_sieve: bool,
    },

    /// Sweep grid over midpoint length and inner zero padding; outer layers fixed
    Grid {
        /// Midpoint kind base: free or zeros
        #[arg(long, default_value = "free")]
        mid_kind: String,

        /// Midpoint length range inclusive, e.g. 1..6
        #[arg(long)]
        mid_len_range: String,

        /// Inner zero padding range inclusive, e.g. 0..6
        #[arg(long)]
        inner_zero_range: String,

        /// Fixed inner slot length (S1)
        #[arg(long, default_value_t = 1usize)]
        inner_slot: usize,

        /// Additional outer layers after the inner layer, as ZERO:SLOT pairs (optional)
        #[arg(long, num_args = 0.., value_delimiter = ' ')]
        outer_layers: Vec<String>,

        /// Number of samples per grid point
        #[arg(long, default_value_t = 50_000usize)]
        samples: usize,

        /// Allowed last digits (comma-separated), e.g. 1,3,7,9 ; empty = all digits allowed
        #[arg(long, default_value = "1,3,7,9")]
        allowed_last_digits: String,

        /// CSV output path (required)
        #[arg(long)]
        out_csv: String,

        /// Enable parallel sampling
        #[arg(long, default_value_t = true)]
        parallel: bool,

        /// Enforce palindromic mirroring of digits (left half mirrors to right)
        #[arg(long, default_value_t = false)]
        mirror: bool,

        /// Track divisibility by these small moduli (comma-separated), e.g. 3,5,7,11
        /// Alias kept for compatibility with earlier docs: --track_primes
        #[arg(long, visible_alias = "track_primes", default_value = "")]
        track_moduli: String,

        /// If set, skip Miller–Rabin when divisible by any tracked modulus
        #[arg(long, default_value_t = true)]
        pre_sieve: bool,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum Midpoint {
    Free(usize),
    Zeros(usize),
}

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
    layers: Vec<Layer>, // inner -> outer
    constraints: Constraints,
    mirror: bool,
}

impl Pattern {
    fn total_len(&self) -> usize {
        let mid = match self.midpoint { Midpoint::Free(l) | Midpoint::Zeros(l) => l };
        let side: usize = self.layers.iter().map(|l| l.zero + l.slot).sum();
        mid + 2*side
    }
}

fn parse_midpoint(s: &str) -> Midpoint {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 { panic!("midpoint must be free:<len> or zeros:<len>"); }
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
        if parts.len() != 2 { panic!("layer must be ZERO:SLOT"); }
        let zero = usize::from_str(parts[0]).expect("bad zero");
        let slot = usize::from_str(parts[1]).expect("bad slot");
        Layer { zero, slot }
    }).collect()
}
fn parse_digit_list(s: &str) -> Vec<u32> {
    if s.trim().is_empty() { vec![] } else { s.split(',').map(|d| u32::from_str(d.trim()).unwrap()).collect() }
}

fn euler_phi(mut n: u32) -> u32 {
    if n <= 1 { return n; }
    let mut result = n;
    let mut p = 2;
    while (p as u64) * (p as u64) <= n as u64 {
        if n % p == 0 {
            while n % p == 0 { n /= p; }
            result = result / p * (p - 1);
        }
        p += 1;
    }
    if n > 1 { result = result / n * (n - 1); }
    result
}

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

    let mid_len = match p.midpoint { Midpoint::Free(l) | Midpoint::Zeros(l) => l };
    let mid_start = (n - mid_len)/2;
    let mid_end = mid_start + mid_len;

    match p.midpoint {
        Midpoint::Zeros(_) => { for i in mid_start..mid_end { spec[i] = None; } }
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
            for i in lz_start..left_end { spec[i] = None; }
            left_end = lz_start;

            let rz_end = right_start + layer.zero;
            for i in right_start..rz_end { spec[i] = None; }
            right_start = rz_end;
        }
        if layer.slot > 0 {
            let ls_start = left_end - layer.slot;
            let rs_end  = right_start + layer.slot;

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
            None => { digits[i] = 0; }
            Some(allowed) => {
                let idx = rng.gen_range(0..allowed.len());
                digits[i] = allowed[idx];
            }
        }
    }
    if p.mirror {
        let n = digits.len();
        for i in 0..(n/2) { digits[n-1-i] = digits[i]; }
    }
    digits_to_biguint(&digits, p.base)
}

/// Miller–Rabin for BigUint (deterministic for <=64bit, otherwise fixed bases).
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
        let bases: [u128; 7] = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
        for &a in &bases { if !miller_rabin_round(n, &d, s, &BigUint::from(a)) { return false; } }
        return true;
    }
    let bases: [u64; 12] = [2,3,5,7,11,13,17,19,23,29,31,37];
    for &a in &bases { if !miller_rabin_round(n, &d, s, &BigUint::from(a)) { return false; } }
    true
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

fn divisible_by_small(n: &BigUint, m: u32) -> bool {
    if m <= 1 { return false; }
    (n % BigUint::from(m)).is_zero()
}

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
    enrichment_factor: f64,
    elapsed_ms: u128,
    tracked_moduli: Vec<u32>,
    divisible_counts: Vec<usize>,
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
    let layers = p.layers.iter().map(|l| format!("{}:{}", l.zero, l.slot)).collect::<Vec<_>>().join(",");
    format!("mid={mid}|layers=[{layers}]|base={}", p.base)
}

/// Conditional PNT baseline given last-digit constraints and base.
/// If mirror && even length: returns 0.0 (divisible by base+1).
fn expected_density_pnt_conditional(p: &Pattern) -> f64 {
    let l = p.total_len() as f64;
    let ln_x = (l - 0.5) * (p.base as f64).ln();
    if ln_x <= 0.0 { return 0.0; }
    if p.mirror && (p.total_len() % 2 == 0) { return 0.0; }

    let b = p.base;
    let phi_b = euler_phi(b) as usize;

    // Allowed set S; if empty => all digits 0..b-1
    let s: Vec<u32> = if p.constraints.allowed_last_digits.is_empty() {
        (0..b).collect()
    } else {
        p.constraints.allowed_last_digits.clone()
    };
    let k = s.len().max(1); // safeguard
    let a = s.iter().filter(|&&d| num_integer::gcd(d as i64, b as i64) == 1).count().max(1);

    // (a/φ(b)) / (k/b) * 1/ln x = (a*b)/(k*φ(b)) * 1/ln x
    (a as f64 * b as f64) / (k as f64 * phi_b as f64) * (1.0 / ln_x)
}

fn write_csv_header(w: &mut dyn Write) {
    writeln!(
        w,
        "pattern,base,total_len,mid_len,inner_zero,samples,primes,prime_density,ci_lo,ci_hi,expected_density_pnt_cond,enrichment_factor,elapsed_ms,tracked_moduli,divisible_counts"
    ).unwrap();
}
fn write_csv_row(w: &mut dyn Write, r: &SampleReport) {
    writeln!(
        w,
        "\"{}\",{},{},{},{},{},{},{:.8},{:.8},{:.8},{:.8},{:.8},{},{:?},{:?}",
        r.pattern, r.base, r.total_len, r.mid_len, r.inner_zero, r.samples, r.primes,
        r.prime_density, r.ci_lo, r.ci_hi, r.expected_density_pnt_cond, r.enrichment_factor,
        r.elapsed_ms, r.tracked_moduli, r.divisible_counts
    ).unwrap();
}

fn do_sample(
    p: &Pattern,
    samples: usize,
    seed: u64,
    parallel: bool,
    track: &[u32],
    pre_sieve: bool,
) -> SampleReport {
    let total_len = p.total_len();
    let mid_len = match p.midpoint { Midpoint::Free(l) | Midpoint::Zeros(l) => l };
    let inner_zero = p.layers.first().map(|l| l.zero).unwrap_or(0);

    let spec = build_digit_spec(p);
    let expected = expected_density_pnt_conditional(p);

    let start = std::time::Instant::now();
    let (primes, counts): (usize, Vec<usize>) = if parallel {
        (0..samples).into_par_iter().map(|i| {
            // deterministic per-sample seed
            let mut rng = StdRng::seed_from_u64(seed ^ (i as u64).wrapping_mul(0x9E3779B97F4A7C15));
            let n = sample_number(p, &spec, &mut rng);

            // track divisibility in one pass
            let mut div = vec![0usize; track.len()];
            let mut blocked = false;
            for (j, &m) in track.iter().enumerate() {
                if divisible_by_small(&n, m) {
                    div[j] = 1;
                    if pre_sieve && m >= 2 { blocked = true; }
                }
            }
            let is_prime = if pre_sieve && blocked { 0usize } else { is_probable_prime(&n) as usize };
            (is_prime, div)
        })
        .reduce(
            || (0usize, vec![0usize; track.len()]),
            |(p1, mut c1), (p2, c2)| {
                for (i, v) in c2.iter().enumerate() { c1[i] += *v; }
                (p1 + p2, c1)
            },
        )
    } else {
        let mut primes = 0usize;
        let mut counts = vec![0usize; track.len()];
        for i in 0..samples {
            let mut rng = StdRng::seed_from_u64(seed ^ (i as u64).wrapping_mul(0x9E3779B97F4A7C15));
            let n = sample_number(p, &spec, &mut rng);
            let mut blocked = false;
            for (j, &m) in track.iter().enumerate() {
                if divisible_by_small(&n, m) {
                    counts[j] += 1;
                    if pre_sieve && m >= 2 { blocked = true; }
                }
            }
            if !(pre_sieve && blocked) && is_probable_prime(&n) { primes += 1; }
        }
        (primes, counts)
    };
    let elapsed = start.elapsed().as_millis();
    let density = primes as f64 / samples as f64;
    let (lo, hi) = wilson_ci(primes, samples, 1.96);
    let enrichment = if expected > 0.0 { density / expected } else if primes == 0 { 0.0 } else { f64::INFINITY };

    SampleReport {
        pattern: format_pattern(p),
        base: p.base,
        total_len,
        mid_len,
        inner_zero,
        samples,
        primes,
        prime_density: density,
        ci_lo: lo,
        ci_hi: hi,
        expected_density_pnt_cond: expected,
        enrichment_factor: enrichment,
        elapsed_ms: elapsed,
        tracked_moduli: track.to_vec(),
        divisible_counts: counts,
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Sample { midpoint, layers, samples, allowed_last_digits, out_json, out_csv, parallel, mirror, track_moduli, pre_sieve } => {
            let mut track = parse_digit_list(&track_moduli);
            // auto-track b+1 for palindrome wall
            if mirror {
                let m = cli.base + 1;
                if !track.contains(&m) { track.push(m); }
            }

            let p = Pattern {
                base: cli.base,
                midpoint: parse_midpoint(&midpoint),
                layers: parse_layers(&layers),
                constraints: Constraints {
                    allowed_last_digits: parse_digit_list(&allowed_last_digits),
                    forbid_leading_zero: true,
                },
                mirror,
            };

            let report = do_sample(&p, samples, cli.seed, parallel, &track, pre_sieve);

            if let Some(path) = out_json.as_ref() {
                let f = File::create(path).expect("create json");
                serde_json::to_writer_pretty(f, &report).expect("write json");
            } else {
                println!("{}", serde_json::to_string_pretty(&report).unwrap());
            }

            if let Some(path) = out_csv.as_ref() {
                let f = File::create(path).expect("create csv");
                let mut w = BufWriter::new(f);
                write_csv_header(&mut w);
                write_csv_row(&mut w, &report);
                w.flush().unwrap();
            }
        }

        Commands::Grid { mid_kind, mid_len_range, inner_zero_range, inner_slot, outer_layers, samples, allowed_last_digits, out_csv, parallel, mirror, track_moduli, pre_sieve } => {
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
            let mid_kind_lower = mid_kind.to_lowercase();
            assert!(mid_kind_lower == "free" || mid_kind_lower == "zeros");

            let outer = parse_layers(&outer_layers);

            let f = File::create(out_csv).expect("create csv");
            let mut w = BufWriter::new(f);
            write_csv_header(&mut w);

            let mut track = parse_digit_list(&track_moduli);
            if mirror {
                let m = cli.base + 1;
                if !track.contains(&m) { track.push(m); }
            }

            let combos: Vec<(usize, usize)> =
                (mid_a..=mid_b).flat_map(|m| (iz_a..=iz_b).map(move |z| (m, z))).collect();

            let results: Vec<SampleReport> = if parallel {
                combos.par_iter().map(|(m, z)| {
                    let mut layers = vec![Layer { zero: *z, slot: inner_slot }];
                    layers.extend_from_slice(&outer);
                    let p = Pattern {
                        base: cli.base,
                        midpoint: if mid_kind_lower == "free" { Midpoint::Free(*m) } else { Midpoint::Zeros(*m) },
                        layers,
                        constraints: Constraints {
                            allowed_last_digits: parse_digit_list(&allowed_last_digits),
                            forbid_leading_zero: true,
                        },
                        mirror,
                    };
                    do_sample(&p, samples, cli.seed ^ ((*m as u64) << 32) ^ (*z as u64), parallel, &track, pre_sieve)
                }).collect()
            } else {
                combos.iter().map(|(m,z)| {
                    let mut layers = vec![Layer { zero: *z, slot: inner_slot }];
                    layers.extend_from_slice(&outer);
                    let p = Pattern {
                        base: cli.base,
                        midpoint: if mid_kind_lower == "free" { Midpoint::Free(*m) } else { Midpoint::Zeros(*m) },
                        layers,
                        constraints: Constraints {
                            allowed_last_digits: parse_digit_list(&allowed_last_digits),
                            forbid_leading_zero: true,
                        },
                        mirror,
                    };
                    do_sample(&p, samples, cli.seed ^ ((*m as u64) << 32) ^ (*z as u64), parallel, &track, pre_sieve)
                }).collect()
            };

            for r in &results { write_csv_row(&mut w, r); }
            w.flush().unwrap();
        }
    }
}
