//! # Educational Module: Understanding Prime Atoms
//!
//! This module explains the "nuts and bolts" of prime construction through
//! membrane patterns, making it accessible to audiences from moderate to
//! graduate-level physics and mathematics backgrounds.
//!
//! ## Core Concepts
//!
//! 1. **Middle Nucleus**: The central digit(s) that form the core
//! 2. **Membranes**: Boundary layers with specific digit patterns
//! 3. **Zero Padding**: "Empty space" between membranes (like electron shells)
//! 4. **Base Metrics**: How different number bases create different "physics"

pub mod base_metrics;

use crate::membrane::MembraneConfig;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::fmt;

pub use base_metrics::{BaseMetricEducation, MeasuredEffect, MetricFieldType};

/// Education level for explanations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EducationLevel {
    /// High school level - simple analogies
    Introductory,
    /// Undergraduate - more technical but accessible
    Moderate,
    /// Graduate - full mathematical treatment
    Advanced,
    /// Research - cutting edge concepts
    Expert,
}

/// # The Prime Atom: A Complete Description
///
/// Just as physical atoms have:
/// - Nucleus (protons/neutrons)
/// - Electron shells at specific energy levels
/// - Quantum numbers describing states
///
/// Prime atoms have:
/// - Middle nucleus (the seed digit(s))
/// - Membrane shells (boundary digits)
/// - Zero-padding distances (shell radii)
/// - Base-dependent quantum states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimeAtom {
    /// The base number system (like which element on periodic table)
    pub base: u32,

    /// The atomic structure
    pub structure: AtomicStructure,

    /// Physical properties
    pub properties: AtomicProperties,

    /// Discovered prime examples
    pub examples: Vec<DiscoveredPrime>,

    /// Educational metadata
    pub education_level: EducationLevel,
}

/// The structural configuration of a prime atom
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomicStructure {
    /// Nuclear configuration
    pub nucleus: Nucleus,

    /// Membrane shells (like electron shells)
    pub shells: Vec<MembraneShell>,

    /// Total zero count (empty space volume)
    pub total_zeros: u32,

    /// Symmetry type
    pub symmetry: SymmetryType,
}

/// The nucleus at the center of the prime atom
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nucleus {
    /// For single-digit nuclei
    pub seed: Option<u32>,

    /// For multi-digit nuclei
    pub pattern: Option<String>,

    /// Nuclear "spin" (based on digit properties)
    pub spin: NuclearSpin,

    /// Resonance frequency (how often it produces primes)
    pub resonance_frequency: f64,
}

/// A membrane shell surrounding the nucleus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembraneShell {
    /// Shell level (1 = innermost, 2 = next, etc.)
    pub level: u32,

    /// The boundary digit for this shell
    pub digit: u32,

    /// Distance from previous shell (zero count)
    pub orbital_radius: u32,

    /// Shell type (s, p, d, f orbital analogy)
    pub orbital_type: OrbitalDesignation,
}

/// Physical and mathematical properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomicProperties {
    /// Expected prime density for this configuration
    pub prime_density: f64,

    /// Gravitational mass in prime space
    pub mass: f64,

    /// Charge (prime digit coupling)
    pub charge: f64,

    /// Stability score (resistance to perturbation)
    pub stability: f64,

    /// Interaction affinity with other atoms
    pub binding_affinity: f64,
}

/// A concrete example of a discovered prime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredPrime {
    /// The actual prime number
    pub value: BigUint,

    /// When it was discovered
    pub discovery_context: String,

    /// Special properties it exhibits
    pub notable_features: Vec<String>,

    /// Visual representation
    pub structure_diagram: String,
}

/// Types of symmetry in membrane construction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymmetryType {
    /// Perfect mirror symmetry
    Symmetric,

    /// Different left/right padding
    Breathing { inhale_exhale_ratio: f64 },

    /// Multiple nested symmetries
    Fractal { depth: u32 },

    /// Rotating/spiral patterns
    Chiral { handedness: Chirality },
}

/// Handedness for chiral structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Chirality {
    Left,
    Right,
}

/// Nuclear spin states (analogy to quantum spin)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NuclearSpin {
    /// No spin (seed 0, 5)
    Zero,

    /// Half-integer spin (seeds 1, 3, 7, 9)
    HalfInteger { value: f64 },

    /// Integer spin (seeds 2, 4, 6, 8)
    Integer { value: i32 },

    /// Exotic spin (patterns like 37, 73)
    Exotic { pattern: String },
}

