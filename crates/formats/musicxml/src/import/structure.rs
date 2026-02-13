//! Measure/part/score structure conversion from MusicXML to MEI.
//!
//! This module handles conversion of structural elements:
//! - `<body>` containing `<mdiv>`
//! - `<mdiv>` containing `<score>`
//! - `<score>` containing `<scoreDef>` and `<section>`
//! - `<section>` containing `<measure>`
//! - `<measure>` containing `<staff>` and control events
//! - `<staff>` containing `<layer>`
//! - `<layer>` containing notes, rests, chords

use crate::context::ConversionContext;
use crate::convert_error::ConversionResult;
use crate::import::restructure::{
    emit_inline_attribute_changes, restructure_with_beams, wrap_tremolo_containers,
};
use crate::import::{
    DirectionConversionResult, convert_chord, convert_direction, convert_measure_rest,
    convert_note, convert_rest, convert_score_def, is_measure_rest,
};
use crate::model::elements::ScorePartwise;
use tusk_model::data::{DataBoolean, DataMeasurementunsigned};
use tusk_model::elements::{
    Body, BodyChild, LayerChild, Mdiv, MdivChild, MeasureChild, Score, ScoreChild, Section,
    SectionChild, Slur, StaffChild, TupletSpan,
};

/// Convert MusicXML content to MEI body.
pub fn convert_body(score: &ScorePartwise, ctx: &mut ConversionContext) -> ConversionResult<Body> {
    let mut body = Body::default();

    // Create mdiv containing the score
    let mdiv = convert_mdiv(score, ctx)?;
    body.children.push(BodyChild::Mdiv(Box::new(mdiv)));

    Ok(body)
}

/// Convert MusicXML score to MEI mdiv.
pub fn convert_mdiv(score: &ScorePartwise, ctx: &mut ConversionContext) -> ConversionResult<Mdiv> {
    let mut mdiv = Mdiv::default();

    // Create score element
    let mei_score = convert_score_content(score, ctx)?;
    mdiv.children.push(MdivChild::Score(Box::new(mei_score)));

    Ok(mdiv)
}

/// Convert MusicXML score content to MEI score element.
pub fn convert_score_content(
    score: &ScorePartwise,
    ctx: &mut ConversionContext,
) -> ConversionResult<Score> {
    let mut mei_score = Score::default();

    // Create scoreDef with staffGrp from part-list
    let score_def = convert_score_def(score, ctx)?;
    mei_score
        .children
        .push(ScoreChild::ScoreDef(Box::new(score_def)));

    // Create section containing measures
    let section = convert_section(score, ctx)?;
    mei_score
        .children
        .push(ScoreChild::Section(Box::new(section)));

    Ok(mei_score)
}

/// Convert MusicXML measures to MEI section.
pub fn convert_section(
    score: &ScorePartwise,
    ctx: &mut ConversionContext,
) -> ConversionResult<Section> {
    let mut section = Section::default();

    // Get the number of measures from the first part (all parts should have same count)
    let measure_count = score.parts.first().map(|p| p.measures.len()).unwrap_or(0);

    // Process measures
    // In MEI, measures contain staves; in MusicXML, parts contain measures.
    // We need to transpose this: for each measure number, collect content from all parts.
    for measure_idx in 0..measure_count {
        let mei_measure = convert_measure(score, measure_idx, ctx)?;
        section
            .children
            .push(SectionChild::Measure(Box::new(mei_measure)));
    }

    // Post-pass: patch completed hairpins with tstamp2.
    // Completed hairpins reference hairpin elements in earlier measures by ID.
    patch_hairpin_tstamp2(&mut section, ctx);

    Ok(section)
}

/// Convert a MusicXML measure (from all parts) to MEI measure.
///
/// Converts MusicXML measure attributes to MEI:
/// - `number` → MEI `@n` (measure number/label)
/// - `implicit="yes"` → MEI `@metcon="false"` (incomplete/pickup measure)
/// - `width` → MEI `@width` (measure width for layout)
/// - `id` → MEI `xml:id` (element ID)
/// - `non_controlling="yes"` → MEI `@control="false"` (non-controlling barline)
pub fn convert_measure(
    score: &ScorePartwise,
    measure_idx: usize,
    ctx: &mut ConversionContext,
) -> ConversionResult<tusk_model::elements::Measure> {
    use tusk_model::elements::Measure;

    let mut mei_measure = Measure::default();

    // Track measure index for cross-measure spanner resolution (e.g. hairpin tstamp2)
    ctx.set_measure_idx(measure_idx);

    // Get measure from first part to extract common attributes and barlines
    if let Some(first_part) = score.parts.first()
        && let Some(musicxml_measure) = first_part.measures.get(measure_idx)
    {
        // Convert measure attributes
        convert_measure_attributes(musicxml_measure, &mut mei_measure, ctx);
        // Convert barlines (left/right) from first part's measure content
        convert_measure_barlines(musicxml_measure, &mut mei_measure, ctx);
        ctx.set_measure(&musicxml_measure.number);
    }

    // Phase 1: Create staff elements for each part (notes, rests, chords)
    // Multi-staff parts (e.g., piano with <staves>2</staves>) keep all notes
    // in a single MEI staff (the first global staff). This preserves the
    // original voice/backup ordering which is essential for roundtrip fidelity,
    // especially with cross-staff notes.
    for part in &score.parts {
        ctx.set_part(&part.id);

        if let Some(musicxml_measure) = part.measures.get(measure_idx) {
            let global_staff = ctx.global_staff_for_part(&part.id, 1).unwrap_or(1);
            ctx.set_staff(global_staff);
            let staff = convert_staff(musicxml_measure, global_staff, ctx)?;
            mei_measure
                .children
                .push(MeasureChild::Staff(Box::new(staff)));
        }
    }

    // Phase 2: Convert directions to control events (after all staves).
    // Separate from staff creation to ensure canonical MEI ordering:
    // all <staff> children first, then all control events.
    // For multi-staff parts, direction.staff (within-part) is resolved to global MEI staff.
    for part in &score.parts {
        ctx.set_part(&part.id);
        let num_staves = ctx.staves_for_part(&part.id);
        // Default to global staff for local staff 1
        let default_global_staff = ctx.global_staff_for_part(&part.id, 1).unwrap_or(1);
        ctx.set_staff(default_global_staff);

        if let Some(musicxml_measure) = part.measures.get(measure_idx) {
            convert_measure_directions(musicxml_measure, &mut mei_measure, num_staves, ctx)?;
        }
    }

    // Emit completed slurs as MEI control events
    emit_slurs(&mut mei_measure, ctx);

    // Emit completed tuplets as MEI control events
    emit_tuplet_spans(&mut mei_measure, ctx);

    // Emit ornament control events (trill, mordent, turn, ornam, fermata, arpeg)
    emit_ornament_events(&mut mei_measure, ctx);

    // Emit completed glissando/slide control events
    emit_gliss_events(&mut mei_measure, ctx);

    Ok(mei_measure)
}

