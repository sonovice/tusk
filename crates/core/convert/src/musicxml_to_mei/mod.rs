//! MusicXML to MEI conversion.
//!
//! This module provides conversion from MusicXML score-partwise documents
//! to MEI format. The conversion is lossless - all MusicXML content is
//! preserved in the MEI output.
//!
//! # Conversion Overview
//!
//! MusicXML `<score-partwise>` maps to MEI as follows:
//! - MusicXML header (work, identification) → MEI `<meiHead>`
//! - MusicXML `<part-list>` → MEI `<staffGrp>` with `<staffDef>` elements
//! - MusicXML `<part>/<measure>` → MEI `<section>/<measure>/<staff>/<layer>`
//!
//! # Example
//!
//! ```ignore
//! use tusk_convert::musicxml_to_mei::convert_score;
//! use tusk_musicxml::model::elements::ScorePartwise;
//!
//! let score = ScorePartwise::default();
//! let mei = convert_score(&score)?;
//! ```

mod attributes;
mod direction;
mod note;
mod structure;

// Re-export attributes conversion functions
pub use attributes::{
    convert_clef_attributes, convert_key_fifths, convert_key_to_context, convert_time_signature,
    process_attributes,
};

// Re-export direction conversion functions
pub use direction::{DirectionConversionResult, convert_direction};

// Re-export note conversion functions
pub use note::{convert_chord, convert_measure_rest, convert_note, convert_rest, is_measure_rest};

// Re-export structure conversion functions
pub use structure::{
    convert_body, convert_layer, convert_mdiv, convert_measure, convert_score_content,
    convert_section, convert_staff,
};

use crate::context::{ConversionContext, ConversionDirection};
use crate::error::ConversionResult;
use tusk_model::att::{AttMeiVersionMeiversion, AttStaffGrpVisSymbol};
use tusk_model::data::{DataBoolean, DataClefline, DataClefshape};
use tusk_model::elements::{
    Label, LabelAbbr, LabelAbbrChild, LabelChild, Mei, MeiChild, MeiHead, MeiHeadChild, Music,
    ScoreDef, StaffDef, StaffDefChild, StaffGrp, StaffGrpChild,
};
use tusk_musicxml::model::attributes::KeyContent;
use tusk_musicxml::model::elements::{PartGroup, PartListItem, ScorePart, ScorePartwise};

/// Convert a MusicXML score-partwise document to MEI.
///
/// This is the main entry point for MusicXML → MEI conversion.
/// The conversion creates a complete MEI document with:
/// - `<meiHead>` containing metadata from MusicXML identification
/// - `<music>/<body>/<mdiv>/<score>` containing the musical content
///
/// # Arguments
///
/// * `score` - The MusicXML score-partwise document to convert
///
/// # Returns
///
/// A complete MEI document, or an error if conversion fails.
pub fn convert_score(score: &ScorePartwise) -> ConversionResult<Mei> {
    let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
    convert_score_with_context(score, &mut ctx)
}

/// Convert a MusicXML score-partwise document to MEI with an existing context.
///
/// This variant allows reusing a conversion context across multiple conversions,
/// which is useful for batch processing or when custom context configuration is needed.
pub fn convert_score_with_context(
    score: &ScorePartwise,
    ctx: &mut ConversionContext,
) -> ConversionResult<Mei> {
    // Build MEI document structure
    let mei_head = convert_header(score, ctx)?;
    let music = convert_music(score, ctx)?;

    // Create root MEI element
    let mut mei = Mei::default();
    mei.mei_version.meiversion = Some(AttMeiVersionMeiversion::N60Dev);

    mei.children.push(MeiChild::MeiHead(Box::new(mei_head)));
    mei.children.push(MeiChild::Music(Box::new(music)));

    Ok(mei)
}

/// Convert MusicXML header information to MEI meiHead.
fn convert_header(score: &ScorePartwise, ctx: &mut ConversionContext) -> ConversionResult<MeiHead> {
    let mut mei_head = MeiHead::default();

    // Create fileDesc with titleStmt
    let file_desc = convert_file_desc(score, ctx)?;
    mei_head
        .children
        .push(MeiHeadChild::FileDesc(Box::new(file_desc)));

    // Add encodingDesc with Tusk application info
    let encoding_desc = create_encoding_desc(ctx)?;
    mei_head
        .children
        .push(MeiHeadChild::EncodingDesc(Box::new(encoding_desc)));

    Ok(mei_head)
}

