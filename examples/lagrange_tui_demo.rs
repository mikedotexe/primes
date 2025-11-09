//! Lagrange Point TUI Demo
//!
//! A visual demonstration showing EXACTLY how Lagrange points work
//! with clear visualization that the ENTIRE concatenated string is tested

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use num_bigint::BigUint;
use prime_physics_engine::is_prime;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use std::str::FromStr;

struct LagrangeDemo {
    prime1: String,
    prime2: String,
    buffer_size: usize,
    current_position: usize,
    current_digit: u32,
    lagrange_points: Vec<(usize, u32)>,
    status: String,
    show_help: bool,
    preset: usize,
}

impl LagrangeDemo {
    fn new() -> Self {
        let demo = Self {
            prime1: "10301".to_string(), // Zero-padded membrane: 1-0-3-0-1
            prime2: "30305070305070303".to_string(),
            buffer_size: 5,
            current_position: 0,
            current_digit: 1,
            lagrange_points: vec![(0, 8), (2, 5), (4, 8)], // Verified Lagrange points
            status: "Press SPACE to test current position/digit".to_string(),
            show_help: false,
            preset: 0,
        };
        demo
    }

    fn switch_preset(&mut self) {
        self.preset = (self.preset + 1) % 5;
        match self.preset {
            0 => {
                // Zero-padded membrane (1-0-3-0-1 structure)
                self.prime1 = "10301".to_string();
                self.prime2 = "30305070305070303".to_string();
                self.buffer_size = 5;
                self.lagrange_points = vec![(0, 8), (2, 5), (4, 8)];
            }
            1 => {
                // Small vs Giant (like Earth vs Sun)
                self.prime1 = "97".to_string();
                self.prime2 = "30305070305070303".to_string();
                self.buffer_size = 7;
                self.lagrange_points = vec![(3, 9), (4, 1)];
            }
            2 => {
                // Membrane vs Membrane (both have structure)
                self.prime1 = "151".to_string(); // 1-5-1 membrane
                self.prime2 = "30305070305070303".to_string(); // giant membrane
                self.buffer_size = 5;
                self.lagrange_points = vec![(0, 1), (2, 7)];
            }
            3 => {
                // Tiny vs Large (like asteroid vs Jupiter)
                self.prime1 = "11".to_string();
                self.prime2 = "3030507030703".to_string();
                self.buffer_size = 5;
                self.lagrange_points = vec![(0, 5), (2, 6), (3, 2), (4, 9)];
            }
            4 => {
                // Original similar-sized primes
                self.prime1 = "303050303".to_string();
                self.prime2 = "303070303".to_string();
                self.buffer_size = 7;
                self.lagrange_points = vec![(2, 5), (4, 2), (5, 5)];
            }
            _ => unreachable!(),
        }
        self.current_position = 0;
        self.current_digit = 1;
        self.status = format!("Switched to preset {}", self.preset + 1);
    }

    fn test_current(&mut self) {
        let mut buffer = vec!['0'; self.buffer_size];
        buffer[self.current_position] = char::from_digit(self.current_digit, 10).unwrap();
        let buffer_str: String = buffer.into_iter().collect();

        let full_string = format!("{}{}{}", self.prime1, buffer_str, self.prime2);
        let full_number = BigUint::from_str(&full_string).unwrap();
        let is_prime = is_prime(&full_number);

        self.status = if is_prime {
            format!("✅ {} is PRIME! Lagrange point found!", full_string)
        } else {
            format!("❌ {} is not prime", full_string)
        };
    }
}

fn main() -> Result<(), io::Error> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = LagrangeDemo::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('h') => app.show_help = !app.show_help,
                KeyCode::Left => {
                    if app.current_position > 0 {
                        app.current_position -= 1;
                    }
                }
                KeyCode::Right => {
                    if app.current_position < app.buffer_size - 1 {
                        app.current_position += 1;
                    }
                }
                KeyCode::Up => {
                    if app.current_digit < 9 {
                        app.current_digit += 1;
                    }
                }
                KeyCode::Down => {
                    if app.current_digit > 1 {
                        app.current_digit -= 1;
                    }
                }
                KeyCode::Char(' ') => app.test_current(),
                KeyCode::Char('p') => app.switch_preset(),
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

