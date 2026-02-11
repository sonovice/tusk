//! Parse → serialize roundtrip tests for all direction types.
//!
//! Each test parses a minimal MusicXML containing a specific direction-type,
//! verifies the parsed model, serializes it back, re-parses, and asserts equality.

use super::*;
use crate::serializer::MusicXmlSerialize;

/// Helper: wrap a direction-type XML fragment in a full score-partwise document.
fn wrap_direction(inner: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>P</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <direction>
        <direction-type>
          {inner}
        </direction-type>
      </direction>
    </measure>
  </part>
</score-partwise>"#
    )
}

/// Helper: parse score, extract the first direction's first direction-type content.
fn parse_first_direction_content(xml: &str) -> DirectionTypeContent {
    let score = parse_score_partwise(xml).expect("parse failed");
    match &score.parts[0].measures[0].content[0] {
        MeasureContent::Direction(dir) => dir.direction_types[0].content.clone(),
        other => panic!("Expected Direction, got {:?}", other),
    }
}

/// Helper: parse → serialize → re-parse → assert equal.
fn assert_parse_serialize_roundtrip(xml: &str) {
    let score1 = parse_score_partwise(xml).expect("first parse failed");
    let xml2 = score1.to_musicxml_string().expect("serialize failed");
    let score2 = parse_score_partwise(&xml2).expect("second parse failed");
    assert_eq!(score1, score2, "parse-serialize roundtrip mismatch");
}

// ============================================================================
// Symbol
// ============================================================================

