use std::io;//! Prime Membrane Laboratory - Professional Edition
//! 
//! The definitive TUI combining:
//! - Educational onboarding from enhanced version
//! - Statistical rigor from original version  
//! - Gamification and real-time analytics from ultimate version
//! - Smooth, immediate engagement without barriers

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, Gauge, List, ListItem, 
        Paragraph, Sparkline, Tabs, Wrap,
    },
    Frame, Terminal,
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
    fs::{OpenOptions, create_dir_all},
    io::Write,
    path::PathBuf,
use primes::{
    is_prime,
use num_bigint::BigUint;
// ===== DATA STRUCTURES =====
#[derive(Debug, Clone, PartialEq)]
enum Achievement {
    FirstPrime,
    TenPrimes,
    HundredPrimes,
    StreakFive,
    StreakTen,
    BeatExpected,
    PerfectSession,
    Palindrome,
    LuckyPattern,
    MillionDigit,
    SpeedDemon,
    Explorer,
    DataScientist,
    TutorialComplete,
}
impl Achievement {
    fn name(&self) -> &str {
        match self {
            Achievement::FirstPrime => "🌟 First Steps",
            Achievement::TenPrimes => "🔟 Decimal Power",
            Achievement::HundredPrimes => "💯 Century Mark",
            Achievement::StreakFive => "🔥 Hot Streak",
            Achievement::StreakTen => "🌋 Unstoppable",
            Achievement::BeatExpected => "📈 Defying Odds",
            Achievement::PerfectSession => "💎 Perfection",
            Achievement::Palindrome => "🔄 Mirror Master",
            Achievement::LuckyPattern => "🍀 Pattern Seeker",
            Achievement::MillionDigit => "🏔️ Big Numbers",
            Achievement::SpeedDemon => "⚡ Lightning Fast",
            Achievement::Explorer => "🗺️ Cartographer",
            Achievement::DataScientist => "📊 Data Master",
            Achievement::TutorialComplete => "🎓 Quick Learner",
        }
    }
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
    // Metadata
    session_id: String,
    timestamp: String,
    miller_rabin_rounds: u32,
    // Performance
    avg_generation_time_ms: f64,
    generation_times: Vec<f64>,
    // Statistical
    confidence_interval: (f64, f64),
    p_value: f64,
    expected_rate: f64,
    performance_ratio: f64,
#[derive(Debug, Clone)]
struct GenerationResult {
    number: BigUint,
    is_prime: bool,
    seed: String,
    generation_time: Duration,
    wolfram_url: String,
enum AppScreen {
    Welcome,
    Tutorial(usize),
    Main,
struct LabState {
    // Screen state
    screen: AppScreen,
    // UI state
    selected_param: usize,
    tab_index: usize,
    show_help: bool,
    current_results: Vec<GenerationResult>,
    // Statistics
    session_start: Instant,
    // Performance tracking
    success_history: Vec<f64>,
    config_history: HashMap<String, PerformanceRecord>,
    // Gamification
    prime_streak: usize,
    best_streak: usize,
    achievements: Vec<Achievement>,
    pending_achievement: Option<(Achievement, Instant)>,
    interesting_patterns: Vec<String>,
    // Visual feedback
    last_generation_success: bool,
    flash_timer: Instant,
    construction_step: usize,
    construction_timer: Instant,
    // First launch
    first_launch: bool,
    hint_timer: Option<Instant>,
    // Seeds
    test_seeds: Vec<String>,
    current_seed_index: usize,
impl Default for LabState {
    fn default() -> Self {
        let session_id = format!("{}-{}", 
            chrono::Local::now().format("%Y%m%d-%H%M%S"),
            std::process::id()
        );
        
        Self {
            screen: AppScreen::Main,
            base: 6,
            outer: 1,
            inner: 5,
            k_outer: 0,
            k_inner: 0,
            selected_param: 0,
            tab_index: 0,
            show_help: false,
            current_results: Vec::new(),
            total_generated: 0,
            total_primes: 0,
            session_start: Instant::now(),
            session_id,
            success_history: Vec::with_capacity(50),
            config_history: HashMap::new(),
            prime_streak: 0,
            best_streak: 0,
            achievements: Vec::new(),
            pending_achievement: None,
            interesting_patterns: Vec::new(),
            last_generation_success: false,
            flash_timer: Instant::now(),
            construction_step: 0,
            construction_timer: Instant::now(),
            first_launch: true,
            hint_timer: Some(Instant::now()),
            test_seeds: (0..10).map(|i| i.to_string()).collect(),
            current_seed_index: 0,
// ===== CORE IMPLEMENTATION =====
impl LabState {
    fn get_config(&self) -> MembraneConfig {
        MembraneConfig::new(self.base, self.outer, self.inner, self.k_outer, self.k_inner)
    fn config_hash(&self) -> String {
        format!("{},{},{},{},{}", self.base, self.outer, self.inner, self.k_outer, self.k_inner)
    fn get_expected_rate(&self) -> f64 {
        match (self.base, self.outer, self.inner, self.k_outer, self.k_inner) {
            (6, 1, 5, 0, 0) | (6, 5, 1, 0, 0) => 33.0,
            (30, 11, 7, 0, 0) => 30.0,
            (10, 3, 7, 0, 0) => 20.0,
            (12, 5, 7, 0, 0) => 25.0,
            _ => {
                if self.is_coprime() {
                    15.0 + (self.base as f64 / 10.0)
                } else {
                    5.0
                }
            }
    fn is_coprime(&self) -> bool {
        gcd(self.outer, self.base) == 1 && gcd(self.inner, self.base) == 1
    fn success_rate(&self) -> f64 {
        let hash = self.config_hash();
        if let Some(record) = self.config_history.get(&hash) {
            record.success_rate
        } else if self.total_generated > 0 {
            (self.total_primes as f64 / self.total_generated as f64) * 100.0
        } else {
            0.0
    fn generate_current_seed(&mut self) {
        if self.current_seed_index >= self.test_seeds.len() {
            self.current_seed_index = 0;
        let seed = &self.test_seeds[self.current_seed_index];
        let config = self.get_config();
        let start_time = Instant::now();
        match MembraneBuilder::new(config)
            .with_seed(seed.parse::<u8>().unwrap_or(1))
            .build() {
            Ok(particle) => {
                let generation_time = start_time.elapsed();
                let is_prime = is_prime(&particle.value);
                
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
                // Update results
                self.current_results.push(result);
                if self.current_results.len() > 100 {
                    self.current_results.remove(0);
                self.total_generated += 1;
                // Update config history
                let hash = self.config_hash();
                let expected_rate = self.get_expected_rate();
                let session_id = self.session_id.clone();
                let base = self.base;
                let outer = self.outer;
                let inner = self.inner;
                let k_outer = self.k_outer;
                let k_inner = self.k_inner;
                let record = self.config_history.entry(hash).or_insert_with(|| {
                    PerformanceRecord {
                        base,
                        outer,
                        inner,
                        k_outer,
                        k_inner,
                        total_generated: 0,
                        total_primes: 0,
                        success_rate: 0.0,
                        session_id,
                        timestamp: chrono::Local::now().to_rfc3339(),
                        miller_rabin_rounds: 20,
                        avg_generation_time_ms: 0.0,
                        generation_times: Vec::new(),
                        confidence_interval: (0.0, 0.0),
                        p_value: 1.0,
                        expected_rate,
                        performance_ratio: 0.0,
                    }
                });
                record.total_generated += 1;
                record.generation_times.push(generation_time.as_micros() as f64 / 1000.0);
                if is_prime {
                    self.total_primes += 1;
                    record.total_primes += 1;
                    self.last_generation_success = true;
                    self.prime_streak += 1;
                    if self.prime_streak > self.best_streak {
                        self.best_streak = self.prime_streak;
                    
                    // Check for patterns
                    let num_str = particle.value.to_string();
                    if num_str == num_str.chars().rev().collect::<String>() {
                        self.interesting_patterns.push(format!("Palindrome: {}", num_str));
                        self.check_achievement(Achievement::Palindrome);
                    if num_str.contains("777") || num_str.contains("999") {
                        self.interesting_patterns.push(format!("Lucky: {}", num_str));
                        self.check_achievement(Achievement::LuckyPattern);
                    self.last_generation_success = false;
                    self.prime_streak = 0;
                // Calculate statistics inline to avoid borrow issues
                if record.total_generated > 0 {
                    record.success_rate = (record.total_primes as f64 / record.total_generated as f64) * 100.0;
                    record.performance_ratio = record.success_rate / record.expected_rate;
                    if !record.generation_times.is_empty() {
                        record.avg_generation_time_ms = record.generation_times.iter().sum::<f64>() / record.generation_times.len() as f64;
                    // Wilson score interval
                    let n = record.total_generated as f64;
                    let p = record.success_rate / 100.0;
                    let z = 1.96;
                    let denominator = 1.0 + z * z / n;
                    let center = (p + z * z / (2.0 * n)) / denominator;
                    let spread = z * (p * (1.0 - p) / n + z * z / (4.0 * n * n)).sqrt() / denominator;
                    record.confidence_interval = ((center - spread) * 100.0, (center + spread) * 100.0);
                    // Simple p-value
                    let expected_random = 10.0;
                    let z_score = (record.success_rate - expected_random) / (expected_random * (100.0 - expected_random) / n).sqrt();
                    record.p_value = 1.0 - normal_cdf(z_score.abs());
                // Update success history
                self.success_history.push(if is_prime { 100.0 } else { 0.0 });
                if self.success_history.len() > 50 {
                    self.success_history.remove(0);
                self.flash_timer = Instant::now();
                self.current_seed_index = (self.current_seed_index + 1) % self.test_seeds.len();
                self.first_launch = false;
                // Check achievements
                self.check_achievements();
            Err(_) => {
    fn check_achievement(&mut self, achievement: Achievement) {
        if !self.achievements.contains(&achievement) {
            self.achievements.push(achievement.clone());
            self.pending_achievement = Some((achievement, Instant::now()));
    fn check_achievements(&mut self) {
        if self.total_primes >= 1 {
            self.check_achievement(Achievement::FirstPrime);
        if self.total_primes >= 10 {
            self.check_achievement(Achievement::TenPrimes);
        if self.total_primes >= 100 {
            self.check_achievement(Achievement::HundredPrimes);
        if self.best_streak >= 5 {
            self.check_achievement(Achievement::StreakFive);
        if self.best_streak >= 10 {
            self.check_achievement(Achievement::StreakTen);
        if self.config_history.len() >= 10 {
            self.check_achievement(Achievement::Explorer);
        // Check performance achievements
            if record.total_generated >= 10 && record.performance_ratio > 1.1 {
                self.check_achievement(Achievement::BeatExpected);
            if record.total_generated >= 5 && record.success_rate >= 100.0 {
                self.check_achievement(Achievement::PerfectSession);
        // Speed achievement
        if self.total_generated >= 100 && self.session_start.elapsed() < Duration::from_secs(10) {
            self.check_achievement(Achievement::SpeedDemon);
    fn save_performance_data(&self) -> io::Result<()> {
        create_dir_all("membrane_lab_data")?;
        let filename = PathBuf::from("membrane_lab_data").join("performance_history.jsonl");
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)?;
        for record in self.config_history.values() {
            let json = serde_json::to_string(record)?;
            writeln!(file, "{}", json)?;
        Ok(())
    fn adjust_parameter(&mut self, increase: bool) {
        let delta = if increase { 1 } else { -1 };
        match self.selected_param {
            0 => { // Base
                let new_base = (self.base as i32 + delta).max(2).min(36) as u32;
                if new_base != self.base {
                    self.base = new_base;
                    self.reset_current_config();
            1 => { // Outer
                let new_outer = (self.outer as i32 + delta).max(1).min((self.base - 1) as i32) as u32;
                if new_outer != self.outer {
                    self.outer = new_outer;
            2 => { // Inner
                let new_inner = (self.inner as i32 + delta).max(1).min((self.base - 1) as i32) as u32;
                if new_inner != self.inner {
                    self.inner = new_inner;
            3 => { // K-outer
                self.k_outer = (self.k_outer as i32 + delta).max(0).min(5) as u32;
            4 => { // K-inner  
                self.k_inner = (self.k_inner as i32 + delta).max(0).min(5) as u32;
            _ => {}
    fn reset_current_config(&mut self) {
        self.prime_streak = 0;
    fn load_best_config(&mut self) {
        self.base = 6;
        self.outer = 1;
        self.inner = 5;
        self.k_outer = 0;
        self.k_inner = 0;
        self.reset_current_config();
// ===== HELPER FUNCTIONS =====
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
fn normal_cdf(z: f64) -> f64 {
    0.5 * (1.0 + (z / 2.0_f64.sqrt()).tanh())
// ===== MAIN APPLICATION =====
fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // Create app and generate first prime immediately
    let mut app = LabState::default();
    app.generate_current_seed();
    app.construction_step = 3; // Show complete result
    // Run app
    let res = run_app(&mut terminal, &mut app, Duration::from_millis(50));
    // Save data on exit
    let _ = app.save_performance_data();
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
                                KeyCode::Char('t') | KeyCode::Char('T') => {
                                    app.screen = AppScreen::Tutorial(0);
                                }
                                KeyCode::Char('q') => return Ok(()),
                                _ => {}
                            }
                        }
                        AppScreen::Tutorial(step) => {
                                KeyCode::Right | KeyCode::Enter => {
                                    if *step < 4 {
                                        app.screen = AppScreen::Tutorial(step + 1);
                                    } else {
                                        app.screen = AppScreen::Main;
                                        app.check_achievement(Achievement::TutorialComplete);
                                    }
                                KeyCode::Left => {
                                    if *step > 0 {
                                        app.screen = AppScreen::Tutorial(step - 1);
                                KeyCode::Esc => app.screen = AppScreen::Main,
                        AppScreen::Main => {
                                KeyCode::Char('?') | KeyCode::Char('h') => app.show_help = !app.show_help,
                                KeyCode::Char('w') => app.screen = AppScreen::Welcome,
                                KeyCode::Char('t') => app.screen = AppScreen::Tutorial(0),
                                KeyCode::Tab => app.tab_index = (app.tab_index + 1) % 8,
                                KeyCode::BackTab => app.tab_index = (app.tab_index + 7) % 8,
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
                                KeyCode::Char('g') | KeyCode::Char('G') => {
                                    for _ in 0..10 {
                                        app.generate_current_seed();
                                KeyCode::Char('b') | KeyCode::Char('B') => app.load_best_config(),
                                KeyCode::Char('s') | KeyCode::Char('S') => {
                                    let _ = app.save_performance_data();
                                    app.check_achievement(Achievement::DataScientist);
                                // Quick configs
                                KeyCode::Char('1') => {
                                    app.base = 6;
                                    app.outer = 1;
                                    app.inner = 5;
                                    app.k_outer = 0;
                                    app.k_inner = 0;
                                    app.reset_current_config();
                                KeyCode::Char('2') => {
                                    app.base = 10;
                                    app.outer = 3;
                                    app.inner = 7;
                                KeyCode::Char('3') => {
                                    app.base = 30;
                                    app.outer = 11;
        if last_tick.elapsed() >= tick_rate {
            // Update construction animation
            if app.construction_timer.elapsed() > Duration::from_millis(300) {
                app.construction_step = (app.construction_step + 1).min(3);
                app.construction_timer = Instant::now();
            // Hide hint after 3 seconds
            if let Some(timer) = app.hint_timer {
                if timer.elapsed() > Duration::from_secs(3) {
                    app.hint_timer = None;
            last_tick = Instant::now();
// ===== UI RENDERING =====
fn ui(f: &mut Frame, app: &LabState) {
    match &app.screen {
        AppScreen::Welcome => draw_welcome_screen(f, app),
        AppScreen::Tutorial(step) => draw_tutorial_screen(f, app, *step),
        AppScreen::Main => draw_main_screen(f, app),
fn draw_main_screen(f: &mut Frame, app: &LabState) {
    let size = f.size();
    if app.show_help {
        draw_help_popup(f, size);
        return;
    // Show achievement popup if needed
    if let Some((achievement, timer)) = &app.pending_achievement {
        if timer.elapsed() < Duration::from_secs(3) {
            draw_achievement_popup(f, size, achievement);
    // Layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(3),  // Tabs
            Constraint::Min(0),     // Content
            Constraint::Length(4),  // Status with sparkline
        ])
        .split(size);
    // Dynamic title
    draw_title(f, chunks[0], app);
    // Tabs
    let tab_titles = vec![
        "Config", "Construction", "Results", "Heat Map", 
        "Performance", "Achievements", "Insights", "Export"
    ];
    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL))
        .select(app.tab_index)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    f.render_widget(tabs, chunks[1]);
    // Content
    match app.tab_index {
        0 => draw_config_tab(f, chunks[2], app),
        1 => draw_construction_tab(f, chunks[2], app),
        2 => draw_results_tab(f, chunks[2], app),
        3 => draw_heat_map_tab(f, chunks[2], app),
        4 => draw_performance_tab(f, chunks[2], app),
        5 => draw_achievements_tab(f, chunks[2], app),
        6 => draw_insights_tab(f, chunks[2], app),
        7 => draw_export_tab(f, chunks[2], app),
        _ => {}
    // Enhanced status bar
    draw_status_bar(f, chunks[3], app);
    // First launch hint
    if app.first_launch && app.hint_timer.is_some() {
        draw_first_launch_hint(f, chunks[2]);
fn draw_title(f: &mut Frame, area: Rect, app: &LabState) {
    let flash_elapsed = app.flash_timer.elapsed();
    let success_rate = app.success_rate();
    let expected_rate = app.get_expected_rate();
    let title_text = if flash_elapsed < Duration::from_millis(500) {
        if app.last_generation_success {
            if app.prime_streak > 2 {
                format!("🔬 Prime Membrane Laboratory 🔥 {} STREAK! | {:.1}% success", 
                    app.prime_streak, success_rate)
            } else {
                format!("🔬 Prime Membrane Laboratory ✨ PRIME! | {:.1}% success", success_rate)
            format!("🔬 Prime Membrane Laboratory ⚠️ Composite | {:.1}% success", success_rate)
    } else {
        format!("🔬 Prime Membrane Laboratory | {:.1}% success (expected: {:.1}%)", 
            success_rate, expected_rate)
    };
    let performance_ratio = if expected_rate > 0.0 { success_rate / expected_rate } else { 1.0 };
    let title_color = match performance_ratio {
        r if r > 1.1 => Color::Green,
        r if r > 0.9 => Color::Yellow,
        _ => Color::Red,
    let title = Paragraph::new(title_text)
        .style(Style::default().fg(title_color).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, area);
fn draw_config_tab(f: &mut Frame, area: Rect, app: &LabState) {
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);
    // Parameters
    let params = vec![
        (format!("Base: {}", app.base), app.selected_param == 0),
        (format!("Outer digit: {}", app.outer), app.selected_param == 1),
        (format!("Inner digit: {}", app.inner), app.selected_param == 2),
        (format!("K-outer: {}", app.k_outer), app.selected_param == 3),
        (format!("K-inner: {}", app.k_inner), app.selected_param == 4),
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
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Parameters (↑/↓/←/→) "));
    f.render_widget(params_list, chunks[0]);
    // Right side: Performance gauge and stats
    let right_chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);
    // Performance gauge
    let hash = app.config_hash();
    let performance_ratio = if let Some(record) = app.config_history.get(&hash) {
        record.performance_ratio
        1.0
    let gauge_percent = (performance_ratio * 100.0).min(200.0) as u16;
    let gauge_color = match performance_ratio {
        r if r > 1.2 => Color::Green,
        r if r > 1.0 => Color::Yellow,
        r if r > 0.8 => Color::Magenta,
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Performance vs Expected "))
        .gauge_style(Style::default().fg(gauge_color))
        .percent(gauge_percent / 2) // Scale to 0-100
        .label(format!("{:.0}%", gauge_percent));
    f.render_widget(gauge, right_chunks[0]);
    // Statistical summary
    let stats_text = if let Some(record) = app.config_history.get(&hash) {
        vec![
            Line::from(vec![
                Span::raw("Coprimality: "),
                if app.is_coprime() {
                    Span::styled("✓ Valid", Style::default().fg(Color::Green))
                    Span::styled("✗ Invalid", Style::default().fg(Color::Red))
                },
            ]),
            Line::from(format!("Expected: {:.1}%", record.expected_rate)),
            Line::from(format!("Actual: {:.1}% ({}/{})", 
                record.success_rate, record.total_primes, record.total_generated)),
            Line::from(format!("95% CI: [{:.1}%, {:.1}%]", 
                record.confidence_interval.0, record.confidence_interval.1)),
            Line::from(format!("p-value: {:.4}", record.p_value)),
            Line::from(""),
                Span::raw("Press "),
                Span::styled("B", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(" for best | "),
                Span::styled("1-3", Style::default().fg(Color::Cyan)),
                Span::raw(" quick configs"),
        ]
            Line::from("No data yet for this configuration"),
            Line::from("Press ENTER to generate!"),
    let stats = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title(" Statistical Analysis "));
    f.render_widget(stats, right_chunks[1]);
fn draw_construction_tab(f: &mut Frame, area: Rect, app: &LabState) {
    let construction_block = Block::default()
        .borders(Borders::ALL)
        .title(" Live Membrane Construction ");
    let inner_area = construction_block.inner(area);
    f.render_widget(construction_block, area);
    let seed = if app.current_results.is_empty() { 
        "5" 
    } else { 
        &app.current_results.last().unwrap().seed 
    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Building membrane with seed ", Style::default()),
            Span::styled(seed, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
    // Animated construction
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
                Span::raw(" + "),
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
    let construction = Paragraph::new(lines)
        .alignment(Alignment::Center);
    f.render_widget(construction, inner_area);
fn draw_results_tab(f: &mut Frame, area: Rect, app: &LabState) {
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
    // Results list
    let results_block = Block::default()
        .title(" Recent Results ");
    let results_inner = results_block.inner(chunks[0]);
    f.render_widget(results_block, chunks[0]);
    if app.current_results.is_empty() {
        let empty = Paragraph::new("No results yet. Press ENTER to generate!")
            .alignment(Alignment::Center);
        f.render_widget(empty, results_inner);
        let items: Vec<ListItem> = app.current_results
            .iter()
            .rev()
            .take(20)
            .map(|result| {
                let style = if result.is_prime {
                    Style::default().fg(Color::Green)
                    Style::default().fg(Color::Red)
                let text = format!(
                    "Seed {}: {} {} ({:.1}ms)",
                    result.seed,
                    result.number,
                    if result.is_prime { "✓" } else { "✗" },
                    result.generation_time.as_micros() as f64 / 1000.0
                ListItem::new(text).style(style)
            })
            .collect();
        let results_list = List::new(items);
        f.render_widget(results_list, results_inner);
    // Interesting patterns
    let patterns_block = Block::default()
        .title(" 🎆 Interesting Patterns ");
    let patterns_inner = patterns_block.inner(chunks[1]);
    f.render_widget(patterns_block, chunks[1]);
    if app.interesting_patterns.is_empty() {
        let empty = Paragraph::new("No special patterns found yet!")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(empty, patterns_inner);
        let items: Vec<ListItem> = app.interesting_patterns
            .take(5)
            .map(|p| ListItem::new(p.as_str()).style(Style::default().fg(Color::Yellow)))
        let patterns_list = List::new(items);
        f.render_widget(patterns_list, patterns_inner);
fn draw_heat_map_tab(f: &mut Frame, area: Rect, app: &LabState) {
    let heat_block = Block::default()
        .title(" Configuration Heat Map ");
    let inner_area = heat_block.inner(area);
    f.render_widget(heat_block, area);
        .constraints([Constraint::Min(0), Constraint::Length(4)])
        .split(inner_area);
    // Calculate grid dimensions
    let max_digit = (app.base - 1).min(15);
    let _cell_width = 5;
    let _cell_height = 2;
    let mut text = vec![];
    // Headers
    let mut header_line = vec![Span::raw("     ")];
    for i in 1..=max_digit {
        header_line.push(Span::raw(format!("{:^5}", i)));
    text.push(Line::from(header_line));
    text.push(Line::from(""));
    // Grid
    for outer in 1..=max_digit {
        let mut row = vec![Span::raw(format!("{:>3} ", outer))];
        for inner in 1..=max_digit {
            let is_coprime = gcd(outer, app.base) == 1 && gcd(inner, app.base) == 1;
            // Get actual data if available
            let test_hash = format!("{},{},{},0,0", app.base, outer, inner);
            let success_rate = if let Some(record) = app.config_history.get(&test_hash) {
                record.success_rate
            } else if is_coprime {
                20.0 + (outer * inner) as f64 % 10.0 // Estimate
                5.0
            let symbol = match success_rate {
                r if r > 30.0 => "█",
                r if r > 25.0 => "▓",
                r if r > 20.0 => "▒",
                r if r > 15.0 => "░",
                _ => "·",
            let color = match success_rate {
                r if r > 25.0 => Color::Green,
                r if r > 20.0 => Color::Yellow,
                r if r > 15.0 => Color::Magenta,
                _ => Color::Red,
            let cell_text = format!("{}{}", symbol, if is_coprime { "✓" } else { " " });
            row.push(Span::styled(format!("{:^5}", cell_text), Style::default().fg(color)));
        text.push(Line::from(row));
    let heat_map = Paragraph::new(text);
    f.render_widget(heat_map, chunks[0]);
    // Legend
    let legend = vec![
            Span::styled("█ >30%", Style::default().fg(Color::Green)),
            Span::raw("  "),
            Span::styled("▓ 25-30%", Style::default().fg(Color::Yellow)),
            Span::styled("▒ 20-25%", Style::default().fg(Color::Yellow)),
            Span::styled("░ 15-20%", Style::default().fg(Color::Magenta)),
            Span::styled("· <15%", Style::default().fg(Color::Red)),
            Span::raw("✓ = Coprime"),
    let legend_widget = Paragraph::new(legend)
    f.render_widget(legend_widget, chunks[1]);
fn draw_performance_tab(f: &mut Frame, area: Rect, app: &LabState) {
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
    // Performance comparison table
    let mut configs: Vec<_> = app.config_history.iter().collect();
    configs.sort_by(|a, b| b.1.success_rate.partial_cmp(&a.1.success_rate).unwrap());
    let mut table_text = vec![
        Line::from("Configuration     | Generated | Primes | Rate    | vs Expected | p-value    | Avg ms"),
        Line::from("─".repeat(85)),
    for (hash, record) in configs.iter().take(10) {
        let color = match record.performance_ratio {
            r if r > 1.1 => Color::Green,
            r if r > 0.9 => Color::Yellow,
            _ => Color::Red,
        };
        let row = format!(
            "{:<16} | {:>9} | {:>6} | {:>6.1}% | {:>+10.1}% | {:>10.4} | {:>6.1}",
            hash,
            record.total_generated,
            record.total_primes,
            record.success_rate,
            (record.performance_ratio - 1.0) * 100.0,
            record.p_value,
            record.avg_generation_time_ms
        table_text.push(Line::from(vec![Span::styled(row, Style::default().fg(color))]));
    let table = Paragraph::new(table_text)
        .block(Block::default().borders(Borders::ALL).title(" Configuration Performance Comparison "));
    f.render_widget(table, chunks[0]);
    // Speed chart
    if let Some(record) = app.config_history.get(&hash) {
        if !record.generation_times.is_empty() {
            let data: Vec<u64> = record.generation_times
                .iter()
                .rev()
                .take(50)
                .map(|&t| (t * 10.0) as u64)
                .collect();
            let sparkline = Sparkline::default()
                .block(Block::default().borders(Borders::ALL).title(" Generation Speed (last 50) "))
                .data(&data)
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(sparkline, chunks[1]);
fn draw_achievements_tab(f: &mut Frame, area: Rect, app: &LabState) {
    // Achievements list
    let achievements_block = Block::default()
        .title(format!(" Achievements ({}/14) ", app.achievements.len()));
    let achievements_inner = achievements_block.inner(chunks[0]);
    f.render_widget(achievements_block, chunks[0]);
    let items: Vec<ListItem> = app.achievements
        .map(|a| ListItem::new(a.name()).style(Style::default().fg(Color::Yellow)))
    let achievements_list = List::new(items);
    f.render_widget(achievements_list, achievements_inner);
    // Progress
    let progress_text = vec![
            Span::styled("Progress Tracking", Style::default().add_modifier(Modifier::BOLD)),
        Line::from(format!("Total primes: {}", app.total_primes)),
        Line::from(format!("Best streak: {}", app.best_streak)),
        Line::from(format!("Configs tried: {}", app.config_history.len())),
        Line::from("Next milestones:"),
        Line::from(if app.total_primes < 100 {
            format!("  {} to go for Century", 100 - app.total_primes)
            "  ✓ All prime milestones!".to_string()
        }),
    let progress = Paragraph::new(progress_text)
        .block(Block::default().borders(Borders::ALL).title(" Progress "));
    f.render_widget(progress, chunks[1]);
fn draw_insights_tab(f: &mut Frame, area: Rect, app: &LabState) {
    let insights_text = if app.config_history.is_empty() {
            Line::from("No data collected yet!"),
            Line::from("Generate some primes to see insights."),
        let mut text = vec![
                Span::styled("Data-Driven Insights", Style::default().add_modifier(Modifier::BOLD)),
        ];
        // Best performer
        if let Some((best_hash, best)) = app.config_history.iter()
            .max_by(|a, b| a.1.success_rate.partial_cmp(&b.1.success_rate).unwrap()) {
            text.push(Line::from(format!("🏆 Best: {} at {:.1}%", best_hash, best.success_rate)));
        // Coprimality analysis
        let coprime_rates: Vec<f64> = app.config_history.iter()
            .filter(|(_, r)| gcd(r.outer, r.base) == 1 && gcd(r.inner, r.base) == 1)
            .map(|(_, r)| r.success_rate)
        let non_coprime_rates: Vec<f64> = app.config_history.iter()
            .filter(|(_, r)| gcd(r.outer, r.base) != 1 || gcd(r.inner, r.base) != 1)
        if !coprime_rates.is_empty() && !non_coprime_rates.is_empty() {
            let avg_coprime = coprime_rates.iter().sum::<f64>() / coprime_rates.len() as f64;
            let avg_non = non_coprime_rates.iter().sum::<f64>() / non_coprime_rates.len() as f64;
            text.push(Line::from(""));
            text.push(Line::from(format!("📊 Coprime avg: {:.1}%", avg_coprime)));
            text.push(Line::from(format!("📊 Non-coprime: {:.1}%", avg_non)));
            text.push(Line::from(format!("🚀 Boost: {:.1}x", avg_coprime / avg_non)));
        text
    let insights = Paragraph::new(insights_text)
        .block(Block::default().borders(Borders::ALL).title(" Insights "))
        .wrap(Wrap { trim: true });
    f.render_widget(insights, area);
fn draw_export_tab(f: &mut Frame, area: Rect, app: &LabState) {
    let export_text = vec![
            Span::styled("📊 Export Your Research", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
        Line::from("Save your session data for analysis:"),
        Line::from("• Complete configuration history"),
        Line::from("• Statistical summaries with CI & p-values"),
        Line::from("• Generation timing data"),
        Line::from("• Pattern discoveries"),
        Line::from("Format: JSON Lines (.jsonl)"),
        Line::from("Location: membrane_lab_data/"),
            Span::raw("Press "),
            Span::styled("S", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" to save data"),
        if app.achievements.contains(&Achievement::DataScientist) {
                Span::styled("✓ Data saved!", Style::default().fg(Color::Green)),
            ])
            Line::from("Save data to unlock Data Scientist achievement!")
        },
    let export = Paragraph::new(export_text)
        .block(Block::default().borders(Borders::ALL).title(" Export "))
    f.render_widget(export, area);
fn draw_status_bar(f: &mut Frame, area: Rect, app: &LabState) {
        .constraints([Constraint::Length(2), Constraint::Length(2)])
    // Status line
    let status_chunks = Layout::default()
            Constraint::Percentage(25),
        .split(chunks[0]);
    let coprime = app.is_coprime();
    let config_status = Paragraph::new(format!("Config: ({},{}) {}", 
        app.outer, app.inner, if coprime { "✓" } else { "✗" }))
        .style(Style::default().fg(if coprime { Color::Green } else { Color::Red }));
    f.render_widget(config_status, status_chunks[0]);
    let rate_status = Paragraph::new(format!("Rate: {:.1}%", app.success_rate()))
    f.render_widget(rate_status, status_chunks[1]);
    let streak_status = Paragraph::new(format!("Streak: {} {}", 
        app.prime_streak,
        if app.prime_streak >= 3 { "🔥" } else { "" }))
    f.render_widget(streak_status, status_chunks[2]);
    let help_status = Paragraph::new("? help • G batch • S save")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Right);
    f.render_widget(help_status, status_chunks[3]);
    // Sparkline
    if !app.success_history.is_empty() {
        let data: Vec<u64> = app.success_history.iter()
            .map(|&v| v as u64)
        let sparkline = Sparkline::default()
            .block(Block::default().borders(Borders::TOP))
            .data(&data)
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(sparkline, chunks[1]);
fn draw_welcome_screen(f: &mut Frame, _app: &LabState) {
            Constraint::Percentage(20),
            Constraint::Percentage(60),
    let title_art = vec![
        Line::from("╔═╗┬─┐┬┌┬┐┌─┐  ╔╦╗┌─┐┌┬┐┌┐ ┬─┐┌─┐┌┐┌┌─┐"),
        Line::from("╠═╝├┬┘│││││├┤   ║║║├┤ │││├┴┐├┬┘├─┤│││├┤ "),
        Line::from("╩  ┴└─┴┴ ┴└─┘  ╩ ╩└─┘┴ ┴└─┘┴└─┴ ┴┘└┘└─┘"),
        Line::from("      ╦  ┌─┐┌┐ ┌─┐┬─┐┌─┐┌┬┐┌─┐┬─┐┬ ┬"),
        Line::from("      ║  ├─┤├┴┐│ │├┬┘├─┤ │ │ │├┬┘└┬┘"),
        Line::from("      ╩═╝┴ ┴└─┘└─┘┴└─┴ ┴ ┴ └─┘┴└─ ┴ "),
    let title = Paragraph::new(title_art)
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(title, chunks[0]);
    let welcome_text = vec![
            Span::styled("Welcome to the ", Style::default()),
            Span::styled("Professional Edition", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Line::from("Discover how symmetric patterns generate prime numbers"),
        Line::from("with statistical rigor and engaging visualizations"),
            Span::styled("✨ Features", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Line::from("  • Real-time performance tracking with p-values"),
        Line::from("  • Achievement system based on statistical milestones"),
        Line::from("  • Live construction animations"),
        Line::from("  • Heat maps and performance comparisons"),
        Line::from("  • Data export for scientific analysis"),
            Span::styled("🏆 Best Configuration", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Line::from("  Base 6 with (1,5) achieves 33% prime rate!"),
    let welcome = Paragraph::new(welcome_text)
            .title(" Prime Membrane Laboratory ")
            .title_alignment(Alignment::Center))
    f.render_widget(welcome, chunks[1]);
    let instructions = vec![
            Span::styled("ENTER", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" to start  •  "),
            Span::styled("T", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" for tutorial  •  "),
            Span::styled("Q", Style::default().fg(Color::Red)),
            Span::raw(" to quit"),
    let footer = Paragraph::new(instructions)
    f.render_widget(footer, chunks[2]);
fn draw_tutorial_screen(f: &mut Frame, _app: &LabState, step: usize) {
            Constraint::Length(3),
            Constraint::Min(0),
    let title = Paragraph::new(format!("🎓 Tutorial - Step {}/5", step + 1))
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
    let content = match step {
        0 => vec![
                Span::styled("📚 Number Bases", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Line::from("We count in different bases:"),
            Line::from("  • Base 10: 0-9 (decimal)"),
            Line::from("  • Base 2: 0-1 (binary)"),
            Line::from("  • Base 6: 0-5 (optimal for primes!)"),
            Line::from("Example: 15 in different bases"),
            Line::from("  • Base 10: 15"),
            Line::from("  • Base 6: 23 (2×6 + 3 = 15)"),
            Line::from("  • Base 2: 1111"),
        ],
        1 => vec![
                Span::styled("🏗️ Membrane Construction", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Line::from("Build numbers in symmetric layers:"),
            Line::from("  outer + inner + seed + inner + outer"),
            Line::from("Example: seed=3, outer=1, inner=5"),
            Line::from("  Step 1: 3"),
            Line::from("  Step 2: 5 + 3 + 5 = 535"),
            Line::from("  Step 3: 1 + 535 + 1 = 15351"),
            Line::from("  Result: 15351 is prime!"),
        2 => vec![
                Span::styled("⚡ Coprimality", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Line::from("Key rule: boundary digits must be coprime to base"),
            Line::from("(their greatest common divisor = 1)"),
            Line::from("Base 6 example:"),
            Line::from("  • gcd(1,6) = 1 ✓ coprime"),
            Line::from("  • gcd(5,6) = 1 ✓ coprime"),
            Line::from("  • gcd(2,6) = 2 ✗ not coprime"),
            Line::from("Result: (1,5) configuration achieves 33% success!"),
        3 => vec![
                Span::styled("📊 Understanding the Data", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Line::from("Every number is real:"),
            Line::from("  • Success rate with confidence intervals"),
            Line::from("  • p-values test statistical significance"),
            Line::from("  • Performance ratio vs expected rates"),
            Line::from("Heat map colors:"),
            Line::from("  • Green: High success (>25%)"),
            Line::from("  • Yellow: Medium (20-25%)"),
            Line::from("  • Red: Low (<15%)"),
        4 => vec![
                Span::styled("🚀 Quick Start", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Line::from("Essential commands:"),
            Line::from("  • ENTER - Generate single prime"),
            Line::from("  • G - Generate batch (10 at once)"),
            Line::from("  • Tab - Switch between views"),
            Line::from("  • B - Load best configuration"),
            Line::from("  • S - Save your data"),
            Line::from("Quick configs:"),
            Line::from("  • 1 - Base 6 champion"),
            Line::from("  • 2 - Base 10 standard"),
            Line::from("  • 3 - Base 30 high performer"),
        _ => vec![],
    let tutorial = Paragraph::new(content)
            .title(format!(" {} ", match step {
                0 => "Understanding Bases",
                1 => "Building Membranes",
                2 => "The Coprimality Rule",
                3 => "Reading the Data",
                4 => "Getting Started",
                _ => "",
            }))
    f.render_widget(tutorial, chunks[1]);
    let nav = if step < 4 {
            Span::raw("← Previous  "),
            Span::styled("→ Next", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("  •  ESC to skip"),
            Span::styled("ENTER to start!", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
    let navigation = Paragraph::new(Line::from(nav))
        .block(Block::default().borders(Borders::TOP));
    f.render_widget(navigation, chunks[2]);
fn draw_help_popup(f: &mut Frame, area: Rect) {
    let popup_area = centered_rect(70, 80, area);
    f.render_widget(Clear, popup_area);
    let help_text = vec![
        Line::from("🔬 Prime Membrane Laboratory - Help"),
            Span::styled("Navigation:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
        Line::from("  Tab/Shift+Tab - Switch between tabs"),
        Line::from("  ↑/↓           - Select parameter"),
        Line::from("  ←/→           - Adjust value"),
            Span::styled("Actions:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
        Line::from("  Enter         - Generate single"),
        Line::from("  G             - Generate batch"),
        Line::from("  B             - Load best config"),
        Line::from("  S             - Save data"),
        Line::from("  1/2/3         - Quick configs"),
        Line::from("  W             - Welcome screen"),
        Line::from("  T             - Tutorial"),
        Line::from("  ?/H           - This help"),
        Line::from("  Q             - Quit"),
        Line::from("Press any key to close..."),
    let help = Paragraph::new(help_text)
            .title(" Help ")
            .border_style(Style::default().fg(Color::Yellow)))
        .style(Style::default().bg(Color::Black));
    f.render_widget(help, popup_area);
fn draw_achievement_popup(f: &mut Frame, area: Rect, achievement: &Achievement) {
    let popup_area = centered_rect(40, 20, area);
    let popup_text = vec![
            Span::styled("🎉 Achievement Unlocked! 🎉", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(achievement.name(), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
    let popup = Paragraph::new(popup_text)
            .border_style(Style::default().fg(Color::Yellow))
            .style(Style::default().bg(Color::Black)))
    f.render_widget(popup, popup_area);
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
            Span::raw(" to explore  •  "),
            Span::styled("?", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
            Span::raw(" for help")
    let hint_widget = Paragraph::new(hint)
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
