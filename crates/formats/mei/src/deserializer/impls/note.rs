//! Deserializer implementations for note-related MEI elements.
//!
//! This module contains implementations for Note, Rest, Chord, Space,
//! and their child elements (Accid, Artic, Dot).

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttAccidAnl, AttAccidGes, AttAccidLog, AttAccidVis, AttArticAnl, AttArticGes, AttArticLog,
    AttArticVis, AttChordAnl, AttChordGes, AttChordLog, AttChordVis, AttDotAnl, AttDotGes,
    AttDotLog, AttDotVis, AttDurationQuality, AttMRestAnl, AttMRestGes, AttMRestLog, AttMRestVis,
    AttNoteAnl, AttNoteGes, AttNoteLog, AttNoteVis, AttRestAnl, AttRestGes, AttRestLog, AttRestVis,
    AttSpaceAnl, AttSpaceGes, AttSpaceLog, AttSpaceVis,
};
use tusk_model::elements::{
    Accid, App, Artic, Chord, ChordChild, Dot, MRest, Note, NoteChild, Rest, RestChild, Space, Syl,
    Verse,
};

use super::{extract_attr, from_attr_string};

// ============================================================================
// Note attribute class implementations
// ============================================================================

// ============================================================================
// Accid attribute class implementations
// ============================================================================

// ============================================================================
// Rest attribute class implementations
// ============================================================================

// ============================================================================
// Dot attribute class implementations
// ============================================================================

// ============================================================================
// Artic attribute class implementations
// ============================================================================

// ============================================================================
// Chord attribute class implementations
// ============================================================================

// ============================================================================
// Space attribute class implementations
// ============================================================================

// ============================================================================
// Element implementations
// ============================================================================

impl MeiDeserialize for Accid {
    fn element_name() -> &'static str {
        "accid"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut accid = Accid::default();

        // Extract attributes into each attribute class
        accid.common.extract_attributes(&mut attrs)?;
        accid.facsimile.extract_attributes(&mut attrs)?;
        accid.accid_log.extract_attributes(&mut attrs)?;
        accid.accid_ges.extract_attributes(&mut attrs)?;
        accid.accid_vis.extract_attributes(&mut attrs)?;
        accid.accid_anl.extract_attributes(&mut attrs)?;

        // Skip to end if not empty (accid has no children we parse)
        if !is_empty {
            reader.skip_to_end("accid")?;
        }

        Ok(accid)
    }
}

impl MeiDeserialize for Artic {
    fn element_name() -> &'static str {
        "artic"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut artic = Artic::default();

        // Extract attributes into each attribute class
        artic.common.extract_attributes(&mut attrs)?;
        artic.facsimile.extract_attributes(&mut attrs)?;
        artic.artic_log.extract_attributes(&mut attrs)?;
        artic.artic_ges.extract_attributes(&mut attrs)?;
        artic.artic_vis.extract_attributes(&mut attrs)?;
        artic.artic_anl.extract_attributes(&mut attrs)?;

        // Skip to end if not empty (artic has no children we parse)
        if !is_empty {
            reader.skip_to_end("artic")?;
        }

        Ok(artic)
    }
}

impl MeiDeserialize for Note {
    fn element_name() -> &'static str {
        "note"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut note = Note::default();

