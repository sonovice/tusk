//! Serializer implementations for revision tracking elements.
//!
//! Contains: RevisionDesc, Change, ChangeDesc, Date, P.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    Change, ChangeChild, ChangeDesc, ChangeDescChild, Date, DateChild, P, PChild, RevisionDesc,
    RevisionDescChild,
};

// ============================================================================
// RevisionDesc
// ============================================================================

impl MeiSerialize for RevisionDesc {
    fn element_name(&self) -> &'static str {
        "revisionDesc"
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

impl MeiSerialize for RevisionDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            RevisionDescChild::Head(_) => "head",
            RevisionDescChild::Change(_) => "change",
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
            RevisionDescChild::Head(elem) => elem.serialize_mei(writer),
            RevisionDescChild::Change(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Change
// ============================================================================

impl MeiSerialize for Change {
    fn element_name(&self) -> &'static str {
        "change"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.datable.collect_attributes());
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

impl MeiSerialize for ChangeChild {
    fn element_name(&self) -> &'static str {
        match self {
            ChangeChild::Date(_) => "date",
            ChangeChild::ChangeDesc(_) => "changeDesc",
            ChangeChild::RespStmt(_) => "respStmt",
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
            ChangeChild::Date(elem) => elem.serialize_mei(writer),
            ChangeChild::ChangeDesc(elem) => elem.serialize_mei(writer),
            ChangeChild::RespStmt(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// ChangeDesc
// ============================================================================

impl MeiSerialize for ChangeDesc {
    fn element_name(&self) -> &'static str {
        "changeDesc"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
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

impl MeiSerialize for ChangeDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            ChangeDescChild::P(_) => "p",
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
            ChangeDescChild::P(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Date
// ============================================================================

impl MeiSerialize for Date {
    fn element_name(&self) -> &'static str {
        "date"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.calendared.collect_attributes());
        attrs.extend(self.datable.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
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

impl MeiSerialize for DateChild {
    fn element_name(&self) -> &'static str {
        match self {
            DateChild::Text(_) => "#text",
            _ => "unknown",
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
            DateChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            _ => Ok(()), // Other children skipped for now
        }
    }
}

// ============================================================================
// P (paragraph)
// ============================================================================

impl MeiSerialize for P {
    fn element_name(&self) -> &'static str {
        "p"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
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

impl MeiSerialize for PChild {
    fn element_name(&self) -> &'static str {
        match self {
            PChild::Text(_) => "#text",
            PChild::Region(_) => "region",
            PChild::Stack(_) => "stack",
            PChild::Subst(_) => "subst",
            PChild::Symbol(_) => "symbol",
            PChild::Extent(_) => "extent",
            PChild::Dimensions(_) => "dimensions",
            PChild::Catchwords(_) => "catchwords",
            PChild::Annot(_) => "annot",
            PChild::PostBox(_) => "postBox",
            PChild::Corr(_) => "corr",
            PChild::Del(_) => "del",
            PChild::Ref(_) => "ref",
            PChild::Date(_) => "date",
            PChild::Restore(_) => "restore",
            PChild::District(_) => "district",
            PChild::Locus(_) => "locus",
            PChild::StageDir(_) => "stageDir",
            PChild::Address(_) => "address",
            PChild::PeriodName(_) => "periodName",
            PChild::Table(_) => "table",
            PChild::Sic(_) => "sic",
            PChild::Stamp(_) => "stamp",
            PChild::Relation(_) => "relation",
            PChild::Expan(_) => "expan",
            PChild::GeogName(_) => "geogName",
            PChild::Dim(_) => "dim",
            PChild::Name(_) => "name",
            PChild::Heraldry(_) => "heraldry",
            PChild::CorpName(_) => "corpName",
            PChild::CastList(_) => "castList",
            PChild::Choice(_) => "choice",
            PChild::Identifier(_) => "identifier",
            PChild::RelationList(_) => "relationList",
            PChild::Lg(_) => "lg",
            PChild::Country(_) => "country",
            PChild::List(_) => "list",
            PChild::Bibl(_) => "bibl",
            PChild::LocusGrp(_) => "locusGrp",
            PChild::HandShift(_) => "handShift",
            PChild::Lb(_) => "lb",
            PChild::GeogFeat(_) => "geogFeat",
            PChild::Pb(_) => "pb",
            PChild::Reg(_) => "reg",
            PChild::PersName(_) => "persName",
            PChild::Orig(_) => "orig",
            PChild::Width(_) => "width",
            PChild::Street(_) => "street",
            PChild::Term(_) => "term",
            PChild::Unclear(_) => "unclear",
            PChild::PostCode(_) => "postCode",
            PChild::BiblStruct(_) => "biblStruct",
            PChild::Fig(_) => "fig",
            PChild::Damage(_) => "damage",
            PChild::Abbr(_) => "abbr",
            PChild::Title(_) => "title",
            PChild::Height(_) => "height",
            PChild::Num(_) => "num",
            PChild::Gap(_) => "gap",
            PChild::SecFolio(_) => "secFolio",
            PChild::Add(_) => "add",
            PChild::Q(_) => "q",
            PChild::Rend(_) => "rend",
            PChild::Supplied(_) => "supplied",
            PChild::Signatures(_) => "signatures",
            PChild::Repository(_) => "repository",
            PChild::EventList(_) => "eventList",
            PChild::Settlement(_) => "settlement",
            PChild::BiblList(_) => "biblList",
            PChild::Quote(_) => "quote",
            PChild::Ptr(_) => "ptr",
            PChild::Seg(_) => "seg",
            PChild::StyleName(_) => "styleName",
            PChild::Bloc(_) => "bloc",
            PChild::Depth(_) => "depth",
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
            PChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            // Elements with existing serializers
            PChild::Ref(elem) => elem.serialize_mei(writer),
            PChild::Date(elem) => elem.serialize_mei(writer),
            PChild::Address(elem) => elem.serialize_mei(writer),
            PChild::PersName(elem) => elem.serialize_mei(writer),
            PChild::CorpName(elem) => elem.serialize_mei(writer),
            PChild::Name(elem) => elem.serialize_mei(writer),
            PChild::Identifier(elem) => elem.serialize_mei(writer),
            PChild::Lb(elem) => elem.serialize_mei(writer),
            PChild::Rend(elem) => elem.serialize_mei(writer),
            PChild::Title(elem) => elem.serialize_mei(writer),
            PChild::Num(elem) => elem.serialize_mei(writer),
            PChild::Ptr(elem) => elem.serialize_mei(writer),
            PChild::Annot(elem) => elem.serialize_mei(writer),
            PChild::Extent(elem) => elem.serialize_mei(writer),
            PChild::Region(elem) => elem.serialize_mei(writer),
            PChild::PostBox(elem) => elem.serialize_mei(writer),
            PChild::PostCode(elem) => elem.serialize_mei(writer),
            PChild::District(elem) => elem.serialize_mei(writer),
            PChild::GeogName(elem) => elem.serialize_mei(writer),
            PChild::GeogFeat(elem) => elem.serialize_mei(writer),
            PChild::Country(elem) => elem.serialize_mei(writer),
            PChild::Settlement(elem) => elem.serialize_mei(writer),
            PChild::Street(elem) => elem.serialize_mei(writer),
            PChild::Bloc(elem) => elem.serialize_mei(writer),
            // Elements that need serializers - for now use default element serialization
            _ => {
                // TODO: Implement serializers for remaining PChild variants
                // For now, we skip unimplemented children with a warning
                Ok(())
            }
        }
    }
}
