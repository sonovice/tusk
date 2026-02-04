//! Serializer implementations for MusicXML score types.
//!
//! This module contains `MusicXmlSerialize` implementations for:
//! - ScorePartwise, Part, Measure
//! - Work, Identification, Defaults
//! - PartList, ScorePart, PartGroup
//! - Note, Pitch, Rest, and related types
//! - Attributes, Direction
//! - Backup, Forward

use std::io::Write;

use crate::model::*;
use crate::serializer::{MusicXmlSerialize, MusicXmlWriter, SerializeResult, push_opt_attr, push_opt_str_attr};

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
        if let Some(ref page_layout) = self.page_layout {
            page_layout.serialize(w)?;
        }
        if let Some(ref system_layout) = self.system_layout {
            system_layout.serialize(w)?;
        }
        for staff_layout in &self.staff_layouts {
            staff_layout.serialize(w)?;
        }
        // TODO: appearance, fonts, etc.
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
            || self.group_abbreviation.is_some()
            || self.group_symbol.is_some()
            || self.group_barline.is_some()
            || self.group_time.is_some()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_opt_text_element("group-name", &self.group_name)?;
        w.write_opt_text_element("group-abbreviation", &self.group_abbreviation)?;
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
        // part-name (required)
        serialize_part_name(w, "part-name", &self.part_name)?;

        // part-abbreviation
        if let Some(ref abbrev) = self.part_abbreviation {
            serialize_part_name(w, "part-abbreviation", abbrev)?;
        }

        // score-instrument
        for inst in &self.score_instruments {
            inst.serialize(w)?;
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
                MeasureContent::Barline(_) => {
                    // TODO: implement barline serialization
                }
            }
        }
        Ok(())
    }
}

// ============================================================================
// Note
// ============================================================================

impl MusicXmlSerialize for Note {
    fn element_name(&self) -> &'static str {
        "note"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "default-x", self.default_x);
        push_opt_attr!(attrs, "default-y", self.default_y);
        push_opt_attr!(attrs, "relative-x", self.relative_x);
        push_opt_attr!(attrs, "relative-y", self.relative_y);
        if let Some(ref po) = self.print_object {
            attrs.push(("print-object", yes_no_str(po).to_string()));
        }
        if let Some(ref pl) = self.print_leger {
            attrs.push(("print-leger", yes_no_str(pl).to_string()));
        }
        if let Some(ref ps) = self.print_spacing {
            attrs.push(("print-spacing", yes_no_str(ps).to_string()));
        }
        push_opt_attr!(attrs, "dynamics", self.dynamics);
        push_opt_attr!(attrs, "end-dynamics", self.end_dynamics);
        push_opt_attr!(attrs, "attack", self.attack);
        push_opt_attr!(attrs, "release", self.release);
        if let Some(ref piz) = self.pizzicato {
            attrs.push(("pizzicato", yes_no_str(piz).to_string()));
        }
        push_opt_str_attr!(attrs, "color", self.color);
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        // Grace
        if let Some(ref grace) = self.grace {
            grace.serialize(w)?;
        }

        // Cue
        if self.cue.is_some() {
            w.write_empty(w.start_element("cue"))?;
        }

        // Chord
        if self.chord.is_some() {
            w.write_empty(w.start_element("chord"))?;
        }

        // Pitch/Unpitched/Rest
        match &self.content {
            FullNoteContent::Pitch(pitch) => pitch.serialize(w)?,
            FullNoteContent::Unpitched(unpitched) => unpitched.serialize(w)?,
            FullNoteContent::Rest(rest) => rest.serialize(w)?,
        }

        // Duration
        if let Some(dur) = self.duration {
            w.write_text_element("duration", &dur.to_string())?;
        }

        // Ties
        for tie in &self.ties {
            tie.serialize(w)?;
        }

        // Voice
        w.write_opt_text_element("voice", &self.voice)?;

        // Type
        if let Some(ref nt) = self.note_type {
            nt.serialize(w)?;
        }

        // Dots
        for dot in &self.dots {
            dot.serialize(w)?;
        }

        // Accidental
        if let Some(ref acc) = self.accidental {
            acc.serialize(w)?;
        }

        // Time modification
        if let Some(ref tm) = self.time_modification {
            tm.serialize(w)?;
        }

        // Stem
        if let Some(ref stem) = self.stem {
            stem.serialize(w)?;
        }

        // Notehead
        if let Some(ref nh) = self.notehead {
            nh.serialize(w)?;
        }

        // Staff
        if let Some(staff) = self.staff {
            w.write_text_element("staff", &staff.to_string())?;
        }

        // Beams
        for beam in &self.beams {
            beam.serialize(w)?;
        }

        Ok(())
    }
}

