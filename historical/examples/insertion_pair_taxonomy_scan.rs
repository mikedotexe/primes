//! Insertion Pair Taxonomy Scan
//!
//! Temporary exploratory driver for the same-scale insertion taxonomy lane.
//!
//! This script:
//! - builds a deterministic pair catalog,
//! - measures midpoint-density and insertion metrics,
//! - exports pair, sweep, position, and family tables,
//! - prints a compact ranked summary,
//! - writes a generated Markdown report alongside the raw artifacts.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example insertion_pair_taxonomy_scan
//! cargo run --example insertion_pair_taxonomy_scan -- --out-dir /tmp/my_scan --small-prime-bound 23 --midpoint-radius 1500
//! ```

use primes::{
    connector::{
        scan_single_digit_hits, small_primes_up_to, ConcatenationSystem, Direction,
        PairResidueProfile,
    },
    is_prime,
    validation::reporting::{
        ensure_dir, export_timestamp_utc, write_csv_rows, write_json_pretty, write_text_file,
    },
    BitSieve,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    path::{Path, PathBuf},
};

const WIDTHS: &[u32] = &[5, 6, 7];
const DIGITS: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
const RESIDUE_MODULI: &[u32] = &[3, 9];
const SWEEP_BOUNDS: &[u32] = &[5, 7, 11, 13, 17, 19, 23, 29, 31];

const DEFAULT_OUT_DIR: &str = "/tmp/primes_insertion_taxonomy";
const DEFAULT_SMALL_PRIME_BOUND: u32 = 19;
const DEFAULT_MIDPOINT_RADIUS: u128 = 1000;
const DEFAULT_SEED: u64 = 20260412;

const GENERATION_DIGITS: std::ops::RangeInclusive<usize> = 2..=6;
const CATEGORY_TARGET: usize = 10;
const PER_DIGIT_TARGET: usize = CATEGORY_TARGET * 4;
const TOTAL_GENERATED_TARGET: usize = 200;

const INTERACTION_MIN_SUPPORT: usize = 4;
const PROMISING_MIN_PAIRS: usize = 8;
const SIGN_STABLE_SHARE_THRESHOLD: f64 = 0.75;
const FAMILY_DIRECTION_PURITY_THRESHOLD: f64 = 0.65;
const MATERIAL_RESIDUAL_THRESHOLD: f64 = 0.10;
const MIDPOINT_FLAT_THRESHOLD_PCT: f64 = 5.0;
const SINGLE_PAIR_DOMINANCE_THRESHOLD: f64 = 0.50;
const SIMPLE_STABLE_MIN_PAIRS: usize = 10;
const SIMPLE_STABLE_SIGN_STABLE_THRESHOLD: f64 = 0.85;
const SIMPLE_STABLE_DIRECTION_PURITY_THRESHOLD: f64 = 0.70;
const SIMPLE_STABLE_MEDIAN_ABS_DELTA_THRESHOLD: f64 = 0.18;
const SIMPLE_STABLE_MAX_PAIR_DOMINANCE_THRESHOLD: f64 = 0.20;
const WINDOW_PREFIX_LIMIT_CAP: usize = 5_000_000;
const TOP_PAIR_SUMMARY_COUNT: usize = 12;
const TOP_FAMILY_SUMMARY_COUNT: usize = 12;

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    small_prime_bound: u32,
    midpoint_radius: u128,
}

#[derive(Debug, Clone, Serialize)]
struct Settings {
    generated_digit_lengths: Vec<usize>,
    per_digit_target: usize,
    total_generated_target: usize,
    category_target: usize,
    widths: Vec<u32>,
    digits: Vec<u8>,
    residue_moduli: Vec<u32>,
    sweep_bounds: Vec<u32>,
    requested_small_prime_bound: u32,
    requested_small_primes: Vec<u32>,
    midpoint_radius: u128,
    seed: u64,
    out_dir: String,
}

#[derive(Debug, Clone)]
struct PairSpec {
    pair_id: String,
    pair_label: String,
    short_label: String,
    source_kind: String,
    construction_category: String,
    pair: ConcatenationSystem,
    is_anchor: bool,
    is_control: bool,
}

#[derive(Debug, Clone, Serialize)]
struct PairSummaryRow {
    pair_id: String,
    pair_label: String,
    short_label: String,
    source_kind: String,
    construction_category: String,
    is_anchor: bool,
    is_control: bool,
    fair_midpoint_candidate: bool,
    fair_midpoint_reason: String,
    same_digit_length: bool,
    left: u128,
    right: u128,
    left_is_prime: bool,
    right_is_prime: bool,
    left_digits: usize,
    right_digits: usize,
    pair_digit_length: usize,
    left_last_digit: u8,
    right_last_digit: u8,
    ending_digit_pair_class: String,
    gap: u128,
    normalized_gap: f64,
    gap_tertile: String,
    left_zero_density: f64,
    right_zero_density: f64,
    avg_zero_density: f64,
    zero_density_bucket: String,
    left_digit_sum: u32,
    right_digit_sum: u32,
    digit_sum_product: u64,
    left_is_palindrome: bool,
    right_is_palindrome: bool,
    left_contains_zero: bool,
    right_contains_zero: bool,
    pair_residue_mod3: u32,
    blocked_connector_mod3: u32,
    pair_residue_mod9: u32,
    blocked_connector_mod9: u32,
    pair_residue_mod11: u32,
    blocked_connector_mod11: u32,
    midpoint: u128,
    midpoint_radius: u128,
    left_window_primes: usize,
    midpoint_window_primes: usize,
    right_window_primes: usize,
    endpoint_avg_window_primes: f64,
    left_density: f64,
    midpoint_density: f64,
    right_density: f64,
    endpoint_avg_density: f64,
    midpoint_lift_pct: f64,
    requested_small_prime_bound: u32,
    requested_small_primes: String,
    raw_candidates_per_direction: usize,
    residue_admissible_candidates_per_direction: usize,
    forward_prime_hits: usize,
    reverse_prime_hits: usize,
    forward_post_filter_rate: f64,
    reverse_post_filter_rate: f64,
    forward_corrected_expected_hits: f64,
    reverse_corrected_expected_hits: f64,
    forward_corrected_ratio: f64,
    reverse_corrected_ratio: f64,
    corrected_expected_hit_delta: f64,
    corrected_residual_ratio_delta: f64,
    bound19_corrected_expected_hit_delta: f64,
    bound19_corrected_residual_ratio_delta: f64,
    sign_stable: bool,
    material_residual: bool,
    midpoint_flat: bool,
    resonance_position_count: usize,
    resonance_total_multiplicity: usize,
    resonance_forward_count: usize,
    resonance_reverse_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct BoundSweepRow {
    pair_id: String,
    pair_label: String,
    source_kind: String,
    construction_category: String,
    fair_midpoint_candidate: bool,
    pair_digit_length: usize,
    gap_tertile: String,
    zero_density_bucket: String,
    pair_residue_mod9: u32,
    ending_digit_pair_class: String,
    bound: u32,
    small_primes: String,
    forward_corrected_expected_hits: f64,
    reverse_corrected_expected_hits: f64,
    corrected_expected_hit_delta: f64,
    forward_corrected_ratio: f64,
    reverse_corrected_ratio: f64,
    corrected_residual_ratio_delta: f64,
    forward_corrected_poisson_z: f64,
    reverse_corrected_poisson_z: f64,
}

#[derive(Debug, Clone, Serialize)]
struct PositionRow {
    pair_id: String,
    pair_label: String,
    source_kind: String,
    construction_category: String,
    fair_midpoint_candidate: bool,
    pair_digit_length: usize,
    gap_tertile: String,
    zero_density_bucket: String,
    pair_residue_mod9: u32,
    ending_digit_pair_class: String,
    direction: Direction,
    width: u32,
    position: u32,
    residue_admissible_candidates: usize,
    prime_hits: usize,
    working_digits: String,
    resonance_position: bool,
    multiplicity: usize,
    naive_expected_hits: f64,
    small_prime_corrected_expected_hits: f64,
    observed_to_corrected_ratio: f64,
    sign_stable: bool,
    material_residual: bool,
    midpoint_flat: bool,
    bound19_corrected_residual_ratio_delta: f64,
}

#[derive(Debug, Clone, Serialize)]
struct FamilySummaryRow {
    family_kind: String,
    family_key: String,
    pair_count: usize,
    fair_pair_count: usize,
    anchor_pair_count: usize,
    simple_family: bool,
    sign_stable_share: f64,
    direction_purity: f64,
    dominant_direction: String,
    material_residual_share: f64,
    midpoint_flat_share: f64,
    positive_delta_share: f64,
    negative_delta_share: f64,
    median_abs_corrected_delta_bound19: f64,
    median_corrected_delta_bound19: f64,
    median_midpoint_lift_pct: f64,
    max_pair_abs_residual_share_bound19: f64,
    promising_family: bool,
    simple_stable_family: bool,
    proof_next_family_candidate: bool,
}

#[derive(Debug, Clone)]
struct PairBucketContext {
    fair_midpoint_candidate: bool,
    pair_digit_length: usize,
    zero_density_bucket: String,
    pair_residue_mod9: u32,
    ending_digit_pair_class: String,
}

#[derive(Debug, Clone)]
struct PairSignalFlags {
    sign_stable: bool,
    material_residual: bool,
    midpoint_flat: bool,
    bound19_corrected_residual_ratio_delta: f64,
}

#[derive(Debug, Clone, Serialize)]
struct SummaryBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: Settings,
    pair_summary: Vec<PairSummaryRow>,
    bound_sweep: Vec<BoundSweepRow>,
    position_rows: Vec<PositionRow>,
    family_summary: Vec<FamilySummaryRow>,
}

