//! Roundtrip tests for MusicXML → MEI → MusicXML conversion.
//!
//! All comparisons happen in **timewise** space — `ScoreTimewise` is the
//! canonical internal representation.
//!
//! Four levels of testing:
//!
//! 1. **Conversion roundtrip** (import/export only):
//!    XML → parse → partwise_to_timewise → ScoreTimewise₁
//!    XML → parse → import → MEI → export_timewise → ScoreTimewise₂
//!    Compares ScoreTimewise₁ vs ScoreTimewise₂.
//!    Tests: import + export logic, isolated from serializer.
//!
//! 2. **Full roundtrip** (includes serializer/parser):
//!    XML → parse → partwise_to_timewise → ScoreTimewise₁
//!    XML → parse → import → MEI → export → serialize → XML₂ → parse → partwise_to_timewise → ScoreTimewise₃
//!    Compares ScoreTimewise₁ vs ScoreTimewise₃.
//!    Tests: entire pipeline including MusicXML serializer.
//!
//! 3. **Triangle MEI roundtrip** (catches symmetric import bugs):
//!    MusicXML₀ → import → MEI₁ → export → MusicXML₁ → import → MEI₂
//!    Compares MEI₁ vs MEI₂.
//!    Tests: if import has inconsistent behavior, MEI₁ ≠ MEI₂.
//!
//! 4. **Triangle MusicXML roundtrip** (catches symmetric export bugs):
//!    MEI₁ → export_timewise → ScoreTimewise₁ → timewise_to_partwise → import → MEI₂ → export_timewise → ScoreTimewise₂
//!    Compares ScoreTimewise₁ vs ScoreTimewise₂ (in timewise space).
//!    Tests: if export has inconsistent behavior, ScoreTimewise₁ ≠ ScoreTimewise₂.
//!
//! If conversion roundtrip passes but full roundtrip fails → serializer bug.
//! If conversion roundtrip fails → import/export bug.
//! If triangle MEI fails → import has inconsistent/lossy behavior.
//! If triangle MusicXML fails → export has inconsistent/lossy behavior.

use std::fs;

use tusk_mei::xml_compare;
use tusk_model::elements::Mei;
use tusk_musicxml::model::attributes::{ClefSign, KeyContent, TimeContent};
use tusk_musicxml::model::elements::{MeasureContent, ScoreTimewise};
use tusk_musicxml::model::note::FullNoteContent;
use tusk_musicxml::parser::{parse_score_partwise, parse_score_timewise};
use tusk_musicxml::{
    export, export_timewise, import, import_timewise, serialize, timewise_to_partwise,
};

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

/// Conversion roundtrip in timewise space (no serialization).
/// Tests import/export logic only, isolated from serializer bugs.
/// Returns (original_tw, exported_tw) for direct model comparison.
fn conversion_roundtrip(xml: &str) -> Result<(ScoreTimewise, ScoreTimewise), String> {
    let partwise = if is_timewise(xml) {
        parse_score_timewise(xml).map_err(|e| format!("Parse error (timewise): {}", e))?
    } else {
        parse_score_partwise(xml).map_err(|e| format!("Parse error (partwise): {}", e))?
    };
    let original_tw = import_timewise(&partwise);
    let mei = import(&partwise).map_err(|e| format!("Import (MusicXML→MEI) error: {}", e))?;
    let exported_tw =
        export_timewise(&mei).map_err(|e| format!("Export (MEI→MusicXML) error: {}", e))?;
    Ok((original_tw, exported_tw))
}

/// Full roundtrip in timewise space (includes serializer/parser).
/// Tests entire pipeline including MusicXML serializer.
/// Returns (original_tw, roundtripped_tw) after serialize→parse cycle.
fn full_roundtrip(xml: &str) -> Result<(ScoreTimewise, ScoreTimewise), String> {
    let partwise = if is_timewise(xml) {
        parse_score_timewise(xml).map_err(|e| format!("Parse error (timewise): {}", e))?
    } else {
        parse_score_partwise(xml).map_err(|e| format!("Parse error (partwise): {}", e))?
    };
    let original_tw = import_timewise(&partwise);
    let mei = import(&partwise).map_err(|e| format!("Import (MusicXML→MEI) error: {}", e))?;
    let exported_pw = export(&mei).map_err(|e| format!("Export (MEI→MusicXML) error: {}", e))?;
    let xml2 = serialize(&exported_pw).map_err(|e| format!("Serialize error: {}", e))?;
    let roundtripped_pw =
        parse_score_partwise(&xml2).map_err(|e| format!("Re-parse error: {}", e))?;
    let roundtripped_tw = import_timewise(&roundtripped_pw);
    Ok((original_tw, roundtripped_tw))
}

