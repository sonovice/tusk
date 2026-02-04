//! MEI to MusicXML conversion.
//!
//! This module provides conversion from MEI documents to MusicXML score-partwise
//! format. The conversion is lossy - many MEI-specific features have no MusicXML
//! equivalent and will be lost (see `docs/conversion-notes.md` for details).
//!
//! # Conversion Overview
//!
//! MEI structure maps to MusicXML as follows:
//! - MEI `<meiHead>` → MusicXML header (work, identification)
//! - MEI `<score>/<scoreDef>/<staffGrp>` → MusicXML `<part-list>`
//! - MEI `<section>/<measure>/<staff>/<layer>` → MusicXML `<part>/<measure>`
//!
//! # Example
//!
//! ```ignore
//! use tusk_convert::mei_to_musicxml::convert_mei;
//! use tusk_model::Mei;
//!
//! let mei = Mei::default();
//! let musicxml = convert_mei(&mei)?;
//! ```

use crate::context::{ConversionContext, ConversionDirection};
use crate::error::ConversionResult;
use tusk_model::data::DataBoolean;
use tusk_model::elements::{
    Body, BodyChild, Mdiv, MdivChild, Mei, MeiChild, MeiHead, MeiHeadChild, Music, Score,
    ScoreChild, ScoreDef, ScoreDefChild, StaffDef, StaffDefChild, StaffGrp, StaffGrpChild,
};
use tusk_musicxml::model::data::YesNo;
use tusk_musicxml::model::elements::{
    Encoding, Identification, Measure, Part, PartList, PartListItem, PartName, ScorePart,
    ScorePartwise, Work,
};

/// Convert an MEI document to MusicXML score-partwise.
///
/// This is the main entry point for MEI → MusicXML conversion.
/// The conversion creates a MusicXML score-partwise document with:
/// - Header from MEI `<meiHead>` (work, identification, encoding)
/// - Part list from MEI `<scoreDef>/<staffGrp>`
/// - Parts with measures from MEI `<section>/<measure>/<staff>/<layer>`
///
/// # Arguments
///
/// * `mei` - The MEI document to convert
///
/// # Returns
///
/// A MusicXML score-partwise document, or an error if conversion fails.
///
/// # Lossy Conversion
///
/// This conversion is lossy. MEI-specific features without MusicXML equivalents
/// will generate warnings in the context. Check `ctx.warnings()` after conversion.
pub fn convert_mei(mei: &Mei) -> ConversionResult<ScorePartwise> {
    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    convert_mei_with_context(mei, &mut ctx)
}

/// Convert an MEI document to MusicXML with an existing context.
///
/// This variant allows reusing a conversion context across multiple conversions,
/// which is useful for batch processing or when custom context configuration is needed.
pub fn convert_mei_with_context(
    mei: &Mei,
    ctx: &mut ConversionContext,
) -> ConversionResult<ScorePartwise> {
    let mut score = ScorePartwise {
        version: Some("4.0".to_string()),
        ..Default::default()
    };

    // Extract meiHead and music from children
    let mut mei_head: Option<&MeiHead> = None;
    let mut music: Option<&Music> = None;

    for child in &mei.children {
        match child {
            MeiChild::MeiHead(h) => mei_head = Some(h),
            MeiChild::Music(m) => music = Some(m),
        }
    }

    // Convert header (metadata)
    if let Some(head) = mei_head {
        let (work, identification) = convert_header(head, ctx)?;
        score.work = work;
        score.identification = identification;
    }

    // Convert music content (requires body/mdiv/score structure)
    if let Some(music_elem) = music
        && let Some(body) = find_body_in_music(music_elem)
        && let Some(mdiv) = find_first_mdiv_in_body(body)
        && let Some(mei_score) = find_score_in_mdiv(mdiv)
    {
        // Convert scoreDef to part-list
        if let Some(score_def) = find_score_def(mei_score) {
            score.part_list = convert_part_list(score_def, ctx)?;
        } else {
            // No scoreDef, create minimal part-list
            score.part_list = PartList::default();
        }

        // Create parts (will be populated in later tasks)
        // For now, create empty parts matching the part-list
        score.parts = create_empty_parts(&score.part_list);
    }

    // If no parts were created, ensure part_list has at least one part
    if score.part_list.items.is_empty() {
        // Create a minimal part-list with one part
        let default_part = ScorePart::new("P1", "Part 1");
        score
            .part_list
            .items
            .push(PartListItem::ScorePart(Box::new(default_part)));
        score.parts.push(Part::new("P1"));
    }

    Ok(score)
}

/// Convert MEI meiHead to MusicXML header elements.
///
/// Returns (Work, Identification) tuple.
fn convert_header(
    mei_head: &MeiHead,
    ctx: &mut ConversionContext,
) -> ConversionResult<(Option<Work>, Option<Identification>)> {
    let mut work: Option<Work> = None;
    let mut identification = Identification::default();

    // Find fileDesc to extract title
    for child in &mei_head.children {
        if let MeiHeadChild::FileDesc(file_desc) = child {
            // Extract title from fileDesc -> titleStmt -> title
            if let Some(title) = extract_title_from_file_desc(file_desc) {
                work = Some(Work {
                    work_title: Some(title),
                    ..Default::default()
                });
            }
        }
    }

    // Add encoding info showing conversion from MEI
    let encoding = Encoding {
        software: vec!["Tusk MusicXML-MEI Converter".to_string()],
        ..Default::default()
    };
    identification.encoding = Some(encoding);

    // Add warning about potential lossy conversion
    ctx.add_warning(
        "meiHead",
        "MEI metadata may be simplified during conversion to MusicXML",
    );

    Ok((work, Some(identification)))
}

/// Extract title text from MEI fileDesc.
fn extract_title_from_file_desc(file_desc: &tusk_model::elements::FileDesc) -> Option<String> {
    use tusk_model::elements::{FileDescChild, TitleChild, TitleStmtChild};

    for child in &file_desc.children {
        if let FileDescChild::TitleStmt(title_stmt) = child {
            for ts_child in &title_stmt.children {
                if let TitleStmtChild::Title(title) = ts_child {
                    // Collect text content from title children
                    let mut text = String::new();
                    for title_child in &title.children {
                        if let TitleChild::Text(t) = title_child {
                            text.push_str(t);
                        }
                    }
                    if !text.is_empty() {
                        return Some(text);
                    }
                }
            }
        }
    }
    None
}

/// Find the Body element in Music.
///
/// Note: Due to code generation limitations, Body might not be directly
/// accessible as a child of Music. This function handles that case.
fn find_body_in_music(_music: &Music) -> Option<&Body> {
    // The generated Music type doesn't include Body as a direct child variant.
    // In actual MEI documents, body is a child of music, but the code generator
    // only included certain children. For now, return None and handle this
    // limitation - the full document structure will need special handling.
    //
    // TODO: Update code generator to include body as a Music child, or use
    // a separate parsing path for the complete document structure.
    None
}

/// Find the first Mdiv in a Body.
fn find_first_mdiv_in_body(body: &Body) -> Option<&Mdiv> {
    for child in &body.children {
        if let BodyChild::Mdiv(mdiv) = child {
            return Some(mdiv);
        }
    }
    None
}

/// Find the Score element in an Mdiv.
fn find_score_in_mdiv(mdiv: &Mdiv) -> Option<&Score> {
    for child in &mdiv.children {
        if let MdivChild::Score(score) = child {
            return Some(score);
        }
    }
    None
}

/// Find the ScoreDef in a Score.
fn find_score_def(score: &Score) -> Option<&ScoreDef> {
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child {
            return Some(score_def);
        }
    }
    None
}

/// Convert MEI scoreDef to MusicXML part-list.
///
/// Maps:
/// - MEI `<staffGrp>` → MusicXML `<part-group>` (start/stop)
/// - MEI `<staffDef>` → MusicXML `<score-part>`
/// - MEI `<label>` → MusicXML `<part-name>`
/// - MEI `<labelAbbr>` → MusicXML `<part-abbreviation>`
fn convert_part_list(
    score_def: &ScoreDef,
    ctx: &mut ConversionContext,
) -> ConversionResult<PartList> {
    let mut part_list = PartList::default();

    // Find staffGrp in scoreDef children
    for child in &score_def.children {
        if let ScoreDefChild::StaffGrp(staff_grp) = child {
            convert_staff_grp_to_part_list(staff_grp, &mut part_list, ctx, 1)?;
        }
    }

    Ok(part_list)
}

/// Recursively convert MEI staffGrp to MusicXML part-list items.
///
/// Returns the next available group number.
fn convert_staff_grp_to_part_list(
    staff_grp: &StaffGrp,
    part_list: &mut PartList,
    ctx: &mut ConversionContext,
    group_num: u32,
) -> ConversionResult<u32> {
    use tusk_musicxml::model::data::StartStop;
    use tusk_musicxml::model::elements::PartGroup;

    let mut current_group_num = group_num;
    let has_children = !staff_grp.children.is_empty();

    // Check if this staffGrp has multiple children that need grouping
    let needs_group = has_children && staff_grp.children.len() > 1
        || staff_grp.staff_grp_vis.symbol.is_some()
        || staff_grp.staff_grp_vis.bar_thru.is_some();

    // Emit part-group start if needed
    if needs_group {
        let part_group = PartGroup {
            group_type: StartStop::Start,
            number: Some(current_group_num.to_string()),
            group_name: extract_label_text(staff_grp),
            group_name_display: None,
            group_abbreviation: extract_label_abbr_text(staff_grp),
            group_abbreviation_display: None,
            group_symbol: convert_staff_grp_symbol(staff_grp),
            group_barline: convert_staff_grp_barline(staff_grp),
            group_time: None,
        };
        part_list
            .items
            .push(PartListItem::PartGroup(Box::new(part_group)));
        current_group_num += 1;
    }

    // Process children
    for child in &staff_grp.children {
        match child {
            StaffGrpChild::StaffDef(staff_def) => {
                let score_part = convert_staff_def_to_score_part(staff_def, ctx)?;
                part_list
                    .items
                    .push(PartListItem::ScorePart(Box::new(score_part)));
            }
            StaffGrpChild::StaffGrp(nested_grp) => {
                current_group_num =
                    convert_staff_grp_to_part_list(nested_grp, part_list, ctx, current_group_num)?;
            }
            StaffGrpChild::Label(_) | StaffGrpChild::LabelAbbr(_) => {
                // Already handled above
            }
            _ => {
                // Other children not mapped to MusicXML
                ctx.add_warning(
                    "staffGrp",
                    "Some staffGrp child elements have no MusicXML equivalent",
                );
            }
        }
    }

    // Emit part-group stop if needed
    if needs_group {
        let stop_group = PartGroup {
            group_type: StartStop::Stop,
            number: Some((group_num).to_string()), // Use the start group number
            group_name: None,
            group_name_display: None,
            group_abbreviation: None,
            group_abbreviation_display: None,
            group_symbol: None,
            group_barline: None,
            group_time: None,
        };
        part_list
            .items
            .push(PartListItem::PartGroup(Box::new(stop_group)));
    }

    Ok(current_group_num)
}

