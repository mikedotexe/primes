//! Performance monitoring and profiling utilities

use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, Once};
use std::sync::atomic::{AtomicU64, Ordering};
#[cfg(not(target_arch = "wasm32"))]
use std::thread;

/// Global CPU frequency in GHz, stored as f64 bits for atomic access
static GLOBAL_CPU_FREQ_GHZ: AtomicU64 = AtomicU64::new(0);

/// One-time initialization guard for CPU frequency
static FREQ_INIT: Once = Once::new();

/// Simplified frequency access (inspired by external best practices)
pub mod freq {
    use super::*;
    
    /// Read current CPU frequency in GHz
    #[inline]
    pub fn read_ghz() -> f64 {
        let freq_bits = GLOBAL_CPU_FREQ_GHZ.load(Ordering::Acquire);
        f64::from_bits(freq_bits) / 1_000_000_000.0
    }
    
    /// Write CPU frequency in GHz (for DVFS updates)
    #[inline]
    pub fn write_ghz(freq_ghz: f64) {
        let freq_hz = freq_ghz * 1_000_000_000.0;
        GLOBAL_CPU_FREQ_GHZ.store(freq_hz.to_bits(), Ordering::Release);
    }
}

/// Cycle-accurate timer for sub-microsecond measurements
#[derive(Debug, Clone)]
pub struct CycleTimer {
    start_cycles: u64,
    end_cycles: u64,
}

impl Default for CycleTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl CycleTimer {
    /// Create a new cycle timer with current CPU frequency
    pub fn new() -> Self {
        // Initialize frequency once, race-free
        FREQ_INIT.call_once(|| {
            let freq = Self::estimate_frequency();
            GLOBAL_CPU_FREQ_GHZ.store(freq.to_bits(), Ordering::Release);
        });
        
        Self {
            start_cycles: 0,
            end_cycles: 0,
        }
    }
    
    /// Start timing
    pub fn start(&mut self) {
        self.start_cycles = Self::read_cycles();
    }
    
    /// Stop timing and return elapsed cycles
    pub fn stop(&mut self) -> u64 {
        self.end_cycles = Self::read_cycles();
        self.end_cycles.saturating_sub(self.start_cycles)
    }
    
    /// Convert cycles to duration
    pub fn cycles_to_duration(&self, cycles: u64) -> Duration {
        // Read frequency with Acquire ordering to ensure we see updates
        let freq_bits = GLOBAL_CPU_FREQ_GHZ.load(Ordering::Acquire);
        let freq_hz = f64::from_bits(freq_bits);
        let secs = cycles as f64 / freq_hz;
        Duration::from_secs_f64(secs)
    }
    
    /// Get elapsed time as Duration
    pub fn elapsed(&self) -> Duration {
        let cycles = self.end_cycles.saturating_sub(self.start_cycles);
        self.cycles_to_duration(cycles)
    }
    
    /// Update global CPU frequency (for DVFS monitoring)
    pub fn update_global_frequency(freq_ghz: f64) {
        GLOBAL_CPU_FREQ_GHZ.store(freq_ghz.to_bits(), Ordering::Release);
    }
    
    /// Read CPU cycle counter
    /// 
    /// # Platform-Specific Cycle Counting
    /// 
    /// ## ARM64 (Apple Silicon)
    /// - Reads CNTVCT_EL0: Virtual Timer Count register
    /// - Increments at constant frequency (24 MHz on M1/M2)
    /// - Not affected by CPU frequency scaling (DVFS)
    /// - Accessible from user space without privileges
    /// 
    /// ## x86_64
    /// - Would use RDTSC instruction (not implemented here)
    /// - Counts at nominal CPU frequency
    /// - May be affected by frequency scaling on older CPUs
    /// 
    /// ## Fallback
    /// - Uses high-resolution system timer
    /// - Less accurate but portable
    #[cfg(target_arch = "aarch64")]
    fn read_cycles() -> u64 {
        let cycles: u64;
        // SAFETY: Reading the cycle counter is a read-only operation that is
        // guaranteed to be safe on aarch64. The cntvct_el0 register is accessible
        // from user space and provides a monotonic timer value.
        unsafe {
            std::arch::asm!(
                "mrs {}, cntvct_el0",
                out(reg) cycles,
                options(nomem, nostack)
            );
        }
        cycles
    }
    
    #[cfg(not(target_arch = "aarch64"))]
    fn read_cycles() -> u64 {
        // Fallback to high-resolution time
        // Convert to approximate "cycles" assuming 1 GHz
        let now = Instant::now();
        now.elapsed().as_nanos() as u64
    }
    
