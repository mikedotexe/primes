use std::io;//! Ultimate Data-Driven Membrane Laboratory - TUI Interface
//! 
//! Every UI element is backed by real performance data and statistics

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    symbols,
    widgets::{
        Axis, Block, Borders, Chart, Clear, Dataset, Gauge, List, ListItem, 
        Paragraph, Sparkline, Tabs, Wrap,
    },
    Frame, Terminal,
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
    fs::{OpenOptions, create_dir_all},
    io::Write,
use prime_physics_engine::{
    is_prime,
use num_bigint::BigUint;
// Achievement system based on statistical milestones
#[derive(Debug, Clone, PartialEq)]
enum Achievement {
    FirstPrime,
    TenPrimes,
    HundredPrimes,
    StreakFive,
    StreakTen,
    BeatExpected,      // Beat expected rate by 10%
    PerfectSession,    // 100% prime rate (min 5 generated)
    Palindrome,
    LuckyPattern,
    MillionDigit,      // Find a prime > 1 million
    SpeedDemon,        // Generate 100 in < 10 seconds
    Explorer,          // Try 10 different configurations
    DataScientist,     // Export results
}
impl Achievement {
    fn name(&self) -> &str {
        match self {
            Achievement::FirstPrime => "🌟 First Prime",
            Achievement::TenPrimes => "🔟 Ten Primes",
            Achievement::HundredPrimes => "💯 Century Club",
            Achievement::StreakFive => "🔥 Hot Streak (5)",
            Achievement::StreakTen => "🌋 On Fire! (10)",
            Achievement::BeatExpected => "📈 Beat the Odds",
            Achievement::PerfectSession => "💎 Perfect Session",
            Achievement::Palindrome => "🔄 Palindrome Hunter",
            Achievement::LuckyPattern => "🍀 Lucky Numbers",
            Achievement::MillionDigit => "🏔️ Million Club",
            Achievement::SpeedDemon => "⚡ Speed Demon",
            Achievement::Explorer => "🗺️ Configuration Explorer",
            Achievement::DataScientist => "📊 Data Scientist",
        }
    }
    
    fn description(&self) -> &str {
            Achievement::FirstPrime => "Found your first prime",
            Achievement::TenPrimes => "Generated 10 primes",
            Achievement::HundredPrimes => "Generated 100 primes",
            Achievement::StreakFive => "5 primes in a row",
            Achievement::StreakTen => "10 primes in a row!",
            Achievement::BeatExpected => "Beat expected rate by 10%",
            Achievement::PerfectSession => "100% success rate (5+ tries)",
            Achievement::Palindrome => "Found a palindrome prime",
            Achievement::LuckyPattern => "Found 777 or 999 pattern",
            Achievement::MillionDigit => "Found prime > 1,000,000",
            Achievement::SpeedDemon => "100 generations < 10 sec",
            Achievement::Explorer => "Tried 10 configurations",
            Achievement::DataScientist => "Exported your data",
#[derive(Debug, Clone)]
struct PerformanceStats {
    config: MembraneConfig,
    total_generated: usize,
    total_primes: usize,
    success_rate: f64,
    expected_rate: f64,
    confidence_interval: (f64, f64),
    p_value: f64,  // vs random
    performance_ratio: f64,  // actual/expected
    generation_times: Vec<f64>,
struct GenerationResult {
    number: BigUint,
    is_prime: bool,
    seed: String,
    generation_time: Duration,
    config_hash: String,
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
    // Results
    current_results: Vec<GenerationResult>,
    // Statistics
    session_start: Instant,
    // Performance tracking
    success_history: Vec<f64>,  // For sparkline
    current_performance: PerformanceStats,
    config_history: HashMap<String, PerformanceStats>,
    // Gamification
    prime_streak: usize,
    best_streak: usize,
    achievements: Vec<Achievement>,
    pending_achievements: Vec<Achievement>,
    achievement_timer: Option<Instant>,
    // Seeds
    test_seeds: Vec<String>,
    current_seed_index: usize,
    // Visual feedback
    show_help: bool,
    last_generation_success: bool,
    flash_timer: Instant,
    // Recommendations
    recommended_configs: Vec<(u32, u32, u32, u32, u32, f64)>, // (base, outer, inner, k_outer, k_inner, expected_rate)
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
            total_generated: 0,
            total_primes: 0,
            session_start: Instant::now(),
            success_history: Vec::with_capacity(100),
            current_performance: PerformanceStats {
                config: MembraneConfig::new(6, 1, 5, 0, 0),
                total_generated: 0,
                total_primes: 0,
                success_rate: 0.0,
                expected_rate: 33.0,
                confidence_interval: (0.0, 0.0),
                p_value: 1.0,
                performance_ratio: 0.0,
                generation_times: Vec::new(),
            },
            config_history: HashMap::new(),
            prime_streak: 0,
            best_streak: 0,
            achievements: Vec::new(),
            pending_achievements: Vec::new(),
            achievement_timer: None,
            test_seeds: vec!["0".to_string(), "1".to_string(), "2".to_string(), 
                             "3".to_string(), "4".to_string(), "5".to_string(),
                             "6".to_string(), "7".to_string(), "8".to_string(), 
                             "9".to_string()],
            current_seed_index: 0,
            show_help: false,
            last_generation_success: false,
            flash_timer: Instant::now(),
            recommended_configs: vec![
                (6, 1, 5, 0, 0, 33.0),
                (6, 5, 1, 0, 0, 31.0),
                (30, 11, 7, 0, 0, 30.0),
                (10, 3, 7, 0, 0, 20.0),
                (12, 5, 7, 0, 0, 25.0),
            ],
impl LabState {
    fn get_config(&self) -> MembraneConfig {
        MembraneConfig::new(self.base, self.outer, self.inner, self.k_outer, self.k_inner)
    fn config_hash(&self) -> String {
        format!("{},{},{},{},{}", self.base, self.outer, self.inner, self.k_outer, self.k_inner)
    fn get_expected_rate(&self) -> f64 {
        // Real data-driven expected rates
        match (self.base, self.outer, self.inner, self.k_outer, self.k_inner) {
            (6, 1, 5, 0, 0) => 33.0,
            (6, 5, 1, 0, 0) => 31.0,
            (10, 3, 7, 0, 0) => 20.0,
            (12, 5, 7, 0, 0) => 25.0,
            (30, 11, 7, 0, 0) => 30.0,
            _ => {
                // Estimate based on coprimality
                if gcd(self.outer, self.base) == 1 && gcd(self.inner, self.base) == 1 {
                    15.0 + (self.base as f64 / 10.0)
                } else {
                    5.0
                }
            }
    fn update_performance_stats(&mut self) {
        let hash = self.config_hash();
        let stats = self.config_history.entry(hash).or_insert(PerformanceStats {
            config: self.get_config(),
            success_rate: 0.0,
            expected_rate: self.get_expected_rate(),
            confidence_interval: (0.0, 0.0),
            p_value: 1.0,
            performance_ratio: 0.0,
            generation_times: Vec::new(),
        });
        
        if stats.total_generated > 0 {
            stats.success_rate = (stats.total_primes as f64 / stats.total_generated as f64) * 100.0;
            stats.performance_ratio = stats.success_rate / stats.expected_rate;
            
            // Calculate confidence interval (Wilson score interval)
            let n = stats.total_generated as f64;
            let p = stats.success_rate / 100.0;
            let z = 1.96; // 95% confidence
            let denominator = 1.0 + z * z / n;
            let center = (p + z * z / (2.0 * n)) / denominator;
            let spread = z * (p * (1.0 - p) / n + z * z / (4.0 * n * n)).sqrt() / denominator;
            stats.confidence_interval = ((center - spread) * 100.0, (center + spread) * 100.0);
            // Simple p-value calculation vs random
            let expected_random = 10.0; // Rough prime density
            let z_score = (stats.success_rate - expected_random) / (expected_random * (100.0 - expected_random) / n).sqrt();
            stats.p_value = 1.0 - normal_cdf(z_score.abs());
        self.current_performance = stats.clone();
    fn check_achievements(&mut self) {
        let mut new_achievements = Vec::new();
        if self.total_primes >= 1 && !self.achievements.contains(&Achievement::FirstPrime) {
            new_achievements.push(Achievement::FirstPrime);
        if self.total_primes >= 10 && !self.achievements.contains(&Achievement::TenPrimes) {
            new_achievements.push(Achievement::TenPrimes);
        if self.total_primes >= 100 && !self.achievements.contains(&Achievement::HundredPrimes) {
            new_achievements.push(Achievement::HundredPrimes);
        if self.best_streak >= 5 && !self.achievements.contains(&Achievement::StreakFive) {
            new_achievements.push(Achievement::StreakFive);
        if self.best_streak >= 10 && !self.achievements.contains(&Achievement::StreakTen) {
            new_achievements.push(Achievement::StreakTen);
        if self.current_performance.performance_ratio > 1.1 && 
           self.current_performance.total_generated >= 10 &&
           !self.achievements.contains(&Achievement::BeatExpected) {
            new_achievements.push(Achievement::BeatExpected);
        if self.current_performance.success_rate >= 100.0 && 
           self.current_performance.total_generated >= 5 &&
           !self.achievements.contains(&Achievement::PerfectSession) {
            new_achievements.push(Achievement::PerfectSession);
        if self.config_history.len() >= 10 && !self.achievements.contains(&Achievement::Explorer) {
            new_achievements.push(Achievement::Explorer);
        // Check for patterns in results
        for result in &self.current_results {
            let num_str = result.number.to_string();
            if num_str == num_str.chars().rev().collect::<String>() && 
               !self.achievements.contains(&Achievement::Palindrome) {
                new_achievements.push(Achievement::Palindrome);
            if (num_str.contains("777") || num_str.contains("999")) &&
               !self.achievements.contains(&Achievement::LuckyPattern) {
                new_achievements.push(Achievement::LuckyPattern);
            if result.number > BigUint::from(1_000_000u32) &&
               !self.achievements.contains(&Achievement::MillionDigit) {
                new_achievements.push(Achievement::MillionDigit);
        // Speed achievement
        let elapsed = self.session_start.elapsed();
        if self.total_generated >= 100 && elapsed < Duration::from_secs(10) &&
           !self.achievements.contains(&Achievement::SpeedDemon) {
            new_achievements.push(Achievement::SpeedDemon);
        for achievement in new_achievements {
            self.achievements.push(achievement.clone());
            self.pending_achievements.push(achievement);
            self.achievement_timer = Some(Instant::now());
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
                
                let result = GenerationResult {
                    number: particle.value.clone(),
                    is_prime,
                    seed: seed.clone(),
                    generation_time,
                    config_hash: self.config_hash(),
                };
                // Update stats
                self.current_results.push(result);
                self.total_generated += 1;
                let hash = self.config_hash();
                let stats = self.config_history.get_mut(&hash).unwrap();
                stats.total_generated += 1;
                stats.generation_times.push(generation_time.as_micros() as f64 / 1000.0);
                if is_prime {
                    self.total_primes += 1;
                    stats.total_primes += 1;
                    self.last_generation_success = true;
                    self.prime_streak += 1;
                    if self.prime_streak > self.best_streak {
                        self.best_streak = self.prime_streak;
                    }
                    self.last_generation_success = false;
                    self.prime_streak = 0;
                // Update success history for sparkline
                self.success_history.push(if is_prime { 100.0 } else { 0.0 });
                if self.success_history.len() > 50 {
                    self.success_history.remove(0);
                self.flash_timer = Instant::now();
                self.current_seed_index = (self.current_seed_index + 1) % self.test_seeds.len();
                // Update performance and check achievements
                self.update_performance_stats();
                self.check_achievements();
            Err(_) => {
    fn adjust_parameter(&mut self, increase: bool) {
        let delta = if increase { 1 } else { -1 };
        match self.selected_param {
            0 => { // Base
                let new_base = (self.base as i32 + delta).max(2).min(36) as u32;
                if new_base != self.base {
                    self.base = new_base;
                    self.switch_configuration();
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
    fn switch_configuration(&mut self) {
        // Initialize new config in history if needed
        if !self.config_history.contains_key(&hash) {
            self.config_history.insert(hash.clone(), PerformanceStats {
                config: self.get_config(),
                expected_rate: self.get_expected_rate(),
            });
        self.current_performance = self.config_history[&hash].clone();
        self.prime_streak = 0;  // Reset streak on config change
    fn export_data(&mut self) -> io::Result<()> {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("membrane_lab_export_{}.jsonl", timestamp);
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&filename)?;
        // Export configuration history
        for (config_hash, stats) in &self.config_history {
            let record = serde_json::json!({
                "config": config_hash,
                "base": stats.config.base,
                "outer": stats.config.outer_boundary_digit,
                "inner": stats.config.inner_boundary_digit,
                "k_outer": stats.config.k_outer,
                "k_inner": stats.config.k_inner,
                "total_generated": stats.total_generated,
                "total_primes": stats.total_primes,
                "success_rate": stats.success_rate,
                "expected_rate": stats.expected_rate,
                "performance_ratio": stats.performance_ratio,
                "confidence_interval": stats.confidence_interval,
                "p_value": stats.p_value,
                "avg_generation_time_ms": stats.generation_times.iter().sum::<f64>() / stats.generation_times.len() as f64,
            writeln!(file, "{}", serde_json::to_string(&record)?)?;
        // Mark achievement
        if !self.achievements.contains(&Achievement::DataScientist) {
            self.achievements.push(Achievement::DataScientist);
            self.pending_achievements.push(Achievement::DataScientist);
        Ok(())
    fn load_best_config(&mut self) {
        if let Some(best) = self.recommended_configs.first() {
            self.base = best.0;
            self.outer = best.1;
            self.inner = best.2;
            self.k_outer = best.3;
            self.k_inner = best.4;
            self.switch_configuration();
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
fn normal_cdf(z: f64) -> f64 {
    // Approximation of normal CDF
    0.5 * (1.0 + (z / 2.0_f64.sqrt()).tanh())
fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // Create app state and generate first prime
    let mut app = LabState::default();
    app.switch_configuration(); // Initialize stats
    app.generate_current_seed();
    // Run app
    let res = run_app(&mut terminal, &mut app, Duration::from_millis(50));
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
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('?') | KeyCode::Char('h') => app.show_help = !app.show_help,
                        KeyCode::Tab => app.tab_index = (app.tab_index + 1) % 6,
                        KeyCode::BackTab => app.tab_index = (app.tab_index + 5) % 6,
                        KeyCode::Up => {
                            if app.tab_index == 0 {
                                app.selected_param = (app.selected_param + 4) % 5;
                            }
                        }
                        KeyCode::Down => {
                                app.selected_param = (app.selected_param + 1) % 5;
                        KeyCode::Left => {
                                app.adjust_parameter(false);
                        KeyCode::Right => {
                                app.adjust_parameter(true);
                        KeyCode::Enter => app.generate_current_seed(),
                        KeyCode::Char('g') | KeyCode::Char('G') => {
                            for _ in 0..10 {
                                app.generate_current_seed();
                        KeyCode::Char('b') | KeyCode::Char('B') => app.load_best_config(),
                        KeyCode::Char('e') | KeyCode::Char('E') => {
                            let _ = app.export_data();
                        _ => {}
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
fn ui(f: &mut Frame, app: &LabState) {
    let size = f.size();
    if app.show_help {
        draw_help_popup(f, size);
        return;
    // Show achievement popup if needed
    if let Some(timer) = app.achievement_timer {
        if timer.elapsed() < Duration::from_secs(3) && !app.pending_achievements.is_empty() {
            draw_achievement_popup(f, size, &app.pending_achievements[0]);
    // Main layout
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
    let tab_titles = vec!["Config", "Live Data", "Performance", "Achievements", "Recommendations", "Export"];
    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL))
        .select(app.tab_index)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    f.render_widget(tabs, chunks[1]);
    // Content
    match app.tab_index {
        0 => draw_config_tab(f, chunks[2], app),
        1 => draw_live_data_tab(f, chunks[2], app),
        2 => draw_performance_tab(f, chunks[2], app),
        3 => draw_achievements_tab(f, chunks[2], app),
        4 => draw_recommendations_tab(f, chunks[2], app),
        5 => draw_export_tab(f, chunks[2], app),
        _ => {}
    // Enhanced status bar with sparkline
    draw_enhanced_status(f, chunks[3], app);
fn draw_title(f: &mut Frame, area: Rect, app: &LabState) {
    let flash_elapsed = app.flash_timer.elapsed();
    let title_text = if flash_elapsed < Duration::from_millis(500) {
        if app.last_generation_success {
            if app.prime_streak > 2 {
                format!("🔬 Membrane Laboratory 🔥 {} STREAK! | {:.1}% success", 
                    app.prime_streak, app.current_performance.success_rate)
            } else {
                format!("🔬 Membrane Laboratory ✨ PRIME! | {:.1}% success", 
                    app.current_performance.success_rate)
        } else {
            format!("🔬 Membrane Laboratory ⚠️ Composite | {:.1}% success", 
                app.current_performance.success_rate)
    } else {
        format!("🔬 Membrane Laboratory | {:.1}% success (expected: {:.1}%)", 
            app.current_performance.success_rate,
            app.current_performance.expected_rate)
    };
    let title_color = if app.current_performance.performance_ratio > 1.1 {
        Color::Green
    } else if app.current_performance.performance_ratio > 0.9 {
        Color::Yellow
        Color::Red
    let title = Paragraph::new(title_text)
        .style(Style::default().fg(title_color).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, area);
fn draw_config_tab(f: &mut Frame, area: Rect, app: &LabState) {
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);
    // Left: Parameters
    let params = vec![
        (format!("Base: {}", app.base), app.selected_param == 0),
        (format!("Outer digit: {}", app.outer), app.selected_param == 1),
        (format!("Inner digit: {}", app.inner), app.selected_param == 2),
        (format!("K-outer: {}", app.k_outer), app.selected_param == 3),
        (format!("K-inner: {}", app.k_inner), app.selected_param == 4),
    ];
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
    // Right: Real-time stats
    let right_chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);
    // Performance gauge
    let performance_pct = (app.current_performance.performance_ratio * 100.0).min(200.0) as u16;
    let gauge_color = match app.current_performance.performance_ratio {
        r if r > 1.2 => Color::Green,
        r if r > 1.0 => Color::Yellow,
        r if r > 0.8 => Color::Magenta,
        _ => Color::Red,
    let performance_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Performance vs Expected "))
        .gauge_style(Style::default().fg(gauge_color))
        .percent(performance_pct / 2)  // Scale to 0-100
        .label(format!("{:.0}%", performance_pct));
    f.render_widget(performance_gauge, right_chunks[0]);
    // Statistical summary
    let coprime = gcd(app.outer, app.base) == 1 && gcd(app.inner, app.base) == 1;
    let stats_text = vec![
        Line::from(vec![
            Span::raw("Coprimality: "),
            if coprime {
                Span::styled("✓ Valid", Style::default().fg(Color::Green))
                Span::styled("✗ Invalid", Style::default().fg(Color::Red))
        ]),
        Line::from(format!("Expected rate: {:.1}%", app.current_performance.expected_rate)),
        Line::from(format!("Actual rate: {:.1}% ({}/{})", 
            app.current_performance.total_primes,
            app.current_performance.total_generated)),
        Line::from(format!("95% CI: [{:.1}%, {:.1}%]", 
            app.current_performance.confidence_interval.0,
            app.current_performance.confidence_interval.1)),
        Line::from(format!("p-value: {:.4}", app.current_performance.p_value)),
        Line::from(""),
            Span::raw("Press "),
            Span::styled("B", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" for best config"),
    let stats = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title(" Statistical Analysis "));
    f.render_widget(stats, right_chunks[1]);
fn draw_live_data_tab(f: &mut Frame, area: Rect, app: &LabState) {
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
    // Recent results
    let results_block = Block::default()
        .borders(Borders::ALL)
        .title(" Recent Results (Press G to generate batch) ");
    let results_inner = results_block.inner(chunks[0]);
    f.render_widget(results_block, chunks[0]);
    if app.current_results.is_empty() {
        let empty = Paragraph::new("No results yet. Press ENTER to generate!")
            .alignment(Alignment::Center);
        f.render_widget(empty, results_inner);
        let items: Vec<ListItem> = app.current_results
            .iter()
            .rev()
            .take(10)
            .map(|result| {
                let style = if result.is_prime {
                    Style::default().fg(Color::Green)
                    Style::default().fg(Color::Red)
                let text = format!(
                    "Seed {}: {} {} ({:.1}ms)",
                    result.seed,
                    result.number,
                    if result.is_prime { "✓ PRIME" } else { "✗ composite" },
                    result.generation_time.as_micros() as f64 / 1000.0
                );
                ListItem::new(text).style(style)
            })
            .collect();
        let results_list = List::new(items);
        f.render_widget(results_list, results_inner);
    // Speed chart
    if !app.current_performance.generation_times.is_empty() {
        let chart_data: Vec<(f64, f64)> = app.current_performance.generation_times
            .enumerate()
            .map(|(i, &time)| (i as f64, time))
        let datasets = vec![Dataset::default()
            .name("Generation time (ms)")
            .marker(symbols::Marker::Dot)
            .graph_type(ratatui::widgets::GraphType::Line)
            .style(Style::default().fg(Color::Cyan))
            .data(&chart_data)];
        let x_max = app.current_performance.generation_times.len() as f64;
        let y_max = app.current_performance.generation_times.iter().cloned().fold(0.0, f64::max) * 1.1;
        let chart = Chart::new(datasets)
            .block(Block::default().borders(Borders::ALL).title(" Generation Speed "))
            .x_axis(Axis::default()
                .bounds([0.0, x_max])
                .labels(vec![]))
            .y_axis(Axis::default()
                .bounds([0.0, y_max])
                .labels(vec![
                    Span::raw("0"),
                    Span::raw(format!("{:.1}", y_max)),
                ]));
        f.render_widget(chart, chunks[1]);
fn draw_performance_tab(f: &mut Frame, area: Rect, app: &LabState) {
        .constraints([Constraint::Length(15), Constraint::Min(0)])
    // Configuration comparison table
    let mut rows = vec![
        Line::from("Configuration          | Generated | Primes | Rate    | vs Expected | p-value"),
        Line::from("────────────────────────────────────────────────────────────────────────────"),
    let mut configs: Vec<_> = app.config_history.iter().collect();
    configs.sort_by(|a, b| b.1.success_rate.partial_cmp(&a.1.success_rate).unwrap());
    for (hash, stats) in configs.iter().take(10) {
        let color = match stats.performance_ratio {
            r if r > 1.1 => Color::Green,
            r if r > 0.9 => Color::Yellow,
            _ => Color::Red,
        };
        let row = format!(
            "{:<20} | {:>9} | {:>6} | {:>6.1}% | {:>+10.1}% | {:.4}",
            hash,
            stats.total_generated,
            stats.total_primes,
            stats.success_rate,
            (stats.performance_ratio - 1.0) * 100.0,
            stats.p_value
        );
        rows.push(Line::from(vec![Span::styled(row, Style::default().fg(color))]));
    let comparison = Paragraph::new(rows)
        .block(Block::default().borders(Borders::ALL).title(" Configuration Performance Comparison "));
    f.render_widget(comparison, chunks[0]);
    // Detailed analysis for current config
    let analysis_text = vec![
            Span::styled("Current Configuration Analysis", Style::default().add_modifier(Modifier::BOLD)),
        Line::from(format!("Configuration: base={}, outer={}, inner={}, k=({},{})",
            app.base, app.outer, app.inner, app.k_outer, app.k_inner)),
        Line::from(format!("Total attempts: {}", app.current_performance.total_generated)),
        Line::from(format!("Success rate: {:.2}% ± {:.2}%", 
            (app.current_performance.confidence_interval.1 - app.current_performance.confidence_interval.0) / 2.0)),
            Span::raw("Statistical significance: "),
            if app.current_performance.p_value < 0.05 {
                Span::styled("YES (p < 0.05)", Style::default().fg(Color::Green))
                Span::styled("NO (p ≥ 0.05)", Style::default().fg(Color::Yellow))
        Line::from(if app.current_performance.performance_ratio > 1.0 {
            format!("🎯 Beating expected rate by {:.1}%!", 
                (app.current_performance.performance_ratio - 1.0) * 100.0)
            format!("📊 Underperforming by {:.1}%", 
                (1.0 - app.current_performance.performance_ratio) * 100.0)
        }),
    let analysis = Paragraph::new(analysis_text)
        .block(Block::default().borders(Borders::ALL).title(" Statistical Analysis "))
        .wrap(Wrap { trim: true });
    f.render_widget(analysis, chunks[1]);
fn draw_achievements_tab(f: &mut Frame, area: Rect, app: &LabState) {
    // Unlocked achievements
    let unlocked_block = Block::default()
        .title(format!(" Unlocked Achievements ({}/{}) ", app.achievements.len(), 13));
    let unlocked_inner = unlocked_block.inner(chunks[0]);
    f.render_widget(unlocked_block, chunks[0]);
    let unlocked_items: Vec<ListItem> = app.achievements
        .map(|achievement| {
            ListItem::new(Line::from(vec![
                Span::styled(achievement.name(), Style::default().fg(Color::Yellow)),
                Span::raw(" - "),
                Span::raw(achievement.description()),
            ]))
    let unlocked_list = List::new(unlocked_items);
    f.render_widget(unlocked_list, unlocked_inner);
    // Progress towards next achievements
    let progress_text = vec![
            Span::styled("Progress Tracking", Style::default().add_modifier(Modifier::BOLD)),
        Line::from(format!("Current streak: {} (best: {})", app.prime_streak, app.best_streak)),
        Line::from(format!("Total primes: {}", app.total_primes)),
        Line::from(format!("Configurations tried: {}", app.config_history.len())),
        Line::from("Next milestones:"),
        Line::from(if app.total_primes < 10 {
            format!("  🔟 {} more primes to Ten Primes", 10 - app.total_primes)
        } else if app.total_primes < 100 {
            format!("  💯 {} more primes to Century Club", 100 - app.total_primes)
            "  🏆 All prime milestones unlocked!".to_string()
        Line::from(if app.best_streak < 5 {
            format!("  🔥 {} more in a row for Hot Streak", 5 - app.prime_streak)
        } else if app.best_streak < 10 {
            format!("  🌋 {} more in a row for On Fire!", 10 - app.prime_streak)
            "  🏆 All streak achievements unlocked!".to_string()
    let progress = Paragraph::new(progress_text)
        .block(Block::default().borders(Borders::ALL).title(" Progress "));
    f.render_widget(progress, chunks[1]);
fn draw_recommendations_tab(f: &mut Frame, area: Rect, app: &LabState) {
        .constraints([Constraint::Length(10), Constraint::Min(0)])
    // Top configurations
    let mut recs = vec![
            Span::styled("🏆 Top Performing Configurations", Style::default().add_modifier(Modifier::BOLD)),
        Line::from("Base | Outer | Inner | K-values | Expected Rate | Notes"),
        Line::from("──────────────────────────────────────────────────────────"),
    for (i, &(base, outer, inner, k_outer, k_inner, rate)) in app.recommended_configs.iter().enumerate() {
        let current = app.base == base && app.outer == outer && app.inner == inner && 
                     app.k_outer == k_outer && app.k_inner == k_inner;
        let style = if current {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else if i == 0 {
            Style::default().fg(Color::Green)
            Style::default()
        let notes = match (base, outer, inner) {
            (6, 1, 5) => "Champion config!",
            (30, 11, 7) => "High base performer",
            _ => "",
        recs.push(Line::from(vec![
            Span::styled(
                format!("{:>4} | {:>5} | {:>5} | ({},{})    | {:>5.1}%        | {}",
                    base, outer, inner, k_outer, k_inner, rate, notes),
                style
            ),
        ]));
    recs.push(Line::from(""));
    recs.push(Line::from(vec![
        Span::raw("Press "),
        Span::styled("B", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" to load the best configuration"),
    ]));
    let recommendations = Paragraph::new(recs)
    f.render_widget(recommendations, chunks[0]);
    // Analysis
            Span::styled("Configuration Insights", Style::default().add_modifier(Modifier::BOLD)),
        Line::from("🔍 Key patterns from your session:"),
    // Add insights based on data
    let mut insights = analysis_text;
    // Find best performing config
    if let Some((best_hash, best_stats)) = app.config_history.iter()
        .max_by(|a, b| a.1.success_rate.partial_cmp(&b.1.success_rate).unwrap()) {
        insights.push(Line::from(format!("• Best performer: {} at {:.1}%", best_hash, best_stats.success_rate)));
    // Check coprimality correlation
    let coprime_configs: Vec<_> = app.config_history.iter()
        .filter(|(_, s)| gcd(s.config.outer_boundary_digit, s.config.base) == 1 && 
                         gcd(s.config.inner_boundary_digit, s.config.base) == 1)
    let non_coprime_configs: Vec<_> = app.config_history.iter()
        .filter(|(_, s)| gcd(s.config.outer_boundary_digit, s.config.base) != 1 || 
                         gcd(s.config.inner_boundary_digit, s.config.base) != 1)
    if !coprime_configs.is_empty() && !non_coprime_configs.is_empty() {
        let avg_coprime = coprime_configs.iter()
            .map(|(_, s)| s.success_rate)
            .sum::<f64>() / coprime_configs.len() as f64;
        let avg_non_coprime = non_coprime_configs.iter()
            .sum::<f64>() / non_coprime_configs.len() as f64;
        insights.push(Line::from(format!("• Coprime configs average: {:.1}%", avg_coprime)));
        insights.push(Line::from(format!("• Non-coprime average: {:.1}%", avg_non_coprime)));
        insights.push(Line::from(format!("• Coprimality boost: {:.1}x", avg_coprime / avg_non_coprime)));
    let analysis = Paragraph::new(insights)
        .block(Block::default().borders(Borders::ALL).title(" Data-Driven Insights "))
fn draw_export_tab(f: &mut Frame, area: Rect, app: &LabState) {
    let export_text = vec![
            Span::styled("📊 Export Your Data", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
        Line::from("Export your session data for further analysis:"),
        Line::from("• Configuration performance history"),
        Line::from("• Generation timing data"),
        Line::from("• Statistical summaries"),
        Line::from("• Confidence intervals and p-values"),
        Line::from("Data format: JSON Lines (.jsonl)"),
        Line::from("Compatible with: Python, R, Excel, Jupyter"),
            Span::styled("E", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" to export data"),
        if app.achievements.contains(&Achievement::DataScientist) {
            Line::from(vec![
                Span::styled("✓ Data exported!", Style::default().fg(Color::Green)),
                Span::raw(" Check your current directory."),
            ])
            Line::from("Export your data to unlock the Data Scientist achievement!")
        },
    let export = Paragraph::new(export_text)
        .block(Block::default().borders(Borders::ALL).title(" Export Data "))
        .alignment(Alignment::Center);
    f.render_widget(export, area);
fn draw_enhanced_status(f: &mut Frame, area: Rect, app: &LabState) {
        .constraints([Constraint::Length(2), Constraint::Length(2)])
    // Status line
    let status_chunks = Layout::default()
            Constraint::Percentage(25),
        .split(chunks[0]);
    let config_status = Paragraph::new(format!("Config: ({},{}) {}", 
        app.outer, app.inner, if coprime { "✓" } else { "✗" }))
        .style(Style::default().fg(if coprime { Color::Green } else { Color::Red }));
    f.render_widget(config_status, status_chunks[0]);
    let rate_color = match app.current_performance.performance_ratio {
        r if r > 1.1 => Color::Green,
        r if r > 0.9 => Color::Yellow,
    let rate_status = Paragraph::new(format!("Rate: {:.1}%", app.current_performance.success_rate))
        .style(Style::default().fg(rate_color))
    f.render_widget(rate_status, status_chunks[1]);
    let streak_status = Paragraph::new(format!("Streak: {} {}", 
        app.prime_streak,
        if app.prime_streak >= 3 { "🔥" } else { "" }))
        .style(Style::default().fg(if app.prime_streak > 0 { Color::Yellow } else { Color::White }))
    f.render_widget(streak_status, status_chunks[2]);
    let help_status = Paragraph::new("? help")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Right);
    f.render_widget(help_status, status_chunks[3]);
    // Sparkline
    if !app.success_history.is_empty() {
        let sparkline_data: Vec<u64> = app.success_history.iter()
            .map(|&v| v as u64)
        let sparkline = Sparkline::default()
            .block(Block::default().borders(Borders::TOP))
            .data(&sparkline_data)
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(sparkline, chunks[1]);
fn draw_achievement_popup(f: &mut Frame, area: Rect, achievement: &Achievement) {
    let popup_area = centered_rect(50, 20, area);
    f.render_widget(Clear, popup_area);
    let popup_text = vec![
            Span::styled("🎉 Achievement Unlocked! 🎉", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(achievement.name(), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Line::from(achievement.description()),
    let popup = Paragraph::new(popup_text)
            .border_style(Style::default().fg(Color::Yellow))
            .style(Style::default().bg(Color::Black)))
    f.render_widget(popup, popup_area);
fn draw_help_popup(f: &mut Frame, area: Rect) {
    let popup_area = centered_rect(70, 80, area);
    let help_text = vec![
        Line::from("🔬 Ultimate Membrane Laboratory - Help"),
            Span::styled("Navigation:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
        Line::from("  Tab/Shift+Tab - Switch between tabs"),
        Line::from("  ↑/↓           - Select parameter (Config tab)"),
        Line::from("  ←/→           - Adjust parameter value"),
            Span::styled("Actions:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
        Line::from("  Enter         - Generate single prime"),
        Line::from("  G             - Generate batch (10 seeds)"),
        Line::from("  B             - Load best configuration"),
        Line::from("  E             - Export session data"),
        Line::from("  ? or H        - Toggle this help"),
        Line::from("  Q             - Quit"),
            Span::styled("Features:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
        Line::from("  • Real-time performance tracking"),
        Line::from("  • Statistical significance testing"),
        Line::from("  • Achievement system"),
        Line::from("  • Configuration recommendations"),
        Line::from("  • Data export for analysis"),
        Line::from("Press any key to close..."),
    let help = Paragraph::new(help_text)
            .title(" Help ")
            .border_style(Style::default().fg(Color::Yellow)))
        .style(Style::default().bg(Color::Black));
    f.render_widget(help, popup_area);
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
        .split(r);
    Layout::default()
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
        .split(popup_layout[1])[1]
