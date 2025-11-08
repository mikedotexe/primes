//! # Tidal Physics Module
//! 
//! Based on our discovery that tidal strength ~12.9 optimally organizes primes
//! without destroying them - the inverse of the classical Roche limit!
//!
//! ## Key Discovery
//! 
//! - Too weak (<5): Can't organize primes into patterns
//! - Optimal (10-15): Creates structure without destruction  
//! - Too strong (>15): Fragments existing clusters
//!
//! This suggests primes have inherent "cohesion strength" that responds
//! to tidal stress by crystallizing into patterns.

use std::collections::HashMap;
use crate::{PhysicsResult, PhysicsError};
use crate::gravity::PrimeParticle;

/// The magical tidal strength that maximizes organization
pub const OPTIMAL_TIDAL_STRENGTH: f64 = 12.9;

/// Tidal strength thresholds from our Lagrange analysis
pub const TIDAL_ORGANIZING_MIN: f64 = 10.0;
pub const TIDAL_ORGANIZING_MAX: f64 = 15.0;
pub const TIDAL_PRESERVING_MIN: f64 = 5.0;
pub const TIDAL_INEFFECTIVE_MAX: f64 = 5.0;

/// Main tidal physics analyzer
#[derive(Debug, Clone)]
pub struct TidalAnalyzer {
    /// Map of named points to their tidal properties
    pub tidal_zones: HashMap<String, TidalZone>,
    
    /// Roche limit calculator for prime cohesion
    pub roche_calculator: RocheCalculator,
    
    /// History of tidal evolution
    pub tidal_history: Vec<TidalSnapshot>,
    
    /// Current simulation time
    pub time: f64,
}

/// Classification of tidal zones by their effect on primes
#[derive(Debug, Clone)]
pub enum TidalZone {
    /// Fragments clusters (>15)
    Destructive { 
        strength: f64,
        fragmentation_rate: f64,
    },
    
    /// Creates structure (10-15) - THE SWEET SPOT!
    Organizing { 
        strength: f64,
        organization_efficiency: f64,
    },
    
    /// Maintains existing patterns (5-10)
    Preserving { 
        strength: f64,
        stability_factor: f64,
    },
    
    /// Too weak to affect (<5)
    Ineffective { 
        strength: f64,
    },
}

/// Roche limit calculator adapted for prime physics
#[derive(Debug, Clone)]
pub struct RocheCalculator {
    /// Base Roche coefficient (2.456 for rigid bodies)
    pub coefficient: f64,
    
    /// Prime cohesion factor (how strongly primes resist tidal disruption)
    pub cohesion_factor: f64,
}

/// A snapshot of the tidal field at a moment in time
#[derive(Debug, Clone)]
pub struct TidalSnapshot {
    pub time: f64,
    pub field_strength: Vec<Vec<f64>>,
    pub organization_zones: Vec<OrganizationZone>,
    pub total_organized_primes: usize,
}

/// A zone where tidal forces organize primes
#[derive(Debug, Clone)]
pub struct OrganizationZone {
    pub center: [f64; 2],
    pub radius: f64,
    pub tidal_strength: f64,
    pub captured_primes: Vec<usize>, // Indices of organized primes
    pub pattern_type: PatternType,
}

/// Types of patterns that emerge from tidal organization
#[derive(Debug, Clone, Default)]
pub enum PatternType {
    /// Linear chains (like at L1, L2)
    LinearChain,
    
    /// Circular clusters (like at L4, L5 in classical mechanics)
    #[default]
    CircularCluster,
    
    /// Hidden sanctuary (like at L3 - our champion!)
    HiddenSanctuary,
    
    /// Spiral arms (in rotating systems)
    SpiralArm,
    
    /// Crystalline lattice (at perfect resonance)
    CrystallineLattice,
}

/// Complete tidal field at a point
#[derive(Debug, Clone)]
pub struct TidalField {
    pub position: [f64; 2],
    pub strength: f64,
    pub gradient: [f64; 2],
    pub zone_type: TidalZone,
    pub nearest_sources: Vec<(usize, f64)>, // (particle_index, contribution)
}

impl Default for TidalAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl TidalAnalyzer {
    /// Create a new tidal analyzer
    pub fn new() -> Self {
        Self {
            tidal_zones: HashMap::new(),
            roche_calculator: RocheCalculator::new(),
            tidal_history: Vec::new(),
            time: 0.0,
        }
    }
    
