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
use std::{fs, io::ErrorKind, path::Path};
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
    let mut bytes = Vec::new();
    serde_json::to_writer_pretty(&mut bytes, value)?;
    write_bytes_if_changed(path, &bytes)
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
    write_bytes_if_changed(path, text.as_bytes())
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

fn write_bytes_if_changed(path: &Path, bytes: &[u8]) -> Result<(), ReportExportError> {
    match fs::read(path) {
        Ok(existing) if existing == bytes => Ok(()),
        Ok(_) => {
            fs::write(path, bytes)?;
            Ok(())
        }
        Err(err) if err.kind() == ErrorKind::NotFound => {
            fs::write(path, bytes)?;
            Ok(())
        }
        Err(err) => Err(err.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use std::{
        fs,
        path::PathBuf,
        process, thread,
        time::{Duration, SystemTime, UNIX_EPOCH},
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
    fn text_export_preserves_mtime_when_content_is_unchanged() {
        let base = unique_test_dir("stable-text");
        let text_path = base.join("nested/report.md");

        write_text_file(&text_path, "# Demo\n").expect("write text");
        let first_modified = fs::metadata(&text_path)
            .expect("text metadata")
            .modified()
            .expect("text modified time");

        thread::sleep(Duration::from_millis(25));
        write_text_file(&text_path, "# Demo\n").expect("rewrite same text");
        let second_modified = fs::metadata(&text_path)
            .expect("text metadata")
            .modified()
            .expect("text modified time");

        assert_eq!(first_modified, second_modified);

        thread::sleep(Duration::from_millis(25));
        write_text_file(&text_path, "# Changed\n").expect("write changed text");
        let third_modified = fs::metadata(&text_path)
            .expect("text metadata")
            .modified()
            .expect("text modified time");

        assert_ne!(second_modified, third_modified);
        assert_eq!(
            fs::read_to_string(&text_path).expect("read changed text"),
            "# Changed\n"
        );

        fs::remove_dir_all(base).expect("cleanup temp dir");
    }

    #[test]
    fn pretty_json_export_preserves_mtime_when_content_is_unchanged() {
        let base = unique_test_dir("stable-json");
        let json_path = base.join("nested/report.json");
        let first_rows = [DemoRow {
            label: "alpha",
            value: 7,
        }];
        let changed_rows = [DemoRow {
            label: "alpha",
            value: 8,
        }];

        write_json_pretty(&json_path, &first_rows).expect("write json");
        let first_modified = fs::metadata(&json_path)
            .expect("json metadata")
            .modified()
            .expect("json modified time");

        thread::sleep(Duration::from_millis(25));
        write_json_pretty(&json_path, &first_rows).expect("rewrite same json");
        let second_modified = fs::metadata(&json_path)
            .expect("json metadata")
            .modified()
            .expect("json modified time");

        assert_eq!(first_modified, second_modified);

        thread::sleep(Duration::from_millis(25));
        write_json_pretty(&json_path, &changed_rows).expect("write changed json");
        let third_modified = fs::metadata(&json_path)
            .expect("json metadata")
            .modified()
            .expect("json modified time");

        assert_ne!(second_modified, third_modified);
        assert!(fs::read_to_string(&json_path)
            .expect("read changed json")
            .contains("\"value\": 8"));

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
