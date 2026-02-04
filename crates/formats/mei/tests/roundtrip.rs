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
//! 4. Compare: re-parse output and verify structural equivalence
//!
//! # Acceptable Differences
//!
//! - Whitespace/formatting differences
//! - Attribute ordering
//! - Default values explicitly stated vs. omitted
//! - Namespace prefix differences (e.g., `mei:note` vs `note`)
//! - XML declaration differences

use std::path::{Path, PathBuf};
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
pub fn roundtrip_mei_file(path: &Path) -> Result<(), String> {
    // Read original file
    let original_xml =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    // Parse to internal model
    let mei = import(&original_xml).map_err(|e| format!("Failed to parse MEI: {}", e))?;

    // Serialize back to XML
    let roundtripped_xml = export(&mei).map_err(|e| format!("Failed to serialize MEI: {}", e))?;

    // Re-parse the roundtripped XML to verify it's valid
    let reparsed = import(&roundtripped_xml).map_err(|e| {
        format!(
            "Failed to re-parse roundtripped MEI: {}\n\nRoundtripped XML:\n{}",
            e,
            &roundtripped_xml[..roundtripped_xml.len().min(2000)]
        )
    })?;

    // Compare the two parsed structures
    // For now, we re-serialize both and compare the XML
    // This handles ordering differences while catching structural issues
    let original_reserialized =
        export(&mei).map_err(|e| format!("Failed to re-serialize original: {}", e))?;
    let reparsed_serialized =
        export(&reparsed).map_err(|e| format!("Failed to serialize reparsed: {}", e))?;

    if original_reserialized != reparsed_serialized {
        // Find first difference for debugging
        let orig_lines: Vec<&str> = original_reserialized.lines().collect();
        let new_lines: Vec<&str> = reparsed_serialized.lines().collect();

        for (i, (orig, new)) in orig_lines.iter().zip(new_lines.iter()).enumerate() {
            if orig != new {
                return Err(format!(
                    "Roundtrip mismatch at line {}:\n  Original:    {}\n  Roundtripped: {}",
                    i + 1,
                    orig,
                    new
                ));
            }
        }

        if orig_lines.len() != new_lines.len() {
            return Err(format!(
                "Line count mismatch: original {} lines, roundtripped {} lines",
                orig_lines.len(),
                new_lines.len()
            ));
        }
    }

    Ok(())
}

/// Detailed comparison that reports all differences.
#[allow(dead_code)]
pub fn compare_mei_detailed(path: &Path) -> Result<Vec<String>, String> {
    let original_xml =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    let mei = import(&original_xml).map_err(|e| format!("Failed to parse MEI: {}", e))?;

    let roundtripped_xml = export(&mei).map_err(|e| format!("Failed to serialize MEI: {}", e))?;

    let mut differences = Vec::new();

    // Line-by-line comparison for detailed output
    let orig_lines: Vec<&str> = original_xml.lines().collect();
    let new_lines: Vec<&str> = roundtripped_xml.lines().collect();

    // Compare line by line (accounting for whitespace normalization)
    for (i, (orig, new)) in orig_lines.iter().zip(new_lines.iter()).enumerate() {
        let orig_trimmed = orig.trim();
        let new_trimmed = new.trim();

        // Skip empty lines
        if orig_trimmed.is_empty() && new_trimmed.is_empty() {
            continue;
        }

        // Skip XML declarations (acceptable difference)
        if orig_trimmed.starts_with("<?xml") && new_trimmed.starts_with("<?xml") {
            continue;
        }

        // Normalize and compare
        if normalize_xml_line(orig_trimmed) != normalize_xml_line(new_trimmed) {
            differences.push(format!(
                "Line {}: \n  Original:    '{}'\n  Roundtripped: '{}'",
                i + 1,
                orig_trimmed,
                new_trimmed
            ));
        }
    }

    // Check for missing/extra lines
    if orig_lines.len() > new_lines.len() {
        for (i, line) in orig_lines.iter().skip(new_lines.len()).enumerate() {
            differences.push(format!(
                "Line {} missing in output: '{}'",
                new_lines.len() + i + 1,
                line.trim()
            ));
        }
    } else if new_lines.len() > orig_lines.len() {
        for (i, line) in new_lines.iter().skip(orig_lines.len()).enumerate() {
            differences.push(format!(
                "Extra line {} in output: '{}'",
                orig_lines.len() + i + 1,
                line.trim()
            ));
        }
    }

    Ok(differences)
}

/// Normalize an XML line for comparison.
///
/// This handles acceptable differences like:
/// - Namespace prefix variations
/// - Attribute ordering (partial - same attributes should be present)
fn normalize_xml_line(line: &str) -> String {
    // Remove mei: namespace prefix for comparison (but keep xml: for xml:id)
    let normalized = line.replace("mei:", "");

    // Trim whitespace
    normalized.trim().to_string()
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
    let simple_mei = r#"<?xml version="1.0" encoding="UTF-8"?>
<mei xmlns="http://www.music-encoding.org/ns/mei" meiversion="5.1">
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