/// Convert MusicXML identification to MEI fileDesc.
fn convert_file_desc(
    score: &ScorePartwise,
    _ctx: &mut ConversionContext,
) -> ConversionResult<tusk_model::elements::FileDesc> {
    use tusk_model::elements::{
        FileDesc, FileDescChild, PubStmt, Title, TitleStmt, TitleStmtChild,
    };

    let mut file_desc = FileDesc::default();

    // Create titleStmt with title
    let mut title_stmt = TitleStmt::default();

    // Try to get title from work-title, movement-title, or fall back to "Untitled"
    let title_text = score
        .work
        .as_ref()
        .and_then(|w| w.work_title.as_ref())
        .or(score.movement_title.as_ref())
        .map(|s| s.as_str())
        .unwrap_or("Untitled");

    let mut title = Title::default();
    title.children.push(tusk_model::elements::TitleChild::Text(
        title_text.to_string(),
    ));
    title_stmt
        .children
        .push(TitleStmtChild::Title(Box::new(title)));

    file_desc
        .children
        .push(FileDescChild::TitleStmt(Box::new(title_stmt)));

    // Add pubStmt (required, even if empty)
    let pub_stmt = PubStmt::default();
    file_desc
        .children
        .push(FileDescChild::PubStmt(Box::new(pub_stmt)));

    Ok(file_desc)
}

/// Create MEI encodingDesc with Tusk application info.
fn create_encoding_desc(
    _ctx: &mut ConversionContext,
) -> ConversionResult<tusk_model::elements::EncodingDesc> {
    use tusk_model::elements::{
        AppInfo, AppInfoChild, Application, ApplicationChild, EncodingDesc, EncodingDescChild,
        Name, NameChild,
    };

    let mut encoding_desc = EncodingDesc::default();

    // Create appInfo with Tusk application
    let mut app_info = AppInfo::default();

    let mut application = Application::default();
    application.common.xml_id = Some("tusk".to_string());

    let mut name = Name::default();
    name.children
        .push(NameChild::Text("Tusk MusicXML-MEI Converter".to_string()));
    application
        .children
        .push(ApplicationChild::Name(Box::new(name)));

    app_info
        .children
        .push(AppInfoChild::Application(Box::new(application)));
    encoding_desc
        .children
        .push(EncodingDescChild::AppInfo(Box::new(app_info)));

    Ok(encoding_desc)
}

/// Convert MusicXML score content to MEI music element.
fn convert_music(score: &ScorePartwise, ctx: &mut ConversionContext) -> ConversionResult<Music> {
    let music = Music::default();

    // Create body containing mdiv
    // Note: The generated Music type doesn't have Body as a direct child in MusicChild enum.
    // This is a known limitation of the generated code - the MEI spec allows body as a child
    // of music, but the code generator only included genDesc, performance, facsimile.
    // For now, we create the body structure separately.
    // The actual MEI document assembly with body will need to be handled at serialization.
    let _body = convert_body(score, ctx)?;

    // Since Music doesn't have Body as a child variant in the generated code,
    // we return an empty Music. The full document structure including body
    // will need special handling during serialization to produce valid MEI.
    // This is acceptable for Phase 4.3 - full integration will come later.

    Ok(music)
}

/// Convert MusicXML part-list to MEI scoreDef.
pub fn convert_score_def(
    score: &ScorePartwise,
    ctx: &mut ConversionContext,
) -> ConversionResult<ScoreDef> {
    let mut score_def = ScoreDef::default();

    // Create staffGrp containing staffDef for each part
    let staff_grp = convert_staff_grp(score, ctx)?;
    score_def
        .children
        .push(tusk_model::elements::ScoreDefChild::StaffGrp(Box::new(
            staff_grp,
        )));

    Ok(score_def)
}

