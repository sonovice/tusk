//! Serializer implementations for core CMN (Common Music Notation) elements.
//!
//! This module contains implementations for structural and utility elements:
//! - BarLine: Explicit barline element
//! - Stem: Explicit stem element
//! - ClefGrp: Grouped clefs
//! - Custos: End-of-line pitch indicator
//! - Pad: Horizontal spacing element
//! - KeyAccid: Accidental in a key signature
//! - MeterSigGrp: Non-standard meter signatures
//! - GrpSym: Brace/bracket for staff grouping
//! - Part/Parts: Part elements for performer views
//! - Ossia: Alternative notation passages
//! - HarpPedal: Harp pedal diagram

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttBarLineAnl, AttBarLineGes, AttBarLineLog, AttBarLineVis, AttClefGrpAnl, AttClefGrpGes,
    AttClefGrpLog, AttClefGrpVis, AttCustosAnl, AttCustosGes, AttCustosLog, AttCustosVis,
    AttGrpSymAnl, AttGrpSymGes, AttGrpSymLog, AttGrpSymVis, AttHarpPedalAnl, AttHarpPedalGes,
    AttHarpPedalLog, AttHarpPedalVis, AttKeyAccidAnl, AttKeyAccidGes, AttKeyAccidLog,
    AttKeyAccidVis, AttMeterSigGrpAnl, AttMeterSigGrpGes, AttMeterSigGrpLog, AttMeterSigGrpVis,
    AttOssiaAnl, AttOssiaGes, AttOssiaLog, AttOssiaVis, AttPadAnl, AttPadGes, AttPadLog, AttPadVis,
    AttPartAnl, AttPartGes, AttPartLog, AttPartVis, AttPartsAnl, AttPartsGes, AttPartsLog,
    AttPartsVis, AttStemAnl, AttStemGes, AttStemLog, AttStemVis,
};
use tusk_model::elements::{
    BarLine, ClefGrp, ClefGrpChild, Custos, CustosChild, GrpSym, GrpSymChild, HarpPedal, KeyAccid,
    MeterSigGrp, MeterSigGrpChild, Ossia, OssiaChild, Pad, Part, PartChild, Parts, PartsChild,
    Stem,
};

use super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// BarLine attribute class implementations
// ============================================================================

impl CollectAttributes for AttBarLineLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "metcon", self.metcon);
        push_attr!(attrs, "control", self.control);
        push_attr!(attrs, "form", self.form);
        attrs
    }
}

impl CollectAttributes for AttBarLineVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "width", self.width);
        if let Some(v) = &self.len {
            attrs.push(("len", v.to_string()));
        }
        push_attr!(attrs, "method", self.method);
        push_attr!(attrs, "place", self.place);
        attrs
    }
}

impl CollectAttributes for AttBarLineGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttBarLineAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for BarLine {
    fn element_name(&self) -> &'static str {
        "barLine"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
        attrs.extend(self.bar_line_log.collect_attributes());
        attrs.extend(self.bar_line_vis.collect_attributes());
        attrs.extend(self.bar_line_ges.collect_attributes());
        attrs.extend(self.bar_line_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// Stem attribute class implementations
// ============================================================================

impl CollectAttributes for AttStemLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttStemVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "pos", self.pos);
        push_attr!(attrs, "len", self.len);
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "dir", self.dir);
        push_attr!(attrs, "flag.pos", self.flag_pos);
        push_attr!(attrs, "flag.form", self.flag_form);
        attrs
    }
}

impl CollectAttributes for AttStemGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttStemAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for Stem {
    fn element_name(&self) -> &'static str {
        "stem"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.stem_log.collect_attributes());
        attrs.extend(self.stem_vis.collect_attributes());
        attrs.extend(self.stem_ges.collect_attributes());
        attrs.extend(self.stem_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// ClefGrp attribute class implementations
// ============================================================================

impl CollectAttributes for AttClefGrpLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttClefGrpVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttClefGrpGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttClefGrpAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for ClefGrp {
    fn element_name(&self) -> &'static str {
        "clefGrp"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.event.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.clef_grp_log.collect_attributes());
        attrs.extend(self.clef_grp_vis.collect_attributes());
        attrs.extend(self.clef_grp_ges.collect_attributes());
        attrs.extend(self.clef_grp_anl.collect_attributes());
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

impl MeiSerialize for ClefGrpChild {
    fn element_name(&self) -> &'static str {
        match self {
            ClefGrpChild::Clef(_) => "clef",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            ClefGrpChild::Clef(clef) => clef.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            ClefGrpChild::Clef(clef) => clef.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ClefGrpChild::Clef(clef) => clef.serialize_children(writer),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ClefGrpChild::Clef(clef) => clef.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Custos attribute class implementations
// ============================================================================

impl CollectAttributes for AttCustosLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "oct", self.oct);
        push_attr!(attrs, "pname", self.pname);
        attrs
    }
}

