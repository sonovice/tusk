//! MEI staffGrp to MusicXML part-list conversion.
//!
//! Converts MEI score definition elements to MusicXML part-list structure:
//! - MEI `<staffGrp>` -> MusicXML `<part-group>` (start/stop)
//! - MEI `<staffDef>` -> MusicXML `<score-part>`
//! - MEI `<label>` -> MusicXML `<part-name>`
//! - MEI `<labelAbbr>` -> MusicXML `<part-abbreviation>`

use crate::context::ConversionContext;
use crate::convert_error::ConversionResult;
use crate::model::elements::{PartList, PartListItem, PartName, ScorePart};
use tusk_model::elements::{
    ScoreDef, ScoreDefChild, StaffDef, StaffDefChild, StaffGrp, StaffGrpChild,
};

use super::utils::{
    extract_label_abbr_text, extract_label_text, extract_staff_def_label,
    extract_staff_def_label_abbr,
};

/// Convert MEI scoreDef to MusicXML part-list.
///
/// Maps:
/// - MEI `<staffGrp>` -> MusicXML `<part-group>` (start/stop)
/// - MEI `<staffDef>` -> MusicXML `<score-part>`
/// - MEI `<label>` -> MusicXML `<part-name>`
/// - MEI `<labelAbbr>` -> MusicXML `<part-abbreviation>`
pub fn convert_mei_part_list(
    score_def: &ScoreDef,
    ctx: &mut ConversionContext,
) -> ConversionResult<PartList> {
    let mut part_list = PartList::default();

    // Find staffGrp in scoreDef children
    for child in &score_def.children {
        if let ScoreDefChild::StaffGrp(staff_grp) = child {
            convert_mei_staff_grp_to_part_list(staff_grp, &mut part_list, ctx, 1)?;
        }
    }

    Ok(part_list)
}

