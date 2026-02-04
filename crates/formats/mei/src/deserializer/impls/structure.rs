//! Deserializer implementations for structural MEI elements.
//!
//! This module contains implementations for Measure, Staff, Layer, Section, Mdiv.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttLayerAnl, AttLayerGes, AttLayerLog, AttLayerVis, AttMdivAnl, AttMdivGes, AttMdivLog,
    AttMdivVis, AttMeasureAnl, AttMeasureGes, AttMeasureLog, AttMeasureVis, AttSectionAnl,
    AttSectionGes, AttSectionLog, AttSectionVis, AttStaffAnl, AttStaffGes, AttStaffLog,
    AttStaffVis,
};
use tusk_model::elements::{
    Beam, Chord, Layer, LayerChild, Mdiv, MdivChild, Measure, MeasureChild, Note, Rest, Section,
    SectionChild, Space, Staff, StaffChild, Tuplet,
};

use super::{extract_attr, from_attr_string};

// ============================================================================
// Measure attribute class implementations
// ============================================================================

impl ExtractAttributes for AttMeasureLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "metcon", self.metcon);
        extract_attr!(attrs, "control", self.control);
        extract_attr!(attrs, "left", self.left);
        extract_attr!(attrs, "right", self.right);
        Ok(())
    }
}

impl ExtractAttributes for AttMeasureGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        Ok(())
    }
}

impl ExtractAttributes for AttMeasureVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "bar.len", self.bar_len);
        extract_attr!(attrs, "bar.method", self.bar_method);
        extract_attr!(attrs, "bar.place", self.bar_place);
        extract_attr!(attrs, "width", self.width);
        Ok(())
    }
}

impl ExtractAttributes for AttMeasureAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "join", vec self.join);
        Ok(())
    }
}

// ============================================================================
// Staff attribute class implementations
// ============================================================================

impl ExtractAttributes for AttStaffLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "metcon", self.metcon);
        extract_attr!(attrs, "def", self.def);
        Ok(())
    }
}

impl ExtractAttributes for AttStaffGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttStaffGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttStaffVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "visible", self.visible);
        Ok(())
    }
}

impl ExtractAttributes for AttStaffAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttStaffAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Layer attribute class implementations
// ============================================================================

impl ExtractAttributes for AttLayerLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "cue", self.cue);
        extract_attr!(attrs, "metcon", self.metcon);
        extract_attr!(attrs, "def", self.def);
        Ok(())
    }
}

impl ExtractAttributes for AttLayerGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttLayerGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttLayerVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "visible", self.visible);
        Ok(())
    }
}

impl ExtractAttributes for AttLayerAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttLayerAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Section attribute class implementations
// ============================================================================

impl ExtractAttributes for AttSectionLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        Ok(())
    }
}

impl ExtractAttributes for AttSectionGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "attacca", self.attacca);
        Ok(())
    }
}

impl ExtractAttributes for AttSectionVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "restart", self.restart);
        Ok(())
    }
}

impl ExtractAttributes for AttSectionAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttSectionAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Mdiv attribute class implementations
// ============================================================================

impl ExtractAttributes for AttMdivLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        Ok(())
    }
}

impl ExtractAttributes for AttMdivGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "attacca", self.attacca);
        Ok(())
    }
}

impl ExtractAttributes for AttMdivVis {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttMdivVis has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttMdivAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttMdivAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Element implementations
// ============================================================================

impl MeiDeserialize for Staff {
    fn element_name() -> &'static str {
        "staff"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut staff = Staff::default();

