//! Import tests for articulations, ornaments, tremolo, and technical notations.

use super::*;
use crate::parser::Parser;
use tusk_model::elements::{
    Fermata as MeiFermata, Mei, MeiChild, Mordent as MeiMordent, Ornam as MeiOrnam, ScoreChild,
    SectionChild, Trill as MeiTrill, Turn as MeiTurn,
};
use tusk_model::ExtensionStore;

/// Assert articulation info in the ExtensionStore for a given element ID.
fn assert_artic_ext(ext_store: &ExtensionStore, id: &str, kind: &str, value: &str, direction: Option<&str>) {
    let info = ext_store
        .articulation_info(id)
        .unwrap_or_else(|| panic!("expected articulation_info for id={id}"));
    let expected_kind = match kind {
        "Articulation" => tusk_model::ArticulationKind::Articulation,
        "Fingering" => tusk_model::ArticulationKind::Fingering,
        "StringNumber" => tusk_model::ArticulationKind::StringNumber,
        _ => panic!("unknown kind: {kind}"),
    };
    assert_eq!(info.kind, expected_kind, "kind mismatch for id={id}");
    assert_eq!(info.value, value, "value mismatch for id={id}");
    let expected_dir = direction.map(|d| match d {
        "Up" => tusk_model::DirectionExt::Up,
        "Down" => tusk_model::DirectionExt::Down,
        _ => panic!("unknown direction: {d}"),
    });
    assert_eq!(
        info.direction, expected_dir,
        "direction mismatch for id={id}"
    );
}

fn parse_and_import(src: &str) -> (Mei, ExtensionStore) {
    let file = Parser::new(src).unwrap().parse().unwrap();
    import(&file).unwrap()
}

