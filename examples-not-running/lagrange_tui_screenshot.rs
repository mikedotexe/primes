//! TUI Screenshot Generator - Renders TUI to HTML/SVG for viewing
//! 
//! This allows us to capture the visual state of the TUI without needing a terminal

use ratatui::{
    backend::TestBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;

// We'll need to duplicate some of the core structures here
use primes::{
    membrane::{MembraneConfig, MembraneBuilder},
    gravity::{PrimeParticle, GravitationalField, PhysicsCache},
    lagrange::{ClusterAnalysis, LagrangePoint},
    is_prime,
};
use num_bigint::BigUint;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
struct LagrangeState {
    particle1: Option<PrimeParticle>,
    particle2: Option<PrimeParticle>,
    config: MembraneConfig,
    lagrange_points: Vec<LagrangePoint>,
    cluster_analysis: Option<ClusterAnalysis>,
    selected_prime: usize,
    show_help: bool,
    is_generating: bool,
    status_message: String,
    total_generations: usize,
    primes_found: usize,
    last_generation_time: Option<Duration>,
    current_prime_distance: Option<BigUint>,
}

impl Default for LagrangeState {
    fn default() -> Self {
        let config = MembraneConfig::new(6, 1, 5, 0, 0);
        Self {
            particle1: None,
            particle2: None,
            config,
            lagrange_points: Vec::new(),
            cluster_analysis: None,
            selected_prime: 0,
            show_help: false,
            is_generating: false,
            status_message: String::from("Press 'g' to generate prime pair"),
            total_generations: 0,
            primes_found: 0,
            last_generation_time: None,
            current_prime_distance: None,
        }
    }
}

fn main() -> Result<(), io::Error> {
    // Create a test backend with a specific size
    let backend = TestBackend::new(150, 40);
    let mut terminal = Terminal::new(backend)?;
    
    // Create state and generate some primes
    let mut state = LagrangeState::default();
    lagrange_tui::generate_prime_pair(&mut state);
    
    // Draw the UI to our test backend
    terminal.draw(|f| draw_ui(f, &state))?;
    
    // Get the buffer contents
    let buffer = terminal.backend().buffer().clone();
    
    // Convert to HTML
    let html = buffer_to_html(&buffer);
    
    // Write to file
    std::fs::write("lagrange_tui_screenshot.html", html)?;
    println!("Screenshot saved to lagrange_tui_screenshot.html");
    
    // Also save as text for easier reading
    let text = buffer_to_text(&buffer);
    std::fs::write("lagrange_tui_screenshot.txt", text)?;
    println!("Text version saved to lagrange_tui_screenshot.txt");
    
    Ok(())
}

fn buffer_to_html(buffer: &ratatui::buffer::Buffer) -> String {
    let mut html = String::from(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Lagrange TUI Screenshot</title>
    <style>
        body {
            background: #1e1e1e;
            color: #ffffff;
            font-family: 'Consolas', 'Monaco', monospace;
            margin: 20px;
        }
        pre {
            line-height: 1.2;
            letter-spacing: 0.05em;
        }
        .fg-black { color: #000000; }
        .fg-red { color: #ff5555; }
        .fg-green { color: #50fa7b; }
        .fg-yellow { color: #f1fa8c; }
        .fg-blue { color: #6272a4; }
        .fg-magenta { color: #ff79c6; }
        .fg-cyan { color: #8be9fd; }
        .fg-gray { color: #6c7086; }
        .fg-darkgray { color: #44475a; }
        .fg-white { color: #ffffff; }
        .bold { font-weight: bold; }
        .dim { opacity: 0.7; }
        .italic { font-style: italic; }
        .underline { text-decoration: underline; }
    </style>
</head>
<body>
<pre>"#);
    
    let (width, height) = buffer.area.as_size();
    
    for y in 0..height {
        for x in 0..width {
            let cell = buffer.get(x, y);
            let style = cell.style();
            
            let mut classes = Vec::new();
            
            // Foreground color
            match style.fg {
                Some(Color::Black) => classes.push("fg-black"),
                Some(Color::Red) => classes.push("fg-red"),
                Some(Color::Green) => classes.push("fg-green"),
                Some(Color::Yellow) => classes.push("fg-yellow"),
                Some(Color::Blue) => classes.push("fg-blue"),
                Some(Color::Magenta) => classes.push("fg-magenta"),
                Some(Color::Cyan) => classes.push("fg-cyan"),
                Some(Color::Gray) => classes.push("fg-gray"),
                Some(Color::DarkGray) => classes.push("fg-darkgray"),
                Some(Color::White) | None => classes.push("fg-white"),
                _ => {}
            }
            
            // Modifiers
            if style.add_modifier.contains(Modifier::BOLD) {
                classes.push("bold");
            }
            if style.add_modifier.contains(Modifier::DIM) {
                classes.push("dim");
            }
            if style.add_modifier.contains(Modifier::ITALIC) {
                classes.push("italic");
            }
            if style.add_modifier.contains(Modifier::UNDERLINED) {
                classes.push("underline");
            }
            
            let symbol = cell.symbol();
            let escaped = html_escape::encode_text(symbol);
            
            if classes.is_empty() {
                html.push_str(&escaped);
            } else {
                html.push_str(&format!(r#"<span class="{}">{}</span>"#, classes.join(" "), escaped));
            }
        }
        html.push('\n');
    }
    
    html.push_str("</pre></body></html>");
    html
}

fn buffer_to_text(buffer: &ratatui::buffer::Buffer) -> String {
    let mut text = String::new();
    let (width, height) = buffer.area.as_size();
    
    for y in 0..height {
        for x in 0..width {
            let cell = buffer.get(x, y);
            text.push_str(cell.symbol());
        }
        text.push('\n');
    }
    
    text
}