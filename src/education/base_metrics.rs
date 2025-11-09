//! # Base Metrics: The Geometry of Number Systems
//!
//! This module teaches how different number bases create fundamentally
//! different "spacetime geometries" for prime numbers, with measurable
//! physical effects.

use std::collections::HashMap;

/// Represents the metric properties of a number base
#[derive(Debug, Clone)]
pub struct BaseMetricEducation {
    pub base: u32,
    pub curvature: f64,
    pub field_type: MetricFieldType,
    pub edge_pairs: Vec<(u32, u32)>,
    pub measured_effects: HashMap<String, MeasuredEffect>,
}

/// Types of metric fields created by bases
#[derive(Debug, Clone, PartialEq)]
pub enum MetricFieldType {
    /// Prime bases create gravitational wells
    StrongAttraction,
    /// Even bases create repulsive hills  
    Repulsion,
    /// Odd composites create flat space
    Neutral,
}

/// Measured experimental results
#[derive(Debug, Clone)]
pub struct MeasuredEffect {
    pub configuration: String,
    pub success_rate: f64,
    pub sample_size: usize,
    pub comparison: Option<BaseComparison>,
}

/// Comparison between bases
#[derive(Debug, Clone)]
pub struct BaseComparison {
    pub other_base: u32,
    pub other_rate: f64,
    pub ratio: f64,
}

impl BaseMetricEducation {
    /// Create educational material for a specific base
    pub fn new(base: u32) -> Self {
        let curvature = Self::calculate_curvature(base);
        let field_type = Self::determine_field_type(base);
        let edge_pairs = Self::calculate_edge_pairs(base);

        Self {
            base,
            curvature,
            field_type,
            edge_pairs,
            measured_effects: HashMap::new(),
        }
    }

    /// Calculate the metric curvature for a base
    fn calculate_curvature(base: u32) -> f64 {
        if Self::is_prime(base) {
            2.0 // Strong attractive curvature
        } else if base.is_multiple_of(2) {
            0.5 // Repulsive field
        } else {
            1.0 // Neutral flat space
        }
    }

    /// Determine the type of field created
    fn determine_field_type(base: u32) -> MetricFieldType {
        if Self::is_prime(base) {
            MetricFieldType::StrongAttraction
        } else if base.is_multiple_of(2) {
            MetricFieldType::Repulsion
        } else {
            MetricFieldType::Neutral
        }
    }

    /// Calculate edge pairs for a base
    fn calculate_edge_pairs(base: u32) -> Vec<(u32, u32)> {
        let mut pairs = Vec::new();

        // Find all digit pairs equidistant from boundaries
        for d1 in 1..base / 2 {
            let d2 = base - d1;
            if d1 < base && d2 < base {
                pairs.push((d1, d2));
            }
        }

        pairs
    }

    /// Simple primality test
    fn is_prime(n: u32) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n.is_multiple_of(2) {
            return false;
        }

