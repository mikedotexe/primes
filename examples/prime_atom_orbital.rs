//! Prime Atom Orbital Visualization
//! 
//! An advanced visualization showing membrane primes as atomic orbitals
//! with electron shells, diagonal bonds, and field effects

use prime_physics_engine::is_prime;
use num_bigint::BigUint;
use std::str::FromStr;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;

#[derive(Clone)]
struct MembraneAtom {
    value: String,
    digits: Vec<char>,
    is_prime: bool,
    electron_shells: Vec<ElectronShell>,
    nucleus_charge: i32,
}

#[derive(Clone)]
struct ElectronShell {
    radius: f64,
    electrons: Vec<Electron>,
    #[allow(dead_code)]
    orbital_type: OrbitalType,
}

#[derive(Clone)]
struct Electron {
    angle: f64,      // Position on the shell
    #[allow(dead_code)]
    spin: bool,      // Up or down
    #[allow(dead_code)]
    energy: f64,     // Energy level
    is_zero: bool,   // Represents a zero in the membrane
}

#[derive(Clone)]
enum OrbitalType {
    S,  // Spherical
    P,  // Dumbbell
    D,  // Complex
}

impl MembraneAtom {
    fn new(membrane: &str) -> Self {
        let digits: Vec<char> = membrane.chars().collect();
        let is_prime = BigUint::from_str(membrane).map(|n| is_prime(&n)).unwrap_or(false);
        
        // Calculate nucleus charge (sum of non-zero digits)
        let nucleus_charge = digits.iter()
            .filter_map(|&c| c.to_digit(10))
            .filter(|&d| d != 0)
            .sum::<u32>() as i32;
        
        // Create electron shells based on the membrane structure
        let mut shells = Vec::new();
        let mut current_radius = 3.0;
        let mut digit_idx = 0;
        
        while digit_idx < digits.len() {
            let mut electrons = Vec::new();
            let electrons_in_shell = 4.min(digits.len() - digit_idx);
            
            for i in 0..electrons_in_shell {
                let angle = (i as f64 * 360.0 / electrons_in_shell as f64) * std::f64::consts::PI / 180.0;
                let digit = digits[digit_idx + i];
                let is_zero = digit == '0';
                let energy = if is_zero { 0.5 } else { digit.to_digit(10).unwrap_or(0) as f64 };
                
                electrons.push(Electron {
                    angle,
                    spin: i % 2 == 0,
                    energy,
                    is_zero,
                });
            }
            
            let orbital_type = match shells.len() {
                0 => OrbitalType::S,
                1 => OrbitalType::P,
                _ => OrbitalType::D,
            };
            
            shells.push(ElectronShell {
                radius: current_radius,
                electrons,
                orbital_type,
            });
            
            digit_idx += electrons_in_shell;
            current_radius += 2.5;
        }
        
        Self {
            value: membrane.to_string(),
            digits,
            is_prime,
            electron_shells: shells,
            nucleus_charge,
        }
    }
}

struct AtomLab {
    atoms: Vec<MembraneAtom>,
    current_atom: usize,
    show_field: bool,
    show_bonds: bool,
    show_orbitals: bool,
    animation_frame: u32,
    view_mode: ViewMode,
}

enum ViewMode {
    Circular,
    Vertical,
    Hybrid,
}

impl AtomLab {
    fn new() -> Self {
        let atoms = vec![
            MembraneAtom::new("10301"),        // Zero-padded: 1-0-3-0-1
            MembraneAtom::new("30703"),        // Simple: 3-0-7-0-3
            MembraneAtom::new("151"),          // Minimal: 1-5-1
            MembraneAtom::new("303050303"),    // Complex: 3-03-05-03-03
            MembraneAtom::new("1003001"),      // Heavy padding: 1-00-3-00-1
            MembraneAtom::new("3305033"),      // Breathing: 33-05-033
        ];
        
        Self {
            atoms,
            current_atom: 0,
            show_field: true,
            show_bonds: true,
            show_orbitals: true,
            animation_frame: 0,
            view_mode: ViewMode::Circular,
        }
    }
    
    fn update(&mut self) {
        self.animation_frame = (self.animation_frame + 1) % 360;
    }
}

fn main() -> Result<(), io::Error> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let mut lab = AtomLab::new();
    
    loop {
        terminal.draw(|f| ui(f, &mut lab))?;
        lab.update();
        
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Left => {
                        if lab.current_atom > 0 {
                            lab.current_atom -= 1;
                        } else {
                            lab.current_atom = lab.atoms.len() - 1;
                        }
                    }
                    KeyCode::Right => {
                        lab.current_atom = (lab.current_atom + 1) % lab.atoms.len();
                    }
                    KeyCode::Char('f') => lab.show_field = !lab.show_field,
                    KeyCode::Char('b') => lab.show_bonds = !lab.show_bonds,
                    KeyCode::Char('o') => lab.show_orbitals = !lab.show_orbitals,
                    KeyCode::Char('v') => {
                        lab.view_mode = match lab.view_mode {
                            ViewMode::Circular => ViewMode::Vertical,
                            ViewMode::Vertical => ViewMode::Hybrid,
                            ViewMode::Hybrid => ViewMode::Circular,
                        };
                    }
                    _ => {}
                }
            }
        }
    }
    
    // Terminal cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    Ok(())
}

