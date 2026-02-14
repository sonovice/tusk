use super::*;

#[test]
fn ext_data_default_is_empty() {
    let ext = ExtData::default();
    // ExtData is now an empty struct (all fields migrated to per-concept maps)
    let json = serde_json::to_string(&ext).unwrap();
    assert_eq!(json, "{}");
}

#[test]
fn format_origin_roundtrip() {
    let origin = FormatOrigin {
        format: SourceFormat::LilyPond,
        version: Some("2.24.0".into()),
        pitch_language: Some("dutch".into()),
    };
    let json = serde_json::to_string(&origin).unwrap();
    let back: FormatOrigin = serde_json::from_str(&json).unwrap();
    assert_eq!(origin, back);
}

#[test]
fn pitch_context_relative_roundtrip() {
    let ctx = PitchContext::Relative {
        ref_pitch: Some(ExtPitch {
            step: 'c',
            alter: 0.0,
            octave: 1,
        }),
    };
    let json = serde_json::to_string(&ctx).unwrap();
    let back: PitchContext = serde_json::from_str(&json).unwrap();
    assert_eq!(ctx, back);
}

#[test]
fn pitch_context_transpose_roundtrip() {
    let ctx = PitchContext::Transpose {
        from: ExtPitch {
            step: 'c',
            alter: 0.0,
            octave: 0,
        },
        to: ExtPitch {
            step: 'd',
            alter: 0.0,
            octave: 0,
        },
    };
    let json = serde_json::to_string(&ctx).unwrap();
    let back: PitchContext = serde_json::from_str(&json).unwrap();
    assert_eq!(ctx, back);
}

#[test]
fn output_def_roundtrip() {
    let def = OutputDef {
        kind: OutputDefKind::Header,
        assignments: vec![ExtAssignment {
            name: "title".into(),
            value: ExtValue::String("My Score".into()),
        }],
        context_blocks: vec![],
    };
    let json = serde_json::to_string(&def).unwrap();
    let back: OutputDef = serde_json::from_str(&json).unwrap();
    assert_eq!(def, back);
}

#[test]
fn output_def_layout_with_context_roundtrip() {
    let def = OutputDef {
        kind: OutputDefKind::Layout,
        assignments: vec![],
        context_blocks: vec![ExtContextBlock {
            items: vec![
                ExtContextModItem::ContextRef("Score".into()),
                ExtContextModItem::Consists("Span_arpeggio_engraver".into()),
                ExtContextModItem::Override {
                    path: "SpacingSpanner.base-shortest-duration".into(),
                    value: ExtValue::Scheme("#(ly:make-moment 1/16)".into()),
                },
            ],
        }],
    };
    let json = serde_json::to_string(&def).unwrap();
    let back: OutputDef = serde_json::from_str(&json).unwrap();
    assert_eq!(def, back);
}

#[test]
fn grace_info_roundtrip() {
    for grace in [
        GraceInfo::Grace,
        GraceInfo::Acciaccatura,
        GraceInfo::Appoggiatura,
        GraceInfo::AfterGrace {
            fraction: Some((3, 4)),
        },
        GraceInfo::AfterGrace { fraction: None },
    ] {
        let json = serde_json::to_string(&grace).unwrap();
        let back: GraceInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(grace, back);
    }
}

#[test]
fn property_op_roundtrip() {
    let op = PropertyOp {
        op_type: PropertyOpType::Override,
        path: "Staff.TimeSignature.color".into(),
        value: Some(ExtValue::Scheme("#red".into())),
        once: true,
    };
    let json = serde_json::to_string(&op).unwrap();
    let back: PropertyOp = serde_json::from_str(&json).unwrap();
    assert_eq!(op, back);
}

#[test]
fn event_sequence_roundtrip() {
    let seq = EventSequence {
        events: vec![
            PositionedEvent {
                position: 0,
                event: ControlEvent::Clef {
                    name: "treble".into(),
                },
            },
            PositionedEvent {
                position: 0,
                event: ControlEvent::Key {
                    step: 'c',
                    alter: 0.0,
                    mode: "major".into(),
                },
            },
            PositionedEvent {
                position: 0,
                event: ControlEvent::Time {
                    numerators: vec![4],
                    denominator: 4,
                },
            },
            PositionedEvent {
                position: 4,
                event: ControlEvent::BarCheck,
            },
            PositionedEvent {
                position: 8,
                event: ControlEvent::BarLine {
                    bar_type: "|.".into(),
                },
            },
        ],
    };
    let json = serde_json::to_string(&seq).unwrap();
    let back: EventSequence = serde_json::from_str(&json).unwrap();
    assert_eq!(seq, back);
}

#[test]
fn variable_assignments_roundtrip() {
    let vars = VariableAssignments {
        assignments: vec![
            ExtAssignment {
                name: "melody".into(),
                value: ExtValue::Music("{ c d e f }".into()),
            },
            ExtAssignment {
                name: "tempo_val".into(),
                value: ExtValue::Number(120.0),
            },
        ],
    };
    let json = serde_json::to_string(&vars).unwrap();
    let back: VariableAssignments = serde_json::from_str(&json).unwrap();
    assert_eq!(vars, back);
}

