// Elbow Event Extraction: Detecting Honorary Zero Dynamics
//
// Analyzes solution space CSV data to identify rare "elbow events" where
// optimal zero-padding (k*) INCREASES as middle length (M) grows.
//
// These events reveal honorary zero dynamics: the membrane construction
// occasionally responds to middle growth by expanding the zero-padding
// "elbow room" around the symmetry axis.
//
// Outputs JSON for Manim animation pipeline.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

// ============================================================================
// CSV Data Structures
// ============================================================================

#[derive(Debug, Deserialize, Clone)]
struct SolutionSpaceRow {
    base: u32,
    #[serde(rename = "M")]
    m: u32,
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
    min_length: u32,
    max_length: u32,
}

#[derive(Debug, Deserialize, Clone)]
struct ExtendedMRow {
    base: u32,
    #[serde(rename = "M")]
    m: u32,
    outer: u32,
    inner: u32,
    k: u32,
    samples: u64,
    prime_count: u64,
    density: f64,
}

// ============================================================================
// Output JSON Structures
// ============================================================================

#[derive(Debug, Serialize, Clone)]
struct KSlice {
    k: u32,
    total_candidates: u64,
    prime_count: u64,
    density: f64,
}

#[derive(Debug, Serialize, Clone)]
struct ElbowEvent {
    // Configuration
    base: u32,
    outer: u32,
    inner: u32,

    // Transition
    m_before: u32,
    m_after: u32,
    k_star_before: u32,
    k_star_after: u32,

    // Density changes
    density_before_at_k_star: f64,
    density_after_at_k_star: f64,
    density_jump: f64,
    density_jump_percentage: f64,

    // Full k-sweep data for animation
    rows_before: Vec<KSlice>,
    rows_after: Vec<KSlice>,

    // Context
    midpoint: f64,
    base_properties: BaseProperties,
    boundary_properties: BoundaryProperties,

    // Significance
    event_type: String,
    statistical_notes: String,
}

#[derive(Debug, Serialize, Clone)]
struct BaseProperties {
    phi_base: u32,       // Euler's totient
    tau_base: u32,       // Number of divisors
    rad_base: u32,       // Radical
}

#[derive(Debug, Serialize, Clone)]
struct BoundaryProperties {
    outer_gcd: u32,
    inner_gcd: u32,
    outer_is_prime: bool,
    inner_is_prime: bool,
    coprime_pair: bool,
}

#[derive(Debug, Serialize)]
struct ElbowEventReport {
    metadata: ReportMetadata,
    events: Vec<ElbowEvent>,
    summary: EventSummary,
}

#[derive(Debug, Serialize)]
struct ReportMetadata {
    generated_at: String,
    data_sources: Vec<String>,
    total_configurations_analyzed: usize,
    detection_criteria: String,
}

#[derive(Debug, Serialize)]
struct EventSummary {
    total_elbow_events: usize,
    events_by_base: HashMap<u32, usize>,
    m_transitions: HashMap<String, usize>,
    avg_density_jump: f64,
    max_density_jump: f64,
    honorary_zero_context: String,
}

// ============================================================================
// Data Processing
// ============================================================================

#[derive(Debug, Clone)]
struct BestK {
    m: u32,
    best_k: u32,
    density: f64,
}

fn read_solution_space_csv(path: &str) -> Result<Vec<SolutionSpaceRow>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut rows = Vec::new();

    for result in rdr.deserialize() {
        let row: SolutionSpaceRow = result?;
        rows.push(row);
    }

    Ok(rows)
}

fn read_extended_m_csv(path: &str) -> Result<Vec<ExtendedMRow>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut rows = Vec::new();

    for result in rdr.deserialize() {
        let row: ExtendedMRow = result?;
        rows.push(row);
    }

    Ok(rows)
}

fn find_best_k(rows: &[SolutionSpaceRow]) -> BestK {
    let mut best_density = -1.0_f64;
    let mut best_k = 0;
    let m = rows[0].m;

    for row in rows {
        if row.density > best_density {
            best_density = row.density;
            best_k = row.k;
        } else if (row.density - best_density).abs() < 1e-12 && row.k < best_k {
            // Tie-breaker: prefer smaller k (minimal padding principle)
            best_k = row.k;
        }
    }

    BestK {
        m,
        best_k,
        density: best_density,
    }
}

fn create_k_slices(rows: &[SolutionSpaceRow]) -> Vec<KSlice> {
    let mut sorted = rows.to_vec();
    sorted.sort_by_key(|r| r.k);

    sorted.iter().map(|r| KSlice {
        k: r.k,
        total_candidates: r.total_candidates,
        prime_count: r.prime_count,
        density: r.density,
    }).collect()
}