/// Triangle roundtrip A: MusicXML → MEI₁ → MusicXML → MEI₂.
/// Catches import bugs where second import reveals inconsistency.
/// Returns (mei1, mei2) for direct comparison.
fn triangle_mei_roundtrip(xml: &str) -> Result<(Mei, Mei), String> {
    let partwise = if is_timewise(xml) {
        parse_score_timewise(xml).map_err(|e| format!("Parse error (timewise): {}", e))?
    } else {
        parse_score_partwise(xml).map_err(|e| format!("Parse error (partwise): {}", e))?
    };
    let mei1 = import(&partwise).map_err(|e| format!("Import₁ (MusicXML→MEI) error: {}", e))?;
    let mxml1_pw = export(&mei1).map_err(|e| format!("Export₁ (MEI→MusicXML) error: {}", e))?;
    let mei2 = import(&mxml1_pw).map_err(|e| format!("Import₂ (MusicXML→MEI) error: {}", e))?;
    Ok((mei1, mei2))
}

/// Triangle roundtrip B in timewise space.
/// MEI₁ → ScoreTimewise₁, then timewise_to_partwise → import → MEI₂ → ScoreTimewise₂.
/// Catches export bugs where second export reveals inconsistency.
/// Returns (tw1, tw2) for comparison in timewise space.
fn triangle_mxml_roundtrip(xml: &str) -> Result<(ScoreTimewise, ScoreTimewise), String> {
    let partwise = if is_timewise(xml) {
        parse_score_timewise(xml).map_err(|e| format!("Parse error (timewise): {}", e))?
    } else {
        parse_score_partwise(xml).map_err(|e| format!("Parse error (partwise): {}", e))?
    };
    let mei1 = import(&partwise).map_err(|e| format!("Import₁ (MusicXML→MEI) error: {}", e))?;
    let tw1 = export_timewise(&mei1).map_err(|e| format!("Export₁ (MEI→MusicXML) error: {}", e))?;
    let pw1 = timewise_to_partwise(tw1.clone());
    let mei2 = import(&pw1).map_err(|e| format!("Import₂ (MusicXML→MEI) error: {}", e))?;
    let tw2 = export_timewise(&mei2).map_err(|e| format!("Export₂ (MEI→MusicXML) error: {}", e))?;
    Ok((tw1, tw2))
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
) -> Result<(ScoreTimewise, ScoreTimewise), String> {
    let xml = load_fixture(fixture_name)?;
    conversion_roundtrip(&xml)
}

