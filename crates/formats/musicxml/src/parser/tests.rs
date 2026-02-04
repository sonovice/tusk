//! Tests for MusicXML parser.

use super::*;

#[test]
fn test_parse_minimal_score_partwise() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE score-partwise PUBLIC "-//Recordare//DTD MusicXML 4.0 Partwise//EN" "http://www.musicxml.org/dtds/partwise.dtd">
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    assert_eq!(score.version, Some("4.0".to_string()));
    assert_eq!(score.part_list.items.len(), 1);
    assert_eq!(score.parts.len(), 1);

    // Check part-list
    match &score.part_list.items[0] {
        PartListItem::ScorePart(sp) => {
            assert_eq!(sp.id, "P1");
            assert_eq!(sp.part_name.value, "Piano");
        }
        _ => panic!("Expected ScorePart"),
    }

    // Check part
    assert_eq!(score.parts[0].id, "P1");
    assert_eq!(score.parts[0].measures.len(), 1);
    assert_eq!(score.parts[0].measures[0].number, "1");
}

#[test]
fn test_parse_work_and_identification() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <work>
    <work-number>BWV 846</work-number>
    <work-title>Prelude in C Major</work-title>
  </work>
  <movement-number>1</movement-number>
  <movement-title>Prelude</movement-title>
  <identification>
    <creator type="composer">Johann Sebastian Bach</creator>
    <creator type="arranger">Test Arranger</creator>
    <rights>Public Domain</rights>
    <encoding>
      <software>Test Software</software>
      <encoding-date>2024-01-15</encoding-date>
    </encoding>
  </identification>
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    // Check work
    let work = score.work.as_ref().expect("work missing");
    assert_eq!(work.work_number, Some("BWV 846".to_string()));
    assert_eq!(work.work_title, Some("Prelude in C Major".to_string()));

    // Check movement info
    assert_eq!(score.movement_number, Some("1".to_string()));
    assert_eq!(score.movement_title, Some("Prelude".to_string()));

    // Check identification
    let ident = score
        .identification
        .as_ref()
        .expect("identification missing");
    assert_eq!(ident.creators.len(), 2);
    assert_eq!(ident.creators[0].value, "Johann Sebastian Bach");
    assert_eq!(ident.creators[0].text_type, Some("composer".to_string()));
    assert_eq!(ident.creators[1].text_type, Some("arranger".to_string()));
    assert_eq!(ident.rights.len(), 1);
    assert_eq!(ident.rights[0].value, "Public Domain");

    // Check encoding
    let encoding = ident.encoding.as_ref().expect("encoding missing");
    assert_eq!(encoding.software, vec!["Test Software".to_string()]);
    assert_eq!(encoding.encoding_date, Some("2024-01-15".to_string()));
}

#[test]
fn test_parse_part_groups() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <part-group type="start" number="1">
      <group-name>Strings</group-name>
      <group-symbol>bracket</group-symbol>
      <group-barline>yes</group-barline>
    </part-group>
    <score-part id="P1">
      <part-name>Violin I</part-name>
    </score-part>
    <score-part id="P2">
      <part-name>Violin II</part-name>
    </score-part>
    <part-group type="stop" number="1"/>
  </part-list>
  <part id="P1">
    <measure number="1">
    </measure>
  </part>
  <part id="P2">
    <measure number="1">
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    assert_eq!(score.part_list.items.len(), 4); // 2 part-groups + 2 score-parts

    // Check first part-group (start)
    match &score.part_list.items[0] {
        PartListItem::PartGroup(pg) => {
            assert_eq!(pg.group_type, StartStop::Start);
            assert_eq!(pg.number, Some("1".to_string()));
            assert_eq!(pg.group_name, Some("Strings".to_string()));
            assert!(matches!(
                &pg.group_symbol,
                Some(GroupSymbolValue {
                    value: GroupSymbol::Bracket,
                    ..
                })
            ));
        }
        _ => panic!("Expected PartGroup"),
    }

    // Check last part-group (stop)
    match &score.part_list.items[3] {
        PartListItem::PartGroup(pg) => {
            assert_eq!(pg.group_type, StartStop::Stop);
            assert_eq!(pg.number, Some("1".to_string()));
        }
        _ => panic!("Expected PartGroup"),
    }
}