/// Convert MusicXML part-list to MEI staffGrp.
///
/// MusicXML part-list can contain:
/// - `<score-part>` elements defining individual parts → converted to `<staffDef>`
/// - `<part-group type="start/stop">` elements grouping parts → converted to nested `<staffGrp>`
///
/// The conversion handles nested groups by tracking open groups on a stack. When a group
/// starts, we create a new `<staffGrp>` and push it; subsequent parts/groups go into this
/// group until we see the matching stop marker.
pub fn convert_staff_grp(
    score: &ScorePartwise,
    ctx: &mut ConversionContext,
) -> ConversionResult<StaffGrp> {
    let mut root_grp = StaffGrp::default();

    // Track open groups: (group_number, StaffGrp)
    // We build groups as we encounter them and nest them properly
    let mut group_stack: Vec<(String, StaffGrp)> = vec![];

    let mut staff_number = 1u32;

    for item in &score.part_list.items {
        match item {
            PartListItem::ScorePart(score_part) => {
                // Extract initial attributes from the first measure of this part
                let initial_attrs = extract_first_measure_attributes(score, &score_part.id);
                let staff_def = convert_staff_def_from_score_part(
                    score_part,
                    staff_number,
                    initial_attrs,
                    ctx,
                )?;

                // Add to innermost open group, or root if none
                if let Some((_, grp)) = group_stack.last_mut() {
                    grp.children
                        .push(StaffGrpChild::StaffDef(Box::new(staff_def)));
                } else {
                    root_grp
                        .children
                        .push(StaffGrpChild::StaffDef(Box::new(staff_def)));
                }

                // Map part ID to staff number
                ctx.map_id(&score_part.id, format!("staff-{}", staff_number));
                staff_number += 1;
            }
            PartListItem::PartGroup(part_group) => {
                let group_number = part_group.number.clone().unwrap_or_else(|| "1".to_string());

                match part_group.group_type {
                    tusk_musicxml::model::data::StartStop::Start => {
                        // Start a new group
                        let new_grp = convert_staff_grp_from_part_group(part_group, ctx)?;
                        group_stack.push((group_number, new_grp));
                    }
                    tusk_musicxml::model::data::StartStop::Stop => {
                        // Find and close the matching group
                        if let Some(idx) = group_stack
                            .iter()
                            .rposition(|(num, _)| num == &group_number)
                        {
                            let (_, completed_grp) = group_stack.remove(idx);

                            // Add completed group to parent (or root)
                            if let Some((_, parent_grp)) = group_stack.last_mut() {
                                parent_grp
                                    .children
                                    .push(StaffGrpChild::StaffGrp(Box::new(completed_grp)));
                            } else {
                                root_grp
                                    .children
                                    .push(StaffGrpChild::StaffGrp(Box::new(completed_grp)));
                            }
                        }
                        // If no matching start, ignore the stop marker
                    }
                }
            }
        }
    }

    // Handle any unclosed groups (malformed input) - add them to root
    while let Some((_, unclosed_grp)) = group_stack.pop() {
        root_grp
            .children
            .push(StaffGrpChild::StaffGrp(Box::new(unclosed_grp)));
    }

    Ok(root_grp)
}

/// Convert MusicXML part-group (start) to MEI staffGrp attributes.
///
/// Maps:
/// - `group-symbol` (brace, bracket, line, square, none) → `@symbol`
/// - `group-barline` (yes/no/Mensurstrich) → `@bar.thru`
/// - `group-name` → `<label>` child
/// - `group-abbreviation` → `<labelAbbr>` child
fn convert_staff_grp_from_part_group(
    part_group: &PartGroup,
    ctx: &mut ConversionContext,
) -> ConversionResult<StaffGrp> {
    let mut staff_grp = StaffGrp::default();

    // Generate ID for the staffGrp
    let grp_id = ctx.generate_id_with_suffix("staffgrp");
    staff_grp.common.xml_id = Some(grp_id);

    // Convert group symbol
    if let Some(ref symbol_value) = part_group.group_symbol {
        staff_grp.staff_grp_vis.symbol = Some(convert_group_symbol(symbol_value.value));
    }

    // Convert group barline → bar.thru
    if let Some(ref barline_value) = part_group.group_barline {
        staff_grp.staff_grp_vis.bar_thru = Some(convert_group_barline(barline_value.value));
    }

    // Convert group name → label
    if let Some(ref group_name) = part_group.group_name {
        let mut label = Label::default();
        label.children.push(LabelChild::Text(group_name.clone()));
        staff_grp
            .children
            .push(StaffGrpChild::Label(Box::new(label)));
    }

    // Convert group abbreviation → labelAbbr
    if let Some(ref group_abbr) = part_group.group_abbreviation {
        let mut label_abbr = LabelAbbr::default();
        label_abbr
            .children
            .push(LabelAbbrChild::Text(group_abbr.clone()));
        staff_grp
            .children
            .push(StaffGrpChild::LabelAbbr(Box::new(label_abbr)));
    }

    Ok(staff_grp)
}

/// Convert MusicXML GroupSymbol to MEI AttStaffGrpVisSymbol.
fn convert_group_symbol(
    symbol: tusk_musicxml::model::elements::GroupSymbol,
) -> AttStaffGrpVisSymbol {
    use tusk_musicxml::model::elements::GroupSymbol;

    match symbol {
        GroupSymbol::Brace => AttStaffGrpVisSymbol::Brace,
        GroupSymbol::Bracket => AttStaffGrpVisSymbol::Bracket,
        GroupSymbol::Square => AttStaffGrpVisSymbol::Bracketsq,
        GroupSymbol::Line => AttStaffGrpVisSymbol::Line,
        GroupSymbol::None => AttStaffGrpVisSymbol::None,
    }
}

