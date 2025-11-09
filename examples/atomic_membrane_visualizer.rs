//! Atomic Membrane Visualizer
//!
//! The ultimate visualization of membrane primes as atomic structures
//! with beautiful ASCII art showing orbitals, bonds, and field effects

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use num_bigint::BigUint;
use prime_physics_engine::is_prime;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::str::FromStr;

#[allow(dead_code)]
const ORBITAL_CHARS: &[char] = &['◯', '○', '◌', '◔', '◕', '●'];
const BOND_CHARS: &[char] = &['·', '∘', '∙', '•', '․', '‥', '…'];
const FIELD_CHARS: &[char] = &['˙', '٭', '✦', '✧', '★', '✯', '✰'];

struct AtomicMembrane {
    value: String,
    digits: Vec<char>,
    is_prime: bool,
    phase: f64,
}

impl AtomicMembrane {
    fn new(membrane: &str) -> Self {
        let digits: Vec<char> = membrane.chars().collect();
        let is_prime = BigUint::from_str(membrane)
            .map(|n| is_prime(&n))
            .unwrap_or(false);

        Self {
            value: membrane.to_string(),
            digits,
            is_prime,
            phase: 0.0,
        }
    }

    fn update(&mut self, dt: f64) {
        self.phase = (self.phase + dt) % (2.0 * std::f64::consts::PI);
    }
}

struct AtomicVisualizer {
    atoms: Vec<AtomicMembrane>,
    current: usize,
    show_legend: bool,
    animation_speed: f64,
    view_mode: ViewMode,
}

#[derive(Clone, Copy)]
enum ViewMode {
    TopDown,   // Looking down at the atom
    SideView,  // Traditional side view
    Isometric, // 3D-like isometric view
}

impl AtomicVisualizer {
    fn new() -> Self {
        let atoms = vec![
            AtomicMembrane::new("10301"),     // Zero-padded beauty
            AtomicMembrane::new("151"),       // Minimal
            AtomicMembrane::new("30703"),     // Simple symmetric
            AtomicMembrane::new("303050303"), // Complex
            AtomicMembrane::new("3305033"),   // Breathing
            AtomicMembrane::new("100030001"), // Triple zero
        ];

        Self {
            atoms,
            current: 0,
            show_legend: true,
            animation_speed: 0.05,
            view_mode: ViewMode::TopDown,
        }
    }