#[test]
fn test_parse_note_with_pitch() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <note>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <duration>4</duration>
        <type>quarter</type>
      </note>
      <note>
        <pitch>
          <step>E</step>
          <alter>-1</alter>
          <octave>4</octave>
        </pitch>
        <duration>4</duration>
        <type>quarter</type>
      </note>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    let measure = &score.parts[0].measures[0];
    assert_eq!(measure.content.len(), 2);

    // First note: C4
    match &measure.content[0] {
        MeasureContent::Note(note) => {
            match &note.content {
                FullNoteContent::Pitch(pitch) => {
                    assert_eq!(pitch.step, Step::C);
                    assert_eq!(pitch.octave, 4);
                    assert!(pitch.alter.is_none());
                }
                _ => panic!("Expected Pitch"),
            }
            assert_eq!(note.duration, Some(4.0));
            assert!(matches!(
                note.note_type.as_ref().map(|nt| &nt.value),
                Some(NoteTypeValue::Quarter)
            ));
        }
        _ => panic!("Expected Note"),
    }

    // Second note: Eb4
    match &measure.content[1] {
        MeasureContent::Note(note) => match &note.content {
            FullNoteContent::Pitch(pitch) => {
                assert_eq!(pitch.step, Step::E);
                assert_eq!(pitch.alter, Some(-1.0));
                assert_eq!(pitch.octave, 4);
            }
            _ => panic!("Expected Pitch"),
        },
        _ => panic!("Expected Note"),
    }
}

#[test]
fn test_parse_rest() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <note>
        <rest/>
        <duration>4</duration>
        <type>quarter</type>
      </note>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    let measure = &score.parts[0].measures[0];
    match &measure.content[0] {
        MeasureContent::Note(note) => {
            assert!(matches!(note.content, FullNoteContent::Rest(_)));
            assert_eq!(note.duration, Some(4.0));
            assert!(matches!(
                note.note_type.as_ref().map(|nt| &nt.value),
                Some(NoteTypeValue::Quarter)
            ));
        }
        _ => panic!("Expected Note (rest)"),
    }
}

#[test]
fn test_parse_chord() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <note>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <duration>4</duration>
        <type>quarter</type>
      </note>
      <note>
        <chord/>
        <pitch>
          <step>E</step>
          <octave>4</octave>
        </pitch>
        <duration>4</duration>
        <type>quarter</type>
      </note>
      <note>
        <chord/>
        <pitch>
          <step>G</step>
          <octave>4</octave>
        </pitch>
        <duration>4</duration>
        <type>quarter</type>
      </note>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    let measure = &score.parts[0].measures[0];
    assert_eq!(measure.content.len(), 3);

    // First note: no chord marker
    match &measure.content[0] {
        MeasureContent::Note(note) => {
            assert!(note.chord.is_none());
        }
        _ => panic!("Expected Note"),
    }

    // Second and third notes: have chord marker
    match &measure.content[1] {
        MeasureContent::Note(note) => {
            assert!(note.chord.is_some());
        }
        _ => panic!("Expected Note"),
    }
    match &measure.content[2] {
        MeasureContent::Note(note) => {
            assert!(note.chord.is_some());
        }
        _ => panic!("Expected Note"),
    }
}

#[test]
fn test_parse_backup_forward() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <note>
        <pitch>
          <step>C</step>
          <octave>5</octave>
        </pitch>
        <duration>16</duration>
        <voice>1</voice>
        <type>whole</type>
      </note>
      <backup>
        <duration>16</duration>
      </backup>
      <note>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <duration>8</duration>
        <voice>2</voice>
        <type>half</type>
      </note>
      <forward>
        <duration>8</duration>
        <voice>2</voice>
      </forward>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    let measure = &score.parts[0].measures[0];
    assert_eq!(measure.content.len(), 4);

    // Check backup
    match &measure.content[1] {
        MeasureContent::Backup(backup) => {
            assert_eq!(backup.duration, 16.0);
        }
        _ => panic!("Expected Backup"),
    }

    // Check forward
    match &measure.content[3] {
        MeasureContent::Forward(forward) => {
            assert_eq!(forward.duration, 8.0);
            assert_eq!(forward.voice, Some("2".to_string()));
        }
        _ => panic!("Expected Forward"),
    }
}

