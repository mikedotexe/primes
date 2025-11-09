//! Enhanced Lagrange Point TUI - Educational visualization of prime interactions
//!
//! This version shows:
//! - Larger primes with different zero-padding densities
//! - Clear explanation of where Lagrange points are
//! - Visual representation of gravitational equilibrium
//! - Animation of primes "coming together"

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use num_bigint::BigUint;
use prime_physics_engine::{
    gravity::{PhysicsCache, PrimeParticle},
    is_prime,
    membrane::{MembraneBuilder, MembraneConfig},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant, SystemTime},
};

#[derive(Debug)]
struct EducationalState {
    // Two larger primes with different padding
    prime1: Option<PrimeParticle>, // Smaller padding
    prime2: Option<PrimeParticle>, // Larger padding

    // Configurations
    config1: MembraneConfig,
    config2: MembraneConfig,

    // Lagrange analysis
    lagrange_points: Vec<LagrangePointInfo>,

    // Animation state
    animation_frame: usize,
    is_animating: bool,
    prime_separation: f64, // Visual separation for animation

    // Educational content
    current_explanation: usize,
    explanations: Vec<String>,

    // UI state
    show_help: bool,
    status_message: String,
    #[allow(dead_code)]
    last_update: Instant,
}

#[derive(Debug, Clone)]
struct LagrangePointInfo {
    point_type: String,
    #[allow(dead_code)]
    position: f64, // Position between primes (0.0 = prime1, 1.0 = prime2)
    value: BigUint,
    is_prime: bool,
    explanation: String,
    stability: f64,
    dominance: String, // Which prime dominates this region
}

impl Default for EducationalState {
    fn default() -> Self {
        let explanations = vec![
            "Lagrange points are positions where gravitational forces balance.".to_string(),
            "L1 between membrane primes preserves structure, averaging only the middle digit."
                .to_string(),
            "For 303050303 and 303070303, L1 = 303060303 (middle: 6 = avg of 5,7).".to_string(),
            "The membrane structure acts like a 'container' - only the center varies.".to_string(),
            "This suggests the 'mass' is in the boundary digits, not the middle seed.".to_string(),
            "In prime space, Lagrange points reveal the structural symmetries.".to_string(),
        ];

        Self {
            prime1: None,
            prime2: None,
            config1: MembraneConfig::new(10, 3, 7, 2, 1), // Moderate padding
            config2: MembraneConfig::new(10, 3, 7, 4, 2), // Heavy padding
            lagrange_points: Vec::new(),
            animation_frame: 0,
            is_animating: false,
            prime_separation: 100.0,
            current_explanation: 0,
            explanations,
            show_help: false,
            status_message: "Press 'g' to generate large membrane primes".to_string(),
            last_update: Instant::now(),
        }
    }
}

impl EducationalState {
    fn generate_large_primes(&mut self) {
        // Generate a prime with moderate padding
        self.config1 = MembraneConfig::new(10, 3, 7, 2, 1);
        // Try seed 17 for a larger middle section
        if let Ok(mut particle) = MembraneBuilder::new(self.config1.clone())
            .with_seed(17)
            .build()
        {
            particle.position = [-50.0, 0.0];
            self.prime1 = Some(particle);
        }

        // Generate a prime with heavy padding - different config
        self.config2 = MembraneConfig::new(10, 3, 7, 4, 2);
        if let Ok(mut particle) = MembraneBuilder::new(self.config2.clone())
            .with_seed(23)
            .build()
        {
            particle.position = [50.0, 0.0];
            self.prime2 = Some(particle);
        }

        // If we couldn't generate, use pre-calculated large primes
        if self.prime1.is_none() {
            self.prime1 = Some(create_large_prime(
                BigUint::from(3007001700703u64), // 3-00-7-0-17-0-7-00-3
                self.config1.clone(),
                [-50.0, 0.0],
            ));
        }

        if self.prime2.is_none() {
            self.prime2 = Some(create_large_prime(
                BigUint::from(300007002300700003u64), // 3-0000-7-00-23-00-7-0000-3
                self.config2.clone(),
                [50.0, 0.0],
            ));
        }

        self.calculate_detailed_lagrange_points();
        self.status_message =
            "Large primes generated! Press 'a' to animate, 'e' to cycle explanations".to_string();
    }

