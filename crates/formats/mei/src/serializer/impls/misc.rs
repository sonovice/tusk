//! Serializer implementations for miscellaneous MEI elements.
//!
//! This module contains implementations for common attribute classes,
//! grouping elements (Beam, Tuplet, GraceGrp), and other elements
//! not covered by specialized submodules.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::AttTabular;
use tusk_model::att::{
    AttAccidental, AttAuthorized, AttBasic, AttBeamAnl, AttBeamGes, AttBeamLog, AttBeamVis,
    AttCalendared, AttClassed, AttColor, AttCommon, AttComponentType, AttDataPointing, AttDatable,
    AttEdit, AttEvidence, AttExtSymAuth, AttFacsimile, AttFiling, AttFoliationScheme,
    AttGraceGrpAnl, AttGraceGrpGes, AttGraceGrpLog, AttGraceGrpVis, AttHorizontalAlign,
    AttInternetMedia, AttKeyMode, AttLabelled, AttLinking, AttLyricsAnl, AttLyricsGes,
    AttLyricsLog, AttLyricsVis, AttMeasurement, AttMeiVersion, AttMetadataPointing, AttMeterSigLog,
    AttNInteger, AttNNumberLike, AttName, AttPerfRes, AttPerfResBasic, AttPitch, AttPointing,
    AttRanging, AttRecordType, AttRegularMethod, AttResponsibility, AttSource, AttTargetEval,
    AttTextRendition, AttTupletAnl, AttTupletGes, AttTupletLog, AttTupletVis, AttTyped,
    AttTypography, AttVerticalAlign, AttWhitespace, AttXy,
};
use tusk_model::elements::{
    Beam, BeamChild, Caption, CaptionChild, GraceGrp, GraceGrpChild, L, LChild, Lg, LgChild, Li,
    LiChild, List, ListChild, Num, Ptr, Ref, Table, TableChild, Td, TdChild, Th, ThChild, Tr,
    TrChild, Tuplet, TupletChild,
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

impl CollectAttributes for AttPerfResBasic {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "adlib", self.adlib);
        if let Some(ref v) = self.count {
            attrs.push(("count", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttPerfRes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "adlib", self.adlib);
        if let Some(ref v) = self.count {
            attrs.push(("count", v.to_string()));
        }
        if let Some(ref v) = self.trans_diat {
            attrs.push(("trans.diat", v.to_string()));
        }
        if let Some(ref v) = self.trans_semi {
            attrs.push(("trans.semi", v.to_string()));
        }
        push_attr!(attrs, "solo", self.solo);
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

impl CollectAttributes for AttFoliationScheme {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "scheme", self.scheme);
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
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "BeamChild::{}",
                other.element_name()
            ))),
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
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "TupletChild::{}",
                other.element_name()
            ))),
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
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "GraceGrpChild::{}",
                other.element_name()
            ))),
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
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "MusicChild::{}::serialize_children",
                other.element_name()
            ))),
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

// ============================================================================
// List element implementation
// ============================================================================

