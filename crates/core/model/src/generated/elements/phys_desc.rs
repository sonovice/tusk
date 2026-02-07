//!Element: `<physDesc>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<physDesc>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PhysDescChild {
    #[serde(rename = "physMedium")]
    PhysMedium(Box<crate::generated::elements::PhysMedium>),
    #[serde(rename = "secFolio")]
    SecFolio(Box<crate::generated::elements::SecFolio>),
    #[serde(rename = "captureMode")]
    CaptureMode(Box<crate::generated::elements::CaptureMode>),
    #[serde(rename = "decoDesc")]
    DecoDesc(Box<crate::generated::elements::DecoDesc>),
    #[serde(rename = "condition")]
    Condition(Box<crate::generated::elements::Condition>),
    #[serde(rename = "rubric")]
    Rubric(Box<crate::generated::elements::Rubric>),
    #[serde(rename = "carrierForm")]
    CarrierForm(Box<crate::generated::elements::CarrierForm>),
    #[serde(rename = "colophon")]
    Colophon(Box<crate::generated::elements::Colophon>),
    #[serde(rename = "scoreFormat")]
    ScoreFormat(Box<crate::generated::elements::ScoreFormat>),
    #[serde(rename = "typeDesc")]
    TypeDesc(Box<crate::generated::elements::TypeDesc>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "history")]
    History(Box<crate::generated::elements::History>),
    #[serde(rename = "explicit")]
    Explicit(Box<crate::generated::elements::Explicit>),
    #[serde(rename = "sealDesc")]
    SealDesc(Box<crate::generated::elements::SealDesc>),
    #[serde(rename = "catchwords")]
    Catchwords(Box<crate::generated::elements::Catchwords>),
    #[serde(rename = "playingSpeed")]
    PlayingSpeed(Box<crate::generated::elements::PlayingSpeed>),
    #[serde(rename = "p")]
    P(Box<crate::generated::elements::P>),
    #[serde(rename = "signatures")]
    Signatures(Box<crate::generated::elements::Signatures>),
    #[serde(rename = "trackConfig")]
    TrackConfig(Box<crate::generated::elements::TrackConfig>),
    #[serde(rename = "addDesc")]
    AddDesc(Box<crate::generated::elements::AddDesc>),
    #[serde(rename = "fileChar")]
    FileChar(Box<crate::generated::elements::FileChar>),
    #[serde(rename = "incip")]
    Incip(Box<crate::generated::elements::Incip>),
    #[serde(rename = "perfDuration")]
    PerfDuration(Box<crate::generated::elements::PerfDuration>),
    #[serde(rename = "plateNum")]
    PlateNum(Box<crate::generated::elements::PlateNum>),
    #[serde(rename = "handList")]
    HandList(Box<crate::generated::elements::HandList>),
    #[serde(rename = "soundChan")]
    SoundChan(Box<crate::generated::elements::SoundChan>),
    #[serde(rename = "supportDesc")]
    SupportDesc(Box<crate::generated::elements::SupportDesc>),
    #[serde(rename = "inscription")]
    Inscription(Box<crate::generated::elements::Inscription>),
    #[serde(rename = "extent")]
    Extent(Box<crate::generated::elements::Extent>),
    #[serde(rename = "layoutDesc")]
    LayoutDesc(Box<crate::generated::elements::LayoutDesc>),
    #[serde(rename = "specRepro")]
    SpecRepro(Box<crate::generated::elements::SpecRepro>),
    #[serde(rename = "scriptDesc")]
    ScriptDesc(Box<crate::generated::elements::ScriptDesc>),
    #[serde(rename = "titlePage")]
    TitlePage(Box<crate::generated::elements::TitlePage>),
    #[serde(rename = "watermarkDesc")]
    WatermarkDesc(Box<crate::generated::elements::WatermarkDesc>),
    #[serde(rename = "stamp")]
    Stamp(Box<crate::generated::elements::Stamp>),
    #[serde(rename = "dimensions")]
    Dimensions(Box<crate::generated::elements::Dimensions>),
    #[serde(rename = "heraldry")]
    Heraldry(Box<crate::generated::elements::Heraldry>),
    #[serde(rename = "bindingDesc")]
    BindingDesc(Box<crate::generated::elements::BindingDesc>),
    #[serde(rename = "foliaDesc")]
    FoliaDesc(Box<crate::generated::elements::FoliaDesc>),
    #[serde(rename = "accMat")]
    AccMat(Box<crate::generated::elements::AccMat>),
}
impl PhysDescChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            PhysDescChild::PhysMedium(elem) => {
                ctx.enter("physMedium", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::SecFolio(elem) => {
                ctx.enter("secFolio", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::CaptureMode(elem) => {
                ctx.enter("captureMode", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::DecoDesc(elem) => {
                ctx.enter("decoDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::Condition(elem) => {
                ctx.enter("condition", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::Rubric(elem) => {
                ctx.enter("rubric", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::CarrierForm(elem) => {
                ctx.enter("carrierForm", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::Colophon(elem) => {
                ctx.enter("colophon", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::ScoreFormat(elem) => {
                ctx.enter("scoreFormat", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::TypeDesc(elem) => {
                ctx.enter("typeDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::History(elem) => {
                ctx.enter("history", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::Explicit(elem) => {
                ctx.enter("explicit", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::SealDesc(elem) => {
                ctx.enter("sealDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::Catchwords(elem) => {
                ctx.enter("catchwords", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::PlayingSpeed(elem) => {
                ctx.enter("playingSpeed", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::P(elem) => {
                ctx.enter("p", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::Signatures(elem) => {
                ctx.enter("signatures", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::TrackConfig(elem) => {
                ctx.enter("trackConfig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::AddDesc(elem) => {
                ctx.enter("addDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::FileChar(elem) => {
                ctx.enter("fileChar", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::Incip(elem) => {
                ctx.enter("incip", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::PerfDuration(elem) => {
                ctx.enter("perfDuration", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::PlateNum(elem) => {
                ctx.enter("plateNum", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::HandList(elem) => {
                ctx.enter("handList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::SoundChan(elem) => {
                ctx.enter("soundChan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::SupportDesc(elem) => {
                ctx.enter("supportDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::Inscription(elem) => {
                ctx.enter("inscription", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::Extent(elem) => {
                ctx.enter("extent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::LayoutDesc(elem) => {
                ctx.enter("layoutDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::SpecRepro(elem) => {
                ctx.enter("specRepro", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::ScriptDesc(elem) => {
                ctx.enter("scriptDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::TitlePage(elem) => {
                ctx.enter("titlePage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::WatermarkDesc(elem) => {
                ctx.enter("watermarkDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::Stamp(elem) => {
                ctx.enter("stamp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::Dimensions(elem) => {
                ctx.enter("dimensions", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::Heraldry(elem) => {
                ctx.enter("heraldry", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::BindingDesc(elem) => {
                ctx.enter("bindingDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::FoliaDesc(elem) => {
                ctx.enter("foliaDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysDescChild::AccMat(elem) => {
                ctx.enter("accMat", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**physical description - Container for information about the appearance, construction, or
      handling of physical materials, such as their dimension, quantity, color, style, and technique
      of creation.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "physDesc")]
pub struct PhysDesc {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<PhysDescChild>,
}
impl Validate for PhysDesc {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}
