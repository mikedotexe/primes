//! Membrane Sphere TUI - 3D-like spherical representation of membrane primes
//!
//! Creates a beautiful spherical visualization with diagonal bonds,
//! showing how zeros create electron shells and non-zero digits form the nucleus

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

struct MembraneSphere {
    membrane: String,
    digits: Vec<char>,
    is_prime: bool,
    radius: f64,
    rotation_x: f64,
    rotation_y: f64,
    rotation_z: f64,
}

impl MembraneSphere {
    fn new(membrane: &str) -> Self {
        let digits: Vec<char> = membrane.chars().collect();
        let is_prime = BigUint::from_str(membrane)
            .map(|n| is_prime(&n))
            .unwrap_or(false);

        Self {
            membrane: membrane.to_string(),
            digits,
            is_prime,
            radius: 10.0,
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
        }
    }

    fn rotate(&mut self, dx: f64, dy: f64, dz: f64) {
        self.rotation_x = (self.rotation_x + dx) % (2.0 * std::f64::consts::PI);
        self.rotation_y = (self.rotation_y + dy) % (2.0 * std::f64::consts::PI);
        self.rotation_z = (self.rotation_z + dz) % (2.0 * std::f64::consts::PI);
    }
}

struct SphereExplorer {
    spheres: Vec<MembraneSphere>,
    current: usize,
    show_bonds: bool,
    show_field: bool,
    show_grid: bool,
    animation_speed: f64,
    view_distance: f64,
}

impl SphereExplorer {
    fn new() -> Self {
        let spheres = vec![
            MembraneSphere::new("10301"),     // 1-0-3-0-1
            MembraneSphere::new("30703"),     // 3-0-7-0-3
            MembraneSphere::new("303050303"), // Complex
            MembraneSphere::new("151"),       // Minimal
            MembraneSphere::new("1003001"),   // Heavy zeros
            MembraneSphere::new("3305033"),   // Breathing
        ];

        Self {
            spheres,
            current: 0,
            show_bonds: true,
            show_field: true,
            show_grid: false,
            animation_speed: 0.02,
            view_distance: 30.0,
        }
    }

