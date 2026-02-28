//! Layer restructuring: beam grouping and tremolo wrapping.
//!
//! After notes are pushed to a flat layer, this module:
//! 1. Groups beamed notes into `<beam>` containers
//! 2. Wraps tremolo-marked notes in `<bTrem>` / `<fTrem>` containers

use crate::context::ConversionContext;
use std::collections::HashMap;
use tusk_model::data::DataDurationCmn;
use tusk_model::elements::{BTrem, BTremChild, Beam, BeamChild, FTrem, FTremChild, LayerChild};

// ============================================================================
// Tremolo Wrapping
// ============================================================================

/// Convert a MusicXML tremolo mark count to MEI @unitdur.
///
/// MusicXML tremolo value = number of beams: 1→8, 2→16, 3→32, 4→64, 5→128.
fn tremolo_value_to_unitdur(value: u8) -> DataDurationCmn {
    match value {
        1 => DataDurationCmn::N8,
        2 => DataDurationCmn::N16,
        3 => DataDurationCmn::N32,
        4 => DataDurationCmn::N64,
        5 => DataDurationCmn::N128,
        _ => DataDurationCmn::N32, // default: 3 marks
    }
}

/// Get the xml:id of a LayerChild (Note or Chord).
fn layer_child_id(child: &LayerChild) -> Option<&str> {
    match child {
        LayerChild::Note(n) => n.common.xml_id.as_deref(),
        LayerChild::Chord(c) => c.common.xml_id.as_deref(),
        _ => None,
    }
}

/// Convert a LayerChild (Note or Chord) to a BTremChild.
fn layer_child_to_btrem_child(child: LayerChild) -> Option<BTremChild> {
    match child {
        LayerChild::Note(n) => Some(BTremChild::Note(n)),
        LayerChild::Chord(c) => Some(BTremChild::Chord(c)),
        _ => None,
    }
}

/// Convert a LayerChild (Note or Chord) to an FTremChild.
fn layer_child_to_ftrem_child(child: LayerChild) -> Option<FTremChild> {
    match child {
        LayerChild::Note(n) => Some(FTremChild::Note(n)),
        LayerChild::Chord(c) => Some(FTremChild::Chord(c)),
        _ => None,
    }
}

/// Post-process layer children to wrap tremolo-marked notes/chords in bTrem/fTrem.
///
/// Scans layer children (which may already be inside Beam containers) for notes
/// whose xml:id appears in the tremolo_map. Wraps them accordingly:
/// - Single → bTrem container around the single note/chord
/// - Start + Stop → fTrem container around both notes/chords
pub fn wrap_tremolo_containers(
    children: Vec<LayerChild>,
    tremolo_map: &HashMap<String, crate::context::PendingTremolo>,
) -> Vec<LayerChild> {
    use crate::model::data::TremoloType;

    let mut result = Vec::new();
    let mut pending_ftrem_start: Option<(LayerChild, u8)> = None;

    for child in children {
        match child {
            LayerChild::Beam(beam) => {
                let wrapped_beam = wrap_tremolo_in_beam(*beam, tremolo_map);
                result.push(LayerChild::Beam(Box::new(wrapped_beam)));
            }
            _ => {
                if let Some(id) = layer_child_id(&child) {
                    if let Some(pt) = tremolo_map.get(id) {
                        match pt.tremolo_type {
                            TremoloType::Single => {
                                let mut btrem = BTrem::default();
                                btrem.b_trem_ges.unitdur = Some(tremolo_value_to_unitdur(pt.value));
                                if let Some(bc) = layer_child_to_btrem_child(child) {
                                    btrem.children.push(bc);
                                }
                                result.push(LayerChild::BTrem(Box::new(btrem)));
                                continue;
                            }
                            TremoloType::Start => {
                                pending_ftrem_start = Some((child, pt.value));
                                continue;
                            }
                            TremoloType::Stop => {
                                if let Some((start_child, value)) = pending_ftrem_start.take() {
                                    let mut ftrem = FTrem::default();
                                    ftrem.f_trem_ges.unitdur =
                                        Some(tremolo_value_to_unitdur(value));
                                    if let Some(fc) = layer_child_to_ftrem_child(start_child) {
                                        ftrem.children.push(fc);
                                    }
                                    if let Some(fc) = layer_child_to_ftrem_child(child) {
                                        ftrem.children.push(fc);
                                    }
                                    result.push(LayerChild::FTrem(Box::new(ftrem)));
                                    continue;
                                }
                                // No matching start — push child directly
                            }
                            TremoloType::Unmeasured => {
                                // Handled as ornam label, not a container
                            }
                        }
                    }
                }
                result.push(child);
            }
        }
    }

    // If there's an orphaned ftrem start, push it back
    if let Some((start_child, _)) = pending_ftrem_start {
        result.push(start_child);
    }

    result
}

