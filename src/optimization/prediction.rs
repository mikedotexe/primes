// src/optimization/prediction.rs
//! Machine learning prediction for optimization effectiveness

use std::collections::HashMap;
use super::{SystemContext, Feedback};

/// Simple ML predictor for optimization outcomes
pub struct OptimizationPredictor {
    /// Historical performance data
    performance_history: HashMap<String, Vec<PerformanceRecord>>,
    /// Learned patterns
    patterns: Vec<Pattern>,
}

#[derive(Debug, Clone)]
struct PerformanceRecord {
    context_features: ContextFeatures,
    strategy: String,
    outcome: Outcome,
}

#[derive(Debug, Clone)]
struct ContextFeatures {
    cpu_freq_normalized: f64,
    memory_pressure: f64,
    thermal_pressure: f64,
    workload_size_log: f64,
    cache_size_ratio: f64,
}

#[derive(Debug, Clone)]
struct Outcome {
    throughput_improvement: f64,
    latency_improvement: f64,
    success: bool,
}

#[derive(Debug, Clone)]
struct Pattern {
    conditions: ContextFeatures,
    strategy: String,
    expected_improvement: f64,
    confidence: f64,
}

impl Default for OptimizationPredictor {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationPredictor {
    pub fn new() -> Self {
        Self {
            performance_history: HashMap::new(),
            patterns: Vec::new(),
        }
    }
    
    /// Record the outcome of applying a strategy
    pub fn record_outcome(&mut self, context: &SystemContext, strategy: &str, feedback: &Feedback) {
        let features = Self::extract_features(context);
        let outcome = Outcome {
            throughput_improvement: feedback.metrics_delta.throughput_change,
            latency_improvement: -feedback.metrics_delta.latency_change, // Negative because lower is better
            success: feedback.success,
        };
        
        let record = PerformanceRecord {
            context_features: features.clone(),
            strategy: strategy.to_string(),
            outcome: outcome.clone(),
        };
        
        self.performance_history
            .entry(strategy.to_string())
            .or_default()
            .push(record);
        
        // Update patterns if we have enough data
        if self.performance_history[strategy].len() >= 10 {
            self.update_patterns(strategy);
        }
    }
    
    /// Predict the effectiveness of a strategy in the given context
    pub fn predict_effectiveness(&self, context: &SystemContext, strategy: &str) -> f64 {
        let features = Self::extract_features(context);
        
        // Find similar patterns
        let similar_patterns: Vec<&Pattern> = self.patterns
            .iter()
            .filter(|p| p.strategy == strategy)
            .filter(|p| Self::similarity(&features, &p.conditions) > 0.7)
            .collect();
        
        if similar_patterns.is_empty() {
            // No patterns - use historical average if available
            if let Some(history) = self.performance_history.get(strategy) {
                let success_rate = history.iter()
                    .filter(|r| r.outcome.success)
                    .count() as f64 / history.len() as f64;
                return success_rate;
            }
            return 0.5; // Default neutral prediction
        }
        
        // Weighted average of similar patterns
        let total_weight: f64 = similar_patterns.iter()
            .map(|p| p.confidence * Self::similarity(&features, &p.conditions))
            .sum();
        
        let weighted_improvement: f64 = similar_patterns.iter()
            .map(|p| p.expected_improvement * p.confidence * Self::similarity(&features, &p.conditions))
            .sum();
        
        weighted_improvement / total_weight
    }
    
    /// Extract ML features from system context
    fn extract_features(context: &SystemContext) -> ContextFeatures {
        ContextFeatures {
            cpu_freq_normalized: context.cpu_freq_ghz / 3.0, // Normalize to ~1.0
            memory_pressure: context.available_memory as f64 / (8.0 * 1024.0 * 1024.0 * 1024.0),
            thermal_pressure: context.thermal_pressure,
            workload_size_log: 0.5, // Would come from workload
            cache_size_ratio: context.cache_sizes.l1d as f64 / (128.0 * 1024.0),
        }
    }
    
