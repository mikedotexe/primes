//! Interactive Membrane Laboratory - TUI Interface
//! 
//! A real-time, interactive terminal interface for exploring membrane prime generation.
//! Researchers can adjust parameters and see immediate results with visual feedback.

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Tabs, Wrap,
    },
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
    fs::{OpenOptions, create_dir_all},
    path::PathBuf,
    io::Write,
};
use primes::{
    is_prime,
    membrane::{MembraneConfig, MembraneBuilder},
};
use num_bigint::BigUint;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
struct GenerationResult {
    number: BigUint,
    is_prime: bool,
    seed: String,
    generation_time: Duration,
    wolfram_url: String,
}

#[derive(Debug)]
struct LabState {
    // Configuration parameters
    base: u32,
    outer: u32,
    inner: u32,
    k_outer: u32,
    k_inner: u32,
    
    // UI state
    selected_param: usize,
    tab_index: usize,
    current_results: Vec<GenerationResult>,
    is_generating: bool,
    generation_progress: f64,
    
    // Statistics
    total_generated: usize,
    total_primes: usize,
    session_start: Instant,
    
    // Seeds to test
    test_seeds: Vec<String>,
    current_seed_index: usize,
    
    // Visual feedback
    show_help: bool,
    last_generation_success: bool,
    flash_timer: Instant,
}

impl Default for LabState {
    fn default() -> Self {
        Self {
            base: 6,
            outer: 1,
            inner: 5,
            k_outer: 0,
            k_inner: 0,
            selected_param: 0,
            tab_index: 0,
            current_results: Vec::new(),
            is_generating: false,
            generation_progress: 0.0,
            total_generated: 0,
            total_primes: 0,
            session_start: Instant::now(),
            test_seeds: vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "5".to_string(),
            ],
            current_seed_index: 0,
            show_help: false,
            last_generation_success: false,
            flash_timer: Instant::now(),
        }
    }
}

impl LabState {
    fn generate_primes(&mut self) {
        self.current_results.clear();
        self.is_generating = true;
        self.generation_progress = 0.0;
        
        for (i, seed) in self.test_seeds.iter().enumerate() {
            let start = Instant::now();
            
            let config = MembraneConfig::new(
                self.base,
                self.outer as u8,
                self.inner as u8,
                self.k_outer as u8,
                self.k_inner as u8,
            );
            
            if let Ok(number) = MembraneBuilder::new(config)
                .with_seed(seed.clone())
                .build_number() {
                
                let is_prime = is_prime(&number);
                let generation_time = start.elapsed();
                
                let wolfram_url = format!(
                    "https://www.wolframalpha.com/input?i=isPrime%28{}%29",
                    number
                );
                
                self.current_results.push(GenerationResult {
                    number,
                    is_prime,
                    seed: seed.clone(),
                    generation_time,
                    wolfram_url,
                });
                
                self.total_generated += 1;
                if is_prime {
                    self.total_primes += 1;
                    self.last_generation_success = true;
                    self.flash_timer = Instant::now();
                }
            }
            
            self.generation_progress = (i + 1) as f64 / self.test_seeds.len() as f64;
        }
        
        self.is_generating = false;
    }
    
    fn success_rate(&self) -> f64 {
        if self.total_generated == 0 {
            0.0
        } else {
            (self.total_primes as f64 / self.total_generated as f64) * 100.0
        }
    }
}

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Create app state
    let mut state = LabState::default();
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(100);
    
    loop {
        // Draw UI
        terminal.draw(|f| draw_ui(f, &mut state))?;
        
        // Handle input
        if crossterm::event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => state.generate_primes(),
                    KeyCode::Char('h') => state.show_help = !state.show_help,
                    KeyCode::Tab => state.tab_index = (state.tab_index + 1) % 3,
                    KeyCode::Up => {
                        if state.selected_param > 0 {
                            state.selected_param -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if state.selected_param < 4 {
                            state.selected_param += 1;
                        }
                    }
                    KeyCode::Left => adjust_parameter(&mut state, -1),
                    KeyCode::Right => adjust_parameter(&mut state, 1),
                    _ => {}
                }
            }
        }
        
        // Update tick
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
    
    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;
    
    Ok(())
}

