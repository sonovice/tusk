//!Element: `<accMat>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<accMat>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccMatChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
    #[serde(rename = "catchwords")]
    Catchwords(Box<crate::generated::elements::Catchwords>),
    #[serde(rename = "num")]
    Num(Box<crate::generated::elements::Num>),
    #[serde(rename = "bibl")]
    Bibl(Box<crate::generated::elements::Bibl>),
    #[serde(rename = "fig")]
    Fig(Box<crate::generated::elements::Fig>),
    #[serde(rename = "seg")]
    Seg(Box<crate::generated::elements::Seg>),
    #[serde(rename = "signatures")]
    Signatures(Box<crate::generated::elements::Signatures>),
    #[serde(rename = "bloc")]
    Bloc(Box<crate::generated::elements::Bloc>),
    #[serde(rename = "name")]
    Name(Box<crate::generated::elements::Name>),
    #[serde(rename = "periodName")]
    PeriodName(Box<crate::generated::elements::PeriodName>),
    #[serde(rename = "extent")]
    Extent(Box<crate::generated::elements::Extent>),
    #[serde(rename = "dim")]
    Dim(Box<crate::generated::elements::Dim>),
    #[serde(rename = "date")]
    Date(Box<crate::generated::elements::Date>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
    #[serde(rename = "locus")]
    Locus(Box<crate::generated::elements::Locus>),
    #[serde(rename = "relation")]
    Relation(Box<crate::generated::elements::Relation>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "country")]
    Country(Box<crate::generated::elements::Country>),
    #[serde(rename = "postCode")]
    PostCode(Box<crate::generated::elements::PostCode>),
    #[serde(rename = "depth")]
    Depth(Box<crate::generated::elements::Depth>),
    #[serde(rename = "district")]
    District(Box<crate::generated::elements::District>),
    #[serde(rename = "region")]
    Region(Box<crate::generated::elements::Region>),
    #[serde(rename = "geogName")]
    GeogName(Box<crate::generated::elements::GeogName>),
    #[serde(rename = "stack")]
    Stack(Box<crate::generated::elements::Stack>),
    #[serde(rename = "title")]
    Title(Box<crate::generated::elements::Title>),
    #[serde(rename = "lb")]
    Lb(Box<crate::generated::elements::Lb>),
    #[serde(rename = "ref")]
    Ref(Box<crate::generated::elements::Ref>),
    #[serde(rename = "p")]
    P(Box<crate::generated::elements::P>),
    #[serde(rename = "postBox")]
    PostBox(Box<crate::generated::elements::PostBox>),
    #[serde(rename = "repository")]
    Repository(Box<crate::generated::elements::Repository>),
    #[serde(rename = "geogFeat")]
    GeogFeat(Box<crate::generated::elements::GeogFeat>),
    #[serde(rename = "address")]
    Address(Box<crate::generated::elements::Address>),
    #[serde(rename = "biblStruct")]
    BiblStruct(Box<crate::generated::elements::BiblStruct>),
    #[serde(rename = "corpName")]
    CorpName(Box<crate::generated::elements::CorpName>),
    #[serde(rename = "abbr")]
    Abbr(Box<crate::generated::elements::Abbr>),
    #[serde(rename = "persName")]
    PersName(Box<crate::generated::elements::PersName>),
    #[serde(rename = "q")]
    Q(Box<crate::generated::elements::Q>),
    #[serde(rename = "secFolio")]
    SecFolio(Box<crate::generated::elements::SecFolio>),
    #[serde(rename = "settlement")]
    Settlement(Box<crate::generated::elements::Settlement>),
    #[serde(rename = "symbol")]
    Symbol(Box<crate::generated::elements::Symbol>),
    #[serde(rename = "stamp")]
    Stamp(Box<crate::generated::elements::Stamp>),
    #[serde(rename = "width")]
    Width(Box<crate::generated::elements::Width>),
    #[serde(rename = "dimensions")]
    Dimensions(Box<crate::generated::elements::Dimensions>),
    #[serde(rename = "term")]
    Term(Box<crate::generated::elements::Term>),
    #[serde(rename = "heraldry")]
    Heraldry(Box<crate::generated::elements::Heraldry>),
    #[serde(rename = "expan")]
    Expan(Box<crate::generated::elements::Expan>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "rend")]
    Rend(Box<crate::generated::elements::Rend>),
    #[serde(rename = "street")]
    Street(Box<crate::generated::elements::Street>),
    #[serde(rename = "ptr")]
    Ptr(Box<crate::generated::elements::Ptr>),
    #[serde(rename = "relationList")]
    RelationList(Box<crate::generated::elements::RelationList>),
    #[serde(rename = "styleName")]
    StyleName(Box<crate::generated::elements::StyleName>),
    #[serde(rename = "locusGrp")]
    LocusGrp(Box<crate::generated::elements::LocusGrp>),
    #[serde(rename = "height")]
    Height(Box<crate::generated::elements::Height>),
}
impl AccMatChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            AccMatChild::Text(_) => {}
            AccMatChild::Catchwords(elem) => {
                ctx.enter("catchwords", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Num(elem) => {
                ctx.enter("num", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Bibl(elem) => {
                ctx.enter("bibl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Fig(elem) => {
                ctx.enter("fig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Seg(elem) => {
                ctx.enter("seg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Signatures(elem) => {
                ctx.enter("signatures", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Bloc(elem) => {
                ctx.enter("bloc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Name(elem) => {
                ctx.enter("name", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::PeriodName(elem) => {
                ctx.enter("periodName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Extent(elem) => {
                ctx.enter("extent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Dim(elem) => {
                ctx.enter("dim", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Date(elem) => {
                ctx.enter("date", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Annot(elem) => {
                ctx.enter("annot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Locus(elem) => {
                ctx.enter("locus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Relation(elem) => {
                ctx.enter("relation", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Country(elem) => {
                ctx.enter("country", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::PostCode(elem) => {
                ctx.enter("postCode", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Depth(elem) => {
                ctx.enter("depth", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::District(elem) => {
                ctx.enter("district", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Region(elem) => {
                ctx.enter("region", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::GeogName(elem) => {
                ctx.enter("geogName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Stack(elem) => {
                ctx.enter("stack", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Title(elem) => {
                ctx.enter("title", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Lb(elem) => {
                ctx.enter("lb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Ref(elem) => {
                ctx.enter("ref", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::P(elem) => {
                ctx.enter("p", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::PostBox(elem) => {
                ctx.enter("postBox", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Repository(elem) => {
                ctx.enter("repository", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::GeogFeat(elem) => {
                ctx.enter("geogFeat", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Address(elem) => {
                ctx.enter("address", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::BiblStruct(elem) => {
                ctx.enter("biblStruct", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::CorpName(elem) => {
                ctx.enter("corpName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Abbr(elem) => {
                ctx.enter("abbr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::PersName(elem) => {
                ctx.enter("persName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Q(elem) => {
                ctx.enter("q", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::SecFolio(elem) => {
                ctx.enter("secFolio", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Settlement(elem) => {
                ctx.enter("settlement", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Symbol(elem) => {
                ctx.enter("symbol", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Stamp(elem) => {
                ctx.enter("stamp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Width(elem) => {
                ctx.enter("width", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Dimensions(elem) => {
                ctx.enter("dimensions", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Term(elem) => {
                ctx.enter("term", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Heraldry(elem) => {
                ctx.enter("heraldry", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Expan(elem) => {
                ctx.enter("expan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Rend(elem) => {
                ctx.enter("rend", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Street(elem) => {
                ctx.enter("street", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Ptr(elem) => {
                ctx.enter("ptr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::RelationList(elem) => {
                ctx.enter("relationList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::StyleName(elem) => {
                ctx.enter("styleName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::LocusGrp(elem) => {
                ctx.enter("locusGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AccMatChild::Height(elem) => {
                ctx.enter("height", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Holds a description of any additional material bound with an item, such as
non-contemporaneous documents or fragments.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "accMat")]
pub struct AccMat {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<AccMatChild>,
}
impl crate::generated::model::ModelPhysDescPart for AccMat {}
impl Validate for AccMat {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}
