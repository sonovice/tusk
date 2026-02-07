//! Serializer implementations for text container MEI elements.
//!
//! This module contains implementations for Group, Quote, Q, Phrase, Line,
//! Refrain, and Stack elements.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttLineAnl, AttLineGes, AttLineLog, AttLineVis, AttPhraseAnl, AttPhraseGes, AttPhraseLog,
    AttPhraseVis, AttRefrainAnl, AttRefrainGes, AttRefrainLog, AttRefrainVis,
};
use tusk_model::elements::{
    Group, GroupChild, Line, LineChild, Phrase, PhraseChild, Q, QChild, Quote, QuoteChild, Refrain,
    RefrainChild, Stack, StackChild,
};

use super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Attribute class implementations for Line
// ============================================================================

// ============================================================================
// Attribute class implementations for Phrase
// ============================================================================

// ============================================================================
// Attribute class implementations for Refrain
// ============================================================================

// ============================================================================
// Group element implementation
// ============================================================================

impl MeiSerialize for Group {
    fn element_name(&self) -> &'static str {
        "group"
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

impl MeiSerialize for GroupChild {
    fn element_name(&self) -> &'static str {
        match self {
            GroupChild::Group(_) => "group",
            GroupChild::Music(_) => "music",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            GroupChild::Group(e) => e.collect_all_attributes(),
            GroupChild::Music(e) => e.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            GroupChild::Group(e) => e.has_children(),
            GroupChild::Music(e) => e.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            GroupChild::Group(e) => e.serialize_children(writer),
            GroupChild::Music(e) => e.serialize_children(writer),
        }
    }
}

// ============================================================================
// Quote element implementation
// ============================================================================

impl MeiSerialize for Quote {
    fn element_name(&self) -> &'static str {
        "quote"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.text_rendition.collect_attributes());
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