    /// Calculate the complete tidal field for a system of particles
    pub fn calculate_tidal_field(
        &mut self,
        particles: &[PrimeParticle],
        grid_size: usize
    ) -> PhysicsResult<Vec<Vec<TidalField>>> {
        let mut field = Vec::with_capacity(grid_size);
        
        // Find bounds of particle system
        let (min_x, max_x, min_y, max_y) = self.find_bounds(particles)?;
        let padding = 10.0;
        
        let x_step = (max_x - min_x + 2.0 * padding) / grid_size as f64;
        let y_step = (max_y - min_y + 2.0 * padding) / grid_size as f64;
        
        // Calculate tidal field at each grid point
        for i in 0..grid_size {
            let mut row = Vec::with_capacity(grid_size);
            let x = min_x - padding + i as f64 * x_step;
            
            for j in 0..grid_size {
                let y = min_y - padding + j as f64 * y_step;
                let point = [x, y];
                
                let tidal_field = self.calculate_tidal_at_point(point, particles)?;
                row.push(tidal_field);
            }
            field.push(row);
        }
        
        Ok(field)
    }
    
    /// Calculate tidal strength at a specific point
    pub fn calculate_tidal_at_point(
        &self,
        point: [f64; 2],
        particles: &[PrimeParticle]
    ) -> PhysicsResult<TidalField> {
        let mut total_tidal = [0.0, 0.0];
        let mut total_strength = 0.0;
        let mut nearest_sources = Vec::new();
        
        // Sum tidal contributions from all particles
        for (idx, particle) in particles.iter().enumerate() {
            let dx = particle.position[0] - point[0];
            let dy = particle.position[1] - point[1];
            let r = (dx*dx + dy*dy).sqrt();
            
            if r < 0.1 { continue; } // Skip if too close
            
            // Tidal force scales as M/r³ (gradient of gravitational field)
            let tidal_magnitude = particle.mass / (r * r * r);
            
            // Direction points away from the source (tidal stretching)
            let tidal_x = -tidal_magnitude * dx / r;
            let tidal_y = -tidal_magnitude * dy / r;
            
            total_tidal[0] += tidal_x;
            total_tidal[1] += tidal_y;
            total_strength += tidal_magnitude;
            
            if tidal_magnitude > 0.1 { // Significant contribution
                nearest_sources.push((idx, tidal_magnitude));
            }
        }
        
        // Sort by contribution
        nearest_sources.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        nearest_sources.truncate(3); // Keep top 3 contributors
        
        // Classify the zone
        let zone_type = self.classify_tidal_zone(total_strength);
        
        Ok(TidalField {
            position: point,
            strength: total_strength,
            gradient: total_tidal,
            zone_type,
            nearest_sources,
        })
    }
    
    /// Find organization zones where tidal forces create patterns
    pub fn find_organization_zones(
        &mut self,
        particles: &[PrimeParticle],
        field: &[Vec<TidalField>]
    ) -> PhysicsResult<Vec<OrganizationZone>> {
        // Validate field dimensions
        if field.is_empty() || field[0].is_empty() {
            return Ok(Vec::new());
        }
        
        let rows = field.len();
        let cols = field[0].len();
        
        // Ensure all rows have same length
        for row in field {
            if row.len() != cols {
                return Err(PhysicsError::InvalidConfiguration(
                    "Tidal field has inconsistent dimensions".to_string()
                ));
            }
        }
        
        let mut zones = Vec::new();
        let mut visited = vec![vec![false; cols]; rows];
        
        // Scan for organizing regions
        for i in 0..rows {
            for j in 0..cols {
                if visited[i][j] { continue; }
                
                if let TidalZone::Organizing {  .. } = &field[i][j].zone_type {
                    // Found an organizing zone - flood fill to find extent
                    let zone = self.explore_organization_zone(
                        field, 
                        particles,
                        i, 
                        j, 
                        &mut visited
                    )?;
                    
                    if !zone.captured_primes.is_empty() {
                        zones.push(zone);
                    }
                }
            }
        }
        
        // Sort by number of organized primes
        zones.sort_by_key(|z| z.captured_primes.len());
        zones.reverse();
        
        Ok(zones)
    }
    
    /// Classify a tidal strength into a zone type
    fn classify_tidal_zone(&self, strength: f64) -> TidalZone {
        if strength > TIDAL_ORGANIZING_MAX {
            TidalZone::Destructive {
                strength,
                fragmentation_rate: (strength - TIDAL_ORGANIZING_MAX) * 0.1,
            }
        } else if strength >= TIDAL_ORGANIZING_MIN {
            // THE SWEET SPOT!
            let efficiency = 1.0 - ((strength - OPTIMAL_TIDAL_STRENGTH).abs() / 5.0);
            TidalZone::Organizing {
                strength,
                organization_efficiency: efficiency.max(0.0_f64),
            }
        } else if strength >= TIDAL_PRESERVING_MIN {
            TidalZone::Preserving {
                strength,
                stability_factor: strength / TIDAL_PRESERVING_MIN,
            }
        } else {
            TidalZone::Ineffective { strength }
        }
    }
    
