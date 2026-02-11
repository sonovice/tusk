//! Serializer implementations for MusicXML score-level types.
//!
//! This module contains `MusicXmlSerialize` implementations for:
//! - ScorePartwise, Part, Measure
//! - Work, Identification, Defaults
//! - PartList, ScorePart, PartGroup
//! - Barline
//! - Helper functions for enum-to-string conversion

use std::io::Write;

use crate::model::*;
use crate::serializer::{
    MusicXmlSerialize, MusicXmlWriter, SerializeResult, push_opt_attr, push_opt_str_attr,
};

// ============================================================================
// ScorePartwise
// ============================================================================

impl MusicXmlSerialize for ScorePartwise {
    fn element_name(&self) -> &'static str {
        "score-partwise"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_str_attr!(attrs, "version", self.version);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        // Work
        if let Some(ref work) = self.work {
            work.serialize(w)?;
        }

        // Movement number and title
        w.write_opt_text_element("movement-number", &self.movement_number)?;
        w.write_opt_text_element("movement-title", &self.movement_title)?;

        // Identification
        if let Some(ref ident) = self.identification {
            ident.serialize(w)?;
        }

        // Defaults
        if let Some(ref defaults) = self.defaults {
            defaults.serialize(w)?;
        }

        // Credits
        for credit in &self.credits {
            credit.serialize(w)?;
        }

        // Part list (required)
        self.part_list.serialize(w)?;

        // Parts
        for part in &self.parts {
            part.serialize(w)?;
        }

        Ok(())
    }
}

// ============================================================================
// Work
// ============================================================================

impl MusicXmlSerialize for Work {
    fn element_name(&self) -> &'static str {
        "work"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        self.work_number.is_some() || self.work_title.is_some() || self.opus.is_some()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_opt_text_element("work-number", &self.work_number)?;
        w.write_opt_text_element("work-title", &self.work_title)?;
        if let Some(ref opus) = self.opus {
            opus.serialize(w)?;
        }
        Ok(())
    }
}

impl MusicXmlSerialize for Opus {
    fn element_name(&self) -> &'static str {
        "opus"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = vec![("xlink:href", self.href.clone())];
        push_opt_str_attr!(attrs, "xlink:type", self.xlink_type);
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// Identification
// ============================================================================

impl MusicXmlSerialize for Identification {
    fn element_name(&self) -> &'static str {
        "identification"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !self.creators.is_empty()
            || !self.rights.is_empty()
            || self.encoding.is_some()
            || self.source.is_some()
            || !self.relations.is_empty()
            || self.miscellaneous.is_some()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        for creator in &self.creators {
            serialize_typed_text(w, "creator", creator)?;
        }
        for right in &self.rights {
            serialize_typed_text(w, "rights", right)?;
        }
        if let Some(ref encoding) = self.encoding {
            encoding.serialize(w)?;
        }
        w.write_opt_text_element("source", &self.source)?;
        for relation in &self.relations {
            serialize_typed_text(w, "relation", relation)?;
        }
        if let Some(ref misc) = self.miscellaneous {
            misc.serialize(w)?;
        }
        Ok(())
    }
}

fn serialize_typed_text<W: Write>(
    w: &mut MusicXmlWriter<W>,
    name: &str,
    typed_text: &TypedText,
) -> SerializeResult<()> {
    let mut start = w.start_element(name);
    if let Some(ref t) = typed_text.text_type {
        start.push_attribute(("type", t.as_str()));
    }
    w.write_start(start)?;
    w.write_text(&typed_text.value)?;
    w.write_end(name)?;
    Ok(())
}

impl MusicXmlSerialize for Encoding {
    fn element_name(&self) -> &'static str {
        "encoding"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_opt_text_element("encoding-date", &self.encoding_date)?;
        for encoder in &self.encoders {
            serialize_typed_text(w, "encoder", encoder)?;
        }
        for software in &self.software {
            w.write_text_element("software", software)?;
        }
        for desc in &self.encoding_descriptions {
            w.write_text_element("encoding-description", desc)?;
        }
        for support in &self.supports {
            support.serialize(w)?;
        }
        Ok(())
    }
}

impl MusicXmlSerialize for Supports {
    fn element_name(&self) -> &'static str {
        "supports"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = vec![
            ("element", self.element.clone()),
            ("type", yes_no_str(&self.support_type).to_string()),
        ];
        push_opt_str_attr!(attrs, "attribute", self.attribute);
        push_opt_str_attr!(attrs, "value", self.value);
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MusicXmlSerialize for Miscellaneous {
    fn element_name(&self) -> &'static str {
        "miscellaneous"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !self.fields.is_empty()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        for field in &self.fields {
            let mut start = w.start_element("miscellaneous-field");
            start.push_attribute(("name", field.name.as_str()));
            w.write_start(start)?;
            w.write_text(&field.value)?;
            w.write_end("miscellaneous-field")?;
        }
        Ok(())
    }
}

// ============================================================================
// Defaults
// ============================================================================

impl MusicXmlSerialize for Defaults {
    fn element_name(&self) -> &'static str {
        "defaults"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        if let Some(ref scaling) = self.scaling {
            scaling.serialize(w)?;
        }
        if self.concert_score.is_some() {
            w.write_empty(w.start_element("concert-score"))?;
        }
        if let Some(ref page_layout) = self.page_layout {
            page_layout.serialize(w)?;
        }
        if let Some(ref system_layout) = self.system_layout {
            system_layout.serialize(w)?;
        }
        for staff_layout in &self.staff_layouts {
            staff_layout.serialize(w)?;
        }
        if let Some(ref appearance) = self.appearance {
            serialize_appearance(w, appearance)?;
        }
        if let Some(ref font) = self.music_font {
            serialize_empty_font(w, "music-font", font)?;
        }
        if let Some(ref font) = self.word_font {
            serialize_empty_font(w, "word-font", font)?;
        }
        for lf in &self.lyric_fonts {
            serialize_lyric_font(w, lf)?;
        }
        for ll in &self.lyric_languages {
            serialize_lyric_language(w, ll)?;
        }
        Ok(())
    }
}