impl CollectAttributes for AttCustosVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        attrs
    }
}

impl CollectAttributes for AttCustosGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttCustosAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for Custos {
    fn element_name(&self) -> &'static str {
        "custos"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.source.collect_attributes());
        attrs.extend(self.custos_log.collect_attributes());
        attrs.extend(self.custos_vis.collect_attributes());
        attrs.extend(self.custos_ges.collect_attributes());
        attrs.extend(self.custos_anl.collect_attributes());
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

impl MeiSerialize for CustosChild {
    fn element_name(&self) -> &'static str {
        match self {
            CustosChild::Accid(_) => "accid",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            CustosChild::Accid(accid) => accid.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            CustosChild::Accid(accid) => accid.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            CustosChild::Accid(accid) => accid.serialize_children(writer),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            CustosChild::Accid(accid) => accid.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Pad attribute class implementations
// ============================================================================

impl CollectAttributes for AttPadLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "width", self.width);
        attrs
    }
}

impl CollectAttributes for AttPadVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttPadGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttPadAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for Pad {
    fn element_name(&self) -> &'static str {
        "pad"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.pad_log.collect_attributes());
        attrs.extend(self.pad_vis.collect_attributes());
        attrs.extend(self.pad_ges.collect_attributes());
        attrs.extend(self.pad_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// KeyAccid attribute class implementations
// ============================================================================

impl CollectAttributes for AttKeyAccidLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "accid", self.accid);
        push_attr!(attrs, "pname", self.pname);
        push_attr!(attrs, "oct", self.oct);
        attrs
    }
}

impl CollectAttributes for AttKeyAccidVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttKeyAccidGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttKeyAccidAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for KeyAccid {
    fn element_name(&self) -> &'static str {
        "keyAccid"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.key_accid_log.collect_attributes());
        attrs.extend(self.key_accid_vis.collect_attributes());
        attrs.extend(self.key_accid_ges.collect_attributes());
        attrs.extend(self.key_accid_anl.collect_attributes());
        // Direct attribute
        if let Some(ref v) = self.form {
            attrs.push(("form", v.clone()));
        }
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// MeterSigGrp attribute class implementations
// ============================================================================

impl CollectAttributes for AttMeterSigGrpLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "func", self.func);
        attrs
    }
}

impl CollectAttributes for AttMeterSigGrpVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttMeterSigGrpGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttMeterSigGrpAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for MeterSigGrp {
    fn element_name(&self) -> &'static str {
        "meterSigGrp"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.meter_sig_grp_log.collect_attributes());
        attrs.extend(self.meter_sig_grp_vis.collect_attributes());
        attrs.extend(self.meter_sig_grp_ges.collect_attributes());
        attrs.extend(self.meter_sig_grp_anl.collect_attributes());
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

impl MeiSerialize for MeterSigGrpChild {
    fn element_name(&self) -> &'static str {
        match self {
            MeterSigGrpChild::MeterSig(_) => "meterSig",
            MeterSigGrpChild::MeterSigGrp(_) => "meterSigGrp",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            MeterSigGrpChild::MeterSig(m) => m.collect_all_attributes(),
            MeterSigGrpChild::MeterSigGrp(m) => m.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            MeterSigGrpChild::MeterSig(m) => m.has_children(),
            MeterSigGrpChild::MeterSigGrp(m) => m.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            MeterSigGrpChild::MeterSig(m) => m.serialize_children(writer),
            MeterSigGrpChild::MeterSigGrp(m) => m.serialize_children(writer),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            MeterSigGrpChild::MeterSig(m) => m.serialize_mei(writer),
            MeterSigGrpChild::MeterSigGrp(m) => m.serialize_mei(writer),
        }
    }
}

