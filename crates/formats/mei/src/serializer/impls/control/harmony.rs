//! Serializer implementations for harmony elements: Harm, Fb, F, Symbol.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttFAnl, AttFGes, AttFLog, AttFVis, AttHarmAnl, AttHarmGes, AttHarmLog, AttHarmVis,
    AttSymbolAnl, AttSymbolGes, AttSymbolLog, AttSymbolVis,
};
use tusk_model::elements::{F, FChild, Fb, FbChild, Harm, HarmChild, Symbol};

use super::super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Harm (Harmony) element serialization
// ============================================================================

impl CollectAttributes for AttHarmLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        push_attr!(attrs, "chordref", self.chordref);
        attrs
    }
}

impl CollectAttributes for AttHarmVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "rendgrid", self.rendgrid);
        attrs
    }
}

impl CollectAttributes for AttHarmGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttHarmAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "inth", vec self.inth);
        push_attr!(attrs, "form", self.form);
        attrs
    }
}

impl MeiSerialize for Harm {
    fn element_name(&self) -> &'static str {
        "harm"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.harm_log.collect_attributes());
        attrs.extend(self.harm_vis.collect_attributes());
        attrs.extend(self.harm_ges.collect_attributes());
        attrs.extend(self.harm_anl.collect_attributes());
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

