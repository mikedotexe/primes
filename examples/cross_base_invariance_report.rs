//! Cross-base invariance scorecard for exact and empirical membrane behavior.
//!
//! This report separates three layers:
//! 1. exact arithmetic facts that truly survive change of base,
//! 2. bounded empirical behavior for symmetric membrane padding on a fixed
//!    cross-base grid,
//! 3. current maintained matched-control coverage and the tranche items it
//!    suggests next.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example cross_base_invariance_report
//! cargo run --example cross_base_invariance_report -- --out-dir /tmp/primes_cross_base_invariance --matched-samples 100
//! ```

use primes::{
    hzlib::num_theory::{factor, phi_from_factor},
    validation::{
        bounded_k::{
            digit_symbol, evaluate_pair_row, format_k, ordered_unit_pairs, select_smoke_pairs,
            summarize_pair_rows, unit_residues, KDominancePairRow, KDominanceSummaryRow,
            DEFAULT_BOUNDED_K_GRID,
        },
        matched_control::{
            run_cross_family_report, summarize_reports, MatchedControlRunSettings,
            MAINTAINED_MATCHED_CONTROL_FAMILIES,
        },
        reporting::{
            ensure_dir, export_timestamp_utc, write_csv_rows, write_json_pretty, write_text_file,
        },
    },
};
use rayon::prelude::*;
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    path::{Path, PathBuf},
};

const BASES: &[u32] = &[6, 10, 12, 14, 30];
const MIDDLE_LENGTHS: &[usize] = &[2, 3];
const DEFAULT_OUT_DIR: &str = "/tmp/primes_cross_base_invariance";
const DEFAULT_MATCHED_SAMPLES: usize = 5;
const REPORT_EXPORT_VERSION: u32 = 1;
const K00_SUPPORT_THRESHOLD: f64 = 0.75;
const K00_CONTRADICTION_THRESHOLD: f64 = 0.50;
const TOP_COUNTEREXAMPLE_COUNT: usize = 6;
const SMOKE_MAX_ORDERED_PAIRS_PER_BASE: usize = 8;
const SMOKE_PAIR_ANCHORS: &[(u32, u32, u32)] = &[(6, 1, 5), (10, 3, 3), (10, 3, 7), (30, 11, 7)];

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    full_catalog: bool,
    matched_samples: usize,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    bases: Vec<u32>,
    middle_lengths: Vec<usize>,
    k_grid: Vec<String>,
    pair_catalog_mode: String,
    max_ordered_pairs_per_base: Option<usize>,
    rayon_threads: usize,
    matched_samples: usize,
    matched_seed_lengths: Vec<usize>,
}

#[derive(Debug, Clone, Serialize)]
struct StructuralRow {
    base: u32,
    factorization: String,
    squarefree: bool,
    radical: u32,
    phi: u32,
    unit_residue_count: usize,
    unit_residues: String,
    complement_closed: bool,
    complement_orbit_count: usize,
    even_midpoint_excluded: bool,
    radical_equals_base: bool,
    radical_equals_phi: bool,
}

#[derive(Debug, Clone, Serialize)]
struct MatchedControlBaseRow {
    base: u32,
    maintained_family_count: usize,
    membrane_rate: f64,
    control_rate: f64,
    lift: f64,
    lift_ci_lo: f64,
    lift_ci_hi: f64,
    positive_q_families: usize,
}

#[derive(Debug, Clone, Serialize)]
struct ClaimScorecardRow {
    claim_id: String,
    claim_label: String,
    claim_kind: String,
    status: String,
    bases_tested: String,
    supporting_bases: String,
    contradicting_bases: String,
    coverage_note: String,
    evidence: String,
}

#[derive(Debug, Clone, Serialize)]
struct TrancheItemRow {
    priority: u8,
    title: String,
    rationale: String,
    source_claims: String,
    suggested_path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    structural_rows: Vec<StructuralRow>,
    k_dominance_pair_rows: Vec<KDominancePairRow>,
    k_dominance_summary_rows: Vec<KDominanceSummaryRow>,
    matched_control_base_rows: Vec<MatchedControlBaseRow>,
    claim_scorecard: Vec<ClaimScorecardRow>,
    tranche_items: Vec<TrancheItemRow>,
}

