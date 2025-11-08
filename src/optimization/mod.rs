// src/optimization/mod.rs
//! Holistic optimization framework for self-improving performance

#![allow(dead_code)] // Framework designed for future extensibility

use std::sync::Arc;
use std::time::Duration;
use crate::PhysicsError;

pub mod strategies;
pub mod telemetry;
pub mod prediction;

/// System context captures current runtime environment
#[derive(Debug, Clone)]
pub struct SystemContext {
    /// CPU architecture and capabilities
    pub cpu_info: CpuInfo,
    /// Current frequency in GHz
    pub cpu_freq_ghz: f64,
    /// Available memory in bytes
    pub available_memory: usize,
    /// Current thermal state (0.0 = cold, 1.0 = throttling)
    pub thermal_pressure: f64,
    /// System load (0.0 = idle, 1.0 = saturated)
    pub system_load: f64,
    /// Cache sizes in bytes
    pub cache_sizes: CacheSizes,
}

#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub arch: Architecture,
    pub core_count: usize,
    pub has_avx2: bool,
    pub has_avx512: bool,
    pub has_neon: bool,
    pub has_sve: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Architecture {
    X86_64,
    AArch64,
    Wasm32,
    Other,
}

#[derive(Debug, Clone)]
pub struct CacheSizes {
    pub l1d: usize,
    pub l2: usize,
    pub l3: usize,
}

/// Workload characteristics
#[derive(Debug, Clone)]
pub struct Workload {
    pub limit: usize,
    pub density_estimate: f64,
    pub memory_footprint: usize,
    pub expected_duration: Duration,
    pub priority: Priority,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Priority {
    Latency,     // Minimize time to first result
    Throughput,  // Maximize results per second
    Efficiency,  // Minimize power consumption
    Balanced,    // Balance all objectives
}

/// Performance metrics from optimization
#[derive(Debug, Clone)]
pub struct Metrics {
    pub throughput: f64,
    pub latency_p50: Duration,
    pub latency_p99: Duration,
    pub memory_used: usize,
    pub cache_hit_rate: f64,
    pub power_estimate: f64,
}

/// Feedback for learning
#[derive(Debug, Clone)]
pub struct Feedback {
    pub metrics_delta: MetricsDelta,
    pub success: bool,
    pub unexpected_behavior: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MetricsDelta {
    pub throughput_change: f64,
    pub latency_change: f64,
    pub memory_change: f64,
    pub power_change: f64,
}

/// Core trait for optimization strategies
pub trait OptimizationStrategy: Send + Sync {
    /// Strategy identifier
    fn name(&self) -> &str;
    
    /// How applicable is this strategy to the current context? (0.0-1.0)
    fn applicability(&self, context: &SystemContext, workload: &Workload) -> f64;
    
    /// Apply the optimization
    fn apply(&self, workload: &mut Workload, context: &SystemContext) -> Result<Metrics, PhysicsError>;
    
    /// Learn from feedback to improve future decisions
    fn learn(&mut self, feedback: &Feedback);
    
    /// Get current effectiveness score
    fn effectiveness(&self) -> f64;
}

/// Optimization pipeline that combines multiple strategies
pub struct OptimizationPipeline {
    strategies: Vec<Box<dyn OptimizationStrategy>>,
    telemetry: Arc<telemetry::TelemetrySystem>,
    predictor: Option<prediction::OptimizationPredictor>,
}

impl OptimizationPipeline {
    pub fn new(telemetry: Arc<telemetry::TelemetrySystem>) -> Self {
        Self {
            strategies: Vec::new(),
            telemetry,
            predictor: None,
        }
    }
    
    /// Add an optimization strategy to the pipeline
    pub fn add_strategy(&mut self, strategy: Box<dyn OptimizationStrategy>) {
        self.strategies.push(strategy);
    }
    
    /// Enable machine learning predictor
    pub fn enable_ml_predictor(&mut self) {
        self.predictor = Some(prediction::OptimizationPredictor::new());
    }
    