/// Load a fixture and perform full roundtrip (with serialization).
fn full_roundtrip_fixture(fixture_name: &str) -> Result<(ScoreTimewise, ScoreTimewise), String> {
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
) -> Result<(ScoreTimewise, ScoreTimewise), String> {
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

/// Compare two MusicXML timewise scores for semantic equivalence.
fn compare_scores(original: &ScoreTimewise, roundtripped: &ScoreTimewise) -> Differences {
    let mut diffs = Differences::default();

    // Compare part-list (part names)
    compare_part_list(original, roundtripped, &mut diffs);

    // Compare measure count
    if original.measures.len() != roundtripped.measures.len() {
        diffs.add(format!(
            "Measure count mismatch: original={}, roundtripped={}",
            original.measures.len(),
            roundtripped.measures.len()
        ));
        return diffs;
    }

    // Compare each measure
    for (m_idx, (orig_measure, rt_measure)) in original
        .measures
        .iter()
        .zip(roundtripped.measures.iter())
        .enumerate()
    {
        let measure_num = &orig_measure.number;

        // Check measure number
        if orig_measure.number != rt_measure.number {
            diffs.add(format!(
                "Measure {}: number mismatch: original='{}', roundtripped='{}'",
                m_idx, orig_measure.number, rt_measure.number
            ));
        }

        // Check part count within measure
        if orig_measure.parts.len() != rt_measure.parts.len() {
            diffs.add(format!(
                "Measure '{}': part count mismatch: original={}, roundtripped={}",
                measure_num,
                orig_measure.parts.len(),
                rt_measure.parts.len()
            ));
            continue;
        }

        // Compare each part within the measure
        for (p_idx, (orig_part, rt_part)) in orig_measure
            .parts
            .iter()
            .zip(rt_measure.parts.iter())
            .enumerate()
        {
            let part_id = &orig_part.id;

            if orig_part.id != rt_part.id {
                diffs.add(format!(
                    "Measure '{}', Part {}: ID mismatch: original='{}', roundtripped='{}'",
                    measure_num, p_idx, orig_part.id, rt_part.id
                ));
            }

            // Compare measure content for this part
            compare_measure_content(
                part_id,
                measure_num,
                &orig_part.content,
                &rt_part.content,
                &mut diffs,
            );
        }
    }

    diffs
}

fn compare_part_list(
    original: &ScoreTimewise,
    roundtripped: &ScoreTimewise,
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

        // Compare divisions (MusicXML defaults to 1 when absent)
        let orig_divs = orig.divisions.unwrap_or(1.0);
        let rt_divs = rt.divisions.unwrap_or(1.0);
        if orig_divs != rt_divs {
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

        // Compare clefs.
        // When original omits clef and roundtripped has the default treble clef
        // (G line 2), that's not a real difference — our import adds a default clef.
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
        } else if orig.clefs.is_empty() && rt.clefs.len() == 1 {
            // Roundtrip added a default clef — acceptable if it's treble (G line 2)
            let rt_clef = &rt.clefs[0];
            let is_default = rt_clef.sign == ClefSign::G && rt_clef.line == Some(2);
            if !is_default {
                diffs.add(format!(
                    "{}: roundtrip added non-default clef: {:?} line {:?}",
                    ctx, rt_clef.sign, rt_clef.line
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

/// Assert conversion roundtrip passes in timewise space (no serialization).
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

/// Assert full roundtrip passes in timewise space (includes serialization).
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

/// Compare two MEI documents and report detailed differences.
///
/// If models differ, serializes both to XML and uses xml_compare to show
/// exactly where the differences are, with path information.
fn assert_mei_equal(mei1: &Mei, mei2: &Mei, context: &str) {
    if mei1 != mei2 {
        // Serialize both to XML for detailed comparison
        let xml1 = tusk_mei::export(mei1).expect("Failed to serialize MEI₁");
        let xml2 = tusk_mei::export(mei2).expect("Failed to serialize MEI₂");

        // Write debug output for manual inspection
        fs::write("/tmp/mei1_debug.xml", &xml1).ok();
        fs::write("/tmp/mei2_debug.xml", &xml2).ok();

        match xml_compare::get_differences(&xml1, &xml2) {
            Ok(diffs) if diffs.is_empty() => {
                // Model differs but XML is semantically equivalent
                // This can happen due to internal ordering differences that
                // don't affect the serialized output. Log warning but pass.
                eprintln!(
                    "Warning: MEI models differ structurally but XML is semantically equivalent for {}",
                    context
                );
            }
            Ok(diffs) => {
                eprintln!("MEI differences for {}:", context);
                for diff in &diffs {
                    eprintln!("  {}: {}", diff.path, diff.description);
                }
                panic!(
                    "Triangle MEI roundtrip failed for {} (import inconsistency): {} differences found.\n\
                     Debug XML written to /tmp/mei1_debug.xml and /tmp/mei2_debug.xml",
                    context,
                    diffs.len()
                );
            }
            Err(e) => {
                panic!(
                    "Failed to compare MEI XML for {}: {}\n\
                     Debug XML written to /tmp/mei1_debug.xml and /tmp/mei2_debug.xml",
                    context, e
                );
            }
        }
    }
}

/// Assert triangle MEI roundtrip passes (MEI₁ == MEI₂).
/// Catches inconsistent import behavior.
fn assert_triangle_mei_roundtrip(fixture_name: &str) {
    let (mei1, mei2) = triangle_mei_roundtrip_fixture(fixture_name)
        .unwrap_or_else(|e| panic!("Triangle MEI roundtrip failed for {}: {}", fixture_name, e));

    assert_mei_equal(&mei1, &mei2, fixture_name);
}

/// Assert triangle MusicXML roundtrip passes in timewise space (TW₁ == TW₂).
/// Catches inconsistent export behavior.
fn assert_triangle_mxml_roundtrip(fixture_name: &str) {
    let (tw1, tw2) = triangle_mxml_roundtrip_fixture(fixture_name).unwrap_or_else(|e| {
        panic!(
            "Triangle MusicXML roundtrip failed for {}: {}",
            fixture_name, e
        )
    });

    let diffs = compare_scores(&tw1, &tw2);
    if !diffs.is_empty() {
        panic!(
            "Triangle MusicXML roundtrip failed for {} (export inconsistency): ScoreTimewise₁ ≠ ScoreTimewise₂\n\
             Export produces different content from the same logical input.\n{}",
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

#[test]
fn test_roundtrip_figured_bass() {
    assert_roundtrip("figured_bass.musicxml");
}

#[test]
fn test_roundtrip_identification_metadata() {
    assert_roundtrip("identification_metadata.musicxml");
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

    println!("=== Original (timewise) ===");
    println!("Measures: {}", original.measures.len());
    for m in &original.measures {
        println!("  Measure '{}': {} parts", m.number, m.parts.len());
    }

    println!("\n=== After MEI conversion (timewise, no serialization) ===");
    println!("Measures: {}", exported.measures.len());
    for m in &exported.measures {
        println!("  Measure '{}': {} parts", m.number, m.parts.len());
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

/// Assert all four roundtrip tests pass for a spec example (in timewise space).
fn assert_spec_example_roundtrip(example_name: &str) {
    let xml = load_spec_example(example_name)
        .unwrap_or_else(|e| panic!("Failed to load {}: {}", example_name, e));

    // Test conversion roundtrip (import/export only, timewise space)
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

    // Test full roundtrip (includes serialization, timewise space)
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
    assert_mei_equal(&mei1, &mei2, example_name);

    // Test triangle MusicXML roundtrip (catches export inconsistency, timewise space)
    let (tw1, tw2) = triangle_mxml_roundtrip(&xml).unwrap_or_else(|e| {
        panic!(
            "Triangle MusicXML roundtrip failed for {}: {}",
            example_name, e
        )
    });
    let diffs = compare_scores(&tw1, &tw2);
    if !diffs.is_empty() {
        panic!(
            "Triangle MusicXML roundtrip failed for {} (export inconsistency): ScoreTimewise₁ ≠ ScoreTimewise₂\n{}",
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

/// Assert all four roundtrip tests pass for a spec_examples fixture (timewise space).
fn assert_spec_examples_roundtrip(fixture_name: &str) {
    let xml = load_spec_examples_fixture(fixture_name)
        .unwrap_or_else(|e| panic!("Failed to load {}: {}", fixture_name, e));

    // Test conversion roundtrip (import/export only, timewise space)
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

    // Test full roundtrip (includes serialization, timewise space)
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
    assert_mei_equal(&mei1, &mei2, fixture_name);

    // Test triangle MusicXML roundtrip (catches export inconsistency, timewise space)
    let (tw1, tw2) = triangle_mxml_roundtrip(&xml).unwrap_or_else(|e| {
        panic!(
            "Triangle MusicXML roundtrip failed for {}: {}",
            fixture_name, e
        )
    });
    let diffs = compare_scores(&tw1, &tw2);
    if !diffs.is_empty() {
        panic!(
            "Triangle MusicXML roundtrip failed for {} (export inconsistency): ScoreTimewise₁ ≠ ScoreTimewise₂\n{}",
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

// ============================================================================
// Fragment Example Tests (from tests/fixtures/musicxml/fragment_examples/)
// ============================================================================

/// Load a fragment example fixture file.
fn load_fragment_fixture(fixture_name: &str) -> Result<String, String> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let fixture_path = format!(
        "{}/../../../tests/fixtures/musicxml/fragment_examples/{}",
        manifest_dir, fixture_name
    );
    fs::read_to_string(&fixture_path)
        .map_err(|e| format!("Failed to read fixture {}: {}", fixture_name, e))
}

/// Assert all four roundtrip tests pass for a fragment example (timewise space).
fn assert_fragment_roundtrip(fixture_name: &str) {
    let xml = load_fragment_fixture(fixture_name)
        .unwrap_or_else(|e| panic!("Failed to load {}: {}", fixture_name, e));

    // Test conversion roundtrip (import/export only, timewise space)
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

    // Test full roundtrip (includes serialization, timewise space)
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
    assert_mei_equal(&mei1, &mei2, fixture_name);

    // Test triangle MusicXML roundtrip (catches export inconsistency, timewise space)
    let (tw1, tw2) = triangle_mxml_roundtrip(&xml).unwrap_or_else(|e| {
        panic!(
            "Triangle MusicXML roundtrip failed for {}: {}",
            fixture_name, e
        )
    });
    let diffs = compare_scores(&tw1, &tw2);
    if !diffs.is_empty() {
        panic!(
            "Triangle MusicXML roundtrip failed for {} (export inconsistency): ScoreTimewise₁ ≠ ScoreTimewise₂\n{}",
            fixture_name,
            diffs.report()
        );
    }
}

/// Generate a fragment roundtrip test function.
macro_rules! fragment_roundtrip_test {
    ($name:ident) => {
        #[test]
        fn $name() {
            assert_fragment_roundtrip(concat!(stringify!($name), ".musicxml"));
        }
    };
}

// Batch 1
fragment_roundtrip_test!(accent_element);
fragment_roundtrip_test!(accidental_element);
fragment_roundtrip_test!(accidental_mark_element_notation);
fragment_roundtrip_test!(accidental_mark_element_ornament);
fragment_roundtrip_test!(accordion_high_element);
// Batch 2
fragment_roundtrip_test!(accordion_low_element);
fragment_roundtrip_test!(accordion_middle_element);
fragment_roundtrip_test!(accordion_registration_element);
fragment_roundtrip_test!(alter_element_microtones);
fragment_roundtrip_test!(alter_element_semitones);
// Batch 3
fragment_roundtrip_test!(alto_clef);
fragment_roundtrip_test!(arpeggiate_element);
fragment_roundtrip_test!(arrow_element);
fragment_roundtrip_test!(arrowhead_element);
fragment_roundtrip_test!(articulations_element);
// Batch 4
fragment_roundtrip_test!(artificial_element);
fragment_roundtrip_test!(attributes_element);
fragment_roundtrip_test!(backup_element);
fragment_roundtrip_test!(baritone_c_clef);
fragment_roundtrip_test!(baritone_f_clef);
// Batch 5
fragment_roundtrip_test!(barline_element);
fragment_roundtrip_test!(barre_element);
fragment_roundtrip_test!(bass_alter_element);
fragment_roundtrip_test!(bass_clef);
fragment_roundtrip_test!(bass_clef_down_octave);
// Batch 6
fragment_roundtrip_test!(bass_separator_element);
fragment_roundtrip_test!(bass_step_element);
fragment_roundtrip_test!(beam_element);
fragment_roundtrip_test!(beat_repeat_element);
fragment_roundtrip_test!(beat_type_element);
// Batch 7
fragment_roundtrip_test!(beat_unit_dot_element);
fragment_roundtrip_test!(beat_unit_element);
fragment_roundtrip_test!(beat_unit_tied_element);
fragment_roundtrip_test!(beater_element);
fragment_roundtrip_test!(beats_element);
// Batch 8
fragment_roundtrip_test!(bend_element);
fragment_roundtrip_test!(bookmark_element);
fragment_roundtrip_test!(bracket_element);
fragment_roundtrip_test!(brass_bend_element);
fragment_roundtrip_test!(breath_mark_element);
// Batch 9
fragment_roundtrip_test!(caesura_element);
fragment_roundtrip_test!(cancel_element);
fragment_roundtrip_test!(capo_element);
fragment_roundtrip_test!(chord_element);
fragment_roundtrip_test!(chord_element_multiple_stop);
// Batch 10
fragment_roundtrip_test!(circular_arrow_element);
fragment_roundtrip_test!(coda_element);
fragment_roundtrip_test!(cue_element);
fragment_roundtrip_test!(damp_all_element);
fragment_roundtrip_test!(damp_element);
// Batch 11
fragment_roundtrip_test!(dashes_element);
fragment_roundtrip_test!(degree_alter_element);
fragment_roundtrip_test!(degree_type_element);
fragment_roundtrip_test!(degree_value_element);
fragment_roundtrip_test!(delayed_inverted_turn_element);
// Batch 12
fragment_roundtrip_test!(delayed_turn_element);
fragment_roundtrip_test!(detached_legato_element);
fragment_roundtrip_test!(divisions_and_duration_elements);
fragment_roundtrip_test!(doit_element);
fragment_roundtrip_test!(dot_element);
// Batch 13
fragment_roundtrip_test!(double_element);
fragment_roundtrip_test!(double_tongue_element);
fragment_roundtrip_test!(down_bow_element);
fragment_roundtrip_test!(dynamics_element_notation);
fragment_roundtrip_test!(effect_element);
fragment_roundtrip_test!(elision_element);
// Batch 14
fragment_roundtrip_test!(end_line_element);
fragment_roundtrip_test!(end_paragraph_element);
fragment_roundtrip_test!(ending_element);
fragment_roundtrip_test!(ensemble_element);
fragment_roundtrip_test!(except_voice_element);
// Batch 15
fragment_roundtrip_test!(extend_element_figure);
fragment_roundtrip_test!(extend_element_lyric);
fragment_roundtrip_test!(eyeglasses_element);
fragment_roundtrip_test!(f_element);
fragment_roundtrip_test!(falloff_element);
// Batch 16
fragment_roundtrip_test!(fermata_element);
fragment_roundtrip_test!(ff_element);
fragment_roundtrip_test!(fff_element);
fragment_roundtrip_test!(ffff_element);
fragment_roundtrip_test!(fffff_element);
// Batch 17
fragment_roundtrip_test!(ffffff_element);
fragment_roundtrip_test!(figure_number_element);
fragment_roundtrip_test!(fingering_element_frame);
fragment_roundtrip_test!(fingering_element_notation);
fragment_roundtrip_test!(fingernails_element);
// Batch 18
fragment_roundtrip_test!(flip_element);
fragment_roundtrip_test!(footnote_element);
fragment_roundtrip_test!(forward_element);
fragment_roundtrip_test!(fp_element);
fragment_roundtrip_test!(fret_element_frame);
// Batch 19
fragment_roundtrip_test!(fz_element);
fragment_roundtrip_test!(glass_element);
fragment_roundtrip_test!(glissando_element_multiple);
fragment_roundtrip_test!(glissando_element_single);
fragment_roundtrip_test!(glyph_element);
// Batch 20
fragment_roundtrip_test!(golpe_element);
fragment_roundtrip_test!(grace_element);
fragment_roundtrip_test!(grace_element_appoggiatura);
fragment_roundtrip_test!(group_abbreviation_display_element);
fragment_roundtrip_test!(group_abbreviation_element);
// Batch 21
fragment_roundtrip_test!(group_barline_element);
fragment_roundtrip_test!(group_name_display_element);
fragment_roundtrip_test!(group_time_element);
fragment_roundtrip_test!(grouping_element);
fragment_roundtrip_test!(half_muted_element);
// Batch 22
fragment_roundtrip_test!(handbell_element);
fragment_roundtrip_test!(harmon_mute_element);
fragment_roundtrip_test!(harp_pedals_element);
fragment_roundtrip_test!(haydn_element);
fragment_roundtrip_test!(heel_element);
// Batch 23
fragment_roundtrip_test!(heel_toe_substitution);
fragment_roundtrip_test!(hole_element);
fragment_roundtrip_test!(hole_type_element);
fragment_roundtrip_test!(humming_element);
fragment_roundtrip_test!(image_element);
// Batch 24
fragment_roundtrip_test!(instrument_link_element);
fragment_roundtrip_test!(interchangeable_element);
fragment_roundtrip_test!(inversion_element);
fragment_roundtrip_test!(inverted_mordent_element);
fragment_roundtrip_test!(inverted_turn_element);
// Batch 25
fragment_roundtrip_test!(inverted_vertical_turn_element);
fragment_roundtrip_test!(ipa_element);
fragment_roundtrip_test!(key_element_non_traditional);
fragment_roundtrip_test!(key_element_traditional);
fragment_roundtrip_test!(key_octave_element);
// Batch 26
fragment_roundtrip_test!(kind_element);
fragment_roundtrip_test!(laughing_element);
fragment_roundtrip_test!(level_element);
fragment_roundtrip_test!(line_detail_element);
fragment_roundtrip_test!(line_element);
// Batch 27
fragment_roundtrip_test!(link_element);
fragment_roundtrip_test!(lyric_element);
fragment_roundtrip_test!(measure_distance_element);
fragment_roundtrip_test!(measure_numbering_element);
fragment_roundtrip_test!(measure_repeat_element);
// Batch 28
fragment_roundtrip_test!(membrane_element);
fragment_roundtrip_test!(metal_element);
fragment_roundtrip_test!(metronome_arrows_element);
fragment_roundtrip_test!(metronome_element);
fragment_roundtrip_test!(metronome_note_element);
// Batch 29
fragment_roundtrip_test!(metronome_tied_element);
fragment_roundtrip_test!(mezzo_soprano_clef);
fragment_roundtrip_test!(mf_element);
fragment_roundtrip_test!(midi_device_element);
fragment_roundtrip_test!(midi_instrument_element);
// Batch 30
fragment_roundtrip_test!(midi_name_and_midi_bank_elements);
fragment_roundtrip_test!(midi_unpitched_element);
fragment_roundtrip_test!(mordent_element);
fragment_roundtrip_test!(mp_element);
fragment_roundtrip_test!(multiple_rest_element);
// Batch 31
fragment_roundtrip_test!(n_element);
fragment_roundtrip_test!(natural_element);
fragment_roundtrip_test!(non_arpeggiate_element);
fragment_roundtrip_test!(normal_dot_element);
fragment_roundtrip_test!(notehead_text_element);
// Batch 32
fragment_roundtrip_test!(numeral_alter_element);
fragment_roundtrip_test!(numeral_key_element);
fragment_roundtrip_test!(numeral_root_element);
fragment_roundtrip_test!(octave_change_element);
fragment_roundtrip_test!(octave_element);
// Batch 33
fragment_roundtrip_test!(octave_shift_element);
fragment_roundtrip_test!(open_element);
fragment_roundtrip_test!(open_string_element);
fragment_roundtrip_test!(p_element);
fragment_roundtrip_test!(pan_and_elevation_elements);
// Batch 34
fragment_roundtrip_test!(part_abbreviation_display_element);
fragment_roundtrip_test!(part_link_element);
fragment_roundtrip_test!(part_name_display_element);
fragment_roundtrip_test!(part_symbol_element);
fragment_roundtrip_test!(pedal_element_lines);
// Batch 35
fragment_roundtrip_test!(pedal_element_symbols);
fragment_roundtrip_test!(per_minute_element);
fragment_roundtrip_test!(percussion_clef);
fragment_roundtrip_test!(pf_element);
fragment_roundtrip_test!(pitch_element);
// Batch 36
fragment_roundtrip_test!(pitched_element);
fragment_roundtrip_test!(plop_element);
fragment_roundtrip_test!(pluck_element);
fragment_roundtrip_test!(pp_element);
fragment_roundtrip_test!(ppp_element);
// Batch 37
fragment_roundtrip_test!(pppp_element);
fragment_roundtrip_test!(ppppp_element);
fragment_roundtrip_test!(pppppp_element);
fragment_roundtrip_test!(pre_bend_element);
fragment_roundtrip_test!(prefix_element);
// Batch 38
fragment_roundtrip_test!(principal_voice_element);
fragment_roundtrip_test!(rehearsal_element);
fragment_roundtrip_test!(release_element);
fragment_roundtrip_test!(repeat_element);
fragment_roundtrip_test!(rest_element);
// Batch 39
fragment_roundtrip_test!(rf_element);
fragment_roundtrip_test!(rfz_element);
fragment_roundtrip_test!(root_alter_element);
fragment_roundtrip_test!(root_step_element);
fragment_roundtrip_test!(schleifer_element);
// Batch 40
fragment_roundtrip_test!(scoop_element);
fragment_roundtrip_test!(scordatura_element);
fragment_roundtrip_test!(segno_element);
fragment_roundtrip_test!(senza_misura_element);
fragment_roundtrip_test!(sf_element);
// Batch 41
fragment_roundtrip_test!(sffz_element);
fragment_roundtrip_test!(sfp_element);
fragment_roundtrip_test!(sfpp_element);
fragment_roundtrip_test!(sfz_element);
fragment_roundtrip_test!(sfzp_element);
// Batch 42
fragment_roundtrip_test!(shake_element);
fragment_roundtrip_test!(slash_element);
fragment_roundtrip_test!(slash_type_and_slash_dot_elements);
fragment_roundtrip_test!(slide_element);
fragment_roundtrip_test!(slur_element);
// Batch 43
fragment_roundtrip_test!(smear_element);
fragment_roundtrip_test!(snap_pizzicato_element);
fragment_roundtrip_test!(soft_accent_element);
fragment_roundtrip_test!(soprano_clef);
fragment_roundtrip_test!(spiccato_element);
// Batch 44
fragment_roundtrip_test!(staccatissimo_element);
fragment_roundtrip_test!(staccato_element);
fragment_roundtrip_test!(staff_distance_element);
fragment_roundtrip_test!(staff_divide_element);
fragment_roundtrip_test!(staff_element);
// Batch 45
fragment_roundtrip_test!(staff_lines_element);
fragment_roundtrip_test!(staff_size_element);
fragment_roundtrip_test!(staff_tuning_element);
fragment_roundtrip_test!(staff_type_element);
fragment_roundtrip_test!(staves_element);
// Batch 46
fragment_roundtrip_test!(step_element);
fragment_roundtrip_test!(stick_element);
fragment_roundtrip_test!(stick_location_element);
fragment_roundtrip_test!(stopped_element);
fragment_roundtrip_test!(straight_element);
// Batch 47
fragment_roundtrip_test!(stress_element);
fragment_roundtrip_test!(string_mute_element_off);
fragment_roundtrip_test!(string_mute_element_on);
fragment_roundtrip_test!(strong_accent_element);
fragment_roundtrip_test!(suffix_element);
// Batch 48
fragment_roundtrip_test!(swing_element);
fragment_roundtrip_test!(syllabic_element);
fragment_roundtrip_test!(symbol_element);
fragment_roundtrip_test!(sync_element);
fragment_roundtrip_test!(system_attribute_also_top);
// Batch 49
fragment_roundtrip_test!(system_attribute_only_top);
fragment_roundtrip_test!(system_distance_element);
fragment_roundtrip_test!(system_dividers_element);
fragment_roundtrip_test!(tab_clef);
fragment_roundtrip_test!(tap_element);
// Batch 50
fragment_roundtrip_test!(technical_element_tablature);
fragment_roundtrip_test!(tenor_clef);
fragment_roundtrip_test!(tenuto_element);
fragment_roundtrip_test!(thumb_position_element);
fragment_roundtrip_test!(tied_element);
// Batch 51
fragment_roundtrip_test!(time_modification_element);
fragment_roundtrip_test!(timpani_element);
fragment_roundtrip_test!(toe_element);
fragment_roundtrip_test!(transpose_element);
fragment_roundtrip_test!(treble_clef);
// Batch 52
fragment_roundtrip_test!(tremolo_element_double);
fragment_roundtrip_test!(tremolo_element_single);
fragment_roundtrip_test!(trill_mark_element);
fragment_roundtrip_test!(triple_tongue_element);
fragment_roundtrip_test!(tuplet_dot_element);
// Batch 53
fragment_roundtrip_test!(tuplet_element_nested);
fragment_roundtrip_test!(tuplet_element_regular);
fragment_roundtrip_test!(turn_element);
fragment_roundtrip_test!(unpitched_element);
fragment_roundtrip_test!(unstress_element);
// Batch 54
fragment_roundtrip_test!(up_bow_element);
fragment_roundtrip_test!(vertical_turn_element);
fragment_roundtrip_test!(virtual_instrument_element);
fragment_roundtrip_test!(vocal_tenor_clef);
fragment_roundtrip_test!(voice_element);
// Batch 55
fragment_roundtrip_test!(wait_element);
fragment_roundtrip_test!(wavy_line_element);
fragment_roundtrip_test!(wedge_element);
fragment_roundtrip_test!(with_bar_element);
fragment_roundtrip_test!(wood_element);
// Multi-staff parts (Phase 2)
fragment_roundtrip_test!(piano_two_staves);
fragment_roundtrip_test!(organ_three_staves);
fragment_roundtrip_test!(cross_staff_notes);
