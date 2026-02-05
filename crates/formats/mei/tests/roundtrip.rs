//! MEI roundtrip tests against official sample encodings.
//!
//! Tests parse MEI files from `specs/mei/sample-encodings/MEI_5.1/Music/`,
//! serialize back to MEI XML, and verify that content is preserved.
//!
//! # Workflow
//!
//! The roundtrip flow is: MEI file → Internal Structures → MEI XML
//!
//! 1. Read original MEI file
//! 2. Parse to internal model: `mei::import(xml_str)` → `tusk_model::elements::Mei`
//! 3. Serialize back to MEI: `mei::export(&mei)` → XML string
//! 4. Compare: tree-based XML comparison with semantic equivalence
//!
//! # Acceptable Differences
//!
//! - Whitespace/formatting differences
//! - Attribute ordering
//! - Default values explicitly stated vs. omitted
//! - Namespace prefix differences (e.g., `mei:note` vs `note`)
//! - XML declaration differences
//! - xmlns declarations (not semantic content)

use std::path::{Path, PathBuf};
use tusk_mei::xml_compare::{CompareError, compare_xml, get_differences};
use tusk_mei::{export, import};

/// Find the workspace root by looking for Cargo.toml with [workspace] section.
fn find_workspace_root() -> Result<PathBuf, String> {
    let mut dir = std::env::current_dir().map_err(|e| e.to_string())?;
    loop {
        let manifest = dir.join("Cargo.toml");
        if manifest.exists() {
            let content = std::fs::read_to_string(&manifest).map_err(|e| e.to_string())?;
            if content.contains("[workspace]") {
                return Ok(dir);
            }
        }
        if !dir.pop() {
            return Err("Could not find workspace root".to_string());
        }
    }
}

/// Get path to MEI 5.1 sample encodings Music directory.
fn sample_encodings_music_dir() -> PathBuf {
    find_workspace_root()
        .expect("find workspace root")
        .join("specs/mei/sample-encodings/MEI_5.1/Music")
}

/// Perform roundtrip test on an MEI file.
///
/// Returns Ok(()) if roundtrip succeeds, Err with details if it fails.
///
/// The comparison is tree-based, handling acceptable differences like
/// attribute ordering, whitespace, and namespace prefixes.
pub fn roundtrip_mei_file(path: &Path) -> Result<(), String> {
    // Read original file
    let original_xml =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    // Parse to internal model
    let mei = import(&original_xml).map_err(|e| format!("Failed to parse MEI: {}", e))?;

    // Serialize back to XML
    let roundtripped_xml = export(&mei).map_err(|e| format!("Failed to serialize MEI: {}", e))?;

    // Re-parse the roundtripped XML to verify it's valid
    let _reparsed = import(&roundtripped_xml).map_err(|e| {
        format!(
            "Failed to re-parse roundtripped MEI: {}\n\nRoundtripped XML:\n{}",
            e,
            &roundtripped_xml[..roundtripped_xml.len().min(2000)]
        )
    })?;

    // Tree-based comparison handles acceptable differences:
    // - Attribute ordering
    // - Whitespace/formatting
    // - Namespace prefixes
    // - XML declarations
    // - xmlns declarations
    compare_xml(&original_xml, &roundtripped_xml).map_err(|e| match e {
        CompareError::ParseError(msg) => format!("XML parse error during comparison: {}", msg),
        CompareError::Differences(diffs) => {
            let mut msg = format!("Roundtrip found {} differences:\n", diffs.len());
            for (i, diff) in diffs.iter().take(20).enumerate() {
                msg.push_str(&format!(
                    "  {}. at {}: {}\n",
                    i + 1,
                    diff.path,
                    diff.description
                ));
            }
            if diffs.len() > 20 {
                msg.push_str(&format!("  ... and {} more\n", diffs.len() - 20));
            }
            msg
        }
    })
}

/// Detailed comparison that reports all differences between original and roundtripped MEI.
///
/// Uses tree-based comparison to handle acceptable differences like attribute
/// ordering, whitespace, and namespace prefixes.
///
/// Returns a list of difference descriptions (empty if documents match).
pub fn compare_mei_detailed(path: &Path) -> Result<Vec<String>, String> {
    let original_xml =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    let mei = import(&original_xml).map_err(|e| format!("Failed to parse MEI: {}", e))?;

    let roundtripped_xml = export(&mei).map_err(|e| format!("Failed to serialize MEI: {}", e))?;

    // Use tree-based comparison for semantic equivalence
    match get_differences(&original_xml, &roundtripped_xml) {
        Ok(diffs) => Ok(diffs
            .into_iter()
            .map(|d| format!("at {}: {}", d.path, d.description))
            .collect()),
        Err(CompareError::ParseError(msg)) => Err(format!("XML parse error: {}", msg)),
        Err(CompareError::Differences(diffs)) => Ok(diffs
            .into_iter()
            .map(|d| format!("at {}: {}", d.path, d.description))
            .collect()),
    }
}

// ============================================================================
// Infrastructure Test
// ============================================================================

