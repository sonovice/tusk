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
use tusk_model::data::DataBoolean;
use tusk_model::elements::{
    Label, LabelAbbr, LabelAbbrChild, LabelChild, ScoreDef, StaffDef, StaffDefChild, StaffGrp,
    StaffGrpChild,
};

/// Convert MusicXML part-list to MEI scoreDef.
///
/// Also maps MusicXML `<defaults>` layout/font data to MEI scoreDef visual
/// attributes (page dimensions, margins, spacing, fonts, vu.height).
pub fn convert_score_def(
    score: &ScorePartwise,
    ctx: &mut ConversionContext,
) -> ConversionResult<ScoreDef> {
    let mut score_def = ScoreDef::default();

    // Map defaults to scoreDef visual attributes
    if let Some(defaults) = &score.defaults {
        apply_defaults_to_score_def(defaults, &mut score_def);
    }

    // Add pgHead for credits (human-readable text summary for MEI tools)
    if let Some(pg_head) = convert_credits_to_pg_head(&score.credits) {
        score_def
            .children
            .push(tusk_model::elements::ScoreDefChild::PgHead(Box::new(
                pg_head,
            )));
    }

    // Create staffGrp containing staffDef for each part
    let staff_grp = convert_staff_grp(score, ctx)?;
    score_def
        .children
        .push(tusk_model::elements::ScoreDefChild::StaffGrp(Box::new(
            staff_grp,
        )));

    Ok(score_def)
}

/// Convert MusicXML credits to MEI pgHead with text content.
///
/// Creates a pgHead containing AnchoredText elements for each credit-words
/// entry. Credit-image entries are omitted (MEI pgHead doesn't support inline
/// graphics). The full credit data is preserved in extMeta JSON for roundtrip.
fn convert_credits_to_pg_head(
    credits: &[crate::model::elements::Credit],
) -> Option<tusk_model::elements::PgHead> {
    use crate::model::elements::CreditContent;
    use tusk_model::elements::{AnchoredText, AnchoredTextChild, PgHead, PgHeadChild};

    let mut texts = Vec::new();
    for credit in credits {
        if let Some(CreditContent::Words(words)) = &credit.content {
            for w in &words.words {
                if !w.value.is_empty() {
                    let mut anchored = AnchoredText::default();
                    anchored
                        .children
                        .push(AnchoredTextChild::Text(w.value.clone()));
                    texts.push(PgHeadChild::AnchoredText(Box::new(anchored)));
                }
            }
        }
    }

    if texts.is_empty() {
        return None;
    }

    let mut pg_head = PgHead::default();
    pg_head.children = texts;
    Some(pg_head)
}

