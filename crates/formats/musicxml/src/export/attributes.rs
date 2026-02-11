//! MEI scoreDef/staffDef to MusicXML Attributes conversion.
//!
//! This module handles the conversion of MEI score and staff definitions
//! to MusicXML attributes elements. It converts:
//! - Key signatures (keysig)
//! - Time signatures (meter.count, meter.unit, meter.sym)
//! - Clefs (clef.shape, clef.line, clef.dis, clef.dis.place)
//! - Transposition (trans.diat, trans.semi)
//! - Staff details (lines)

use crate::context::ConversionContext;
use crate::import::attributes::{
    FOR_PART_LABEL_PREFIX, KEY_LABEL_PREFIX, TIME_LABEL_PREFIX, extract_label_segment,
};
use crate::model::attributes::StaffDetails;
use tusk_model::elements::{ScoreDef, StaffDef};

/// Label prefix for staff-details JSON stored on staffDef @label.
const STAFF_DETAILS_LABEL_PREFIX: &str = "musicxml:staff-details,";

/// Label marker for Jianpu clef stored on staffDef @label.
const CLEF_JIANPU_LABEL: &str = "musicxml:clef-jianpu";

/// Extract MusicXML StaffDetails from ExtensionStore or MEI StaffDef label.
///
/// Preferred: read from ExtensionStore typed data.
/// Fallback 1: recover full StaffDetails from JSON in @label (lossless roundtrip).
/// Fallback 2: build minimal StaffDetails from @lines only.
fn extract_staff_details(
    staff_def: &StaffDef,
    ctx: &ConversionContext,
) -> Option<StaffDetails> {
    // Try ExtensionStore first
    if let Some(ref id) = staff_def.basic.xml_id {
        if let Some(ext) = ctx.ext_store().get(id) {
            if let Some(ref sde) = ext.staff_details_extras {
                if let Ok(sd) = serde_json::from_value::<StaffDetails>(sde.details.clone()) {
                    return Some(sd);
                }
            }
        }
    }

    // Fallback: try JSON label
    if let Some(ref label) = staff_def.labelled.label {
        if let Some(json) = extract_label_segment(label, STAFF_DETAILS_LABEL_PREFIX) {
            if let Ok(sd) = serde_json::from_str::<StaffDetails>(json) {
                return Some(sd);
            }
        }
    }

    // Fallback: build from @lines only
    let lines = staff_def
        .staff_def_log
        .lines
        .as_ref()
        .and_then(|s| s.parse::<u64>().ok());

    lines.map(|l| StaffDetails {
        staff_lines: Some(l as u32),
        ..Default::default()
    })
}

/// Extract MusicXML Key from ExtensionStore or MEI StaffDef label.
fn extract_key_from_label(
    staff_def: &StaffDef,
    ctx: &ConversionContext,
) -> Option<crate::model::attributes::Key> {
    // Try ExtensionStore first
    if let Some(ref id) = staff_def.basic.xml_id {
        if let Some(ext) = ctx.ext_store().get(id) {
            if let Some(ref ke) = ext.key_extras {
                if let Ok(key) =
                    serde_json::from_value::<crate::model::attributes::Key>(ke.key.clone())
                {
                    return Some(key);
                }
            }
        }
    }
    // Fallback: label
    let label = staff_def.labelled.label.as_ref()?;
    let json = extract_label_segment(label, KEY_LABEL_PREFIX)?;
    serde_json::from_str(json).ok()
}

/// Extract MusicXML Time from ExtensionStore or MEI StaffDef label.
fn extract_time_from_label(
    staff_def: &StaffDef,
    ctx: &ConversionContext,
) -> Option<crate::model::attributes::Time> {
    // Try ExtensionStore first
    if let Some(ref id) = staff_def.basic.xml_id {
        if let Some(ext) = ctx.ext_store().get(id) {
            if let Some(ref te) = ext.time_extras {
                if let Ok(time) =
                    serde_json::from_value::<crate::model::attributes::Time>(te.time.clone())
                {
                    return Some(time);
                }
            }
        }
    }
    // Fallback: label
    let label = staff_def.labelled.label.as_ref()?;
    let json = extract_label_segment(label, TIME_LABEL_PREFIX)?;
    serde_json::from_str(json).ok()
}

/// Extract ForPart vec from ExtensionStore or MEI StaffDef label.
fn extract_for_parts_from_label(
    staff_def: &StaffDef,
    ctx: &ConversionContext,
) -> Option<Vec<crate::model::attributes::ForPart>> {
    // Try ExtensionStore first
    if let Some(ref id) = staff_def.basic.xml_id {
        if let Some(ext) = ctx.ext_store().get(id) {
            if let Some(ref fpd) = ext.for_part {
                let entries: Vec<crate::model::attributes::ForPart> = fpd
                    .entries
                    .iter()
                    .filter_map(|v| serde_json::from_value(v.clone()).ok())
                    .collect();
                if !entries.is_empty() {
                    return Some(entries);
                }
            }
        }
    }
    // Fallback: label
    let label = staff_def.labelled.label.as_ref()?;
    let json = extract_label_segment(label, FOR_PART_LABEL_PREFIX)?;
    serde_json::from_str(json).ok()
}

