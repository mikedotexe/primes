use serde_json::Value;
use std::{
    fs,
    path::PathBuf,
    process::{self, Command},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[test]
fn verifier_cli_rejects_corrupt_residue_row() {
    let mut certificate: Value = serde_json::from_str(include_str!(
        "../docs/witness/seed60_proof_carrying_witness.json"
    ))
    .expect("tracked seed-60 certificate parses as JSON");

    let residue_rows = certificate
        .get_mut("residue_rows")
        .and_then(Value::as_array_mut)
        .expect("certificate has residue rows");
    residue_rows[0]["value_mod"] = Value::from(0);
    residue_rows[0]["survived"] = Value::from(false);

    let corrupt_path = unique_temp_path("proof-carrying-witness-corrupt", "json");
    fs::write(
        &corrupt_path,
        serde_json::to_string_pretty(&certificate).expect("serialize corrupt certificate"),
    )
    .expect("write corrupt certificate");

    let output = Command::new(env!("CARGO_BIN_EXE_verify-proof-carrying-witness"))
        .arg(&corrupt_path)
        .output()
        .expect("run proof-carrying witness verifier");
    fs::remove_file(&corrupt_path).expect("remove corrupt certificate fixture");

    assert!(
        !output.status.success(),
        "corrupt certificate unexpectedly passed verifier"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("proof-carrying witness certificate failed verification"),
        "stderr did not include verifier failure header: {stderr}"
    );
    assert!(
        stderr.contains("residue_rows[0]"),
        "stderr did not identify the corrupt residue row: {stderr}"
    );
}

#[test]
fn verifier_cli_accepts_tracked_certificate_and_writes_ok_report() {
    let report_path = unique_temp_path("proof-carrying-witness-report", "json");
    let output = Command::new(env!("CARGO_BIN_EXE_verify-proof-carrying-witness"))
        .arg("docs/witness/seed60_proof_carrying_witness.json")
        .arg("--json-out")
        .arg(&report_path)
        .output()
        .expect("run proof-carrying witness verifier");

    assert!(
        output.status.success(),
        "tracked certificate failed verifier: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let report_text = fs::read_to_string(&report_path).expect("read verifier report");
    fs::remove_file(&report_path).expect("remove verifier report fixture");
    let report: Value = serde_json::from_str(&report_text).expect("verifier report parses as JSON");

    assert_eq!(report["schema_version"], "proof-carrying-witness-v1");
    assert_eq!(report["ok"], true);
    assert_eq!(report["witness_seed"], 60);
    assert_eq!(report["checked_residue_row_count"], 9);
    assert_eq!(
        report["failures"].as_array().expect("failures array").len(),
        0
    );
}

#[test]
fn witness_lean_catalog_checker_exports_expected_positive_surface() {
    let checks_dir = unique_temp_dir("proof-carrying-witness-lean-catalog-positive-checks");
    let checks_path = checks_dir.join("CatalogChecks.lean");
    fs::create_dir_all(&checks_dir).expect("create temporary canonical checks output dir");
    let output = Command::new(env!(
        "CARGO_BIN_EXE_export_proof_carrying_witness_lean_catalog_checks"
    ))
    .arg("--manifest")
    .arg("docs/witness/witness_lean_catalog_manifest.json")
    .arg("--out")
    .arg(&checks_path)
    .arg("--shard-size")
    .arg("1")
    .arg("--module-prefix")
    .arg("PrimeArithmetic.Generated.Witness")
    .output()
    .expect("run witness Lean catalog checker exporter");

    assert!(
        output.status.success(),
        "checker exporter failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let umbrella = fs::read_to_string(&checks_path).expect("read generated Lean checks umbrella");
    let tracked_umbrella =
        fs::read_to_string("lean-proofs/PrimeArithmetic/Generated/Witness/CatalogChecks.lean")
            .expect("read tracked witness Lean catalog check umbrella");
    let shard_names = [
        "CatalogChecksShard01.lean",
        "CatalogChecksShard02.lean",
        "CatalogChecksShard03.lean",
    ];
    let mut shard_texts = Vec::new();
    for shard_name in shard_names {
        let shard_text =
            fs::read_to_string(checks_dir.join(shard_name)).expect("read generated catalog shard");
        let tracked_shard = fs::read_to_string(format!(
            "lean-proofs/PrimeArithmetic/Generated/Witness/{shard_name}"
        ))
        .expect("read tracked catalog shard");
        assert_eq!(
            shard_text, tracked_shard,
            "exported witness Lean catalog shard drifted: {shard_name}"
        );
        shard_texts.push(shard_text);
    }
    let checks = shard_texts.join("\n");
    fs::remove_dir_all(&checks_dir).expect("remove generated Lean checks fixture");

    let umbrella_imports: Vec<_> = umbrella
        .lines()
        .filter(|line| line.starts_with("import PrimeArithmetic.Generated.Witness.Catalog"))
        .collect();
    let generated_imports: Vec<_> = checks
        .lines()
        .filter(|line| line.starts_with("import PrimeArithmetic.Generated.Witness."))
        .collect();

    assert_eq!(
        umbrella_imports,
        vec![
            "import PrimeArithmetic.Generated.Witness.CatalogChecksShard01",
            "import PrimeArithmetic.Generated.Witness.CatalogChecksShard02",
            "import PrimeArithmetic.Generated.Witness.CatalogChecksShard03",
        ]
    );
    assert_eq!(checks.matches("example : True := by").count(), 186);
    assert_eq!(
        generated_imports,
        vec![
            "import PrimeArithmetic.Generated.Witness.Seed60",
            "import PrimeArithmetic.Generated.Witness.Teaching38",
            "import PrimeArithmetic.Generated.Witness.TimestampPolicy29Trial0",
        ]
    );
    assert!(checks.contains("import PrimeArithmetic.Witness.TeachingSeedCertificate"));
    assert_eq!(
        umbrella, tracked_umbrella,
        "exported witness Lean catalog check umbrella drifted from the tracked artifact"
    );
}

#[test]
fn witness_lean_catalog_checker_preserves_mtimes_when_contents_are_unchanged() {
    let checks_dir = unique_temp_dir("proof-carrying-witness-lean-catalog-stable-checks");
    let checks_path = checks_dir.join("CatalogChecks.lean");
    fs::create_dir_all(&checks_dir).expect("create temporary canonical checks output dir");

    let run_export = || {
        Command::new(env!(
            "CARGO_BIN_EXE_export_proof_carrying_witness_lean_catalog_checks"
        ))
        .arg("--manifest")
        .arg("docs/witness/witness_lean_catalog_manifest.json")
        .arg("--out")
        .arg(&checks_path)
        .arg("--shard-size")
        .arg("1")
        .arg("--module-prefix")
        .arg("PrimeArithmetic.Generated.Witness")
        .output()
        .expect("run witness Lean catalog checker exporter")
    };

    let first_output = run_export();
    assert!(
        first_output.status.success(),
        "checker exporter failed: {}",
        String::from_utf8_lossy(&first_output.stderr)
    );

    let generated_paths = [
        checks_path.clone(),
        checks_dir.join("CatalogChecksShard01.lean"),
        checks_dir.join("CatalogChecksShard02.lean"),
        checks_dir.join("CatalogChecksShard03.lean"),
    ];
    let first_mtimes: Vec<_> = generated_paths.iter().map(file_mtime).collect();

    thread::sleep(Duration::from_millis(25));
    let second_output = run_export();
    assert!(
        second_output.status.success(),
        "checker exporter failed on stable rewrite: {}",
        String::from_utf8_lossy(&second_output.stderr)
    );
    let second_mtimes: Vec<_> = generated_paths.iter().map(file_mtime).collect();

    assert_eq!(first_mtimes, second_mtimes);

    fs::remove_dir_all(&checks_dir).expect("remove generated Lean checks fixture");
}

#[test]
fn witness_lean_catalog_checker_rejects_corrupt_theorem_link() {
    let mut manifest: Value = serde_json::from_str(include_str!(
        "../docs/witness/witness_lean_catalog_manifest.json"
    ))
    .expect("tracked witness Lean catalog manifest parses as JSON");

    manifest["artifacts"][0]["theorem_names"]["width"] =
        Value::from("PrimeArithmetic.Generated.Witness.Seed60.definitelyMissingWidthTheorem");

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_nanos();
    let stem = format!("CatalogChecksCorrupt{}{}", process::id(), nanos);
    let manifest_path = unique_temp_path("proof-carrying-witness-lean-catalog-corrupt", "json");
    let checks_path = PathBuf::from("lean-proofs")
        .join("PrimeArithmetic")
        .join("Generated")
        .join("Witness")
        .join(format!("{stem}.lean"));
    fs::write(
        &manifest_path,
        serde_json::to_string_pretty(&manifest).expect("serialize corrupt Lean catalog manifest"),
    )
    .expect("write corrupt Lean catalog manifest");

    let export_output = Command::new(env!(
        "CARGO_BIN_EXE_export_proof_carrying_witness_lean_catalog_checks"
    ))
    .arg("--manifest")
    .arg(&manifest_path)
    .arg("--out")
    .arg(&checks_path)
    .arg("--shard-size")
    .arg("1")
    .arg("--module-prefix")
    .arg("PrimeArithmetic.Generated.Witness")
    .output()
    .expect("run witness Lean catalog checker exporter");

    assert!(
        export_output.status.success(),
        "checker exporter failed unexpectedly: {}",
        String::from_utf8_lossy(&export_output.stderr)
    );

    let lean_output = Command::new("lake")
        .arg("env")
        .arg("lean")
        .arg(format!(
            "PrimeArithmetic/Generated/Witness/{stem}Shard01.lean"
        ))
        .current_dir("lean-proofs")
        .output()
        .expect("run Lean on corrupt witness Lean catalog checks");

    fs::remove_file(&manifest_path).expect("remove corrupt manifest fixture");
    fs::remove_file(&checks_path).expect("remove corrupt Lean checks fixture");
    for index in 1..=3 {
        let shard_path = PathBuf::from("lean-proofs")
            .join("PrimeArithmetic")
            .join("Generated")
            .join("Witness")
            .join(format!("{stem}Shard{index:02}.lean"));
        if shard_path.exists() {
            fs::remove_file(&shard_path).expect("remove corrupt Lean check shard fixture");
        }
    }

    assert!(
        !lean_output.status.success(),
        "corrupt witness Lean catalog unexpectedly passed Lean declaration check"
    );
    let output_text = format!(
        "{}{}",
        String::from_utf8_lossy(&lean_output.stdout),
        String::from_utf8_lossy(&lean_output.stderr)
    );
    assert!(
        output_text.contains("definitelyMissingWidthTheorem"),
        "Lean failure did not identify the corrupt theorem link: {output_text}"
    );
}

#[test]
fn witness_lean_certificate_exporter_preserves_mtime_when_contents_are_unchanged() {
    let out_dir = unique_generated_lean_dir("WitnessCliStableExport");
    let lean_path = out_dir.join("Teaching38.lean");
    fs::create_dir_all(&out_dir).expect("create temporary generated Lean output dir");

    let run_export = || {
        Command::new(env!(
            "CARGO_BIN_EXE_export_proof_carrying_witness_lean_certificate"
        ))
        .arg("--certificate")
        .arg("docs/witness/teaching38_proof_carrying_witness.json")
        .arg("--out")
        .arg(&lean_path)
        .output()
        .expect("run witness Lean certificate exporter")
    };

    let first_output = run_export();
    assert!(
        first_output.status.success(),
        "Lean exporter failed: {}",
        String::from_utf8_lossy(&first_output.stderr)
    );
    let first_mtime = file_mtime(&lean_path);

    thread::sleep(Duration::from_millis(25));
    let second_output = run_export();
    assert!(
        second_output.status.success(),
        "Lean exporter failed on stable rewrite: {}",
        String::from_utf8_lossy(&second_output.stderr)
    );
    let second_mtime = file_mtime(&lean_path);

    assert_eq!(first_mtime, second_mtime);

    fs::remove_dir_all(&out_dir).expect("remove temporary generated Lean output dir");
}

#[test]
fn witness_search_policy_atlas_exporter_matches_tracked_artifacts() {
    let out_dir = unique_temp_dir("proof-carrying-witness-search-policy-atlas");
    fs::create_dir_all(&out_dir).expect("create temporary atlas output dir");

    let output = Command::new(env!(
        "CARGO_BIN_EXE_export_proof_carrying_witness_search_policy_atlas"
    ))
    .arg("--certificate-dir")
    .arg("docs/witness")
    .arg("--out-dir")
    .arg(&out_dir)
    .output()
    .expect("run witness search-policy atlas exporter");

    assert!(
        output.status.success(),
        "search-policy atlas exporter failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let exported_json = fs::read_to_string(out_dir.join("witness_search_policy_atlas.json"))
        .expect("read exported atlas json");
    let exported_markdown = fs::read_to_string(out_dir.join("witness_search_policy_atlas.md"))
        .expect("read exported atlas markdown");
    fs::remove_dir_all(&out_dir).expect("remove temporary atlas output dir");

    let tracked_json = fs::read_to_string("docs/witness/witness_search_policy_atlas.json")
        .expect("read tracked atlas json");
    let tracked_markdown = fs::read_to_string("docs/witness/witness_search_policy_atlas.md")
        .expect("read tracked atlas markdown");
    let parsed: Value = serde_json::from_str(&exported_json).expect("atlas json parses");

    assert_eq!(
        parsed["schema_version"],
        "proof-carrying-witness-search-policy-atlas-v1"
    );
    assert_eq!(parsed["summary"]["artifact_count"], 3);
    assert_eq!(
        parsed["summary"]["all_have_first_accepted_survivor_theorem"],
        true
    );
    assert_eq!(
        exported_json, tracked_json,
        "exported witness search-policy atlas JSON drifted from the tracked artifact"
    );
    assert_eq!(
        exported_markdown, tracked_markdown,
        "exported witness search-policy atlas Markdown drifted from the tracked artifact"
    );
}

#[test]
fn witness_policy_matrix_exporter_writes_candidate_bundle() {
    let out_dir = unique_temp_dir("proof-carrying-witness-policy-matrix");
    fs::create_dir_all(&out_dir).expect("create temporary policy matrix output dir");

    let output = Command::new(env!(
        "CARGO_BIN_EXE_export_proof_carrying_witness_policy_matrix"
    ))
    .arg("--out-dir")
    .arg(&out_dir)
    .output()
    .expect("run witness policy matrix exporter");

    assert!(
        output.status.success(),
        "policy matrix exporter failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let report_text = fs::read_to_string(out_dir.join("witness_policy_matrix.json"))
        .expect("read policy matrix json");
    let markdown = fs::read_to_string(out_dir.join("witness_policy_matrix.md"))
        .expect("read policy matrix markdown");
    let atlas_text = fs::read_to_string(out_dir.join("witness_policy_matrix_atlas.json"))
        .expect("read policy matrix atlas json");
    let atlas_markdown = fs::read_to_string(out_dir.join("witness_policy_matrix_atlas.md"))
        .expect("read policy matrix atlas markdown");
    let manifest_text =
        fs::read_to_string(out_dir.join("artifact_manifest.json")).expect("read manifest");
    let report: Value = serde_json::from_str(&report_text).expect("policy matrix parses as JSON");
    let atlas: Value =
        serde_json::from_str(&atlas_text).expect("policy matrix atlas parses as JSON");
    let manifest: Value = serde_json::from_str(&manifest_text).expect("manifest parses as JSON");

    assert_eq!(
        report["schema_version"],
        "proof-carrying-witness-policy-matrix-v1"
    );
    assert_eq!(
        atlas["schema_version"],
        "proof-carrying-witness-policy-matrix-atlas-v1"
    );
    assert_eq!(report["summary"]["row_count"], 21);
    assert_eq!(report["summary"]["lane_count"], 6);
    assert_eq!(report["summary"]["matrix_lean_promoted_count"], 18);
    assert_eq!(report["summary"]["small_lean_candidate_count"], 0);
    assert_eq!(report["summary"]["atlas_only_large_candidate_count"], 0);
    assert_eq!(atlas["summary"]["row_count"], 21);
    assert_eq!(atlas["summary"]["promoted_replay_certified_count"], 21);
    assert_eq!(atlas["summary"]["unpromoted_replay_candidate_count"], 0);
    assert_eq!(atlas["summary"]["promoted_large_replay_geometry_count"], 12);
    assert_eq!(
        atlas["next_replay_target"]["status"],
        "none-current-smoke-matrix-fully-covered"
    );
    assert!(atlas["next_replay_target"]["artifact_id"].is_null());
    assert_eq!(
        atlas["promoted_large_replay_geometry_rows"][10]["artifact_id"],
        "matrix-decimal-readable-64d-seed0"
    );
    assert!(markdown.contains("Proof-Carrying Witness Policy Matrix"));
    assert!(markdown.contains("matrix-base6-compact-18d-seed0"));
    assert!(markdown.contains("generated-lean-policy-matrix"));
    assert!(atlas_markdown.contains("Proof-Carrying Witness Policy-Matrix Atlas"));
    assert!(atlas_markdown.contains("Promoted Large Replay Geometry"));
    assert!(atlas_markdown.contains("lean-replay-certified"));
    assert!(atlas_markdown.contains("not a primality proof"));
    assert_eq!(
        manifest["artifact_id"],
        "proof-carrying-witness-policy-matrix-smoke-v1"
    );
    assert!(out_dir
        .join("certificates/matrix_base6_compact_18d_seed0.json")
        .is_file());
    assert!(out_dir
        .join("certificates/matrix_seed60_canonical_128d.json")
        .is_file());
    for certificate in [
        "matrix_decimal_readable_22d_seed0.json",
        "matrix_decimal_classic_22d_seed0.json",
        "matrix_decimal_breathing_22d_seed0.json",
        "matrix_decimal_readable_64d_seed0.json",
        "matrix_decimal_readable_96d_seed0.json",
        "matrix_decimal_classic_64d_seed0.json",
        "matrix_decimal_breathing_64d_seed0.json",
        "matrix_decimal_classic_96d_seed0.json",
        "matrix_decimal_breathing_96d_seed0.json",
        "matrix_base30_wheel_64d_seed0.json",
        "matrix_base30_wheel_96d_seed0.json",
        "matrix_base6_compact_18d_seed0.json",
        "matrix_base12_compact_18d_seed0.json",
        "matrix_base6_compact_64d_seed0.json",
        "matrix_base6_compact_96d_seed0.json",
        "matrix_base12_compact_64d_seed0.json",
        "matrix_base12_compact_96d_seed0.json",
        "matrix_base30_wheel_18d_seed0.json",
    ] {
        let exported = fs::read_to_string(out_dir.join("certificates").join(certificate))
            .expect("read exported promoted matrix certificate");
        let tracked =
            fs::read_to_string(PathBuf::from("docs/witness/policy_matrix").join(certificate))
                .expect("read tracked promoted matrix certificate");
        assert_eq!(
            exported, tracked,
            "promoted matrix certificate {certificate} drifted from policy matrix output"
        );
    }

    fs::remove_dir_all(&out_dir).expect("remove temporary policy matrix output dir");
}

#[test]
fn witness_lean_certificate_exporter_matches_tracked_catalog_manifest() {
    let out_dir = unique_generated_lean_dir("WitnessCliCatalogFixture");
    let manifest_path = unique_temp_path("proof-carrying-witness-lean-catalog-export", "json");
    fs::create_dir_all(&out_dir).expect("create temporary generated Lean output dir");

    let output = Command::new(env!(
        "CARGO_BIN_EXE_export_proof_carrying_witness_lean_certificate"
    ))
    .arg("--catalog")
    .arg("--certificate-dir")
    .arg("docs/witness")
    .arg("--out-dir")
    .arg(&out_dir)
    .arg("--manifest-out")
    .arg(&manifest_path)
    .output()
    .expect("run witness Lean certificate catalog exporter");

    assert!(
        output.status.success(),
        "certificate catalog exporter failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let manifest_text = fs::read_to_string(&manifest_path).expect("read exported Lean manifest");
    fs::remove_file(&manifest_path).expect("remove exported Lean manifest fixture");
    fs::remove_dir_all(&out_dir).expect("remove temporary generated Lean output dir");
    let manifest: Value =
        serde_json::from_str(&manifest_text).expect("exported Lean manifest parses as JSON");
    let tracked_manifest: Value = serde_json::from_str(include_str!(
        "../docs/witness/witness_lean_catalog_manifest.json"
    ))
    .expect("tracked witness Lean catalog manifest parses as JSON");
    let artifacts = manifest["artifacts"]
        .as_array()
        .expect("manifest artifacts array");

    let artifact_ids: Vec<_> = artifacts
        .iter()
        .map(|artifact| artifact["artifact_id"].as_str().expect("artifact id"))
        .collect();
    let generated_modules: Vec<_> = artifacts
        .iter()
        .map(|artifact| {
            artifact["generated_lean_module"]
                .as_str()
                .expect("generated Lean module")
        })
        .collect();

    assert_eq!(
        artifact_ids,
        vec![
            "seed60-canonical-128d",
            "teaching-seed0-38d",
            "timestamp-policy-trial0-29d",
        ]
    );
    assert_eq!(
        generated_modules,
        vec![
            "PrimeArithmetic.Generated.Witness.Seed60",
            "PrimeArithmetic.Generated.Witness.Teaching38",
            "PrimeArithmetic.Generated.Witness.TimestampPolicy29Trial0",
        ]
    );
    assert!(artifacts[0]["theorem_wrapper"].is_null());
    assert!(artifacts[2]["theorem_wrapper"].is_null());

    let teaching_wrapper = artifacts[1]["theorem_wrapper"]
        .as_object()
        .expect("teaching artifact has theorem wrapper");
    assert_eq!(
        teaching_wrapper["lean_module"],
        "PrimeArithmetic.Witness.TeachingSeedCertificate"
    );
    let wrapper_theorems: Vec<_> = teaching_wrapper["theorem_names"]
        .as_array()
        .expect("wrapper theorem names array")
        .iter()
        .map(|theorem| theorem.as_str().expect("wrapper theorem name"))
        .collect();
    assert_eq!(
        wrapper_theorems,
        vec![
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_width",
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_shift",
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_gradient",
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_value",
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_value_eq_shift_add_gradient",
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_residue_moduli_nodup",
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_residue_funnel_affine_checks",
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_residue_funnel_survives",
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_rejection_examples_reject",
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_search_replay_seeds_length",
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_search_replay_witness_seed",
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_search_replay_rejections_reject",
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_search_replay_survivors_survive",
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_pre_witness_replay_complete",
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_search_replay_witness_survives",
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_search_replay_sound",
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_search_replay_survivor_list_exact",
            "PrimeArithmetic.Witness.TeachingSeedCertificate.teaching38_search_replay_first_accepted_survivor",
        ]
    );
    assert_eq!(
        manifest, tracked_manifest,
        "exported Lean catalog manifest drifted from the tracked artifact"
    );
}

#[test]
fn witness_policy_matrix_lean_catalog_exporter_matches_tracked_manifest_and_checks() {
    let out_dir = unique_generated_lean_dir("WitnessCliMatrixCatalogFixture");
    let manifest_path = unique_temp_path(
        "proof-carrying-witness-policy-matrix-lean-catalog-export",
        "json",
    );
    let checks_dir = unique_temp_dir("proof-carrying-witness-policy-matrix-lean-catalog-checks");
    let checks_path = checks_dir.join("MatrixCatalogChecks.lean");
    fs::create_dir_all(&out_dir).expect("create temporary generated Lean output dir");
    fs::create_dir_all(&checks_dir).expect("create temporary matrix checks output dir");

    let output = Command::new(env!(
        "CARGO_BIN_EXE_export_proof_carrying_witness_lean_certificate"
    ))
    .arg("--policy-matrix-catalog")
    .arg("--certificate-dir")
    .arg("docs/witness/policy_matrix")
    .arg("--out-dir")
    .arg(&out_dir)
    .arg("--manifest-out")
    .arg(&manifest_path)
    .output()
    .expect("run policy-matrix Lean certificate catalog exporter");

    assert!(
        output.status.success(),
        "policy-matrix certificate catalog exporter failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let manifest_text =
        fs::read_to_string(&manifest_path).expect("read exported policy-matrix Lean manifest");
    let manifest: Value = serde_json::from_str(&manifest_text)
        .expect("exported policy-matrix Lean manifest parses as JSON");
    let tracked_manifest_text =
        fs::read_to_string("docs/witness/witness_policy_matrix_lean_catalog_manifest.json")
            .expect("read tracked policy-matrix Lean catalog manifest");
    let tracked_manifest: Value = serde_json::from_str(&tracked_manifest_text)
        .expect("tracked policy-matrix Lean catalog manifest parses as JSON");
    let artifacts = manifest["artifacts"]
        .as_array()
        .expect("policy-matrix manifest artifacts array");

    let artifact_ids: Vec<_> = artifacts
        .iter()
        .map(|artifact| artifact["artifact_id"].as_str().expect("artifact id"))
        .collect();
    let generated_modules: Vec<_> = artifacts
        .iter()
        .map(|artifact| {
            artifact["generated_lean_module"]
                .as_str()
                .expect("generated Lean module")
        })
        .collect();

    assert_eq!(
        artifact_ids,
        vec![
            "matrix-decimal-readable-22d-seed0",
            "matrix-decimal-classic-22d-seed0",
            "matrix-decimal-breathing-22d-seed0",
            "matrix-base6-compact-18d-seed0",
            "matrix-base12-compact-18d-seed0",
            "matrix-base30-wheel-18d-seed0",
            "matrix-decimal-readable-64d-seed0",
            "matrix-decimal-classic-64d-seed0",
            "matrix-decimal-breathing-64d-seed0",
            "matrix-base6-compact-64d-seed0",
            "matrix-base12-compact-64d-seed0",
            "matrix-base30-wheel-64d-seed0",
            "matrix-decimal-readable-96d-seed0",
            "matrix-decimal-classic-96d-seed0",
            "matrix-decimal-breathing-96d-seed0",
            "matrix-base6-compact-96d-seed0",
            "matrix-base12-compact-96d-seed0",
            "matrix-base30-wheel-96d-seed0",
        ]
    );
    assert_eq!(
        generated_modules,
        vec![
            "PrimeArithmetic.Generated.Witness.MatrixDecimalReadable22",
            "PrimeArithmetic.Generated.Witness.MatrixDecimalClassic22",
            "PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing22",
            "PrimeArithmetic.Generated.Witness.MatrixBase6Compact18",
            "PrimeArithmetic.Generated.Witness.MatrixBase12Compact18",
            "PrimeArithmetic.Generated.Witness.MatrixBase30Wheel18",
            "PrimeArithmetic.Generated.Witness.MatrixDecimalReadable64",
            "PrimeArithmetic.Generated.Witness.MatrixDecimalClassic64",
            "PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing64",
            "PrimeArithmetic.Generated.Witness.MatrixBase6Compact64",
            "PrimeArithmetic.Generated.Witness.MatrixBase12Compact64",
            "PrimeArithmetic.Generated.Witness.MatrixBase30Wheel64",
            "PrimeArithmetic.Generated.Witness.MatrixDecimalReadable96",
            "PrimeArithmetic.Generated.Witness.MatrixDecimalClassic96",
            "PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing96",
            "PrimeArithmetic.Generated.Witness.MatrixBase6Compact96",
            "PrimeArithmetic.Generated.Witness.MatrixBase12Compact96",
            "PrimeArithmetic.Generated.Witness.MatrixBase30Wheel96",
        ]
    );
    assert!(artifacts
        .iter()
        .all(|artifact| artifact["theorem_wrapper"].is_null()));
    assert_eq!(
        manifest, tracked_manifest,
        "exported policy-matrix Lean catalog manifest drifted from the tracked artifact"
    );

    let checks_output = Command::new(env!(
        "CARGO_BIN_EXE_export_proof_carrying_witness_lean_catalog_checks"
    ))
    .arg("--manifest")
    .arg(&manifest_path)
    .arg("--out")
    .arg(&checks_path)
    .arg("--shard-size")
    .arg("3")
    .arg("--module-prefix")
    .arg("PrimeArithmetic.Generated.Witness")
    .output()
    .expect("run policy-matrix Lean catalog checker exporter");

    assert!(
        checks_output.status.success(),
        "policy-matrix checker exporter failed: {}",
        String::from_utf8_lossy(&checks_output.stderr)
    );

    let umbrella = fs::read_to_string(&checks_path).expect("read generated matrix Lean umbrella");
    let tracked_umbrella = fs::read_to_string(
        "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixCatalogChecks.lean",
    )
    .expect("read tracked matrix Lean catalog check umbrella");
    let shard_names = [
        "MatrixCatalogChecksShard01.lean",
        "MatrixCatalogChecksShard02.lean",
        "MatrixCatalogChecksShard03.lean",
        "MatrixCatalogChecksShard04.lean",
        "MatrixCatalogChecksShard05.lean",
        "MatrixCatalogChecksShard06.lean",
    ];
    let mut shard_texts = Vec::new();
    for shard_name in shard_names {
        let shard_text =
            fs::read_to_string(checks_dir.join(shard_name)).expect("read generated matrix shard");
        let tracked_shard = fs::read_to_string(format!(
            "lean-proofs/PrimeArithmetic/Generated/Witness/{shard_name}"
        ))
        .expect("read tracked matrix shard");
        assert_eq!(
            shard_text, tracked_shard,
            "exported policy-matrix Lean catalog shard drifted: {shard_name}"
        );
        shard_texts.push(shard_text);
    }
    let checks = shard_texts.join("\n");
    let umbrella_imports: Vec<_> = umbrella
        .lines()
        .filter(|line| line.starts_with("import PrimeArithmetic.Generated.Witness.Matrix"))
        .collect();
    let generated_imports: Vec<_> = checks
        .lines()
        .filter(|line| line.starts_with("import PrimeArithmetic.Generated.Witness.Matrix"))
        .collect();

    assert_eq!(
        umbrella_imports,
        vec![
            "import PrimeArithmetic.Generated.Witness.MatrixCatalogChecksShard01",
            "import PrimeArithmetic.Generated.Witness.MatrixCatalogChecksShard02",
            "import PrimeArithmetic.Generated.Witness.MatrixCatalogChecksShard03",
            "import PrimeArithmetic.Generated.Witness.MatrixCatalogChecksShard04",
            "import PrimeArithmetic.Generated.Witness.MatrixCatalogChecksShard05",
            "import PrimeArithmetic.Generated.Witness.MatrixCatalogChecksShard06",
        ]
    );
    assert_eq!(
        generated_imports,
        vec![
            "import PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing22",
            "import PrimeArithmetic.Generated.Witness.MatrixDecimalClassic22",
            "import PrimeArithmetic.Generated.Witness.MatrixDecimalReadable22",
            "import PrimeArithmetic.Generated.Witness.MatrixBase12Compact18",
            "import PrimeArithmetic.Generated.Witness.MatrixBase30Wheel18",
            "import PrimeArithmetic.Generated.Witness.MatrixBase6Compact18",
            "import PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing64",
            "import PrimeArithmetic.Generated.Witness.MatrixDecimalClassic64",
            "import PrimeArithmetic.Generated.Witness.MatrixDecimalReadable64",
            "import PrimeArithmetic.Generated.Witness.MatrixBase12Compact64",
            "import PrimeArithmetic.Generated.Witness.MatrixBase30Wheel64",
            "import PrimeArithmetic.Generated.Witness.MatrixBase6Compact64",
            "import PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing96",
            "import PrimeArithmetic.Generated.Witness.MatrixDecimalClassic96",
            "import PrimeArithmetic.Generated.Witness.MatrixDecimalReadable96",
            "import PrimeArithmetic.Generated.Witness.MatrixBase12Compact96",
            "import PrimeArithmetic.Generated.Witness.MatrixBase30Wheel96",
            "import PrimeArithmetic.Generated.Witness.MatrixBase6Compact96",
        ]
    );
    assert!(checks.contains(
        "  have _ := @PrimeArithmetic.Generated.Witness.MatrixDecimalReadable22.searchReplayFirstAcceptedSurvivor"
    ));
    assert!(checks.contains(
        "  have _ := @PrimeArithmetic.Generated.Witness.MatrixDecimalReadable64.searchReplayFirstAcceptedSurvivor"
    ));
    assert!(checks.contains(
        "  have _ := @PrimeArithmetic.Generated.Witness.MatrixDecimalReadable96.searchReplayFirstAcceptedSurvivor"
    ));
    assert!(checks.contains(
        "  have _ := @PrimeArithmetic.Generated.Witness.MatrixBase30Wheel64.searchReplayFirstAcceptedSurvivor"
    ));
    assert!(checks.contains(
        "  have _ := @PrimeArithmetic.Generated.Witness.MatrixBase30Wheel96.searchReplayFirstAcceptedSurvivor"
    ));
    assert!(checks.contains(
        "  have _ := @PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing64.searchReplayFirstAcceptedSurvivor"
    ));
    assert!(checks.contains(
        "  have _ := @PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing96.searchReplayFirstAcceptedSurvivor"
    ));
    assert!(checks.contains(
        "  have _ := @PrimeArithmetic.Generated.Witness.MatrixBase12Compact64.searchReplayFirstAcceptedSurvivor"
    ));
    assert!(checks.contains(
        "  have _ := @PrimeArithmetic.Generated.Witness.MatrixBase12Compact96.searchReplayFirstAcceptedSurvivor"
    ));
    assert!(checks.contains(
        "  have _ := @PrimeArithmetic.Generated.Witness.MatrixBase6Compact64.searchReplayFirstAcceptedSurvivor"
    ));
    assert!(checks.contains(
        "  have _ := @PrimeArithmetic.Generated.Witness.MatrixBase6Compact96.searchReplayFirstAcceptedSurvivor"
    ));
    assert!(!checks
        .contains("PrimeArithmetic.Generated.Witness.MatrixBase30Wheel64.search_replay_seed"));
    assert!(!checks
        .contains("PrimeArithmetic.Generated.Witness.MatrixBase30Wheel96.search_replay_seed"));
    assert!(!checks
        .contains("PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing64.search_replay_seed"));
    assert!(!checks
        .contains("PrimeArithmetic.Generated.Witness.MatrixDecimalBreathing96.search_replay_seed"));
    assert!(!checks
        .contains("PrimeArithmetic.Generated.Witness.MatrixBase12Compact64.search_replay_seed"));
    assert!(!checks
        .contains("PrimeArithmetic.Generated.Witness.MatrixBase12Compact96.search_replay_seed"));
    assert!(!checks
        .contains("PrimeArithmetic.Generated.Witness.MatrixBase6Compact64.search_replay_seed"));
    assert!(!checks
        .contains("PrimeArithmetic.Generated.Witness.MatrixBase6Compact96.search_replay_seed"));
    assert!(!checks
        .contains("PrimeArithmetic.Generated.Witness.MatrixDecimalReadable64.search_replay_seed"));
    assert!(!checks
        .contains("PrimeArithmetic.Generated.Witness.MatrixDecimalReadable96.search_replay_seed"));
    assert!(!checks
        .contains("PrimeArithmetic.Generated.Witness.MatrixDecimalClassic96.search_replay_seed"));
    assert_eq!(checks.matches("example : True := by").count(), 1038);
    assert_eq!(
        umbrella, tracked_umbrella,
        "exported policy-matrix Lean catalog check umbrella drifted from the tracked artifact"
    );

    fs::remove_file(&manifest_path).expect("remove exported matrix Lean manifest fixture");
    fs::remove_dir_all(&checks_dir).expect("remove exported matrix Lean checks fixture");
    fs::remove_dir_all(&out_dir).expect("remove temporary generated Lean output dir");
}

#[test]
fn witness_lean_certificate_exporter_matches_tracked_modules() {
    for (label, certificate, lean_path) in [
        (
            "Seed60",
            "docs/witness/seed60_proof_carrying_witness.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/Seed60.lean",
        ),
        (
            "Teaching38",
            "docs/witness/teaching38_proof_carrying_witness.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/Teaching38.lean",
        ),
        (
            "TimestampPolicy29Trial0",
            "docs/witness/timestamp_policy_29d_trial0_proof_carrying_witness.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/TimestampPolicy29Trial0.lean",
        ),
        (
            "MatrixDecimalReadable22",
            "docs/witness/policy_matrix/matrix_decimal_readable_22d_seed0.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixDecimalReadable22.lean",
        ),
        (
            "MatrixDecimalClassic22",
            "docs/witness/policy_matrix/matrix_decimal_classic_22d_seed0.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixDecimalClassic22.lean",
        ),
        (
            "MatrixDecimalBreathing22",
            "docs/witness/policy_matrix/matrix_decimal_breathing_22d_seed0.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixDecimalBreathing22.lean",
        ),
        (
            "MatrixDecimalReadable64",
            "docs/witness/policy_matrix/matrix_decimal_readable_64d_seed0.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixDecimalReadable64.lean",
        ),
        (
            "MatrixDecimalReadable96",
            "docs/witness/policy_matrix/matrix_decimal_readable_96d_seed0.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixDecimalReadable96.lean",
        ),
        (
            "MatrixDecimalClassic64",
            "docs/witness/policy_matrix/matrix_decimal_classic_64d_seed0.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixDecimalClassic64.lean",
        ),
        (
            "MatrixDecimalBreathing64",
            "docs/witness/policy_matrix/matrix_decimal_breathing_64d_seed0.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixDecimalBreathing64.lean",
        ),
        (
            "MatrixDecimalBreathing96",
            "docs/witness/policy_matrix/matrix_decimal_breathing_96d_seed0.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixDecimalBreathing96.lean",
        ),
        (
            "MatrixBase30Wheel64",
            "docs/witness/policy_matrix/matrix_base30_wheel_64d_seed0.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixBase30Wheel64.lean",
        ),
        (
            "MatrixBase30Wheel96",
            "docs/witness/policy_matrix/matrix_base30_wheel_96d_seed0.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixBase30Wheel96.lean",
        ),
        (
            "MatrixBase12Compact64",
            "docs/witness/policy_matrix/matrix_base12_compact_64d_seed0.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixBase12Compact64.lean",
        ),
        (
            "MatrixBase12Compact96",
            "docs/witness/policy_matrix/matrix_base12_compact_96d_seed0.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixBase12Compact96.lean",
        ),
        (
            "MatrixBase6Compact64",
            "docs/witness/policy_matrix/matrix_base6_compact_64d_seed0.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixBase6Compact64.lean",
        ),
        (
            "MatrixBase6Compact96",
            "docs/witness/policy_matrix/matrix_base6_compact_96d_seed0.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixBase6Compact96.lean",
        ),
        (
            "MatrixBase6Compact18",
            "docs/witness/policy_matrix/matrix_base6_compact_18d_seed0.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixBase6Compact18.lean",
        ),
        (
            "MatrixBase12Compact18",
            "docs/witness/policy_matrix/matrix_base12_compact_18d_seed0.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixBase12Compact18.lean",
        ),
        (
            "MatrixBase30Wheel18",
            "docs/witness/policy_matrix/matrix_base30_wheel_18d_seed0.json",
            "lean-proofs/PrimeArithmetic/Generated/Witness/MatrixBase30Wheel18.lean",
        ),
    ] {
        let lean_path = PathBuf::from(lean_path);
        let original_lean = fs::read_to_string(&lean_path)
            .unwrap_or_else(|err| panic!("read tracked {label} generated Lean module: {err}"));

        let output = Command::new(env!(
            "CARGO_BIN_EXE_export_proof_carrying_witness_lean_certificate"
        ))
        .arg("--certificate")
        .arg(certificate)
        .arg("--out")
        .arg(&lean_path)
        .output()
        .unwrap_or_else(|err| panic!("run witness Lean certificate exporter for {label}: {err}"));

        let regenerated_lean = fs::read_to_string(&lean_path)
            .unwrap_or_else(|err| panic!("read regenerated {label} generated Lean module: {err}"));
        if regenerated_lean != original_lean {
            fs::write(&lean_path, &original_lean).unwrap_or_else(|err| {
                panic!("restore tracked {label} generated Lean module: {err}")
            });
        }

        assert!(
            output.status.success(),
            "{label} Lean exporter failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            regenerated_lean, original_lean,
            "{label} generated Lean module drifted from exporter output"
        );
    }
}

fn file_mtime(path: &PathBuf) -> SystemTime {
    fs::metadata(path)
        .unwrap_or_else(|err| panic!("read metadata for {}: {err}", path.display()))
        .modified()
        .unwrap_or_else(|err| panic!("read mtime for {}: {err}", path.display()))
}

fn unique_temp_path(label: &str, extension: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{label}-{}-{nanos}.{extension}", process::id()))
}

fn unique_temp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{label}-{}-{nanos}", process::id()))
}

fn unique_generated_lean_dir(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_nanos();
    PathBuf::from("lean-proofs")
        .join("PrimeArithmetic")
        .join("Generated")
        .join(format!("{prefix}{}{}", process::id(), nanos))
}
