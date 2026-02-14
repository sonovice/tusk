//! Figured bass handling for LilyPond export.
//!
//! Extracts figure events from MEI `<fb>` control events and
//! reconstructs `\new FiguredBass \figuremode { ... }` context structure.

use tusk_model::elements::{MeasureChild, ScoreChild, ScoreDefChild, SectionChild};
use tusk_model::extensions::ExtensionStore;

use crate::model::Music;

/// Metadata for a FiguredBass context, extracted from staffGrp label.
pub(super) struct FiguredBassMeta {
    pub(super) name: Option<String>,
    pub(super) with_block_str: Option<String>,
}

/// Collect figure events from Fb control events in the score via ext_store.
pub(super) fn collect_figure_mode_fbs(score: &tusk_model::elements::Score, ext_store: &ExtensionStore) -> Vec<Music> {
    let mut events = Vec::new();
    for child in &score.children {
        if let ScoreChild::Section(section) = child {
            for sc in &section.children {
                if let SectionChild::Measure(measure) = sc {
                    for mc in &measure.children {
                        if let MeasureChild::Fb(fb) = mc
                            && let Some(fe) = parse_figure_event_from_ext(fb, ext_store)
                        {
                            events.push(Music::Figure(fe));
                        }
                    }
                }
            }
        }
    }
    events
}

/// Parse a FigureEvent from an Fb element via ext_store.
fn parse_figure_event_from_ext(
    fb: &tusk_model::elements::Fb,
    ext_store: &ExtensionStore,
) -> Option<crate::model::note::FigureEvent> {
    let id = fb.common.xml_id.as_deref()?;
    let info = ext_store.figured_bass_info(id)?;
    parse_figure_event_str(&info.serialized)
}

/// Parse a figure event string back into a FigureEvent.
///
/// Re-parses through the LilyPond parser by wrapping in `\figuremode { ... }`.
fn parse_figure_event_str(s: &str) -> Option<crate::model::note::FigureEvent> {
    use crate::parser::Parser;
    let src = format!("\\figuremode {{ {s} }}");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let crate::model::ToplevelExpression::Music(Music::FigureMode { body }) = item {
            if let Music::Sequential(items) = body.as_ref() {
                for m in items {
                    if let Music::Figure(fe) = m {
                        return Some(fe.clone());
                    }
                }
            }
            if let Music::Figure(fe) = body.as_ref() {
                return Some(fe.clone());
            }
        }
    }
    None
}

/// Extract FiguredBass context metadata from the staffGrp via ext_store.
///
/// The import stores figured-bass context under key `"{grp_id}-figuredbass"`.
pub(super) fn extract_figured_bass_meta(
    score: &tusk_model::elements::Score,
    ext_store: &ExtensionStore,
) -> Option<FiguredBassMeta> {
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child {
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child {
                    if let Some(id) = grp.common.xml_id.as_deref() {
                        let fb_key = format!("{id}-figuredbass");
                        if let Some(ctx) = ext_store.staff_context(&fb_key) {
                            return Some(FiguredBassMeta {
                                name: ctx.name.clone(),
                                with_block_str: ctx.with_block.clone(),
                            });
                        }
                    }
                }
            }
        }
    }
    None
}
