//! Figured bass handling for LilyPond export.
//!
//! Extracts figure events from MEI `<fb>` control events and
//! reconstructs `\new FiguredBass \figuremode { ... }` context structure.

use tusk_model::elements::{MeasureChild, ScoreChild, ScoreDefChild, SectionChild};

use crate::model::Music;

/// Metadata for a FiguredBass context, extracted from staffGrp label.
pub(super) struct FiguredBassMeta {
    pub(super) name: Option<String>,
    pub(super) with_block_str: Option<String>,
}

/// Collect figure events from Fb control events in the score.
///
/// Returns a list of `Music::Figure` items extracted from
/// `<fb>` elements with `lilypond:figure,` labels.
pub(super) fn collect_figure_mode_fbs(score: &tusk_model::elements::Score) -> Vec<Music> {
    let mut events = Vec::new();
    for child in &score.children {
        if let ScoreChild::Section(section) = child {
            for sc in &section.children {
                if let SectionChild::Measure(measure) = sc {
                    for mc in &measure.children {
                        if let MeasureChild::Fb(fb) = mc
                            && let Some(fe) = parse_figure_event_from_fb(fb)
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

/// Parse a FigureEvent from an Fb element's label.
///
/// Label format: `lilypond:figure,SERIALIZED`
fn parse_figure_event_from_fb(
    fb: &tusk_model::elements::Fb,
) -> Option<crate::model::note::FigureEvent> {
    let label = fb.common.label.as_deref()?;
    let serialized = label.strip_prefix("lilypond:figure,")?;
    // Unescape label value
    let unescaped = crate::import::signatures::unescape_label_value(serialized);
    parse_figure_event_str(&unescaped)
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

/// Extract FiguredBass context metadata from the staffGrp label.
pub(super) fn extract_figured_bass_meta(
    score: &tusk_model::elements::Score,
) -> Option<FiguredBassMeta> {
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child {
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child
                    && let Some(label) = &grp.common.label
                {
                    for segment in label.split('|') {
                        if let Some(rest) = segment.strip_prefix("lilypond:figuredbass") {
                            return Some(parse_figured_bass_meta(rest));
                        }
                    }
                }
            }
        }
    }
    None
}

/// Parse FiguredBass metadata from the label suffix.
///
/// Format: `[,name=Name][,with=...]`
fn parse_figured_bass_meta(s: &str) -> FiguredBassMeta {
    let mut name = None;
    let mut with_block_str = None;
    let parts: Vec<&str> = s.split(',').collect();
    for part in &parts {
        if let Some(n) = part.strip_prefix("name=") {
            name = Some(n.to_string());
        } else if let Some(w) = part.strip_prefix("with=") {
            with_block_str = Some(w.to_string());
        }
    }
    FiguredBassMeta {
        name,
        with_block_str,
    }
}
