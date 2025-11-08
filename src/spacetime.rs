//! Spacetime and base-dependent metrics


/// Base-dependent spacetime metric
#[derive(Debug, Clone)]
pub struct BaseMetric {
    pub base: u32,
    pub curvature: f64,
}

impl BaseMetric {
    pub fn new(base: u32) -> Self {
        let curvature = match base {
            p if is_prime(p) => 2.0,
            p if p % 2 == 0 => 0.5,
            _ => 1.0,
        };
        
        Self { base, curvature }
    }
}

/// Potential field in mathematical spacetime
#[derive(Debug, Clone)]
pub struct PotentialField {
    pub resolution: f64,
}

impl PotentialField {
    pub fn new(resolution: f64) -> Self {
        Self { resolution }
    }
}

/// Phase space for trajectory analysis
#[derive(Debug, Clone)]
pub struct PhaseSpace {
    pub dimensions: usize,
}

impl PhaseSpace {
    pub fn new(dimensions: usize) -> Self {
        Self { dimensions }
    }
}

fn is_prime(n: u32) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    
    for i in (3..=(n as f64).sqrt() as u32).step_by(2) {
        if n % i == 0 { return false; }
    }
    true
}