/// Convert MusicXML directions in a measure to MEI control events.
///
/// Iterates all measure content to track beat position correctly.
/// Notes/rests advance beat_position, backup/forward adjust it,
/// so directions get the correct tstamp based on their position in the stream.
///
/// For multi-staff parts (`num_staves > 1`), the direction's within-part
/// `@staff` is resolved to the global MEI staff number before conversion.
fn convert_measure_directions(
    musicxml_measure: &crate::model::elements::Measure,
    mei_measure: &mut tusk_model::elements::Measure,
    num_staves: u32,
    ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use crate::model::elements::MeasureContent;

    // Restore per-part divisions from cache. Phase 1 processes all parts
    // sequentially, so ctx.divisions() may be stale from a different part.
    // Divisions persist across measures, so we use the cached value.
    ctx.restore_part_divisions();

    // Also check this measure for new divisions (mid-piece division changes)
    for content in &musicxml_measure.content {
        if let MeasureContent::Attributes(attrs) = content {
            if let Some(divs) = attrs.divisions {
                ctx.set_divisions(divs);
            }
        }
    }

    // Reset beat position so directions get correct tstamp
    ctx.reset_beat_position();

    // Save the part ID for multi-staff staff resolution
    let part_id = ctx.position().part_id.clone();

    for content in &musicxml_measure.content {
        match content {
            MeasureContent::Direction(direction) => {
                // For multi-staff parts, resolve within-part staff to global MEI staff
                if num_staves > 1 {
                    if let Some(ref pid) = part_id {
                        let dir_local_staff = direction.staff.unwrap_or(1);
                        if let Some(global) = ctx.global_staff_for_part(pid, dir_local_staff) {
                            ctx.set_staff(global);
                        }
                    }
                }
                let results = convert_direction(direction, ctx)?;

                for result in results {
                    match result {
                        DirectionConversionResult::Dynam(dynam) => {
                            mei_measure
                                .children
                                .push(MeasureChild::Dynam(Box::new(dynam)));
                        }
                        DirectionConversionResult::Hairpin(hairpin) => {
                            mei_measure
                                .children
                                .push(MeasureChild::Hairpin(Box::new(hairpin)));
                        }
                        DirectionConversionResult::Tempo(tempo) => {
                            mei_measure
                                .children
                                .push(MeasureChild::Tempo(Box::new(tempo)));
                        }
                        DirectionConversionResult::Dir(dir) => {
                            mei_measure.children.push(MeasureChild::Dir(Box::new(dir)));
                        }
                    }
                }
            }
            MeasureContent::Harmony(harmony) => {
                // For multi-staff parts, resolve within-part staff to global MEI staff
                if num_staves > 1 {
                    if let Some(ref pid) = part_id {
                        let harm_local_staff = harmony.staff.unwrap_or(1);
                        if let Some(global) = ctx.global_staff_for_part(pid, harm_local_staff) {
                            ctx.set_staff(global);
                        }
                    }
                }
                let harm = super::harmony::convert_harmony(harmony, ctx);
                mei_measure
                    .children
                    .push(MeasureChild::Harm(Box::new(harm)));
            }
            MeasureContent::FiguredBass(fb) => {
                // For multi-staff parts, resolve within-part staff to global MEI staff
                if num_staves > 1 {
                    if let Some(ref pid) = part_id {
                        let fb_local_staff = fb.staff.unwrap_or(1);
                        if let Some(global) = ctx.global_staff_for_part(pid, fb_local_staff) {
                            ctx.set_staff(global);
                        }
                    }
                }
                let mei_fb = super::figured_bass::convert_figured_bass(fb, ctx);
                mei_measure
                    .children
                    .push(MeasureChild::Fb(Box::new(mei_fb)));
            }
            MeasureContent::Note(note) => {
                // Advance beat position for non-chord, non-grace notes
                if !note.is_chord()
                    && !note.is_grace()
                    && let Some(duration) = note.duration
                {
                    ctx.advance_beat_position(duration);
                }
            }
            MeasureContent::Backup(backup) => {
                ctx.advance_beat_position(-backup.duration);
            }
            MeasureContent::Forward(forward) => {
                ctx.advance_beat_position(forward.duration);
            }
            MeasureContent::Print(print) => {
                // Print is measure-level, not per-part — only import from first staff
                if ctx.current_staff() == 1 {
                    for child in super::print::convert_print(print, ctx) {
                        mei_measure.children.push(child);
                    }
                }
            }
            MeasureContent::Sound(sound) => {
                // Sound is measure-level — only import from first staff
                if ctx.current_staff() == 1 {
                    mei_measure
                        .children
                        .push(super::sound::convert_sound(sound, ctx));
                }
            }
            MeasureContent::Attributes(attrs) => {
                // Import measure-style elements from attributes (first staff only)
                if ctx.current_staff() == 1 && !attrs.measure_styles.is_empty() {
                    for child in
                        super::measure_style::convert_measure_styles(&attrs.measure_styles, ctx)
                    {
                        mei_measure.children.push(child);
                    }
                }
            }
            MeasureContent::Listening(listening) => {
                if ctx.current_staff() == 1 {
                    mei_measure
                        .children
                        .push(super::listening::convert_listening(listening, ctx));
                }
            }
            MeasureContent::Grouping(grouping) => {
                if ctx.current_staff() == 1 {
                    mei_measure
                        .children
                        .push(super::listening::convert_grouping(grouping, ctx));
                }
            }
            MeasureContent::Link(link) => {
                if ctx.current_staff() == 1 {
                    mei_measure
                        .children
                        .push(super::listening::convert_link(link, ctx));
                }
            }
            MeasureContent::Bookmark(bookmark) => {
                if ctx.current_staff() == 1 {
                    mei_measure
                        .children
                        .push(super::listening::convert_bookmark(bookmark, ctx));
                }
            }
            // Barlines are handled separately in convert_measure_barlines().
            MeasureContent::Barline(_) => {}
        }
    }

    Ok(())
}

