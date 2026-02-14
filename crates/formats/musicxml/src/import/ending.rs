//! Ending/volta restructuring for MusicXML → MEI import.
//!
//! MusicXML represents volta brackets as `<barline><ending>` on boundary
//! measures. MEI uses structural `<ending>` elements that wrap measures.
//!
//! After all measures are converted to `SectionChild::Measure`, this module
//! scans MusicXML barlines for ending boundaries and restructures the section
//! children to wrap measure ranges in `SectionChild::Ending`.

use crate::context::ConversionContext;
use crate::model::data::StartStopDiscontinue;
use crate::model::elements::{MeasureContent, ScorePartwise};
use tusk_model::data::DataWord;
use tusk_model::elements::{Ending as MeiEnding, EndingChild, MeasureChild, Section, SectionChild};

/// Ending boundary detected from MusicXML barlines.
struct EndingBoundary {
    start_idx: usize,
    end_idx: usize,
    number: String,
    text: Option<String>,
    /// "stop", "discontinue", or None if open-ended (no explicit close)
    stop_type: Option<String>,
}

/// Detect barline endings in MusicXML and restructure MEI section children
/// to wrap measures inside `<ending>` containers.
///
/// 1. Scans the first part's barlines for `<ending type="start">` and `<ending type="stop/discontinue">`
/// 2. Groups the flat measure list into ending spans
/// 3. Restructures the section children, replacing measure runs with `SectionChild::Ending`
/// 4. Strips ending data from barline dir ExtensionStore entries to avoid duplication
pub fn restructure_endings(
    score: &ScorePartwise,
    section: &mut Section,
    ctx: &mut ConversionContext,
) {
    let first_part = match score.parts.first() {
        Some(p) => p,
        None => return,
    };

    let mut boundaries: Vec<EndingBoundary> = Vec::new();
    let mut pending_start: Option<(usize, String, Option<String>)> = None;

    for (measure_idx, mxml_measure) in first_part.measures.iter().enumerate() {
        for content in &mxml_measure.content {
            if let MeasureContent::Barline(barline) = content {
                if let Some(ref ending) = barline.ending {
                    match ending.ending_type {
                        StartStopDiscontinue::Start => {
                            // If there's an unfinished pending start, close it implicitly
                            if let Some((start, num, text)) = pending_start.take() {
                                boundaries.push(EndingBoundary {
                                    start_idx: start,
                                    end_idx: measure_idx.saturating_sub(1).max(start),
                                    number: num,
                                    text,
                                    stop_type: None, // implicitly closed
                                });
                            }
                            pending_start =
                                Some((measure_idx, ending.number.clone(), ending.text.clone()));
                        }
                        StartStopDiscontinue::Stop => {
                            if let Some((start, num, text)) = pending_start.take() {
                                boundaries.push(EndingBoundary {
                                    start_idx: start,
                                    end_idx: measure_idx,
                                    number: num,
                                    text,
                                    stop_type: Some("stop".to_string()),
                                });
                            }
                        }
                        StartStopDiscontinue::Discontinue => {
                            if let Some((start, num, text)) = pending_start.take() {
                                boundaries.push(EndingBoundary {
                                    start_idx: start,
                                    end_idx: measure_idx,
                                    number: num,
                                    text,
                                    stop_type: Some("discontinue".to_string()),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // Close any trailing pending start — open-ended ending
    if let Some((start, num, text)) = pending_start.take() {
        let last = first_part.measures.len().saturating_sub(1).max(start);
        boundaries.push(EndingBoundary {
            start_idx: start,
            end_idx: last,
            number: num,
            text,
            stop_type: None, // no explicit close
        });
    }

    if boundaries.is_empty() {
        return;
    }

    // Strip ending data from barline dirs on boundary measures.
    // This avoids double-emitting endings on export — the structural <ending> container
    // takes precedence.
    for boundary in &boundaries {
        for idx in [boundary.start_idx, boundary.end_idx] {
            if let Some(SectionChild::Measure(measure)) = section.children.get_mut(idx) {
                strip_ending_from_barline_dirs(measure, ctx);
            }
        }
    }

    // Restructure: replace measure ranges with Ending containers.
    // Process boundaries in reverse order so index shifts don't affect earlier ranges.
    for boundary in boundaries.iter().rev() {
        let start = boundary.start_idx;
        let end = boundary.end_idx;
        if start >= section.children.len() || end >= section.children.len() || start > end {
            continue;
        }

        // Drain the range of children
        let drained: Vec<SectionChild> = section.children.drain(start..=end).collect();

        // Build MEI <ending> with drained measures as children
        let mut mei_ending = MeiEnding::default();
        mei_ending.common.n = Some(DataWord::from(boundary.number.clone()));
        if let Some(ref text) = boundary.text {
            mei_ending.common.label = Some(text.clone());
        }
        // Store stop type for roundtrip: "stop", "discontinue", or absent (open-ended)
        mei_ending.common.r#type = boundary.stop_type.clone();

        for child in drained {
            match child {
                SectionChild::Measure(m) => {
                    mei_ending.children.push(EndingChild::Measure(m));
                }
                SectionChild::Section(s) => {
                    mei_ending.children.push(EndingChild::Section(s));
                }
                SectionChild::Ending(e) => {
                    // Nested endings shouldn't happen, but flatten them
                    for ec in e.children {
                        mei_ending.children.push(ec);
                    }
                }
                SectionChild::Expansion(exp) => {
                    mei_ending.children.push(EndingChild::Expansion(exp));
                }
            }
        }

        section
            .children
            .insert(start, SectionChild::Ending(Box::new(mei_ending)));
    }
}

/// Strip ending data from barline dirs on a measure.
///
/// Removes the `ending` field from `BarlineData` in ExtensionStore for each
/// barline dir on the measure. This prevents the ending from being double-emitted
/// on export (the structural `<ending>` container takes over).
fn strip_ending_from_barline_dirs(
    measure: &mut tusk_model::elements::Measure,
    ctx: &mut ConversionContext,
) {
    for child in &measure.children {
        if let MeasureChild::Dir(dir) = child {
            if let Some(ref id) = dir.common.xml_id {
                // Identify barline dirs by ExtensionStore membership
                if let Some(bd) = ctx.ext_store_mut().barline_mut(id) {
                    bd.ending = None;
                }
            }
        }
    }
}