        // Extract attributes into each attribute class
        note.common.extract_attributes(&mut attrs)?;
        note.facsimile.extract_attributes(&mut attrs)?;
        note.note_log.extract_attributes(&mut attrs)?;
        note.note_ges.extract_attributes(&mut attrs)?;
        note.note_vis.extract_attributes(&mut attrs)?;
        note.note_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Read children if not an empty element - use recursive parsing for proper child handling
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("note")?
            {
                match name.as_str() {
                    "accid" => {
                        let accid = Accid::from_mei_event(reader, child_attrs, child_empty)?;
                        note.children.push(NoteChild::Accid(Box::new(accid)));
                    }
                    "artic" => {
                        let artic = Artic::from_mei_event(reader, child_attrs, child_empty)?;
                        note.children.push(NoteChild::Artic(Box::new(artic)));
                    }
                    "dot" => {
                        let dot = Dot::from_mei_event(reader, child_attrs, child_empty)?;
                        note.children.push(NoteChild::Dot(Box::new(dot)));
                    }
                    "verse" => {
                        let verse = Verse::from_mei_event(reader, child_attrs, child_empty)?;
                        note.children.push(NoteChild::Verse(Box::new(verse)));
                    }
                    "syl" => {
                        let syl = Syl::from_mei_event(reader, child_attrs, child_empty)?;
                        note.children.push(NoteChild::Syl(Box::new(syl)));
                    }
                    "app" => {
                        let app = App::from_mei_event(reader, child_attrs, child_empty)?;
                        note.children.push(NoteChild::App(Box::new(app)));
                    }
                    // Other child types can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        reader.skip_unknown_child(&name, "note", child_empty)?;
                    }
                }
            }
        }

        Ok(note)
    }
}

impl MeiDeserialize for Dot {
    fn element_name() -> &'static str {
        "dot"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut dot = Dot::default();

        // Extract attributes into each attribute class
        dot.common.extract_attributes(&mut attrs)?;
        dot.facsimile.extract_attributes(&mut attrs)?;
        dot.dot_log.extract_attributes(&mut attrs)?;
        dot.dot_ges.extract_attributes(&mut attrs)?;
        dot.dot_vis.extract_attributes(&mut attrs)?;
        dot.dot_anl.extract_attributes(&mut attrs)?;

        // Skip to end if not empty (dot has no children we parse)
        if !is_empty {
            reader.skip_to_end("dot")?;
        }

        Ok(dot)
    }
}

/// Helper to parse Dot from raw child element data
pub(crate) fn parse_dot_from_raw(mut attrs: AttributeMap) -> Dot {
    let mut dot = Dot::default();
    let _ = dot.common.extract_attributes(&mut attrs);
    let _ = dot.facsimile.extract_attributes(&mut attrs);
    let _ = dot.dot_log.extract_attributes(&mut attrs);
    let _ = dot.dot_ges.extract_attributes(&mut attrs);
    let _ = dot.dot_vis.extract_attributes(&mut attrs);
    let _ = dot.dot_anl.extract_attributes(&mut attrs);
    dot
}

impl MeiDeserialize for Rest {
    fn element_name() -> &'static str {
        "rest"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut rest = Rest::default();

        // Extract attributes into each attribute class
        rest.common.extract_attributes(&mut attrs)?;
        rest.facsimile.extract_attributes(&mut attrs)?;
        rest.rest_log.extract_attributes(&mut attrs)?;
        rest.rest_ges.extract_attributes(&mut attrs)?;
        rest.rest_vis.extract_attributes(&mut attrs)?;
        rest.rest_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Read children if not an empty element
        if !is_empty {
            let children_raw = reader.read_children_raw("rest")?;
            for (name, child_attrs, _child_empty, _content) in children_raw {
                match name.as_str() {
                    "dot" => {
                        let dot = parse_dot_from_raw(child_attrs);
                        rest.children.push(RestChild::Dot(Box::new(dot)));
                    }
                    _ => {
                        tracing::warn!("skipping unknown child <{name}> in <rest>");
                    }
                }
            }
        }

        Ok(rest)
    }
}

impl MeiDeserialize for Chord {
    fn element_name() -> &'static str {
        "chord"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut chord = Chord::default();

