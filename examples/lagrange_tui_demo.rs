//! Prime Connector Hunter - Research-Grade TUI
//!
//! Explore "prime connectors" C such that concat(prime1, C, prime2) is prime.
//!
//! Example: 10301 + [00006] + 3007003007003 = 10301000063007003007003 (prime!)
//!
//! Features:
//! - Automated exhaustive search across buffer sizes
//! - Pattern detection and statistical analysis
//! - Grid visualization with heatmaps
//! - CSV/Markdown export
//! - Session persistence
//! - DOS orange aesthetic

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
    text::{Line, Span, Text},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame, Terminal,
};
use std::collections::HashMap;
use std::io::{self, Write as _};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// ========================================
// CONSTANTS: DOS Orange Theme
// ========================================

const DOS_ORANGE: Color = Color::Rgb(255, 165, 0);
const DOS_AMBER: Color = Color::Rgb(255, 191, 0);
const DOS_DARK: Color = Color::Rgb(139, 69, 19);
const DOS_BLACK: Color = Color::Black;
const DOS_WHITE: Color = Color::Rgb(255, 255, 255);

// ========================================
// DATA STRUCTURES
// ========================================

#[derive(Clone, Debug)]
struct PresetInfo {
    name: String,
    prime1: String,
    prime2: String,
    category: PresetCategory,
    recommended_buffers: Vec<usize>,
    known_connectors: Vec<String>,
    _description: String,
}

#[derive(Clone, Debug, PartialEq)]
enum PresetCategory {
    Membrane,
    Twin,
    SophieGermain,
    Asymmetric,
}

#[derive(Clone, Debug)]
struct Connector {
    buffer_size: usize,
    position: usize,
    digit: u8,
    pattern: String,
    full_number: String,
    discovered_at: Instant,
}

#[derive(Clone, Debug)]
struct SearchProgress {
    active: bool,
    current_buffer: usize,
    total_buffers: usize,
    current_position: usize,
    total_positions: usize,
    current_digit: u8,
    connectors_found: usize,
    tests_completed: usize,
    start_time: Option<Instant>,
}

impl SearchProgress {
    fn new() -> Self {
        Self {
            active: false,
            current_buffer: 0,
            total_buffers: 0,
            current_position: 0,
            total_positions: 0,
            current_digit: 0,
            connectors_found: 0,
            tests_completed: 0,
            start_time: None,
        }
    }

    fn progress_percent(&self) -> u16 {
        if self.total_buffers == 0 || self.total_positions == 0 {
            return 0;
        }
        let total_tests = self.total_buffers * self.total_positions * 9;
        let completed = self.tests_completed;
        ((completed as f64 / total_tests as f64) * 100.0).min(100.0) as u16
    }

    fn status_text(&self) -> String {
        if !self.active {
            return "Search idle - Press 'S' to start automated hunt".to_string();
        }
        format!(
            "Hunting: Buffer {}/{} | Pos {}/{} | Digit {} | Found: {} | {}%",
            self.current_buffer,
            self.total_buffers,
            self.current_position,
            self.total_positions,
            self.current_digit,
            self.connectors_found,
            self.progress_percent()
        )
    }
}

#[derive(Clone, Debug)]
struct PatternAnalysis {
    position_histogram: HashMap<usize, usize>,
    digit_histogram: HashMap<u8, usize>,
    buffer_histogram: HashMap<usize, usize>,
    _symmetric_pairs: Vec<(usize, usize)>,
    resonance_positions: Vec<usize>,
    insights: Vec<String>,
}

impl PatternAnalysis {
    fn new() -> Self {
        Self {
            position_histogram: HashMap::new(),
            digit_histogram: HashMap::new(),
            buffer_histogram: HashMap::new(),
            _symmetric_pairs: Vec::new(),
            resonance_positions: Vec::new(),
            insights: Vec::new(),
        }
    }

