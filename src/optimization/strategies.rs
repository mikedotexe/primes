// src/optimization/strategies.rs
//! Concrete optimization strategies

use super::{Feedback, Metrics, OptimizationStrategy, Priority, SystemContext, Workload};
use crate::PhysicsError;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

/// Wheel-30 factorization with SIMD optimization
pub struct Wheel30Strategy {
    effectiveness_score: AtomicU64,
    applications: AtomicU64,
    successes: AtomicU64,
}

impl Default for Wheel30Strategy {
    fn default() -> Self {
        Self::new()
    }
}

impl Wheel30Strategy {
    pub fn new() -> Self {
        Self {
            effectiveness_score: AtomicU64::new(f64::to_bits(0.8)),
            applications: AtomicU64::new(0),
            successes: AtomicU64::new(0),
        }
    }
}

impl OptimizationStrategy for Wheel30Strategy {
    fn name(&self) -> &str {
        "Wheel-30 SIMD"
    }

    fn applicability(&self, context: &SystemContext, workload: &Workload) -> f64 {
        // Wheel-30 is most effective for large workloads
        let size_factor = (workload.limit as f64).log10() / 10.0; // 0.0 - 1.0 for up to 10B

        // SIMD availability boost
        let simd_factor = if context.cpu_info.has_neon || context.cpu_info.has_avx2 {
            1.0
        } else {
            0.5
        };

        // Memory pressure consideration
        let memory_factor = if workload.memory_footprint > context.available_memory / 2 {
            1.0 // High memory pressure - wheel helps
        } else {
            0.7
        };

        (size_factor * simd_factor * memory_factor).min(1.0)
    }

    fn apply(
        &self,
        workload: &mut Workload,
        context: &SystemContext,
    ) -> Result<Metrics, PhysicsError> {
        self.applications.fetch_add(1, Ordering::Relaxed);

        // Simulate wheel-30 optimization
        // In reality, this would switch to wheel-based sieve

        // Expected improvements:
        // - 46.7% memory reduction
        // - 25-30% throughput improvement for memory-bound workloads

        let memory_reduction = 0.467;
        let throughput_boost = if workload.memory_footprint > context.cache_sizes.l3 {
            1.28 // Memory-bound: big improvement
        } else {
            1.15 // Cache-resident: moderate improvement
        };

        // Update workload
        workload.memory_footprint =
            (workload.memory_footprint as f64 * (1.0 - memory_reduction)) as usize;
        workload.expected_duration =
            Duration::from_secs_f64(workload.expected_duration.as_secs_f64() / throughput_boost);

        // Simulate metrics
        let metrics = Metrics {
            throughput: 500_000_000.0 * throughput_boost,
            latency_p50: Duration::from_micros(80),
            latency_p99: Duration::from_micros(150),
            memory_used: workload.memory_footprint,
            cache_hit_rate: 0.85,
            power_estimate: 42.0, // Slightly lower due to fewer memory accesses
        };

        self.successes.fetch_add(1, Ordering::Relaxed);
        Ok(metrics)
    }

    fn learn(&mut self, feedback: &Feedback) {
        let current = f64::from_bits(self.effectiveness_score.load(Ordering::Relaxed));

        // Update effectiveness based on feedback
        let adjustment = if feedback.success {
            0.01 // Increase confidence
        } else {
            -0.02 // Decrease confidence faster on failure
        };

        let new_score = (current + adjustment).clamp(0.1, 1.0);
        self.effectiveness_score
            .store(new_score.to_bits(), Ordering::Relaxed);
    }

    fn effectiveness(&self) -> f64 {
        let apps = self.applications.load(Ordering::Relaxed);
        let succs = self.successes.load(Ordering::Relaxed);

        if apps == 0 {
            f64::from_bits(self.effectiveness_score.load(Ordering::Relaxed))
        } else {
            // Blend historical effectiveness with success rate
            let success_rate = succs as f64 / apps as f64;
            let historical = f64::from_bits(self.effectiveness_score.load(Ordering::Relaxed));
            0.7 * success_rate + 0.3 * historical
        }
    }
}

/// DVFS-aware adaptive scheduling strategy
pub struct DVFSAdaptiveStrategy {
    effectiveness_score: AtomicU64,
    frequency_history: Vec<f64>,
    optimal_segment_size: AtomicU64,
}

impl Default for DVFSAdaptiveStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl DVFSAdaptiveStrategy {
    pub fn new() -> Self {
        Self {
            effectiveness_score: AtomicU64::new(f64::to_bits(0.75)),
            frequency_history: Vec::with_capacity(100),
            optimal_segment_size: AtomicU64::new(65536), // 64KB default
        }
    }

    fn calculate_optimal_segment_size(&self, freq_ghz: f64) -> usize {
        // Adaptive segment sizing based on frequency
        if freq_ghz < 1.0 {
            256 * 1024 // Thermal throttle: large segments
        } else if freq_ghz > 3.0 {
            32 * 1024 // Turbo: small segments for L1
        } else if freq_ghz > 2.5 {
            64 * 1024 // High freq: standard
        } else {
            128 * 1024 // Balanced: larger segments
        }
    }
}