fn ui(f: &mut Frame, lab: &mut AtomLab) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),    // Title
            Constraint::Min(20),      // Main display
            Constraint::Length(6),    // Info panel
            Constraint::Length(3),    // Controls
        ])
        .split(f.size());
    
    // Title
    let title = Paragraph::new("⚛️  MEMBRANE PRIME ATOMIC ORBITALS ⚛️")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);
    
    // Main atomic display
    render_atom_display(f, chunks[1], &lab.atoms[lab.current_atom], lab);
    
    // Info panel
    render_info_panel(f, chunks[2], &lab.atoms[lab.current_atom]);
    
    // Controls
    let controls = vec![
        Line::from("←/→: Switch atoms  v: View mode  f: Fields  b: Bonds  o: Orbitals  q: Quit"),
    ];
    let controls_widget = Paragraph::new(controls)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(controls_widget, chunks[3]);
}

fn render_atom_display(f: &mut Frame, area: Rect, atom: &MembraneAtom, lab: &AtomLab) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!("Membrane Atom: {}", atom.value));
    
    let inner_area = block.inner(area);
    f.render_widget(block, area);
    
    // Create a drawing area
    let center_x = inner_area.width as f64 / 2.0;
    let center_y = inner_area.height as f64 / 2.0;
    
    // Custom ASCII rendering
    let mut buffer = vec![vec![' '; inner_area.width as usize]; inner_area.height as usize];
    
    match lab.view_mode {
        ViewMode::Circular => render_circular_atom(&mut buffer, atom, center_x, center_y, lab),
        ViewMode::Vertical => render_vertical_atom(&mut buffer, atom, center_x, center_y),
        ViewMode::Hybrid => render_hybrid_atom(&mut buffer, atom, center_x, center_y, lab),
    }
    
    // Convert buffer to terminal output
    for (y, row) in buffer.iter().enumerate() {
        let y_pos = inner_area.y + y as u16;
        if y_pos >= inner_area.y + inner_area.height {
            break;
        }
        
        for (x, &ch) in row.iter().enumerate() {
            let x_pos = inner_area.x + x as u16;
            if x_pos >= inner_area.x + inner_area.width {
                break;
            }
            
            let style = match ch {
                '●' | '◉' => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                '○' | '◯' => Style::default().fg(Color::Blue),
                '⚡' | '✦' => Style::default().fg(Color::Green),
                '·' | '∘' => Style::default().fg(Color::DarkGray),
                '╱' | '╲' | '─' | '│' => Style::default().fg(Color::Cyan),
                '1'..='9' => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                '0' => Style::default().fg(Color::Blue),
                _ => Style::default(),
            };
            
            let text = Line::from(vec![Span::styled(ch.to_string(), style)]);
            let para = Paragraph::new(text);
            let rect = Rect {
                x: x_pos,
                y: y_pos,
                width: 1,
                height: 1,
            };
            f.render_widget(para, rect);
        }
    }
}

fn render_circular_atom(buffer: &mut Vec<Vec<char>>, atom: &MembraneAtom, cx: f64, cy: f64, lab: &AtomLab) {
    // Draw nucleus
    let nucleus_size = (atom.nucleus_charge as f64 / 5.0).max(1.0).min(3.0) as usize;
    for dy in -(nucleus_size as i32)..=nucleus_size as i32 {
        for dx in -(nucleus_size as i32)..=nucleus_size as i32 {
            let x = (cx + dx as f64) as usize;
            let y = (cy + dy as f64 / 2.0) as usize;
            if x < buffer[0].len() && y < buffer.len() {
                if dx.abs() + dy.abs() <= nucleus_size as i32 {
                    buffer[y][x] = if atom.is_prime { '◉' } else { '●' };
                }
            }
        }
    }
    
    // Draw electron shells
    for shell in &atom.electron_shells {
        let radius = shell.radius;
        
        // Draw orbital path
        if lab.show_orbitals {
            draw_circle(buffer, cx, cy, radius, '·');
        }
        
        // Draw electrons
        for electron in &shell.electrons {
            let angle = electron.angle + (lab.animation_frame as f64 * 0.01);
            let x = cx + radius * angle.cos();
            let y = cy + radius * angle.sin() / 2.0;
            
            if let (Some(xi), Some(yi)) = (to_buffer_coord(x, buffer[0].len()), to_buffer_coord(y, buffer.len())) {
                buffer[yi][xi] = if electron.is_zero { '○' } else { '●' };
                
                // Draw bonds to nucleus
                if lab.show_bonds {
                    draw_line(buffer, cx, cy, x, y, '·');
                }
            }
        }
    }
    
    // Draw field effect
    if lab.show_field && atom.is_prime {
        let field_radius = atom.electron_shells.last().map(|s| s.radius + 3.0).unwrap_or(8.0);
        draw_field_effect(buffer, cx, cy, field_radius, lab.animation_frame);
    }
}

