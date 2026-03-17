//! Exact probe for two nearby questions:
//!
//! 1. How much of membrane prime density overlaps with ordinary palindrome behavior?
//! 2. Does mirror-symmetric zero placement outperform nearby layouts that keep
//!    the same digits and total zero budget?
//!
//! This keeps the core membrane idea front and center:
//! outer + 0^k1 + inner + 0^k2 + middle + 0^k2 + inner + 0^k1 + outer
//!
//! The palindrome split is only one lens. The stronger structural test is to
//! compare the canonical mirror-symmetric layout against other four-slot zero
//! layouts built from the same outer/inner digits and the same total amount of
//! zero padding.

use num_bigint::BigUint;
use primes::is_prime;
use std::cmp::Ordering;

#[derive(Clone, Copy)]
struct ProbeConfig<'a> {
    label: &'a str,
    base: u32,
    outer: u32,
    inner: u32,
    k_outer: u32,
    k_inner: u32,
    max_seed_len: usize,
}

#[derive(Default)]
struct BucketStats {
    candidates: usize,
    primes: usize,
    examples: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ZeroLayout {
    left_outer: u32,
    left_inner: u32,
    right_inner: u32,
    right_outer: u32,
}

#[derive(Clone)]
struct LayoutStats {
    layout: ZeroLayout,
    candidates: usize,
    primes: usize,
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║               MEMBRANE STRUCTURE PROBE                          ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("Exact enumeration over seed spaces.");
    println!("Question 1: does membrane prime density survive outside the palindromic subset?");
    println!("Question 2: does symmetric zero-padding outperform nearby broken-symmetry layouts?");
    println!();

    let configs = [
        ProbeConfig {
            label: "Base 10 exclusive",
            base: 10,
            outer: 3,
            inner: 7,
            k_outer: 1,
            k_inner: 1,
            max_seed_len: 4,
        },
        ProbeConfig {
            label: "Base 10 stretched",
            base: 10,
            outer: 3,
            inner: 7,
            k_outer: 2,
            k_inner: 1,
            max_seed_len: 4,
        },
        ProbeConfig {
            label: "Base 10 breathing",
            base: 10,
            outer: 3,
            inner: 3,
            k_outer: 0,
            k_inner: 1,
            max_seed_len: 4,
        },
        ProbeConfig {
            label: "Base 6 champion",
            base: 6,
            outer: 1,
            inner: 5,
            k_outer: 0,
            k_inner: 0,
            max_seed_len: 5,
        },
    ];

