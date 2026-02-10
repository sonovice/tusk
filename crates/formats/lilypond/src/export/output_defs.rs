//! Export of header, paper, layout, and midi blocks.
//!
//! Reads LilyPond header/paper/layout/midi blocks from MEI `ExtMeta` elements
//! and `ScoreDef` labels, parsing them back to AST nodes.

use tusk_model::elements::{Mei, MeiChild, MeiHeadChild, ScoreChild};

use crate::model::{ScoreItem, ToplevelExpression};

/// Extract top-level header/paper/layout/midi blocks from MeiHead ExtMeta.
pub(super) fn extract_toplevel_blocks(
    mei: &Mei,
) -> (
    Option<crate::model::HeaderBlock>,
    Option<crate::model::PaperBlock>,
    Option<crate::model::LayoutBlock>,
    Option<crate::model::MidiBlock>,
) {
    let mut header = None;
    let mut paper = None;
    let mut layout = None;
    let mut midi = None;

    for child in &mei.children {
        if let MeiChild::MeiHead(head) = child {
            for hc in &head.children {
                if let MeiHeadChild::ExtMeta(ext) = hc {
                    let label = match ext.common.label.as_deref() {
                        Some(l) => l,
                        None => continue,
                    };
                    if let Some(escaped) = label.strip_prefix("lilypond:header,") {
                        header = parse_header_from_label(escaped);
                    } else if let Some(escaped) = label.strip_prefix("lilypond:paper,") {
                        paper = parse_paper_from_label(escaped);
                    } else if let Some(escaped) = label.strip_prefix("lilypond:layout,") {
                        layout = parse_layout_from_label(escaped);
                    } else if let Some(escaped) = label.strip_prefix("lilypond:midi,") {
                        midi = parse_midi_from_label(escaped);
                    }
                }
            }
        }
    }

    (header, paper, layout, midi)
}

/// Extract score-level header/layout/midi blocks from ScoreDef label.
pub(super) fn extract_score_blocks(score: &tusk_model::elements::Score) -> Vec<ScoreItem> {
    let mut items = Vec::new();

    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child
            && let Some(label) = &score_def.common.label
        {
            for segment in label.split('|') {
                if let Some(escaped) = segment.strip_prefix("lilypond:score-header,") {
                    if let Some(hb) = parse_header_from_label(escaped) {
                        items.push(ScoreItem::Header(hb));
                    }
                } else if let Some(escaped) = segment.strip_prefix("lilypond:score-layout,") {
                    if let Some(lb) = parse_layout_from_label(escaped) {
                        items.push(ScoreItem::Layout(lb));
                    }
                } else if let Some(escaped) = segment.strip_prefix("lilypond:score-midi,")
                    && let Some(mb) = parse_midi_from_label(escaped)
                {
                    items.push(ScoreItem::Midi(mb));
                }
            }
        }
    }

    items
}

/// Parse a serialized \header block from a label value.
fn parse_header_from_label(escaped: &str) -> Option<crate::model::HeaderBlock> {
    use crate::parser::Parser;
    let serialized = crate::import::signatures::unescape_label_value(escaped);
    let src = format!("{serialized}\n");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let ToplevelExpression::Header(hb) = item {
            return Some(hb.clone());
        }
    }
    None
}

/// Parse a serialized \paper block from a label value.
fn parse_paper_from_label(escaped: &str) -> Option<crate::model::PaperBlock> {
    use crate::parser::Parser;
    let serialized = crate::import::signatures::unescape_label_value(escaped);
    let src = format!("{serialized}\n");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let ToplevelExpression::Paper(pb) = item {
            return Some(pb.clone());
        }
    }
    None
}

/// Parse a serialized \layout block from a label value.
fn parse_layout_from_label(escaped: &str) -> Option<crate::model::LayoutBlock> {
    use crate::parser::Parser;
    let serialized = crate::import::signatures::unescape_label_value(escaped);
    let src = format!("{serialized}\n");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let ToplevelExpression::Layout(lb) = item {
            return Some(lb.clone());
        }
    }
    None
}

/// Parse a serialized \midi block from a label value.
fn parse_midi_from_label(escaped: &str) -> Option<crate::model::MidiBlock> {
    use crate::parser::Parser;
    let serialized = crate::import::signatures::unescape_label_value(escaped);
    let src = format!("{serialized}\n");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let ToplevelExpression::Midi(mb) = item {
            return Some(mb.clone());
        }
    }
    None
}