        // Extract attributes into each attribute class
        chord.common.extract_attributes(&mut attrs)?;
        chord.facsimile.extract_attributes(&mut attrs)?;
        chord.chord_log.extract_attributes(&mut attrs)?;
        chord.chord_ges.extract_attributes(&mut attrs)?;
        chord.chord_vis.extract_attributes(&mut attrs)?;
        chord.chord_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Read children if not an empty element - use recursive parsing for proper child handling
        // This is necessary because notes within chords can have their own children (accid, artic, etc.)
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("chord")?
            {
                match name.as_str() {
                    "note" => {
                        let note = Note::from_mei_event(reader, child_attrs, child_empty)?;
                        chord.children.push(ChordChild::Note(Box::new(note)));
                    }
                    "artic" => {
                        let artic = Artic::from_mei_event(reader, child_attrs, child_empty)?;
                        chord.children.push(ChordChild::Artic(Box::new(artic)));
                    }
                    // Other child types (verse, syl, etc.) can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        reader.skip_unknown_child(&name, "chord", child_empty)?;
                    }
                }
            }
        }

        Ok(chord)
    }
}

impl MeiDeserialize for Space {
    fn element_name() -> &'static str {
        "space"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut space = Space::default();

        // Extract attributes into each attribute class
        space.common.extract_attributes(&mut attrs)?;
        space.facsimile.extract_attributes(&mut attrs)?;
        space.duration_quality.extract_attributes(&mut attrs)?;
        space.space_log.extract_attributes(&mut attrs)?;
        space.space_ges.extract_attributes(&mut attrs)?;
        space.space_vis.extract_attributes(&mut attrs)?;
        space.space_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Space has no children per MEI spec (<empty/>)
        // Skip to end if not empty (handles malformed input gracefully)
        if !is_empty {
            reader.skip_to_end("space")?;
        }

        Ok(space)
    }
}

// ============================================================================
// MRest (measure rest) attribute class implementations
// ============================================================================

// ============================================================================
// MRest (measure rest) element implementation
// ============================================================================

impl MeiDeserialize for MRest {
    fn element_name() -> &'static str {
        "mRest"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut m_rest = MRest::default();

        // Extract attributes into each attribute class
        m_rest.common.extract_attributes(&mut attrs)?;
        m_rest.facsimile.extract_attributes(&mut attrs)?;
        m_rest.m_rest_log.extract_attributes(&mut attrs)?;
        m_rest.m_rest_ges.extract_attributes(&mut attrs)?;
        m_rest.m_rest_vis.extract_attributes(&mut attrs)?;
        m_rest.m_rest_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // MRest is typically an empty element, but skip to end if not empty
        if !is_empty {
            reader.skip_to_end("mRest")?;
        }

        Ok(m_rest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};

    #[test]
    fn note_deserializes_from_empty_element() {
        let xml = r#"<note/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert!(note.common.xml_id.is_none());
        assert!(note.note_log.dur.is_none());
        assert!(note.children.is_empty());
    }

    #[test]
    fn note_deserializes_xml_id() {
        let xml = r#"<note xml:id="n1"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.common.xml_id, Some("n1".to_string()));
    }

