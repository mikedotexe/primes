//! Unit-cycle base-neighbor report helpers.
//!
//! This scout compares nearby bases as normalized unit-cycle geometries. It is
//! deliberately geometry-first: neighboring bases can have very different
//! allowed digit cycles even before prime-density language enters the room.

use crate::validation::{
    affine_phase_residual::{
        build_affine_phase_residual_row, AffinePhaseResidualRow, DEFAULT_PHASE_RESIDUAL_TOP_LIMIT,
    },
    bounded_k::{digit_symbol, unit_residues},
    unit_cycle_phase::{annotate_unit_cycle_phase_row, normalize_unit_cycle_geometry},
};
use serde::Serialize;

pub const DEFAULT_UNIT_CYCLE_NEIGHBOR_BASES: &[u32] = &[56, 57, 58, 59, 60];
pub const DEFAULT_UNIT_CYCLE_NEIGHBOR_SCAN_MIDDLE_LENGTH: usize = 2;
pub const DEFAULT_UNIT_CYCLE_NEIGHBOR_FOCUS_MIDDLE_LENGTH: usize = 3;
pub const DEFAULT_UNIT_CYCLE_NEIGHBOR_TOP_LIMIT: usize = DEFAULT_PHASE_RESIDUAL_TOP_LIMIT;

#[derive(Debug, Clone, Serialize)]
pub struct UnitCycleBaseNeighborSettings {
    pub bases: Vec<u32>,
    pub scan_middle_length: usize,
    pub focus_middle_length: usize,
    pub top_limit: usize,
}