#[test]
fn staff_context_roundtrip() {
    let ctx = StaffContext {
        context_type: "PianoStaff".into(),
        name: Some("piano".into()),
        with_block: Some("\\consists \"Span_arpeggio_engraver\"".into()),
        keyword: Some(ContextKeywordExt::New),
    };
    let json = serde_json::to_string(&ctx).unwrap();
    let back: StaffContext = serde_json::from_str(&json).unwrap();
    assert_eq!(ctx, back);
}

#[test]
fn repeat_info_roundtrip() {
    let info = RepeatInfo {
        repeat_type: RepeatTypeExt::Volta,
        count: 2,
        alternative_count: Some(2),
        ending_index: None,
    };
    let json = serde_json::to_string(&info).unwrap();
    let back: RepeatInfo = serde_json::from_str(&json).unwrap();
    assert_eq!(info, back);
}

#[test]
fn lyrics_info_roundtrip() {
    let info = LyricsInfo {
        style: LyricsStyle::LyricsTo,
        voice_id: Some("melody".into()),
        count: Some(3),
    };
    let json = serde_json::to_string(&info).unwrap();
    let back: LyricsInfo = serde_json::from_str(&json).unwrap();
    assert_eq!(info, back);
}

#[test]
fn ext_data_empty_roundtrip() {
    let ext = ExtData::default();
    let json = serde_json::to_string(&ext).unwrap();
    let back: ExtData = serde_json::from_str(&json).unwrap();
    assert_eq!(ext, back);
}

#[test]
fn extension_store_basic_operations() {
    let mut store = ExtensionStore::new();
    assert!(store.is_empty());

    let ext = ExtData::default();

    store.insert("note-1".into(), ext.clone());
    assert_eq!(store.len(), 1);
    assert!(!store.is_empty());

    let retrieved = store.get("note-1").unwrap();
    assert_eq!(retrieved, &ext);

    // entry() for new element
    let _entry = store.entry("note-2".into());
    assert_eq!(store.len(), 2);

    // remove
    let removed = store.remove("note-1");
    assert!(removed.is_some());
    assert_eq!(store.len(), 1);
    assert!(store.get("note-1").is_none());
}

#[test]
fn extension_store_roundtrip() {
    let mut store = ExtensionStore::new();

    store.insert_staff_context(
        "staff-1".into(),
        StaffContext {
            context_type: "Staff".into(),
            name: None,
            with_block: None,
            keyword: Some(ContextKeywordExt::New),
        },
    );

    store.insert_event_sequence(
        "staff-2".into(),
        EventSequence {
            events: vec![PositionedEvent {
                position: 0,
                event: ControlEvent::Clef {
                    name: "bass".into(),
                },
            }],
        },
    );

    let json = serde_json::to_string(&store).unwrap();
    let back: ExtensionStore = serde_json::from_str(&json).unwrap();
    assert_eq!(store, back);
}

#[test]
fn chord_repetition_roundtrip() {
    let cr = ChordRepetition;
    let json = serde_json::to_string(&cr).unwrap();
    let back: ChordRepetition = serde_json::from_str(&json).unwrap();
    assert_eq!(cr, back);
}

#[test]
fn context_change_roundtrip() {
    let cc = ContextChange {
        context_type: "Staff".into(),
        name: "right".into(),
    };
    let json = serde_json::to_string(&cc).unwrap();
    let back: ContextChange = serde_json::from_str(&json).unwrap();
    assert_eq!(cc, back);
}

#[test]
fn function_call_roundtrip() {
    let fc = FunctionCall {
        name: "breathe".into(),
        args: vec![],
        is_partial: false,
    };
    let json = serde_json::to_string(&fc).unwrap();
    let back: FunctionCall = serde_json::from_str(&json).unwrap();
    assert_eq!(fc, back);
}

#[test]
fn toplevel_markup_roundtrip() {
    let m = ToplevelMarkup {
        position: 2,
        kind: ToplevelMarkupKind::Markup("\\bold { Title }".into()),
    };
    let json = serde_json::to_string(&m).unwrap();
    let back: ToplevelMarkup = serde_json::from_str(&json).unwrap();
    assert_eq!(m, back);
}

#[test]
fn tweak_info_roundtrip() {
    let tweak = TweakInfo {
        path: "Beam.positions".into(),
        value: ExtValue::Scheme("#'(2 . 3)".into()),
    };
    let json = serde_json::to_string(&tweak).unwrap();
    let back: TweakInfo = serde_json::from_str(&json).unwrap();
    assert_eq!(tweak, back);
}

#[test]
fn book_structure_roundtrip() {
    let bs = BookStructure {
        book_index: Some(0),
        bookpart_index: Some(1),
        score_index: None,
        book_output_defs: vec![OutputDef {
            kind: OutputDefKind::Paper,
            assignments: vec![ExtAssignment {
                name: "indent".into(),
                value: ExtValue::Number(0.0),
            }],
            context_blocks: vec![],
        }],
        bookpart_output_defs: vec![],
    };
    let json = serde_json::to_string(&bs).unwrap();
    let back: BookStructure = serde_json::from_str(&json).unwrap();
    assert_eq!(bs, back);
}