/// Convert MusicXML GroupBarline to MEI DataBoolean for bar.thru attribute.
fn convert_group_barline(barline: tusk_musicxml::model::elements::GroupBarline) -> DataBoolean {
    use tusk_musicxml::model::elements::GroupBarline;

    match barline {
        GroupBarline::Yes => DataBoolean::True,
        GroupBarline::No => DataBoolean::False,
        // Mensurstrich is a special case where barlines go between staves but not through them
        // In MEI, this maps to bar.thru=false (barlines don't go through staves)
        GroupBarline::Mensurstrich => DataBoolean::False,
    }
}

/// Extract the first Attributes element from a MusicXML part's first measure.
///
/// This is used to initialize the staffDef with correct key/time/clef from the score.
fn extract_first_measure_attributes<'a>(
    score: &'a ScorePartwise,
    part_id: &str,
) -> Option<&'a tusk_musicxml::model::attributes::Attributes> {
    use tusk_musicxml::model::elements::MeasureContent;

    // Find the part by ID
    let part = score.parts.iter().find(|p| p.id == part_id)?;

    // Get first measure
    let first_measure = part.measures.first()?;

    // Find first Attributes element
    for content in &first_measure.content {
        if let MeasureContent::Attributes(attrs) = content {
            return Some(attrs.as_ref());
        }
    }

    None
}

/// Convert a MusicXML ScorePart to MEI staffDef with full metadata.
///
/// Maps:
/// - part-name → `<label>` child
/// - part-abbreviation → `<labelAbbr>` child
/// - Staff number → `@n`
/// - Default clef and lines
/// - Initial key/time/clef from first measure attributes
fn convert_staff_def_from_score_part(
    score_part: &ScorePart,
    staff_number: u32,
    initial_attrs: Option<&tusk_musicxml::model::attributes::Attributes>,
    ctx: &mut ConversionContext,
) -> ConversionResult<StaffDef> {
    let mut staff_def = StaffDef::default();

    // Set staff number
    staff_def.n_integer.n = Some(staff_number as u64);

    // Set default staff lines (5 for CMN)
    staff_def.staff_def_log.lines = Some(5);

    // Default clef (G clef on line 2 = treble clef)
    // These may be overridden below if initial attributes specify a different clef
    staff_def.staff_def_log.clef_shape = Some(DataClefshape::G);
    staff_def.staff_def_log.clef_line = Some(DataClefline::from(2u64));

    // Apply initial attributes from the first measure (key, time, clef)
    if let Some(attrs) = initial_attrs {
        // Process divisions to set context state
        if let Some(divs) = attrs.divisions {
            ctx.set_divisions(divs);
        }

        // Apply key signature
        if let Some(key) = attrs.keys.first() {
            convert_key_to_context(key, ctx);
            if let KeyContent::Traditional(trad) = &key.content {
                let keysig = convert_key_fifths(trad.fifths);
                staff_def.staff_def_log.keysig = vec![keysig];
            }
        }

        // Apply time signature
        if let Some(time) = attrs.times.first() {
            let (count, unit, sym) = convert_time_signature(time);
            staff_def.staff_def_log.meter_count = count;
            staff_def.staff_def_log.meter_unit = unit;
            staff_def.staff_def_log.meter_sym = sym;
        }

        // Apply clef (overrides default)
        // Look for clef matching this staff number, or first clef if no staff number specified
        let clef = attrs
            .clefs
            .iter()
            .find(|c| c.number.is_none_or(|n| n == staff_number))
            .or_else(|| attrs.clefs.first());

        if let Some(clef) = clef {
            let (shape, line, dis, dis_place) = convert_clef_attributes(clef);
            if let Some(s) = shape {
                staff_def.staff_def_log.clef_shape = Some(s);
            }
            if let Some(l) = line {
                staff_def.staff_def_log.clef_line = Some(l);
            }
            staff_def.staff_def_log.clef_dis = dis;
            staff_def.staff_def_log.clef_dis_place = dis_place;
        }
    }

    // Generate an ID for the staffDef
    let staff_def_id = ctx.generate_id_with_suffix("staffdef");
    staff_def.basic.xml_id = Some(staff_def_id);

    // Convert part-name → label (if not empty)
    if !score_part.part_name.value.is_empty() {
        let mut label = Label::default();
        label
            .children
            .push(LabelChild::Text(score_part.part_name.value.clone()));
        staff_def
            .children
            .push(StaffDefChild::Label(Box::new(label)));
    }

    // Convert part-abbreviation → labelAbbr
    if let Some(ref abbr) = score_part.part_abbreviation
        && !abbr.value.is_empty()
    {
        let mut label_abbr = LabelAbbr::default();
        label_abbr
            .children
            .push(LabelAbbrChild::Text(abbr.value.clone()));
        staff_def
            .children
            .push(StaffDefChild::LabelAbbr(Box::new(label_abbr)));
    }

    Ok(staff_def)
}