/// Emit completed slurs as MEI `<slur>` control events.
///
/// Drains all completed slurs from the context and adds them to the measure.
fn emit_slurs(mei_measure: &mut tusk_model::elements::Measure, ctx: &mut ConversionContext) {
    for completed in ctx.drain_completed_slurs() {
        let mut slur = Slur::default();

        // Generate ID for the slur
        let slur_id = ctx.generate_id_with_suffix("slur");
        slur.common.xml_id = Some(slur_id);

        // Set startid and endid (with # prefix for URI references)
        slur.slur_log.startid = Some(tusk_model::data::DataUri::from(format!(
            "#{}",
            completed.start_id
        )));
        slur.slur_log.endid = Some(tusk_model::data::DataUri::from(format!(
            "#{}",
            completed.end_id
        )));

        slur.slur_log.staff = Some((completed.mei_staff as u64).to_string());

        mei_measure
            .children
            .push(MeasureChild::Slur(Box::new(slur)));
    }
}

/// Emit completed tuplets as MEI `<tupletSpan>` control events.
///
/// Drains all completed tuplets from the context and adds them to the measure.
fn emit_tuplet_spans(mei_measure: &mut tusk_model::elements::Measure, ctx: &mut ConversionContext) {
    use crate::model::data::AboveBelow;
    use crate::model::notations::ShowTuplet;
    use tusk_model::data::DataBoolean;

    for completed in ctx.drain_completed_tuplets() {
        let mut ts = TupletSpan::default();

        let ts_id = ctx.generate_id_with_suffix("tupletspan");
        ts.common.xml_id = Some(ts_id);

        ts.tuplet_span_log.startid = Some(tusk_model::data::DataUri::from(format!(
            "#{}",
            completed.start_id
        )));
        ts.tuplet_span_log.endid = Some(tusk_model::data::DataUri::from(format!(
            "#{}",
            completed.end_id
        )));

        ts.tuplet_span_log.staff = Some((completed.mei_staff as u64).to_string());
        ts.tuplet_span_log.num = Some(completed.num.to_string());
        ts.tuplet_span_log.numbase = Some(completed.numbase.to_string());

        // Visual attributes
        if let Some(bracket) = completed.bracket {
            ts.tuplet_span_vis.bracket_visible = Some(if bracket {
                DataBoolean::True
            } else {
                DataBoolean::False
            });
        }

        if let Some(show_number) = completed.show_number {
            match show_number {
                ShowTuplet::Both => {
                    ts.tuplet_span_vis.num_visible = Some(DataBoolean::True);
                    ts.tuplet_span_vis.num_format = Some("ratio".to_string());
                }
                ShowTuplet::None => {
                    ts.tuplet_span_vis.num_visible = Some(DataBoolean::False);
                }
                ShowTuplet::Actual => {
                    ts.tuplet_span_vis.num_visible = Some(DataBoolean::True);
                }
            }
        }

        if let Some(placement) = completed.placement {
            ts.tuplet_span_vis.num_place = Some(match placement {
                AboveBelow::Above => tusk_model::data::DataStaffrelBasic::Above,
                AboveBelow::Below => tusk_model::data::DataStaffrelBasic::Below,
            });
            ts.tuplet_span_vis.bracket_place = Some(match placement {
                AboveBelow::Above => tusk_model::data::DataStaffrelBasic::Above,
                AboveBelow::Below => tusk_model::data::DataStaffrelBasic::Below,
            });
        }

        mei_measure
            .children
            .push(MeasureChild::TupletSpan(Box::new(ts)));
    }
}

/// Emit ornament control events (trill, mordent, turn, ornam, fermata, arpeg).
///
/// Drains all pending ornament events from the context and adds them to the measure.
fn emit_ornament_events(
    mei_measure: &mut tusk_model::elements::Measure,
    ctx: &mut ConversionContext,
) {
    for event in ctx.drain_ornament_events() {
        mei_measure.children.push(event);
    }
}

/// Emit completed glissando/slide events as MEI `<gliss>` control events.
fn emit_gliss_events(mei_measure: &mut tusk_model::elements::Measure, ctx: &mut ConversionContext) {
    use tusk_model::data::{DataLineform, DataUri};
    use tusk_model::elements::{Gliss, GlissChild, MeasureChild};

    for completed in ctx.drain_completed_glisses() {
        let mut gliss = Gliss::default();
        gliss.common.xml_id = Some(ctx.generate_id_with_suffix("gliss"));
        gliss.gliss_log.startid = Some(DataUri::from(format!("#{}", completed.start_id)));
        gliss.gliss_log.endid = Some(DataUri::from(format!("#{}", completed.end_id)));
        gliss.gliss_log.staff = Some((completed.mei_staff as u64).to_string());

        // Map line-type → MEI @lform
        gliss.gliss_vis.lform = completed.line_type.as_deref().and_then(|lt| match lt {
            "solid" => Some(DataLineform::Solid),
            "dashed" => Some(DataLineform::Dashed),
            "dotted" => Some(DataLineform::Dotted),
            "wavy" => Some(DataLineform::Wavy),
            _ => None,
        });

        // Text content
        if !completed.text.is_empty() {
            gliss.children.push(GlissChild::Text(completed.text));
        }

        // Label for slide roundtrip
        if let Some(label) = completed.label {
            gliss.common.label = Some(label);
        }

        mei_measure
            .children
            .push(MeasureChild::Gliss(Box::new(gliss)));
    }
}

