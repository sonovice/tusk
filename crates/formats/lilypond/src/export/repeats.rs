//! Repeat and alternative wrapping for MEI→LilyPond export.
//!
//! Collects `<dir>` elements with `lilypond:repeat,` and `lilypond:ending,` labels
//! from MEI measure children and wraps Music items in `Music::Repeat` with alternatives.

use tusk_model::elements::MeasureChild;

use crate::model::Music;

/// Collected repeat span info from Dir elements with `lilypond:repeat,` labels.
pub(super) struct RepeatSpanInfo {
    pub start_id: String,
    pub end_id: String,
    pub repeat_type: crate::model::RepeatType,
    pub count: u32,
    pub num_alternatives: u32,
}

/// Collected ending span info from Dir elements with `lilypond:ending,` labels.
pub(super) struct EndingSpanInfo {
    pub start_id: String,
    pub end_id: String,
    pub index: u32,
}

/// Collect repeat spans from Dir elements in measure children.
pub(super) fn collect_repeat_spans(measure_children: &[MeasureChild]) -> Vec<RepeatSpanInfo> {
    let mut spans = Vec::new();
    for mc in measure_children {
        if let MeasureChild::Dir(dir) = mc
            && let Some(label) = dir.common.label.as_deref()
            && let Some(rest) = label.strip_prefix("lilypond:repeat,")
        {
            let start_id = dir
                .dir_log
                .startid
                .as_ref()
                .map(|u| u.0.trim_start_matches('#').to_string())
                .unwrap_or_default();
            let end_id = dir
                .dir_log
                .endid
                .as_ref()
                .map(|u| u.0.trim_start_matches('#').to_string())
                .unwrap_or_default();

            if let Some((repeat_type, count, num_alts)) = parse_repeat_label(rest) {
                spans.push(RepeatSpanInfo {
                    start_id,
                    end_id,
                    repeat_type,
                    count,
                    num_alternatives: num_alts,
                });
            }
        }
    }
    spans
}

/// Parse repeat label value: `TYPE,COUNT[,alts=N]`
fn parse_repeat_label(s: &str) -> Option<(crate::model::RepeatType, u32, u32)> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() < 2 {
        return None;
    }
    let repeat_type = crate::model::RepeatType::from_name(parts[0])?;
    let count: u32 = parts[1].parse().ok()?;
    let num_alts = parts
        .iter()
        .find_map(|p| p.strip_prefix("alts="))
        .and_then(|n| n.parse().ok())
        .unwrap_or(0);
    Some((repeat_type, count, num_alts))
}

/// Collect ending spans from Dir elements in measure children.
pub(super) fn collect_ending_spans(measure_children: &[MeasureChild]) -> Vec<EndingSpanInfo> {
    let mut spans = Vec::new();
    for mc in measure_children {
        if let MeasureChild::Dir(dir) = mc
            && let Some(label) = dir.common.label.as_deref()
            && let Some(rest) = label.strip_prefix("lilypond:ending,")
        {
            let start_id = dir
                .dir_log
                .startid
                .as_ref()
                .map(|u| u.0.trim_start_matches('#').to_string())
                .unwrap_or_default();
            let end_id = dir
                .dir_log
                .endid
                .as_ref()
                .map(|u| u.0.trim_start_matches('#').to_string())
                .unwrap_or_default();
            if let Ok(index) = rest.parse::<u32>() {
                spans.push(EndingSpanInfo {
                    start_id,
                    end_id,
                    index,
                });
            }
        }
    }
    // Sort by index for deterministic ordering
    spans.sort_by_key(|s| s.index);
    spans
}

