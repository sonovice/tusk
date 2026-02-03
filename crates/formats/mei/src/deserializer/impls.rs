//! Manual implementations of deserialization traits for MEI types.
//!
//! This module contains hand-written implementations for key attribute classes
//! and elements to demonstrate and test the deserialization pattern.
//!
//! In the future, these implementations should be code-generated from the MEI ODD
//! specification to cover all types.

use super::{AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader};
use serde::Deserialize;
use std::io::BufRead;
use tusk_model::att::{
    AttAccidAnl, AttAccidGes, AttAccidLog, AttAccidVis, AttArticAnl, AttArticGes, AttArticLog,
    AttArticVis, AttCommon, AttFacsimile, AttNoteAnl, AttNoteGes, AttNoteLog, AttNoteVis,
};
use tusk_model::elements::{Accid, Artic, Note, NoteChild};

/// Parse a value using serde_json from XML attribute string.
/// Tries multiple JSON formats to handle different serde derives:
/// - For numbers/booleans: parse as-is (e.g., "4" -> 4)
/// - For strings/enums: wrap in quotes (e.g., "c" -> "c")
fn from_attr_string<T: for<'de> Deserialize<'de>>(s: &str) -> Result<T, String> {
    // First try parsing as-is (for numbers, booleans)
    if let Ok(v) = serde_json::from_str(s) {
        return Ok(v);
    }
    // Then try as a quoted string (for strings, enums)
    let json = format!("\"{}\"", s);
    serde_json::from_str(&json).map_err(|e| e.to_string())
}

/// Helper macro to extract an optional attribute using serde deserialization.
macro_rules! extract_attr {
    ($attrs:expr, $name:expr, $field:expr) => {
        if let Some(value) = $attrs.remove($name) {
            match from_attr_string(&value) {
                Ok(v) => $field = Some(v),
                Err(_) => {
                    // In lenient mode, we can skip invalid values
                    // For strict mode, we'd return an error
                }
            }
        }
    };
    // For String fields (no serde parsing needed)
    ($attrs:expr, $name:expr, string $field:expr) => {
        if let Some(value) = $attrs.remove($name) {
            $field = Some(value);
        }
    };
    // For Vec fields
    ($attrs:expr, $name:expr, vec $field:expr) => {
        if let Some(value) = $attrs.remove($name) {
            let mut items = Vec::new();
            for part in value.split_whitespace() {
                if let Ok(v) = from_attr_string(part) {
                    items.push(v);
                }
            }
            $field = items;
        }
    };
}

// ============================================================================
// Attribute class implementations
// ============================================================================

impl ExtractAttributes for AttCommon {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "xml:id", string self.xml_id);
        extract_attr!(attrs, "xml:base", self.xml_base);
        extract_attr!(attrs, "label", string self.label);
        extract_attr!(attrs, "copyof", self.copyof);
        extract_attr!(attrs, "corresp", vec self.corresp);
        extract_attr!(attrs, "follows", vec self.follows);
        extract_attr!(attrs, "next", vec self.next);
        extract_attr!(attrs, "precedes", vec self.precedes);
        extract_attr!(attrs, "prev", vec self.prev);
        extract_attr!(attrs, "sameas", vec self.sameas);
        extract_attr!(attrs, "synch", vec self.synch);
        extract_attr!(attrs, "n", self.n);
        extract_attr!(attrs, "resp", vec self.resp);
        extract_attr!(attrs, "class", vec self.class);
        extract_attr!(attrs, "type", vec self.r#type);
        Ok(())
    }
}

impl ExtractAttributes for AttFacsimile {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "facs", vec self.facs);
        Ok(())
    }
}

impl ExtractAttributes for AttNoteLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dots", self.dots);
        extract_attr!(attrs, "colored", self.colored);
        extract_attr!(attrs, "cue", self.cue);
        extract_attr!(attrs, "dur", self.dur);
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "grace", self.grace);
        extract_attr!(attrs, "grace.time", self.grace_time);
        extract_attr!(attrs, "pname", self.pname);
        extract_attr!(attrs, "oct", self.oct);
        extract_attr!(attrs, "dur.quality", self.dur_quality);
        Ok(())
    }
}

impl ExtractAttributes for AttNoteGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "accid.ges", self.accid_ges);
        extract_attr!(attrs, "artic.ges", vec self.artic_ges);
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        extract_attr!(attrs, "instr", self.instr);
        extract_attr!(attrs, "vel", self.vel);
        extract_attr!(attrs, "num", self.num);
        extract_attr!(attrs, "numbase", self.numbase);
        extract_attr!(attrs, "oct.ges", self.oct_ges);
        extract_attr!(attrs, "pname.ges", self.pname_ges);
        extract_attr!(attrs, "pnum", self.pnum);
        extract_attr!(attrs, "tab.fing", self.tab_fing);
        extract_attr!(attrs, "tab.fret", self.tab_fret);
        extract_attr!(attrs, "tab.line", self.tab_line);
        extract_attr!(attrs, "tab.string", self.tab_string);
        extract_attr!(attrs, "tab.course", self.tab_course);
        extract_attr!(attrs, "extremis", self.extremis);
        Ok(())
    }
}