// ============================================================================
// GrpSym attribute class implementations
// ============================================================================

impl CollectAttributes for AttGrpSymLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "symbol", self.symbol);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "level", self.level);
        attrs
    }
}

impl CollectAttributes for AttGrpSymVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
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
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttGrpSymGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttGrpSymAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for GrpSym {
    fn element_name(&self) -> &'static str {
        "grpSym"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.grp_sym_log.collect_attributes());
        attrs.extend(self.grp_sym_vis.collect_attributes());
        attrs.extend(self.grp_sym_ges.collect_attributes());
        attrs.extend(self.grp_sym_anl.collect_attributes());
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

impl MeiSerialize for GrpSymChild {
    fn element_name(&self) -> &'static str {
        match self {
            GrpSymChild::Label(_) => "label",
            GrpSymChild::LabelAbbr(_) => "labelAbbr",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            GrpSymChild::Label(l) => l.collect_all_attributes(),
            GrpSymChild::LabelAbbr(l) => l.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            GrpSymChild::Label(l) => l.has_children(),
            GrpSymChild::LabelAbbr(l) => l.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            GrpSymChild::Label(l) => l.serialize_children(writer),
            GrpSymChild::LabelAbbr(l) => l.serialize_children(writer),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            GrpSymChild::Label(l) => l.serialize_mei(writer),
            GrpSymChild::LabelAbbr(l) => l.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Part/Parts attribute class implementations
// ============================================================================

impl CollectAttributes for AttPartLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttPartVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttPartGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttPartAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttPartsLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttPartsVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttPartsGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttPartsAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for Part {
    fn element_name(&self) -> &'static str {
        "part"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.part_log.collect_attributes());
        attrs.extend(self.part_vis.collect_attributes());
        attrs.extend(self.part_ges.collect_attributes());
        attrs.extend(self.part_anl.collect_attributes());
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

impl MeiSerialize for PartChild {
    fn element_name(&self) -> &'static str {
        match self {
            PartChild::Section(_) => "section",
            PartChild::ScoreDef(_) => "scoreDef",
            PartChild::StaffDef(_) => "staffDef",
            PartChild::Ending(_) => "ending",
            PartChild::Pb(_) => "pb",
            PartChild::Sb(_) => "sb",
            PartChild::Annot(_) => "annot",
            PartChild::AnchoredText(_) => "anchoredText",
            PartChild::Div(_) => "div",
            PartChild::Add(_) => "add",
            PartChild::App(_) => "app",
            PartChild::Choice(_) => "choice",
            PartChild::Corr(_) => "corr",
            PartChild::Damage(_) => "damage",
            PartChild::Del(_) => "del",
            PartChild::Gap(_) => "gap",
            PartChild::HandShift(_) => "handShift",
            PartChild::Orig(_) => "orig",
            PartChild::Reg(_) => "reg",
            PartChild::Restore(_) => "restore",
            PartChild::Sic(_) => "sic",
            PartChild::Supplied(_) => "supplied",
            PartChild::Unclear(_) => "unclear",
            PartChild::Subst(_) => "subst",
            PartChild::Cb(_) => "cb",
            PartChild::ColLayout(_) => "colLayout",
            PartChild::Curve(_) => "curve",
            PartChild::Line(_) => "line",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            PartChild::Section(e) => e.collect_all_attributes(),
            PartChild::ScoreDef(e) => e.collect_all_attributes(),
            PartChild::StaffDef(e) => e.collect_all_attributes(),
            PartChild::Ending(e) => e.collect_all_attributes(),
            PartChild::Pb(e) => e.collect_all_attributes(),
            PartChild::Sb(e) => e.collect_all_attributes(),
            PartChild::Annot(e) => e.collect_all_attributes(),
            PartChild::AnchoredText(e) => e.collect_all_attributes(),
            _ => Vec::new(), // Other children not commonly used
        }
    }

    fn has_children(&self) -> bool {
        match self {
            PartChild::Section(e) => e.has_children(),
            PartChild::ScoreDef(e) => e.has_children(),
            PartChild::StaffDef(e) => e.has_children(),
            PartChild::Ending(e) => e.has_children(),
            PartChild::Pb(e) => e.has_children(),
            PartChild::Sb(e) => e.has_children(),
            PartChild::Annot(e) => e.has_children(),
            PartChild::AnchoredText(e) => e.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PartChild::Section(e) => e.serialize_children(writer),
            PartChild::ScoreDef(e) => e.serialize_children(writer),
            PartChild::StaffDef(e) => e.serialize_children(writer),
            PartChild::Ending(e) => e.serialize_children(writer),
            PartChild::Pb(e) => e.serialize_children(writer),
            PartChild::Sb(e) => e.serialize_children(writer),
            PartChild::Annot(e) => e.serialize_children(writer),
            PartChild::AnchoredText(e) => e.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PartChild::Section(e) => e.serialize_mei(writer),
            PartChild::ScoreDef(e) => e.serialize_mei(writer),
            PartChild::StaffDef(e) => e.serialize_mei(writer),
            PartChild::Ending(e) => e.serialize_mei(writer),
            PartChild::Pb(e) => e.serialize_mei(writer),
            PartChild::Sb(e) => e.serialize_mei(writer),
            PartChild::Annot(e) => e.serialize_mei(writer),
            PartChild::AnchoredText(e) => e.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "PartChild::{}",
                other.element_name()
            ))),
        }
    }
}