/// Extract label text from staffGrp children.
fn extract_label_text(staff_grp: &StaffGrp) -> Option<String> {
    use tusk_model::elements::LabelChild;

    for child in &staff_grp.children {
        if let StaffGrpChild::Label(label) = child {
            let mut text = String::new();
            for label_child in &label.children {
                if let LabelChild::Text(t) = label_child {
                    text.push_str(t);
                }
            }
            if !text.is_empty() {
                return Some(text);
            }
        }
    }
    None
}

/// Extract labelAbbr text from staffGrp children.
fn extract_label_abbr_text(staff_grp: &StaffGrp) -> Option<String> {
    use tusk_model::elements::LabelAbbrChild;

    for child in &staff_grp.children {
        if let StaffGrpChild::LabelAbbr(label_abbr) = child {
            let mut text = String::new();
            for label_child in &label_abbr.children {
                if let LabelAbbrChild::Text(t) = label_child {
                    text.push_str(t);
                }
            }
            if !text.is_empty() {
                return Some(text);
            }
        }
    }
    None
}

/// Convert MEI staffGrp @symbol to MusicXML group-symbol.
fn convert_staff_grp_symbol(
    staff_grp: &StaffGrp,
) -> Option<tusk_musicxml::model::elements::GroupSymbolValue> {
    use tusk_model::att::AttStaffGrpVisSymbol;
    use tusk_musicxml::model::elements::{GroupSymbol, GroupSymbolValue};

    staff_grp.staff_grp_vis.symbol.as_ref().map(|sym| {
        let value = match sym {
            AttStaffGrpVisSymbol::Brace => GroupSymbol::Brace,
            AttStaffGrpVisSymbol::Bracket => GroupSymbol::Bracket,
            AttStaffGrpVisSymbol::Bracketsq => GroupSymbol::Square,
            AttStaffGrpVisSymbol::Line => GroupSymbol::Line,
            AttStaffGrpVisSymbol::None => GroupSymbol::None,
        };
        GroupSymbolValue {
            value,
            default_x: None,
            relative_x: None,
            color: None,
        }
    })
}

/// Convert MEI staffGrp @bar.thru to MusicXML group-barline.
fn convert_staff_grp_barline(
    staff_grp: &StaffGrp,
) -> Option<tusk_musicxml::model::elements::GroupBarlineValue> {
    use tusk_model::data::DataBoolean;
    use tusk_musicxml::model::elements::{GroupBarline, GroupBarlineValue};

    staff_grp.staff_grp_vis.bar_thru.as_ref().map(|bar_thru| {
        let value = match bar_thru {
            DataBoolean::True => GroupBarline::Yes,
            DataBoolean::False => GroupBarline::No,
        };
        GroupBarlineValue { value, color: None }
    })
}

/// Convert MEI staffDef to MusicXML score-part.
fn convert_staff_def_to_score_part(
    staff_def: &StaffDef,
    ctx: &mut ConversionContext,
) -> ConversionResult<ScorePart> {
    // Generate part ID from staff number or xml:id
    let part_id = staff_def
        .basic
        .xml_id
        .clone()
        .or_else(|| staff_def.n_integer.n.map(|n| format!("P{}", n)))
        .unwrap_or_else(|| ctx.generate_id_with_suffix("part"));

    // Extract label (part name) from staffDef children
    let part_name = extract_staff_def_label(staff_def).unwrap_or_default();

    let mut score_part = ScorePart::new(&part_id, &part_name);

    // Extract labelAbbr (part abbreviation)
    if let Some(abbr) = extract_staff_def_label_abbr(staff_def) {
        score_part.part_abbreviation = Some(PartName {
            value: abbr,
            ..Default::default()
        });
    }

    // Map MEI staffDef ID to MusicXML part ID
    if let Some(ref xml_id) = staff_def.basic.xml_id {
        ctx.map_id(xml_id.clone(), part_id.clone());
    }

    Ok(score_part)
}

/// Extract label text from staffDef children.
fn extract_staff_def_label(staff_def: &StaffDef) -> Option<String> {
    use tusk_model::elements::LabelChild;

    for child in &staff_def.children {
        if let StaffDefChild::Label(label) = child {
            let mut text = String::new();
            for label_child in &label.children {
                if let LabelChild::Text(t) = label_child {
                    text.push_str(t);
                }
            }
            if !text.is_empty() {
                return Some(text);
            }
        }
    }
    None
}

/// Extract labelAbbr text from staffDef children.
fn extract_staff_def_label_abbr(staff_def: &StaffDef) -> Option<String> {
    use tusk_model::elements::LabelAbbrChild;

    for child in &staff_def.children {
        if let StaffDefChild::LabelAbbr(label_abbr) = child {
            let mut text = String::new();
            for label_child in &label_abbr.children {
                if let LabelAbbrChild::Text(t) = label_child {
                    text.push_str(t);
                }
            }
            if !text.is_empty() {
                return Some(text);
            }
        }
    }
    None
}

/// Create empty Part elements matching the part-list.
fn create_empty_parts(part_list: &PartList) -> Vec<Part> {
    let mut parts = Vec::new();

    for item in &part_list.items {
        if let PartListItem::ScorePart(score_part) = item {
            parts.push(Part::new(&score_part.id));
        }
    }

    parts
}

// ============================================================================
// MEI Measure → MusicXML Measure Conversion
// ============================================================================

/// Convert an MEI measure to a MusicXML measure.
///
/// This converts the structural attributes of an MEI measure to MusicXML:
/// - MEI `@n` → MusicXML `@number` (measure number/label)
/// - MEI `@metcon="false"` → MusicXML `@implicit="yes"` (pickup/incomplete measure)
/// - MEI `@control="false"` → MusicXML `@non-controlling="yes"` (non-controlling barline)
/// - MEI `@width` → MusicXML `@width` (measure width)
/// - MEI `xml:id` → MusicXML `@id` (element ID)
///
/// Note: This function converts the measure attributes only. The measure content
/// (notes, rests, etc.) will be converted by subsequent functions in Phase 4.4.
///
/// # Arguments
///
/// * `mei_measure` - The MEI measure to convert
/// * `part_id` - The MusicXML part ID this measure belongs to
/// * `ctx` - The conversion context for state tracking
///
/// # Returns
///
/// A MusicXML Measure element, or an error if conversion fails.
pub fn convert_mei_measure(
    mei_measure: &tusk_model::elements::Measure,
    _part_id: &str,
    ctx: &mut ConversionContext,
) -> ConversionResult<Measure> {
    // Create MusicXML measure with number
    // Use @n if present, otherwise generate a measure number
    let measure_number = mei_measure
        .common
        .n
        .as_ref()
        .map(|n| n.to_string())
        .unwrap_or_else(|| ctx.generate_id_with_suffix("measure"));

    let mut mxml_measure = Measure::new(&measure_number);

    // Convert xml:id to id
    if let Some(ref xml_id) = mei_measure.common.xml_id {
        mxml_measure.id = Some(xml_id.clone());
        // Map the ID in context
        ctx.map_id(xml_id, xml_id.clone());
    }

    // Convert metcon="false" → implicit="yes"
    // In MEI, metcon="false" means the measure content doesn't conform to the meter
    // In MusicXML, implicit="yes" means the measure doesn't count in measure numbering
    if let Some(DataBoolean::False) = mei_measure.measure_log.metcon {
        mxml_measure.implicit = Some(YesNo::Yes);
    }

    // Convert control="false" → non_controlling="yes"
    // In MEI, control="false" means the right barline doesn't indicate alignment
    // In MusicXML, non_controlling="yes" is used for measures in multi-rest regions
    if let Some(DataBoolean::False) = mei_measure.measure_log.control {
        mxml_measure.non_controlling = Some(YesNo::Yes);
    }

    // Convert width
    // MEI @width is in DataMeasurementunsigned format (e.g., "200vu")
    // MusicXML @width is a floating point number in tenths
    if let Some(ref width) = mei_measure.measure_vis.width
        && let Some(numeric_width) = parse_mei_measurement(width)
    {
        mxml_measure.width = Some(numeric_width);
    }

    // Note: Measure content (staff/layer/note/rest) conversion will be implemented
    // in subsequent tasks (convert MEI note, rest, chord to MusicXML)

    Ok(mxml_measure)
}

/// Parse an MEI measurement value (e.g., "200vu", "100", "50.5vu") to f64.
///
/// MEI measurements can include units like "vu" (virtual units), "pt" (points),
/// etc. This function extracts the numeric value, discarding the unit suffix.
fn parse_mei_measurement(measurement: &tusk_model::data::DataMeasurementunsigned) -> Option<f64> {
    let s = measurement.to_string();

    // Try to parse as a simple number first
    if let Ok(val) = s.parse::<f64>() {
        return Some(val);
    }

    // Try to extract numeric prefix (handle "200vu", "100pt", etc.)
    let numeric_part: String = s
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    numeric_part.parse::<f64>().ok()
}

// ============================================================================
// MEI Note → MusicXML Note Conversion
// ============================================================================