/// Apply repeat and alternative wrapping to Music items.
///
/// For each repeat span, finds the body range in the items by start/end ID,
/// then finds any associated endings by ID overlap. Wraps the body in
/// Music::Repeat with alternative endings.
///
/// Processes innermost repeats first (smallest ranges) to handle nesting correctly.
pub(super) fn apply_repeat_wrapping(
    items: &mut Vec<Music>,
    item_ids: &[Option<String>],
    repeat_spans: &[RepeatSpanInfo],
    ending_spans: &[EndingSpanInfo],
) {
    if repeat_spans.is_empty() || items.is_empty() {
        return;
    }

    let mut ranges: Vec<(usize, usize, usize)> = Vec::new();
    let mut ids: Vec<Option<String>> = item_ids.to_vec();

    for (si, span) in repeat_spans.iter().enumerate() {
        let start_idx = ids
            .iter()
            .position(|id| id.as_deref().is_some_and(|i| i == span.start_id));
        let body_end_idx = ids
            .iter()
            .rposition(|id| id.as_deref().is_some_and(|i| i == span.end_id));

        if let (Some(s), Some(be)) = (start_idx, body_end_idx) {
            let full_end = if span.num_alternatives > 0 {
                ending_spans
                    .iter()
                    .filter(|e| e.index < span.num_alternatives)
                    .filter_map(|e| {
                        ids.iter()
                            .rposition(|id| id.as_deref().is_some_and(|i| i == e.end_id))
                    })
                    .max()
                    .unwrap_or(be)
            } else {
                be
            };
            ranges.push((s, full_end, si));
        }
    }

    // Sort by range size ascending (innermost first) for correct nesting
    ranges.sort_by(|a, b| {
        let size_a = a.1 - a.0;
        let size_b = b.1 - b.0;
        size_a.cmp(&size_b).then(b.0.cmp(&a.0))
    });

    for &(_orig_start, _orig_end, span_idx) in &ranges {
        let span = &repeat_spans[span_idx];

        let cur_body_start = ids
            .iter()
            .position(|id| id.as_deref().is_some_and(|i| i == span.start_id));
        let cur_body_end = ids
            .iter()
            .rposition(|id| id.as_deref().is_some_and(|i| i == span.end_id));

        let (Some(bs), Some(be)) = (cur_body_start, cur_body_end) else {
            continue;
        };
        if bs > be || be >= items.len() {
            continue;
        }

        if span.num_alternatives > 0 {
            wrap_repeat_with_alternatives(items, &mut ids, bs, be, span, ending_spans);
        } else {
            wrap_repeat_body_only(items, &mut ids, bs, be, span);
        }
    }
}

/// Wrap a repeat that has alternative endings.
fn wrap_repeat_with_alternatives(
    items: &mut Vec<Music>,
    ids: &mut Vec<Option<String>>,
    bs: usize,
    be: usize,
    span: &RepeatSpanInfo,
    ending_spans: &[EndingSpanInfo],
) {
    let mut sorted_endings: Vec<&EndingSpanInfo> = ending_spans
        .iter()
        .filter(|e| e.index < span.num_alternatives)
        .collect();
    sorted_endings.sort_by_key(|e| e.index);

    // Find the actual end position including all alternatives
    let mut alt_end_pos = be;
    for ending in &sorted_endings {
        if let Some(e) = ids
            .iter()
            .rposition(|id| id.as_deref().is_some_and(|i| i == ending.end_id))
            && e < items.len()
            && e > alt_end_pos
        {
            alt_end_pos = e;
        }
    }

    // Extract the full range [bs..=alt_end_pos]
    let full_items: Vec<Music> = items.drain(bs..=alt_end_pos).collect();
    let full_ids: Vec<Option<String>> = ids.drain(bs..=alt_end_pos).collect();

    let body_len = be - bs + 1;
    let body_items: Vec<Music> = full_items[..body_len].to_vec();
    let remaining_items = &full_items[body_len..];
    let remaining_ids = &full_ids[body_len..];

    // Split remaining items into alternatives by matching ending IDs
    let mut alternatives: Vec<Music> = Vec::new();
    let mut consumed = 0;
    for ending in &sorted_endings {
        let local_start = remaining_ids[consumed..]
            .iter()
            .position(|id| id.as_deref().is_some_and(|i| i == ending.start_id));
        let local_end = remaining_ids[consumed..]
            .iter()
            .rposition(|id| id.as_deref().is_some_and(|i| i == ending.end_id));
        if let (Some(ls), Some(le)) = (local_start, local_end) {
            let s = consumed + ls;
            let e = consumed + le;
            let alt_items: Vec<Music> = remaining_items[s..=e].to_vec();
            let alt_music = if alt_items.len() == 1 {
                alt_items.into_iter().next().unwrap()
            } else {
                Music::Sequential(alt_items)
            };
            alternatives.push(alt_music);
            consumed = e + 1;
        }
    }

    let repeat = Music::Repeat {
        repeat_type: span.repeat_type,
        count: span.count,
        body: Box::new(Music::Sequential(body_items)),
        alternatives: if alternatives.is_empty() {
            None
        } else {
            Some(alternatives)
        },
    };
    items.insert(bs, repeat);
    ids.insert(bs, Some(span.start_id.clone()));
}

/// Wrap a repeat with no alternatives (body only).
fn wrap_repeat_body_only(
    items: &mut Vec<Music>,
    ids: &mut Vec<Option<String>>,
    bs: usize,
    be: usize,
    span: &RepeatSpanInfo,
) {
    let start_id = ids[bs].clone();
    let body_items: Vec<Music> = items.drain(bs..=be).collect();
    let repeat = Music::Repeat {
        repeat_type: span.repeat_type,
        count: span.count,
        body: Box::new(Music::Sequential(body_items)),
        alternatives: None,
    };
    items.insert(bs, repeat);
    ids.drain(bs..=be);
    ids.insert(bs, start_id);
}
