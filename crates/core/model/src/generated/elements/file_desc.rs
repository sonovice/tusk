//!Element: `<fileDesc>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<fileDesc>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileDescChild {
    #[serde(rename = "seriesStmt")]
    SeriesStmt(Box<crate::generated::elements::SeriesStmt>),
    #[serde(rename = "sourceDesc")]
    SourceDesc(Box<crate::generated::elements::SourceDesc>),
    #[serde(rename = "editionStmt")]
    EditionStmt(Box<crate::generated::elements::EditionStmt>),
    #[serde(rename = "notesStmt")]
    NotesStmt(Box<crate::generated::elements::NotesStmt>),
    #[serde(rename = "extent")]
    Extent(Box<crate::generated::elements::Extent>),
    #[serde(rename = "pubStmt")]
    PubStmt(Box<crate::generated::elements::PubStmt>),
    #[serde(rename = "titleStmt")]
    TitleStmt(Box<crate::generated::elements::TitleStmt>),
}
impl FileDescChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            FileDescChild::SeriesStmt(elem) => {
                ctx.enter("seriesStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FileDescChild::SourceDesc(elem) => {
                ctx.enter("sourceDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FileDescChild::EditionStmt(elem) => {
                ctx.enter("editionStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FileDescChild::NotesStmt(elem) => {
                ctx.enter("notesStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FileDescChild::Extent(elem) => {
                ctx.enter("extent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FileDescChild::PubStmt(elem) => {
                ctx.enter("pubStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FileDescChild::TitleStmt(elem) => {
                ctx.enter("titleStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///file description - Contains a full bibliographic description of the MEI file.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "fileDesc")]
pub struct FileDesc {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<FileDescChild>,
}
impl Validate for FileDesc {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}
