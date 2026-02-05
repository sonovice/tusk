//! Round-trip serialization tests for harmony elements.
//!
//! Tests for ChordTable, ChordDef, ChordMember, Barre elements.

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;

// ============================================================================
// ChordTable Tests
// ============================================================================

#[test]
fn chord_table_roundtrip_empty() {
    use tusk_model::elements::ChordTable;

    let xml = r#"<chordTable/>"#;
    let parsed = ChordTable::from_mei_str(xml).expect("parse");
    let output = parsed.to_mei_string().expect("serialize");

    // Verify roundtrip
    let reparsed = ChordTable::from_mei_str(&output).expect("reparse");
    assert!(reparsed.children.is_empty());
}

#[test]
fn chord_table_roundtrip_with_id() {
    use tusk_model::elements::ChordTable;

    let xml = r#"<chordTable xml:id="ct1"/>"#;
    let parsed = ChordTable::from_mei_str(xml).expect("parse");
    let output = parsed.to_mei_string().expect("serialize");

    let reparsed = ChordTable::from_mei_str(&output).expect("reparse");
    assert_eq!(reparsed.common.xml_id, Some("ct1".to_string()));
}

#[test]
fn chord_table_roundtrip_with_chord_defs() {
    use tusk_model::elements::ChordTable;

    let xml = r#"<chordTable xml:id="ct1">
        <chordDef xml:id="cd1"/>
        <chordDef xml:id="cd2" tab.pos="5"/>
    </chordTable>"#;
    let parsed = ChordTable::from_mei_str(xml).expect("parse");
    let output = parsed.to_mei_string().expect("serialize");

    let reparsed = ChordTable::from_mei_str(&output).expect("reparse");
    assert_eq!(reparsed.children.len(), 2);
}

// ============================================================================
// ChordDef Tests
// ============================================================================

#[test]
fn chord_def_roundtrip_empty() {
    use tusk_model::elements::ChordDef;

    let xml = r#"<chordDef/>"#;
    let parsed = ChordDef::from_mei_str(xml).expect("parse");
    let output = parsed.to_mei_string().expect("serialize");

    let reparsed = ChordDef::from_mei_str(&output).expect("reparse");
    assert!(reparsed.children.is_empty());
}

#[test]
fn chord_def_roundtrip_with_tab_pos() {
    use tusk_model::elements::ChordDef;

    let xml = r#"<chordDef xml:id="cd1" tab.pos="5"/>"#;
    let parsed = ChordDef::from_mei_str(xml).expect("parse");
    let output = parsed.to_mei_string().expect("serialize");

    let reparsed = ChordDef::from_mei_str(&output).expect("reparse");
    assert_eq!(reparsed.common.xml_id, Some("cd1".to_string()));
    assert_eq!(reparsed.chord_def_log.tab_pos, Some(5));
}

#[test]
fn chord_def_roundtrip_with_chord_members() {
    use tusk_model::elements::ChordDef;

    let xml = r#"<chordDef xml:id="cd1">
        <chordMember pname="c" oct="4"/>
        <chordMember pname="e" oct="4"/>
        <chordMember pname="g" oct="4"/>
    </chordDef>"#;
    let parsed = ChordDef::from_mei_str(xml).expect("parse");
    let output = parsed.to_mei_string().expect("serialize");

    let reparsed = ChordDef::from_mei_str(&output).expect("reparse");
    assert_eq!(reparsed.children.len(), 3);
}

#[test]
fn chord_def_roundtrip_with_barre_and_members() {
    use tusk_model::elements::ChordDef;

    let xml = r##"<chordDef xml:id="cd1">
        <barre fret="2" startid="#cm1" endid="#cm3"/>
        <chordMember xml:id="cm1" pname="c" oct="4" tab.fret="3"/>
        <chordMember xml:id="cm2" pname="e" oct="4" tab.fret="2"/>
        <chordMember xml:id="cm3" pname="g" oct="4" tab.fret="0"/>
    </chordDef>"##;
    let parsed = ChordDef::from_mei_str(xml).expect("parse");
    let output = parsed.to_mei_string().expect("serialize");

    let reparsed = ChordDef::from_mei_str(&output).expect("reparse");
    assert_eq!(reparsed.children.len(), 4);
}

// ============================================================================
// ChordMember Tests
// ============================================================================

#[test]
fn chord_member_roundtrip_empty() {
    use tusk_model::elements::ChordMember;

    let xml = r#"<chordMember/>"#;
    let parsed = ChordMember::from_mei_str(xml).expect("parse");
    let output = parsed.to_mei_string().expect("serialize");

    let reparsed = ChordMember::from_mei_str(&output).expect("reparse");
    assert!(reparsed.chord_member_log.pname.is_none());
}

