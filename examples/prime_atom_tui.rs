//! Prime Atom TUI - Visualizing membrane primes as atomic structures
//!
//! This creates a circular/spherical representation of membrane primes
//! with vertical orientation and diagonal ASCII art

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use num_bigint::BigUint;
use primes::is_prime;
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

struct PrimeAtom {
    membrane: String,
    structure: Vec<char>,
    is_prime: bool,
    current_rotation: f32,
    show_field_lines: bool,
    show_orbital_view: bool,
}

impl PrimeAtom {
    fn new(membrane: &str) -> Self {
        let structure: Vec<char> = membrane.chars().collect();
        let is_prime = if let Ok(num) = BigUint::from_str(membrane) {
            is_prime(&num)
        } else {
            false
        };

        Self {
            membrane: membrane.to_string(),
            structure,
            is_prime,
            current_rotation: 0.0,
            show_field_lines: true,
            show_orbital_view: false,
        }
    }

    fn rotate(&mut self, angle: f32) {
        self.current_rotation = (self.current_rotation + angle) % 360.0;
    }
}

struct AtomExplorer {
    atoms: Vec<PrimeAtom>,
    current_atom: usize,
    show_help: bool,
}

impl AtomExplorer {
    fn new() -> Self {
        let atoms = vec![
            PrimeAtom::new("10301"),     // 1-0-3-0-1
            PrimeAtom::new("151"),       // 1-5-1
            PrimeAtom::new("303050303"), // 3-03-05-03-03
            PrimeAtom::new("3305033"),   // 33-05-033
            PrimeAtom::new("30703"),     // 3-07-03
        ];

        Self {
            atoms,
            current_atom: 0,
            show_help: false,
        }
    }

    fn next_atom(&mut self) {
        self.current_atom = (self.current_atom + 1) % self.atoms.len();
    }

    fn prev_atom(&mut self) {
        if self.current_atom == 0 {
            self.current_atom = self.atoms.len() - 1;
        } else {
            self.current_atom -= 1;
        }
    }
}

fn main() -> Result<(), io::Error> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = AtomExplorer::new();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('h') => app.show_help = !app.show_help,
                KeyCode::Left => app.prev_atom(),
                KeyCode::Right => app.next_atom(),
                KeyCode::Char('r') => app.atoms[app.current_atom].rotate(15.0),
                KeyCode::Char('f') => {
                    let atom = &mut app.atoms[app.current_atom];
                    atom.show_field_lines = !atom.show_field_lines;
                }
                KeyCode::Char('o') => {
                    let atom = &mut app.atoms[app.current_atom];
                    atom.show_orbital_view = !atom.show_orbital_view;
                }
                _ => {}
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

fn ui(f: &mut Frame, app: &mut AtomExplorer) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(20),   // Main atom display
            Constraint::Length(4), // Info
            Constraint::Length(4), // Controls
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new("⚛️  PRIME ATOM EXPLORER")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main atom display
    let atom_display = Block::default()
        .borders(Borders::ALL)
        .title("Membrane Prime as Atomic Structure");
    f.render_widget(atom_display, chunks[1]);

    // Render the atom inside
    render_atom(f, chunks[1], &app.atoms[app.current_atom]);

    // Info
    let atom = &app.atoms[app.current_atom];
    let info_text = vec![
        Line::from(vec![
            Span::raw("Prime: "),
            Span::styled(
                &atom.membrane,
                Style::default().fg(if atom.is_prime {
                    Color::Green
                } else {
                    Color::Red
                }),
            ),
            Span::raw(if atom.is_prime { " ✓" } else { " ✗" }),
        ]),
        Line::from(format!(
            "Structure: {}",
            visualize_structure(&atom.structure)
        )),
    ];
    let info = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::ALL).title("Current Atom"));
    f.render_widget(info, chunks[2]);

    // Controls
    let controls = if app.show_help {
        vec![
            Line::from("←/→: Switch atoms  r: Rotate  f: Toggle fields  o: Toggle orbital view"),
            Line::from("h: Hide help  q: Quit"),
        ]
    } else {
        vec![Line::from("Press 'h' for help")]
    };
    let controls_widget = Paragraph::new(controls).block(Block::default().borders(Borders::ALL));
    f.render_widget(controls_widget, chunks[3]);
}

fn render_atom(f: &mut Frame, area: Rect, atom: &PrimeAtom) {
    // Calculate center
    let center_x = area.x + area.width / 2;
    let center_y = area.y + area.height / 2;

    // Create a circular representation
    let radius = (area.height.min(area.width / 2) / 2).max(5) as i32;

    // First, draw the circular structure
    if atom.show_orbital_view {
        render_orbital_view(f, area, atom, center_x, center_y, radius);
    } else {
        render_vertical_structure(f, area, atom, center_x, center_y);
    }

    // Optionally show field lines
    if atom.show_field_lines {
        render_field_lines(f, area, center_x, center_y, radius, atom.is_prime);
    }
}

fn render_vertical_structure(
    f: &mut Frame,
    area: Rect,
    atom: &PrimeAtom,
    center_x: u16,
    center_y: u16,
) {
    // Render the membrane structure vertically
    let mut y_offset = center_y.saturating_sub(atom.structure.len() as u16 / 2);

    for &ch in atom.structure.iter() {
        if y_offset < area.y || y_offset >= area.y + area.height {
            continue;
        }

        let style = if ch == '0' {
            Style::default().fg(Color::Blue)
        } else {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        };

        // Create vertical text
        let text = Line::from(vec![Span::styled(ch.to_string(), style)]);
        let para = Paragraph::new(text).alignment(Alignment::Center);

        let rect = Rect {
            x: center_x.saturating_sub(1),
            y: y_offset,
            width: 3,
            height: 1,
        };

        f.render_widget(para, rect);
        y_offset += 1;
    }
}