/// Recursively convert MEI staffGrp to MusicXML part-list items.
///
/// Returns the next available group number.
pub fn convert_mei_staff_grp_to_part_list(
    staff_grp: &StaffGrp,
    part_list: &mut PartList,
    ctx: &mut ConversionContext,
    group_num: u32,
) -> ConversionResult<u32> {
    use crate::model::data::StartStop;
    use crate::model::elements::PartGroup;

    let mut current_group_num = group_num;

    // Only emit a part-group if this staffGrp has explicit grouping attributes
    // (symbol, bar_thru, or a label). A plain staffGrp with multiple children
    // does NOT need a part-group wrapper — it's just the root container.
    let has_group_attrs = staff_grp.staff_grp_vis.symbol.is_some()
        || staff_grp.staff_grp_vis.bar_thru.is_some()
        || extract_label_text(staff_grp).is_some()
        || extract_label_abbr_text(staff_grp).is_some();
    let needs_group = has_group_attrs;

    // Emit part-group start if needed
    if needs_group {
        let part_group = PartGroup {
            group_type: StartStop::Start,
            number: Some(current_group_num.to_string()),
            group_name: extract_label_text(staff_grp),
            group_name_display: None,
            group_abbreviation: extract_label_abbr_text(staff_grp),
            group_abbreviation_display: None,
            group_symbol: convert_mei_staff_grp_symbol(staff_grp),
            group_barline: convert_mei_staff_grp_barline(staff_grp),
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
                let score_part = convert_mei_staff_def_to_score_part(staff_def, ctx)?;
                part_list
                    .items
                    .push(PartListItem::ScorePart(Box::new(score_part)));
            }
            StaffGrpChild::StaffGrp(nested_grp) => {
                if is_multi_staff_part(nested_grp) {
                    // Multi-staff part: merge all staffDefs into a single ScorePart
                    let score_part = convert_multi_staff_grp_to_score_part(nested_grp, ctx)?;
                    part_list
                        .items
                        .push(PartListItem::ScorePart(Box::new(score_part)));
                } else {
                    current_group_num = convert_mei_staff_grp_to_part_list(
                        nested_grp,
                        part_list,
                        ctx,
                        current_group_num,
                    )?;
                }
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

/// Convert MEI staffGrp @symbol (string) to MusicXML group-symbol.
pub fn convert_mei_staff_grp_symbol(
    staff_grp: &StaffGrp,
) -> Option<crate::model::elements::GroupSymbolValue> {
    use crate::model::elements::{GroupSymbol, GroupSymbolValue};

    staff_grp.staff_grp_vis.symbol.as_ref().map(|sym| {
        let value = match sym.as_str() {
            "brace" => GroupSymbol::Brace,
            "bracket" => GroupSymbol::Bracket,
            "bracketsq" => GroupSymbol::Square,
            "line" => GroupSymbol::Line,
            "none" => GroupSymbol::None,
            _ => GroupSymbol::Brace, // fallback
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
pub fn convert_mei_staff_grp_barline(
    staff_grp: &StaffGrp,
) -> Option<crate::model::elements::GroupBarlineValue> {
    use crate::model::elements::{GroupBarline, GroupBarlineValue};

    staff_grp.staff_grp_vis.bar_thru.as_ref().map(|bar_thru| {
        use tusk_model::data::DataBoolean;
        let value = match bar_thru {
            DataBoolean::True => GroupBarline::Yes,
            DataBoolean::False => GroupBarline::No,
        };
        GroupBarlineValue { value, color: None }
    })
}

/// Detect whether a nested staffGrp represents a multi-staff part (e.g., piano).
///
/// A staffGrp is considered a multi-staff part if:
/// 1. It has `@bar.thru="true"` (staves share barlines — set during import for multi-staff parts)
/// 2. All non-label children are StaffDef elements (no nested StaffGrp)
/// 3. It has at least 2 StaffDef children
/// 4. Individual staffDefs do NOT have their own labels (labels are on the staffGrp)
///    — this distinguishes from grouped separate instruments like "Trombones 1&2"
fn is_multi_staff_part(staff_grp: &StaffGrp) -> bool {
    let has_bar_thru =
        staff_grp.staff_grp_vis.bar_thru == Some(tusk_model::data::DataBoolean::True);

    let staff_defs: Vec<&StaffDef> = staff_grp
        .children
        .iter()
        .filter_map(|c| {
            if let StaffGrpChild::StaffDef(sd) = c {
                Some(sd.as_ref())
            } else {
                None
            }
        })
        .collect();

    let has_nested_grp = staff_grp
        .children
        .iter()
        .any(|c| matches!(c, StaffGrpChild::StaffGrp(_)));

    // Check that individual staffDefs don't have labels
    // (multi-staff parts have labels on the staffGrp, not individual staves)
    let staff_defs_have_labels = staff_defs
        .iter()
        .any(|sd| extract_staff_def_label(sd).is_some());

    has_bar_thru && staff_defs.len() >= 2 && !has_nested_grp && !staff_defs_have_labels
}

/// Convert a multi-staff MEI staffGrp (e.g., piano with brace) to a single MusicXML ScorePart.
///
/// Uses the first staffDef's ID and label for the part identity.
/// Registers all staffDefs in the context for staff number mapping.
fn convert_multi_staff_grp_to_score_part(
    staff_grp: &StaffGrp,
    ctx: &mut ConversionContext,
) -> ConversionResult<ScorePart> {
    // Collect all staffDefs in order
    let staff_defs: Vec<&StaffDef> = staff_grp
        .children
        .iter()
        .filter_map(|c| {
            if let StaffGrpChild::StaffDef(sd) = c {
                Some(sd.as_ref())
            } else {
                None
            }
        })
        .collect();

    let first_def = staff_defs
        .first()
        .expect("multi-staff part must have at least one staffDef");

    // Use the first staffDef's ID as the part ID
    let part_id = first_def
        .basic
        .xml_id
        .clone()
        .or_else(|| first_def.n_integer.n.as_ref().map(|n| format!("P{}", n)))
        .unwrap_or_else(|| ctx.generate_id_with_suffix("part"));

    // Extract label from the staffGrp itself (not individual staffDefs)
    let part_name = extract_label_text(staff_grp)
        .or_else(|| extract_staff_def_label(first_def))
        .unwrap_or_default();

    let mut score_part = ScorePart::new(&part_id, &part_name);

    // Extract abbreviation from the staffGrp
    if let Some(abbr) =
        extract_label_abbr_text(staff_grp).or_else(|| extract_staff_def_label_abbr(first_def))
    {
        score_part.part_abbreviation = Some(PartName {
            value: abbr,
            ..Default::default()
        });
    }

    // Extract part-symbol for multi-staff part and store in context
    extract_part_symbol_from_staff_grp(staff_grp, &part_id, ctx);

    // Extract instrument definitions from first staffDef
    extract_instruments_from_staff_def(first_def, &mut score_part);

    // Register all staffDefs in the context for staff number mapping
    for (idx, staff_def) in staff_defs.iter().enumerate() {
        let local_staff = (idx + 1) as u32;
        let global_staff = staff_def
            .n_integer
            .n
            .as_ref()
            .and_then(|n| n.parse().ok())
            .unwrap_or(local_staff);

        ctx.register_part_staff(&part_id, local_staff, global_staff);

        // Map each staffDef's xml:id to the part ID
        if let Some(ref xml_id) = staff_def.basic.xml_id {
            ctx.map_id(xml_id.clone(), part_id.clone());
        }
    }

    Ok(score_part)
}

/// Convert MEI staffDef to MusicXML score-part.
pub fn convert_mei_staff_def_to_score_part(
    staff_def: &StaffDef,
    ctx: &mut ConversionContext,
) -> ConversionResult<ScorePart> {
    // Generate part ID from staff number or xml:id
    let part_id = staff_def
        .basic
        .xml_id
        .clone()
        .or_else(|| staff_def.n_integer.n.as_ref().map(|n| format!("P{}", n)))
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

    // Extract instrument definitions from instrDef children
    extract_instruments_from_staff_def(staff_def, &mut score_part);

    // Map MEI staffDef ID to MusicXML part ID
    if let Some(ref xml_id) = staff_def.basic.xml_id {
        ctx.map_id(xml_id.clone(), part_id.clone());
    }

    // Register single-staff part in the part-staff map so export can find global staff number
    let global_staff = staff_def
        .n_integer
        .n
        .as_ref()
        .and_then(|n| n.parse().ok())
        .unwrap_or(1);
    ctx.register_part_staff(&part_id, 1, global_staff);

    Ok(score_part)
}

/// Extract MusicXML PartSymbol from a multi-staff MEI staffGrp and store in context.
///
/// Primary: recover full PartSymbol from JSON in @label (`musicxml:part-symbol,{json}`).
/// Fallback: build PartSymbol from @symbol attribute only.
fn extract_part_symbol_from_staff_grp(
    staff_grp: &StaffGrp,
    part_id: &str,
    ctx: &mut ConversionContext,
) {
    use crate::import::parts::PART_SYMBOL_LABEL_PREFIX;
    use crate::model::attributes::{PartSymbol, PartSymbolValue};

    // Try JSON label first (lossless roundtrip)
    if let Some(ref label) = staff_grp.common.label {
        if let Some(json) = label.strip_prefix(PART_SYMBOL_LABEL_PREFIX) {
            if let Ok(ps) = serde_json::from_str::<PartSymbol>(json) {
                ctx.set_part_symbol(part_id, ps);
                return;
            }
        }
    }

    // Fallback: build from @symbol attribute
    if let Some(ref sym) = staff_grp.staff_grp_vis.symbol {
        let value = match sym.as_str() {
            "brace" => PartSymbolValue::Brace,
            "bracket" => PartSymbolValue::Bracket,
            "bracketsq" => PartSymbolValue::Square,
            "line" => PartSymbolValue::Line,
            "none" => PartSymbolValue::None,
            _ => return, // Unknown symbol, don't set
        };
        // Only store non-default (brace is the default for multi-staff parts)
        if value != PartSymbolValue::Brace {
            ctx.set_part_symbol(
                part_id,
                PartSymbol {
                    value,
                    top_staff: None,
                    bottom_staff: None,
                    default_x: None,
                    color: None,
                },
            );
        }
    }
}

/// Extract instrument definitions from MEI instrDef children on a staffDef.
///
/// Recovers MusicXML score-instruments and midi-assignments from instrDef @label JSON.
fn extract_instruments_from_staff_def(staff_def: &StaffDef, score_part: &mut ScorePart) {
    use crate::import::parts::{INSTRUMENT_LABEL_PREFIX, InstrumentData};

    for child in &staff_def.children {
        if let StaffDefChild::InstrDef(instr_def) = child {
            if let Some(ref label) = instr_def.labelled.label {
                if let Some(json) = label.strip_prefix(INSTRUMENT_LABEL_PREFIX) {
                    if let Ok(data) = serde_json::from_str::<InstrumentData>(json) {
                        score_part.score_instruments.push(data.score_instrument);
                        score_part.midi_assignments.extend(data.midi_assignments);
                        continue;
                    }
                }
            }
            // Fallback: no JSON label, skip (shouldn't happen in roundtrip)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ConversionDirection;
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::{Label, LabelAbbr, LabelAbbrChild, LabelChild, StaffDefChild};

    // ========================================================================
    // Part List Conversion Tests
    // ========================================================================

    #[test]
    fn test_convert_single_staff_def() {
        // Create a staffDef with label
        let mut staff_def = StaffDef::default();
        staff_def.basic.xml_id = Some("staff-1".to_string());
        staff_def.n_integer.n = Some("1".to_string());

        let mut label = Label::default();
        label.children.push(LabelChild::Text("Piano".to_string()));
        staff_def
            .children
            .push(StaffDefChild::Label(Box::new(label)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let result = convert_mei_staff_def_to_score_part(&staff_def, &mut ctx);
        assert!(result.is_ok());

        let score_part = result.unwrap();
        assert_eq!(score_part.id, "staff-1");
        assert_eq!(score_part.part_name.value, "Piano");
    }

    #[test]
    fn test_convert_staff_def_with_abbreviation() {
        let mut staff_def = StaffDef::default();
        staff_def.basic.xml_id = Some("staff-1".to_string());
        staff_def.n_integer.n = Some("1".to_string());

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
        let result = convert_mei_staff_def_to_score_part(&staff_def, &mut ctx);
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
        staff_def.n_integer.n = Some("3".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        let result = convert_mei_staff_def_to_score_part(&staff_def, &mut ctx);
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
        staff_def1.n_integer.n = Some("1".to_string());
        let mut label1 = Label::default();
        label1
            .children
            .push(LabelChild::Text("Soprano".to_string()));
        staff_def1
            .children
            .push(StaffDefChild::Label(Box::new(label1)));

        let mut staff_def2 = StaffDef::default();
        staff_def2.basic.xml_id = Some("P2".to_string());
        staff_def2.n_integer.n = Some("2".to_string());
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
        let result = convert_mei_part_list(&score_def, &mut ctx);
        assert!(result.is_ok());

        let part_list = result.unwrap();
        // No symbol/bar_thru/label on staffGrp, so no part-group wrapper — just 2 parts
        assert_eq!(part_list.items.len(), 2);

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
        staff_grp.staff_grp_vis.symbol = Some("brace".to_string());

        let mut staff_def = StaffDef::default();
        staff_def.basic.xml_id = Some("P1".to_string());
        staff_grp
            .children
            .push(StaffGrpChild::StaffDef(Box::new(staff_def)));

        let result = convert_mei_staff_grp_symbol(&staff_grp);
        assert!(result.is_some());

        use crate::model::elements::GroupSymbol;
        assert_eq!(result.unwrap().value, GroupSymbol::Brace);
    }

    #[test]
    fn test_convert_staff_grp_with_bar_thru() {
        let mut staff_grp = StaffGrp::default();
        staff_grp.staff_grp_vis.bar_thru = Some(DataBoolean::True);

        let result = convert_mei_staff_grp_barline(&staff_grp);
        assert!(result.is_some());

        use crate::model::elements::GroupBarline;
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
        outer_grp.staff_grp_vis.symbol = Some("bracket".to_string());

        let mut outer_label = Label::default();
        outer_label
            .children
            .push(LabelChild::Text("Strings".to_string()));
        outer_grp
            .children
            .push(StaffGrpChild::Label(Box::new(outer_label)));

        // Create inner staffGrp (violins) with brace
        let mut inner_grp = StaffGrp::default();
        inner_grp.staff_grp_vis.symbol = Some("brace".to_string());

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
        let result = convert_mei_part_list(&score_def, &mut ctx);
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
    // Symbol Conversion Tests
    // ========================================================================

    #[test]
    fn test_convert_all_staff_grp_symbols() {
        use crate::model::elements::GroupSymbol;

        let test_cases = [
            ("brace", GroupSymbol::Brace),
            ("bracket", GroupSymbol::Bracket),
            ("bracketsq", GroupSymbol::Square),
            ("line", GroupSymbol::Line),
            ("none", GroupSymbol::None),
        ];

        for (mei_sym, expected_mxml) in test_cases {
            let mut staff_grp = StaffGrp::default();
            staff_grp.staff_grp_vis.symbol = Some(mei_sym.to_string());

            let result = convert_mei_staff_grp_symbol(&staff_grp);
            assert!(result.is_some());
            assert_eq!(result.unwrap().value, expected_mxml);
        }
    }

    #[test]
    fn test_convert_bar_thru_values() {
        use crate::model::elements::GroupBarline;

        // Test true -> yes
        let mut staff_grp = StaffGrp::default();
        staff_grp.staff_grp_vis.bar_thru = Some(DataBoolean::True);
        let result = convert_mei_staff_grp_barline(&staff_grp);
        assert_eq!(result.unwrap().value, GroupBarline::Yes);

        // Test false -> no
        let mut staff_grp = StaffGrp::default();
        staff_grp.staff_grp_vis.bar_thru = Some(tusk_model::data::DataBoolean::False);
        let result = convert_mei_staff_grp_barline(&staff_grp);
        assert_eq!(result.unwrap().value, GroupBarline::No);
    }
}