/// Map MusicXML `<defaults>` fields to MEI scoreDef visual attributes.
///
/// Maps:
/// - `scaling` → `@vu.height` (millimeters-per-half-interline-space)
/// - `page-layout` → `@page.height`, `@page.width`, `@page.topmar`, etc.
/// - `system-layout` → `@system.leftmar`, `@system.rightmar`, `@spacing.system`
/// - `staff-layout` → `@spacing.staff`
/// - `music-font` → `@music.name`, `@music.size`
/// - `word-font` → `@text.fam`, `@text.size`, `@text.style`, `@text.weight`
/// - `lyric-font` (first) → `@lyric.fam`, `@lyric.size`, `@lyric.style`, `@lyric.weight`
fn apply_defaults_to_score_def(
    defaults: &crate::model::elements::Defaults,
    score_def: &mut ScoreDef,
) {
    use tusk_model::data::{
        DataFontfamily, DataMeasurementsigned, DataMeasurementunsigned, DataMusicfont,
    };

    let vis = &mut score_def.score_def_vis;

    // Scaling → vu.height
    // MusicXML scaling defines mm-per-tenths; MEI vu.height = mm for one virtual unit
    // (half interline space). vu.height = millimeters / (tenths / 2) = 2*mm/tenths
    // But MEI typically stores "Xmm" string. Format as "<value>mm".
    if let Some(scaling) = &defaults.scaling {
        if scaling.tenths != 0.0 {
            let vu = scaling.millimeters / (scaling.tenths / 2.0);
            vis.vu_height = Some(format!("{vu:.4}mm"));
        }
    }

    // Page layout → page.height, page.width, page margins
    // MusicXML values are in tenths; MEI uses measurement strings (vu by default).
    // Store as plain numbers (tenths) — vu.height provides the conversion factor.
    if let Some(pl) = &defaults.page_layout {
        if let Some(h) = pl.page_height {
            vis.page_height = Some(DataMeasurementunsigned(format_tenths(h)));
        }
        if let Some(w) = pl.page_width {
            vis.page_width = Some(DataMeasurementunsigned(format_tenths(w)));
        }
        // Use the first "both" margins, or fall back to first margins entry
        if let Some(margins) = pl
            .page_margins
            .iter()
            .find(|m| {
                m.margin_type.is_none()
                    || m.margin_type == Some(crate::model::elements::MarginType::Both)
            })
            .or_else(|| pl.page_margins.first())
        {
            vis.page_topmar = Some(DataMeasurementunsigned(format_tenths(margins.top_margin)));
            vis.page_botmar = Some(DataMeasurementunsigned(format_tenths(
                margins.bottom_margin,
            )));
            vis.page_leftmar = Some(DataMeasurementunsigned(format_tenths(margins.left_margin)));
            vis.page_rightmar = Some(DataMeasurementunsigned(format_tenths(margins.right_margin)));
        }
    }

    // System layout → system margins and spacing
    if let Some(sl) = &defaults.system_layout {
        if let Some(sm) = &sl.system_margins {
            vis.system_leftmar = Some(DataMeasurementunsigned(format_tenths(sm.left_margin)));
            vis.system_rightmar = Some(DataMeasurementunsigned(format_tenths(sm.right_margin)));
        }
        if let Some(sd) = sl.system_distance {
            vis.spacing_system = Some(DataMeasurementsigned(format_tenths(sd)));
        }
        // top-system-distance → system.topmar
        if let Some(tsd) = sl.top_system_distance {
            vis.system_topmar = Some(DataMeasurementunsigned(format_tenths(tsd)));
        }
    }

    // Staff layout → spacing.staff (use first staff-layout with distance, no number filter)
    if let Some(sl) = defaults.staff_layouts.first() {
        if let Some(sd) = sl.staff_distance {
            vis.spacing_staff = Some(DataMeasurementsigned(format_tenths(sd)));
        }
    }

    // Music font → music.name, music.size
    if let Some(mf) = &defaults.music_font {
        if let Some(ref family) = mf.font_family {
            vis.music_name = Some(DataMusicfont(family.clone()));
        }
        if let Some(ref size) = mf.font_size {
            vis.music_size = Some(convert_font_size_to_mei(size));
        }
    }

    // Word font → text.fam, text.size, text.style, text.weight
    if let Some(wf) = &defaults.word_font {
        if let Some(ref family) = wf.font_family {
            vis.text_fam = Some(DataFontfamily(family.clone()));
        }
        if let Some(ref size) = wf.font_size {
            vis.text_size = Some(convert_font_size_to_mei(size));
        }
        if let Some(style) = &wf.font_style {
            vis.text_style = Some(convert_font_style_to_mei(style));
        }
        if let Some(weight) = &wf.font_weight {
            vis.text_weight = Some(convert_font_weight_to_mei(weight));
        }
    }

    // Lyric font (first entry) → lyric.fam, lyric.size, lyric.style, lyric.weight
    if let Some(lf) = defaults.lyric_fonts.first() {
        if let Some(ref family) = lf.font_family {
            vis.lyric_fam = Some(DataFontfamily(family.clone()));
        }
        if let Some(ref size) = lf.font_size {
            vis.lyric_size = Some(convert_font_size_to_mei(size));
        }
        if let Some(style) = &lf.font_style {
            vis.lyric_style = Some(convert_font_style_to_mei(style));
        }
        if let Some(weight) = &lf.font_weight {
            vis.lyric_weight = Some(convert_font_weight_to_mei(weight));
        }
    }
}

