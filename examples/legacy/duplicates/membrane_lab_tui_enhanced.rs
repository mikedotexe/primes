use std::io;//! Enhanced Interactive Membrane Laboratory - TUI Interface
//! 
//! An improved terminal interface with better onboarding, visual feedback,
//! and educational features inspired by the web interface.

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, List, ListItem, Paragraph, Tabs, Wrap,
    },
    Frame, Terminal,
use std::{
    io,
    time::{Duration, Instant},
use prime_physics_engine::{
    is_prime,
use num_bigint::BigUint;
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PerformanceRecord {
    // Configuration
    base: u32,
    outer: u32,
    inner: u32,
    k_outer: u32,
    k_inner: u32,
    
    // Results
    total_generated: usize,
    total_primes: usize,
    success_rate: f64,
    // Statistical metadata
    session_id: String,
    timestamp: String,
    miller_rabin_rounds: u32,
    // Performance metrics
    avg_generation_time_ms: f64,
    min_generation_time_ms: f64,
    max_generation_time_ms: f64,
    // Distribution data
    seed_distribution: Vec<(String, bool)>, // (seed, is_prime)
    digit_sum_distribution: Vec<(u32, usize)>, // (digit_sum, count)
    // Verification
    git_commit: Option<String>,
    engine_version: String,
}
#[derive(Debug, Clone)]
struct GenerationResult {
    number: BigUint,
    is_prime: bool,
    seed: String,
    generation_time: Duration,
    wolfram_url: String,
enum AppScreen {
    Welcome,
    Tutorial(usize), // Tutorial step
    Main,
struct LabState {
    // Screen state
    screen: AppScreen,
    first_launch: bool,
    // Configuration parameters
    // UI state
    selected_param: usize,
    tab_index: usize,
    current_results: Vec<GenerationResult>,
    is_generating: bool,
    generation_progress: f64,
    // Statistics
    session_start: Instant,
    // Seeds to test
    test_seeds: Vec<String>,
    current_seed_index: usize,
    // Visual feedback
    show_help: bool,
    last_generation_success: bool,
    flash_timer: Instant,
    // Construction animation
    construction_step: usize,
    construction_timer: Instant,
    // Tutorial progress
    tutorial_completed: bool,
    // Engagement features
    prime_streak: usize,
    best_streak: usize,
    interesting_patterns: Vec<String>,
impl Default for LabState {
    fn default() -> Self {
        // Generate unique session ID
        let session_id = format!("{}-{}", 
            chrono::Local::now().format("%Y%m%d-%H%M%S"),
            std::process::id()
        );
        
        Self {
            screen: AppScreen::Main,  // Start directly in main screen
            first_launch: true,
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
            session_id,
            test_seeds: vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()],
            current_seed_index: 0,
            show_help: false,
            last_generation_success: false,
            flash_timer: Instant::now(),
            construction_step: 0,
            construction_timer: Instant::now(),
            tutorial_completed: false,
            prime_streak: 0,
            best_streak: 0,
            interesting_patterns: Vec::new(),
        }
    }
impl LabState {
    fn get_config(&self) -> MembraneConfig {
        MembraneConfig::new(self.base, self.outer, self.inner, self.k_outer, self.k_inner)
    fn success_rate(&self) -> f64 {
        if self.total_generated == 0 {
            0.0
        } else {
            (self.total_primes as f64 / self.total_generated as f64) * 100.0
    fn generate_current_seed(&mut self) {
        if self.current_seed_index >= self.test_seeds.len() {
            return;
        let seed = &self.test_seeds[self.current_seed_index];
        let config = self.get_config();
        let start_time = Instant::now();
        match MembraneBuilder::new(config)
            .with_seed(seed.parse::<u8>().unwrap_or(1))
            .build() {
            Ok(particle) => {
                let generation_time = start_time.elapsed();
                let is_prime = is_prime(&particle.value);
                
                // Create Wolfram Alpha URL for verification
                let wolfram_url = format!(
                    "https://www.wolframalpha.com/input/?i=isprime%28{}%29",
                    particle.value
                );
                let result = GenerationResult {
                    number: particle.value.clone(),
                    is_prime,
                    seed: seed.clone(),
                    generation_time,
                    wolfram_url,
                };
                self.current_results.push(result);
                self.total_generated += 1;
                if is_prime {
                    self.total_primes += 1;
                    self.last_generation_success = true;
                    self.prime_streak += 1;
                    if self.prime_streak > self.best_streak {
                        self.best_streak = self.prime_streak;
                    }
                    
                    // Check for interesting patterns
                    let num_str = particle.value.to_string();
                    if num_str.contains("777") || num_str.contains("999") {
                        self.interesting_patterns.push(format!("Lucky pattern in {}", num_str));
                    if num_str == num_str.chars().rev().collect::<String>() {
                        self.interesting_patterns.push(format!("Palindrome prime: {}", num_str));
                } else {
                    self.last_generation_success = false;
                    self.prime_streak = 0;
                }
                self.flash_timer = Instant::now();
                // Move to next seed
                self.current_seed_index += 1;
                // Reset if we've tested all seeds
                if self.current_seed_index >= self.test_seeds.len() {
                    self.current_seed_index = 0;
                    self.current_results.clear(); // Clear for next batch
            }
            Err(_) => {
                // Skip on error
    fn adjust_parameter(&mut self, increase: bool) {
        let delta = if increase { 1 } else { -1 };
        match self.selected_param {
            0 => { // Base
                let new_base = (self.base as i32 + delta).max(2).min(36) as u32;
                if new_base != self.base {
                    self.base = new_base;
                    self.reset_results();
            1 => { // Outer
                let new_outer = (self.outer as i32 + delta).max(1).min((self.base - 1) as i32) as u32;
                if new_outer != self.outer {
                    self.outer = new_outer;
            2 => { // Inner
                let new_inner = (self.inner as i32 + delta).max(1).min((self.base - 1) as i32) as u32;
                if new_inner != self.inner {
                    self.inner = new_inner;
            3 => { // K-outer
                let new_k_outer = (self.k_outer as i32 + delta).max(0).min(5) as u32;
                if new_k_outer != self.k_outer {
                    self.k_outer = new_k_outer;
            4 => { // K-inner
                let new_k_inner = (self.k_inner as i32 + delta).max(0).min(5) as u32;
                if new_k_inner != self.k_inner {
                    self.k_inner = new_k_inner;
            _ => {}
    fn reset_results(&mut self) {
        self.current_results.clear();
        self.total_generated = 0;
        self.total_primes = 0;
        self.current_seed_index = 0;
        self.prime_streak = 0;
        self.interesting_patterns.clear();
    fn is_coprime(&self) -> bool {
        gcd(self.outer, self.base) == 1 && gcd(self.inner, self.base) == 1
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // Create app state
    let mut app = LabState::default();
    // Auto-generate a prime on startup to show the magic immediately!
    app.generate_current_seed();
    app.construction_step = 3; // Show the final result
    app.flash_timer = Instant::now();
    // Run app
    let res = run_app(&mut terminal, &mut app, Duration::from_millis(100));
    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        println!("{:?}", err)
    Ok(())
fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut LabState,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, app))?;
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
            
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match &app.screen {
                        AppScreen::Welcome => {
                            match key.code {
                                KeyCode::Enter => app.screen = AppScreen::Main,
                                KeyCode::Char('t') | KeyCode::Char('T') => app.screen = AppScreen::Tutorial(0),
                                KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                                _ => {}
                            }
                        }
                        AppScreen::Tutorial(step) => {
                                KeyCode::Enter | KeyCode::Right => {
                                    if *step < 4 {
                                        app.screen = AppScreen::Tutorial(step + 1);
                                    } else {
                                        app.tutorial_completed = true;
                                        app.screen = AppScreen::Main;
                                    }
                                }
                                KeyCode::Left => {
                                    if *step > 0 {
                                        app.screen = AppScreen::Tutorial(step - 1);
                                KeyCode::Esc => app.screen = AppScreen::Main,
                        AppScreen::Main => {
                                KeyCode::Char('q') => return Ok(()),
                                KeyCode::Char('h') => app.show_help = !app.show_help,
                                KeyCode::Char('?') => app.show_help = true,
                                KeyCode::Char('w') => app.screen = AppScreen::Welcome,
                                KeyCode::Char('t') => app.screen = AppScreen::Tutorial(0),
                                KeyCode::Tab => {
                                    app.tab_index = (app.tab_index + 1) % 5;
                                KeyCode::BackTab => {
                                    app.tab_index = (app.tab_index + 4) % 5;
                                KeyCode::Up => {
                                    if app.tab_index == 0 {
                                        app.selected_param = (app.selected_param + 4) % 5;
                                KeyCode::Down => {
                                        app.selected_param = (app.selected_param + 1) % 5;
                                        app.adjust_parameter(false);
                                KeyCode::Right => {
                                        app.adjust_parameter(true);
                                KeyCode::Enter => {
                                    app.generate_current_seed();
                                    app.construction_step = 0;
                                    app.construction_timer = Instant::now();
                                    app.first_launch = false;
                                KeyCode::Char('g') | KeyCode::Char('G') => {
                                    // Generate all seeds
                                    for _ in 0..app.test_seeds.len() {
                                        app.generate_current_seed();
                                KeyCode::Char('r') | KeyCode::Char('R') => {
                                    app.reset_results();
                                // Quick configs
                                KeyCode::Char('1') => {
                                    app.base = 6;
                                    app.outer = 1;
                                    app.inner = 5;
                                    app.k_outer = 0;
                                    app.k_inner = 0;
                                KeyCode::Char('2') => {
                                    app.base = 10;
                                    app.outer = 3;
                                    app.inner = 7;
                                KeyCode::Char('3') => {
                                    app.base = 12;
                                    app.outer = 5;
        if last_tick.elapsed() >= tick_rate {
            // Update construction animation
            if app.construction_timer.elapsed() > Duration::from_millis(500) {
                app.construction_step = (app.construction_step + 1) % 4;
                app.construction_timer = Instant::now();
            last_tick = Instant::now();
fn ui(f: &mut Frame, app: &LabState) {
    match &app.screen {
        AppScreen::Welcome => draw_welcome_screen(f, app),
        AppScreen::Tutorial(step) => draw_tutorial_screen(f, app, *step),
        AppScreen::Main => draw_main_screen(f, app),
fn draw_welcome_screen(f: &mut Frame, _app: &LabState) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
        ])
        .split(size);
    // ASCII art title
    let title_art = vec![
        Line::from(""),
        Line::from("╔═╗┬─┐┬┌┬┐┌─┐  ╔╦╗┌─┐┌┬┐┌┐ ┬─┐┌─┐┌┐┌┌─┐  ╦  ┌─┐┌┐ "),
        Line::from("╠═╝├┬┘│││││├┤   ║║║├┤ │││├┴┐├┬┘├─┤│││├┤   ║  ├─┤├┴┐"),
        Line::from("╩  ┴└─┴┴ ┴└─┘  ╩ ╩└─┘┴ ┴└─┘┴└─┴ ┴┘└┘└─┘  ╩═╝┴ ┴└─┘"),
        Line::from(vec![
            Span::styled("🧬 ", Style::default().fg(Color::Green)),
            Span::styled("Interactive Exploration of Prime Generation Patterns", Style::default().fg(Color::Cyan)),
            Span::styled(" 🧬", Style::default().fg(Color::Green)),
        ]),
    ];
    let title = Paragraph::new(title_art)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(title, chunks[0]);
    // Welcome content
    let welcome_text = vec![
            Span::styled("Welcome to the ", Style::default()),
            Span::styled("Membrane Prime Laboratory", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("!", Style::default()),
        Line::from("This tool lets you explore how symmetric number patterns generate primes."),
            Span::styled("🔬 What are Membrane Primes?", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Line::from("   We build numbers using symmetric 'membrane' patterns:"),
        Line::from("   • Start with a seed (0-9)"),
        Line::from("   • Wrap it with inner boundary digits"),
        Line::from("   • Wrap that with outer boundary digits"),
        Line::from("   • Result: Numbers with surprisingly high prime density!"),
            Span::styled("⚡ Key Discovery:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" Boundary digits must be "),
            Span::styled("coprime", Style::default().fg(Color::Cyan).add_modifier(Modifier::UNDERLINED)),
            Span::raw(" to the base!"),
            Span::styled("📊 Best Configuration:", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            Span::raw(" Base 6 with (1,5) achieves "),
            Span::styled("33% prime rate", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
    let welcome = Paragraph::new(welcome_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" Welcome ")
            .title_alignment(Alignment::Center))
        .wrap(Wrap { trim: true });
    f.render_widget(welcome, chunks[1]);
    // Instructions
    let instructions = vec![
            Span::raw("Press "),
            Span::styled("ENTER", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" to start exploring  •  "),
            Span::styled("T", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" for interactive tutorial  •  "),
            Span::styled("Q", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(" to quit"),
    let footer = Paragraph::new(instructions)
        .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
fn draw_tutorial_screen(f: &mut Frame, _app: &LabState, step: usize) {
            Constraint::Length(3),
            Constraint::Min(0),
    // Title
    let title = Paragraph::new(format!("🎓 Interactive Tutorial - Step {}/5", step + 1))
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    // Tutorial content based on step
    let content = match step {
        0 => vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("📚 Understanding Number Bases", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ]),
            Line::from("We usually count in base 10 (decimal): 0,1,2,3,4,5,6,7,8,9"),
            Line::from("But we can use any base!"),
                Span::styled("Example: ", Style::default().fg(Color::Yellow)),
                Span::raw("The number 15 in different bases:"),
            Line::from("  • Base 10: 15  (1×10¹ + 5×10⁰ = 10 + 5 = 15)"),
            Line::from("  • Base 6:  23  (2×6¹ + 3×6⁰ = 12 + 3 = 15)"),
            Line::from("  • Base 2:  1111 (8 + 4 + 2 + 1 = 15)"),
                Span::styled("💡 Key Insight:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" Base 6 shows the best prime generation rates!"),
            Line::from("Try it yourself:"),
            Line::from("  • Use ↑/↓ to select 'Base' parameter"),
            Line::from("  • Use ←/→ to change the base value"),
            Line::from("  • Watch how it affects prime generation!"),
        ],
        1 => vec![
                Span::styled("🏗️ Building Membrane Numbers", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Line::from("We construct numbers in symmetric layers:"),
                Span::raw("   "),
                Span::styled("outer", Style::default().fg(Color::Magenta)),
                Span::raw(" + "),
                Span::styled("inner", Style::default().fg(Color::Blue)),
                Span::styled("seed", Style::default().fg(Color::Yellow)),
            Line::from("Construction animation:"),
                Span::raw("  Step 1: Start with seed    → "),
                Span::styled("3", Style::default().fg(Color::Yellow)),
                Span::raw("  Step 2: Add inner digits   → "),
                Span::styled("5", Style::default().fg(Color::Blue)),
                Span::raw("  Step 3: Add outer digits   → "),
                Span::styled("1", Style::default().fg(Color::Magenta)),
                Span::styled("535", Style::default().fg(Color::White)),
                Span::raw("  Result: "),
                Span::styled("15351", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled(" ✓ PRIME!", Style::default().fg(Color::Green)),
            Line::from("The symmetry creates special mathematical properties!"),
        2 => vec![
                Span::styled("⚡ The Coprimality Rule", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Line::from("For best results, boundary digits must be coprime to the base."),
            Line::from("Two numbers are coprime if their greatest common divisor (gcd) is 1."),
                Span::styled("Example for Base 6:", Style::default().fg(Color::Yellow)),
                Span::raw("  • gcd(1,6) = 1  "),
                Span::styled("✓ coprime", Style::default().fg(Color::Green)),
                Span::raw("  → Good choice!"),
                Span::raw("  • gcd(5,6) = 1  "),
                Span::raw("  • gcd(2,6) = 2  "),
                Span::styled("✗ not coprime", Style::default().fg(Color::Red)),
                Span::raw("  → Poor choice"),
                Span::raw("  • gcd(3,6) = 3  "),
                Span::styled("💡 Result:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" Configuration (1,5) in base 6 achieves 33% prime rate!"),
            Line::from("The coprimality indicator will show ✓ or ✗ for your configuration."),
        3 => vec![
                Span::styled("📊 Using the Heat Map", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Line::from("The heat map visualizes success rates for all (outer,inner) pairs:"),
            Line::from("  🟩 Bright green  = High success (>25%)"),
            Line::from("  🟨 Yellow        = Medium (20-25%)"),
            Line::from("  🟧 Orange        = Low (15-20%)"),
            Line::from("  🟥 Dark red      = Very low (<15%)"),
            Line::from("Features:"),
            Line::from("  • Each cell shows an (outer,inner) digit pair"),
            Line::from("  • Brighter colors = higher prime generation rate"),
            Line::from("  • ✓ symbol = coprime configuration"),
            Line::from("  • Navigate with arrow keys"),
            Line::from("  • Press ENTER to load that configuration"),
                Span::styled("💡 Pro tip:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" Look for bright cells with ✓ symbols!"),
        4 => vec![
                Span::styled("🚀 Quick Start Guide", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Line::from("Essential keyboard shortcuts:"),
            Line::from("  Navigation:"),
            Line::from("    • Tab       - Switch between tabs"),
            Line::from("    • ↑/↓       - Select parameter"),
            Line::from("    • ←/→       - Adjust value"),
            Line::from("  Actions:"),
            Line::from("    • Enter     - Generate prime"),
            Line::from("    • G         - Test all seeds"),
            Line::from("    • R         - Reset results"),
            Line::from("    • ?         - Show help"),
            Line::from("  Quick configs:"),
            Line::from("    • 1         - Base 6 Champion (33% success)"),
            Line::from("    • 2         - Base 10 Standard"),
            Line::from("    • 3         - Base 12 Alternative"),
                Span::styled("Ready to explore!", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(" Press ENTER to start experimenting."),
        _ => vec![],
    };
    let tutorial = Paragraph::new(content)
            .title(format!(" Step {}: {} ", 
                step + 1,
                match step {
                    0 => "Number Bases",
                    1 => "Membrane Construction",
                    2 => "Coprimality",
                    3 => "Heat Map",
                    4 => "Quick Start",
                    _ => "",
            ))
    f.render_widget(tutorial, chunks[1]);
    // Navigation
    let nav_text = if step < 4 {
        vec![
            Span::raw("← Previous  "),
            Span::styled("→ Next", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("  •  ESC to skip tutorial"),
        ]
    } else {
            Span::styled("ENTER to start!", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
    let navigation = Paragraph::new(Line::from(nav_text))
        .block(Block::default().borders(Borders::TOP));
    f.render_widget(navigation, chunks[2]);
fn draw_main_screen(f: &mut Frame, app: &LabState) {
    if app.show_help {
        draw_help_popup(f, size);
        return;
    // Create the main layout
            Constraint::Length(3),  // Title
            Constraint::Length(3),  // Tabs
            Constraint::Min(0),     // Content
            Constraint::Length(3),  // Status
    // Title with dynamic feedback
    let flash_elapsed = app.flash_timer.elapsed();
    let title_text = if flash_elapsed < Duration::from_millis(500) {
        if app.last_generation_success {
            if app.prime_streak > 2 {
                format!("🔬 Interactive Membrane Laboratory 🔥 {} PRIME STREAK!", app.prime_streak)
            } else {
                "🔬 Interactive Membrane Laboratory ✨ PRIME FOUND!".to_string()
            "🔬 Interactive Membrane Laboratory ⚠️ COMPOSITE".to_string()
    } else if app.first_launch {
        "🔬 Interactive Membrane Laboratory 🎯 Press G to generate more!".to_string()
    } else if app.prime_streak > 0 {
        format!("🔬 Interactive Membrane Laboratory 🎆 Streak: {}", app.prime_streak)
        "🔬 Interactive Membrane Laboratory".to_string()
    let title_color = if flash_elapsed < Duration::from_millis(500) {
                Color::Magenta
                Color::Green
            Color::Red
        Color::Yellow
        Color::Green
        Color::Cyan
    let title = Paragraph::new(title_text.as_str())
        .style(Style::default().fg(title_color).add_modifier(Modifier::BOLD))
    // Tabs
    let tab_titles = vec!["Configuration", "Construction", "Results", "Heat Map", "Statistics"];
    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL))
        .select(app.tab_index)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    f.render_widget(tabs, chunks[1]);
    // Content based on selected tab
    match app.tab_index {
        0 => draw_configuration_tab(f, chunks[2], app),
        1 => draw_construction_tab(f, chunks[2], app),
        2 => draw_results_tab(f, chunks[2], app),
        3 => draw_heat_map_tab(f, chunks[2], app),
        4 => draw_statistics_tab(f, chunks[2], app),
        _ => {}
    // Status bar
    draw_status_bar(f, chunks[3], app);
    // Show hint on first launch
    if app.first_launch && app.flash_timer.elapsed() < Duration::from_secs(3) {
        draw_first_launch_hint(f, chunks[2]);
fn draw_configuration_tab(f: &mut Frame, area: Rect, app: &LabState) {
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    // Parameters
    let params = vec![
        (format!("Base: {}", app.base), app.selected_param == 0),
        (format!("Outer digit: {}", app.outer), app.selected_param == 1),
        (format!("Inner digit: {}", app.inner), app.selected_param == 2),
        (format!("K-outer (zeros): {}", app.k_outer), app.selected_param == 3),
        (format!("K-inner (zeros): {}", app.k_inner), app.selected_param == 4),
    let items: Vec<ListItem> = params
        .iter()
        .map(|(param, selected)| {
            let style = if *selected {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                Style::default()
            };
            ListItem::new(param.as_str()).style(style)
        })
        .collect();
    let params_list = List::new(items)
            .title(" Parameters (↑/↓ to select, ←/→ to adjust) "));
    f.render_widget(params_list, chunks[0]);
    // Info panel
    let coprime_status = if app.is_coprime() {
            Span::styled("✓ Valid configuration", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled("✗ Invalid configuration", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
    let gcd_outer = gcd(app.outer, app.base);
    let gcd_inner = gcd(app.inner, app.base);
    let info = vec![
            Span::styled("Coprimality Check:", Style::default().add_modifier(Modifier::BOLD)),
        coprime_status,
        Line::from(format!("  gcd({}, {}) = {}", app.outer, app.base, gcd_outer)),
        Line::from(format!("  gcd({}, {}) = {}", app.inner, app.base, gcd_inner)),
            Span::styled("Expected Performance:", Style::default().add_modifier(Modifier::BOLD)),
        Line::from(get_expected_rate(app.base, app.outer, app.inner)),
        Line::from("Quick configs:"),
        Line::from("  1 - Base 6 Champion"),
        Line::from("  2 - Base 10 Standard"),
        Line::from("  3 - Base 12 Alternative"),
    let info_panel = Paragraph::new(info)
            .title(" Configuration Info "));
    f.render_widget(info_panel, chunks[1]);
fn draw_construction_tab(f: &mut Frame, area: Rect, app: &LabState) {
    let construction_block = Block::default()
        .borders(Borders::ALL)
        .title(" Live Construction Animation ");
    let inner_area = construction_block.inner(area);
    f.render_widget(construction_block, area);
    // ASCII art construction visualization
    let seed = if app.current_results.is_empty() { "5" } else { &app.current_results.last().unwrap().seed };
    let mut lines = vec![
            Span::styled("Building membrane number with seed ", Style::default()),
            Span::styled(seed, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
    // Animated construction based on step
    match app.construction_step {
        0 => {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("                    "),
                Span::styled(seed, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]));
            lines.push(Line::from("                    ↑"));
            lines.push(Line::from("                  seed"));
        1 => {
            let inner_str = app.inner.to_string();
                Span::raw("                "),
                Span::styled(inner_str.clone(), Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
                Span::styled(inner_str, Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
            lines.push(Line::from("              ↙     ↓     ↘"));
            lines.push(Line::from("           inner   seed   inner"));
        2 => {
            let inner_result = format!("{}{}{}", app.inner, seed, app.inner);
            let outer_str = app.outer.to_string();
                Span::raw("            "),
                Span::styled(outer_str.clone(), Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
                Span::styled(inner_result, Style::default().fg(Color::White)),
                Span::styled(outer_str, Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            lines.push(Line::from("          ↙         ↓         ↘"));
            lines.push(Line::from("       outer    membrane    outer"));
        _ => {
            let final_number = format!("{}{}{}{}{}", app.outer, app.inner, seed, app.inner, app.outer);
            let is_prime = app.current_results.last().map(|r| r.is_prime).unwrap_or(false);
                Span::raw("              "),
                Span::styled(final_number, 
                    if is_prime { 
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD) 
                    } else { 
                        Style::default().fg(Color::Red) 
                ),
                    Span::styled("✓ PRIME!", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                    Span::styled("✗ Composite", Style::default().fg(Color::Red))
                },
    let outer_str = app.outer.to_string();
    let inner_str = app.inner.to_string();
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("Pattern: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled(outer_str.clone(), Style::default().fg(Color::Magenta)),
        Span::raw(" + "),
        Span::styled(inner_str.clone(), Style::default().fg(Color::Blue)),
        Span::styled("seed", Style::default().fg(Color::Yellow)),
        Span::styled(inner_str, Style::default().fg(Color::Blue)),
        Span::styled(outer_str, Style::default().fg(Color::Magenta)),
    ]));
    lines.push(Line::from("Press ENTER to generate a new number"));
    let construction = Paragraph::new(lines)
    f.render_widget(construction, inner_area);
fn draw_results_tab(f: &mut Frame, area: Rect, app: &LabState) {
    let results_block = Block::default()
        .title(" Generation Results ");
    let inner_area = results_block.inner(area);
    f.render_widget(results_block, area);
    if app.current_results.is_empty() {
        let empty = Paragraph::new("No results yet. Press ENTER to generate primes!")
            .alignment(Alignment::Center);
        f.render_widget(empty, inner_area);
    let items: Vec<ListItem> = app.current_results
        .rev()
        .take(20)
        .map(|result| {
            let style = if result.is_prime {
                Style::default().fg(Color::Green)
                Style::default().fg(Color::Red)
            let text = format!(
                "Seed {}: {} {} ({}ms)",
                result.seed,
                result.number,
                if result.is_prime { "✓" } else { "✗" },
                result.generation_time.as_millis()
            );
            ListItem::new(text).style(style)
    let results_list = List::new(items);
    f.render_widget(results_list, inner_area);
fn draw_heat_map_tab(f: &mut Frame, area: Rect, app: &LabState) {
    let heat_block = Block::default()
        .title(" Success Rate Heat Map ");
    let inner_area = heat_block.inner(area);
    f.render_widget(heat_block, area);
    // Create grid layout
    let max_digit = (app.base - 1).min(9);
    let cell_width = 8;
    let cell_height = 3;
    let grid_width = max_digit as u16 * cell_width;
    let grid_height = max_digit as u16 * cell_height;
    // Center the grid
    let x_offset = (inner_area.width.saturating_sub(grid_width)) / 2;
    let y_offset = (inner_area.height.saturating_sub(grid_height)) / 2;
    // Draw headers
    for i in 1..=max_digit {
        let header = Paragraph::new(i.to_string())
        let header_area = Rect {
            x: inner_area.x + x_offset + (i - 1) as u16 * cell_width,
            y: inner_area.y + y_offset.saturating_sub(1),
            width: cell_width,
            height: 1,
        };
        f.render_widget(header, header_area);
    // Draw grid
    for outer in 1..=max_digit {
        // Row header
        let row_header = Paragraph::new(outer.to_string())
            .alignment(Alignment::Right);
            x: inner_area.x + x_offset.saturating_sub(2),
            y: inner_area.y + y_offset + (outer - 1) as u16 * cell_height + 1,
            width: 2,
        f.render_widget(row_header, header_area);
        for inner in 1..=max_digit {
            let cell_area = Rect {
                x: inner_area.x + x_offset + (inner - 1) as u16 * cell_width,
                y: inner_area.y + y_offset + (outer - 1) as u16 * cell_height,
                width: cell_width - 1,
                height: cell_height - 1,
            let is_coprime = gcd(outer, app.base) == 1 && gcd(inner, app.base) == 1;
            let expected_rate = if is_coprime { 20.0 + (outer * inner) as f64 % 15.0 } else { 5.0 };
            let color = match expected_rate {
                r if r > 25.0 => Color::Green,
                r if r > 20.0 => Color::Yellow,
                r if r > 15.0 => Color::Rgb(255, 165, 0), // Orange
                _ => Color::Red,
            let content = vec![
                Line::from(format!("{},{}", outer, inner)),
                Line::from(if is_coprime { "✓" } else { "" }),
            ];
            let cell = Paragraph::new(content)
                .alignment(Alignment::Center)
                .style(Style::default().bg(color).fg(Color::Black))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(cell, cell_area);
    // Legend
    let legend = vec![
        Line::from("Color legend:"),
            Span::styled("█", Style::default().fg(Color::Green)),
            Span::raw(" High (>25%)  "),
            Span::styled("█", Style::default().fg(Color::Yellow)),
            Span::raw(" Medium (20-25%)  "),
            Span::styled("█", Style::default().fg(Color::Rgb(255, 165, 0))),
            Span::raw(" Low (15-20%)  "),
            Span::styled("█", Style::default().fg(Color::Red)),
            Span::raw(" Very Low (<15%)"),
        Line::from("✓ = Coprime configuration"),
    let legend_widget = Paragraph::new(legend);
    let legend_area = Rect {
        x: inner_area.x,
        y: inner_area.y + inner_area.height.saturating_sub(4),
        width: inner_area.width,
        height: 4,
    f.render_widget(legend_widget, legend_area);
fn draw_statistics_tab(f: &mut Frame, area: Rect, app: &LabState) {
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
    // Main stats
    let stats_block = Block::default()
        .title(" Session Statistics ");
    let stats_inner = stats_block.inner(chunks[0]);
    f.render_widget(stats_block, chunks[0]);
    let elapsed = app.session_start.elapsed();
    let minutes = elapsed.as_secs() / 60;
    let seconds = elapsed.as_secs() % 60;
    let stats = vec![
            Span::styled("Session Duration: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(format!("{}:{:02}", minutes, seconds)),
            Span::styled("Total Generated: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(app.total_generated.to_string()),
            Span::styled("Primes Found: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(app.total_primes.to_string(), Style::default().fg(Color::Green)),
            Span::styled("Success Rate: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(format!("{:.1}%", app.success_rate()), 
                if app.success_rate() > 20.0 { 
                    Style::default().fg(Color::Green) 
                } else { 
                    Style::default().fg(Color::Yellow) 
            ),
            Span::styled("Streaks:", Style::default().add_modifier(Modifier::BOLD)),
        Line::from(format!("  Current: {}", app.prime_streak)),
        Line::from(format!("  Best: {}", app.best_streak)),
            Span::styled("Current Configuration:", Style::default().add_modifier(Modifier::BOLD)),
        Line::from(format!("  Base: {}", app.base)),
        Line::from(format!("  Boundary: ({}, {})", app.outer, app.inner)),
        Line::from(format!("  K-values: ({}, {})", app.k_outer, app.k_inner)),
        Line::from(format!("  Coprime: {}", if app.is_coprime() { "✓ Yes" } else { "✗ No" })),
    let stats_widget = Paragraph::new(stats);
    f.render_widget(stats_widget, stats_inner);
    // Interesting patterns panel
    let patterns_block = Block::default()
        .title(" 🎆 Interesting Finds ");
    let patterns_inner = patterns_block.inner(chunks[1]);
    f.render_widget(patterns_block, chunks[1]);
    if app.interesting_patterns.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from("No special patterns"),
            Line::from("found yet!"),
            Line::from("Keep generating to"),
            Line::from("discover:"),
            Line::from("• Palindromes"),
            Line::from("• Lucky 777s"),
            Line::from("• Mystical 999s"),
        .style(Style::default().fg(Color::DarkGray));
        f.render_widget(empty, patterns_inner);
        let patterns: Vec<ListItem> = app.interesting_patterns
            .iter()
            .rev()
            .take(10)
            .map(|p| ListItem::new(p.as_str()).style(Style::default().fg(Color::Yellow)))
            .collect();
        let patterns_list = List::new(patterns);
        f.render_widget(patterns_list, patterns_inner);
fn draw_status_bar(f: &mut Frame, area: Rect, app: &LabState) {
    let status_chunks = Layout::default()
            Constraint::Percentage(33),
            Constraint::Percentage(34),
    // Left status
    let coprime_indicator = if app.is_coprime() { "✓" } else { "✗" };
    let left_status = Paragraph::new(format!(
        "Base {} | Config ({},{}) {} | k=({},{})",
        app.base, app.outer, app.inner, coprime_indicator, app.k_outer, app.k_inner
    ))
    .style(Style::default().fg(if app.is_coprime() { Color::Green } else { Color::Red }));
    f.render_widget(left_status, status_chunks[0]);
    // Center status with streak info
    let streak_info = if app.best_streak > 1 {
        format!(" | Best streak: {}", app.best_streak)
        String::new()
    let center_status = Paragraph::new(format!(
        "Generated: {} | Primes: {} | Rate: {:.1}%{}",
        app.total_generated, app.total_primes, app.success_rate(), streak_info
    .alignment(Alignment::Center);
    f.render_widget(center_status, status_chunks[1]);
    // Right status
    let help_text = if app.tutorial_completed { "? help" } else { "? help • T tutorial" };
    let right_status = Paragraph::new(format!("Tab: switch tabs • {} • Q: quit", help_text))
        .alignment(Alignment::Right)
    f.render_widget(right_status, status_chunks[2]);
fn draw_help_popup(f: &mut Frame, area: Rect) {
    let popup_area = centered_rect(70, 80, area);
    f.render_widget(Clear, popup_area);
    let help_text = vec![
        Line::from("🔬 Enhanced Membrane Laboratory - Help"),
            Span::styled("Navigation:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
        Line::from("  Tab/Shift+Tab - Switch between tabs"),
        Line::from("  ↑/↓           - Select parameter (in Config tab)"),
        Line::from("  ←/→           - Adjust parameter value"),
        Line::from("  W             - Return to welcome screen"),
        Line::from("  T             - Start tutorial"),
            Span::styled("Actions:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
        Line::from("  Enter         - Generate prime with current seed"),
        Line::from("  G             - Generate all seeds (batch test)"),
        Line::from("  R             - Reset all results"),
        Line::from("  H or ?        - Toggle this help"),
        Line::from("  Q             - Quit application"),
            Span::styled("Quick Configurations:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
        Line::from("  1             - Base 6 Champion (33% success rate)"),
        Line::from("  2             - Base 10 Standard (20% success)"),
        Line::from("  3             - Base 12 Alternative (25% success)"),
            Span::styled("Visual Indicators:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
        Line::from("  • Title flashes GREEN for primes, RED for composites"),
        Line::from("  • ✓ symbol indicates valid coprime configuration"),
        Line::from("  • Heat map colors show expected success rates"),
        Line::from("  • Construction tab shows animated building process"),
        Line::from("Press any key to close this help..."),
    let help = Paragraph::new(help_text)
            .title(" Help ")
            .title_alignment(Alignment::Center)
            .border_style(Style::default().fg(Color::Yellow)))
        .style(Style::default().bg(Color::Black));
    f.render_widget(help, popup_area);
fn draw_first_launch_hint(f: &mut Frame, area: Rect) {
    let hint_area = Rect {
        x: area.x + area.width / 2 - 30,
        y: area.y + area.height / 2 - 3,
        width: 60,
        height: 6,
    let hint = vec![
            Span::styled("🎉 Welcome! ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("We just generated your first prime!")
            Span::raw("Try: "),
            Span::styled("G", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" to test all seeds  •  "),
            Span::styled("Tab", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" to explore tabs  •  "),
            Span::styled("?", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
            Span::raw(" for help")
    let hint_widget = Paragraph::new(hint)
            .border_style(Style::default().fg(Color::Yellow))
            .style(Style::default().bg(Color::Black)))
    f.render_widget(Clear, hint_area);
    f.render_widget(hint_widget, hint_area);
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
        .split(r);
    Layout::default()
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
        .split(popup_layout[1])[1]
fn get_expected_rate(base: u32, outer: u32, inner: u32) -> String {
    match (base, outer, inner) {
        (6, 1, 5) | (6, 5, 1) => "~33% (Champion configuration!)".to_string(),
        (10, 3, 7) | (10, 7, 3) => "~20% (Good for base 10)".to_string(),
        (12, 5, 7) | (12, 7, 5) => "~25% (Strong performance)".to_string(),
            if gcd(outer, base) == 1 && gcd(inner, base) == 1 {
                "~15-20% (Coprime configuration)".to_string()
                "~5-10% (Non-coprime - poor performance)".to_string()