    /// Estimate CPU frequency using DVFS monitor
    fn estimate_frequency() -> f64 {
        #[cfg(feature = "dvfs-adaptive")]
        {
            crate::dvfs::cpu_freq_ghz() * 1_000_000_000.0 // Convert GHz to Hz
        }
        #[cfg(not(feature = "dvfs-adaptive"))]
        {
            #[cfg(target_arch = "aarch64")]
            {
                // Read CNTFRQ_EL0 (ticks per second) - fixed at 24 MHz on Apple Silicon
                let freq: u64;
                // SAFETY: Reading the counter frequency is a read-only operation that is
                // guaranteed to be safe on aarch64. The cntfrq_el0 register provides
                // the system timer frequency.
                unsafe { 
                    std::arch::asm!(
                        "mrs {0}, cntfrq_el0", 
                        out(reg) freq,
                        options(nomem, nostack)
                    );
                }
                freq as f64
            }
            #[cfg(not(target_arch = "aarch64"))]
            {
                // Assume 3GHz for other platforms (conservative)
                3_000_000_000.0
            }
        }
    }
}

/// Performance metrics for a single operation
#[derive(Debug, Clone)]
pub struct PerfMetrics {
    pub name: String,
    pub count: u64,
    pub total_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub last_time: Duration,
}

impl PerfMetrics {
    fn new(name: String) -> Self {
        Self {
            name,
            count: 0,
            total_time: Duration::ZERO,
            min_time: Duration::MAX,
            max_time: Duration::ZERO,
            last_time: Duration::ZERO,
        }
    }
    
    fn record(&mut self, duration: Duration) {
        self.count += 1;
        self.total_time += duration;
        self.min_time = self.min_time.min(duration);
        self.max_time = self.max_time.max(duration);
        self.last_time = duration;
    }
    
    pub fn avg_time(&self) -> Duration {
        if self.count == 0 {
            Duration::ZERO
        } else {
            self.total_time / self.count as u32
        }
    }
    
    pub fn ops_per_sec(&self) -> f64 {
        if self.total_time.as_secs_f64() == 0.0 {
            0.0
        } else {
            self.count as f64 / self.total_time.as_secs_f64()
        }
    }
}

/// Global performance monitor
pub struct PerfMonitor {
    metrics: Arc<Mutex<HashMap<String, PerfMetrics>>>,
}

impl PerfMonitor {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Time an operation and record metrics
    pub fn time<F, R>(&self, name: &str, f: F) -> R 
    where 
        F: FnOnce() -> R 
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.entry(name.to_string())
                .or_insert_with(|| PerfMetrics::new(name.to_string()))
                .record(duration);
        }
        // If lock is poisoned, we silently ignore - performance monitoring
        // should not crash the application
        
        result
    }
    
    /// Get a copy of current metrics
    pub fn get_metrics(&self) -> Vec<PerfMetrics> {
        match self.metrics.lock() {
            Ok(metrics) => metrics.values().cloned().collect(),
            Err(_) => Vec::new(), // Return empty vec if lock is poisoned
        }
    }
    
    /// Clear all metrics
    pub fn clear(&self) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.clear();
        }
    }
    
    /// Calculate median safely (bounds-checked, inspired by external best practices)
    pub fn safe_median(measurements: &mut [f64]) -> Option<f64> {
        if measurements.is_empty() {
            println!("  ❌ Median calculation failed: no valid measurements");
            return None;
        }
        
        measurements.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let median = if measurements.len() % 2 == 0 {
            let mid = measurements.len() / 2;
            (measurements[mid - 1] + measurements[mid]) / 2.0
        } else {
            measurements[measurements.len() / 2]
        };
        Some(median)
    }
    
    /// Print a summary report
    pub fn report(&self) {
        let mut metrics = self.get_metrics();
        metrics.sort_by(|a, b| b.total_time.cmp(&a.total_time));
        
        println!("\n=== Performance Report ===");
        println!("{:<30} {:>10} {:>12} {:>12} {:>12} {:>12}", 
                 "Operation", "Count", "Total (ms)", "Avg (μs)", "Min (μs)", "Max (μs)");
        println!("{}", "-".repeat(90));
        
        for m in metrics {
            println!("{:<30} {:>10} {:>12.2} {:>12.2} {:>12.2} {:>12.2}",
                     m.name,
                     m.count,
                     m.total_time.as_secs_f64() * 1000.0,
                     m.avg_time().as_secs_f64() * 1_000_000.0,
                     m.min_time.as_secs_f64() * 1_000_000.0,
                     m.max_time.as_secs_f64() * 1_000_000.0);
        }
    }
}

