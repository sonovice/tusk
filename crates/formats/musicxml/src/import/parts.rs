//! Part-list and staffGrp conversion from MusicXML to MEI.
//!
//! This module handles conversion of:
//! - MusicXML `<part-list>` → MEI `<scoreDef>` with `<staffGrp>`
//! - MusicXML `<score-part>` → MEI `<staffDef>`
//! - MusicXML `<part-group>` → nested MEI `<staffGrp>`

use crate::context::ConversionContext;
use crate::convert_error::ConversionResult;
use crate::import::{
    convert_clef_attributes, convert_key_fifths, convert_key_to_context, convert_time_signature,
};
use crate::model::attributes::KeyContent;
use crate::model::elements::{PartGroup, PartListItem, ScorePart, ScorePartwise};
use tusk_model::elements::{
    Label, LabelAbbr, LabelAbbrChild, LabelChild, ScoreDef, StaffDef, StaffDefChild, StaffGrp,
    StaffGrpChild,
};

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
                    crate::model::data::StartStop::Start => {
                        // Start a new group
                        let new_grp = convert_staff_grp_from_part_group(part_group, ctx)?;
                        group_stack.push((group_number, new_grp));
                    }
                    crate::model::data::StartStop::Stop => {
                        // Find and close the matching group
                        if let Some(idx) = group_stack
                            .iter()
                            .rposition(|(num, _)| num == &group_number)
                        {
                            // Move any groups pushed AFTER this one (higher indices) into this group
                            // This handles cases like:
                            //   <part-group 2 start>
                            //   <part>P14</part>
                            //   <part-group 1 start>
                            //   <part>P15</part>
                            //   <part-group 2 stop>  -- group 1 should be nested inside group 2
                            while group_stack.len() > idx + 1 {
                                let (_, inner_grp) = group_stack.pop().unwrap();
                                if let Some((_, outer_grp)) = group_stack.get_mut(idx) {
                                    outer_grp
                                        .children
                                        .push(StaffGrpChild::StaffGrp(Box::new(inner_grp)));
                                }
                            }

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
        staff_grp.staff_grp_vis.symbol = Some(convert_group_symbol(symbol_value.value.clone()));
    }

    // Convert group barline → bar.thru
    if let Some(ref barline_value) = part_group.group_barline {
        use crate::model::elements::GroupBarline;
        use tusk_model::data::DataBoolean;
        staff_grp.staff_grp_vis.bar_thru = Some(match barline_value.value {
            GroupBarline::Yes => DataBoolean::True,
            GroupBarline::No | GroupBarline::Mensurstrich => DataBoolean::False,
        });
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

/// Convert MusicXML GroupSymbol to MEI @symbol string.
fn convert_group_symbol(symbol: crate::model::elements::GroupSymbol) -> String {
    use crate::model::elements::GroupSymbol;

    match symbol {
        GroupSymbol::Brace => "brace".to_string(),
        GroupSymbol::Bracket => "bracket".to_string(),
        GroupSymbol::Square => "bracketsq".to_string(),
        GroupSymbol::Line => "line".to_string(),
        GroupSymbol::None => "none".to_string(),
    }
}

/// Convert MusicXML GroupBarline to MEI DataBoolean for bar.thru attribute.
#[allow(dead_code)]
fn convert_group_barline_to_string(barline: crate::model::elements::GroupBarline) -> String {
    use crate::model::elements::GroupBarline;

    match barline {
        GroupBarline::Yes => "true".to_string(),
        GroupBarline::No | GroupBarline::Mensurstrich => "false".to_string(),
    }
}

/// Extract the first Attributes element from a MusicXML part's first measure.
///
/// This is used to initialize the staffDef with correct key/time/clef from the score.
fn extract_first_measure_attributes<'a>(
    score: &'a ScorePartwise,
    part_id: &str,
) -> Option<&'a crate::model::attributes::Attributes> {
    use crate::model::elements::MeasureContent;

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
pub fn convert_staff_def_from_score_part(
    score_part: &ScorePart,
    staff_number: u32,
    initial_attrs: Option<&crate::model::attributes::Attributes>,
    ctx: &mut ConversionContext,
) -> ConversionResult<StaffDef> {
    let mut staff_def = StaffDef::default();

    staff_def.n_integer.n = Some((staff_number as u64).to_string());

    staff_def.staff_def_log.lines = Some("5".to_string());

    staff_def.staff_def_log.clef_shape = Some(tusk_model::data::DataClefshape::G);
    staff_def.staff_def_log.clef_line = Some(tusk_model::data::DataClefline::from(2));

    let divs = initial_attrs.and_then(|a| a.divisions).unwrap_or(1.0);
    ctx.set_divisions(divs);
    staff_def.staff_def_ges.ppq = Some((divs as u64).to_string());

    // Apply initial attributes from the first measure (key, time, clef)
    if let Some(attrs) = initial_attrs {
        // Apply key signature
        if let Some(key) = attrs.keys.first() {
            convert_key_to_context(key, ctx);
            if let KeyContent::Traditional(trad) = &key.content {
                let keysig = convert_key_fifths(trad.fifths);
                staff_def.staff_def_log.keysig = Some(keysig);
            }
        }

        // Apply time signature
        if let Some(time) = attrs.times.first() {
            let (count, unit, sym) = convert_time_signature(time);
            staff_def.staff_def_log.meter_count = count;
            staff_def.staff_def_log.meter_unit = unit.map(|u| u.to_string());
            staff_def.staff_def_log.meter_sym = sym;
        }

        // Apply clef (overrides default)
        // For single-staff parts or first staff of multi-staff parts, use clef number=1 or None
        // Note: MusicXML clef@number is 1-based within the part, not global across all parts
        // TODO: Multi-staff parts (like piano) should create multiple staffDef elements
        let clef = attrs
            .clefs
            .iter()
            .find(|c| c.number.is_none() || c.number == Some(1))
            .or_else(|| attrs.clefs.first());

        if let Some(clef) = clef {
            let (shape, line, dis, dis_place) = convert_clef_attributes(clef);
            staff_def.staff_def_log.clef_shape = shape;
            staff_def.staff_def_log.clef_line = line;
            staff_def.staff_def_log.clef_dis = dis;
            staff_def.staff_def_log.clef_dis_place = dis_place;
        }
    }

    // Use the original MusicXML part ID as the staffDef xml:id
    // This preserves the ID through the roundtrip conversion
    staff_def.basic.xml_id = Some(score_part.id.clone());

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ConversionDirection;
    use crate::import::test_utils::make_score_part;
    use crate::model::elements::{PartList, PartListItem, PartName};

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
        let score_part = make_score_part("P1", "");
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let staff_def = convert_staff_def_from_score_part(&score_part, 1, None, &mut ctx)
            .expect("conversion should succeed");

        assert_eq!(staff_def.n_integer.n.as_deref(), Some("1"));
    }

    #[test]
    fn convert_staff_def_sets_default_lines() {
        let score_part = make_score_part("P1", "");
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let staff_def = convert_staff_def_from_score_part(&score_part, 1, None, &mut ctx)
            .expect("conversion should succeed");

        assert_eq!(staff_def.staff_def_log.lines.as_deref(), Some("5"));
    }

    #[test]
    fn convert_staff_def_sets_default_clef() {
        let score_part = make_score_part("P1", "");
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let staff_def = convert_staff_def_from_score_part(&score_part, 1, None, &mut ctx)
            .expect("conversion should succeed");

        assert_eq!(
            staff_def.staff_def_log.clef_shape,
            Some(tusk_model::data::DataClefshape::G)
        );
        assert_eq!(
            staff_def.staff_def_log.clef_line,
            Some(tusk_model::data::DataClefline::from(2))
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
            let LabelChild::Text(t) = c;
            Some(t.as_str())
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
            let LabelAbbrChild::Text(t) = c;
            Some(t.as_str())
        });
        assert_eq!(text, Some("Vln. I"));
    }

    #[test]
    fn convert_part_group_creates_nested_staff_grp() {
        use crate::model::data::StartStop;
        use crate::model::elements::{
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
            assert_eq!(nested_grp.staff_grp_vis.symbol, Some("bracket".to_string()));

            // Should have bar.thru=true (from group-barline="yes")
            assert_eq!(
                nested_grp.staff_grp_vis.bar_thru,
                Some(tusk_model::data::DataBoolean::True)
            );

            // Should have label "Strings"
            let has_label = nested_grp.children.iter().any(|c| {
                let StaffGrpChild::Label(l) = c else {
                    return false;
                };
                l.children.iter().any(|lc| {
                    let LabelChild::Text(t) = lc;
                    t == "Strings"
                })
            });
            assert!(has_label, "Nested staffGrp should have 'Strings' label");

            // Should have labelAbbr "Str."
            let has_abbr = nested_grp.children.iter().any(|c| {
                let StaffGrpChild::LabelAbbr(l) = c else {
                    return false;
                };
                l.children.iter().any(|lc| {
                    let LabelAbbrChild::Text(t) = lc;
                    t == "Str."
                })
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
        use crate::model::data::StartStop;
        use crate::model::elements::{GroupSymbol, GroupSymbolValue, PartGroup};

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
            assert_eq!(nested_grp.staff_grp_vis.symbol, Some("brace".to_string()));
        } else {
            panic!("Expected nested StaffGrp");
        }
    }

    #[test]
    fn convert_part_group_mensurstrich_barline() {
        use crate::model::data::StartStop;
        use crate::model::elements::{GroupBarline, GroupBarlineValue, PartGroup};

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
            assert_eq!(
                nested_grp.staff_grp_vis.bar_thru,
                Some(tusk_model::data::DataBoolean::False)
            );
        } else {
            panic!("Expected nested StaffGrp");
        }
    }
}