#[derive(Debug, Clone)]
struct MidpointStats {
    left_is_prime: bool,
    right_is_prime: bool,
    midpoint: u128,
    left_window_primes: usize,
    midpoint_window_primes: usize,
    right_window_primes: usize,
    endpoint_avg_window_primes: f64,
    left_density: f64,
    midpoint_density: f64,
    right_density: f64,
    endpoint_avg_density: f64,
    midpoint_lift_pct: f64,
}

#[derive(Debug)]
struct PrimeWindowCounter {
    limit: usize,
    prefix: Vec<u32>,
}

fn main() {
    let options = parse_args();
    let requested_small_primes = small_primes_up_to(options.small_prime_bound);
    let settings = Settings {
        generated_digit_lengths: GENERATION_DIGITS.collect(),
        per_digit_target: PER_DIGIT_TARGET,
        total_generated_target: TOTAL_GENERATED_TARGET,
        category_target: CATEGORY_TARGET,
        widths: WIDTHS.to_vec(),
        digits: DIGITS.to_vec(),
        residue_moduli: RESIDUE_MODULI.to_vec(),
        sweep_bounds: SWEEP_BOUNDS.to_vec(),
        requested_small_prime_bound: options.small_prime_bound,
        requested_small_primes: requested_small_primes.clone(),
        midpoint_radius: options.midpoint_radius,
        seed: DEFAULT_SEED,
        out_dir: options.out_dir.display().to_string(),
    };

    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let window_counter = build_prime_window_counter(options.midpoint_radius);
    let generated_pairs = generate_same_scale_pairs();
    let mut pair_specs = generated_pairs;
    pair_specs.extend(anchor_pair_specs());

    let mut pair_summary = Vec::new();
    let mut bound_sweep = Vec::new();
    let mut position_rows = Vec::new();

    for spec in &pair_specs {
        let midpoint_stats =
            midpoint_density_stats(spec.pair, options.midpoint_radius, window_counter.as_ref());
        let summary = scan_single_digit_hits(spec.pair, WIDTHS, DIGITS, RESIDUE_MODULI);
        let requested_audit = summary.signal_audit(&requested_small_primes);
        let resonance_positions = summary.resonance_positions();
        let resonance_position_count = resonance_positions.len();
        let resonance_total_multiplicity = resonance_positions
            .iter()
            .map(|position| position.multiplicity())
            .sum();
        let resonance_forward_count = resonance_positions
            .iter()
            .filter(|position| position.direction == Direction::Forward)
            .count();
        let resonance_reverse_count = resonance_positions
            .iter()
            .filter(|position| position.direction == Direction::Reverse)
            .count();
        let left_digits = decimal_digits(spec.pair.left);
        let right_digits = decimal_digits(spec.pair.right);
        let same_digit_length = left_digits == right_digits;
        let fair_midpoint_candidate =
            midpoint_stats.left_is_prime && midpoint_stats.right_is_prime && same_digit_length;
        let fair_midpoint_reason =
            if !midpoint_stats.left_is_prime || !midpoint_stats.right_is_prime {
                "one endpoint is not prime".to_string()
            } else if !same_digit_length {
                format!("digit-length mismatch: {} vs {}", left_digits, right_digits)
            } else {
                "same-scale prime pair".to_string()
            };
        let pair_digit_length = left_digits.max(right_digits);
        let gap = spec.pair.right.abs_diff(spec.pair.left);
        let normalized_gap = gap as f64 / 10f64.powi((pair_digit_length.saturating_sub(1)) as i32);
        let left_zero_density = zero_density_u128(spec.pair.left);
        let right_zero_density = zero_density_u128(spec.pair.right);
        let avg_zero_density = (left_zero_density + right_zero_density) / 2.0;
        let zero_density_bucket = zero_density_bucket(avg_zero_density).to_string();
        let left_digit_sum = digit_sum_u128(spec.pair.left);
        let right_digit_sum = digit_sum_u128(spec.pair.right);
        let pair_residue_mod3 = PairResidueProfile::new(spec.pair, 3);
        let pair_residue_mod9 = PairResidueProfile::new(spec.pair, 9);
        let pair_residue_mod11 = PairResidueProfile::new(spec.pair, 11);
        let ending_digit_pair_class = format!("{}-{}", spec.pair.left % 10, spec.pair.right % 10);
        let bucket_context = PairBucketContext {
            fair_midpoint_candidate,
            pair_digit_length,
            zero_density_bucket: zero_density_bucket.clone(),
            pair_residue_mod9: pair_residue_mod9.pair_residue,
            ending_digit_pair_class: ending_digit_pair_class.clone(),
        };
        let sweep_rows = build_sweep_rows(spec, &summary, &bucket_context);
        let sign_stable = is_sign_stable(&sweep_rows);
        let bound19_row = sweep_rows
            .iter()
            .find(|row| row.bound == 19)
            .expect("missing bound-19 sweep row");
        let material_residual =
            bound19_row.corrected_residual_ratio_delta.abs() >= MATERIAL_RESIDUAL_THRESHOLD;
        let midpoint_flat = midpoint_stats.midpoint_lift_pct.abs() <= MIDPOINT_FLAT_THRESHOLD_PCT;

        let pair_row = PairSummaryRow {
            pair_id: spec.pair_id.clone(),
            pair_label: spec.pair_label.clone(),
            short_label: spec.short_label.clone(),
            source_kind: spec.source_kind.clone(),
            construction_category: spec.construction_category.clone(),
            is_anchor: spec.is_anchor,
            is_control: spec.is_control,
            fair_midpoint_candidate,
            fair_midpoint_reason,
            same_digit_length,
            left: spec.pair.left,
            right: spec.pair.right,
            left_is_prime: midpoint_stats.left_is_prime,
            right_is_prime: midpoint_stats.right_is_prime,
            left_digits,
            right_digits,
            pair_digit_length,
            left_last_digit: (spec.pair.left % 10) as u8,
            right_last_digit: (spec.pair.right % 10) as u8,
            ending_digit_pair_class,
            gap,
            normalized_gap,
            gap_tertile: String::new(),
            left_zero_density,
            right_zero_density,
            avg_zero_density,
            zero_density_bucket: zero_density_bucket.clone(),
            left_digit_sum,
            right_digit_sum,
            digit_sum_product: left_digit_sum as u64 * right_digit_sum as u64,
            left_is_palindrome: is_palindrome_u128(spec.pair.left),
            right_is_palindrome: is_palindrome_u128(spec.pair.right),
            left_contains_zero: contains_zero_u128(spec.pair.left),
            right_contains_zero: contains_zero_u128(spec.pair.right),
            pair_residue_mod3: pair_residue_mod3.pair_residue,
            blocked_connector_mod3: pair_residue_mod3.blocked_connector_residue,
            pair_residue_mod9: pair_residue_mod9.pair_residue,
            blocked_connector_mod9: pair_residue_mod9.blocked_connector_residue,
            pair_residue_mod11: pair_residue_mod11.pair_residue,
            blocked_connector_mod11: pair_residue_mod11.blocked_connector_residue,
            midpoint: midpoint_stats.midpoint,
            midpoint_radius: options.midpoint_radius,
            left_window_primes: midpoint_stats.left_window_primes,
            midpoint_window_primes: midpoint_stats.midpoint_window_primes,
            right_window_primes: midpoint_stats.right_window_primes,
            endpoint_avg_window_primes: midpoint_stats.endpoint_avg_window_primes,
            left_density: midpoint_stats.left_density,
            midpoint_density: midpoint_stats.midpoint_density,
            right_density: midpoint_stats.right_density,
            endpoint_avg_density: midpoint_stats.endpoint_avg_density,
            midpoint_lift_pct: midpoint_stats.midpoint_lift_pct,
            requested_small_prime_bound: options.small_prime_bound,
            requested_small_primes: format!("{:?}", requested_small_primes),
            raw_candidates_per_direction: summary.forward.raw_candidates(),
            residue_admissible_candidates_per_direction: summary
                .forward
                .residue_admissible_candidates(),
            forward_prime_hits: summary.forward.prime_hits(),
            reverse_prime_hits: summary.reverse.prime_hits(),
            forward_post_filter_rate: summary.forward.post_filter_prime_rate(),
            reverse_post_filter_rate: summary.reverse.post_filter_prime_rate(),
            forward_corrected_expected_hits: requested_audit
                .forward
                .small_prime_corrected_expected_hits,
            reverse_corrected_expected_hits: requested_audit
                .reverse
                .small_prime_corrected_expected_hits,
            forward_corrected_ratio: requested_audit.forward.observed_to_corrected_ratio,
            reverse_corrected_ratio: requested_audit.reverse.observed_to_corrected_ratio,
            corrected_expected_hit_delta: requested_audit.corrected_expected_hit_delta(),
            corrected_residual_ratio_delta: requested_audit.corrected_residual_ratio_delta(),
            bound19_corrected_expected_hit_delta: bound19_row.corrected_expected_hit_delta,
            bound19_corrected_residual_ratio_delta: bound19_row.corrected_residual_ratio_delta,
            sign_stable,
            material_residual,
            midpoint_flat,
            resonance_position_count,
            resonance_total_multiplicity,
            resonance_forward_count,
            resonance_reverse_count,
        };
        let signal_flags = PairSignalFlags {
            sign_stable: pair_row.sign_stable,
            material_residual: pair_row.material_residual,
            midpoint_flat: pair_row.midpoint_flat,
            bound19_corrected_residual_ratio_delta: pair_row.bound19_corrected_residual_ratio_delta,
        };

        let mut pair_position_rows =
            build_position_rows(spec, &summary, &bucket_context, &signal_flags);

        pair_summary.push(pair_row);
        bound_sweep.extend(sweep_rows);
        position_rows.append(&mut pair_position_rows);
    }

    assign_gap_tertiles(&mut pair_summary);
    propagate_gap_tertiles(&pair_summary, &mut bound_sweep, &mut position_rows);

    let family_summary = build_family_summary(&pair_summary);

    write_csv_rows(options.out_dir.join("pair_summary.csv"), &pair_summary)
        .expect("failed to write pair summary CSV");
    write_csv_rows(options.out_dir.join("bound_sweep.csv"), &bound_sweep)
        .expect("failed to write bound sweep CSV");
    write_csv_rows(options.out_dir.join("position_rows.csv"), &position_rows)
        .expect("failed to write position rows CSV");
    write_csv_rows(options.out_dir.join("family_summary.csv"), &family_summary)
        .expect("failed to write family summary CSV");

    let bundle = SummaryBundle {
        export_version: 1,
        generated_at_utc: export_timestamp_utc(),
        settings,
        pair_summary,
        bound_sweep,
        position_rows,
        family_summary,
    };
    write_json_pretty(options.out_dir.join("summary.json"), &bundle)
        .expect("failed to write summary JSON");
    write_text_file(
        options.out_dir.join("report.md"),
        &render_markdown_report(&bundle),
    )
    .expect("failed to write markdown report");

    print_ranked_summary(&bundle);
}