/// Convert an MEI note to a MusicXML note.
///
/// This converts an MEI note element to MusicXML, including:
/// - Pitch (pname, oct → step, octave; accid.ges → alter)
/// - Duration (dur → type; dots → dot elements; calculated duration in divisions)
/// - Grace notes (@grace → grace element)
/// - Written accidentals (accid child → accidental element)
/// - Stem direction (@stem.dir → stem element)
/// - Cue notes (@cue → cue element)
/// - IDs (xml:id → id attribute)
///
/// # Lossy Conversion Notes
///
/// The following MEI attributes are lost in conversion:
/// - Analytical attributes (@pclass, @deg, etc.) - no MusicXML equivalent
/// - Gestural attributes other than accid.ges - limited MusicXML support
/// - Visual attributes beyond stem direction - partial support
/// - Editorial child elements (app, choice, etc.) - no MusicXML equivalent
///
/// # Arguments
///
/// * `mei_note` - The MEI note to convert
/// * `ctx` - The conversion context for state tracking
///
/// # Returns
///
/// A MusicXML Note element, or an error if conversion fails.
pub fn convert_mei_note(
    mei_note: &tusk_model::elements::Note,
    ctx: &mut ConversionContext,
) -> ConversionResult<tusk_musicxml::model::note::Note> {
    use tusk_musicxml::model::elements::Empty;
    use tusk_musicxml::model::note::{Dot, Note as MxmlNote, NoteType, Stem};

    // Build the MusicXML note
    let mut mxml_note: MxmlNote;

    // Determine if this is a grace note (grace notes have no duration)
    let is_grace = mei_note.note_log.grace.is_some();

    // Convert pitch from MEI (pname, oct) to MusicXML (step, octave, alter)
    let pitch = convert_mei_pitch(mei_note, ctx)?;

    if is_grace {
        // Grace note: no duration
        let grace = convert_mei_grace(mei_note);
        mxml_note = MxmlNote::grace_note(pitch, grace);
    } else {
        // Regular note: calculate duration
        let duration = calculate_mei_note_duration(mei_note, ctx);
        mxml_note = MxmlNote::pitched(pitch, duration);
    }

    // Set ID from xml:id
    if let Some(ref xml_id) = mei_note.common.xml_id {
        mxml_note.id = Some(xml_id.clone());
        ctx.map_id(xml_id, xml_id.clone());
    }

    // Convert note type (graphical duration)
    if let Some(ref dur) = mei_note.note_log.dur {
        mxml_note.note_type = Some(NoteType::new(convert_mei_duration_to_note_type(dur)));
    }

    // Convert dots
    if let Some(ref dots) = mei_note.note_log.dots {
        let dot_count = dots.to_string().parse::<u64>().unwrap_or(0);
        for _ in 0..dot_count {
            mxml_note.dots.push(Dot::default());
        }
    }

    // Convert written accidental from child element
    for child in &mei_note.children {
        if let tusk_model::elements::NoteChild::Accid(accid) = child {
            mxml_note.accidental = Some(convert_mei_accid_to_mxml(accid, ctx)?);
        }
    }

    // Convert stem direction
    if let Some(ref stem_dir) = mei_note.note_vis.stem_dir {
        mxml_note.stem = Some(Stem::new(convert_mei_stem_direction(stem_dir)));
    }

    // Convert cue note
    if let Some(DataBoolean::True) = mei_note.note_log.cue {
        mxml_note.cue = Some(Empty);
    }

    // Add warnings for lossy attributes
    add_note_conversion_warnings(mei_note, ctx);

    Ok(mxml_note)
}

/// Convert MEI pitch attributes to MusicXML Pitch.
fn convert_mei_pitch(
    mei_note: &tusk_model::elements::Note,
    _ctx: &mut ConversionContext,
) -> ConversionResult<tusk_musicxml::model::note::Pitch> {
    use tusk_musicxml::model::data::Step;
    use tusk_musicxml::model::note::Pitch;

    // Get pitch name (pname)
    let step = if let Some(ref pname) = mei_note.note_log.pname {
        convert_mei_pname_to_step(pname)?
    } else {
        // Default to C if not specified (shouldn't happen in valid MEI)
        Step::C
    };

    // Get octave
    let octave = if let Some(ref oct) = mei_note.note_log.oct {
        oct.to_string().parse::<u8>().unwrap_or(4)
    } else {
        4 // Default octave
    };

    // Get chromatic alteration from gestural accidental
    let alter = convert_mei_gestural_accid_to_alter(&mei_note.note_ges.accid_ges);

    Ok(Pitch {
        step,
        alter,
        octave,
    })
}

/// Convert MEI pitch name to MusicXML Step.
fn convert_mei_pname_to_step(
    pname: &tusk_model::data::DataPitchname,
) -> ConversionResult<tusk_musicxml::model::data::Step> {
    use tusk_musicxml::model::data::Step;

    let name = pname.to_string().to_lowercase();
    match name.as_str() {
        "a" => Ok(Step::A),
        "b" => Ok(Step::B),
        "c" => Ok(Step::C),
        "d" => Ok(Step::D),
        "e" => Ok(Step::E),
        "f" => Ok(Step::F),
        "g" => Ok(Step::G),
        _ => {
            // Return C as fallback for invalid pitch names
            Ok(Step::C)
        }
    }
}

/// Convert MEI gestural accidental to MusicXML alter value.
fn convert_mei_gestural_accid_to_alter(
    accid_ges: &Option<tusk_model::data::DataAccidentalGestural>,
) -> Option<f64> {
    use tusk_model::data::{DataAccidentalGestural, DataAccidentalGesturalBasic};

    accid_ges.as_ref().map(|ges| match ges {
        DataAccidentalGestural::DataAccidentalGesturalBasic(basic) => match basic {
            DataAccidentalGesturalBasic::Tf => -3.0, // Triple flat
            DataAccidentalGesturalBasic::Ff => -2.0, // Double flat
            DataAccidentalGesturalBasic::F => -1.0,  // Flat
            DataAccidentalGesturalBasic::N => 0.0,   // Natural
            DataAccidentalGesturalBasic::S => 1.0,   // Sharp
            DataAccidentalGesturalBasic::Ss => 2.0,  // Double sharp
            DataAccidentalGesturalBasic::Ts => 3.0,  // Triple sharp
        },
        // For extended gestural accidentals, return 0 as fallback
        _ => 0.0,
    })
}

/// Calculate MEI note duration in MusicXML divisions.
fn calculate_mei_note_duration(
    mei_note: &tusk_model::elements::Note,
    ctx: &ConversionContext,
) -> f64 {
    // First check if we have gestural duration in ppq (most accurate)
    if let Some(dur_ppq) = mei_note.note_ges.dur_ppq {
        return dur_ppq as f64;
    }

    // Calculate from written duration
    let divisions = ctx.divisions();
    let base_duration = if let Some(ref dur) = mei_note.note_log.dur {
        duration_to_quarter_notes(dur)
    } else {
        1.0 // Default to quarter note
    };

    // Apply dots
    let dot_count = mei_note
        .note_log
        .dots
        .as_ref()
        .map(|d| d.to_string().parse::<u64>().unwrap_or(0))
        .unwrap_or(0);

    let dotted_duration = apply_dots(base_duration, dot_count);

    // Convert to divisions
    dotted_duration * divisions
}

/// Convert MEI duration to quarter note units.
fn duration_to_quarter_notes(dur: &tusk_model::data::DataDuration) -> f64 {
    use tusk_model::data::{DataDuration, DataDurationCmn};

    match dur {
        DataDuration::DataDurationCmn(cmn) => match cmn {
            DataDurationCmn::Long => 16.0,    // Long = 4 whole notes
            DataDurationCmn::Breve => 8.0,    // Breve = 2 whole notes
            DataDurationCmn::N1 => 4.0,       // Whole = 4 quarters
            DataDurationCmn::N2 => 2.0,       // Half = 2 quarters
            DataDurationCmn::N4 => 1.0,       // Quarter
            DataDurationCmn::N8 => 0.5,       // Eighth
            DataDurationCmn::N16 => 0.25,     // 16th
            DataDurationCmn::N32 => 0.125,    // 32nd
            DataDurationCmn::N64 => 0.0625,   // 64th
            DataDurationCmn::N128 => 0.03125, // 128th
            DataDurationCmn::N256 => 0.015625,
            DataDurationCmn::N512 => 0.0078125,
            DataDurationCmn::N1024 => 0.00390625,
            DataDurationCmn::N2048 => 0.001953125, // 2048th note
        },
        // For mensural durations, return quarter note as fallback
        _ => 1.0,
    }
}

/// Apply augmentation dots to a duration.
fn apply_dots(base_duration: f64, dots: u64) -> f64 {
    let mut duration = base_duration;
    let mut dot_value = base_duration / 2.0;
    for _ in 0..dots {
        duration += dot_value;
        dot_value /= 2.0;
    }
    duration
}

/// Convert MEI duration to MusicXML NoteTypeValue.
fn convert_mei_duration_to_note_type(
    dur: &tusk_model::data::DataDuration,
) -> tusk_musicxml::model::note::NoteTypeValue {
    use tusk_model::data::{DataDuration, DataDurationCmn};
    use tusk_musicxml::model::note::NoteTypeValue;

    match dur {
        DataDuration::DataDurationCmn(cmn) => match cmn {
            DataDurationCmn::Long => NoteTypeValue::Long,
            DataDurationCmn::Breve => NoteTypeValue::Breve,
            DataDurationCmn::N1 => NoteTypeValue::Whole,
            DataDurationCmn::N2 => NoteTypeValue::Half,
            DataDurationCmn::N4 => NoteTypeValue::Quarter,
            DataDurationCmn::N8 => NoteTypeValue::Eighth,
            DataDurationCmn::N16 => NoteTypeValue::N16th,
            DataDurationCmn::N32 => NoteTypeValue::N32nd,
            DataDurationCmn::N64 => NoteTypeValue::N64th,
            DataDurationCmn::N128 => NoteTypeValue::N128th,
            DataDurationCmn::N256 => NoteTypeValue::N256th,
            DataDurationCmn::N512 => NoteTypeValue::N512th,
            DataDurationCmn::N1024 => NoteTypeValue::N1024th,
            DataDurationCmn::N2048 => NoteTypeValue::N1024th, // MusicXML doesn't have 2048th, use 1024th
        },
        // For mensural durations, return quarter as fallback
        _ => NoteTypeValue::Quarter,
    }
}

/// Convert MEI grace attribute to MusicXML Grace element.
fn convert_mei_grace(mei_note: &tusk_model::elements::Note) -> tusk_musicxml::model::note::Grace {
    use tusk_model::data::DataGrace;
    use tusk_musicxml::model::data::YesNo;
    use tusk_musicxml::model::note::Grace;

    let mut grace = Grace::default();

    if let Some(ref grace_type) = mei_note.note_log.grace {
        match grace_type {
            DataGrace::Unacc => {
                // Unaccented = slashed grace note
                grace.slash = Some(YesNo::Yes);
            }
            DataGrace::Acc => {
                // Accented = no slash (appoggiatura-style)
                grace.slash = Some(YesNo::No);
            }
            _ => {
                // Other grace types default to no slash
                grace.slash = Some(YesNo::No);
            }
        }
    }

    grace
}

