//! Deserializer implementations for MIDI-related MEI elements.
//!
//! This module contains implementations for Midi and InstrGrp elements.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{AttMidiAnl, AttMidiGes, AttMidiLog};
use tusk_model::elements::{InstrDef, InstrGrp, InstrGrpChild, Midi, MidiChild};

use super::{extract_attr, from_attr_string};

// ============================================================================
// Midi attribute class implementations
// ============================================================================

impl ExtractAttributes for AttMidiLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec_string self.part);
        extract_attr!(attrs, "partstaff", vec_string self.partstaff);
        extract_attr!(attrs, "staff", vec self.staff);
        Ok(())
    }
}

impl ExtractAttributes for AttMidiGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttMidiGes is empty - no attributes to extract
        Ok(())
    }
}

impl ExtractAttributes for AttMidiAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttMidiAnl is empty - no attributes to extract
        Ok(())
    }
}

// ============================================================================
// MeiDeserialize implementations
// ============================================================================

impl MeiDeserialize for Midi {
    fn element_name() -> &'static str {
        "midi"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut midi = Midi::default();

        // Extract attributes into each attribute class
        midi.common.extract_attributes(&mut attrs)?;
        midi.midi_log.extract_attributes(&mut attrs)?;
        midi.midi_ges.extract_attributes(&mut attrs)?;
        midi.midi_anl.extract_attributes(&mut attrs)?;

        // Read children if not empty
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("midi")?
            {
                match name.as_str() {
                    // MIDI child elements - skip for now, will be implemented in Phase 7
                    // "trkName" | "vel" | "chanPr" | "marker" | "prog" | "cue" | "cc" |
                    // "chan" | "metaText" | "noteOff" | "hex" | "noteOn" | "port" | "seqNum"
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(midi)
    }
}

impl MeiDeserialize for InstrGrp {
    fn element_name() -> &'static str {
        "instrGrp"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut instr_grp = InstrGrp::default();

        // Extract attributes into common attribute class
        instr_grp.common.extract_attributes(&mut attrs)?;

        // Read children if not empty
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("instrGrp")?
            {
                match name.as_str() {
                    "instrDef" => {
                        let instr_def = InstrDef::from_mei_event(reader, child_attrs, child_empty)?;
                        instr_grp
                            .children
                            .push(InstrGrpChild::InstrDef(Box::new(instr_def)));
                    }
                    // Unknown children are skipped (lenient mode)
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(instr_grp)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use crate::deserializer::MeiDeserialize;
    use tusk_model::elements::{InstrGrp, InstrGrpChild, Midi};

    // ============================================================================
    // Midi tests
    // ============================================================================

    #[test]
    fn midi_deserializes_from_empty_element() {
        let xml = r#"<midi/>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert!(midi.common.xml_id.is_none());
        assert!(midi.children.is_empty());
    }

    #[test]
    fn midi_deserializes_xml_id() {
        let xml = r#"<midi xml:id="midi1"/>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert_eq!(midi.common.xml_id, Some("midi1".to_string()));
    }

    #[test]
    fn midi_deserializes_staff_attribute() {
        let xml = r#"<midi staff="1"/>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert_eq!(midi.midi_log.staff, vec![1]);
    }

    #[test]
    fn midi_deserializes_layer_attribute() {
        let xml = r#"<midi layer="1"/>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert_eq!(midi.midi_log.layer, vec![1]);
    }

    #[test]
    fn midi_deserializes_multiple_staff_values() {
        let xml = r#"<midi staff="1 2"/>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert_eq!(midi.midi_log.staff, vec![1, 2]);
    }

    #[test]
    fn midi_deserializes_part_attribute() {
        let xml = r#"<midi part="P1"/>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert_eq!(midi.midi_log.part, vec!["P1".to_string()]);
    }

    #[test]
    fn midi_handles_unknown_attributes_leniently() {
        let xml = r#"<midi xml:id="midi1" unknown="value"/>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(midi.common.xml_id, Some("midi1".to_string()));
    }

    #[test]
    fn midi_handles_unknown_children_leniently() {
        let xml = r#"<midi xml:id="midi1">
            <unknownElement>ignored</unknownElement>
        </midi>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(midi.common.xml_id, Some("midi1".to_string()));
        assert!(midi.children.is_empty()); // unknown element was skipped
    }

    // ============================================================================
    // InstrGrp tests
    // ============================================================================

    #[test]
    fn instr_grp_deserializes_from_empty_element() {
        let xml = r#"<instrGrp/>"#;
        let instr_grp = InstrGrp::from_mei_str(xml).expect("should deserialize");

        assert!(instr_grp.common.xml_id.is_none());
        assert!(instr_grp.children.is_empty());
    }

    #[test]
    fn instr_grp_deserializes_xml_id() {
        let xml = r#"<instrGrp xml:id="ig1"/>"#;
        let instr_grp = InstrGrp::from_mei_str(xml).expect("should deserialize");

        assert_eq!(instr_grp.common.xml_id, Some("ig1".to_string()));
    }

    #[test]
    fn instr_grp_deserializes_with_instr_def_child() {
        let xml = r#"<instrGrp xml:id="ig1">
            <instrDef xml:id="id1" n="1" label="Piano"/>
        </instrGrp>"#;
        let instr_grp = InstrGrp::from_mei_str(xml).expect("should deserialize");

        assert_eq!(instr_grp.common.xml_id, Some("ig1".to_string()));
        assert_eq!(instr_grp.children.len(), 1);

        match &instr_grp.children[0] {
            InstrGrpChild::InstrDef(instr_def) => {
                assert_eq!(instr_def.basic.xml_id, Some("id1".to_string()));
                assert_eq!(instr_def.labelled.label, Some("Piano".to_string()));
            }
        }
    }

    #[test]
    fn instr_grp_deserializes_with_multiple_instr_def_children() {
        let xml = r#"<instrGrp xml:id="ig1">
            <instrDef xml:id="id1" n="1" label="Violin"/>
            <instrDef xml:id="id2" n="2" label="Viola"/>
            <instrDef xml:id="id3" n="3" label="Cello"/>
        </instrGrp>"#;
        let instr_grp = InstrGrp::from_mei_str(xml).expect("should deserialize");

        assert_eq!(instr_grp.common.xml_id, Some("ig1".to_string()));
        assert_eq!(instr_grp.children.len(), 3);

        // Check each child
        let labels: Vec<_> = instr_grp
            .children
            .iter()
            .map(|c| match c {
                InstrGrpChild::InstrDef(id) => id.labelled.label.clone(),
            })
            .collect();
        assert_eq!(
            labels,
            vec![
                Some("Violin".to_string()),
                Some("Viola".to_string()),
                Some("Cello".to_string())
            ]
        );
    }

    #[test]
    fn instr_grp_handles_unknown_attributes_leniently() {
        let xml = r#"<instrGrp xml:id="ig1" unknown="value"/>"#;
        let instr_grp = InstrGrp::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(instr_grp.common.xml_id, Some("ig1".to_string()));
    }

    #[test]
    fn instr_grp_handles_unknown_children_leniently() {
        let xml = r#"<instrGrp xml:id="ig1">
            <unknownElement>ignored</unknownElement>
            <instrDef xml:id="id1"/>
        </instrGrp>"#;
        let instr_grp = InstrGrp::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(instr_grp.common.xml_id, Some("ig1".to_string()));
        assert_eq!(instr_grp.children.len(), 1); // unknown element was skipped
    }
}