impl ExtractAttributes for AttNoteVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "enclose", self.enclose);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", string self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "head.altsym", self.head_altsym);
        extract_attr!(attrs, "head.auth", self.head_auth);
        extract_attr!(attrs, "head.color", self.head_color);
        extract_attr!(attrs, "head.fill", self.head_fill);
        extract_attr!(attrs, "head.fillcolor", self.head_fillcolor);
        extract_attr!(attrs, "head.mod", vec self.head_mod);
        extract_attr!(attrs, "head.rotation", self.head_rotation);
        extract_attr!(attrs, "head.shape", self.head_shape);
        extract_attr!(attrs, "head.visible", self.head_visible);
        extract_attr!(attrs, "loc", self.loc);
        extract_attr!(attrs, "stem.with", self.stem_with);
        extract_attr!(attrs, "stem.form", self.stem_form);
        extract_attr!(attrs, "stem.dir", self.stem_dir);
        extract_attr!(attrs, "stem.len", self.stem_len);
        extract_attr!(attrs, "stem.mod", self.stem_mod);
        extract_attr!(attrs, "stem.pos", self.stem_pos);
        extract_attr!(attrs, "stem.sameas", self.stem_sameas);
        extract_attr!(attrs, "stem.visible", self.stem_visible);
        extract_attr!(attrs, "stem.x", self.stem_x);
        extract_attr!(attrs, "stem.y", self.stem_y);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "visible", self.visible);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        extract_attr!(attrs, "breaksec", self.breaksec);
        extract_attr!(attrs, "lig", self.lig);
        Ok(())
    }
}

impl ExtractAttributes for AttNoteAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "accid", self.accid);
        extract_attr!(attrs, "artic", vec self.artic);
        extract_attr!(attrs, "deg", self.deg);
        extract_attr!(attrs, "intm", self.intm);
        extract_attr!(attrs, "mfunc", self.mfunc);
        extract_attr!(attrs, "beam", vec self.beam);
        extract_attr!(attrs, "gliss", self.gliss);
        extract_attr!(attrs, "lv", self.lv);
        extract_attr!(attrs, "ornam", vec self.ornam);
        extract_attr!(attrs, "slur", vec self.slur);
        extract_attr!(attrs, "syl", string self.syl);
        extract_attr!(attrs, "tie", vec self.tie);
        extract_attr!(attrs, "tuplet", vec self.tuplet);
        extract_attr!(attrs, "fermata", self.fermata);
        extract_attr!(attrs, "pclass", self.pclass);
        extract_attr!(attrs, "psolfa", string self.psolfa);
        Ok(())
    }
}

// ============================================================================
// Accid attribute class implementations
// ============================================================================

impl ExtractAttributes for AttAccidLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "accid", self.accid);
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "func", self.func);
        Ok(())
    }
}

impl ExtractAttributes for AttAccidGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "accid.ges", self.accid_ges);
        Ok(())
    }
}

impl ExtractAttributes for AttAccidVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "enclose", self.enclose);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", string self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "onstaff", self.onstaff);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "loc", self.loc);
        extract_attr!(attrs, "ploc", self.ploc);
        extract_attr!(attrs, "oloc", self.oloc);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttAccidAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttAccidAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Artic attribute class implementations
// ============================================================================

impl ExtractAttributes for AttArticLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "artic", vec self.artic);
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        Ok(())
    }
}

impl ExtractAttributes for AttArticGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "artic.ges", vec self.artic_ges);
        Ok(())
    }
}

impl ExtractAttributes for AttArticVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "enclose", self.enclose);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", string self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "onstaff", self.onstaff);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "loc", self.loc);
        extract_attr!(attrs, "ploc", self.ploc);
        extract_attr!(attrs, "oloc", self.oloc);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttArticAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttArticAnl has no attributes
        Ok(())
    }
}

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

/// Helper to parse Accid from raw child element data
fn parse_accid_from_raw(mut attrs: AttributeMap) -> Accid {
    let mut accid = Accid::default();
    let _ = accid.common.extract_attributes(&mut attrs);
    let _ = accid.facsimile.extract_attributes(&mut attrs);
    let _ = accid.accid_log.extract_attributes(&mut attrs);
    let _ = accid.accid_ges.extract_attributes(&mut attrs);
    let _ = accid.accid_vis.extract_attributes(&mut attrs);
    let _ = accid.accid_anl.extract_attributes(&mut attrs);
    accid
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

/// Helper to parse Artic from raw child element data
fn parse_artic_from_raw(mut attrs: AttributeMap) -> Artic {
    let mut artic = Artic::default();
    let _ = artic.common.extract_attributes(&mut attrs);
    let _ = artic.facsimile.extract_attributes(&mut attrs);
    let _ = artic.artic_log.extract_attributes(&mut attrs);
    let _ = artic.artic_ges.extract_attributes(&mut attrs);
    let _ = artic.artic_vis.extract_attributes(&mut attrs);
    let _ = artic.artic_anl.extract_attributes(&mut attrs);
    artic
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

        // Read children if not an empty element
        if !is_empty {
            let children_raw = reader.read_children_raw("note")?;
            for (name, child_attrs, _child_empty, _content) in children_raw {
                match name.as_str() {
                    "accid" => {
                        let accid = parse_accid_from_raw(child_attrs);
                        note.children.push(NoteChild::Accid(Box::new(accid)));
                    }
                    "artic" => {
                        let artic = parse_artic_from_raw(child_attrs);
                        note.children.push(NoteChild::Artic(Box::new(artic)));
                    }
                    // Other child types can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        // Unknown child element - skip in lenient mode
                    }
                }
            }
        }

        Ok(note)
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
}