/// Convert MEI accid element to MusicXML Accidental.
fn convert_mei_accid_to_mxml(
    accid: &tusk_model::elements::Accid,
    _ctx: &mut ConversionContext,
) -> ConversionResult<tusk_musicxml::model::note::Accidental> {
    use tusk_model::att::AttAccidLogFunc;
    use tusk_model::data::DataEnclosure;
    use tusk_musicxml::model::data::YesNo;
    use tusk_musicxml::model::note::{Accidental, AccidentalValue};

    let value = if let Some(ref accid_val) = accid.accid_log.accid {
        convert_mei_written_accid_to_mxml(accid_val)
    } else {
        AccidentalValue::Natural // Default if not specified
    };

    let mut mxml_accid = Accidental::new(value);

    // Convert cautionary/editorial function
    if let Some(ref func) = accid.accid_log.func {
        match func {
            AttAccidLogFunc::Caution => {
                mxml_accid.cautionary = Some(YesNo::Yes);
            }
            AttAccidLogFunc::Edit => {
                mxml_accid.editorial = Some(YesNo::Yes);
            }
        }
    }

    // Convert enclosure
    if let Some(ref enclose) = accid.accid_vis.enclose {
        match enclose {
            DataEnclosure::Paren => {
                mxml_accid.parentheses = Some(YesNo::Yes);
            }
            DataEnclosure::Brack => {
                mxml_accid.bracket = Some(YesNo::Yes);
            }
            DataEnclosure::Box => {
                // MusicXML doesn't have box enclosure, use brackets as fallback
                mxml_accid.bracket = Some(YesNo::Yes);
            }
            DataEnclosure::None => {}
        }
    }

    Ok(mxml_accid)
}

/// Convert MEI written accidental to MusicXML AccidentalValue.
fn convert_mei_written_accid_to_mxml(
    accid: &tusk_model::data::DataAccidentalWritten,
) -> tusk_musicxml::model::note::AccidentalValue {
    use tusk_model::data::{DataAccidentalWritten, DataAccidentalWrittenBasic};
    use tusk_musicxml::model::note::AccidentalValue;

    match accid {
        DataAccidentalWritten::DataAccidentalWrittenBasic(basic) => match basic {
            DataAccidentalWrittenBasic::S => AccidentalValue::Sharp,
            DataAccidentalWrittenBasic::F => AccidentalValue::Flat,
            DataAccidentalWrittenBasic::Ss => AccidentalValue::SharpSharp,
            DataAccidentalWrittenBasic::X => AccidentalValue::DoubleSharp,
            DataAccidentalWrittenBasic::Ff => AccidentalValue::FlatFlat,
            DataAccidentalWrittenBasic::Xs | DataAccidentalWrittenBasic::Sx => {
                AccidentalValue::TripleSharp
            }
            DataAccidentalWrittenBasic::Ts => AccidentalValue::TripleSharp,
            DataAccidentalWrittenBasic::Tf => AccidentalValue::TripleFlat,
            DataAccidentalWrittenBasic::N => AccidentalValue::Natural,
            DataAccidentalWrittenBasic::Nf => AccidentalValue::NaturalFlat,
            DataAccidentalWrittenBasic::Ns => AccidentalValue::NaturalSharp,
        },
        // For extended accidentals, return natural as fallback
        _ => AccidentalValue::Natural,
    }
}

/// Convert MEI stem direction to MusicXML StemValue.
fn convert_mei_stem_direction(
    stem_dir: &tusk_model::data::DataStemdirection,
) -> tusk_musicxml::model::note::StemValue {
    use tusk_model::data::{DataStemdirection, DataStemdirectionBasic};
    use tusk_musicxml::model::note::StemValue;

    match stem_dir {
        DataStemdirection::DataStemdirectionBasic(basic) => match basic {
            DataStemdirectionBasic::Up => StemValue::Up,
            DataStemdirectionBasic::Down => StemValue::Down,
        },
        // For extended directions (left, right, ne, nw, se, sw), default to up
        DataStemdirection::DataStemdirectionExtended(_) => StemValue::Up,
    }
}

/// Add warnings for MEI attributes that are lost in conversion.
fn add_note_conversion_warnings(
    mei_note: &tusk_model::elements::Note,
    ctx: &mut ConversionContext,
) {
    // Warn about analytical attributes (100% loss)
    if mei_note.note_anl != tusk_model::att::AttNoteAnl::default() {
        ctx.add_warning(
            "note",
            "MEI analytical attributes (@pclass, @deg, etc.) have no MusicXML equivalent",
        );
    }

    // Warn about gestural attributes that aren't mapped
    if mei_note.note_ges.vel.is_some() {
        ctx.add_warning(
            "note",
            "MEI @vel (velocity) attribute is lost in MusicXML conversion",
        );
    }

    // Warn about editorial children
    for child in &mei_note.children {
        match child {
            tusk_model::elements::NoteChild::App(_)
            | tusk_model::elements::NoteChild::Choice(_)
            | tusk_model::elements::NoteChild::Corr(_)
            | tusk_model::elements::NoteChild::Sic(_)
            | tusk_model::elements::NoteChild::Del(_)
            | tusk_model::elements::NoteChild::Add(_)
            | tusk_model::elements::NoteChild::Subst(_) => {
                ctx.add_warning(
                    "note",
                    "MEI editorial markup (app, choice, corr, sic, etc.) is lost in MusicXML conversion",
                );
                break; // Only warn once
            }
            _ => {}
        }
    }
}

// ============================================================================
// MEI Rest → MusicXML Rest Conversion
// ============================================================================

/// Convert an MEI rest to a MusicXML note containing a rest.
///
/// This converts an MEI rest element to MusicXML. In MusicXML, rests are
/// represented as `<note>` elements containing a `<rest>` child rather than
/// pitch information.
///
/// # Conversion Details
///
/// - Duration: MEI `@dur` → MusicXML `<type>` (graphical); calculated duration in divisions
/// - Dots: MEI `@dots` → MusicXML `<dot>` elements
/// - Cue rests: MEI `@cue` → MusicXML `<cue>` element
/// - IDs: MEI `xml:id` → MusicXML `@id` attribute
///
/// # Lossy Conversion Notes
///
/// The following MEI attributes are lost in conversion:
/// - Timing attributes (@tstamp, @tstamp.ges, @tstamp.real) - MusicXML uses position in measure
/// - Staff/layer positioning (@staff, @layer) - determined by measure/note sequence
/// - Analytical attributes - no MusicXML equivalent
/// - Editorial child elements (app, choice, etc.) - no MusicXML equivalent
/// - Facsimile references (@facs) - no MusicXML equivalent
///
/// # Arguments
///
/// * `mei_rest` - The MEI rest to convert
/// * `ctx` - The conversion context for state tracking
///
/// # Returns
///
/// A MusicXML Note element containing a Rest, or an error if conversion fails.
pub fn convert_mei_rest(
    mei_rest: &tusk_model::elements::Rest,
    ctx: &mut ConversionContext,
) -> ConversionResult<tusk_musicxml::model::note::Note> {
    use tusk_musicxml::model::elements::Empty;
    use tusk_musicxml::model::note::{Dot, Note as MxmlNote, NoteType, Rest as MxmlRest};

    // Calculate duration from MEI rest attributes
    let duration = calculate_mei_rest_duration(mei_rest, ctx);

    // Create the MusicXML rest element
    let mxml_rest = MxmlRest::new();

    // Create the MusicXML note containing the rest
    let mut mxml_note = MxmlNote::rest(mxml_rest, duration);

    // Set ID from xml:id
    if let Some(ref xml_id) = mei_rest.common.xml_id {
        mxml_note.id = Some(xml_id.clone());
        ctx.map_id(xml_id, xml_id.clone());
    }

    // Convert note type (graphical duration)
    if let Some(note_type_value) = mei_rest
        .rest_log
        .dur
        .as_ref()
        .and_then(convert_mei_duration_rests_to_note_type)
    {
        mxml_note.note_type = Some(NoteType::new(note_type_value));
    }

    // Convert dots - check both @dots attribute and <dot> children
    let dot_count = get_mei_rest_dot_count(mei_rest);
    for _ in 0..dot_count {
        mxml_note.dots.push(Dot::default());
    }

    // Convert cue rest
    if let Some(DataBoolean::True) = mei_rest.rest_log.cue {
        mxml_note.cue = Some(Empty);
    }

    // Add warnings for lossy attributes
    add_rest_conversion_warnings(mei_rest, ctx);

    Ok(mxml_note)
}

/// Calculate MEI rest duration in MusicXML divisions.
fn calculate_mei_rest_duration(
    mei_rest: &tusk_model::elements::Rest,
    ctx: &ConversionContext,
) -> f64 {
    // First check if we have gestural duration in ppq (most accurate)
    if let Some(dur_ppq) = mei_rest.rest_ges.dur_ppq {
        return dur_ppq as f64;
    }

    // Calculate from written duration
    let divisions = ctx.divisions();
    let base_duration = if let Some(ref dur) = mei_rest.rest_log.dur {
        duration_rests_to_quarter_notes(dur)
    } else {
        1.0 // Default to quarter note
    };

    // Apply dots
    let dot_count = get_mei_rest_dot_count(mei_rest);

    let dotted_duration = apply_dots(base_duration, dot_count);

    // Convert to divisions
    dotted_duration * divisions
}

/// Convert MEI rest duration (DataDurationrests) to quarter note units.
fn duration_rests_to_quarter_notes(dur: &tusk_model::data::DataDurationrests) -> f64 {
    use tusk_model::data::{DataDurationCmn, DataDurationrests};

    match dur {
        DataDurationrests::DataDurationCmn(cmn) => match cmn {
            DataDurationCmn::Long => 16.0,    // Long = 4 whole notes
            DataDurationCmn::Breve => 8.0,    // Breve = 2 whole notes
            DataDurationCmn::N1 => 4.0,       // Whole = 4 quarters
            DataDurationCmn::N2 => 2.0,       // Half = 2 quarters
            DataDurationCmn::N4 => 1.0,       // Quarter
            DataDurationCmn::N8 => 0.5,       // Eighth
            DataDurationCmn::N16 => 0.25,     // 16th
            DataDurationCmn::N32 => 0.125,    // 32nd
            DataDurationCmn::N64 => 0.0625,   // 64th
            DataDurationCmn::N128 => 0.03125, // 128th
            DataDurationCmn::N256 => 0.015625,
            DataDurationCmn::N512 => 0.0078125,
            DataDurationCmn::N1024 => 0.00390625,
            DataDurationCmn::N2048 => 0.001953125,
        },
        // For mensural rest durations, return quarter note as fallback
        DataDurationrests::DataDurationrestsMensural(_) => 1.0,
    }
}