fn parse_args() -> Options {
    let mut args = env::args().skip(1);
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut small_prime_bound = DEFAULT_SMALL_PRIME_BOUND;
    let mut midpoint_radius = DEFAULT_MIDPOINT_RADIUS;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => {
                out_dir = PathBuf::from(parse_next::<String>(&mut args, "--out-dir"));
            }
            "--small-prime-bound" => {
                small_prime_bound = parse_next::<u32>(&mut args, "--small-prime-bound");
            }
            "--midpoint-radius" => {
                midpoint_radius = parse_next::<u128>(&mut args, "--midpoint-radius");
            }
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            other => {
                eprintln!("Unrecognized argument: {other}");
                print_usage();
                std::process::exit(1);
            }
        }
    }

    Options {
        out_dir,
        small_prime_bound,
        midpoint_radius,
    }
}

fn parse_next<T>(args: &mut impl Iterator<Item = String>, flag: &str) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let raw = args.next().unwrap_or_else(|| {
        eprintln!("Missing value for {flag}");
        print_usage();
        std::process::exit(1);
    });
    raw.parse::<T>().unwrap_or_else(|err| {
        eprintln!("Invalid value for {flag}: {err}");
        print_usage();
        std::process::exit(1);
    })
}

fn print_usage() {
    println!("Insertion pair taxonomy scan");
    println!();
    println!("Usage:");
    println!("  cargo run --example insertion_pair_taxonomy_scan -- [options]");
    println!();
    println!("Options:");
    println!(
        "  --out-dir <path>            Output directory for CSV/JSON artifacts (default: {DEFAULT_OUT_DIR})"
    );
    println!(
        "  --small-prime-bound <n>     Use all primes <= n for the requested correction layer (default: {DEFAULT_SMALL_PRIME_BOUND})"
    );
    println!(
        "  --midpoint-radius <n>       Radius for midpoint/end-window density counts (default: {DEFAULT_MIDPOINT_RADIUS})"
    );
}

