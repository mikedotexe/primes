// Pattern Analyzer: Data-Driven Discovery Tool
//
// Loads the complete solution space CSV and automatically discovers patterns
// WITHOUT hypothesis bias. Computes correlations, identifies optimal k* for
// each configuration, detects anomalies, and generates natural groupings.
//
// This tool lets the DATA reveal what matters.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone)]
struct ConfigData {
    base: u32,
    m: usize,
    outer: u32,
    inner: u32,
    k: u32,
    total_candidates: u64,
    prime_count: u64,
    density: f64,
    midpoint: f64,
    phi_base: u32,
    tau_base: u32,
    rad_base: u32,
    outer_gcd: u32,
    inner_gcd: u32,
    outer_is_prime: bool,
    inner_is_prime: bool,
    min_length: usize,
    max_length: usize,
}

impl ConfigData {
    fn from_csv_line(line: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let fields: Vec<&str> = line.split(',').collect();

        Ok(ConfigData {
            base: fields[0].parse()?,
            m: fields[1].parse()?,
            outer: fields[2].parse()?,
            inner: fields[3].parse()?,
            k: fields[4].parse()?,
            total_candidates: fields[5].parse()?,
            prime_count: fields[6].parse()?,
            density: fields[7].parse()?,
            midpoint: fields[8].parse()?,
            phi_base: fields[9].parse()?,
            tau_base: fields[10].parse()?,
            rad_base: fields[11].parse()?,
            outer_gcd: fields[12].parse()?,
            inner_gcd: fields[13].parse()?,
            outer_is_prime: fields[14].parse::<String>()? == "true",
            inner_is_prime: fields[15].parse::<String>()? == "true",
            min_length: fields[16].parse()?,
            max_length: fields[17].parse()?,
        })
    }

    fn config_key(&self) -> String {
        format!("{}-{}-{}-{}", self.base, self.m, self.outer, self.inner)
    }
}

// ============================================================================
// Pattern Discovery
// ============================================================================

#[derive(Clone)]
struct OptimalKFinding {
    base: u32,
    m: usize,
    outer: u32,
    inner: u32,
    optimal_k: u32,
    optimal_density: f64,
    densities: Vec<(u32, f64)>,  // (k, density) for all k
}

struct PatternSummary {
    total_configs: usize,
    unique_settings: usize,

    // k* distribution
    k_star_distribution: HashMap<u32, usize>,

    // By M
    k_star_by_m: HashMap<usize, Vec<u32>>,

    // By base
    k_star_by_base: HashMap<u32, Vec<u32>>,

    // Anomalies (configs where k* > 0)
    anomalies: Vec<OptimalKFinding>,
}

fn load_data(filename: &str) -> Result<Vec<ConfigData>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        if i == 0 { continue; }  // Skip header

        let line = line?;
        if line.trim().is_empty() { continue; }

        match ConfigData::from_csv_line(&line) {
            Ok(config) => data.push(config),
            Err(e) => eprintln!("Error parsing line {}: {}", i, e),
        }
    }

    Ok(data)
}

fn find_optimal_k(configs: &[ConfigData]) -> Vec<OptimalKFinding> {
    let mut by_setting: HashMap<String, Vec<ConfigData>> = HashMap::new();

    // Group by (base, M, outer, inner)
    for config in configs {
        by_setting
            .entry(config.config_key())
            .or_insert_with(Vec::new)
            .push(config.clone());
    }

    let mut findings = Vec::new();

    for (_, group) in by_setting {
        let mut densities: Vec<(u32, f64)> = group
            .iter()
            .map(|c| (c.k, c.density))
            .collect();

        densities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        if let Some(first) = group.first() {
            if let Some(&(optimal_k, optimal_density)) = densities.first() {
                findings.push(OptimalKFinding {
                    base: first.base,
                    m: first.m,
                    outer: first.outer,
                    inner: first.inner,
                    optimal_k,
                    optimal_density,
                    densities,
                });
            }
        }
    }

    findings
}

