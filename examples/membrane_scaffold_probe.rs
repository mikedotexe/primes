//! Exact probe for the centered membrane scaffold.
//!
//! The question here is narrower than "are membranes just palindromes?"
//! We keep the same:
//! - base
//! - anchor digit multiset {outer, outer, inner, inner}
//! - total zero budget
//! - seed family
//!
//! Then we compare:
//! - centered zero layouts (symmetric gaps about the middle)
//! - asymmetric same-budget layouts
//! - the canonical membrane template itself
//!
//! This tests whether symmetric zero-padding about a middle carries signal
//! beyond nearby templates that preserve the same digits and coprimality.

use num_bigint::BigUint;
use primal::is_prime as is_prime_u64;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Template {
    anchors: [u32; 4],
    gaps: [u32; 4],
}

#[derive(Clone)]
struct TemplateStats {
    template: Template,
    candidates: usize,
    primes: usize,
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║              MEMBRANE SCAFFOLD CONTROL PROBE                    ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("Exact enumeration over same-budget template families.");
    println!("Question: does symmetric zero-padding about a middle outperform nearby controls?");
    println!();

    let configs = [
        ProbeConfig {
            label: "Base 10 exclusive",
            base: 10,
            outer: 3,
            inner: 7,
            k_outer: 1,
            k_inner: 1,
            max_seed_len: 3,
        },
        ProbeConfig {
            label: "Base 10 stretched",
            base: 10,
            outer: 3,
            inner: 7,
            k_outer: 2,
            k_inner: 1,
            max_seed_len: 3,
        },
        ProbeConfig {
            label: "Base 10 breathing",
            base: 10,
            outer: 3,
            inner: 3,
            k_outer: 0,
            k_inner: 1,
            max_seed_len: 3,
        },
    ];

    for cfg in configs {
        analyze_config(cfg);
    }

    analyze_spacing_scaffold();
}

fn analyze_config(cfg: ProbeConfig<'_>) {
    let zero_budget = 2 * cfg.k_outer + 2 * cfg.k_inner;
    let templates = enumerate_templates(cfg);
    let canonical = canonical_template(cfg);

    println!("{}", "═".repeat(78));
    println!(
        "{}  |  base {}  canonical {}  |  zero budget {}",
        cfg.label,
        cfg.base,
        canonical.label(cfg.base),
        zero_budget
    );
    println!("{}", "═".repeat(78));

    for seed_len in 1..=cfg.max_seed_len {
        let mut results = Vec::with_capacity(templates.len());
        for &template in &templates {
            results.push(evaluate_template(cfg, template, seed_len));
        }

        results.sort_by(compare_template_stats);

        let canonical_rank = results
            .iter()
            .position(|stats| stats.template == canonical)
            .map(|idx| idx + 1)
            .unwrap_or(0);
        let canonical_stats = results
            .iter()
            .find(|stats| stats.template == canonical)
            .cloned()
            .unwrap();

        let centered: Vec<&TemplateStats> = results
            .iter()
            .filter(|stats| stats.template.is_centered())
            .collect();
        let asymmetric: Vec<&TemplateStats> = results
            .iter()
            .filter(|stats| !stats.template.is_centered())
            .collect();
        let centered_mirror: Vec<&TemplateStats> = results
            .iter()
            .filter(|stats| stats.template.is_centered() && stats.template.has_mirrored_anchors())
            .collect();
        let centered_nonmirror: Vec<&TemplateStats> = results
            .iter()
            .filter(|stats| stats.template.is_centered() && !stats.template.has_mirrored_anchors())
            .collect();

        let centered_avg = average_density(&centered);
        let asym_avg = average_density(&asymmetric);
        let centered_mirror_avg = average_density(&centered_mirror);
        let centered_nonmirror_avg = average_density(&centered_nonmirror);
        let best_centered = centered[0];
        let best_asym = asymmetric[0];

        println!(
            "seed_len {:>2} | templates {:>3} | canonical {:>6.2}% rank {:>3}/{:<3} | centered avg {:>6.2}% | asym avg {:>6.2}% | centered mirror {:>6.2}% | centered nonmirror {:>6.2}%",
            seed_len,
            results.len(),
            density_counts(canonical_stats.primes, canonical_stats.candidates) * 100.0,
            canonical_rank,
            results.len(),
            centered_avg * 100.0,
            asym_avg * 100.0,
            centered_mirror_avg * 100.0,
            centered_nonmirror_avg * 100.0
        );

        println!(
            "           best centered {} {:>6.2}% | best asym {} {:>6.2}%",
            best_centered.template.label(cfg.base),
            density_counts(best_centered.primes, best_centered.candidates) * 100.0,
            best_asym.template.label(cfg.base),
            density_counts(best_asym.primes, best_asym.candidates) * 100.0
        );
    }

    println!();
}