#[test]
fn test_parse_attributes() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <attributes>
        <divisions>4</divisions>
        <key>
          <fifths>-3</fifths>
          <mode>minor</mode>
        </key>
        <time>
          <beats>3</beats>
          <beat-type>4</beat-type>
        </time>
        <staves>2</staves>
        <clef number="1">
          <sign>G</sign>
          <line>2</line>
        </clef>
        <clef number="2">
          <sign>F</sign>
          <line>4</line>
        </clef>
      </attributes>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    let measure = &score.parts[0].measures[0];
    match &measure.content[0] {
        MeasureContent::Attributes(attrs) => {
            assert_eq!(attrs.divisions, Some(4.0));
            assert_eq!(attrs.staves, Some(2));

            // Key
            assert_eq!(attrs.keys.len(), 1);
            let key = &attrs.keys[0];
            match &key.content {
                KeyContent::Traditional(tk) => {
                    assert_eq!(tk.fifths, -3);
                    assert_eq!(tk.mode, Some(Mode::Minor));
                }
                _ => panic!("Expected Traditional key"),
            }

            // Time
            assert_eq!(attrs.times.len(), 1);
            let time = &attrs.times[0];
            match &time.content {
                TimeContent::Standard(std_time) => {
                    assert_eq!(std_time.signatures.len(), 1);
                    assert_eq!(std_time.signatures[0].beats, "3");
                    assert_eq!(std_time.signatures[0].beat_type, "4");
                }
                _ => panic!("Expected Standard time"),
            }

            // Clefs
            assert_eq!(attrs.clefs.len(), 2);
            assert_eq!(attrs.clefs[0].sign, ClefSign::G);
            assert_eq!(attrs.clefs[0].line, Some(2));
            assert_eq!(attrs.clefs[0].number, Some(1));
            assert_eq!(attrs.clefs[1].sign, ClefSign::F);
            assert_eq!(attrs.clefs[1].line, Some(4));
            assert_eq!(attrs.clefs[1].number, Some(2));
        }
        _ => panic!("Expected Attributes"),
    }
}

#[test]
fn test_parse_direction_dynamics() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <direction placement="below">
        <direction-type>
          <dynamics>
            <ff/>
          </dynamics>
        </direction-type>
        <sound dynamics="112"/>
      </direction>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    let measure = &score.parts[0].measures[0];
    match &measure.content[0] {
        MeasureContent::Direction(dir) => {
            assert_eq!(dir.placement, Some(AboveBelow::Below));
            assert_eq!(dir.direction_types.len(), 1);

            match &dir.direction_types[0].content {
                DirectionTypeContent::Dynamics(dynamics) => {
                    // dynamics should have ff in the values list
                    assert!(!dynamics.values.is_empty());
                }
                _ => panic!("Expected Dynamics"),
            }

            // Check sound
            let sound = dir.sound.as_ref().expect("sound missing");
            assert_eq!(sound.dynamics, Some(112.0));
        }
        _ => panic!("Expected Direction"),
    }
}

#[test]
fn test_parse_direction_wedge() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <direction>
        <direction-type>
          <wedge type="crescendo"/>
        </direction-type>
      </direction>
      <note>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <duration>4</duration>
        <type>quarter</type>
      </note>
      <direction>
        <direction-type>
          <wedge type="stop"/>
        </direction-type>
      </direction>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    let measure = &score.parts[0].measures[0];

    // First direction - crescendo start
    match &measure.content[0] {
        MeasureContent::Direction(dir) => match &dir.direction_types[0].content {
            DirectionTypeContent::Wedge(wedge) => {
                assert_eq!(wedge.wedge_type, WedgeType::Crescendo);
            }
            _ => panic!("Expected Wedge"),
        },
        _ => panic!("Expected Direction"),
    }

    // Third element - wedge stop
    match &measure.content[2] {
        MeasureContent::Direction(dir) => match &dir.direction_types[0].content {
            DirectionTypeContent::Wedge(wedge) => {
                assert_eq!(wedge.wedge_type, WedgeType::Stop);
            }
            _ => panic!("Expected Wedge"),
        },
        _ => panic!("Expected Direction"),
    }
}

#[test]
fn test_parse_beams() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <note>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <duration>1</duration>
        <type>eighth</type>
        <beam number="1">begin</beam>
      </note>
      <note>
        <pitch>
          <step>D</step>
          <octave>4</octave>
        </pitch>
        <duration>1</duration>
        <type>eighth</type>
        <beam number="1">continue</beam>
      </note>
      <note>
        <pitch>
          <step>E</step>
          <octave>4</octave>
        </pitch>
        <duration>1</duration>
        <type>eighth</type>
        <beam number="1">end</beam>
      </note>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    let measure = &score.parts[0].measures[0];

    // First note - beam begin
    match &measure.content[0] {
        MeasureContent::Note(note) => {
            assert_eq!(note.beams.len(), 1);
            assert_eq!(note.beams[0].number, Some(1));
            assert_eq!(note.beams[0].value, BeamValue::Begin);
        }
        _ => panic!("Expected Note"),
    }

    // Second note - beam continue
    match &measure.content[1] {
        MeasureContent::Note(note) => {
            assert_eq!(note.beams[0].value, BeamValue::Continue);
        }
        _ => panic!("Expected Note"),
    }

    // Third note - beam end
    match &measure.content[2] {
        MeasureContent::Note(note) => {
            assert_eq!(note.beams[0].value, BeamValue::End);
        }
        _ => panic!("Expected Note"),
    }
}

