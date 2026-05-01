//! Seed-to-witness demo helpers.
//!
//! A seed in this module is a starting point on an affine membrane lane, not a
//! promise that the exact seed is prime. The search walks forward until the
//! first residue-admissible probable-prime witness is found.

use crate::validation::large_affine_witness::{
    build_big_affine_lane, candidate_value, classify_mersenne, compact_description,
    is_probable_prime_fixed_bases, middle_digits, residue_allows_seed, residue_moduli,
    template_digits, BigAffineLane, PRIMARY_BASE, PRIMARY_INNER, PRIMARY_K, PRIMARY_OUTER,
    PROBABLE_PRIME_BASES,
};
use num_bigint::BigUint;
use serde::Serialize;
use std::{error::Error, fmt, time::Instant};

pub const DEFAULT_VISIBLE_DIGITS: usize = 128;
pub const DEFAULT_MAX_STEPS: u64 = 20_000;

#[derive(Debug, Clone, Serialize)]
pub struct SeedToWitnessConfig {
    pub input_seed: u64,
    pub max_steps: u64,
    pub exact_seed_only: bool,
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub k_outer: u32,
    pub k_inner: u32,
    pub visible_digits: usize,
    pub probable_prime_bases: Vec<u64>,
}

impl SeedToWitnessConfig {
    pub fn default_for_seed(input_seed: u64) -> Self {
        Self {
            input_seed,
            max_steps: DEFAULT_MAX_STEPS,
            exact_seed_only: false,
            base: PRIMARY_BASE,
            outer: PRIMARY_OUTER,
            inner: PRIMARY_INNER,
            k_outer: PRIMARY_K.0,
            k_inner: PRIMARY_K.1,
            visible_digits: DEFAULT_VISIBLE_DIGITS,
            probable_prime_bases: PROBABLE_PRIME_BASES.to_vec(),
        }
    }

    pub fn with_visible_digits(mut self, visible_digits: usize) -> Self {
        self.visible_digits = visible_digits;
        self
    }

    pub fn with_exact_seed_only(mut self, exact_seed_only: bool) -> Self {
        self.exact_seed_only = exact_seed_only;
        self
    }

    pub fn with_max_steps(mut self, max_steps: u64) -> Self {
        self.max_steps = max_steps;
        self
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SeedToWitnessResult {
    pub input_seed: u64,
    pub witness_seed: u64,
    pub steps_to_witness: u64,
    pub exact_seed_only: bool,
    pub max_steps: u64,
    pub scanned_seed_count: u64,
    pub residue_survivor_count: u64,
    pub residue_rejected_count: u64,
    pub probable_prime_tests: u64,
    pub elapsed_seconds: f64,
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub k_outer: u32,
    pub k_inner: u32,
    pub middle_length: usize,
    pub visible_digits: usize,
    pub residue_moduli_label: String,
    pub shift: String,
    pub gradient: String,
    pub affine_line: String,
    pub middle_digits: String,
    pub template_digits: String,
    pub decimal_value: String,
    pub decimal_digits: usize,
    pub compact_description: String,
    pub confirmation: String,
    pub is_mersenne: bool,
    pub mersenne_exponent: Option<u64>,
    pub mersenne_class: String,
    pub verification_snippets: Vec<VerificationSnippet>,
}

#[derive(Debug, Clone, Serialize)]
pub struct VerificationSnippet {
    pub tool: String,
    pub snippet: String,
}

#[derive(Debug, Clone)]
struct SearchHitStats {
    steps_to_witness: u64,
    scanned_seed_count: u64,
    residue_survivor_count: u64,
    probable_prime_tests: u64,
    elapsed_seconds: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeedToWitnessError {
    VisibleDigitsTooSmall {
        visible_digits: usize,
        fixed_digits: usize,
    },
    SeedRangeOverflow {
        input_seed: u64,
        max_steps: u64,
    },
    NoWitnessFound {
        input_seed: u64,
        max_steps: u64,
        exact_seed_only: bool,
        scanned_seed_count: u64,
        residue_survivor_count: u64,
    },
}

impl fmt::Display for SeedToWitnessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VisibleDigitsTooSmall {
                visible_digits,
                fixed_digits,
            } => write!(
                f,
                "visible digit target {visible_digits} is below fixed template width {fixed_digits}"
            ),
            Self::SeedRangeOverflow {
                input_seed,
                max_steps,
            } => write!(
                f,
                "seed search range overflows u64: start={input_seed}, max_steps={max_steps}"
            ),
            Self::NoWitnessFound {
                input_seed,
                max_steps,
                exact_seed_only,
                scanned_seed_count,
                residue_survivor_count,
            } => {
                if *exact_seed_only {
                    write!(
                        f,
                        "seed {input_seed} is not a witness in exact-seed-only mode"
                    )
                } else {
                    write!(
                        f,
                        "no witness found from seed {input_seed} within {max_steps} steps ({scanned_seed_count} scanned, {residue_survivor_count} residue survivors)"
                    )
                }
            }
        }
    }
}