fn generate_same_scale_pairs() -> Vec<PairSpec> {
    let sieve = BitSieve::new(999_999);
    let primes = sieve.primes();
    let reserved = reserved_pair_keys();
    let mut used = reserved.clone();
    let mut rng = StdRng::seed_from_u64(DEFAULT_SEED);
    let mut specs = Vec::new();

    for digits in GENERATION_DIGITS {
        let bucket: Vec<u128> = primes
            .iter()
            .copied()
            .filter(|prime| decimal_digits(*prime as u128) == digits)
            .map(|prime| prime as u128)
            .collect();

        let near_pool = build_offset_pool(&bucket, 1);
        let medium_pool = build_offset_pool(&bucket, medium_offset(bucket.len()));
        let far_pool = build_offset_pool(&bucket, far_offset(bucket.len()));

        let near_pairs = select_pairs_from_pool(&near_pool, CATEGORY_TARGET, &mut used, &reserved);
        let medium_pairs =
            select_pairs_from_pool(&medium_pool, CATEGORY_TARGET, &mut used, &reserved);
        let far_pairs = select_pairs_from_pool(&far_pool, CATEGORY_TARGET, &mut used, &reserved);
        let random_pairs =
            select_random_pairs(&bucket, CATEGORY_TARGET, &mut rng, &mut used, &reserved);

        specs.extend(pairs_to_specs(digits, "near", near_pairs));
        specs.extend(pairs_to_specs(digits, "medium", medium_pairs));
        specs.extend(pairs_to_specs(digits, "far", far_pairs));
        specs.extend(pairs_to_specs(digits, "random", random_pairs));
    }

    assert_eq!(
        specs.len(),
        TOTAL_GENERATED_TARGET,
        "generated catalog should contain exactly {TOTAL_GENERATED_TARGET} pairs"
    );

    specs
}

fn reserved_pair_keys() -> BTreeSet<(u128, u128)> {
    anchor_pair_specs()
        .into_iter()
        .map(|spec| pair_key(spec.pair.left, spec.pair.right))
        .collect()
}

fn anchor_pair_specs() -> Vec<PairSpec> {
    vec![
        PairSpec {
            pair_id: "anchor_midpoint_membrane".to_string(),
            pair_label: "303050303 ∘ 307050703".to_string(),
            short_label: "Midpoint membrane".to_string(),
            source_kind: "anchor".to_string(),
            construction_category: "anchor".to_string(),
            pair: ConcatenationSystem::new(303050303, 307050703),
            is_anchor: true,
            is_control: false,
        },
        PairSpec {
            pair_id: "anchor_twin_profile".to_string(),
            pair_label: "11 ∘ 13".to_string(),
            short_label: "Twin profile".to_string(),
            source_kind: "anchor".to_string(),
            construction_category: "anchor".to_string(),
            pair: ConcatenationSystem::new(11, 13),
            is_anchor: true,
            is_control: false,
        },
        PairSpec {
            pair_id: "anchor_sophie_profile".to_string(),
            pair_label: "23 ∘ 47".to_string(),
            short_label: "Sophie profile".to_string(),
            source_kind: "anchor".to_string(),
            construction_category: "anchor".to_string(),
            pair: ConcatenationSystem::new(23, 47),
            is_anchor: true,
            is_control: false,
        },
        PairSpec {
            pair_id: "control_canonical_cross_scale".to_string(),
            pair_label: "10301 ∘ 3007003007003".to_string(),
            short_label: "Canonical control".to_string(),
            source_kind: "control".to_string(),
            construction_category: "control".to_string(),
            pair: ConcatenationSystem::new(10301, 3007003007003),
            is_anchor: false,
            is_control: true,
        },
    ]
}

fn pairs_to_specs(digits: usize, category: &str, pairs: Vec<(u128, u128)>) -> Vec<PairSpec> {
    pairs
        .into_iter()
        .enumerate()
        .map(|(index, (left, right))| PairSpec {
            pair_id: format!("d{digits}_{category}_{:02}", index + 1),
            pair_label: format!("{left} ∘ {right}"),
            short_label: format!("d{digits} {category} {:02}", index + 1),
            source_kind: "generated".to_string(),
            construction_category: category.to_string(),
            pair: ConcatenationSystem::new(left, right),
            is_anchor: false,
            is_control: false,
        })
        .collect()
}

fn build_offset_pool(primes: &[u128], offset: usize) -> Vec<(u128, u128)> {
    if offset == 0 || offset >= primes.len() {
        return Vec::new();
    }

    (0..(primes.len() - offset))
        .map(|index| (primes[index], primes[index + offset]))
        .collect()
}

fn medium_offset(len: usize) -> usize {
    (len / 10).max(3)
}

fn far_offset(len: usize) -> usize {
    (len / 2).max(medium_offset(len) + 1).max(5)
}

fn select_pairs_from_pool(
    pool: &[(u128, u128)],
    target: usize,
    used: &mut BTreeSet<(u128, u128)>,
    reserved: &BTreeSet<(u128, u128)>,
) -> Vec<(u128, u128)> {
    let mut selected = Vec::new();
    if pool.is_empty() {
        return selected;
    }

    for index in evenly_spaced_indices(pool.len(), target) {
        let pair = pool[index];
        if try_take_pair(pair, used, reserved, &mut selected) && selected.len() == target {
            return selected;
        }
    }

    for &pair in pool {
        if try_take_pair(pair, used, reserved, &mut selected) && selected.len() == target {
            break;
        }
    }

    selected
}

fn select_random_pairs(
    primes: &[u128],
    target: usize,
    rng: &mut StdRng,
    used: &mut BTreeSet<(u128, u128)>,
    reserved: &BTreeSet<(u128, u128)>,
) -> Vec<(u128, u128)> {
    let mut selected = Vec::new();
    let mut attempts = 0usize;

    while selected.len() < target && attempts < 50_000 {
        let left_index = rng.gen_range(0..primes.len());
        let mut right_index = rng.gen_range(0..primes.len());
        if right_index == left_index {
            right_index = (right_index + 1) % primes.len();
        }
        let (i, j) = if left_index < right_index {
            (left_index, right_index)
        } else {
            (right_index, left_index)
        };
        let pair = (primes[i], primes[j]);
        let _ = try_take_pair(pair, used, reserved, &mut selected);
        attempts += 1;
    }

    if selected.len() < target {
        for i in 0..primes.len() {
            for j in (i + 1)..primes.len() {
                let pair = (primes[i], primes[j]);
                if try_take_pair(pair, used, reserved, &mut selected) && selected.len() == target {
                    return selected;
                }
            }
        }
    }

    assert_eq!(
        selected.len(),
        target,
        "failed to select {target} random pairs"
    );
    selected
}