/// Check if a MEI StaffDef has a Jianpu clef marker in its label.
fn has_jianpu_clef_label(staff_def: &StaffDef) -> bool {
    staff_def
        .labelled
        .label
        .as_deref()
        .is_some_and(|l| l.split('|').any(|seg| seg == CLEF_JIANPU_LABEL))
}

/// Convert MEI keysig attribute to MusicXML fifths value.
///
/// MEI uses format: "0" for no accidentals, "Ns" for N sharps, "Nf" for N flats.
/// MusicXML uses integer values: 0 for C major, positive for sharps, negative for flats.
///
/// # Examples
/// - "0" → 0 (C major)
/// - "2s" → 2 (D major)
/// - "3f" → -3 (Eb major)
pub fn convert_mei_keysig_to_fifths(keysig: &str) -> Option<i8> {
    let s = keysig;

    // Handle "0" (C major)
    if s == "0" {
        return Some(0);
    }

    // Handle "mixed" - no direct MusicXML equivalent
    if s == "mixed" {
        return None;
    }

    // Parse "Ns" (sharps) or "Nf" (flats)
    if let Some(num_str) = s.strip_suffix('s')
        && let Ok(n) = num_str.parse::<i8>()
    {
        return Some(n);
    }

    if let Some(num_str) = s.strip_suffix('f')
        && let Ok(n) = num_str.parse::<i8>()
    {
        return Some(-n);
    }

    None
}

/// Convert MEI meter.sym attribute to MusicXML TimeSymbol.
///
/// Maps MEI @meter.sym to MusicXML TimeSymbol.
/// - Common → TimeSymbol::Common
/// - Cut → TimeSymbol::Cut
/// - Open → None (handled as senza misura)
pub fn convert_mei_meter_sym_to_mxml(
    meter_sym: &tusk_model::data::DataMetersign,
) -> Option<crate::model::attributes::TimeSymbol> {
    use crate::model::attributes::TimeSymbol;
    use tusk_model::data::DataMetersign;

    match meter_sym {
        DataMetersign::Common => Some(TimeSymbol::Common),
        DataMetersign::Cut => Some(TimeSymbol::Cut),
        DataMetersign::Open => None, // Open is senza misura, handled separately
    }
}

/// Convert MEI @clef.shape to MusicXML ClefSign.
pub fn convert_mei_clef_shape_to_mxml(
    shape: &tusk_model::data::DataClefshape,
) -> crate::model::attributes::ClefSign {
    use crate::model::attributes::ClefSign;
    use tusk_model::data::DataClefshape;

    match shape {
        DataClefshape::G | DataClefshape::Gg => ClefSign::G,
        DataClefshape::F => ClefSign::F,
        DataClefshape::C => ClefSign::C,
        DataClefshape::Perc => ClefSign::Percussion,
        DataClefshape::Tab => ClefSign::Tab,
    }
}

/// Convert MEI octave displacement (@clef.dis + @clef.dis.place) to MusicXML octave-change.
pub(crate) fn convert_mei_clef_dis_to_octave_change(
    dis: Option<&tusk_model::data::DataOctaveDis>,
    dis_place: Option<&tusk_model::data::DataStaffrelBasic>,
) -> Option<i32> {
    use tusk_model::data::DataStaffrelBasic;

    let dis_value = dis?.0;
    let octaves = match dis_value {
        8 => 1,
        15 => 2,
        22 => 3,
        _ => return None,
    };

    let direction = dis_place.map_or(1, |place| match place {
        DataStaffrelBasic::Above => 1,
        DataStaffrelBasic::Below => -1,
    });

    Some(octaves * direction)
}

