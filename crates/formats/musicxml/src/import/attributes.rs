//! Attributes conversion: key, time, clef, and divisions.
//!
//! This module handles conversion of MusicXML attributes elements to MEI.
//! Attributes define key signatures, time signatures, clefs, and divisions.

use crate::context::ConversionContext;
use crate::model::attributes::{
    Clef, ClefSign, Key, KeyContent, Mode, Time, TimeContent, TimeSymbol,
};
use tusk_model::data::{
    DataClefline, DataClefshape, DataKeyfifths, DataMetersign, DataOctaveDis, DataStaffrelBasic,
};
use tusk_model::elements::StaffDef;
use tusk_model::musicxml_ext::{DoubleData, ForPartData, KeyExtras, TimeExtras, TransposeData};

/// Label prefix for non-traditional key JSON stored on staffDef @label.
pub const KEY_LABEL_PREFIX: &str = "musicxml:key,";

/// Label prefix for interchangeable/complex time JSON stored on staffDef @label.
pub const TIME_LABEL_PREFIX: &str = "musicxml:time,";

/// Label prefix for for-part JSON stored on staffDef @label.
pub const FOR_PART_LABEL_PREFIX: &str = "musicxml:for-part,";

/// Label prefix for transpose JSON stored on staffDef @label.
pub const TRANSPOSE_LABEL_PREFIX: &str = "musicxml:transpose,";

/// Append a prefixed JSON segment to a staffDef label, using `|` separator.
pub fn append_label(staff_def: &mut StaffDef, segment: String) {
    match &mut staff_def.labelled.label {
        Some(existing) => {
            existing.push('|');
            existing.push_str(&segment);
        }
        None => {
            staff_def.labelled.label = Some(segment);
        }
    }
}

/// Extract a label segment by prefix from a `|`-separated label string.
pub fn extract_label_segment<'a>(label: &'a str, prefix: &str) -> Option<&'a str> {
    for segment in label.split('|') {
        if let Some(json) = segment.strip_prefix(prefix) {
            return Some(json);
        }
    }
    None
}

// ============================================================================
// Attributes Conversion (Key, Time, Clef)
// ============================================================================

/// Convert MusicXML key fifths to MEI keysig data type.
///
/// MusicXML uses `<fifths>` with integer values (-7 to 7).
/// MEI uses `@keysig` with format: "0" for no accidentals, "Ns" for N sharps, "Nf" for N flats.
///
/// # Examples
/// - 0 → "0" (C major / A minor)
/// - 2 → "2s" (D major / B minor)
/// - -3 → "3f" (Eb major / C minor)
pub fn convert_key_fifths(fifths: i8) -> DataKeyfifths {
    if fifths == 0 {
        DataKeyfifths("0".to_string())
    } else if fifths > 0 {
        DataKeyfifths(format!("{}s", fifths))
    } else {
        DataKeyfifths(format!("{}f", -fifths))
    }
}

/// Convert MusicXML key signature to update the conversion context.
///
/// This updates the context's key signature state for accidental tracking.
/// The key signature affects how accidentals are determined for subsequent notes.
pub fn convert_key_to_context(key: &Key, ctx: &mut ConversionContext) {
    if let KeyContent::Traditional(trad) = &key.content {
        let mode_str = trad.mode.as_ref().map(|m| match m {
            Mode::Major => "major".to_string(),
            Mode::Minor => "minor".to_string(),
            Mode::Dorian => "dorian".to_string(),
            Mode::Phrygian => "phrygian".to_string(),
            Mode::Lydian => "lydian".to_string(),
            Mode::Mixolydian => "mixolydian".to_string(),
            Mode::Aeolian => "aeolian".to_string(),
            Mode::Ionian => "ionian".to_string(),
            Mode::Locrian => "locrian".to_string(),
            Mode::None => "none".to_string(),
            Mode::Other(s) => s.clone(),
        });
        ctx.set_key_signature(trad.fifths, mode_str);
    }
}