    #[test]
    fn note_deserializes_duration() {
        let xml = r#"<note dur="4"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            note.note_log.dur,
            Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
        );
    }

    #[test]
    fn note_deserializes_pitch() {
        let xml = r#"<note pname="c" oct="4"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            note.note_log.pname,
            Some(DataPitchname::from("c".to_string()))
        );
        assert_eq!(note.note_log.oct, Some(DataOctave(4)));
    }

    #[test]
    fn note_deserializes_full_attributes() {
        let xml = r#"<note xml:id="n1" dur="4" pname="c" oct="4"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.common.xml_id, Some("n1".to_string()));
        assert_eq!(
            note.note_log.dur,
            Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
        );
        assert_eq!(
            note.note_log.pname,
            Some(DataPitchname::from("c".to_string()))
        );
        assert_eq!(note.note_log.oct, Some(DataOctave(4)));
    }

    #[test]
    fn note_deserializes_with_xml_declaration() {
        let xml = r#"<?xml version="1.0"?><note xml:id="n1" dur="4"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.common.xml_id, Some("n1".to_string()));
    }

    #[test]
    fn note_deserializes_with_accid_child() {
        // Note with accid child element
        let xml = r#"<note xml:id="n1" dur="4"><accid accid="s"/></note>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.common.xml_id, Some("n1".to_string()));
        // Children should now be parsed
        assert_eq!(note.children.len(), 1);
        match &note.children[0] {
            NoteChild::Accid(accid) => {
                assert!(accid.accid_log.accid.is_some());
            }
            other => panic!("Expected Accid, got {:?}", other),
        }
    }

    #[test]
    fn note_handles_unknown_attributes_leniently() {
        let xml = r#"<note xml:id="n1" unknown="value" dur="4"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(note.common.xml_id, Some("n1".to_string()));
    }

    #[test]
    fn note_deserializes_dots() {
        let xml = r#"<note dur="4" dots="1"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            note.note_log.dots,
            Some(tusk_model::data::DataAugmentdot(1))
        );
    }

    #[test]
    fn note_deserializes_label() {
        let xml = r#"<note label="test note"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.common.label, Some("test note".to_string()));
    }

    #[test]
    fn note_deserializes_staff_layer_vectors() {
        let xml = r#"<note staff="1 2" layer="1"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        // staff and layer are Vec<> types
        assert!(!note.note_log.staff.is_empty());
    }

    #[test]
    fn note_deserializes_escaped_attribute_values() {
        let xml = r#"<note label="Test &amp; Value"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.common.label, Some("Test & Value".to_string()));
    }

    #[test]
    fn roundtrip_serialization_deserialization() {
        use crate::serializer::MeiSerialize;

        // Create a note
        let mut original = Note::default();
        original.common.xml_id = Some("n1".to_string());
        original.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
        original.note_log.pname = Some(DataPitchname::from("c".to_string()));
        original.note_log.oct = Some(DataOctave(4));

        // Serialize
        let xml = original.to_mei_string().expect("should serialize");

        // Deserialize
        let parsed = Note::from_mei_str(&xml).expect("should deserialize");

        // Compare
        assert_eq!(original.common.xml_id, parsed.common.xml_id);
        assert_eq!(original.note_log.dur, parsed.note_log.dur);
        assert_eq!(original.note_log.pname, parsed.note_log.pname);
        assert_eq!(original.note_log.oct, parsed.note_log.oct);
    }

    // ============================================================================
    // Note with child elements tests
    // ============================================================================

    #[test]
    fn note_deserializes_accid_child() {
        let xml = r#"<note xml:id="n1" dur="4" pname="c" oct="4"><accid accid.ges="s"/></note>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.common.xml_id, Some("n1".to_string()));
        assert_eq!(note.children.len(), 1);

        // Verify the child is an Accid
        match &note.children[0] {
            tusk_model::elements::NoteChild::Accid(accid) => {
                assert!(accid.accid_ges.accid_ges.is_some());
            }
            other => panic!("Expected Accid child, got {:?}", other),
        }
    }

    #[test]
    fn note_deserializes_artic_child() {
        let xml = r#"<note xml:id="n1" dur="4"><artic artic="stacc"/></note>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.children.len(), 1);

        // Verify the child is an Artic
        match &note.children[0] {
            tusk_model::elements::NoteChild::Artic(artic) => {
                assert!(!artic.artic_log.artic.is_empty());
            }
            other => panic!("Expected Artic child, got {:?}", other),
        }
    }

    #[test]
    fn note_deserializes_multiple_children() {
        let xml = r#"<note xml:id="n1" dur="4" pname="e" oct="5">
            <artic artic="ten"/>
            <accid accid.ges="f"/>
        </note>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.children.len(), 2);

        // First child should be artic
        match &note.children[0] {
            tusk_model::elements::NoteChild::Artic(_) => {}
            other => panic!("Expected Artic first, got {:?}", other),
        }

        // Second child should be accid
        match &note.children[1] {
            tusk_model::elements::NoteChild::Accid(_) => {}
            other => panic!("Expected Accid second, got {:?}", other),
        }
    }

    #[test]
    fn note_deserializes_accid_with_written_accidental() {
        let xml = r#"<note><accid accid="s"/></note>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.children.len(), 1);
        match &note.children[0] {
            tusk_model::elements::NoteChild::Accid(accid) => {
                assert!(accid.accid_log.accid.is_some());
            }
            other => panic!("Expected Accid, got {:?}", other),
        }
    }

    #[test]
    fn note_deserializes_empty_accid_child() {
        let xml = r#"<note><accid/></note>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.children.len(), 1);
        match &note.children[0] {
            tusk_model::elements::NoteChild::Accid(_) => {}
            other => panic!("Expected Accid, got {:?}", other),
        }
    }

    #[test]
    fn note_deserializes_empty_artic_child() {
        let xml = r#"<note><artic/></note>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.children.len(), 1);
        match &note.children[0] {
            tusk_model::elements::NoteChild::Artic(_) => {}
            other => panic!("Expected Artic, got {:?}", other),
        }
    }

    #[test]
    fn note_ignores_unknown_child_elements() {
        // Unknown child elements should be skipped in lenient mode
        let xml = r#"<note><unknownElement/><accid accid.ges="f"/></note>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        // Only the accid should be parsed, unknown element skipped
        assert_eq!(note.children.len(), 1);
        match &note.children[0] {
            tusk_model::elements::NoteChild::Accid(_) => {}
            other => panic!("Expected Accid, got {:?}", other),
        }
    }

    #[test]
    fn note_deserializes_syl_child() {
        // Note with syl (syllable) child element for lyrics
        let xml = r#"<note pname="c" oct="4" dur="4"><syl>A</syl></note>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.children.len(), 1);
        match &note.children[0] {
            tusk_model::elements::NoteChild::Syl(syl) => {
                // Syl should have text content "A"
                assert_eq!(syl.children.len(), 1);
                match &syl.children[0] {
                    tusk_model::elements::SylChild::Text(text) => {
                        assert_eq!(text, "A");
                    }
                    other => panic!("Expected text content, got {:?}", other),
                }
            }
            other => panic!("Expected Syl, got {:?}", other),
        }
    }

    #[test]
    fn note_deserializes_syl_with_attributes() {
        // Syl with connector and word position attributes
        let xml = r#"<note pname="c" oct="4" dur="8"><syl con="d" wordpos="i">leop</syl></note>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.children.len(), 1);
        match &note.children[0] {
            tusk_model::elements::NoteChild::Syl(syl) => {
                // Check attributes were parsed
                assert!(syl.syl_log.con.is_some());
                assert!(syl.syl_log.wordpos.is_some());
                // Check text content
                assert_eq!(syl.children.len(), 1);
                match &syl.children[0] {
                    tusk_model::elements::SylChild::Text(text) => {
                        assert_eq!(text, "leop");
                    }
                    other => panic!("Expected text content, got {:?}", other),
                }
            }
            other => panic!("Expected Syl, got {:?}", other),
        }
    }

    // ============================================================================
    // Rest deserialization tests
    // ============================================================================

    #[test]
    fn rest_deserializes_from_empty_element() {
        let xml = r#"<rest/>"#;
        let rest = Rest::from_mei_str(xml).expect("should deserialize");

        assert!(rest.common.xml_id.is_none());
        assert!(rest.rest_log.dur.is_none());
        assert!(rest.children.is_empty());
    }

    #[test]
    fn rest_deserializes_xml_id() {
        let xml = r#"<rest xml:id="r1"/>"#;
        let rest = Rest::from_mei_str(xml).expect("should deserialize");

        assert_eq!(rest.common.xml_id, Some("r1".to_string()));
    }

    #[test]
    fn rest_deserializes_duration() {
        use tusk_model::data::{DataDurationCmn, DataDurationrests};

        let xml = r#"<rest dur="4"/>"#;
        let rest = Rest::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            rest.rest_log.dur,
            Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4))
        );
    }

    #[test]
    fn rest_deserializes_full_attributes() {
        use tusk_model::data::{DataAugmentdot, DataDurationCmn, DataDurationrests};

        let xml = r#"<rest xml:id="r1" dur="2" dots="1"/>"#;
        let rest = Rest::from_mei_str(xml).expect("should deserialize");

        assert_eq!(rest.common.xml_id, Some("r1".to_string()));
        assert_eq!(
            rest.rest_log.dur,
            Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N2))
        );
        assert_eq!(rest.rest_log.dots, Some(DataAugmentdot(1)));
    }

    #[test]
    fn rest_deserializes_with_xml_declaration() {
        let xml = r#"<?xml version="1.0"?><rest xml:id="r1" dur="4"/>"#;
        let rest = Rest::from_mei_str(xml).expect("should deserialize");

        assert_eq!(rest.common.xml_id, Some("r1".to_string()));
    }

    #[test]
    fn rest_deserializes_staff_layer_vectors() {
        let xml = r#"<rest staff="1 2" layer="1"/>"#;
        let rest = Rest::from_mei_str(xml).expect("should deserialize");

        // staff and layer are Vec<> types
        assert!(!rest.rest_log.staff.is_empty());
    }

    #[test]
    fn rest_handles_unknown_attributes_leniently() {
        let xml = r#"<rest xml:id="r1" unknown="value" dur="4"/>"#;
        let rest = Rest::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(rest.common.xml_id, Some("r1".to_string()));
    }

    #[test]
    fn rest_deserializes_with_dot_child() {
        let xml = r#"<rest xml:id="r1" dur="4"><dot/></rest>"#;
        let rest = Rest::from_mei_str(xml).expect("should deserialize");

        assert_eq!(rest.common.xml_id, Some("r1".to_string()));
        assert_eq!(rest.children.len(), 1);
        match &rest.children[0] {
            tusk_model::elements::RestChild::Dot(_) => {}
            other => panic!("Expected Dot, got {:?}", other),
        }
    }

    #[test]
    fn rest_ignores_unknown_child_elements() {
        let xml = r#"<rest><unknownElement/><dot/></rest>"#;
        let rest = Rest::from_mei_str(xml).expect("should deserialize");

        // Only the dot should be parsed, unknown element skipped
        assert_eq!(rest.children.len(), 1);
        match &rest.children[0] {
            tusk_model::elements::RestChild::Dot(_) => {}
            other => panic!("Expected Dot, got {:?}", other),
        }
    }

    // ============================================================================
    // Dot deserialization tests
    // ============================================================================

    #[test]
    fn dot_deserializes_from_empty_element() {
        let xml = r#"<dot/>"#;
        let dot = Dot::from_mei_str(xml).expect("should deserialize");

        assert!(dot.common.xml_id.is_none());
    }

    #[test]
    fn dot_deserializes_xml_id() {
        let xml = r#"<dot xml:id="d1"/>"#;
        let dot = Dot::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dot.common.xml_id, Some("d1".to_string()));
    }

    #[test]
    fn dot_deserializes_form_attribute() {
        let xml = r#"<dot form="aug"/>"#;
        let dot = Dot::from_mei_str(xml).expect("should deserialize");

        // Just verify that form was parsed (the actual enum variant isn't easily accessible)
        assert!(dot.dot_log.form.is_some());
    }

    // ============================================================================
    // Chord deserialization tests
    // ============================================================================

    #[test]
    fn chord_deserializes_from_empty_element() {
        let xml = r#"<chord/>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert!(chord.common.xml_id.is_none());
        assert!(chord.chord_log.dur.is_none());
        assert!(chord.children.is_empty());
    }

    #[test]
    fn chord_deserializes_xml_id() {
        let xml = r#"<chord xml:id="c1"/>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert_eq!(chord.common.xml_id, Some("c1".to_string()));
    }

    #[test]
    fn chord_deserializes_duration() {
        let xml = r#"<chord dur="4"/>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            chord.chord_log.dur,
            Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
        );
    }

    #[test]
    fn chord_deserializes_full_attributes() {
        let xml = r#"<chord xml:id="c1" dur="4" dots="1" staff="1" layer="1"/>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert_eq!(chord.common.xml_id, Some("c1".to_string()));
        assert_eq!(
            chord.chord_log.dur,
            Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
        );
        assert_eq!(
            chord.chord_log.dots,
            Some(tusk_model::data::DataAugmentdot(1))
        );
        assert!(!chord.chord_log.staff.is_empty());
    }

    #[test]
    fn chord_deserializes_with_xml_declaration() {
        let xml = r#"<?xml version="1.0"?><chord xml:id="c1" dur="4"/>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert_eq!(chord.common.xml_id, Some("c1".to_string()));
    }

    #[test]
    fn chord_deserializes_with_note_children() {
        let xml = r#"<chord xml:id="c1" dur="4">
            <note pname="c" oct="4"/>
            <note pname="e" oct="4"/>
            <note pname="g" oct="4"/>
        </chord>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert_eq!(chord.common.xml_id, Some("c1".to_string()));
        assert_eq!(chord.children.len(), 3);

        // First child should be a note with pname c
        match &chord.children[0] {
            tusk_model::elements::ChordChild::Note(note) => {
                assert_eq!(
                    note.note_log.pname,
                    Some(DataPitchname::from("c".to_string()))
                );
                assert_eq!(note.note_log.oct, Some(DataOctave(4)));
            }
            other => panic!("Expected Note, got {:?}", other),
        }
    }

    #[test]
    fn chord_deserializes_with_artic_child() {
        let xml = r#"<chord dur="4"><artic artic="stacc"/></chord>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert_eq!(chord.children.len(), 1);
        match &chord.children[0] {
            tusk_model::elements::ChordChild::Artic(_) => {}
            other => panic!("Expected Artic, got {:?}", other),
        }
    }

    #[test]
    fn chord_deserializes_mixed_children() {
        let xml = r#"<chord dur="4">
            <note pname="c" oct="4"/>
            <artic artic="ten"/>
            <note pname="e" oct="4"/>
        </chord>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert_eq!(chord.children.len(), 3);

        // First should be note
        match &chord.children[0] {
            tusk_model::elements::ChordChild::Note(_) => {}
            other => panic!("Expected Note first, got {:?}", other),
        }

        // Second should be artic
        match &chord.children[1] {
            tusk_model::elements::ChordChild::Artic(_) => {}
            other => panic!("Expected Artic second, got {:?}", other),
        }

        // Third should be note
        match &chord.children[2] {
            tusk_model::elements::ChordChild::Note(_) => {}
            other => panic!("Expected Note third, got {:?}", other),
        }
    }

    #[test]
    fn chord_handles_unknown_attributes_leniently() {
        let xml = r#"<chord xml:id="c1" unknown="value" dur="4"/>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(chord.common.xml_id, Some("c1".to_string()));
    }

    #[test]
    fn chord_ignores_unknown_child_elements() {
        let xml = r#"<chord><unknownElement/><note pname="c" oct="4"/></chord>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        // Only the note should be parsed, unknown element skipped
        assert_eq!(chord.children.len(), 1);
        match &chord.children[0] {
            tusk_model::elements::ChordChild::Note(_) => {}
            other => panic!("Expected Note, got {:?}", other),
        }
    }

    #[test]
    fn chord_deserializes_gestural_attributes() {
        let xml = r#"<chord dur="4" dur.ges="8"/>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert!(chord.chord_ges.dur_ges.is_some());
    }

    #[test]
    fn chord_deserializes_visual_attributes() {
        let xml = r#"<chord dur="4" stem.dir="up"/>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert!(chord.chord_vis.stem_dir.is_some());
    }

    #[test]
    fn chord_deserializes_analytical_attributes() {
        let xml = r#"<chord dur="4" fermata="above"/>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert!(chord.chord_anl.fermata.is_some());
    }

    #[test]
    fn chord_deserializes_notes_with_accid_children() {
        // Notes within chords can have accid children - they must be parsed recursively
        let xml = r#"<chord dur="4" stem.dir="up">
            <note pname="e" oct="4">
                <accid accid="s"/>
            </note>
            <note pname="e" oct="5">
                <accid accid="s"/>
            </note>
        </chord>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert_eq!(chord.children.len(), 2);

        // First note should have an accid child
        match &chord.children[0] {
            tusk_model::elements::ChordChild::Note(note) => {
                assert_eq!(
                    note.note_log.pname,
                    Some(DataPitchname::from("e".to_string()))
                );
                assert_eq!(note.note_log.oct, Some(DataOctave(4)));
                // The accid child should be parsed
                assert_eq!(note.children.len(), 1);
                match &note.children[0] {
                    tusk_model::elements::NoteChild::Accid(accid) => {
                        assert!(accid.accid_log.accid.is_some());
                    }
                    other => panic!("Expected Accid, got {:?}", other),
                }
            }
            other => panic!("Expected Note, got {:?}", other),
        }

        // Second note should also have an accid child
        match &chord.children[1] {
            tusk_model::elements::ChordChild::Note(note) => {
                assert_eq!(note.note_log.oct, Some(DataOctave(5)));
                assert_eq!(note.children.len(), 1);
                match &note.children[0] {
                    tusk_model::elements::NoteChild::Accid(_) => {}
                    other => panic!("Expected Accid, got {:?}", other),
                }
            }
            other => panic!("Expected Note, got {:?}", other),
        }
    }

    // ============================================================================
    // Space deserialization tests
    // ============================================================================

    #[test]
    fn space_deserializes_from_empty_element() {
        let xml = r#"<space/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert!(space.common.xml_id.is_none());
        assert!(space.space_log.dur.is_none());
    }

    #[test]
    fn space_deserializes_xml_id() {
        let xml = r#"<space xml:id="s1"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert_eq!(space.common.xml_id, Some("s1".to_string()));
    }

    #[test]
    fn space_deserializes_duration() {
        let xml = r#"<space dur="4"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            space.space_log.dur,
            Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
        );
    }

    #[test]
    fn space_deserializes_full_attributes() {
        use tusk_model::data::DataAugmentdot;

        let xml = r#"<space xml:id="s1" dur="2" dots="1"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert_eq!(space.common.xml_id, Some("s1".to_string()));
        assert_eq!(
            space.space_log.dur,
            Some(DataDuration::DataDurationCmn(DataDurationCmn::N2))
        );
        assert_eq!(space.space_log.dots, Some(DataAugmentdot(1)));
    }

    #[test]
    fn space_deserializes_with_xml_declaration() {
        let xml = r#"<?xml version="1.0"?><space xml:id="s1" dur="4"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert_eq!(space.common.xml_id, Some("s1".to_string()));
    }

    #[test]
    fn space_deserializes_staff_layer_vectors() {
        let xml = r#"<space staff="1 2" layer="1"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        // staff and layer are Vec<> types
        assert!(!space.space_log.staff.is_empty());
    }

    #[test]
    fn space_handles_unknown_attributes_leniently() {
        let xml = r#"<space xml:id="s1" unknown="value" dur="4"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(space.common.xml_id, Some("s1".to_string()));
    }

    #[test]
    fn space_deserializes_visual_compressable() {
        let xml = r#"<space dur="4" compressable="true"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert!(space.space_vis.compressable.is_some());
    }

    #[test]
    fn space_deserializes_gestural_duration() {
        let xml = r#"<space dur="4" dur.ges="8"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert!(space.space_ges.dur_ges.is_some());
    }

    #[test]
    fn space_deserializes_analytical_fermata() {
        let xml = r#"<space dur="4" fermata="above"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert!(space.space_anl.fermata.is_some());
    }

    #[test]
    fn space_deserializes_analytical_beam() {
        let xml = r#"<space dur="8" beam="i"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert!(!space.space_anl.beam.is_empty());
    }

    #[test]
    fn space_deserializes_analytical_tuplet() {
        let xml = r#"<space dur="8" tuplet="i1"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert!(!space.space_anl.tuplet.is_empty());
    }
}