/// Convert MEI scoreDef to MusicXML Attributes.
///
/// This converts the following MEI attributes to MusicXML:
/// - keysig → key (fifths)
/// - meter.count, meter.unit, meter.sym → time (beats, beat-type, symbol)
/// - clef.shape, clef.line, clef.dis, clef.dis.place → clef (sign, line, clef-octave-change)
/// - divisions from context
///
/// # Arguments
///
/// * `score_def` - The MEI scoreDef to convert
/// * `ctx` - The conversion context for divisions and warnings
///
/// # Returns
///
/// A MusicXML Attributes element.
pub fn convert_mei_score_def_to_attributes(
    score_def: &ScoreDef,
    ctx: &mut ConversionContext,
) -> crate::model::attributes::Attributes {
    use crate::model::attributes::{
        Attributes, Clef, Key, KeyContent, SenzaMisura, StandardTime, Time, TimeContent,
        TimeSignature, TraditionalKey,
    };

    let mut attrs = Attributes::default();

    // Set divisions from context
    let divs = ctx.divisions();
    if divs > 0.0 {
        attrs.divisions = Some(divs);
    }

    // Convert key signature
    if let Some(keysig) = score_def.score_def_log.keysig.as_ref()
        && let Some(fifths) = convert_mei_keysig_to_fifths(keysig.0.as_str())
    {
        attrs.keys.push(Key {
            number: None,
            print_object: None,
            id: None,
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths,
                mode: None,
            }),
            key_octaves: Vec::new(),
        });
    }

    // Convert time signature
    if score_def.score_def_log.meter_sym.as_ref() == Some(&tusk_model::data::DataMetersign::Open) {
        // Senza misura
        attrs.times.push(Time {
            number: None,
            symbol: None,
            separator: None,
            print_object: None,
            id: None,
            content: TimeContent::SenzaMisura(SenzaMisura { symbol: None }),
        });
    } else if score_def.score_def_log.meter_count.is_some()
        || score_def.score_def_log.meter_unit.is_some()
    {
        let beats = score_def
            .score_def_log
            .meter_count
            .clone()
            .unwrap_or_else(|| "4".to_string());
        let beat_type = score_def
            .score_def_log
            .meter_unit
            .clone()
            .unwrap_or_else(|| "4".to_string());

        let symbol = score_def
            .score_def_log
            .meter_sym
            .as_ref()
            .and_then(convert_mei_meter_sym_to_mxml);

        attrs.times.push(Time {
            number: None,
            symbol,
            separator: None,
            print_object: None,
            id: None,
            content: TimeContent::Standard(StandardTime {
                signatures: vec![TimeSignature { beats, beat_type }],
                interchangeable: None,
            }),
        });
    }

    // Convert clef
    if let Some(shape) = &score_def.score_def_log.clef_shape {
        let sign = convert_mei_clef_shape_to_mxml(shape);
        let line = score_def
            .score_def_log
            .clef_line
            .as_ref()
            .map(|c| c.0 as u32);
        let octave_change = convert_mei_clef_dis_to_octave_change(
            score_def.score_def_log.clef_dis.as_ref(),
            score_def.score_def_log.clef_dis_place.as_ref(),
        );

        attrs.clefs.push(Clef {
            number: None,
            additional: None,
            size: None,
            after_barline: None,
            print_object: None,
            id: None,
            sign,
            line,
            clef_octave_change: octave_change,
        });
    }

    attrs
}

/// Convert MEI staffDef to MusicXML Attributes.
///
/// This converts the following MEI attributes to MusicXML:
/// - keysig → key (fifths)
/// - meter.count, meter.unit, meter.sym → time (beats, beat-type, symbol)
/// - clef.shape, clef.line, clef.dis, clef.dis.place → clef (sign, line, clef-octave-change)
/// - trans.diat, trans.semi → transpose (diatonic, chromatic)
/// - lines → staff-details (staff-lines)
///
/// # Arguments
///
/// * `staff_def` - The MEI staffDef to convert
/// * `ctx` - The conversion context for warnings
///
/// # Returns
///
/// A MusicXML Attributes element.
pub fn convert_mei_staff_def_to_attributes(
    staff_def: &StaffDef,
    ctx: &mut ConversionContext,
) -> crate::model::attributes::Attributes {
    use crate::model::attributes::{
        Attributes, Clef, Key, KeyContent, SenzaMisura, StandardTime, Time, TimeContent,
        TimeSignature, TraditionalKey, Transpose,
    };

    let mut attrs = Attributes::default();

    // Convert key signature — try JSON label first for lossless roundtrip
    if let Some(key) = extract_key_from_label(staff_def, ctx) {
        attrs.keys.push(key);
    } else if let Some(keysig) = staff_def.staff_def_log.keysig.as_ref()
        && let Some(fifths) = convert_mei_keysig_to_fifths(keysig.0.as_str())
    {
        attrs.keys.push(Key {
            number: None,
            print_object: None,
            id: None,
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths,
                mode: None,
            }),
            key_octaves: Vec::new(),
        });
    }

    // Convert time signature — try JSON label first for lossless roundtrip
    if let Some(time) = extract_time_from_label(staff_def, ctx) {
        attrs.times.push(time);
    } else if staff_def.staff_def_log.meter_sym.as_ref()
        == Some(&tusk_model::data::DataMetersign::Open)
    {
        // Senza misura
        attrs.times.push(Time {
            number: None,
            symbol: None,
            separator: None,
            print_object: None,
            id: None,
            content: TimeContent::SenzaMisura(SenzaMisura { symbol: None }),
        });
    } else if staff_def.staff_def_log.meter_count.is_some()
        || staff_def.staff_def_log.meter_unit.is_some()
    {
        let beats = staff_def
            .staff_def_log
            .meter_count
            .clone()
            .unwrap_or_else(|| "4".to_string());
        let beat_type = staff_def
            .staff_def_log
            .meter_unit
            .clone()
            .unwrap_or_else(|| "4".to_string());

        let symbol = staff_def
            .staff_def_log
            .meter_sym
            .as_ref()
            .and_then(convert_mei_meter_sym_to_mxml);

        attrs.times.push(Time {
            number: None,
            symbol,
            separator: None,
            print_object: None,
            id: None,
            content: TimeContent::Standard(StandardTime {
                signatures: vec![TimeSignature { beats, beat_type }],
                interchangeable: None,
            }),
        });
    }

    // Convert clef; MEI uses Option<String> for clef attributes
    if let Some(shape) = &staff_def.staff_def_log.clef_shape {
        let mut sign = convert_mei_clef_shape_to_mxml(shape);
        // Override with Jianpu if label indicates it (G in MEI but jianpu in MusicXML)
        if has_jianpu_clef_label(staff_def) {
            sign = crate::model::attributes::ClefSign::Jianpu;
        }
        let line = staff_def
            .staff_def_log
            .clef_line
            .as_ref()
            .map(|c| c.0 as u32);
        let octave_change = convert_mei_clef_dis_to_octave_change(
            staff_def.staff_def_log.clef_dis.as_ref(),
            staff_def.staff_def_log.clef_dis_place.as_ref(),
        );

        attrs.clefs.push(Clef {
            number: None,
            additional: None,
            size: None,
            after_barline: None,
            print_object: None,
            id: None,
            sign,
            line,
            clef_octave_change: octave_change,
        });
    }

    // Convert for-part (concert score per-part transposition) from label
    if let Some(for_parts) = extract_for_parts_from_label(staff_def, ctx) {
        attrs.for_parts = for_parts;
    }

    // Convert transposition; MEI @trans.semi and @trans.diat are Option<String>
    if staff_def.staff_def_log.trans_diat.is_some() || staff_def.staff_def_log.trans_semi.is_some()
    {
        let chromatic = staff_def
            .staff_def_log
            .trans_semi
            .as_ref()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0) as f64;
        let diatonic = staff_def
            .staff_def_log
            .trans_diat
            .as_ref()
            .and_then(|s| s.parse().ok());

        attrs.transposes.push(Transpose {
            number: None,
            id: None,
            diatonic,
            chromatic,
            octave_change: None,
            double: None,
        });
    }

    // Convert staff details from label JSON or fallback to @lines
    if let Some(sd) = extract_staff_details(staff_def, ctx) {
        attrs.staff_details.push(sd);
    }

    attrs
}

