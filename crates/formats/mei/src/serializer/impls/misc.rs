//! Serializer implementations for miscellaneous MEI elements.
//!
//! This module contains implementations for common attribute classes,
//! grouping elements (Beam, Tuplet, GraceGrp), and other elements
//! not covered by specialized submodules.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttAccidental, AttAuthorized, AttBasic, AttBeamAnl, AttBeamGes, AttBeamLog, AttBeamVis,
    AttCalendared, AttClassed, AttColor, AttCommon, AttComponentType, AttDataPointing, AttDatable,
    AttEdit, AttEvidence, AttExtSymAuth, AttFacsimile, AttFiling, AttGraceGrpAnl, AttGraceGrpGes,
    AttGraceGrpLog, AttGraceGrpVis, AttHorizontalAlign, AttInternetMedia, AttKeyMode, AttLabelled,
    AttLinking, AttMeasurement, AttMeiVersion, AttMetadataPointing, AttMeterSigLog, AttNInteger,
    AttNNumberLike, AttName, AttPitch, AttPointing, AttRanging, AttRecordType, AttRegularMethod,
    AttResponsibility, AttSource, AttTargetEval, AttTextRendition, AttTupletAnl, AttTupletGes,
    AttTupletLog, AttTupletVis, AttTyped, AttTypography, AttVerticalAlign, AttWhitespace, AttXy,
};
use tusk_model::elements::{
    Beam, BeamChild, GraceGrp, GraceGrpChild, Num, Ptr, Ref, Tuplet, TupletChild,
};

use super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Common attribute class implementations
// ============================================================================

impl CollectAttributes for AttCommon {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();

        // xml:id should come first by convention
        push_attr!(attrs, "xml:id", clone self.xml_id);
        push_attr!(attrs, "xml:base", self.xml_base);
        push_attr!(attrs, "label", clone self.label);
        push_attr!(attrs, "copyof", self.copyof);
        push_attr!(attrs, "corresp", vec self.corresp);
        push_attr!(attrs, "follows", vec self.follows);
        push_attr!(attrs, "next", vec self.next);
        push_attr!(attrs, "precedes", vec self.precedes);
        push_attr!(attrs, "prev", vec self.prev);
        push_attr!(attrs, "sameas", vec self.sameas);
        push_attr!(attrs, "synch", vec self.synch);
        push_attr!(attrs, "n", self.n);
        push_attr!(attrs, "resp", vec self.resp);
        push_attr!(attrs, "class", vec self.class);
        push_attr!(attrs, "type", vec self.r#type);

        attrs
    }
}

impl CollectAttributes for AttFacsimile {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "facs", vec self.facs);
        attrs
    }
}

impl CollectAttributes for AttMetadataPointing {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "decls", vec self.decls);
        attrs
    }
}

impl CollectAttributes for AttPointing {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "xlink:actuate", self.xlink_actuate);
        push_attr!(attrs, "xlink:role", self.xlink_role);
        push_attr!(attrs, "xlink:show", self.xlink_show);
        push_attr!(attrs, "target", vec self.target);
        push_attr!(attrs, "targettype", clone self.targettype);
        attrs
    }
}

impl CollectAttributes for AttTargetEval {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "evaluate", self.evaluate);
        attrs
    }
}

// ============================================================================
// Staff attribute class implementations
// ============================================================================

impl CollectAttributes for AttBasic {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "xml:id", clone self.xml_id);
        push_attr!(attrs, "xml:base", self.xml_base);
        attrs
    }
}

impl CollectAttributes for AttLabelled {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "label", clone self.label);
        attrs
    }
}

impl CollectAttributes for AttLinking {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "copyof", self.copyof);
        push_attr!(attrs, "corresp", vec self.corresp);
        push_attr!(attrs, "follows", vec self.follows);
        push_attr!(attrs, "next", vec self.next);
        push_attr!(attrs, "precedes", vec self.precedes);
        push_attr!(attrs, "prev", vec self.prev);
        push_attr!(attrs, "sameas", vec self.sameas);
        push_attr!(attrs, "synch", vec self.synch);
        attrs
    }
}

