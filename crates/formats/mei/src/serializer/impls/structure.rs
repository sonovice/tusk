//! Serializer implementations for structural MEI elements.
//!
//! This module contains implementations for Measure, Staff, Layer, Section, Mdiv,
//! and their child elements.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttLayerAnl, AttLayerGes, AttLayerLog, AttLayerVis, AttMdivAnl, AttMdivGes, AttMdivLog,
    AttMdivVis, AttMeasureAnl, AttMeasureGes, AttMeasureLog, AttMeasureVis, AttSectionAnl,
    AttSectionGes, AttSectionLog, AttSectionVis, AttStaffAnl, AttStaffGes, AttStaffLog,
    AttStaffVis,
};
use tusk_model::elements::{
    Layer, LayerChild, Mdiv, MdivChild, Measure, MeasureChild, Section, SectionChild, Staff,
    StaffChild,
};

use super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Measure attribute class implementations
// ============================================================================

impl CollectAttributes for AttMeasureLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "metcon", self.metcon);
        push_attr!(attrs, "control", self.control);
        push_attr!(attrs, "left", self.left);
        push_attr!(attrs, "right", self.right);
        attrs
    }
}

impl CollectAttributes for AttMeasureGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        attrs
    }
}

impl CollectAttributes for AttMeasureVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "bar.len", self.bar_len);
        push_attr!(attrs, "bar.method", self.bar_method);
        push_attr!(attrs, "bar.place", self.bar_place);
        push_attr!(attrs, "width", self.width);
        attrs
    }
}

impl CollectAttributes for AttMeasureAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "join", vec self.join);
        attrs
    }
}

// ============================================================================
// Staff attribute class implementations
// ============================================================================

impl CollectAttributes for AttStaffLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "metcon", self.metcon);
        push_attr!(attrs, "def", self.def);
        attrs
    }
}

impl CollectAttributes for AttStaffGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttStaffGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttStaffVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "visible", self.visible);
        attrs
    }
}

impl CollectAttributes for AttStaffAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttStaffAnl has no attributes
        Vec::new()
    }
}

// ============================================================================
// Layer attribute class implementations
// ============================================================================

impl CollectAttributes for AttLayerLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "cue", self.cue);
        push_attr!(attrs, "metcon", self.metcon);
        push_attr!(attrs, "def", self.def);
        attrs
    }
}

impl CollectAttributes for AttLayerGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttLayerGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttLayerVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "visible", self.visible);
        attrs
    }
}

impl CollectAttributes for AttLayerAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttLayerAnl has no attributes
        Vec::new()
    }
}

// ============================================================================
// Section attribute class implementations
// ============================================================================

impl CollectAttributes for AttSectionLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        attrs
    }
}

impl CollectAttributes for AttSectionGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "attacca", self.attacca);
        attrs
    }
}

impl CollectAttributes for AttSectionVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "restart", self.restart);
        attrs
    }
}

impl CollectAttributes for AttSectionAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttSectionAnl has no attributes
        Vec::new()
    }
}

// ============================================================================
// Mdiv attribute class implementations
// ============================================================================

impl CollectAttributes for AttMdivLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        attrs
    }
}

impl CollectAttributes for AttMdivGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "attacca", self.attacca);
        attrs
    }
}

impl CollectAttributes for AttMdivVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttMdivVis has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttMdivAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttMdivAnl has no attributes
        Vec::new()
    }
}

// ============================================================================
// Staff element implementation
// ============================================================================

