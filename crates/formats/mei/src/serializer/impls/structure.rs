//! Serializer implementations for structural MEI elements.
//!
//! This module contains implementations for Measure, Staff, Layer, Section, Mdiv,
//! and their child elements.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttClefAnl, AttClefGes, AttClefLog, AttClefVis, AttEndingAnl, AttEndingGes, AttEndingLog,
    AttEndingVis, AttEvent, AttLayerAnl, AttLayerGes, AttLayerLog, AttLayerVis, AttMRestAnl,
    AttMRestGes, AttMRestLog, AttMRestVis, AttMdivAnl, AttMdivGes, AttMdivLog, AttMdivVis,
    AttMeasureAnl, AttMeasureGes, AttMeasureLog, AttMeasureVis, AttPbAnl, AttPbGes, AttPbLog,
    AttPbVis, AttSbAnl, AttSbGes, AttSbLog, AttSbVis, AttSectionAnl, AttSectionGes, AttSectionLog,
    AttSectionVis, AttStaffAnl, AttStaffGes, AttStaffLog, AttStaffVis,
};
use tusk_model::elements::{
    Body, BodyChild, Clef, Ending, EndingChild, Layer, LayerChild, MRest, Mdiv, MdivChild, Measure,
    MeasureChild, Pb, Sb, Score, ScoreChild, Section, SectionChild, Staff, StaffChild, StaffDef,
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
// MRest (measure rest) attribute class implementations
// ============================================================================

impl CollectAttributes for AttMRestLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "cue", self.cue);
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        attrs
    }
}

impl CollectAttributes for AttMRestGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        attrs
    }
}

impl CollectAttributes for AttMRestVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "cutout", self.cutout);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "ploc", self.ploc);
        push_attr!(attrs, "oloc", self.oloc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}

impl CollectAttributes for AttMRestAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "fermata", self.fermata);
        attrs
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
// Ending attribute class implementations
// ============================================================================

impl CollectAttributes for AttEndingLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        attrs
    }
}

impl CollectAttributes for AttEndingGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttEndingGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttEndingVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        attrs
    }
}

impl CollectAttributes for AttEndingAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttEndingAnl has no attributes
        Vec::new()
    }
}

// ============================================================================
// Sb (system break) attribute class implementations
// ============================================================================

impl CollectAttributes for AttSbLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        attrs
    }
}

impl CollectAttributes for AttSbGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttSbGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttSbVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "form", self.form);
        attrs
    }
}

impl CollectAttributes for AttSbAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttSbAnl has no attributes
        Vec::new()
    }
}

// ============================================================================
// Pb (page break) attribute class implementations
// ============================================================================

impl CollectAttributes for AttPbLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        attrs
    }
}

impl CollectAttributes for AttPbGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttPbGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttPbVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "folium", self.folium);
        attrs
    }
}

impl CollectAttributes for AttPbAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttPbAnl has no attributes
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
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "StaffChild::{}::serialize_children",
                other.element_name()
            ))),
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
            LayerChild::Beam(beam) => beam.collect_all_attributes(),
            LayerChild::Tuplet(tuplet) => tuplet.collect_all_attributes(),
            LayerChild::Accid(accid) => accid.collect_all_attributes(),
            LayerChild::Artic(artic) => artic.collect_all_attributes(),
            LayerChild::Dot(dot) => dot.collect_all_attributes(),
            LayerChild::MRest(mrest) => mrest.collect_all_attributes(),
            LayerChild::Clef(clef) => clef.collect_all_attributes(),
            // Other child types - not yet implemented
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            LayerChild::Note(note) => note.has_children(),
            LayerChild::Rest(rest) => rest.has_children(),
            LayerChild::Chord(chord) => chord.has_children(),
            LayerChild::Beam(beam) => beam.has_children(),
            LayerChild::Tuplet(tuplet) => tuplet.has_children(),
            LayerChild::Accid(_) => false,
            LayerChild::Artic(_) => false,
            LayerChild::Dot(_) => false,
            LayerChild::Space(_) => false, // Space has no children per MEI spec
            LayerChild::MRest(_) => false, // MRest has no children per MEI spec
            LayerChild::Clef(_) => false,  // Clef has no children per MEI spec
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            LayerChild::Note(note) => note.serialize_children(writer),
            LayerChild::Rest(rest) => rest.serialize_children(writer),
            LayerChild::Chord(chord) => chord.serialize_children(writer),
            LayerChild::Beam(beam) => beam.serialize_children(writer),
            LayerChild::Tuplet(tuplet) => tuplet.serialize_children(writer),
            LayerChild::MRest(_) => Ok(()), // MRest has no children
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "LayerChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// MRest (measure rest) element implementation
// ============================================================================