#[test]
fn chord_member_roundtrip_with_pitch() {
    use tusk_model::elements::ChordMember;

    let xml = r#"<chordMember pname="c" oct="4"/>"#;
    let parsed = ChordMember::from_mei_str(xml).expect("parse");
    let output = parsed.to_mei_string().expect("serialize");

    let reparsed = ChordMember::from_mei_str(&output).expect("reparse");
    assert!(reparsed.chord_member_log.pname.is_some());
    assert!(reparsed.chord_member_log.oct.is_some());
}

#[test]
fn chord_member_roundtrip_with_tab_attributes() {
    use tusk_model::elements::ChordMember;

    let xml = r#"<chordMember xml:id="cm1" tab.fret="3" tab.string="1"/>"#;
    let parsed = ChordMember::from_mei_str(xml).expect("parse");
    let output = parsed.to_mei_string().expect("serialize");

    let reparsed = ChordMember::from_mei_str(&output).expect("reparse");
    assert_eq!(reparsed.common.xml_id, Some("cm1".to_string()));
    assert!(reparsed.chord_member_log.tab_fret.is_some());
    assert!(reparsed.chord_member_log.tab_string.is_some());
}

#[test]
fn chord_member_roundtrip_with_accid_ges() {
    use tusk_model::elements::ChordMember;

    let xml = r#"<chordMember pname="f" oct="4" accid.ges="s"/>"#;
    let parsed = ChordMember::from_mei_str(xml).expect("parse");
    let output = parsed.to_mei_string().expect("serialize");

    let reparsed = ChordMember::from_mei_str(&output).expect("reparse");
    assert!(reparsed.chord_member_ges.accid_ges.is_some());
}

// ============================================================================
// Barre Tests
// ============================================================================

#[test]
fn barre_roundtrip_empty() {
    use tusk_model::elements::Barre;

    let xml = r#"<barre/>"#;
    let parsed = Barre::from_mei_str(xml).expect("parse");
    let output = parsed.to_mei_string().expect("serialize");

    let reparsed = Barre::from_mei_str(&output).expect("reparse");
    assert!(reparsed.fret.is_none());
}

#[test]
fn barre_roundtrip_with_fret() {
    use tusk_model::elements::Barre;

    let xml = r#"<barre fret="2"/>"#;
    let parsed = Barre::from_mei_str(xml).expect("parse");
    let output = parsed.to_mei_string().expect("serialize");

    let reparsed = Barre::from_mei_str(&output).expect("reparse");
    assert_eq!(reparsed.fret, Some(2));
}

#[test]
fn barre_roundtrip_with_start_end_id() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Barre;

    let xml = r##"<barre startid="#cm1" endid="#cm3"/>"##;
    let parsed = Barre::from_mei_str(xml).expect("parse");
    let output = parsed.to_mei_string().expect("serialize");

    let reparsed = Barre::from_mei_str(&output).expect("reparse");
    assert_eq!(
        reparsed.start_end_id.startid,
        Some(DataUri("#cm1".to_string()))
    );
    assert_eq!(
        reparsed.start_end_id.endid,
        Some(DataUri("#cm3".to_string()))
    );
}

#[test]
fn barre_roundtrip_complete() {
    use tusk_model::elements::Barre;

    let xml = r##"<barre xml:id="b1" fret="5" startid="#cm1" endid="#cm4"/>"##;
    let parsed = Barre::from_mei_str(xml).expect("parse");
    let output = parsed.to_mei_string().expect("serialize");

    let reparsed = Barre::from_mei_str(&output).expect("reparse");
    assert_eq!(reparsed.common.xml_id, Some("b1".to_string()));
    assert_eq!(reparsed.fret, Some(5));
    assert!(reparsed.start_end_id.startid.is_some());
    assert!(reparsed.start_end_id.endid.is_some());
}

// ============================================================================
// Complex Hierarchy Tests
// ============================================================================

#[test]
fn chord_table_roundtrip_complete_guitar_chord() {
    use tusk_model::elements::ChordTable;

    // A typical guitar chord (C major) in tablature format
    let xml = r##"<chordTable xml:id="guitar-chords">
        <chordDef xml:id="c-major" tab.pos="1">
            <barre fret="0"/>
            <chordMember xml:id="cm1" tab.string="6" tab.fret="0"/>
            <chordMember xml:id="cm2" tab.string="5" tab.fret="3"/>
            <chordMember xml:id="cm3" tab.string="4" tab.fret="2"/>
            <chordMember xml:id="cm4" tab.string="3" tab.fret="0"/>
            <chordMember xml:id="cm5" tab.string="2" tab.fret="1"/>
            <chordMember xml:id="cm6" tab.string="1" tab.fret="0"/>
        </chordDef>
    </chordTable>"##;
    let parsed = ChordTable::from_mei_str(xml).expect("parse");
    let output = parsed.to_mei_string().expect("serialize");

    let reparsed = ChordTable::from_mei_str(&output).expect("reparse");
    assert_eq!(reparsed.common.xml_id, Some("guitar-chords".to_string()));
    assert_eq!(reparsed.children.len(), 1);
}