/// Convert MusicXML time signature to MEI meter attributes.
///
/// Returns (meter_count, meter_unit, meter_sym):
/// - meter_count: The top number (beats per measure), may contain expressions like "3+2"
/// - meter_unit: The bottom number (beat unit) as f64
/// - meter_sym: Optional meter symbol (common time, cut time)
///
/// # Examples
/// - Time::new("4", "4") → (Some("4"), Some(4.0), None)
/// - Time::common() → (Some("4"), Some(4.0), Some(DataMetersign::Common))
/// - Time::cut() → (Some("2"), Some(2.0), Some(DataMetersign::Cut))
pub fn convert_time_signature(time: &Time) -> (Option<String>, Option<f64>, Option<DataMetersign>) {
    let meter_sym = time.symbol.as_ref().and_then(|s| match s {
        TimeSymbol::Common => Some(DataMetersign::Common),
        TimeSymbol::Cut => Some(DataMetersign::Cut),
        // Other symbols don't have direct MEI equivalents - map to None
        _ => None,
    });

    match &time.content {
        TimeContent::Standard(std) => {
            if let Some(sig) = std.signatures.first() {
                let count = Some(sig.beats.clone());
                let unit = sig.beat_type.parse::<f64>().ok();
                (count, unit, meter_sym)
            } else {
                (None, None, meter_sym)
            }
        }
        TimeContent::SenzaMisura(_) => {
            // Senza misura: no meter
            (None, None, Some(DataMetersign::Open))
        }
    }
}

/// Convert MusicXML clef to MEI clef attributes.
///
/// Returns (clef_shape, clef_line, clef_dis, clef_dis_place):
/// - clef_shape: The clef symbol (G, F, C, perc, TAB)
/// - clef_line: The staff line (1-based from bottom)
/// - clef_dis: Octave displacement amount (8, 15, 22) if transposing clef
/// - clef_dis_place: Direction of displacement (above, below)
///
/// # Examples
/// - Clef::treble() → (G, 2, None, None)
/// - Clef::bass() → (F, 4, None, None)
/// - Clef::treble_8vb() → (G, 2, Some(8), Some(below))
pub fn convert_clef_attributes(
    clef: &Clef,
) -> (
    Option<DataClefshape>,
    Option<DataClefline>,
    Option<DataOctaveDis>,
    Option<DataStaffrelBasic>,
) {
    let shape = Some(match clef.sign {
        ClefSign::G => DataClefshape::G,
        ClefSign::F => DataClefshape::F,
        ClefSign::C => DataClefshape::C,
        ClefSign::Percussion => DataClefshape::Perc,
        ClefSign::Tab => DataClefshape::Tab,
        ClefSign::Jianpu => DataClefshape::G, // No direct equivalent, default to G
        ClefSign::None => return (None, None, None, None),
    });

    let line = clef.line.map(|l| DataClefline(l as u64));

    // Handle octave displacement
    let (dis, dis_place) = match clef.clef_octave_change {
        Some(change) if change != 0 => {
            let amount = change.unsigned_abs() as u64;
            // MEI uses 8, 15, 22 for 1, 2, 3 octaves
            let dis_value = amount * 7 + 1; // 1→8, 2→15, 3→22
            let dis = Some(DataOctaveDis(dis_value));
            let place = if change > 0 {
                Some(DataStaffrelBasic::Above)
            } else {
                Some(DataStaffrelBasic::Below)
            };
            (dis, place)
        }
        _ => (None, None),
    };

    (shape, line, dis, dis_place)
}

