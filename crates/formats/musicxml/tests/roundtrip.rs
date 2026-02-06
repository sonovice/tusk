//! Roundtrip tests for MusicXML → MEI → MusicXML conversion.
//!
//! Three levels of testing:
//!
//! 1. **Conversion roundtrip** (import/export only):
//!    XML → parse → ScorePartwise₁ → import → Mei → export → ScorePartwise₂
//!    Compares ScorePartwise₁ vs ScorePartwise₂ directly.
//!    Tests: import + export logic, isolated from serializer.
//!
//! 2. **Full roundtrip** (includes serializer/parser):
//!    XML → parse → ScorePartwise₁ → import → Mei → export → serialize → XML₂ → parse → ScorePartwise₃
//!    Compares ScorePartwise₁ vs ScorePartwise₃.
//!    Tests: entire pipeline including MusicXML serializer.
//!
//! 3. **Triangle MEI roundtrip** (catches symmetric import bugs):
//!    MusicXML₀ → import → MEI₁ → export → MusicXML₁ → import → MEI₂
//!    Compares MEI₁ vs MEI₂.
//!    Tests: if import has inconsistent behavior, MEI₁ ≠ MEI₂.
//!
//! 4. **Triangle MusicXML roundtrip** (catches symmetric export bugs):
//!    MEI₁ → export → MusicXML₁ → import → MEI₂ → export → MusicXML₂
//!    Compares MusicXML₁ vs MusicXML₂.
//!    Tests: if export has inconsistent behavior, MusicXML₁ ≠ MusicXML₂.
//!
//! If conversion roundtrip passes but full roundtrip fails → serializer bug.
//! If conversion roundtrip fails → import/export bug.
//! If triangle MEI fails → import has inconsistent/lossy behavior.
//! If triangle MusicXML fails → export has inconsistent/lossy behavior.

use std::fs;

use tusk_model::elements::Mei;
use tusk_musicxml::model::attributes::{KeyContent, TimeContent};
use tusk_musicxml::model::elements::{MeasureContent, ScorePartwise};
use tusk_musicxml::model::note::FullNoteContent;
use tusk_musicxml::parser::{parse_score_partwise, parse_score_timewise};
use tusk_musicxml::{export, import, serialize};

/// Read a file and convert from UTF-16 to UTF-8 if needed.
/// Handles both UTF-16 BE and LE with BOM detection.
fn read_xml_file(path: &str) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|e| format!("Failed to read file {}: {}", path, e))?;

    // Check for UTF-16 BOM (Byte Order Mark)
    if bytes.len() >= 2 {
        // UTF-16 BE BOM: FE FF
        if bytes[0] == 0xFE && bytes[1] == 0xFF {
            let u16_chars: Vec<u16> = bytes[2..]
                .chunks_exact(2)
                .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
                .collect();
            return String::from_utf16(&u16_chars)
                .map_err(|e| format!("UTF-16 BE decode error: {}", e));
        }
        // UTF-16 LE BOM: FF FE
        if bytes[0] == 0xFF && bytes[1] == 0xFE {
            let u16_chars: Vec<u16> = bytes[2..]
                .chunks_exact(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .collect();
            return String::from_utf16(&u16_chars)
                .map_err(|e| format!("UTF-16 LE decode error: {}", e));
        }
    }

    // No BOM, assume UTF-8
    String::from_utf8(bytes).map_err(|e| format!("UTF-8 decode error: {}", e))
}

// ============================================================================
// Roundtrip Test Harness
// ============================================================================

/// Detect whether a MusicXML string is in timewise format.
fn is_timewise(xml: &str) -> bool {
    xml.contains("<score-timewise")
}

/// Parse MusicXML, auto-detecting partwise vs timewise format.
fn parse_musicxml(xml: &str) -> Result<ScorePartwise, String> {
    if is_timewise(xml) {
        parse_score_timewise(xml).map_err(|e| format!("Parse error (timewise): {}", e))
    } else {
        parse_score_partwise(xml).map_err(|e| format!("Parse error (partwise): {}", e))
    }
}

/// Conversion roundtrip: MusicXML → MEI → MusicXML (no serialization).
/// Tests import/export logic only, isolated from serializer bugs.
/// Returns (original, exported) for direct model comparison.
fn conversion_roundtrip(xml: &str) -> Result<(ScorePartwise, ScorePartwise), String> {
    let original = parse_musicxml(xml)?;
    let mei = import(&original).map_err(|e| format!("Import (MusicXML→MEI) error: {}", e))?;
    let exported = export(&mei).map_err(|e| format!("Export (MEI→MusicXML) error: {}", e))?;
    Ok((original, exported))
}

