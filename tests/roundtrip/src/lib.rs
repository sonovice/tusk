//! Shared cross-format roundtrip harness.
//!
//! Provides fixture loading, cross-format pipelines, and assertion helpers
//! used by all roundtrip test files.

use std::path::{Path, PathBuf};

// Re-export crates for test files.
pub use tusk_format;
pub use tusk_lilypond;
pub use tusk_mei;
pub use tusk_model;
pub use tusk_musicxml;

// ============================================================================
// Fixture loading
// ============================================================================

/// Root of `tests/fixtures/` relative to this crate's manifest dir.
pub fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures")
}

/// Load a fixture file from `tests/fixtures/{subdir}/{name}`.
pub fn load_fixture(subdir: &str, name: &str) -> String {
    let path = fixtures_root().join(subdir).join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// Read a file and convert from UTF-16 to UTF-8 if needed.
pub fn read_xml_file(path: &str) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("read {path}: {e}"))?;
    if bytes.len() >= 2 {
        if bytes[0] == 0xFE && bytes[1] == 0xFF {
            let u16s: Vec<u16> = bytes[2..]
                .chunks_exact(2)
                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                .collect();
            return String::from_utf16(&u16s).map_err(|e| format!("UTF-16 BE: {e}"));
        }
        if bytes[0] == 0xFF && bytes[1] == 0xFE {
            let u16s: Vec<u16> = bytes[2..]
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();
            return String::from_utf16(&u16s).map_err(|e| format!("UTF-16 LE: {e}"));
        }
    }
    String::from_utf8(bytes).map_err(|e| format!("UTF-8: {e}"))
}

/// Collect all fixture paths matching `tests/fixtures/{subdir}/**/*.{ext}`.
pub fn collect_fixtures(subdir: &str, ext: &str) -> Vec<PathBuf> {
    let dir = fixtures_root().join(subdir);
    if !dir.exists() {
        return Vec::new();
    }
    collect_files_recursive(&dir, ext)
}

fn collect_files_recursive(dir: &Path, ext: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return files;
    };
    let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            files.extend(collect_files_recursive(&path, ext));
        } else if path.extension().and_then(|e| e.to_str()) == Some(ext) {
            files.push(path);
        }
    }
    files
}

/// Sorted directory entries with a given extension (non-recursive).
pub fn sorted_dir_entries(dir: &Path, ext: &str) -> Vec<std::fs::DirEntry> {
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .expect("read dir")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|x| x.to_str())
                .is_some_and(|x| x == ext)
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());
    entries
}

// ============================================================================
// Cross-format pipeline
// ============================================================================

/// Two-pass cross-format stabilization.
///
/// Pass 1: source → format A → format B → format A → `a1`
/// Pass 2: `a1` → format B → format A → `a2`
/// Returns `(a1, a2)` for the caller to assert stability.
pub fn cross_roundtrip<F, G>(
    source: &str,
    a_to_b: F,
    b_to_a: G,
) -> Result<(String, String), String>
where
    F: Fn(&str) -> Result<String, String>,
    G: Fn(&str) -> Result<String, String>,
{
    let b1 = a_to_b(source)?;
    let a1 = b_to_a(&b1)?;
    let b2 = a_to_b(&a1)?;
    let a2 = b_to_a(&b2)?;
    Ok((a1, a2))
}

/// Assert that two strings are equal (for pipeline stabilization).
pub fn assert_stable(a1: &str, a2: &str, context: &str) {
    if a1 != a2 {
        eprintln!("=== PASS1 for {context} ===\n{a1}\n=== PASS2 for {context} ===\n{a2}\n=== END ===");
        let lines1: Vec<_> = a1.lines().collect();
        let lines2: Vec<_> = a2.lines().collect();
        let first_diff = lines1
            .iter()
            .zip(lines2.iter())
            .enumerate()
            .find(|(_, (l1, l2))| l1 != l2);
        if let Some((i, (l1, l2))) = first_diff {
            panic!(
                "Cross-format not stable for {context}.\n\
                 First diff at line {i}:\n  pass1: {l1}\n  pass2: {l2}"
            );
        } else {
            panic!(
                "Cross-format not stable for {context} (line count: {} vs {})",
                lines1.len(),
                lines2.len()
            );
        }
    }
}