    /// Rank strategies by predicted effectiveness (returns indices)
    fn rank_strategies(&self, context: &SystemContext, workload: &Workload) -> Vec<usize> {
        let mut indices: Vec<_> = (0..self.strategies.len()).collect();
        
        indices.sort_by(|&i, &j| {
            let score_i = self.strategies[i].applicability(context, workload) * self.strategies[i].effectiveness();
            let score_j = self.strategies[j].applicability(context, workload) * self.strategies[j].effectiveness();
            score_j.partial_cmp(&score_i).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        indices
    }
    
    /// Automatically optimize a workload
    pub fn auto_optimize(&mut self, mut workload: Workload) -> Result<OptimizedWorkload, PhysicsError> {
        let context = self.telemetry.capture_context();
        let ranked_indices = self.rank_strategies(&context, &workload);
        
        let mut total_metrics = Metrics {
            throughput: 0.0,
            latency_p50: Duration::ZERO,
            latency_p99: Duration::ZERO,
            memory_used: 0,
            cache_hit_rate: 0.0,
            power_estimate: 0.0,
        };
        
        let mut applied_strategies = Vec::new();
        
        for &idx in &ranked_indices {
            // Skip if applicability is too low
            if self.strategies[idx].applicability(&context, &workload) < 0.3 {
                continue;
            }
            
            let before = self.telemetry.snapshot();
            let strategy_name = self.strategies[idx].name().to_string();
            
            match self.strategies[idx].apply(&mut workload, &context) {
                Ok(metrics) => {
                    let after = self.telemetry.snapshot();
                    let feedback = Feedback::from_snapshots(before, after, &metrics);
                    
                    // Let strategy learn from this application
                    self.strategies[idx].learn(&feedback);
                    
                    // Update ML predictor if available
                    if let Some(predictor) = &mut self.predictor {
                        predictor.record_outcome(&context, &strategy_name, &feedback);
                    }
                    
                    // Accumulate metrics
                    total_metrics = Self::combine_metrics(total_metrics, metrics);
                    applied_strategies.push(strategy_name.clone());
                    
                    // Log success
                    self.telemetry.record_optimization_success(&strategy_name, &feedback);
                }
                Err(e) => {
                    self.telemetry.record_optimization_failure(&strategy_name, &e);
                }
            }
        }
        
        Ok(OptimizedWorkload {
            workload,
            metrics: total_metrics,
            applied_strategies,
        })
    }
    
    /// Combine metrics from multiple optimizations
    fn combine_metrics(a: Metrics, b: Metrics) -> Metrics {
        Metrics {
            throughput: a.throughput.max(b.throughput),
            latency_p50: a.latency_p50.min(b.latency_p50),
            latency_p99: a.latency_p99.min(b.latency_p99),
            memory_used: a.memory_used.min(b.memory_used),
            cache_hit_rate: (a.cache_hit_rate + b.cache_hit_rate) / 2.0,
            power_estimate: (a.power_estimate + b.power_estimate) / 2.0,
        }
    }
}

/// Result of optimization
#[derive(Debug)]
pub struct OptimizedWorkload {
    pub workload: Workload,
    pub metrics: Metrics,
    pub applied_strategies: Vec<String>,
}

impl Feedback {
    /// Create feedback from before/after snapshots
    fn from_snapshots(
        before: telemetry::Snapshot,
        after: telemetry::Snapshot,
        metrics: &Metrics,
    ) -> Self {
        let metrics_delta = MetricsDelta {
            throughput_change: (after.throughput - before.throughput) / before.throughput,
            latency_change: (after.latency - before.latency).as_secs_f64() / before.latency.as_secs_f64(),
            memory_change: (after.memory_used as f64 - before.memory_used as f64) / before.memory_used as f64,
            power_change: (after.power - before.power) / before.power,
        };
        
        let success = metrics_delta.throughput_change > 0.0 || metrics_delta.latency_change < 0.0;
        
        let mut unexpected_behavior = Vec::new();
        
        // Detect unexpected behavior
        if metrics_delta.memory_change > 0.5 {
            unexpected_behavior.push("Memory usage increased by >50%".to_string());
        }
        if metrics_delta.power_change > 0.3 {
            unexpected_behavior.push("Power consumption increased by >30%".to_string());
        }
        if metrics.cache_hit_rate < 0.5 {
            unexpected_behavior.push("Cache hit rate below 50%".to_string());
        }
        
        Self {
            metrics_delta,
            success,
            unexpected_behavior,
        }
    }
}

/// Multi-dimensional performance point
#[derive(Debug, Clone)]
pub struct PerformancePoint {
    pub throughput: f64,
    pub latency_p99: Duration,
    pub memory_efficiency: f64,
    pub cache_efficiency: f64,
    pub power_efficiency: f64,
    pub thermal_headroom: f64,
}

impl PerformancePoint {
    /// Calculate holistic score with user-defined weights
    pub fn holistic_score(&self, weights: &Weights) -> f64 {
        let normalized_throughput = (self.throughput / 1_000_000_000.0).min(1.0);
        let normalized_latency = 1.0 - (self.latency_p99.as_secs_f64() / 0.001).min(1.0);
        
        weights.throughput * normalized_throughput
            + weights.latency * normalized_latency
            + weights.memory * self.memory_efficiency
            + weights.cache * self.cache_efficiency
            + weights.power * self.power_efficiency
            + weights.thermal * self.thermal_headroom
    }
}

/// User-defined optimization weights
#[derive(Debug, Clone)]
pub struct Weights {
    pub throughput: f64,
    pub latency: f64,
    pub memory: f64,
    pub cache: f64,
    pub power: f64,
    pub thermal: f64,
}

impl Default for Weights {
    fn default() -> Self {
        Self {
            throughput: 0.3,
            latency: 0.2,
            memory: 0.15,
            cache: 0.15,
            power: 0.1,
            thermal: 0.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_point_scoring() {
        let point = PerformancePoint {
            throughput: 500_000_000.0,
            latency_p99: Duration::from_micros(500),
            memory_efficiency: 0.8,
            cache_efficiency: 0.9,
            power_efficiency: 0.7,
            thermal_headroom: 0.6,
        };
        
        let weights = Weights::default();
        let score = point.holistic_score(&weights);
        
        assert!(score > 0.0 && score <= 1.0);
    }
}