fn main() {
    let options = parse_args();
    let rayon_threads = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1);
    let _ = rayon::ThreadPoolBuilder::new()
        .num_threads(rayon_threads)
        .build_global();
    ensure_dir(&options.out_dir).expect("failed to create report output directory");

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        bases: BASES.to_vec(),
        middle_lengths: MIDDLE_LENGTHS.to_vec(),
        k_grid: DEFAULT_BOUNDED_K_GRID
            .iter()
            .map(|&config| format_k(config))
            .collect(),
        pair_catalog_mode: if options.full_catalog {
            "full".to_string()
        } else {
            "smoke".to_string()
        },
        max_ordered_pairs_per_base: if options.full_catalog {
            None
        } else {
            Some(SMOKE_MAX_ORDERED_PAIRS_PER_BASE)
        },
        rayon_threads,
        matched_samples: options.matched_samples,
        matched_seed_lengths: vec![2],
    };

    let structural_rows = build_structural_rows();
    write_csv_rows(
        options.out_dir.join("structural_rows.csv"),
        &structural_rows,
    )
    .expect("failed to write structural rows CSV");

    let k_dominance_pair_rows = build_k_dominance_pair_rows(options.full_catalog);
    write_csv_rows(
        options.out_dir.join("k_dominance_pair_rows.csv"),
        &k_dominance_pair_rows,
    )
    .expect("failed to write k-dominance pair CSV");

    let k_dominance_summary_rows = build_k_dominance_summary_rows(&k_dominance_pair_rows);
    write_csv_rows(
        options.out_dir.join("k_dominance_summary_rows.csv"),
        &k_dominance_summary_rows,
    )
    .expect("failed to write k-dominance summary CSV");

    let matched_control_base_rows = build_matched_control_base_rows(options.matched_samples);
    write_csv_rows(
        options.out_dir.join("matched_control_base_rows.csv"),
        &matched_control_base_rows,
    )
    .expect("failed to write matched-control CSV");
    let claim_scorecard = build_claim_scorecard(
        &settings,
        &structural_rows,
        &k_dominance_summary_rows,
        &matched_control_base_rows,
    );
    let tranche_items = build_tranche_items(
        &settings,
        &structural_rows,
        &k_dominance_summary_rows,
        &matched_control_base_rows,
        &claim_scorecard,
    );

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        structural_rows,
        k_dominance_pair_rows,
        k_dominance_summary_rows,
        matched_control_base_rows,
        claim_scorecard,
        tranche_items,
    };

    write_csv_rows(
        options.out_dir.join("claim_scorecard.csv"),
        &bundle.claim_scorecard,
    )
    .expect("failed to write claim scorecard CSV");
    write_csv_rows(
        options.out_dir.join("tranche_items.csv"),
        &bundle.tranche_items,
    )
    .expect("failed to write tranche items CSV");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle)
        .expect("failed to write report JSON");
    write_text_file(
        options.out_dir.join("report.md"),
        &render_markdown_report(&bundle),
    )
    .expect("failed to write report markdown");

    print_summary(&bundle);
}