#[test]
fn test_parse_ties() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <note>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <duration>4</duration>
        <tie type="start"/>
        <type>quarter</type>
        <notations>
          <tied type="start"/>
        </notations>
      </note>
      <note>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <duration>4</duration>
        <tie type="stop"/>
        <type>quarter</type>
        <notations>
          <tied type="stop"/>
        </notations>
      </note>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    let measure = &score.parts[0].measures[0];

    // First note - tie start
    match &measure.content[0] {
        MeasureContent::Note(note) => {
            assert_eq!(note.ties.len(), 1);
            assert_eq!(note.ties[0].tie_type, StartStop::Start);
        }
        _ => panic!("Expected Note"),
    }

    // Second note - tie stop
    match &measure.content[1] {
        MeasureContent::Note(note) => {
            assert_eq!(note.ties.len(), 1);
            assert_eq!(note.ties[0].tie_type, StartStop::Stop);
        }
        _ => panic!("Expected Note"),
    }
}

#[test]
fn test_parse_score_instrument() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
      <score-instrument id="P1-I1">
        <instrument-name>Acoustic Grand Piano</instrument-name>
        <instrument-sound>keyboard.piano.grand</instrument-sound>
      </score-instrument>
      <midi-instrument id="P1-I1">
        <midi-channel>1</midi-channel>
        <midi-program>1</midi-program>
        <volume>80</volume>
        <pan>0</pan>
      </midi-instrument>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    match &score.part_list.items[0] {
        PartListItem::ScorePart(sp) => {
            assert_eq!(sp.score_instruments.len(), 1);
            assert_eq!(sp.score_instruments[0].id, "P1-I1");
            assert_eq!(
                sp.score_instruments[0].instrument_name,
                "Acoustic Grand Piano"
            );
            assert_eq!(
                sp.score_instruments[0].instrument_sound,
                Some("keyboard.piano.grand".to_string())
            );

            // Check MIDI
            assert_eq!(sp.midi_assignments.len(), 1);
            match &sp.midi_assignments[0] {
                MidiAssignment::MidiInstrument(mi) => {
                    assert_eq!(mi.id, "P1-I1");
                    assert_eq!(mi.midi_channel, Some(1));
                    assert_eq!(mi.midi_program, Some(1));
                    assert_eq!(mi.volume, Some(80.0));
                    assert_eq!(mi.pan, Some(0.0));
                }
                _ => panic!("Expected MidiInstrument"),
            }
        }
        _ => panic!("Expected ScorePart"),
    }
}

#[test]
fn test_parse_defaults() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <defaults>
    <scaling>
      <millimeters>7.2319</millimeters>
      <tenths>40</tenths>
    </scaling>
    <page-layout>
      <page-height>1545</page-height>
      <page-width>1194</page-width>
      <page-margins type="both">
        <left-margin>70</left-margin>
        <right-margin>70</right-margin>
        <top-margin>88</top-margin>
        <bottom-margin>88</bottom-margin>
      </page-margins>
    </page-layout>
  </defaults>
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    let defaults = score.defaults.as_ref().expect("defaults missing");

    // Scaling
    let scaling = defaults.scaling.as_ref().expect("scaling missing");
    assert!((scaling.millimeters - 7.2319).abs() < 0.001);
    assert!((scaling.tenths - 40.0).abs() < 0.001);

    // Page layout
    let page = defaults.page_layout.as_ref().expect("page-layout missing");
    assert_eq!(page.page_height, Some(1545.0));
    assert_eq!(page.page_width, Some(1194.0));
    assert_eq!(page.page_margins.len(), 1);

    let margins = &page.page_margins[0];
    assert_eq!(margins.left_margin, 70.0);
    assert_eq!(margins.right_margin, 70.0);
    assert_eq!(margins.top_margin, 88.0);
    assert_eq!(margins.bottom_margin, 88.0);
}

#[test]
fn test_parse_missing_score_partwise_returns_error() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<not-a-score>
</not-a-score>"#;

    let result = parse_score_partwise(xml);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ParseError::MissingElement(_)));
}

#[test]
fn test_parse_empty_document_returns_error() {
    let xml = "";
    let result = parse_score_partwise(xml);
    assert!(result.is_err());
}

// ============================================================================
// Score-Timewise Tests
// ============================================================================

#[test]
fn test_parse_minimal_score_timewise() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE score-timewise PUBLIC "-//Recordare//DTD MusicXML 4.0 Timewise//EN" "http://www.musicxml.org/dtds/timewise.dtd">
<score-timewise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <measure number="1">
    <part id="P1">
    </part>
  </measure>