impl MeiSerialize for List {
    fn element_name(&self) -> &'static str {
        "list"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.classed.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.linking.collect_attributes());
        attrs.extend(self.n_number_like.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
        attrs.extend(self.xy.collect_attributes());
        push_attr!(attrs, "form", clone self.form);
        push_attr!(attrs, "type", clone self.r#type);
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

impl MeiSerialize for ListChild {
    fn element_name(&self) -> &'static str {
        match self {
            ListChild::Head(_) => "head",
            ListChild::Li(_) => "li",
            ListChild::Label(_) => "label",
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
            ListChild::Head(elem) => elem.serialize_mei(writer),
            ListChild::Li(elem) => elem.serialize_mei(writer),
            ListChild::Label(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Li (list item) element implementation
// ============================================================================

impl MeiSerialize for Li {
    fn element_name(&self) -> &'static str {
        "li"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
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

impl MeiSerialize for LiChild {
    fn element_name(&self) -> &'static str {
        match self {
            LiChild::Text(_) => "#text",
            LiChild::District(_) => "district",
            LiChild::Country(_) => "country",
            LiChild::Signatures(_) => "signatures",
            LiChild::PostBox(_) => "postBox",
            LiChild::Abbr(_) => "abbr",
            LiChild::Ptr(_) => "ptr",
            LiChild::RelationList(_) => "relationList",
            LiChild::BiblStruct(_) => "biblStruct",
            LiChild::Width(_) => "width",
            LiChild::Symbol(_) => "symbol",
            LiChild::Add(_) => "add",
            LiChild::GeogFeat(_) => "geogFeat",
            LiChild::Num(_) => "num",
            LiChild::Rend(_) => "rend",
            LiChild::Restore(_) => "restore",
            LiChild::Street(_) => "street",
            LiChild::List(_) => "list",
            LiChild::Region(_) => "region",
            LiChild::BiblList(_) => "biblList",
            LiChild::SecFolio(_) => "secFolio",
            LiChild::Orig(_) => "orig",
            LiChild::Repository(_) => "repository",
            LiChild::Settlement(_) => "settlement",
            LiChild::Damage(_) => "damage",
            LiChild::Seg(_) => "seg",
            LiChild::Heraldry(_) => "heraldry",
            LiChild::Stamp(_) => "stamp",
            LiChild::Lb(_) => "lb",
            LiChild::PersName(_) => "persName",
            LiChild::Name(_) => "name",
            LiChild::Supplied(_) => "supplied",
            LiChild::CorpName(_) => "corpName",
            LiChild::LocusGrp(_) => "locusGrp",
            LiChild::Choice(_) => "choice",
            LiChild::PeriodName(_) => "periodName",
            LiChild::Table(_) => "table",
            LiChild::GeogName(_) => "geogName",
            LiChild::Lg(_) => "lg",
            LiChild::StyleName(_) => "styleName",
            LiChild::Quote(_) => "quote",
            LiChild::Gap(_) => "gap",
            LiChild::Date(_) => "date",
            LiChild::Corr(_) => "corr",
            LiChild::Bibl(_) => "bibl",
            LiChild::Pb(_) => "pb",
            LiChild::Catchwords(_) => "catchwords",
            LiChild::Relation(_) => "relation",
            LiChild::Reg(_) => "reg",
            LiChild::Sic(_) => "sic",
            LiChild::Fig(_) => "fig",
            LiChild::Bloc(_) => "bloc",
            LiChild::Title(_) => "title",
            LiChild::Del(_) => "del",
            LiChild::Depth(_) => "depth",
            LiChild::Subst(_) => "subst",
            LiChild::Unclear(_) => "unclear",
            LiChild::P(_) => "p",
            LiChild::EventList(_) => "eventList",
            LiChild::Term(_) => "term",
            LiChild::Extent(_) => "extent",
            LiChild::CastList(_) => "castList",
            LiChild::Annot(_) => "annot",
            LiChild::Height(_) => "height",
            LiChild::Q(_) => "q",
            LiChild::Dim(_) => "dim",
            LiChild::Address(_) => "address",
            LiChild::Dimensions(_) => "dimensions",
            LiChild::Identifier(_) => "identifier",
            LiChild::PostCode(_) => "postCode",
            LiChild::HandShift(_) => "handShift",
            LiChild::Stack(_) => "stack",
            LiChild::Locus(_) => "locus",
            LiChild::Ref(_) => "ref",
            LiChild::Expan(_) => "expan",
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
            LiChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            // Elements with existing serializers
            LiChild::Ref(elem) => elem.serialize_mei(writer),
            LiChild::Date(elem) => elem.serialize_mei(writer),
            LiChild::Address(elem) => elem.serialize_mei(writer),
            LiChild::PersName(elem) => elem.serialize_mei(writer),
            LiChild::CorpName(elem) => elem.serialize_mei(writer),
            LiChild::Name(elem) => elem.serialize_mei(writer),
            LiChild::Identifier(elem) => elem.serialize_mei(writer),
            LiChild::Lb(elem) => elem.serialize_mei(writer),
            LiChild::Rend(elem) => elem.serialize_mei(writer),
            LiChild::Title(elem) => elem.serialize_mei(writer),
            LiChild::Num(elem) => elem.serialize_mei(writer),
            LiChild::Ptr(elem) => elem.serialize_mei(writer),
            LiChild::Annot(elem) => elem.serialize_mei(writer),
            LiChild::Extent(elem) => elem.serialize_mei(writer),
            LiChild::Region(elem) => elem.serialize_mei(writer),
            LiChild::PostBox(elem) => elem.serialize_mei(writer),
            LiChild::PostCode(elem) => elem.serialize_mei(writer),
            LiChild::District(elem) => elem.serialize_mei(writer),
            LiChild::GeogName(elem) => elem.serialize_mei(writer),
            LiChild::GeogFeat(elem) => elem.serialize_mei(writer),
            LiChild::Country(elem) => elem.serialize_mei(writer),
            LiChild::Settlement(elem) => elem.serialize_mei(writer),
            LiChild::Street(elem) => elem.serialize_mei(writer),
            LiChild::Bloc(elem) => elem.serialize_mei(writer),
            LiChild::P(elem) => elem.serialize_mei(writer),
            LiChild::Bibl(elem) => elem.serialize_mei(writer),
            LiChild::List(elem) => elem.serialize_mei(writer),
            // Elements that need serializers - for now skip with warning
            _ => {
                // TODO: Implement serializers for remaining LiChild variants
                Ok(())
            }
        }
    }
}

// ============================================================================
// Lyrics attribute classes
// ============================================================================

impl CollectAttributes for AttLyricsAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // Empty attribute class
        Vec::new()
    }
}

impl CollectAttributes for AttLyricsGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // Empty attribute class
        Vec::new()
    }
}

impl CollectAttributes for AttLyricsLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "staff", vec self.staff);
        attrs
    }
}