/// Full roundtrip: MusicXML → MEI → MusicXML → XML string → MusicXML.
/// Tests entire pipeline including MusicXML serializer/parser.
/// Returns (original, roundtripped) after serialize→parse cycle.
fn full_roundtrip(xml: &str) -> Result<(ScorePartwise, ScorePartwise), String> {
    let original = parse_musicxml(xml)?;
    let mei = import(&original).map_err(|e| format!("Import (MusicXML→MEI) error: {}", e))?;
    let exported = export(&mei).map_err(|e| format!("Export (MEI→MusicXML) error: {}", e))?;
    let xml2 = serialize(&exported).map_err(|e| format!("Serialize error: {}", e))?;
    let roundtripped = parse_score_partwise(&xml2).map_err(|e| format!("Re-parse error: {}", e))?;
    Ok((original, roundtripped))
}

/// Triangle roundtrip A: MusicXML → MEI₁ → MusicXML → MEI₂.
/// Catches import bugs where second import reveals inconsistency.
/// Returns (mei1, mei2) for direct comparison.
fn triangle_mei_roundtrip(xml: &str) -> Result<(Mei, Mei), String> {
    let original = parse_musicxml(xml)?;
    let mei1 = import(&original).map_err(|e| format!("Import₁ (MusicXML→MEI) error: {}", e))?;
    let mxml1 = export(&mei1).map_err(|e| format!("Export₁ (MEI→MusicXML) error: {}", e))?;
    let mei2 = import(&mxml1).map_err(|e| format!("Import₂ (MusicXML→MEI) error: {}", e))?;
    Ok((mei1, mei2))
}

/// Triangle roundtrip B: MEI₁ → MusicXML₁ → MEI₂ → MusicXML₂.
/// Catches export bugs where second export reveals inconsistency.
/// Returns (mxml1, mxml2) for comparison.
fn triangle_mxml_roundtrip(xml: &str) -> Result<(ScorePartwise, ScorePartwise), String> {
    let original = parse_musicxml(xml)?;
    let mei1 = import(&original).map_err(|e| format!("Import₁ (MusicXML→MEI) error: {}", e))?;
    let mxml1 = export(&mei1).map_err(|e| format!("Export₁ (MEI→MusicXML) error: {}", e))?;
    let mei2 = import(&mxml1).map_err(|e| format!("Import₂ (MusicXML→MEI) error: {}", e))?;
    let mxml2 = export(&mei2).map_err(|e| format!("Export₂ (MEI→MusicXML) error: {}", e))?;
    Ok((mxml1, mxml2))
}

/// Load a fixture file from tests/fixtures/musicxml/.
fn load_fixture(fixture_name: &str) -> Result<String, String> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let fixture_path = format!(
        "{}/../../../tests/fixtures/musicxml/{}",
        manifest_dir, fixture_name
    );
    fs::read_to_string(&fixture_path)
        .map_err(|e| format!("Failed to read fixture {}: {}", fixture_name, e))
}

/// Load a fixture and perform conversion roundtrip (no serialization).
fn conversion_roundtrip_fixture(
    fixture_name: &str,
) -> Result<(ScorePartwise, ScorePartwise), String> {
    let xml = load_fixture(fixture_name)?;
    conversion_roundtrip(&xml)
}

/// Load a fixture and perform full roundtrip (with serialization).
fn full_roundtrip_fixture(fixture_name: &str) -> Result<(ScorePartwise, ScorePartwise), String> {
    let xml = load_fixture(fixture_name)?;
    full_roundtrip(&xml)
}

/// Load a fixture and perform triangle MEI roundtrip.
fn triangle_mei_roundtrip_fixture(fixture_name: &str) -> Result<(Mei, Mei), String> {
    let xml = load_fixture(fixture_name)?;
    triangle_mei_roundtrip(&xml)
}

/// Load a fixture and perform triangle MusicXML roundtrip.
fn triangle_mxml_roundtrip_fixture(
    fixture_name: &str,
) -> Result<(ScorePartwise, ScorePartwise), String> {
    let xml = load_fixture(fixture_name)?;
    triangle_mxml_roundtrip(&xml)
}

// ============================================================================
// Comparison Logic
// ============================================================================

/// Differences found during comparison.
#[derive(Debug, Default)]
struct Differences {
    items: Vec<String>,
}