</score-timewise>"#;

    let score = parse_score_timewise(xml).expect("parse failed");

    assert_eq!(score.version, Some("4.0".to_string()));
    assert_eq!(score.part_list.items.len(), 1);
    assert_eq!(score.parts.len(), 1);

    // Check part-list
    match &score.part_list.items[0] {
        PartListItem::ScorePart(sp) => {
            assert_eq!(sp.id, "P1");
            assert_eq!(sp.part_name.value, "Piano");
        }
        _ => panic!("Expected ScorePart"),
    }

    // Check converted part structure
    assert_eq!(score.parts[0].id, "P1");
    assert_eq!(score.parts[0].measures.len(), 1);
    assert_eq!(score.parts[0].measures[0].number, "1");
}

#[test]
fn test_parse_score_timewise_multiple_parts() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-timewise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Violin</part-name>
    </score-part>
    <score-part id="P2">
      <part-name>Cello</part-name>
    </score-part>
  </part-list>
  <measure number="1">
    <part id="P1">
      <note>
        <pitch>
          <step>C</step>
          <octave>5</octave>
        </pitch>
        <duration>4</duration>
        <type>quarter</type>
      </note>
    </part>
    <part id="P2">
      <note>
        <pitch>
          <step>G</step>
          <octave>3</octave>
        </pitch>
        <duration>4</duration>
        <type>quarter</type>
      </note>
    </part>
  </measure>
</score-timewise>"#;

    let score = parse_score_timewise(xml).expect("parse failed");

    assert_eq!(score.parts.len(), 2);

    // Check Violin part
    assert_eq!(score.parts[0].id, "P1");
    assert_eq!(score.parts[0].measures.len(), 1);
    assert_eq!(score.parts[0].measures[0].number, "1");
    assert_eq!(score.parts[0].measures[0].content.len(), 1);

    match &score.parts[0].measures[0].content[0] {
        MeasureContent::Note(note) => match &note.content {
            FullNoteContent::Pitch(pitch) => {
                assert_eq!(pitch.step, Step::C);
                assert_eq!(pitch.octave, 5);
            }
            _ => panic!("Expected Pitch"),
        },
        _ => panic!("Expected Note"),
    }

    // Check Cello part
    assert_eq!(score.parts[1].id, "P2");
    assert_eq!(score.parts[1].measures.len(), 1);

    match &score.parts[1].measures[0].content[0] {
        MeasureContent::Note(note) => match &note.content {
            FullNoteContent::Pitch(pitch) => {
                assert_eq!(pitch.step, Step::G);
                assert_eq!(pitch.octave, 3);
            }
            _ => panic!("Expected Pitch"),
        },
        _ => panic!("Expected Note"),
    }
}

#[test]
fn test_parse_score_timewise_multiple_measures() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-timewise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <measure number="1">
    <part id="P1">
      <note>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <duration>4</duration>
        <type>quarter</type>
      </note>
    </part>
  </measure>
  <measure number="2">
    <part id="P1">
      <note>
        <pitch>
          <step>D</step>
          <octave>4</octave>
        </pitch>
        <duration>4</duration>
        <type>quarter</type>
      </note>
    </part>
  </measure>
  <measure number="3">
    <part id="P1">
      <note>
        <pitch>
          <step>E</step>
          <octave>4</octave>
        </pitch>
        <duration>4</duration>
        <type>quarter</type>
      </note>
    </part>
  </measure>
</score-timewise>"#;

    let score = parse_score_timewise(xml).expect("parse failed");

    assert_eq!(score.parts.len(), 1);
    assert_eq!(score.parts[0].measures.len(), 3);

    // Measure 1
    assert_eq!(score.parts[0].measures[0].number, "1");
    match &score.parts[0].measures[0].content[0] {
        MeasureContent::Note(note) => match &note.content {
            FullNoteContent::Pitch(pitch) => assert_eq!(pitch.step, Step::C),
            _ => panic!("Expected Pitch"),
        },
        _ => panic!("Expected Note"),
    }

    // Measure 2
    assert_eq!(score.parts[0].measures[1].number, "2");
    match &score.parts[0].measures[1].content[0] {
        MeasureContent::Note(note) => match &note.content {
            FullNoteContent::Pitch(pitch) => assert_eq!(pitch.step, Step::D),
            _ => panic!("Expected Pitch"),
        },
        _ => panic!("Expected Note"),
    }

    // Measure 3
    assert_eq!(score.parts[0].measures[2].number, "3");
    match &score.parts[0].measures[2].content[0] {
        MeasureContent::Note(note) => match &note.content {
            FullNoteContent::Pitch(pitch) => assert_eq!(pitch.step, Step::E),
            _ => panic!("Expected Pitch"),
        },
        _ => panic!("Expected Note"),
    }
}

