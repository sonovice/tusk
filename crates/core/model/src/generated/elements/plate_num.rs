//!Element: `<plateNum>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<plateNum>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlateNumChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
    #[serde(rename = "stamp")]
    Stamp(Box<crate::generated::elements::Stamp>),
    #[serde(rename = "signatures")]
    Signatures(Box<crate::generated::elements::Signatures>),
    #[serde(rename = "street")]
    Street(Box<crate::generated::elements::Street>),
    #[serde(rename = "date")]
    Date(Box<crate::generated::elements::Date>),
    #[serde(rename = "num")]
    Num(Box<crate::generated::elements::Num>),
    #[serde(rename = "ptr")]
    Ptr(Box<crate::generated::elements::Ptr>),
    #[serde(rename = "biblStruct")]
    BiblStruct(Box<crate::generated::elements::BiblStruct>),
    #[serde(rename = "extent")]
    Extent(Box<crate::generated::elements::Extent>),
    #[serde(rename = "lb")]
    Lb(Box<crate::generated::elements::Lb>),
    #[serde(rename = "symbol")]
    Symbol(Box<crate::generated::elements::Symbol>),
    #[serde(rename = "term")]
    Term(Box<crate::generated::elements::Term>),
    #[serde(rename = "bloc")]
    Bloc(Box<crate::generated::elements::Bloc>),
    #[serde(rename = "locusGrp")]
    LocusGrp(Box<crate::generated::elements::LocusGrp>),
    #[serde(rename = "relation")]
    Relation(Box<crate::generated::elements::Relation>),
    #[serde(rename = "catchwords")]
    Catchwords(Box<crate::generated::elements::Catchwords>),
    #[serde(rename = "district")]
    District(Box<crate::generated::elements::District>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "country")]
    Country(Box<crate::generated::elements::Country>),
    #[serde(rename = "seg")]
    Seg(Box<crate::generated::elements::Seg>),
    #[serde(rename = "styleName")]
    StyleName(Box<crate::generated::elements::StyleName>),
    #[serde(rename = "expan")]
    Expan(Box<crate::generated::elements::Expan>),
    #[serde(rename = "rend")]
    Rend(Box<crate::generated::elements::Rend>),
    #[serde(rename = "title")]
    Title(Box<crate::generated::elements::Title>),
    #[serde(rename = "periodName")]
    PeriodName(Box<crate::generated::elements::PeriodName>),
    #[serde(rename = "address")]
    Address(Box<crate::generated::elements::Address>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "depth")]
    Depth(Box<crate::generated::elements::Depth>),
    #[serde(rename = "postBox")]
    PostBox(Box<crate::generated::elements::PostBox>),
    #[serde(rename = "postCode")]
    PostCode(Box<crate::generated::elements::PostCode>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
    #[serde(rename = "q")]
    Q(Box<crate::generated::elements::Q>),
    #[serde(rename = "settlement")]
    Settlement(Box<crate::generated::elements::Settlement>),
    #[serde(rename = "corpName")]
    CorpName(Box<crate::generated::elements::CorpName>),
    #[serde(rename = "width")]
    Width(Box<crate::generated::elements::Width>),
    #[serde(rename = "geogFeat")]
    GeogFeat(Box<crate::generated::elements::GeogFeat>),
    #[serde(rename = "bibl")]
    Bibl(Box<crate::generated::elements::Bibl>),
    #[serde(rename = "geogName")]
    GeogName(Box<crate::generated::elements::GeogName>),
    #[serde(rename = "height")]
    Height(Box<crate::generated::elements::Height>),
    #[serde(rename = "heraldry")]
    Heraldry(Box<crate::generated::elements::Heraldry>),
    #[serde(rename = "name")]
    Name(Box<crate::generated::elements::Name>),
    #[serde(rename = "ref")]
    Ref(Box<crate::generated::elements::Ref>),
    #[serde(rename = "relationList")]
    RelationList(Box<crate::generated::elements::RelationList>),
    #[serde(rename = "dim")]
    Dim(Box<crate::generated::elements::Dim>),
    #[serde(rename = "locus")]
    Locus(Box<crate::generated::elements::Locus>),
    #[serde(rename = "region")]
    Region(Box<crate::generated::elements::Region>),
    #[serde(rename = "abbr")]
    Abbr(Box<crate::generated::elements::Abbr>),
    #[serde(rename = "p")]
    P(Box<crate::generated::elements::P>),
    #[serde(rename = "repository")]
    Repository(Box<crate::generated::elements::Repository>),
    #[serde(rename = "stack")]
    Stack(Box<crate::generated::elements::Stack>),
    #[serde(rename = "fig")]
    Fig(Box<crate::generated::elements::Fig>),
    #[serde(rename = "persName")]
    PersName(Box<crate::generated::elements::PersName>),
    #[serde(rename = "dimensions")]
    Dimensions(Box<crate::generated::elements::Dimensions>),
    #[serde(rename = "secFolio")]
    SecFolio(Box<crate::generated::elements::SecFolio>),
}
impl PlateNumChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            PlateNumChild::Text(_) => {}
            PlateNumChild::Stamp(elem) => {
                ctx.enter("stamp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Signatures(elem) => {
                ctx.enter("signatures", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Street(elem) => {
                ctx.enter("street", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Date(elem) => {
                ctx.enter("date", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Num(elem) => {
                ctx.enter("num", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Ptr(elem) => {
                ctx.enter("ptr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::BiblStruct(elem) => {
                ctx.enter("biblStruct", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Extent(elem) => {
                ctx.enter("extent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Lb(elem) => {
                ctx.enter("lb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Symbol(elem) => {
                ctx.enter("symbol", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Term(elem) => {
                ctx.enter("term", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Bloc(elem) => {
                ctx.enter("bloc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::LocusGrp(elem) => {
                ctx.enter("locusGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Relation(elem) => {
                ctx.enter("relation", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Catchwords(elem) => {
                ctx.enter("catchwords", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::District(elem) => {
                ctx.enter("district", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Country(elem) => {
                ctx.enter("country", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Seg(elem) => {
                ctx.enter("seg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::StyleName(elem) => {
                ctx.enter("styleName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Expan(elem) => {
                ctx.enter("expan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Rend(elem) => {
                ctx.enter("rend", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Title(elem) => {
                ctx.enter("title", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::PeriodName(elem) => {
                ctx.enter("periodName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Address(elem) => {
                ctx.enter("address", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Depth(elem) => {
                ctx.enter("depth", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::PostBox(elem) => {
                ctx.enter("postBox", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::PostCode(elem) => {
                ctx.enter("postCode", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Annot(elem) => {
                ctx.enter("annot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Q(elem) => {
                ctx.enter("q", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Settlement(elem) => {
                ctx.enter("settlement", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::CorpName(elem) => {
                ctx.enter("corpName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Width(elem) => {
                ctx.enter("width", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::GeogFeat(elem) => {
                ctx.enter("geogFeat", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Bibl(elem) => {
                ctx.enter("bibl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::GeogName(elem) => {
                ctx.enter("geogName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Height(elem) => {
                ctx.enter("height", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Heraldry(elem) => {
                ctx.enter("heraldry", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Name(elem) => {
                ctx.enter("name", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Ref(elem) => {
                ctx.enter("ref", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::RelationList(elem) => {
                ctx.enter("relationList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Dim(elem) => {
                ctx.enter("dim", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Locus(elem) => {
                ctx.enter("locus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Region(elem) => {
                ctx.enter("region", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Abbr(elem) => {
                ctx.enter("abbr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::P(elem) => {
                ctx.enter("p", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Repository(elem) => {
                ctx.enter("repository", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Stack(elem) => {
                ctx.enter("stack", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Fig(elem) => {
                ctx.enter("fig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::PersName(elem) => {
                ctx.enter("persName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::Dimensions(elem) => {
                ctx.enter("dimensions", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PlateNumChild::SecFolio(elem) => {
                ctx.enter("secFolio", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**plate number - Designation assigned to a resource by a music publisher, usually printed
at the bottom of each page, and sometimes appearing also on the title page.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "plateNum")]
pub struct PlateNum {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<PlateNumChild>,
}
impl crate::generated::model::ModelPhysDescPart for PlateNum {}
impl crate::generated::model::ModelTitlePagePart for PlateNum {}
impl Validate for PlateNum {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}
