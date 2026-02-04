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
}