/// Build MusicXML Attributes for the first measure by merging scoreDef and staffDef.
///
/// - scoreDef provides: key signature, time signature (global)
/// - staffDef provides: clef, transposition, staff lines (per-staff)
pub fn build_first_measure_attributes(
    score_def: Option<&tusk_model::elements::ScoreDef>,
    staff_def: Option<&tusk_model::elements::StaffDef>,
    ctx: &mut ConversionContext,
) -> crate::model::attributes::Attributes {
    use crate::model::attributes::{
        Attributes, Clef, Key, KeyContent, SenzaMisura, StandardTime, Time, TimeContent,
        TimeSignature, TraditionalKey, Transpose,
    };

    let mut attrs = Attributes::default();

    // Set divisions from context
    let divisions = ctx.divisions();
    attrs.divisions = Some(divisions);

    // Key signature — try JSON label on staffDef for lossless roundtrip
    if let Some(key) = staff_def.and_then(|sd| extract_key_from_label(sd, ctx)) {
        attrs.keys.push(key);
    } else {
        let keysig = score_def
            .and_then(|sd| sd.score_def_log.keysig.as_ref())
            .or_else(|| staff_def.and_then(|sd| sd.staff_def_log.keysig.as_ref()));

        if let Some(keysig) = keysig
            && let Some(fifths) = convert_mei_keysig_to_fifths(keysig.0.as_str())
        {
            attrs.keys.push(Key {
                number: None,
                print_object: None,
                id: None,
                content: KeyContent::Traditional(TraditionalKey {
                    cancel: None,
                    fifths,
                    mode: None,
                }),
                key_octaves: Vec::new(),
            });
        }
    }

    // Time signature — try JSON label on staffDef for lossless roundtrip
    if let Some(time) = staff_def.and_then(|sd| extract_time_from_label(sd, ctx)) {
        attrs.times.push(time);
    } else {
        let meter_sym = score_def
            .and_then(|sd| sd.score_def_log.meter_sym.as_ref())
            .or_else(|| staff_def.and_then(|sd| sd.staff_def_log.meter_sym.as_ref()));
        let meter_count = score_def
            .and_then(|sd| sd.score_def_log.meter_count.as_ref())
            .or_else(|| staff_def.and_then(|sd| sd.staff_def_log.meter_count.as_ref()));
        let meter_unit = score_def
            .and_then(|sd| sd.score_def_log.meter_unit.as_ref().cloned())
            .or_else(|| staff_def.and_then(|sd| sd.staff_def_log.meter_unit.as_ref().cloned()));

        if meter_sym == Some(&tusk_model::data::DataMetersign::Open) {
            // Senza misura
            attrs.times.push(Time {
                number: None,
                symbol: None,
                separator: None,
                print_object: None,
                id: None,
                content: TimeContent::SenzaMisura(SenzaMisura { symbol: None }),
            });
        } else if meter_count.is_some() || meter_unit.is_some() {
            let beats = meter_count
                .map(|s| s.to_string())
                .unwrap_or_else(|| "4".to_string());
            let beat_type = meter_unit.unwrap_or_else(|| "4".to_string());

            let symbol = meter_sym
                .as_ref()
                .and_then(|s| convert_mei_meter_sym_to_mxml(s));

            attrs.times.push(Time {
                number: None,
                symbol,
                separator: None,
                print_object: None,
                id: None,
                content: TimeContent::Standard(StandardTime {
                    signatures: vec![TimeSignature { beats, beat_type }],
                    interchangeable: None,
                }),
            });
        }
    }

    // Get clef from staffDef (per-staff attribute)
    if let Some(staff_def) = staff_def {
        if let Some(shape) = &staff_def.staff_def_log.clef_shape {
            let mut sign = convert_mei_clef_shape_to_mxml(shape);
            // Override with Jianpu if label indicates it
            if has_jianpu_clef_label(staff_def) {
                sign = crate::model::attributes::ClefSign::Jianpu;
            }
            let line = staff_def
                .staff_def_log
                .clef_line
                .as_ref()
                .map(|c| c.0 as u32);

            // Convert octave displacement
            let octave_change = convert_mei_clef_dis_to_octave_change(
                staff_def.staff_def_log.clef_dis.as_ref(),
                staff_def.staff_def_log.clef_dis_place.as_ref(),
            );

            attrs.clefs.push(Clef {
                number: None,
                additional: None,
                size: None,
                after_barline: None,
                print_object: None,
                id: None,
                sign,
                line,
                clef_octave_change: octave_change,
            });
        }

        // Get for-part from label (concert score per-part transposition)
        if let Some(for_parts) = extract_for_parts_from_label(staff_def, ctx) {
            attrs.for_parts = for_parts;
        }

        // Get transposition from staffDef (MEI uses Option<String>)
        if staff_def.staff_def_log.trans_diat.is_some()
            || staff_def.staff_def_log.trans_semi.is_some()
        {
            let chromatic = staff_def
                .staff_def_log
                .trans_semi
                .as_ref()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0) as f64;
            let diatonic = staff_def
                .staff_def_log
                .trans_diat
                .as_ref()
                .and_then(|s| s.parse().ok())
                .map(|d: i32| d);

            attrs.transposes.push(Transpose {
                number: None,
                id: None,
                diatonic,
                chromatic,
                octave_change: None,
                double: None,
            });
        }

        // Get staff details from label JSON or fallback to @lines
        if let Some(sd) = extract_staff_details(staff_def, ctx) {
            attrs.staff_details.push(sd);
        }
    }

    attrs
}

