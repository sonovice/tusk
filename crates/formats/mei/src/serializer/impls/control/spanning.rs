//! Serializer implementations for spanning/continuation elements:
//! BeamSpan, Octave, Gliss, Lv, BracketSpan, BTrem, FTrem.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttBTremAnl, AttBTremGes, AttBTremLog, AttBTremVis, AttBeamSpanAnl, AttBeamSpanGes,
    AttBeamSpanLog, AttBeamSpanVis, AttBracketSpanAnl, AttBracketSpanGes, AttBracketSpanLog,
    AttBracketSpanVis, AttFTremAnl, AttFTremGes, AttFTremLog, AttFTremVis, AttGlissAnl,
    AttGlissGes, AttGlissLog, AttGlissVis, AttLvAnl, AttLvGes, AttLvLog, AttLvVis, AttOctaveAnl,
    AttOctaveGes, AttOctaveLog, AttOctaveVis,
};
use tusk_model::elements::{
    BTrem, BTremChild, BeamSpan, BracketSpan, FTrem, FTremChild, Gliss, Lv, Octave,
};

use super::super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// BeamSpan attribute class implementations
// ============================================================================

impl MeiSerialize for BeamSpan {
    fn element_name(&self) -> &'static str {
        "beamSpan"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.beam_span_log.collect_attributes());
        attrs.extend(self.beam_span_vis.collect_attributes());
        attrs.extend(self.beam_span_ges.collect_attributes());
        attrs.extend(self.beam_span_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // BeamSpan is an empty element
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// Octave attribute class implementations
// ============================================================================

impl MeiSerialize for Octave {
    fn element_name(&self) -> &'static str {
        "octave"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.octave_log.collect_attributes());
        attrs.extend(self.octave_vis.collect_attributes());
        attrs.extend(self.octave_ges.collect_attributes());
        attrs.extend(self.octave_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Children are not serialized for now
        Ok(())
    }
}

// ============================================================================
// Gliss attribute class implementations
// ============================================================================

impl MeiSerialize for Gliss {
    fn element_name(&self) -> &'static str {
        "gliss"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.gliss_log.collect_attributes());
        attrs.extend(self.gliss_vis.collect_attributes());
        attrs.extend(self.gliss_ges.collect_attributes());
        attrs.extend(self.gliss_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Children are not serialized for now
        Ok(())
    }
}

// ============================================================================
// Lv attribute class implementations
// ============================================================================

impl MeiSerialize for Lv {
    fn element_name(&self) -> &'static str {
        "lv"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lv_log.collect_attributes());
        attrs.extend(self.lv_vis.collect_attributes());
        attrs.extend(self.lv_ges.collect_attributes());
        attrs.extend(self.lv_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Children (Curve) are not serialized for now
        Ok(())
    }
}

// ============================================================================
// BracketSpan attribute class implementations
// ============================================================================

impl MeiSerialize for BracketSpan {
    fn element_name(&self) -> &'static str {
        "bracketSpan"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.bracket_span_log.collect_attributes());
        attrs.extend(self.bracket_span_vis.collect_attributes());
        attrs.extend(self.bracket_span_ges.collect_attributes());
        attrs.extend(self.bracket_span_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Children are not serialized for now
        Ok(())
    }
}

// ============================================================================
// BTrem (bowed tremolo) attribute class implementations
// ============================================================================

impl MeiSerialize for BTrem {
    fn element_name(&self) -> &'static str {
        "bTrem"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.b_trem_log.collect_attributes());
        attrs.extend(self.b_trem_vis.collect_attributes());
        attrs.extend(self.b_trem_ges.collect_attributes());
        attrs.extend(self.b_trem_anl.collect_attributes());
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

impl MeiSerialize for BTremChild {
    fn element_name(&self) -> &'static str {
        match self {
            BTremChild::Note(_) => "note",
            BTremChild::Chord(_) => "chord",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            BTremChild::Note(elem) => elem.collect_all_attributes(),
            BTremChild::Chord(elem) => elem.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            BTremChild::Note(elem) => elem.has_children(),
            BTremChild::Chord(elem) => elem.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            BTremChild::Note(elem) => elem.serialize_children(writer),
            BTremChild::Chord(elem) => elem.serialize_children(writer),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            BTremChild::Note(elem) => elem.serialize_mei(writer),
            BTremChild::Chord(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// FTrem (fingered tremolo) attribute class implementations
// ============================================================================

impl MeiSerialize for FTrem {
    fn element_name(&self) -> &'static str {
        "fTrem"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.f_trem_log.collect_attributes());
        attrs.extend(self.f_trem_vis.collect_attributes());
        attrs.extend(self.f_trem_ges.collect_attributes());
        attrs.extend(self.f_trem_anl.collect_attributes());
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

impl MeiSerialize for FTremChild {
    fn element_name(&self) -> &'static str {
        match self {
            FTremChild::Note(_) => "note",
            FTremChild::Chord(_) => "chord",
            FTremChild::Clef(_) => "clef",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            FTremChild::Note(elem) => elem.collect_all_attributes(),
            FTremChild::Chord(elem) => elem.collect_all_attributes(),
            FTremChild::Clef(elem) => elem.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            FTremChild::Note(elem) => elem.has_children(),
            FTremChild::Chord(elem) => elem.has_children(),
            FTremChild::Clef(_) => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FTremChild::Note(elem) => elem.serialize_children(writer),
            FTremChild::Chord(elem) => elem.serialize_children(writer),
            FTremChild::Clef(_) => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FTremChild::Note(elem) => elem.serialize_mei(writer),
            FTremChild::Chord(elem) => elem.serialize_mei(writer),
            FTremChild::Clef(elem) => elem.serialize_mei(writer),
        }
    }
}