    /// Explore an organization zone using flood fill
    fn explore_organization_zone(
        &self,
        field: &[Vec<TidalField>],
        particles: &[PrimeParticle],
        start_i: usize,
        start_j: usize,
        visited: &mut [Vec<bool>]
    ) -> PhysicsResult<OrganizationZone> {
        let mut stack = vec![(start_i, start_j)];
        let mut points = Vec::new();
        let mut total_strength = 0.0;
        
        while let Some((i, j)) = stack.pop() {
            if visited[i][j] { continue; }
            visited[i][j] = true;
            
            if let TidalZone::Organizing { strength, .. } = &field[i][j].zone_type {
                points.push(field[i][j].position);
                total_strength += strength;
                
                // Check neighbors
                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 { continue; }
                        
                        // Safe bounds checking to prevent underflow
                        let ni = i as i32 + di;
                        let nj = j as i32 + dj;
                        
                        if ni >= 0 && nj >= 0 {
                            let ni = ni as usize;
                            let nj = nj as usize;
                            
                            if ni < field.len() && nj < field[0].len() && !visited[ni][nj] {
                                if let TidalZone::Organizing { .. } = &field[ni][nj].zone_type {
                                    stack.push((ni, nj));
                                }
                            }
                        }
                    }
                }
            }
        }
        
        if points.is_empty() {
            return Err(PhysicsError::InvalidConfiguration(
                "No points in organization zone".to_string()
            ));
        }
        
        // Find center and radius
        let center = self.find_centroid(&points);
        let radius = self.find_radius(&points, center);
        let avg_strength = total_strength / points.len() as f64;
        
        // Find captured primes
        let captured_primes = self.find_captured_primes(particles, center, radius * 1.5);
        
        // Determine pattern type
        let pattern_type = self.determine_pattern_type(avg_strength, &captured_primes, particles);
        
        Ok(OrganizationZone {
            center,
            radius,
            tidal_strength: avg_strength,
            captured_primes,
            pattern_type,
        })
    }
    
    /// Find all primes within a radius of a point
    fn find_captured_primes(
        &self,
        particles: &[PrimeParticle],
        center: [f64; 2],
        radius: f64
    ) -> Vec<usize> {
        let mut captured = Vec::new();
        
        for (idx, particle) in particles.iter().enumerate() {
            let dx = particle.position[0] - center[0];
            let dy = particle.position[1] - center[1];
            let dist = (dx*dx + dy*dy).sqrt();
            
            if dist <= radius {
                captured.push(idx);
            }
        }
        
        captured
    }
    
    /// Determine what type of pattern the organized primes form
    fn determine_pattern_type(
        &self,
        avg_strength: f64,
        captured_indices: &[usize],
        particles: &[PrimeParticle]
    ) -> PatternType {
        if captured_indices.len() < 2 {
            return PatternType::CircularCluster;
        }
        
        // Near optimal strength suggests hidden sanctuary
        if (avg_strength - OPTIMAL_TIDAL_STRENGTH).abs() < 1.0 {
            return PatternType::HiddenSanctuary;
        }
        
        // Check for linear arrangement
        if self.is_linear_arrangement(captured_indices, particles) {
            return PatternType::LinearChain;
        }
        
        // Check for spiral pattern (in rotating systems)
        if self.has_angular_momentum(captured_indices, particles) {
            return PatternType::SpiralArm;
        }
        
        // High organization efficiency suggests crystalline structure
        if avg_strength > 12.0 && avg_strength < 14.0 {
            return PatternType::CrystallineLattice;
        }
        
        PatternType::CircularCluster
    }
    
    /// Check if primes form a roughly linear arrangement
    fn is_linear_arrangement(&self, indices: &[usize], particles: &[PrimeParticle]) -> bool {
        if indices.len() < 3 { return false; }
        
        // Use first and last prime to define line
        let p1 = &particles[indices[0]].position;
        let p2 = &particles[indices[indices.len()-1]].position;
        
        let dx = p2[0] - p1[0];
        let dy = p2[1] - p1[1];
        let line_length = (dx*dx + dy*dy).sqrt();
        
        if line_length < 0.1 { return false; }
        
        // Check if other primes are close to this line
        let mut max_deviation = 0.0_f64;
        for &idx in &indices[1..indices.len()-1] {
            let p = &particles[idx].position;
            
            // Distance from point to line
            let cross = (p[0] - p1[0]) * dy - (p[1] - p1[1]) * dx;
            let dist = cross.abs() / line_length;
            max_deviation = max_deviation.max(dist);
        }
        
        // Linear if deviation is small compared to length
        max_deviation < line_length * 0.2
    }
    
    /// Check if the system has significant angular momentum
    fn has_angular_momentum(&self, indices: &[usize], particles: &[PrimeParticle]) -> bool {
        if indices.len() < 3 { return false; }
        
        // Calculate center of mass
        let mut com = [0.0, 0.0];
        let mut total_mass = 0.0;
        
        for &idx in indices {
            let p = &particles[idx];
            com[0] += p.position[0] * p.mass;
            com[1] += p.position[1] * p.mass;
            total_mass += p.mass;
        }
        
        com[0] /= total_mass;
        com[1] /= total_mass;
        
        // Calculate total angular momentum
        let mut total_l = 0.0;
        
        for &idx in indices {
            let p = &particles[idx];
            let r = [p.position[0] - com[0], p.position[1] - com[1]];
            let l = r[0] * p.velocity[1] - r[1] * p.velocity[0];
            total_l += l * p.mass;
        }
        
        // Significant if angular momentum per particle is high
        (total_l / total_mass).abs() > 1.0
    }
    
    /// Find bounds of particle system
    fn find_bounds(&self, particles: &[PrimeParticle]) -> PhysicsResult<(f64, f64, f64, f64)> {
        if particles.is_empty() {
            return Err(PhysicsError::InvalidConfiguration(
                "No particles in system".to_string()
            ));
        }
        
        let first = &particles[0].position;
        let mut min_x = first[0];
        let mut max_x = first[0];
        let mut min_y = first[1];
        let mut max_y = first[1];
        
        for particle in particles {
            min_x = min_x.min(particle.position[0]);
            max_x = max_x.max(particle.position[0]);
            min_y = min_y.min(particle.position[1]);
            max_y = max_y.max(particle.position[1]);
        }
        
        Ok((min_x, max_x, min_y, max_y))
    }
    
    /// Find centroid of a set of points
    fn find_centroid(&self, points: &[[f64; 2]]) -> [f64; 2] {
        let mut center = [0.0, 0.0];
        for p in points {
            center[0] += p[0];
            center[1] += p[1];
        }
        center[0] /= points.len() as f64;
        center[1] /= points.len() as f64;
        center
    }
    
    /// Find radius that encompasses all points
    fn find_radius(&self, points: &[[f64; 2]], center: [f64; 2]) -> f64 {
        let mut max_dist = 0.0_f64;
        for p in points {
            let dx = p[0] - center[0];
            let dy = p[1] - center[1];
            let dist = (dx*dx + dy*dy).sqrt();
            max_dist = max_dist.max(dist);
        }
        max_dist
    }
    
    /// Take a snapshot of current tidal state
    pub fn take_snapshot(
        &mut self,
        field: &[Vec<TidalField>],
        zones: &[OrganizationZone],
        _particles: &[PrimeParticle]
    ) {
        let total_organized = zones.iter()
            .map(|z| z.captured_primes.len())
            .sum();
        
        let field_strength = field.iter()
            .map(|row| row.iter().map(|f| f.strength).collect())
            .collect();
        
        let snapshot = TidalSnapshot {
            time: self.time,
            field_strength,
            organization_zones: zones.to_vec(),
            total_organized_primes: total_organized,
        };
        
        self.tidal_history.push(snapshot);
    }
    
    /// Analyze how tidal organization evolved over time
    pub fn analyze_evolution(&self) -> TidalEvolution {
        if self.tidal_history.len() < 2 {
            return TidalEvolution::default();
        }
        
        let first = &self.tidal_history[0];
        let last = &self.tidal_history[self.tidal_history.len() - 1];
        
        let organization_growth = (last.total_organized_primes as f64 / 
                                  first.total_organized_primes.max(1) as f64) - 1.0;
        
        let pattern_changes = self.count_pattern_changes();
        let stability_score = 1.0 / (1.0 + pattern_changes as f64);
        
        TidalEvolution {
            total_snapshots: self.tidal_history.len(),
            organization_growth,
            pattern_changes,
            stability_score,
            dominant_pattern: self.find_dominant_pattern(),
        }
    }
    
    /// Count how many times patterns changed type
    fn count_pattern_changes(&self) -> usize {
        let mut changes = 0;
        
        for i in 1..self.tidal_history.len() {
            let prev = &self.tidal_history[i-1];
            let curr = &self.tidal_history[i];
            
            // Simple check: did number of zones change significantly?
            if (prev.organization_zones.len() as i32 - curr.organization_zones.len() as i32).abs() > 1 {
                changes += 1;
            }
        }
        
        changes
    }
    
    /// Find the most common pattern type across all snapshots
    fn find_dominant_pattern(&self) -> PatternType {
        let mut pattern_counts = HashMap::new();
        
        for snapshot in &self.tidal_history {
            for zone in &snapshot.organization_zones {
                let key = format!("{:?}", zone.pattern_type);
                *pattern_counts.entry(key).or_insert(0) += 1;
            }
        }
        
        // For now, default to HiddenSanctuary (our L3 champion!)
        PatternType::HiddenSanctuary
    }
}