    fn analyze(&mut self, connectors: &[Connector]) {
        self.position_histogram.clear();
        self.digit_histogram.clear();
        self.buffer_histogram.clear();
        self.insights.clear();

        for c in connectors {
            *self.position_histogram.entry(c.position).or_insert(0) += 1;
            *self.digit_histogram.entry(c.digit).or_insert(0) += 1;
            *self.buffer_histogram.entry(c.buffer_size).or_insert(0) += 1;
        }

        // Find resonance positions (multiple working digits)
        self.resonance_positions = self
            .position_histogram
            .iter()
            .filter(|(_, &count)| count > 1)
            .map(|(&pos, _)| pos)
            .collect();

        // Generate insights
        if let Some((&most_common_pos, &count)) = self
            .position_histogram
            .iter()
            .max_by_key(|(_, &count)| count)
        {
            if count > 1 {
                self.insights.push(format!(
                    "Clustering detected: Position {} has {} connectors",
                    most_common_pos, count
                ));
            }
        }

        if let Some((&most_common_digit, &count)) =
            self.digit_histogram.iter().max_by_key(|(_, &count)| count)
        {
            if count > 1 {
                self.insights.push(format!(
                    "Digit preference: {} appears in {} connectors",
                    most_common_digit, count
                ));
            }
        }

        if !self.resonance_positions.is_empty() {
            self.insights.push(format!(
                "Resonance positions: {:?} (multiple working digits)",
                self.resonance_positions
            ));
        }
    }
}

#[derive(PartialEq)]
enum AppMode {
    Browse,
    Hunt,
    Results,
}

struct ConnectorHunterApp {
    // Core state
    mode: AppMode,
    preset_index: usize,
    presets: Vec<PresetInfo>,

    // Manual browse mode
    buffer: Vec<u8>,
    cursor: usize,
    last_test_result: Option<bool>,

    // Search state
    search_progress: Arc<Mutex<SearchProgress>>,
    search_handle: Option<thread::JoinHandle<Vec<Connector>>>,

    // Results
    connectors: Vec<Connector>,
    pattern_analysis: PatternAnalysis,

    // UI state
    status: String,
    show_help: bool,
    high_contrast: bool,

    // Grid visualization
    grid_view: HashMap<(usize, usize, u8), bool>, // (buffer, position, digit) -> is_prime
    _grid_buffer_index: usize,

    // Session
    session_start: Instant,
}

impl ConnectorHunterApp {
    fn new() -> Self {
        let presets = create_preset_library();
        let mut app = Self {
            mode: AppMode::Browse,
            preset_index: 0,
            presets,
            buffer: vec![0; 7],
            cursor: 0,
            last_test_result: None,
            search_progress: Arc::new(Mutex::new(SearchProgress::new())),
            search_handle: None,
            connectors: Vec::new(),
            pattern_analysis: PatternAnalysis::new(),
            status: "Welcome! Press 'h' for help, 'S' to start automated hunt".to_string(),
            show_help: false,
            high_contrast: false,
            grid_view: HashMap::new(),
            _grid_buffer_index: 0,
            session_start: Instant::now(),
        };
        app.apply_preset(0);
        app
    }

    fn current_preset(&self) -> &PresetInfo {
        &self.presets[self.preset_index]
    }

    fn apply_preset(&mut self, index: usize) {
        self.preset_index = index % self.presets.len();

        // Get default size before borrowing
        let default_size = self
            .current_preset()
            .recommended_buffers
            .first()
            .copied()
            .unwrap_or(7);
        let preset_name = self.current_preset().name.clone();

        // Reset buffer to recommended size
        self.buffer = vec![0; default_size];
        self.cursor = 0;

        // Clear results
        self.connectors.clear();
        self.pattern_analysis = PatternAnalysis::new();
        self.grid_view.clear();

        self.status = format!("Loaded preset: {}", preset_name);
    }

    fn next_preset(&mut self) {
        self.apply_preset(self.preset_index + 1);
    }

    fn prev_preset(&mut self) {
        let prev = if self.preset_index == 0 {
            self.presets.len() - 1
        } else {
            self.preset_index - 1
        };
        self.apply_preset(prev);
    }

    fn connector_string(&self) -> String {
        self.buffer.iter().map(|d| char::from(b'0' + *d)).collect()
    }

    fn full_number_string(&self) -> String {
        let preset = self.current_preset();
        format!(
            "{}{}{}",
            preset.prime1,
            self.connector_string(),
            preset.prime2
        )
    }

    fn test_current(&mut self) {
        let connector = self.connector_string();
        let full = self.full_number_string();

        match BigUint::from_str(&full) {
            Ok(n) => {
                let prime = is_prime(&n);
                self.last_test_result = Some(prime);
                if prime {
                    let new_connector = Connector {
                        buffer_size: self.buffer.len(),
                        position: self.cursor,
                        digit: self.buffer[self.cursor],
                        pattern: connector.clone(),
                        full_number: full.clone(),
                        discovered_at: Instant::now(),
                    };

                    // Add if not duplicate
                    if !self.connectors.iter().any(|c| c.pattern == connector) {
                        self.connectors.push(new_connector);
                        self.pattern_analysis.analyze(&self.connectors);
                    }

                    self.status = format!("✓ PRIME: {}", full);
                } else {
                    self.status = format!("✗ Composite: {}", full);
                }
            }
            Err(_) => {
                self.last_test_result = None;
                self.status = "Error: Could not parse number".to_string();
            }
        }
    }

