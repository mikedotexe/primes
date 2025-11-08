//! ASCII Art Utilities - Beautiful console output for membrane primes
//! 
//! This module provides reusable ASCII art generation functions
//! that can be used throughout the codebase for visual output.

use std::fmt::Write as FmtWrite;

/// Generate a visual membrane structure diagram
pub fn membrane_diagram(base: u32, outer: u32, inner: u32, k_outer: u32, k_inner: u32, center: u32) -> String {
    let mut diagram = String::new();
    
    // Convert to appropriate base representation
    let outer_str = digit_to_string(outer, base);
    let inner_str = digit_to_string(inner, base);
    let center_str = digit_to_string(center, base);
    
    // Pattern line
    writeln!(&mut diagram, "  {} {} {} {} {} {} {} {} {}",
        outer_str,
        "0".repeat(k_outer as usize),
        inner_str,
        "0".repeat(k_inner as usize),
        center_str,
        "0".repeat(k_inner as usize),
        inner_str,
        "0".repeat(k_outer as usize),
        outer_str
    ).unwrap();
    
    // Visual connector line
    write!(&mut diagram, "  ").unwrap();
    write!(&mut diagram, "│{}│{}│{}│{}│",
        "─".repeat(k_outer as usize + 1),
        "─".repeat(k_inner as usize + 1),
        "─".repeat(k_inner as usize + 1),
        "─".repeat(k_outer as usize + 1)
    ).unwrap();
    
    diagram
}

/// Create a box with title
pub fn boxed_title(title: &str, width: usize) -> String {
    let mut result = String::new();
    let _padding = (width - title.len() - 2) / 2;
    
    writeln!(&mut result, "╔{}╗", "═".repeat(width - 2)).unwrap();
    writeln!(&mut result, "║{:^width$}║", title, width = width - 2).unwrap();
    writeln!(&mut result, "╚{}╝", "═".repeat(width - 2)).unwrap();
    
    result
}

/// Create a simple box around content
pub fn simple_box(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut result = String::new();
    
    writeln!(&mut result, "┌{}┐", "─".repeat(max_width + 2)).unwrap();
    for line in lines {
        writeln!(&mut result, "│ {line:<max_width$} │").unwrap();
    }
    writeln!(&mut result, "└{}┘", "─".repeat(max_width + 2)).unwrap();
    
    result
}

/// Generate a progress bar
pub fn progress_bar(current: f64, max: f64, width: usize, label: &str) -> String {
    let _percentage = (current / max * 100.0) as u32;
    let filled = ((current / max) * width as f64) as usize;
    let empty = width - filled;
    
    format!("{}: {}{}│ {:.1}%",
        label,
        "█".repeat(filled),
        "░".repeat(empty),
        current / max * 100.0
    )
}

/// Create a visual comparison chart
pub fn comparison_chart(items: Vec<(&str, f64)>, max_width: usize) -> String {
    let mut chart = String::new();
    let max_value = items.iter().map(|(_, v)| *v).fold(0.0, f64::max);
    
    for (label, value) in items {
        let bar_width = ((value / max_value) * max_width as f64) as usize;
        writeln!(&mut chart, "{:<15} │{}│ {:.1}%",
            label,
            "█".repeat(bar_width),
            value * 100.0
        ).unwrap();
    }
    
    chart
}

/// Draw a membrane structure with breathing visualization
pub fn breathing_diagram(k_outer: u32, k_inner: u32, symmetric: bool) -> String {
    if symmetric {
        format!(r#"
    Symmetric k=({},{}):
         3
        {} {}
       7   7
      {}     {}
     C       C
      {}     {}
       7   7
        {} {}
         3"#,
            k_outer, k_inner,
            "0".repeat(k_outer as usize), "0".repeat(k_outer as usize),
            "0".repeat(k_inner as usize), "0".repeat(k_inner as usize),
            "0".repeat(k_inner as usize), "0".repeat(k_inner as usize),
            "0".repeat(k_outer as usize), "0".repeat(k_outer as usize)
        )
    } else {
        format!(r#"
    Breathing k=({},{}):
         3
        {}
       3 3
      {}   {}
     C     C
      {}   {}
       3 3
        {}
         3"#,
            k_outer, k_inner,
            if k_outer > 0 { "╱╲" } else { "33" },
            "0".repeat(k_inner as usize), "0".repeat(k_inner as usize),
            "0".repeat(k_inner as usize), "0".repeat(k_inner as usize),
            if k_outer > 0 { "╲╱" } else { "33" }
        )
    }
}

