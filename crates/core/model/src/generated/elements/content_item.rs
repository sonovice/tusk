//!Element: `<contentItem>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<contentItem>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentItemChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
    #[serde(rename = "restore")]
    Restore(Box<crate::generated::elements::Restore>),
    #[serde(rename = "extent")]
    Extent(Box<crate::generated::elements::Extent>),
    #[serde(rename = "choice")]
    Choice(Box<crate::generated::elements::Choice>),
    #[serde(rename = "rend")]
    Rend(Box<crate::generated::elements::Rend>),
    #[serde(rename = "handShift")]
    HandShift(Box<crate::generated::elements::HandShift>),
    #[serde(rename = "secFolio")]
    SecFolio(Box<crate::generated::elements::SecFolio>),
    #[serde(rename = "country")]
    Country(Box<crate::generated::elements::Country>),
    #[serde(rename = "dim")]
    Dim(Box<crate::generated::elements::Dim>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "reg")]
    Reg(Box<crate::generated::elements::Reg>),
    #[serde(rename = "biblStruct")]
    BiblStruct(Box<crate::generated::elements::BiblStruct>),
    #[serde(rename = "postCode")]
    PostCode(Box<crate::generated::elements::PostCode>),
    #[serde(rename = "height")]
    Height(Box<crate::generated::elements::Height>),
    #[serde(rename = "street")]
    Street(Box<crate::generated::elements::Street>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
    #[serde(rename = "locusGrp")]
    LocusGrp(Box<crate::generated::elements::LocusGrp>),
    #[serde(rename = "region")]
    Region(Box<crate::generated::elements::Region>),
    #[serde(rename = "width")]
    Width(Box<crate::generated::elements::Width>),
    #[serde(rename = "name")]
    Name(Box<crate::generated::elements::Name>),
    #[serde(rename = "term")]
    Term(Box<crate::generated::elements::Term>),
    #[serde(rename = "fig")]
    Fig(Box<crate::generated::elements::Fig>),
    #[serde(rename = "heraldry")]
    Heraldry(Box<crate::generated::elements::Heraldry>),
    #[serde(rename = "bloc")]
    Bloc(Box<crate::generated::elements::Bloc>),
    #[serde(rename = "lb")]
    Lb(Box<crate::generated::elements::Lb>),
    #[serde(rename = "gap")]
    Gap(Box<crate::generated::elements::Gap>),
    #[serde(rename = "ref")]
    Ref(Box<crate::generated::elements::Ref>),
    #[serde(rename = "corr")]
    Corr(Box<crate::generated::elements::Corr>),
    #[serde(rename = "repository")]
    Repository(Box<crate::generated::elements::Repository>),
    #[serde(rename = "abbr")]
    Abbr(Box<crate::generated::elements::Abbr>),
    #[serde(rename = "damage")]
    Damage(Box<crate::generated::elements::Damage>),
    #[serde(rename = "relation")]
    Relation(Box<crate::generated::elements::Relation>),
    #[serde(rename = "orig")]
    Orig(Box<crate::generated::elements::Orig>),
    #[serde(rename = "catchwords")]
    Catchwords(Box<crate::generated::elements::Catchwords>),
    #[serde(rename = "expan")]
    Expan(Box<crate::generated::elements::Expan>),
    #[serde(rename = "date")]
    Date(Box<crate::generated::elements::Date>),
    #[serde(rename = "postBox")]
    PostBox(Box<crate::generated::elements::PostBox>),
    #[serde(rename = "supplied")]
    Supplied(Box<crate::generated::elements::Supplied>),
    #[serde(rename = "sic")]
    Sic(Box<crate::generated::elements::Sic>),
    #[serde(rename = "stamp")]
    Stamp(Box<crate::generated::elements::Stamp>),
    #[serde(rename = "locus")]
    Locus(Box<crate::generated::elements::Locus>),
    #[serde(rename = "corpName")]
    CorpName(Box<crate::generated::elements::CorpName>),
    #[serde(rename = "symbol")]
    Symbol(Box<crate::generated::elements::Symbol>),
    #[serde(rename = "pb")]
    Pb(Box<crate::generated::elements::Pb>),
    #[serde(rename = "periodName")]
    PeriodName(Box<crate::generated::elements::PeriodName>),
    #[serde(rename = "persName")]
    PersName(Box<crate::generated::elements::PersName>),
    #[serde(rename = "del")]
    Del(Box<crate::generated::elements::Del>),
    #[serde(rename = "geogName")]
    GeogName(Box<crate::generated::elements::GeogName>),
    #[serde(rename = "seg")]
    Seg(Box<crate::generated::elements::Seg>),
    #[serde(rename = "address")]
    Address(Box<crate::generated::elements::Address>),
    #[serde(rename = "settlement")]
    Settlement(Box<crate::generated::elements::Settlement>),
    #[serde(rename = "q")]
    Q(Box<crate::generated::elements::Q>),
    #[serde(rename = "unclear")]
    Unclear(Box<crate::generated::elements::Unclear>),
    #[serde(rename = "ptr")]
    Ptr(Box<crate::generated::elements::Ptr>),
    #[serde(rename = "depth")]
    Depth(Box<crate::generated::elements::Depth>),
    #[serde(rename = "styleName")]
    StyleName(Box<crate::generated::elements::StyleName>),
    #[serde(rename = "district")]
    District(Box<crate::generated::elements::District>),
    #[serde(rename = "num")]
    Num(Box<crate::generated::elements::Num>),
    #[serde(rename = "subst")]
    Subst(Box<crate::generated::elements::Subst>),
    #[serde(rename = "title")]
    Title(Box<crate::generated::elements::Title>),
    #[serde(rename = "dimensions")]
    Dimensions(Box<crate::generated::elements::Dimensions>),
    #[serde(rename = "geogFeat")]
    GeogFeat(Box<crate::generated::elements::GeogFeat>),
    #[serde(rename = "relationList")]
    RelationList(Box<crate::generated::elements::RelationList>),
    #[serde(rename = "signatures")]
    Signatures(Box<crate::generated::elements::Signatures>),
    #[serde(rename = "stack")]
    Stack(Box<crate::generated::elements::Stack>),
    #[serde(rename = "bibl")]
    Bibl(Box<crate::generated::elements::Bibl>),
    #[serde(rename = "add")]
    Add(Box<crate::generated::elements::Add>),
}
impl ContentItemChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            ContentItemChild::Text(_) => {}
            ContentItemChild::Restore(elem) => {
                ctx.enter("restore", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Extent(elem) => {
                ctx.enter("extent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Choice(elem) => {
                ctx.enter("choice", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Rend(elem) => {
                ctx.enter("rend", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::HandShift(elem) => {
                ctx.enter("handShift", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::SecFolio(elem) => {
                ctx.enter("secFolio", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Country(elem) => {
                ctx.enter("country", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Dim(elem) => {
                ctx.enter("dim", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Reg(elem) => {
                ctx.enter("reg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::BiblStruct(elem) => {
                ctx.enter("biblStruct", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::PostCode(elem) => {
                ctx.enter("postCode", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Height(elem) => {
                ctx.enter("height", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Street(elem) => {
                ctx.enter("street", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Annot(elem) => {
                ctx.enter("annot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::LocusGrp(elem) => {
                ctx.enter("locusGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Region(elem) => {
                ctx.enter("region", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Width(elem) => {
                ctx.enter("width", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Name(elem) => {
                ctx.enter("name", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Term(elem) => {
                ctx.enter("term", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Fig(elem) => {
                ctx.enter("fig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Heraldry(elem) => {
                ctx.enter("heraldry", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Bloc(elem) => {
                ctx.enter("bloc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Lb(elem) => {
                ctx.enter("lb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Gap(elem) => {
                ctx.enter("gap", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Ref(elem) => {
                ctx.enter("ref", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Corr(elem) => {
                ctx.enter("corr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Repository(elem) => {
                ctx.enter("repository", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Abbr(elem) => {
                ctx.enter("abbr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Damage(elem) => {
                ctx.enter("damage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Relation(elem) => {
                ctx.enter("relation", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Orig(elem) => {
                ctx.enter("orig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Catchwords(elem) => {
                ctx.enter("catchwords", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Expan(elem) => {
                ctx.enter("expan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Date(elem) => {
                ctx.enter("date", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::PostBox(elem) => {
                ctx.enter("postBox", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Supplied(elem) => {
                ctx.enter("supplied", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Sic(elem) => {
                ctx.enter("sic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Stamp(elem) => {
                ctx.enter("stamp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Locus(elem) => {
                ctx.enter("locus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::CorpName(elem) => {
                ctx.enter("corpName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Symbol(elem) => {
                ctx.enter("symbol", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Pb(elem) => {
                ctx.enter("pb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::PeriodName(elem) => {
                ctx.enter("periodName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::PersName(elem) => {
                ctx.enter("persName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Del(elem) => {
                ctx.enter("del", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::GeogName(elem) => {
                ctx.enter("geogName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Seg(elem) => {
                ctx.enter("seg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Address(elem) => {
                ctx.enter("address", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Settlement(elem) => {
                ctx.enter("settlement", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Q(elem) => {
                ctx.enter("q", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Unclear(elem) => {
                ctx.enter("unclear", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Ptr(elem) => {
                ctx.enter("ptr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Depth(elem) => {
                ctx.enter("depth", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::StyleName(elem) => {
                ctx.enter("styleName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::District(elem) => {
                ctx.enter("district", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Num(elem) => {
                ctx.enter("num", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Subst(elem) => {
                ctx.enter("subst", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Title(elem) => {
                ctx.enter("title", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Dimensions(elem) => {
                ctx.enter("dimensions", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::GeogFeat(elem) => {
                ctx.enter("geogFeat", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::RelationList(elem) => {
                ctx.enter("relationList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Signatures(elem) => {
                ctx.enter("signatures", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Stack(elem) => {
                ctx.enter("stack", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Bibl(elem) => {
                ctx.enter("bibl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ContentItemChild::Add(elem) => {
                ctx.enter("add", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Contains a single entry within a content description element.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "contentItem")]
pub struct ContentItem {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<ContentItemChild>,
}
impl Validate for ContentItem {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}