fn try_take_pair(
    pair: (u128, u128),
    used: &mut BTreeSet<(u128, u128)>,
    reserved: &BTreeSet<(u128, u128)>,
    selected: &mut Vec<(u128, u128)>,
) -> bool {
    let key = pair_key(pair.0, pair.1);
    if reserved.contains(&key) || used.contains(&key) {
        return false;
    }
    used.insert(key);
    selected.push(pair);
    true
}

fn evenly_spaced_indices(len: usize, target: usize) -> Vec<usize> {
    if len == 0 || target == 0 {
        return Vec::new();
    }
    if target == 1 {
        return vec![0];
    }
    let mut indices = BTreeSet::new();
    for k in 0..target {
        indices.insert(k * (len - 1) / (target - 1));
    }
    indices.into_iter().collect()
}

fn pair_key(left: u128, right: u128) -> (u128, u128) {
    if left <= right {
        (left, right)
    } else {
        (right, left)
    }
}

fn build_prime_window_counter(midpoint_radius: u128) -> Option<PrimeWindowCounter> {
    let limit = usize::try_from(999_999u128.saturating_add(midpoint_radius)).ok()?;
    if limit > WINDOW_PREFIX_LIMIT_CAP {
        return None;
    }

    let sieve = BitSieve::new(limit);
    let mut is_prime_small = vec![false; limit + 1];
    for prime in sieve.primes() {
        is_prime_small[prime] = true;
    }

    let mut prefix = vec![0u32; limit + 1];
    for index in 1..=limit {
        prefix[index] = prefix[index - 1] + u32::from(is_prime_small[index]);
    }

    Some(PrimeWindowCounter { limit, prefix })
}

fn midpoint_density_stats(
    pair: ConcatenationSystem,
    radius: u128,
    window_counter: Option<&PrimeWindowCounter>,
) -> MidpointStats {
    let left_is_prime = is_prime_u128(pair.left);
    let right_is_prime = is_prime_u128(pair.right);
    let midpoint = midpoint_u128(pair.left, pair.right);
    let left_window_primes = count_primes_in_window(pair.left, radius, window_counter);
    let midpoint_window_primes = count_primes_in_window(midpoint, radius, window_counter);
    let right_window_primes = count_primes_in_window(pair.right, radius, window_counter);
    let window_size = (radius * 2 + 1) as f64;
    let left_density = left_window_primes as f64 / window_size;
    let midpoint_density = midpoint_window_primes as f64 / window_size;
    let right_density = right_window_primes as f64 / window_size;
    let endpoint_avg_density = (left_density + right_density) / 2.0;
    let endpoint_avg_window_primes = (left_window_primes as f64 + right_window_primes as f64) / 2.0;
    let midpoint_lift_pct = if endpoint_avg_density > 0.0 {
        (midpoint_density / endpoint_avg_density - 1.0) * 100.0
    } else {
        0.0
    };

    MidpointStats {
        left_is_prime,
        right_is_prime,
        midpoint,
        left_window_primes,
        midpoint_window_primes,
        right_window_primes,
        endpoint_avg_window_primes,
        left_density,
        midpoint_density,
        right_density,
        endpoint_avg_density,
        midpoint_lift_pct,
    }
}

fn count_primes_in_window(
    center: u128,
    radius: u128,
    window_counter: Option<&PrimeWindowCounter>,
) -> usize {
    if let Some(counter) = window_counter {
        let start = center.saturating_sub(radius);
        let end = center.saturating_add(radius);
        if let (Ok(start), Ok(end)) = (usize::try_from(start), usize::try_from(end)) {
            if end <= counter.limit {
                let before = start
                    .checked_sub(1)
                    .map(|index| counter.prefix[index])
                    .unwrap_or(0);
                return (counter.prefix[end] - before) as usize;
            }
        }
    }

    let start = center.saturating_sub(radius);
    let end = center.saturating_add(radius);
    let mut count = 0usize;
    for value in start..=end {
        if is_prime_u128(value) {
            count += 1;
        }
    }
    count
}

fn build_sweep_rows(
    spec: &PairSpec,
    summary: &primes::connector::PairScanSummary,
    context: &PairBucketContext,
) -> Vec<BoundSweepRow> {
    SWEEP_BOUNDS
        .iter()
        .copied()
        .map(|bound| {
            let small_primes = small_primes_up_to(bound);
            let audit = summary.signal_audit(&small_primes);
            BoundSweepRow {
                pair_id: spec.pair_id.clone(),
                pair_label: spec.pair_label.clone(),
                source_kind: spec.source_kind.clone(),
                construction_category: spec.construction_category.clone(),
                fair_midpoint_candidate: context.fair_midpoint_candidate,
                pair_digit_length: context.pair_digit_length,
                gap_tertile: String::new(),
                zero_density_bucket: context.zero_density_bucket.clone(),
                pair_residue_mod9: context.pair_residue_mod9,
                ending_digit_pair_class: context.ending_digit_pair_class.clone(),
                bound,
                small_primes: format!("{:?}", small_primes),
                forward_corrected_expected_hits: audit.forward.small_prime_corrected_expected_hits,
                reverse_corrected_expected_hits: audit.reverse.small_prime_corrected_expected_hits,
                corrected_expected_hit_delta: audit.corrected_expected_hit_delta(),
                forward_corrected_ratio: audit.forward.observed_to_corrected_ratio,
                reverse_corrected_ratio: audit.reverse.observed_to_corrected_ratio,
                corrected_residual_ratio_delta: audit.corrected_residual_ratio_delta(),
                forward_corrected_poisson_z: audit.forward.corrected_poisson_residual_z,
                reverse_corrected_poisson_z: audit.reverse.corrected_poisson_residual_z,
            }
        })
        .collect()
}

fn is_sign_stable(rows: &[BoundSweepRow]) -> bool {
    if rows.is_empty() {
        return false;
    }
    let all_positive = rows
        .iter()
        .all(|row| row.corrected_residual_ratio_delta > 0.0);
    let all_negative = rows
        .iter()
        .all(|row| row.corrected_residual_ratio_delta < 0.0);
    all_positive || all_negative
}

