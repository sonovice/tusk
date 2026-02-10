//! Import of header, paper, layout, and midi blocks.
//!
//! Stores LilyPond header/paper/layout/midi blocks in MEI `ExtMeta` elements
//! (under `MeiHead`) and in `ScoreDef` labels for lossless roundtrip.

use tusk_model::elements::{
    ExtMeta, ExtMetaChild, FileDesc, FileDescChild, MeiHead, MeiHeadChild, Title, TitleChild,
    TitleStmt, TitleStmtChild,
};

use crate::model::{self, ScoreItem, ToplevelExpression};
use crate::serializer;

use super::signatures;

/// Build MeiHead from LilyPond file, populating metadata from \header and
/// storing all blocks (header/paper/layout/midi) as ExtMeta for lossless roundtrip.
pub(super) fn build_mei_head_from_file(file: &model::LilyPondFile) -> MeiHead {
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

    // Store top-level \header as ExtMeta for lossless roundtrip
    if let Some(hb) = top_header {
        let serialized = serializer::serialize_header_block(hb);
        let escaped = signatures::escape_label_value_pub(&serialized);
        let mut ext = ExtMeta::default();
        ext.common.label = Some(format!("lilypond:header,{escaped}"));
        ext.children.push(ExtMetaChild::Text(header_summary(hb)));
        head.children.push(MeiHeadChild::ExtMeta(Box::new(ext)));
    }

    // Store top-level \paper as ExtMeta
    for item in &file.items {
        if let ToplevelExpression::Paper(pb) = item {
            let serialized = serializer::serialize_paper_block(pb);
            let escaped = signatures::escape_label_value_pub(&serialized);
            let mut ext = ExtMeta::default();
            ext.common.label = Some(format!("lilypond:paper,{escaped}"));
            head.children.push(MeiHeadChild::ExtMeta(Box::new(ext)));
        }
    }

    // Store top-level \layout as ExtMeta
    for item in &file.items {
        if let ToplevelExpression::Layout(lb) = item {
            let serialized = serializer::serialize_layout_block(lb);
            let escaped = signatures::escape_label_value_pub(&serialized);
            let mut ext = ExtMeta::default();
            ext.common.label = Some(format!("lilypond:layout,{escaped}"));
            head.children.push(MeiHeadChild::ExtMeta(Box::new(ext)));
        }
    }

    // Store top-level \midi as ExtMeta
    for item in &file.items {
        if let ToplevelExpression::Midi(mb) = item {
            let serialized = serializer::serialize_midi_block(mb);
            let escaped = signatures::escape_label_value_pub(&serialized);
            let mut ext = ExtMeta::default();
            ext.common.label = Some(format!("lilypond:midi,{escaped}"));
            head.children.push(MeiHeadChild::ExtMeta(Box::new(ext)));
        }
    }

    head
}

/// Build a short human-readable summary of a header block for ExtMeta text content.
fn header_summary(hb: &model::HeaderBlock) -> String {
    let mut parts = Vec::new();
    for field in &hb.fields {
        if let model::AssignmentValue::String(s) = &field.value {
            parts.push(format!("{}: {s}", field.name));
        }
    }
    if parts.is_empty() {
        "LilyPond header".to_string()
    } else {
        parts.join("; ")
    }
}

/// Build a label segment for score-level \header/\layout/\midi blocks.
///
/// Scans the LilyPond file for the first `\score` block and serializes
/// its header/layout/midi items as label entries.
///
/// Format: `lilypond:score-header,{escaped}|lilypond:score-layout,{escaped}|lilypond:score-midi,{escaped}`
pub(super) fn build_score_blocks_label(file: &model::LilyPondFile) -> String {
    let mut segments = Vec::new();

    for item in &file.items {
        if let ToplevelExpression::Score(sb) = item {
            for si in &sb.items {
                match si {
                    ScoreItem::Header(hb) => {
                        let serialized = serializer::serialize_header_block(hb);
                        let escaped = signatures::escape_label_value_pub(&serialized);
                        segments.push(format!("lilypond:score-header,{escaped}"));
                    }
                    ScoreItem::Layout(lb) => {
                        let serialized = serializer::serialize_layout_block(lb);
                        let escaped = signatures::escape_label_value_pub(&serialized);
                        segments.push(format!("lilypond:score-layout,{escaped}"));
                    }
                    ScoreItem::Midi(mb) => {
                        let serialized = serializer::serialize_midi_block(mb);
                        let escaped = signatures::escape_label_value_pub(&serialized);
                        segments.push(format!("lilypond:score-midi,{escaped}"));
                    }
                    ScoreItem::Music(_) => {}
                }
            }
            break; // Only first score block
        }
    }

    segments.join("|")
}