#[test]
fn test_harness_can_find_workspace_root() {
    let root = find_workspace_root().expect("should find workspace root");
    assert!(
        root.join("Cargo.toml").exists(),
        "workspace root should have Cargo.toml"
    );
    assert!(
        root.join("crates").exists(),
        "workspace root should have crates directory"
    );
}

#[test]
fn test_harness_can_find_sample_encodings() {
    let music_dir = sample_encodings_music_dir();
    assert!(
        music_dir.exists(),
        "sample encodings Music directory should exist: {:?}",
        music_dir
    );
    assert!(
        music_dir.join("Complete_examples").exists(),
        "Complete_examples should exist"
    );
}

#[test]
fn test_roundtrip_helper_works_on_simple_mei() {
    // Create a minimal valid MEI document
    // Note: meiversion is omitted as the model is generated from 6.0-dev ODD spec
    // and doesn't support 5.1 as a valid value
    let simple_mei = r#"<?xml version="1.0" encoding="UTF-8"?>
<mei xmlns="http://www.music-encoding.org/ns/mei">
  <meiHead>
    <fileDesc>
      <titleStmt>
        <title>Test</title>
      </titleStmt>
      <pubStmt/>
    </fileDesc>
  </meiHead>
  <music/>
</mei>"#;

    // Write to temp file
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let temp_file = temp_dir.path().join("test.mei");
    std::fs::write(&temp_file, simple_mei).expect("write temp file");

    // Test roundtrip
    let result = roundtrip_mei_file(&temp_file);
    assert!(result.is_ok(), "Simple MEI roundtrip failed: {:?}", result);
}

// ============================================================================
// Complete Examples Roundtrip Tests
// ============================================================================

#[test]
fn test_roundtrip_aguado_walzer_g_major() {
    let path = sample_encodings_music_dir().join("Complete_examples/Aguado_Walzer_G-major.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Aguado_Walzer_G-major.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_ahle_jesu_meines_herzens_freud() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Ahle_Jesu_meines_Herzens_Freud.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Ahle_Jesu_meines_Herzens_Freud.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_altenburg_concerto_c_major() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Altenburg_Concerto_C-major.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Altenburg_Concerto_C-major.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_altenburg_ein_feste_burg() {
    let path = sample_encodings_music_dir().join("Complete_examples/Altenburg_Ein_feste_Burg.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Altenburg_Ein_feste_Burg.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_altenburg_macht_auf_die_tor() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Altenburg_Macht_auf_die_Tor.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Altenburg_Macht_auf_die_Tor.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_bach_jc_fughette_no2() {
    let path = sample_encodings_music_dir().join("Complete_examples/Bach-JC_Fughette_No2.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Bach-JC_Fughette_No2.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_bach_jc_fughette_for_brass_quartet_g_major() {
    let path = sample_encodings_music_dir()
        .join("Complete_examples/Bach-JC_Fughette_for_BrassQuartet_G-major.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Bach-JC_Fughette_for_BrassQuartet_G-major.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_bach_js_brandenburg_concerto_no2_i_bwv1047() {
    let path = sample_encodings_music_dir()
        .join("Complete_examples/Bach-JS_BrandenburgConcert_No2_I_BWV1047.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Bach-JS_BrandenburgConcert_No2_I_BWV1047.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_bach_js_brandenburg_concerto_no2_ii_bwv1047() {
    let path = sample_encodings_music_dir()
        .join("Complete_examples/Bach-JS_BrandenburgConcert_No2_II_BWV1047.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Bach-JS_BrandenburgConcert_No2_II_BWV1047.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_bach_js_brandenburg_concerto_no2_iii_bwv1047() {
    let path = sample_encodings_music_dir()
        .join("Complete_examples/Bach-JS_BrandenburgConcert_No2_III_BWV1047.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Bach-JS_BrandenburgConcert_No2_III_BWV1047.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_bach_js_brandenburg_concerto_no4_i_bwv1049() {
    let path = sample_encodings_music_dir()
        .join("Complete_examples/Bach-JS_BrandenburgConcert_No4_I_BWV1049.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Bach-JS_BrandenburgConcert_No4_I_BWV1049.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_bach_js_brandenburg_concerto_no4_ii_bwv1049() {
    let path = sample_encodings_music_dir()
        .join("Complete_examples/Bach-JS_BrandenburgConcert_No4_II_BWV1049.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Bach-JS_BrandenburgConcert_No4_II_BWV1049.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_bach_js_ein_feste_burg() {
    let path = sample_encodings_music_dir().join("Complete_examples/Bach-JS_Ein_feste_Burg.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Bach-JS_Ein_feste_Burg.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_bach_js_herzliebster_jesu_bwv244_46() {
    let path = sample_encodings_music_dir()
        .join("Complete_examples/Bach-JS_Herzliebster_Jesu_BWV244-46.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Bach-JS_Herzliebster_Jesu_BWV244-46.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_bach_js_hilf_herr_jesu_bwv344() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Bach-JS_Hilf_Herr_Jesu_BWV344.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Bach-JS_Hilf_Herr_Jesu_BWV344.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}