/// Convert MEI rest duration (DataDurationrests) to MusicXML NoteTypeValue.
fn convert_mei_duration_rests_to_note_type(
    dur: &tusk_model::data::DataDurationrests,
) -> Option<tusk_musicxml::model::note::NoteTypeValue> {
    use tusk_model::data::{DataDurationCmn, DataDurationrests};
    use tusk_musicxml::model::note::NoteTypeValue;

    match dur {
        DataDurationrests::DataDurationCmn(cmn) => {
            let value = match cmn {
                DataDurationCmn::Long => NoteTypeValue::Long,
                DataDurationCmn::Breve => NoteTypeValue::Breve,
                DataDurationCmn::N1 => NoteTypeValue::Whole,
                DataDurationCmn::N2 => NoteTypeValue::Half,
                DataDurationCmn::N4 => NoteTypeValue::Quarter,
                DataDurationCmn::N8 => NoteTypeValue::Eighth,
                DataDurationCmn::N16 => NoteTypeValue::N16th,
                DataDurationCmn::N32 => NoteTypeValue::N32nd,
                DataDurationCmn::N64 => NoteTypeValue::N64th,
                DataDurationCmn::N128 => NoteTypeValue::N128th,
                DataDurationCmn::N256 => NoteTypeValue::N256th,
                DataDurationCmn::N512 => NoteTypeValue::N512th,
                DataDurationCmn::N1024 => NoteTypeValue::N1024th,
                DataDurationCmn::N2048 => NoteTypeValue::N1024th, // MusicXML doesn't have 2048th
            };
            Some(value)
        }
        // Mensural rest durations have no direct MusicXML equivalent
        DataDurationrests::DataDurationrestsMensural(_) => None,
    }
}

/// Get the total dot count from MEI rest (both @dots attribute and <dot> children).
fn get_mei_rest_dot_count(mei_rest: &tusk_model::elements::Rest) -> u64 {
    use tusk_model::elements::RestChild;

    // First check the @dots attribute
    if let Some(ref dots) = mei_rest.rest_log.dots {
        return dots.to_string().parse::<u64>().unwrap_or(0);
    }

    // Count <dot> children as fallback
    mei_rest
        .children
        .iter()
        .filter(|c| matches!(c, RestChild::Dot(_)))
        .count() as u64
}