impl MusicXmlSerialize for Grace {
    fn element_name(&self) -> &'static str {
        "grace"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "steal-time-previous", self.steal_time_previous);
        push_opt_attr!(attrs, "steal-time-following", self.steal_time_following);
        push_opt_attr!(attrs, "make-time", self.make_time);
        if let Some(ref s) = self.slash {
            attrs.push(("slash", yes_no_str(s).to_string()));
        }
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MusicXmlSerialize for Pitch {
    fn element_name(&self) -> &'static str {
        "pitch"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text_element("step", step_str(&self.step))?;
        if let Some(alter) = self.alter {
            w.write_text_element("alter", &alter.to_string())?;
        }
        w.write_text_element("octave", &self.octave.to_string())?;
        Ok(())
    }
}

impl MusicXmlSerialize for Unpitched {
    fn element_name(&self) -> &'static str {
        "unpitched"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        self.display_step.is_some() || self.display_octave.is_some()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        if let Some(ref step) = self.display_step {
            w.write_text_element("display-step", step_str(step))?;
        }
        if let Some(oct) = self.display_octave {
            w.write_text_element("display-octave", &oct.to_string())?;
        }
        Ok(())
    }
}

impl MusicXmlSerialize for Rest {
    fn element_name(&self) -> &'static str {
        "rest"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref m) = self.measure {
            attrs.push(("measure", yes_no_str(m).to_string()));
        }
        attrs
    }

    fn has_children(&self) -> bool {
        self.display_step.is_some() || self.display_octave.is_some()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        if let Some(ref step) = self.display_step {
            w.write_text_element("display-step", step_str(step))?;
        }
        if let Some(oct) = self.display_octave {
            w.write_text_element("display-octave", &oct.to_string())?;
        }
        Ok(())
    }
}

impl MusicXmlSerialize for Tie {
    fn element_name(&self) -> &'static str {
        "tie"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = vec![("type", start_stop_str(&self.tie_type).to_string())];
        push_opt_str_attr!(attrs, "time-only", self.time_only);
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MusicXmlSerialize for NoteType {
    fn element_name(&self) -> &'static str {
        "type"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref size) = self.size {
            attrs.push(("size", symbol_size_str(size).to_string()));
        }
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text(&self.value.to_string())?;
        Ok(())
    }
}

impl MusicXmlSerialize for Dot {
    fn element_name(&self) -> &'static str {
        "dot"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "default-x", self.default_x);
        push_opt_attr!(attrs, "default-y", self.default_y);
        push_opt_attr!(attrs, "relative-x", self.relative_x);
        push_opt_attr!(attrs, "relative-y", self.relative_y);
        if let Some(ref p) = self.placement {
            attrs.push(("placement", above_below_str(p).to_string()));
        }
        push_opt_str_attr!(attrs, "color", self.color);
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MusicXmlSerialize for Accidental {
    fn element_name(&self) -> &'static str {
        "accidental"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref c) = self.cautionary {
            attrs.push(("cautionary", yes_no_str(c).to_string()));
        }
        if let Some(ref e) = self.editorial {
            attrs.push(("editorial", yes_no_str(e).to_string()));
        }
        if let Some(ref p) = self.parentheses {
            attrs.push(("parentheses", yes_no_str(p).to_string()));
        }
        if let Some(ref b) = self.bracket {
            attrs.push(("bracket", yes_no_str(b).to_string()));
        }
        if let Some(ref s) = self.size {
            attrs.push(("size", symbol_size_str(s).to_string()));
        }
        push_opt_str_attr!(attrs, "smufl", self.smufl);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text(accidental_value_str(&self.value))?;
        Ok(())
    }
}