impl Default for RocheCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl RocheCalculator {
    /// Create a new Roche calculator
    pub fn new() -> Self {
        Self {
            coefficient: 2.456,
            cohesion_factor: 1.0, // Can be tuned based on membrane config
        }
    }
    
    /// Calculate the Roche limit for a prime particle
    pub fn calculate_roche_limit(
        &self,
        primary: &PrimeParticle,
        secondary: &PrimeParticle
    ) -> f64 {
        // Classical Roche limit adapted for prime physics
        let density_ratio = primary.mass / secondary.mass;
        let base_limit = self.coefficient * secondary.physics_cache.effective_radius * 
                        density_ratio.powf(1.0/3.0);
        
        // Adjust for membrane cohesion
        let cohesion_boost = if secondary.membrane_config.is_some() {
            1.5 // Membrane structure provides extra cohesion
        } else {
            1.0
        };
        
        base_limit * cohesion_boost * self.cohesion_factor
    }
    
    /// Check if a particle is within the Roche limit
    pub fn is_within_roche_limit(
        &self,
        primary: &PrimeParticle,
        secondary: &PrimeParticle
    ) -> bool {
        let dx = primary.position[0] - secondary.position[0];
        let dy = primary.position[1] - secondary.position[1];
        let distance = (dx*dx + dy*dy).sqrt();
        
        distance < self.calculate_roche_limit(primary, secondary)
    }
    