fn render_orbital_view(
    f: &mut Frame,
    area: Rect,
    atom: &PrimeAtom,
    center_x: u16,
    center_y: u16,
    radius: i32,
) {
    // Draw atom as concentric circles with digits at orbital positions
    let mut orbital_radius = 3;
    let mut digit_index = 0;

    while digit_index < atom.structure.len() && orbital_radius < radius {
        // Draw orbital circle using ASCII
        draw_circle(f, area, center_x, center_y, orbital_radius);

        // Place digits at cardinal points
        if digit_index < atom.structure.len() {
            let ch = atom.structure[digit_index];
            let style = if ch == '0' {
                Style::default().fg(Color::Blue)
            } else {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            };

            // Place at four cardinal points
            place_orbital_digit(f, area, center_x, center_y, orbital_radius, ch, style, 0); // Top
            if digit_index + 1 < atom.structure.len() {
                digit_index += 1;
                place_orbital_digit(
                    f,
                    area,
                    center_x,
                    center_y,
                    orbital_radius,
                    atom.structure[digit_index],
                    style,
                    90,
                ); // Right
            }
            if digit_index + 1 < atom.structure.len() {
                digit_index += 1;
                place_orbital_digit(
                    f,
                    area,
                    center_x,
                    center_y,
                    orbital_radius,
                    atom.structure[digit_index],
                    style,
                    180,
                ); // Bottom
            }
            if digit_index + 1 < atom.structure.len() {
                digit_index += 1;
                place_orbital_digit(
                    f,
                    area,
                    center_x,
                    center_y,
                    orbital_radius,
                    atom.structure[digit_index],
                    style,
                    270,
                ); // Left
            }
        }

        digit_index += 1;
        orbital_radius += 3;
    }
}

fn draw_circle(f: &mut Frame, area: Rect, center_x: u16, center_y: u16, radius: i32) {
    // Simple circle drawing using ASCII characters
    let circle_chars = ['·', '∘', '○', '◯'];
    let char_index = (radius as usize / 3).min(circle_chars.len() - 1);
    let circle_char = circle_chars[char_index];

    // Draw approximate circle using Bresenham-like approach
    for angle in 0..360 {
        let radian = (angle as f32) * std::f32::consts::PI / 180.0;
        let x = center_x as i32 + (radius as f32 * radian.cos()) as i32;
        let y = center_y as i32 + (radius as f32 * radian.sin() / 2.0) as i32; // Adjust for terminal aspect ratio

        if x >= area.x as i32
            && x < (area.x + area.width) as i32
            && y >= area.y as i32
            && y < (area.y + area.height) as i32
        {
            let text = Line::from(vec![Span::raw(circle_char.to_string())]);
            let para = Paragraph::new(text);
            let rect = Rect {
                x: x as u16,
                y: y as u16,
                width: 1,
                height: 1,
            };
            f.render_widget(para, rect);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn place_orbital_digit(
    f: &mut Frame,
    area: Rect,
    center_x: u16,
    center_y: u16,
    radius: i32,
    ch: char,
    style: Style,
    angle: i32,
) {
    let radian = (angle as f32) * std::f32::consts::PI / 180.0;
    let x = center_x as i32 + (radius as f32 * radian.cos()) as i32;
    let y = center_y as i32 + (radius as f32 * radian.sin() / 2.0) as i32;

    if x >= area.x as i32
        && x < (area.x + area.width) as i32
        && y >= area.y as i32
        && y < (area.y + area.height) as i32
    {
        let text = Line::from(vec![Span::styled(ch.to_string(), style)]);
        let para = Paragraph::new(text);
        let rect = Rect {
            x: x as u16,
            y: y as u16,
            width: 1,
            height: 1,
        };
        f.render_widget(para, rect);
    }
}

fn render_field_lines(
    f: &mut Frame,
    area: Rect,
    center_x: u16,
    center_y: u16,
    radius: i32,
    is_prime: bool,
) {
    // Draw field lines emanating from the atom
    let field_char = if is_prime { '⚡' } else { '·' };
    let field_color = if is_prime {
        Color::Green
    } else {
        Color::DarkGray
    };

    // Draw radial field lines
    for angle in (0..360).step_by(45) {
        let radian = (angle as f32) * std::f32::consts::PI / 180.0;
        for r in (radius + 2)..(radius + 5) {
            let x = center_x as i32 + (r as f32 * radian.cos()) as i32;
            let y = center_y as i32 + (r as f32 * radian.sin() / 2.0) as i32;

            if x >= area.x as i32
                && x < (area.x + area.width) as i32
                && y >= area.y as i32
                && y < (area.y + area.height) as i32
            {
                let text = Line::from(vec![Span::styled(
                    field_char.to_string(),
                    Style::default().fg(field_color),
                )]);
                let para = Paragraph::new(text);
                let rect = Rect {
                    x: x as u16,
                    y: y as u16,
                    width: 1,
                    height: 1,
                };
                f.render_widget(para, rect);
            }
        }
    }
}

fn visualize_structure(structure: &[char]) -> String {
    structure
        .iter()
        .map(|&c| if c == '0' { '◯' } else { c })
        .collect()
}