impl MusicXmlSerialize for Scaling {
    fn element_name(&self) -> &'static str {
        "scaling"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text_element("millimeters", &self.millimeters.to_string())?;
        w.write_text_element("tenths", &self.tenths.to_string())?;
        Ok(())
    }
}

impl MusicXmlSerialize for PageLayout {
    fn element_name(&self) -> &'static str {
        "page-layout"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        if let Some(h) = self.page_height {
            w.write_text_element("page-height", &h.to_string())?;
        }
        if let Some(width) = self.page_width {
            w.write_text_element("page-width", &width.to_string())?;
        }
        for margins in &self.page_margins {
            margins.serialize(w)?;
        }
        Ok(())
    }
}

impl MusicXmlSerialize for PageMargins {
    fn element_name(&self) -> &'static str {
        "page-margins"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref mt) = self.margin_type {
            attrs.push(("type", margin_type_str(mt).to_string()));
        }
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text_element("left-margin", &self.left_margin.to_string())?;
        w.write_text_element("right-margin", &self.right_margin.to_string())?;
        w.write_text_element("top-margin", &self.top_margin.to_string())?;
        w.write_text_element("bottom-margin", &self.bottom_margin.to_string())?;
        Ok(())
    }
}

impl MusicXmlSerialize for SystemLayout {
    fn element_name(&self) -> &'static str {
        "system-layout"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        if let Some(ref margins) = self.system_margins {
            margins.serialize(w)?;
        }
        if let Some(d) = self.system_distance {
            w.write_text_element("system-distance", &d.to_string())?;
        }
        if let Some(d) = self.top_system_distance {
            w.write_text_element("top-system-distance", &d.to_string())?;
        }
        if let Some(ref dividers) = self.system_dividers {
            serialize_system_dividers(w, dividers)?;
        }
        Ok(())
    }
}

impl MusicXmlSerialize for SystemMargins {
    fn element_name(&self) -> &'static str {
        "system-margins"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text_element("left-margin", &self.left_margin.to_string())?;
        w.write_text_element("right-margin", &self.right_margin.to_string())?;
        Ok(())
    }
}

impl MusicXmlSerialize for StaffLayout {
    fn element_name(&self) -> &'static str {
        "staff-layout"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "number", self.number);
        attrs
    }

    fn has_children(&self) -> bool {
        self.staff_distance.is_some()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        if let Some(d) = self.staff_distance {
            w.write_text_element("staff-distance", &d.to_string())?;
        }
        Ok(())
    }
}

// ============================================================================
// Appearance, Fonts, System Dividers
// ============================================================================

fn serialize_appearance<W: Write>(
    w: &mut MusicXmlWriter<W>,
    appearance: &Appearance,
) -> SerializeResult<()> {
    let start = w.start_element("appearance");
    w.write_start(start)?;
    for lw in &appearance.line_widths {
        let mut s = w.start_element("line-width");
        s.push_attribute(("type", lw.line_width_type.as_str()));
        w.write_start(s)?;
        w.write_text(&lw.value.to_string())?;
        w.write_end("line-width")?;
    }
    for ns in &appearance.note_sizes {
        let mut s = w.start_element("note-size");
        s.push_attribute(("type", note_size_type_str(&ns.note_size_type)));
        w.write_start(s)?;
        w.write_text(&ns.value.to_string())?;
        w.write_end("note-size")?;
    }
    for d in &appearance.distances {
        let mut s = w.start_element("distance");
        s.push_attribute(("type", d.distance_type.as_str()));
        w.write_start(s)?;
        w.write_text(&d.value.to_string())?;
        w.write_end("distance")?;
    }
    for g in &appearance.glyphs {
        let mut s = w.start_element("glyph");
        s.push_attribute(("type", g.glyph_type.as_str()));
        w.write_start(s)?;
        w.write_text(&g.value)?;
        w.write_end("glyph")?;
    }
    for oa in &appearance.other_appearances {
        let mut s = w.start_element("other-appearance");
        s.push_attribute(("type", oa.appearance_type.as_str()));
        w.write_start(s)?;
        w.write_text(&oa.value)?;
        w.write_end("other-appearance")?;
    }
    w.write_end("appearance")?;
    Ok(())
}

fn note_size_type_str(nst: &NoteSizeType) -> &'static str {
    match nst {
        NoteSizeType::Cue => "cue",
        NoteSizeType::Grace => "grace",
        NoteSizeType::GraceCue => "grace-cue",
        NoteSizeType::Large => "large",
    }
}

fn serialize_empty_font<W: Write>(
    w: &mut MusicXmlWriter<W>,
    name: &str,
    font: &EmptyFont,
) -> SerializeResult<()> {
    let mut start = w.start_element(name);
    push_opt_str_attr_start(&mut start, "font-family", &font.font_family);
    if let Some(ref style) = font.font_style {
        start.push_attribute(("font-style", font_style_str(style)));
    }
    if let Some(ref size) = font.font_size {
        start.push_attribute(("font-size", font_size_str(size).as_str()));
    }
    if let Some(ref weight) = font.font_weight {
        start.push_attribute(("font-weight", font_weight_str(weight)));
    }
    w.write_empty(start)?;
    Ok(())
}

fn serialize_lyric_font<W: Write>(
    w: &mut MusicXmlWriter<W>,
    lf: &LyricFont,
) -> SerializeResult<()> {
    let mut start = w.start_element("lyric-font");
    push_opt_str_attr_start(&mut start, "number", &lf.number);
    push_opt_str_attr_start(&mut start, "name", &lf.name);
    push_opt_str_attr_start(&mut start, "font-family", &lf.font_family);
    if let Some(ref style) = lf.font_style {
        start.push_attribute(("font-style", font_style_str(style)));
    }
    if let Some(ref size) = lf.font_size {
        start.push_attribute(("font-size", font_size_str(size).as_str()));
    }
    if let Some(ref weight) = lf.font_weight {
        start.push_attribute(("font-weight", font_weight_str(weight)));
    }
    w.write_empty(start)?;
    Ok(())
}

fn serialize_lyric_language<W: Write>(
    w: &mut MusicXmlWriter<W>,
    ll: &LyricLanguage,
) -> SerializeResult<()> {
    let mut start = w.start_element("lyric-language");
    push_opt_str_attr_start(&mut start, "number", &ll.number);
    push_opt_str_attr_start(&mut start, "name", &ll.name);
    start.push_attribute(("xml:lang", ll.lang.as_str()));
    w.write_empty(start)?;
    Ok(())
}