    for cfg in configs {
        analyze_palindrome_overlap(cfg);
        analyze_zero_layout_symmetry(cfg);
    }
}

fn analyze_palindrome_overlap(cfg: ProbeConfig<'_>) {
    println!("{}", "═".repeat(78));
    println!("PALINDROME OVERLAP");
    println!(
        "{}  |  base {}  ({},{}) k=({},{})",
        cfg.label, cfg.base, cfg.outer, cfg.inner, cfg.k_outer, cfg.k_inner
    );
    println!("{}", "═".repeat(78));
    println!(
        "{:>7} {:>8} {:>10} {:>12} {:>12} {:>12} {:>11}",
        "seedlen", "digits", "family", "all dens", "pal dens", "nonpal dens", "pal share"
    );
    println!("{}", "-".repeat(78));

    for seed_len in 1..=cfg.max_seed_len {
        let family_size = (cfg.base as usize).pow(seed_len as u32);
        let mut all = BucketStats::default();
        let mut pal = BucketStats::default();
        let mut nonpal = BucketStats::default();
        let mut total_digits = None;

        for seed in 0..family_size {
            let seed_str = to_base_string_padded(seed as u32, cfg.base, seed_len);
            let membrane = build_membrane_string(cfg, &seed_str);
            let decimal = BigUint::parse_bytes(membrane.as_bytes(), cfg.base).unwrap();
            let prime = is_prime(&decimal);
            let membrane_is_palindrome = is_palindrome(&membrane);

            if total_digits.is_none() {
                total_digits = Some(membrane.chars().count());
            }

            all.candidates += 1;
            if prime {
                all.primes += 1;
                if all.examples.len() < 3 {
                    all.examples.push(membrane.clone());
                }
            }

            let bucket = if membrane_is_palindrome {
                &mut pal
            } else {
                &mut nonpal
            };

            bucket.candidates += 1;
            if prime {
                bucket.primes += 1;
                if bucket.examples.len() < 3 {
                    bucket.examples.push(membrane);
                }
            }
        }

        let all_density = density(&all);
        let pal_density = density(&pal);
        let nonpal_density = density(&nonpal);
        let pal_share = if all.primes > 0 {
            pal.primes as f64 / all.primes as f64
        } else {
            0.0
        };

        println!(
            "{:>7} {:>8} {:>10} {:>11.2}% {:>11.2}% {:>11.2}% {:>10.1}%",
            seed_len,
            total_digits.unwrap_or(0),
            family_size,
            all_density * 100.0,
            pal_density * 100.0,
            nonpal_density * 100.0,
            pal_share * 100.0
        );

        if seed_len == 2 || seed_len == 3 {
            if !pal.examples.is_empty() || !nonpal.examples.is_empty() {
                println!("         pal primes:    {:?}", pal.examples);
                println!("         nonpal primes: {:?}", nonpal.examples);
            }
        }

        if total_digits.unwrap_or(0) % 2 == 0 && pal.primes == 0 && pal.candidates > 0 {
            println!(
                "         note: palindromic subset vanished at even total length (base {} => divisibility by base+1 heuristic applies).",
                cfg.base
            );
        }
    }

    println!();
}

fn analyze_zero_layout_symmetry(cfg: ProbeConfig<'_>) {
    let total_zero_budget = 2 * cfg.k_outer + 2 * cfg.k_inner;

    println!("{}", "─".repeat(78));
    println!("ZERO-LAYOUT SYMMETRY");
    println!(
        "{}  |  base {}  canonical layout {}  |  total zero budget {}",
        cfg.label,
        cfg.base,
        canonical_layout(cfg).label(),
        total_zero_budget
    );
    println!("{}", "─".repeat(78));

    if total_zero_budget == 0 {
        println!("No zero-padding to permute; the canonical layout is the only layout.");
        println!();
        return;
    }

    let layouts = enumerate_zero_layouts(total_zero_budget);

    for seed_len in 1..=cfg.max_seed_len {
        let mut results = Vec::with_capacity(layouts.len());

        for &layout in &layouts {
            results.push(evaluate_layout(cfg, layout, seed_len));
        }

        results.sort_by(compare_layout_stats);

        let canonical = canonical_layout(cfg);
        let canonical_rank = results
            .iter()
            .position(|stats| stats.layout == canonical)
            .map(|idx| idx + 1)
            .unwrap_or(0);
        let canonical_stats = results
            .iter()
            .find(|stats| stats.layout == canonical)
            .cloned()
            .unwrap();

        let mirror_layouts: Vec<&LayoutStats> = results
            .iter()
            .filter(|stats| stats.layout.is_mirror_symmetric())
            .collect();
        let asymmetric_layouts: Vec<&LayoutStats> = results
            .iter()
            .filter(|stats| !stats.layout.is_mirror_symmetric())
            .collect();

        let mirror_avg = average_density(&mirror_layouts);
        let asymmetric_avg = average_density(&asymmetric_layouts);
        let best = &results[0];
        let digits = total_digits(cfg, seed_len);

        println!(
            "seed_len {:>2} | digits {:>2} | layouts {:>2} | canonical {:>7.2}% rank {:>2}/{:<2} | mirror avg {:>7.2}% | asym avg {:>7.2}% | best {} {:>7.2}%",
            seed_len,
            digits,
            results.len(),
            density_counts(canonical_stats.primes, canonical_stats.candidates) * 100.0,
            canonical_rank,
            results.len(),
            mirror_avg * 100.0,
            asymmetric_avg * 100.0,
            best.layout.label(),
            density_counts(best.primes, best.candidates) * 100.0
        );

        if seed_len == 2 || seed_len == 3 {
            let top_layouts: Vec<String> = results
                .iter()
                .take(3)
                .map(|stats| {
                    format!(
                        "{}={:.2}%",
                        stats.layout.label(),
                        density_counts(stats.primes, stats.candidates) * 100.0
                    )
                })
                .collect();
            println!("           top layouts: {}", top_layouts.join(", "));
        }
    }

    println!();
}

fn density(stats: &BucketStats) -> f64 {
    if stats.candidates == 0 {
        0.0
    } else {
        stats.primes as f64 / stats.candidates as f64
    }
}

fn build_membrane_string(cfg: ProbeConfig<'_>, seed_str: &str) -> String {
    build_layout_string(cfg, seed_str, canonical_layout(cfg))
}

fn to_base_string_padded(mut n: u32, base: u32, width: usize) -> String {
    let mut out = String::new();
    for _ in 0..width {
        let digit = n % base;
        let ch = if digit < 10 {
            char::from_digit(digit, 10).unwrap()
        } else {
            char::from_u32('A' as u32 + digit - 10).unwrap()
        };
        out.insert(0, ch);
        n /= base;
    }
    out
}

fn to_base_string(mut n: u32, base: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    while n > 0 {
        let digit = n % base;
        let ch = if digit < 10 {
            char::from_digit(digit, 10).unwrap()
        } else {
            char::from_u32('A' as u32 + digit - 10).unwrap()
        };
        result.insert(0, ch);
        n /= base;
    }
    result
}

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn canonical_layout(cfg: ProbeConfig<'_>) -> ZeroLayout {
    ZeroLayout {
        left_outer: cfg.k_outer,
        left_inner: cfg.k_inner,
        right_inner: cfg.k_inner,
        right_outer: cfg.k_outer,
    }
}

fn evaluate_layout(cfg: ProbeConfig<'_>, layout: ZeroLayout, seed_len: usize) -> LayoutStats {
    let family_size = (cfg.base as usize).pow(seed_len as u32);
    let mut primes = 0;

    for seed in 0..family_size {
        let seed_str = to_base_string_padded(seed as u32, cfg.base, seed_len);
        let membrane = build_layout_string(cfg, &seed_str, layout);
        let decimal = BigUint::parse_bytes(membrane.as_bytes(), cfg.base).unwrap();
        if is_prime(&decimal) {
            primes += 1;
        }
    }

    LayoutStats {
        layout,
        candidates: family_size,
        primes,
    }
}

fn build_layout_string(cfg: ProbeConfig<'_>, seed_str: &str, layout: ZeroLayout) -> String {
    let outer = to_base_string(cfg.outer, cfg.base);
    let inner = to_base_string(cfg.inner, cfg.base);

    format!(
        "{}{}{}{}{}{}{}{}{}",
        outer,
        "0".repeat(layout.left_outer as usize),
        inner,
        "0".repeat(layout.left_inner as usize),
        seed_str,
        "0".repeat(layout.right_inner as usize),
        inner,
        "0".repeat(layout.right_outer as usize),
        outer
    )
}

fn enumerate_zero_layouts(total_zero_budget: u32) -> Vec<ZeroLayout> {
    let mut layouts = Vec::new();

    for left_outer in 0..=total_zero_budget {
        for left_inner in 0..=(total_zero_budget - left_outer) {
            for right_inner in 0..=(total_zero_budget - left_outer - left_inner) {
                let right_outer = total_zero_budget - left_outer - left_inner - right_inner;
                layouts.push(ZeroLayout {
                    left_outer,
                    left_inner,
                    right_inner,
                    right_outer,
                });
            }
        }
    }

    layouts
}

fn average_density(layouts: &[&LayoutStats]) -> f64 {
    if layouts.is_empty() {
        return 0.0;
    }

    layouts
        .iter()
        .map(|stats| density_counts(stats.primes, stats.candidates))
        .sum::<f64>()
        / layouts.len() as f64
}

fn density_counts(primes: usize, candidates: usize) -> f64 {
    if candidates == 0 {
        0.0
    } else {
        primes as f64 / candidates as f64
    }
}

fn total_digits(cfg: ProbeConfig<'_>, seed_len: usize) -> usize {
    let outer_len = to_base_string(cfg.outer, cfg.base).len();
    let inner_len = to_base_string(cfg.inner, cfg.base).len();
    2 * outer_len
        + 2 * inner_len
        + (2 * cfg.k_outer + 2 * cfg.k_inner) as usize
        + seed_len
}

fn compare_layout_stats(left: &LayoutStats, right: &LayoutStats) -> Ordering {
    density_counts(right.primes, right.candidates)
        .partial_cmp(&density_counts(left.primes, left.candidates))
        .unwrap_or(Ordering::Equal)
        .then_with(|| left.layout.cmp_key().cmp(&right.layout.cmp_key()))
}

impl ZeroLayout {
    fn is_mirror_symmetric(self) -> bool {
        self.left_outer == self.right_outer && self.left_inner == self.right_inner
    }

    fn label(self) -> String {
        format!(
            "[{},{},{},{}]",
            self.left_outer, self.left_inner, self.right_inner, self.right_outer
        )
    }

    fn cmp_key(self) -> (u32, u32, u32, u32) {
        (
            self.left_outer,
            self.left_inner,
            self.right_inner,
            self.right_outer,
        )
    }
}
