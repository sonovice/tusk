//! Serializer implementations for work-specific child elements.
//!
//! Contains: AltId, ExtMeta, Key, Meter, Incip, Creation, PerfMedium, Classification.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    AltId, AltIdChild, CastGrp, CastGrpChild, CastItem, CastItemChild, CastList, CastListChild,
    Classification, ClassificationChild, Creation, CreationChild, ExtMeta, ExtMetaChild, Incip,
    IncipChild, IncipCode, IncipCodeChild, IncipText, IncipTextChild, Key, KeyChild, Meter,
    MeterChild, PerfMedium, PerfMediumChild, PerfRes, PerfResChild, PerfResList, PerfResListChild,
    RoleDesc, RoleDescChild, Term, TermChild, TermList, TermListChild,
};

// ============================================================================
// AltId
// ============================================================================

impl MeiSerialize for AltId {
    fn element_name(&self) -> &'static str {
        "altId"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for AltIdChild {
    fn element_name(&self) -> &'static str {
        match self {
            AltIdChild::Text(_) => "#text",
            AltIdChild::Stack(_) => "stack",
            AltIdChild::Lb(_) => "lb",
            AltIdChild::Rend(_) => "rend",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, AltIdChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            AltIdChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            AltIdChild::Lb(elem) => elem.serialize_mei(writer),
            AltIdChild::Rend(elem) => elem.serialize_mei(writer),
            // Stack needs its own serializer - for now write empty element
            AltIdChild::Stack(_) => {
                let start = writer.start_element("stack")?;
                writer.write_empty(start)?;
                Ok(())
            }
        }
    }
}

// ============================================================================
// ExtMeta
// ============================================================================

impl MeiSerialize for ExtMeta {
    fn element_name(&self) -> &'static str {
        "extMeta"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.whitespace.collect_attributes());
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

impl MeiSerialize for ExtMetaChild {
    fn element_name(&self) -> &'static str {
        match self {
            ExtMetaChild::Text(_) => "#text",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ExtMetaChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
        }
    }
}

// ============================================================================
// Key
// ============================================================================

impl MeiSerialize for Key {
    fn element_name(&self) -> &'static str {
        "key"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.accidental.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.key_mode.collect_attributes());
        attrs.extend(self.pitch.collect_attributes());
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

impl MeiSerialize for KeyChild {
    fn element_name(&self) -> &'static str {
        match self {
            KeyChild::Text(_) => "#text",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            KeyChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
        }
    }
}

// ============================================================================
// Meter
// ============================================================================

impl MeiSerialize for Meter {
    fn element_name(&self) -> &'static str {
        "meter"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.meter_sig_log.collect_attributes());
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

impl MeiSerialize for MeterChild {
    fn element_name(&self) -> &'static str {
        match self {
            MeterChild::Text(_) => "#text",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            MeterChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
        }
    }
}

// ============================================================================
// Incip
// ============================================================================

impl MeiSerialize for Incip {
    fn element_name(&self) -> &'static str {
        "incip"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for IncipChild {
    fn element_name(&self) -> &'static str {
        match self {
            IncipChild::Meter(_) => "meter",
            IncipChild::PerfResList(_) => "perfResList",
            IncipChild::IncipCode(_) => "incipCode",
            IncipChild::IncipText(_) => "incipText",
            IncipChild::Key(_) => "key",
            IncipChild::Role(_) => "role",
            IncipChild::Graphic(_) => "graphic",
            IncipChild::ClefGrp(_) => "clefGrp",
            IncipChild::Score(_) => "score",
            IncipChild::Head(_) => "head",
            IncipChild::Mensuration(_) => "mensuration",
            IncipChild::Tempo(_) => "tempo",
            IncipChild::PerfRes(_) => "perfRes",
            IncipChild::Clef(_) => "clef",
            IncipChild::Annot(_) => "annot",
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
            IncipChild::Meter(elem) => elem.serialize_mei(writer),
            IncipChild::Key(elem) => elem.serialize_mei(writer),
            IncipChild::Head(elem) => elem.serialize_mei(writer),
            IncipChild::Tempo(elem) => elem.serialize_mei(writer),
            IncipChild::Annot(elem) => elem.serialize_mei(writer),
            IncipChild::Score(elem) => elem.serialize_mei(writer),
            IncipChild::IncipCode(elem) => elem.serialize_mei(writer),
            IncipChild::IncipText(elem) => elem.serialize_mei(writer),
            // The following children need dedicated serializers - for now write empty element
            _ => {
                let name = self.element_name();
                let start = writer.start_element(name)?;
                writer.write_empty(start)?;
                Ok(())
            }
        }
    }
}