impl MeiSerialize for HarmChild {
    fn element_name(&self) -> &'static str {
        match self {
            HarmChild::Text(_) => "$text",
            HarmChild::Fb(_) => "fb",
            HarmChild::Rend(_) => "rend",
            HarmChild::Lb(_) => "lb",
            HarmChild::Ref(_) => "ref",
            HarmChild::Line(_) => "line",
            HarmChild::Curve(_) => "curve",
            HarmChild::AnchoredText(_) => "anchoredText",
            HarmChild::Title(_) => "title",
            HarmChild::Num(_) => "num",
            HarmChild::Date(_) => "date",
            HarmChild::Address(_) => "address",
            HarmChild::Annot(_) => "annot",
            HarmChild::Bibl(_) => "bibl",
            HarmChild::BiblStruct(_) => "biblStruct",
            HarmChild::PersName(_) => "persName",
            HarmChild::CorpName(_) => "corpName",
            HarmChild::Name(_) => "name",
            HarmChild::GeogName(_) => "geogName",
            HarmChild::Identifier(_) => "identifier",
            HarmChild::Ptr(_) => "ptr",
            HarmChild::Extent(_) => "extent",
            HarmChild::Fig(_) => "fig",
            HarmChild::Seg(_) => "seg",
            HarmChild::Symbol(_) => "symbol",
            HarmChild::Term(_) => "term",
            HarmChild::Stack(_) => "stack",
            HarmChild::PostBox(_) => "postBox",
            HarmChild::PostCode(_) => "postCode",
            HarmChild::Street(_) => "street",
            HarmChild::Bloc(_) => "bloc",
            HarmChild::Country(_) => "country",
            HarmChild::District(_) => "district",
            HarmChild::GeogFeat(_) => "geogFeat",
            HarmChild::Region(_) => "region",
            HarmChild::Settlement(_) => "settlement",
            HarmChild::PeriodName(_) => "periodName",
            HarmChild::StyleName(_) => "styleName",
            HarmChild::Catchwords(_) => "catchwords",
            HarmChild::Dim(_) => "dim",
            HarmChild::Dimensions(_) => "dimensions",
            HarmChild::Depth(_) => "depth",
            HarmChild::Height(_) => "height",
            HarmChild::Width(_) => "width",
            HarmChild::Heraldry(_) => "heraldry",
            HarmChild::Locus(_) => "locus",
            HarmChild::LocusGrp(_) => "locusGrp",
            HarmChild::Repository(_) => "repository",
            HarmChild::SecFolio(_) => "secFolio",
            HarmChild::Signatures(_) => "signatures",
            HarmChild::Stamp(_) => "stamp",
            HarmChild::Relation(_) => "relation",
            HarmChild::RelationList(_) => "relationList",
            // Editorial elements
            HarmChild::Abbr(_) => "abbr",
            HarmChild::Add(_) => "add",
            HarmChild::Choice(_) => "choice",
            HarmChild::Corr(_) => "corr",
            HarmChild::Damage(_) => "damage",
            HarmChild::Del(_) => "del",
            HarmChild::Expan(_) => "expan",
            HarmChild::Gap(_) => "gap",
            HarmChild::HandShift(_) => "handShift",
            HarmChild::Orig(_) => "orig",
            HarmChild::Q(_) => "q",
            HarmChild::Reg(_) => "reg",
            HarmChild::Restore(_) => "restore",
            HarmChild::Sic(_) => "sic",
            HarmChild::Subst(_) => "subst",
            HarmChild::Supplied(_) => "supplied",
            HarmChild::Unclear(_) => "unclear",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            HarmChild::Text(_) => Vec::new(),
            HarmChild::Fb(fb) => fb.collect_all_attributes(),
            HarmChild::Rend(r) => r.collect_all_attributes(),
            HarmChild::Lb(lb) => lb.collect_all_attributes(),
            HarmChild::Ref(r) => r.collect_all_attributes(),
            HarmChild::Symbol(s) => s.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            HarmChild::Text(_) => false,
            HarmChild::Fb(fb) => fb.has_children(),
            HarmChild::Rend(r) => r.has_children(),
            HarmChild::Lb(_) => false,
            HarmChild::Ref(r) => r.has_children(),
            HarmChild::Symbol(_) => false,
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            HarmChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            HarmChild::Fb(fb) => fb.serialize_children(writer),
            HarmChild::Rend(r) => r.serialize_children(writer),
            HarmChild::Lb(_) => Ok(()),
            HarmChild::Ref(r) => r.serialize_children(writer),
            HarmChild::Symbol(_) => Ok(()),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "HarmChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Fb (Figured Bass) element serialization
// ============================================================================

impl MeiSerialize for Fb {
    fn element_name(&self) -> &'static str {
        "fb"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
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

impl MeiSerialize for FbChild {
    fn element_name(&self) -> &'static str {
        match self {
            FbChild::F(_) => "f",
            FbChild::Gap(_) => "gap",
            FbChild::Sic(_) => "sic",
            FbChild::Damage(_) => "damage",
            FbChild::Unclear(_) => "unclear",
            FbChild::Orig(_) => "orig",
            FbChild::Corr(_) => "corr",
            FbChild::Restore(_) => "restore",
            FbChild::Subst(_) => "subst",
            FbChild::Reg(_) => "reg",
            FbChild::HandShift(_) => "handShift",
            FbChild::Add(_) => "add",
            FbChild::Choice(_) => "choice",
            FbChild::Supplied(_) => "supplied",
            FbChild::Del(_) => "del",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            FbChild::F(f) => f.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            FbChild::F(f) => f.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FbChild::F(f) => f.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "FbChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// F (Figure) element serialization
// ============================================================================

impl CollectAttributes for AttFLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}

impl CollectAttributes for AttFVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}

impl CollectAttributes for AttFGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttFAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttFAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for F {
    fn element_name(&self) -> &'static str {
        "f"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.f_log.collect_attributes());
        attrs.extend(self.f_vis.collect_attributes());
        attrs.extend(self.f_ges.collect_attributes());
        attrs.extend(self.f_anl.collect_attributes());
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

impl MeiSerialize for FChild {
    fn element_name(&self) -> &'static str {
        match self {
            FChild::Text(_) => "$text",
            FChild::Symbol(_) => "symbol",
            FChild::Rend(_) => "rend",
            FChild::Lb(_) => "lb",
            FChild::Title(_) => "title",
            FChild::Num(_) => "num",
            FChild::Date(_) => "date",
            FChild::Ref(_) => "ref",
            FChild::Ptr(_) => "ptr",
            FChild::Annot(_) => "annot",
            FChild::Bibl(_) => "bibl",
            FChild::BiblStruct(_) => "biblStruct",
            FChild::PersName(_) => "persName",
            FChild::CorpName(_) => "corpName",
            FChild::Name(_) => "name",
            FChild::GeogName(_) => "geogName",
            FChild::Identifier(_) => "identifier",
            FChild::Extent(_) => "extent",
            FChild::Fig(_) => "fig",
            FChild::Seg(_) => "seg",
            FChild::Term(_) => "term",
            FChild::Stack(_) => "stack",
            FChild::PostBox(_) => "postBox",
            FChild::PostCode(_) => "postCode",
            FChild::Street(_) => "street",
            FChild::Bloc(_) => "bloc",
            FChild::Country(_) => "country",
            FChild::District(_) => "district",
            FChild::GeogFeat(_) => "geogFeat",
            FChild::Region(_) => "region",
            FChild::Settlement(_) => "settlement",
            FChild::PeriodName(_) => "periodName",
            FChild::StyleName(_) => "styleName",
            FChild::Catchwords(_) => "catchwords",
            FChild::Dim(_) => "dim",
            FChild::Dimensions(_) => "dimensions",
            FChild::Depth(_) => "depth",
            FChild::Height(_) => "height",
            FChild::Width(_) => "width",
            FChild::Heraldry(_) => "heraldry",
            FChild::Locus(_) => "locus",
            FChild::LocusGrp(_) => "locusGrp",
            FChild::Repository(_) => "repository",
            FChild::SecFolio(_) => "secFolio",
            FChild::Signatures(_) => "signatures",
            FChild::Stamp(_) => "stamp",
            FChild::Relation(_) => "relation",
            FChild::RelationList(_) => "relationList",
            FChild::Address(_) => "address",
            // Editorial elements
            FChild::Abbr(_) => "abbr",
            FChild::Add(_) => "add",
            FChild::Choice(_) => "choice",
            FChild::Corr(_) => "corr",
            FChild::Damage(_) => "damage",
            FChild::Del(_) => "del",
            FChild::Expan(_) => "expan",
            FChild::Gap(_) => "gap",
            FChild::HandShift(_) => "handShift",
            FChild::Orig(_) => "orig",
            FChild::Q(_) => "q",
            FChild::Reg(_) => "reg",
            FChild::Restore(_) => "restore",
            FChild::Sic(_) => "sic",
            FChild::Subst(_) => "subst",
            FChild::Supplied(_) => "supplied",
            FChild::Unclear(_) => "unclear",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            FChild::Text(_) => Vec::new(),
            FChild::Symbol(s) => s.collect_all_attributes(),
            FChild::Rend(r) => r.collect_all_attributes(),
            FChild::Lb(lb) => lb.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            FChild::Text(_) => false,
            FChild::Symbol(_) => false,
            FChild::Rend(r) => r.has_children(),
            FChild::Lb(_) => false,
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FChild::Text(_) => Ok(()),
            FChild::Symbol(_) => Ok(()),
            FChild::Rend(r) => r.serialize_children(writer),
            FChild::Lb(_) => Ok(()),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "FChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            FChild::Symbol(elem) => elem.serialize_mei(writer),
            FChild::Rend(elem) => elem.serialize_mei(writer),
            FChild::Lb(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "FChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Symbol element serialization
// ============================================================================

impl CollectAttributes for AttSymbolLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "startid", self.startid);
        attrs
    }
}

impl CollectAttributes for AttSymbolVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "scale", self.scale);
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

impl CollectAttributes for AttSymbolGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttSymbolGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttSymbolAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttSymbolAnl has no attributes
        Vec::new()
    }
}

impl MeiSerialize for Symbol {
    fn element_name(&self) -> &'static str {
        "symbol"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.symbol_log.collect_attributes());
        attrs.extend(self.symbol_vis.collect_attributes());
        attrs.extend(self.symbol_ges.collect_attributes());
        attrs.extend(self.symbol_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}
