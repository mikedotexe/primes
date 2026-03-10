//! # Prime Fingerprinting
//!
//! **Layer**: Analysis tools
//!
//! Modular arithmetic fingerprinting for classifying and comparing prime
//! construction methods. Generates residue distributions and structural
//! signatures for different membrane configurations.

pub mod constructors;
pub mod export;
pub mod profile;
pub mod signature;

pub use constructors::PrimeConstructor;
pub use export::{export_csv, export_ndjson};
pub use profile::{compute_modular_profile, ModularProfile};
pub use signature::{PrimeConstructorSignature, SignatureFeatures};