    fn start_exhaustive_search(&mut self, min_buffer: usize, max_buffer: usize) {
        // Cancel existing search if active
        if self.search_handle.is_some() {
            self.cancel_search();
        }

        let preset = self.current_preset().clone();
        let progress = Arc::clone(&self.search_progress);

        // Initialize progress
        {
            let mut prog = progress.lock().unwrap();
            prog.active = true;
            prog.current_buffer = min_buffer;
            prog.total_buffers = max_buffer - min_buffer + 1;
            prog.current_position = 0;
            prog.total_positions = max_buffer; // Will be updated per buffer
            prog.current_digit = 1;
            prog.connectors_found = 0;
            prog.tests_completed = 0;
            prog.start_time = Some(Instant::now());
        }

        // Spawn search thread
        let handle = thread::spawn(move || {
            let mut found = Vec::new();

            for buffer_size in min_buffer..=max_buffer {
                for position in 0..buffer_size {
                    for digit in 1..=9u8 {
                        // Update progress
                        {
                            let mut prog = progress.lock().unwrap();
                            prog.current_buffer = buffer_size - min_buffer + 1;
                            prog.current_position = position;
                            prog.current_digit = digit;
                            prog.total_positions = buffer_size;
                            prog.tests_completed += 1;
                        }

                        // Build connector
                        let mut buffer = vec![0u8; buffer_size];
                        buffer[position] = digit;
                        let connector: String =
                            buffer.iter().map(|d| char::from(b'0' + *d)).collect();
                        let full = format!("{}{}{}", preset.prime1, connector, preset.prime2);

                        // Test primality
                        if let Ok(n) = BigUint::from_str(&full) {
                            if is_prime(&n) {
                                let new_connector = Connector {
                                    buffer_size,
                                    position,
                                    digit,
                                    pattern: connector.clone(),
                                    full_number: full.clone(),
                                    discovered_at: Instant::now(),
                                };

                                found.push(new_connector);

                                // Update counter
                                {
                                    let mut prog = progress.lock().unwrap();
                                    prog.connectors_found = found.len();
                                }
                            }
                        }

                        // Small delay to keep UI responsive
                        thread::sleep(Duration::from_micros(100));
                    }
                }
            }

            // Mark as complete
            {
                let mut prog = progress.lock().unwrap();
                prog.active = false;
            }

            found
        });

        self.search_handle = Some(handle);
        self.mode = AppMode::Hunt;
        self.status = "Exhaustive search started! Press 'C' to cancel".to_string();
    }

    fn cancel_search(&mut self) {
        // Note: Can't actually cancel thread, but we can mark it inactive
        {
            let mut prog = self.search_progress.lock().unwrap();
            prog.active = false;
        }
        self.search_handle = None;
        self.status = "Search cancelled".to_string();
    }

    fn check_search_completion(&mut self) {
        if let Some(handle) = self.search_handle.take() {
            if handle.is_finished() {
                if let Ok(found) = handle.join() {
                    // Merge results
                    for connector in found {
                        if !self
                            .connectors
                            .iter()
                            .any(|c| c.pattern == connector.pattern)
                        {
                            self.connectors.push(connector);
                        }
                    }

                    // Update analysis
                    self.pattern_analysis.analyze(&self.connectors);

                    // Build grid view
                    self.populate_grid_view();

                    self.mode = AppMode::Results;
                    self.status = format!(
                        "Search complete! Found {} prime connectors",
                        self.connectors.len()
                    );
                }
            } else {
                // Still running, put it back
                self.search_handle = Some(handle);
            }
        }
    }

    fn populate_grid_view(&mut self) {
        for connector in &self.connectors {
            self.grid_view.insert(
                (connector.buffer_size, connector.position, connector.digit),
                true,
            );
        }
    }

    fn export_csv(&mut self) -> io::Result<()> {
        let filename = format!("connectors_{}.csv", self.current_preset().prime1);

        let mut file = std::fs::File::create(&filename)?;

        // Write header
        writeln!(
            file,
            "timestamp,prime1,prime2,buffer_size,position,digit,connector,full_number"
        )?;

        // Write data
        let preset = self.current_preset();
        for c in &self.connectors {
            writeln!(
                file,
                "{},{},{},{},{},{},{},{}",
                c.discovered_at.elapsed().as_secs(),
                preset.prime1,
                preset.prime2,
                c.buffer_size,
                c.position,
                c.digit,
                c.pattern,
                c.full_number
            )?;
        }

        self.status = format!("Exported to {}", filename);
        Ok(())
    }

