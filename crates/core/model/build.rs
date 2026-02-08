use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.join("../../..");
    let rng_path = workspace_root.join("specs/mei/validation/mei-all.rng");
    let output = manifest_dir.join("src/generated");

    // Only re-run when the RNG spec or this build script changes
    println!("cargo::rerun-if-changed={}", rng_path.display());
    println!("cargo::rerun-if-changed=build.rs");

    if !rng_path.exists() {
        panic!(
            "MEI RNG specification not found at '{}'.\n\
             Place the MEI spec files under specs/mei/validation/ to enable code generation.",
            rng_path.display()
        );
    }

    let defs = tusk_mei_codegen::rng::parse_rng_file(&rng_path)
        .expect("Failed to parse MEI RNG specification");

    tusk_mei_codegen::generator::generate_all(&defs, &output)
        .expect("Failed to generate model code");
}