    fn calculate_detailed_lagrange_points(&mut self) {
        self.lagrange_points.clear();

        if let (Some(p1), Some(p2)) = (&self.prime1, &self.prime2) {
            let val1 = &p1.value;
            let val2 = &p2.value;

            // L1 - Between the primes (closer to smaller prime)
            let mass_ratio = p1.mass / (p1.mass + p2.mass);
            let l1_position = 0.5 - (0.2 * mass_ratio); // Slightly offset based on mass
            let l1_value = interpolate_value(val1, val2, l1_position);

            self.lagrange_points.push(LagrangePointInfo {
                point_type: "L1".to_string(),
                position: l1_position,
                value: l1_value.clone(),
                is_prime: is_prime(&l1_value),
                explanation: "L1: Between primes - unstable equilibrium. Small perturbations grow."
                    .to_string(),
                stability: 0.3,
                dominance: "Balanced".to_string(),
            });

            // L2 - Beyond the smaller prime
            let l2_position = -0.2;
            let l2_value = extrapolate_value(val1, val2, l2_position);

            self.lagrange_points.push(LagrangePointInfo {
                point_type: "L2".to_string(),
                position: l2_position,
                value: l2_value.clone(),
                is_prime: is_prime(&l2_value),
                explanation: "L2: Beyond smaller prime - shielded from larger prime's influence."
                    .to_string(),
                stability: 0.2,
                dominance: "Prime 1".to_string(),
            });

            // L3 - Beyond the larger prime
            let l3_position = 1.2;
            let l3_value = extrapolate_value(val1, val2, l3_position);

            self.lagrange_points.push(LagrangePointInfo {
                point_type: "L3".to_string(),
                position: l3_position,
                value: l3_value.clone(),
                is_prime: is_prime(&l3_value),
                explanation: "L3: Far side of larger prime - weakest influence zone.".to_string(),
                stability: 0.1,
                dominance: "Prime 2".to_string(),
            });

            // L4 - Leading trojan point (60° ahead)
            let l4_position = 0.5;
            let _l4_offset = 0.866; // sin(60°) for vertical offset
            let l4_value = interpolate_value(val1, val2, 0.4);

            self.lagrange_points.push(LagrangePointInfo {
                point_type: "L4".to_string(),
                position: l4_position,
                value: l4_value.clone(),
                is_prime: is_prime(&l4_value),
                explanation: "L4: Leading trojan - stable! Forms equilateral triangle with primes."
                    .to_string(),
                stability: 0.8,
                dominance: "Shared".to_string(),
            });

            // L5 - Trailing trojan point (60° behind)
            let l5_position = 0.5;
            let l5_value = interpolate_value(val1, val2, 0.6);

            self.lagrange_points.push(LagrangePointInfo {
                point_type: "L5".to_string(),
                position: l5_position,
                value: l5_value.clone(),
                is_prime: is_prime(&l5_value),
                explanation: "L5: Trailing trojan - equally stable. Natural parking orbit."
                    .to_string(),
                stability: 0.8,
                dominance: "Shared".to_string(),
            });
        }
    }

    fn animate_step(&mut self) {
        if self.is_animating {
            self.animation_frame = (self.animation_frame + 1) % 100;

            // Oscillate the separation to show gravitational interaction
            let t = self.animation_frame as f64 / 100.0;
            self.prime_separation = 80.0 + 20.0 * (t * std::f64::consts::TAU).sin();

            // Update positions based on separation
            if let Some(p1) = &mut self.prime1 {
                p1.position[0] = -self.prime_separation / 2.0;
            }
            if let Some(p2) = &mut self.prime2 {
                p2.position[0] = self.prime_separation / 2.0;
            }

            // Recalculate Lagrange points with new positions
            self.calculate_detailed_lagrange_points();
        }
    }

    fn cycle_explanation(&mut self) {
        self.current_explanation = (self.current_explanation + 1) % self.explanations.len();
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = EducationalState::default();
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);