/// Orbital type designation (s, p, d, f analogy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrbitalDesignation {
    /// Spherical (k = 0 or 1)
    S,

    /// Dumbbell (k = 2)
    P,

    /// Cloverleaf (k = 3-4)
    D,

    /// Complex (k = 5+)
    F,

    /// Exotic high-k orbitals
    G { k_value: u32 },
}

impl PrimeAtom {
    /// Create a new prime atom from a membrane configuration
    pub fn from_config(config: &MembraneConfig) -> Self {
        let structure = AtomicStructure {
            nucleus: Nucleus {
                seed: Some(5), // Default seed
                pattern: None,
                spin: NuclearSpin::HalfInteger { value: 0.5 },
                resonance_frequency: 0.0,
            },
            shells: vec![
                MembraneShell {
                    level: 1,
                    digit: config.inner,
                    orbital_radius: config.k_inner,
                    orbital_type: k_to_orbital(config.k_inner),
                },
                MembraneShell {
                    level: 2,
                    digit: config.outer,
                    orbital_radius: config.k_outer,
                    orbital_type: k_to_orbital(config.k_outer),
                },
            ],
            total_zeros: 2 * (config.k_inner + config.k_outer),
            symmetry: SymmetryType::Symmetric,
        };

        let properties = AtomicProperties {
            prime_density: config.expected_density,
            mass: 10.0, // Placeholder
            charge: 0.1,
            stability: 0.8,
            binding_affinity: 0.5,
        };

        PrimeAtom {
            base: config.base,
            structure,
            properties,
            examples: Vec::new(),
            education_level: EducationLevel::Moderate,
        }
    }

    /// Get a human-readable explanation at the specified education level
    pub fn explain(&self, level: EducationLevel) -> String {
        match level {
            EducationLevel::Introductory => self.explain_introductory(),
            EducationLevel::Moderate => self.explain_moderate(),
            EducationLevel::Advanced => self.explain_advanced(),
            EducationLevel::Expert => self.explain_expert(),
        }
    }

    fn explain_introductory(&self) -> String {
        format!(
            "Imagine a prime number as an atom in base {}!\n\n\
             • Center: Like a nucleus with {} energy\n\
             • Shells: {} layers surrounding it\n\
             • Empty space: {} zeros between layers\n\
             • Success rate: {:.1}% chance of being prime\n\n\
             It's like building a number with a specific pattern that \
             'wants' to be prime!",
            self.base,
            self.structure.nucleus.spin.description(),
            self.structure.shells.len(),
            self.structure.total_zeros,
            self.properties.prime_density * 100.0
        )
    }

    fn explain_moderate(&self) -> String {
        let shells_desc = self
            .structure
            .shells
            .iter()
            .map(|s| {
                format!(
                    "Level {}: digit {} at radius {} ({})",
                    s.level,
                    s.digit,
                    s.orbital_radius,
                    s.orbital_type.symbol()
                )
            })
            .collect::<Vec<_>>()
            .join("\n  ");

        format!(
            "Prime Atom Structure (Base {}):\n\n\
             Nucleus: {:?}\n\
             Electron Shells:\n  {}\n\n\
             Physical Properties:\n\
             • Mass: {:.2} (from digit count and resonance)\n\
             • Charge: {:.2} (prime digit density)\n\
             • Stability: {:.2} (resistance to perturbation)\n\n\
             This configuration has {:.1}% prime density, meaning \
             about 1 in {:.0} numbers with this pattern are prime.",
            self.base,
            self.structure.nucleus,
            shells_desc,
            self.properties.mass,
            self.properties.charge,
            self.properties.stability,
            self.properties.prime_density * 100.0,
            1.0 / self.properties.prime_density
        )
    }

    fn explain_advanced(&self) -> String {
        format!(
            "Prime Atom Analysis (Base {}):\n\n\
             Hamiltonian: H = T + V\n\
             where T = kinetic term from membrane oscillations\n\
                   V = potential from base metric curvature\n\n\
             Wave Function: Ψ(n) = A × exp(-(n-n₀)²/2σ²) × sin(2πn/λ + φ)\n\
             where n₀ = preferred nucleus position\n\
                   σ = membrane width parameter\n\
                   λ = base-dependent wavelength\n\
                   φ = phase from boundary conditions\n\n\
             Quantum Numbers:\n\
             • Principal (n): {} (shell count)\n\
             • Azimuthal (l): {:?} (orbital shapes)\n\
             • Magnetic (m): {} (symmetry breaking)\n\
             • Spin (s): {:?}\n\n\
             Selection Rules:\n\
             • Δn = ±1 (shell transitions)\n\
             • Δl = ±1 (orbital changes)\n\
             • Conservation of parity under base transformation",
            self.base,
            self.structure.shells.len(),
            self.structure
                .shells
                .iter()
                .map(|s| s.orbital_type.symbol())
                .collect::<Vec<_>>()
                .join(","),
            match &self.structure.symmetry {
                SymmetryType::Symmetric => 0,
                SymmetryType::Breathing { .. } => 1,
                _ => 2,
            },
            self.structure.nucleus.spin
        )
    }