fn analyze_spacing_scaffold() {
    let configs = [
        SpacingConfig {
            label: "Base 10 spacing",
            base: 10,
            zero_budget: 2,
            max_seed_len: 2,
        },
        SpacingConfig {
            label: "Base 10 spacing",
            base: 10,
            zero_budget: 4,
            max_seed_len: 2,
        },
        SpacingConfig {
            label: "Base 6 spacing",
            base: 6,
            zero_budget: 2,
            max_seed_len: 2,
        },
    ];

    println!("{}", "═".repeat(78));
    println!("INDEPENDENT-DIGIT SPACING CONTROL");
    println!("{}", "═".repeat(78));
    println!("Four open slots around a middle, with only the zero-gap pattern changing.");
    println!("Boundary digits are restricted to units mod base; inner digits and seed vary freely.");
    println!();

    for cfg in configs {
        analyze_spacing_config(cfg);
    }
}

#[derive(Clone, Copy)]
struct SpacingConfig<'a> {
    label: &'a str,
    base: u32,
    zero_budget: u32,
    max_seed_len: usize,
}

#[derive(Clone)]
struct GapStats {
    gaps: [u32; 4],
    candidates: usize,
    primes: usize,
}

fn analyze_spacing_config(cfg: SpacingConfig<'_>) {
    let gaps = gap_vectors(cfg.zero_budget);
    let boundary_digits = coprime_digits(cfg.base);

    println!(
        "{}  |  base {}  |  zero budget {}  |  boundary units {:?}",
        cfg.label, cfg.base, cfg.zero_budget, boundary_digits
    );

    for seed_len in 1..=cfg.max_seed_len {
        let mut results = Vec::with_capacity(gaps.len());

        for gap_pattern in &gaps {
            results.push(evaluate_spacing_gaps(cfg, *gap_pattern, &boundary_digits, seed_len));
        }

        results.sort_by(compare_gap_stats);

        let centered: Vec<&GapStats> = results.iter().filter(|stats| is_centered_gaps(stats.gaps)).collect();
        let asymmetric: Vec<&GapStats> = results.iter().filter(|stats| !is_centered_gaps(stats.gaps)).collect();
        let centered_avg = average_gap_density(&centered);
        let asym_avg = average_gap_density(&asymmetric);
        let best_centered = centered[0];
        let best_asym = asymmetric[0];

        println!(
            "seed_len {:>2} | gap templates {:>2} | centered avg {:>6.2}% | asym avg {:>6.2}% | best centered gaps={:?} {:>6.2}% | best asym gaps={:?} {:>6.2}%",
            seed_len,
            results.len(),
            centered_avg * 100.0,
            asym_avg * 100.0,
            best_centered.gaps,
            density_counts(best_centered.primes, best_centered.candidates) * 100.0,
            best_asym.gaps,
            density_counts(best_asym.primes, best_asym.candidates) * 100.0
        );
    }

    println!();
}

fn evaluate_spacing_gaps(
    cfg: SpacingConfig<'_>,
    gaps: [u32; 4],
    boundary_digits: &[u32],
    seed_len: usize,
) -> GapStats {
    let family_size = boundary_digits.len()
        * cfg.base as usize
        * cfg.base as usize
        * boundary_digits.len()
        * (cfg.base as usize).pow(seed_len as u32);
    let mut primes = 0;
    let base_u64 = cfg.base as u64;

    for &left_outer in boundary_digits {
        for left_inner in 0..cfg.base {
            for seed in 0..(cfg.base as usize).pow(seed_len as u32) {
                let seed_digits = digits_padded(seed as u32, cfg.base, seed_len);
                for right_inner in 0..cfg.base {
                    for &right_outer in boundary_digits {
                        let value = build_spacing_value(
                            base_u64,
                            gaps,
                            left_outer,
                            left_inner,
                            &seed_digits,
                            right_inner,
                            right_outer,
                        );
                        if is_prime_u64(value) {
                            primes += 1;
                        }
                    }
                }
            }
        }
    }

    GapStats {
        gaps,
        candidates: family_size,
        primes,
    }
}

fn evaluate_template(cfg: ProbeConfig<'_>, template: Template, seed_len: usize) -> TemplateStats {
    let family_size = (cfg.base as usize).pow(seed_len as u32);
    let mut primes = 0;

    for seed in 0..family_size {
        let seed_str = to_base_string_padded(seed as u32, cfg.base, seed_len);
        let value = build_template_string(cfg.base, template, &seed_str);
        let decimal = BigUint::parse_bytes(value.as_bytes(), cfg.base).unwrap();
        if is_prime(&decimal) {
            primes += 1;
        }
    }

    TemplateStats {
        template,
        candidates: family_size,
        primes,
    }
}

fn enumerate_templates(cfg: ProbeConfig<'_>) -> Vec<Template> {
    let anchors = unique_anchor_patterns(cfg.outer, cfg.inner);
    let gaps = gap_vectors(2 * cfg.k_outer + 2 * cfg.k_inner);
    let mut out = Vec::with_capacity(anchors.len() * gaps.len());

    for anchor_pattern in anchors {
        for gap_pattern in &gaps {
            out.push(Template {
                anchors: anchor_pattern,
                gaps: *gap_pattern,
            });
        }
    }

    out
}