fn serialize_system_dividers<W: Write>(
    w: &mut MusicXmlWriter<W>,
    dividers: &SystemDividers,
) -> SerializeResult<()> {
    let start = w.start_element("system-dividers");
    w.write_start(start)?;
    if let Some(ref ld) = dividers.left_divider {
        let mut s = w.start_element("left-divider");
        if let Some(ref po) = ld.print_object {
            s.push_attribute(("print-object", yes_no_str(po)));
        }
        w.write_empty(s)?;
    }
    if let Some(ref rd) = dividers.right_divider {
        let mut s = w.start_element("right-divider");
        if let Some(ref po) = rd.print_object {
            s.push_attribute(("print-object", yes_no_str(po)));
        }
        w.write_empty(s)?;
    }
    w.write_end("system-dividers")?;
    Ok(())
}

// ============================================================================
// Credit
// ============================================================================

impl MusicXmlSerialize for Credit {
    fn element_name(&self) -> &'static str {
        "credit"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "page", self.page);
        attrs
    }

    fn has_children(&self) -> bool {
        !self.credit_types.is_empty() || self.content.is_some()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        for ct in &self.credit_types {
            w.write_text_element("credit-type", ct)?;
        }
        if let Some(ref content) = self.content {
            match content {
                CreditContent::Words(words) => {
                    for word in &words.words {
                        serialize_formatted_text_id(w, "credit-words", word)?;
                    }
                }
                CreditContent::Image(img) => {
                    img.credit_image.serialize(w)?;
                }
            }
        }
        Ok(())
    }
}

fn serialize_formatted_text_id<W: Write>(
    w: &mut MusicXmlWriter<W>,
    name: &str,
    text: &FormattedTextId,
) -> SerializeResult<()> {
    let mut start = w.start_element(name);
    push_opt_str_attr_start(&mut start, "id", &text.id);
    push_opt_attr_start(&mut start, "default-x", &text.default_x);
    push_opt_attr_start(&mut start, "default-y", &text.default_y);
    push_opt_str_attr_start(&mut start, "font-family", &text.font_family);
    if let Some(ref size) = text.font_size {
        start.push_attribute(("font-size", font_size_str(size).as_str()));
    }
    if let Some(ref style) = text.font_style {
        start.push_attribute(("font-style", font_style_str(style)));
    }
    if let Some(ref weight) = text.font_weight {
        start.push_attribute(("font-weight", font_weight_str(weight)));
    }
    if let Some(ref j) = text.justify {
        start.push_attribute(("justify", left_center_right_str(j)));
    }
    if let Some(ref h) = text.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = text.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    w.write_start(start)?;
    w.write_text(&text.value)?;
    w.write_end(name)?;
    Ok(())
}

impl MusicXmlSerialize for Image {
    fn element_name(&self) -> &'static str {
        "credit-image"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = vec![
            ("source", self.source.clone()),
            ("type", self.image_type.clone()),
        ];
        push_opt_attr!(attrs, "height", self.height);
        push_opt_attr!(attrs, "width", self.width);
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// PartList
// ============================================================================

impl MusicXmlSerialize for PartList {
    fn element_name(&self) -> &'static str {
        "part-list"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !self.items.is_empty()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        for item in &self.items {
            match item {
                PartListItem::PartGroup(pg) => pg.serialize(w)?,
                PartListItem::ScorePart(sp) => sp.serialize(w)?,
            }
        }
        Ok(())
    }
}

impl MusicXmlSerialize for PartGroup {
    fn element_name(&self) -> &'static str {
        "part-group"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = vec![("type", start_stop_str(&self.group_type).to_string())];
        push_opt_str_attr!(attrs, "number", self.number);
        attrs
    }

    fn has_children(&self) -> bool {
        self.group_name.is_some()
            || self.group_name_display.is_some()
            || self.group_abbreviation.is_some()
            || self.group_abbreviation_display.is_some()
            || self.group_symbol.is_some()
            || self.group_barline.is_some()
            || self.group_time.is_some()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_opt_text_element("group-name", &self.group_name)?;
        if let Some(ref gnd) = self.group_name_display {
            super::print::serialize_name_display(w, "group-name-display", gnd)?;
        }
        w.write_opt_text_element("group-abbreviation", &self.group_abbreviation)?;
        if let Some(ref gad) = self.group_abbreviation_display {
            super::print::serialize_name_display(w, "group-abbreviation-display", gad)?;
        }
        if let Some(ref gs) = self.group_symbol {
            let mut start = w.start_element("group-symbol");
            push_opt_attr_start(&mut start, "default-x", &gs.default_x);
            push_opt_attr_start(&mut start, "relative-x", &gs.relative_x);
            push_opt_str_attr_start(&mut start, "color", &gs.color);
            w.write_start(start)?;
            w.write_text(group_symbol_str(&gs.value))?;
            w.write_end("group-symbol")?;
        }
        if let Some(ref gb) = self.group_barline {
            let mut start = w.start_element("group-barline");
            push_opt_str_attr_start(&mut start, "color", &gb.color);
            w.write_start(start)?;
            w.write_text(group_barline_str(&gb.value))?;
            w.write_end("group-barline")?;
        }
        if self.group_time.is_some() {
            let start = w.start_element("group-time");
            w.write_empty(start)?;
        }
        Ok(())
    }
}

impl MusicXmlSerialize for ScorePart {
    fn element_name(&self) -> &'static str {
        "score-part"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        vec![("id", self.id.clone())]
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        // part-link
        for pl in &self.part_links {
            serialize_part_link(w, pl)?;
        }

        // part-name (required)
        serialize_part_name(w, "part-name", &self.part_name)?;

        // part-name-display
        if let Some(ref pnd) = self.part_name_display {
            super::print::serialize_name_display(w, "part-name-display", pnd)?;
        }

        // part-abbreviation
        if let Some(ref abbrev) = self.part_abbreviation {
            serialize_part_name(w, "part-abbreviation", abbrev)?;
        }

