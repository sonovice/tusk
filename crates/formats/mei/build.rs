use std::path::PathBuf;

/// MEI version labels and their corresponding RNG file names under codegen/schema/versions/.
const VERSIONED_MODELS: &[(&str, &str)] = &[
    ("v2_1_1", "mei-all_v2.1.1.rng"),
    ("v3_0_0", "mei-all_v3.0.0.rng"),
    ("v4_0_1", "mei-all_v4.0.1.rng"),
    ("v5_0", "mei-all_v.5.0.rng"),
    ("v5_1", "mei-all_v5.1.rng"),
    ("v6_0_dev", "mei-all_v6.0-dev.rng"),
];

/// Skip codegen if stamp file exists and is newer than all input paths.
fn skip_if_up_to_date(inputs: &[PathBuf], stamp: &PathBuf) -> bool {
    let stamp_meta = match std::fs::metadata(stamp) {
        Ok(m) => m,
        Err(_) => return false,
    };
    let stamp_mtime = match stamp_meta.modified() {
        Ok(t) => t,
        Err(_) => return false,
    };
    for p in inputs {
        let m = match std::fs::metadata(p) {
            Ok(m) => m,
            Err(_) => return false,
        };
        let t = match m.modified() {
            Ok(t) => t,
            Err(_) => return false,
        };
        if t > stamp_mtime {
            return false;
        }
    }
    true
}

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.join("../../..");
    let versions_dir = workspace_root.join("crates/formats/mei/codegen/schema/versions");
    let rng_path = versions_dir.join("mei-all_v6.0-dev.rng");
    let mei_src = manifest_dir.join("src");
    let build_rs = manifest_dir.join("build.rs");
    let stamp = mei_src.join(".codegen-stamp");

    // Only re-run when spec files or this build script change
    println!("cargo::rerun-if-changed={}", rng_path.display());
    println!("cargo::rerun-if-changed={}", versions_dir.display());
    println!("cargo::rerun-if-changed=build.rs");

    if !rng_path.exists() {
        panic!(
            "MEI RNG specification not found at '{}'.\n\
             Schemas live under crates/formats/mei/codegen/schema/versions/.",
            rng_path.display()
        );
    }

    let mut inputs = vec![rng_path.clone(), build_rs];
    for &(_, rng_file) in VERSIONED_MODELS {
        inputs.push(versions_dir.join(rng_file));
    }
    if skip_if_up_to_date(&inputs, &stamp) {
        return;
    }

    // Generate trait impls from the main (6.0-dev) RNG spec
    let defs = tusk_mei_codegen::rng::parse_rng_file(&rng_path)
        .expect("Failed to parse MEI RNG specification");

    tusk_mei_codegen::generator::generate_mei_attr_impls(&defs, &mei_src)
        .expect("Failed to generate MEI attribute trait impls");

    tusk_mei_codegen::generator::generate_mei_element_ser_impls(&defs, &mei_src)
        .expect("Failed to generate MEI element serializer impls");

    tusk_mei_codegen::generator::generate_mei_element_deser_impls(&defs, &mei_src)
        .expect("Failed to generate MEI element deserializer impls");

    // Generate versioned import models from version-specific RNG specs
    for &(label, rng_file) in VERSIONED_MODELS {
        let version_rng = versions_dir.join(rng_file);
        if !version_rng.exists() {
            println!(
                "cargo::warning=Skipping versioned model '{}': RNG file not found at '{}'",
                label,
                version_rng.display()
            );
            continue;
        }

        let version_defs = tusk_mei_codegen::rng::parse_rng_file(&version_rng)
            .unwrap_or_else(|e| panic!("Failed to parse RNG for '{}': {}", label, e));

        let output = mei_src.join("versions").join(label);
        let module_path = format!("crate::versions::{}", label);
        let config = tusk_mei_codegen::generator::CodegenConfig { module_path };

        tusk_mei_codegen::generator::generate_all_with_config(&version_defs, &output, &config)
            .unwrap_or_else(|e| panic!("Failed to generate versioned model '{}': {}", label, e));
    }

    let _ = std::fs::File::create(&stamp);
}