impl MeiSerialize for Staff {
    fn element_name(&self) -> &'static str {
        "staff"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.linking.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.n_integer.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
        attrs.extend(self.typed.collect_attributes());
        attrs.extend(self.staff_log.collect_attributes());
        attrs.extend(self.staff_vis.collect_attributes());
        attrs.extend(self.staff_ges.collect_attributes());
        attrs.extend(self.staff_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for StaffChild {
    fn element_name(&self) -> &'static str {
        match self {
            StaffChild::Layer(_) => "layer",
            // Other child types will have their element names here
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            StaffChild::Layer(layer) => layer.collect_all_attributes(),
            // Other child types - not yet implemented
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            StaffChild::Layer(layer) => layer.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            StaffChild::Layer(layer) => layer.serialize_children(writer),
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Layer element implementation
// ============================================================================

impl MeiSerialize for Layer {
    fn element_name(&self) -> &'static str {
        "layer"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.linking.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.n_integer.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
        attrs.extend(self.typed.collect_attributes());
        attrs.extend(self.layer_log.collect_attributes());
        attrs.extend(self.layer_vis.collect_attributes());
        attrs.extend(self.layer_ges.collect_attributes());
        attrs.extend(self.layer_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for LayerChild {
    fn element_name(&self) -> &'static str {
        match self {
            LayerChild::Note(_) => "note",
            LayerChild::Rest(_) => "rest",
            LayerChild::Chord(_) => "chord",
            LayerChild::Space(_) => "space",
            LayerChild::Beam(_) => "beam",
            LayerChild::Tuplet(_) => "tuplet",
            LayerChild::Clef(_) => "clef",
            LayerChild::Accid(_) => "accid",
            LayerChild::Artic(_) => "artic",
            LayerChild::Dot(_) => "dot",
            LayerChild::BarLine(_) => "barLine",
            LayerChild::KeySig(_) => "keySig",
            LayerChild::MeterSig(_) => "meterSig",
            LayerChild::MRest(_) => "mRest",
            LayerChild::MSpace(_) => "mSpace",
            LayerChild::MultiRest(_) => "multiRest",
            // Many other child types...
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            LayerChild::Note(note) => note.collect_all_attributes(),
            LayerChild::Rest(rest) => rest.collect_all_attributes(),
            LayerChild::Chord(chord) => chord.collect_all_attributes(),
            LayerChild::Space(space) => space.collect_all_attributes(),
            LayerChild::Accid(accid) => accid.collect_all_attributes(),
            LayerChild::Artic(artic) => artic.collect_all_attributes(),
            LayerChild::Dot(dot) => dot.collect_all_attributes(),
            // Other child types - not yet implemented
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            LayerChild::Note(note) => note.has_children(),
            LayerChild::Rest(rest) => rest.has_children(),
            LayerChild::Chord(chord) => chord.has_children(),
            LayerChild::Accid(_) => false,
            LayerChild::Artic(_) => false,
            LayerChild::Dot(_) => false,
            LayerChild::Space(_) => false, // Space has no children per MEI spec
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            LayerChild::Note(note) => note.serialize_children(writer),
            LayerChild::Rest(rest) => rest.serialize_children(writer),
            LayerChild::Chord(chord) => chord.serialize_children(writer),
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Measure element implementation
// ============================================================================

impl MeiSerialize for Measure {
    fn element_name(&self) -> &'static str {
        "measure"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.measure_log.collect_attributes());
        attrs.extend(self.measure_ges.collect_attributes());
        attrs.extend(self.measure_vis.collect_attributes());
        attrs.extend(self.measure_anl.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for MeasureChild {
    fn element_name(&self) -> &'static str {
        match self {
            MeasureChild::Staff(_) => "staff",
            MeasureChild::Hairpin(_) => "hairpin",
            MeasureChild::Slur(_) => "slur",
            MeasureChild::Tie(_) => "tie",
            MeasureChild::Dynam(_) => "dynam",
            MeasureChild::Dir(_) => "dir",
            MeasureChild::Tempo(_) => "tempo",
            MeasureChild::Fermata(_) => "fermata",
            MeasureChild::Breath(_) => "breath",
            MeasureChild::Caesura(_) => "caesura",
            MeasureChild::Trill(_) => "trill",
            MeasureChild::Mordent(_) => "mordent",
            MeasureChild::Turn(_) => "turn",
            MeasureChild::Harm(_) => "harm",
            MeasureChild::Pedal(_) => "pedal",
            MeasureChild::Arpeg(_) => "arpeg",
            MeasureChild::Gliss(_) => "gliss",
            MeasureChild::Bend(_) => "bend",
            MeasureChild::Octave(_) => "octave",
            MeasureChild::BeamSpan(_) => "beamSpan",
            MeasureChild::TupletSpan(_) => "tupletSpan",
            MeasureChild::BracketSpan(_) => "bracketSpan",
            MeasureChild::Phrase(_) => "phrase",
            MeasureChild::Lv(_) => "lv",
            MeasureChild::Ornam(_) => "ornam",
            MeasureChild::RepeatMark(_) => "repeatMark",
            MeasureChild::HarpPedal(_) => "harpPedal",
            MeasureChild::Fing(_) => "fing",
            MeasureChild::FingGrp(_) => "fingGrp",
            MeasureChild::AnchoredText(_) => "anchoredText",
            MeasureChild::Curve(_) => "curve",
            MeasureChild::Line(_) => "line",
            MeasureChild::Midi(_) => "midi",
            MeasureChild::Attacca(_) => "attacca",
            MeasureChild::CpMark(_) => "cpMark",
            MeasureChild::MetaMark(_) => "metaMark",
            MeasureChild::Reh(_) => "reh",
            MeasureChild::MNum(_) => "mNum",
            MeasureChild::StaffDef(_) => "staffDef",
            MeasureChild::Ossia(_) => "ossia",
            MeasureChild::Annot(_) => "annot",
            MeasureChild::Relation(_) => "relation",
            MeasureChild::RelationList(_) => "relationList",
            MeasureChild::Sp(_) => "sp",
            MeasureChild::StageDir(_) => "stageDir",
            MeasureChild::Pb(_) => "pb",
            MeasureChild::Sb(_) => "sb",
            MeasureChild::Cb(_) => "cb",
            MeasureChild::ColLayout(_) => "colLayout",
            MeasureChild::Gap(_) => "gap",
            MeasureChild::HandShift(_) => "handShift",
            // Editorial elements
            MeasureChild::Add(_) => "add",
            MeasureChild::App(_) => "app",
            MeasureChild::Choice(_) => "choice",
            MeasureChild::Corr(_) => "corr",
            MeasureChild::Damage(_) => "damage",
            MeasureChild::Del(_) => "del",
            MeasureChild::Orig(_) => "orig",
            MeasureChild::Reg(_) => "reg",
            MeasureChild::Restore(_) => "restore",
            MeasureChild::Sic(_) => "sic",
            MeasureChild::Subst(_) => "subst",
            MeasureChild::Supplied(_) => "supplied",
            MeasureChild::Unclear(_) => "unclear",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            MeasureChild::Staff(staff) => staff.collect_all_attributes(),
            // Other child types not yet implemented - return empty
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            MeasureChild::Staff(staff) => staff.has_children(),
            // Other child types - assume no children for now
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            MeasureChild::Staff(staff) => staff.serialize_children(writer),
            // Other child types - no-op
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Section element implementation
// ============================================================================

impl MeiSerialize for Section {
    fn element_name(&self) -> &'static str {
        "section"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
        attrs.extend(self.section_log.collect_attributes());
        attrs.extend(self.section_ges.collect_attributes());
        attrs.extend(self.section_vis.collect_attributes());
        attrs.extend(self.section_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for SectionChild {
    fn element_name(&self) -> &'static str {
        match self {
            SectionChild::Measure(_) => "measure",
            SectionChild::Staff(_) => "staff",
            SectionChild::Section(_) => "section",
            SectionChild::Expansion(_) => "expansion",
            SectionChild::Subst(_) => "subst",
            SectionChild::App(_) => "app",
            SectionChild::Ending(_) => "ending",
            SectionChild::Sb(_) => "sb",
            SectionChild::AnchoredText(_) => "anchoredText",
            SectionChild::Orig(_) => "orig",
            SectionChild::ScoreDef(_) => "scoreDef",
            SectionChild::Relation(_) => "relation",
            SectionChild::Annot(_) => "annot",
            SectionChild::Choice(_) => "choice",
            SectionChild::Add(_) => "add",
            SectionChild::Sic(_) => "sic",
            SectionChild::Reg(_) => "reg",
            SectionChild::Damage(_) => "damage",
            SectionChild::Curve(_) => "curve",
            SectionChild::Cb(_) => "cb",
            SectionChild::ColLayout(_) => "colLayout",
            SectionChild::Unclear(_) => "unclear",
            SectionChild::Pb(_) => "pb",
            SectionChild::Div(_) => "div",
            SectionChild::Gap(_) => "gap",
            SectionChild::Del(_) => "del",
            SectionChild::Line(_) => "line",
            SectionChild::HandShift(_) => "handShift",
            SectionChild::Restore(_) => "restore",
            SectionChild::StaffDef(_) => "staffDef",
            SectionChild::RelationList(_) => "relationList",
            SectionChild::Supplied(_) => "supplied",
            SectionChild::Corr(_) => "corr",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            SectionChild::Measure(measure) => measure.collect_all_attributes(),
            SectionChild::Staff(staff) => staff.collect_all_attributes(),
            SectionChild::Section(section) => section.collect_all_attributes(),
            // Other child types not yet implemented - return empty
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            SectionChild::Measure(measure) => measure.has_children(),
            SectionChild::Staff(staff) => staff.has_children(),
            SectionChild::Section(section) => section.has_children(),
            // Other child types - assume no children for now
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SectionChild::Measure(measure) => measure.serialize_children(writer),
            SectionChild::Staff(staff) => staff.serialize_children(writer),
            SectionChild::Section(section) => section.serialize_children(writer),
            // Other child types - no-op
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Mdiv element implementation
// ============================================================================

impl MeiSerialize for Mdiv {
    fn element_name(&self) -> &'static str {
        "mdiv"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.mdiv_log.collect_attributes());
        attrs.extend(self.mdiv_ges.collect_attributes());
        attrs.extend(self.mdiv_vis.collect_attributes());
        attrs.extend(self.mdiv_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for MdivChild {
    fn element_name(&self) -> &'static str {
        match self {
            MdivChild::Mdiv(_) => "mdiv",
            MdivChild::Score(_) => "score",
            MdivChild::Parts(_) => "parts",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            MdivChild::Mdiv(mdiv) => mdiv.collect_all_attributes(),
            // Score and Parts not yet fully implemented - return empty
            MdivChild::Score(_) => Vec::new(),
            MdivChild::Parts(_) => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            MdivChild::Mdiv(mdiv) => mdiv.has_children(),
            // Score and Parts - assume they have children
            MdivChild::Score(_) => true,
            MdivChild::Parts(_) => true,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            MdivChild::Mdiv(mdiv) => mdiv.serialize_children(writer),
            // Score and Parts not yet fully implemented - no-op
            MdivChild::Score(_) => Ok(()),
            MdivChild::Parts(_) => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializer::MeiSerialize;

    // ============================================================================
    // Mdiv serialization tests
    // ============================================================================

    #[test]
    fn mdiv_serializes_to_mei_xml() {
        let mut mdiv = Mdiv::default();
        mdiv.common.xml_id = Some("m1".to_string());

        let xml = mdiv.to_mei_string().expect("should serialize");

        assert!(xml.contains("<mdiv"), "should have mdiv element: {}", xml);
        assert!(xml.contains("xml:id=\"m1\""), "should have xml:id: {}", xml);
        assert!(xml.contains("/>"), "should be self-closing: {}", xml);
    }

    #[test]
    fn empty_mdiv_serializes_minimal() {
        let mdiv = Mdiv::default();
        let xml = mdiv.to_mei_string().expect("should serialize");

        assert!(xml.contains("<mdiv"), "should have mdiv element: {}", xml);
        assert!(xml.contains("/>"), "should be self-closing: {}", xml);
    }

    #[test]
    fn mdiv_serializes_with_label() {
        let mut mdiv = Mdiv::default();
        mdiv.common.xml_id = Some("m1".to_string());
        mdiv.common.label = Some("Movement 1".to_string());

        let xml = mdiv.to_mei_string().expect("should serialize");

        assert!(
            xml.contains("label=\"Movement 1\""),
            "should have label: {}",
            xml
        );
    }

    #[test]
    fn mdiv_serializes_with_nested_mdiv() {
        let mut mdiv = Mdiv::default();
        mdiv.common.xml_id = Some("m1".to_string());

        // Add nested mdiv
        let mut nested = Mdiv::default();
        nested.common.xml_id = Some("m1a".to_string());
        mdiv.children.push(MdivChild::Mdiv(Box::new(nested)));

        let xml = mdiv.to_mei_string().expect("should serialize");

        assert!(xml.contains("<mdiv"), "should have mdiv element: {}", xml);
        assert!(xml.contains("</mdiv>"), "should have closing tag: {}", xml);
        assert!(
            xml.contains("xml:id=\"m1a\""),
            "should have nested mdiv: {}",
            xml
        );
    }

    #[test]
    fn mdiv_roundtrip_serialization_deserialization() {
        use crate::deserializer::MeiDeserialize;

        // Create an mdiv
        let mut original = Mdiv::default();
        original.common.xml_id = Some("m1".to_string());
        original.common.label = Some("Movement 1".to_string());

        // Add nested mdiv
        let mut nested = Mdiv::default();
        nested.common.xml_id = Some("m1a".to_string());
        original.children.push(MdivChild::Mdiv(Box::new(nested)));

        // Serialize
        let xml = original.to_mei_string().expect("should serialize");

        // Deserialize
        let parsed = Mdiv::from_mei_str(&xml).expect("should deserialize");

        // Compare
        assert_eq!(original.common.xml_id, parsed.common.xml_id);
        assert_eq!(original.common.label, parsed.common.label);
        assert_eq!(original.children.len(), parsed.children.len());

        // Check nested mdiv
        match (&original.children[0], &parsed.children[0]) {
            (MdivChild::Mdiv(orig_nested), MdivChild::Mdiv(parsed_nested)) => {
                assert_eq!(orig_nested.common.xml_id, parsed_nested.common.xml_id);
            }
            _ => panic!("Expected nested Mdiv"),
        }
    }
}