impl MusicXmlSerialize for TimeModification {
    fn element_name(&self) -> &'static str {
        "time-modification"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text_element("actual-notes", &self.actual_notes.to_string())?;
        w.write_text_element("normal-notes", &self.normal_notes.to_string())?;
        if let Some(ref nt) = self.normal_type {
            w.write_text_element("normal-type", &nt.to_string())?;
        }
        for _ in &self.normal_dots {
            w.write_empty(w.start_element("normal-dot"))?;
        }
        Ok(())
    }
}

impl MusicXmlSerialize for Stem {
    fn element_name(&self) -> &'static str {
        "stem"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "default-y", self.default_y);
        push_opt_attr!(attrs, "relative-y", self.relative_y);
        push_opt_str_attr!(attrs, "color", self.color);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text(stem_value_str(&self.value))?;
        Ok(())
    }
}

impl MusicXmlSerialize for Notehead {
    fn element_name(&self) -> &'static str {
        "notehead"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref f) = self.filled {
            attrs.push(("filled", yes_no_str(f).to_string()));
        }
        if let Some(ref p) = self.parentheses {
            attrs.push(("parentheses", yes_no_str(p).to_string()));
        }
        push_opt_str_attr!(attrs, "font-family", self.font_family);
        if let Some(ref style) = self.font_style {
            attrs.push(("font-style", font_style_str(style).to_string()));
        }
        if let Some(ref size) = self.font_size {
            attrs.push(("font-size", font_size_str(size)));
        }
        if let Some(ref weight) = self.font_weight {
            attrs.push(("font-weight", font_weight_str(weight).to_string()));
        }
        push_opt_str_attr!(attrs, "color", self.color);
        push_opt_str_attr!(attrs, "smufl", self.smufl);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text(notehead_value_str(&self.value))?;
        Ok(())
    }
}

impl MusicXmlSerialize for Beam {
    fn element_name(&self) -> &'static str {
        "beam"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "number", self.number);
        if let Some(ref r) = self.repeater {
            attrs.push(("repeater", yes_no_str(r).to_string()));
        }
        if let Some(ref f) = self.fan {
            attrs.push(("fan", fan_str(f).to_string()));
        }
        push_opt_str_attr!(attrs, "color", self.color);
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text(beam_value_str(&self.value))?;
        Ok(())
    }
}

// ============================================================================
// Backup and Forward
// ============================================================================

impl MusicXmlSerialize for Backup {
    fn element_name(&self) -> &'static str {
        "backup"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text_element("duration", &self.duration.to_string())?;
        Ok(())
    }
}

impl MusicXmlSerialize for Forward {
    fn element_name(&self) -> &'static str {
        "forward"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text_element("duration", &self.duration.to_string())?;
        w.write_opt_text_element("voice", &self.voice)?;
        if let Some(staff) = self.staff {
            w.write_text_element("staff", &staff.to_string())?;
        }
        Ok(())
    }
}

// ============================================================================
// Attributes (stub - will be expanded)
// ============================================================================

impl MusicXmlSerialize for Attributes {
    fn element_name(&self) -> &'static str {
        "attributes"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        // Divisions
        if let Some(div) = self.divisions {
            w.write_text_element("divisions", &div.to_string())?;
        }

        // Key
        for key in &self.keys {
            key.serialize(w)?;
        }

        // Time
        for time in &self.times {
            time.serialize(w)?;
        }

        // Staves
        if let Some(staves) = self.staves {
            w.write_text_element("staves", &staves.to_string())?;
        }

        // Clef
        for clef in &self.clefs {
            clef.serialize(w)?;
        }

        Ok(())
    }
}