impl Error for SeedToWitnessError {}

pub fn find_seed_to_witness(
    config: SeedToWitnessConfig,
) -> Result<SeedToWitnessResult, SeedToWitnessError> {
    let fixed_digits = fixed_template_digits((config.k_outer, config.k_inner));
    let middle_length = config.visible_digits.checked_sub(fixed_digits).ok_or(
        SeedToWitnessError::VisibleDigitsTooSmall {
            visible_digits: config.visible_digits,
            fixed_digits,
        },
    )?;
    let lane = build_big_affine_lane(
        config.base,
        config.outer,
        config.inner,
        middle_length,
        (config.k_outer, config.k_inner),
    );
    let scan_count = if config.exact_seed_only {
        1
    } else {
        config.max_steps
    };
    if scan_count > 0
        && config
            .input_seed
            .checked_add(scan_count.saturating_sub(1))
            .is_none()
    {
        return Err(SeedToWitnessError::SeedRangeOverflow {
            input_seed: config.input_seed,
            max_steps: config.max_steps,
        });
    }

    let moduli = residue_moduli(config.base);
    let started = Instant::now();
    let mut residue_survivors = 0u64;
    let mut probable_prime_tests = 0u64;

    for offset in 0..scan_count {
        let seed = config.input_seed + offset;
        if !residue_allows_seed(&lane, seed, &moduli) {
            continue;
        }
        residue_survivors += 1;
        probable_prime_tests += 1;
        let value = candidate_value(&lane, seed);
        if let Some(confirmation) = confirmation_label(&value, &config.probable_prime_bases) {
            let elapsed = started.elapsed().as_secs_f64().max(1e-12);
            return Ok(build_result(
                &config,
                &lane,
                seed,
                SearchHitStats {
                    steps_to_witness: offset,
                    scanned_seed_count: scan_count.min(offset + 1),
                    residue_survivor_count: residue_survivors,
                    probable_prime_tests,
                    elapsed_seconds: elapsed,
                },
                confirmation,
            ));
        }
    }

    Err(SeedToWitnessError::NoWitnessFound {
        input_seed: config.input_seed,
        max_steps: config.max_steps,
        exact_seed_only: config.exact_seed_only,
        scanned_seed_count: scan_count,
        residue_survivor_count: residue_survivors,
    })
}