impl Default for UnitCycleBaseNeighborSettings {
    fn default() -> Self {
        Self {
            bases: DEFAULT_UNIT_CYCLE_NEIGHBOR_BASES.to_vec(),
            scan_middle_length: DEFAULT_UNIT_CYCLE_NEIGHBOR_SCAN_MIDDLE_LENGTH,
            focus_middle_length: DEFAULT_UNIT_CYCLE_NEIGHBOR_FOCUS_MIDDLE_LENGTH,
            top_limit: DEFAULT_UNIT_CYCLE_NEIGHBOR_TOP_LIMIT,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UnitCycleBaseGeometryRow {
    pub base: u32,
    pub factorization_label: String,
    pub unit_count: usize,
    pub unit_density: f64,
    pub normalized_radius: f64,
    pub normalized_circumference: f64,
    pub unit_arc_length: f64,
    pub adjacent_chord_length: f64,
    pub diameter_step_count: usize,
    pub diameter_pair_count: usize,
    pub complement_pair_count: usize,
    pub complement_diameter_count: usize,
    pub complement_adjacent_count: usize,
    pub first_units_label: String,
    pub sample_diameter_pairs_label: String,
    pub sample_complement_pairs_label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnitCycleNeighborDeltaRow {
    pub left_base: u32,
    pub right_base: u32,
    pub left_unit_count: usize,
    pub right_unit_count: usize,
    pub unit_count_delta: isize,
    pub left_adjacent_chord_length: f64,
    pub right_adjacent_chord_length: f64,
    pub adjacent_chord_delta: f64,
    pub interpretation: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnitCycleNeighborPhaseRow {
    pub selection_reason: String,
    pub base: u32,
    pub middle_length: usize,
    pub low_digit: u32,
    pub high_digit: u32,
    pub pair_label: String,
    pub reverse_pair_label: String,
    pub unit_count: usize,
    pub cyclic_distance: usize,
    pub distance_fraction: f64,
    pub distance_label: String,
    pub low_to_high_arc_label: String,
    pub edge_label: String,
    pub raw_delta_pp: f64,
    pub residual_after_size_pp: f64,
    pub residue_survivor_delta_pp: f64,
    pub survivor_prime_residual_delta_pp: f64,
    pub low_high_prime_hits: usize,
    pub high_low_prime_hits: usize,
    pub low_high_survivor_count: usize,
    pub high_low_survivor_count: usize,
    pub lead_tag: String,
    pub first_low_high_witness: String,
    pub first_high_low_witness: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnitCycleBaseNeighborSummary {
    pub base_count: usize,
    pub scan_middle_length: usize,
    pub focus_middle_length: usize,
    pub geometry_row_count: usize,
    pub scan_phase_row_count: usize,
    pub top_phase_row_count: usize,
    pub focus_phase_row_count: usize,
    pub base57_unit_count: usize,
    pub base58_unit_count: usize,
    pub base57_adjacent_chord_length: f64,
    pub base58_adjacent_chord_length: f64,
    pub strongest_scan_base: u32,
    pub strongest_scan_pair: String,
    pub strongest_scan_reverse_pair: String,
    pub strongest_scan_survivor_residual_pp: f64,
    pub strong_line: String,
    pub caution_line: String,
    pub payload_note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnitCycleBaseNeighborReport {
    pub settings: UnitCycleBaseNeighborSettings,
    pub summary: UnitCycleBaseNeighborSummary,
    pub base_geometry_rows: Vec<UnitCycleBaseGeometryRow>,
    pub neighbor_delta_rows: Vec<UnitCycleNeighborDeltaRow>,
    pub scan_phase_rows: Vec<UnitCycleNeighborPhaseRow>,
    pub top_phase_rows: Vec<UnitCycleNeighborPhaseRow>,
    pub focus_phase_rows: Vec<UnitCycleNeighborPhaseRow>,
}

pub fn build_unit_cycle_base_neighbor_report(
    settings: UnitCycleBaseNeighborSettings,
) -> UnitCycleBaseNeighborReport {
    let base_geometry_rows = build_base_geometry_rows(&settings.bases);
    let neighbor_delta_rows = build_neighbor_delta_rows(&base_geometry_rows);
    let scan_phase_rows = build_scan_phase_rows(&settings);
    let mut top_phase_rows = scan_phase_rows.clone();
    top_phase_rows.sort_by(|left, right| {
        right
            .survivor_prime_residual_delta_pp
            .abs()
            .partial_cmp(&left.survivor_prime_residual_delta_pp.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    top_phase_rows.truncate(settings.top_limit);
    let focus_phase_rows = build_focus_phase_rows(&settings);
    let summary = build_summary(
        &settings,
        &base_geometry_rows,
        &scan_phase_rows,
        &top_phase_rows,
        &focus_phase_rows,
    );

    UnitCycleBaseNeighborReport {
        settings,
        summary,
        base_geometry_rows,
        neighbor_delta_rows,
        scan_phase_rows,
        top_phase_rows,
        focus_phase_rows,
    }
}

fn build_base_geometry_rows(bases: &[u32]) -> Vec<UnitCycleBaseGeometryRow> {
    bases.iter().copied().map(build_base_geometry_row).collect()
}

fn build_base_geometry_row(base: u32) -> UnitCycleBaseGeometryRow {
    let units = unit_residues(base);
    let unit_count = units.len();
    let normalized_radius = 1.0;
    let normalized_circumference = 2.0 * std::f64::consts::PI;
    let unit_arc_length = normalized_circumference / unit_count as f64;
    let adjacent_chord_length = 2.0 * (std::f64::consts::PI / unit_count as f64).sin();
    let diameter_step_count = unit_count / 2;
    let diameter_pairs = diameter_pairs(base);
    let complement_pairs = complement_pairs(base);
    let complement_diameter_count = complement_pairs
        .iter()
        .filter(|&&(low, high)| {
            normalize_unit_cycle_geometry(base, low, high).distance_label == "diameter"
        })
        .count();
    let complement_adjacent_count = complement_pairs
        .iter()
        .filter(|&&(low, high)| {
            normalize_unit_cycle_geometry(base, low, high).distance_label == "adjacent"
        })
        .count();

    UnitCycleBaseGeometryRow {
        base,
        factorization_label: factorization_label(base),
        unit_count,
        unit_density: unit_count as f64 / (base - 1) as f64,
        normalized_radius,
        normalized_circumference,
        unit_arc_length,
        adjacent_chord_length,
        diameter_step_count,
        diameter_pair_count: diameter_pairs.len(),
        complement_pair_count: complement_pairs.len(),
        complement_diameter_count,
        complement_adjacent_count,
        first_units_label: digit_list_label(&units, 14),
        sample_diameter_pairs_label: pair_list_label(&diameter_pairs, 6),
        sample_complement_pairs_label: pair_list_label(&complement_pairs, 6),
    }
}

fn build_neighbor_delta_rows(rows: &[UnitCycleBaseGeometryRow]) -> Vec<UnitCycleNeighborDeltaRow> {
    rows.windows(2)
        .map(|pair| {
            let left = &pair[0];
            let right = &pair[1];
            let unit_count_delta = right.unit_count as isize - left.unit_count as isize;
            let adjacent_chord_delta = right.adjacent_chord_length - left.adjacent_chord_length;
            let interpretation = if unit_count_delta > 0 {
                "right base has denser unit-cycle beads".to_string()
            } else if unit_count_delta < 0 {
                "right base has sparser unit-cycle beads".to_string()
            } else {
                "neighbor bases have equal unit-cycle bead counts".to_string()
            };
            UnitCycleNeighborDeltaRow {
                left_base: left.base,
                right_base: right.base,
                left_unit_count: left.unit_count,
                right_unit_count: right.unit_count,
                unit_count_delta,
                left_adjacent_chord_length: left.adjacent_chord_length,
                right_adjacent_chord_length: right.adjacent_chord_length,
                adjacent_chord_delta,
                interpretation,
            }
        })
        .collect()
}

fn build_scan_phase_rows(
    settings: &UnitCycleBaseNeighborSettings,
) -> Vec<UnitCycleNeighborPhaseRow> {
    let mut rows = Vec::new();
    for &base in &settings.bases {
        let units = unit_residues(base);
        for (left_idx, &low_digit) in units.iter().enumerate() {
            for &high_digit in units.iter().skip(left_idx + 1) {
                let residual = build_affine_phase_residual_row(
                    base,
                    settings.scan_middle_length,
                    low_digit,
                    high_digit,
                );
                rows.push(phase_row_from_residual("broad_m2_neighbor_scan", &residual));
            }
        }
    }
    rows
}

fn build_focus_phase_rows(
    settings: &UnitCycleBaseNeighborSettings,
) -> Vec<UnitCycleNeighborPhaseRow> {
    let mut specs = Vec::new();
    for &base in &settings.bases {
        if let Some((low, high)) = first_diameter_pair(base) {
            specs.push(("diameter_anchor", base, low, high));
        }
        let complement = (1, base - 1);
        if unit_residues(base).contains(&complement.1)
            && !specs.iter().any(|&(_, spec_base, low, high)| {
                spec_base == base && low == complement.0 && high == complement.1
            })
        {
            specs.push(("base_complement_edge", base, complement.0, complement.1));
        }
    }

    specs
        .into_iter()
        .map(|(reason, base, low_digit, high_digit)| {
            let residual = build_affine_phase_residual_row(
                base,
                settings.focus_middle_length,
                low_digit,
                high_digit,
            );
            phase_row_from_residual(reason, &residual)
        })
        .collect()
}

fn phase_row_from_residual(
    selection_reason: &str,
    residual: &AffinePhaseResidualRow,
) -> UnitCycleNeighborPhaseRow {
    let row = annotate_unit_cycle_phase_row(residual);
    UnitCycleNeighborPhaseRow {
        selection_reason: selection_reason.to_string(),
        base: row.base,
        middle_length: row.middle_length,
        low_digit: row.low_digit,
        high_digit: row.high_digit,
        pair_label: readable_pair_label(row.low_digit, row.high_digit),
        reverse_pair_label: readable_pair_label(row.high_digit, row.low_digit),
        unit_count: row.unit_count,
        cyclic_distance: row.cyclic_distance,
        distance_fraction: row.distance_fraction,
        distance_label: row.distance_label,
        low_to_high_arc_label: row.low_to_high_arc_label,
        edge_label: row.edge_label,
        raw_delta_pp: row.raw_delta_pp,
        residual_after_size_pp: row.residual_after_size_pp,
        residue_survivor_delta_pp: row.residue_survivor_delta_pp,
        survivor_prime_residual_delta_pp: row.survivor_prime_residual_delta_pp,
        low_high_prime_hits: row.low_high_prime_hits,
        high_low_prime_hits: row.high_low_prime_hits,
        low_high_survivor_count: row.low_high_survivor_count,
        high_low_survivor_count: row.high_low_survivor_count,
        lead_tag: row.lead_tag,
        first_low_high_witness: row.first_low_high_witness,
        first_high_low_witness: row.first_high_low_witness,
    }
}

fn build_summary(
    settings: &UnitCycleBaseNeighborSettings,
    geometry_rows: &[UnitCycleBaseGeometryRow],
    scan_phase_rows: &[UnitCycleNeighborPhaseRow],
    top_phase_rows: &[UnitCycleNeighborPhaseRow],
    focus_phase_rows: &[UnitCycleNeighborPhaseRow],
) -> UnitCycleBaseNeighborSummary {
    let base57 = geometry_rows.iter().find(|row| row.base == 57);
    let base58 = geometry_rows.iter().find(|row| row.base == 58);
    let strongest = top_phase_rows.first();
    UnitCycleBaseNeighborSummary {
        base_count: settings.bases.len(),
        scan_middle_length: settings.scan_middle_length,
        focus_middle_length: settings.focus_middle_length,
        geometry_row_count: geometry_rows.len(),
        scan_phase_row_count: scan_phase_rows.len(),
        top_phase_row_count: top_phase_rows.len(),
        focus_phase_row_count: focus_phase_rows.len(),
        base57_unit_count: base57.map(|row| row.unit_count).unwrap_or_default(),
        base58_unit_count: base58.map(|row| row.unit_count).unwrap_or_default(),
        base57_adjacent_chord_length: base57
            .map(|row| row.adjacent_chord_length)
            .unwrap_or_default(),
        base58_adjacent_chord_length: base58
            .map(|row| row.adjacent_chord_length)
            .unwrap_or_default(),
        strongest_scan_base: strongest.map(|row| row.base).unwrap_or_default(),
        strongest_scan_pair: strongest
            .map(|row| row.pair_label.clone())
            .unwrap_or_default(),
        strongest_scan_reverse_pair: strongest
            .map(|row| row.reverse_pair_label.clone())
            .unwrap_or_default(),
        strongest_scan_survivor_residual_pp: strongest
            .map(|row| row.survivor_prime_residual_delta_pp)
            .unwrap_or_default(),
        strong_line: "nearby bases can have sharply different unit-cycle bead densities, so neighbor-base geometry is a useful scout before density language starts".to_string(),
        caution_line: "the unit circle is a normalized explanatory map; arbitrary base58 payload conversion still requires preserving the represented integer or payload bytes".to_string(),
        payload_note: "for arbitrary base58-encoded payloads, dropping to base57 is a transcoding problem unless the payload is generated directly inside a constrained affine/residue grammar".to_string(),
    }
}

fn first_diameter_pair(base: u32) -> Option<(u32, u32)> {
    diameter_pairs(base).into_iter().next()
}

fn diameter_pairs(base: u32) -> Vec<(u32, u32)> {
    let units = unit_residues(base);
    if units.len() < 2 || !units.len().is_multiple_of(2) {
        return Vec::new();
    }
    let half = units.len() / 2;
    (0..half)
        .map(|idx| (units[idx], units[idx + half]))
        .collect()
}

fn complement_pairs(base: u32) -> Vec<(u32, u32)> {
    let units = unit_residues(base);
    units
        .iter()
        .copied()
        .filter_map(|low| {
            let high = base - low;
            if low < high && units.contains(&high) {
                Some((low, high))
            } else {
                None
            }
        })
        .collect()
}

fn factorization_label(base: u32) -> String {
    let mut remaining = base;
    let mut factor = 2;
    let mut parts = Vec::new();
    while factor * factor <= remaining {
        let mut exponent = 0;
        while remaining.is_multiple_of(factor) {
            remaining /= factor;
            exponent += 1;
        }
        if exponent == 1 {
            parts.push(factor.to_string());
        } else if exponent > 1 {
            parts.push(format!("{factor}^{exponent}"));
        }
        factor += if factor == 2 { 1 } else { 2 };
    }
    if remaining > 1 {
        parts.push(remaining.to_string());
    }
    parts.join(" * ")
}

fn digit_list_label(digits: &[u32], limit: usize) -> String {
    let mut labels = digits
        .iter()
        .take(limit)
        .map(readable_digit_label)
        .collect::<Vec<_>>();
    if digits.len() > limit {
        labels.push("...".to_string());
    }
    labels.join(" ")
}

fn pair_list_label(pairs: &[(u32, u32)], limit: usize) -> String {
    let mut labels = pairs
        .iter()
        .take(limit)
        .map(|&(low, high)| readable_pair_label(low, high))
        .collect::<Vec<_>>();
    if pairs.len() > limit {
        labels.push("...".to_string());
    }
    labels.join(" ")
}

fn readable_pair_label(low: u32, high: u32) -> String {
    format!(
        "({},{})",
        readable_digit_label(&low),
        readable_digit_label(&high)
    )
}

fn readable_digit_label(digit: &u32) -> String {
    if *digit < 36 {
        digit_symbol(*digit)
    } else {
        format!("[{digit}]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base57_and_base58_have_different_unit_cycle_density() {
        let base57 = build_base_geometry_row(57);
        let base58 = build_base_geometry_row(58);

        assert_eq!(base57.unit_count, 36);
        assert_eq!(base58.unit_count, 28);
        assert!(base57.adjacent_chord_length < base58.adjacent_chord_length);
        assert_eq!(base57.diameter_step_count, 18);
        assert_eq!(base58.diameter_step_count, 14);
    }

    #[test]
    fn diameter_and_complement_are_distinct_for_base57_and_base58() {
        let base57_diameter = normalize_unit_cycle_geometry(57, 1, 29);
        let base57_complement = normalize_unit_cycle_geometry(57, 1, 56);
        let base58_diameter = normalize_unit_cycle_geometry(58, 1, 31);
        let base58_complement = normalize_unit_cycle_geometry(58, 1, 57);

        assert_eq!(base57_diameter.distance_label, "diameter");
        assert_eq!(base58_diameter.distance_label, "diameter");
        assert_eq!(base57_complement.distance_label, "adjacent");
        assert_eq!(base58_complement.distance_label, "adjacent");
        assert_eq!(base57_complement.edge_label, "base_complement");
        assert_eq!(base58_complement.edge_label, "base_complement");
    }

    #[test]
    fn compact_neighbor_report_builds_expected_small_surface() {
        let report = build_unit_cycle_base_neighbor_report(UnitCycleBaseNeighborSettings {
            bases: vec![57, 58],
            scan_middle_length: 1,
            focus_middle_length: 1,
            top_limit: 6,
        });

        assert_eq!(report.base_geometry_rows.len(), 2);
        assert_eq!(report.neighbor_delta_rows.len(), 1);
        assert_eq!(report.scan_phase_rows.len(), 1008);
        assert_eq!(report.top_phase_rows.len(), 6);
        assert_eq!(report.summary.base57_unit_count, 36);
        assert_eq!(report.summary.base58_unit_count, 28);
    }
}