impl MusicXmlSerialize for Key {
    fn element_name(&self) -> &'static str {
        "key"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "number", self.number);
        if let Some(ref po) = self.print_object {
            attrs.push(("print-object", yes_no_str(po).to_string()));
        }
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        match &self.content {
            KeyContent::Traditional(trad) => {
                if let Some(ref cancel) = trad.cancel {
                    w.write_text_element("cancel", &cancel.fifths.to_string())?;
                }
                w.write_text_element("fifths", &trad.fifths.to_string())?;
                if let Some(ref mode) = trad.mode {
                    w.write_text_element("mode", mode_str(mode))?;
                }
            }
            KeyContent::NonTraditional(nt) => {
                for alt in &nt.alterations {
                    w.write_text_element("key-step", step_str(&alt.key_step))?;
                    w.write_text_element("key-alter", &alt.key_alter.to_string())?;
                }
            }
        }
        Ok(())
    }
}

impl MusicXmlSerialize for Time {
    fn element_name(&self) -> &'static str {
        "time"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "number", self.number);
        if let Some(ref sym) = self.symbol {
            attrs.push(("symbol", time_symbol_str(sym).to_string()));
        }
        if let Some(ref po) = self.print_object {
            attrs.push(("print-object", yes_no_str(po).to_string()));
        }
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        match &self.content {
            TimeContent::Standard(std) => {
                for sig in &std.signatures {
                    w.write_text_element("beats", &sig.beats)?;
                    w.write_text_element("beat-type", &sig.beat_type)?;
                }
            }
            TimeContent::SenzaMisura(sm) => {
                if let Some(ref symbol) = sm.symbol {
                    w.write_text_element("senza-misura", symbol)?;
                } else {
                    w.write_empty(w.start_element("senza-misura"))?;
                }
            }
        }
        Ok(())
    }
}

impl MusicXmlSerialize for Clef {
    fn element_name(&self) -> &'static str {
        "clef"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "number", self.number);
        if let Some(ref po) = self.print_object {
            attrs.push(("print-object", yes_no_str(po).to_string()));
        }
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        w.write_text_element("sign", clef_sign_str(&self.sign))?;
        if let Some(line) = self.line {
            w.write_text_element("line", &line.to_string())?;
        }
        if let Some(oct) = self.clef_octave_change {
            w.write_text_element("clef-octave-change", &oct.to_string())?;
        }
        Ok(())
    }
}

// ============================================================================
// Direction (stub)
// ============================================================================

impl MusicXmlSerialize for Direction {
    fn element_name(&self) -> &'static str {
        "direction"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref p) = self.placement {
            attrs.push(("placement", above_below_str(p).to_string()));
        }
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        // Direction types
        for dt in &self.direction_types {
            dt.serialize(w)?;
        }

        // Offset
        if let Some(ref offset) = self.offset {
            let mut start = w.start_element("offset");
            if let Some(ref sound) = offset.sound {
                start.push_attribute(("sound", yes_no_str(sound)));
            }
            w.write_start(start)?;
            w.write_text(&offset.value.to_string())?;
            w.write_end("offset")?;
        }

        // Staff
        if let Some(staff) = self.staff {
            w.write_text_element("staff", &staff.to_string())?;
        }

        // Sound (if present, needs further implementation)

        Ok(())
    }
}

impl MusicXmlSerialize for DirectionType {
    fn element_name(&self) -> &'static str {
        "direction-type"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        match &self.content {
            DirectionTypeContent::Dynamics(dynamics) => {
                let start = w.start_element("dynamics");
                w.write_start(start)?;
                for d in &dynamics.values {
                    serialize_dynamics_value(w, d)?;
                }
                w.write_end("dynamics")?;
            }
            DirectionTypeContent::Wedge(wedge) => {
                wedge.serialize(w)?;
            }
            DirectionTypeContent::Metronome(metronome) => {
                metronome.serialize(w)?;
            }
            DirectionTypeContent::Words(words) => {
                for word in words {
                    serialize_words(w, word)?;
                }
            }
            _ => {
                // TODO: implement other direction types (Rehearsal, Segno, Coda, etc.)
            }
        }
        Ok(())
    }
}