fn canonical_template(cfg: ProbeConfig<'_>) -> Template {
    Template {
        anchors: [cfg.outer, cfg.inner, cfg.inner, cfg.outer],
        gaps: [cfg.k_outer, cfg.k_inner, cfg.k_inner, cfg.k_outer],
    }
}

fn unique_anchor_patterns(outer: u32, inner: u32) -> Vec<[u32; 4]> {
    if outer == inner {
        return vec![[outer, outer, outer, outer]];
    }

    vec![
        [outer, outer, inner, inner],
        [outer, inner, outer, inner],
        [outer, inner, inner, outer],
        [inner, outer, outer, inner],
        [inner, outer, inner, outer],
        [inner, inner, outer, outer],
    ]
}

fn gap_vectors(total: u32) -> Vec<[u32; 4]> {
    let mut out = Vec::new();

    for g0 in 0..=total {
        for g1 in 0..=(total - g0) {
            for g2 in 0..=(total - g0 - g1) {
                let g3 = total - g0 - g1 - g2;
                out.push([g0, g1, g2, g3]);
            }
        }
    }

    out
}

fn build_template_string(base: u32, template: Template, seed_str: &str) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}",
        to_base_string(template.anchors[0], base),
        "0".repeat(template.gaps[0] as usize),
        to_base_string(template.anchors[1], base),
        "0".repeat(template.gaps[1] as usize),
        seed_str,
        "0".repeat(template.gaps[2] as usize),
        to_base_string(template.anchors[2], base),
        "0".repeat(template.gaps[3] as usize),
        to_base_string(template.anchors[3], base)
    )
}

fn average_density(stats: &[&TemplateStats]) -> f64 {
    if stats.is_empty() {
        return 0.0;
    }

    stats
        .iter()
        .map(|entry| density_counts(entry.primes, entry.candidates))
        .sum::<f64>()
        / stats.len() as f64
}

fn average_gap_density(stats: &[&GapStats]) -> f64 {
    if stats.is_empty() {
        return 0.0;
    }

    stats
        .iter()
        .map(|entry| density_counts(entry.primes, entry.candidates))
        .sum::<f64>()
        / stats.len() as f64
}

fn density_counts(primes: usize, candidates: usize) -> f64 {
    if candidates == 0 {
        0.0
    } else {
        primes as f64 / candidates as f64
    }
}

fn compare_template_stats(left: &TemplateStats, right: &TemplateStats) -> Ordering {
    density_counts(right.primes, right.candidates)
        .partial_cmp(&density_counts(left.primes, left.candidates))
        .unwrap_or(Ordering::Equal)
        .then_with(|| left.template.cmp_key().cmp(&right.template.cmp_key()))
}

fn compare_gap_stats(left: &GapStats, right: &GapStats) -> Ordering {
    density_counts(right.primes, right.candidates)
        .partial_cmp(&density_counts(left.primes, left.candidates))
        .unwrap_or(Ordering::Equal)
        .then_with(|| left.gaps.cmp(&right.gaps))
}

fn is_centered_gaps(gaps: [u32; 4]) -> bool {
    gaps[0] == gaps[3] && gaps[1] == gaps[2]
}

impl Template {
    fn is_centered(self) -> bool {
        self.gaps[0] == self.gaps[3] && self.gaps[1] == self.gaps[2]
    }

    fn has_mirrored_anchors(self) -> bool {
        self.anchors[0] == self.anchors[3] && self.anchors[1] == self.anchors[2]
    }

    fn cmp_key(self) -> ([u32; 4], [u32; 4]) {
        (self.anchors, self.gaps)
    }

    fn label(self, base: u32) -> String {
        format!(
            "[{} {} {} {}] gaps={:?}",
            to_base_string(self.anchors[0], base),
            to_base_string(self.anchors[1], base),
            to_base_string(self.anchors[2], base),
            to_base_string(self.anchors[3], base),
            self.gaps
        )
    }
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

fn digits_padded(mut n: u32, base: u32, width: usize) -> Vec<u32> {
    let mut out = vec![0; width];
    for idx in (0..width).rev() {
        out[idx] = n % base;
        n /= base;
    }
    out
}

fn coprime_digits(base: u32) -> Vec<u32> {
    (1..base).filter(|&digit| gcd(digit, base) == 1).collect()
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn build_spacing_value(
    base: u64,
    gaps: [u32; 4],
    left_outer: u32,
    left_inner: u32,
    seed_digits: &[u32],
    right_inner: u32,
    right_outer: u32,
) -> u64 {
    let mut value = left_outer as u64;
    value *= base.pow(gaps[0]);
    value = value * base + left_inner as u64;
    value *= base.pow(gaps[1]);
    for &digit in seed_digits {
        value = value * base + digit as u64;
    }
    value *= base.pow(gaps[2]);
    value = value * base + right_inner as u64;
    value *= base.pow(gaps[3]);
    value = value * base + right_outer as u64;
    value
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