        // part-abbreviation-display
        if let Some(ref pad) = self.part_abbreviation_display {
            super::print::serialize_name_display(w, "part-abbreviation-display", pad)?;
        }

        // group
        for g in &self.groups {
            w.write_text_element("group", g)?;
        }

        // score-instrument
        for inst in &self.score_instruments {
            inst.serialize(w)?;
        }

        // player
        for p in &self.players {
            serialize_player(w, p)?;
        }

        // midi-device and midi-instrument
        for ma in &self.midi_assignments {
            match ma {
                MidiAssignment::MidiDevice(md) => md.serialize(w)?,
                MidiAssignment::MidiInstrument(mi) => mi.serialize(w)?,
            }
        }

        Ok(())
    }
}

fn serialize_part_name<W: Write>(
    w: &mut MusicXmlWriter<W>,
    name: &str,
    pn: &PartName,
) -> SerializeResult<()> {
    let mut start = w.start_element(name);
    if let Some(ref po) = pn.print_object {
        start.push_attribute(("print-object", yes_no_str(po)));
    }
    push_opt_attr_start(&mut start, "default-x", &pn.default_x);
    push_opt_attr_start(&mut start, "default-y", &pn.default_y);
    push_opt_str_attr_start(&mut start, "font-family", &pn.font_family);
    if let Some(ref style) = pn.font_style {
        start.push_attribute(("font-style", font_style_str(style)));
    }
    if let Some(ref size) = pn.font_size {
        start.push_attribute(("font-size", font_size_str(size).as_str()));
    }
    if let Some(ref weight) = pn.font_weight {
        start.push_attribute(("font-weight", font_weight_str(weight)));
    }
    if let Some(ref j) = pn.justify {
        start.push_attribute(("justify", left_center_right_str(j)));
    }
    w.write_start(start)?;
    w.write_text(&pn.value)?;
    w.write_end(name)?;
    Ok(())
}

fn serialize_player<W: Write>(w: &mut MusicXmlWriter<W>, p: &Player) -> SerializeResult<()> {
    let mut start = w.start_element("player");
    start.push_attribute(("id", p.id.as_str()));
    w.write_start(start)?;
    w.write_text_element("player-name", &p.player_name)?;
    w.write_end("player")?;
    Ok(())
}

fn serialize_part_link<W: Write>(w: &mut MusicXmlWriter<W>, pl: &PartLink) -> SerializeResult<()> {
    let mut start = w.start_element("part-link");
    start.push_attribute(("xlink:href", pl.href.as_str()));
    push_opt_str_attr_start(&mut start, "xlink:type", &pl.xlink_type);
    push_opt_str_attr_start(&mut start, "xlink:role", &pl.xlink_role);
    push_opt_str_attr_start(&mut start, "xlink:title", &pl.xlink_title);
    push_opt_str_attr_start(&mut start, "xlink:show", &pl.xlink_show);
    push_opt_str_attr_start(&mut start, "xlink:actuate", &pl.xlink_actuate);

    if pl.instrument_links.is_empty() && pl.group_links.is_empty() {
        w.write_empty(start)?;
    } else {
        w.write_start(start)?;
        for il in &pl.instrument_links {
            let mut il_start = w.start_element("instrument-link");
            il_start.push_attribute(("id", il.id.as_str()));
            w.write_empty(il_start)?;
        }
        for gl in &pl.group_links {
            w.write_text_element("group-link", gl)?;
        }
        w.write_end("part-link")?;
    }
    Ok(())
}

impl MusicXmlSerialize for ScoreInstrument {
    fn element_name(&self) -> &'static str {
        "score-instrument"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        vec![("id", self.id.clone())]
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text_element("instrument-name", &self.instrument_name)?;
        w.write_opt_text_element("instrument-abbreviation", &self.instrument_abbreviation)?;
        w.write_opt_text_element("instrument-sound", &self.instrument_sound)?;
        if self.solo.is_some() {
            w.write_empty(w.start_element("solo"))?;
        }
        if let Some(ref ens) = self.ensemble {
            if let Some(v) = ens.value {
                w.write_text_element("ensemble", &v.to_string())?;
            } else {
                w.write_empty(w.start_element("ensemble"))?;
            }
        }
        if let Some(ref vi) = self.virtual_instrument {
            vi.serialize(w)?;
        }
        Ok(())
    }
}

impl MusicXmlSerialize for VirtualInstrument {
    fn element_name(&self) -> &'static str {
        "virtual-instrument"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        self.virtual_library.is_some() || self.virtual_name.is_some()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_opt_text_element("virtual-library", &self.virtual_library)?;
        w.write_opt_text_element("virtual-name", &self.virtual_name)?;
        Ok(())
    }
}

impl MusicXmlSerialize for MidiDevice {
    fn element_name(&self) -> &'static str {
        "midi-device"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_str_attr!(attrs, "id", self.id);
        push_opt_attr!(attrs, "port", self.port);
        attrs
    }

    fn has_children(&self) -> bool {
        self.value.is_some()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        if let Some(ref v) = self.value {
            w.write_text(v)?;
        }
        Ok(())
    }
}

impl MusicXmlSerialize for MidiInstrument {
    fn element_name(&self) -> &'static str {
        "midi-instrument"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        vec![("id", self.id.clone())]
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        if let Some(ch) = self.midi_channel {
            w.write_text_element("midi-channel", &ch.to_string())?;
        }
        w.write_opt_text_element("midi-name", &self.midi_name)?;
        if let Some(b) = self.midi_bank {
            w.write_text_element("midi-bank", &b.to_string())?;
        }
        if let Some(p) = self.midi_program {
            w.write_text_element("midi-program", &p.to_string())?;
        }
        if let Some(u) = self.midi_unpitched {
            w.write_text_element("midi-unpitched", &u.to_string())?;
        }
        if let Some(v) = self.volume {
            w.write_text_element("volume", &v.to_string())?;
        }
        if let Some(p) = self.pan {
            w.write_text_element("pan", &p.to_string())?;
        }
        if let Some(e) = self.elevation {
            w.write_text_element("elevation", &e.to_string())?;
        }
        Ok(())
    }
}

// ============================================================================
// Part and Measure
// ============================================================================

