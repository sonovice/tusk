use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.join("../../..");
    let rng_path = workspace_root.join("specs/mei/validation/mei-all.rng");
    let mei_src = manifest_dir.join("src");

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

    tusk_mei_codegen::generator::generate_mei_attr_impls(&defs, &mei_src)
        .expect("Failed to generate MEI attribute trait impls");

    tusk_mei_codegen::generator::generate_mei_element_ser_impls(&defs, &mei_src)
        .expect("Failed to generate MEI element serializer impls");

    tusk_mei_codegen::generator::generate_mei_element_deser_impls(&defs, &mei_src)
        .expect("Failed to generate MEI element deserializer impls");
}