pub fn render_seed_to_witness_transcript(result: &SeedToWitnessResult) -> String {
    let mut lines = Vec::new();
    lines.push("# Seed To Witness Transcript".to_string());
    lines.push(String::new());
    lines.push("## Construction".to_string());
    lines.push(format!("- input seed: `{}`", result.input_seed));
    lines.push(format!(
        "- search mode: `{}`",
        if result.exact_seed_only {
            "exact seed only"
        } else {
            "walk forward"
        }
    ));
    lines.push(format!(
        "- lane: base `{}`, pair `({}, {})`, k=`({}, {})`, M=`{}`",
        result.base,
        result.outer,
        result.inner,
        result.k_outer,
        result.k_inner,
        result.middle_length
    ));
    lines.push(format!("- visible digits: `{}`", result.visible_digits));
    lines.push(format!("- affine line: `{}`", result.affine_line));
    lines.push(String::new());
    lines.push("## Residue Funnel".to_string());
    lines.push(format!("- moduli: `{}`", result.residue_moduli_label));
    lines.push(format!(
        "- scanned: `{}` raw seeds -> `{}` residue survivors / probable-prime tests -> `1` witness",
        result.scanned_seed_count, result.residue_survivor_count
    ));
    lines.push(String::new());
    lines.push("## Witness".to_string());
    lines.push(format!("- witness seed: `{}`", result.witness_seed));
    lines.push(format!(
        "- steps from input seed: `{}`",
        result.steps_to_witness
    ));
    lines.push(format!("- middle digits: `{}`", result.middle_digits));
    lines.push(format!("- template: `{}`", result.template_digits));
    lines.push(format!("- decimal digits: `{}`", result.decimal_digits));
    lines.push(format!(
        "- compact description: `{}`",
        result.compact_description
    ));
    lines.push(format!("- Mersenne class: `{}`", result.mersenne_class));
    lines.push(String::new());
    lines.push("```text".to_string());
    lines.push(result.decimal_value.clone());
    lines.push("```".to_string());
    lines.push(String::new());
    lines.push("## Local Confirmation".to_string());
    lines.push(format!("- `{}`", result.confirmation));
    lines.push(
        "- Above `u64`, this repo says probable-prime witness unless a proof certificate is added."
            .to_string(),
    );
    lines.push(String::new());
    lines.push("## Copyable External Checks".to_string());
    for snippet in &result.verification_snippets {
        lines.push(format!("- {}: `{}`", snippet.tool, snippet.snippet));
    }
    lines.join("\n")
}

fn build_result(
    config: &SeedToWitnessConfig,
    lane: &BigAffineLane,
    witness_seed: u64,
    stats: SearchHitStats,
    confirmation: String,
) -> SeedToWitnessResult {
    let value = candidate_value(lane, witness_seed);
    let mersenne = classify_mersenne(&value);
    let decimal_value = value.to_str_radix(10);
    let residue_rejected_count = stats
        .scanned_seed_count
        .saturating_sub(stats.residue_survivor_count);
    SeedToWitnessResult {
        input_seed: config.input_seed,
        witness_seed,
        steps_to_witness: stats.steps_to_witness,
        exact_seed_only: config.exact_seed_only,
        max_steps: config.max_steps,
        scanned_seed_count: stats.scanned_seed_count,
        residue_survivor_count: stats.residue_survivor_count,
        residue_rejected_count,
        probable_prime_tests: stats.probable_prime_tests,
        elapsed_seconds: stats.elapsed_seconds,
        base: lane.base,
        outer: lane.outer,
        inner: lane.inner,
        k_outer: lane.k_outer,
        k_inner: lane.k_inner,
        middle_length: lane.middle_length,
        visible_digits: config.visible_digits,
        residue_moduli_label: join_moduli(&residue_moduli(lane.base)),
        shift: lane.shift.to_str_radix(10),
        gradient: lane.gradient.to_str_radix(10),
        affine_line: format!(
            "N(s) = {} + {}*s",
            lane.shift.to_str_radix(10),
            lane.gradient.to_str_radix(10)
        ),
        middle_digits: middle_digits(lane.base, lane.middle_length, witness_seed),
        template_digits: template_digits(lane, witness_seed),
        decimal_digits: decimal_value.len(),
        compact_description: compact_description(lane, witness_seed),
        is_mersenne: mersenne.is_mersenne,
        mersenne_exponent: mersenne.mersenne_exponent,
        mersenne_class: mersenne.mersenne_class,
        verification_snippets: verification_snippets(&decimal_value),
        decimal_value,
        confirmation,
    }
}

