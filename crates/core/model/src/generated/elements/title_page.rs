//!Element: `<titlePage>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<titlePage>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TitlePageChild {
    #[serde(rename = "creator")]
    Creator(Box<crate::generated::elements::Creator>),
    #[serde(rename = "cb")]
    Cb(Box<crate::generated::elements::Cb>),
    #[serde(rename = "availability")]
    Availability(Box<crate::generated::elements::Availability>),
    #[serde(rename = "table")]
    Table(Box<crate::generated::elements::Table>),
    #[serde(rename = "eventList")]
    EventList(Box<crate::generated::elements::EventList>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "editor")]
    Editor(Box<crate::generated::elements::Editor>),
    #[serde(rename = "byline")]
    Byline(Box<crate::generated::elements::Byline>),
    #[serde(rename = "imprint")]
    Imprint(Box<crate::generated::elements::Imprint>),
    #[serde(rename = "series")]
    Series(Box<crate::generated::elements::Series>),
    #[serde(rename = "titlePart")]
    TitlePart(Box<crate::generated::elements::TitlePart>),
    #[serde(rename = "add")]
    Add(Box<crate::generated::elements::Add>),
    #[serde(rename = "reg")]
    Reg(Box<crate::generated::elements::Reg>),
    #[serde(rename = "restore")]
    Restore(Box<crate::generated::elements::Restore>),
    #[serde(rename = "perfMedium")]
    PerfMedium(Box<crate::generated::elements::PerfMedium>),
    #[serde(rename = "pb")]
    Pb(Box<crate::generated::elements::Pb>),
    #[serde(rename = "lg")]
    Lg(Box<crate::generated::elements::Lg>),
    #[serde(rename = "distributor")]
    Distributor(Box<crate::generated::elements::Distributor>),
    #[serde(rename = "unpub")]
    Unpub(Box<crate::generated::elements::Unpub>),
    #[serde(rename = "accessRestrict")]
    AccessRestrict(Box<crate::generated::elements::AccessRestrict>),
    #[serde(rename = "price")]
    Price(Box<crate::generated::elements::Price>),
    #[serde(rename = "sponsor")]
    Sponsor(Box<crate::generated::elements::Sponsor>),
    #[serde(rename = "p")]
    P(Box<crate::generated::elements::P>),
    #[serde(rename = "argument")]
    Argument(Box<crate::generated::elements::Argument>),
    #[serde(rename = "sysReq")]
    SysReq(Box<crate::generated::elements::SysReq>),
    #[serde(rename = "useRestrict")]
    UseRestrict(Box<crate::generated::elements::UseRestrict>),
    #[serde(rename = "damage")]
    Damage(Box<crate::generated::elements::Damage>),
    #[serde(rename = "castList")]
    CastList(Box<crate::generated::elements::CastList>),
    #[serde(rename = "edition")]
    Edition(Box<crate::generated::elements::Edition>),
    #[serde(rename = "orig")]
    Orig(Box<crate::generated::elements::Orig>),
    #[serde(rename = "perfDuration")]
    PerfDuration(Box<crate::generated::elements::PerfDuration>),
    #[serde(rename = "supplied")]
    Supplied(Box<crate::generated::elements::Supplied>),
    #[serde(rename = "colLayout")]
    ColLayout(Box<crate::generated::elements::ColLayout>),
    #[serde(rename = "quote")]
    Quote(Box<crate::generated::elements::Quote>),
    #[serde(rename = "sic")]
    Sic(Box<crate::generated::elements::Sic>),
    #[serde(rename = "unclear")]
    Unclear(Box<crate::generated::elements::Unclear>),
    #[serde(rename = "plateNum")]
    PlateNum(Box<crate::generated::elements::PlateNum>),
    #[serde(rename = "fig")]
    Fig(Box<crate::generated::elements::Fig>),
    #[serde(rename = "epigraph")]
    Epigraph(Box<crate::generated::elements::Epigraph>),
    #[serde(rename = "handShift")]
    HandShift(Box<crate::generated::elements::HandShift>),
    #[serde(rename = "corr")]
    Corr(Box<crate::generated::elements::Corr>),
    #[serde(rename = "del")]
    Del(Box<crate::generated::elements::Del>),
    #[serde(rename = "contributor")]
    Contributor(Box<crate::generated::elements::Contributor>),
    #[serde(rename = "gap")]
    Gap(Box<crate::generated::elements::Gap>),
    #[serde(rename = "publisher")]
    Publisher(Box<crate::generated::elements::Publisher>),
    #[serde(rename = "title")]
    Title(Box<crate::generated::elements::Title>),
    #[serde(rename = "pubPlace")]
    PubPlace(Box<crate::generated::elements::PubPlace>),
    #[serde(rename = "lb")]
    Lb(Box<crate::generated::elements::Lb>),
    #[serde(rename = "imprimatur")]
    Imprimatur(Box<crate::generated::elements::Imprimatur>),
    #[serde(rename = "contents")]
    Contents(Box<crate::generated::elements::Contents>),
    #[serde(rename = "funder")]
    Funder(Box<crate::generated::elements::Funder>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "biblList")]
    BiblList(Box<crate::generated::elements::BiblList>),
    #[serde(rename = "dedication")]
    Dedication(Box<crate::generated::elements::Dedication>),
    #[serde(rename = "date")]
    Date(Box<crate::generated::elements::Date>),
    #[serde(rename = "list")]
    List(Box<crate::generated::elements::List>),
}
impl TitlePageChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            TitlePageChild::Creator(elem) => {
                ctx.enter("creator", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Cb(elem) => {
                ctx.enter("cb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Availability(elem) => {
                ctx.enter("availability", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Table(elem) => {
                ctx.enter("table", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::EventList(elem) => {
                ctx.enter("eventList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Editor(elem) => {
                ctx.enter("editor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Byline(elem) => {
                ctx.enter("byline", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Imprint(elem) => {
                ctx.enter("imprint", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Series(elem) => {
                ctx.enter("series", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::TitlePart(elem) => {
                ctx.enter("titlePart", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Add(elem) => {
                ctx.enter("add", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Reg(elem) => {
                ctx.enter("reg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Restore(elem) => {
                ctx.enter("restore", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::PerfMedium(elem) => {
                ctx.enter("perfMedium", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Pb(elem) => {
                ctx.enter("pb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Lg(elem) => {
                ctx.enter("lg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Distributor(elem) => {
                ctx.enter("distributor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Unpub(elem) => {
                ctx.enter("unpub", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::AccessRestrict(elem) => {
                ctx.enter("accessRestrict", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Price(elem) => {
                ctx.enter("price", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Sponsor(elem) => {
                ctx.enter("sponsor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::P(elem) => {
                ctx.enter("p", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Argument(elem) => {
                ctx.enter("argument", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::SysReq(elem) => {
                ctx.enter("sysReq", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::UseRestrict(elem) => {
                ctx.enter("useRestrict", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Damage(elem) => {
                ctx.enter("damage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::CastList(elem) => {
                ctx.enter("castList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Edition(elem) => {
                ctx.enter("edition", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Orig(elem) => {
                ctx.enter("orig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::PerfDuration(elem) => {
                ctx.enter("perfDuration", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Supplied(elem) => {
                ctx.enter("supplied", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::ColLayout(elem) => {
                ctx.enter("colLayout", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Quote(elem) => {
                ctx.enter("quote", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Sic(elem) => {
                ctx.enter("sic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Unclear(elem) => {
                ctx.enter("unclear", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::PlateNum(elem) => {
                ctx.enter("plateNum", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Fig(elem) => {
                ctx.enter("fig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Epigraph(elem) => {
                ctx.enter("epigraph", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::HandShift(elem) => {
                ctx.enter("handShift", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Corr(elem) => {
                ctx.enter("corr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Del(elem) => {
                ctx.enter("del", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Contributor(elem) => {
                ctx.enter("contributor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Gap(elem) => {
                ctx.enter("gap", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Publisher(elem) => {
                ctx.enter("publisher", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Title(elem) => {
                ctx.enter("title", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::PubPlace(elem) => {
                ctx.enter("pubPlace", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Lb(elem) => {
                ctx.enter("lb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Imprimatur(elem) => {
                ctx.enter("imprimatur", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Contents(elem) => {
                ctx.enter("contents", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Funder(elem) => {
                ctx.enter("funder", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::BiblList(elem) => {
                ctx.enter("biblList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Dedication(elem) => {
                ctx.enter("dedication", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::Date(elem) => {
                ctx.enter("date", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitlePageChild::List(elem) => {
                ctx.enter("list", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Contains a transcription of the title page of a text.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "titlePage")]
pub struct TitlePage {
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
    pub children: Vec<TitlePageChild>,
}
impl crate::generated::model::ModelFrontAndBackPart for TitlePage {}
impl crate::generated::model::ModelPhysDescPart for TitlePage {}
impl Validate for TitlePage {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}
