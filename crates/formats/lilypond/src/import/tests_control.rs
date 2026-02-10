//! Import tests for articulations, ornaments, tremolo, and technical notations.

use super::*;
use crate::parser::Parser;
use tusk_model::elements::{
    Fermata as MeiFermata, Mei, MeiChild, Mordent as MeiMordent, Ornam as MeiOrnam, ScoreChild,
    SectionChild, Trill as MeiTrill, Turn as MeiTurn,
};

fn parse_and_import(src: &str) -> Mei {
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
    let mei = parse_and_import("{ c4-. d4-> }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 2, "expected 2 dir control events for artics");
    assert_eq!(
        dirs[0].common.label.as_deref(),
        Some("lilypond:artic,staccato")
    );
    assert!(dirs[0].dir_log.startid.is_some());
    assert_eq!(
        dirs[1].common.label.as_deref(),
        Some("lilypond:artic,accent")
    );
}

#[test]
fn import_articulation_with_direction() {
    let mei = parse_and_import("{ c4^. d4_- }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 2);
    assert_eq!(
        dirs[0].common.label.as_deref(),
        Some("lilypond:artic,staccato,dir=up")
    );
    assert_eq!(
        dirs[1].common.label.as_deref(),
        Some("lilypond:artic,tenuto,dir=down")
    );
}

#[test]
fn import_fingering_creates_dir() {
    let mei = parse_and_import("{ c4-1 d4^3 }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 2, "expected 2 fingering dir events");
    assert_eq!(dirs[0].common.label.as_deref(), Some("lilypond:fing,1"));
    assert_eq!(
        dirs[1].common.label.as_deref(),
        Some("lilypond:fing,3,dir=up")
    );
}

#[test]
fn import_named_articulation() {
    let mei = parse_and_import("{ c4-\\staccato }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 1);
    assert_eq!(
        dirs[0].common.label.as_deref(),
        Some("lilypond:artic,staccato")
    );
}

#[test]
fn import_multiple_artics_on_one_note() {
    let mei = parse_and_import("{ c4-. -3 }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 2, "expected staccato + fingering");
    assert_eq!(
        dirs[0].common.label.as_deref(),
        Some("lilypond:artic,staccato")
    );
    assert_eq!(dirs[1].common.label.as_deref(), Some("lilypond:fing,3"));
}

// ---------------------------------------------------------------------------
// Ornament and tremolo import tests (Phase 13.2)
// ---------------------------------------------------------------------------

#[test]
fn import_trill_creates_trill_element() {
    let mei = parse_and_import("{ c4\\trill }");
    let trills = measure_trills(&mei);
    assert_eq!(trills.len(), 1, "expected 1 trill control event");
    assert!(trills[0].trill_log.startid.is_some());
    assert!(trills[0].trill_log.staff.is_some());
}

#[test]
fn import_mordent_creates_mordent_element() {
    let mei = parse_and_import("{ d4\\mordent }");
    let mordents = measure_mordents(&mei);
    assert_eq!(mordents.len(), 1, "expected 1 mordent control event");
    assert_eq!(mordents[0].mordent_log.form.as_deref(), Some("lower"));
    assert!(mordents[0].mordent_log.startid.is_some());
}

#[test]
fn import_prall_creates_upper_mordent() {
    let mei = parse_and_import("{ f4\\prall }");
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
    let mei = parse_and_import("{ e4\\turn }");
    let turns = measure_turns(&mei);
    assert_eq!(turns.len(), 1, "expected 1 turn control event");
    assert_eq!(turns[0].turn_log.form.as_deref(), Some("upper"));
}

#[test]
fn import_reverseturn_creates_lower_turn() {
    let mei = parse_and_import("{ a4\\reverseturn }");
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
    let mei = parse_and_import("{ b4\\fermata }");
    let fermatas = measure_fermatas(&mei);
    assert_eq!(fermatas.len(), 1, "expected 1 fermata control event");
    assert!(fermatas[0].fermata_log.startid.is_some());
}

#[test]
fn import_prallprall_creates_ornam() {
    let mei = parse_and_import("{ g4\\prallprall }");
    let ornams = measure_ornams(&mei);
    assert_eq!(ornams.len(), 1, "expected 1 ornam control event");
    let label = ornams[0].common.label.as_deref().unwrap();
    assert!(
        label.contains("prallprall"),
        "label should contain prallprall: {label}"
    );
}

#[test]
fn import_tremolo_wraps_note_in_btrem() {
    let mei = parse_and_import("{ e4:32 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 1);
    if let LayerChild::BTrem(btrem) = &children[0] {
        assert_eq!(btrem.children.len(), 1);
        assert!(matches!(
            btrem.children[0],
            tusk_model::elements::BTremChild::Note(_)
        ));
        let label = btrem.common.label.as_deref().unwrap();
        assert!(
            label.contains("32"),
            "label should contain tremolo value: {label}"
        );
        assert_eq!(btrem.b_trem_log.num.as_deref(), Some("3")); // 32nds = 3 slashes
    } else {
        panic!("expected BTrem, got {:?}", children[0]);
    }
}

#[test]
fn import_tremolo_chord_wraps_in_btrem() {
    let mei = parse_and_import("{ <c e g>4:32 }");
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
    let mei = parse_and_import("{ c4\\trill\\fermata }");
    let trills = measure_trills(&mei);
    let fermatas = measure_fermatas(&mei);
    assert_eq!(trills.len(), 1, "expected 1 trill");
    assert_eq!(fermatas.len(), 1, "expected 1 fermata");
}

#[test]
fn import_upbow_stays_as_dir() {
    // upbow is not a native MEI ornament, should stay as <dir>
    let mei = parse_and_import("{ c4\\upbow }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 1);
    assert_eq!(
        dirs[0].common.label.as_deref(),
        Some("lilypond:artic,upbow")
    );
    // Should NOT be a trill/mordent/turn/fermata/ornam
    assert_eq!(measure_trills(&mei).len(), 0);
    assert_eq!(measure_mordents(&mei).len(), 0);
}

// ---------------------------------------------------------------------------
// Technical notation import tests (Phase 14.2)
// ---------------------------------------------------------------------------

#[test]
fn import_string_number_creates_dir() {
    let mei = parse_and_import("{ c4-\\1 d4^\\2 }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 2, "expected 2 string number dir events");
    assert_eq!(dirs[0].common.label.as_deref(), Some("lilypond:string,1"));
    assert!(dirs[0].dir_log.startid.is_some());
    assert_eq!(
        dirs[1].common.label.as_deref(),
        Some("lilypond:string,2,dir=up")
    );
}

#[test]
fn import_open_creates_dir() {
    let mei = parse_and_import("{ c4\\open }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 1, "expected 1 dir for \\open");
    assert_eq!(dirs[0].common.label.as_deref(), Some("lilypond:artic,open"));
    assert!(dirs[0].dir_log.startid.is_some());
}

#[test]
fn import_harmonic_creates_dir() {
    let mei = parse_and_import("{ c4\\harmonic }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 1, "expected 1 dir for \\harmonic");
    assert_eq!(
        dirs[0].common.label.as_deref(),
        Some("lilypond:artic,harmonic")
    );
}

#[test]
fn import_flageolet_creates_dir() {
    let mei = parse_and_import("{ c4\\flageolet }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 1, "expected 1 dir for \\flageolet");
    assert_eq!(
        dirs[0].common.label.as_deref(),
        Some("lilypond:artic,flageolet")
    );
}

#[test]
fn import_combined_string_and_open() {
    let mei = parse_and_import("{ c4-\\1 -\\open }");
    let dirs = measure_dirs(&mei);
    assert_eq!(dirs.len(), 2, "expected string number + open");
    assert_eq!(dirs[0].common.label.as_deref(), Some("lilypond:string,1"));
    assert_eq!(dirs[1].common.label.as_deref(), Some("lilypond:artic,open"));
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
    let mei = parse_and_import("{ \\tuplet 3/2 { c8 d e } }");
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
    let mei = parse_and_import("{ \\tuplet 3/2 4 { c8 d e f g a } }");
    let spans = measure_tuplet_spans(&mei);
    assert_eq!(spans.len(), 1);
    let ts = &spans[0];
    assert_eq!(ts.tuplet_span_log.num.as_deref(), Some("3"));
    assert_eq!(ts.tuplet_span_log.numbase.as_deref(), Some("2"));
    // Label should contain span duration info
    let label = ts.common.label.as_deref().unwrap();
    assert!(
        label.contains("span=4"),
        "label should contain span=4: {label}"
    );
}

#[test]
fn import_tuplet_5_4() {
    let mei = parse_and_import("{ \\tuplet 5/4 { c16 d e f g } }");
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
    let mei = parse_and_import("{ \\tuplet 3/2 { \\tuplet 3/2 { c32 d e } f16 g } }");
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
    let mei = parse_and_import("{ \\tuplet 3/2 { c8 d e } }");
    let spans = measure_tuplet_spans(&mei);
    let label = spans[0].common.label.as_deref().unwrap();
    assert!(
        label.starts_with("lilypond:tuplet,3/2"),
        "label should start with lilypond:tuplet,3/2: {label}"
    );
}