fn confirmation_label(value: &BigUint, bases: &[u64]) -> Option<String> {
    if let Ok(value_u64) = value.to_str_radix(10).parse::<u64>() {
        if primal::is_prime(value_u64) {
            return Some("deterministic_primal_u64".to_string());
        }
        return None;
    }
    if is_probable_prime_fixed_bases(value, bases) {
        Some(format!("probable_prime_fixed_{}_bases", bases.len()))
    } else {
        None
    }
}

fn verification_snippets(decimal_value: &str) -> Vec<VerificationSnippet> {
    vec![
        VerificationSnippet {
            tool: "WolframAlpha".to_string(),
            snippet: format!("isprime({decimal_value})"),
        },
        VerificationSnippet {
            tool: "Mathematica".to_string(),
            snippet: format!("PrimeQ[{decimal_value}]"),
        },
        VerificationSnippet {
            tool: "PARI/GP".to_string(),
            snippet: format!("isprime({decimal_value})"),
        },
        VerificationSnippet {
            tool: "Sage".to_string(),
            snippet: format!("is_prime(Integer(\"{decimal_value}\"))"),
        },
    ]
}

fn fixed_template_digits((k_outer, k_inner): (u32, u32)) -> usize {
    (4 + 2 * (k_outer + k_inner)) as usize
}

fn join_moduli(moduli: &[u32]) -> String {
    moduli
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_sixty_returns_known_128_digit_witness() {
        let result =
            find_seed_to_witness(SeedToWitnessConfig::default_for_seed(60)).expect("witness");
        assert_eq!(result.input_seed, 60);
        assert_eq!(result.witness_seed, 60);
        assert_eq!(result.steps_to_witness, 0);
        assert_eq!(result.visible_digits, 128);
        assert_eq!(result.decimal_digits, 128);
        assert!(result
            .decimal_value
            .starts_with("300700000000000000000000000000"));
        assert!(result.decimal_value.ends_with("0000000006007003"));
        assert_eq!(result.confirmation, "probable_prime_fixed_20_bases");
        assert!(!result.is_mersenne);
        assert_eq!(result.mersenne_class, "not_mersenne");
    }

    #[test]
    fn seed_zero_walks_to_reproducible_38_digit_witness() {
        let config = SeedToWitnessConfig::default_for_seed(0)
            .with_visible_digits(38)
            .with_max_steps(100);
        let result = find_seed_to_witness(config).expect("witness");
        assert_eq!(result.witness_seed, 3);
        assert_eq!(result.steps_to_witness, 3);
        assert_eq!(
            result.decimal_value,
            "30070000000000000000000000000000307003"
        );
        assert!(result.scanned_seed_count >= 4);
    }

    #[test]
    fn exact_seed_only_succeeds_or_fails_cleanly() {
        let success = find_seed_to_witness(
            SeedToWitnessConfig::default_for_seed(60).with_exact_seed_only(true),
        )
        .expect("exact witness");
        assert_eq!(success.witness_seed, 60);

        let failure = find_seed_to_witness(
            SeedToWitnessConfig::default_for_seed(0)
                .with_visible_digits(38)
                .with_exact_seed_only(true),
        )
        .expect_err("exact seed is not a witness");
        assert!(matches!(
            failure,
            SeedToWitnessError::NoWitnessFound {
                exact_seed_only: true,
                ..
            }
        ));
    }

    #[test]
    fn transcript_contains_local_and_external_verification_hooks() {
        let result =
            find_seed_to_witness(SeedToWitnessConfig::default_for_seed(60)).expect("witness");
        let transcript = render_seed_to_witness_transcript(&result);
        assert!(transcript.contains("N(s) ="));
        assert!(transcript.contains(&result.decimal_value));
        assert!(transcript.contains("probable_prime_fixed_20_bases"));
        assert!(transcript.contains("Mersenne class"));
        assert!(transcript.contains("not_mersenne"));
        assert!(transcript.contains("WolframAlpha"));
        assert!(transcript.contains("PrimeQ["));
        assert!(transcript.contains("PARI/GP"));
        assert!(transcript.contains("Sage"));
    }
}