impl MeiSerialize for MRest {
    fn element_name(&self) -> &'static str {
        "mRest"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.m_rest_log.collect_attributes());
        attrs.extend(self.m_rest_vis.collect_attributes());
        attrs.extend(self.m_rest_ges.collect_attributes());
        attrs.extend(self.m_rest_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // MRest has no children per MEI spec
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// Clef attribute class implementations
// ============================================================================

impl CollectAttributes for AttClefLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "shape", self.shape);
        push_attr!(attrs, "line", self.line);
        push_attr!(attrs, "oct", self.oct);
        push_attr!(attrs, "dis", self.dis);
        push_attr!(attrs, "dis.place", self.dis_place);
        push_attr!(attrs, "cautionary", self.cautionary);
        attrs
    }
}

impl CollectAttributes for AttClefGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new() // AttClefGes has no attributes
    }
}

impl CollectAttributes for AttClefVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        attrs
    }
}

impl CollectAttributes for AttClefAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new() // AttClefAnl has no attributes
    }
}

impl CollectAttributes for AttEvent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        attrs
    }
}

// ============================================================================
// Clef element implementation
// ============================================================================

impl MeiSerialize for Clef {
    fn element_name(&self) -> &'static str {
        "clef"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.event.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.clef_log.collect_attributes());
        attrs.extend(self.clef_ges.collect_attributes());
        attrs.extend(self.clef_vis.collect_attributes());
        attrs.extend(self.clef_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Clef has no children per MEI spec
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
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
            MeasureChild::Dynam(dynam) => dynam.collect_all_attributes(),
            MeasureChild::Dir(dir) => dir.collect_all_attributes(),
            MeasureChild::Hairpin(hairpin) => hairpin.collect_all_attributes(),
            MeasureChild::Tempo(tempo) => tempo.collect_all_attributes(),
            MeasureChild::Slur(slur) => slur.collect_all_attributes(),
            MeasureChild::Tie(tie) => tie.collect_all_attributes(),
            MeasureChild::Fermata(fermata) => fermata.collect_all_attributes(),
            MeasureChild::Trill(trill) => trill.collect_all_attributes(),
            MeasureChild::Harm(harm) => harm.collect_all_attributes(),
            MeasureChild::Pedal(pedal) => pedal.collect_all_attributes(),
            MeasureChild::TupletSpan(tuplet_span) => tuplet_span.collect_all_attributes(),
            // Other child types not yet implemented - return empty
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            MeasureChild::Staff(staff) => staff.has_children(),
            MeasureChild::Dynam(dynam) => dynam.has_children(),
            MeasureChild::Dir(dir) => dir.has_children(),
            MeasureChild::Hairpin(_) => false, // Hairpin has no children
            MeasureChild::Tempo(tempo) => tempo.has_children(),
            MeasureChild::Slur(_) => false, // Slur has no children (just attributes)
            MeasureChild::Tie(_) => false,  // Tie has no children
            MeasureChild::Fermata(_) => false, // Fermata has no children
            MeasureChild::Trill(_) => false, // Trill has no children
            MeasureChild::Harm(harm) => harm.has_children(),
            MeasureChild::Pedal(_) => false, // Pedal has no children
            MeasureChild::TupletSpan(_) => false, // TupletSpan has no children
            // Other child types - assume no children for now
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            MeasureChild::Staff(staff) => staff.serialize_children(writer),
            MeasureChild::Dynam(dynam) => dynam.serialize_children(writer),
            MeasureChild::Dir(dir) => dir.serialize_children(writer),
            MeasureChild::Tempo(tempo) => tempo.serialize_children(writer),
            MeasureChild::Fermata(_) => Ok(()), // Fermata has no children
            MeasureChild::Trill(_) => Ok(()),   // Trill has no children
            MeasureChild::Harm(harm) => harm.serialize_children(writer),
            MeasureChild::Pedal(_) => Ok(()), // Pedal has no children
            MeasureChild::TupletSpan(_) => Ok(()), // TupletSpan has no children
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "MeasureChild::{}::serialize_children",
                other.element_name()
            ))),
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

// ============================================================================
// Sb (system break) element implementation
// ============================================================================

impl MeiSerialize for Sb {
    fn element_name(&self) -> &'static str {
        "sb"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.source.collect_attributes());
        attrs.extend(self.sb_log.collect_attributes());
        attrs.extend(self.sb_ges.collect_attributes());
        attrs.extend(self.sb_vis.collect_attributes());
        attrs.extend(self.sb_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        // Sb is an empty element
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Sb has no children
        Ok(())
    }
}