        // Extract attributes from the various attribute classes
        // AttBasic
        extract_attr!(attrs, "xml:id", string staff.basic.xml_id);
        extract_attr!(attrs, "xml:base", staff.basic.xml_base);
        // AttLabelled
        extract_attr!(attrs, "label", string staff.labelled.label);
        // AttLinking
        extract_attr!(attrs, "copyof", staff.linking.copyof);
        extract_attr!(attrs, "corresp", vec staff.linking.corresp);
        extract_attr!(attrs, "follows", vec staff.linking.follows);
        extract_attr!(attrs, "next", vec staff.linking.next);
        extract_attr!(attrs, "precedes", vec staff.linking.precedes);
        extract_attr!(attrs, "prev", vec staff.linking.prev);
        extract_attr!(attrs, "sameas", vec staff.linking.sameas);
        extract_attr!(attrs, "synch", vec staff.linking.synch);
        // AttNInteger
        extract_attr!(attrs, "n", staff.n_integer.n);
        // AttResponsibility
        extract_attr!(attrs, "resp", vec staff.responsibility.resp);
        // AttTyped
        extract_attr!(attrs, "class", vec staff.typed.class);
        extract_attr!(attrs, "type", vec staff.typed.r#type);
        // AttFacsimile
        staff.facsimile.extract_attributes(&mut attrs)?;
        // AttMetadataPointing
        staff.metadata_pointing.extract_attributes(&mut attrs)?;
        // Staff-specific attribute classes
        staff.staff_log.extract_attributes(&mut attrs)?;
        staff.staff_vis.extract_attributes(&mut attrs)?;
        staff.staff_ges.extract_attributes(&mut attrs)?;
        staff.staff_anl.extract_attributes(&mut attrs)?;

        // Read children if not empty - use recursive parsing for layer children
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("staff")?
            {
                match name.as_str() {
                    "layer" => {
                        let layer = Layer::from_mei_event(reader, child_attrs, child_empty)?;
                        staff.children.push(StaffChild::Layer(Box::new(layer)));
                    }
                    // Other child types can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(staff)
    }
}

/// Helper to parse Staff from raw child element data
pub(crate) fn parse_staff_from_raw(mut attrs: AttributeMap) -> Staff {
    let mut staff = Staff::default();
    // AttBasic
    if let Some(v) = attrs.remove("xml:id") {
        staff.basic.xml_id = Some(v);
    }
    if let Some(v) = attrs
        .remove("xml:base")
        .and_then(|v| from_attr_string(&v).ok())
    {
        staff.basic.xml_base = Some(v);
    }
    // AttLabelled
    if let Some(v) = attrs.remove("label") {
        staff.labelled.label = Some(v);
    }
    // AttNInteger
    if let Some(n) = attrs
        .remove("n")
        .and_then(|v| from_attr_string::<u64>(&v).ok())
    {
        staff.n_integer.n = Some(n);
    }
    // AttFacsimile
    let _ = staff.facsimile.extract_attributes(&mut attrs);
    // AttMetadataPointing
    let _ = staff.metadata_pointing.extract_attributes(&mut attrs);
    // Staff-specific
    let _ = staff.staff_log.extract_attributes(&mut attrs);
    let _ = staff.staff_vis.extract_attributes(&mut attrs);
    let _ = staff.staff_ges.extract_attributes(&mut attrs);
    let _ = staff.staff_anl.extract_attributes(&mut attrs);
    staff
}

impl MeiDeserialize for Layer {
    fn element_name() -> &'static str {
        "layer"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut layer = Layer::default();

        // Extract attributes from the various attribute classes
        // AttBasic
        extract_attr!(attrs, "xml:id", string layer.basic.xml_id);
        extract_attr!(attrs, "xml:base", layer.basic.xml_base);
        // AttLabelled
        extract_attr!(attrs, "label", string layer.labelled.label);
        // AttLinking
        extract_attr!(attrs, "copyof", layer.linking.copyof);
        extract_attr!(attrs, "corresp", vec layer.linking.corresp);
        extract_attr!(attrs, "follows", vec layer.linking.follows);
        extract_attr!(attrs, "next", vec layer.linking.next);
        extract_attr!(attrs, "precedes", vec layer.linking.precedes);
        extract_attr!(attrs, "prev", vec layer.linking.prev);
        extract_attr!(attrs, "sameas", vec layer.linking.sameas);
        extract_attr!(attrs, "synch", vec layer.linking.synch);
        // AttNInteger
        extract_attr!(attrs, "n", layer.n_integer.n);
        // AttResponsibility
        extract_attr!(attrs, "resp", vec layer.responsibility.resp);
        // AttTyped
        extract_attr!(attrs, "class", vec layer.typed.class);
        extract_attr!(attrs, "type", vec layer.typed.r#type);
        // AttFacsimile
        layer.facsimile.extract_attributes(&mut attrs)?;
        // AttMetadataPointing
        layer.metadata_pointing.extract_attributes(&mut attrs)?;
        // Layer-specific attribute classes
        layer.layer_log.extract_attributes(&mut attrs)?;
        layer.layer_vis.extract_attributes(&mut attrs)?;
        layer.layer_ges.extract_attributes(&mut attrs)?;
        layer.layer_anl.extract_attributes(&mut attrs)?;

        // Read children if not empty - use recursive parsing for proper child element handling
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("layer")?
            {
                match name.as_str() {
                    "note" => {
                        let note = Note::from_mei_event(reader, child_attrs, child_empty)?;
                        layer.children.push(LayerChild::Note(Box::new(note)));
                    }
                    "rest" => {
                        let rest = Rest::from_mei_event(reader, child_attrs, child_empty)?;
                        layer.children.push(LayerChild::Rest(Box::new(rest)));
                    }
                    "chord" => {
                        let chord = Chord::from_mei_event(reader, child_attrs, child_empty)?;
                        layer.children.push(LayerChild::Chord(Box::new(chord)));
                    }
                    "space" => {
                        let space = Space::from_mei_event(reader, child_attrs, child_empty)?;
                        layer.children.push(LayerChild::Space(Box::new(space)));
                    }
                    "beam" => {
                        let beam = Beam::from_mei_event(reader, child_attrs, child_empty)?;
                        layer.children.push(LayerChild::Beam(Box::new(beam)));
                    }
                    "tuplet" => {
                        let tuplet = Tuplet::from_mei_event(reader, child_attrs, child_empty)?;
                        layer.children.push(LayerChild::Tuplet(Box::new(tuplet)));
                    }
                    // Other child types can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(layer)
    }
}

impl MeiDeserialize for Measure {
    fn element_name() -> &'static str {
        "measure"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut measure = Measure::default();

        // Extract attributes into each attribute class
        measure.common.extract_attributes(&mut attrs)?;
        measure.facsimile.extract_attributes(&mut attrs)?;
        measure.metadata_pointing.extract_attributes(&mut attrs)?;
        measure.pointing.extract_attributes(&mut attrs)?;
        measure.measure_log.extract_attributes(&mut attrs)?;
        measure.measure_ges.extract_attributes(&mut attrs)?;
        measure.measure_vis.extract_attributes(&mut attrs)?;
        measure.measure_anl.extract_attributes(&mut attrs)?;
        measure.target_eval.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Read children if not an empty element
        if !is_empty {
            let children_raw = reader.read_children_raw("measure")?;
            for (name, child_attrs, _child_empty, _content) in children_raw {
                match name.as_str() {
                    "staff" => {
                        let staff = parse_staff_from_raw(child_attrs);
                        measure.children.push(MeasureChild::Staff(Box::new(staff)));
                    }
                    // Other child types can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        // Unknown child element - skip in lenient mode
                    }
                }
            }
        }

        Ok(measure)
    }
}

impl MeiDeserialize for Section {
    fn element_name() -> &'static str {
        "section"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut section = Section::default();

        // Extract attributes into each attribute class
        section.common.extract_attributes(&mut attrs)?;
        section.facsimile.extract_attributes(&mut attrs)?;
        section.metadata_pointing.extract_attributes(&mut attrs)?;
        section.pointing.extract_attributes(&mut attrs)?;
        section.target_eval.extract_attributes(&mut attrs)?;
        section.section_log.extract_attributes(&mut attrs)?;
        section.section_ges.extract_attributes(&mut attrs)?;
        section.section_vis.extract_attributes(&mut attrs)?;
        section.section_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Read children if not an empty element
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("section")?
            {
                match name.as_str() {
                    "measure" => {
                        let measure = Measure::from_mei_event(reader, child_attrs, child_empty)?;
                        section
                            .children
                            .push(SectionChild::Measure(Box::new(measure)));
                    }
                    "staff" => {
                        let staff = Staff::from_mei_event(reader, child_attrs, child_empty)?;
                        section.children.push(SectionChild::Staff(Box::new(staff)));
                    }
                    "section" => {
                        // Handle nested sections recursively
                        let nested_section =
                            Section::from_mei_event(reader, child_attrs, child_empty)?;
                        section
                            .children
                            .push(SectionChild::Section(Box::new(nested_section)));
                    }
                    // Other child types can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(section)
    }
}

impl MeiDeserialize for Mdiv {
    fn element_name() -> &'static str {
        "mdiv"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut mdiv = Mdiv::default();

        // Extract attributes into each attribute class
        mdiv.common.extract_attributes(&mut attrs)?;
        mdiv.facsimile.extract_attributes(&mut attrs)?;
        mdiv.metadata_pointing.extract_attributes(&mut attrs)?;
        mdiv.mdiv_log.extract_attributes(&mut attrs)?;
        mdiv.mdiv_ges.extract_attributes(&mut attrs)?;
        mdiv.mdiv_vis.extract_attributes(&mut attrs)?;
        mdiv.mdiv_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Read children if not an empty element
        // mdiv can contain: nested mdiv, score, or parts
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("mdiv")?
            {
                match name.as_str() {
                    "mdiv" => {
                        // Handle nested mdiv recursively
                        let nested_mdiv = Mdiv::from_mei_event(reader, child_attrs, child_empty)?;
                        mdiv.children.push(MdivChild::Mdiv(Box::new(nested_mdiv)));
                    }
                    // Note: score and parts are more complex elements that would need
                    // their own MeiDeserialize implementations. For now, we skip them
                    // in lenient mode and only parse nested mdiv elements.
                    // Other child types can be added here as needed
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(mdiv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // Mdiv deserialization tests
    // ============================================================================

    #[test]
    fn mdiv_deserializes_from_empty_element() {
        let xml = r#"<mdiv/>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        assert!(mdiv.common.xml_id.is_none());
        assert!(mdiv.children.is_empty());
    }

    #[test]
    fn mdiv_deserializes_xml_id() {
        let xml = r#"<mdiv xml:id="m1"/>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mdiv.common.xml_id, Some("m1".to_string()));
    }

    #[test]
    fn mdiv_deserializes_common_attributes() {
        let xml = r#"<mdiv xml:id="m1" n="1" label="Movement 1"/>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mdiv.common.xml_id, Some("m1".to_string()));
        assert!(mdiv.common.n.is_some());
        assert_eq!(mdiv.common.label, Some("Movement 1".to_string()));
    }

    #[test]
    fn mdiv_deserializes_attacca() {
        let xml = r#"<mdiv attacca="true"/>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        assert!(mdiv.mdiv_ges.attacca.is_some());
    }

    #[test]
    fn mdiv_deserializes_with_nested_mdiv() {
        let xml = r#"<mdiv xml:id="m1">
            <mdiv xml:id="m1a"/>
            <mdiv xml:id="m1b"/>
        </mdiv>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mdiv.common.xml_id, Some("m1".to_string()));
        assert_eq!(mdiv.children.len(), 2);

        // First child should be mdiv
        match &mdiv.children[0] {
            MdivChild::Mdiv(child_mdiv) => {
                assert_eq!(child_mdiv.common.xml_id, Some("m1a".to_string()));
            }
            other => panic!("Expected Mdiv, got {:?}", other),
        }

        // Second child should be mdiv
        match &mdiv.children[1] {
            MdivChild::Mdiv(child_mdiv) => {
                assert_eq!(child_mdiv.common.xml_id, Some("m1b".to_string()));
            }
            other => panic!("Expected Mdiv, got {:?}", other),
        }
    }

    #[test]
    fn mdiv_handles_unknown_attributes_leniently() {
        let xml = r#"<mdiv xml:id="m1" unknown="value"/>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(mdiv.common.xml_id, Some("m1".to_string()));
    }

    #[test]
    fn mdiv_deserializes_with_xml_declaration() {
        let xml = r#"<?xml version="1.0"?><mdiv xml:id="m1"/>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mdiv.common.xml_id, Some("m1".to_string()));
    }

    #[test]
    fn mdiv_ignores_unknown_child_elements() {
        let xml = r#"<mdiv><unknownElement/><mdiv xml:id="nested"/></mdiv>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        // Only the mdiv child should be parsed, unknown element skipped
        assert_eq!(mdiv.children.len(), 1);
        match &mdiv.children[0] {
            MdivChild::Mdiv(child) => {
                assert_eq!(child.common.xml_id, Some("nested".to_string()));
            }
            other => panic!("Expected Mdiv, got {:?}", other),
        }
    }
}
