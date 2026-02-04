//! Serializer implementations for work-specific child elements.
//!
//! Contains: AltId, ExtMeta, Key, Meter, Incip, Creation, PerfMedium, Classification.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    AltId, AltIdChild, Classification, ClassificationChild, Creation, CreationChild, ExtMeta,
    ExtMetaChild, Incip, IncipChild, Key, KeyChild, Meter, MeterChild, PerfMedium, PerfMediumChild,
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
            // TermList needs dedicated serializer - for now write empty element
            ClassificationChild::TermList(_) => {
                let start = writer.start_element("termList")?;
                writer.write_empty(start)?;
                Ok(())
            }
        }
    }
}
