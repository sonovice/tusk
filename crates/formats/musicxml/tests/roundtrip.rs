//! Roundtrip tests for MusicXML → MEI → MusicXML conversion.
//!
//! These tests verify that MusicXML files can be:
//! 1. Parsed into MusicXML model types
//! 2. Converted to MEI (internal model)
//! 3. Converted back to MusicXML
//! 4. Compared for semantic equivalence with the original

use std::fs;

use tusk_musicxml::model::attributes::{KeyContent, TimeContent};
use tusk_musicxml::model::elements::{MeasureContent, ScorePartwise};
use tusk_musicxml::model::note::FullNoteContent;
use tusk_musicxml::parser::parse_score_partwise;
use tusk_musicxml::{export, import};

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

/// Perform a full roundtrip: MusicXML → MEI → MusicXML
/// Returns the original parsed score and the roundtripped score.
fn roundtrip(xml: &str) -> Result<(ScorePartwise, ScorePartwise), String> {
    // Step 1: Parse original MusicXML
    let original = parse_score_partwise(xml).map_err(|e| format!("Parse error: {}", e))?;

    // Step 2: Convert to MEI
    let mei = import(&original).map_err(|e| format!("Import (MusicXML→MEI) error: {}", e))?;

    // Step 3: Convert back to MusicXML
    let roundtripped = export(&mei).map_err(|e| format!("Export (MEI→MusicXML) error: {}", e))?;

    Ok((original, roundtripped))
}

