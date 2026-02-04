//! Serializer implementations for publication statement elements.
//!
//! Contains: PubStmt, Publisher, Address, PubPlace, Availability, Identifier, Distributor, Unpub.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    Address, AddressChild, Availability, AvailabilityChild, Distributor, DistributorChild,
    Identifier, IdentifierChild, PubPlace, PubPlaceChild, PubStmt, PubStmtChild, Publisher,
    PublisherChild, Unpub, UnpubChild,
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
            _ => Ok(()), // Other children skipped for now
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
            _ => Ok(()), // Other children skipped for now
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
            AvailabilityChild::Date(elem) => elem.serialize_mei(writer),
            AvailabilityChild::Distributor(elem) => elem.serialize_mei(writer),
            AvailabilityChild::Identifier(elem) => elem.serialize_mei(writer),
            AvailabilityChild::Address(elem) => elem.serialize_mei(writer),
            AvailabilityChild::Head(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
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
            IdentifierChild::Date(_) => "date",
            IdentifierChild::Name(_) => "name",
            IdentifierChild::PersName(_) => "persName",
            IdentifierChild::CorpName(_) => "corpName",
            IdentifierChild::Address(_) => "address",
            IdentifierChild::Identifier(_) => "identifier",
            IdentifierChild::Rend(_) => "rend",
            IdentifierChild::Lb(_) => "lb",
            IdentifierChild::Title(_) => "title",
            _ => "unknown",
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
            IdentifierChild::Date(elem) => elem.serialize_mei(writer),
            IdentifierChild::Name(elem) => elem.serialize_mei(writer),
            IdentifierChild::PersName(elem) => elem.serialize_mei(writer),
            IdentifierChild::CorpName(elem) => elem.serialize_mei(writer),
            IdentifierChild::Address(elem) => elem.serialize_mei(writer),
            IdentifierChild::Identifier(elem) => elem.serialize_mei(writer),
            IdentifierChild::Rend(elem) => elem.serialize_mei(writer),
            IdentifierChild::Lb(elem) => elem.serialize_mei(writer),
            IdentifierChild::Title(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
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
            _ => Ok(()), // Other children skipped for now
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
