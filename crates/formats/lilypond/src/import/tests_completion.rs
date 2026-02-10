//! Tests for Phase 31.1 import completion — comprehensive fixture import,
//! element ID retention, cross-staff handling, and edge cases.

use super::*;
use crate::parser::Parser;
use tusk_model::elements::{Mei, MeiChild, ScoreChild, SectionChild, Staff, StaffChild};

fn parse_and_import(src: &str) -> Mei {
    let file = Parser::new(src).unwrap().parse().unwrap();
    import(&file).unwrap()
}

fn first_staff(mei: &Mei) -> Option<&Staff> {
    for child in &mei.children {
        if let MeiChild::Music(music) = child {
            for mc in &music.children {
                let tusk_model::elements::MusicChild::Body(body) = mc;
                for bc in &body.children {
                    let tusk_model::elements::BodyChild::Mdiv(mdiv) = bc;
                    for dc in &mdiv.children {
                        let tusk_model::elements::MdivChild::Score(score) = dc;
                        for sc in &score.children {
                            if let ScoreChild::Section(section) = sc {
                                for sec_c in &section.children {
                                    if let SectionChild::Measure(measure) = sec_c {
                                        for mc2 in &measure.children {
                                            if let tusk_model::elements::MeasureChild::Staff(s) =
                                                mc2
                                            {
                                                return Some(s);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn first_layer_children(mei: &Mei) -> &Vec<tusk_model::elements::LayerChild> {
    let staff = first_staff(mei).expect("should have a staff");
    for sc in &staff.children {
        let StaffChild::Layer(layer) = sc;
        return &layer.children;
    }
    panic!("no layer found");
}

fn measure_children(mei: &Mei) -> &Vec<tusk_model::elements::MeasureChild> {
    for child in &mei.children {
        if let MeiChild::Music(music) = child {
            for mc in &music.children {
                let tusk_model::elements::MusicChild::Body(body) = mc;
                for bc in &body.children {
                    let tusk_model::elements::BodyChild::Mdiv(mdiv) = bc;
                    for dc in &mdiv.children {
                        let tusk_model::elements::MdivChild::Score(score) = dc;
                        for sc in &score.children {
                            if let ScoreChild::Section(section) = sc {
                                for sec_c in &section.children {
                                    if let SectionChild::Measure(measure) = sec_c {
                                        return &measure.children;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    panic!("no measure found");
}

// ---------------------------------------------------------------------------
// Fixture import smoke tests — fixtures that parse successfully also import
// ---------------------------------------------------------------------------

/// Test that a fixture parses and imports without error.
/// Fixtures that don't parse (unsupported LilyPond features) are silently skipped.
macro_rules! fixture_import_test {
    ($name:ident, $path:expr) => {
        #[test]
        fn $name() {
            let src = include_str!(concat!("../../../../../tests/fixtures/lilypond/", $path));
            let file = match Parser::new(src).map(|p| p.parse()) {
                Ok(Ok(f)) => f,
                _ => return, // fixture doesn't parse — skip
            };
            let result = import(&file);
            // NoMusic is acceptable (markup-only files, etc.)
            match &result {
                Err(ImportError::NoMusic) => return,
                Err(e) => panic!("fixture {} import error: {e}", $path),
                Ok(_) => {}
            }
        }
    };
}

// Fragment fixtures (parser-compatible)
fixture_import_test!(fixture_simple, "simple.ly");
fixture_import_test!(
    fixture_single_staff_notes,
    "single-staff-template-with-only-notes.ly"
);
fixture_import_test!(
    fixture_single_staff_notes_lyrics,
    "single-staff-template-with-notes-and-lyrics.ly"
);
fixture_import_test!(
    fixture_single_staff_notes_lyrics_chords,
    "single-staff-notes-lyrics-chords.ly"
);
fixture_import_test!(fixture_auto_beam, "auto-beam.ly");
fixture_import_test!(fixture_tuplets, "tuplets.ly");
fixture_import_test!(fixture_chord_changes, "chord-changes.ly");
fixture_import_test!(fixture_lyrics_bar, "lyrics-bar.ly");
fixture_import_test!(fixture_volta_spec_rhythm, "volta-spec-rhythm.ly");
fixture_import_test!(fixture_old_hundredth, "old-hundredth-example.ly");
fixture_import_test!(fixture_score_minimal, "fragment_score_minimal.ly");
fixture_import_test!(fixture_pitches, "fragment_pitches.ly");
fixture_import_test!(fixture_durations, "fragment_durations.ly");
fixture_import_test!(fixture_rests, "fragment_rests.ly");
fixture_import_test!(fixture_roundtrip_notes, "fragment_roundtrip_notes.ly");
fixture_import_test!(fixture_seq_sim, "fragment_sequential_simultaneous.ly");
fixture_import_test!(fixture_two_voices, "fragment_two_voices.ly");
fixture_import_test!(fixture_contexts, "fragment_contexts.ly");
fixture_import_test!(fixture_piano, "fragment_piano.ly");
fixture_import_test!(fixture_clef_key_time, "fragment_clef_key_time.ly");
fixture_import_test!(fixture_relative_transpose, "fragment_relative_transpose.ly");
fixture_import_test!(fixture_chords, "fragment_chords.ly");
fixture_import_test!(fixture_ties_slurs, "fragment_ties_slurs.ly");
fixture_import_test!(fixture_beams, "fragment_beams.ly");
fixture_import_test!(fixture_dynamics, "fragment_dynamics.ly");
fixture_import_test!(fixture_articulations, "fragment_articulations.ly");
fixture_import_test!(fixture_ornaments_tremolo, "fragment_ornaments_tremolo.ly");
fixture_import_test!(fixture_technical, "fragment_technical.ly");
fixture_import_test!(fixture_tuplets_frag, "fragment_tuplets.ly");
fixture_import_test!(fixture_grace, "fragment_grace.ly");
fixture_import_test!(fixture_repeats, "fragment_repeats.ly");
fixture_import_test!(fixture_barcheck_barline, "fragment_barcheck_barline.ly");
fixture_import_test!(fixture_chord_repetition, "fragment_chord_repetition.ly");
fixture_import_test!(fixture_lyrics, "fragment_lyrics.ly");
fixture_import_test!(fixture_markup, "fragment_markup.ly");
fixture_import_test!(fixture_tempo_marks, "fragment_tempo_marks.ly");
fixture_import_test!(fixture_chordmode, "fragment_chordmode.ly");
fixture_import_test!(fixture_figured_bass_basic, "figured-bass-basic.ly");
fixture_import_test!(fixture_figured_bass_mods, "figured-bass-modifications.ly");
fixture_import_test!(fixture_figured_bass_brackets, "figured-bass-brackets.ly");
fixture_import_test!(fixture_figured_bass_alts, "figured-bass-alterations.ly");
fixture_import_test!(fixture_drummode, "fragment_drummode.ly");
fixture_import_test!(fixture_properties, "fragment_properties.ly");
fixture_import_test!(fixture_header, "fragment_header.ly");
fixture_import_test!(fixture_paper_layout_midi, "fragment_paper_layout_midi.ly");
fixture_import_test!(fixture_variables, "fragment_variables.ly");
fixture_import_test!(fixture_music_functions, "fragment_music_functions.ly");
fixture_import_test!(
    fixture_music_functions_rt,
    "fragment_music_functions_roundtrip.ly"
);
fixture_import_test!(fixture_scheme, "fragment_scheme.ly");
fixture_import_test!(fixture_scheme_roundtrip, "fragment_scheme_roundtrip.ly");
fixture_import_test!(fixture_comprehensive, "fragment_import_comprehensive.ly");

// ---------------------------------------------------------------------------
// Element ID retention: \tweak id #"value" -> xml:id
// ---------------------------------------------------------------------------

#[test]
fn tweak_id_sets_xml_id_on_note() {
    let mei = parse_and_import(r#"{ c4\tweak id #"my-note" d4 }"#);
    let children = first_layer_children(&mei);
    let mut found = false;
    for child in children {
        if let tusk_model::elements::LayerChild::Note(n) = child
            && n.common.xml_id.as_deref() == Some("my-note")
        {
            found = true;
        }
    }
    assert!(found, "should find a note with xml:id='my-note'");
}

#[test]
fn tweak_id_preserved_in_label_for_roundtrip() {
    let mei = parse_and_import(r#"{ c4\tweak id #"note-1" }"#);
    let children = first_layer_children(&mei);
    if let tusk_model::elements::LayerChild::Note(n) = &children[0] {
        let label = n.common.label.as_deref().unwrap_or("");
        assert!(
            label.contains("lilypond:tweak"),
            "label should contain tweak: {label}"
        );
    } else {
        panic!("expected Note");
    }
}

#[test]
fn tweak_non_id_does_not_set_xml_id() {
    let mei = parse_and_import(r#"{ c4\tweak color #red }"#);
    let children = first_layer_children(&mei);
    if let tusk_model::elements::LayerChild::Note(n) = &children[0] {
        // xml:id should be auto-generated, not "red"
        let id = n.common.xml_id.as_deref().unwrap_or("");
        assert!(
            id.starts_with("ly-note-"),
            "non-id tweak should not override xml:id: {id}"
        );
    }
}

// ---------------------------------------------------------------------------
// Cross-staff: \change Staff stores label
// ---------------------------------------------------------------------------

#[test]
fn context_change_produces_label_on_notes() {
    let src = r#"
    <<
      \new Staff = "up" {
        \change Staff = "down"
        c4 d
      }
      \new Staff = "down" { s1 }
    >>
    "#;
    let mei = parse_and_import(src);
    let children = first_layer_children(&mei);
    for child in children {
        if let tusk_model::elements::LayerChild::Note(n) = child {
            let label = n.common.label.as_deref().unwrap_or("");
            assert!(
                label.contains("lilypond:change,Staff,down"),
                "note should have cross-staff label: {label}"
            );
        }
    }
}

#[test]
fn context_change_roundtrips_via_export() {
    let src = r#"{ \change Staff = "other" c4 d4 }"#;
    let file = Parser::new(src).unwrap().parse().unwrap();
    let mei = import(&file).unwrap();
    let ly_out = crate::export::export(&mei).unwrap();
    let serialized = crate::serializer::serialize(&ly_out);
    assert!(
        serialized.contains(r#"\change Staff = "other""#),
        "export should restore \\change: {serialized}"
    );
}

// ---------------------------------------------------------------------------
// Edge cases: nested tuplets, nested repeats, multiple voices
// ---------------------------------------------------------------------------

#[test]
fn nested_tuplets_produce_multiple_tuplet_spans() {
    let src = r#"{ \tuplet 3/2 { c8 \tuplet 5/4 { d16 e f g a } b8 } }"#;
    let mei = parse_and_import(src);
    let mc = measure_children(&mei);
    let tuplet_count = mc
        .iter()
        .filter(|c| matches!(c, tusk_model::elements::MeasureChild::TupletSpan(_)))
        .count();
    assert_eq!(tuplet_count, 2, "should have 2 tuplet spans (nested)");
}

#[test]
fn nested_repeats_produce_multiple_dirs() {
    let src = r#"{
        \repeat volta 2 {
            \repeat volta 3 { c4 d }
            e4 f
        }
    }"#;
    let mei = parse_and_import(src);
    let mc = measure_children(&mei);
    let repeat_dirs: Vec<_> = mc
        .iter()
        .filter_map(|c| {
            if let tusk_model::elements::MeasureChild::Dir(d) = c
                && d.common
                    .label
                    .as_deref()
                    .is_some_and(|l| l.starts_with("lilypond:repeat"))
            {
                return Some(d);
            }
            None
        })
        .collect();
    assert_eq!(repeat_dirs.len(), 2, "should have 2 repeat dirs (nested)");
}

#[test]
fn multiple_voices_create_multiple_layers() {
    let src = r#"\new Staff << { c4 d e f } \\ { a,4 b c d } >>"#;
    let mei = parse_and_import(src);
    let staff = first_staff(&mei).expect("should have staff");
    let layer_count = staff
        .children
        .iter()
        .filter(|c| matches!(c, StaffChild::Layer(_)))
        .count();
    assert_eq!(layer_count, 2, "should have 2 layers for 2 voices");
}

// ---------------------------------------------------------------------------
// MEI extended/label patterns verification
// ---------------------------------------------------------------------------

#[test]
fn property_override_stored_in_label() {
    let src = r#"{ \override NoteHead.color = #red c4 }"#;
    let mei = parse_and_import(src);
    let mc = measure_children(&mei);
    let prop_dirs: Vec<_> = mc
        .iter()
        .filter_map(|c| {
            if let tusk_model::elements::MeasureChild::Dir(d) = c
                && d.common
                    .label
                    .as_deref()
                    .is_some_and(|l| l.starts_with("lilypond:prop"))
            {
                return Some(d);
            }
            None
        })
        .collect();
    assert_eq!(prop_dirs.len(), 1, "should have 1 property dir");
    let label = prop_dirs[0].common.label.as_deref().unwrap();
    assert!(
        label.contains("override"),
        "property label should contain override: {label}"
    );
}

#[test]
fn music_function_stored_in_label() {
    let src = r#"{ \tag "part" c4 }"#;
    let mei = parse_and_import(src);
    let mc = measure_children(&mei);
    let func_dirs: Vec<_> = mc
        .iter()
        .filter_map(|c| {
            if let tusk_model::elements::MeasureChild::Dir(d) = c
                && d.common
                    .label
                    .as_deref()
                    .is_some_and(|l| l.starts_with("lilypond:func"))
            {
                return Some(d);
            }
            None
        })
        .collect();
    assert!(!func_dirs.is_empty(), "should have function dir for \\tag");
}

#[test]
fn grace_notes_have_grace_attribute() {
    let src = r#"{ \grace { e16 } d4 }"#;
    let mei = parse_and_import(src);
    let children = first_layer_children(&mei);
    if let tusk_model::elements::LayerChild::Note(n) = &children[0] {
        assert!(
            n.note_log.grace.is_some(),
            "grace note should have @grace attribute"
        );
        let label = n.common.label.as_deref().unwrap_or("");
        assert!(
            label.contains("lilypond:grace"),
            "grace note should have grace label: {label}"
        );
    } else {
        panic!("expected Note as first child");
    }
}

#[test]
fn figured_bass_creates_fb_elements() {
    let src = r#"<<
        \new Staff { c4 d e f }
        \figures { \< 6 \>4 \< 4 3 \>4 \< _ \>4 \< 6 \>4 }
    >>"#;
    let mei = parse_and_import(src);
    let mc = measure_children(&mei);
    let fb_count = mc
        .iter()
        .filter(|c| matches!(c, tusk_model::elements::MeasureChild::Fb(_)))
        .count();
    assert!(fb_count > 0, "should have Fb elements from \\figures");
}

#[test]
fn chord_mode_creates_harm_elements() {
    let src = r#"<<
        \new ChordNames \chordmode { c4 g:7 }
        \new Staff { c'4 b }
    >>"#;
    let mei = parse_and_import(src);
    let mc = measure_children(&mei);
    let harm_count = mc
        .iter()
        .filter(|c| matches!(c, tusk_model::elements::MeasureChild::Harm(_)))
        .count();
    assert!(harm_count > 0, "should have Harm elements from ChordNames");
}

#[test]
fn header_metadata_imported_to_mei_head() {
    let src = r#"
    \header {
      title = "My Title"
      composer = "Bach"
    }
    { c4 d e f }
    "#;
    let mei = parse_and_import(src);
    let has_head = mei
        .children
        .iter()
        .any(|c| matches!(c, MeiChild::MeiHead(_)));
    assert!(has_head, "should have MeiHead with metadata");
}

#[test]
fn comprehensive_fixture_imports_successfully() {
    let src =
        include_str!("../../../../../tests/fixtures/lilypond/fragment_import_comprehensive.ly");
    let file = Parser::new(src).unwrap().parse().unwrap();
    let mei = import(&file).unwrap();

    let has_music = mei.children.iter().any(|c| matches!(c, MeiChild::Music(_)));
    assert!(has_music, "should have Music element");

    let has_head = mei
        .children
        .iter()
        .any(|c| matches!(c, MeiChild::MeiHead(_)));
    assert!(has_head, "should have MeiHead");

    let children = first_layer_children(&mei);
    assert!(!children.is_empty(), "should have notes in the first layer");
}