/// Format a tenths value as a string, stripping trailing zeros.
fn format_tenths(value: f64) -> String {
    // If the value is an integer, format without decimals
    if value == value.trunc() {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

/// Convert MusicXML FontSize to MEI DataFontsize.
fn convert_font_size_to_mei(size: &crate::model::data::FontSize) -> tusk_model::data::DataFontsize {
    use crate::model::data::{CssFontSize, FontSize};
    use tusk_model::data::{DataFontsize, DataFontsizenumeric, DataFontsizeterm};
    match size {
        FontSize::Points(pts) => {
            DataFontsize::MeiDataFontsizenumeric(DataFontsizenumeric(format!("{pts}")))
        }
        FontSize::Css(css) => DataFontsize::MeiDataFontsizeterm(match css {
            CssFontSize::XxSmall => DataFontsizeterm::XxSmall,
            CssFontSize::XSmall => DataFontsizeterm::XSmall,
            CssFontSize::Small => DataFontsizeterm::Small,
            CssFontSize::Medium => DataFontsizeterm::Normal,
            CssFontSize::Large => DataFontsizeterm::Large,
            CssFontSize::XLarge => DataFontsizeterm::XLarge,
            CssFontSize::XxLarge => DataFontsizeterm::XxLarge,
        }),
    }
}

/// Convert MusicXML FontStyle to MEI DataFontstyle.
fn convert_font_style_to_mei(
    style: &crate::model::data::FontStyle,
) -> tusk_model::data::DataFontstyle {
    match style {
        crate::model::data::FontStyle::Normal => tusk_model::data::DataFontstyle::Normal,
        crate::model::data::FontStyle::Italic => tusk_model::data::DataFontstyle::Italic,
    }
}

/// Convert MusicXML FontWeight to MEI DataFontweight.
fn convert_font_weight_to_mei(
    weight: &crate::model::data::FontWeight,
) -> tusk_model::data::DataFontweight {
    match weight {
        crate::model::data::FontWeight::Normal => tusk_model::data::DataFontweight::Normal,
        crate::model::data::FontWeight::Bold => tusk_model::data::DataFontweight::Bold,
    }
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

                // Detect multi-staff parts (e.g., piano with <staves>2</staves>)
                let multi_staff_attrs = extract_attributes_with_staves(score, &score_part.id);
                let num_staves = multi_staff_attrs
                    .and_then(|a| a.staves)
                    .or_else(|| initial_attrs.and_then(|a| a.staves))
                    .unwrap_or(1);

                if num_staves > 1 {
                    // Multi-staff part: create a nested staffGrp with multiple staffDefs
                    // Use the staves-containing attrs for clefs, but prefer initial_attrs
                    // for divisions (the first attributes block has the operational divisions).
                    let clef_attrs = multi_staff_attrs.or(initial_attrs);
                    let div_attrs = initial_attrs.or(multi_staff_attrs);
                    let child = convert_multi_staff_part(
                        score_part,
                        staff_number,
                        num_staves,
                        clef_attrs,
                        div_attrs,
                        ctx,
                    )?;

                    // Add nested staffGrp to innermost open group, or root
                    if let Some((_, grp)) = group_stack.last_mut() {
                        grp.children.push(child);
                    } else {
                        root_grp.children.push(child);
                    }

                    // Map part ID to first staff number
                    ctx.map_id(&score_part.id, format!("staff-{}", staff_number));
                    staff_number += num_staves;
                } else {
                    // Single-staff part: existing path
                    let staff_def = convert_staff_def_from_score_part(
                        score_part,
                        staff_number,
                        initial_attrs,
                        None,
                        true,
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

                    // Map part ID to staff number, register in part-staff map
                    ctx.map_id(&score_part.id, format!("staff-{}", staff_number));
                    ctx.register_part_staff(&score_part.id, 1, staff_number);
                    staff_number += 1;
                }
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
        staff_grp.staff_grp_vis.symbol = Some(convert_group_symbol(symbol_value.value));
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
/// - part-name → `<label>` child (when `include_label` is true)
/// - part-abbreviation → `<labelAbbr>` child (when `include_label` is true)
/// - Staff number → `@n`
/// - Default clef and lines
/// - Initial key/time/clef from first measure attributes
///
/// `clef_number`: which MusicXML clef `@number` to select from attributes.
/// For single-staff parts, pass `None` (selects number=1 or unnumbered).
/// For multi-staff parts, pass `Some(local_staff)` to pick the right clef.
///
/// `include_label`: whether to add label/labelAbbr children to this staffDef.
/// For multi-staff parts, labels go on the containing staffGrp instead.
pub fn convert_staff_def_from_score_part(
    score_part: &ScorePart,
    staff_number: u32,
    initial_attrs: Option<&crate::model::attributes::Attributes>,
    clef_number: Option<u32>,
    include_label: bool,
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
            match &key.content {
                KeyContent::Traditional(trad) => {
                    let keysig = convert_key_fifths(trad.fifths);
                    staff_def.staff_def_log.keysig = Some(keysig);
                    // Store full Key as JSON if it has key_octaves
                    if !key.key_octaves.is_empty() {
                        if let Ok(json) = serde_json::to_string(key) {
                            crate::import::attributes::append_label(
                                &mut staff_def,
                                format!(
                                    "{}{}",
                                    crate::import::attributes::KEY_LABEL_PREFIX,
                                    json
                                ),
                            );
                        }
                    }
                }
                KeyContent::NonTraditional(_) => {
                    // No MEI @keysig equivalent; store full Key as JSON
                    if let Ok(json) = serde_json::to_string(key) {
                        crate::import::attributes::append_label(
                            &mut staff_def,
                            format!(
                                "{}{}",
                                crate::import::attributes::KEY_LABEL_PREFIX,
                                json
                            ),
                        );
                    }
                }
            }
        }

        // Apply time signature
        if let Some(time) = attrs.times.first() {
            let (count, unit, sym) = convert_time_signature(time);
            staff_def.staff_def_log.meter_count = count;
            staff_def.staff_def_log.meter_unit = unit.map(|u| u.to_string());
            staff_def.staff_def_log.meter_sym = sym;

            // Store full Time as JSON if it has interchangeable or separator
            let has_extra = matches!(&time.content,
                crate::model::attributes::TimeContent::Standard(std) if std.interchangeable.is_some())
                || time.separator.is_some();
            if has_extra {
                if let Ok(json) = serde_json::to_string(time) {
                    crate::import::attributes::append_label(
                        &mut staff_def,
                        format!("{}{}", crate::import::attributes::TIME_LABEL_PREFIX, json),
                    );
                }
            }
        }

        // Apply clef (overrides default)
        // MusicXML clef@number is 1-based within the part, not global across all parts.
        // For multi-staff parts, each staff has its own clef identified by @number.
        let clef = match clef_number {
            Some(n) => attrs
                .clefs
                .iter()
                .find(|c| c.number == Some(n) || (n == 1 && c.number.is_none()))
                .or_else(|| attrs.clefs.first()),
            None => attrs
                .clefs
                .iter()
                .find(|c| c.number.is_none() || c.number == Some(1))
                .or_else(|| attrs.clefs.first()),
        };

        if let Some(clef) = clef {
            let (shape, line, dis, dis_place) = convert_clef_attributes(clef);
            staff_def.staff_def_log.clef_shape = shape;
            staff_def.staff_def_log.clef_line = line;
            staff_def.staff_def_log.clef_dis = dis;
            staff_def.staff_def_log.clef_dis_place = dis_place;
        }

        // Apply staff details
        // Pick matching staff number or first entry
        let sd = match clef_number {
            Some(n) => attrs
                .staff_details
                .iter()
                .find(|sd| sd.number == Some(n) || (n == 1 && sd.number.is_none()))
                .or_else(|| attrs.staff_details.first()),
            None => attrs
                .staff_details
                .iter()
                .find(|sd| sd.number.is_none() || sd.number == Some(1))
                .or_else(|| attrs.staff_details.first()),
        };
        if let Some(sd) = sd {
            apply_staff_details_to_staff_def(sd, &mut staff_def);
        }
    }

    // Use the original MusicXML part ID as the staffDef xml:id
    // This preserves the ID through the roundtrip conversion
    staff_def.basic.xml_id = Some(score_part.id.clone());

    if include_label {
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
    }

    Ok(staff_def)
}

/// Label prefix for staff-details JSON stored on staffDef @label.
const STAFF_DETAILS_LABEL_PREFIX: &str = "musicxml:staff-details,";

/// Label prefix for part-symbol JSON stored on multi-staff staffGrp @label.
pub(crate) const PART_SYMBOL_LABEL_PREFIX: &str = "musicxml:part-symbol,";

/// Apply MusicXML StaffDetails to a MEI StaffDef.
///
/// Maps semantic fields to MEI attributes:
/// - staff_lines → @lines
/// - staff_size → @scale (percentage string)
///
/// Stores the full StaffDetails as JSON in @label for lossless roundtrip of
/// all fields (staff_type, line_details, staff_tunings, capo, show_frets, etc.).
fn apply_staff_details_to_staff_def(
    sd: &crate::model::attributes::StaffDetails,
    staff_def: &mut StaffDef,
) {
    // Map staff_lines → @lines
    if let Some(lines) = sd.staff_lines {
        staff_def.staff_def_log.lines = Some(lines.to_string());
    }

    // Map staff_size → @scale as percentage
    if let Some(ref ss) = sd.staff_size {
        staff_def.staff_def_vis.scale =
            Some(tusk_model::data::DataPercent(format!("{}%", ss.value)));
    }

    // Store full StaffDetails as JSON in @label for lossless roundtrip
    // Only store if there's meaningful data beyond just staff_lines
    let has_extra = sd.staff_type.is_some()
        || !sd.line_details.is_empty()
        || !sd.staff_tunings.is_empty()
        || sd.capo.is_some()
        || sd.staff_size.is_some()
        || sd.show_frets.is_some()
        || sd.print_object.is_some()
        || sd.print_spacing.is_some();

    if has_extra {
        // Clear the number field — it's handled via MEI @n / part mapping
        let mut sd_for_json = sd.clone();
        sd_for_json.number = None;
        if let Ok(json) = serde_json::to_string(&sd_for_json) {
            crate::import::attributes::append_label(
                staff_def,
                format!("{}{}", STAFF_DETAILS_LABEL_PREFIX, json),
            );
        }
    }
}

/// Convert a multi-staff MusicXML part to a nested MEI staffGrp with multiple staffDefs.
///
/// For parts with `<staves>N</staves>` (N > 1), creates:
/// - A `<staffGrp>` with `@symbol="brace"` and `@bar.thru="true"`
/// - N `<staffDef>` children, each with the appropriate clef
/// - Label/labelAbbr on the staffGrp (not individual staffDefs)
fn convert_multi_staff_part(
    score_part: &ScorePart,
    first_staff_number: u32,
    num_staves: u32,
    clef_attrs: Option<&crate::model::attributes::Attributes>,
    div_attrs: Option<&crate::model::attributes::Attributes>,
    ctx: &mut ConversionContext,
) -> ConversionResult<StaffGrpChild> {
    let mut nested_grp = StaffGrp::default();

    // Set symbol and bar.thru for the multi-staff group
    // Default to brace (piano/keyboard style) — can be overridden by part-symbol in attributes
    if let Some(attrs) = clef_attrs
        && let Some(ref ps) = attrs.part_symbol
    {
        use crate::model::attributes::PartSymbolValue;
        nested_grp.staff_grp_vis.symbol = Some(match ps.value {
            PartSymbolValue::Brace => "brace".to_string(),
            PartSymbolValue::Bracket => "bracket".to_string(),
            PartSymbolValue::Square => "bracketsq".to_string(),
            PartSymbolValue::Line => "line".to_string(),
            PartSymbolValue::None => "none".to_string(),
        });

        // Store full PartSymbol as JSON in @label for lossless roundtrip
        // (preserves top-staff, bottom-staff, default-x, color)
        let has_extra = ps.top_staff.is_some()
            || ps.bottom_staff.is_some()
            || ps.default_x.is_some()
            || ps.color.is_some();
        if has_extra {
            if let Ok(json) = serde_json::to_string(ps) {
                nested_grp.common.label = Some(format!("{}{}", PART_SYMBOL_LABEL_PREFIX, json));
            }
        }
    } else {
        nested_grp.staff_grp_vis.symbol = Some("brace".to_string());
    }
    nested_grp.staff_grp_vis.bar_thru = Some(DataBoolean::True);

    // Generate ID for the staffGrp
    let grp_id = ctx.generate_id_with_suffix("staffgrp");
    nested_grp.common.xml_id = Some(grp_id);

    // Add label/labelAbbr to the staffGrp (not individual staffDefs)
    if !score_part.part_name.value.is_empty() {
        let mut label = Label::default();
        label
            .children
            .push(LabelChild::Text(score_part.part_name.value.clone()));
        nested_grp
            .children
            .push(StaffGrpChild::Label(Box::new(label)));
    }
    if let Some(ref abbr) = score_part.part_abbreviation
        && !abbr.value.is_empty()
    {
        let mut label_abbr = LabelAbbr::default();
        label_abbr
            .children
            .push(LabelAbbrChild::Text(abbr.value.clone()));
        nested_grp
            .children
            .push(StaffGrpChild::LabelAbbr(Box::new(label_abbr)));
    }

    // Set divisions from div_attrs (the first attributes block has operational divisions)
    let divs = div_attrs.and_then(|a| a.divisions).unwrap_or(1.0);
    ctx.set_divisions(divs);

    // Create staffDefs for each staff in the part
    for local_staff in 1..=num_staves {
        let global_staff = first_staff_number + local_staff - 1;

        let mut staff_def = convert_staff_def_from_score_part(
            score_part,
            global_staff,
            clef_attrs,
            Some(local_staff),
            false, // labels go on the staffGrp, not individual staffDefs
            ctx,
        )?;

        // Override ppq with the operational divisions (from first attributes block)
        staff_def.staff_def_ges.ppq = Some((divs as u64).to_string());

        // Set xml:id: first staff keeps original part ID, others get suffixed
        if local_staff == 1 {
            staff_def.basic.xml_id = Some(score_part.id.clone());
        } else {
            staff_def.basic.xml_id = Some(format!("{}-staff-{}", score_part.id, local_staff));
        }

        nested_grp
            .children
            .push(StaffGrpChild::StaffDef(Box::new(staff_def)));

        // Register in part-staff map
        ctx.register_part_staff(&score_part.id, local_staff, global_staff);
    }

    Ok(StaffGrpChild::StaffGrp(Box::new(nested_grp)))
}

/// Extract the Attributes element from a part's first measure that contains a `<staves>` value.
///
/// Unlike `extract_first_measure_attributes` which returns the first Attributes block,
/// this scans ALL Attributes blocks in the first measure to find one with `staves` set.
/// This is needed because multi-staff declarations may appear in a later Attributes block
/// (e.g., `staves_element.musicxml` has staves in the second `<attributes>` element).
fn extract_attributes_with_staves<'a>(
    score: &'a ScorePartwise,
    part_id: &str,
) -> Option<&'a crate::model::attributes::Attributes> {
    use crate::model::elements::MeasureContent;

    let part = score.parts.iter().find(|p| p.id == part_id)?;
    let first_measure = part.measures.first()?;

    for content in &first_measure.content {
        if let MeasureContent::Attributes(attrs) = content {
            if attrs.staves.is_some() {
                return Some(attrs.as_ref());
            }
        }
    }

    None
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
        let staff_def =
            convert_staff_def_from_score_part(&score_part, 1, None, None, true, &mut ctx)
                .expect("conversion should succeed");

        assert_eq!(staff_def.n_integer.n.as_deref(), Some("1"));
    }

    #[test]
    fn convert_staff_def_sets_default_lines() {
        let score_part = make_score_part("P1", "");
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let staff_def =
            convert_staff_def_from_score_part(&score_part, 1, None, None, true, &mut ctx)
                .expect("conversion should succeed");

        assert_eq!(staff_def.staff_def_log.lines.as_deref(), Some("5"));
    }

    #[test]
    fn convert_staff_def_sets_default_clef() {
        let score_part = make_score_part("P1", "");
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let staff_def =
            convert_staff_def_from_score_part(&score_part, 1, None, None, true, &mut ctx)
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
        let staff_def =
            convert_staff_def_from_score_part(&score_part, 1, None, None, true, &mut ctx)
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
        let text = label
            .children
            .iter()
            .map(|c| {
                let LabelChild::Text(t) = c;
                t.as_str()
            })
            .next();
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
        let staff_def =
            convert_staff_def_from_score_part(&score_part, 1, None, None, true, &mut ctx)
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
        let text = label_abbr
            .children
            .iter()
            .map(|c| {
                let LabelAbbrChild::Text(t) = c;
                t.as_str()
            })
            .next();
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
