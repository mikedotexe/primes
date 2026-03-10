//! Prime Discovery Dashboard - Real-time Physics Visualization
//! 
//! A comprehensive dashboard showing live prime generation, Lagrange point analysis,
//! and prime particle physics in real-time. The ultimate research tool for exploring
//! membrane prime construction with visual feedback.

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Sparkline, Tabs, Wrap,
    },
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
    collections::VecDeque,
};
use primes::{
    membrane::{MembraneConfig, MembraneBuilder},
    PrimeUniverse,
    is_prime,
};
use num_bigint::BigUint;

#[derive(Debug, Clone)]
struct GenerationStats {
    config: MembraneConfig,
    total_generated: usize,
    total_primes: usize,
    generation_times: VecDeque<Duration>,
    success_rates: VecDeque<f64>,
}

impl GenerationStats {
    fn new(config: MembraneConfig) -> Self {
        Self {
            config,
            total_generated: 0,
            total_primes: 0,
            generation_times: VecDeque::with_capacity(100),
            success_rates: VecDeque::with_capacity(100),
        }
    }
    
    fn record_generation(&mut self, is_prime: bool, duration: Duration) {
        self.total_generated += 1;
        if is_prime {
            self.total_primes += 1;
        }
        
        // Keep last 100 generation times
        if self.generation_times.len() >= 100 {
            self.generation_times.pop_front();
        }
        self.generation_times.push_back(duration);
        
        // Update success rate
        let current_rate = if self.total_generated > 0 {
            (self.total_primes as f64 / self.total_generated as f64) * 100.0
        } else {
            0.0
        };
        
        if self.success_rates.len() >= 100 {
            self.success_rates.pop_front();
        }
        self.success_rates.push_back(current_rate);
    }
    
    fn success_rate(&self) -> f64 {
        if self.total_generated == 0 {
            0.0
        } else {
            (self.total_primes as f64 / self.total_generated as f64) * 100.0
        }
    }
    
    fn avg_generation_time(&self) -> Duration {
        if self.generation_times.is_empty() {
            Duration::from_millis(0)
        } else {
            let total: Duration = self.generation_times.iter().sum();
            total / self.generation_times.len() as u32
        }
    }
}

#[derive(Debug)]
struct DashboardState {
    // Generation state
    current_config: MembraneConfig,
    stats: GenerationStats,
    universe: PrimeUniverse,
    
    // UI state
    tab_index: usize,
    paused: bool,
    auto_generate: bool,
    selected_param: usize,
    
    // Real-time data
    recent_primes: VecDeque<(BigUint, Instant)>,
    session_start: Instant,
    last_update: Instant,
    
    // Visual data for sparklines
    sparkline_data: Vec<u64>,
}

impl Default for DashboardState {
    fn default() -> Self {
        let config = MembraneConfig::new(6, 1, 5, 0, 0);
        Self {
            current_config: config.clone(),
            stats: GenerationStats::new(config),
            universe: PrimeUniverse::new(),
            tab_index: 0,
            paused: false,
            auto_generate: true,
            selected_param: 0,
            recent_primes: VecDeque::with_capacity(10),
            session_start: Instant::now(),
            last_update: Instant::now(),
            sparkline_data: Vec::with_capacity(50),
        }
    }
}