// ============================================================================
// Pb (page break) element implementation
// ============================================================================

impl MeiSerialize for Pb {
    fn element_name(&self) -> &'static str {
        "pb"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.source.collect_attributes());
        attrs.extend(self.pb_log.collect_attributes());
        attrs.extend(self.pb_ges.collect_attributes());
        attrs.extend(self.pb_vis.collect_attributes());
        attrs.extend(self.pb_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        // Pb can have children (pgFoot, pgDesc, pgHead) but we're not serializing them yet
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Pb children (pgFoot, pgDesc, pgHead) not yet implemented
        Ok(())
    }
}

// ============================================================================
// Ending element implementation
// ============================================================================

impl MeiSerialize for Ending {
    fn element_name(&self) -> &'static str {
        "ending"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
        attrs.extend(self.ending_log.collect_attributes());
        attrs.extend(self.ending_ges.collect_attributes());
        attrs.extend(self.ending_vis.collect_attributes());
        attrs.extend(self.ending_anl.collect_attributes());
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

impl MeiSerialize for EndingChild {
    fn element_name(&self) -> &'static str {
        match self {
            EndingChild::Measure(_) => "measure",
            EndingChild::Staff(_) => "staff",
            EndingChild::Section(_) => "section",
            EndingChild::ScoreDef(_) => "scoreDef",
            EndingChild::StaffDef(_) => "staffDef",
            EndingChild::Sb(_) => "sb",
            EndingChild::Pb(_) => "pb",
            EndingChild::Cb(_) => "cb",
            EndingChild::Annot(_) => "annot",
            EndingChild::App(_) => "app",
            EndingChild::Choice(_) => "choice",
            EndingChild::Orig(_) => "orig",
            EndingChild::Reg(_) => "reg",
            EndingChild::Sic(_) => "sic",
            EndingChild::Corr(_) => "corr",
            EndingChild::Add(_) => "add",
            EndingChild::Del(_) => "del",
            EndingChild::Subst(_) => "subst",
            EndingChild::Supplied(_) => "supplied",
            EndingChild::Unclear(_) => "unclear",
            EndingChild::Damage(_) => "damage",
            EndingChild::Gap(_) => "gap",
            EndingChild::Restore(_) => "restore",
            EndingChild::AnchoredText(_) => "anchoredText",
            EndingChild::ColLayout(_) => "colLayout",
            EndingChild::Curve(_) => "curve",
            EndingChild::Expansion(_) => "expansion",
            EndingChild::HandShift(_) => "handShift",
            EndingChild::Line(_) => "line",
            EndingChild::Relation(_) => "relation",
            EndingChild::RelationList(_) => "relationList",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            EndingChild::Measure(measure) => measure.collect_all_attributes(),
            EndingChild::Staff(staff) => staff.collect_all_attributes(),
            EndingChild::Section(section) => section.collect_all_attributes(),
            EndingChild::ScoreDef(score_def) => score_def.collect_all_attributes(),
            EndingChild::StaffDef(staff_def) => staff_def.collect_all_attributes(),
            EndingChild::Sb(sb) => sb.collect_all_attributes(),
            EndingChild::Pb(pb) => pb.collect_all_attributes(),
            // Other child types not yet implemented - return empty
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            EndingChild::Measure(measure) => measure.has_children(),
            EndingChild::Staff(staff) => staff.has_children(),
            EndingChild::Section(section) => section.has_children(),
            EndingChild::ScoreDef(score_def) => score_def.has_children(),
            EndingChild::StaffDef(staff_def) => staff_def.has_children(),
            EndingChild::Sb(sb) => sb.has_children(),
            EndingChild::Pb(pb) => pb.has_children(),
            // Other child types - assume no children for now
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            EndingChild::Measure(measure) => measure.serialize_children(writer),
            EndingChild::Staff(staff) => staff.serialize_children(writer),
            EndingChild::Section(section) => section.serialize_children(writer),
            EndingChild::ScoreDef(score_def) => score_def.serialize_children(writer),
            EndingChild::StaffDef(staff_def) => staff_def.serialize_children(writer),
            EndingChild::Sb(sb) => sb.serialize_children(writer),
            EndingChild::Pb(pb) => pb.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "EndingChild::{}::serialize_children",
                other.element_name()
            ))),
        }
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
            SectionChild::ScoreDef(score_def) => score_def.collect_all_attributes(),
            SectionChild::Sb(sb) => sb.collect_all_attributes(),
            SectionChild::Pb(pb) => pb.collect_all_attributes(),
            SectionChild::Div(div) => div.collect_all_attributes(),
            SectionChild::StaffDef(staff_def) => staff_def.collect_all_attributes(),
            SectionChild::Ending(ending) => ending.collect_all_attributes(),
            // Other child types not yet implemented - return empty
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            SectionChild::Measure(measure) => measure.has_children(),
            SectionChild::Staff(staff) => staff.has_children(),
            SectionChild::Section(section) => section.has_children(),
            SectionChild::ScoreDef(score_def) => score_def.has_children(),
            SectionChild::Sb(sb) => sb.has_children(),
            SectionChild::Pb(pb) => pb.has_children(),
            SectionChild::Div(div) => div.has_children(),
            SectionChild::StaffDef(staff_def) => staff_def.has_children(),
            SectionChild::Ending(ending) => ending.has_children(),
            // Other child types - assume no children for now
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SectionChild::Measure(measure) => measure.serialize_children(writer),
            SectionChild::Staff(staff) => staff.serialize_children(writer),
            SectionChild::Section(section) => section.serialize_children(writer),
            SectionChild::ScoreDef(score_def) => score_def.serialize_children(writer),
            SectionChild::Sb(sb) => sb.serialize_children(writer),
            SectionChild::Pb(pb) => pb.serialize_children(writer),
            SectionChild::Div(div) => div.serialize_children(writer),
            SectionChild::StaffDef(staff_def) => staff_def.serialize_children(writer),
            SectionChild::Ending(ending) => ending.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "SectionChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Body element implementation