fn analyze_patterns(findings: &[OptimalKFinding]) -> PatternSummary {
    let mut k_star_distribution = HashMap::new();
    let mut k_star_by_m = HashMap::new();
    let mut k_star_by_base = HashMap::new();
    let mut anomalies = Vec::new();

    for finding in findings {
        // Overall distribution
        *k_star_distribution.entry(finding.optimal_k).or_insert(0) += 1;

        // By M
        k_star_by_m
            .entry(finding.m)
            .or_insert_with(Vec::new)
            .push(finding.optimal_k);

        // By base
        k_star_by_base
            .entry(finding.base)
            .or_insert_with(Vec::new)
            .push(finding.optimal_k);

        // Anomalies (k* > 0)
        if finding.optimal_k > 0 {
            anomalies.push((*finding).clone());
        }
    }

    PatternSummary {
        total_configs: findings.len(),
        unique_settings: findings.len(),
        k_star_distribution,
        k_star_by_m,
        k_star_by_base,
        anomalies,
    }
}

// ============================================================================
// Reporting
// ============================================================================

fn print_summary(summary: &PatternSummary) {
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║           PATTERN DISCOVERY SUMMARY                   ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    println!("Total unique (base, M, outer, inner) configurations: {}", summary.total_configs);

    println!("\n────────────────────────────────────────────────────────");
    println!("OVERALL k* DISTRIBUTION");
    println!("────────────────────────────────────────────────────────\n");

    let mut k_values: Vec<_> = summary.k_star_distribution.keys().collect();
    k_values.sort();

    for k in k_values {
        let count = summary.k_star_distribution[k];
        let percent = 100.0 * count as f64 / summary.total_configs as f64;
        let bar = "█".repeat((percent / 2.0) as usize);
        println!("  k*={}: {:4} configs ({:5.1}%) {}", k, count, percent, bar);
    }

    println!("\n────────────────────────────────────────────────────────");
    println!("k* BY MIDDLE LENGTH (M)");
    println!("────────────────────────────────────────────────────────\n");

    let mut m_values: Vec<_> = summary.k_star_by_m.keys().collect();
    m_values.sort();

    for &m in m_values {
        let k_stars = &summary.k_star_by_m[&m];
        let k0_count = k_stars.iter().filter(|&&k| k == 0).count();
        let k_nonzero_count = k_stars.len() - k0_count;
        let k0_pct = 100.0 * k0_count as f64 / k_stars.len() as f64;

        println!("  M={}: {} total | k*=0: {} ({:.1}%) | k*>0: {}",
            m, k_stars.len(), k0_count, k0_pct, k_nonzero_count);
    }

    println!("\n────────────────────────────────────────────────────────");
    println!("k* BY BASE");
    println!("────────────────────────────────────────────────────────\n");

    let mut bases: Vec<_> = summary.k_star_by_base.keys().collect();
    bases.sort();

    for &base in bases {
        let k_stars = &summary.k_star_by_base[&base];
        let k0_count = k_stars.iter().filter(|&&k| k == 0).count();
        let k_nonzero_count = k_stars.len() - k0_count;
        let k0_pct = 100.0 * k0_count as f64 / k_stars.len() as f64;

        println!("  Base {:2}: {} configs | k*=0: {:4} ({:5.1}%) | k*>0: {:3}",
            base, k_stars.len(), k0_count, k0_pct, k_nonzero_count);
    }

    println!("\n────────────────────────────────────────────────────────");
    println!("ANOMALIES: Configurations with k* > 0");
    println!("────────────────────────────────────────────────────────\n");

    if summary.anomalies.is_empty() {
        println!("  ✓ UNIVERSAL k*=0: NO anomalies found!");
        println!("  All configurations prefer zero padding.");
    } else {
        println!("  Found {} configurations with k*>0:\n", summary.anomalies.len());

        for (i, anomaly) in summary.anomalies.iter().take(20).enumerate() {
            println!("  {}. Base {}, M={}, ({},{}) → k*={} (density={:.4})",
                i + 1,
                anomaly.base,
                anomaly.m,
                anomaly.outer,
                anomaly.inner,
                anomaly.optimal_k,
                anomaly.optimal_density
            );

            // Show density comparison
            print!("     Densities: ");
            for &(k, density) in &anomaly.densities {
                let marker = if k == anomaly.optimal_k { "★" } else { " " };
                print!("k={}: {:.4}{} | ", k, density, marker);
            }
            println!();
        }

        if summary.anomalies.len() > 20 {
            println!("\n  ... and {} more anomalies", summary.anomalies.len() - 20);
        }
    }
}

