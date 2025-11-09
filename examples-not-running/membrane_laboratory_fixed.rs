use std::io;//! Prime Membrane Laboratory - Professional Edition (Fixed)
//! 
//! Consolidated TUI with all features, fixed borrow checker issues

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, List, ListItem, 
        Paragraph, Sparkline, Tabs,
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
// ===== ACHIEVEMENTS =====
#[derive(Debug, Clone, PartialEq)]
enum Achievement {
    FirstPrime,
    TenPrimes,
    StreakFive,
    BeatExpected,
    Palindrome,
    Explorer,
    DataScientist,
}
impl Achievement {
    fn name(&self) -> &str {
        match self {
            Achievement::FirstPrime => "🌟 First Prime",
            Achievement::TenPrimes => "🔟 Ten Primes",
            Achievement::StreakFive => "🔥 Hot Streak (5)",
            Achievement::BeatExpected => "📈 Beat the Odds",
            Achievement::Palindrome => "🔄 Palindrome Hunter",
            Achievement::Explorer => "🗺️ Explorer (5+ configs)",
            Achievement::DataScientist => "📊 Data Scientist",
        }
    }
// ===== DATA STRUCTURES =====
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PerformanceStats {
    base: u32,
    outer: u32,
    inner: u32,
    total_generated: usize,
    total_primes: usize,
    success_rate: f64,
    expected_rate: f64,
    performance_ratio: f64,
    avg_time_ms: f64,
#[derive(Debug, Clone)]
struct GenerationResult {
    number: BigUint,
    is_prime: bool,
    seed: String,
    time_ms: f64,
struct LabState {
    // Configuration
    k_outer: u32,
    k_inner: u32,
    
    // UI state
    selected_param: usize,
    tab_index: usize,
    show_help: bool,
    show_welcome: bool,
    show_tutorial: bool,
    tutorial_step: usize,
    // Results
    current_results: Vec<GenerationResult>,
    success_history: Vec<f64>,
    // Statistics
    session_start: Instant,
    config_stats: HashMap<String, PerformanceStats>,
    // Gamification
    prime_streak: usize,
    best_streak: usize,
    achievements: Vec<Achievement>,
    pending_achievement: Option<(Achievement, Instant)>,
    interesting_patterns: Vec<String>,
    // Visual feedback
    last_was_prime: bool,
    flash_timer: Instant,
    construction_step: usize,
    construction_timer: Instant,
    first_launch: bool,
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
            show_help: false,
            show_welcome: false,
            show_tutorial: false,
            tutorial_step: 0,
            current_results: Vec::new(),
            success_history: Vec::with_capacity(50),
            total_generated: 0,
            total_primes: 0,
            session_start: Instant::now(),
            config_stats: HashMap::new(),
            prime_streak: 0,
            best_streak: 0,
            achievements: Vec::new(),
            pending_achievement: None,
            interesting_patterns: Vec::new(),
            last_was_prime: false,
            flash_timer: Instant::now(),
            construction_step: 0,
            construction_timer: Instant::now(),
            first_launch: true,
impl LabState {
    fn get_config(&self) -> MembraneConfig {
        MembraneConfig::new(self.base, self.outer, self.inner, self.k_outer, self.k_inner)
    fn config_key(&self) -> String {
        format!("{},{},{}", self.base, self.outer, self.inner)
    fn is_coprime(&self) -> bool {
        gcd(self.outer, self.base) == 1 && gcd(self.inner, self.base) == 1
    fn get_expected_rate(&self) -> f64 {
        match (self.base, self.outer, self.inner) {
            (6, 1, 5) | (6, 5, 1) => 33.0,
            (10, 3, 7) => 20.0,
            (30, 11, 7) => 30.0,
            _ => if self.is_coprime() { 15.0 } else { 5.0 }
    fn success_rate(&self) -> f64 {
        if let Some(stats) = self.config_stats.get(&self.config_key()) {
            stats.success_rate
        } else if self.total_generated > 0 {
            (self.total_primes as f64 / self.total_generated as f64) * 100.0
        } else {
            0.0
    fn generate_prime(&mut self) {
        let seed = (self.current_results.len() % 10).to_string();
        let config = self.get_config();
        let start = Instant::now();
        
        match MembraneBuilder::new(config)
            .with_seed(seed.parse::<u8>().unwrap_or(0))
            .build() {
            Ok(particle) => {
                let time_ms = start.elapsed().as_micros() as f64 / 1000.0;
                let is_prime = is_prime(&particle.value);
                
                let result = GenerationResult {
                    number: particle.value.clone(),
                    is_prime,
                    seed: seed.clone(),
                    time_ms,
                };
                // Update results
                self.current_results.push(result);
                if self.current_results.len() > 50 {
                    self.current_results.remove(0);
                }
                // Update totals
                self.total_generated += 1;
                if is_prime {
                    self.total_primes += 1;
                    self.prime_streak += 1;
                    if self.prime_streak > self.best_streak {
                        self.best_streak = self.prime_streak;
                    }
                    
                    // Check patterns
                    let num_str = particle.value.to_string();
                    if num_str == num_str.chars().rev().collect::<String>() {
                        self.interesting_patterns.push(format!("Palindrome: {}", num_str));
                        self.unlock_achievement(Achievement::Palindrome);
                } else {
                    self.prime_streak = 0;
                self.last_was_prime = is_prime;
                self.flash_timer = Instant::now();
                self.construction_step = 0;
                self.construction_timer = Instant::now();
                // Update config stats
                self.update_config_stats(is_prime, time_ms);
                // Update history
                self.success_history.push(if is_prime { 100.0 } else { 0.0 });
                if self.success_history.len() > 50 {
                    self.success_history.remove(0);
                // Check achievements
                self.check_achievements();
                self.first_launch = false;
            }
            Err(_) => {}
    fn update_config_stats(&mut self, is_prime: bool, time_ms: f64) {
        let key = self.config_key();
        let expected = self.get_expected_rate();
        let stats = self.config_stats.entry(key).or_insert(PerformanceStats {
            base: self.base,
            outer: self.outer,
            inner: self.inner,
            success_rate: 0.0,
            expected_rate: expected,
            performance_ratio: 0.0,
            avg_time_ms: 0.0,
        });
        stats.total_generated += 1;
        if is_prime {
            stats.total_primes += 1;
        stats.success_rate = (stats.total_primes as f64 / stats.total_generated as f64) * 100.0;
        stats.performance_ratio = stats.success_rate / stats.expected_rate;
        stats.avg_time_ms = (stats.avg_time_ms * (stats.total_generated - 1) as f64 + time_ms) / stats.total_generated as f64;
    fn check_achievements(&mut self) {
        if self.total_primes >= 1 {
            self.unlock_achievement(Achievement::FirstPrime);
        if self.total_primes >= 10 {
            self.unlock_achievement(Achievement::TenPrimes);
        if self.best_streak >= 5 {
            self.unlock_achievement(Achievement::StreakFive);
        if self.config_stats.len() >= 5 {
            self.unlock_achievement(Achievement::Explorer);
        // Check beat expected
            if stats.total_generated >= 10 && stats.performance_ratio > 1.1 {
                self.unlock_achievement(Achievement::BeatExpected);
    fn unlock_achievement(&mut self, achievement: Achievement) {
        if !self.achievements.contains(&achievement) {
            self.achievements.push(achievement.clone());
            self.pending_achievement = Some((achievement, Instant::now()));
    fn adjust_parameter(&mut self, increase: bool) {
        let delta = if increase { 1 } else { -1 };
        match self.selected_param {
            0 => self.base = (self.base as i32 + delta).max(2).min(36) as u32,
            1 => self.outer = (self.outer as i32 + delta).max(1).min((self.base - 1) as i32) as u32,
            2 => self.inner = (self.inner as i32 + delta).max(1).min((self.base - 1) as i32) as u32,
            3 => self.k_outer = (self.k_outer as i32 + delta).max(0).min(5) as u32,
            4 => self.k_inner = (self.k_inner as i32 + delta).max(0).min(5) as u32,
            _ => {}
        self.prime_streak = 0; // Reset on config change
    fn save_data(&mut self) -> io::Result<()> {
        create_dir_all("membrane_lab_data")?;
        let path = PathBuf::from("membrane_lab_data").join("stats.jsonl");
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        for stats in self.config_stats.values() {
            writeln!(file, "{}", serde_json::to_string(stats)?)?;
        self.unlock_achievement(Achievement::DataScientist);
        Ok(())
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
// ===== MAIN =====
fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // Create app and generate first prime
    let mut app = LabState::default();
    app.generate_prime();
    // Run app
    let res = run_app(&mut terminal, &mut app, Duration::from_millis(50));
    // Cleanup
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
                    // Handle screens
                    if app.show_welcome {
                        match key.code {
                            KeyCode::Enter => app.show_welcome = false,
                            KeyCode::Char('t') => {
                                app.show_welcome = false;
                                app.show_tutorial = true;
                                app.tutorial_step = 0;
                            }
                            KeyCode::Char('q') => return Ok(()),
                            _ => {}
                        }
                    } else if app.show_tutorial {
                            KeyCode::Right | KeyCode::Enter => {
                                if app.tutorial_step < 3 {
                                    app.tutorial_step += 1;
                                } else {
                                    app.show_tutorial = false;
                                }
                            KeyCode::Left => {
                                if app.tutorial_step > 0 {
                                    app.tutorial_step -= 1;
                            KeyCode::Esc => app.show_tutorial = false,
                    } else {
                        // Main screen
                            KeyCode::Char('?') | KeyCode::Char('h') => app.show_help = !app.show_help,
                            KeyCode::Char('w') => app.show_welcome = true,
                            KeyCode::Tab => app.tab_index = (app.tab_index + 1) % 6,
                            KeyCode::BackTab => app.tab_index = (app.tab_index + 5) % 6,
                            KeyCode::Up => {
                                if app.tab_index == 0 {
                                    app.selected_param = (app.selected_param + 4) % 5;
                            KeyCode::Down => {
                                    app.selected_param = (app.selected_param + 1) % 5;
                                    app.adjust_parameter(false);
                            KeyCode::Right => {
                                    app.adjust_parameter(true);
                            KeyCode::Enter => app.generate_prime(),
                            KeyCode::Char('g') => {
                                for _ in 0..10 {
                                    app.generate_prime();
                            KeyCode::Char('s') => {
                                let _ = app.save_data();
                            // Quick configs
                            KeyCode::Char('1') => {
                                app.base = 6;
                                app.outer = 1;
                                app.inner = 5;
                            KeyCode::Char('2') => {
                                app.base = 10;
                                app.outer = 3;
                                app.inner = 7;
        if last_tick.elapsed() >= tick_rate {
            // Update animations
            if app.construction_timer.elapsed() > Duration::from_millis(300) {
                app.construction_step = (app.construction_step + 1).min(3);
                app.construction_timer = Instant::now();
            last_tick = Instant::now();
// ===== UI RENDERING =====
fn ui(f: &mut Frame, app: &LabState) {
    if app.show_welcome {
        draw_welcome(f, app);
    } else if app.show_tutorial {
        draw_tutorial(f, app);
    } else if app.show_help {
        draw_help(f);
    } else {
        draw_main(f, app);
fn draw_main(f: &mut Frame, app: &LabState) {
    let size = f.size();
    // Show achievement popup
    if let Some((achievement, timer)) = &app.pending_achievement {
        if timer.elapsed() < Duration::from_secs(2) {
            draw_achievement_popup(f, size, achievement);
    // Layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(3),  // Tabs
            Constraint::Min(0),     // Content
            Constraint::Length(4),  // Status
        ])
        .split(size);
    // Title
    draw_title(f, chunks[0], app);
    // Tabs
    let tabs = Tabs::new(vec!["Config", "Construction", "Results", "Heat Map", "Stats", "Export"])
        .block(Block::default().borders(Borders::ALL))
        .select(app.tab_index)
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    f.render_widget(tabs, chunks[1]);
    // Content
    match app.tab_index {
        0 => draw_config_tab(f, chunks[2], app),
        1 => draw_construction_tab(f, chunks[2], app),
        2 => draw_results_tab(f, chunks[2], app),
        3 => draw_heat_map_tab(f, chunks[2], app),
        4 => draw_stats_tab(f, chunks[2], app),
        5 => draw_export_tab(f, chunks[2], app),
        _ => {}
    // Status
    draw_status(f, chunks[3], app);
    // First launch hint
    if app.first_launch && app.flash_timer.elapsed() < Duration::from_secs(3) {
        draw_first_hint(f, chunks[2]);
fn draw_title(f: &mut Frame, area: Rect, app: &LabState) {
    let rate = app.success_rate();
    let expected = app.get_expected_rate();
    let title_text = if app.flash_timer.elapsed() < Duration::from_millis(500) {
        if app.last_was_prime {
            if app.prime_streak > 2 {
                format!("🔬 Membrane Laboratory 🔥 {} STREAK! | {:.1}%", app.prime_streak, rate)
            } else {
                format!("🔬 Membrane Laboratory ✨ PRIME! | {:.1}%", rate)
            format!("🔬 Membrane Laboratory ⚠️ Composite | {:.1}%", rate)
        format!("🔬 Membrane Laboratory | {:.1}% (expected: {:.1}%)", rate, expected)
    };
    let color = if rate > expected * 1.1 {
        Color::Green
    } else if rate > expected * 0.9 {
        Color::Yellow
        Color::Red
    let title = Paragraph::new(title_text)
        .style(Style::default().fg(color).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, area);
fn draw_config_tab(f: &mut Frame, area: Rect, app: &LabState) {
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    // Parameters
    let params = vec![
        (format!("Base: {}", app.base), app.selected_param == 0),
        (format!("Outer: {}", app.outer), app.selected_param == 1),
        (format!("Inner: {}", app.inner), app.selected_param == 2),
        (format!("K-outer: {}", app.k_outer), app.selected_param == 3),
        (format!("K-inner: {}", app.k_inner), app.selected_param == 4),
    ];
    let items: Vec<ListItem> = params.iter()
        .map(|(p, sel)| {
            let style = if *sel {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                Style::default()
            };
            ListItem::new(p.as_str()).style(style)
        })
        .collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Parameters "));
    f.render_widget(list, chunks[0]);
    // Stats
    let stats = if let Some(s) = app.config_stats.get(&app.config_key()) {
        vec![
            Line::from(vec![
                Span::raw("Coprime: "),
                if app.is_coprime() {
                    Span::styled("✓", Style::default().fg(Color::Green))
                    Span::styled("✗", Style::default().fg(Color::Red))
                },
            ]),
            Line::from(format!("Expected: {:.1}%", s.expected_rate)),
            Line::from(format!("Actual: {:.1}% ({}/{})", s.success_rate, s.total_primes, s.total_generated)),
            Line::from(format!("Performance: {:.0}%", s.performance_ratio * 100.0)),
            Line::from(""),
            Line::from("Quick: 1=Base6 2=Base10"),
        ]
            Line::from("No data yet"),
            Line::from("Press ENTER to generate!"),
    let stats_widget = Paragraph::new(stats)
        .block(Block::default().borders(Borders::ALL).title(" Stats "));
    f.render_widget(stats_widget, chunks[1]);
fn draw_construction_tab(f: &mut Frame, area: Rect, app: &LabState) {
    let block = Block::default().borders(Borders::ALL).title(" Construction ");
    let inner = block.inner(area);
    f.render_widget(block, area);
    let seed = app.current_results.last()
        .map(|r| r.seed.clone())
        .unwrap_or_else(|| "5".to_string());
    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("Building with seed "),
            Span::styled(&seed, Style::default().fg(Color::Yellow).bold()),
        ]),
    match app.construction_step {
        0 => {
            lines.push(Line::from(vec![
                Span::raw("          "),
                Span::styled(&seed, Style::default().fg(Color::Yellow).bold()),
            ]));
        1 => {
                Span::raw("       "),
                Span::styled(app.inner.to_string(), Style::default().fg(Color::Blue).bold()),
                Span::raw(" + "),
        2 => {
            let middle = format!("{}{}{}", app.inner, seed, app.inner);
                Span::raw("    "),
                Span::styled(app.outer.to_string(), Style::default().fg(Color::Magenta).bold()),
                Span::styled(middle, Style::default().fg(Color::White)),
        _ => {
            if let Some(result) = app.current_results.last() {
                let color = if result.is_prime { Color::Green } else { Color::Red };
                let symbol = if result.is_prime { "✓ PRIME!" } else { "✗ Composite" };
                lines.push(Line::from(vec![
                    Span::raw("       "),
                    Span::styled(result.number.to_string(), Style::default().fg(color).bold()),
                ]));
                lines.push(Line::from(""));
                    Span::styled(symbol, Style::default().fg(color)),
    let construction = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(construction, inner);
fn draw_results_tab(f: &mut Frame, area: Rect, app: &LabState) {
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
    let items: Vec<ListItem> = app.current_results.iter().rev().take(20)
        .map(|r| {
            let style = if r.is_prime {
                Style::default().fg(Color::Green)
                Style::default().fg(Color::Red)
            let text = format!("Seed {}: {} {} ({:.1}ms)", 
                r.seed, r.number, 
                if r.is_prime { "✓" } else { "✗" },
                r.time_ms);
            ListItem::new(text).style(style)
        .block(Block::default().borders(Borders::ALL).title(" Results "));
    // Patterns
    let patterns: Vec<ListItem> = app.interesting_patterns.iter().rev().take(5)
        .map(|p| ListItem::new(p.as_str()).style(Style::default().fg(Color::Yellow)))
    let patterns_list = List::new(patterns)
        .block(Block::default().borders(Borders::ALL).title(" Patterns "));
    f.render_widget(patterns_list, chunks[1]);
fn draw_heat_map_tab(f: &mut Frame, area: Rect, app: &LabState) {
    let block = Block::default().borders(Borders::ALL).title(" Heat Map ");
    let mut lines = vec![];
    let max_digit = (app.base - 1).min(9);
    // Header
    let mut header = vec![Span::raw("   ")];
    for i in 1..=max_digit {
        header.push(Span::raw(format!("{:3}", i)));
    lines.push(Line::from(header));
    lines.push(Line::from(""));
    // Grid
    for outer in 1..=max_digit {
        let mut row = vec![Span::raw(format!("{:2} ", outer))];
        for inner in 1..=max_digit {
            let coprime = gcd(outer, app.base) == 1 && gcd(inner, app.base) == 1;
            let symbol = if coprime { "█" } else { "░" };
            let color = if coprime { Color::Green } else { Color::Red };
            row.push(Span::styled(format!("{:3}", symbol), Style::default().fg(color)));
        lines.push(Line::from(row));
    lines.push(Line::from("█ = Coprime (good)  ░ = Not coprime"));
    let heat_map = Paragraph::new(lines);
    f.render_widget(heat_map, inner);
fn draw_stats_tab(f: &mut Frame, area: Rect, app: &LabState) {
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
    // Config comparison
    let mut configs: Vec<_> = app.config_stats.iter().collect();
    configs.sort_by(|a, b| b.1.success_rate.partial_cmp(&a.1.success_rate).unwrap());
        Line::from("Config    | Rate    | vs Expected"),
        Line::from("─".repeat(35)),
    for (key, stats) in configs.iter().take(10) {
        let color = if stats.performance_ratio > 1.1 {
            Color::Green
        } else if stats.performance_ratio > 0.9 {
            Color::Yellow
            Color::Red
        };
        lines.push(Line::from(vec![
            Span::styled(
                format!("{:9} | {:6.1}% | {:+6.1}%", 
                    key, stats.success_rate, 
                    (stats.performance_ratio - 1.0) * 100.0),
                Style::default().fg(color)
            ),
        ]));
    let comparison = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(" Configs "));
    f.render_widget(comparison, chunks[0]);
    // Achievements
    let achievements: Vec<ListItem> = app.achievements.iter()
        .map(|a| ListItem::new(a.name()).style(Style::default().fg(Color::Yellow)))
    let achievements_list = List::new(achievements)
        .block(Block::default().borders(Borders::ALL)
            .title(format!(" Achievements ({}/7) ", app.achievements.len())));
    f.render_widget(achievements_list, chunks[1]);
fn draw_export_tab(f: &mut Frame, area: Rect, app: &LabState) {
    let text = vec![
            Span::styled("📊 Export Data", Style::default().fg(Color::Yellow).bold()),
        Line::from("Save your session data:"),
        Line::from("• Configuration stats"),
        Line::from("• Success rates"),
        Line::from("• Performance metrics"),
            Span::raw("Press "),
            Span::styled("S", Style::default().fg(Color::Green).bold()),
            Span::raw(" to save"),
        if app.achievements.contains(&Achievement::DataScientist) {
                Span::styled("✓ Data saved!", Style::default().fg(Color::Green)),
            ])
            Line::from("Save to unlock achievement!")
        },
    let export = Paragraph::new(text)
        .alignment(Alignment::Center);
    f.render_widget(export, area);
fn draw_status(f: &mut Frame, area: Rect, app: &LabState) {
        .constraints([Constraint::Length(2), Constraint::Length(2)])
    // Status line
    let status_chunks = Layout::default()
            Constraint::Percentage(25),
        .split(chunks[0]);
    let coprime = if app.is_coprime() { "✓" } else { "✗" };
    let config = Paragraph::new(format!("({},{}) {}", app.outer, app.inner, coprime))
        .style(Style::default().fg(if app.is_coprime() { Color::Green } else { Color::Red }));
    f.render_widget(config, status_chunks[0]);
    let rate = Paragraph::new(format!("{:.1}%", app.success_rate()))
    f.render_widget(rate, status_chunks[1]);
    let streak = Paragraph::new(format!("Streak: {} {}", 
        app.prime_streak,
        if app.prime_streak >= 3 { "🔥" } else { "" }))
    f.render_widget(streak, status_chunks[2]);
    let help = Paragraph::new("? help")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Right);
    f.render_widget(help, status_chunks[3]);
    // Sparkline
    if !app.success_history.is_empty() {
        let data: Vec<u64> = app.success_history.iter()
            .map(|&v| v as u64)
            .collect();
        let sparkline = Sparkline::default()
            .block(Block::default().borders(Borders::TOP))
            .data(&data)
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(sparkline, chunks[1]);
fn draw_welcome(f: &mut Frame, _app: &LabState) {
            Constraint::Percentage(30),
            Constraint::Percentage(50),
            Constraint::Percentage(20),
    let title = vec![
        Line::from("╔═╗┬─┐┬┌┬┐┌─┐  ╔╦╗┌─┐┌┬┐┌┐ ┬─┐┌─┐┌┐┌┌─┐"),
        Line::from("╠═╝├┬┘│││││├┤   ║║║├┤ │││├┴┐├┬┘├─┤│││├┤ "),
        Line::from("╩  ┴└─┴┴ ┴└─┘  ╩ ╩└─┘┴ ┴└─┘┴└─┴ ┴┘└┘└─┘"),
    let title_widget = Paragraph::new(title)
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(title_widget, chunks[0]);
    let content = vec![
        Line::from("Discover prime patterns through symmetric construction"),
        Line::from("✨ Real-time performance tracking"),
        Line::from("🏆 Achievement system"),
        Line::from("📊 Statistical analysis"),
        Line::from("🔥 Streak tracking"),
        Line::from("Best config: Base 6 with (1,5) → 33% success!"),
    let welcome = Paragraph::new(content)
    f.render_widget(welcome, chunks[1]);
    let footer = vec![
            Span::styled("ENTER", Style::default().fg(Color::Green).bold()),
            Span::raw(" to start • "),
            Span::styled("T", Style::default().fg(Color::Yellow)),
            Span::raw(" for tutorial • "),
            Span::styled("Q", Style::default().fg(Color::Red)),
            Span::raw(" to quit"),
    let footer_widget = Paragraph::new(footer)
    f.render_widget(footer_widget, chunks[2]);
fn draw_tutorial(f: &mut Frame, app: &LabState) {
    let content = match app.tutorial_step {
        0 => vec![
            Line::from(vec![Span::styled("Number Bases", Style::default().fg(Color::Green).bold())]),
            Line::from("Different counting systems:"),
            Line::from("• Base 10: 0-9"),
            Line::from("• Base 6: 0-5 (optimal!)"),
            Line::from("• Base 2: 0-1"),
        ],
        1 => vec![
            Line::from(vec![Span::styled("Membrane Construction", Style::default().fg(Color::Green).bold())]),
            Line::from("Symmetric layers:"),
            Line::from("outer + inner + seed + inner + outer"),
            Line::from("Example: 1 + 5 + 3 + 5 + 1 = 15351"),
        2 => vec![
            Line::from(vec![Span::styled("Coprimality", Style::default().fg(Color::Green).bold())]),
            Line::from("Digits must be coprime to base"),
            Line::from("gcd(digit, base) = 1"),
            Line::from("This is the key to success!"),
        3 => vec![
            Line::from(vec![Span::styled("Quick Start", Style::default().fg(Color::Green).bold())]),
            Line::from("• ENTER - Generate prime"),
            Line::from("• G - Generate batch"),
            Line::from("• Tab - Switch views"),
            Line::from("• 1/2 - Quick configs"),
        _ => vec![],
    let tutorial = Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!(" Tutorial {}/4 ", app.tutorial_step + 1)))
    let area = centered_rect(60, 60, size);
    f.render_widget(Clear, area);
    f.render_widget(tutorial, area);
fn draw_help(f: &mut Frame) {
    let area = centered_rect(60, 70, size);
    let help = vec![
        Line::from("Membrane Laboratory - Help"),
        Line::from("Navigation:"),
        Line::from("  Tab     - Switch tabs"),
        Line::from("  ↑/↓     - Select parameter"),
        Line::from("  ←/→     - Adjust value"),
        Line::from("Actions:"),
        Line::from("  Enter   - Generate single"),
        Line::from("  G       - Generate batch"),
        Line::from("  S       - Save data"),
        Line::from("  1/2     - Quick configs"),
        Line::from("  W       - Welcome screen"),
        Line::from("  T       - Tutorial"),
        Line::from("  ?       - This help"),
        Line::from("  Q       - Quit"),
    let help_widget = Paragraph::new(help)
            .title(" Help ")
            .style(Style::default().bg(Color::Black)))
    f.render_widget(help_widget, area);
fn draw_achievement_popup(f: &mut Frame, area: Rect, achievement: &Achievement) {
    let popup = centered_rect(40, 20, area);
            Span::styled("🎉 Achievement! 🎉", Style::default().fg(Color::Yellow).bold()),
            Span::styled(achievement.name(), Style::default().fg(Color::Green).bold()),
    let achievement_widget = Paragraph::new(text)
    f.render_widget(Clear, popup);
    f.render_widget(achievement_widget, popup);
fn draw_first_hint(f: &mut Frame, area: Rect) {
    let hint = Rect {
        x: area.x + area.width / 2 - 25,
        y: area.y + area.height / 2 - 2,
        width: 50,
        height: 5,
            Span::styled("Welcome! ", Style::default().fg(Color::Yellow).bold()),
            Span::raw("We generated your first prime!"),
            Span::raw("Try "),
            Span::styled("G", Style::default().fg(Color::Green).bold()),
            Span::raw(" for batch • "),
            Span::styled("Tab", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" to explore"),
    let hint_widget = Paragraph::new(text)
    f.render_widget(Clear, hint);
    f.render_widget(hint_widget, hint);
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
        .split(r);
    Layout::default()
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
        .split(popup_layout[1])[1]
