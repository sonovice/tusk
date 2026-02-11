//! Chord-mode / Harm handling for LilyPond export.
//!
//! Extracts chord-mode events from MEI `<harm>` control events and
//! reconstructs `\new ChordNames \chordmode { ... }` context structure.

use tusk_model::StaffContext;
use tusk_model::elements::{MeasureChild, ScoreChild, ScoreDefChild, SectionChild};

use crate::model::Music;

/// Metadata for a ChordNames context, extracted from staffGrp label.
pub(super) struct ChordNamesMeta {
    pub(super) name: Option<String>,
    pub(super) with_block_str: Option<String>,
}

/// Collect chord-mode events from Harm control events in the score.
///
/// Returns a list of `Music::ChordModeEntry` items extracted from
/// `<harm>` elements with `tusk:chord-mode,{JSON}` labels.
pub(super) fn collect_chord_mode_harms(score: &tusk_model::elements::Score) -> Vec<Music> {
    let mut events = Vec::new();
    for child in &score.children {
        if let ScoreChild::Section(section) = child {
            for sc in &section.children {
                if let SectionChild::Measure(measure) = sc {
                    for mc in &measure.children {
                        if let MeasureChild::Harm(harm) = mc
                            && let Some(ce) = parse_chord_mode_from_harm(harm)
                        {
                            events.push(Music::ChordModeEntry(ce));
                        }
                    }
                }
            }
        }
    }
    events
}

/// Parse a ChordModeEvent from a Harm element's typed JSON label.
fn parse_chord_mode_from_harm(
    harm: &tusk_model::elements::Harm,
) -> Option<crate::model::note::ChordModeEvent> {
    let label = harm.common.label.as_deref()?;
    let json = label.strip_prefix("tusk:chord-mode,")?;
    let info: tusk_model::ChordModeInfo = serde_json::from_str(json).ok()?;
    parse_chord_mode_event_str(&info.serialized)
}

/// Parse a chord-mode event string back into a ChordModeEvent.
///
/// Re-parses through the LilyPond parser by wrapping in `\chordmode { ... }`.
fn parse_chord_mode_event_str(s: &str) -> Option<crate::model::note::ChordModeEvent> {
    use crate::parser::Parser;
    let src = format!("\\chordmode {{ {s} }}");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let crate::model::ToplevelExpression::Music(Music::ChordMode { body }) = item {
            if let Music::Sequential(items) = body.as_ref() {
                for m in items {
                    if let Music::ChordModeEntry(ce) = m {
                        return Some(ce.clone());
                    }
                }
            }
            if let Music::ChordModeEntry(ce) = body.as_ref() {
                return Some(ce.clone());
            }
        }
    }
    None
}

/// Extract ChordNames context metadata from the staffGrp label.
pub(super) fn extract_chord_names_meta(
    score: &tusk_model::elements::Score,
) -> Option<ChordNamesMeta> {
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child {
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child
                    && let Some(label) = &grp.common.label
                {
                    for segment in label.split('|') {
                        if let Some(json) = segment.strip_prefix("tusk:chord-names-context,")
                            && let Ok(ctx) = serde_json::from_str::<StaffContext>(json)
                        {
                            return Some(ChordNamesMeta {
                                name: ctx.name,
                                with_block_str: ctx.with_block,
                            });
                        }
                    }
                }
            }
        }
    }
    None
}
