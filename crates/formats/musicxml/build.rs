//! Generates versioned MusicXML model types from each version's XSD schema.
//!
//! Schemas live under codegen/schema/versions/musicxml-X.Y/schema/ (2.0–4.1).
//! Output is written to src/versions/vX_Y/.

use std::path::PathBuf;

const VERSIONED_XSD: &[(&str, &str)] = &[
    ("v2_0", "musicxml-2.0/schema/musicxml.xsd"),
    ("v3_0", "musicxml-3.0/schema/musicxml.xsd"),
    ("v3_1", "musicxml-3.1/schema/musicxml.xsd"),
    ("v4_0", "musicxml-4.0/schema/musicxml.xsd"),
    ("v4_1", "musicxml-4.1/schema/musicxml.xsd"),
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
    let versions_dir = workspace_root.join("crates/formats/musicxml/codegen/schema/versions");
    let musicxml_src = manifest_dir.join("src/versions");
    let build_rs = manifest_dir.join("build.rs");
    let stamp = musicxml_src.join(".codegen-stamp");

    println!("cargo::rerun-if-changed=build.rs");

    let mut inputs = vec![build_rs];
    for &(_, rel_path) in VERSIONED_XSD {
        let xsd_path = versions_dir.join(rel_path);
        println!("cargo::rerun-if-changed={}", xsd_path.display());
        inputs.push(xsd_path);
    }

    if skip_if_up_to_date(&inputs, &stamp) {
        return;
    }

    for &(label, rel_path) in VERSIONED_XSD {
        let xsd_path = versions_dir.join(rel_path);
        if !xsd_path.exists() {
            // Only warn if the generated output is also missing (codegen truly needed).
            // On CI / fresh checkouts the XSDs are absent but generated code is committed.
            if !musicxml_src.join(label).exists() {
                println!(
                    "cargo::warning=tusk-musicxml: skipping versioned model '{}': XSD not found at {}",
                    label,
                    xsd_path.display()
                );
            }
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

    let _ = std::fs::File::create(&stamp);
}