    loop {
        terminal.draw(|f| draw_educational_ui(f, &state))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('g') => {
                        state.generate_large_primes();
                    }
                    KeyCode::Char('a') => {
                        state.is_animating = !state.is_animating;
                        state.status_message = if state.is_animating {
                            "Animation started - watch the primes interact!".to_string()
                        } else {
                            "Animation paused".to_string()
                        };
                    }
                    KeyCode::Char('e') => {
                        state.cycle_explanation();
                    }
                    KeyCode::Char('h') | KeyCode::Char('?') => {
                        state.show_help = !state.show_help;
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            state.animate_step();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn draw_educational_ui(f: &mut Frame, state: &EducationalState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(15), // Prime visualization
            Constraint::Length(10), // Lagrange diagram
            Constraint::Min(5),     // Explanations
            Constraint::Length(3),  // Status
        ])
        .split(f.size());

    // Header
    let header = Paragraph::new(Line::from(vec![
        Span::styled("🎓 ", Style::default().fg(Color::Yellow)),
        Span::styled(
            "Lagrange Points in Prime Space",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            " - Understanding Gravitational Equilibrium",
            Style::default().fg(Color::Gray),
        ),
    ]))
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);

    // Prime visualization
    draw_prime_visualization(f, state, chunks[1]);

    // Lagrange point diagram
    draw_lagrange_diagram(f, state, chunks[2]);

    // Explanations
    draw_explanations(f, state, chunks[3]);

    // Status bar
    draw_status_bar(f, state, chunks[4]);
}

