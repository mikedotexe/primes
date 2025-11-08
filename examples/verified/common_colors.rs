//! Common color scheme for all visualization tools
//! Unified branding and accessibility

use ratatui::style::Color;
pub struct PrimeColors;
impl PrimeColors {
    // Primary brand colors
    pub const PRIME_GREEN: Color = Color::Rgb(0, 255, 136);      // Bright green for primes
    pub const COMPOSITE_RED: Color = Color::Rgb(255, 68, 102);   // Red for composites
    pub const MEMBRANE_BLUE: Color = Color::Rgb(52, 152, 219);   // Blue for membranes
    
    // Membrane structure colors
    pub const OUTER_PURPLE: Color = Color::Rgb(155, 89, 182);    // Purple for outer digits
    pub const INNER_CYAN: Color = Color::Rgb(52, 152, 219);      // Cyan for inner digits
    pub const SEED_ORANGE: Color = Color::Rgb(243, 156, 18);     // Orange for seeds
    // UI colors
    pub const TITLE_CYAN: Color = Color::Rgb(26, 188, 156);      // Cyan for titles
    pub const WARNING_YELLOW: Color = Color::Rgb(241, 196, 15);  // Yellow for warnings
    pub const ERROR_RED: Color = Color::Rgb(231, 76, 60);        // Red for errors
    pub const SUCCESS_GREEN: Color = Color::Rgb(39, 174, 96);    // Green for success
    // Statistical colors
    pub const EXCELLENT: Color = Color::Rgb(39, 174, 96);        // >30% success
    pub const GOOD: Color = Color::Rgb(241, 196, 15);            // 20-30% success
    pub const FAIR: Color = Color::Rgb(230, 126, 34);            // 10-20% success
    pub const POOR: Color = Color::Rgb(231, 76, 60);             // <10% success
    // Heat map colors
    pub const HEAT_HIGH: Color = Color::Rgb(39, 174, 96);        // High performance
    pub const HEAT_MED: Color = Color::Rgb(241, 196, 15);        // Medium performance
    pub const HEAT_LOW: Color = Color::Rgb(231, 76, 60);         // Low performance
    pub const HEAT_INVALID: Color = Color::Rgb(127, 140, 141);   // Invalid configs
    // Text colors
    pub const TEXT_PRIMARY: Color = Color::Rgb(236, 240, 241);   // Primary text
    pub const TEXT_SECONDARY: Color = Color::Rgb(149, 165, 166); // Secondary text
    pub const TEXT_MUTED: Color = Color::Rgb(108, 122, 137);     // Muted text
    // Background colors
    pub const BG_DARK: Color = Color::Rgb(44, 62, 80);           // Dark background
    pub const BG_LIGHT: Color = Color::Rgb(236, 240, 241);       // Light background
    pub const BG_ACCENT: Color = Color::Rgb(52, 73, 94);         // Accent background
}

// Helper functions for semantic colors
impl PrimeColors {
    pub fn performance_color(success_rate: f64) -> Color {
        match success_rate {
            r if r >= 0.30 => Self::EXCELLENT,
            r if r >= 0.20 => Self::GOOD,
            r if r >= 0.10 => Self::FAIR,
            _ => Self::POOR,
        }
    }
    
    pub fn coprimality_color(is_coprime: bool) -> Color {
        if is_coprime {
            Self::SUCCESS_GREEN
        } else {
            Self::ERROR_RED
        }
    }
    
    pub fn prime_result_color(is_prime: bool) -> Color {
        if is_prime {
            Self::PRIME_GREEN
        } else {
            Self::COMPOSITE_RED
        }
    }
    
    pub fn quality_indicator_color(quality: &str) -> Color {
        match quality {
            "✨ PERFECT" => Self::EXCELLENT,
            "⚡ EXCELLENT" => Self::GOOD,
            "📊 DECENT" => Self::FAIR,
            "🤔 POOR" => Self::WARNING_YELLOW,
            "💥 BROKEN" => Self::ERROR_RED,
            _ => Self::TEXT_PRIMARY,
        }
    }
}

fn main() {
    println!("Common Colors - Unified color scheme for all visualization tools");
    println!("This module provides consistent colors for:");
    println!("  - Prime/Composite results");
    println!("  - Performance indicators");
    println!("  - Coprimality checks");
    println!("  - Quality assessment");
    println!("  - Heat map visualization");
    println!("\nUse this module in your TUI applications for consistent branding.");
}