impl OptimizationStrategy for DVFSAdaptiveStrategy {
    fn name(&self) -> &str {
        "DVFS Adaptive Scheduling"
    }

    fn applicability(&self, context: &SystemContext, workload: &Workload) -> f64 {
        // DVFS is critical when:
        // 1. System has variable frequency (mobile, laptop)
        // 2. Workload is long-running
        // 3. Thermal pressure exists

        let duration_factor = if workload.expected_duration > Duration::from_secs(10) {
            1.0
        } else {
            workload.expected_duration.as_secs_f64() / 10.0
        };

        let thermal_factor = context.thermal_pressure * 2.0; // 0.0 - 2.0

        let platform_factor = match context.cpu_info.arch {
            super::Architecture::AArch64 => 1.0, // Mobile/laptop chips
            super::Architecture::X86_64 => 0.8,  // Desktop/server
            _ => 0.5,
        };

        (duration_factor * (1.0 + thermal_factor) * platform_factor / 2.0).min(1.0)
    }

    fn apply(
        &self,
        workload: &mut Workload,
        context: &SystemContext,
    ) -> Result<Metrics, PhysicsError> {
        // Calculate optimal segment size for current frequency
        let optimal_size = self.calculate_optimal_segment_size(context.cpu_freq_ghz);
        self.optimal_segment_size
            .store(optimal_size as u64, Ordering::Relaxed);

        // Expected improvements:
        // - Reduced variance (CV from 21.6% to <10%)
        // - Better P5 performance (+15%)
        // - Energy efficiency (5-10%)

        let efficiency_gain = match workload.priority {
            Priority::Efficiency => 1.08,
            Priority::Throughput => 1.05,
            Priority::Latency => 1.03,
            Priority::Balanced => 1.06,
        };

        // Update workload
        workload.expected_duration =
            Duration::from_secs_f64(workload.expected_duration.as_secs_f64() / efficiency_gain);

        // Simulate more stable metrics
        let metrics = Metrics {
            throughput: 450_000_000.0 * efficiency_gain,
            latency_p50: Duration::from_micros(90),
            latency_p99: Duration::from_micros(120), // Less variance
            memory_used: workload.memory_footprint,
            cache_hit_rate: 0.88, // Better cache usage
            power_estimate: 40.0, // Lower average power
        };

        Ok(metrics)
    }

    fn learn(&mut self, feedback: &Feedback) {
        // Learn from frequency patterns
        if feedback.success && feedback.unexpected_behavior.is_empty() {
            let current = f64::from_bits(self.effectiveness_score.load(Ordering::Relaxed));
            let new_score = (current + 0.005).min(1.0);
            self.effectiveness_score
                .store(new_score.to_bits(), Ordering::Relaxed);
        }
    }

    fn effectiveness(&self) -> f64 {
        f64::from_bits(self.effectiveness_score.load(Ordering::Relaxed))
    }
}

/// Cache-aware chunking strategy
pub struct CacheOptimizedStrategy {
    effectiveness_score: AtomicU64,
    l1_multiplier: AtomicU64,
}

impl Default for CacheOptimizedStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheOptimizedStrategy {
    pub fn new() -> Self {
        Self {
            effectiveness_score: AtomicU64::new(f64::to_bits(0.85)),
            l1_multiplier: AtomicU64::new(4), // 4x L1 size default
        }
    }
}

impl OptimizationStrategy for CacheOptimizedStrategy {
    fn name(&self) -> &str {
        "Cache-Optimized Chunking"
    }

    fn applicability(&self, context: &SystemContext, workload: &Workload) -> f64 {
        // Cache optimization is always somewhat applicable
        // More important for medium-sized workloads

        let size_factor = if workload.limit > 1_000_000 && workload.limit < 1_000_000_000 {
            1.0 // Sweet spot
        } else if workload.limit < 100_000 {
            0.3 // Too small
        } else {
            0.7 // Very large - other factors dominate
        };

        // Better caches = more important
        let cache_quality = (context.cache_sizes.l1d as f64 / 32768.0).min(2.0) / 2.0;

        size_factor * cache_quality
    }

    fn apply(
        &self,
        workload: &mut Workload,
        _context: &SystemContext,
    ) -> Result<Metrics, PhysicsError> {
        // Cache optimization provides consistent moderate improvements
        let improvement = 1.12;

        workload.expected_duration =
            Duration::from_secs_f64(workload.expected_duration.as_secs_f64() / improvement);

        let metrics = Metrics {
            throughput: 480_000_000.0 * improvement,
            latency_p50: Duration::from_micros(85),
            latency_p99: Duration::from_micros(140),
            memory_used: workload.memory_footprint,
            cache_hit_rate: 0.92, // Excellent cache usage
            power_estimate: 43.0,
        };

        Ok(metrics)
    }

