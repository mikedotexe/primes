use std::{env, path::PathBuf, process::Command};

fn main() {
    // Only compile Metal shaders when metal feature is enabled on macOS
    let has_metal_feature = env::var("CARGO_FEATURE_METAL").is_ok();
    let is_macos = cfg!(target_os = "macos");

    if has_metal_feature && is_macos {
        println!("cargo:rerun-if-changed=shaders/sieve_affine.metal");
        println!("cargo:rerun-if-changed=shaders/sieve_optimized.metal");

        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let metallib_output = out_dir.join("membrane_prime.metallib");

        // Check if shaders directory exists
        let shader_path = PathBuf::from("shaders/sieve_affine.metal");
        if !shader_path.exists() {
            // Use the Metal files in src/metal directory as fallback
            let metal_files = vec!["src/metal/membrane_affine_sieve.metal"];
            let mut air_files = Vec::new();

            for metal_file in metal_files {
                let file_stem = PathBuf::from(metal_file)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                let air_output = out_dir.join(format!("{file_stem}.air"));

                let metal_status = Command::new("xcrun")
                    .args(["-sdk", "macosx", "metal", "-c", "-o"])
                    .arg(&air_output)
                    .arg(metal_file)
                    .status()
                    .expect("Failed to compile Metal shader");

                if !metal_status.success() {
                    panic!("Metal compilation failed for {metal_file}");
                }
                air_files.push(air_output);
            }

            // Link all .air files to .metallib
            let mut metallib_cmd = Command::new("xcrun");
            metallib_cmd.args(["-sdk", "macosx", "metallib", "-o"]);
            metallib_cmd.arg(&metallib_output);
            for air_file in air_files {
                metallib_cmd.arg(air_file);
            }

            let metallib_status = metallib_cmd.status().expect("Failed to create metallib");

            if !metallib_status.success() {
                panic!("Metallib creation failed");
            }
        } else {
            // Original path with shaders directory
            let air_output = out_dir.join("sieve_affine.air");
            let metal_status = Command::new("xcrun")
                .args(["-sdk", "macosx", "metal", "-c", "-o"])
                .arg(&air_output)
                .arg("shaders/sieve_affine.metal")
                .status()
                .expect("Failed to compile Metal shader");

            if !metal_status.success() {
                panic!("Metal compilation failed");
            }

            // Link .air to .metallib
            let metallib_status = Command::new("xcrun")
                .args(["-sdk", "macosx", "metallib", "-o"])
                .arg(&metallib_output)
                .arg(&air_output)
                .status()
                .expect("Failed to create metallib");

            if !metallib_status.success() {
                panic!("Metallib creation failed");
            }
        }

        println!(
            "cargo:rustc-env=METALLIB_PATH={}",
            metallib_output.display()
        );

        // Compile optimized shader - temporarily disabled due to simd_ballot issue
        let metallib_optimized = out_dir.join("membrane_prime_optimized.metallib");
        // if PathBuf::from("shaders/sieve_optimized.metal").exists() {
        //     let air_optimized = out_dir.join("sieve_optimized.air");
        //     let metal_status = Command::new("xcrun")
        //         .args(&["-sdk", "macosx", "metal", "-c", "-o"])
        //         .arg(&air_optimized)
        //         .arg("shaders/sieve_optimized.metal")
        //         .status()
        //         .expect("Failed to compile optimized Metal shader");
        //
        //     if !metal_status.success() {
        //         panic!("Optimized Metal compilation failed");
        //     }
        //
        //     let metallib_status = Command::new("xcrun")
        //         .args(&["-sdk", "macosx", "metallib", "-o"])
        //         .arg(&metallib_optimized)
        //         .arg(&air_optimized)
        //         .status()
        //         .expect("Failed to create optimized metallib");
        //
        //     if !metallib_status.success() {
        //         panic!("Optimized Metallib creation failed");
        //     }
        // }

        // Only set optimized path if it exists
        if metallib_optimized.exists() {
            println!(
                "cargo:rustc-env=METALLIB_OPTIMIZED_PATH={}",
                metallib_optimized.display()
            );
        } else {
            // Set to regular metallib as fallback
            println!(
                "cargo:rustc-env=METALLIB_OPTIMIZED_PATH={}",
                metallib_output.display()
            );
        }
    }
}