impl MusicXmlSerialize for Part {
    fn element_name(&self) -> &'static str {
        "part"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        vec![("id", self.id.clone())]
    }

    fn has_children(&self) -> bool {
        !self.measures.is_empty()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        for measure in &self.measures {
            measure.serialize(w)?;
        }
        Ok(())
    }
}

impl MusicXmlSerialize for Measure {
    fn element_name(&self) -> &'static str {
        "measure"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = vec![("number", self.number.clone())];
        push_opt_str_attr!(attrs, "text", self.text);
        if let Some(ref imp) = self.implicit {
            attrs.push(("implicit", yes_no_str(imp).to_string()));
        }
        if let Some(ref nc) = self.non_controlling {
            attrs.push(("non-controlling", yes_no_str(nc).to_string()));
        }
        push_opt_attr!(attrs, "width", self.width);
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        !self.content.is_empty()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        for item in &self.content {
            match item {
                MeasureContent::Note(note) => note.serialize(w)?,
                MeasureContent::Backup(backup) => backup.serialize(w)?,
                MeasureContent::Forward(forward) => forward.serialize(w)?,
                MeasureContent::Attributes(attrs) => attrs.serialize(w)?,
                MeasureContent::Direction(dir) => dir.serialize(w)?,
                MeasureContent::Harmony(harmony) => harmony.serialize(w)?,
                MeasureContent::FiguredBass(fb) => fb.serialize(w)?,
                MeasureContent::Print(print) => print.serialize(w)?,
                MeasureContent::Sound(sound) => sound.serialize(w)?,
                MeasureContent::Listening(listening) => listening.serialize(w)?,
                MeasureContent::Barline(barline) => barline.serialize(w)?,
                MeasureContent::Grouping(grouping) => grouping.serialize(w)?,
                MeasureContent::Link(link) => link.serialize(w)?,
                MeasureContent::Bookmark(bookmark) => bookmark.serialize(w)?,
            }
        }
        Ok(())
    }
}

// ============================================================================
// Barline
// ============================================================================

impl MusicXmlSerialize for crate::model::elements::Barline {
    fn element_name(&self) -> &'static str {
        "barline"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref loc) = self.location {
            attrs.push(("location", loc.to_musicxml_str().to_string()));
        }
        if let Some(ref s) = self.segno_attr {
            attrs.push(("segno", s.clone()));
        }
        if let Some(ref c) = self.coda_attr {
            attrs.push(("coda", c.clone()));
        }
        if let Some(d) = self.divisions {
            attrs.push(("divisions", d.to_string()));
        }
        attrs
    }

    fn has_children(&self) -> bool {
        self.bar_style.is_some() || self.has_extra_children()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        // XSD order: bar-style, editorial, wavy-line, segno, coda, fermata(0-2), ending, repeat
        if let Some(ref style) = self.bar_style {
            w.write_text_element("bar-style", style.to_musicxml_str())?;
        }
        if let Some(ref ft) = self.footnote {
            super::elements::serialize_formatted_text(w, "footnote", ft)?;
        }
        if let Some(ref lv) = self.level {
            super::elements::serialize_level(w, lv)?;
        }
        if let Some(ref wl) = self.wavy_line {
            super::notations::serialize_wavy_line(w, wl)?;
        }
        if let Some(ref segno) = self.segno {
            serialize_segno_coda(
                w,
                "segno",
                segno.default_x,
                segno.default_y,
                &segno.color,
                segno.halign.as_ref(),
                segno.valign.as_ref(),
                &segno.smufl,
                &segno.id,
            )?;
        }
        if let Some(ref coda) = self.coda {
            serialize_segno_coda(
                w,
                "coda",
                coda.default_x,
                coda.default_y,
                &coda.color,
                coda.halign.as_ref(),
                coda.valign.as_ref(),
                &coda.smufl,
                &coda.id,
            )?;
        }
        for fermata in &self.fermatas {
            super::notations::serialize_fermata(w, fermata)?;
        }
        if let Some(ref ending) = self.ending {
            serialize_ending(w, ending)?;
        }
        if let Some(ref repeat) = self.repeat {
            serialize_repeat(w, repeat)?;
        }
        Ok(())
    }
}

/// Serialize a segno or coda empty element with position attributes.
#[allow(clippy::too_many_arguments)]
fn serialize_segno_coda<W: Write>(
    w: &mut MusicXmlWriter<W>,
    tag: &str,
    default_x: Option<f64>,
    default_y: Option<f64>,
    color: &Option<String>,
    halign: Option<&LeftCenterRight>,
    valign: Option<&Valign>,
    smufl: &Option<String>,
    id: &Option<String>,
) -> SerializeResult<()> {
    let mut elem = w.start_element(tag);
    push_opt_attr_start(&mut elem, "default-x", &default_x);
    push_opt_attr_start(&mut elem, "default-y", &default_y);
    push_opt_str_attr_start(&mut elem, "color", color);
    if let Some(h) = halign {
        elem.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(v) = valign {
        elem.push_attribute(("valign", valign_str(v)));
    }
    push_opt_str_attr_start(&mut elem, "smufl", smufl);
    push_opt_str_attr_start(&mut elem, "id", id);
    w.write_empty(elem)?;
    Ok(())
}

/// Serialize an ending element.
fn serialize_ending<W: Write>(
    w: &mut MusicXmlWriter<W>,
    ending: &crate::model::elements::Ending,
) -> SerializeResult<()> {
    let mut elem = w.start_element("ending");
    elem.push_attribute(("number", ending.number.as_str()));
    elem.push_attribute(("type", start_stop_discontinue_str(&ending.ending_type)));
    push_opt_attr_start(&mut elem, "default-y", &ending.default_y);
    push_opt_attr_start(&mut elem, "end-length", &ending.end_length);
    if let Some(ref po) = ending.print_object {
        elem.push_attribute(("print-object", yes_no_str(po)));
    }
    push_opt_attr_start(&mut elem, "default-x", &ending.default_x);
    push_opt_attr_start(&mut elem, "text-x", &ending.text_x);
    push_opt_attr_start(&mut elem, "text-y", &ending.text_y);
    if let Some(ref text) = ending.text {
        w.write_start(elem)?;
        w.write_text(text)?;
        w.write_end("ending")?;
    } else {
        w.write_empty(elem)?;
    }
    Ok(())
}

/// Serialize a repeat element.
fn serialize_repeat<W: Write>(
    w: &mut MusicXmlWriter<W>,
    repeat: &crate::model::elements::Repeat,
) -> SerializeResult<()> {
    use crate::model::elements::{BackwardForward, Winged};
    let mut elem = w.start_element("repeat");
    elem.push_attribute((
        "direction",
        match repeat.direction {
            BackwardForward::Forward => "forward",
            BackwardForward::Backward => "backward",
        },
    ));
    if let Some(t) = repeat.times {
        elem.push_attribute(("times", t.to_string().as_str()));
    }
    if let Some(ref aj) = repeat.after_jump {
        elem.push_attribute(("after-jump", yes_no_str(aj)));
    }
    if let Some(ref w_val) = repeat.winged {
        elem.push_attribute((
            "winged",
            match w_val {
                Winged::None => "none",
                Winged::Straight => "straight",
                Winged::Curved => "curved",
                Winged::DoubleStraight => "double-straight",
                Winged::DoubleCurved => "double-curved",
            },
        ));
    }
    w.write_empty(elem)?;
    Ok(())
}

fn start_stop_discontinue_str(ssd: &StartStopDiscontinue) -> &'static str {
    match ssd {
        StartStopDiscontinue::Start => "start",
        StartStopDiscontinue::Stop => "stop",
        StartStopDiscontinue::Discontinue => "discontinue",
    }
}