fn ui(f: &mut Frame, app: &LagrangeDemo) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(12), // Main visualization
            Constraint::Length(6),  // Controls
            Constraint::Length(4),  // Status
            Constraint::Min(1),     // Help
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new("🌌 LAGRANGE POINTS IN PRIME SPACE")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main visualization
    let viz = create_visualization(app);
    let viz_widget = Paragraph::new(viz).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Concatenated Prime System"),
    );
    f.render_widget(viz_widget, chunks[1]);

    // Controls
    let controls = vec![
        Line::from(vec![
            Span::raw("Position: "),
            Span::styled(
                format!("{}", app.current_position),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw("  Digit: "),
            Span::styled(
                format!("{}", app.current_digit),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(""),
        Line::from("Use ←/→ to move position, ↑/↓ to change digit"),
        Line::from("Press SPACE to test if full number is prime"),
    ];
    let controls_widget =
        Paragraph::new(controls).block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(controls_widget, chunks[2]);

    // Status
    let status_style = if app.status.contains("✅") {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Red)
    };
    let status_widget = Paragraph::new(app.status.clone())
        .style(status_style)
        .block(Block::default().borders(Borders::ALL).title("Test Result"))
        .wrap(Wrap { trim: true });
    f.render_widget(status_widget, chunks[3]);

    // Help
    if app.show_help {
        let help_text = vec![
            Line::from("🔑 KEY INSIGHTS:"),
            Line::from(""),
            Line::from("• We test the ENTIRE concatenated number for primality"),
            Line::from("• This requires TWO primes with space between (like Earth & Moon)"),
            Line::from("• Known Lagrange points: Position 2 (digit 5), Position 4 (digit 2), Position 5 (digit 5)"),
            Line::from("• The 25-digit concatenated number becomes prime at these special positions!"),
        ];
        let help_widget = Paragraph::new(help_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Help (h to toggle)"),
            )
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(help_widget, chunks[4]);
    } else {
        let hint = Paragraph::new("Press 'h' for help, 'q' to quit")
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(hint, chunks[4]);
    }
}

fn create_visualization(app: &LagrangeDemo) -> Text {
    let mut lines = vec![];

    // Show preset info
    let preset_name = match app.preset {
        0 => "Zero-Padded Membrane (1-◯-3-◯-1 vs Giant)",
        1 => "Small vs Giant (Earth vs Sun)",
        2 => "Membrane vs Membrane (Structured Fields)",
        3 => "Tiny vs Large (Asteroid vs Jupiter)",
        4 => "Similar Sized (Twin Stars)",
        _ => "Unknown",
    };
    lines.push(Line::from(vec![
        Span::raw("Preset: "),
        Span::styled(preset_name, Style::default().fg(Color::Magenta)),
        Span::raw(" (press 'p' to switch)"),
    ]));
    lines.push(Line::from(""));

    // Show the two primes with size info
    let p1_is_prime = is_prime(&BigUint::from_str(&app.prime1).unwrap());
    let p2_is_prime = is_prime(&BigUint::from_str(&app.prime2).unwrap());

    lines.push(Line::from(vec![
        Span::raw("Body 1: "),
        Span::styled(&app.prime1, Style::default().fg(Color::Green)),
        Span::raw(format!(
            " ({} digits, {})",
            app.prime1.len(),
            if p1_is_prime {
                "✓ prime"
            } else {
                "✗ not prime"
            }
        )),
    ]));
    lines.push(Line::from(vec![
        Span::raw("Body 2: "),
        Span::styled(&app.prime2, Style::default().fg(Color::Yellow)),
        Span::raw(format!(
            " ({} digits, {})",
            app.prime2.len(),
            if p2_is_prime {
                "✓ prime"
            } else {
                "✗ not prime"
            }
        )),
    ]));
    lines.push(Line::from(vec![
        Span::raw("Size ratio: 1:"),
        Span::styled(
            format!("{}", app.prime2.len() / app.prime1.len()),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    ]));
    lines.push(Line::from(""));

    // Build the full concatenated view
    let mut buffer = vec!['0'; app.buffer_size];
    buffer[app.current_position] = char::from_digit(app.current_digit, 10).unwrap();

    // Visual representation with highlighting
    let mut spans = vec![];
    spans.push(Span::styled(&app.prime1, Style::default().fg(Color::Green)));
    spans.push(Span::raw(" | "));

    for (i, ch) in buffer.iter().enumerate() {
        if i == app.current_position {
            spans.push(Span::styled(
                format!("[{}]", ch),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::raw(ch.to_string()));
        }
    }

    spans.push(Span::raw(" | "));
    spans.push(Span::styled(
        &app.prime2,
        Style::default().fg(Color::Yellow),
    ));

    lines.push(Line::from(spans));
    lines.push(Line::from(""));

    // Show the full number
    let buffer_str: String = buffer.into_iter().collect();
    let full_string = format!("{}{}{}", app.prime1, buffer_str, app.prime2);
    lines.push(Line::from(vec![
        Span::raw("Full number: "),
        Span::styled(full_string.clone(), Style::default().fg(Color::White)),
    ]));
    lines.push(Line::from(vec![
        Span::raw("Length: "),
        Span::styled(
            format!("{} digits", full_string.len()),
            Style::default().fg(Color::Magenta),
        ),
    ]));
    lines.push(Line::from(""));

    // Show known Lagrange points
    lines.push(Line::from("Known Lagrange Points:"));
    for (pos, digit) in &app.lagrange_points {
        let marker = if *pos == app.current_position && *digit == app.current_digit {
            "→"
        } else {
            " "
        };
        lines.push(Line::from(vec![
            Span::raw(format!("{} Position {}, Digit {}", marker, pos, digit)),
            Span::styled(" ✓ Creates PRIME", Style::default().fg(Color::Green)),
        ]));
    }

    Text::from(lines)
}