fn detect_elbow_events(solution_space: &[SolutionSpaceRow]) -> Vec<ElbowEvent> {
    // Group by (base, M, outer, inner)
    let mut groups: HashMap<(u32, u32, u32, u32), Vec<SolutionSpaceRow>> = HashMap::new();

    for row in solution_space {
        groups
            .entry((row.base, row.m, row.outer, row.inner))
            .or_insert_with(Vec::new)
            .push(row.clone());
    }

    // Group by (base, outer, inner) to track M evolution
    let mut by_boundary: HashMap<(u32, u32, u32), Vec<BestK>> = HashMap::new();

    for ((base, _m, outer, inner), grp) in groups.iter() {
        let best = find_best_k(grp);
        by_boundary
            .entry((*base, *outer, *inner))
            .or_insert_with(Vec::new)
            .push(best);
    }

    // Find elbow events: k* increases when M increases
    let mut events = Vec::new();

    for ((base, outer, inner), mut seq) in by_boundary.into_iter() {
        seq.sort_by_key(|bk| bk.m);

        for window in seq.windows(2) {
            let before = &window[0];
            let after = &window[1];

            // Elbow event: M increases by 1 AND k* increases
            if after.m == before.m + 1 && after.best_k > before.best_k {
                // Extract full k-sweep data
                let rows_before = groups.get(&(base, before.m, outer, inner)).unwrap();
                let rows_after = groups.get(&(base, after.m, outer, inner)).unwrap();

                let k_slices_before = create_k_slices(rows_before);
                let k_slices_after = create_k_slices(rows_after);

                // Get representative row for metadata
                let rep = &rows_before[0];

                // Calculate density jump
                let density_jump = after.density - before.density;
                let density_jump_pct = if before.density > 0.0 {
                    (density_jump / before.density) * 100.0
                } else {
                    0.0
                };

                // Classify event type
                let event_type = if after.best_k == before.best_k + 1 {
                    format!("Single-step elbow (k: {} → {})", before.best_k, after.best_k)
                } else {
                    format!("Multi-step elbow (k: {} → {})", before.best_k, after.best_k)
                };

                // Statistical notes
                let statistical_notes = if density_jump > 0.0 {
                    format!(
                        "Positive density response: {:.2}pp gain with padding expansion. \
                         This is a RARE configuration that benefits from elbow room.",
                        density_jump * 100.0
                    )
                } else {
                    format!(
                        "Density decreased {:.2}pp despite k* shift (unusual). \
                         May indicate noise or subtle base-specific effects.",
                        density_jump.abs() * 100.0
                    )
                };

                events.push(ElbowEvent {
                    base,
                    outer,
                    inner,
                    m_before: before.m,
                    m_after: after.m,
                    k_star_before: before.best_k,
                    k_star_after: after.best_k,
                    density_before_at_k_star: before.density,
                    density_after_at_k_star: after.density,
                    density_jump,
                    density_jump_percentage: density_jump_pct,
                    rows_before: k_slices_before,
                    rows_after: k_slices_after,
                    midpoint: rep.midpoint,
                    base_properties: BaseProperties {
                        phi_base: rep.phi_base,
                        tau_base: rep.tau_base,
                        rad_base: rep.rad_base,
                    },
                    boundary_properties: BoundaryProperties {
                        outer_gcd: rep.outer_gcd,
                        inner_gcd: rep.inner_gcd,
                        outer_is_prime: rep.outer_is_prime,
                        inner_is_prime: rep.inner_is_prime,
                        coprime_pair: rep.outer_gcd == 1 && rep.inner_gcd == 1,
                    },
                    event_type,
                    statistical_notes,
                });
            }
        }
    }

    events.sort_by(|a, b| {
        // Sort by density jump (descending) for most dramatic events first
        b.density_jump.partial_cmp(&a.density_jump).unwrap()
    });

    events
}

fn create_summary(events: &[ElbowEvent]) -> EventSummary {
    let mut events_by_base: HashMap<u32, usize> = HashMap::new();
    let mut m_transitions: HashMap<String, usize> = HashMap::new();
    let mut total_density_jump = 0.0_f64;
    let mut max_density_jump = 0.0_f64;

    for event in events {
        *events_by_base.entry(event.base).or_insert(0) += 1;

        let transition = format!("M={} → M={}", event.m_before, event.m_after);
        *m_transitions.entry(transition).or_insert(0) += 1;

        total_density_jump += event.density_jump.abs();
        max_density_jump = max_density_jump.max(event.density_jump.abs());
    }

    let avg_density_jump = if !events.is_empty() {
        total_density_jump / events.len() as f64
    } else {
        0.0
    };

    let honorary_zero_context = format!(
        "Elbow events are RARE exceptions to k*=0 universality. They occur when the \
         membrane construction's middle (M) grows and the system gains local benefit \
         from expanding zero-padding around the honorary zero (symmetry axis). These \
         represent structured pockets in parameter space where 'elbow room' dynamics \
         temporarily override the minimal padding principle. Found {} events across \
         {} total configurations analyzed.",
        events.len(),
        events.len()  // Placeholder - will be updated in main
    );

    EventSummary {
        total_elbow_events: events.len(),
        events_by_base,
        m_transitions,
        avg_density_jump,
        max_density_jump,
        honorary_zero_context,
    }
}

