//!Element: `<bibl>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<bibl>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BiblChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
    #[serde(rename = "dim")]
    Dim(Box<crate::generated::elements::Dim>),
    #[serde(rename = "district")]
    District(Box<crate::generated::elements::District>),
    #[serde(rename = "styleName")]
    StyleName(Box<crate::generated::elements::StyleName>),
    #[serde(rename = "genre")]
    Genre(Box<crate::generated::elements::Genre>),
    #[serde(rename = "date")]
    Date(Box<crate::generated::elements::Date>),
    #[serde(rename = "textLang")]
    TextLang(Box<crate::generated::elements::TextLang>),
    #[serde(rename = "funder")]
    Funder(Box<crate::generated::elements::Funder>),
    #[serde(rename = "symbol")]
    Symbol(Box<crate::generated::elements::Symbol>),
    #[serde(rename = "geogName")]
    GeogName(Box<crate::generated::elements::GeogName>),
    #[serde(rename = "relation")]
    Relation(Box<crate::generated::elements::Relation>),
    #[serde(rename = "unpub")]
    Unpub(Box<crate::generated::elements::Unpub>),
    #[serde(rename = "relationList")]
    RelationList(Box<crate::generated::elements::RelationList>),
    #[serde(rename = "address")]
    Address(Box<crate::generated::elements::Address>),
    #[serde(rename = "geogFeat")]
    GeogFeat(Box<crate::generated::elements::GeogFeat>),
    #[serde(rename = "physLoc")]
    PhysLoc(Box<crate::generated::elements::PhysLoc>),
    #[serde(rename = "ptr")]
    Ptr(Box<crate::generated::elements::Ptr>),
    #[serde(rename = "periodName")]
    PeriodName(Box<crate::generated::elements::PeriodName>),
    #[serde(rename = "pb")]
    Pb(Box<crate::generated::elements::Pb>),
    #[serde(rename = "persName")]
    PersName(Box<crate::generated::elements::PersName>),
    #[serde(rename = "series")]
    Series(Box<crate::generated::elements::Series>),
    #[serde(rename = "perfDuration")]
    PerfDuration(Box<crate::generated::elements::PerfDuration>),
    #[serde(rename = "extent")]
    Extent(Box<crate::generated::elements::Extent>),
    #[serde(rename = "depth")]
    Depth(Box<crate::generated::elements::Depth>),
    #[serde(rename = "region")]
    Region(Box<crate::generated::elements::Region>),
    #[serde(rename = "fig")]
    Fig(Box<crate::generated::elements::Fig>),
    #[serde(rename = "ref")]
    Ref(Box<crate::generated::elements::Ref>),
    #[serde(rename = "lb")]
    Lb(Box<crate::generated::elements::Lb>),
    #[serde(rename = "q")]
    Q(Box<crate::generated::elements::Q>),
    #[serde(rename = "contributor")]
    Contributor(Box<crate::generated::elements::Contributor>),
    #[serde(rename = "sponsor")]
    Sponsor(Box<crate::generated::elements::Sponsor>),
    #[serde(rename = "name")]
    Name(Box<crate::generated::elements::Name>),
    #[serde(rename = "signatures")]
    Signatures(Box<crate::generated::elements::Signatures>),
    #[serde(rename = "settlement")]
    Settlement(Box<crate::generated::elements::Settlement>),
    #[serde(rename = "street")]
    Street(Box<crate::generated::elements::Street>),
    #[serde(rename = "creator")]
    Creator(Box<crate::generated::elements::Creator>),
    #[serde(rename = "publisher")]
    Publisher(Box<crate::generated::elements::Publisher>),
    #[serde(rename = "expan")]
    Expan(Box<crate::generated::elements::Expan>),
    #[serde(rename = "availability")]
    Availability(Box<crate::generated::elements::Availability>),
    #[serde(rename = "repository")]
    Repository(Box<crate::generated::elements::Repository>),
    #[serde(rename = "bloc")]
    Bloc(Box<crate::generated::elements::Bloc>),
    #[serde(rename = "height")]
    Height(Box<crate::generated::elements::Height>),
    #[serde(rename = "stack")]
    Stack(Box<crate::generated::elements::Stack>),
    #[serde(rename = "bibl")]
    Bibl(Box<crate::generated::elements::Bibl>),
    #[serde(rename = "recipient")]
    Recipient(Box<crate::generated::elements::Recipient>),
    #[serde(rename = "heraldry")]
    Heraldry(Box<crate::generated::elements::Heraldry>),
    #[serde(rename = "num")]
    Num(Box<crate::generated::elements::Num>),
    #[serde(rename = "distributor")]
    Distributor(Box<crate::generated::elements::Distributor>),
    #[serde(rename = "respStmt")]
    RespStmt(Box<crate::generated::elements::RespStmt>),
    #[serde(rename = "biblStruct")]
    BiblStruct(Box<crate::generated::elements::BiblStruct>),
    #[serde(rename = "imprint")]
    Imprint(Box<crate::generated::elements::Imprint>),
    #[serde(rename = "catchwords")]
    Catchwords(Box<crate::generated::elements::Catchwords>),
    #[serde(rename = "biblScope")]
    BiblScope(Box<crate::generated::elements::BiblScope>),
    #[serde(rename = "pubPlace")]
    PubPlace(Box<crate::generated::elements::PubPlace>),
    #[serde(rename = "stamp")]
    Stamp(Box<crate::generated::elements::Stamp>),
    #[serde(rename = "secFolio")]
    SecFolio(Box<crate::generated::elements::SecFolio>),
    #[serde(rename = "postCode")]
    PostCode(Box<crate::generated::elements::PostCode>),
    #[serde(rename = "width")]
    Width(Box<crate::generated::elements::Width>),
    #[serde(rename = "editor")]
    Editor(Box<crate::generated::elements::Editor>),
    #[serde(rename = "corpName")]
    CorpName(Box<crate::generated::elements::CorpName>),
    #[serde(rename = "term")]
    Term(Box<crate::generated::elements::Term>),
    #[serde(rename = "seg")]
    Seg(Box<crate::generated::elements::Seg>),
    #[serde(rename = "creation")]
    Creation(Box<crate::generated::elements::Creation>),
    #[serde(rename = "title")]
    Title(Box<crate::generated::elements::Title>),
    #[serde(rename = "abbr")]
    Abbr(Box<crate::generated::elements::Abbr>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
    #[serde(rename = "rend")]
    Rend(Box<crate::generated::elements::Rend>),
    #[serde(rename = "locusGrp")]
    LocusGrp(Box<crate::generated::elements::LocusGrp>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "country")]
    Country(Box<crate::generated::elements::Country>),
    #[serde(rename = "postBox")]
    PostBox(Box<crate::generated::elements::PostBox>),
    #[serde(rename = "dimensions")]
    Dimensions(Box<crate::generated::elements::Dimensions>),
    #[serde(rename = "locus")]
    Locus(Box<crate::generated::elements::Locus>),
    #[serde(rename = "relatedItem")]
    RelatedItem(Box<crate::generated::elements::RelatedItem>),
    #[serde(rename = "edition")]
    Edition(Box<crate::generated::elements::Edition>),
}
impl BiblChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            BiblChild::Text(_) => {}
            BiblChild::Dim(elem) => {
                ctx.enter("dim", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::District(elem) => {
                ctx.enter("district", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::StyleName(elem) => {
                ctx.enter("styleName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Genre(elem) => {
                ctx.enter("genre", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Date(elem) => {
                ctx.enter("date", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::TextLang(elem) => {
                ctx.enter("textLang", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Funder(elem) => {
                ctx.enter("funder", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Symbol(elem) => {
                ctx.enter("symbol", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::GeogName(elem) => {
                ctx.enter("geogName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Relation(elem) => {
                ctx.enter("relation", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Unpub(elem) => {
                ctx.enter("unpub", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::RelationList(elem) => {
                ctx.enter("relationList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Address(elem) => {
                ctx.enter("address", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::GeogFeat(elem) => {
                ctx.enter("geogFeat", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::PhysLoc(elem) => {
                ctx.enter("physLoc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Ptr(elem) => {
                ctx.enter("ptr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::PeriodName(elem) => {
                ctx.enter("periodName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Pb(elem) => {
                ctx.enter("pb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::PersName(elem) => {
                ctx.enter("persName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Series(elem) => {
                ctx.enter("series", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::PerfDuration(elem) => {
                ctx.enter("perfDuration", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Extent(elem) => {
                ctx.enter("extent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Depth(elem) => {
                ctx.enter("depth", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Region(elem) => {
                ctx.enter("region", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Fig(elem) => {
                ctx.enter("fig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Ref(elem) => {
                ctx.enter("ref", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Lb(elem) => {
                ctx.enter("lb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Q(elem) => {
                ctx.enter("q", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Contributor(elem) => {
                ctx.enter("contributor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Sponsor(elem) => {
                ctx.enter("sponsor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Name(elem) => {
                ctx.enter("name", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Signatures(elem) => {
                ctx.enter("signatures", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Settlement(elem) => {
                ctx.enter("settlement", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Street(elem) => {
                ctx.enter("street", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Creator(elem) => {
                ctx.enter("creator", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Publisher(elem) => {
                ctx.enter("publisher", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Expan(elem) => {
                ctx.enter("expan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Availability(elem) => {
                ctx.enter("availability", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Repository(elem) => {
                ctx.enter("repository", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Bloc(elem) => {
                ctx.enter("bloc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Height(elem) => {
                ctx.enter("height", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Stack(elem) => {
                ctx.enter("stack", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Bibl(elem) => {
                ctx.enter("bibl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Recipient(elem) => {
                ctx.enter("recipient", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Heraldry(elem) => {
                ctx.enter("heraldry", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Num(elem) => {
                ctx.enter("num", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Distributor(elem) => {
                ctx.enter("distributor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::RespStmt(elem) => {
                ctx.enter("respStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::BiblStruct(elem) => {
                ctx.enter("biblStruct", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Imprint(elem) => {
                ctx.enter("imprint", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Catchwords(elem) => {
                ctx.enter("catchwords", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::BiblScope(elem) => {
                ctx.enter("biblScope", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::PubPlace(elem) => {
                ctx.enter("pubPlace", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Stamp(elem) => {
                ctx.enter("stamp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::SecFolio(elem) => {
                ctx.enter("secFolio", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::PostCode(elem) => {
                ctx.enter("postCode", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Width(elem) => {
                ctx.enter("width", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Editor(elem) => {
                ctx.enter("editor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::CorpName(elem) => {
                ctx.enter("corpName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Term(elem) => {
                ctx.enter("term", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Seg(elem) => {
                ctx.enter("seg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Creation(elem) => {
                ctx.enter("creation", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Title(elem) => {
                ctx.enter("title", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Abbr(elem) => {
                ctx.enter("abbr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Annot(elem) => {
                ctx.enter("annot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Rend(elem) => {
                ctx.enter("rend", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::LocusGrp(elem) => {
                ctx.enter("locusGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Country(elem) => {
                ctx.enter("country", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::PostBox(elem) => {
                ctx.enter("postBox", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Dimensions(elem) => {
                ctx.enter("dimensions", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Locus(elem) => {
                ctx.enter("locus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::RelatedItem(elem) => {
                ctx.enter("relatedItem", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BiblChild::Edition(elem) => {
                ctx.enter("edition", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**bibliographic reference - Provides a loosely-structured bibliographic citation in which
the sub-components may or may not be explicitly marked.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "bibl")]
pub struct Bibl {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub pointing: crate::generated::att::AttPointing,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<BiblChild>,
}
impl crate::generated::model::ModelBiblLike for Bibl {}
impl Validate for Bibl {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}