/// Load a fixture file and perform roundtrip test.
fn roundtrip_fixture(fixture_name: &str) -> Result<(ScorePartwise, ScorePartwise), String> {
    // Build path relative to the crate manifest directory
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let fixture_path = format!(
        "{}/../../../tests/fixtures/musicxml/{}",
        manifest_dir, fixture_name
    );
    let xml = fs::read_to_string(&fixture_path)
        .map_err(|e| format!("Failed to read fixture {}: {}", fixture_name, e))?;
    roundtrip(&xml)
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

/// Assert that a roundtrip test passes with no differences.
fn assert_roundtrip(fixture_name: &str) {
    let (original, roundtripped) = roundtrip_fixture(fixture_name)
        .unwrap_or_else(|e| panic!("Roundtrip failed for {}: {}", fixture_name, e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for {}:\n{}",
            fixture_name,
            diffs.report()
        );
    }
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
    let (original, roundtripped) =
        roundtrip_fixture(fixture_name).unwrap_or_else(|e| panic!("Roundtrip failed: {}", e));

    println!("=== Original ===");
    println!("Parts: {}", original.parts.len());
    for part in &original.parts {
        println!("  Part '{}': {} measures", part.id, part.measures.len());
    }

    println!("\n=== Roundtripped ===");
    println!("Parts: {}", roundtripped.parts.len());
    for part in &roundtripped.parts {
        println!("  Part '{}': {} measures", part.id, part.measures.len());
    }
}

// ============================================================================
// Spec Examples Tests (from specs/musicxml/examples/)
// ============================================================================

/// Load a spec example file and perform roundtrip test.
fn roundtrip_spec_example(example_name: &str) -> Result<(ScorePartwise, ScorePartwise), String> {
    // Build path relative to the crate manifest directory
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let fixture_path = format!(
        "{}/../../../specs/musicxml/examples/{}",
        manifest_dir, example_name
    );
    let xml = read_xml_file(&fixture_path)?;
    roundtrip(&xml)
}

#[test]
fn test_roundtrip_spec_telemann() {
    let (original, roundtripped) = roundtrip_spec_example("Telemann.musicxml")
        .unwrap_or_else(|e| panic!("Roundtrip failed for Telemann: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for Telemann.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_spec_binchois() {
    let (original, roundtripped) = roundtrip_spec_example("Binchois.musicxml")
        .unwrap_or_else(|e| panic!("Roundtrip failed for Binchois: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for Binchois.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_spec_mozart_piano_sonata() {
    let (original, roundtripped) = roundtrip_spec_example("MozartPianoSonata.musicxml")
        .unwrap_or_else(|e| panic!("Roundtrip failed for MozartPianoSonata: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for MozartPianoSonata.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_spec_actor_prelude_sample() {
    let (original, roundtripped) = roundtrip_spec_example("ActorPreludeSample.musicxml")
        .unwrap_or_else(|e| panic!("Roundtrip failed for ActorPreludeSample: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for ActorPreludeSample.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_spec_beet_an_ge_sample() {
    let (original, roundtripped) = roundtrip_spec_example("BeetAnGeSample.musicxml")
        .unwrap_or_else(|e| panic!("Roundtrip failed for BeetAnGeSample: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for BeetAnGeSample.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_spec_brah_wi_me_sample() {
    let (original, roundtripped) = roundtrip_spec_example("BrahWiMeSample.musicxml")
        .unwrap_or_else(|e| panic!("Roundtrip failed for BrahWiMeSample: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for BrahWiMeSample.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_spec_brooke_west_sample() {
    let (original, roundtripped) = roundtrip_spec_example("BrookeWestSample.musicxml")
        .unwrap_or_else(|e| panic!("Roundtrip failed for BrookeWestSample: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for BrookeWestSample.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_spec_chant() {
    let (original, roundtripped) = roundtrip_spec_example("Chant.musicxml")
        .unwrap_or_else(|e| panic!("Roundtrip failed for Chant: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for Chant.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_spec_debu_mand_sample() {
    let (original, roundtripped) = roundtrip_spec_example("DebuMandSample.musicxml")
        .unwrap_or_else(|e| panic!("Roundtrip failed for DebuMandSample: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for DebuMandSample.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_spec_dichterliebe01() {
    let (original, roundtripped) = roundtrip_spec_example("Dichterliebe01.musicxml")
        .unwrap_or_else(|e| panic!("Roundtrip failed for Dichterliebe01: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for Dichterliebe01.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_spec_echigo_jishi() {
    let (original, roundtripped) = roundtrip_spec_example("Echigo-Jishi.musicxml")
        .unwrap_or_else(|e| panic!("Roundtrip failed for Echigo-Jishi: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for Echigo-Jishi.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_spec_faur_reve_sample() {
    let (original, roundtripped) = roundtrip_spec_example("FaurReveSample.musicxml")
        .unwrap_or_else(|e| panic!("Roundtrip failed for FaurReveSample: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for FaurReveSample.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_spec_mahl_fa_ge4_sample() {
    let (original, roundtripped) = roundtrip_spec_example("MahlFaGe4Sample.musicxml")
        .unwrap_or_else(|e| panic!("Roundtrip failed for MahlFaGe4Sample: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for MahlFaGe4Sample.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_spec_moza_chlo_sample() {
    let (original, roundtripped) = roundtrip_spec_example("MozaChloSample.musicxml")
        .unwrap_or_else(|e| panic!("Roundtrip failed for MozaChloSample: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for MozaChloSample.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_spec_mozart_trio() {
    let (original, roundtripped) = roundtrip_spec_example("MozartTrio.musicxml")
        .unwrap_or_else(|e| panic!("Roundtrip failed for MozartTrio: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for MozartTrio.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_spec_moza_veil_sample() {
    let (original, roundtripped) = roundtrip_spec_example("MozaVeilSample.musicxml")
        .unwrap_or_else(|e| panic!("Roundtrip failed for MozaVeilSample: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for MozaVeilSample.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_spec_saltarello() {
    let (original, roundtripped) = roundtrip_spec_example("Saltarello.musicxml")
        .unwrap_or_else(|e| panic!("Roundtrip failed for Saltarello: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for Saltarello.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_spec_schb_av_ma_sample() {
    let (original, roundtripped) = roundtrip_spec_example("SchbAvMaSample.musicxml")
        .unwrap_or_else(|e| panic!("Roundtrip failed for SchbAvMaSample: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for SchbAvMaSample.musicxml:\n{}",
            diffs.report()
        );
    }
}

// ============================================================================
// Spec Doc Example Tests (from tests/fixtures/musicxml/spec_examples/)
// ============================================================================

/// Load a spec_examples fixture file and perform roundtrip test.
fn roundtrip_spec_examples_fixture(
    fixture_name: &str,
) -> Result<(ScorePartwise, ScorePartwise), String> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let fixture_path = format!(
        "{}/../../../tests/fixtures/musicxml/spec_examples/{}",
        manifest_dir, fixture_name
    );
    let xml = fs::read_to_string(&fixture_path)
        .map_err(|e| format!("Failed to read fixture {}: {}", fixture_name, e))?;
    roundtrip(&xml)
}

#[test]
fn test_roundtrip_assess_and_player_elements() {
    let (original, roundtripped) =
        roundtrip_spec_examples_fixture("assess_and_player_elements.musicxml")
            .unwrap_or_else(|e| panic!("Roundtrip failed for assess_and_player_elements: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for assess_and_player_elements.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_concert_score_and_for_part_elements() {
    let (original, roundtripped) =
        roundtrip_spec_examples_fixture("concert_score_and_for_part_elements.musicxml")
            .unwrap_or_else(|e| {
                panic!(
                    "Roundtrip failed for concert_score_and_for_part_elements: {}",
                    e
                )
            });

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for concert_score_and_for_part_elements.musicxml:\n{}",
            diffs.report()
        );
    }
}

#[test]
fn test_roundtrip_instrument_change_element() {
    let (original, roundtripped) =
        roundtrip_spec_examples_fixture("instrument_change_element.musicxml")
            .unwrap_or_else(|e| panic!("Roundtrip failed for instrument_change_element: {}", e));

    let diffs = compare_scores(&original, &roundtripped);
    if !diffs.is_empty() {
        panic!(
            "Roundtrip differences found for instrument_change_element.musicxml:\n{}",
            diffs.report()
        );
    }
}