fn draw_ui(f: &mut Frame, state: &mut LabState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());
    
    // Header
    let header = Paragraph::new(Line::from(vec![
        Span::styled("🧪 ", Style::default().fg(Color::Cyan)),
        Span::styled("Membrane Laboratory", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" - "),
        Span::styled("Interactive Prime Generator", Style::default().fg(Color::Gray)),
    ]))
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);
    
    // Main content area with tabs
    let tab_titles = vec!["Parameters", "Results", "Statistics"];
    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("Navigation"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(state.tab_index);
    
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(chunks[1]);
    
    f.render_widget(tabs, main_chunks[0]);
    
    match state.tab_index {
        0 => draw_parameters_tab(f, state, main_chunks[1]),
        1 => draw_results_tab(f, state, main_chunks[1]),
        2 => draw_statistics_tab(f, state, main_chunks[1]),
        _ => {}
    }
    
    // Status bar
    let status_style = if state.flash_timer.elapsed() < Duration::from_millis(500) && state.last_generation_success {
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };
    
    let status = Paragraph::new(Line::from(vec![
        Span::raw("Press "),
        Span::styled("Space", Style::default().fg(Color::Cyan)),
        Span::raw(" to generate | "),
        Span::styled("Tab", Style::default().fg(Color::Cyan)),
        Span::raw(" to switch tabs | "),
        Span::styled("q", Style::default().fg(Color::Cyan)),
        Span::raw(" to quit | Success Rate: "),
        Span::styled(format!("{:.1}%", state.success_rate()), status_style),
    ]))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[2]);
}

fn draw_parameters_tab(f: &mut Frame, state: &mut LabState, area: Rect) {
    let params = vec![
        ("Base", state.base),
        ("Outer", state.outer),
        ("Inner", state.inner),
        ("K Outer", state.k_outer),
        ("K Inner", state.k_inner),
    ];
    
    let items: Vec<ListItem> = params
        .iter()
        .enumerate()
        .map(|(i, (name, value))| {
            let style = if i == state.selected_param {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{:10}", name), style),
                Span::raw(": "),
                Span::styled(value.to_string(), style),
            ]))
        })
        .collect();
    
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Parameters"))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    
    f.render_widget(list, area);
}

fn draw_results_tab(f: &mut Frame, state: &mut LabState, area: Rect) {
    let items: Vec<ListItem> = state.current_results
        .iter()
        .map(|result| {
            let style = if result.is_prime {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Red)
            };
            ListItem::new(Line::from(vec![
                Span::raw("Seed "),
                Span::styled(&result.seed, Style::default().fg(Color::Cyan)),
                Span::raw(" → "),
                Span::styled(result.number.to_string(), style),
                Span::raw(format!(" ({:.1}ms)", result.generation_time.as_secs_f64() * 1000.0)),
            ]))
        })
        .collect();
    
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Generation Results"));
    
    f.render_widget(list, area);
}

fn draw_statistics_tab(f: &mut Frame, state: &mut LabState, area: Rect) {
    let runtime = state.session_start.elapsed();
    let stats_text = vec![
        Line::from(vec![
            Span::raw("Session Runtime: "),
            Span::styled(format!("{:.1}s", runtime.as_secs_f64()), Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::raw("Total Generated: "),
            Span::styled(state.total_generated.to_string(), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::raw("Total Primes: "),
            Span::styled(state.total_primes.to_string(), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::raw("Success Rate: "),
            Span::styled(format!("{:.1}%", state.success_rate()), Style::default().fg(Color::Magenta)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Current Config: "),
            Span::styled(
                format!("({},{}) k=({},{})", state.outer, state.inner, state.k_outer, state.k_inner),
                Style::default().fg(Color::Blue)
            ),
        ]),
    ];
    
    let paragraph = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title("Statistics"))
        .wrap(Wrap { trim: true });
    
    f.render_widget(paragraph, area);
}

fn adjust_parameter(state: &mut LabState, delta: i32) {
    match state.selected_param {
        0 => state.base = (state.base as i32 + delta).max(2).min(30) as u32,
        1 => state.outer = (state.outer as i32 + delta).max(1).min(9) as u32,
        2 => state.inner = (state.inner as i32 + delta).max(1).min(9) as u32,
        3 => state.k_outer = (state.k_outer as i32 + delta).max(0).min(5) as u32,
        4 => state.k_inner = (state.k_inner as i32 + delta).max(0).min(5) as u32,
        _ => {}
    }
}