impl CollectAttributes for AttNInteger {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "n", self.n);
        attrs
    }
}

impl CollectAttributes for AttResponsibility {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "resp", vec self.resp);
        attrs
    }
}

impl CollectAttributes for AttTyped {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "class", vec self.class);
        push_attr!(attrs, "type", vec self.r#type);
        attrs
    }
}

// ============================================================================
// Grouping element attribute class implementations
// ============================================================================

// Beam attribute classes
impl CollectAttributes for AttBeamLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "beam.with", self.beam_with);
        attrs
    }
}

impl CollectAttributes for AttBeamVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "cue", self.cue);
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "slash", self.slash);
        if let Some(v) = &self.slope {
            attrs.push(("slope", v.to_string()));
        }
        push_attr!(attrs, "visible", self.visible);
        attrs
    }
}

impl CollectAttributes for AttBeamGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttBeamAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// Tuplet attribute classes
impl CollectAttributes for AttTupletLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam.with", self.beam_with);
        push_attr!(attrs, "dur", vec self.dur);
        if let Some(v) = &self.num {
            attrs.push(("num", v.to_string()));
        }
        if let Some(v) = &self.numbase {
            attrs.push(("numbase", v.to_string()));
        }
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        attrs
    }
}

impl CollectAttributes for AttTupletVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "num.place", self.num_place);
        push_attr!(attrs, "num.visible", self.num_visible);
        push_attr!(attrs, "bracket.place", self.bracket_place);
        push_attr!(attrs, "bracket.visible", self.bracket_visible);
        push_attr!(attrs, "num.format", self.num_format);
        attrs
    }
}

impl CollectAttributes for AttTupletGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        if let Some(v) = &self.dur_metrical {
            attrs.push(("dur.metrical", v.to_string()));
        }
        if let Some(v) = &self.dur_ppq {
            attrs.push(("dur.ppq", v.to_string()));
        }
        if let Some(v) = &self.dur_real {
            attrs.push(("dur.real", v.to_string()));
        }
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        attrs
    }
}

impl CollectAttributes for AttTupletAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// GraceGrp attribute classes
impl CollectAttributes for AttGraceGrpLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "grace", self.grace);
        push_attr!(attrs, "grace.time", self.grace_time);
        push_attr!(attrs, "attach", self.attach);
        attrs
    }
}

impl CollectAttributes for AttGraceGrpVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        attrs
    }
}

impl CollectAttributes for AttGraceGrpGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttGraceGrpAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Header-related attribute class implementations
// ============================================================================

impl CollectAttributes for AttDatable {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "enddate", self.enddate);
        push_attr!(attrs, "isodate", self.isodate);
        push_attr!(attrs, "notafter", self.notafter);
        push_attr!(attrs, "notbefore", self.notbefore);
        push_attr!(attrs, "startdate", self.startdate);
        attrs
    }
}

impl CollectAttributes for AttMeiVersion {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "meiversion", self.meiversion);
        attrs
    }
}

impl CollectAttributes for AttCalendared {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "calendar", clone self.calendar);
        attrs
    }
}

impl CollectAttributes for AttAuthorized {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "codedval", vec self.codedval);
        push_attr!(attrs, "auth", clone self.auth);
        push_attr!(attrs, "auth.uri", self.auth_uri);
        attrs
    }
}

impl CollectAttributes for AttClassed {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "class", vec self.class);
        attrs
    }
}

impl CollectAttributes for AttFiling {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref v) = self.nonfiling {
            attrs.push(("nonfiling", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttNNumberLike {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "n", self.n);
        attrs
    }
}

impl CollectAttributes for AttEdit {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "source", vec self.source);
        push_attr!(attrs, "cert", self.cert);
        push_attr!(attrs, "evidence", self.evidence);
        attrs
    }
}