/// Walk MEI to find layer children (first layer of first staff).
fn layer_children(mei: &Mei) -> &[LayerChild] {
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
                                            if let MeasureChild::Staff(staff) = mc2
                                                && let Some(
                                                    tusk_model::elements::StaffChild::Layer(layer),
                                                ) = staff.children.first()
                                            {
                                                return &layer.children;
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
    &[]
}

/// Walk MEI to find all Dir elements in the first measure.
fn measure_dirs(mei: &Mei) -> Vec<&tusk_model::elements::Dir> {
    let mut dirs = Vec::new();
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
                                            if let MeasureChild::Dir(d) = mc2 {
                                                dirs.push(d.as_ref());
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
    dirs
}

/// Collect all Trill control events from the first measure.
fn measure_trills(mei: &Mei) -> Vec<&MeiTrill> {
    let mut trills = Vec::new();
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
                                            if let MeasureChild::Trill(t) = mc2 {
                                                trills.push(t.as_ref());
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
    trills
}

/// Collect all Mordent control events from the first measure.
fn measure_mordents(mei: &Mei) -> Vec<&MeiMordent> {
    let mut mordents = Vec::new();
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
                                            if let MeasureChild::Mordent(m) = mc2 {
                                                mordents.push(m.as_ref());
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
    mordents
}

/// Collect all Turn control events from the first measure.
fn measure_turns(mei: &Mei) -> Vec<&MeiTurn> {
    let mut turns = Vec::new();
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
                                            if let MeasureChild::Turn(t) = mc2 {
                                                turns.push(t.as_ref());
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
    turns
}

/// Collect all Fermata control events from the first measure.
fn measure_fermatas(mei: &Mei) -> Vec<&MeiFermata> {
    let mut fermatas = Vec::new();
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
                                            if let MeasureChild::Fermata(f) = mc2 {
                                                fermatas.push(f.as_ref());
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
    fermatas
}

/// Collect all Ornam control events from the first measure.
fn measure_ornams(mei: &Mei) -> Vec<&MeiOrnam> {
    let mut ornams = Vec::new();
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
                                            if let MeasureChild::Ornam(o) = mc2 {
                                                ornams.push(o.as_ref());
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
    ornams
}

// ---------------------------------------------------------------------------
// Articulation, fingering, string number import tests (Phase 12.2)
// ---------------------------------------------------------------------------

#[test]
fn import_articulation_creates_dir() {
    let (mei, ext_store) = parse_and_import("{ c4-. d4-> }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 2, "expected 2 dir control events for artics");
    let dir0_id = dirs[0].common.xml_id.as_deref().unwrap();
    assert_artic_ext(&ext_store, dir0_id, "Articulation", "staccato", None);
    assert!(dirs[0].dir_log.startid.is_some());
    let dir1_id = dirs[1].common.xml_id.as_deref().unwrap();
    assert_artic_ext(&ext_store, dir1_id, "Articulation", "accent", None);
}

#[test]
fn import_articulation_with_direction() {
    let (mei, ext_store) = parse_and_import("{ c4^. d4_- }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 2);
    let dir0_id = dirs[0].common.xml_id.as_deref().unwrap();
    assert_artic_ext(&ext_store, dir0_id, "Articulation", "staccato", Some("Up"));
    let dir1_id = dirs[1].common.xml_id.as_deref().unwrap();
    assert_artic_ext(&ext_store, dir1_id, "Articulation", "tenuto", Some("Down"));
}

#[test]
fn import_fingering_creates_dir() {
    let (mei, ext_store) = parse_and_import("{ c4-1 d4^3 }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 2, "expected 2 fingering dir events");
    let dir0_id = dirs[0].common.xml_id.as_deref().unwrap();
    assert_artic_ext(&ext_store, dir0_id, "Fingering", "1", None);
    let dir1_id = dirs[1].common.xml_id.as_deref().unwrap();
    assert_artic_ext(&ext_store, dir1_id, "Fingering", "3", Some("Up"));
}

#[test]
fn import_named_articulation() {
    let (mei, ext_store) = parse_and_import("{ c4-\\staccato }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 1);
    let dir0_id = dirs[0].common.xml_id.as_deref().unwrap();
    assert_artic_ext(&ext_store, dir0_id, "Articulation", "staccato", None);
}

#[test]
fn import_multiple_artics_on_one_note() {
    let (mei, ext_store) = parse_and_import("{ c4-. -3 }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 2, "expected staccato + fingering");
    let dir0_id = dirs[0].common.xml_id.as_deref().unwrap();
    assert_artic_ext(&ext_store, dir0_id, "Articulation", "staccato", None);
    let dir1_id = dirs[1].common.xml_id.as_deref().unwrap();
    assert_artic_ext(&ext_store, dir1_id, "Fingering", "3", None);
}

// ---------------------------------------------------------------------------
// Ornament and tremolo import tests (Phase 13.2)
// ---------------------------------------------------------------------------

#[test]
fn import_trill_creates_trill_element() {
    let (mei, _ext_store) = parse_and_import("{ c4\\trill }");
    let trills = measure_trills(&mei);
    assert_eq!(trills.len(), 1, "expected 1 trill control event");
    assert!(trills[0].trill_log.startid.is_some());
    assert!(trills[0].trill_log.staff.is_some());
}

#[test]
fn import_mordent_creates_mordent_element() {
    let (mei, _ext_store) = parse_and_import("{ d4\\mordent }");
    let mordents = measure_mordents(&mei);
    assert_eq!(mordents.len(), 1, "expected 1 mordent control event");
    assert_eq!(mordents[0].mordent_log.form.as_deref(), Some("lower"));
    assert!(mordents[0].mordent_log.startid.is_some());
}

#[test]
fn import_prall_creates_upper_mordent() {
    let (mei, _ext_store) = parse_and_import("{ f4\\prall }");
    let mordents = measure_mordents(&mei);
    assert_eq!(
        mordents.len(),
        1,
        "expected 1 mordent (prall) control event"
    );
    assert_eq!(mordents[0].mordent_log.form.as_deref(), Some("upper"));
}

#[test]
fn import_turn_creates_turn_element() {
    let (mei, _ext_store) = parse_and_import("{ e4\\turn }");
    let turns = measure_turns(&mei);
    assert_eq!(turns.len(), 1, "expected 1 turn control event");
    assert_eq!(turns[0].turn_log.form.as_deref(), Some("upper"));
}

#[test]
fn import_reverseturn_creates_lower_turn() {
    let (mei, _ext_store) = parse_and_import("{ a4\\reverseturn }");
    let turns = measure_turns(&mei);
    assert_eq!(
        turns.len(),
        1,
        "expected 1 turn (reverseturn) control event"
    );
    assert_eq!(turns[0].turn_log.form.as_deref(), Some("lower"));
}

#[test]
fn import_fermata_creates_fermata_element() {
    let (mei, _ext_store) = parse_and_import("{ b4\\fermata }");
    let fermatas = measure_fermatas(&mei);
    assert_eq!(fermatas.len(), 1, "expected 1 fermata control event");
    assert!(fermatas[0].fermata_log.startid.is_some());
}

#[test]
fn import_prallprall_creates_ornam() {
    let (mei, ext_store) = parse_and_import("{ g4\\prallprall }");
    let ornams = measure_ornams(&mei);
    assert_eq!(ornams.len(), 1, "expected 1 ornam control event");
    let ornam_id = ornams[0].common.xml_id.as_deref().unwrap();
    let info = ext_store.ornament_info(ornam_id).expect("should have ornament info");
    assert!(
        info.name.contains("prallprall"),
        "ornament name should contain prallprall: {}",
        info.name
    );
}

#[test]
fn import_tremolo_wraps_note_in_btrem() {
    let (mei, ext_store) = parse_and_import("{ e4:32 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 1);
    if let LayerChild::BTrem(btrem) = &children[0] {
        assert_eq!(btrem.children.len(), 1);
        assert!(matches!(
            btrem.children[0],
            tusk_model::elements::BTremChild::Note(_)
        ));
        let btrem_id = btrem.common.xml_id.as_deref().unwrap();
        let info = ext_store.tremolo_info(btrem_id).expect("should have tremolo info");
        assert!(
            info.value == 32,
            "tremolo value should be 32: {}",
            info.value
        );
        assert_eq!(btrem.b_trem_log.num.as_deref(), Some("3")); // 32nds = 3 slashes
    } else {
        panic!("expected BTrem, got {:?}", children[0]);
    }
}

#[test]
fn import_tremolo_chord_wraps_in_btrem() {
    let (mei, _ext_store) = parse_and_import("{ <c e g>4:32 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 1);
    if let LayerChild::BTrem(btrem) = &children[0] {
        assert!(matches!(
            btrem.children[0],
            tusk_model::elements::BTremChild::Chord(_)
        ));
    } else {
        panic!("expected BTrem for chord tremolo");
    }
}

#[test]
fn import_combined_ornaments_on_one_note() {
    let (mei, _ext_store) = parse_and_import("{ c4\\trill\\fermata }");
    let trills = measure_trills(&mei);
    let fermatas = measure_fermatas(&mei);
    assert_eq!(trills.len(), 1, "expected 1 trill");
    assert_eq!(fermatas.len(), 1, "expected 1 fermata");
}

#[test]
fn import_upbow_stays_as_dir() {
    // upbow is not a native MEI ornament, should stay as <dir>
    let (mei, ext_store) = parse_and_import("{ c4\\upbow }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 1);
    let dir0_id = dirs[0].common.xml_id.as_deref().unwrap();
    assert_artic_ext(&ext_store, dir0_id, "Articulation", "upbow", None);
    // Should NOT be a trill/mordent/turn/fermata/ornam
    assert_eq!(measure_trills(&mei).len(), 0);
    assert_eq!(measure_mordents(&mei).len(), 0);
}

// ---------------------------------------------------------------------------
// Technical notation import tests (Phase 14.2)
// ---------------------------------------------------------------------------

#[test]
fn import_string_number_creates_dir() {
    let (mei, ext_store) = parse_and_import("{ c4-\\1 d4^\\2 }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 2, "expected 2 string number dir events");
    let dir0_id = dirs[0].common.xml_id.as_deref().unwrap();
    assert_artic_ext(&ext_store, dir0_id, "StringNumber", "1", None);
    assert!(dirs[0].dir_log.startid.is_some());
    let dir1_id = dirs[1].common.xml_id.as_deref().unwrap();
    assert_artic_ext(&ext_store, dir1_id, "StringNumber", "2", Some("Up"));
}

#[test]
fn import_open_creates_dir() {
    let (mei, ext_store) = parse_and_import("{ c4\\open }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 1, "expected 1 dir for \\open");
    let dir0_id = dirs[0].common.xml_id.as_deref().unwrap();
    assert_artic_ext(&ext_store, dir0_id, "Articulation", "open", None);
    assert!(dirs[0].dir_log.startid.is_some());
}

#[test]
fn import_harmonic_creates_dir() {
    let (mei, ext_store) = parse_and_import("{ c4\\harmonic }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 1, "expected 1 dir for \\harmonic");
    let dir0_id = dirs[0].common.xml_id.as_deref().unwrap();
    assert_artic_ext(&ext_store, dir0_id, "Articulation", "harmonic", None);
}

#[test]
fn import_flageolet_creates_dir() {
    let (mei, ext_store) = parse_and_import("{ c4\\flageolet }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 1, "expected 1 dir for \\flageolet");
    let dir0_id = dirs[0].common.xml_id.as_deref().unwrap();
    assert_artic_ext(&ext_store, dir0_id, "Articulation", "flageolet", None);
}

#[test]
fn import_combined_string_and_open() {
    let (mei, ext_store) = parse_and_import("{ c4-\\1 -\\open }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 2, "expected string number + open");
    let dir0_id = dirs[0].common.xml_id.as_deref().unwrap();
    assert_artic_ext(&ext_store, dir0_id, "StringNumber", "1", None);
    let dir1_id = dirs[1].common.xml_id.as_deref().unwrap();
    assert_artic_ext(&ext_store, dir1_id, "Articulation", "open", None);
}

// ---------------------------------------------------------------------------
// Tuplet import tests
// ---------------------------------------------------------------------------

use tusk_model::elements::TupletSpan;

/// Walk MEI to find all TupletSpan elements in the first measure.
fn measure_tuplet_spans(mei: &Mei) -> Vec<&TupletSpan> {
    let mut spans = Vec::new();
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
                                            if let MeasureChild::TupletSpan(ts) = mc2 {
                                                spans.push(ts.as_ref());
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
    spans
}

#[test]
fn import_tuplet_creates_tuplet_span() {
    let (mei, _ext_store) = parse_and_import("{ \\tuplet 3/2 { c8 d e } }");
    let spans = measure_tuplet_spans(&mei);
    assert_eq!(spans.len(), 1, "expected 1 tupletSpan");
    let ts = &spans[0];
    assert_eq!(ts.tuplet_span_log.num.as_deref(), Some("3"));
    assert_eq!(ts.tuplet_span_log.numbase.as_deref(), Some("2"));
    assert!(ts.tuplet_span_log.startid.is_some());
    assert!(ts.tuplet_span_log.endid.is_some());
    assert!(ts.tuplet_span_log.staff.is_some());
    // startid and endid should reference different notes
    assert_ne!(
        ts.tuplet_span_log.startid.as_ref().unwrap().0,
        ts.tuplet_span_log.endid.as_ref().unwrap().0
    );
}

#[test]
fn import_tuplet_with_span_duration() {
    let (mei, ext_store) = parse_and_import("{ \\tuplet 3/2 4 { c8 d e f g a } }");
    let spans = measure_tuplet_spans(&mei);
    assert_eq!(spans.len(), 1);
    let ts = &spans[0];
    assert_eq!(ts.tuplet_span_log.num.as_deref(), Some("3"));
    assert_eq!(ts.tuplet_span_log.numbase.as_deref(), Some("2"));
    // ExtensionStore should contain span duration info
    let ts_id = ts.common.xml_id.as_deref().unwrap();
    let info = ext_store.tuplet_info(ts_id).expect("should have tuplet info");
    assert_eq!(
        info.span_duration.as_ref().map(|d| d.base),
        Some(4),
        "span duration base should be 4"
    );
}

#[test]
fn import_tuplet_5_4() {
    let (mei, _ext_store) = parse_and_import("{ \\tuplet 5/4 { c16 d e f g } }");
    let spans = measure_tuplet_spans(&mei);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].tuplet_span_log.num.as_deref(), Some("5"));
    assert_eq!(spans[0].tuplet_span_log.numbase.as_deref(), Some("4"));
    // 5 notes in the layer
    let lc = layer_children(&mei);
    assert_eq!(lc.len(), 5, "expected 5 notes in layer");
}

#[test]
fn import_nested_tuplets() {
    let (mei, _ext_store) = parse_and_import("{ \\tuplet 3/2 { \\tuplet 3/2 { c32 d e } f16 g } }");
    let spans = measure_tuplet_spans(&mei);
    assert_eq!(spans.len(), 2, "expected 2 tupletSpans (inner + outer)");
    // Both should be 3/2
    for ts in &spans {
        assert_eq!(ts.tuplet_span_log.num.as_deref(), Some("3"));
        assert_eq!(ts.tuplet_span_log.numbase.as_deref(), Some("2"));
    }
}

#[test]
fn import_tuplet_label_format() {
    let (mei, ext_store) = parse_and_import("{ \\tuplet 3/2 { c8 d e } }");
    let spans = measure_tuplet_spans(&mei);
    let ts_id = spans[0].common.xml_id.as_deref().unwrap();
    let info = ext_store.tuplet_info(ts_id).expect("should have tuplet info");
    assert_eq!(info.num, 3);
    assert_eq!(info.denom, 2);
}

// ---------------------------------------------------------------------------
// Grace note import tests
// ---------------------------------------------------------------------------

#[test]
fn import_grace_sets_grace_attr() {
    let (mei, ext_store) = parse_and_import("{ \\grace c16 d4 }");
    let lc = layer_children(&mei);
    assert_eq!(lc.len(), 2, "expected 2 layer children: {lc:?}");
    // First note should be grace
    if let LayerChild::Note(note) = &lc[0] {
        assert!(
            note.note_log.grace.is_some(),
            "grace note should have @grace"
        );
        assert_eq!(
            note.note_log.grace,
            Some(tusk_model::generated::data::DataGrace::Unacc)
        );
        let note_id = note.common.xml_id.as_deref().unwrap();
        let info = ext_store.grace_info(note_id).expect("should have grace info");
        assert!(matches!(info, tusk_model::GraceInfo::Grace));
    } else {
        panic!("expected Note, got {:?}", lc[0]);
    }
    // Second note should NOT be grace
    if let LayerChild::Note(note) = &lc[1] {
        assert!(
            note.note_log.grace.is_none(),
            "main note should not have @grace"
        );
    }
}

#[test]
fn import_acciaccatura_sets_unacc() {
    let (mei, ext_store) = parse_and_import("{ \\acciaccatura d8 c4 }");
    let lc = layer_children(&mei);
    if let LayerChild::Note(note) = &lc[0] {
        assert_eq!(
            note.note_log.grace,
            Some(tusk_model::generated::data::DataGrace::Unacc)
        );
        let note_id = note.common.xml_id.as_deref().unwrap();
        let info = ext_store.grace_info(note_id).expect("should have grace info");
        assert!(
            matches!(info, tusk_model::GraceInfo::Acciaccatura),
            "expected Acciaccatura, got {info:?}"
        );
    } else {
        panic!("expected Note");
    }
}

#[test]
fn import_appoggiatura_sets_acc() {
    let (mei, ext_store) = parse_and_import("{ \\appoggiatura d8 c2 }");
    let lc = layer_children(&mei);
    if let LayerChild::Note(note) = &lc[0] {
        assert_eq!(
            note.note_log.grace,
            Some(tusk_model::generated::data::DataGrace::Acc)
        );
        let note_id = note.common.xml_id.as_deref().unwrap();
        let info = ext_store.grace_info(note_id).expect("should have grace info");
        assert!(
            matches!(info, tusk_model::GraceInfo::Appoggiatura),
            "expected Appoggiatura, got {info:?}"
        );
    } else {
        panic!("expected Note");
    }
}

#[test]
fn import_grace_multiple_notes() {
    let (mei, _ext_store) = parse_and_import("{ \\grace { c16 d16 } e4 }");
    let lc = layer_children(&mei);
    assert_eq!(lc.len(), 3, "expected 3 layer children: {lc:?}");
    // First two notes should be grace
    for (i, child) in lc.iter().enumerate().take(2) {
        if let LayerChild::Note(note) = child {
            assert!(
                note.note_log.grace.is_some(),
                "note {i} should be grace: {:?}",
                note.note_log.grace
            );
        }
    }
    // Third note should NOT be grace
    if let LayerChild::Note(note) = &lc[2] {
        assert!(
            note.note_log.grace.is_none(),
            "main note should not be grace"
        );
    }
}

#[test]
fn import_after_grace_main_not_grace() {
    let (mei, ext_store) = parse_and_import("{ \\afterGrace c2 { d16 e16 } }");
    let lc = layer_children(&mei);
    assert_eq!(lc.len(), 3, "expected 3 layer children: {lc:?}");
    // First note (main) should NOT be grace
    if let LayerChild::Note(note) = &lc[0] {
        assert!(
            note.note_log.grace.is_none(),
            "main note should not be grace"
        );
    }
    // Grace notes should have @grace and AfterGrace info
    for (i, child) in lc.iter().enumerate().take(3).skip(1) {
        if let LayerChild::Note(note) = child {
            assert!(
                note.note_log.grace.is_some(),
                "after-grace note {i} should have @grace"
            );
            let note_id = note.common.xml_id.as_deref().unwrap();
            let info = ext_store.grace_info(note_id).expect("should have grace info");
            assert!(
                matches!(info, tusk_model::GraceInfo::AfterGrace { .. }),
                "expected AfterGrace, got {info:?}"
            );
        }
    }
}

#[test]
fn import_after_grace_with_fraction() {
    let (mei, ext_store) = parse_and_import("{ \\afterGrace 3/4 c2 { d16 } }");
    let lc = layer_children(&mei);
    // Grace note should have fraction in ext_store
    if let LayerChild::Note(note) = &lc[1] {
        let note_id = note.common.xml_id.as_deref().unwrap();
        let info = ext_store.grace_info(note_id).expect("should have grace info");
        if let tusk_model::GraceInfo::AfterGrace { fraction } = info {
            assert_eq!(
                *fraction,
                Some((3, 4)),
                "fraction should be (3,4)"
            );
        } else {
            panic!("expected AfterGrace, got {info:?}");
        }
    }
}

#[test]
fn import_grace_chord() {
    let (mei, ext_store) = parse_and_import("{ \\grace <c e>16 d4 }");
    let lc = layer_children(&mei);
    assert_eq!(lc.len(), 2);
    if let LayerChild::Chord(chord) = &lc[0] {
        assert!(
            chord.chord_log.grace.is_some(),
            "grace chord should have @grace"
        );
        let chord_id = chord.common.xml_id.as_deref().unwrap();
        let info = ext_store.grace_info(chord_id).expect("should have grace info");
        assert!(matches!(info, tusk_model::GraceInfo::Grace));
    } else {
        panic!("expected Chord, got {:?}", lc[0]);
    }
}

// ---------------------------------------------------------------------------
// Repeat import tests
// ---------------------------------------------------------------------------

#[test]
fn import_repeat_volta_creates_dir() {
    let (mei, ext_store) = parse_and_import("{ \\repeat volta 2 { c4 d e f } }");
    let dirs = measure_dirs(&mei);
    let repeat_dirs: Vec<_> = dirs
        .iter()
        .filter(|d| {
            let id = d.common.xml_id.as_deref().unwrap_or("");
            ext_store.repeat_info(id).is_some()
        })
        .collect();
    assert_eq!(repeat_dirs.len(), 1, "should create one repeat Dir");
    let dir_id = repeat_dirs[0].common.xml_id.as_deref().unwrap();
    let info = ext_store.repeat_info(dir_id).unwrap();
    assert_eq!(info.repeat_type, tusk_model::RepeatTypeExt::Volta);
    assert_eq!(info.count, 2);
    assert!(
        repeat_dirs[0].dir_log.startid.is_some(),
        "should have startid"
    );
    assert!(repeat_dirs[0].dir_log.endid.is_some(), "should have endid");
}

#[test]
fn import_repeat_with_alternatives_creates_ending_dirs() {
    let (mei, ext_store) = parse_and_import("{ \\repeat volta 2 { c4 d } \\alternative { { e4 } { f4 } } }");
    let dirs = measure_dirs(&mei);
    let repeat_dirs: Vec<_> = dirs
        .iter()
        .filter(|d| {
            let id = d.common.xml_id.as_deref().unwrap_or("");
            ext_store.repeat_info(id).is_some()
        })
        .collect();
    let ending_dirs: Vec<_> = dirs
        .iter()
        .filter(|d| {
            let id = d.common.xml_id.as_deref().unwrap_or("");
            ext_store.ending_info(id).is_some()
        })
        .collect();
    assert_eq!(repeat_dirs.len(), 1, "one repeat Dir");
    assert_eq!(ending_dirs.len(), 2, "two ending Dirs");
    let repeat_id = repeat_dirs[0].common.xml_id.as_deref().unwrap();
    let rinfo = ext_store.repeat_info(repeat_id).unwrap();
    assert_eq!(rinfo.alternative_count, Some(2));
    let end0_id = ending_dirs[0].common.xml_id.as_deref().unwrap();
    let end0 = ext_store.ending_info(end0_id).unwrap();
    assert_eq!(end0.index, 0);
    let end1_id = ending_dirs[1].common.xml_id.as_deref().unwrap();
    let end1 = ext_store.ending_info(end1_id).unwrap();
    assert_eq!(end1.index, 1);
}

#[test]
fn import_repeat_unfold_creates_dir() {
    let (mei, ext_store) = parse_and_import("{ \\repeat unfold 4 { c8 d } }");
    let dirs = measure_dirs(&mei);
    let repeat_dirs: Vec<_> = dirs
        .iter()
        .filter(|d| {
            let id = d.common.xml_id.as_deref().unwrap_or("");
            ext_store.repeat_info(id).is_some()
        })
        .collect();
    assert_eq!(repeat_dirs.len(), 1);
    let dir_id = repeat_dirs[0].common.xml_id.as_deref().unwrap();
    let info = ext_store.repeat_info(dir_id).unwrap();
    assert_eq!(info.repeat_type, tusk_model::RepeatTypeExt::Unfold);
    assert_eq!(info.count, 4);
}

#[test]
fn import_repeat_percent_creates_dir() {
    let (mei, ext_store) = parse_and_import("{ \\repeat percent 4 { c4 d e f } }");
    let dirs = measure_dirs(&mei);
    let repeat_dirs: Vec<_> = dirs
        .iter()
        .filter(|d| {
            let id = d.common.xml_id.as_deref().unwrap_or("");
            ext_store.repeat_info(id).is_some()
        })
        .collect();
    assert_eq!(repeat_dirs.len(), 1);
    let dir_id = repeat_dirs[0].common.xml_id.as_deref().unwrap();
    let info = ext_store.repeat_info(dir_id).unwrap();
    assert_eq!(info.repeat_type, tusk_model::RepeatTypeExt::Percent);
}

#[test]
fn import_nested_repeat_creates_multiple_dirs() {
    let (mei, ext_store) = parse_and_import("{ \\repeat volta 2 { \\repeat unfold 3 { c8 d } e4 } }");
    let dirs = measure_dirs(&mei);
    let repeat_dirs: Vec<_> = dirs
        .iter()
        .filter(|d| {
            let id = d.common.xml_id.as_deref().unwrap_or("");
            ext_store.repeat_info(id).is_some()
        })
        .collect();
    assert_eq!(
        repeat_dirs.len(),
        2,
        "should create two repeat Dirs (nested)"
    );
}

// ---------------------------------------------------------------------------
// Bar check and bar line import tests (Phase 18.2)
// ---------------------------------------------------------------------------

/// Walk MEI to find the first staffDef.
fn first_staff_def(mei: &Mei) -> Option<&tusk_model::elements::StaffDef> {
    for child in &mei.children {
        if let MeiChild::Music(music) = child {
            for mc in &music.children {
                let tusk_model::elements::MusicChild::Body(body) = mc;
                for bc in &body.children {
                    let tusk_model::elements::BodyChild::Mdiv(mdiv) = bc;
                    for dc in &mdiv.children {
                        let tusk_model::elements::MdivChild::Score(score) = dc;
                        for sc in &score.children {
                            if let ScoreChild::ScoreDef(sd) = sc {
                                for sd_c in &sd.children {
                                    if let tusk_model::elements::ScoreDefChild::StaffGrp(grp) = sd_c
                                    {
                                        for grp_c in &grp.children {
                                            if let tusk_model::elements::StaffGrpChild::StaffDef(
                                                sdef,
                                            ) = grp_c
                                            {
                                                return Some(sdef);
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

#[test]
fn import_bar_check_encoded_in_label() {
    let (mei, ext_store) = parse_and_import("{ c4 d e f | g4 a b c }");
    let sdef = first_staff_def(&mei).unwrap();
    let sdef_id = sdef.basic.xml_id.as_deref().unwrap();
    let seq = ext_store.event_sequence(sdef_id).expect("should have event_sequence");
    let has_bar_check = seq.events.iter().any(|e| matches!(e.event, tusk_model::ControlEvent::BarCheck));
    assert!(has_bar_check, "event sequence should contain BarCheck");
}

#[test]
fn import_bar_line_encoded_in_label() {
    let (mei, ext_store) = parse_and_import("{ c4 d e f \\bar \"|.\" }");
    let sdef = first_staff_def(&mei).unwrap();
    let sdef_id = sdef.basic.xml_id.as_deref().unwrap();
    let seq = ext_store.event_sequence(sdef_id).expect("should have event_sequence");
    let has_bar_line = seq.events.iter().any(|e| matches!(e.event, tusk_model::ControlEvent::BarLine { .. }));
    assert!(has_bar_line, "event sequence should contain BarLine");
}

#[test]
fn import_bar_check_does_not_create_layer_children() {
    let (mei, _ext_store) = parse_and_import("{ c4 | d4 }");
    let lc = layer_children(&mei);
    // Bar check should not create any layer children -- only notes
    assert_eq!(
        lc.len(),
        2,
        "expected 2 layer children (notes only): {lc:?}"
    );
}

#[test]
fn import_bar_line_does_not_create_layer_children() {
    let (mei, _ext_store) = parse_and_import("{ c4 d4 \\bar \"|.\" }");
    let lc = layer_children(&mei);
    assert_eq!(
        lc.len(),
        2,
        "expected 2 layer children (notes only): {lc:?}"
    );
}

#[test]
fn import_multiple_bar_checks_encoded() {
    let (mei, ext_store) = parse_and_import("{ c4 | d4 | e4 }");
    let sdef = first_staff_def(&mei).unwrap();
    let sdef_id = sdef.basic.xml_id.as_deref().unwrap();
    let seq = ext_store.event_sequence(sdef_id).expect("should have event_sequence");
    let has_bar_check = seq.events.iter().any(|e| matches!(e.event, tusk_model::ControlEvent::BarCheck));
    assert!(has_bar_check, "event sequence should contain BarCheck events");
}

// ---------------------------------------------------------------------------
// Chord repetition import tests (Phase 19.2)
// ---------------------------------------------------------------------------

#[test]
fn import_chord_repetition_expands_to_chord() {
    let (mei, _ext_store) = parse_and_import("{ <c e g>4 q q }");
    let lc = layer_children(&mei);
    // q expands to full chord -- all 3 should be Chord elements
    assert_eq!(lc.len(), 3, "expected 3 chords: {lc:?}");
    for (i, child) in lc.iter().enumerate() {
        assert!(
            matches!(child, LayerChild::Chord(_)),
            "child {i} should be Chord: {child:?}"
        );
    }
}

#[test]
fn import_chord_repetition_has_label() {
    let (mei, ext_store) = parse_and_import("{ <c e g>4 q }");
    let lc = layer_children(&mei);
    assert_eq!(lc.len(), 2);
    // First chord (original) should NOT have chord-rep in ext_store
    if let LayerChild::Chord(c) = &lc[0] {
        let id = c.common.xml_id.as_deref().unwrap_or("");
        assert!(
            ext_store.chord_repetition(id).is_none(),
            "original chord should not have chord_repetition"
        );
    }
    // Second chord (from q) should have chord-rep in ext_store
    if let LayerChild::Chord(c) = &lc[1] {
        let id = c.common.xml_id.as_deref().unwrap();
        assert!(
            ext_store.chord_repetition(id).is_some(),
            "chord from q should have chord_repetition: id={id}"
        );
    } else {
        panic!("expected Chord");
    }
}

#[test]
fn import_chord_repetition_preserves_duration() {
    let (mei, _ext_store) = parse_and_import("{ <c e g>4 q2. }");
    let lc = layer_children(&mei);
    assert_eq!(lc.len(), 2);
    if let LayerChild::Chord(c) = &lc[1] {
        // q2. should have dotted half note duration
        let dur = c.chord_log.dur.as_ref().unwrap();
        assert!(matches!(
            dur,
            tusk_model::generated::data::DataDuration::MeiDataDurationCmn(
                tusk_model::generated::data::DataDurationCmn::N2
            )
        ));
        assert_eq!(c.chord_log.dots.as_ref().map(|d| d.0), Some(1));
    } else {
        panic!("expected Chord");
    }
}

#[test]
fn import_chord_repetition_same_pitches() {
    let (mei, _ext_store) = parse_and_import("{ <c e g>4 q }");
    let lc = layer_children(&mei);
    // Both chords should have same number of child notes (3)
    if let (LayerChild::Chord(c1), LayerChild::Chord(c2)) = (&lc[0], &lc[1]) {
        assert_eq!(c1.children.len(), c2.children.len(), "same pitch count");
    } else {
        panic!("expected two Chords");
    }
}

#[test]
fn import_chord_repetition_with_dynamics() {
    let (mei, _ext_store) = parse_and_import("{ <c e g>4\\f q\\p }");
    let lc = layer_children(&mei);
    assert_eq!(lc.len(), 2);
    // Both notes should have dynamics attached via control events
    let mut dynam_count = 0;
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
                                            if let MeasureChild::Dynam(_) = mc2 {
                                                dynam_count += 1;
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
    assert_eq!(dynam_count, 2, "expected 2 dynam control events");
}

// ---------------------------------------------------------------------------
// Markup import tests
// ---------------------------------------------------------------------------

/// Helper: find the first staffDef from MEI.
fn first_staff_def_ctrl(mei: &Mei) -> Option<&tusk_model::elements::StaffDef> {
    for child in &mei.children {
        if let MeiChild::Music(music) = child {
            for mc in &music.children {
                let tusk_model::elements::MusicChild::Body(body) = mc;
                for bc in &body.children {
                    let tusk_model::elements::BodyChild::Mdiv(mdiv) = bc;
                    for dc in &mdiv.children {
                        let tusk_model::elements::MdivChild::Score(score) = dc;
                        for sc in &score.children {
                            if let ScoreChild::ScoreDef(score_def) = sc {
                                for sd_child in &score_def.children {
                                    if let tusk_model::elements::ScoreDefChild::StaffGrp(grp) =
                                        sd_child
                                    {
                                        for gc in &grp.children {
                                            if let tusk_model::elements::StaffGrpChild::StaffDef(
                                                sdef,
                                            ) = gc
                                            {
                                                return Some(sdef);
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

#[test]
fn import_markup_encoded_in_label() {
    let (mei, ext_store) = parse_and_import(r#"{ c'4 \markup { Hello } d'4 }"#);
    let sdef = first_staff_def_ctrl(&mei).unwrap();
    let sdef_id = sdef.basic.xml_id.as_deref().unwrap();
    let seq = ext_store.event_sequence(sdef_id).expect("should have event_sequence");
    let has_markup = seq.events.iter().any(|e| matches!(e.event, tusk_model::ControlEvent::Markup { .. }));
    assert!(has_markup, "event sequence should contain Markup entry");
}

#[test]
fn import_markup_does_not_create_layer_children() {
    let (mei, _ext_store) = parse_and_import(r#"{ c'4 \markup \bold "text" d'4 }"#);
    let children = layer_children(&mei);
    // Only 2 notes, no markup child
    assert_eq!(
        children.len(),
        2,
        "should have 2 layer children, got {}",
        children.len()
    );
}

#[test]
fn import_markuplist_encoded_in_label() {
    let (mei, ext_store) = parse_and_import(r#"{ c'4 \markuplist { "one" "two" } d'4 }"#);
    let sdef = first_staff_def_ctrl(&mei).unwrap();
    let sdef_id = sdef.basic.xml_id.as_deref().unwrap();
    let seq = ext_store.event_sequence(sdef_id).expect("should have event_sequence");
    let has_markuplist = seq.events.iter().any(|e| matches!(e.event, tusk_model::ControlEvent::MarkupList { .. }));
    assert!(has_markuplist, "event sequence should contain MarkupList entry");
}

#[test]
fn import_markup_position_correct() {
    // Markup between notes at position 1 (after first note)
    let (mei, ext_store) = parse_and_import(r#"{ c'4 \markup "between" d'4 }"#);
    let sdef = first_staff_def_ctrl(&mei).unwrap();
    let sdef_id = sdef.basic.xml_id.as_deref().unwrap();
    let seq = ext_store.event_sequence(sdef_id).expect("should have event_sequence");
    // After first note (index 1), markup should be at position 1
    let markup_event = seq.events.iter().find(|e| matches!(e.event, tusk_model::ControlEvent::Markup { .. }));
    assert!(markup_event.is_some(), "should have Markup event");
    assert_eq!(markup_event.unwrap().position, 1, "markup at position 1");
}

#[test]
fn import_markup_with_command() {
    let (mei, ext_store) = parse_and_import(r#"{ c'4 \markup \bold \italic "styled" d'4 }"#);
    let sdef = first_staff_def_ctrl(&mei).unwrap();
    let sdef_id = sdef.basic.xml_id.as_deref().unwrap();
    let seq = ext_store.event_sequence(sdef_id).expect("should have event_sequence");
    let markup_event = seq.events.iter().find(|e| matches!(e.event, tusk_model::ControlEvent::Markup { .. }));
    assert!(markup_event.is_some(), "should contain Markup entry");
    // Serialized form should contain \bold
    if let tusk_model::ControlEvent::Markup { serialized } = &markup_event.unwrap().event {
        assert!(serialized.contains("\\bold"), "should contain bold: {serialized}");
    }
}