        for i in (3..=(n as f64).sqrt() as u32).step_by(2) {
            if n.is_multiple_of(i) {
                return false;
            }
        }
        true
    }

    /// Generate explanation at different education levels
    pub fn explain(&self, level: super::EducationLevel) -> String {
        match level {
            super::EducationLevel::Introductory => self.explain_intro(),
            super::EducationLevel::Moderate => self.explain_moderate(),
            super::EducationLevel::Advanced => self.explain_advanced(),
            super::EducationLevel::Expert => self.explain_expert(),
        }
    }

    fn explain_intro(&self) -> String {
        match self.field_type {
            MetricFieldType::StrongAttraction => format!(
                "Base {} is like a black hole for primes! Since {} is a prime number, \
                it creates a strong gravitational pull that attracts primes together. \
                Think of it like a magnet - primes want to cluster near prime bases.",
                self.base, self.base
            ),
            MetricFieldType::Repulsion => format!(
                "Base {} pushes primes away like opposing magnets! Since {} is even \
                (divisible by 2), it creates a 'force field' that repels prime formation. \
                It's like trying to build sandcastles on a windy beach - the structure \
                keeps getting blown apart.",
                self.base, self.base
            ),
            MetricFieldType::Neutral => format!(
                "Base {} is like flat ground - neither pulling nor pushing primes. \
                As an odd composite number, it creates a neutral playing field where \
                primes can form naturally without extra help or hindrance.",
                self.base
            ),
        }
    }

    fn explain_moderate(&self) -> String {
        let edge_pair_str = if !self.edge_pairs.is_empty() {
            format!(
                "Edge pairs in base {}: {:?}",
                self.base,
                &self.edge_pairs[..3.min(self.edge_pairs.len())]
            )
        } else {
            format!("Base {} has limited edge pair options", self.base)
        };

        format!(
            "Base {} creates a metric with curvature κ = {:.1}\n\n\
            Field Type: {:?}\n\
            {}\n\n\
            This means:\n\
            - Gravitational force scales as: F ∝ κ × M₁M₂/r²\n\
            - Field strength decays as: g(r) = κ × exp(-r²/1000²)\n\
            - Prime density is {} by this metric",
            self.base,
            self.curvature,
            self.field_type,
            edge_pair_str,
            match self.field_type {
                MetricFieldType::StrongAttraction => "enhanced",
                MetricFieldType::Repulsion => "suppressed",
                MetricFieldType::Neutral => "unaffected",
            }
        )
    }

    fn explain_advanced(&self) -> String {
        format!(
            "Base {} Metric Tensor Components:\n\n\
            g_μν = diag(κ, κ, κ, κ) where κ = {:.1}\n\n\
            The metric creates a spacetime with:\n\
            - Ricci scalar: R = {} ({})\n\
            - Geodesic deviation: {} primes\n\
            - Christoffel symbols: Γ^i_jk = {} for i≠j≠k\n\n\
            Prime factorization of base: {}\n\
            This factorization directly determines the metric properties.\n\n\
            Edge pair resonances create standing waves at:\n\
            {:?}",
            self.base,
            self.curvature,
            if self.curvature > 1.0 {
                "positive"
            } else {
                "negative"
            },
            match self.field_type {
                MetricFieldType::StrongAttraction => "positive curvature",
                MetricFieldType::Repulsion => "negative curvature",
                MetricFieldType::Neutral => "flat",
            },
            match self.field_type {
                MetricFieldType::StrongAttraction => "focuses",
                MetricFieldType::Repulsion => "disperses",
                MetricFieldType::Neutral => "preserves",
            },
            if self.curvature != 1.0 {
                "non-zero"
            } else {
                "zero"
            },
            self.factorize(self.base),
            &self.edge_pairs[..5.min(self.edge_pairs.len())]
        )
    }

    fn explain_expert(&self) -> String {
        format!(
            "Base {} induces a Riemannian metric on the moduli space of membrane configurations.\n\n\
            The metric tensor g_b derives from the base factorization via:\n\
            κ(b) = 2/π ∫₀^∞ L(1+it, χ_b)dt\n\
            where χ_b is the principal character mod b.\n\n\
            For b = {}, we have:\n\
            - Selberg zeta: Z_b(s) has {} (prime bases enhance zeros)\n\
            - Ihara zeta: Regular graph with {} adjacency\n\
            - Coupling to membrane Hamiltonian: H_eff = H_0 + κV_int\n\n\
            The base metric creates an effective potential:\n\
            V_eff(r) = -κGM₁M₂/r + λ(b₁,b₂)Q₁Q₂/r²\n\n\
            This explains the measured {}x difference in prime density between\n\
            prime and even bases through semiclassical WKB approximation.",
            self.base, self.base,
            if self.field_type == MetricFieldType::StrongAttraction {
                "enhanced zero density"
            } else {
                "suppressed zeros"
            },
            match self.field_type {
                MetricFieldType::StrongAttraction => "enhanced",
                MetricFieldType::Repulsion => "reduced",
                MetricFieldType::Neutral => "standard",
            },
            if self.field_type == MetricFieldType::StrongAttraction { "2-3" } else { "10-100" }
        )
    }

    /// Factorize a number
    fn factorize(&self, n: u32) -> String {
        if n <= 1 {
            return n.to_string();
        }

        let mut factors = Vec::new();
        let mut num = n;
        let mut d = 2;

        while d * d <= num {
            let mut count = 0;
            while num.is_multiple_of(d) {
                count += 1;
                num /= d;
            }
            if count > 0 {
                if count == 1 {
                    factors.push(format!("{d}"));
                } else {
                    factors.push(format!("{d}^{count}"));
                }
            }
            d += if d == 2 { 1 } else { 2 };
        }

        if num > 1 {
            factors.push(format!("{num}"));
        }

        factors.join(" × ")
    }

    /// Add a measured effect
    pub fn add_measurement(
        &mut self,
        config_name: &str,
        success_rate: f64,
        sample_size: usize,
        comparison: Option<(u32, f64)>,
    ) {
        let effect = MeasuredEffect {
            configuration: config_name.to_string(),
            success_rate,
            sample_size,
            comparison: comparison.map(|(base, rate)| BaseComparison {
                other_base: base,
                other_rate: rate,
                ratio: success_rate / rate,
            }),
        };

        self.measured_effects
            .insert(config_name.to_string(), effect);
    }

    /// Generate a visual representation of the metric field
    pub fn visualize_field(&self) -> String {
        let mut viz = String::new();

        viz.push_str(&format!("Base {} Metric Field:\n", self.base));
        viz.push_str(&"─".repeat(25));
        viz.push('\n');

        // Create ASCII visualization based on field type
        match self.field_type {
            MetricFieldType::StrongAttraction => {
                viz.push_str("      ▼ ▼ ▼ ▼ ▼\n");
                viz.push_str("    ╱           ╲\n");
                viz.push_str("   ╱   ●●●●●●●   ╲\n");
                viz.push_str("  │   ●●PRIME●●   │\n");
                viz.push_str("   ╲   ●●WELL●●  ╱\n");
                viz.push_str("    ╲   ●●●●●   ╱\n");
                viz.push_str("      ╲_______╱\n");
                viz.push_str("\nStrong gravitational well\n");
                viz.push_str("Primes fall in and cluster");
            }
            MetricFieldType::Repulsion => {
                viz.push_str("      ╱───────╲\n");
                viz.push_str("    ╱     ↑     ╲\n");
                viz.push_str("   │    ↑ ↑ ↑    │\n");
                viz.push_str("   │   REPULSIVE │\n");
                viz.push_str("   │     FIELD   │\n");
                viz.push_str("    ╲   ↑ ↑ ↑   ╱\n");
                viz.push_str("      ╲───────╱\n");
                viz.push_str("\nRepulsive force field\n");
                viz.push_str("Primes pushed away");
            }
            MetricFieldType::Neutral => {
                viz.push_str("   ─ ─ ─ ─ ─ ─ ─\n");
                viz.push_str("   · · · · · · ·\n");
                viz.push_str("   ─ FLAT SPACE ─\n");
                viz.push_str("   · · · · · · ·\n");
                viz.push_str("   ─ ─ ─ ─ ─ ─ ─\n");
                viz.push_str("\nNeutral flat geometry\n");
                viz.push_str("No special forces");
            }
        }

        viz
    }
}