    fn explain_expert(&self) -> String {
        format!(
            "Topological Prime Field Theory (Base {}):\n\n\
             Action: S[φ] = ∫ d^n x √|g| [ R + L_matter + L_membrane ]\n\n\
             Membrane Lagrangian:\n\
             L_membrane = -T ∫ d^(p+1)ξ √|det(G_ab)| + ∫ A_p+1\n\
             where T = membrane tension ~ 1/prime_density\n\
                   G_ab = induced metric on worldvolume\n\
                   A_p+1 = RR potential coupling\n\n\
             Supersymmetry: {} preserved\n\
             Moduli Space: {} complex dimensions\n\n\
             D-brane Interpretation:\n\
             • D{}-brane wrapping {} cycle\n\
             • Open strings ending on membrane give boundary digits\n\
             • Closed string modes in bulk determine k-values\n\n\
             Holographic Correspondence:\n\
             Prime density ↔ Entanglement entropy\n\
             Membrane tension ↔ Central charge\n\
             Base metric ↔ Bulk geometry",
            self.base,
            match &self.structure.symmetry {
                SymmetryType::Symmetric => "N=2",
                _ => "N=1",
            },
            self.structure.shells.len(),
            self.base - 1,
            self.base % 4
        )
    }

    /// Add a discovered prime example
    pub fn add_example(&mut self, prime: BigUint, context: String) {
        let structure_diagram = self.visualize_structure(&prime);
        let features = self.analyze_features(&prime);

        self.examples.push(DiscoveredPrime {
            value: prime,
            discovery_context: context,
            notable_features: features,
            structure_diagram,
        });
    }

    /// Create ASCII art visualization of the atomic structure
    pub fn visualize(&self) -> String {
        let mut vis = String::new();

        // Header
        vis.push_str(&format!(
            "\n{:^50}\n",
            format!("Prime Atom (Base {})", self.base)
        ));
        vis.push_str(&"=".repeat(50));
        vis.push_str("\n\n");

        // Orbital diagram
        let max_radius = self
            .structure
            .shells
            .iter()
            .map(|s| s.orbital_radius)
            .max()
            .unwrap_or(5) as usize;

        let size = 15 + 4 * max_radius;
        let center = size / 2;

        // Create grid
        let mut grid = vec![vec![' '; size]; size];

        // Draw nucleus
        let nucleus_char = match &self.structure.nucleus.seed {
            Some(s) => char::from_digit(*s, 10).unwrap_or('*'),
            None => '*',
        };
        grid[center][center] = nucleus_char;

        // Draw shells
        for shell in &self.structure.shells {
            let radius = 3 + 2 * shell.orbital_radius as usize;
            let shell_char = char::from_digit(shell.digit, 10).unwrap_or('#');

            // Draw circle (simplified)
            for angle in 0..360 {
                let theta = angle as f64 * std::f64::consts::PI / 180.0;
                let x = center as i32 + (radius as f64 * theta.cos()) as i32;
                let y = center as i32 + (radius as f64 * theta.sin()) as i32;

                if x >= 0 && x < size as i32 && y >= 0 && y < size as i32 && angle % 45 == 0 {
                    // Draw at 8 points
                    grid[y as usize][x as usize] = shell_char;
                }
            }
        }

        // Draw zero padding as dots
        for shell in &self.structure.shells {
            let inner_radius = if shell.level == 1 {
                1
            } else {
                3 + 2 * self.structure.shells[shell.level as usize - 2].orbital_radius as usize
            };
            let outer_radius = 3 + 2 * shell.orbital_radius as usize;

            for r in inner_radius..outer_radius {
                if r % 2 == 0 {
                    for angle in (0..360).step_by(60) {
                        let theta = angle as f64 * std::f64::consts::PI / 180.0;
                        let x = center as i32 + (r as f64 * theta.cos()) as i32;
                        let y = center as i32 + (r as f64 * theta.sin()) as i32;

                        if x >= 0
                            && x < size as i32
                            && y >= 0
                            && y < size as i32
                            && grid[y as usize][x as usize] == ' '
                        {
                            grid[y as usize][x as usize] = '·';
                        }
                    }
                }
            }
        }

        // Convert grid to string
        for row in &grid {
            vis.push_str(&row.iter().collect::<String>());
            vis.push('\n');
        }

        // Add legend
        vis.push_str("\nLegend:\n");
        vis.push_str(&format!("  {nucleus_char} = Nucleus (seed/pattern)\n"));
        for shell in &self.structure.shells {
            let shell_char = char::from_digit(shell.digit, 10).unwrap_or('#');
            vis.push_str(&format!(
                "  {} = Shell {} (digit {}, {} orbital)\n",
                shell_char,
                shell.level,
                shell.digit,
                shell.orbital_type.symbol()
            ));
        }
        vis.push_str("  · = Zero padding (empty space)\n");

        vis
    }