/// Patch completed hairpins with @tstamp2 on their MEI hairpin elements.
///
/// After all measures are converted, completed hairpins (from wedge stop events)
/// hold the hairpin_id and tstamp2 value. This function walks all measures in the
/// section and sets @tstamp2 on the matching hairpin element.
fn patch_hairpin_tstamp2(section: &mut Section, ctx: &mut ConversionContext) {
    use tusk_model::generated::data::DataMeasurebeat;

    let completed = ctx.drain_completed_hairpins();
    if completed.is_empty() {
        return;
    }

    // Build a lookup from hairpin_id → tstamp2
    let mut lookup: std::collections::HashMap<String, (String, Option<f64>)> = completed
        .into_iter()
        .map(|c| (c.hairpin_id, (c.tstamp2, c.stop_spread)))
        .collect();

    // Walk all measures, find hairpin children, patch tstamp2
    for section_child in &mut section.children {
        if let SectionChild::Measure(measure) = section_child {
            for measure_child in &mut measure.children {
                if let MeasureChild::Hairpin(hairpin) = measure_child {
                    if let Some(ref id) = hairpin.common.xml_id {
                        if let Some((tstamp2, stop_spread)) = lookup.remove(id) {
                            hairpin.hairpin_log.tstamp2 = Some(DataMeasurebeat::from(tstamp2));
                            // Store stop spread in extension store for roundtrip
                            if let Some(spread) = stop_spread {
                                ctx.ext_store_mut().entry(id.clone()).wedge_stop_spread =
                                    Some(spread);
                            }
                        }
                    }
                }
            }
        }
        if lookup.is_empty() {
            break;
        }
    }
}

/// Convert MusicXML measure barlines to MEI measure @left and @right.
///
/// Iterates measure content for Barline elements and sets mei_measure.measure_log.left
/// and measure_log.right from location (left/right). Middle barlines are not represented
/// in MEI and are skipped.
///
/// Barlines with extra children (repeat, ending, fermata, segno, coda, wavy-line) or
/// extra attributes (segno, coda, divisions) are also stored as JSON-in-label on
/// `<dir>` control events for lossless roundtrip.
fn convert_measure_barlines(
    musicxml_measure: &crate::model::elements::Measure,
    mei_measure: &mut tusk_model::elements::Measure,
    ctx: &mut ConversionContext,
) {
    use crate::model::elements::{BarlineLocation, MeasureContent};
    use tusk_model::data::DataBarrendition;

    for content in &musicxml_measure.content {
        if let MeasureContent::Barline(barline) = content {
            // Basic bar-style → MEI @left/@right
            let rend = barline
                .bar_style
                .map(bar_style_to_mei_barrendition)
                .unwrap_or(DataBarrendition::Single);
            let loc = barline.location.unwrap_or(BarlineLocation::Right);
            match loc {
                BarlineLocation::Left => mei_measure.measure_log.left = Some(rend),
                BarlineLocation::Right => mei_measure.measure_log.right = Some(rend),
                BarlineLocation::Middle => {
                    // Middle barlines have no direct MEI @left/@right mapping;
                    // the barline extras roundtrip (JSON-in-label) preserves them.
                }
            }

            // Extra children/attrs → JSON-in-label <dir>
            if barline.has_extra_children() || barline.has_extra_attrs() {
                let dir_child = super::barline::convert_barline(barline, ctx);
                mei_measure.children.push(dir_child);
            }
        }
    }
}

/// Map MusicXML bar-style to MEI DataBarrendition.
fn bar_style_to_mei_barrendition(
    style: crate::model::elements::BarStyle,
) -> tusk_model::data::DataBarrendition {
    use crate::model::elements::BarStyle;
    use tusk_model::data::DataBarrendition;
    match style {
        BarStyle::Regular => DataBarrendition::Single,
        BarStyle::Dotted => DataBarrendition::Dotted,
        BarStyle::Dashed => DataBarrendition::Dashed,
        BarStyle::Heavy => DataBarrendition::Heavy,
        BarStyle::LightLight => DataBarrendition::Dbl,
        BarStyle::LightHeavy | BarStyle::HeavyLight => DataBarrendition::Single,
        BarStyle::HeavyHeavy => DataBarrendition::Dblheavy,
        BarStyle::Tick | BarStyle::Short => DataBarrendition::Single,
        BarStyle::None => DataBarrendition::Invis,
    }
}

/// Convert MusicXML measure attributes to MEI measure attributes.
///
/// Maps:
/// - `number` → `@n` (measure number/label)
/// - `implicit="yes"` → `@metcon="false"` (metrically incomplete)
/// - `width` → `@width` (measure width)
/// - `id` → `xml:id` (element ID)
/// - `non_controlling="yes"` → `@control="false"` (non-controlling barline)
fn convert_measure_attributes(
    musicxml_measure: &crate::model::elements::Measure,
    mei_measure: &mut tusk_model::elements::Measure,
    ctx: &mut ConversionContext,
) {
    use crate::model::data::YesNo;

    // Measure number → @n
    mei_measure.common.n = Some(tusk_model::data::DataWord::from(
        musicxml_measure.number.clone(),
    ));

    // text → @label (displayed measure number when different from @number)
    if let Some(ref text) = musicxml_measure.text {
        mei_measure.common.label = Some(text.clone());
    }

    // implicit="yes" → metcon="false" (metrically non-conformant / pickup measure)
    // In MusicXML, implicit="yes" means the measure doesn't count in measure numbering
    // In MEI, metcon="false" means the measure content doesn't conform to the prevailing meter
    if let Some(YesNo::Yes) = musicxml_measure.implicit {
        mei_measure.measure_log.metcon = Some(DataBoolean::False);
    }

    if let Some(width) = musicxml_measure.width {
        mei_measure.measure_vis.width = Some(DataMeasurementunsigned::from(format!("{}vu", width)));
    }

    // id → xml:id (with mapping)
    if let Some(ref id) = musicxml_measure.id {
        let mei_id = ctx.generate_id_with_suffix("measure");
        ctx.map_id(id, mei_id.clone());
        mei_measure.common.xml_id = Some(mei_id);
    }

    // non_controlling="yes" → control="false" (barline is not controlling)
    // In MusicXML, non-controlling measures in multi-rest regions have non_controlling="yes"
    // In MEI, control="false" means the right bar line doesn't indicate alignment across parts
    if let Some(YesNo::Yes) = musicxml_measure.non_controlling {
        mei_measure.measure_log.control = Some(DataBoolean::False);
    }
}

