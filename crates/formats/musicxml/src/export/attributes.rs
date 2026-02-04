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
use tusk_model::elements::{ScoreDef, StaffDef};

/// Convert MEI keysig attribute to MusicXML fifths value.
///
/// MEI uses format: "0" for no accidentals, "Ns" for N sharps, "Nf" for N flats.
/// MusicXML uses integer values: 0 for C major, positive for sharps, negative for flats.
///
/// # Examples
/// - "0" → 0 (C major)
/// - "2s" → 2 (D major)
/// - "3f" → -3 (Eb major)
pub fn convert_mei_keysig_to_fifths(keysig: &tusk_model::data::DataKeyfifths) -> Option<i8> {
    let s = keysig.0.as_str();

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
/// Maps:
/// - DataMetersign::Common → TimeSymbol::Common
/// - DataMetersign::Cut → TimeSymbol::Cut
/// - DataMetersign::Open → None (handled as senza misura)
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

/// Convert MEI clef.shape attribute to MusicXML ClefSign.
///
/// Maps:
/// - G → G
/// - GG → G (double G clef maps to G)
/// - F → F
/// - C → C
/// - perc → percussion
/// - TAB → TAB
pub fn convert_mei_clef_shape_to_mxml(
    shape: &tusk_model::data::DataClefshape,
) -> crate::model::attributes::ClefSign {
    use crate::model::attributes::ClefSign;
    use tusk_model::data::DataClefshape;

    match shape {
        DataClefshape::G => ClefSign::G,
        DataClefshape::Gg => ClefSign::G, // Double G maps to G
        DataClefshape::F => ClefSign::F,
        DataClefshape::C => ClefSign::C,
        DataClefshape::Perc => ClefSign::Percussion,
        DataClefshape::Tab => ClefSign::Tab,
    }
}

/// Convert MEI octave displacement (clef.dis + clef.dis.place) to MusicXML octave-change.
///
/// MEI uses:
/// - clef.dis: 8, 15, 22 for 1, 2, 3 octaves
/// - clef.dis.place: "above" or "below"
///
/// MusicXML uses:
/// - clef-octave-change: positive for up, negative for down
fn convert_mei_clef_dis_to_octave_change(
    dis: Option<&tusk_model::data::DataOctaveDis>,
    dis_place: Option<&tusk_model::data::DataStaffrelBasic>,
) -> Option<i32> {
    use tusk_model::data::DataStaffrelBasic;

    let dis_value = dis?;
    let octaves = match dis_value.0 {
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
pub fn convert_score_def_to_attributes(
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
    if let Some(keysig) = score_def.score_def_log.keysig.first()
        && let Some(fifths) = convert_mei_keysig_to_fifths(keysig)
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
    if score_def.score_def_log.meter_sym == Some(tusk_model::data::DataMetersign::Open) {
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
            .map(|u| format!("{}", u as i32))
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
            .map(|l| l.0 as u32);
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
pub fn convert_staff_def_to_attributes(
    staff_def: &StaffDef,
    _ctx: &mut ConversionContext,
) -> crate::model::attributes::Attributes {
    use crate::model::attributes::{
        Attributes, Clef, Key, KeyContent, SenzaMisura, StaffDetails, StandardTime, Time,
        TimeContent, TimeSignature, TraditionalKey, Transpose,
    };

    let mut attrs = Attributes::default();

    // Convert key signature
    if let Some(keysig) = staff_def.staff_def_log.keysig.first()
        && let Some(fifths) = convert_mei_keysig_to_fifths(keysig)
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
    if staff_def.staff_def_log.meter_sym == Some(tusk_model::data::DataMetersign::Open) {
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
            .map(|u| format!("{}", u as i32))
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

    // Convert clef
    if let Some(shape) = &staff_def.staff_def_log.clef_shape {
        let sign = convert_mei_clef_shape_to_mxml(shape);
        let line = staff_def
            .staff_def_log
            .clef_line
            .as_ref()
            .map(|l| l.0 as u32);
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

    // Convert transposition
    if staff_def.staff_def_log.trans_diat.is_some() || staff_def.staff_def_log.trans_semi.is_some()
    {
        let chromatic = staff_def.staff_def_log.trans_semi.unwrap_or(0) as f64;
        let diatonic = staff_def.staff_def_log.trans_diat.map(|d| d as i32);

        attrs.transposes.push(Transpose {
            number: None,
            id: None,
            diatonic,
            chromatic,
            octave_change: None,
            double: None,
        });
    }

    // Convert staff lines
    if let Some(lines) = staff_def.staff_def_log.lines {
        attrs.staff_details.push(StaffDetails {
            number: None,
            show_frets: None,
            print_object: None,
            print_spacing: None,
            staff_type: None,
            staff_lines: Some(lines as u32),
            line_details: Vec::new(),
            staff_tunings: Vec::new(),
            capo: None,
            staff_size: None,
        });
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
        use tusk_model::data::DataKeyfifths;

        // C major (no accidentals)
        assert_eq!(
            convert_mei_keysig_to_fifths(&DataKeyfifths("0".to_string())),
            Some(0)
        );

        // Sharp keys
        assert_eq!(
            convert_mei_keysig_to_fifths(&DataKeyfifths("1s".to_string())),
            Some(1)
        ); // G major
        assert_eq!(
            convert_mei_keysig_to_fifths(&DataKeyfifths("2s".to_string())),
            Some(2)
        ); // D major
        assert_eq!(
            convert_mei_keysig_to_fifths(&DataKeyfifths("3s".to_string())),
            Some(3)
        ); // A major
        assert_eq!(
            convert_mei_keysig_to_fifths(&DataKeyfifths("7s".to_string())),
            Some(7)
        ); // C# major

        // Flat keys
        assert_eq!(
            convert_mei_keysig_to_fifths(&DataKeyfifths("1f".to_string())),
            Some(-1)
        ); // F major
        assert_eq!(
            convert_mei_keysig_to_fifths(&DataKeyfifths("2f".to_string())),
            Some(-2)
        ); // Bb major
        assert_eq!(
            convert_mei_keysig_to_fifths(&DataKeyfifths("3f".to_string())),
            Some(-3)
        ); // Eb major
        assert_eq!(
            convert_mei_keysig_to_fifths(&DataKeyfifths("7f".to_string())),
            Some(-7)
        ); // Cb major

        // Invalid returns None
        assert_eq!(
            convert_mei_keysig_to_fifths(&DataKeyfifths("invalid".to_string())),
            None
        );
        assert_eq!(
            convert_mei_keysig_to_fifths(&DataKeyfifths("mixed".to_string())),
            None
        );
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
        use tusk_model::data::{DataClefline, DataClefshape, DataKeyfifths, DataMetersign};

        let mut score_def = ScoreDef::default();

        // Set key signature (D major = 2 sharps)
        score_def.score_def_log.keysig = vec![DataKeyfifths("2s".to_string())];

        // Set time signature (4/4 common time)
        score_def.score_def_log.meter_count = Some("4".to_string());
        score_def.score_def_log.meter_unit = Some(4.0);
        score_def.score_def_log.meter_sym = Some(DataMetersign::Common);

        // Set clef (treble clef)
        score_def.score_def_log.clef_shape = Some(DataClefshape::G);
        score_def.score_def_log.clef_line = Some(DataClefline(2));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let attrs = convert_score_def_to_attributes(&score_def, &mut ctx);

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
        use tusk_model::data::{DataClefline, DataClefshape, DataKeyfifths};

        let mut staff_def = StaffDef::default();

        // Set key signature (Bb major = 2 flats)
        staff_def.staff_def_log.keysig = vec![DataKeyfifths("2f".to_string())];

        // Set time signature (3/4)
        staff_def.staff_def_log.meter_count = Some("3".to_string());
        staff_def.staff_def_log.meter_unit = Some(4.0);

        // Set clef (bass clef)
        staff_def.staff_def_log.clef_shape = Some(DataClefshape::F);
        staff_def.staff_def_log.clef_line = Some(DataClefline(4));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let attrs = convert_staff_def_to_attributes(&staff_def, &mut ctx);

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
        staff_def.staff_def_log.clef_line = Some(DataClefline(2));
        staff_def.staff_def_log.clef_dis = Some(DataOctaveDis(8));
        staff_def.staff_def_log.clef_dis_place = Some(DataStaffrelBasic::Below);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let attrs = convert_staff_def_to_attributes(&staff_def, &mut ctx);

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
        staff_def.staff_def_log.clef_line = Some(DataClefline(2));
        staff_def.staff_def_log.clef_dis = Some(DataOctaveDis(8));
        staff_def.staff_def_log.clef_dis_place = Some(DataStaffrelBasic::Above);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let attrs = convert_staff_def_to_attributes(&staff_def, &mut ctx);

        assert_eq!(attrs.clefs.len(), 1);
        assert_eq!(attrs.clefs[0].clef_octave_change, Some(1));
    }

    #[test]
    fn test_convert_mei_score_def_with_divisions() {
        let score_def = ScoreDef::default();

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let attrs = convert_score_def_to_attributes(&score_def, &mut ctx);

        // Divisions should be included in attributes
        assert_eq!(attrs.divisions, Some(4.0));
    }

    #[test]
    fn test_convert_mei_score_def_empty() {
        let score_def = ScoreDef::default();
        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);

        let attrs = convert_score_def_to_attributes(&score_def, &mut ctx);

        // All should be empty/None
        assert!(attrs.keys.is_empty());
        assert!(attrs.times.is_empty());
        assert!(attrs.clefs.is_empty());
    }

    #[test]
    fn test_convert_mei_staff_def_with_transposition() {
        let mut staff_def = StaffDef::default();

        // Set transposition for Bb clarinet (sounds M2 lower)
        staff_def.staff_def_log.trans_diat = Some(-1);
        staff_def.staff_def_log.trans_semi = Some(-2);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let attrs = convert_staff_def_to_attributes(&staff_def, &mut ctx);

        assert_eq!(attrs.transposes.len(), 1);
        assert_eq!(attrs.transposes[0].diatonic, Some(-1));
        assert_eq!(attrs.transposes[0].chromatic, -2.0);
    }

    #[test]
    fn test_convert_mei_staff_def_with_staff_lines() {
        let mut staff_def = StaffDef::default();

        // Guitar tab has 6 lines
        staff_def.staff_def_log.lines = Some(6);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let attrs = convert_staff_def_to_attributes(&staff_def, &mut ctx);

        assert_eq!(attrs.staff_details.len(), 1);
        assert_eq!(attrs.staff_details[0].staff_lines, Some(6));
    }

    #[test]
    fn test_convert_mei_senza_misura() {
        use tusk_model::data::DataMetersign;

        let mut score_def = ScoreDef::default();
        score_def.score_def_log.meter_sym = Some(DataMetersign::Open);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let attrs = convert_score_def_to_attributes(&score_def, &mut ctx);

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
        score_def.score_def_log.meter_unit = Some(8.0);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let attrs = convert_score_def_to_attributes(&score_def, &mut ctx);

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
        score_def.score_def_log.meter_unit = Some(8.0);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let attrs = convert_score_def_to_attributes(&score_def, &mut ctx);

        assert_eq!(attrs.times.len(), 1);
        if let crate::model::attributes::TimeContent::Standard(std) = &attrs.times[0].content {
            assert_eq!(std.signatures[0].beats, "3+2");
            assert_eq!(std.signatures[0].beat_type, "8");
        } else {
            panic!("Expected standard time");
        }
    }
}