#[test]
fn test_parse_direction_symbol() {
    let xml = wrap_direction(r#"<symbol font-family="MaestroTimes">metNote8thUp</symbol>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::Symbol(symbols) => {
            assert_eq!(symbols.len(), 1);
            assert_eq!(symbols[0].value, "metNote8thUp");
            assert_eq!(symbols[0].font_family, Some("MaestroTimes".to_string()));
        }
        other => panic!("Expected Symbol, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// Dashes
// ============================================================================

#[test]
fn test_parse_direction_dashes() {
    let xml =
        wrap_direction(r#"<dashes type="start" number="1" dash-length="6" space-length="3"/>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::Dashes(d) => {
            assert_eq!(d.dash_type, StartStopContinue::Start);
            assert_eq!(d.number, Some(1));
            assert_eq!(d.dash_length, Some(6.0));
            assert_eq!(d.space_length, Some(3.0));
        }
        other => panic!("Expected Dashes, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// Bracket
// ============================================================================

#[test]
fn test_parse_direction_bracket() {
    let xml = wrap_direction(
        r#"<bracket type="start" number="1" line-end="down" end-length="10" line-type="dashed"/>"#,
    );
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::Bracket(b) => {
            assert_eq!(b.bracket_type, StartStopContinue::Start);
            assert_eq!(b.number, Some(1));
            assert_eq!(b.line_end, LineEnd::Down);
            assert_eq!(b.end_length, Some(10.0));
            assert_eq!(b.line_type, Some(LineType::Dashed));
        }
        other => panic!("Expected Bracket, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// Harp Pedals
// ============================================================================

#[test]
fn test_parse_direction_harp_pedals() {
    let xml = wrap_direction(
        r#"<harp-pedals>
              <pedal-tuning><pedal-step>D</pedal-step><pedal-alter>-1</pedal-alter></pedal-tuning>
              <pedal-tuning><pedal-step>C</pedal-step><pedal-alter>0</pedal-alter></pedal-tuning>
           </harp-pedals>"#,
    );
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::HarpPedals(hp) => {
            assert_eq!(hp.pedal_tunings.len(), 2);
            assert_eq!(hp.pedal_tunings[0].pedal_step, "D");
            assert_eq!(hp.pedal_tunings[0].pedal_alter, -1.0);
            assert_eq!(hp.pedal_tunings[1].pedal_step, "C");
            assert_eq!(hp.pedal_tunings[1].pedal_alter, 0.0);
        }
        other => panic!("Expected HarpPedals, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// Damp
// ============================================================================

#[test]
fn test_parse_direction_damp() {
    let xml = wrap_direction(r#"<damp default-y="-10"/>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::Damp(d) => {
            assert_eq!(d.default_y, Some(-10.0));
        }
        other => panic!("Expected Damp, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// Damp All
// ============================================================================

#[test]
fn test_parse_direction_damp_all() {
    let xml = wrap_direction(r#"<damp-all halign="center"/>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::DampAll(d) => {
            assert_eq!(d.halign, Some(LeftCenterRight::Center));
        }
        other => panic!("Expected DampAll, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// Eyeglasses
// ============================================================================

#[test]
fn test_parse_direction_eyeglasses() {
    let xml = wrap_direction(r#"<eyeglasses default-x="5"/>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::Eyeglasses(e) => {
            assert_eq!(e.default_x, Some(5.0));
        }
        other => panic!("Expected Eyeglasses, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// String Mute
// ============================================================================

#[test]
fn test_parse_direction_string_mute_on() {
    let xml = wrap_direction(r#"<string-mute type="on"/>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::StringMute(sm) => {
            assert_eq!(sm.mute_type, StringMuteType::On);
        }
        other => panic!("Expected StringMute, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

#[test]
fn test_parse_direction_string_mute_off() {
    let xml = wrap_direction(r#"<string-mute type="off"/>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::StringMute(sm) => {
            assert_eq!(sm.mute_type, StringMuteType::Off);
        }
        other => panic!("Expected StringMute, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// Scordatura
// ============================================================================

#[test]
fn test_parse_direction_scordatura() {
    let xml = wrap_direction(
        r#"<scordatura>
              <accord string="1">
                 <tuning-step>E</tuning-step>
                 <tuning-octave>5</tuning-octave>
              </accord>
              <accord string="3">
                 <tuning-step>E</tuning-step>
                 <tuning-alter>-1</tuning-alter>
                 <tuning-octave>4</tuning-octave>
              </accord>
           </scordatura>"#,
    );
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::Scordatura(sc) => {
            assert_eq!(sc.accords.len(), 2);
            assert_eq!(sc.accords[0].string, 1);
            assert_eq!(sc.accords[0].tuning_step, "E");
            assert_eq!(sc.accords[0].tuning_octave, 5);
            assert_eq!(sc.accords[0].tuning_alter, None);
            assert_eq!(sc.accords[1].string, 3);
            assert_eq!(sc.accords[1].tuning_alter, Some(-1.0));
        }
        other => panic!("Expected Scordatura, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// Image
// ============================================================================

#[test]
fn test_parse_direction_image() {
    let xml =
        wrap_direction(r#"<image source="logo.png" type="image/png" height="50" width="100"/>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::Image(img) => {
            assert_eq!(img.source, "logo.png");
            assert_eq!(img.image_type, "image/png");
            assert_eq!(img.height, Some(50.0));
            assert_eq!(img.width, Some(100.0));
        }
        other => panic!("Expected Image, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// Principal Voice
// ============================================================================

#[test]
fn test_parse_direction_principal_voice() {
    let xml =
        wrap_direction(r#"<principal-voice symbol="Hauptstimme" type="start">I</principal-voice>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::PrincipalVoice(pv) => {
            assert_eq!(pv.voice_type, StartStop::Start);
            assert_eq!(pv.symbol, PrincipalVoiceSymbol::Hauptstimme);
            assert_eq!(pv.value, Some("I".to_string()));
        }
        other => panic!("Expected PrincipalVoice, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

#[test]
fn test_parse_direction_principal_voice_stop() {
    let xml = wrap_direction(r#"<principal-voice symbol="plain" type="stop"></principal-voice>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::PrincipalVoice(pv) => {
            assert_eq!(pv.voice_type, StartStop::Stop);
            assert_eq!(pv.symbol, PrincipalVoiceSymbol::Plain);
            assert_eq!(pv.value, None);
        }
        other => panic!("Expected PrincipalVoice, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// Percussion — various content types
// ============================================================================

#[test]
fn test_parse_direction_percussion_glass() {
    let xml = wrap_direction(r#"<percussion><glass>wind chimes</glass></percussion>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::Percussion(percs) => {
            assert_eq!(percs.len(), 1);
            assert!(matches!(&percs[0].content, PercussionContent::Glass(s) if s == "wind chimes"));
        }
        other => panic!("Expected Percussion, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

#[test]
fn test_parse_direction_percussion_timpani() {
    let xml = wrap_direction(r#"<percussion><timpani/></percussion>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::Percussion(percs) => {
            assert_eq!(percs.len(), 1);
            assert!(matches!(&percs[0].content, PercussionContent::Timpani));
        }
        other => panic!("Expected Percussion, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

#[test]
fn test_parse_direction_percussion_stick() {
    let xml = wrap_direction(
        r#"<percussion>
              <stick tip="up">
                 <stick-type>yarn</stick-type>
                 <stick-material>soft</stick-material>
              </stick>
           </percussion>"#,
    );
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::Percussion(percs) => {
            assert_eq!(percs.len(), 1);
            match &percs[0].content {
                PercussionContent::Stick(s) => {
                    assert_eq!(s.stick_type, "yarn");
                    assert_eq!(s.stick_material, "soft");
                    assert_eq!(s.tip, Some(TipDirection::Up));
                }
                other => panic!("Expected Stick, got {:?}", other),
            }
        }
        other => panic!("Expected Percussion, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

#[test]
fn test_parse_direction_percussion_beater() {
    let xml = wrap_direction(r#"<percussion><beater tip="down">wire brush</beater></percussion>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::Percussion(percs) => {
            assert_eq!(percs.len(), 1);
            match &percs[0].content {
                PercussionContent::Beater(b) => {
                    assert_eq!(b.value, "wire brush");
                    assert_eq!(b.tip, Some(TipDirection::Down));
                }
                other => panic!("Expected Beater, got {:?}", other),
            }
        }
        other => panic!("Expected Percussion, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// Accordion Registration
// ============================================================================

#[test]
fn test_parse_direction_accordion_registration() {
    let xml = wrap_direction(
        r#"<accordion-registration>
              <accordion-high/>
              <accordion-middle>2</accordion-middle>
              <accordion-low/>
           </accordion-registration>"#,
    );
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::AccordionRegistration(ar) => {
            assert_eq!(ar.accordion_high, Some(true));
            assert_eq!(ar.accordion_middle, Some(2));
            assert_eq!(ar.accordion_low, Some(true));
        }
        other => panic!("Expected AccordionRegistration, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

#[test]
fn test_parse_direction_accordion_registration_empty() {
    // Self-closing accordion-registration with no children
    let xml = wrap_direction(r#"<accordion-registration/>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::AccordionRegistration(ar) => {
            assert_eq!(ar.accordion_high, None);
            assert_eq!(ar.accordion_middle, None);
            assert_eq!(ar.accordion_low, None);
        }
        other => panic!("Expected AccordionRegistration, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// Staff Divide
// ============================================================================

#[test]
fn test_parse_direction_staff_divide_down() {
    let xml = wrap_direction(r#"<staff-divide type="down"/>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::StaffDivide(sd) => {
            assert_eq!(sd.divide_type, StaffDivideType::Down);
        }
        other => panic!("Expected StaffDivide, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

#[test]
fn test_parse_direction_staff_divide_up_down() {
    let xml = wrap_direction(r#"<staff-divide type="up-down"/>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::StaffDivide(sd) => {
            assert_eq!(sd.divide_type, StaffDivideType::UpDown);
        }
        other => panic!("Expected StaffDivide, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// Other Direction
// ============================================================================

#[test]
fn test_parse_direction_other_direction() {
    let xml = wrap_direction(
        r#"<other-direction smufl="wiggleArpeggiatoUp">Custom arpeggio</other-direction>"#,
    );
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::OtherDirection(od) => {
            assert_eq!(od.value, Some("Custom arpeggio".to_string()));
            assert_eq!(od.smufl, Some("wiggleArpeggiatoUp".to_string()));
        }
        other => panic!("Expected OtherDirection, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

#[test]
fn test_parse_direction_other_direction_empty() {
    let xml = wrap_direction(r#"<other-direction print-object="no"/>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::OtherDirection(od) => {
            assert_eq!(od.value, None);
            assert_eq!(od.print_object, Some(YesNo::No));
        }
        other => panic!("Expected OtherDirection, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// Segno (already parsed before 27.1, but test parse-serialize roundtrip)
// ============================================================================

#[test]
fn test_parse_direction_segno() {
    let xml = wrap_direction(r#"<segno default-y="20" halign="center"/>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::Segno(segnos) => {
            assert_eq!(segnos.len(), 1);
            assert_eq!(segnos[0].default_y, Some(20.0));
            assert_eq!(segnos[0].halign, Some(LeftCenterRight::Center));
        }
        other => panic!("Expected Segno, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// Coda (already parsed before 27.1, but test parse-serialize roundtrip)
// ============================================================================

#[test]
fn test_parse_direction_coda() {
    let xml = wrap_direction(r#"<coda default-y="25"/>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::Coda(codas) => {
            assert_eq!(codas.len(), 1);
            assert_eq!(codas[0].default_y, Some(25.0));
        }
        other => panic!("Expected Coda, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// Rehearsal (already parsed before 27.1, but test parse-serialize roundtrip)
// ============================================================================

#[test]
fn test_parse_direction_rehearsal() {
    let xml = wrap_direction(r#"<rehearsal enclosure="square" font-weight="bold">A</rehearsal>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::Rehearsal(reh) => {
            assert_eq!(reh.len(), 1);
            assert_eq!(reh[0].value, "A");
            assert_eq!(reh[0].enclosure, Some(EnclosureShape::Square));
            assert_eq!(reh[0].font_weight, Some(FontWeight::Bold));
        }
        other => panic!("Expected Rehearsal, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// Pedal (already parsed before 27.1, but test parse-serialize roundtrip)
// ============================================================================

#[test]
fn test_parse_direction_pedal() {
    let xml = wrap_direction(r#"<pedal type="start" line="yes" sign="yes"/>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::Pedal(p) => {
            assert_eq!(p.pedal_type, PedalType::Start);
            assert_eq!(p.line, Some(YesNo::Yes));
            assert_eq!(p.sign, Some(YesNo::Yes));
        }
        other => panic!("Expected Pedal, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}

// ============================================================================
// Octave Shift (already parsed before 27.1, but test parse-serialize roundtrip)
// ============================================================================

#[test]
fn test_parse_direction_octave_shift() {
    let xml = wrap_direction(r#"<octave-shift type="down" size="8"/>"#);
    match parse_first_direction_content(&xml) {
        DirectionTypeContent::OctaveShift(os) => {
            assert_eq!(os.shift_type, OctaveShiftType::Down);
            assert_eq!(os.size, Some(8));
        }
        other => panic!("Expected OctaveShift, got {:?}", other),
    }
    assert_parse_serialize_roundtrip(&xml);
}