/// Convert MusicXML measure content to MEI staff.
pub fn convert_staff(
    _measure: &crate::model::elements::Measure,
    staff_number: u32,
    ctx: &mut ConversionContext,
) -> ConversionResult<tusk_model::elements::Staff> {
    use tusk_model::elements::Staff;

    let mut staff = Staff::default();
    // Set staff number using n_integer.n (u64)
    staff.n_integer.n = Some((staff_number as u64).to_string());

    // Create a layer for the content
    // Note: Full measure content conversion will be implemented in subsequent tasks
    let layer = convert_layer(_measure, 1, ctx)?;
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    Ok(staff)
}

/// Convert MusicXML measure content to MEI layer.
pub fn convert_layer(
    measure: &crate::model::elements::Measure,
    layer_number: u32,
    ctx: &mut ConversionContext,
) -> ConversionResult<tusk_model::elements::Layer> {
    use crate::import::process_attributes;
    use crate::model::elements::MeasureContent;
    use tusk_model::elements::Layer;

    let mut layer = Layer::default();
    // Set layer number using n_integer.n (u64)
    layer.n_integer.n = Some((layer_number as u64).to_string());

    ctx.set_layer(layer_number);
    ctx.reset_beat_position();

    // Collect all notes from the measure content for chord detection
    let notes: Vec<&crate::model::note::Note> = measure
        .content
        .iter()
        .filter_map(|c| match c {
            MeasureContent::Note(note) => Some(note.as_ref()),
            _ => None,
        })
        .collect();

    // Collect inline attribute changes separately so beam restructuring
    // (which uses index-based ranges) isn't disrupted by non-note elements.
    let mut inline_attr_changes: Vec<LayerChild> = Vec::new();

    // Track which notes we've processed (for chord grouping)
    let mut processed_note_indices: std::collections::HashSet<usize> =
        std::collections::HashSet::new();

    // Process measure content
    let mut note_index = 0;
    for content in &measure.content {
        match content {
            MeasureContent::Note(note) => {
                // Find the index of this note in our notes vec
                let current_note_index = notes
                    .iter()
                    .position(|n| std::ptr::eq(*n, note.as_ref()))
                    .unwrap_or(note_index);
                note_index += 1;

                // Skip if already processed as part of a chord
                if processed_note_indices.contains(&current_note_index) {
                    continue;
                }

                // Skip chord notes (they are processed with their root note)
                if note.is_chord() {
                    continue;
                }

                // Handle rests
                if note.is_rest() {
                    if is_measure_rest(note) {
                        // Measure rest → MEI mRest
                        let mei_mrest = convert_measure_rest(note, ctx)?;
                        layer.children.push(LayerChild::MRest(Box::new(mei_mrest)));
                    } else {
                        // Regular rest → MEI rest
                        let mei_rest = convert_rest(note, ctx)?;
                        layer.children.push(LayerChild::Rest(Box::new(mei_rest)));
                    }

                    // Advance beat position for rests
                    if let Some(duration) = note.duration {
                        ctx.advance_beat_position(duration);
                    }
                    processed_note_indices.insert(current_note_index);
                    continue;
                }

                // Check if this note is followed by chord notes
                let mut chord_notes: Vec<crate::model::note::Note> = vec![note.as_ref().clone()];
                processed_note_indices.insert(current_note_index);

                // Look ahead for chord notes
                for (i, following_note) in notes.iter().enumerate().skip(current_note_index + 1) {
                    if following_note.is_chord() && !following_note.is_rest() {
                        chord_notes.push((*following_note).clone());
                        processed_note_indices.insert(i);
                    } else {
                        // First non-chord note ends the chord group
                        break;
                    }
                }

                if chord_notes.len() > 1 {
                    let mei_chord = convert_chord(&chord_notes, ctx)?;
                    // Register tremolo wrapping if pending
                    if let Some(pt) = ctx.take_pending_tremolo() {
                        if let Some(ref id) = mei_chord.common.xml_id {
                            ctx.register_tremolo_for_id(id.clone(), pt);
                        }
                    }
                    layer.children.push(LayerChild::Chord(Box::new(mei_chord)));
                } else {
                    let mei_note = convert_note(note, ctx)?;
                    // Register tremolo wrapping if pending
                    if let Some(pt) = ctx.take_pending_tremolo() {
                        if let Some(ref id) = mei_note.common.xml_id {
                            ctx.register_tremolo_for_id(id.clone(), pt);
                        }
                    }
                    layer.children.push(LayerChild::Note(Box::new(mei_note)));
                }

                // Advance beat position if not a grace note
                if !note.is_grace()
                    && let Some(duration) = note.duration
                {
                    ctx.advance_beat_position(duration);
                }
            }
            MeasureContent::Attributes(attrs) => {
                // Process attributes for context state (divisions, key, etc.)
                process_attributes(attrs, ctx, None);

                // Emit inline MEI elements for mid-score attribute changes.
                // The first measure's first attributes block is already in the staffDef,
                // so only emit for changes detected against tracked state.
                // Collected separately to avoid disrupting beam restructuring indices.
                emit_inline_attribute_changes(attrs, &mut inline_attr_changes, ctx);
            }
            MeasureContent::Backup(backup) => {
                // Move beat position backward
                ctx.advance_beat_position(-backup.duration);
            }
            MeasureContent::Forward(forward) => {
                // Move beat position forward
                ctx.advance_beat_position(forward.duration);
            }
            // Non-note content types are handled in convert_measure_content_phase1
            // (directions, harmony, figured bass, print, sound, listening, grouping,
            // link, bookmark) or convert_measure_barlines (barlines). They don't
            // appear in layers — only notes, attributes, backup, and forward do.
            MeasureContent::Direction(_)
            | MeasureContent::Harmony(_)
            | MeasureContent::FiguredBass(_)
            | MeasureContent::Print(_)
            | MeasureContent::Sound(_)
            | MeasureContent::Listening(_)
            | MeasureContent::Barline(_)
            | MeasureContent::Grouping(_)
            | MeasureContent::Link(_)
            | MeasureContent::Bookmark(_) => {}
        }
    }

    // Restructure layer children to wrap beamed notes/chords in <beam> elements
    layer.children = restructure_with_beams(layer.children, &notes);

    // Post-process: wrap notes/chords in bTrem/fTrem containers based on
    // registered tremolo info. Must run AFTER beam restructuring since both
    // operate on layer children indices.
    let tremolo_map = ctx.drain_tremolo_map();
    if !tremolo_map.is_empty() {
        layer.children = wrap_tremolo_containers(layer.children, &tremolo_map);
    }

    // Prepend inline attribute changes (collected separately to avoid
    // disrupting beam restructuring which uses index-based ranges).
    if !inline_attr_changes.is_empty() {
        inline_attr_changes.append(&mut layer.children);
        layer.children = inline_attr_changes;
    }

    Ok(layer)
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::*;
    use crate::context::ConversionDirection;
    use crate::import::test_utils::make_score_part;
    use crate::model::elements::{Part, PartList, PartListItem};
    use tusk_model::elements::MdivChild;

    // ============================================================================
    // Score Structure Tests
    // ============================================================================

    #[test]
    fn convert_score_creates_body_with_mdiv() {
        let score = ScorePartwise::default();
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let body = convert_body(&score, &mut ctx).expect("conversion should succeed");

        assert_eq!(body.children.len(), 1);
        assert!(matches!(&body.children[0], BodyChild::Mdiv(_)));
    }

    #[test]
    fn convert_mdiv_contains_score() {
        let score = ScorePartwise::default();
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mdiv = convert_mdiv(&score, &mut ctx).expect("conversion should succeed");

        assert_eq!(mdiv.children.len(), 1);
        assert!(matches!(&mdiv.children[0], MdivChild::Score(_)));
    }

    #[test]
    fn convert_score_content_has_score_def_and_section() {
        let mut score = ScorePartwise::default();
        score.part_list = PartList {
            items: vec![PartListItem::ScorePart(Box::new(make_score_part(
                "P1", "Piano",
            )))],
        };

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_score = convert_score_content(&score, &mut ctx).expect("conversion should succeed");

        // Should have scoreDef followed by section
        assert!(mei_score.children.len() >= 2);
        assert!(matches!(&mei_score.children[0], ScoreChild::ScoreDef(_)));
        assert!(matches!(&mei_score.children[1], ScoreChild::Section(_)));
    }

    // ============================================================================
    // Measure Conversion Tests
    // ============================================================================

    #[test]
    fn convert_section_creates_measures() {
        use crate::model::elements::Measure;

        let mut score = ScorePartwise::default();
        score.part_list = PartList {
            items: vec![PartListItem::ScorePart(Box::new(make_score_part(
                "P1", "Piano",
            )))],
        };
        score.parts = vec![Part {
            id: "P1".to_string(),
            measures: vec![Measure::new("1"), Measure::new("2")],
        }];

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let section = convert_section(&score, &mut ctx).expect("conversion should succeed");

        // Should have 2 measures
        let measure_count = section
            .children
            .iter()
            .filter(|c| matches!(c, SectionChild::Measure(_)))
            .count();
        assert_eq!(measure_count, 2);
    }

    #[test]
    fn convert_measure_sets_measure_number() {
        use crate::model::elements::Measure;

        let mut score = ScorePartwise::default();
        score.part_list = PartList {
            items: vec![PartListItem::ScorePart(Box::new(make_score_part(
                "P1", "Piano",
            )))],
        };
        score.parts = vec![Part {
            id: "P1".to_string(),
            measures: vec![Measure::new("42")],
        }];

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_measure = convert_measure(&score, 0, &mut ctx).expect("conversion should succeed");

        // Check measure number is set via common.n
        assert!(mei_measure.common.n.is_some());
        let n = mei_measure.common.n.as_ref().unwrap();
        assert_eq!(n.0.as_str(), "42");
    }

    // ============================================================================
    // Measure Attribute Conversion Tests
    // ============================================================================

    #[test]
    fn convert_measure_implicit_yes_sets_metcon_false() {
        use crate::model::data::YesNo;
        use crate::model::elements::Measure;

        let mut score = ScorePartwise::default();
        score.part_list = PartList {
            items: vec![PartListItem::ScorePart(Box::new(make_score_part(
                "P1", "Piano",
            )))],
        };

        // Create a pickup measure (implicit="yes")
        let mut pickup_measure = Measure::new("0");
        pickup_measure.implicit = Some(YesNo::Yes);

        score.parts = vec![Part {
            id: "P1".to_string(),
            measures: vec![pickup_measure],
        }];

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_measure = convert_measure(&score, 0, &mut ctx).expect("conversion should succeed");

        // implicit="yes" → metcon="false"
        assert_eq!(
            mei_measure.measure_log.metcon,
            Some(tusk_model::data::DataBoolean::False)
        );
    }

    #[test]
    fn convert_measure_implicit_no_does_not_set_metcon() {
        use crate::model::data::YesNo;
        use crate::model::elements::Measure;

        let mut score = ScorePartwise::default();
        score.part_list = PartList {
            items: vec![PartListItem::ScorePart(Box::new(make_score_part(
                "P1", "Piano",
            )))],
        };

        // Regular measure (implicit="no" or absent)
        let mut regular_measure = Measure::new("1");
        regular_measure.implicit = Some(YesNo::No);

        score.parts = vec![Part {
            id: "P1".to_string(),
            measures: vec![regular_measure],
        }];

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_measure = convert_measure(&score, 0, &mut ctx).expect("conversion should succeed");

        // implicit="no" → metcon not set (defaults to true)
        assert!(mei_measure.measure_log.metcon.is_none());
    }

    #[test]
    fn convert_measure_width_sets_width_attribute() {
        use crate::model::elements::Measure;

        let mut score = ScorePartwise::default();
        score.part_list = PartList {
            items: vec![PartListItem::ScorePart(Box::new(make_score_part(
                "P1", "Piano",
            )))],
        };

        // Measure with explicit width
        let mut measure_with_width = Measure::new("1");
        measure_with_width.width = Some(150.5);

        score.parts = vec![Part {
            id: "P1".to_string(),
            measures: vec![measure_with_width],
        }];

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_measure = convert_measure(&score, 0, &mut ctx).expect("conversion should succeed");

        // width → @width with virtual units
        assert!(mei_measure.measure_vis.width.is_some());
        let width = mei_measure.measure_vis.width.as_ref().unwrap();
        assert_eq!(width.0.as_str(), "150.5vu");
    }

    #[test]
    fn convert_measure_id_sets_xml_id_and_maps() {
        use crate::model::elements::Measure;

        let mut score = ScorePartwise::default();
        score.part_list = PartList {
            items: vec![PartListItem::ScorePart(Box::new(make_score_part(
                "P1", "Piano",
            )))],
        };

        // Measure with explicit ID
        let mut measure_with_id = Measure::new("1");
        measure_with_id.id = Some("measure1".to_string());

        score.parts = vec![Part {
            id: "P1".to_string(),
            measures: vec![measure_with_id],
        }];

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_measure = convert_measure(&score, 0, &mut ctx).expect("conversion should succeed");

        // id → xml:id (generated)
        assert!(mei_measure.common.xml_id.is_some());

        // ID should be mapped
        let mei_id = ctx.get_mei_id("measure1");
        assert!(mei_id.is_some());
        assert_eq!(mei_measure.common.xml_id.as_deref(), mei_id);
    }

    #[test]
    fn convert_measure_non_controlling_yes_sets_control_false() {
        use crate::model::data::YesNo;
        use crate::model::elements::Measure;

        let mut score = ScorePartwise::default();
        score.part_list = PartList {
            items: vec![PartListItem::ScorePart(Box::new(make_score_part(
                "P1", "Piano",
            )))],
        };

        // Non-controlling measure (in multi-rest region)
        let mut non_controlling_measure = Measure::new("2");
        non_controlling_measure.non_controlling = Some(YesNo::Yes);

        score.parts = vec![Part {
            id: "P1".to_string(),
            measures: vec![non_controlling_measure],
        }];

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_measure = convert_measure(&score, 0, &mut ctx).expect("conversion should succeed");

        // non_controlling="yes" → control="false"
        assert_eq!(
            mei_measure.measure_log.control,
            Some(tusk_model::data::DataBoolean::False)
        );
    }

    #[test]
    fn convert_measure_no_optional_attributes() {
        use crate::model::elements::Measure;

        let mut score = ScorePartwise::default();
        score.part_list = PartList {
            items: vec![PartListItem::ScorePart(Box::new(make_score_part(
                "P1", "Piano",
            )))],
        };

        // Basic measure with only required number
        let basic_measure = Measure::new("1");

        score.parts = vec![Part {
            id: "P1".to_string(),
            measures: vec![basic_measure],
        }];

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_measure = convert_measure(&score, 0, &mut ctx).expect("conversion should succeed");

        // Only @n should be set, optional attributes should be None
        assert!(mei_measure.common.n.is_some());
        assert_eq!(
            mei_measure.common.n.as_ref().map(|w| w.0.as_str()),
            Some("1")
        );
        assert!(mei_measure.measure_log.metcon.is_none());
        assert!(mei_measure.measure_vis.width.is_none());
        assert!(mei_measure.common.xml_id.is_none());
        assert!(mei_measure.measure_log.control.is_none());
    }

    #[test]
    fn convert_measure_all_attributes_combined() {
        use crate::model::data::YesNo;
        use crate::model::elements::Measure;

        let mut score = ScorePartwise::default();
        score.part_list = PartList {
            items: vec![PartListItem::ScorePart(Box::new(make_score_part(
                "P1", "Piano",
            )))],
        };

        // Measure with all optional attributes
        let full_measure = Measure {
            number: "0".to_string(),
            text: Some("Pickup".to_string()),
            implicit: Some(YesNo::Yes),
            non_controlling: Some(YesNo::Yes),
            width: Some(200.0),
            id: Some("m0".to_string()),
            content: vec![],
        };

        score.parts = vec![Part {
            id: "P1".to_string(),
            measures: vec![full_measure],
        }];

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_measure = convert_measure(&score, 0, &mut ctx).expect("conversion should succeed");

        // All attributes should be converted
        assert_eq!(
            mei_measure.common.n.as_ref().map(|w| w.0.as_str()),
            Some("0")
        );
        assert_eq!(
            mei_measure.measure_log.metcon,
            Some(tusk_model::data::DataBoolean::False)
        );
        assert_eq!(
            mei_measure.measure_log.control,
            Some(tusk_model::data::DataBoolean::False)
        );
        assert_eq!(
            mei_measure.measure_vis.width.as_ref().map(|w| w.0.as_str()),
            Some("200vu")
        );
        assert_eq!(mei_measure.common.label.as_deref(), Some("Pickup"));
        assert!(mei_measure.common.xml_id.is_some());
        assert!(ctx.get_mei_id("m0").is_some());
    }

    // ============================================================================
    // Context Tracking Tests
    // ============================================================================

    #[test]
    fn conversion_tracks_current_position() {
        use crate::import::convert_score_with_context;
        use crate::model::elements::Measure;

        let mut score = ScorePartwise::default();
        score.part_list = PartList {
            items: vec![PartListItem::ScorePart(Box::new(make_score_part(
                "P1", "Piano",
            )))],
        };
        score.parts = vec![Part {
            id: "P1".to_string(),
            measures: vec![Measure::new("5")],
        }];

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let (_mei, _ext) =
            convert_score_with_context(&score, &mut ctx).expect("conversion should succeed");

        // After conversion, context should track last processed position
        assert_eq!(ctx.position().part_id.as_deref(), Some("P1"));
        assert_eq!(ctx.position().measure_number.as_deref(), Some("5"));
    }

    // ============================================================================
    // Layer Integration Tests
    // ============================================================================

    #[test]
    fn convert_layer_with_notes_creates_note_children() {
        use crate::model::data::Step;
        use crate::model::elements::{Measure, MeasureContent};
        use crate::model::note::{Note, NoteType, NoteTypeValue, Pitch};

        let mut measure = Measure::new("1");

        // Add a note
        let mut note = Note::pitched(Pitch::new(Step::C, 4), 4.0);
        note.note_type = Some(NoteType::new(NoteTypeValue::Quarter));
        measure.content.push(MeasureContent::Note(Box::new(note)));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0);

        let layer = convert_layer(&measure, 1, &mut ctx).expect("conversion should succeed");

        // Should have one note child
        assert_eq!(layer.children.len(), 1);
        assert!(matches!(layer.children[0], LayerChild::Note(_)));
    }

    #[test]
    fn convert_layer_with_rests_creates_rest_children() {
        use crate::model::elements::{Measure, MeasureContent};
        use crate::model::note::{Note, NoteType, NoteTypeValue, Rest};

        let mut measure = Measure::new("1");

        // Add a rest
        let mut note = Note::rest(Rest::new(), 4.0);
        note.note_type = Some(NoteType::new(NoteTypeValue::Quarter));
        measure.content.push(MeasureContent::Note(Box::new(note)));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0);

        let layer = convert_layer(&measure, 1, &mut ctx).expect("conversion should succeed");

        // Should have one rest child
        assert_eq!(layer.children.len(), 1);
        assert!(matches!(layer.children[0], LayerChild::Rest(_)));
    }

    #[test]
    fn convert_layer_with_measure_rest_creates_mrest_child() {
        use crate::model::data::YesNo;
        use crate::model::elements::{Measure, MeasureContent};
        use crate::model::note::{Note, Rest};

        let mut measure = Measure::new("1");

        // Add a measure rest
        let mut rest = Rest::new();
        rest.measure = Some(YesNo::Yes);
        let note = Note::rest(rest, 16.0);
        measure.content.push(MeasureContent::Note(Box::new(note)));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0);

        let layer = convert_layer(&measure, 1, &mut ctx).expect("conversion should succeed");

        // Should have one mRest child
        assert_eq!(layer.children.len(), 1);
        assert!(matches!(layer.children[0], LayerChild::MRest(_)));
    }

    #[test]
    fn convert_layer_advances_beat_position_for_rest() {
        use crate::model::elements::{Measure, MeasureContent};
        use crate::model::note::{Note, NoteType, NoteTypeValue, Rest};

        let mut measure = Measure::new("1");

        // Add a rest with duration
        let mut note = Note::rest(Rest::new(), 4.0);
        note.note_type = Some(NoteType::new(NoteTypeValue::Quarter));
        measure.content.push(MeasureContent::Note(Box::new(note)));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0);

        let _layer = convert_layer(&measure, 1, &mut ctx).expect("conversion should succeed");

        // Beat position should have advanced by the rest duration
        assert_eq!(ctx.beat_position(), 4.0);
    }

    #[test]
    fn convert_layer_with_chord() {
        use crate::model::data::Step;
        use crate::model::elements::{Empty, Measure, MeasureContent};
        use crate::model::note::{Note, NoteType, NoteTypeValue, Pitch};

        let mut measure = Measure::new("1");

        // Add a chord (C4, E4)
        let mut note1 = Note::pitched(Pitch::new(Step::C, 4), 4.0);
        note1.note_type = Some(NoteType::new(NoteTypeValue::Quarter));
        measure.content.push(MeasureContent::Note(Box::new(note1)));

        let mut note2 = Note::pitched(Pitch::new(Step::E, 4), 4.0);
        note2.chord = Some(Empty);
        note2.note_type = Some(NoteType::new(NoteTypeValue::Quarter));
        measure.content.push(MeasureContent::Note(Box::new(note2)));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0);

        let layer = convert_layer(&measure, 1, &mut ctx).expect("conversion should succeed");

        // Should have one chord child (not two separate notes)
        assert_eq!(layer.children.len(), 1);
        assert!(matches!(layer.children[0], LayerChild::Chord(_)));
    }

    #[test]
    fn convert_layer_with_chord_advances_beat_position() {
        use crate::model::data::Step;
        use crate::model::elements::{Empty, Measure, MeasureContent};
        use crate::model::note::{Note, NoteType, NoteTypeValue, Pitch};

        let mut measure = Measure::new("1");

        // Add a chord (C4, E4) with duration 4
        let mut note1 = Note::pitched(Pitch::new(Step::C, 4), 4.0);
        note1.note_type = Some(NoteType::new(NoteTypeValue::Quarter));
        measure.content.push(MeasureContent::Note(Box::new(note1)));

        let mut note2 = Note::pitched(Pitch::new(Step::E, 4), 4.0);
        note2.chord = Some(Empty);
        note2.note_type = Some(NoteType::new(NoteTypeValue::Quarter));
        measure.content.push(MeasureContent::Note(Box::new(note2)));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0);

        let _layer = convert_layer(&measure, 1, &mut ctx).expect("conversion should succeed");

        // Beat position should have advanced by the chord duration (once, not twice)
        assert_eq!(ctx.beat_position(), 4.0);
    }

    #[test]
    fn convert_layer_mixed_notes_and_chords() {
        use crate::model::data::Step;
        use crate::model::elements::{Empty, Measure, MeasureContent};
        use crate::model::note::{Note, NoteType, NoteTypeValue, Pitch};

        let mut measure = Measure::new("1");

        // Single note
        let mut note1 = Note::pitched(Pitch::new(Step::C, 4), 4.0);
        note1.note_type = Some(NoteType::new(NoteTypeValue::Quarter));
        measure.content.push(MeasureContent::Note(Box::new(note1)));

        // Chord (E4, G4)
        let mut note2 = Note::pitched(Pitch::new(Step::E, 4), 4.0);
        note2.note_type = Some(NoteType::new(NoteTypeValue::Quarter));
        measure.content.push(MeasureContent::Note(Box::new(note2)));

        let mut note3 = Note::pitched(Pitch::new(Step::G, 4), 4.0);
        note3.chord = Some(Empty);
        note3.note_type = Some(NoteType::new(NoteTypeValue::Quarter));
        measure.content.push(MeasureContent::Note(Box::new(note3)));

        // Another single note
        let mut note4 = Note::pitched(Pitch::new(Step::A, 4), 4.0);
        note4.note_type = Some(NoteType::new(NoteTypeValue::Quarter));
        measure.content.push(MeasureContent::Note(Box::new(note4)));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0);

        let layer = convert_layer(&measure, 1, &mut ctx).expect("conversion should succeed");

        // Should have: Note, Chord, Note
        assert_eq!(layer.children.len(), 3);
        assert!(matches!(layer.children[0], LayerChild::Note(_)));
        assert!(matches!(layer.children[1], LayerChild::Chord(_)));
        assert!(matches!(layer.children[2], LayerChild::Note(_)));
    }
}