    /// Calculate similarity between two feature vectors
    fn similarity(a: &ContextFeatures, b: &ContextFeatures) -> f64 {
        let diffs = [
            (a.cpu_freq_normalized - b.cpu_freq_normalized).abs(),
            (a.memory_pressure - b.memory_pressure).abs(),
            (a.thermal_pressure - b.thermal_pressure).abs(),
            (a.workload_size_log - b.workload_size_log).abs(),
            (a.cache_size_ratio - b.cache_size_ratio).abs(),
        ];
        
        let distance = diffs.iter().map(|d| d * d).sum::<f64>().sqrt();
        1.0 / (1.0 + distance) // Convert distance to similarity
    }
    
    /// Update patterns based on historical data
    fn update_patterns(&mut self, strategy: &str) {
        let history = match self.performance_history.get(strategy) {
            Some(h) => h,
            None => return,
        };
        
        // Simple clustering - find common successful contexts
        // In a real implementation, this would use k-means or DBSCAN
        
        let successful_records: Vec<&PerformanceRecord> = history
            .iter()
            .filter(|r| r.outcome.success && r.outcome.throughput_improvement > 0.1)
            .collect();
        
        if successful_records.len() < 5 {
            return;
        }
        
        // Create a pattern from the centroid of successful records
        let mut centroid = ContextFeatures {
            cpu_freq_normalized: 0.0,
            memory_pressure: 0.0,
            thermal_pressure: 0.0,
            workload_size_log: 0.0,
            cache_size_ratio: 0.0,
        };
        
        for record in &successful_records {
            centroid.cpu_freq_normalized += record.context_features.cpu_freq_normalized;
            centroid.memory_pressure += record.context_features.memory_pressure;
            centroid.thermal_pressure += record.context_features.thermal_pressure;
            centroid.workload_size_log += record.context_features.workload_size_log;
            centroid.cache_size_ratio += record.context_features.cache_size_ratio;
        }
        
        let n = successful_records.len() as f64;
        centroid.cpu_freq_normalized /= n;
        centroid.memory_pressure /= n;
        centroid.thermal_pressure /= n;
        centroid.workload_size_log /= n;
        centroid.cache_size_ratio /= n;
        
        let avg_improvement = successful_records
            .iter()
            .map(|r| r.outcome.throughput_improvement)
            .sum::<f64>() / n;
        
        let pattern = Pattern {
            conditions: centroid,
            strategy: strategy.to_string(),
            expected_improvement: avg_improvement,
            confidence: (successful_records.len() as f64 / history.len() as f64).sqrt(),
        };
        
        // Replace or add pattern
        if let Some(existing) = self.patterns.iter_mut().find(|p| p.strategy == strategy) {
            *existing = pattern;
        } else {
            self.patterns.push(pattern);
        }
    }
    
    /// Get insights about optimization patterns
    pub fn get_insights(&self) -> Vec<String> {
        let mut insights = Vec::new();
        
        for pattern in &self.patterns {
            insights.push(format!(
                "{}: Best at freq={:.1}GHz, thermal={:.0}%, improvement={:.0}% (confidence={:.0}%)",
                pattern.strategy,
                pattern.conditions.cpu_freq_normalized * 3.0,
                pattern.conditions.thermal_pressure * 100.0,
                pattern.expected_improvement * 100.0,
                pattern.confidence * 100.0
            ));
        }
        
        insights
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::optimization::{CpuInfo, Architecture, CacheSizes, MetricsDelta};
    
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
    
    #[test]
    fn test_prediction_learning() {
        let mut predictor = OptimizationPredictor::new();
        let context = test_context();
        
        // Record several successful outcomes
        for i in 0..15 {
            let feedback = Feedback {
                metrics_delta: MetricsDelta {
                    throughput_change: 0.2 + (i as f64 * 0.01),
                    latency_change: -0.1,
                    memory_change: -0.3,
                    power_change: 0.05,
                },
                success: true,
                unexpected_behavior: Vec::new(),
            };
            
            predictor.record_outcome(&context, "TestStrategy", &feedback);
        }
        
        // Predict effectiveness (should be reasonable given the input data)
        let effectiveness = predictor.predict_effectiveness(&context, "TestStrategy");
        assert!(effectiveness >= 0.0 && effectiveness <= 1.0, "Effectiveness should be in valid range [0,1]");
        
        // Get insights
        let insights = predictor.get_insights();
        assert!(!insights.is_empty());
    }
}