fn build_position_rows(
    spec: &PairSpec,
    summary: &primes::connector::PairScanSummary,
    context: &PairBucketContext,
    flags: &PairSignalFlags,
) -> Vec<PositionRow> {
    let audit = summary.signal_audit(&small_primes_up_to(19));
    let mut rows = Vec::new();

    for row in audit
        .forward_positions
        .iter()
        .chain(audit.reverse_positions.iter())
    {
        rows.push(PositionRow {
            pair_id: spec.pair_id.clone(),
            pair_label: spec.pair_label.clone(),
            source_kind: spec.source_kind.clone(),
            construction_category: spec.construction_category.clone(),
            fair_midpoint_candidate: context.fair_midpoint_candidate,
            pair_digit_length: context.pair_digit_length,
            gap_tertile: String::new(),
            zero_density_bucket: context.zero_density_bucket.clone(),
            pair_residue_mod9: context.pair_residue_mod9,
            ending_digit_pair_class: context.ending_digit_pair_class.clone(),
            direction: row.direction,
            width: row.width,
            position: row.position,
            residue_admissible_candidates: row.residue_admissible_candidates,
            prime_hits: row.prime_hits,
            working_digits: format!("{:?}", row.working_digits),
            resonance_position: row.working_digits.len() > 1,
            multiplicity: row.working_digits.len(),
            naive_expected_hits: row.naive_expected_hits,
            small_prime_corrected_expected_hits: row.small_prime_corrected_expected_hits,
            observed_to_corrected_ratio: row.observed_to_corrected_ratio,
            sign_stable: flags.sign_stable,
            material_residual: flags.material_residual,
            midpoint_flat: flags.midpoint_flat,
            bound19_corrected_residual_ratio_delta: flags.bound19_corrected_residual_ratio_delta,
        });
    }

    rows
}

fn assign_gap_tertiles(rows: &mut [PairSummaryRow]) {
    let mut by_length: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (index, row) in rows.iter().enumerate() {
        if row.fair_midpoint_candidate {
            by_length
                .entry(row.pair_digit_length)
                .or_default()
                .push(index);
        }
    }

    for (_digits, indices) in by_length {
        let mut sorted = indices;
        sorted.sort_by(|left, right| {
            rows[*left]
                .normalized_gap
                .total_cmp(&rows[*right].normalized_gap)
                .then_with(|| rows[*left].pair_id.cmp(&rows[*right].pair_id))
        });
        let len = sorted.len();
        for (rank, index) in sorted.into_iter().enumerate() {
            let bucket = match (rank * 3) / len.max(1) {
                0 => "low",
                1 => "medium",
                _ => "high",
            };
            rows[index].gap_tertile = bucket.to_string();
        }
    }

    for row in rows.iter_mut().filter(|row| !row.fair_midpoint_candidate) {
        row.gap_tertile = "control".to_string();
    }
}

fn propagate_gap_tertiles(
    pair_rows: &[PairSummaryRow],
    sweep_rows: &mut [BoundSweepRow],
    position_rows: &mut [PositionRow],
) {
    let gap_map: BTreeMap<&str, &str> = pair_rows
        .iter()
        .map(|row| (row.pair_id.as_str(), row.gap_tertile.as_str()))
        .collect();

    for row in sweep_rows {
        row.gap_tertile = gap_map
            .get(row.pair_id.as_str())
            .copied()
            .unwrap_or("unknown")
            .to_string();
    }

    for row in position_rows {
        row.gap_tertile = gap_map
            .get(row.pair_id.as_str())
            .copied()
            .unwrap_or("unknown")
            .to_string();
    }
}

fn build_family_summary(rows: &[PairSummaryRow]) -> Vec<FamilySummaryRow> {
    let fair_rows: Vec<_> = rows
        .iter()
        .filter(|row| row.fair_midpoint_candidate)
        .cloned()
        .collect();
    let mut groups: BTreeMap<(String, String), Vec<PairSummaryRow>> = BTreeMap::new();

    for row in &fair_rows {
        groups
            .entry((
                "digit_length".to_string(),
                row.pair_digit_length.to_string(),
            ))
            .or_default()
            .push(row.clone());
        groups
            .entry(("gap_tertile".to_string(), row.gap_tertile.clone()))
            .or_default()
            .push(row.clone());
        groups
            .entry((
                "zero_density_bucket".to_string(),
                row.zero_density_bucket.clone(),
            ))
            .or_default()
            .push(row.clone());
        groups
            .entry((
                "mod9_residue_bucket".to_string(),
                format!("r{}", row.pair_residue_mod9),
            ))
            .or_default()
            .push(row.clone());
        groups
            .entry((
                "ending_digit_pair_class".to_string(),
                row.ending_digit_pair_class.clone(),
            ))
            .or_default()
            .push(row.clone());
    }

    for row in &fair_rows {
        groups
            .entry((
                "digit_length_x_gap_tertile".to_string(),
                format!("{}x{}", row.pair_digit_length, row.gap_tertile),
            ))
            .or_default()
            .push(row.clone());
    }

    let mut family_rows = Vec::new();
    for ((kind, key), members) in groups {
        if kind == "digit_length_x_gap_tertile" && members.len() < INTERACTION_MIN_SUPPORT {
            continue;
        }
        family_rows.push(summarize_family(kind, key, members));
    }
    family_rows
}

fn is_simple_family_kind(kind: &str) -> bool {
    matches!(
        kind,
        "digit_length"
            | "gap_tertile"
            | "zero_density_bucket"
            | "mod9_residue_bucket"
            | "ending_digit_pair_class"
    )
}

fn dominant_direction_label(positive_delta_share: f64, negative_delta_share: f64) -> String {
    if positive_delta_share > negative_delta_share {
        "forward".to_string()
    } else if negative_delta_share > positive_delta_share {
        "reverse".to_string()
    } else {
        "mixed".to_string()
    }
}