fn serialize_dynamics_value<W: Write>(w: &mut MusicXmlWriter<W>, value: &DynamicsValue) -> SerializeResult<()> {
    let name = match value {
        DynamicsValue::P => "p",
        DynamicsValue::Pp => "pp",
        DynamicsValue::Ppp => "ppp",
        DynamicsValue::Pppp => "pppp",
        DynamicsValue::Ppppp => "ppppp",
        DynamicsValue::Pppppp => "pppppp",
        DynamicsValue::F => "f",
        DynamicsValue::Ff => "ff",
        DynamicsValue::Fff => "fff",
        DynamicsValue::Ffff => "ffff",
        DynamicsValue::Fffff => "fffff",
        DynamicsValue::Ffffff => "ffffff",
        DynamicsValue::Mp => "mp",
        DynamicsValue::Mf => "mf",
        DynamicsValue::Sf => "sf",
        DynamicsValue::Sfp => "sfp",
        DynamicsValue::Sfpp => "sfpp",
        DynamicsValue::Fp => "fp",
        DynamicsValue::Rf => "rf",
        DynamicsValue::Rfz => "rfz",
        DynamicsValue::Sfz => "sfz",
        DynamicsValue::Sffz => "sffz",
        DynamicsValue::Fz => "fz",
        DynamicsValue::N => "n",
        DynamicsValue::Sfzp => "sfzp",
        DynamicsValue::OtherDynamics(s) => {
            let start = w.start_element("other-dynamics");
            w.write_start(start)?;
            w.write_text(s)?;
            w.write_end("other-dynamics")?;
            return Ok(());
        }
    };
    w.write_empty(w.start_element(name))?;
    Ok(())
}

fn serialize_words<W: Write>(w: &mut MusicXmlWriter<W>, words: &Words) -> SerializeResult<()> {
    let mut start = w.start_element("words");
    push_opt_str_attr_start(&mut start, "font-family", &words.font_family);
    if let Some(ref style) = words.font_style {
        start.push_attribute(("font-style", font_style_str(style)));
    }
    if let Some(ref size) = words.font_size {
        start.push_attribute(("font-size", font_size_str(size).as_str()));
    }
    if let Some(ref weight) = words.font_weight {
        start.push_attribute(("font-weight", font_weight_str(weight)));
    }
    push_opt_attr_start(&mut start, "default-x", &words.default_x);
    push_opt_attr_start(&mut start, "default-y", &words.default_y);
    if let Some(ref j) = words.justify {
        start.push_attribute(("justify", left_center_right_str(j)));
    }
    if let Some(ref h) = words.halign {
        start.push_attribute(("halign", left_center_right_str(h)));
    }
    if let Some(ref v) = words.valign {
        start.push_attribute(("valign", valign_str(v)));
    }
    w.write_start(start)?;
    w.write_text(&words.value)?;
    w.write_end("words")?;
    Ok(())
}

impl MusicXmlSerialize for Wedge {
    fn element_name(&self) -> &'static str {
        "wedge"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = vec![("type", wedge_type_str(&self.wedge_type).to_string())];
        push_opt_attr!(attrs, "number", self.number);
        push_opt_attr!(attrs, "spread", self.spread);
        if let Some(ref niente) = self.niente {
            attrs.push(("niente", yes_no_str(niente).to_string()));
        }
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MusicXmlSerialize for Metronome {
    fn element_name(&self) -> &'static str {
        "metronome"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref p) = self.parentheses {
            attrs.push(("parentheses", yes_no_str(p).to_string()));
        }
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        match &self.content {
            MetronomeContent::BeatUnit { beat_unit, beat_unit_dots, per_minute } => {
                w.write_text_element("beat-unit", beat_unit)?;
                for _ in beat_unit_dots {
                    w.write_empty(w.start_element("beat-unit-dot"))?;
                }
                w.write_text_element("per-minute", per_minute)?;
            }
            MetronomeContent::BeatUnitEquivalent(modulation) => {
                w.write_text_element("beat-unit", &modulation.beat_unit_1)?;
                for _ in &modulation.beat_unit_dots_1 {
                    w.write_empty(w.start_element("beat-unit-dot"))?;
                }
                w.write_text_element("beat-unit", &modulation.beat_unit_2)?;
                for _ in &modulation.beat_unit_dots_2 {
                    w.write_empty(w.start_element("beat-unit-dot"))?;
                }
            }
        }
        Ok(())
    }
}