/// Assert MEI XML stability using tree-based comparison.
pub fn assert_stable_mei(xml1: &str, xml2: &str, context: &str) {
    if let Err(e) = tusk_mei::xml_compare::compare_xml(xml1, xml2) {
        match e {
            tusk_mei::xml_compare::CompareError::Differences(diffs) => {
                let mut msg = format!(
                    "Cross-format MEI not stable for {context}: {} differences\n",
                    diffs.len()
                );
                for (i, d) in diffs.iter().take(10).enumerate() {
                    msg.push_str(&format!("  {}. {}: {}\n", i + 1, d.path, d.description));
                }
                panic!("{msg}");
            }
            tusk_mei::xml_compare::CompareError::ParseError(msg) => {
                panic!("MEI XML parse error for {context}: {msg}");
            }
        }
    }
}

// ============================================================================
// Format-specific pipeline helpers
// ============================================================================

/// MusicXML string → MEI XML string.
pub fn musicxml_to_mei(xml: &str) -> Result<String, String> {
    let pw = tusk_musicxml::parse_score_partwise(xml)
        .or_else(|_| tusk_musicxml::parse_score_timewise(xml))
        .map_err(|e| format!("MusicXML parse: {e}"))?;
    let (mei, _ext) = tusk_musicxml::import(&pw).map_err(|e| format!("MusicXML→MEI import: {e}"))?;
    tusk_mei::export(&mei).map_err(|e| format!("MEI export: {e}"))
}

/// MEI XML string → MusicXML string.
pub fn mei_to_musicxml(xml: &str) -> Result<String, String> {
    let mei = tusk_mei::import(xml).map_err(|e| format!("MEI parse: {e}"))?;
    let ext = tusk_model::ExtensionStore::default();
    let pw = tusk_musicxml::export_with_ext(&mei, &ext)
        .map_err(|e| format!("MEI→MusicXML export: {e}"))?;
    tusk_musicxml::serialize(&pw).map_err(|e| format!("MusicXML serialize: {e}"))
}

/// MusicXML string → LilyPond string.
pub fn musicxml_to_lilypond(xml: &str) -> Result<String, String> {
    let pw = tusk_musicxml::parse_score_partwise(xml)
        .or_else(|_| tusk_musicxml::parse_score_timewise(xml))
        .map_err(|e| format!("MusicXML parse: {e}"))?;
    let (mei, ext) = tusk_musicxml::import(&pw).map_err(|e| format!("MusicXML→MEI import: {e}"))?;
    let ly_file =
        tusk_lilypond::export::export(&mei, &ext).map_err(|e| format!("MEI→LilyPond export: {e}"))?;
    Ok(tusk_lilypond::serializer::serialize(&ly_file))
}

/// LilyPond string → MusicXML string.
pub fn lilypond_to_musicxml(ly: &str) -> Result<String, String> {
    let file = tusk_lilypond::parser::Parser::new(ly)
        .and_then(|p| p.parse())
        .map_err(|e| format!("LilyPond parse: {e}"))?;
    let (mei, ext) =
        tusk_lilypond::import::import(&file).map_err(|e| format!("LilyPond→MEI import: {e}"))?;
    let pw =
        tusk_musicxml::export_with_ext(&mei, &ext).map_err(|e| format!("MEI→MusicXML export: {e}"))?;
    tusk_musicxml::serialize(&pw).map_err(|e| format!("MusicXML serialize: {e}"))
}

/// MEI XML string → LilyPond string.
pub fn mei_to_lilypond(xml: &str) -> Result<String, String> {
    let mei = tusk_mei::import(xml).map_err(|e| format!("MEI parse: {e}"))?;
    let ext = tusk_model::ExtensionStore::default();
    let ly_file =
        tusk_lilypond::export::export(&mei, &ext).map_err(|e| format!("MEI→LilyPond export: {e}"))?;
    Ok(tusk_lilypond::serializer::serialize(&ly_file))
}