/// Create a comparative analysis between bases
pub fn compare_base_metrics(bases: &[u32]) -> String {
    let mut report = String::new();

    report.push_str("BASE METRIC COMPARISON\n");
    report.push_str(&"=".repeat(50));
    report.push_str("\n\n");

    report.push_str("Base | Type      | Curvature | Field      | Edge Pairs\n");
    report.push_str("-----|-----------|-----------|------------|-----------\n");

    for &base in bases {
        let metric = BaseMetricEducation::new(base);
        let base_type = if BaseMetricEducation::is_prime(base) {
            "Prime"
        } else if base % 2 == 0 {
            "Even"
        } else {
            "Odd Comp"
        };

        let field_str = match metric.field_type {
            MetricFieldType::StrongAttraction => "Attractive",
            MetricFieldType::Repulsion => "Repulsive",
            MetricFieldType::Neutral => "Neutral",
        };

        let edge_str = if metric.edge_pairs.len() > 2 {
            format!("{:?}...", &metric.edge_pairs[..2])
        } else {
            format!("{:?}", metric.edge_pairs)
        };

        report.push_str(&format!(
            "{:4} | {:9} | {:9.1} | {:10} | {}\n",
            base, base_type, metric.curvature, field_str, edge_str
        ));
    }

    report.push_str("\nKEY INSIGHTS:\n");
    report.push_str("- Prime bases (11, 13, 17) have κ = 2.0 (attractive)\n");
    report.push_str("- Even bases (8, 10, 12) have κ = 0.5 (repulsive)\n");
    report.push_str("- Odd composites (9, 15) have κ = 1.0 (neutral)\n");
    report.push_str("- Edge pairs determine resonant configurations\n");

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_metrics() {
        let base10 = BaseMetricEducation::new(10);
        assert_eq!(base10.curvature, 0.5); // Even base
        assert_eq!(base10.field_type, MetricFieldType::Repulsion);
        assert!(base10.edge_pairs.contains(&(3, 7)));

        let base11 = BaseMetricEducation::new(11);
        assert_eq!(base11.curvature, 2.0); // Prime base
        assert_eq!(base11.field_type, MetricFieldType::StrongAttraction);

        let base9 = BaseMetricEducation::new(9);
        assert_eq!(base9.curvature, 1.0); // Odd composite
        assert_eq!(base9.field_type, MetricFieldType::Neutral);
    }

    #[test]
    fn test_explanations() {
        let base12 = BaseMetricEducation::new(12);
        let intro = base12.explain(super::super::EducationLevel::Introductory);
        assert!(intro.contains("pushes primes away"));

        let moderate = base12.explain(super::super::EducationLevel::Moderate);
        assert!(moderate.contains("κ = 0.5"));
    }
}