// ============================================================================
// Helper functions for enum to string conversion
// ============================================================================

pub(crate) fn yes_no_str(yn: &YesNo) -> &'static str {
    match yn {
        YesNo::Yes => "yes",
        YesNo::No => "no",
    }
}

pub(crate) fn start_stop_str(ss: &StartStop) -> &'static str {
    match ss {
        StartStop::Start => "start",
        StartStop::Stop => "stop",
    }
}

pub(crate) fn show_tuplet_str(st: &notations::ShowTuplet) -> &'static str {
    match st {
        notations::ShowTuplet::Actual => "actual",
        notations::ShowTuplet::Both => "both",
        notations::ShowTuplet::None => "none",
    }
}

pub(crate) fn line_shape_str(ls: &LineShape) -> &'static str {
    match ls {
        LineShape::Straight => "straight",
        LineShape::Curved => "curved",
    }
}

pub(crate) fn start_stop_continue_str(ssc: &StartStopContinue) -> &'static str {
    match ssc {
        StartStopContinue::Start => "start",
        StartStopContinue::Stop => "stop",
        StartStopContinue::Continue => "continue",
    }
}

pub(crate) fn tied_type_str(tt: &notations::TiedType) -> &'static str {
    match tt {
        notations::TiedType::Start => "start",
        notations::TiedType::Stop => "stop",
        notations::TiedType::Continue => "continue",
        notations::TiedType::LetRing => "let-ring",
    }
}

pub(crate) fn over_under_str(ou: &OverUnder) -> &'static str {
    match ou {
        OverUnder::Over => "over",
        OverUnder::Under => "under",
    }
}

pub(crate) fn up_down_str(ud: &UpDown) -> &'static str {
    match ud {
        UpDown::Up => "up",
        UpDown::Down => "down",
    }
}

fn margin_type_str(mt: &MarginType) -> &'static str {
    match mt {
        MarginType::Odd => "odd",
        MarginType::Even => "even",
        MarginType::Both => "both",
    }
}

fn group_symbol_str(gs: &GroupSymbol) -> &'static str {
    match gs {
        GroupSymbol::None => "none",
        GroupSymbol::Brace => "brace",
        GroupSymbol::Line => "line",
        GroupSymbol::Bracket => "bracket",
        GroupSymbol::Square => "square",
    }
}

fn group_barline_str(gb: &GroupBarline) -> &'static str {
    match gb {
        GroupBarline::Yes => "yes",
        GroupBarline::No => "no",
        GroupBarline::Mensurstrich => "Mensurstrich",
    }
}

pub(crate) fn step_str(s: &Step) -> &'static str {
    match s {
        Step::A => "A",
        Step::B => "B",
        Step::C => "C",
        Step::D => "D",
        Step::E => "E",
        Step::F => "F",
        Step::G => "G",
    }
}

pub(crate) fn font_style_str(fs: &FontStyle) -> &'static str {
    match fs {
        FontStyle::Normal => "normal",
        FontStyle::Italic => "italic",
    }
}

pub(crate) fn font_weight_str(fw: &FontWeight) -> &'static str {
    match fw {
        FontWeight::Normal => "normal",
        FontWeight::Bold => "bold",
    }
}

pub(crate) fn font_size_str(fs: &FontSize) -> String {
    match fs {
        FontSize::Points(p) => p.to_string(),
        FontSize::Css(s) => s.to_string(),
    }
}

pub(crate) fn left_center_right_str(lcr: &LeftCenterRight) -> &'static str {
    match lcr {
        LeftCenterRight::Left => "left",
        LeftCenterRight::Center => "center",
        LeftCenterRight::Right => "right",
    }
}

pub(crate) fn valign_str(v: &Valign) -> &'static str {
    match v {
        Valign::Top => "top",
        Valign::Middle => "middle",
        Valign::Bottom => "bottom",
        Valign::Baseline => "baseline",
    }
}

pub(crate) fn above_below_str(ab: &AboveBelow) -> &'static str {
    match ab {
        AboveBelow::Above => "above",
        AboveBelow::Below => "below",
    }
}

pub(crate) fn left_right_str(lr: &LeftRight) -> &'static str {
    match lr {
        LeftRight::Left => "left",
        LeftRight::Right => "right",
    }
}

pub(crate) fn top_bottom_str(tb: &TopBottom) -> &'static str {
    match tb {
        TopBottom::Top => "top",
        TopBottom::Bottom => "bottom",
    }
}

pub(crate) fn upright_inverted_str(ui: &UprightInverted) -> &'static str {
    match ui {
        UprightInverted::Upright => "upright",
        UprightInverted::Inverted => "inverted",
    }
}

pub(crate) fn line_type_str(lt: &LineType) -> &'static str {
    match lt {
        LineType::Solid => "solid",
        LineType::Dashed => "dashed",
        LineType::Dotted => "dotted",
        LineType::Wavy => "wavy",
    }
}