impl CollectAttributes for AttLyricsVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "place", self.place);
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

// ============================================================================
// Lg (line group) element
// ============================================================================

impl MeiSerialize for Lg {
    fn element_name(&self) -> &'static str {
        "lg"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.xy.collect_attributes());
        attrs.extend(self.lyrics_anl.collect_attributes());
        attrs.extend(self.lyrics_ges.collect_attributes());
        attrs.extend(self.lyrics_log.collect_attributes());
        attrs.extend(self.lyrics_vis.collect_attributes());
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

impl MeiSerialize for LgChild {
    fn element_name(&self) -> &'static str {
        match self {
            LgChild::L(_) => "l",
            LgChild::Head(_) => "head",
            LgChild::Lg(_) => "lg",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            LgChild::L(elem) => elem.serialize_mei(writer),
            LgChild::Head(elem) => elem.serialize_mei(writer),
            LgChild::Lg(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// L (line) element
// ============================================================================

impl MeiSerialize for L {
    fn element_name(&self) -> &'static str {
        "l"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        // Element-local attribute: @rhythm
        push_attr!(attrs, "rhythm", clone self.rhythm);
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

impl MeiSerialize for LChild {
    fn element_name(&self) -> &'static str {
        match self {
            LChild::Text(_) => "#text",
            LChild::Del(_) => "del",
            LChild::Unclear(_) => "unclear",
            LChild::Identifier(_) => "identifier",
            LChild::BiblStruct(_) => "biblStruct",
            LChild::Expan(_) => "expan",
            LChild::Width(_) => "width",
            LChild::Bloc(_) => "bloc",
            LChild::Relation(_) => "relation",
            LChild::Symbol(_) => "symbol",
            LChild::PeriodName(_) => "periodName",
            LChild::Title(_) => "title",
            LChild::Num(_) => "num",
            LChild::Abbr(_) => "abbr",
            LChild::Ptr(_) => "ptr",
            LChild::Rend(_) => "rend",
            LChild::Date(_) => "date",
            LChild::StyleName(_) => "styleName",
            LChild::LocusGrp(_) => "locusGrp",
            LChild::Subst(_) => "subst",
            LChild::Signatures(_) => "signatures",
            LChild::District(_) => "district",
            LChild::Orig(_) => "orig",
            LChild::Lb(_) => "lb",
            LChild::Catchwords(_) => "catchwords",
            LChild::Q(_) => "q",
            LChild::Repository(_) => "repository",
            LChild::CorpName(_) => "corpName",
            LChild::GeogName(_) => "geogName",
            LChild::Choice(_) => "choice",
            LChild::Bibl(_) => "bibl",
            LChild::Fig(_) => "fig",
            LChild::Stamp(_) => "stamp",
            LChild::Heraldry(_) => "heraldry",
            LChild::Country(_) => "country",
            LChild::Depth(_) => "depth",
            LChild::Corr(_) => "corr",
            LChild::Dim(_) => "dim",
            LChild::Gap(_) => "gap",
            LChild::Syl(_) => "syl",
            LChild::GeogFeat(_) => "geogFeat",
            LChild::Reg(_) => "reg",
            LChild::PersName(_) => "persName",
            LChild::Seg(_) => "seg",
            LChild::Region(_) => "region",
            LChild::Sic(_) => "sic",
            LChild::Extent(_) => "extent",
            LChild::Ref(_) => "ref",
            LChild::Locus(_) => "locus",
            LChild::Address(_) => "address",
            LChild::Pb(_) => "pb",
            LChild::Name(_) => "name",
            LChild::Settlement(_) => "settlement",
            LChild::Add(_) => "add",
            LChild::Height(_) => "height",
            LChild::Street(_) => "street",
            LChild::RelationList(_) => "relationList",
            LChild::Annot(_) => "annot",
            LChild::HandShift(_) => "handShift",
            LChild::PostCode(_) => "postCode",
            LChild::PostBox(_) => "postBox",
            LChild::Damage(_) => "damage",
            LChild::SecFolio(_) => "secFolio",
            LChild::Stack(_) => "stack",
            LChild::Restore(_) => "restore",
            LChild::Supplied(_) => "supplied",
            LChild::Dimensions(_) => "dimensions",
            LChild::Term(_) => "term",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, LChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            LChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            // Elements with existing serializers
            LChild::Ref(elem) => elem.serialize_mei(writer),
            LChild::Date(elem) => elem.serialize_mei(writer),
            LChild::PersName(elem) => elem.serialize_mei(writer),
            LChild::CorpName(elem) => elem.serialize_mei(writer),
            LChild::Name(elem) => elem.serialize_mei(writer),
            LChild::GeogName(elem) => elem.serialize_mei(writer),
            LChild::Identifier(elem) => elem.serialize_mei(writer),
            LChild::Title(elem) => elem.serialize_mei(writer),
            LChild::Bibl(elem) => elem.serialize_mei(writer),
            LChild::Rend(elem) => elem.serialize_mei(writer),
            LChild::Num(elem) => elem.serialize_mei(writer),
            LChild::Ptr(elem) => elem.serialize_mei(writer),
            LChild::Lb(elem) => elem.serialize_mei(writer),
            LChild::Annot(elem) => elem.serialize_mei(writer),
            LChild::Extent(elem) => elem.serialize_mei(writer),
            LChild::Address(elem) => elem.serialize_mei(writer),
            LChild::PostBox(elem) => elem.serialize_mei(writer),
            LChild::PostCode(elem) => elem.serialize_mei(writer),
            LChild::Street(elem) => elem.serialize_mei(writer),
            LChild::District(elem) => elem.serialize_mei(writer),
            LChild::Region(elem) => elem.serialize_mei(writer),
            LChild::Country(elem) => elem.serialize_mei(writer),
            LChild::Settlement(elem) => elem.serialize_mei(writer),
            LChild::GeogFeat(elem) => elem.serialize_mei(writer),
            LChild::Bloc(elem) => elem.serialize_mei(writer),
            // Other elements that need their own serializers - skip for now
            _ => {
                // TODO: Implement serializers for remaining LChild variants
                Ok(())
            }
        }
    }
}

// ============================================================================
// Div element implementation
// ============================================================================

use tusk_model::elements::{Div, DivChild};

impl MeiSerialize for Div {
    fn element_name(&self) -> &'static str {
        "div"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.classed.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.linking.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.n_number_like.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
        // Element-local attribute
        if let Some(t) = &self.r#type {
            attrs.push(("type", t.clone()));
        }
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

impl MeiSerialize for DivChild {
    fn element_name(&self) -> &'static str {
        match self {
            DivChild::P(_) => "p",
            DivChild::Lg(_) => "lg",
            DivChild::List(_) => "list",
            DivChild::Head(_) => "head",
            DivChild::Quote(_) => "quote",
            DivChild::Table(_) => "table",
            DivChild::Div(_) => "div",
            DivChild::Fig(_) => "fig",
            DivChild::Pb(_) => "pb",
            DivChild::Lb(_) => "lb",
            DivChild::Cb(_) => "cb",
            DivChild::ColLayout(_) => "colLayout",
            DivChild::Sp(_) => "sp",
            DivChild::BiblList(_) => "biblList",
            DivChild::CastList(_) => "castList",
            DivChild::EventList(_) => "eventList",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            DivChild::P(p) => p.collect_all_attributes(),
            DivChild::Lg(lg) => lg.collect_all_attributes(),
            DivChild::List(list) => list.collect_all_attributes(),
            DivChild::Head(head) => head.collect_all_attributes(),
            DivChild::Div(div) => div.collect_all_attributes(),
            // Other child types not yet implemented - return empty
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            DivChild::P(p) => p.has_children(),
            DivChild::Lg(lg) => lg.has_children(),
            DivChild::List(list) => list.has_children(),
            DivChild::Head(head) => head.has_children(),
            DivChild::Div(div) => div.has_children(),
            // Other child types - assume no children for now
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            DivChild::P(p) => p.serialize_children(writer),
            DivChild::Lg(lg) => lg.serialize_children(writer),
            DivChild::List(list) => list.serialize_children(writer),
            DivChild::Head(head) => head.serialize_children(writer),
            DivChild::Div(div) => div.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "DivChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// AttTabular attribute class implementation
// ============================================================================

impl CollectAttributes for AttTabular {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "colspan", self.colspan);
        push_attr!(attrs, "rowspan", self.rowspan);
        attrs
    }
}

// ============================================================================
// Table element implementations
// ============================================================================

impl MeiSerialize for Table {
    fn element_name(&self) -> &'static str {
        "table"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.xy.collect_attributes());
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

impl MeiSerialize for TableChild {
    fn element_name(&self) -> &'static str {
        match self {
            TableChild::Caption(_) => "caption",
            TableChild::Tr(_) => "tr",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            TableChild::Caption(c) => c.collect_all_attributes(),
            TableChild::Tr(tr) => tr.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            TableChild::Caption(c) => c.has_children(),
            TableChild::Tr(tr) => tr.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            TableChild::Caption(c) => c.serialize_children(writer),
            TableChild::Tr(tr) => tr.serialize_children(writer),
        }
    }
}

impl MeiSerialize for Tr {
    fn element_name(&self) -> &'static str {
        "tr"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.xy.collect_attributes());
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

impl MeiSerialize for TrChild {
    fn element_name(&self) -> &'static str {
        match self {
            TrChild::Td(_) => "td",
            TrChild::Th(_) => "th",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            TrChild::Td(td) => td.collect_all_attributes(),
            TrChild::Th(th) => th.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            TrChild::Td(td) => td.has_children(),
            TrChild::Th(th) => th.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            TrChild::Td(td) => td.serialize_children(writer),
            TrChild::Th(th) => th.serialize_children(writer),
        }
    }
}

impl MeiSerialize for Td {
    fn element_name(&self) -> &'static str {
        "td"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.xy.collect_attributes());
        attrs.extend(self.tabular.collect_attributes());
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

impl MeiSerialize for TdChild {
    fn element_name(&self) -> &'static str {
        match self {
            TdChild::Text(_) => "#text",
            TdChild::Rend(_) => "rend",
            TdChild::Lb(_) => "lb",
            TdChild::PersName(_) => "persName",
            TdChild::CorpName(_) => "corpName",
            TdChild::Name(_) => "name",
            TdChild::Title(_) => "title",
            TdChild::Date(_) => "date",
            TdChild::Ref(_) => "ref",
            TdChild::Ptr(_) => "ptr",
            TdChild::Identifier(_) => "identifier",
            TdChild::Seg(_) => "seg",
            TdChild::P(_) => "p",
            TdChild::List(_) => "list",
            TdChild::Table(_) => "table",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            TdChild::Rend(r) => r.collect_all_attributes(),
            TdChild::Lb(lb) => lb.collect_all_attributes(),
            TdChild::PersName(pn) => pn.collect_all_attributes(),
            TdChild::CorpName(cn) => cn.collect_all_attributes(),
            TdChild::Name(n) => n.collect_all_attributes(),
            TdChild::Title(t) => t.collect_all_attributes(),
            TdChild::Date(d) => d.collect_all_attributes(),
            TdChild::Ref(r) => r.collect_all_attributes(),
            TdChild::Ptr(p) => p.collect_all_attributes(),
            TdChild::Identifier(i) => i.collect_all_attributes(),
            TdChild::Seg(s) => s.collect_all_attributes(),
            TdChild::P(p) => p.collect_all_attributes(),
            TdChild::List(l) => l.collect_all_attributes(),
            TdChild::Table(t) => t.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            TdChild::Text(_) => false,
            TdChild::Rend(r) => r.has_children(),
            TdChild::Lb(_) => false,
            TdChild::PersName(pn) => pn.has_children(),
            TdChild::CorpName(cn) => cn.has_children(),
            TdChild::Name(n) => n.has_children(),
            TdChild::Title(t) => t.has_children(),
            TdChild::Date(d) => d.has_children(),
            TdChild::Ref(r) => r.has_children(),
            TdChild::Ptr(_) => false,
            TdChild::Identifier(i) => i.has_children(),
            TdChild::Seg(s) => s.has_children(),
            TdChild::P(p) => p.has_children(),
            TdChild::List(l) => l.has_children(),
            TdChild::Table(t) => t.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            TdChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            TdChild::Rend(r) => r.serialize_children(writer),
            TdChild::Lb(_) => Ok(()),
            TdChild::PersName(pn) => pn.serialize_children(writer),
            TdChild::CorpName(cn) => cn.serialize_children(writer),
            TdChild::Name(n) => n.serialize_children(writer),
            TdChild::Title(t) => t.serialize_children(writer),
            TdChild::Date(d) => d.serialize_children(writer),
            TdChild::Ref(r) => r.serialize_children(writer),
            TdChild::Ptr(_) => Ok(()),
            TdChild::Identifier(i) => i.serialize_children(writer),
            TdChild::Seg(s) => s.serialize_children(writer),
            TdChild::P(p) => p.serialize_children(writer),
            TdChild::List(l) => l.serialize_children(writer),
            TdChild::Table(t) => t.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "TdChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

impl MeiSerialize for Th {
    fn element_name(&self) -> &'static str {
        "th"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.xy.collect_attributes());
        attrs.extend(self.tabular.collect_attributes());
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

impl MeiSerialize for ThChild {
    fn element_name(&self) -> &'static str {
        match self {
            ThChild::Text(_) => "#text",
            ThChild::Rend(_) => "rend",
            ThChild::Lb(_) => "lb",
            ThChild::PersName(_) => "persName",
            ThChild::CorpName(_) => "corpName",
            ThChild::Name(_) => "name",
            ThChild::Title(_) => "title",
            ThChild::Date(_) => "date",
            ThChild::Ref(_) => "ref",
            ThChild::Ptr(_) => "ptr",
            ThChild::Identifier(_) => "identifier",
            ThChild::Seg(_) => "seg",
            ThChild::P(_) => "p",
            ThChild::List(_) => "list",
            ThChild::Table(_) => "table",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            ThChild::Rend(r) => r.collect_all_attributes(),
            ThChild::Lb(lb) => lb.collect_all_attributes(),
            ThChild::PersName(pn) => pn.collect_all_attributes(),
            ThChild::CorpName(cn) => cn.collect_all_attributes(),
            ThChild::Name(n) => n.collect_all_attributes(),
            ThChild::Title(t) => t.collect_all_attributes(),
            ThChild::Date(d) => d.collect_all_attributes(),
            ThChild::Ref(r) => r.collect_all_attributes(),
            ThChild::Ptr(p) => p.collect_all_attributes(),
            ThChild::Identifier(i) => i.collect_all_attributes(),
            ThChild::Seg(s) => s.collect_all_attributes(),
            ThChild::P(p) => p.collect_all_attributes(),
            ThChild::List(l) => l.collect_all_attributes(),
            ThChild::Table(t) => t.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            ThChild::Text(_) => false,
            ThChild::Rend(r) => r.has_children(),
            ThChild::Lb(_) => false,
            ThChild::PersName(pn) => pn.has_children(),
            ThChild::CorpName(cn) => cn.has_children(),
            ThChild::Name(n) => n.has_children(),
            ThChild::Title(t) => t.has_children(),
            ThChild::Date(d) => d.has_children(),
            ThChild::Ref(r) => r.has_children(),
            ThChild::Ptr(_) => false,
            ThChild::Identifier(i) => i.has_children(),
            ThChild::Seg(s) => s.has_children(),
            ThChild::P(p) => p.has_children(),
            ThChild::List(l) => l.has_children(),
            ThChild::Table(t) => t.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ThChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            ThChild::Rend(r) => r.serialize_children(writer),
            ThChild::Lb(_) => Ok(()),
            ThChild::PersName(pn) => pn.serialize_children(writer),
            ThChild::CorpName(cn) => cn.serialize_children(writer),
            ThChild::Name(n) => n.serialize_children(writer),
            ThChild::Title(t) => t.serialize_children(writer),
            ThChild::Date(d) => d.serialize_children(writer),
            ThChild::Ref(r) => r.serialize_children(writer),
            ThChild::Ptr(_) => Ok(()),
            ThChild::Identifier(i) => i.serialize_children(writer),
            ThChild::Seg(s) => s.serialize_children(writer),
            ThChild::P(p) => p.serialize_children(writer),
            ThChild::List(l) => l.serialize_children(writer),
            ThChild::Table(t) => t.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "ThChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

impl MeiSerialize for Caption {
    fn element_name(&self) -> &'static str {
        "caption"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
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

impl MeiSerialize for CaptionChild {
    fn element_name(&self) -> &'static str {
        match self {
            CaptionChild::Text(_) => "#text",
            CaptionChild::Rend(_) => "rend",
            CaptionChild::Lb(_) => "lb",
            CaptionChild::PersName(_) => "persName",
            CaptionChild::CorpName(_) => "corpName",
            CaptionChild::Name(_) => "name",
            CaptionChild::Title(_) => "title",
            CaptionChild::Date(_) => "date",
            CaptionChild::Ref(_) => "ref",
            CaptionChild::Ptr(_) => "ptr",
            CaptionChild::Identifier(_) => "identifier",
            CaptionChild::Seg(_) => "seg",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            CaptionChild::Rend(r) => r.collect_all_attributes(),
            CaptionChild::Lb(lb) => lb.collect_all_attributes(),
            CaptionChild::PersName(pn) => pn.collect_all_attributes(),
            CaptionChild::CorpName(cn) => cn.collect_all_attributes(),
            CaptionChild::Name(n) => n.collect_all_attributes(),
            CaptionChild::Title(t) => t.collect_all_attributes(),
            CaptionChild::Date(d) => d.collect_all_attributes(),
            CaptionChild::Ref(r) => r.collect_all_attributes(),
            CaptionChild::Ptr(p) => p.collect_all_attributes(),
            CaptionChild::Identifier(i) => i.collect_all_attributes(),
            CaptionChild::Seg(s) => s.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            CaptionChild::Text(_) => false,
            CaptionChild::Rend(r) => r.has_children(),
            CaptionChild::Lb(_) => false,
            CaptionChild::PersName(pn) => pn.has_children(),
            CaptionChild::CorpName(cn) => cn.has_children(),
            CaptionChild::Name(n) => n.has_children(),
            CaptionChild::Title(t) => t.has_children(),
            CaptionChild::Date(d) => d.has_children(),
            CaptionChild::Ref(r) => r.has_children(),
            CaptionChild::Ptr(_) => false,
            CaptionChild::Identifier(i) => i.has_children(),
            CaptionChild::Seg(s) => s.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            CaptionChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            CaptionChild::Rend(r) => r.serialize_children(writer),
            CaptionChild::Lb(_) => Ok(()),
            CaptionChild::PersName(pn) => pn.serialize_children(writer),
            CaptionChild::CorpName(cn) => cn.serialize_children(writer),
            CaptionChild::Name(n) => n.serialize_children(writer),
            CaptionChild::Title(t) => t.serialize_children(writer),
            CaptionChild::Date(d) => d.serialize_children(writer),
            CaptionChild::Ref(r) => r.serialize_children(writer),
            CaptionChild::Ptr(_) => Ok(()),
            CaptionChild::Identifier(i) => i.serialize_children(writer),
            CaptionChild::Seg(s) => s.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "CaptionChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}