impl Differences {
    fn add(&mut self, msg: impl Into<String>) {
        self.items.push(msg.into());
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn report(&self) -> String {
        self.items.join("\n")
    }
}

/// Compare two MusicXML scores for semantic equivalence.
fn compare_scores(original: &ScorePartwise, roundtripped: &ScorePartwise) -> Differences {
    let mut diffs = Differences::default();

    // Compare part count
    if original.parts.len() != roundtripped.parts.len() {
        diffs.add(format!(
            "Part count mismatch: original={}, roundtripped={}",
            original.parts.len(),
            roundtripped.parts.len()
        ));
        return diffs;
    }

    // Compare part-list (part names)
    compare_part_list(original, roundtripped, &mut diffs);

    // Compare each part
    for (i, (orig_part, rt_part)) in original
        .parts
        .iter()
        .zip(roundtripped.parts.iter())
        .enumerate()
    {
        let part_id = &orig_part.id;

        // Check part ID
        if orig_part.id != rt_part.id {
            diffs.add(format!(
                "Part {}: ID mismatch: original='{}', roundtripped='{}'",
                i, orig_part.id, rt_part.id
            ));
        }

        // Check measure count
        if orig_part.measures.len() != rt_part.measures.len() {
            diffs.add(format!(
                "Part '{}': measure count mismatch: original={}, roundtripped={}",
                part_id,
                orig_part.measures.len(),
                rt_part.measures.len()
            ));
            continue;
        }

        // Compare each measure
        for (m_idx, (orig_measure, rt_measure)) in orig_part
            .measures
            .iter()
            .zip(rt_part.measures.iter())
            .enumerate()
        {
            let measure_num = &orig_measure.number;

            // Check measure number
            if orig_measure.number != rt_measure.number {
                diffs.add(format!(
                    "Part '{}', Measure {}: number mismatch: original='{}', roundtripped='{}'",
                    part_id, m_idx, orig_measure.number, rt_measure.number
                ));
            }

            // Compare measure content
            compare_measure_content(
                part_id,
                measure_num,
                &orig_measure.content,
                &rt_measure.content,
                &mut diffs,
            );
        }
    }

    diffs
}

fn compare_part_list(
    original: &ScorePartwise,
    roundtripped: &ScorePartwise,
    diffs: &mut Differences,
) {
    use tusk_musicxml::model::elements::PartListItem;

    let orig_parts: Vec<_> = original
        .part_list
        .items
        .iter()
        .filter_map(|item| match item {
            PartListItem::ScorePart(sp) => Some(sp.as_ref()),
            _ => None,
        })
        .collect();

    let rt_parts: Vec<_> = roundtripped
        .part_list
        .items
        .iter()
        .filter_map(|item| match item {
            PartListItem::ScorePart(sp) => Some(sp.as_ref()),
            _ => None,
        })
        .collect();

    if orig_parts.len() != rt_parts.len() {
        diffs.add(format!(
            "Part-list: score-part count mismatch: original={}, roundtripped={}",
            orig_parts.len(),
            rt_parts.len()
        ));
        return;
    }

    for (i, (orig, rt)) in orig_parts.iter().zip(rt_parts.iter()).enumerate() {
        if orig.id != rt.id {
            diffs.add(format!(
                "Part-list[{}]: ID mismatch: original='{}', roundtripped='{}'",
                i, orig.id, rt.id
            ));
        }
        if orig.part_name.value != rt.part_name.value {
            diffs.add(format!(
                "Part-list[{}] (ID='{}'): name mismatch: original='{}', roundtripped='{}'",
                i, orig.id, orig.part_name.value, rt.part_name.value
            ));
        }
    }
}

fn compare_measure_content(
    part_id: &str,
    measure_num: &str,
    original: &[MeasureContent],
    roundtripped: &[MeasureContent],
    diffs: &mut Differences,
) {
    // Extract notes from both (ignoring backups, forwards, etc.)
    let orig_notes: Vec<_> = original
        .iter()
        .filter_map(|c| match c {
            MeasureContent::Note(n) => Some(n.as_ref()),
            _ => None,
        })
        .collect();

    let rt_notes: Vec<_> = roundtripped
        .iter()
        .filter_map(|c| match c {
            MeasureContent::Note(n) => Some(n.as_ref()),
            _ => None,
        })
        .collect();

    if orig_notes.len() != rt_notes.len() {
        diffs.add(format!(
            "Part '{}', Measure '{}': note count mismatch: original={}, roundtripped={}",
            part_id,
            measure_num,
            orig_notes.len(),
            rt_notes.len()
        ));
        return;
    }

    // Compare each note
    for (n_idx, (orig_note, rt_note)) in orig_notes.iter().zip(rt_notes.iter()).enumerate() {
        let ctx = format!(
            "Part '{}', Measure '{}', Note {}",
            part_id, measure_num, n_idx
        );

        // Compare pitch/rest
        compare_note_content(&ctx, &orig_note.content, &rt_note.content, diffs);

        // Compare duration
        if orig_note.duration != rt_note.duration {
            diffs.add(format!(
                "{}: duration mismatch: original={:?}, roundtripped={:?}",
                ctx, orig_note.duration, rt_note.duration
            ));
        }

        // Compare note type
        let orig_type = orig_note.note_type.as_ref().map(|t| t.value);
        let rt_type = rt_note.note_type.as_ref().map(|t| t.value);
        if orig_type != rt_type {
            diffs.add(format!(
                "{}: note type mismatch: original={:?}, roundtripped={:?}",
                ctx, orig_type, rt_type
            ));
        }

        // Compare dots
        if orig_note.dots.len() != rt_note.dots.len() {
            diffs.add(format!(
                "{}: dot count mismatch: original={}, roundtripped={}",
                ctx,
                orig_note.dots.len(),
                rt_note.dots.len()
            ));
        }

        // Compare chord flag
        if orig_note.chord.is_some() != rt_note.chord.is_some() {
            diffs.add(format!(
                "{}: chord flag mismatch: original={}, roundtripped={}",
                ctx,
                orig_note.chord.is_some(),
                rt_note.chord.is_some()
            ));
        }

        // Compare grace note
        if orig_note.grace.is_some() != rt_note.grace.is_some() {
            diffs.add(format!(
                "{}: grace flag mismatch: original={}, roundtripped={}",
                ctx,
                orig_note.grace.is_some(),
                rt_note.grace.is_some()
            ));
        }

        // Compare cue note
        if orig_note.cue.is_some() != rt_note.cue.is_some() {
            diffs.add(format!(
                "{}: cue flag mismatch: original={}, roundtripped={}",
                ctx,
                orig_note.cue.is_some(),
                rt_note.cue.is_some()
            ));
        }
    }

    // Compare attributes
    compare_attributes(part_id, measure_num, original, roundtripped, diffs);
}

fn compare_note_content(
    ctx: &str,
    original: &FullNoteContent,
    roundtripped: &FullNoteContent,
    diffs: &mut Differences,
) {
    match (original, roundtripped) {
        (FullNoteContent::Pitch(orig), FullNoteContent::Pitch(rt)) => {
            if orig.step != rt.step {
                diffs.add(format!(
                    "{}: step mismatch: original={:?}, roundtripped={:?}",
                    ctx, orig.step, rt.step
                ));
            }
            if orig.octave != rt.octave {
                diffs.add(format!(
                    "{}: octave mismatch: original={}, roundtripped={}",
                    ctx, orig.octave, rt.octave
                ));
            }
            // Allow small floating-point differences in alter
            let orig_alter = orig.alter.unwrap_or(0.0);
            let rt_alter = rt.alter.unwrap_or(0.0);
            if (orig_alter - rt_alter).abs() > 0.001 {
                diffs.add(format!(
                    "{}: alter mismatch: original={}, roundtripped={}",
                    ctx, orig_alter, rt_alter
                ));
            }
        }
        (FullNoteContent::Rest(_), FullNoteContent::Rest(_)) => {
            // Rests match
        }
        (FullNoteContent::Unpitched(_), FullNoteContent::Unpitched(_)) => {
            // Unpitched notes match
        }
        _ => {
            diffs.add(format!(
                "{}: note type mismatch: original={:?}, roundtripped={:?}",
                ctx,
                std::mem::discriminant(original),
                std::mem::discriminant(roundtripped)
            ));
        }
    }
}

fn compare_attributes(
    part_id: &str,
    measure_num: &str,
    original: &[MeasureContent],
    roundtripped: &[MeasureContent],
    diffs: &mut Differences,
) {
    let orig_attrs: Vec<_> = original
        .iter()
        .filter_map(|c| match c {
            MeasureContent::Attributes(a) => Some(a.as_ref()),
            _ => None,
        })
        .collect();

    let rt_attrs: Vec<_> = roundtripped
        .iter()
        .filter_map(|c| match c {
            MeasureContent::Attributes(a) => Some(a.as_ref()),
            _ => None,
        })
        .collect();

    // Compare first attributes block (most common case)
    if let (Some(orig), Some(rt)) = (orig_attrs.first(), rt_attrs.first()) {
        let ctx = format!("Part '{}', Measure '{}'", part_id, measure_num);

        // Compare divisions
        if orig.divisions != rt.divisions {
            diffs.add(format!(
                "{}: divisions mismatch: original={:?}, roundtripped={:?}",
                ctx, orig.divisions, rt.divisions
            ));
        }

        // Compare key signatures
        if let (Some(orig_key), Some(rt_key)) = (orig.keys.first(), rt.keys.first()) {
            // Extract fifths from key content
            let orig_fifths = match &orig_key.content {
                KeyContent::Traditional(k) => Some(k.fifths),
                KeyContent::NonTraditional(_) => None,
            };
            let rt_fifths = match &rt_key.content {
                KeyContent::Traditional(k) => Some(k.fifths),
                KeyContent::NonTraditional(_) => None,
            };
            if orig_fifths != rt_fifths {
                diffs.add(format!(
                    "{}: key fifths mismatch: original={:?}, roundtripped={:?}",
                    ctx, orig_fifths, rt_fifths
                ));
            }
        } else if orig.keys.len() != rt.keys.len() {
            diffs.add(format!(
                "{}: key count mismatch: original={}, roundtripped={}",
                ctx,
                orig.keys.len(),
                rt.keys.len()
            ));
        }

        // Compare time signatures
        if let (Some(orig_time), Some(rt_time)) = (orig.times.first(), rt.times.first()) {
            // Extract beats/beat-type from time content
            let orig_sig = match &orig_time.content {
                TimeContent::Standard(t) => t.signatures.first(),
                TimeContent::SenzaMisura(_) => None,
            };
            let rt_sig = match &rt_time.content {
                TimeContent::Standard(t) => t.signatures.first(),
                TimeContent::SenzaMisura(_) => None,
            };

            if let (Some(orig_s), Some(rt_s)) = (orig_sig, rt_sig) {
                if orig_s.beats != rt_s.beats {
                    diffs.add(format!(
                        "{}: time beats mismatch: original='{}', roundtripped='{}'",
                        ctx, orig_s.beats, rt_s.beats
                    ));
                }
                if orig_s.beat_type != rt_s.beat_type {
                    diffs.add(format!(
                        "{}: time beat-type mismatch: original='{}', roundtripped='{}'",
                        ctx, orig_s.beat_type, rt_s.beat_type
                    ));
                }
            }
        } else if orig.times.len() != rt.times.len() {
            diffs.add(format!(
                "{}: time signature count mismatch: original={}, roundtripped={}",
                ctx,
                orig.times.len(),
                rt.times.len()
            ));
        }

        // Compare clefs
        if let (Some(orig_clef), Some(rt_clef)) = (orig.clefs.first(), rt.clefs.first()) {
            if orig_clef.sign != rt_clef.sign {
                diffs.add(format!(
                    "{}: clef sign mismatch: original={:?}, roundtripped={:?}",
                    ctx, orig_clef.sign, rt_clef.sign
                ));
            }
            if orig_clef.line != rt_clef.line {
                diffs.add(format!(
                    "{}: clef line mismatch: original={:?}, roundtripped={:?}",
                    ctx, orig_clef.line, rt_clef.line
                ));
            }
        } else if orig.clefs.len() != rt.clefs.len() {
            diffs.add(format!(
                "{}: clef count mismatch: original={}, roundtripped={}",
                ctx,
                orig.clefs.len(),
                rt.clefs.len()
            ));
        }
    }
}

/// Assert conversion roundtrip passes (import/export only, no serialization).
fn assert_conversion_roundtrip(fixture_name: &str) {
    let (original, exported) = conversion_roundtrip_fixture(fixture_name)
        .unwrap_or_else(|e| panic!("Conversion roundtrip failed for {}: {}", fixture_name, e));

    let diffs = compare_scores(&original, &exported);
    if !diffs.is_empty() {
        panic!(
            "Conversion roundtrip differences for {} (import/export bug):\n{}",
            fixture_name,
            diffs.report()
        );
    }
}

/// Assert full roundtrip passes (includes serialization).
fn assert_full_roundtrip(fixture_name: &str) {
    let (original, roundtripped) = full_roundtrip_fixture(fixture_name)
        .unwrap_or_else(|e| panic!("Full roundtrip failed for {}: {}", fixture_name, e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Full roundtrip differences for {} (serializer bug if conversion passed):\n{}",
            fixture_name,
            diffs.report()
        );
    }
}

/// Assert triangle MEI roundtrip passes (MEI₁ == MEI₂).
/// Catches inconsistent import behavior.
fn assert_triangle_mei_roundtrip(fixture_name: &str) {
    let (mei1, mei2) = triangle_mei_roundtrip_fixture(fixture_name)
        .unwrap_or_else(|e| panic!("Triangle MEI roundtrip failed for {}: {}", fixture_name, e));

    if mei1 != mei2 {
        let mei1_str = format!("{:#?}", mei1);
        let mei2_str = format!("{:#?}", mei2);
        fs::write("/tmp/mei1_debug.txt", &mei1_str).unwrap();
        fs::write("/tmp/mei2_debug.txt", &mei2_str).unwrap();
        panic!(
            "Triangle MEI roundtrip failed for {} (import inconsistency): MEI₁ ≠ MEI₂\n\
             Import produces different MEI from the same logical content.\n\
             Debug output written to /tmp/mei1_debug.txt and /tmp/mei2_debug.txt",
            fixture_name
        );
    }
}

/// Assert triangle MusicXML roundtrip passes (MusicXML₁ == MusicXML₂).
/// Catches inconsistent export behavior.
fn assert_triangle_mxml_roundtrip(fixture_name: &str) {
    let (mxml1, mxml2) = triangle_mxml_roundtrip_fixture(fixture_name).unwrap_or_else(|e| {
        panic!(
            "Triangle MusicXML roundtrip failed for {}: {}",
            fixture_name, e
        )
    });

    let diffs = compare_scores(&mxml1, &mxml2);
    if !diffs.is_empty() {
        panic!(
            "Triangle MusicXML roundtrip failed for {} (export inconsistency): MusicXML₁ ≠ MusicXML₂\n\
             Export produces different MusicXML from the same logical content.\n{}",
            fixture_name,
            diffs.report()
        );
    }
}

/// Assert all four roundtrip tests pass.
/// Runs: conversion, full, triangle MEI, triangle MusicXML.
fn assert_roundtrip(fixture_name: &str) {
    assert_conversion_roundtrip(fixture_name);
    assert_full_roundtrip(fixture_name);
    assert_triangle_mei_roundtrip(fixture_name);
    assert_triangle_mxml_roundtrip(fixture_name);
}

// ============================================================================
// Roundtrip Tests for Fixtures
// ============================================================================

#[test]
fn test_roundtrip_hello_world() {
    assert_roundtrip("hello_world.musicxml");
}

#[test]
fn test_roundtrip_scale() {
    assert_roundtrip("scale.musicxml");
}

#[test]
fn test_roundtrip_durations() {
    assert_roundtrip("durations.musicxml");
}

#[test]
fn test_roundtrip_chords_and_rests() {
    assert_roundtrip("chords_and_rests.musicxml");
}

#[test]
fn test_roundtrip_high_divisions() {
    assert_roundtrip("high_divisions.musicxml");
}

#[test]
fn test_roundtrip_directions() {
    assert_roundtrip("directions.musicxml");
}

// ============================================================================
// Debug Helper Tests (can be used to inspect conversion output)
// ============================================================================

#[test]
#[ignore] // Run with --ignored to debug
fn debug_roundtrip_output() {
    let fixture_name = "hello_world.musicxml";
    let (original, exported) = conversion_roundtrip_fixture(fixture_name)
        .unwrap_or_else(|e| panic!("Conversion roundtrip failed: {}", e));

    println!("=== Original ===");
    println!("Parts: {}", original.parts.len());
    for part in &original.parts {
        println!("  Part '{}': {} measures", part.id, part.measures.len());
    }

    println!("\n=== After MEI conversion (no serialization) ===");
    println!("Parts: {}", exported.parts.len());
    for part in &exported.parts {
        println!("  Part '{}': {} measures", part.id, part.measures.len());
    }
}

// ============================================================================
// Spec Examples Tests (from specs/musicxml/examples/)
// ============================================================================

/// Load a spec example file.
fn load_spec_example(example_name: &str) -> Result<String, String> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let fixture_path = format!(
        "{}/../../../specs/musicxml/examples/{}",
        manifest_dir, example_name
    );
    read_xml_file(&fixture_path)
}