impl DashboardState {
    fn generate_next(&mut self) {
        let start = Instant::now();
        let seed = self.stats.total_generated.to_string();
        
        if let Ok(number) = MembraneBuilder::new(self.current_config.clone())
            .with_seed(seed)
            .build_number() {
            
            let is_prime = is_prime(&number);
            let duration = start.elapsed();
            
            self.stats.record_generation(is_prime, duration);
            
            if is_prime {
                if self.recent_primes.len() >= 10 {
                    self.recent_primes.pop_front();
                }
                self.recent_primes.push_back((number.clone(), Instant::now()));
                
                // Add to sparkline data
                if self.sparkline_data.len() >= 50 {
                    self.sparkline_data.remove(0);
                }
                self.sparkline_data.push(1);
            } else {
                // Add zero to sparkline for non-prime
                if self.sparkline_data.len() >= 50 {
                    self.sparkline_data.remove(0);
                }
                self.sparkline_data.push(0);
            }
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
    let mut state = DashboardState::default();
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();
    
    loop {
        // Draw UI
        terminal.draw(|f| draw_ui(f, &mut state))?;
        
        // Handle input
        if crossterm::event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => state.paused = !state.paused,
                    KeyCode::Char('a') => state.auto_generate = !state.auto_generate,
                    KeyCode::Char('g') => {
                        if !state.auto_generate {
                            state.generate_next();
                        }
                    }
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
        
        // Auto-generate if enabled
        if state.auto_generate && !state.paused && last_tick.elapsed() >= tick_rate {
            state.generate_next();
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

fn draw_ui(f: &mut Frame, state: &mut DashboardState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(15),    // Main content
            Constraint::Length(3),  // Status bar
        ])
        .split(f.area());
    
    // Header
    draw_header(f, state, chunks[0]);
    
    // Main content with tabs
    let tabs = vec!["Overview", "Statistics", "Configuration"];
    let tab_widget = Tabs::new(tabs)
        .block(Block::default().borders(Borders::ALL).title("Views"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(state.tab_index);
    
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(chunks[1]);
    
    f.render_widget(tab_widget, main_chunks[0]);
    
    match state.tab_index {
        0 => draw_overview_tab(f, state, main_chunks[1]),
        1 => draw_statistics_tab(f, state, main_chunks[1]),
        2 => draw_configuration_tab(f, state, main_chunks[1]),
        _ => {}
    }
    
    // Status bar
    draw_status_bar(f, state, chunks[2]);
}

fn draw_header(f: &mut Frame, state: &DashboardState, area: Rect) {
    let runtime = state.session_start.elapsed();
    let header = Paragraph::new(Line::from(vec![
        Span::styled("🌌 ", Style::default().fg(Color::Magenta)),
        Span::styled("Prime Discovery Dashboard", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" | Runtime: "),
        Span::styled(format!("{:.1}s", runtime.as_secs_f64()), Style::default().fg(Color::Yellow)),
        Span::raw(" | "),
        if state.paused {
            Span::styled("PAUSED", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        } else {
            Span::styled("RUNNING", Style::default().fg(Color::Green))
        },
    ]))
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center);
    f.render_widget(header, area);
}

fn draw_overview_tab(f: &mut Frame, state: &DashboardState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    
    // Left side - Real-time stats
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(5)])
        .split(chunks[0]);
    
    // Success rate gauge
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Success Rate"))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(state.stats.success_rate() as u16)
        .label(format!("{:.1}%", state.stats.success_rate()));
    f.render_widget(gauge, left_chunks[0]);
    
    // Recent primes list
    let primes: Vec<ListItem> = state.recent_primes
        .iter()
        .map(|(prime, _)| {
            ListItem::new(Line::from(vec![
                Span::raw("✓ "),
                Span::styled(prime.to_string(), Style::default().fg(Color::Green)),
            ]))
        })
        .collect();
    
    let list = List::new(primes)
        .block(Block::default().borders(Borders::ALL).title("Recent Primes"));
    f.render_widget(list, left_chunks[1]);
    
    // Right side - Sparkline
    let sparkline_data: Vec<u64> = state.sparkline_data.clone();
    let sparkline = Sparkline::default()
        .block(Block::default().borders(Borders::ALL).title("Prime Generation Pattern"))
        .data(&sparkline_data)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(sparkline, chunks[1]);
}

fn draw_statistics_tab(f: &mut Frame, state: &DashboardState, area: Rect) {
    let stats_text = vec![
        Line::from(vec![
            Span::raw("Total Generated: "),
            Span::styled(state.stats.total_generated.to_string(), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::raw("Total Primes: "),
            Span::styled(state.stats.total_primes.to_string(), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::raw("Success Rate: "),
            Span::styled(format!("{:.2}%", state.stats.success_rate()), Style::default().fg(Color::Magenta)),
        ]),
        Line::from(vec![
            Span::raw("Avg Generation Time: "),
            Span::styled(
                format!("{:.1}ms", state.stats.avg_generation_time().as_secs_f64() * 1000.0),
                Style::default().fg(Color::Cyan)
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Current Configuration: "),
            Span::styled(
                format!("Base {} ({},{}) k=({},{})", 
                    state.current_config.base,
                    state.current_config.outer,
                    state.current_config.inner,
                    state.current_config.k_outer,
                    state.current_config.k_inner
                ),
                Style::default().fg(Color::Blue)
            ),
        ]),
    ];
    
    let paragraph = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title("Statistics"))
        .wrap(Wrap { trim: true });
    
    f.render_widget(paragraph, area);
}

fn draw_configuration_tab(f: &mut Frame, state: &DashboardState, area: Rect) {
    let params = vec![
        ("Base", state.current_config.base),
        ("Outer", state.current_config.outer as u32),
        ("Inner", state.current_config.inner as u32),
        ("K Outer", state.current_config.k_outer as u32),
        ("K Inner", state.current_config.k_inner as u32),
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
        .block(Block::default().borders(Borders::ALL).title("Configuration (↑↓ to select, ←→ to adjust)"));
    
    f.render_widget(list, area);
}

fn draw_status_bar(f: &mut Frame, state: &DashboardState, area: Rect) {
    let status = Paragraph::new(Line::from(vec![
        Span::raw("Space: "),
        Span::styled("Pause", Style::default().fg(Color::Cyan)),
        Span::raw(" | Tab: "),
        Span::styled("Switch View", Style::default().fg(Color::Cyan)),
        Span::raw(" | a: "),
        Span::styled("Auto-generate", Style::default().fg(Color::Cyan)),
        Span::raw(" | g: "),
        Span::styled("Generate One", Style::default().fg(Color::Cyan)),
        Span::raw(" | q: "),
        Span::styled("Quit", Style::default().fg(Color::Cyan)),
    ]))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, area);
}

fn adjust_parameter(state: &mut DashboardState, delta: i32) {
    match state.selected_param {
        0 => {
            state.current_config.base = (state.current_config.base as i32 + delta).max(2).min(30) as u32;
            state.stats = GenerationStats::new(state.current_config.clone());
        }
        1 => {
            state.current_config.outer = (state.current_config.outer as i32 + delta).max(1).min(9) as u8;
            state.stats = GenerationStats::new(state.current_config.clone());
        }
        2 => {
            state.current_config.inner = (state.current_config.inner as i32 + delta).max(1).min(9) as u8;
            state.stats = GenerationStats::new(state.current_config.clone());
        }
        3 => {
            state.current_config.k_outer = (state.current_config.k_outer as i32 + delta).max(0).min(5) as u8;
            state.stats = GenerationStats::new(state.current_config.clone());
        }
        4 => {
            state.current_config.k_inner = (state.current_config.k_inner as i32 + delta).max(0).min(5) as u8;
            state.stats = GenerationStats::new(state.current_config.clone());
        }
        _ => {}
    }
}