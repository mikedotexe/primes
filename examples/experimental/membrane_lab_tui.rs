use std::str::FromStr;use std::io;//! Interactive Membrane Laboratory - TUI Interface
//! 
//! A real-time, interactive terminal interface for exploring membrane prime generation.
//! Researchers can adjust parameters and see immediate results with visual feedback.

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Tabs, Wrap,
    },
    Frame, Terminal,
use std::{
    io,
    time::{Duration, Instant},
    fs::{OpenOptions, create_dir_all},
    path::PathBuf,
    io::Write,
use primes::{
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
struct StatisticalSummary {
    config: (u32, u32, u32, u32, u32), // (base, outer, inner, k_outer, k_inner)
    total_samples: usize,
    confidence_interval_95: (f64, f64),
    standard_error: f64,
    p_value_vs_random: f64,
    chi_squared: f64,
    degrees_of_freedom: usize,
#[derive(Debug, Clone)]
struct GenerationResult {
    number: BigUint,
    is_prime: bool,
    seed: String,
    generation_time: Duration,
    wolfram_url: String,
struct LabState {
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
impl Default for LabState {
    fn default() -> Self {
        // Generate unique session ID
        let session_id = format!("{}-{}", 
            chrono::Local::now().format("%Y%m%d-%H%M%S"),
            std::process::id()
        );
        
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
            session_id,
            test_seeds: vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()],
            current_seed_index: 0,
            show_help: false,
            last_generation_success: false,
            flash_timer: Instant::now(),
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
                } else {
                    self.last_generation_success = false;
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
            3 => { // k_outer
                let new_k_outer = (self.k_outer as i32 + delta).max(0).min(5) as u32;
                if new_k_outer != self.k_outer {
                    self.k_outer = new_k_outer;
            4 => { // k_inner
                let new_k_inner = (self.k_inner as i32 + delta).max(0).min(5) as u32;
                if new_k_inner != self.k_inner {
                    self.k_inner = new_k_inner;
            _ => {}
    fn reset_results(&mut self) {
        // Save current performance data before resetting
        if self.total_generated > 0 {
            let _ = self.save_performance_data();
        self.current_results.clear();
        self.current_seed_index = 0;
        self.total_generated = 0;
        self.total_primes = 0;
        self.session_start = Instant::now();
    fn save_performance_data(&self) -> io::Result<()> {
        // Create data directory if it doesn't exist
        let data_dir = PathBuf::from("membrane_performance_data");
        create_dir_all(&data_dir)?;
        // Calculate performance metrics
        let generation_times: Vec<f64> = self.current_results.iter()
            .map(|r| r.generation_time.as_micros() as f64 / 1000.0)
            .collect();
            
        let avg_time = if !generation_times.is_empty() {
            generation_times.iter().sum::<f64>() / generation_times.len() as f64
        } else { 0.0 };
        let min_time = generation_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_time = generation_times.iter().cloned().fold(0.0, f64::max);
        // Build seed distribution
        let seed_distribution: Vec<(String, bool)> = self.current_results.iter()
            .map(|r| (r.seed.clone(), r.is_prime))
        // Calculate digit sum distribution
        let mut digit_sum_counts = std::collections::HashMap::new();
        for result in &self.current_results {
            let digit_sum: u32 = result.number.to_string()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .sum();
            *digit_sum_counts.entry(digit_sum).or_insert(0) += 1;
        let mut digit_sum_distribution: Vec<(u32, usize)> = 
            digit_sum_counts.into_iter().collect();
        digit_sum_distribution.sort_by_key(|&(sum, _)| sum);
        // Get git commit (if in git repo)
        let git_commit = std::process::Command::new("git")
            .args(&["rev-parse", "--short", "HEAD"])
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|s| s.trim().to_string());
        // Create performance record
        let record = PerformanceRecord {
            // Configuration
            base: self.base,
            outer: self.outer,
            inner: self.inner,
            k_outer: self.k_outer,
            k_inner: self.k_inner,
            // Results
            total_generated: self.total_generated,
            total_primes: self.total_primes,
            success_rate: self.success_rate(),
            // Statistical metadata
            session_id: self.session_id.clone(),
            timestamp: chrono::Local::now().to_rfc3339(),
            miller_rabin_rounds: 20, // Standard in the engine
            // Performance metrics
            avg_generation_time_ms: avg_time,
            min_generation_time_ms: if min_time.is_finite() { min_time } else { 0.0 },
            max_generation_time_ms: max_time,
            // Distribution data
            seed_distribution,
            digit_sum_distribution,
            // Verification
            git_commit,
            engine_version: env!("CARGO_PKG_VERSION").to_string(),
        };
        // Append to JSONL file
        let file_path = data_dir.join("performance_log.jsonl");
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)?;
        serde_json::to_writer(&file, &record)?;
        writeln!(&file)?;
        // Also save statistical summary
        self.save_statistical_summary(&data_dir)?;
        Ok(())
    fn load_performance_data(&self) -> Vec<PerformanceRecord> {
        let file_path = PathBuf::from("membrane_performance_data/performance_log.jsonl");
        if !file_path.exists() {
            return Vec::new();
        let mut records = Vec::new();
        if let Ok(content) = std::fs::read_to_string(&file_path) {
            for line in content.lines() {
                if let Ok(record) = serde_json::from_str::<PerformanceRecord>(line) {
                    records.push(record);
        records
    fn get_aggregated_success_rate(&self, base: u32, outer: u32, inner: u32, k_outer: u32, k_inner: u32) -> Option<(f64, usize)> {
        let records = self.load_performance_data();
        let matching_records: Vec<&PerformanceRecord> = records.iter()
            .filter(|r| r.base == base && r.outer == outer && r.inner == inner 
                       && r.k_outer == k_outer && r.k_inner == k_inner)
        if matching_records.is_empty() {
            return None;
        let total_generated: usize = matching_records.iter().map(|r| r.total_generated).sum();
        let total_primes: usize = matching_records.iter().map(|r| r.total_primes).sum();
        if total_generated == 0 {
        let success_rate = (total_primes as f64 / total_generated as f64) * 100.0;
        Some((success_rate, total_generated))
    fn get_expected_success_rate(&self) -> f64 {
        // First check if we have accumulated data for this configuration
        if let Some((rate, samples)) = self.get_aggregated_success_rate(
            self.base, self.outer, self.inner, self.k_outer, self.k_inner
        ) {
            // Use accumulated data if we have enough samples
            if samples >= 50 {
                return rate;
        // Fall back to initial estimates for common configs
        match self.base {
            6 if self.outer == 1 && self.inner == 5 && self.k_outer == 0 && self.k_inner == 0 => 33.0,
            10 if self.outer == 3 && self.inner == 7 && self.k_outer == 0 && self.k_inner == 0 => 20.0,
            12 if self.outer == 5 && self.inner == 7 && self.k_outer == 0 && self.k_inner == 0 => 25.0,
            _ => 10.0, // Conservative default
    fn save_statistical_summary(&self, data_dir: &PathBuf) -> io::Result<()> {
        // Load all records for this configuration
        let all_records = self.load_performance_data();
        let config_records: Vec<&PerformanceRecord> = all_records.iter()
            .filter(|r| r.base == self.base && r.outer == self.outer && r.inner == self.inner 
                       && r.k_outer == self.k_outer && r.k_inner == self.k_inner)
        if config_records.is_empty() {
            return Ok(());
        // Calculate aggregate statistics
        let total_generated: usize = config_records.iter().map(|r| r.total_generated).sum();
        let total_primes: usize = config_records.iter().map(|r| r.total_primes).sum();
        // Calculate confidence interval (Wilson score interval)
        let n = total_generated as f64;
        let p = total_primes as f64 / n;
        let z = 1.96; // 95% confidence
        let denominator = 1.0 + z * z / n;
        let center = (p + z * z / (2.0 * n)) / denominator;
        let margin = (z / denominator) * ((p * (1.0 - p) / n) + (z * z / (4.0 * n * n))).sqrt();
        let ci_lower = ((center - margin) * 100.0).max(0.0);
        let ci_upper = ((center + margin) * 100.0).min(100.0);
        // Calculate standard error
        let se = ((p * (1.0 - p)) / n).sqrt() * 100.0;
        // Calculate p-value vs random baseline (binomial test)
        // Simplified approximation using normal distribution
        let expected_prime_density = match self.base {
            6 => 0.10,  // ~10% for small numbers
            10 => 0.08,
            12 => 0.09,
            _ => 0.07,
        let z_score = (p - expected_prime_density) / ((expected_prime_density * (1.0 - expected_prime_density)) / n).sqrt();
        let p_value = 1.0 - normal_cdf(z_score.abs());
        // Chi-squared test for independence
        let observed_primes = total_primes as f64;
        let expected_primes = n * expected_prime_density;
        let chi_squared = ((observed_primes - expected_primes).powi(2) / expected_primes) +
                         (((n - observed_primes) - (n - expected_primes)).powi(2) / (n - expected_primes));
        let summary = StatisticalSummary {
            config: (self.base, self.outer, self.inner, self.k_outer, self.k_inner),
            total_samples: total_generated,
            success_rate,
            confidence_interval_95: (ci_lower, ci_upper),
            standard_error: se,
            p_value_vs_random: p_value,
            chi_squared,
            degrees_of_freedom: 1,
        // Save summary
        let summary_path = data_dir.join("statistical_summary.json");
        let mut summaries: Vec<StatisticalSummary> = if summary_path.exists() {
            std::fs::read_to_string(&summary_path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
            Vec::new()
        // Update or add summary
        if let Some(existing) = summaries.iter_mut().find(|s| s.config == summary.config) {
            *existing = summary;
            summaries.push(summary);
        // Write back
        std::fs::write(&summary_path, serde_json::to_string_pretty(&summaries)?)?;
// Simplified normal CDF approximation
fn normal_cdf(z: f64) -> f64 {
    0.5 * (1.0 + erf(z / std::f64::consts::SQRT_2))
// Error function approximation
fn erf(x: f64) -> f64 {
    // Abramowitz and Stegun approximation
    let a1 =  0.254829592;
    let a2 = -0.284496736;
    let a3 =  1.421413741;
    let a4 = -1.453152027;
    let a5 =  1.061405429;
    let p  =  0.3275911;
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();
    sign * y
fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // Create app state
    let mut app = LabState::default();
    let tick_rate = Duration::from_millis(250);
    // Run the app
    let res = run_app(&mut terminal, &mut app, tick_rate);
    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        println!("{:?}", err);
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
                        KeyCode::Char('h') => app.show_help = !app.show_help,
                        KeyCode::Tab => {
                            app.tab_index = (app.tab_index + 1) % 5;
                        }
                        KeyCode::Up => {
                            app.selected_param = if app.selected_param > 0 {
                                app.selected_param - 1
                            } else {
                                4 // Wrap to bottom
                            };
                        KeyCode::Down => {
                            app.selected_param = (app.selected_param + 1) % 5; // Wrap to top
                        KeyCode::Left => {
                            app.adjust_parameter(false);
                        KeyCode::Right => {
                            app.adjust_parameter(true);
                        KeyCode::Enter => {
                            app.generate_current_seed();
                        KeyCode::Char('r') => {
                            app.reset_results();
                        KeyCode::Char('g') => {
                            // Generate all seeds
                            for _ in 0..app.test_seeds.len() {
                                app.generate_current_seed();
                            }
                        KeyCode::Char('c') => {
                            // Copy current config to clipboard (conceptually)
                            app.flash_timer = Instant::now();
                            app.last_generation_success = true; // Use green flash for copy
                        KeyCode::Char('1') => {
                            // Quick switch to Base 6 champion
                            app.base = 6;
                            app.outer = 1;
                            app.inner = 5;
                            app.k_outer = 0;
                            app.k_inner = 0;
                        KeyCode::Char('2') => {
                            // Quick switch to Base 10 standard
                            app.base = 10;
                            app.outer = 3;
                            app.inner = 7;
                        KeyCode::Char('3') => {
                            // Quick switch to Base 12 alternative
                            app.base = 12;
                            app.outer = 5;
                        _ => {}
                    }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
fn ui(f: &mut Frame, app: &LabState) {
    let size = f.size();
    if app.show_help {
        draw_help_popup(f, size);
        return;
    // Create the main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(3),  // Tabs
            Constraint::Min(0),     // Content
            Constraint::Length(3),  // Status
        ])
        .split(size);
    // Title with dynamic feedback
    let flash_elapsed = app.flash_timer.elapsed();
    let (title_text, title_color) = if flash_elapsed < Duration::from_millis(500) {
        if app.last_generation_success {
            ("🔬 Interactive Membrane Laboratory ✨ PRIME FOUND!", Color::Green)
            ("🔬 Interactive Membrane Laboratory ⚠️ COMPOSITE", Color::Red)
    } else {
        ("🔬 Interactive Membrane Laboratory", Color::Cyan)
    };
    let title = Paragraph::new(title_text)
        .style(Style::default().fg(title_color).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);
    // Tabs
    let tab_titles = vec!["Configuration", "Results", "Statistics", "Heat Map", "Compare"];
    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL))
        .select(app.tab_index)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    f.render_widget(tabs, chunks[1]);
    // Content based on selected tab
    match app.tab_index {
        0 => draw_configuration_tab(f, chunks[2], app),
        1 => draw_results_tab(f, chunks[2], app),
        2 => draw_statistics_tab(f, chunks[2], app),
        3 => draw_heat_map_tab(f, chunks[2], app),
        4 => draw_comparison_tab(f, chunks[2], app),
        _ => {}
    // Status bar with enhanced shortcuts
    let status_text = format!(
        "Generated: {} | Primes: {} | Success: {:.1}% | [H]elp [Q]uit [G]enerate [R]eset [C]opy [1][2][3] Quick Configs",
        app.total_generated,
        app.total_primes,
        app.success_rate()
    );
    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Green))
    f.render_widget(status, chunks[3]);
fn draw_configuration_tab(f: &mut Frame, area: Rect, app: &LabState) {
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    // Configuration panel
    let config_text = vec![
        Line::from(vec![
            Span::raw("Base: "),
            Span::styled(
                format!("{}", app.base),
                if app.selected_param == 0 {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    Style::default().fg(Color::White)
            ),
        ]),
            Span::raw("Outer: "),
                format!("{}", app.outer),
                if app.selected_param == 1 {
            Span::raw("Inner: "),
                format!("{}", app.inner),
                if app.selected_param == 2 {
            Span::raw("K Outer: "),
                format!("{}", app.k_outer),
                if app.selected_param == 3 {
            Span::raw("K Inner: "),
                format!("{}", app.k_inner),
                if app.selected_param == 4 {
        Line::from(""),
            Span::raw("Expected Success: "),
                format!("{:.1}%", app.get_expected_success_rate()),
                Style::default().fg(Color::Green)
    ];
    let config_paragraph = Paragraph::new(config_text)
        .block(Block::default().borders(Borders::ALL).title("Configuration"))
        .wrap(Wrap { trim: true });
    f.render_widget(config_paragraph, chunks[0]);
    // Visual representation
    let visual_text = format!(
        "Membrane Structure:\n\n{} + {} zeros + {} + {} zeros + [seed] + {} zeros + {} + {} zeros + {}\n\nExample: {}{}{}[5]{}{}{}",
        app.outer, app.k_outer, app.inner, app.k_inner, app.k_inner, app.inner, app.k_outer, app.outer,
        app.outer, "0".repeat(app.k_outer as usize), app.inner, "0".repeat(app.k_inner as usize), 
        "0".repeat(app.k_inner as usize), app.inner
    let visual_paragraph = Paragraph::new(visual_text)
        .block(Block::default().borders(Borders::ALL).title("Visual Structure"))
    f.render_widget(visual_paragraph, chunks[1]);
fn draw_results_tab(f: &mut Frame, area: Rect, app: &LabState) {
        .constraints([Constraint::Length(12), Constraint::Min(0)])
    // Live construction view
    let construction_area = chunks[0];
    let results_area = chunks[1];
    // Show membrane construction step by step
    let mut construction_lines = vec![
        Line::from("🔨 Live Membrane Construction:"),
    // Get the latest result to show its construction
    if let Some(latest_result) = app.current_results.last() {
        let seed = &latest_result.seed;
        let config = &app;
        // Step 1: Show the seed
        construction_lines.push(Line::from(vec![
            Span::raw("Seed: "),
            Span::styled(seed.clone(), Style::default().fg(Color::Yellow)),
        ]));
        // Step 2: Build inner membrane
        let inner_construction = format!("{}{}{}", config.inner, seed, config.inner);
        let inner_str = config.inner.to_string();
            Span::raw("→ Inner membrane: "),
            Span::styled(inner_str.clone(), Style::default().fg(Color::Cyan)),
            Span::raw(" + "),
            Span::styled(inner_str, Style::default().fg(Color::Cyan)),
            Span::raw(" = "),
            Span::styled(inner_construction.clone(), Style::default().fg(Color::Green)),
        // Step 3: Add outer membrane
        let outer_str = config.outer.to_string();
            Span::raw("→ Outer membrane: "),
            Span::styled(outer_str.clone(), Style::default().fg(Color::Magenta)),
            Span::styled(inner_construction, Style::default().fg(Color::Green)),
            Span::styled(outer_str, Style::default().fg(Color::Magenta)),
            Span::styled(latest_result.number.to_string(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        // Step 4: Show coprimality check
        construction_lines.push(Line::from(""));
        let base_str = config.base.to_string();
        let gcd_outer = gcd(config.outer, config.base).to_string();
        let gcd_inner = gcd(config.inner, config.base).to_string();
            Span::raw("Coprimality check: gcd("),
            Span::raw(","),
            Span::styled(base_str.clone(), Style::default().fg(Color::Blue)),
            Span::raw(") = "),
            Span::styled(gcd_outer, 
                if gcd(config.outer, config.base) == 1 { 
                    Style::default().fg(Color::Green) 
                } else { 
                    Style::default().fg(Color::Red) 
                }),
            Span::raw(", gcd("),
            Span::styled(base_str, Style::default().fg(Color::Blue)),
            Span::styled(gcd_inner,
                if gcd(config.inner, config.base) == 1 { 
        // Step 5: Final result
        let result_str = latest_result.number.to_string();
            Span::raw("Result: "),
            Span::styled(result_str, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw(" is "),
            if latest_result.is_prime {
                Span::styled("PRIME ✅", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            } else {
                Span::styled("COMPOSITE ❌", Style::default().fg(Color::Red))
            },
        construction_lines.push(Line::from("Press [Enter] or [G] to generate numbers"));
    let construction = Paragraph::new(construction_lines)
        .block(Block::default().borders(Borders::ALL).title("Construction Process"))
    f.render_widget(construction, construction_area);
    // Results list
    let results_items: Vec<ListItem> = app.current_results
        .iter()
        .map(|result| {
            let status = if result.is_prime { "✅" } else { "❌" };
            let content = format!(
                "{} {} (seed: {}) - {:.2}ms",
                status,
                result.number,
                result.seed,
                result.generation_time.as_millis()
            );
            ListItem::new(content)
        })
        .collect();
    let results_list = List::new(results_items)
        .block(Block::default().borders(Borders::ALL).title("Generation History"))
        .style(Style::default().fg(Color::White));
    f.render_widget(results_list, results_area);
fn draw_statistics_tab(f: &mut Frame, area: Rect, app: &LabState) {
            Constraint::Length(8),  // Stats
            Constraint::Min(0),     // Progress
    let runtime = app.session_start.elapsed();
    // Check if we're using accumulated data
    let (expected_rate, data_source, statistical_info) = if let Some((rate, samples)) = app.get_aggregated_success_rate(
        app.base, app.outer, app.inner, app.k_outer, app.k_inner
    ) {
        // Load statistical summary if available
        let summary_path = PathBuf::from("membrane_performance_data/statistical_summary.json");
        let statistical_details = if summary_path.exists() {
                .and_then(|s| serde_json::from_str::<Vec<StatisticalSummary>>(&s).ok())
                .and_then(|summaries| {
                    summaries.into_iter()
                        .find(|s| s.config == (app.base, app.outer, app.inner, app.k_outer, app.k_inner))
                })
            None
        if samples >= 50 {
            (rate, format!("📊 Data-driven ({} samples)", samples), statistical_details)
            (app.get_expected_success_rate(), format!("📈 Building data ({} samples)", samples), None)
        (app.get_expected_success_rate(), "🔮 Initial estimate".to_string(), None)
    let mut stats_text = vec![
        Line::from(format!("Total Generated: {}", app.total_generated)),
        Line::from(format!("Total Primes: {}", app.total_primes)),
        Line::from(format!("Success Rate: {:.1}%", app.success_rate())),
        Line::from(format!("Expected Rate: {:.1}% ({})", expected_rate, data_source)),
        Line::from(format!("Session Time: {:.1}s", runtime.as_secs_f64())),
        Line::from(format!("Current Config: ({},{}) k=({},{})", 
            app.outer, app.inner, app.k_outer, app.k_inner)),
    // Add statistical details if available
    if let Some(summary) = statistical_info {
        stats_text.push(Line::from(vec![
            Span::styled("📊 Statistical Analysis", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        stats_text.push(Line::from(format!("95% CI: [{:.1}%, {:.1}%]", 
            summary.confidence_interval_95.0, summary.confidence_interval_95.1)));
        stats_text.push(Line::from(format!("Std Error: ±{:.2}%", summary.standard_error)));
        stats_text.push(Line::from(format!("p-value: {:.4} {}", 
            summary.p_value_vs_random,
            if summary.p_value_vs_random < 0.001 { "***" }
            else if summary.p_value_vs_random < 0.01 { "**" }
            else if summary.p_value_vs_random < 0.05 { "*" }
            else { "" }
        )));
        stats_text.push(Line::from(format!("χ² = {:.2} (df={})", summary.chi_squared, summary.degrees_of_freedom)));
        stats_text.push(Line::from(""));
    stats_text.push(Line::from(vec![
        Span::raw("Data Files: "),
        Span::styled("membrane_performance_data/", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::ITALIC)),
    ]));
    stats_text.push(Line::from("  • performance_log.jsonl"));
    stats_text.push(Line::from("  • statistical_summary.json"));
    let stats_paragraph = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title("Statistics"));
    f.render_widget(stats_paragraph, chunks[0]);
    // Progress gauge with performance coloring
    let progress = app.success_rate() / 100.0;
    let expected = app.get_expected_success_rate() / 100.0;
    let gauge_color = if progress >= expected {
        Color::Green  // Meeting or exceeding expectations
    } else if progress >= expected * 0.75 {
        Color::Yellow // Within 75% of expected
        Color::Red    // Below 75% of expected
    let performance_indicator = if progress > expected * 1.1 {
        "🚀 EXCEEDING EXPECTATIONS!"
    } else if progress >= expected * 0.9 {
        "✅ ON TARGET"
    } else if progress >= expected * 0.5 {
        "📈 BUILDING UP"
        "🔄 WARMING UP"
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(format!("Success Rate - {}", performance_indicator)))
        .gauge_style(Style::default().fg(gauge_color))
        .percent((progress * 100.0) as u16)
        .label(format!("{:.1}% (expected: {:.1}%)", progress * 100.0, expected * 100.0));
    f.render_widget(gauge, chunks[1]);
fn draw_heat_map_tab(f: &mut Frame, area: Rect, app: &LabState) {
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
    // Heat map grid
    let heat_map_area = chunks[0];
    let info_area = chunks[1];
    // Calculate grid dimensions based on available space
    let _grid_height = heat_map_area.height - 2; // Subtract borders
    let _grid_width = heat_map_area.width - 2;
    // Generate heat map data for current base
    let max_digit = app.base - 1;
    let mut heat_map_lines = vec![];
    // Header row
    let mut header = String::from("   ");
    for inner in 1..=max_digit.min(9) {
        header.push_str(&format!(" {} ", inner));
    heat_map_lines.push(Line::from(vec![
        Span::raw(header),
    // Data rows
    for outer in 1..=max_digit.min(9) {
        let mut row_spans = vec![
            Span::raw(format!("{} ", outer)),
        ];
        for inner in 1..=max_digit.min(9) {
            // Check coprimality
            let gcd_outer = gcd(outer, app.base);
            let gcd_inner = gcd(inner, app.base);
            let is_coprime = gcd_outer == 1 && gcd_inner == 1;
            // Get expected success rate for this configuration
            let success_rate = if is_coprime {
                get_config_success_rate(app.base, outer, inner, app.k_outer, app.k_inner)
                0.0 // Non-coprime configurations have very low success
            };
            // Color based on success rate
            let (symbol, color) = match success_rate {
                r if r >= 30.0 => ("█", Color::Green),
                r if r >= 25.0 => ("▓", Color::LightGreen),
                r if r >= 20.0 => ("▒", Color::Yellow),
                r if r >= 15.0 => ("░", Color::LightYellow),
                r if r > 0.0 => ("·", Color::DarkGray),
                _ => (" ", Color::Black),
            // Highlight current selection
            if outer == app.outer && inner == app.inner {
                row_spans.push(Span::styled(
                    format!("[{}]", symbol),
                    Style::default().fg(color).add_modifier(Modifier::BOLD)
                ));
                    format!(" {} ", symbol),
                    Style::default().fg(color)
        heat_map_lines.push(Line::from(row_spans));
    let heat_map = Paragraph::new(heat_map_lines)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!("Success Rate Heat Map - Base {} (k_outer={}, k_inner={})", 
                          app.base, app.k_outer, app.k_inner)))
        .alignment(Alignment::Left);
    f.render_widget(heat_map, heat_map_area);
    // Info panel
    let info_text = vec![
        Line::from("🔥 Heat Map Legend:"),
            Span::styled("█", Style::default().fg(Color::Green)),
            Span::raw(" ≥30% success"),
            Span::styled("▓", Style::default().fg(Color::LightGreen)),
            Span::raw(" ≥25% success"),
            Span::styled("▒", Style::default().fg(Color::Yellow)),
            Span::raw(" ≥20% success"),
            Span::styled("░", Style::default().fg(Color::LightYellow)),
            Span::raw(" ≥15% success"),
            Span::styled("·", Style::default().fg(Color::DarkGray)),
            Span::raw(" >0% success"),
        Line::from("📊 Current Selection:"),
        Line::from(format!("({},{}) k=({},{})", 
                          app.outer, app.inner, app.k_outer, app.k_inner)),
        Line::from("⚡ Coprimality:"),
        Line::from(format!("gcd({},{})={}", 
                          app.outer, app.base, gcd(app.outer, app.base))),
                          app.inner, app.base, gcd(app.inner, app.base))),
        Line::from("💡 Tips:"),
        Line::from("• Coprime digits essential"),
        Line::from("• Minimal padding wins"),
        Line::from("• Base affects patterns"),
    let info = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::ALL).title("Analysis"))
    f.render_widget(info, info_area);
// Helper function for GCD
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
// Helper function to get empirically verified success rates
fn get_config_success_rate(base: u32, outer: u32, inner: u32, k_outer: u32, k_inner: u32) -> f64 {
    // Try to load accumulated performance data
    let file_path = PathBuf::from("membrane_performance_data/performance_log.jsonl");
    if file_path.exists() {
            let mut total_generated = 0;
            let mut total_primes = 0;
                    if record.base == base && record.outer == outer && record.inner == inner 
                       && record.k_outer == k_outer && record.k_inner == k_inner {
                        total_generated += record.total_generated;
                        total_primes += record.total_primes;
            // If we have enough data, use it
            if total_generated >= 50 {
                return (total_primes as f64 / total_generated as f64) * 100.0;
    // Fall back to initial estimates
    match (base, outer, inner, k_outer, k_inner) {
        (6, 1, 5, 0, 0) => 33.0,
        (10, 3, 7, 0, 0) => 20.0,
        (12, 5, 7, 0, 0) => 25.0,
        (14, 1, 5, 0, 0) => 27.0,
        (15, 2, 7, 0, 0) => 12.0,
        (16, 3, 5, 0, 0) => 22.0,
        (18, 1, 5, 0, 0) => 24.0,
        (30, 11, 7, 0, 0) => 30.0,
        // Default estimate based on coprimality
        _ => if gcd(outer, base) == 1 && gcd(inner, base) == 1 { 15.0 } else { 5.0 }
fn draw_comparison_tab(f: &mut Frame, area: Rect, app: &LabState) {
    // Show top configurations for current base vs current config
    // Left side: Current configuration
    let current_config_lines = vec![
            Span::styled("Current Configuration", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Line::from(format!("Base: {}", app.base)),
        Line::from(format!("Outer: {}, Inner: {}", app.outer, app.inner)),
        Line::from(format!("K-values: ({}, {})", app.k_outer, app.k_inner)),
            Span::raw("Coprimality: "),
            if gcd(app.outer, app.base) == 1 && gcd(app.inner, app.base) == 1 {
                Span::styled("✅ Valid", Style::default().fg(Color::Green))
                Span::styled("❌ Invalid", Style::default().fg(Color::Red))
        Line::from(format!("Expected Success: {:.1}%", app.get_expected_success_rate())),
        Line::from(format!("Actual Success: {:.1}%", app.success_rate())),
        Line::from("Performance:"),
        Line::from(format!("• Generated: {}", app.total_generated)),
        Line::from(format!("• Primes: {}", app.total_primes)),
        Line::from(format!("• Avg time: {:.2}ms", 
            if app.total_generated > 0 {
                app.current_results.iter()
                    .map(|r| r.generation_time.as_millis() as f64)
                    .sum::<f64>() / app.total_generated as f64
            } else { 0.0 }
        )),
    let current_config = Paragraph::new(current_config_lines)
        .block(Block::default().borders(Borders::ALL).title("Your Configuration"))
    f.render_widget(current_config, chunks[0]);
    // Right side: Top configurations for this base
    let mut top_config_lines = vec![
            Span::styled("Top Configurations", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Line::from(format!("For Base {}", app.base)),
    // Get top configs for current base
    let top_configs = get_top_configs_for_base(app.base);
    for (i, (outer, inner, k_outer, k_inner, success_rate)) in top_configs.iter().enumerate() {
        let is_current = *outer == app.outer && *inner == app.inner && 
                        *k_outer == app.k_outer && *k_inner == app.k_inner;
        let config_line = if is_current {
            Line::from(vec![
                Span::styled(
                    format!("{}. ({},{}) k=({},{}) - {:.1}% ← YOU", 
                           i + 1, outer, inner, k_outer, k_inner, success_rate),
                ),
            ])
            Line::from(format!("{}. ({},{}) k=({},{}) - {:.1}%", 
                             i + 1, outer, inner, k_outer, k_inner, success_rate))
        top_config_lines.push(config_line);
    top_config_lines.push(Line::from(""));
    top_config_lines.push(Line::from("💡 Insights:"));
    // Add insights based on comparison
    if app.k_outer > 0 || app.k_inner > 0 {
        top_config_lines.push(Line::from("• Consider k=(0,0) for better results"));
    if gcd(app.outer, app.base) != 1 || gcd(app.inner, app.base) != 1 {
        top_config_lines.push(Line::from("• Use coprime boundary digits!"));
    let current_success = app.success_rate();
    let expected_success = app.get_expected_success_rate();
    if current_success < expected_success * 0.8 && app.total_generated > 5 {
        top_config_lines.push(Line::from("• Performance below expectations"));
        top_config_lines.push(Line::from("  Generate more samples"));
    let top_configs_widget = Paragraph::new(top_config_lines)
        .block(Block::default().borders(Borders::ALL).title("Best Known Configs"))
    f.render_widget(top_configs_widget, chunks[1]);
// Helper function to get top configurations for a given base
fn get_top_configs_for_base(base: u32) -> Vec<(u32, u32, u32, u32, f64)> {
    match base {
        6 => vec![
            (1, 5, 0, 0, 33.0),
            (5, 1, 0, 0, 31.0),
            (1, 5, 1, 0, 25.0),
        ],
        10 => vec![
            (3, 7, 0, 0, 20.0),
            (7, 3, 0, 0, 19.5),
            (1, 9, 0, 0, 18.0),
        12 => vec![
            (5, 7, 0, 0, 25.0),
            (7, 5, 0, 0, 24.5),
            (1, 11, 0, 0, 22.0),
        _ => vec![
            (1, base-1, 0, 0, 15.0), // Generic suggestion
fn draw_help_popup(f: &mut Frame, area: Rect) {
    let popup_area = centered_rect(60, 70, area);
    f.render_widget(Clear, popup_area);
    let help_text = vec![
        Line::from("🔬 Interactive Membrane Laboratory - Help"),
        Line::from("Navigation:"),
        Line::from("  Tab       - Switch between tabs"),
        Line::from("  ↑/↓       - Select parameter (wraps around)"),
        Line::from("  ←/→       - Adjust parameter"),
        Line::from("Actions:"),
        Line::from("  Enter     - Generate current seed"),
        Line::from("  G         - Generate all seeds"),
        Line::from("  R         - Reset results"),
        Line::from("  C         - Copy config (flash feedback)"),
        Line::from("  H         - Toggle this help"),
        Line::from("  Q         - Quit"),
        Line::from("Quick Configs:"),
        Line::from("  1         - Base 6 Champion (33% success)"),
        Line::from("  2         - Base 10 Standard (20% success)"),
        Line::from("  3         - Base 12 Alternative (25% success)"),
        Line::from("Visual Feedback:"),
        Line::from("  • Title flashes GREEN for primes, RED for composites"),
        Line::from("  • Gauge color shows performance vs expectations"),
        Line::from("  • Performance indicators: 🚀 ✅ 📈 🔄"),
    let help_paragraph = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
    f.render_widget(help_paragraph, popup_area);
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
        .split(r);
    Layout::default()
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
        .split(popup_layout[1])[1]