#[test]
fn test_parse_score_timewise_with_header() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-timewise version="4.0">
  <work>
    <work-title>Test Timewise Score</work-title>
  </work>
  <identification>
    <creator type="composer">Test Composer</creator>
  </identification>
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <measure number="1">
    <part id="P1">
    </part>
  </measure>
</score-timewise>"#;

    let score = parse_score_timewise(xml).expect("parse failed");

    // Verify header is preserved
    let work = score.work.as_ref().expect("work missing");
    assert_eq!(work.work_title, Some("Test Timewise Score".to_string()));

    let ident = score
        .identification
        .as_ref()
        .expect("identification missing");
    assert_eq!(ident.creators.len(), 1);
    assert_eq!(ident.creators[0].value, "Test Composer");
}

#[test]
fn test_parse_score_timewise_preserves_measure_attributes() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-timewise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <measure number="1" implicit="yes" width="200.5">
    <part id="P1">
    </part>
  </measure>
</score-timewise>"#;

    let score = parse_score_timewise(xml).expect("parse failed");

    let measure = &score.parts[0].measures[0];
    assert_eq!(measure.number, "1");
    assert_eq!(measure.implicit, Some(YesNo::Yes));
    assert_eq!(measure.width, Some(200.5));
}

#[test]
fn test_parse_score_timewise_with_complex_content() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-timewise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <measure number="1">
    <part id="P1">
      <attributes>
        <divisions>1</divisions>
        <key>
          <fifths>0</fifths>
        </key>
        <time>
          <beats>4</beats>
          <beat-type>4</beat-type>
        </time>
        <clef>
          <sign>G</sign>
          <line>2</line>
        </clef>
      </attributes>
      <direction placement="above">
        <direction-type>
          <dynamics>
            <f/>
          </dynamics>
        </direction-type>
      </direction>
      <note>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <duration>1</duration>
        <type>quarter</type>
      </note>
    </part>
  </measure>
</score-timewise>"#;

    let score = parse_score_timewise(xml).expect("parse failed");

    let measure = &score.parts[0].measures[0];
    assert_eq!(measure.content.len(), 3); // attributes, direction, note

    // Verify attributes
    match &measure.content[0] {
        MeasureContent::Attributes(attrs) => {
            assert_eq!(attrs.divisions, Some(1.0));
        }
        _ => panic!("Expected Attributes"),
    }

    // Verify direction
    match &measure.content[1] {
        MeasureContent::Direction(dir) => {
            assert_eq!(dir.placement, Some(AboveBelow::Above));
        }
        _ => panic!("Expected Direction"),
    }

    // Verify note
    match &measure.content[2] {
        MeasureContent::Note(note) => match &note.content {
            FullNoteContent::Pitch(pitch) => assert_eq!(pitch.step, Step::C),
            _ => panic!("Expected Pitch"),
        },
        _ => panic!("Expected Note"),
    }
}

#[test]
fn test_parse_missing_score_timewise_returns_error() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<not-a-score>
</not-a-score>"#;

    let result = parse_score_timewise(xml);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ParseError::MissingElement(_)));
}

// ============================================================================
// Duration and Divisions Tests
// ============================================================================

#[test]
fn test_parse_divisions_and_calculate_durations() {
    use crate::model::duration::DurationContext;
    use crate::model::note::NoteTypeValue;

    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <attributes>
        <divisions>4</divisions>
        <time>
          <beats>4</beats>
          <beat-type>4</beat-type>
        </time>
      </attributes>
      <note>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <duration>4</duration>
        <type>quarter</type>
      </note>
      <note>
        <pitch>
          <step>D</step>
          <octave>4</octave>
        </pitch>
        <duration>2</duration>
        <type>eighth</type>
      </note>
      <note>
        <pitch>
          <step>E</step>
          <octave>4</octave>
        </pitch>
        <duration>2</duration>
        <type>eighth</type>
      </note>
      <note>
        <pitch>
          <step>F</step>
          <octave>4</octave>
        </pitch>
        <duration>8</duration>
        <type>half</type>
      </note>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    let measure = &score.parts[0].measures[0];

    // Extract divisions from attributes
    let divisions = match &measure.content[0] {
        MeasureContent::Attributes(attrs) => attrs.divisions.expect("divisions missing"),
        _ => panic!("Expected Attributes"),
    };

    assert_eq!(divisions, 4.0);

    // Create duration context
    let ctx = DurationContext::with_divisions(divisions);

    // Verify calculated durations match expected values
    assert_eq!(ctx.duration_for_type(NoteTypeValue::Quarter, 0), 4.0);
    assert_eq!(ctx.duration_for_type(NoteTypeValue::Eighth, 0), 2.0);
    assert_eq!(ctx.duration_for_type(NoteTypeValue::Half, 0), 8.0);
    assert_eq!(ctx.duration_for_type(NoteTypeValue::Whole, 0), 16.0);

    // Verify notes match calculated durations
    match &measure.content[1] {
        MeasureContent::Note(note) => {
            assert_eq!(note.duration, Some(4.0)); // quarter
        }
        _ => panic!("Expected Note"),
    }

    match &measure.content[2] {
        MeasureContent::Note(note) => {
            assert_eq!(note.duration, Some(2.0)); // eighth
        }
        _ => panic!("Expected Note"),
    }

    match &measure.content[4] {
        MeasureContent::Note(note) => {
            assert_eq!(note.duration, Some(8.0)); // half
        }
        _ => panic!("Expected Note"),
    }
}