impl MeiSerialize for Parts {
    fn element_name(&self) -> &'static str {
        "parts"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.parts_log.collect_attributes());
        attrs.extend(self.parts_vis.collect_attributes());
        attrs.extend(self.parts_ges.collect_attributes());
        attrs.extend(self.parts_anl.collect_attributes());
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

impl MeiSerialize for PartsChild {
    fn element_name(&self) -> &'static str {
        match self {
            PartsChild::Part(_) => "part",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            PartsChild::Part(p) => p.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            PartsChild::Part(p) => p.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PartsChild::Part(p) => p.serialize_children(writer),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PartsChild::Part(p) => p.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Ossia attribute class implementations
// ============================================================================

impl CollectAttributes for AttOssiaLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttOssiaVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttOssiaGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttOssiaAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for Ossia {
    fn element_name(&self) -> &'static str {
        "ossia"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.ossia_log.collect_attributes());
        attrs.extend(self.ossia_vis.collect_attributes());
        attrs.extend(self.ossia_ges.collect_attributes());
        attrs.extend(self.ossia_anl.collect_attributes());
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

impl MeiSerialize for OssiaChild {
    fn element_name(&self) -> &'static str {
        match self {
            OssiaChild::Layer(_) => "layer",
            OssiaChild::Staff(_) => "staff",
            OssiaChild::OLayer(_) => "oLayer",
            OssiaChild::OStaff(_) => "oStaff",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            OssiaChild::Layer(l) => l.collect_all_attributes(),
            OssiaChild::Staff(s) => s.collect_all_attributes(),
            _ => Vec::new(), // OLayer/OStaff not fully implemented
        }
    }

    fn has_children(&self) -> bool {
        match self {
            OssiaChild::Layer(l) => l.has_children(),
            OssiaChild::Staff(s) => s.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            OssiaChild::Layer(l) => l.serialize_children(writer),
            OssiaChild::Staff(s) => s.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            OssiaChild::Layer(l) => l.serialize_mei(writer),
            OssiaChild::Staff(s) => s.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "OssiaChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// HarpPedal attribute class implementations
// ============================================================================

impl CollectAttributes for AttHarpPedalLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "c", self.c);
        push_attr!(attrs, "d", self.d);
        push_attr!(attrs, "e", self.e);
        push_attr!(attrs, "f", self.f);
        push_attr!(attrs, "g", self.g);
        push_attr!(attrs, "a", self.a);
        push_attr!(attrs, "b", self.b);
        attrs
    }
}

impl CollectAttributes for AttHarpPedalVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttHarpPedalGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttHarpPedalAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for HarpPedal {
    fn element_name(&self) -> &'static str {
        "harpPedal"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.harp_pedal_log.collect_attributes());
        attrs.extend(self.harp_pedal_vis.collect_attributes());
        attrs.extend(self.harp_pedal_ges.collect_attributes());
        attrs.extend(self.harp_pedal_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}