    fn export_markdown(&mut self) -> io::Result<()> {
        let filename = format!("connectors_{}.md", self.current_preset().prime1);

        let mut file = std::fs::File::create(&filename)?;
        let preset = self.current_preset();

        writeln!(file, "# Prime Connector Hunt Results\n")?;
        writeln!(
            file,
            "**Duration**: {:.1} seconds\n",
            self.session_start.elapsed().as_secs_f64()
        )?;
        writeln!(file, "**Preset**: {}\n", preset.name)?;
        writeln!(file, "**Category**: {:?}\n", preset.category)?;

        writeln!(file, "\n## Prime Pair\n")?;
        writeln!(file, "- **Prime 1**: {}", preset.prime1)?;
        writeln!(file, "- **Prime 2**: {}\n", preset.prime2)?;

        writeln!(
            file,
            "\n## Discovered Connectors ({} total)\n",
            self.connectors.len()
        )?;

        for (i, c) in self.connectors.iter().enumerate() {
            writeln!(file, "### {}. Connector: `{}`\n", i + 1, c.pattern)?;
            writeln!(file, "```")?;
            writeln!(
                file,
                "{} | {} | {}",
                preset.prime1, c.pattern, preset.prime2
            )?;
            writeln!(file, "```\n")?;
            writeln!(file, "- **Buffer size**: {}", c.buffer_size)?;
            writeln!(file, "- **Position**: {}", c.position)?;
            writeln!(file, "- **Digit**: {}", c.digit)?;
            writeln!(file, "- **Full number**: {}", c.full_number)?;
            writeln!(file, "- **Length**: {} digits\n", c.full_number.len())?;
        }

        // Pattern analysis
        if !self.pattern_analysis.insights.is_empty() {
            writeln!(file, "\n## Pattern Analysis\n")?;
            for insight in &self.pattern_analysis.insights {
                writeln!(file, "- {}", insight)?;
            }
        }

        writeln!(file, "\n---\n")?;
        writeln!(file, "*Generated by Prime Connector Hunter*")?;

        self.status = format!("Exported to {}", filename);
        Ok(())
    }

    // Manual mode controls
    fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.cursor + 1 < self.buffer.len() {
            self.cursor += 1;
        }
    }

    fn increment_digit(&mut self) {
        if self.buffer[self.cursor] < 9 {
            self.buffer[self.cursor] += 1;
        }
    }

    fn decrement_digit(&mut self) {
        if self.buffer[self.cursor] > 0 {
            self.buffer[self.cursor] -= 1;
        }
    }

    fn resize_buffer(&mut self, delta: isize) {
        let new_size = (self.buffer.len() as isize + delta).clamp(1, 64) as usize;
        if new_size > self.buffer.len() {
            self.buffer.resize(new_size, 0);
        } else {
            self.buffer.truncate(new_size);
            if self.cursor >= new_size {
                self.cursor = new_size - 1;
            }
        }
    }
}

// ========================================
// PRESET LIBRARY
// ========================================

