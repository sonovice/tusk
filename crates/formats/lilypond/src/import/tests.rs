use super::*;
use crate::parser::Parser;
use tusk_model::elements::{
    ChordChild, Dynam, DynamChild, Hairpin, Mei, MeiChild, ScoreDef, Slur, Staff, StaffDef,
    StaffGrpChild,
};
use tusk_model::generated::data::{
    DataClefshape, DataDuration, DataDurationCmn, DataDurationrests, DataStaffrelBasic,
};

fn parse_and_import(src: &str) -> Mei {
    let file = Parser::new(src).unwrap().parse().unwrap();
    import(&file).unwrap()
}

/// Walk MEI to find the first staff in the first measure.
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
                                            if let MeasureChild::Staff(staff) = mc2 {
                                                return Some(staff);
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

/// Walk MEI to find all staves in the first measure.
fn all_staves(mei: &Mei) -> Vec<&Staff> {
    let mut staves = Vec::new();
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
                                            if let MeasureChild::Staff(staff) = mc2 {
                                                staves.push(staff.as_ref());
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
    staves
}

/// Walk MEI to find the scoreDef.
fn find_score_def(mei: &Mei) -> Option<&ScoreDef> {
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
                                return Some(sd);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Walk MEI to find layer children (first layer of first staff).
fn layer_children(mei: &Mei) -> &[LayerChild] {
    if let Some(staff) = first_staff(mei)
        && let Some(StaffChild::Layer(layer)) = staff.children.first()
    {
        return &layer.children;
    }
    &[]
}

/// Count the number of layers in the first staff.
fn layer_count(mei: &Mei) -> usize {
    first_staff(mei).map(|s| s.children.len()).unwrap_or(0)
}

/// Get layer children for a specific layer index (0-based).
fn nth_layer_children(mei: &Mei, idx: usize) -> &[LayerChild] {
    if let Some(staff) = first_staff(mei)
        && let Some(StaffChild::Layer(layer)) = staff.children.get(idx)
    {
        return &layer.children;
    }
    &[]
}

#[test]
fn import_single_note() {
    let mei = parse_and_import("{ c'4 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 1);
    if let LayerChild::Note(note) = &children[0] {
        assert_eq!(note.note_log.pname.as_ref().unwrap().0, "c");
        assert_eq!(note.note_log.oct.as_ref().unwrap().0, 4); // c' = oct 4
        assert!(matches!(
            note.note_log.dur,
            Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4))
        ));
    } else {
        panic!("expected Note");
    }
}

#[test]
fn import_note_with_accidental() {
    let mei = parse_and_import("{ cis''2 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 1);
    if let LayerChild::Note(note) = &children[0] {
        assert_eq!(note.note_log.pname.as_ref().unwrap().0, "c");
        assert_eq!(note.note_log.oct.as_ref().unwrap().0, 5); // c'' = oct 5
        assert!(note.note_ges.accid_ges.is_some()); // sharp
        assert!(matches!(
            note.note_log.dur,
            Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N2))
        ));
    } else {
        panic!("expected Note");
    }
}

#[test]
fn import_rest() {
    let mei = parse_and_import("{ r4 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 1);
    if let LayerChild::Rest(rest) = &children[0] {
        assert!(matches!(
            rest.rest_log.dur,
            Some(DataDurationrests::MeiDataDurationCmn(DataDurationCmn::N4))
        ));
    } else {
        panic!("expected Rest");
    }
}

#[test]
fn import_dotted_rest() {
    let mei = parse_and_import("{ r2. }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 1);
    if let LayerChild::Rest(rest) = &children[0] {
        assert!(matches!(
            rest.rest_log.dur,
            Some(DataDurationrests::MeiDataDurationCmn(DataDurationCmn::N2))
        ));
        assert_eq!(rest.rest_log.dots.as_ref().unwrap().0, 1);
    } else {
        panic!("expected Rest");
    }
}

#[test]
fn import_multi_measure_rest() {
    let mei = parse_and_import("{ R1*4 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 1);
    if let LayerChild::MRest(mrest) = &children[0] {
        assert!(mrest.common.label.is_some());
        let label = mrest.common.label.as_ref().unwrap();
        assert!(label.starts_with("tusk:mrest,"));
        assert!(label.contains("\"base\":1"));
        assert!(label.contains("[4,1]"));
    } else {
        panic!("expected MRest");
    }
}

#[test]
fn import_pitched_rest() {
    let mei = parse_and_import("{ c4\\rest }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 1);
    if let LayerChild::Rest(rest) = &children[0] {
        assert!(rest.common.label.is_some());
        assert!(
            rest.common
                .label
                .as_ref()
                .unwrap()
                .starts_with("tusk:pitched-rest,")
        );
    } else {
        panic!("expected Rest for pitched rest");
    }
}

#[test]
fn import_multiple_events() {
    let mei = parse_and_import("{ c4 d8 r4 e16 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 4);
    assert!(matches!(&children[0], LayerChild::Note(_)));
    assert!(matches!(&children[1], LayerChild::Note(_)));
    assert!(matches!(&children[2], LayerChild::Rest(_)));
    assert!(matches!(&children[3], LayerChild::Note(_)));
}

#[test]
fn import_skip_ignored() {
    let mei = parse_and_import("{ c4 s4 d4 }");
    let children = layer_children(&mei);
    // Skip is ignored, so only c4 and d4
    assert_eq!(children.len(), 2);
}

#[test]
fn import_from_score_block() {
    let mei = parse_and_import("\\score { { c4 d4 } }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 2);
}

#[test]
fn import_nested_relative() {
    let mei = parse_and_import("\\relative c' { c4 d e f }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 4);
}

#[test]
fn import_simultaneous_two_voices() {
    let mei = parse_and_import("<< { c'4 d'4 } { e'4 f'4 } >>");
    assert_eq!(layer_count(&mei), 2);
    let voice1 = nth_layer_children(&mei, 0);
    let voice2 = nth_layer_children(&mei, 1);
    assert_eq!(voice1.len(), 2);
    assert_eq!(voice2.len(), 2);
    // Voice 1: c d
    if let LayerChild::Note(n) = &voice1[0] {
        assert_eq!(n.note_log.pname.as_ref().unwrap().0, "c");
    } else {
        panic!("expected Note");
    }
    // Voice 2: e f
    if let LayerChild::Note(n) = &voice2[0] {
        assert_eq!(n.note_log.pname.as_ref().unwrap().0, "e");
    } else {
        panic!("expected Note");
    }
}

#[test]
fn import_simultaneous_three_voices() {
    let mei = parse_and_import("<< { c'4 } { e'4 } { g'4 } >>");
    assert_eq!(layer_count(&mei), 3);
    assert_eq!(nth_layer_children(&mei, 0).len(), 1);
    assert_eq!(nth_layer_children(&mei, 1).len(), 1);
    assert_eq!(nth_layer_children(&mei, 2).len(), 1);
}

#[test]
fn import_sequential_single_layer() {
    let mei = parse_and_import("{ c'4 d'4 e'4 }");
    assert_eq!(layer_count(&mei), 1);
    assert_eq!(layer_children(&mei).len(), 3);
}

#[test]
fn import_nested_sequential_in_simultaneous() {
    // Outer sequential wrapping simultaneous
    let mei = parse_and_import("{ << { c'4 } { e'4 } >> }");
    // The outer sequential contains a simultaneous -- but find_music
    // walks into it and finds the simultaneous at the section level
    // The top-level is Sequential([Simultaneous([...])]) -- the
    // collect_events will flatten both voices into one layer since
    // extract_voices sees a Sequential at top level
    assert_eq!(layer_count(&mei), 1);
}

// --- Phase 5.2: Context import tests ---

#[test]
fn import_new_staff_creates_staff() {
    let mei = parse_and_import("\\new Staff { c'4 d'4 }");
    let staves = all_staves(&mei);
    assert_eq!(staves.len(), 1);
    assert_eq!(staves[0].n_integer.n.as_deref(), Some("1"));
    // Should have one layer with 2 notes
    assert_eq!(staves[0].children.len(), 1);
}

#[test]
fn import_staff_group_creates_multiple_staves() {
    let mei =
        parse_and_import("\\new StaffGroup << \\new Staff { c'4 d'4 } \\new Staff { e'4 f'4 } >>");
    let staves = all_staves(&mei);
    assert_eq!(staves.len(), 2);
    assert_eq!(staves[0].n_integer.n.as_deref(), Some("1"));
    assert_eq!(staves[1].n_integer.n.as_deref(), Some("2"));
}

#[test]
fn import_staff_group_symbol() {
    let mei = parse_and_import("\\new StaffGroup << \\new Staff { c'4 } \\new Staff { e'4 } >>");
    let sd = find_score_def(&mei).unwrap();
    let sg = &sd.children[0];
    if let ScoreDefChild::StaffGrp(grp) = sg {
        assert_eq!(grp.staff_grp_vis.symbol.as_deref(), Some("bracket"));
    } else {
        panic!("expected StaffGrp");
    }
}

#[test]
fn import_piano_staff_symbol() {
    let mei = parse_and_import("\\new PianoStaff << \\new Staff { c'4 } \\new Staff { e'4 } >>");
    let sd = find_score_def(&mei).unwrap();
    if let ScoreDefChild::StaffGrp(grp) = &sd.children[0] {
        assert_eq!(grp.staff_grp_vis.symbol.as_deref(), Some("brace"));
    } else {
        panic!("expected StaffGrp");
    }
}

#[test]
fn import_named_staff_label() {
    let mei = parse_and_import("\\new Staff = \"violin\" { c'4 }");
    let sd = find_score_def(&mei).unwrap();
    if let ScoreDefChild::StaffGrp(grp) = &sd.children[0] {
        if let StaffGrpChild::StaffDef(sdef) = &grp.children[0] {
            let label = sdef.labelled.label.as_deref().unwrap();
            assert!(label.contains("\"name\":\"violin\""), "label: {label}");
        } else {
            panic!("expected StaffDef");
        }
    }
}

#[test]
fn import_group_label() {
    let mei = parse_and_import("\\new StaffGroup = \"orch\" << \\new Staff { c'4 } >>");
    let sd = find_score_def(&mei).unwrap();
    if let ScoreDefChild::StaffGrp(grp) = &sd.children[0] {
        let label = grp.common.label.as_deref().unwrap();
        assert!(label.contains("tusk:group-context,"), "label: {label}");
        assert!(label.contains("\"name\":\"orch\""), "label: {label}");
    }
}

#[test]
fn import_staff_count_from_fixture() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_contexts.ly"
    ))
    .unwrap();
    let mei = parse_and_import(&src);
    let staves = all_staves(&mei);
    assert_eq!(staves.len(), 2, "fragment_contexts.ly should have 2 staves");
}

/// Find the first staffDef in the scoreDef.
fn first_staff_def(mei: &Mei) -> Option<&StaffDef> {
    let sd = find_score_def(mei)?;
    for child in &sd.children {
        if let ScoreDefChild::StaffGrp(grp) = child {
            for gc in &grp.children {
                if let StaffGrpChild::StaffDef(sdef) = gc {
                    return Some(sdef);
                }
            }
        }
    }
    None
}

// --- Phase 6.2: Clef/key/time import tests ---

#[test]
fn import_clef_sets_staff_def() {
    let mei = parse_and_import("{ \\clef \"treble\" c'4 }");
    let sdef = first_staff_def(&mei).unwrap();
    assert_eq!(
        sdef.staff_def_log.clef_shape,
        Some(DataClefshape::G),
        "treble clef should be G shape"
    );
    assert_eq!(
        sdef.staff_def_log.clef_line.as_ref().map(|l| l.0),
        Some(2),
        "treble clef should be on line 2"
    );
}

#[test]
fn import_bass_clef_sets_staff_def() {
    let mei = parse_and_import("{ \\clef \"bass\" c4 }");
    let sdef = first_staff_def(&mei).unwrap();
    assert_eq!(sdef.staff_def_log.clef_shape, Some(DataClefshape::F));
    assert_eq!(sdef.staff_def_log.clef_line.as_ref().map(|l| l.0), Some(4));
}

#[test]
fn import_alto_clef_sets_staff_def() {
    let mei = parse_and_import("{ \\clef \"alto\" c'4 }");
    let sdef = first_staff_def(&mei).unwrap();
    assert_eq!(sdef.staff_def_log.clef_shape, Some(DataClefshape::C));
    assert_eq!(sdef.staff_def_log.clef_line.as_ref().map(|l| l.0), Some(3));
}

#[test]
fn import_key_sets_staff_def() {
    let mei = parse_and_import("{ \\key d \\major c'4 }");
    let sdef = first_staff_def(&mei).unwrap();
    // D major = 2 sharps
    assert_eq!(
        sdef.staff_def_log.keysig.as_ref().map(|k| k.0.as_str()),
        Some("2")
    );
}

#[test]
fn import_key_minor_sets_staff_def() {
    let mei = parse_and_import("{ \\key a \\minor c'4 }");
    let sdef = first_staff_def(&mei).unwrap();
    // A minor = 0 sharps/flats
    assert_eq!(
        sdef.staff_def_log.keysig.as_ref().map(|k| k.0.as_str()),
        Some("0")
    );
}

#[test]
fn import_key_flat_sets_staff_def() {
    let mei = parse_and_import("{ \\key bes \\major c'4 }");
    let sdef = first_staff_def(&mei).unwrap();
    // Bb major = -2
    assert_eq!(
        sdef.staff_def_log.keysig.as_ref().map(|k| k.0.as_str()),
        Some("-2")
    );
}

#[test]
fn import_time_sets_staff_def() {
    let mei = parse_and_import("{ \\time 3/4 c'4 }");
    let sdef = first_staff_def(&mei).unwrap();
    assert_eq!(sdef.staff_def_log.meter_count.as_deref(), Some("3"));
    assert_eq!(sdef.staff_def_log.meter_unit.as_deref(), Some("4"));
}

#[test]
fn import_time_compound_sets_staff_def() {
    let mei = parse_and_import("{ \\time 2+3/8 c'4 }");
    let sdef = first_staff_def(&mei).unwrap();
    assert_eq!(sdef.staff_def_log.meter_count.as_deref(), Some("2+3"));
    assert_eq!(sdef.staff_def_log.meter_unit.as_deref(), Some("8"));
}

#[test]
fn import_clef_key_time_label_stored() {
    let mei = parse_and_import("{ \\clef \"treble\" \\key d \\major \\time 4/4 c'4 d'4 }");
    let sdef = first_staff_def(&mei).unwrap();
    let label = sdef.labelled.label.as_deref().unwrap();
    assert!(label.contains("tusk:events,"), "label: {label}");
    assert!(
        label.contains("\"Clef\""),
        "label should contain Clef event: {label}"
    );
    assert!(
        label.contains("\"Key\""),
        "label should contain Key event: {label}"
    );
    assert!(
        label.contains("\"Time\""),
        "label should contain Time event: {label}"
    );
}

#[test]
fn import_clef_change_mid_stream() {
    let mei = parse_and_import("{ \\clef \"treble\" c'4 d'4 \\clef \"bass\" e4 f4 }");
    let sdef = first_staff_def(&mei).unwrap();
    // First clef is treble
    assert_eq!(sdef.staff_def_log.clef_shape, Some(DataClefshape::G));
    // Label has both clefs
    let label = sdef.labelled.label.as_deref().unwrap();
    assert!(
        label.contains("\"Clef\""),
        "label should contain Clef events: {label}"
    );
    assert!(
        label.contains("\"treble\""),
        "label should contain treble: {label}"
    );
    assert!(
        label.contains("\"bass\""),
        "label should contain bass: {label}"
    );
}

#[test]
fn import_transposed_clef() {
    let mei = parse_and_import("{ \\clef \"treble_8\" c4 }");
    let sdef = first_staff_def(&mei).unwrap();
    assert_eq!(sdef.staff_def_log.clef_shape, Some(DataClefshape::G));
    assert_eq!(sdef.staff_def_log.clef_line.as_ref().map(|l| l.0), Some(2));
    assert_eq!(sdef.staff_def_log.clef_dis.as_ref().map(|d| d.0), Some(8));
    assert_eq!(
        sdef.staff_def_log.clef_dis_place,
        Some(DataStaffrelBasic::Below)
    );
}

#[test]
fn import_staff_with_block_label() {
    let mei =
        parse_and_import("\\new Staff \\with { \\consists \"Span_arpeggio_engraver\" } { c'4 }");
    let sd = find_score_def(&mei).unwrap();
    if let ScoreDefChild::StaffGrp(grp) = &sd.children[0] {
        if let StaffGrpChild::StaffDef(sdef) = &grp.children[0] {
            let label = sdef.labelled.label.as_deref().unwrap();
            assert!(
                label.contains("\"with_block\""),
                "label should contain with_block: {label}"
            );
            assert!(label.contains("Span_arpeggio_engraver"), "label: {label}");
        } else {
            panic!("expected StaffDef");
        }
    }
}

// --- Phase 7.2: Relative / transpose import tests ---

#[test]
fn import_relative_resolves_pitches() {
    // \relative c' { c d e f } -> absolute: c' d' e' f'
    let mei = parse_and_import("\\relative c' { c4 d e f }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 4);

    // c' (octave 4)
    if let LayerChild::Note(n) = &children[0] {
        assert_eq!(n.note_log.pname.as_ref().unwrap().0, "c");
        assert_eq!(n.note_log.oct.as_ref().unwrap().0, 4);
    } else {
        panic!("expected Note");
    }
    // d' (octave 4)
    if let LayerChild::Note(n) = &children[1] {
        assert_eq!(n.note_log.pname.as_ref().unwrap().0, "d");
        assert_eq!(n.note_log.oct.as_ref().unwrap().0, 4);
    } else {
        panic!("expected Note");
    }
    // e' (octave 4)
    if let LayerChild::Note(n) = &children[2] {
        assert_eq!(n.note_log.pname.as_ref().unwrap().0, "e");
        assert_eq!(n.note_log.oct.as_ref().unwrap().0, 4);
    } else {
        panic!("expected Note");
    }
    // f' (octave 4)
    if let LayerChild::Note(n) = &children[3] {
        assert_eq!(n.note_log.pname.as_ref().unwrap().0, "f");
        assert_eq!(n.note_log.oct.as_ref().unwrap().0, 4);
    } else {
        panic!("expected Note");
    }
}

#[test]
fn import_relative_descending() {
    // \relative c' { c b a g } -> c'=4, b=3, a=3, g=3
    let mei = parse_and_import("\\relative c' { c4 b a g }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 4);

    if let LayerChild::Note(n) = &children[0] {
        assert_eq!(n.note_log.oct.as_ref().unwrap().0, 4); // c'
    }
    if let LayerChild::Note(n) = &children[1] {
        assert_eq!(n.note_log.oct.as_ref().unwrap().0, 3); // b (below c')
    }
    if let LayerChild::Note(n) = &children[2] {
        assert_eq!(n.note_log.oct.as_ref().unwrap().0, 3); // a (below b)
    }
    if let LayerChild::Note(n) = &children[3] {
        assert_eq!(n.note_log.oct.as_ref().unwrap().0, 3); // g (below a)
    }
}

#[test]
fn import_relative_label_stored() {
    let mei = parse_and_import("\\relative c' { c4 d e f }");
    let sdef = first_staff_def(&mei).unwrap();
    let label = sdef.labelled.label.as_deref().unwrap();
    assert!(
        label.contains("tusk:pitch-context,"),
        "label should contain pitch context: {label}"
    );
}

#[test]
fn import_transpose_applies() {
    // \transpose c d { c4 } -> c transposed up a whole step = d
    let mei = parse_and_import("\\transpose c d { c4 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 1);
    if let LayerChild::Note(n) = &children[0] {
        assert_eq!(n.note_log.pname.as_ref().unwrap().0, "d");
    } else {
        panic!("expected Note");
    }
}

#[test]
fn import_transpose_label_stored() {
    let mei = parse_and_import("\\transpose c d { c4 }");
    let sdef = first_staff_def(&mei).unwrap();
    let label = sdef.labelled.label.as_deref().unwrap();
    assert!(
        label.contains("tusk:pitch-context,"),
        "label should contain pitch context: {label}"
    );
}

#[test]
fn import_chord_basic() {
    let mei = parse_and_import("{ <c' e' g'>4 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 1);
    if let LayerChild::Chord(chord) = &children[0] {
        assert!(matches!(
            chord.chord_log.dur,
            Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4))
        ));
        assert_eq!(chord.children.len(), 3);
        // First note: c'
        let ChordChild::Note(n) = &chord.children[0];
        assert_eq!(n.note_log.pname.as_ref().unwrap().0, "c");
        assert_eq!(n.note_log.oct.as_ref().unwrap().0, 4);
        // Second note: e'
        let ChordChild::Note(n) = &chord.children[1];
        assert_eq!(n.note_log.pname.as_ref().unwrap().0, "e");
        assert_eq!(n.note_log.oct.as_ref().unwrap().0, 4);
        // Third note: g'
        let ChordChild::Note(n) = &chord.children[2];
        assert_eq!(n.note_log.pname.as_ref().unwrap().0, "g");
        assert_eq!(n.note_log.oct.as_ref().unwrap().0, 4);
    } else {
        panic!("expected Chord, got: {:?}", children[0]);
    }
}

#[test]
fn import_chord_dotted() {
    let mei = parse_and_import("{ <c' e'>2. }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 1);
    if let LayerChild::Chord(chord) = &children[0] {
        assert!(matches!(
            chord.chord_log.dur,
            Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N2))
        ));
        assert_eq!(chord.chord_log.dots.as_ref().unwrap().0, 1);
        assert_eq!(chord.children.len(), 2);
    } else {
        panic!("expected Chord");
    }
}

#[test]
fn import_chord_with_accidentals() {
    let mei = parse_and_import("{ <cis' es' g'>4 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 1);
    if let LayerChild::Chord(chord) = &children[0] {
        assert_eq!(chord.children.len(), 3);
        // cis' -- sharp
        let ChordChild::Note(n) = &chord.children[0];
        assert!(n.note_ges.accid_ges.is_some());
        // es' -- flat
        let ChordChild::Note(n) = &chord.children[1];
        assert!(n.note_ges.accid_ges.is_some());
        // g' -- natural (no accidental)
        let ChordChild::Note(n) = &chord.children[2];
        assert!(n.note_ges.accid_ges.is_none());
    } else {
        panic!("expected Chord");
    }
}

#[test]
fn import_chord_force_cautionary() {
    let mei = parse_and_import("{ <cis'! e'?>4 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 1);
    if let LayerChild::Chord(chord) = &children[0] {
        assert_eq!(chord.children.len(), 2);
        // cis'! -- forced accidental
        let ChordChild::Note(n) = &chord.children[0];
        assert!(!n.children.is_empty(), "should have Accid child");
        // e'? -- cautionary
        let ChordChild::Note(n) = &chord.children[1];
        assert!(!n.children.is_empty(), "should have Accid child");
    } else {
        panic!("expected Chord");
    }
}

#[test]
fn import_chord_mixed_with_notes() {
    let mei = parse_and_import("{ c'4 <d' f'>8 e'2 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 3);
    assert!(matches!(children[0], LayerChild::Note(_)));
    assert!(matches!(children[1], LayerChild::Chord(_)));
    assert!(matches!(children[2], LayerChild::Note(_)));
}

// --- Phase 9.2: Tie / Slur / Phrasing slur import tests ---

/// Collect all Slur control events from the first measure.
fn measure_slurs(mei: &Mei) -> Vec<&Slur> {
    let mut slurs = Vec::new();
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
                                            if let MeasureChild::Slur(slur) = mc2 {
                                                slurs.push(slur.as_ref());
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
    slurs
}

#[test]
fn import_tie_sets_note_attr() {
    let mei = parse_and_import("{ c4~ c4 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 2);
    // First note: tie="i" (initial)
    if let LayerChild::Note(n) = &children[0] {
        assert_eq!(n.note_anl.tie.as_ref().unwrap().0, "i");
    } else {
        panic!("expected Note");
    }
    // Second note: tie="t" (terminal)
    if let LayerChild::Note(n) = &children[1] {
        assert_eq!(n.note_anl.tie.as_ref().unwrap().0, "t");
    } else {
        panic!("expected Note");
    }
}

#[test]
fn import_slur_creates_control_event() {
    let mei = parse_and_import("{ d4( e4 f4) }");
    let slurs = measure_slurs(&mei);
    assert_eq!(slurs.len(), 1, "expected 1 slur control event");
    let slur = slurs[0];
    assert!(slur.slur_log.startid.is_some());
    assert!(slur.slur_log.endid.is_some());
    // Not a phrase
    assert!(slur.common.label.is_none() || slur.common.label.as_deref() != Some("lilypond:phrase"));
}

#[test]
fn import_phrasing_slur_creates_labeled_control_event() {
    let mei = parse_and_import("{ g4\\( a4 b4\\) }");
    let slurs = measure_slurs(&mei);
    assert_eq!(slurs.len(), 1, "expected 1 phrase control event");
    let slur = slurs[0];
    assert_eq!(slur.common.label.as_deref(), Some("lilypond:phrase"));
    assert!(slur.slur_log.startid.is_some());
    assert!(slur.slur_log.endid.is_some());
}

#[test]
fn import_chord_tie() {
    let mei = parse_and_import("{ <c e g>4~ <c e g>4 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 2);
    // First chord: all notes have tie="i"
    if let LayerChild::Chord(chord) = &children[0] {
        for child in &chord.children {
            let ChordChild::Note(n) = child;
            assert_eq!(n.note_anl.tie.as_ref().unwrap().0, "i");
        }
    } else {
        panic!("expected Chord");
    }
    // Second chord: all notes have tie="t"
    if let LayerChild::Chord(chord) = &children[1] {
        for child in &chord.children {
            let ChordChild::Note(n) = child;
            assert_eq!(n.note_anl.tie.as_ref().unwrap().0, "t");
        }
    } else {
        panic!("expected Chord");
    }
}

#[test]
fn import_combined_tie_and_slur() {
    let mei = parse_and_import("{ c4~( d4 e4) }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 3);
    // First note has tie="i"
    if let LayerChild::Note(n) = &children[0] {
        assert_eq!(n.note_anl.tie.as_ref().unwrap().0, "i");
    }
    // Second note has tie="t" (continuation from first)
    if let LayerChild::Note(n) = &children[1] {
        assert_eq!(n.note_anl.tie.as_ref().unwrap().0, "t");
    }
    // Slur from first to last note
    let slurs = measure_slurs(&mei);
    assert_eq!(slurs.len(), 1);
}

// --- Phase 10.2: Beam import tests ---

#[test]
fn import_beam_creates_beam_element() {
    let mei = parse_and_import("{ c8[ d e f] }");
    let children = layer_children(&mei);
    assert_eq!(
        children.len(),
        1,
        "expected 1 beam element, got {children:?}"
    );
    if let LayerChild::Beam(beam) = &children[0] {
        assert_eq!(beam.children.len(), 4, "beam should contain 4 notes");
        assert!(beam.common.xml_id.is_some());
    } else {
        panic!("expected Beam, got {:?}", children[0]);
    }
}

#[test]
fn import_multiple_beams() {
    let mei = parse_and_import("{ c8[ d] e8[ f] }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 2, "expected 2 beam elements");
    for child in children {
        if let LayerChild::Beam(beam) = child {
            assert_eq!(beam.children.len(), 2);
        } else {
            panic!("expected Beam");
        }
    }
}

#[test]
fn import_beam_with_unbeamed_notes() {
    let mei = parse_and_import("{ c4 d8[ e f] g4 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 3, "expected note + beam + note");
    assert!(matches!(children[0], LayerChild::Note(_)));
    assert!(matches!(children[1], LayerChild::Beam(_)));
    assert!(matches!(children[2], LayerChild::Note(_)));
}

#[test]
fn import_autobeam_in_event_label() {
    let mei = parse_and_import("{ \\autoBeamOff c8 d \\autoBeamOn e8 }");
    let sd = first_staff_def(&mei).unwrap();
    let label = sd.labelled.label.as_deref().unwrap_or("");
    assert!(
        label.contains("AutoBeamOff"),
        "label should contain AutoBeamOff: {label}"
    );
    assert!(
        label.contains("AutoBeamOn"),
        "label should contain AutoBeamOn: {label}"
    );
}

#[test]
fn import_beam_preserves_note_content() {
    let mei = parse_and_import("{ cis'8[ d' ees' f'] }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 1);
    if let LayerChild::Beam(beam) = &children[0] {
        assert_eq!(beam.children.len(), 4);
        // Check first note has correct pitch
        if let tusk_model::elements::BeamChild::Note(n) = &beam.children[0] {
            assert_eq!(n.note_log.pname.as_ref().unwrap().0, "c");
            assert_eq!(n.note_log.oct.as_ref().unwrap().0, 4);
        } else {
            panic!("expected Note in beam");
        }
    } else {
        panic!("expected Beam");
    }
}

// --- Phase 11.2: Dynamics and hairpin import tests ---

/// Collect all Dynam control events from the first measure.
fn measure_dynams(mei: &Mei) -> Vec<&Dynam> {
    let mut dynams = Vec::new();
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
                                            if let MeasureChild::Dynam(d) = mc2 {
                                                dynams.push(d.as_ref());
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
    dynams
}

/// Collect all Hairpin control events from the first measure.
fn measure_hairpins(mei: &Mei) -> Vec<&Hairpin> {
    let mut hairpins = Vec::new();
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
                                            if let MeasureChild::Hairpin(h) = mc2 {
                                                hairpins.push(h.as_ref());
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
    hairpins
}

#[test]
fn import_dynamic_creates_dynam() {
    let mei = parse_and_import("{ c4\\f d4\\p }");
    let dynams = measure_dynams(&mei);
    assert_eq!(dynams.len(), 2, "expected 2 dynam control events");
    // First dynamic: f
    assert_eq!(dynams[0].children.len(), 1);
    let DynamChild::Text(t) = &dynams[0].children[0];
    assert_eq!(t, "f");
    assert!(dynams[0].dynam_log.startid.is_some());
    assert!(dynams[0].dynam_log.staff.is_some());
    // Second dynamic: p
    let DynamChild::Text(t) = &dynams[1].children[0];
    assert_eq!(t, "p");
}

#[test]
fn import_crescendo_hairpin() {
    let mei = parse_and_import("{ c4\\< d4 e4\\! }");
    let hairpins = measure_hairpins(&mei);
    assert_eq!(hairpins.len(), 1, "expected 1 hairpin");
    let hp = hairpins[0];
    assert_eq!(hp.hairpin_log.form.as_deref(), Some("cres"));
    assert!(hp.hairpin_log.startid.is_some());
    assert!(hp.hairpin_log.endid.is_some());
    assert!(hp.hairpin_log.staff.is_some());
}

#[test]
fn import_decrescendo_hairpin() {
    let mei = parse_and_import("{ c4\\> d4 e4\\! }");
    let hairpins = measure_hairpins(&mei);
    assert_eq!(hairpins.len(), 1, "expected 1 hairpin");
    assert_eq!(hairpins[0].hairpin_log.form.as_deref(), Some("dim"));
}

#[test]
fn import_dynamic_and_hairpin_combined() {
    let mei = parse_and_import("{ c4\\f\\< d4 e4\\!\\ff }");
    let dynams = measure_dynams(&mei);
    let hairpins = measure_hairpins(&mei);
    assert_eq!(dynams.len(), 2, "expected 2 dynamics (f and ff)");
    assert_eq!(hairpins.len(), 1, "expected 1 hairpin");
    let DynamChild::Text(t) = &dynams[0].children[0];
    assert_eq!(t, "f");
    let DynamChild::Text(t) = &dynams[1].children[0];
    assert_eq!(t, "ff");
}

#[test]
fn import_hairpin_on_chord() {
    let mei = parse_and_import("{ <c e g>4\\< <d f a>4\\! }");
    let hairpins = measure_hairpins(&mei);
    assert_eq!(hairpins.len(), 1);
    assert_eq!(hairpins[0].hairpin_log.form.as_deref(), Some("cres"));
}

// ---------------------------------------------------------------------------
// Lyrics import tests
// ---------------------------------------------------------------------------

/// Extract verse/syl text from a note's children for a given verse number.
fn get_verse_text(note: &tusk_model::elements::Note, verse_n: &str) -> Option<String> {
    use tusk_model::elements::{NoteChild, SylChild, VerseChild};
    for nc in &note.children {
        if let NoteChild::Verse(verse) = nc
            && verse.common.n.as_ref().is_some_and(|n| n.0 == verse_n)
        {
            for vc in &verse.children {
                if let VerseChild::Syl(syl) = vc
                    && let Some(sc) = syl.children.first()
                {
                    let SylChild::Text(t) = sc;
                    return Some(t.clone());
                }
            }
        }
    }
    None
}

/// Extract syl @con from a note's verse for a given verse number.
fn get_verse_con(note: &tusk_model::elements::Note, verse_n: &str) -> Option<String> {
    use tusk_model::elements::{NoteChild, VerseChild};
    for nc in &note.children {
        if let NoteChild::Verse(verse) = nc
            && verse.common.n.as_ref().is_some_and(|n| n.0 == verse_n)
        {
            for vc in &verse.children {
                if let VerseChild::Syl(syl) = vc {
                    return syl.syl_log.con.clone();
                }
            }
        }
    }
    None
}

/// Extract syl @wordpos from a note's verse for a given verse number.
fn get_verse_wordpos(note: &tusk_model::elements::Note, verse_n: &str) -> Option<String> {
    use tusk_model::elements::{NoteChild, VerseChild};
    for nc in &note.children {
        if let NoteChild::Verse(verse) = nc
            && verse.common.n.as_ref().is_some_and(|n| n.0 == verse_n)
        {
            for vc in &verse.children {
                if let VerseChild::Syl(syl) = vc {
                    return syl.syl_log.wordpos.clone();
                }
            }
        }
    }
    None
}

/// Extract syl @label from a note's verse for a given verse number.
fn get_verse_syl_label(note: &tusk_model::elements::Note, verse_n: &str) -> Option<String> {
    use tusk_model::elements::{NoteChild, VerseChild};
    for nc in &note.children {
        if let NoteChild::Verse(verse) = nc
            && verse.common.n.as_ref().is_some_and(|n| n.0 == verse_n)
        {
            for vc in &verse.children {
                if let VerseChild::Syl(syl) = vc {
                    return syl.common.label.clone();
                }
            }
        }
    }
    None
}

#[test]
fn import_addlyrics_basic() {
    let mei = parse_and_import("{ c'4 d'4 e'4 f'4 } \\addlyrics { one two three four }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 4);
    // Check verse text on each note
    if let LayerChild::Note(n) = &children[0] {
        assert_eq!(get_verse_text(n, "1").as_deref(), Some("one"));
    }
    if let LayerChild::Note(n) = &children[1] {
        assert_eq!(get_verse_text(n, "1").as_deref(), Some("two"));
    }
    if let LayerChild::Note(n) = &children[2] {
        assert_eq!(get_verse_text(n, "1").as_deref(), Some("three"));
    }
    if let LayerChild::Note(n) = &children[3] {
        assert_eq!(get_verse_text(n, "1").as_deref(), Some("four"));
    }
}

#[test]
fn import_addlyrics_hyphens() {
    let mei = parse_and_import("{ c'4 d'4 e'4 } \\addlyrics { hel -- lo world }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 3);
    if let LayerChild::Note(n) = &children[0] {
        assert_eq!(get_verse_text(n, "1").as_deref(), Some("hel"));
        assert_eq!(get_verse_con(n, "1").as_deref(), Some("d"));
        assert_eq!(get_verse_wordpos(n, "1").as_deref(), Some("i"));
    }
    if let LayerChild::Note(n) = &children[1] {
        assert_eq!(get_verse_text(n, "1").as_deref(), Some("lo"));
        assert_eq!(get_verse_con(n, "1"), None);
        assert_eq!(get_verse_wordpos(n, "1").as_deref(), Some("t"));
    }
    if let LayerChild::Note(n) = &children[2] {
        assert_eq!(get_verse_text(n, "1").as_deref(), Some("world"));
        assert_eq!(get_verse_con(n, "1"), None);
        assert_eq!(get_verse_wordpos(n, "1"), None);
    }
}

#[test]
fn import_addlyrics_extender() {
    let mei = parse_and_import("{ c'4 d'4 } \\addlyrics { hold __ rest }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 2);
    if let LayerChild::Note(n) = &children[0] {
        assert_eq!(get_verse_text(n, "1").as_deref(), Some("hold"));
        assert_eq!(
            get_verse_syl_label(n, "1").as_deref(),
            Some("tusk:extender,null")
        );
    }
    if let LayerChild::Note(n) = &children[1] {
        assert_eq!(get_verse_text(n, "1").as_deref(), Some("rest"));
    }
}

#[test]
fn import_addlyrics_label_on_staffdef() {
    let mei = parse_and_import("{ c'4 d'4 } \\addlyrics { do re }");
    let sd = find_score_def(&mei).unwrap();
    // Should have lyrics label on staffDef
    let sdef = find_staff_def(sd);
    assert!(sdef.is_some());
    let label = sdef.unwrap().labelled.label.as_deref().unwrap_or("");
    assert!(
        label.contains("tusk:lyrics-info,"),
        "label should contain lyrics info: {label}"
    );
}

/// Helper to find first staffDef in a scoreDef.
fn find_staff_def(sd: &ScoreDef) -> Option<&StaffDef> {
    for c in &sd.children {
        if let ScoreDefChild::StaffGrp(grp) = c {
            for gc in &grp.children {
                if let StaffGrpChild::StaffDef(sdef) = gc {
                    return Some(sdef);
                }
            }
        }
    }
    None
}

#[test]
fn import_lyricsto_basic() {
    let src = r#"\score {
  <<
    \new Voice = "melody" { c'4 d'4 }
    \new Lyrics \lyricsto "melody" { do re }
  >>
}"#;
    let mei = parse_and_import(src);
    let children = layer_children(&mei);
    assert_eq!(children.len(), 2);
    if let LayerChild::Note(n) = &children[0] {
        assert_eq!(get_verse_text(n, "1").as_deref(), Some("do"));
    }
    if let LayerChild::Note(n) = &children[1] {
        assert_eq!(get_verse_text(n, "1").as_deref(), Some("re"));
    }
}

// Articulation, ornament, tremolo, and technical import tests moved to tests_control.rs