/// LilyPond string → MEI XML string.
pub fn lilypond_to_mei(ly: &str) -> Result<String, String> {
    let file = tusk_lilypond::parser::Parser::new(ly)
        .and_then(|p| p.parse())
        .map_err(|e| format!("LilyPond parse: {e}"))?;
    let (mei, _ext) =
        tusk_lilypond::import::import(&file).map_err(|e| format!("LilyPond→MEI import: {e}"))?;
    tusk_mei::export(&mei).map_err(|e| format!("MEI export: {e}"))
}

/// LilyPond string → LilyPond string (via MEI roundtrip).
pub fn lilypond_via_mei(ly: &str) -> Result<String, String> {
    let file = tusk_lilypond::parser::Parser::new(ly)
        .and_then(|p| p.parse())
        .map_err(|e| format!("LilyPond parse: {e}"))?;
    let (mei, ext) =
        tusk_lilypond::import::import(&file).map_err(|e| format!("LilyPond→MEI import: {e}"))?;
    let exported =
        tusk_lilypond::export::export(&mei, &ext).map_err(|e| format!("MEI→LilyPond export: {e}"))?;
    Ok(tusk_lilypond::serializer::serialize(&exported))
}

// ============================================================================
// Try-helpers: skip on parse/convert error, panic on instability
// ============================================================================

/// Try MusicXML->MEI->MusicXML cross roundtrip. Skip on error, panic on instability.
pub fn try_musicxml_via_mei(src: &str, name: &str) {
    match cross_roundtrip(src, musicxml_to_mei, mei_to_musicxml) {
        Ok((a1, a2)) => assert_stable(&a1, &a2, name),
        Err(_) => {}
    }
}

/// Try MusicXML->LilyPond->MusicXML. Skip on error, panic on instability.
pub fn try_musicxml_via_lilypond(src: &str, name: &str) {
    match cross_roundtrip(src, musicxml_to_lilypond, lilypond_to_musicxml) {
        Ok((a1, a2)) => assert_stable(&a1, &a2, name),
        Err(_) => {}
    }
}

/// Try MEI->MusicXML->MEI. Skip on error, panic on MEI instability.
pub fn try_mei_via_musicxml(src: &str, name: &str) {
    match cross_roundtrip(src, mei_to_musicxml, musicxml_to_mei) {
        Ok((a1, a2)) => assert_stable_mei(&a1, &a2, name),
        Err(_) => {}
    }
}

/// Try MEI->LilyPond->MEI. Skip on error, panic on MEI instability.
pub fn try_mei_via_lilypond(src: &str, name: &str) {
    // Large scores (e.g. Gluck) need more stack for recursive serialization.
    let src = src.to_string();
    let name = name.to_string();
    let handle = std::thread::Builder::new()
        .stack_size(16 * 1024 * 1024)
        .spawn(move || {
            match cross_roundtrip(&src, mei_to_lilypond, lilypond_to_mei) {
                Ok((a1, a2)) => assert_stable_mei(&a1, &a2, &name),
                Err(_) => {}
            }
        })
        .expect("spawn thread");
    if let Err(e) = handle.join() {
        std::panic::resume_unwind(e);
    }
}

/// Try LilyPond->MEI->LilyPond. Skip on error, panic on instability.
pub fn try_lilypond_via_mei(src: &str, name: &str) {
    let ly1 = match lilypond_via_mei(src) {
        Ok(ly) => ly,
        Err(_) => return,
    };
    let ly2 = match lilypond_via_mei(&ly1) {
        Ok(ly) => ly,
        Err(_) => return,
    };
    assert_stable(&ly1, &ly2, name);
}

/// LilyPond string → LilyPond string (via MusicXML roundtrip).
pub fn lilypond_via_musicxml(ly: &str) -> Result<String, String> {
    let mxml = lilypond_to_musicxml(ly)?;
    musicxml_to_lilypond(&mxml)
}

/// Try LilyPond->MusicXML->LilyPond. Skip on error, panic on instability.
pub fn try_lilypond_via_musicxml(src: &str, name: &str) {
    let ly1 = match lilypond_via_musicxml(src) {
        Ok(ly) => ly,
        Err(_) => return,
    };
    let ly2 = match lilypond_via_musicxml(&ly1) {
        Ok(ly) => ly,
        Err(_) => return,
    };
    assert_stable(&ly1, &ly2, name);
}
