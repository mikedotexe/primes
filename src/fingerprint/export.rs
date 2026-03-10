//! Export Functions for Fingerprint Data
//!
//! Exports signatures to ML-friendly formats (NDJSON, CSV)

use super::signature::PrimeConstructorSignature;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Export signatures to NDJSON (newline-delimited JSON)
pub fn export_ndjson<P: AsRef<Path>>(
    signatures: &[PrimeConstructorSignature],
    path: P,
) -> io::Result<()> {
    let mut file = File::create(path)?;

    for sig in signatures {
        let json = serde_json::to_string(sig)?;
        writeln!(file, "{}", json)?;
    }

    Ok(())
}

/// Export signatures to CSV (feature vectors with label)
pub fn export_csv<P: AsRef<Path>>(
    signatures: &[PrimeConstructorSignature],
    path: P,
) -> io::Result<()> {
    let mut file = File::create(path)?;

    // Write header
    writeln!(file, "label,sample_size,{}", generate_feature_header())?;

    // Write data rows
    for sig in signatures {
        let features = sig.to_feature_vector();
        write!(file, "{},{}", sig.label, sig.sample_size)?;
        for feature in features {
            write!(file, ",{}", feature)?;
        }
        writeln!(file)?;
    }

    Ok(())
}

/// Generate CSV header for feature columns
fn generate_feature_header() -> String {
    let mut headers = Vec::new();

    // Modular features
    for r in 0..3 {
        headers.push(format!("mod3_r{}", r));
    }
    for r in 0..7 {
        headers.push(format!("mod7_r{}", r));
    }
    for r in 0..11 {
        headers.push(format!("mod11_r{}", r));
    }
    for r in 0..13 {
        headers.push(format!("mod13_r{}", r));
    }
    for r in 0..17 {
        headers.push(format!("mod17_r{}", r));
    }
    for r in 0..19 {
        headers.push(format!("mod19_r{}", r));
    }

    // Digit features
    for d in 0..10 {
        headers.push(format!("digit_{}", d));
    }

    // Scalar features
    headers.extend(vec![
        "zero_fraction".to_string(),
        "digit_entropy".to_string(),
        "palindrome_rate".to_string(),
        "mean_digit_count".to_string(),
        "var_digit_count".to_string(),
        "zero_three_only_rate".to_string(),
        "zero_six_only_rate".to_string(),
    ]);

    // Hardy-Littlewood normalized features
    headers.extend(vec![
        "hl_modular_divergence".to_string(),
        "hl_coverage_deviation".to_string(),
    ]);

    // Gap statistics features
    for modulus in [3, 7, 11, 13, 17, 19] {
        headers.push(format!("gap_mod{}_mean", modulus));
        headers.push(format!("gap_mod{}_var", modulus));
        headers.push(format!("gap_mod{}_small_excess", modulus));
        headers.push(format!("gap_mod{}_large_excess", modulus));
    }

    headers.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fingerprint::signature::PrimeConstructorSignature;
    use num_bigint::BigUint;

    #[test]
    fn test_export_ndjson() {
        let numbers: Vec<BigUint> = vec![BigUint::from(101u32), BigUint::from(103u32)];
        let sig = PrimeConstructorSignature::from_numbers("test".to_string(), &numbers);

        let temp_path = "/tmp/test_fingerprint.ndjson";
        export_ndjson(&[sig], temp_path).unwrap();

        // Verify file exists
        assert!(std::path::Path::new(temp_path).exists());
    }

    #[test]
    fn test_feature_header() {
        let header = generate_feature_header();
        assert!(header.contains("mod3_r0"));
        assert!(header.contains("digit_5"));
        assert!(header.contains("zero_fraction"));
        assert!(header.contains("gap_mod7_mean"));
    }
}
