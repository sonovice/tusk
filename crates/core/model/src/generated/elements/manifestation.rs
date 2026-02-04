//!Element: `<manifestation>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<manifestation>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ManifestationChild {
    #[serde(rename = "physLoc")]
    PhysLoc(Box<crate::generated::elements::PhysLoc>),
    #[serde(rename = "itemList")]
    ItemList(Box<crate::generated::elements::ItemList>),
    #[serde(rename = "componentList")]
    ComponentList(Box<crate::generated::elements::ComponentList>),
    #[serde(rename = "seriesStmt")]
    SeriesStmt(Box<crate::generated::elements::SeriesStmt>),
    #[serde(rename = "contents")]
    Contents(Box<crate::generated::elements::Contents>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "dedication")]
    Dedication(Box<crate::generated::elements::Dedication>),
    #[serde(rename = "locus")]
    Locus(Box<crate::generated::elements::Locus>),
    #[serde(rename = "classification")]
    Classification(Box<crate::generated::elements::Classification>),
    #[serde(rename = "relationList")]
    RelationList(Box<crate::generated::elements::RelationList>),
    #[serde(rename = "pubStmt")]
    PubStmt(Box<crate::generated::elements::PubStmt>),
    #[serde(rename = "notesStmt")]
    NotesStmt(Box<crate::generated::elements::NotesStmt>),
    #[serde(rename = "editionStmt")]
    EditionStmt(Box<crate::generated::elements::EditionStmt>),
    #[serde(rename = "availability")]
    Availability(Box<crate::generated::elements::Availability>),
    #[serde(rename = "locusGrp")]
    LocusGrp(Box<crate::generated::elements::LocusGrp>),
    #[serde(rename = "langUsage")]
    LangUsage(Box<crate::generated::elements::LangUsage>),
    #[serde(rename = "history")]
    History(Box<crate::generated::elements::History>),
    #[serde(rename = "biblList")]
    BiblList(Box<crate::generated::elements::BiblList>),
    #[serde(rename = "extMeta")]
    ExtMeta(Box<crate::generated::elements::ExtMeta>),
    #[serde(rename = "physDesc")]
    PhysDesc(Box<crate::generated::elements::PhysDesc>),
    #[serde(rename = "creation")]
    Creation(Box<crate::generated::elements::Creation>),
    #[serde(rename = "titleStmt")]
    TitleStmt(Box<crate::generated::elements::TitleStmt>),
}
impl ManifestationChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            ManifestationChild::PhysLoc(elem) => {
                ctx.enter("physLoc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::ItemList(elem) => {
                ctx.enter("itemList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::ComponentList(elem) => {
                ctx.enter("componentList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::SeriesStmt(elem) => {
                ctx.enter("seriesStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::Contents(elem) => {
                ctx.enter("contents", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::Dedication(elem) => {
                ctx.enter("dedication", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::Locus(elem) => {
                ctx.enter("locus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::Classification(elem) => {
                ctx.enter("classification", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::RelationList(elem) => {
                ctx.enter("relationList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::PubStmt(elem) => {
                ctx.enter("pubStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::NotesStmt(elem) => {
                ctx.enter("notesStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::EditionStmt(elem) => {
                ctx.enter("editionStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::Availability(elem) => {
                ctx.enter("availability", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::LocusGrp(elem) => {
                ctx.enter("locusGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::LangUsage(elem) => {
                ctx.enter("langUsage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::History(elem) => {
                ctx.enter("history", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::BiblList(elem) => {
                ctx.enter("biblList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::ExtMeta(elem) => {
                ctx.enter("extMeta", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::PhysDesc(elem) => {
                ctx.enter("physDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::Creation(elem) => {
                ctx.enter("creation", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationChild::TitleStmt(elem) => {
                ctx.enter("titleStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///A bibliographic description of a physical embodiment of an expression of a work.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "manifestation")]
pub struct Manifestation {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub authorized: crate::generated::att::AttAuthorized,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub component_type: crate::generated::att::AttComponentType,
    #[serde(flatten)]
    pub data_pointing: crate::generated::att::AttDataPointing,
    #[serde(flatten)]
    pub pointing: crate::generated::att::AttPointing,
    #[serde(flatten)]
    pub record_type: crate::generated::att::AttRecordType,
    #[serde(flatten)]
    pub target_eval: crate::generated::att::AttTargetEval,
    ///Indicates the manifestation is a unique physical object.
    #[serde(rename = "@singleton", skip_serializing_if = "Option::is_none")]
    pub singleton: Option<crate::generated::data::DataBoolean>,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<ManifestationChild>,
}
impl crate::generated::model::ModelManifestationLike for Manifestation {}
impl Validate for Manifestation {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}