// ============================================================================
// IncipCode
// ============================================================================

impl MeiSerialize for IncipCode {
    fn element_name(&self) -> &'static str {
        "incipCode"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.internet_media.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.whitespace.collect_attributes());
        // Element-local attribute: @form
        if let Some(ref form) = self.form {
            attrs.push(("form", form.clone()));
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

impl MeiSerialize for IncipCodeChild {
    fn element_name(&self) -> &'static str {
        match self {
            IncipCodeChild::Text(_) => "#text",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            IncipCodeChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
        }
    }
}

// ============================================================================
// IncipText
// ============================================================================

impl MeiSerialize for IncipText {
    fn element_name(&self) -> &'static str {
        "incipText"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.internet_media.collect_attributes());
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

impl MeiSerialize for IncipTextChild {
    fn element_name(&self) -> &'static str {
        match self {
            IncipTextChild::P(_) => "p",
            IncipTextChild::Head(_) => "head",
            IncipTextChild::Lg(_) => "lg",
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
            IncipTextChild::P(elem) => elem.serialize_mei(writer),
            IncipTextChild::Head(elem) => elem.serialize_mei(writer),
            IncipTextChild::Lg(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Creation
// ============================================================================

impl MeiSerialize for Creation {
    fn element_name(&self) -> &'static str {
        "creation"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.datable.collect_attributes());
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

impl MeiSerialize for CreationChild {
    fn element_name(&self) -> &'static str {
        match self {
            CreationChild::Text(_) => "#text",
            CreationChild::Street(_) => "street",
            CreationChild::Symbol(_) => "symbol",
            CreationChild::GeogFeat(_) => "geogFeat",
            CreationChild::Depth(_) => "depth",
            CreationChild::Bibl(_) => "bibl",
            CreationChild::GeogName(_) => "geogName",
            CreationChild::Address(_) => "address",
            CreationChild::Catchwords(_) => "catchwords",
            CreationChild::Date(_) => "date",
            CreationChild::Locus(_) => "locus",
            CreationChild::PersName(_) => "persName",
            CreationChild::PostBox(_) => "postBox",
            CreationChild::Settlement(_) => "settlement",
            CreationChild::Stack(_) => "stack",
            CreationChild::Name(_) => "name",
            CreationChild::Stamp(_) => "stamp",
            CreationChild::StyleName(_) => "styleName",
            CreationChild::Country(_) => "country",
            CreationChild::SecFolio(_) => "secFolio",
            CreationChild::Identifier(_) => "identifier",
            CreationChild::Num(_) => "num",
            CreationChild::LocusGrp(_) => "locusGrp",
            CreationChild::Height(_) => "height",
            CreationChild::District(_) => "district",
            CreationChild::Ref(_) => "ref",
            CreationChild::Relation(_) => "relation",
            CreationChild::BiblStruct(_) => "biblStruct",
            CreationChild::PeriodName(_) => "periodName",
            CreationChild::PostCode(_) => "postCode",
            CreationChild::Heraldry(_) => "heraldry",
            CreationChild::Repository(_) => "repository",
            CreationChild::Width(_) => "width",
            CreationChild::Lb(_) => "lb",
            CreationChild::Title(_) => "title",
            CreationChild::Head(_) => "head",
            CreationChild::Abbr(_) => "abbr",
            CreationChild::Bloc(_) => "bloc",
            CreationChild::CorpName(_) => "corpName",
            CreationChild::Ptr(_) => "ptr",
            CreationChild::Extent(_) => "extent",
            CreationChild::Seg(_) => "seg",
            CreationChild::Term(_) => "term",
            CreationChild::Dimensions(_) => "dimensions",
            CreationChild::Fig(_) => "fig",
            CreationChild::Region(_) => "region",
            CreationChild::Signatures(_) => "signatures",
            CreationChild::RelationList(_) => "relationList",
            CreationChild::Rend(_) => "rend",
            CreationChild::Expan(_) => "expan",
            CreationChild::Q(_) => "q",
            CreationChild::Dim(_) => "dim",
            CreationChild::Annot(_) => "annot",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, CreationChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            CreationChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            CreationChild::Date(elem) => elem.serialize_mei(writer),
            CreationChild::PersName(elem) => elem.serialize_mei(writer),
            CreationChild::Address(elem) => elem.serialize_mei(writer),
            CreationChild::Name(elem) => elem.serialize_mei(writer),
            CreationChild::Identifier(elem) => elem.serialize_mei(writer),
            CreationChild::Lb(elem) => elem.serialize_mei(writer),
            CreationChild::Title(elem) => elem.serialize_mei(writer),
            CreationChild::Head(elem) => elem.serialize_mei(writer),
            CreationChild::CorpName(elem) => elem.serialize_mei(writer),
            CreationChild::Extent(elem) => elem.serialize_mei(writer),
            CreationChild::Rend(elem) => elem.serialize_mei(writer),
            CreationChild::Annot(elem) => elem.serialize_mei(writer),
            // The following children need dedicated serializers - for now write empty element
            _ => {
                let name = self.element_name();
                let start = writer.start_element(name)?;
                writer.write_empty(start)?;
                Ok(())
            }
        }
    }
}

// ============================================================================
// PerfMedium
// ============================================================================

impl MeiSerialize for PerfMedium {
    fn element_name(&self) -> &'static str {
        "perfMedium"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for PerfMediumChild {
    fn element_name(&self) -> &'static str {
        match self {
            PerfMediumChild::PerfResList(_) => "perfResList",
            PerfMediumChild::Annot(_) => "annot",
            PerfMediumChild::CastList(_) => "castList",
            PerfMediumChild::Head(_) => "head",
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
            PerfMediumChild::Head(elem) => elem.serialize_mei(writer),
            PerfMediumChild::Annot(elem) => elem.serialize_mei(writer),
            PerfMediumChild::PerfResList(elem) => elem.serialize_mei(writer),
            PerfMediumChild::CastList(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// PerfResList
// ============================================================================

impl MeiSerialize for PerfResList {
    fn element_name(&self) -> &'static str {
        "perfResList"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.perf_res_basic.collect_attributes());
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

impl MeiSerialize for PerfResListChild {
    fn element_name(&self) -> &'static str {
        match self {
            PerfResListChild::Head(_) => "head",
            PerfResListChild::Annot(_) => "annot",
            PerfResListChild::PerfRes(_) => "perfRes",
            PerfResListChild::PerfResList(_) => "perfResList",
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
            PerfResListChild::Head(elem) => elem.serialize_mei(writer),
            PerfResListChild::Annot(elem) => elem.serialize_mei(writer),
            PerfResListChild::PerfRes(elem) => elem.serialize_mei(writer),
            PerfResListChild::PerfResList(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// PerfRes
// ============================================================================

impl MeiSerialize for PerfRes {
    fn element_name(&self) -> &'static str {
        "perfRes"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.perf_res.collect_attributes());
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

impl MeiSerialize for PerfResChild {
    fn element_name(&self) -> &'static str {
        match self {
            PerfResChild::Text(_) => "#text",
            PerfResChild::PerfRes(_) => "perfRes",
            PerfResChild::RelationList(_) => "relationList",
            PerfResChild::Stamp(_) => "stamp",
            PerfResChild::Signatures(_) => "signatures",
            PerfResChild::StyleName(_) => "styleName",
            PerfResChild::GeogFeat(_) => "geogFeat",
            PerfResChild::LocusGrp(_) => "locusGrp",
            PerfResChild::Address(_) => "address",
            PerfResChild::Abbr(_) => "abbr",
            PerfResChild::SecFolio(_) => "secFolio",
            PerfResChild::Bloc(_) => "bloc",
            PerfResChild::Width(_) => "width",
            PerfResChild::Annot(_) => "annot",
            PerfResChild::Depth(_) => "depth",
            PerfResChild::Catchwords(_) => "catchwords",
            PerfResChild::Dimensions(_) => "dimensions",
            PerfResChild::Ptr(_) => "ptr",
            PerfResChild::Country(_) => "country",
            PerfResChild::Q(_) => "q",
            PerfResChild::Date(_) => "date",
            PerfResChild::PostCode(_) => "postCode",
            PerfResChild::Region(_) => "region",
            PerfResChild::Settlement(_) => "settlement",
            PerfResChild::BiblStruct(_) => "biblStruct",
            PerfResChild::Relation(_) => "relation",
            PerfResChild::Title(_) => "title",
            PerfResChild::Locus(_) => "locus",
            PerfResChild::PersName(_) => "persName",
            PerfResChild::PostBox(_) => "postBox",
            PerfResChild::Seg(_) => "seg",
            PerfResChild::Extent(_) => "extent",
            PerfResChild::Heraldry(_) => "heraldry",
            PerfResChild::Identifier(_) => "identifier",
            PerfResChild::Height(_) => "height",
            PerfResChild::Name(_) => "name",
            PerfResChild::PeriodName(_) => "periodName",
            PerfResChild::Ref(_) => "ref",
            PerfResChild::Expan(_) => "expan",
            PerfResChild::Num(_) => "num",
            PerfResChild::Repository(_) => "repository",
            PerfResChild::Stack(_) => "stack",
            PerfResChild::Symbol(_) => "symbol",
            PerfResChild::Rend(_) => "rend",
            PerfResChild::Term(_) => "term",
            PerfResChild::Lb(_) => "lb",
            PerfResChild::Fig(_) => "fig",
            PerfResChild::Street(_) => "street",
            PerfResChild::Bibl(_) => "bibl",
            PerfResChild::GeogName(_) => "geogName",
            PerfResChild::CorpName(_) => "corpName",
            PerfResChild::District(_) => "district",
            PerfResChild::Dim(_) => "dim",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, PerfResChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PerfResChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            PerfResChild::Annot(elem) => elem.serialize_mei(writer),
            PerfResChild::PerfRes(elem) => elem.serialize_mei(writer),
            PerfResChild::Rend(elem) => elem.serialize_mei(writer),
            PerfResChild::Lb(elem) => elem.serialize_mei(writer),
            // Most children need dedicated serializers - skip for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Classification
// ============================================================================

impl MeiSerialize for Classification {
    fn element_name(&self) -> &'static str {
        "classification"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for ClassificationChild {
    fn element_name(&self) -> &'static str {
        match self {
            ClassificationChild::TermList(_) => "termList",
            ClassificationChild::Head(_) => "head",
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
            ClassificationChild::Head(elem) => elem.serialize_mei(writer),
            ClassificationChild::TermList(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// TermList
// ============================================================================

impl MeiSerialize for TermList {
    fn element_name(&self) -> &'static str {
        "termList"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
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

impl MeiSerialize for TermListChild {
    fn element_name(&self) -> &'static str {
        match self {
            TermListChild::Head(_) => "head",
            TermListChild::Label(_) => "label",
            TermListChild::Term(_) => "term",
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
            TermListChild::Head(elem) => elem.serialize_mei(writer),
            TermListChild::Label(elem) => elem.serialize_mei(writer),
            TermListChild::Term(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Term
// ============================================================================

impl MeiSerialize for Term {
    fn element_name(&self) -> &'static str {
        "term"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for TermChild {
    fn element_name(&self) -> &'static str {
        match self {
            TermChild::Text(_) => "#text",
            TermChild::Bibl(_) => "bibl",
            TermChild::Address(_) => "address",
            TermChild::Heraldry(_) => "heraldry",
            TermChild::Dimensions(_) => "dimensions",
            TermChild::Locus(_) => "locus",
            TermChild::Fig(_) => "fig",
            TermChild::Stack(_) => "stack",
            TermChild::StyleName(_) => "styleName",
            TermChild::Settlement(_) => "settlement",
            TermChild::Stamp(_) => "stamp",
            TermChild::Ref(_) => "ref",
            TermChild::District(_) => "district",
            TermChild::CorpName(_) => "corpName",
            TermChild::PeriodName(_) => "periodName",
            TermChild::Rend(_) => "rend",
            TermChild::Symbol(_) => "symbol",
            TermChild::Extent(_) => "extent",
            TermChild::Q(_) => "q",
            TermChild::Seg(_) => "seg",
            TermChild::Abbr(_) => "abbr",
            TermChild::Num(_) => "num",
            TermChild::Ptr(_) => "ptr",
            TermChild::RelationList(_) => "relationList",
            TermChild::Signatures(_) => "signatures",
            TermChild::Depth(_) => "depth",
            TermChild::Street(_) => "street",
            TermChild::Identifier(_) => "identifier",
            TermChild::Annot(_) => "annot",
            TermChild::LocusGrp(_) => "locusGrp",
            TermChild::Term(_) => "term",
            TermChild::Width(_) => "width",
            TermChild::GeogName(_) => "geogName",
            TermChild::PersName(_) => "persName",
            TermChild::Country(_) => "country",
            TermChild::SecFolio(_) => "secFolio",
            TermChild::Catchwords(_) => "catchwords",
            TermChild::Height(_) => "height",
            TermChild::Expan(_) => "expan",
            TermChild::Lb(_) => "lb",
            TermChild::PostCode(_) => "postCode",
            TermChild::Name(_) => "name",
            TermChild::Relation(_) => "relation",
            TermChild::Date(_) => "date",
            TermChild::Repository(_) => "repository",
            TermChild::Title(_) => "title",
            TermChild::Dim(_) => "dim",
            TermChild::BiblStruct(_) => "biblStruct",
            TermChild::Bloc(_) => "bloc",
            TermChild::GeogFeat(_) => "geogFeat",
            TermChild::PostBox(_) => "postBox",
            TermChild::Region(_) => "region",
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
            TermChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            TermChild::Rend(elem) => elem.serialize_mei(writer),
            TermChild::Ref(elem) => elem.serialize_mei(writer),
            TermChild::PersName(elem) => elem.serialize_mei(writer),
            TermChild::CorpName(elem) => elem.serialize_mei(writer),
            TermChild::Name(elem) => elem.serialize_mei(writer),
            TermChild::Date(elem) => elem.serialize_mei(writer),
            TermChild::Identifier(elem) => elem.serialize_mei(writer),
            TermChild::Title(elem) => elem.serialize_mei(writer),
            TermChild::Bibl(elem) => elem.serialize_mei(writer),
            TermChild::BiblStruct(elem) => elem.serialize_mei(writer),
            TermChild::Lb(elem) => elem.serialize_mei(writer),
            TermChild::Num(elem) => elem.serialize_mei(writer),
            TermChild::Term(elem) => elem.serialize_mei(writer),
            TermChild::GeogName(elem) => elem.serialize_mei(writer),
            TermChild::Extent(elem) => elem.serialize_mei(writer),
            TermChild::Annot(elem) => elem.serialize_mei(writer),
            // Many other child types exist but are rarely used in practice.
            // Return error for unimplemented children to catch missing implementations.
            _ => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "TermChild::{}",
                self.element_name()
            ))),
        }
    }
}

// ============================================================================
// LangUsage
// ============================================================================

impl MeiSerialize for tusk_model::elements::LangUsage {
    fn element_name(&self) -> &'static str {
        "langUsage"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for tusk_model::elements::LangUsageChild {
    fn element_name(&self) -> &'static str {
        match self {
            tusk_model::elements::LangUsageChild::Head(_) => "head",
            tusk_model::elements::LangUsageChild::Language(_) => "language",
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
            tusk_model::elements::LangUsageChild::Head(elem) => elem.serialize_mei(writer),
            tusk_model::elements::LangUsageChild::Language(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Language
// ============================================================================

impl MeiSerialize for tusk_model::elements::Language {
    fn element_name(&self) -> &'static str {
        "language"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for tusk_model::elements::LanguageChild {
    fn element_name(&self) -> &'static str {
        match self {
            tusk_model::elements::LanguageChild::Text(_) => "#text",
            tusk_model::elements::LanguageChild::Settlement(_) => "settlement",
            tusk_model::elements::LanguageChild::Signatures(_) => "signatures",
            tusk_model::elements::LanguageChild::Seg(_) => "seg",
            tusk_model::elements::LanguageChild::CorpName(_) => "corpName",
            tusk_model::elements::LanguageChild::Lb(_) => "lb",
            tusk_model::elements::LanguageChild::Rend(_) => "rend",
            tusk_model::elements::LanguageChild::SecFolio(_) => "secFolio",
            tusk_model::elements::LanguageChild::StyleName(_) => "styleName",
            tusk_model::elements::LanguageChild::Symbol(_) => "symbol",
            tusk_model::elements::LanguageChild::Dim(_) => "dim",
            tusk_model::elements::LanguageChild::Width(_) => "width",
            tusk_model::elements::LanguageChild::Expan(_) => "expan",
            tusk_model::elements::LanguageChild::Extent(_) => "extent",
            tusk_model::elements::LanguageChild::Bibl(_) => "bibl",
            tusk_model::elements::LanguageChild::PersName(_) => "persName",
            tusk_model::elements::LanguageChild::Height(_) => "height",
            tusk_model::elements::LanguageChild::Heraldry(_) => "heraldry",
            tusk_model::elements::LanguageChild::PostCode(_) => "postCode",
            tusk_model::elements::LanguageChild::Stack(_) => "stack",
            tusk_model::elements::LanguageChild::Depth(_) => "depth",
            tusk_model::elements::LanguageChild::GeogFeat(_) => "geogFeat",
            tusk_model::elements::LanguageChild::Name(_) => "name",
            tusk_model::elements::LanguageChild::Address(_) => "address",
            tusk_model::elements::LanguageChild::Q(_) => "q",
            tusk_model::elements::LanguageChild::Ref(_) => "ref",
            tusk_model::elements::LanguageChild::Term(_) => "term",
            tusk_model::elements::LanguageChild::Relation(_) => "relation",
            tusk_model::elements::LanguageChild::PeriodName(_) => "periodName",
            tusk_model::elements::LanguageChild::RelationList(_) => "relationList",
            tusk_model::elements::LanguageChild::Date(_) => "date",
            tusk_model::elements::LanguageChild::Fig(_) => "fig",
            tusk_model::elements::LanguageChild::PostBox(_) => "postBox",
            tusk_model::elements::LanguageChild::Identifier(_) => "identifier",
            tusk_model::elements::LanguageChild::Ptr(_) => "ptr",
            tusk_model::elements::LanguageChild::Region(_) => "region",
            tusk_model::elements::LanguageChild::Title(_) => "title",
            tusk_model::elements::LanguageChild::Bloc(_) => "bloc",
            tusk_model::elements::LanguageChild::District(_) => "district",
            tusk_model::elements::LanguageChild::BiblStruct(_) => "biblStruct",
            tusk_model::elements::LanguageChild::Catchwords(_) => "catchwords",
            tusk_model::elements::LanguageChild::Num(_) => "num",
            tusk_model::elements::LanguageChild::Country(_) => "country",
            tusk_model::elements::LanguageChild::Dimensions(_) => "dimensions",
            tusk_model::elements::LanguageChild::Repository(_) => "repository",
            tusk_model::elements::LanguageChild::Abbr(_) => "abbr",
            tusk_model::elements::LanguageChild::Stamp(_) => "stamp",
            tusk_model::elements::LanguageChild::GeogName(_) => "geogName",
            tusk_model::elements::LanguageChild::LocusGrp(_) => "locusGrp",
            tusk_model::elements::LanguageChild::Locus(_) => "locus",
            tusk_model::elements::LanguageChild::Street(_) => "street",
            tusk_model::elements::LanguageChild::Annot(_) => "annot",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, tusk_model::elements::LanguageChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            tusk_model::elements::LanguageChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            tusk_model::elements::LanguageChild::Rend(elem) => elem.serialize_mei(writer),
            tusk_model::elements::LanguageChild::Ref(elem) => elem.serialize_mei(writer),
            tusk_model::elements::LanguageChild::PersName(elem) => elem.serialize_mei(writer),
            tusk_model::elements::LanguageChild::CorpName(elem) => elem.serialize_mei(writer),
            tusk_model::elements::LanguageChild::Name(elem) => elem.serialize_mei(writer),
            tusk_model::elements::LanguageChild::Date(elem) => elem.serialize_mei(writer),
            tusk_model::elements::LanguageChild::Identifier(elem) => elem.serialize_mei(writer),
            tusk_model::elements::LanguageChild::Title(elem) => elem.serialize_mei(writer),
            tusk_model::elements::LanguageChild::Bibl(elem) => elem.serialize_mei(writer),
            tusk_model::elements::LanguageChild::BiblStruct(elem) => elem.serialize_mei(writer),
            tusk_model::elements::LanguageChild::Lb(elem) => elem.serialize_mei(writer),
            tusk_model::elements::LanguageChild::Num(elem) => elem.serialize_mei(writer),
            tusk_model::elements::LanguageChild::Term(elem) => elem.serialize_mei(writer),
            tusk_model::elements::LanguageChild::GeogName(elem) => elem.serialize_mei(writer),
            tusk_model::elements::LanguageChild::Extent(elem) => elem.serialize_mei(writer),
            tusk_model::elements::LanguageChild::Annot(elem) => elem.serialize_mei(writer),
            // Many other child types exist but are rarely used in practice.
            // Return error for unimplemented children to catch missing implementations.
            _ => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "LanguageChild::{}",
                self.element_name()
            ))),
        }
    }
}

// ============================================================================
// CastList
// ============================================================================

impl MeiSerialize for CastList {
    fn element_name(&self) -> &'static str {
        "castList"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for CastListChild {
    fn element_name(&self) -> &'static str {
        match self {
            CastListChild::Head(_) => "head",
            CastListChild::CastItem(_) => "castItem",
            CastListChild::CastGrp(_) => "castGrp",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            CastListChild::Head(elem) => elem.collect_all_attributes(),
            CastListChild::CastItem(elem) => elem.collect_all_attributes(),
            CastListChild::CastGrp(elem) => elem.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            CastListChild::Head(elem) => elem.serialize_mei(writer),
            CastListChild::CastItem(elem) => elem.serialize_mei(writer),
            CastListChild::CastGrp(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// CastItem
// ============================================================================

impl MeiSerialize for CastItem {
    fn element_name(&self) -> &'static str {
        "castItem"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for CastItemChild {
    fn element_name(&self) -> &'static str {
        match self {
            CastItemChild::Text(_) => "$text",
            CastItemChild::Actor(_) => "actor",
            CastItemChild::RoleDesc(_) => "roleDesc",
            CastItemChild::PerfRes(_) => "perfRes",
            CastItemChild::Role(_) => "role",
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
            CastItemChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            CastItemChild::RoleDesc(elem) => elem.serialize_mei(writer),
            CastItemChild::PerfRes(elem) => elem.serialize_mei(writer),
            CastItemChild::Actor(_) | CastItemChild::Role(_) => {
                // Not fully implemented yet - write empty element
                let name = self.element_name();
                let start = writer.start_element(name)?;
                writer.write_empty(start)?;
                Ok(())
            }
        }
    }
}

// ============================================================================
// CastGrp
// ============================================================================

impl MeiSerialize for CastGrp {
    fn element_name(&self) -> &'static str {
        "castGrp"
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

impl MeiSerialize for CastGrpChild {
    fn element_name(&self) -> &'static str {
        match self {
            CastGrpChild::CastGrp(_) => "castGrp",
            CastGrpChild::RoleDesc(_) => "roleDesc",
            CastGrpChild::CastItem(_) => "castItem",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            CastGrpChild::CastGrp(elem) => elem.collect_all_attributes(),
            CastGrpChild::RoleDesc(elem) => elem.collect_all_attributes(),
            CastGrpChild::CastItem(elem) => elem.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            CastGrpChild::CastGrp(elem) => elem.serialize_mei(writer),
            CastGrpChild::RoleDesc(elem) => elem.serialize_mei(writer),
            CastGrpChild::CastItem(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// RoleDesc
// ============================================================================

impl MeiSerialize for RoleDesc {
    fn element_name(&self) -> &'static str {
        "roleDesc"
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

impl MeiSerialize for RoleDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            RoleDescChild::Text(_) => "$text",
            _ => "unknown",
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
            RoleDescChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            _ => {
                // Not fully implemented yet - write empty element
                let name = self.element_name();
                let start = writer.start_element(name)?;
                writer.write_empty(start)?;
                Ok(())
            }
        }
    }
}