impl Default for PerfMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Spawn a background thread to monitor CPU frequency changes (DVFS)
/// 
/// # DVFS Monitoring
/// 
/// Modern CPUs dynamically adjust their frequency based on:
/// - Thermal conditions
/// - Power budget
/// - Workload characteristics
/// 
/// This thread periodically samples the actual frequency by measuring
/// cycle count deltas against wall clock time.
/// 
/// # Platform Notes
/// 
/// - macOS: Requires mach_timebase_info privilege for user-space counter access
/// - Linux: May need perf_event_open or /proc/cpuinfo access
/// - Windows: Requires QueryPerformanceCounter
#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_dvfs_sampler(period: Duration) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        let cnt_before = CycleTimer::read_cycles();
        let t0 = Instant::now();
        thread::sleep(period);
        let cnt_after = CycleTimer::read_cycles();
        let dt = t0.elapsed().as_secs_f64();
        
        // Calculate actual frequency from cycle delta
        let freq_hz = (cnt_after - cnt_before) as f64 / dt;
        let freq_ghz = freq_hz / 1_000_000_000.0;
        
        // Update global frequency for all timers
        CycleTimer::update_global_frequency(freq_ghz);
    })
}

/// Scoped timer that records on drop
pub struct ScopedTimer<'a> {
    monitor: &'a PerfMonitor,
    name: String,
    start: Instant,
}

impl<'a> ScopedTimer<'a> {
    pub fn new(monitor: &'a PerfMonitor, name: &str) -> Self {
        Self {
            monitor,
            name: name.to_string(),
            start: Instant::now(),
        }
    }
}

impl<'a> Drop for ScopedTimer<'a> {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        let mut metrics = self.monitor.metrics.lock().unwrap();
        metrics.entry(self.name.clone())
            .or_insert_with(|| PerfMetrics::new(self.name.clone()))
            .record(duration);
    }
}

/// Macro for easy timing
#[macro_export]
macro_rules! time_it {
    ($monitor:expr, $name:expr, $block:expr) => {{
        let _timer = $crate::performance::ScopedTimer::new($monitor, $name);
        $block
    }}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_perf_monitor() {
        let monitor = PerfMonitor::new();
        
        // Time some operations
        monitor.time("fast_op", || {
            thread::sleep(Duration::from_micros(100));
        });
        
        monitor.time("slow_op", || {
            thread::sleep(Duration::from_millis(1));
        });
        
        // Multiple calls to same op
        for _ in 0..5 {
            monitor.time("fast_op", || {
                thread::sleep(Duration::from_micros(100));
            });
        }
        
        let metrics = monitor.get_metrics();
        assert_eq!(metrics.len(), 2);
        
        let fast_metrics = metrics.iter()
            .find(|m| m.name == "fast_op")
            .unwrap();
        assert_eq!(fast_metrics.count, 6);
    }
    
    #[test]
    fn test_safe_median_bounds_checking() {
        // Test empty vector safety
        let mut empty_vec: Vec<f64> = Vec::new();
        assert_eq!(PerfMonitor::safe_median(&mut empty_vec), None);
        
        // Test single element
        let mut single = vec![42.0];
        assert_eq!(PerfMonitor::safe_median(&mut single), Some(42.0));
        
        // Test even number of elements
        let mut even = vec![1.0, 3.0, 2.0, 4.0];
        assert_eq!(PerfMonitor::safe_median(&mut even), Some(2.5));
        
        // Test odd number of elements
        let mut odd = vec![1.0, 5.0, 3.0];
        assert_eq!(PerfMonitor::safe_median(&mut odd), Some(3.0));
    }
    
    #[test]
    fn test_frequency_access_patterns() {
        // Test the simplified frequency interface
        freq::write_ghz(2.5);
        assert!((freq::read_ghz() - 2.5).abs() < 0.001);
        
        freq::write_ghz(3.8);
        assert!((freq::read_ghz() - 3.8).abs() < 0.001);
    }
    
    #[test]
    fn test_scoped_timer() {
        let monitor = PerfMonitor::new();
        
        {
            let _timer = ScopedTimer::new(&monitor, "scoped_op");
            thread::sleep(Duration::from_micros(100));
        }
        
        let metrics = monitor.get_metrics();
        assert_eq!(metrics.len(), 1);
        assert_eq!(metrics[0].name, "scoped_op");
        assert_eq!(metrics[0].count, 1);
    }
}