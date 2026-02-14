//! Import of header, paper, layout, and midi blocks.
//!
//! Stores LilyPond header/paper/layout/midi blocks as typed `OutputDef` extensions
//! in MEI `ExtMeta` elements (under `MeiHead`) and in `ScoreDef` labels for lossless
//! roundtrip. Uses JSON-serialized `OutputDef` structs instead of serialized LilyPond
//! source, eliminating re-parsing on export.

use crate::model::{self, ScoreItem, ToplevelExpression};
use tusk_model::elements::{
    ExtMeta, ExtMetaChild, FileDesc, FileDescChild, MeiHead, MeiHeadChild, Title, TitleChild,
    TitleStmt, TitleStmtChild,
};
use tusk_model::ExtensionStore;

use super::output_def_conv;

/// Build MeiHead from LilyPond file, populating metadata from \header and
/// storing all blocks (header/paper/layout/midi) as typed OutputDef extensions.
pub(super) fn build_mei_head_from_file(file: &model::LilyPondFile, ext_store: &mut ExtensionStore) -> MeiHead {
    let mut head = MeiHead::default();

    // Find top-level \header
    let top_header = file.items.iter().find_map(|item| {
        if let ToplevelExpression::Header(hb) = item {
            Some(hb)
        } else {
            None
        }
    });

    // Build fileDesc with titleStmt from header title
    let mut title_stmt = TitleStmt::default();
    if let Some(hb) = top_header {
        // Extract title field for MEI Title element
        for field in &hb.fields {
            if field.name == "title"
                && let model::AssignmentValue::String(s) = &field.value
            {
                let mut title = Title::default();
                title.children.push(TitleChild::Text(s.clone()));
                title_stmt
                    .children
                    .push(TitleStmtChild::Title(Box::new(title)));
            }
        }
    }
    let mut file_desc = FileDesc::default();
    file_desc
        .children
        .push(FileDescChild::TitleStmt(Box::new(title_stmt)));
    head.children
        .push(MeiHeadChild::FileDesc(Box::new(file_desc)));

    // Collect all top-level output defs as typed OutputDef structs
    let mut output_defs = Vec::new();

    if let Some(hb) = top_header {
        output_defs.push(output_def_conv::header_to_output_def(hb));
    }

    for item in &file.items {
        match item {
            ToplevelExpression::Paper(pb) => {
                output_defs.push(output_def_conv::paper_to_output_def(pb));
            }
            ToplevelExpression::Layout(lb) => {
                output_defs.push(output_def_conv::layout_to_output_def(lb));
            }
            ToplevelExpression::Midi(mb) => {
                output_defs.push(output_def_conv::midi_to_output_def(mb));
            }
            _ => {}
        }
    }

    // Store all output defs in ext_store
    if !output_defs.is_empty() {
        let ext_id = "ly-extmeta-0".to_string();
        let mut ext = ExtMeta::default();
        ext.common.xml_id = Some(ext_id.clone());
        // Human-readable summary
        let summary: Vec<String> = output_defs
            .iter()
            .map(|od| format!("{:?}", od.kind))
            .collect();
        ext.children.push(ExtMetaChild::Text(summary.join(", ")));
        head.children.push(MeiHeadChild::ExtMeta(Box::new(ext)));
        ext_store.insert_output_defs(ext_id, output_defs);
    }

    head
}

/// Collect score-level output defs from a score block.
pub(super) fn collect_score_block_output_defs(sb: &model::ScoreBlock) -> Vec<tusk_model::OutputDef> {
    let mut output_defs = Vec::new();

    for si in &sb.items {
        match si {
            ScoreItem::Header(hb) => {
                output_defs.push(output_def_conv::header_to_output_def(hb));
            }
            ScoreItem::Layout(lb) => {
                output_defs.push(output_def_conv::layout_to_output_def(lb));
            }
            ScoreItem::Midi(mb) => {
                output_defs.push(output_def_conv::midi_to_output_def(mb));
            }
            ScoreItem::Music(_) => {}
        }
    }

    output_defs
}