fn summarize_family(
    family_kind: String,
    family_key: String,
    members: Vec<PairSummaryRow>,
) -> FamilySummaryRow {
    let pair_count = members.len();
    let fair_pair_count = members.len();
    let anchor_pair_count = members.iter().filter(|row| row.is_anchor).count();
    let simple_family = is_simple_family_kind(&family_kind);
    let sign_stable_share =
        members.iter().filter(|row| row.sign_stable).count() as f64 / pair_count as f64;
    let material_residual_share =
        members.iter().filter(|row| row.material_residual).count() as f64 / pair_count as f64;
    let midpoint_flat_share =
        members.iter().filter(|row| row.midpoint_flat).count() as f64 / pair_count as f64;
    let positive_delta_share = members
        .iter()
        .filter(|row| row.bound19_corrected_residual_ratio_delta > 0.0)
        .count() as f64
        / pair_count as f64;
    let negative_delta_share = members
        .iter()
        .filter(|row| row.bound19_corrected_residual_ratio_delta < 0.0)
        .count() as f64
        / pair_count as f64;
    let direction_purity = positive_delta_share.max(negative_delta_share);
    let dominant_direction = dominant_direction_label(positive_delta_share, negative_delta_share);

    let abs_deltas: Vec<f64> = members
        .iter()
        .map(|row| row.bound19_corrected_residual_ratio_delta.abs())
        .collect();
    let signed_deltas: Vec<f64> = members
        .iter()
        .map(|row| row.bound19_corrected_residual_ratio_delta)
        .collect();
    let midpoint_lifts: Vec<f64> = members.iter().map(|row| row.midpoint_lift_pct).collect();
    let total_abs_residual: f64 = abs_deltas.iter().sum();
    let max_pair_abs_residual_share_bound19 = if total_abs_residual > 0.0 {
        abs_deltas.iter().copied().fold(0.0_f64, f64::max) / total_abs_residual
    } else {
        0.0
    };
    let median_abs_corrected_delta_bound19 = median(abs_deltas);
    let median_corrected_delta_bound19 = median(signed_deltas);
    let median_midpoint_lift_pct = median(midpoint_lifts);

    let promising_family = pair_count >= PROMISING_MIN_PAIRS
        && sign_stable_share >= SIGN_STABLE_SHARE_THRESHOLD
        && direction_purity >= FAMILY_DIRECTION_PURITY_THRESHOLD
        && median_abs_corrected_delta_bound19 >= MATERIAL_RESIDUAL_THRESHOLD;
    let simple_stable_family = simple_family
        && pair_count >= SIMPLE_STABLE_MIN_PAIRS
        && sign_stable_share >= SIMPLE_STABLE_SIGN_STABLE_THRESHOLD
        && direction_purity >= SIMPLE_STABLE_DIRECTION_PURITY_THRESHOLD
        && median_abs_corrected_delta_bound19 >= SIMPLE_STABLE_MEDIAN_ABS_DELTA_THRESHOLD
        && median_midpoint_lift_pct.abs() <= MIDPOINT_FLAT_THRESHOLD_PCT
        && max_pair_abs_residual_share_bound19 <= SIMPLE_STABLE_MAX_PAIR_DOMINANCE_THRESHOLD;
    let proof_next_family_candidate = promising_family
        && simple_stable_family
        && max_pair_abs_residual_share_bound19 <= SINGLE_PAIR_DOMINANCE_THRESHOLD;

    FamilySummaryRow {
        family_kind,
        family_key,
        pair_count,
        fair_pair_count,
        anchor_pair_count,
        simple_family,
        sign_stable_share,
        direction_purity,
        dominant_direction,
        material_residual_share,
        midpoint_flat_share,
        positive_delta_share,
        negative_delta_share,
        median_abs_corrected_delta_bound19,
        median_corrected_delta_bound19,
        median_midpoint_lift_pct,
        max_pair_abs_residual_share_bound19,
        promising_family,
        simple_stable_family,
        proof_next_family_candidate,
    }
}

fn median(mut values: Vec<f64>) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.sort_by(|left, right| left.total_cmp(right));
    let mid = values.len() / 2;
    if values.len() % 2 == 1 {
        values[mid]
    } else {
        (values[mid - 1] + values[mid]) / 2.0
    }
}

fn collect_sorted_top_pairs(bundle: &SummaryBundle) -> Vec<PairSummaryRow> {
    let mut rows: Vec<_> = bundle
        .pair_summary
        .iter()
        .filter(|row| row.fair_midpoint_candidate)
        .cloned()
        .collect();
    rows.sort_by(|left, right| {
        right
            .bound19_corrected_residual_ratio_delta
            .abs()
            .total_cmp(&left.bound19_corrected_residual_ratio_delta.abs())
            .then_with(|| left.pair_id.cmp(&right.pair_id))
    });
    rows
}

fn collect_sorted_coherent_families(bundle: &SummaryBundle) -> Vec<FamilySummaryRow> {
    let mut rows: Vec<_> = bundle
        .family_summary
        .iter()
        .filter(|row| row.promising_family)
        .cloned()
        .collect();
    rows.sort_by(|left, right| {
        right
            .simple_stable_family
            .cmp(&left.simple_stable_family)
            .then_with(|| right.direction_purity.total_cmp(&left.direction_purity))
            .then_with(|| right.sign_stable_share.total_cmp(&left.sign_stable_share))
            .then_with(|| {
                right
                    .median_abs_corrected_delta_bound19
                    .total_cmp(&left.median_abs_corrected_delta_bound19)
            })
            .then_with(|| left.family_kind.cmp(&right.family_kind))
            .then_with(|| left.family_key.cmp(&right.family_key))
    });
    rows
}

fn collect_simple_stable_families(coherent: &[FamilySummaryRow]) -> Vec<FamilySummaryRow> {
    coherent
        .iter()
        .filter(|row| row.simple_stable_family)
        .cloned()
        .collect()
}

fn print_ranked_summary(bundle: &SummaryBundle) {
    println!("=== Insertion Pair Taxonomy Scan ===\n");
    println!("Output directory: {}", bundle.settings.out_dir);
    let generated_count = bundle
        .pair_summary
        .iter()
        .filter(|row| row.source_kind == "generated")
        .count();
    let fair_count = bundle
        .pair_summary
        .iter()
        .filter(|row| row.fair_midpoint_candidate)
        .count();
    let control_count = bundle
        .pair_summary
        .iter()
        .filter(|row| row.is_control)
        .count();
    println!(
        "Catalog: {} generated pairs, {} fair pairs, {} control pair",
        generated_count, fair_count, control_count
    );
    println!();

    println!("Top fair pairs by |delta@19|");
    println!("----------------------------");
    let top_pairs = collect_sorted_top_pairs(bundle);
    for row in top_pairs.into_iter().take(TOP_PAIR_SUMMARY_COUNT) {
        println!(
            "  - {:<18} delta@19 {:+.3} | stable={} material={} midpoint_flat={} | len {} | gap {} | {}",
            row.pair_id,
            row.bound19_corrected_residual_ratio_delta,
            yn(row.sign_stable),
            yn(row.material_residual),
            yn(row.midpoint_flat),
            row.pair_digit_length,
            row.gap_tertile,
            row.pair_label
        );
    }
    println!();

    println!("Coherent families");
    println!("-----------------");
    let promising = collect_sorted_coherent_families(bundle);
    if promising.is_empty() {
        println!("  none");
    } else {
        for row in promising.iter().take(TOP_FAMILY_SUMMARY_COUNT) {
            println!(
                "  - {:<24} {:<12} n={} lean={} purity={:.0}% stable={:.0}% median|d|={:.3} focus={}",
                row.family_kind,
                row.family_key,
                row.pair_count,
                direction_label_short(&row.dominant_direction),
                row.direction_purity * 100.0,
                row.sign_stable_share * 100.0,
                row.median_abs_corrected_delta_bound19,
                yn(row.simple_stable_family)
            );
        }
        if promising.len() > TOP_FAMILY_SUMMARY_COUNT {
            println!(
                "  ... {} more coherent families in family_summary.csv",
                promising.len() - TOP_FAMILY_SUMMARY_COUNT
            );
        }
    }
    println!();

    println!("Simple stable families");
    println!("----------------------");
    let proof_next = collect_simple_stable_families(&promising);
    if proof_next.is_empty() {
        println!("  none");
    } else {
        for row in proof_next.iter().take(TOP_FAMILY_SUMMARY_COUNT) {
            println!(
                "  - {} = {} (n={}, lean={}, purity={:.0}%, stable={:.0}%, median|d|={:.3}, midpoint={:+.2}%)",
                row.family_kind,
                row.family_key,
                row.pair_count,
                direction_label_short(&row.dominant_direction),
                row.direction_purity * 100.0,
                row.sign_stable_share * 100.0,
                row.median_abs_corrected_delta_bound19,
                row.median_midpoint_lift_pct
            );
        }
        if proof_next.len() > TOP_FAMILY_SUMMARY_COUNT {
            println!(
                "  ... {} more simple stable families in family_summary.csv",
                proof_next.len() - TOP_FAMILY_SUMMARY_COUNT
            );
        }
    }
    println!();
    println!("Artifacts written:");
    println!(
        "  - {}",
        Path::new(&bundle.settings.out_dir)
            .join("pair_summary.csv")
            .display()
    );
    println!(
        "  - {}",
        Path::new(&bundle.settings.out_dir)
            .join("bound_sweep.csv")
            .display()
    );
    println!(
        "  - {}",
        Path::new(&bundle.settings.out_dir)
            .join("position_rows.csv")
            .display()
    );
    println!(
        "  - {}",
        Path::new(&bundle.settings.out_dir)
            .join("family_summary.csv")
            .display()
    );
    println!(
        "  - {}",
        Path::new(&bundle.settings.out_dir)
            .join("summary.json")
            .display()
    );
    println!(
        "  - {}",
        Path::new(&bundle.settings.out_dir)
            .join("report.md")
            .display()
    );
}

