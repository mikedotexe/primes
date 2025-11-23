//! Prime Fingerprinting Infrastructure
//!
//! This module provides spectral/modular fingerprinting for prime construction methods.
//! It enables classification and comparison of different prime generators based on their
//! modular arithmetic signatures, digit distributions, and structural properties.
//!
//! Key concepts:
//! - **Modular Profile**: Distribution of residues across multiple moduli
//! - **Constructor Signature**: Complete fingerprint including modular, digit, and structural features
//! - **Fingerprint Atlas**: Labeled dataset of signatures for ML classification

pub mod profile;
pub mod signature;
pub mod constructors;
pub mod export;

pub use profile::{ModularProfile, compute_modular_profile};
pub use signature::{PrimeConstructorSignature, SignatureFeatures};
pub use constructors::PrimeConstructor;
pub use export::{export_ndjson, export_csv};