/// Create a wave pattern to show resonance
pub fn resonance_wave(distance: u32, quality: &str) -> String {
    match quality {
        "good" => format!(r#"
    Distance {distance} resonance:
      ╱╲    ╱╲    ╱╲    ╱╲
     ╱  ╲  ╱  ╲  ╱  ╲  ╱  ╲
    ╱    ╲╱    ╲╱    ╲╱    ╲
    CONSTRUCTIVE ✓"#),
        
        "poor" => format!(r#"
    Distance {distance} resonance:
      ╱╲      ╱╲      ╱╲
     ╱  ╲  ╱╲╱  ╲  ╱╲╱  ╲
    ╱    ╲╱  ╲   ╲╱  ╲   ╲
    DESTRUCTIVE ✗"#),
        
        _ => format!(r#"
    Distance {distance} resonance:
    ─────────────────────
    NO OSCILLATION"#)
    }
}

/// Generate a centered banner
pub fn banner(text: &str, width: usize) -> String {
    let mut result = String::new();
    writeln!(&mut result, "{}", "═".repeat(width)).unwrap();
    writeln!(&mut result, "{text:^width$}").unwrap();
    writeln!(&mut result, "{}", "═".repeat(width)).unwrap();
    result
}

/// Create a visual representation of the 5-7 relationship
pub fn five_seven_diagram() -> &'static str {
    r#"
    The 5-7 Phenomenon:
    ═══════════════════
    
        5 ←────── 2 ──────→ 7
        ↑                   ↑
     Prime               Prime
        ↓                   ↓
        └─── Twin Primes ───┘
    
    • Distance = 2 (minimal prime gap)
    • Sum = 12 (highly composite)
    • Product = 35 (semiprime)
    • Ratio = 1.4 ≈ √2"#
}

/// Show GPU speedup visually
pub fn speedup_meter(original: f64, optimized: f64) -> String {
    let speedup = optimized / original;
    let bar_size = (speedup.log10() * 20.0) as usize;
    
    format!(r#"
    Performance Speedup: {:.0}x
    ═══════════════════════════
    
    CPU  │█│ {:.0} k/s
    GPU  │{}│ {:.1} M/s
    
    🚀 Acceleration achieved!"#,
        speedup,
        original / 1000.0,
        "█".repeat(bar_size.min(50)),
        optimized / 1_000_000.0
    )
}

/// Create an atomic prime visualization
pub fn atomic_prime(pattern: &str, value: &str, verified: bool) -> String {
    format!(r#"
    ⚛️  Atomic Prime Structure:
    {}
    → {} {}
    "#, 
        pattern, 
        value,
        if verified { "✓ VERIFIED" } else { "⏳ CHECKING..." }
    )
}

/// Helper to convert digit to appropriate base representation
fn digit_to_string(digit: u32, base: u32) -> String {
    match base {
        12 => match digit {
            10 => "A".to_string(),
            11 => "B".to_string(),
            _ => digit.to_string(),
        },
        16 => format!("{digit:X}"),
        _ => digit.to_string(),
    }
}

/// Create a summary statistics box
pub fn stats_box(title: &str, stats: Vec<(&str, String)>) -> String {
    let mut result = String::new();
    let max_label = stats.iter().map(|(l, _)| l.len()).max().unwrap_or(0);
    let max_value = stats.iter().map(|(_, v)| v.len()).max().unwrap_or(0);
    let width = max_label + max_value + 7;
    
    writeln!(&mut result, "╔{}╗", "═".repeat(width)).unwrap();
    writeln!(&mut result, "║{title:^width$}║").unwrap();
    writeln!(&mut result, "╠{}╣", "═".repeat(width)).unwrap();
    
    for (label, value) in stats {
        writeln!(&mut result, "║ {label:<max_label$} │ {value:<max_value$} ║"
        ).unwrap();
    }
    
    writeln!(&mut result, "╚{}╝", "═".repeat(width)).unwrap();
    
    result
}

/// Create a visual separator
pub fn separator(style: &str, width: usize) -> String {
    match style {
        "double" => "═".repeat(width),
        "single" => "─".repeat(width),
        "dotted" => "·".repeat(width),
        "wave" => "∿".repeat(width / 2),
        _ => "-".repeat(width),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_membrane_diagram() {
        let diagram = membrane_diagram(10, 3, 7, 1, 1, 5);
        assert!(diagram.contains("3 0 7 0 5 0 7 0 3"));
    }
    
    #[test]
    fn test_progress_bar() {
        let bar = progress_bar(30.2, 100.0, 20, "Density");
        assert!(bar.contains("30.2%"));
        assert!(bar.contains("██████"));
    }
}