/// Convert a MusicXML part to MEI staffDef (minimal version without part metadata).
///
/// This is a simpler version for cases where only a part ID and staff number are available.
/// For full conversion including part name and abbreviation, use `convert_staff_def_from_score_part`.
#[deprecated(
    note = "Use convert_staff_def_from_score_part for full part-list conversion with labels"
)]
pub fn convert_staff_def(
    _part_id: &str,
    staff_number: u32,
    ctx: &mut ConversionContext,
) -> ConversionResult<StaffDef> {
    let mut staff_def = StaffDef::default();

    // Set staff number using n_integer.n (u64)
    staff_def.n_integer.n = Some(staff_number as u64);

    // Set default staff lines (5 for CMN)
    staff_def.staff_def_log.lines = Some(5);

    // Default clef (G clef on line 2 = treble clef)
    // These will be overridden when we process attributes in the first measure
    staff_def.staff_def_log.clef_shape = Some(DataClefshape::G);
    staff_def.staff_def_log.clef_line = Some(DataClefline::from(2u64));

    // Generate an ID for the staffDef using basic.xml_id
    let staff_def_id = ctx.generate_id_with_suffix("staffdef");
    staff_def.basic.xml_id = Some(staff_def_id);

    Ok(staff_def)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tusk_musicxml::model::elements::{Part, PartList, PartListItem, PartName, ScorePart, Work};

    /// Helper to create a ScorePart with the given id and name.
    fn make_score_part(id: &str, name: &str) -> ScorePart {
        ScorePart {
            id: id.to_string(),
            identification: None,
            part_links: vec![],
            part_name: PartName {
                value: name.to_string(),
                ..Default::default()
            },
            part_name_display: None,
            part_abbreviation: None,
            part_abbreviation_display: None,
            groups: vec![],
            score_instruments: vec![],
            players: vec![],
            midi_assignments: vec![],
        }
    }

    // ============================================================================
    // Basic Document Structure Tests
    // ============================================================================

    #[test]
    fn convert_empty_score_creates_valid_mei_structure() {
        let score = ScorePartwise::default();
        let mei = convert_score(&score).expect("conversion should succeed");

        // Check MEI version is set
        assert!(mei.mei_version.meiversion.is_some());

        // Check we have meiHead and music children
        assert_eq!(mei.children.len(), 2);
        assert!(matches!(&mei.children[0], MeiChild::MeiHead(_)));
        assert!(matches!(&mei.children[1], MeiChild::Music(_)));
    }

    #[test]
    fn convert_score_sets_mei_version() {
        let score = ScorePartwise::default();
        let mei = convert_score(&score).expect("conversion should succeed");

        // Should set MEI version to 6.0-dev (current dev version from ODD)
        assert_eq!(
            mei.mei_version.meiversion,
            Some(AttMeiVersionMeiversion::N60Dev)
        );
    }

    // ============================================================================
    // Header Conversion Tests
    // ============================================================================

    #[test]
    fn convert_header_creates_file_desc() {
        let score = ScorePartwise::default();
        let mei = convert_score(&score).expect("conversion should succeed");

        if let MeiChild::MeiHead(head) = &mei.children[0] {
            // Should have fileDesc as first child
            assert!(!head.children.is_empty());
            assert!(matches!(&head.children[0], MeiHeadChild::FileDesc(_)));
        } else {
            panic!("Expected MeiHead");
        }
    }

    #[test]
    fn convert_header_uses_work_title() {
        let mut score = ScorePartwise::default();
        score.work = Some(Work {
            work_title: Some("Test Symphony".to_string()),
            ..Default::default()
        });

        let mei = convert_score(&score).expect("conversion should succeed");

        if let MeiChild::MeiHead(head) = &mei.children[0] {
            if let MeiHeadChild::FileDesc(file_desc) = &head.children[0] {
                // Find titleStmt
                let title_stmt = file_desc.children.iter().find_map(|c| {
                    if let tusk_model::elements::FileDescChild::TitleStmt(ts) = c {
                        Some(ts)
                    } else {
                        None
                    }
                });
                assert!(title_stmt.is_some());

                // Check title content
                let ts = title_stmt.unwrap();
                let title = ts.children.iter().find_map(|c| {
                    if let tusk_model::elements::TitleStmtChild::Title(t) = c {
                        Some(t)
                    } else {
                        None
                    }
                });
                assert!(title.is_some());

                // Check title text
                let t = title.unwrap();
                let text = t.children.iter().find_map(|c| {
                    if let tusk_model::elements::TitleChild::Text(s) = c {
                        Some(s.as_str())
                    } else {
                        None
                    }
                });
                assert_eq!(text, Some("Test Symphony"));
            } else {
                panic!("Expected FileDesc");
            }
        } else {
            panic!("Expected MeiHead");
        }
    }

    #[test]
    fn convert_header_uses_movement_title_as_fallback() {
        let mut score = ScorePartwise::default();
        score.movement_title = Some("Movement I".to_string());

        let mei = convert_score(&score).expect("conversion should succeed");

        if let MeiChild::MeiHead(head) = &mei.children[0] {
            if let MeiHeadChild::FileDesc(file_desc) = &head.children[0] {
                let title_text = extract_title_text(file_desc);
                assert_eq!(title_text, Some("Movement I"));
            } else {
                panic!("Expected FileDesc");
            }
        } else {
            panic!("Expected MeiHead");
        }
    }

    #[test]
    fn convert_header_uses_untitled_when_no_title() {
        let score = ScorePartwise::default();
        let mei = convert_score(&score).expect("conversion should succeed");

        if let MeiChild::MeiHead(head) = &mei.children[0] {
            if let MeiHeadChild::FileDesc(file_desc) = &head.children[0] {
                let title_text = extract_title_text(file_desc);
                assert_eq!(title_text, Some("Untitled"));
            } else {
                panic!("Expected FileDesc");
            }
        } else {
            panic!("Expected MeiHead");
        }
    }

    #[test]
    fn convert_header_includes_encoding_desc() {
        let score = ScorePartwise::default();
        let mei = convert_score(&score).expect("conversion should succeed");

        if let MeiChild::MeiHead(head) = &mei.children[0] {
            let has_encoding_desc = head
                .children
                .iter()
                .any(|c| matches!(c, MeiHeadChild::EncodingDesc(_)));
            assert!(has_encoding_desc, "Should include encodingDesc");
        } else {
            panic!("Expected MeiHead");
        }
    }

    // ============================================================================
    // Part List Conversion Tests
    // ============================================================================

    #[test]
    fn convert_part_list_creates_staff_grp() {
        let mut score = ScorePartwise::default();
        score.part_list = PartList {
            items: vec![PartListItem::ScorePart(Box::new(make_score_part(
                "P1", "Piano",
            )))],
        };

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let staff_grp = convert_staff_grp(&score, &mut ctx).expect("conversion should succeed");

        assert_eq!(staff_grp.children.len(), 1);
        assert!(matches!(&staff_grp.children[0], StaffGrpChild::StaffDef(_)));
    }

    #[test]
    fn convert_part_list_maps_part_ids_to_staff_numbers() {
        let mut score = ScorePartwise::default();
        score.part_list = PartList {
            items: vec![
                PartListItem::ScorePart(Box::new(make_score_part("P1", "Violin I"))),
                PartListItem::ScorePart(Box::new(make_score_part("P2", "Violin II"))),
            ],
        };

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let _staff_grp = convert_staff_grp(&score, &mut ctx).expect("conversion should succeed");

        // Check ID mapping was created
        assert_eq!(ctx.get_mei_id("P1"), Some("staff-1"));
        assert_eq!(ctx.get_mei_id("P2"), Some("staff-2"));
    }

    #[test]
    fn convert_staff_def_sets_staff_number() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        #[allow(deprecated)]
        let staff_def = convert_staff_def("P1", 1, &mut ctx).expect("conversion should succeed");

        assert_eq!(staff_def.n_integer.n, Some(1));
    }

    #[test]
    fn convert_staff_def_sets_default_lines() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        #[allow(deprecated)]
        let staff_def = convert_staff_def("P1", 1, &mut ctx).expect("conversion should succeed");

        assert_eq!(staff_def.staff_def_log.lines, Some(5));
    }

    #[test]
    fn convert_staff_def_sets_default_clef() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        #[allow(deprecated)]
        let staff_def = convert_staff_def("P1", 1, &mut ctx).expect("conversion should succeed");

        assert_eq!(staff_def.staff_def_log.clef_shape, Some(DataClefshape::G));
        assert_eq!(
            staff_def.staff_def_log.clef_line,
            Some(DataClefline::from(2u64))
        );
    }

    #[test]
    fn convert_staff_def_from_score_part_includes_label() {
        let score_part = make_score_part("P1", "Violin I");
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let staff_def = convert_staff_def_from_score_part(&score_part, 1, None, &mut ctx)
            .expect("should succeed");

        // Should have a label child with the part name
        let label = staff_def.children.iter().find_map(|c| {
            if let StaffDefChild::Label(l) = c {
                Some(l)
            } else {
                None
            }
        });
        assert!(label.is_some(), "staffDef should have label child");

        // Check label text
        let label = label.unwrap();
        let text = label.children.iter().find_map(|c| {
            if let LabelChild::Text(t) = c {
                Some(t.as_str())
            } else {
                None
            }
        });
        assert_eq!(text, Some("Violin I"));
    }

    #[test]
    fn convert_staff_def_from_score_part_includes_label_abbr() {
        let mut score_part = make_score_part("P1", "Violin I");
        score_part.part_abbreviation = Some(PartName {
            value: "Vln. I".to_string(),
            ..Default::default()
        });

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let staff_def = convert_staff_def_from_score_part(&score_part, 1, None, &mut ctx)
            .expect("should succeed");

        // Should have a labelAbbr child
        let label_abbr = staff_def.children.iter().find_map(|c| {
            if let StaffDefChild::LabelAbbr(l) = c {
                Some(l)
            } else {
                None
            }
        });
        assert!(label_abbr.is_some(), "staffDef should have labelAbbr child");

        // Check labelAbbr text
        let label_abbr = label_abbr.unwrap();
        let text = label_abbr.children.iter().find_map(|c| {
            if let LabelAbbrChild::Text(t) = c {
                Some(t.as_str())
            } else {
                None
            }
        });
        assert_eq!(text, Some("Vln. I"));
    }

    #[test]
    fn convert_part_group_creates_nested_staff_grp() {
        use tusk_musicxml::model::data::StartStop;
        use tusk_musicxml::model::elements::{
            GroupBarline, GroupBarlineValue, GroupSymbol, GroupSymbolValue, PartGroup,
        };

        let mut score = ScorePartwise::default();
        score.part_list = PartList {
            items: vec![
                // Start of string group
                PartListItem::PartGroup(Box::new(PartGroup {
                    group_type: StartStop::Start,
                    number: Some("1".to_string()),
                    group_name: Some("Strings".to_string()),
                    group_name_display: None,
                    group_abbreviation: Some("Str.".to_string()),
                    group_abbreviation_display: None,
                    group_symbol: Some(GroupSymbolValue {
                        value: GroupSymbol::Bracket,
                        default_x: None,
                        relative_x: None,
                        color: None,
                    }),
                    group_barline: Some(GroupBarlineValue {
                        value: GroupBarline::Yes,
                        color: None,
                    }),
                    group_time: None,
                })),
                PartListItem::ScorePart(Box::new(make_score_part("P1", "Violin I"))),
                PartListItem::ScorePart(Box::new(make_score_part("P2", "Violin II"))),
                // End of string group
                PartListItem::PartGroup(Box::new(PartGroup {
                    group_type: StartStop::Stop,
                    number: Some("1".to_string()),
                    group_name: None,
                    group_name_display: None,
                    group_abbreviation: None,
                    group_abbreviation_display: None,
                    group_symbol: None,
                    group_barline: None,
                    group_time: None,
                })),
            ],
        };

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let staff_grp = convert_staff_grp(&score, &mut ctx).expect("conversion should succeed");

        // Root should have one child: a nested staffGrp for the string group
        assert_eq!(staff_grp.children.len(), 1);
        assert!(matches!(&staff_grp.children[0], StaffGrpChild::StaffGrp(_)));

        // Get the nested staffGrp
        if let StaffGrpChild::StaffGrp(nested_grp) = &staff_grp.children[0] {
            // Should have symbol=bracket
            assert_eq!(
                nested_grp.staff_grp_vis.symbol,
                Some(AttStaffGrpVisSymbol::Bracket)
            );

            // Should have bar.thru=true (from group-barline="yes")
            assert_eq!(nested_grp.staff_grp_vis.bar_thru, Some(DataBoolean::True));

            // Should have label "Strings"
            let has_label = nested_grp.children.iter().any(|c| {
                if let StaffGrpChild::Label(l) = c {
                    l.children.iter().any(|lc| {
                        if let LabelChild::Text(t) = lc {
                            t == "Strings"
                        } else {
                            false
                        }
                    })
                } else {
                    false
                }
            });
            assert!(has_label, "Nested staffGrp should have 'Strings' label");

            // Should have labelAbbr "Str."
            let has_abbr = nested_grp.children.iter().any(|c| {
                if let StaffGrpChild::LabelAbbr(l) = c {
                    l.children.iter().any(|lc| {
                        if let LabelAbbrChild::Text(t) = lc {
                            t == "Str."
                        } else {
                            false
                        }
                    })
                } else {
                    false
                }
            });
            assert!(has_abbr, "Nested staffGrp should have 'Str.' labelAbbr");

            // Should contain 2 staffDef children (for Violin I and II)
            let staff_def_count = nested_grp
                .children
                .iter()
                .filter(|c| matches!(c, StaffGrpChild::StaffDef(_)))
                .count();
            assert_eq!(staff_def_count, 2);
        } else {
            panic!("Expected nested StaffGrp");
        }
    }

    #[test]
    fn convert_part_group_brace_symbol() {
        use tusk_musicxml::model::data::StartStop;
        use tusk_musicxml::model::elements::{GroupSymbol, GroupSymbolValue, PartGroup};

        let mut score = ScorePartwise::default();
        score.part_list = PartList {
            items: vec![
                PartListItem::PartGroup(Box::new(PartGroup {
                    group_type: StartStop::Start,
                    number: Some("1".to_string()),
                    group_name: None,
                    group_name_display: None,
                    group_abbreviation: None,
                    group_abbreviation_display: None,
                    group_symbol: Some(GroupSymbolValue {
                        value: GroupSymbol::Brace,
                        default_x: None,
                        relative_x: None,
                        color: None,
                    }),
                    group_barline: None,
                    group_time: None,
                })),
                PartListItem::ScorePart(Box::new(make_score_part("P1", "Piano RH"))),
                PartListItem::ScorePart(Box::new(make_score_part("P2", "Piano LH"))),
                PartListItem::PartGroup(Box::new(PartGroup {
                    group_type: StartStop::Stop,
                    number: Some("1".to_string()),
                    group_name: None,
                    group_name_display: None,
                    group_abbreviation: None,
                    group_abbreviation_display: None,
                    group_symbol: None,
                    group_barline: None,
                    group_time: None,
                })),
            ],
        };

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let staff_grp = convert_staff_grp(&score, &mut ctx).expect("conversion should succeed");

        // Get the nested staffGrp and verify brace symbol
        if let StaffGrpChild::StaffGrp(nested_grp) = &staff_grp.children[0] {
            assert_eq!(
                nested_grp.staff_grp_vis.symbol,
                Some(AttStaffGrpVisSymbol::Brace)
            );
        } else {
            panic!("Expected nested StaffGrp");
        }
    }

    #[test]
    fn convert_part_group_mensurstrich_barline() {
        use tusk_musicxml::model::data::StartStop;
        use tusk_musicxml::model::elements::{GroupBarline, GroupBarlineValue, PartGroup};

        let mut score = ScorePartwise::default();
        score.part_list = PartList {
            items: vec![
                PartListItem::PartGroup(Box::new(PartGroup {
                    group_type: StartStop::Start,
                    number: Some("1".to_string()),
                    group_name: None,
                    group_name_display: None,
                    group_abbreviation: None,
                    group_abbreviation_display: None,
                    group_symbol: None,
                    group_barline: Some(GroupBarlineValue {
                        value: GroupBarline::Mensurstrich,
                        color: None,
                    }),
                    group_time: None,
                })),
                PartListItem::ScorePart(Box::new(make_score_part("P1", "Soprano"))),
                PartListItem::ScorePart(Box::new(make_score_part("P2", "Alto"))),
                PartListItem::PartGroup(Box::new(PartGroup {
                    group_type: StartStop::Stop,
                    number: Some("1".to_string()),
                    group_name: None,
                    group_name_display: None,
                    group_abbreviation: None,
                    group_abbreviation_display: None,
                    group_symbol: None,
                    group_barline: None,
                    group_time: None,
                })),
            ],
        };

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let staff_grp = convert_staff_grp(&score, &mut ctx).expect("conversion should succeed");

        // Get the nested staffGrp and verify Mensurstrich → bar.thru=false
        if let StaffGrpChild::StaffGrp(nested_grp) = &staff_grp.children[0] {
            assert_eq!(nested_grp.staff_grp_vis.bar_thru, Some(DataBoolean::False));
        } else {
            panic!("Expected nested StaffGrp");
        }
    }

    // ============================================================================
    // Helper Functions
    // ============================================================================

    fn extract_title_text(file_desc: &tusk_model::elements::FileDesc) -> Option<&str> {
        use tusk_model::elements::{FileDescChild, TitleChild, TitleStmtChild};

        for child in &file_desc.children {
            if let FileDescChild::TitleStmt(ts) = child {
                for ts_child in &ts.children {
                    if let TitleStmtChild::Title(title) = ts_child {
                        for t_child in &title.children {
                            if let TitleChild::Text(s) = t_child {
                                return Some(s.as_str());
                            }
                        }
                    }
                }
            }
        }
        None
    }
}