fn render_markdown_report(bundle: &SummaryBundle) -> String {
    let generated_count = bundle
        .pair_summary
        .iter()
        .filter(|row| row.source_kind == "generated")
        .count();
    let fair_count = bundle
        .pair_summary
        .iter()
        .filter(|row| row.fair_midpoint_candidate)
        .count();
    let control_count = bundle
        .pair_summary
        .iter()
        .filter(|row| row.is_control)
        .count();
    let top_pairs = collect_sorted_top_pairs(bundle);
    let coherent = collect_sorted_coherent_families(bundle);
    let simple_stable = collect_simple_stable_families(&coherent);

    let mut lines = vec![
        "# Insertion Pair Taxonomy Report".to_string(),
        String::new(),
        "_Generated from `historical/examples/insertion_pair_taxonomy_scan.rs`._".to_string(),
        String::new(),
        "## Run".to_string(),
        String::new(),
        format!("- Generated at: `{}`", bundle.generated_at_utc),
        format!("- Output directory: `{}`", bundle.settings.out_dir),
        format!(
            "- Catalog: `{}` generated pairs, `{}` fair pairs, `{}` control pair",
            generated_count, fair_count, control_count
        ),
        format!("- Widths: `{:?}`", bundle.settings.widths),
        format!("- Digits: `{:?}`", bundle.settings.digits),
        format!("- Residue moduli: `{:?}`", bundle.settings.residue_moduli),
        format!(
            "- Requested small-prime correction: `<= {}` via `{:?}`",
            bundle.settings.requested_small_prime_bound, bundle.settings.requested_small_primes
        ),
        format!("- Midpoint radius: `{}`", bundle.settings.midpoint_radius),
        String::new(),
        "## Focus".to_string(),
        String::new(),
        format!("- Coherent families found: `{}`", coherent.len()),
        format!("- Simple stable families found: `{}`", simple_stable.len()),
    ];

    if simple_stable.is_empty() {
        lines.push("- No simple stable family met the tightened screen.".to_string());
    } else {
        lines.push("- Tightest surviving family rules:".to_string());
        for family in &simple_stable {
            lines.push(format!(
                "  - `{}` = `{}` leaning `{}` with purity `{:.0}%`, sign-stable share `{:.0}%`, median `|delta@19| = {:.3}`",
                family.family_kind,
                family.family_key,
                family.dominant_direction,
                family.direction_purity * 100.0,
                family.sign_stable_share * 100.0,
                family.median_abs_corrected_delta_bound19
            ));
        }
    }

    lines.extend([
        String::new(),
        "## Top Fair Pairs".to_string(),
        String::new(),
        "| Pair | delta@19 | Stable | Material | Midpoint Flat | Label |".to_string(),
        "|---|---:|:---:|:---:|:---:|---|".to_string(),
    ]);
    for row in top_pairs.iter().take(TOP_PAIR_SUMMARY_COUNT) {
        lines.push(format!(
            "| `{}` | `{:+.3}` | `{}` | `{}` | `{}` | {} |",
            row.pair_id,
            row.bound19_corrected_residual_ratio_delta,
            yn(row.sign_stable),
            yn(row.material_residual),
            yn(row.midpoint_flat),
            row.pair_label
        ));
    }

    lines.extend([
        String::new(),
        "## Coherent Families".to_string(),
        String::new(),
        "| Family | Key | n | Lean | Purity | Stable | median |d| | Focus |".to_string(),
        "|---|---|---:|---|---:|---:|---:|:---:|".to_string(),
    ]);
    for row in coherent.iter().take(TOP_FAMILY_SUMMARY_COUNT) {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` | `{:.0}%` | `{:.0}%` | `{:.3}` | `{}` |",
            row.family_kind,
            row.family_key,
            row.pair_count,
            row.dominant_direction,
            row.direction_purity * 100.0,
            row.sign_stable_share * 100.0,
            row.median_abs_corrected_delta_bound19,
            yn(row.simple_stable_family)
        ));
    }

    lines.extend([
        String::new(),
        "## Artifacts".to_string(),
        String::new(),
        format!("- `{}/pair_summary.csv`", bundle.settings.out_dir),
        format!("- `{}/bound_sweep.csv`", bundle.settings.out_dir),
        format!("- `{}/position_rows.csv`", bundle.settings.out_dir),
        format!("- `{}/family_summary.csv`", bundle.settings.out_dir),
        format!("- `{}/summary.json`", bundle.settings.out_dir),
        format!("- `{}/report.md`", bundle.settings.out_dir),
        String::new(),
        "## Notes".to_string(),
        String::new(),
        "- This report is exploratory and should be treated as hypothesis-ranking infrastructure, not claim promotion.".to_string(),
        "- The tightened family screen rewards simple bucket rules, pair-level sign stability, family-level directional coherence, and low single-pair dominance.".to_string(),
    ]);

    lines.join("\n")
}

fn yn(value: bool) -> &'static str {
    if value {
        "Y"
    } else {
        "N"
    }
}

fn direction_label_short(label: &str) -> &'static str {
    match label {
        "forward" => "F",
        "reverse" => "R",
        _ => "M",
    }
}

fn midpoint_u128(left: u128, right: u128) -> u128 {
    left / 2 + right / 2 + ((left % 2 + right % 2) / 2)
}

fn is_prime_u128(value: u128) -> bool {
    is_prime(&num_bigint::BigUint::from(value))
}

fn decimal_digits(value: u128) -> usize {
    value.to_string().len()
}

fn digit_sum_u128(value: u128) -> u32 {
    value
        .to_string()
        .bytes()
        .map(|byte| (byte - b'0') as u32)
        .sum()
}

fn zero_density_u128(value: u128) -> f64 {
    let text = value.to_string();
    let zeros = text.bytes().filter(|&byte| byte == b'0').count();
    zeros as f64 / text.len() as f64
}

fn zero_density_bucket(avg_zero_density: f64) -> &'static str {
    if avg_zero_density == 0.0 {
        "none"
    } else if avg_zero_density <= 0.25 {
        "some"
    } else {
        "high"
    }
}

fn contains_zero_u128(value: u128) -> bool {
    value.to_string().bytes().any(|byte| byte == b'0')
}

fn is_palindrome_u128(value: u128) -> bool {
    let text = value.to_string();
    text.bytes().eq(text.bytes().rev())
}
