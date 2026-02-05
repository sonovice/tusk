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

#[test]
fn test_roundtrip_bach_js_musikalisches_opfer_trio_bwv1079() {
    let path = sample_encodings_music_dir()
        .join("Complete_examples/Bach-JS_Musikalisches_Opfer_Trio_BWV1079.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Bach-JS_Musikalisches_Opfer_Trio_BWV1079.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_bach_js_wie_bist_du_meine_seele_bwv435() {
    let path = sample_encodings_music_dir()
        .join("Complete_examples/Bach-JS_Wie_bist_du_meine_Seele_BWV435.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Bach-JS_Wie_bist_du_meine_Seele_BWV435.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_beethoven_hymn_to_joy() {
    let path = sample_encodings_music_dir().join("Complete_examples/Beethoven_Hymn_to_joy.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Beethoven_Hymn_to_joy.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_beethoven_song_op98() {
    let path = sample_encodings_music_dir().join("Complete_examples/Beethoven_Song_Op98.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Beethoven_Song_Op98.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_beethoven_string_quartet_op18_no1() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Beethoven_StringQuartet_Op18_No1.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Beethoven_StringQuartet_Op18_No1.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_berlioz_symphony_op25() {
    let path = sample_encodings_music_dir().join("Complete_examples/Berlioz_Symphony_Op25.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Berlioz_Symphony_Op25.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_borodin_string_trio_g_minor() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Borodin_StringTrio_g-minor.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Borodin_StringTrio_g-minor.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_brahms_string_quartet_op51_no1() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Brahms_StringQuartet_Op51_No1.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Brahms_StringQuartet_Op51_No1.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_brahms_wie_melodien_zieht_es_mir() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Brahms_WieMelodienZiehtEsMir.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Brahms_WieMelodienZiehtEsMir.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_chopin_etude_op10_no9() {
    let path = sample_encodings_music_dir().join("Complete_examples/Chopin_Etude_Op10_No9.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Chopin_Etude_Op10_No9.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_chopin_mazurka_op6_no1() {
    let path = sample_encodings_music_dir().join("Complete_examples/Chopin_Mazurka_Op6_No1.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Chopin_Mazurka_Op6_No1.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_czerny_praeludium_et_fuga_op603_no6() {
    let path = sample_encodings_music_dir()
        .join("Complete_examples/Czerny_Praeludium_et_Fuga_Op603_No6.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Czerny_Praeludium_et_Fuga_Op603_No6.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_czerny_string_quartet_d_minor() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Czerny_StringQuartet_d-minor.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Czerny_StringQuartet_d-minor.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_debussy_golliwoggs_cakewalk() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Debussy_Golliwogg'sCakewalk.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Debussy_Golliwogg'sCakewalk.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_debussy_mandoline() {
    let path = sample_encodings_music_dir().join("Complete_examples/Debussy_Mandoline.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Debussy_Mandoline.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_echigo_jishi() {
    let path = sample_encodings_music_dir().join("Complete_examples/Echigo-Jishi.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Echigo-Jishi.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_gluck_che_faro_senza_euridice() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Gluck_CheFaroSenzaEuridice.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Gluck_CheFaroSenzaEuridice.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_grieg_butterfly_op43_no1() {
    let path = sample_encodings_music_dir().join("Complete_examples/Grieg_Butterfly_Op43_No1.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Grieg_Butterfly_Op43_No1.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_grieg_little_bird_op43_no4() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Grieg_Little_bird_Op43_No4.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Grieg_Little_bird_Op43_No4.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_handel_arie() {
    let path = sample_encodings_music_dir().join("Complete_examples/Handel_Arie.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Handel_Arie.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_handel_concerto_grosso() {
    let path = sample_encodings_music_dir().join("Complete_examples/Handel_Concerto_grosso.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Handel_Concerto_grosso.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_handel_messias() {
    let path = sample_encodings_music_dir().join("Complete_examples/Handel_Messias.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Handel_Messias.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_haydn_string_quartet_op1_no1() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Haydn_StringQuartet_Op1_No1.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Haydn_StringQuartet_Op1_No1.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_hopkins_gather_round_the_christmas_tree() {
    let path = sample_encodings_music_dir()
        .join("Complete_examples/Hopkins_GatherRoundTheChristmasTree.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Hopkins_GatherRoundTheChristmasTree.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_hummel_concerto_for_trumpet_e_major() {
    let path = sample_encodings_music_dir()
        .join("Complete_examples/Hummel_Concerto_for_trumpet_E-major.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Hummel_Concerto_for_trumpet_E-major.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_hummel_preludes_op67_no11() {
    let path = sample_encodings_music_dir().join("Complete_examples/Hummel_Preludes_Op67_No11.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Hummel_Preludes_Op67_No11.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_ives_the_cage() {
    let path = sample_encodings_music_dir().join("Complete_examples/Ives_TheCage.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Ives_TheCage.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_joplin_elite_syncopations() {
    let path = sample_encodings_music_dir().join("Complete_examples/Joplin_Elite_Syncopations.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Joplin_Elite_Syncopations.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_joplin_maple_leaf_rag() {
    let path = sample_encodings_music_dir().join("Complete_examples/Joplin_Maple_leaf_Rag.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Joplin_Maple_leaf_Rag.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_kirnberger_fugue_for_brass_quartet_eb_major() {
    let path = sample_encodings_music_dir()
        .join("Complete_examples/Kirnberger_Fugue_for_BrassQuartet_Eb-major.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Kirnberger_Fugue_for_BrassQuartet_Eb-major.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_krebs_trio_for_2_pianos_eb_major() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Krebs_Trio_for_2_pianos_Eb-major.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Krebs_Trio_for_2_pianos_Eb-major.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_krebs_trio_for_2_pianos_c_minor() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Krebs_Trio_for_2_pianos_c-minor.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Krebs_Trio_for_2_pianos_c-minor.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_liszt_four_little_pieces_no1() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Liszt_Four_little_pieces_No1.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Liszt_Four_little_pieces_No1.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_lully_la_descente_de_mars() {
    let path = sample_encodings_music_dir().join("Complete_examples/Lully_LaDescenteDeMars.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Lully_LaDescenteDeMars.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_mahler_song() {
    let path = sample_encodings_music_dir().join("Complete_examples/Mahler_Song.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Mahler_Song.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_marney_break_thou_the_bread_of_life() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Marney_BreakThouTheBreadOfLife.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Marney_BreakThouTheBreadOfLife.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_mcferrin_dont_worry() {
    let path = sample_encodings_music_dir().join("Complete_examples/McFerrin_Don't_worry.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "McFerrin_Don't_worry.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_mozart_das_veilchen_kv476() {
    let path = sample_encodings_music_dir().join("Complete_examples/Mozart_Das_Veilchen_KV476.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Mozart_Das_Veilchen_KV476.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_mozart_fugue_g_minor_kv401() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Mozart_Fugue_g-minor_KV401.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Mozart_Fugue_g-minor_KV401.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_mozart_quintett_kv581() {
    let path = sample_encodings_music_dir().join("Complete_examples/Mozart_Quintett_KV581.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Mozart_Quintett_KV581.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_pachelbel_canon_in_d() {
    let path = sample_encodings_music_dir().join("Complete_examples/Pachelbel_Canon_in_D.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Pachelbel_Canon_in_D.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_parker_gillespie_shawnuff() {
    let path = sample_encodings_music_dir().join("Complete_examples/Parker-Gillespie_ShawNuff.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Parker-Gillespie_ShawNuff.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_ponchielli_larrivo_del_re() {
    let path = sample_encodings_music_dir().join("Complete_examples/Ponchielli_LarrivoDelRe.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Ponchielli_LarrivoDelRe.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_praetorius_puer_nobis_nascitur() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Praetorius_PuerNobisNascitur.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Praetorius_PuerNobisNascitur.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_ravel_le_tombeau() {
    let path = sample_encodings_music_dir().join("Complete_examples/Ravel_Le_tombeau.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Ravel_Le_tombeau.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_rimsky_korsakov_string_quartet_b_la_f() {
    let path = sample_encodings_music_dir()
        .join("Complete_examples/Rimsky-Korsakov_StringQuartet_B-LA-F.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Rimsky-Korsakov_StringQuartet_B-LA-F.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_saint_saens_le_carneval_des_animaux() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Saint-Saens_LeCarnevalDesAnimaux.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Saint-Saens_LeCarnevalDesAnimaux.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_scarlatti_sonata_in_c_major() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Scarlatti_Sonata_in_C-major.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Scarlatti_Sonata_in_C-major.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_schubert_erlkoenig() {
    let path = sample_encodings_music_dir().join("Complete_examples/Schubert_Erlkoenig.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Schubert_Erlkoenig.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_schubert_lindenbaum() {
    let path = sample_encodings_music_dir().join("Complete_examples/Schubert_Lindenbaum.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Schubert_Lindenbaum.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_schuetz_domine_deus() {
    let path = sample_encodings_music_dir().join("Complete_examples/Schuetz_DomineDeus.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Schuetz_DomineDeus.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_schuetz_jubilate_deo() {
    let path = sample_encodings_music_dir().join("Complete_examples/Schuetz_Jubilate_Deo.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Schuetz_Jubilate_Deo.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_schumann_landmann_op68_no10() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Schumann_Landmann_Op68_No10.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Schumann_Landmann_Op68_No10.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_schumann_song_op48_no1() {
    let path = sample_encodings_music_dir().join("Complete_examples/Schumann_Song_Op48_No1.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Schumann_Song_Op48_No1.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_schumann_string_quartet_op41_no1() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Schumann_StringQuartet_Op41_No1.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Schumann_StringQuartet_Op41_No1.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_telemann_concert() {
    let path = sample_encodings_music_dir().join("Complete_examples/Telemann_Concert.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Telemann_Concert.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_telemann_suite() {
    let path = sample_encodings_music_dir().join("Complete_examples/Telemann_Suite.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Telemann_Suite.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_vivaldi_violin_concert_op8_no2() {
    let path =
        sample_encodings_music_dir().join("Complete_examples/Vivaldi_ViolinConcert_Op8_No2.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Vivaldi_ViolinConcert_Op8_No2.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_weber_arie() {
    let path = sample_encodings_music_dir().join("Complete_examples/Weber_Arie.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Weber_Arie.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_webern_variations_for_piano_op27_no2() {
    let path = sample_encodings_music_dir()
        .join("Complete_examples/Webern_Variations_for_Piano_Op27_No2.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Webern_Variations_for_Piano_Op27_No2.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

// ============================================================================
// Lyrics Roundtrip Tests
// ============================================================================

#[test]
fn test_roundtrip_lyrics_attribute_syl() {
    let path = sample_encodings_music_dir().join("Lyrics/attribute_syl.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Lyrics/attribute_syl.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_lyrics_element_syl() {
    let path = sample_encodings_music_dir().join("Lyrics/element_syl.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Lyrics/element_syl.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_lyrics_lyrics() {
    let path = sample_encodings_music_dir().join("Lyrics/lyrics.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Lyrics/lyrics.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_roundtrip_lyrics_multiple_verses() {
    let path = sample_encodings_music_dir().join("Lyrics/multiple_verses.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Lyrics/multiple_verses.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}

// ============================================================================
// Encoding Alternatives Roundtrip Tests (Mozart's Das Veilchen)
// ============================================================================

#[test]
fn test_roundtrip_encoding_alternatives_das_veilchen_0parameters() {
    let path = sample_encodings_music_dir()
        .join("Encoding_alternatives/Mozart_Veilchen/Das_Veilchen_0Parameters.mei");
    let result = roundtrip_mei_file(&path);
    assert!(
        result.is_ok(),
        "Das_Veilchen_0Parameters.mei roundtrip failed: {}",
        result.unwrap_err()
    );
}