fn create_preset_library() -> Vec<PresetInfo> {
    vec![
        PresetInfo {
            name: "The Four Connectors (10301 ∘ 3007003007003)".to_string(),
            prime1: "10301".to_string(),
            prime2: "3007003007003".to_string(),
            category: PresetCategory::Membrane,
            recommended_buffers: vec![5, 6, 7],
            known_connectors: vec![
                "0066600".to_string(),
                "0006000".to_string(),
                "006000".to_string(),
                "00006".to_string(),
            ],
            _description: "Original discovery: 4 known equilibrium connectors".to_string(),
        },
        PresetInfo {
            name: "Zero-Padded Membrane (10301 ∘ 30305070305070303)".to_string(),
            prime1: "10301".to_string(),
            prime2: "30305070305070303".to_string(),
            category: PresetCategory::Membrane,
            recommended_buffers: vec![5, 7],
            known_connectors: vec![],
            _description: "1-0-3-0-1 membrane structure meets giant membrane".to_string(),
        },
        PresetInfo {
            name: "Twin Primes (11 ∘ 13)".to_string(),
            prime1: "11".to_string(),
            prime2: "13".to_string(),
            category: PresetCategory::Twin,
            recommended_buffers: vec![3, 5, 7],
            known_connectors: vec![],
            _description: "Smallest twin prime pair - do connectors exist?".to_string(),
        },
        PresetInfo {
            name: "Twin Primes (41 ∘ 43)".to_string(),
            prime1: "41".to_string(),
            prime2: "43".to_string(),
            category: PresetCategory::Twin,
            recommended_buffers: vec![3, 5, 7],
            known_connectors: vec![],
            _description: "Larger twin pair - testing connector patterns".to_string(),
        },
        PresetInfo {
            name: "Sophie Germain (23 ∘ 47)".to_string(),
            prime1: "23".to_string(),
            prime2: "47".to_string(),
            category: PresetCategory::SophieGermain,
            recommended_buffers: vec![3, 5],
            known_connectors: vec![],
            _description: "23 and 2×23+1=47 both prime".to_string(),
        },
        PresetInfo {
            name: "Sophie Germain (53 ∘ 107)".to_string(),
            prime1: "53".to_string(),
            prime2: "107".to_string(),
            category: PresetCategory::SophieGermain,
            recommended_buffers: vec![3, 5, 7],
            known_connectors: vec![],
            _description: "53 and 2×53+1=107 both prime".to_string(),
        },
        PresetInfo {
            name: "Membrane Pair (151 ∘ 30305070305070303)".to_string(),
            prime1: "151".to_string(),
            prime2: "30305070305070303".to_string(),
            category: PresetCategory::Membrane,
            recommended_buffers: vec![5, 7],
            known_connectors: vec![],
            _description: "1-5-1 membrane meets giant membrane".to_string(),
        },
        PresetInfo {
            name: "Asymmetric (2 ∘ 1000000007)".to_string(),
            prime1: "2".to_string(),
            prime2: "1000000007".to_string(),
            category: PresetCategory::Asymmetric,
            recommended_buffers: vec![3, 5, 7, 10],
            known_connectors: vec![],
            _description: "Smallest prime meets 10-digit prime".to_string(),
        },
        PresetInfo {
            name: "Asymmetric (7 ∘ 3007003007003)".to_string(),
            prime1: "7".to_string(),
            prime2: "3007003007003".to_string(),
            category: PresetCategory::Asymmetric,
            recommended_buffers: vec![5, 7, 10],
            known_connectors: vec![],
            _description: "Single digit meets membrane prime".to_string(),
        },
    ]
}

// ========================================
// MAIN LOOP
// ========================================

fn main() -> Result<(), io::Error> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = ConnectorHunterApp::new();
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        if app.mode == AppMode::Hunt {
                            app.cancel_search();
                        }
                        break;
                    }
                    KeyCode::Char('h') => app.show_help = !app.show_help,
                    KeyCode::Char('a') => app.high_contrast = !app.high_contrast,
                    KeyCode::Char('p') => app.next_preset(),
                    KeyCode::Char('P') => app.prev_preset(),
                    KeyCode::Char('S') => {
                        // Start exhaustive search
                        app.start_exhaustive_search(3, 10);
                    }
                    KeyCode::Char('C') => {
                        if app.mode == AppMode::Hunt {
                            app.cancel_search();
                            app.mode = AppMode::Browse;
                        }
                    }
                    KeyCode::Char('e') => {
                        if let Err(e) = app.export_csv() {
                            app.status = format!("Export failed: {}", e);
                        }
                    }
                    KeyCode::Char('m') => {
                        if let Err(e) = app.export_markdown() {
                            app.status = format!("Export failed: {}", e);
                        }
                    }
                    KeyCode::Char('r') => {
                        app.mode = AppMode::Results;
                    }
                    KeyCode::Char('b') => {
                        app.mode = AppMode::Browse;
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if app.mode == AppMode::Browse {
                            app.test_current();
                        }
                    }
                    KeyCode::Left => {
                        if app.mode == AppMode::Browse {
                            app.move_left();
                        }
                    }
                    KeyCode::Right => {
                        if app.mode == AppMode::Browse {
                            app.move_right();
                        }
                    }
                    KeyCode::Up => {
                        if app.mode == AppMode::Browse {
                            app.increment_digit();
                        }
                    }
                    KeyCode::Down => {
                        if app.mode == AppMode::Browse {
                            app.decrement_digit();
                        }
                    }
                    KeyCode::Char('+') | KeyCode::Char('=') => {
                        if app.mode == AppMode::Browse {
                            app.resize_buffer(1);
                        }
                    }
                    KeyCode::Char('-') | KeyCode::Char('_') => {
                        if app.mode == AppMode::Browse {
                            app.resize_buffer(-1);
                        }
                    }
                    _ => {}
                },
                Event::Mouse(_)
                | Event::Resize(_, _)
                | Event::FocusGained
                | Event::FocusLost
                | Event::Paste(_) => {
                    // Ignore these events
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.check_search_completion();
            last_tick = Instant::now();
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

// ========================================
// UI RENDERING
// ========================================

fn ui(f: &mut Frame, app: &mut ConnectorHunterApp) {
    let (bg, fg) = if app.high_contrast {
        (DOS_BLACK, DOS_WHITE)
    } else {
        (DOS_BLACK, DOS_ORANGE)
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(10), // Main content
            Constraint::Length(4),  // Controls
            Constraint::Length(3),  // Status
            Constraint::Min(5),     // Mode-specific panel
        ])
        .split(f.size());

    // Title
    render_title(f, chunks[0], fg, bg, app);

    // Main content (preset info + current state)
    render_main_content(f, chunks[1], fg, bg, app);

    // Controls
    render_controls(f, chunks[2], fg, bg, app);

    // Status
    render_status(f, chunks[3], fg, bg, app);

    // Mode-specific panel
    match app.mode {
        AppMode::Browse => render_browse_panel(f, chunks[4], fg, bg, app),
        AppMode::Hunt => render_hunt_panel(f, chunks[4], fg, bg, app),
        AppMode::Results => render_results_panel(f, chunks[4], fg, bg, app),
    }

    // Help overlay
    if app.show_help {
        render_help_overlay(f, fg, bg);
    }
}