    /// Visualize a specific prime's structure
    fn visualize_structure(&self, prime: &BigUint) -> String {
        let prime_str = prime.to_string();
        let mut diagram = String::new();

        // Try to parse the structure
        diagram.push_str(&format!("Structure of {prime_str}:\n"));
        diagram.push_str(&format!("Length: {} digits\n\n", prime_str.len()));

        // Simple linear representation
        let chars: Vec<char> = prime_str.chars().collect();
        if chars.len() >= 5 {
            diagram.push_str("Possible membrane structure:\n");
            diagram.push_str(&format!("{} ", chars[0])); // outer

            let mut i = 1;
            while i < chars.len() && chars[i] == '0' {
                diagram.push('0');
                i += 1;
            }

            if i < chars.len() {
                diagram.push_str(&format!(" {} ", chars[i])); // inner
                i += 1;

                while i < chars.len() && chars[i] == '0' {
                    diagram.push('0');
                    i += 1;
                }

                diagram.push_str(" [");
                while i < chars.len() - 3 {
                    diagram.push(chars[i]);
                    i += 1;
                }
                diagram.push_str("] ");

                // Right side (abbreviated)
                diagram.push_str("... ");
                diagram.push(chars[chars.len() - 1]);
            }
        } else {
            diagram.push_str(&prime_str);
        }

        diagram
    }

    /// Analyze special features of a prime
    fn analyze_features(&self, prime: &BigUint) -> Vec<String> {
        let mut features = Vec::new();
        let prime_str = prime.to_string();

        // Check for 37/73 patterns
        if prime_str.contains("37") {
            features.push("Contains magical 37 pattern".to_string());
        }
        if prime_str.contains("73") {
            features.push("Contains mirror 73 pattern".to_string());
        }

        // Check prime digit density
        let prime_digits = prime_str
            .chars()
            .filter(|&c| matches!(c, '2' | '3' | '5' | '7'))
            .count();
        let density = prime_digits as f64 / prime_str.len() as f64;
        features.push(format!("Prime digit density: {:.1}%", density * 100.0));

        // Check for palindromes
        if prime_str == prime_str.chars().rev().collect::<String>() {
            features.push("Perfect palindrome!".to_string());
        }

        // Length category
        match prime_str.len() {
            1..=10 => features.push("Small prime (high quantum effects)".to_string()),
            11..=20 => features.push("Medium prime (balanced properties)".to_string()),
            21..=50 => features.push("Large prime (classical behavior)".to_string()),
            _ => features.push("Massive prime (gravitational dominance)".to_string()),
        }

        features
    }
}

/// Convert k-value to orbital designation
fn k_to_orbital(k: u32) -> OrbitalDesignation {
    match k {
        0..=1 => OrbitalDesignation::S,
        2 => OrbitalDesignation::P,
        3..=4 => OrbitalDesignation::D,
        5..=6 => OrbitalDesignation::F,
        _ => OrbitalDesignation::G { k_value: k },
    }
}

impl NuclearSpin {
    fn description(&self) -> &str {
        match self {
            NuclearSpin::Zero => "neutral",
            NuclearSpin::HalfInteger { .. } => "half-spin",
            NuclearSpin::Integer { .. } => "full-spin",
            NuclearSpin::Exotic { .. } => "exotic",
        }
    }
}

impl OrbitalDesignation {
    fn symbol(&self) -> &str {
        match self {
            OrbitalDesignation::S => "s",
            OrbitalDesignation::P => "p",
            OrbitalDesignation::D => "d",
            OrbitalDesignation::F => "f",
            OrbitalDesignation::G { .. } => "g",
        }
    }
}

