use std::path::PathBuf;

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
    let rng_path = workspace_root.join("crates/formats/mei/codegen/schema/versions/mei-all_v6.0-dev.rng");
    let output = manifest_dir.join("src/generated");
    let build_rs = manifest_dir.join("build.rs");
    let stamp = output.join(".codegen-stamp");

    // Only re-run when the RNG spec or this build script changes
    println!("cargo::rerun-if-changed={}", rng_path.display());
    println!("cargo::rerun-if-changed=build.rs");

    if !rng_path.exists() {
        panic!(
            "MEI RNG specification not found at '{}'.\n\
             Schemas live under crates/formats/mei/codegen/schema/versions/.",
            rng_path.display()
        );
    }

    if skip_if_up_to_date(&[rng_path.clone(), build_rs], &stamp) {
        return;
    }

    let defs = tusk_mei_codegen::rng::parse_rng_file(&rng_path)
        .expect("Failed to parse MEI RNG specification");

    tusk_mei_codegen::generator::generate_all(&defs, &output)
        .expect("Failed to generate model code");

    let _ = std::fs::File::create(&stamp);
}
