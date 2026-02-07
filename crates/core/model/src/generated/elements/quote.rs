//!Element: `<quote>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<quote>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QuoteChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
    #[serde(rename = "biblList")]
    BiblList(Box<crate::generated::elements::BiblList>),
    #[serde(rename = "symbol")]
    Symbol(Box<crate::generated::elements::Symbol>),
    #[serde(rename = "gap")]
    Gap(Box<crate::generated::elements::Gap>),
    #[serde(rename = "term")]
    Term(Box<crate::generated::elements::Term>),
    #[serde(rename = "choice")]
    Choice(Box<crate::generated::elements::Choice>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
    #[serde(rename = "dim")]
    Dim(Box<crate::generated::elements::Dim>),
    #[serde(rename = "depth")]
    Depth(Box<crate::generated::elements::Depth>),
    #[serde(rename = "del")]
    Del(Box<crate::generated::elements::Del>),
    #[serde(rename = "relation")]
    Relation(Box<crate::generated::elements::Relation>),
    #[serde(rename = "dimensions")]
    Dimensions(Box<crate::generated::elements::Dimensions>),
    #[serde(rename = "stageDir")]
    StageDir(Box<crate::generated::elements::StageDir>),
    #[serde(rename = "date")]
    Date(Box<crate::generated::elements::Date>),
    #[serde(rename = "sic")]
    Sic(Box<crate::generated::elements::Sic>),
    #[serde(rename = "expan")]
    Expan(Box<crate::generated::elements::Expan>),
    #[serde(rename = "pb")]
    Pb(Box<crate::generated::elements::Pb>),
    #[serde(rename = "locus")]
    Locus(Box<crate::generated::elements::Locus>),
    #[serde(rename = "rend")]
    Rend(Box<crate::generated::elements::Rend>),
    #[serde(rename = "bibl")]
    Bibl(Box<crate::generated::elements::Bibl>),
    #[serde(rename = "postCode")]
    PostCode(Box<crate::generated::elements::PostCode>),
    #[serde(rename = "damage")]
    Damage(Box<crate::generated::elements::Damage>),
    #[serde(rename = "district")]
    District(Box<crate::generated::elements::District>),
    #[serde(rename = "seg")]
    Seg(Box<crate::generated::elements::Seg>),
    #[serde(rename = "locusGrp")]
    LocusGrp(Box<crate::generated::elements::LocusGrp>),
    #[serde(rename = "height")]
    Height(Box<crate::generated::elements::Height>),
    #[serde(rename = "street")]
    Street(Box<crate::generated::elements::Street>),
    #[serde(rename = "catchwords")]
    Catchwords(Box<crate::generated::elements::Catchwords>),
    #[serde(rename = "periodName")]
    PeriodName(Box<crate::generated::elements::PeriodName>),
    #[serde(rename = "settlement")]
    Settlement(Box<crate::generated::elements::Settlement>),
    #[serde(rename = "corpName")]
    CorpName(Box<crate::generated::elements::CorpName>),
    #[serde(rename = "lb")]
    Lb(Box<crate::generated::elements::Lb>),
    #[serde(rename = "address")]
    Address(Box<crate::generated::elements::Address>),
    #[serde(rename = "stamp")]
    Stamp(Box<crate::generated::elements::Stamp>),
    #[serde(rename = "unclear")]
    Unclear(Box<crate::generated::elements::Unclear>),
    #[serde(rename = "lg")]
    Lg(Box<crate::generated::elements::Lg>),
    #[serde(rename = "handShift")]
    HandShift(Box<crate::generated::elements::HandShift>),
    #[serde(rename = "stack")]
    Stack(Box<crate::generated::elements::Stack>),
    #[serde(rename = "repository")]
    Repository(Box<crate::generated::elements::Repository>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "extent")]
    Extent(Box<crate::generated::elements::Extent>),
    #[serde(rename = "table")]
    Table(Box<crate::generated::elements::Table>),
    #[serde(rename = "orig")]
    Orig(Box<crate::generated::elements::Orig>),
    #[serde(rename = "persName")]
    PersName(Box<crate::generated::elements::PersName>),
    #[serde(rename = "postBox")]
    PostBox(Box<crate::generated::elements::PostBox>),
    #[serde(rename = "name")]
    Name(Box<crate::generated::elements::Name>),
    #[serde(rename = "ptr")]
    Ptr(Box<crate::generated::elements::Ptr>),
    #[serde(rename = "title")]
    Title(Box<crate::generated::elements::Title>),
    #[serde(rename = "region")]
    Region(Box<crate::generated::elements::Region>),
    #[serde(rename = "sp")]
    Sp(Box<crate::generated::elements::Sp>),
    #[serde(rename = "heraldry")]
    Heraldry(Box<crate::generated::elements::Heraldry>),
    #[serde(rename = "num")]
    Num(Box<crate::generated::elements::Num>),
    #[serde(rename = "corr")]
    Corr(Box<crate::generated::elements::Corr>),
    #[serde(rename = "relationList")]
    RelationList(Box<crate::generated::elements::RelationList>),
    #[serde(rename = "add")]
    Add(Box<crate::generated::elements::Add>),
    #[serde(rename = "subst")]
    Subst(Box<crate::generated::elements::Subst>),
    #[serde(rename = "reg")]
    Reg(Box<crate::generated::elements::Reg>),
    #[serde(rename = "geogName")]
    GeogName(Box<crate::generated::elements::GeogName>),
    #[serde(rename = "abbr")]
    Abbr(Box<crate::generated::elements::Abbr>),
    #[serde(rename = "geogFeat")]
    GeogFeat(Box<crate::generated::elements::GeogFeat>),
    #[serde(rename = "country")]
    Country(Box<crate::generated::elements::Country>),
    #[serde(rename = "ref")]
    Ref(Box<crate::generated::elements::Ref>),
    #[serde(rename = "secFolio")]
    SecFolio(Box<crate::generated::elements::SecFolio>),
    #[serde(rename = "width")]
    Width(Box<crate::generated::elements::Width>),
    #[serde(rename = "list")]
    List(Box<crate::generated::elements::List>),
    #[serde(rename = "p")]
    P(Box<crate::generated::elements::P>),
    #[serde(rename = "signatures")]
    Signatures(Box<crate::generated::elements::Signatures>),
    #[serde(rename = "bloc")]
    Bloc(Box<crate::generated::elements::Bloc>),
    #[serde(rename = "biblStruct")]
    BiblStruct(Box<crate::generated::elements::BiblStruct>),
    #[serde(rename = "fig")]
    Fig(Box<crate::generated::elements::Fig>),
    #[serde(rename = "castList")]
    CastList(Box<crate::generated::elements::CastList>),
    #[serde(rename = "q")]
    Q(Box<crate::generated::elements::Q>),
    #[serde(rename = "styleName")]
    StyleName(Box<crate::generated::elements::StyleName>),
    #[serde(rename = "supplied")]
    Supplied(Box<crate::generated::elements::Supplied>),
    #[serde(rename = "restore")]
    Restore(Box<crate::generated::elements::Restore>),
    #[serde(rename = "quote")]
    Quote(Box<crate::generated::elements::Quote>),
    #[serde(rename = "eventList")]
    EventList(Box<crate::generated::elements::EventList>),
}
impl QuoteChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            QuoteChild::Text(_) => {}
            QuoteChild::BiblList(elem) => {
                ctx.enter("biblList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Symbol(elem) => {
                ctx.enter("symbol", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Gap(elem) => {
                ctx.enter("gap", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Term(elem) => {
                ctx.enter("term", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Choice(elem) => {
                ctx.enter("choice", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Annot(elem) => {
                ctx.enter("annot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Dim(elem) => {
                ctx.enter("dim", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Depth(elem) => {
                ctx.enter("depth", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Del(elem) => {
                ctx.enter("del", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Relation(elem) => {
                ctx.enter("relation", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Dimensions(elem) => {
                ctx.enter("dimensions", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::StageDir(elem) => {
                ctx.enter("stageDir", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Date(elem) => {
                ctx.enter("date", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Sic(elem) => {
                ctx.enter("sic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Expan(elem) => {
                ctx.enter("expan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Pb(elem) => {
                ctx.enter("pb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Locus(elem) => {
                ctx.enter("locus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Rend(elem) => {
                ctx.enter("rend", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Bibl(elem) => {
                ctx.enter("bibl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::PostCode(elem) => {
                ctx.enter("postCode", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Damage(elem) => {
                ctx.enter("damage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::District(elem) => {
                ctx.enter("district", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Seg(elem) => {
                ctx.enter("seg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::LocusGrp(elem) => {
                ctx.enter("locusGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Height(elem) => {
                ctx.enter("height", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Street(elem) => {
                ctx.enter("street", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Catchwords(elem) => {
                ctx.enter("catchwords", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::PeriodName(elem) => {
                ctx.enter("periodName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Settlement(elem) => {
                ctx.enter("settlement", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::CorpName(elem) => {
                ctx.enter("corpName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Lb(elem) => {
                ctx.enter("lb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Address(elem) => {
                ctx.enter("address", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Stamp(elem) => {
                ctx.enter("stamp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Unclear(elem) => {
                ctx.enter("unclear", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Lg(elem) => {
                ctx.enter("lg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::HandShift(elem) => {
                ctx.enter("handShift", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Stack(elem) => {
                ctx.enter("stack", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Repository(elem) => {
                ctx.enter("repository", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Extent(elem) => {
                ctx.enter("extent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Table(elem) => {
                ctx.enter("table", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Orig(elem) => {
                ctx.enter("orig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::PersName(elem) => {
                ctx.enter("persName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::PostBox(elem) => {
                ctx.enter("postBox", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Name(elem) => {
                ctx.enter("name", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Ptr(elem) => {
                ctx.enter("ptr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Title(elem) => {
                ctx.enter("title", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Region(elem) => {
                ctx.enter("region", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Sp(elem) => {
                ctx.enter("sp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Heraldry(elem) => {
                ctx.enter("heraldry", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Num(elem) => {
                ctx.enter("num", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Corr(elem) => {
                ctx.enter("corr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::RelationList(elem) => {
                ctx.enter("relationList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Add(elem) => {
                ctx.enter("add", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Subst(elem) => {
                ctx.enter("subst", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Reg(elem) => {
                ctx.enter("reg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::GeogName(elem) => {
                ctx.enter("geogName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Abbr(elem) => {
                ctx.enter("abbr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::GeogFeat(elem) => {
                ctx.enter("geogFeat", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Country(elem) => {
                ctx.enter("country", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Ref(elem) => {
                ctx.enter("ref", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::SecFolio(elem) => {
                ctx.enter("secFolio", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Width(elem) => {
                ctx.enter("width", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::List(elem) => {
                ctx.enter("list", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::P(elem) => {
                ctx.enter("p", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Signatures(elem) => {
                ctx.enter("signatures", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Bloc(elem) => {
                ctx.enter("bloc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::BiblStruct(elem) => {
                ctx.enter("biblStruct", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Fig(elem) => {
                ctx.enter("fig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::CastList(elem) => {
                ctx.enter("castList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Q(elem) => {
                ctx.enter("q", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::StyleName(elem) => {
                ctx.enter("styleName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Supplied(elem) => {
                ctx.enter("supplied", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Restore(elem) => {
                ctx.enter("restore", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::Quote(elem) => {
                ctx.enter("quote", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            QuoteChild::EventList(elem) => {
                ctx.enter("eventList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**quoted material - Contains a paragraph-like block of text attributed to an external
source, normally set off from the surrounding text by spacing or other typographic
distinction.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "quote")]
pub struct Quote {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub text_rendition: crate::generated::att::AttTextRendition,
    #[serde(flatten)]
    pub xy: crate::generated::att::AttXy,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<QuoteChild>,
}
impl crate::generated::model::ModelQuoteLike for Quote {}
impl Validate for Quote {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}