impl fmt::Display for PrimeAtom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PrimeAtom[Base {}, {} shells, {:.1}% density]",
            self.base,
            self.structure.shells.len(),
            self.properties.prime_density * 100.0
        )
    }
}

/// # Cross-Base Interactions
///
/// When atoms from different bases interact, special phenomena occur
pub struct CrossBaseInteraction {
    pub atom1: PrimeAtom,
    pub atom2: PrimeAtom,
    pub interaction_type: InteractionType,
    pub binding_energy: f64,
    pub products: Vec<InteractionProduct>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    /// Atoms attract (same parity bases)
    Attractive { force: f64 },

    /// Atoms repel (even vs odd bases)
    Repulsive { force: f64 },

    /// Form a stable molecule
    Bonding { bond_order: f64 },

    /// Exchange properties
    Resonant { frequency: f64 },

    /// Chaotic interaction
    Chaotic { lyapunov_exponent: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionProduct {
    /// A new prime is catalyzed
    NewPrime { value: BigUint },

    /// Energy is released
    Energy { amount: f64 },

    /// Pattern propagates
    Wave { wavelength: f64, amplitude: f64 },

    /// Stable compound forms
    Molecule { components: Vec<u32> },
}

/// Create a "periodic table" of prime atoms
pub struct PrimePeriodicTable {
    /// Atoms organized by base and configuration
    pub elements: Vec<Vec<PrimeAtom>>,

    /// Discovered interactions
    pub reactions: Vec<CrossBaseInteraction>,
}

impl PrimePeriodicTable {
    /// Get all stable configurations for a given base
    pub fn get_stable_atoms(base: u32) -> Vec<PrimeAtom> {
        // Based on our discoveries
        match base {
            10 => vec![
                // The magical (3,7) - our "hydrogen"
                PrimeAtom::from_config(&MembraneConfig::new(10, 3, 7, 2, 2)),
                // High-density breathing patterns - "helium"
                PrimeAtom::from_config(&MembraneConfig::breathing(10, 3, 7, 0, 1, 3, 2)),
                // Twin boundaries - "lithium"
                PrimeAtom::from_config(&MembraneConfig::new(10, 3, 3, 1, 1)),
            ],
            11 => vec![
                // Prime base special - "beryllium"
                PrimeAtom::from_config(&MembraneConfig::new(11, 3, 8, 2, 2)),
            ],
            12 => vec![
                // Bridge configuration - "boron"
                PrimeAtom::from_config(&MembraneConfig::new(12, 5, 7, 2, 2)),
            ],
            _ => vec![
                // Generic configuration
                PrimeAtom::from_config(&MembraneConfig::new(base, 1, base - 1, 1, 1)),
            ],
        }
    }
}

/// Educational examples with increasing complexity
pub mod examples {
    use super::*;

    /// Example 1: The simplest prime atom (Base 10)
    pub fn hydrogen_prime() -> PrimeAtom {
        let mut atom = PrimeAtom::from_config(&MembraneConfig::new(10, 3, 7, 2, 2));

        // Add discovered examples
        atom.add_example(
            "30070070003".parse().unwrap(),
            "First discovered symmetric (3,7) prime".to_string(),
        );

        atom.add_example(
            "300700070003".parse().unwrap(),
            "Extra zero in middle maintains primality".to_string(),
        );

        atom
    }

    /// Example 2: A breathing prime atom
    pub fn helium_prime() -> PrimeAtom {
        let mut atom = PrimeAtom::from_config(&MembraneConfig::breathing(10, 3, 3, 1, 0, 0, 1));

        atom.add_example(
            "31303".parse().unwrap(),
            "Asymmetric breathing pattern with 25% density".to_string(),
        );

        atom
    }

    /// Example 3: Cross-base interaction
    pub fn demonstrate_interaction() -> CrossBaseInteraction {
        let atom_10 = hydrogen_prime();
        let atom_11 = PrimeAtom::from_config(&MembraneConfig::new(11, 3, 8, 2, 2));

        CrossBaseInteraction {
            atom1: atom_10,
            atom2: atom_11,
            interaction_type: InteractionType::Attractive { force: 2.5 },
            binding_energy: -15.3, // Negative = bound state
            products: vec![InteractionProduct::Wave {
                wavelength: 37.0,
                amplitude: 0.73,
            }],
        }
    }
}