#[test]
fn test_parse_dotted_notes_with_divisions() {
    use crate::model::duration::DurationContext;
    use crate::model::note::NoteTypeValue;

    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <attributes>
        <divisions>4</divisions>
      </attributes>
      <note>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <duration>6</duration>
        <type>quarter</type>
        <dot/>
      </note>
      <note>
        <pitch>
          <step>D</step>
          <octave>4</octave>
        </pitch>
        <duration>3</duration>
        <type>eighth</type>
        <dot/>
      </note>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    let measure = &score.parts[0].measures[0];

    // Extract divisions
    let divisions = match &measure.content[0] {
        MeasureContent::Attributes(attrs) => attrs.divisions.expect("divisions missing"),
        _ => panic!("Expected Attributes"),
    };

    let ctx = DurationContext::with_divisions(divisions);

    // Verify dotted calculations
    // Dotted quarter = quarter + quarter/2 = 4 + 2 = 6
    assert_eq!(ctx.duration_for_type(NoteTypeValue::Quarter, 1), 6.0);
    // Dotted eighth = eighth + eighth/2 = 2 + 1 = 3
    assert_eq!(ctx.duration_for_type(NoteTypeValue::Eighth, 1), 3.0);

    // Check first note (dotted quarter)
    match &measure.content[1] {
        MeasureContent::Note(note) => {
            assert_eq!(note.duration, Some(6.0));
            assert_eq!(note.dots.len(), 1);
        }
        _ => panic!("Expected Note"),
    }

    // Check second note (dotted eighth)
    match &measure.content[2] {
        MeasureContent::Note(note) => {
            assert_eq!(note.duration, Some(3.0));
            assert_eq!(note.dots.len(), 1);
        }
        _ => panic!("Expected Note"),
    }
}

#[test]
fn test_infer_note_type_from_parsed_durations() {
    use crate::model::duration::DurationContext;
    use crate::model::note::NoteTypeValue;

    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <attributes>
        <divisions>96</divisions>
      </attributes>
      <note>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <duration>96</duration>
        <type>quarter</type>
      </note>
      <note>
        <pitch>
          <step>D</step>
          <octave>4</octave>
        </pitch>
        <duration>48</duration>
        <type>eighth</type>
      </note>
      <note>
        <pitch>
          <step>E</step>
          <octave>4</octave>
        </pitch>
        <duration>144</duration>
        <type>quarter</type>
        <dot/>
      </note>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    let measure = &score.parts[0].measures[0];

    // Extract divisions
    let divisions = match &measure.content[0] {
        MeasureContent::Attributes(attrs) => attrs.divisions.expect("divisions missing"),
        _ => panic!("Expected Attributes"),
    };

    assert_eq!(divisions, 96.0);

    let ctx = DurationContext::with_divisions(divisions);

    // Verify note type inference
    assert_eq!(ctx.infer_note_type(96.0), Some((NoteTypeValue::Quarter, 0)));
    assert_eq!(ctx.infer_note_type(48.0), Some((NoteTypeValue::Eighth, 0)));
    assert_eq!(
        ctx.infer_note_type(144.0),
        Some((NoteTypeValue::Quarter, 1))
    ); // dotted quarter
    assert_eq!(ctx.infer_note_type(192.0), Some((NoteTypeValue::Half, 0)));
    assert_eq!(ctx.infer_note_type(384.0), Some((NoteTypeValue::Whole, 0)));
}