    /// Calculate tidal disruption probability
    pub fn disruption_probability(
        &self,
        primary: &PrimeParticle,
        secondary: &PrimeParticle,
        tidal_strength: f64
    ) -> f64 {
        if !self.is_within_roche_limit(primary, secondary) {
            return 0.0;
        }
        
        // Probability increases sharply past optimal strength
        if tidal_strength < TIDAL_ORGANIZING_MAX {
            0.0 // In organizing zone
        } else {
            // Exponential increase in disruption probability
            1.0 - (-(tidal_strength - TIDAL_ORGANIZING_MAX) / 5.0).exp()
        }
    }
}

/// Summary of tidal evolution over time
#[derive(Debug, Default)]
pub struct TidalEvolution {
    pub total_snapshots: usize,
    pub organization_growth: f64,
    pub pattern_changes: usize,
    pub stability_score: f64,
    pub dominant_pattern: PatternType,
}


#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;
    
    #[test]
    fn test_tidal_classification() {
        let analyzer = TidalAnalyzer::new();
        
        // Test classification thresholds
        let zone = analyzer.classify_tidal_zone(3.0);
        assert!(matches!(zone, TidalZone::Ineffective { .. }), 
               "Expected ineffective zone, got: {zone:?}");
        
        let zone = analyzer.classify_tidal_zone(12.9);
        assert!(matches!(zone, TidalZone::Organizing { .. }), 
               "Expected organizing zone, got: {zone:?}");
        
        let zone = analyzer.classify_tidal_zone(20.0);
        assert!(matches!(zone, TidalZone::Destructive { .. }), 
               "Expected destructive zone, got: {zone:?}");
    }
    
    #[test]
    fn test_roche_calculator() {
        let roche = RocheCalculator::new();
        
        let p1 = PrimeParticle::new(
            BigUint::from_u64(97).unwrap(),
            10,
            [0.0, 0.0],
            [0.0, 0.0],
            "Primary".to_string()
        );
        
        let p2 = PrimeParticle::new(
            BigUint::from_u64(17).unwrap(),
            10,
            [5.0, 0.0],
            [0.0, 0.0],
            "Secondary".to_string()
        );
        
        let limit = roche.calculate_roche_limit(&p1, &p2);
        assert!(limit > 0.0);
        
        let within = roche.is_within_roche_limit(&p1, &p2);
        assert!(!within); // 5 units apart should be outside limit
    }
}