fn render_title(f: &mut Frame, area: Rect, fg: Color, bg: Color, app: &ConnectorHunterApp) {
    let mode_text = match app.mode {
        AppMode::Browse => "BROWSE",
        AppMode::Hunt => "HUNT",
        AppMode::Results => "RESULTS",
    };

    let title = Paragraph::new(format!("🔍 PRIME CONNECTOR HUNTER [{}]", mode_text))
        .style(Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(DOS_AMBER))
                .style(Style::default().bg(bg)),
        );
    f.render_widget(title, area);
}

fn render_main_content(f: &mut Frame, area: Rect, fg: Color, bg: Color, app: &ConnectorHunterApp) {
    let preset = app.current_preset();

    let category_icon = match preset.category {
        PresetCategory::Membrane => "🌀",
        PresetCategory::Twin => "👯",
        PresetCategory::SophieGermain => "♀",
        PresetCategory::Asymmetric => "⚖",
    };

    // Bind values that need to live long enough
    let connector_str = app.connector_string();
    let full_number_str = app.full_number_string();

    let lines = vec![
        Line::from(vec![
            Span::styled(
                format!("{} {} ", category_icon, preset.name),
                Style::default().fg(DOS_AMBER).add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(
                "(Preset {}/{})",
                app.preset_index + 1,
                app.presets.len()
            )),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Prime 1: ", Style::default().fg(fg)),
            Span::styled(
                &preset.prime1,
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(" ({} digits)", preset.prime1.len())),
        ]),
        Line::from(vec![
            Span::styled("Prime 2: ", Style::default().fg(fg)),
            Span::styled(
                &preset.prime2,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(" ({} digits)", preset.prime2.len())),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Current connector: "),
            Span::styled(
                connector_str,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Full number: "),
            Span::styled(&full_number_str, Style::default().fg(DOS_WHITE)),
        ]),
        Line::from(format!("Total connectors found: {}", app.connectors.len())),
    ];

    let content = Paragraph::new(Text::from(lines))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Current Configuration")
                .border_style(Style::default().fg(DOS_AMBER))
                .style(Style::default().bg(bg)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(content, area);
}

fn render_controls(f: &mut Frame, area: Rect, fg: Color, bg: Color, app: &ConnectorHunterApp) {
    let controls = match app.mode {
        AppMode::Browse => vec![
            Line::from("←/→: move | ↑/↓: change digit | +/-: resize buffer | SPACE: test"),
            Line::from("p/P: presets | S: start hunt | r: results | h: help | q: quit"),
        ],
        AppMode::Hunt => vec![
            Line::from("Automated search in progress..."),
            Line::from("C: cancel search | r: view results | h: help | q: quit"),
        ],
        AppMode::Results => vec![
            Line::from("e: export CSV | m: export Markdown | b: back to browse"),
            Line::from("p/P: change preset | S: new hunt | h: help | q: quit"),
        ],
    };

    let widget = Paragraph::new(Text::from(controls))
        .style(Style::default().fg(fg).bg(bg))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Controls")
                .border_style(Style::default().fg(DOS_AMBER))
                .style(Style::default().bg(bg)),
        );

    f.render_widget(widget, area);
}

fn render_status(f: &mut Frame, area: Rect, fg: Color, bg: Color, app: &ConnectorHunterApp) {
    let status_style = match app.last_test_result {
        Some(true) => Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
        Some(false) => Style::default().fg(Color::Red),
        None => Style::default().fg(fg),
    };

    let status = Paragraph::new(app.status.clone())
        .style(status_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Status")
                .border_style(Style::default().fg(DOS_AMBER))
                .style(Style::default().bg(bg)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(status, area);
}

fn render_browse_panel(f: &mut Frame, area: Rect, fg: Color, bg: Color, app: &ConnectorHunterApp) {
    let mut lines = vec![
        Line::from(vec![Span::styled(
            "Manual Testing Mode",
            Style::default().fg(DOS_AMBER).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    // Show buffer with cursor
    let mut buffer_spans = vec![Span::raw("Connector: ")];
    for (i, &digit) in app.buffer.iter().enumerate() {
        let style = if i == app.cursor {
            Style::default()
                .fg(DOS_BLACK)
                .bg(DOS_ORANGE)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(DOS_WHITE)
        };
        buffer_spans.push(Span::styled(format!("[{}]", digit), style));
    }
    lines.push(Line::from(buffer_spans));

    lines.push(Line::from(""));
    lines.push(Line::from(format!(
        "Position: {} | Digit: {} | Buffer size: {}",
        app.cursor,
        app.buffer[app.cursor],
        app.buffer.len()
    )));

    // Show known connectors for this preset
    if !app.current_preset().known_connectors.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from("Known connectors for this preset:"));
        for pattern in &app.current_preset().known_connectors {
            lines.push(Line::from(format!("  • {}", pattern)));
        }
    }

    let widget = Paragraph::new(Text::from(lines))
        .style(Style::default().fg(fg).bg(bg))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Browse Panel")
                .border_style(Style::default().fg(DOS_AMBER))
                .style(Style::default().bg(bg)),
        );

    f.render_widget(widget, area);
}

fn render_hunt_panel(f: &mut Frame, area: Rect, fg: Color, bg: Color, app: &ConnectorHunterApp) {
    let progress = app.search_progress.lock().unwrap();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(3)])
        .split(area);

    // Progress bar
    let progress_text = progress.status_text();
    let progress_pct = progress.progress_percent();

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Search Progress")
                .border_style(Style::default().fg(DOS_AMBER))
                .style(Style::default().bg(bg)),
        )
        .gauge_style(Style::default().fg(DOS_ORANGE).bg(DOS_DARK))
        .label(progress_text)
        .percent(progress_pct);

    f.render_widget(gauge, chunks[0]);

    // Real-time results
    let mut lines = vec![
        Line::from(vec![Span::styled(
            "Live Results",
            Style::default().fg(DOS_AMBER).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.connectors.is_empty() {
        lines.push(Line::from("No connectors found yet..."));
    } else {
        let recent = app.connectors.iter().rev().take(5);
        for (i, c) in recent.enumerate() {
            lines.push(Line::from(vec![
                Span::styled(format!("{}. ", i + 1), Style::default().fg(fg)),
                Span::styled(
                    &c.pattern,
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!(
                    " (buf={}, pos={}, d={})",
                    c.buffer_size, c.position, c.digit
                )),
            ]));
        }
    }

    let results = Paragraph::new(Text::from(lines))
        .style(Style::default().fg(fg).bg(bg))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(DOS_AMBER))
                .style(Style::default().bg(bg)),
        );

    f.render_widget(results, chunks[1]);
}

fn render_results_panel(f: &mut Frame, area: Rect, fg: Color, bg: Color, app: &ConnectorHunterApp) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Left: All connectors
    let mut lines = vec![
        Line::from(vec![Span::styled(
            format!("All Connectors ({})", app.connectors.len()),
            Style::default().fg(DOS_AMBER).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.connectors.is_empty() {
        lines.push(Line::from(
            "No connectors found yet. Press 'S' to start hunt.",
        ));
    } else {
        for (i, c) in app.connectors.iter().enumerate() {
            lines.push(Line::from(vec![
                Span::styled(format!("{}. ", i + 1), Style::default().fg(fg)),
                Span::styled(
                    &c.pattern,
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!(
                    " (size={}, pos={}, d={})",
                    c.buffer_size, c.position, c.digit
                )),
            ]));
        }
    }

    let connectors_list = Paragraph::new(Text::from(lines))
        .style(Style::default().fg(fg).bg(bg))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Discovered Connectors")
                .border_style(Style::default().fg(DOS_AMBER))
                .style(Style::default().bg(bg)),
        )
        .wrap(Wrap { trim: false })
        .scroll((0, 0));

    f.render_widget(connectors_list, chunks[0]);

    // Right: Pattern analysis
    let mut analysis_lines = vec![
        Line::from(vec![Span::styled(
            "Pattern Analysis",
            Style::default().fg(DOS_AMBER).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.pattern_analysis.insights.is_empty() {
        analysis_lines.push(Line::from("Run a search to generate insights."));
    } else {
        for insight in &app.pattern_analysis.insights {
            analysis_lines.push(Line::from(format!("• {}", insight)));
        }

        // Position histogram
        if !app.pattern_analysis.position_histogram.is_empty() {
            analysis_lines.push(Line::from(""));
            analysis_lines.push(Line::from("Position distribution:"));
            let mut pos_vec: Vec<_> = app.pattern_analysis.position_histogram.iter().collect();
            pos_vec.sort_by_key(|(pos, _)| **pos);
            for (pos, count) in pos_vec.iter().take(5) {
                analysis_lines.push(Line::from(format!(
                    "  Pos {}: {} █",
                    pos,
                    "▓".repeat(**count)
                )));
            }
        }

        // Digit histogram
        if !app.pattern_analysis.digit_histogram.is_empty() {
            analysis_lines.push(Line::from(""));
            analysis_lines.push(Line::from("Digit distribution:"));
            let mut dig_vec: Vec<_> = app.pattern_analysis.digit_histogram.iter().collect();
            dig_vec.sort_by_key(|(dig, _)| **dig);
            for (dig, count) in dig_vec {
                analysis_lines.push(Line::from(format!(
                    "  Digit {}: {} █",
                    dig,
                    "▓".repeat(*count)
                )));
            }
        }
    }

    let analysis_widget = Paragraph::new(Text::from(analysis_lines))
        .style(Style::default().fg(fg).bg(bg))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Insights")
                .border_style(Style::default().fg(DOS_AMBER))
                .style(Style::default().bg(bg)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(analysis_widget, chunks[1]);
}

fn render_help_overlay(f: &mut Frame, fg: Color, bg: Color) {
    let area = centered_rect(80, 70, f.size());

    let help_text = vec![
        Line::from(vec![Span::styled(
            "Prime Connector Hunter - Help",
            Style::default().fg(DOS_AMBER).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from("CONCEPT:"),
        Line::from("  Find zero-padding patterns C such that concat(prime1, C, prime2) is prime."),
        Line::from("  Example: 10301 + [00006] + 3007003007003 = prime!"),
        Line::from(""),
        Line::from("MODES:"),
        Line::from("  Browse: Manual testing with arrow keys"),
        Line::from("  Hunt: Automated exhaustive search"),
        Line::from("  Results: View all discovered connectors + analysis"),
        Line::from(""),
        Line::from("KEY COMMANDS:"),
        Line::from("  p/P     - Cycle through preset prime pairs"),
        Line::from("  S       - Start automated hunt (buffer 3-10)"),
        Line::from("  C       - Cancel active search"),
        Line::from("  r       - View results"),
        Line::from("  b       - Return to browse mode"),
        Line::from("  e       - Export results to CSV"),
        Line::from("  m       - Export results to Markdown"),
        Line::from("  h       - Toggle this help"),
        Line::from("  a       - Toggle high contrast"),
        Line::from("  q/Esc   - Quit"),
        Line::from(""),
        Line::from("BROWSE MODE:"),
        Line::from("  ←/→     - Move cursor"),
        Line::from("  ↑/↓     - Change digit (0-9)"),
        Line::from("  +/-     - Resize buffer"),
        Line::from("  SPACE   - Test if current number is prime"),
        Line::from(""),
        Line::from("Press 'h' to close this help screen."),
    ];

    let help = Paragraph::new(Text::from(help_text))
        .style(Style::default().fg(fg).bg(bg))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help")
                .border_style(Style::default().fg(DOS_AMBER))
                .style(Style::default().bg(bg)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(ratatui::widgets::Clear, area);
    f.render_widget(help, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
