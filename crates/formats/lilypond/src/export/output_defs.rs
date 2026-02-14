//! Export of header, paper, layout, and midi blocks.
//!
//! Reads typed `OutputDef` extensions from MEI `ExtMeta` elements and `ScoreDef`
//! labels, converting them back to LilyPond AST nodes. No re-parsing of LilyPond
//! source is needed.

use tusk_model::elements::{Mei, MeiChild, MeiHeadChild, ScoreChild};
use tusk_model::extensions::{ExtensionStore, OutputDefKind};

use crate::import::output_def_conv;
use crate::model::ScoreItem;

/// Extract top-level header/paper/layout/midi blocks from MeiHead ExtMeta via ext_store.
pub(super) fn extract_toplevel_blocks(
    mei: &Mei,
    ext_store: &ExtensionStore,
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
                    let id = match ext.common.xml_id.as_deref() {
                        Some(id) => id,
                        None => continue,
                    };
                    if let Some(defs) = ext_store.output_defs(id) {
                        for def in defs {
                            match def.kind {
                                OutputDefKind::Header if header.is_none() => {
                                    header = Some(output_def_conv::output_def_to_header(def));
                                }
                                OutputDefKind::Paper if paper.is_none() => {
                                    paper = Some(output_def_conv::output_def_to_paper(def));
                                }
                                OutputDefKind::Layout if layout.is_none() => {
                                    layout = Some(output_def_conv::output_def_to_layout(def));
                                }
                                OutputDefKind::Midi if midi.is_none() => {
                                    midi = Some(output_def_conv::output_def_to_midi(def));
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    (header, paper, layout, midi)
}

/// Extract score-level header/layout/midi blocks from ScoreDef via ext_store.
pub(super) fn extract_score_blocks(score: &tusk_model::elements::Score, ext_store: &ExtensionStore) -> Vec<ScoreItem> {
    let mut items = Vec::new();

    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child {
            if let Some(id) = score_def.common.xml_id.as_deref() {
                if let Some(defs) = ext_store.output_defs(id) {
                    for def in defs {
                        match def.kind {
                            OutputDefKind::Header => {
                                items.push(ScoreItem::Header(
                                    output_def_conv::output_def_to_header(def),
                                ));
                            }
                            OutputDefKind::Layout => {
                                items.push(ScoreItem::Layout(
                                    output_def_conv::output_def_to_layout(def),
                                ));
                            }
                            OutputDefKind::Midi => {
                                items.push(ScoreItem::Midi(
                                    output_def_conv::output_def_to_midi(def),
                                ));
                            }
                            OutputDefKind::Paper => {
                                // Paper doesn't appear at score level,
                                // but handle gracefully
                            }
                        }
                    }
                }
            }
        }
    }

    items
}
