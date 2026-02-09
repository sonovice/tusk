//! Serializer for MusicXML `<print>` elements.

use std::io::Write;

use super::score::yes_no_str;
use super::{MusicXmlSerialize, MusicXmlWriter, SerializeResult, push_opt_attr, push_opt_str_attr};
use crate::model::elements::{NameDisplay, NameDisplayContent};
use crate::model::print::{MeasureLayout, MeasureNumbering, Print};

impl MusicXmlSerialize for Print {
    fn element_name(&self) -> &'static str {
        "print"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "staff-spacing", self.staff_spacing);
        if let Some(ref ns) = self.new_system {
            attrs.push(("new-system", yes_no_str(ns).to_string()));
        }
        if let Some(ref np) = self.new_page {
            attrs.push(("new-page", yes_no_str(np).to_string()));
        }
        push_opt_attr!(attrs, "blank-page", self.blank_page);
        push_opt_str_attr!(attrs, "page-number", self.page_number);
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        self.page_layout.is_some()
            || self.system_layout.is_some()
            || !self.staff_layouts.is_empty()
            || self.measure_layout.is_some()
            || self.measure_numbering.is_some()
            || self.part_name_display.is_some()
            || self.part_abbreviation_display.is_some()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        // Layout group (same order as defaults)
        if let Some(ref pl) = self.page_layout {
            pl.serialize(w)?;
        }
        if let Some(ref sl) = self.system_layout {
            sl.serialize(w)?;
        }
        for staff_layout in &self.staff_layouts {
            staff_layout.serialize(w)?;
        }
        // Print-specific children
        if let Some(ref ml) = self.measure_layout {
            serialize_measure_layout(w, ml)?;
        }
        if let Some(ref mn) = self.measure_numbering {
            serialize_measure_numbering(w, mn)?;
        }
        if let Some(ref pnd) = self.part_name_display {
            serialize_name_display(w, "part-name-display", pnd)?;
        }
        if let Some(ref pad) = self.part_abbreviation_display {
            serialize_name_display(w, "part-abbreviation-display", pad)?;
        }
        Ok(())
    }
}

fn serialize_measure_layout<W: Write>(
    w: &mut MusicXmlWriter<W>,
    ml: &MeasureLayout,
) -> SerializeResult<()> {
    if let Some(distance) = ml.measure_distance {
        let start = w.start_element("measure-layout");
        w.write_start(start)?;
        w.write_text_element("measure-distance", &distance.to_string())?;
        w.write_end("measure-layout")?;
    } else {
        let start = w.start_element("measure-layout");
        w.write_empty(start)?;
    }
    Ok(())
}

fn serialize_measure_numbering<W: Write>(
    w: &mut MusicXmlWriter<W>,
    mn: &MeasureNumbering,
) -> SerializeResult<()> {
    let mut start = w.start_element("measure-numbering");
    if let Some(ref system) = mn.system {
        start.push_attribute(("system", system.as_str()));
    }
    if let Some(staff) = mn.staff {
        start.push_attribute(("staff", staff.to_string().as_str()));
    }
    if let Some(ref mra) = mn.multiple_rest_always {
        start.push_attribute(("multiple-rest-always", yes_no_str(mra)));
    }
    if let Some(ref mrr) = mn.multiple_rest_range {
        start.push_attribute(("multiple-rest-range", yes_no_str(mrr)));
    }
    push_start_opt_f64(&mut start, "default-x", mn.default_x);
    push_start_opt_f64(&mut start, "default-y", mn.default_y);
    if let Some(ref ff) = mn.font_family {
        start.push_attribute(("font-family", ff.as_str()));
    }
    push_start_opt_f64(&mut start, "font-size", mn.font_size);
    if let Some(ref fs) = mn.font_style {
        start.push_attribute(("font-style", fs.as_str()));
    }
    if let Some(ref fw) = mn.font_weight {
        start.push_attribute(("font-weight", fw.as_str()));
    }
    if let Some(ref ha) = mn.halign {
        start.push_attribute(("halign", ha.as_str()));
    }
    if let Some(ref va) = mn.valign {
        start.push_attribute(("valign", va.as_str()));
    }
    w.write_start(start)?;
    w.write_text(mn.value.as_str())?;
    w.write_end("measure-numbering")?;
    Ok(())
}

fn serialize_name_display<W: Write>(
    w: &mut MusicXmlWriter<W>,
    element_name: &str,
    nd: &NameDisplay,
) -> SerializeResult<()> {
    let mut start = w.start_element(element_name);
    if let Some(ref po) = nd.print_object {
        start.push_attribute(("print-object", yes_no_str(po)));
    }

    if nd.content.is_empty() {
        w.write_empty(start)?;
    } else {
        w.write_start(start)?;

        for elem in &nd.content {
            match elem {
                NameDisplayContent::DisplayText(dt) => {
                    let mut dt_start = w.start_element("display-text");
                    if let Some(ref ff) = dt.font_family {
                        dt_start.push_attribute(("font-family", ff.as_str()));
                    }
                    if let Some(ref fs) = dt.font_size {
                        dt_start.push_attribute(("font-size", fs.to_string().as_str()));
                    }
                    if let Some(ref fs) = dt.font_style {
                        dt_start.push_attribute(("font-style", fs.to_string().as_str()));
                    }
                    if let Some(ref fw) = dt.font_weight {
                        dt_start.push_attribute(("font-weight", fw.to_string().as_str()));
                    }
                    if let Some(dx) = dt.default_x {
                        dt_start.push_attribute(("default-x", dx.to_string().as_str()));
                    }
                    if let Some(dy) = dt.default_y {
                        dt_start.push_attribute(("default-y", dy.to_string().as_str()));
                    }
                    if let Some(ref ha) = dt.halign {
                        dt_start.push_attribute(("halign", ha.to_string().as_str()));
                    }
                    if let Some(ref va) = dt.valign {
                        dt_start.push_attribute(("valign", va.to_string().as_str()));
                    }
                    if let Some(ref j) = dt.justify {
                        dt_start.push_attribute(("justify", j.to_string().as_str()));
                    }
                    if let Some(ref id) = dt.id {
                        dt_start.push_attribute(("id", id.as_str()));
                    }
                    w.write_start(dt_start)?;
                    w.write_text(&dt.value)?;
                    w.write_end("display-text")?;
                }
                NameDisplayContent::AccidentalText(at) => {
                    let mut at_start = w.start_element("accidental-text");
                    if let Some(ref smufl) = at.smufl {
                        at_start.push_attribute(("smufl", smufl.as_str()));
                    }
                    w.write_start(at_start)?;
                    w.write_text(&at.value)?;
                    w.write_end("accidental-text")?;
                }
            }
        }

        w.write_end(element_name)?;
    }
    Ok(())
}

/// Push an optional f64 attribute to a BytesStart (avoids lifetime issues with to_string()).
fn push_start_opt_f64(
    start: &mut quick_xml::events::BytesStart<'static>,
    name: &str,
    val: Option<f64>,
) {
    if let Some(v) = val {
        start.push_attribute((name, v.to_string().as_str()));
    }
}