fn parse_args() -> Options {
    let mut args = env::args().skip(1);
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut full_catalog = false;
    let mut matched_samples = DEFAULT_MATCHED_SAMPLES;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => {
                out_dir = PathBuf::from(parse_next::<String>(&mut args, "--out-dir"));
            }
            "--full" => {
                full_catalog = true;
            }
            "--matched-samples" => {
                matched_samples = parse_next::<usize>(&mut args, "--matched-samples");
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
        full_catalog,
        matched_samples,
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
    println!("Cross-base invariance report");
    println!();
    println!("Usage:");
    println!("  cargo run --example cross_base_invariance_report -- [options]");
    println!();
    println!("Options:");
    println!("  --out-dir <path>          Output directory for report artifacts (default: {DEFAULT_OUT_DIR})");
    println!("  --full                    Use the exhaustive ordered-pair catalog instead of the default smoke catalog");
    println!("  --matched-samples <n>     Samples per arm for the matched-control coverage snapshot (default: {DEFAULT_MATCHED_SAMPLES})");
}

fn build_structural_rows() -> Vec<StructuralRow> {
    BASES
        .iter()
        .copied()
        .map(|base| {
            let factors = factor(base as u64);
            let radical = radical_from_factor(&factors) as u32;
            let phi = phi_from_factor(&factors) as u32;
            let units = unit_residues(base);
            let complement_closed = units.iter().all(|&r| units.contains(&((base - r) % base)));
            let complement_orbit_count = units
                .iter()
                .map(|&r| r.min((base - r) % base))
                .collect::<BTreeSet<_>>()
                .len();
            StructuralRow {
                base,
                factorization: format_factorization(&factors),
                squarefree: factors.iter().all(|(_, exp)| *exp == 1),
                radical,
                phi,
                unit_residue_count: units.len(),
                unit_residues: format_residues(base, &units),
                complement_closed,
                complement_orbit_count,
                even_midpoint_excluded: if base % 2 == 0 {
                    !units.contains(&(base / 2))
                } else {
                    true
                },
                radical_equals_base: radical == base,
                radical_equals_phi: radical == phi,
            }
        })
        .collect()
}

fn build_k_dominance_pair_rows(full_catalog: bool) -> Vec<KDominancePairRow> {
    let tasks: Vec<_> = BASES
        .iter()
        .copied()
        .flat_map(|base| {
            let pairs = if full_catalog {
                ordered_unit_pairs(base)
            } else {
                let anchors = SMOKE_PAIR_ANCHORS
                    .iter()
                    .filter(|&&(anchor_base, _, _)| anchor_base == base)
                    .map(|&(_, outer, inner)| (outer, inner))
                    .collect::<Vec<_>>();
                select_smoke_pairs(base, SMOKE_MAX_ORDERED_PAIRS_PER_BASE, &anchors)
            };
            MIDDLE_LENGTHS
                .iter()
                .copied()
                .flat_map(move |middle_length| {
                    pairs
                        .clone()
                        .into_iter()
                        .map(move |(outer, inner)| (base, middle_length, outer, inner))
                })
        })
        .collect();

    let mut rows: Vec<_> = tasks
        .par_iter()
        .map(|&(base, middle_length, outer, inner)| {
            evaluate_pair_row(base, middle_length, outer, inner, DEFAULT_BOUNDED_K_GRID)
        })
        .collect();
    rows.sort_by(|left, right| {
        left.base
            .cmp(&right.base)
            .then_with(|| left.middle_length.cmp(&right.middle_length))
            .then_with(|| left.outer.cmp(&right.outer))
            .then_with(|| left.inner.cmp(&right.inner))
    });
    rows
}

fn build_k_dominance_summary_rows(pair_rows: &[KDominancePairRow]) -> Vec<KDominanceSummaryRow> {
    summarize_pair_rows(pair_rows)
}

fn build_matched_control_base_rows(samples: usize) -> Vec<MatchedControlBaseRow> {
    let settings = MatchedControlRunSettings {
        samples,
        min_seed_len: 2,
        max_seed_len: 2,
        ..MatchedControlRunSettings::default()
    };
    let reports = run_cross_family_report(&MAINTAINED_MATCHED_CONTROL_FAMILIES, settings);
    let summary = summarize_reports(&reports, settings);
    let family_counts = maintained_family_counts_by_base();

    let mut rows: Vec<_> = summary
        .base_summaries
        .iter()
        .map(|row| MatchedControlBaseRow {
            base: row.base,
            maintained_family_count: *family_counts.get(&row.base).unwrap_or(&0),
            membrane_rate: row.membrane.rate,
            control_rate: row.control.rate,
            lift: row.lift,
            lift_ci_lo: row.lift_ci.0,
            lift_ci_hi: row.lift_ci.1,
            positive_q_families: row.positive_q_families,
        })
        .collect();
    rows.sort_by_key(|row| row.base);
    rows
}

fn maintained_family_counts_by_base() -> BTreeMap<u32, usize> {
    let mut counts = BTreeMap::new();
    for family in MAINTAINED_MATCHED_CONTROL_FAMILIES {
        *counts.entry(family.base).or_default() += 1;
    }
    counts
}

fn build_claim_scorecard(
    settings: &ReportSettings,
    structural_rows: &[StructuralRow],
    k_summaries: &[KDominanceSummaryRow],
    matched_rows: &[MatchedControlBaseRow],
) -> Vec<ClaimScorecardRow> {
    let bases_tested = format_base_list(BASES);
    let unit_support = structural_rows
        .iter()
        .filter(|row| row.unit_residue_count as u32 == row.phi)
        .map(|row| row.base)
        .collect::<Vec<_>>();
    let unit_contra = structural_rows
        .iter()
        .filter(|row| row.unit_residue_count as u32 != row.phi)
        .map(|row| row.base)
        .collect::<Vec<_>>();
    let complement_support = structural_rows
        .iter()
        .filter(|row| row.complement_closed)
        .map(|row| row.base)
        .collect::<Vec<_>>();
    let complement_contra = structural_rows
        .iter()
        .filter(|row| !row.complement_closed)
        .map(|row| row.base)
        .collect::<Vec<_>>();
    let radical_contra = structural_rows
        .iter()
        .filter(|row| !row.radical_equals_base)
        .map(|row| row.base)
        .collect::<Vec<_>>();
    let radical_support = structural_rows
        .iter()
        .filter(|row| row.radical_equals_base)
        .map(|row| row.base)
        .collect::<Vec<_>>();

    let m2 = k_summaries
        .iter()
        .filter(|row| row.middle_length == 2)
        .collect::<Vec<_>>();
    let m3 = k_summaries
        .iter()
        .filter(|row| row.middle_length == 3)
        .collect::<Vec<_>>();

    let m2_support = m2
        .iter()
        .filter(|row| row.k00_noninferior_share >= K00_SUPPORT_THRESHOLD)
        .map(|row| row.base)
        .collect::<Vec<_>>();
    let m2_contra = m2
        .iter()
        .filter(|row| row.k00_noninferior_share <= K00_CONTRADICTION_THRESHOLD)
        .map(|row| row.base)
        .collect::<Vec<_>>();
    let m3_support = m3
        .iter()
        .filter(|row| row.k00_noninferior_share >= K00_SUPPORT_THRESHOLD)
        .map(|row| row.base)
        .collect::<Vec<_>>();
    let m3_contra = m3
        .iter()
        .filter(|row| row.k00_noninferior_share <= K00_CONTRADICTION_THRESHOLD)
        .map(|row| row.base)
        .collect::<Vec<_>>();

    let matched_bases = matched_rows.iter().map(|row| row.base).collect::<Vec<_>>();
    let unmatched_bases = BASES
        .iter()
        .copied()
        .filter(|base| !matched_bases.contains(base))
        .collect::<Vec<_>>();
    let matched_positive_q = matched_rows
        .iter()
        .filter(|row| row.positive_q_families > 0)
        .map(|row| row.base)
        .collect::<Vec<_>>();

    let pair_catalog_note = if settings.pair_catalog_mode == "full" {
        "Exhaustive scan on all ordered unit pairs in every tested base.".to_string()
    } else {
        format!(
            "Deterministic smoke catalog with up to {} ordered unit pairs per base, plus maintained anchor pairs.",
            settings
                .max_ordered_pairs_per_base
                .unwrap_or(SMOKE_MAX_ORDERED_PAIRS_PER_BASE)
        )
    };

    vec![
        ClaimScorecardRow {
            claim_id: "units_equal_totient".to_string(),
            claim_label: "Admissible residues are exactly the units, so the count is φ(base)."
                .to_string(),
            claim_kind: "exact_invariant".to_string(),
            status: claim_status(
                unit_contra.is_empty(),
                unit_support.len(),
                unit_contra.len(),
            ),
            bases_tested: bases_tested.clone(),
            supporting_bases: format_base_list(&unit_support),
            contradicting_bases: format_base_list(&unit_contra),
            coverage_note: "Exact arithmetic check on every tested base.".to_string(),
            evidence: "unit_residue_count == phi(base) on every tested base".to_string(),
        },
        ClaimScorecardRow {
            claim_id: "unit_complement_symmetry".to_string(),
            claim_label: "Unit residues stay closed under complement/negation across tested bases."
                .to_string(),
            claim_kind: "exact_invariant".to_string(),
            status: claim_status(
                complement_contra.is_empty(),
                complement_support.len(),
                complement_contra.len(),
            ),
            bases_tested: bases_tested.clone(),
            supporting_bases: format_base_list(&complement_support),
            contradicting_bases: format_base_list(&complement_contra),
            coverage_note: "Exact arithmetic check on every tested base.".to_string(),
            evidence: "complement_closed == true on every tested base".to_string(),
        },
        ClaimScorecardRow {
            claim_id: "radical_equals_base".to_string(),
            claim_label:
                "Naive interchangeability of base and radical survives cross-base pressure."
                    .to_string(),
            claim_kind: "refuted_naive_invariant".to_string(),
            status: if radical_contra.is_empty() {
                "supported".to_string()
            } else {
                "refuted".to_string()
            },
            bases_tested: bases_tested.clone(),
            supporting_bases: format_base_list(&radical_support),
            contradicting_bases: format_base_list(&radical_contra),
            coverage_note:
                "Non-squarefree bases matter here; base 12 is the decisive spoiler in this set."
                    .to_string(),
            evidence: "rad(base) != base on at least one tested base".to_string(),
        },
        ClaimScorecardRow {
            claim_id: "k00_m2_bounded_grid".to_string(),
            claim_label: "Minimal padding k=(0,0) dominates the bounded M=2 cross-base k-grid."
                .to_string(),
            claim_kind: "bounded_empirical".to_string(),
            status: empirical_status(&m2_support, &m2_contra, m2.len()),
            bases_tested: bases_tested.clone(),
            supporting_bases: format_base_list(&m2_support),
            contradicting_bases: format_base_list(&m2_contra),
            coverage_note: format!(
                "Bounded grid over k={{(0,0),(0,1),(1,0),(1,1),(2,2)}}. {}",
                pair_catalog_note
            ),
            evidence: format!(
                "k00_noninferior_share by base: {}",
                format_middle_length_summary(m2)
            ),
        },
        ClaimScorecardRow {
            claim_id: "k00_m3_bounded_grid".to_string(),
            claim_label: "Minimal padding k=(0,0) dominates the bounded M=3 cross-base k-grid."
                .to_string(),
            claim_kind: "bounded_empirical".to_string(),
            status: empirical_status(&m3_support, &m3_contra, m3.len()),
            bases_tested: bases_tested.clone(),
            supporting_bases: format_base_list(&m3_support),
            contradicting_bases: format_base_list(&m3_contra),
            coverage_note: format!(
                "Bounded grid over k={{(0,0),(0,1),(1,0),(1,1),(2,2)}}. {}",
                pair_catalog_note
            ),
            evidence: format!(
                "k00_noninferior_share by base: {}",
                format_middle_length_summary(m3)
            ),
        },
        ClaimScorecardRow {
            claim_id: "matched_control_multi_base".to_string(),
            claim_label: "Current maintained matched-control families show lift in multiple bases."
                .to_string(),
            claim_kind: "maintained_empirical".to_string(),
            status: if matched_positive_q.len() >= 2 {
                "supported".to_string()
            } else if matched_positive_q.is_empty() {
                "refuted".to_string()
            } else {
                "mixed".to_string()
            },
            bases_tested: format_base_list(&matched_bases),
            supporting_bases: format_base_list(&matched_positive_q),
            contradicting_bases: format_base_list(&unmatched_bases),
            coverage_note: if unmatched_bases.is_empty() {
                "Current maintained family set covers every base in this report.".to_string()
            } else {
                format!(
                    "Coverage gap: maintained matched-control family set does not currently include {}.",
                    format_base_list(&unmatched_bases)
                )
            },
            evidence: format!(
                "maintained matched-control bases present: {}",
                format_base_list(&matched_bases)
            ),
        },
    ]
}

fn build_tranche_items(
    settings: &ReportSettings,
    structural_rows: &[StructuralRow],
    k_summaries: &[KDominanceSummaryRow],
    matched_rows: &[MatchedControlBaseRow],
    claims: &[ClaimScorecardRow],
) -> Vec<TrancheItemRow> {
    let mut items = Vec::new();

    let covered_bases: Vec<u32> = matched_rows.iter().map(|row| row.base).collect();
    let missing_control_bases = BASES
        .iter()
        .copied()
        .filter(|base| !covered_bases.contains(base))
        .collect::<Vec<_>>();
    if !missing_control_bases.is_empty() {
        items.push(TrancheItemRow {
            priority: 1,
            title: "Extend maintained matched-control families to uncovered bases".to_string(),
            rationale: format!(
                "The current maintained control lane covers {}, but the cross-base scorecard also relies on {}. Adding at least one maintained family for {} would let the empirical invariance lane stop treating those bases as coverage gaps.",
                format_base_list(&covered_bases),
                format_base_list(BASES),
                format_base_list(&missing_control_bases)
            ),
            source_claims: "matched_control_multi_base".to_string(),
            suggested_path: "src/validation/matched_control.rs + examples/membrane_vs_random.rs"
                .to_string(),
        });
    }

    let m2 = k_summaries
        .iter()
        .filter(|row| row.middle_length == 2)
        .max_by(|left, right| {
            left.strongest_counterexample_margin_pp
                .total_cmp(&right.strongest_counterexample_margin_pp)
                .then_with(|| left.base.cmp(&right.base))
        });
    let m2_claim = claims
        .iter()
        .find(|row| row.claim_id == "k00_m2_bounded_grid")
        .expect("missing M=2 claim row");
    let m3_claim = claims
        .iter()
        .find(|row| row.claim_id == "k00_m3_bounded_grid")
        .expect("missing M=3 claim row");
    if m3_claim.status == "supported" && m2_claim.status != "supported" {
        items.push(TrancheItemRow {
            priority: 1,
            title: "Split the M=3 bounded-grid law candidate from the M=2 anomaly lane"
                .to_string(),
            rationale: "The cross-base scorecard supports a cleaner bounded M=3 story than M=2. Treating them separately would let the repo promote the stable part without dragging the anomaly-rich part along.".to_string(),
            source_claims: "k00_m2_bounded_grid,k00_m3_bounded_grid".to_string(),
            suggested_path: "new maintained report/example for M=3 bounded-grid dominance + separate anomaly reproducer for M=2".to_string(),
        });
    }

    if let Some(counterexample) = m2 {
        if counterexample.strongest_counterexample_margin_pp > 0.0 {
            items.push(TrancheItemRow {
                priority: 2,
                title: format!(
                    "Add a dedicated bounded-grid counterexample reproducer for base {} M={}",
                    counterexample.base, counterexample.middle_length
                ),
                rationale: format!(
                    "The strongest current bounded-grid counterexample in this lane is base {} {} with {} beating k=(0,0) by {:.2} percentage points. A dedicated reproducer would turn this from folklore into a tracked audit target.",
                    counterexample.base,
                    counterexample.strongest_counterexample_pair,
                    counterexample.strongest_counterexample_best_k,
                    counterexample.strongest_counterexample_margin_pp
                ),
                source_claims: "k00_m2_bounded_grid".to_string(),
                suggested_path: "historical/examples/base_specific_k_counterexample.rs"
                    .to_string(),
            });
        }
    }

    let non_squarefree_bases = structural_rows
        .iter()
        .filter(|row| !row.squarefree)
        .map(|row| row.base)
        .collect::<Vec<_>>();
    if !non_squarefree_bases.is_empty() {
        items.push(TrancheItemRow {
            priority: 2,
            title: "Promote a maintained radical-vs-totient cross-base table".to_string(),
            rationale: format!(
                "The exact scorecard shows non-squarefree bases {} break the naive habit of treating base, radical, and totient as interchangeable. Making that table maintained would strengthen the repo's fact/speculation boundary.",
                format_base_list(&non_squarefree_bases)
            ),
            source_claims: "radical_equals_base,units_equal_totient".to_string(),
            suggested_path: "new maintained example/docs lane for cross-base structural residues"
                .to_string(),
        });
    }

    if settings.pair_catalog_mode != "full" {
        items.push(TrancheItemRow {
            priority: 2,
            title: "Add a full-catalog rerun lane for the strongest smoke findings".to_string(),
            rationale: "The default scorecard is now intentionally smoke-sized. The next disciplined step after any interesting smoke signal is a bounded full-catalog rerun on the same bases and k-grid before promotion into claim language.".to_string(),
            source_claims: "k00_m2_bounded_grid,k00_m3_bounded_grid".to_string(),
            suggested_path: "examples/cross_base_invariance_report.rs --full + generated artifact diff".to_string(),
        });
    }

    items.sort_by(|left, right| {
        left.priority
            .cmp(&right.priority)
            .then_with(|| left.title.cmp(&right.title))
    });
    items
}

fn print_summary(bundle: &ReportBundle) {
    println!("=== Cross-Base Invariance Report ===\n");
    println!("Output directory: {}", bundle.settings.out_dir());
    println!(
        "Bases: {:?} | middle lengths {:?} | bounded k-grid {:?} | pair catalog {} | rayon threads {}",
        bundle.settings.bases,
        bundle.settings.middle_lengths,
        bundle.settings.k_grid,
        bundle.settings.pair_catalog_mode,
        bundle.settings.rayon_threads
    );
    println!();

    println!("Exact structural scorecard");
    println!("--------------------------");
    for row in &bundle.structural_rows {
        println!(
            "  - base {:>2}: rad={} phi={} units={} complement={} squarefree={}",
            row.base,
            row.radical,
            row.phi,
            row.unit_residue_count,
            yn(row.complement_closed),
            yn(row.squarefree)
        );
    }
    println!();

    println!("Bounded k-dominance summary");
    println!("---------------------------");
    for row in &bundle.k_dominance_summary_rows {
        println!(
            "  - base {:>2} M={}: k00 noninferior {:.0}% | strict/tied {}/{} of {} | strongest counterexample {} via {} at +{:.2}pp",
            row.base,
            row.middle_length,
            row.k00_noninferior_share * 100.0,
            row.k00_strict_best_pairs,
            row.k00_tied_best_pairs,
            row.ordered_pair_count,
            row.strongest_counterexample_pair,
            row.strongest_counterexample_best_k,
            row.strongest_counterexample_margin_pp
        );
    }
    println!();

    println!("Maintained matched-control coverage");
    println!("----------------------------------");
    for row in &bundle.matched_control_base_rows {
        println!(
            "  - base {:>2}: families={} lift {:.3} [{:.3}, {:.3}] positive-q={}",
            row.base,
            row.maintained_family_count,
            row.lift,
            row.lift_ci_lo,
            row.lift_ci_hi,
            row.positive_q_families
        );
    }
    println!();

    println!("Tranche items");
    println!("-------------");
    if bundle.tranche_items.is_empty() {
        println!("  none");
    } else {
        for row in &bundle.tranche_items {
            println!("  - [P{}] {}", row.priority, row.title);
        }
    }
    println!();
    println!("Artifacts written:");
    println!(
        "  - {}",
        Path::new(&bundle.settings.out_dir())
            .join("structural_rows.csv")
            .display()
    );
    println!(
        "  - {}",
        Path::new(&bundle.settings.out_dir())
            .join("k_dominance_pair_rows.csv")
            .display()
    );
    println!(
        "  - {}",
        Path::new(&bundle.settings.out_dir())
            .join("k_dominance_summary_rows.csv")
            .display()
    );
    println!(
        "  - {}",
        Path::new(&bundle.settings.out_dir())
            .join("matched_control_base_rows.csv")
            .display()
    );
    println!(
        "  - {}",
        Path::new(&bundle.settings.out_dir())
            .join("claim_scorecard.csv")
            .display()
    );
    println!(
        "  - {}",
        Path::new(&bundle.settings.out_dir())
            .join("tranche_items.csv")
            .display()
    );
    println!(
        "  - {}",
        Path::new(&bundle.settings.out_dir())
            .join("summary.json")
            .display()
    );
    println!(
        "  - {}",
        Path::new(&bundle.settings.out_dir())
            .join("report.md")
            .display()
    );
}

fn render_markdown_report(bundle: &ReportBundle) -> String {
    let mut counterexamples: Vec<_> = bundle
        .k_dominance_pair_rows
        .iter()
        .filter(|row| row.best_minus_k00_pp > 0.0)
        .collect();
    counterexamples.sort_by(|left, right| {
        right
            .best_minus_k00_pp
            .total_cmp(&left.best_minus_k00_pp)
            .then_with(|| left.base.cmp(&right.base))
            .then_with(|| left.middle_length.cmp(&right.middle_length))
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });
    counterexamples.truncate(TOP_COUNTEREXAMPLE_COUNT);

    let mut lines = vec![
        "# Cross-Base Invariance Report".to_string(),
        String::new(),
        "_Generated from `examples/cross_base_invariance_report.rs`._".to_string(),
        String::new(),
        "## Run".to_string(),
        String::new(),
        format!("- Generated at: `{}`", bundle.generated_at_utc),
        format!("- Bases: `{:?}`", bundle.settings.bases),
        format!("- Middle lengths: `{:?}`", bundle.settings.middle_lengths),
        format!("- Bounded k-grid: `{:?}`", bundle.settings.k_grid),
        format!("- Pair catalog: `{}`", bundle.settings.pair_catalog_mode),
        format!(
            "- Max ordered pairs/base: `{}`",
            bundle
                .settings
                .max_ordered_pairs_per_base
                .map(|value| value.to_string())
                .unwrap_or_else(|| "all".to_string())
        ),
        format!("- Rayon threads: `{}`", bundle.settings.rayon_threads),
        format!(
            "- Matched-control snapshot: `{} samples/arm`, seed lengths `{:?}`",
            bundle.settings.matched_samples, bundle.settings.matched_seed_lengths
        ),
        String::new(),
        "## Claim Scorecard".to_string(),
        String::new(),
        "| Claim | Kind | Status | Supporting | Contradicting |".to_string(),
        "|---|---|---|---|---|".to_string(),
    ];
    for row in &bundle.claim_scorecard {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | {} | {} |",
            row.claim_id, row.claim_kind, row.status, row.supporting_bases, row.contradicting_bases
        ));
    }

    lines.extend([
        String::new(),
        "## Exact Structural Rows".to_string(),
        String::new(),
        "| Base | Factors | rad | phi | Units | Complement Closed | Squarefree |".to_string(),
        "|---:|---|---:|---:|---:|:---:|:---:|".to_string(),
    ]);
    for row in &bundle.structural_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |",
            row.base,
            row.factorization,
            row.radical,
            row.phi,
            row.unit_residue_count,
            yn(row.complement_closed),
            yn(row.squarefree)
        ));
    }

    lines.extend([
        String::new(),
        "## Bounded k-Dominance".to_string(),
        String::new(),
        "| Base | M | k00 noninferior | strict best | tied best | strongest counterexample |"
            .to_string(),
        "|---:|---:|---:|---:|---:|---|".to_string(),
    ]);
    for row in &bundle.k_dominance_summary_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{:.0}%` | `{}` | `{}` | {} via {} at `{:.2}pp` |",
            row.base,
            row.middle_length,
            row.k00_noninferior_share * 100.0,
            row.k00_strict_best_pairs,
            row.k00_tied_best_pairs,
            row.strongest_counterexample_pair,
            row.strongest_counterexample_best_k,
            row.strongest_counterexample_margin_pp
        ));
    }

    if !counterexamples.is_empty() {
        lines.extend([
            String::new(),
            "## Representative Counterexamples".to_string(),
            String::new(),
            "| Base | M | Pair | k00 | Best k | Margin |".to_string(),
            "|---:|---:|---|---:|---|---:|".to_string(),
        ]);
        for row in counterexamples {
            lines.push(format!(
                "| `{}` | `{}` | {} | `{:.2}%` | `{}` | `{:.2}pp` |",
                row.base,
                row.middle_length,
                row.pair_label,
                row.rate_k00 * 100.0,
                row.best_k,
                row.best_minus_k00_pp
            ));
        }
    }

    lines.extend([
        String::new(),
        "## Maintained Matched-Control Coverage".to_string(),
        String::new(),
        "| Base | Families | Lift | CI | Positive-q families |".to_string(),
        "|---:|---:|---:|---|---:|".to_string(),
    ]);
    for row in &bundle.matched_control_base_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{:.3}` | `[{:.3}, {:.3}]` | `{}` |",
            row.base,
            row.maintained_family_count,
            row.lift,
            row.lift_ci_lo,
            row.lift_ci_hi,
            row.positive_q_families
        ));
    }

    lines.extend([String::new(), "## Tranche Items".to_string(), String::new()]);
    if bundle.tranche_items.is_empty() {
        lines.push("- None generated on this run.".to_string());
    } else {
        for item in &bundle.tranche_items {
            lines.push(format!(
                "- `[P{}]` **{}**: {}",
                item.priority, item.title, item.rationale
            ));
            lines.push(format!(
                "  Source: `{}` | Suggested path: `{}`",
                item.source_claims, item.suggested_path
            ));
        }
    }

    lines.extend([
        String::new(),
        "## Artifacts".to_string(),
        String::new(),
        format!("- `{}/structural_rows.csv`", bundle.settings.out_dir()),
        format!(
            "- `{}/k_dominance_pair_rows.csv`",
            bundle.settings.out_dir()
        ),
        format!(
            "- `{}/k_dominance_summary_rows.csv`",
            bundle.settings.out_dir()
        ),
        format!(
            "- `{}/matched_control_base_rows.csv`",
            bundle.settings.out_dir()
        ),
        format!("- `{}/claim_scorecard.csv`", bundle.settings.out_dir()),
        format!("- `{}/tranche_items.csv`", bundle.settings.out_dir()),
        format!("- `{}/summary.json`", bundle.settings.out_dir()),
        format!("- `{}/report.md`", bundle.settings.out_dir()),
    ]);

    lines.join("\n")
}