/// Process beam children for tremolo wrapping.
fn wrap_tremolo_in_beam(
    mut beam: Beam,
    tremolo_map: &HashMap<String, crate::context::PendingTremolo>,
) -> Beam {
    use crate::model::data::TremoloType;

    fn beam_child_id(child: &BeamChild) -> Option<&str> {
        match child {
            BeamChild::Note(n) => n.common.xml_id.as_deref(),
            BeamChild::Chord(c) => c.common.xml_id.as_deref(),
            _ => None,
        }
    }

    fn beam_child_to_btrem_child(child: BeamChild) -> Option<BTremChild> {
        match child {
            BeamChild::Note(n) => Some(BTremChild::Note(n)),
            BeamChild::Chord(c) => Some(BTremChild::Chord(c)),
            _ => None,
        }
    }

    fn beam_child_to_ftrem_child(child: BeamChild) -> Option<FTremChild> {
        match child {
            BeamChild::Note(n) => Some(FTremChild::Note(n)),
            BeamChild::Chord(c) => Some(FTremChild::Chord(c)),
            _ => None,
        }
    }

    let has_tremolo = beam.children.iter().any(|c| {
        beam_child_id(c)
            .map(|id| tremolo_map.contains_key(id))
            .unwrap_or(false)
    });

    if !has_tremolo {
        return beam;
    }

    let mut new_children: Vec<BeamChild> = Vec::new();
    let mut pending_ftrem_start: Option<(BeamChild, u8)> = None;

    for child in std::mem::take(&mut beam.children) {
        if let Some(id) = beam_child_id(&child) {
            if let Some(pt) = tremolo_map.get(id) {
                match pt.tremolo_type {
                    TremoloType::Single => {
                        let mut btrem = BTrem::default();
                        btrem.b_trem_ges.unitdur = Some(tremolo_value_to_unitdur(pt.value));
                        if let Some(bc) = beam_child_to_btrem_child(child) {
                            btrem.children.push(bc);
                        }
                        new_children.push(BeamChild::BTrem(Box::new(btrem)));
                        continue;
                    }
                    TremoloType::Start => {
                        pending_ftrem_start = Some((child, pt.value));
                        continue;
                    }
                    TremoloType::Stop => {
                        if let Some((start_child, value)) = pending_ftrem_start.take() {
                            let mut ftrem = FTrem::default();
                            ftrem.f_trem_ges.unitdur = Some(tremolo_value_to_unitdur(value));
                            if let Some(fc) = beam_child_to_ftrem_child(start_child) {
                                ftrem.children.push(fc);
                            }
                            if let Some(fc) = beam_child_to_ftrem_child(child) {
                                ftrem.children.push(fc);
                            }
                            new_children.push(BeamChild::FTrem(Box::new(ftrem)));
                            continue;
                        }
                    }
                    // Unmeasured tremolo: no bTrem/fTrem wrapping needed —
                    // handled as regular ornament notation on the note.
                    TremoloType::Unmeasured => {}
                }
            }
        }
        new_children.push(child);
    }

    if let Some((start_child, _)) = pending_ftrem_start {
        new_children.push(start_child);
    }

    beam.children = new_children;
    beam
}