#[test]
fn test_divisions_changes_across_measures() {
    use crate::model::duration::DurationContext;
    use crate::model::note::NoteTypeValue;

    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <attributes>
        <divisions>1</divisions>
      </attributes>
      <note>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <duration>1</duration>
        <type>quarter</type>
      </note>
    </measure>
    <measure number="2">
      <attributes>
        <divisions>4</divisions>
      </attributes>
      <note>
        <pitch>
          <step>D</step>
          <octave>4</octave>
        </pitch>
        <duration>4</duration>
        <type>quarter</type>
      </note>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    // Measure 1: divisions=1
    let measure1 = &score.parts[0].measures[0];
    let div1 = match &measure1.content[0] {
        MeasureContent::Attributes(attrs) => attrs.divisions.expect("divisions missing"),
        _ => panic!("Expected Attributes"),
    };
    assert_eq!(div1, 1.0);

    let ctx1 = DurationContext::with_divisions(div1);
    assert_eq!(ctx1.duration_for_type(NoteTypeValue::Quarter, 0), 1.0);

    // Measure 2: divisions=4
    let measure2 = &score.parts[0].measures[1];
    let div2 = match &measure2.content[0] {
        MeasureContent::Attributes(attrs) => attrs.divisions.expect("divisions missing"),
        _ => panic!("Expected Attributes"),
    };
    assert_eq!(div2, 4.0);

    let ctx2 = DurationContext::with_divisions(div2);
    assert_eq!(ctx2.duration_for_type(NoteTypeValue::Quarter, 0), 4.0);
}

#[test]
fn test_measure_duration_calculation() {
    use crate::model::duration::{DurationContext, measure_duration};

    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <attributes>
        <divisions>4</divisions>
        <time>
          <beats>4</beats>
          <beat-type>4</beat-type>
        </time>
      </attributes>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    let measure = &score.parts[0].measures[0];

    // Extract divisions and time signature
    let (divisions, beats, beat_type) = match &measure.content[0] {
        MeasureContent::Attributes(attrs) => {
            let div = attrs.divisions.expect("divisions missing");
            let time = &attrs.times[0];
            match &time.content {
                TimeContent::Standard(std_time) => {
                    let b: u32 = std_time.signatures[0].beats.parse().unwrap();
                    let bt: u32 = std_time.signatures[0].beat_type.parse().unwrap();
                    (div, b, bt)
                }
                _ => panic!("Expected Standard time"),
            }
        }
        _ => panic!("Expected Attributes"),
    };

    assert_eq!(divisions, 4.0);
    assert_eq!(beats, 4);
    assert_eq!(beat_type, 4);

    // 4/4 time with divisions=4 should have measure duration of 16
    let expected_measure_duration = measure_duration(beats, beat_type, divisions);
    assert_eq!(expected_measure_duration, 16.0);
}

#[test]
fn test_backup_forward_with_divisions_context() {
    use crate::model::duration::DurationContext;

    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <attributes>
        <divisions>4</divisions>
      </attributes>
      <note>
        <pitch>
          <step>C</step>
          <octave>5</octave>
        </pitch>
        <duration>16</duration>
        <voice>1</voice>
        <type>whole</type>
      </note>
      <backup>
        <duration>16</duration>
      </backup>
      <note>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <duration>8</duration>
        <voice>2</voice>
        <type>half</type>
      </note>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse_score_partwise(xml).expect("parse failed");

    let measure = &score.parts[0].measures[0];

    // Extract divisions
    let divisions = match &measure.content[0] {
        MeasureContent::Attributes(attrs) => attrs.divisions.expect("divisions missing"),
        _ => panic!("Expected Attributes"),
    };

    let ctx = DurationContext::with_divisions(divisions);

    // Whole note has duration 16 (= 4 quarter notes)
    match &measure.content[1] {
        MeasureContent::Note(note) => {
            assert_eq!(note.duration, Some(16.0));
            // In quarter notes: 16 / 4 = 4
            assert_eq!(ctx.to_quarter_notes(16.0), 4.0);
        }
        _ => panic!("Expected Note"),
    }

    // Backup of 16 = 4 quarter notes
    match &measure.content[2] {
        MeasureContent::Backup(backup) => {
            assert_eq!(backup.duration, 16.0);
            assert_eq!(ctx.to_quarter_notes(16.0), 4.0);
        }
        _ => panic!("Expected Backup"),
    }

    // Half note has duration 8 (= 2 quarter notes)
    match &measure.content[3] {
        MeasureContent::Note(note) => {
            assert_eq!(note.duration, Some(8.0));
            assert_eq!(ctx.to_quarter_notes(8.0), 2.0);
        }
        _ => panic!("Expected Note"),
    }
}