/// Process MusicXML attributes element and update context and optional staffDef.
///
/// This function handles:
/// - divisions: Updates the duration context
/// - keys: Updates context key signature and optionally staffDef keysig
/// - times: Optionally updates staffDef meter attributes
/// - clefs: Optionally updates staffDef clef attributes
///
/// # Arguments
/// * `attrs` - The MusicXML attributes to process
/// * `ctx` - The conversion context to update
/// * `staff_def` - Optional StaffDef to update with the attributes
pub fn process_attributes(
    attrs: &crate::model::attributes::Attributes,
    ctx: &mut ConversionContext,
    mut staff_def: Option<&mut StaffDef>,
) {
    // Update divisions
    if let Some(divs) = attrs.divisions {
        ctx.set_divisions(divs);
    }

    // Process key signatures
    for key in &attrs.keys {
        // Update context state
        convert_key_to_context(key, ctx);

        // Update staffDef if provided
        if let Some(sd) = staff_def.as_deref_mut() {
            match &key.content {
                KeyContent::Traditional(trad) => {
                    let keysig = convert_key_fifths(trad.fifths);
                    sd.staff_def_log.keysig = Some(keysig);
                    // Store full Key as JSON if it has key_octaves (extra data beyond keysig)
                    if !key.key_octaves.is_empty() {
                        if let Ok(json) = serde_json::to_string(key) {
                            append_label(sd, format!("{}{}", KEY_LABEL_PREFIX, json));
                            if let Some(ref id) = sd.basic.xml_id {
                                ctx.ext_store_mut().insert_key_extras(
                                    id.clone(),
                                    KeyExtras {
                                        key: serde_json::to_value(key).unwrap_or_default(),
                                    },
                                );
                            }
                        }
                    }
                }
                KeyContent::NonTraditional(_) => {
                    // No MEI @keysig equivalent; store full Key as JSON
                    if let Ok(json) = serde_json::to_string(key) {
                        append_label(sd, format!("{}{}", KEY_LABEL_PREFIX, json));
                        if let Some(ref id) = sd.basic.xml_id {
                            ctx.ext_store_mut().insert_key_extras(
                                id.clone(),
                                KeyExtras {
                                    key: serde_json::to_value(key).unwrap_or_default(),
                                },
                            );
                        }
                    }
                }
            }
        }
    }

    // Process time signatures
    for time in &attrs.times {
        if let Some(sd) = staff_def.as_deref_mut() {
            let (count, unit, sym) = convert_time_signature(time);
            sd.staff_def_log.meter_count = count;
            sd.staff_def_log.meter_unit = unit.map(|u| u.to_string());
            sd.staff_def_log.meter_sym = sym;

            // Store full Time as JSON if it has interchangeable or separator (extra data)
            let has_extra = matches!(&time.content, TimeContent::Standard(std) if std.interchangeable.is_some())
                || time.separator.is_some();
            if has_extra {
                if let Ok(json) = serde_json::to_string(time) {
                    append_label(sd, format!("{}{}", TIME_LABEL_PREFIX, json));
                    if let Some(ref id) = sd.basic.xml_id {
                        ctx.ext_store_mut().insert_time_extras(
                            id.clone(),
                            TimeExtras {
                                time: serde_json::to_value(time).unwrap_or_default(),
                            },
                        );
                    }
                }
            }
        }
    }

    // Process clefs
    for clef in &attrs.clefs {
        if let Some(sd) = staff_def.as_deref_mut() {
            let (shape, line, dis, dis_place) = convert_clef_attributes(clef);
            sd.staff_def_log.clef_shape = shape;
            sd.staff_def_log.clef_line = line;
            sd.staff_def_log.clef_dis = dis;
            sd.staff_def_log.clef_dis_place = dis_place;

            // Store Jianpu clef in label for lossless roundtrip (mapped to G in MEI)
            if clef.sign == ClefSign::Jianpu {
                append_label(sd, "musicxml:clef-jianpu".to_string());
            }
        }
    }

    // Process transposition
    if !attrs.transposes.is_empty() {
        if let Some(sd) = staff_def.as_deref_mut() {
            // Apply the first transpose to MEI @trans.semi / @trans.diat
            let t = &attrs.transposes[0];

            // Effective chromatic = chromatic + octave_change * 12
            let effective_semi = t.chromatic as i32 + t.octave_change.unwrap_or(0) * 12;
            sd.staff_def_log.trans_semi = Some(effective_semi.to_string());

            if let Some(diat) = t.diatonic {
                let effective_diat = diat + t.octave_change.unwrap_or(0) * 7;
                sd.staff_def_log.trans_diat = Some(effective_diat.to_string());
            }

            // Store full TransposeData for lossless roundtrip (octave_change, double, number, id)
            let td = TransposeData {
                number: t.number,
                diatonic: t.diatonic,
                chromatic: t.chromatic,
                octave_change: t.octave_change,
                double: t.double.as_ref().map(|d| DoubleData {
                    above: d.above.map(|v| v == crate::model::data::YesNo::Yes),
                }),
                id: t.id.clone(),
            };
            if let Ok(json) = serde_json::to_string(&td) {
                append_label(sd, format!("{}{}", TRANSPOSE_LABEL_PREFIX, json));
            }
            if let Some(ref id) = sd.basic.xml_id {
                ctx.ext_store_mut().insert_transpose(id.clone(), td);
            }
        }
    }

    // Process for-part (concert score per-part transposition)
    if !attrs.for_parts.is_empty() {
        if let Some(sd) = staff_def {
            if let Ok(json) = serde_json::to_string(&attrs.for_parts) {
                append_label(sd, format!("{}{}", FOR_PART_LABEL_PREFIX, json));
                if let Some(ref id) = sd.basic.xml_id {
                    ctx.ext_store_mut().insert_for_part(
                        id.clone(),
                        ForPartData {
                            entries: attrs
                                .for_parts
                                .iter()
                                .filter_map(|fp| serde_json::to_value(fp).ok())
                                .collect(),
                        },
                    );
                }
            }
        }
    }
}
