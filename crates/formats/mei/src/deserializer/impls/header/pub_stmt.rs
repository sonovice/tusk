//! Publication statement elements (PubStmt, Publisher, Availability, etc.).

use super::super::{extract_attr, from_attr_string};
use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::elements::{
    AccessRestrict, AccessRestrictChild, Availability, AvailabilityChild, Date, DateChild,
    Distributor, DistributorChild, Identifier, IdentifierChild, Price, PriceChild, PubPlace,
    PubPlaceChild, PubStmt, PubStmtChild, Publisher, PublisherChild, SysReq, SysReqChild, Unpub,
    UnpubChild, UseRestrict, UseRestrictChild,
};

// MeiDeserialize trait implementations
impl MeiDeserialize for PubStmt {
    fn element_name() -> &'static str {
        "pubStmt"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_pub_stmt_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Publisher {
    fn element_name() -> &'static str {
        "publisher"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_publisher_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for PubPlace {
    fn element_name() -> &'static str {
        "pubPlace"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_pub_place_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Availability {
    fn element_name() -> &'static str {
        "availability"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_availability_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Distributor {
    fn element_name() -> &'static str {
        "distributor"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_distributor_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for UseRestrict {
    fn element_name() -> &'static str {
        "useRestrict"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_use_restrict_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for AccessRestrict {
    fn element_name() -> &'static str {
        "accessRestrict"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_access_restrict_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for SysReq {
    fn element_name() -> &'static str {
        "sysReq"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_sys_req_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Price {
    fn element_name() -> &'static str {
        "price"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_price_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Unpub {
    fn element_name() -> &'static str {
        "unpub"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_unpub_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<pubStmt>` element from within another element.
pub(crate) fn parse_pub_stmt_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PubStmt> {
    let mut pub_stmt = PubStmt::default();

    // Extract attributes into each attribute class
    pub_stmt.common.extract_attributes(&mut attrs)?;
    pub_stmt.bibl.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("pubStmt")?
        {
            match name.as_str() {
                "head" => {
                    let head = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt.children.push(PubStmtChild::Head(Box::new(head)));
                }
                "unpub" => {
                    let unpub = parse_unpub_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt.children.push(PubStmtChild::Unpub(Box::new(unpub)));
                }
                "publisher" => {
                    let publisher = parse_publisher_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::Publisher(Box::new(publisher)));
                }
                "pubPlace" => {
                    let pub_place = parse_pub_place_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::PubPlace(Box::new(pub_place)));
                }
                "date" => {
                    let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt.children.push(PubStmtChild::Date(Box::new(date)));
                }
                "identifier" => {
                    let identifier = parse_identifier_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::Identifier(Box::new(identifier)));
                }
                "availability" => {
                    let availability =
                        parse_availability_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::Availability(Box::new(availability)));
                }
                "distributor" => {
                    let distributor =
                        parse_distributor_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::Distributor(Box::new(distributor)));
                }
                "respStmt" => {
                    let resp_stmt =
                        super::parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::RespStmt(Box::new(resp_stmt)));
                }
                "address" => {
                    let address =
                        super::parse_address_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::Address(Box::new(address)));
                }
                // Unknown children are skipped in lenient mode
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(pub_stmt)
}

/// Parse an `<unpub>` element from within another element.
pub(crate) fn parse_unpub_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Unpub> {
    let mut unpub = Unpub::default();

    // Extract attributes
    unpub.common.extract_attributes(&mut attrs)?;
    unpub.bibl.extract_attributes(&mut attrs)?;
    unpub.lang.extract_attributes(&mut attrs)?;

    // unpub can contain text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("unpub")? {
            if !text.is_empty() {
                unpub.children.push(UnpubChild::Text(text));
            }
        }
    }

    Ok(unpub)
}

/// Parse a `<publisher>` element from within another element.
pub(crate) fn parse_publisher_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Publisher> {
    let mut publisher = Publisher::default();

    // Extract attributes
    publisher.common.extract_attributes(&mut attrs)?;
    publisher.bibl.extract_attributes(&mut attrs)?;
    publisher.facsimile.extract_attributes(&mut attrs)?;
    publisher.lang.extract_attributes(&mut attrs)?;

    // publisher can contain mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("publisher")? {
            match content {
                MixedContent::Text(text) => {
                    publisher.children.push(PublisherChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "corpName" => {
                            let corp_name = super::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            publisher
                                .children
                                .push(PublisherChild::CorpName(Box::new(corp_name)));
                        }
                        "persName" => {
                            let pers_name = super::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            publisher
                                .children
                                .push(PublisherChild::PersName(Box::new(pers_name)));
                        }
                        "address" => {
                            let address =
                                super::parse_address_from_event(reader, child_attrs, child_empty)?;
                            publisher
                                .children
                                .push(PublisherChild::Address(Box::new(address)));
                        }
                        "ref" => {
                            let ref_elem =
                                super::parse_ref_from_event(reader, child_attrs, child_empty)?;
                            publisher
                                .children
                                .push(PublisherChild::Ref(Box::new(ref_elem)));
                        }
                        "identifier" => {
                            let identifier =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            publisher
                                .children
                                .push(PublisherChild::Identifier(Box::new(identifier)));
                        }
                        "date" => {
                            let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                            publisher
                                .children
                                .push(PublisherChild::Date(Box::new(date)));
                        }
                        "ptr" => {
                            let ptr =
                                super::parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            publisher.children.push(PublisherChild::Ptr(Box::new(ptr)));
                        }
                        "rend" => {
                            let rend = super::super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            publisher
                                .children
                                .push(PublisherChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            publisher.children.push(PublisherChild::Lb(Box::new(lb)));
                        }
                        _ => {
                            // Skip unknown children
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(publisher)
}

/// Parse a `<pubPlace>` element from within another element.
pub(crate) fn parse_pub_place_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PubPlace> {
    let mut pub_place = PubPlace::default();

    // Extract attributes
    pub_place.common.extract_attributes(&mut attrs)?;
    pub_place.bibl.extract_attributes(&mut attrs)?;
    pub_place.facsimile.extract_attributes(&mut attrs)?;
    pub_place.lang.extract_attributes(&mut attrs)?;

    // pubPlace can contain mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("pubPlace")? {
            match content {
                MixedContent::Text(text) => {
                    pub_place.children.push(PubPlaceChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "geogName" => {
                            let geog_name = super::parse_geog_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pub_place
                                .children
                                .push(PubPlaceChild::GeogName(Box::new(geog_name)));
                        }
                        "address" => {
                            let address =
                                super::parse_address_from_event(reader, child_attrs, child_empty)?;
                            pub_place
                                .children
                                .push(PubPlaceChild::Address(Box::new(address)));
                        }
                        "ref" => {
                            let ref_elem =
                                super::parse_ref_from_event(reader, child_attrs, child_empty)?;
                            pub_place
                                .children
                                .push(PubPlaceChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr =
                                super::parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            pub_place.children.push(PubPlaceChild::Ptr(Box::new(ptr)));
                        }
                        "persName" => {
                            let pers_name = super::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pub_place
                                .children
                                .push(PubPlaceChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pub_place
                                .children
                                .push(PubPlaceChild::CorpName(Box::new(corp_name)));
                        }
                        "date" => {
                            let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                            pub_place.children.push(PubPlaceChild::Date(Box::new(date)));
                        }
                        "identifier" => {
                            let identifier =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            pub_place
                                .children
                                .push(PubPlaceChild::Identifier(Box::new(identifier)));
                        }
                        "name" => {
                            let name_elem =
                                super::parse_name_from_event(reader, child_attrs, child_empty)?;
                            pub_place
                                .children
                                .push(PubPlaceChild::Name(Box::new(name_elem)));
                        }
                        "annot" => {
                            let annot =
                                super::parse_annot_from_event(reader, child_attrs, child_empty)?;
                            pub_place
                                .children
                                .push(PubPlaceChild::Annot(Box::new(annot)));
                        }
                        "lb" => {
                            let lb = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pub_place.children.push(PubPlaceChild::Lb(Box::new(lb)));
                        }
                        "rend" => {
                            let rend = super::super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pub_place.children.push(PubPlaceChild::Rend(Box::new(rend)));
                        }
                        _ => {
                            // Skip unknown children
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(pub_place)
}

/// Parse a `<date>` element from within another element.
pub(crate) fn parse_date_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Date> {
    let mut date = Date::default();

    // Extract attributes
    date.common.extract_attributes(&mut attrs)?;
    date.bibl.extract_attributes(&mut attrs)?;
    date.calendared.extract_attributes(&mut attrs)?;
    date.datable.extract_attributes(&mut attrs)?;
    date.edit.extract_attributes(&mut attrs)?;
    date.facsimile.extract_attributes(&mut attrs)?;
    date.lang.extract_attributes(&mut attrs)?;

    // date can contain text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("date")? {
            if !text.is_empty() {
                date.children.push(DateChild::Text(text));
            }
        }
    }

    Ok(date)
}

/// Parse an `<identifier>` element from within another element.
pub(crate) fn parse_identifier_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Identifier> {
    let mut identifier = Identifier::default();

    // Extract attributes
    identifier.common.extract_attributes(&mut attrs)?;
    identifier.authorized.extract_attributes(&mut attrs)?;
    identifier.bibl.extract_attributes(&mut attrs)?;
    identifier.facsimile.extract_attributes(&mut attrs)?;

    // identifier can contain mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("identifier")? {
            match content {
                MixedContent::Text(text) => {
                    identifier.children.push(IdentifierChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "ref" => {
                            let ref_elem =
                                super::parse_ref_from_event(reader, child_attrs, child_empty)?;
                            identifier
                                .children
                                .push(IdentifierChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr =
                                super::parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            identifier
                                .children
                                .push(IdentifierChild::Ptr(Box::new(ptr)));
                        }
                        "rend" => {
                            let rend = super::super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            identifier
                                .children
                                .push(IdentifierChild::Rend(Box::new(rend)));
                        }
                        "name" => {
                            let name_elem =
                                super::parse_name_from_event(reader, child_attrs, child_empty)?;
                            identifier
                                .children
                                .push(IdentifierChild::Name(Box::new(name_elem)));
                        }
                        "persName" => {
                            let pers_name = super::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            identifier
                                .children
                                .push(IdentifierChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            identifier
                                .children
                                .push(IdentifierChild::CorpName(Box::new(corp_name)));
                        }
                        "date" => {
                            let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                            identifier
                                .children
                                .push(IdentifierChild::Date(Box::new(date)));
                        }
                        "lb" => {
                            let lb = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            identifier.children.push(IdentifierChild::Lb(Box::new(lb)));
                        }
                        _ => {
                            // Skip unknown children
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(identifier)
}

/// Parse an `<availability>` element from within another element.
pub(crate) fn parse_availability_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Availability> {
    let mut availability = Availability::default();

    // Extract attributes
    availability.common.extract_attributes(&mut attrs)?;
    availability.bibl.extract_attributes(&mut attrs)?;
    availability.data_pointing.extract_attributes(&mut attrs)?;

    // availability can contain mixed content: text and elements
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("availability")? {
            match content {
                MixedContent::Text(text) => {
                    availability.children.push(AvailabilityChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "head" => {
                        let head = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                        availability
                            .children
                            .push(AvailabilityChild::Head(Box::new(head)));
                    }
                    "identifier" => {
                        let identifier =
                            parse_identifier_from_event(reader, child_attrs, child_empty)?;
                        availability
                            .children
                            .push(AvailabilityChild::Identifier(Box::new(identifier)));
                    }
                    "distributor" => {
                        let distributor =
                            parse_distributor_from_event(reader, child_attrs, child_empty)?;
                        availability
                            .children
                            .push(AvailabilityChild::Distributor(Box::new(distributor)));
                    }
                    "useRestrict" => {
                        let use_restrict =
                            parse_use_restrict_from_event(reader, child_attrs, child_empty)?;
                        availability
                            .children
                            .push(AvailabilityChild::UseRestrict(Box::new(use_restrict)));
                    }
                    "date" => {
                        let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                        availability
                            .children
                            .push(AvailabilityChild::Date(Box::new(date)));
                    }
                    "accessRestrict" => {
                        let access_restrict =
                            parse_access_restrict_from_event(reader, child_attrs, child_empty)?;
                        availability
                            .children
                            .push(AvailabilityChild::AccessRestrict(Box::new(access_restrict)));
                    }
                    "price" => {
                        let price = parse_price_from_event(reader, child_attrs, child_empty)?;
                        availability
                            .children
                            .push(AvailabilityChild::Price(Box::new(price)));
                    }
                    "address" => {
                        let address =
                            super::parse_address_from_event(reader, child_attrs, child_empty)?;
                        availability
                            .children
                            .push(AvailabilityChild::Address(Box::new(address)));
                    }
                    "sysReq" => {
                        let sys_req = parse_sys_req_from_event(reader, child_attrs, child_empty)?;
                        availability
                            .children
                            .push(AvailabilityChild::SysReq(Box::new(sys_req)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                },
            }
        }
    }

    Ok(availability)
}

/// Parse a `<distributor>` element from within another element.
pub(crate) fn parse_distributor_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Distributor> {
    let mut distributor = Distributor::default();

    // Extract attributes
    distributor.common.extract_attributes(&mut attrs)?;
    distributor.bibl.extract_attributes(&mut attrs)?;
    distributor.facsimile.extract_attributes(&mut attrs)?;
    distributor.lang.extract_attributes(&mut attrs)?;

    // distributor can contain mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("distributor")? {
            match content {
                MixedContent::Text(text) => {
                    distributor.children.push(DistributorChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "corpName" => {
                            let corp_name = super::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            distributor
                                .children
                                .push(DistributorChild::CorpName(Box::new(corp_name)));
                        }
                        "persName" => {
                            let pers_name = super::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            distributor
                                .children
                                .push(DistributorChild::PersName(Box::new(pers_name)));
                        }
                        "address" => {
                            let address =
                                super::parse_address_from_event(reader, child_attrs, child_empty)?;
                            distributor
                                .children
                                .push(DistributorChild::Address(Box::new(address)));
                        }
                        "ref" => {
                            let ref_elem =
                                super::parse_ref_from_event(reader, child_attrs, child_empty)?;
                            distributor
                                .children
                                .push(DistributorChild::Ref(Box::new(ref_elem)));
                        }
                        "identifier" => {
                            let identifier =
                                parse_identifier_from_event(reader, child_attrs, child_empty)?;
                            distributor
                                .children
                                .push(DistributorChild::Identifier(Box::new(identifier)));
                        }
                        "date" => {
                            let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                            distributor
                                .children
                                .push(DistributorChild::Date(Box::new(date)));
                        }
                        "ptr" => {
                            let ptr =
                                super::parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            distributor
                                .children
                                .push(DistributorChild::Ptr(Box::new(ptr)));
                        }
                        "rend" => {
                            let rend = super::super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            distributor
                                .children
                                .push(DistributorChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            distributor
                                .children
                                .push(DistributorChild::Lb(Box::new(lb)));
                        }
                        _ => {
                            // Skip unknown children
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(distributor)
}

/// Parse a `<useRestrict>` element from within another element.
pub(crate) fn parse_use_restrict_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<UseRestrict> {
    let mut use_restrict = UseRestrict::default();

    // Extract attributes
    use_restrict.common.extract_attributes(&mut attrs)?;
    use_restrict.authorized.extract_attributes(&mut attrs)?;
    use_restrict.bibl.extract_attributes(&mut attrs)?;
    use_restrict.lang.extract_attributes(&mut attrs)?;

    // useRestrict can contain mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("useRestrict")? {
            match content {
                MixedContent::Text(text) => {
                    use_restrict.children.push(UseRestrictChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "p" => {
                            let p = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                            use_restrict.children.push(UseRestrictChild::P(Box::new(p)));
                        }
                        "head" => {
                            let head =
                                super::parse_head_from_event(reader, child_attrs, child_empty)?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::Head(Box::new(head)));
                        }
                        "ref" => {
                            let ref_elem =
                                super::parse_ref_from_event(reader, child_attrs, child_empty)?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr =
                                super::parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::Ptr(Box::new(ptr)));
                        }
                        "rend" => {
                            let rend = super::super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            use_restrict
                                .children
                                .push(UseRestrictChild::Lb(Box::new(lb)));
                        }
                        _ => {
                            // Skip unknown children
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(use_restrict)
}

/// Parse an `<accessRestrict>` element from within another element.
pub(crate) fn parse_access_restrict_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<AccessRestrict> {
    let mut access_restrict = AccessRestrict::default();

    // Extract attributes
    access_restrict.common.extract_attributes(&mut attrs)?;
    access_restrict.authorized.extract_attributes(&mut attrs)?;
    access_restrict.bibl.extract_attributes(&mut attrs)?;
    access_restrict.lang.extract_attributes(&mut attrs)?;

    // accessRestrict can contain mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("accessRestrict")? {
            match content {
                MixedContent::Text(text) => {
                    access_restrict
                        .children
                        .push(AccessRestrictChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "p" => {
                            let p = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                            access_restrict
                                .children
                                .push(AccessRestrictChild::P(Box::new(p)));
                        }
                        "head" => {
                            let head =
                                super::parse_head_from_event(reader, child_attrs, child_empty)?;
                            access_restrict
                                .children
                                .push(AccessRestrictChild::Head(Box::new(head)));
                        }
                        "ref" => {
                            let ref_elem =
                                super::parse_ref_from_event(reader, child_attrs, child_empty)?;
                            access_restrict
                                .children
                                .push(AccessRestrictChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr =
                                super::parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            access_restrict
                                .children
                                .push(AccessRestrictChild::Ptr(Box::new(ptr)));
                        }
                        "rend" => {
                            let rend = super::super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            access_restrict
                                .children
                                .push(AccessRestrictChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            access_restrict
                                .children
                                .push(AccessRestrictChild::Lb(Box::new(lb)));
                        }
                        _ => {
                            // Skip unknown children
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(access_restrict)
}

/// Parse a `<sysReq>` element from within another element.
pub(crate) fn parse_sys_req_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<SysReq> {
    let mut sys_req = SysReq::default();

    // Extract attributes
    sys_req.common.extract_attributes(&mut attrs)?;
    sys_req.bibl.extract_attributes(&mut attrs)?;
    sys_req.lang.extract_attributes(&mut attrs)?;

    // sysReq can contain mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("sysReq")? {
            match content {
                MixedContent::Text(text) => {
                    sys_req.children.push(SysReqChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "p" => {
                            let p = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                            sys_req.children.push(SysReqChild::P(Box::new(p)));
                        }
                        "head" => {
                            let head =
                                super::parse_head_from_event(reader, child_attrs, child_empty)?;
                            sys_req.children.push(SysReqChild::Head(Box::new(head)));
                        }
                        "ref" => {
                            let ref_elem =
                                super::parse_ref_from_event(reader, child_attrs, child_empty)?;
                            sys_req.children.push(SysReqChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr =
                                super::parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            sys_req.children.push(SysReqChild::Ptr(Box::new(ptr)));
                        }
                        "rend" => {
                            let rend = super::super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            sys_req.children.push(SysReqChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            sys_req.children.push(SysReqChild::Lb(Box::new(lb)));
                        }
                        _ => {
                            // Skip unknown children
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(sys_req)
}

/// Parse a `<price>` element from within another element.
pub(crate) fn parse_price_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Price> {
    let mut price = Price::default();

    // Extract attributes
    price.common.extract_attributes(&mut attrs)?;
    price.bibl.extract_attributes(&mut attrs)?;
    price.lang.extract_attributes(&mut attrs)?;

    // Extract element-specific attributes
    extract_attr!(attrs, "amount", price.amount);
    extract_attr!(attrs, "currency", string price.currency);

    // price can contain text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("price")? {
            if !text.is_empty() {
                price.children.push(PriceChild::Text(text));
            }
        }
    }

    Ok(price)
}