pub(crate) fn start_stop_single_str(sss: &StartStopSingle) -> &'static str {
    match sss {
        StartStopSingle::Start => "start",
        StartStopSingle::Stop => "stop",
        StartStopSingle::Single => "single",
    }
}

pub(crate) fn fermata_shape_str(fs: &notations::FermataShape) -> &'static str {
    match fs {
        notations::FermataShape::Normal => "normal",
        notations::FermataShape::Angled => "angled",
        notations::FermataShape::Square => "square",
        notations::FermataShape::DoubleAngled => "double-angled",
        notations::FermataShape::DoubleSquare => "double-square",
        notations::FermataShape::DoubleDot => "double-dot",
        notations::FermataShape::HalfCurve => "half-curve",
        notations::FermataShape::Curlew => "curlew",
        notations::FermataShape::Empty => "",
    }
}

pub(crate) fn symbol_size_str(ss: &SymbolSize) -> &'static str {
    match ss {
        SymbolSize::Full => "full",
        SymbolSize::Cue => "cue",
        SymbolSize::GraceCue => "grace-cue",
        SymbolSize::Large => "large",
    }
}

pub(crate) fn stem_value_str(sv: &StemValue) -> &'static str {
    match sv {
        StemValue::Down => "down",
        StemValue::Up => "up",
        StemValue::Double => "double",
        StemValue::None => "none",
    }
}

pub(crate) fn beam_value_str(bv: &BeamValue) -> &'static str {
    match bv {
        BeamValue::Begin => "begin",
        BeamValue::Continue => "continue",
        BeamValue::End => "end",
        BeamValue::ForwardHook => "forward hook",
        BeamValue::BackwardHook => "backward hook",
    }
}

pub(crate) fn fan_str(f: &Fan) -> &'static str {
    match f {
        Fan::Accel => "accel",
        Fan::Rit => "rit",
        Fan::None => "none",
    }
}

pub(crate) fn notehead_value_str(nv: &NoteheadValue) -> &'static str {
    match nv {
        NoteheadValue::Slash => "slash",
        NoteheadValue::Triangle => "triangle",
        NoteheadValue::Diamond => "diamond",
        NoteheadValue::Square => "square",
        NoteheadValue::Cross => "cross",
        NoteheadValue::X => "x",
        NoteheadValue::CircleX => "circle-x",
        NoteheadValue::InvertedTriangle => "inverted triangle",
        NoteheadValue::ArrowDown => "arrow down",
        NoteheadValue::ArrowUp => "arrow up",
        NoteheadValue::Circled => "circled",
        NoteheadValue::Slashed => "slashed",
        NoteheadValue::BackSlashed => "back slashed",
        NoteheadValue::Normal => "normal",
        NoteheadValue::Cluster => "cluster",
        NoteheadValue::CircleDot => "circle dot",
        NoteheadValue::LeftTriangle => "left triangle",
        NoteheadValue::Rectangle => "rectangle",
        NoteheadValue::None => "none",
        NoteheadValue::Do => "do",
        NoteheadValue::Re => "re",
        NoteheadValue::Mi => "mi",
        NoteheadValue::Fa => "fa",
        NoteheadValue::FaUp => "fa up",
        NoteheadValue::So => "so",
        NoteheadValue::La => "la",
        NoteheadValue::Ti => "ti",
        NoteheadValue::Other => "other",
    }
}

pub(crate) fn accidental_value_str(av: &AccidentalValue) -> &'static str {
    match av {
        AccidentalValue::Sharp => "sharp",
        AccidentalValue::Natural => "natural",
        AccidentalValue::Flat => "flat",
        AccidentalValue::DoubleSharp => "double-sharp",
        AccidentalValue::SharpSharp => "sharp-sharp",
        AccidentalValue::FlatFlat => "flat-flat",
        AccidentalValue::NaturalSharp => "natural-sharp",
        AccidentalValue::NaturalFlat => "natural-flat",
        AccidentalValue::QuarterFlat => "quarter-flat",
        AccidentalValue::QuarterSharp => "quarter-sharp",
        AccidentalValue::ThreeQuartersFlat => "three-quarters-flat",
        AccidentalValue::ThreeQuartersSharp => "three-quarters-sharp",
        AccidentalValue::SharpDown => "sharp-down",
        AccidentalValue::SharpUp => "sharp-up",
        AccidentalValue::NaturalDown => "natural-down",
        AccidentalValue::NaturalUp => "natural-up",
        AccidentalValue::FlatDown => "flat-down",
        AccidentalValue::FlatUp => "flat-up",
        AccidentalValue::DoubleSharpDown => "double-sharp-down",
        AccidentalValue::DoubleSharpUp => "double-sharp-up",
        AccidentalValue::FlatFlatDown => "flat-flat-down",
        AccidentalValue::FlatFlatUp => "flat-flat-up",
        AccidentalValue::ArrowDown => "arrow-down",
        AccidentalValue::ArrowUp => "arrow-up",
        AccidentalValue::TripleSharp => "triple-sharp",
        AccidentalValue::TripleFlat => "triple-flat",
        AccidentalValue::SlashQuarterSharp => "slash-quarter-sharp",
        AccidentalValue::SlashSharp => "slash-sharp",
        AccidentalValue::SlashFlat => "slash-flat",
        AccidentalValue::DoubleSlashFlat => "double-slash-flat",
        AccidentalValue::Sharp1 => "sharp-1",
        AccidentalValue::Sharp2 => "sharp-2",
        AccidentalValue::Sharp3 => "sharp-3",
        AccidentalValue::Sharp5 => "sharp-5",
        AccidentalValue::Flat1 => "flat-1",
        AccidentalValue::Flat2 => "flat-2",
        AccidentalValue::Flat3 => "flat-3",
        AccidentalValue::Flat4 => "flat-4",
        AccidentalValue::Sori => "sori",
        AccidentalValue::Koron => "koron",
        AccidentalValue::Other => "other",
    }
}

pub(crate) fn clef_sign_str(cs: &ClefSign) -> &'static str {
    match cs {
        ClefSign::G => "G",
        ClefSign::F => "F",
        ClefSign::C => "C",
        ClefSign::Percussion => "percussion",
        ClefSign::Tab => "TAB",
        ClefSign::Jianpu => "jianpu",
        ClefSign::None => "none",
    }
}

