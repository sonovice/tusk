//! Generates versioned MusicXML model types from each version's XSD schema.
//!
//! Versions 2.0–4.0 have musicxml.xsd in specs/musicxml/versions/musicxml-X.Y/schema/.
//! Output is written to src/versions/vX_Y/ (data.rs, mod.rs).

use std::path::PathBuf;

const VERSIONED_XSD: &[(&str, &str)] = &[
    ("v2_0", "musicxml-2.0/schema/musicxml.xsd"),
    ("v3_0", "musicxml-3.0/schema/musicxml.xsd"),
    ("v3_1", "musicxml-3.1/schema/musicxml.xsd"),
    ("v4_0", "musicxml-4.0/schema/musicxml.xsd"),
];

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.join("../../..");
    let versions_dir = workspace_root.join("specs/musicxml/versions");
    let musicxml_src = manifest_dir.join("src/versions");

    println!("cargo::rerun-if-changed=build.rs");

    for &(label, rel_path) in VERSIONED_XSD {
        let xsd_path = versions_dir.join(rel_path);
        println!("cargo::rerun-if-changed={}", xsd_path.display());
        if !xsd_path.exists() {
            println!(
                "cargo::warning=tusk-musicxml: skipping versioned model '{}': XSD not found at {}",
                label,
                xsd_path.display()
            );
            continue;
        }

        let schema = match tusk_musicxml_codegen::parse_xsd(&xsd_path) {
            Ok(s) => s,
            Err(e) => {
                println!(
                    "cargo::warning=tusk-musicxml: failed to parse XSD for '{}': {}",
                    label, e
                );
                continue;
            }
        };

        let output = musicxml_src.join(label);
        if let Err(e) = tusk_musicxml_codegen::generate(&schema, &output) {
            println!(
                "cargo::warning=tusk-musicxml: failed to generate '{}': {}",
                label, e
            );
        }
    }
}
