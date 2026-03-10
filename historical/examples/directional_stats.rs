//! Directional Statistics Scanner
//!
//! Purpose: Full-scale connector scan with per-prime elimination tracking
//! to investigate the -1.95% directional asymmetry in prime concatenations.
//!
//! This module tracks:
//! - Per-small-prime elimination counts (forward vs reverse)
//! - Binary metrics: ones_density, bit_length, trailing_zeros
//! - Decimal metrics: zero_density in connector
//!
//! Outputs:
//! - directional_stats_forward.csv
//! - directional_stats_reverse.csv
//! - directional_ghosts.csv (connectors prime in at least one direction)
//! - directional_summary.txt
//!
//! Usage:
//! ```bash
//! cargo run --release --example directional_stats -- 10301 3007003007003 5 7
//! ```

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime_miller_rabin;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write as IoWrite, BufWriter};
use std::str::FromStr;

const SMALL_PRIMES: &[u64] = &[3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

#[derive(Clone, Copy, Debug)]
enum Direction {
    Forward,
    Reverse,
}

impl Direction {
    fn name(&self) -> &'static str {
        match self {
            Direction::Forward => "forward",
            Direction::Reverse => "reverse",
        }
    }
}

#[derive(Debug, Clone)]
struct BinaryMetrics {
    ones_density_sum: f64,
    ones_count_sum: u64,
    bit_length_sum: u64,
    trailing_zeros_dist: HashMap<u32, u64>,
    samples: u64,
}

impl BinaryMetrics {
    fn new() -> Self {
        Self {
            ones_density_sum: 0.0,
            ones_count_sum: 0,
            bit_length_sum: 0,
            trailing_zeros_dist: HashMap::new(),
            samples: 0,
        }
    }

    fn record(&mut self, n: &BigUint) {
        let bit_length = n.bits() as u64;
        let ones_count = n.count_ones() as u64;
        let ones_density = if bit_length > 0 {
            ones_count as f64 / bit_length as f64
        } else {
            0.0
        };
        let trailing_zeros = count_trailing_zeros(n);

        self.ones_density_sum += ones_density;
        self.ones_count_sum += ones_count;
        self.bit_length_sum += bit_length;
        *self.trailing_zeros_dist.entry(trailing_zeros).or_insert(0) += 1;
        self.samples += 1;
    }

    fn mean_ones_density(&self) -> f64 {
        if self.samples > 0 {
            self.ones_density_sum / self.samples as f64
        } else {
            0.0
        }
    }

    fn mean_ones_count(&self) -> f64 {
        if self.samples > 0 {
            self.ones_count_sum as f64 / self.samples as f64
        } else {
            0.0
        }
    }

    fn mean_bit_length(&self) -> f64 {
        if self.samples > 0 {
            self.bit_length_sum as f64 / self.samples as f64
        } else {
            0.0
        }
    }
}

struct DirectionalStats {
    direction: Direction,
    tested: u64,
    skipped_mod3: u64,
    eliminations: HashMap<u64, u64>,  // prime -> count
    passed_small_primes: u64,
    mr_composite: u64,
    primes_found: u64,
    binary_metrics_all: BinaryMetrics,
    binary_metrics_primes: BinaryMetrics,
    zero_density_sum_primes: f64,
}

impl DirectionalStats {
    fn new(direction: Direction) -> Self {
        Self {
            direction,
            tested: 0,
            skipped_mod3: 0,
            eliminations: HashMap::new(),
            passed_small_primes: 0,
            mr_composite: 0,
            primes_found: 0,
            binary_metrics_all: BinaryMetrics::new(),
            binary_metrics_primes: BinaryMetrics::new(),
            zero_density_sum_primes: 0.0,
        }
    }

    fn record_skipped_mod3(&mut self) {
        self.skipped_mod3 += 1;
    }

    fn record_tested(&mut self) {
        self.tested += 1;
    }

    fn record_eliminated(&mut self, p: u64) {
        *self.eliminations.entry(p).or_insert(0) += 1;
    }

    fn record_passed_small_primes(&mut self) {
        self.passed_small_primes += 1;
    }

    fn record_mr_composite(&mut self) {
        self.mr_composite += 1;
    }

    fn record_prime(&mut self) {
        self.primes_found += 1;
    }

    fn write_csv(&self, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        writeln!(
            writer,
            "prime,tested,skipped_mod3,eliminated_by,passed_small_primes,mr_composite,primes_found"
        )?;

        for &p in SMALL_PRIMES {
            let elim = self.eliminations.get(&p).copied().unwrap_or(0);
            writeln!(
                writer,
                "{},{},{},{},{},{},{}",
                p,
                self.tested,
                self.skipped_mod3,
                elim,
                self.passed_small_primes,
                self.mr_composite,
                self.primes_found
            )?;
        }

        writer.flush()?;
        Ok(())
    }
}

fn count_trailing_zeros(n: &BigUint) -> u32 {
    if n.is_zero() {
        return 0;
    }

    let mut count = 0u32;
    let mut temp = n.clone();
    let two = BigUint::from(2u32);

    while &temp % &two == BigUint::zero() {
        count += 1;
        temp /= &two;
    }

    count
}