fn render_vertical_atom(buffer: &mut Vec<Vec<char>>, atom: &MembraneAtom, cx: f64, cy: f64) {
    // Render digits vertically with connections
    let start_y = cy - (atom.digits.len() as f64 / 2.0);
    
    for (i, &digit) in atom.digits.iter().enumerate() {
        let y = start_y + i as f64;
        if let (Some(xi), Some(yi)) = (to_buffer_coord(cx, buffer[0].len()), to_buffer_coord(y, buffer.len())) {
            buffer[yi][xi] = digit;
            
            // Draw connections between non-zero digits
            if i > 0 && digit != '0' && atom.digits[i-1] != '0' {
                let prev_y = start_y + (i-1) as f64;
                draw_line(buffer, cx, prev_y, cx, y, '│');
            }
        }
    }
}

fn render_hybrid_atom(buffer: &mut Vec<Vec<char>>, atom: &MembraneAtom, cx: f64, cy: f64, lab: &AtomLab) {
    // Combine circular and vertical representations
    render_circular_atom(buffer, atom, cx - 10.0, cy, lab);
    
    // Add vertical representation on the right
    let vcx = cx + 10.0;
    render_vertical_atom(buffer, atom, vcx, cy);
    
    // Draw connection between representations
    draw_line(buffer, cx - 5.0, cy, cx + 5.0, cy, '─');
}

fn draw_circle(buffer: &mut Vec<Vec<char>>, cx: f64, cy: f64, radius: f64, ch: char) {
    let steps = (radius * 8.0) as i32;
    for i in 0..steps {
        let angle = (i as f64 * 2.0 * std::f64::consts::PI) / steps as f64;
        let x = cx + radius * angle.cos();
        let y = cy + radius * angle.sin() / 2.0; // Adjust for terminal aspect ratio
        
        if let (Some(xi), Some(yi)) = (to_buffer_coord(x, buffer[0].len()), to_buffer_coord(y, buffer.len())) {
            if buffer[yi][xi] == ' ' {
                buffer[yi][xi] = ch;
            }
        }
    }
}

fn draw_line(buffer: &mut Vec<Vec<char>>, x1: f64, y1: f64, x2: f64, y2: f64, ch: char) {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let steps = dx.abs().max(dy.abs()) as i32;
    
    if steps == 0 { return; }
    
    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        let x = x1 + dx * t;
        let y = y1 + dy * t;
        
        if let (Some(xi), Some(yi)) = (to_buffer_coord(x, buffer[0].len()), to_buffer_coord(y, buffer.len())) {
            if buffer[yi][xi] == ' ' {
                // Choose appropriate line character
                let line_char = if dx.abs() > dy.abs() {
                    '─'
                } else if dy.abs() > dx.abs() {
                    '│'
                } else if (dx > 0.0 && dy > 0.0) || (dx < 0.0 && dy < 0.0) {
                    '╲'
                } else {
                    '╱'
                };
                buffer[yi][xi] = if ch == '·' { ch } else { line_char };
            }
        }
    }
}

fn draw_field_effect(buffer: &mut Vec<Vec<char>>, cx: f64, cy: f64, radius: f64, frame: u32) {
    let phase = (frame as f64 * 0.1) % (2.0 * std::f64::consts::PI);
    
    for angle_deg in (0..360).step_by(20) {
        let angle = (angle_deg as f64 + phase * 10.0) * std::f64::consts::PI / 180.0;
        let r = radius + (phase.sin() * 2.0);
        let x = cx + r * angle.cos();
        let y = cy + r * angle.sin() / 2.0;
        
        if let (Some(xi), Some(yi)) = (to_buffer_coord(x, buffer[0].len()), to_buffer_coord(y, buffer.len())) {
            if buffer[yi][xi] == ' ' {
                buffer[yi][xi] = if frame % 10 < 5 { '⚡' } else { '✦' };
            }
        }
    }
}

fn to_buffer_coord(pos: f64, max: usize) -> Option<usize> {
    let coord = pos as usize;
    if coord < max { Some(coord) } else { None }
}

fn render_info_panel(f: &mut Frame, area: Rect, atom: &MembraneAtom) {
    let info_text = vec![
        Line::from(vec![
            Span::raw("Value: "),
            Span::styled(
                &atom.value,
                Style::default().fg(if atom.is_prime { Color::Green } else { Color::Red }).add_modifier(Modifier::BOLD)
            ),
            Span::raw(if atom.is_prime { " ✓ PRIME" } else { " ✗ NOT PRIME" }),
        ]),
        Line::from(format!("Structure: {}", atom.digits.iter().map(|&c| if c == '0' { '◯' } else { c }).collect::<String>())),
        Line::from(format!("Nucleus charge: {}", atom.nucleus_charge)),
        Line::from(format!("Electron shells: {}", atom.electron_shells.len())),
        Line::from(format!("Total electrons: {}", atom.digits.len())),
    ];
    
    let info = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::ALL).title("Atomic Properties"));
    f.render_widget(info, area);
}