pub(crate) fn time_symbol_str(ts: &TimeSymbol) -> &'static str {
    match ts {
        TimeSymbol::Common => "common",
        TimeSymbol::Cut => "cut",
        TimeSymbol::SingleNumber => "single-number",
        TimeSymbol::Note => "note",
        TimeSymbol::DottedNote => "dotted-note",
        TimeSymbol::Normal => "normal",
    }
}

pub(crate) fn time_separator_str(ts: &TimeSeparator) -> &'static str {
    match ts {
        TimeSeparator::None => "none",
        TimeSeparator::Horizontal => "horizontal",
        TimeSeparator::Diagonal => "diagonal",
        TimeSeparator::Vertical => "vertical",
        TimeSeparator::Adjacent => "adjacent",
    }
}

pub(crate) fn time_relation_str(tr: &TimeRelation) -> &'static str {
    match tr {
        TimeRelation::Parentheses => "parentheses",
        TimeRelation::Bracket => "bracket",
        TimeRelation::Equals => "equals",
        TimeRelation::Slash => "slash",
        TimeRelation::Space => "space",
        TimeRelation::Hyphen => "hyphen",
    }
}

pub(crate) fn wedge_type_str(wt: &WedgeType) -> &'static str {
    match wt {
        WedgeType::Crescendo => "crescendo",
        WedgeType::Diminuendo => "diminuendo",
        WedgeType::Stop => "stop",
        WedgeType::Continue => "continue",
    }
}

pub(crate) fn mode_str(m: &Mode) -> &'static str {
    match m {
        Mode::Major => "major",
        Mode::Minor => "minor",
        Mode::Dorian => "dorian",
        Mode::Phrygian => "phrygian",
        Mode::Lydian => "lydian",
        Mode::Mixolydian => "mixolydian",
        Mode::Aeolian => "aeolian",
        Mode::Ionian => "ionian",
        Mode::Locrian => "locrian",
        Mode::None => "none",
        Mode::Other(_) => "other", // Simplified - actual value would need dynamic string
    }
}

// Helper to push optional attribute to BytesStart
pub(crate) fn push_opt_attr_start<T: std::fmt::Display>(
    start: &mut quick_xml::events::BytesStart<'_>,
    name: &'static str,
    opt: &Option<T>,
) {
    if let Some(v) = opt {
        start.push_attribute((name, v.to_string().as_str()));
    }
}

pub(crate) fn push_opt_str_attr_start(
    start: &mut quick_xml::events::BytesStart<'_>,
    name: &'static str,
    opt: &Option<String>,
) {
    if let Some(v) = opt {
        start.push_attribute((name, v.as_str()));
    }
}

// ============================================================================
// ScoreTimewise serialization
// ============================================================================

impl ScoreTimewise {
    /// Serialize this timewise score to a MusicXML writer.
    ///
    /// Uses `<score-timewise>` as the root element and nests parts inside
    /// measures (the inverse of partwise).
    pub fn serialize_timewise<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        let mut start = w.start_element("score-timewise");
        if let Some(ref v) = self.version {
            start.push_attribute(("version", v.as_str()));
        }
        w.write_start(start)?;

        // Header elements (same order as partwise)
        if let Some(ref work) = self.work {
            work.serialize(w)?;
        }
        w.write_opt_text_element("movement-number", &self.movement_number)?;
        w.write_opt_text_element("movement-title", &self.movement_title)?;
        if let Some(ref ident) = self.identification {
            ident.serialize(w)?;
        }
        if let Some(ref defaults) = self.defaults {
            defaults.serialize(w)?;
        }
        for credit in &self.credits {
            credit.serialize(w)?;
        }
        self.part_list.serialize(w)?;

        // Measures (each containing parts)
        for measure in &self.measures {
            measure.serialize_timewise(w)?;
        }

        w.write_end("score-timewise")?;
        Ok(())
    }
}

impl TimewiseMeasure {
    /// Serialize a timewise measure to the writer.
    pub fn serialize_timewise<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        let mut start = w.start_element("measure");
        start.push_attribute(("number", self.number.as_str()));
        if let Some(ref text) = self.text {
            start.push_attribute(("text", text.as_str()));
        }
        if let Some(ref imp) = self.implicit {
            start.push_attribute(("implicit", yes_no_str(imp)));
        }
        if let Some(ref nc) = self.non_controlling {
            start.push_attribute(("non-controlling", yes_no_str(nc)));
        }
        if let Some(width) = self.width {
            let s = width.to_string();
            start.push_attribute(("width", s.as_str()));
        }
        w.write_start(start)?;

        for part in &self.parts {
            part.serialize_timewise(w)?;
        }

        w.write_end("measure")?;
        Ok(())
    }
}

impl TimewisePart {
    /// Serialize a timewise part (within a measure) to the writer.
    pub fn serialize_timewise<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        let mut start = w.start_element("part");
        start.push_attribute(("id", self.id.as_str()));
        w.write_start(start)?;

        for item in &self.content {
            match item {
                MeasureContent::Note(note) => note.serialize(w)?,
                MeasureContent::Backup(backup) => backup.serialize(w)?,
                MeasureContent::Forward(forward) => forward.serialize(w)?,
                MeasureContent::Attributes(attrs) => attrs.serialize(w)?,
                MeasureContent::Direction(dir) => dir.serialize(w)?,
                MeasureContent::Harmony(harmony) => harmony.serialize(w)?,
                MeasureContent::FiguredBass(fb) => fb.serialize(w)?,
                MeasureContent::Print(print) => print.serialize(w)?,
                MeasureContent::Sound(sound) => sound.serialize(w)?,
                MeasureContent::Listening(listening) => listening.serialize(w)?,
                MeasureContent::Barline(barline) => barline.serialize(w)?,
                MeasureContent::Grouping(grouping) => grouping.serialize(w)?,
                MeasureContent::Link(link) => link.serialize(w)?,
                MeasureContent::Bookmark(bookmark) => bookmark.serialize(w)?,
            }
        }

        w.write_end("part")?;
        Ok(())
    }
}