/// Build MusicXML Attributes for a multi-staff part's first measure.
///
/// Like `build_first_measure_attributes()` but:
/// - Sets `staves = Some(num_staves)`
/// - Emits one clef per staff with `clef.number = Some(local_staff)`
/// - Key/time from scoreDef (shared across staves)
/// - Transposition and staff-details from first staffDef only
pub fn build_first_measure_attributes_multi(
    score_def: Option<&tusk_model::elements::ScoreDef>,
    part_id: &str,
    num_staves: u32,
    staff_defs: &[&tusk_model::elements::StaffDef],
    ctx: &mut ConversionContext,
) -> crate::model::attributes::Attributes {
    use crate::model::attributes::{
        Attributes, Clef, Key, KeyContent, SenzaMisura, StandardTime, Time, TimeContent,
        TimeSignature, TraditionalKey, Transpose,
    };

    // Find the staffDefs belonging to this part via context mapping
    let part_staff_defs: Vec<&tusk_model::elements::StaffDef> = (1..=num_staves)
        .filter_map(|local| {
            let global = ctx.global_staff_for_part(part_id, local)?;
            staff_defs
                .iter()
                .find(|sd| {
                    sd.n_integer.n.as_ref().and_then(|n| n.parse::<u32>().ok()) == Some(global)
                })
                .copied()
        })
        .collect();

    let first_def = part_staff_defs.first().copied();

    let mut attrs = Attributes {
        divisions: Some(ctx.divisions()),
        staves: Some(num_staves),
        part_symbol: ctx.part_symbol(part_id).cloned(),
        ..Default::default()
    };

    // Key signature — try JSON label on first staffDef for lossless roundtrip
    if let Some(key) = first_def.and_then(|sd| extract_key_from_label(sd, ctx)) {
        attrs.keys.push(key);
    } else {
        let keysig = score_def
            .and_then(|sd| sd.score_def_log.keysig.as_ref())
            .or_else(|| first_def.and_then(|sd| sd.staff_def_log.keysig.as_ref()));
        if let Some(keysig) = keysig
            && let Some(fifths) = convert_mei_keysig_to_fifths(keysig.0.as_str())
        {
            attrs.keys.push(Key {
                number: None,
                print_object: None,
                id: None,
                content: KeyContent::Traditional(TraditionalKey {
                    cancel: None,
                    fifths,
                    mode: None,
                }),
                key_octaves: Vec::new(),
            });
        }
    }

    // Time signature — try JSON label on first staffDef for lossless roundtrip
    if let Some(time) = first_def.and_then(|sd| extract_time_from_label(sd, ctx)) {
        attrs.times.push(time);
    } else {
        let meter_sym = score_def
            .and_then(|sd| sd.score_def_log.meter_sym.as_ref())
            .or_else(|| first_def.and_then(|sd| sd.staff_def_log.meter_sym.as_ref()));
        let meter_count = score_def
            .and_then(|sd| sd.score_def_log.meter_count.as_ref())
            .or_else(|| first_def.and_then(|sd| sd.staff_def_log.meter_count.as_ref()));
        let meter_unit = score_def
            .and_then(|sd| sd.score_def_log.meter_unit.as_ref().cloned())
            .or_else(|| first_def.and_then(|sd| sd.staff_def_log.meter_unit.as_ref().cloned()));

        if meter_sym == Some(&tusk_model::data::DataMetersign::Open) {
            attrs.times.push(Time {
                number: None,
                symbol: None,
                separator: None,
                print_object: None,
                id: None,
                content: TimeContent::SenzaMisura(SenzaMisura { symbol: None }),
            });
        } else if meter_count.is_some() || meter_unit.is_some() {
            let beats = meter_count
                .map(|s| s.to_string())
                .unwrap_or_else(|| "4".to_string());
            let beat_type = meter_unit.unwrap_or_else(|| "4".to_string());
            let symbol = meter_sym
                .as_ref()
                .and_then(|s| convert_mei_meter_sym_to_mxml(s));
            attrs.times.push(Time {
                number: None,
                symbol,
                separator: None,
                print_object: None,
                id: None,
                content: TimeContent::Standard(StandardTime {
                    signatures: vec![TimeSignature { beats, beat_type }],
                    interchangeable: None,
                }),
            });
        }
    }

    // Per-staff clefs with number attribute
    for (idx, staff_def) in part_staff_defs.iter().enumerate() {
        let local_staff = (idx + 1) as u32;
        if let Some(shape) = &staff_def.staff_def_log.clef_shape {
            let mut sign = convert_mei_clef_shape_to_mxml(shape);
            if has_jianpu_clef_label(staff_def) {
                sign = crate::model::attributes::ClefSign::Jianpu;
            }
            let line = staff_def
                .staff_def_log
                .clef_line
                .as_ref()
                .map(|c| c.0 as u32);
            let octave_change = convert_mei_clef_dis_to_octave_change(
                staff_def.staff_def_log.clef_dis.as_ref(),
                staff_def.staff_def_log.clef_dis_place.as_ref(),
            );
            attrs.clefs.push(Clef {
                number: Some(local_staff),
                additional: None,
                size: None,
                after_barline: None,
                print_object: None,
                id: None,
                sign,
                line,
                clef_octave_change: octave_change,
            });
        }
    }

    // For-part from first staffDef
    if let Some(for_parts) = first_def.and_then(|sd| extract_for_parts_from_label(sd, ctx)) {
        attrs.for_parts = for_parts;
    }

    // Transposition from first staffDef
    if let Some(staff_def) = first_def {
        if staff_def.staff_def_log.trans_diat.is_some()
            || staff_def.staff_def_log.trans_semi.is_some()
        {
            let chromatic = staff_def
                .staff_def_log
                .trans_semi
                .as_ref()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0) as f64;
            let diatonic = staff_def
                .staff_def_log
                .trans_diat
                .as_ref()
                .and_then(|s| s.parse().ok())
                .map(|d: i32| d);
            attrs.transposes.push(Transpose {
                number: None,
                id: None,
                diatonic,
                chromatic,
                octave_change: None,
                double: None,
            });
        }

        // Staff details from first staffDef (label JSON or @lines fallback)
        if let Some(sd) = extract_staff_details(staff_def, ctx) {
            attrs.staff_details.push(sd);
        }
    }

    attrs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ConversionDirection;
    use crate::model::attributes::KeyContent;

    #[test]
    fn test_convert_mei_keysig_to_mxml_fifths() {
        // C major (no accidentals)
        assert_eq!(convert_mei_keysig_to_fifths("0"), Some(0));

        // Sharp keys
        assert_eq!(convert_mei_keysig_to_fifths("1s"), Some(1)); // G major
        assert_eq!(convert_mei_keysig_to_fifths("2s"), Some(2)); // D major
        assert_eq!(convert_mei_keysig_to_fifths("3s"), Some(3)); // A major
        assert_eq!(convert_mei_keysig_to_fifths("7s"), Some(7)); // C# major

        // Flat keys
        assert_eq!(convert_mei_keysig_to_fifths("1f"), Some(-1)); // F major
        assert_eq!(convert_mei_keysig_to_fifths("2f"), Some(-2)); // Bb major
        assert_eq!(convert_mei_keysig_to_fifths("3f"), Some(-3)); // Eb major
        assert_eq!(convert_mei_keysig_to_fifths("7f"), Some(-7)); // Cb major

        // Invalid returns None
        assert_eq!(convert_mei_keysig_to_fifths("invalid"), None);
        assert_eq!(convert_mei_keysig_to_fifths("mixed"), None);
    }

    #[test]
    fn test_convert_mei_meter_sym_to_mxml_time() {
        use crate::model::attributes::TimeSymbol;
        use tusk_model::data::DataMetersign;

        assert_eq!(
            convert_mei_meter_sym_to_mxml(&DataMetersign::Common),
            Some(TimeSymbol::Common)
        );
        assert_eq!(
            convert_mei_meter_sym_to_mxml(&DataMetersign::Cut),
            Some(TimeSymbol::Cut)
        );
        assert_eq!(
            convert_mei_meter_sym_to_mxml(&DataMetersign::Open),
            None // Open/senza misura handled differently
        );
    }

    #[test]
    fn test_convert_mei_clef_shape_to_mxml() {
        use crate::model::attributes::ClefSign;
        use tusk_model::data::DataClefshape;

        assert_eq!(
            convert_mei_clef_shape_to_mxml(&DataClefshape::G),
            ClefSign::G
        );
        assert_eq!(
            convert_mei_clef_shape_to_mxml(&DataClefshape::F),
            ClefSign::F
        );
        assert_eq!(
            convert_mei_clef_shape_to_mxml(&DataClefshape::C),
            ClefSign::C
        );
        assert_eq!(
            convert_mei_clef_shape_to_mxml(&DataClefshape::Perc),
            ClefSign::Percussion
        );
        assert_eq!(
            convert_mei_clef_shape_to_mxml(&DataClefshape::Tab),
            ClefSign::Tab
        );
        assert_eq!(
            convert_mei_clef_shape_to_mxml(&DataClefshape::Gg),
            ClefSign::G
        ); // GG maps to G
    }

    #[test]
    fn test_convert_mei_score_def_to_mxml_attributes() {
        let mut score_def = ScoreDef::default();

        // Set key signature (D major = 2 sharps)
        score_def.score_def_log.keysig = Some(tusk_model::data::DataKeyfifths("2s".to_string()));

        // Set time signature (4/4 common time)
        score_def.score_def_log.meter_count = Some("4".to_string());
        score_def.score_def_log.meter_unit = Some("4".to_string());
        score_def.score_def_log.meter_sym = Some(tusk_model::data::DataMetersign::Common);

        // Set clef (treble clef)
        score_def.score_def_log.clef_shape = Some(tusk_model::data::DataClefshape::G);
        score_def.score_def_log.clef_line = Some(tusk_model::data::DataClefline::from(2));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let attrs = convert_mei_score_def_to_attributes(&score_def, &mut ctx);

        // Check key signature
        assert_eq!(attrs.keys.len(), 1);
        if let KeyContent::Traditional(trad) = &attrs.keys[0].content {
            assert_eq!(trad.fifths, 2);
        } else {
            panic!("Expected traditional key");
        }

        // Check time signature
        assert_eq!(attrs.times.len(), 1);
        assert_eq!(
            attrs.times[0].symbol,
            Some(crate::model::attributes::TimeSymbol::Common)
        );
        if let crate::model::attributes::TimeContent::Standard(std) = &attrs.times[0].content {
            assert_eq!(std.signatures[0].beats, "4");
            assert_eq!(std.signatures[0].beat_type, "4");
        } else {
            panic!("Expected standard time");
        }

        // Check clef
        assert_eq!(attrs.clefs.len(), 1);
        assert_eq!(attrs.clefs[0].sign, crate::model::attributes::ClefSign::G);
        assert_eq!(attrs.clefs[0].line, Some(2));
    }

    #[test]
    fn test_convert_mei_staff_def_to_mxml_attributes() {
        let mut staff_def = StaffDef::default();

        // Set key signature (Bb major = 2 flats)
        staff_def.staff_def_log.keysig = Some(tusk_model::data::DataKeyfifths("2f".to_string()));

        // Set time signature (3/4)
        staff_def.staff_def_log.meter_count = Some("3".to_string());
        staff_def.staff_def_log.meter_unit = Some("4".to_string());

        // Set clef (bass clef)
        staff_def.staff_def_log.clef_shape = Some(tusk_model::data::DataClefshape::F);
        staff_def.staff_def_log.clef_line = Some(tusk_model::data::DataClefline::from(4));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let attrs = convert_mei_staff_def_to_attributes(&staff_def, &mut ctx);

        // Check key signature
        assert_eq!(attrs.keys.len(), 1);
        if let KeyContent::Traditional(trad) = &attrs.keys[0].content {
            assert_eq!(trad.fifths, -2);
        } else {
            panic!("Expected traditional key");
        }

        // Check time signature
        assert_eq!(attrs.times.len(), 1);
        if let crate::model::attributes::TimeContent::Standard(std) = &attrs.times[0].content {
            assert_eq!(std.signatures[0].beats, "3");
            assert_eq!(std.signatures[0].beat_type, "4");
        } else {
            panic!("Expected standard time");
        }

        // Check clef
        assert_eq!(attrs.clefs.len(), 1);
        assert_eq!(attrs.clefs[0].sign, crate::model::attributes::ClefSign::F);
        assert_eq!(attrs.clefs[0].line, Some(4));
    }

    #[test]
    fn test_convert_mei_clef_with_octave_displacement() {
        use tusk_model::data::{DataClefline, DataClefshape, DataOctaveDis, DataStaffrelBasic};

        let mut staff_def = StaffDef::default();

        // Set treble clef 8vb (tenor voice)
        staff_def.staff_def_log.clef_shape = Some(DataClefshape::G);
        staff_def.staff_def_log.clef_line = Some(DataClefline::from(2));
        staff_def.staff_def_log.clef_dis = Some(DataOctaveDis::from(8));
        staff_def.staff_def_log.clef_dis_place = Some(DataStaffrelBasic::Below);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let attrs = convert_mei_staff_def_to_attributes(&staff_def, &mut ctx);

        assert_eq!(attrs.clefs.len(), 1);
        assert_eq!(attrs.clefs[0].sign, crate::model::attributes::ClefSign::G);
        assert_eq!(attrs.clefs[0].line, Some(2));
        assert_eq!(attrs.clefs[0].clef_octave_change, Some(-1));
    }

    #[test]
    fn test_convert_mei_clef_8va() {
        use tusk_model::data::{DataClefline, DataClefshape, DataOctaveDis, DataStaffrelBasic};

        let mut staff_def = StaffDef::default();

        // Set treble clef 8va
        staff_def.staff_def_log.clef_shape = Some(DataClefshape::G);
        staff_def.staff_def_log.clef_line = Some(DataClefline::from(2));
        staff_def.staff_def_log.clef_dis = Some(DataOctaveDis::from(8));
        staff_def.staff_def_log.clef_dis_place = Some(DataStaffrelBasic::Above);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let attrs = convert_mei_staff_def_to_attributes(&staff_def, &mut ctx);

        assert_eq!(attrs.clefs.len(), 1);
        assert_eq!(attrs.clefs[0].clef_octave_change, Some(1));
    }

    #[test]
    fn test_convert_mei_score_def_with_divisions() {
        let score_def = ScoreDef::default();

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let attrs = convert_mei_score_def_to_attributes(&score_def, &mut ctx);

        // Divisions should be included in attributes
        assert_eq!(attrs.divisions, Some(4.0));
    }

    #[test]
    fn test_convert_mei_score_def_empty() {
        let score_def = ScoreDef::default();
        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);

        let attrs = convert_mei_score_def_to_attributes(&score_def, &mut ctx);

        // All should be empty/None
        assert!(attrs.keys.is_empty());
        assert!(attrs.times.is_empty());
        assert!(attrs.clefs.is_empty());
    }

    #[test]
    fn test_convert_mei_staff_def_with_transposition() {
        let mut staff_def = StaffDef::default();

        // Set transposition for Bb clarinet (sounds M2 lower)
        staff_def.staff_def_log.trans_diat = Some("-1".to_string());
        staff_def.staff_def_log.trans_semi = Some("-2".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let attrs = convert_mei_staff_def_to_attributes(&staff_def, &mut ctx);

        assert_eq!(attrs.transposes.len(), 1);
        assert_eq!(attrs.transposes[0].diatonic, Some(-1));
        assert_eq!(attrs.transposes[0].chromatic, -2.0);
    }

    #[test]
    fn test_convert_mei_staff_def_with_staff_lines() {
        let mut staff_def = StaffDef::default();

        // Guitar tab has 6 lines
        staff_def.staff_def_log.lines = Some("6".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let attrs = convert_mei_staff_def_to_attributes(&staff_def, &mut ctx);

        assert_eq!(attrs.staff_details.len(), 1);
        assert_eq!(attrs.staff_details[0].staff_lines, Some(6));
    }

    #[test]
    fn test_convert_mei_senza_misura() {
        use tusk_model::data::DataMetersign;

        let mut score_def = ScoreDef::default();
        score_def.score_def_log.meter_sym = Some(DataMetersign::Open);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let attrs = convert_mei_score_def_to_attributes(&score_def, &mut ctx);

        assert_eq!(attrs.times.len(), 1);
        if let crate::model::attributes::TimeContent::SenzaMisura(_) = &attrs.times[0].content {
            // Success
        } else {
            panic!("Expected senza misura");
        }
    }

    #[test]
    fn test_convert_mei_compound_meter() {
        let mut score_def = ScoreDef::default();

        // 6/8 time
        score_def.score_def_log.meter_count = Some("6".to_string());
        score_def.score_def_log.meter_unit = Some("8".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let attrs = convert_mei_score_def_to_attributes(&score_def, &mut ctx);

        assert_eq!(attrs.times.len(), 1);
        if let crate::model::attributes::TimeContent::Standard(std) = &attrs.times[0].content {
            assert_eq!(std.signatures[0].beats, "6");
            assert_eq!(std.signatures[0].beat_type, "8");
        } else {
            panic!("Expected standard time");
        }
    }

    #[test]
    fn test_convert_mei_additive_meter() {
        let mut score_def = ScoreDef::default();

        // 3+2/8 additive meter
        score_def.score_def_log.meter_count = Some("3+2".to_string());
        score_def.score_def_log.meter_unit = Some("8".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let attrs = convert_mei_score_def_to_attributes(&score_def, &mut ctx);

        assert_eq!(attrs.times.len(), 1);
        if let crate::model::attributes::TimeContent::Standard(std) = &attrs.times[0].content {
            assert_eq!(std.signatures[0].beats, "3+2");
            assert_eq!(std.signatures[0].beat_type, "8");
        } else {
            panic!("Expected standard time");
        }
    }
}