// ============================================================================

impl MeiSerialize for Body {
    fn element_name(&self) -> &'static str {
        "body"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
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

impl MeiSerialize for BodyChild {
    fn element_name(&self) -> &'static str {
        match self {
            BodyChild::Div(_) => "div",
            BodyChild::Mdiv(_) => "mdiv",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            BodyChild::Div(_) => Vec::new(), // Div not fully implemented yet
            BodyChild::Mdiv(mdiv) => mdiv.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            BodyChild::Div(_) => true,
            BodyChild::Mdiv(mdiv) => mdiv.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            BodyChild::Div(_) => Ok(()), // Div not fully implemented yet
            BodyChild::Mdiv(mdiv) => mdiv.serialize_children(writer),
        }
    }
}

// ============================================================================
// Score element implementation
// ============================================================================

impl MeiSerialize for Score {
    fn element_name(&self) -> &'static str {
        "score"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        // score_anl, score_ges, score_log, score_vis have no serializers yet - add empty
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

impl MeiSerialize for ScoreChild {
    fn element_name(&self) -> &'static str {
        match self {
            ScoreChild::Section(_) => "section",
            ScoreChild::ScoreDef(_) => "scoreDef",
            ScoreChild::StaffDef(_) => "staffDef",
            ScoreChild::Ending(_) => "ending",
            ScoreChild::Pb(_) => "pb",
            ScoreChild::Sb(_) => "sb",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            ScoreChild::Section(s) => s.collect_all_attributes(),
            ScoreChild::ScoreDef(s) => s.collect_all_attributes(),
            ScoreChild::StaffDef(s) => s.collect_all_attributes(),
            ScoreChild::Ending(e) => e.collect_all_attributes(),
            ScoreChild::Pb(p) => p.collect_all_attributes(),
            ScoreChild::Sb(s) => s.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            ScoreChild::Section(s) => s.has_children(),
            ScoreChild::ScoreDef(s) => s.has_children(),
            ScoreChild::StaffDef(s) => s.has_children(),
            ScoreChild::Ending(e) => e.has_children(),
            ScoreChild::Pb(p) => p.has_children(),
            ScoreChild::Sb(s) => s.has_children(),
            _ => true,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ScoreChild::Section(s) => s.serialize_children(writer),
            ScoreChild::ScoreDef(s) => s.serialize_children(writer),
            ScoreChild::StaffDef(s) => s.serialize_children(writer),
            ScoreChild::Ending(e) => e.serialize_children(writer),
            ScoreChild::Pb(p) => p.serialize_children(writer),
            ScoreChild::Sb(s) => s.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "ScoreChild::{}::serialize_children",
                other.element_name()
            ))),
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
            MdivChild::Score(score) => score.collect_all_attributes(),
            MdivChild::Parts(_) => Vec::new(), // Parts not yet fully implemented
        }
    }

    fn has_children(&self) -> bool {
        match self {
            MdivChild::Mdiv(mdiv) => mdiv.has_children(),
            MdivChild::Score(score) => score.has_children(),
            MdivChild::Parts(_) => true,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            MdivChild::Mdiv(mdiv) => mdiv.serialize_children(writer),
            MdivChild::Score(score) => score.serialize_children(writer),
            MdivChild::Parts(_) => Ok(()), // Parts not yet fully implemented
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