fn digit_sum_mod3(s: &str) -> u32 {
    s.chars()
        .filter_map(|c| c.to_digit(10))
        .fold(0u32, |acc, d| (acc + d) % 3)
}

fn zero_density(connector: &str) -> f64 {
    let zeros = connector.chars().filter(|&c| c == '0').count();
    zeros as f64 / connector.len() as f64
}

fn concat_forward(p1: &str, p2: &str, connector: &str) -> BigUint {
    let full = format!("{}{}{}", p1, connector, p2);
    BigUint::from_str(&full).unwrap()
}

fn concat_reverse(p1: &str, p2: &str, connector: &str) -> BigUint {
    let full = format!("{}{}{}", p2, connector, p1);
    BigUint::from_str(&full).unwrap()
}

fn small_prime_elimination(n: &BigUint, stats: &mut DirectionalStats) -> Option<u64> {
    for &p in SMALL_PRIMES {
        let p_big = BigUint::from(p);
        if (n % &p_big).is_zero() {
            stats.record_eliminated(p);
            return Some(p);
        }
    }
    None
}

struct ConnectorResult {
    is_prime: bool,
    bit_length: u64,
    ones_count: u64,
    ones_density: f64,
    trailing_zeros: u32,
}

fn analyze_direction(
    direction: Direction,
    p_left: &str,
    p_right: &str,
    connector: &str,
    stats: &mut DirectionalStats,
) -> ConnectorResult {
    stats.record_tested();

    let n = match direction {
        Direction::Forward => concat_forward(p_left, p_right, connector),
        Direction::Reverse => concat_reverse(p_left, p_right, connector),
    };

    // Binary metrics
    let bit_length = n.bits();
    let ones_count = n.count_ones();
    let ones_density = if bit_length > 0 {
        ones_count as f64 / bit_length as f64
    } else {
        0.0
    };
    let trailing_zeros = count_trailing_zeros(&n);

    // Record for all candidates
    stats.binary_metrics_all.record(&n);

    // Check small prime elimination
    if let Some(_) = small_prime_elimination(&n, stats) {
        return ConnectorResult {
            is_prime: false,
            bit_length,
            ones_count,
            ones_density,
            trailing_zeros,
        };
    }

    stats.record_passed_small_primes();

    // Miller-Rabin primality test
    let is_prime = is_prime_miller_rabin(&n);

    if is_prime {
        stats.record_prime();
        stats.binary_metrics_primes.record(&n);
        stats.zero_density_sum_primes += zero_density(connector);
    } else {
        stats.record_mr_composite();
    }

    ConnectorResult {
        is_prime,
        bit_length,
        ones_count,
        ones_density,
        trailing_zeros,
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 5 {
        eprintln!("Usage: {} <p1> <p2> <min_len> <max_len>", args[0]);
        eprintln!("Example: {} 10301 3007003007003 5 7", args[0]);
        std::process::exit(1);
    }

    let p_left = &args[1];
    let p_right = &args[2];
    let min_len: usize = args[3].parse().expect("min_len must be integer");
    let max_len: usize = args[4].parse().expect("max_len must be integer");

    println!("🔬 Directional Statistics Scanner");
    println!("{}", "=".repeat(70));
    println!("Prime pair: {} and {}", p_left, p_right);
    println!("Connector lengths: {}-{}", min_len, max_len);
    println!();

    let mut fwd_stats = DirectionalStats::new(Direction::Forward);
    let mut rev_stats = DirectionalStats::new(Direction::Reverse);

    // Open ghosts CSV
    let ghosts_file = File::create("collab/directional_ghosts.csv")?;
    let mut ghosts_writer = BufWriter::new(ghosts_file);
    writeln!(
        ghosts_writer,
        "connector,len,class,fwd_bits,rev_bits,fwd_ones,rev_ones,fwd_density,rev_density,fwd_trailing,rev_trailing"
    )?;

    let total_lengths: Vec<usize> = (min_len..=max_len).collect();

    for &len in &total_lengths {
        let max = 10u64.pow(len as u32);
        println!("Processing length {}... (0 to {})", len, max - 1);

        for i in 0..max {
            let connector = format!("{:0width$}", i, width = len);

            // Mod-3 prefilter
            let c_mod3 = digit_sum_mod3(&connector);
            if c_mod3 == 2 {
                fwd_stats.record_skipped_mod3();
                rev_stats.record_skipped_mod3();
                continue;
            }

            // Analyze both directions
            let fwd_result = analyze_direction(
                Direction::Forward,
                p_left,
                p_right,
                &connector,
                &mut fwd_stats,
            );

            let rev_result = analyze_direction(
                Direction::Reverse,
                p_left,
                p_right,
                &connector,
                &mut rev_stats,
            );

            // Write to ghosts CSV if prime in at least one direction
            let class = match (fwd_result.is_prime, rev_result.is_prime) {
                (true, true) => "both",
                (true, false) => "forward_only",
                (false, true) => "reverse_only",
                (false, false) => continue,
            };

            writeln!(
                ghosts_writer,
                "{},{},{},{},{},{},{},{:.8},{:.8},{},{}",
                connector,
                len,
                class,
                fwd_result.bit_length,
                rev_result.bit_length,
                fwd_result.ones_count,
                rev_result.ones_count,
                fwd_result.ones_density,
                rev_result.ones_density,
                fwd_result.trailing_zeros,
                rev_result.trailing_zeros,
            )?;
        }

        println!("  Length {} complete. Forward: {}, Reverse: {}",
                 len, fwd_stats.primes_found, rev_stats.primes_found);
    }

    ghosts_writer.flush()?;

    // Write per-direction stats
    fwd_stats.write_csv("collab/directional_stats_forward.csv")?;
    rev_stats.write_csv("collab/directional_stats_reverse.csv")?;

    // Write summary
    let mut summary = File::create("collab/directional_summary.txt")?;
    writeln!(summary, "DIRECTIONAL STATISTICS SUMMARY")?;
    writeln!(summary, "{}", "=".repeat(70))?;
    writeln!(summary)?;

    writeln!(summary, "Forward Direction:")?;
    writeln!(summary, "  Tested:             {}", fwd_stats.tested)?;
    writeln!(summary, "  Skipped (mod 3):    {}", fwd_stats.skipped_mod3)?;
    writeln!(summary, "  Passed small primes: {}", fwd_stats.passed_small_primes)?;
    writeln!(summary, "  MR composite:       {}", fwd_stats.mr_composite)?;
    writeln!(summary, "  Primes found:       {}", fwd_stats.primes_found)?;
    writeln!(summary)?;
    writeln!(summary, "  Binary Metrics (Primes Only):")?;
    writeln!(summary, "    Mean ones_density: {:.6}", fwd_stats.binary_metrics_primes.mean_ones_density())?;
    writeln!(summary, "    Mean ones_count:   {:.2}", fwd_stats.binary_metrics_primes.mean_ones_count())?;
    writeln!(summary, "    Mean bit_length:   {:.2}", fwd_stats.binary_metrics_primes.mean_bit_length())?;
    writeln!(summary, "    Mean zero_density: {:.6}",
             if fwd_stats.primes_found > 0 {
                 fwd_stats.zero_density_sum_primes / fwd_stats.primes_found as f64
             } else {
                 0.0
             })?;
    writeln!(summary)?;

    writeln!(summary, "Reverse Direction:")?;
    writeln!(summary, "  Tested:             {}", rev_stats.tested)?;
    writeln!(summary, "  Skipped (mod 3):    {}", rev_stats.skipped_mod3)?;
    writeln!(summary, "  Passed small primes: {}", rev_stats.passed_small_primes)?;
    writeln!(summary, "  MR composite:       {}", rev_stats.mr_composite)?;
    writeln!(summary, "  Primes found:       {}", rev_stats.primes_found)?;
    writeln!(summary)?;
    writeln!(summary, "  Binary Metrics (Primes Only):")?;
    writeln!(summary, "    Mean ones_density: {:.6}", rev_stats.binary_metrics_primes.mean_ones_density())?;
    writeln!(summary, "    Mean ones_count:   {:.2}", rev_stats.binary_metrics_primes.mean_ones_count())?;
    writeln!(summary, "    Mean bit_length:   {:.2}", rev_stats.binary_metrics_primes.mean_bit_length())?;
    writeln!(summary, "    Mean zero_density: {:.6}",
             if rev_stats.primes_found > 0 {
                 rev_stats.zero_density_sum_primes / rev_stats.primes_found as f64
             } else {
                 0.0
             })?;
    writeln!(summary)?;

    writeln!(summary, "Asymmetry Analysis:")?;
    writeln!(summary, "  Prime count delta:  {} ({:+.2}%)",
             fwd_stats.primes_found as i64 - rev_stats.primes_found as i64,
             ((fwd_stats.primes_found as f64 - rev_stats.primes_found as f64) /
              fwd_stats.primes_found as f64 * 100.0))?;

    writeln!(summary)?;
    writeln!(summary, "Per-Prime Elimination Deltas (Forward - Reverse):")?;
    for &p in SMALL_PRIMES {
        let fwd_elim = fwd_stats.eliminations.get(&p).copied().unwrap_or(0);
        let rev_elim = rev_stats.eliminations.get(&p).copied().unwrap_or(0);
        let delta = fwd_elim as i64 - rev_elim as i64;
        let delta_pct = if fwd_elim > 0 {
            delta as f64 / fwd_elim as f64 * 100.0
        } else {
            0.0
        };
        writeln!(summary, "  mod {:2}: {:+8} ({:+.3}%)", p, delta, delta_pct)?;
    }

    summary.flush()?;

    println!();
    println!("✅ Complete!");
    println!("Outputs:");
    println!("  - collab/directional_stats_forward.csv");
    println!("  - collab/directional_stats_reverse.csv");
    println!("  - collab/directional_ghosts.csv");
    println!("  - collab/directional_summary.txt");

    Ok(())
}