/// Add warnings for MEI rest attributes that are lost in conversion.
fn add_rest_conversion_warnings(
    mei_rest: &tusk_model::elements::Rest,
    ctx: &mut ConversionContext,
) {
    use tusk_model::elements::RestChild;

    // Warn about timing attributes (100% loss)
    if mei_rest.rest_log.tstamp.is_some()
        || mei_rest.rest_log.tstamp_ges.is_some()
        || mei_rest.rest_log.tstamp_real.is_some()
    {
        ctx.add_warning(
            "rest",
            "MEI timing attributes (@tstamp, @tstamp.ges, @tstamp.real) are lost in MusicXML conversion",
        );
    }

    // Warn about staff/layer positioning (position determined by sequence in MusicXML)
    if !mei_rest.rest_log.staff.is_empty() || !mei_rest.rest_log.layer.is_empty() {
        ctx.add_warning(
            "rest",
            "MEI @staff/@layer attributes are not directly mapped; position in MusicXML is determined by sequence",
        );
    }

    // Warn about facsimile links
    if !mei_rest.facsimile.facs.is_empty() {
        ctx.add_warning(
            "rest",
            "MEI @facs (facsimile link) has no MusicXML equivalent",
        );
    }

    // Warn about analytical attributes
    if mei_rest.rest_anl != tusk_model::att::AttRestAnl::default() {
        ctx.add_warning(
            "rest",
            "MEI analytical attributes have no MusicXML equivalent",
        );
    }

    // Warn about editorial children
    for child in &mei_rest.children {
        match child {
            RestChild::App(_)
            | RestChild::Choice(_)
            | RestChild::Corr(_)
            | RestChild::Sic(_)
            | RestChild::Del(_)
            | RestChild::Add(_)
            | RestChild::Subst(_) => {
                ctx.add_warning(
                    "rest",
                    "MEI editorial markup (app, choice, corr, sic, etc.) is lost in MusicXML conversion",
                );
                break; // Only warn once
            }
            _ => {}
        }
    }

    // Warn about mensural durations
    if mei_rest.rest_log.dur.as_ref().is_some_and(|dur| {
        matches!(
            dur,
            tusk_model::data::DataDurationrests::DataDurationrestsMensural(_)
        )
    }) {
        ctx.add_warning(
            "rest",
            "MEI mensural rest duration has no direct MusicXML equivalent",
        );
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use tusk_model::att::{AttMeiVersionMeiversion, AttStaffGrpVisSymbol};
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::{
        FileDesc, FileDescChild, Label, LabelAbbr, LabelAbbrChild, LabelChild, PubStmt, Title,
        TitleChild, TitleStmt, TitleStmtChild,
    };

    // ========================================================================
    // Basic Conversion Tests
    // ========================================================================

    #[test]
    fn test_convert_empty_mei() {
        let mei = Mei::default();
        let result = convert_mei(&mei);
        assert!(result.is_ok());

        let score = result.unwrap();
        assert_eq!(score.version.as_deref(), Some("4.0"));
        // Should have at least one part
        assert!(!score.part_list.items.is_empty());
        assert!(!score.parts.is_empty());
    }

    #[test]
    fn test_convert_mei_with_context() {
        let mei = Mei::default();
        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);

        let result = convert_mei_with_context(&mei, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.is_mei_to_musicxml());
    }

    // ========================================================================
    // Header Conversion Tests
    // ========================================================================

    #[test]
    fn test_convert_mei_with_title() {
        let mut mei = Mei::default();

        // Create meiHead with title
        let mut mei_head = MeiHead::default();
        let mut file_desc = FileDesc::default();
        let mut title_stmt = TitleStmt::default();
        let mut title = Title::default();
        title
            .children
            .push(TitleChild::Text("Symphony No. 5".to_string()));
        title_stmt
            .children
            .push(TitleStmtChild::Title(Box::new(title)));

        let pub_stmt = PubStmt::default();
        file_desc
            .children
            .push(FileDescChild::TitleStmt(Box::new(title_stmt)));
        file_desc
            .children
            .push(FileDescChild::PubStmt(Box::new(pub_stmt)));

        mei_head
            .children
            .push(MeiHeadChild::FileDesc(Box::new(file_desc)));
        mei.children.push(MeiChild::MeiHead(Box::new(mei_head)));

        let result = convert_mei(&mei);
        assert!(result.is_ok());

        let score = result.unwrap();
        assert!(score.work.is_some());
        assert_eq!(
            score.work.as_ref().unwrap().work_title.as_deref(),
            Some("Symphony No. 5")
        );
    }

    #[test]
    fn test_convert_mei_with_identification() {
        let mut mei = Mei::default();
        let mei_head = MeiHead::default();
        mei.children.push(MeiChild::MeiHead(Box::new(mei_head)));

        let result = convert_mei(&mei);
        assert!(result.is_ok());

        let score = result.unwrap();
        assert!(score.identification.is_some());
        let ident = score.identification.as_ref().unwrap();
        assert!(ident.encoding.is_some());
        let encoding = ident.encoding.as_ref().unwrap();
        assert!(!encoding.software.is_empty());
        assert!(encoding.software[0].contains("Tusk"));
    }

    // ========================================================================
    // Part List Conversion Tests
    // ========================================================================

    #[test]
    fn test_convert_single_staff_def() {
        // Create a staffDef with label
        let mut staff_def = StaffDef::default();
        staff_def.basic.xml_id = Some("staff-1".to_string());
        staff_def.n_integer.n = Some(1);

        let mut label = Label::default();
        label.children.push(LabelChild::Text("Piano".to_string()));
        staff_def
            .children
            .push(StaffDefChild::Label(Box::new(label)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let result = convert_staff_def_to_score_part(&staff_def, &mut ctx);
        assert!(result.is_ok());

        let score_part = result.unwrap();
        assert_eq!(score_part.id, "staff-1");
        assert_eq!(score_part.part_name.value, "Piano");
    }

    #[test]
    fn test_convert_staff_def_with_abbreviation() {
        let mut staff_def = StaffDef::default();
        staff_def.basic.xml_id = Some("staff-1".to_string());
        staff_def.n_integer.n = Some(1);

        let mut label = Label::default();
        label
            .children
            .push(LabelChild::Text("Violin I".to_string()));
        staff_def
            .children
            .push(StaffDefChild::Label(Box::new(label)));

        let mut label_abbr = LabelAbbr::default();
        label_abbr
            .children
            .push(LabelAbbrChild::Text("Vln. I".to_string()));
        staff_def
            .children
            .push(StaffDefChild::LabelAbbr(Box::new(label_abbr)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let result = convert_staff_def_to_score_part(&staff_def, &mut ctx);
        assert!(result.is_ok());

        let score_part = result.unwrap();
        assert_eq!(score_part.part_name.value, "Violin I");
        assert!(score_part.part_abbreviation.is_some());
        assert_eq!(
            score_part.part_abbreviation.as_ref().unwrap().value,
            "Vln. I"
        );
    }

    #[test]
    fn test_convert_staff_def_generates_id_from_n() {
        let mut staff_def = StaffDef::default();
        staff_def.n_integer.n = Some(3);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let result = convert_staff_def_to_score_part(&staff_def, &mut ctx);
        assert!(result.is_ok());

        let score_part = result.unwrap();
        assert_eq!(score_part.id, "P3");
    }

    // ========================================================================
    // StaffGrp to Part-List Tests
    // ========================================================================

    #[test]
    fn test_convert_simple_staff_grp() {
        let mut staff_grp = StaffGrp::default();

        // Add two staff defs
        let mut staff_def1 = StaffDef::default();
        staff_def1.basic.xml_id = Some("P1".to_string());
        staff_def1.n_integer.n = Some(1);
        let mut label1 = Label::default();
        label1
            .children
            .push(LabelChild::Text("Soprano".to_string()));
        staff_def1
            .children
            .push(StaffDefChild::Label(Box::new(label1)));

        let mut staff_def2 = StaffDef::default();
        staff_def2.basic.xml_id = Some("P2".to_string());
        staff_def2.n_integer.n = Some(2);
        let mut label2 = Label::default();
        label2.children.push(LabelChild::Text("Alto".to_string()));
        staff_def2
            .children
            .push(StaffDefChild::Label(Box::new(label2)));

        staff_grp
            .children
            .push(StaffGrpChild::StaffDef(Box::new(staff_def1)));
        staff_grp
            .children
            .push(StaffGrpChild::StaffDef(Box::new(staff_def2)));

        // Create scoreDef with staffGrp
        let mut score_def = ScoreDef::default();
        score_def
            .children
            .push(ScoreDefChild::StaffGrp(Box::new(staff_grp)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let result = convert_part_list(&score_def, &mut ctx);
        assert!(result.is_ok());

        let part_list = result.unwrap();
        // Should have: group-start, part1, part2, group-stop (because 2 children)
        assert_eq!(part_list.items.len(), 4);

        // Check that we have the two score-parts
        let parts: Vec<_> = part_list
            .items
            .iter()
            .filter_map(|item| {
                if let PartListItem::ScorePart(sp) = item {
                    Some(sp.as_ref())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].id, "P1");
        assert_eq!(parts[0].part_name.value, "Soprano");
        assert_eq!(parts[1].id, "P2");
        assert_eq!(parts[1].part_name.value, "Alto");
    }

    #[test]
    fn test_convert_staff_grp_with_symbol() {
        let mut staff_grp = StaffGrp::default();
        staff_grp.staff_grp_vis.symbol = Some(AttStaffGrpVisSymbol::Brace);

        let mut staff_def = StaffDef::default();
        staff_def.basic.xml_id = Some("P1".to_string());
        staff_grp
            .children
            .push(StaffGrpChild::StaffDef(Box::new(staff_def)));

        let result = convert_staff_grp_symbol(&staff_grp);
        assert!(result.is_some());

        use tusk_musicxml::model::elements::GroupSymbol;
        assert_eq!(result.unwrap().value, GroupSymbol::Brace);
    }

    #[test]
    fn test_convert_staff_grp_with_bar_thru() {
        let mut staff_grp = StaffGrp::default();
        staff_grp.staff_grp_vis.bar_thru = Some(DataBoolean::True);

        let result = convert_staff_grp_barline(&staff_grp);
        assert!(result.is_some());

        use tusk_musicxml::model::elements::GroupBarline;
        assert_eq!(result.unwrap().value, GroupBarline::Yes);
    }

    #[test]
    fn test_convert_staff_grp_with_label() {
        let mut staff_grp = StaffGrp::default();

        let mut label = Label::default();
        label
            .children
            .push(LabelChild::Text("Woodwinds".to_string()));
        staff_grp
            .children
            .push(StaffGrpChild::Label(Box::new(label)));

        let text = extract_label_text(&staff_grp);
        assert_eq!(text, Some("Woodwinds".to_string()));
    }

    #[test]
    fn test_convert_nested_staff_grp() {
        // Create outer staffGrp with bracket
        let mut outer_grp = StaffGrp::default();
        outer_grp.staff_grp_vis.symbol = Some(AttStaffGrpVisSymbol::Bracket);

        let mut outer_label = Label::default();
        outer_label
            .children
            .push(LabelChild::Text("Strings".to_string()));
        outer_grp
            .children
            .push(StaffGrpChild::Label(Box::new(outer_label)));

        // Create inner staffGrp (violins) with brace
        let mut inner_grp = StaffGrp::default();
        inner_grp.staff_grp_vis.symbol = Some(AttStaffGrpVisSymbol::Brace);

        let mut inner_label = Label::default();
        inner_label
            .children
            .push(LabelChild::Text("Violins".to_string()));
        inner_grp
            .children
            .push(StaffGrpChild::Label(Box::new(inner_label)));

        let mut vln1 = StaffDef::default();
        vln1.basic.xml_id = Some("P1".to_string());
        let mut vln1_label = Label::default();
        vln1_label
            .children
            .push(LabelChild::Text("Violin I".to_string()));
        vln1.children
            .push(StaffDefChild::Label(Box::new(vln1_label)));
        inner_grp
            .children
            .push(StaffGrpChild::StaffDef(Box::new(vln1)));

        let mut vln2 = StaffDef::default();
        vln2.basic.xml_id = Some("P2".to_string());
        let mut vln2_label = Label::default();
        vln2_label
            .children
            .push(LabelChild::Text("Violin II".to_string()));
        vln2.children
            .push(StaffDefChild::Label(Box::new(vln2_label)));
        inner_grp
            .children
            .push(StaffGrpChild::StaffDef(Box::new(vln2)));

        // Add inner group to outer
        outer_grp
            .children
            .push(StaffGrpChild::StaffGrp(Box::new(inner_grp)));

        // Add viola directly to outer
        let mut viola = StaffDef::default();
        viola.basic.xml_id = Some("P3".to_string());
        let mut viola_label = Label::default();
        viola_label
            .children
            .push(LabelChild::Text("Viola".to_string()));
        viola
            .children
            .push(StaffDefChild::Label(Box::new(viola_label)));
        outer_grp
            .children
            .push(StaffGrpChild::StaffDef(Box::new(viola)));

        // Create scoreDef
        let mut score_def = ScoreDef::default();
        score_def
            .children
            .push(ScoreDefChild::StaffGrp(Box::new(outer_grp)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let result = convert_part_list(&score_def, &mut ctx);
        assert!(result.is_ok());

        let part_list = result.unwrap();

        // Should have:
        // - outer group start (Strings/Bracket)
        // - inner group start (Violins/Brace)
        // - Violin I
        // - Violin II
        // - inner group stop
        // - Viola
        // - outer group stop

        // Count groups and parts
        let groups: Vec<_> = part_list
            .items
            .iter()
            .filter_map(|item| {
                if let PartListItem::PartGroup(pg) = item {
                    Some(pg.as_ref())
                } else {
                    None
                }
            })
            .collect();
        let parts: Vec<_> = part_list
            .items
            .iter()
            .filter_map(|item| {
                if let PartListItem::ScorePart(sp) = item {
                    Some(sp.as_ref())
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(groups.len(), 4); // 2 starts + 2 stops
        assert_eq!(parts.len(), 3); // Vln I, Vln II, Viola
    }

    // ========================================================================
    // Empty Parts Creation Tests
    // ========================================================================

    #[test]
    fn test_create_empty_parts() {
        let mut part_list = PartList::default();
        part_list
            .items
            .push(PartListItem::ScorePart(Box::new(ScorePart::new(
                "P1", "Piano",
            ))));
        part_list
            .items
            .push(PartListItem::ScorePart(Box::new(ScorePart::new(
                "P2", "Violin",
            ))));

        let parts = create_empty_parts(&part_list);
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].id, "P1");
        assert_eq!(parts[1].id, "P2");
    }

    // ========================================================================
    // Warning Generation Tests
    // ========================================================================

    #[test]
    fn test_conversion_generates_warnings() {
        let mut mei = Mei::default();
        let mei_head = MeiHead::default();
        mei.children.push(MeiChild::MeiHead(Box::new(mei_head)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let _ = convert_mei_with_context(&mei, &mut ctx);

        // Should have at least one warning about metadata simplification
        assert!(ctx.has_warnings());
    }

    // ========================================================================
    // Symbol Conversion Tests
    // ========================================================================

    #[test]
    fn test_convert_all_staff_grp_symbols() {
        use tusk_musicxml::model::elements::GroupSymbol;

        let test_cases = [
            (AttStaffGrpVisSymbol::Brace, GroupSymbol::Brace),
            (AttStaffGrpVisSymbol::Bracket, GroupSymbol::Bracket),
            (AttStaffGrpVisSymbol::Bracketsq, GroupSymbol::Square),
            (AttStaffGrpVisSymbol::Line, GroupSymbol::Line),
            (AttStaffGrpVisSymbol::None, GroupSymbol::None),
        ];

        for (mei_sym, expected_mxml) in test_cases {
            let mut staff_grp = StaffGrp::default();
            staff_grp.staff_grp_vis.symbol = Some(mei_sym);

            let result = convert_staff_grp_symbol(&staff_grp);
            assert!(result.is_some());
            assert_eq!(result.unwrap().value, expected_mxml);
        }
    }

    #[test]
    fn test_convert_bar_thru_values() {
        use tusk_musicxml::model::elements::GroupBarline;

        // Test true -> yes
        let mut staff_grp = StaffGrp::default();
        staff_grp.staff_grp_vis.bar_thru = Some(DataBoolean::True);
        let result = convert_staff_grp_barline(&staff_grp);
        assert_eq!(result.unwrap().value, GroupBarline::Yes);

        // Test false -> no
        let mut staff_grp = StaffGrp::default();
        staff_grp.staff_grp_vis.bar_thru = Some(DataBoolean::False);
        let result = convert_staff_grp_barline(&staff_grp);
        assert_eq!(result.unwrap().value, GroupBarline::No);
    }

    // ========================================================================
    // MEI Measure → MusicXML Measure Conversion Tests
    // ========================================================================

    #[test]
    fn test_convert_mei_measure_basic() {
        use tusk_model::data::DataWord;
        use tusk_model::elements::Measure as MeiMeasure;

        let mut mei_measure = MeiMeasure::default();
        mei_measure.common.n = Some(DataWord::from("1".to_string()));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(1.0); // Set divisions for duration calculations

        let result = convert_mei_measure(&mei_measure, "P1", &mut ctx);
        assert!(result.is_ok());

        let mxml_measure = result.unwrap();
        assert_eq!(mxml_measure.number, "1");
    }

    #[test]
    fn test_convert_mei_measure_with_id() {
        use tusk_model::data::DataWord;
        use tusk_model::elements::Measure as MeiMeasure;

        let mut mei_measure = MeiMeasure::default();
        mei_measure.common.n = Some(DataWord::from("5".to_string()));
        mei_measure.common.xml_id = Some("m5".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(1.0);

        let result = convert_mei_measure(&mei_measure, "P1", &mut ctx);
        assert!(result.is_ok());

        let mxml_measure = result.unwrap();
        assert_eq!(mxml_measure.number, "5");
        assert_eq!(mxml_measure.id, Some("m5".to_string()));
    }

    #[test]
    fn test_convert_mei_measure_implicit() {
        use tusk_model::data::DataWord;
        use tusk_model::elements::Measure as MeiMeasure;
        use tusk_musicxml::model::data::YesNo;

        let mut mei_measure = MeiMeasure::default();
        mei_measure.common.n = Some(DataWord::from("0".to_string()));
        // metcon="false" means pickup/incomplete measure → implicit="yes" in MusicXML
        mei_measure.measure_log.metcon = Some(DataBoolean::False);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(1.0);

        let result = convert_mei_measure(&mei_measure, "P1", &mut ctx);
        assert!(result.is_ok());

        let mxml_measure = result.unwrap();
        assert_eq!(mxml_measure.implicit, Some(YesNo::Yes));
    }

    #[test]
    fn test_convert_mei_measure_non_controlling() {
        use tusk_model::data::DataWord;
        use tusk_model::elements::Measure as MeiMeasure;
        use tusk_musicxml::model::data::YesNo;

        let mut mei_measure = MeiMeasure::default();
        mei_measure.common.n = Some(DataWord::from("2".to_string()));
        // control="false" means non-controlling barline
        mei_measure.measure_log.control = Some(DataBoolean::False);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(1.0);

        let result = convert_mei_measure(&mei_measure, "P1", &mut ctx);
        assert!(result.is_ok());

        let mxml_measure = result.unwrap();
        assert_eq!(mxml_measure.non_controlling, Some(YesNo::Yes));
    }

    #[test]
    fn test_convert_mei_measure_with_width() {
        use tusk_model::data::{DataMeasurementunsigned, DataWord};
        use tusk_model::elements::Measure as MeiMeasure;

        let mut mei_measure = MeiMeasure::default();
        mei_measure.common.n = Some(DataWord::from("1".to_string()));
        mei_measure.measure_vis.width = Some(DataMeasurementunsigned::from("200vu".to_string()));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(1.0);

        let result = convert_mei_measure(&mei_measure, "P1", &mut ctx);
        assert!(result.is_ok());

        let mxml_measure = result.unwrap();
        assert!(mxml_measure.width.is_some());
        // The width value should be parsed as f64
        assert_eq!(mxml_measure.width, Some(200.0));
    }

    #[test]
    fn test_convert_mei_measure_generates_number_if_missing() {
        use tusk_model::elements::Measure as MeiMeasure;

        let mei_measure = MeiMeasure::default();

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(1.0);

        let result = convert_mei_measure(&mei_measure, "P1", &mut ctx);
        assert!(result.is_ok());

        let mxml_measure = result.unwrap();
        // Should generate a measure number even if not specified
        assert!(!mxml_measure.number.is_empty());
    }

    // ========================================================================
    // MEI Note → MusicXML Note Conversion Tests
    // ========================================================================

    #[test]
    fn test_convert_mei_note_basic_pitch() {
        use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
        use tusk_model::elements::Note as MeiNote;
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::FullNoteContent;

        let mut mei_note = MeiNote::default();
        mei_note.note_log.pname = Some(DataPitchname::from("c".to_string()));
        mei_note.note_log.oct = Some(DataOctave::from(4u64));
        mei_note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(1.0);

        let result = convert_mei_note(&mei_note, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check pitch
        if let FullNoteContent::Pitch(pitch) = &mxml_note.content {
            assert_eq!(pitch.step, Step::C);
            assert_eq!(pitch.octave, 4);
            assert!(pitch.alter.is_none());
        } else {
            panic!("Expected pitched note");
        }
    }

    #[test]
    fn test_convert_mei_note_with_duration() {
        use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
        use tusk_model::elements::Note as MeiNote;
        use tusk_musicxml::model::note::NoteTypeValue;

        let mut mei_note = MeiNote::default();
        mei_note.note_log.pname = Some(DataPitchname::from("e".to_string()));
        mei_note.note_log.oct = Some(DataOctave::from(5u64));
        mei_note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N2)); // Half note

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0); // 4 divisions per quarter

        let result = convert_mei_note(&mei_note, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check note type
        assert!(mxml_note.note_type.is_some());
        assert_eq!(
            mxml_note.note_type.as_ref().unwrap().value,
            NoteTypeValue::Half
        );
        // Check duration in divisions (half note = 2 quarters = 8 divisions)
        assert!(mxml_note.duration.is_some());
        assert_eq!(mxml_note.duration.unwrap(), 8.0);
    }

    #[test]
    fn test_convert_mei_note_with_dots() {
        use tusk_model::data::{
            DataAugmentdot, DataDuration, DataDurationCmn, DataOctave, DataPitchname,
        };
        use tusk_model::elements::Note as MeiNote;

        let mut mei_note = MeiNote::default();
        mei_note.note_log.pname = Some(DataPitchname::from("g".to_string()));
        mei_note.note_log.oct = Some(DataOctave::from(4u64));
        mei_note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
        mei_note.note_log.dots = Some(DataAugmentdot::from(1u64)); // Dotted quarter

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_note(&mei_note, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check dots
        assert_eq!(mxml_note.dots.len(), 1);
        // Dotted quarter = 1.5 quarters = 6 divisions
        assert_eq!(mxml_note.duration.unwrap(), 6.0);
    }

    #[test]
    fn test_convert_mei_note_with_accidental() {
        use tusk_model::data::{
            DataAccidentalWritten, DataAccidentalWrittenBasic, DataDuration, DataDurationCmn,
            DataOctave, DataPitchname,
        };
        use tusk_model::elements::{Accid, Note as MeiNote, NoteChild};
        use tusk_musicxml::model::note::AccidentalValue;

        let mut mei_note = MeiNote::default();
        mei_note.note_log.pname = Some(DataPitchname::from("f".to_string()));
        mei_note.note_log.oct = Some(DataOctave::from(4u64));
        mei_note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

        // Add sharp accidental as child element
        let mut accid = Accid::default();
        accid.accid_log.accid = Some(DataAccidentalWritten::DataAccidentalWrittenBasic(
            DataAccidentalWrittenBasic::S,
        ));
        mei_note.children.push(NoteChild::Accid(Box::new(accid)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_note(&mei_note, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check accidental
        assert!(mxml_note.accidental.is_some());
        assert_eq!(
            mxml_note.accidental.as_ref().unwrap().value,
            AccidentalValue::Sharp
        );
    }

    #[test]
    fn test_convert_mei_note_with_gestural_accidental() {
        use tusk_model::data::{
            DataAccidentalGestural, DataAccidentalGesturalBasic, DataDuration, DataDurationCmn,
            DataOctave, DataPitchname,
        };
        use tusk_model::elements::Note as MeiNote;
        use tusk_musicxml::model::note::FullNoteContent;

        let mut mei_note = MeiNote::default();
        mei_note.note_log.pname = Some(DataPitchname::from("b".to_string()));
        mei_note.note_log.oct = Some(DataOctave::from(4u64));
        mei_note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
        // Set gestural accidental (sounding pitch is B flat)
        mei_note.note_ges.accid_ges = Some(DataAccidentalGestural::DataAccidentalGesturalBasic(
            DataAccidentalGesturalBasic::F,
        ));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_note(&mei_note, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check pitch alter (gestural accidental → alter)
        if let FullNoteContent::Pitch(pitch) = &mxml_note.content {
            assert_eq!(pitch.alter, Some(-1.0)); // Flat = -1
        } else {
            panic!("Expected pitched note");
        }
    }

    #[test]
    fn test_convert_mei_note_with_stem_direction() {
        use tusk_model::data::{
            DataDuration, DataDurationCmn, DataOctave, DataPitchname, DataStemdirection,
            DataStemdirectionBasic,
        };
        use tusk_model::elements::Note as MeiNote;
        use tusk_musicxml::model::note::StemValue;

        let mut mei_note = MeiNote::default();
        mei_note.note_log.pname = Some(DataPitchname::from("d".to_string()));
        mei_note.note_log.oct = Some(DataOctave::from(5u64));
        mei_note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
        mei_note.note_vis.stem_dir = Some(DataStemdirection::DataStemdirectionBasic(
            DataStemdirectionBasic::Down,
        ));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_note(&mei_note, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check stem direction
        assert!(mxml_note.stem.is_some());
        assert_eq!(mxml_note.stem.as_ref().unwrap().value, StemValue::Down);
    }

    #[test]
    fn test_convert_mei_note_grace() {
        use tusk_model::data::{
            DataDuration, DataDurationCmn, DataGrace, DataOctave, DataPitchname,
        };
        use tusk_model::elements::Note as MeiNote;

        let mut mei_note = MeiNote::default();
        mei_note.note_log.pname = Some(DataPitchname::from("a".to_string()));
        mei_note.note_log.oct = Some(DataOctave::from(4u64));
        mei_note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));
        mei_note.note_log.grace = Some(DataGrace::Unacc); // Unaccented/slashed grace note

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_note(&mei_note, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check grace note
        assert!(mxml_note.is_grace());
        assert!(mxml_note.grace.is_some());
        // Grace notes should not have duration
        assert!(mxml_note.duration.is_none());
        // Unaccented grace should have slash
        use tusk_musicxml::model::data::YesNo;
        assert_eq!(mxml_note.grace.as_ref().unwrap().slash, Some(YesNo::Yes));
    }

    #[test]
    fn test_convert_mei_note_cue() {
        use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
        use tusk_model::elements::Note as MeiNote;

        let mut mei_note = MeiNote::default();
        mei_note.note_log.pname = Some(DataPitchname::from("c".to_string()));
        mei_note.note_log.oct = Some(DataOctave::from(5u64));
        mei_note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
        mei_note.note_log.cue = Some(DataBoolean::True);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_note(&mei_note, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        assert!(mxml_note.is_cue());
    }

    #[test]
    fn test_convert_mei_note_with_id() {
        use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
        use tusk_model::elements::Note as MeiNote;

        let mut mei_note = MeiNote::default();
        mei_note.common.xml_id = Some("note-1".to_string());
        mei_note.note_log.pname = Some(DataPitchname::from("e".to_string()));
        mei_note.note_log.oct = Some(DataOctave::from(4u64));
        mei_note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_note(&mei_note, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check ID mapping
        assert!(mxml_note.id.is_some());
        assert_eq!(mxml_note.id.as_ref().unwrap(), "note-1");
    }

    #[test]
    fn test_convert_mei_note_all_pitches() {
        use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
        use tusk_model::elements::Note as MeiNote;
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::FullNoteContent;

        let pitch_mappings = [
            ("c", Step::C),
            ("d", Step::D),
            ("e", Step::E),
            ("f", Step::F),
            ("g", Step::G),
            ("a", Step::A),
            ("b", Step::B),
        ];

        for (mei_pname, expected_step) in pitch_mappings {
            let mut mei_note = MeiNote::default();
            mei_note.note_log.pname = Some(DataPitchname::from(mei_pname.to_string()));
            mei_note.note_log.oct = Some(DataOctave::from(4u64));
            mei_note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

            let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
            ctx.set_divisions(4.0);

            let result = convert_mei_note(&mei_note, &mut ctx);
            assert!(result.is_ok(), "Failed for pitch {}", mei_pname);

            let mxml_note = result.unwrap();
            if let FullNoteContent::Pitch(pitch) = &mxml_note.content {
                assert_eq!(
                    pitch.step, expected_step,
                    "Pitch mismatch for {}",
                    mei_pname
                );
            } else {
                panic!("Expected pitched note for {}", mei_pname);
            }
        }
    }

    #[test]
    fn test_convert_mei_note_all_durations() {
        use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
        use tusk_model::elements::Note as MeiNote;
        use tusk_musicxml::model::note::NoteTypeValue;

        let duration_mappings = [
            (DataDurationCmn::Breve, NoteTypeValue::Breve),
            (DataDurationCmn::N1, NoteTypeValue::Whole),
            (DataDurationCmn::N2, NoteTypeValue::Half),
            (DataDurationCmn::N4, NoteTypeValue::Quarter),
            (DataDurationCmn::N8, NoteTypeValue::Eighth),
            (DataDurationCmn::N16, NoteTypeValue::N16th),
            (DataDurationCmn::N32, NoteTypeValue::N32nd),
            (DataDurationCmn::N64, NoteTypeValue::N64th),
        ];

        for (mei_dur, expected_type) in duration_mappings {
            let mut mei_note = MeiNote::default();
            mei_note.note_log.pname = Some(DataPitchname::from("c".to_string()));
            mei_note.note_log.oct = Some(DataOctave::from(4u64));
            mei_note.note_log.dur = Some(DataDuration::DataDurationCmn(mei_dur));

            let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
            ctx.set_divisions(4.0);

            let result = convert_mei_note(&mei_note, &mut ctx);
            assert!(result.is_ok(), "Failed for duration {:?}", mei_dur);

            let mxml_note = result.unwrap();
            assert!(
                mxml_note.note_type.is_some(),
                "No note type for {:?}",
                mei_dur
            );
            assert_eq!(
                mxml_note.note_type.as_ref().unwrap().value,
                expected_type,
                "Duration mismatch for {:?}",
                mei_dur
            );
        }
    }

    // ========================================================================
    // MEI Rest → MusicXML Rest Conversion Tests
    // ========================================================================

    #[test]
    fn test_convert_mei_rest_basic() {
        use tusk_model::data::{DataDurationCmn, DataDurationrests};
        use tusk_model::elements::Rest as MeiRest;

        let mut mei_rest = MeiRest::default();
        mei_rest.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_rest(&mei_rest, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check that it's a rest
        assert!(mxml_note.is_rest());
        // Check duration (quarter note = 1 quarter = 4 divisions)
        assert_eq!(mxml_note.duration, Some(4.0));
    }

    #[test]
    fn test_convert_mei_rest_with_duration_type() {
        use tusk_model::data::{DataDurationCmn, DataDurationrests};
        use tusk_model::elements::Rest as MeiRest;
        use tusk_musicxml::model::note::NoteTypeValue;

        let mut mei_rest = MeiRest::default();
        mei_rest.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N2)); // Half rest

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_rest(&mei_rest, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check note type (graphical duration)
        assert!(mxml_note.note_type.is_some());
        assert_eq!(
            mxml_note.note_type.as_ref().unwrap().value,
            NoteTypeValue::Half
        );
        // Check duration in divisions (half rest = 2 quarters = 8 divisions)
        assert_eq!(mxml_note.duration, Some(8.0));
    }

    #[test]
    fn test_convert_mei_rest_with_dots() {
        use tusk_model::data::{DataAugmentdot, DataDurationCmn, DataDurationrests};
        use tusk_model::elements::Rest as MeiRest;

        let mut mei_rest = MeiRest::default();
        mei_rest.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4));
        mei_rest.rest_log.dots = Some(DataAugmentdot::from(1u64)); // Dotted quarter

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_rest(&mei_rest, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check dots
        assert_eq!(mxml_note.dots.len(), 1);
        // Dotted quarter = 1.5 quarters = 6 divisions
        assert_eq!(mxml_note.duration.unwrap(), 6.0);
    }

    #[test]
    fn test_convert_mei_rest_with_id() {
        use tusk_model::data::{DataDurationCmn, DataDurationrests};
        use tusk_model::elements::Rest as MeiRest;

        let mut mei_rest = MeiRest::default();
        mei_rest.common.xml_id = Some("rest-1".to_string());
        mei_rest.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_rest(&mei_rest, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check ID mapping
        assert!(mxml_note.id.is_some());
        assert_eq!(mxml_note.id.as_ref().unwrap(), "rest-1");
    }

    #[test]
    fn test_convert_mei_rest_cue() {
        use tusk_model::data::{DataDurationCmn, DataDurationrests};
        use tusk_model::elements::Rest as MeiRest;

        let mut mei_rest = MeiRest::default();
        mei_rest.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4));
        mei_rest.rest_log.cue = Some(DataBoolean::True);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_rest(&mei_rest, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        assert!(mxml_note.is_cue());
    }

    #[test]
    fn test_convert_mei_rest_with_dur_ppq() {
        use tusk_model::elements::Rest as MeiRest;

        let mut mei_rest = MeiRest::default();
        // Set gestural duration directly (12 ppq)
        mei_rest.rest_ges.dur_ppq = Some(12);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0); // Even with divisions set, dur.ppq takes precedence

        let result = convert_mei_rest(&mei_rest, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Duration should be exactly the dur.ppq value
        assert_eq!(mxml_note.duration, Some(12.0));
    }

    #[test]
    fn test_convert_mei_rest_whole_rest() {
        use tusk_model::data::{DataDurationCmn, DataDurationrests};
        use tusk_model::elements::Rest as MeiRest;
        use tusk_musicxml::model::note::NoteTypeValue;

        let mut mei_rest = MeiRest::default();
        mei_rest.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N1)); // Whole rest

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_rest(&mei_rest, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        assert!(mxml_note.is_rest());
        assert!(mxml_note.note_type.is_some());
        assert_eq!(
            mxml_note.note_type.as_ref().unwrap().value,
            NoteTypeValue::Whole
        );
        // Whole rest = 4 quarters = 16 divisions
        assert_eq!(mxml_note.duration, Some(16.0));
    }

    #[test]
    fn test_convert_mei_rest_all_durations() {
        use tusk_model::data::{DataDurationCmn, DataDurationrests};
        use tusk_model::elements::Rest as MeiRest;
        use tusk_musicxml::model::note::NoteTypeValue;

        let duration_mappings = [
            (DataDurationCmn::Breve, NoteTypeValue::Breve),
            (DataDurationCmn::N1, NoteTypeValue::Whole),
            (DataDurationCmn::N2, NoteTypeValue::Half),
            (DataDurationCmn::N4, NoteTypeValue::Quarter),
            (DataDurationCmn::N8, NoteTypeValue::Eighth),
            (DataDurationCmn::N16, NoteTypeValue::N16th),
            (DataDurationCmn::N32, NoteTypeValue::N32nd),
            (DataDurationCmn::N64, NoteTypeValue::N64th),
        ];

        for (mei_dur, expected_type) in duration_mappings {
            let mut mei_rest = MeiRest::default();
            mei_rest.rest_log.dur = Some(DataDurationrests::DataDurationCmn(mei_dur));

            let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
            ctx.set_divisions(4.0);

            let result = convert_mei_rest(&mei_rest, &mut ctx);
            assert!(result.is_ok(), "Failed for duration {:?}", mei_dur);

            let mxml_note = result.unwrap();
            assert!(
                mxml_note.note_type.is_some(),
                "No note type for {:?}",
                mei_dur
            );
            assert_eq!(
                mxml_note.note_type.as_ref().unwrap().value,
                expected_type,
                "Duration mismatch for {:?}",
                mei_dur
            );
        }
    }

    #[test]
    fn test_convert_mei_rest_double_dotted() {
        use tusk_model::data::{DataAugmentdot, DataDurationCmn, DataDurationrests};
        use tusk_model::elements::Rest as MeiRest;

        let mut mei_rest = MeiRest::default();
        mei_rest.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N2));
        mei_rest.rest_log.dots = Some(DataAugmentdot::from(2u64)); // Double-dotted half

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_rest(&mei_rest, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check dots
        assert_eq!(mxml_note.dots.len(), 2);
        // Double-dotted half = 2 + 1 + 0.5 = 3.5 quarters = 14 divisions
        assert_eq!(mxml_note.duration.unwrap(), 14.0);
    }

    #[test]
    fn test_convert_mei_rest_generates_warnings_for_timing() {
        use tusk_model::data::{DataBeat, DataDurationCmn, DataDurationrests};
        use tusk_model::elements::Rest as MeiRest;

        let mut mei_rest = MeiRest::default();
        mei_rest.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4));
        mei_rest.rest_log.tstamp = Some(DataBeat::from(1.0));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let _ = convert_mei_rest(&mei_rest, &mut ctx);

        // Should have warnings about timing attributes being lost
        assert!(ctx.has_warnings());
    }

    #[test]
    fn test_convert_mei_rest_default_duration() {
        use tusk_model::elements::Rest as MeiRest;

        // Rest with no duration specified
        let mei_rest = MeiRest::default();

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_rest(&mei_rest, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Should default to quarter note = 4 divisions
        assert_eq!(mxml_note.duration, Some(4.0));
    }
}