    fn update(&mut self) {
        self.atoms[self.current].update(self.animation_speed);
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = AtomicVisualizer::new();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        app.update();

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Left => {
                        app.current = if app.current == 0 {
                            app.atoms.len() - 1
                        } else {
                            app.current - 1
                        };
                    }
                    KeyCode::Right => {
                        app.current = (app.current + 1) % app.atoms.len();
                    }
                    KeyCode::Char('l') => app.show_legend = !app.show_legend,
                    KeyCode::Char('v') => {
                        app.view_mode = match app.view_mode {
                            ViewMode::TopDown => ViewMode::SideView,
                            ViewMode::SideView => ViewMode::Isometric,
                            ViewMode::Isometric => ViewMode::TopDown,
                        };
                    }
                    KeyCode::Char(' ') => {
                        app.animation_speed = if app.animation_speed > 0.0 { 0.0 } else { 0.05 };
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &mut AtomicVisualizer) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(20),   // Main display
            Constraint::Length(4), // Info
            Constraint::Length(3), // Controls
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new("⚛️  ATOMIC MEMBRANE VISUALIZATION ⚛️")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main atomic display
    render_atomic_display(f, chunks[1], &app.atoms[app.current], app);

    // Info panel
    let atom = &app.atoms[app.current];
    let info_lines = vec![
        Line::from(vec![
            Span::raw("Membrane: "),
            Span::styled(
                &atom.value,
                Style::default()
                    .fg(if atom.is_prime {
                        Color::Green
                    } else {
                        Color::Red
                    })
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(if atom.is_prime {
                " ✓ PRIME ATOM"
            } else {
                " ✗ COMPOSITE ATOM"
            }),
        ]),
        Line::from(format!(
            "Structure: {} | Electrons: {}",
            visualize_membrane(&atom.digits),
            atom.digits.len()
        )),
    ];
    let info = Paragraph::new(info_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Atomic Properties"),
    );
    f.render_widget(info, chunks[2]);

    // Controls
    let controls = vec![Line::from(
        "←/→: Switch atom  v: View mode  SPACE: Pause  l: Legend  q: Quit",
    )];
    let controls_widget = Paragraph::new(controls)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(controls_widget, chunks[3]);
}

fn render_atomic_display(f: &mut Frame, area: Rect, atom: &AtomicMembrane, app: &AtomicVisualizer) {
    let view_name = match app.view_mode {
        ViewMode::TopDown => "Top-Down View",
        ViewMode::SideView => "Side View",
        ViewMode::Isometric => "Isometric View",
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!("Atomic Structure - {}", view_name));

    let inner = block.inner(area);
    f.render_widget(block, area);

    match app.view_mode {
        ViewMode::TopDown => render_top_down_atom(f, inner, atom, app),
        ViewMode::SideView => render_side_view_atom(f, inner, atom, app),
        ViewMode::Isometric => render_isometric_atom(f, inner, atom, app),
    }

    // Render legend if enabled
    if app.show_legend {
        render_legend(f, inner);
    }
}

fn render_top_down_atom(f: &mut Frame, area: Rect, atom: &AtomicMembrane, _app: &AtomicVisualizer) {
    let cx = area.width / 2;
    let cy = area.height / 2;

    // Create layers based on digit positions
    let mut layers = vec![];
    let mut current_layer = vec![];

    for (i, &digit) in atom.digits.iter().enumerate() {
        current_layer.push(digit);
        if current_layer.len() >= 4 || i == atom.digits.len() - 1 {
            layers.push(current_layer.clone());
            current_layer.clear();
        }
    }

    // Render nucleus (center)
    render_nucleus(f, area, cx, cy, atom.is_prime);

    // Render electron shells
    for (layer_idx, layer) in layers.iter().enumerate() {
        let radius = (layer_idx + 1) * 3;
        render_electron_shell(
            f,
            area,
            cx,
            cy,
            radius as u16,
            layer,
            atom.phase,
            atom.is_prime,
        );
    }

    // Render field effect
    if atom.is_prime {
        let outer_radius = (layers.len() + 1) * 3;
        render_prime_field(f, area, cx, cy, outer_radius as u16, atom.phase);
    }
}

fn render_side_view_atom(
    f: &mut Frame,
    area: Rect,
    atom: &AtomicMembrane,
    _app: &AtomicVisualizer,
) {
    let cx = area.width / 2;
    let _cy = area.height / 2;

    // Vertical arrangement with orbital paths
    let spacing = area.height / (atom.digits.len() + 2) as u16;

    for (i, &digit) in atom.digits.iter().enumerate() {
        let y = area.y + spacing * (i + 1) as u16;

        // Draw orbital path
        for dx in -(cx as i16 / 2)..=(cx as i16 / 2) {
            let x = (cx as i16 + dx) as u16;
            if x < area.x + area.width {
                let orbital_char = if dx.abs() < 3 { '─' } else { '·' };
                render_char(f, x, y, orbital_char, Style::default().fg(Color::DarkGray));
            }
        }

        // Draw electron
        let offset = ((atom.phase + i as f64) * 2.0).sin() * (cx as f64 / 3.0);
        let electron_x = (cx as f64 + offset) as u16;

        let (ch, color) = if digit == '0' {
            ('◯', Color::Blue)
        } else {
            (digit, Color::Yellow)
        };

        render_char(
            f,
            electron_x,
            y,
            ch,
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        );

        // Draw bond to center
        if i > 0 {
            let prev_y = area.y + spacing * i as u16;
            draw_vertical_bond(f, cx, prev_y, y, atom.is_prime);
        }
    }
}

fn render_isometric_atom(
    f: &mut Frame,
    area: Rect,
    atom: &AtomicMembrane,
    _app: &AtomicVisualizer,
) {
    let cx = area.width / 2;
    let cy = area.height / 2;

    // Create 3D-like effect with offset layers
    let _layer_offset = 2;
    let depth_offset = 1;

    // Group digits into shells
    let shells = group_into_shells(&atom.digits);

    for (shell_idx, shell) in shells.iter().enumerate() {
        let z_offset = shell_idx as u16 * depth_offset;
        let radius = (shell_idx + 1) as u16 * 4;

        // Draw back half of shell
        for (i, &digit) in shell.iter().enumerate() {
            let angle = (i as f64 / shell.len() as f64) * 2.0 * std::f64::consts::PI + atom.phase;

            if angle.sin() < 0.0 {
                // Back half
                let x = cx + (radius as f64 * angle.cos()) as u16 + z_offset;
                let y = cy + (radius as f64 * angle.sin() / 2.0) as u16 - z_offset;

                let (ch, color) = electron_appearance(digit, false);
                render_char(f, x, y, ch, Style::default().fg(color));
            }
        }

        // Draw orbital ring
        draw_isometric_ring(f, area, cx + z_offset, cy - z_offset, radius);

        // Draw front half of shell
        for (i, &digit) in shell.iter().enumerate() {
            let angle = (i as f64 / shell.len() as f64) * 2.0 * std::f64::consts::PI + atom.phase;

            if angle.sin() >= 0.0 {
                // Front half
                let x = cx + (radius as f64 * angle.cos()) as u16 + z_offset;
                let y = cy + (radius as f64 * angle.sin() / 2.0) as u16 - z_offset;

                let (ch, color) = electron_appearance(digit, true);
                render_char(
                    f,
                    x,
                    y,
                    ch,
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                );
            }
        }
    }

    // Draw nucleus with 3D offset
    let nucleus_offset = shells.len() as u16 * depth_offset;
    render_3d_nucleus(
        f,
        area,
        cx + nucleus_offset,
        cy - nucleus_offset,
        atom.is_prime,
    );
}

// Helper functions

fn render_nucleus(f: &mut Frame, area: Rect, cx: u16, cy: u16, is_prime: bool) {
    let nucleus_chars = if is_prime {
        vec![
            (0, 0, '★'),
            (-1, 0, '✦'),
            (1, 0, '✦'),
            (0, -1, '✦'),
            (0, 1, '✦'),
        ]
    } else {
        vec![(0, 0, '◉'), (-1, 0, '●'), (1, 0, '●')]
    };

    let color = if is_prime { Color::Green } else { Color::Red };

    for (dx, dy, ch) in nucleus_chars {
        let x = (cx as i16 + dx) as u16;
        let y = (cy as i16 + dy) as u16;
        if x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height {
            render_char(
                f,
                x,
                y,
                ch,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            );
        }
    }
}

fn render_electron_shell(
    f: &mut Frame,
    area: Rect,
    cx: u16,
    cy: u16,
    radius: u16,
    electrons: &[char],
    phase: f64,
    is_prime: bool,
) {
    // Draw orbital path
    for angle_deg in (0..360).step_by(10) {
        let angle = (angle_deg as f64) * std::f64::consts::PI / 180.0;
        let x = cx as f64 + radius as f64 * angle.cos();
        let y = cy as f64 + radius as f64 * angle.sin() / 2.0; // Terminal aspect ratio

        if let (Some(xi), Some(yi)) = (
            to_area_coord(x, area.x, area.width),
            to_area_coord(y, area.y, area.height),
        ) {
            render_char(f, xi, yi, '·', Style::default().fg(Color::DarkGray));
        }
    }

    // Place electrons
    for (i, &electron) in electrons.iter().enumerate() {
        let angle = (i as f64 / electrons.len() as f64) * 2.0 * std::f64::consts::PI + phase;
        let x = cx as f64 + radius as f64 * angle.cos();
        let y = cy as f64 + radius as f64 * angle.sin() / 2.0;

        if let (Some(xi), Some(yi)) = (
            to_area_coord(x, area.x, area.width),
            to_area_coord(y, area.y, area.height),
        ) {
            let (ch, color) = if electron == '0' {
                ('◯', Color::Blue)
            } else {
                (electron, Color::Yellow)
            };

            render_char(
                f,
                xi,
                yi,
                ch,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            );

            // Draw bond to nucleus
            draw_radial_bond(f, area, cx, cy, xi, yi, is_prime);
        }
    }
}

fn render_prime_field(f: &mut Frame, area: Rect, cx: u16, cy: u16, radius: u16, phase: f64) {
    let field_radius = radius + 2;

    for i in 0..8 {
        let angle = (i as f64 / 8.0) * 2.0 * std::f64::consts::PI + phase * 2.0;
        let x = cx as f64 + field_radius as f64 * angle.cos();
        let y = cy as f64 + field_radius as f64 * angle.sin() / 2.0;

        if let (Some(xi), Some(yi)) = (
            to_area_coord(x, area.x, area.width),
            to_area_coord(y, area.y, area.height),
        ) {
            let field_char = FIELD_CHARS[(i + (phase * 10.0) as usize) % FIELD_CHARS.len()];
            render_char(f, xi, yi, field_char, Style::default().fg(Color::Green));
        }
    }
}

fn render_3d_nucleus(f: &mut Frame, area: Rect, cx: u16, cy: u16, is_prime: bool) {
    // 3D-style nucleus
    let layers = vec![
        vec![(0, 0, if is_prime { '★' } else { '◉' })],
        vec![(-1, -1, '◔'), (1, -1, '◕'), (-1, 1, '◕'), (1, 1, '◔')],
    ];

    let color = if is_prime { Color::Green } else { Color::Red };

    for (layer_idx, layer) in layers.iter().enumerate() {
        let opacity = if layer_idx == 0 {
            Modifier::BOLD
        } else {
            Modifier::empty()
        };

        for &(dx, dy, ch) in layer {
            let x = (cx as i16 + dx) as u16;
            let y = (cy as i16 + dy) as u16;
            if x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height {
                render_char(
                    f,
                    x,
                    y,
                    ch,
                    Style::default().fg(color).add_modifier(opacity),
                );
            }
        }
    }
}

fn draw_isometric_ring(f: &mut Frame, area: Rect, cx: u16, cy: u16, radius: u16) {
    // Draw elliptical ring for 3D effect
    for angle_deg in (0..360).step_by(15) {
        let angle = (angle_deg as f64) * std::f64::consts::PI / 180.0;
        let x = cx as f64 + radius as f64 * angle.cos();
        let y = cy as f64 + radius as f64 * angle.sin() / 3.0; // More elliptical

        if let (Some(xi), Some(yi)) = (
            to_area_coord(x, area.x, area.width),
            to_area_coord(y, area.y, area.height),
        ) {
            let ch = if angle_deg % 45 == 0 { '∘' } else { '·' };
            render_char(f, xi, yi, ch, Style::default().fg(Color::DarkGray));
        }
    }
}

fn draw_radial_bond(f: &mut Frame, area: Rect, cx: u16, cy: u16, ex: u16, ey: u16, is_prime: bool) {
    let dx = ex as i16 - cx as i16;
    let dy = ey as i16 - cy as i16;
    let steps = dx.abs().max(dy.abs());

    if steps == 0 {
        return;
    }

    let bond_style = Style::default().fg(if is_prime {
        Color::Cyan
    } else {
        Color::DarkGray
    });

    for i in 1..steps {
        let t = i as f64 / steps as f64;
        let x = cx as f64 + dx as f64 * t;
        let y = cy as f64 + dy as f64 * t;

        if let (Some(xi), Some(yi)) = (
            to_area_coord(x, area.x, area.width),
            to_area_coord(y, area.y, area.height),
        ) {
            let ch = BOND_CHARS
                [(i as usize * BOND_CHARS.len() / steps as usize).min(BOND_CHARS.len() - 1)];
            render_char(f, xi, yi, ch, bond_style);
        }
    }
}

fn draw_vertical_bond(f: &mut Frame, x: u16, y1: u16, y2: u16, is_prime: bool) {
    let bond_style = Style::default().fg(if is_prime {
        Color::Cyan
    } else {
        Color::DarkGray
    });

    for y in y1.min(y2)..=y1.max(y2) {
        render_char(f, x, y, '│', bond_style);
    }
}

fn render_legend(f: &mut Frame, area: Rect) {
    let legend_items = vec![
        ("◯", "Zero (empty orbital)"),
        ("1-9", "Non-zero digit"),
        ("★/◉", "Prime/Composite nucleus"),
        ("·∘•", "Bonds"),
        ("✦✧★", "Prime field"),
    ];

    let legend_x = area.x + area.width - 25;
    let legend_y = area.y + 1;

    for (i, (symbol, desc)) in legend_items.iter().enumerate() {
        let y = legend_y + i as u16;
        if y < area.y + area.height - 1 {
            let line = format!("{}: {}", symbol, desc);
            let text = Line::from(vec![Span::styled(
                line,
                Style::default().fg(Color::DarkGray),
            )]);
            let para = Paragraph::new(text);
            let rect = Rect {
                x: legend_x,
                y,
                width: 24,
                height: 1,
            };
            f.render_widget(para, rect);
        }
    }
}

fn render_char(f: &mut Frame, x: u16, y: u16, ch: char, style: Style) {
    let text = Line::from(vec![Span::styled(ch.to_string(), style)]);
    let para = Paragraph::new(text);
    let rect = Rect {
        x,
        y,
        width: 1,
        height: 1,
    };
    f.render_widget(para, rect);
}

fn electron_appearance(digit: char, is_front: bool) -> (char, Color) {
    if digit == '0' {
        (if is_front { '◯' } else { '○' }, Color::Blue)
    } else {
        (
            digit,
            if is_front {
                Color::Yellow
            } else {
                Color::DarkGray
            },
        )
    }
}

fn group_into_shells(digits: &[char]) -> Vec<Vec<char>> {
    let mut shells = vec![];
    let mut remaining = digits.to_vec();
    let mut shell_size = 2;

    while !remaining.is_empty() {
        let take = shell_size.min(remaining.len());
        shells.push(remaining.drain(..take).collect());
        shell_size = (shell_size + 2).min(8);
    }

    shells
}

fn to_area_coord(pos: f64, start: u16, size: u16) -> Option<u16> {
    let coord = pos.round() as u16;
    if coord >= start && coord < start + size {
        Some(coord)
    } else {
        None
    }
}

fn visualize_membrane(digits: &[char]) -> String {
    digits
        .iter()
        .map(|&c| if c == '0' { '◯' } else { c })
        .collect()
}