impl CollectAttributes for AttXy {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref v) = self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(ref v) = self.y {
            attrs.push(("y", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttEvidence {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "cert", self.cert);
        push_attr!(attrs, "evidence", self.evidence);
        attrs
    }
}

impl CollectAttributes for AttName {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "codedval", vec self.codedval);
        push_attr!(attrs, "auth", clone self.auth);
        push_attr!(attrs, "auth.uri", self.auth_uri);
        push_attr!(attrs, "enddate", self.enddate);
        push_attr!(attrs, "isodate", self.isodate);
        push_attr!(attrs, "notafter", self.notafter);
        push_attr!(attrs, "notbefore", self.notbefore);
        push_attr!(attrs, "startdate", self.startdate);
        if let Some(ref v) = self.nonfiling {
            attrs.push(("nonfiling", v.to_string()));
        }
        push_attr!(attrs, "nymref", self.nymref);
        push_attr!(attrs, "role", vec self.role);
        attrs
    }
}

impl CollectAttributes for AttDataPointing {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "data", vec self.data);
        attrs
    }
}

// ============================================================================
// Rend-related attribute class implementations
// ============================================================================

impl CollectAttributes for AttSource {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "source", vec self.source);
        attrs
    }
}

impl CollectAttributes for AttColor {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        attrs
    }
}

impl CollectAttributes for AttExtSymAuth {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        attrs
    }
}

impl CollectAttributes for AttHorizontalAlign {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "halign", self.halign);
        attrs
    }
}

impl CollectAttributes for AttTextRendition {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altrend", vec self.altrend);
        push_attr!(attrs, "rend", vec self.rend);
        attrs
    }
}

impl CollectAttributes for AttTypography {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        attrs
    }
}

impl CollectAttributes for AttVerticalAlign {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "valign", self.valign);
        attrs
    }
}

impl CollectAttributes for AttWhitespace {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "xml:space", self.xml_space);
        attrs
    }
}

impl CollectAttributes for AttComponentType {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "comptype", self.comptype);
        attrs
    }
}

impl CollectAttributes for AttRecordType {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "recordtype", self.recordtype);
        attrs
    }
}

impl CollectAttributes for AttRegularMethod {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "method", self.method);
        attrs
    }
}

// ============================================================================
// Grouping element implementations
// ============================================================================

impl MeiSerialize for Beam {
    fn element_name(&self) -> &'static str {
        "beam"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.beam_log.collect_attributes());
        attrs.extend(self.beam_vis.collect_attributes());
        attrs.extend(self.beam_ges.collect_attributes());
        attrs.extend(self.beam_anl.collect_attributes());
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