// ============================================================================
// Main Execution
// ============================================================================

fn main() -> Result<(), Box<dyn Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║        Elbow Event Extraction: Honorary Zero Dynamics        ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    // Read CSV files
    println!("📁 Reading solution_space_complete.csv...");
    let solution_space = read_solution_space_csv("solution_space_complete.csv")?;
    println!("   ✓ Loaded {} rows", solution_space.len());

    // Optional: Try to read extended_m_results.csv if it exists
    let extended_exists = std::path::Path::new("extended_m_results.csv").exists();
    if extended_exists {
        println!("📁 Reading extended_m_results.csv...");
        let extended = read_extended_m_csv("extended_m_results.csv")?;
        println!("   ✓ Loaded {} rows", extended.len());
        println!("   ⚠ Note: Extended M results use sampling (not exhaustive enumeration)");
        println!("   → Including only solution_space_complete.csv for rigorous detection");
    }
    println!();

    // Detect elbow events
    println!("🔍 Detecting elbow events (k* increases when M increases)...");
    let events = detect_elbow_events(&solution_space);
    println!("   ✓ Found {} elbow events", events.len());
    println!();

    // Display events
    if events.is_empty() {
        println!("   No elbow events detected in dataset.");
        println!("   This indicates k*=0 universality is extremely strong.");
    } else {
        println!("   Elbow Events Detected:");
        println!("   ────────────────────────────────────────────────────────────");
        for (i, event) in events.iter().enumerate() {
            println!(
                "   {}. Base {}, ({}, {}): M={} → M={}, k*={} → k*={}, Δdensity={:+.4}",
                i + 1,
                event.base,
                event.outer,
                event.inner,
                event.m_before,
                event.m_after,
                event.k_star_before,
                event.k_star_after,
                event.density_jump
            );
        }
        println!("   ────────────────────────────────────────────────────────────");
    }
    println!();

    // Create summary
    let mut summary = create_summary(&events);
    summary.honorary_zero_context = format!(
        "Elbow events are RARE exceptions to k*=0 universality. They occur when the \
         membrane construction's middle (M) grows and the system gains local benefit \
         from expanding zero-padding around the honorary zero (symmetry axis). These \
         represent structured pockets in parameter space where 'elbow room' dynamics \
         temporarily override the minimal padding principle. Found {} events across \
         {} total configurations analyzed.",
        events.len(),
        solution_space.len()
    );

    // Create report
    let report = ElbowEventReport {
        metadata: ReportMetadata {
            generated_at: chrono::Local::now().to_rfc3339(),
            data_sources: vec![
                "solution_space_complete.csv (exhaustive M≤3)".to_string(),
            ],
            total_configurations_analyzed: solution_space.len(),
            detection_criteria:
                "k* increases when M increases by 1 (consecutive M transitions)".to_string(),
        },
        events,
        summary,
    };

    // Write JSON output
    println!("💾 Writing elbow_events.json...");
    let file = File::create("elbow_events.json")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &report)?;
    println!("   ✓ JSON written successfully");
    println!();

    // Summary statistics
    println!("📊 Summary:");
    println!("   • Total elbow events: {}", report.summary.total_elbow_events);
    println!("   • Average density jump: {:.4}", report.summary.avg_density_jump);
    println!("   • Maximum density jump: {:.4}", report.summary.max_density_jump);
    println!();
    println!("   Events by base:");
    for (base, count) in report.summary.events_by_base.iter() {
        println!("     - Base {}: {} events", base, count);
    }
    println!();
    println!("   M transitions:");
    for (transition, count) in report.summary.m_transitions.iter() {
        println!("     - {}: {} events", transition, count);
    }
    println!();

    println!("✅ Complete! Ready for Manim animation pipeline.");
    println!();
    println!("Next steps:");
    println!("  1. Review elbow_events.json for event details");
    println!("  2. Run: python visualizations/manim_elbow_room.py");
    println!("  3. Output animations will be in animations/ directory");

    Ok(())
}