impl MeiSerialize for QuoteChild {
    fn element_name(&self) -> &'static str {
        match self {
            QuoteChild::Text(_) => "",
            QuoteChild::P(_) => "p",
            QuoteChild::Lg(_) => "lg",
            QuoteChild::Rend(_) => "rend",
            QuoteChild::Lb(_) => "lb",
            QuoteChild::Seg(_) => "seg",
            QuoteChild::Fig(_) => "fig",
            QuoteChild::Annot(_) => "annot",
            QuoteChild::Bibl(_) => "bibl",
            QuoteChild::Ref(_) => "ref",
            QuoteChild::Ptr(_) => "ptr",
            QuoteChild::Num(_) => "num",
            QuoteChild::Date(_) => "date",
            QuoteChild::Name(_) => "name",
            QuoteChild::PersName(_) => "persName",
            QuoteChild::CorpName(_) => "corpName",
            QuoteChild::Title(_) => "title",
            QuoteChild::Symbol(_) => "symbol",
            QuoteChild::Q(_) => "q",
            QuoteChild::Quote(_) => "quote",
            QuoteChild::Stack(_) => "stack",
            QuoteChild::Sp(_) => "sp",
            QuoteChild::StageDir(_) => "stageDir",
            QuoteChild::Abbr(_) => "abbr",
            QuoteChild::Expan(_) => "expan",
            QuoteChild::Corr(_) => "corr",
            QuoteChild::Sic(_) => "sic",
            QuoteChild::Add(_) => "add",
            QuoteChild::Del(_) => "del",
            QuoteChild::Orig(_) => "orig",
            QuoteChild::Reg(_) => "reg",
            QuoteChild::Choice(_) => "choice",
            QuoteChild::Subst(_) => "subst",
            QuoteChild::Supplied(_) => "supplied",
            QuoteChild::Damage(_) => "damage",
            QuoteChild::Gap(_) => "gap",
            QuoteChild::Restore(_) => "restore",
            QuoteChild::Unclear(_) => "unclear",
            QuoteChild::HandShift(_) => "handShift",
            QuoteChild::Extent(_) => "extent",
            QuoteChild::Table(_) => "table",
            QuoteChild::RelationList(_) => "relationList",
            QuoteChild::Locus(_) => "locus",
            QuoteChild::Dimensions(_) => "dimensions",
            QuoteChild::GeogName(_) => "geogName",
            QuoteChild::Identifier(_) => "identifier",
            QuoteChild::PostCode(_) => "postCode",
            QuoteChild::List(_) => "list",
            QuoteChild::Stamp(_) => "stamp",
            QuoteChild::Region(_) => "region",
            QuoteChild::District(_) => "district",
            QuoteChild::Catchwords(_) => "catchwords",
            QuoteChild::Pb(_) => "pb",
            QuoteChild::Dim(_) => "dim",
            QuoteChild::EventList(_) => "eventList",
            QuoteChild::Settlement(_) => "settlement",
            QuoteChild::CastList(_) => "castList",
            QuoteChild::Height(_) => "height",
            QuoteChild::Heraldry(_) => "heraldry",
            QuoteChild::BiblList(_) => "biblList",
            QuoteChild::Street(_) => "street",
            QuoteChild::StyleName(_) => "styleName",
            QuoteChild::Relation(_) => "relation",
            QuoteChild::Repository(_) => "repository",
            QuoteChild::LocusGrp(_) => "locusGrp",
            QuoteChild::GeogFeat(_) => "geogFeat",
            QuoteChild::PostBox(_) => "postBox",
            QuoteChild::Bloc(_) => "bloc",
            QuoteChild::Signatures(_) => "signatures",
            QuoteChild::Depth(_) => "depth",
            QuoteChild::Address(_) => "address",
            QuoteChild::PeriodName(_) => "periodName",
            QuoteChild::Term(_) => "term",
            QuoteChild::SecFolio(_) => "secFolio",
            QuoteChild::BiblStruct(_) => "biblStruct",
            QuoteChild::Width(_) => "width",
            QuoteChild::Country(_) => "country",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            QuoteChild::Text(_) => Vec::new(),
            QuoteChild::P(e) => e.collect_all_attributes(),
            QuoteChild::Lg(e) => e.collect_all_attributes(),
            QuoteChild::Rend(e) => e.collect_all_attributes(),
            QuoteChild::Lb(e) => e.collect_all_attributes(),
            QuoteChild::Seg(e) => e.collect_all_attributes(),
            QuoteChild::Fig(e) => e.collect_all_attributes(),
            QuoteChild::Annot(e) => e.collect_all_attributes(),
            QuoteChild::Bibl(e) => e.collect_all_attributes(),
            QuoteChild::Ref(e) => e.collect_all_attributes(),
            QuoteChild::Ptr(e) => e.collect_all_attributes(),
            QuoteChild::Num(e) => e.collect_all_attributes(),
            QuoteChild::Date(e) => e.collect_all_attributes(),
            QuoteChild::Name(e) => e.collect_all_attributes(),
            QuoteChild::PersName(e) => e.collect_all_attributes(),
            QuoteChild::CorpName(e) => e.collect_all_attributes(),
            QuoteChild::Title(e) => e.collect_all_attributes(),
            QuoteChild::Symbol(e) => e.collect_all_attributes(),
            QuoteChild::Q(e) => e.collect_all_attributes(),
            QuoteChild::Quote(e) => e.collect_all_attributes(),
            QuoteChild::Stack(e) => e.collect_all_attributes(),
            QuoteChild::Sp(e) => e.collect_all_attributes(),
            QuoteChild::StageDir(e) => e.collect_all_attributes(),
            QuoteChild::Abbr(e) => e.collect_all_attributes(),
            QuoteChild::Expan(e) => e.collect_all_attributes(),
            QuoteChild::Corr(e) => e.collect_all_attributes(),
            QuoteChild::Sic(e) => e.collect_all_attributes(),
            QuoteChild::Add(e) => e.collect_all_attributes(),
            QuoteChild::Del(e) => e.collect_all_attributes(),
            QuoteChild::Orig(e) => e.collect_all_attributes(),
            QuoteChild::Reg(e) => e.collect_all_attributes(),
            QuoteChild::Choice(e) => e.collect_all_attributes(),
            QuoteChild::Subst(e) => e.collect_all_attributes(),
            QuoteChild::Supplied(e) => e.collect_all_attributes(),
            QuoteChild::Damage(e) => e.collect_all_attributes(),
            QuoteChild::Gap(e) => e.collect_all_attributes(),
            QuoteChild::Restore(e) => e.collect_all_attributes(),
            QuoteChild::Unclear(e) => e.collect_all_attributes(),
            QuoteChild::HandShift(e) => e.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            QuoteChild::Text(_) => false,
            QuoteChild::P(e) => e.has_children(),
            QuoteChild::Lg(e) => e.has_children(),
            QuoteChild::Rend(e) => e.has_children(),
            QuoteChild::Lb(_) => false,
            QuoteChild::Seg(e) => e.has_children(),
            QuoteChild::Fig(e) => e.has_children(),
            QuoteChild::Annot(e) => e.has_children(),
            QuoteChild::Bibl(e) => e.has_children(),
            QuoteChild::Ref(e) => e.has_children(),
            QuoteChild::Ptr(_) => false,
            QuoteChild::Num(e) => e.has_children(),
            QuoteChild::Date(e) => e.has_children(),
            QuoteChild::Name(e) => e.has_children(),
            QuoteChild::PersName(e) => e.has_children(),
            QuoteChild::CorpName(e) => e.has_children(),
            QuoteChild::Title(e) => e.has_children(),
            QuoteChild::Symbol(_) => false,
            QuoteChild::Q(e) => e.has_children(),
            QuoteChild::Quote(e) => e.has_children(),
            QuoteChild::Stack(e) => e.has_children(),
            QuoteChild::Sp(e) => e.has_children(),
            QuoteChild::StageDir(e) => e.has_children(),
            QuoteChild::Abbr(e) => e.has_children(),
            QuoteChild::Expan(e) => e.has_children(),
            QuoteChild::Corr(e) => e.has_children(),
            QuoteChild::Sic(e) => e.has_children(),
            QuoteChild::Add(e) => e.has_children(),
            QuoteChild::Del(e) => e.has_children(),
            QuoteChild::Orig(e) => e.has_children(),
            QuoteChild::Reg(e) => e.has_children(),
            QuoteChild::Choice(e) => e.has_children(),
            QuoteChild::Subst(e) => e.has_children(),
            QuoteChild::Supplied(e) => e.has_children(),
            QuoteChild::Damage(e) => e.has_children(),
            QuoteChild::Gap(_) => false,
            QuoteChild::Restore(e) => e.has_children(),
            QuoteChild::Unclear(e) => e.has_children(),
            QuoteChild::HandShift(_) => false,
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            QuoteChild::P(e) => e.serialize_children(writer),
            QuoteChild::Lg(e) => e.serialize_children(writer),
            QuoteChild::Rend(e) => e.serialize_children(writer),
            QuoteChild::Seg(e) => e.serialize_children(writer),
            QuoteChild::Fig(e) => e.serialize_children(writer),
            QuoteChild::Annot(e) => e.serialize_children(writer),
            QuoteChild::Bibl(e) => e.serialize_children(writer),
            QuoteChild::Ref(e) => e.serialize_children(writer),
            QuoteChild::Num(e) => e.serialize_children(writer),
            QuoteChild::Date(e) => e.serialize_children(writer),
            QuoteChild::Name(e) => e.serialize_children(writer),
            QuoteChild::PersName(e) => e.serialize_children(writer),
            QuoteChild::CorpName(e) => e.serialize_children(writer),
            QuoteChild::Title(e) => e.serialize_children(writer),
            QuoteChild::Q(e) => e.serialize_children(writer),
            QuoteChild::Quote(e) => e.serialize_children(writer),
            QuoteChild::Stack(e) => e.serialize_children(writer),
            QuoteChild::Sp(e) => e.serialize_children(writer),
            QuoteChild::StageDir(e) => e.serialize_children(writer),
            QuoteChild::Abbr(e) => e.serialize_children(writer),
            QuoteChild::Expan(e) => e.serialize_children(writer),
            QuoteChild::Corr(e) => e.serialize_children(writer),
            QuoteChild::Sic(e) => e.serialize_children(writer),
            QuoteChild::Add(e) => e.serialize_children(writer),
            QuoteChild::Del(e) => e.serialize_children(writer),
            QuoteChild::Orig(e) => e.serialize_children(writer),
            QuoteChild::Reg(e) => e.serialize_children(writer),
            QuoteChild::Choice(e) => e.serialize_children(writer),
            QuoteChild::Subst(e) => e.serialize_children(writer),
            QuoteChild::Supplied(e) => e.serialize_children(writer),
            QuoteChild::Damage(e) => e.serialize_children(writer),
            QuoteChild::Restore(e) => e.serialize_children(writer),
            QuoteChild::Unclear(e) => e.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            QuoteChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            _ => {
                let name = self.element_name();
                let attrs = self.collect_all_attributes();

                let mut start = writer.start_element(name)?;
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
    }
}

// ============================================================================
// Q element implementation
// ============================================================================

impl MeiSerialize for Q {
    fn element_name(&self) -> &'static str {
        "q"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.classed.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.linking.collect_attributes());
        attrs.extend(self.n_number_like.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
        attrs.extend(self.text_rendition.collect_attributes());
        // Q-specific type attribute
        if !self.r#type.is_empty() {
            attrs.push(("type", self.r#type.join(" ")));
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

impl MeiSerialize for QChild {
    fn element_name(&self) -> &'static str {
        match self {
            QChild::Text(_) => "",
            QChild::Rend(_) => "rend",
            QChild::Lb(_) => "lb",
            QChild::Seg(_) => "seg",
            QChild::Fig(_) => "fig",
            QChild::Annot(_) => "annot",
            QChild::Bibl(_) => "bibl",
            QChild::Ref(_) => "ref",
            QChild::Ptr(_) => "ptr",
            QChild::Num(_) => "num",
            QChild::Date(_) => "date",
            QChild::Name(_) => "name",
            QChild::PersName(_) => "persName",
            QChild::CorpName(_) => "corpName",
            QChild::Title(_) => "title",
            QChild::Symbol(_) => "symbol",
            QChild::Q(_) => "q",
            QChild::Stack(_) => "stack",
            QChild::Abbr(_) => "abbr",
            QChild::Expan(_) => "expan",
            QChild::Extent(_) => "extent",
            QChild::PeriodName(_) => "periodName",
            QChild::PostCode(_) => "postCode",
            QChild::Region(_) => "region",
            QChild::BiblStruct(_) => "biblStruct",
            QChild::Relation(_) => "relation",
            QChild::Repository(_) => "repository",
            QChild::Signatures(_) => "signatures",
            QChild::Width(_) => "width",
            QChild::GeogName(_) => "geogName",
            QChild::Dimensions(_) => "dimensions",
            QChild::Identifier(_) => "identifier",
            QChild::LocusGrp(_) => "locusGrp",
            QChild::Settlement(_) => "settlement",
            QChild::Street(_) => "street",
            QChild::Address(_) => "address",
            QChild::Term(_) => "term",
            QChild::Catchwords(_) => "catchwords",
            QChild::Depth(_) => "depth",
            QChild::District(_) => "district",
            QChild::PostBox(_) => "postBox",
            QChild::RelationList(_) => "relationList",
            QChild::Bloc(_) => "bloc",
            QChild::Height(_) => "height",
            QChild::Stamp(_) => "stamp",
            QChild::StyleName(_) => "styleName",
            QChild::GeogFeat(_) => "geogFeat",
            QChild::Heraldry(_) => "heraldry",
            QChild::SecFolio(_) => "secFolio",
            QChild::Country(_) => "country",
            QChild::Pb(_) => "pb",
            QChild::Dim(_) => "dim",
            QChild::Locus(_) => "locus",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            QChild::Text(_) => Vec::new(),
            QChild::Rend(e) => e.collect_all_attributes(),
            QChild::Lb(e) => e.collect_all_attributes(),
            QChild::Seg(e) => e.collect_all_attributes(),
            QChild::Fig(e) => e.collect_all_attributes(),
            QChild::Annot(e) => e.collect_all_attributes(),
            QChild::Bibl(e) => e.collect_all_attributes(),
            QChild::Ref(e) => e.collect_all_attributes(),
            QChild::Ptr(e) => e.collect_all_attributes(),
            QChild::Num(e) => e.collect_all_attributes(),
            QChild::Date(e) => e.collect_all_attributes(),
            QChild::Name(e) => e.collect_all_attributes(),
            QChild::PersName(e) => e.collect_all_attributes(),
            QChild::CorpName(e) => e.collect_all_attributes(),
            QChild::Title(e) => e.collect_all_attributes(),
            QChild::Symbol(e) => e.collect_all_attributes(),
            QChild::Q(e) => e.collect_all_attributes(),
            QChild::Stack(e) => e.collect_all_attributes(),
            QChild::Abbr(e) => e.collect_all_attributes(),
            QChild::Expan(e) => e.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            QChild::Text(_) => false,
            QChild::Rend(e) => e.has_children(),
            QChild::Lb(_) => false,
            QChild::Seg(e) => e.has_children(),
            QChild::Fig(e) => e.has_children(),
            QChild::Annot(e) => e.has_children(),
            QChild::Bibl(e) => e.has_children(),
            QChild::Ref(e) => e.has_children(),
            QChild::Ptr(_) => false,
            QChild::Num(e) => e.has_children(),
            QChild::Date(e) => e.has_children(),
            QChild::Name(e) => e.has_children(),
            QChild::PersName(e) => e.has_children(),
            QChild::CorpName(e) => e.has_children(),
            QChild::Title(e) => e.has_children(),
            QChild::Symbol(_) => false,
            QChild::Q(e) => e.has_children(),
            QChild::Stack(e) => e.has_children(),
            QChild::Abbr(e) => e.has_children(),
            QChild::Expan(e) => e.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            QChild::Rend(e) => e.serialize_children(writer),
            QChild::Seg(e) => e.serialize_children(writer),
            QChild::Fig(e) => e.serialize_children(writer),
            QChild::Annot(e) => e.serialize_children(writer),
            QChild::Bibl(e) => e.serialize_children(writer),
            QChild::Ref(e) => e.serialize_children(writer),
            QChild::Num(e) => e.serialize_children(writer),
            QChild::Date(e) => e.serialize_children(writer),
            QChild::Name(e) => e.serialize_children(writer),
            QChild::PersName(e) => e.serialize_children(writer),
            QChild::CorpName(e) => e.serialize_children(writer),
            QChild::Title(e) => e.serialize_children(writer),
            QChild::Q(e) => e.serialize_children(writer),
            QChild::Stack(e) => e.serialize_children(writer),
            QChild::Abbr(e) => e.serialize_children(writer),
            QChild::Expan(e) => e.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            QChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            _ => {
                let name = self.element_name();
                let attrs = self.collect_all_attributes();

                let mut start = writer.start_element(name)?;
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
    }
}

// ============================================================================
// Phrase element implementation
// ============================================================================

impl MeiSerialize for Phrase {
    fn element_name(&self) -> &'static str {
        "phrase"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.phrase_anl.collect_attributes());
        attrs.extend(self.phrase_ges.collect_attributes());
        attrs.extend(self.phrase_log.collect_attributes());
        attrs.extend(self.phrase_vis.collect_attributes());
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

impl MeiSerialize for PhraseChild {
    fn element_name(&self) -> &'static str {
        match self {
            PhraseChild::Curve(_) => "curve",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        // Curve doesn't have MeiSerialize implemented yet
        Vec::new()
    }

    fn has_children(&self) -> bool {
        match self {
            PhraseChild::Curve(_) => false,
        }
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// Line element implementation
// ============================================================================

impl MeiSerialize for Line {
    fn element_name(&self) -> &'static str {
        "line"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.line_anl.collect_attributes());
        attrs.extend(self.line_ges.collect_attributes());
        attrs.extend(self.line_log.collect_attributes());
        attrs.extend(self.line_vis.collect_attributes());
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

impl MeiSerialize for LineChild {
    fn element_name(&self) -> &'static str {
        match self {
            LineChild::Text(_) => "",
            LineChild::Rend(_) => "rend",
            LineChild::Lb(_) => "lb",
            LineChild::Seg(_) => "seg",
            LineChild::Fig(_) => "fig",
            LineChild::Annot(_) => "annot",
            LineChild::Bibl(_) => "bibl",
            LineChild::Ref(_) => "ref",
            LineChild::Ptr(_) => "ptr",
            LineChild::Num(_) => "num",
            LineChild::Date(_) => "date",
            LineChild::Name(_) => "name",
            LineChild::PersName(_) => "persName",
            LineChild::CorpName(_) => "corpName",
            LineChild::Title(_) => "title",
            LineChild::Symbol(_) => "symbol",
            LineChild::Q(_) => "q",
            LineChild::Stack(_) => "stack",
            LineChild::Abbr(_) => "abbr",
            LineChild::Expan(_) => "expan",
            LineChild::StyleName(_) => "styleName",
            LineChild::GeogFeat(_) => "geogFeat",
            LineChild::Depth(_) => "depth",
            LineChild::Address(_) => "address",
            LineChild::Signatures(_) => "signatures",
            LineChild::PostBox(_) => "postBox",
            LineChild::Repository(_) => "repository",
            LineChild::SecFolio(_) => "secFolio",
            LineChild::Dimensions(_) => "dimensions",
            LineChild::Width(_) => "width",
            LineChild::Identifier(_) => "identifier",
            LineChild::Heraldry(_) => "heraldry",
            LineChild::PostCode(_) => "postCode",
            LineChild::PeriodName(_) => "periodName",
            LineChild::Bloc(_) => "bloc",
            LineChild::Country(_) => "country",
            LineChild::Stamp(_) => "stamp",
            LineChild::Term(_) => "term",
            LineChild::Locus(_) => "locus",
            LineChild::BiblStruct(_) => "biblStruct",
            LineChild::Height(_) => "height",
            LineChild::Extent(_) => "extent",
            LineChild::Catchwords(_) => "catchwords",
            LineChild::District(_) => "district",
            LineChild::RelationList(_) => "relationList",
            LineChild::Dim(_) => "dim",
            LineChild::Region(_) => "region",
            LineChild::LocusGrp(_) => "locusGrp",
            LineChild::GeogName(_) => "geogName",
            LineChild::Relation(_) => "relation",
            LineChild::Settlement(_) => "settlement",
            LineChild::Street(_) => "street",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            LineChild::Text(_) => Vec::new(),
            LineChild::Rend(e) => e.collect_all_attributes(),
            LineChild::Lb(e) => e.collect_all_attributes(),
            LineChild::Seg(e) => e.collect_all_attributes(),
            LineChild::Fig(e) => e.collect_all_attributes(),
            LineChild::Annot(e) => e.collect_all_attributes(),
            LineChild::Bibl(e) => e.collect_all_attributes(),
            LineChild::Ref(e) => e.collect_all_attributes(),
            LineChild::Ptr(e) => e.collect_all_attributes(),
            LineChild::Num(e) => e.collect_all_attributes(),
            LineChild::Date(e) => e.collect_all_attributes(),
            LineChild::Name(e) => e.collect_all_attributes(),
            LineChild::PersName(e) => e.collect_all_attributes(),
            LineChild::CorpName(e) => e.collect_all_attributes(),
            LineChild::Title(e) => e.collect_all_attributes(),
            LineChild::Symbol(e) => e.collect_all_attributes(),
            LineChild::Q(e) => e.collect_all_attributes(),
            LineChild::Stack(e) => e.collect_all_attributes(),
            LineChild::Abbr(e) => e.collect_all_attributes(),
            LineChild::Expan(e) => e.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            LineChild::Text(_) => false,
            LineChild::Rend(e) => e.has_children(),
            LineChild::Lb(_) => false,
            LineChild::Seg(e) => e.has_children(),
            LineChild::Fig(e) => e.has_children(),
            LineChild::Annot(e) => e.has_children(),
            LineChild::Bibl(e) => e.has_children(),
            LineChild::Ref(e) => e.has_children(),
            LineChild::Ptr(_) => false,
            LineChild::Num(e) => e.has_children(),
            LineChild::Date(e) => e.has_children(),
            LineChild::Name(e) => e.has_children(),
            LineChild::PersName(e) => e.has_children(),
            LineChild::CorpName(e) => e.has_children(),
            LineChild::Title(e) => e.has_children(),
            LineChild::Symbol(_) => false,
            LineChild::Q(e) => e.has_children(),
            LineChild::Stack(e) => e.has_children(),
            LineChild::Abbr(e) => e.has_children(),
            LineChild::Expan(e) => e.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            LineChild::Rend(e) => e.serialize_children(writer),
            LineChild::Seg(e) => e.serialize_children(writer),
            LineChild::Fig(e) => e.serialize_children(writer),
            LineChild::Annot(e) => e.serialize_children(writer),
            LineChild::Bibl(e) => e.serialize_children(writer),
            LineChild::Ref(e) => e.serialize_children(writer),
            LineChild::Num(e) => e.serialize_children(writer),
            LineChild::Date(e) => e.serialize_children(writer),
            LineChild::Name(e) => e.serialize_children(writer),
            LineChild::PersName(e) => e.serialize_children(writer),
            LineChild::CorpName(e) => e.serialize_children(writer),
            LineChild::Title(e) => e.serialize_children(writer),
            LineChild::Q(e) => e.serialize_children(writer),
            LineChild::Stack(e) => e.serialize_children(writer),
            LineChild::Abbr(e) => e.serialize_children(writer),
            LineChild::Expan(e) => e.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            LineChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            _ => {
                let name = self.element_name();
                let attrs = self.collect_all_attributes();

                let mut start = writer.start_element(name)?;
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
    }
}

// ============================================================================
// Refrain element implementation
// ============================================================================

impl MeiSerialize for Refrain {
    fn element_name(&self) -> &'static str {
        "refrain"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.refrain_log.collect_attributes());
        attrs.extend(self.refrain_vis.collect_attributes());
        attrs.extend(self.refrain_ges.collect_attributes());
        attrs.extend(self.refrain_anl.collect_attributes());
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

impl MeiSerialize for RefrainChild {
    fn element_name(&self) -> &'static str {
        match self {
            RefrainChild::Syl(_) => "syl",
            RefrainChild::Lb(_) => "lb",
            RefrainChild::Space(_) => "space",
            RefrainChild::Dir(_) => "dir",
            RefrainChild::Dynam(_) => "dynam",
            RefrainChild::Tempo(_) => "tempo",
            RefrainChild::Volta(_) => "volta",
            RefrainChild::App(_) => "app",
            RefrainChild::Choice(_) => "choice",
            RefrainChild::Subst(_) => "subst",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            RefrainChild::Syl(e) => e.collect_all_attributes(),
            RefrainChild::Lb(e) => e.collect_all_attributes(),
            RefrainChild::Space(e) => e.collect_all_attributes(),
            RefrainChild::Dir(e) => e.collect_all_attributes(),
            RefrainChild::Dynam(e) => e.collect_all_attributes(),
            RefrainChild::Tempo(e) => e.collect_all_attributes(),
            RefrainChild::Volta(e) => e.collect_all_attributes(),
            RefrainChild::App(e) => e.collect_all_attributes(),
            RefrainChild::Choice(e) => e.collect_all_attributes(),
            RefrainChild::Subst(e) => e.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            RefrainChild::Syl(e) => e.has_children(),
            RefrainChild::Lb(_) => false,
            RefrainChild::Space(_) => false,
            RefrainChild::Dir(e) => e.has_children(),
            RefrainChild::Dynam(e) => e.has_children(),
            RefrainChild::Tempo(e) => e.has_children(),
            RefrainChild::Volta(e) => e.has_children(),
            RefrainChild::App(e) => e.has_children(),
            RefrainChild::Choice(e) => e.has_children(),
            RefrainChild::Subst(e) => e.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            RefrainChild::Syl(e) => e.serialize_children(writer),
            RefrainChild::Dir(e) => e.serialize_children(writer),
            RefrainChild::Dynam(e) => e.serialize_children(writer),
            RefrainChild::Tempo(e) => e.serialize_children(writer),
            RefrainChild::Volta(e) => e.serialize_children(writer),
            RefrainChild::App(e) => e.serialize_children(writer),
            RefrainChild::Choice(e) => e.serialize_children(writer),
            RefrainChild::Subst(e) => e.serialize_children(writer),
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Stack element implementation
// ============================================================================

impl MeiSerialize for Stack {
    fn element_name(&self) -> &'static str {
        "stack"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        // Stack-specific attributes
        if let Some(ref delim) = self.delim {
            attrs.push(("delim", delim.clone()));
        }
        if let Some(ref align) = self.align {
            attrs.push(("align", align.clone()));
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

impl MeiSerialize for StackChild {
    fn element_name(&self) -> &'static str {
        match self {
            StackChild::Text(_) => "",
            StackChild::Rend(_) => "rend",
            StackChild::Lb(_) => "lb",
            StackChild::Seg(_) => "seg",
            StackChild::Fig(_) => "fig",
            StackChild::Annot(_) => "annot",
            StackChild::Bibl(_) => "bibl",
            StackChild::Ref(_) => "ref",
            StackChild::Ptr(_) => "ptr",
            StackChild::Num(_) => "num",
            StackChild::Date(_) => "date",
            StackChild::Name(_) => "name",
            StackChild::PersName(_) => "persName",
            StackChild::CorpName(_) => "corpName",
            StackChild::Title(_) => "title",
            StackChild::Symbol(_) => "symbol",
            StackChild::Q(_) => "q",
            StackChild::Stack(_) => "stack",
            StackChild::Abbr(_) => "abbr",
            StackChild::Expan(_) => "expan",
            StackChild::StyleName(_) => "styleName",
            StackChild::Width(_) => "width",
            StackChild::RelationList(_) => "relationList",
            StackChild::Address(_) => "address",
            StackChild::LocusGrp(_) => "locusGrp",
            StackChild::Street(_) => "street",
            StackChild::Depth(_) => "depth",
            StackChild::Term(_) => "term",
            StackChild::Region(_) => "region",
            StackChild::PostCode(_) => "postCode",
            StackChild::Heraldry(_) => "heraldry",
            StackChild::Height(_) => "height",
            StackChild::GeogName(_) => "geogName",
            StackChild::Identifier(_) => "identifier",
            StackChild::Country(_) => "country",
            StackChild::Dim(_) => "dim",
            StackChild::PeriodName(_) => "periodName",
            StackChild::District(_) => "district",
            StackChild::Repository(_) => "repository",
            StackChild::SecFolio(_) => "secFolio",
            StackChild::Settlement(_) => "settlement",
            StackChild::BiblStruct(_) => "biblStruct",
            StackChild::Dimensions(_) => "dimensions",
            StackChild::Signatures(_) => "signatures",
            StackChild::Locus(_) => "locus",
            StackChild::Extent(_) => "extent",
            StackChild::Catchwords(_) => "catchwords",
            StackChild::Stamp(_) => "stamp",
            StackChild::Bloc(_) => "bloc",
            StackChild::GeogFeat(_) => "geogFeat",
            StackChild::PostBox(_) => "postBox",
            StackChild::Relation(_) => "relation",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            StackChild::Text(_) => Vec::new(),
            StackChild::Rend(e) => e.collect_all_attributes(),
            StackChild::Lb(e) => e.collect_all_attributes(),
            StackChild::Seg(e) => e.collect_all_attributes(),
            StackChild::Fig(e) => e.collect_all_attributes(),
            StackChild::Annot(e) => e.collect_all_attributes(),
            StackChild::Bibl(e) => e.collect_all_attributes(),
            StackChild::Ref(e) => e.collect_all_attributes(),
            StackChild::Ptr(e) => e.collect_all_attributes(),
            StackChild::Num(e) => e.collect_all_attributes(),
            StackChild::Date(e) => e.collect_all_attributes(),
            StackChild::Name(e) => e.collect_all_attributes(),
            StackChild::PersName(e) => e.collect_all_attributes(),
            StackChild::CorpName(e) => e.collect_all_attributes(),
            StackChild::Title(e) => e.collect_all_attributes(),
            StackChild::Symbol(e) => e.collect_all_attributes(),
            StackChild::Q(e) => e.collect_all_attributes(),
            StackChild::Stack(e) => e.collect_all_attributes(),
            StackChild::Abbr(e) => e.collect_all_attributes(),
            StackChild::Expan(e) => e.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            StackChild::Text(_) => false,
            StackChild::Rend(e) => e.has_children(),
            StackChild::Lb(_) => false,
            StackChild::Seg(e) => e.has_children(),
            StackChild::Fig(e) => e.has_children(),
            StackChild::Annot(e) => e.has_children(),
            StackChild::Bibl(e) => e.has_children(),
            StackChild::Ref(e) => e.has_children(),
            StackChild::Ptr(_) => false,
            StackChild::Num(e) => e.has_children(),
            StackChild::Date(e) => e.has_children(),
            StackChild::Name(e) => e.has_children(),
            StackChild::PersName(e) => e.has_children(),
            StackChild::CorpName(e) => e.has_children(),
            StackChild::Title(e) => e.has_children(),
            StackChild::Symbol(_) => false,
            StackChild::Q(e) => e.has_children(),
            StackChild::Stack(e) => e.has_children(),
            StackChild::Abbr(e) => e.has_children(),
            StackChild::Expan(e) => e.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            StackChild::Rend(e) => e.serialize_children(writer),
            StackChild::Seg(e) => e.serialize_children(writer),
            StackChild::Fig(e) => e.serialize_children(writer),
            StackChild::Annot(e) => e.serialize_children(writer),
            StackChild::Bibl(e) => e.serialize_children(writer),
            StackChild::Ref(e) => e.serialize_children(writer),
            StackChild::Num(e) => e.serialize_children(writer),
            StackChild::Date(e) => e.serialize_children(writer),
            StackChild::Name(e) => e.serialize_children(writer),
            StackChild::PersName(e) => e.serialize_children(writer),
            StackChild::CorpName(e) => e.serialize_children(writer),
            StackChild::Title(e) => e.serialize_children(writer),
            StackChild::Q(e) => e.serialize_children(writer),
            StackChild::Stack(e) => e.serialize_children(writer),
            StackChild::Abbr(e) => e.serialize_children(writer),
            StackChild::Expan(e) => e.serialize_children(writer),
            _ => Ok(()),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            StackChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            _ => {
                let name = self.element_name();
                let attrs = self.collect_all_attributes();

                let mut start = writer.start_element(name)?;
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
    }
}

// ============================================================================
// Cb (column beginning) element implementation
// ============================================================================

use tusk_model::att::{
    AttCurveAnl, AttCurveGes, AttCurveLog, AttCurveVis, AttDivLineLog, AttExtSym, AttStaffLoc,
    AttVisibility, AttVisualOffsetHo,
};
use tusk_model::elements::{Cb, Curve, DivLine};

impl MeiSerialize for Cb {
    fn element_name(&self) -> &'static str {
        "cb"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.linking.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
        attrs.extend(self.source.collect_attributes());
        attrs.extend(self.typed.collect_attributes());

        // Element-specific n attribute (column number)
        if let Some(v) = &self.n {
            attrs.push(("n", v.to_string()));
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
// DivLine (division line in neumes) element implementation
// ============================================================================

impl MeiSerialize for DivLine {
    fn element_name(&self) -> &'static str {
        "divLine"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.classed.collect_attributes());
        attrs.extend(self.color.collect_attributes());
        attrs.extend(self.div_line_log.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.linking.collect_attributes());
        attrs.extend(self.n_number_like.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
        attrs.extend(self.ext_sym.collect_attributes());
        attrs.extend(self.staff_loc.collect_attributes());
        attrs.extend(self.visibility.collect_attributes());
        attrs.extend(self.xy.collect_attributes());
        attrs.extend(self.visual_offset_ho.collect_attributes());
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
// Curve (generic curved line) element implementation
// ============================================================================

impl MeiSerialize for Curve {
    fn element_name(&self) -> &'static str {
        "curve"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.curve_anl.collect_attributes());
        attrs.extend(self.curve_ges.collect_attributes());
        attrs.extend(self.curve_log.collect_attributes());
        attrs.extend(self.curve_vis.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}
