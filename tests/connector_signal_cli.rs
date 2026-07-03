use serde_json::Value;
use std::{
    fs,
    path::PathBuf,
    process::{self, Command},
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn connector_replication_null_atlas_exporter_writes_expected_artifacts() {
    let out_dir = unique_temp_path("connector-replication-null-atlas-export", "dir");
    fs::create_dir_all(&out_dir).expect("create temp connector null atlas export dir");

    let export_output = Command::new(env!(
        "CARGO_BIN_EXE_export_connector_replication_null_atlas"
    ))
    .arg("--out-dir")
    .arg(&out_dir)
    .output()
    .expect("run connector replication null atlas exporter");

    assert!(
        export_output.status.success(),
        "connector replication null atlas exporter failed: {}{}",
        String::from_utf8_lossy(&export_output.stdout),
        String::from_utf8_lossy(&export_output.stderr)
    );
    let json_path = out_dir.join("connector_replication_null_atlas.json");
    let markdown_path = out_dir.join("connector_replication_null_atlas.md");
    let manifest_path = out_dir
        .join("connector_replication_null_atlas_manifest")
        .join("artifact_manifest.json");
    assert!(json_path.exists(), "missing null atlas JSON export");
    assert!(markdown_path.exists(), "missing null atlas Markdown export");
    assert!(manifest_path.exists(), "missing null atlas manifest export");

    let atlas: Value = serde_json::from_str(
        &fs::read_to_string(&json_path).expect("read connector null atlas JSON"),
    )
    .expect("parse connector null atlas JSON");
    assert_eq!(
        atlas.get("schema_version").and_then(Value::as_str),
        Some("connector-replication-null-atlas-v1")
    );
    assert_eq!(
        atlas
            .get("summary")
            .and_then(|summary| summary.get("theorem_candidate_count"))
            .and_then(Value::as_u64),
        Some(0)
    );

    fs::remove_dir_all(out_dir).expect("cleanup connector null atlas export dir");
}

#[test]
fn connector_atlas_checker_rejects_corrupt_residue_survivor_theorem_link() {
    let mut atlas: Value = serde_json::from_str(include_str!(
        "../docs/connector/connector_signal_atlas.json"
    ))
    .expect("tracked connector atlas parses as JSON");

    let missing_theorem = "PairResidueProfile.definitelyMissingSurvivorCountTheorem";
    let rows = atlas
        .get_mut("residue_survivor_rows")
        .and_then(Value::as_array_mut)
        .expect("connector atlas has residue survivor rows");
    rows[0]["lean_theorem"] = Value::from(missing_theorem);

    let atlas_path = unique_temp_path("connector-signal-atlas-corrupt", "json");
    let checks_path = unique_temp_path("connector-signal-atlas-corrupt-checks", "lean");
    fs::write(
        &atlas_path,
        serde_json::to_string_pretty(&atlas).expect("serialize corrupt connector atlas"),
    )
    .expect("write corrupt connector atlas");

    let export_output = Command::new(env!("CARGO_BIN_EXE_export_connector_signal_atlas_checks"))
        .arg("--atlas")
        .arg(&atlas_path)
        .arg("--out")
        .arg(&checks_path)
        .output()
        .expect("run connector atlas check exporter");

    assert!(
        export_output.status.success(),
        "connector atlas checker exporter rejected a syntactically valid corrupt theorem link: {}",
        String::from_utf8_lossy(&export_output.stderr)
    );
    let checks = fs::read_to_string(&checks_path).expect("read generated connector Lean checks");
    assert!(
        checks.contains(missing_theorem),
        "generated connector Lean checks did not include the corrupt theorem link"
    );

    let build_output = Command::new("lake")
        .arg("build")
        .arg("PrimeArithmetic.Connector.ConcatenationFamilies")
        .current_dir("lean-proofs")
        .output()
        .expect("build connector Lean module before temp declaration check");
    assert!(
        build_output.status.success(),
        "connector Lean module build failed before corrupt declaration check: {}{}",
        String::from_utf8_lossy(&build_output.stdout),
        String::from_utf8_lossy(&build_output.stderr)
    );

    let lean_output = Command::new("lake")
        .arg("env")
        .arg("lean")
        .arg(&checks_path)
        .current_dir("lean-proofs")
        .output()
        .expect("run Lean on corrupt connector atlas checks");

    fs::remove_file(&atlas_path).expect("remove corrupt connector atlas fixture");
    fs::remove_file(&checks_path).expect("remove corrupt connector Lean checks fixture");

    assert!(
        !lean_output.status.success(),
        "corrupt connector atlas theorem link unexpectedly passed Lean declaration check"
    );
    let output_text = format!(
        "{}{}",
        String::from_utf8_lossy(&lean_output.stdout),
        String::from_utf8_lossy(&lean_output.stderr)
    );
    assert!(
        output_text.contains("definitelyMissingSurvivorCountTheorem"),
        "Lean failure did not identify the corrupt theorem link: {output_text}"
    );
}

#[test]
fn connector_width6_stress_checker_rejects_corrupt_digit8_classifier_theorem_link() {
    let mut report: Value = serde_json::from_str(include_str!(
        "../docs/connector/connector_width6_stress.json"
    ))
    .expect("tracked connector width-6 stress report parses as JSON");

    let missing_theorem = "definitelyMissingDigit8ClassifierTheorem";
    let cell_profiles = report
        .get_mut("ladder_peak_matched_control_screen")
        .and_then(|screen| screen.get_mut("digit8_edge_zoom_probe"))
        .and_then(|probe| probe.get_mut("residue_profile"))
        .and_then(|profile| profile.get_mut("cell_profiles"))
        .and_then(Value::as_array_mut)
        .expect("stress report has digit-8 residue cell profiles");
    let mut corrupted = false;
    for cell in cell_profiles {
        let Some(rows) = cell.get_mut("separator_rows").and_then(Value::as_array_mut) else {
            continue;
        };
        for row in rows {
            if row.get("lean_theorem").and_then(Value::as_str).is_some() {
                row["lean_theorem"] = Value::from(missing_theorem);
                corrupted = true;
                break;
            }
        }
        if corrupted {
            break;
        }
    }
    assert!(
        corrupted,
        "stress report did not contain a theorem-backed separator row"
    );

    let stress_path = unique_temp_path("connector-width6-stress-corrupt", "json");
    let checks_path = unique_temp_path("connector-width6-stress-corrupt-checks", "lean");
    fs::write(
        &stress_path,
        serde_json::to_string_pretty(&report).expect("serialize corrupt connector stress report"),
    )
    .expect("write corrupt connector stress report");

    let export_output = Command::new(env!("CARGO_BIN_EXE_export_connector_width6_stress_checks"))
        .arg("--stress")
        .arg(&stress_path)
        .arg("--out")
        .arg(&checks_path)
        .output()
        .expect("run connector stress check exporter");

    assert!(
        export_output.status.success(),
        "connector stress checker exporter rejected a syntactically valid corrupt theorem link: {}",
        String::from_utf8_lossy(&export_output.stderr)
    );
    let checks = fs::read_to_string(&checks_path).expect("read generated connector stress checks");
    assert!(
        checks.contains(missing_theorem),
        "generated connector stress Lean checks did not include the corrupt theorem link"
    );

    let build_output = Command::new("lake")
        .arg("build")
        .arg("PrimeArithmetic.Connector.ConcatenationProfileExamples")
        .current_dir("lean-proofs")
        .output()
        .expect("build connector Lean module before temp declaration check");
    assert!(
        build_output.status.success(),
        "connector Lean module build failed before corrupt stress declaration check: {}{}",
        String::from_utf8_lossy(&build_output.stdout),
        String::from_utf8_lossy(&build_output.stderr)
    );

    let lean_output = Command::new("lake")
        .arg("env")
        .arg("lean")
        .arg(&checks_path)
        .current_dir("lean-proofs")
        .output()
        .expect("run Lean on corrupt connector stress checks");

    fs::remove_file(&stress_path).expect("remove corrupt connector stress fixture");
    fs::remove_file(&checks_path).expect("remove corrupt connector stress Lean checks fixture");

    assert!(
        !lean_output.status.success(),
        "corrupt connector stress theorem link unexpectedly passed Lean declaration check"
    );
    let output_text = format!(
        "{}{}",
        String::from_utf8_lossy(&lean_output.stdout),
        String::from_utf8_lossy(&lean_output.stderr)
    );
    assert!(
        output_text.contains("definitelyMissingDigit8ClassifierTheorem"),
        "Lean failure did not identify the corrupt stress theorem link: {output_text}"
    );
}

fn unique_temp_path(label: &str, extension: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{label}-{}-{nanos}.{extension}", process::id()))
}