// ============================================================================
// Helper functions for enum to string conversion
// ============================================================================

fn yes_no_str(yn: &YesNo) -> &'static str {
    match yn {
        YesNo::Yes => "yes",
        YesNo::No => "no",
    }
}

fn start_stop_str(ss: &StartStop) -> &'static str {
    match ss {
        StartStop::Start => "start",
        StartStop::Stop => "stop",
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

fn step_str(s: &Step) -> &'static str {
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

fn font_style_str(fs: &FontStyle) -> &'static str {
    match fs {
        FontStyle::Normal => "normal",
        FontStyle::Italic => "italic",
    }
}

fn font_weight_str(fw: &FontWeight) -> &'static str {
    match fw {
        FontWeight::Normal => "normal",
        FontWeight::Bold => "bold",
    }
}

fn font_size_str(fs: &FontSize) -> String {
    match fs {
        FontSize::Points(p) => p.to_string(),
        FontSize::Css(s) => s.to_string(),
    }
}

fn left_center_right_str(lcr: &LeftCenterRight) -> &'static str {
    match lcr {
        LeftCenterRight::Left => "left",
        LeftCenterRight::Center => "center",
        LeftCenterRight::Right => "right",
    }
}

fn valign_str(v: &Valign) -> &'static str {
    match v {
        Valign::Top => "top",
        Valign::Middle => "middle",
        Valign::Bottom => "bottom",
        Valign::Baseline => "baseline",
    }
}

fn above_below_str(ab: &AboveBelow) -> &'static str {
    match ab {
        AboveBelow::Above => "above",
        AboveBelow::Below => "below",
    }
}

fn symbol_size_str(ss: &SymbolSize) -> &'static str {
    match ss {
        SymbolSize::Full => "full",
        SymbolSize::Cue => "cue",
        SymbolSize::GraceCue => "grace-cue",
        SymbolSize::Large => "large",
    }
}

fn stem_value_str(sv: &StemValue) -> &'static str {
    match sv {
        StemValue::Down => "down",
        StemValue::Up => "up",
        StemValue::Double => "double",
        StemValue::None => "none",
    }
}

fn beam_value_str(bv: &BeamValue) -> &'static str {
    match bv {
        BeamValue::Begin => "begin",
        BeamValue::Continue => "continue",
        BeamValue::End => "end",
        BeamValue::ForwardHook => "forward hook",
        BeamValue::BackwardHook => "backward hook",
    }
}

fn fan_str(f: &Fan) -> &'static str {
    match f {
        Fan::Accel => "accel",
        Fan::Rit => "rit",
        Fan::None => "none",
    }
}

fn notehead_value_str(nv: &NoteheadValue) -> &'static str {
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

fn accidental_value_str(av: &AccidentalValue) -> &'static str {
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
        _ => "other", // Simplified for now
    }
}

fn clef_sign_str(cs: &ClefSign) -> &'static str {
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

fn time_symbol_str(ts: &TimeSymbol) -> &'static str {
    match ts {
        TimeSymbol::Common => "common",
        TimeSymbol::Cut => "cut",
        TimeSymbol::SingleNumber => "single-number",
        TimeSymbol::Note => "note",
        TimeSymbol::DottedNote => "dotted-note",
        TimeSymbol::Normal => "normal",
    }
}

fn wedge_type_str(wt: &WedgeType) -> &'static str {
    match wt {
        WedgeType::Crescendo => "crescendo",
        WedgeType::Diminuendo => "diminuendo",
        WedgeType::Stop => "stop",
        WedgeType::Continue => "continue",
    }
}

fn mode_str(m: &Mode) -> &'static str {
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
fn push_opt_attr_start<T: std::fmt::Display>(start: &mut quick_xml::events::BytesStart<'_>, name: &'static str, opt: &Option<T>) {
    if let Some(v) = opt {
        start.push_attribute((name, v.to_string().as_str()));
    }
}

fn push_opt_str_attr_start(start: &mut quick_xml::events::BytesStart<'_>, name: &'static str, opt: &Option<String>) {
    if let Some(v) = opt {
        start.push_attribute((name, v.as_str()));
    }
}
