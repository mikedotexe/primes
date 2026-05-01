//! Shared report-artifact helpers for maintained and exploratory analyses.
//!
//! This module keeps the repo's export surface consistent:
//! - stable UTC export timestamps,
//! - parent-directory creation for artifact paths,
//! - pretty JSON bundles,
//! - row-oriented CSV exports,
//! - generated Markdown/text summaries.

use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReportExportError {
    #[error("I/O error while exporting report artifact: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON serialization failed for report artifact: {0}")]
    Json(#[from] serde_json::Error),
    #[error("CSV serialization failed for report artifact: {0}")]
    Csv(#[from] csv::Error),
}

pub fn export_timestamp_utc() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
}

pub fn ensure_dir(path: impl AsRef<Path>) -> Result<(), ReportExportError> {
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn write_json_pretty<T: Serialize>(
    path: impl AsRef<Path>,
    value: &T,
) -> Result<(), ReportExportError> {
    let path = path.as_ref();
    ensure_parent_dir(path)?;
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, value)?;
    Ok(())
}

pub fn write_csv_rows<T: Serialize>(
    path: impl AsRef<Path>,
    rows: &[T],
) -> Result<(), ReportExportError> {
    let path = path.as_ref();
    ensure_parent_dir(path)?;
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub fn write_text_file(path: impl AsRef<Path>, text: &str) -> Result<(), ReportExportError> {
    let path = path.as_ref();
    ensure_parent_dir(path)?;
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(text.as_bytes())?;
    writer.flush()?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArtifactManifest {
    pub artifact_id: String,
    pub generator_cmd: String,
    pub args: Vec<String>,
    pub upstream_inputs: Vec<String>,
    pub expected_outputs: Vec<String>,
}

pub fn write_artifact_manifest(
    out_dir: impl AsRef<Path>,
    manifest: &ArtifactManifest,
) -> Result<(), ReportExportError> {
    let out_dir = out_dir.as_ref();
    ensure_dir(out_dir)?;
    write_json_pretty(out_dir.join("artifact_manifest.json"), manifest)
}

fn ensure_parent_dir(path: &Path) -> Result<(), ReportExportError> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use std::{
        fs,
        path::PathBuf,
        process,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[derive(Debug, Serialize)]
    struct DemoRow {
        label: &'static str,
        value: u32,
    }

    fn unique_test_dir(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "primes-reporting-{label}-{}-{nanos}",
            process::id()
        ))
    }

    #[test]
    fn timestamp_uses_utc_z_suffix() {
        let stamp = export_timestamp_utc();
        assert!(stamp.ends_with('Z'));
        assert!(stamp.contains('T'));
    }

    #[test]
    fn json_csv_and_text_exports_create_parent_dirs() {
        let base = unique_test_dir("artifacts");
        let json_path = base.join("nested/report.json");
        let csv_path = base.join("nested/report.csv");
        let text_path = base.join("nested/report.md");
        let rows = [DemoRow {
            label: "alpha",
            value: 7,
        }];

        write_json_pretty(&json_path, &rows).expect("write json");
        write_csv_rows(&csv_path, &rows).expect("write csv");
        write_text_file(&text_path, "# Demo\n").expect("write text");

        let json_text = fs::read_to_string(&json_path).expect("read json");
        let csv_text = fs::read_to_string(&csv_path).expect("read csv");
        let text = fs::read_to_string(&text_path).expect("read text");

        assert!(json_text.contains("\"label\": \"alpha\""));
        assert!(csv_text.starts_with("label,value"));
        assert_eq!(text, "# Demo\n");

        fs::remove_dir_all(base).expect("cleanup temp dir");
    }

    #[test]
    fn artifact_manifest_writes_sidecar_json() {
        let base = unique_test_dir("manifest");
        let manifest = ArtifactManifest {
            artifact_id: "demo_artifact".to_string(),
            generator_cmd: "cargo".to_string(),
            args: vec![
                "run".to_string(),
                "--example".to_string(),
                "demo".to_string(),
            ],
            upstream_inputs: vec!["/tmp/input.json".to_string()],
            expected_outputs: vec!["report.md".to_string(), "summary.json".to_string()],
        };

        write_artifact_manifest(&base, &manifest).expect("write manifest");
        let manifest_text =
            fs::read_to_string(base.join("artifact_manifest.json")).expect("read manifest");

        assert!(manifest_text.contains("\"artifact_id\": \"demo_artifact\""));
        assert!(manifest_text.contains("\"expected_outputs\""));

        fs::remove_dir_all(base).expect("cleanup temp dir");
    }
}