#[test]
fn ext_data_skips_none_in_json() {
    let ext = ExtData::default();
    let json = serde_json::to_string(&ext).unwrap();
    // Empty struct serializes to empty object
    assert_eq!(json, "{}");
}

#[test]
fn all_source_formats() {
    for fmt in [
        SourceFormat::LilyPond,
        SourceFormat::MusicXML,
        SourceFormat::MEI,
    ] {
        let json = serde_json::to_string(&fmt).unwrap();
        let back: SourceFormat = serde_json::from_str(&json).unwrap();
        assert_eq!(fmt, back);
    }
}

#[test]
fn all_repeat_types() {
    for rt in [
        RepeatTypeExt::Volta,
        RepeatTypeExt::Unfold,
        RepeatTypeExt::Percent,
        RepeatTypeExt::Tremolo,
        RepeatTypeExt::Segno,
    ] {
        let json = serde_json::to_string(&rt).unwrap();
        let back: RepeatTypeExt = serde_json::from_str(&json).unwrap();
        assert_eq!(rt, back);
    }
}

#[test]
fn all_property_op_types() {
    for op in [
        PropertyOpType::Override,
        PropertyOpType::Revert,
        PropertyOpType::Set,
        PropertyOpType::Unset,
        PropertyOpType::Tweak,
    ] {
        let json = serde_json::to_string(&op).unwrap();
        let back: PropertyOpType = serde_json::from_str(&json).unwrap();
        assert_eq!(op, back);
    }
}

#[test]
fn all_lyrics_styles() {
    for style in [
        LyricsStyle::AddLyrics,
        LyricsStyle::LyricsTo,
        LyricsStyle::LyricMode,
    ] {
        let json = serde_json::to_string(&style).unwrap();
        let back: LyricsStyle = serde_json::from_str(&json).unwrap();
        assert_eq!(style, back);
    }
}

#[test]
fn all_output_def_kinds() {
    for kind in [
        OutputDefKind::Header,
        OutputDefKind::Paper,
        OutputDefKind::Layout,
        OutputDefKind::Midi,
    ] {
        let json = serde_json::to_string(&kind).unwrap();
        let back: OutputDefKind = serde_json::from_str(&json).unwrap();
        assert_eq!(kind, back);
    }
}

#[test]
fn pitched_rest_roundtrip() {
    let pr = PitchedRest {
        pitch: "fis'".into(),
    };
    let json = serde_json::to_string(&pr).unwrap();
    let back: PitchedRest = serde_json::from_str(&json).unwrap();
    assert_eq!(pr, back);
}

#[test]
fn multi_measure_rest_info_roundtrip() {
    let info = MultiMeasureRestInfo {
        base: 1,
        dots: 0,
        multipliers: vec![(4, 1)],
    };
    let json = serde_json::to_string(&info).unwrap();
    let back: MultiMeasureRestInfo = serde_json::from_str(&json).unwrap();
    assert_eq!(info, back);

    // With dots and fraction multiplier
    let info2 = MultiMeasureRestInfo {
        base: 2,
        dots: 1,
        multipliers: vec![(3, 2)],
    };
    let json2 = serde_json::to_string(&info2).unwrap();
    let back2: MultiMeasureRestInfo = serde_json::from_str(&json2).unwrap();
    assert_eq!(info2, back2);
}

#[test]
fn drum_event_roundtrip() {
    let de = DrumEvent {
        serialized: "bd4".into(),
    };
    let json = serde_json::to_string(&de).unwrap();
    let back: DrumEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(de, back);
}

#[test]
fn lyric_extender_roundtrip() {
    let le = LyricExtender;
    let json = serde_json::to_string(&le).unwrap();
    let back: LyricExtender = serde_json::from_str(&json).unwrap();
    assert_eq!(le, back);
}

#[test]
fn extension_store_typed_accessors() {
    let mut store = ExtensionStore::new();

    // Test MusicXML harmony accessor
    let harmony = crate::musicxml_ext::HarmonyData::default();
    store.insert_harmony("h1".into(), harmony.clone());
    assert_eq!(store.harmony("h1"), Some(&harmony));
    assert!(store.harmony("nonexistent").is_none());

    // Test MusicXML barline accessor
    let barline = crate::musicxml_ext::BarlineData::default();
    store.insert_barline("b1".into(), barline.clone());
    assert_eq!(store.barline("b1"), Some(&barline));

    // Test LilyPond format_origin accessor
    let origin = FormatOrigin {
        format: SourceFormat::LilyPond,
        version: None,
        pitch_language: None,
    };
    store.insert_format_origin("fo1".into(), origin.clone());
    assert_eq!(store.format_origin("fo1"), Some(&origin));

    // Test wedge_spread accessor (f64 value)
    store.insert_wedge_spread("w1".into(), 15.0);
    assert_eq!(store.wedge_spread("w1"), Some(&15.0));
}