// ============================================================================
// Inline Attribute Changes
// ============================================================================

/// Emit inline MEI elements for mid-score attribute changes.
///
/// Compares the given MusicXML attributes against tracked state in the context.
/// When a clef, key, or time signature differs from the last-known value,
/// emits the corresponding inline MEI element (Clef, KeySig, MeterSig) into
/// the layer and updates the tracked state.
///
/// `target_local_staff`: when `Some(n)`, only emit clefs for staff `n` and
/// key/time only for staff 1 (global attrs). When `None`, emit all.
pub fn emit_inline_attribute_changes(
    attrs: &crate::model::attributes::Attributes,
    inline_children: &mut Vec<LayerChild>,
    ctx: &mut ConversionContext,
    target_local_staff: Option<u32>,
) {
    use crate::import::attributes::{
        convert_clef_attributes, convert_key_fifths, convert_time_signature,
    };
    use crate::model::attributes::KeyContent;

    let part_id = ctx.position().part_id.clone().unwrap_or_default();
    let emit_global = target_local_staff.is_none() || target_local_staff == Some(1);

    // --- Key signature changes (global — only staff 1) ---
    for key in &attrs.keys {
        if let KeyContent::Traditional(trad) = &key.content {
            let tracked = ctx.tracked_attrs().key_fifths.get(&part_id).copied();
            if emit_global && tracked.is_some() && tracked != Some(trad.fifths) {
                let mut keysig = tusk_model::elements::KeySig::default();
                keysig.key_sig_log.sig = Some(convert_key_fifths(trad.fifths));
                inline_children.push(LayerChild::KeySig(Box::new(keysig)));
            }
            ctx.tracked_attrs_mut()
                .key_fifths
                .insert(part_id.clone(), trad.fifths);
        }
    }

    // --- Time signature changes (global — only staff 1) ---
    for time in &attrs.times {
        let (count, unit, sym) = convert_time_signature(time);
        let new_val = (
            count.clone(),
            unit.map(|u| u.to_string()),
            sym.map(|s| format!("{:?}", s)),
        );
        let tracked = ctx.tracked_attrs().time_sig.get(&part_id).cloned();
        if emit_global && tracked.is_some() && tracked.as_ref() != Some(&new_val) {
            let mut metersig = tusk_model::elements::MeterSig::default();
            metersig.meter_sig_log.count = count.clone();
            metersig.meter_sig_log.unit = unit.map(|u| u.to_string());
            metersig.meter_sig_log.sym = sym;
            inline_children.push(LayerChild::MeterSig(Box::new(metersig)));
        }
        ctx.tracked_attrs_mut()
            .time_sig
            .insert(part_id.clone(), new_val);
    }

    // --- Clef changes (per-staff) ---
    for clef in &attrs.clefs {
        let local_staff = clef.number.unwrap_or(1);
        let matches_target = target_local_staff.is_none()
            || target_local_staff == Some(local_staff);
        // Only process clefs for the target staff — other staffs will process
        // their own clefs when called with their target_local_staff.
        if !matches_target {
            continue;
        }
        let new_val = (
            format!("{:?}", clef.sign),
            clef.line,
            clef.clef_octave_change,
        );
        let key = (part_id.clone(), local_staff);
        let tracked = ctx.tracked_attrs().clef.get(&key).cloned();
        if tracked.is_some() && tracked.as_ref() != Some(&new_val) {
            let (shape, line, dis, dis_place) = convert_clef_attributes(clef);
            let mut mei_clef = tusk_model::elements::Clef::default();
            mei_clef.clef_log.shape = shape;
            mei_clef.clef_log.line = line;
            mei_clef.clef_log.dis = dis;
            mei_clef.clef_log.dis_place = dis_place;
            mei_clef.event.staff = Some(local_staff.to_string());
            inline_children.push(LayerChild::Clef(Box::new(mei_clef)));
        }
        ctx.tracked_attrs_mut().clef.insert(key, new_val);
    }
}