    fn update(&mut self) {
        let sphere = &mut self.spheres[self.current];
        sphere.rotate(
            self.animation_speed,
            self.animation_speed * 0.7,
            self.animation_speed * 0.3,
        );
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut explorer = SphereExplorer::new();

    loop {
        terminal.draw(|f| ui(f, &mut explorer))?;
        explorer.update();

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Left => {
                        explorer.current = if explorer.current == 0 {
                            explorer.spheres.len() - 1
                        } else {
                            explorer.current - 1
                        };
                    }
                    KeyCode::Right => {
                        explorer.current = (explorer.current + 1) % explorer.spheres.len();
                    }
                    KeyCode::Char('b') => explorer.show_bonds = !explorer.show_bonds,
                    KeyCode::Char('f') => explorer.show_field = !explorer.show_field,
                    KeyCode::Char('g') => explorer.show_grid = !explorer.show_grid,
                    KeyCode::Char('+') => {
                        explorer.view_distance = (explorer.view_distance - 5.0).max(10.0)
                    }
                    KeyCode::Char('-') => {
                        explorer.view_distance = (explorer.view_distance + 5.0).min(50.0)
                    }
                    KeyCode::Char(' ') => {
                        explorer.animation_speed = if explorer.animation_speed > 0.0 {
                            0.0
                        } else {
                            0.02
                        };
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

fn ui(f: &mut Frame, explorer: &mut SphereExplorer) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(20),
            Constraint::Length(5),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new("🌐 MEMBRANE PRIME SPHERES 🌐")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main sphere display
    render_sphere(f, chunks[1], &explorer.spheres[explorer.current], explorer);

    // Info
    let sphere = &explorer.spheres[explorer.current];
    let info_lines = vec![
        Line::from(vec![
            Span::raw("Membrane: "),
            Span::styled(
                &sphere.membrane,
                Style::default()
                    .fg(if sphere.is_prime {
                        Color::Green
                    } else {
                        Color::Red
                    })
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(if sphere.is_prime {
                " ✓ PRIME"
            } else {
                " ✗ NOT PRIME"
            }),
        ]),
        Line::from(format!(
            "Structure: {}",
            visualize_structure(&sphere.digits)
        )),
        Line::from(format!(
            "Rotation: ({:.1}, {:.1}, {:.1})",
            sphere.rotation_x.to_degrees(),
            sphere.rotation_y.to_degrees(),
            sphere.rotation_z.to_degrees()
        )),
    ];
    let info = Paragraph::new(info_lines)
        .block(Block::default().borders(Borders::ALL).title("Properties"));
    f.render_widget(info, chunks[2]);

    // Controls
    let controls = vec![Line::from(
        "←/→: Switch  SPACE: Pause  b: Bonds  f: Field  g: Grid  +/-: Zoom  q: Quit",
    )];
    let controls_widget = Paragraph::new(controls)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(controls_widget, chunks[3]);
}

fn render_sphere(f: &mut Frame, area: Rect, sphere: &MembraneSphere, explorer: &SphereExplorer) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!("3D Membrane Sphere: {}", sphere.membrane));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Create render buffer
    let width = inner.width as usize;
    let height = inner.height as usize;
    let mut buffer = vec![vec![(' ', 0.0); width]; height];
    let mut z_buffer = vec![vec![f64::NEG_INFINITY; width]; height];

    let cx = width as f64 / 2.0;
    let cy = height as f64 / 2.0;

    // Render the sphere structure
    render_sphere_structure(&mut buffer, &mut z_buffer, sphere, cx, cy, explorer);

    // Convert buffer to display
    for (y, row) in buffer.iter().enumerate() {
        for (x, &(ch, intensity)) in row.iter().enumerate() {
            if ch != ' ' {
                let color = match ch {
                    '1'..='9' => {
                        // Non-zero digits glow based on primality
                        if sphere.is_prime {
                            interpolate_color(Color::Yellow, Color::Green, intensity)
                        } else {
                            interpolate_color(Color::Red, Color::Yellow, intensity)
                        }
                    }
                    '0' | '◯' => interpolate_color(Color::Blue, Color::Cyan, intensity),
                    '·' | '∘' => Color::DarkGray,
                    '─' | '│' | '╱' | '╲' | '╳' => {
                        interpolate_color(Color::Cyan, Color::White, intensity)
                    }
                    '⚡' | '✦' | '★' => {
                        interpolate_color(Color::Green, Color::Yellow, intensity)
                    }
                    _ => Color::White,
                };

                let mut style = Style::default().fg(color);
                if ch >= '1' && ch <= '9' {
                    style = style.add_modifier(Modifier::BOLD);
                }

                let text = Line::from(vec![Span::styled(ch.to_string(), style)]);
                let para = Paragraph::new(text);
                let rect = Rect {
                    x: inner.x + x as u16,
                    y: inner.y + y as u16,
                    width: 1,
                    height: 1,
                };
                f.render_widget(para, rect);
            }
        }
    }
}

fn render_sphere_structure(
    buffer: &mut Vec<Vec<(char, f64)>>,
    z_buffer: &mut Vec<Vec<f64>>,
    sphere: &MembraneSphere,
    cx: f64,
    cy: f64,
    explorer: &SphereExplorer,
) {
    let num_points = sphere.digits.len();

    // Calculate 3D positions for each digit on the sphere
    let mut positions_3d = Vec::new();
    for i in 0..num_points {
        let theta = (i as f64 / num_points as f64) * 2.0 * std::f64::consts::PI;
        let phi = ((i as f64 + 0.5) / num_points as f64) * std::f64::consts::PI;

        let x = sphere.radius * phi.sin() * theta.cos();
        let y = sphere.radius * phi.sin() * theta.sin();
        let z = sphere.radius * phi.cos();

        positions_3d.push((x, y, z));
    }

    // Apply rotations
    let rotated_positions: Vec<_> = positions_3d
        .iter()
        .map(|&(x, y, z)| {
            rotate_3d(
                x,
                y,
                z,
                sphere.rotation_x,
                sphere.rotation_y,
                sphere.rotation_z,
            )
        })
        .collect();

    // Project to 2D and render
    for (i, &(x3d, y3d, z3d)) in rotated_positions.iter().enumerate() {
        let (x2d, y2d) = project_3d_to_2d(x3d, y3d, z3d, explorer.view_distance);
        let screen_x = cx + x2d;
        let screen_y = cy + y2d / 2.0; // Adjust for terminal aspect ratio

        if let (Some(sx), Some(sy)) = (
            to_screen_coord(screen_x, buffer[0].len()),
            to_screen_coord(screen_y, buffer.len()),
        ) {
            let depth = z3d + explorer.view_distance;
            if depth > z_buffer[sy][sx] {
                z_buffer[sy][sx] = depth;
                let intensity = (depth / (2.0 * explorer.view_distance)).clamp(0.0, 1.0);
                buffer[sy][sx] = (sphere.digits[i], intensity);

                // Draw bonds
                if explorer.show_bonds {
                    // Connect to adjacent points
                    let next_i = (i + 1) % num_points;
                    let (nx3d, ny3d, nz3d) = rotated_positions[next_i];
                    draw_3d_line(
                        buffer,
                        z_buffer,
                        x3d,
                        y3d,
                        z3d,
                        nx3d,
                        ny3d,
                        nz3d,
                        cx,
                        cy,
                        explorer.view_distance,
                        '─',
                        intensity,
                    );

                    // Diagonal connections for structure
                    if i < num_points - 2 {
                        let diag_i = (i + num_points / 3) % num_points;
                        let (dx3d, dy3d, dz3d) = rotated_positions[diag_i];
                        draw_3d_line(
                            buffer,
                            z_buffer,
                            x3d,
                            y3d,
                            z3d,
                            dx3d,
                            dy3d,
                            dz3d,
                            cx,
                            cy,
                            explorer.view_distance,
                            '╱',
                            intensity * 0.5,
                        );
                    }
                }
            }
        }
    }

    // Draw center/nucleus
    let (nx2d, ny2d) = project_3d_to_2d(0.0, 0.0, 0.0, explorer.view_distance);
    let nucleus_x = cx + nx2d;
    let nucleus_y = cy + ny2d / 2.0;

    if let (Some(nx), Some(ny)) = (
        to_screen_coord(nucleus_x, buffer[0].len()),
        to_screen_coord(nucleus_y, buffer.len()),
    ) {
        if explorer.view_distance > z_buffer[ny][nx] {
            z_buffer[ny][nx] = explorer.view_distance;
            buffer[ny][nx] = (if sphere.is_prime { '★' } else { '◉' }, 1.0);
        }
    }

    // Draw field effect
    if explorer.show_field && sphere.is_prime {
        draw_field_3d(buffer, z_buffer, sphere, cx, cy, explorer);
    }

    // Draw reference grid
    if explorer.show_grid {
        draw_reference_grid(buffer, z_buffer, cx, cy, sphere.radius * 1.5, explorer);
    }
}

fn rotate_3d(x: f64, y: f64, z: f64, rx: f64, ry: f64, rz: f64) -> (f64, f64, f64) {
    // Rotate around X
    let (y1, z1) = (y * rx.cos() - z * rx.sin(), y * rx.sin() + z * rx.cos());

    // Rotate around Y
    let (x2, z2) = (x * ry.cos() + z1 * ry.sin(), -x * ry.sin() + z1 * ry.cos());

    // Rotate around Z
    let (x3, y3) = (x2 * rz.cos() - y1 * rz.sin(), x2 * rz.sin() + y1 * rz.cos());

    (x3, y3, z2)
}

fn project_3d_to_2d(x: f64, y: f64, z: f64, view_distance: f64) -> (f64, f64) {
    let perspective = view_distance / (view_distance + z);
    (x * perspective, y * perspective)
}

fn draw_3d_line(
    buffer: &mut Vec<Vec<(char, f64)>>,
    z_buffer: &mut Vec<Vec<f64>>,
    x1: f64,
    y1: f64,
    z1: f64,
    x2: f64,
    y2: f64,
    z2: f64,
    cx: f64,
    cy: f64,
    view_distance: f64,
    ch: char,
    base_intensity: f64,
) {
    let steps = 20;
    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        let x = x1 + (x2 - x1) * t;
        let y = y1 + (y2 - y1) * t;
        let z = z1 + (z2 - z1) * t;

        let (x2d, y2d) = project_3d_to_2d(x, y, z, view_distance);
        let screen_x = cx + x2d;
        let screen_y = cy + y2d / 2.0;

        if let (Some(sx), Some(sy)) = (
            to_screen_coord(screen_x, buffer[0].len()),
            to_screen_coord(screen_y, buffer.len()),
        ) {
            let depth = z + view_distance;
            if depth > z_buffer[sy][sx] - 0.1 {
                // Small offset to allow overlapping
                z_buffer[sy][sx] = depth;
                let intensity = base_intensity * (depth / (2.0 * view_distance)).clamp(0.0, 1.0);
                if buffer[sy][sx].0 == ' ' || buffer[sy][sx].0 == '·' {
                    buffer[sy][sx] = (ch, intensity);
                }
            }
        }
    }
}

fn draw_field_3d(
    buffer: &mut Vec<Vec<(char, f64)>>,
    z_buffer: &mut Vec<Vec<f64>>,
    sphere: &MembraneSphere,
    cx: f64,
    cy: f64,
    explorer: &SphereExplorer,
) {
    let field_radius = sphere.radius * 1.3;

    for i in 0..12 {
        let theta = (i as f64 / 12.0) * 2.0 * std::f64::consts::PI;
        for j in 0..8 {
            let phi = (j as f64 / 8.0) * std::f64::consts::PI;

            let x = field_radius * phi.sin() * theta.cos();
            let y = field_radius * phi.sin() * theta.sin();
            let z = field_radius * phi.cos();

            let (rx, ry, rz) = rotate_3d(
                x,
                y,
                z,
                sphere.rotation_x,
                sphere.rotation_y,
                sphere.rotation_z,
            );
            let (x2d, y2d) = project_3d_to_2d(rx, ry, rz, explorer.view_distance);

            let screen_x = cx + x2d;
            let screen_y = cy + y2d / 2.0;

            if let (Some(sx), Some(sy)) = (
                to_screen_coord(screen_x, buffer[0].len()),
                to_screen_coord(screen_y, buffer.len()),
            ) {
                let depth = rz + explorer.view_distance;
                if depth > z_buffer[sy][sx] && buffer[sy][sx].0 == ' ' {
                    z_buffer[sy][sx] = depth;
                    let intensity = (depth / (2.0 * explorer.view_distance)).clamp(0.0, 1.0);
                    buffer[sy][sx] = ('⚡', intensity * 0.5);
                }
            }
        }
    }
}

fn draw_reference_grid(
    buffer: &mut Vec<Vec<(char, f64)>>,
    _z_buffer: &mut Vec<Vec<f64>>,
    cx: f64,
    cy: f64,
    radius: f64,
    explorer: &SphereExplorer,
) {
    // Draw latitude lines
    for i in 1..6 {
        let phi = (i as f64 / 6.0) * std::f64::consts::PI;
        for j in 0..24 {
            let theta = (j as f64 / 24.0) * 2.0 * std::f64::consts::PI;

            let x = radius * phi.sin() * theta.cos();
            let y = radius * phi.sin() * theta.sin();
            let z = radius * phi.cos();

            let (rx, ry, rz) = rotate_3d(x, y, z, 0.0, 0.0, 0.0);
            let (x2d, y2d) = project_3d_to_2d(rx, ry, rz, explorer.view_distance);

            let screen_x = cx + x2d;
            let screen_y = cy + y2d / 2.0;

            if let (Some(sx), Some(sy)) = (
                to_screen_coord(screen_x, buffer[0].len()),
                to_screen_coord(screen_y, buffer.len()),
            ) {
                if buffer[sy][sx].0 == ' ' {
                    buffer[sy][sx] = ('·', 0.3);
                }
            }
        }
    }
}

fn to_screen_coord(pos: f64, max: usize) -> Option<usize> {
    let coord = pos.round() as usize;
    if coord < max {
        Some(coord)
    } else {
        None
    }
}

fn interpolate_color(from: Color, to: Color, t: f64) -> Color {
    // Simple color interpolation
    match (from, to, (t * 3.0) as u8) {
        (Color::Blue, Color::Cyan, 0) => Color::Blue,
        (Color::Blue, Color::Cyan, 1) => Color::LightBlue,
        (Color::Blue, Color::Cyan, _) => Color::Cyan,
        (Color::Yellow, Color::Green, 0) => Color::Yellow,
        (Color::Yellow, Color::Green, 1) => Color::LightGreen,
        (Color::Yellow, Color::Green, _) => Color::Green,
        (Color::Red, Color::Yellow, 0) => Color::Red,
        (Color::Red, Color::Yellow, 1) => Color::LightRed,
        (Color::Red, Color::Yellow, _) => Color::Yellow,
        (Color::Cyan, Color::White, 0) => Color::Cyan,
        (Color::Cyan, Color::White, 1) => Color::LightCyan,
        (Color::Cyan, Color::White, _) => Color::White,
        (Color::Green, Color::Yellow, 0) => Color::Green,
        (Color::Green, Color::Yellow, 1) => Color::LightGreen,
        (Color::Green, Color::Yellow, _) => Color::Yellow,
        _ => from,
    }
}

fn visualize_structure(digits: &[char]) -> String {
    digits
        .iter()
        .map(|&c| if c == '0' { '◯' } else { c })
        .collect()
}
