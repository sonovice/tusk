//! Serializer implementations for address-related elements.
//!
//! Contains: AddrLine, AddrLineChild, GeogName, GeogNameChild, Settlement, Country,
//! PostCode, Street, District, Bloc, GeogFeat, PostBox, Region and their children.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    AddrLine, AddrLineChild, Bloc, BlocChild, Country, CountryChild, District, DistrictChild,
    GeogFeat, GeogFeatChild, GeogName, GeogNameChild, PostBox, PostBoxChild, PostCode,
    PostCodeChild, Region, RegionChild, Settlement, SettlementChild, Street, StreetChild,
};

// ============================================================================
// AddrLine
// ============================================================================

impl MeiSerialize for AddrLine {
    fn element_name(&self) -> &'static str {
        "addrLine"
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

impl MeiSerialize for AddrLineChild {
    fn element_name(&self) -> &'static str {
        match self {
            AddrLineChild::Text(_) => "#text",
            AddrLineChild::GeogName(_) => "geogName",
            AddrLineChild::Address(_) => "address",
            AddrLineChild::Settlement(_) => "settlement",
            AddrLineChild::Country(_) => "country",
            AddrLineChild::PostCode(_) => "postCode",
            AddrLineChild::Street(_) => "street",
            AddrLineChild::District(_) => "district",
            AddrLineChild::Bloc(_) => "bloc",
            AddrLineChild::GeogFeat(_) => "geogFeat",
            AddrLineChild::PostBox(_) => "postBox",
            AddrLineChild::Region(_) => "region",
            AddrLineChild::Rend(_) => "rend",
            AddrLineChild::Lb(_) => "lb",
            AddrLineChild::Date(_) => "date",
            AddrLineChild::Name(_) => "name",
            AddrLineChild::PersName(_) => "persName",
            AddrLineChild::CorpName(_) => "corpName",
            AddrLineChild::Title(_) => "title",
            AddrLineChild::Identifier(_) => "identifier",
            AddrLineChild::Num(_) => "num",
            AddrLineChild::Ref(_) => "ref",
            AddrLineChild::Ptr(_) => "ptr",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, AddrLineChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            AddrLineChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            AddrLineChild::GeogName(elem) => elem.serialize_mei(writer),
            AddrLineChild::Address(elem) => elem.serialize_mei(writer),
            AddrLineChild::Settlement(elem) => elem.serialize_mei(writer),
            AddrLineChild::Country(elem) => elem.serialize_mei(writer),
            AddrLineChild::PostCode(elem) => elem.serialize_mei(writer),
            AddrLineChild::Street(elem) => elem.serialize_mei(writer),
            AddrLineChild::District(elem) => elem.serialize_mei(writer),
            AddrLineChild::Bloc(elem) => elem.serialize_mei(writer),
            AddrLineChild::GeogFeat(elem) => elem.serialize_mei(writer),
            AddrLineChild::PostBox(elem) => elem.serialize_mei(writer),
            AddrLineChild::Region(elem) => elem.serialize_mei(writer),
            AddrLineChild::Rend(elem) => elem.serialize_mei(writer),
            AddrLineChild::Lb(elem) => elem.serialize_mei(writer),
            AddrLineChild::Date(elem) => elem.serialize_mei(writer),
            AddrLineChild::Name(elem) => elem.serialize_mei(writer),
            AddrLineChild::PersName(elem) => elem.serialize_mei(writer),
            AddrLineChild::CorpName(elem) => elem.serialize_mei(writer),
            AddrLineChild::Title(elem) => elem.serialize_mei(writer),
            AddrLineChild::Identifier(elem) => elem.serialize_mei(writer),
            AddrLineChild::Num(elem) => elem.serialize_mei(writer),
            AddrLineChild::Ref(elem) => elem.serialize_mei(writer),
            AddrLineChild::Ptr(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "AddrLineChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// GeogName
// ============================================================================

impl MeiSerialize for GeogName {
    fn element_name(&self) -> &'static str {
        "geogName"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.name.collect_attributes());
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

impl MeiSerialize for GeogNameChild {
    fn element_name(&self) -> &'static str {
        match self {
            GeogNameChild::Text(_) => "#text",
            GeogNameChild::GeogName(_) => "geogName",
            GeogNameChild::Address(_) => "address",
            GeogNameChild::Settlement(_) => "settlement",
            GeogNameChild::Country(_) => "country",
            GeogNameChild::PostCode(_) => "postCode",
            GeogNameChild::Street(_) => "street",
            GeogNameChild::District(_) => "district",
            GeogNameChild::Bloc(_) => "bloc",
            GeogNameChild::GeogFeat(_) => "geogFeat",
            GeogNameChild::PostBox(_) => "postBox",
            GeogNameChild::Region(_) => "region",
            GeogNameChild::Rend(_) => "rend",
            GeogNameChild::Lb(_) => "lb",
            GeogNameChild::Date(_) => "date",
            GeogNameChild::Name(_) => "name",
            GeogNameChild::PersName(_) => "persName",
            GeogNameChild::CorpName(_) => "corpName",
            GeogNameChild::Title(_) => "title",
            GeogNameChild::Identifier(_) => "identifier",
            GeogNameChild::Num(_) => "num",
            GeogNameChild::Ref(_) => "ref",
            GeogNameChild::Ptr(_) => "ptr",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, GeogNameChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            GeogNameChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            GeogNameChild::GeogName(elem) => elem.serialize_mei(writer),
            GeogNameChild::Address(elem) => elem.serialize_mei(writer),
            GeogNameChild::Settlement(elem) => elem.serialize_mei(writer),
            GeogNameChild::Country(elem) => elem.serialize_mei(writer),
            GeogNameChild::PostCode(elem) => elem.serialize_mei(writer),
            GeogNameChild::Street(elem) => elem.serialize_mei(writer),
            GeogNameChild::District(elem) => elem.serialize_mei(writer),
            GeogNameChild::Bloc(elem) => elem.serialize_mei(writer),
            GeogNameChild::GeogFeat(elem) => elem.serialize_mei(writer),
            GeogNameChild::PostBox(elem) => elem.serialize_mei(writer),
            GeogNameChild::Region(elem) => elem.serialize_mei(writer),
            GeogNameChild::Rend(elem) => elem.serialize_mei(writer),
            GeogNameChild::Lb(elem) => elem.serialize_mei(writer),
            GeogNameChild::Date(elem) => elem.serialize_mei(writer),
            GeogNameChild::Name(elem) => elem.serialize_mei(writer),
            GeogNameChild::PersName(elem) => elem.serialize_mei(writer),
            GeogNameChild::CorpName(elem) => elem.serialize_mei(writer),
            GeogNameChild::Title(elem) => elem.serialize_mei(writer),
            GeogNameChild::Identifier(elem) => elem.serialize_mei(writer),
            GeogNameChild::Num(elem) => elem.serialize_mei(writer),
            GeogNameChild::Ref(elem) => elem.serialize_mei(writer),
            GeogNameChild::Ptr(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "GeogNameChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Settlement
// ============================================================================

impl MeiSerialize for Settlement {
    fn element_name(&self) -> &'static str {
        "settlement"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.name.collect_attributes());
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

impl MeiSerialize for SettlementChild {
    fn element_name(&self) -> &'static str {
        match self {
            SettlementChild::Text(_) => "#text",
            SettlementChild::GeogName(_) => "geogName",
            SettlementChild::Address(_) => "address",
            SettlementChild::Settlement(_) => "settlement",
            SettlementChild::Country(_) => "country",
            SettlementChild::PostCode(_) => "postCode",
            SettlementChild::Street(_) => "street",
            SettlementChild::District(_) => "district",
            SettlementChild::Bloc(_) => "bloc",
            SettlementChild::GeogFeat(_) => "geogFeat",
            SettlementChild::PostBox(_) => "postBox",
            SettlementChild::Region(_) => "region",
            SettlementChild::Rend(_) => "rend",
            SettlementChild::Lb(_) => "lb",
            SettlementChild::Date(_) => "date",
            SettlementChild::Name(_) => "name",
            SettlementChild::PersName(_) => "persName",
            SettlementChild::CorpName(_) => "corpName",
            SettlementChild::Title(_) => "title",
            SettlementChild::Identifier(_) => "identifier",
            SettlementChild::Num(_) => "num",
            SettlementChild::Ref(_) => "ref",
            SettlementChild::Ptr(_) => "ptr",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, SettlementChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SettlementChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            SettlementChild::GeogName(elem) => elem.serialize_mei(writer),
            SettlementChild::Address(elem) => elem.serialize_mei(writer),
            SettlementChild::Settlement(elem) => elem.serialize_mei(writer),
            SettlementChild::Country(elem) => elem.serialize_mei(writer),
            SettlementChild::PostCode(elem) => elem.serialize_mei(writer),
            SettlementChild::Street(elem) => elem.serialize_mei(writer),
            SettlementChild::District(elem) => elem.serialize_mei(writer),
            SettlementChild::Bloc(elem) => elem.serialize_mei(writer),
            SettlementChild::GeogFeat(elem) => elem.serialize_mei(writer),
            SettlementChild::PostBox(elem) => elem.serialize_mei(writer),
            SettlementChild::Region(elem) => elem.serialize_mei(writer),
            SettlementChild::Rend(elem) => elem.serialize_mei(writer),
            SettlementChild::Lb(elem) => elem.serialize_mei(writer),
            SettlementChild::Date(elem) => elem.serialize_mei(writer),
            SettlementChild::Name(elem) => elem.serialize_mei(writer),
            SettlementChild::PersName(elem) => elem.serialize_mei(writer),
            SettlementChild::CorpName(elem) => elem.serialize_mei(writer),
            SettlementChild::Title(elem) => elem.serialize_mei(writer),
            SettlementChild::Identifier(elem) => elem.serialize_mei(writer),
            SettlementChild::Num(elem) => elem.serialize_mei(writer),
            SettlementChild::Ref(elem) => elem.serialize_mei(writer),
            SettlementChild::Ptr(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "SettlementChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Country
// ============================================================================

impl MeiSerialize for Country {
    fn element_name(&self) -> &'static str {
        "country"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.name.collect_attributes());
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

impl MeiSerialize for CountryChild {
    fn element_name(&self) -> &'static str {
        match self {
            CountryChild::Text(_) => "#text",
            CountryChild::GeogName(_) => "geogName",
            CountryChild::Address(_) => "address",
            CountryChild::Settlement(_) => "settlement",
            CountryChild::Country(_) => "country",
            CountryChild::PostCode(_) => "postCode",
            CountryChild::Street(_) => "street",
            CountryChild::District(_) => "district",
            CountryChild::Bloc(_) => "bloc",
            CountryChild::GeogFeat(_) => "geogFeat",
            CountryChild::PostBox(_) => "postBox",
            CountryChild::Region(_) => "region",
            CountryChild::Rend(_) => "rend",
            CountryChild::Lb(_) => "lb",
            CountryChild::Date(_) => "date",
            CountryChild::Name(_) => "name",
            CountryChild::PersName(_) => "persName",
            CountryChild::CorpName(_) => "corpName",
            CountryChild::Title(_) => "title",
            CountryChild::Identifier(_) => "identifier",
            CountryChild::Num(_) => "num",
            CountryChild::Ref(_) => "ref",
            CountryChild::Ptr(_) => "ptr",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, CountryChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            CountryChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            CountryChild::GeogName(elem) => elem.serialize_mei(writer),
            CountryChild::Address(elem) => elem.serialize_mei(writer),
            CountryChild::Settlement(elem) => elem.serialize_mei(writer),
            CountryChild::Country(elem) => elem.serialize_mei(writer),
            CountryChild::PostCode(elem) => elem.serialize_mei(writer),
            CountryChild::Street(elem) => elem.serialize_mei(writer),
            CountryChild::District(elem) => elem.serialize_mei(writer),
            CountryChild::Bloc(elem) => elem.serialize_mei(writer),
            CountryChild::GeogFeat(elem) => elem.serialize_mei(writer),
            CountryChild::PostBox(elem) => elem.serialize_mei(writer),
            CountryChild::Region(elem) => elem.serialize_mei(writer),
            CountryChild::Rend(elem) => elem.serialize_mei(writer),
            CountryChild::Lb(elem) => elem.serialize_mei(writer),
            CountryChild::Date(elem) => elem.serialize_mei(writer),
            CountryChild::Name(elem) => elem.serialize_mei(writer),
            CountryChild::PersName(elem) => elem.serialize_mei(writer),
            CountryChild::CorpName(elem) => elem.serialize_mei(writer),
            CountryChild::Title(elem) => elem.serialize_mei(writer),
            CountryChild::Identifier(elem) => elem.serialize_mei(writer),
            CountryChild::Num(elem) => elem.serialize_mei(writer),
            CountryChild::Ref(elem) => elem.serialize_mei(writer),
            CountryChild::Ptr(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "CountryChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// PostCode
// ============================================================================

impl MeiSerialize for PostCode {
    fn element_name(&self) -> &'static str {
        "postCode"
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

impl MeiSerialize for PostCodeChild {
    fn element_name(&self) -> &'static str {
        match self {
            PostCodeChild::Text(_) => "#text",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, PostCodeChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PostCodeChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "PostCodeChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Street
// ============================================================================

impl MeiSerialize for Street {
    fn element_name(&self) -> &'static str {
        "street"
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

impl MeiSerialize for StreetChild {
    fn element_name(&self) -> &'static str {
        match self {
            StreetChild::Text(_) => "#text",
            StreetChild::GeogName(_) => "geogName",
            StreetChild::Address(_) => "address",
            StreetChild::Settlement(_) => "settlement",
            StreetChild::Country(_) => "country",
            StreetChild::PostCode(_) => "postCode",
            StreetChild::Street(_) => "street",
            StreetChild::District(_) => "district",
            StreetChild::Bloc(_) => "bloc",
            StreetChild::GeogFeat(_) => "geogFeat",
            StreetChild::PostBox(_) => "postBox",
            StreetChild::Region(_) => "region",
            StreetChild::Rend(_) => "rend",
            StreetChild::Lb(_) => "lb",
            StreetChild::Date(_) => "date",
            StreetChild::Name(_) => "name",
            StreetChild::PersName(_) => "persName",
            StreetChild::CorpName(_) => "corpName",
            StreetChild::Title(_) => "title",
            StreetChild::Identifier(_) => "identifier",
            StreetChild::Num(_) => "num",
            StreetChild::Ref(_) => "ref",
            StreetChild::Ptr(_) => "ptr",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, StreetChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            StreetChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            StreetChild::GeogName(elem) => elem.serialize_mei(writer),
            StreetChild::Address(elem) => elem.serialize_mei(writer),
            StreetChild::Settlement(elem) => elem.serialize_mei(writer),
            StreetChild::Country(elem) => elem.serialize_mei(writer),
            StreetChild::PostCode(elem) => elem.serialize_mei(writer),
            StreetChild::Street(elem) => elem.serialize_mei(writer),
            StreetChild::District(elem) => elem.serialize_mei(writer),
            StreetChild::Bloc(elem) => elem.serialize_mei(writer),
            StreetChild::GeogFeat(elem) => elem.serialize_mei(writer),
            StreetChild::PostBox(elem) => elem.serialize_mei(writer),
            StreetChild::Region(elem) => elem.serialize_mei(writer),
            StreetChild::Rend(elem) => elem.serialize_mei(writer),
            StreetChild::Lb(elem) => elem.serialize_mei(writer),
            StreetChild::Date(elem) => elem.serialize_mei(writer),
            StreetChild::Name(elem) => elem.serialize_mei(writer),
            StreetChild::PersName(elem) => elem.serialize_mei(writer),
            StreetChild::CorpName(elem) => elem.serialize_mei(writer),
            StreetChild::Title(elem) => elem.serialize_mei(writer),
            StreetChild::Identifier(elem) => elem.serialize_mei(writer),
            StreetChild::Num(elem) => elem.serialize_mei(writer),
            StreetChild::Ref(elem) => elem.serialize_mei(writer),
            StreetChild::Ptr(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "StreetChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// District
// ============================================================================

impl MeiSerialize for District {
    fn element_name(&self) -> &'static str {
        "district"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.name.collect_attributes());
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

impl MeiSerialize for DistrictChild {
    fn element_name(&self) -> &'static str {
        match self {
            DistrictChild::Text(_) => "#text",
            DistrictChild::GeogName(_) => "geogName",
            DistrictChild::Address(_) => "address",
            DistrictChild::Settlement(_) => "settlement",
            DistrictChild::Country(_) => "country",
            DistrictChild::PostCode(_) => "postCode",
            DistrictChild::Street(_) => "street",
            DistrictChild::District(_) => "district",
            DistrictChild::Bloc(_) => "bloc",
            DistrictChild::GeogFeat(_) => "geogFeat",
            DistrictChild::PostBox(_) => "postBox",
            DistrictChild::Region(_) => "region",
            DistrictChild::Rend(_) => "rend",
            DistrictChild::Lb(_) => "lb",
            DistrictChild::Date(_) => "date",
            DistrictChild::Name(_) => "name",
            DistrictChild::PersName(_) => "persName",
            DistrictChild::CorpName(_) => "corpName",
            DistrictChild::Title(_) => "title",
            DistrictChild::Identifier(_) => "identifier",
            DistrictChild::Num(_) => "num",
            DistrictChild::Ref(_) => "ref",
            DistrictChild::Ptr(_) => "ptr",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, DistrictChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            DistrictChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            DistrictChild::GeogName(elem) => elem.serialize_mei(writer),
            DistrictChild::Address(elem) => elem.serialize_mei(writer),
            DistrictChild::Settlement(elem) => elem.serialize_mei(writer),
            DistrictChild::Country(elem) => elem.serialize_mei(writer),
            DistrictChild::PostCode(elem) => elem.serialize_mei(writer),
            DistrictChild::Street(elem) => elem.serialize_mei(writer),
            DistrictChild::District(elem) => elem.serialize_mei(writer),
            DistrictChild::Bloc(elem) => elem.serialize_mei(writer),
            DistrictChild::GeogFeat(elem) => elem.serialize_mei(writer),
            DistrictChild::PostBox(elem) => elem.serialize_mei(writer),
            DistrictChild::Region(elem) => elem.serialize_mei(writer),
            DistrictChild::Rend(elem) => elem.serialize_mei(writer),
            DistrictChild::Lb(elem) => elem.serialize_mei(writer),
            DistrictChild::Date(elem) => elem.serialize_mei(writer),
            DistrictChild::Name(elem) => elem.serialize_mei(writer),
            DistrictChild::PersName(elem) => elem.serialize_mei(writer),
            DistrictChild::CorpName(elem) => elem.serialize_mei(writer),
            DistrictChild::Title(elem) => elem.serialize_mei(writer),
            DistrictChild::Identifier(elem) => elem.serialize_mei(writer),
            DistrictChild::Num(elem) => elem.serialize_mei(writer),
            DistrictChild::Ref(elem) => elem.serialize_mei(writer),
            DistrictChild::Ptr(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "DistrictChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Bloc
// ============================================================================

impl MeiSerialize for Bloc {
    fn element_name(&self) -> &'static str {
        "bloc"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.name.collect_attributes());
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

impl MeiSerialize for BlocChild {
    fn element_name(&self) -> &'static str {
        match self {
            BlocChild::Text(_) => "#text",
            BlocChild::GeogName(_) => "geogName",
            BlocChild::Address(_) => "address",
            BlocChild::Settlement(_) => "settlement",
            BlocChild::Country(_) => "country",
            BlocChild::PostCode(_) => "postCode",
            BlocChild::Street(_) => "street",
            BlocChild::District(_) => "district",
            BlocChild::Bloc(_) => "bloc",
            BlocChild::GeogFeat(_) => "geogFeat",
            BlocChild::PostBox(_) => "postBox",
            BlocChild::Region(_) => "region",
            BlocChild::Rend(_) => "rend",
            BlocChild::Lb(_) => "lb",
            BlocChild::Date(_) => "date",
            BlocChild::Name(_) => "name",
            BlocChild::PersName(_) => "persName",
            BlocChild::CorpName(_) => "corpName",
            BlocChild::Title(_) => "title",
            BlocChild::Identifier(_) => "identifier",
            BlocChild::Num(_) => "num",
            BlocChild::Ref(_) => "ref",
            BlocChild::Ptr(_) => "ptr",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, BlocChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            BlocChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            BlocChild::GeogName(elem) => elem.serialize_mei(writer),
            BlocChild::Address(elem) => elem.serialize_mei(writer),
            BlocChild::Settlement(elem) => elem.serialize_mei(writer),
            BlocChild::Country(elem) => elem.serialize_mei(writer),
            BlocChild::PostCode(elem) => elem.serialize_mei(writer),
            BlocChild::Street(elem) => elem.serialize_mei(writer),
            BlocChild::District(elem) => elem.serialize_mei(writer),
            BlocChild::Bloc(elem) => elem.serialize_mei(writer),
            BlocChild::GeogFeat(elem) => elem.serialize_mei(writer),
            BlocChild::PostBox(elem) => elem.serialize_mei(writer),
            BlocChild::Region(elem) => elem.serialize_mei(writer),
            BlocChild::Rend(elem) => elem.serialize_mei(writer),
            BlocChild::Lb(elem) => elem.serialize_mei(writer),
            BlocChild::Date(elem) => elem.serialize_mei(writer),
            BlocChild::Name(elem) => elem.serialize_mei(writer),
            BlocChild::PersName(elem) => elem.serialize_mei(writer),
            BlocChild::CorpName(elem) => elem.serialize_mei(writer),
            BlocChild::Title(elem) => elem.serialize_mei(writer),
            BlocChild::Identifier(elem) => elem.serialize_mei(writer),
            BlocChild::Num(elem) => elem.serialize_mei(writer),
            BlocChild::Ref(elem) => elem.serialize_mei(writer),
            BlocChild::Ptr(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "BlocChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// GeogFeat
// ============================================================================

impl MeiSerialize for GeogFeat {
    fn element_name(&self) -> &'static str {
        "geogFeat"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.name.collect_attributes());
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

impl MeiSerialize for GeogFeatChild {
    fn element_name(&self) -> &'static str {
        match self {
            GeogFeatChild::Text(_) => "#text",
            GeogFeatChild::GeogName(_) => "geogName",
            GeogFeatChild::Address(_) => "address",
            GeogFeatChild::Settlement(_) => "settlement",
            GeogFeatChild::Country(_) => "country",
            GeogFeatChild::PostCode(_) => "postCode",
            GeogFeatChild::Street(_) => "street",
            GeogFeatChild::District(_) => "district",
            GeogFeatChild::Bloc(_) => "bloc",
            GeogFeatChild::GeogFeat(_) => "geogFeat",
            GeogFeatChild::PostBox(_) => "postBox",
            GeogFeatChild::Region(_) => "region",
            GeogFeatChild::Rend(_) => "rend",
            GeogFeatChild::Lb(_) => "lb",
            GeogFeatChild::Date(_) => "date",
            GeogFeatChild::Name(_) => "name",
            GeogFeatChild::PersName(_) => "persName",
            GeogFeatChild::CorpName(_) => "corpName",
            GeogFeatChild::Title(_) => "title",
            GeogFeatChild::Identifier(_) => "identifier",
            GeogFeatChild::Num(_) => "num",
            GeogFeatChild::Ref(_) => "ref",
            GeogFeatChild::Ptr(_) => "ptr",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, GeogFeatChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            GeogFeatChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            GeogFeatChild::GeogName(elem) => elem.serialize_mei(writer),
            GeogFeatChild::Address(elem) => elem.serialize_mei(writer),
            GeogFeatChild::Settlement(elem) => elem.serialize_mei(writer),
            GeogFeatChild::Country(elem) => elem.serialize_mei(writer),
            GeogFeatChild::PostCode(elem) => elem.serialize_mei(writer),
            GeogFeatChild::Street(elem) => elem.serialize_mei(writer),
            GeogFeatChild::District(elem) => elem.serialize_mei(writer),
            GeogFeatChild::Bloc(elem) => elem.serialize_mei(writer),
            GeogFeatChild::GeogFeat(elem) => elem.serialize_mei(writer),
            GeogFeatChild::PostBox(elem) => elem.serialize_mei(writer),
            GeogFeatChild::Region(elem) => elem.serialize_mei(writer),
            GeogFeatChild::Rend(elem) => elem.serialize_mei(writer),
            GeogFeatChild::Lb(elem) => elem.serialize_mei(writer),
            GeogFeatChild::Date(elem) => elem.serialize_mei(writer),
            GeogFeatChild::Name(elem) => elem.serialize_mei(writer),
            GeogFeatChild::PersName(elem) => elem.serialize_mei(writer),
            GeogFeatChild::CorpName(elem) => elem.serialize_mei(writer),
            GeogFeatChild::Title(elem) => elem.serialize_mei(writer),
            GeogFeatChild::Identifier(elem) => elem.serialize_mei(writer),
            GeogFeatChild::Num(elem) => elem.serialize_mei(writer),
            GeogFeatChild::Ref(elem) => elem.serialize_mei(writer),
            GeogFeatChild::Ptr(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "GeogFeatChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// PostBox
// ============================================================================

impl MeiSerialize for PostBox {
    fn element_name(&self) -> &'static str {
        "postBox"
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

impl MeiSerialize for PostBoxChild {
    fn element_name(&self) -> &'static str {
        match self {
            PostBoxChild::Text(_) => "#text",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, PostBoxChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PostBoxChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "PostBoxChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Region
// ============================================================================

impl MeiSerialize for Region {
    fn element_name(&self) -> &'static str {
        "region"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.name.collect_attributes());
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

impl MeiSerialize for RegionChild {
    fn element_name(&self) -> &'static str {
        match self {
            RegionChild::Text(_) => "#text",
            RegionChild::GeogName(_) => "geogName",
            RegionChild::Address(_) => "address",
            RegionChild::Settlement(_) => "settlement",
            RegionChild::Country(_) => "country",
            RegionChild::PostCode(_) => "postCode",
            RegionChild::Street(_) => "street",
            RegionChild::District(_) => "district",
            RegionChild::Bloc(_) => "bloc",
            RegionChild::GeogFeat(_) => "geogFeat",
            RegionChild::PostBox(_) => "postBox",
            RegionChild::Region(_) => "region",
            RegionChild::Rend(_) => "rend",
            RegionChild::Lb(_) => "lb",
            RegionChild::Date(_) => "date",
            RegionChild::Name(_) => "name",
            RegionChild::PersName(_) => "persName",
            RegionChild::CorpName(_) => "corpName",
            RegionChild::Title(_) => "title",
            RegionChild::Identifier(_) => "identifier",
            RegionChild::Num(_) => "num",
            RegionChild::Ref(_) => "ref",
            RegionChild::Ptr(_) => "ptr",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, RegionChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            RegionChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            RegionChild::GeogName(elem) => elem.serialize_mei(writer),
            RegionChild::Address(elem) => elem.serialize_mei(writer),
            RegionChild::Settlement(elem) => elem.serialize_mei(writer),
            RegionChild::Country(elem) => elem.serialize_mei(writer),
            RegionChild::PostCode(elem) => elem.serialize_mei(writer),
            RegionChild::Street(elem) => elem.serialize_mei(writer),
            RegionChild::District(elem) => elem.serialize_mei(writer),
            RegionChild::Bloc(elem) => elem.serialize_mei(writer),
            RegionChild::GeogFeat(elem) => elem.serialize_mei(writer),
            RegionChild::PostBox(elem) => elem.serialize_mei(writer),
            RegionChild::Region(elem) => elem.serialize_mei(writer),
            RegionChild::Rend(elem) => elem.serialize_mei(writer),
            RegionChild::Lb(elem) => elem.serialize_mei(writer),
            RegionChild::Date(elem) => elem.serialize_mei(writer),
            RegionChild::Name(elem) => elem.serialize_mei(writer),
            RegionChild::PersName(elem) => elem.serialize_mei(writer),
            RegionChild::CorpName(elem) => elem.serialize_mei(writer),
            RegionChild::Title(elem) => elem.serialize_mei(writer),
            RegionChild::Identifier(elem) => elem.serialize_mei(writer),
            RegionChild::Num(elem) => elem.serialize_mei(writer),
            RegionChild::Ref(elem) => elem.serialize_mei(writer),
            RegionChild::Ptr(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "RegionChild::{}",
                other.element_name()
            ))),
        }
    }
}
