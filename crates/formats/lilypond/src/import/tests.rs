use super::*;
use crate::parser::Parser;
use tusk_model::elements::{
    ChordChild, Dynam, DynamChild, Hairpin, Mei, MeiChild, ScoreDef, Slur, Staff, StaffDef,
    StaffGrpChild,
};
use tusk_model::generated::data::{
    DataClefshape, DataDuration, DataDurationCmn, DataDurationrests, DataStaffrelBasic,
};

fn parse_and_import(src: &str) -> (Mei, ExtensionStore) {
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
    let (mei, _ext_store) = parse_and_import("{ c'4 }");
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
    let (mei, _ext_store) = parse_and_import("{ cis''2 }");
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
    let (mei, _ext_store) = parse_and_import("{ r4 }");
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
    let (mei, _ext_store) = parse_and_import("{ r2. }");
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
    let (mei, ext_store) = parse_and_import("{ R1*4 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 1);
    if let LayerChild::MRest(mrest) = &children[0] {
        let mrest_id = mrest.common.xml_id.as_deref().unwrap();
        let info = ext_store.mrest_info(mrest_id).expect("should have mrest info");
        assert_eq!(info.base, 1, "base should be 1");
        assert!(!info.multipliers.is_empty(), "should have multipliers");
        assert_eq!(info.multipliers[0], (4, 1), "multiplier should be (4,1)");
    } else {
        panic!("expected MRest");
    }
}

#[test]
fn import_pitched_rest() {
    let (mei, ext_store) = parse_and_import("{ c4\\rest }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 1);
    if let LayerChild::Rest(rest) = &children[0] {
        let rest_id = rest.common.xml_id.as_deref().unwrap();
        let info = ext_store.pitched_rest(rest_id).expect("should have pitched rest info");
        assert!(!info.pitch.is_empty(), "pitch should be set");
    } else {
        panic!("expected Rest for pitched rest");
    }
}

#[test]
fn import_multiple_events() {
    let (mei, _ext_store) = parse_and_import("{ c4 d8 r4 e16 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 4);
    assert!(matches!(&children[0], LayerChild::Note(_)));
    assert!(matches!(&children[1], LayerChild::Note(_)));
    assert!(matches!(&children[2], LayerChild::Rest(_)));
    assert!(matches!(&children[3], LayerChild::Note(_)));
}

#[test]
fn import_skip_preserved() {
    let (mei, _ext_store) = parse_and_import("{ c4 s4 d4 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 3);
    assert!(matches!(&children[0], LayerChild::Note(_)));
    assert!(matches!(&children[1], LayerChild::Space(_)));
    assert!(matches!(&children[2], LayerChild::Note(_)));
}

#[test]
fn import_skip_with_duration_and_dots() {
    let (mei, _ext_store) = parse_and_import("{ s4. }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 1);
    let LayerChild::Space(space) = &children[0] else {
        panic!("expected Space");
    };
    // Check dur = quarter
    assert!(space.space_log.dur.is_some());
    // Check dots = 1
    assert_eq!(space.space_log.dots.as_ref().map(|d| d.0), Some(1));
}

#[test]
fn import_skip_in_voice() {
    // Multi-voice: skip used as spacer in second voice
    let (mei, _ext_store) = parse_and_import("<< { c4 d4 } \\\\ { s4 e4 } >>");
    // Second voice (layer index 1) should have s4 e4
    let children = nth_layer_children(&mei, 1);
    assert_eq!(children.len(), 2);
    assert!(matches!(&children[0], LayerChild::Space(_)));
    assert!(matches!(&children[1], LayerChild::Note(_)));
}

#[test]
fn import_from_score_block() {
    let (mei, _ext_store) = parse_and_import("\\score { { c4 d4 } }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 2);
}

#[test]
fn import_nested_relative() {
    let (mei, _ext_store) = parse_and_import("\\relative c' { c4 d e f }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 4);
}

#[test]
fn import_simultaneous_two_voices() {
    let (mei, _ext_store) = parse_and_import("<< { c'4 d'4 } { e'4 f'4 } >>");
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
    let (mei, _ext_store) = parse_and_import("<< { c'4 } { e'4 } { g'4 } >>");
    assert_eq!(layer_count(&mei), 3);
    assert_eq!(nth_layer_children(&mei, 0).len(), 1);
    assert_eq!(nth_layer_children(&mei, 1).len(), 1);
    assert_eq!(nth_layer_children(&mei, 2).len(), 1);
}

#[test]
fn import_sequential_single_layer() {
    let (mei, _ext_store) = parse_and_import("{ c'4 d'4 e'4 }");
    assert_eq!(layer_count(&mei), 1);
    assert_eq!(layer_children(&mei).len(), 3);
}

#[test]
fn import_nested_sequential_in_simultaneous() {
    // Outer sequential wrapping simultaneous: { << { c'4 } { e'4 } >> }
    // extract_voices unwraps the single-item Sequential and finds the
    // inner Simultaneous, splitting into 2 layers (one per voice)
    let (mei, _ext_store) = parse_and_import("{ << { c'4 } { e'4 } >> }");
    assert_eq!(layer_count(&mei), 2);
}

// --- Phase 5.2: Context import tests ---

#[test]
fn import_new_staff_creates_staff() {
    let (mei, _ext_store) = parse_and_import("\\new Staff { c'4 d'4 }");
    let staves = all_staves(&mei);
    assert_eq!(staves.len(), 1);
    assert_eq!(staves[0].n_integer.n.as_deref(), Some("1"));
    // Should have one layer with 2 notes
    assert_eq!(staves[0].children.len(), 1);
}

#[test]
fn import_staff_group_creates_multiple_staves() {
    let (mei, _ext_store) =
        parse_and_import("\\new StaffGroup << \\new Staff { c'4 d'4 } \\new Staff { e'4 f'4 } >>");
    let staves = all_staves(&mei);
    assert_eq!(staves.len(), 2);
    assert_eq!(staves[0].n_integer.n.as_deref(), Some("1"));
    assert_eq!(staves[1].n_integer.n.as_deref(), Some("2"));
}

#[test]
fn import_staff_group_symbol() {
    let (mei, _ext_store) = parse_and_import("\\new StaffGroup << \\new Staff { c'4 } \\new Staff { e'4 } >>");
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
    let (mei, _ext_store) = parse_and_import("\\new PianoStaff << \\new Staff { c'4 } \\new Staff { e'4 } >>");
    let sd = find_score_def(&mei).unwrap();
    if let ScoreDefChild::StaffGrp(grp) = &sd.children[0] {
        assert_eq!(grp.staff_grp_vis.symbol.as_deref(), Some("brace"));
    } else {
        panic!("expected StaffGrp");
    }
}

#[test]
fn import_named_staff_label() {
    let (mei, ext_store) = parse_and_import("\\new Staff = \"violin\" { c'4 }");
    let sd = find_score_def(&mei).unwrap();
    if let ScoreDefChild::StaffGrp(grp) = &sd.children[0] {
        if let StaffGrpChild::StaffDef(sdef) = &grp.children[0] {
            let sdef_id = sdef.basic.xml_id.as_deref().unwrap();
            let ctx = ext_store.staff_context(sdef_id).expect("should have staff context");
            assert_eq!(ctx.name.as_deref(), Some("violin"), "name should be violin");
        } else {
            panic!("expected StaffDef");
        }
    }
}

#[test]
fn import_group_label() {
    let (mei, ext_store) = parse_and_import("\\new StaffGroup = \"orch\" << \\new Staff { c'4 } >>");
    let sd = find_score_def(&mei).unwrap();
    if let ScoreDefChild::StaffGrp(grp) = &sd.children[0] {
        let grp_id = grp.common.xml_id.as_deref().unwrap();
        let ctx = ext_store.staff_context(grp_id).expect("should have group context");
        assert_eq!(ctx.name.as_deref(), Some("orch"), "name should be orch");
    }
}

#[test]
fn import_staff_count_from_fixture() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_contexts.ly"
    ))
    .unwrap();
    let (mei, _ext_store) = parse_and_import(&src);
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
    let (mei, _ext_store) = parse_and_import("{ \\clef \"treble\" c'4 }");
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
    let (mei, _ext_store) = parse_and_import("{ \\clef \"bass\" c4 }");
    let sdef = first_staff_def(&mei).unwrap();
    assert_eq!(sdef.staff_def_log.clef_shape, Some(DataClefshape::F));
    assert_eq!(sdef.staff_def_log.clef_line.as_ref().map(|l| l.0), Some(4));
}

#[test]
fn import_alto_clef_sets_staff_def() {
    let (mei, _ext_store) = parse_and_import("{ \\clef \"alto\" c'4 }");
    let sdef = first_staff_def(&mei).unwrap();
    assert_eq!(sdef.staff_def_log.clef_shape, Some(DataClefshape::C));
    assert_eq!(sdef.staff_def_log.clef_line.as_ref().map(|l| l.0), Some(3));
}

#[test]
fn import_key_sets_staff_def() {
    let (mei, _ext_store) = parse_and_import("{ \\key d \\major c'4 }");
    let sdef = first_staff_def(&mei).unwrap();
    // D major = 2 sharps
    assert_eq!(
        sdef.staff_def_log.keysig.as_ref().map(|k| k.0.as_str()),
        Some("2")
    );
}

#[test]
fn import_key_minor_sets_staff_def() {
    let (mei, _ext_store) = parse_and_import("{ \\key a \\minor c'4 }");
    let sdef = first_staff_def(&mei).unwrap();
    // A minor = 0 sharps/flats
    assert_eq!(
        sdef.staff_def_log.keysig.as_ref().map(|k| k.0.as_str()),
        Some("0")
    );
}

#[test]
fn import_key_flat_sets_staff_def() {
    let (mei, _ext_store) = parse_and_import("{ \\key bes \\major c'4 }");
    let sdef = first_staff_def(&mei).unwrap();
    // Bb major = -2
    assert_eq!(
        sdef.staff_def_log.keysig.as_ref().map(|k| k.0.as_str()),
        Some("-2")
    );
}

#[test]
fn import_time_sets_staff_def() {
    let (mei, _ext_store) = parse_and_import("{ \\time 3/4 c'4 }");
    let sdef = first_staff_def(&mei).unwrap();
    assert_eq!(sdef.staff_def_log.meter_count.as_deref(), Some("3"));
    assert_eq!(sdef.staff_def_log.meter_unit.as_deref(), Some("4"));
}

#[test]
fn import_time_compound_sets_staff_def() {
    let (mei, _ext_store) = parse_and_import("{ \\time 2+3/8 c'4 }");
    let sdef = first_staff_def(&mei).unwrap();
    assert_eq!(sdef.staff_def_log.meter_count.as_deref(), Some("2+3"));
    assert_eq!(sdef.staff_def_log.meter_unit.as_deref(), Some("8"));
}

#[test]
fn import_clef_key_time_label_stored() {
    let (mei, ext_store) = parse_and_import("{ \\clef \"treble\" \\key d \\major \\time 4/4 c'4 d'4 }");
    let sdef = first_staff_def(&mei).unwrap();
    let sdef_id = sdef.basic.xml_id.as_deref().unwrap();
    let seq = ext_store.event_sequence(sdef_id).expect("should have event_sequence");
    let has_clef = seq.events.iter().any(|e| matches!(e.event, tusk_model::ControlEvent::Clef { .. }));
    let has_key = seq.events.iter().any(|e| matches!(e.event, tusk_model::ControlEvent::Key { .. }));
    let has_time = seq.events.iter().any(|e| matches!(e.event, tusk_model::ControlEvent::Time { .. }));
    assert!(has_clef, "should contain Clef event");
    assert!(has_key, "should contain Key event");
    assert!(has_time, "should contain Time event");
}

#[test]
fn import_clef_change_mid_stream() {
    let (mei, ext_store) = parse_and_import("{ \\clef \"treble\" c'4 d'4 \\clef \"bass\" e4 f4 }");
    let sdef = first_staff_def(&mei).unwrap();
    // First clef is treble
    assert_eq!(sdef.staff_def_log.clef_shape, Some(DataClefshape::G));
    // Event sequence has both clefs
    let sdef_id = sdef.basic.xml_id.as_deref().unwrap();
    let seq = ext_store.event_sequence(sdef_id).expect("should have event_sequence");
    let clef_events: Vec<_> = seq.events.iter().filter(|e| matches!(e.event, tusk_model::ControlEvent::Clef { .. })).collect();
    assert!(clef_events.len() >= 2, "should contain at least 2 Clef events");
    // Check names
    let clef_names: Vec<&str> = clef_events.iter().filter_map(|e| {
        if let tusk_model::ControlEvent::Clef { name } = &e.event { Some(name.as_str()) } else { None }
    }).collect();
    assert!(clef_names.contains(&"treble"), "should contain treble");
    assert!(clef_names.contains(&"bass"), "should contain bass");
}

#[test]
fn import_transposed_clef() {
    let (mei, _ext_store) = parse_and_import("{ \\clef \"treble_8\" c4 }");
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
    let (mei, ext_store) =
        parse_and_import("\\new Staff \\with { \\consists \"Span_arpeggio_engraver\" } { c'4 }");
    let sd = find_score_def(&mei).unwrap();
    if let ScoreDefChild::StaffGrp(grp) = &sd.children[0] {
        if let StaffGrpChild::StaffDef(sdef) = &grp.children[0] {
            let sdef_id = sdef.basic.xml_id.as_deref().unwrap();
            let ctx = ext_store.staff_context(sdef_id).expect("should have staff context");
            assert!(
                ctx.with_block.is_some(),
                "should have with_block"
            );
            assert!(
                ctx.with_block.as_deref().unwrap().contains("Span_arpeggio_engraver"),
                "with_block should contain Span_arpeggio_engraver: {:?}",
                ctx.with_block
            );
        } else {
            panic!("expected StaffDef");
        }
    }
}

// --- Phase 7.2: Relative / transpose import tests ---

#[test]
fn import_relative_resolves_pitches() {
    // \relative c' { c d e f } -> absolute: c' d' e' f'
    let (mei, _ext_store) = parse_and_import("\\relative c' { c4 d e f }");
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
    let (mei, _ext_store) = parse_and_import("\\relative c' { c4 b a g }");
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
    let (mei, ext_store) = parse_and_import("\\relative c' { c4 d e f }");
    let sdef = first_staff_def(&mei).unwrap();
    let sdef_id = sdef.basic.xml_id.as_deref().unwrap();
    let ctx = ext_store.pitch_context(sdef_id).expect("should have pitch context");
    // PitchContext should be Relative variant
    assert!(
        matches!(ctx, tusk_model::PitchContext::Relative { .. }),
        "should be Relative pitch context: {ctx:?}"
    );
}

#[test]
fn import_transpose_applies() {
    // \transpose c d { c4 } -> c transposed up a whole step = d
    let (mei, _ext_store) = parse_and_import("\\transpose c d { c4 }");
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
    let (mei, ext_store) = parse_and_import("\\transpose c d { c4 }");
    let sdef = first_staff_def(&mei).unwrap();
    let sdef_id = sdef.basic.xml_id.as_deref().unwrap();
    let ctx = ext_store.pitch_context(sdef_id).expect("should have pitch context");
    assert!(
        matches!(ctx, tusk_model::PitchContext::Transpose { .. }),
        "should be Transpose pitch context: {ctx:?}"
    );
}

#[test]
fn import_chord_basic() {
    let (mei, _ext_store) = parse_and_import("{ <c' e' g'>4 }");
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
    let (mei, _ext_store) = parse_and_import("{ <c' e'>2. }");
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
    let (mei, _ext_store) = parse_and_import("{ <cis' es' g'>4 }");
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
    let (mei, _ext_store) = parse_and_import("{ <cis'! e'?>4 }");
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
    let (mei, _ext_store) = parse_and_import("{ c'4 <d' f'>8 e'2 }");
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
    let (mei, _ext_store) = parse_and_import("{ c4~ c4 }");
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
    let (mei, ext_store) = parse_and_import("{ d4( e4 f4) }");
    let slurs = measure_slurs(&mei);
    assert_eq!(slurs.len(), 1, "expected 1 slur control event");
    let slur = slurs[0];
    assert!(slur.slur_log.startid.is_some());
    assert!(slur.slur_log.endid.is_some());
    // Not a phrase
    let slur_id = slur.common.xml_id.as_deref().unwrap_or("");
    assert!(
        ext_store.phrasing_slur(slur_id).is_none(),
        "regular slur should not be a phrasing slur"
    );
}

#[test]
fn import_phrasing_slur_creates_labeled_control_event() {
    let (mei, ext_store) = parse_and_import("{ g4\\( a4 b4\\) }");
    let slurs = measure_slurs(&mei);
    assert_eq!(slurs.len(), 1, "expected 1 phrase control event");
    let slur = slurs[0];
    let slur_id = slur.common.xml_id.as_deref().unwrap();
    assert!(
        ext_store.phrasing_slur(slur_id).is_some(),
        "should have phrasing slur in ext_store"
    );
    assert!(slur.slur_log.startid.is_some());
    assert!(slur.slur_log.endid.is_some());
}

#[test]
fn import_chord_tie() {
    let (mei, _ext_store) = parse_and_import("{ <c e g>4~ <c e g>4 }");
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
    let (mei, _ext_store) = parse_and_import("{ c4~( d4 e4) }");
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
    let (mei, _ext_store) = parse_and_import("{ c8[ d e f] }");
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
    let (mei, _ext_store) = parse_and_import("{ c8[ d] e8[ f] }");
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
    let (mei, _ext_store) = parse_and_import("{ c4 d8[ e f] g4 }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 3, "expected note + beam + note");
    assert!(matches!(children[0], LayerChild::Note(_)));
    assert!(matches!(children[1], LayerChild::Beam(_)));
    assert!(matches!(children[2], LayerChild::Note(_)));
}

#[test]
fn import_autobeam_in_event_label() {
    let (mei, ext_store) = parse_and_import("{ \\autoBeamOff c8 d \\autoBeamOn e8 }");
    let sd = first_staff_def(&mei).unwrap();
    let sdef_id = sd.basic.xml_id.as_deref().unwrap();
    let seq = ext_store.event_sequence(sdef_id).expect("should have event_sequence");
    let has_off = seq.events.iter().any(|e| matches!(e.event, tusk_model::ControlEvent::AutoBeamOff));
    let has_on = seq.events.iter().any(|e| matches!(e.event, tusk_model::ControlEvent::AutoBeamOn));
    assert!(has_off, "should contain AutoBeamOff");
    assert!(has_on, "should contain AutoBeamOn");
}

#[test]
fn import_beam_preserves_note_content() {
    let (mei, _ext_store) = parse_and_import("{ cis'8[ d' ees' f'] }");
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
    let (mei, _ext_store) = parse_and_import("{ c4\\f d4\\p }");
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
    let (mei, _ext_store) = parse_and_import("{ c4\\< d4 e4\\! }");
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
    let (mei, _ext_store) = parse_and_import("{ c4\\> d4 e4\\! }");
    let hairpins = measure_hairpins(&mei);
    assert_eq!(hairpins.len(), 1, "expected 1 hairpin");
    assert_eq!(hairpins[0].hairpin_log.form.as_deref(), Some("dim"));
}

#[test]
fn import_dynamic_and_hairpin_combined() {
    let (mei, _ext_store) = parse_and_import("{ c4\\f\\< d4 e4\\!\\ff }");
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
    let (mei, _ext_store) = parse_and_import("{ <c e g>4\\< <d f a>4\\! }");
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

/// Extract syl xml:id from a note's verse for a given verse number.
fn get_verse_syl_id(note: &tusk_model::elements::Note, verse_n: &str) -> Option<String> {
    use tusk_model::elements::{NoteChild, VerseChild};
    for nc in &note.children {
        if let NoteChild::Verse(verse) = nc
            && verse.common.n.as_ref().is_some_and(|n| n.0 == verse_n)
        {
            for vc in &verse.children {
                if let VerseChild::Syl(syl) = vc {
                    return syl.common.xml_id.clone();
                }
            }
        }
    }
    None
}

#[test]
fn import_addlyrics_basic() {
    let (mei, _ext_store) = parse_and_import("{ c'4 d'4 e'4 f'4 } \\addlyrics { one two three four }");
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
    let (mei, _ext_store) = parse_and_import("{ c'4 d'4 e'4 } \\addlyrics { hel -- lo world }");
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
    let (mei, ext_store) = parse_and_import("{ c'4 d'4 } \\addlyrics { hold __ rest }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 2);
    if let LayerChild::Note(n) = &children[0] {
        assert_eq!(get_verse_text(n, "1").as_deref(), Some("hold"));
        // Extender data now in ext_store keyed by syl's xml:id
        let syl_id = get_verse_syl_id(n, "1").expect("syl should have xml:id");
        assert!(
            ext_store.lyric_extender(&syl_id).is_some(),
            "should have lyric extender in ext_store for syl {syl_id}"
        );
    }
    if let LayerChild::Note(n) = &children[1] {
        assert_eq!(get_verse_text(n, "1").as_deref(), Some("rest"));
    }
}

#[test]
fn import_addlyrics_label_on_staffdef() {
    let (mei, ext_store) = parse_and_import("{ c'4 d'4 } \\addlyrics { do re }");
    let sd = find_score_def(&mei).unwrap();
    // Should have lyrics info in ext_store for staffDef
    let sdef = find_staff_def(sd);
    assert!(sdef.is_some());
    let sdef_id = sdef.unwrap().basic.xml_id.as_deref().unwrap();
    assert!(
        ext_store.lyrics_info(sdef_id).is_some(),
        "should have lyrics info in ext_store for staffDef {sdef_id}"
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
    let (mei, _ext_store) = parse_and_import(src);
    let children = layer_children(&mei);
    assert_eq!(children.len(), 2);
    if let LayerChild::Note(n) = &children[0] {
        assert_eq!(get_verse_text(n, "1").as_deref(), Some("do"));
    }
    if let LayerChild::Note(n) = &children[1] {
        assert_eq!(get_verse_text(n, "1").as_deref(), Some("re"));
    }
}

// --- Phase 36.1: Fixed pitch context import tests ---

#[test]
fn import_fixed_label_stored() {
    let (mei, ext_store) = parse_and_import("\\fixed c' { c4 d e f }");
    let sdef = first_staff_def(&mei).unwrap();
    let sdef_id = sdef.basic.xml_id.as_deref().unwrap();
    let ctx = ext_store.pitch_context(sdef_id).expect("should have pitch context");
    // Verify it's stored as Fixed, not Relative
    assert!(
        matches!(ctx, tusk_model::PitchContext::Fixed { .. }),
        "should be Fixed pitch context: {ctx:?}"
    );
}

#[test]
fn import_fixed_resolves_pitches() {
    // \fixed c' { c d e f } -> absolute: c' d' e' f' (all octave 4)
    let (mei, _ext_store) = parse_and_import("\\fixed c' { c4 d e f }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 4);

    for (i, expected) in ["c", "d", "e", "f"].iter().enumerate() {
        if let LayerChild::Note(n) = &children[i] {
            assert_eq!(n.note_log.pname.as_ref().unwrap().0, *expected);
            assert_eq!(
                n.note_log.oct.as_ref().unwrap().0,
                4,
                "note {expected} should be octave 4"
            );
        } else {
            panic!("expected Note at index {i}");
        }
    }
}

#[test]
fn import_fixed_with_octave_marks() {
    // \fixed c' { c' c, } -> c'' (octave 5) and c (octave 3)
    let (mei, _ext_store) = parse_and_import("\\fixed c' { c'4 c, }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 2);

    if let LayerChild::Note(n) = &children[0] {
        assert_eq!(n.note_log.pname.as_ref().unwrap().0, "c");
        assert_eq!(
            n.note_log.oct.as_ref().unwrap().0,
            5,
            "c' in fixed c' should be octave 5"
        );
    } else {
        panic!("expected Note");
    }
    if let LayerChild::Note(n) = &children[1] {
        assert_eq!(n.note_log.pname.as_ref().unwrap().0, "c");
        assert_eq!(
            n.note_log.oct.as_ref().unwrap().0,
            3,
            "c, in fixed c' should be octave 3"
        );
    } else {
        panic!("expected Note");
    }
}

#[test]
fn import_fixed_with_accidentals() {
    // \fixed c' { cis4 bes } -> cis' (octave 4, sharp) and bes' (octave 4, flat)
    let (mei, _ext_store) = parse_and_import("\\fixed c' { cis4 bes }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 2);

    if let LayerChild::Note(n) = &children[0] {
        assert_eq!(n.note_log.pname.as_ref().unwrap().0, "c");
        assert_eq!(n.note_log.oct.as_ref().unwrap().0, 4);
        assert!(n.note_ges.accid_ges.is_some(), "cis should have accid_ges");
    } else {
        panic!("expected Note");
    }
    if let LayerChild::Note(n) = &children[1] {
        assert_eq!(n.note_log.pname.as_ref().unwrap().0, "b");
        assert_eq!(n.note_log.oct.as_ref().unwrap().0, 4);
        assert!(n.note_ges.accid_ges.is_some(), "bes should have accid_ges");
    } else {
        panic!("expected Note");
    }
}

#[test]
fn import_fixed_no_sequential_dependency() {
    // Unlike \relative, \fixed does NOT update the reference pitch after each note.
    // \fixed c' { c g c g } -> all at octave 4 (c' g' c' g')
    // In \relative c' { c g c g } -> c'=4, g=3 (closest g below c), c=4 (up from g), g=3
    let (mei, _ext_store) = parse_and_import("\\fixed c' { c4 g c g }");
    let children = layer_children(&mei);
    assert_eq!(children.len(), 4);

    let expected_octs = [4, 4, 4, 4];
    for (i, &oct) in expected_octs.iter().enumerate() {
        if let LayerChild::Note(n) = &children[i] {
            assert_eq!(
                n.note_log.oct.as_ref().unwrap().0,
                oct,
                "note at index {i} should be octave {oct}"
            );
        } else {
            panic!("expected Note at index {i}");
        }
    }
}

// Articulation, ornament, tremolo, and technical import tests moved to tests_control.rs
