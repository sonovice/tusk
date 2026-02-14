use super::*;

#[test]
fn harmony_data_roundtrip() {
    let data = HarmonyData {
        chords: vec![HarmonyChordData {
            root_type: "root".into(),
            root_step: Some("C".into()),
            root_alter: None,
            root_text: None,
            numeral_value: None,
            numeral_key: None,
            function: None,
            kind: KindData {
                value: "major-seventh".into(),
                text: Some("maj7".into()),
                use_symbols: None,
                stack_degrees: None,
                parentheses_degrees: None,
                bracket_degrees: None,
                halign: None,
            },
            inversion: None,
            bass: Some(BassData {
                step: "E".into(),
                alter: None,
                text: None,
                separator: None,
                arrangement: None,
            }),
            degrees: vec![DegreeData {
                value: 9,
                alter: 0.0,
                degree_type: "add".into(),
                symbol: None,
                value_text: None,
                plus_minus: None,
            }],
        }],
        frame: None,
        offset: None,
        harmony_type: Some("explicit".into()),
        print_object: None,
        print_frame: None,
        arrangement: None,
        placement: Some("above".into()),
        visual: None,
        id: None,
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: HarmonyData = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn transpose_data_roundtrip() {
    let data = TransposeData {
        number: None,
        diatonic: Some(-1),
        chromatic: -2.0,
        octave_change: Some(-1),
        double: Some(DoubleData { above: Some(true) }),
        id: None,
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: TransposeData = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn sound_data_roundtrip() {
    let data = SoundData {
        tempo: Some(120.0),
        dynamics: Some(80.0),
        dacapo: Some(true),
        fine: Some("yes".into()),
        swing: Some(SwingData {
            content_type: "ratio".into(),
            first: Some(2),
            second: Some(1),
            swing_type: Some("eighth".into()),
            style: None,
        }),
        ..Default::default()
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: SoundData = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn score_header_data_roundtrip() {
    let data = ScoreHeaderData {
        identification: Some(IdentificationData {
            creators: vec![TypedTextData {
                text_type: Some("composer".into()),
                value: "J.S. Bach".into(),
            }],
            rights: vec![TypedTextData {
                text_type: None,
                value: "Public Domain".into(),
            }],
            ..Default::default()
        }),
        work: Some(WorkData {
            work_number: Some("BWV 846".into()),
            work_title: Some("Prelude in C Major".into()),
            opus: None,
        }),
        movement_number: Some("1".into()),
        movement_title: Some("Prelude".into()),
        ..Default::default()
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: ScoreHeaderData = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn print_data_roundtrip() {
    let data = PrintData {
        new_system: Some(true),
        new_page: None,
        blank_page: None,
        page_number: Some("3".into()),
        staff_spacing: Some(10.0),
        ..Default::default()
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: PrintData = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn measure_style_data_roundtrip() {
    let data = MeasureStyleData {
        number: Some(1),
        content: MeasureStyleContentData::MultipleRest {
            value: 4,
            use_symbols: Some(true),
        },
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: MeasureStyleData = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);

    // Beat repeat variant
    let data2 = MeasureStyleData {
        number: None,
        content: MeasureStyleContentData::BeatRepeat {
            repeat_type: "start".into(),
            slashes: Some(2),
            use_dots: None,
        },
    };
    let json2 = serde_json::to_string(&data2).unwrap();
    let back2: MeasureStyleData = serde_json::from_str(&json2).unwrap();
    assert_eq!(data2, back2);
}

#[test]
fn barline_data_roundtrip() {
    let data = BarlineData {
        location: Some("right".into()),
        bar_style: Some("light-heavy".into()),
        repeat: Some(RepeatData {
            direction: "backward".into(),
            times: Some(2),
            after_jump: None,
            winged: None,
        }),
        ending: Some(EndingData {
            number: "1,2".into(),
            ending_type: "start".into(),
            text: Some("1. 2.".into()),
            visual: None,
        }),
        ..Default::default()
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: BarlineData = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn listening_data_roundtrip() {
    let data = ListeningData::Grouping(musicxml::listening::Grouping {
        grouping_type: "start".into(),
        member_of: Some("phrase".into()),
        ..Default::default()
    });
    let json = serde_json::to_string(&data).unwrap();
    let back: ListeningData = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn note_visual_data_roundtrip() {
    let data = NoteVisualData {
        default_x: Some(10.5),
        default_y: Some(-5.0),
        color: Some("#FF0000".into()),
        print_object: Some(false),
        dynamics: Some(90.0),
        pizzicato: Some(true),
        ..Default::default()
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: NoteVisualData = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn direction_visual_data_roundtrip() {
    let data = DirectionVisualData {
        words: vec![WordsVisualData {
            value: "cresc.".into(),
            visual: VisualAttrs {
                font_family: Some("Times".into()),
                font_size: Some(12.0),
                font_style: Some("italic".into()),
                ..Default::default()
            },
            enclosure: None,
            halign: None,
            valign: None,
            justify: None,
            id: None,
        }],
        wedge_color: Some("#000000".into()),
        wedge_niente: None,
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: DirectionVisualData = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn instrument_data_roundtrip() {
    let data = InstrumentData {
        score_instrument: ScoreInstrumentData {
            id: "P1-I1".into(),
            name: "Flute".into(),
            abbreviation: Some("Fl.".into()),
            sound: Some("wind.flutes.flute".into()),
            solo: Some(true),
            ensemble: None,
            virtual_instrument: None,
        },
        midi_assignments: vec![MidiAssignmentData {
            device: None,
            instrument: Some(MidiInstrumentDataInner {
                id: "P1-I1".into(),
                channel: Some(1),
                name: None,
                bank: None,
                program: Some(74),
                unpitched: None,
                volume: Some(80.0),
                pan: Some(0.0),
                elevation: None,
            }),
        }],
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: InstrumentData = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn part_details_data_roundtrip() {
    use crate::musicxml::elements::NameDisplay;
    let data = PartDetailsData {
        part_name_display: Some(NameDisplay {
            print_object: Some(crate::musicxml::data::YesNo::Yes),
            ..Default::default()
        }),
        groups: vec!["group1".into()],
        ..Default::default()
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: PartDetailsData = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn group_details_data_roundtrip() {
    use crate::musicxml::elements::NameDisplay;
    let data = GroupDetailsData {
        group_name_display: Some(NameDisplay::default()),
        group_time: Some(true),
        ..Default::default()
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: GroupDetailsData = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn note_extras_roundtrip() {
    let data = NoteExtras {
        notehead: Some(musicxml::note::Notehead {
            value: musicxml::note::NoteheadValue::Diamond,
            filled: Some(musicxml::data::YesNo::Yes),
            ..Default::default()
        }),
        instruments: vec!["P1-I1".into(), "P1-I2".into()],
        play: Some(PlayData {
            id: Some("play-1".into()),
            entries: vec![musicxml::direction::PlayEntry::Mute("straight".into())],
        }),
        ..Default::default()
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: NoteExtras = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn stem_extras_roundtrip() {
    for stem in [StemExtras::Double, StemExtras::None] {
        let json = serde_json::to_string(&stem).unwrap();
        let back: StemExtras = serde_json::from_str(&json).unwrap();
        assert_eq!(stem, back);
    }
}

#[test]
fn key_extras_roundtrip() {
    let data = KeyExtras {
        key: Some(musicxml::attributes::Key {
            number: None,
            print_object: None,
            id: None,
            content: musicxml::attributes::KeyContent::Traditional(
                musicxml::attributes::TraditionalKey {
                    cancel: None,
                    fifths: 2,
                    mode: Some(musicxml::attributes::Mode::Major),
                },
            ),
            key_octaves: Vec::new(),
        }),
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: KeyExtras = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn time_extras_roundtrip() {
    let data = TimeExtras {
        time: Some(musicxml::attributes::Time {
            number: None,
            symbol: Some(musicxml::attributes::TimeSymbol::Common),
            separator: None,
            print_object: None,
            id: None,
            content: musicxml::attributes::TimeContent::Standard(
                musicxml::attributes::StandardTime {
                    signatures: vec![musicxml::attributes::TimeSignature {
                        beats: "4".into(),
                        beat_type: "4".into(),
                    }],
                    interchangeable: None,
                },
            ),
        }),
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: TimeExtras = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn for_part_data_roundtrip() {
    let data = ForPartData {
        entries: vec![musicxml::attributes::ForPart {
            number: None,
            id: None,
            part_clef: None,
            part_transpose: musicxml::attributes::PartTranspose {
                diatonic: Some(-1),
                chromatic: -2.0,
                ..Default::default()
            },
        }],
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: ForPartData = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn staff_details_extras_roundtrip() {
    let data = StaffDetailsExtras {
        details: musicxml::attributes::StaffDetails {
            staff_lines: Some(6),
            ..Default::default()
        },
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: StaffDetailsExtras = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn part_symbol_extras_roundtrip() {
    let data = PartSymbolExtras {
        value: "brace".into(),
        top_staff: Some(1),
        bottom_staff: Some(2),
        default_x: Some(-15.0),
        color: None,
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: PartSymbolExtras = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn lyric_extras_roundtrip() {
    let data = LyricExtras {
        lyric: Some(musicxml::lyric::Lyric {
            number: Some("1".into()),
            name: None,
            justify: Some(musicxml::data::LeftCenterRight::Left),
            default_x: None,
            default_y: None,
            relative_x: None,
            relative_y: None,
            placement: None,
            color: None,
            print_object: None,
            time_only: Some("1".into()),
            id: None,
            content: musicxml::lyric::LyricContent::Humming,
            end_line: false,
            end_paragraph: false,
            footnote: None,
            level: None,
        }),
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: LyricExtras = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn visual_attrs_roundtrip() {
    let data = VisualAttrs {
        font_family: Some("Bravura".into()),
        font_size: Some(24.0),
        font_style: Some("italic".into()),
        font_weight: Some("bold".into()),
        color: Some("#FF0000".into()),
        default_x: Some(10.0),
        default_y: Some(-5.0),
        relative_x: Some(2.0),
        relative_y: Some(-1.0),
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: VisualAttrs = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn frame_data_roundtrip() {
    let data = FrameData {
        strings: 6,
        frets: 4,
        first_fret: Some(FirstFretData {
            value: 3,
            text: Some("III".into()),
            location: Some("right".into()),
        }),
        notes: vec![FrameNoteData {
            string: 1,
            fret: 0,
            fingering: None,
            barre: None,
        }],
        visual: None,
        unplayed: Some("x".into()),
        id: None,
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: FrameData = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn offset_data_roundtrip() {
    let data = OffsetData {
        value: 24.0,
        sound: Some(true),
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: OffsetData = serde_json::from_str(&json).unwrap();
    assert_eq!(data, back);
}

#[test]
fn direction_content_data_roundtrip() {
    // Simple marker type
    let damp = DirectionContentData::Damp(musicxml::direction::Damp::default());
    let json = serde_json::to_string(&damp).unwrap();
    let back: DirectionContentData = serde_json::from_str(&json).unwrap();
    assert_eq!(damp, back);

    // Type with structured data
    let mut pedal_val = musicxml::direction::Pedal::new(musicxml::direction::PedalType::Start);
    pedal_val.line = Some(musicxml::data::YesNo::Yes);
    let pedal = DirectionContentData::Pedal(pedal_val);
    let json = serde_json::to_string(&pedal).unwrap();
    let back: DirectionContentData = serde_json::from_str(&json).unwrap();
    assert_eq!(pedal, back);

    // Rehearsal with text
    let rehearsal = DirectionContentData::Rehearsal(vec![musicxml::direction::Rehearsal::new("A")]);
    let json = serde_json::to_string(&rehearsal).unwrap();
    let back: DirectionContentData = serde_json::from_str(&json).unwrap();
    assert_eq!(rehearsal, back);
}
