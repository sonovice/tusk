//! MEI pattern entities (generated from ODD).
//!
//! Pattern entities define reusable content patterns that can be referenced
//! by element content models via macroRef.
//!
//! DO NOT EDIT - regenerate with: cargo run -p mei-codegen
use serde::{Deserialize, Serialize};
///Content for pattern entity `macro.metaLike.page`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MacroMetaLikePageContent {
    #[serde(rename = "pgDesc")]
    PgDesc(Box<crate::generated::elements::PgDesc>),
    #[serde(rename = "pgHead")]
    PgHead(Box<crate::generated::elements::PgHead>),
    #[serde(rename = "pgFoot")]
    PgFoot(Box<crate::generated::elements::PgFoot>),
}
///Groups elements that contain meta-data about a single page.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MacroMetaLikePage {
    /// Pattern entity content.
    #[serde(default, rename = "$value")]
    pub content: Vec<MacroMetaLikePageContent>,
}
///Content for pattern entity `macro.watermark`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MacroWatermarkContent {
    #[serde(rename = "country")]
    Country(Box<crate::generated::elements::Country>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "depth")]
    Depth(Box<crate::generated::elements::Depth>),
    #[serde(rename = "fig")]
    Fig(Box<crate::generated::elements::Fig>),
    #[serde(rename = "num")]
    Num(Box<crate::generated::elements::Num>),
    #[serde(rename = "dimensions")]
    Dimensions(Box<crate::generated::elements::Dimensions>),
    #[serde(rename = "ref")]
    Ref(Box<crate::generated::elements::Ref>),
    #[serde(rename = "rend")]
    Rend(Box<crate::generated::elements::Rend>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
    #[serde(rename = "height")]
    Height(Box<crate::generated::elements::Height>),
    #[serde(rename = "bloc")]
    Bloc(Box<crate::generated::elements::Bloc>),
    #[serde(rename = "geogName")]
    GeogName(Box<crate::generated::elements::GeogName>),
    #[serde(rename = "address")]
    Address(Box<crate::generated::elements::Address>),
    #[serde(rename = "region")]
    Region(Box<crate::generated::elements::Region>),
    #[serde(rename = "stack")]
    Stack(Box<crate::generated::elements::Stack>),
    #[serde(rename = "biblList")]
    BiblList(Box<crate::generated::elements::BiblList>),
    #[serde(rename = "ptr")]
    Ptr(Box<crate::generated::elements::Ptr>),
    #[serde(rename = "term")]
    Term(Box<crate::generated::elements::Term>),
    #[serde(rename = "title")]
    Title(Box<crate::generated::elements::Title>),
    #[serde(rename = "relationList")]
    RelationList(Box<crate::generated::elements::RelationList>),
    #[serde(rename = "geogFeat")]
    GeogFeat(Box<crate::generated::elements::GeogFeat>),
    #[serde(rename = "relation")]
    Relation(Box<crate::generated::elements::Relation>),
    #[serde(rename = "extent")]
    Extent(Box<crate::generated::elements::Extent>),
    #[serde(rename = "bibl")]
    Bibl(Box<crate::generated::elements::Bibl>),
    #[serde(rename = "date")]
    Date(Box<crate::generated::elements::Date>),
    #[serde(rename = "heraldry")]
    Heraldry(Box<crate::generated::elements::Heraldry>),
    #[serde(rename = "locus")]
    Locus(Box<crate::generated::elements::Locus>),
    #[serde(rename = "locusGrp")]
    LocusGrp(Box<crate::generated::elements::LocusGrp>),
    #[serde(rename = "settlement")]
    Settlement(Box<crate::generated::elements::Settlement>),
    #[serde(rename = "biblStruct")]
    BiblStruct(Box<crate::generated::elements::BiblStruct>),
    #[serde(rename = "district")]
    District(Box<crate::generated::elements::District>),
    #[serde(rename = "dim")]
    Dim(Box<crate::generated::elements::Dim>),
    #[serde(rename = "width")]
    Width(Box<crate::generated::elements::Width>),
    #[serde(rename = "q")]
    Q(Box<crate::generated::elements::Q>),
}
///Groups elements that contain meta-data about a watermark.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MacroWatermark {
    /// Pattern entity content.
    #[serde(default, rename = "$value")]
    pub content: Vec<MacroWatermarkContent>,
}
///Permits any XML elements except those from the MEI or SVG namespace.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MacroAnyXml;
///Content for pattern entity `macro.titlePart`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MacroTitlePartContent {
    #[serde(rename = "settlement")]
    Settlement(Box<crate::generated::elements::Settlement>),
    #[serde(rename = "pb")]
    Pb(Box<crate::generated::elements::Pb>),
    #[serde(rename = "biblStruct")]
    BiblStruct(Box<crate::generated::elements::BiblStruct>),
    #[serde(rename = "name")]
    Name(Box<crate::generated::elements::Name>),
    #[serde(rename = "persName")]
    PersName(Box<crate::generated::elements::PersName>),
    #[serde(rename = "street")]
    Street(Box<crate::generated::elements::Street>),
    #[serde(rename = "fig")]
    Fig(Box<crate::generated::elements::Fig>),
    #[serde(rename = "geogName")]
    GeogName(Box<crate::generated::elements::GeogName>),
    #[serde(rename = "postCode")]
    PostCode(Box<crate::generated::elements::PostCode>),
    #[serde(rename = "bloc")]
    Bloc(Box<crate::generated::elements::Bloc>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
    #[serde(rename = "abbr")]
    Abbr(Box<crate::generated::elements::Abbr>),
    #[serde(rename = "expan")]
    Expan(Box<crate::generated::elements::Expan>),
    #[serde(rename = "lb")]
    Lb(Box<crate::generated::elements::Lb>),
    #[serde(rename = "ptr")]
    Ptr(Box<crate::generated::elements::Ptr>),
    #[serde(rename = "repository")]
    Repository(Box<crate::generated::elements::Repository>),
    #[serde(rename = "periodName")]
    PeriodName(Box<crate::generated::elements::PeriodName>),
    #[serde(rename = "country")]
    Country(Box<crate::generated::elements::Country>),
    #[serde(rename = "bibl")]
    Bibl(Box<crate::generated::elements::Bibl>),
    #[serde(rename = "region")]
    Region(Box<crate::generated::elements::Region>),
    #[serde(rename = "date")]
    Date(Box<crate::generated::elements::Date>),
    #[serde(rename = "symbol")]
    Symbol(Box<crate::generated::elements::Symbol>),
    #[serde(rename = "styleName")]
    StyleName(Box<crate::generated::elements::StyleName>),
    #[serde(rename = "address")]
    Address(Box<crate::generated::elements::Address>),
    #[serde(rename = "num")]
    Num(Box<crate::generated::elements::Num>),
    #[serde(rename = "ref")]
    Ref(Box<crate::generated::elements::Ref>),
    #[serde(rename = "corpName")]
    CorpName(Box<crate::generated::elements::CorpName>),
    #[serde(rename = "geogFeat")]
    GeogFeat(Box<crate::generated::elements::GeogFeat>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "rend")]
    Rend(Box<crate::generated::elements::Rend>),
    #[serde(rename = "district")]
    District(Box<crate::generated::elements::District>),
    #[serde(rename = "postBox")]
    PostBox(Box<crate::generated::elements::PostBox>),
    #[serde(rename = "stack")]
    Stack(Box<crate::generated::elements::Stack>),
}
///Groups elements that may appear as part of a bibliographic title.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MacroTitlePart {
    /// Pattern entity content.
    #[serde(default, rename = "$value")]
    pub content: Vec<MacroTitlePartContent>,
}
///Content for pattern entity `macro.struc-unstrucContent`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MacroStrucUnstrucContentContent {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
    #[serde(rename = "periodName")]
    PeriodName(Box<crate::generated::elements::PeriodName>),
    #[serde(rename = "postBox")]
    PostBox(Box<crate::generated::elements::PostBox>),
    #[serde(rename = "region")]
    Region(Box<crate::generated::elements::Region>),
    #[serde(rename = "secFolio")]
    SecFolio(Box<crate::generated::elements::SecFolio>),
    #[serde(rename = "symbol")]
    Symbol(Box<crate::generated::elements::Symbol>),
    #[serde(rename = "name")]
    Name(Box<crate::generated::elements::Name>),
    #[serde(rename = "title")]
    Title(Box<crate::generated::elements::Title>),
    #[serde(rename = "p")]
    P(Box<crate::generated::elements::P>),
    #[serde(rename = "lb")]
    Lb(Box<crate::generated::elements::Lb>),
    #[serde(rename = "signatures")]
    Signatures(Box<crate::generated::elements::Signatures>),
    #[serde(rename = "catchwords")]
    Catchwords(Box<crate::generated::elements::Catchwords>),
    #[serde(rename = "extent")]
    Extent(Box<crate::generated::elements::Extent>),
    #[serde(rename = "ref")]
    Ref(Box<crate::generated::elements::Ref>),
    #[serde(rename = "relation")]
    Relation(Box<crate::generated::elements::Relation>),
    #[serde(rename = "height")]
    Height(Box<crate::generated::elements::Height>),
    #[serde(rename = "abbr")]
    Abbr(Box<crate::generated::elements::Abbr>),
    #[serde(rename = "settlement")]
    Settlement(Box<crate::generated::elements::Settlement>),
    #[serde(rename = "dim")]
    Dim(Box<crate::generated::elements::Dim>),
    #[serde(rename = "street")]
    Street(Box<crate::generated::elements::Street>),
    #[serde(rename = "heraldry")]
    Heraldry(Box<crate::generated::elements::Heraldry>),
    #[serde(rename = "geogFeat")]
    GeogFeat(Box<crate::generated::elements::GeogFeat>),
    #[serde(rename = "district")]
    District(Box<crate::generated::elements::District>),
    #[serde(rename = "q")]
    Q(Box<crate::generated::elements::Q>),
    #[serde(rename = "width")]
    Width(Box<crate::generated::elements::Width>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
    #[serde(rename = "persName")]
    PersName(Box<crate::generated::elements::PersName>),
    #[serde(rename = "rend")]
    Rend(Box<crate::generated::elements::Rend>),
    #[serde(rename = "term")]
    Term(Box<crate::generated::elements::Term>),
    #[serde(rename = "date")]
    Date(Box<crate::generated::elements::Date>),
    #[serde(rename = "num")]
    Num(Box<crate::generated::elements::Num>),
    #[serde(rename = "fig")]
    Fig(Box<crate::generated::elements::Fig>),
    #[serde(rename = "postCode")]
    PostCode(Box<crate::generated::elements::PostCode>),
    #[serde(rename = "expan")]
    Expan(Box<crate::generated::elements::Expan>),
    #[serde(rename = "biblStruct")]
    BiblStruct(Box<crate::generated::elements::BiblStruct>),
    #[serde(rename = "locus")]
    Locus(Box<crate::generated::elements::Locus>),
    #[serde(rename = "ptr")]
    Ptr(Box<crate::generated::elements::Ptr>),
    #[serde(rename = "bloc")]
    Bloc(Box<crate::generated::elements::Bloc>),
    #[serde(rename = "repository")]
    Repository(Box<crate::generated::elements::Repository>),
    #[serde(rename = "seg")]
    Seg(Box<crate::generated::elements::Seg>),
    #[serde(rename = "dimensions")]
    Dimensions(Box<crate::generated::elements::Dimensions>),
    #[serde(rename = "styleName")]
    StyleName(Box<crate::generated::elements::StyleName>),
    #[serde(rename = "address")]
    Address(Box<crate::generated::elements::Address>),
    #[serde(rename = "stack")]
    Stack(Box<crate::generated::elements::Stack>),
    #[serde(rename = "stamp")]
    Stamp(Box<crate::generated::elements::Stamp>),
    #[serde(rename = "relationList")]
    RelationList(Box<crate::generated::elements::RelationList>),
    #[serde(rename = "corpName")]
    CorpName(Box<crate::generated::elements::CorpName>),
    #[serde(rename = "depth")]
    Depth(Box<crate::generated::elements::Depth>),
    #[serde(rename = "geogName")]
    GeogName(Box<crate::generated::elements::GeogName>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "locusGrp")]
    LocusGrp(Box<crate::generated::elements::LocusGrp>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "country")]
    Country(Box<crate::generated::elements::Country>),
    #[serde(rename = "bibl")]
    Bibl(Box<crate::generated::elements::Bibl>),
}
///Provides a choice between structured and unstructured/mixed content.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MacroStrucUnstrucContent {
    /// Pattern entity content.
    #[serde(default, rename = "$value")]
    pub content: Vec<MacroStrucUnstrucContentContent>,
}
///Content for pattern entity `macro.musicPart`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MacroMusicPartContent {
    #[serde(rename = "front")]
    Front(Box<crate::generated::elements::Front>),
    #[serde(rename = "back")]
    Back(Box<crate::generated::elements::Back>),
    #[serde(rename = "group")]
    Group(Box<crate::generated::elements::Group>),
    #[serde(rename = "body")]
    Body(Box<crate::generated::elements::Body>),
}
///Groups elements that may appear as part of the music element.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MacroMusicPart {
    /// Pattern entity content.
    #[serde(default, rename = "$value")]
    pub content: Vec<MacroMusicPartContent>,
}
///Content for pattern entity `macro.availabilityPart`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MacroAvailabilityPartContent {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "date")]
    Date(Box<crate::generated::elements::Date>),
    #[serde(rename = "accessRestrict")]
    AccessRestrict(Box<crate::generated::elements::AccessRestrict>),
    #[serde(rename = "address")]
    Address(Box<crate::generated::elements::Address>),
    #[serde(rename = "useRestrict")]
    UseRestrict(Box<crate::generated::elements::UseRestrict>),
    #[serde(rename = "price")]
    Price(Box<crate::generated::elements::Price>),
    #[serde(rename = "sysReq")]
    SysReq(Box<crate::generated::elements::SysReq>),
    #[serde(rename = "distributor")]
    Distributor(Box<crate::generated::elements::Distributor>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
}
/**Groups elements that may appear as part of a description of the availability of and access
to a bibliographic item.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MacroAvailabilityPart {
    /// Pattern entity content.
    #[serde(default, rename = "$value")]
    pub content: Vec<MacroAvailabilityPartContent>,
}
///Content for pattern entity `macro.bibldescPart`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MacroBibldescPartContent {
    #[serde(rename = "availability")]
    Availability(Box<crate::generated::elements::Availability>),
    #[serde(rename = "pubStmt")]
    PubStmt(Box<crate::generated::elements::PubStmt>),
    #[serde(rename = "editionStmt")]
    EditionStmt(Box<crate::generated::elements::EditionStmt>),
    #[serde(rename = "physLoc")]
    PhysLoc(Box<crate::generated::elements::PhysLoc>),
    #[serde(rename = "seriesStmt")]
    SeriesStmt(Box<crate::generated::elements::SeriesStmt>),
    #[serde(rename = "physDesc")]
    PhysDesc(Box<crate::generated::elements::PhysDesc>),
}
/**Groups manifestation- and item-specific elements that may appear as part of a
bibliographic description.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MacroBibldescPart {
    /// Pattern entity content.
    #[serde(default, rename = "$value")]
    pub content: Vec<MacroBibldescPartContent>,
}
