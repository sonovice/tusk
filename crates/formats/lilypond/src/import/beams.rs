//! Beam grouping and duration utility helpers for LilyPond import.

use tusk_model::elements::{Beam, BeamChild, Layer, LayerChild};

/// Group a range of layer children into a `<beam>` element.
///
/// Replaces `layer.children[start..=end]` with a single `LayerChild::Beam`
/// containing those elements as `BeamChild` items.
pub(super) fn group_beamed_notes(layer: &mut Layer, start: usize, end: usize, beam_id: u32) {
    if start >= layer.children.len() || end >= layer.children.len() || start > end {
        return;
    }

    let mut beam = Beam::default();
    beam.common.xml_id = Some(format!("ly-beam-{beam_id}"));

    // Drain the range and convert LayerChild → BeamChild
    let items: Vec<LayerChild> = layer.children.drain(start..=end).collect();
    for item in items {
        if let Some(bc) = layer_child_to_beam_child(item) {
            beam.children.push(bc);
        }
    }

    // Insert the beam at the start position
    layer
        .children
        .insert(start, LayerChild::Beam(Box::new(beam)));
}

/// Convert a LayerChild to a BeamChild (Note, Rest, Chord).
fn layer_child_to_beam_child(child: LayerChild) -> Option<BeamChild> {
    match child {
        LayerChild::Note(n) => Some(BeamChild::Note(n)),
        LayerChild::Rest(r) => Some(BeamChild::Rest(r)),
        LayerChild::Chord(c) => Some(BeamChild::Chord(c)),
        LayerChild::Beam(b) => Some(BeamChild::Beam(b)),
        _ => None,
    }
}

/// Estimate beats from a LilyPond Duration (assuming 4/4 time).
pub(super) fn duration_to_beats(dur: &crate::model::Duration) -> f64 {
    let base_beats = 4.0 / dur.base as f64;
    let dot_factor = 2.0 - 0.5f64.powi(dur.dots as i32);
    let mut beats = base_beats * dot_factor;
    for &(n, d) in &dur.multipliers {
        beats *= n as f64 / d as f64;
    }
    beats
}