fn print_anomaly_analysis(anomalies: &[OptimalKFinding]) {
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║              ANOMALY DEEP DIVE                         ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    // Group anomalies by base
    let mut by_base: HashMap<u32, Vec<&OptimalKFinding>> = HashMap::new();
    for anomaly in anomalies {
        by_base.entry(anomaly.base).or_insert_with(Vec::new).push(anomaly);
    }

    println!("Anomalies by Base:\n");
    let mut bases: Vec<_> = by_base.keys().collect();
    bases.sort();

    for &base in bases {
        let anoms = &by_base[&base];
        println!("  Base {}: {} anomalies", base, anoms.len());

        // Group by M
        let mut by_m: HashMap<usize, usize> = HashMap::new();
        for anom in anoms {
            *by_m.entry(anom.m).or_insert(0) += 1;
        }

        let mut ms: Vec<_> = by_m.keys().collect();
        ms.sort();
        print!("    M distribution: ");
        for &m in ms {
            print!("M={}: {} | ", m, by_m[&m]);
        }
        println!();
    }

    // Common properties analysis
    println!("\n────────────────────────────────────────────────────────");
    println!("Do anomalies share common properties?");
    println!("────────────────────────────────────────────────────────\n");

    // Check M distribution
    let mut m_counts: HashMap<usize, usize> = HashMap::new();
    for anom in anomalies {
        *m_counts.entry(anom.m).or_insert(0) += 1;
    }

    print!("  M values: ");
    for (&m, &count) in &m_counts {
        let pct = 100.0 * count as f64 / anomalies.len() as f64;
        print!("M={}: {} ({:.1}%) | ", m, count, pct);
    }
    println!();

    // Check k* distribution among anomalies
    let mut k_counts: HashMap<u32, usize> = HashMap::new();
    for anom in anomalies {
        *k_counts.entry(anom.optimal_k).or_insert(0) += 1;
    }

    print!("  k* values: ");
    for (&k, &count) in &k_counts {
        let pct = 100.0 * count as f64 / anomalies.len() as f64;
        print!("k*={}: {} ({:.1}%) | ", k, count, pct);
    }
    println!("\n");
}

// ============================================================================
// Main Analysis
// ============================================================================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║        PATTERN ANALYZER: DATA-DRIVEN DISCOVERY        ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    println!("Loading solution space data...");
    let data = load_data("solution_space_complete.csv")?;
    println!("  ✓ Loaded {} configuration results\n", data.len());

    println!("Finding optimal k* for each configuration...");
    let findings = find_optimal_k(&data);
    println!("  ✓ Analyzed {} unique configurations\n", findings.len());

    println!("Discovering patterns...");
    let summary = analyze_patterns(&findings);

    print_summary(&summary);

    if !summary.anomalies.is_empty() {
        print_anomaly_analysis(&summary.anomalies);
    }

    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║                   NEXT STEPS                           ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    println!("The data has revealed its patterns. Key observations:");
    println!("  1. What is the k*=0 percentage across all configs?");
    println!("  2. Are there any bases with consistent k*>0?");
    println!("  3. Does M value correlate with k* behavior?");
    println!("  4. What makes anomalies special (if any exist)?\n");

    println!("No hypotheses imposed. Just truth from data. 🔬\n");

    Ok(())
}