// ============================================================================
// Beam Restructuring
// ============================================================================

/// Represents a beam group found in the notes.
struct BeamRange {
    start: usize,
    end: usize,
}

/// Check if a MusicXML note has a measured tremolo (start/stop/single) ornament.
fn note_has_tremolo(note: &crate::model::note::Note) -> bool {
    if let Some(ref notations) = note.notations {
        if let Some(ref ornaments) = notations.ornaments {
            if let Some(ref t) = ornaments.tremolo {
                return matches!(
                    t.tremolo_type,
                    crate::model::data::TremoloType::Start
                        | crate::model::data::TremoloType::Stop
                        | crate::model::data::TremoloType::Single
                );
            }
        }
    }
    false
}

fn detect_beam_groups(notes: &[&crate::model::note::Note]) -> Vec<BeamRange> {
    use crate::model::note::BeamValue;

    let mut groups = Vec::new();
    let mut beam_start: Option<usize> = None;
    let mut event_index = 0;
    let mut all_tremolo_in_group = true;

    for note in notes {
        if note.is_chord() {
            continue;
        }

        let has_begin = note
            .beams
            .iter()
            .any(|b| b.number.unwrap_or(1) == 1 && b.value == BeamValue::Begin);
        let has_end = note
            .beams
            .iter()
            .any(|b| b.number.unwrap_or(1) == 1 && b.value == BeamValue::End);

        if has_begin && beam_start.is_none() {
            beam_start = Some(event_index);
            all_tremolo_in_group = true;
        }

        if beam_start.is_some() && !note_has_tremolo(note) {
            all_tremolo_in_group = false;
        }

        if has_end {
            if let Some(start) = beam_start {
                // Suppress beam groups where all notes are tremolo —
                // the tremolo container (bTrem/fTrem) handles the visual grouping.
                if !all_tremolo_in_group {
                    groups.push(BeamRange {
                        start,
                        end: event_index,
                    });
                }
            }
            beam_start = None;
        }

        event_index += 1;
    }

    groups
}

/// Restructure layer children to wrap beamed elements in Beam containers.
pub fn restructure_with_beams(
    children: Vec<LayerChild>,
    notes: &[&crate::model::note::Note],
) -> Vec<LayerChild> {
    let groups = detect_beam_groups(notes);

    if groups.is_empty() {
        return children;
    }

    let mut result = Vec::new();
    let mut i = 0;
    let mut group_idx = 0;

    while i < children.len() {
        if group_idx < groups.len() && groups[group_idx].start == i {
            let group = &groups[group_idx];

            let mut beam = Beam::default();

            for j in group.start..=group.end {
                if j < children.len() {
                    let beam_child = layer_child_to_beam_child(&children[j]);
                    if let Some(bc) = beam_child {
                        beam.children.push(bc);
                    }
                }
            }

            if !beam.children.is_empty() {
                result.push(LayerChild::Beam(Box::new(beam)));
            }

            i = group.end + 1;
            group_idx += 1;
        } else {
            result.push(children[i].clone());
            i += 1;
        }
    }

    result
}

/// Convert a LayerChild to a BeamChild.
///
/// Returns None if the child type cannot be contained in a beam.
fn layer_child_to_beam_child(child: &LayerChild) -> Option<BeamChild> {
    match child {
        LayerChild::Note(n) => Some(BeamChild::Note(n.clone())),
        LayerChild::Chord(c) => Some(BeamChild::Chord(c.clone())),
        LayerChild::Rest(r) => Some(BeamChild::Rest(r.clone())),
        _ => None,
    }
}