impl MeiSerialize for BeamChild {
    fn element_name(&self) -> &'static str {
        match self {
            BeamChild::Note(_) => "note",
            BeamChild::Rest(_) => "rest",
            BeamChild::Chord(_) => "chord",
            BeamChild::Space(_) => "space",
            BeamChild::Beam(_) => "beam",
            BeamChild::Tuplet(_) => "tuplet",
            BeamChild::GraceGrp(_) => "graceGrp",
            _ => "unknown", // Other children not yet implemented
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new() // Handled by recursive serialization
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            BeamChild::Note(note) => note.serialize_mei(writer),
            BeamChild::Rest(rest) => rest.serialize_mei(writer),
            BeamChild::Chord(chord) => chord.serialize_mei(writer),
            BeamChild::Space(space) => space.serialize_mei(writer),
            BeamChild::Beam(beam) => beam.serialize_mei(writer),
            BeamChild::Tuplet(tuplet) => tuplet.serialize_mei(writer),
            BeamChild::GraceGrp(grace_grp) => grace_grp.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for Tuplet {
    fn element_name(&self) -> &'static str {
        "tuplet"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.tuplet_log.collect_attributes());
        attrs.extend(self.tuplet_vis.collect_attributes());
        attrs.extend(self.tuplet_ges.collect_attributes());
        attrs.extend(self.tuplet_anl.collect_attributes());
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

impl MeiSerialize for TupletChild {
    fn element_name(&self) -> &'static str {
        match self {
            TupletChild::Note(_) => "note",
            TupletChild::Rest(_) => "rest",
            TupletChild::Chord(_) => "chord",
            TupletChild::Space(_) => "space",
            TupletChild::Beam(_) => "beam",
            TupletChild::Tuplet(_) => "tuplet",
            TupletChild::GraceGrp(_) => "graceGrp",
            _ => "unknown", // Other children not yet implemented
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new() // Handled by recursive serialization
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            TupletChild::Note(note) => note.serialize_mei(writer),
            TupletChild::Rest(rest) => rest.serialize_mei(writer),
            TupletChild::Chord(chord) => chord.serialize_mei(writer),
            TupletChild::Space(space) => space.serialize_mei(writer),
            TupletChild::Beam(beam) => beam.serialize_mei(writer),
            TupletChild::Tuplet(tuplet) => tuplet.serialize_mei(writer),
            TupletChild::GraceGrp(grace_grp) => grace_grp.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for GraceGrp {
    fn element_name(&self) -> &'static str {
        "graceGrp"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.grace_grp_log.collect_attributes());
        attrs.extend(self.grace_grp_vis.collect_attributes());
        attrs.extend(self.grace_grp_ges.collect_attributes());
        attrs.extend(self.grace_grp_anl.collect_attributes());
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

impl MeiSerialize for GraceGrpChild {
    fn element_name(&self) -> &'static str {
        match self {
            GraceGrpChild::Note(_) => "note",
            GraceGrpChild::Rest(_) => "rest",
            GraceGrpChild::Chord(_) => "chord",
            GraceGrpChild::Space(_) => "space",
            GraceGrpChild::Beam(_) => "beam",
            GraceGrpChild::Tuplet(_) => "tuplet",
            GraceGrpChild::GraceGrp(_) => "graceGrp",
            _ => "unknown", // Other children not yet implemented
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new() // Handled by recursive serialization
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            GraceGrpChild::Note(note) => note.serialize_mei(writer),
            GraceGrpChild::Rest(rest) => rest.serialize_mei(writer),
            GraceGrpChild::Chord(chord) => chord.serialize_mei(writer),
            GraceGrpChild::Space(space) => space.serialize_mei(writer),
            GraceGrpChild::Beam(beam) => beam.serialize_mei(writer),
            GraceGrpChild::Tuplet(tuplet) => tuplet.serialize_mei(writer),
            GraceGrpChild::GraceGrp(grace_grp) => grace_grp.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

// ============================================================================
// Mei (root element) implementation
// ============================================================================

impl MeiSerialize for tusk_model::elements::Mei {
    fn element_name(&self) -> &'static str {
        "mei"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        // AttId: xml:id
        push_attr!(attrs, "xml:id", clone self.id.xml_id);
        // AttMeiVersion: meiversion
        attrs.extend(self.mei_version.collect_attributes());
        // AttResponsibility: resp
        attrs.extend(self.responsibility.collect_attributes());
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

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        let name = self.element_name();
        let attrs = self.collect_all_attributes();

        let mut start = writer.start_element(name)?;

        // Add namespace declarations for root element
        writer.add_root_namespaces(&mut start);

        for (attr_name, value) in attrs {
            start.push_attribute((attr_name, value.as_str()));
        }

        if self.has_children() {
            writer.write_start(start)?;
            self.serialize_children(writer)?;
            writer.write_end(name)?;
        } else {
            writer.write_empty(start)?;
        }

        Ok(())
    }
}

impl MeiSerialize for tusk_model::elements::MeiChild {
    fn element_name(&self) -> &'static str {
        match self {
            tusk_model::elements::MeiChild::MeiHead(_) => "meiHead",
            tusk_model::elements::MeiChild::Music(_) => "music",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new() // Handled by recursive serialization
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            tusk_model::elements::MeiChild::MeiHead(mei_head) => mei_head.serialize_mei(writer),
            tusk_model::elements::MeiChild::Music(music) => music.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Music element implementation
// ============================================================================

impl MeiSerialize for tusk_model::elements::Music {
    fn element_name(&self) -> &'static str {
        "music"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.mei_version.collect_attributes());
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

impl MeiSerialize for tusk_model::elements::MusicChild {
    fn element_name(&self) -> &'static str {
        match self {
            tusk_model::elements::MusicChild::Body(_) => "body",
            tusk_model::elements::MusicChild::Group(_) => "group",
            tusk_model::elements::MusicChild::Front(_) => "front",
            tusk_model::elements::MusicChild::Back(_) => "back",
            tusk_model::elements::MusicChild::Facsimile(_) => "facsimile",
            tusk_model::elements::MusicChild::GenDesc(_) => "genDesc",
            tusk_model::elements::MusicChild::Performance(_) => "performance",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            tusk_model::elements::MusicChild::Body(body) => body.collect_all_attributes(),
            // Other children not fully implemented yet
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            tusk_model::elements::MusicChild::Body(body) => body.has_children(),
            _ => true,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            tusk_model::elements::MusicChild::Body(body) => body.serialize_children(writer),
            // Other children not fully implemented yet
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Key/Meter/Pitch attribute class implementations
// ============================================================================

impl CollectAttributes for AttAccidental {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "accid", self.accid);
        attrs
    }
}

impl CollectAttributes for AttPitch {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "pname", self.pname);
        attrs
    }
}

impl CollectAttributes for AttKeyMode {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "mode", self.mode);
        attrs
    }
}

impl CollectAttributes for AttMeterSigLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "count", clone self.count);
        push_attr!(attrs, "sym", self.sym);
        if let Some(ref v) = self.unit {
            attrs.push(("unit", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttInternetMedia {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "mimetype", clone self.mimetype);
        attrs
    }
}

impl CollectAttributes for AttMeasurement {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "unit", self.unit);
        attrs
    }
}

impl CollectAttributes for AttRanging {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref v) = self.atleast {
            attrs.push(("atleast", v.to_string()));
        }
        if let Some(ref v) = self.atmost {
            attrs.push(("atmost", v.to_string()));
        }
        if let Some(ref v) = self.min {
            attrs.push(("min", v.to_string()));
        }
        if let Some(ref v) = self.max {
            attrs.push(("max", v.to_string()));
        }
        push_attr!(attrs, "confidence", self.confidence);
        attrs
    }
}

// ============================================================================
// Num element implementation
// ============================================================================

impl MeiSerialize for Num {
    fn element_name(&self) -> &'static str {
        "num"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.measurement.collect_attributes());
        attrs.extend(self.ranging.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                tusk_model::elements::NumChild::Text(text) => writer.write_text(text)?,
                // Other children delegate to their own serialize_mei
                _ => {
                    // Most child types need their own serializer implementations
                    // For now, skip unimplemented children
                }
            }
        }
        Ok(())
    }
}

// ============================================================================
// Ref element implementation
// ============================================================================

impl MeiSerialize for Ref {
    fn element_name(&self) -> &'static str {
        "ref"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.internet_media.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                tusk_model::elements::RefChild::Text(text) => writer.write_text(text)?,
                // Other children delegate to their own serialize_mei
                _ => {
                    // Most child types need their own serializer implementations
                    // For now, skip unimplemented children
                }
            }
        }
        Ok(())
    }
}

// ============================================================================
// Ptr element implementation
// ============================================================================

impl MeiSerialize for Ptr {
    fn element_name(&self) -> &'static str {
        "ptr"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.internet_media.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Ptr is an empty element
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}
