//! Serializer implementations for publication statement elements.
//!
//! Contains: PubStmt, Publisher, Address, PubPlace, Availability, Identifier, Distributor, Unpub.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    AccessRestrict, AccessRestrictChild, Address, AddressChild, Availability, AvailabilityChild,
    Distributor, DistributorChild, Identifier, IdentifierChild, Price, PriceChild, PubPlace,
    PubPlaceChild, PubStmt, PubStmtChild, Publisher, PublisherChild, SysReq, SysReqChild, Unpub,
    UnpubChild, UseRestrict, UseRestrictChild,
};

// ============================================================================
// PubStmt
// ============================================================================

impl MeiSerialize for PubStmt {
    fn element_name(&self) -> &'static str {
        "pubStmt"
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

impl MeiSerialize for PubStmtChild {
    fn element_name(&self) -> &'static str {
        match self {
            PubStmtChild::Date(_) => "date",
            PubStmtChild::Publisher(_) => "publisher",
            PubStmtChild::Address(_) => "address",
            PubStmtChild::PubPlace(_) => "pubPlace",
            PubStmtChild::RespStmt(_) => "respStmt",
            PubStmtChild::Availability(_) => "availability",
            PubStmtChild::Identifier(_) => "identifier",
            PubStmtChild::Distributor(_) => "distributor",
            PubStmtChild::Head(_) => "head",
            PubStmtChild::Unpub(_) => "unpub",
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
            PubStmtChild::Date(elem) => elem.serialize_mei(writer),
            PubStmtChild::Head(elem) => elem.serialize_mei(writer),
            PubStmtChild::Publisher(elem) => elem.serialize_mei(writer),
            PubStmtChild::Address(elem) => elem.serialize_mei(writer),
            PubStmtChild::PubPlace(elem) => elem.serialize_mei(writer),
            PubStmtChild::RespStmt(elem) => elem.serialize_mei(writer),
            PubStmtChild::Availability(elem) => elem.serialize_mei(writer),
            PubStmtChild::Identifier(elem) => elem.serialize_mei(writer),
            PubStmtChild::Distributor(elem) => elem.serialize_mei(writer),
            PubStmtChild::Unpub(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Publisher
// ============================================================================

impl MeiSerialize for Publisher {
    fn element_name(&self) -> &'static str {
        "publisher"
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

impl MeiSerialize for PublisherChild {
    fn element_name(&self) -> &'static str {
        match self {
            PublisherChild::Text(_) => "#text",
            PublisherChild::Date(_) => "date",
            PublisherChild::Name(_) => "name",
            PublisherChild::PersName(_) => "persName",
            PublisherChild::CorpName(_) => "corpName",
            PublisherChild::Address(_) => "address",
            PublisherChild::Identifier(_) => "identifier",
            PublisherChild::Rend(_) => "rend",
            PublisherChild::Lb(_) => "lb",
            PublisherChild::Title(_) => "title",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, PublisherChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PublisherChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            PublisherChild::Date(elem) => elem.serialize_mei(writer),
            PublisherChild::Name(elem) => elem.serialize_mei(writer),
            PublisherChild::PersName(elem) => elem.serialize_mei(writer),
            PublisherChild::CorpName(elem) => elem.serialize_mei(writer),
            PublisherChild::Address(elem) => elem.serialize_mei(writer),
            PublisherChild::Identifier(elem) => elem.serialize_mei(writer),
            PublisherChild::Rend(elem) => elem.serialize_mei(writer),
            PublisherChild::Lb(elem) => elem.serialize_mei(writer),
            PublisherChild::Title(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "PublisherChild::{}",
                self.element_name()
            ))),
        }
    }
}

// ============================================================================
// Address
// ============================================================================

impl MeiSerialize for Address {
    fn element_name(&self) -> &'static str {
        "address"
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

impl MeiSerialize for AddressChild {
    fn element_name(&self) -> &'static str {
        match self {
            AddressChild::Settlement(_) => "settlement",
            AddressChild::Country(_) => "country",
            AddressChild::PostCode(_) => "postCode",
            AddressChild::Street(_) => "street",
            AddressChild::District(_) => "district",
            AddressChild::Bloc(_) => "bloc",
            AddressChild::GeogFeat(_) => "geogFeat",
            AddressChild::PostBox(_) => "postBox",
            AddressChild::Region(_) => "region",
            AddressChild::AddrLine(_) => "addrLine",
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
            AddressChild::Settlement(elem) => elem.serialize_mei(writer),
            AddressChild::Country(elem) => elem.serialize_mei(writer),
            AddressChild::PostCode(elem) => elem.serialize_mei(writer),
            AddressChild::Street(elem) => elem.serialize_mei(writer),
            AddressChild::District(elem) => elem.serialize_mei(writer),
            AddressChild::Bloc(elem) => elem.serialize_mei(writer),
            AddressChild::GeogFeat(elem) => elem.serialize_mei(writer),
            AddressChild::PostBox(elem) => elem.serialize_mei(writer),
            AddressChild::Region(elem) => elem.serialize_mei(writer),
            AddressChild::AddrLine(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// PubPlace
// ============================================================================

impl MeiSerialize for PubPlace {
    fn element_name(&self) -> &'static str {
        "pubPlace"
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

impl MeiSerialize for PubPlaceChild {
    fn element_name(&self) -> &'static str {
        match self {
            PubPlaceChild::Text(_) => "#text",
            PubPlaceChild::Date(_) => "date",
            PubPlaceChild::Name(_) => "name",
            PubPlaceChild::PersName(_) => "persName",
            PubPlaceChild::CorpName(_) => "corpName",
            PubPlaceChild::Address(_) => "address",
            PubPlaceChild::Identifier(_) => "identifier",
            PubPlaceChild::Rend(_) => "rend",
            PubPlaceChild::Lb(_) => "lb",
            PubPlaceChild::Title(_) => "title",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, PubPlaceChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PubPlaceChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            PubPlaceChild::Date(elem) => elem.serialize_mei(writer),
            PubPlaceChild::Name(elem) => elem.serialize_mei(writer),
            PubPlaceChild::PersName(elem) => elem.serialize_mei(writer),
            PubPlaceChild::CorpName(elem) => elem.serialize_mei(writer),
            PubPlaceChild::Address(elem) => elem.serialize_mei(writer),
            PubPlaceChild::Identifier(elem) => elem.serialize_mei(writer),
            PubPlaceChild::Rend(elem) => elem.serialize_mei(writer),
            PubPlaceChild::Lb(elem) => elem.serialize_mei(writer),
            PubPlaceChild::Title(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "PubPlaceChild::{}",
                self.element_name()
            ))),
        }
    }
}

// ============================================================================
// Availability
// ============================================================================

impl MeiSerialize for Availability {
    fn element_name(&self) -> &'static str {
        "availability"
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

impl MeiSerialize for AvailabilityChild {
    fn element_name(&self) -> &'static str {
        match self {
            AvailabilityChild::Text(_) => "#text",
            AvailabilityChild::SysReq(_) => "sysReq",
            AvailabilityChild::Distributor(_) => "distributor",
            AvailabilityChild::Price(_) => "price",
            AvailabilityChild::Date(_) => "date",
            AvailabilityChild::Identifier(_) => "identifier",
            AvailabilityChild::AccessRestrict(_) => "accessRestrict",
            AvailabilityChild::Address(_) => "address",
            AvailabilityChild::Head(_) => "head",
            AvailabilityChild::UseRestrict(_) => "useRestrict",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, AvailabilityChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            AvailabilityChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            AvailabilityChild::SysReq(elem) => elem.serialize_mei(writer),
            AvailabilityChild::Distributor(elem) => elem.serialize_mei(writer),
            AvailabilityChild::Price(elem) => elem.serialize_mei(writer),
            AvailabilityChild::Date(elem) => elem.serialize_mei(writer),
            AvailabilityChild::Identifier(elem) => elem.serialize_mei(writer),
            AvailabilityChild::AccessRestrict(elem) => elem.serialize_mei(writer),
            AvailabilityChild::Address(elem) => elem.serialize_mei(writer),
            AvailabilityChild::Head(elem) => elem.serialize_mei(writer),
            AvailabilityChild::UseRestrict(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Identifier
// ============================================================================

impl MeiSerialize for Identifier {
    fn element_name(&self) -> &'static str {
        "identifier"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for IdentifierChild {
    fn element_name(&self) -> &'static str {
        match self {
            IdentifierChild::Text(_) => "#text",
            IdentifierChild::RelationList(_) => "relationList",
            IdentifierChild::Del(_) => "del",
            IdentifierChild::Stack(_) => "stack",
            IdentifierChild::Ref(_) => "ref",
            IdentifierChild::Ptr(_) => "ptr",
            IdentifierChild::Damage(_) => "damage",
            IdentifierChild::Bibl(_) => "bibl",
            IdentifierChild::Dim(_) => "dim",
            IdentifierChild::Locus(_) => "locus",
            IdentifierChild::Annot(_) => "annot",
            IdentifierChild::PostBox(_) => "postBox",
            IdentifierChild::PersName(_) => "persName",
            IdentifierChild::Region(_) => "region",
            IdentifierChild::PeriodName(_) => "periodName",
            IdentifierChild::Rend(_) => "rend",
            IdentifierChild::Choice(_) => "choice",
            IdentifierChild::Add(_) => "add",
            IdentifierChild::Reg(_) => "reg",
            IdentifierChild::PostCode(_) => "postCode",
            IdentifierChild::BiblStruct(_) => "biblStruct",
            IdentifierChild::Repository(_) => "repository",
            IdentifierChild::Settlement(_) => "settlement",
            IdentifierChild::Bloc(_) => "bloc",
            IdentifierChild::Height(_) => "height",
            IdentifierChild::Expan(_) => "expan",
            IdentifierChild::Q(_) => "q",
            IdentifierChild::Street(_) => "street",
            IdentifierChild::Corr(_) => "corr",
            IdentifierChild::Dimensions(_) => "dimensions",
            IdentifierChild::GeogFeat(_) => "geogFeat",
            IdentifierChild::Gap(_) => "gap",
            IdentifierChild::Depth(_) => "depth",
            IdentifierChild::Catchwords(_) => "catchwords",
            IdentifierChild::StyleName(_) => "styleName",
            IdentifierChild::Width(_) => "width",
            IdentifierChild::Heraldry(_) => "heraldry",
            IdentifierChild::Name(_) => "name",
            IdentifierChild::Identifier(_) => "identifier",
            IdentifierChild::Title(_) => "title",
            IdentifierChild::Orig(_) => "orig",
            IdentifierChild::Relation(_) => "relation",
            IdentifierChild::Supplied(_) => "supplied",
            IdentifierChild::Lb(_) => "lb",
            IdentifierChild::Restore(_) => "restore",
            IdentifierChild::Term(_) => "term",
            IdentifierChild::HandShift(_) => "handShift",
            IdentifierChild::Signatures(_) => "signatures",
            IdentifierChild::Date(_) => "date",
            IdentifierChild::Country(_) => "country",
            IdentifierChild::Unclear(_) => "unclear",
            IdentifierChild::CorpName(_) => "corpName",
            IdentifierChild::Fig(_) => "fig",
            IdentifierChild::Extent(_) => "extent",
            IdentifierChild::Symbol(_) => "symbol",
            IdentifierChild::Pb(_) => "pb",
            IdentifierChild::Abbr(_) => "abbr",
            IdentifierChild::LocusGrp(_) => "locusGrp",
            IdentifierChild::District(_) => "district",
            IdentifierChild::Sic(_) => "sic",
            IdentifierChild::Subst(_) => "subst",
            IdentifierChild::GeogName(_) => "geogName",
            IdentifierChild::Address(_) => "address",
            IdentifierChild::Seg(_) => "seg",
            IdentifierChild::SecFolio(_) => "secFolio",
            IdentifierChild::Num(_) => "num",
            IdentifierChild::Stamp(_) => "stamp",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, IdentifierChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            IdentifierChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            // Elements with serializers
            IdentifierChild::Address(elem) => elem.serialize_mei(writer),
            IdentifierChild::Annot(elem) => elem.serialize_mei(writer),
            IdentifierChild::Bibl(elem) => elem.serialize_mei(writer),
            IdentifierChild::BiblStruct(elem) => elem.serialize_mei(writer),
            IdentifierChild::Bloc(elem) => elem.serialize_mei(writer),
            IdentifierChild::CorpName(elem) => elem.serialize_mei(writer),
            IdentifierChild::Country(elem) => elem.serialize_mei(writer),
            IdentifierChild::Date(elem) => elem.serialize_mei(writer),
            IdentifierChild::District(elem) => elem.serialize_mei(writer),
            IdentifierChild::Extent(elem) => elem.serialize_mei(writer),
            IdentifierChild::GeogFeat(elem) => elem.serialize_mei(writer),
            IdentifierChild::GeogName(elem) => elem.serialize_mei(writer),
            IdentifierChild::Identifier(elem) => elem.serialize_mei(writer),
            IdentifierChild::Lb(elem) => elem.serialize_mei(writer),
            IdentifierChild::Name(elem) => elem.serialize_mei(writer),
            IdentifierChild::Num(elem) => elem.serialize_mei(writer),
            IdentifierChild::PersName(elem) => elem.serialize_mei(writer),
            IdentifierChild::PostBox(elem) => elem.serialize_mei(writer),
            IdentifierChild::PostCode(elem) => elem.serialize_mei(writer),
            IdentifierChild::Ptr(elem) => elem.serialize_mei(writer),
            IdentifierChild::Ref(elem) => elem.serialize_mei(writer),
            IdentifierChild::Region(elem) => elem.serialize_mei(writer),
            IdentifierChild::Rend(elem) => elem.serialize_mei(writer),
            IdentifierChild::Settlement(elem) => elem.serialize_mei(writer),
            IdentifierChild::Street(elem) => elem.serialize_mei(writer),
            IdentifierChild::Term(elem) => elem.serialize_mei(writer),
            IdentifierChild::Title(elem) => elem.serialize_mei(writer),
            // Editorial and other elements that need serializers
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "IdentifierChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Distributor
// ============================================================================

impl MeiSerialize for Distributor {
    fn element_name(&self) -> &'static str {
        "distributor"
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

impl MeiSerialize for DistributorChild {
    fn element_name(&self) -> &'static str {
        match self {
            DistributorChild::Text(_) => "#text",
            DistributorChild::Date(_) => "date",
            DistributorChild::Name(_) => "name",
            DistributorChild::PersName(_) => "persName",
            DistributorChild::CorpName(_) => "corpName",
            DistributorChild::Address(_) => "address",
            DistributorChild::Identifier(_) => "identifier",
            DistributorChild::Rend(_) => "rend",
            DistributorChild::Lb(_) => "lb",
            DistributorChild::Title(_) => "title",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, DistributorChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            DistributorChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            DistributorChild::Date(elem) => elem.serialize_mei(writer),
            DistributorChild::Name(elem) => elem.serialize_mei(writer),
            DistributorChild::PersName(elem) => elem.serialize_mei(writer),
            DistributorChild::CorpName(elem) => elem.serialize_mei(writer),
            DistributorChild::Address(elem) => elem.serialize_mei(writer),
            DistributorChild::Identifier(elem) => elem.serialize_mei(writer),
            DistributorChild::Rend(elem) => elem.serialize_mei(writer),
            DistributorChild::Lb(elem) => elem.serialize_mei(writer),
            DistributorChild::Title(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "DistributorChild::{}",
                self.element_name()
            ))),
        }
    }
}

// ============================================================================
// Unpub
// ============================================================================

impl MeiSerialize for Unpub {
    fn element_name(&self) -> &'static str {
        "unpub"
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

impl MeiSerialize for UnpubChild {
    fn element_name(&self) -> &'static str {
        match self {
            UnpubChild::Text(_) => "#text",
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
            UnpubChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
        }
    }
}

// ============================================================================
// UseRestrict
// ============================================================================

impl MeiSerialize for UseRestrict {
    fn element_name(&self) -> &'static str {
        "useRestrict"
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

impl MeiSerialize for UseRestrictChild {
    fn element_name(&self) -> &'static str {
        match self {
            UseRestrictChild::Text(_) => "#text",
            UseRestrictChild::PersName(_) => "persName",
            UseRestrictChild::CorpName(_) => "corpName",
            UseRestrictChild::District(_) => "district",
            UseRestrictChild::Region(_) => "region",
            UseRestrictChild::Dimensions(_) => "dimensions",
            UseRestrictChild::Street(_) => "street",
            UseRestrictChild::Lb(_) => "lb",
            UseRestrictChild::Signatures(_) => "signatures",
            UseRestrictChild::GeogFeat(_) => "geogFeat",
            UseRestrictChild::PeriodName(_) => "periodName",
            UseRestrictChild::RelationList(_) => "relationList",
            UseRestrictChild::Catchwords(_) => "catchwords",
            UseRestrictChild::PostBox(_) => "postBox",
            UseRestrictChild::Address(_) => "address",
            UseRestrictChild::Height(_) => "height",
            UseRestrictChild::Annot(_) => "annot",
            UseRestrictChild::Symbol(_) => "symbol",
            UseRestrictChild::Relation(_) => "relation",
            UseRestrictChild::Country(_) => "country",
            UseRestrictChild::Dim(_) => "dim",
            UseRestrictChild::Fig(_) => "fig",
            UseRestrictChild::Q(_) => "q",
            UseRestrictChild::Stamp(_) => "stamp",
            UseRestrictChild::PostCode(_) => "postCode",
            UseRestrictChild::Width(_) => "width",
            UseRestrictChild::LocusGrp(_) => "locusGrp",
            UseRestrictChild::Ptr(_) => "ptr",
            UseRestrictChild::Ref(_) => "ref",
            UseRestrictChild::Head(_) => "head",
            UseRestrictChild::Repository(_) => "repository",
            UseRestrictChild::P(_) => "p",
            UseRestrictChild::Seg(_) => "seg",
            UseRestrictChild::Settlement(_) => "settlement",
            UseRestrictChild::Term(_) => "term",
            UseRestrictChild::Rend(_) => "rend",
            UseRestrictChild::Title(_) => "title",
            UseRestrictChild::BiblStruct(_) => "biblStruct",
            UseRestrictChild::Depth(_) => "depth",
            UseRestrictChild::Num(_) => "num",
            UseRestrictChild::Extent(_) => "extent",
            UseRestrictChild::SecFolio(_) => "secFolio",
            UseRestrictChild::Stack(_) => "stack",
            UseRestrictChild::Identifier(_) => "identifier",
            UseRestrictChild::Bibl(_) => "bibl",
            UseRestrictChild::Date(_) => "date",
            UseRestrictChild::Bloc(_) => "bloc",
            UseRestrictChild::Heraldry(_) => "heraldry",
            UseRestrictChild::StyleName(_) => "styleName",
            UseRestrictChild::Abbr(_) => "abbr",
            UseRestrictChild::Name(_) => "name",
            UseRestrictChild::Expan(_) => "expan",
            UseRestrictChild::Locus(_) => "locus",
            UseRestrictChild::GeogName(_) => "geogName",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, UseRestrictChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            UseRestrictChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            UseRestrictChild::PersName(elem) => elem.serialize_mei(writer),
            UseRestrictChild::CorpName(elem) => elem.serialize_mei(writer),
            UseRestrictChild::District(elem) => elem.serialize_mei(writer),
            UseRestrictChild::Region(elem) => elem.serialize_mei(writer),
            UseRestrictChild::Street(elem) => elem.serialize_mei(writer),
            UseRestrictChild::Lb(elem) => elem.serialize_mei(writer),
            UseRestrictChild::GeogFeat(elem) => elem.serialize_mei(writer),
            UseRestrictChild::PostBox(elem) => elem.serialize_mei(writer),
            UseRestrictChild::Address(elem) => elem.serialize_mei(writer),
            UseRestrictChild::Annot(elem) => elem.serialize_mei(writer),
            UseRestrictChild::Country(elem) => elem.serialize_mei(writer),
            UseRestrictChild::PostCode(elem) => elem.serialize_mei(writer),
            UseRestrictChild::Ptr(elem) => elem.serialize_mei(writer),
            UseRestrictChild::Ref(elem) => elem.serialize_mei(writer),
            UseRestrictChild::Head(elem) => elem.serialize_mei(writer),
            UseRestrictChild::P(elem) => elem.serialize_mei(writer),
            UseRestrictChild::Settlement(elem) => elem.serialize_mei(writer),
            UseRestrictChild::Rend(elem) => elem.serialize_mei(writer),
            UseRestrictChild::Title(elem) => elem.serialize_mei(writer),
            UseRestrictChild::Num(elem) => elem.serialize_mei(writer),
            UseRestrictChild::Extent(elem) => elem.serialize_mei(writer),
            UseRestrictChild::Identifier(elem) => elem.serialize_mei(writer),
            UseRestrictChild::Date(elem) => elem.serialize_mei(writer),
            UseRestrictChild::Bloc(elem) => elem.serialize_mei(writer),
            UseRestrictChild::Name(elem) => elem.serialize_mei(writer),
            UseRestrictChild::GeogName(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "UseRestrictChild::{}",
                self.element_name()
            ))),
        }
    }
}

// ============================================================================
// AccessRestrict
// ============================================================================

impl MeiSerialize for AccessRestrict {
    fn element_name(&self) -> &'static str {
        "accessRestrict"
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

impl MeiSerialize for AccessRestrictChild {
    fn element_name(&self) -> &'static str {
        match self {
            AccessRestrictChild::Text(_) => "#text",
            AccessRestrictChild::P(_) => "p",
            AccessRestrictChild::District(_) => "district",
            AccessRestrictChild::PeriodName(_) => "periodName",
            AccessRestrictChild::Bibl(_) => "bibl",
            AccessRestrictChild::Signatures(_) => "signatures",
            AccessRestrictChild::Stack(_) => "stack",
            AccessRestrictChild::Lb(_) => "lb",
            AccessRestrictChild::Annot(_) => "annot",
            AccessRestrictChild::Address(_) => "address",
            AccessRestrictChild::Bloc(_) => "bloc",
            AccessRestrictChild::PostCode(_) => "postCode",
            AccessRestrictChild::GeogName(_) => "geogName",
            AccessRestrictChild::Depth(_) => "depth",
            AccessRestrictChild::PersName(_) => "persName",
            AccessRestrictChild::Identifier(_) => "identifier",
            AccessRestrictChild::LocusGrp(_) => "locusGrp",
            AccessRestrictChild::GeogFeat(_) => "geogFeat",
            AccessRestrictChild::Dim(_) => "dim",
            AccessRestrictChild::Num(_) => "num",
            AccessRestrictChild::Ptr(_) => "ptr",
            AccessRestrictChild::Country(_) => "country",
            AccessRestrictChild::Rend(_) => "rend",
            AccessRestrictChild::Region(_) => "region",
            AccessRestrictChild::BiblStruct(_) => "biblStruct",
            AccessRestrictChild::Locus(_) => "locus",
            AccessRestrictChild::Repository(_) => "repository",
            AccessRestrictChild::Seg(_) => "seg",
            AccessRestrictChild::Settlement(_) => "settlement",
            AccessRestrictChild::Stamp(_) => "stamp",
            AccessRestrictChild::Head(_) => "head",
            AccessRestrictChild::StyleName(_) => "styleName",
            AccessRestrictChild::PostBox(_) => "postBox",
            AccessRestrictChild::Relation(_) => "relation",
            AccessRestrictChild::SecFolio(_) => "secFolio",
            AccessRestrictChild::Heraldry(_) => "heraldry",
            AccessRestrictChild::Abbr(_) => "abbr",
            AccessRestrictChild::Term(_) => "term",
            AccessRestrictChild::Q(_) => "q",
            AccessRestrictChild::Extent(_) => "extent",
            AccessRestrictChild::Street(_) => "street",
            AccessRestrictChild::Height(_) => "height",
            AccessRestrictChild::Fig(_) => "fig",
            AccessRestrictChild::Catchwords(_) => "catchwords",
            AccessRestrictChild::Expan(_) => "expan",
            AccessRestrictChild::Symbol(_) => "symbol",
            AccessRestrictChild::Title(_) => "title",
            AccessRestrictChild::CorpName(_) => "corpName",
            AccessRestrictChild::Width(_) => "width",
            AccessRestrictChild::Name(_) => "name",
            AccessRestrictChild::Date(_) => "date",
            AccessRestrictChild::RelationList(_) => "relationList",
            AccessRestrictChild::Dimensions(_) => "dimensions",
            AccessRestrictChild::Ref(_) => "ref",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, AccessRestrictChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            AccessRestrictChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            AccessRestrictChild::P(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::District(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::Lb(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::Annot(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::Address(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::Bloc(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::PostCode(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::GeogName(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::PersName(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::Identifier(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::GeogFeat(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::Num(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::Ptr(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::Country(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::Rend(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::Region(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::Settlement(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::Head(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::PostBox(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::Extent(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::Street(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::Title(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::CorpName(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::Name(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::Date(elem) => elem.serialize_mei(writer),
            AccessRestrictChild::Ref(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "AccessRestrictChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// SysReq
// ============================================================================

impl MeiSerialize for SysReq {
    fn element_name(&self) -> &'static str {
        "sysReq"
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

impl MeiSerialize for SysReqChild {
    fn element_name(&self) -> &'static str {
        match self {
            SysReqChild::Text(_) => "#text",
            SysReqChild::Date(_) => "date",
            SysReqChild::Width(_) => "width",
            SysReqChild::Name(_) => "name",
            SysReqChild::CorpName(_) => "corpName",
            SysReqChild::BiblStruct(_) => "biblStruct",
            SysReqChild::PeriodName(_) => "periodName",
            SysReqChild::PostCode(_) => "postCode",
            SysReqChild::Identifier(_) => "identifier",
            SysReqChild::Q(_) => "q",
            SysReqChild::Region(_) => "region",
            SysReqChild::Country(_) => "country",
            SysReqChild::Repository(_) => "repository",
            SysReqChild::Stamp(_) => "stamp",
            SysReqChild::Lb(_) => "lb",
            SysReqChild::Bibl(_) => "bibl",
            SysReqChild::Head(_) => "head",
            SysReqChild::Annot(_) => "annot",
            SysReqChild::LocusGrp(_) => "locusGrp",
            SysReqChild::Title(_) => "title",
            SysReqChild::Address(_) => "address",
            SysReqChild::Street(_) => "street",
            SysReqChild::RelationList(_) => "relationList",
            SysReqChild::SecFolio(_) => "secFolio",
            SysReqChild::Heraldry(_) => "heraldry",
            SysReqChild::Dim(_) => "dim",
            SysReqChild::Num(_) => "num",
            SysReqChild::Height(_) => "height",
            SysReqChild::Seg(_) => "seg",
            SysReqChild::Signatures(_) => "signatures",
            SysReqChild::Bloc(_) => "bloc",
            SysReqChild::Locus(_) => "locus",
            SysReqChild::District(_) => "district",
            SysReqChild::Relation(_) => "relation",
            SysReqChild::Stack(_) => "stack",
            SysReqChild::Abbr(_) => "abbr",
            SysReqChild::Term(_) => "term",
            SysReqChild::Settlement(_) => "settlement",
            SysReqChild::Catchwords(_) => "catchwords",
            SysReqChild::Depth(_) => "depth",
            SysReqChild::Expan(_) => "expan",
            SysReqChild::PostBox(_) => "postBox",
            SysReqChild::StyleName(_) => "styleName",
            SysReqChild::Fig(_) => "fig",
            SysReqChild::Ptr(_) => "ptr",
            SysReqChild::Rend(_) => "rend",
            SysReqChild::Symbol(_) => "symbol",
            SysReqChild::GeogFeat(_) => "geogFeat",
            SysReqChild::P(_) => "p",
            SysReqChild::GeogName(_) => "geogName",
            SysReqChild::PersName(_) => "persName",
            SysReqChild::Dimensions(_) => "dimensions",
            SysReqChild::Extent(_) => "extent",
            SysReqChild::Ref(_) => "ref",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, SysReqChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SysReqChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            SysReqChild::Date(elem) => elem.serialize_mei(writer),
            SysReqChild::Name(elem) => elem.serialize_mei(writer),
            SysReqChild::CorpName(elem) => elem.serialize_mei(writer),
            SysReqChild::PostCode(elem) => elem.serialize_mei(writer),
            SysReqChild::Identifier(elem) => elem.serialize_mei(writer),
            SysReqChild::Region(elem) => elem.serialize_mei(writer),
            SysReqChild::Country(elem) => elem.serialize_mei(writer),
            SysReqChild::Lb(elem) => elem.serialize_mei(writer),
            SysReqChild::Head(elem) => elem.serialize_mei(writer),
            SysReqChild::Annot(elem) => elem.serialize_mei(writer),
            SysReqChild::Title(elem) => elem.serialize_mei(writer),
            SysReqChild::Address(elem) => elem.serialize_mei(writer),
            SysReqChild::Street(elem) => elem.serialize_mei(writer),
            SysReqChild::Num(elem) => elem.serialize_mei(writer),
            SysReqChild::Bloc(elem) => elem.serialize_mei(writer),
            SysReqChild::District(elem) => elem.serialize_mei(writer),
            SysReqChild::Settlement(elem) => elem.serialize_mei(writer),
            SysReqChild::PostBox(elem) => elem.serialize_mei(writer),
            SysReqChild::Ptr(elem) => elem.serialize_mei(writer),
            SysReqChild::Rend(elem) => elem.serialize_mei(writer),
            SysReqChild::GeogFeat(elem) => elem.serialize_mei(writer),
            SysReqChild::P(elem) => elem.serialize_mei(writer),
            SysReqChild::GeogName(elem) => elem.serialize_mei(writer),
            SysReqChild::PersName(elem) => elem.serialize_mei(writer),
            SysReqChild::Extent(elem) => elem.serialize_mei(writer),
            SysReqChild::Ref(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "SysReqChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Price
// ============================================================================

impl MeiSerialize for Price {
    fn element_name(&self) -> &'static str {
        "price"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        if let Some(amount) = self.amount {
            attrs.push(("amount", amount.to_string()));
        }
        if let Some(ref currency) = self.currency {
            attrs.push(("currency", currency.clone()));
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

impl MeiSerialize for PriceChild {
    fn element_name(&self) -> &'static str {
        match self {
            PriceChild::Text(_) => "#text",
            PriceChild::Stack(_) => "stack",
            PriceChild::Width(_) => "width",
            PriceChild::Street(_) => "street",
            PriceChild::PostBox(_) => "postBox",
            PriceChild::GeogFeat(_) => "geogFeat",
            PriceChild::LocusGrp(_) => "locusGrp",
            PriceChild::Depth(_) => "depth",
            PriceChild::Signatures(_) => "signatures",
            PriceChild::StyleName(_) => "styleName",
            PriceChild::Term(_) => "term",
            PriceChild::Address(_) => "address",
            PriceChild::Abbr(_) => "abbr",
            PriceChild::Fig(_) => "fig",
            PriceChild::Relation(_) => "relation",
            PriceChild::SecFolio(_) => "secFolio",
            PriceChild::Q(_) => "q",
            PriceChild::Repository(_) => "repository",
            PriceChild::CorpName(_) => "corpName",
            PriceChild::Title(_) => "title",
            PriceChild::Expan(_) => "expan",
            PriceChild::Identifier(_) => "identifier",
            PriceChild::Date(_) => "date",
            PriceChild::Lb(_) => "lb",
            PriceChild::Name(_) => "name",
            PriceChild::PersName(_) => "persName",
            PriceChild::PostCode(_) => "postCode",
            PriceChild::Dim(_) => "dim",
            PriceChild::Heraldry(_) => "heraldry",
            PriceChild::Locus(_) => "locus",
            PriceChild::Ptr(_) => "ptr",
            PriceChild::Head(_) => "head",
            PriceChild::Catchwords(_) => "catchwords",
            PriceChild::Country(_) => "country",
            PriceChild::Stamp(_) => "stamp",
            PriceChild::Annot(_) => "annot",
            PriceChild::Bloc(_) => "bloc",
            PriceChild::Ref(_) => "ref",
            PriceChild::Region(_) => "region",
            PriceChild::Seg(_) => "seg",
            PriceChild::Settlement(_) => "settlement",
            PriceChild::District(_) => "district",
            PriceChild::RelationList(_) => "relationList",
            PriceChild::Symbol(_) => "symbol",
            PriceChild::Bibl(_) => "bibl",
            PriceChild::BiblStruct(_) => "biblStruct",
            PriceChild::GeogName(_) => "geogName",
            PriceChild::Extent(_) => "extent",
            PriceChild::Height(_) => "height",
            PriceChild::Dimensions(_) => "dimensions",
            PriceChild::PeriodName(_) => "periodName",
            PriceChild::Num(_) => "num",
            PriceChild::Rend(_) => "rend",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, PriceChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PriceChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            PriceChild::Street(elem) => elem.serialize_mei(writer),
            PriceChild::PostBox(elem) => elem.serialize_mei(writer),
            PriceChild::GeogFeat(elem) => elem.serialize_mei(writer),
            PriceChild::Address(elem) => elem.serialize_mei(writer),
            PriceChild::CorpName(elem) => elem.serialize_mei(writer),
            PriceChild::Title(elem) => elem.serialize_mei(writer),
            PriceChild::Identifier(elem) => elem.serialize_mei(writer),
            PriceChild::Date(elem) => elem.serialize_mei(writer),
            PriceChild::Lb(elem) => elem.serialize_mei(writer),
            PriceChild::Name(elem) => elem.serialize_mei(writer),
            PriceChild::PersName(elem) => elem.serialize_mei(writer),
            PriceChild::PostCode(elem) => elem.serialize_mei(writer),
            PriceChild::Ptr(elem) => elem.serialize_mei(writer),
            PriceChild::Head(elem) => elem.serialize_mei(writer),
            PriceChild::Country(elem) => elem.serialize_mei(writer),
            PriceChild::Annot(elem) => elem.serialize_mei(writer),
            PriceChild::Bloc(elem) => elem.serialize_mei(writer),
            PriceChild::Ref(elem) => elem.serialize_mei(writer),
            PriceChild::Region(elem) => elem.serialize_mei(writer),
            PriceChild::Settlement(elem) => elem.serialize_mei(writer),
            PriceChild::District(elem) => elem.serialize_mei(writer),
            PriceChild::GeogName(elem) => elem.serialize_mei(writer),
            PriceChild::Extent(elem) => elem.serialize_mei(writer),
            PriceChild::Num(elem) => elem.serialize_mei(writer),
            PriceChild::Rend(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "PriceChild::{}",
                other.element_name()
            ))),
        }
    }
}