fn draw_prime_visualization(f: &mut Frame, state: &EducationalState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Title
            Constraint::Length(7), // Prime 1
            Constraint::Length(7), // Prime 2
        ])
        .split(area);

    // Title
    let title = Paragraph::new("Membrane Primes with Different Zero-Padding Densities")
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    // Prime 1 - Moderate padding
    if let Some(p1) = &state.prime1 {
        let visual = format_large_prime_visual(&p1.value, &state.config1);
        let value_str = p1.value.to_string();
        let config_str = format!(
            "({},{}) k=({},{}) base {}",
            state.config1.outer,
            state.config1.inner,
            state.config1.k_outer,
            state.config1.k_inner,
            state.config1.base
        );
        let structure_str = format_membrane_breakdown(&p1.value, &state.config1);

        let info = vec![
            Line::from(vec![
                Span::raw("Prime 1 (Moderate Padding): "),
                Span::styled(value_str, Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw("Config: "),
                Span::styled(config_str, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![Span::raw(visual)]),
            Line::from(vec![
                Span::raw("Structure: "),
                Span::styled(structure_str, Style::default().fg(Color::Magenta)),
            ]),
        ];

        let prime1_widget = Paragraph::new(info)
            .block(Block::default().borders(Borders::ALL).title("⚛️ Atom 1"))
            .wrap(ratatui::widgets::Wrap { trim: false });
        f.render_widget(prime1_widget, chunks[1]);
    }

    // Prime 2 - Heavy padding
    if let Some(p2) = &state.prime2 {
        let visual = format_large_prime_visual(&p2.value, &state.config2);
        let value_str = p2.value.to_string();
        let config_str = format!(
            "({},{}) k=({},{}) base {}",
            state.config2.outer,
            state.config2.inner,
            state.config2.k_outer,
            state.config2.k_inner,
            state.config2.base
        );
        let structure_str = format_membrane_breakdown(&p2.value, &state.config2);

        let info = vec![
            Line::from(vec![
                Span::raw("Prime 2 (Heavy Padding): "),
                Span::styled(value_str, Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw("Config: "),
                Span::styled(config_str, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![Span::raw(visual)]),
            Line::from(vec![
                Span::raw("Structure: "),
                Span::styled(structure_str, Style::default().fg(Color::Magenta)),
            ]),
        ];

        let prime2_widget = Paragraph::new(info)
            .block(Block::default().borders(Borders::ALL).title("⚛️ Atom 2"))
            .wrap(ratatui::widgets::Wrap { trim: false });
        f.render_widget(prime2_widget, chunks[2]);
    }
}

fn draw_lagrange_diagram(f: &mut Frame, state: &EducationalState, area: Rect) {
    let mut lines = vec![];

    // ASCII art diagram showing Lagrange points
    lines.push(Line::from("Lagrange Points in Prime Space:"));
    lines.push(Line::from(""));

    if state.prime1.is_some() && state.prime2.is_some() {
        // Visual representation
        let width = area.width as usize - 4;
        let mut diagram = vec![' '; width];

        // Place primes
        let p1_pos = width / 4;
        let p2_pos = 3 * width / 4;
        diagram[p1_pos] = '●';
        diagram[p2_pos] = '●';

        // Place Lagrange points
        for lpoint in &state.lagrange_points {
            let pos = match lpoint.point_type.as_str() {
                "L1" => (p1_pos + p2_pos) / 2,
                "L2" => p1_pos / 2,
                "L3" => p2_pos + (width - p2_pos) / 2,
                "L4" => (p1_pos + p2_pos) / 2,
                "L5" => (p1_pos + p2_pos) / 2,
                _ => width / 2,
            };

            if pos < width {
                diagram[pos] = match lpoint.point_type.as_str() {
                    "L1" => '①',
                    "L2" => '②',
                    "L3" => '③',
                    "L4" => '④',
                    "L5" => '⑤',
                    _ => '?',
                };
            }
        }

        // Draw the diagram
        let _diagram_str: String = diagram.iter().collect();
        lines.push(Line::from(vec![Span::raw(
            "   L2        P1              L1              P2        L3",
        )]));
        lines.push(Line::from(vec![
            Span::raw("   ②         "),
            Span::styled(
                "●",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("              ①              "),
            Span::styled(
                "●",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("         ③"),
        ]));
        lines.push(Line::from(vec![Span::raw(
            "   ├─────────┼───────────────┼───────────────┼─────────┤",
        )]));

        // Show L4 and L5 above and below
        lines.push(Line::from(vec![Span::raw(
            "                      L4 ④ (above)",
        )]));
        lines.push(Line::from(vec![Span::raw(
            "                      L5 ⑤ (below)",
        )]));

        // Show current values
        lines.push(Line::from(""));
        for lpoint in &state.lagrange_points {
            let value_str = lpoint.value.to_string();
            let mut spans = vec![
                Span::styled(lpoint.point_type.clone(), Style::default().fg(Color::Cyan)),
                Span::raw(": "),
                Span::raw(value_str),
            ];

            if lpoint.is_prime {
                spans.push(Span::styled(
                    " ✓ PRIME!",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ));
            }

            lines.push(Line::from(spans));
        }
    }

    let diagram_widget = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("🌌 Gravitational Map"),
    );
    f.render_widget(diagram_widget, area);
}

fn draw_explanations(f: &mut Frame, state: &EducationalState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Current explanation
    let explanation_text = if state.current_explanation < state.explanations.len() {
        &state.explanations[state.current_explanation]
    } else {
        "Press 'e' to cycle through explanations"
    };

    let explanation = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("💡 ", Style::default().fg(Color::Yellow)),
            Span::styled(
                "Understanding Lagrange Points",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(explanation_text),
        Line::from(""),
        Line::from(vec![
            Span::raw("Current: "),
            Span::styled(
                format!(
                    "{}/{}",
                    state.current_explanation + 1,
                    state.explanations.len()
                ),
                Style::default().fg(Color::Gray),
            ),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL))
    .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(explanation, chunks[0]);

    // Lagrange point details
    if let Some(lpoint) = state.lagrange_points.get(0) {
        let stability_bar = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Stability"))
            .gauge_style(Style::default().fg(Color::Green))
            .percent((lpoint.stability * 100.0) as u16);

        let details = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Selected: ", Style::default().fg(Color::Gray)),
                Span::styled(&lpoint.point_type, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::raw("Dominance: "),
                Span::styled(&lpoint.dominance, Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""),
            Line::from(lpoint.explanation.as_str()),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Point Details"),
        );

        let detail_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(5), Constraint::Length(3)])
            .split(chunks[1]);

        f.render_widget(details, detail_chunks[0]);
        f.render_widget(stability_bar, detail_chunks[1]);
    }
}

fn draw_status_bar(f: &mut Frame, state: &EducationalState, area: Rect) {
    let status = Paragraph::new(Line::from(vec![
        Span::raw("Commands: "),
        Span::styled(
            "g",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(":generate "),
        Span::styled(
            "a",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(":animate "),
        Span::styled(
            "e",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(":explain "),
        Span::styled(
            "h",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(":help "),
        Span::styled(
            "q",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(":quit | "),
        Span::styled(&state.status_message, Style::default().fg(Color::Yellow)),
    ]))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, area);
}

// Helper functions

fn create_large_prime(value: BigUint, config: MembraneConfig, position: [f64; 2]) -> PrimeParticle {
    PrimeParticle {
        value: value.clone(),
        base: config.base,
        position,
        velocity: [0.0, 0.0],
        mass: (value.to_string().len() as f64).ln() * 10.0,
        charge: 1.0,
        name: format!("Prime {}", value),
        membrane_config: Some(config),
        creation_time: SystemTime::now(),
        trajectory_history: Vec::new(),
        physics_cache: PhysicsCache::default(),
    }
}

fn format_large_prime_visual(value: &BigUint, _config: &MembraneConfig) -> String {
    let val_str = value.to_string();
    let mut result = String::new();

    // Try to parse the structure based on the config
    // This is a simplified visualization
    let chars: Vec<char> = val_str.chars().collect();

    for (i, &ch) in chars.iter().enumerate() {
        if ch == '0' {
            result.push('◯');
        } else {
            result.push(ch);
        }

        if i < chars.len() - 1 {
            result.push('─');
        }
    }

    result
}

fn format_membrane_breakdown(_value: &BigUint, config: &MembraneConfig) -> String {
    // Show the conceptual breakdown
    format!(
        "{}-{}×◯-{}-{}×◯-[seed]-{}×◯-{}-{}×◯-{}",
        config.outer,
        config.k_outer,
        config.inner,
        config.k_inner,
        config.k_inner,
        config.inner,
        config.k_outer,
        config.outer
    )
}

fn interpolate_value(v1: &BigUint, v2: &BigUint, t: f64) -> BigUint {
    // For membrane primes, we should do structural interpolation
    // This preserves the membrane structure and only changes the middle
    let s1 = v1.to_string();
    let s2 = v2.to_string();

    // For now, do simple numeric interpolation
    // TODO: Implement proper membrane-aware interpolation
    let diff = if v2 > v1 { v2 - v1 } else { v1 - v2 };
    let offset = (diff.to_string().parse::<f64>().unwrap_or(0.0) * t) as u64;

    // Special case: if t = 0.5 and both have same structure except middle
    // then the result preserves the membrane structure
    if (t - 0.5).abs() < 0.01 && s1.len() == s2.len() {
        // Check if they differ in only one position
        let diffs: Vec<_> = s1
            .chars()
            .zip(s2.chars())
            .enumerate()
            .filter(|(_, (c1, c2))| c1 != c2)
            .map(|(i, _)| i)
            .collect();

        if diffs.len() == 1 {
            // They differ in exactly one position - likely the middle
            let pos = diffs[0];
            if let (Some(d1), Some(d2)) = (
                s1.chars().nth(pos).unwrap().to_digit(10),
                s2.chars().nth(pos).unwrap().to_digit(10),
            ) {
                let avg = (d1 + d2) / 2;
                let mut result = s1.clone();
                result.replace_range(pos..pos + 1, &avg.to_string());
                return result.parse().unwrap_or(v1 + BigUint::from(offset));
            }
        }
    }

    v1 + BigUint::from(offset)
}

fn extrapolate_value(v1: &BigUint, v2: &BigUint, t: f64) -> BigUint {
    // Extrapolation beyond the range
    if t < 0.0 {
        // Before v1
        let diff = if v2 > v1 { v2 - v1 } else { v1 - v2 };
        let offset = (diff.to_string().parse::<f64>().unwrap_or(0.0) * t.abs()) as u64;
        if offset > v1.to_string().parse::<u64>().unwrap_or(0) {
            BigUint::from(1u32)
        } else {
            v1 - BigUint::from(offset)
        }
    } else {
        // After v2
        let diff = if v2 > v1 { v2 - v1 } else { v1 - v2 };
        let offset = (diff.to_string().parse::<f64>().unwrap_or(0.0) * (t - 1.0)) as u64;
        v2 + BigUint::from(offset)
    }
}