fn radical_from_factor(factors: &[(u64, u32)]) -> u64 {
    factors.iter().map(|(prime, _)| *prime).product()
}

fn format_factorization(factors: &[(u64, u32)]) -> String {
    factors
        .iter()
        .map(|(prime, exp)| {
            if *exp == 1 {
                prime.to_string()
            } else {
                format!("{prime}^{exp}")
            }
        })
        .collect::<Vec<_>>()
        .join(" * ")
}

fn format_residues(base: u32, residues: &[u32]) -> String {
    let _ = base;
    format!(
        "[{}]",
        residues
            .iter()
            .map(|&digit| digit_symbol(digit))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn claim_status(all_supported: bool, support_count: usize, contradiction_count: usize) -> String {
    if all_supported {
        "supported".to_string()
    } else if contradiction_count > 0 && support_count > 0 {
        "mixed".to_string()
    } else if contradiction_count > 0 {
        "refuted".to_string()
    } else {
        "incomplete".to_string()
    }
}

fn empirical_status(support: &[u32], contradiction: &[u32], total_rows: usize) -> String {
    if support.len() == total_rows {
        "supported".to_string()
    } else if !contradiction.is_empty() {
        "mixed".to_string()
    } else if support.is_empty() {
        "refuted".to_string()
    } else {
        "mixed".to_string()
    }
}

fn format_base_list(bases: &[u32]) -> String {
    if bases.is_empty() {
        "none".to_string()
    } else {
        bases
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn format_middle_length_summary(rows: Vec<&KDominanceSummaryRow>) -> String {
    rows.into_iter()
        .map(|row| format!("b{}={:.0}%", row.base, row.k00_noninferior_share * 100.0))
        .collect::<Vec<_>>()
        .join(", ")
}

fn yn(value: bool) -> &'static str {
    if value {
        "Y"
    } else {
        "N"
    }
}

impl ReportSettings {
    fn out_dir(&self) -> &str {
        &self.out_dir
    }
}