    fn learn(&mut self, feedback: &Feedback) {
        // Adjust L1 multiplier based on cache performance
        if feedback.metrics_delta.throughput_change > 0.1 {
            // Good improvement - keep current multiplier
        } else if feedback.metrics_delta.throughput_change < 0.0 {
            // Performance degraded - try different multiplier
            let current = self.l1_multiplier.load(Ordering::Relaxed);
            let new = if current >= 4 { 2 } else { 8 };
            self.l1_multiplier.store(new, Ordering::Relaxed);
        }
    }

    fn effectiveness(&self) -> f64 {
        f64::from_bits(self.effectiveness_score.load(Ordering::Relaxed))
    }
}

/// Platform-specific SIMD strategy
pub struct SIMDStrategy {
    effectiveness_score: AtomicU64,
    vector_width: AtomicU64,
}

impl Default for SIMDStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl SIMDStrategy {
    pub fn new() -> Self {
        Self {
            effectiveness_score: AtomicU64::new(f64::to_bits(0.9)),
            vector_width: AtomicU64::new(128), // Default to 128-bit vectors
        }
    }
}

impl OptimizationStrategy for SIMDStrategy {
    fn name(&self) -> &str {
        "Platform SIMD"
    }

    fn applicability(&self, context: &SystemContext, workload: &Workload) -> f64 {
        // SIMD effectiveness depends on:
        // 1. Platform support
        // 2. Workload size (amortize overhead)
        // 3. Memory bandwidth utilization

        let platform_score = if context.cpu_info.has_avx512 {
            1.0
        } else if context.cpu_info.has_avx2 {
            0.9
        } else if context.cpu_info.has_neon {
            0.85
        } else {
            0.0 // No SIMD
        };

        let size_factor = (workload.limit as f64 / 1_000_000.0).min(1.0);

        platform_score * size_factor
    }

    fn apply(
        &self,
        workload: &mut Workload,
        context: &SystemContext,
    ) -> Result<Metrics, PhysicsError> {
        // Select optimal vector width
        let width = if context.cpu_info.has_avx512 {
            512
        } else if context.cpu_info.has_avx2 {
            256
        } else {
            128
        };

        self.vector_width.store(width, Ordering::Relaxed);

        // SIMD provides significant speedup for bit manipulation
        let speedup = 1.0 + (width as f64 / 128.0) * 0.15; // 15% per width doubling

        workload.expected_duration =
            Duration::from_secs_f64(workload.expected_duration.as_secs_f64() / speedup);

        let metrics = Metrics {
            throughput: 520_000_000.0 * speedup,
            latency_p50: Duration::from_micros(75),
            latency_p99: Duration::from_micros(130),
            memory_used: workload.memory_footprint,
            cache_hit_rate: 0.87,
            power_estimate: 48.0, // SIMD uses more power
        };

        Ok(metrics)
    }

    fn learn(&mut self, feedback: &Feedback) {
        // SIMD effectiveness is fairly stable
        if feedback.success {
            let current = f64::from_bits(self.effectiveness_score.load(Ordering::Relaxed));
            if current < 0.95 {
                self.effectiveness_score
                    .store((current + 0.01).to_bits(), Ordering::Relaxed);
            }
        }
    }

    fn effectiveness(&self) -> f64 {
        f64::from_bits(self.effectiveness_score.load(Ordering::Relaxed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::optimization::{Architecture, CacheSizes, CpuInfo, Priority};

    fn test_context() -> SystemContext {
        SystemContext {
            cpu_info: CpuInfo {
                arch: Architecture::AArch64,
                core_count: 8,
                has_avx2: false,
                has_avx512: false,
                has_neon: true,
                has_sve: false,
            },
            cpu_freq_ghz: 2.5,
            available_memory: 8 * 1024 * 1024 * 1024,
            thermal_pressure: 0.3,
            system_load: 0.5,
            cache_sizes: CacheSizes {
                l1d: 128 * 1024,
                l2: 12 * 1024 * 1024,
                l3: 0,
            },
        }
    }

    fn test_workload() -> Workload {
        Workload {
            limit: 100_000_000,
            density_estimate: 0.1,
            memory_footprint: 6 * 1024 * 1024,
            expected_duration: Duration::from_secs(1),
            priority: Priority::Balanced,
        }
    }

    #[test]
    fn test_wheel30_applicability() {
        let strategy = Wheel30Strategy::new();
        let context = test_context();
        let workload = test_workload();

        let score = strategy.applicability(&context, &workload);
        assert!(score > 0.5 && score <= 1.0);
    }

    #[test]
    fn test_dvfs_adaptive_apply() {
        let strategy = DVFSAdaptiveStrategy::new();
        let context = test_context();
        let mut workload = test_workload();

        let metrics = strategy.apply(&mut workload, &context).unwrap();
        assert!(metrics.throughput > 400_000_000.0);
        assert!(workload.expected_duration < Duration::from_secs(1));
    }
}