/// Assert all three roundtrip tests pass for a spec example.
fn assert_spec_example_roundtrip(example_name: &str) {
    let xml = load_spec_example(example_name)
        .unwrap_or_else(|e| panic!("Failed to load {}: {}", example_name, e));

    // Test conversion roundtrip (import/export only)
    let (original, exported) = conversion_roundtrip(&xml)
        .unwrap_or_else(|e| panic!("Conversion roundtrip failed for {}: {}", example_name, e));
    let diffs = compare_scores(&original, &exported);
    if !diffs.is_empty() {
        panic!(
            "Conversion roundtrip differences for {} (import/export bug):\n{}",
            example_name,
            diffs.report()
        );
    }

    // Test full roundtrip (includes serialization)
    let (original, roundtripped) = full_roundtrip(&xml)
        .unwrap_or_else(|e| panic!("Full roundtrip failed for {}: {}", example_name, e));
    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Full roundtrip differences for {} (serializer bug):\n{}",
            example_name,
            diffs.report()
        );
    }

    // Test triangle MEI roundtrip (catches import inconsistency)
    let (mei1, mei2) = triangle_mei_roundtrip(&xml)
        .unwrap_or_else(|e| panic!("Triangle MEI roundtrip failed for {}: {}", example_name, e));
    if mei1 != mei2 {
        panic!(
            "Triangle MEI roundtrip failed for {} (import inconsistency): MEI₁ ≠ MEI₂",
            example_name
        );
    }

    // Test triangle MusicXML roundtrip (catches export inconsistency)
    let (mxml1, mxml2) = triangle_mxml_roundtrip(&xml).unwrap_or_else(|e| {
        panic!(
            "Triangle MusicXML roundtrip failed for {}: {}",
            example_name, e
        )
    });
    let diffs = compare_scores(&mxml1, &mxml2);
    if !diffs.is_empty() {
        panic!(
            "Triangle MusicXML roundtrip failed for {} (export inconsistency): MusicXML₁ ≠ MusicXML₂\n{}",
            example_name,
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_spec_telemann() {
    assert_spec_example_roundtrip("Telemann.musicxml");
}

#[test]
fn test_roundtrip_spec_binchois() {
    assert_spec_example_roundtrip("Binchois.musicxml");
}

#[test]
fn test_roundtrip_spec_mozart_piano_sonata() {
    assert_spec_example_roundtrip("MozartPianoSonata.musicxml");
}

#[test]
fn test_roundtrip_spec_actor_prelude_sample() {
    assert_spec_example_roundtrip("ActorPreludeSample.musicxml");
}

#[test]
fn test_roundtrip_spec_beet_an_ge_sample() {
    assert_spec_example_roundtrip("BeetAnGeSample.musicxml");
}

#[test]
fn test_roundtrip_spec_brah_wi_me_sample() {
    assert_spec_example_roundtrip("BrahWiMeSample.musicxml");
}

#[test]
fn test_roundtrip_spec_brooke_west_sample() {
    assert_spec_example_roundtrip("BrookeWestSample.musicxml");
}

#[test]
fn test_roundtrip_spec_chant() {
    assert_spec_example_roundtrip("Chant.musicxml");
}

#[test]
fn test_roundtrip_spec_debu_mand_sample() {
    assert_spec_example_roundtrip("DebuMandSample.musicxml");
}

#[test]
fn test_roundtrip_spec_dichterliebe01() {
    assert_spec_example_roundtrip("Dichterliebe01.musicxml");
}

#[test]
fn test_roundtrip_spec_echigo_jishi() {
    assert_spec_example_roundtrip("Echigo-Jishi.musicxml");
}

#[test]
fn test_roundtrip_spec_faur_reve_sample() {
    assert_spec_example_roundtrip("FaurReveSample.musicxml");
}

#[test]
fn test_roundtrip_spec_mahl_fa_ge4_sample() {
    assert_spec_example_roundtrip("MahlFaGe4Sample.musicxml");
}

#[test]
fn test_roundtrip_spec_moza_chlo_sample() {
    assert_spec_example_roundtrip("MozaChloSample.musicxml");
}

#[test]
fn test_roundtrip_spec_mozart_trio() {
    assert_spec_example_roundtrip("MozartTrio.musicxml");
}

#[test]
fn test_roundtrip_spec_moza_veil_sample() {
    assert_spec_example_roundtrip("MozaVeilSample.musicxml");
}

#[test]
fn test_roundtrip_spec_saltarello() {
    assert_spec_example_roundtrip("Saltarello.musicxml");
}

#[test]
fn test_roundtrip_spec_schb_av_ma_sample() {
    assert_spec_example_roundtrip("SchbAvMaSample.musicxml");
}

// ============================================================================
// Spec Doc Example Tests (from tests/fixtures/musicxml/spec_examples/)
// ============================================================================

/// Load a spec_examples fixture file.
fn load_spec_examples_fixture(fixture_name: &str) -> Result<String, String> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let fixture_path = format!(
        "{}/../../../tests/fixtures/musicxml/spec_examples/{}",
        manifest_dir, fixture_name
    );
    fs::read_to_string(&fixture_path)
        .map_err(|e| format!("Failed to read fixture {}: {}", fixture_name, e))
}

/// Assert all three roundtrip tests pass for a spec_examples fixture.
fn assert_spec_examples_roundtrip(fixture_name: &str) {
    let xml = load_spec_examples_fixture(fixture_name)
        .unwrap_or_else(|e| panic!("Failed to load {}: {}", fixture_name, e));

    // Test conversion roundtrip (import/export only)
    let (original, exported) = conversion_roundtrip(&xml)
        .unwrap_or_else(|e| panic!("Conversion roundtrip failed for {}: {}", fixture_name, e));
    let diffs = compare_scores(&original, &exported);
    if !diffs.is_empty() {
        panic!(
            "Conversion roundtrip differences for {} (import/export bug):\n{}",
            fixture_name,
            diffs.report()
        );
    }

    // Test full roundtrip (includes serialization)
    let (original, roundtripped) = full_roundtrip(&xml)
        .unwrap_or_else(|e| panic!("Full roundtrip failed for {}: {}", fixture_name, e));
    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Full roundtrip differences for {} (serializer bug):\n{}",
            fixture_name,
            diffs.report()
        );
    }

    // Test triangle MEI roundtrip (catches import inconsistency)
    let (mei1, mei2) = triangle_mei_roundtrip(&xml)
        .unwrap_or_else(|e| panic!("Triangle MEI roundtrip failed for {}: {}", fixture_name, e));
    if mei1 != mei2 {
        panic!(
            "Triangle MEI roundtrip failed for {} (import inconsistency): MEI₁ ≠ MEI₂",
            fixture_name
        );
    }

    // Test triangle MusicXML roundtrip (catches export inconsistency)
    let (mxml1, mxml2) = triangle_mxml_roundtrip(&xml).unwrap_or_else(|e| {
        panic!(
            "Triangle MusicXML roundtrip failed for {}: {}",
            fixture_name, e
        )
    });
    let diffs = compare_scores(&mxml1, &mxml2);
    if !diffs.is_empty() {
        panic!(
            "Triangle MusicXML roundtrip failed for {} (export inconsistency): MusicXML₁ ≠ MusicXML₂\n{}",
            fixture_name,
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_assess_and_player_elements() {
    assert_spec_examples_roundtrip("assess_and_player_elements.musicxml");
}

#[test]
fn test_roundtrip_concert_score_and_for_part_elements() {
    assert_spec_examples_roundtrip("concert_score_and_for_part_elements.musicxml");
}

#[test]
fn test_roundtrip_instrument_change_element() {
    assert_spec_examples_roundtrip("instrument_change_element.musicxml");
}

#[test]
fn test_roundtrip_movement_number_and_movement_title_elements() {
    assert_spec_examples_roundtrip("movement_number_and_movement_title_elements.musicxml");
}

#[test]
fn test_roundtrip_score_timewise_element() {
    assert_spec_examples_roundtrip("score_timewise_element.musicxml");
}

#[test]
fn test_roundtrip_tutorial_apres_un_reve() {
    assert_spec_examples_roundtrip("tutorial_apres_un_reve.musicxml");
}

#[test]
fn test_roundtrip_tutorial_chopin_prelude() {
    assert_spec_examples_roundtrip("tutorial_chopin_prelude.musicxml");
}

#[test]
fn test_roundtrip_tutorial_chord_symbols() {
    assert_spec_examples_roundtrip("tutorial_chord_symbols.musicxml");
}

#[test]
fn test_roundtrip_tutorial_hello_world() {
    assert_spec_examples_roundtrip("tutorial_hello_world.musicxml");
}

#[test]
fn test_roundtrip_tutorial_percussion() {
    assert_spec_examples_roundtrip("tutorial_percussion.musicxml");
}

#[test]
fn test_roundtrip_tutorial_tablature() {
    assert_spec_examples_roundtrip("tutorial_tablature.musicxml");
}
